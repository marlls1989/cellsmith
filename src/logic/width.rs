//! Detection of the **pulse-cause hazards**: one signal racing itself, the two edges of a single pin.
//!
//! A **pulse** on an input `p`, applied to a fully-initialised stable state `s`, is `p` toggled (the
//! OPENING edge), the cascade that toggle opens left to run some distance, and `p` toggled back (the
//! CLOSING edge). That distance is the pulse's **width**, counted here in next-state rounds of the
//! [`machine`]: writing `t[0..last]` for the settling trace of the opening toggle
//! ([`machine::settle_trace`] — `t[0]` the toggle itself, `t[last]` its fixpoint), closing the pulse
//! after `i` rounds is the **cut** `i`, which settles `toggle(t[i], [p])`, and a wider pulse is a later
//! cut.
//!
//! That is the [`Cause::Pulse`] half of the hazard taxonomy, the sibling of [`Cause::Race`] — two
//! signals racing each other — which [`super::confluence`] detects. What the machine then does is the
//! other, independent axis, and this pass files one [`Hazard`] per [`Outcome`] it observes:
//!
//! - [`Outcome::Indeterminate`] — the cuts that settle disagree. Since `out_0` is `s` itself (see
//!   [`detect`]), one cut settling anywhere off `s` is already two outcomes, and the record's `group` is
//!   the state variables they differ in.
//! - [`Outcome::Oscillation`] — some cut leaves the machine walking a periodic cycle instead of reaching
//!   a fixpoint, and the record's `group` is what that cycle rings over.
//!
//! A pulse showing both files both, sharing the one [`Cause::Pulse`]: they are different phenomena over
//! their own nodes — a ring is not a disagreement between landing points — and each names the nodes its
//! own reading puts at risk.
//!
//! An oscillating cut is a pulse-cause hazard and is never also filed as a race-cause one: a race's
//! `condition` claims a primary-input assignment under which the group oscillates, and a pulse returns
//! `p` to the assignment it started from — which is stable — so that claim would be false.
//!
//! [`detect`] walks the fully-initialised reachable stable states — the same `Machine::arc_eligible`
//! measurement gate the arc derivation and [`super::confluence`] apply — and probes every cut of a
//! pulse, rather than the two ends alone: a cascade cut in flight reaches states neither the narrowest
//! nor the widest pulse settles to, and those are outcomes of the same hazard.
//!
//! One pin's pulse decides different nodes from different states, and on a cell whose internals form a
//! chain those node sets nest — one set per depth the cascade was cut at, every one of them the same
//! pulse seen from further in. [`detect`] reports the **maximal** sets alone: an observation is dropped
//! only where another observation of the same cause AND outcome decides a strict superset of its nodes,
//! and two sets that nest neither way both stand. What makes that sound is how the hazard is measured.
//! The block characterising one names the nodes it protects in Liberate's `-probe`, and Liberate narrows
//! the pulse until the probed behaviour fails, so the width a probe set reports is the maximum over the
//! nodes in it. A block probing a strict superset therefore asks a strictly stronger question, and the
//! subset's answer is contained in it; a superset and an incomparable set ask different questions, and
//! both are asked. The outcome is part of that key because the argument holds only within one: a
//! divergence covering a ring's nodes measures how wide the pulse must be to land somewhere definite,
//! which answers nothing about the ring.
//!
//! What that leaves open is whether the width one node needs can itself differ with the start state it
//! is characterised from. If it can, a dropped observation's nodes end up measured from the surviving
//! observation's state rather than from their own, and the collapse trades that conservatism for a
//! fraction of the characterisation cost.
//!
//! **Implementation notes:** states are probed in parallel and their per-state dedup maps merged
//! together, as in [`super::confluence::detect`]. Merge order cannot move the result: the maximal sets
//! of a collection are the same however it is accumulated, since an observation dropped against an
//! incumbent stays dominated when that incumbent is itself dropped — only a strict superset of it
//! displaces it, and strict inclusion is transitive — and among observations over the SAME nodes the
//! survivor is the min `(prevector.len, discovered)`, a total order. Both levels of the dedup are
//! [`BTreeMap`]s, so report order is deterministic independent of any hash map's.

use std::collections::{BTreeMap, BTreeSet};

use rayon::prelude::*;

use espresso_logic::bdd::{Brand, ManagerCell};
use espresso_logic::{Minterm, Symbol};

use crate::logic::analysis::Machine;
use crate::logic::arcs::{ArcLevels, Edge};
use crate::logic::confluence::{edge_from, node_levels_at, oscillating_group, protected};
use crate::logic::hazard::{Cause, Hazard, Outcome};
use crate::logic::machine;

/// One **minimum-pulse-width** constraint, generated from a pulse-cause [`Hazard`] to remove it: the
/// width a pulse on the pin must have for the nodes it names to reach the outcome a wide pulse settles
/// to. Liberate measures that width off the emitted block, narrowing the pulse until the probed
/// behaviour fails. It is a SINGLE-pin constraint — the sibling of
/// [`Constraint`](crate::logic::confluence::Constraint), which relates two primary inputs — and picking
/// this struct IS the classification, so it carries neither a kind nor a related pin.
#[derive(Debug, Clone)]
pub struct MinPulseWidth {
    /// The constrained pin, which the emitted block names on BOTH `-pin` and `-related_pin`: the
    /// constraint relates the pin to itself.
    pub pin: Symbol,
    /// The pulse's OPENING polarity — rise means the pulse is high, fall low. The block states that one
    /// edge, and Liberate searches the width itself.
    pub edge: Edge,
    /// The prevector: the input-assignment path that drives every state variable into the state where
    /// the hazard manifests (each node projected onto the inputs).
    pub prevector: Vec<Minterm<Symbol>>,
    /// The levels the cell's outputs hold in that state — the block's `-ic` initial condition, sampled
    /// at the same probed state as `prevector`.
    pub levels: ArcLevels,
    /// The nodes this constraint protects, each with the level it holds at the probed state, exactly as
    /// [`Constraint::nodes`](crate::logic::confluence::Constraint::nodes) does it: the state variables
    /// the hazard names, in signal declaration order. The emitted block gives each a column of its own
    /// and names them all in one Liberate `-probe`.
    pub nodes: Vec<(Symbol, bool)>,
    /// The probed state itself: every input and state variable at the level it holds there. The
    /// prevector reaches it and the levels sample its pins, but only this names the internal nodes no
    /// emitted column carries.
    pub state: Minterm<Symbol>,
}

/// Generate the minimum-pulse-width constraints that avoid a cell's pulse-cause hazards: one
/// [`MinPulseWidth`] per pulse-cause [`Hazard`], protecting the nodes that observation names. The cause
/// states the pin and the pulse's opening edge; any other cause is another generator's record and is
/// passed over.
///
/// The map is 1:1. [`detect`] keys on the cause together with the outcome and keeps the maximal node
/// sets under inclusion, so two records of one pulse — its ring and its divergence — can protect the
/// same nodes and generate the same constraint twice; identical constraints render identical blocks, and
/// the emitter states a block once however many firings reach it.
///
/// No `clock_pins` either, unlike [`confluence::constrain`](super::confluence::constrain): a pulse
/// relates one pin to itself, so there is no pair for a declared clock to direct and nothing for the
/// declaration to decide.
pub(crate) fn constrain(hz: &[Hazard]) -> Vec<MinPulseWidth> {
    hz.iter()
        .filter_map(|h| match &h.cause {
            Cause::Pulse { pin, edge } => Some(MinPulseWidth {
                pin: pin.clone(),
                edge: *edge,
                prevector: h.prevector.clone(),
                levels: h.levels.clone(),
                nodes: protected(&h.group, &h.node_levels),
                state: h.state.clone(),
            }),
            Cause::Race { .. } => None,
        })
        .collect()
}

/// Why every state value a pulse walk reads is defined, exactly as in [`super::confluence`]: a probe
/// starts from a state `Machine::arc_eligible` admits, and settling from a fully-initialised state
/// leaves every state column determinate, so the whole trajectory — the opening cascade and every cut's
/// settle alike — is total and this message is unreachable.
const DETERMINATE: &str =
    "a settle from a fully-initialised state leaves every state column determinate";

/// The observations kept under one (cause, outcome) key: one per maximal set of named nodes, keyed by
/// that set so an observation over nodes already spoken for meets its incumbent rather than joining it.
type Maximal = BTreeMap<String, Hazard>;

/// Detected pulse-cause hazards under the dedup key [`record`] computes — one probed state's own map, or
/// the merge of several.
type Detected = BTreeMap<(Cause, Outcome), Maximal>;

/// Detect a cell's pulse-cause hazards by pulsing every input at every fully-initialised reachable
/// stable state (`Machine::arc_eligible`: a state carrying an uninitialised state variable is at an
/// unknown state, from which nothing can be concluded). Produces one [`Hazard`] per (cause, outcome) per
/// maximal set of named nodes (see the module note on why a set another's strictly contains carries
/// nothing), keeping the representative reached along the shortest prevector, and generates no
/// constraint. Empty for cells whose pulses all settle back to where they started.
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
    // input and fill this state's own dedup map. Each state is independent — the parallel unit — and the
    // maps merge commutatively in the `reduce` below.
    let per_state = |(discovered, s): (usize, &Minterm<Symbol>)| -> Detected {
        let mut found = Detected::new();

        // Both depend only on `s`, so they are computed once here and cloned per hazard. Sampling them
        // side by side at the one probed state is what keeps them consistent through the
        // min-`(prevector.len, discovered)` dedup: a surviving representative carries the levels of the
        // very state its prevector walks to.
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

            // Close the pulse at every cut of the opening cascade. Cut 0 — closing at the opening toggle
            // itself, the zero-width pulse — is skipped by derivation rather than probed: `toggle`
            // writes only the named input's column, so closing at `t[0]` reproduces `s` exactly, and `s`
            // is stable, hence `out_0 == s`.
            let cuts: Vec<Result<Minterm<Symbol>, Vec<Minterm<Symbol>>>> = trace
                .iter()
                .skip(1)
                .map(|cut| {
                    let closed = machine::toggle(cut, &[p.as_str()]);
                    machine::settle_or_cycle(&deltas, &closed)
                })
                .collect();

            // Does `w` hold a different value under some closing cut than it does at `s`? Both are total
            // (see `DETERMINATE`), so this is a comparison of values, not of definedness. `out_0` is `s`,
            // so a `w` answering yes is one the cuts that settle disagree over.
            let diverges = |w: &Symbol| {
                cuts.iter().filter_map(|out| out.as_ref().ok()).any(|out| {
                    out.value_of(w.as_str()).expect(DETERMINATE)
                        != s.value_of(w.as_str()).expect(DETERMINATE)
                })
            };
            let diverging: Vec<Symbol> = m
                .state_vars
                .iter()
                .filter(|w| diverges(w))
                .cloned()
                .collect();
            // Every node any cut rings over. Several cuts of one pulse can ring, and they are the one
            // cause under the one outcome, so they are the one record over the nodes between them.
            let rings: BTreeSet<Symbol> = cuts
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

            // The two waypoints a well-spaced pulse walks through, in causal order: `t[last]`, where the
            // OPENING edge's own cascade came to rest, and then where the machine settles once the
            // CLOSING edge is placed on it — the widest cut, the last of `cuts`. The order is
            // load-bearing: a transition cannot be a rise and a fall at once, so a pulse's two edges are
            // necessarily sequential and one waypoint is reached through the other. `out_0` is no member
            // of this — the zero-width pulse violates the constraint the hazard produces rather than
            // landing anywhere the machine legitimately does. A widest cut that rings names no landing
            // point at all, and the opening waypoint is then the whole of what a pulse reaches.
            let settled = |group: &[Symbol]| -> Vec<Minterm<Symbol>> {
                let opened = trace
                    .last()
                    .expect("a settle trace is seeded with the node itself")
                    .project_to(group);
                std::iter::once(opened)
                    .chain(
                        cuts.last()
                            .and_then(|out| out.as_ref().ok())
                            .map(|out| out.project_to(group)),
                    )
                    .collect()
            };

            let cause = Cause::Pulse {
                pin: p.clone(),
                edge: edge_from(s, p.as_str()),
            };
            // The cuts that settle disagree over these nodes, so which state a pulse leaves them in is
            // not determined: the hazard proper, empty exactly where every cut that settles agrees with
            // `s`.
            if !diverging.is_empty() {
                record(
                    &mut found,
                    Hazard {
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
                    },
                );
            }
            // A cut that rings names at least one state variable — the inputs hold through a settle and
            // a combinational coordinate lies on no dependency cycle (see [`super::resolve`]), so a
            // cycle that moved no held coordinate would be a fixpoint — hence a non-empty group here is
            // exactly "some cut did not settle".
            if !ringing.is_empty() {
                record(
                    &mut found,
                    Hazard {
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
                    },
                );
            }
        }

        found
    };

    // Probe every fully-initialised reachable state in parallel, then fold the per-state dedup maps
    // together. The filter comes AFTER `enumerate`, so `discovered` stays the BFS index of the state —
    // the tie-break the dedup reads — rather than a position in the filtered sequence. The merge is
    // associative and commutative (see the module note on why keeping the maximal sets is), so the
    // folded result equals the sequential one regardless of state/thread order.
    ex.order
        .par_iter()
        .enumerate()
        .filter(|(_, s)| m.arc_eligible(s))
        .map(per_state)
        .reduce(Detected::new, |mut acc, other| {
            for hz in other.into_values().flat_map(Maximal::into_values) {
                record(&mut acc, hz);
            }
            acc
        })
        .into_values()
        .flat_map(Maximal::into_values)
        .collect()
}

/// Record a detected pulse-cause hazard into the dedup map, keeping the observations whose named nodes
/// no other observation under the same key strictly contains and, among observations over the same
/// nodes, the min `(prevector.len, discovered)` representative.
///
/// The key is the cause — the pulsed pin with its opening edge — TOGETHER WITH the outcome. The outcome
/// belongs there because the maximal-set collapse only speaks within one: a divergence over a ring's
/// nodes would otherwise stand for it, and the width that lands the machine somewhere definite says
/// nothing about the width that leaves it ringing.
fn record(map: &mut Detected, hz: Hazard) {
    let nodes = hz.group.join(",");
    let kept = map.entry((hz.cause.clone(), hz.outcome)).or_default();

    // An incumbent over a strict superset is characterised against these nodes too, so this observation
    // asks nothing that is not already asked (module note) and it goes; the `retain` is the converse
    // pass, retiring the incumbents this one has come to speak for. An incumbent over the SAME nodes is
    // neither case — it is this hazard reached along another walk, and meets the tie-break below.
    if kept.values().any(|e| strictly_within(&hz.group, &e.group)) {
        return;
    }
    kept.retain(|_, e| !strictly_within(&e.group, &hz.group));
    // The `Option` read here is the incumbent — no entry yet for these nodes, or one this candidate beats
    // on `(prevector.len, discovered)` — nothing to do with a state value's determinacy.
    if kept
        .get(&nodes)
        .is_none_or(|e| (hz.prevector.len(), hz.discovered) < (e.prevector.len(), e.discovered))
    {
        kept.insert(nodes, hz);
    }
}

/// Is every node of `inner` among `outer`'s, with `outer` naming at least one more? Strict on purpose:
/// two observations over the same nodes are one hazard reached along different walks, settled by the
/// walk-length tie-break rather than by one displacing the other.
fn strictly_within(inner: &[Symbol], outer: &[Symbol]) -> bool {
    let inner: BTreeSet<&Symbol> = inner.iter().collect();
    let outer: BTreeSet<&Symbol> = outer.iter().collect();
    inner.len() < outer.len() && inner.is_subset(&outer)
}

#[cfg(test)]
mod tests {
    use crate::logic::arcs::Edge;
    use crate::logic::hazard::{Cause, Hazard, Outcome};
    use crate::model::analyse_one as analyse;
    use crate::model::AnalysedCell;
    use std::collections::BTreeSet;

    /// The pulsed pin and its opening edge of a detected record. Every record this pass files carries a
    /// pulse cause — it probes nothing else — so any other cause is a detection fault, not a case.
    fn pulse(hz: &Hazard) -> (String, char) {
        match &hz.cause {
            Cause::Pulse { pin, edge } => (pin.to_string(), edge.rf()),
            Cause::Race { pins } => panic!("a pulse probe filed a race over {pins:?}"),
        }
    }

    /// One detected hazard as what identifies it: the pulsed pin, the pulse's opening edge, the outcome
    /// and the nodes the record names.
    fn keys(cell: &AnalysedCell) -> BTreeSet<(String, char, Outcome, String)> {
        cell.width_dependence
            .iter()
            .map(|hz| {
                let (pin, edge) = pulse(hz);
                (pin, edge, hz.outcome, hz.group.join(","))
            })
            .collect()
    }

    /// The one record on `pin` in `edge` with `outcome`, or a panic naming what was detected instead.
    fn on<'h>(cell: &'h AnalysedCell, pin: &str, edge: Edge, outcome: Outcome) -> &'h Hazard {
        let mut found = cell
            .width_dependence
            .iter()
            .filter(|hz| hz.outcome == outcome && pulse(hz) == (pin.to_string(), edge.rf()));
        let hz = found.next().unwrap_or_else(|| {
            panic!(
                "no {outcome:?} {pin}{} hazard in {:?}",
                edge.rf(),
                keys(cell)
            )
        });
        assert!(
            found.next().is_none(),
            "more than one {outcome:?} {pin}{} hazard in {:?}",
            edge.rf(),
            keys(cell)
        );
        hz
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
    /// the opening edge's fixpoint, then where the closing edge settles.
    fn waypoints(hz: &Hazard) -> Vec<String> {
        hz.settled_strs()
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
        // is Q disagreeing with D): the opening toggle opens the slave, whose one step copies M into Q,
        // and closing at that cut leaves Q there — while the zero-width pulse leaves Q as it was. M holds
        // through both (δ_M = M at CLK=1), so the width decides Q alone.
        //
        // CLK↓ from a state where M disagrees with D (Q equals M at every stable CLK-high state): the
        // opening toggle opens the master, whose one step takes D into M; closing there re-opens the
        // slave, which then copies the new M into Q. So the wide pulse moves BOTH, in signal order
        // [Q, M], where the zero-width one moves neither. Every cut of either pulse settles, so both are
        // one indeterminate record and nothing rings.
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
        // something to move), and the closing CLK↓ re-opens the master onto D — which the stable probed
        // state already had M at — so Q is where the cascade left it at both waypoints.
        let rise = on(&cell, "CLK", Edge::Rise, Outcome::Indeterminate);
        let q_rise = level_at(rise, "Q");
        assert_eq!(waypoints(rise), held(&[("Q", !q_rise)]),);
        // CLK↓: the opening cascade rests with M at D (= !M at the probed state) and Q untouched, and
        // only then does the closing CLK↑ walk that M into Q. The two waypoints differ, and they cannot
        // be swapped — Q moves BECAUSE the closing edge follows the opening one.
        let fall = on(&cell, "CLK", Edge::Fall, Outcome::Indeterminate);
        let q_fall = level_at(fall, "Q");
        assert_eq!(
            waypoints(fall),
            [
                state(&[("Q", q_fall), ("M", !q_fall)]),
                state(&[("Q", !q_fall), ("M", !q_fall)]),
            ],
        );
        // A D pulse is inert at either clock level: at CLK=1 nothing reads D at all, and at CLK=0 the
        // transparent master tracks D straight back to where it was, whichever cut closes the pulse.
        assert!(
            !cell.width_dependence.iter().any(|hz| pulse(hz).0 == "D"),
            "a D pulse settles back to where it started, got {:?}",
            keys(&cell)
        );
    }

    #[test]
    fn transparent_latch_pulse_width_decides_its_hold() {
        // Latch transparent while E is high. E↑ from a state where Q disagrees with D opens it: one step
        // takes D into Q, and closing after that step holds the new value, where the zero-width pulse
        // holds the old.
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
        // while E is low) is stable at once — a one-node trace, with no cut past the opening toggle to
        // close at. A D pulse is inert for the same reason on the other side: with E low D reaches
        // nothing, and with E high the transparent latch tracks D back.
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
        // on it: both waypoints are the captured value.
        let hz = on(&cell, "E", Edge::Rise, Outcome::Indeterminate);
        let q0 = level_at(hz, "Q");
        assert_eq!(waypoints(hz), held(&[("Q", !q0)]));
    }

    #[test]
    fn sr_latch_pulse_width_decides_the_pair_and_can_leave_it_ringing() {
        // Cross-NOR SR (the `examples/cells.toml` cell): asserting S from the reset state (S=0, R=0,
        // Q=0, Qn=1) opens a two-step cascade — Qn falls, then Q rises. Closing at the first cut lands
        // on the illegal both-low state under S=R=0, which rings (both rise, both fall, …): no
        // fixpoint. Closing at the second lands on the set state, which holds.
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
        // So one set pulse is TWO records under the one cause: the cuts that settle disagree — the reset
        // state it started from against the set state — and one cut rings. Both name {Q, Qn}, and
        // asserting R from the set state is the mirror image of it. A pulse the other way round is inert
        // either way: dropping an input that is already released is not a state any walk reaches, and
        // from a co-asserted state the opening toggle settles at once.
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
        // opening cascade rests at the set state, and the closing S↓ holds it there. The set pulse is
        // observed only from the reset state — from the set state the opening toggle is stable at once,
        // leaving no cut to close at — so the levels are fixed, not read back off the representative.
        let set = held(&[("Q", true), ("Qn", false)]);
        assert_eq!(
            waypoints(on(&cell, "S", Edge::Rise, Outcome::Indeterminate)),
            set
        );
        assert_eq!(
            waypoints(on(&cell, "S", Edge::Rise, Outcome::Oscillation)),
            set
        );
        // The reset pulse mirrors it, from the set state onto the reset one.
        let reset = held(&[("Q", false), ("Qn", true)]);
        assert_eq!(
            waypoints(on(&cell, "R", Edge::Rise, Outcome::Indeterminate)),
            reset
        );
        assert_eq!(
            waypoints(on(&cell, "R", Edge::Rise, Outcome::Oscillation)),
            reset
        );
        // The ringing cut is a pulse-cause hazard and files no race-cause one: the cell's only
        // race-cause oscillation is still the simultaneous release confluence detects, whose condition —
        // S and R both low — is an input assignment the pair really does ring under, which the pulse's
        // returning edge is not.
        assert_eq!(
            cell.oscillation.len(),
            1,
            "the pulse cut adds no race-cause record, got {:?}",
            cell.oscillation
        );
        assert_eq!(cell.oscillation[0].group, ["Q", "Qn"]);
        assert_eq!(cell.oscillation[0].condition_str(), "!S*!R");
    }

    #[test]
    fn mutex_release_pulse_width_decides_which_grant_survives() {
        // Cross-coupled mutex, both requests up and A granted (A=1, B=1, Qa=1, Qb=0). Dropping A opens
        // a two-step cascade — Qa falls, then Qb rises on B. Closing at the first cut re-asserts A onto
        // the no-grant state with both requests up, which is the mutex's own oscillation point: no
        // fixpoint. Closing at the second re-asserts it after B has taken the grant, which holds. So the
        // release pulse is two records under one cause — its width decides which request ends up
        // granted, and it can leave the pair ringing — each over the grant pair.
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
        // grant handed to B, and re-asserting A onto it changes nothing (Qa = !Qb*A is held low by Qb).
        // Only the co-asserted state carries this hazard — with B down the released grant is simply
        // taken back — so the grants are fixed rather than read off the representative.
        let to_b = held(&[("Qa", false), ("Qb", true)]);
        assert_eq!(
            waypoints(on(&cell, "A", Edge::Fall, Outcome::Indeterminate)),
            to_b
        );
        assert_eq!(
            waypoints(on(&cell, "A", Edge::Fall, Outcome::Oscillation)),
            to_b
        );
        let to_a = held(&[("Qa", true), ("Qb", false)]);
        assert_eq!(
            waypoints(on(&cell, "B", Edge::Fall, Outcome::Indeterminate)),
            to_a
        );
        assert_eq!(
            waypoints(on(&cell, "B", Edge::Fall, Outcome::Oscillation)),
            to_a
        );
        // A rise pulse from idle is inert: the grant it takes is handed straight back when the request
        // drops again, whichever cut closes the pulse.
        assert!(
            !cell.width_dependence.iter().any(|hz| pulse(hz).1 == 'R'),
            "a request pulse from idle settles back to idle, got {:?}",
            keys(&cell)
        );
        // As with the SR latch, the interior cut's ring stays a pulse-cause record: the cell still
        // detects exactly the one race-cause oscillation confluence records at A*B — the pair asserted
        // together.
        assert_eq!(
            cell.oscillation.len(),
            1,
            "the pulse cut adds no race-cause record, got {:?}",
            cell.oscillation
        );
        assert_eq!(cell.oscillation[0].group, ["Qa", "Qb"]);
        assert_eq!(cell.oscillation[0].condition_str(), "A*B");
    }

    #[test]
    fn a_ring_and_a_divergence_of_one_pulse_keep_their_own_nodes() {
        // The cross-NOR SR again, with an internal node L latching the set output while S is asserted.
        // A set pulse from the reset state (S=0, R=0, Q=0, Qn=1) opens a three-step cascade: Qn falls,
        // then Q rises, then L follows Q. Closing at the first cut lands on the both-low state under
        // S=R=0, which rings over {Q, Qn} — L holds through it, S being low. Closing later settles, and
        // the widest cut settles with L raised, which the reset state it started from is not. Signal
        // order (outputs first, then internals) is [Q, Qn, L].
        //
        // So the ring names {Q, Qn} and the divergence {Q, Qn, L}: the ring's nodes are a strict subset,
        // and only because the outcome is part of the dedup key does it survive rather than being
        // collapsed onto the wider set — the width that lands the machine somewhere definite answers
        // nothing about the width that leaves it ringing.
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
        // The reset pulse walks no L (δ_L = L while S is low), so it names {Q, Qn} under both outcomes.
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
        // set with L raised, and the closing S↓ holds all three there.
        assert_eq!(
            waypoints(on(&cell, "S", Edge::Rise, Outcome::Indeterminate)),
            held(&[("Q", true), ("Qn", false), ("L", true)]),
        );
        assert_eq!(
            waypoints(on(&cell, "S", Edge::Rise, Outcome::Oscillation)),
            held(&[("Q", true), ("Qn", false)]),
        );
    }

    #[test]
    fn same_phase_cascade_pulse_width_decides_how_far_the_data_gets() {
        // Two latches transparent on the same clock phase (`examples/sequentials.toml`'s TCASC), so a
        // CLK-low pulse walks D through M and then M through Q, one stage per step. Three widths, three
        // landing points: too narrow to move anything, wide enough for M alone, wide enough for both —
        // one indeterminate record over both nodes. Every cut settles; a same-phase cascade has nothing
        // to ring.
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
        // D=0 over a set pair — and the dedup keeps whichever is reached along the shorter walk. They are
        // mirror images, so the waypoints are stated relative to the D the surviving representative
        // holds: the opening CLK↓ leaves both stages transparent and the cascade walks D into M and then
        // into Q, and the closing CLK↑ freezes both where it found them.
        let hz = on(&cell, "CLK", Edge::Fall, Outcome::Indeterminate);
        let d0 = level_at(hz, "D");
        assert_eq!(waypoints(hz), held(&[("Q", d0), ("M", d0)]),);
    }

    #[test]
    fn nested_node_sets_collapse_onto_the_widest() {
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
        // CLK↓ cause under the one indeterminate outcome, and the first two decide strict subsets of the
        // third's nodes, so the widest stands for them and one record is reported on CLK↓.
        //
        // EN↑ is a cause of its own and untouched by that: with CLK low, raising EN opens the second
        // stage for one step — Q takes M — and dropping EN again holds Q where the pulse left it.
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

    /// One generated constraint as the triple that identifies it: the constrained pin, the pulse's
    /// opening edge and the nodes it protects. The levels sampled beside those nodes name WHICH probed
    /// state the representative came from, which is a choice the exploration order makes.
    fn constrained(cell: &AnalysedCell) -> BTreeSet<(String, char, String)> {
        cell.min_pulse_widths
            .iter()
            .map(|pw| {
                let nodes: Vec<&str> = pw.nodes.iter().map(|(n, _)| n.as_str()).collect();
                (pw.pin.to_string(), pw.edge.rf(), nodes.join(","))
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
    fn a_cell_that_did_not_opt_in_generates_no_min_pulse_width() {
        // Detection always runs — the hazard is reported whether or not the cell asks for constraint
        // arcs — while generation sits behind the same per-cell opt-in as every other constraint.
        let cell = analyse(&dff("").replace("constraint_arcs = true\n", ""));
        assert!(
            !cell.width_dependence.is_empty(),
            "the flop's clock pulses are width-dependent all the same, got {:?}",
            keys(&cell)
        );
        assert!(cell.min_pulse_widths.is_empty());
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
        assert!(cell.width_dependence.is_empty());
    }
}
