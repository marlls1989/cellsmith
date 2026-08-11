//! Detection of the **pulse-cause hazards**: one signal racing itself, the two edges of a single pin.
//!
//! A **pulse** on an input `p`, applied to a fully-initialised stable state `s`, is `p` toggled (the
//! OPENING edge), the cascade that toggle opens left to run some distance, and `p` toggled back (the
//! CLOSING edge). That distance is the pulse's **width**, counted here in next-state rounds of the
//! [`machine`]: writing `t[0..last]` for the settling trace of the opening toggle
//! (`machine::settle_trace` — `t[0]` the toggle itself, `t[last]` its convergence point), closing the
//! pulse after `i` rounds is the **cut** `i`, which settles `toggle(t[i], [p])`, and a wider pulse is a
//! later cut.
//!
//! The cuts are not peers. The close at cut `last` — the one placed once the opening cascade has reached
//! its convergence point — is the **reference**: after the cell has settled, closing now and closing
//! three days later are the same event, so that close is the behaviour a minimum pulse width is defined
//! RELATIVE TO rather than one outcome among several. Every earlier close is a **candidate**, the
//! narrowest of them the zero-width close, whose outcome is `s` itself. A hazard is a candidate that
//! disagrees with the reference, or a candidate that does not converge.
//!
//! That is the [`Cause::Pulse`] half of the hazard taxonomy, the sibling of [`Cause::Race`] — two
//! signals racing each other — which [`super::confluence`] detects. What the machine then does is the
//! other, independent axis, and this pass files one [`Hazard`] per [`Outcome`] it observes:
//!
//! - [`Outcome::Indeterminate`] — a candidate converges somewhere the reference does not, so which state
//!   the pulse leaves the cell in is decided by its width; the record's `group` is the state variables
//!   the two differ in.
//! - [`Outcome::Oscillation`] — a candidate leaves the machine walking a periodic cycle instead of
//!   reaching a convergence point, and the record's `group` is what that cycle rings over.
//!
//! A pulse showing both files both, sharing the one [`Cause::Pulse`]: they are different phenomena over
//! their own nodes — a ring is not a disagreement between landing points — and each names the nodes its
//! own reading puts at risk.
//!
//! An oscillating candidate is a pulse-cause hazard and is never also filed as a race-cause one. The
//! cause is what the timing is between, and here it is one pin's two edges: the ring is reached by
//! closing the pulse partway through the opening cascade, which no separation between two pins can
//! forbid and only a wide enough pulse can.
//!
//! [`detect`] walks the fully-initialised reachable stable states — the same `Machine::arc_eligible`
//! measurement gate the arc derivation and [`super::confluence`] apply — and measures every candidate
//! against the reference, not the zero-width one alone: a cascade cut in flight reaches states neither
//! the narrowest close nor the reference lands on, and those are outcomes of the same hazard.
//!
//! **Every observation is reported.** One pin's pulse decides different nodes from different states, and
//! on a cell whose internals form a chain those node sets nest — one set per depth the cascade was cut
//! at, every one of them the same pulse seen from further in. Each of them is its own record: this pass
//! states what it observed, and which of those observations a `define_arc` is rendered from — where a
//! probe set another one contains asks nothing that is not already asked — is
//! [`crate::emit::arcs_tcl`]'s to decide.
//!
//! **Implementation notes:** states are probed in parallel and their per-state records concatenated, as
//! in [`super::confluence::detect`]. Concatenation is associative, so the folded result holds the same
//! records however the work was split; nothing reads the order they come out in.

use std::collections::BTreeSet;

use rayon::prelude::*;

use espresso_logic::bdd::{Brand, ManagerCell};
use espresso_logic::{Minterm, Symbol};

use crate::logic::analysis::Machine;
use crate::logic::arcs::ArcLevels;
use crate::logic::confluence::{edge_from, node_levels_at, oscillating_group};
use crate::logic::hazard::{Cause, Hazard, Outcome};
use crate::logic::machine;

/// Why every state value a pulse walk reads is defined, exactly as in [`super::confluence`]: a probe
/// starts from a state `Machine::arc_eligible` admits, and settling from a fully-initialised state
/// leaves every state column determinate, so the whole trajectory — the opening cascade and every cut's
/// settle alike — is total and this message is unreachable.
const DETERMINATE: &str =
    "a settle from a fully-initialised state leaves every state column determinate";

/// Detect a cell's pulse-cause hazards by pulsing every input at every fully-initialised reachable
/// stable state (`Machine::arc_eligible`: a state carrying an uninitialised state variable is at an
/// unknown state, from which nothing can be concluded). Files a [`Hazard`] for every observation it
/// makes — one per (cause, outcome) per probed state — and generates no constraint. Empty for cells whose
/// every candidate close agrees with its reference.
pub fn detect<B: Brand, C: ManagerCell + Send + Sync>(m: &Machine<B, C>) -> Vec<Hazard> {
    // With no memory every coordinate is a function of the inputs alone, so returning `p` to its
    // pre-pulse value returns the whole machine to `s`: a pulse can leave no net effect for its width to
    // decide.
    if m.state_vars.is_empty() {
        return Vec::new();
    }
    // No input-count guard: a pulse-cause hazard relates one pin to ITSELF, so `confluence::detect`'s
    // pair-wise `n < 2` early-out — a race there relates two inputs — is that pass's rule and not this
    // one's.

    let inputs = &m.cell.inputs;
    let ex = &m.explored;
    // Both coordinate halves, stepped together, as every re-walk of this machine steps them.
    let deltas: Vec<machine::Delta<B, C>> = m.coordinate_deltas();

    // The per-state probe body: for one reachable state `s` (its BFS index `discovered`), pulse every
    // input and collect this state's own records. Each state is independent — the parallel unit — and the
    // results merge commutatively in the `reduce` below.
    let per_state = |(discovered, s): (usize, &Minterm<Symbol>)| -> Vec<Hazard> {
        let mut found: Vec<Hazard> = Vec::new();

        // Both depend only on `s`, so they are computed once here and cloned per hazard. Sampling them
        // side by side at the one probed state is what keeps them consistent wherever the record travels:
        // a record carries the levels of the very state its prevector walks to.
        let prevector_s = ex.path_to(s, inputs);
        let levels_s = ArcLevels::at(m, s);

        for p in inputs {
            let opened = machine::toggle(s, &[p.as_str()]);
            // An opening toggle that never settles is the single-toggle oscillation
            // `confluence::detect` already records: the machine never comes to rest for a second edge to
            // be placed against, so there is no pulse here to widen.
            let Ok(trace) = machine::settle_trace(&deltas, &opened) else {
                continue;
            };

            let (rested, earlier) = trace
                .split_last()
                .expect("a settle trace is seeded with the node itself");

            // The REFERENCE: the close at cut `last`, placed once the opening cascade has reached its
            // convergence point.
            //
            // A reference that rings files nothing — neither outcome, on this pin from this state —
            // because there is behaviour here for no candidate to be measured against. Nothing is lost
            // by passing over it: that close is the closing edge ALONE toggled from a convergence
            // point, which is the single-pin race oscillation `confluence::detect` already records. It
            // probes every arc-eligible state of `explored.order` with every input singly; `t[last]` is
            // `settle(toggle(s, [p]))`, which the BFS puts into that very set; and the call here is
            // `settle_or_cycle` over the same `deltas` from that same state.
            let closed = machine::toggle(rested, &[p.as_str()]);
            let Ok(reference) = machine::settle_or_cycle(&deltas, &closed) else {
                continue;
            };

            // The CANDIDATES: every close earlier than the reference's. Cut 0 — the zero-width pulse —
            // is `s` itself by derivation rather than by settling (`toggle` writes only the named
            // input's column, so closing at `t[0]` reproduces `s`, which is a convergence point), and
            // it is the close the generated constraint forbids, so it decides whether there is a hazard
            // at all. The rest are settled here. An opening toggle that comes to rest at once leaves
            // none of them, and cut 0 is then the reference — which agrees with itself, so reading `s`
            // as a candidate below needs no case of its own.
            let candidates: Vec<Result<Minterm<Symbol>, Vec<Minterm<Symbol>>>> = earlier
                .iter()
                .skip(1)
                .map(|cut| {
                    let closed = machine::toggle(cut, &[p.as_str()]);
                    machine::settle_or_cycle(&deltas, &closed)
                })
                .collect();

            // Does some candidate leave `w` where the reference does not? Every state read here is
            // total (see `DETERMINATE`), so this is a comparison of values, not of definedness.
            let diverges = |w: &Symbol| {
                let level = |x: &Minterm<Symbol>| x.value_of(w.as_str()).expect(DETERMINATE);
                let settles_to = level(&reference);
                level(s) != settles_to
                    || candidates
                        .iter()
                        .filter_map(|out| out.as_ref().ok())
                        .any(|out| level(out) != settles_to)
            };
            let diverging: Vec<Symbol> = m
                .state_vars
                .iter()
                .filter(|w| diverges(w))
                .cloned()
                .collect();
            // Every node a candidate rings over. Several candidates of one pulse can ring, and they are
            // the one cause under the one outcome, so they are the one record over the nodes between
            // them.
            let rings: BTreeSet<Symbol> = candidates
                .iter()
                .filter_map(|out| out.as_ref().err())
                .flat_map(|cycle| oscillating_group(cycle, &m.state_vars))
                .collect();
            let ringing: Vec<Symbol> = m
                .state_vars
                .iter()
                .filter(|w| rings.contains(*w))
                .cloned()
                .collect();

            // The two waypoints a pulse wide enough to be measured walks through, in causal order:
            // `t[last]`, where the OPENING edge's own cascade came to rest, and then the reference,
            // where the machine settles once the CLOSING edge is placed on it. The order is
            // load-bearing: a transition cannot be a rise and a fall at once, so a pulse's two edges are
            // necessarily sequential and one waypoint is reached through the other. No candidate is a
            // member — a candidate is what the generated constraint forbids, not somewhere the machine
            // legitimately lands.
            let settled = |group: &[Symbol]| -> Vec<Minterm<Symbol>> {
                vec![rested.project_to(group), reference.project_to(group)]
            };

            let cause = Cause::Pulse {
                pin: p.clone(),
                edge: edge_from(s, p.as_str()),
            };
            // A candidate converges somewhere the reference does not over these nodes, so which state a
            // pulse leaves them in is decided by its width: the hazard proper, empty exactly where every
            // candidate that converges agrees with the reference.
            if !diverging.is_empty() {
                found.push(Hazard {
                    cause: cause.clone(),
                    outcome: Outcome::Indeterminate,
                    condition: s.project_to(inputs),
                    settled: settled(&diverging),
                    node_levels: node_levels_at(s, &diverging),
                    group: diverging,
                    prevector: prevector_s.clone(),
                    levels: levels_s.clone(),
                    state: s.clone(),
                    discovered,
                });
            }
            // A candidate that rings names at least one state variable — the inputs hold through a
            // settle and a combinational coordinate lies on no dependency cycle (see
            // [`super::resolve`]), so a cycle that moved no held coordinate would be a convergence point
            // — hence a non-empty group here is exactly "some candidate did not converge".
            if !ringing.is_empty() {
                found.push(Hazard {
                    cause,
                    outcome: Outcome::Oscillation,
                    condition: s.project_to(inputs),
                    settled: settled(&ringing),
                    node_levels: node_levels_at(s, &ringing),
                    group: ringing,
                    prevector: prevector_s.clone(),
                    levels: levels_s.clone(),
                    state: s.clone(),
                    discovered,
                });
            }
        }

        found
    };

    // Probe every fully-initialised reachable state in parallel, then fold the per-state records
    // together. The filter comes AFTER `enumerate`, so `discovered` stays the BFS index of the state —
    // the key a downstream reader tells two observations apart by — rather than a position in the
    // filtered sequence. Concatenation is associative, so the folded result holds the same records
    // however the work was split.
    ex.order
        .par_iter()
        .enumerate()
        .filter(|(_, s)| m.arc_eligible(s))
        .map(per_state)
        .reduce(Vec::new, |mut acc, mut other| {
            acc.append(&mut other);
            acc
        })
}

#[cfg(test)]
mod tests {
    use crate::logic::arcs::Edge;
    use crate::logic::constraint::{Constraint, ConstraintKind};
    use crate::logic::hazard::{Cause, Hazard, Outcome};
    use crate::model::analyse_one as analyse;
    use crate::model::AnalysedCell;
    use std::collections::BTreeSet;

    /// This pass's own records, picked out of the cell's one `hazards` list by their cause: the
    /// race-cause ones sharing that list are [`super::super::confluence`]'s.
    fn pulses(cell: &AnalysedCell) -> Vec<&Hazard> {
        cell.hazards
            .iter()
            .filter(|hz| matches!(hz.cause, Cause::Pulse { .. }))
            .collect()
    }

    /// The pulsed pin and its opening edge of a record [`pulses`] has already picked out, so any other
    /// cause reaching here is a filtering fault rather than a case to handle.
    fn pulse(hz: &Hazard) -> (String, char) {
        match &hz.cause {
            Cause::Pulse { pin, edge } => (pin.to_string(), edge.rf()),
            Cause::Race { pins } => panic!("a race over {pins:?} came through the pulse filter"),
        }
    }

    /// One detected hazard as what identifies it: the pulsed pin, the pulse's opening edge, the outcome
    /// and the nodes the record names.
    fn keys(cell: &AnalysedCell) -> BTreeSet<(String, char, Outcome, String)> {
        pulses(cell)
            .into_iter()
            .map(|hz| {
                let (pin, edge) = pulse(hz);
                (pin, edge, hz.outcome, hz.group.join(","))
            })
            .collect()
    }

    /// The cell's race-cause oscillations — [`super::super::confluence`]'s records. The pulse tests read
    /// them to check that a ringing candidate stays a pulse-cause record and files no race.
    fn race_rings(cell: &AnalysedCell) -> Vec<&Hazard> {
        cell.hazards
            .iter()
            .filter(|hz| {
                matches!(hz.cause, Cause::Race { .. }) && hz.outcome == Outcome::Oscillation
            })
            .collect()
    }

    /// Every record on `pin` in `edge` with `outcome`, or a panic naming what was detected instead. A
    /// pulse is probed from every state that carries it and each probe files its own record, so what a
    /// test states of one observation it states of each.
    fn on<'h>(cell: &'h AnalysedCell, pin: &str, edge: Edge, outcome: Outcome) -> Vec<&'h Hazard> {
        let found: Vec<&Hazard> = pulses(cell)
            .into_iter()
            .filter(|hz| hz.outcome == outcome && pulse(hz) == (pin.to_string(), edge.rf()))
            .collect();
        assert!(
            !found.is_empty(),
            "no {outcome:?} {pin}{} hazard in {:?}",
            edge.rf(),
            keys(cell)
        );
        found
    }

    /// Every record on `pin` in `edge` with `outcome` over the nodes `nodes` — all four fields of a
    /// detection key ([`keys`]), which is what tells two observations of one pulse apart.
    fn on_nodes<'h>(
        cell: &'h AnalysedCell,
        pin: &str,
        edge: Edge,
        outcome: Outcome,
        nodes: &str,
    ) -> Vec<&'h Hazard> {
        let found: Vec<&Hazard> = on(cell, pin, edge, outcome)
            .into_iter()
            .filter(|hz| hz.group.join(",") == nodes)
            .collect();
        assert!(
            !found.is_empty(),
            "no {outcome:?} {pin}{} hazard over {{{nodes}}} in {:?}",
            edge.rf(),
            keys(cell)
        );
        found
    }

    /// A state written out from `(node, level)` pairs in group order, in the report's spelling.
    fn state(pairs: &[(&str, bool)]) -> String {
        let body: Vec<String> = pairs
            .iter()
            .map(|(n, v)| format!("{n}={}", *v as u8))
            .collect();
        format!("{{{}}}", body.join(", "))
    }

    /// Both waypoints at one state: what a pulse whose closing edge moves nothing walks through, the
    /// machine holding where the opening cascade rested.
    fn held(pairs: &[(&str, bool)]) -> Vec<String> {
        vec![state(pairs), state(pairs)]
    }

    /// The two waypoints a record states, as the report renders them and in the order it holds them —
    /// the opening edge's convergence point, then the reference, where the closing edge settles. Every
    /// pulse record states exactly those two: a probe whose reference does not converge files nothing.
    fn waypoints(hz: &Hazard) -> Vec<String> {
        let stated = hz.settled_strs();
        assert_eq!(
            stated.len(),
            2,
            "a pulse record states two waypoints, got {stated:?}"
        );
        stated
    }

    /// The level `node` holds at the state a record was observed from.
    fn level_at(hz: &Hazard, node: &str) -> bool {
        hz.state.value_of(node).expect("a probed state is total")
    }

    #[test]
    fn dff_pulse_width_decides_the_slave_then_the_master() {
        // Master-slave DFF: M is transparent while CLK is low, Q while CLK is high. Both are state
        // variables, in signal order (outputs first) [Q, M].
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
        // CLK↑ from a state where Q disagrees with M (M equals D at every stable CLK-low state, so that
        // is Q disagreeing with D): the opening toggle opens the slave, whose one step copies M into Q
        // and rests there, so the reference — the close after that one step — leaves Q at M, while the
        // zero-width candidate leaves Q as it was. The cascade rests in ONE round, so the reference is
        // the pulse's only computed close and the disagreement is the zero-width candidate's alone. M
        // holds through both (δ_M = M at CLK=1), so the width decides Q alone.
        //
        // CLK↓ from a state where M disagrees with D (Q equals M at every stable CLK-high state): the
        // opening toggle opens the master, whose one step takes D into M; the reference re-opens the
        // slave, which then copies the new M into Q. So the reference moves BOTH, in signal order
        // [Q, M], where the zero-width candidate moves neither. Every candidate of either pulse
        // converges, so both are one indeterminate record and nothing rings.
        assert_eq!(
            keys(&cell),
            [
                (
                    "CLK".to_string(),
                    'R',
                    Outcome::Indeterminate,
                    "Q".to_string()
                ),
                (
                    "CLK".to_string(),
                    'F',
                    Outcome::Indeterminate,
                    "Q,M".to_string()
                ),
            ]
            .into_iter()
            .collect(),
        );
        // CLK↑: the opening cascade rests with Q at M (= !Q at the probed state, the pulse having
        // something to move), and the reference's closing CLK↓ re-opens the master onto D — which the
        // stable probed state already had M at — so Q is where the cascade left it at both waypoints.
        // The two states carrying this pulse are mirror images, so each record's waypoints are read
        // against the Q ITS probed state holds.
        for rise in on(&cell, "CLK", Edge::Rise, Outcome::Indeterminate) {
            let q_rise = level_at(rise, "Q");
            assert_eq!(waypoints(rise), held(&[("Q", !q_rise)]),);
        }
        // CLK↓: the opening cascade rests with M at D (= !M at the probed state) and Q untouched, and
        // only then does the reference's closing CLK↑ walk that M into Q. The two waypoints differ, and
        // they cannot be swapped — Q moves BECAUSE the closing edge follows the opening one.
        for fall in on(&cell, "CLK", Edge::Fall, Outcome::Indeterminate) {
            let q_fall = level_at(fall, "Q");
            assert_eq!(
                waypoints(fall),
                [
                    state(&[("Q", q_fall), ("M", !q_fall)]),
                    state(&[("Q", !q_fall), ("M", !q_fall)]),
                ],
            );
        }
        // A D pulse is inert at either clock level: at CLK=1 nothing reads D at all, and at CLK=0 the
        // transparent master tracks D straight back to where it was, so every candidate lands on the
        // reference.
        assert!(
            !pulses(&cell).iter().any(|hz| pulse(hz).0 == "D"),
            "a D pulse settles back to where it started, got {:?}",
            keys(&cell)
        );
    }

    #[test]
    fn transparent_latch_pulse_width_decides_its_hold() {
        // Latch transparent while E is high. E↑ from a state where Q disagrees with D opens it: one step
        // takes D into Q, and the reference — closing after that step — holds the new value, where the
        // zero-width candidate holds the old.
        let cell = analyse(
            r#"
[[cell]]
name = "LATCH"
inputs = ["E", "D"]
[cell.outputs]
Q = "E*D + !E*Q"
"#,
        );
        // E↓ is inert: at every reachable E=1 state Q already equals D, so the toggled node (δ_Q = Q
        // while E is low) is stable at once — a one-node trace, whose only close is the reference,
        // leaving no candidate to disagree with it. A D pulse is inert for the same reason on the other
        // side: with E low D reaches nothing, and with E high the transparent latch tracks D back.
        assert_eq!(
            keys(&cell),
            [(
                "E".to_string(),
                'R',
                Outcome::Indeterminate,
                "Q".to_string()
            )]
            .into_iter()
            .collect(),
        );
        // The opening edge leaves Q at D (= !Q at the probed state), and the closing edge shuts the latch
        // on it: both waypoints are the captured value, at each of the states this pulse is observed from.
        for hz in on(&cell, "E", Edge::Rise, Outcome::Indeterminate) {
            let q0 = level_at(hz, "Q");
            assert_eq!(waypoints(hz), held(&[("Q", !q0)]));
        }
    }

    #[test]
    fn sr_latch_pulse_width_decides_the_pair_and_can_leave_it_ringing() {
        // Cross-NOR SR (the `examples/cells.toml` cell): asserting S from the reset state (S=0, R=0,
        // Q=0, Qn=1) opens a two-step cascade — Qn falls, then Q rises. The candidate closing at the
        // first cut lands on the illegal both-low state under S=R=0, which rings (both rise, both fall,
        // …): no convergence point. The reference, closing after the second, lands on the set state,
        // which holds.
        let cell = analyse(
            r#"
[[cell]]
name = "SR"
inputs = ["S", "R"]
[cell.outputs]
Q  = "!(R+Qn)"
Qn = "!(S+Q)"
"#,
        );
        // So one set pulse is TWO records under the one cause: the zero-width candidate disagrees with
        // the reference — the reset state it started from against the set state — and the interior
        // candidate rings. Both name {Q, Qn}, and asserting R from the set state is the mirror image of
        // it. A pulse the other way round is inert either way: dropping an input that is already
        // released is not a state any walk reaches, and from a co-asserted state the opening toggle
        // settles at once.
        let both = |pin: &str| {
            [
                (
                    pin.to_string(),
                    'R',
                    Outcome::Indeterminate,
                    "Q,Qn".to_string(),
                ),
                (
                    pin.to_string(),
                    'R',
                    Outcome::Oscillation,
                    "Q,Qn".to_string(),
                ),
            ]
        };
        assert_eq!(
            keys(&cell),
            both("S").into_iter().chain(both("R")).collect(),
        );
        // Both records of the set pulse walk the same two waypoints, projected onto the same nodes: the
        // opening cascade rests at the set state, and the reference's closing S↓ holds it there. The set
        // pulse is observed only from the reset state — from the set state the opening toggle is stable
        // at once, leaving no candidate — so the levels are fixed, not read back off the representative.
        let set = held(&[("Q", true), ("Qn", false)]);
        for outcome in [Outcome::Indeterminate, Outcome::Oscillation] {
            for hz in on(&cell, "S", Edge::Rise, outcome) {
                assert_eq!(waypoints(hz), set);
            }
        }
        // The reset pulse mirrors it, from the set state onto the reset one.
        let reset = held(&[("Q", false), ("Qn", true)]);
        for outcome in [Outcome::Indeterminate, Outcome::Oscillation] {
            for hz in on(&cell, "R", Edge::Rise, outcome) {
                assert_eq!(waypoints(hz), reset);
            }
        }
        // The ringing candidate is a pulse-cause hazard and files no race-cause one: the cell's only
        // race-cause oscillation is still the simultaneous release confluence detects, whose cause is
        // the S and R edges racing each other rather than one pin's two edges.
        let rings = race_rings(&cell);
        assert_eq!(
            rings.len(),
            1,
            "the ringing candidate adds no race-cause record, got {rings:?}"
        );
        assert_eq!(rings[0].group, ["Q", "Qn"]);
        // That race is the simultaneous release, so it is toggled FROM the co-asserted state, which is
        // the assignment its `when` states. (It rings at S=R=0, which is where the release lands.)
        assert_eq!(rings[0].condition_str(), "S*R");
    }

    #[test]
    fn mutex_release_pulse_width_decides_which_grant_survives() {
        // Cross-coupled mutex, both requests up and A granted (A=1, B=1, Qa=1, Qb=0). Dropping A opens
        // a two-step cascade — Qa falls, then Qb rises on B. The candidate closing at the first cut
        // re-asserts A onto the no-grant state with both requests up, which is the mutex's own
        // oscillation point: no convergence point. The reference re-asserts it after B has taken the
        // grant, which holds. So the release pulse is two records under one cause — its width decides
        // which request ends up granted, and it can leave the pair ringing — each over the grant pair.
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
        let both = |pin: &str| {
            [
                (
                    pin.to_string(),
                    'F',
                    Outcome::Indeterminate,
                    "Qa,Qb".to_string(),
                ),
                (
                    pin.to_string(),
                    'F',
                    Outcome::Oscillation,
                    "Qa,Qb".to_string(),
                ),
            ]
        };
        assert_eq!(
            keys(&cell),
            both("A").into_iter().chain(both("B")).collect(),
        );
        // Both records of the A release walk the same two waypoints: the opening cascade rests with the
        // grant handed to B, and the reference re-asserting A onto it changes nothing (Qa = !Qb*A is
        // held low by Qb).
        // Only the co-asserted state carries this hazard — with B down the released grant is simply
        // taken back — so the grants are fixed rather than read off the representative.
        let to_b = held(&[("Qa", false), ("Qb", true)]);
        for outcome in [Outcome::Indeterminate, Outcome::Oscillation] {
            for hz in on(&cell, "A", Edge::Fall, outcome) {
                assert_eq!(waypoints(hz), to_b);
            }
        }
        let to_a = held(&[("Qa", true), ("Qb", false)]);
        for outcome in [Outcome::Indeterminate, Outcome::Oscillation] {
            for hz in on(&cell, "B", Edge::Fall, outcome) {
                assert_eq!(waypoints(hz), to_a);
            }
        }
        // A rise pulse from idle is inert: the grant it takes is handed straight back when the request
        // drops again, so every candidate lands on the reference.
        assert!(
            !pulses(&cell).iter().any(|hz| pulse(hz).1 == 'R'),
            "a request pulse from idle settles back to idle, got {:?}",
            keys(&cell)
        );
        // As with the SR latch, the interior candidate's ring stays a pulse-cause record: the cell still
        // detects exactly the one race-cause oscillation confluence records at A*B — the pair asserted
        // together.
        let rings = race_rings(&cell);
        assert_eq!(
            rings.len(),
            1,
            "the ringing candidate adds no race-cause record, got {rings:?}"
        );
        assert_eq!(rings[0].group, ["Qa", "Qb"]);
        // That race is the pair asserted together, so it is toggled FROM the idle state, which is the
        // assignment its `when` states. (It rings at A=B=1, which is where the pair lands.)
        assert_eq!(rings[0].condition_str(), "!A*!B");
    }

    #[test]
    fn a_ring_and_a_divergence_of_one_pulse_keep_their_own_nodes() {
        // The cross-NOR SR again, with an internal node L latching the set output while S is asserted.
        // A set pulse from the reset state (S=0, R=0, Q=0, Qn=1) opens a three-step cascade: Qn falls,
        // then Q rises, then L follows Q. The candidate closing at the first cut lands on the both-low
        // state under S=R=0, which rings over {Q, Qn} — L holds through it, S being low. Later
        // candidates converge, and the reference converges with L raised, which the reset state the
        // zero-width candidate holds is not. Signal order (outputs first, then internals) is [Q, Qn, L].
        //
        // So the ring names {Q, Qn} and the divergence {Q, Qn, L}: different phenomena over their own
        // nodes, each filed as it was observed — the width that lands the machine somewhere definite
        // answers nothing about the width that leaves it ringing.
        let cell = analyse(
            r#"
[[cell]]
name = "SRL"
inputs = ["S", "R"]
[cell.internal]
L = "S*Q + !S*L"
[cell.outputs]
Q  = "!(R+Qn)"
Qn = "!(S+Q)"
"#,
        );
        // The set pulse is observed from the already-set state too — the one whose L has not yet followed
        // Q — and there the cascade moves L alone, so that observation names {L}. The reset pulse walks
        // no L (δ_L = L while S is low), so it names {Q, Qn} under both outcomes.
        assert_eq!(
            keys(&cell),
            [
                (
                    "S".to_string(),
                    'R',
                    Outcome::Indeterminate,
                    "Q,Qn,L".to_string()
                ),
                (
                    "S".to_string(),
                    'R',
                    Outcome::Indeterminate,
                    "L".to_string()
                ),
                (
                    "S".to_string(),
                    'R',
                    Outcome::Oscillation,
                    "Q,Qn".to_string()
                ),
                (
                    "R".to_string(),
                    'R',
                    Outcome::Indeterminate,
                    "Q,Qn".to_string()
                ),
                (
                    "R".to_string(),
                    'R',
                    Outcome::Oscillation,
                    "Q,Qn".to_string()
                ),
            ]
            .into_iter()
            .collect(),
        );
        // Each record's waypoints are its own nodes' halves of the same two states: the cascade rests
        // set with L raised, and the reference's closing S↓ holds all three there.
        for hz in on_nodes(&cell, "S", Edge::Rise, Outcome::Indeterminate, "Q,Qn,L") {
            assert_eq!(
                waypoints(hz),
                held(&[("Q", true), ("Qn", false), ("L", true)])
            );
        }
        for hz in on_nodes(&cell, "S", Edge::Rise, Outcome::Oscillation, "Q,Qn") {
            assert_eq!(waypoints(hz), held(&[("Q", true), ("Qn", false)]));
        }
    }

    #[test]
    fn same_phase_cascade_pulse_width_decides_how_far_the_data_gets() {
        // Two latches transparent on the same clock phase (`examples/sequentials.toml`'s TCASC), so a
        // CLK-low pulse walks D through M and then M through Q, one stage per step. Three widths, three
        // landing points: too narrow to move anything, wide enough for M alone, and the reference, wide
        // enough for both — one indeterminate record over both nodes. Every candidate converges; a
        // same-phase cascade has nothing to ring.
        let cell = analyse(
            r#"
[[cell]]
name = "TCASC"
inputs = ["CLK", "D"]
clock = ["CLK"]
constraint_arcs = true
[cell.internal]
M = "!CLK*D + CLK*M"
[cell.outputs]
Q = "!CLK*M + CLK*Q"
"#,
        );
        assert_eq!(
            keys(&cell),
            [(
                "CLK".to_string(),
                'F',
                Outcome::Indeterminate,
                "Q,M".to_string()
            )]
            .into_iter()
            .collect(),
        );
        // Two reachable CLK-high states carry this hazard — one with D=1 over a cleared pair, one with
        // D=0 over a set pair — and each is its own record. They are mirror images, so each record's
        // waypoints are read against the D ITS probed state holds: the opening CLK↓ leaves both stages
        // transparent and the cascade walks D into M and then into Q, and the reference's closing CLK↑
        // freezes both where it found them.
        for hz in on(&cell, "CLK", Edge::Fall, Outcome::Indeterminate) {
            let d0 = level_at(hz, "D");
            assert_eq!(waypoints(hz), held(&[("Q", d0), ("M", d0)]),);
        }
    }

    #[test]
    fn nested_node_sets_are_each_reported() {
        // The same cascade as TCASC with its second stage gated: M is transparent while CLK is low, Q
        // follows M while CLK is low AND EN is high, and holds otherwise. Signal order (outputs first) is
        // [Q, M].
        let cell = analyse(
            r#"
[[cell]]
name = "GCASC"
inputs = ["CLK", "D", "EN"]
[cell.internal]
M = "!CLK*D + CLK*M"
[cell.outputs]
Q = "!CLK*(EN*M + !EN*Q) + CLK*Q"
"#,
        );
        // A CLK↓ pulse decides different nodes from different CLK-high states: with EN low the open
        // master moves alone ({M}); with EN high over a master already at D the second stage moves alone
        // ({Q}); with EN high over a master off D the pulse walks both ({Q, M}). All three are the one
        // CLK↓ cause under the one indeterminate outcome, and each is reported as it was observed. The
        // first two decide strict subsets of the third's nodes, and it is emission that reads that —
        // `emit::arcs_tcl`'s `dominates` renders the widest alone.
        //
        // EN↑ is a cause of its own: with CLK low, raising EN opens the second stage for one step — Q
        // takes M — and dropping EN again holds Q where the pulse left it.
        assert_eq!(
            keys(&cell),
            [
                (
                    "CLK".to_string(),
                    'F',
                    Outcome::Indeterminate,
                    "Q,M".to_string()
                ),
                (
                    "CLK".to_string(),
                    'F',
                    Outcome::Indeterminate,
                    "Q".to_string()
                ),
                (
                    "CLK".to_string(),
                    'F',
                    Outcome::Indeterminate,
                    "M".to_string()
                ),
                (
                    "EN".to_string(),
                    'R',
                    Outcome::Indeterminate,
                    "Q".to_string()
                ),
            ]
            .into_iter()
            .collect(),
        );
    }

    #[test]
    fn incomparable_node_sets_both_stand() {
        // Two independent latches on one clock, each opened by a level of SEL: A follows D while CLK is
        // low and SEL high, B while CLK is low and SEL low.
        let cell = analyse(
            r#"
[[cell]]
name = "SPLIT"
inputs = ["CLK", "D", "SEL"]
[cell.outputs]
A = "!CLK*(SEL*D + !SEL*A) + CLK*A"
B = "!CLK*(!SEL*D + SEL*B) + CLK*B"
"#,
        );
        // A CLK↓ pulse decides A alone where SEL is high and B alone where it is low. Neither node set
        // contains the other, so neither block's measurement would answer for the other's node and both
        // records stand on the one CLK↓ cause. SEL's own pulse opens one stage for its width while CLK is
        // low, and which stage that is IS the edge: SEL↑ decides A, SEL↓ decides B.
        assert_eq!(
            keys(&cell),
            [
                (
                    "CLK".to_string(),
                    'F',
                    Outcome::Indeterminate,
                    "A".to_string()
                ),
                (
                    "CLK".to_string(),
                    'F',
                    Outcome::Indeterminate,
                    "B".to_string()
                ),
                (
                    "SEL".to_string(),
                    'R',
                    Outcome::Indeterminate,
                    "A".to_string()
                ),
                (
                    "SEL".to_string(),
                    'F',
                    Outcome::Indeterminate,
                    "B".to_string()
                ),
            ]
            .into_iter()
            .collect(),
        );
    }

    /// The constraints generated from this pass's records, picked out of the cell's one `constraints`
    /// list by their kind: the separations sharing that list come from race-cause hazards.
    fn widths(cell: &AnalysedCell) -> Vec<&Constraint> {
        cell.constraints
            .iter()
            .filter(|c| matches!(c.kind, ConstraintKind::MinPulseWidth))
            .collect()
    }

    /// One generated constraint as the triple that identifies it: the constrained pin, the pulse's
    /// opening edge and the victim nodes it probes. The levels sampled beside those nodes name WHICH probed
    /// state the representative came from, which is a choice the exploration order makes.
    fn constrained(cell: &AnalysedCell) -> BTreeSet<(String, char, String)> {
        widths(cell)
            .into_iter()
            .map(|c| {
                let nodes: Vec<&str> = c.nodes.iter().map(|p| p.node.as_str()).collect();
                (c.pin.to_string(), c.pin_edge.rf(), nodes.join(","))
            })
            .collect()
    }

    /// A master-slave DFF opting into constraint arcs, with whatever cell-level keys `extra` adds.
    fn dff(extra: &str) -> String {
        format!(
            r#"
[[cell]]
name = "DFF"
inputs = ["CLK", "D"]
constraint_arcs = true
{extra}[cell.internal]
M = "!CLK*D + CLK*M"
[cell.outputs]
Q = "CLK*M + !CLK*Q"
"#
        )
    }

    #[test]
    fn declaring_a_clock_does_not_move_the_min_pulse_width_constraints() {
        // A confluence constraint's kind IS the declaration's — a pair holding one declared clock is
        // directed setup/hold, any other pair symmetric non_seq. A pulse relates one pin to itself, so
        // there is no pair to direct and nothing for the declaration to decide: the same cell generates
        // the same constraints either way.
        let declared = analyse(&dff("clock = [\"CLK\"]\n"));
        let plain = analyse(&dff(""));
        assert_eq!(
            constrained(&declared),
            [
                ("CLK".to_string(), 'R', "Q".to_string()),
                ("CLK".to_string(), 'F', "Q,M".to_string()),
            ]
            .into_iter()
            .collect(),
        );
        assert_eq!(constrained(&declared), constrained(&plain));
    }

    #[test]
    fn a_pulse_that_rings_and_diverges_over_one_node_set_is_one_constraint() {
        // The cross-NOR SR's set pulse is two records over {Q, Qn}: the interior candidate rings, and
        // the zero-width one holds the reset state the reference leaves. One S↑ from one state — one
        // situation seen as two phenomena — and the width that removes the ring is the width that
        // removes the divergence, so the pair is a single constraint. The reset pulse
        // mirrors it, so the cell constrains two pulses out of four records.
        let cell = analyse(
            r#"
[[cell]]
name = "SR"
inputs = ["S", "R"]
constraint_arcs = true
[cell.outputs]
Q  = "!(R+Qn)"
Qn = "!(S+Q)"
"#,
        );
        assert_eq!(
            pulses(&cell).len(),
            4,
            "detection files a record per phenomenon, got {:?}",
            keys(&cell)
        );
        assert_eq!(
            constrained(&cell),
            [
                ("S".to_string(), 'R', "Q,Qn".to_string()),
                ("R".to_string(), 'R', "Q,Qn".to_string()),
            ]
            .into_iter()
            .collect(),
        );
        assert_eq!(
            widths(&cell).len(),
            2,
            "one constraint per situation, got {:?}",
            widths(&cell)
        );
    }

    #[test]
    fn a_cell_that_did_not_opt_in_generates_no_min_pulse_width() {
        // Detection always runs — the hazard is reported whether or not the cell asks for constraint
        // arcs — while generation sits behind the same per-cell opt-in as every other constraint.
        let cell = analyse(&dff("").replace("constraint_arcs = true\n", ""));
        assert!(
            !pulses(&cell).is_empty(),
            "the flop's clock pulses are width-dependent all the same, got {:?}",
            keys(&cell)
        );
        assert!(widths(&cell).is_empty());
    }

    #[test]
    fn combinational_cell_has_no_pulse_hazard() {
        // No state variable, so no memory for a pulse to leave anything in: closing it returns every
        // coordinate to what the pre-pulse inputs make of it.
        let cell = analyse(
            r#"
[[cell]]
name = "ND2"
inputs = ["A", "B"]
[cell.outputs]
Y = "!(A*B)"
"#,
        );
        assert!(pulses(&cell).is_empty());
    }
}
