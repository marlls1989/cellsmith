//! lobsterate CLI: read a minimal multi-cell TOML spec and emit, for every cell, the Liberate arcs
//! (`define_arc` + prevectors), a behavioural Verilog model (sequential UDP + wrapper), and a minimal
//! Liberty fragment (`statetable` for hysteretic outputs, plain `function` for combinational ones).

use std::error::Error;
use std::fs;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

use clap::Parser;

use lobsterate::emit::arcs_tcl::{cell_arcs_tcl, ArcsTclOptions};
use lobsterate::emit::liberty::cell_liberty;
use lobsterate::emit::verilog::cell_verilog;
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

    /// Emit `-when` conditions on the arcs (off by default).
    #[arg(long)]
    when: bool,

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

    let arc_opts = ArcsTclOptions {
        emit_when: cli.when,
    };
    let arcs = render(&cells, |c| Ok(cell_arcs_tcl(c, arc_opts)?))?;
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
