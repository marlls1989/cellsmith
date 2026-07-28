//! CLI integration checks driving the built binary directly (no extra dependencies): stdout mode and
//! its banners, file mode and its four artifacts, stdin (`-`), and the non-zero exit on a bad spec.

use std::collections::BTreeSet;
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

/// `stdout` with its arcs.tcl section ([`arcs_section`]) reduced to its records — a record being a
/// `define_arc`/`define_leakage`/`#` command with its indented continuation lines folded onto one line
/// and its whitespace collapsed — sorted. Other sections are left verbatim. Two runs that emit the same
/// arcs in a different order compare equal under this reduction.
fn canonical(stdout: &str) -> String {
    let arcs = arcs_section(stdout);
    let start = arcs.as_ptr() as usize - stdout.as_ptr() as usize;
    let end = start + arcs.len();

    let mut records: Vec<String> = Vec::new();
    for line in arcs.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if trimmed.starts_with("define_arc")
            || trimmed.starts_with("define_leakage")
            || trimmed.starts_with('#')
        {
            records.push(trimmed.to_string());
        } else {
            let last = records
                .last_mut()
                .expect("a continuation line follows its command");
            last.push(' ');
            last.push_str(trimmed);
        }
    }
    for r in &mut records {
        *r = r.split_whitespace().collect::<Vec<_>>().join(" ");
    }
    records.sort();

    format!(
        "{}{}{}",
        &stdout[..start],
        records.join("\n"),
        &stdout[end..]
    )
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
    assert!(collapsed_dff.contains("statetable (\"CLK D\", \"Q_st\")"));
    assert!(collapsed_dff.split_whitespace().any(|t| t == "R"));
    assert!(!collapsed_dff.contains("pin (M)"));

    let uncollapsed_dff = dff_liberty_fragment(&uncollapsed);
    assert!(uncollapsed_dff.contains("statetable (\"CLK D\", \"Q_st M\")"));
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

/// The `TWO` cell with `when = "transition"` declared, for the CLI+TOML union check.
const TWO_UNION: &str = r#"
[[cell]]
name = "TWO"
inputs = ["A", "B"]
when = "transition"
[cell.outputs]
Y = "A"
Z = "A*B + Z*(A+B)"
"#;

/// The `-vector { ... }` value of a `define_arc` block (the text between `-vector {` and its closing
/// `}`).
fn vector_of(block: &str) -> &str {
    let after = block
        .split("-vector {")
        .nth(1)
        .expect("block carries a -vector line");
    &after[..after.find('}').expect("-vector value is brace-closed")]
}

/// Every `define_arc` block in `arcs`, each truncated at its trailing blank line (the block separator),
/// so trailing `define_leakage`/constraint text attached to the final split segment does not leak into
/// the last block.
fn arc_blocks(arcs: &str) -> Vec<&str> {
    arcs.split("define_arc")
        .skip(1)
        .map(|b| match b.find("\n\n") {
            Some(off) => &b[..off],
            None => b,
        })
        .collect()
}

/// An OR-AND 2-2: `A` rising drives `Y` rising from three side-input contexts (`B` low, `(C,D)` at `01`,
/// `10` or `11`), all firings of ONE transition.
const OA22: &str = r#"
[[cell]]
name = "OA22"
inputs = ["A", "B", "C", "D"]
[cell.outputs]
Y = "(A+B)*(C+D)"
"#;

/// The value of a single-token `define_arc` field (`-type`, `-related_pin`, `-pin`). The trailing space
/// in the match keeps `-pin` from picking up the `-pinlist` line.
fn tag_of(block: &str, tag: &str) -> Option<String> {
    block
        .lines()
        .map(str::trim)
        .find(|l| l.starts_with(&format!("{tag} ")))
        .and_then(|l| l.split_whitespace().nth(1))
        .map(str::to_string)
}

/// The block's `-vector` column for `pin`, indexed through its own `-pinlist` line.
fn vector_column(block: &str, pin: &str) -> String {
    let pinlist = block
        .lines()
        .map(str::trim)
        .find(|l| l.starts_with("-pinlist {"))
        .and_then(|l| l.split('{').nth(1))
        .and_then(|l| l.split('}').next())
        .expect("a block renders a -pinlist");
    let idx = pinlist
        .split_whitespace()
        .position(|p| p == pin)
        .expect("the pin appears in the block's -pinlist");
    vector_of(block)
        .split_whitespace()
        .nth(idx)
        .expect("the -vector has a column per pin")
        .to_string()
}

/// The default run emits ONE unconditioned block per transition — a related pin's edge driving an output
/// pin's edge at one `-type` — however many side-input contexts the transition was measured from. `OA22`
/// fires its `A`-rise → `Y`-rise transition from three of them; one block comes out.
#[test]
fn default_run_emits_one_general_block_per_transition() {
    let out = run_spec("oa22_default", OA22, &[]);
    let arcs = arcs_section(&out);
    let transitions: Vec<&str> = arc_blocks(arcs)
        .into_iter()
        .filter(|b| !b.contains("-type hidden"))
        .collect();
    assert!(!transitions.is_empty(), "OA22 emits transition arcs");

    let mut seen: BTreeSet<(String, String, String, String, String)> = BTreeSet::new();
    for b in &transitions {
        let related = tag_of(b, "-related_pin").expect("a transition block names its related pin");
        let pin = tag_of(b, "-pin").expect("a transition block names its measured pin");
        let ty = tag_of(b, "-type").expect("a transition block declares a -type");
        let key = (
            related.clone(),
            pin.clone(),
            ty,
            vector_column(b, &related),
            vector_column(b, &pin),
        );
        assert!(
            seen.insert(key.clone()),
            "a transition is emitted twice: {key:?}"
        );
    }
    assert!(
        seen.iter()
            .any(|(related, pin, _, r, p)| related == "A" && pin == "Y" && r == "R" && p == "R"),
        "the A-rise → Y-rise transition is emitted"
    );
    assert!(
        !has_arc_when(arcs),
        "the default run emits no arc -when lines"
    );
}

#[test]
fn default_run_emits_no_arc_when_lines() {
    let out = run_spec("two_default", TWO, &[]);
    assert!(
        !has_arc_when(arcs_section(&out)),
        "default output carries no arc -when lines"
    );
    // The `-when` arcs are added ON TOP of the always-emitted general arcs, so the default run emits
    // strictly fewer `define_arc` blocks than bare `--when`.
    let when_out = run_spec("two_when_more", TWO, &["--when"]);
    let define_arcs = |s: &str| arcs_section(s).matches("define_arc").count();
    assert!(
        define_arcs(&out) < define_arcs(&when_out),
        "default emits strictly fewer define_arc blocks than --when: {} not < {}",
        define_arcs(&out),
        define_arcs(&when_out),
    );
}

#[test]
fn bare_when_emits_arc_when_lines() {
    let default = run_spec("two_default_for_bare", TWO, &[]);
    let out = run_spec("two_when", TWO, &["--when"]);
    assert!(
        has_arc_when(arcs_section(&out)),
        "bare --when carries arc -when lines"
    );
    // The general arcs stay put and the `-when` blocks are added on top, so bare --when emits strictly
    // more `define_arc` blocks than the default run.
    let define_arcs = |s: &str| arcs_section(s).matches("define_arc").count();
    assert!(
        define_arcs(&out) > define_arcs(&default),
        "bare --when adds -when blocks on top of the general arcs: {} not > {}",
        define_arcs(&out),
        define_arcs(&default),
    );
}

#[test]
fn when_hidden_emits_only_hidden_when_lines() {
    let default_out = run_spec("two_hidden_default", TWO, &[]);
    let default = arcs_section(&default_out);
    let out = run_spec("two_hidden", TWO, &["--when=hidden"]);
    let arcs = arcs_section(&out);
    assert!(
        hidden_when_count(arcs) >= 1,
        "hidden arc -when lines are present"
    );
    // The hidden general blocks are unconditionally emitted, so selecting the class adds -when
    // blocks on top without dropping any general block.
    let hidden_catchall_count = |arcs: &str| {
        arcs.split("define_arc")
            .skip(1)
            .filter(|b| b.contains("-type hidden") && !has_arc_when(b))
            .count()
    };
    assert_eq!(
        hidden_catchall_count(arcs),
        hidden_catchall_count(default),
        "hidden general blocks are still present"
    );
    assert_eq!(
        transition_when_count(arcs),
        0,
        "no transition block carries a -when"
    );
}

#[test]
fn when_transition_emits_only_transition_when_lines() {
    let default_out = run_spec("two_t_default", TWO, &[]);
    let default = arcs_section(&default_out);
    let out = run_spec("two_transition", TWO, &["--when=transition"]);
    let arcs = arcs_section(&out);
    assert!(
        transition_when_count(arcs) >= 1,
        "transition arc -when lines are present"
    );
    // The transition general blocks are unconditionally emitted, so selecting the class adds -when
    // blocks on top without dropping any general block.
    let transition_catchall_count = |arcs: &str| {
        arcs.split("define_arc")
            .skip(1)
            .filter(|b| !b.contains("-type hidden") && !has_arc_when(b))
            .count()
    };
    assert_eq!(
        transition_catchall_count(arcs),
        transition_catchall_count(default),
        "transition general blocks are still present"
    );
    assert_eq!(
        hidden_when_count(arcs),
        0,
        "no hidden block carries a -when"
    );
}

#[test]
fn when_hidden_and_transition_equals_bare_when() {
    let both = run_spec("two_both", TWO, &["--when=hidden", "--when=transition"]);
    let bare = run_spec("two_bare_eq", TWO, &["--when"]);
    assert_eq!(
        canonical(&both),
        canonical(&bare),
        "selecting both classes equals the bare flag"
    );
}

/// A bare `--when` is the blanket selection, so combining it with a valued occurrence selects every
/// class — in either order — and the run is indistinguishable from bare `--when` alone.
#[test]
fn bare_when_unions_with_a_valued_when_in_either_order() {
    let bare = run_spec("two_mixed_bare_eq", TWO, &["--when"]);
    let bare_first = run_spec("two_mixed", TWO, &["--when", "--when=hidden"]);
    let valued_first = run_spec("two_mixed_rev", TWO, &["--when=hidden", "--when"]);
    assert_eq!(
        canonical(&bare_first),
        canonical(&bare),
        "a bare --when before a valued one still selects every class"
    );
    assert_eq!(
        canonical(&valued_first),
        canonical(&bare),
        "a bare --when after a valued one still selects every class"
    );
    // Both classes carry their `-when` blocks, so the equality above is not two empty selections.
    let arcs = arcs_section(&bare_first);
    assert!(
        transition_when_count(arcs) >= 1 && hidden_when_count(arcs) >= 1,
        "both arc classes carry -when lines: {} transition, {} hidden",
        transition_when_count(arcs),
        hidden_when_count(arcs),
    );
}

#[test]
fn cli_class_unions_with_cell_when() {
    // The cell selects `transition`; the CLI adds `hidden`; the union equals bare `--when`.
    let out = run_spec("two_union", TWO_UNION, &["--when=hidden"]);
    let bare = run_spec("two_union_bare_eq", TWO, &["--when"]);
    assert_eq!(
        canonical(&out),
        canonical(&bare),
        "the CLI class unions with the cell's own, matching bare --when"
    );
}

/// With `--when=transition` on the `TWO` fixture, the transition arc that becomes the general
/// representative also carries a rendered `-when` condition (its related pin is not `TWO`'s only input), so
/// its `-vector` value appears in two distinct blocks: the general one without `-when`, and the `-when`
/// pass's own block for that same arc.
#[test]
fn when_transition_duplicates_a_vector_with_and_without_when() {
    let out = run_spec("two_dup_transition", TWO, &["--when=transition"]);
    let arcs = arcs_section(&out);
    let (with_when, without_when): (Vec<&str>, Vec<&str>) = arcs
        .split("define_arc")
        .skip(1)
        .filter(|b| !b.contains("-type hidden"))
        .partition(|b| has_arc_when(b));
    let with_when_vectors: Vec<&str> = with_when.iter().map(|b| vector_of(b)).collect();
    let without_when_vectors: Vec<&str> = without_when.iter().map(|b| vector_of(b)).collect();
    assert!(
        with_when_vectors
            .iter()
            .any(|v| without_when_vectors.contains(v)),
        "a transition -vector value appears both with and without -when"
    );
}

/// Analogous to [`when_transition_duplicates_a_vector_with_and_without_when`], for the hidden class.
#[test]
fn when_hidden_duplicates_a_vector_with_and_without_when() {
    let out = run_spec("two_dup_hidden", TWO, &["--when=hidden"]);
    let arcs = arcs_section(&out);
    let (with_when, without_when): (Vec<&str>, Vec<&str>) = arcs
        .split("define_arc")
        .skip(1)
        .filter(|b| b.contains("-type hidden"))
        .partition(|b| has_arc_when(b));
    let with_when_vectors: Vec<&str> = with_when.iter().map(|b| vector_of(b)).collect();
    let without_when_vectors: Vec<&str> = without_when.iter().map(|b| vector_of(b)).collect();
    assert!(
        with_when_vectors
            .iter()
            .any(|v| without_when_vectors.contains(v)),
        "a hidden -vector value appears both with and without -when"
    );
}

/// Bare `--when`'s arcs section is a superset of the default run's: every default `define_arc` block
/// (the always-emitted general arcs) still appears verbatim once the `-when` blocks are added on top.
#[test]
fn when_output_contains_every_default_arc_block() {
    let default = run_spec("two_when_subset_default", TWO, &[]);
    let when_out = run_spec("two_when_subset_when", TWO, &["--when"]);
    let default_arcs = arcs_section(&default);
    let when_arcs = arcs_section(&when_out);
    for block in arc_blocks(default_arcs) {
        assert!(
            when_arcs.contains(&format!("define_arc{block}")),
            "the --when output should contain every default define_arc block:\n{block}"
        );
    }
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
