//! Behavioural edge-sensitivity classification.
//!
//! Each candidate node — every output **and** every internal state variable — is classified, purely
//! from the cell's already-explored toggle-and-settle behaviour, as edge-triggered (and on which clock
//! edge or edges), level-sensitive, or combinational. An edge-triggered node captures a next-state
//! value at each active edge of one declared clock and holds otherwise; a level node follows a data
//! input through a transparent phase. Captures are synthesised through the [`super::regions`] FR cover
//! pipeline and recorded verbatim as ordinary functions — an inverting flop's capture is simply `!D`,
//! never special-cased.
//!
//! [`classify`] is a **post-exploration** read-only pass over the shared [`Machine`]: it re-walks the
//! exploration with [`machine::toggle`]/[`machine::settle`], mirroring [`super::arcs::derive`]'s
//! per-node walk, and only ADDS an edge-sensitivity annotation. It never re-derives the exploration,
//! the prevectors or the hazards — those stay byte-identical whether the annotation is on or off.
//!
//! See `docs/edge-collapse.md` for the concept-first walkthrough: the phase-asymmetry transition
//! predicate, the capture and off-edge synthesis, the cell-level fold and toggle-flop decomposition,
//! how the master-slave hold and async-agreement guarantees are subsumed behaviourally, and the
//! retained restrictions.
//!
//! # Classification
//!
//! Each candidate node's observations are aggregated across the walk, then classified:
//!
//! * **level** — some *data* input is transparent to the node in one clock phase but not the other (the
//!   node follows an input during a phase). A level node emits its ordinary hysteretic regions and takes
//!   no annotation; an internal level node is a foldable master.
//! * **register** — a declared clock's edge(s) change the node and the node's stable value is
//!   independent of that clock's LEVEL (its exact off-edge synthesises cleanly — off-edge phase
//!   agreement — so no data input is transparent to it). The node captures a next-state value at each
//!   active edge and holds otherwise. A node combinational in a clock's level is never a register on
//!   that clock, whatever the clock count.
//! * **none** — combinational: no annotation.
//!
//! The capture (per active edge) and the off-edge (hold + async set/clear) functions are synthesised
//! from the sampled pre-states and stable states over a deterministic two-tier header, reusing the
//! [`super::regions`] region pipeline.

use std::collections::{BTreeMap, BTreeSet};

use espresso_logic::bdd::{Bdd, BddBuilder, Brand, ManagerCell};
use espresso_logic::{Cover, CoverType, CubeType, Minimizable, Minterm, Symbol};
use rayon::prelude::*;

use crate::logic::analysis::Machine;
use crate::logic::arcs::Edge;
use crate::logic::machine;
use crate::logic::regions::{self, StateRegions};

/// One recognised edge-triggered register: a node re-expressed as an edge seam on one or more clocks.
#[derive(Debug, Clone)]
pub struct EdgeRegister {
    /// The node that becomes the register's output coordinate.
    pub node: Symbol,
    /// The captured next-state function per active edge, as combinational state-table regions (total —
    /// off is the complement of on, empty hold). Each capture carries ITS OWN clock. Grouped by clock in
    /// cell input-pin order, `Rise` before `Fall` within a clock; a single-clock register keeps one entry
    /// per active edge (two for a dual-edge register with `Rise` first), byte-identical to a single-clock
    /// keying.
    pub captures: Vec<(Symbol, Edge, StateRegions)>,
    /// The off-edge (hold) function as state-table regions, keyed by the clock set's phase vector: on/off
    /// are the async set/clear covers, hold is the quiescent region; never references any of the clocks.
    pub off_edge: StateRegions,
    /// The register's column set: the first-appearance union of the captures' cols then `off_edge.cols`.
    pub cols: Vec<Symbol>,
}

impl EdgeRegister {
    /// The distinct clocks the register keys off, in first-appearance (capture) order.
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

/// The behavioural edge-sensitivity of a cell: its recognised edge registers and the cell-level set of
/// internal level-sensitive master nodes folded away (a cross-coupled pair shares one folded master).
#[derive(Debug, Default)]
pub struct EdgeSensitivity {
    pub registers: Vec<EdgeRegister>,
    pub folded: Vec<Symbol>,
}

/// A single edge's capture observations for one candidate: whether any sample changed the value, and
/// the `(pre-state, post-value)` samples (unchanged clock-toggle samples included).
#[derive(Default, Clone)]
struct CapAgg {
    changed: bool,
    samples: Vec<(Minterm<Symbol>, bool)>,
}

/// The aggregated observations of one candidate node across the whole exploration walk.
#[derive(Default, Clone)]
struct CandAgg {
    /// Per `(data input, clock)`: whether a data-input toggle changed the node in the clock's `(low,
    /// high)` phase. A phase-asymmetric change (transparent in one phase only) is level sensitivity.
    changed_data: BTreeMap<(Symbol, Symbol), (bool, bool)>,
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
        for (k, (f, t)) in other.changed_data {
            let e = self.changed_data.entry(k).or_insert((false, false));
            e.0 |= f;
            e.1 |= t;
        }
        self.changed_clocks.extend(other.changed_clocks);
        for (k, cap) in other.captures {
            let e = self.captures.entry(k).or_default();
            e.changed |= cap.changed;
            e.samples.extend(cap.samples);
        }
        self.stable.extend(other.stable);
    }
}

/// A synthesised register: its per-clock, per-edge captures (each carrying its clock, grouped by clock in
/// input-pin order with Rise first), its off-edge, and whether tier-2 header escalation was needed
/// (tier-2 nodes survive the fold).
type Synthesised = (Vec<(Symbol, Edge, StateRegions)>, StateRegions, bool);

/// The behavioural class of a candidate node.
enum Class {
    Level,
    Register {
        /// The register's keying clocks in input-pin order, each with its active `(rise, fall)` edges.
        clocks: Vec<(Symbol, bool, bool)>,
    },
    None,
}

/// Discover each node's edge sensitivity from the cell's toggle-and-settle behaviour. Read-only over the
/// shared [`Machine`]: it re-walks the exploration and only ADDS an annotation, mirroring
/// [`super::arcs::derive`].
pub fn classify<B: Brand, C: ManagerCell + Send + Sync>(m: &Machine<B, C>) -> EdgeSensitivity {
    // No state variables ⇒ nothing can be a register (and no builder to mint region covers from).
    let Some((_, any_delta)) = m.deltas.first() else {
        return EdgeSensitivity::default();
    };
    let builder = any_delta.builder();

    let cell = m.cell;
    let inputs = &cell.inputs;
    let deltas = &m.deltas;
    let ex = &m.explored;

    // Input classes: a pin declared both clock and async counts async-only.
    let async_set: BTreeSet<&str> = cell.async_pins.iter().map(Symbol::as_str).collect();
    let clock_vec: Vec<Symbol> = cell
        .clock_pins
        .iter()
        .filter(|c| !async_set.contains(c.as_str()))
        .cloned()
        .collect();
    let clock_set: BTreeSet<&str> = clock_vec.iter().map(Symbol::as_str).collect();

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
            let is_async = async_set.contains(related.as_str());
            let is_clock = clock_set.contains(related.as_str());
            let rose = np.value_of(related.as_str()) == Some(true);
            for (i, c) in candidates.iter().enumerate() {
                let (Some(b0), Some(b1)) = (v0[i], value(c, &np)) else {
                    continue;
                };
                if is_async {
                    // Async pins are excluded from the hold discipline (handled via the off-edge
                    // stable-state analysis, not the capture/level classification).
                    continue;
                }
                if is_clock {
                    let cap = out[i].captures.entry((related.clone(), rose)).or_default();
                    cap.samples.push((node.clone(), b1));
                    if b0 != b1 {
                        cap.changed = true;
                        out[i].changed_clocks.insert(related.clone());
                    }
                } else if b0 != b1 {
                    // A data toggle that moved the node: record it per clock phase. Transparency in one
                    // phase but not the other (a phase-asymmetric change) is level sensitivity.
                    for k in &clock_vec {
                        if let Some(ph) = node.value_of(k.as_str()) {
                            let e = out[i]
                                .changed_data
                                .entry((related.clone(), k.clone()))
                                .or_insert((false, false));
                            if ph {
                                e.1 = true;
                            } else {
                                e.0 = true;
                            }
                        }
                    }
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

    // Classify every node BEFORE any synthesis, so the header (which excludes internal level nodes) is
    // settled first. The phase-symmetric permit/block decision consults the EXACT off-edge synthesis
    // (`synth_off_edge` over the non-clock inputs), so it needs the builder and the input header.
    let classes: Vec<Class> = aggs
        .iter()
        .map(|agg| classify_one(&builder, inputs, agg))
        .collect();
    let internal_level: BTreeSet<Symbol> = candidates
        .iter()
        .zip(&classes)
        .filter(|(name, c)| matches!(c, Class::Level) && !output_names.contains(name.as_str()))
        .map(|(name, _)| name.clone())
        .collect();

    let mut registers: Vec<EdgeRegister> = Vec::new();
    // Internal level nodes pulled back into a tier-2 header survive (become unfoldable).
    let mut tier2_kept: BTreeSet<Symbol> = BTreeSet::new();
    // Candidates that actually landed in `registers` (synth_register succeeded). A `Class::Register`
    // candidate whose synthesis returned `None` has no edge annotation, so its RAW function is still
    // emitted and it must be treated as a surviving raw-function emitter, not a register.
    let mut actually_registered: BTreeSet<Symbol> = BTreeSet::new();

    for (i, class) in classes.iter().enumerate() {
        let Class::Register { clocks } = class else {
            continue;
        };
        let name = &candidates[i];
        // Per-clock active edges (input-pin order, Rise first within each clock).
        let clock_edges: Vec<(Symbol, Vec<(bool, Edge)>)> = clocks
            .iter()
            .map(|(clock, rise, fall)| {
                let mut edges: Vec<(bool, Edge)> = Vec::new();
                if *rise {
                    edges.push((true, Edge::Rise));
                }
                if *fall {
                    edges.push((false, Edge::Fall));
                }
                (clock.clone(), edges)
            })
            .collect();

        if let Some((captures, off_edge, tier2)) = synth_register(
            &builder,
            &candidates,
            &internal_level,
            inputs,
            &clock_edges,
            &aggs[i],
        ) {
            if tier2 {
                tier2_kept.extend(internal_level.iter().cloned());
            }
            let cols = register_cols(&captures, &off_edge);
            actually_registered.insert(name.clone());
            registers.push(EdgeRegister {
                node: name.clone(),
                captures,
                off_edge,
                cols,
            });
        }
    }

    // FOLD (cell-level): an internal level master is folded when nothing surviving still references it.
    let ref_reg: BTreeSet<&str> = registers
        .iter()
        .flat_map(|r| r.cols.iter().map(Symbol::as_str))
        .collect();
    // Function support of every candidate, for the surviving-level-signal reference check.
    let mut fn_of: BTreeMap<&str, &Bdd<B, C>> = BTreeMap::new();
    for (n, d) in deltas {
        fn_of.insert(n.as_str(), d);
    }
    for (n, d) in &m.out_deltas {
        fn_of.insert(n.as_str(), d);
    }
    // Every surviving signal whose RAW function is still emitted — level nodes, `Class::None` hysteretic
    // survivors (combinational or ≥2-clock nodes, whose region cols come straight from their function
    // support), AND `Class::Register` candidates whose synthesis failed (no edge annotation, so the raw
    // function is emitted as a fallback). Only candidates that actually landed in `registers` are excluded:
    // their raw function is replaced by the edge seam, so their cols are already accounted for via
    // `ref_reg`. A folded master must not be referenced by ANY node whose raw function is emitted,
    // including registration fallbacks, or the survivor's cols would name a dropped node (statetable
    // invariant I3).
    let survivor_names: Vec<&Symbol> = candidates
        .iter()
        .filter(|n| !actually_registered.contains(*n))
        .collect();

    let folded: Vec<Symbol> = candidates
        .iter()
        .filter(|m| internal_level.contains(*m))
        .filter(|m| {
            // (a) no register capture/off-edge cover references it,
            if ref_reg.contains(m.as_str()) {
                return false;
            }
            // (b) no OTHER surviving signal (level or hysteretic `Class::None`) references it,
            let referenced = survivor_names.iter().any(|l| {
                *l != *m
                    && fn_of
                        .get(l.as_str())
                        .is_some_and(|f| f.variables().any(|v| v.as_str() == m.as_str()))
            });
            if referenced {
                return false;
            }
            // (c) internal (guaranteed by internal_level), (d) not tier-2 re-included.
            !tier2_kept.contains(*m)
        })
        .cloned()
        .collect();

    EdgeSensitivity { registers, folded }
}

/// Classify one candidate from its aggregated observations. The discriminator is level-vs-edge: a
/// register's stable value is independent of its clock's LEVEL — its exact off-edge synthesises cleanly
/// (off-edge phase agreement) — whereas a node that is combinational in a clock's level (or transparent
/// to a data input) is never a register on that clock, regardless of how many clocks change it. The
/// phase-symmetric permit/block decision and the per-clock viability both consult the EXACT off-edge
/// synthesis, so the shared `builder` and the cell `inputs` are threaded in.
fn classify_one<B: Brand, C: ManagerCell>(
    builder: &BddBuilder<B, C>,
    inputs: &[Symbol],
    agg: &CandAgg,
) -> Class {
    // Level: a data input is transparent to the node, against a clock that actually gates it (a clock
    // whose own toggle moves the node). Restricting to those clocks avoids a uniform reset reading as
    // transparent against an unrelated clock it is independent of. Two transparency signatures block a
    // register:
    //   * phase-ASYMMETRIC (`f != t`): the node follows the input during one phase only — the classic
    //     latch signature.
    //   * phase-SYMMETRIC (`f == t`, both phases): the input moves the node in both phases. Whether that
    //     is genuine data transparency or a real set/clear is decided by the EXACT off-edge synthesis
    //     (`synth_off_edge` — the same synthesis the emitter binds to): the input BLOCKS a register only
    //     if it lands purely Held, i.e. it never appears in a Forced projection of the synthesised
    //     off-edge (`off_edge.cols`, the support of the forced set/clear covers). A pure-transparency
    //     input (e.g. XLAT's D, the XOR of two opposite-phase latches, forcing the node to no constant)
    //     drops out of `off_edge.cols` and blocks: were it permitted the node would key off the single
    //     clock and drop the data input, emitting a self-contradictory register that ALSO carries
    //     combinational data arcs. A phase-symmetric FORCER — an async-style set/clear that lands
    //     Forced0/Forced1, single-literal, disjunctive `R+G` OR conjunctive `R*G` (GATEDR's gated clear)
    //     alike — appears in `off_edge.cols` and is permitted: it is real set/clear, and the node stays a
    //     register. Grounding in the exact per-projection synthesis (which marks Forced only where the
    //     observed samples of a projection actually agree to a constant) also avoids the marginal test's
    //     mirror risk of KEEPING a register on an unreachable/partial projection.
    let mut off_edge_cache: BTreeMap<Symbol, Option<StateRegions>> = BTreeMap::new();
    let level = agg.changed_data.iter().any(|((d, k), (f, t))| {
        if !agg.changed_clocks.contains(k) {
            return false;
        }
        if f != t {
            return true; // phase-asymmetric ⇒ the classic latch signature
        }
        // Phase-symmetric: permit iff `d` participates in a Forced projection of the exact off-edge.
        let off = off_edge_cache.entry(k.clone()).or_insert_with(|| {
            let header_off: Vec<Symbol> = inputs
                .iter()
                .filter(|p| p.as_str() != k.as_str())
                .cloned()
                .collect();
            synth_off_edge(builder, &header_off, std::slice::from_ref(k), &agg.stable)
        });
        match off {
            // `d` lands in a forced set/clear cover ⇒ real set/clear ⇒ permitted (not level).
            Some(sr) => !sr.cols.iter().any(|c| c == d),
            // No clean off-edge synthesis (phase disagreement) ⇒ forcing cannot be confirmed ⇒ block.
            None => true,
        }
    });
    if level {
        return Class::Level;
    }
    // Per-clock viability, reusing the level check's lazily-filled off-edge cache. A changed clock `k` is
    // a viable register key iff its EXACT off-edge synthesises cleanly (off-edge phase AGREEMENT ⇒ the
    // node's stable value is independent of `k`'s level, not combinational in it) AND every OTHER changed
    // clock `j` is a level forcer carried by `k`'s off-edge (`j` appears in `off_edge.cols`). A node that
    // is combinational in `k`'s level yields no clean off-edge (phase disagreement) and is never a
    // register on `k`, whatever the clock count.
    let viable: Vec<Symbol> = agg
        .changed_clocks
        .iter()
        .filter(|k| {
            let off = off_edge_cache.entry((*k).clone()).or_insert_with(|| {
                let header_off: Vec<Symbol> = inputs
                    .iter()
                    .filter(|p| p.as_str() != k.as_str())
                    .cloned()
                    .collect();
                synth_off_edge(builder, &header_off, std::slice::from_ref(*k), &agg.stable)
            });
            match off {
                Some(sr) => agg
                    .changed_clocks
                    .iter()
                    .filter(|j| j.as_str() != k.as_str())
                    .all(|j| sr.cols.iter().any(|c| c == j)),
                None => false,
            }
        })
        .cloned()
        .collect();

    match viable.len() {
        1 => {
            let clock = viable.into_iter().next().unwrap();
            let rise = agg
                .captures
                .get(&(clock.clone(), true))
                .is_some_and(|c| c.changed);
            let fall = agg
                .captures
                .get(&(clock.clone(), false))
                .is_some_and(|c| c.changed);
            if !rise && !fall {
                return Class::None;
            }
            Class::Register {
                clocks: vec![(clock, rise, fall)],
            }
        }
        0 => Class::None,
        // TODO(scope-a wave 3): recognise multi-clock via the joint phase-vector off-edge
        _ => Class::None,
    }
}

/// Synthesise a register's captures and off-edge for a candidate, escalating tier-1 → tier-2 on a
/// capture conflict. Returns `None` (fall back to level, no annotation) when a tier-2 capture still
/// conflicts or the off-edge phases disagree (behavioural F2). The `bool` is whether tier-2 was used.
#[allow(clippy::too_many_arguments)]
fn synth_register<B: Brand, C: ManagerCell>(
    builder: &BddBuilder<B, C>,
    candidates: &[Symbol],
    internal_level: &BTreeSet<Symbol>,
    inputs: &[Symbol],
    clock_edges: &[(Symbol, Vec<(bool, Edge)>)],
    agg: &CandAgg,
) -> Option<Synthesised> {
    let clocks: Vec<Symbol> = clock_edges.iter().map(|(c, _)| c.clone()).collect();
    let clock_set: BTreeSet<&str> = clocks.iter().map(Symbol::as_str).collect();

    for tier2 in [false, true] {
        // Capture per clock (input-pin order), per active edge (Rise first). Each capture's header is the
        // inputs minus THAT capture's clock, then the candidate signal names; internal level nodes are
        // excluded at tier-1 and re-included at tier-2. The candidate's own name is always present (a
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
                        .filter(|c| tier2 || !internal_level.contains(*c))
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
        if conflict {
            if tier2 {
                return None; // a tier-2 conflict falls back to level
            }
            continue; // escalate to tier-2
        }

        // Off-edge over the inputs minus ALL the register's clocks: the hold-and-async-set/clear behaviour
        // is input driven, so the state coordinates are not columns (the value held is the register's own,
        // absent from the header, and any forcing comes from an async input). A data input that never
        // forces simply lands every projection in `hold` and drops out of the cols.
        let header_off: Vec<Symbol> = inputs
            .iter()
            .filter(|p| !clock_set.contains(p.as_str()))
            .cloned()
            .collect();
        let off_edge = synth_off_edge(builder, &header_off, &clocks, &agg.stable)?;

        return Some((captures, off_edge, tier2));
    }
    None
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

/// Synthesise the off-edge (hold + async set/clear) region from the stable-state samples over
/// `header_off`, for the register's clock SET. Each stable sample is keyed by the PHASE VECTOR of
/// `clocks` (a sample with any of those clocks unset is skipped — generalising the single-clock
/// unset-skip). Within a projection every observed phase vector is phase-classified and ALL classes must
/// AGREE, else the projection blocks the whole annotation (behavioural F2 — level fallback) and `None` is
/// returned. The agreed Forced class gives the async set/clear cover; agreeing held (and unobserved)
/// projections default to hold. For a single clock the two phase vectors `[false]`/`[true]` are exactly
/// today's `(low, high)` split, so this reduces byte-identically.
fn synth_off_edge<B: Brand, C: ManagerCell>(
    builder: &BddBuilder<B, C>,
    header_off: &[Symbol],
    clocks: &[Symbol],
    stable: &[(Minterm<Symbol>, bool)],
) -> Option<StateRegions> {
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
        // Every observed phase vector's class must agree; a disagreement blocks the annotation.
        let mut agreed: Option<Phase> = None;
        for vals in phases.values() {
            let Some(cls) = phase_class(vals) else {
                continue;
            };
            match agreed {
                None => agreed = Some(cls),
                Some(prev) if prev != cls => return None, // phase disagreement blocks the annotation
                Some(_) => {}
            }
        }
        let cube = cube_bdd(builder, proj);
        match agreed {
            Some(Phase::Forced1) => on_pts = on_pts.or(&cube),
            Some(Phase::Forced0) => off_pts = off_pts.or(&cube),
            _ => {} // held or unobserved ⇒ hold
        }
    }

    let hold = !&on_pts.or(&off_pts);
    Some(regions_from(&on_pts, &off_pts, &hold, header_off))
}

/// The register's column set: the first-appearance union of every capture's cols then the off-edge's
/// cols.
fn register_cols(
    captures: &[(Symbol, Edge, StateRegions)],
    off_edge: &StateRegions,
) -> Vec<Symbol> {
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

    fn cols_of(sr: &StateRegions) -> Vec<&str> {
        sr.cols.iter().map(Symbol::as_str).collect()
    }

    fn clocks_of(er: &EdgeRegister) -> Vec<&str> {
        er.clocks().into_iter().map(Symbol::as_str).collect()
    }

    fn reg<'a>(es: &'a EdgeSensitivity, node: &str) -> &'a EdgeRegister {
        es.registers
            .iter()
            .find(|r| r.node.as_str() == node)
            .unwrap_or_else(|| panic!("no register for {node}: {:?}", node_list(es)))
    }

    fn node_list(es: &EdgeSensitivity) -> Vec<&str> {
        es.registers.iter().map(|r| r.node.as_str()).collect()
    }

    fn folded_list(es: &EdgeSensitivity) -> Vec<&str> {
        es.folded.iter().map(Symbol::as_str).collect()
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
            let mut nodes = node_list(&es);
            nodes.sort();
            assert_eq!(nodes, ["enA", "enB", "sela2", "selb2"]);
            let s2 = reg(&es, "sela2");
            assert_eq!(clocks_of(s2), ["CLKA"]);
            assert_eq!(s2.captures[0].1, Edge::Rise);
            let ena = reg(&es, "enA");
            assert_eq!(ena.captures[0].1, Edge::Fall);
            assert_eq!(clocks_of(ena), ["CLKA"]);
            let sb2 = reg(&es, "selb2");
            assert_eq!(sb2.captures[0].1, Edge::Rise);
            let enb = reg(&es, "enB");
            assert_eq!(enb.captures[0].1, Edge::Fall);
            // sela2's capture must not reference the folded sela1.
            assert!(!s2.captures[0].2.cols.iter().any(|c| c == "sela1"));
            let folded = folded_list(&es);
            assert!(folded.contains(&"sela1"), "sela1 folded, got {folded:?}");
            assert!(folded.contains(&"selb1"), "selb1 folded, got {folded:?}");
            assert!(!node_list(&es).contains(&"GCLK"), "GCLK is not a register");
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
            (MASTER_ONLY_RESET_TOML, "Q"),
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
    fn edge_master_only_reset_async_still_blocks() {
        // R clears Q only while CLK=1 ⇒ phase disagreement ⇒ no annotation.
        with_machine!(MOR_ASYNC_TOML, |_b, _a, _m2, m| {
            let es = classify(&m);
            assert!(
                !node_list(&es).contains(&"Q"),
                "phase-split reset must block: {:?}",
                node_list(&es)
            );
        });
    }

    #[test]
    fn edge_both_latch_reset_recognised_with_async_off() {
        // R clears both latches ⇒ phase agreement ⇒ Q recognised, off_edge.off covers R.
        with_machine!(BOTH_RESET_TOML, |builder, _a, _m2, m| {
            let es = classify(&m);
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
            assert!(es.registers.is_empty(), "no register: {:?}", node_list(&es));
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
        // R is declared a clock alongside CLK, but only CLK's off-edge synthesises cleanly (R's own
        // off-edge disagrees phase-wise) ⇒ Q is a Rise register keyed on CLK, R landing as its async clear.
        with_machine!(RDFF_TOML, |builder, _a, _m2, m| {
            let es = classify(&m);
            let q = reg(&es, "Q");
            assert_eq!(clocks_of(q), ["CLK"]);
            assert_eq!(q.captures[0].1, Edge::Rise);
            let off = builder.build_cover(&q.off_edge.off_cover);
            let r = builder.var("R");
            assert!(off.equivalent_to(&r), "off_edge.off must cover R");
            let cols = q.cols.iter().map(Symbol::as_str).collect::<Vec<_>>();
            for c in ["D", "R"] {
                assert!(cols.contains(&c), "col {c} missing from {cols:?}");
            }
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
    fn edge_master_only_reset_async_protects_master_from_fold() {
        // Q's registration fails (see edge_master_only_reset_async_still_blocks), so Q's raw function is
        // emitted and still references the master M ⇒ M must not be folded away.
        with_machine!(MOR_ASYNC_TOML, |_b, _a, _m2, m| {
            let es = classify(&m);
            assert!(
                !folded_list(&es).contains(&"M"),
                "failed-registration master M must survive, folded={:?}",
                folded_list(&es)
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
            "wide cell trips the guard ⇒ default EdgeSensitivity"
        );
        assert!(EdgeSensitivity::default().registers.is_empty());
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
            assert!(off.edge.registers.is_empty());
            assert!(!on.edge.registers.is_empty());
        }
    }
}
