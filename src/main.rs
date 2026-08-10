//! cellsmith CLI: read a minimal multi-cell TOML spec and emit, for every cell, the Liberate arcs
//! (`define_arc`), the structural Liberate `define_cell` blocks (`cells.tcl`), a
//! behavioural Verilog model (sequential UDP + wrapper), and a minimal Liberty fragment (`statetable`
//! for hysteretic outputs, plain `function` for combinational ones).

use std::collections::{BTreeSet, HashMap};
use std::error::Error;
use std::fs;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

use clap::{Arg, ArgAction, ArgMatches, Args, Command, FromArgMatches, Parser};
use espresso_logic::{Minterm, Symbol};
use rayon::prelude::*;

use cellsmith::emit::arcs_tcl::{cell_arcs, ArcsTclOptions, CellArcs};
use cellsmith::emit::define_cell::cell_define_cell;
use cellsmith::emit::liberty::library_liberty;
use cellsmith::emit::verilog::cell_verilog;
use cellsmith::logic::hazard::{Cause, Hazard, Outcome, Racer};
use cellsmith::logic::machine::{ExplorationBudget, ExplorationLimit};
use cellsmith::model::{parse_spec, AnalysedCell, ArcClass, ArcClasses, ConstraintPins};

/// Generate Cadence Liberate transition arcs, a behavioural Verilog model and a
/// Liberty fragment for logic cells, including state-holding/hysteretic cells.
#[derive(Parser)]
#[command(name = "cellsmith", version, about, long_about = None)]
struct Cli {
    /// TOML cell spec ("-" reads stdin).
    spec: String,

    /// Output directory.
    #[arg(short, long, default_value = ".")]
    outdir: PathBuf,

    /// Output base name [default: the spec file stem].
    #[arg(short, long)]
    name: Option<String>,

    /// The arc classes whose `-when` arcs are also emitted; the flag's help text lives with
    /// [`WhenArg`], as clap takes no help from the doc comment of a flattened field.
    #[command(flatten)]
    when: WhenArg,

    /// Suppress hidden (internal-power) arcs.
    #[arg(long)]
    no_internal: bool,

    /// Suppress `define_leakage` blocks.
    #[arg(long)]
    no_leakage: bool,

    /// Suppress the `<base>_cells.tcl` artifact.
    #[arg(long)]
    no_cells: bool,

    /// The input pins derived constraint arcs are emitted for; the flag's help text lives with
    /// [`ConstraintsArg`], as clap takes no help from the doc comment of a flattened field.
    #[command(flatten)]
    constraints: ConstraintsArg,

    /// Suppress the edge-register annotation.
    #[arg(long)]
    no_edge_collapse: bool,

    /// Voltage for logic `0` [default: 0].
    #[arg(long, value_name = "VOLTAGE")]
    logic_low: Option<String>,

    /// Voltage for logic `1` [default: $VDD].
    #[arg(long, value_name = "VOLTAGE")]
    logic_high: Option<String>,

    /// Write the artifacts to stdout instead of to files.
    #[arg(long)]
    stdout: bool,

    /// Ceiling on pooled seed minterms.
    #[arg(long, value_name = "N", default_value_t = ExplorationBudget::default().candidates)]
    max_candidates: usize,

    /// Ceiling on recorded stable states.
    #[arg(long, value_name = "N", default_value_t = ExplorationBudget::default().states)]
    max_states: usize,
}

/// The `--when` flag, resolved to the set of arc classes it selects. Every occurrence of the flag is
/// unioned in, and a bare occurrence — which clap records as an occurrence carrying no value — selects
/// every class, so `--when --when=hidden` selects every class in either order. Reading the occurrence
/// groups back from [`ArgMatches`] is what keeps a bare occurrence visible next to a valued one, hence
/// the hand-written [`Args`] implementation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct WhenArg {
    /// The selected classes; empty when the flag is absent.
    classes: ArcClasses,
}

/// The `--when` argument definition, shared by both `augment_args` entry points.
fn when_arg() -> Arg {
    Arg::new("when")
        .long("when")
        .value_name("CLASS")
        .value_parser(clap::value_parser!(ArcClass))
        .num_args(0..=1)
        .require_equals(true)
        .action(ArgAction::Append)
        .help("Also emit `-when`-conditioned arcs; bare = every class, repeatable")
}

impl Args for WhenArg {
    fn augment_args(cmd: Command) -> Command {
        cmd.arg(when_arg())
    }

    fn augment_args_for_update(cmd: Command) -> Command {
        cmd.arg(when_arg())
    }
}

impl FromArgMatches for WhenArg {
    fn from_arg_matches(matches: &ArgMatches) -> Result<Self, clap::Error> {
        let mut classes = ArcClasses::default();
        for occurrence in matches
            .get_occurrences::<ArcClass>("when")
            .into_iter()
            .flatten()
        {
            let mut values = occurrence.copied().peekable();
            classes = classes.union(if values.peek().is_none() {
                ArcClasses::ALL // a bare `--when`: every class
            } else {
                values.collect()
            });
        }
        Ok(Self { classes })
    }

    fn update_from_arg_matches(&mut self, matches: &ArgMatches) -> Result<(), clap::Error> {
        *self = Self::from_arg_matches(matches)?;
        Ok(())
    }
}

/// The `--constraints` flag, resolved to the input pins it selects. Every occurrence of the flag is
/// unioned in, and a bare occurrence — which clap records as an occurrence carrying no value — selects
/// every pin, so `--constraints --constraints=D` selects every pin in either order. Reading the
/// occurrence groups back from [`ArgMatches`] is what keeps a bare occurrence visible next to a valued
/// one, hence the hand-written [`Args`] implementation, as for [`WhenArg`].
#[derive(Debug, Clone, PartialEq, Eq)]
struct ConstraintsArg {
    /// The selected pins; [`ConstraintPins::Off`] when the flag is absent.
    pins: ConstraintPins,
}

/// The `--constraints` argument definition, shared by both `augment_args` entry points.
fn constraints_arg() -> Arg {
    Arg::new("constraints")
        .long("constraints")
        .value_name("PIN")
        // A pin name is any identifier the spec uses, so the parser only rules out the empty one —
        // `--constraints=` names no pin, and is a mistyped flag rather than a selection.
        .value_parser(clap::builder::NonEmptyStringValueParser::new())
        .num_args(0..=1)
        .require_equals(true)
        .action(ArgAction::Append)
        .help(
            "Emit derived setup/hold, non_seq & min_pulse_width constraint arcs; bare = every input \
             pin, repeatable, unioned with each cell's own `constraint_arcs`",
        )
}

impl Args for ConstraintsArg {
    fn augment_args(cmd: Command) -> Command {
        cmd.arg(constraints_arg())
    }

    fn augment_args_for_update(cmd: Command) -> Command {
        cmd.arg(constraints_arg())
    }
}

impl FromArgMatches for ConstraintsArg {
    fn from_arg_matches(matches: &ArgMatches) -> Result<Self, clap::Error> {
        let mut pins = ConstraintPins::Off;
        for occurrence in matches
            .get_occurrences::<String>("constraints")
            .into_iter()
            .flatten()
        {
            let mut values = occurrence.peekable();
            pins = pins.union(&if values.peek().is_none() {
                ConstraintPins::All // a bare `--constraints`: every pin
            } else {
                ConstraintPins::Named(values.map(|s| Symbol::from(s.as_str())).collect())
            });
        }
        Ok(Self { pins })
    }

    fn update_from_arg_matches(&mut self, matches: &ArgMatches) -> Result<(), clap::Error> {
        *self = Self::from_arg_matches(matches)?;
        Ok(())
    }
}

fn main() {
    if let Err(e) = run(Cli::parse()) {
        eprintln!("cellsmith: {e}");
        std::process::exit(1);
    }
}

fn run(cli: Cli) -> Result<(), Box<dyn Error>> {
    let src = read_spec(&cli.spec)?;
    let mut spec = parse_spec(&src)?;
    // `--constraints` is a blanket opt-in: every pin selected on the command line is added to each
    // cell's own `constraint_arcs`, so a cell can ask for constraints on more pins but never opt back
    // out of a pin the CLI selected. Applied before analysis so the single per-cell selection is what
    // generation and emission both read downstream.
    for c in &mut spec.cells {
        c.constraint_arcs = c.constraint_arcs.union(&cli.constraints.pins);
    }
    // `--no-edge-collapse` is a blanket disable: it opts every cell out of the edge-register collapse,
    // exactly as if each had declared `no_edge_collapse = true`.
    if cli.no_edge_collapse {
        for c in &mut spec.cells {
            c.no_edge_collapse = true;
        }
    }
    // `--when` is a blanket UNION: every class selected on the command line is added to each cell's
    // own `when` set, so a cell can select more classes but never opt back out of a CLI-selected one.
    for c in &mut spec.cells {
        c.when = c.when.union(cli.when.classes);
    }
    // `--logic-low`/`--logic-high` are per-field CLI defaults: a cell's own key wins, so the CLI value
    // only fills in where the cell left its own key unset.
    if let Some(v) = &cli.logic_low {
        for c in &mut spec.cells {
            c.logic_low.get_or_insert_with(|| v.clone());
        }
    }
    if let Some(v) = &cli.logic_high {
        for c in &mut spec.cells {
            c.logic_high.get_or_insert_with(|| v.clone());
        }
    }
    let budget = ExplorationBudget {
        candidates: cli.max_candidates,
        states: cli.max_states,
    };
    let cells: Vec<AnalysedCell> = spec.analyse_with(&budget)?;

    // A cell whose exploration stopped at a budget ceiling has no arcs, hazards, leakage states or
    // constraints — emitting its artifacts anyway would present that silence as the cell's behaviour, so
    // this is an error and nothing is written. Every offending cell is named, not just the first. A cell
    // explores once, however many views it carries (`AnalysedCell::arc_view`), and a ceiling that stopped
    // that exploration is carried by every view of the cell, so consulting both fields names each
    // offending cell exactly once.
    let unexplored: Vec<(&AnalysedCell, ExplorationLimit)> = cells
        .iter()
        .filter_map(|c| {
            c.unexplored
                .or(c.arc_view().unexplored)
                .map(|limit| (c, limit))
        })
        .collect();
    if !unexplored.is_empty() {
        for (c, limit) in unexplored {
            let (stopped_at, flag) = match limit {
                ExplorationLimit::Candidates(n) => (
                    format!("the candidate budget ({n} seed minterms)"),
                    "--max-candidates",
                ),
                ExplorationLimit::States(n) => (
                    format!("the state budget ({n} explored states)"),
                    "--max-states",
                ),
            };
            eprintln!(
                "cellsmith: error: cell {:?}: exploration stopped at {stopped_at}; no arcs, hazards, \
                 leakage states or constraints are derived — raise it with {flag}",
                c.repr_name(),
            );
        }
        // Each cell's diagnostic is already complete on stderr, so there is no error value left for
        // `main` to print: leave with the failing status before any artifact is rendered.
        std::process::exit(1);
    }

    // Rendered before the diagnostics, because one of them reports what the rendering could not say.
    let arc_opts = ArcsTclOptions {
        emit_internal: !cli.no_internal,
        emit_leakage: !cli.no_leakage,
    };
    let rendered: Vec<CellArcs> = cells.par_iter().map(|c| cell_arcs(c, arc_opts)).collect();

    // Each warning is one contiguous block of lines (a header plus its indented detail fields);
    // distinct warnings are separated by a single blank line when printed, so a block reads as a unit.
    let mut warnings: Vec<String> = Vec::new();

    // Diagnose the cell's detected hazards, one warning per SITUATION — one cause, at one input
    // condition, probed from one state. Detection files a record per (cause, outcome), so a situation
    // showing both outcomes arrives as two records; they are gathered here into the single entry whose
    // `detected` field names what was observed there. The pass reads the ARC VIEW, the same analysis
    // `cell_arcs` renders: it is that view's hazards the emitted constraint arcs come from, so reporting
    // the other view's would describe arcs the run never wrote.
    for c in &cells {
        let mut situations: HashMap<Situation, Vec<&Hazard>> = HashMap::new();
        for a in &c.arc_view().hazards {
            situations.entry(Situation::of(a)).or_default().push(a);
        }
        for (situation, records) in &situations {
            warnings.push(hazard_warning(c, situation, records));
        }
    }

    // Diagnose the arcs no block could state: every arc should express the cell state it measures
    // from, and `-ic` and `-vector` reach exactly the `-pinlist`, so a firing that differs only in an
    // internal node with no column renders a block already emitted. Exposing those nodes is the remedy,
    // which is why the warning names the state as well as the arc.
    for (c, r) in cells.iter().zip(&rendered) {
        if r.masked.is_empty() {
            continue;
        }
        let mut lines = vec![format!(
            "cellsmith: warning: cell {:?}: {} block(s) conflate {} arcs: too few nodes exposed for -ic to express the cell state",
            c.repr_name(),
            r.masked.len(),
            r.masked.iter().map(|m| m.states.len()).sum::<usize>(),
        )];
        for m in &r.masked {
            // Every state the block covers, as equals — it expresses none of them, and which firing
            // reached the emitter first is nothing to report. What differs across them wants exposing.
            let mut fields = vec![("arc", m.arc_str())];
            fields.extend(m.state_strs().into_iter().map(|s| ("cell state", s)));
            lines.extend(subblock(&fields));
        }
        warnings.push(lines.join("\n"));
    }

    if !warnings.is_empty() {
        eprintln!("{}", warnings.join("\n\n"));
    }

    // Constraints avoid a hazard already reported by the warnings above, so the constraint arcs are
    // emitted (below, gated by the per-cell opt-in) without a separate diagnostic.

    let base = cli.name.unwrap_or_else(|| base_name(&cli.spec));
    let arcs: String = rendered.iter().map(|r| r.tcl.as_str()).collect();
    let verilog = render(&cells, cell_verilog);
    let liberty = library_liberty(&base, &cells);
    let cells_tcl = render(&cells, cell_define_cell);

    if cli.stdout {
        let mut out = io::stdout().lock();
        write!(out, "{}", banner("arcs.tcl", &arcs))?;
        write!(out, "{}", banner("verilog", &verilog))?;
        write!(out, "{}", banner("liberty", &liberty))?;
        if !cli.no_cells {
            write!(out, "{}", banner("cells.tcl", &cells_tcl))?;
        }
        return Ok(());
    }

    fs::create_dir_all(&cli.outdir)?;
    write_file(&cli.outdir, &format!("{base}_arcs.tcl"), &arcs)?;
    write_file(&cli.outdir, &format!("{base}.v"), &verilog)?;
    write_file(&cli.outdir, &format!("{base}.lib"), &liberty)?;
    if !cli.no_cells {
        write_file(&cli.outdir, &format!("{base}_cells.tcl"), &cells_tcl)?;
    }
    Ok(())
}

/// Read the spec source from a path, or from stdin when the path is `-`.
fn read_spec(spec: &str) -> io::Result<String> {
    if spec == "-" {
        let mut buf = String::new();
        io::stdin().read_to_string(&mut buf)?;
        Ok(buf)
    } else {
        fs::read_to_string(spec)
    }
}

/// Concatenate one artifact across every cell.
// `one` has trait bound `Sync` (not `Send`). Rayon's `par_iter().map()` requires `F: Send`,
// but a reference `&F` is `Fn` with `&F: Send` whenever `F: Sync`. Pass `&one` to satisfy this.
fn render(cells: &[AnalysedCell], one: impl (Fn(&AnalysedCell) -> String) + Sync) -> String {
    cells.par_iter().map(&one).collect::<Vec<String>>().concat()
}

/// A stdout section banner for one artifact.
fn banner(kind: &str, body: &str) -> String {
    format!("// ===== cellsmith {kind} =====\n{body}\n")
}

/// Render one hazard detail sub-block as its lines: the first is `-`-bulleted, the rest indented to
/// align under it. Each is a colon-labelled field whose values are column-aligned (the longest label,
/// `reached along:`, sets the column). No trailing newline — callers join a warning's lines with `\n`.
fn subblock(fields: &[(&str, String)]) -> Vec<String> {
    fields
        .iter()
        .enumerate()
        .map(|(i, (label, value))| {
            let marker = if i == 0 { "  - " } else { "    " };
            format!("{marker}{:<14} {value}", format!("{label}:"))
        })
        .collect()
}

/// The occasion one hazard warning reports: a cause, the primary-input condition it occurs under, and
/// the probed state it acts from. Detection files one record per (cause, outcome), so the records
/// sharing a situation are the outcomes observed there, and the warning names them together.
#[derive(PartialEq, Eq, Hash)]
struct Situation<'a> {
    cause: &'a Cause,
    condition: &'a Minterm<Symbol>,
    state: &'a Minterm<Symbol>,
}

impl<'a> Situation<'a> {
    /// The situation `hazard` was observed in.
    fn of(hazard: &'a Hazard) -> Self {
        Self {
            cause: &hazard.cause,
            condition: &hazard.condition,
            state: &hazard.state,
        }
    }
}

/// One situation's warning: a header naming what causes the hazard, the state it happens at and the
/// nodes it puts at risk, over the detail sub-block. `records` are the situation's detected hazards, one
/// per outcome observed; the fields that follow from the situation alone — its condition and the path
/// into its state — are the same in each, so they are read from the first.
fn hazard_warning(cell: &AnalysedCell, situation: &Situation, records: &[&Hazard]) -> String {
    let first = records
        .first()
        .expect("a situation is only entered by a record");
    let outcomes: BTreeSet<Outcome> = records.iter().map(|h| h.outcome).collect();
    let mut fields = vec![
        (
            "detected",
            outcomes
                .iter()
                .map(|o| outcome_str(*o))
                .collect::<Vec<_>>()
                .join(", "),
        ),
        ("when", first.condition_str()),
        ("reached along", first.path_str()),
    ];
    match situation.cause {
        Cause::Race { pins } => {
            fields.push(("pre-hazard", first.pre_state_str()));
            if outcomes.contains(&Outcome::Indeterminate) {
                fields.push(("orders", orders_str(pins)));
            }
            if outcomes.contains(&Outcome::Oscillation) {
                fields.push(("triggered by", trigger_str(pins)));
            }
        }
        // A pulse returns its pin to the value it started from, so the pre-pulse input state IS the
        // condition the hazard occurs under — `when` states it, and a separate pre-hazard field would
        // only restate it. Where a pulse decides two node sets neither of which contains the other,
        // detection keeps both observations, so each indeterminate record states its own landings.
        Cause::Pulse { .. } => fields.extend(
            records
                .iter()
                .filter(|h| h.outcome == Outcome::Indeterminate)
                .map(|h| ("outcomes", h.settled_strs().join(" | "))),
        ),
    }
    let mut lines = vec![format!(
        "cellsmith: warning: cell {:?}: {} causes a hazard at {} on nodes {{{}}}",
        cell.repr_name(),
        cause_str(situation.cause),
        Hazard::state_str(situation.state),
        hazard_nodes(records).join(", "),
    )];
    lines.extend(subblock(&fields));
    lines.join("\n")
}

/// What causes the hazard, as the header names it: the timing that has to be wrong for the cell to be
/// at risk, rather than the transition itself. A pulse is a hazard when it is too SHORT — exactly what
/// the generated minimum pulse width forbids — and a pair of edges when too little separates them, what
/// the generated setup/hold separation forbids. A lone toggle observed not to converge has no second
/// edge to be separated from, and no constraint follows from it, so there the transition is the whole of
/// the condition.
fn cause_str(cause: &Cause) -> String {
    match cause {
        Cause::Pulse { pin, edge } => format!("a short pulse on {pin}{}", edge.arrow()),
        Cause::Race { pins } => match toggles(pins).as_slice() {
            [one] => format!("toggling {one}"),
            many => format!("too little separation between {}", many.join(" and ")),
        },
    }
}

/// The name the `detected` field reports an outcome under.
fn outcome_str(outcome: Outcome) -> &'static str {
    match outcome {
        Outcome::Indeterminate => "indeterminate",
        Outcome::Oscillation => "oscillation",
    }
}

/// The nodes a situation puts at risk: every node its records name, without repeats. Each outcome names
/// the nodes its own reading decides — a ring is over what cycles, an indeterminate settling over what
/// the competing landings differ in — and the two need not agree, so the entry names their union.
fn hazard_nodes<'a>(records: &[&'a Hazard]) -> Vec<&'a str> {
    let mut nodes: Vec<&str> = Vec::new();
    for h in records {
        for n in &h.group {
            if !nodes.contains(&n.as_str()) {
                nodes.push(n.as_str());
            }
        }
    }
    nodes
}

/// The triggering transitions of an indeterminate race: every order its edges can arrive in, since
/// which lands first is what the settled state depends on (`A↓ then B↑ vs B↑ then A↓`).
fn orders_str(pins: &[Racer]) -> String {
    orderings(pins.len())
        .into_iter()
        .map(|order| {
            order
                .into_iter()
                .map(|i| format!("{}{}", pins[i].pin, pins[i].edge.arrow()))
                .collect::<Vec<_>>()
                .join(" then ")
        })
        .collect::<Vec<_>>()
        .join(" vs ")
}

/// Every ordering of `n` positions, as sequences of indices: each position taken first in turn, each
/// followed by every ordering of those left, so the identity order comes out first.
fn orderings(n: usize) -> Vec<Vec<usize>> {
    if n == 0 {
        return vec![Vec::new()];
    }
    let mut orders: Vec<Vec<usize>> = Vec::new();
    for first in 0..n {
        for mut rest in orderings(n - 1) {
            // `rest` indexes the `n - 1` positions left once `first` is taken, so an index at or past
            // it names the position one further along.
            for i in &mut rest {
                if *i >= first {
                    *i += 1;
                }
            }
            orders.push(std::iter::once(first).chain(rest).collect());
        }
    }
    orders
}

/// The triggering transition of an oscillating race: the toggles it was observed under. Two or more
/// arrive together, which is what drives the cycle (`simultaneous toggle S↓ & R↓`); one arrives with
/// nothing to coincide with (`toggling A↓`).
fn trigger_str(pins: &[Racer]) -> String {
    match toggles(pins).as_slice() {
        [one] => format!("toggling {one}"),
        many => format!("simultaneous toggle {}", many.join(" & ")),
    }
}

/// Each racing pin with the edge it makes (`A↓`), in the order the probe named them.
fn toggles(pins: &[Racer]) -> Vec<String> {
    pins.iter()
        .map(|r| format!("{}{}", r.pin, r.edge.arrow()))
        .collect()
}

/// The default output base name derived from the spec path (stem), or "cells" for stdin.
fn base_name(spec: &str) -> String {
    if spec == "-" {
        return "cells".to_owned();
    }
    Path::new(spec)
        .file_stem()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_else(|| "cells".to_owned())
}

/// Write one artifact file into `dir`, reporting the path.
fn write_file(dir: &Path, name: &str, body: &str) -> io::Result<()> {
    let path = dir.join(name);
    fs::write(&path, body)?;
    eprintln!("wrote {}", path.display());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    /// The classes `args` select, parsed through the real CLI.
    fn when_classes(args: &[&str]) -> ArcClasses {
        let mut argv = vec!["cellsmith"];
        argv.extend_from_slice(args);
        argv.push("s.toml");
        Cli::try_parse_from(argv).unwrap().when.classes
    }

    #[test]
    fn when_bare_flag_selects_all_and_keeps_positional() {
        let cli = Cli::try_parse_from(["cellsmith", "--when", "s.toml"]).unwrap();
        assert_eq!(cli.when.classes, ArcClasses::ALL);
        // `require_equals` keeps the positional `<SPEC>` from being swallowed as the class value.
        assert_eq!(cli.spec, "s.toml");
    }

    #[test]
    fn when_equals_selects_one_class() {
        let when = when_classes(&["--when=hidden"]);
        assert!(when.contains(ArcClass::Hidden));
        assert!(!when.contains(ArcClass::Transition));
    }

    #[test]
    fn when_repeats_union_their_classes() {
        assert_eq!(
            when_classes(&["--when=hidden", "--when=transition", "--when=constraint"]),
            ArcClasses::ALL,
        );
    }

    #[test]
    fn when_bare_unions_with_a_valued_occurrence_in_either_order() {
        // The bare occurrence is the superset, so it wins whichever side of the valued one it lands.
        assert_eq!(when_classes(&["--when", "--when=hidden"]), ArcClasses::ALL);
        assert_eq!(when_classes(&["--when=hidden", "--when"]), ArcClasses::ALL);
    }

    #[test]
    fn when_absent_selects_no_class() {
        let cli = Cli::try_parse_from(["cellsmith", "s.toml"]).unwrap();
        assert_eq!(cli.when.classes, ArcClasses::default());
    }

    #[test]
    fn when_rejects_an_unknown_class() {
        assert!(Cli::try_parse_from(["cellsmith", "--when=bogus", "s.toml"]).is_err());
    }

    #[test]
    fn when_rejects_an_empty_value() {
        assert!(Cli::try_parse_from(["cellsmith", "--when=", "s.toml"]).is_err());
    }

    #[test]
    fn when_does_not_take_a_spaced_value() {
        // `require_equals`: the spaced token is the positional `<SPEC>`, so a second one is unexpected.
        assert!(Cli::try_parse_from(["cellsmith", "--when", "hidden", "s.toml"]).is_err());
    }

    /// The pins `args` select, parsed through the real CLI.
    fn constraint_pins(args: &[&str]) -> ConstraintPins {
        let mut argv = vec!["cellsmith"];
        argv.extend_from_slice(args);
        argv.push("s.toml");
        Cli::try_parse_from(argv).unwrap().constraints.pins
    }

    #[test]
    fn constraints_bare_flag_selects_every_pin_and_keeps_positional() {
        let cli = Cli::try_parse_from(["cellsmith", "--constraints", "s.toml"]).unwrap();
        assert_eq!(cli.constraints.pins, ConstraintPins::All);
        // `require_equals` keeps the positional `<SPEC>` from being swallowed as the pin name.
        assert_eq!(cli.spec, "s.toml");
    }

    #[test]
    fn constraints_equals_names_one_pin() {
        assert_eq!(
            constraint_pins(&["--constraints=D"]),
            ConstraintPins::Named(vec![Symbol::from("D")]),
        );
    }

    #[test]
    fn constraints_repeats_union_their_pins() {
        assert_eq!(
            constraint_pins(&["--constraints=D", "--constraints=CLK"]),
            ConstraintPins::Named(vec![Symbol::from("D"), Symbol::from("CLK")]),
        );
    }

    #[test]
    fn constraints_bare_unions_with_a_valued_occurrence_in_either_order() {
        // The bare occurrence is the superset, so it wins whichever side of the valued one it lands.
        assert_eq!(
            constraint_pins(&["--constraints", "--constraints=D"]),
            ConstraintPins::All,
        );
        assert_eq!(
            constraint_pins(&["--constraints=D", "--constraints"]),
            ConstraintPins::All,
        );
    }

    #[test]
    fn constraints_absent_selects_no_pin() {
        let cli = Cli::try_parse_from(["cellsmith", "s.toml"]).unwrap();
        assert_eq!(cli.constraints.pins, ConstraintPins::Off);
    }

    #[test]
    fn constraints_rejects_an_empty_value() {
        assert!(Cli::try_parse_from(["cellsmith", "--constraints=", "s.toml"]).is_err());
    }

    #[test]
    fn constraints_does_not_take_a_spaced_value() {
        // `require_equals`: the spaced token is the positional `<SPEC>`, so a second one is unexpected.
        assert!(Cli::try_parse_from(["cellsmith", "--constraints", "D", "s.toml"]).is_err());
    }

    #[test]
    fn cli_constraint_pins_are_added_to_the_cells_own() {
        let mut spec = parse_spec(
            r#"
[[cell]]
name = "X"
inputs = ["A", "B"]
constraint_arcs = "A"
[cell.outputs]
Y = "A*B"
"#,
        )
        .unwrap();
        let cli = Cli::try_parse_from(["cellsmith", "--constraints=B", "s.toml"]).unwrap();
        for c in &mut spec.cells {
            c.constraint_arcs = c.constraint_arcs.union(&cli.constraints.pins);
        }
        assert_eq!(
            spec.cells[0].constraint_arcs,
            ConstraintPins::Named(vec![Symbol::from("A"), Symbol::from("B")]),
            "the flag adds its pin without dropping the cell's own",
        );
    }

    #[test]
    fn cell_logic_high_key_wins_over_cli_default() {
        let mut spec = parse_spec(
            r#"
[[cell]]
name = "X"
inputs = ["A"]
logic_high = "$VDDH"
[cell.outputs]
Y = "A"
"#,
        )
        .unwrap();
        let cli = Cli::try_parse_from(["cellsmith", "--logic-high=$VDD", "s.toml"]).unwrap();
        if let Some(v) = &cli.logic_high {
            for c in &mut spec.cells {
                c.logic_high.get_or_insert_with(|| v.clone());
            }
        }
        assert_eq!(spec.cells[0].logic_high.as_deref(), Some("$VDDH"));
    }

    #[test]
    fn base_name_strips_dir_and_extension() {
        assert_eq!(base_name("/some/dir/cells.toml"), "cells");
        assert_eq!(base_name("cells.toml"), "cells");
        assert_eq!(base_name("plain"), "plain"); // no extension: the whole stem
        assert_eq!(base_name("-"), "cells"); // stdin sentinel
    }

    #[test]
    fn banner_wraps_body_with_a_labelled_header() {
        assert_eq!(
            banner("arcs.tcl", "BODY"),
            "// ===== cellsmith arcs.tcl =====\nBODY\n",
        );
    }

    #[test]
    fn read_spec_reads_a_file() {
        let path =
            std::env::temp_dir().join(format!("cellsmith_read_spec_{}.toml", std::process::id()));
        fs::write(&path, "hello = 1\n").unwrap();
        let got = read_spec(path.to_str().unwrap()).unwrap();
        assert_eq!(got, "hello = 1\n");
        fs::remove_file(&path).ok();
    }

    #[test]
    fn read_spec_errors_on_a_missing_path() {
        assert!(read_spec("/no/such/cellsmith/spec.toml").is_err());
    }
}
