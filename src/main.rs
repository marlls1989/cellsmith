//! cellsmith CLI: read a minimal multi-cell TOML spec and emit, for every cell, the Liberate arcs
//! (`define_arc` + prevectors), the structural Liberate `define_cell` blocks (`cells.tcl`), a
//! behavioural Verilog model (sequential UDP + wrapper), and a minimal Liberty fragment (`statetable`
//! for hysteretic outputs, plain `function` for combinational ones).

use std::collections::BTreeMap;
use std::error::Error;
use std::fs;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

use clap::{Arg, ArgAction, ArgMatches, Args, Command, FromArgMatches, Parser};
use rayon::prelude::*;

use cellsmith::emit::arcs_tcl::{cell_arcs_tcl, ArcsTclOptions};
use cellsmith::emit::define_cell::cell_define_cell;
use cellsmith::emit::liberty::library_liberty;
use cellsmith::emit::verilog::cell_verilog;
use cellsmith::model::{parse_spec, AnalysedCell, ArcClass, ArcClasses};

/// Generate Cadence Liberate transition arcs (with prevectors), a behavioural Verilog model and a
/// Liberty fragment for logic cells, including state-holding/hysteretic cells.
#[derive(Parser)]
#[command(name = "cellsmith", version, about, long_about = None)]
struct Cli {
    /// TOML cell spec to read ("-" reads from stdin).
    spec: String,

    /// Directory for the generated files.
    #[arg(short, long, default_value = ".")]
    outdir: PathBuf,

    /// Base name for the output files (defaults to the spec file stem, or "cells" for stdin).
    #[arg(short, long)]
    name: Option<String>,

    /// The arc classes whose `-when` arcs are also emitted; the flag's help text lives with
    /// [`WhenArg`], as clap takes no help from the doc comment of a flattened field.
    #[command(flatten)]
    when: WhenArg,

    /// Suppress hidden (internal-power) arcs — input toggles where no output changes (emitted by default).
    #[arg(long)]
    no_internal: bool,

    /// Suppress `define_leakage` blocks — static leakage states derived from the machine's settled seed
    /// states (emitted by default).
    #[arg(long)]
    no_leakage: bool,

    /// Suppress the `<base>_cells.tcl` define_cell artifact (emitted by default).
    #[arg(long)]
    no_cells: bool,

    /// Emit derived setup/hold & non_seq constraint arcs (off by default; a cell can opt in with
    /// `constraint_arcs = true`).
    #[arg(long)]
    constraints: bool,

    /// Suppress the behavioural edge-register annotation (on by default); a cell can opt out
    /// individually with `no_edge_collapse = true`.
    #[arg(long)]
    no_edge_collapse: bool,

    /// Write all four artifacts to stdout (with banners) instead of writing files.
    #[arg(long)]
    stdout: bool,
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
        .help(
            "Also emit the `-when`-conditioned arcs, per arc class (off by default). The \
             deduplicated arcs without `-when` — one per emitted vector, keeping the shortest \
             prevector — are always emitted; a selected class adds its `-when` arcs on top, so an \
             arc can appear both with and without its condition. Bare `--when` selects every class; \
             `--when=hidden` / `--when=transition` select one; repeat the flag to select several. A \
             value must be attached with `=` (the space form is not accepted). A cell can select \
             classes itself with `when = ...`, and the two selections are unioned",
        )
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
    // `--constraints` is a blanket opt-in: it enables constraint-arc generation for every cell,
    // exactly as if each had declared `constraint_arcs = true`. Applied before analysis so the single
    // per-cell flag gates both generation and emission downstream.
    if cli.constraints {
        for c in &mut spec.cells {
            c.constraint_arcs = true;
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
    let cells: Vec<AnalysedCell> = spec.analyse()?;

    // Diagnose detected oscillation hazards: a periodic, non-settling cycle rather than a fixpoint,
    // naming the nodes (outputs or internals) that oscillate — the user should know, as this is never
    // expressed as deterministic timing.
    // Each warning is one contiguous block of lines (a header plus its indented detail fields);
    // distinct warnings are separated by a single blank line when printed, so a block reads as a unit.
    let mut warnings: Vec<String> = Vec::new();
    for c in &cells {
        for a in &c.oscillation {
            // The condition leads the sub-block as `when` (as in the race warning). How the machine
            // reached it — path into the pre-hazard state and the simultaneous toggle that triggers the
            // oscillation — comes from the representative pair-probe race (min by `(prevector.len,
            // discovered)`, matching the constraint tie-break). A single-toggle oscillation carries no
            // race, so only `when` is shown.
            let mut fields = vec![("when", a.condition_str())];
            if let Some(r) = a
                .races
                .iter()
                .min_by_key(|r| (r.prevector.len(), r.discovered))
            {
                fields.push(("reached along", r.path_str()));
                fields.push(("pre-hazard", r.pre_state_str()));
                fields.push((
                    "triggered by",
                    format!("simultaneous toggle {}", r.transition_str()),
                ));
            }
            let mut lines = vec![format!(
                "cellsmith: warning: cell {:?}: nodes {{{}}} oscillate",
                c.repr_name(),
                a.group.join(", "),
            )];
            lines.extend(subblock(&fields));
            warnings.push(lines.join("\n"));
        }
    }

    // Diagnose detected order-dependent hazards, grouped per racing pin pair: the settled state
    // depends on which of the pair's edges lands first (non-confluence). Each hazard on the pair is its
    // own `-`-bulleted sub-block (condition, path into the pre-hazard state, and the two settle orders
    // whose outcomes diverge), so multiple hazards on one pair read as a list.
    type RacePairs<'a> = BTreeMap<(&'a str, &'a str), Vec<Vec<String>>>;
    for c in &cells {
        let mut pairs: RacePairs = BTreeMap::new();
        for od in &c.order_dependence {
            let (x, y) = (od.x.as_str(), od.y.as_str());
            let key = if x <= y { (x, y) } else { (y, x) };
            pairs.entry(key).or_default().push(subblock(&[
                ("when", od.condition_str()),
                ("reached along", od.path_str()),
                ("pre-hazard", od.pre_state_str()),
                ("orders", od.transition_str()),
            ]));
        }
        for ((x, y), hazards) in &pairs {
            let mut lines = vec![format!(
                "cellsmith: warning: cell {:?}: inputs ({x}, {y}) race",
                c.repr_name(),
            )];
            lines.extend(hazards.iter().flatten().cloned());
            warnings.push(lines.join("\n"));
        }
    }

    if !warnings.is_empty() {
        eprintln!("{}", warnings.join("\n\n"));
    }

    // Constraints are the remedy for a hazard already reported by the oscillation and
    // order-dependence warnings above, so the constraint arcs are emitted (below, gated by the
    // per-cell opt-in) without a separate diagnostic.

    let arc_opts = ArcsTclOptions {
        emit_internal: !cli.no_internal,
        emit_leakage: !cli.no_leakage,
    };
    let base = cli.name.unwrap_or_else(|| base_name(&cli.spec));
    let arcs = render(&cells, |c| cell_arcs_tcl(c, arc_opts));
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
            when_classes(&["--when=hidden", "--when=transition"]),
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
    fn the_removed_no_when_flag_is_rejected() {
        // `--no-when` is gone, with no alias: the old flag is an unknown argument, so a stale script
        // fails loudly instead of silently getting the (now default) unconditional arcs.
        assert!(Cli::try_parse_from(["cellsmith", "--no-when", "s.toml"]).is_err());
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
