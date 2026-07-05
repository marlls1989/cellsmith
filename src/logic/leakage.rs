//! Static leakage states for Cadence Liberate `define_leakage`, derived from the **settled seed
//! states** of the machine exploration — the forced on/off-set cover states that initialise the BFS
//! (see [`machine::explore`]) — NOT the full reachable set.
//!
//! Each seed is a settled node: its primary inputs are fully fixed, and every state variable it forces
//! carries a concrete value (unforced ones stay absent). A leakage state records the seed's input
//! assignment together with each output that resolves at it. An output left undefined at a seed (its
//! value still depends on an unresolved state variable) is simply omitted — the seed is still emitted,
//! with defined literals only.

use std::collections::BTreeSet;

use espresso_logic::bdd::{Brand, ManagerCell};
use espresso_logic::{Minterm, Symbol};

use crate::logic::analysis::Machine;

/// One static leakage state: a settled seed of the machine exploration.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct LeakageState {
    /// The seed's primary-input assignment (inputs are always fully fixed at a node), projected onto
    /// `cell.inputs`.
    pub inputs: Minterm<Symbol>,
    /// Each RESOLVED output's settled value at the seed, in `cell.outputs` order.
    pub outputs: Vec<(Symbol, bool)>,
}

/// Derive the cell's static leakage states from the settled BFS seeds. For each seed, keep only the
/// outputs that resolve there (defined literals only — an undefined output is dropped, but the seed is
/// still emitted). Collect into a [`BTreeSet`] for a deterministic, sorted result (`Minterm: Ord`);
/// each seed's fully-fixed input vector makes the states distinct, so this only orders them.
pub(crate) fn derive<B: Brand, C: ManagerCell>(m: &Machine<B, C>) -> Vec<LeakageState> {
    m.explored
        .seeds()
        .map(|node| {
            let inputs = node.project_to(&m.cell.inputs);
            let outputs = m
                .cell
                .outputs
                .iter()
                .filter_map(|o| m.output_value(&o.name, node).map(|v| (o.name.clone(), v)))
                .collect();
            LeakageState { inputs, outputs }
        })
        .collect::<BTreeSet<LeakageState>>()
        .into_iter()
        .collect()
}

#[cfg(test)]
mod tests {
    use espresso_logic::Symbol;

    use crate::model::analyse_one as analyse;

    #[test]
    fn c_element_has_forced_on_and_off_seeds() {
        // C2 (2-input Muller-C): the settled seeds are exactly the two forced covers — inputs A=1,B=1
        // holding Q=1, and A=0,B=0 holding Q=0.
        let cell = analyse(
            r#"
[[cell]]
name = "C2"
inputs = ["A", "B"]
[cell.outputs]
Q = "A*B + Q*(A+B)"
"#,
        );
        let leak = &cell.leakage;
        assert_eq!(
            leak.len(),
            2,
            "expected exactly two leakage states, got {leak:?}"
        );

        let on = leak
            .iter()
            .find(|l| l.inputs.value_of("A") == Some(true) && l.inputs.value_of("B") == Some(true))
            .expect("an A=1,B=1 seed");
        assert_eq!(on.outputs, vec![(Symbol::from("Q"), true)]);
        let off = leak
            .iter()
            .find(|l| {
                l.inputs.value_of("A") == Some(false) && l.inputs.value_of("B") == Some(false)
            })
            .expect("an A=0,B=0 seed");
        assert_eq!(off.outputs, vec![(Symbol::from("Q"), false)]);
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
            let a = l.inputs.value_of("A").expect("A fixed at a seed");
            let b = l.inputs.value_of("B").expect("B fixed at a seed");
            assert_eq!(l.outputs.len(), 1, "AND2 has a single output");
            assert_eq!(l.outputs[0].0.as_str(), "Y");
            assert_eq!(l.outputs[0].1, a && b, "Y == A&&B at every seed");
        }
    }

    #[test]
    fn dff_emits_seeds_even_when_output_undefined() {
        // Rising-edge DFF: the CLK=0 seeds settle the master M but leave Q undefined. Per the
        // defined-literals-only rule they are STILL emitted — with inputs and any resolved output, but
        // never a fabricated Q value.
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
        assert!(!leak.is_empty(), "DFF must still yield leakage states");
        // Q is the only output; a seed that leaves it undefined carries no Q literal (rather than a
        // fabricated value), yet is still present.
        assert!(
            leak.iter()
                .any(|l| l.outputs.iter().all(|(n, _)| n.as_str() != "Q")),
            "expected a seed with Q undefined (CLK=0), emitted without a Q literal"
        );
        // No seed names any output other than Q (the internal master M is never emitted).
        assert!(leak
            .iter()
            .all(|l| l.outputs.iter().all(|(n, _)| n.as_str() == "Q")));
    }

    #[test]
    fn mutex_never_seeds_the_oscillation_input() {
        // Cross-coupled mutex Qa=!Qb*A, Qb=!Qa*B: three settled seeds, none at A=1,B=1 (the input where
        // the oscillation hazard sits is never a seed).
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
        assert_eq!(leak.len(), 3, "expected three leakage states, got {leak:?}");
        assert!(
            leak.iter()
                .all(|l| !(l.inputs.value_of("A") == Some(true)
                    && l.inputs.value_of("B") == Some(true))),
            "the A=1,B=1 input — where the oscillation hazard sits — must never be a seed"
        );
    }
}
