//! Hazard **detection** and constraint **generation** over **confluence** of the asynchronous state
//! machine. Detection ([`detect`]) finds the two hazards two closely-timed input edges can create;
//! constraint generation (`constrain`) turns each detected hazard into the timing separation that
//! removes it. Detection happens first; constraint generation follows from each detected hazard.
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
//! oscillates or `s_xy == s_yx`, the pair is **confluent** at `s` — no order-dependent hazard.
//! Otherwise the state has diverged, but global divergence alone is not the verdict: it must *interact*
//! with the racing pair in the immediate combinational neighbourhood — some state variable `w` whose
//! value differs between `s_xy` and `s_yx` must have **both** `x` and `y` in the direct support of its
//! transition function `δ_w`. The model minimisation ([`super::minimise`]) composes through
//! combinational logic only — a state variable is kept as a variable, never substituted through — so
//! both pins in `δ_w`'s direct support means the pins meet within one combinational neighbourhood. A
//! divergence mediated only across a latch boundary — `δ_w` does not itself see both pins — is a settled
//! snapshot carried across that latch (e.g. the two domains of a dual-clock synchroniser),
//! design-tolerated rather than a pin-pair hazard. This filter is what stops a mutex's grant divergence
//! from being reported as an order-dependent hazard.
//!
//! The same walk detects an **oscillation hazard**: probed from `s`, the pair applied *simultaneously*
//! (or, degenerately, a single input toggle) can drive the state into a **periodic oscillation** rather
//! than a fixpoint ([`machine::settle_or_cycle`] returning the cycle instead of settling). That is
//! reported as an [`Oscillation`]. A mutex is order-dependent by design (that is its function as an
//! arbiter); the hazard it *detects* is the oscillation at simultaneity; ordinary settling of one
//! request before the other is the normal, hazard-free case — and each pair-probe observation records a
//! [`Race`] so the generated
//! constraint has the racing pins/edges its divergence-derived constraint (discarded by the
//! combinational-neighbourhood filter) would otherwise have supplied.
//!
//! `constrain` then generates one [`Constraint`] per detected hazard. A constraint's **kind is decided
//! solely by the declared clock**: a pair containing exactly one declared clock is a directed
//! **setup/hold** (clock ← data — the DFF's `D` around `CLK`); any other pair is a symmetric **non_seq**
//! (a mutex's `A`/`B`, a C-element's `A↓`/`B↑`, an SR latch's simultaneous release). Clocks are
//! *declared* inputs; the race geometry is left out of the decision because inferring a clock from race
//! order would be state-dependent — the same pins read one way from one held state and the other way
//! from another — so it would distinguish nothing real.
//!
//! The reachable states and the prevector into `s` come from the shared [`machine::explore`], the same
//! exploration the delay-arc BFS uses.
//!
//! **Implementation notes** (concept in `hazard-detection.md`, not restated here): each reachable state's
//! per-input settle (`single`) is computed once and reused across every pair probe, so [`detect`] costs
//! O(n) settles per state rather than O(n²). States are probed in parallel and their per-state dedup maps
//! merged together; the merge is order-independent. [`detect`]'s `order_dependence` dedup and
//! `constrain`'s own [`Constraint`] dedup (`constraint_key`) both keep the min
//! `(prevector.len, discovered)` representative per canonical key — a total order, so the surviving entry
//! is fixed regardless of merge order. [`detect`]'s `oscillation` dedup instead keeps an arbitrary colliding
//! representative — on a collision `group`/`condition` coincide (the key is injective in them) but
//! `stable` does not, so it UNIONS `stable` as a set (collision-order-independent) and UNIONS the
//! colliding pair-probe [`Race`]s — races are never dropped. All three dedup maps are [`BTreeMap`]s,
//! so iteration order — and hence report/emission order — is deterministic independent of any hash map's
//! order. A fold may only gain a constraint, never lose one.

use std::collections::{BTreeMap, BTreeSet};

use rayon::prelude::*;

use espresso_logic::bdd::{Brand, ManagerCell};
use espresso_logic::{Minterm, Symbol};

use crate::logic::analysis::Machine;
use crate::logic::arcs::{ArcLevels, Edge};
use crate::logic::hazard::{OrderDependence, Oscillation, Race};
use crate::logic::machine;

/// The kind of a constraint arc: a directed setup/hold (clock ← data) or a symmetric non-sequential
/// (oscillation / mutual-exclusion) relation between two request inputs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConstraintKind {
    SetupHold,
    NonSeq,
}

/// One constraint arc between two **primary inputs**. For [`ConstraintKind::SetupHold`], `related` is
/// the clock and `pin` the data pin; for [`ConstraintKind::NonSeq`], the two are symmetric requests.
#[derive(Debug, Clone)]
pub struct Constraint {
    pub kind: ConstraintKind,
    pub related: Symbol,
    pub related_edge: Edge,
    pub pin: Symbol,
    pub pin_edge: Edge,
    /// The prevector: the input-assignment path that drives every state variable into the state where
    /// the constraint manifests (each node projected onto the inputs).
    pub prevector: Vec<Minterm<Symbol>>,
    /// The levels the cell's outputs hold in that state — the constraint arc's `-ic` initial condition,
    /// sampled at the same probed state as `prevector`.
    pub levels: ArcLevels,
    /// The nodes this constraint protects, each with the level it holds at the probed state: the state
    /// variables whose settled value the hazard puts at risk — a flop's master latch, for the setup
    /// constraint that separates its clock from its data — in signal declaration order. The emitted
    /// block gives each a column of its own and names them all in one Liberate `-probe`, so the
    /// characterisation measures the nodes the constraint is actually about.
    pub nodes: Vec<(Symbol, bool)>,
}

impl Constraint {
    /// The input condition under which the hazard this constraint avoids occurs: the two switching
    /// edges, plus any other inputs held at a fixed value in the pre-toggle state (e.g. `A↓ & B↑ with
    /// R=0`). `path_to` seeds its chain with the probed node itself, so `prevector` always names at
    /// least that state's held inputs.
    pub fn condition(&self) -> String {
        let mut cond = format!(
            "{}{} & {}{}",
            self.related,
            self.related_edge.arrow(),
            self.pin,
            self.pin_edge.arrow()
        );
        let state = self
            .prevector
            .last()
            .expect("path_to seeds its chain with the probed node itself");
        let others = crate::logic::fixed_pairs(state, &[self.related.as_str(), self.pin.as_str()]);
        if !others.is_empty() {
            cond.push_str(&format!(" with {}", others.join(", ")));
        }
        cond
    }
}

/// What a hazard observation sampled at the state it was probed from, which a constraint carries
/// forward as one: the walk into that state, the levels the cell's pins hold there, and the nodes the
/// hazard puts at risk with the level each holds. All three are read at the one state, so they travel
/// together — a surviving representative carries the sample of the very state its prevector walks to.
struct Probed {
    prevector: Vec<Minterm<Symbol>>,
    levels: ArcLevels,
    nodes: Vec<(Symbol, bool)>,
}

/// The nodes a hazard puts at risk, each with the level the observation sampled for it. An observation
/// samples every node of the group it was recorded with — `record_oscillation` keys on that group, so a
/// race only ever joins an oscillation naming the same nodes — so every entry is there.
fn protected(group: &[Symbol], levels: &BTreeMap<Symbol, bool>) -> Vec<(Symbol, bool)> {
    group
        .iter()
        .map(|node| {
            let level = *levels
                .get(node)
                .expect("a hazard observation samples every node of its own group");
            (node.clone(), level)
        })
        .collect()
}

/// The level each of `group`'s nodes holds at the probed state — what a constraint block states as the
/// start condition of the node it protects. A hazard's group holds state variables, which are machine
/// coordinates, and a probed state is fully initialised, so every one of them is defined there.
fn node_levels_at(state: &Minterm<Symbol>, group: &[Symbol]) -> BTreeMap<Symbol, bool> {
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
fn edge_from(node: &Minterm<Symbol>, name: &str) -> Edge {
    if node
        .value_of(name)
        .expect("every input is fixed at an explored node")
    {
        Edge::Fall
    } else {
        Edge::Rise
    }
}

/// A canonical dedup key: setup/hold is directed; non_seq is unordered over its two pins.
/// A constraint's protected nodes as one key fragment, in their own order.
fn names_of(nodes: &[(Symbol, bool)]) -> String {
    nodes
        .iter()
        .map(|(n, _)| n.as_str())
        .collect::<Vec<_>>()
        .join(",")
}

fn constraint_key(c: &Constraint) -> String {
    match c.kind {
        ConstraintKind::SetupHold => format!(
            "SH|{}{}|{}{}|{}",
            c.related,
            c.related_edge.rf(),
            c.pin,
            c.pin_edge.rf(),
            names_of(&c.nodes)
        ),
        ConstraintKind::NonSeq => {
            let a = format!("{}{}", c.related, c.related_edge.rf());
            let b = format!("{}{}", c.pin, c.pin_edge.rf());
            let (lo, hi) = if a <= b { (a, b) } else { (b, a) };
            format!("NS|{lo}|{hi}|{}", names_of(&c.nodes))
        }
    }
}

/// The detected hazards of one pass over the reachable state machine: the two shapes the metastability
/// risk takes, reported symmetrically. No generated constraint is nested here — `constrain` turns
/// these into [`Constraint`]s downstream.
#[derive(Debug, Default)]
pub struct DetectedHazards {
    /// Order-dependent hazards: pairs whose settled state depends on which edge lands first.
    pub order_dependence: Vec<OrderDependence>,
    /// Oscillation hazards: pairs (or single toggles) that drive a periodic, non-settling cycle.
    pub oscillation: Vec<Oscillation>,
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
fn oscillating_group(cycle: &[Minterm<Symbol>], state_vars: &[Symbol]) -> Vec<Symbol> {
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
/// from which nothing can be concluded. Produces the two detected hazards symmetrically —
/// [`OrderDependence`] and [`Oscillation`] — but generates no constraint (that is `constrain`'s job).
/// Empty for confluent cells (ordinary combinational / self-holding gates without oscillation) and for
/// cells with too few inputs or no state to latch.
pub fn detect<B: Brand, C: ManagerCell + Send + Sync>(m: &Machine<B, C>) -> DetectedHazards {
    let cell = m.cell;
    let inputs = &cell.inputs;
    let n = inputs.len();
    if n < 2 {
        return DetectedHazards::default(); // a hazard relates two inputs
    }

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
    // single toggle and every unordered input pair, filling this state's own dedup maps. Each state is
    // independent — the parallel unit — and the maps merge commutatively in the `reduce` below.
    //
    // `order_dependence` deduplicates by its unordered `(pin,edge)|(pin,edge)` key, keeping the min
    // `(prevector.len, discovered)` representative; `oscillation` deduplicates by `group|condition`,
    // keeping the incumbent representative while appending every colliding pair-probe [`Race`]. Both are
    // BTreeMaps, so the final iteration order is deterministic regardless of merge order.
    let per_state = |(discovered, s): (usize, &Minterm<Symbol>)| -> (
        BTreeMap<String, OrderDependence>,
        BTreeMap<String, Oscillation>,
    ) {
        debug_assert!(
            m.arc_eligible(s),
            "detect: a probe may only start from a fully-initialised state"
        );
        let mut order_dependence: BTreeMap<String, OrderDependence> = BTreeMap::new();
        let mut oscillation: BTreeMap<String, Oscillation> = BTreeMap::new();

        // `path_to` depends only on `s`: compute the prevector into `s` once and clone it per hazard.
        let prevector_s = ex.path_to(s, inputs);
        // The output levels likewise depend only on `s`. Sampling them here — beside the prevector, at
        // the one probed state — is what keeps the two consistent through `record_constraint`'s
        // min-by-`(prevector.len, discovered)` dedup: a surviving representative carries the levels of
        // the very state its prevector walks to.
        let levels_s = ArcLevels::at(m, s);

        // Each input's single-toggle settle, computed once per state (O(n) instead of O(n²)): reused as
        // `r_x`/`r_y` across every pair and as the base of the `s_xy`/`s_yx` compositions below.
        let single: Vec<Result<Minterm<Symbol>, Vec<Minterm<Symbol>>>> = inputs
            .iter()
            .map(|x| settle_toggle(s, &[x.as_str()]))
            .collect();

        // Single-toggle oscillation capture: a lone input toggle that never settles is itself an
        // oscillation (no competing order to report — `stable` is empty, and no [`Race`] is appended:
        // one toggle names no racing pair, so it generates no constraint). Recorded once per input per
        // state. Its `group|condition` key shares the report key space with the simultaneous-pair
        // observations below, so a colliding pair-probe [`Race`] is appended to the surviving entry
        // (append-never-drop), never dropped; the reported representative is an arbitrary equal-quality
        // choice, made when merging per-state maps — only the races are exhaustively unioned across
        // states.
        for (i, r) in single.iter().enumerate() {
            if let Err(cycle) = r {
                let group = oscillating_group(cycle, state_vars);
                record_oscillation(
                    &mut oscillation,
                    inputs,
                    s,
                    &[inputs[i].as_str()],
                    group,
                    Vec::new(),
                    None,
                );
            }
        }

        for i in 0..n {
            for j in (i + 1)..n {
                let x = &inputs[i];
                let y = &inputs[j];

                let r_x = &single[i];
                let r_y = &single[j];

                // Compose both settle orders once per pair: x-then-y (`s_xy`) and y-then-x (`s_yx`). Each
                // is `Some` only when its base single settles and the second toggle settles too. Reused by
                // the simultaneous-oscillation stable-set and the divergence check.
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
                    let mut stable_set: BTreeSet<Minterm<Symbol>> = BTreeSet::new();
                    if let Some(sxy) = &s_xy {
                        stable_set.insert(sxy.project_to(&group));
                    }
                    if let Some(syx) = &s_yx {
                        stable_set.insert(syx.project_to(&group));
                    }

                    // Record the pair-probe race: the racing pins/edges (taken at `s`) and prevector its
                    // generated constraint needs. This supplies an oscillating pair's (e.g. a mutex's)
                    // constraint, standing in for the divergence-derived one the
                    // combinational-neighbourhood filter below discards for it.
                    let race = Race {
                        x: x.clone(),
                        x_edge: edge_from(s, x.as_str()),
                        y: y.clone(),
                        y_edge: edge_from(s, y.as_str()),
                        prevector: prevector_s.clone(),
                        levels: levels_s.clone(),
                        node_levels: node_levels_at(s, &group),
                        discovered,
                    };
                    record_oscillation(
                        &mut oscillation,
                        inputs,
                        s,
                        &[x.as_str(), y.as_str()],
                        group,
                        stable_set.into_iter().collect(),
                        Some(race),
                    );
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

                // Non-confluent and interacting ⇒ an order-dependent hazard: the divergent state
                // variables and their two competing settled outcomes, at the input condition where the
                // pair races. The constraint generated from it (see [`constrain`]) has its kind decided
                // there, solely by the declared clock, since the hazard is a property of the cell rather
                // than of the declaration.
                let group: Vec<Symbol> =
                    state_vars.iter().filter(|w| diverges(w)).cloned().collect();
                let node_levels = node_levels_at(s, &group);
                let mut stable_set: BTreeSet<Minterm<Symbol>> = BTreeSet::new();
                stable_set.insert(s_xy.project_to(&group));
                stable_set.insert(s_yx.project_to(&group));
                let condition = machine::toggle(s, &[x.as_str(), y.as_str()]).project_to(inputs);
                record_order_dependence(
                    &mut order_dependence,
                    OrderDependence {
                        x: x.clone(),
                        x_edge: edge_from(s, x.as_str()),
                        y: y.clone(),
                        y_edge: edge_from(s, y.as_str()),
                        condition,
                        group,
                        stable: stable_set.into_iter().collect(),
                        prevector: prevector_s.clone(),
                        levels: levels_s.clone(),
                        node_levels,
                        discovered,
                    },
                );
            }
        }

        (order_dependence, oscillation)
    };

    // Probe every fully-initialised reachable state in parallel, then fold the per-state dedup maps
    // together. The filter comes AFTER `enumerate`, so `discovered` stays the BFS index of the state —
    // the tie-break both dedup reads use — rather than a position in the filtered sequence. The merge is
    // associative and commutative: `record_order_dependence` keeps the min `(prevector.len, discovered)`
    // — a total order per key — and `merge_oscillation` unions races into an arbitrary surviving
    // representative, so the folded result equals the sequential one regardless of state/thread order.
    let (order_dependence, oscillation) = ex
        .order
        .par_iter()
        .enumerate()
        .filter(|(_, s)| m.arc_eligible(s))
        .map(per_state)
        .reduce(
            || (BTreeMap::new(), BTreeMap::new()),
            |(mut oa, mut osca), (ob, oscb)| {
                for od in ob.into_values() {
                    record_order_dependence(&mut oa, od);
                }
                for (k, o) in oscb {
                    merge_oscillation(&mut osca, k, o);
                }
                (oa, osca)
            },
        );

    DetectedHazards {
        order_dependence: order_dependence.into_values().collect(),
        oscillation: oscillation.into_values().collect(),
    }
}

/// Generate the constraints that avoid a cell's detected hazards. One [`Constraint`] is built per
/// [`OrderDependence`] and per oscillation [`Race`], its kind decided solely by `clock_pins` (a pair
/// with exactly one declared clock is a directed setup/hold, else a symmetric non_seq). Deduped by the
/// canonical [`constraint_key`], keeping the min `(prevector.len, discovered)` representative; BTreeMap
/// gives deterministic output order.
pub(crate) fn constrain(hz: &DetectedHazards, clock_pins: &[Symbol]) -> Vec<Constraint> {
    let mut found: BTreeMap<String, (Constraint, usize)> = BTreeMap::new();
    for od in &hz.order_dependence {
        record_constraint(
            &mut found,
            make_constraint(
                &od.x,
                od.x_edge,
                &od.y,
                od.y_edge,
                clock_pins,
                Probed {
                    prevector: od.prevector.clone(),
                    levels: od.levels.clone(),
                    nodes: protected(&od.group, &od.node_levels),
                },
            ),
            od.discovered,
        );
    }
    for osc in &hz.oscillation {
        for race in &osc.races {
            record_constraint(
                &mut found,
                make_constraint(
                    &race.x,
                    race.x_edge,
                    &race.y,
                    race.y_edge,
                    clock_pins,
                    Probed {
                        prevector: race.prevector.clone(),
                        levels: race.levels.clone(),
                        nodes: protected(&osc.group, &race.node_levels),
                    },
                ),
                race.discovered,
            );
        }
    }
    found.into_values().map(|(c, _)| c).collect()
}

/// Build the constraint that avoids a hazard on pins `x`,`y` with edges taken at the probed state: a
/// directed setup/hold when exactly one of the pair is a declared clock (clock ← data), else a symmetric
/// non_seq. `prevector` is the (pre-cloned) path into that state and `levels` the (pre-cloned) output
/// levels sampled there.
fn make_constraint(
    x: &str,
    x_edge: Edge,
    y: &str,
    y_edge: Edge,
    clock_pins: &[Symbol],
    probed: Probed,
) -> Constraint {
    let Probed {
        prevector,
        levels,
        nodes,
    } = probed;
    let is_clock = |p: &str| clock_pins.iter().any(|c| c.as_str() == p);
    if is_clock(x) ^ is_clock(y) {
        let (clk, clk_edge, data, data_edge) = if is_clock(x) {
            (x, x_edge, y, y_edge)
        } else {
            (y, y_edge, x, x_edge)
        };
        Constraint {
            kind: ConstraintKind::SetupHold,
            related: Symbol::from(clk),
            related_edge: clk_edge,
            pin: Symbol::from(data),
            pin_edge: data_edge,
            prevector,
            levels,
            nodes,
        }
    } else {
        Constraint {
            kind: ConstraintKind::NonSeq,
            related: Symbol::from(x),
            related_edge: x_edge,
            pin: Symbol::from(y),
            pin_edge: y_edge,
            prevector,
            levels,
            nodes,
        }
    }
}

/// Record a detected order-dependent hazard into the dedup map, keyed by its unordered
/// `(pin,edge)|(pin,edge)` key, keeping the min `(prevector.len, discovered)` representative.
fn record_order_dependence(map: &mut BTreeMap<String, OrderDependence>, od: OrderDependence) {
    let a = format!("{}{}", od.x, od.x_edge.rf());
    let b = format!("{}{}", od.y, od.y_edge.rf());
    let (lo, hi) = if a <= b { (a, b) } else { (b, a) };
    let key = format!("{lo}|{hi}");
    // The `Option` read here is the incumbent — no entry yet, or one this candidate beats on
    // `(prevector.len, discovered)` — nothing to do with a state value's determinacy.
    if map
        .get(&key)
        .is_none_or(|e| (od.prevector.len(), od.discovered) < (e.prevector.len(), e.discovered))
    {
        map.insert(key, od);
    }
}

/// Record a detected oscillation hazard into one state's LOCAL dedup map, keyed by `group|condition`,
/// while appending any pair-probe [`Race`] to the surviving entry. The cross-state representative is
/// chosen later, when folding per-state maps together: [`merge_oscillation`] keeps whichever colliding
/// representative it sees first (an arbitrary, equal-quality tie) and unions every state's races.
fn record_oscillation(
    oscillation: &mut BTreeMap<String, Oscillation>,
    inputs: &[Symbol],
    node: &Minterm<Symbol>,
    names: &[&str],
    group: Vec<Symbol>,
    stable: Vec<Minterm<Symbol>>,
    race: Option<Race>,
) {
    let toggled = machine::toggle(node, names);
    let condition = toggled.project_to(inputs);
    let key = format!(
        "{}|{}",
        group.join(","),
        crate::logic::literals_str(&condition)
    );
    let entry = oscillation.entry(key).or_insert_with(|| Oscillation {
        group,
        condition,
        stable,
        races: Vec::new(),
    });
    if let Some(race) = race {
        entry.races.push(race);
    }
}

/// Merge one state's oscillation entry into the accumulator when folding the per-state maps. On a key
/// collision `group`/`condition` coincide (the key is injective in them), but `stable` does *not*: a
/// single-toggle observation records an empty `stable` while a pair-probe records a non-empty set, and
/// both share the key space. So keep the incumbent's remaining (key-determined) fields, UNION `stable`
/// as a set (dedup + canonical sort — collision-order-independent) and UNION the [`Race`]s. Races are
/// never dropped; they feed [`constrain`].
fn merge_oscillation(map: &mut BTreeMap<String, Oscillation>, key: String, osc: Oscillation) {
    match map.entry(key) {
        std::collections::btree_map::Entry::Vacant(v) => {
            v.insert(osc);
        }
        std::collections::btree_map::Entry::Occupied(mut e) => {
            let entry = e.get_mut();
            let mut merged: BTreeSet<Minterm<Symbol>> =
                std::mem::take(&mut entry.stable).into_iter().collect();
            merged.extend(osc.stable);
            entry.stable = merged.into_iter().collect();
            entry.races.extend(osc.races);
        }
    }
}

/// Record a generated constraint into the dedup map, keeping the min `(prevector.len, discovered)`
/// representative per canonical key.
fn record_constraint(
    found: &mut BTreeMap<String, (Constraint, usize)>,
    cons: Constraint,
    discovered: usize,
) {
    let key = constraint_key(&cons);
    // As in `record_order_dependence`: the `Option` is the incumbent, not a value's determinacy.
    if found
        .get(&key)
        .is_none_or(|(e, ed)| (cons.prevector.len(), discovered) < (e.prevector.len(), *ed))
    {
        found.insert(key, (cons, discovered));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::analyse_one as analyse;

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
        // The DFF detects an order-dependent hazard between CLK and D.
        assert!(
            cell.order_dependence
                .iter()
                .any(|od| [od.x.as_str(), od.y.as_str()]
                    .iter()
                    .all(|p| *p == "CLK" || *p == "D")),
            "expected an order-dependent hazard between CLK and D, got {:?}",
            cell.order_dependence
        );
        assert!(
            cell.oscillation.is_empty(),
            "a DFF detects no oscillation hazard, got {:?}",
            cell.oscillation
        );
        // …from which a setup/hold constraint of D w.r.t. CLK is generated; because the kind follows the
        // declared clock, not the geometry, nothing on the pair is generated as non_seq.
        let cons = cell.constraints.clone();
        eprintln!("DFF constraints: {cons:#?}");
        assert!(
            cons.iter().all(|c| c.kind == ConstraintKind::SetupHold),
            "a declared-clock DFF yields only setup/hold, got {cons:?}"
        );
        assert!(
            cons.iter()
                .any(|c| c.related == "CLK" && c.related_edge == Edge::Rise && c.pin == "D"),
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
        let cons = cell.constraints.clone();
        assert!(!cons.is_empty());
        assert!(
            cons.iter().all(|c| c.kind == ConstraintKind::NonSeq),
            "an undeclared DFF yields only non_seq, got {cons:?}"
        );
    }

    #[test]
    fn mutex_has_non_seq_between_requests() {
        // Cross-coupled mutex: A and B race symmetrically. Their order-divergence is on the coupled
        // grant outputs (Qa/Qb), neither of which has *both* A and B in its own δ's direct support, so
        // the combinational-neighbourhood filter discards it — the mutex detects no order-dependent
        // hazard. But the simultaneous A*B toggle drives the state into an oscillation hazard, whose
        // pair-probe race supplies the pair's generated non_seq constraint.
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
        // Detects exactly one oscillation hazard (backed by a single pair-probe race) and no
        // order-dependent hazard.
        assert_eq!(
            cell.oscillation.len(),
            1,
            "expected one oscillation hazard, got {:?}",
            cell.oscillation
        );
        assert_eq!(
            cell.oscillation[0].races.len(),
            1,
            "expected one pair-probe race, got {:?}",
            cell.oscillation[0].races
        );
        assert!(
            cell.order_dependence.is_empty(),
            "a mutex detects no order-dependent hazard, got {:?}",
            cell.order_dependence
        );
        let cons = cell.constraints.clone();
        eprintln!("MUT constraints: {cons:#?}");
        assert!(
            cons.iter().any(|c| c.kind == ConstraintKind::NonSeq
                && [c.related.as_str(), c.pin.as_str()]
                    .iter()
                    .all(|p| *p == "A" || *p == "B")),
            "expected a non_seq constraint between A and B, got {cons:?}"
        );
        assert!(
            cons.iter().all(|c| c.kind == ConstraintKind::NonSeq),
            "a mutex yields only non_seq constraints, got {cons:?}"
        );
    }

    #[test]
    fn c_element_has_non_seq_constraint() {
        // A C-element is order-sensitive: A↓ racing B↑ leaves Q history-dependent. That is an
        // order-dependent hazard (not an oscillation), from which a non_seq constraint between A and B
        // is generated.
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
        // Detects an order-dependent hazard between A and B, and no oscillation hazard.
        assert!(
            cell.order_dependence
                .iter()
                .any(|od| [od.x.as_str(), od.y.as_str()]
                    .iter()
                    .all(|p| *p == "A" || *p == "B")),
            "expected an order-dependent hazard between A and B, got {:?}",
            cell.order_dependence
        );
        assert!(
            cell.oscillation.is_empty(),
            "a C-element detects no oscillation hazard, got {:?}",
            cell.oscillation
        );
        let cons = cell.constraints.clone();
        eprintln!("C2 constraints: {cons:#?}");
        assert!(
            cons.iter().any(|c| c.kind == ConstraintKind::NonSeq
                && [c.related.as_str(), c.pin.as_str()]
                    .iter()
                    .all(|p| *p == "A" || *p == "B")),
            "expected a non_seq constraint between A and B, got {cons:?}"
        );
    }

    #[test]
    fn constraint_prevector_lengths_are_minimal() {
        // Multiset of per-key minimal prevector lengths — pins the min-by-len quality criterion. The
        // minimum runs over the fully-initialised probed states (`Machine::arc_eligible`), so a cell
        // whose seeds leave a state variable undriven measures from further along its BFS. Re-capture
        // only for a deliberate algorithm change.
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
        let mut dff_lens: Vec<usize> = dff.constraints.iter().map(|c| c.prevector.len()).collect();
        dff_lens.sort();
        // Every DFF seed sits at CLK=0, where δ_M = !CLK*D + CLK*M forces M but δ_Q = CLK*M + !CLK*Q
        // holds Q: Q is undriven there, so no probe starts at a seed. The shortest eligible state a
        // CLK↑ probe can start from is three input states along — CLK low, a pulse that drives Q, and
        // CLK low again.
        assert_eq!(dff_lens, vec![3, 3]);

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
        let mut c2_lens: Vec<usize> = c2.constraints.iter().map(|c| c.prevector.len()).collect();
        c2_lens.sort();
        // C2's single state variable is forced at both seeds, so every state in its explored order is
        // eligible and the minimum is the BFS distance alone.
        assert_eq!(c2_lens, vec![2, 2]);
    }

    #[test]
    fn constraint_levels_travel_with_the_representative_prevector() {
        // The levels and the prevector are sampled at the SAME probed state, so the representative the
        // min-`(prevector.len, discovered)` dedup keeps carries a consistent pair: each surviving
        // constraint matches one detected hazard on BOTH, never a mix of two states.
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
        let cons = cell.constraints.clone();
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
                cell.order_dependence
                    .iter()
                    .any(|od| od.prevector == c.prevector && od.levels == c.levels),
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
        let cons = cell.constraints.clone();
        eprintln!("SR constraints: {cons:#?}");
        assert!(
            cons.iter().any(|c| c.kind == ConstraintKind::NonSeq),
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
        // The (C1, C2) divergence is latch-mediated, so it is filtered at detection — no C1/C2
        // order-dependent hazard is reported.
        assert!(
            !cell
                .order_dependence
                .iter()
                .any(|od| [od.x.as_str(), od.y.as_str()]
                    .iter()
                    .all(|p| *p == "C1" || *p == "C2")),
            "the C1/C2 divergence is latch-mediated and must be filtered, got {:?}",
            cell.order_dependence
        );
        let cons = cell.constraints.clone();
        eprintln!("SYNC2 constraints: {cons:#?}");
        assert!(
            !cons.iter().any(|c| [c.related.as_str(), c.pin.as_str()]
                .iter()
                .all(|p| *p == "C1" || *p == "C2")),
            "the C1/C2 divergence is latch-mediated and must be filtered, got {cons:?}"
        );
        assert!(
            cons.iter().any(|c| [c.related.as_str(), c.pin.as_str()]
                .iter()
                .all(|p| *p == "C1" || *p == "D")),
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
        assert!(cell.order_dependence.is_empty());
        assert!(cell.oscillation.is_empty());
    }
}
