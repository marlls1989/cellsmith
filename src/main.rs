//! cellsmith CLI: read a minimal multi-cell TOML spec and emit, for every cell, the Liberate arcs
//! (`define_arc` + prevectors), a behavioural Verilog model (sequential UDP + wrapper), and a minimal
//! Liberty fragment (`statetable` for hysteretic outputs, plain `function` for combinational ones).

use std::collections::BTreeMap;
use std::error::Error;
use std::fs;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

use clap::Parser;
use rayon::prelude::*;

use cellsmith::emit::arcs_tcl::{cell_arcs_tcl, ArcsTclOptions};
use cellsmith::emit::liberty::library_liberty;
use cellsmith::emit::verilog::cell_verilog;
use cellsmith::model::{parse_spec, AnalysedCell};

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

    /// Suppress the `-when` conditions on arcs (emitted by default); collapses arcs that share a
    /// (related, pin, edge) to a single representative.
    #[arg(long)]
    no_when: bool,

    /// Suppress hidden (internal-power) arcs — input toggles where no output changes (emitted by default).
    #[arg(long)]
    no_internal: bool,

    /// Suppress `define_leakage` blocks — static leakage states derived from the machine's settled seed
    /// states (emitted by default).
    #[arg(long)]
    no_leakage: bool,

    /// Emit derived setup/hold & non_seq constraint arcs (off by default; a cell can opt in with
    /// `constraint_arcs = true`).
    #[arg(long)]
    constraints: bool,

    /// Suppress the behavioural edge-register annotation (on by default); a cell can opt out
    /// individually with `no_edge_collapse = true`.
    #[arg(long)]
    no_edge_collapse: bool,

    /// Write all three artifacts to stdout (with banners) instead of writing files.
    #[arg(long)]
    stdout: bool,
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
        emit_when: !cli.no_when,
        emit_internal: !cli.no_internal,
        emit_leakage: !cli.no_leakage,
    };
    let base = cli.name.unwrap_or_else(|| base_name(&cli.spec));
    let arcs = render(&cells, |c| cell_arcs_tcl(c, arc_opts));
    let verilog = render(&cells, cell_verilog);
    let liberty = library_liberty(&base, &cells);

    if cli.stdout {
        let mut out = io::stdout().lock();
        write!(out, "{}", banner("arcs.tcl", &arcs))?;
        write!(out, "{}", banner("verilog", &verilog))?;
        write!(out, "{}", banner("liberty", &liberty))?;
        return Ok(());
    }

    fs::create_dir_all(&cli.outdir)?;
    write_file(&cli.outdir, &format!("{base}_arcs.tcl"), &arcs)?;
    write_file(&cli.outdir, &format!("{base}.v"), &verilog)?;
    write_file(&cli.outdir, &format!("{base}.lib"), &liberty)?;
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
// `one` is only `Sync` (not `Send`); passing it by value into `par_iter().map()` would additionally
// require `Send`, so it is called through a closure that captures it by reference instead.
#[allow(clippy::redundant_closure)]
fn render(cells: &[AnalysedCell], one: impl (Fn(&AnalysedCell) -> String) + Sync) -> String {
    cells
        .par_iter()
        .map(|c| one(c))
        .collect::<Vec<String>>()
        .concat()
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
