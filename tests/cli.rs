//! CLI integration checks driving the built binary directly (no extra dependencies): stdout mode and
//! its banners, file mode and its four artifacts, stdin (`-`), and the non-zero exit on a bad spec.

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
fn stdout_mode_emits_all_four_banners() {
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
    assert!(stdout.contains("// ===== cellsmith cells.tcl ====="));
    assert!(stdout.contains("define_arc"));

    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn file_mode_writes_the_four_artifacts() {
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
    assert!(outdir.join("cli_cells.tcl").is_file());

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

/// Write `spec` to a fresh scratch dir, run the binary in `--stdout` mode with `extra` args followed by
/// the spec path, assert success, remove the dir, and return stdout.
fn run_spec(tag: &str, spec: &str, extra: &[&str]) -> String {
    let dir = scratch_dir(tag);
    let path = dir.join("cells.toml");
    std::fs::write(&path, spec).unwrap();

    let mut cmd = Command::new(BIN);
    cmd.arg("--stdout").args(extra).arg(&path);
    let out = cmd.output().expect("run cellsmith");
    assert!(out.status.success(), "exit: {:?}", out.status);

    std::fs::remove_dir_all(&dir).ok();
    String::from_utf8(out.stdout).unwrap()
}

/// Run the MULTI spec through the built binary in `--stdout` mode, optionally with
/// `--no-edge-collapse`, and return its stdout.
fn run_multi(no_edge_collapse: bool) -> String {
    let (tag, extra): (&str, &[&str]) = if no_edge_collapse {
        (
            "multi_no_collapse",
            &["--constraints", "--no-edge-collapse"],
        )
    } else {
        ("multi_collapse", &["--constraints"])
    };
    run_spec(tag, MULTI, extra)
}

/// The `cell (DFF)` Liberty fragment, from its header up to the next `cell (` (or end of string).
fn dff_liberty_fragment(stdout: &str) -> &str {
    let liberty = stdout
        .split("// ===== cellsmith liberty =====")
        .nth(1)
        .expect("liberty banner present");
    let start = liberty.find("cell (DFF)").expect("DFF cell present");
    let rest = &liberty[start..];
    match rest[1..].find("cell (") {
        Some(off) => &rest[..off + 1],
        None => rest,
    }
}

/// The `arcs.tcl` section of `--stdout` output: the text between the arcs banner and the next `// =====`
/// banner (the verilog one, or end of string).
fn arcs_section(stdout: &str) -> &str {
    let after = stdout
        .split("// ===== cellsmith arcs.tcl =====")
        .nth(1)
        .expect("arcs banner present");
    match after.find("// =====") {
        Some(off) => &after[..off],
        None => after,
    }
}

/// Whether any arc `-when` line appears in the `arcs.tcl` section. An arc `-when` is its own indented
/// continuation line (`\t-when "..." \`); a `define_leakage` `-when` is inline (`define_leakage -when
/// "..." { NAME }`) and so is deliberately NOT matched by the `starts_with("-when")` discriminator.
fn has_arc_when(arcs: &str) -> bool {
    arcs.lines().any(|l| l.trim_start().starts_with("-when"))
}

/// The number of `-type hidden` `define_arc` blocks carrying an arc `-when` line.
fn hidden_when_count(arcs: &str) -> usize {
    arcs.split("define_arc")
        .filter(|b| b.contains("-type hidden") && has_arc_when(b))
        .count()
}

/// The number of non-hidden (transition) `define_arc` blocks carrying an arc `-when` line. The leading
/// pre-first-block preamble is skipped; it carries no arc `-when` line regardless.
fn transition_when_count(arcs: &str) -> usize {
    arcs.split("define_arc")
        .skip(1)
        .filter(|b| !b.contains("-type hidden") && has_arc_when(b))
        .count()
}

/// `--no-edge-collapse` flips the MULTI spec's DFF between its collapsed edge-register Liberty form
/// (a `R` token statetable row, no `pin (M)`) and its two-latch level form (`pin (M)` with
/// `internal_node : "M"`, no edge token). Existing multi-cell CLI assertions (cell-name presence,
/// oscillate/race hazard warnings) still hold under default (on) collapse -- collapse only re-expresses
/// already-explored behaviour, it does not change the hazard diagnostics.
#[test]
fn no_edge_collapse_flag_flips_dff_liberty_between_edge_and_level_forms() {
    let collapsed = run_multi(false);
    let uncollapsed = run_multi(true);

    let collapsed_dff = dff_liberty_fragment(&collapsed);
    assert!(collapsed_dff.contains("statetable (\"CLK D\", \"Q\")"));
    assert!(collapsed_dff.split_whitespace().any(|t| t == "R"));
    assert!(!collapsed_dff.contains("pin (M)"));

    let uncollapsed_dff = dff_liberty_fragment(&uncollapsed);
    assert!(uncollapsed_dff.contains("statetable (\"CLK D\", \"Q M\")"));
    assert!(uncollapsed_dff.contains("pin (M)"));
    assert!(uncollapsed_dff.contains("internal_node : \"M\";"));
    assert!(!uncollapsed_dff.split_whitespace().any(|t| t == "R"));

    // Existing multi-cell assertions still pass, unmodified, under default collapse.
    for cell in ["C2", "MUT", "DFF"] {
        assert!(collapsed.contains(cell), "cell {cell} missing from stdout");
    }
}

/// A two-output cell carrying both arc classes: `Y = A` yields transition arcs, and the `Z` c-element
/// yields internal-power (hidden) arcs — both with `-when` lines in some context.
const TWO: &str = r#"
[[cell]]
name = "TWO"
inputs = ["A", "B"]
[cell.outputs]
Y = "A"
Z = "A*B + Z*(A+B)"
"#;

/// The `TWO` cell with `no_when = "transition"` declared, for the CLI+TOML union check.
const TWO_UNION: &str = r#"
[[cell]]
name = "TWO"
inputs = ["A", "B"]
no_when = "transition"
[cell.outputs]
Y = "A"
Z = "A*B + Z*(A+B)"
"#;

#[test]
fn default_run_emits_arc_when_lines() {
    let out = run_spec("two_default", TWO, &[]);
    assert!(
        has_arc_when(arcs_section(&out)),
        "default output carries arc -when lines"
    );
}

#[test]
fn bare_no_when_suppresses_every_arc_when_line() {
    let out = run_spec("two_none", TWO, &["--no-when"]);
    assert!(
        !has_arc_when(arcs_section(&out)),
        "bare --no-when leaves no arc -when line anywhere"
    );
}

#[test]
fn no_when_hidden_suppresses_only_hidden_when_lines() {
    let out = run_spec("two_hidden", TWO, &["--no-when=hidden"]);
    let arcs = arcs_section(&out);
    assert!(
        transition_when_count(arcs) >= 1,
        "transition arc -when lines remain"
    );
    assert_eq!(
        hidden_when_count(arcs),
        0,
        "no -type hidden block carries a -when"
    );
}

#[test]
fn no_when_transition_suppresses_only_transition_when_lines() {
    let default_out = run_spec("two_t_default", TWO, &[]);
    let default = arcs_section(&default_out);
    let out = run_spec("two_transition", TWO, &["--no-when=transition"]);
    let arcs = arcs_section(&out);
    // Every hidden block that had a -when still has one; no non-hidden block carries one.
    assert_eq!(
        hidden_when_count(arcs),
        hidden_when_count(default),
        "hidden -when lines are untouched by --no-when=transition"
    );
    assert!(
        hidden_when_count(arcs) >= 1,
        "the hidden -when lines are actually present to be preserved"
    );
    assert_eq!(
        transition_when_count(arcs),
        0,
        "no transition block carries a -when"
    );
}

#[test]
fn no_when_hidden_and_transition_equals_bare_no_when() {
    let both = run_spec(
        "two_both",
        TWO,
        &["--no-when=hidden", "--no-when=transition"],
    );
    let bare = run_spec("two_bare_eq", TWO, &["--no-when"]);
    assert_eq!(both, bare, "selecting both classes equals the bare flag");
}

#[test]
fn cli_class_unions_with_cell_no_when() {
    // The cell selects `transition`; the CLI adds `hidden`; the union suppresses every arc -when.
    let out = run_spec("two_union", TWO_UNION, &["--no-when=hidden"]);
    assert!(
        !has_arc_when(arcs_section(&out)),
        "the CLI class unions with the cell's own, leaving no arc -when line"
    );
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
