//! cellsmith CLI: read a minimal multi-cell TOML spec and emit, for every cell, the Liberate arcs
//! (`define_arc`), the structural Liberate `define_cell` blocks (`cells.tcl`), a
//! behavioural Verilog model (sequential UDP + wrapper), and a minimal Liberty fragment (`statetable`
//! for hysteretic outputs, plain `function` for combinational ones).

use std::collections::{BTreeMap, HashMap};
use std::error::Error;
use std::fmt;
use std::fs;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

use clap::{Arg, ArgAction, ArgMatches, Args, Command, FromArgMatches, Parser};
use espresso_logic::{Minterm, Symbol};
use rayon::prelude::*;

use cellsmith::emit::arcs_tcl::{cell_arcs, ArcsTclOptions, CellArcs};
use cellsmith::emit::block::Description;
use cellsmith::emit::define_cell::cell_define_cell;
use cellsmith::emit::liberty::library_liberty;
use cellsmith::emit::verilog::cell_verilog;
use cellsmith::logic::hazard::{Cause, Hazard, Outcome, Racer};
use cellsmith::logic::machine::{ExplorationBudget, ExplorationLimit};
use cellsmith::model::{parse_spec, AnalysedCell, ArcClass, ArcClasses, ConstraintPins};
use cellsmith::report::{self, Commas, State};

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

    /// Emit derived constraint arcs; every input pin.
    #[arg(long)]
    constraints: bool,

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

fn main() {
    if let Err(e) = run(Cli::parse()) {
        eprintln!("cellsmith: {e}");
        std::process::exit(1);
    }
}

fn run(cli: Cli) -> Result<(), Box<dyn Error>> {
    let src = read_spec(&cli.spec)?;
    let mut spec = parse_spec(&src)?;
    // `--constraints` is a blanket opt-in: it asks every cell for constraint arcs on every input pin,
    // exactly as if each had declared `constraint_arcs = true`. Which pins one cell wants is the spec's
    // `constraint_arcs` to say, and the flag subsumes any such selection rather than narrowing it.
    // Applied before analysis so the single per-cell selection is what generation and emission both
    // read downstream.
    if cli.constraints {
        for c in &mut spec.cells {
            c.constraint_arcs = ConstraintPins::All;
        }
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

    // Each warning is one contiguous block of lines (a header plus its indented detail fields), written
    // as it is composed into the one locked handle; a blank line before every warning but the first
    // keeps the blocks reading as units.
    let mut err = io::stderr().lock();
    let mut warned = false;

    // Diagnose the cell's detected hazards, one warning per OCCASION — one cause, which is a transition
    // out of one starting state. Detection files a record per (cause, outcome), so an occasion showing
    // both outcomes arrives as two records; they are gathered here into the single entry whose body
    // names each outcome beside the nodes it puts at risk. The pass reads the ARC VIEW, the same
    // analysis `cell_arcs` renders: it is that view's hazards the emitted constraint arcs come from, so
    // reporting the other view's would describe arcs the run never wrote.
    for c in &cells {
        let mut occasions: HashMap<Occasion, Vec<&Hazard>> = HashMap::new();
        for a in &c.arc_view().hazards {
            occasions.entry(Occasion::of(a)).or_default().push(a);
        }
        for (occasion, records) in &occasions {
            if std::mem::replace(&mut warned, true) {
                writeln!(err)?;
            }
            hazard_warning(&mut err, c, occasion, records)?;
        }
    }

    // Diagnose the measurements no block could state: every block should express the cell state it
    // measures from, and its columns reach exactly its `-pinlist`, so a firing that differs only in an
    // internal node with no column renders a block already emitted. Exposing those nodes is the remedy,
    // which is why the warning names the state as well as the block.
    for (c, r) in cells.iter().zip(&rendered) {
        if r.conflations.is_empty() {
            continue;
        }
        if std::mem::replace(&mut warned, true) {
            writeln!(err)?;
        }
        writeln!(
            err,
            "cellsmith: warning: cell {:?}: {} block(s) conflate {} measurements: too few nodes exposed to express the cell state",
            c.repr_name(),
            r.conflations.len(),
            r.conflations.iter().map(|m| m.states.len()).sum::<usize>(),
        )?;
        for m in &r.conflations {
            // Every state the block covers, as equals — it expresses none of them, and which firing
            // reached the emitter first is nothing to report. What differs across them wants exposing.
            let block = Description(&m.block);
            let states: Vec<State> = m.states.iter().map(State).collect();
            let mut fields: Vec<(&str, &dyn fmt::Display)> = vec![("block", &block)];
            fields.extend(
                states
                    .iter()
                    .map(|s| ("cell state", s as &dyn fmt::Display)),
            );
            subblock(&mut err, "  - ", &fields)?;
        }
    }
    // The report is complete: release the handle before the artifacts are written, each of which
    // reports its path on this same stream.
    drop(err);

    // Constraints avoid a hazard already reported by the warnings above, so the constraint arcs are
    // emitted (below, gated by the per-cell opt-in) without a separate diagnostic.

    let base = cli.name.unwrap_or_else(|| base_name(&cli.spec));
    let arcs = Deck(&rendered);
    let verilog = render(&cells, cell_verilog);
    let liberty = library_liberty(&base, &cells);
    let cells_tcl = render(&cells, cell_define_cell);

    if cli.stdout {
        // Buffered, because an artifact reaches the handle as the many small writes its own `Display`
        // makes rather than as one string.
        let mut out = io::BufWriter::new(io::stdout().lock());
        banner(&mut out, "arcs.tcl", &arcs)?;
        banner(&mut out, "verilog", &verilog)?;
        banner(&mut out, "liberty", &liberty)?;
        if !cli.no_cells {
            banner(&mut out, "cells.tcl", &cells_tcl)?;
        }
        out.flush()?;
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

/// Every cell's Liberate blocks, in the order the cells were analysed and each cell's in the order its
/// emitter stated them. The blocks travel as values and become text here, at the sink they are written
/// to.
struct Deck<'a>(&'a [CellArcs]);

impl fmt::Display for Deck<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for cell in self.0 {
            for block in &cell.blocks {
                write!(f, "{block}")?;
            }
        }
        Ok(())
    }
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

/// One artifact under its stdout section header, written into `out` as the artifact renders itself.
fn banner(out: &mut impl io::Write, kind: &str, body: &impl fmt::Display) -> io::Result<()> {
    writeln!(out, "// ===== cellsmith {kind} =====")?;
    writeln!(out, "{body}")
}

/// Write one warning detail block: colon-labelled fields, indented under the header with their values
/// column-aligned. `lead` opens the first line — a hazard warning states one block and opens it at the
/// same indent as the rest, while the masked-arc warning states a block per conflated arc and bullets
/// each so the blocks read apart.
fn subblock(
    w: &mut impl io::Write,
    lead: &str,
    fields: &[(&str, &dyn fmt::Display)],
) -> io::Result<()> {
    for (i, (label, value)) in fields.iter().enumerate() {
        let marker = if i == 0 { lead } else { "    " };
        // The colon belongs to the label, so it is what the 16-column field is padded around: the label
        // and its colon go out first, then the padding that would have followed them.
        let padding = 16usize.saturating_sub(label.len() + 1);
        writeln!(w, "{marker}{label}:{:padding$} {value}", "")?;
    }
    Ok(())
}

/// The occasion one hazard warning reports — the CAUSE: a transition, made from one starting state, at
/// the input condition that state stands at. Detection files one record per (cause, outcome), so the
/// records sharing an occasion are the outcomes observed there, and the warning names them together.
#[derive(PartialEq, Eq, Hash)]
struct Occasion<'a> {
    cause: &'a Cause,
    condition: &'a Minterm<Symbol>,
    state: &'a Minterm<Symbol>,
}

impl<'a> Occasion<'a> {
    /// The occasion `hazard` was observed on.
    fn of(hazard: &'a Hazard) -> Self {
        Self {
            cause: &hazard.cause,
            condition: &hazard.condition,
            state: &hazard.state,
        }
    }
}

/// What one outcome does at an occasion: the nodes that reading puts at risk, and the states the
/// machine lands at once the timing is honoured. Both are gathered over the occasion's records of that
/// outcome — the victims unioned, each kept at the position it was first named, and the landings kept in
/// the order the records state them, since a pulse's are a sequence and a race's alternatives.
#[derive(Default)]
struct Effect<'a> {
    victims: Vec<&'a Symbol>,
    landings: Vec<&'a Minterm<Symbol>>,
}

/// One occasion's warning: a header naming what causes the hazard and the state it is caused from, over
/// a detail block that names the effect. `records` are the occasion's detected hazards, one per outcome
/// observed; the fields that follow from the occasion alone — its condition and the path into its state
/// — are the same in each, so they are read from the first, while each outcome contributes a field of
/// its own naming the nodes THAT reading puts at risk and where it leaves them.
fn hazard_warning<'a>(
    w: &mut impl io::Write,
    cell: &AnalysedCell,
    occasion: &Occasion,
    records: &[&'a Hazard],
) -> io::Result<()> {
    let first = records
        .first()
        .expect("an occasion is only entered by a record");
    // One entry per outcome, over the nodes and landing states every record of that outcome names.
    // `Outcome`'s own order sets the order the fields come out in, so a warning reads the same however
    // detection filed them.
    let mut effects: BTreeMap<Outcome, Effect<'a>> = BTreeMap::new();
    for h in records {
        let effect = effects.entry(h.outcome).or_default();
        for n in &h.group {
            if !effect.victims.contains(&n) {
                effect.victims.push(n);
            }
        }
        effect.landings.extend(&h.settled);
    }
    // Successive landings naming the same state are one place the machine comes to rest: a pulse's two
    // waypoints coincide wherever the closing edge moves nothing the outcome names, and reporting that
    // state twice would offer the reader two landings to tell apart where there is only one. A race's
    // are already distinct, detection holding them as a set.
    for effect in effects.values_mut() {
        effect.landings.dedup();
    }
    // The values every field is written from, held here so the field list can borrow them. `orders`
    // and `triggered by` each report one outcome, so each is present only where that outcome was
    // observed at this occasion.
    let when = first.condition();
    let path = report::Path(first.path());
    // A pulse returns its pin to the value it started from, so the pre-pulse input state IS the
    // condition the hazard occurs under — `when` states it, and a separate pre-hazard field would only
    // restate it. A toggle and a race leave their pins where they landed, so the state they started
    // from is worth naming.
    let pre_state = match occasion.cause {
        Cause::Toggle { .. } | Cause::Race { .. } => Some(State(first.pre_state())),
        Cause::Pulse { .. } => None,
    };
    // Which order the edges arrive in is what the settled state depends on, and ordering takes two of
    // them: a lone toggle has no second edge to arrive after.
    let pair = match occasion.cause {
        Cause::Race { pins } => Some(pins),
        Cause::Toggle { .. } | Cause::Pulse { .. } => None,
    };
    let orders = pair
        .filter(|_| effects.contains_key(&Outcome::Indeterminate))
        .map(orders_str);
    let trigger =
        trigger_str(occasion.cause).filter(|_| effects.contains_key(&Outcome::Oscillation));
    let landings: Vec<(&str, EffectField)> = effects
        .iter()
        .map(|(outcome, effect)| {
            (
                outcome_str(*outcome),
                EffectField {
                    cause: occasion.cause,
                    effect,
                },
            )
        })
        .collect();

    let mut fields: Vec<(&str, &dyn fmt::Display)> =
        vec![("when", &when), ("reached along", &path)];
    if let Some(pre_state) = &pre_state {
        fields.push(("pre-hazard", pre_state));
    }
    if let Some(orders) = &orders {
        fields.push(("orders", orders));
    }
    if let Some(trigger) = &trigger {
        fields.push(("triggered by", trigger));
    }
    fields.extend(
        landings
            .iter()
            .map(|(label, effect)| (*label, effect as &dyn fmt::Display)),
    );

    writeln!(
        w,
        "cellsmith: warning: cell {:?}: {} causes a hazard at {}",
        cell.repr_name(),
        cause_str(occasion.cause),
        State(occasion.state),
    )?;
    subblock(w, "    ", &fields)
}

/// What causes the hazard, as the header names it: the timing that has to be wrong for the cell to be
/// at risk, rather than the transition itself. A pulse is a hazard when it is too SHORT — exactly what
/// the generated minimum pulse width forbids — and a pair of edges when too little separates them, what
/// the generated setup/hold separation forbids. A lone toggle observed not to converge has no second
/// edge to be separated from, and no constraint follows from it, so there the transition is the whole of
/// the condition.
fn cause_str(cause: &Cause) -> String {
    match cause {
        Cause::Toggle { pin } => format!("toggling {pin}"),
        Cause::Race { pins: [a, b] } => format!("too little separation between {a} and {b}"),
        Cause::Pulse { pin, edge } => format!("a short pulse on {pin}{edge}"),
    }
}

/// The label an outcome's own field carries — the name it is reported under, beside the nodes that
/// reading puts at risk.
fn outcome_str(outcome: Outcome) -> &'static str {
    match outcome {
        Outcome::Indeterminate => "indeterminate",
        Outcome::Oscillation => "oscillation",
    }
}

/// One outcome's field value: the nodes it puts at risk, and — where the records name any — the states
/// the machine lands at once the timing IS honoured, which for a short pulse is where it would have gone
/// had the pulse been wide enough.
///
/// The landings are joined by what the cause makes them. An input cause's are ALTERNATIVES: either
/// winner is a legitimate result of separating the edges, and nothing orders them among themselves, so
/// they read as `or`. A pulse's are the two waypoints a wide enough one walks through — where the
/// opening edge's own cascade comes to rest, and then where the closing edge leaves the machine — so
/// they read with the same `→` the path field uses for a sequence.
///
/// The clause is absent, rather than empty, where the records name no landing at all: a lone toggle has
/// no second edge to be separated from, and a pair whose every order rings has no timing that brings the
/// machine to rest either. The header and `triggered by` already say which of the two it is.
struct EffectField<'a> {
    cause: &'a Cause,
    effect: &'a Effect<'a>,
}

impl fmt::Display for EffectField<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{{}}}", Commas(&self.effect.victims))?;
        if self.effect.landings.is_empty() {
            return Ok(());
        }
        let separator = match self.cause {
            Cause::Toggle { .. } | Cause::Race { .. } => " or ",
            Cause::Pulse { .. } => " → ",
        };
        f.write_str(" lands at ")?;
        for (i, state) in self.effect.landings.iter().enumerate() {
            if i > 0 {
                f.write_str(separator)?;
            }
            write!(f, "{}", State(state))?;
        }
        Ok(())
    }
}

/// The triggering transitions of an indeterminate race: the two orders its edges can arrive in, since
/// which lands first is what the settled state depends on (`A↓ then B↑ vs B↑ then A↓`).
fn orders_str([a, b]: &[Racer; 2]) -> String {
    format!("{a} then {b} vs {b} then {a}")
}

/// The triggering transition of an oscillating cause, where the cause names one: a pair arrives
/// together, which is what drives the cycle (`simultaneous toggle S↓ & R↓`), and a lone toggle arrives
/// with nothing to coincide with (`toggling A↓`). A pulse is its own two edges, which the header
/// already names in full.
fn trigger_str(cause: &Cause) -> Option<String> {
    match cause {
        Cause::Toggle { pin } => Some(format!("toggling {pin}")),
        Cause::Race { pins: [a, b] } => Some(format!("simultaneous toggle {a} & {b}")),
        Cause::Pulse { .. } => None,
    }
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

/// Write one artifact file into `dir`, reporting the path. The artifact renders itself into the file's
/// own writer, buffered because it arrives as the many small writes its `Display` makes.
fn write_file(dir: &Path, name: &str, body: &impl fmt::Display) -> io::Result<()> {
    let path = dir.join(name);
    let mut out = io::BufWriter::new(fs::File::create(&path)?);
    write!(out, "{body}")?;
    out.flush()?;
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

    #[test]
    fn constraints_is_a_bare_flag_and_keeps_the_positional() {
        let cli = Cli::try_parse_from(["cellsmith", "--constraints", "s.toml"]).unwrap();
        assert!(cli.constraints);
        assert_eq!(cli.spec, "s.toml");
    }

    #[test]
    fn constraints_absent_asks_for_none() {
        let cli = Cli::try_parse_from(["cellsmith", "s.toml"]).unwrap();
        assert!(!cli.constraints);
    }

    #[test]
    fn constraints_names_no_pin() {
        // Which pins one cell wants constraint arcs on is the spec's `constraint_arcs` to say, so the
        // flag names none: an `=PIN` value is unexpected, and a spaced one is a second positional.
        assert!(Cli::try_parse_from(["cellsmith", "--constraints=D", "s.toml"]).is_err());
        assert!(Cli::try_parse_from(["cellsmith", "--constraints", "D", "s.toml"]).is_err());
    }

    #[test]
    fn cli_constraints_selects_every_pin_over_the_cells_own() {
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
        let cli = Cli::try_parse_from(["cellsmith", "--constraints", "s.toml"]).unwrap();
        if cli.constraints {
            for c in &mut spec.cells {
                c.constraint_arcs = ConstraintPins::All;
            }
        }
        assert_eq!(
            spec.cells[0].constraint_arcs,
            ConstraintPins::All,
            "the flag subsumes the cell's own narrower selection",
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
        let mut out = Vec::new();
        banner(&mut out, "arcs.tcl", &"BODY").unwrap();
        assert_eq!(
            String::from_utf8(out).unwrap(),
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
