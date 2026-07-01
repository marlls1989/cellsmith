//! End-to-end parity and validity checks over the three per-cell artifacts.
//!
//! Full byte-parity with the Haskell `genLiberateTemplate` goldens is not a goal: lobsterate emits
//! pins in declaration order (not alphabetical), drops the `vclk`/alias layer, and factors don't-care
//! cubes via BDD paths rather than Quine–McCluskey. What we *can* pin down is that the logic matches —
//! the sequential-UDP next-state table of a 2-input C-element is canonical — and that the emitted
//! Liberty is syntactically valid (parses back through `liberty-parse`).

use lobsterate::emit::arcs_tcl::{cell_arcs_tcl, ArcsTclOptions};
use lobsterate::emit::liberty::cell_liberty;
use lobsterate::emit::verilog::cell_verilog;
use lobsterate::model::{parse_spec, AnalysedCell};

fn analyse_one(src: &str) -> AnalysedCell {
    parse_spec(src).unwrap().cells.remove(0).analyse().unwrap()
}

const C2: &str = r#"
[[cell]]
name = "C2"
inputs = ["A", "B"]
[cell.outputs]
Q = "A*B + Q*(A+B)"
"#;

/// The 2-input C-element UDP table is canonical: `00→0`, `11→1`, `01`/`10` hold. This matches the
/// hsNCL golden `CELEM2_Q` primitive body exactly (only the primitive/module names differ).
#[test]
fn c_element_udp_table_matches_golden_logic() {
    let v = cell_verilog(&analyse_one(C2));
    for row in [
        "0 0 : ? : 0;",
        "0 1 : ? : -;",
        "1 0 : ? : -;",
        "1 1 : ? : 1;",
    ] {
        assert!(v.contains(row), "missing UDP row {row:?} in:\n{v}");
    }
    assert!(v.contains("primitive C2_Q(Q, A, B);"));
    assert!(v.contains("reg    Q;"));
}

/// The emitted Liberty fragment must be syntactically valid: wrapped in a `library`, it round-trips
/// through `liberty-parse`, and the cell/pin/statetable groups survive.
#[test]
fn liberty_fragment_parses() {
    let frag = cell_liberty(&analyse_one(C2));
    let wrapped = format!("library (test) {{\n{frag}}}\n");
    let lib = liberty_parse::parse_lib(&wrapped).expect("emitted Liberty must parse");
    let cell = lib
        .iter()
        .flat_map(|g| g.subgroups.iter())
        .find(|g| g.type_ == "cell" && g.name == "C2")
        .expect("C2 cell present after round-trip");
    // The hysteretic output carries a statetable; the inputs are plain pins.
    assert!(cell.subgroups.iter().any(|g| g.type_ == "statetable"));
    assert!(cell
        .subgroups
        .iter()
        .any(|g| g.type_ == "pin" && g.name == "A"));
}

/// Multiple cells concatenate into a single, still-parseable Liberty fragment (no missing separators).
#[test]
fn multi_cell_liberty_concatenates_cleanly() {
    let spec = r#"
[[cell]]
name = "C2"
inputs = ["A", "B"]
[cell.outputs]
Q = "A*B + Q*(A+B)"

[[cell]]
name = "ND2"
inputs = ["A", "B"]
[cell.outputs]
Y = "!(A*B)"
"#;
    let cells: Vec<AnalysedCell> = parse_spec(spec)
        .unwrap()
        .cells
        .iter()
        .map(|c| c.analyse().unwrap())
        .collect();
    let frag: String = cells.iter().map(cell_liberty).collect();
    let wrapped = format!("library (test) {{\n{frag}}}\n");
    let lib = liberty_parse::parse_lib(&wrapped).expect("concatenated Liberty must parse");
    let names: Vec<String> = lib
        .iter()
        .flat_map(|g| g.subgroups.iter())
        .filter(|g| g.type_ == "cell")
        .map(|g| g.name.clone())
        .collect();
    assert_eq!(names, ["C2", "ND2"]);
}

/// The `-when` flag threads through to the arc text.
#[test]
fn when_flag_reaches_arcs() {
    let cell = analyse_one(C2);
    let off = cell_arcs_tcl(&cell, ArcsTclOptions::default()).unwrap();
    let on = cell_arcs_tcl(&cell, ArcsTclOptions { emit_when: true }).unwrap();
    assert!(!off.contains("-when"));
    assert!(on.contains("-when"));
}
