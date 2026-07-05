//! Emit a minimal Liberty fragment for a cell: input `pin` groups, then per output and internal state
//! node either a plain combinational `function` or — for a hysteretic (self-holding) signal — a
//! `statetable` whose next-state encodes the three regions as `H` (on) / `L` (off) / `N`
//! (no-change = hold). Internal state nodes are emitted as `direction : internal` pins and appear as
//! internal-node columns in the state tables of the outputs that reference them.
//!
//! `cell_liberty` renders one cell as a bare `cell (...) { ... }` group; `library_liberty` wraps all of
//! a run's cells in a single `library (<name>) { ... }` group — the `.lib` file cellsmith writes. Groups
//! are built with `liberty-parse`'s `Group`/`Attribute`/`Value`
//! trees (the same idiom as `pseudosync/src/lib.rs`) and rendered by wrapping in `Liberty` — `Group`
//! itself has no `Display`.

use liberty_parse::{
    ast::Value,
    liberty::{Attribute, Group, Liberty},
};

use crate::logic::regions::{StateCube, StateRegions};
use crate::model::{AnalysedCell, AnalysedOutput};

/// Add a simple `name : value;` attribute to a group.
///
/// Constructed via `Group`'s own `attributes` map so we never name `IndexMap` directly — the crate
/// and `liberty-parse` pull different major versions of `indexmap`, whose types are incompatible.
fn set_attr(group: &mut Group, name: &str, value: Value) {
    group
        .attributes
        .insert(name.to_owned(), vec![Attribute::Simple(value)]);
}

/// Wrap every cell's Liberty fragment in a single `library (<name>) { ... }` group so the output is a
/// self-contained `.lib` that Liberate can consume directly as `user_data` — no external harness
/// needed. Each cell fragment (oscillation comments included) is indented one level inside the
/// library.
pub fn library_liberty(name: &str, cells: &[AnalysedCell]) -> String {
    let mut out = format!("library ({name}) {{\n");
    for cell in cells {
        for line in cell_liberty(cell).lines() {
            if line.is_empty() {
                out.push('\n');
            } else {
                out.push_str("  ");
                out.push_str(line);
                out.push('\n');
            }
        }
    }
    out.push_str("}\n");
    out
}

/// The Liberty `cell (...) { ... }` fragment for a cell, as text (newline-terminated so fragments
/// concatenate cleanly). Interlocked (mutex/arbiter) cells are prefixed with a comment recording the
/// metastable condition and the mutually-exclusive (forbidden-both-high) grants.
pub fn cell_liberty(cell: &AnalysedCell) -> String {
    let mut out = String::new();
    for a in &cell.oscillation {
        out.push_str(&format!(
            "/* oscillation: {} metastable; grants {} mutually exclusive (both-high forbidden) */\n",
            a.condition_str(),
            a.group.join(", "),
        ));
    }
    out.push_str(&format!("{}\n", Liberty(vec![cell_group(cell)])));
    out
}

/// Build the `cell` group: one input `pin` per primary input, then each output pin, then each internal
/// state node as a `direction : internal` pin — each with a `statetable` when hysteretic.
fn cell_group(cell: &AnalysedCell) -> Group {
    let mut group = Group::new("cell", &cell.name);

    for input in &cell.inputs {
        group.subgroups.push(input_pin(input));
    }
    let n_out = cell.outputs.len();
    for (i, (sig, sr)) in cell.signal_regions().enumerate() {
        let direction = if i < n_out { "output" } else { "internal" };
        push_signal(&mut group, sig, sr, direction);
    }

    group
}

/// Emit a signal's pin (and, if hysteretic, its `statetable`). `direction` is `"output"` for an
/// external pin or `"internal"` for an internal state node — modelled in the state table exactly like
/// an output, but with no external connection.
fn push_signal(group: &mut Group, sig: &AnalysedOutput, sr: &StateRegions, direction: &str) {
    if sr.hysteretic {
        group.subgroups.push(statetable_group(sr, &sig.name));
        // A hysteretic pin reads the state node of the same name defined by its statetable.
        group
            .subgroups
            .push(signal_pin(&sig.name, direction, &sig.name));
    } else {
        group
            .subgroups
            .push(signal_pin(&sig.name, direction, &function_sop(sr)));
    }
}

/// `pin (<name>) { direction : input; }`
fn input_pin(name: &str) -> Group {
    let mut pin = Group::new("pin", name);
    set_attr(&mut pin, "direction", Value::Expression("input".to_owned()));
    pin
}

/// `pin (<name>) { direction : <output|internal>; function : "<func>"; }`
fn signal_pin(name: &str, direction: &str, func: &str) -> Group {
    let mut pin = Group::new("pin", name);
    set_attr(
        &mut pin,
        "direction",
        Value::Expression(direction.to_owned()),
    );
    set_attr(&mut pin, "function", Value::String(func.to_owned()));
    pin
}

/// `statetable ("<inputs>", "<pin>") { table : "<rows>"; }` — the hysteretic next-state table.
fn statetable_group(sr: &StateRegions, pin: &str) -> Group {
    // The header is one verbatim string: input-node list and internal-node list, each quoted.
    let header = format!("\"{}\", \"{}\"", sr.cols.join(" "), pin);
    let mut st = Group::new("statetable", &header);
    set_attr(&mut st, "table", Value::String(table_string(sr)));
    st
}

/// Build the state table string: one comma-separated row per region cube,
/// `<input state> : <current state> : <next state>`, current state always `-` (any).
fn table_string(sr: &StateRegions) -> String {
    let mut rows: Vec<String> = Vec::new();
    for cube in &sr.on {
        rows.push(format!("{} : - : H", state_pattern(cube)));
    }
    for cube in &sr.off {
        rows.push(format!("{} : - : L", state_pattern(cube)));
    }
    for cube in &sr.hold {
        rows.push(format!("{} : - : N", state_pattern(cube)));
    }
    rows.sort();
    rows.join(" , ")
}

/// Render a cube as space-separated `H`/`L`/`-` symbols (Liberty state-table input levels).
fn state_pattern(cube: &StateCube) -> String {
    cube.iter()
        .map(|c| match c {
            Some(true) => "H",
            Some(false) => "L",
            None => "-",
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Render the on-region as a Liberty function string: a sum (`+`) of product (`*`) cubes, each a
/// product of literals (`!` for negation) over the column header. A single empty cube is the
/// tautology `"1"`; no cubes is the contradiction `"0"`.
fn function_sop(sr: &StateRegions) -> String {
    if sr.on.is_empty() {
        return "0".to_owned();
    }
    let products: Vec<String> = sr
        .on
        .iter()
        .map(|cube| {
            let lits: Vec<String> = sr
                .cols
                .iter()
                .zip(cube.iter())
                .filter_map(|(col, val)| match val {
                    Some(true) => Some(col.to_string()),
                    Some(false) => Some(format!("!{col}")),
                    None => None,
                })
                .collect();
            if lits.is_empty() {
                "1".to_owned()
            } else {
                lits.join("*")
            }
        })
        .collect();
    // If any product is the constant "1" the whole function is a tautology.
    if products.iter().any(|p| p == "1") {
        "1".to_owned()
    } else {
        products.join(" + ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::analyse_one as analyse;

    #[test]
    fn c_element_emits_statetable() {
        let cell = analyse(
            r#"
[[cell]]
name = "C2"
inputs = ["A", "B"]
[cell.outputs]
Q = "A*B + Q*(A+B)"
"#,
        );
        let lib = cell_liberty(&cell);
        eprintln!("{lib}");
        assert!(lib.contains("cell (C2)"));
        assert!(lib.contains("pin (A)"));
        assert!(lib.contains("direction : input;"));
        assert!(lib.contains("statetable (\"A B\", \"Q\")"));
        assert!(lib.contains(": - : H")); // on
        assert!(lib.contains(": - : L")); // off
        assert!(lib.contains(": - : N")); // hold
        assert!(lib.contains("pin (Q)"));
        assert!(lib.contains("function : \"Q\";"));
    }

    #[test]
    fn dff_internal_master_is_an_internal_pin_with_statetable() {
        let cell = analyse(
            r#"
[[cell]]
name = "DFF"
inputs = ["CLK", "D"]
[cell.internal]
M = "!CLK*D + CLK*M"
[cell.outputs]
Q = "CLK*M + !CLK*Q"
"#,
        );
        let frag = cell_liberty(&cell);
        eprintln!("{frag}");
        // The slave Q's statetable carries the internal master M as an input node — but not D, which
        // Q's function (CLK*M + !CLK*Q) does not depend on.
        assert!(frag.contains("statetable (\"CLK M\", \"Q\")"));
        // The master is an internal pin with its own statetable.
        assert!(frag.contains("statetable (\"CLK D\", \"M\")"));
        assert!(frag.contains("pin (M)"));
        assert!(frag.contains("direction : internal;"));
        // The fragment still round-trips through liberty-parse.
        let wrapped = format!("library (test) {{\n{frag}}}\n");
        let lib = liberty_parse::parse_lib(&wrapped).expect("emitted Liberty must parse");
        let cellg = lib
            .iter()
            .flat_map(|g| g.subgroups.iter())
            .find(|g| g.type_ == "cell" && g.name == "DFF")
            .expect("DFF cell present");
        assert!(cellg
            .subgroups
            .iter()
            .any(|g| g.type_ == "pin" && g.name == "M"));
    }

    #[test]
    fn combinational_emits_plain_function() {
        let cell = analyse(
            r#"
[[cell]]
name = "ND2"
inputs = ["A", "B"]
[cell.outputs]
Y = "!(A*B)"
"#,
        );
        let lib = cell_liberty(&cell);
        eprintln!("{lib}");
        assert!(lib.contains("cell (ND2)"));
        assert!(!lib.contains("statetable"));
        assert!(lib.contains("pin (Y)"));
        // NAND on-set = !(A*B), Espresso-minimised to the two-cube SOP !B + !A.
        assert!(lib.contains("function : \"!B + !A\";"));
        assert!(lib.contains("direction : output;"));
    }

    #[test]
    fn function_sop_renders_literals() {
        let cell = analyse(
            r#"
[[cell]]
name = "AOI"
inputs = ["A", "B", "C"]
[cell.outputs]
Y = "A*B + !C"
"#,
        );
        let sr = &cell.regions[0];
        let f = function_sop(sr);
        // Must be a valid product-of-literals sum mentioning the pins.
        assert!(f.contains('+') || f.contains('*') || f.contains('!'));
        assert!(f.contains('A') || f.contains('C'));
    }
}
