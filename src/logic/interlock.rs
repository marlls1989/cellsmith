//! Oscillation / metastability: the report type only.
//!
//! Metastability is the **periodic oscillation of the state** under an input change probed from a
//! **reachable** state (primarily a **simultaneous change of ≥2 inputs**, e.g. a mutex's requests
//! co-asserting). It is detected during the state-space exploration in [`super::confluence`], where
//! [`super::machine::settle`] revisits a non-fixpoint state — never by enumerating state assignments
//! (an undefined state variable simply means uninitialised).
//!
//! The detection lives with the exploration; this module carries only the resulting [`Oscillation`]
//! report type.

use espresso_logic::{Minterm, Symbol};

/// One metastable (oscillation) condition of a cell: the oscillating state variables, the primary-input
/// condition under which they oscillate, and the competing order-of-arrival outcomes (if any).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Oscillation {
    /// The oscillating state variables, in signal declaration order.
    pub group: Vec<Symbol>,
    /// Primary-input condition under which the group is metastable, as a full input assignment.
    pub condition: Minterm<Symbol>,
    /// The competing stable states — each a group-projected minterm (group order), sorted for
    /// determinism.
    pub stable: Vec<Minterm<Symbol>>,
}

impl Oscillation {
    /// The condition as a Boolean product of literals (`A*B`, `!R*S`, …).
    pub fn condition_str(&self) -> String {
        crate::logic::literals_str(&self.condition)
    }

    /// A competing stable state as a brace-wrapped literal product (`{Qa=1, Qb=0}`).
    pub fn state_str(state: &Minterm<Symbol>) -> String {
        format!("{{{}}}", crate::logic::fixed_pairs(state, &[]).join(", "))
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;
    use crate::model::analyse_one as analyse;

    #[test]
    fn mutex_has_one_oscillation_point() {
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
        let arb = &cell.oscillation;
        assert_eq!(arb.len(), 1, "exactly one metastable condition");
        let a = &arb[0];
        assert_eq!(a.group, ["Qa", "Qb"]);
        assert_eq!(a.condition_str(), "A*B");
        // Competing stable states: Qa high / Qb low, and the mirror.
        assert_eq!(a.stable.len(), 2);
        let states: BTreeSet<String> = a.stable.iter().map(Oscillation::state_str).collect();
        assert_eq!(
            states,
            ["{Qa=1, Qb=0}".to_string(), "{Qa=0, Qb=1}".to_string()]
                .into_iter()
                .collect()
        );
    }

    #[test]
    fn c_element_self_hold_is_not_oscillation() {
        // A C-element is bistable in the hold region, but that is self-feedback, not mutual coupling.
        let cell = analyse(
            r#"
[[cell]]
name = "C2"
inputs = ["A", "B"]
[cell.outputs]
Q = "A*B + Q*(A+B)"
"#,
        );
        assert!(cell.oscillation.is_empty());
    }

    #[test]
    fn non_mutual_sr_is_not_oscillation() {
        // These SR functions each reference only their own state (no mutual edge), so no interlock.
        let cell = analyse(
            r#"
[[cell]]
name = "SR"
inputs = ["S", "R"]
[cell.outputs]
Q = "S + Q*!R"
Qn = "R + Qn*!S"
"#,
        );
        assert!(cell.oscillation.is_empty());
    }

    #[test]
    fn combinational_is_not_oscillation() {
        let cell = analyse(
            r#"
[[cell]]
name = "ND2"
inputs = ["A", "B"]
[cell.outputs]
Y = "!(A*B)"
"#,
        );
        assert!(cell.oscillation.is_empty());
    }
}
