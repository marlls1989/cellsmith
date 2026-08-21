//! The shared **state-machine pass**: build a cell's asynchronous state machine once and derive both
//! its transition arcs ([`super::arcs`]) and its confluence hazards ([`super::confluence`]) from the
//! same exploration.
//!
//! A cell is a state machine over `inputs × coordinates` — every signal surviving the minimisation, the
//! state variables and the combinational survivors alike (see [`machine`] and `resolve`). The
//! signals' BDDs are built and minimised once in [`crate::model::Cell::analyse`]; this pass reads that
//! shared map. After the fold every coordinate's next-state δ **is** its entry in the map — a direct
//! lookup, no per-signal composition. Only the one
//! `machine::explore` BFS is set up here, and it is the same setup for both derivations, so it is done
//! **once** and shared through [`Machine`]. It is done once per CELL rather than once per view: a cell
//! that exposes internal nodes is analysed as two views, and the second one takes the first's explored
//! states projected onto its own coordinates ([`Exploration`]) instead of exploring again. Only plain
//! data ([`Arc`]; the detected [`Hazard`]s; the
//! generated `Constraint`s; the explored states themselves, which are minterms and carry no BDD
//! handle) escapes
//! into [`Derivations`]; the live BDD handles never leave this pass.
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
use crate::logic::confluence;
use crate::logic::constraint::{self, Constraint};
use crate::logic::hazard::Hazard;
use crate::logic::leakage::{self, LeakageState};
use crate::logic::{machine, resolve, width};
use crate::model::{AnalysedCell, ConstraintPins};

/// What one fully explored machine yields: the transition arcs, the detected hazards — one [`Hazard`]
/// per (cause, outcome) pair the two detection passes observe — and the constraints generated to avoid
/// them.
///
/// This is what [`analyse_machine`] returns on success, and it is matched by `Cell::analyse` into the
/// corresponding [`crate::model::AnalysedCell`] fields (see `model.rs`). A pass the budget stopped
/// derives none of these and carries the counter that stopped it instead.
#[derive(Debug)]
pub struct Derivations {
    pub(crate) arcs: Vec<Arc>,
    pub(crate) hidden_arcs: Vec<HiddenArc>,
    pub(crate) constraints: Vec<Constraint>,
    pub(crate) hazards: Vec<Hazard>,
    pub(crate) leakage: Vec<LeakageState>,
    pub(crate) edge: crate::logic::edge::EdgeArcs,
    /// The exploration this pass performed, handed back so a second view of the same cell projects it
    /// onto its own coordinates instead of repeating it. `None` when the pass reused an exploration.
    pub(crate) explored: Option<machine::Explored>,
}

/// Where a view's explored states come from.
///
/// A cell explores once. The view that owns the exploration is `Fresh` and performs it under the
/// budget; a second view of the same cell is `Reused` and carries the states that view reached. A
/// ceiling that stops the exploration ends the analysis there, so a `Reused` view is only ever reached
/// once the exploration succeeded.
pub enum Exploration<'e> {
    /// This view performs the exploration, bounded by this budget.
    Fresh(&'e machine::ExplorationBudget),
    /// This view reuses the states the cell's other view already explored.
    Reused(&'e machine::Explored),
}

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
    /// The δ of every signal surviving the minimisation that is *not* a state variable — the
    /// combinational half of the machine's [`machine::Coordinates`], in signal order (outputs first, then
    /// internals). These are the outputs and exposed internals the minimisation kept because something
    /// addresses them by name; each is a node column of its own, stepped with the state variables.
    pub(crate) combinational: Vec<machine::Delta<B, C>>,
    /// The cell's exposed internal nodes — the internals the spec lists in `expose`, in declared order,
    /// which is the order [`super::arcs::ArcLevels`] fills its exposed levels in.
    ///
    /// Exposure is read only to SAMPLE levels: these names are absent from the exploration's
    /// `seed_funcs` below, so exposing a node cannot change which states are reached. That is what makes
    /// the change an arcs-only one — the machine explores the same state space either way.
    pub(crate) exposed: Vec<Symbol>,
    /// The reachable stable states over THIS view's coordinates, from the cell's one
    /// [`machine::explore`] BFS — run here, or carried onto these coordinates from the view that ran it
    /// (see [`Exploration`]).
    pub(crate) explored: machine::Explored,
}

impl<'c, B: Brand, C: ManagerCell> Machine<'c, B, C> {
    /// Build the shared machine for `cell` from the minimised `bdds` map (built once in
    /// [`crate::model::Cell::analyse`]), taking its explored states from `exploration` — discovered here
    /// under a budget, or carried over from the cell's other view. That is the only fallible step: a
    /// fresh exploration stops when one of the budget's two counters passes its ceiling, and that
    /// counter comes back as the error.
    pub fn build(
        cell: &'c AnalysedCell,
        bdds: &BTreeMap<Symbol, Bdd<B, C>>,
        exploration: Exploration<'_>,
    ) -> Result<Machine<'c, B, C>, machine::ExplorationLimit>
    where
        C: Send + Sync,
    {
        let inputs = &cell.inputs;

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

        // `minimise`'s minimised-model support invariant (I3): every signal's signal-name support is a
        // subset of the state variables, so a state variable's next-state δ and a combinational
        // output's δ are both a direct lookup in the shared map — no per-signal composition remains.
        debug_assert!(
            signals.iter().all(|s| {
                bdds[&s.name]
                    .variables()
                    .all(|v| !bdds.contains_key(&v) || state_set.contains(&v))
            }),
            "analyse_machine: a signal's support escapes the state set — minimise invariant I3 broken"
        );

        // The machine's coordinates, both halves read directly from the minimised map: the state
        // variables' δ (the transition functions), and the δ of every other surviving signal. I3 leaves
        // a surviving signal either self-reaching or preserved with no consumers, so the second half is
        // exactly the cell's combinational outputs plus its surviving combinational exposures — every
        // other internal was folded away and is not in `signals()` at all.
        let deltas: Vec<machine::Delta<B, C>> = state_vars
            .iter()
            .map(|v| (v.clone(), bdds[v].clone()))
            .collect();
        let combinational: Vec<machine::Delta<B, C>> = signals
            .iter()
            .map(|s| &s.name)
            .filter(|nm| !state_set.contains(*nm))
            .map(|nm| (nm.clone(), bdds[nm].clone()))
            .collect();
        // The exposed nodes surviving in this view. A combinational exposure is one of the coordinates
        // above; a state-variable exposure is one of the state columns.
        let exposed: Vec<Symbol> = cell.exposed_signals().cloned().collect();

        let coords = machine::Coordinates {
            state: &deltas,
            combinational: &combinational,
        };
        let explored = match exploration {
            // Explore the reachable stable states. Candidates are seeded from the on/off covers of every
            // signal function (state δ plus the combinational OUTPUTS, so combinational cells seed too,
            // while an exposed internal stays out of the pool and cannot move the exploration);
            // [`machine::explore`] records the visitation order and predecessors, shared by both
            // derivations.
            Exploration::Fresh(budget) => {
                let output_names: BTreeSet<&Symbol> =
                    cell.outputs.iter().map(|o| &o.name).collect();
                let seed_funcs: Vec<_> = deltas
                    .iter()
                    .chain(
                        combinational
                            .iter()
                            .filter(|(n, _)| output_names.contains(n)),
                    )
                    .map(|(_, d)| d.clone())
                    .collect();
                machine::explore(coords, &seed_funcs, inputs, budget)?
            }
            // The cell's other view already explored these states, so they are carried onto THIS view's
            // node columns — the inputs followed by this view's coordinates, in `machine::explore`'s own
            // column order. The target names are computed here and never passed in, so a projection onto
            // the wrong view's coordinates cannot be written.
            Exploration::Reused(e) => {
                let full_names: Vec<Symbol> =
                    inputs.iter().cloned().chain(coords.names()).collect();
                e.project_to(&full_names)
            }
        };

        Ok(Machine {
            cell,
            state_vars,
            state_set,
            deltas,
            combinational,
            exposed,
            explored,
        })
    }

    /// MEASUREMENT ELIGIBILITY: a reachable stable state is measurement-eligible iff every STATE column
    /// is determinate — no don't-care. A don't-care is a MISSING variable, never coerced to 0/1, so an
    /// ineligible start would read an uninitialised latch as though it held a value. Traversal is
    /// untouched — a partial state stays a seed in the explored order — but no measurement quantifies
    /// over one: the arc derivation, the behavioural edge classification and the hazard probes all gate
    /// on this one predicate.
    pub(crate) fn arc_eligible(&self, s: &Minterm<Symbol>) -> bool {
        self.state_vars
            .iter()
            .all(|w| s.value_of(w.as_str()).is_some())
    }

    /// The combinational coordinates that carry an external output pin, in signal order — the half of
    /// [`machine::Coordinates::combinational`] the exploration seeds from and the edge classification
    /// takes its candidate functions from. The rest are exposed internals, which no such quantity reads.
    pub(crate) fn combinational_outputs(&self) -> impl Iterator<Item = &machine::Delta<B, C>> {
        self.combinational
            .iter()
            .filter(|(n, _)| self.cell.outputs.iter().any(|o| o.name == *n))
    }

    /// The machine's coordinate δ set, in [`machine::Coordinates`] order — the state variables followed
    /// by the combinational survivors — which is the set one [`machine::step`] writes and the set
    /// [`machine::explore`] settled the reachable states over. Every re-walk settles over all of them: a
    /// narrower set leaves a combinational coordinate's column holding its pre-toggle value, so the node
    /// would read back stale and would not match the explored state it belongs to.
    pub(crate) fn coordinate_deltas(&self) -> Vec<machine::Delta<B, C>> {
        machine::Coordinates {
            state: &self.deltas,
            combinational: &self.combinational,
        }
        .stepped()
    }

    /// The value of `name` at a node, or `None` when the node does not define it. Every output is a
    /// coordinate of the machine — a state variable or a combinational survivor — so the value is that
    /// node column, absent where the node leaves it undetermined. The `Option` is what the traversal
    /// needs: it walks partial states too, and a node it has not settled leaves columns absent. No
    /// MEASUREMENT reads one — the arc derivation, the hazard probes and [`leakage::derive`] all gate on
    /// [`Self::arc_eligible`] first, and at such a node every output resolves, so each unwraps.
    pub(crate) fn output_value(&self, name: &str, node: &Minterm<Symbol>) -> Option<bool> {
        node.value_of(name)
    }

    /// The value an exposed node holds at a node, read as [`Self::output_value`] is: an exposure is a
    /// coordinate either way — a state variable or a combinational survivor — so its level is that node
    /// column, absent where the node leaves it undetermined.
    ///
    /// This `Option` is the RAW read. Every SAMPLING site instead wraps it in `.expect()` on the
    /// determinacy invariant, which holds there: see [`super::arcs::ArcLevels::at`].
    pub(crate) fn exposed_value(&self, name: &str, node: &Minterm<Symbol>) -> Option<bool> {
        node.value_of(name)
    }
}

/// Build the cell's state machine from the minimised `bdds` map and derive its arcs and hazards from the
/// shared exploration, which `exploration` either budgets or supplies. The builder was minted once in
/// [`crate::model::Cell::analyse`]; this pass only reads the shared map. An exploration stopped by one of
/// the budget's counters comes back as the error carrying that counter (see
/// [`machine::ExplorationBudget`]): nothing was derived, and the caller reports it.
///
/// A [`Exploration::Reused`] view cannot itself be stopped — the exploration it reads already ran to
/// completion in the view that performed it — so the error is reached only by a [`Exploration::Fresh`]
/// view.
pub fn analyse_machine<B: Brand, C: ManagerCell + Send + Sync>(
    cell: &AnalysedCell,
    bdds: &BTreeMap<Symbol, Bdd<B, C>>,
    collapse: bool,
    exploration: Exploration<'_>,
) -> Result<Derivations, machine::ExplorationLimit> {
    let reused = matches!(exploration, Exploration::Reused(_));
    let m = Machine::build(cell, bdds, exploration)?;
    let (arcs, hidden_arcs) = arcs::derive(&m);
    // Detect the hazards, then generate the constraints that avoid them — two separate stages. Every
    // hazard is always detected — the race-cause and pulse-cause ones alike (they drive the warnings and
    // annotations); what the cell's selection decides is which of them get a constraint. The selection
    // (the per-cell `constraint_arcs`, unioned for every cell with the global `--constraints` flag) names
    // the pins whose constraints are wanted, and it acts on the constraints generation RETURNS, never on
    // the hazards handed to it: a pin nobody asked constraints for is still probed and still reported,
    // and only loses its blocks. Which pins reach a given constraint is that constraint's kind to answer
    // ([`Constraint::selected_by`]), the two ends of a symmetric separation being equals and those of a
    // directed one not. Emission then picks each general block's representative among exactly
    // the constraints that come out, so an unselected pin's observations decide nothing about a selected
    // pin's block.
    let detected = confluence::detect(&m);
    let width_dependence = width::detect(&m);
    // The one `hazards` record set: what the two detection passes returned, concatenated. Generation
    // reads it whole — a constraint follows its record's cause, so both passes' records reach the one
    // generator.
    let hazards: Vec<Hazard> = detected.into_iter().chain(width_dependence).collect();
    let constraints = match &cell.constraint_arcs_declared {
        // Nothing is wanted of any pin, so generation is skipped whole rather than run and discarded.
        ConstraintPins::Off => Vec::new(),
        selection => constraint::constrain(&hazards, &m.cell.clock_pins)
            .into_iter()
            .filter(|c| c.selected_by(selection))
            .collect(),
    };
    // Behavioural edge classification is read-only over the explored machine — it mints only
    // already-existing names and mutates nothing (the exploration-unchanged invariant holds BY
    // CONSTRUCTION). The derived `arcs` are its label domain: every timing arc it labels is one of the
    // pipeline's own delay arcs. The opt-out (`collapse == false`) SKIPS the classify() call entirely
    // rather than discarding its result: a real bypass, byte-identical to the Default annotation.
    let edge = if collapse {
        crate::logic::edge::classify(&m, &arcs)
    } else {
        crate::logic::edge::EdgeArcs::default()
    };
    let leakage = leakage::derive(&m);
    Ok(Derivations {
        arcs,
        hidden_arcs,
        constraints,
        hazards,
        leakage,
        edge,
        // Handed back only by the view that performed the exploration: the derivations above are the
        // whole of what a reusing view owes its caller.
        explored: (!reused).then_some(m.explored),
    })
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use espresso_logic::Symbol;

    use crate::emit::arcs_tcl::{cell_arcs_tcl, ArcsTclOptions};
    use crate::emit::liberty::cell_liberty;
    use crate::emit::tcl::VectorValue;
    use crate::logic::arcs::Edge;
    use crate::logic::constraint::{Constraint, ConstraintKind};
    use crate::logic::hazard::{Cause, Outcome};
    use crate::logic::machine::{ExplorationBudget, ExplorationLimit};
    use crate::logic::resolve;
    use crate::model::{analyse_one, parse_spec, AnalysedCell, ModelError};

    /// Analyse the one cell `src` declares under the default budget, expecting the exploration to be
    /// stopped: the [`ModelError`] the failure comes back as.
    fn budget_verdict(src: &str) -> ModelError {
        parse_spec(src)
            .expect("the spec parses")
            .cells
            .remove(0)
            .analyse()
            .expect_err("the exploration passes a ceiling, so the analysis fails")
    }

    #[test]
    fn oversized_cell_trips_the_candidate_budget() {
        // 24 inputs, so each forced cover cube of Y carries 23 don't-care input columns and expands to
        // 2^23 seed minterms — past the default candidate ceiling. The exploration stops there and the
        // analysis fails with it: a cell derives no arcs, hazards, leakage states or constraints
        // without an exploration, so there is nothing to hand back.
        let n = 24;
        let list = (0..n)
            .map(|i| format!("\"I{i}\""))
            .collect::<Vec<_>>()
            .join(", ");
        let src = format!(
            "[[cell]]\nname = \"WIDE\"\nconstraint_arcs = true\ninputs = [{list}]\n[cell.outputs]\nY = \"I0\"\n"
        );
        let err = budget_verdict(&src);
        assert_eq!(
            err,
            ModelError::Exploration {
                cell: Symbol::from("WIDE"),
                source: ExplorationLimit::Candidates(ExplorationBudget::default().candidates),
            },
            "the candidate counter is the one that stopped it, and the failure names the cell",
        );
        // The counter and its ceiling read off the leaf; the cell and the flag are the wrapper's.
        let msg = err.to_string();
        assert!(
            msg.contains("cell \"WIDE\"")
                && msg.contains("candidate budget")
                && msg.contains("--max-candidates"),
            "the diagnostic names the cell, the budget and the flag: {msg}"
        );
    }

    #[test]
    fn an_exposing_cell_over_budget_reports_the_same_verdict() {
        // The same over-budget shape, exposing an internal node so the cell would carry two views. The
        // cell explores once, in the arc view, and the ceiling that stopped it ends the analysis there:
        // the model view is never built, and the one verdict names the cell exactly once.
        let n = 24;
        let list = (0..n)
            .map(|i| format!("\"I{i}\""))
            .collect::<Vec<_>>()
            .join(", ");
        // Y is the only seed function (an exposed internal stays out of the pool), and each cube of its
        // forced cover carries 23 don't-care input columns, so the first cube alone is charged at 2^23 —
        // past the ceiling. That keeps the fixture cheap as well as over budget: a cube charged past the
        // ceiling is never expanded.
        let src = format!(
            "[[cell]]\nname = \"WIDEX\"\nconstraint_arcs = true\nexpose = [\"M\"]\ninputs = [{list}]\n[cell.internal]\nM = \"I0\"\n[cell.outputs]\nY = \"M\"\n"
        );
        assert_eq!(
            budget_verdict(&src),
            ModelError::Exploration {
                cell: Symbol::from("WIDEX"),
                source: ExplorationLimit::Candidates(ExplorationBudget::default().candidates),
            },
            "an exposing cell reaches the same single verdict as one with no view of its own",
        );
    }

    #[test]
    fn wide_machine_with_a_narrow_pool_is_analysed() {
        // A machine 24 coordinates wide — 6 inputs and 18 state variables — is analysed in full: the
        // candidate counter reads the input columns alone (6 of them, so no cube expands past 2^6 seed
        // minterms) and the state counter reads the states actually discovered, so a cell carrying many
        // state variables is no longer turned away for its width.
        //
        // Each `Qj` is set at one input vector, holds at the complementary vector and clears everywhere
        // else: genuine memory (under the hold vector its δ reads `Qj`), a distinct δ per `j` so the
        // minimisation dedups none of them, and one hold vector each so the explored set stays close to
        // the 2^6 input vectors.
        let (n, k) = (6, 18);
        let literal = |i: usize, v: bool| if v { format!("I{i}") } else { format!("!I{i}") };
        let vector = |bits: usize, invert: bool| {
            (0..n)
                .map(|i| literal(i, ((bits >> i) & 1 == 1) != invert))
                .collect::<Vec<_>>()
                .join("*")
        };
        let inputs = (0..n)
            .map(|i| format!("\"I{i}\""))
            .collect::<Vec<_>>()
            .join(", ");
        let outputs = (0..k)
            .map(|j| format!("Q{j} = \"{} + Q{j}*{}\"", vector(j, true), vector(j, false)))
            .collect::<Vec<_>>()
            .join("\n");
        let cell = analyse_one(&format!(
            "[[cell]]\nname = \"WIDESTATE\"\ninputs = [{inputs}]\n[cell.outputs]\n{outputs}\n"
        ));

        let signals: Vec<&crate::model::AnalysedOutput> = cell.signals().collect();
        let state_vars = resolve::state_variables(&signals);
        assert_eq!(state_vars.len(), k, "every output is a state variable");
        assert_eq!(cell.inputs.len() + state_vars.len(), 24, "machine width");
        // A stopped exploration fails the analysis, so `analyse_one` returning at all is the exploration
        // having run to completion.
        assert!(
            !cell.arcs.is_empty(),
            "a fully explored machine yields arcs"
        );
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
        assert!(!cell.hazards.iter().any(|h| {
            matches!(h.cause, Cause::Toggle { .. } | Cause::Race { .. })
                && h.outcome == Outcome::Oscillation
        }));
        assert_eq!(cell.regions.len(), 1);
        let q = &cell.regions[0];
        assert!(q.hysteretic, "a single-input keeper holds its own state");
        assert!(!q.on.is_empty(), "Q is forced high when A is high");
        // No measured arc: the only rise leaves the uninitialised state, which is not characterised.
        assert!(
            cell.arcs.is_empty(),
            "a single-input keeper has no arc between reachable stable states"
        );
        // Nor any width-dependent hazard, for the same reason: that one rise is the cell's only
        // transition, so at every state a pulse may start from Q is already high and A's toggle leaves
        // it there — every cut of every pulse lands back where it started.
        assert!(
            !cell
                .hazards
                .iter()
                .any(|h| matches!(h.cause, Cause::Pulse { .. })),
            "a single-input keeper's pulses all settle back to where they started"
        );
        // Emission is well-formed: a statetable for the hysteretic output, and no panic on the arcs.
        assert!(liberty_parser::liberty::Liberty(cell_liberty(&cell))
            .to_string()
            .contains("statetable"));
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
            cell.arcs
                .iter()
                .any(|a| cell.async_pins.contains(&a.related)),
            "R is a declared async pin, so its arcs are async-typed",
        );
        assert!(
            !cell.constraints.is_empty(),
            "the CLK/D setup-hold hazard is constrained",
        );
        let tcl = cell_arcs_tcl(&cell, ArcsTclOptions::default());
        assert!(tcl.contains("define_arc"));
    }

    #[test]
    fn a_named_pin_keeps_its_own_constraints_and_leaves_detection_alone() {
        // A DFF constrains both its pins: `D`, held around the clock it races, and `CLK`, whose own
        // pulse has a minimum width. Naming one of the two decides which constraints are generated and
        // nothing else — the hazards behind BOTH are still detected and still reported, so the pin left
        // out keeps its warning and loses only its blocks.
        // The pins constrained, as a set: which observation supplied a constraint states nothing, so
        // two runs agree here without agreeing on the records behind it.
        let pins = |c: &crate::model::AnalysedCell| {
            let mut v: Vec<String> = c.constraints.iter().map(|k| k.pin.to_string()).collect();
            v.sort();
            v.dedup();
            v
        };
        // What the warnings are rendered from: each detected hazard's cause, named by its pins.
        let detected = |c: &crate::model::AnalysedCell| {
            let mut v: Vec<String> = c
                .hazards
                .iter()
                .map(|h| match &h.cause {
                    Cause::Pulse { pin, .. } => format!("pulse {pin}"),
                    Cause::Toggle { pin } => format!("toggle {}", pin.pin),
                    Cause::Race { pins: [x, y] } => format!("race {}+{}", x.pin, y.pin),
                })
                .collect();
            v.sort();
            v.dedup();
            v
        };

        let every = analyse_one(&dff("true"));
        let data_only = analyse_one(&dff("\"D\""));
        assert_eq!(pins(&every), ["CLK", "D"]);
        assert_eq!(pins(&data_only), ["D"]);
        assert_eq!(
            detected(&data_only),
            detected(&every),
            "a pin left out of the selection is still probed and still reported",
        );
        assert!(
            detected(&data_only).contains(&"pulse CLK".to_owned()),
            "the width hazard behind the dropped constraint is one of them",
        );

        // And the deck follows the selection: the separation blocks come out, the minimum width the
        // unselected `CLK` would have carried does not.
        let tcl = cell_arcs_tcl(&data_only, ArcsTclOptions::default());
        assert!(tcl.contains("-type setup"));
        assert!(tcl.contains("-type hold"));
        assert!(!tcl.contains("-type min_pulse_width"));
    }

    #[test]
    fn exposure_leaves_the_exploration_untouched() {
        // Exposed nodes are read to sample levels and never seed the exploration, so the same cell
        // explores the same states in the same order whether or not it exposes its master — and an
        // exposure-free cell carries no exposed nodes and no δ for them at all.
        let dff = |expose: &str| {
            format!(
                r#"
[[cell]]
name = "DFF"
inputs = ["CLK", "D"]
clock = ["CLK"]
{expose}[cell.internal]
M = "!CLK*D + CLK*M"
[cell.outputs]
Q = "CLK*M + !CLK*Q"
"#
            )
        };
        let budget = crate::logic::machine::ExplorationBudget::default();
        let plain = analyse_one(&dff(""));
        let plain_builder = espresso_logic::sync_bdd_builder!();
        let plain_bdds = crate::model::build_signal_bdds(&plain, &plain_builder);
        let mp = super::Machine::build(&plain, &plain_bdds, super::Exploration::Fresh(&budget))
            .expect("fixture is explored");

        let exposed = analyse_one(&dff("expose = [\"M\"]\n"));
        let exposed_builder = espresso_logic::sync_bdd_builder!();
        let exposed_bdds = crate::model::build_signal_bdds(&exposed, &exposed_builder);
        let me = super::Machine::build(&exposed, &exposed_bdds, super::Exploration::Fresh(&budget))
            .expect("fixture is explored");

        assert!(mp.exposed.is_empty(), "nothing is exposed");
        assert_eq!(me.exposed, ["M"]);
        assert!(
            me.state_set.contains("M"),
            "the master holds memory, so exposing it names a state coordinate",
        );
        assert!(
            mp.combinational.is_empty() && me.combinational.is_empty(),
            "both of this cell's signals hold memory, so neither view has a combinational coordinate",
        );
        // As sets: the walk records a level's states in whatever order its workers claimed them, so
        // two runs of the same cell agree on WHICH states are reached without agreeing on the sequence.
        assert_eq!(
            mp.explored
                .order
                .iter()
                .collect::<std::collections::BTreeSet<_>>(),
            me.explored
                .order
                .iter()
                .collect::<std::collections::BTreeSet<_>>(),
            "exposing a node does not change which states are reached",
        );
    }

    /// The wave-1 coordinate fixture, built once per test: the keeper `Q` — a state variable — beside
    /// the exposed combinational node `W`, so one cell carries both kinds of coordinate. The
    /// minimisation runs with `W` preserved (`Preserved::with_exposed`), which is what keeps a
    /// combinational exposure in the model at all.
    const COORDINATE_FIXTURE: &str = r#"
[[cell]]
name = "C2X"
inputs = ["A", "B"]
expose = ["W"]
[cell.internal]
W = "A*B"
[cell.outputs]
Q = "W + Q*(A+B)"
"#;

    /// `COORDINATE_FIXTURE` minimised with its exposure preserved, ready for [`super::Machine::build`].
    fn coordinate_fixture() -> crate::model::AnalysedCell {
        let mut cell = crate::model::parse_spec(COORDINATE_FIXTURE)
            .unwrap()
            .cells
            .remove(0)
            .analyse_signals()
            .unwrap();
        let builder = espresso_logic::sync_bdd_builder!();
        let mut bdds = crate::model::build_signal_bdds(&cell, &builder);
        let order: Vec<espresso_logic::Symbol> = cell.signals().map(|s| s.name.clone()).collect();
        let preserved = crate::logic::minimise::Preserved::with_exposed(
            cell.outputs.iter().map(|o| o.name.clone()).collect(),
            cell.exposed.iter().cloned().collect(),
        );
        let min = crate::logic::minimise::minimise_state_space(&mut bdds, &order, &preserved);
        crate::model::recompute_signal_metadata(&mut cell, &bdds, &min);
        cell
    }

    #[test]
    fn every_explored_node_carries_a_column_per_coordinate() {
        // What promoting the combinational survivors to coordinates establishes: each one is a column
        // of every explored node, carrying exactly what its δ evaluates to there — the same relation a
        // state variable's column already stood in. Read over both halves at once, so a coordinate that
        // was left out of the node's columns, or left holding a value its δ contradicts, fails here.
        let cell = coordinate_fixture();
        let builder = espresso_logic::sync_bdd_builder!();
        let bdds = crate::model::build_signal_bdds(&cell, &builder);
        let m = super::Machine::build(
            &cell,
            &bdds,
            super::Exploration::Fresh(&crate::logic::machine::ExplorationBudget::default()),
        )
        .expect("fixture is explored");

        assert_eq!(m.state_vars, ["Q"], "the keeper is the state coordinate");
        assert_eq!(
            m.combinational
                .iter()
                .map(|(n, _)| n.as_str())
                .collect::<Vec<_>>(),
            ["W"],
            "the exposed combinational node is the other coordinate",
        );
        assert!(
            !m.explored.order.is_empty(),
            "the fixture explores at least one state"
        );
        for node in &m.explored.order {
            for (name, delta) in m.deltas.iter().chain(&m.combinational) {
                assert!(
                    node.vars().iter().any(|v| v == name),
                    "coordinate {name} has no column at {node:?}",
                );
                assert_eq!(
                    node.value_of(name.as_str()),
                    delta.evaluate_fast(node),
                    "coordinate {name}'s column disagrees with its δ at {node:?}",
                );
            }
        }
    }

    #[test]
    fn the_field_readers_return_the_coordinate_columns() {
        // `output_value` and `exposed_value` read a coordinate's column whichever half it belongs to.
        // Pinned over the fixture's whole explored set as `(A, B, Q, W)`: the keeper holds through the
        // two single-input vectors — hence each of them twice, once per held value — is set at `A·B`
        // and cleared at neither input, while the exposure tracks `A·B`.
        let cell = coordinate_fixture();
        let builder = espresso_logic::sync_bdd_builder!();
        let bdds = crate::model::build_signal_bdds(&cell, &builder);
        let m = super::Machine::build(
            &cell,
            &bdds,
            super::Exploration::Fresh(&crate::logic::machine::ExplorationBudget::default()),
        )
        .expect("fixture is explored");

        let mut read: Vec<(bool, bool, bool, bool)> = m
            .explored
            .order
            .iter()
            .map(|node| {
                let of = |pin: &str| {
                    node.value_of(pin)
                        .expect("every input is fixed in an explored state")
                };
                (
                    of("A"),
                    of("B"),
                    m.output_value("Q", node)
                        .expect("the keeper resolves at every explored state"),
                    m.exposed_value("W", node)
                        .expect("the exposure resolves at every explored state"),
                )
            })
            .collect();
        read.sort();
        assert_eq!(
            read,
            [
                (false, false, false, false),
                (false, true, false, false),
                (false, true, true, false),
                (true, false, false, false),
                (true, false, true, false),
                (true, true, true, true),
            ],
        );
    }

    /// One constraint as a comparable descriptor: its kind, the pins that kind names with the edge each
    /// makes, and the victim nodes it probes. A symmetric separation reads the same either way round, so
    /// its two ends are sorted rather than recorded as constrained and related — which is the very thing
    /// the selections below must not depend on. The probed state is left out: these tests compare WHICH
    /// constraints a selection returns, and the selections of one cell answer over the same states.
    fn describe(c: &Constraint) -> String {
        let end = |pin: &Symbol, edge: Edge| format!("{pin}/{}", VectorValue::from(edge));
        let head = match &c.kind {
            ConstraintKind::SetupHold { clock, clock_edge } => format!(
                "setup_hold {} around {}",
                end(&c.pin, c.pin_edge),
                end(clock, *clock_edge),
            ),
            ConstraintKind::NonSeq { other, other_edge } => {
                let mut ends = [end(&c.pin, c.pin_edge), end(other, *other_edge)];
                ends.sort();
                format!("non_seq {} {}", ends[0], ends[1])
            }
            ConstraintKind::MinPulseWidth => format!("min_pulse_width {}", end(&c.pin, c.pin_edge)),
        };
        let nodes: Vec<String> = c.nodes.iter().map(|v| v.node.to_string()).collect();
        format!("{head} over {}", nodes.join(","))
    }

    /// The constraints a cell generated, as the set of their descriptors.
    fn constrained(cell: &AnalysedCell) -> BTreeSet<String> {
        cell.constraints.iter().map(describe).collect()
    }

    /// A descriptor set from the descriptors themselves, for stating an expectation.
    fn set<const N: usize>(descriptors: [&str; N]) -> BTreeSet<String> {
        descriptors.iter().map(|d| (*d).to_owned()).collect()
    }

    /// The cross-coupled mutex, its inputs declared in `inputs` order and its constraint arcs selected
    /// by `selection`.
    fn mutex(inputs: &str, selection: &str) -> String {
        format!(
            r#"
[[cell]]
name = "MUT"
inputs = [{inputs}]
constraint_arcs = {selection}
[cell.outputs]
Qa = "!Qb * A"
Qb = "!Qa * B"
"#
        )
    }

    /// A master-slave DFF with `CLK` declared a clock, its constraint arcs selected by `selection`.
    fn dff(selection: &str) -> String {
        format!(
            r#"
[[cell]]
name = "DFF"
inputs = ["CLK", "D"]
clock = ["CLK"]
constraint_arcs = {selection}
[cell.internal]
M = "!CLK*D + CLK*M"
[cell.outputs]
Q = "CLK*M + !CLK*Q"
"#
        )
    }

    /// The one separation the mutex generates: neither request is a declared clock, so the pair whose
    /// simultaneous assertion rings is symmetric. Both requests rise — the ring is reached FROM the idle
    /// state — and it endangers both grants.
    const MUTEX_SEPARATION: &str = "non_seq A/R B/R over Qa,Qb";

    #[test]
    fn either_end_of_a_symmetric_separation_selects_it() {
        // A and B are equals here, so naming either names the separation that holds them apart, and the
        // two selections return the same one.
        //
        // What differs between them is the release pulse each request carries of its own: dropping a
        // granted request opens a cascade whose width decides which request ends up granted, so A↓ and
        // B↓ are a minimum pulse width each, over the same grant pair. A pulse names ONE pin, so it
        // answers only to the selection naming that pin.
        assert_eq!(
            constrained(&analyse_one(&mutex(r#""A", "B""#, r#"["A"]"#))),
            set([MUTEX_SEPARATION, "min_pulse_width A/F over Qa,Qb"]),
        );
        assert_eq!(
            constrained(&analyse_one(&mutex(r#""A", "B""#, r#"["B"]"#))),
            set([MUTEX_SEPARATION, "min_pulse_width B/F over Qa,Qb"]),
        );
    }

    #[test]
    fn the_input_declaration_order_does_not_move_a_symmetric_separation() {
        // The same cell, its two inputs declared the other way round: a separation is between the pins,
        // so which of them the spec happens to declare first is nothing the constraint is about. One
        // selection, naming A, answers the same in both spellings.
        let declared = constrained(&analyse_one(&mutex(r#""A", "B""#, r#"["A"]"#)));
        let reversed = constrained(&analyse_one(&mutex(r#""B", "A""#, r#"["A"]"#)));
        assert_eq!(declared, reversed);
        assert!(
            declared.contains(MUTEX_SEPARATION),
            "naming A selects the separation whichever order the inputs are declared in, got {declared:?}",
        );
    }

    #[test]
    fn a_directed_separation_is_selected_by_its_data_pin() {
        // CLK is declared a clock, so the CLK/D race is DIRECTED: D is the data, held around CLK. Naming
        // D asks for what D is subject to, which is that separation. The flop's pulses are on CLK alone —
        // a clock pulse too narrow to carry the master through to the slave — so naming D brings back no
        // minimum pulse width.
        let data = constrained(&analyse_one(&dff(r#"["D"]"#)));
        assert!(!data.is_empty(), "the flop's CLK/D race is constrained");
        assert!(
            data.iter().all(|c| c.starts_with("setup_hold D/")),
            "naming the data pin selects the separations it is held by, got {data:?}",
        );
    }

    #[test]
    fn a_declared_clock_selects_its_pulse_width_and_not_the_separation() {
        // Naming CLK asks for what CLK is itself subject to: the width of its own pulse. A rise pulse
        // opens the slave, so a short one leaves Q where it was rather than at M; a fall pulse opens the
        // master onto D and only the closing rise walks that into Q, so a short one leaves both. Those
        // two widths are the whole of what CLK gets — the CLK/D separation is what D is held by, not what
        // CLK is, so it is not here.
        let clock = constrained(&analyse_one(&dff(r#"["CLK"]"#)));
        assert_eq!(
            clock,
            set([
                "min_pulse_width CLK/R over Q",
                "min_pulse_width CLK/F over Q,M",
            ]),
        );
        // The flop has two input pins, so between them the two selections name every constraint the cell
        // generates: selecting narrows what comes back and loses nothing no pin asks for.
        let data = constrained(&analyse_one(&dff(r#"["D"]"#)));
        assert_eq!(
            data.union(&clock).cloned().collect::<BTreeSet<String>>(),
            constrained(&analyse_one(&dff("true"))),
        );
    }
}
