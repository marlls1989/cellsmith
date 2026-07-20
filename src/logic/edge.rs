//! Behavioural per-arc edge classification.
//!
//! Every arc a candidate node — every output **and** every internal state variable — can present is
//! classified, purely from the cell's already-explored toggle-and-settle behaviour, into one of three
//! categories. Classification is PER ARC: there is no register verdict on a node, and arcs of all three
//! categories coexist freely on one output (an async-reset flop carries all three).
//!
//! * **CAPTURE** (edge) — a clock edge makes the node capture-and-hold a value that then holds
//!   independently of the clock's LEVEL until that clock's next edge. [`EdgeArcs::captures`].
//! * **RELEASE / OPENING** (edge) — a clock edge takes a latch from OPAQUE to TRANSPARENT, so data that
//!   changed while it was closed is transmitted to the node BY THE EDGE; the delivered value then
//!   TRACKS rather than holds. [`ArcClass::Release`] in [`EdgeArcs::labels`].
//! * **COMBINATIONAL** — a data change propagating while the latch is already transparent. Not recorded
//!   here; it falls out of [`super::arcs`] as an ordinary data arc.
//!
//! A latch therefore has no capture but does have an edge arc, its opening. Captures and releases are
//! distinct categories internally — the delivered value holds versus tracks — but both are timing arcs
//! on a clock edge and both emit Liberate `-type edge` (see [`crate::emit::arcs_tcl`]). A CONDITIONED
//! release (a clock edge reaching an output only through a second, currently-open latch) is the same
//! category with its condition in the arc's `-when`: conditioning never reclassifies an arc.
//!
//! Everything here is derived BEHAVIOURALLY from observed toggle-and-settle transitions, never from the
//! shape of an equation, and nothing branches on a declared input class — an async pin need not be
//! declared, its effect being classified from its own observed moves (`forcing_pins`). The
//! characterisation is consequently implementation-style invariant: the NAND-implemented `NDLAT` /
//! `NDFF` / `NHPIPE` fixtures characterise identically to their pass-transistor twins.
//!
//! [`classify`] is a **post-exploration** read-only pass over the shared [`Machine`]: it re-walks the
//! exploration with [`machine::toggle`]/[`machine::settle`], mirroring [`super::arcs::derive`]'s
//! per-node walk, and only ADDS an edge annotation. It never re-derives the exploration, the
//! prevectors or the hazards — those stay byte-identical whether the annotation is on or off.
//!
//! # The pipeline
//!
//! Per candidate, from the aggregated walk observations (`capture_arcs`):
//!
//! 1. **SEED by CONTENT** over all firings of a `(clock, direction)`, changed or not: the edge carries
//!    state content or pin content, judged with the non-clock inputs that move the node (coexisting
//!    combinational arcs) eliminated.
//! 2. **LEVEL-INDEPENDENCE VETO** (`pinned_by_clock_levels`): a clock whose LEVELS alone pin the node
//!    to a constant decides it by level, so it is the combinational clock-gate class (ICG/ICM `GCLK`) —
//!    neither a capture nor a release.
//! 3. **CAPTURE RULE**, each arc decided independently: keep `(clock, direction)` iff it CHANGED the
//!    node and the delivered phase is QUIET (`phase_quiet`) — no live data reaches the node inside the
//!    phase, with the node's forcing pins exempted and co-resident clock movers admitted unless they
//!    change a phase-wide carrier the node tracks.
//! 4. **PER-ARC LABELS, SOURCED FROM THE ARC PIPELINE**: every timing arc is one of the delay arcs
//!    [`super::arcs::derive`] observed — nothing else exists to label, so a masked edge (a `DFF`'s
//!    fall, `ICM`'s competing branch) never appears at all. Each clock-related arc key
//!    `(output, clock, direction)` gets [`ArcClass::Capture`] when the direction was kept by the
//!    capture rule, no label when the clock was vetoed (a level-acting clock gate stays
//!    combinational), and [`ArcClass::Release`] otherwise — an edge that moved the node without
//!    holding afterwards released a latch.
//!
//! The capture (per active edge) and the off-edge (hold + set/clear forcing) functions are synthesised
//! from the sampled pre-states and stable states over a deterministic two-tier header, reusing the
//! [`super::regions`] region pipeline, and recorded verbatim as ordinary functions — an inverting flop's
//! capture is simply `!D`, never special-cased.
//!
//! See `docs/edge-collapse.md` for the concept-first walkthrough: the three categories, the decision
//! pipeline, the capture and off-edge synthesis, the cell-level fold (and its group-fold follow-up), and
//! the retained restrictions.

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
/// state variables, plus the cell-level set of internal capture-less master nodes folded away (a
/// cross-coupled pair shares one folded master).
///
/// Every arc on an output falls in exactly one of THREE categories:
///
/// 1. **CAPTURE** (edge): a clock edge makes the node capture-and-hold a value that then holds
///    independently of the clock level until that clock's next edge — [`EdgeArcs::captures`].
/// 2. **RELEASE / OPENING** (edge): a clock edge takes a latch from OPAQUE to TRANSPARENT, so data that
///    changed while the latch was closed is transmitted to the node BY THE CLOCK EDGE; the delivered
///    value then TRACKS rather than holds — [`ArcClass::Release`] in [`EdgeArcs::labels`].
/// 3. **COMBINATIONAL**: a data change propagating while the latch is already transparent — not recorded
///    here; it falls out of [`super::arcs`] as an ordinary data arc.
///
/// A latch has no capture but it does have an opening, so both categories are real timing arcs and both
/// emit as Liberate `-type edge`; they differ only in what the delivered value does afterwards.
#[derive(Debug, Default)]
pub struct EdgeArcs {
    pub captures: Vec<EdgeCaptures>,
    pub folded: Vec<Symbol>,
    /// The per-arc `-type` labels of the cell's CLOCK-RELATED delay arcs, keyed by the arc's identity
    /// in the arc pipeline: `(output, related clock, the clock's own direction)`. SOURCED FROM the
    /// derived arcs — a key exists only where [`super::arcs::derive`] observed the transition, so a
    /// masked edge (a `DFF`'s fall, `ICM`'s competing branch reaching an internal latch) has no entry
    /// by construction, and internal nodes — which carry no delay arc — are never labelled. An ABSENT
    /// key on a clock-related arc means the clock acts there by LEVEL (the vetoed clock-gate class)
    /// and the arc stays `-type combinational`.
    pub labels: BTreeMap<(Symbol, Symbol, Edge), ArcClass>,
}

/// The edge category of one labelled delay arc. The two categories are distinct — the delivered value
/// HOLDS after a capture and TRACKS after a release — but both are timing arcs on a clock edge and
/// both emit Liberate `-type edge`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArcClass {
    Capture,
    Release,
}

/// A single edge's capture observations for one candidate: whether any sample changed the value, the
/// `(pre-state, post-value)` samples (unchanged clock-toggle samples included) and the full per-firing
/// census.
#[derive(Default, Clone)]
struct CapAgg {
    changed: bool,
    samples: Vec<(Minterm<Symbol>, bool)>,
    /// One entry per settling firing of this edge — CHANGED OR NOT: `(pre-state, destination stable
    /// state, post value)`. The census the decision core replays for the edge's content and for the
    /// held-acquisition hold walk over the post-edge phase.
    firings: Vec<(Minterm<Symbol>, Minterm<Symbol>, bool)>,
}

/// The aggregated observations of one candidate node across the whole exploration walk.
#[derive(Default, Clone)]
struct CandAgg {
    /// One entry per single-input toggle that CHANGED the node: `(toggled input, SOURCE stable state,
    /// destination stable state, post value)`. Every moving toggle is recorded uniformly — clock, data
    /// and async alike — and the capture-and-hold fixpoint reads them back to decide which clocks keep
    /// edge arcs. The source state is kept so a move can be replayed from where it started (the hold
    /// walk needs the pre-toggle state, not just where it landed).
    moves: Vec<(Symbol, Minterm<Symbol>, Minterm<Symbol>, bool)>,
    /// The distinct clocks whose toggle changed the node.
    changed_clocks: BTreeSet<Symbol>,
    /// Per `(clock, is_rise)`: the capture observations.
    captures: BTreeMap<(Symbol, bool), CapAgg>,
    /// The `(stable state, value)` samples, for the off-edge synthesis.
    stable: Vec<(Minterm<Symbol>, bool)>,
}

impl CandAgg {
    /// Fold another node's contribution for the same candidate into this one.
    fn merge(&mut self, other: CandAgg) {
        self.moves.extend(other.moves);
        self.changed_clocks.extend(other.changed_clocks);
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
/// input-pin order with Rise first), its off-edge, and whether tier-2 header escalation was needed
/// (tier-2 nodes survive the fold).
type Synthesised = (Vec<(Symbol, Edge, StateRegions)>, StateRegions, bool);

/// One candidate edge arc on a node: `(clock, is_rise)`. The decision core's whole currency — arcs, never
/// a per-node register verdict, so edge and combinational arcs coexist freely on one output.
type Arc = (Symbol, bool);

/// Discover each node's edge arcs from the cell's toggle-and-settle behaviour and label the cell's
/// delay arcs per arc. Read-only over the shared [`Machine`]: it re-walks the exploration and only ADDS
/// an annotation, mirroring [`super::arcs::derive`] — whose derived arcs are also the SOURCE of the
/// label domain: only an observed arc is ever labelled.
pub fn classify<B: Brand, C: ManagerCell + Send + Sync>(
    m: &Machine<B, C>,
    delay_arcs: &[DelayArc],
) -> EdgeArcs {
    // No state variables ⇒ nothing can carry an edge arc (and no builder to mint region covers from).
    let Some((_, any_delta)) = m.deltas.first() else {
        return EdgeArcs::default();
    };
    let builder = any_delta.builder();

    let cell = m.cell;
    let inputs = &cell.inputs;
    let deltas = &m.deltas;
    let ex = &m.explored;

    // The declared clocks. Every declared clock is a candidate edge key; whether a clock keeps edge arcs
    // on a given node is decided by the capture-and-hold fixpoint, not by input-class routing.
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

    let value = |name: &Symbol, node: &Minterm<Symbol>| m.output_value(name.as_str(), node);

    // The observation walk: own rayon par_iter over the reachable stable states, mirroring
    // `arcs::derive`'s per-node walk. Each node toggles one input at a time, settles, and records the
    // candidate values before/after. The walk produces plain data (minterms); no BDD is built here.
    let per_node = |node: &Minterm<Symbol>| -> Vec<CandAgg> {
        let mut out: Vec<CandAgg> = vec![CandAgg::default(); candidates.len()];
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
                    // A clock toggle: record every sample for the capture synthesis, changed or not,
                    // and the firing itself — pre, destination and post — for the decision core.
                    let cap = out[i].captures.entry((related.clone(), rose)).or_default();
                    cap.samples.push((node.clone(), b1));
                    cap.firings.push((node.clone(), np.clone(), b1));
                    if b0 != b1 {
                        cap.changed = true;
                        out[i].changed_clocks.insert(related.clone());
                    }
                }
                if b0 != b1 {
                    // Every moving toggle — clock, data or async alike — is a uniform move: the source
                    // state, the destination stable state and the post value the fixpoint replays.
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

    // The per-arc decision core: for each candidate the seed-veto-fixpoint yields the set of `(clock,
    // direction)` arcs it keeps (empty ⇒ no annotation). Computed BEFORE any synthesis, so the header
    // (which excludes internal capture-less nodes) is settled first. The single-input transition table is
    // node-independent, so it is built once and shared by every candidate's hold walks.
    let trans = Transitions::build(m);
    let (capture_sets, vetoed_sets): (Vec<BTreeSet<Arc>>, Vec<BTreeSet<Symbol>>) = candidates
        .iter()
        .zip(&aggs)
        .map(|(name, agg)| capture_arcs(m, &trans, name, &clock_set, agg))
        .unzip();
    // The PER-ARC LABELS, sourced from the arc pipeline: each derived delay arc whose related pin is a
    // declared clock is labelled from its own `(output, clock, direction)` identity — Capture when the
    // decision core kept that direction, nothing when the clock is level-vetoed on the node (the arc
    // stays combinational), Release otherwise (the arc's existence attests the change). An arc the
    // machine never presents has no key, so masking needs no rule here: it already happened in
    // `arcs::derive`.
    let cand_index: HashMap<&str, usize> = candidates
        .iter()
        .enumerate()
        .map(|(i, c)| (c.as_str(), i))
        .collect();
    let mut labels: BTreeMap<(Symbol, Symbol, Edge), ArcClass> = BTreeMap::new();
    for a in delay_arcs {
        if !clock_set.contains(a.related.as_str()) {
            continue;
        }
        let Some(&i) = cand_index.get(a.output.as_str()) else {
            continue;
        };
        let is_rise = a.end.value_of(a.related.as_str()) == Some(true);
        let class = if capture_sets[i].contains(&(a.related.clone(), is_rise)) {
            ArcClass::Capture
        } else if vetoed_sets[i].contains(&a.related) {
            continue; // a level-acting clock: the arc stays combinational
        } else {
            ArcClass::Release
        };
        let edge = if is_rise { Edge::Rise } else { Edge::Fall };
        labels.insert((a.output.clone(), a.related.clone(), edge), class);
    }
    // The internal, capture-less nodes: excluded from a capture's tier-1 header (so a slave's capture
    // generalises past a master it will fold) and the fold candidates. Output nodes are never folded, so
    // their names always stay available in the header.
    let internal_captureless: BTreeSet<Symbol> = candidates
        .iter()
        .zip(&capture_sets)
        .filter(|(name, s)| s.is_empty() && !output_names.contains(name.as_str()))
        .map(|(name, _)| name.clone())
        .collect();

    let mut captures: Vec<EdgeCaptures> = Vec::new();
    // Internal capture-less nodes pulled back into a tier-2 header survive (become unfoldable).
    let mut tier2_kept: BTreeSet<Symbol> = BTreeSet::new();

    for (i, s) in capture_sets.iter().enumerate() {
        if s.is_empty() {
            continue;
        }
        let name = &candidates[i];
        let agg = &aggs[i];
        // The keying clocks in cell input-pin order, each with the `(is_rise, Edge)` directions the
        // decision core kept (Rise before Fall). Every clock present carries at least one kept direction.
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

        let (node_captures, off_edge, tier2) = synth_node_captures(
            &builder,
            name,
            &candidates,
            &internal_captureless,
            inputs,
            &clock_edges,
            agg,
        );
        if tier2 {
            tier2_kept.extend(internal_captureless.iter().cloned());
        }
        let cols = capture_cols(&node_captures, &off_edge);
        captures.push(EdgeCaptures {
            node: name.clone(),
            captures: node_captures,
            off_edge,
            cols,
        });
    }

    // FOLD (cell-level): an internal capture-less master is folded when nothing surviving references it.
    let ref_reg: BTreeSet<&str> = captures
        .iter()
        .flat_map(|r| r.cols.iter().map(Symbol::as_str))
        .collect();
    // Function support of every candidate, for the surviving-signal reference check.
    let mut fn_of: BTreeMap<&str, &Bdd<B, C>> = BTreeMap::new();
    for (n, d) in deltas {
        fn_of.insert(n.as_str(), d);
    }
    for (n, d) in &m.out_deltas {
        fn_of.insert(n.as_str(), d);
    }
    // Every surviving signal whose RAW function is still emitted — the capture-less candidates (level and
    // combinational nodes, whose region cols come straight from their function support). Candidates that
    // carry edge arcs are excluded: their raw function is replaced by the edge seam, so their cols are
    // accounted for via `ref_reg`. A folded master must not be referenced by ANY surviving raw function,
    // or the survivor's cols would name a dropped node (statetable invariant I3).
    let survivor_names: Vec<&Symbol> = candidates
        .iter()
        .zip(&capture_sets)
        .filter(|(_, s)| s.is_empty())
        .map(|(n, _)| n)
        .collect();

    let folded: Vec<Symbol> = candidates
        .iter()
        .filter(|m| internal_captureless.contains(*m))
        .filter(|m| {
            // (a) no capture/off-edge cover references it,
            if ref_reg.contains(m.as_str()) {
                return false;
            }
            // (b) no OTHER surviving signal references it,
            let referenced = survivor_names.iter().any(|l| {
                *l != *m
                    && fn_of
                        .get(l.as_str())
                        .is_some_and(|f| f.variables().any(|v| v.as_str() == m.as_str()))
            });
            if referenced {
                return false;
            }
            // (c) internal (guaranteed by internal_captureless), (d) not tier-2 re-included.
            !tier2_kept.contains(*m)
        })
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
/// state machine, not any one candidate — so it is built once per cell and every candidate's hold walk
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
/// Rule R\* no longer consults it — a change-free edge takes no arc whatever the phase it closes — so its
/// remaining consumer is the replay-faithfulness harness, which reads an OPEN phase against the cover.
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

/// The `(clock, direction)` edge arcs one candidate node keeps, in three stages, together with the
/// clocks VETOED on it. A vetoed clock decides the node by its LEVEL, so its arcs are ordinary
/// combinational clock-gate arcs — neither captures nor releases; the veto verdict is returned so the
/// per-arc labelling can tell a level-acting clock (no label) from a releasing one. The veto is judged
/// for EVERY clock whose edge moved the node, seeded or not: a clock with no edge content can still be
/// level-acting (`RDFF`'s clock-declared reset pins the node by its level and must not label the arc a
/// release).
///
/// 1. **SEED by CONTENT** over ALL firings of each direction, changed or not: the edge has *state
///    content* (two firings from equal non-clock input projections deliver different values) or *pin
///    content* (a pin outside the eliminated set changes the delivered value). The ELIMINATED set is the
///    non-clock inputs whose toggle moves the node — coexisting combinational arcs (async resets, latch
///    data); they contribute no edge content but never disqualify the clock.
/// 2. **CAPTURED-CONTENT-IRRELEVANCE VETO**, before the fixpoint: a clock is vetoed on the node when some
///    cube of CLOCK LITERALS ALONE pins the node to a constant, that clock's literal being NECESSARY to
///    the pinning — in such a phase the captured content is irrelevant and the clock LEVEL alone decides
///    the node, which is a combinational clock gate, not a capture (see [`pinned_by_clock_levels`]).
/// 3. **PER-ARC RULE**: keep `(K, d)` iff the direction CHANGED the node in some firing (a real
///    effect) and the DELIVERED phase is QUIET ([`phase_quiet`]): no live data reaches the node
///    inside the phase, judged with the node's behaviourally-classified forcing pins
///    ([`forcing_pins`]) exempted and co-resident clock movers admitted unless they change a
///    phase-wide carrier the node tracks. Each arc is decided independently — no fixpoint, no
///    mutual support between arcs.
fn capture_arcs<B: Brand, C: ManagerCell>(
    m: &Machine<'_, B, C>,
    tr: &Transitions<'_>,
    node: &Symbol,
    clock_set: &BTreeSet<&str>,
    agg: &CandAgg,
) -> (BTreeSet<Arc>, BTreeSet<Symbol>) {
    let inputs = &m.cell.inputs;

    // E(N): the non-clock inputs that move the node — coexisting combinational arcs.
    let eliminated: BTreeSet<&str> = agg
        .moves
        .iter()
        .map(|(x, _, _, _)| x.as_str())
        .filter(|x| !clock_set.contains(x))
        .collect();

    // (1) SEED by content, over every observed direction of every declared clock.
    let mut s: BTreeSet<Arc> = agg
        .captures
        .iter()
        .filter(|((clock, _), cap)| edge_has_content(inputs, clock, &eliminated, cap))
        .map(|(arc, _)| arc.clone())
        .collect();

    // (2) CAPTURED-CONTENT-IRRELEVANCE VETO, judged for every clock the seed names PLUS every clock
    // whose edge CHANGED the node: an unseeded clock never keeps a capture, but its veto verdict still
    // decides whether its arcs are level-acting (combinational) or releases.
    let clock_pins: Vec<&str> = inputs
        .iter()
        .map(Symbol::as_str)
        .filter(|p| clock_set.contains(p))
        .collect();
    let candidate_clocks: BTreeSet<Symbol> = s
        .iter()
        .map(|(k, _)| k.clone())
        .chain(agg.changed_clocks.iter().cloned())
        .collect();
    let mut vetoed: BTreeSet<Symbol> = BTreeSet::new();
    for k in &candidate_clocks {
        if pinned_by_clock_levels(m, node, &clock_pins, k.as_str()) {
            s.retain(|(clock, _)| clock != k);
            vetoed.insert(k.clone());
        }
    }

    // (3) PER-ARC RULE: keep (K, d) iff CHANGED and the delivered phase is QUIET.
    if !s.is_empty() {
        let forcing = forcing_pins(m, tr, node, clock_set, &agg.moves);
        s.retain(|(k, d)| {
            let changed = agg
                .captures
                .get(&(k.clone(), *d))
                .is_some_and(|c| c.changed);
            changed && phase_quiet(m, tr, node, clock_set, k, *d, &forcing)
        });
    }

    (s, vetoed)
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
    clock_set: &BTreeSet<&str>,
    moves: &[(Symbol, Minterm<Symbol>, Minterm<Symbol>, bool)],
) -> BTreeMap<Symbol, (bool, bool)> {
    let inputs = &m.cell.inputs;
    // Clause 2 - GLOBAL CLAMP (non-clock pins): exactly one level of the pin pins the node to one
    // constant across ALL reachable stable states (an async override whose release re-acquires, like
    // a toggle flop's reset). No tracked data pin satisfies this: its tracking is confined to a
    // clock-phase region, and elsewhere the node varies under the same pin level.
    let mut clamp: BTreeMap<Symbol, (bool, bool)> = BTreeMap::new();
    for x in inputs {
        if clock_set.contains(x.as_str()) {
            continue;
        }
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

/// Is the delivered phase of `(clock, is_rise)` QUIET on `node` — no NON-CLOCK toggle changes the node
/// from any reachable stable state of the phase, transitions into or out of a forced region excluded?
fn phase_quiet<B: Brand, C: ManagerCell>(
    m: &Machine<'_, B, C>,
    tr: &Transitions<'_>,
    node: &Symbol,
    clock_set: &BTreeSet<&str>,
    clock: &Symbol,
    is_rise: bool,
    forcing: &BTreeMap<Symbol, (bool, bool)>,
) -> bool {
    let is_forced = |s: &Minterm<Symbol>| {
        forcing
            .iter()
            .any(|(p, (a, _))| s.value_of(p.as_str()) == Some(*a))
    };
    // Phase-wide CARRIERS: the state variables W != node with node == W (or == !W) across every
    // non-forced stable state of the delivered phase. A clock-toggle mover disqualifies only when it
    // changes a carrier - the node then TRACKS a live signal through the phase (transparency). A
    // clock toggle moving the node without touching a carrier is a mux switch between held values
    // (an opaque presentation) or a co-resident capture, neither of which is tracking.
    let mut carrier_pol: Vec<(usize, [bool; 2])> = m
        .state_vars
        .iter()
        .enumerate()
        .filter(|(_, sv)| sv.as_str() != node.as_str())
        .map(|(wi, _)| (wi, [true, true])) // [id still viable, neg still viable]
        .collect();
    for s in tr.order.iter() {
        if s.value_of(clock.as_str()) != Some(is_rise) || is_forced(s) {
            continue;
        }
        let Some(v) = m.output_value(node.as_str(), s) else {
            continue;
        };
        for (wi, pol) in carrier_pol.iter_mut() {
            match s.value_of(m.state_vars[*wi].as_str()) {
                Some(w) => {
                    if w != v {
                        pol[0] = false;
                    }
                    if w == v {
                        pol[1] = false;
                    }
                }
                None => {
                    pol[0] = false;
                    pol[1] = false;
                }
            }
        }
    }
    // INDEPENDENCE: a candidate carrier must move somewhere in the machine WITHOUT the node
    // moving with it. A complement node of the same latch (a cross-coupled NAND pair) co-moves with
    // the node on every transition and phase-wide equals its negation - it is the same storage
    // element seen from the other side, not a live signal the node tracks.
    let independent = |w: &Symbol| -> bool {
        tr.order.iter().enumerate().any(|(si, s)| {
            let (Some(wv), Some(nv)) = (s.value_of(w.as_str()), m.output_value(node.as_str(), s))
            else {
                return false;
            };
            tr.next[si].iter().any(|ni| {
                ni.is_some_and(|ni| {
                    let dest = &tr.order[ni];
                    dest.value_of(w.as_str()).is_some_and(|dw| dw != wv)
                        && m.output_value(node.as_str(), dest) == Some(nv)
                })
            })
        })
    };
    let carriers: Vec<&Symbol> = carrier_pol
        .iter()
        .filter(|(_, pol)| pol[0] || pol[1])
        .map(|(wi, _)| &m.state_vars[*wi])
        .filter(|w| independent(w))
        .collect();

    for (si, s) in tr.order.iter().enumerate() {
        if s.value_of(clock.as_str()) != Some(is_rise) || is_forced(s) {
            continue;
        }
        let Some(v) = m.output_value(node.as_str(), s) else {
            continue;
        };
        for (xi, x) in m.cell.inputs.iter().enumerate() {
            if x == clock {
                continue; // the clock under test stays pinned across its own phase
            }
            let Some(ni) = tr.next[si][xi] else { continue };
            let dest = &tr.order[ni];
            if is_forced(dest) {
                continue;
            }
            if m.output_value(node.as_str(), dest) == Some(v)
                || m.output_value(node.as_str(), dest).is_none()
            {
                continue; // the node did not move
            }
            if forcing.contains_key(x) {
                continue; // a forcing pin's assertion is a coexisting combinational arc
            }
            if !clock_set.contains(x.as_str()) {
                return false; // live data reaches the node inside the phase
            }
            // A co-resident clock toggle: disqualifying only when it changes a carrier the node
            // tracks phase-wide.
            if carriers
                .iter()
                .any(|w| dest.value_of(w.as_str()) != s.value_of(w.as_str()))
            {
                return false;
            }
        }
    }
    true
}

/// Does `clock` act COMBINATIONALLY on `node` — is there a phase in which the CAPTURED CONTENT is
/// IRRELEVANT, the clock LEVEL alone deciding the node's settled value?
///
/// Operationally: some cube of CLOCK LITERALS ALONE pins the node to a constant over every reachable
/// stable state it covers — regardless of every data pin and every state coordinate — and `clock`'s
/// literal is NECESSARY to that pinning (dropping it unpins the node). Necessity is what keeps the test
/// local to the gating clocks: a node pinned by some OTHER clock's level is vetoed on that clock only,
/// its own capture clock surviving in the larger cube that merely inherits the pinning.
///
/// An integrated clock gate (`GCLK = CLK*EL`) is pinned by `!CLK`, and a multi-clock one
/// (`GCLK = enA*CLKA + enB*CLKB`) by `!CLKA*!CLKB`, so both clocks go; a dual-edge flop
/// (`Q = CLK*L1 + !CLK*L2`) is pinned by neither phase — the captured content stays relevant throughout —
/// and keeps both its edges, as does a reset flop, whose forcing cube `CLK*R` is not clock literals alone.
fn pinned_by_clock_levels<B: Brand, C: ManagerCell>(
    m: &Machine<'_, B, C>,
    node: &Symbol,
    clocks: &[&str],
    clock: &str,
) -> bool {
    let others: Vec<&str> = clocks.iter().copied().filter(|c| *c != clock).collect();

    // Is the node constant over every reachable stable state matching `ctx` (a partial assignment of
    // `others`) plus the optional literal on `clock`? An unwitnessed cube pins nothing.
    let pins = |ctx: &[Option<bool>], lit: Option<bool>| -> bool {
        let mut seen: Option<bool> = None;
        for state in &m.explored.order {
            if lit.is_some_and(|l| state.value_of(clock) != Some(l))
                || others
                    .iter()
                    .zip(ctx)
                    .any(|(p, l)| l.is_some_and(|l| state.value_of(*p) != Some(l)))
            {
                continue;
            }
            let Some(v) = m.output_value(node.as_str(), state) else {
                continue;
            };
            match seen {
                None => seen = Some(v),
                Some(prev) if prev == v => {}
                _ => return false,
            }
        }
        seen.is_some()
    };

    // Every cube over the OTHER clocks (each pin low, high or don't-care), as a base-3 counter.
    let mut ctx: Vec<Option<bool>> = vec![None; others.len()];
    for code in 0..3usize.pow(u32::try_from(others.len()).unwrap_or(u32::MAX)) {
        let mut rest = code;
        for slot in ctx.iter_mut() {
            *slot = [None, Some(false), Some(true)][rest % 3];
            rest /= 3;
        }
        // `clock`'s literal must do the pinning: a context that already pins on its own says nothing
        // about this clock.
        if pins(&ctx, None) {
            continue;
        }
        if [false, true].iter().any(|l| pins(&ctx, Some(*l))) {
            return true;
        }
    }
    false
}

/// Does a clock direction's firing census carry edge CONTENT — is there anything for the edge to deliver?
/// STATE content: two firings whose pre-states share a non-clock input projection deliver DIFFERENT values
/// (the edge transports state). PIN content: a pin outside the ELIMINATED set changes the delivered value
/// (the edge transports that pin). Seeding is by content over ALL firings, never gated on whether a firing
/// changed the node: content is what the edge could deliver, and Rule R\* is where the change and the hold
/// obligation are judged.
fn edge_has_content(
    inputs: &[Symbol],
    clock: &Symbol,
    eliminated: &BTreeSet<&str>,
    cap: &CapAgg,
) -> bool {
    let others: Vec<&str> = inputs
        .iter()
        .map(Symbol::as_str)
        .filter(|p| *p != clock.as_str())
        .collect();

    // STATE content.
    let mut by_proj: BTreeMap<Minterm<Symbol>, bool> = BTreeMap::new();
    for (pre, _dest, post) in &cap.firings {
        let proj = pre.project_to(others.iter().copied());
        if by_proj
            .insert(proj, *post)
            .is_some_and(|prev| prev != *post)
        {
            return true;
        }
    }

    // PIN content: within each projection of the OTHER pins, does `p` flip the delivered value?
    for p in others.iter().filter(|p| !eliminated.contains(*p)) {
        let rest: Vec<&str> = others.iter().copied().filter(|q| q != p).collect();
        let mut groups: BTreeMap<Minterm<Symbol>, [BTreeSet<bool>; 2]> = BTreeMap::new();
        for (pre, _dest, post) in &cap.firings {
            let Some(pv) = pre.value_of(*p) else { continue };
            groups
                .entry(pre.project_to(rest.iter().copied()))
                .or_default()[usize::from(pv)]
            .insert(*post);
        }
        if groups
            .values()
            .any(|[low, high]| low.iter().any(|v| high.contains(&!*v)))
        {
            return true;
        }
    }
    false
}

/// Synthesise a node's captures and off-edge, escalating tier-1 → tier-2 on a capture conflict. The
/// off-edge is synthesised JOINTLY over the node's whole clock set. The `bool` is whether tier-2 was used.
///
/// The fixpoint (and the replay-faithfulness harness) guarantee this is only ever called on a node whose
/// clocks hold cleanly, so neither a surviving tier-2 capture conflict nor a joint off-edge disagreement
/// can occur; both are guarded by a `debug_assert!` rather than a silent fallback. Emission is
/// UNCONDITIONAL — every recognised edge arc is kept, with no per-clock off-edge fallback.
#[allow(clippy::too_many_arguments)]
fn synth_node_captures<B: Brand, C: ManagerCell>(
    builder: &BddBuilder<B, C>,
    node: &Symbol,
    candidates: &[Symbol],
    internal_captureless: &BTreeSet<Symbol>,
    inputs: &[Symbol],
    clock_edges: &[(Symbol, Vec<(bool, Edge)>)],
    agg: &CandAgg,
) -> Synthesised {
    let clocks: Vec<Symbol> = clock_edges.iter().map(|(c, _)| c.clone()).collect();

    for tier2 in [false, true] {
        // Capture per clock (input-pin order), per active edge (Rise first). Each capture's header is the
        // inputs minus THAT capture's clock, then the candidate signal names; internal capture-less nodes
        // are excluded at tier-1 and re-included at tier-2. The node's own name is always present (a
        // toggle flop captures a function of its own prior state). Any OTHER clock stays in the header as
        // an ordinary level column.
        let mut captures: Vec<(Symbol, Edge, StateRegions)> = Vec::new();
        let mut conflict = false;
        'clocks: for (clock, edges) in clock_edges {
            let header: Vec<Symbol> = inputs
                .iter()
                .filter(|p| p.as_str() != clock.as_str())
                .cloned()
                .chain(
                    candidates
                        .iter()
                        .filter(|c| tier2 || !internal_captureless.contains(*c))
                        .cloned(),
                )
                .collect();

            for (is_rise, edge) in edges {
                let samples = agg
                    .captures
                    .get(&(clock.clone(), *is_rise))
                    .map(|c| c.samples.as_slice())
                    .unwrap_or(&[]);
                match synth_capture(builder, &header, samples) {
                    Some(sr) => captures.push((clock.clone(), *edge, sr)),
                    None => {
                        conflict = true;
                        break 'clocks;
                    }
                }
            }
        }
        if conflict && !tier2 {
            continue; // escalate to tier-2
        }
        // A surviving tier-2 conflict is unreachable given the fixpoint + harness: assert, never fall back.
        debug_assert!(
            !conflict,
            "tier-2 capture conflict on node {}",
            node.as_str()
        );

        // Off-edge over ALL the inputs, the node's own clocks INCLUDED: the hold-and-set/clear behaviour is
        // input driven, so the state coordinates are not columns (the value held is the node's own, absent
        // from the header, and any forcing comes from an input). A data input that never forces simply
        // lands every projection in `hold` and drops out of the cols; a PHASE-AGREED forcing makes each
        // clock a don't-care in every forcing cube, so it drops out of the cover support too, while a
        // phase-CONDITIONED reset keeps its gating clock pinned to the forcing level (`CLK*R`).
        let off_edge = synth_off_edge(builder, inputs, &clocks, &agg.stable);

        return (captures, off_edge, tier2);
    }
    unreachable!("the tier loop always returns")
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

    /// The per-arc labels as `(output, clock, clock direction, class)` string tuples, in the map's own
    /// (deterministic) key order.
    fn label_list(es: &EdgeArcs) -> Vec<(&str, &str, Edge, ArcClass)> {
        es.labels
            .iter()
            .map(|((n, c, e), cls)| (n.as_str(), c.as_str(), *e, *cls))
            .collect()
    }

    /// The labels carried by one output's arcs, as `(clock, clock direction, class)`.
    fn labels_of<'a>(es: &'a EdgeArcs, node: &str) -> Vec<(&'a str, Edge, ArcClass)> {
        label_list(es)
            .into_iter()
            .filter(|(n, _, _, _)| *n == node)
            .map(|(_, c, e, cls)| (c, e, cls))
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

    // === Step 3 (2): stay level (no annotation) ===

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
            // M is an output master (never folded); the slave Q is recognised and its capture equals the
            // master's held value M over the reachable states (D and M coincide there, so generalisation
            // may render the cover as either — both are the same captured value).
            assert!(
                !folded_list(&es).contains(&"M"),
                "an output master is not folded"
            );
            let mut reach = builder.constant(false);
            for state in &m.explored.order {
                reach = reach.or(&super::cube_bdd(&builder, state));
            }
            let on = builder.build_cover(&q.captures[0].2.on_cover).and(&reach);
            let want = builder.var("M").and(&reach);
            assert!(
                on.equivalent_to(&want),
                "capture equals the surviving master M's value"
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
        // The self-fed master M has no *data* input (R is async), so `data_changed` cannot mark it level:
        // the ring is decomposed into TWO edge seams rather than folding M into Q. M is the inverting
        // node — it captures !Q on the falling edge, recorded verbatim (inversion is not special-cased) —
        // and Q captures the master M on the rising edge (the self-referential ring, M in Q's cols).
        with_machine!(TOGGLE_FLOP_TOML, |builder, _a, _m2, m| {
            let es = classify(&m);
            assert_captures_faithful(&m, &es);
            let q = reg(&es, "Q");
            let mm = reg(&es, "M");
            assert_eq!(q.captures[0].1, Edge::Rise);
            assert_eq!(mm.captures[0].1, Edge::Fall);
            assert!(
                q.captures[0].2.cols.iter().any(|c| c == "M"),
                "Q captures the master M (ring), cols {:?}",
                cols_of(&q.captures[0].2)
            );
            // M's falling capture is self-inverting: at the pre-fall (CLK=1) states M equals Q, so
            // capturing !M is capturing !Q — recorded verbatim as `!R*!M`, no special-casing of inversion.
            let mcap = &mm.captures[0].2;
            assert!(
                mcap.cols.iter().any(|c| c == "M"),
                "self in cols: {:?}",
                cols_of(mcap)
            );
            let m_on = builder.build_cover(&mcap.on_cover);
            let want = (!&builder.var("R")).and(&!&builder.var("M"));
            assert!(
                m_on.equivalent_to(&want),
                "M captures !M (=!Q), inverting, no special-casing"
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

    // === Phase-symmetric data transparency must NOT read as an edge register ===

    // Two opposite-phase D latches XORed: M follows D while CLK=0, M2 follows D while CLK=1, and T = M⊕M2
    // is transparent to D in BOTH phases. D is phase-SYMMETRIC (not a latch signature) and lands Held
    // off-edge (it forces T to no constant), so it is genuine data transparency — recognising T as an
    // edge register would DROP D while the same run emits combinational D→T arcs under both phases. T must
    // stay level; D must survive as a data dependency of T's function.
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
        // recognised consistently as an edge register. Grounding the permit decision in the exact off-edge
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
            // R is LEVEL-ACTING on Q (R=1 alone pins it to 0), and the veto covers unseeded clocks
            // too: R's assert arcs carry NO label — never a release — so they emit
            // `-type combinational`, exactly as when R is not declared a clock (SYNCR).
            assert_eq!(
                labels_of(&es, "Q"),
                [("CLK", Edge::Rise, ArcClass::Capture)],
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

            // The guard has teeth: classification is a no-op when suppressed and does recognise registers
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
    // overall -- the whole chain follows D through CLK's low phase, so it stays LEVEL and carries ZERO
    // edge arcs on every node (it falls out as level, not by any dismissal). The XLAT analogue.
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
        // Q's arc set is EXACTLY {CLKA:Rise, CLKB:Rise}, and the two rejections are the two prongs of the
        // no-vacuous guard meeting opposite answers:
        //
        // * CLKA:Fall is change-free AND its opposite phase (CLKA=1) is NOT transparent — Q is hysteretic
        //   there, only failing the hold obligation because the co-resident CLKB moves it — so the edge is
        //   a total non-event and takes no arc;
        // * CLKB:Rise is equally change-free, but its opposite phase (CLKB=0) IS genuinely transparent, so
        //   it is a real latch close (the output stops tracking) and keeps its arc.
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
                [("CLKA", Edge::Rise)],
                "Q carries exactly the conditioned CLKA rising capture"
            );
            // The capture characterises the condition: Q captures D when CLKB is transparent
            // (CLKB=0) and re-delivers its own held value when CLKB is opaque (CLKB=1).
            let on = builder.build_cover(&q.captures[0].2.on_cover);
            let clkb = builder.var("CLKB");
            let want = clkb
                .and(&builder.var("Q"))
                .or(&(!&clkb).and(&builder.var("D")));
            assert!(
                on.equivalent_to(&want),
                "capture is CLKB*Q + !CLKB*D (conditioned on CLKB transparent)"
            );
            // Q is a latch on CLKB: no CLKB edge arc in either direction.
            assert!(
                !arcs.iter().any(|(c, _)| *c == "CLKB"),
                "a latch carries no edge arc on its own clock"
            );
            // The surviving master node keeps CLKA; the inner master folds.
            assert!(
                clocks_of(reg(&es, "M2")).contains(&"CLKA"),
                "master node keeps CLKA"
            );
            assert!(folded_list(&es).contains(&"M1"), "the inner master folds");
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

            // KNOWN DIFFERENCE, asserted rather than hidden: the pass DFF folds its master M, whereas the
            // NAND master pair M/Mn is captureless and MUTUALLY REFERENCING, so each is "referenced
            // elsewhere" by the other and the per-node fold rule strands BOTH as surviving level
            // internals. Arcs, covers and the Q characterisation are invariant — only the folding
            // differs. FOLLOW-UP (contained, fold rule only): group-fold a set of mutually-referencing
            // captureless nodes when the set as a whole has no reference from outside it.
            assert!(
                folded_list(&es).is_empty(),
                "the mutually-referencing NAND master pair is not folded, folded={:?}",
                folded_list(&es)
            );
            assert_eq!(
                node_list(&es),
                ["Q", "Qn"],
                "M/Mn survive as level internals carrying no arc"
            );
        });
    }

    #[test]
    fn edge_nand_hierarchical_two_clocks_matches_the_pass_gate_pipe() {
        // (3) A NAND flop on CLKA feeding a NAND latch on CLKB characterises exactly as `HPIPE`: Q takes
        // the conditioned CLKA rising capture and NOTHING on CLKB (its own latch clock), and the surviving
        // master node keeps CLKA.
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
                [("CLKA", Edge::Rise)],
                "Q carries exactly the conditioned CLKA rising capture"
            );
            assert!(
                !arcs.iter().any(|(c, _)| *c == "CLKB"),
                "a latch carries no edge arc on its own clock"
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
            let on = builder.build_cover(&q.captures[0].2.on_cover).and(&reach);
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
        // arc at all: the label set is exactly the one capture.
        with_machine!(DFF_TOML, |_b, _a, _m2, m| {
            let es = classify(&m);
            assert_eq!(
                label_list(&es),
                [("Q", "CLK", Edge::Rise, ArcClass::Capture)]
            );
        });

        // A plain latch is the pure release case: no capture anywhere, and the enable's opening edge
        // labels the observed CLK-rise arcs.
        with_machine!(DLAT_TOML, |_b, _a, _m2, m| {
            let es = classify(&m);
            assert!(es.captures.is_empty(), "a latch has no capture");
            assert_eq!(
                label_list(&es),
                [("Q", "CLK", Edge::Rise, ArcClass::Release)]
            );
        });

        // Two latches on the SAME phase never form a flop, so the cascade is captureless — but the
        // chain's output opens on CLK's fall and that arc is a release.
        with_machine!(TCASC_TOML, |_b, _a, _m2, m| {
            let es = classify(&m);
            assert!(
                es.captures.is_empty(),
                "a transparent cascade has no capture"
            );
            assert_eq!(
                label_list(&es),
                [("Q", "CLK", Edge::Fall, ArcClass::Release)]
            );
        });

        // A dual-edge flop captures on BOTH directions; the masters' own openings are internal and
        // carry no delay arc, so no release key exists.
        with_machine!(DET_TOML, |_b, _a, _m2, m| {
            let es = classify(&m);
            assert_eq!(
                label_list(&es),
                [
                    ("Q", "CLK", Edge::Rise, ArcClass::Capture),
                    ("Q", "CLK", Edge::Fall, ArcClass::Capture),
                ]
            );
        });
    }

    #[test]
    fn edge_labels_two_clock_sourced_from_arcs() {
        // Two independent flops merging into one output: every clock arc Q presents is a capture, and
        // the internal masters — whose own openings carry no delay arc — take no key.
        with_machine!(DCMUX_TOML, |_b, _a, _m2, m| {
            let es = classify(&m);
            assert_eq!(
                label_list(&es),
                [
                    ("Q", "CLKA", Edge::Rise, ArcClass::Capture),
                    ("Q", "CLKA", Edge::Fall, ArcClass::Capture),
                    ("Q", "CLKB", Edge::Rise, ArcClass::Capture),
                    ("Q", "CLKB", Edge::Fall, ArcClass::Capture),
                ]
            );
        });

        // HPIPE: a CLKA flop feeding a CLKB latch. Q carries BOTH a CLKA rising capture and a CLKB
        // falling release — the two categories coexist on one output, each labelling its own arcs.
        with_machine!(HPIPE_TOML, |_b, _a, _m2, m| {
            let es = classify(&m);
            assert_eq!(
                label_list(&es),
                [
                    ("Q", "CLKA", Edge::Rise, ArcClass::Capture),
                    ("Q", "CLKB", Edge::Fall, ArcClass::Release),
                ]
            );
            assert_eq!(
                reg(&es, "Q")
                    .captures
                    .iter()
                    .map(|(c, e, _)| (c.as_str(), *e))
                    .collect::<Vec<_>>(),
                [("CLKA", Edge::Rise)],
                "the capture set is untouched by the release labelling"
            );
        });

        // MCDFF: two latches on UNRELATED clocks. It stays captureless (its zero-capture fixture is a
        // separate assertion and must remain true), yet it is not timing-invisible — Q's CLKB rise opens
        // its own latch and Q's CLKA fall is a CONDITIONED release reaching Q only through the open CLKB
        // latch. Conditioning never reclassifies an arc.
        with_machine!(MCDFF_TOML, |_b, _a, _m2, m| {
            let es = classify(&m);
            assert!(
                es.captures.is_empty(),
                "captures and labels are separate: MCDFF still captures nothing, got {:?}",
                node_list(&es)
            );
            assert_eq!(
                label_list(&es),
                [
                    ("Q", "CLKA", Edge::Fall, ArcClass::Release),
                    ("Q", "CLKB", Edge::Rise, ArcClass::Release),
                ]
            );
        });
    }

    #[test]
    fn edge_clock_gate_arcs_carry_no_label() {
        // A gated clock is combinational in a held enable: the veto strips its clocks, so its arcs take
        // no label and stay `-type combinational`. The enable latch behind it is internal — no delay
        // arc, no key — so the whole label map is empty.
        with_machine!(ICG_TOML, |_b, _a, _m2, m| {
            let es = classify(&m);
            assert!(
                es.labels.is_empty(),
                "a clock gate's arcs carry no edge label: {:?}",
                label_list(&es)
            );
        });

        // ICM: both clocks are vetoed on GCLK, and the competing-branch propagation into the OTHER
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
        // output's complement explicitly, so the complement output lists the SAME key with the same
        // class — never a different edge.
        for (toml, want) in [
            (
                NDLAT_TOML,
                vec![
                    ("Q", "CLK", Edge::Rise, ArcClass::Release),
                    ("Qn", "CLK", Edge::Rise, ArcClass::Release),
                ],
            ),
            (
                NDFF_TOML,
                vec![
                    ("Q", "CLK", Edge::Rise, ArcClass::Capture),
                    ("Qn", "CLK", Edge::Rise, ArcClass::Capture),
                ],
            ),
            (
                NHPIPE_TOML,
                vec![
                    ("Q", "CLKA", Edge::Rise, ArcClass::Capture),
                    ("Q", "CLKB", Edge::Fall, ArcClass::Release),
                    ("Qn", "CLKA", Edge::Rise, ArcClass::Capture),
                    ("Qn", "CLKB", Edge::Fall, ArcClass::Release),
                ],
            ),
        ] {
            with_machine!(toml, |_b, _a, _m2, m| {
                let es = classify(&m);
                assert_eq!(label_list(&es), want);
            });
        }
    }
}
