//! CLI integration checks driving the built binary directly (no extra dependencies): stdout mode and
//! its banners, file mode and its three artifacts, stdin (`-`), and the non-zero exit on a bad spec.

use std::io::Write;
use std::process::{Command, Stdio};

/// The binary under test, provided by Cargo for integration tests.
const BIN: &str = env!("CARGO_BIN_EXE_cellsmith");

const C2: &str = r#"
[[cell]]
name = "C2"
inputs = ["A", "B"]
[cell.outputs]
Q = "A*B + Q*(A+B)"
"#;

/// A unique scratch directory for one test, removed by the caller.
fn scratch_dir(tag: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("cellsmith_cli_{tag}_{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

#[test]
fn stdout_mode_emits_all_three_banners() {
    let dir = scratch_dir("stdout");
    let spec = dir.join("cells.toml");
    std::fs::write(&spec, C2).unwrap();

    let out = Command::new(BIN)
        .arg("--stdout")
        .arg(&spec)
        .output()
        .expect("run cellsmith");
    assert!(out.status.success(), "exit: {:?}", out.status);
    let stdout = String::from_utf8(out.stdout).unwrap();
    assert!(stdout.contains("// ===== cellsmith arcs.tcl ====="));
    assert!(stdout.contains("// ===== cellsmith verilog ====="));
    assert!(stdout.contains("// ===== cellsmith liberty ====="));
    assert!(stdout.contains("define_arc"));

    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn file_mode_writes_the_three_artifacts() {
    let dir = scratch_dir("file");
    let spec = dir.join("cells.toml");
    std::fs::write(&spec, C2).unwrap();
    let outdir = dir.join("out");

    let status = Command::new(BIN)
        .arg("--outdir")
        .arg(&outdir)
        .arg("--name")
        .arg("cli")
        .arg(&spec)
        .status()
        .expect("run cellsmith");
    assert!(status.success());
    assert!(outdir.join("cli_arcs.tcl").is_file());
    assert!(outdir.join("cli.v").is_file());
    assert!(outdir.join("cli.lib").is_file());

    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn stdin_dash_reads_the_spec() {
    let mut child = Command::new(BIN)
        .arg("--stdout")
        .arg("-")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("spawn cellsmith");
    child
        .stdin
        .take()
        .unwrap()
        .write_all(C2.as_bytes())
        .unwrap();
    let out = child.wait_with_output().expect("wait cellsmith");
    assert!(out.status.success());
    let stdout = String::from_utf8(out.stdout).unwrap();
    assert!(stdout.contains("// ===== cellsmith arcs.tcl ====="));
    assert!(stdout.contains("define_arc"));
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
/// land in stdout, and both hazard classes (MUT's oscillation, C2/DFF's order-dependent race) are
/// diagnosed on stderr. Order-insensitive `contains` checks only — no full-output compare.
#[test]
fn multi_cell_spec_covers_all_cells() {
    let dir = scratch_dir("multi");
    let spec = dir.join("cells.toml");
    std::fs::write(&spec, MULTI).unwrap();

    let out = Command::new(BIN)
        .arg("--stdout")
        .arg("--constraints")
        .arg(&spec)
        .output()
        .expect("run cellsmith");
    assert!(out.status.success(), "exit: {:?}", out.status);

    let stdout = String::from_utf8(out.stdout).unwrap();
    assert!(stdout.contains("// ===== cellsmith arcs.tcl ====="));
    assert!(stdout.contains("// ===== cellsmith verilog ====="));
    assert!(stdout.contains("// ===== cellsmith liberty ====="));
    assert!(stdout.contains("define_arc"));
    assert!(stdout.contains("library ("));
    for cell in ["C2", "MUT", "DFF"] {
        assert!(stdout.contains(cell), "cell {cell} missing from stdout");
    }

    let stderr = String::from_utf8(out.stderr).unwrap();
    assert!(
        stderr.contains("oscillate"),
        "no oscillation warning:\n{stderr}"
    );
    assert!(stderr.contains("race"), "no race warning:\n{stderr}");

    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn bad_spec_exits_non_zero() {
    let dir = scratch_dir("bad");
    let spec = dir.join("bad.toml");
    // Undefined variable Z in the output function: a hard analysis error.
    std::fs::write(
        &spec,
        "[[cell]]\nname = \"X\"\ninputs = [\"A\"]\n[cell.outputs]\nY = \"A*Z\"\n",
    )
    .unwrap();

    let status = Command::new(BIN)
        .arg("--stdout")
        .arg(&spec)
        .status()
        .expect("run cellsmith");
    assert!(!status.success(), "a bad spec must exit non-zero");

    std::fs::remove_dir_all(&dir).ok();
}
