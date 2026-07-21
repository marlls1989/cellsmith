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
//! 'Sequential' is a property of an OUTPUT, not of the cell: each output pin is classified per output
//! (Liberty UG Vol.1 pp.5-31..5-33):
//! - (A) an output that IS a state variable binds `internal_node : "<own name>"` — the node carries
//!   the output's own name, so no alias is minted;
//! - (B) an output whose regions reference a state node carries `state_function : "<sop>"`, which names
//!   PIN ports (inputs, internal pins, or output pins carrying an `internal_node`) — including a former
//!   feedthrough or inverter of a single state node, rendered by the ordinary SOP renderer as a plain
//!   or negated literal (e.g. `!Q`) (Liberty UG Vol.1 p.5-31, and the `pin(QNZ){state_function:"QN"}`
//!   / feedthrough `pin(Y){state_function:"A"}` examples on p.5-33);
//! - (C) an output over primary inputs only carries a plain `function : "<sop>"`, EVEN inside a cell
//!   that has a statetable.
//!
//! A GENUINE INTERNAL state node keeps its own name and is emitted as a
//! `direction : internal; internal_node : "<name>";` pin (Liberty UG Vol.1 `pin(n1)` example).
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

use crate::emit::statetable::{build_state_model, EdgeRow, EdgeTok, Next, StateModel};
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
    let groups: Vec<Group> = cell.name.iter().map(|n| cell_group(cell, n)).collect();
    out.push_str(&format!("{}\n", Liberty(groups)));
    out
}

/// Build the `cell` group: one input `pin` per primary input, then — for a sequential cell — the single
/// joint `statetable`, its output pins (in declaration order), and its genuine-internal pins. Output
/// pins are classified per output (see [`output_pin`]): a state variable, a state-dependent function,
/// or a plain combinational `function` even alongside a statetable. A cell with no state model — purely
/// combinational, or one the fold emptied — emits each output as a plain `function` pin and no internal
/// pin at all. `name` is the cell name this group is emitted under; a cell with several declared names
/// yields one identical group per name.
fn cell_group(cell: &AnalysedCell, name: &Symbol) -> Group {
    let mut group = Group::new("cell", name);

    for input in &cell.inputs {
        group.subgroups.push(input_pin(input));
    }

    match build_state_model(cell) {
        // No state model: every output carries a plain `function`. The cell is either purely
        // combinational, or the fold emptied its state model outright. A folded internal still sits in
        // `cell.internals` -- that list is pruned by the state-space minimisation, a different mechanism
        // from the fold -- so the internals are filtered here rather than assumed away: an internal state
        // node emits no external pin, and rendering its region as a `function` would strip the hold term
        // and publish the node as a spurious output.
        None => {
            debug_assert!(
                cell.internals
                    .iter()
                    .all(|s| cell.edge.folded.contains(&s.name)),
                "no state model: every surviving internal is folded -- minimise I3 plus the fold"
            );
            let n_out = cell.outputs.len();
            for (sig, sr) in cell.signal_regions().take(n_out) {
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
                // A folded master carries no node in the collapsed model — skip its internal pin so only
                // surviving state nodes (`node_of` members) get a pin group.
                if i >= n_out && model.node_of.contains_key(&sig.name) {
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

/// An output pin of a cell that has a joint statetable, classified per output (Liberty UG Vol.1
/// pp.5-31..5-33):
/// - (A) an output that IS a state variable (a table node) reads its own node via `internal_node`;
/// - (B) an output that references a state node names PIN ports through `state_function`;
/// - (C) an output over primary inputs only carries a plain `function`.
fn output_pin(name: &Symbol, sr: &StateRegions, model: &StateModel) -> Group {
    let mut pin = Group::new("pin", name);
    set_attr(
        &mut pin,
        "direction",
        Value::Expression("output".to_owned()),
    );

    if let Some(node) = model.node_of.get(name) {
        // (A) This output IS a state variable — read its own node. (Its cols may reference other
        // nodes, so this must be checked before the state-dependence predicate below.)
        set_attr(
            &mut pin,
            "internal_node",
            Value::String(node.as_str().to_owned()),
        );
    } else if sr.cols.iter().any(|c| model.node_of.contains_key(c)) {
        // (B) This output DEPENDS on a state node — a state_function over PIN ports.
        set_attr(&mut pin, "state_function", Value::String(function_sop(sr)));
    } else {
        // (C) Combinational output over primary inputs only — a plain `function`. By minimise
        // invariant I3 a surviving combinational output's support is inputs + state nodes only, so
        // 'no node_of column' == 'no transitive state dependence' (ref statetable.rs:112-122).
        set_attr(&mut pin, "function", Value::String(function_sop(sr)));
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
/// then the edge-triggered rows ([`StateModel::edge_rows`]), comma-and-newline-joined (one statetable row
/// per line in the emitted Liberty) in model (deterministic) order. Inputs/current use `H`/`L`/`-`; an
/// edge row's clock column instead carries the token `R`/`F`/`~R`/`~F`; next uses `H`/`L`/`N`.
fn table_string(model: &StateModel) -> String {
    let mut rows: Vec<String> = model
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
        .collect();
    for er in &model.edge_rows {
        rows.push(format!(
            "{} : {} : {}",
            edge_input_pattern(er, &model.input_nodes),
            state_pattern(&er.current),
            next_pattern(&er.next),
        ));
    }
    rows.join(" ,\n")
}

/// Render an edge row's input field: the ordinary `H`/`L`/`-` level symbols, except the register's clock
/// column carries the edge token (`R`/`F`/`~R`/`~F`) in place of a level.
fn edge_input_pattern(er: &EdgeRow, input_nodes: &[Symbol]) -> String {
    input_nodes
        .iter()
        .zip(er.inputs.iter())
        .map(|(node, val)| {
            if *node == er.clock {
                edge_token(er.token)
            } else {
                level_symbol(val)
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// The Liberty state-table symbol for a clock-edge token. A dual-edge register's off-edge row owns
/// neither clock face, so its clock column prints the level don't-care `-`.
fn edge_token(token: EdgeTok) -> &'static str {
    match token {
        EdgeTok::Rise => "R",
        EdgeTok::Fall => "F",
        EdgeTok::NotRise => "~R",
        EdgeTok::NotFall => "~F",
        EdgeTok::Level => "-",
    }
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
    cube.iter().map(level_symbol).collect::<Vec<_>>().join(" ")
}

/// The Liberty state-table level symbol for one cube value: `H`/`L`/`-`.
fn level_symbol(val: &Option<bool>) -> &'static str {
    match val {
        Some(true) => "H",
        Some(false) => "L",
        None => "-",
    }
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

    /// Locate a cell group by name in a parsed library.
    fn find_cell<'a>(lib: &'a Liberty, name: &str) -> &'a Group {
        lib.iter()
            .flat_map(|g| g.subgroups.iter())
            .find(|g| g.type_ == "cell" && g.name == name)
            .unwrap_or_else(|| panic!("{name} cell present"))
    }

    /// Locate a pin group by name inside a cell group.
    fn find_pin<'a>(cellg: &'a Group, name: &str) -> &'a Group {
        cellg
            .subgroups
            .iter()
            .find(|g| g.type_ == "pin" && g.name == name)
            .unwrap_or_else(|| panic!("{name} pin present"))
    }

    /// The string value of a pin's simple `String` attribute, if present.
    fn attr_string(pin: &Group, name: &str) -> Option<String> {
        match pin.attributes.get(name)?.first()? {
            Attribute::Simple(Value::String(s)) => Some(s.clone()),
            _ => None,
        }
    }

    /// The keyword value of a pin's simple `Expression` attribute (e.g. `direction : internal`).
    fn attr_expr(pin: &Group, name: &str) -> Option<String> {
        match pin.attributes.get(name)?.first()? {
            Attribute::Simple(Value::Expression(s)) => Some(s.clone()),
            _ => None,
        }
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
        // One joint statetable over the output's own node `Q`.
        assert!(lib.contains("statetable (\"A B\", \"Q\")"));
        assert!(lib.contains("H H : - : H")); // on
        assert!(lib.contains("L L : - : L")); // off
        assert!(lib.contains("H L : - : N")); // hold
        assert!(lib.contains("L H : - : N")); // hold
                                              // The output pin binds to its own node; no combinational `function`.
        assert!(lib.contains("pin (Q)"));
        assert!(lib.contains("internal_node : \"Q\";"));
        assert!(!lib.contains("function : "));
        parse_frag(&lib);
    }

    #[test]
    fn dff_emits_one_joint_statetable() {
        // MIGRATED two-latch coverage: the same DFF with a declared clock but collapse opted OUT keeps
        // its master-slave statetable (`Q M` nodes, six per-output rows, a `pin (M)`).
        let cell = analyse(
            r#"
[[cell]]
name = "DFF"
inputs = ["CLK", "D"]
clock = ["CLK"]
no_edge_collapse = true
[cell.internal]
M = "!CLK*D + CLK*M"
[cell.outputs]
Q = "CLK*M + !CLK*Q"
"#,
        );
        let frag = cell_liberty(&cell);
        eprintln!("{frag}");
        // Exactly one joint table over both state nodes (output Q and internal M).
        assert_eq!(frag.matches("statetable").count(), 1);
        assert!(frag.contains("statetable (\"CLK D\", \"Q M\")"));
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
    fn dff_collapses_to_edge_statetable() {
        // Default (collapse ON) with a declared clock: the master-slave DFF becomes ONE rising-edge
        // register Q, folding M away. The table carries only the register's node `Q` and edge rows.
        let cell = analyse(
            r#"
[[cell]]
name = "DFF"
inputs = ["CLK", "D"]
clock = ["CLK"]
[cell.internal]
M = "!CLK*D + CLK*M"
[cell.outputs]
Q = "CLK*M + !CLK*Q"
"#,
        );
        let frag = cell_liberty(&cell);
        eprintln!("{frag}");
        assert_eq!(frag.matches("statetable").count(), 1);
        assert!(frag.contains("statetable (\"CLK D\", \"Q\")"));
        // Rising-edge capture (R) drives Q from D; the off-edge face (~R) holds.
        assert!(frag.contains("R H : - : H"));
        assert!(frag.contains("R L : - : L"));
        assert!(frag.contains("~R - : - : N"));
        // The folded master M keeps no pin group and no node column in the statetable header.
        assert!(!frag.contains("pin (M)"));
        assert!(!frag.contains("\"Q M\""));
        assert!(!frag.contains("function : "));
        let lib = parse_frag(&frag);
        let cellg = find_cell(&lib, "DFF");
        assert!(!cellg
            .subgroups
            .iter()
            .any(|g| g.type_ == "pin" && g.name == "M"));
        // Q binds its own node exactly as an uncollapsed state output would.
        let q = find_pin(cellg, "Q");
        assert_eq!(attr_string(q, "internal_node").as_deref(), Some("Q"));
    }

    #[test]
    fn ndff_group_folds_the_mutually_referencing_nand_master_pair() {
        // The cross-coupled-NAND master-slave flop: M/Mn are captureless and mutually referencing, so
        // they fold together exactly as the pass DFF's lone M folds. Q and Qn survive as the two edge
        // registers (Qn carries its own genuine !D capture).
        let cell = analyse(
            r#"
[[cell]]
name = "NDFF"
inputs = ["CLK", "D"]
clock = ["CLK"]
[cell.internal]
Mn = "!( !(!D*!CLK) * M )"
M = "!( !(D*!CLK) * Mn )"
[cell.outputs]
Qn = "!( !(!M*CLK) * Q )"
Q = "!( !(M*CLK) * Qn )"
"#,
        );
        let frag = cell_liberty(&cell);
        eprintln!("{frag}");
        assert_eq!(frag.matches("statetable").count(), 1);
        assert!(frag.contains("statetable (\"CLK D\", \"Q Qn\")"));
        // The folded master pair keeps no pin group and no node column in the statetable header.
        assert!(!frag.contains("pin (M)"));
        assert!(!frag.contains("pin (Mn)"));
        assert!(!frag.contains("function : "));
        let lib = parse_frag(&frag);
        let cellg = find_cell(&lib, "NDFF");
        for gone in ["M", "Mn"] {
            assert!(
                !cellg
                    .subgroups
                    .iter()
                    .any(|g| g.type_ == "pin" && g.name == gone),
                "folded master {gone} must not emit a pin"
            );
        }
        // Q and Qn bind their own nodes as the two surviving edge registers.
        let q = find_pin(cellg, "Q");
        assert_eq!(attr_string(q, "internal_node").as_deref(), Some("Q"));
        let qn = find_pin(cellg, "Qn");
        assert_eq!(attr_string(qn, "internal_node").as_deref(), Some("Qn"));
    }

    #[test]
    fn a_wholly_folded_internal_leaves_a_combinational_cell_with_no_pin() {
        // The fold can empty the state model outright: L is the cell's only state signal and it reaches
        // no output, so once it folds `build_state_model` returns None and the cell renders through the
        // combinational branch. L nonetheless survives in `cell.internals` -- that list is pruned by the
        // state-space minimisation, a different mechanism from the fold -- so the combinational branch
        // still sees it as a signal. It must emit NO pin: an internal state node has no external pin, and
        // rendering its region as a `function` would strip the hold term and publish `D*!CLK` as an
        // output.
        let cell = analyse(
            r#"
[[cell]]
name = "FOLDONLY"
inputs = ["CLK", "A", "D"]
clock = ["CLK"]
[cell.internal]
L = "!CLK*D + CLK*L"
[cell.outputs]
Y = "A*D"
"#,
        );
        assert_eq!(
            cell.edge
                .folded
                .iter()
                .map(Symbol::as_str)
                .collect::<Vec<_>>(),
            ["L"],
            "premise: L is folded"
        );
        let frag = cell_liberty(&cell);
        eprintln!("{frag}");
        assert!(!frag.contains("statetable"), "the fold emptied the model");
        assert!(!frag.contains("pin (L)"), "folded internal emits no pin");
        let lib = parse_frag(&frag);
        let cellg = find_cell(&lib, "FOLDONLY");
        assert!(
            !cellg
                .subgroups
                .iter()
                .any(|g| g.type_ == "pin" && g.name == "L"),
            "folded internal L must not emit a pin"
        );
        // The genuine output is untouched.
        let y = find_pin(cellg, "Y");
        assert_eq!(attr_expr(y, "direction").as_deref(), Some("output"));
        assert_eq!(attr_string(y, "function").as_deref(), Some("A*D"));
    }

    #[test]
    fn icm_collapses_shared_boundary_registers_with_both_edges() {
        // Two three-latch synchronisers collapse to four edge registers (sela2/enA on CLKA,
        // selb2/enB on CLKB); the folded relays sela1/selb1 vanish, and GCLK stays a state_function.
        let cell = analyse(
            r#"
[[cell]]
name = "ICM"
inputs = ["CLKA", "CLKB", "RA", "RB", "S"]
clock = ["CLKA", "CLKB"]
[cell.internal]
sela = "!enB*!S"
selb = "!enA*S"
sela1 = "!RA*(!CLKA*sela+CLKA*sela1)"
sela2 = "!RA*(CLKA*sela1+!CLKA*sela2)"
enA   = "!RA*(!CLKA*sela2+CLKA*enA)"
selb1 = "!RB*(!CLKB*selb+CLKB*selb1)"
selb2 = "!RB*(CLKB*selb1+!CLKB*selb2)"
enB   = "!RB*(!CLKB*selb2+CLKB*enB)"
[cell.outputs]
GCLK = "enA*CLKA+enB*CLKB"
"#,
        );
        let frag = cell_liberty(&cell);
        eprintln!("{frag}");
        // Both a rising (sela2/selb2) and a falling (enA/enB) register are present.
        assert!(frag.contains("R "));
        assert!(frag.contains("F "));
        let lib = parse_frag(&frag);
        let cellg = find_cell(&lib, "ICM");
        // The surviving state nodes are exactly the four registers — no folded relays.
        for node in ["sela2", "enA", "selb2", "enB"] {
            assert!(
                cellg
                    .subgroups
                    .iter()
                    .any(|g| g.type_ == "pin" && g.name == node),
                "expected pin {node}"
            );
        }
        for gone in ["sela1", "selb1"] {
            assert!(
                !cellg
                    .subgroups
                    .iter()
                    .any(|g| g.type_ == "pin" && g.name == gone),
                "folded relay {gone} must not emit a pin"
            );
        }
        // GCLK depends on the register nodes: a state_function (branch B), never its own node.
        let gclk = find_pin(cellg, "GCLK");
        assert!(gclk.attributes.contains_key("state_function"));
        assert!(!gclk.attributes.contains_key("internal_node"));
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
        // Y names PIN ports through state_function — inputs and the internal pin L, never a table node.
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
        assert!(frag.contains("statetable (\"A B\", \"Qa Qb\")"));
        // The old joint race row `H H : L L : H H` is split into two per-output rows: each grant drives
        // high off its own request and the other grant currently low, deferring (`-`) the other node.
        assert!(!frag.contains("H H : L L : H H"));
        assert!(frag.contains("H - : - L : H -"));
        assert!(frag.contains("- H : L - : - H"));
        // Both outputs bind to their own nodes — no `function` (nor `state_function`) token anywhere.
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

    #[test]
    fn multi_name_emits_one_identical_cell_group_per_name() {
        let cell = analyse(
            r#"
[[cell]]
name = ["INVX1", "INVX2"]
inputs = ["A"]
[cell.outputs]
Y = "!A"
"#,
        );
        let frag = cell_liberty(&cell);
        eprintln!("{frag}");
        assert!(frag.contains("cell (INVX1)"));
        assert!(frag.contains("cell (INVX2)"));
        let lib = parse_frag(&frag);
        let cell_groups: Vec<_> = lib
            .iter()
            .flat_map(|g| g.subgroups.iter())
            .filter(|g| g.type_ == "cell")
            .collect();
        let invx1 = cell_groups
            .iter()
            .find(|g| g.name == "INVX1")
            .expect("INVX1 cell present");
        let invx2 = cell_groups
            .iter()
            .find(|g| g.name == "INVX2")
            .expect("INVX2 cell present");
        // Identical pin sets: same pin names and directions, differing only in the group name.
        let pins = |g: &&Group| -> Vec<(String, String)> {
            g.subgroups
                .iter()
                .filter(|p| p.type_ == "pin")
                .map(|p| {
                    (
                        p.name.clone(),
                        p.attributes
                            .get("direction")
                            .map(|v| format!("{v:?}"))
                            .unwrap_or_default(),
                    )
                })
                .collect()
        };
        assert_eq!(pins(invx1), pins(invx2));
    }

    #[test]
    fn mixed_cell_classifies_each_output_independently() {
        // Internal latch L self-holds (a state node); output Y = C*L is state-dependent; output
        // Z = A*B is combinational over primary inputs only — even inside a sequential cell.
        let cell = analyse(
            r#"
[[cell]]
name = "MIX"
inputs = ["A", "B", "C", "D"]
[cell.internal]
L = "!C*D + C*L"
[cell.outputs]
Y = "C*L"
Z = "A*B"
"#,
        );
        let frag = cell_liberty(&cell);
        eprintln!("{frag}");
        let lib = parse_frag(&frag);
        let cellg = find_cell(&lib, "MIX");

        // Z is combinational over inputs only: plain `function`, exactly `A*B`, nothing sequential.
        let z = find_pin(cellg, "Z");
        assert_eq!(attr_string(z, "function").as_deref(), Some("A*B"));
        assert!(!z.attributes.contains_key("state_function"));
        assert!(!z.attributes.contains_key("internal_node"));

        // Y depends on the state node L: `state_function`, never a plain function nor its own node.
        let y = find_pin(cellg, "Y");
        assert!(y.attributes.contains_key("state_function"));
        assert!(!y.attributes.contains_key("function"));
        assert!(!y.attributes.contains_key("internal_node"));

        // L is the genuine internal state node.
        let l = find_pin(cellg, "L");
        assert_eq!(attr_expr(l, "direction").as_deref(), Some("internal"));
        assert_eq!(attr_string(l, "internal_node").as_deref(), Some("L"));

        // Exactly one statetable group in the cell.
        assert_eq!(
            cellg
                .subgroups
                .iter()
                .filter(|g| g.type_ == "statetable")
                .count(),
            1
        );
    }

    #[test]
    fn transitive_state_dependence_survives_fold() {
        // Internal latch L self-holds; internal relay W = C*L merely feeds Z2 = W + E. minimise folds
        // the relay W into its consumer Z2 (model.rs:163-165, minimise.rs:456-474), so Z2's cols
        // contain the state node L directly — the transitive case collapses to the direct predicate.
        let cell = analyse(
            r#"
[[cell]]
name = "TRW"
inputs = ["C", "D", "E"]
[cell.internal]
L = "!C*D + C*L"
W = "C*L"
[cell.outputs]
Z2 = "W + E"
"#,
        );
        let frag = cell_liberty(&cell);
        eprintln!("{frag}");
        let lib = parse_frag(&frag);
        let cellg = find_cell(&lib, "TRW");

        // Key check only (deterministic, independent of Espresso SOP ordering): Z2 is state-dependent.
        let z2 = find_pin(cellg, "Z2");
        assert!(z2.attributes.contains_key("state_function"));
        assert!(!z2.attributes.contains_key("function"));
    }

    #[test]
    fn projection_outputs_render_bare_state_literals() {
        // C-element output Q self-holds (a state node); projection outputs Qc = Q and Qn = !Q are
        // aliases of that single node. Outputs are never purged (model.rs:417), so both survive.
        let cell = analyse(
            r#"
[[cell]]
name = "C2P"
inputs = ["A", "B"]
[cell.outputs]
Q = "A*B + Q*(A+B)"
Qc = "Q"
Qn = "!Q"
"#,
        );
        let frag = cell_liberty(&cell);
        eprintln!("{frag}");
        let lib = parse_frag(&frag);
        let cellg = find_cell(&lib, "C2P");

        // Q IS a state variable — output_pin branch A binds its own node, no function/state_function.
        let q = find_pin(cellg, "Q");
        assert_eq!(attr_string(q, "internal_node").as_deref(), Some("Q"));
        assert!(!q.attributes.contains_key("function"));
        assert!(!q.attributes.contains_key("state_function"));

        // Qc is a bare feedthrough of the state node: state_function exactly `Q`.
        let qc = find_pin(cellg, "Qc");
        assert_eq!(attr_string(qc, "state_function").as_deref(), Some("Q"));
        assert!(!qc.attributes.contains_key("function"));
        assert!(!qc.attributes.contains_key("internal_node"));

        // Qn is the negated feedthrough: state_function exactly `!Q`.
        let qn = find_pin(cellg, "Qn");
        assert_eq!(attr_string(qn, "state_function").as_deref(), Some("!Q"));
        assert!(!qn.attributes.contains_key("function"));
        assert!(!qn.attributes.contains_key("internal_node"));
    }

    /// Parse the single-cell `src` and analyse it twice: once as written, once with
    /// `no_edge_collapse` forced true on every cell -- the same blanket mutation the
    /// `--no-edge-collapse` CLI flag applies (main.rs:82-88). Proves the per-cell TOML switch and
    /// the CLI flag are the identical code path, not two independently-tested mechanisms.
    fn analyse_both(src: &str) -> (crate::model::AnalysedCell, crate::model::AnalysedCell) {
        let default = crate::model::parse_spec(src)
            .unwrap()
            .cells
            .remove(0)
            .analyse()
            .unwrap();
        let mut spec = crate::model::parse_spec(src).unwrap();
        for c in &mut spec.cells {
            c.no_edge_collapse = true;
        }
        let forced = spec.cells.remove(0).analyse().unwrap();
        (default, forced)
    }

    /// Three shapes the behavioural classifier leaves fully level (no edge token) even under default (on)
    /// collapse: a single latch, a gated (self-referencing) latch, and a two-latch DFF whose clock is
    /// never declared. Mirrors `statetable.rs`'s shrunk fixtures. The structural pass's MCDFF/EMDFF are no
    /// longer here -- EMDFF's slave Q is now a recognised register (see `emdff_emits_edge_statetable`),
    /// and MCDFF stays level for a different reason (two clocks, see `mcdff_two_clock_stays_level`).
    const NON_COLLAPSIBLE: [&str; 3] = [
        r#"
[[cell]]
name = "DLAT"
inputs = ["CLK", "D"]
clock = ["CLK"]
[cell.outputs]
Q = "CLK*D + !CLK*Q"
"#,
        r#"
[[cell]]
name = "GLAT"
inputs = ["CLK", "D"]
clock = ["CLK"]
[cell.outputs]
Q = "CLK*(D+Q) + !CLK*Q"
"#,
        r#"
[[cell]]
name = "UCDFF"
inputs = ["CLK", "D"]
[cell.internal]
M = "!CLK*D + CLK*M"
[cell.outputs]
Q = "CLK*M + !CLK*Q"
"#,
    ];

    #[test]
    fn non_collapsible_suite_liberty_matches_the_no_edge_collapse_flag() {
        // No `R`/`F`/`~R`/`~F` edge token appears as its own statetable field, whether the flag is
        // left off (default collapse, a no-op on these shapes) or forced on -- and the two runs emit
        // byte-identical Liberty.
        fn has_edge_token(frag: &str) -> bool {
            frag.split_whitespace()
                .any(|tok| matches!(tok, "R" | "F" | "~R" | "~F"))
        }
        for src in NON_COLLAPSIBLE {
            let (default, forced) = analyse_both(src);
            let frag_default = cell_liberty(&default);
            let frag_forced = cell_liberty(&forced);
            assert!(
                !has_edge_token(&frag_default),
                "unexpected edge token in {}",
                default.repr_name()
            );
            assert!(!has_edge_token(&frag_forced));
            assert_eq!(frag_default, frag_forced);
            parse_frag(&frag_default);
        }
    }

    #[test]
    fn dff_opt_out_restores_pin_m_internal_node_via_either_switch() {
        // The two-latch DFF, opted out directly (`no_edge_collapse = true` in the TOML) versus opted
        // out via the CLI-flag-equivalent blanket mutation over the whole spec: both switches restore
        // the SAME two-latch Liberty -- a genuine `pin (M)` carrying `internal_node : "M"`.
        const DFF: &str = r#"
[[cell]]
name = "DFF"
inputs = ["CLK", "D"]
clock = ["CLK"]
[cell.internal]
M = "!CLK*D + CLK*M"
[cell.outputs]
Q = "CLK*M + !CLK*Q"
"#;
        let direct = {
            let mut spec = crate::model::parse_spec(DFF).unwrap();
            spec.cells[0].no_edge_collapse = true;
            spec.cells.remove(0).analyse().unwrap()
        };
        let via_flag = {
            // Mirrors main.rs:82-88's blanket application of `--no-edge-collapse` over every cell.
            let mut spec = crate::model::parse_spec(DFF).unwrap();
            for c in &mut spec.cells {
                c.no_edge_collapse = true;
            }
            spec.cells.remove(0).analyse().unwrap()
        };

        let frag_direct = cell_liberty(&direct);
        let frag_via_flag = cell_liberty(&via_flag);
        for frag in [&frag_direct, &frag_via_flag] {
            assert!(frag.contains("pin (M)"));
            assert!(frag.contains("internal_node : \"M\";"));
        }
        assert_eq!(frag_direct, frag_via_flag);
    }

    #[test]
    fn emdff_emits_edge_statetable_over_surviving_master() {
        // The exposed-master DFF: the behavioural pass recognises the slave Q as a rising-edge register
        // while the declared-output master M survives as a level node. The statetable carries both nodes;
        // Q's rows are edge rows (an `R` token), M's are level rows, and M keeps its own output pin.
        let cell = analyse(
            r#"
[[cell]]
name = "EMDFF"
inputs = ["CLK", "D"]
clock = ["CLK"]
[cell.outputs]
Q = "CLK*M + !CLK*Q"
M = "!CLK*D + CLK*M"
"#,
        );
        let frag = cell_liberty(&cell);
        eprintln!("{frag}");
        assert_eq!(frag.matches("statetable").count(), 1);
        // Node order follows signals() (outputs sorted: M before Q).
        assert!(frag.contains("statetable (\"CLK D\", \"M Q\")"));
        // Q (second column) captures the INPUT D at the rising edge — the cover prefers the input over the
        // internal master M (they coincide over the CLK=0 capture domain); the ~R face holds.
        assert!(frag.contains("R H : - - : - H"));
        assert!(frag.contains("R L : - - : - L"));
        assert!(frag.contains("~R - : - - : - N"));
        // M (first column) is a level latch on CLK, sampling D while transparent-low.
        assert!(frag.contains("L H : - - : H -"));
        assert!(frag.contains("L L : - - : L -"));
        let lib = parse_frag(&frag);
        let cellg = find_cell(&lib, "EMDFF");
        // M is a surviving output binding its own node.
        let m = find_pin(cellg, "M");
        assert_eq!(attr_string(m, "internal_node").as_deref(), Some("M"));
        let q = find_pin(cellg, "Q");
        assert_eq!(attr_string(q, "internal_node").as_deref(), Some("Q"));
    }

    #[test]
    fn mcdff_two_clock_stays_level() {
        // A master/slave pair split across two declared clocks: Q depends transitively on both clocks, so
        // no single clock keys it and the classifier recognises no register -- a fully level joint table.
        let cell = analyse(
            r#"
[[cell]]
name = "MCDFF"
inputs = ["CLKA", "CLKB", "D"]
clock = ["CLKA", "CLKB"]
[cell.internal]
M = "!CLKA*D + CLKA*M"
[cell.outputs]
Q = "CLKB*M + !CLKB*Q"
"#,
        );
        let frag = cell_liberty(&cell);
        eprintln!("{frag}");
        let has_edge_token = frag
            .split_whitespace()
            .any(|t| matches!(t, "R" | "F" | "~R" | "~F"));
        assert!(!has_edge_token, "two-clock pair stays level, no edge token");
        assert!(frag.contains("statetable (\"CLKA CLKB D\", \"Q M\")"));
        parse_frag(&frag);
    }

    #[test]
    fn det_dual_edge_renders_both_tokens_and_level_off_edge() {
        // Dual-edge mux-DET: Q captures D on BOTH clock edges (L1/L2 fold away). The statetable carries an
        // `R` and an `F` capture group, then the off-edge hold as a `-` Level clock column (captures win
        // at the edges by Liberty first-match priority).
        let cell = analyse(
            r#"
[[cell]]
name = "DET"
inputs = ["CLK", "D"]
clock = ["CLK"]
[cell.internal]
L1 = "!CLK*D + CLK*L1"
L2 = "CLK*D + !CLK*L2"
[cell.outputs]
Q = "CLK*L1 + !CLK*L2"
"#,
        );
        let frag = cell_liberty(&cell);
        eprintln!("{frag}");
        assert_eq!(frag.matches("statetable").count(), 1);
        assert!(frag.contains("statetable (\"CLK D\", \"Q\")"));
        // Both clock faces capture D.
        assert!(frag.contains("R H : - : H"));
        assert!(frag.contains("R L : - : L"));
        assert!(frag.contains("F H : - : H"));
        assert!(frag.contains("F L : - : L"));
        // The off-edge hold row prints `-` in the clock column (a Level token) and holds.
        assert!(frag.contains("- - : - : N"));
        // The folded latches L1/L2 keep no pin and no node column.
        assert!(!frag.contains("pin (L1)"));
        assert!(!frag.contains("pin (L2)"));
        parse_frag(&frag);
    }

    #[test]
    fn dcmux_statetable_is_a_level_model() {
        // DCMUX collapses to a LEVEL model: its falls are combinational and the seam fixpoint empties Q's
        // set, so the joint statetable renders level rows with NO edge (R/F) token in any column. The two
        // rise DELAY arcs still render `-type edge` (covered in the arcs_tcl emitter tests).
        let cell = analyse(
            r#"
[[cell]]
name = "DCMUX"
inputs = ["CLKA", "CLKB", "DA", "DB"]
clock = ["CLKA", "CLKB"]
[cell.internal]
MA = "!CLKA*DA + CLKA*MA"
MB = "!CLKB*DB + CLKB*MB"
[cell.outputs]
Q = "CLKA*MA + CLKB*MB + !CLKA*!CLKB*Q"
"#,
        );
        let frag = cell_liberty(&cell);
        eprintln!("{frag}");
        // Column order of the statetable input header.
        let header = frag
            .lines()
            .find(|l| l.contains("statetable ("))
            .expect("a statetable header");
        let cols: Vec<&str> = header
            .split('"')
            .nth(1)
            .expect("input header field")
            .split_whitespace()
            .collect();
        let is_edge = |t: &str| t == "R" || t == "F";
        for line in frag.lines() {
            let Some((pattern, _)) = line.trim().split_once(':') else {
                continue;
            };
            let toks: Vec<&str> = pattern.split_whitespace().collect();
            if toks.len() != cols.len() {
                continue; // not a statetable data row
            }
            assert!(
                toks.iter().all(|t| !is_edge(t)),
                "DCMUX is a level model: no edge token in {line}"
            );
        }
        parse_frag(&frag);
    }
}
