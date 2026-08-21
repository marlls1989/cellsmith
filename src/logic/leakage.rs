//! Static leakage states for Cadence Liberate `define_leakage`, one per **fully-initialised reachable
//! stable state** of the machine exploration (see `machine::explore`).
//!
//! A cell leaks differently in each state it can rest in, and two rest states can share an input
//! assignment while differing in what the cell holds — a bistable's whole point — so the state, not the
//! input vector, is the unit. Each one records the full machine state it rests at, the levels every
//! output and every exposed internal node holds there, that state's input assignment, and the
//! prevector.
//!
//! The full state is what identifies a rest state: two states a leakage block renders identically
//! differ only in a node no column of the block carries, so a report of that conflation names their
//! `state` minterms. The prevector is the model's path into the state — the exploration's walk over
//! input assignments, which primes the internal nodes on the way — and is what
//! [`LeakageState::input_forced`] reads.
//!
//! Only fully-initialised states qualify, per `Machine::arc_eligible`: a state carrying an
//! uninitialised state variable is at an unknown state, and nothing static can be concluded from it.
//! Every level resolves at such a state by construction, so a leakage state is never partial.

use espresso_logic::bdd::{Brand, ManagerCell};
use espresso_logic::{Minterm, Symbol};

use crate::logic::analysis::Machine;
use crate::logic::arcs::HeldLevel;

/// The levels one rest state holds, keyed by name: every output pin's level in `cell.outputs` order,
/// then every exposed internal node's level in the machine's exposure order.
///
/// A rest state is a single settled point of the machine — the block stating it measures no transition —
/// so a node holds ONE level there, and the type carries one. There is no second end for a level to be
/// read at.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct RestLevels {
    pub(crate) outputs: Vec<HeldLevel>,
    pub(crate) exposed: Vec<HeldLevel>,
}

impl RestLevels {
    /// The levels every output and exposed node of `m`'s cell holds at `node`, the state it rests in.
    ///
    /// TOTAL, never partial: a rest state is one `Machine::arc_eligible` admits — [`derive`] samples only
    /// those — and at such a node every state column is determinate. An output or an exposed node is
    /// either a state variable, which `arc_eligible` requires to be defined, or a combinational signal
    /// whose support lies within the inputs plus the state variables (minimise invariant I3, asserted in
    /// `Machine::build` at `analysis.rs`), so every level resolves.
    pub(crate) fn at<B: Brand, C: ManagerCell>(
        m: &Machine<B, C>,
        node: &Minterm<Symbol>,
    ) -> RestLevels {
        RestLevels {
            outputs: m
                .cell
                .outputs
                .iter()
                .map(|o| {
                    let level = m
                        .output_value(&o.name, node)
                        .expect("every output is defined at a fully-initialised rest state");
                    HeldLevel {
                        node: o.name.clone(),
                        level,
                    }
                })
                .collect(),
            exposed: m
                .exposed
                .iter()
                .map(|exposed| {
                    let level = m
                        .exposed_value(exposed.as_str(), node)
                        .expect("every exposed node is defined at a fully-initialised rest state");
                    HeldLevel {
                        node: exposed.clone(),
                        level,
                    }
                })
                .collect(),
        }
    }
}

/// One static leakage state: a fully-initialised reachable stable state of the machine.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LeakageState {
    /// The state's primary-input assignment (inputs are always fully fixed at a node): `state`
    /// projected onto `cell.inputs`.
    pub(crate) inputs: Minterm<Symbol>,
    /// The full machine state the cell rests at, over the input AND state-variable columns. Two rest
    /// states agreeing on `inputs` and on `levels` still differ here — in a state variable no leakage
    /// column names — which is what a conflation report has to point at.
    pub(crate) state: Minterm<Symbol>,
    /// The levels the cell holds at `state`: every output's settled value and every exposed internal
    /// node's level, one apiece — the rest state is the single point they are all read at.
    pub(crate) levels: RestLevels,
    /// The prevector: the input-assignment sequence that drives the cell — its internal nodes included
    /// — into this state.
    pub(crate) prevector: Vec<Minterm<Symbol>>,
}

impl LeakageState {
    /// Whether the inputs alone drive the cell into this state, which its prevector being a single step
    /// states.
    ///
    /// [`Explored::path_to`](super::machine::Explored::path_to) seeds the chain with the node itself and
    /// then walks predecessors back to a start, so a chain of one is a node with no predecessor —
    /// exactly one of [`Explored::seeds`](super::machine::Explored::seeds). Reaching it primes nothing:
    /// settling from the input assignment lands there.
    pub fn input_forced(&self) -> bool {
        self.prevector.len() == 1
    }
}

/// Derive the cell's static leakage states: every reachable stable state `Machine::arc_eligible`
/// admits, in exploration order. `Explored::order` holds each reachable state once, so this is one
/// leakage state per rest state of the cell, each carrying the BFS path that reaches it.
pub fn derive<B: Brand, C: ManagerCell>(m: &Machine<B, C>) -> Vec<LeakageState> {
    m.explored
        .order
        .iter()
        .filter(|node| m.arc_eligible(node))
        .map(|node| {
            let inputs = node.project_to(&m.cell.inputs);
            // Total, never partial: [`RestLevels::at`] rests its totality on the same `arc_eligible`
            // states the filter above keeps, so every output and every exposed node resolves here.
            let levels = RestLevels::at(m, node);
            let prevector = m.explored.path_to(node, &m.cell.inputs);
            LeakageState {
                inputs,
                state: node.clone(),
                levels,
                prevector,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use espresso_logic::Symbol;

    use super::{LeakageState, RestLevels};
    use crate::logic::arcs::HeldLevel;
    use crate::model::analyse_one as analyse;

    /// A rest state a cell's leakage states record: the A and B inputs it fixes, and the Q level the
    /// cell resolves there.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    struct RestState {
        a: bool,
        b: bool,
        q: bool,
    }

    /// The rest states a cell's leakage states record, sorted — without the paths into them.
    fn states(cell: &crate::model::AnalysedCell) -> Vec<RestState> {
        let mut v: Vec<RestState> = cell
            .leakage
            .iter()
            .map(|l| RestState {
                a: l.inputs.value_of("A").expect("A fixed at a rest state"),
                b: l.inputs.value_of("B").expect("B fixed at a rest state"),
                q: l.levels
                    .outputs
                    .iter()
                    .find(|h| h.node.as_str() == "Q")
                    .expect("Q resolved at a rest state")
                    .level,
            })
            .collect();
        v.sort();
        v
    }

    #[test]
    fn c_element_rests_in_both_hold_states() {
        // C2 (2-input Muller-C): every reachable rest state, not just the two forced covers. The two
        // hold inputs (A≠B) each rest at BOTH Q levels — which is the whole reason a leakage state is
        // the state and not the input vector — while the forcing inputs rest at one each.
        let cell = analyse(
            r#"
[[cell]]
name = "C2"
inputs = ["A", "B"]
[cell.outputs]
Q = "A*B + Q*(A+B)"
"#,
        );
        assert_eq!(
            states(&cell),
            vec![
                RestState {
                    a: false,
                    b: false,
                    q: false
                }, // A=0,B=0 forces Q=0
                RestState {
                    a: false,
                    b: true,
                    q: false
                }, // A=0,B=1 holds, reached from Q=0
                RestState {
                    a: false,
                    b: true,
                    q: true
                }, //          … and from Q=1
                RestState {
                    a: true,
                    b: false,
                    q: false
                }, // A=1,B=0 holds, reached from Q=0
                RestState {
                    a: true,
                    b: false,
                    q: true
                }, //          … and from Q=1
                RestState {
                    a: true,
                    b: true,
                    q: true
                }, // A=1,B=1 forces Q=1
            ],
            "leakage states: {:?}",
            cell.leakage,
        );

        // Each hold state is primed by a prevector that walks in from the forcing input it kept.
        let held_high = cell
            .leakage
            .iter()
            .find(|l| {
                l.inputs.value_of("A") == Some(true)
                    && l.inputs.value_of("B") == Some(false)
                    && l.levels.outputs
                        == vec![HeldLevel {
                            node: Symbol::from("Q"),
                            level: true,
                        }]
            })
            .expect("A=1,B=0 holding Q=1");
        assert!(
            held_high.prevector.len() > 1,
            "a hold state is reached from elsewhere, so its prevector walks: {:?}",
            held_high.prevector,
        );
    }

    #[test]
    fn and2_seeds_the_full_input_square() {
        // Purely combinational AND2: the off-set is maximised, so every one of the four input vectors is
        // a seed, each with Y resolved to A&&B.
        let cell = analyse(
            r#"
[[cell]]
name = "AND2"
inputs = ["A", "B"]
[cell.outputs]
Y = "A*B"
"#,
        );
        let leak = &cell.leakage;
        assert_eq!(
            leak.len(),
            4,
            "expected the full 2-input square, got {leak:?}"
        );
        for l in leak {
            let a = l.inputs.value_of("A").expect("A fixed at a rest state");
            let b = l.inputs.value_of("B").expect("B fixed at a rest state");
            assert_eq!(l.levels.outputs.len(), 1, "AND2 has a single output");
            assert_eq!(l.levels.outputs[0].node.as_str(), "Y");
            assert_eq!(
                l.levels.outputs[0].level,
                a && b,
                "Y == A&&B at every rest state"
            );
            // A combinational cell holds nothing, so the inputs alone drive it into every one of its
            // rest states and each is its own start.
            assert!(l.input_forced(), "no walk is needed to reach {l:?}");
        }
    }

    #[test]
    fn dff_rests_at_every_fully_initialised_state() {
        // Rising-edge DFF: the eight rest states are CLK and D free over the master/slave pair — at
        // CLK=1 the master holds what Q holds, at CLK=0 it follows D and Q holds. A state that leaves a
        // latch uninitialised is at an unknown state and is not one of them, so Q resolves everywhere.
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
        let leak = &cell.leakage;
        assert_eq!(leak.len(), 8, "expected eight rest states, got {leak:?}");
        for l in leak {
            // Q is the only output and it always resolves: no state is emitted partially, and the
            // internal master M is never named among the outputs.
            assert_eq!(
                l.levels.outputs.len(),
                1,
                "every output resolves at a rest state: {l:?}"
            );
            assert_eq!(l.levels.outputs[0].node.as_str(), "Q");
            assert!(
                l.inputs.value_of("CLK").is_some() && l.inputs.value_of("D").is_some(),
                "inputs are fully fixed at a rest state: {l:?}"
            );
        }
    }

    #[test]
    fn an_exposed_master_is_measured_at_every_rest_state() {
        // The same DFF with its master exposed. Exposure keeps M as a machine coordinate of the arc
        // view, so that view's leakage states measure M's level beside Q's, each read at the state it
        // rests in — the level the state's own M column fixes.
        let cell = analyse(
            r#"
[[cell]]
name = "DFF"
inputs = ["CLK", "D"]
expose = ["M"]
[cell.internal]
M = "!CLK*D + CLK*M"
[cell.outputs]
Q = "CLK*M + !CLK*Q"
"#,
        );
        let leak = &cell.arc_view().leakage;
        assert_eq!(leak.len(), 8, "expected eight rest states, got {leak:?}");
        for l in leak {
            let m = l.state.value_of("M").expect("M fixed at a rest state");
            // The annotation is the claim: a rest state carries one level per node, and the exposed
            // master's is the one its own state column fixes.
            let levels: &RestLevels = &l.levels;
            assert_eq!(
                levels.exposed,
                vec![HeldLevel {
                    node: Symbol::from("M"),
                    level: m,
                }],
                "the exposed master is measured at the level the state fixes: {l:?}",
            );
        }

        // A state the cell HOLDS is walked into: at CLK=1,D=0 the pair still carries the 1 it captured
        // on an earlier edge, which no input assignment establishes on its own.
        let held_high = leak
            .iter()
            .find(|l| {
                l.inputs.value_of("CLK") == Some(true)
                    && l.inputs.value_of("D") == Some(false)
                    && l.state.value_of("M") == Some(true)
            })
            .expect("CLK=1,D=0 holding M=1");
        assert!(
            !held_high.input_forced(),
            "a held state is walked into: {:?}",
            held_high.prevector,
        );
    }

    #[test]
    fn rest_states_sharing_a_condition_keep_their_own_walks() {
        // A dual-clock synchroniser rests in states that differ only in latch levels no condition names,
        // so several of its rest states agree on both `inputs` and `levels` — the pair a `-when` renders.
        // The model keeps them apart by the full `state`, which is what a report of that conflation
        // points at, and the walk into each remains the model datum identifying it: two states under one
        // condition are reached along different walks.
        let cell = analyse(
            r#"
[[cell]]
name = "SYNC"
inputs = ["CLKA", "CLKB", "D"]
clock = ["CLKA", "CLKB"]
[cell.internal]
a1 = "!CLKA*D + CLKA*a1"
a2 = "CLKA*a1 + !CLKA*a2"
b1 = "!CLKB*a2 + CLKB*b1"
[cell.outputs]
Q = "CLKB*b1 + !CLKB*Q"
"#,
        );
        let mut per_when: BTreeMap<String, Vec<&LeakageState>> = BTreeMap::new();
        let mut stated: BTreeMap<String, ()> = BTreeMap::new();
        for l in &cell.leakage {
            let when = format!("{:?}|{:?}", l.inputs, l.levels);
            per_when.entry(when.clone()).or_default().push(l);
            // A walked block renders its walk, so the condition and the walk together are what it
            // states. A walk-free one renders the condition alone and could only collide with another
            // walk-free state under the same `-when` — which cannot happen, since walk-free means the
            // inputs alone drive the cell there and so determine the state.
            let block = format!("{when}|{:?}", l.prevector);
            assert!(
                stated.insert(block.clone(), ()).is_none(),
                "no two leakage states share a condition and a walk, got {block} twice",
            );
        }
        let shared: Vec<&Vec<&LeakageState>> =
            per_when.values().filter(|ls| ls.len() > 1).collect();
        assert!(
            !shared.is_empty(),
            "the fixture rests in several states under one condition, got {:?}",
            per_when
                .iter()
                .map(|(when, ls)| (when, ls.len()))
                .collect::<Vec<_>>(),
        );
        // The states a condition conflates are pairwise distinct machine states: they differ in a latch
        // column the condition does not carry, so `state` is what a report of the conflation names.
        for ls in shared {
            for (i, a) in ls.iter().enumerate() {
                for b in &ls[i + 1..] {
                    assert_ne!(
                        a.state, b.state,
                        "two rest states under one condition are one machine state",
                    );
                }
            }
        }
    }

    #[test]
    fn mutex_rests_in_both_arbitration_outcomes() {
        // Cross-coupled mutex Qa=!Qb*A, Qb=!Qa*B. A=1,B=1 is no forced cover, but the cell does rest
        // there — in whichever one-hot state it arbitrated into — so both outcomes are leakage states,
        // told apart by the prevector that walks in from the side that won.
        let cell = analyse(
            r#"
[[cell]]
name = "MUT"
inputs = ["A", "B"]
[cell.outputs]
Qa = "!Qb*A"
Qb = "!Qa*B"
"#,
        );
        let leak = &cell.leakage;
        assert_eq!(leak.len(), 5, "expected five rest states, got {leak:?}");

        let both_high: Vec<_> = leak
            .iter()
            .filter(|l| {
                l.inputs.value_of("A") == Some(true) && l.inputs.value_of("B") == Some(true)
            })
            .collect();
        assert_eq!(
            both_high.len(),
            2,
            "A=1,B=1 rests in both outcomes, got {both_high:?}",
        );
        for l in &both_high {
            let qa = l
                .levels
                .outputs
                .iter()
                .find(|h| h.node.as_str() == "Qa")
                .expect("Qa resolved")
                .level;
            let qb = l
                .levels
                .outputs
                .iter()
                .find(|h| h.node.as_str() == "Qb")
                .expect("Qb resolved")
                .level;
            assert!(qa ^ qb, "an arbitrated rest state is one-hot: {l:?}");
        }
        // The two share an input assignment and differ only in what the cell holds, so the prevector
        // is the only thing that tells them apart.
        assert_ne!(
            both_high[0].prevector, both_high[1].prevector,
            "the two outcomes are reached along different walks",
        );
    }
}
