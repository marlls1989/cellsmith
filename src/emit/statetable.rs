//! Unified joint state-table model for a sequential cell — the single source the Liberty renderer
//! and the behavioural Verilog draw from, built at **emission time** (not in the minimiser).
//!
//! A cell's hysteretic signals (its **state variables**: outputs and internal nodes on a dependency
//! cycle, as classified in [`crate::logic::regions`]) are folded into ONE joint next-state table. Each
//! [`StateRow`] pairs a primary-input pattern and a current-state pattern with a per-node next-state
//! action (`H`/`L`/`N` = drive-high/drive-low/hold, or `-` = unconstrained here). Every state variable,
//! whether output or internal, keeps its own name as its state-table node.
//!
//! CONSTRUCTION. The rows are built by **cover algebra**. Each node's three
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
//! - Every node is resolved to a port through a `direction : internal` pin's `internal_node` attribute.
//!   A state OUTPUT cannot share its name with its node (the port already holds it), so it mints one —
//!   `Q` -> `Q_st` — and reads it through a `state_function`; a genuine internal node and a derived
//!   register keep their own names.
//!
//! EDGE REGISTERS. When [`crate::logic::edge`] has recognised a node as an edge-triggered register, that
//! register's node keeps its column but its rows come from the annotation ([`EdgeRow`]) rather than the
//! level cover pass: a capture cube stamps the active edge token (`R`/`F`) with the register's next
//! action, an off-edge cube the hold/async action. A single-edge register prints the off-edge on its
//! inactive face (`~R`/`~F`); a dual-edge register (both edges capture) prints its off-edge with a
//! `Level` `-` token AFTER the two capture groups, so first-match priority keeps the captures winning at
//! the edges. Any folded master vanishes entirely — no node, no column, no rows. The clock sits in the
//! input header; the renderer prints the token there, e.g. `... R H : - : H` / `... ~R - : - : N`. A
//! register node is a state-table node even when its region is non-hysteretic (a combinational output
//! made sequential — the dual-edge mux-DET Q).

use std::collections::{BTreeMap, BTreeSet, HashMap};

use espresso_logic::{Anonymous, Cover, Minimizable, Minterm, Symbol};

use crate::logic::arcs::Edge;
use crate::logic::regions::{StateCube, StateRegions};
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

/// The clock-edge token an [`EdgeRow`] prints in its clock column: the active edge (`Rise`/`Fall`) of a
/// capture row, the inactive face (`NotRise`/`NotFall`) of a single-edge register's off-edge (hold /
/// async) row, or `Level` for a dual-edge register's off-edge row (which owns neither clock face).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum EdgeTok {
    /// Rising clock edge — Liberty `R`.
    Rise,
    /// Falling clock edge — Liberty `F`.
    Fall,
    /// The non-rising face of a rise register (its off-edge hold/async path) — Liberty `~R`.
    NotRise,
    /// The non-falling face of a fall register — Liberty `~F`.
    NotFall,
    /// A dual-edge register's off-edge (hold/async) row: neither clock face, printed as `-` in the clock
    /// column. Both edges capture, so Liberty first-match priority keeps the capture rows winning there.
    Level,
}

/// One edge-triggered row of the joint state table: an [`EdgeCaptures`](crate::logic::edge::EdgeCaptures)'s
/// capture or off-edge behaviour. `inputs` is aligned to [`StateModel::input_nodes`], `current`/`next` to
/// [`StateModel::internal_nodes`] — the same layout as [`StateRow`]. The register's `clock` sits in
/// `inputs` as a `None` placeholder; the renderer prints `token` in that column instead of a level. Every
/// next slot other than the register's own stays `None` (`-`, deferred), exactly as for the level rows.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct EdgeRow {
    pub clock: Symbol,
    pub token: EdgeTok,
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
    /// State-node columns (`current`/`next` order) under their TABLE-NODE names — what the statetable
    /// header prints. A state output's node is minted (`Q` -> `Q_st`); an internal state node and a
    /// derived register keep their own name. Folded masters are excluded; recognised edge-register nodes
    /// keep their column.
    pub internal_nodes: Vec<Symbol>,
    /// The same columns in the same order under their SIGNAL names — what the covers, the minterm keys
    /// and the machine are written in. `internal_nodes[i]` is the table node of `state_signals[i]`;
    /// anything querying the machine or a region by column must read this list, not `internal_nodes`.
    pub state_signals: Vec<Symbol>,
    /// Each state signal's ORIGINAL name → its state-table node name.
    pub node_of: BTreeMap<Symbol, Symbol>,
    /// The joint level (level-sensitive) next-state rows, deduplicated and sorted.
    pub rows: Vec<StateRow>,
    /// The edge-triggered rows contributed by the cell's recognised edge registers, after the level rows
    /// in register (`signals()`) order. Empty for a cell with no collapsed master-slave pair.
    pub edge_rows: Vec<EdgeRow>,
}

/// Build the joint state-table model of a cell, or `None` if the cell has no state variable (a purely
/// combinational cell emits `function:`, never a `statetable`).
///
/// State signals are the cell's hysteretic [`signal_regions`](AnalysedCell::signal_regions) entries —
/// identical to [`crate::logic::resolve::state_variables`] by construction, so this reads the cached
/// `hysteretic` flag rather than re-running the classifier.
pub fn build_state_model(cell: &AnalysedCell) -> Option<StateModel> {
    // Behavioural edge annotation: the cell's recognised edge registers and the cell-level set of
    // internal level-sensitive masters folded away (empty when the cell opted out). A folded master
    // vanishes entirely — no node, no column, no rows; an edge-register node keeps its column but its
    // rows come from the annotation in (e), never the level cover pass in (d).
    let edge_regs = &cell.edge.captures;
    let folded: BTreeSet<Symbol> = cell.edge.folded.iter().cloned().collect();
    let edge_nodes: BTreeSet<Symbol> = edge_regs.iter().map(|er| er.node.clone()).collect();
    // A register node is ALWAYS a state-table node, whether or not its region is hysteretic: a
    // combinational output made sequential (the dual-edge mux-DET Q) is still a register column.
    let is_node = |sig: &Symbol, sr: &StateRegions| {
        (sr.hysteretic || edge_nodes.contains(sig)) && !folded.contains(sig)
    };
    // Registers factored out of read-gated outputs (the read-gate factorisation) that are not themselves
    // declared signals are first-class internal nodes: their edge rows flow through the name-driven
    // machinery in (e) exactly like a declared register's, and their cols name primary inputs. A declared
    // register reused by the factorisation is already a signal, so it is not re-appended here.
    let signal_names: BTreeSet<Symbol> = cell
        .signal_regions()
        .map(|(sig, _)| sig.name.clone())
        .collect();
    let derived_nodes: Vec<Symbol> = cell
        .edge
        .derived
        .iter()
        .map(|d| d.name.clone())
        .filter(|n| !signal_names.contains(n))
        .collect();

    // (a) State signals = the hysteretic signals plus the (possibly non-hysteretic) edge-register nodes,
    // in `signals()` order, then the minted derived registers, minus any folded master (absorbed into its
    // register's capture, no column).
    let mut state_names: BTreeSet<Symbol> = cell
        .signal_regions()
        .filter(|(sig, sr)| is_node(&sig.name, sr))
        .map(|(sig, _)| sig.name.clone())
        .collect();
    state_names.extend(derived_nodes.iter().cloned());
    if state_names.is_empty() {
        return None;
    }

    // (b) node_of: every SURVIVING state node's state-table node name. A state OUTPUT cannot lend its own
    // name to the node — the output pin holds that name and reads the node through a `state_function`, so
    // node and pin must be distinguishable — and mints one instead (`Q` -> `Q_st`, escalating past any
    // real signal of that name; see [`crate::logic::mint_state_node`]). A genuine internal state node and
    // a derived register have no competing output pin, so each keeps its own name. Folded masters are
    // excluded.
    let output_names: BTreeSet<Symbol> = cell.outputs.iter().map(|o| o.name.clone()).collect();
    let mut taken: BTreeSet<String> = cell
        .inputs
        .iter()
        .map(|s| s.to_string())
        .chain(cell.outputs.iter().map(|o| o.name.to_string()))
        .chain(cell.internals.iter().map(|o| o.name.to_string()))
        .chain(derived_nodes.iter().map(|n| n.to_string()))
        .collect();
    // `internal_nodes` and `state_orig` run in lockstep, one entry per surviving state node: the former
    // carries the TABLE-NODE name (what the statetable header prints), the latter the SIGNAL name the
    // cover algebra and the edge annotations key by. The two coincide for everything but a state output,
    // which is exactly why they must be tracked separately.
    let mut node_of: BTreeMap<Symbol, Symbol> = BTreeMap::new();
    let mut internal_nodes: Vec<Symbol> = Vec::new();
    let mut state_orig: Vec<Symbol> = Vec::new();
    for (sig, sr) in cell.signal_regions() {
        if !is_node(&sig.name, sr) {
            continue;
        }
        let node = if output_names.contains(&sig.name) {
            let minted = crate::logic::mint_state_node(sig.name.as_str(), |n| taken.contains(n));
            taken.insert(minted.clone());
            Symbol::from(minted.as_str())
        } else {
            sig.name.clone()
        };
        node_of.insert(sig.name.clone(), node.clone());
        internal_nodes.push(node);
        state_orig.push(sig.name.clone());
    }
    // The minted derived registers, appended in `edge.derived` order: each keeps its own name as its node,
    // having no output pin to compete with.
    for n in &derived_nodes {
        node_of.insert(n.clone(), n.clone());
        internal_nodes.push(n.clone());
        state_orig.push(n.clone());
    }

    // (c) Column partition: a state-signal-named col is a current-value column (middle field); every
    // other col is an input-node column — I3 guarantees it is a primary input (minimise.rs invariant
    // I3), asserted below. Level (non-edge, non-folded) signals contribute their non-state cols; each
    // edge register additionally contributes its own non-state cols and its clock (which the level maths
    // never sees, the clock having been projected out of the cofactors). input_nodes = the union of
    // input-side cols, ordered by cell.inputs order.
    let mut input_cols: BTreeSet<Symbol> = BTreeSet::new();
    for (sig, sr) in cell.signal_regions().filter(|(_, sr)| sr.hysteretic) {
        if folded.contains(&sig.name) || edge_nodes.contains(&sig.name) {
            continue;
        }
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
    for er in edge_regs {
        for col in &er.cols {
            if !state_names.contains(col) {
                debug_assert!(
                    cell.inputs.contains(col),
                    "edge-register column {col} must be a primary input or a state node",
                );
                input_cols.insert(col.clone());
            }
        }
        for clock in er.clocks() {
            debug_assert!(
                cell.inputs.contains(clock),
                "edge-register clock {clock} must be a primary input",
            );
            input_cols.insert(clock.clone());
        }
    }
    let input_nodes: Vec<Symbol> = cell
        .inputs
        .iter()
        .filter(|i| input_cols.contains(*i))
        .cloned()
        .collect();

    // (d) Level rows via cover algebra (see the module doc), over the LEVEL signals only — the hysteretic
    // signals that are neither folded masters nor edge-register nodes. Shared header = the input nodes
    // followed by the surviving nodes' ORIGINAL names in node order; edge-register nodes keep their header
    // column (a level signal may reference one) even though they contribute no level row. `state_orig`
    // (built alongside `internal_nodes` above) doubles as the node-order name list for `current`/`next` —
    // it must be the SIGNAL names, since the covers and the minterm keys are written in those terms.
    let level: Vec<(Symbol, &StateRegions)> = cell
        .signal_regions()
        .filter(|(_, sr)| sr.hysteretic)
        .filter(|(sig, _)| !folded.contains(&sig.name) && !edge_nodes.contains(&sig.name))
        .map(|(sig, sr)| (sig.name.clone(), sr))
        .collect();
    let shared_header: Vec<Symbol> = input_nodes
        .iter()
        .cloned()
        .chain(state_orig.iter().cloned())
        .collect();
    // Column layout: original node name -> slot index in node order, built once and shared by the level
    // pass (BY NAME output stamping) and the edge-row pass (input/current column splitting).
    let cols_layout = ColumnLayout::new(&input_nodes, &state_orig);

    // Each pass stacks every level node's minimised region cover for one action into a single multi-output
    // F cover over the shared header, joint-minimises it, and folds each cube into `row_map`. A zero-cube
    // region cover (e.g. an empty hold set) contributes no column and is skipped — the fold reads outputs
    // BY NAME, so a missing column just means no cube asserts that node in that region.
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
        for (name, sr) in &level {
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
                .or_insert_with(|| vec![None; cols_layout.n_nodes]);
            for (out, asserted) in cube.outputs().vars().iter().zip(cube.outputs().iter()) {
                if !asserted {
                    continue;
                }
                let i = cols_layout.node_index[out];
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

    // Emit level rows in Minterm order (BTreeMap iteration). Each row reads its input and current levels
    // off the shared-header key BY NAME (an absent column reads `None` = `-`); unstamped next slots stay
    // `None` (=> `-`), including every edge-register node's slot — those are set only by (e).
    let rows: Vec<StateRow> = row_map
        .into_iter()
        .map(|(key, next)| StateRow {
            inputs: input_nodes.iter().map(|c| key.value_of(c)).collect(),
            current: state_orig.iter().map(|n| key.value_of(n)).collect(),
            next,
        })
        .collect();

    // (e) Edge rows from the register annotations, in `signals()` (register) order, cubes in cover order.
    // Each active edge (`captures`, Rise before Fall) contributes a capture group: its on-cubes drive the
    // register high at the active token, its off-cubes low. The off-edge follows: for a single-edge
    // register it fires at the inactive face (`NotRise`/`NotFall`); for a dual-edge register (both edges
    // capture) it carries a `Level` `-` clock column and is placed AFTER the capture groups, so Liberty
    // first-match priority keeps the captures winning at the edges. Its on/off cubes are the async
    // set/clear, its hold cube the quiescent no-change. Every next slot other than the register's own
    // stays `-`; a capture cube that references the register's own node stamps that node's current column.
    let mut edge_rows: Vec<EdgeRow> = Vec::new();
    for er in edge_regs {
        let reg = cols_layout.node_index[&er.node];
        let single = er.captures.len() == 1;
        let mut push =
            |clock: &Symbol, token: EdgeTok, action: Next, cube: &StateCube, cols: &[Symbol]| {
                edge_rows.push(cols_layout.edge_row(clock, token, reg, action, cube, cols));
            };
        for (clock, edge, capture) in &er.captures {
            let active = match edge {
                Edge::Rise => EdgeTok::Rise,
                Edge::Fall => EdgeTok::Fall,
            };
            for (action, cubes) in [(Next::High, &capture.on), (Next::Low, &capture.off)] {
                for cube in cubes {
                    push(clock, active, action, cube, &capture.cols);
                }
            }
        }
        // Off-edge token: the single-capture register's inactive face, or the `Level` `-` column for a
        // multi-capture register (two edges of one clock, or captures spread across clocks — either way
        // its capture groups already own every clock face the off-edge could name). The off-edge is
        // clock-independent; its row marks the register's clock column (the sole clock for a single-clock
        // register).
        let off_token = if single {
            match er.captures[0].1 {
                Edge::Rise => EdgeTok::NotRise,
                Edge::Fall => EdgeTok::NotFall,
            }
        } else {
            EdgeTok::Level
        };
        let off_clock = er.clocks();
        let off_clock = off_clock[0];
        let off = &er.off_edge;
        for (action, cubes) in [
            (Next::High, &off.on),
            (Next::Low, &off.off),
            (Next::Hold, &off.hold),
        ] {
            for cube in cubes {
                push(off_clock, off_token, action, cube, &off.cols);
            }
        }
    }

    Some(StateModel {
        input_nodes,
        internal_nodes,
        state_signals: state_orig,
        node_of,
        rows,
        edge_rows,
    })
}

/// Column layout shared by every [`EdgeRow`] built for one cell: each input/state node's original name
/// mapped to its slot index in the row vectors, plus the two widths, so `edge_row` doesn't rebuild these
/// loop-invariant maps per row.
struct ColumnLayout<'a> {
    input_index: HashMap<&'a Symbol, usize>,
    node_index: HashMap<&'a Symbol, usize>,
    n_inputs: usize,
    n_nodes: usize,
}

impl<'a> ColumnLayout<'a> {
    /// Index `inputs` and `nodes` by name into their slot positions.
    fn new(inputs: &'a [Symbol], nodes: &'a [Symbol]) -> Self {
        let input_index: HashMap<&'a Symbol, usize> =
            inputs.iter().enumerate().map(|(i, n)| (n, i)).collect();
        let node_index: HashMap<&'a Symbol, usize> =
            nodes.iter().enumerate().map(|(i, n)| (n, i)).collect();
        debug_assert_eq!(input_index.len(), inputs.len());
        debug_assert_eq!(node_index.len(), nodes.len());
        ColumnLayout {
            n_inputs: inputs.len(),
            n_nodes: nodes.len(),
            input_index,
            node_index,
        }
    }

    /// Assemble one [`EdgeRow`] from a region cube, splitting each set literal by name into an input
    /// column (aligned to `input_nodes` via `input_index`) or a current-state column (aligned to
    /// `internal_nodes` via `node_index`). A capture cofactor never references the register's clock, so
    /// that column keeps its `None` placeholder and the renderer prints the edge token there; an off-edge
    /// forcing cover MAY pin the clock (a phase-conditioned `CLK*R` clear), and that level lands in
    /// `inputs` so the renderer prints the clock literal instead. Only the register's own next slot
    /// carries `action`.
    fn edge_row(
        &self,
        clock: &Symbol,
        token: EdgeTok,
        reg: usize,
        action: Next,
        cube: &StateCube,
        cols: &[Symbol],
    ) -> EdgeRow {
        let mut inputs = vec![None; self.n_inputs];
        let mut current = vec![None; self.n_nodes];
        for (col, val) in cols.iter().zip(cube.iter()) {
            if val.is_none() {
                continue;
            }
            if let Some(&i) = self.input_index.get(&col) {
                inputs[i] = *val;
            } else if let Some(&i) = self.node_index.get(&col) {
                current[i] = *val;
            }
        }
        let mut next = vec![None; self.n_nodes];
        next[reg] = Some(action);
        EdgeRow {
            clock: clock.clone(),
            token,
            inputs,
            current,
            next,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::logic::regions::StateRegions;
    use crate::model::{analyse_one as analyse, AnalysedOutput};
    use espresso_logic::bdd::{Bdd, BddBuilder, Brand, ManagerCell};
    use espresso_logic::{bdd_builder, sync_bdd_builder};

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
        // MIGRATED two-latch coverage: a declared clock with collapse opted OUT keeps the master-slave
        // joint table (both Q and M as nodes, six per-output rows, no edge rows).
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
        let m = build_state_model(&cell).expect("DFF is sequential");
        // The output Q mints its node; the internal master M, having no output pin to compete with,
        // keeps its own name.
        assert_eq!(names(&m.internal_nodes), ["Q_st", "M"]);
        assert_eq!(names(&m.state_signals), ["Q", "M"]);
        assert_eq!(m.node_of[&Symbol::from("M")], "M");
        assert_eq!(m.node_of[&Symbol::from("Q")], "Q_st");
        assert!(m.edge_rows.is_empty());
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
    fn dff_collapses_to_edge_rows() {
        // Default collapse with a declared clock: the DFF becomes ONE rising-edge register Q that folds
        // M away. Only Q is a node; M is neither a node nor a header column, and there are no level rows.
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
        let m = build_state_model(&cell).expect("DFF is sequential");
        assert_eq!(names(&m.input_nodes), ["CLK", "D"]);
        assert_eq!(names(&m.internal_nodes), ["Q_st"]);
        assert!(!m.node_of.contains_key(&Symbol::from("M")));
        // No level rows survive; the behaviour is entirely edge rows.
        assert!(m.rows.is_empty());
        assert_eq!(m.edge_rows.len(), 3);
        // inputs align to [CLK, D] (clock slot `None`, renderer prints the token); current is `[Q]`.
        let rise_hi = EdgeRow {
            clock: Symbol::from("CLK"),
            token: EdgeTok::Rise,
            inputs: vec![X, T],
            current: vec![X],
            next: vec![HI],
        };
        let rise_lo = EdgeRow {
            clock: Symbol::from("CLK"),
            token: EdgeTok::Rise,
            inputs: vec![X, F],
            current: vec![X],
            next: vec![LO],
        };
        let hold = EdgeRow {
            clock: Symbol::from("CLK"),
            token: EdgeTok::NotRise,
            inputs: vec![X, X],
            current: vec![X],
            next: vec![NO],
        };
        assert!(m.edge_rows.contains(&rise_hi));
        assert!(m.edge_rows.contains(&rise_lo));
        assert!(m.edge_rows.contains(&hold));
    }

    #[test]
    fn bdet_read_gate_factorisation_statetable() {
        // BDET: the read-gate factorisation makes `Yst` a first-class state node with native dual-edge
        // rows; the DET masters L1/L2 fold away. The read-gated output `Y` is not a table node (it emits a
        // state_function), so it never appears in the state model.
        let cell = analyse(
            r#"
[[cell]]
name = "BDET"
inputs = ["CLK", "D", "A"]
clock = ["CLK"]
[cell.internal]
L1 = "!CLK*D + CLK*L1"
L2 = "CLK*D + !CLK*L2"
[cell.outputs]
Y = "!((CLK*L1 + !CLK*L2)*A)"
"#,
        );
        let m = build_state_model(&cell).expect("BDET is sequential");
        // Yst is the sole state node; the masters folded; Y is not a table node; A is not a table column.
        assert_eq!(names(&m.internal_nodes), ["Y_st"]);
        assert_eq!(names(&m.input_nodes), ["CLK", "D"]);
        assert!(!m.node_of.contains_key(&Symbol::from("L1")));
        assert!(!m.node_of.contains_key(&Symbol::from("Y")));
        // Native dual-edge rows: BOTH edges capture, delivering !D (D=L drives Yst high on each edge).
        assert!(m.edge_rows.iter().any(|r| r.token == EdgeTok::Rise));
        assert!(m.edge_rows.iter().any(|r| r.token == EdgeTok::Fall));
        let rise_hi = EdgeRow {
            clock: Symbol::from("CLK"),
            token: EdgeTok::Rise,
            inputs: vec![X, F],
            current: vec![X],
            next: vec![HI],
        };
        let fall_lo = EdgeRow {
            clock: Symbol::from("CLK"),
            token: EdgeTok::Fall,
            inputs: vec![X, T],
            current: vec![X],
            next: vec![LO],
        };
        assert!(
            m.edge_rows.contains(&rise_hi),
            "rise captures !D: {:?}",
            m.edge_rows
        );
        assert!(
            m.edge_rows.contains(&fall_lo),
            "fall captures !D: {:?}",
            m.edge_rows
        );
    }

    #[test]
    fn icm_collapses_to_four_edge_nodes() {
        // The ICM interlock: two three-latch synchronisers collapse to exactly the four shared-boundary
        // registers, folding the relays sela1/selb1 away. Both clock edges appear.
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
        let m = build_state_model(&cell).expect("ICM is sequential");
        // The surviving state nodes are EXACTLY the four shared-boundary registers (order follows the
        // post-minimise `signals()` order; assert the set).
        assert_eq!(
            m.internal_nodes.iter().cloned().collect::<BTreeSet<_>>(),
            ["sela2", "enA", "selb2", "enB"]
                .into_iter()
                .map(Symbol::from)
                .collect::<BTreeSet<_>>(),
        );
        for gone in ["sela1", "selb1"] {
            assert!(!m.node_of.contains_key(&Symbol::from(gone)));
        }
        // Both a rising (sela2/selb2) and a falling (enA/enB) capture token are present.
        assert!(m.edge_rows.iter().any(|r| r.token == EdgeTok::Rise));
        assert!(m.edge_rows.iter().any(|r| r.token == EdgeTok::Fall));
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
        // The joint race resolves into two per-output rows: each grant drives high off its own request
        // and the other grant being currently low, the other slot deferred `-`.
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
    fn minted_node_escalates_past_a_real_signal_of_that_name() {
        // Output Q would mint `Q_st`, but the cell already declares an input by that name. The mint
        // escalates to `Q_st2` rather than colliding with the input, and the input keeps its own column.
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
        // The colliding input is untouched: still a plain input column under its own name.
        assert!(names(&m.input_nodes).contains(&"Q_st"));
        // The signal side of the column is the OUTPUT, not the like-named input.
        assert_eq!(names(&m.state_signals), ["Q"]);
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
            // MIGRATED: the two-latch DFF stays in the level-row reconstruction with collapse opted out
            // (the reconstruction covers level rows only; the edge form is asserted separately).
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
                    cell.repr_name()
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
                        cell.repr_name(),
                        sig.name
                    );
                }
            }
        }
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

    /// Three shapes the behavioural classifier leaves fully level (no register, no fold) even under
    /// default (on) collapse: a single latch (no seam to sample), a gated latch (self-referencing
    /// transparent cofactor), and a two-latch DFF whose clock is never declared. MCDFF and EMDFF have
    /// their own fixtures instead: MCDFF's two-clock slave stays level
    /// (`mcdff_two_clock_pair_stays_level`), while EMDFF's slave is recognised as an edge register
    /// (`emdff_recognises_slave_over_surviving_master`).
    const NON_COLLAPSIBLE: [&str; 3] = [
        // Single latch: Q has no master seam, so no clock edge captures it.
        r#"
[[cell]]
name = "DLAT"
inputs = ["CLK", "D"]
clock = ["CLK"]
[cell.outputs]
Q = "CLK*D + !CLK*Q"
"#,
        // Gated latch: D is transparent to Q while CLK high -- level, never a register.
        r#"
[[cell]]
name = "GLAT"
inputs = ["CLK", "D"]
clock = ["CLK"]
[cell.outputs]
Q = "CLK*(D+Q) + !CLK*Q"
"#,
        // Undeclared-clock DFF: the two-latch shape, but CLK is never declared a clock, so no edge.
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
    fn non_collapsible_suite_edge_rows_empty_with_and_without_the_flag() {
        for src in NON_COLLAPSIBLE {
            let (default, forced) = analyse_both(src);
            assert!(
                default.edge.captures.is_empty(),
                "unexpected edge register recognised in {}",
                default.repr_name()
            );
            let m_default = build_state_model(&default).expect("fixture is sequential");
            let m_forced = build_state_model(&forced).expect("fixture is sequential");
            assert!(m_default.edge_rows.is_empty());
            assert!(m_forced.edge_rows.is_empty());
            // Byte-identical joint model whether the flag is left off (default collapse, no-op here)
            // or forced on: same nodes, same rows.
            assert_eq!(
                format!("{:?}", m_default.internal_nodes),
                format!("{:?}", m_forced.internal_nodes),
            );
            assert_eq!(
                format!("{:?}", m_default.rows),
                format!("{:?}", m_forced.rows)
            );
        }
    }

    #[test]
    fn dff_opt_out_restores_level_rows_via_either_switch() {
        // The two-latch DFF, opted out directly (`no_edge_collapse = true` in the TOML) versus opted
        // out via the CLI-flag-equivalent blanket mutation over the whole spec: both switches restore
        // the SAME level (non-edge) joint table -- Q and M both nodes, six rows, no edge rows.
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

        for cell in [&direct, &via_flag] {
            assert!(cell.edge.captures.is_empty());
            let m = build_state_model(cell).expect("DFF is sequential");
            assert!(
                m.edge_rows.is_empty(),
                "level rows must return, not edge rows"
            );
            assert_eq!(names(&m.internal_nodes), ["Q_st", "M"]);
            assert_eq!(m.rows.len(), 6);
        }
        // Both switches produce byte-identical joint models.
        let m_direct = build_state_model(&direct).unwrap();
        let m_via_flag = build_state_model(&via_flag).unwrap();
        assert_eq!(
            format!("{:?}", m_direct.rows),
            format!("{:?}", m_via_flag.rows),
        );
    }

    // Exposed-master DFF: M is a declared output (never foldable). The behavioural classifier recognises
    // the slave Q as a rising-edge register keyed off CLK, over the surviving master M, from its observed
    // toggle-and-settle behaviour.
    const EMDFF: &str = r#"
[[cell]]
name = "EMDFF"
inputs = ["CLK", "D"]
clock = ["CLK"]
[cell.outputs]
Q = "CLK*M + !CLK*Q"
M = "!CLK*D + CLK*M"
"#;

    #[test]
    fn emdff_recognises_slave_over_surviving_master() {
        // Q is an edge register keyed off CLK; the master M -- a surviving declared output -- keeps its
        // own level column and is never folded. Q contributes edge rows, M the level rows.
        let cell = analyse(EMDFF);
        let q = cell
            .edge
            .captures
            .iter()
            .find(|r| r.node == "Q")
            .expect("Q is recognised as an edge register");
        assert_eq!(
            q.clocks()
                .into_iter()
                .map(Symbol::as_str)
                .collect::<Vec<_>>(),
            ["CLK"]
        );
        assert!(
            !cell.edge.folded.iter().any(|f| f == "M"),
            "M survives as a level node, never folded"
        );
        let m = build_state_model(&cell).expect("sequential");
        assert!(
            m.node_of.contains_key(&Symbol::from("M")),
            "M is a surviving level column"
        );
        assert!(names(&m.internal_nodes).contains(&"M_st"));
        assert!(!m.edge_rows.is_empty(), "Q contributes edge rows");
        assert!(!m.rows.is_empty(), "M contributes level rows");
    }

    // Master/slave pair split across two DIFFERENT declared clocks: M latches on CLKA, Q on CLKB. Q tracks
    // live data through CLKB's transparent phase, so its seam set empties under the fixpoint: the
    // classifier recognises NO register and Q stays fully level.
    const MCDFF: &str = r#"
[[cell]]
name = "MCDFF"
inputs = ["CLKA", "CLKB", "D"]
clock = ["CLKA", "CLKB"]
[cell.internal]
M = "!CLKA*D + CLKA*M"
[cell.outputs]
Q = "CLKB*M + !CLKB*Q"
"#;

    #[test]
    fn mcdff_two_clock_pair_stays_level() {
        let cell = analyse(MCDFF);
        assert!(
            cell.edge.captures.is_empty(),
            "a two-clock pair keys off no single clock: {:?}",
            cell.edge
                .captures
                .iter()
                .map(|r| r.node.as_str())
                .collect::<Vec<_>>()
        );
        let m = build_state_model(&cell).expect("sequential");
        assert!(m.edge_rows.is_empty(), "no edge rows: stays level");
        // Both latches keep their own level columns.
        assert_eq!(names(&m.internal_nodes), ["Q_st", "M"]);
    }

    // Dual-edge mux-DET: two transparent-opposite latches feed a mux; Q captures D on BOTH clock edges and
    // L1/L2 fold away. Q is a combinational output made sequential (its region is non-hysteretic).
    const DET: &str = r#"
[[cell]]
name = "DET"
inputs = ["CLK", "D"]
clock = ["CLK"]
[cell.internal]
L1 = "!CLK*D + CLK*L1"
L2 = "CLK*D + !CLK*L2"
[cell.outputs]
Q = "CLK*L1 + !CLK*L2"
"#;

    #[test]
    fn det_dual_edge_emits_both_tokens_then_level_hold() {
        let cell = analyse(DET);
        let m = build_state_model(&cell).expect("DET is sequential");
        // Q is a state-table node despite its non-hysteretic (combinational-output) region; L1/L2 fold.
        assert_eq!(names(&m.internal_nodes), ["Q_st"]);
        assert!(
            m.rows.is_empty(),
            "no level rows: L1/L2 folded, Q is the register"
        );

        let cap = |token, d, next| EdgeRow {
            clock: Symbol::from("CLK"),
            token,
            inputs: vec![X, d],
            current: vec![X],
            next: vec![next],
        };
        // Both clock faces capture D (Rise group then Fall group).
        assert!(m.edge_rows.contains(&cap(EdgeTok::Rise, T, HI)));
        assert!(m.edge_rows.contains(&cap(EdgeTok::Rise, F, LO)));
        assert!(m.edge_rows.contains(&cap(EdgeTok::Fall, T, HI)));
        assert!(m.edge_rows.contains(&cap(EdgeTok::Fall, F, LO)));
        // Between edges the register holds: a Level (`-` clock column) off-edge row.
        assert!(m.edge_rows.contains(&cap(EdgeTok::Level, None, NO)));
        // The Level off-edge rows land AFTER every capture row (Liberty first-match priority).
        let first_level = m
            .edge_rows
            .iter()
            .position(|r| r.token == EdgeTok::Level)
            .expect("a Level off-edge row");
        let last_cap = m
            .edge_rows
            .iter()
            .rposition(|r| matches!(r.token, EdgeTok::Rise | EdgeTok::Fall))
            .expect("capture rows");
        assert!(
            last_cap < first_level,
            "captures precede the Level off-edge"
        );
    }

    // Inverting DFF: the slave captures !M (=!D) on the rising edge -- inversion recorded verbatim.
    const INVERTING_DFF: &str = r#"
[[cell]]
name = "IDFF"
inputs = ["CLK", "D"]
clock = ["CLK"]
[cell.internal]
M = "!CLK*D + CLK*M"
[cell.outputs]
Q = "CLK*!M + !CLK*Q"
"#;

    #[test]
    fn inverting_dff_emits_not_d_capture_rows() {
        let cell = analyse(INVERTING_DFF);
        let m = build_state_model(&cell).expect("IDFF is sequential");
        assert_eq!(names(&m.internal_nodes), ["Q_st"]);
        assert!(m.rows.is_empty());
        // Rising capture is !D: D low drives Q high, D high drives Q low.
        let rise = |d, next| EdgeRow {
            clock: Symbol::from("CLK"),
            token: EdgeTok::Rise,
            inputs: vec![X, d],
            current: vec![X],
            next: vec![next],
        };
        assert!(m.edge_rows.contains(&rise(F, HI)));
        assert!(m.edge_rows.contains(&rise(T, LO)));
        // Single-edge register: the off-edge holds on the inactive (~R) face, never a Level row.
        assert!(m.edge_rows.contains(&EdgeRow {
            clock: Symbol::from("CLK"),
            token: EdgeTok::NotRise,
            inputs: vec![X, X],
            current: vec![X],
            next: vec![NO],
        }));
        assert!(!m.edge_rows.iter().any(|r| r.token == EdgeTok::Level));
    }

    // Toggle flop: the self-fed master M and slave Q form a ring. With an async reset resolving the state,
    // it decomposes into TWO edge registers -- Q captures M on the rising edge, M captures !Q (=!M) on the
    // falling edge -- neither folds. M's capture references its OWN node (a current-state self column).
    const TOGGLE_FLOP: &str = r#"
[[cell]]
name = "TFF"
inputs = ["CLK", "R"]
clock = ["CLK"]
async = ["R"]
[cell.internal]
M = "!R*(!CLK*!Q + CLK*M)"
[cell.outputs]
Q = "!R*(CLK*M + !CLK*Q)"
"#;

    #[test]
    fn toggle_flop_capture_stamps_own_current_column() {
        let cell = analyse(TOGGLE_FLOP);
        let m = build_state_model(&cell).expect("TFF is sequential");
        // Two edge registers survive; the ring does NOT fold the self-fed master.
        assert_eq!(names(&m.internal_nodes), ["Q_st", "M"]);
        let qi = index_of_node(&m, "Q");
        let mi = index_of_node(&m, "M");
        // Both captures are the toggle ring `!R*!Q` over cols [R, Q]: they drive their node's next off Q's
        // OWN current column (the master M is dropped from the cover in favour of the output Q). Q's rising
        // capture is thus the self-column path — a capture cube carrying the register's own node.
        assert!(
            m.edge_rows.iter().any(|r| r.token == EdgeTok::Rise
                && r.next[qi].is_some()
                && r.current[qi].is_some()),
            "Q's rising capture stamps its OWN current column (the toggle ring)"
        );
        // M's falling-edge capture closes the same ring on Q's current column.
        assert!(
            m.edge_rows.iter().any(|r| r.token == EdgeTok::Fall
                && r.next[mi].is_some()
                && r.current[qi].is_some()),
            "toggle-flop M capture closes the ring on Q's current column"
        );
    }

    // DCMUX: a genuinely independent two-clock capture. Q captures each independently-clocked master at
    // that clock's own edge, holding otherwise.
    const DCMUX: &str = r#"
[[cell]]
name = "DCMUX"
inputs = ["CLKA", "CLKB", "DA", "DB"]
clock = ["CLKA", "CLKB"]
[cell.internal]
MA = "!CLKA*DA + CLKA*MA"
MB = "!CLKB*DB + CLKB*MB"
[cell.outputs]
Q = "CLKA*MA + CLKB*MB + !CLKA*!CLKB*Q"
"#;

    #[test]
    fn dcmux_is_a_level_model_no_edge_rows() {
        // DCMUX collapses to a LEVEL model: its falls are combinational and the seam fixpoint empties Q's
        // set, so nothing contributes an edge capture row -- the whole cell renders as level rows. The two
        // rises stay `-type edge` DELAY arcs (covered in the arcs_tcl emitter tests).
        let cell = analyse(DCMUX);
        let m = build_state_model(&cell).expect("DCMUX has level state (MA/MB/Q)");
        assert!(
            !m.edge_rows
                .iter()
                .any(|r| matches!(r.token, EdgeTok::Rise | EdgeTok::Fall)),
            "a level model carries no edge capture rows"
        );
    }

    // Hierarchical master-slave across two clocks (HPIPE): a CLKA rising-edge master pair feeds a CLKB
    // slave latch on Q. The slave keeps BOTH clocks' arcs -- CLKA:Rise and CLKB:Fall on the same node.
    const HPIPE: &str = r#"
[[cell]]
name = "HPIPE"
inputs = ["CLKA", "CLKB", "D"]
clock = ["CLKA", "CLKB"]
[cell.internal]
M1 = "!CLKA*D + CLKA*M1"
M2 = "CLKA*M1 + !CLKA*M2"
[cell.outputs]
Q = "!CLKB*M2 + CLKB*Q"
"#;

    #[test]
    fn hierarchical_slave_keeps_both_clock_edge_rows() {
        let cell = analyse(HPIPE);
        let m = build_state_model(&cell).expect("HPIPE is sequential");
        let qi = index_of_node(&m, "Q");
        // The slave Q's own next-slot is stamped by BOTH the conditioned CLKA:Rise capture AND its own
        // CLKB:Fall opening (the master-slave reveal): the CLKB reveal is a first-class capture row, so the
        // emitted statetable carries the CLKB-fall rows directly.
        assert!(
            m.edge_rows
                .iter()
                .any(|r| r.clock == "CLKA" && r.token == EdgeTok::Rise && r.next[qi].is_some()),
            "Q carries a CLKA:Rise capture row"
        );
        assert!(
            m.edge_rows
                .iter()
                .any(|r| r.clock == "CLKB" && r.token == EdgeTok::Fall && r.next[qi].is_some()),
            "Q carries a CLKB:Fall capture row (its own latch opening)"
        );
    }

    /// The node-order slot of a state SIGNAL in the joint model (`current`/`next` index). Keyed by signal
    /// name, not table node, since callers name the cell's signals.
    fn index_of_node(m: &StateModel, name: &str) -> usize {
        m.state_signals
            .iter()
            .position(|n| n == name)
            .unwrap_or_else(|| panic!("{name} is a state node"))
    }

    // A rising-edge DFF with a MASTER-ONLY (phase-conditioned) clear: R clears only the master M, so Q
    // clears only while the slave is transparent (CLK high). The clear cover is `CLK*R`, the row that the
    // pre-fix renderer corrupted to `~R - H : - : L`.
    const MOR: &str = r#"
[[cell]]
name = "MOR"
inputs = ["CLK", "D", "R"]
clock = ["CLK"]
[cell.internal]
M = "!R*(!CLK*D + CLK*M)"
[cell.outputs]
Q = "CLK*M + !CLK*Q"
"#;

    // The same master-only clear with R declared async — identical statetable, distinct code path.
    const MORA: &str = r#"
[[cell]]
name = "MORA"
inputs = ["CLK", "D", "R"]
clock = ["CLK"]
async = ["R"]
[cell.internal]
M = "!R*(!CLK*D + CLK*M)"
[cell.outputs]
Q = "CLK*M + !CLK*Q"
"#;

    // A cross-coupled set/reset latch: a two-node LEVEL model whose settle takes several steps, exercising
    // deferral (`-`) and multi-step first-match jointly.
    const SR_FLOP: &str = r#"
[[cell]]
name = "SR"
inputs = ["S", "R"]
[cell.outputs]
Q = "!(R + Qn)"
Qn = "!(S + Q)"
"#;

    // The canonical single rising-edge DFF (master folds; Q is the register).
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

    // A rising-edge DFF with a FULL-async clear (R clears Q directly) declared alongside CLK as a clock.
    // R is level-acting on Q, so its clear is the free-clock `~R - H` row that stays correct under the fix.
    const RDFF: &str = r#"
[[cell]]
name = "RDFF"
inputs = ["CLK", "D", "R"]
clock = ["CLK", "R"]
[cell.internal]
M = "!R*(!CLK*D + CLK*M)"
[cell.outputs]
Q = "!R*(CLK*M + !CLK*Q)"
"#;

    /// One rendered statetable row as its emitted token strings: `H`/`L`/`-` levels, the clock-edge tokens
    /// `R`/`F`/`~R`/`~F`, and `N` (hold) in the next field.
    struct RenderedRow {
        inputs: Vec<String>,
        current: Vec<String>,
        next: Vec<String>,
    }

    /// Parse the `statetable ("<inputs>", "<nodes>") { table : "..."; }` block out of a rendered cell: the
    /// two header node lists and the rows, each field split into its emitted tokens. Reads the RENDERED
    /// output (not the model), so it is the one place `edge_input_pattern`'s clock-column rendering is
    /// exercised behaviourally — a dropped clock literal reaches the replay through here.
    fn parse_rendered_statetable(liberty: &str) -> (Vec<String>, Vec<String>, Vec<RenderedRow>) {
        let start = liberty
            .find("statetable (")
            .expect("fixture renders a statetable");
        let block = &liberty[start..];
        let brace = block.find('{').expect("statetable header brace");
        // The header carries exactly two quoted fields: the input nodes then the state nodes.
        let quoted: Vec<&str> = block[..brace].split('"').collect();
        let words = |f: &str| {
            f.split_whitespace()
                .map(str::to_owned)
                .collect::<Vec<String>>()
        };
        let input_names = words(quoted[1]);
        let state_names = words(quoted[3]);

        // Search past the header brace so the `table :` attribute is not confused with `statetable`.
        let table_at = brace + block[brace..].find("table").expect("a table attribute");
        let rest = &block[table_at..];
        let q1 = rest.find('"').expect("table opening quote");
        let q2 = rest[q1 + 1..].find('"').expect("table closing quote") + q1 + 1;
        // Rows are comma-separated, fields colon-separated; drop the line-continuation backslashes first.
        let content = rest[q1 + 1..q2].replace('\\', " ");
        let rows = content
            .split(',')
            .map(|row| {
                let fields: Vec<&str> = row.split(':').collect();
                assert_eq!(fields.len(), 3, "malformed statetable row {row:?}");
                RenderedRow {
                    inputs: words(fields[0]),
                    current: words(fields[1]),
                    next: words(fields[2]),
                }
            })
            .collect();
        (input_names, state_names, rows)
    }

    /// Does a rendered row match one machine event `(cur, toggled input, destination)`? Input columns
    /// match the destination's steady input level (`H`/`L`) or the toggled clock's edge (`R`/`F` fire only
    /// for the toggled clock on this step; `~R`/`~F` match every event that is NOT that rising/falling
    /// edge); current-state columns match the evolving node vector `cur`. `-` matches anything — a level
    /// don't-care, or a dual-edge off-edge's free clock column.
    fn row_matches(
        input_names: &[String],
        row: &RenderedRow,
        cur: &[bool],
        event: Option<&str>,
        dest: &Minterm<Symbol>,
    ) -> bool {
        for (name, tok) in input_names.iter().zip(&row.inputs) {
            let toggled_here = event == Some(name.as_str());
            let rose = toggled_here && dest.value_of(name.as_str()) == Some(true);
            let fell = toggled_here && dest.value_of(name.as_str()) == Some(false);
            let ok = match tok.as_str() {
                "-" => true,
                "H" => dest.value_of(name.as_str()) == Some(true),
                "L" => dest.value_of(name.as_str()) == Some(false),
                "R" => rose,
                "F" => fell,
                "~R" => !rose,
                "~F" => !fell,
                other => panic!("unknown input token {other:?}"),
            };
            if !ok {
                return false;
            }
        }
        for (idx, tok) in row.current.iter().enumerate() {
            let ok = match tok.as_str() {
                "-" => true,
                "H" => cur[idx],
                "L" => !cur[idx],
                other => panic!("unknown current token {other:?}"),
            };
            if !ok {
                return false;
            }
        }
        true
    }

    /// The rendered rows' first-match next value for one node under a single event: scan rows in emission
    /// order (level rows then edge rows), skip any that defer this node (`-`) or fail to match, and take
    /// the first definite action. No matching row leaves the node held.
    fn predict_node(
        input_names: &[String],
        rows: &[RenderedRow],
        node: usize,
        cur: &[bool],
        event: Option<&str>,
        dest: &Minterm<Symbol>,
    ) -> bool {
        for row in rows {
            match row.next[node].as_str() {
                "-" => continue, // deferred to a lower-priority row per Liberty per-output resolution
                action if row_matches(input_names, row, cur, event, dest) => {
                    return match action {
                        "H" => true,
                        "L" => false,
                        "N" => cur[node],
                        other => panic!("unknown next token {other:?}"),
                    };
                }
                _ => {}
            }
        }
        cur[node] // no row decides this node: it holds
    }

    /// Settle the rendered statetable as a next-state machine: apply the joint first-match next-state, with
    /// the toggled input's clock edge live only on the first step (later steps are quiescent settling), to
    /// a fixpoint. Mirrors the async cell's own `settle`, so the fixpoint is comparable to the machine's
    /// settled state; an oscillation where the machine settled is a faithfulness failure.
    fn settle_rendered(
        input_names: &[String],
        rows: &[RenderedRow],
        cur0: Vec<bool>,
        toggled: &str,
        dest: &Minterm<Symbol>,
    ) -> Vec<bool> {
        let mut cur = cur0;
        let mut event = Some(toggled);
        let mut seen: BTreeSet<Vec<bool>> = BTreeSet::new();
        loop {
            let next: Vec<bool> = (0..cur.len())
                .map(|i| predict_node(input_names, rows, i, &cur, event, dest))
                .collect();
            if next == cur {
                return cur;
            }
            assert!(
                seen.insert(next.clone()),
                "rendered statetable oscillates where the machine settled"
            );
            cur = next;
            event = None; // the clock edge is a one-time event; later settle steps are quiescent
        }
    }

    /// Replay the RENDERED statetable of one fixture against its machine: for every reachable-stable
    /// transition, settle the rendered rows (level and edge, jointly, under Liberty first-match) and check
    /// the settled node values against the machine's own settled state. This is the joint edge+level
    /// coverage `emitted_rows_reconstruct_per_node_regions` lacks — the only test that fails when
    /// `edge_input_pattern` drops a clock literal (the MOR/MORA `~R - H` clear-on-any-non-rising bug).
    fn replay_rendered_statetable(src: &str) {
        let cell = analyse(src);
        // The rendered rows are the device under test — the sole path through `edge_input_pattern`.
        let liberty = crate::emit::liberty::cell_liberty(&cell);
        let (input_names, state_names, rows) = parse_rendered_statetable(&liberty);
        let model = build_state_model(&cell).expect("fixture is sequential");
        assert_eq!(
            input_names.iter().map(String::as_str).collect::<Vec<_>>(),
            names(&model.input_nodes),
        );
        assert_eq!(
            state_names.iter().map(String::as_str).collect::<Vec<_>>(),
            names(&model.internal_nodes),
        );
        // The rendered header carries TABLE-NODE names; the machine below answers only to SIGNAL names,
        // and the two differ for every state output. Column i of the rendered rows is `state_sigs[i]`.
        let state_sigs = names(&model.state_signals);

        // Rebuild the machine from the folded cell to read its settled reference values. `build_signal_bdds`
        // is pure over the folded `expr`s, so this reproduces the machine `analyse` explored.
        let builder = sync_bdd_builder!();
        let bdds = crate::model::build_signal_bdds(&cell, &builder);
        let machine = crate::logic::analysis::Machine::build(
            &cell,
            &bdds,
            crate::logic::analysis::Exploration::Fresh(
                &crate::logic::machine::ExplorationBudget::default(),
            ),
        )
        .expect("fixture is explored");

        // Per-node async-forcing covers (the off-edge set/clear), used below to detect a node whose async
        // force lapses between the start and destination state. A node with no edge-register entry (a
        // pure level node) has no forcing and never triggers that skip.
        let forcing: BTreeMap<&str, _> = cell
            .edge
            .captures
            .iter()
            .map(|er| {
                (
                    er.node.as_str(),
                    (
                        builder.build_cover(&er.off_edge.on_cover),
                        builder.build_cover(&er.off_edge.off_cover),
                    ),
                )
            })
            .collect();
        // `forced(node, s)` = the constant the async set/clear clamps `node` to at `s`, or `None` when
        // nothing forces it there.
        let forced = |node: &str, s: &Minterm<Symbol>| -> Option<bool> {
            forcing.get(node).and_then(|(on, off)| {
                if on.evaluate_fast(s) == Some(true) {
                    Some(true)
                } else if off.evaluate_fast(s) == Some(true) {
                    Some(false)
                } else {
                    None
                }
            })
        };

        // Eligible = every STATE column determinate; an uninitialised start is traversal-only, per the
        // measurement-eligibility filter, and a partial destination is likewise not a measurement context.
        let eligible = |s: &Minterm<Symbol>| {
            machine
                .state_vars
                .iter()
                .all(|w| s.value_of(w.as_str()).is_some())
        };

        // Both coordinate halves, stepped together: a narrower settle would leave a combinational
        // coordinate's column at its stale pre-toggle value, so the replayed destination would not
        // match the state the rendered rows are checked against.
        let deltas = machine.coordinate_deltas();
        for s in &machine.explored.order {
            if !eligible(s) {
                continue;
            }
            for x in &cell.inputs {
                let toggled = crate::logic::machine::toggle(s, &[x.as_str()]);
                let Some(dest) = crate::logic::machine::settle(&deltas, &toggled) else {
                    continue; // the toggle oscillates ⇒ not a measurable transition
                };
                if !eligible(&dest) {
                    continue;
                }
                let cur0: Vec<bool> = state_sigs
                    .iter()
                    .map(|n| {
                        machine
                            .output_value(n, s)
                            .expect("eligible start is determinate")
                    })
                    .collect();
                let got = settle_rendered(&input_names, &rows, cur0, x.as_str(), &dest);
                for (i, node) in state_sigs.iter().enumerate() {
                    // When the async set/clear forcing `node` at the start state stops forcing it by the
                    // destination, the node re-acquires its value through level transparency the edge model
                    // abstracts away (TFF's master re-tracking `!Q` when R de-asserts at CLK=0). The
                    // established replay harness (`assert_captures_faithful` clause 4) checks only
                    // determinism there, never the exact value, and the statetable is deterministic by
                    // construction — so skip the exact check when the force lapses.
                    if forced(node, s).is_some() && forced(node, &dest).is_none() {
                        continue;
                    }
                    let want = machine.output_value(node, &dest);
                    assert_eq!(
                        Some(got[i]),
                        want,
                        "replay unfaithful: {} node {node} from {s:?} toggling {x} settled to {dest:?}: \
                         rendered rows predict {:?} != machine {want:?}",
                        cell.repr_name(),
                        got[i],
                    );
                }
            }
        }
    }

    /// The rendered-row replay over the joint level+edge first-match semantics, for the fixtures that
    /// carry the phase-conditioned clear (MOR/MORA), a multi-step two-node level model (SR), a dual-edge
    /// register (DET), a two-seam toggle register (TFF), the canonical DFF, and a full-async clear DFF
    /// (RDFF). Fails against an `edge_input_pattern` that drops the `CLK*R` clock literal (which would make
    /// MOR/MORA clear on any non-rising event with R high, including CLK=0 where the cell holds).
    #[test]
    fn rendered_rows_replay_machine_settled_values() {
        for src in [MOR, MORA, SR_FLOP, TOGGLE_FLOP, DET, DFF, RDFF] {
            replay_rendered_statetable(src);
        }
    }
}
