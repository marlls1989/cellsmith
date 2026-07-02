//! The shared **state-machine pass**: build a cell's asynchronous state machine once and derive both
//! its transition arcs ([`super::arcs`]) and its confluence hazards ([`super::confluence`]) from the
//! same exploration.
//!
//! A cell is a state machine over `inputs × state-variables` (see [`machine`] and [`resolve`]). Building
//! it — every signal's BDD, each state variable's next-state δ, the combinational outputs' δ, the shared
//! headers, and the one [`machine::explore`] BFS — is the same setup for both derivations, so it is done
//! **once** here and shared through [`Machine`]. Only plain data ([`Arc`], [`Constraint`],
//! [`Arbitration`]) escapes into [`MachineAnalysis`]; the live BDD handles never leave this pass.
//!
//! The BDD brand is a **generic type parameter** `<B, C>` carried by [`Machine`]: the builder is minted
//! per cell (a fresh brand each cell, so handles from two cells cannot be mixed) and lives on
//! [`analyse_machine`]'s stack for the duration of the pass.

use std::collections::{BTreeMap, BTreeSet};

use espresso_logic::bdd::{Bdd, BddBuilder, Brand, ManagerCell};
use espresso_logic::{bdd_builder, Symbol, Symbols};

use crate::logic::arcs::{self, Arc};
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
    pub constraints: Vec<Constraint>,
    pub arbitration: Vec<Arbitration>,
}

/// The single home for the combinatorial blow-up guard: a cell whose machine header (inputs + state
/// variables) exceeds this width is not explored at all — both arcs and hazards come back empty.
pub(crate) const MAX_MACHINE_VARS: usize = 22;

/// A cell's asynchronous state machine, built once and shared by the arc and confluence derivations. The
/// BDD brand is a generic parameter scoped to the builder that minted these handles.
pub(crate) struct Machine<'c, B: Brand, C: ManagerCell> {
    pub(crate) cell: &'c AnalysedCell,
    /// State variables in signal order (outputs first, then internals).
    pub(crate) state_vars: Vec<String>,
    /// The same state variables as a set, for membership tests.
    pub(crate) state_set: BTreeSet<String>,
    /// Each state variable's next-state function δ (over inputs + state variables).
    pub(crate) deltas: Vec<machine::Delta<B, C>>,
    /// The combinational outputs' δ, built **once** (an output's value at a node is read from its δ; a
    /// state output instead reads its own state field).
    pub(crate) out_deltas: BTreeMap<String, Bdd<B, C>>,
    /// The full node header (inputs + state variables).
    pub(crate) full_header: std::sync::Arc<Symbols<Symbol>>,
    /// The input-only header the arcs and constraints are expressed over.
    pub(crate) input_header: std::sync::Arc<Symbols<Symbol>>,
    /// The reachable stable states, discovered by one [`machine::explore`] BFS.
    pub(crate) explored: machine::Explored,
}

impl<'c, B: Brand, C: ManagerCell> Machine<'c, B, C> {
    /// Build the shared machine for `cell` using `builder`'s manager. Returns `None` — leaving the cell
    /// unexplored — when the header would exceed [`MAX_MACHINE_VARS`] (the combinatorial blow-up guard).
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
        let state_vars: Vec<String> = signals
            .iter()
            .map(|s| s.name.clone())
            .filter(|nm| state_set.contains(nm))
            .collect();
        let k = state_vars.len();

        // Guard against a combinatorial blow-up on pathologically wide cells.
        if n + k > MAX_MACHINE_VARS {
            return None;
        }

        let bdds: BTreeMap<String, Bdd<B, C>> = signals
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
        let out_deltas: BTreeMap<String, Bdd<B, C>> = cell
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

        // The shared headers: the full node header (inputs + state variables) and the input-only header
        // the arcs and constraints are expressed over.
        let full_names: Vec<String> = inputs.iter().cloned().chain(state_vars.clone()).collect();
        let full_header = machine::header(&full_names);
        let input_header = machine::header(inputs);

        // Explore the reachable stable states once. Candidates are seeded from the on/off covers of every
        // signal function (state δ plus the combinational outputs, so combinational cells seed too);
        // [`machine::explore`] records the visitation order and predecessors, shared by both derivations.
        let seed_funcs: Vec<_> = deltas
            .iter()
            .map(|(_, d)| d.clone())
            .chain(out_deltas.values().cloned())
            .collect();
        let explored = machine::explore(&deltas, &seed_funcs, &full_header, inputs, &state_vars);

        Some(Machine {
            cell,
            state_vars,
            state_set,
            deltas,
            out_deltas,
            full_header,
            input_header,
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
    let arcs = arcs::derive(&m);
    let hz = confluence::derive(&m);
    MachineAnalysis {
        arcs,
        constraints: hz.constraints,
        arbitration: hz.arbitration,
    }
}
