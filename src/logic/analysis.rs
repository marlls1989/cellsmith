//! The shared **state-machine pass**: build a cell's asynchronous state machine once and derive both
//! its transition arcs ([`super::arcs`]) and its confluence hazards ([`super::confluence`]) from the
//! same exploration.
//!
//! A cell is a state machine over `inputs × state-variables` (see [`machine`] and [`resolve`]). Building
//! it — every signal's BDD, each state variable's next-state δ, the combinational outputs' δ, and the
//! one [`machine::explore`] BFS — is the same setup for both derivations, so it is done
//! **once** here and shared through [`Machine`]. Only plain data ([`Arc`], [`Constraint`],
//! [`Arbitration`]) escapes into [`MachineAnalysis`]; the live BDD handles never leave this pass.
//!
//! The BDD brand is a **generic type parameter** `<B, C>` carried by [`Machine`]: the builder is minted
//! per cell (a fresh brand each cell, so handles from two cells cannot be mixed) and lives on
//! [`analyse_machine`]'s stack for the duration of the pass.

use std::collections::{BTreeMap, BTreeSet};

use espresso_logic::bdd::{Bdd, BddBuilder, Brand, ManagerCell};
use espresso_logic::{bdd_builder, Symbol};

use crate::logic::arcs::{self, Arc, HiddenArc};
use crate::logic::confluence::{self, Constraint};
use crate::logic::interlock::Arbitration;
use crate::logic::{machine, resolve};
use crate::model::AnalysedCell;

/// The plain-data outcome of the shared machine pass: the transition arcs, the constraints derived to
/// avoid the cell's hazards, and its arbitration annotations. Empty when the cell is not explored (the
/// combinatorial blow-up guard, see [`MAX_MACHINE_VARS`]).
#[derive(Debug, Default)]
pub struct MachineAnalysis {
    pub arcs: Vec<Arc>,
    pub hidden_arcs: Vec<HiddenArc>,
    pub constraints: Vec<Constraint>,
    pub arbitration: Vec<Arbitration>,
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
pub(crate) struct Machine<'c, B: Brand, C: ManagerCell> {
    pub(crate) cell: &'c AnalysedCell,
    /// State variables in signal order (outputs first, then internals).
    pub(crate) state_vars: Vec<Symbol>,
    /// The same state variables as a set, for membership tests.
    pub(crate) state_set: BTreeSet<Symbol>,
    /// Each state variable's next-state function δ (over inputs + state variables).
    pub(crate) deltas: Vec<machine::Delta<B, C>>,
    /// The combinational outputs' δ, built **once** (an output's value at a node is read from its δ; a
    /// state output instead reads its own state field).
    pub(crate) out_deltas: BTreeMap<Symbol, Bdd<B, C>>,
    /// The reachable stable states, discovered by one [`machine::explore`] BFS.
    pub(crate) explored: machine::Explored,
}

impl<'c, B: Brand, C: ManagerCell> Machine<'c, B, C> {
    /// Build the shared machine for `cell` using `builder`'s manager. Returns `None` — leaving the cell
    /// unexplored — when the machine width would exceed [`MAX_MACHINE_VARS`] (the combinatorial blow-up guard).
    pub(crate) fn build(
        builder: &BddBuilder<B, C>,
        cell: &'c AnalysedCell,
    ) -> Option<Machine<'c, B, C>> {
        let inputs = &cell.inputs;
        let n = inputs.len();

        let signals: Vec<&crate::model::AnalysedOutput> = cell.signals().collect();
        let deps = resolve::dependency_map(&signals);
        let state_set = resolve::state_variables(&signals);
        // State variables in signal order (outputs first, then internals).
        let state_vars: Vec<Symbol> = signals
            .iter()
            .map(|s| s.name.clone())
            .filter(|nm| state_set.contains(nm))
            .collect();
        let k = state_vars.len();

        // Guard against a combinatorial blow-up on pathologically wide cells.
        if n + k > MAX_MACHINE_VARS {
            return None;
        }

        let bdds: BTreeMap<Symbol, Bdd<B, C>> = signals
            .iter()
            .map(|s| (s.name.clone(), builder.build(&s.expr)))
            .collect();

        // δ of each state variable (the machine's transition functions), and of each *combinational*
        // output. The combinational deltas are built once here — the arc derivation reads an output's
        // value from its δ, and both derivations seed exploration from them.
        let deltas: Vec<machine::Delta<B, C>> = state_vars
            .iter()
            .map(|v| (v.clone(), resolve::delta(v, &bdds, &deps, &state_set)))
            .collect();
        let out_deltas: BTreeMap<Symbol, Bdd<B, C>> = cell
            .outputs
            .iter()
            .filter(|o| !state_set.contains(&o.name))
            .map(|o| {
                (
                    o.name.clone(),
                    resolve::delta(&o.name, &bdds, &deps, &state_set),
                )
            })
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
}

/// Build the cell's state machine once and derive its arcs and hazards from the shared exploration. The
/// builder is minted here (a fresh brand for this cell only) and lives on this stack for the whole pass.
/// Returns an empty [`MachineAnalysis`] when the cell is not explored (the blow-up guard).
pub fn analyse_machine(cell: &AnalysedCell) -> MachineAnalysis {
    let builder = bdd_builder!();
    let Some(m) = Machine::build(&builder, cell) else {
        return MachineAnalysis::default();
    };
    let (arcs, hidden_arcs) = arcs::derive(&m);
    let hz = confluence::derive(&m);
    MachineAnalysis {
        arcs,
        hidden_arcs,
        constraints: hz.constraints,
        arbitration: hz.arbitration,
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
        // constraints and arbitration all come back empty (the MachineAnalysis::default path) — yet the
        // emitters must still run without panicking.
        let n = MAX_MACHINE_VARS + 1; // 23 primary inputs, 0 state variables ⇒ machine width 23 > 22
        let list = (0..n)
            .map(|i| format!("\"I{i}\""))
            .collect::<Vec<_>>()
            .join(", ");
        let src =
            format!("[[cell]]\nname = \"WIDE\"\ninputs = [{list}]\n[cell.outputs]\nY = \"I0\"\n");
        let cell = analyse_one(&src);
        assert!(cell.arcs.is_empty(), "guard must suppress arcs");
        assert!(
            cell.constraints.is_empty(),
            "guard must suppress constraints"
        );
        assert!(
            cell.arbitration.is_empty(),
            "guard must suppress arbitration"
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
        assert!(cell.arbitration.is_empty());
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
        let tcl = cell_arcs_tcl(
            &cell,
            ArcsTclOptions {
                emit_constraints: true,
                ..Default::default()
            },
        );
        assert!(tcl.contains("define_arc"));
    }
}
