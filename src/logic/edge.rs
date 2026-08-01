//! Behavioural per-arc edge classification.
//!
//! Timing arcs are NOT derived here. Every arc the cell has already came out of [`super::arcs::derive`],
//! which exists wherever a single-input toggle between reachable stable states changes an output. This
//! module only attaches a LABEL to the clock-related ones. Labelling is PER ARC: an output pin's arcs are
//! each typed independently, so edge and combinational arcs coexist freely on one output (an async-reset
//! flop carries both).
//!
//! # The definition
//!
//! **A clock toggle that takes a latch from opaque to transparent, and whose resulting output value
//! DEPENDS ON LATCH CONTENT rather than arriving regardless, is an EDGE ARC on that output.** An edge arc
//! emits Liberate `-type edge` (see [`crate::emit::arcs_tcl`]). An
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
//! its effect being classified from its own observed moves (`forcing_pins`). The characterisation is
//! consequently IMPLEMENTATION-STYLE INVARIANT: the NAND-implemented `NDLAT` / `NDFF` / `NHPIPE`
//! fixtures characterise identically to their pass-transistor twins `DLAT` / `DFF` / `HPIPE`.
//!
//! [`classify`] is a **post-exploration** read-only pass over the shared [`Machine`]: it re-walks the
//! exploration with [`machine::toggle`]/[`machine::settle`], mirroring [`super::arcs::derive`]'s
//! per-node walk, and only ADDS an edge annotation. It never re-derives the exploration, the
//! prevectors or the hazards — those stay byte-identical whether the annotation is on or off.
//!
//! # The mechanism
//!
//! Only FULLY-DETERMINATE reachable states take part in ARC MEASUREMENT and typing — a state with a
//! don't-care (uninitialised) state column is arc-INELIGIBLE (a don't-care is a MISSING variable, never
//! coerced to 0/1, in the `Minterm` and in BDD evaluation alike). Traversal is untouched: partial states
//! remain seeds, they are simply not measured from. (`forcing_pins`'s constant-pinning scan is a
//! separate behavioural classification, not an arc measurement: it ranges over every reachable stable
//! state, reading a node only where that node's own value is determinate there.) NO machine state is ever
//! coerced, defaulted or re-settled under a held value — an oscillating configuration is an invalid state
//! and never participates in any test; everything is read off the combinational stable-state machinery
//! [`machine::explore`]/[`machine::settle`] already produce.
//!
//! The pipeline is one analysis over the machine's `toggle`/`settle` observations:
//!
//! 1. **Arc typing — GENERATION and PROPAGATION**, per arc at full identity
//!    `(output, related, direction, machine start minterm)`, gated by vacuity (only a firing that changed
//!    the output) and by a GENERATOR in the output's cone (the state variables `δ_output` depends on).
//!    * **Generation** — a latch goes OPAQUE→TRANSPARENT across the clock's edge. Opacity is read from the
//!      live dependency loops at the eligible stable states: at a stable state a dependency edge `n → m`
//!      between state variables is LIVE iff `δ_m`, restricted ([`Bdd::restrict_to`]) to all of its support
//!      EXCEPT `n` at the state's values, still depends on `n` (the residual is non-constant in `n`); a
//!      latch is OPAQUE in a phase iff some eligible stable state of that phase carries a live dependency
//!      cycle through it, TRANSPARENT iff none does. A latch GENERATES on `(K, d)` iff it is opaque at the
//!      source level and transparent at the delivered level. Generation at the output itself types the arc.
//!    * **Propagation** — restriction-survival from the arc's SOURCE: restrict `δ_output` on all of its
//!      support MINUS the source latch `W` to the post-arc stable state's values; the arc propagates iff
//!      the residual still depends on `W`. Candidate sources are the K-associated latches in the output's
//!      cone (generator and closer); applied PER ARC (each firing restricts to its own post-state) and
//!      transitively along the dependency chain.
//! 2. **The seam set `S`** — per candidate node (every output and internal state variable), the
//!    `(clock, direction)` toggles on which the node carries an edge SEAM: the typing holds AND the
//!    delivered value HOLDS through the phase, the last a greatest fixpoint (`seam_fixpoint`) that removes
//!    `(K, d)` when a non-forcing change of the node inside its delivered phase occurs at a toggle not
//!    itself an edge of `S`. A node with a non-empty `S` is an edge register; its per-edge next-state
//!    functions and off-edge are synthesised into [`EdgeArcs::captures`].
//! 3. **Cover synthesis** — `synth_capture`, `generalise` and `regions_from` over one uniform
//!    header (all inputs except the keying clock plus every candidate), with an ordered drop-loop that
//!    prefers inputs over internals so the fold-eligible internals drop out of the cover.
//! 4. **Fold** — internal non-seam nodes fold away as an emission-time reachability fixpoint.
//!
//! The capture and off-edge functions are recorded verbatim as ordinary functions — an inverting flop's
//! next state is simply `!D`, never special-cased.
//!
//! See `docs/edge-collapse.md` for the concept-first walkthrough.

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use espresso_logic::bdd::{Bdd, BddBuilder, Brand, ManagerCell};
use espresso_logic::{Anonymous, Cover, CoverType, CubeType, Minimizable, Minterm, Symbol};
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
    /// active edge (two for a dual-edge node with `Rise` first).
    pub captures: Vec<(Symbol, Edge, StateRegions)>,
    /// The off-edge (hold) function as state-table regions, keyed by the clock set's phase vector: on/off
    /// are the set/clear covers, hold is the quiescent region. A phase-agreed forcing drops the clocks
    /// from its cover; a phase-conditioned one keeps its gating clock literal (`CLK*R`).
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
/// An EDGE arc is a clock toggle that takes a latch from opaque to transparent and whose resulting output
/// value depends on latch content. A node that additionally HOLDS its
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
    /// `(output, clock, direction)` that differ only in internal state can type differently. Every reader
    /// asks the set whether an identity is a member, so it is a [`HashSet`].
    pub labels: HashSet<(Symbol, Symbol, Edge, Minterm<Symbol>)>,
    /// The read-gate factorisations recognised across the cell's outputs (see [`DerivedRegister`]). Empty
    /// for every cell that carries no read-gated register output. Each entry names a state-holding register
    /// the emitters render as a first-class internal node, and carries the combinational read function(s)
    /// of the output(s) that read it through a gate.
    pub derived: Vec<DerivedRegister>,
}

/// A post-processing derived internal edge register: content is a function of already-explored state,
/// never a new state variable — nothing is re-explored and no machine field is touched. It is minted only
/// for a READ-GATED register output — one whose forcing pin READS the held state without changing it
/// (`BDET`'s output-enable `A`), as opposed to one that CHANGES the state (`RDFF`'s reset `R`). Folding
/// such an output's master into the output would destroy the content the output re-acquires when the gate
/// releases; instead the state-holding register is factored out as its own node (`Y_st`) with native edge
/// capture, and the output becomes a combinational [`state_function`](crate::emit::liberty) over it.
///
/// The register additionally carries an ordinary [`EdgeCaptures`] entry on [`EdgeArcs::captures`] (its
/// captures are the output's already-synthesised covers cofactored at the read-gates' pass levels), so the
/// entire downstream edge-row / UDP machinery — which is name-driven — flows through unchanged. When the
/// factored content matches an ALREADY-DECLARED register (up to inversion — `DETP`'s buried `T`), that
/// register is reused and nothing is minted; the entry then only records the reading output's function.
#[derive(Debug, Clone)]
pub struct DerivedRegister {
    /// The register node's name — a freshly minted, collision-checked name, or the name of the reused
    /// declared register when a match was found (nothing minted).
    pub name: Symbol,
    /// The register's value over machine coordinates, evaluable at any explored stable state — the harness
    /// resolves the derived node's value through this instead of `Machine::output_value`.
    pub content: Cover<Symbol, Anonymous>,
    /// Per read-gated output that reads this register: the output's combinational read function, as
    /// state-table regions over the register node and the gate pins.
    pub reads: Vec<(Symbol, StateRegions)>,
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

/// One candidate edge arc on a node: `(clock, is_rise)`. The decision core's whole currency — each arc on
/// a node is typed independently, so edge and combinational arcs coexist freely on one output.
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

    // The scan context: the transition table, the per-state arc eligibility and the declared clock set,
    // all node-independent, built once and indexed into by every scan below.
    let scan = Scan::new(m);

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

    // The observation walk over the ELIGIBLE reachable stable states, mirroring `arcs::derive`'s per-node
    // walk. Each node toggles one input at a time, settles, and records the candidate values before/after.
    // The walk produces plain data (minterms); no BDD is built here.
    let per_node = |node: &Minterm<Symbol>| -> Vec<CandAgg> {
        let mut out: Vec<CandAgg> = vec![CandAgg::default(); candidates.len()];
        if !scan.is_eligible(node) {
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
            let is_clock = scan.clock_set.contains(related.as_str());
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

    // The raw explored order — `per_node` gates on eligibility itself, contributing nothing from an
    // ineligible state. The walk keeps every state so its index space stays aligned with `scan.next`
    // and `scan.eligible`, which every later scan indexes by state position.
    let aggs: Vec<CandAgg> = ex.order.par_iter().map(per_node).reduce(
        || vec![CandAgg::default(); candidates.len()],
        |mut a, b| {
            for (ai, bi) in a.iter_mut().zip(b) {
                ai.merge(bi);
            }
            a
        },
    );

    // Each candidate's forcing pins. Computed BEFORE any synthesis, so the seam set — which decides the
    // fold-eligible internals the drop-loop prefers to shed — is settled first. Forcing plays no part in
    // typing; it is consumed only by off-edge synthesis and the seam fixpoint's forcing exemption.
    let forcing_of: Vec<BTreeMap<Symbol, (bool, bool)>> = candidates
        .iter()
        .zip(&aggs)
        .map(|(name, agg)| scan.forcing_pins(name, &agg.moves))
        .collect();

    // Every candidate's raw function δ (state δ then combinational-output δ), for the output CONE
    // (`support(δ_node)` state variables), the propagation walk and the fold's surviving-signal reference
    // check.
    let mut fn_of: BTreeMap<&str, &Bdd<B, C>> = BTreeMap::new();
    for (n, d) in deltas {
        fn_of.insert(n.as_str(), d);
    }
    for (n, d) in &m.out_deltas {
        fn_of.insert(n.as_str(), d);
    }

    // OPACITY over the live dependency loops. `live_succ[i]` is state `i`'s live-dependency graph among the
    // state variables (built once); a latch is OPAQUE in a `(clock, level)` phase iff some ELIGIBLE stable
    // state of that phase carries a live dependency cycle through it. Node-independent, so precomputed for
    // every `(state variable, clock, level)` — the memoisation the generation test reads back.
    let clocks: Vec<&Symbol> = inputs
        .iter()
        .filter(|p| scan.clock_set.contains(p.as_str()))
        .collect();
    let live_succ = scan.live_successors();
    let mut opaque_of: HashMap<(Symbol, Symbol, bool), bool> = HashMap::new();
    for w in &m.state_vars {
        for clock in &clocks {
            for level in [false, true] {
                let v = scan.order.iter().enumerate().any(|(i, s)| {
                    scan.eligible[i]
                        && s.value_of(clock.as_str()) == Some(level)
                        && reaches_self(&live_succ[i], w)
                });
                opaque_of.insert((w.clone(), (*clock).clone(), level), v);
            }
        }
    }
    // TRANSPARENCY of a latch in a `(clock, level)` phase: (a) no live dependency cycle through it at any
    // eligible stable state of the phase (`!opaque`, UNCHANGED), and (b) its value VARIES across the phase's
    // eligible stable states. A phase where the latch is one constant everywhere is a clamp, not an opening —
    // the value arrives regardless, whoever supplies the constant (a reset, or the toggled clock's own
    // level), so it is not the delivered side of a generation. Conjunct (b) is a plain scan of the eligible
    // stable-state order; no BDD work. This live-delivered-phase clause is what keeps a clock-DECLARED reset
    // (`RDFF`'s `R`, pinning `Q` at 0 across its whole phase) from reading as a generating edge, while a
    // clock-latched constant whose phase does vary (`SETLR`) stays a genuine opening.
    let transparent = |w: &Symbol, clock: &Symbol, level: bool| -> bool {
        if opaque_of[&(w.clone(), clock.clone(), level)] {
            return false;
        }
        let mut seen: Option<bool> = None;
        for (i, s) in scan.order.iter().enumerate() {
            if !scan.eligible[i] || s.value_of(clock.as_str()) != Some(level) {
                continue;
            }
            let Some(v) = s.value_of(w.as_str()) else {
                continue;
            };
            match seen {
                None => seen = Some(v),
                Some(prev) => {
                    if prev != v {
                        return true;
                    }
                }
            }
        }
        false
    };
    // A latch GENERATES on `(K, d)` iff it is opaque at the source level (`!d`) and transparent at the
    // delivered level (`d`). It is K-ASSOCIATED when its opacity differs across `K`'s two phases (a real latch
    // on `K` — the generator that opens on `d`, or the closer that shuts).
    let generates = |w: &Symbol, clock: &Symbol, is_rise: bool| -> bool {
        opaque_of[&(w.clone(), clock.clone(), !is_rise)] && transparent(w, clock, is_rise)
    };
    let k_assoc = |w: &Symbol, clock: &Symbol| -> bool {
        opaque_of[&(w.clone(), clock.clone(), false)]
            != opaque_of[&(w.clone(), clock.clone(), true)]
    };
    // A node's DIRECT-SUPPORT K-LATCHES: the state variables `δ_node` reads in ONE step that are
    // K-associated (a real latch on `clock`). Used SOLELY by the closer-exposure birth test to pick a
    // mux's two legs. It BOUNDS NOTHING — it is NOT a propagation depth. Propagation (see `propagates`) is
    // transitive and unbounded; this listing only names what a node reads DIRECTLY, which is exactly what
    // the two-leg mux shape reads off, and no more.
    let cone = |n: &str, clock: &Symbol| -> Vec<Symbol> {
        fn_of
            .get(n)
            .map(|f| {
                f.variables()
                    .filter(|v| m.state_set.contains(v) && k_assoc(v, clock))
                    .collect()
            })
            .unwrap_or_default()
    };
    // BIRTH: is an edge BORN at node `n` on `(clock, is_rise)` at the post-arc stable state `sp`? Two ways,
    // both evaluated at ANY node — never only the output:
    //   (a) BY GENERATION — a latch at `n` goes opaque→transparent across the edge (`generates`).
    //   (b) BY CLOSER-EXPOSURE — the toggle switches `n` to expose a latch it closes on THIS edge: a closer
    //       `c` and a generator `g`, distinct from `n` and from each other, both K-associated and in
    //       `δ_n`'s DIRECT support (`cone`), with `δ_n` restricted all-but-`c` at `sp` still depending on
    //       `c` (the two-leg mux shape — `DET`'s `Q` exposing the donor it just closed). The direct-support
    //       condition is the mux event itself, not a depth bound; a closer-exposure edge can be born at an
    //       internal node and then propagate onward.
    let born = |n: &Symbol, clock: &Symbol, is_rise: bool, sp: &Minterm<Symbol>| -> bool {
        if m.state_set.contains(n.as_str()) && generates(n, clock, is_rise) {
            return true;
        }
        let Some(f) = fn_of.get(n.as_str()) else {
            return false;
        };
        let legs = cone(n.as_str(), clock);
        legs.iter().any(|g| {
            g != n
                && generates(g, clock, is_rise)
                && legs
                    .iter()
                    .any(|c| c != g && c != n && residual_depends(f, sp, c.as_str()))
        })
    };
    // PROPAGATION (transitive, NO depth limit): from the output `o`, restriction-survival back along the
    // dependency chain to a `root` node. A hop `node → w` survives iff `δ_node` restricted to all-but-`w`
    // at the post-arc stable state `sp` still depends on `w`; a MASKED hop — whose residual is constant in
    // its predecessor (`ICG`'s `CLK*EL` swallowing `EL` at `CLK=0`, or a closed next stage) — dies.
    // Reaching `root` means `root`'s edge reaches `o`; `o` itself is the first node tested (a birth at `o`
    // types the arc directly).
    let propagates = |o: &Symbol, sp: &Minterm<Symbol>, root: &Symbol| -> bool {
        let mut visited: BTreeSet<Symbol> = BTreeSet::new();
        let mut stack: Vec<Symbol> = vec![o.clone()];
        while let Some(node) = stack.pop() {
            if &node == root {
                return true;
            }
            if !visited.insert(node.clone()) {
                continue;
            }
            let Some(f) = fn_of.get(node.as_str()) else {
                continue;
            };
            for w in f.variables() {
                if !m.state_set.contains(&w) || visited.contains(&w) {
                    continue;
                }
                if residual_depends(f, sp, w.as_str()) {
                    stack.push(w);
                }
            }
        }
        false
    };
    // ARC TYPING at a firing's post-arc stable state `sp`: an arc is EDGE iff some BIRTH node's edge
    // PROPAGATES to the output — `∃ b: born(b, K, d, sp) ∧ propagates(o, sp, root=b)`. Births are the
    // generators (a latch opaque→transparent) plus the closer-exposure nodes, both found at ANY node; the
    // birth universe is every candidate (an output or a state variable, each carrying a raw function).
    // Propagation is transitive with no depth cutoff, so a generator revealed through a DEEP same-phase
    // pipe or a BURIED mux types identically to a shallow one — there is no one-step-cone gate. Per firing
    // — `sp` is that firing's own destination — so two firings of one `(output, clock, direction)` can
    // type differently.
    let types_edge = |o: &Symbol, clock: &Symbol, is_rise: bool, sp: &Minterm<Symbol>| -> bool {
        candidates
            .iter()
            .any(|b| born(b, clock, is_rise, sp) && propagates(o, sp, b))
    };

    // THE PER-ARC LABELS: each derived delay arc whose related pin is a declared clock is an edge arc iff it
    // TYPES EDGE at its own firing's post-arc stable state. Membership is the identity itself, so two
    // firings of one `(output, clock, direction)` can type differently, and an unobserved edge — masked in
    // `arcs::derive`, or from an ineligible start — has no identity to add.
    let mut labels: HashSet<(Symbol, Symbol, Edge, Minterm<Symbol>)> = HashSet::new();
    for a in delay_arcs {
        if !scan.clock_set.contains(a.related.as_str()) || !scan.is_eligible(&a.start) {
            continue;
        }
        let is_rise = a.end.value_of(a.related.as_str()) == Some(true);
        let Some(sp) = machine::settle(deltas, &machine::toggle(&a.start, &[a.related.as_str()]))
        else {
            continue;
        };
        if types_edge(&a.output, &a.related, is_rise, &sp) {
            let edge = if is_rise { Edge::Rise } else { Edge::Fall };
            labels.insert((a.output.clone(), a.related.clone(), edge, a.start.clone()));
        }
    }

    // THE SEAM SET per candidate: the `(clock, direction)` toggles the node TYPES EDGE on at some eligible
    // CHANGED firing (the initial set), tightened by the greatest fixpoint in `seam_fixpoint` — the
    // delivered value must hold through the phase. A non-empty seam set is an edge register; an empty one is
    // level (a latch that merely tracks, or a clock gate).
    let seam_of: Vec<BTreeSet<Arc>> = candidates
        .iter()
        .zip(&aggs)
        .zip(&forcing_of)
        .map(|((name, agg), node_forcing)| {
            let mut s: BTreeSet<Arc> = BTreeSet::new();
            for ((clock, is_rise), cap) in &agg.captures {
                if !cap.changed {
                    continue; // vacuity gate: some eligible firing must change the node
                }
                let edge = cap.firings.iter().any(|(pre, np, post)| {
                    scan.is_eligible(pre)
                        && scan.is_eligible(np)
                        && m.output_value(name.as_str(), pre) != Some(*post)
                        && types_edge(name, clock, *is_rise, np)
                });
                if edge {
                    s.insert((clock.clone(), *is_rise));
                }
            }
            scan.seam_fixpoint(name.as_str(), node_forcing, &mut s);
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

    // READ-GATE FACTORISATION: a register output whose forcing pin merely READS the held state (`BDET`'s
    // output-enable) rather than CHANGING it (`RDFF`'s reset) is refactored — the state-holding register
    // becomes its own node with native edge capture and the output a combinational read over it — so the
    // fold does not destroy the content the output re-acquires when the gate releases. The discriminator is
    // STATE-CHANGE-IN-CONE: a forcing pin that never moves a state variable in the output's cone is a
    // read-gate. `derived` carries the factored registers (minted, or an existing declared register reused
    // up to inversion) and the reading outputs' combinational functions; `read_support` redirects each
    // read-gated output's fold seed onto its read function's support so its masters fold.
    let mut derived: Vec<DerivedRegister> = Vec::new();
    let mut read_support: HashMap<Symbol, Vec<Symbol>> = HashMap::new();
    let mut factored: BTreeSet<Symbol> = BTreeSet::new();
    if let Some(b) = builder.as_ref() {
        // The transitive state cone of a node: the state variables its δ depends on, directly or through
        // other state variables' δ.
        let cone_of = |o: &str| -> BTreeSet<Symbol> {
            let mut seen: BTreeSet<Symbol> = BTreeSet::new();
            let mut stack: Vec<Symbol> = fn_of
                .get(o)
                .map(|f| f.variables().filter(|v| m.state_set.contains(v)).collect())
                .unwrap_or_default();
            while let Some(w) = stack.pop() {
                if !seen.insert(w.clone()) {
                    continue;
                }
                if let Some(f) = fn_of.get(w.as_str()) {
                    for v in f.variables() {
                        if m.state_set.contains(&v) && !seen.contains(&v) {
                            stack.push(v);
                        }
                    }
                }
            }
            seen
        };
        let index_of: HashMap<&str, usize> = candidates
            .iter()
            .enumerate()
            .map(|(i, c)| (c.as_str(), i))
            .collect();
        let mut taken: BTreeSet<String> = inputs
            .iter()
            .map(|s| s.to_string())
            .chain(candidates.iter().map(|s| s.to_string()))
            .collect();
        let mut derived_map: BTreeMap<Symbol, DerivedRegister> = BTreeMap::new();
        let mut minted: Vec<EdgeCaptures> = Vec::new();

        // Every register output, in candidate order.
        let output_regs: Vec<usize> = seam_of
            .iter()
            .enumerate()
            .filter(|(i, s)| !s.is_empty() && output_names.contains(candidates[*i].as_str()))
            .map(|(i, _)| i)
            .collect();
        for iy in output_regs {
            let y = candidates[iy].clone();
            let Some(dy) = fn_of.get(y.as_str()).copied() else {
                continue;
            };
            let cone = cone_of(y.as_str());
            // A forcing pin is a READ-GATE iff toggling it never moves any state variable in the output's
            // cone (its pass level is the pin's un-asserted level).
            let mut gate_pass: Vec<(Symbol, bool)> = Vec::new();
            for (pin, (asserted, _)) in &forcing_of[iy] {
                let changes_cone = cone.iter().any(|w| {
                    index_of
                        .get(w.as_str())
                        .is_some_and(|&iw| aggs[iw].moves.iter().any(|(p, _, _, _)| p == pin))
                });
                if !changes_cone {
                    gate_pass.push((pin.clone(), !*asserted));
                }
            }
            if gate_pass.is_empty() {
                continue; // an ordinary register: its forcing pins all change the held state
            }

            // The register content the output reads: δ_Y cofactored at the read-gates' pass levels.
            let pass_min = Minterm::with_labels(
                &gate_pass
                    .iter()
                    .map(|(g, p)| (g.as_str(), Some(*p)))
                    .collect::<Vec<_>>(),
            )
            .expect("distinct read-gate pins");
            let content_bdd = dy.restrict_to(&pass_min);
            // The read function's columns: the register node plus every non-clock input the output reads.
            let gate_cols: Vec<Symbol> = inputs
                .iter()
                .filter(|p| {
                    !scan.clock_set.contains(p.as_str())
                        && dy.variables().any(|v| v.as_str() == p.as_str())
                })
                .cloned()
                .collect();

            // Reuse a declared register whose content matches (up to inversion — a NAND read of a DET
            // holds `!T`), else mint a fresh node holding the cofactored content.
            let matched = seam_of
                .iter()
                .enumerate()
                .filter(|(j, s)| *j != iy && !s.is_empty())
                .map(|(j, _)| candidates[j].clone())
                .find(|reg| {
                    let v = b.var(reg.as_str());
                    content_bdd.equivalent_to(&v) || content_bdd.equivalent_to(&!&v)
                });
            let reg_name = match &matched {
                Some(reg) => reg.clone(),
                None => {
                    let nm = crate::logic::mint_state_node(y.as_ref(), |n| taken.contains(n));
                    taken.insert(nm.clone());
                    Symbol::from(nm.as_str())
                }
            };

            // The read function over [register, read-columns], sampled from the machine: the register
            // value is resolved through the cofactored content, the output value directly.
            let read_header: Vec<Symbol> = std::iter::once(reg_name.clone())
                .chain(gate_cols.iter().cloned())
                .collect();
            let read_samples: Vec<(Minterm<Symbol>, bool)> = ex
                .order
                .iter()
                .filter_map(|s| {
                    if !scan.is_eligible(s) {
                        return None;
                    }
                    // The register node's value at this state: a reused declared register resolves through
                    // its own value (it holds `T`, while the cofactored content is `!T`); a minted register
                    // holds exactly the cofactored content.
                    let rv = match &matched {
                        Some(reg) => value(reg, s)?,
                        None => content_bdd.evaluate_fast(s)?,
                    };
                    let yv = value(&y, s)?;
                    let mut labels: Vec<(&str, Option<bool>)> = vec![(reg_name.as_str(), Some(rv))];
                    for g in &gate_cols {
                        labels.push((g.as_str(), s.value_of(g.as_str())));
                    }
                    Some((Minterm::with_labels(&labels).ok()?, yv))
                })
                .collect();
            let reads_sr = synth_capture(b, &read_header, &read_samples)
                .expect("a read-gated output is a function of its register and gate pins");

            // Mint the register's EdgeCaptures from the output's own — cofactored gate-free: the captures
            // lose the gate columns, the off-edge collapses to a pure hold.
            if matched.is_none() {
                let y_ec = captures
                    .iter()
                    .find(|ec| ec.node == y)
                    .expect("a register output has an EdgeCaptures entry");
                let node_captures: Vec<(Symbol, Edge, StateRegions)> = y_ec
                    .captures
                    .iter()
                    .map(|(clock, edge, sr)| {
                        (clock.clone(), *edge, cofactor_capture(b, sr, &pass_min))
                    })
                    .collect();
                let off_edge = cofactor_off_edge(b, &y_ec.off_edge, &pass_min);
                let cols = capture_cols(&node_captures, &off_edge);
                minted.push(EdgeCaptures {
                    node: reg_name.clone(),
                    captures: node_captures,
                    off_edge,
                    cols,
                });
            }

            let content = regions::minimise_bdd(&content_bdd);
            derived_map
                .entry(reg_name.clone())
                .or_insert_with(|| DerivedRegister {
                    name: reg_name.clone(),
                    content,
                    reads: Vec::new(),
                })
                .reads
                .push((y.clone(), reads_sr));
            read_support.insert(y.clone(), read_header);
            factored.insert(y);
        }

        // Drop the factored outputs' register entries and add the minted registers.
        captures.retain(|ec| !factored.contains(&ec.node));
        captures.extend(minted);
        derived = derived_map.into_values().collect();
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
    // loops so oscillation stays detectable — minimisation is untouched by this. The minimise fixpoint
    // invariant I3 (`src/logic/minimise.rs`) holds by construction: every kept survivor's support is kept
    // by closure.
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
    // A READ-GATED output carries a seam but emits a
    // combinational read over its factored register — so it is a live sink like any non-seam output, and it
    // propagates through its READ FUNCTION's support (register + gate pins), not its raw function. That
    // redirect is what folds the masters it re-expresses (`BDET`'s `L1/L2`).
    let mut live: BTreeSet<&str> = BTreeSet::new();
    for (name, s) in candidates.iter().zip(&seam_of) {
        if !s.is_empty() && !factored.contains(name) {
            continue;
        }
        if !foldable.contains(name.as_str()) || ref_reg.contains(name.as_str()) {
            live.insert(name.as_str());
        }
    }
    let mut worklist: Vec<&str> = live.iter().copied().collect();

    // Propagate liveness along each live node's raw-function support — semantic BDD support, never equation
    // shape — until the least fixpoint is reached. A read-gated output propagates through its read support.
    while let Some(l) = worklist.pop() {
        if let Some(sup) = read_support.get(l) {
            for v in sup {
                if let Some(&n) = foldable.get(v.as_str()) {
                    if live.insert(n) {
                        worklist.push(n);
                    }
                }
            }
            continue;
        }
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
        derived,
    }
}

/// The classifier's scan context over one machine: the single-input transition table across the reachable
/// stable states, those states' ARC ELIGIBILITY, and the declared clock set. All three are
/// NODE-INDEPENDENT — they describe the cell's state machine, not any one candidate — so they are read
/// once per cell and every seam, phase and forcing scan indexes into them rather than re-settling.
struct Scan<'a, B: Brand, C: ManagerCell> {
    /// The machine every scan reads.
    m: &'a Machine<'a, B, C>,
    /// The reachable stable states: the index space `next` and `eligible` are aligned with.
    order: &'a [Minterm<Symbol>],
    /// `next[s][x]` is the index of the stable state reached by toggling input `x` — the machine's own
    /// input order — in `order[s]` and settling. `None` when that toggle oscillates, or lands outside the
    /// explored set.
    next: Vec<Vec<Option<usize>>>,
    /// Each state's measurement eligibility ([`Machine::arc_eligible`]).
    eligible: Vec<bool>,
    /// The declared clocks, for membership tests. Every declared clock is a candidate edge key; whether a
    /// clock keeps edge arcs on a given node is decided behaviourally, not by input-class routing.
    clock_set: HashSet<&'a str>,
}

impl<'a, B: Brand, C: ManagerCell + Send + Sync> Scan<'a, B, C> {
    fn new(m: &'a Machine<'a, B, C>) -> Self {
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
        Scan {
            m,
            order,
            next,
            eligible: order.iter().map(|s| m.arc_eligible(s)).collect(),
            clock_set: m.cell.clock_pins.iter().map(Symbol::as_str).collect(),
        }
    }

    /// Is `s` eligible to be measured from (see [`Machine::arc_eligible`])? The measurement gate for a
    /// state reached during a walk; `eligible` answers the same question for a state already in `order`.
    fn is_eligible(&self, s: &Minterm<Symbol>) -> bool {
        self.m.arc_eligible(s)
    }

    /// The LIVE dependency graph among the state variables at every reachable stable state. `live_succ[i]`
    /// maps each state variable `n` to the state variables whose δ, restricted to all of its support EXCEPT
    /// `n` at state `i`'s values, still depends on `n` — `n`'s successors in state `i`'s live-dependency
    /// graph. A cycle through a latch in this graph is the memory signature: the latch is bistable (opaque)
    /// there. Only ELIGIBLE states carry a graph; an ineligible state's entry is empty and is never measured
    /// from. No machine state is constructed — every edge is decided by restriction of the existing δ at the
    /// state.
    fn live_successors(&self) -> Vec<BTreeMap<Symbol, BTreeSet<Symbol>>> {
        self.order
            .iter()
            .enumerate()
            .map(|(i, s)| {
                let mut succ: BTreeMap<Symbol, BTreeSet<Symbol>> = BTreeMap::new();
                if !self.eligible[i] {
                    return succ;
                }
                for (target, delta) in &self.m.deltas {
                    for n in delta.variables() {
                        if self.m.state_set.contains(&n) && residual_depends(delta, s, n.as_str()) {
                            succ.entry(n).or_default().insert(target.clone());
                        }
                    }
                }
                succ
            })
            .collect()
    }

    /// The greatest-fixpoint filter that keeps `s` to the `(clock, direction)` toggles whose DELIVERED VALUE
    /// HOLDS through the phase. A `(k, d)` is removed when some NON-FORCING change of `node` inside its
    /// delivered phase (`clock == d`) happens at a toggle that is NOT itself an edge of `s` — live data (a
    /// non-clock input) or a non-seam clock. A co-resident clock's edge that IS in `s` is another seam of the
    /// node, not a disqualifier, so iterating to a fixpoint lets one seam's removal cascade to another
    /// (`MCDFF` loses `(CLKB, Rise)` on live D, then `(CLKA, Fall)` because the in-phase CLKB rise is gone).
    /// Only ELIGIBLE states, at both ends of a transition, take part.
    fn seam_fixpoint(
        &self,
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
                for (si, st) in self.order.iter().enumerate() {
                    if !self.eligible[si]
                        || st.value_of(k.as_str()) != Some(*is_rise)
                        || is_forced(st)
                    {
                        continue;
                    }
                    let Some(v) = self.m.output_value(node, st) else {
                        continue;
                    };
                    // The machine's own input order — the order `next`'s columns are built from.
                    for (xi, x) in self.m.cell.inputs.iter().enumerate() {
                        if x == k {
                            continue;
                        }
                        let Some(ni) = self.next[si][xi] else {
                            continue;
                        };
                        if !self.eligible[ni] {
                            continue;
                        }
                        let dest = &self.order[ni];
                        if is_forced(dest) {
                            continue;
                        }
                        match self.m.output_value(node, dest) {
                            Some(dv) if dv != v => {}
                            _ => continue, // the node did not move (or is undefined) here
                        }
                        if node_forcing.contains_key(x) {
                            continue; // a forcing pin's assertion is a coexisting combinational arc
                        }
                        // Is the toggle of `x` itself an edge of the node's current seam set?
                        let x_is_seam = self.clock_set.contains(x.as_str()) && {
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
    fn forcing_pins(
        &self,
        node: &Symbol,
        moves: &[(Symbol, Minterm<Symbol>, Minterm<Symbol>, bool)],
    ) -> BTreeMap<Symbol, (bool, bool)> {
        let inputs = &self.m.cell.inputs;
        // Clause 2 - GLOBAL CONSTANT-PINNING: exactly one level of the pin holds the node at one constant
        // across ALL reachable stable states (an async override whose release re-acquires, like a toggle
        // flop's reset). No tracked data pin satisfies this: its tracking is confined to a clock-phase
        // region, and elsewhere the node varies under the same pin level. A REAL capture clock never pins
        // the node to a constant either (the node carries content in both phases), so declaration plays no
        // part — a level-forcing reset is a forcing pin whether or not it was declared a clock, which is how
        // `RDFF`'s clock-declared `R` is handled here.
        let mut pinning: BTreeMap<Symbol, (bool, bool)> = BTreeMap::new();
        for x in inputs {
            let pinned_value = |level: bool| -> Option<bool> {
                let mut seen: Option<bool> = None;
                for s in self.order {
                    if s.value_of(x.as_str()) != Some(level) {
                        continue;
                    }
                    let Some(v) = self.m.output_value(node.as_str(), s) else {
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
            match (pinned_value(false), pinned_value(true)) {
                (Some(_), Some(_)) | (None, None) => {} // both levels pin to a constant (degenerate) or neither
                (Some(v), None) => {
                    pinning.insert(x.clone(), (false, v));
                }
                (None, Some(v)) => {
                    pinning.insert(x.clone(), (true, v));
                }
            }
        }
        // Monotone accumulation: established forcing pins are never re-litigated, so each round can only
        // ADD pins and the loop terminates within `inputs.len()` rounds.
        let mut forcing: BTreeMap<Symbol, (bool, bool)> = pinning;
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
}

/// Does `f`, once every variable of its support EXCEPT `freed` is fixed to `state`'s values, still depend
/// on `freed`? The restriction-survival probe underlying both the live-dependency test and propagation:
/// [`Bdd::restrict_to`] fixes the pinned variables and leaves `freed` free (a variable the minterm does not
/// carry is left free — no silent default), and the residual depends on `freed` iff `freed` remains in its
/// support (the residual is non-constant in `freed`).
fn residual_depends<B: Brand, C: ManagerCell>(
    f: &Bdd<B, C>,
    state: &Minterm<Symbol>,
    freed: &str,
) -> bool {
    let fixed: Vec<Symbol> = f.variables().filter(|v| v.as_str() != freed).collect();
    let residual = f.restrict_to(&state.project_to(fixed.iter().map(Symbol::as_str)));
    residual.variables().any(|v| v.as_str() == freed)
}

/// Does state variable `w` lie on a directed cycle of live dependency edges — is `w` reachable from itself
/// by following `succ`? A self-loop (`w → w` live) or any longer loop through `w` answers yes: `w` is
/// opaque (bistable) at this state.
fn reaches_self(succ: &BTreeMap<Symbol, BTreeSet<Symbol>>, w: &Symbol) -> bool {
    let mut seen: BTreeSet<&Symbol> = BTreeSet::new();
    let mut stack: Vec<&Symbol> = succ.get(w).into_iter().flatten().collect();
    while let Some(x) = stack.pop() {
        if x == w {
            return true;
        }
        if seen.insert(x) {
            if let Some(next) = succ.get(x) {
                stack.extend(next.iter());
            }
        }
    }
    false
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

/// Cofactor a TOTAL capture's regions at the read-gates' pass levels (`pass` fixes the gate pins): the gate
/// columns vanish and the capture is re-based over its surviving support. The on-set is the cofactored
/// on-cover, the off-set its complement (a capture is total), the hold empty. Used to mint a factored
/// register's captures from the read-gated output's own (`!(D*A)|A=1 → !D`).
fn cofactor_capture<B: Brand, C: ManagerCell>(
    builder: &BddBuilder<B, C>,
    sr: &StateRegions,
    pass: &Minterm<Symbol>,
) -> StateRegions {
    let on = builder.build_cover(&sr.on_cover).restrict_to(pass);
    let off = !&on;
    let hold = builder.constant(false);
    let header: Vec<Symbol> = sr
        .cols
        .iter()
        .filter(|c| pass.value_of(c.as_str()).is_none())
        .cloned()
        .collect();
    regions_from(&on, &off, &hold, &header)
}

/// Cofactor an off-edge's regions at the read-gates' pass levels: the set/clear/hold covers each lose the
/// gate columns. A read-gate whose non-pass level forces the output collapses to a PURE HOLD at the pass
/// level (`BDET`, whose factored register has no async set/clear of its own).
fn cofactor_off_edge<B: Brand, C: ManagerCell>(
    builder: &BddBuilder<B, C>,
    sr: &StateRegions,
    pass: &Minterm<Symbol>,
) -> StateRegions {
    let on = builder.build_cover(&sr.on_cover).restrict_to(pass);
    let off = builder.build_cover(&sr.off_cover).restrict_to(pass);
    let hold = !&on.or(&off);
    let header: Vec<Symbol> = sr
        .cols
        .iter()
        .filter(|c| pass.value_of(c.as_str()).is_none())
        .cloned()
        .collect();
    regions_from(&on, &off, &hold, &header)
}

/// Synthesise a capture region from its `(pre-state, post-value)` samples over `header`. The witnessed
/// on-samples are the ON-set, the witnessed off-samples the OFF-set and the unwitnessed remainder a
/// don't-care set: the capture is the ON-set generalised (incompletely-specified minimisation) so it
/// generalises past the reachable pre-states to the underlying function — reachability need not cover
/// every projection for the cover to land on the true capture. The generalised on-set is total, its off
/// the exact complement (empty hold). Returns `None` when a projection carries both an on- and an
/// off-sample — a conflict that the uniform header (all inputs except the keying clock plus every
/// candidate) makes impossible for the classifier's own calls, since equal projections are then the same
/// machine minterm.
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
            let preserved = crate::logic::minimise::Preserved::outputs(
                $analysed.outputs.iter().map(|o| o.name.clone()).collect(),
            );
            let min = crate::logic::minimise::minimise_state_space(&mut $bdds, &order, &preserved);
            crate::model::recompute_signal_metadata(&mut $analysed, &$bdds, &min);
            let $m = crate::logic::analysis::Machine::build(
                &$analysed,
                &$bdds,
                &crate::logic::machine::ExplorationBudget::default(),
            )
            .unwrap();
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
        let inputs = &m.cell.inputs;

        // The classifier's own scan context — the transition table and the ELIGIBLE stable states, measured
        // only where every state column is determinate (a don't-care is a missing variable, never coerced) —
        // and the live dependency graph at each. Both feed the restriction-survival transparency test the
        // harness reads open phases with.
        let scan = Scan::new(m);
        let live_succ = scan.live_successors();
        // A node is TRANSPARENT in `clock`'s `level` phase iff it carries NO live dependency cycle at any
        // eligible stable state of the phase (not opaque) AND its value VARIES across those states — the
        // same restriction-survival form the classifier types generation on. A phase pinned to one constant
        // everywhere is a forcing, not an opening, so it is not transparent.
        let phase_transparent = |node: &Symbol, clock: &Symbol, level: bool| -> bool {
            let opaque = m.state_set.contains(node.as_str())
                && scan.order.iter().enumerate().any(|(i, s)| {
                    scan.eligible[i]
                        && s.value_of(clock.as_str()) == Some(level)
                        && super::reaches_self(&live_succ[i], node)
                });
            if opaque {
                return false;
            }
            let mut seen: Option<bool> = None;
            for (i, s) in scan.order.iter().enumerate() {
                if !scan.eligible[i] || s.value_of(clock.as_str()) != Some(level) {
                    continue;
                }
                let Some(v) = m.output_value(node.as_str(), s) else {
                    continue;
                };
                match seen {
                    None => seen = Some(v),
                    Some(prev) if prev != v => return true,
                    Some(_) => {}
                }
            }
            false
        };

        // The read-gate factorisation's MINTED registers are not machine nodes, so their value is resolved
        // through their content cover (built once on the harness builder) rather than `output_value`. A
        // declared register reused by the factorisation stays a machine node and resolves normally. This is
        // what makes the derived register's captures and off-edge replay for real against the machine below,
        // rather than passing vacuously on an all-`None` value.
        let is_machine_node = |name: &Symbol| {
            m.state_set.contains(name.as_str()) || m.cell.outputs.iter().any(|o| &o.name == name)
        };
        let derived_content: BTreeMap<Symbol, Bdd<B, C>> = es
            .derived
            .iter()
            .filter(|d| !is_machine_node(&d.name))
            .map(|d| (d.name.clone(), builder.build_cover(&d.content)))
            .collect();

        for r in &es.captures {
            let node = r.node.as_str();
            let value = |s: &Minterm<Symbol>| -> Option<bool> {
                match derived_content.get(&r.node) {
                    Some(cb) => cb.evaluate_fast(s),
                    None => m.output_value(node, s),
                }
            };

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
                scan.order.iter().enumerate().any(|(si, s)| {
                    s.value_of(clock.as_str()) == Some(!*is_rise)
                        && scan.next[si][xi].is_some_and(|ni| value(&scan.order[ni]) != value(s))
                })
            };
            let is_transparent =
                |(clock, is_rise): &Arc| -> bool { phase_transparent(&r.node, clock, !*is_rise) };

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
            for s in scan.order {
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
            for (si, s) in scan.order.iter().enumerate() {
                for (xi, x) in inputs.iter().enumerate() {
                    let Some(ni) = scan.next[si][xi] else {
                        continue;
                    };
                    let dest = &scan.order[ni];
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

    /// A machine-check of the fold's emission guarantee: NOTHING THAT SURVIVES EMISSION MAY NAME A FOLDED
    /// NODE. A fold — the group fold especially — drops the node's column from the emitted table, so a
    /// survivor still referencing it would emit a dangling column. For every folded name this checks both
    /// routes a reference can take: a surviving capture's cover columns, and the raw function of a surviving
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

        // A read-gated output emits its READ FUNCTION (over the factored register and gate pins), not its
        // raw δ — that function must name no folded node, and the output is excluded from the raw-function
        // check (b) below, which would otherwise flag the masters it re-expresses.
        let read_gated: BTreeSet<&str> = es
            .derived
            .iter()
            .flat_map(|d| d.reads.iter().map(|(o, _)| o.as_str()))
            .collect();
        for d in &es.derived {
            for (o, sr) in &d.reads {
                for col in &sr.cols {
                    assert!(
                        !folded.contains(col.as_str()),
                        "dropped reference: read function of {o} names folded node {col}, folded {:?}",
                        folded_list(es)
                    );
                }
            }
        }

        // (b) no surviving capture-less candidate's raw function has a folded node in its support. The
        // candidate population is the classifier's own: every output plus every non-output state
        // variable. A candidate that carries a capture is not a survivor of this kind — its raw function
        // is replaced by the edge seam — and the folded nodes themselves are gone. A read-gated output is
        // likewise not a survivor of this kind — it emits its read function, checked above.
        let output_names: BTreeSet<&str> = m.cell.outputs.iter().map(|o| o.name.as_str()).collect();
        let captured: BTreeSet<&str> = node_list(es)
            .into_iter()
            .chain(read_gated.iter().copied())
            .collect();
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

    /// Replay-faithfulness for the read-gate factorisation's READ FUNCTIONS: at every reachable stable
    /// state, the emitted read cover — evaluated with the factored register resolved through its own value
    /// (a minted register through its content cover, a reused declared register through `output_value`) and
    /// the gate pins at their state values — reproduces the machine's output. This is the semantic
    /// correctness gate for `Y = state_function(register, gates)`, stronger than any literal SOP match.
    fn assert_reads_faithful<B: Brand, C: ManagerCell + Send + Sync>(
        m: &Machine<B, C>,
        es: &EdgeArcs,
    ) {
        let Some((_, any)) = m.deltas.first() else {
            return;
        };
        let b = any.builder();
        let is_machine = |n: &str| {
            m.state_set.contains(n) || m.cell.outputs.iter().any(|o| o.name.as_str() == n)
        };
        for d in &es.derived {
            let content = b.build_cover(&d.content);
            let reg_value = |s: &Minterm<Symbol>| -> Option<bool> {
                if is_machine(d.name.as_str()) {
                    m.output_value(d.name.as_str(), s)
                } else {
                    content.evaluate_fast(s)
                }
            };
            for (o, sr) in &d.reads {
                let read = b.build_cover(&sr.on_cover);
                let mut exercised = false;
                for s in &m.explored.order {
                    let (Some(rv), Some(yv)) = (reg_value(s), m.output_value(o.as_str(), s)) else {
                        continue;
                    };
                    let mut labels: Vec<(&str, Option<bool>)> = vec![(d.name.as_str(), Some(rv))];
                    for c in &sr.cols {
                        if c != &d.name {
                            labels.push((c.as_str(), s.value_of(c.as_str())));
                        }
                    }
                    let mm: Minterm<Symbol> =
                        Minterm::with_labels(&labels).expect("distinct read columns");
                    let Some(got) = read.evaluate_fast(&mm) else {
                        continue;
                    };
                    exercised = true;
                    assert_eq!(
                        got, yv,
                        "read unfaithful: {o} over {} at {s:?}: read {got} != machine {yv}",
                        d.name
                    );
                }
                assert!(exercised, "read function of {o} was never exercised");
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

    // === Floor: the canonical flop and interlock keep exactly their arcs ===

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
            // GCLK = enA*CLKA + enB*CLKB is a combinational clock gate: on either clock's edge no
            // clock-associated source survives the restriction (the competing enable is not that clock's
            // associate, and the branch's own enable is masked at the destination), so GCLK carries no arc.
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

    // === Reset forcing vs blocking: a clear is a coexisting combinational arc, not a blocker ===

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

    // === Inverting capture and exposed/tapped masters ===

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

    // === Cross-coupled NAND slave ===

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

    // === Dual-edge mux (DET) ===

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

    const CHAIN3_TOML: &str = r#"
[[cell]]
name = "CHAIN3"
inputs = ["K1", "K2", "K3", "D"]
clock = ["K1", "K2", "K3"]
[cell.internal]
L1 = "!K1*D + K1*L1"
L2 = "!K2*L1 + K2*L2"
[cell.outputs]
L3 = "!K3*L2 + K3*L3"
"#;

    #[test]
    fn edge_three_latch_chain_two_birth_teeth() {
        // Three latches in series on three DISTINCT clocks — `L1@K1 -> L2@K2 -> L3@K3`, output `L3`, with
        // `L2`/`L3` open (transparent) at the firing. A `K1` edge captures into `L1` and flows through the
        // two open cross-clock latches to `L3`. The generator `L1` sits TWO hops from `L3` (`L3`'s direct
        // support is `{L2}`, a `K2`-latch carrying no `K1` birth); the two-birth transitive gate propagates
        // `L1`'s generation birth through the open latches and types the `K1->L3` arc EDGE.
        with_machine!(CHAIN3_TOML, |_b, _a, _m2, m| {
            let es = classify(&m);
            // A pure latch chain holds no capture: no register, nothing minted — full replay faithfulness.
            assert!(
                es.captures.is_empty(),
                "a pure latch chain holds no capture"
            );
            assert!(es.derived.is_empty());
            assert_captures_faithful(&m, &es);

            // The two-birth gate types the `K1->L3` arc EDGE (the generator two hops away, through the open
            // cross-clock latches).
            assert!(
                label_list(&es).contains(&("L3", "K1", Edge::Fall)),
                "K1->L3 must type edge under the two-birth gate: {:?}",
                label_list(&es)
            );

            // The teeth: the generator `L1` sits two hops from `L3` — `L3` reads `L2` directly, `L2` reads
            // `L1` — so the two-birth gate reaches it by propagating `L1`'s generation birth transitively
            // along the dependency chain, hop by hop from `L2` up to `L3`.
            let direct = |n: &str| -> Vec<String> {
                let f = m
                    .out_deltas
                    .get(&Symbol::from(n))
                    .or_else(|| {
                        m.deltas
                            .iter()
                            .find(|(s, _)| s.as_str() == n)
                            .map(|(_, d)| d)
                    })
                    .expect("a delta for the queried node");
                f.variables()
                    .filter(|v| m.state_set.contains(v))
                    .map(|v| v.to_string())
                    .collect()
            };
            assert!(direct("L3").contains(&"L2".to_string()));
            assert!(
                !direct("L3").contains(&"L1".to_string()),
                "L1 is two hops from L3, not in its direct support"
            );
            assert!(direct("L2").contains(&"L1".to_string()));
        });
    }

    const BDET_TOML: &str = r#"
[[cell]]
name = "BDET"
inputs = ["CLK", "D", "A"]
clock = ["CLK"]
[cell.internal]
L1 = "!CLK*D + CLK*L1"
L2 = "CLK*D + !CLK*L2"
[cell.outputs]
Y = "!((CLK*L1 + !CLK*L2)*A)"
"#;

    const DETP_TOML: &str = r#"
[[cell]]
name = "DETP"
inputs = ["CLK", "CLKB", "D", "A"]
clock = ["CLK", "CLKB"]
[cell.internal]
L1 = "!CLK*D + CLK*L1"
L2 = "CLK*D + !CLK*L2"
T = "!CLKB*(CLK*L1 + !CLK*L2) + CLKB*T"
[cell.outputs]
Y = "!(T*A)"
"#;

    #[test]
    fn edge_bdet_read_gate_factorisation() {
        // BDET: a dual-edge flop read through an output-enable `A` (`Y = !(M*A)`, `M = CLK*L1+!CLK*L2`). `A`
        // is a READ-GATE — toggling it never moves the DET latches `L1/L2` in `Y`'s cone — so the register
        // is factored out as `Y_st` with native dual-edge capture and `Y` becomes a combinational read over
        // it, freeing the masters to fold.
        with_machine!(BDET_TOML, |builder, _a, _m2, m| {
            let es = classify(&m);
            assert_captures_faithful(&m, &es);
            assert_reads_faithful(&m, &es);
            assert_no_dropped_references(&m, &es);

            // The DET latches fold; the sole state node is the minted register.
            let mut folded = folded_list(&es);
            folded.sort();
            assert_eq!(folded, ["L1", "L2"]);
            assert_eq!(node_list(&es), ["Y_st"], "Y is factored out, Y_st minted");

            // `Y_st` is a dual-edge register; its native captures deliver `!D` on both edges (the NAND read
            // inverts the held content — inversion is not special-cased).
            let yst = reg(&es, "Y_st");
            assert_eq!(yst.captures.len(), 2, "dual edge");
            let nd = !&builder.var("D");
            for (_, _, cap) in &yst.captures {
                assert!(
                    builder.build_cover(&cap.on_cover).equivalent_to(&nd),
                    "each edge captures !D"
                );
            }

            // One derived register `Y_st`, read by `Y`; nothing else minted. The read function's
            // machine-faithfulness (equivalent to `Y = !(M*A)`) is proven by assert_reads_faithful.
            assert_eq!(es.derived.len(), 1);
            assert_eq!(es.derived[0].name, "Y_st");
            let reads: Vec<&str> = es.derived[0]
                .reads
                .iter()
                .map(|(o, _)| o.as_str())
                .collect();
            assert_eq!(reads, ["Y"]);

            // `CLK->Y` stays edge on both edges; `A` carries no edge label (it is not a clock), so `A->Y`
            // arcs render `-type combinational`.
            assert_eq!(
                labels_of(&es, "Y"),
                [("CLK", Edge::Rise), ("CLK", Edge::Fall)]
            );
        });
    }

    #[test]
    fn edge_detp_reads_declared_register_no_mint() {
        // DETP: a DET mux buried in a cross-clock latch `T` (`T = !CLKB*(CLK*L1+!CLK*L2)+CLKB*T`), read
        // through `A` (`Y = !(T*A)`). `T` is a declared register; the factorisation REUSES it — `Y`'s
        // cofactored content `!T` matches `T` up to inversion — and mints NOTHING.
        //
        // Two-birth teeth: the CLK->Y arcs are the closer-exposure-at-an-internal-node case. The DET mux
        // switch is a closer-exposure birth at the internal node `T` (itself not `CLK`-associated); the
        // two-birth gate propagates that birth onward from `T` to `Y` and types the CLK->Y arcs EDGE.
        with_machine!(DETP_TOML, |_b, _a, _m2, m| {
            let es = classify(&m);
            assert_captures_faithful(&m, &es);
            assert_reads_faithful(&m, &es);
            assert_no_dropped_references(&m, &es);

            // The two CLK->Y arcs type EDGE under the two-birth gate (the internal-node birth propagates
            // onward to Y). This is the teeth arc.
            assert!(label_list(&es).contains(&("Y", "CLK", Edge::Rise)));
            assert!(label_list(&es).contains(&("Y", "CLK", Edge::Fall)));

            // `T` keeps its native edge register; `Y` is factored to a read over it; nothing minted.
            assert!(
                node_list(&es).contains(&"T"),
                "T stays a register: {:?}",
                node_list(&es)
            );
            assert!(
                !node_list(&es).contains(&"Y"),
                "Y is a combinational read, not a register"
            );
            assert_eq!(es.derived.len(), 1);
            assert_eq!(es.derived[0].name, "T", "reuses the declared register");
            let reads: Vec<&str> = es.derived[0]
                .reads
                .iter()
                .map(|(o, _)| o.as_str())
                .collect();
            assert_eq!(reads, ["Y"]);
        });
    }

    #[test]
    fn edge_read_gate_controls_do_not_factor() {
        // Register-forcing control (RDFF: reset `R` CHANGES the held state) and no-gate control (plain DET:
        // no forcing read pin) both leave `derived` empty — the factorisation fires only for a read-gate.
        with_machine!(RDFF_TOML, |_b, _a, _m2, m| {
            let es = classify(&m);
            assert!(
                es.derived.is_empty(),
                "RDFF's reset changes state, not a read-gate"
            );
        });
        with_machine!(DET_TOML, |_b, _a, _m2, m| {
            let es = classify(&m);
            assert!(es.derived.is_empty(), "plain DET has no read gate");
            assert!(node_list(&es).contains(&"Q"), "DET's Q stays a register");
        });
    }

    const COMPOSED_TOML: &str = r#"
[[cell]]
name = "RDFFRE"
inputs = ["CLK", "D", "R", "A"]
clock = ["CLK"]
[cell.internal]
M = "!R*(!CLK*D + CLK*M)"
[cell.outputs]
Q = "!R*(CLK*M + !CLK*Q)"
Y = "!(Q*A)"
"#;

    #[test]
    fn edge_composed_register_clear_and_read_gate() {
        // A resettable DFF register `Q` (register-forcing reset `R`) with a read-gated second output
        // `Y = !(Q*A)`. Only the gated output factors: `Q` keeps its native register and its off-edge `R`
        // clear, and `Y` reuses `Q` (content `!Q` matches `Q` up to inversion), minting nothing.
        with_machine!(COMPOSED_TOML, |builder, _a, _m2, m| {
            let es = classify(&m);
            assert_captures_faithful(&m, &es);
            assert_reads_faithful(&m, &es);
            assert_no_dropped_references(&m, &es);

            let q = reg(&es, "Q");
            let off = builder.build_cover(&q.off_edge.off_cover);
            assert!(
                off.equivalent_to(&builder.var("R")),
                "Q keeps its off-edge R clear"
            );

            assert_eq!(es.derived.len(), 1);
            assert_eq!(
                es.derived[0].name, "Q",
                "the register keeps its clear, only Y factors"
            );
            let reads: Vec<&str> = es.derived[0]
                .reads
                .iter()
                .map(|(o, _)| o.as_str())
                .collect();
            assert_eq!(reads, ["Y"]);
            assert!(
                !node_list(&es).contains(&"Y"),
                "Y is a combinational read, not a register"
            );
        });
    }

    #[test]
    fn edge_read_gate_corrupted_cover_teeth() {
        // The harness has teeth on the DERIVED registers: corrupt a derived content cover and the
        // replay must fail. A test that cannot fail on the bug it targets is not a test.
        with_machine!(BDET_TOML, |builder, _a, _m2, m| {
            let mut es = classify(&m);
            assert!(!es.derived.is_empty(), "BDET factors a derived register");
            // Invert the minted register's content: the resolver now reads the wrong held value.
            let good = builder.build_cover(&es.derived[0].content);
            es.derived[0].content = regions::minimise_bdd(&!&good);

            let prev = std::panic::take_hook();
            std::panic::set_hook(Box::new(|_| {}));
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                assert_captures_faithful(&m, &es)
            }));
            std::panic::set_hook(prev);
            assert!(
                result.is_err(),
                "corrupting a derived cover must make the replay harness fail"
            );
        });
    }

    // === Phase-symmetric data transparency stays a level cell ===

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
            // D is preserved as a live data dependency: some reachable state where toggling D moves T.
            // Typing T as a register keyed off CLK would drop D while the run still emits D→T data arcs, so
            // T correctly stays level and D survives in T's function.
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

    // A gated conjunctive clear: R*G forces both latches to 0 (needs G high too), while the R=1,G=0 states
    // merely hold. The off-edge synthesis lands R*G as a Forced0 cover, so R and G each participate in a
    // forced projection and Q stays a register with the conjunctive clear carried in off_edge.off.
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
        // recognised consistently as an edge register. The off-edge synthesis carries each clear as its own
        // forced cover, so R+G and R*G, sync and async-declared, all keep Q a register alike.
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

    // === Declared clocks that act by level, not by edge (RDFF, ICG) ===

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
        // GCLK = CLK*EL is combinational CAUSALLY: on the fall EL opens, but GCLK's residual over
        // all-but-EL at the destination (CLK=0) is 0 — the CLK*EL gate swallows the captured value, so no
        // propagation source survives; on the rise nothing generates. EL is the classic transparent latch
        // (level, phase-asymmetric to EN) and survives unfolded because GCLK's raw function still
        // references it.
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
        // Q carries an edge seam, so its raw function is replaced by that seam and nothing surviving
        // references the master ⇒ M folds away.
        with_machine!(MOR_ASYNC_TOML, |_b, _a, _m2, m| {
            let es = classify(&m);
            assert_eq!(
                folded_list(&es),
                ["M"],
                "the recognised slave folds its master"
            );
        });
    }

    // === Exploration budget ===

    #[test]
    fn edge_budget_overrun_yields_default() {
        // A cell whose exploration passes a budget ceiling has no Machine ⇒ default annotation. 24
        // inputs put 2^23 seed minterms in each of Y's forced cover cubes, past the default ceiling.
        let n = 24;
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
        let preserved = crate::logic::minimise::Preserved::outputs(
            analysed.outputs.iter().map(|o| o.name.clone()).collect(),
        );
        let min = crate::logic::minimise::minimise_state_space(&mut bdds, &order, &preserved);
        crate::model::recompute_signal_metadata(&mut analysed, &bdds, &min);
        assert!(
            crate::logic::analysis::Machine::build(
                &analysed,
                &bdds,
                &crate::logic::machine::ExplorationBudget::default(),
            )
            .is_err(),
            "wide cell passes the candidate budget ⇒ default EdgeArcs"
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
    /// The invariant holds BY CONSTRUCTION: `classify` takes `&Machine` read-only, mutates nothing, and the
    /// annotation may carry emission-time derived registers (the read-gate factorisation) that are
    /// functions of already-explored state, never new state variables. This test additionally proves the
    /// flag-gating is PURE — when opted out (`no_edge_collapse`) the classify() call is skipped and the
    /// annotation is the byte-identical Default, with every other field untouched. `BDET`/`DETP` exercise
    /// the factorisation path: only `edge` differs there too.
    #[test]
    fn edge_classification_changes_only_the_edge_annotation() {
        for src in [DFF_TOML, ICM_TOML, BDET_TOML, DETP_TOML] {
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

    // === Grounded per-arc fixtures (DCMUX, COEX, transparent cascade, clock-and-async) ===

    // DCMUX -- two independently-clocked masters merged into one output Q. Each clock's RISING edge is a
    // generation at Q (Q self-loops only when both clocks are low, and each rise takes it transparent to
    // that clock's master), so both rises carry `-type edge`. But the FALLS are combinational: nothing
    // generates on a fall (each master generates on its own rise), and Q switching away delivers the OTHER
    // clock's held value, arriving regardless. With no fall seam to hold against, the seam fixpoint empties
    // Q's set — an in-phase fall of the co-resident clock is a non-seam change — so Q is NOT an edge
    // register; it models as level rows with edge-labelled rises. The internal masters carry no arcs.
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
    fn edge_dcmux_level_model_with_edge_labelled_rises() {
        with_machine!(DCMUX_TOML, |_b, _a, _m2, m| {
            let es = classify(&m);
            assert_captures_faithful(&m, &es);
            // The seam fixpoint empties Q's set (its falls are combinational, so each in-phase fall is a
            // non-seam change), so NOTHING is an edge register — the cell is a level model.
            assert!(
                es.captures.is_empty(),
                "DCMUX models as level rows, no edge register, got {:?}",
                node_list(&es)
            );
            // The two rises stay `-type edge` (generation at Q); the falls are combinational, so no fall
            // label. Q carries exactly the two rising edge labels.
            assert_eq!(
                label_list(&es),
                [("Q", "CLKA", Edge::Rise), ("Q", "CLKB", Edge::Rise)],
                "rises edge-labelled, falls combinational"
            );
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

    // === Hierarchical master-slave across two clocks (HPIPE) ===

    // HPIPE -- a CLKA rising-edge master pair (M1/M2 capture D on CLKA) feeding a CLKB slave latch on Q (a
    // derived/gated-clock chain). Every hierarchically-related clock's edge arcs survive: the slave Q keeps
    // both CLKA and CLKB, the master node M2 keeps CLKA, and no seam set is emptied.
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
        // * CLKB:Fall is Q's OWN latch opening — Q holds M2 in CLKB=1 and reveals it on the fall, a
        //   first-class seam with its own capture (M2). The replay harness predicts this reveal directly,
        //   with no exemption.
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
    // variable that the pass-transistor style does not have. That complement is the node's exact negation
    // on every reachable state — it moves only in lockstep with the node it complements — which is exactly
    // what keeps the arc sets invariant.

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
        for state in m
            .explored
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
            // The complement node is the node's negation on every reachable state — it moves only in
            // lockstep with the node it complements, so it does not disqualify the CLKA edge.
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

    // === Openings: a latch with no capture still carries an edge arc ===
    //
    // An opening is the clock edge that takes a node from OPAQUE to TRANSPARENT: data that changed while
    // the node was closed is delivered BY THAT EDGE, and the delivered value then TRACKS its data rather
    // than holding. It is an edge arc like any other — a latch has no capture but it does have an opening,
    // so it is not timing-invisible.
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
        // real edge arc — GENERATION at the output, `Q` opaque at `CLK=0` and transparent at `CLK=1` — so
        // the CLK-rise arc is labelled edge while `Q`'s seam empties (it tracks `D` in its open phase).
        with_machine!(DLAT_TOML, |_b, _a, _m2, m| {
            let es = classify(&m);
            assert!(es.captures.is_empty(), "a latch has no capture");
            assert_eq!(label_list(&es), [("Q", "CLK", Edge::Rise)]);
        });

        // A transparent-LOW cascade is the fall-mirror of `DLAT`: `Q` is opaque across the high phase and
        // transparent through the low phase, so it GENERATES on the fall — the CLK-fall opening is a real
        // edge arc labelled on the output — while its seam empties (it tracks `M`/`D` once open) so it keeps
        // NO capture. The internal master carries no delay arc, so the label set is exactly `Q`'s fall.
        with_machine!(TCASC_TOML, |_b, _a, _m2, m| {
            let es = classify(&m);
            assert!(
                es.captures.is_empty(),
                "a transparent cascade has no capture"
            );
            assert_eq!(
                label_list(&es),
                [("Q", "CLK", Edge::Fall)],
                "the transparent-low cascade opens on the fall — a generation on the output"
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
        // Two independently-clocked masters merged into one output. Each clock's RISING edge is a
        // GENERATION at Q (Q opaque only when both clocks are low, transparent to that clock's master on the
        // rise) — edge — but the falls have no generator in Q's cone and no surviving source (the mux
        // delivers the OTHER clock's held value, arriving regardless), so they emit `-type combinational`.
        // The internal masters carry no delay arc and take no key.
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

    // === SETLR: a constant is a legitimate latched value ===

    // Y opens on the rising edge into the CLK=1 phase where it delivers !R — a value that VARIES with the
    // async clear R, so the phase is live and the rise genuinely generates. The latched value happens to be
    // the constant 1 (gated by the clear), which is a value like any other.
    const SETLR_TOML: &str = r#"
[[cell]]
name = "SETLR"
inputs = ["CLK", "R"]
clock = ["CLK"]
async = ["R"]
[cell.outputs]
Y = "!R*(CLK + !CLK*Y)"
"#;

    // The hard-constant twin: the CLK=1 phase pins Y to 1 EVERYWHERE (no live variation), so the delivered
    // value arrives regardless of latch content — a forcing by the clock's own level, not an opening. The
    // rise types combinational.
    const SETLR_HARD_TOML: &str = r#"
[[cell]]
name = "SETLRN"
inputs = ["CLK", "R"]
clock = ["CLK"]
async = ["R"]
[cell.outputs]
Y = "CLK + !CLK*!R*Y"
"#;

    #[test]
    fn edge_setlr_registers_the_constant_hard_constant_twin_is_combinational() {
        // The positive form: Y is an edge register, the rise labelled edge, the async R its off-edge clear.
        with_machine!(SETLR_TOML, |builder, _a, _m2, m| {
            let es = classify(&m);
            assert_captures_faithful(&m, &es);
            let y = reg(&es, "Y");
            assert_eq!(clocks_of(y), ["CLK"]);
            assert_eq!(y.captures.len(), 1);
            assert_eq!(y.captures[0].1, Edge::Rise, "Y generates on the rise");
            assert_eq!(
                labels_of(&es, "Y"),
                [("CLK", Edge::Rise)],
                "the rise types edge — a live delivered phase"
            );
            let off = builder.build_cover(&y.off_edge.off_cover);
            assert!(
                off.equivalent_to(&builder.var("R")),
                "off_edge.off is the async clear R"
            );
        });
        // The hard-constant twin: nothing generates into the clock-pinned phase, so Y stays combinational
        // — no register, no edge label.
        with_machine!(SETLR_HARD_TOML, |_b, _a, _m2, m| {
            let es = classify(&m);
            assert_captures_faithful(&m, &es);
            assert!(
                !node_list(&es).contains(&"Y"),
                "a clock-level-pinned phase is a forcing, not a register: {:?}",
                node_list(&es)
            );
            assert!(
                es.labels.is_empty(),
                "the hard-constant twin's rise is combinational: {:?}",
                label_list(&es)
            );
        });
    }

    // === The pseudo-latch probe: self-referential in the equation, never bistable in behaviour ===

    // `Y*A*B` is a self-reference in the equation, but Y never sits on a live dependency cycle that opens
    // across a clock edge — no phase takes Y opaque→transparent — so it never generates and stays
    // combinational. Generation is behavioural bistability, not textual self-reference.
    const PSEUDO_LATCH_TOML: &str = r#"
[[cell]]
name = "PLAT"
inputs = ["CLK", "A", "B"]
clock = ["CLK"]
[cell.outputs]
Y = "CLK*A + !CLK*B + Y*A*B"
"#;

    #[test]
    fn edge_pseudo_latch_probe_never_generates() {
        with_machine!(PSEUDO_LATCH_TOML, |_b, _a, _m2, m| {
            let es = classify(&m);
            assert_captures_faithful(&m, &es);
            assert!(
                !node_list(&es).contains(&"Y"),
                "the pseudo-latch never generates — no register: {:?}",
                node_list(&es)
            );
            assert!(
                es.labels.is_empty(),
                "every arc is combinational: {:?}",
                label_list(&es)
            );
        });
    }

    // === Per-arc sibling splits: one (output, clock, direction) typing differently by context ===

    // The distinct-type siblings on Y's CLK Fall, keyed by the mask column `mask`: where the fall reveals
    // the opening latch `L` the arc types EDGE, where the mask cuts `L` out of the residual it types
    // COMBINATIONAL — the fall then moves Y only through the CLK level. Both siblings share the vector and
    // differ only in prevector and type, which is legal in Liberate.
    fn assert_fall_splits_on_mask(
        src: &str,
        mask: &str,
        edge_when: bool, // the mask value at which the fall reveals `L`
    ) {
        with_machine!(src, |_b, _a, _m2, m| {
            let (arcs, _) = crate::logic::arcs::derive(&m);
            let es = classify(&m);
            assert_captures_faithful(&m, &es);
            let mut edge_ctx = 0;
            let mut comb_ctx = 0;
            for a in &arcs {
                if a.output.as_str() != "Y"
                    || a.related.as_str() != "CLK"
                    || a.end.value_of("CLK") != Some(false)
                {
                    continue; // only Y's CLK Fall arcs
                }
                let labelled = es.labels.contains(&(
                    a.output.clone(),
                    a.related.clone(),
                    Edge::Fall,
                    a.start.clone(),
                ));
                match a.start.value_of(mask) {
                    Some(v) if v == edge_when => {
                        assert!(
                            labelled,
                            "with {mask} admitting L the fall reveals it and types edge: {a:?}"
                        );
                        edge_ctx += 1;
                    }
                    Some(_) => {
                        assert!(
                            !labelled,
                            "with {mask} masking L the fall types combinational: {a:?}"
                        );
                        comb_ctx += 1;
                    }
                    None => {}
                }
            }
            assert!(
                edge_ctx > 0 && comb_ctx > 0,
                "Y's CLK Fall must split into an edge sibling and a combinational one \
                 (edge={edge_ctx}, comb={comb_ctx})"
            );
        });
    }

    // A masked latch: `L` opens on the fall, `Y = CLK*A + B*L` gates it behind the AND-mask B. The fall
    // residual over all-but-`L` is `L` when B=1 (edge) and 0 when B=0 (combinational, the fall moving Y
    // through the dropping `CLK*A` term).
    const MASKL_TOML: &str = r#"
[[cell]]
name = "MASKL"
inputs = ["CLK", "D", "A", "B"]
clock = ["CLK"]
[cell.internal]
L = "!CLK*D + CLK*L"
[cell.outputs]
Y = "CLK*A + B*L"
"#;

    // An OR-masked latch: `Y = !CLK*R + L`. The fall residual over all-but-`L` is `L` when R=0 (edge) and
    // the constant 1 when R=1 — the OR-mask swallows the opening, so the fall types combinational.
    const ORLAT_TOML: &str = r#"
[[cell]]
name = "ORLAT"
inputs = ["CLK", "D", "R"]
clock = ["CLK"]
[cell.internal]
L = "!CLK*D + CLK*L"
[cell.outputs]
Y = "!CLK*R + L"
"#;

    #[test]
    fn edge_masked_latch_fall_arc_splits_per_context() {
        assert_fall_splits_on_mask(MASKL_TOML, "B", true);
    }

    #[test]
    fn edge_or_masked_latch_fall_arc_splits_per_context() {
        assert_fall_splits_on_mask(ORLAT_TOML, "R", false);
    }

    // === Source scoping: DET exposes a surviving source, a clock gate reaches none ===

    #[test]
    fn edge_source_scoping_det_dual_edge_versus_clock_gate_none() {
        // DET's Q exposes a K-associated latch's content on BOTH edges (`Q = L1` on the rise, `L2` on the
        // fall), so both CLK edges have a surviving propagation source and type edge.
        with_machine!(DET_TOML, |_b, _a, _m2, m| {
            let es = classify(&m);
            assert_captures_faithful(&m, &es);
            let labs = label_list(&es);
            assert!(
                labs.contains(&("Q", "CLK", Edge::Rise))
                    && labs.contains(&("Q", "CLK", Edge::Fall))
                    && labs.len() == 2,
                "DET exposes D on both edges: {labs:?}"
            );
        });
        // ICG's GCLK reaches no surviving source — the captured `EL` is swallowed by the `CLK*EL` gate at
        // the destination — so neither clock edge types edge.
        with_machine!(ICG_TOML, |_b, _a, _m2, m| {
            let es = classify(&m);
            assert_captures_faithful(&m, &es);
            assert!(
                es.labels.is_empty(),
                "ICG's GCLK is combinational — the opening latch is masked: {:?}",
                label_list(&es)
            );
        });
        // ICM's GCLK likewise: the competing enable is not the toggled clock's associate, and the branch's
        // own enable is masked at the destination, so no candidate source survives on either clock.
        with_machine!(ICM_TOML, |_b, _a, _m2, m| {
            let es = classify(&m);
            assert_captures_faithful(&m, &es);
            assert!(
                es.labels.is_empty(),
                "ICM's GCLK is combinational — no clock-associated source survives: {:?}",
                label_list(&es)
            );
        });
    }

    // === Birth + transitive propagation: a generation reveal types edge at any pipe depth ===

    // A rising-edge flop written as a two-latch same-clock pipe: Q (transparent while K2=0) captures D on
    // the rise, Y (transparent while K2=1) reveals it. Y generates on the rise.
    const PIPE2_TOML: &str = r#"
[[cell]]
name = "PIPE2"
inputs = ["K2", "D"]
clock = ["K2"]
[cell.internal]
Q = "!K2*D + K2*Q"
[cell.outputs]
Y = "K2*Q + !K2*Y"
"#;

    // The same reveal one stage deeper: Q → T → Y, with T (transparent while K2=1) between the master and
    // the output. Y still generates on the rise; the birth propagates through the extra stage with no depth
    // limit, so Y types EDGE identically to PIPE2. (PIPE2's `Q` is the master; PIPE3 inserts a same-phase
    // slave `T` before `Y`.)
    const PIPE3_TOML: &str = r#"
[[cell]]
name = "PIPE3"
inputs = ["K2", "D"]
clock = ["K2"]
[cell.internal]
Q = "!K2*D + K2*Q"
T = "K2*Q + !K2*T"
[cell.outputs]
Y = "K2*T + !K2*Y"
"#;

    #[test]
    fn edge_birth_propagation_is_depth_invariant() {
        // The reveal types EDGE at any pipe depth: propagation is a single transitive restriction-survival
        // chain with no cutoff, so the shallow pipe (PIPE2) and the one-deeper pipe (PIPE3) carry the SAME
        // `Y@K2:Rise` edge label. Each is proven against the machine by the replay harness.
        let y_labels = |src: &str| -> Vec<(String, Edge)> {
            with_machine!(src, |_b, _a, _m2, m| {
                let es = classify(&m);
                assert_captures_faithful(&m, &es);
                labels_of(&es, "Y")
                    .into_iter()
                    .map(|(c, e)| (c.to_string(), e))
                    .collect()
            })
        };
        let pipe2 = y_labels(PIPE2_TOML);
        let pipe3 = y_labels(PIPE3_TOML);
        assert_eq!(
            pipe2,
            [("K2".to_string(), Edge::Rise)],
            "shallow pipe reveals on the rise"
        );
        assert_eq!(
            pipe3, pipe2,
            "the one-deeper pipe types identically — propagation is depth-invariant"
        );
    }

    // === Generation and propagation coerce no state: no state-coercion identifier survives ===

    #[test]
    fn no_state_coercion_identifier_survives_in_src() {
        // Generation and propagation type edges purely by restriction-survival over the machine's own
        // stable states, coercing no state. This gate proves that discipline is intact: no code identifier
        // names a state-perturbation mechanism. Each needle is assembled from two halves so the gate never
        // matches its own source, and only CODE is scanned — the part of each line before `//` — so the
        // ordinary physical word (a phase clamped to a constant is a forcing) survives in prose.
        let needles: Vec<String> = [
            ("fr", "eeze"),
            ("fro", "zen"),
            ("per", "turb"),
            ("cl", "amp"),
            ("co-mo", "ver"),
        ]
        .iter()
        .map(|(a, b)| format!("{a}{b}"))
        .collect();

        fn rs_files(dir: &std::path::Path, out: &mut Vec<std::path::PathBuf>) {
            for entry in std::fs::read_dir(dir).expect("read_dir src") {
                let path = entry.expect("dir entry").path();
                if path.is_dir() {
                    rs_files(&path, out);
                } else if path.extension().is_some_and(|e| e == "rs") {
                    out.push(path);
                }
            }
        }

        let src = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        let mut files = Vec::new();
        rs_files(&src, &mut files);
        assert!(!files.is_empty(), "no .rs files found under {src:?}");

        let mut hits: Vec<String> = Vec::new();
        for path in &files {
            let text = std::fs::read_to_string(path).expect("read src file");
            for (i, line) in text.lines().enumerate() {
                let code = line.split("//").next().unwrap_or("").to_ascii_lowercase();
                for needle in &needles {
                    if code.contains(needle.as_str()) {
                        hits.push(format!("{}:{}: {}", path.display(), i + 1, line.trim()));
                    }
                }
            }
        }
        assert!(
            hits.is_empty(),
            "a state-coercion identifier survives in src (generation/propagation coerce no state):\n{}",
            hits.join("\n")
        );
    }
}
