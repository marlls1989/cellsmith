//! The shared **state-machine pass**: build a cell's asynchronous state machine once and derive both
//! its transition arcs ([`super::arcs`]) and its confluence hazards ([`super::confluence`]) from the
//! same exploration.
//!
//! A cell is a state machine over `inputs × state-variables` (see [`machine`] and [`resolve`]). The
//! signals' BDDs are built and minimised once in [`crate::model::Cell::analyse`]; this pass reads that
//! shared map. After the fold every state variable's next-state δ **is** its entry in the map — a direct
//! lookup, no per-signal composition — and the combinational outputs' δ likewise. Only the one
//! [`machine::explore`] BFS is set up here, and it is the same setup for both derivations, so it is done
//! **once** and shared through [`Machine`]. Only plain data ([`Arc`]; the detected [`OrderDependence`]
//! and [`Oscillation`] hazards; the generated [`Constraint`]s) escapes into [`MachineAnalysis`]; the live
//! BDD handles never leave this pass.
//!
//! The BDD brand is a **generic type parameter** `<B, C>` carried by [`Machine`]: the builder is minted
//! once per cell in [`crate::model::Cell::analyse`] (a fresh brand each cell, so handles from two cells
//! cannot be mixed) and the shared map is threaded into this pass, the minimisation and the regions cache.
//!
//! The machine model and the two-stage detect/constrain hazard pipeline are described concept-first in
//! `state-machine-arc-engine.md` and `hazard-detection.md`; this module only wires the shared pass.

use std::collections::{BTreeMap, BTreeSet};

use espresso_logic::bdd::{Bdd, Brand, ManagerCell};
use espresso_logic::{Minterm, Symbol};

use crate::logic::arcs::{self, Arc, HiddenArc};
use crate::logic::confluence::{self, Constraint};
use crate::logic::hazard::{OrderDependence, Oscillation};
use crate::logic::leakage::{self, LeakageState};
use crate::logic::{machine, resolve};
use crate::model::AnalysedCell;

/// The plain-data outcome of the shared machine pass: the transition arcs, the two detected hazards
/// (order-dependent and oscillation), and the constraints generated to avoid them. Empty when the cell
/// is not explored (the combinatorial blow-up guard, see [`MAX_MACHINE_VARS`]).
///
/// `MachineAnalysis` itself never escapes this module: [`analyse_machine`]'s result is copied field-for-
/// field into the matching [`crate::model::AnalysedCell`] fields by `Cell::analyse` (see `model.rs`).
#[derive(Debug, Default)]
pub struct MachineAnalysis {
    pub arcs: Vec<Arc>,
    pub hidden_arcs: Vec<HiddenArc>,
    pub constraints: Vec<Constraint>,
    pub order_dependence: Vec<OrderDependence>,
    pub oscillation: Vec<Oscillation>,
    pub leakage: Vec<LeakageState>,
}

/// The single home for the combinatorial blow-up guard: a cell whose machine width (inputs + state
/// variables) exceeds this bound is not explored at all — both arcs and hazards come back empty.
///
/// The bound is on `inputs + state variables`, and 22 is a deliberate memory/time ceiling: exploration
/// materialises candidate pools by expanding the signals' input-projected FR covers (`cover_over_fr`)
/// into full input minterms (via [`Cover::maximize`](espresso_logic::Cover::maximize)), so a machine of
/// width `w` can seed on the order of `2^w` minterms. At 22 that worst case is ~4M candidates — the
/// largest pool we accept — and each extra variable *doubles* it, so raising the constant grows the pool
/// (and the exploration cost) exponentially.
pub(crate) const MAX_MACHINE_VARS: usize = 22;

/// A cell's asynchronous state machine, built once and shared by the arc and confluence derivations. The
/// BDD brand is a generic parameter scoped to the builder that minted these handles.
pub struct Machine<'c, B: Brand, C: ManagerCell> {
    pub(crate) cell: &'c AnalysedCell,
    /// State variables in signal order (outputs first, then internals).
    pub(crate) state_vars: Vec<Symbol>,
    /// The same state variables as a set, for membership tests.
    pub(crate) state_set: BTreeSet<Symbol>,
    /// Each state variable's next-state function δ (over inputs + state variables), read directly from
    /// the minimised model: after the fold a state variable's map entry **is** its δ.
    pub(crate) deltas: Vec<machine::Delta<B, C>>,
    /// The combinational outputs' δ, read directly from the minimised map (an output's value at a node is
    /// read from its δ; a state output instead reads its own state field).
    pub(crate) out_deltas: BTreeMap<Symbol, Bdd<B, C>>,
    /// The reachable stable states, discovered by one [`machine::explore`] BFS.
    pub(crate) explored: machine::Explored,
}

impl<'c, B: Brand, C: ManagerCell> Machine<'c, B, C> {
    /// Build the shared machine for `cell` from the minimised `bdds` map (built once in
    /// [`crate::model::Cell::analyse`]). Returns `None` — leaving the cell unexplored — when the machine
    /// width would exceed [`MAX_MACHINE_VARS`] (the combinatorial blow-up guard).
    pub fn build(
        cell: &'c AnalysedCell,
        bdds: &BTreeMap<Symbol, Bdd<B, C>>,
    ) -> Option<Machine<'c, B, C>>
    where
        C: Send + Sync,
    {
        let inputs = &cell.inputs;
        let n = inputs.len();

        let signals: Vec<&crate::model::AnalysedOutput> = cell.signals().collect();
        // Feedback was recomputed post-fold, so this classifier is now exact: every surviving state
        // variable genuinely self-reaches.
        let state_set = resolve::state_variables(&signals);
        // State variables in signal order (outputs first, then internals).
        let state_vars: Vec<Symbol> = signals
            .iter()
            .map(|s| s.name.clone())
            .filter(|nm| state_set.contains(nm))
            .collect();
        let k = state_vars.len();

        // Guard against a combinatorial blow-up on pathologically wide cells (now on the minimised width).
        if n + k > MAX_MACHINE_VARS {
            return None;
        }

        // The minimise fixpoint invariant (I3): every signal's signal-name support is a subset of the
        // state variables, so a state variable's next-state δ and a combinational output's δ are both a
        // direct lookup in the shared map — no per-signal composition remains.
        debug_assert!(
            signals.iter().all(|s| {
                bdds[&s.name]
                    .variables()
                    .all(|v| !bdds.contains_key(&v) || state_set.contains(&v))
            }),
            "analyse_machine: a signal's support escapes the state set — minimise invariant I3 broken"
        );

        // δ of each state variable (the machine's transition functions) and of each *combinational*
        // output are read directly from the minimised map — the arc derivation reads an output's value
        // from its δ, and both derivations seed exploration from them.
        let deltas: Vec<machine::Delta<B, C>> = state_vars
            .iter()
            .map(|v| (v.clone(), bdds[v].clone()))
            .collect();
        let out_deltas: BTreeMap<Symbol, Bdd<B, C>> = cell
            .outputs
            .iter()
            .filter(|o| !state_set.contains(&o.name))
            .map(|o| (o.name.clone(), bdds[&o.name].clone()))
            .collect();

        // Explore the reachable stable states once. Candidates are seeded from the on/off covers of every
        // signal function (state δ plus the combinational outputs, so combinational cells seed too);
        // [`machine::explore`] records the visitation order and predecessors, shared by both derivations.
        let seed_funcs: Vec<_> = deltas
            .iter()
            .map(|(_, d)| d.clone())
            .chain(out_deltas.values().cloned())
            .collect();
        let explored = machine::explore(&deltas, &seed_funcs, inputs, &state_vars);

        Some(Machine {
            cell,
            state_vars,
            state_set,
            deltas,
            out_deltas,
            explored,
        })
    }

    /// The value of `name` at a node, or `None` when the node does not define it: a state output reads
    /// its state field (absent ⇒ undefined); a combinational output is its δ evaluated at the node
    /// (`Err` ⇒ still depends on absent state ⇒ undefined). An arc is only measured where the output is
    /// defined at both ends.
    pub(crate) fn output_value(&self, name: &str, node: &Minterm<Symbol>) -> Option<bool> {
        if self.state_set.contains(name) {
            node.value_of(name)
        } else {
            // Every non-state output has a δ in `out_deltas` (one is computed for each of `cell.outputs`
            // when the machine is built), so this lookup cannot miss.
            debug_assert!(
                self.out_deltas.contains_key(name),
                "output_value: output {name:?} has no entry in out_deltas"
            );
            self.out_deltas[name].evaluate_fast(node)
        }
    }
}

/// Build the cell's state machine from the minimised `bdds` map and derive its arcs and hazards from the
/// shared exploration. The builder was minted once in [`crate::model::Cell::analyse`]; this pass only
/// reads the shared map. Returns an empty [`MachineAnalysis`] when the cell is not explored (the blow-up
/// guard).
pub fn analyse_machine<B: Brand, C: ManagerCell + Send + Sync>(
    cell: &AnalysedCell,
    bdds: &BTreeMap<Symbol, Bdd<B, C>>,
) -> MachineAnalysis {
    let Some(m) = Machine::build(cell, bdds) else {
        return MachineAnalysis::default();
    };
    let (arcs, hidden_arcs) = arcs::derive(&m);
    // Detect the hazards, then generate the constraints that avoid them — two separate stages.
    // Hazards are always detected (they drive the oscillation/race warnings and annotations);
    // constraint generation is gated on the cell's opt-in (the per-cell `constraint_arcs`, also set
    // for every cell by the global `--constraints` flag), so no constraint is generated — hence none
    // emitted — unless the cell requested it.
    let detected = confluence::detect(&m);
    let constraints = if cell.constraint_arcs_declared {
        confluence::constrain(&detected, &m.cell.clock_pins)
    } else {
        Vec::new()
    };
    MachineAnalysis {
        arcs,
        hidden_arcs,
        constraints,
        order_dependence: detected.order_dependence,
        oscillation: detected.oscillation,
        leakage: leakage::derive(&m),
    }
}

#[cfg(test)]
mod tests {
    use super::MAX_MACHINE_VARS;
    use crate::emit::arcs_tcl::{cell_arcs_tcl, ArcsTclOptions};
    use crate::emit::liberty::cell_liberty;
    use crate::emit::verilog::cell_verilog;
    use crate::model::analyse_one;

    #[test]
    fn oversized_cell_trips_the_blowup_guard() {
        // inputs + state variables > MAX_MACHINE_VARS ⇒ the machine is left unexplored, so arcs,
        // constraints and both detected hazards all come back empty (the MachineAnalysis::default path)
        // — yet the emitters must still run without panicking.
        let n = MAX_MACHINE_VARS + 1; // 23 primary inputs, 0 state variables ⇒ machine width 23 > 22
        let list = (0..n)
            .map(|i| format!("\"I{i}\""))
            .collect::<Vec<_>>()
            .join(", ");
        // Opt in to constraints so the empty result below is the *guard* suppressing generation
        // (MachineAnalysis::default), not merely the per-cell gate leaving `constraints` untouched.
        let src = format!(
            "[[cell]]\nname = \"WIDE\"\nconstraint_arcs = true\ninputs = [{list}]\n[cell.outputs]\nY = \"I0\"\n"
        );
        let cell = analyse_one(&src);
        assert!(cell.arcs.is_empty(), "guard must suppress arcs");
        assert!(
            cell.constraints.is_empty(),
            "guard must suppress constraints"
        );
        assert!(
            cell.oscillation.is_empty(),
            "guard must suppress oscillation"
        );
        assert!(
            cell.order_dependence.is_empty(),
            "guard must suppress order-dependent hazards"
        );
        assert!(
            cell.leakage.is_empty(),
            "guard must suppress leakage states"
        );
        // Emission still succeeds (no panic); the artifacts are simply arc-free.
        let _ = cell_arcs_tcl(&cell, ArcsTclOptions::default());
        let _ = cell_verilog(&cell);
        let _ = cell_liberty(&cell);
    }

    #[test]
    fn single_input_state_holder_is_coherent() {
        // Blind spot: a state-holding cell with fewer than two inputs. A single-input set-only keeper
        // (Q = A + Q) must be handled without panic. Its region view is a proper hysteretic state table
        // (Q holds while A is low, is set when A is high); it has no *measured* arc because the only
        // transition rises out of the uninitialised state, which is deliberately not characterised.
        let cell = analyse_one(
            r#"
[[cell]]
name = "KEEP"
inputs = ["A"]
[cell.outputs]
Q = "A + Q"
"#,
        );
        assert!(cell.oscillation.is_empty());
        assert_eq!(cell.regions.len(), 1);
        let q = &cell.regions[0];
        assert!(q.hysteretic, "a single-input keeper holds its own state");
        assert!(!q.on.is_empty(), "Q is forced high when A is high");
        // No measured arc: the only rise leaves the uninitialised state, which is not characterised.
        assert!(
            cell.arcs.is_empty(),
            "a single-input keeper has no arc between reachable stable states"
        );
        // Emission is well-formed: a statetable for the hysteretic output, and no panic on the arcs.
        assert!(cell_liberty(&cell).contains("statetable"));
        let _ = cell_arcs_tcl(&cell, ArcsTclOptions::default());
    }

    #[test]
    fn midsize_multistate_cell_is_coherent() {
        // Blind spot: a cell larger than the 2-input C-element but well within the guard, carrying
        // multiple state signals (internal master M and output Q) plus an async reset. Arcs, constraints
        // and regions are all produced coherently.
        let cell = analyse_one(
            r#"
[[cell]]
name = "DFFR"
inputs = ["CLK", "D", "R"]
async = ["R"]
clock = ["CLK"]
constraint_arcs = true
[cell.internal]
M = "!R*(!CLK*D + CLK*M)"
[cell.outputs]
Q = "!R*(CLK*M + !CLK*Q)"
"#,
        );
        // Two state signals (output Q, internal M) ⇒ one region entry per signal.
        assert_eq!(cell.regions.len(), cell.signals().count());
        assert_eq!(cell.regions.len(), 2);
        assert!(
            !cell.arcs.is_empty(),
            "a clocked DFF produces transition arcs"
        );
        assert!(
            cell.arcs.iter().any(|a| a.is_async),
            "R is a declared async pin, so its arcs are async-typed",
        );
        assert!(
            !cell.constraints.is_empty(),
            "the CLK/D setup-hold hazard is constrained",
        );
        let tcl = cell_arcs_tcl(&cell, ArcsTclOptions::default());
        assert!(tcl.contains("define_arc"));
    }
}
