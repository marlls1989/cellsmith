//! Unified joint state-table model for a sequential cell — the single source the Liberty renderer
//! and the behavioural Verilog draw from, built at **emission time** (not in the minimiser).
//!
//! A cell's hysteretic signals (its **state variables**: outputs and internal nodes on a dependency
//! cycle, as classified in [`crate::logic::regions`]) are folded into ONE joint next-state table over
//! the cross product of their per-signal regions. Each [`StateRow`] pairs a primary-input pattern and
//! a current-state pattern with a per-node next-state action (`H`/`L`/`N` = drive-high/drive-low/hold).
//! Output state variables get an emission-time `{name}_st` alias node so no state-table node ever names
//! an external output pin; internal state nodes keep their own name.
//!
//! LIBERTY SPEC FACTS (verified against the Liberty User Guide Vol.1 2017.06 §5 pp.5-23..5-33 and the
//! Liberty Reference Manual R-2020.09 p.217):
//! - A sequential cell carries exactly **one** `statetable` group; every state node of the cell is a
//!   column of that one table.
//! - Within each table field, node values are **space-separated**; whole rows are **comma-separated**.
//!   Master-slave example:
//!   `statetable ("D CP CPN", "MQ SQ") { table : "H/L R ~F : - - : H/L N,\ ..." }`
//! - The statetable node namespace is **isolated** from the pin namespace; each node is resolved to a
//!   port through a pin's `internal_node` attribute. An output pin therefore reads its `_st` node, and
//!   the node name may differ from any pin name.

use std::collections::{BTreeMap, BTreeSet};

use espresso_logic::Symbol;

use crate::logic::regions::{StateCube, StateRegions};
use crate::model::{AnalysedCell, AnalysedOutput};

/// A single state node's next-state action in a joint row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Next {
    /// Drive the node high (the signal's `on` region — Liberty `H`).
    High,
    /// Drive the node low (the signal's `off` region — Liberty `L`).
    Low,
    /// Hold the node (the signal's `hold` region — Liberty `N`).
    Hold,
}

/// One row of the joint state table: an input pattern and a current-state pattern mapped to a
/// per-node next action. `inputs` is aligned to [`StateModel::input_nodes`], `current` and `next` to
/// [`StateModel::internal_nodes`] (i.e. the state signals in node order). `Some(true)`/`Some(false)`
/// are fixed levels; `None` is a don't-care (`-`).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct StateRow {
    pub inputs: Vec<Option<bool>>,
    pub current: Vec<Option<bool>>,
    pub next: Vec<Next>,
}

/// The joint state-table model of a sequential cell: the input-node and internal-node column headers,
/// the original-signal → node-name map, and the deduplicated, sorted joint rows.
#[derive(Debug)]
pub struct StateModel {
    /// Primary-input columns, ordered by the cell's input-pin order.
    pub input_nodes: Vec<Symbol>,
    /// State-node columns (`current`/`next` order): the state signals in `signals()` order, each mapped
    /// to its node name (`{name}_st` alias for an output, own name for an internal).
    pub internal_nodes: Vec<Symbol>,
    /// Each state signal's ORIGINAL name → its state-table node name.
    pub node_of: BTreeMap<Symbol, Symbol>,
    /// The joint next-state rows, deduplicated and sorted.
    pub rows: Vec<StateRow>,
}

/// Mint a fresh state-node name for an output state variable: `{name}_st`, escalating `{name}_st2`,
/// `{name}_st3`, … until it collides with no `reserved` name. (Naming lifted from the deleted minimiser
/// hoist, old `minimise.rs:476-495`.)
fn mint_node(name: &Symbol, reserved: &BTreeSet<Symbol>) -> Symbol {
    let mut i = 1usize;
    loop {
        let cand = if i == 1 {
            Symbol::from(format!("{name}_st"))
        } else {
            Symbol::from(format!("{name}_st{i}"))
        };
        if !reserved.contains(&cand) {
            break cand;
        }
        i += 1;
    }
}

/// Extend a partial assignment with a region cube's fixed literals, keyed by the ORIGINAL column
/// symbol (a primary input or another state signal's current value). Returns `None` if any literal
/// conflicts with the assignment (the branch is pruned).
fn extend(
    assignment: &BTreeMap<Symbol, bool>,
    cols: &[Symbol],
    cube: &StateCube,
) -> Option<BTreeMap<Symbol, bool>> {
    let mut a = assignment.clone();
    for (col, val) in cols.iter().zip(cube.iter()) {
        if let Some(b) = val {
            match a.get(col) {
                Some(existing) if existing != b => return None,
                _ => {
                    a.insert(col.clone(), *b);
                }
            }
        }
    }
    Some(a)
}

/// DFS over the state signals in node order, branching over each signal's three regions (on→`High`,
/// off→`Low`, hold→`Hold`) and each cube within them. `assignment` carries the accumulated
/// input/current-value literals; `tags` the accumulated per-node next actions. At a leaf every node has
/// a tag, so materialise one [`StateRow`].
#[allow(clippy::too_many_arguments)]
fn build_rows(
    state_sigs: &[(&AnalysedOutput, &StateRegions)],
    idx: usize,
    assignment: &BTreeMap<Symbol, bool>,
    tags: &[Next],
    input_nodes: &[Symbol],
    rows: &mut BTreeSet<StateRow>,
) {
    if idx == state_sigs.len() {
        let inputs = input_nodes
            .iter()
            .map(|n| assignment.get(n).copied())
            .collect();
        // A node's own current value is never constrained by its own cube (self is projected out in
        // regions.rs:79-83); it stays `None` unless another node's cube constrained it.
        let current = state_sigs
            .iter()
            .map(|(sig, _)| assignment.get(&sig.name).copied())
            .collect();
        rows.insert(StateRow {
            inputs,
            current,
            next: tags.to_vec(),
        });
        return;
    }

    let (_, sr) = state_sigs[idx];
    for (region, tag) in [
        (&sr.on, Next::High),
        (&sr.off, Next::Low),
        (&sr.hold, Next::Hold),
    ] {
        for cube in region {
            if let Some(next_assign) = extend(assignment, &sr.cols, cube) {
                let mut next_tags = tags.to_vec();
                next_tags.push(tag);
                build_rows(
                    state_sigs,
                    idx + 1,
                    &next_assign,
                    &next_tags,
                    input_nodes,
                    rows,
                );
            }
        }
    }
}

/// Build the joint state-table model of a cell, or `None` if the cell has no state variable (a purely
/// combinational cell emits `function:`, never a `statetable`).
///
/// State signals are the cell's hysteretic [`signal_regions`](AnalysedCell::signal_regions) entries —
/// identical to [`crate::logic::resolve::state_variables`] by construction, so this reads the cached
/// `hysteretic` flag rather than re-running the classifier.
pub fn build_state_model(cell: &AnalysedCell) -> Option<StateModel> {
    // (a) State signals = the hysteretic signals, in `signals()` order.
    let state_names: BTreeSet<Symbol> = cell
        .signal_regions()
        .filter(|(_, sr)| sr.hysteretic)
        .map(|(sig, _)| sig.name.clone())
        .collect();
    if state_names.is_empty() {
        return None;
    }

    // (b) node_of: an OUTPUT state variable mints a fresh `{name}_st` node so the table never names an
    // external output pin; a genuine INTERNAL state node keeps its own name. `reserved` is the cell's
    // inputs ∪ all output names ∪ all internal names ∪ mints-so-far.
    let mut reserved: BTreeSet<Symbol> = cell.inputs.iter().cloned().collect();
    reserved.extend(cell.outputs.iter().map(|o| o.name.clone()));
    reserved.extend(cell.internals.iter().map(|s| s.name.clone()));

    let n_out = cell.outputs.len();
    let mut node_of: BTreeMap<Symbol, Symbol> = BTreeMap::new();
    let mut internal_nodes: Vec<Symbol> = Vec::new();
    for (i, (sig, sr)) in cell.signal_regions().enumerate() {
        if !sr.hysteretic {
            continue;
        }
        let node = if i < n_out {
            let m = mint_node(&sig.name, &reserved);
            reserved.insert(m.clone());
            m
        } else {
            sig.name.clone()
        };
        node_of.insert(sig.name.clone(), node.clone());
        internal_nodes.push(node);
    }

    // (c) Column partition: a state-signal-named col is a current-value column (middle field); every
    // other col is an input-node column — I3 guarantees it is a primary input (minimise.rs invariant
    // I3), asserted below. input_nodes = the union of input-side cols, ordered by cell.inputs order.
    let mut input_cols: BTreeSet<Symbol> = BTreeSet::new();
    for (_, sr) in cell.signal_regions().filter(|(_, sr)| sr.hysteretic) {
        for col in &sr.cols {
            if !state_names.contains(col) {
                debug_assert!(
                    cell.inputs.contains(col),
                    "I3: non-state-signal state-table column {col} must be a primary input",
                );
                input_cols.insert(col.clone());
            }
        }
    }
    let input_nodes: Vec<Symbol> = cell
        .inputs
        .iter()
        .filter(|i| input_cols.contains(*i))
        .cloned()
        .collect();

    // (d) Joint rows: DFS over the state signals in node order, deduped in a BTreeSet and returned
    // sorted. Overlapping-cube duplicates dedupe because identical input/current patterns always share
    // a next-vector (per-node regions are disjoint).
    let state_sigs: Vec<(&AnalysedOutput, &StateRegions)> = cell
        .signal_regions()
        .filter(|(_, sr)| sr.hysteretic)
        .collect();
    let mut rows: BTreeSet<StateRow> = BTreeSet::new();
    build_rows(
        &state_sigs,
        0,
        &BTreeMap::new(),
        &[],
        &input_nodes,
        &mut rows,
    );

    Some(StateModel {
        input_nodes,
        internal_nodes,
        node_of,
        rows: rows.into_iter().collect(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::analyse_one as analyse;

    const T: Option<bool> = Some(true);
    const F: Option<bool> = Some(false);
    const X: Option<bool> = None;

    fn names(v: &[Symbol]) -> Vec<&str> {
        v.iter().map(Symbol::as_str).collect()
    }

    #[test]
    fn c2_joint_table() {
        let cell = analyse(
            r#"
[[cell]]
name = "C2"
inputs = ["A", "B"]
[cell.outputs]
Q = "A*B + Q*(A+B)"
"#,
        );
        let m = build_state_model(&cell).expect("C2 is sequential");
        assert_eq!(names(&m.input_nodes), ["A", "B"]);
        assert_eq!(names(&m.internal_nodes), ["Q_st"]);
        // The on row (A*B) and the off row (!A*!B); Q's own current is projected out (None).
        assert!(m.rows.contains(&StateRow {
            inputs: vec![T, T],
            current: vec![X],
            next: vec![Next::High],
        }));
        assert!(m.rows.contains(&StateRow {
            inputs: vec![F, F],
            current: vec![X],
            next: vec![Next::Low],
        }));
        // Two hold rows (A xor B).
        let holds = m.rows.iter().filter(|r| r.next == [Next::Hold]).count();
        assert_eq!(holds, 2);
    }

    #[test]
    fn dff_joint_table_internal_unaliased() {
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
        let m = build_state_model(&cell).expect("DFF is sequential");
        // Q is aliased to Q_st; the internal master M keeps its own name.
        assert_eq!(names(&m.internal_nodes), ["Q_st", "M"]);
        assert_eq!(m.node_of[&Symbol::from("M")], "M");
        assert_eq!(m.node_of[&Symbol::from("Q")], "Q_st");
        // Exactly the four joint rows, including: Q drives high off M's current, M holds through CLK.
        assert_eq!(m.rows.len(), 4);
        assert!(m.rows.contains(&StateRow {
            inputs: vec![T, X],
            current: vec![X, T],
            next: vec![Next::High, Next::Hold],
        }));
    }

    #[test]
    fn mut_joint_table_race_row() {
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
        let m = build_state_model(&cell).expect("MUT is sequential");
        assert_eq!(names(&m.internal_nodes), ["Qa_st", "Qb_st"]);
        // The race: both requests high, both grants currently low → both drive high.
        assert!(m.rows.contains(&StateRow {
            inputs: vec![T, T],
            current: vec![F, F],
            next: vec![Next::High, Next::High],
        }));
    }

    #[test]
    fn sr_input_and_node_headers() {
        let cell = analyse(
            r#"
[[cell]]
name = "SR"
inputs = ["S", "R"]
[cell.outputs]
Q = "!(R + Qn)"
Qn = "!(S + Q)"
"#,
        );
        let m = build_state_model(&cell).expect("SR is sequential");
        assert_eq!(names(&m.input_nodes), ["S", "R"]);
        assert_eq!(names(&m.internal_nodes), ["Q_st", "Qn_st"]);
    }

    #[test]
    fn collision_escalates_past_reserved_input() {
        // A reserved input named `Q_st` forces the mint to escalate to `Q_st2`.
        let cell = analyse(
            r#"
[[cell]]
name = "COLL"
inputs = ["A", "Q_st"]
[cell.outputs]
Q = "A*Q_st + Q*(A+Q_st)"
"#,
        );
        let m = build_state_model(&cell).expect("COLL is sequential");
        assert_eq!(names(&m.internal_nodes), ["Q_st2"]);
        assert_eq!(m.node_of[&Symbol::from("Q")], "Q_st2");
    }

    #[test]
    fn gated_latch_combinational_output_absent_from_nodes() {
        // GL: internal L self-holds (state), output Y = C*L is combinational (no cycle). Only L is a
        // node; Y is not in node_of, but the cell is still sequential.
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
        let m = build_state_model(&cell).expect("GL is sequential");
        assert_eq!(names(&m.internal_nodes), ["L"]);
        assert!(!m.node_of.contains_key(&Symbol::from("Y")));
        assert_eq!(m.node_of[&Symbol::from("L")], "L");
    }

    #[test]
    fn combinational_cell_has_no_model() {
        let cell = analyse(
            r#"
[[cell]]
name = "ND2"
inputs = ["A", "B"]
[cell.outputs]
Y = "!(A*B)"
"#,
        );
        assert!(build_state_model(&cell).is_none());
    }
}
