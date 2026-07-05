//! The two **detected hazards** of an asynchronous cell: the report types only.
//!
//! Two closely-timed input changes can drive a cell into metastability. Detection
//! ([`super::confluence`]) finds the two shapes that risk takes and reports each here; a
//! [`super::confluence::Constraint`] is then *generated* from a detected hazard to specify the timing
//! separation that removes it. A hazard is detected; a constraint is generated — never the reverse.
//!
//! - An [`OrderDependence`] hazard: the settled state depends on which of two input edges lands first
//!   (non-confluence). Detected when two settle orders diverge and the divergence interacts with the
//!   racing pair in the immediate combinational neighbourhood.
//! - An [`Oscillation`] hazard: two simultaneous input edges (or, degenerately, a single toggle) drive
//!   the state into a periodic, non-settling cycle rather than a fixpoint. Detected when
//!   [`super::machine::settle_or_cycle`] returns the cycle instead of settling.
//!
//! Both are *detected* during the state-space exploration in [`super::confluence`], never by
//! enumerating state assignments (an undefined state variable simply means uninitialised). Metastability
//! is the shared physical risk both create — the reason a constraint is generated — not a type and not a
//! synonym for oscillation. This module carries only the resulting report types.
//!
//! **Implementation note:** deduplication is [`super::confluence`]'s job, not this module's.
//! [`OrderDependence`] is keyed by the unordered `(pin,edge)|(pin,edge)` pair, keeping the min
//! `(prevector.len, discovered)` representative; [`Oscillation`] is keyed by `group|condition`,
//! first-insertion-wins (earliest reachable state, by exploration order), with every colliding pair-probe
//! [`Race`] appended rather than dropped. `discovered` (on both [`Race`] and [`OrderDependence`]) is the
//! probed state's index in exploration order — the determinism token that reproduces that tie-break and,
//! downstream, [`super::confluence::constrain`]'s own constraint dedup. See `hazard-detection.md` for the
//! concept.

use espresso_logic::{Minterm, Symbol};

use crate::logic::arcs::Edge;

/// One detected **oscillation hazard** of a cell: the oscillating state variables, the primary-input
/// condition under which they oscillate, and the competing order-of-arrival outcomes (if any).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Oscillation {
    /// The oscillating state variables, in signal declaration order.
    pub group: Vec<Symbol>,
    /// Primary-input condition under which the group oscillates, as a full input assignment.
    pub condition: Minterm<Symbol>,
    /// The competing stable states — each a group-projected minterm (group order), sorted for
    /// determinism.
    pub stable: Vec<Minterm<Symbol>>,
    /// One [`Race`] per pair-probe observation of this oscillation — the racing pins/edges and prevector
    /// a generated constraint needs. A single-input-toggle observation appends none. Colliding pair
    /// observations append to the surviving entry rather than dropping, so this stays in bijection with
    /// the constraint key.
    pub races: Vec<Race>,
}

/// One pair-probe observation of an [`Oscillation`]: the two racing pins and their edges, the prevector
/// into the probed state, and the state's exploration-order index (the constraint dedup tie-break token).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Race {
    pub x: Symbol,
    pub x_edge: Edge,
    pub y: Symbol,
    pub y_edge: Edge,
    /// The prevector: the input-assignment path that drives every state variable into the probed state
    /// (each node projected onto the inputs).
    pub prevector: Vec<Minterm<Symbol>>,
    /// Index of the probed state in exploration order — the determinism token used to reproduce the
    /// dedup tie-break (min by `(prevector.len, discovered)`).
    pub discovered: usize,
}

/// One detected **order-dependent hazard** of a cell: two input edges whose settle order changes the
/// settled state (non-confluence). Reported symmetrically to [`Oscillation`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrderDependence {
    pub x: Symbol,
    pub x_edge: Edge,
    pub y: Symbol,
    pub y_edge: Edge,
    /// Primary-input condition under which the pair races, as a full input assignment.
    pub condition: Minterm<Symbol>,
    /// The state variables that diverge between the two settle orders, in signal declaration order.
    pub group: Vec<Symbol>,
    /// The competing settled states — each a group-projected minterm (group order), sorted for
    /// determinism.
    pub stable: Vec<Minterm<Symbol>>,
    /// The prevector: the input-assignment path that drives every state variable into the probed state.
    pub prevector: Vec<Minterm<Symbol>>,
    /// Index of the probed state in exploration order — the determinism token used to reproduce the
    /// dedup tie-break (min by `(prevector.len, discovered)`).
    pub discovered: usize,
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

impl OrderDependence {
    /// The condition as a Boolean product of literals (`A*B`, `!R*S`, …).
    pub fn condition_str(&self) -> String {
        crate::logic::literals_str(&self.condition)
    }

    /// A competing settled state as a brace-wrapped literal product (`{Q=1}`).
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
        assert_eq!(arb.len(), 1, "exactly one oscillation hazard");
        let a = &arb[0];
        assert_eq!(a.group, ["Qa", "Qb"]);
        assert_eq!(a.condition_str(), "A*B");
        // Exactly one pair-probe race backs the oscillation (the A*B co-assertion), carrying the A/B
        // pins the generated constraint needs.
        assert_eq!(a.races.len(), 1, "one pair-probe race, got {:?}", a.races);
        let race = &a.races[0];
        assert!(
            [race.x.as_str(), race.y.as_str()]
                .iter()
                .all(|p| *p == "A" || *p == "B"),
            "the race is between A and B, got {race:?}"
        );
        // Competing stable states: Qa high / Qb low, and the mirror.
        assert_eq!(a.stable.len(), 2);
        let states: BTreeSet<String> = a.stable.iter().map(Oscillation::state_str).collect();
        assert_eq!(
            states,
            ["{Qa=1, Qb=0}".to_string(), "{Qa=0, Qb=1}".to_string()]
                .into_iter()
                .collect()
        );
        // A mutex detects no order-dependent hazard (its grant divergence is latch-filtered).
        assert!(
            cell.order_dependence.is_empty(),
            "a mutex detects no order-dependent hazard, got {:?}",
            cell.order_dependence
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
        // The C-element is order-dependent (A↓ racing B↑), so it detects an order-dependent hazard.
        assert!(
            !cell.order_dependence.is_empty(),
            "a C-element detects an order-dependent hazard"
        );
    }

    #[test]
    fn non_mutual_sr_is_not_oscillation() {
        // These SR functions each reference only their own state (no mutual edge), so no oscillation.
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
        assert!(cell.order_dependence.is_empty());
    }
}
