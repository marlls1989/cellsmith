//! Behavioural per-arc edge classification.
//!
//! Timing arcs are NOT derived here. Every arc the cell has already came out of [`super::arcs::derive`],
//! which exists wherever a single-input toggle between reachable stable states changes an output. This
//! module only attaches a LABEL to the clock-related ones — it never builds a parallel set of
//! `(node, clock, direction)` tuples of its own. Labelling is PER ARC: there is no register verdict on a
//! node, and edge and combinational arcs coexist freely on one output (an async-reset flop carries both).
//!
//! # The definition
//!
//! **A clock toggle that takes a latch from opaque to transparent, and whose resulting output value
//! DEPENDS ON LATCH CONTENT rather than arriving regardless, is an EDGE ARC on that output.** There is
//! ONE category — an edge arc — and it emits Liberate `-type edge` (see [`crate::emit::arcs_tcl`]). An
//! arc that does not meet the definition — a data change propagating through an already-transparent
//! latch, or a clock acting by its LEVEL (a clock gate) — carries no label and stays an ordinary
//! combinational data arc.
//!
//! A CONDITIONED arc is still an arc: conditioning on data, on state, on a second clock's level or on
//! clock phase puts a condition in the arc's `-when` and never suppresses or reclassifies it. MASKING
//! needs no rule — it falls out of arc derivation: an arc exists only where a transition actually reaches
//! an output, so a flop master's opening (stopped by its closed slave) and a gated clock's falling edge
//! (cancelled by the gating condition it controls) simply never present an arc.
//!
//! Everything is derived BEHAVIOURALLY from observed toggle-and-settle transitions, never from the shape
//! of an equation, and nothing branches on a declared input class — an async pin need not be declared,
//! its effect being classified from its own observed moves ([`forcing_pins`]). The characterisation is
//! consequently IMPLEMENTATION-STYLE INVARIANT: the NAND-implemented `NDLAT` / `NDFF` / `NHPIPE`
//! fixtures characterise identically to their pass-transistor twins.
//!
//! [`classify`] is a **post-exploration** read-only pass over the shared [`Machine`]: it re-walks the
//! exploration with [`machine::toggle`]/[`machine::settle`], mirroring [`super::arcs::derive`]'s
//! per-node walk, and only ADDS an edge annotation. It never re-derives the exploration, the
//! prevectors or the hazards — those stay byte-identical whether the annotation is on or off.
//!
//! # The mechanism
//!
//! Only FULLY-DETERMINATE reachable states take part in any measurement — a state with a don't-care
//! (uninitialised) state column is arc-INELIGIBLE and quantified out of every witness (a don't-care is a
//! MISSING variable, never coerced to 0/1). Traversal is untouched: partial states remain seeds, they are
//! simply not measured from.
//!
//! The pipeline is one analysis over the machine's `toggle`/`settle` observations:
//!
//! 1. **Arc typing** — per arc at full identity `(output, related, direction, machine start minterm)`.
//!    **Condition 1'** (a latch opens): some state variable `W` in `{output} ∪ support(δ_output)` that
//!    [`stores`] in the source phase but not the delivered phase — the clock edge takes it
//!    opaque→transparent — the delivered phase non-empty after forcing exclusion. Then, over the changed
//!    firing, ONE of: **(a) generation** — the output is itself the opener; **(b) frozen-exposure** — an
//!    eligible reachable state anchored at the destination, differing in one clock-associated latch,
//!    shows the output a different value; **(c) masked-delivery** — withholding the opener's δ reverts the
//!    output's change. Condition 1' plus any arm ⇒ the arc is edge.
//! 2. **The seam set `S`** — per candidate node (every output and internal state variable), the
//!    `(clock, direction)` toggles on which the node carries an edge SEAM: the typing holds AND the
//!    delivered value HOLDS through the phase, the last a greatest fixpoint (`seam_fixpoint`) that removes
//!    `(K, d)` when a non-forcing change of the node inside its delivered phase occurs at a toggle not
//!    itself an edge of `S`. A node with a non-empty `S` is an edge register; its per-edge next-state
//!    functions and off-edge are synthesised into [`EdgeArcs::captures`].
//! 3. **Cover synthesis** — [`synth_capture`], [`generalise`] and [`regions_from`] over one uniform
//!    header (all inputs except the keying clock plus every candidate), with an ordered drop-loop that
//!    prefers inputs over internals so the fold-eligible internals drop out of the cover.
//! 4. **Fold** — internal non-seam nodes fold away as an emission-time reachability fixpoint.
//!
//! The capture and off-edge functions are recorded verbatim as ordinary functions — an inverting flop's
//! next state is simply `!D`, never special-cased.
//!
//! See `docs/edge-collapse.md` for the concept-first walkthrough.

use std::collections::{BTreeMap, BTreeSet, HashMap};

use espresso_logic::bdd::{Bdd, BddBuilder, Brand, ManagerCell};
use espresso_logic::{Cover, CoverType, CubeType, Minimizable, Minterm, Symbol};
use rayon::prelude::*;

use crate::logic::analysis::Machine;
use crate::logic::arcs::{Arc as DelayArc, Edge};
use crate::logic::machine;
use crate::logic::regions::{self, StateRegions};

/// The edge arcs carried by one node: the node re-expressed as an edge seam on one or more clocks, each
/// edge making the node capture-and-hold a next-state value.
#[derive(Debug, Clone)]
pub struct EdgeCaptures {
    /// The node the edge arcs belong to.
    pub node: Symbol,
    /// The captured next-state function per active edge, as combinational state-table regions (total —
    /// off is the complement of on, empty hold). Each capture carries ITS OWN clock. Grouped by clock in
    /// cell input-pin order, `Rise` before `Fall` within a clock; a single-clock node keeps one entry per
    /// active edge (two for a dual-edge node with `Rise` first), byte-identical to a single-clock keying.
    pub captures: Vec<(Symbol, Edge, StateRegions)>,
    /// The off-edge (hold) function as state-table regions, keyed by the clock set's phase vector: on/off
    /// are the async set/clear covers, hold is the quiescent region; never references any of the clocks.
    pub off_edge: StateRegions,
    /// The node's column set: the first-appearance union of the captures' cols then `off_edge.cols`.
    pub cols: Vec<Symbol>,
}

impl EdgeCaptures {
    /// The distinct clocks the edge arcs key off, in first-appearance (capture) order.
    pub fn clocks(&self) -> Vec<&Symbol> {
        let mut out: Vec<&Symbol> = Vec::new();
        for (clock, _, _) in &self.captures {
            if !out.contains(&clock) {
                out.push(clock);
            }
        }
        out
    }
}

/// The behavioural edge arcs of a cell: the per-node edge captures recognised across its outputs and
/// state variables, plus the cell-level set of internal non-seam master nodes folded away. A mutually-
/// or transitively-referencing set of such nodes folds together whenever nothing outside the set reaches
/// an output, so a cross-coupled pair — or a longer reference chain — shares one fold, not just a single
/// self-holding node.
///
/// There is ONE arc category, an EDGE arc: a clock toggle that takes a latch from opaque to transparent
/// and whose resulting output value depends on latch content. A node that additionally HOLDS its
/// delivered value through the phase is an edge register, its per-edge next-state functions recorded in
/// [`EdgeArcs::captures`]; a latch that merely tracks live data through its open phase carries an edge
/// arc but no capture (its seam set is empty). Both are real timing arcs and both emit Liberate
/// `-type edge`.
#[derive(Debug, Default)]
pub struct EdgeArcs {
    pub captures: Vec<EdgeCaptures>,
    pub folded: Vec<Symbol>,
    /// The set of the cell's CLOCK-RELATED delay arcs that are EDGE arcs, by full arc identity —
    /// `(output, related clock, the clock's own direction, machine start minterm)`, the same identity
    /// [`super::arcs::derive`] keys an [`super::arcs::Arc`] on. SOURCED FROM the derived arcs: an entry
    /// exists only where the pipeline observed the transition (so a masked edge — a `DFF`'s fall,
    /// `ICM`'s competing branch reaching an internal latch — has none by construction, and internal
    /// nodes carrying no delay arc are never labelled). An arc whose identity is ABSENT stays
    /// `-type combinational`: it acts by the clock's LEVEL, or propagates data through an
    /// already-transparent latch. Membership is PER FIRING, so two firings of one
    /// `(output, clock, direction)` that differ only in internal state can type differently.
    pub labels: BTreeSet<(Symbol, Symbol, Edge, Minterm<Symbol>)>,
}

/// A single clock edge's observations for one candidate: whether any sample changed the value (the
/// vacuity gate), the `(pre-state, post-value)` samples (unchanged clock-toggle samples included) for
/// cover synthesis, and the full per-firing census the typing replays for content-dependence.
#[derive(Default, Clone)]
struct CapAgg {
    changed: bool,
    samples: Vec<(Minterm<Symbol>, bool)>,
    /// One entry per settling firing of this edge — CHANGED OR NOT: `(pre-state, destination stable
    /// state, post value)`.
    firings: Vec<(Minterm<Symbol>, Minterm<Symbol>, bool)>,
}

/// The aggregated observations of one candidate node across the whole exploration walk.
#[derive(Default, Clone)]
struct CandAgg {
    /// One entry per single-input toggle that CHANGED the node: `(toggled input, SOURCE stable state,
    /// destination stable state, post value)`. Every moving toggle is recorded uniformly — clock, data
    /// and async alike — and the decision core reads them back for the node's forcing pins. The source
    /// state is kept so a move can be replayed from where it started (the forcing classification needs
    /// the pre-toggle state, not just where it landed).
    moves: Vec<(Symbol, Minterm<Symbol>, Minterm<Symbol>, bool)>,
    /// Per `(clock, is_rise)`: the clock-edge observations.
    captures: BTreeMap<(Symbol, bool), CapAgg>,
    /// The `(stable state, value)` samples, for the off-edge synthesis.
    stable: Vec<(Minterm<Symbol>, bool)>,
}

impl CandAgg {
    /// Fold another node's contribution for the same candidate into this one.
    fn merge(&mut self, other: CandAgg) {
        self.moves.extend(other.moves);
        for (k, cap) in other.captures {
            let e = self.captures.entry(k).or_default();
            e.changed |= cap.changed;
            e.samples.extend(cap.samples);
            e.firings.extend(cap.firings);
        }
        self.stable.extend(other.stable);
    }
}

/// A synthesised register: its per-clock, per-edge captures (each carrying its clock, grouped by clock in
/// input-pin order with Rise first) and its off-edge.
type Synthesised = (Vec<(Symbol, Edge, StateRegions)>, StateRegions);

/// One candidate edge arc on a node: `(clock, is_rise)`. The decision core's whole currency — arcs, never
/// a verdict on the node, so edge and combinational arcs coexist freely on one output.
type Arc = (Symbol, bool);

/// Discover each node's edge arcs from the cell's toggle-and-settle behaviour and label the cell's
/// delay arcs per arc. Read-only over the shared [`Machine`]: it re-walks the exploration and only ADDS
/// an annotation, mirroring [`super::arcs::derive`] — whose derived arcs are also the SOURCE of the
/// label domain: only an observed arc is ever labelled.
pub fn classify<B: Brand, C: ManagerCell + Send + Sync>(
    m: &Machine<B, C>,
    delay_arcs: &[DelayArc],
) -> EdgeArcs {
    // The builder mints region covers, and is only present when the cell has state variables. With no
    // state variable no latch can open, so condition 1' is vacuously false and both `labels` and
    // `captures` come out empty from the loops below — there is no early return (its absence was what let
    // an unrelated flop change an unrelated arc's type). Every path that needs the builder is guarded by
    // a non-empty opener set, which implies a state variable exists.
    let builder = m.deltas.first().map(|(_, d)| d.builder());

    let cell = m.cell;
    let inputs = &cell.inputs;
    let deltas = &m.deltas;
    let ex = &m.explored;

    // The declared clocks. Every declared clock is a candidate edge key; whether a clock keeps edge arcs
    // on a given node is decided behaviourally, not by input-class routing.
    let clock_set: BTreeSet<&str> = cell.clock_pins.iter().map(Symbol::as_str).collect();

    // Candidates: every output (value read via `Machine::output_value`, so combinational outputs are
    // included) plus every internal state variable (the state-machine coordinates that are not outputs).
    let output_names: BTreeSet<&str> = cell.outputs.iter().map(|o| o.name.as_str()).collect();
    let mut candidates: Vec<Symbol> = cell.outputs.iter().map(|o| o.name.clone()).collect();
    for sv in &m.state_vars {
        if !output_names.contains(sv.as_str()) {
            candidates.push(sv.clone());
        }
    }

    // ELIGIBILITY (aligned with `ex.order`): a reachable stable state is arc-eligible iff every STATE
    // column is determinate — no don't-care. A don't-care is a MISSING variable, never coerced to 0/1, so
    // an ineligible start would read an uninitialised latch as though it held a value. Traversal is
    // untouched — a partial state stays a seed in `ex.order` — but no measurement quantifies over one.
    let eligible: Vec<bool> = ex
        .order
        .iter()
        .map(|s| {
            m.state_vars
                .iter()
                .all(|w| s.value_of(w.as_str()).is_some())
        })
        .collect();
    let is_eligible = |s: &Minterm<Symbol>| {
        m.state_vars
            .iter()
            .all(|w| s.value_of(w.as_str()).is_some())
    };

    let value = |name: &Symbol, node: &Minterm<Symbol>| m.output_value(name.as_str(), node);

    // The observation walk over the ELIGIBLE reachable stable states, mirroring `arcs::derive`'s per-node
    // walk. Each node toggles one input at a time, settles, and records the candidate values before/after.
    // The walk produces plain data (minterms); no BDD is built here.
    let per_node = |node: &Minterm<Symbol>| -> Vec<CandAgg> {
        let mut out: Vec<CandAgg> = vec![CandAgg::default(); candidates.len()];
        if !is_eligible(node) {
            return out; // an uninitialised start is not measured from
        }
        let v0: Vec<Option<bool>> = candidates.iter().map(|c| value(c, node)).collect();
        for (i, b) in v0.iter().enumerate() {
            if let Some(b) = b {
                out[i].stable.push((node.clone(), *b));
            }
        }
        for related in inputs {
            let toggled = machine::toggle(node, &[related.as_str()]);
            let Some(np) = machine::settle(deltas, &toggled) else {
                continue;
            };
            let is_clock = clock_set.contains(related.as_str());
            let rose = np.value_of(related.as_str()) == Some(true);
            for (i, c) in candidates.iter().enumerate() {
                let (Some(b0), Some(b1)) = (v0[i], value(c, &np)) else {
                    continue;
                };
                if is_clock {
                    // A clock toggle: record every sample for the cover synthesis, changed or not, and the
                    // firing itself — pre, destination and post — for the typing.
                    let cap = out[i].captures.entry((related.clone(), rose)).or_default();
                    cap.samples.push((node.clone(), b1));
                    cap.firings.push((node.clone(), np.clone(), b1));
                    if b0 != b1 {
                        cap.changed = true;
                    }
                }
                if b0 != b1 {
                    // Every moving toggle — clock, data or async alike — is a uniform move: the source
                    // state, the destination stable state and the post value the decision core reads.
                    out[i]
                        .moves
                        .push((related.clone(), node.clone(), np.clone(), b1));
                }
            }
        }
        out
    };

    let aggs: Vec<CandAgg> = ex.order.par_iter().map(per_node).reduce(
        || vec![CandAgg::default(); candidates.len()],
        |mut a, b| {
            for (ai, bi) in a.iter_mut().zip(b) {
                ai.merge(bi);
            }
            a
        },
    );

    // The single-input transition table is node-independent, so it is built once and shared by every
    // stores/seam scan rather than re-settling.
    let trans = Transitions::build(m);

    // Each candidate's forcing pins (a state var is always a candidate, so this doubles as the forcing
    // lookup every `W` needs). Computed BEFORE any synthesis, so the seam set — which decides the
    // fold-eligible internals the drop-loop prefers to shed — is settled first.
    let forcing_of: Vec<BTreeMap<Symbol, (bool, bool)>> = candidates
        .iter()
        .zip(&aggs)
        .map(|(name, agg)| forcing_pins(m, &trans, name, &agg.moves))
        .collect();
    let forcing_by_name: HashMap<&str, &BTreeMap<Symbol, (bool, bool)>> = candidates
        .iter()
        .zip(&forcing_of)
        .map(|(n, f)| (n.as_str(), f))
        .collect();

    // Every candidate's raw function δ (state δ then combinational-output δ), for the opener SCOPE
    // `{node} ∪ support(δ_node)` and the fold's surviving-signal reference check.
    let mut fn_of: BTreeMap<&str, &Bdd<B, C>> = BTreeMap::new();
    for (n, d) in deltas {
        fn_of.insert(n.as_str(), d);
    }
    for (n, d) in &m.out_deltas {
        fn_of.insert(n.as_str(), d);
    }

    // `stores(W, K, level)` for every state variable, clock and phase — node-independent, so computed
    // once over the ELIGIBLE states. A latch OPENS on `(K, d)` when it stores in the source phase (`!d`)
    // but not the delivered one (`d`); it is K-ASSOCIATED when its two phases disagree.
    let clocks: Vec<&Symbol> = inputs
        .iter()
        .filter(|p| clock_set.contains(p.as_str()))
        .collect();
    let mut stores_of: HashMap<(Symbol, Symbol, bool), bool> = HashMap::new();
    for w in &m.state_vars {
        let fw = forcing_by_name[w.as_str()];
        for clock in &clocks {
            for level in [false, true] {
                let v = stores(m, &trans, &eligible, w, clock, level, fw);
                stores_of.insert((w.clone(), (*clock).clone(), level), v);
            }
        }
    }
    let opens = |w: &Symbol, clock: &Symbol, is_rise: bool| -> bool {
        stores_of[&(w.clone(), clock.clone(), !is_rise)]
            && !stores_of[&(w.clone(), clock.clone(), is_rise)]
    };

    // The opener SCOPE of a candidate: the state variables in `{node} ∪ support(δ_node)` — the only
    // latches whose opening can deliver content to `node` (condition 1').
    let opener_scope = |node: &str| -> Vec<Symbol> {
        let mut scope: BTreeSet<Symbol> = BTreeSet::new();
        if m.state_set.contains(node) {
            scope.insert(Symbol::from(node));
        }
        if let Some(f) = fn_of.get(node) {
            for v in f.variables() {
                if m.state_set.contains(&v) {
                    scope.insert(v);
                }
            }
        }
        scope.into_iter().collect()
    };

    // Is the delivered phase (`clock == is_rise`) non-empty over the eligible states once the node's own
    // forcing region is excluded?
    let delivered_nonempty =
        |clock: &Symbol, is_rise: bool, node_forcing: &BTreeMap<Symbol, (bool, bool)>| -> bool {
            trans.order.iter().enumerate().any(|(i, s)| {
                eligible[i]
                    && s.value_of(clock.as_str()) == Some(is_rise)
                    && !node_forcing
                        .iter()
                        .any(|(p, (a, _))| s.value_of(p.as_str()) == Some(*a))
            })
        };

    // THE PER-ARC LABELS (arc typing): each derived delay arc whose related pin is a declared clock is an
    // edge arc iff CONDITION 1' (an in-scope latch opens on the clock's own direction, delivered phase
    // non-empty) holds AND one of the three arms — GENERATION, FROZEN-EXPOSURE, MASKED-DELIVERY — fires at
    // the arc's OWN firing (its full machine start context). Membership is the identity itself, so two
    // firings of one `(output, clock, direction)` can type differently, and an unobserved edge — masked in
    // `arcs::derive`, or from an ineligible start — has no identity to add.
    let mut labels: BTreeSet<(Symbol, Symbol, Edge, Minterm<Symbol>)> = BTreeSet::new();
    for a in delay_arcs {
        if !clock_set.contains(a.related.as_str()) {
            continue;
        }
        let is_rise = a.end.value_of(a.related.as_str()) == Some(true);
        let scope = opener_scope(a.output.as_str());
        let opened: Vec<&Symbol> = scope
            .iter()
            .filter(|w| opens(w, &a.related, is_rise))
            .collect();
        let node_forcing = forcing_by_name[a.output.as_str()];
        if opened.is_empty() || !delivered_nonempty(&a.related, is_rise, node_forcing) {
            continue; // condition 1' fails: no in-scope latch opens, or no delivered phase to reason about
        }
        let Some(np) = machine::settle(deltas, &machine::toggle(&a.start, &[a.related.as_str()]))
        else {
            continue;
        };
        let k_assoc = |w: &Symbol| {
            stores_of[&(w.clone(), a.related.clone(), false)]
                != stores_of[&(w.clone(), a.related.clone(), true)]
        };
        let is_edge = opened.iter().any(|w| w.as_str() == a.output.as_str())
            || frozen_exposure(
                m,
                &trans,
                &eligible,
                inputs,
                a.output.as_str(),
                &np,
                &k_assoc,
            )
            || masked_delivery(
                m,
                builder.as_ref(),
                deltas,
                a.output.as_str(),
                &a.related,
                &opened,
                &a.start,
                &np,
            );
        if is_edge {
            let edge = if is_rise { Edge::Rise } else { Edge::Fall };
            labels.insert((a.output.clone(), a.related.clone(), edge, a.start.clone()));
        }
    }

    // THE SEAM SET per candidate: the `(clock, direction)` toggles on which the node carries an edge seam —
    // the typing holds for some eligible changed firing AND the delivered value holds through the phase
    // (the greatest fixpoint in `seam_fixpoint`). A non-empty seam set is an edge register; an empty one is
    // level (a latch that merely tracks, or a clock gate).
    let seam_of: Vec<BTreeSet<Arc>> = candidates
        .iter()
        .zip(&aggs)
        .zip(&forcing_of)
        .map(|((name, agg), node_forcing)| {
            let scope = opener_scope(name.as_str());
            let mut s: BTreeSet<Arc> = BTreeSet::new();
            for ((clock, is_rise), cap) in &agg.captures {
                if !cap.changed {
                    continue; // vacuity gate: some eligible firing must change the node
                }
                let opened: Vec<&Symbol> =
                    scope.iter().filter(|w| opens(w, clock, *is_rise)).collect();
                if opened.is_empty() || !delivered_nonempty(clock, *is_rise, node_forcing) {
                    continue;
                }
                let k_assoc = |w: &Symbol| {
                    stores_of[&(w.clone(), clock.clone(), false)]
                        != stores_of[&(w.clone(), clock.clone(), true)]
                };
                // (a) GENERATION short-circuits; otherwise (b) FROZEN-EXPOSURE / (c) MASKED-DELIVERY over
                // the firings.
                let content = opened.iter().any(|w| w.as_str() == name.as_str())
                    || cap.firings.iter().any(|(pre, np, _)| {
                        frozen_exposure(m, &trans, &eligible, inputs, name.as_str(), np, &k_assoc)
                            || masked_delivery(
                                m,
                                builder.as_ref(),
                                deltas,
                                name.as_str(),
                                clock,
                                &opened,
                                pre,
                                np,
                            )
                    });
                if content {
                    s.insert((clock.clone(), *is_rise));
                }
            }
            seam_fixpoint(
                m,
                &trans,
                &eligible,
                inputs,
                &clock_set,
                name.as_str(),
                node_forcing,
                &mut s,
            );
            s
        })
        .collect();

    // The internal non-seam nodes: the drop-loop prefers to shed these from a survivor's cover, and they
    // are the fold candidates. Output nodes are never folded, so their names stay available in the header.
    let internal_nonseam: BTreeSet<Symbol> = candidates
        .iter()
        .zip(&seam_of)
        .filter(|(name, s)| s.is_empty() && !output_names.contains(name.as_str()))
        .map(|(name, _)| name.clone())
        .collect();
    let inputs_set: BTreeSet<&str> = inputs.iter().map(Symbol::as_str).collect();

    let mut captures: Vec<EdgeCaptures> = Vec::new();
    for (i, s) in seam_of.iter().enumerate() {
        if s.is_empty() {
            continue;
        }
        let name = &candidates[i];
        let agg = &aggs[i];
        // The keying clocks in cell input-pin order, each with the `(is_rise, Edge)` directions kept
        // (Rise before Fall). Every clock present carries at least one seam direction.
        let clock_edges: Vec<(Symbol, Vec<(bool, Edge)>)> = inputs
            .iter()
            .filter(|p| s.iter().any(|(clock, _)| clock == *p))
            .map(|clock| {
                let mut edges: Vec<(bool, Edge)> = Vec::new();
                if s.contains(&(clock.clone(), true)) {
                    edges.push((true, Edge::Rise));
                }
                if s.contains(&(clock.clone(), false)) {
                    edges.push((false, Edge::Fall));
                }
                (clock.clone(), edges)
            })
            .collect();

        let (node_captures, off_edge) = synth_node_captures(
            builder
                .as_ref()
                .expect("a seam implies a state variable, hence a builder"),
            &candidates,
            &internal_nonseam,
            &inputs_set,
            inputs,
            &clock_edges,
            agg,
        );
        let cols = capture_cols(&node_captures, &off_edge);
        captures.push(EdgeCaptures {
            node: name.clone(),
            captures: node_captures,
            off_edge,
            cols,
        });
    }

    // FOLD (cell-level): folding is decided at emission as a REACHABILITY question — once the collapse is
    // done, does this value still influence anything the cell emits? An internal non-seam node folds unless
    // a chain of raw-function references, starting from surviving emitted content — a surviving
    // capture/off-edge cover column, or the raw function of a survivor that can never fold — reaches it. A
    // mutually-referencing non-seam set that reaches no such sink influences nothing and collapses as one,
    // exactly as a lone self-holding master does.
    //
    // This is computed as a least-fixpoint liveness marking, which is the complement of the greatest
    // fixpoint "assume every candidate folds, then reinstate any candidate referenced from OUTSIDE the
    // folded set": a node's own function only propagates once the node is already live, so self-reference
    // alone never marks it, while any chain that reaches a live sink strands the whole chain.
    //
    // The criterion is deliberately NARROWER than early minimisation's, which preserves self-referential
    // loops so oscillation stays detectable — minimisation is untouched by this. Statetable invariant I3
    // (`src/logic/minimise.rs`) holds by construction: every kept survivor's support is kept by closure.
    let ref_reg: BTreeSet<&str> = captures
        .iter()
        .flat_map(|r| r.cols.iter().map(Symbol::as_str))
        .collect();
    // The foldable population: internal non-seam nodes.
    let foldable: BTreeSet<&str> = internal_nonseam.iter().map(Symbol::as_str).collect();

    // The liveness seeds. Every non-seam candidate that is NOT foldable — a non-seam OUTPUT — has its RAW
    // function emitted and can never fold, so it is a sink whose support must survive. Candidates that
    // CARRY a seam are neither seeds nor propagation sources: their raw function is replaced by the edge
    // seam, so their references reach us through `ref_reg` instead. On top of that, any foldable node named
    // by a surviving capture or off-edge cover column is itself live.
    let mut live: BTreeSet<&str> = BTreeSet::new();
    for (name, s) in candidates.iter().zip(&seam_of) {
        if !s.is_empty() {
            continue;
        }
        if !foldable.contains(name.as_str()) || ref_reg.contains(name.as_str()) {
            live.insert(name.as_str());
        }
    }
    let mut worklist: Vec<&str> = live.iter().copied().collect();

    // Propagate liveness along each live node's raw-function support — semantic BDD support, never equation
    // shape — until the least fixpoint is reached.
    while let Some(l) = worklist.pop() {
        let Some(f) = fn_of.get(l) else {
            continue;
        };
        for v in f.variables() {
            // Take the name back out of `foldable` so the marking borrows the candidate list, not the BDD.
            if let Some(&n) = foldable.get(v.as_str()) {
                if live.insert(n) {
                    worklist.push(n);
                }
            }
        }
    }

    // Everything foldable that liveness never reached folds, in candidate declaration order.
    let folded: Vec<Symbol> = candidates
        .iter()
        .filter(|m| foldable.contains(m.as_str()) && !live.contains(m.as_str()))
        .cloned()
        .collect();

    EdgeArcs {
        captures,
        folded,
        labels,
    }
}

/// The single-input transition table over the reachable stable states: `next[s][x]` is the index of the
/// stable state reached by toggling input `x` in `order[s]` and settling (`None` when that toggle
/// oscillates, or lands outside the explored set). The table is NODE-INDEPENDENT — it describes the cell's
/// state machine, not any one candidate — so it is built once per cell and every candidate's phase scan
/// indexes into it rather than re-settling.
struct Transitions<'a> {
    order: &'a [Minterm<Symbol>],
    next: Vec<Vec<Option<usize>>>,
}

impl<'a> Transitions<'a> {
    fn build<B: Brand, C: ManagerCell + Send + Sync>(m: &'a Machine<'_, B, C>) -> Self {
        let order = &m.explored.order[..];
        let index: HashMap<&Minterm<Symbol>, usize> =
            order.iter().enumerate().map(|(i, s)| (s, i)).collect();
        let next: Vec<Vec<Option<usize>>> = order
            .par_iter()
            .map(|s| {
                m.cell
                    .inputs
                    .iter()
                    .map(|x| {
                        machine::settle(&m.deltas, &machine::toggle(s, &[x.as_str()]))
                            .and_then(|np| index.get(&np).copied())
                    })
                    .collect()
            })
            .collect();
        Transitions { order, next }
    }
}

/// Is `node` genuinely TRANSPARENT in `clock`'s `level` phase? Two conjuncts, both behavioural:
///
/// * MEMORYLESS — no two reachable stable states of the phase that agree on every column except `node`
///   differ in `node` (the phase pins the node's value, so nothing is being remembered across it), and
/// * MOVING — the node actually moves within the phase under some non-`clock` toggle (an inert phase is
///   not transparency, it is a node that simply never changes there).
///
/// Transparency in a phase is the LATCH signature: the node tracks live data there instead of presenting
/// an already-captured value, so it cannot hold independent of the clock's LEVEL and carries no edge arc
/// on that clock. The contrasting phase is HYSTERETIC — the node holds there, only being moved by an edge
/// or a co-resident clock.
///
/// The FORCING region takes no part in either conjunct: a set/clear that overrides the node in the phase
/// is a coexisting combinational arc, not the node tracking data, so a phase whose only movement is a
/// forcing (a flop's reset asserting across its closed phase) is not transparent.
///
/// The classifier itself does not consult this — the edge arcs are decided by condition 1' and the seam
/// fixpoint from [`stores`]. Its only consumer is the replay-faithfulness harness, which reads an OPEN
/// phase against the cover.
#[cfg_attr(not(test), allow(dead_code))]
fn transparent<B: Brand, C: ManagerCell>(
    m: &Machine<'_, B, C>,
    tr: &Transitions<'_>,
    node: &Symbol,
    clock: &Symbol,
    level: bool,
    forced: (&Bdd<B, C>, &Bdd<B, C>),
) -> bool {
    let is_forced = |s: &Minterm<Symbol>| {
        forced.0.evaluate_fast(s) == Some(true) || forced.1.evaluate_fast(s) == Some(true)
    };
    // Every column but the node itself: two phase states agreeing here and differing in the node are a
    // witness of memory.
    let cols: Vec<&str> = tr
        .order
        .first()
        .map(|s| {
            s.vars()
                .iter()
                .map(Symbol::as_str)
                .filter(|v| *v != node.as_str())
                .collect()
        })
        .unwrap_or_default();

    let mut seen: BTreeMap<Minterm<Symbol>, bool> = BTreeMap::new();
    let mut moves = false;
    for (i, s) in tr.order.iter().enumerate() {
        if s.value_of(clock.as_str()) != Some(level) || is_forced(s) {
            continue;
        }
        let Some(v) = m.output_value(node.as_str(), s) else {
            continue;
        };
        if let Some(prev) = seen.insert(s.project_to(cols.iter().copied()), v) {
            if prev != v {
                return false; // memory: the phase does not pin the node
            }
        }
        if !moves {
            for (xi, x) in m.cell.inputs.iter().enumerate() {
                if x == clock {
                    continue;
                }
                if let Some(ni) = tr.next[i][xi] {
                    let dest = &tr.order[ni];
                    if !is_forced(dest) && m.output_value(node.as_str(), dest) != Some(v) {
                        moves = true;
                        break;
                    }
                }
            }
        }
    }
    moves
}

/// A MEMORY WITNESS for state variable `w` in `clock`'s `level` phase: two forcing-excluded ELIGIBLE
/// stable states of that phase that DIFFER in `w` while AGREEING on the inputs and on every state variable
/// NOT DRIVEN BY `w` in that phase. When such a pair exists the phase is not pinning `w` — something is
/// being remembered across it, the latch signature. [`stores`]`(w, K, source-level)` and not
/// [`stores`]`(w, K, delivered-level)` is condition 1': `K`'s edge takes `w` from opaque (remembers) to
/// transparent (pinned by the phase).
///
/// It is reshaped from a phase-transparency test with two load-bearing changes:
///
/// 1. **No MOVING conjunct.** A memory witness is enough — the `DFF` slave holds its value across its own
///    hold phase without moving there (its tracked master is frozen at the moment of the edge), and the
///    slave is exactly what opens. Requiring movement would deny it.
/// 2. **PER-PHASE DIRECTIONAL co-mover projection.** A co-mover `W'` is projected out of the witness
///    columns ONLY where `w` DRIVES it in this phase — flipping `w` at an eligible in-phase stable state
///    and re-settling leaves `w` flipped (it did not snap back — `w` is not transparent here) and moves
///    `W'` to a different defined value. A `W'` that snaps back, is unmoved, or only oscillates keeps its
///    column. This is what makes a toggle flop's `Q` open (a global, undirected projection did not) while
///    a NAND latch's `Qn = !Q` — driven by `Q` in both phases — is still projected out, so the NAND form
///    classifies like its pass-transistor twin.
///
/// The FORCING region takes no part: a set/clear that overrides `w` in the phase is a coexisting
/// combinational arc, not memory, so both witness states must lie outside it.
fn stores<B: Brand, C: ManagerCell>(
    m: &Machine<'_, B, C>,
    tr: &Transitions<'_>,
    eligible: &[bool],
    w: &Symbol,
    clock: &Symbol,
    level: bool,
    forcing: &BTreeMap<Symbol, (bool, bool)>,
) -> bool {
    let is_forced = |s: &Minterm<Symbol>| {
        forcing
            .iter()
            .any(|(p, (a, _))| s.value_of(p.as_str()) == Some(*a))
    };
    let state_set: BTreeSet<&str> = m.state_vars.iter().map(Symbol::as_str).collect();
    let builder = m.deltas.first().map(|(_, d)| d.builder());
    // Does `w` DRIVE `wprime` in this phase? At some eligible, forcing-excluded in-phase stable state,
    // FREEZE `w` at its flipped value (withhold its δ, hold it there) and re-settle the rest: `wprime` is
    // driven when it moves to a different defined value. Freezing is what makes this a clean causal probe —
    // a free toggle of a cross-coupled complement pair (`Qn = !Q`) merely oscillates, settling nowhere, so
    // the complement would never be recognised as driven and a NAND latch would not open; holding `w`
    // resolves the pair (`Qn` follows to `!w`) so the NAND form classifies like its pass-transistor twin.
    // A settle that still oscillates yields no witness and keeps the column.
    let driven_by_w = |wprime: &str| -> bool {
        let Some(builder) = builder.as_ref() else {
            return false;
        };
        tr.order.iter().enumerate().any(|(i, s)| {
            if !eligible[i] || s.value_of(clock.as_str()) != Some(level) || is_forced(s) {
                return false;
            }
            let Some(w0) = s.value_of(w.as_str()) else {
                return false;
            };
            let Some(p0) = s.value_of(wprime) else {
                return false;
            };
            let frozen = freeze_delta(builder, &m.deltas, w, !w0);
            match machine::settle(&frozen, &machine::toggle(s, &[w.as_str()])) {
                Some(re) => matches!(re.value_of(wprime), Some(p1) if p1 != p0),
                None => false,
            }
        })
    };
    // The witness columns: every column but `w`, dropping any state variable `w` drives in this phase.
    let cols: Vec<&str> = tr
        .order
        .first()
        .map(|s| {
            s.vars()
                .iter()
                .map(Symbol::as_str)
                .filter(|c| *c != w.as_str())
                .filter(|c| !state_set.contains(c) || !driven_by_w(c))
                .collect()
        })
        .unwrap_or_default();

    let mut seen: BTreeMap<Minterm<Symbol>, bool> = BTreeMap::new();
    for (i, s) in tr.order.iter().enumerate() {
        if !eligible[i] || s.value_of(clock.as_str()) != Some(level) || is_forced(s) {
            continue;
        }
        let Some(v) = m.output_value(w.as_str(), s) else {
            continue;
        };
        if let Some(prev) = seen.insert(s.project_to(cols.iter().copied()), v) {
            if prev != v {
                return true; // two phase states agree on the columns yet differ in `w`: memory
            }
        }
    }
    false
}

/// `deltas` with `w`'s next-state function replaced by the constant `value`, freezing `w` there while
/// everything else settles. The order is preserved (a positional slice of the node's state columns), so
/// [`machine::settle`] over the result is well-formed.
fn freeze_delta<B: Brand, C: ManagerCell>(
    builder: &BddBuilder<B, C>,
    deltas: &[machine::Delta<B, C>],
    w: &Symbol,
    value: bool,
) -> Vec<machine::Delta<B, C>> {
    deltas
        .iter()
        .map(|(n, d)| {
            if n == w {
                (n.clone(), builder.constant(value))
            } else {
                (n.clone(), d.clone())
            }
        })
        .collect()
}

/// Arm (b) FROZEN-EXPOSURE for output `o` at a firing's destination `np`: is there an ELIGIBLE reachable
/// stable state anchored at `np` — same input projection — agreeing with `np` on every latch except ONE
/// K-ASSOCIATED state variable `W`, where `o`'s value differs? Then `o`'s post-edge value depends on the
/// frozen content of a latch the clock's edge exposes (`DET`'s `Q` reads whichever of `L1`/`L2` the edge
/// froze). `k_associated(W)` reuses the per-phase [`stores`]: `W`'s two phases of `K` disagree.
fn frozen_exposure<B: Brand, C: ManagerCell>(
    m: &Machine<'_, B, C>,
    tr: &Transitions<'_>,
    eligible: &[bool],
    inputs: &[Symbol],
    o: &str,
    np: &Minterm<Symbol>,
    k_associated: &impl Fn(&Symbol) -> bool,
) -> bool {
    let sp_inputs = np.project_to(inputs.iter().map(Symbol::as_str));
    let Some(o_sp) = m.output_value(o, np) else {
        return false;
    };
    for (i, t) in tr.order.iter().enumerate() {
        if !eligible[i] || t.project_to(inputs.iter().map(Symbol::as_str)) != sp_inputs {
            continue;
        }
        // Differ from `np` in exactly one state variable.
        let mut diff: Option<&Symbol> = None;
        let mut single = true;
        for w in &m.state_vars {
            if t.value_of(w.as_str()) != np.value_of(w.as_str()) {
                if diff.is_some() {
                    single = false;
                    break;
                }
                diff = Some(w);
            }
        }
        if !single {
            continue;
        }
        let Some(w) = diff else { continue };
        if k_associated(w) && m.output_value(o, t) != Some(o_sp) {
            return true;
        }
    }
    false
}

/// Arm (c) MASKED-DELIVERY for a firing `s -> np` of clock `k` on output `o` (which CHANGED): withhold an
/// opened latch's δ — freeze it at its pre-edge value, so it cannot deliver — and re-settle the same
/// firing. If `o`'s change REVERTS (`o` returns to its pre-edge value), `o` carried the opener's delivered
/// content, so the edge is what moved it. `o` itself, if opened, is handled by arm (a) and excluded here.
#[allow(clippy::too_many_arguments)]
fn masked_delivery<B: Brand, C: ManagerCell>(
    m: &Machine<'_, B, C>,
    builder: Option<&BddBuilder<B, C>>,
    deltas: &[machine::Delta<B, C>],
    o: &str,
    k: &Symbol,
    opened: &[&Symbol],
    s: &Minterm<Symbol>,
    np: &Minterm<Symbol>,
) -> bool {
    let (Some(before), Some(after)) = (m.output_value(o, s), m.output_value(o, np)) else {
        return false;
    };
    if before == after {
        return false; // masked-delivery reasons about a change to revert
    }
    let builder = builder.expect("a non-empty opener set implies a state variable and a builder");
    for w in opened.iter().filter(|w| w.as_str() != o) {
        let Some(wv) = s.value_of(w.as_str()) else {
            continue;
        };
        let frozen = freeze_delta(builder, deltas, w, wv);
        if let Some(re) = machine::settle(&frozen, &machine::toggle(s, &[k.as_str()])) {
            if m.output_value(o, &re) == Some(before) {
                return true;
            }
        }
    }
    false
}

/// The greatest-fixpoint filter that keeps `s` to the `(clock, direction)` toggles whose DELIVERED VALUE
/// HOLDS through the phase. A `(k, d)` is removed when some NON-FORCING change of `node` inside its
/// delivered phase (`clock == d`) happens at a toggle that is NOT itself an edge of `s` — live data (a
/// non-clock input) or a non-seam clock. A co-resident clock's edge that IS in `s` is another seam of the
/// node, not a disqualifier, so iterating to a fixpoint lets one seam's removal cascade to another
/// (`MCDFF` loses `(CLKB, Rise)` on live D, then `(CLKA, Fall)` because the in-phase CLKB rise is gone).
/// Only ELIGIBLE states, at both ends of a transition, take part.
#[allow(clippy::too_many_arguments)]
fn seam_fixpoint<B: Brand, C: ManagerCell>(
    m: &Machine<'_, B, C>,
    tr: &Transitions<'_>,
    eligible: &[bool],
    inputs: &[Symbol],
    clock_set: &BTreeSet<&str>,
    node: &str,
    node_forcing: &BTreeMap<Symbol, (bool, bool)>,
    s: &mut BTreeSet<Arc>,
) {
    let is_forced = |st: &Minterm<Symbol>| {
        node_forcing
            .iter()
            .any(|(p, (a, _))| st.value_of(p.as_str()) == Some(*a))
    };
    loop {
        let mut to_remove: Option<Arc> = None;
        'search: for (k, is_rise) in s.iter() {
            for (si, st) in tr.order.iter().enumerate() {
                if !eligible[si] || st.value_of(k.as_str()) != Some(*is_rise) || is_forced(st) {
                    continue;
                }
                let Some(v) = m.output_value(node, st) else {
                    continue;
                };
                for (xi, x) in inputs.iter().enumerate() {
                    if x == k {
                        continue;
                    }
                    let Some(ni) = tr.next[si][xi] else { continue };
                    if !eligible[ni] {
                        continue;
                    }
                    let dest = &tr.order[ni];
                    if is_forced(dest) {
                        continue;
                    }
                    match m.output_value(node, dest) {
                        Some(dv) if dv != v => {}
                        _ => continue, // the node did not move (or is undefined) here
                    }
                    if node_forcing.contains_key(x) {
                        continue; // a forcing pin's assertion is a coexisting combinational arc
                    }
                    // Is the toggle of `x` itself an edge of the node's current seam set?
                    let x_is_seam = clock_set.contains(x.as_str()) && {
                        let xdir = dest.value_of(x.as_str()) == Some(true);
                        s.contains(&(x.clone(), xdir))
                    };
                    if !x_is_seam {
                        to_remove = Some((k.clone(), *is_rise));
                        break 'search;
                    }
                }
            }
        }
        match to_remove {
            Some(r) => {
                s.remove(&r);
            }
            None => break,
        }
    }
}

/// The node's FORCING PINS, classified behaviourally from its own observed moves: a pin is forcing
/// iff every (undiscounted) move it causes lands the node on ONE constant value with one uniform
/// destination level of the pin — a set or clear, whatever the pin's declared class. Stratified:
/// moves whose source or destination lie under an already-established forcing pin's asserted level
/// are discounted before re-classifying (a clear pulsing the node inside a preset's region is still
/// a clear). A pin dragging the node BOTH ways (a tracked data pin) never classifies. Returns
/// `pin -> (asserted level, forced node value)`.
fn forcing_pins<B: Brand, C: ManagerCell>(
    m: &Machine<'_, B, C>,
    tr: &Transitions<'_>,
    node: &Symbol,
    moves: &[(Symbol, Minterm<Symbol>, Minterm<Symbol>, bool)],
) -> BTreeMap<Symbol, (bool, bool)> {
    let inputs = &m.cell.inputs;
    // Clause 2 - GLOBAL CLAMP: exactly one level of the pin pins the node to one constant across ALL
    // reachable stable states (an async override whose release re-acquires, like a toggle flop's reset).
    // No tracked data pin satisfies this: its tracking is confined to a clock-phase region, and elsewhere
    // the node varies under the same pin level. A REAL capture clock never pins the node to a constant
    // either (the node carries content in both phases), so declaration plays no part — a level-forcing
    // reset is a forcing pin whether or not it was declared a clock (the behavioural principle the deleted
    // level veto used to serve for `RDFF`'s clock-declared `R`).
    let mut clamp: BTreeMap<Symbol, (bool, bool)> = BTreeMap::new();
    for x in inputs {
        let clamp_value = |level: bool| -> Option<bool> {
            let mut seen: Option<bool> = None;
            for s in tr.order {
                if s.value_of(x.as_str()) != Some(level) {
                    continue;
                }
                let Some(v) = m.output_value(node.as_str(), s) else {
                    continue;
                };
                match seen {
                    None => seen = Some(v),
                    Some(p) if p == v => {}
                    _ => return None,
                }
            }
            seen
        };
        match (clamp_value(false), clamp_value(true)) {
            (Some(_), Some(_)) | (None, None) => {} // both levels clamp (degenerate) or neither
            (Some(v), None) => {
                clamp.insert(x.clone(), (false, v));
            }
            (None, Some(v)) => {
                clamp.insert(x.clone(), (true, v));
            }
        }
    }
    // Monotone accumulation: established forcing pins are never re-litigated, so each round can only
    // ADD pins and the loop terminates within `inputs.len()` rounds.
    let mut forcing: BTreeMap<Symbol, (bool, bool)> = clamp;
    loop {
        let mut added = false;
        for x in inputs {
            if forcing.contains_key(x) {
                continue;
            }
            let mut dest_levels: BTreeSet<bool> = BTreeSet::new();
            let mut posts: BTreeSet<bool> = BTreeSet::new();
            let mut any = false;
            for (pin, src, dest, post) in moves {
                if pin != x {
                    continue;
                }
                any = true;
                let discounted = forcing.iter().any(|(p, (a, _))| {
                    p != x
                        && (src.value_of(p.as_str()) == Some(*a)
                            || dest.value_of(p.as_str()) == Some(*a))
                });
                if discounted {
                    continue;
                }
                if let Some(l) = dest.value_of(x.as_str()) {
                    dest_levels.insert(l);
                }
                posts.insert(*post);
            }
            if any && dest_levels.len() == 1 && posts.len() == 1 {
                forcing.insert(
                    x.clone(),
                    (
                        dest_levels.into_iter().next().unwrap(),
                        posts.into_iter().next().unwrap(),
                    ),
                );
                added = true;
            }
        }
        if !added {
            return forcing;
        }
    }
}

/// Are the `(pre-projection, post-value)` samples CONFLICT-FREE over `cols` — no two samples whose
/// pre-states project equally deliver different values? The drop-loop's sole test: a sample-level
/// grouping, no BDD quantification.
fn conflict_free(cols: &[Symbol], samples: &[(Minterm<Symbol>, bool)]) -> bool {
    let mut seen: HashMap<Minterm<Symbol>, bool> = HashMap::new();
    for (pre, post) in samples {
        let proj = pre.project_to(cols.iter().map(Symbol::as_str));
        if let Some(prev) = seen.insert(proj, *post) {
            if prev != *post {
                return false;
            }
        }
    }
    true
}

/// The cover columns for one edge's samples: start from the uniform `header`, then attempt to drop each
/// candidate column, permanently keeping any drop that leaves the samples conflict-free. Inputs are never
/// dropped. The DROP ORDER is fold-eligibility order — fold-eligible level internals (`fold_eligible`,
/// the seam-empty internals settled before synthesis) first, then everything else (outputs and edge-form
/// nodes), reverse header order within each class — so a cover prefers columns that survive emission and
/// the internals the fold wants gone drop out. Deterministic and espresso-independent (`ICM`'s
/// inter-determined `sela1`/`sela2` force the ordering: an unordered loop could legally keep `sela1`).
fn drop_loop_columns(
    header: &[Symbol],
    samples: &[(Minterm<Symbol>, bool)],
    inputs_set: &BTreeSet<&str>,
    fold_eligible: &BTreeSet<Symbol>,
) -> Vec<Symbol> {
    let mut class1: Vec<Symbol> = header
        .iter()
        .filter(|c| !inputs_set.contains(c.as_str()) && fold_eligible.contains(*c))
        .cloned()
        .collect();
    let mut class2: Vec<Symbol> = header
        .iter()
        .filter(|c| !inputs_set.contains(c.as_str()) && !fold_eligible.contains(*c))
        .cloned()
        .collect();
    class1.reverse();
    class2.reverse();

    let mut kept: Vec<Symbol> = header.to_vec();
    for col in class1.iter().chain(&class2) {
        let trial: Vec<Symbol> = kept.iter().filter(|c| *c != col).cloned().collect();
        if conflict_free(&trial, samples) {
            kept = trial;
        }
    }
    kept
}

/// Synthesise a node's captures and off-edge over ONE uniform header (all inputs except the keying clock
/// plus every candidate), with the [`drop_loop_columns`] preference applied to each edge's cover before
/// [`generalise`]. The uniform header is conflict-free by construction — within a `(clock, direction)`
/// sample group the clock's pre-level is fixed, the header carries every other input and every state
/// variable, so equal projections are the same machine minterm and `settle` is deterministic; a conflict
/// cannot occur (asserted, never retried). The off-edge is synthesised JOINTLY over the node's whole
/// clock set.
#[allow(clippy::too_many_arguments)]
fn synth_node_captures<B: Brand, C: ManagerCell>(
    builder: &BddBuilder<B, C>,
    candidates: &[Symbol],
    fold_eligible: &BTreeSet<Symbol>,
    inputs_set: &BTreeSet<&str>,
    inputs: &[Symbol],
    clock_edges: &[(Symbol, Vec<(bool, Edge)>)],
    agg: &CandAgg,
) -> Synthesised {
    let clocks: Vec<Symbol> = clock_edges.iter().map(|(c, _)| c.clone()).collect();

    // Capture per clock (input-pin order), per active edge (Rise first). The uniform header is the inputs
    // minus THAT capture's clock, then every candidate signal name; any OTHER clock stays as a level
    // column. The drop-loop then sheds the columns emission does not need.
    let mut captures: Vec<(Symbol, Edge, StateRegions)> = Vec::new();
    for (clock, edges) in clock_edges {
        let header: Vec<Symbol> = inputs
            .iter()
            .filter(|p| p.as_str() != clock.as_str())
            .cloned()
            .chain(candidates.iter().cloned())
            .collect();

        for (is_rise, edge) in edges {
            let samples = agg
                .captures
                .get(&(clock.clone(), *is_rise))
                .map(|c| c.samples.as_slice())
                .unwrap_or(&[]);
            let cols = drop_loop_columns(&header, samples, inputs_set, fold_eligible);
            let sr = synth_capture(builder, &cols, samples)
                .expect("the uniform header is conflict-free, so no dropped subset conflicts");
            captures.push((clock.clone(), *edge, sr));
        }
    }

    // Off-edge over ALL the inputs, the node's own clocks INCLUDED: the hold-and-set/clear behaviour is
    // input driven, so the state coordinates are not columns (the value held is the node's own, absent
    // from the header, and any forcing comes from an input). A data input that never forces simply lands
    // every projection in `hold` and drops out of the cols; a PHASE-AGREED forcing makes each clock a
    // don't-care in every forcing cube, so it drops out of the cover support too, while a phase-CONDITIONED
    // reset keeps its gating clock pinned to the forcing level (`CLK*R`).
    let off_edge = synth_off_edge(builder, inputs, &clocks, &agg.stable);

    (captures, off_edge)
}

/// The three-valued phase classification of a projection's observed values.
#[derive(PartialEq, Clone, Copy)]
enum Phase {
    Forced1,
    Forced0,
    Held,
}

/// Classify one phase's observed values: all high, all low, or mixed (held). `None` when unobserved.
fn phase_class(vals: &[bool]) -> Option<Phase> {
    if vals.is_empty() {
        None
    } else if vals.iter().all(|b| *b) {
        Some(Phase::Forced1)
    } else if vals.iter().all(|b| !*b) {
        Some(Phase::Forced0)
    } else {
        Some(Phase::Held)
    }
}

/// Build the BDD of a fully/partly-fixed minterm as a cube (AND of its fixed literals; don't-cares
/// skipped). Mirrors the `regions.rs` reconstruction idiom.
fn cube_bdd<B: Brand, C: ManagerCell>(
    builder: &BddBuilder<B, C>,
    m: &Minterm<Symbol>,
) -> Bdd<B, C> {
    let mut p = builder.constant(true);
    for (v, val) in m.vars().iter().zip(m.iter()) {
        match val {
            Some(true) => p = p.and(&builder.var(v.as_str())),
            Some(false) => p = p.and(&!&builder.var(v.as_str())),
            None => {}
        }
    }
    p
}

/// The support of the given BDDs, restricted to and ordered by `header` (mirrors `regions.rs`'s
/// self-projected column rule).
fn support_in_header<B: Brand, C: ManagerCell>(
    bdds: &[&Bdd<B, C>],
    header: &[Symbol],
) -> Vec<Symbol> {
    let sup: BTreeSet<Symbol> = bdds.iter().flat_map(|b| b.variables()).collect();
    header
        .iter()
        .filter(|h| sup.contains(*h))
        .cloned()
        .collect()
}

/// Assemble a [`StateRegions`] from an on/off region-BDD pair over `header`, reusing the `regions.rs`
/// cover pipeline so the emitted cubes are byte-compatible. `hold_bdd` is the quiescent gap (empty for a
/// total capture).
fn regions_from<B: Brand, C: ManagerCell>(
    on_bdd: &Bdd<B, C>,
    off_bdd: &Bdd<B, C>,
    hold_bdd: &Bdd<B, C>,
    header: &[Symbol],
) -> StateRegions {
    let cols = support_in_header(&[on_bdd, off_bdd], header);
    let on_cover = regions::minimise(regions::f_side(
        &on_bdd.cover_over_fr(cols.iter().map(Symbol::as_str)),
    ));
    let off_cover = regions::minimise(regions::f_side(
        &off_bdd.cover_over_fr(cols.iter().map(Symbol::as_str)),
    ));
    let hold_cover = regions::minimise_bdd(hold_bdd);
    StateRegions {
        on: regions::region_cubes(&on_cover, &cols),
        off: regions::region_cubes(&off_cover, &cols),
        hold: regions::region_cubes(&hold_cover, &cols),
        cols,
        on_cover,
        off_cover,
        hold_cover,
        hysteretic: true,
    }
}

/// Synthesise a capture region from its `(pre-state, post-value)` samples over `header`. The witnessed
/// on-samples are the ON-set, the witnessed off-samples the OFF-set and the unwitnessed remainder a
/// don't-care set: the capture is the ON-set generalised (incompletely-specified minimisation) so it
/// generalises past the reachable pre-states to the underlying function — reachability need not cover
/// every projection for the cover to land on the true capture. The generalised on-set is total, its off
/// the exact complement (empty hold). Returns `None` when a projection carries both an on- and an
/// off-sample (a conflict that tier-2 must disambiguate).
fn synth_capture<B: Brand, C: ManagerCell>(
    builder: &BddBuilder<B, C>,
    header: &[Symbol],
    samples: &[(Minterm<Symbol>, bool)],
) -> Option<StateRegions> {
    let mut on_pts = builder.constant(false);
    let mut off_pts = builder.constant(false);
    for (pre, post) in samples {
        let cube = cube_bdd(builder, &pre.project_to(header.iter().map(Symbol::as_str)));
        if *post {
            on_pts = on_pts.or(&cube);
        } else {
            off_pts = off_pts.or(&cube);
        }
    }
    if !on_pts.and(&off_pts).is_contradiction() {
        return None; // a projection is both on and off under this header
    }
    let on_bdd = generalise(builder, &on_pts, &off_pts, header);
    let off_bdd = !&on_bdd;
    let hold = builder.constant(false);
    Some(regions_from(&on_bdd, &off_bdd, &hold, header))
}

/// Generalise a witnessed on-set against a witnessed off-set, treating the unwitnessed remainder as a
/// don't-care set (incompletely-specified minimisation over `CoverType::FR`). Returns the minimised
/// on-set as a BDD. When either side is empty there is no boundary to generalise against, so the
/// witnessed on-set is returned unchanged (avoiding a collapse to a constant over the all-don't-care
/// space).
fn generalise<B: Brand, C: ManagerCell>(
    builder: &BddBuilder<B, C>,
    on_pts: &Bdd<B, C>,
    off_pts: &Bdd<B, C>,
    header: &[Symbol],
) -> Bdd<B, C> {
    if on_pts.is_contradiction() || off_pts.is_contradiction() {
        return on_pts.clone();
    }
    let cols = support_in_header(&[on_pts, off_pts], header);
    let cols_str = || cols.iter().map(Symbol::as_str);
    // The F cubes of `on_pts` and the R cubes of `off_pts` (the latter read off `¬off_pts`'s FR cover),
    // assembled into one FR cover whose don't-care set is everything neither on nor off.
    let on_fr = on_pts.cover_over_fr(cols_str());
    let off_fr = (!off_pts).cover_over_fr(cols_str());
    let fr = Cover::from_cubes(
        CoverType::FR,
        on_fr
            .cubes()
            .filter(|c| c.cube_type() == CubeType::F)
            .cloned()
            .chain(
                off_fr
                    .cubes()
                    .filter(|c| c.cube_type() == CubeType::R)
                    .cloned(),
            ),
    );
    match fr.minimize() {
        Ok(min) => builder.build_cover(&regions::f_side(&min)),
        Err(_) => on_pts.clone(),
    }
}

/// Synthesise the off-edge (hold + set/clear) region from the stable-state samples over `header_off` —
/// the CLOCK-INCLUSIVE header, every input of the cell. Each stable sample is keyed by the PHASE VECTOR of
/// `clocks` (a sample with any of those clocks unset is skipped) and then phase-classified: a Forced class
/// gives the set/clear cover, a mixed one defaults to hold.
///
/// Because the header names the clocks, a projection fully determines the phase, so no two-phase
/// disagreement can arise. A phase-AGREED forcing makes each clock a don't-care in every forcing cube and
/// it drops back out of the cover support (a universal DFF hold stays a universal hold, a both-latch reset
/// stays `R`); a phase-CONDITIONED reset instead keeps its gating clock pinned to the forcing level, so it
/// synthesises first-class as `CLK*R` — a combinational reset whose condition includes the clock.
fn synth_off_edge<B: Brand, C: ManagerCell>(
    builder: &BddBuilder<B, C>,
    header_off: &[Symbol],
    clocks: &[Symbol],
    stable: &[(Minterm<Symbol>, bool)],
) -> StateRegions {
    // Group the stable samples by off-edge projection, then by the clocks' phase vector.
    let mut groups: BTreeMap<Minterm<Symbol>, BTreeMap<Vec<bool>, Vec<bool>>> = BTreeMap::new();
    'sample: for (state, val) in stable {
        let mut phase: Vec<bool> = Vec::with_capacity(clocks.len());
        for c in clocks {
            match state.value_of(c.as_str()) {
                Some(b) => phase.push(b),
                None => continue 'sample, // a keying clock unset ⇒ skip this sample
            }
        }
        let proj = state.project_to(header_off.iter().map(Symbol::as_str));
        groups
            .entry(proj)
            .or_default()
            .entry(phase)
            .or_default()
            .push(*val);
    }

    let mut on_pts = builder.constant(false);
    let mut off_pts = builder.constant(false);
    for (proj, phases) in &groups {
        // The header names the clocks, so a projection carries exactly one phase vector — the classes
        // cannot disagree, and a mixed projection is simply held.
        let agreed = phases.values().find_map(|vals| phase_class(vals));
        let cube = cube_bdd(builder, proj);
        match agreed {
            Some(Phase::Forced1) => on_pts = on_pts.or(&cube),
            Some(Phase::Forced0) => off_pts = off_pts.or(&cube),
            _ => {} // held or unobserved ⇒ hold
        }
    }

    let hold = !&on_pts.or(&off_pts);
    regions_from(&on_pts, &off_pts, &hold, header_off)
}

/// The node's column set: the first-appearance union of every capture's cols then the off-edge's cols.
fn capture_cols(captures: &[(Symbol, Edge, StateRegions)], off_edge: &StateRegions) -> Vec<Symbol> {
    let mut cols: Vec<Symbol> = Vec::new();
    let sources = captures
        .iter()
        .map(|(_, _, sr)| &sr.cols)
        .chain([&off_edge.cols]);
    for src in sources {
        for s in src {
            if !cols.contains(s) {
                cols.push(s.clone());
            }
        }
    }
    cols
}

#[cfg(test)]
mod tests {
    use super::*;
    use espresso_logic::sync_bdd_builder;
    use std::collections::BTreeSet;

    /// Replay `Cell::analyse`'s model pipeline for a single-cell TOML, then run the body with the shared
    /// builder, the analysed cell, the minimised BDD map and the built `Machine` bound to the given
    /// idents.
    macro_rules! with_machine {
        ($src:expr, |$builder:ident, $analysed:ident, $bdds:ident, $m:ident| $body:block) => {{
            let mut $analysed = crate::model::parse_spec($src)
                .unwrap()
                .cells
                .remove(0)
                .analyse_signals()
                .unwrap();
            let $builder = sync_bdd_builder!();
            let mut $bdds = crate::model::build_signal_bdds(&$analysed, &$builder);
            let order: Vec<Symbol> = $analysed.signals().map(|s| s.name.clone()).collect();
            let output_set: BTreeSet<Symbol> =
                $analysed.outputs.iter().map(|o| o.name.clone()).collect();
            let min = crate::logic::minimise::minimise_state_space(&mut $bdds, &order, &output_set);
            crate::model::recompute_signal_metadata(&mut $analysed, &$bdds, &min);
            let $m = crate::logic::analysis::Machine::build(&$analysed, &$bdds).unwrap();
            $body
        }};
    }

    /// Tests call the classifier THROUGH the pipeline, exactly as `analyse_machine` wires it: derive
    /// the delay arcs first, then label against them. Shadowing the glob-imported `super::classify`
    /// keeps every call site in the natural form.
    fn classify<B: Brand, C: ManagerCell + Send + Sync>(m: &Machine<B, C>) -> EdgeArcs {
        let (arcs, _) = crate::logic::arcs::derive(m);
        super::classify(m, &arcs)
    }

    fn cols_of(sr: &StateRegions) -> Vec<&str> {
        sr.cols.iter().map(Symbol::as_str).collect()
    }

    fn clocks_of(er: &EdgeCaptures) -> Vec<&str> {
        er.clocks().into_iter().map(Symbol::as_str).collect()
    }

    fn reg<'a>(es: &'a EdgeArcs, node: &str) -> &'a EdgeCaptures {
        es.captures
            .iter()
            .find(|r| r.node.as_str() == node)
            .unwrap_or_else(|| panic!("no edge arcs for {node}: {:?}", node_list(es)))
    }

    fn node_list(es: &EdgeArcs) -> Vec<&str> {
        es.captures.iter().map(|r| r.node.as_str()).collect()
    }

    fn folded_list(es: &EdgeArcs) -> Vec<&str> {
        es.folded.iter().map(Symbol::as_str).collect()
    }

    /// The DISTINCT edge arcs as `(output, clock, clock direction)` tuples, in sorted order. The label
    /// set keys on the full start context, so several firings of one `(output, clock, direction)` collapse
    /// to a single tuple here — the test cares which directions are edge, not how many contexts present
    /// them.
    fn label_list(es: &EdgeArcs) -> Vec<(&str, &str, Edge)> {
        let mut v: Vec<(&str, &str, Edge)> = es
            .labels
            .iter()
            .map(|(n, c, e, _)| (n.as_str(), c.as_str(), *e))
            .collect();
        v.sort();
        v.dedup();
        v
    }

    /// The edge arcs carried by one output, as `(clock, clock direction)`.
    fn labels_of<'a>(es: &'a EdgeArcs, node: &str) -> Vec<(&'a str, Edge)> {
        label_list(es)
            .into_iter()
            .filter(|(n, _, _)| *n == node)
            .map(|(_, c, e)| (c, e))
            .collect()
    }

    /// Replay-faithfulness harness: prove the emitted edge arcs against the machine's own
    /// [`machine::toggle`]/[`machine::settle`] behaviour over every reachable stable state. The arcs plus
    /// the off-edge are read as ONE joint model — in an OPEN (genuinely transparent) phase the node equals
    /// the capture cover, elsewhere it holds unless a forcing region overrides it — and the model is
    /// replayed against the cell. Five clauses, none of them polarity-blind:
    ///
    /// 1. **TRANSPARENCY** — in an open phase the node equals that arc's capture cover at the CURRENT
    ///    state. A wrong polarity claims the closed phase is the open one, where this fails.
    /// 2. **CAPTURE** — a capturing edge fired from a matching pre-phase delivers the cover evaluated at
    ///    the pre-state (a forcing at the destination overriding it).
    /// 3. **NON-CAPTURING clock toggles** — the node lands on the joint model's prediction at the
    ///    destination, so a spurious or a missing direction shows up here.
    /// 4. **HOLD** — a non-clock toggle either lands in a forced region (predicted), is a RELEASE from one
    ///    (whose outcomes must agree per `(pin, destination projection)` — behavioural, no declared class)
    ///    or leaves the node unchanged.
    /// 5. **EXERCISE** — the no-vacuous guard in executable form: every emitted `(clock, direction)` has a
    ///    value-CHANGING firing or closes a genuinely TRANSPARENT phase. It rejects a `DFF` `CLK:Fall` arc
    ///    and an `HPIPE` `CLKA:Fall` arc (change-free with a hysteretic opposite phase) while admitting
    ///    `HPIPE` `CLKB:Rise` (change-free, but a real latch close into a transparent phase).
    fn assert_captures_faithful<B: Brand, C: ManagerCell + Send + Sync>(
        m: &Machine<B, C>,
        es: &EdgeArcs,
    ) {
        let Some((_, any_delta)) = m.deltas.first() else {
            return; // no state variables ⇒ nothing carries a capture
        };
        let builder = any_delta.builder();
        let tr = Transitions::build(m);
        let inputs = &m.cell.inputs;

        for r in &es.captures {
            let node = r.node.as_str();
            let value = |s: &Minterm<Symbol>| m.output_value(node, s);

            // The emitted model: one capture cover per arc, plus the clock-inclusive forcing covers.
            let covers: BTreeMap<Arc, Bdd<B, C>> = r
                .captures
                .iter()
                .map(|(clock, edge, sr)| {
                    (
                        (clock.clone(), *edge == Edge::Rise),
                        builder.build_cover(&sr.on_cover),
                    )
                })
                .collect();
            let forced_on = builder.build_cover(&r.off_edge.on_cover);
            let forced_off = builder.build_cover(&r.off_edge.off_cover);
            let forced = |s: &Minterm<Symbol>| {
                if forced_on.evaluate_fast(s) == Some(true) {
                    Some(true)
                } else if forced_off.evaluate_fast(s) == Some(true) {
                    Some(false)
                } else {
                    None
                }
            };

            // Does some firing of this arc CHANGE the node? The behavioural half of the no-vacuous guard,
            // and the discriminator between a latch CLOSE (change-free: the node already tracks the
            // delivered value through the open phase) and an edge that moves the node.
            let changes = |(clock, is_rise): &Arc| -> bool {
                let xi = inputs.iter().position(|p| p == clock).unwrap();
                tr.order.iter().enumerate().any(|(si, s)| {
                    s.value_of(clock.as_str()) == Some(!*is_rise)
                        && tr.next[si][xi].is_some_and(|ni| value(&tr.order[ni]) != value(s))
                })
            };
            let is_transparent = |(clock, is_rise): &Arc| -> bool {
                super::transparent(m, &tr, &r.node, clock, !*is_rise, (&forced_on, &forced_off))
            };

            // The OPEN phases: a CHANGE-FREE arc whose pre-phase is genuinely transparent is a latch
            // close, and its pre-phase is where the node tracks that arc's cover.
            let open: Vec<(Symbol, bool, &Bdd<B, C>)> = covers
                .keys()
                .filter(|arc| !changes(arc) && is_transparent(arc))
                .map(|arc| (arc.0.clone(), !arc.1, &covers[arc]))
                .collect();
            let open_at = |s: &Minterm<Symbol>| {
                open.iter()
                    .find(|(clock, level, _)| s.value_of(clock.as_str()) == Some(*level))
                    .map(|(_, _, cov)| *cov)
            };

            // (1) TRANSPARENCY.
            for s in tr.order {
                // An unsettled coordinate leaves the node's value undetermined there: nothing to prove.
                if forced(s).is_some() || value(s).is_none() {
                    continue;
                }
                if let Some(cov) = open_at(s) {
                    let got = value(s);
                    let want = cov.evaluate_fast(s);
                    if want.is_none() {
                        continue; // an undetermined cover column: nothing to prove
                    }
                    assert_eq!(
                        got, want,
                        "transparency unfaithful: node {node} at open state {s:?}: observed {got:?} \
                         != capture cover {want:?}"
                    );
                }
            }

            // (2)-(4) the replay. Release outcomes are collected and checked for agreement, never
            // exempted by a declared pin class.
            let mut releases: BTreeMap<(Symbol, Minterm<Symbol>), Option<bool>> = BTreeMap::new();
            for (si, s) in tr.order.iter().enumerate() {
                for (xi, x) in inputs.iter().enumerate() {
                    let Some(ni) = tr.next[si][xi] else { continue };
                    let dest = &tr.order[ni];
                    let (Some(_), Some(_)) = (value(s), value(dest)) else {
                        continue; // an undetermined coordinate at either end: nothing to prove
                    };
                    let got = value(dest);
                    let rising = s.value_of(x.as_str()) == Some(false);

                    // (2) CAPTURE: this toggle is an emitted arc firing from its matching pre-phase.
                    if let Some(cov) = covers.get(&(x.clone(), rising)) {
                        let want = forced(dest)
                            .map(Some)
                            .unwrap_or_else(|| cov.evaluate_fast(s));
                        if want.is_none() {
                            continue;
                        }
                        assert_eq!(
                            got, want,
                            "capture unfaithful: node {node}, clock {x} {} from pre {s:?} settled to \
                             {dest:?}: observed {got:?} != synthesised capture {want:?}",
                            if rising { "Rise" } else { "Fall" }
                        );
                        continue;
                    }

                    // (3)/(4) the joint model's prediction at the destination: a forcing, else the open
                    // phase's cover, else a release, else the held value.
                    let want = if let Some(v) = forced(dest) {
                        Some(v)
                    } else if let Some(cov) = open_at(dest) {
                        cov.evaluate_fast(dest)
                    } else if forced(s).is_some() {
                        // A RELEASE from a forcing region: the node re-acquires, which the covers do not
                        // model. Its outcomes must at least AGREE per (pin, destination projection).
                        let key = (
                            x.clone(),
                            dest.project_to(inputs.iter().map(Symbol::as_str)),
                        );
                        if let Some(prev) = releases.insert(key.clone(), got) {
                            assert_eq!(
                                prev, got,
                                "release unfaithful: node {node}, pin {x} into {:?}: {prev:?} != \
                                 {got:?}",
                                key.1
                            );
                        }
                        continue;
                    } else if m.cell.clock_pins.iter().any(|c| c == x)
                        && !covers.keys().any(|(k, _)| k == x)
                        && super::transparent(
                            m,
                            &tr,
                            &r.node,
                            x,
                            dest.value_of(x.as_str()) == Some(true),
                            (&forced_on, &forced_off),
                        )
                    {
                        // An UN-ARCED latch clock OPENING into its transparent phase: the node
                        // re-tracks the value captured upstream (HPIPE's CLKB fall revealing the
                        // CLKA capture), which the edge model does not predict. Require only
                        // DETERMINISM per (pin, destination-minus-node projection).
                        let others: Vec<&str> = dest
                            .vars()
                            .iter()
                            .map(Symbol::as_str)
                            .filter(|v| *v != node)
                            .collect();
                        let key = (x.clone(), dest.project_to(others));
                        if let Some(prev) = releases.insert(key.clone(), got) {
                            assert_eq!(
                                prev, got,
                                "reveal unfaithful: node {node}, latch clock {x} into {:?}: \
                                 {prev:?} != {got:?}",
                                key.1
                            );
                        }
                        continue;
                    } else {
                        value(s) // quiescent ⇒ the node holds
                    };
                    if want.is_none() {
                        continue;
                    }
                    assert_eq!(
                        got, want,
                        "off-edge unfaithful: node {node}, toggle {x} from {s:?} settled to {dest:?}: \
                         observed {got:?} != model prediction {want:?}"
                    );
                }
            }

            // (5) EXERCISE: no vacuous arc.
            for arc in covers.keys() {
                assert!(
                    changes(arc) || is_transparent(arc),
                    "vacuous edge arc: node {node}, clock {} {} never changes the node and its \
                     opposite phase is not transparent",
                    arc.0.as_str(),
                    if arc.1 { "Rise" } else { "Fall" }
                );
            }
        }
    }

    /// The machine-checkable form of statetable invariant I3 (the `debug_assert!` in
    /// `crate::emit::statetable`): NOTHING THAT SURVIVES EMISSION MAY NAME A FOLDED NODE. A fold — the
    /// group fold especially — drops the node's column from the emitted table, so a survivor still
    /// referencing it would emit a dangling column. For every folded name this checks both routes a
    /// reference can take: a surviving capture's cover columns, and the raw function of a surviving
    /// capture-less candidate (whose δ is emitted verbatim). Support is read SEMANTICALLY from the BDD,
    /// never from equation shape.
    fn assert_no_dropped_references<B: Brand, C: ManagerCell + Send + Sync>(
        m: &Machine<B, C>,
        es: &EdgeArcs,
    ) {
        let folded: BTreeSet<&str> = es.folded.iter().map(Symbol::as_str).collect();
        if folded.is_empty() {
            return;
        }

        // (a) no surviving capture names a folded node in its cover columns.
        for r in &es.captures {
            for col in &r.cols {
                assert!(
                    !folded.contains(col.as_str()),
                    "dropped reference: capture on {} names folded node {col}, folded {:?}",
                    r.node,
                    folded_list(es)
                );
            }
        }

        // (b) no surviving capture-less candidate's raw function has a folded node in its support. The
        // candidate population is the classifier's own: every output plus every non-output state
        // variable. A candidate that carries a capture is not a survivor of this kind — its raw function
        // is replaced by the edge seam — and the folded nodes themselves are gone.
        let output_names: BTreeSet<&str> = m.cell.outputs.iter().map(|o| o.name.as_str()).collect();
        let captured: BTreeSet<&str> = node_list(es).into_iter().collect();
        let mut fn_of: BTreeMap<&str, &Bdd<B, C>> = BTreeMap::new();
        for (n, d) in &m.deltas {
            fn_of.insert(n.as_str(), d);
        }
        for (n, d) in &m.out_deltas {
            fn_of.insert(n.as_str(), d);
        }
        let candidates = output_names
            .iter()
            .copied()
            .chain(
                m.state_vars
                    .iter()
                    .map(Symbol::as_str)
                    .filter(|sv| !output_names.contains(sv)),
            )
            .filter(|c| !captured.contains(c) && !folded.contains(c));
        for cand in candidates {
            let Some(f) = fn_of.get(cand) else { continue };
            for v in f.variables() {
                assert!(
                    !folded.contains(v.as_str()),
                    "dropped reference: surviving capture-less {cand} still names folded node {v}, \
                     folded {:?}",
                    folded_list(es)
                );
            }
        }
    }

    // --- fixtures ---

    const DFF_TOML: &str = r#"
[[cell]]
name = "DFF"
inputs = ["CLK", "D"]
clock = ["CLK"]
[cell.internal]
M = "!CLK*D + CLK*M"
[cell.outputs]
Q = "CLK*M + !CLK*Q"
"#;

    const ICM_TOML: &str = r#"
[[cell]]
name = "ICM"
inputs = ["CLKA", "CLKB", "RA", "RB", "S"]
clock = ["CLKA", "CLKB"]
[cell.internal]
sela = "!enB*!S"
selb = "!enA*S"
sela1 = "!RA*(!CLKA*sela+CLKA*sela1)"
sela2 = "!RA*(CLKA*sela1+!CLKA*sela2)"
enA   = "!RA*(!CLKA*sela2+CLKA*enA)"
selb1 = "!RB*(!CLKB*selb+CLKB*selb1)"
selb2 = "!RB*(CLKB*selb1+!CLKB*selb2)"
enB   = "!RB*(!CLKB*selb2+CLKB*enB)"
[cell.outputs]
GCLK = "enA*CLKA+enB*CLKB"
"#;

    // === Step 1 done-when / Step 3 (1) FLOOR ===

    #[test]
    fn edge_dff_floor() {
        with_machine!(DFF_TOML, |_b, _a, _m2, m| {
            let es = classify(&m);
            assert_captures_faithful(&m, &es);
            assert_eq!(node_list(&es), ["Q"], "only Q is a register");
            let q = reg(&es, "Q");
            assert_eq!(clocks_of(q), ["CLK"]);
            assert_eq!(q.captures.len(), 1);
            let (clk, edge, cap) = &q.captures[0];
            assert_eq!(clk, "CLK");
            assert_eq!(*edge, Edge::Rise);
            assert_eq!(cols_of(cap), ["D"]);
            assert_eq!(cap.on, vec![vec![Some(true)]]);
            assert_eq!(cap.off, vec![vec![Some(false)]]);
            assert!(cap.hold.is_empty(), "capture is total, empty hold");
            // off_edge: empty cols, universal hold.
            assert!(q.off_edge.cols.is_empty());
            assert!(q.off_edge.on.is_empty());
            assert!(q.off_edge.off.is_empty());
            assert_eq!(q.off_edge.hold, vec![vec![]], "universal hold");
            assert_eq!(q.cols.iter().map(Symbol::as_str).collect::<Vec<_>>(), ["D"]);
            assert_eq!(folded_list(&es), ["M"], "master M folded");
        });
    }

    #[test]
    fn edge_icm_floor() {
        with_machine!(ICM_TOML, |_b, _a, _m2, m| {
            let es = classify(&m);
            assert_captures_faithful(&m, &es);
            // GCLK = enA*CLKA + enB*CLKB is a combinational clock gate: `!CLKA*!CLKB` pins it to 0 with
            // the captured enables irrelevant, so the veto strips both clocks and it carries no arc.
            assert!(
                !node_list(&es).contains(&"GCLK"),
                "GCLK is a combinational clock gate"
            );
            // The enable flops keep a single falling capture each.
            for (name, clock) in [("enA", "CLKA"), ("enB", "CLKB")] {
                let r = reg(&es, name);
                assert_eq!(clocks_of(r), [clock], "{name}");
                assert_eq!(r.captures.len(), 1, "{name}");
                assert_eq!(r.captures[0].1, Edge::Fall, "{name}");
            }
            // Each synchroniser's three latches are two flops in series: sela1/selb1 fold as the
            // first flop's internal master, sela2/selb2 capture on the rising edge and enA/enB on
            // the falling edge — every capture on its chain's own single clock.
            for (name, clock) in [("sela2", "CLKA"), ("selb2", "CLKB")] {
                let r = reg(&es, name);
                assert_eq!(clocks_of(r), [clock], "{name} is single-clock");
                assert_eq!(r.captures.len(), 1, "{name}");
                assert_eq!(r.captures[0].1, Edge::Rise, "{name}");
            }
            let mut folded = folded_list(&es);
            folded.sort();
            assert_eq!(
                folded,
                ["sela1", "selb1"],
                "the internal masters fold, exactly"
            );
            for name in ["enA", "enB", "sela2", "selb2"] {
                assert!(
                    node_list(&es).contains(&name),
                    "{name} survives unfolded, node_list={:?}",
                    node_list(&es)
                );
            }
        });
    }

    // === Fixtures: stay-level cases ===

    const DLAT_TOML: &str = r#"
[[cell]]
name = "DLAT"
inputs = ["CLK", "D"]
clock = ["CLK"]
[cell.outputs]
Q = "CLK*D + !CLK*Q"
"#;

    const GLAT_TOML: &str = r#"
[[cell]]
name = "GLAT"
inputs = ["CLK", "D"]
clock = ["CLK"]
[cell.outputs]
Q = "CLK*(D+Q) + !CLK*Q"
"#;

    const MUX_TWO_CLOCK_TOML: &str = r#"
[[cell]]
name = "MUXLAT"
inputs = ["CLKA", "CLKB", "D"]
clock = ["CLKA", "CLKB"]
[cell.outputs]
Q = "CLKA*D + !CLKA*(CLKB*D + !CLKB*Q)"
"#;

    const UCDFF_TOML: &str = r#"
[[cell]]
name = "UCDFF"
inputs = ["CLK", "D"]
[cell.internal]
M = "!CLK*D + CLK*M"
[cell.outputs]
Q = "CLK*M + !CLK*Q"
"#;

    const MASTER_ONLY_RESET_TOML: &str = r#"
[[cell]]
name = "MOR"
inputs = ["CLK", "D", "R"]
clock = ["CLK"]
[cell.internal]
M = "!R*(!CLK*D + CLK*M)"
[cell.outputs]
Q = "CLK*M + !CLK*Q"
"#;

    const EXPOSED_MASTER_TOML: &str = r#"
[[cell]]
name = "EMDFF"
inputs = ["CLK", "D"]
clock = ["CLK"]
[cell.outputs]
Q = "CLK*M + !CLK*Q"
M = "!CLK*D + CLK*M"
"#;

    const TAPPED_MASTER_TOML: &str = r#"
[[cell]]
name = "TAPDFF"
inputs = ["CLK", "D"]
clock = ["CLK"]
[cell.internal]
M = "!CLK*D + CLK*M"
[cell.outputs]
Q = "CLK*M + !CLK*Q"
T = "M"
"#;

    const INVERTING_DFF_TOML: &str = r#"
[[cell]]
name = "IDFF"
inputs = ["CLK", "D"]
clock = ["CLK"]
[cell.internal]
M = "!CLK*D + CLK*M"
[cell.outputs]
Q = "CLK*!M + !CLK*Q"
"#;

    // === No capture kept: the node takes no edge annotation ===

    #[test]
    fn edge_stay_level_fixtures() {
        for (src, name) in [
            (DLAT_TOML, "Q"),
            (GLAT_TOML, "Q"),
            (UCDFF_TOML, "Q"),
            (MUX_TWO_CLOCK_TOML, "Q"),
        ] {
            with_machine!(src, |_b, _a, _m2, m| {
                let es = classify(&m);
                assert!(
                    !node_list(&es).contains(&name),
                    "{name} must not be a register in {:?}",
                    node_list(&es)
                );
            });
        }
    }

    // === Step 3 (3): behavioural F2 ===

    const MOR_ASYNC_TOML: &str = r#"
[[cell]]
name = "MORA"
inputs = ["CLK", "D", "R"]
clock = ["CLK"]
async = ["R"]
[cell.internal]
M = "!R*(!CLK*D + CLK*M)"
[cell.outputs]
Q = "CLK*M + !CLK*Q"
"#;

    const BOTH_RESET_TOML: &str = r#"
[[cell]]
name = "BR"
inputs = ["CLK", "D", "R"]
clock = ["CLK"]
async = ["R"]
[cell.internal]
M = "!R*(!CLK*D + CLK*M)"
[cell.outputs]
Q = "!R*(CLK*M + !CLK*Q)"
"#;

    #[test]
    fn edge_phase_conditioned_reset_is_a_forcing_not_a_blocker() {
        // R clears Q only while CLK=1 — a combinational reset whose condition includes the clock. With the
        // CLOCK-INCLUSIVE off-edge header that synthesises first-class as `CLK*R`, so Q keeps its rising
        // capture instead of being blocked. Declaring R async changes nothing: the decision is behavioural.
        for (src, label) in [
            (MASTER_ONLY_RESET_TOML, "sync R"),
            (MOR_ASYNC_TOML, "async-declared R"),
        ] {
            with_machine!(src, |builder, _a, _m2, m| {
                let es = classify(&m);
                assert_captures_faithful(&m, &es);
                let q = reg(&es, "Q");
                assert_eq!(clocks_of(q), ["CLK"], "{label}");
                assert_eq!(q.captures.len(), 1, "{label}: rise only");
                assert_eq!(q.captures[0].1, Edge::Rise, "{label}");
                let off = builder.build_cover(&q.off_edge.off_cover);
                let want = builder.var("CLK").and(&builder.var("R"));
                assert!(
                    off.equivalent_to(&want),
                    "{label}: off_edge.off is the phase-conditioned reset CLK*R"
                );
            });
        }
    }

    #[test]
    fn edge_both_latch_reset_recognised_with_async_off() {
        // R clears both latches ⇒ phase agreement ⇒ Q recognised, off_edge.off covers R.
        with_machine!(BOTH_RESET_TOML, |builder, _a, _m2, m| {
            let es = classify(&m);
            assert_captures_faithful(&m, &es);
            let q = reg(&es, "Q");
            // off_edge.off is forced-0 exactly where R is asserted.
            let off = builder.build_cover(&q.off_edge.off_cover);
            let r = builder.var("R");
            assert!(off.equivalent_to(&r), "off_edge.off must cover R");
        });
    }

    // === Step 3 (4): new recognitions ===

    #[test]
    fn edge_inverting_dff_captures_not_d() {
        with_machine!(INVERTING_DFF_TOML, |builder, _a, _m2, m| {
            let es = classify(&m);
            assert_captures_faithful(&m, &es);
            let q = reg(&es, "Q");
            assert_eq!(q.captures.len(), 1);
            let (_clk, edge, cap) = &q.captures[0];
            assert_eq!(*edge, Edge::Rise);
            // capture == !D, recorded verbatim (no special-casing).
            let on = builder.build_cover(&cap.on_cover);
            assert!(on.equivalent_to(&!&builder.var("D")), "capture must be !D");
            assert_eq!(folded_list(&es), ["M"]);
        });
    }

    #[test]
    fn edge_exposed_master_recognises_slave_over_surviving_master() {
        with_machine!(EXPOSED_MASTER_TOML, |builder, _a, _m2, m| {
            let es = classify(&m);
            assert_captures_faithful(&m, &es);
            let q = reg(&es, "Q");
            assert_eq!(q.captures[0].1, Edge::Rise);
            // M is an output master (never folded); the slave Q is recognised. The cover PREFERS the input
            // D over the internal M (cover columns prefer inputs), and D coincides with M over the CLK=0
            // capture domain — where the rising edge samples the master — so `D` and `M` are the same
            // captured value where the cover is ever evaluated.
            assert!(
                !folded_list(&es).contains(&"M"),
                "an output master is not folded"
            );
            assert_eq!(
                cols_of(&q.captures[0].2),
                ["D"],
                "the capture prefers input D"
            );
            // Over the CLK=0 (capture) domain of the reachable states, the cover (D) equals the master M.
            let mut reach0 = builder.constant(false);
            for state in m
                .explored
                .order
                .iter()
                .filter(|s| s.value_of("CLK") == Some(false))
            {
                reach0 = reach0.or(&super::cube_bdd(&builder, state));
            }
            let on = builder.build_cover(&q.captures[0].2.on_cover).and(&reach0);
            let want = builder.var("M").and(&reach0);
            assert!(
                on.equivalent_to(&want),
                "capture (D) equals the surviving master M over the CLK=0 capture domain"
            );
        });
    }

    #[test]
    fn edge_tapped_master_survives_unfolded() {
        with_machine!(TAPPED_MASTER_TOML, |_b, _a, _m2, m| {
            let es = classify(&m);
            assert_captures_faithful(&m, &es);
            let _q = reg(&es, "Q");
            assert!(
                !folded_list(&es).contains(&"M"),
                "a tapped master survives, folded={:?}",
                folded_list(&es)
            );
        });
    }

    // An INITIALISABLE toggle flop: the bare resetless `M="!CLK*!Q+CLK*M", Q="CLK*M+!CLK*Q"` is
    // uninitialisable (no input forces its state ⇒ the exploration reaches ZERO stable states, exactly the
    // `single_input_state_holder` precedent), so nothing is characterised. Adding an async reset resolves
    // the state; the inverting self-capture `!Q` is then exercised on the rising edge.
    const TOGGLE_FLOP_TOML: &str = r#"
[[cell]]
name = "TFF"
inputs = ["CLK", "R"]
clock = ["CLK"]
async = ["R"]
[cell.internal]
M = "!R*(!CLK*!Q + CLK*M)"
[cell.outputs]
Q = "!R*(CLK*M + !CLK*Q)"
"#;

    #[test]
    fn edge_toggle_flop_inverting_self_capture() {
        // The self-fed master M has no *data* input (R is async), so the ring is decomposed into TWO edge
        // seams rather than folding M into Q. Q captures on the rising edge and M on the falling edge, both
        // over the uniform header projected to [R, Q] — the ring closes on Q's OWN prior state: the captured
        // next state is `!R*!Q` (toggle, gated by the async reset), recorded verbatim (inversion is not
        // special-cased). The internal master M is dropped from the cover in favour of the input/output
        // columns [R, Q].
        with_machine!(TOGGLE_FLOP_TOML, |builder, _a, _m2, m| {
            let es = classify(&m);
            assert_captures_faithful(&m, &es);
            let q = reg(&es, "Q");
            let mm = reg(&es, "M");
            assert_eq!(q.captures[0].1, Edge::Rise);
            assert_eq!(mm.captures[0].1, Edge::Fall);
            let want = (!&builder.var("R")).and(&!&builder.var("Q"));
            // Q's rising capture is the toggle !R*!Q over cols [R, Q] (the self-referential ring closes on
            // Q, not the folded/dropped master).
            assert_eq!(cols_of(&q.captures[0].2), ["R", "Q"]);
            let q_on = builder.build_cover(&q.captures[0].2.on_cover);
            assert!(
                q_on.equivalent_to(&want),
                "Q captures !R*!Q (the toggle ring)"
            );
            // M's falling capture is the same toggle over the same cols: at the pre-fall (CLK=1) states
            // M equals Q, so capturing !M is capturing !Q.
            let mcap = &mm.captures[0].2;
            assert_eq!(cols_of(mcap), ["R", "Q"]);
            let m_on = builder.build_cover(&mcap.on_cover);
            assert!(
                m_on.equivalent_to(&want),
                "M captures !R*!Q (=!R*!M at the pre-fall states), inverting, no special-casing"
            );
            // The ring survives whole: M carries its own falling capture, so it is not an internal non-seam
            // node and is structurally ineligible for the fold fixpoint.
            assert!(
                folded_list(&es).is_empty(),
                "a self-referential ring whose master carries a real capture folds nothing, got {:?}",
                folded_list(&es)
            );
        });
    }

    // === Step 3 (5): cross-coupled NAND slave ===

    const XNAND_TOML: &str = r#"
[[cell]]
name = "XN"
inputs = ["CLK", "D"]
clock = ["CLK"]
[cell.internal]
M = "!CLK*D + CLK*M"
[cell.outputs]
Q = "!( !(M*CLK) * Qn )"
Qn = "!( !(!M*CLK) * Q )"
"#;

    #[test]
    fn edge_cross_coupled_nand_two_registers_shared_master() {
        with_machine!(XNAND_TOML, |builder, _a, _m2, m| {
            let es = classify(&m);
            assert_captures_faithful(&m, &es);
            let q = reg(&es, "Q");
            let qn = reg(&es, "Qn");
            assert_eq!(q.captures[0].1, Edge::Rise);
            assert_eq!(qn.captures[0].1, Edge::Rise);
            let q_on = builder.build_cover(&q.captures[0].2.on_cover);
            let qn_on = builder.build_cover(&qn.captures[0].2.on_cover);
            assert!(q_on.equivalent_to(&builder.var("D")), "Q captures D");
            assert!(qn_on.equivalent_to(&!&builder.var("D")), "Qn captures !D");
            assert_eq!(folded_list(&es), ["M"], "shared master M folded once");
        });
    }

    // === Step 3 (6): dual-edge mux-DET ===

    const DET_TOML: &str = r#"
[[cell]]
name = "DET"
inputs = ["CLK", "D"]
clock = ["CLK"]
[cell.internal]
L1 = "!CLK*D + CLK*L1"
L2 = "CLK*D + !CLK*L2"
[cell.outputs]
Q = "CLK*L1 + !CLK*L2"
"#;

    #[test]
    fn edge_dual_edge_det_captures_d_on_both_edges() {
        with_machine!(DET_TOML, |builder, _a, _m2, m| {
            let es = classify(&m);
            assert_captures_faithful(&m, &es);
            let q = reg(&es, "Q");
            assert_eq!(q.captures.len(), 2, "dual edge");
            assert_eq!(q.captures[0].1, Edge::Rise);
            assert_eq!(q.captures[1].1, Edge::Fall);
            for (_, _, cap) in &q.captures {
                let on = builder.build_cover(&cap.on_cover);
                assert!(on.equivalent_to(&builder.var("D")), "each edge captures D");
            }
            let mut folded = folded_list(&es);
            folded.sort();
            assert_eq!(folded, ["L1", "L2"]);
        });
    }

    // === Phase-symmetric data transparency must NOT read as a capture ===

    // Two opposite-phase D latches XORed: M follows D while CLK=0, M2 follows D while CLK=1, and T = M⊕M2
    // is transparent to D in BOTH phases. D is phase-SYMMETRIC (not a latch signature) and lands Held
    // off-edge (it forces T to no constant), so it is genuine data transparency — capturing T on CLK
    // would DROP D while the same run emits combinational D→T arcs under both phases. T must keep no
    // capture; D must survive as a data dependency of T's function.
    const XLAT_TOML: &str = r#"
[[cell]]
name = "XLAT"
inputs = ["CLK", "D"]
clock = ["CLK"]
[cell.internal]
M = "!CLK*D + CLK*M"
M2 = "CLK*D + !CLK*M2"
[cell.outputs]
T = "M*!M2 + !M*M2"
"#;

    #[test]
    fn edge_phase_symmetric_transparency_stays_level() {
        with_machine!(XLAT_TOML, |_b, _a, _m2, m| {
            let es = classify(&m);
            // No register for T (nor for M/M2): the whole cell is level/combinational, so there is no
            // self-contradictory register-plus-combinational-D-arc emission.
            assert!(
                !node_list(&es).contains(&"T"),
                "phase-symmetric D transparency must not read as a register: {:?}",
                node_list(&es)
            );
            assert!(es.captures.is_empty(), "no edge arcs: {:?}", node_list(&es));
            // D is preserved as a live data dependency: some reachable state where toggling D moves T. A
            // register keyed off CLK would have dropped D while the run still emits D→T data arcs — the
            // very contradiction this fix removes.
            let d_drives_t = m.explored.order.iter().any(|node| {
                let before = m.output_value("T", node);
                let Some(np) = machine::settle(&m.deltas, &machine::toggle(node, &["D"])) else {
                    return false;
                };
                matches!((before, m.output_value("T", &np)), (Some(b0), Some(b1)) if b0 != b1)
            });
            assert!(d_drives_t, "D must remain a live data dependency of T");
        });
    }

    // === Conjunctive / disjunctive / single-literal clear consistency ===

    // A gated conjunctive clear: R*G forces both latches to 0 (needs G high too). The clear is not
    // marginally forcing — the R=1,G=0 states hold — so a marginal forcing test wrongly reads it Held and
    // blocks. The EXACT off-edge synthesis lands R*G Forced0, so R and G each participate in a Forced
    // projection and Q stays a register with the conjunctive clear carried in off_edge.off.
    const GATEDR_TOML: &str = r#"
[[cell]]
name = "GATEDR"
inputs = ["CLK", "D", "R", "G"]
clock = ["CLK"]
[cell.internal]
M = "!(R*G)*(!CLK*D + CLK*M)"
[cell.outputs]
Q = "!(R*G)*(CLK*M + !CLK*Q)"
"#;

    // The single-literal sync clear: R alone forces both latches to 0.
    const SYNC_R_CLEAR_TOML: &str = r#"
[[cell]]
name = "SYNCR"
inputs = ["CLK", "D", "R"]
clock = ["CLK"]
[cell.internal]
M = "!R*(!CLK*D + CLK*M)"
[cell.outputs]
Q = "!R*(CLK*M + !CLK*Q)"
"#;

    // The disjunctive sync clear: R+G forces both latches to 0.
    const SYNC_RG_OR_CLEAR_TOML: &str = r#"
[[cell]]
name = "SYNCRG"
inputs = ["CLK", "D", "R", "G"]
clock = ["CLK"]
[cell.internal]
M = "!(R+G)*(!CLK*D + CLK*M)"
[cell.outputs]
Q = "!(R+G)*(CLK*M + !CLK*Q)"
"#;

    // The async-declared conjunctive clear: same construct as GATEDR with R and G declared async.
    const ASYNC_RG_AND_CLEAR_TOML: &str = r#"
[[cell]]
name = "AGATEDR"
inputs = ["CLK", "D", "R", "G"]
clock = ["CLK"]
async = ["R", "G"]
[cell.internal]
M = "!(R*G)*(!CLK*D + CLK*M)"
[cell.outputs]
Q = "!(R*G)*(CLK*M + !CLK*Q)"
"#;

    #[test]
    fn edge_gated_conjunctive_clear_recognised() {
        with_machine!(GATEDR_TOML, |builder, _a, _m2, m| {
            let es = classify(&m);
            assert_captures_faithful(&m, &es);
            let q = reg(&es, "Q");
            assert_eq!(clocks_of(q), ["CLK"]);
            // The conjunctive clear is carried faithfully: off_edge.off is forced-0 exactly where R*G.
            let off = builder.build_cover(&q.off_edge.off_cover);
            let rg = builder.var("R").and(&builder.var("G"));
            assert!(off.equivalent_to(&rg), "off_edge.off must cover R*G");
            // D (captured value) and the clear's R, G all survive as register columns.
            let cols = q.cols.iter().map(Symbol::as_str).collect::<Vec<_>>();
            for c in ["D", "R", "G"] {
                assert!(cols.contains(&c), "col {c} missing from {cols:?}");
            }
        });
    }

    #[test]
    fn edge_clear_variants_consistently_registers() {
        // The SAME clear construct — single-literal, disjunctive, conjunctive; sync or async-declared — is
        // recognised consistently as a capture. Grounding the permit decision in the exact off-edge
        // synthesis removes the R+G-vs-R*G and sync-vs-async inconsistency of the marginal test.
        for (src, label) in [
            (SYNC_R_CLEAR_TOML, "sync single-literal R"),
            (SYNC_RG_OR_CLEAR_TOML, "sync disjunctive R+G"),
            (GATEDR_TOML, "sync conjunctive R*G"),
            (ASYNC_RG_AND_CLEAR_TOML, "async conjunctive R*G"),
        ] {
            with_machine!(src, |_b, _a, _m2, m| {
                let es = classify(&m);
                assert_captures_faithful(&m, &es);
                assert!(
                    node_list(&es).contains(&"Q"),
                    "{label}: Q must be a register, got {:?}",
                    node_list(&es)
                );
            });
        }
    }

    // === Wave 1 batch 3: multi-clock-shaped fixtures for the per-clock viability discriminator ===

    const RDFF_TOML: &str = r#"
[[cell]]
name = "RDFF"
inputs = ["CLK", "D", "R"]
clock = ["CLK", "R"]
[cell.internal]
M = "!R*(!CLK*D + CLK*M)"
[cell.outputs]
Q = "!R*(CLK*M + !CLK*Q)"
"#;

    #[test]
    fn edge_rdff_recognised_despite_two_declared_clocks() {
        // R is declared a clock alongside CLK, but R's own directions carry no edge CONTENT (they deliver
        // a constant 0), so R is never seeded and lands as Q's clear instead ⇒ Q keeps a single CLK rising
        // arc. The master carries arcs of its own here, so it survives and Q's capture names it.
        with_machine!(RDFF_TOML, |builder, _a, _m2, m| {
            let es = classify(&m);
            assert_captures_faithful(&m, &es);
            let q = reg(&es, "Q");
            assert_eq!(clocks_of(q), ["CLK"]);
            assert_eq!(q.captures.len(), 1);
            assert_eq!(q.captures[0].1, Edge::Rise);
            let off = builder.build_cover(&q.off_edge.off_cover);
            let r = builder.var("R");
            assert!(off.equivalent_to(&r), "off_edge.off must cover R");
            // The transparent master M is a latch on CLK — no edge arc — and folds, exactly as in
            // MOR: declaring R a clock changes nothing behavioural. Q's capture generalises past
            // the folded master to !R*D.
            assert!(folded_list(&es).contains(&"M"), "the master folds");
            let on = builder.build_cover(&q.captures[0].2.on_cover);
            let want = (!&r).and(&builder.var("D"));
            assert!(on.equivalent_to(&want), "capture is !R*D");
            let cols = q.cols.iter().map(Symbol::as_str).collect::<Vec<_>>();
            for c in ["D", "R"] {
                assert!(cols.contains(&c), "col {c} missing from {cols:?}");
            }
            // R is LEVEL-ACTING on Q (R=1 alone pins it to 0): R's assert arcs carry NO edge label — no
            // in-scope latch opens on R, and no arm fires — so they emit `-type combinational`, exactly as
            // when R is not declared a clock (SYNCR). Only CLK's rise is an edge arc on Q.
            assert_eq!(
                labels_of(&es, "Q"),
                [("CLK", Edge::Rise)],
                "declaring R a clock must not label its level arcs"
            );
        });
    }

    const ICG_TOML: &str = r#"
[[cell]]
name = "ICG"
inputs = ["CLK", "EN"]
clock = ["CLK"]
[cell.internal]
EL = "!CLK*EN + CLK*EL"
[cell.outputs]
GCLK = "CLK*EL"
"#;

    #[test]
    fn edge_icg_gclk_blocked_el_survives_as_level() {
        // GCLK is combinational in EL, which is itself held (memory) across CLK's high phase ⇒ GCLK's
        // off-edge synthesis hits a phase disagreement and is blocked — NOT by a declared-clock-count gate.
        // EL is the classic transparent latch (level, phase-asymmetric to EN) and survives unfolded because
        // GCLK's raw function still references it.
        with_machine!(ICG_TOML, |_b, _a, _m2, m| {
            let es = classify(&m);
            assert!(
                !node_list(&es).contains(&"GCLK"),
                "GCLK must not be a register: {:?}",
                node_list(&es)
            );
            assert!(
                !node_list(&es).contains(&"EL"),
                "EL carries no edge annotation: {:?}",
                node_list(&es)
            );
            assert!(
                !folded_list(&es).contains(&"EL"),
                "EL survives unfolded, folded={:?}",
                folded_list(&es)
            );
        });
    }

    #[test]
    fn edge_master_only_reset_master_folds_once_recognised() {
        // Q's registration now succeeds, so its raw function is replaced by the edge seam and nothing
        // surviving references the master ⇒ M folds away.
        with_machine!(MOR_ASYNC_TOML, |_b, _a, _m2, m| {
            let es = classify(&m);
            assert_eq!(
                folded_list(&es),
                ["M"],
                "the recognised slave folds its master"
            );
        });
    }

    // === Step 3 (7): blow-up guard ===

    #[test]
    fn edge_blowup_guard_yields_default() {
        // A machine wider than MAX_MACHINE_VARS is never built ⇒ no Machine ⇒ default annotation.
        let n = crate::logic::analysis::MAX_MACHINE_VARS + 1;
        let list = (0..n)
            .map(|i| format!("\"I{i}\""))
            .collect::<Vec<_>>()
            .join(", ");
        let src =
            format!("[[cell]]\nname = \"WIDE\"\ninputs = [{list}]\n[cell.outputs]\nY = \"I0\"\n");
        let mut analysed = crate::model::parse_spec(&src)
            .unwrap()
            .cells
            .remove(0)
            .analyse_signals()
            .unwrap();
        let builder = sync_bdd_builder!();
        let mut bdds = crate::model::build_signal_bdds(&analysed, &builder);
        let order: Vec<Symbol> = analysed.signals().map(|s| s.name.clone()).collect();
        let output_set: BTreeSet<Symbol> =
            analysed.outputs.iter().map(|o| o.name.clone()).collect();
        let min = crate::logic::minimise::minimise_state_space(&mut bdds, &order, &output_set);
        crate::model::recompute_signal_metadata(&mut analysed, &bdds, &min);
        assert!(
            crate::logic::analysis::Machine::build(&analysed, &bdds).is_none(),
            "wide cell trips the guard ⇒ default EdgeArcs"
        );
        assert!(EdgeArcs::default().captures.is_empty());
    }

    // === Permanent guard: the CRITICAL INVARIANT ===

    /// Parse a single-cell spec and analyse it, forcing `no_edge_collapse` on every cell.
    fn analyse_toggled(src: &str, no_collapse: bool) -> crate::model::AnalysedCell {
        let mut spec = crate::model::parse_spec(src).unwrap();
        for c in &mut spec.cells {
            c.no_edge_collapse = no_collapse;
        }
        spec.cells[0].analyse().unwrap()
    }

    /// PERMANENT guard on the CRITICAL INVARIANT: behavioural edge classification re-expresses
    /// already-explored behaviour and must change ONLY `edge` — every other `AnalysedCell` field (the
    /// exploration, prevector/vector and hazard outputs) is byte-for-byte identical whether classification
    /// is on or off.
    ///
    /// The invariant holds BY CONSTRUCTION: `classify` takes `&Machine` read-only, mutates nothing, and
    /// mints only names that already exist in the explored machine. This test additionally proves the
    /// flag-gating is PURE — when opted out (`no_edge_collapse`) the classify() call is skipped and the
    /// annotation is the byte-identical Default, with every other field untouched.
    #[test]
    fn edge_classification_changes_only_the_edge_annotation() {
        for src in [DFF_TOML, ICM_TOML] {
            let off = analyse_toggled(src, true); // classification suppressed
            let on = analyse_toggled(src, false); // classification active

            // Every exploration-derived field is identical (Debug-string equality across all of them
            // except `edge`).
            macro_rules! unchanged {
                ($field:ident) => {
                    assert_eq!(
                        format!("{:?}", off.$field),
                        format!("{:?}", on.$field),
                        concat!(
                            "edge classification changed AnalysedCell::",
                            stringify!($field)
                        ),
                    );
                };
            }
            unchanged!(name);
            unchanged!(inputs);
            unchanged!(outputs);
            unchanged!(internals);
            unchanged!(async_pins);
            unchanged!(arcs);
            unchanged!(hidden_arcs);
            unchanged!(leakage);
            unchanged!(order_dependence);
            unchanged!(oscillation);
            unchanged!(clock_pins);
            unchanged!(constraints);
            unchanged!(constraint_arcs_declared);
            unchanged!(regions);

            // The guard has teeth: classification is a no-op when suppressed and does recognise captures
            // on these fixtures when active.
            assert!(off.edge.captures.is_empty());
            assert!(!on.edge.captures.is_empty());
        }
    }

    // === Step 8: grounded per-arc fixtures (DCMUX, COEX, transparent-cascade, clock-and-async) ===

    // DCMUX -- a genuinely INDEPENDENT two-clock capture: Q captures each independently-clocked master at
    // that clock's own edge, holding otherwise. CLKA and CLKB are unrelated inputs (no structural
    // derivation, no privileging), so Q carries edge arcs on BOTH clocks with a joint off-edge universal
    // hold. The internal masters are level (transparent to their data) and carry no arcs.
    const DCMUX_TOML: &str = r#"
[[cell]]
name = "DCMUX"
inputs = ["CLKA", "CLKB", "DA", "DB"]
clock = ["CLKA", "CLKB"]
[cell.internal]
MA = "!CLKA*DA + CLKA*MA"
MB = "!CLKB*DB + CLKB*MB"
[cell.outputs]
Q = "CLKA*MA + CLKB*MB + !CLKA*!CLKB*Q"
"#;

    #[test]
    fn edge_dcmux_carries_both_clocks_joint_hold() {
        with_machine!(DCMUX_TOML, |_b, _a, _m2, m| {
            let es = classify(&m);
            assert_captures_faithful(&m, &es);
            // Only Q carries arcs; the internal masters carry none.
            assert_eq!(node_list(&es), ["Q"], "only Q carries edge arcs");
            let q = reg(&es, "Q");
            // Both independent clocks keep edge arcs on the one output (per-arc, no privileging).
            let clks = clocks_of(q);
            assert!(
                clks.contains(&"CLKA") && clks.contains(&"CLKB"),
                "Q carries both clocks' arcs, got {clks:?}"
            );
            assert!(
                q.captures.iter().any(|(c, _, _)| c == "CLKA"),
                "a CLKA edge arc"
            );
            assert!(
                q.captures.iter().any(|(c, _, _)| c == "CLKB"),
                "a CLKB edge arc"
            );
            // Joint off-edge is a universal hold: no async set/clear, no columns.
            assert!(q.off_edge.cols.is_empty(), "no off-edge columns");
            assert!(q.off_edge.on.is_empty() && q.off_edge.off.is_empty());
            assert_eq!(q.off_edge.hold, vec![vec![]], "universal hold");
        });
    }

    // COEX -- a CLK-rise capture coexisting with a non-async combinational set B (forces Q high in either
    // clock phase, surviving as a Forced1 off-edge column) AND an async clear R (forces Q low). Edge,
    // combinational and async arcs all coexist on the one output.
    const COEX_TOML: &str = r#"
[[cell]]
name = "COEX"
inputs = ["CLK", "D", "B", "R"]
clock = ["CLK"]
async = ["R"]
[cell.internal]
M = "!R*(B + !CLK*D + CLK*M)"
[cell.outputs]
Q = "!R*(B + CLK*M + !CLK*Q)"
"#;

    #[test]
    fn edge_coex_edge_and_combinational_on_one_output() {
        with_machine!(COEX_TOML, |builder, _a, _m2, m| {
            let es = classify(&m);
            assert_captures_faithful(&m, &es);
            let q = reg(&es, "Q");
            // The edge arc: CLK rising captures.
            assert_eq!(clocks_of(q), ["CLK"]);
            assert_eq!(q.captures.len(), 1);
            assert_eq!(q.captures[0].1, Edge::Rise);
            // The combinational set B is a Forced1 off-edge column (only while not cleared); the async
            // clear R is a Forced0 column. Edge and combinational arcs coexist on the one output.
            let on = builder.build_cover(&q.off_edge.on_cover);
            let off = builder.build_cover(&q.off_edge.off_cover);
            let b = builder.var("B");
            let r = builder.var("R");
            assert!(
                on.equivalent_to(&b.and(&!&r)),
                "off_edge.on is the combinational set B (clear dominating)"
            );
            assert!(off.equivalent_to(&r), "off_edge.off is the async clear R");
            assert!(
                cols_of(&q.off_edge).contains(&"B"),
                "B survives as an off-edge column"
            );
        });
    }

    // Transparent cascade (zero-arc): a level latch feeding a same-phase level latch is transparent
    // overall -- the whole chain follows D through CLK's low phase, so no node keeps a capture and none
    // carries an edge arc (it falls out of the quiet-phase rule, not from any dismissal). The XLAT
    // analogue.
    const TCASC_TOML: &str = r#"
[[cell]]
name = "TCASC"
inputs = ["CLK", "D"]
clock = ["CLK"]
[cell.internal]
M = "!CLK*D + CLK*M"
[cell.outputs]
Q = "!CLK*M + CLK*Q"
"#;

    #[test]
    fn edge_transparent_cascade_zero_arcs() {
        with_machine!(TCASC_TOML, |_b, _a, _m2, m| {
            let es = classify(&m);
            assert_captures_faithful(&m, &es); // vacuous, but keeps the discipline uniform
            assert!(
                es.captures.is_empty(),
                "a transparent cascade carries zero edge arcs, got {:?}",
                node_list(&es)
            );
        });
    }

    // Clock-and-async: CLK's rising edge captures D while an async preset (PRE, force 1) and async clear
    // (CLR, force 0) coexist on the same output. The edge arc and both async set/clear off-edge classes
    // are carried together.
    const CAFF_TOML: &str = r#"
[[cell]]
name = "CAFF"
inputs = ["CLK", "D", "PRE", "CLR"]
clock = ["CLK"]
async = ["PRE", "CLR"]
[cell.internal]
M = "!CLR*(PRE + !CLK*D + CLK*M)"
[cell.outputs]
Q = "!CLR*(PRE + CLK*M + !CLK*Q)"
"#;

    #[test]
    fn edge_clock_and_async_set_clear_coexist() {
        with_machine!(CAFF_TOML, |builder, _a, _m2, m| {
            let es = classify(&m);
            assert_captures_faithful(&m, &es);
            let q = reg(&es, "Q");
            assert_eq!(clocks_of(q), ["CLK"]);
            assert_eq!(q.captures[0].1, Edge::Rise);
            // The async set/clear off-edge covers: PRE forces 1 (only while not cleared), CLR forces 0.
            let on = builder.build_cover(&q.off_edge.on_cover);
            let off = builder.build_cover(&q.off_edge.off_cover);
            let pre = builder.var("PRE");
            let clr = builder.var("CLR");
            assert!(
                on.equivalent_to(&pre.and(&!&clr)),
                "off_edge.on is the async preset PRE (clear dominating)"
            );
            assert!(
                off.equivalent_to(&clr),
                "off_edge.off is the async clear CLR"
            );
        });
    }

    // === Step 9: hierarchical master-slave-across-two-clocks (correction regression guard) ===

    // HPIPE -- a CLKA rising-edge master pair (M1/M2 capture D on CLKA) feeding a CLKB slave latch on Q (a
    // derived/gated-clock chain). The pair jointly disagrees at the naive joint-off-edge level the
    // pre-amendment rule would have checked, yet EVERY hierarchically-related clock's edge arcs must
    // SURVIVE: the slave Q keeps both CLKA and CLKB, the master node M2 keeps CLKA, and no set is emptied.
    const HPIPE_TOML: &str = r#"
[[cell]]
name = "HPIPE"
inputs = ["CLKA", "CLKB", "D"]
clock = ["CLKA", "CLKB"]
[cell.internal]
M1 = "!CLKA*D + CLKA*M1"
M2 = "CLKA*M1 + !CLKA*M2"
[cell.outputs]
Q = "!CLKB*M2 + CLKB*Q"
"#;

    #[test]
    fn edge_hierarchical_two_clocks_exact_arc_set() {
        // Q's seam set is EXACTLY {CLKA:Rise, CLKB:Fall}:
        //
        // * CLKA:Rise is the conditioned capture — Q takes the master pair's content when CLKB is
        //   transparent (CLKB=0) and re-delivers its own held value when CLKB is opaque (CLKB=1);
        // * CLKB:Fall is Q's OWN latch opening — Q holds M2 in CLKB=1 and reveals it on the fall. This
        //   closes the known imprecision the old model carved out of the replay harness: the CLKB reveal
        //   is now a first-class seam with its own capture (M2).
        with_machine!(HPIPE_TOML, |builder, _a, _m2, m| {
            let es = classify(&m);
            assert_captures_faithful(&m, &es);
            let q = reg(&es, "Q");
            let arcs: Vec<(&str, Edge)> = q
                .captures
                .iter()
                .map(|(c, e, _)| (c.as_str(), *e))
                .collect();
            assert_eq!(
                arcs,
                [("CLKA", Edge::Rise), ("CLKB", Edge::Fall)],
                "Q carries the conditioned CLKA capture and its own CLKB-fall opening"
            );
            // The CLKA capture characterises the condition: Q captures D when CLKB is transparent
            // (CLKB=0) and re-delivers its own held value when CLKB is opaque (CLKB=1).
            let (_, _, cap_a) = q
                .captures
                .iter()
                .find(|(c, e, _)| c == "CLKA" && *e == Edge::Rise)
                .unwrap();
            let on = builder.build_cover(&cap_a.on_cover);
            let clkb = builder.var("CLKB");
            let want = clkb
                .and(&builder.var("Q"))
                .or(&(!&clkb).and(&builder.var("D")));
            assert!(
                on.equivalent_to(&want),
                "CLKA capture is CLKB*Q + !CLKB*D (conditioned on CLKB transparent)"
            );
            // The CLKB-fall opening reveals the surviving master node M2.
            let (_, _, cap_b) = q
                .captures
                .iter()
                .find(|(c, e, _)| c == "CLKB" && *e == Edge::Fall)
                .unwrap();
            assert_eq!(cols_of(cap_b), ["M2"], "CLKB fall reveals M2");
            let on_b = builder.build_cover(&cap_b.on_cover);
            assert!(
                on_b.equivalent_to(&builder.var("M2")),
                "CLKB fall captures M2"
            );
            // The surviving master node keeps CLKA; the inner master folds.
            assert!(
                clocks_of(reg(&es, "M2")).contains(&"CLKA"),
                "master node keeps CLKA"
            );
            assert_eq!(folded_list(&es), ["M1"], "only the inner master folds");
        });
    }

    // MCDFF -- a master/slave pair split across two DIFFERENT declared clocks: M latches on CLKA, Q on
    // CLKB. Two latches on unrelated clocks can never form a flop, so the classifier recognises NO
    // register on either node -- both stay fully level.
    const MCDFF_TOML: &str = r#"
[[cell]]
name = "MCDFF"
inputs = ["CLKA", "CLKB", "D"]
clock = ["CLKA", "CLKB"]
[cell.internal]
M = "!CLKA*D + CLKA*M"
[cell.outputs]
Q = "CLKB*M + !CLKB*Q"
"#;

    #[test]
    fn edge_mcdff_two_clock_pair_zero_captures() {
        with_machine!(MCDFF_TOML, |_b, _a, _m2, m| {
            let es = classify(&m);
            assert_captures_faithful(&m, &es);
            assert!(
                es.captures.is_empty(),
                "a two-clock master/slave pair carries zero edge arcs on either node, got {:?}",
                node_list(&es)
            );
        });
    }

    // === IMPLEMENTATION-STYLE INVARIANCE: the same logical cells built from cross-coupled NANDs ===
    //
    // The classifier is behavioural — it reads `machine::toggle`/`settle`, never an equation's shape — so
    // re-expressing a cell in a different implementation style must characterise IDENTICALLY. The trio
    // below rebuilds `DLAT`, `DFF` and `HPIPE` out of the cross-coupled-NAND latch idiom
    // (`Qn = !(!(!D*CLK) * Q)`, `Q = !(!(D*CLK) * Qn)`) and pins the same arcs, clocks and covers.
    //
    // The NAND idiom carries its complement node explicitly, so each latch contributes a second state
    // variable that the pass-transistor style does not have. That complement is NOT a carrier (it never
    // moves without the node it complements), which is exactly what keeps the arc sets invariant.

    const NDLAT_TOML: &str = r#"
[[cell]]
name = "NDLAT"
inputs = ["CLK", "D"]
clock = ["CLK"]
[cell.outputs]
Qn = "!( !(!D*CLK) * Q )"
Q = "!( !(D*CLK) * Qn )"
"#;

    const NDFF_TOML: &str = r#"
[[cell]]
name = "NDFF"
inputs = ["CLK", "D"]
clock = ["CLK"]
[cell.internal]
Mn = "!( !(!D*!CLK) * M )"
M = "!( !(D*!CLK) * Mn )"
[cell.outputs]
Qn = "!( !(!M*CLK) * Q )"
Q = "!( !(M*CLK) * Qn )"
"#;

    const NHPIPE_TOML: &str = r#"
[[cell]]
name = "NHPIPE"
inputs = ["CLKA", "CLKB", "D"]
clock = ["CLKA", "CLKB"]
[cell.internal]
M1n = "!( !(!D*!CLKA) * M1 )"
M1 = "!( !(D*!CLKA) * M1n )"
M2n = "!( !(!M1*CLKA) * M2 )"
M2 = "!( !(M1*CLKA) * M2n )"
[cell.outputs]
Qn = "!( !(!M2*!CLKB) * Q )"
Q = "!( !(M2*!CLKB) * Qn )"
"#;

    /// The union of the machine's FULLY SETTLED reachable states, for restricting a cover equivalence to
    /// the states the cell can actually rest in (the `edge_exposed_master_*` idiom). A state with an
    /// unsettled coordinate leaves that column free in its cube and would readmit combinations the cell
    /// never rests in — such as a complement pair holding `Q == Qn`.
    fn reachable<B: Brand, C: ManagerCell + Send + Sync>(
        builder: &BddBuilder<B, C>,
        m: &Machine<B, C>,
    ) -> Bdd<B, C> {
        let mut reach = builder.constant(false);
        for state in Transitions::build(m)
            .order
            .iter()
            .filter(|s| s.vars().iter().all(|v| s.value_of(v.as_str()).is_some()))
        {
            reach = reach.or(&super::cube_bdd(builder, state));
        }
        reach
    }

    #[test]
    fn edge_nand_latch_is_a_latch_like_the_pass_gate_one() {
        // (1) A cross-coupled-NAND D latch characterises exactly as `DLAT`: no edge arc on either node.
        with_machine!(NDLAT_TOML, |_b, _a, _m2, m| {
            let es = classify(&m);
            assert_captures_faithful(&m, &es);
            assert!(
                es.captures.is_empty(),
                "a NAND latch is a latch, arcs on {:?}",
                node_list(&es)
            );
        });
    }

    #[test]
    fn edge_nand_master_slave_matches_the_pass_gate_flop() {
        // (2) A NAND master-slave flop characterises exactly as `DFF`: Q captures D on the rising edge,
        // with the same cover, the same total capture and the same universal off-edge hold. The
        // complement node Qn carries the same edge capturing `!D` — inversion is not special, it is just
        // another captured function (the `XNAND` precedent).
        let (dff, dff_folded) = with_machine!(DFF_TOML, |_b, _a, _m2, m| {
            let es = classify(&m);
            (reg(&es, "Q").clone(), es.folded.clone())
        });
        assert_eq!(
            dff_folded.iter().map(Symbol::as_str).collect::<Vec<_>>(),
            ["M"],
            "the pass DFF folds its master M"
        );
        with_machine!(NDFF_TOML, |builder, _a, _m2, m| {
            let es = classify(&m);
            assert_captures_faithful(&m, &es);
            let q = reg(&es, "Q");
            assert_eq!(q.captures.len(), 1);
            let (clk, edge, cap) = &q.captures[0];
            assert_eq!((clk.as_str(), *edge), ("CLK", Edge::Rise));
            assert_eq!(cols_of(cap), cols_of(&dff.captures[0].2));
            assert_eq!(cap.on, dff.captures[0].2.on, "same capture as the pass DFF");
            assert_eq!(cap.off, dff.captures[0].2.off);
            assert_eq!(cap.hold, dff.captures[0].2.hold);
            assert_eq!(q.off_edge.hold, dff.off_edge.hold, "universal hold");
            assert!(q.off_edge.on.is_empty() && q.off_edge.off.is_empty());
            let on = builder.build_cover(&cap.on_cover);
            assert!(on.equivalent_to(&builder.var("D")), "Q captures D exactly");
            // The complement carries the same edge capturing !D.
            let qn = reg(&es, "Qn");
            assert_eq!(
                (qn.captures[0].0.as_str(), qn.captures[0].1),
                ("CLK", Edge::Rise)
            );
            let qn_on = builder.build_cover(&qn.captures[0].2.on_cover);
            assert!(qn_on.equivalent_to(&!&builder.var("D")), "Qn captures !D");

            // The pass DFF folds its lone master M; the NAND master pair M/Mn is captureless and
            // MUTUALLY REFERENCING, reaching no output once collapsed, so the reachability fixpoint
            // folds the pair together — the two forms converge on folding their master(s), the NAND
            // idiom simply carrying the complement as a second candidate.
            let mut ndff_folded = folded_list(&es);
            ndff_folded.sort();
            assert_eq!(
                ndff_folded,
                ["M", "Mn"],
                "the mutually-referencing NAND master pair folds together, exactly as the pass DFF folds its lone M"
            );
            // Qn is a genuine second declared output of the NAND topology, carrying its own !D capture
            // (asserted above) — not an unfolded internal.
            assert_eq!(
                node_list(&es),
                ["Q", "Qn"],
                "Q and Qn are the only surviving declared outputs"
            );
        });
    }

    #[test]
    fn edge_nand_hierarchical_two_clocks_matches_the_pass_gate_pipe() {
        // (3) A NAND flop on CLKA feeding a NAND latch on CLKB characterises exactly as `HPIPE`: Q takes
        // the conditioned CLKA rising capture AND its own CLKB-fall opening, and the surviving master node
        // keeps CLKA.
        with_machine!(NHPIPE_TOML, |builder, _a, _m2, m| {
            let es = classify(&m);
            assert_captures_faithful(&m, &es);
            let q = reg(&es, "Q");
            let arcs: Vec<(&str, Edge)> = q
                .captures
                .iter()
                .map(|(c, e, _)| (c.as_str(), *e))
                .collect();
            assert_eq!(
                arcs,
                [("CLKA", Edge::Rise), ("CLKB", Edge::Fall)],
                "Q matches the pass-gate HPIPE: conditioned CLKA capture and its own CLKB-fall opening"
            );

            let reach = reachable(&builder, &m);
            // The complement node is the node's negation on every reachable state — which is why it is
            // not an independent carrier and does not disqualify the CLKA edge.
            let q_var = builder.var("Q");
            let qn_var = builder.var("Qn");
            assert!(
                q_var.and(&reach).equivalent_to(&(!&qn_var).and(&reach)),
                "Qn == !Q on every reachable state"
            );
            // Same conditioned capture as the pass-gate HPIPE, written over the complement the NAND style
            // exposes: D while CLKB is transparent, the held value re-delivered while CLKB is opaque.
            let (_, _, cap_a) = q
                .captures
                .iter()
                .find(|(c, e, _)| c == "CLKA" && *e == Edge::Rise)
                .unwrap();
            let on = builder.build_cover(&cap_a.on_cover).and(&reach);
            let clkb = builder.var("CLKB");
            let want = clkb
                .and(&!&qn_var)
                .or(&(!&clkb).and(&builder.var("D")))
                .and(&reach);
            assert!(
                on.equivalent_to(&want),
                "capture is CLKB*!Qn + !CLKB*D (conditioned on CLKB transparent)"
            );
            // The surviving master node keeps CLKA.
            assert!(
                clocks_of(reg(&es, "M2")).contains(&"CLKA"),
                "master node keeps CLKA"
            );
            assert_eq!(
                reg(&es, "M2").captures[0].1,
                Edge::Rise,
                "master captures on the CLKA rising edge"
            );
            // The inner NAND master pair M1/M1n is captureless and mutually referencing, mirroring
            // NDFF's M/Mn — it folds together, just as the pass-gate HPIPE folds its lone M1.
            let mut folded = folded_list(&es);
            folded.sort();
            assert_eq!(
                folded,
                ["M1", "M1n"],
                "the inner NAND master pair folds together, mirroring the pass-gate HPIPE folding its lone M1"
            );
            // M2/M2n carry the CLKA captures, so they were never fold candidates under either rule.
            assert!(
                !folded.iter().any(|n| *n == "M2" || *n == "M2n"),
                "M2/M2n survive, carrying the CLKA captures"
            );
        });
    }

    // === THE OPENING (RELEASE) PARTITION ===
    //
    // An opening is the clock edge that takes a node from OPAQUE to TRANSPARENT: data that changed while
    // the node was closed is delivered BY THAT EDGE, and the delivered value then TRACKS rather than
    // holds. It is the third leg of the per-arc classification, disjoint from the captures and equally an
    // edge arc — a latch has no capture but it does have an opening.
    //
    // The lists below are GROUNDED: each was read off the machine before being pinned, never predicted
    // from the equations' shape.

    #[test]
    fn edge_labels_single_clock_sourced_from_arcs() {
        // Labels are SOURCED FROM the delay arcs, so only observable transitions carry one. DFF's
        // masked CLK fall never changes Q — no arc, no key — and the internal master M carries no delay
        // arc at all: the label set is exactly the one rising edge arc.
        with_machine!(DFF_TOML, |_b, _a, _m2, m| {
            let es = classify(&m);
            assert_eq!(label_list(&es), [("Q", "CLK", Edge::Rise)]);
        });

        // A plain latch has no capture anywhere, but its enable's OPENING edge (opaque→transparent) is a
        // real edge arc — arm (a): Q is the opener — so the CLK-rise arc is labelled edge.
        with_machine!(DLAT_TOML, |_b, _a, _m2, m| {
            let es = classify(&m);
            assert!(es.captures.is_empty(), "a latch has no capture");
            assert_eq!(label_list(&es), [("Q", "CLK", Edge::Rise)]);
        });

        // Two latches on the SAME phase never form a flop, and the cascade holds NO independently
        // distinguishable state — `Q == M` in every reachable state, so neither node has a memory witness
        // (condition 1' finds no opener). The transparent cascade is a ZERO-ARC cell: no capture AND no
        // edge label, exactly as its own fixture describes it.
        with_machine!(TCASC_TOML, |_b, _a, _m2, m| {
            let es = classify(&m);
            assert!(
                es.captures.is_empty(),
                "a transparent cascade has no capture"
            );
            assert!(
                es.labels.is_empty(),
                "a transparent cascade carries no edge label: {:?}",
                label_list(&es)
            );
        });

        // A dual-edge flop is edge on BOTH directions; the masters' own openings are internal and carry
        // no delay arc, so no further key exists.
        with_machine!(DET_TOML, |_b, _a, _m2, m| {
            let es = classify(&m);
            assert_eq!(
                label_list(&es),
                [("Q", "CLK", Edge::Rise), ("Q", "CLK", Edge::Fall)]
            );
        });
    }

    #[test]
    fn edge_labels_two_clock_sourced_from_arcs() {
        // Two independent flops merging into one output. Each clock's RISING edge captures its own master
        // (edge), but the falling edges are switch-away / CROSS-CLOCK exposures — the settled value arrives
        // regardless of the exposed master's frozen content (the mux delivers the OTHER clock's master, or
        // the held value), so no arm fires and they emit `-type combinational`. The internal masters carry
        // no delay arc and take no key.
        with_machine!(DCMUX_TOML, |_b, _a, _m2, m| {
            let es = classify(&m);
            assert_eq!(
                label_list(&es),
                [("Q", "CLKA", Edge::Rise), ("Q", "CLKB", Edge::Rise)]
            );
        });

        // HPIPE: a CLKA flop feeding a CLKB latch. Q carries BOTH the CLKA rising arc (a capture) and the
        // CLKB falling arc (its own latch opening) — both edge, each labelling its own arcs.
        with_machine!(HPIPE_TOML, |_b, _a, _m2, m| {
            let es = classify(&m);
            assert_eq!(
                label_list(&es),
                [("Q", "CLKA", Edge::Rise), ("Q", "CLKB", Edge::Fall)]
            );
            assert_eq!(
                reg(&es, "Q")
                    .captures
                    .iter()
                    .map(|(c, e, _)| (c.as_str(), *e))
                    .collect::<Vec<_>>(),
                [("CLKA", Edge::Rise), ("CLKB", Edge::Fall)],
                "Q's seams: the CLKA capture and its own CLKB-fall opening"
            );
        });

        // MCDFF: two latches on UNRELATED clocks. It stays captureless (its zero-capture fixture is a
        // separate assertion and must remain true), yet it is not timing-invisible — Q's CLKB rise opens
        // its own latch and Q's CLKA fall reaches Q only through the open CLKB latch. Conditioning never
        // reclassifies an arc.
        with_machine!(MCDFF_TOML, |_b, _a, _m2, m| {
            let es = classify(&m);
            assert!(
                es.captures.is_empty(),
                "captures and labels are separate: MCDFF still captures nothing, got {:?}",
                node_list(&es)
            );
            assert_eq!(
                label_list(&es),
                [("Q", "CLKA", Edge::Fall), ("Q", "CLKB", Edge::Rise)]
            );
        });
    }

    #[test]
    fn edge_clock_gate_arcs_carry_no_label() {
        // A gated clock is combinational in a held enable: GCLK goes low on the fall REGARDLESS of the
        // held enable, so no arm fires and its arcs carry no edge label — `-type combinational`. The
        // enable latch behind it is internal (no delay arc, no key), so the whole label set is empty.
        with_machine!(ICG_TOML, |_b, _a, _m2, m| {
            let es = classify(&m);
            assert!(
                es.labels.is_empty(),
                "a clock gate's arcs carry no edge label: {:?}",
                label_list(&es)
            );
        });

        // ICM: GCLK is a two-clock combinational gate, and the competing-branch propagation into the OTHER
        // synchroniser's first master is masked by the master-slave chain before it can reach GCLK —
        // no arc exists, so no label can either. Internal latches (sela1/selb1) are clocked only by
        // their own branch clock and, carrying no delay arc, take no label at all.
        with_machine!(ICM_TOML, |_b, _a, _m2, m| {
            let es = classify(&m);
            assert!(
                es.labels.is_empty(),
                "ICM's clock-gate arcs carry no edge label: {:?}",
                label_list(&es)
            );
        });
    }

    #[test]
    fn edge_nand_labels_mirror_the_pass_gate_twins() {
        // IMPLEMENTATION-STYLE INVARIANCE for the labels too: the NAND-built trio labels exactly the
        // same (output, clock, direction) arcs as its pass-transistor twin. The NAND idiom carries each
        // output's complement explicitly, so the complement output lists the SAME keys — never a
        // different edge.
        for (toml, want) in [
            (
                NDLAT_TOML,
                vec![("Q", "CLK", Edge::Rise), ("Qn", "CLK", Edge::Rise)],
            ),
            (
                NDFF_TOML,
                vec![("Q", "CLK", Edge::Rise), ("Qn", "CLK", Edge::Rise)],
            ),
            (
                NHPIPE_TOML,
                vec![
                    ("Q", "CLKA", Edge::Rise),
                    ("Q", "CLKB", Edge::Fall),
                    ("Qn", "CLKA", Edge::Rise),
                    ("Qn", "CLKB", Edge::Fall),
                ],
            ),
        ] {
            with_machine!(toml, |_b, _a, _m2, m| {
                let es = classify(&m);
                assert_eq!(label_list(&es), want);
            });
        }
    }

    #[test]
    fn folded_nodes_are_referenced_by_nothing_that_survives() {
        // The emission invariant the group fold widens, over the sequential fixtures that exercise every
        // fold shape: a lone master (DFF, HPIPE), a mutually-referencing capture-less pair (NDFF,
        // NHPIPE), several independent masters (DET, ICM), a ring that folds nothing (the toggle flop)
        // and the two masters kept live by an outside reference (tapped, exposed).
        for src in [
            DFF_TOML,
            NDFF_TOML,
            HPIPE_TOML,
            NHPIPE_TOML,
            DET_TOML,
            ICM_TOML,
            TOGGLE_FLOP_TOML,
            TAPPED_MASTER_TOML,
            EXPOSED_MASTER_TOML,
        ] {
            with_machine!(src, |_b, _a, _m2, m| {
                let es = classify(&m);
                assert_no_dropped_references(&m, &es);
            });
        }
    }
}
