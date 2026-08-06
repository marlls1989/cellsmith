//! Static leakage states for Cadence Liberate `define_leakage`, one per **fully-initialised reachable
//! stable state** of the machine exploration (see [`machine::explore`](super::machine::explore)).
//!
//! A cell leaks differently in each state it can rest in, and two rest states can share an input
//! assignment while differing in what the cell holds — a bistable's whole point — so the state, not the
//! input vector, is the unit. Each one records its input assignment, every output's settled value, and
//! the prevector that drives the cell into it: the input-assignment sequence that primes the internal
//! nodes, which is what tells two states apart at the same inputs.
//!
//! Only fully-initialised states qualify, per `Machine::arc_eligible`: a state carrying an
//! uninitialised state variable is at an unknown state, and nothing static can be concluded from it.
//! Every output resolves at such a state by construction, so a leakage state is never partial.

use espresso_logic::bdd::{Brand, ManagerCell};
use espresso_logic::{Minterm, Symbol};

use crate::logic::analysis::Machine;

/// One static leakage state: a fully-initialised reachable stable state of the machine.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct LeakageState {
    /// The state's primary-input assignment (inputs are always fully fixed at a node), projected onto
    /// `cell.inputs`.
    pub inputs: Minterm<Symbol>,
    /// EVERY output's settled value at the state, in `cell.outputs` order.
    pub outputs: Vec<(Symbol, bool)>,
    /// The prevector: the input-assignment sequence that drives the cell — its internal nodes included
    /// — into this state.
    pub prevector: Vec<Minterm<Symbol>>,
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
            // Total, never partial: at a fully-initialised state an output is either a state variable,
            // which `arc_eligible` requires to be defined, or a combinational signal whose support lies
            // within the inputs plus the state variables — resolved either way.
            let outputs = m
                .cell
                .outputs
                .iter()
                .map(|o| {
                    let v = m
                        .output_value(&o.name, node)
                        .expect("every output is defined at a fully-initialised state");
                    (o.name.clone(), v)
                })
                .collect();
            let prevector = m.explored.path_to(node, &m.cell.inputs);
            LeakageState {
                inputs,
                outputs,
                prevector,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use espresso_logic::Symbol;

    use crate::model::analyse_one as analyse;

    /// The (A, B, Q) triples a cell's leakage states record, sorted — the rest states, without the
    /// paths into them.
    fn states(cell: &crate::model::AnalysedCell) -> Vec<(bool, bool, bool)> {
        let mut v: Vec<(bool, bool, bool)> = cell
            .leakage
            .iter()
            .map(|l| {
                (
                    l.inputs.value_of("A").expect("A fixed at a rest state"),
                    l.inputs.value_of("B").expect("B fixed at a rest state"),
                    l.outputs
                        .iter()
                        .find(|(n, _)| n.as_str() == "Q")
                        .expect("Q resolved at a rest state")
                        .1,
                )
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
                (false, false, false), // A=0,B=0 forces Q=0
                (false, true, false),  // A=0,B=1 holds, reached from Q=0
                (false, true, true),   //          … and from Q=1
                (true, false, false),  // A=1,B=0 holds, reached from Q=0
                (true, false, true),   //          … and from Q=1
                (true, true, true),    // A=1,B=1 forces Q=1
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
                    && l.outputs == vec![(Symbol::from("Q"), true)]
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
            assert_eq!(l.outputs.len(), 1, "AND2 has a single output");
            assert_eq!(l.outputs[0].0.as_str(), "Y");
            assert_eq!(l.outputs[0].1, a && b, "Y == A&&B at every rest state");
            // A combinational cell holds nothing, so every state is its own start.
            assert_eq!(l.prevector.len(), 1, "no walk is needed to reach {l:?}");
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
                l.outputs.len(),
                1,
                "every output resolves at a rest state: {l:?}"
            );
            assert_eq!(l.outputs[0].0.as_str(), "Q");
            assert!(
                l.inputs.value_of("CLK").is_some() && l.inputs.value_of("D").is_some(),
                "inputs are fully fixed at a rest state: {l:?}"
            );
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
                .outputs
                .iter()
                .find(|(n, _)| n.as_str() == "Qa")
                .expect("Qa resolved")
                .1;
            let qb = l
                .outputs
                .iter()
                .find(|(n, _)| n.as_str() == "Qb")
                .expect("Qb resolved")
                .1;
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
