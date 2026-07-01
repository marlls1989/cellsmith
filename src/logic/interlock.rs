//! Arbitration / metastability: the report type.
//!
//! Metastability is the **periodic oscillation of the state** under an input change probed from a
//! **reachable** state — primarily a **simultaneous change of ≥2 inputs** (a mutex's requests
//! co-asserting), detected as an integral part of the state-space exploration in [`super::confluence`]:
//! [`super::machine::settle`] revisiting a non-fixpoint state (see [`super::machine::settle_or_cycle`]).
//! It is never detected by enumerating state assignments: held state is the product of the sequential
//! behaviour, an **undefined state variable simply means uninitialised**, and coercing it to fabricated
//! concrete values manufactures arbitration on states the cell can never reach (the same mistake fixed
//! on the arc side in commit 5a7c302).
//!
//! This module now only carries the report type.

use espresso_logic::{Minterm, Symbol};

/// One metastable (arbitration) condition of a cell: the oscillating state variables, the primary-input
/// condition under which they oscillate, and the competing order-of-arrival outcomes (if any).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Arbitration {
    /// The oscillating state variables, in signal declaration order.
    pub group: Vec<String>,
    /// Primary-input condition under which the group is metastable, as a full input assignment.
    pub condition: Minterm<Symbol>,
    /// The competing stable states — each a group-projected minterm (group order), sorted for
    /// determinism.
    pub stable: Vec<Minterm<Symbol>>,
}

impl Arbitration {
    /// The condition as a Boolean product of literals (`A*B`, `!R*S`, …).
    pub fn condition_str(&self) -> String {
        literals_str(&self.condition)
    }

    /// A competing stable state as a brace-wrapped literal product (`{Qa=1, Qb=0}`).
    pub fn state_str(state: &Minterm<Symbol>) -> String {
        let inner: Vec<String> = state
            .vars()
            .iter()
            .zip(state.iter())
            .filter_map(|(n, v)| v.map(|b| format!("{}={}", n.as_str(), if b { 1 } else { 0 })))
            .collect();
        format!("{{{}}}", inner.join(", "))
    }
}

/// A minterm's fixed values as a product of literals: `A*B`, `!R*S` (in the minterm's variable order).
/// No fixed value ⇒ the tautology `1`.
pub(crate) fn literals_str(m: &Minterm<Symbol>) -> String {
    let lits: Vec<String> = m
        .vars()
        .iter()
        .zip(m.iter())
        .filter_map(|(n, v)| {
            v.map(|b| {
                if b {
                    n.as_str().to_string()
                } else {
                    format!("!{}", n.as_str())
                }
            })
        })
        .collect();
    if lits.is_empty() {
        "1".to_owned()
    } else {
        lits.join("*")
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;
    use crate::model::{parse_spec, AnalysedCell};

    fn analyse(src: &str) -> AnalysedCell {
        parse_spec(src).unwrap().cells.remove(0).analyse().unwrap()
    }

    #[test]
    fn mutex_has_one_arbitration_point() {
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
        let arb = &cell.arbitration;
        assert_eq!(arb.len(), 1, "exactly one metastable condition");
        let a = &arb[0];
        assert_eq!(a.group, ["Qa", "Qb"]);
        assert_eq!(a.condition_str(), "A*B");
        // Competing stable states: Qa high / Qb low, and the mirror.
        assert_eq!(a.stable.len(), 2);
        let states: BTreeSet<String> = a.stable.iter().map(Arbitration::state_str).collect();
        assert_eq!(
            states,
            ["{Qa=1, Qb=0}".to_string(), "{Qa=0, Qb=1}".to_string()]
                .into_iter()
                .collect()
        );
    }

    #[test]
    fn c_element_self_hold_is_not_arbitration() {
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
        assert!(cell.arbitration.is_empty());
    }

    #[test]
    fn non_mutual_sr_is_not_arbitration() {
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
        assert!(cell.arbitration.is_empty());
    }

    #[test]
    fn combinational_is_not_arbitration() {
        let cell = analyse(
            r#"
[[cell]]
name = "ND2"
inputs = ["A", "B"]
[cell.outputs]
Y = "!(A*B)"
"#,
        );
        assert!(cell.arbitration.is_empty());
    }
}
