//! Hazard **detection** over **confluence** of the asynchronous state machine. *Confluent* is term
//! rewriting's word — the Church–Rosser property — and here it ranges over the cell's settled
//! machine states, the operation being settling after toggling one input then the other, in each
//! order. [`detect`] probes what two closely-timed input edges do to the machine and files each
//! observation as a [`Hazard`] whose cause is a **race**, naming the pins whose transitions the
//! observation was made under. The timing that removes a detected hazard is generated downstream,
//! by `super::constraint`.
//!
//! A delay arc ([`super::arcs`]) records a single input edge that *causes* an output edge. A
//! **constraint** arc instead records that two inputs must not change too close together — a setup/hold
//! (data vs clock) or a non-sequential relation (two racing requests). Its origin is a detected hazard:
//! for a pair of near-simultaneous input edges the machine is **non-confluent** — the settled state
//! depends on which edge lands first — or oscillates outright, and either risks metastability.
//!
//! [`detect`] walks the fully-initialised reachable states — the same `Machine::arc_eligible`
//! measurement gate the arc derivation applies — and, for a stable state `s` and an unordered input pair
//! `{x, y}` (all other inputs held), settles `x` then `y` (`s_xy`) and `y` then `x` (`s_yx`). If either
//! oscillates or `s_xy == s_yx`, the pair is **confluent** at `s` — where the machine settles is
//! determined, so nothing is [`Outcome::Indeterminate`]. Otherwise the state has diverged, but global
//! divergence alone is not the verdict: it must *interact*
//! with the racing pair in the immediate combinational neighbourhood — some state variable `w` whose
//! value differs between `s_xy` and `s_yx` must have **both** `x` and `y` in the direct support of its
//! transition function `δ_w`. The model minimisation ([`super::minimise`]) composes through
//! combinational logic only — a state variable is kept as a variable, never substituted through — so
//! both pins in `δ_w`'s direct support means the pins meet within one combinational neighbourhood. A
//! divergence mediated only across a latch boundary — `δ_w` does not itself see both pins — is a settled
//! snapshot carried across that latch (e.g. the two domains of a dual-clock synchroniser),
//! design-tolerated rather than a pin-pair hazard. The filter gates the divergence alone — a race that
//! never settles is filed whatever its neighbourhood — and is what stops a mutex's grant divergence from
//! being reported.
//!
//! The same walk observes the other outcome: probed from `s`, the pair applied *simultaneously* (or,
//! degenerately, a single input toggle) can drive the state into a **periodic oscillation** rather than
//! a convergence point (`machine::settle_or_cycle` returning the cycle instead of settling), filed as
//! [`Outcome::Oscillation`] under the same racing cause. The two outcomes are independent readings of
//! one probe, so a pair that both diverges and never settles files a record for each, sharing the cause
//! rather than merging into one record. A lone toggle that never settles was observed under the one pin
//! it toggled, so its cause names that pin alone; a constraint separates two edges in time, so none
//! follows from it. A mutex resolves a race by design (that is its function as an arbiter), and the
//! record it detects is the oscillation at simultaneity, carrying on its own cause the racing
//! pins/edges the divergence-derived constraint (discarded by the combinational-neighbourhood filter)
//! would otherwise have supplied; ordinary settling of one request before the other is the normal,
//! hazard-free case.
//!
//! A record's own cause carries the racing pins and the edge each makes, which is what the separation
//! generated from it (`super::constraint`) is built out of.
//!
//! The reachable states and the prevector into `s` come from the shared `machine::explore`, the same
//! exploration the delay-arc BFS uses.
//!
//! **Every observation is reported**, under either outcome. A pair probed from ten reachable states that
//! diverges at each of them is ten [`Outcome::Indeterminate`] records, and one that rings at each of them
//! is ten [`Outcome::Oscillation`] records: this pass states what it observed and deduplicates, ranks and
//! selects nothing, and which of those observations a `define_arc` is rendered from is
//! [`crate::emit::arcs_tcl`]'s to decide.
//!
//! **Implementation notes** (concept in `hazard-detection.md`, not restated here): each reachable state's
//! per-input settle (`single`) is computed once and reused across every pair probe, so [`detect`] costs
//! O(n) settles per state rather than O(n²). States are probed in parallel and their per-state results
//! merged together; the merge is concatenation, which is associative, so the folded result holds the same
//! records however the work was split. Nothing reads the order the records come out in.

use std::collections::{BTreeMap, BTreeSet};

use rayon::prelude::*;

use espresso_logic::bdd::{Brand, ManagerCell};
use espresso_logic::{Minterm, Symbol};

use crate::logic::analysis::Machine;
use crate::logic::arcs::{ArcLevels, Edge};
use crate::logic::hazard::{Cause, Hazard, Outcome, Racer};
use crate::logic::machine;

/// The level each of `group`'s nodes holds at the probed state — what a constraint block states as the
/// start condition of each victim node it probes. A hazard's group holds state variables, which are machine
/// coordinates, and a probed state is fully initialised, so every one of them is defined there.
///
/// Shared with [`super::width`], which samples its pulses' nodes at the pre-pulse state through it.
pub(super) fn node_levels_at(state: &Minterm<Symbol>, group: &[Symbol]) -> BTreeMap<Symbol, bool> {
    group
        .iter()
        .map(|w| {
            let level = state
                .value_of(w.as_str())
                .expect("a hazard's group node is defined at the fully-initialised probed state");
            (w.clone(), level)
        })
        .collect()
}

/// The direction `name` toggles from its current value at `node`. Explored nodes carry a complete input
/// assignment, so an input's value is always fixed there.
///
/// Shared with [`super::width`], which reads a pulse's opening edge through it.
pub(super) fn edge_from(node: &Minterm<Symbol>, name: &str) -> Edge {
    if node
        .value_of(name)
        .expect("every input is fixed at an explored node")
    {
        Edge::Fall
    } else {
        Edge::Rise
    }
}

/// One racer of a detected race: the pin, and the edge it makes when toggled from `node`.
fn racer(node: &Minterm<Symbol>, pin: &Symbol) -> Racer {
    Racer {
        pin: pin.clone(),
        edge: edge_from(node, pin.as_str()),
    }
}

/// The detected hazards of one pass over the reachable state machine, split by the outcome observed —
/// every record carries [`Cause::Race`], the cause this pass probes for. No generated constraint is
/// nested here — `super::constraint` turns these into constraints downstream.
#[derive(Debug, Default)]
pub struct DetectedHazards {
    /// [`Outcome::Indeterminate`]: races whose settled state depends on which edge lands first — one
    /// record per observation, a pair diverging from several states filing one for each.
    pub(crate) order_dependence: Vec<Hazard>,
    /// [`Outcome::Oscillation`]: pairs (or single toggles) that drive a periodic, non-settling cycle —
    /// one record per observation, a pair ringing from several states filing one for each.
    pub(crate) oscillation: Vec<Hazard>,
}

/// Why every state value the hazard path reads is defined: a settle from a fully-initialised state
/// leaves every state column determinate. `machine`'s `step` evaluates each δ over the node's concrete
/// inputs and state values, and the minimise invariant I3 bounds a δ's support to the inputs plus the
/// state variables (asserted in `Machine::build`), so a total node steps to a total node. Every probe
/// starts from a state `Machine::arc_eligible` admits, so its whole trajectory — the singles, the two
/// orders and the simultaneous settle alike — is total, and this message is unreachable.
const DETERMINATE: &str =
    "a settle from a fully-initialised state leaves every state column determinate";

/// The state variables that oscillate across a `settle_or_cycle` cycle — those whose VALUE differs
/// between any two cycle nodes — in `state_vars` declaration order.
///
/// Shared with [`super::width`], which names the nodes of a pulse cut that never settles through it.
pub(super) fn oscillating_group(cycle: &[Minterm<Symbol>], state_vars: &[Symbol]) -> Vec<Symbol> {
    state_vars
        .iter()
        .filter(|v| {
            let mut vals = cycle
                .iter()
                .map(|m| m.value_of(v.as_str()).expect(DETERMINATE));
            let Some(first) = vals.next() else {
                return false;
            };
            vals.any(|val| val != first)
        })
        .cloned()
        .collect()
}

/// Detect a cell's hazards by re-walking its shared state machine ([`Machine`]) and testing pairwise
/// input-order confluence. Probes only the fully-initialised reachable stable states
/// (`Machine::arc_eligible`): a state carrying an uninitialised state variable is at an unknown state,
/// from which nothing can be concluded. Files a [`Hazard`] for every observation it makes —
/// [`Outcome::Indeterminate`] where the settle orders diverge, [`Outcome::Oscillation`] where the
/// machine never settles — but generates no constraint (that is `super::constraint`'s job). Empty for
/// confluent cells (ordinary combinational / self-holding gates that always settle) and for cells with
/// no state to latch. The input count empties only the PAIR probes: a single toggle races the cell's own
/// feedback and is probed however few inputs there are.
pub fn detect<B: Brand, C: ManagerCell + Send + Sync>(m: &Machine<B, C>) -> DetectedHazards {
    let cell = m.cell;
    let inputs = &cell.inputs;
    let n = inputs.len();

    let state_vars = &m.state_vars;
    let k = state_vars.len();
    if k == 0 {
        return DetectedHazards::default(); // no state to latch ⇒ always confluent
    }

    // Both coordinate halves, stepped together, exactly as the original exploration stepped them: a
    // combinational survivor is not excluded from settling just because nothing below reads its column.
    let deltas: Vec<machine::Delta<B, C>> = m.coordinate_deltas();
    // The direct support of every coordinate's δ — precomputed once, used by the
    // combinational-neighbourhood divergence filter below (see the module doc). Left over the merged
    // set rather than filtered down to the state variables: `support` is only ever INDEXED at a state
    // key (`support[w]` for a diverging state variable `w`), so a combinational entry sits unread —
    // harmless, and no guard is added to carve it back out.
    let support: BTreeMap<Symbol, BTreeSet<Symbol>> = deltas
        .iter()
        .map(|(n, d)| (n.clone(), d.variables().collect()))
        .collect();

    let ex = &m.explored;

    let settle_toggle =
        |node: &Minterm<Symbol>, names: &[&str]| -> Result<Minterm<Symbol>, Vec<Minterm<Symbol>>> {
            let toggled = machine::toggle(node, names);
            machine::settle_or_cycle(&deltas, &toggled)
        };

    // The per-state probe body: for one reachable state `s` (its BFS index `discovered`), settle every
    // single toggle and every unordered input pair, collecting this state's own records. Each state is
    // independent — the parallel unit — and the results are concatenated in the `reduce` below.
    //
    // Every observation this body makes is reported, under either outcome.
    let per_state = |(discovered, s): (usize, &Minterm<Symbol>)| -> (Vec<Hazard>, Vec<Hazard>) {
        debug_assert!(
            m.arc_eligible(s),
            "detect: a probe may only start from a fully-initialised state"
        );
        let mut order_dependence: Vec<Hazard> = Vec::new();
        let mut oscillation: Vec<Hazard> = Vec::new();

        // `path_to` depends only on `s`: compute the prevector into `s` once and clone it per hazard.
        let prevector_s = ex.path_to(s, inputs);
        // The output levels likewise depend only on `s`. Sampling them here — beside the prevector, at
        // the one probed state — is what keeps the two consistent wherever the record travels: a
        // record carries the levels of the very state its prevector walks to.
        let levels_s = ArcLevels::at(m, s);
        // The `when` every record of this state states: the standing input assignment the probed
        // transition happens FROM. It is a projection of `s`, so it depends only on `s` too — the pins a
        // probe toggles are the ones the emitted block writes as edges, and an edge is not part of the
        // condition it fires under.
        let condition_s = s.project_to(inputs);

        // Each input's single-toggle settle, computed once per state (O(n) instead of O(n²)): reused as
        // `r_x`/`r_y` across every pair and as the base of the `s_xy`/`s_yx` compositions below.
        let single: Vec<Result<Minterm<Symbol>, Vec<Minterm<Symbol>>>> = inputs
            .iter()
            .map(|x| settle_toggle(s, &[x.as_str()]))
            .collect();

        // Single-toggle capture: a lone input toggle that never settles is itself a non-settling
        // observation. The observation was made under one pin's transition, and the cause names it —
        // which is why a race's `pins` carries one member or two and is never empty. `settled` is empty:
        // there is no competing order for the machine to land in, and no constraint follows from it
        // either, a separation needing two edges to separate. Recorded once per input per state.
        for (i, r) in single.iter().enumerate() {
            if let Err(cycle) = r {
                let group = oscillating_group(cycle, state_vars);
                let node_levels = node_levels_at(s, &group);
                oscillation.push(Hazard {
                    cause: Cause::Race {
                        pins: vec![racer(s, &inputs[i])],
                    },
                    outcome: Outcome::Oscillation,
                    group,
                    condition: condition_s.clone(),
                    settled: Vec::new(),
                    prevector: prevector_s.clone(),
                    levels: levels_s.clone(),
                    node_levels,
                    state: s.clone(),
                    discovered,
                });
            }
        }

        // The pair probes proper, and the one thing here that needs two inputs: a race between two pins
        // has no second pin below this count. The single-toggle capture above is deliberately not gated
        // on it — a lone toggle rings around the cell's own feedback whatever its input count, and it is
        // the record every downstream claim about a ringing single toggle rests on.
        if n < 2 {
            return (order_dependence, oscillation);
        }

        for i in 0..n {
            for j in (i + 1)..n {
                let x = &inputs[i];
                let y = &inputs[j];

                let r_x = &single[i];
                let r_y = &single[j];

                // Compose both settle orders once per pair: x-then-y (`s_xy`) and y-then-x (`s_yx`). Each
                // is `Some` only when its base single settles and the second toggle settles too. Reused by
                // the settled set an oscillating simultaneous probe reports and by the divergence check.
                let s_xy = r_x
                    .as_ref()
                    .ok()
                    .and_then(|sx| settle_toggle(sx, &[y.as_str()]).ok());
                let s_yx = r_y
                    .as_ref()
                    .ok()
                    .and_then(|sy| settle_toggle(sy, &[x.as_str()]).ok());

                // Simultaneous probe: x and y toggled together. Oscillation here is the mutex/arbiter
                // case proper — the pair asserted at once, driving the state into a periodic cycle.
                let r_sim = settle_toggle(s, &[x.as_str(), y.as_str()]);
                if let Err(cycle) = &r_sim {
                    let group = oscillating_group(cycle, state_vars);
                    let mut settled_set: BTreeSet<Minterm<Symbol>> = BTreeSet::new();
                    if let Some(sxy) = &s_xy {
                        settled_set.insert(sxy.project_to(&group));
                    }
                    if let Some(syx) = &s_yx {
                        settled_set.insert(syx.project_to(&group));
                    }

                    // The record carries the racing pins/edges (taken at `s`) and the prevector the
                    // constraint generated from it needs. This is what supplies an oscillating pair's
                    // (e.g. a mutex's) constraint, standing in for the divergence-derived one the
                    // combinational-neighbourhood filter below discards for it.
                    let node_levels = node_levels_at(s, &group);
                    oscillation.push(Hazard {
                        cause: Cause::Race {
                            pins: vec![racer(s, x), racer(s, y)],
                        },
                        outcome: Outcome::Oscillation,
                        group,
                        condition: condition_s.clone(),
                        settled: settled_set.into_iter().collect(),
                        prevector: prevector_s.clone(),
                        levels: levels_s.clone(),
                        node_levels,
                        state: s.clone(),
                        discovered,
                    });
                }

                let (Some(s_xy), Some(s_yx)) = (s_xy.as_ref(), s_yx.as_ref()) else {
                    continue; // a toggle in one of the two orders oscillates → confluent (no hazard)
                };
                if s_xy == s_yx {
                    continue; // confluent at this state — no hazard
                }

                // Does `w` hold a different value in the two settle orders? Both are total (see
                // `DETERMINATE`), so this is a comparison of values, not of definedness.
                let diverges = |w: &Symbol| {
                    s_xy.value_of(w.as_str()).expect(DETERMINATE)
                        != s_yx.value_of(w.as_str()).expect(DETERMINATE)
                };

                // Global divergence is not enough: it must interact with {x, y} in the immediate
                // combinational neighbourhood — some state variable that actually diverges between the
                // two settle orders must have BOTH x and y in the direct support of its own δ. Otherwise
                // the divergence is a settled snapshot mediated across a latch boundary (e.g. a
                // dual-clock synchroniser's two domains), not a pin-pair hazard.
                let interacts = state_vars.iter().any(|w| {
                    diverges(w)
                        && support[w].contains(x.as_str())
                        && support[w].contains(y.as_str())
                });
                if !interacts {
                    continue; // divergence real but latch-mediated — no hazard
                }

                // Non-confluent and interacting ⇒ the race settles indeterminately: the divergent state
                // variables and their two competing settled outcomes, from the input assignment the
                // pair races out of. The constraint generated from it ([`super::constraint`]) has its
                // kind decided there, solely by the declared clock, since the hazard is a property of
                // the cell rather than of the declaration.
                let group: Vec<Symbol> =
                    state_vars.iter().filter(|w| diverges(w)).cloned().collect();
                let node_levels = node_levels_at(s, &group);
                let mut settled_set: BTreeSet<Minterm<Symbol>> = BTreeSet::new();
                settled_set.insert(s_xy.project_to(&group));
                settled_set.insert(s_yx.project_to(&group));
                order_dependence.push(Hazard {
                    cause: Cause::Race {
                        pins: vec![racer(s, x), racer(s, y)],
                    },
                    outcome: Outcome::Indeterminate,
                    group,
                    condition: condition_s.clone(),
                    settled: settled_set.into_iter().collect(),
                    prevector: prevector_s.clone(),
                    levels: levels_s.clone(),
                    node_levels,
                    state: s.clone(),
                    discovered,
                });
            }
        }

        (order_dependence, oscillation)
    };

    // Probe every fully-initialised reachable state in parallel, then fold the per-state results
    // together. The filter comes AFTER `enumerate`, so `discovered` stays the BFS index of the state —
    // the key a downstream reader tells two observations apart by — rather than a position in the
    // filtered sequence. The merge concatenates both halves, which is associative, so the folded result
    // holds the same records regardless of state/thread order.
    let (order_dependence, oscillation) = ex
        .order
        .par_iter()
        .enumerate()
        .filter(|(_, s)| m.arc_eligible(s))
        .map(per_state)
        .reduce(
            || (Vec::new(), Vec::new()),
            |(mut oa, mut osca), (mut ob, mut oscb)| {
                oa.append(&mut ob);
                osca.append(&mut oscb);
                (oa, osca)
            },
        );

    DetectedHazards {
        order_dependence,
        oscillation,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::logic::constraint::{Constraint, ConstraintKind};
    use crate::model::{analyse_one as analyse, AnalysedCell};

    /// The separations generated from this pass's records, picked out of the cell's one `constraints`
    /// list by their kind: the minimum pulse widths sharing that list come from pulse-cause hazards.
    fn separations(cell: &AnalysedCell) -> Vec<&Constraint> {
        cell.constraints
            .iter()
            .filter(|c| !matches!(c.kind, ConstraintKind::MinPulseWidth))
            .collect()
    }

    /// The pin a separation holds its own constrained pin apart from, and the edge that pin makes. A
    /// minimum pulse width relates a pin to itself and names no second one, so it reaches here only
    /// through a filtering fault.
    fn related(c: &Constraint) -> (&str, Edge) {
        match &c.kind {
            ConstraintKind::SetupHold { clock, clock_edge } => (clock.as_str(), *clock_edge),
            ConstraintKind::NonSeq { other, other_edge } => (other.as_str(), *other_edge),
            ConstraintKind::MinPulseWidth => {
                panic!("a minimum pulse width came through the separation filter")
            }
        }
    }

    /// The two pins a separation holds apart, sorted, so a test pins which pins it relates rather than
    /// which side of it each landed on.
    fn apart(c: &Constraint) -> Vec<&str> {
        let mut pins = vec![related(c).0, c.pin.as_str()];
        pins.sort();
        pins
    }

    /// The pins a detected race names. Every record this pass files is caused by a race — it probes
    /// input pairs and single toggles, never a pulse — so the pulse arm cannot arise here.
    fn racing_pins(hz: &Hazard) -> &[Racer] {
        match &hz.cause {
            Cause::Race { pins } => pins,
            Cause::Pulse { .. } => unreachable!("confluence detects a race, never a pulse"),
        }
    }

    /// The pins a detected hazard races, sorted: a race is unordered, so a test pins which pins are in
    /// it, not the order the probe happened to take them in.
    fn racing(hz: &Hazard) -> Vec<&str> {
        let mut pins: Vec<&str> = racing_pins(hz).iter().map(|r| r.pin.as_str()).collect();
        pins.sort();
        pins
    }

    /// The cell's detected races settling under `outcome`. A cell carries one hazard list spanning both
    /// causes, so what this pass detected is read back off it by both axes: the racing cause it probes
    /// for, and the outcome the test is about.
    fn races(cell: &AnalysedCell, outcome: Outcome) -> Vec<&Hazard> {
        cell.hazards
            .iter()
            .filter(|hz| matches!(hz.cause, Cause::Race { .. }) && hz.outcome == outcome)
            .collect()
    }

    #[test]
    fn dff_with_declared_clock_yields_only_setup_hold() {
        // Rising-edge DFF with CLK declared a clock: the CLK↔D hazard yields a setup/hold constraint of
        // D w.r.t. CLK, and — because the kind follows the declared clock, not the geometry — nothing on
        // the pair is reported as non_seq.
        let cell = analyse(
            r#"
[[cell]]
name = "DFF"
inputs = ["CLK", "D"]
clock = ["CLK"]
constraint_arcs = true
[cell.internal]
M = "!CLK*D + CLK*M"
[cell.outputs]
Q = "CLK*M + !CLK*Q"
"#,
        );
        // The DFF detects a CLK/D race settling indeterminately.
        let indeterminate = races(&cell, Outcome::Indeterminate);
        assert!(
            indeterminate.iter().any(|hz| racing(hz) == ["CLK", "D"]),
            "expected an indeterminate CLK/D race, got {indeterminate:?}"
        );
        let oscillating = races(&cell, Outcome::Oscillation);
        assert!(
            oscillating.is_empty(),
            "a DFF detects no oscillation, got {oscillating:?}"
        );
        // …from which a setup/hold constraint of D w.r.t. CLK is generated; because the kind follows the
        // declared clock, not the geometry, nothing on the pair is generated as non_seq.
        let cons = separations(&cell);
        eprintln!("DFF constraints: {cons:#?}");
        assert!(
            cons.iter()
                .all(|c| matches!(c.kind, ConstraintKind::SetupHold { .. })),
            "a declared-clock DFF yields only setup/hold, got {cons:?}"
        );
        assert!(
            cons.iter()
                .any(|c| related(c) == ("CLK", Edge::Rise) && c.pin == "D"),
            "expected a setup/hold of D around CLK↑, got {cons:?}"
        );
    }

    #[test]
    fn dff_without_declared_clock_is_non_seq() {
        // The same DFF with no clock declared: the hazard is real but, with no clock to designate a data
        // pin, its constraint is a symmetric non_seq — the kind is a property of the declaration, not
        // the cell.
        let cell = analyse(
            r#"
[[cell]]
name = "DFF"
inputs = ["CLK", "D"]
constraint_arcs = true
[cell.internal]
M = "!CLK*D + CLK*M"
[cell.outputs]
Q = "CLK*M + !CLK*Q"
"#,
        );
        let cons = separations(&cell);
        assert!(!cons.is_empty());
        assert!(
            cons.iter()
                .all(|c| matches!(c.kind, ConstraintKind::NonSeq { .. })),
            "an undeclared DFF yields only non_seq, got {cons:?}"
        );
    }

    #[test]
    fn mutex_has_non_seq_between_requests() {
        // Cross-coupled mutex: A and B race symmetrically. Their order-divergence is on the coupled
        // grant outputs (Qa/Qb), neither of which has *both* A and B in its own δ's direct support, so
        // the combinational-neighbourhood filter discards it — the mutex detects no indeterminate race.
        // But the simultaneous A*B toggle drives the state into an oscillation, and the racing pins on
        // that record supply the pair's generated non_seq constraint.
        let cell = analyse(
            r#"
[[cell]]
name = "MUT"
inputs = ["A", "B"]
constraint_arcs = true
[cell.outputs]
Qa = "!Qb * A"
Qb = "!Qa * B"
"#,
        );
        // Detects exactly one oscillation — the A/B pair probed together — and no indeterminate race.
        let oscillating = races(&cell, Outcome::Oscillation);
        assert_eq!(
            oscillating.len(),
            1,
            "expected one oscillation, got {oscillating:?}"
        );
        assert_eq!(
            racing(oscillating[0]),
            ["A", "B"],
            "the record's own cause names the racing pair, got {:?}",
            oscillating[0].cause
        );
        let indeterminate = races(&cell, Outcome::Indeterminate);
        assert!(
            indeterminate.is_empty(),
            "a mutex detects no indeterminate race, got {indeterminate:?}"
        );
        let cons = separations(&cell);
        eprintln!("MUT constraints: {cons:#?}");
        assert!(
            cons.iter()
                .any(|c| matches!(c.kind, ConstraintKind::NonSeq { .. }) && apart(c) == ["A", "B"]),
            "expected a non_seq constraint between A and B, got {cons:?}"
        );
        assert!(
            cons.iter()
                .all(|c| matches!(c.kind, ConstraintKind::NonSeq { .. })),
            "a mutex yields only non_seq constraints, got {cons:?}"
        );
    }

    #[test]
    fn c_element_has_non_seq_constraint() {
        // A C-element is order-sensitive: A↓ racing B↑ leaves Q history-dependent. The race settles
        // indeterminately (it does settle), and from that a non_seq constraint between A and B is
        // generated.
        let cell = analyse(
            r#"
[[cell]]
name = "C2"
inputs = ["A", "B"]
constraint_arcs = true
[cell.outputs]
Q = "A*B + Q*(A+B)"
"#,
        );
        // Detects an indeterminate A/B race, and no oscillation.
        let indeterminate = races(&cell, Outcome::Indeterminate);
        assert!(
            indeterminate.iter().any(|hz| racing(hz) == ["A", "B"]),
            "expected an indeterminate A/B race, got {indeterminate:?}"
        );
        let oscillating = races(&cell, Outcome::Oscillation);
        assert!(
            oscillating.is_empty(),
            "a C-element detects no oscillation, got {oscillating:?}"
        );
        let cons = separations(&cell);
        eprintln!("C2 constraints: {cons:#?}");
        assert!(
            cons.iter()
                .any(|c| matches!(c.kind, ConstraintKind::NonSeq { .. }) && apart(c) == ["A", "B"]),
            "expected a non_seq constraint between A and B, got {cons:?}"
        );
    }

    #[test]
    fn a_pair_endangering_different_nodes_yields_a_constraint_each() {
        // A dual-clock mux: `Q = CLKA*MA + CLKB*MB + !CLKA*!CLKB*Q`. With CLKA high the A latch is
        // transparent, so `MA = DA` reaches Q — and that decides whether MB's divergence does too.
        // Racing CLKB against DB endangers MB alone where DA holds Q at 1, and both Q and MB where DA
        // is 0. Same pins, same edges, different nodes at risk: two hazards, each characterised from
        // its own pre-hazard state rather than collapsed onto whichever was reached first.
        let cell = analyse(
            r#"
[[cell]]
name = "DCMUX"
inputs = ["CLKA", "CLKB", "DA", "DB"]
clock = ["CLKA", "CLKB"]
constraint_arcs = true
[cell.internal]
MA = "!CLKA*DA + CLKA*MA"
MB = "!CLKB*DB + CLKB*MB"
[cell.outputs]
Q = "CLKA*MA + CLKB*MB + !CLKA*!CLKB*Q"
"#,
        );
        let mut endangered: Vec<Vec<&str>> = separations(&cell)
            .into_iter()
            .filter(|c| related(c).0 == "CLKB" && c.pin.as_str() == "DB")
            .map(|c| c.nodes.iter().map(|p| p.node.as_str()).collect())
            .collect();
        endangered.sort();
        endangered.dedup();
        assert!(
            endangered.contains(&vec!["MB"]) && endangered.contains(&vec!["Q", "MB"]),
            "both node sets are constrained, got {endangered:?}",
        );
    }

    #[test]
    fn constraint_prevector_lengths_are_minimal() {
        // Verifies that the constraint prevectors have minimal length as a consequence of BFS exploration
        // order. The exploration finds shortest paths first because it examines all states at distance d
        // before distance d+1 from a seed. The minimum is measured over fully-initialised probed states
        // (`Machine::arc_eligible`), so a cell whose seeds leave a state variable undriven measures from
        // further along its BFS. Re-capture only for a deliberate algorithm change.
        let dff = analyse(
            r#"
[[cell]]
name = "DFF"
inputs = ["CLK", "D"]
clock = ["CLK"]
constraint_arcs = true
[cell.internal]
M = "!CLK*D + CLK*M"
[cell.outputs]
Q = "CLK*M + !CLK*Q"
"#,
        );
        let mut dff_lens: Vec<usize> = separations(&dff)
            .iter()
            .map(|c| c.prevector.len())
            .collect();
        dff_lens.sort();
        // Every DFF seed sits at CLK=0, where δ_M = !CLK*D + CLK*M forces M but δ_Q = CLK*M + !CLK*Q
        // holds Q: Q is undriven there, so no probe starts at a seed. The shortest eligible state a
        // CLK↑ probe can start from is three input states along — CLK low, a pulse that drives Q, and
        // CLK low again.
        //
        // Two lengths per race, because a constraint is keyed on the state it is probed from and the
        // flop stands at CLK=0 with the master already at D in TWO states — the one whose slave still
        // holds the old value and the one that has taken the new. Each is its own constraint, and the
        // second is one input state further along: the walk that flips Q.
        assert_eq!(dff_lens, vec![3, 3, 4, 4]);

        let c2 = analyse(
            r#"
[[cell]]
name = "C2"
inputs = ["A", "B"]
constraint_arcs = true
[cell.outputs]
Q = "A*B + Q*(A+B)"
"#,
        );
        let mut c2_lens: Vec<usize> = separations(&c2).iter().map(|c| c.prevector.len()).collect();
        c2_lens.sort();
        // C2's single state variable is forced at both seeds, so every state in its explored order is
        // eligible and the minimum is the BFS distance alone. Four constraints at that one distance: the
        // A↓/B↑ race and its mirror, each probed from both states the hold region carries — δ_Q = Q with
        // exactly one request up, so Q high and Q low are both stable under that input assignment.
        assert_eq!(c2_lens, vec![2, 2, 2, 2]);
    }

    #[test]
    fn constraint_levels_travel_with_the_representative_prevector() {
        // The levels and the prevector are sampled at the SAME probed state, so the constraint the
        // situation collapse keeps carries a consistent pair: each generated constraint matches one
        // detected hazard on BOTH, never a mix of two states.
        let cell = analyse(
            r#"
[[cell]]
name = "DFF"
inputs = ["CLK", "D"]
clock = ["CLK"]
constraint_arcs = true
[cell.internal]
M = "!CLK*D + CLK*M"
[cell.outputs]
Q = "CLK*M + !CLK*Q"
"#,
        );
        let cons = separations(&cell);
        assert!(!cons.is_empty());
        let outputs: Vec<Symbol> = cell.outputs.iter().map(|o| o.name.clone()).collect();
        for c in &cons {
            assert_eq!(
                c.levels
                    .outputs
                    .iter()
                    .map(|(n, _)| n.clone())
                    .collect::<Vec<_>>(),
                outputs
            );
            assert!(
                races(&cell, Outcome::Indeterminate)
                    .iter()
                    .any(|hz| hz.prevector == c.prevector && hz.levels == c.levels),
                "constraint {c:?} must carry one probed state's prevector AND its levels"
            );
        }
    }

    #[test]
    fn sr_latch_has_non_seq_constraint() {
        // The SR latch's simultaneous release (11→00) is a real order-hazard, filed as a non_seq S↔R.
        let cell = analyse(
            r#"
[[cell]]
name = "SR"
inputs = ["S", "R"]
constraint_arcs = true
[cell.outputs]
Q = "S + Q*!R"
Qn = "R + Qn*!S"
"#,
        );
        let cons = separations(&cell);
        eprintln!("SR constraints: {cons:#?}");
        assert!(
            cons.iter()
                .any(|c| matches!(c.kind, ConstraintKind::NonSeq { .. })),
            "expected a non_seq constraint between S and R, got {cons:?}"
        );
    }

    #[test]
    fn latch_mediated_divergence_is_not_a_constraint() {
        // Two-domain sampling chain: M1 (transparent when C1 is low) samples D; Q (transparent when C2
        // is low) samples M1. No clocks declared, so every derived constraint here is NonSeq. A (C1, C2)
        // order-divergence is real (e.g. whether Q ends up latching M1's old value or D's new one
        // depends on whether C2 or C1 closes first) but is mediated only across the M1↔Q latch
        // boundary: neither δ_M1 (support {C1, D, M1}) nor δ_Q (support {C2, M1, Q}) has both C1 and C2
        // in its own direct support, so it must be filtered. The (C1, D) hazard is direct — δ_M1 has
        // both C1 and D — and must survive.
        let cell = analyse(
            r#"
[[cell]]
name = "SYNC2"
inputs = ["C1", "C2", "D"]
constraint_arcs = true
[cell.internal]
M1 = "!C1*D + C1*M1"
[cell.outputs]
Q = "!C2*M1 + C2*Q"
"#,
        );
        // The (C1, C2) divergence is latch-mediated, so it is filtered at detection — no C1/C2 race is
        // reported.
        let indeterminate = races(&cell, Outcome::Indeterminate);
        assert!(
            !indeterminate.iter().any(|hz| racing(hz) == ["C1", "C2"]),
            "the C1/C2 divergence is latch-mediated and must be filtered, got {indeterminate:?}"
        );
        let cons = separations(&cell);
        eprintln!("SYNC2 constraints: {cons:#?}");
        assert!(
            !cons.iter().any(|c| apart(c) == ["C1", "C2"]),
            "the C1/C2 divergence is latch-mediated and must be filtered, got {cons:?}"
        );
        assert!(
            cons.iter().any(|c| apart(c) == ["C1", "D"]),
            "expected a constraint for the genuine C1/D hazard (direct support of δ_M1), got {cons:?}"
        );
    }

    #[test]
    fn combinational_has_no_constraints() {
        // Opts in, so generation actually runs — the empty result is the confluence analysis finding no
        // hazard to constrain, not the per-cell gate skipping generation.
        let cell = analyse(
            r#"
[[cell]]
name = "ND2"
inputs = ["A", "B"]
constraint_arcs = true
[cell.outputs]
Y = "!(A*B)"
"#,
        );
        assert!(cell.constraints.is_empty());
        assert!(races(&cell, Outcome::Indeterminate).is_empty());
        assert!(races(&cell, Outcome::Oscillation).is_empty());
    }
}
