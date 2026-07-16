//! Emit a minimal Liberty fragment for a cell: input `pin` groups, then — for a sequential cell — ONE
//! joint `statetable` group, and finally the output and internal-node `pin` groups that bind to it.
//!
//! A sequential library cell carries **exactly one** state table (Liberty 2017.06 Vol.1 §5 p.5-23: "a
//! sequential library cell can have only one state table"); every state variable of the cell is one
//! column of that single table. The joint model is built at emission time by
//! [`crate::emit::statetable::build_state_model`], which folds the cell's hysteretic signals (its
//! **state variables**: outputs and internal nodes on a dependency cycle) into one next-state table.
//! Within a table field node values are **space-separated**; whole rows are **comma-separated**. The
//! next-state action per node is `H` (drive high = on region) / `L` (drive low = off) / `N` (hold =
//! no-change) / `-` (unconstrained here — a legal next value that defers the node to a lower-priority
//! row, per Liberty's per-output next-state resolution). A purely combinational cell has no state
//! table: each output emits a plain `function`.
//!
//! The state-table node namespace is isolated from the pin namespace, so nodes bind to ports through
//! pin attributes:
//! - an OUTPUT state variable is aliased at emission time to a `{name}_st` node so no table node ever
//!   names an external output pin; its pin carries `internal_node : "{name}_st"` and
//!   `inverted_output : false` (the `_st` alias itself gets no pin group — it is anchored by this
//!   attribute);
//! - a GENUINE INTERNAL state node keeps its own name and is emitted as a
//!   `direction : internal; internal_node : "<name>";` pin (Liberty UG Vol.1 `pin(n1)` example);
//! - an output that merely projects a state node (a feedthrough or inverter) carries that node's
//!   `internal_node` with `inverted_output` `false`/`true`;
//! - any other output of a sequential cell carries `state_function : "<sop>"`, which names PIN ports
//!   (inputs, internal pins, or output pins carrying an `internal_node`) and NEVER a bare `_st` node —
//!   the table namespace is isolated (Liberty UG Vol.1 p.5-31, and the `pin(QNZ){state_function:"QN"}`
//!   / feedthrough `pin(Y){state_function:"A"}` examples on p.5-33). A cell with a state table has no
//!   plain `function` attribute anywhere.
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

use espresso_logic::Symbol;
use rayon::prelude::*;

use crate::emit::statetable::{build_state_model, Next, StateModel};
use crate::logic::hazard::Oscillation;
use crate::logic::regions::{StateCube, StateRegions};
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

/// Wrap every cell's Liberty fragment in a single `library (<name>) { ... }` group so the output is a
/// self-contained `.lib` that Liberate can consume directly as `user_data` — no external harness
/// needed. Each cell fragment (oscillation comments included) is indented one level inside the
/// library.
pub fn library_liberty(name: &str, cells: &[AnalysedCell]) -> String {
    let mut out = format!("library ({name}) {{\n");
    let frags: Vec<String> = cells.par_iter().map(cell_liberty).collect();
    for frag in &frags {
        for line in frag.lines() {
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
/// concatenate cleanly). A cell with a detected oscillation hazard is prefixed with a comment
/// recording the racing condition and the competing settled outcomes.
pub fn cell_liberty(cell: &AnalysedCell) -> String {
    let mut out = String::new();
    for a in &cell.oscillation {
        let states: Vec<String> = a.stable.iter().map(Oscillation::state_str).collect();
        out.push_str(&format!(
            "/* oscillation: {} risks metastability in {}, settling to one of {} */\n",
            a.condition_str(),
            a.group.join(", "),
            states.join(" | "),
        ));
    }
    out.push_str(&format!("{}\n", Liberty(vec![cell_group(cell)])));
    out
}

/// Build the `cell` group: one input `pin` per primary input, then — for a sequential cell — the single
/// joint `statetable`, its output pins (in declaration order), and its genuine-internal pins. A purely
/// combinational cell (no state model) emits each output as a plain `function` pin instead.
fn cell_group(cell: &AnalysedCell) -> Group {
    let mut group = Group::new("cell", cell.repr_name());

    for input in &cell.inputs {
        group.subgroups.push(input_pin(input));
    }

    match build_state_model(cell) {
        // Purely combinational: every signal is an output carrying a plain `function`.
        None => {
            for (sig, sr) in cell.signal_regions() {
                group
                    .subgroups
                    .push(function_pin(&sig.name, &function_sop(sr)));
            }
        }
        // Sequential: one joint statetable, then output pins bound to it, then internal-node pins.
        Some(model) => {
            group.subgroups.push(statetable_group(&model));

            let n_out = cell.outputs.len();
            for (i, (sig, sr)) in cell.signal_regions().enumerate() {
                if i < n_out {
                    group.subgroups.push(output_pin(&sig.name, sr, &model));
                }
            }
            for (i, (sig, _)) in cell.signal_regions().enumerate() {
                if i >= n_out {
                    group.subgroups.push(internal_pin(&sig.name));
                }
            }
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

/// `pin (<name>) { direction : output; function : "<func>"; }` — a combinational output.
fn function_pin(name: &str, func: &str) -> Group {
    let mut pin = Group::new("pin", name);
    set_attr(
        &mut pin,
        "direction",
        Value::Expression("output".to_owned()),
    );
    set_attr(&mut pin, "function", Value::String(func.to_owned()));
    pin
}

/// The output pin of a sequential cell, bound to the joint statetable. Three cases (Liberty UG Vol.1
/// pp.5-31..5-33):
/// - a state variable (`sig` is itself a table node) reads its own `{name}_st` alias node;
/// - a pure projection of a state node (a feedthrough/inverter, no hold) reads that node with the
///   matching `inverted_output`;
/// - any other output names PIN ports through `state_function` (never a bare `_st` node).
fn output_pin(name: &Symbol, sr: &StateRegions, model: &StateModel) -> Group {
    let mut pin = Group::new("pin", name);
    set_attr(
        &mut pin,
        "direction",
        Value::Expression("output".to_owned()),
    );

    if let Some(node) = model.node_of.get(name) {
        // (1) This output is itself a state variable — read its own aliased node.
        set_attr(
            &mut pin,
            "internal_node",
            Value::String(node.as_str().to_owned()),
        );
        set_attr(
            &mut pin,
            "inverted_output",
            Value::Expression("false".to_owned()),
        );
    } else if let Some((node, inverted)) = projection_of(sr, model) {
        // (2) A pure projection of a state node — a feedthrough (false) or inverter (true).
        set_attr(
            &mut pin,
            "internal_node",
            Value::String(node.as_str().to_owned()),
        );
        let inv = if inverted { "true" } else { "false" };
        set_attr(
            &mut pin,
            "inverted_output",
            Value::Expression(inv.to_owned()),
        );
    } else {
        // (3) Any other output — a combinational function over PIN ports, never a `_st` node.
        set_attr(&mut pin, "state_function", Value::String(function_sop(sr)));
    }
    pin
}

/// `pin (<name>) { direction : internal; internal_node : "<name>"; }` — a genuine internal state node,
/// anchoring the same-named table column to a port (Liberty UG Vol.1 `pin(n1)` example).
fn internal_pin(name: &Symbol) -> Group {
    let mut pin = Group::new("pin", name);
    set_attr(
        &mut pin,
        "direction",
        Value::Expression("internal".to_owned()),
    );
    set_attr(
        &mut pin,
        "internal_node",
        Value::String(name.as_str().to_owned()),
    );
    pin
}

/// If `sr` is a pure projection of a single state node — one column, empty hold, and an on-set of a
/// single one-literal cube — return that node's table name and whether the projection is inverted
/// (`on = [[Some(false)]]`). Otherwise `None`, so the output falls back to `state_function`.
fn projection_of(sr: &StateRegions, model: &StateModel) -> Option<(Symbol, bool)> {
    if sr.cols.len() != 1 || !sr.hold.is_empty() {
        return None;
    }
    let node = model.node_of.get(&sr.cols[0])?.clone();
    match sr.on.as_slice() {
        [cube] if cube.as_slice() == [Some(true)] => Some((node, false)),
        [cube] if cube.as_slice() == [Some(false)] => Some((node, true)),
        _ => None,
    }
}

/// `statetable ("<inputs>", "<nodes>") { table : "<rows>"; }` — the cell's single joint next-state
/// table. Node values are space-separated within a field; rows are comma-separated.
fn statetable_group(model: &StateModel) -> Group {
    let header = format!(
        "\"{}\", \"{}\"",
        join_nodes(&model.input_nodes),
        join_nodes(&model.internal_nodes),
    );
    let mut st = Group::new("statetable", &header);
    set_attr(&mut st, "table", Value::String(table_string(model)));
    st
}

/// Render the joint table body: one `<inputs> : <current> : <next>` row per [`StateModel::rows`] entry,
/// comma-and-newline-joined (one statetable row per line in the emitted Liberty) in model (deterministic)
/// order. Inputs/current use `H`/`L`/`-`; next uses `H`/`L`/`N`.
fn table_string(model: &StateModel) -> String {
    model
        .rows
        .iter()
        .map(|row| {
            format!(
                "{} : {} : {}",
                state_pattern(&row.inputs),
                state_pattern(&row.current),
                next_pattern(&row.next),
            )
        })
        .collect::<Vec<_>>()
        .join(" ,\n")
}

/// Render a per-node next-state action vector as space-separated `H`/`L`/`N`/`-` symbols. A `None` slot
/// is a node this row leaves unconstrained (`-`), deferred to a lower-priority row per Liberty's
/// per-output next-state resolution.
fn next_pattern(next: &[Option<Next>]) -> String {
    next.iter()
        .map(|n| match n {
            Some(Next::High) => "H",
            Some(Next::Low) => "L",
            Some(Next::Hold) => "N",
            None => "-",
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Space-join a node list into a single statetable-header field.
fn join_nodes(nodes: &[Symbol]) -> String {
    nodes
        .iter()
        .map(Symbol::as_str)
        .collect::<Vec<_>>()
        .join(" ")
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

    /// Wrap a bare cell fragment and assert it round-trips through `liberty_parse::parse_lib`.
    fn parse_frag(frag: &str) -> Liberty {
        let wrapped = format!("library (test) {{\n{frag}}}\n");
        liberty_parse::parse_lib(&wrapped).expect("emitted Liberty must parse")
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
        // One joint statetable over the aliased output node `Q_st`.
        assert!(lib.contains("statetable (\"A B\", \"Q_st\")"));
        assert!(lib.contains("H H : - : H")); // on
        assert!(lib.contains("L L : - : L")); // off
        assert!(lib.contains("H L : - : N")); // hold
        assert!(lib.contains("L H : - : N")); // hold
                                              // The output pin binds to its `_st` node; no combinational `function`.
        assert!(lib.contains("pin (Q)"));
        assert!(lib.contains("internal_node : \"Q_st\";"));
        assert!(lib.contains("inverted_output : false;"));
        assert!(!lib.contains("function : "));
        parse_frag(&lib);
    }

    #[test]
    fn dff_emits_one_joint_statetable() {
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
        // Exactly one joint table over both state nodes (aliased Q_st and internal M).
        assert_eq!(frag.matches("statetable").count(), 1);
        assert!(frag.contains("statetable (\"CLK D\", \"Q_st M\")"));
        // Per-output next-state rows (Q first, M second): Q rows constrain CLK/M and defer M (`-`);
        // M rows constrain CLK/D and defer Q (`-`).
        assert!(frag.contains("H - : - H : H -")); // Q drives high off M currently high
        assert!(frag.contains("H - : - L : L -")); // Q drives low off M currently low
        assert!(frag.contains("L - : - - : N -")); // Q holds while CLK low
        assert!(frag.contains("L H : - - : - H")); // M samples D high while CLK low
        assert!(frag.contains("L L : - - : - L")); // M samples D low while CLK low
        assert!(frag.contains("H - : - - : - N")); // M holds while CLK high
                                                   // The master is a genuine internal pin anchoring its same-named node.
        assert!(frag.contains("pin (M)"));
        assert!(frag.contains("direction : internal;"));
        assert!(frag.contains("internal_node : \"M\";"));
        assert!(!frag.contains("function : "));
        // The fragment still round-trips through liberty-parse.
        let lib = parse_frag(&frag);
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
    fn gated_latch_projection_output_uses_state_function() {
        // GL: internal L self-holds (a state node); output Y = C*L is combinational over pins C and L.
        let cell = analyse(
            r#"
[[cell]]
name = "GL"
inputs = ["C", "D"]
[cell.internal]
L = "!C*D + C*L"
[cell.outputs]
Y = "C*L"
"#,
        );
        let frag = cell_liberty(&cell);
        eprintln!("{frag}");
        assert!(frag.contains("statetable (\"C D\", \"L\")"));
        // Y names PIN ports through state_function — inputs and the internal pin L, never an `_st` node.
        assert!(frag.contains("state_function : "));
        assert!(frag.contains('C') && frag.contains('L'));
        // Y is combinational: it carries no internal_node of its own.
        let lib = parse_frag(&frag);
        let cellg = lib
            .iter()
            .flat_map(|g| g.subgroups.iter())
            .find(|g| g.type_ == "cell" && g.name == "GL")
            .expect("GL cell present");
        let y = cellg
            .subgroups
            .iter()
            .find(|g| g.type_ == "pin" && g.name == "Y")
            .expect("Y pin present");
        assert!(!y.attributes.contains_key("internal_node"));
        // The internal state node L is a direction:internal pin.
        assert!(frag.contains("pin (L)"));
        assert!(frag.contains("direction : internal;"));
    }

    #[test]
    fn mutex_emits_joint_statetable_no_function() {
        let cell = analyse(
            r#"
[[cell]]
name = "MUT"
inputs = ["A", "B"]
[cell.outputs]
Qa = "!Qb * A"
Qb = "!Qa * B"
"#,
        );
        let frag = cell_liberty(&cell);
        eprintln!("{frag}");
        assert!(frag.contains("statetable (\"A B\", \"Qa_st Qb_st\")"));
        // The old joint race row `H H : L L : H H` is split into two per-output rows: each grant drives
        // high off its own request and the other grant currently low, deferring (`-`) the other node.
        assert!(!frag.contains("H H : L L : H H"));
        assert!(frag.contains("H - : - L : H -"));
        assert!(frag.contains("- H : L - : - H"));
        // Both outputs bind to their `_st` nodes — no `function` (nor `state_function`) token anywhere.
        assert!(!frag.contains("function : "));
        parse_frag(&frag);
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
