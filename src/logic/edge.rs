//! Behavioural edge-sensitivity classification.
//!
//! This is the **behavioural** successor to the structural [`super::collapse`] pass: rather than
//! nominating a master-slave latch pair structurally, it discovers per node — outputs **and** internal
//! state variables — whether that node is edge-triggered (and on which clock edge/edges), level-
//! sensitive, or combinational, purely from the cell's already-explored toggle-and-settle behaviour. It
//! is a strict superset of the structural detector by construction: every latch topology that settles
//! into an edge seam is recognised, inverting captures (`capture = !D`) included and recorded verbatim —
//! inversion is never special-cased.
//!
//! [`classify`] is a **post-exploration** read-only pass over the shared [`Machine`]: it re-walks the
//! exploration ([`Machine::explored`]) with [`machine::toggle`]/[`machine::settle`], exactly mirroring
//! [`super::arcs::derive`]'s per-node walk, and only ADDS an edge-sensitivity annotation. It never
//! re-derives the exploration, the prevectors or the hazards — those stay byte-identical.
//!
//! # Classification
//!
//! Each candidate node's observations are aggregated across the walk, then classified:
//!
//! * **level** — some *data* input is transparent to the node in one clock phase but not the other (the
//!   node follows an input during a phase). A level node emits its ordinary hysteretic regions and takes
//!   no annotation; an internal level node is a foldable master.
//! * **register** — exactly one declared clock's edge(s) change the node, and no data input is
//!   transparent to it. The node captures a next-state value at each active edge and holds otherwise.
//! * **none** — combinational, or changed by two or more distinct clocks: no annotation.
//!
//! The capture (per active edge) and the off-edge (hold + async set/clear) functions are synthesised
//! from the sampled pre-states and stable states over a deterministic two-tier header, reusing the
//! [`super::regions`] region pipeline so the emitted cubes are byte-compatible with the structural pass.

use std::collections::{BTreeMap, BTreeSet};

use espresso_logic::bdd::{Bdd, BddBuilder, Brand, ManagerCell};
use espresso_logic::{Cover, CoverType, CubeType, Minimizable, Minterm, Symbol};
use rayon::prelude::*;

use crate::logic::analysis::Machine;
use crate::logic::arcs::Edge;
use crate::logic::machine;
use crate::logic::regions::{self, StateRegions};

/// One recognised edge-triggered register: a node re-expressed as an edge seam on `clock`.
#[derive(Debug, Clone)]
pub struct EdgeRegister {
    /// The node that becomes the register's output coordinate.
    pub node: Symbol,
    /// The declared clock the node keys off.
    pub clock: Symbol,
    /// The captured next-state function per active edge, as combinational state-table regions (total —
    /// off is the complement of on, empty hold). One entry for a single-edge register, two for a
    /// dual-edge register with `Rise` first.
    pub captures: Vec<(Edge, StateRegions)>,
    /// The off-edge (hold) function as state-table regions: on/off are the async set/clear covers, hold
    /// is the quiescent region; never references `clock`.
    pub off_edge: StateRegions,
    /// The register's column set: the first-appearance union of the captures' cols then `off_edge.cols`.
    pub cols: Vec<Symbol>,
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

/// A synthesised register: its per-edge captures (Rise first), its off-edge, and whether tier-2 header
/// escalation was needed (tier-2 nodes survive the fold).
type Synthesised = (Vec<(Edge, StateRegions)>, StateRegions, bool);

/// The behavioural class of a candidate node.
enum Class {
    Level,
    Register {
        clock: Symbol,
        rise: bool,
        fall: bool,
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
    // settled first.
    let classes: Vec<Class> = aggs.iter().map(classify_one).collect();
    let internal_level: BTreeSet<Symbol> = candidates
        .iter()
        .zip(&classes)
        .filter(|(name, c)| matches!(c, Class::Level) && !output_names.contains(name.as_str()))
        .map(|(name, _)| name.clone())
        .collect();

    let mut registers: Vec<EdgeRegister> = Vec::new();
    // Internal level nodes pulled back into a tier-2 header survive (become unfoldable).
    let mut tier2_kept: BTreeSet<Symbol> = BTreeSet::new();

    for (i, class) in classes.iter().enumerate() {
        let Class::Register { clock, rise, fall } = class else {
            continue;
        };
        let name = &candidates[i];
        // Active edges, Rise first.
        let mut edges: Vec<(bool, Edge)> = Vec::new();
        if *rise {
            edges.push((true, Edge::Rise));
        }
        if *fall {
            edges.push((false, Edge::Fall));
        }

        if let Some((captures, off_edge, tier2)) = synth_register(
            &builder,
            &candidates,
            &internal_level,
            inputs,
            clock,
            &edges,
            &aggs[i],
        ) {
            if tier2 {
                tier2_kept.extend(internal_level.iter().cloned());
            }
            let cols = register_cols(&captures, &off_edge);
            registers.push(EdgeRegister {
                node: name.clone(),
                clock: clock.clone(),
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
    let level_names: Vec<&Symbol> = candidates
        .iter()
        .zip(&classes)
        .filter(|(_, c)| matches!(c, Class::Level))
        .map(|(n, _)| n)
        .collect();

    let folded: Vec<Symbol> = candidates
        .iter()
        .filter(|m| internal_level.contains(*m))
        .filter(|m| {
            // (a) no register capture/off-edge cover references it,
            if ref_reg.contains(m.as_str()) {
                return false;
            }
            // (b) no OTHER surviving level signal references it,
            let referenced = level_names.iter().any(|l| {
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

/// Classify one candidate from its aggregated observations.
fn classify_one(agg: &CandAgg) -> Class {
    // Level: a data input is transparent to the node in one phase of a clock that actually gates it (a
    // clock whose own toggle moves the node) but not the other. Restricting to those clocks avoids a
    // uniform reset reading as transparent against an unrelated clock it is independent of.
    let level = agg
        .changed_data
        .iter()
        .any(|((_, k), (f, t))| f != t && agg.changed_clocks.contains(k));
    if level {
        return Class::Level;
    }
    // A register keys off exactly one clock; two or more distinct clocks ⇒ no annotation.
    if agg.changed_clocks.len() != 1 {
        return Class::None;
    }
    let clock = agg.changed_clocks.iter().next().unwrap().clone();
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
    Class::Register { clock, rise, fall }
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
    clock: &Symbol,
    edges: &[(bool, Edge)],
    agg: &CandAgg,
) -> Option<Synthesised> {
    for tier2 in [false, true] {
        // Header: inputs (minus the clock) then the candidate signal names; internal level nodes are
        // excluded at tier-1 and re-included at tier-2. The candidate's own name is always present (a
        // toggle flop captures a function of its own prior state).
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

        // Capture per active edge.
        let mut captures: Vec<(Edge, StateRegions)> = Vec::new();
        let mut conflict = false;
        for (is_rise, edge) in edges {
            let samples = agg
                .captures
                .get(&(clock.clone(), *is_rise))
                .map(|c| c.samples.as_slice())
                .unwrap_or(&[]);
            match synth_capture(builder, &header, samples) {
                Some(sr) => captures.push((*edge, sr)),
                None => {
                    conflict = true;
                    break;
                }
            }
        }
        if conflict {
            if tier2 {
                return None; // a tier-2 conflict falls back to level
            }
            continue; // escalate to tier-2
        }

        // Off-edge over the non-clock inputs: the hold-and-async-set/clear behaviour is input driven, so
        // the state coordinates are not columns (the value held is the register's own, absent from the
        // header, and any forcing comes from an async input). A data input that never forces simply lands
        // every projection in `hold` and drops out of the cols.
        let header_off: Vec<Symbol> = inputs
            .iter()
            .filter(|p| p.as_str() != clock.as_str())
            .cloned()
            .collect();
        let off_edge = synth_off_edge(builder, &header_off, clock, &agg.stable)?;

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
/// `header_off`. Each projection is classified per clock phase; a projection whose classification
/// differs between the two phases blocks the whole annotation (behavioural F2 — level fallback), so
/// `None` is returned. Forced projections give the async set/clear covers; agreeing held (and
/// unobserved) projections default to hold.
fn synth_off_edge<B: Brand, C: ManagerCell>(
    builder: &BddBuilder<B, C>,
    header_off: &[Symbol],
    clock: &Symbol,
    stable: &[(Minterm<Symbol>, bool)],
) -> Option<StateRegions> {
    // Group the stable samples by projection, split into the clock's two phases.
    let mut groups: BTreeMap<Minterm<Symbol>, (Vec<bool>, Vec<bool>)> = BTreeMap::new();
    for (state, val) in stable {
        let Some(ph) = state.value_of(clock.as_str()) else {
            continue;
        };
        let proj = state.project_to(header_off.iter().map(Symbol::as_str));
        let g = groups.entry(proj).or_default();
        if ph {
            g.1.push(*val);
        } else {
            g.0.push(*val);
        }
    }

    let mut on_pts = builder.constant(false);
    let mut off_pts = builder.constant(false);
    for (proj, (low, high)) in &groups {
        let cl = phase_class(low);
        let ch = phase_class(high);
        if let (Some(a), Some(b)) = (cl, ch) {
            if a != b {
                return None; // phase disagreement blocks the annotation
            }
        }
        let cube = cube_bdd(builder, proj);
        match cl.or(ch) {
            Some(Phase::Forced1) => on_pts = on_pts.or(&cube),
            Some(Phase::Forced0) => off_pts = off_pts.or(&cube),
            _ => {} // held or unobserved ⇒ hold
        }
    }

    let hold = !&on_pts.or(&off_pts);
    Some(regions_from(&on_pts, &off_pts, &hold, header_off))
}

/// The register's column set: the first-appearance union of every capture's cols then the off-edge's
/// cols (mirrors `collapse.rs`'s `union_cols`).
fn register_cols(captures: &[(Edge, StateRegions)], off_edge: &StateRegions) -> Vec<Symbol> {
    let mut cols: Vec<Symbol> = Vec::new();
    let sources = captures
        .iter()
        .map(|(_, sr)| &sr.cols)
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
    use crate::logic::collapse::{recognise_edge_registers, EdgeRegister as StructReg};
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

    // --- fixtures (collapse.rs re-encoded as single-cell TOML) ---

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
            assert_eq!(q.clock, "CLK");
            assert_eq!(q.captures.len(), 1);
            let (edge, cap) = &q.captures[0];
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
            assert_eq!(s2.clock, "CLKA");
            assert_eq!(s2.captures[0].0, Edge::Rise);
            let ena = reg(&es, "enA");
            assert_eq!(ena.captures[0].0, Edge::Fall);
            assert_eq!(ena.clock, "CLKA");
            let sb2 = reg(&es, "selb2");
            assert_eq!(sb2.captures[0].0, Edge::Rise);
            let enb = reg(&es, "enB");
            assert_eq!(enb.captures[0].0, Edge::Fall);
            // sela2's capture must not reference the folded sela1.
            assert!(!s2.captures[0].1.cols.iter().any(|c| c == "sela1"));
            let folded = folded_list(&es);
            assert!(folded.contains(&"sela1"), "sela1 folded, got {folded:?}");
            assert!(folded.contains(&"selb1"), "selb1 folded, got {folded:?}");
            assert!(!node_list(&es).contains(&"GCLK"), "GCLK is not a register");
        });
    }

    // === Fixtures: collapse.rs stay-level cases re-encoded as single-cell TOML ===

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

    // === Step 2: executable superset gate ===

    /// Rebuild a region's minimised on/off/hold covers as BDDs in the shared builder and compare them by
    /// `equivalent_to` MASKED to the reachable state set: the structural and behavioural covers must agree
    /// on every reachable state. Full-space equality would over-constrain a shared-boundary register whose
    /// behavioural capture legitimately drops a syntactically-present-but-behaviourally-redundant literal
    /// (e.g. ICM's `enA` captures `sela2`, the reachable-equal simplification of the structural
    /// `!RA*sela2` — `RA*sela2` is unreachable). Reachable-masked equality is the true strict-superset /
    /// same-behaviour gate.
    macro_rules! assert_regions_equiv {
        ($builder:expr, $reach:expr, $a:expr, $b:expr, $ctx:expr) => {{
            let a = $a;
            let b = $b;
            for (region, ca, cb) in [
                ("on", &a.on_cover, &b.on_cover),
                ("off", &a.off_cover, &b.off_cover),
                ("hold", &a.hold_cover, &b.hold_cover),
            ] {
                let la = $builder.build_cover(ca).and($reach);
                let lb = $builder.build_cover(cb).and($reach);
                assert!(
                    la.equivalent_to(&lb),
                    "{} region {} differs over reachable states",
                    $ctx,
                    region,
                );
            }
        }};
    }

    fn differential(src: &str) {
        with_machine!(src, |builder, analysed, bdds, m| {
            let order: Vec<Symbol> = analysed.signals().map(|s| s.name.clone()).collect();
            let output_set: BTreeSet<Symbol> =
                analysed.outputs.iter().map(|o| o.name.clone()).collect();
            let clocks = analysed.clock_pins.clone();
            let structural: Vec<StructReg> =
                recognise_edge_registers(&bdds, &order, &output_set, &clocks);
            let es = classify(&m);
            // The reachable state set as a BDD (OR of every explored stable-state cube), for the masked
            // region comparison.
            let mut reach = builder.constant(false);
            for state in &m.explored.order {
                reach = reach.or(&super::cube_bdd(&builder, state));
            }
            for sr in &structural {
                let br = es
                    .registers
                    .iter()
                    .find(|r| r.node == sr.node && r.clock == sr.clock)
                    .unwrap_or_else(|| {
                        panic!("no behavioural register for structural {:?}", sr.node)
                    });
                let bcap = br
                    .captures
                    .iter()
                    .find(|(e, _)| *e == sr.edge)
                    .map(|(_, c)| c)
                    .unwrap_or_else(|| {
                        panic!("behavioural {:?} lacks edge {:?}", sr.node, sr.edge)
                    });
                assert_regions_equiv!(
                    builder,
                    &reach,
                    &sr.capture,
                    bcap,
                    format!("{} capture", sr.node)
                );
                assert_regions_equiv!(
                    builder,
                    &reach,
                    &sr.off_edge,
                    &br.off_edge,
                    format!("{} off_edge", sr.node)
                );
                if let Some(fm) = &sr.folded_master {
                    assert!(
                        es.folded.contains(fm),
                        "structural folded master {fm:?} missing from behavioural folded {:?}",
                        folded_list(&es)
                    );
                }
            }
        });
    }

    #[test]
    fn differential_superset_over_every_fixture() {
        for src in [
            DFF_TOML,
            ICM_TOML,
            DLAT_TOML,
            GLAT_TOML,
            MUX_TWO_CLOCK_TOML,
            EXPOSED_MASTER_TOML,
            TAPPED_MASTER_TOML,
            INVERTING_DFF_TOML,
            MASTER_ONLY_RESET_TOML,
            UCDFF_TOML,
        ] {
            differential(src);
        }
    }

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
            let (edge, cap) = &q.captures[0];
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
            assert_eq!(q.captures[0].0, Edge::Rise);
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
            let on = builder.build_cover(&q.captures[0].1.on_cover).and(&reach);
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
            assert_eq!(q.captures[0].0, Edge::Rise);
            assert_eq!(mm.captures[0].0, Edge::Fall);
            assert!(
                q.captures[0].1.cols.iter().any(|c| c == "M"),
                "Q captures the master M (ring), cols {:?}",
                cols_of(&q.captures[0].1)
            );
            // M's falling capture is self-inverting: at the pre-fall (CLK=1) states M equals Q, so
            // capturing !M is capturing !Q — recorded verbatim as `!R*!M`, no special-casing of inversion.
            let mcap = &mm.captures[0].1;
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
            assert_eq!(q.captures[0].0, Edge::Rise);
            assert_eq!(qn.captures[0].0, Edge::Rise);
            let q_on = builder.build_cover(&q.captures[0].1.on_cover);
            let qn_on = builder.build_cover(&qn.captures[0].1.on_cover);
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
            assert_eq!(q.captures[0].0, Edge::Rise);
            assert_eq!(q.captures[1].0, Edge::Fall);
            for (_, cap) in &q.captures {
                let on = builder.build_cover(&cap.on_cover);
                assert!(on.equivalent_to(&builder.var("D")), "each edge captures D");
            }
            let mut folded = folded_list(&es);
            folded.sort();
            assert_eq!(folded, ["L1", "L2"]);
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
}
