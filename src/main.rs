//! lobsterate CLI: read a minimal multi-cell TOML spec and emit, for every cell, the Liberate arcs
//! (`define_arc` + prevectors), a behavioural Verilog model (sequential UDP + wrapper), and a minimal
//! Liberty fragment (`statetable` for hysteretic outputs, plain `function` for combinational ones).

use std::collections::BTreeMap;
use std::error::Error;
use std::fs;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

use clap::Parser;

use lobsterate::emit::arcs_tcl::{cell_arcs_tcl, ArcsTclOptions};
use lobsterate::emit::liberty::cell_liberty;
use lobsterate::emit::verilog::cell_verilog;
use lobsterate::logic::confluence;
use lobsterate::model::{parse_spec, AnalysedCell};

/// Generate Cadence Liberate transition arcs (with prevectors), a behavioural Verilog model and a
/// Liberty fragment for logic cells, including state-holding/hysteretic cells.
#[derive(Parser)]
#[command(name = "lobsterate", version, about, long_about = None)]
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

    /// Emit derived setup/hold & non_seq constraint arcs (off by default; a cell can opt in with
    /// `constraint_arcs = true`).
    #[arg(long)]
    constraints: bool,

    /// Write all three artifacts to stdout (with banners) instead of writing files.
    #[arg(long)]
    stdout: bool,
}

fn main() {
    if let Err(e) = run(Cli::parse()) {
        eprintln!("lobsterate: {e}");
        std::process::exit(1);
    }
}

fn run(cli: Cli) -> Result<(), Box<dyn Error>> {
    let src = read_spec(&cli.spec)?;
    let cells: Vec<AnalysedCell> = parse_spec(&src)?
        .cells
        .iter()
        .map(|c| c.analyse())
        .collect::<Result<_, _>>()?;

    // Diagnose interlocked cells: their arbitration is detected and annotated, but never expressed as
    // deterministic timing, so the user should know — naming the nodes (outputs or internals) that
    // arbitrate.
    for c in &cells {
        for a in &c.arbitration {
            eprintln!(
                "lobsterate: warning: cell {:?}: nodes {{{}}} arbitrate (metastable at {}) — \
                 annotated only, not modelled as timing.",
                c.name,
                a.group.join(", "),
                a.condition_str(),
            );
        }
    }

    // Diagnose every derived constraint, for any cell, listing the hazard conditions that require it.
    // Each input pin pair is uniformly one kind (setup/hold if it holds a declared clock, else
    // non_seq), so its conditions are gathered and reported once.
    type ConstraintPairs<'a> = BTreeMap<(&'a str, &'a str), (&'static str, Vec<String>)>;
    for c in &cells {
        let mut pairs: ConstraintPairs = BTreeMap::new();
        for con in &c.constraints {
            let (a, b) = (con.related.as_str(), con.pin.as_str());
            let key = if a <= b { (a, b) } else { (b, a) };
            let kind = match con.kind {
                confluence::ConstraintKind::SetupHold => "setup/hold",
                confluence::ConstraintKind::NonSeq => "non_seq",
            };
            pairs
                .entry(key)
                .or_insert((kind, Vec::new()))
                .1
                .push(con.condition());
        }
        for ((a, b), (kind, conditions)) in &pairs {
            eprintln!(
                "lobsterate: warning: cell {:?}: inputs ({a}, {b}) need a {kind} constraint — hazard when {}.",
                c.name,
                conditions.join("; "),
            );
        }
    }

    let arc_opts = ArcsTclOptions {
        emit_when: !cli.no_when,
        emit_constraints: cli.constraints,
    };
    let arcs = render(&cells, |c| Ok(cell_arcs_tcl(c, arc_opts)))?;
    let verilog = render(&cells, |c| Ok(cell_verilog(c)))?;
    let liberty = render(&cells, |c| Ok(cell_liberty(c)))?;

    if cli.stdout {
        let mut out = io::stdout().lock();
        write!(out, "{}", banner("arcs.tcl", &arcs))?;
        write!(out, "{}", banner("verilog", &verilog))?;
        write!(out, "{}", banner("liberty", &liberty))?;
        return Ok(());
    }

    let base = cli.name.unwrap_or_else(|| base_name(&cli.spec));
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

/// Concatenate one artifact across every cell, propagating the first emitter error.
fn render(
    cells: &[AnalysedCell],
    mut one: impl FnMut(&AnalysedCell) -> Result<String, Box<dyn Error>>,
) -> Result<String, Box<dyn Error>> {
    let mut out = String::new();
    for cell in cells {
        out.push_str(&one(cell)?);
    }
    Ok(out)
}

/// A stdout section banner for one artifact.
fn banner(kind: &str, body: &str) -> String {
    format!("// ===== lobsterate {kind} =====\n{body}\n")
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
