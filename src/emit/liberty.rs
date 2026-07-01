//! Emit a minimal Liberty fragment for a cell: input `pin` groups plus, per output, either a plain
//! combinational `function` or — for a hysteretic (self-holding) output — a `statetable` whose
//! next-state encodes the three regions as `H` (on) / `L` (off) / `N` (no-change = hold).
//!
//! The fragment is a bare `cell (...) { ... }` group, not a full library wrapper: the user drops it
//! into their own Liberate harness. Groups are built with `liberty-parse`'s `Group`/`Attribute`/`Value`
//! trees (the same idiom as `pseudosync/src/lib.rs`) and rendered by wrapping in `Liberty` — `Group`
//! itself has no `Display`.

use liberty_parse::{
    ast::Value,
    liberty::{Attribute, Group, Liberty},
};

use crate::logic::regions::{state_regions, StateCube, StateRegions};
use crate::model::AnalysedCell;

/// Add a simple `name : value;` attribute to a group.
///
/// Constructed via `Group`'s own `attributes` map so we never name `IndexMap` directly — the crate
/// and `liberty-parse` pull different major versions of `indexmap`, whose types are incompatible.
fn set_attr(group: &mut Group, name: &str, value: Value) {
    group
        .attributes
        .insert(name.to_owned(), vec![Attribute::Simple(value)]);
}

/// The Liberty `cell (...) { ... }` fragment for a cell, as text (newline-terminated so fragments
/// concatenate cleanly).
pub fn cell_liberty(cell: &AnalysedCell) -> String {
    format!("{}\n", Liberty(vec![cell_group(cell)]))
}

/// Build the `cell` group: one input `pin` per primary input, then each output's pin (and, if
/// hysteretic, its `statetable`).
fn cell_group(cell: &AnalysedCell) -> Group {
    let mut group = Group::new("cell", &cell.name);

    for input in &cell.inputs {
        group.subgroups.push(input_pin(input));
    }

    for output in &cell.outputs {
        let sr = state_regions(output, &cell.inputs);
        if sr.hysteretic {
            group.subgroups.push(statetable_group(&sr, &output.name));
            // The pin reads the internal state node of the same name (see the plan's open note on
            // the exact internal-node wiring).
            group.subgroups.push(output_pin(&output.name, &output.name));
        } else {
            group
                .subgroups
                .push(output_pin(&output.name, &function_sop(&sr)));
        }
    }

    group
}

/// `pin (<name>) { direction : input; }`
fn input_pin(name: &str) -> Group {
    let mut pin = Group::new("pin", name);
    set_attr(&mut pin, "direction", Value::Expression("input".to_owned()));
    pin
}

/// `pin (<name>) { direction : output; function : "<func>"; }`
fn output_pin(name: &str, func: &str) -> Group {
    let mut pin = Group::new("pin", name);
    set_attr(
        &mut pin,
        "direction",
        Value::Expression("output".to_owned()),
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
                    Some(true) => Some(col.clone()),
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
    use crate::model::parse_spec;

    fn analyse(src: &str) -> AnalysedCell {
        parse_spec(src).unwrap().cells.remove(0).analyse().unwrap()
    }

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
        assert!(lib.contains("function :"));
        // NAND on-set = !A + !B (as SOP over the two off/hold-free cubes).
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
        let sr = state_regions(&cell.outputs[0], &cell.inputs);
        let f = function_sop(&sr);
        // Must be a valid product-of-literals sum mentioning the pins.
        assert!(f.contains('+') || f.contains('*') || f.contains('!'));
        assert!(f.contains('A') || f.contains('C'));
    }
}
