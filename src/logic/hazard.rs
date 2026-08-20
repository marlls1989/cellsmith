//! The **detected hazards** of an asynchronous cell: the report record only.
//!
//! Closely-timed input changes can drive a cell into metastability. Detection finds each occasion and
//! reports it here as a [`Hazard`]; a `super::constraint::Constraint` is then *generated* from a
//! detected hazard to specify the timing that removes it. Detection happens first; constraint
//! generation follows from each detected hazard.
//!
//! A hazard is read on two independent axes — what the timing is between, and what the machine then
//! does.
//!
//! [`Cause`] is what the timing is between:
//!
//! - [`Cause::Toggle`] — one input toggled alone, its cascade ringing around the cell's own feedback
//!   instead of settling. The record names that pin and the edge it makes.
//! - [`Cause::Race`] — two inputs toggled together that don't converge. The record names both pins, one
//!   [`PinEdge`](crate::logic::arcs::PinEdge) each with the edge it makes, in the order the probe took
//!   them.
//! - [`Cause::Pulse`] — one signal racing itself: the two edges of a single pin. A **pulse** on input
//!   `p` from a stable state `s` is `p` toggled (the *opening* edge), the cascade that toggle opens left
//!   to run some distance, and `p` toggled back (the *closing* edge) before settling; that distance is
//!   the pulse's **width**. Detection ([`super::width`]) measures it in next-state rounds of the opening
//!   edge's settling trace `t[0..last]`, and closing the pulse after `i` rounds is the **cut** `i` —
//!   a wider pulse is a later cut.
//!
//! [`Outcome`] is what the machine then does:
//!
//! - [`Outcome::Indeterminate`] — it settles, but which state it settles to is not determined.
//! - [`Outcome::Oscillation`] — it never settles: the state walks a periodic cycle instead of reaching a
//!   stable state (a state `x` with `delta(x) == x`). Detected when
//!   `super::machine::settle_or_cycle` returns the cycle instead of settling.
//!
//! The axes are independent, so a hazard is one of the three causes settling indeterminately or not
//! settling at all. One [`Hazard`] carries one (cause, outcome) pair, so a probe that observes both
//! outcomes under one cause files two records carrying that same cause: a mutex's `A↓`, which both
//! settles unpredictably and can fail to settle, is two records rather than one.
//!
//! Detection runs over the state-space exploration in [`super::confluence`] and [`super::width`]. An
//! uninitialised state variable is at an UNKNOWN state — not a value, and not a third one — so no
//! detection runs from a state carrying one. Metastability is the shared physical risk they all create
//! — the reason a constraint is generated. This module carries only the resulting report record.
//!
//! **Implementation note:** a record is filed for every observation, and which of them a block is
//! rendered from is [`crate::emit::arcs_tcl`]'s to decide: the observation naming a maximal set of
//! victim nodes under containment supplies one. `discovered` is the probed state's index in exploration order,
//! and with `Hazard::ordinal` it forms the `(discovered, ordinal)` key that settles the choice between
//! observations dominating equally. See `hazard-detection.md` for the concept.

use std::collections::BTreeMap;

use espresso_logic::{BoolExpr, Minterm, Symbol};

use crate::logic::arcs::{ArcLevels, Edge, PinEdge};

/// What the hazard's timing is between: inputs that don't converge when toggled, one alone or two
/// together, or one signal racing itself.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Cause {
    /// One input toggled alone, its cascade never settling: the pin the probe toggled, with the edge it
    /// makes. It names one edge, so there is nothing for a separation to hold it apart from.
    Toggle { pin: PinEdge },
    /// Two inputs probed together that don't converge: the pair the probe toggled, one [`PinEdge`]
    /// each, in the order it named them.
    Race { pins: [PinEdge; 2] },
    /// One signal racing itself: the two edges of a single pin, bounding a pulse.
    Pulse {
        pin: Symbol,
        /// The OPENING edge of the pulse — rise means the pulse is high, fall low.
        edge: Edge,
    },
}

/// What the machine does under the hazard's timing. The variant is the whole of the classification:
/// everything that differs between the two lives in the shared fields of [`Hazard`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Outcome {
    /// The machine settles, but which state it settles to is not determined.
    Indeterminate,
    /// The machine never settles: the state walks a periodic cycle instead of reaching a stable
    /// state.
    Oscillation,
}

/// One detected hazard: a [`Cause`], the [`Outcome`] it produces, and the observation the generated
/// constraint is built from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hazard {
    pub cause: Cause,
    pub outcome: Outcome,
    /// The state variables the hazard decides — those that diverge across the competing outcomes, or
    /// that oscillate — in signal declaration order.
    pub group: Vec<Symbol>,
    /// The condition the hazard occurs under: the probed state's input projection, which is the standing
    /// input assignment the probed transition happens FROM. The pins a probe toggles are the ones an
    /// emitted block writes as edges, and an edge is not part of the condition it fires under, so this is
    /// the pre-transition assignment under every cause alike — a race's as much as a pulse's.
    pub condition: Minterm<Symbol>,
    /// Where the machine lands when the timing is honoured — separated edges for a race, an adequate
    /// width for a pulse — each state a group-projected minterm (group order).
    ///
    /// How the members relate follows from the cause. A race has a winner, and either winner is a
    /// legitimate result of honouring the timing, so its members are ALTERNATIVES: their order among
    /// themselves carries nothing, and they are held sorted only so the report is deterministic. A pulse
    /// is one signal's two edges, and a transition cannot be a rise and a fall at once, so its members
    /// cannot be reordered: they are a SEQUENCE in causal order, the machine's landing point as the
    /// pulse widens. A `Vec` carries order either way, so the two readings share the type.
    pub settled: Vec<Minterm<Symbol>>,
    /// The prevector: the input-assignment path that drives every state variable into the probed state
    /// (each node projected onto the inputs).
    pub(crate) prevector: Vec<Minterm<Symbol>>,
    /// The levels the cell's outputs hold at the probed state — sampled at the SAME state as
    /// `prevector`, so the pair the constraint carries is consistent.
    pub(crate) levels: ArcLevels,
    /// The level each node the hazard names holds at the PROBED state, by name. Sampled at the same
    /// state as `prevector` and `levels`, and covering every entry of the hazard's `group`, so the
    /// constraint generated from this observation can state the start level of each node it probes.
    pub(crate) node_levels: BTreeMap<Symbol, bool>,
    /// The probed state itself: every input and state variable at the level it holds there. The
    /// prevector reaches it and the levels sample its pins, but only this names the internal nodes no
    /// emitted column carries.
    pub state: Minterm<Symbol>,
    /// Index of the probed state in `ex.order` (the sequential BFS exploration order) — the leading
    /// component of the tie-break between equally dominant observations: the earlier-discovered one is
    /// kept. The exploration is breadth-first, so this already orders the observations by the length of
    /// the walk that reaches them.
    pub(crate) discovered: usize,
}

impl Hazard {
    /// The condition as a product of literals over the fixed inputs of the state the hazard is probed
    /// from (`A & B`, `!R & S`, …).
    pub fn condition(&self) -> BoolExpr {
        crate::logic::condition(&self.condition)
    }

    /// The path into the pre-hazard state: the sequence of input states the machine walks — driving its
    /// hidden state — to reach the state the probe acts on. Last state is the pre-hazard state.
    pub fn path(&self) -> &[Minterm<Symbol>] {
        &self.prevector
    }

    /// The pre-hazard state: the reachable stable state the probe starts from (the path's last input
    /// state).
    pub fn pre_state(&self) -> &Minterm<Symbol> {
        self.prevector
            .last()
            .expect("path_to seeds its chain with the probed node itself")
    }

    /// A fixed rank over the (cause, outcome) cells, so that two hazards can be ordered by which cell
    /// they occupy. A lone toggle and a pair race take the same rank: both are inputs failing to
    /// converge, which is the distinction the rank draws. It is the second component of the
    /// representative tie-break, after `discovered`: two records read from one probed state still pick
    /// a representative deterministically.
    pub(crate) fn ordinal(&self) -> u8 {
        let cause = match self.cause {
            Cause::Toggle { .. } | Cause::Race { .. } => 0,
            Cause::Pulse { .. } => 2,
        };
        let outcome = match self.outcome {
            Outcome::Indeterminate => 0,
            Outcome::Oscillation => 1,
        };
        cause + outcome
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
        let oscillating: Vec<&Hazard> = cell
            .hazards
            .iter()
            .filter(|h| {
                matches!(h.cause, Cause::Toggle { .. } | Cause::Race { .. })
                    && h.outcome == Outcome::Oscillation
            })
            .collect();
        assert_eq!(oscillating.len(), 1, "exactly one oscillation hazard");
        let a = oscillating[0];
        assert_eq!(a.group, ["Qa", "Qb"]);
        // A `when` is the assignment the transition happens FROM, and this ring is A and B toggled
        // together: the pair rises out of the idle state, where neither request is up. (Co-asserted is
        // where it rings, and that is the pair's destination, not its condition.)
        assert_eq!(a.condition().to_string(), "!A & !B");
        // The A*B co-assertion is a pair-probe race, carrying the A/B pins the generated constraint
        // needs.
        let Cause::Race { pins } = &a.cause else {
            panic!("expected a pair-probe race, got {:?}", a.cause)
        };
        assert!(
            pins.iter()
                .all(|r| r.pin.as_str() == "A" || r.pin.as_str() == "B"),
            "the race is between A and B, got {pins:?}"
        );
        // Competing settled states: Qa high / Qb low, and the mirror.
        assert_eq!(a.settled.len(), 2);
        let states: BTreeSet<String> = a
            .settled
            .iter()
            .map(|s| crate::report::State(s).to_string())
            .collect();
        assert_eq!(
            states,
            ["{Qa=1, Qb=0}".to_string(), "{Qa=0, Qb=1}".to_string()]
                .into_iter()
                .collect()
        );
        // A mutex detects no order-dependent hazard (its grant divergence is latch-filtered).
        let order_dependent: Vec<&Hazard> = cell
            .hazards
            .iter()
            .filter(|h| {
                matches!(h.cause, Cause::Race { .. }) && h.outcome == Outcome::Indeterminate
            })
            .collect();
        assert!(
            order_dependent.is_empty(),
            "a mutex detects no order-dependent hazard, got {order_dependent:?}"
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
        assert!(!cell.hazards.iter().any(|h| {
            matches!(h.cause, Cause::Toggle { .. } | Cause::Race { .. })
                && h.outcome == Outcome::Oscillation
        }));
        // The C-element is order-dependent (A↓ racing B↑), so it detects an order-dependent hazard.
        assert!(
            cell.hazards.iter().any(|h| {
                matches!(h.cause, Cause::Toggle { .. } | Cause::Race { .. })
                    && h.outcome == Outcome::Indeterminate
            }),
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
        assert!(!cell.hazards.iter().any(|h| {
            matches!(h.cause, Cause::Toggle { .. } | Cause::Race { .. })
                && h.outcome == Outcome::Oscillation
        }));
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
        assert!(!cell.hazards.iter().any(|h| {
            matches!(h.cause, Cause::Toggle { .. } | Cause::Race { .. })
                && h.outcome == Outcome::Oscillation
        }));
        assert!(!cell.hazards.iter().any(|h| {
            matches!(h.cause, Cause::Toggle { .. } | Cause::Race { .. })
                && h.outcome == Outcome::Indeterminate
        }));
    }
}
