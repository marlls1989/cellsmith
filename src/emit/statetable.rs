//! Unified joint state-table model for a sequential cell — the single source the Liberty renderer
//! and the behavioural Verilog draw from, built at **emission time** (not in the minimiser).
//!
//! A cell's hysteretic signals (its **state variables**: outputs and internal nodes on a dependency
//! cycle, as classified in [`crate::logic::regions`]) are folded into ONE joint next-state table. Each
//! [`StateRow`] pairs a primary-input pattern and a current-state pattern with a per-node next-state
//! action (`H`/`L`/`N` = drive-high/drive-low/hold, or `-` = unconstrained here). Output state
//! variables get an emission-time `{name}_st` alias node so no state-table node ever names an external
//! output pin; internal state nodes keep their own name.
//!
//! CONSTRUCTION. The rows are built by **cover algebra**, not a cube cross-product. Each node's three
//! minimised region covers (on/off/hold, cached on [`StateRegions`]) are re-based onto one **shared
//! header** — the input nodes followed by the state signals' original names — renamed so their single
//! output column carries the node's original name, then stacked (`Cover::extend`) into three
//! multi-output F covers (ON/OFF/HOLD). Each is Espresso joint-minimised independently (cube-shared
//! across nodes), and every resulting cube is folded into a row keyed by its input pattern: each
//! asserted output column stamps that node's slot with the pass's action, the same key accumulating
//! across passes. Unstamped slots stay `-`.
//!
//! A `-` in a next field is legal Liberty and defers that node: Liberty resolves next-state PER OUTPUT,
//! so a node reads the first row specifying it non-`-` and `-` lets a lower-priority row decide (Vol.1
//! §5; the master-slave `CP(R)`/`CPN(F)` split-row example). Because on/off/hold are disjoint per node,
//! no node is ever stamped two different definite actions.
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

use espresso_logic::{Anonymous, Cover, Minimizable, Minterm, Symbol};

use crate::logic::regions::StateRegions;
use crate::model::AnalysedCell;

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
/// [`StateModel::internal_nodes`] (i.e. the state signals in node order). In `inputs`/`current`,
/// `Some(true)`/`Some(false)` are fixed levels and `None` is a don't-care (`-`); in `next`, `Some(_)`
/// is a definite action and `None` is a node this row leaves unconstrained (`-`, deferred to a
/// lower-priority row per Liberty's per-output resolution).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct StateRow {
    pub inputs: Vec<Option<bool>>,
    pub current: Vec<Option<bool>>,
    pub next: Vec<Option<Next>>,
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

    // (d) Joint rows via cover algebra (see the module doc). Shared header = the input nodes followed
    // by the state signals' ORIGINAL names in node order; I3 (section (c)) guarantees every region
    // column is one of these. `state_orig` doubles as the node-order name list for `current`/`next`.
    let state_orig: Vec<Symbol> = cell
        .signal_regions()
        .filter(|(_, sr)| sr.hysteretic)
        .map(|(sig, _)| sig.name.clone())
        .collect();
    let regions: Vec<&StateRegions> = cell
        .signal_regions()
        .filter(|(_, sr)| sr.hysteretic)
        .map(|(_, sr)| sr)
        .collect();
    let shared_header: Vec<Symbol> = input_nodes
        .iter()
        .cloned()
        .chain(state_orig.iter().cloned())
        .collect();
    // ORIGINAL signal name -> its slot index in node order, for stamping outputs BY NAME.
    let index_of: BTreeMap<&Symbol, usize> =
        state_orig.iter().enumerate().map(|(i, n)| (n, i)).collect();
    let k = state_orig.len();

    // Each pass stacks every node's minimised region cover for one action into a single multi-output F
    // cover over the shared header, joint-minimises it, and folds each cube into `row_map`. A zero-cube
    // region cover (e.g. an empty hold set) contributes no column and is skipped — the fold reads
    // outputs BY NAME, so a missing column just means no cube asserts that node in that region.
    type Pick = fn(&StateRegions) -> &Cover<Symbol, Anonymous>;
    let passes: [(Next, Pick); 3] = [
        (Next::High, |sr| &sr.on_cover),
        (Next::Low, |sr| &sr.off_cover),
        (Next::Hold, |sr| &sr.hold_cover),
    ];

    let mut row_map: BTreeMap<Minterm<Symbol>, Vec<Option<Next>>> = BTreeMap::new();
    for (tag, pick) in passes {
        let mut labels: Vec<Symbol> = Vec::new();
        let mut joint: Option<Cover<Symbol, Symbol>> = None;
        for (name, sr) in state_orig.iter().zip(regions.iter().copied()) {
            let cover = pick(sr);
            // rename_outputs needs a one-output header; a zero-cube cover has none — skip it.
            if cover.num_cubes() == 0 {
                continue;
            }
            let column = cover
                .clone()
                .rename_outputs::<Symbol, _>([name.as_str()])
                .expect("single-output region cover renames to one node name")
                .over_vars(shared_header.iter().map(Symbol::as_str));
            labels.push(name.clone());
            match joint.as_mut() {
                None => joint = Some(column),
                Some(acc) => acc.extend(&column),
            }
        }
        let Some(joint) = joint else { continue };
        debug_assert_eq!(
            joint
                .output_labels()
                .iter()
                .cloned()
                .collect::<BTreeSet<_>>(),
            labels.iter().cloned().collect::<BTreeSet<_>>(),
            "joint {tag:?} cover carries exactly the stacked node names",
        );
        // Joint (multi-output, cube-shared) minimisation, falling back to the un-minimised cover.
        let minimised = joint.clone().minimize().unwrap_or(joint);
        for cube in minimised.cubes() {
            let slots = row_map
                .entry(cube.inputs().clone())
                .or_insert_with(|| vec![None; k]);
            for (out, asserted) in cube.outputs().vars().iter().zip(cube.outputs().iter()) {
                if !asserted {
                    continue;
                }
                let i = index_of[out];
                // Accepted disjointness guarantee: on/off/hold are pairwise disjoint PER NODE, so a
                // slot is never stamped two DIFFERENT definite actions; overlapping cubes across passes
                // only ever re-stamp the same tag into a slot.
                debug_assert!(
                    slots[i].is_none() || slots[i] == Some(tag),
                    "state slot for {out} stamped conflicting next actions",
                );
                slots[i] = Some(tag);
            }
        }
    }

    // Emit rows in Minterm order (BTreeMap iteration). Each row reads its input and current levels off
    // the shared-header key BY NAME (an absent column reads `None` = `-`); unstamped next slots stay
    // `None` (=> `-`).
    let rows: Vec<StateRow> = row_map
        .into_iter()
        .map(|(key, next)| StateRow {
            inputs: input_nodes.iter().map(|c| key.value_of(c)).collect(),
            current: state_orig.iter().map(|n| key.value_of(n)).collect(),
            next,
        })
        .collect();

    Some(StateModel {
        input_nodes,
        internal_nodes,
        node_of,
        rows,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::logic::regions::StateRegions;
    use crate::model::{analyse_one as analyse, AnalysedOutput};
    use espresso_logic::bdd::{Bdd, BddBuilder, Brand, ManagerCell};
    use espresso_logic::bdd_builder;

    const T: Option<bool> = Some(true);
    const F: Option<bool> = Some(false);
    const X: Option<bool> = None;

    // Next-action slots: High / Low / hold (N) / unconstrained `-`.
    const HI: Option<Next> = Some(Next::High);
    const LO: Option<Next> = Some(Next::Low);
    const NO: Option<Next> = Some(Next::Hold);
    const DC: Option<Next> = None;

    fn names(v: &[Symbol]) -> Vec<&str> {
        v.iter().map(Symbol::as_str).collect()
    }

    fn row(inputs: &[Option<bool>], current: &[Option<bool>], next: &[Option<Next>]) -> StateRow {
        StateRow {
            inputs: inputs.to_vec(),
            current: current.to_vec(),
            next: next.to_vec(),
        }
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
        // One node, so every row carries a definite action (no deferral). On A*B, off !A*!B, hold A^B.
        assert_eq!(m.rows.len(), 4);
        assert!(m.rows.contains(&row(&[T, T], &[X], &[HI])));
        assert!(m.rows.contains(&row(&[F, F], &[X], &[LO])));
        assert!(m.rows.contains(&row(&[F, T], &[X], &[NO])));
        assert!(m.rows.contains(&row(&[T, F], &[X], &[NO])));
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
        // Per-output rows: Q rows are keyed by CLK/M (M slot deferred `-`); M rows keyed by CLK/D
        // (Q slot deferred `-`). Six rows in all.
        assert_eq!(m.rows.len(), 6);
        // Q: drives off the master's current, holds while transparent-low.
        assert!(m.rows.contains(&row(&[T, X], &[X, T], &[HI, DC])));
        assert!(m.rows.contains(&row(&[T, X], &[X, F], &[LO, DC])));
        assert!(m.rows.contains(&row(&[F, X], &[X, X], &[NO, DC])));
        // M: samples D while CLK low, holds while CLK high.
        assert!(m.rows.contains(&row(&[F, T], &[X, X], &[DC, HI])));
        assert!(m.rows.contains(&row(&[F, F], &[X, X], &[DC, LO])));
        assert!(m.rows.contains(&row(&[T, X], &[X, X], &[DC, NO])));
    }

    #[test]
    fn mut_joint_table_split_race_rows() {
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
        // The old joint race row `H H : L L : H H` is now two per-output rows: each grant drives high
        // off its own request and the other grant being currently low, the other slot deferred `-`.
        assert!(m.rows.contains(&row(&[T, X], &[X, F], &[HI, DC])));
        assert!(m.rows.contains(&row(&[X, T], &[F, X], &[DC, HI])));
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

    /// Rebuild a BDD from the emitted rows selecting one per-node action: OR of the selected rows, each
    /// the AND of its fixed input/current literals over the joint header (input nodes ++ state-signal
    /// original names). The reconstruct idiom mirrors `regions.rs`'s equivalence test.
    fn reconstruct_action<B: Brand, C: ManagerCell>(
        builder: &BddBuilder<B, C>,
        input_nodes: &[Symbol],
        state_orig: &[Symbol],
        rows: &[StateRow],
        node: usize,
        want: Next,
    ) -> Bdd<B, C> {
        let mut cover = builder.constant(false);
        for r in rows.iter().filter(|r| r.next[node] == Some(want)) {
            let mut product = builder.constant(true);
            for (col, val) in input_nodes.iter().zip(r.inputs.iter()) {
                match val {
                    Some(true) => product = product.and(&builder.var(col.as_str())),
                    Some(false) => product = product.and(&!builder.var(col.as_str())),
                    None => {}
                }
            }
            for (col, val) in state_orig.iter().zip(r.current.iter()) {
                match val {
                    Some(true) => product = product.and(&builder.var(col.as_str())),
                    Some(false) => product = product.and(&!builder.var(col.as_str())),
                    None => {}
                }
            }
            cover = cover.or(&product);
        }
        cover
    }

    /// The crux of the cover construction: for every state signal of every fixture, the BDD
    /// reconstructed from the emitted joint rows carrying that node's `H`/`L`/`N` action must be
    /// logically equivalent to that signal's reference on/off/hold region — proving the per-node
    /// next-state functions survive the joint multi-output minimisation and the `-`-deferred fold.
    #[test]
    fn emitted_rows_reconstruct_per_node_regions() {
        let cells = [
            r#"
[[cell]]
name = "C2"
inputs = ["A", "B"]
[cell.outputs]
Q = "A*B + Q*(A+B)"
"#,
            r#"
[[cell]]
name = "DFF"
inputs = ["CLK", "D"]
[cell.internal]
M = "!CLK*D + CLK*M"
[cell.outputs]
Q = "CLK*M + !CLK*Q"
"#,
            r#"
[[cell]]
name = "MUT"
inputs = ["A", "B"]
[cell.outputs]
Qa = "!Qb * A"
Qb = "!Qa * B"
"#,
            r#"
[[cell]]
name = "SR"
inputs = ["S", "R"]
[cell.outputs]
Q = "!(R + Qn)"
Qn = "!(S + Q)"
"#,
            r#"
[[cell]]
name = "GL"
inputs = ["C", "D"]
[cell.internal]
L = "!C*D + C*L"
[cell.outputs]
Y = "C*L"
"#,
        ];

        for src in cells {
            let cell = analyse(src);
            let m = build_state_model(&cell).expect("fixture is sequential");

            // Emitted rows carry unique (inputs, current) keys. `inputs`/`current` have a fixed width
            // per model, so their concatenation is an unambiguous key.
            let mut keys: BTreeSet<Vec<Option<bool>>> = BTreeSet::new();
            for r in &m.rows {
                let mut key = r.inputs.clone();
                key.extend(r.current.iter().copied());
                assert!(
                    keys.insert(key),
                    "duplicate (inputs, current) row key in {}",
                    cell.name
                );
            }

            // State signals in node order == the hysteretic signals in signals() order.
            let state: Vec<(&AnalysedOutput, &StateRegions)> = cell
                .signal_regions()
                .filter(|(_, sr)| sr.hysteretic)
                .collect();
            let state_orig: Vec<Symbol> = state.iter().map(|(sig, _)| sig.name.clone()).collect();

            for (i, (sig, _sr)) in state.iter().enumerate() {
                // Reference on/off/hold BDDs, built exactly as `state_regions` does, on one builder so
                // `equivalent_to` shares a manager with the reconstruction.
                let builder = bdd_builder!();
                let f = builder.build(&sig.expr);
                let self_state: Vec<&str> = if sig.feedback.contains(&sig.name) {
                    vec![sig.name.as_str()]
                } else {
                    vec![]
                };
                let on_bdd = f.forall(&self_state);
                let off_bdd = (!f.clone()).forall(&self_state);
                let hold_bdd = !on_bdd.or(&off_bdd);

                for (want, reference, label) in [
                    (Next::High, &on_bdd, "on"),
                    (Next::Low, &off_bdd, "off"),
                    (Next::Hold, &hold_bdd, "hold"),
                ] {
                    let got =
                        reconstruct_action(&builder, &m.input_nodes, &state_orig, &m.rows, i, want);
                    assert!(
                        got.equivalent_to(reference),
                        "{} region mismatch for {}.{}",
                        label,
                        cell.name,
                        sig.name
                    );
                }
            }
        }
    }
}
