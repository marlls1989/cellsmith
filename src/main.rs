//! cellsmith CLI: read a minimal multi-cell TOML spec and emit, for every cell, the Liberate arcs
//! (`define_arc`), the structural Liberate `define_cell` blocks (`cells.tcl`), a
//! behavioural Verilog model (sequential UDP + wrapper), and a minimal Liberty fragment (`statetable`
//! for hysteretic outputs, plain `function` for combinational ones).

use std::collections::{BTreeMap, HashMap};
use std::convert::Infallible;
use std::fmt;
use std::fs;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

use clap::{Arg, ArgAction, ArgMatches, Args, Command, FromArgMatches, Parser};
use espresso_logic::{Minterm, Symbol};
use liberty_parser::liberty::{Group, Liberty};
use rayon::prelude::*;

use cellsmith::emit::arcs_tcl::{cell_arcs, ArcsTclOptions, CellArcs, Deck};
use cellsmith::emit::block::Description;
use cellsmith::emit::define_cell::{cell_define_cell, Declarations, DefineCell};
use cellsmith::emit::liberty::{cell_liberty, library_liberty};
use cellsmith::emit::verilog::{cell_verilog, Item, Verilog};
use cellsmith::logic::arcs::PinEdge;
use cellsmith::logic::hazard::{Cause, Hazard, Outcome};
use cellsmith::logic::machine::ExplorationBudget;
use cellsmith::model::{parse_spec, AnalysedCell, ArcClass, ArcClasses, ConstraintPins, Spec};
use cellsmith::report::{self, Commas, State};

/// Generate Cadence Liberate transition arcs, a behavioural Verilog model and a
/// Liberty fragment for logic cells, including state-holding/hysteretic cells.
#[derive(Parser)]
#[command(name = "cellsmith", version, about, long_about = None)]
struct Cli {
    /// TOML cell spec ("-" reads stdin).
    #[arg(value_parser = spec_source)]
    spec: PathArg,

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

/// A path argument that may instead name the standard stream: [`PathArg::StdStream`] is standard input
/// where the argument is read and standard output where it is written. Which of the two it means comes
/// from the site that consumes it, and that is what an `Option<PathBuf>` would leave unsaid.
#[derive(Debug, Clone, PartialEq, Eq)]
enum PathArg {
    File(PathBuf),
    StdStream,
}

/// The `<SPEC>` argument's value parser: `-` names standard input and every other argument is a path.
/// No argument is rejected, so the result is infallible, and it is a `Result` only because that is what
/// clap parses through.
fn spec_source(arg: &str) -> Result<PathArg, Infallible> {
    Ok(if arg == "-" {
        PathArg::StdStream
    } else {
        PathArg::File(PathBuf::from(arg))
    })
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

fn run(cli: Cli) -> io::Result<()> {
    let src = read_spec(&cli.spec)?;
    let mut spec = parse_spec(&src)?;
    apply_overrides(&mut spec, &cli);
    let budget = ExplorationBudget {
        candidates: cli.max_candidates,
        states: cli.max_states,
    };
    // A cell whose exploration stopped at a budget ceiling has no arcs, hazards, leakage states or
    // constraints — emitting its artifacts anyway would present that silence as the cell's behaviour —
    // so the analysis fails at an over-budget cell, whichever the parallel analysis reaches, and
    // nothing is written.
    let cells: Vec<AnalysedCell> = spec.analyse_with(&budget)?;

    let base = cli.name.unwrap_or_else(|| base_name(&cli.spec));
    let arc_opts = ArcsTclOptions {
        emit_internal: !cli.no_internal,
        emit_leakage: !cli.no_leakage,
    };
    // Rendered before the diagnostics, because one of them reports what the rendering could not say.
    let artifacts = artifacts(&cells, &base, arc_opts);

    // Buffered, because a warning reaches the handle as the many small writes composing it makes rather
    // than as one string, and stderr itself is unbuffered.
    let mut err = io::BufWriter::new(io::stderr().lock());
    diagnostics(&mut err, &cells, &artifacts.rendered)?;
    // The report is complete: flush it and release the handle before the artifacts are written, each of
    // which reports its path on this same stream.
    err.flush()?;
    drop(err);

    // Constraints avoid a hazard already reported by the warnings above, so the constraint arcs are
    // emitted (below, gated by the per-cell opt-in) without a separate diagnostic.

    // Where the artifacts go: one file per artifact under a directory, or all of them to standard
    // output behind banners. `--stdout` names the destination outright, so a directory given beside it
    // has nothing left to say.
    let destination = if cli.stdout {
        PathArg::StdStream
    } else {
        PathArg::File(cli.outdir)
    };
    match destination {
        PathArg::StdStream => {
            // Buffered, because an artifact reaches the handle as the many small writes its own
            // `Display` makes rather than as one string.
            let mut out = io::BufWriter::new(io::stdout().lock());
            emit_stdout(&mut out, &artifacts, cli.no_cells)?;
            out.flush()?;
        }
        PathArg::File(dir) => emit_files(&dir, &base, &artifacts, cli.no_cells)?,
    }
    Ok(())
}

/// Fold the command line's cell-level options into every cell of `spec`. Each is the CLI face of a key
/// the spec writes per cell, and folding them in here leaves one selection per cell for the analysis and
/// the emitters to read.
fn apply_overrides(spec: &mut Spec, cli: &Cli) {
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
}

/// Everything one run emits, rendered as values in cell order.
struct Artifacts<'a> {
    rendered: Vec<CellArcs>,
    model: Vec<Item<'a>>,
    liberty: Liberty,
    declarations: Vec<DefineCell>,
}

/// Render a run's artifacts from the analysed cells, under the arc emitter's `opts` and with `base`
/// naming the Liberty library. Nothing is written here: each artifact is the values it is made of, and
/// the text is composed at the writer it goes out on.
fn artifacts<'a>(cells: &'a [AnalysedCell], base: &str, opts: ArcsTclOptions) -> Artifacts<'a> {
    // Each artifact's values are stated for all the cells at once and flattened in cell order.
    let rendered: Vec<CellArcs> = cells.par_iter().map(|c| cell_arcs(c, opts)).collect();
    let model: Vec<Item> = cells.par_iter().flat_map_iter(cell_verilog).collect();
    let groups: Vec<Group> = cells.par_iter().flat_map_iter(cell_liberty).collect();
    // A Liberty document ends at the library group's closing brace, so the newline that ends the last
    // line of the artifact is the writer's: each sink states it alongside the document.
    let liberty = library_liberty(base, groups);
    let declarations: Vec<DefineCell> = cells.par_iter().flat_map_iter(cell_define_cell).collect();
    Artifacts {
        rendered,
        model,
        liberty,
        declarations,
    }
}

/// Report what the analysis found and what the rendering could not state, as the warnings a run writes
/// into `w`.
///
/// Each warning is one contiguous block of lines (a header plus its indented detail fields), written as
/// it is composed into the one handle; a blank line before every warning but the first keeps the blocks
/// reading as units.
fn diagnostics(
    w: &mut impl io::Write,
    cells: &[AnalysedCell],
    rendered: &[CellArcs],
) -> io::Result<()> {
    let mut warned = false;

    // Diagnose the cell's detected hazards, one warning per OCCASION — one cause, which is a transition
    // out of one starting state. Detection files a record per (cause, outcome), so an occasion showing
    // both outcomes arrives as two records; they are gathered here into the single entry whose body
    // names each outcome beside the nodes it puts at risk. The pass reads the ARC VIEW, the same
    // analysis `cell_arcs` renders: it is that view's hazards the emitted constraint arcs come from, so
    // reporting the other view's would describe arcs the run never wrote.
    for c in cells {
        let mut occasions: HashMap<Occasion, Vec<&Hazard>> = HashMap::new();
        for a in &c.arc_view().hazards {
            occasions.entry(Occasion::of(a)).or_default().push(a);
        }
        for (occasion, records) in &occasions {
            if std::mem::replace(&mut warned, true) {
                writeln!(w)?;
            }
            hazard_warning(w, c, occasion, records)?;
        }
    }

    // Diagnose the measurements no block could state: every block should express the cell state it
    // measures from, and its columns reach exactly its `-pinlist`, so a firing that differs only in an
    // internal node with no column renders a block already emitted. Exposing those nodes is the remedy,
    // which is why the warning names the state as well as the block.
    for (c, r) in cells.iter().zip(rendered) {
        if r.conflations.is_empty() {
            continue;
        }
        if std::mem::replace(&mut warned, true) {
            writeln!(w)?;
        }
        writeln!(
            w,
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
            let mut fields: Vec<SubblockField> = vec![SubblockField {
                label: "block",
                value: &block,
            }];
            fields.extend(states.iter().map(|s| SubblockField {
                label: "cell state",
                value: s,
            }));
            subblock(w, "  - ", &fields)?;
        }
    }
    Ok(())
}

/// Write every artifact into `out` behind its own section banner, the whole run on the one stream.
fn emit_stdout(out: &mut impl io::Write, a: &Artifacts, no_cells: bool) -> io::Result<()> {
    banner(out, "arcs.tcl", &Deck(&a.rendered))?;
    banner(out, "verilog", &Verilog(&a.model))?;
    banner(out, "liberty", &format_args!("{}\n", a.liberty))?;
    if !no_cells {
        banner(out, "cells.tcl", &Declarations(&a.declarations))?;
    }
    Ok(())
}

/// Write every artifact into a file of its own under `dir`, each named from `base`.
fn emit_files(dir: &Path, base: &str, a: &Artifacts, no_cells: bool) -> io::Result<()> {
    fs::create_dir_all(dir)?;
    write_file(dir, &format!("{base}_arcs.tcl"), &Deck(&a.rendered))?;
    write_file(dir, &format!("{base}.v"), &Verilog(&a.model))?;
    write_file(
        dir,
        &format!("{base}.lib"),
        &format_args!("{}\n", a.liberty),
    )?;
    if !no_cells {
        write_file(
            dir,
            &format!("{base}_cells.tcl"),
            &Declarations(&a.declarations),
        )?;
    }
    Ok(())
}

/// Read the spec's source text from wherever the argument named.
fn read_spec(spec: &PathArg) -> io::Result<String> {
    match spec {
        PathArg::StdStream => {
            let mut buf = String::new();
            io::stdin().read_to_string(&mut buf)?;
            Ok(buf)
        }
        PathArg::File(path) => fs::read_to_string(path),
    }
}

/// One artifact under its stdout section header, written into `out` as the artifact renders itself.
fn banner(out: &mut impl io::Write, kind: &str, body: &impl fmt::Display) -> io::Result<()> {
    writeln!(out, "// ===== cellsmith {kind} =====")?;
    writeln!(out, "{body}")
}

/// One field of a warning's subblock: the colon-labelled name written to stderr and the value rendered
/// beside it.
struct SubblockField<'a> {
    label: &'a str,
    value: &'a dyn fmt::Display,
}

/// Write one warning detail block: colon-labelled fields, indented under the header with their values
/// column-aligned. `lead` opens the first line — a hazard warning states one block and opens it at the
/// same indent as the rest, while the masked-arc warning states a block per conflated arc and bullets
/// each so the blocks read apart.
fn subblock(w: &mut impl io::Write, lead: &str, fields: &[SubblockField]) -> io::Result<()> {
    for (i, SubblockField { label, value }) in fields.iter().enumerate() {
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
        .map(Orders);
    let trigger =
        Trigger::of(occasion.cause).filter(|_| effects.contains_key(&Outcome::Oscillation));
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

    let mut fields: Vec<SubblockField> = vec![
        SubblockField {
            label: "when",
            value: &when,
        },
        SubblockField {
            label: "reached along",
            value: &path,
        },
    ];
    if let Some(pre_state) = &pre_state {
        fields.push(SubblockField {
            label: "pre-hazard",
            value: pre_state,
        });
    }
    if let Some(orders) = &orders {
        fields.push(SubblockField {
            label: "orders",
            value: orders,
        });
    }
    if let Some(trigger) = &trigger {
        fields.push(SubblockField {
            label: "triggered by",
            value: trigger,
        });
    }
    fields.extend(landings.iter().map(|(label, effect)| SubblockField {
        label,
        value: effect,
    }));

    writeln!(
        w,
        "cellsmith: warning: cell {:?}: {} causes a hazard at {}",
        cell.repr_name(),
        CauseHeader(occasion.cause),
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
struct CauseHeader<'a>(&'a Cause);

impl fmt::Display for CauseHeader<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            Cause::Toggle { pin } => write!(f, "toggling {pin}"),
            Cause::Race { pins: [a, b] } => write!(f, "too little separation between {a} and {b}"),
            Cause::Pulse { pin } => write!(f, "a short pulse on {pin}"),
        }
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
struct Orders<'a>(&'a [PinEdge; 2]);

impl fmt::Display for Orders<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let [a, b] = self.0;
        write!(f, "{a} then {b} vs {b} then {a}")
    }
}

/// The triggering transition of an oscillating cause: a pair arrives together, which is what drives the
/// cycle (`simultaneous toggle S↓ & R↓`), and a lone toggle arrives with nothing to coincide with
/// (`toggling A↓`). The variant is which of the two it is, so each carries the edges its own wording
/// names and no other.
enum Trigger<'a> {
    Toggle(&'a PinEdge),
    Simultaneous(&'a [PinEdge; 2]),
}

impl<'a> Trigger<'a> {
    /// The trigger `cause` names, or `None` where it names none: a pulse is its own two edges, which the
    /// header already states in full, so the warning carries no field for it.
    fn of(cause: &'a Cause) -> Option<Self> {
        match cause {
            Cause::Toggle { pin } => Some(Trigger::Toggle(pin)),
            Cause::Race { pins } => Some(Trigger::Simultaneous(pins)),
            Cause::Pulse { .. } => None,
        }
    }
}

impl fmt::Display for Trigger<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Trigger::Toggle(pin) => write!(f, "toggling {pin}"),
            Trigger::Simultaneous([a, b]) => write!(f, "simultaneous toggle {a} & {b}"),
        }
    }
}

/// The default output base name: the spec path's stem, or "cells" where the spec came from stdin and
/// there is no path to take a stem from.
fn base_name(spec: &PathArg) -> String {
    match spec {
        PathArg::StdStream => "cells".to_owned(),
        PathArg::File(path) => path
            .file_stem()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_else(|| "cells".to_owned()),
    }
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

    use cellsmith::emit::block::Block;

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
        assert_eq!(cli.spec, PathArg::File("s.toml".into()));
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
        assert_eq!(cli.spec, PathArg::File("s.toml".into()));
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

    /// `-` as the spec argument names the standard stream, which is where `read_spec` then reads the
    /// source from. The routing is what is stated here; the read itself is `io::stdin`'s, and the file
    /// arm is covered at [`read_spec_reads_a_file`].
    #[test]
    fn dash_names_the_standard_stream() {
        let cli = Cli::try_parse_from(["cellsmith", "--stdout", "-"]).unwrap();
        assert_eq!(cli.spec, PathArg::StdStream);
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
        apply_overrides(&mut spec, &cli);
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
        apply_overrides(&mut spec, &cli);
        assert_eq!(spec.cells[0].logic_high.as_deref(), Some("$VDDH"));
    }

    #[test]
    fn cli_when_unions_into_each_cells_own() {
        let mut spec = parse_spec(
            r#"
[[cell]]
name = "X"
inputs = ["A", "B"]
when = "transition"
[cell.outputs]
Y = "A*B"
"#,
        )
        .unwrap();
        let cli = Cli::try_parse_from(["cellsmith", "--when=hidden", "s.toml"]).unwrap();
        apply_overrides(&mut spec, &cli);
        let when = spec.cells[0].when;
        assert!(
            when.contains(ArcClass::Transition),
            "the cell keeps the class it selected itself",
        );
        assert!(
            when.contains(ArcClass::Hidden),
            "the CLI class is added to it",
        );
    }

    #[test]
    fn base_name_strips_dir_and_extension() {
        let file = |p: &str| base_name(&PathArg::File(p.into()));
        assert_eq!(file("/some/dir/cells.toml"), "cells");
        assert_eq!(file("cells.toml"), "cells");
        assert_eq!(file("plain"), "plain"); // no extension: the whole stem
        assert_eq!(base_name(&PathArg::StdStream), "cells"); // no path to take a stem from
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
        let got = read_spec(&PathArg::File(path.clone())).unwrap();
        assert_eq!(got, "hello = 1\n");
        fs::remove_file(&path).ok();
    }

    #[test]
    fn read_spec_errors_on_a_missing_path() {
        assert!(read_spec(&PathArg::File("/no/such/cellsmith/spec.toml".into())).is_err());
    }

    const C2: &str = r#"
[[cell]]
name = "C2"
inputs = ["A", "B"]
[cell.outputs]
Q = "A*B + Q*(A+B)"
"#;

    /// A unique scratch directory for one test, removed by the caller.
    fn scratch_dir(tag: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("cellsmith_cli_{tag}_{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn stdout_mode_emits_all_four_banners() {
        let cells = parse_spec(C2)
            .unwrap()
            .analyse_with(&ExplorationBudget::default())
            .unwrap();
        let a = artifacts(&cells, "cells", ArcsTclOptions::default());

        let mut out = Vec::new();
        emit_stdout(&mut out, &a, false).unwrap();
        let stdout = String::from_utf8(out).unwrap();

        assert!(stdout.contains("// ===== cellsmith arcs.tcl ====="));
        assert!(stdout.contains("// ===== cellsmith verilog ====="));
        assert!(stdout.contains("// ===== cellsmith liberty ====="));
        assert!(stdout.contains("// ===== cellsmith cells.tcl ====="));
        assert!(stdout.contains("define_arc"));
    }

    #[test]
    fn file_mode_writes_the_four_artifacts() {
        let dir = scratch_dir("file");
        let spec = dir.join("cells.toml");
        fs::write(&spec, C2).unwrap();
        let outdir = dir.join("out");

        let cli = Cli::try_parse_from([
            "cellsmith",
            "--outdir",
            outdir.to_str().unwrap(),
            "--name",
            "cli",
            spec.to_str().unwrap(),
        ])
        .unwrap();
        assert!(run(cli).is_ok());
        assert!(outdir.join("cli_arcs.tcl").is_file());
        assert!(outdir.join("cli.v").is_file());
        assert!(outdir.join("cli.lib").is_file());
        assert!(outdir.join("cli_cells.tcl").is_file());

        fs::remove_dir_all(&dir).ok();
    }

    const MULTI: &str = r#"
[[cell]]
name = "C2"
inputs = ["A", "B"]
[cell.outputs]
Q = "A*B + Q*(A+B)"

[[cell]]
name = "MUT"
inputs = ["A", "B"]
[cell.outputs]
Qa = "!Qb * A"
Qb = "!Qa * B"

[[cell]]
name = "DFF"
inputs = ["CLK", "D"]
clock = ["CLK"]
[cell.internal]
M = "!CLK*D + CLK*M"
[cell.outputs]
Q = "CLK*M + !CLK*Q"
"#;

    /// A 3-cell spec (C2, MUT, DFF) exercises the whole pipeline at once: every cell's three artifacts
    /// land in the stdout stream, and both hazard classes (MUT's oscillation, C2/DFF's order-dependent
    /// race) are diagnosed on the warning stream. Order-insensitive `contains` checks only — no
    /// full-output compare.
    #[test]
    fn multi_cell_spec_covers_all_cells() {
        let mut spec = parse_spec(MULTI).unwrap();
        let cli =
            Cli::try_parse_from(["cellsmith", "--constraints", "--stdout", "s.toml"]).unwrap();
        apply_overrides(&mut spec, &cli);
        let cells = spec.analyse_with(&ExplorationBudget::default()).unwrap();
        let a = artifacts(&cells, "cells", ArcsTclOptions::default());

        let mut warnings = Vec::new();
        diagnostics(&mut warnings, &cells, &a.rendered).unwrap();
        let warnings = String::from_utf8(warnings).unwrap();

        let mut out = Vec::new();
        emit_stdout(&mut out, &a, false).unwrap();
        let stdout = String::from_utf8(out).unwrap();

        assert!(stdout.contains("// ===== cellsmith arcs.tcl ====="));
        assert!(stdout.contains("// ===== cellsmith verilog ====="));
        assert!(stdout.contains("// ===== cellsmith liberty ====="));
        assert!(stdout.contains("define_arc"));
        assert!(stdout.contains("library ("));
        for cell in ["C2", "MUT", "DFF"] {
            assert!(stdout.contains(cell), "cell {cell} missing from stdout");
        }

        // A warning's header names the timing that causes the hazard, and its body one field per outcome
        // observed there — so a race reads as too little separation between its two edges, a pulse-width
        // hazard as a short pulse, and an oscillation is named where it was detected.
        assert!(
            warnings.contains("oscillation"),
            "no oscillation warning:\n{warnings}"
        );
        assert!(
            warnings.contains("too little separation between"),
            "no race warning:\n{warnings}"
        );
        assert!(
            warnings.contains("a short pulse on"),
            "no width-dependent hazard warning:\n{warnings}"
        );

        assert!(
            a.rendered
                .iter()
                .flat_map(|c| &c.blocks)
                .any(|b| matches!(b, Block::MinPulseWidth(_))),
            "no min_pulse_width constraint arcs",
        );
    }

    /// The warnings a run of `spec` reports: every cell analysed under the default budget and its blocks
    /// rendered, which is all the diagnostics read.
    fn diagnosed(spec: &str) -> String {
        let cells = parse_spec(spec)
            .unwrap()
            .analyse_with(&ExplorationBudget::default())
            .unwrap();
        let rendered: Vec<CellArcs> = cells
            .iter()
            .map(|c| cell_arcs(c, ArcsTclOptions::default()))
            .collect();
        let mut out = Vec::new();
        diagnostics(&mut out, &cells, &rendered).unwrap();
        String::from_utf8(out).unwrap()
    }

    /// One cause showing both outcomes is one warning entry. A mutex pulsed on `A↓` from `A*B` both
    /// settles indeterminately and rings, and detection files a record per outcome, so the two reach the
    /// report as a single entry whose body gives each outcome a field of its own, naming the nodes that
    /// reading puts at risk and where it leaves them.
    #[test]
    fn both_outcomes_at_one_cause_are_one_entry() {
        let warnings = diagnosed(MULTI);

        // Warnings are separated by a blank line, so one block is one entry.
        let entries: Vec<&str> = warnings
            .split("\n\n")
            .filter(|e| e.contains("cell \"MUT\"") && e.contains("a short pulse on A↓"))
            .collect();
        assert_eq!(entries.len(), 1, "MUT's A↓ pulse is one entry:\n{warnings}");
        let entry = entries[0];
        // Each outcome is a field of its own, over the nodes THAT reading decides: the mutex's coupled
        // grants both ways round.
        for outcome in ["indeterminate", "oscillation"] {
            assert!(
                entry
                    .lines()
                    .any(|l| l.trim_start().starts_with(&format!("{outcome}:"))
                        && l.contains("{Qa, Qb}")),
                "the entry names its {outcome} outcome over the nodes it decides:\n{entry}"
            );
        }
        // The header states the cause and the state it acts from; the nodes belong to the outcomes, which
        // need not agree on them.
        let header = entry.lines().next().expect("an entry has a header");
        assert!(
            !header.contains("nodes"),
            "the header carries no node set:\n{entry}"
        );
    }

    /// The value of the `label:` field in the one hazard entry whose header contains `header`. Warnings
    /// are separated by a blank line, so an entry is one block of the split.
    fn hazard_field<'a>(warnings: &'a str, header: &str, label: &str) -> &'a str {
        let entries: Vec<&str> = warnings
            .split("\n\n")
            .filter(|e| e.contains(header))
            .collect();
        assert_eq!(entries.len(), 1, "{header} names one entry:\n{warnings}");
        let prefix = format!("{label}:");
        entries[0]
            .lines()
            .find_map(|l| l.trim_start().strip_prefix(&prefix))
            .unwrap_or_else(|| panic!("no {label} field:\n{}", entries[0]))
            .trim_start()
    }

    /// Every hazard kind names where the machine lands, beside the nodes it attacks. That landing is
    /// `Hazard::settled` — for a race the results of its two orders, alternatives joined by `or`; for a
    /// pulse the two waypoints one wide enough walks through, in causal order and joined by `→`. Each
    /// expectation below is derived from the cell's own equations, and all four kinds are covered:
    /// race→indeterminate, race→oscillation, pulse→indeterminate and pulse→oscillation.
    #[test]
    fn every_hazard_kind_names_where_the_machine_lands() {
        let warnings = diagnosed(MULTI);

        // C2 (`Q = A*B + Q*(A+B)`) raced from `{A=1, B=0, Q=0}`: A↓ first leaves both inputs low, so Q stays
        // 0 and the later B↑ cannot lift it; B↑ first co-asserts the pair, which drives Q to 1, and the
        // later A↓ leaves Q holding on B. Either order is a legitimate settling, so the two read as
        // alternatives.
        assert_eq!(
            hazard_field(
                &warnings,
                r#"cell "C2": too little separation between A↓ and B↑ causes a hazard at {A=1, B=0, Q=0}"#,
                "indeterminate",
            ),
            "{Q} lands at {Q=0} or {Q=1}",
        );

        // MUT (`Qa = !Qb*A`, `Qb = !Qa*B`) with A↑ and B↑ separated from the idle state: whichever request
        // rises first takes its grant and locks the other out, so the ring settles to one grant or the
        // mirror.
        assert_eq!(
            hazard_field(
                &warnings,
                r#"cell "MUT": too little separation between A↑ and B↑ causes a hazard at {A=0, B=0, Qa=0, Qb=0}"#,
                "oscillation",
            ),
            "{Qa, Qb} lands at {Qa=0, Qb=1} or {Qa=1, Qb=0}",
        );

        // DFF (`M = !CLK*D + CLK*M`, `Q = CLK*M + !CLK*Q`) pulsed low on CLK from `{CLK=1, D=1, Q=0, M=0}`:
        // the opening CLK↓ opens the master and it takes D, resting at `{Q=0, M=1}`; the closing CLK↑ then
        // hands that to the slave, leaving `{Q=1, M=1}`. The two waypoints differ, and the pulse walks the
        // first to reach the second.
        assert_eq!(
            hazard_field(
                &warnings,
                r#"cell "DFF": a short pulse on CLK↓ causes a hazard at {CLK=1, D=1, Q=0, M=0}"#,
                "indeterminate",
            ),
            "{Q, M} lands at {Q=0, M=1} → {Q=1, M=1}",
        );

        // MUT pulsed low on A from `{A=1, B=1, Qa=1, Qb=0}`: A↓ drops A's grant and B's, waiting, takes it;
        // A↑ back finds B holding, so the machine is already where the closing edge leaves it and the two
        // waypoints name one landing. Both outcomes are observed here and both state it.
        for outcome in ["indeterminate", "oscillation"] {
            assert_eq!(
                hazard_field(
                    &warnings,
                    r#"cell "MUT": a short pulse on A↓ causes a hazard at {A=1, B=1, Qa=1, Qb=0}"#,
                    outcome,
                ),
                "{Qa, Qb} lands at {Qa=0, Qb=1}",
                "the {outcome} outcome states where a wide enough pulse lands",
            );
        }
    }

    /// A cell whose forced covers expand past the candidate ceiling: 10 inputs put 2^9 seed minterms in
    /// each of Y's two cover cubes, so `--max-candidates 512` stops the exploration and a raised ceiling
    /// lets the same cell through.
    const WIDE: &str = r#"
[[cell]]
name = "WIDE"
inputs = ["I0", "I1", "I2", "I3", "I4", "I5", "I6", "I7", "I8", "I9"]
[cell.outputs]
Y = "I0"
"#;

    #[test]
    fn candidate_budget_overrun_errors_and_writes_nothing() {
        let dir = scratch_dir("budget");
        let spec = dir.join("wide.toml");
        fs::write(&spec, WIDE).unwrap();
        let outdir = dir.join("out");

        let cli = Cli::try_parse_from([
            "cellsmith",
            "--outdir",
            outdir.to_str().unwrap(),
            "--max-candidates",
            "512",
            spec.to_str().unwrap(),
        ])
        .unwrap();
        let err =
            run(cli).expect_err("an exploration stopped at a budget is an error, not a warning");
        assert!(
            err.to_string().contains(
                "cell \"WIDE\": exploration stopped at the candidate budget \
                 (512 seed minterms) — raise it with --max-candidates"
            ),
            "missing the budget diagnostic:\n{err}"
        );
        // Nothing is emitted for a spec that could not be analysed: an arc-free artifact would read as
        // the cell's behaviour.
        let written: Vec<_> = fs::read_dir(&outdir)
            .map(|d| d.map(|e| e.unwrap().path()).collect())
            .unwrap_or_default();
        assert!(written.is_empty(), "artifacts written anyway: {written:?}");

        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn raising_the_candidate_budget_analyses_the_same_cell() {
        let dir = scratch_dir("budget_raised");
        let spec = dir.join("wide.toml");
        fs::write(&spec, WIDE).unwrap();
        let outdir = dir.join("out");

        let cli = Cli::try_parse_from([
            "cellsmith",
            "--outdir",
            outdir.to_str().unwrap(),
            "--max-candidates",
            "4096",
            spec.to_str().unwrap(),
        ])
        .unwrap();
        assert!(run(cli).is_ok());
        let arcs = fs::read_to_string(outdir.join("wide_arcs.tcl")).unwrap();
        assert!(arcs.contains("WIDE"), "cell missing from the arcs:\n{arcs}");
        assert!(
            arcs.contains("define_arc"),
            "the raised ceiling must let the arcs be derived:\n{arcs}"
        );

        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn a_bad_spec_is_an_error() {
        let dir = scratch_dir("bad");
        let spec = dir.join("bad.toml");
        // Undefined variable Z in the output function: a hard analysis error.
        fs::write(
            &spec,
            "[[cell]]\nname = \"X\"\ninputs = [\"A\"]\n[cell.outputs]\nY = \"A*Z\"\n",
        )
        .unwrap();

        let cli = Cli::try_parse_from(["cellsmith", "--stdout", spec.to_str().unwrap()]).unwrap();
        assert!(run(cli).is_err());

        fs::remove_dir_all(&dir).ok();
    }
}
