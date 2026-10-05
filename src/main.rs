//! cellsmith CLI: read a minimal multi-cell TOML spec and emit, for every cell, the Liberate arcs
//! (`define_arc`), the structural Liberate `define_cell` blocks (`cells.tcl`), a
//! behavioural Verilog model (sequential UDP + wrapper), and a minimal Liberty fragment (`statetable`
//! for hysteretic outputs, plain `function` for combinational ones).

use std::collections::HashMap;
use std::convert::Infallible;
use std::ffi::OsString;
use std::fmt;
use std::fs;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

use clap::{Arg, ArgAction, ArgMatches, Args, Command, FromArgMatches, Parser};
use liberty_parser::liberty::{Group, Liberty};
use rayon::prelude::*;

use cellsmith::emit::arcs_tcl::{cell_arcs, ArcsTclOptions, CellArcs, Deck};
use cellsmith::emit::define_cell::{cell_define_cell, Declarations, DefineCell};
use cellsmith::emit::liberty::{cell_liberty, library_liberty};
use cellsmith::emit::verilog::{cell_verilog, Item, Verilog};
use cellsmith::logic::hazard::Hazard;
use cellsmith::logic::machine::ExplorationBudget;
use cellsmith::model::{parse_spec, AnalysedCell, ArcClass, ArcClasses, ConstraintPins, Spec};
use cellsmith::report::{conflation_warning, hazard_warning, Occasion};

/// Generate Cadence Liberate transition arcs, a behavioural Verilog model and a
/// Liberty fragment for logic cells, including state-holding/hysteretic cells.
#[derive(Parser)]
#[command(name = "cellsmith", version, about, long_about = None)]
struct Cli {
    /// TOML cell spec ("-" reads stdin).
    #[arg(value_parser = spec_source)]
    spec: PathArg,

    /// Where the artifacts go, resolved from `--stdout` and `-o/--outdir`; the flags' help text lives
    /// with [`PathArg`]'s [`Args`] implementation, as clap takes no help from the doc comment of a
    /// flattened field.
    #[command(flatten)]
    destination: PathArg,

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

/// The output destination's two flags. `--stdout` names the standard stream outright, so a directory
/// given beside it has nothing left to say; without it the artifacts go under `--outdir`.
fn destination_args() -> [Arg; 2] {
    [
        Arg::new("outdir")
            .short('o')
            .long("outdir")
            .value_name("OUTDIR")
            .value_parser(clap::value_parser!(PathBuf))
            .default_value(".")
            .action(ArgAction::Set)
            .help("Output directory"),
        Arg::new("stdout")
            .long("stdout")
            .action(ArgAction::SetTrue)
            .help("Write the artifacts to stdout instead of to files"),
    ]
}

/// The output destination, resolved while the arguments are parsed: [`PathArg::StdStream`] under
/// `--stdout`, and the `--outdir` directory otherwise.
impl Args for PathArg {
    fn augment_args(cmd: Command) -> Command {
        cmd.args(destination_args())
    }

    fn augment_args_for_update(cmd: Command) -> Command {
        cmd.args(destination_args())
    }
}

impl FromArgMatches for PathArg {
    fn from_arg_matches(matches: &ArgMatches) -> Result<Self, clap::Error> {
        Ok(if matches.get_flag("stdout") {
            PathArg::StdStream
        } else {
            PathArg::File(
                matches
                    .get_one::<PathBuf>("outdir")
                    .expect("`--outdir` carries a default")
                    .clone(),
            )
        })
    }

    fn update_from_arg_matches(&mut self, matches: &ArgMatches) -> Result<(), clap::Error> {
        *self = Self::from_arg_matches(matches)?;
        Ok(())
    }
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
        eprintln!("cellsmith: error: {e}");
        std::process::exit(1);
    }
}

fn run(cli: Cli) -> io::Result<()> {
    let src = read_spec(&cli.spec, io::stdin())?;
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
    // output behind banners.
    match cli.destination {
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
        conflation_warning(w, c, &r.conflations)?;
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

/// A failed file operation and the path it was made on. An `io::Error` states what went wrong and
/// nothing about where, and the path is the piece of context this layer holds: a run that cannot read
/// its spec or write an artifact says which file it meant.
#[derive(Debug)]
struct PathError {
    /// The path the operation was given.
    path: PathBuf,
    /// What the operation reported.
    source: io::Error,
}

impl PathError {
    /// The failure `source` reports, on `path`.
    fn at(path: &Path, source: io::Error) -> Self {
        Self {
            path: path.to_owned(),
            source,
        }
    }
}

impl fmt::Display for PathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.path.display(), self.source)
    }
}

impl std::error::Error for PathError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.source)
    }
}

impl From<PathError> for io::Error {
    fn from(err: PathError) -> Self {
        // Naming the path says nothing new about what went wrong, so the leaf error's kind is the
        // wrapper's: a caller matching on `NotFound` still sees it.
        io::Error::new(err.source.kind(), err)
    }
}

/// The `attempt`th name a file of this run's may take beside `artifact`. It sits in the artifact's own
/// directory, because a rename does not cross from one mount point to another; it is hidden, so nothing
/// reading the directory takes it for an artifact; and it carries the process id, so two runs writing
/// into one directory try different names.
fn sibling(artifact: &Path, attempt: u32) -> PathBuf {
    let name = artifact
        .file_name()
        .expect("an artifact's path ends in the file name built for it");
    let mut hidden = OsString::from(".");
    hidden.push(name);
    hidden.push(format!(".{}.{attempt}.tmp", std::process::id()));
    artifact.with_file_name(hidden)
}

/// A file this run created beside an artifact, under a name no other entry held.
struct Reserved {
    path: PathBuf,
    file: fs::File,
}

/// Create a file of this run's own beside `artifact`, at the first of its [`sibling`] names nothing
/// holds. `create_new` refuses any entry already at a name — a symlink included, so nothing is written
/// through one and nothing is truncated — and a refused name passes on to the next. A failure names the
/// artifact, the path the run was asked for.
fn reserve(artifact: &Path) -> io::Result<Reserved> {
    let mut attempt = 0;
    loop {
        let path = sibling(artifact, attempt);
        match fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
        {
            Ok(file) => return Ok(Reserved { path, file }),
            Err(e) if e.kind() == io::ErrorKind::AlreadyExists => attempt += 1,
            Err(e) => return Err(PathError::at(artifact, e).into()),
        }
    }
}

/// Move whatever `artifact`'s path holds to a name beside it, from which [`restore`] can put it back.
/// The result is that name, or `None` where the path held nothing. The name is reserved first, so the
/// rename replaces a file of this run's own and no other entry.
fn set_aside(artifact: &Path) -> io::Result<Option<PathBuf>> {
    let Reserved { path, .. } = reserve(artifact)?;
    match fs::rename(artifact, &path) {
        Ok(()) => Ok(Some(path)),
        Err(e) => {
            let _ = fs::remove_file(&path);
            match e.kind() {
                io::ErrorKind::NotFound => Ok(None),
                _ => Err(PathError::at(artifact, e).into()),
            }
        }
    }
}

/// Rename the entry [`set_aside`] moved to `previous` back onto `artifact`, replacing whatever this run
/// put there. It runs while a failure is being unwound, and that failure is the one the run reports, so
/// a restore that fails in turn is a warning naming where the entry is kept.
fn restore(previous: &Path, artifact: &Path) {
    if let Err(e) = fs::rename(previous, artifact) {
        eprintln!(
            "cellsmith: warning: {}: the previous artifact could not be put back and is kept at {}: {e}",
            artifact.display(),
            previous.display(),
        );
    }
}

/// One artifact on its way to disk: the file its text is written to, and the path it takes once every
/// artifact of the run has been written.
struct Staged {
    /// Where the text was written.
    temporary: PathBuf,
    /// The name the artifact is known by, which the temporary is renamed onto.
    artifact: PathBuf,
}

impl Staged {
    /// Rename the temporary onto the artifact, first setting aside whatever the artifact's path held.
    /// The result is where that entry went, or `None` where the path held nothing. A placement that
    /// fails has already put the entry back.
    fn place(&self) -> io::Result<Option<PathBuf>> {
        let previous = set_aside(&self.artifact)?;
        if let Err(e) = fs::rename(&self.temporary, &self.artifact) {
            if let Some(previous) = &previous {
                restore(previous, &self.artifact);
            }
            return Err(PathError::at(&self.artifact, e).into());
        }
        Ok(previous)
    }
}

/// An artifact a commit has renamed into place, and where the entry its path held was set aside —
/// `None` where the path held nothing.
struct Placed<'a> {
    artifact: &'a Path,
    previous: Option<PathBuf>,
}

impl Placed<'_> {
    /// Take this run's artifact back out, leaving its path as the run found it: the entry set aside is
    /// put back over it, and where there was none the artifact is removed.
    fn undo(&self) {
        match &self.previous {
            Some(previous) => restore(previous, self.artifact),
            None => {
                if let Err(e) = fs::remove_file(self.artifact) {
                    eprintln!(
                        "cellsmith: warning: {}: this run's artifact could not be removed: {e}",
                        self.artifact.display(),
                    );
                }
            }
        }
    }
}

/// A run's artifacts on their way to disk. Each is written to a temporary file beside the artifact it
/// becomes, and the temporaries are renamed onto the artifacts once they all exist. Placing one first
/// sets aside whatever its path held; where any placement fails, every entry set aside is put back and
/// every artifact placed without one is removed. So once a run ends, the artifacts it writes are either
/// all this run's or all as they stood before it — a Liberate run reading a directory a half-finished
/// run left behind has no way to tell which artifact belongs to which. That holds over the artifacts the
/// run writes and no others: under `--no-cells` the run writes no `<base>_cells.tcl`, and one already in
/// the directory stays as it is. Putting an entry back is itself a rename, and where that fails the run
/// warns, naming where the entry is kept. A staging dropped before it is committed removes the files it
/// wrote.
#[derive(Default)]
struct Staging {
    written: Vec<Staged>,
}

impl Staging {
    /// Write the text of the artifact at `artifact` into a temporary of its own beside it. The artifact
    /// renders itself into that file's writer, buffered because it arrives as the many small writes its
    /// `Display` makes. A failure names the artifact, the path the run was asked for.
    fn write(&mut self, artifact: &Path, body: &impl fmt::Display) -> io::Result<()> {
        let Reserved { path, file } = reserve(artifact)?;
        // Staged from the moment the file exists: whatever fails below, the drop has the path to remove.
        self.written.push(Staged {
            temporary: path,
            artifact: artifact.to_owned(),
        });
        let mut out = io::BufWriter::new(file);
        write!(out, "{body}").map_err(|e| PathError::at(artifact, e))?;
        out.flush().map_err(|e| PathError::at(artifact, e))?;
        Ok(())
    }

    /// Rename every staged file onto the artifact it was written for, and report each path once they
    /// have all landed. Where one cannot be placed, every artifact already placed is undone before the
    /// failure is returned.
    fn commit(mut self) -> io::Result<()> {
        let mut placed: Vec<Placed> = Vec::with_capacity(self.written.len());
        for staged in &self.written {
            match staged.place() {
                Ok(previous) => placed.push(Placed {
                    artifact: &staged.artifact,
                    previous,
                }),
                Err(e) => {
                    for p in &placed {
                        p.undo();
                    }
                    return Err(e);
                }
            }
        }
        // Every artifact is in place, so nothing set aside will be put back. Removing those entries is
        // best-effort, as the drop's is: the artifacts are what the run delivers.
        for Placed { artifact, previous } in &placed {
            if let Some(previous) = previous {
                let _ = fs::remove_file(previous);
            }
            eprintln!("wrote {}", artifact.display());
        }
        // The temporaries were renamed onto the artifacts; nothing is left for the drop to remove.
        self.written.clear();
        Ok(())
    }
}

impl Drop for Staging {
    fn drop(&mut self) {
        // A staging still holding files is one whose run failed, and the failure is what gets reported:
        // removing what it wrote is best-effort, and a temporary already renamed away is simply absent.
        for Staged { temporary, .. } in &self.written {
            let _ = fs::remove_file(temporary);
        }
    }
}

/// Write every artifact into a file of its own at `dir` joined with a name built from `base`, and
/// rename them into place together once they are all written. A `base` with a directory component
/// places the artifacts in that directory under `dir`, which has to exist already.
fn emit_files(dir: &Path, base: &str, a: &Artifacts, no_cells: bool) -> io::Result<()> {
    fs::create_dir_all(dir).map_err(|e| PathError::at(dir, e))?;
    let mut staging = Staging::default();
    staging.write(&dir.join(format!("{base}_arcs.tcl")), &Deck(&a.rendered))?;
    staging.write(&dir.join(format!("{base}.v")), &Verilog(&a.model))?;
    staging.write(
        &dir.join(format!("{base}.lib")),
        &format_args!("{}\n", a.liberty),
    )?;
    if !no_cells {
        staging.write(
            &dir.join(format!("{base}_cells.tcl")),
            &Declarations(&a.declarations),
        )?;
    }
    staging.commit()
}

/// Read the spec's source text from wherever the argument named. The standard stream is `stdin`, a
/// parameter so that a test can supply the text.
fn read_spec(spec: &PathArg, mut stdin: impl Read) -> io::Result<String> {
    match spec {
        PathArg::StdStream => {
            let mut buf = String::new();
            stdin.read_to_string(&mut buf)?;
            Ok(buf)
        }
        PathArg::File(path) => fs::read_to_string(path).map_err(|e| PathError::at(path, e).into()),
    }
}

/// One artifact under its stdout section header, written into `out` as the artifact renders itself.
fn banner(out: &mut impl io::Write, kind: &str, body: &impl fmt::Display) -> io::Result<()> {
    writeln!(out, "// ===== cellsmith {kind} =====")?;
    writeln!(out, "{body}")
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

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

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
    /// source from. The routing is what is stated here; the read through the stream is covered at
    /// [`read_spec_reads_the_standard_stream`], and the file arm at [`read_spec_reads_a_file`].
    #[test]
    fn dash_names_the_standard_stream() {
        let cli = Cli::try_parse_from(["cellsmith", "--stdout", "-"]).unwrap();
        assert_eq!(cli.spec, PathArg::StdStream);
    }

    /// A spec named `-` is read from the standard stream: the source text is what the stream holds.
    #[test]
    fn read_spec_reads_the_standard_stream() {
        let cli = Cli::try_parse_from(["cellsmith", "-"]).unwrap();
        let got = read_spec(&cli.spec, C2.as_bytes()).unwrap();
        assert_eq!(got, C2);
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
        let got = read_spec(&PathArg::File(path.clone()), io::empty()).unwrap();
        assert_eq!(got, "hello = 1\n");
        fs::remove_file(&path).ok();
    }

    /// A spec that cannot be read fails naming the path it was given: the os error says only what went
    /// wrong, and which file it was asked for is what the caller needs to act on it.
    #[test]
    fn read_spec_errors_on_a_missing_path() {
        let path = "/no/such/cellsmith/spec.toml";
        let err = read_spec(&PathArg::File(path.into()), io::empty())
            .expect_err("a missing spec has no source text to read");
        assert!(
            err.to_string().contains(path),
            "the error names the path it was given:\n{err}"
        );
        assert_eq!(err.kind(), io::ErrorKind::NotFound);
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
        let artifacts = ["cli_arcs.tcl", "cli.v", "cli.lib", "cli_cells.tcl"];
        for name in artifacts {
            assert!(outdir.join(name).is_file(), "{name} was not written");
        }
        // Each artifact is renamed into place from a temporary that the run does not outlive, so the
        // four are the whole of what a completed run leaves behind.
        assert_eq!(entries(&outdir), names(&artifacts));

        fs::remove_dir_all(&dir).ok();
    }

    /// The names of the entries `dir` holds.
    fn entries(dir: &Path) -> HashSet<String> {
        fs::read_dir(dir)
            .unwrap()
            .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
            .collect()
    }

    /// `list` as the set [`entries`] compares against.
    fn names(list: &[&str]) -> HashSet<String> {
        list.iter().map(|n| n.to_string()).collect()
    }

    /// A combinational cell, for the tests about where a run writes, which need a spec that analyses
    /// cleanly and say nothing about what it holds.
    const INV: &str = r#"
[[cell]]
name = "INV"
inputs = ["A"]
[cell.outputs]
Y = "!A"
"#;

    /// `--stdout` names the destination outright, so a directory given beside it has nothing left to
    /// say: a run given both lands no artifact in the directory.
    #[test]
    fn stdout_takes_precedence_over_the_outdir() {
        let dir = scratch_dir("stdout_outdir");
        let spec = dir.join("inv.toml");
        fs::write(&spec, INV).unwrap();
        let outdir = dir.join("out");
        fs::create_dir(&outdir).unwrap();

        let cli = Cli::try_parse_from([
            "cellsmith",
            "--stdout",
            "--outdir",
            outdir.to_str().unwrap(),
            spec.to_str().unwrap(),
        ])
        .unwrap();
        run(cli).expect("the spec analyses cleanly");
        assert!(
            entries(&outdir).is_empty(),
            "an artifact landed in the directory: {:?}",
            entries(&outdir),
        );

        fs::remove_dir_all(&dir).ok();
    }

    /// A `--name` with a directory component names a directory under the output one, and the artifacts
    /// land there.
    #[test]
    fn a_name_with_a_directory_writes_into_that_directory() {
        let dir = scratch_dir("sub_name");
        let spec = dir.join("inv.toml");
        fs::write(&spec, INV).unwrap();
        let outdir = dir.join("out");
        let sub = outdir.join("sub");
        fs::create_dir_all(&sub).unwrap();

        let cli = Cli::try_parse_from([
            "cellsmith",
            "--outdir",
            outdir.to_str().unwrap(),
            "--name",
            "sub/base",
            spec.to_str().unwrap(),
        ])
        .unwrap();
        run(cli).expect("the named directory exists");
        assert_eq!(
            entries(&sub),
            names(&["base_arcs.tcl", "base.v", "base.lib", "base_cells.tcl"]),
        );
        assert_eq!(entries(&outdir), names(&["sub"]));

        fs::remove_dir_all(&dir).ok();
    }

    /// An entry already at the name a temporary would take — here a symlink to a file outside the
    /// output directory — is neither followed nor truncated: the run passes over the name for another,
    /// succeeds, and leaves the entry as it found it.
    #[cfg(unix)]
    #[test]
    fn an_entry_at_a_temporarys_name_is_left_alone() {
        let dir = scratch_dir("planted");
        let spec = dir.join("inv.toml");
        fs::write(&spec, INV).unwrap();
        let victim = dir.join("victim.txt");
        fs::write(&victim, "not an artifact").unwrap();
        let outdir = dir.join("out");
        fs::create_dir(&outdir).unwrap();
        let artifacts = ["cli_arcs.tcl", "cli.v", "cli.lib", "cli_cells.tcl"];
        // The first name each artifact's temporary would take, every one a symlink to the victim.
        let planted: Vec<PathBuf> = artifacts
            .iter()
            .map(|n| sibling(&outdir.join(n), 0))
            .collect();
        for p in &planted {
            std::os::unix::fs::symlink(&victim, p).unwrap();
        }

        let cli = Cli::try_parse_from([
            "cellsmith",
            "--outdir",
            outdir.to_str().unwrap(),
            "--name",
            "cli",
            spec.to_str().unwrap(),
        ])
        .unwrap();
        run(cli).expect("a taken name is passed over, not written through");

        assert_eq!(fs::read_to_string(&victim).unwrap(), "not an artifact");
        for p in &planted {
            assert_eq!(
                fs::read_link(p).unwrap(),
                victim,
                "{} was replaced",
                p.display()
            );
        }
        for name in artifacts {
            let file_type = fs::symlink_metadata(outdir.join(name)).unwrap().file_type();
            assert!(file_type.is_file(), "{name} is not a file of its own");
        }
        // The planted entries stay beside the artifacts, and nothing else of the run's does.
        let mut expected = names(&artifacts);
        expected.extend(
            planted
                .iter()
                .map(|p| p.file_name().unwrap().to_string_lossy().into_owned()),
        );
        assert_eq!(entries(&outdir), expected);

        fs::remove_dir_all(&dir).ok();
    }

    /// A commit that fails part-way leaves every path it was placing as it found it. The three artifacts
    /// are the three cases a placement meets: one replacing a file, one landing where nothing was, and
    /// one whose path holds a non-empty directory, which no file can be renamed onto. Staged last, that
    /// one fails once the other two are in place.
    #[test]
    fn a_commit_failing_part_way_leaves_the_previous_artifacts() {
        let dir = scratch_dir("commit_fails");
        let replacing = dir.join("replacing.txt");
        fs::write(&replacing, "previous run").unwrap();
        let landing = dir.join("landing.txt");
        let blocked = dir.join("blocked.txt");
        fs::create_dir(&blocked).unwrap();
        fs::write(blocked.join("kept.txt"), "previous run").unwrap();
        let before = entries(&dir);

        let mut staging = Staging::default();
        for artifact in [&replacing, &landing, &blocked] {
            staging.write(artifact, &"this run").unwrap();
        }
        let err = staging
            .commit()
            .expect_err("no file can be renamed onto a non-empty directory");
        let named = err
            .get_ref()
            .and_then(|e| e.downcast_ref::<PathError>())
            .expect("the failure carries the path it was made on");
        assert_eq!(
            named.path, blocked,
            "the failure names the blocked artifact"
        );

        assert_eq!(fs::read_to_string(&replacing).unwrap(), "previous run");
        assert!(
            fs::symlink_metadata(&landing).is_err(),
            "this run's artifact stayed where nothing was",
        );
        assert_eq!(
            fs::read_to_string(blocked.join("kept.txt")).unwrap(),
            "previous run"
        );
        // Neither this run's temporaries nor the entries it set aside are left behind.
        assert_eq!(entries(&dir), before);

        fs::remove_dir_all(&dir).ok();
    }

    /// A commit that succeeds replaces what each path held with this run's artifact, and keeps no copy
    /// of what it replaced.
    #[test]
    fn a_commit_replaces_the_previous_artifacts() {
        let dir = scratch_dir("commit_replaces");
        let replacing = dir.join("replacing.txt");
        fs::write(&replacing, "previous run").unwrap();
        let landing = dir.join("landing.txt");

        let mut staging = Staging::default();
        for artifact in [&replacing, &landing] {
            staging.write(artifact, &"this run").unwrap();
        }
        staging.commit().expect("both paths take a file");

        for artifact in [&replacing, &landing] {
            assert_eq!(fs::read_to_string(artifact).unwrap(), "this run");
        }
        assert_eq!(entries(&dir), names(&["replacing.txt", "landing.txt"]));

        fs::remove_dir_all(&dir).ok();
    }

    /// A write that fails names the artifact it was writing, the path the run was asked for, and not
    /// the temporary beside it.
    #[test]
    fn a_failed_write_names_the_artifact() {
        let dir = scratch_dir("write_fails");
        let artifact = dir.join("missing").join("cli.v");

        let mut staging = Staging::default();
        let err = staging
            .write(&artifact, &"this run")
            .expect_err("no file can be created in a directory that does not exist");
        let named = err
            .get_ref()
            .and_then(|e| e.downcast_ref::<PathError>())
            .expect("the failure carries the path it was made on");
        assert_eq!(named.path, artifact);

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
                 (512 seed minterms); no arcs, hazards, leakage states or constraints are derived \
                 — raise it with --max-candidates"
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
