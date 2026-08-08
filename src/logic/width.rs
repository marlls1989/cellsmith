//! Detection of the **width-dependent hazard**: a pulse whose settled outcome depends on how wide the
//! pulse is.
//!
//! A **pulse** on an input `p`, applied to a fully-initialised stable state `s`, is `p` toggled, the
//! cascade that toggle opens left to run some distance, and `p` toggled back. That distance is the
//! pulse's **width**, counted here in next-state rounds of the [`machine`]: writing `t[0..last]` for
//! the settling trace of the opening toggle ([`machine::settle_trace`] — `t[0]` the toggle itself,
//! `t[last]` its fixpoint), closing the pulse at **cut** `i` settles `toggle(t[i], [p])`, and a wider
//! pulse is a later cut.
//!
//! The **width-dependent hazard** at `(s, p)` is more than one outcome across those cuts: the settled
//! state depends on how far apart two edges of the *same* signal are — the same sentence an
//! [`OrderDependence`](crate::logic::hazard::OrderDependence) makes about which of two edges of
//! *different* signals lands first. A clock pulse too narrow to carry a flop's master through to its
//! slave, beside one wide enough to, is the shape of it: one pin, two settled states.
//!
//! [`detect`] walks the fully-initialised reachable stable states — the same `Machine::arc_eligible`
//! measurement gate the arc derivation and [`super::confluence`] apply — and probes every cut of every
//! input's pulse, rather than the two ends alone: a cascade cut in flight reaches states neither the
//! narrowest nor the widest pulse settles to, and those are outcomes of the same hazard. A cut that
//! leaves the machine oscillating is carried as a [`PulseOutcome::NoFixpoint`] outcome and is never
//! filed as an [`Oscillation`](crate::logic::hazard::Oscillation): an oscillation's `condition` claims a
//! primary-input assignment under which the group oscillates, and a pulse returns `p` to the assignment
//! it started from — which is stable — so that claim would be false.
//!
//! One pin's pulse decides different nodes from different states, and on a cell whose internals form a
//! chain those node sets nest — one set per depth the cascade was cut at, every one of them the same
//! pulse seen from further in. [`detect`] reports the **maximal** sets alone: an observation is dropped
//! only where another observation of the same pin and edge decides a strict superset of its nodes, and
//! two sets that nest neither way both stand. What makes that sound is how the hazard is measured. The
//! block characterising one names the nodes it protects in Liberate's `-probe`, and Liberate narrows the
//! pulse until the probed behaviour fails, so the width a probe set reports is the maximum over the
//! nodes in it. A block probing a strict superset therefore asks a strictly stronger question, and the
//! subset's answer is contained in it; a superset and an incomparable set ask different questions, and
//! both are asked.
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
use crate::logic::arcs::ArcLevels;
use crate::logic::confluence::{edge_from, node_levels_at, oscillating_group};
use crate::logic::hazard::{PulseOutcome, WidthDependence};
use crate::logic::machine;

/// Why every state value a pulse walk reads is defined, exactly as in [`super::confluence`]: a probe
/// starts from a state `Machine::arc_eligible` admits, and settling from a fully-initialised state
/// leaves every state column determinate, so the whole trajectory — the opening cascade and every cut's
/// settle alike — is total and this message is unreachable.
const DETERMINATE: &str =
    "a settle from a fully-initialised state leaves every state column determinate";

/// The observations kept under one pin/edge key: one per maximal set of decided nodes, keyed by that set
/// so an observation over nodes already spoken for meets its incumbent rather than joining it.
type Maximal = BTreeMap<String, WidthDependence>;

/// Detected width-dependent hazards under the dedup key [`record`] computes — one probed state's own
/// map, or the merge of several.
type Detected = BTreeMap<String, Maximal>;

/// Detect a cell's width-dependent hazards by pulsing every input at every fully-initialised reachable
/// stable state (`Machine::arc_eligible`: a state carrying an uninitialised state variable is at an
/// unknown state, from which nothing can be concluded). Produces one [`WidthDependence`] per pin/edge per
/// maximal set of decided nodes (see the module note on why a set another's strictly contains carries
/// nothing), keeping the representative reached along the shortest prevector, and generates no
/// constraint. Empty for cells whose pulses all settle back to where they started.
pub fn detect<B: Brand, C: ManagerCell + Send + Sync>(m: &Machine<B, C>) -> Vec<WidthDependence> {
    // With no memory every coordinate is a function of the inputs alone, so returning `p` to its
    // pre-pulse value returns the whole machine to `s`: a pulse can leave no net effect for its width to
    // decide.
    if m.state_vars.is_empty() {
        return Vec::new();
    }
    // No input-count guard: a width-dependent hazard relates one pin to ITSELF, so
    // `confluence::detect`'s pair-wise `n < 2` early-out — a hazard there relates two inputs — is that
    // pass's rule and not this one's.

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
            // is stable, hence `out_0 == s`. It enters the outcomes below as `s` all the same.
            let mut settled: BTreeSet<Minterm<Symbol>> = BTreeSet::new();
            let mut cycles: Vec<Vec<Minterm<Symbol>>> = Vec::new();
            for cut in trace.iter().skip(1) {
                let closed = machine::toggle(cut, &[p.as_str()]);
                match machine::settle_or_cycle(&deltas, &closed) {
                    Ok(out) => {
                        settled.insert(out);
                    }
                    Err(cycle) => cycles.push(cycle),
                }
            }

            // The hazard is more than one outcome across the cuts. `out_0` is `s`, so one settled
            // outcome away from `s` is already two outcomes; a cut that never settles is an outcome of
            // its own whatever the others did.
            if cycles.is_empty() && settled.iter().all(|out| out == s) {
                continue;
            }

            // Does `w` hold a different value under some closing cut than it does at `s`? Both are total
            // (see `DETERMINATE`), so this is a comparison of values, not of definedness.
            let diverges = |w: &Symbol| {
                settled.iter().any(|out| {
                    out.value_of(w.as_str()).expect(DETERMINATE)
                        != s.value_of(w.as_str()).expect(DETERMINATE)
                })
            };
            // The nodes the width decides: every state variable a settled cut moves off `s`, together
            // with every state variable an oscillating cut names. The union is what leaves the group
            // non-empty where only an interior cut oscillates and every settled cut agrees with `s` —
            // the hazard still names nodes, which is what the emitted block probes.
            let oscillating: BTreeSet<Symbol> = cycles
                .iter()
                .flat_map(|cycle| oscillating_group(cycle, &m.state_vars))
                .collect();
            let group: Vec<Symbol> = m
                .state_vars
                .iter()
                .filter(|w| oscillating.contains(*w) || diverges(w))
                .cloned()
                .collect();

            // The competing outcomes, each projected onto the nodes at risk: `s` is the zero-width one,
            // and every cut that oscillated collapses into the single `NoFixpoint`, since a cut with no
            // fixpoint has no state to name.
            let mut outcomes: BTreeSet<PulseOutcome> = std::iter::once(s)
                .chain(settled.iter())
                .map(|out| PulseOutcome::Settled(out.project_to(&group)))
                .collect();
            if !cycles.is_empty() {
                outcomes.insert(PulseOutcome::NoFixpoint);
            }

            record(
                &mut found,
                WidthDependence {
                    pin: p.clone(),
                    edge: edge_from(s, p.as_str()),
                    node_levels: node_levels_at(s, &group),
                    group,
                    outcomes: outcomes.into_iter().collect(),
                    prevector: prevector_s.clone(),
                    levels: levels_s.clone(),
                    state: s.clone(),
                    discovered,
                },
            );
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
            for wd in other.into_values().flat_map(Maximal::into_values) {
                record(&mut acc, wd);
            }
            acc
        })
        .into_values()
        .flat_map(Maximal::into_values)
        .collect()
}

/// Record a detected width-dependent hazard into the dedup map, keyed by the pulsed pin and its opening
/// edge, keeping the observations whose decided nodes no other observation of that key strictly contains
/// and, among observations over the same nodes, the min `(prevector.len, discovered)` representative.
fn record(map: &mut Detected, wd: WidthDependence) {
    let key = format!("{}{}", wd.pin, wd.edge.rf());
    let nodes = wd.group.join(",");
    let kept = map.entry(key).or_default();

    // An incumbent over a strict superset is characterised against these nodes too, so this observation
    // asks nothing that is not already asked (module note) and it goes; the `retain` is the converse
    // pass, retiring the incumbents this one has come to speak for. An incumbent over the SAME nodes is
    // neither case — it is this hazard reached along another walk, and meets the tie-break below.
    if kept.values().any(|e| strictly_within(&wd.group, &e.group)) {
        return;
    }
    kept.retain(|_, e| !strictly_within(&e.group, &wd.group));
    // The `Option` read here is the incumbent — no entry yet for these nodes, or one this candidate beats
    // on `(prevector.len, discovered)` — nothing to do with a state value's determinacy.
    if kept
        .get(&nodes)
        .is_none_or(|e| (wd.prevector.len(), wd.discovered) < (e.prevector.len(), e.discovered))
    {
        kept.insert(nodes, wd);
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
    use crate::logic::hazard::WidthDependence;
    use crate::model::analyse_one as analyse;
    use crate::model::AnalysedCell;
    use std::collections::BTreeSet;

    /// One detected hazard as the triple that identifies it: the pulsed pin, the pulse's opening edge
    /// and the nodes its width decides.
    fn keys(cell: &AnalysedCell) -> BTreeSet<(String, char, String)> {
        cell.width_dependence
            .iter()
            .map(|wd| (wd.pin.to_string(), wd.edge.rf(), wd.group.join(",")))
            .collect()
    }

    /// The one hazard on `pin` in `edge`, or a panic naming what was detected instead.
    fn on<'w>(cell: &'w AnalysedCell, pin: &str, edge: Edge) -> &'w WidthDependence {
        let mut found = cell
            .width_dependence
            .iter()
            .filter(|wd| wd.pin == pin && wd.edge == edge);
        let wd = found
            .next()
            .unwrap_or_else(|| panic!("no {pin}{} hazard in {:?}", edge.rf(), keys(cell)));
        assert!(
            found.next().is_none(),
            "more than one {pin}{} hazard in {:?}",
            edge.rf(),
            keys(cell)
        );
        wd
    }

    /// A hazard's competing outcomes as the report renders them (`{Q=1, M=0}`, `no fixpoint`), as a set
    /// — they are a set of outcomes, and the vector's order is the canonical sort.
    fn outcomes(wd: &WidthDependence) -> BTreeSet<String> {
        wd.outcome_strs().into_iter().collect()
    }

    /// The outcome set written out from `(node, level)` pairs in group order, in the report's spelling.
    fn settled(states: &[&[(&str, bool)]]) -> BTreeSet<String> {
        states
            .iter()
            .map(|pairs| {
                let body: Vec<String> = pairs
                    .iter()
                    .map(|(n, v)| format!("{n}={}", *v as u8))
                    .collect();
                format!("{{{}}}", body.join(", "))
            })
            .collect()
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
        // CLK↑ from a state where Q disagrees with M (e.g. CLK=0, D=0, Q=1, M=0): the opening toggle
        // opens the slave, whose one step copies M into Q, and closing at that cut leaves Q there —
        // while the zero-width pulse leaves Q as it was. M holds through both (δ_M = M at CLK=1), so the
        // width decides Q alone.
        //
        // CLK↓ from a state where M disagrees with D (e.g. CLK=1, D=0, Q=1, M=1): the opening toggle
        // opens the master, whose one step takes D into M; closing there re-opens the slave, which then
        // copies the new M into Q. So the wide pulse moves BOTH, in signal order [Q, M], where the
        // zero-width one moves neither.
        assert_eq!(
            keys(&cell),
            [
                ("CLK".to_string(), 'R', "Q".to_string()),
                ("CLK".to_string(), 'F', "Q,M".to_string()),
            ]
            .into_iter()
            .collect(),
        );
        // A D pulse is inert at either clock level: at CLK=1 nothing reads D at all, and at CLK=0 the
        // transparent master tracks D straight back to where it was, whichever cut closes the pulse.
        assert!(
            !cell.width_dependence.iter().any(|wd| wd.pin == "D"),
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
            [("E".to_string(), 'R', "Q".to_string())]
                .into_iter()
                .collect(),
        );
        assert_eq!(
            outcomes(on(&cell, "E", Edge::Rise)),
            settled(&[&[("Q", false)], &[("Q", true)]]),
        );
    }

    #[test]
    fn sr_latch_pulse_width_decides_the_pair_and_can_leave_it_ringing() {
        // Cross-NOR SR (the `examples/cells.toml` cell): asserting S from the reset state (S=0, R=0,
        // Q=0, Qn=1) opens a two-step cascade — Qn falls, then Q rises. Closing at the first cut lands
        // on the illegal both-low state under S=R=0, which rings (both rise, both fall, …): no
        // fixpoint. Closing at the second lands on the set state, which holds. So one pulse decides
        // between the reset state it started from, the set state, and a ringing pair.
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
        assert_eq!(
            keys(&cell),
            [
                ("S".to_string(), 'R', "Q,Qn".to_string()),
                ("R".to_string(), 'R', "Q,Qn".to_string()),
            ]
            .into_iter()
            .collect(),
        );
        let ringing = ["no fixpoint".to_string()].into_iter().collect();
        let both = |states: &[&[(&str, bool)]]| -> BTreeSet<String> {
            settled(states).union(&ringing).cloned().collect()
        };
        assert_eq!(
            outcomes(on(&cell, "S", Edge::Rise)),
            both(&[&[("Q", false), ("Qn", true)], &[("Q", true), ("Qn", false)],]),
        );
        // R↑ from the set state is the mirror image, over the same two settled states.
        assert_eq!(
            outcomes(on(&cell, "R", Edge::Rise)),
            both(&[&[("Q", false), ("Qn", true)], &[("Q", true), ("Qn", false)],]),
        );
        // The ringing cut is carried as an outcome of the width hazard and files no oscillation of its
        // own: the cell's only oscillation is still the simultaneous release confluence detects, whose
        // condition — S and R both low — is an input assignment the pair really does ring under, which
        // the pulse's returning edge is not.
        assert_eq!(
            cell.oscillation.len(),
            1,
            "the pulse cut adds no oscillation, got {:?}",
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
        // width of a release pulse decides which request ends up granted, or leaves the pair ringing.
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
        assert_eq!(
            keys(&cell),
            [
                ("A".to_string(), 'F', "Qa,Qb".to_string()),
                ("B".to_string(), 'F', "Qa,Qb".to_string()),
            ]
            .into_iter()
            .collect(),
        );
        let grants = |wd: &WidthDependence| {
            let mut expected = settled(&[
                &[("Qa", true), ("Qb", false)],
                &[("Qa", false), ("Qb", true)],
            ]);
            expected.insert("no fixpoint".to_string());
            assert_eq!(outcomes(wd), expected);
        };
        grants(on(&cell, "A", Edge::Fall));
        grants(on(&cell, "B", Edge::Fall));
        // A rise pulse from idle is inert: the grant it takes is handed straight back when the request
        // drops again, whichever cut closes the pulse.
        assert!(
            !cell.width_dependence.iter().any(|wd| wd.edge == Edge::Rise),
            "a request pulse from idle settles back to idle, got {:?}",
            keys(&cell)
        );
        // The interior cut's ring is an outcome of the width hazard, not a second oscillation: the cell
        // still detects exactly the one confluence records at A*B — the pair asserted together —
        // carrying its single pair-probe race and its two competing grants.
        assert_eq!(
            cell.oscillation.len(),
            1,
            "the pulse cut adds no oscillation, got {:?}",
            cell.oscillation
        );
        assert_eq!(cell.oscillation[0].group, ["Qa", "Qb"]);
        assert_eq!(cell.oscillation[0].condition_str(), "A*B");
        assert_eq!(cell.oscillation[0].races.len(), 1);
        assert_eq!(cell.oscillation[0].stable.len(), 2);
    }

    #[test]
    fn same_phase_cascade_pulse_width_decides_how_far_the_data_gets() {
        // Two latches transparent on the same clock phase (`examples/sequentials.toml`'s TCASC), so a
        // CLK-low pulse walks D through M and then M through Q, one stage per step. Three widths, three
        // outcomes: too narrow to move anything, wide enough for M alone, wide enough for both. Every
        // cut settles — a same-phase cascade has nothing to ring.
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
            [("CLK".to_string(), 'F', "Q,M".to_string())]
                .into_iter()
                .collect(),
        );
        let wd = on(&cell, "CLK", Edge::Fall);
        // Two reachable CLK-high states carry this hazard — one with D=1 over a cleared pair, one with
        // D=0 over a set pair — and the dedup keeps whichever is reached along the shorter walk. They
        // are mirror images, so the three outcomes are stated relative to the pre-pulse levels the
        // surviving representative holds: `M` alone moves (to D, which differs from it, else the pulse
        // would move nothing), and then `Q` follows it.
        let q0 = wd.state.value_of("Q").expect("a probed state is total");
        let m0 = wd.state.value_of("M").expect("a probed state is total");
        assert_eq!(
            outcomes(wd),
            settled(&[
                &[("Q", q0), ("M", m0)],
                &[("Q", q0), ("M", !m0)],
                &[("Q", !q0), ("M", !m0)],
            ]),
        );
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
        // CLK↓ key, and the first two decide strict subsets of the third's nodes, so the widest stands
        // for them and one hazard is reported on CLK↓.
        //
        // EN↑ is a key of its own and untouched by that: with CLK low, raising EN opens the second stage
        // for one step — Q takes M — and dropping EN again holds Q where the pulse left it.
        assert_eq!(
            keys(&cell),
            [
                ("CLK".to_string(), 'F', "Q,M".to_string()),
                ("EN".to_string(), 'R', "Q".to_string()),
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
        // hazards stand on the one CLK↓ key. SEL's own pulse opens one stage for its width while CLK is
        // low, and which stage that is IS the edge: SEL↑ decides A, SEL↓ decides B.
        assert_eq!(
            keys(&cell),
            [
                ("CLK".to_string(), 'F', "A".to_string()),
                ("CLK".to_string(), 'F', "B".to_string()),
                ("SEL".to_string(), 'R', "A".to_string()),
                ("SEL".to_string(), 'F', "B".to_string()),
            ]
            .into_iter()
            .collect(),
        );
    }

    #[test]
    fn combinational_cell_has_no_width_hazard() {
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
