//! Master-slave latch → edge-register recognition.
//!
//! A cell modelled as two opposite-phase level-sensitive latches in series (a transparent-low master
//! feeding a transparent-high slave, or vice versa, on the **same declared clock**) is edge-equivalent
//! to a single edge-triggered register. This module recognises that shape in a **post-exploration**
//! pass and re-expresses the already-derived behaviour as edge annotations — it never re-derives it.
//! The two-latch model stays the source of truth: [`recognise_edge_registers`] is strictly READ-ONLY
//! over the shared per-cell BDD map and mutates nothing.
//!
//! Recognition is **functional from the start** and keys off **declared clocks only**: a cell without a
//! declared clock never collapses. A candidate pair is nominated structurally (opposite-phase latches on
//! one clock, master feeding the slave's transparent cofactor) and then confirmed by exact BDD equality
//! of the cofactor edge-equivalence conditions (F1/F2 below) — one shared per-cell builder makes `==`
//! an exact function-equality test.
//!
//! # Latch classification
//!
//! A signal `s` with transition function `δ_s` is a **latch** w.r.t. a declared clock `c` with
//! transparency phase `p` iff `c ∈ vars(δ_s)`, the transparent cofactor `T_s = δ_s|c=p` does **not**
//! reference `s`, and the hold cofactor `H_s = δ_s|c=¬p` **does**. A signal latch-shaped w.r.t. two or
//! more declared clocks is rejected (not a latch).
//!
//! # Pairing and confirmation
//!
//! `(m master, s slave)` are paired when both are latches on the **same** `c` with **opposite** phases
//! and `m ∈ vars(T_s)`. The guards, all exact BDD identities:
//!
//! * **G2** `m ∉ vars(H_s)` — the master only feeds the slave's transparent path.
//! * **G3** `s ∉ vars(δ_m)` — no reverse dependency.
//! * **G5** monotone hold for both: `H|x=0 ∧ ¬H|x=1 == false` (the hold cofactor is monotone in the
//!   latch's own variable).
//! * **F1** `T_s|m=0 == H_s|s=0 ∧ T_s|m=1 == H_s|s=1` — the captured-through value matches the held one.
//! * **F2** `H_m|m=0 == H_s|s=0 ∧ H_m|m=1 == H_s|s=1` — master and slave hold the same value.
//!
//! # Fold eligibility and annotation
//!
//! A master `m` is **foldable** when it is internal (not an output pin), its sole consumer is `s` (no
//! other surviving signal references it), and it is **not itself a slave** (no same-clock opposite-phase
//! latch in `vars(T_m)`). That last clause is the shared-boundary crux: in the ICM synchronisers `sela1`
//! feeds `sela2` feeds `enA`, so `sela2` is both a slave (of `sela1`) and the master of `enA` — it must
//! **survive** rather than be greedily folded into `enA`.
//!
//! Annotation runs a worklist over the signals in order to a fixpoint. A slave `s` whose pair passes all
//! guards is annotated when either its master is foldable (→ `folded_master = Some(m)`, capture
//! `Cap = T_s[m := T_m]`) or its master is already annotated as a slave in its own right (→
//! `folded_master = None`, `Cap = T_s` unchanged). A chain head that can neither fold nor be annotated —
//! an exposed, tapped, multi-consumer or undeclared-clock master — propagates nothing, so the whole cell
//! emits unchanged.

use std::collections::{BTreeMap, BTreeSet};

use espresso_logic::bdd::{Bdd, Brand, ManagerCell};
use espresso_logic::Symbol;

use crate::logic::arcs::Edge;
use crate::logic::regions::{state_regions, StateRegions};

/// One recognised edge-triggered register: an opposite-phase latch pair on `clock` re-expressed as an
/// edge seam on `node`.
#[derive(Debug, Clone)]
pub struct EdgeRegister {
    /// The slave signal that becomes the register's output coordinate.
    pub node: Symbol,
    /// The declared clock the pair keys off.
    pub clock: Symbol,
    /// The active clock edge: `Rise` for a transparent-high slave (capture at the rising seam), `Fall`
    /// for a transparent-low one.
    pub edge: Edge,
    /// The register's column set: the first-appearance union of `capture.cols` then `off_edge.cols`.
    pub cols: Vec<Symbol>,
    /// The captured (next-state) function as combinational state-table regions (on/off covers, empty
    /// hold); never references `clock`.
    pub capture: StateRegions,
    /// The off-edge (hold) function as state-table regions: on/off are the async set/clear covers, hold
    /// is the quiescent region; never references `clock`.
    pub off_edge: StateRegions,
    /// The pure master latch folded into this register, when it was foldable — `Some(m)`; `None` when
    /// the master survives as a register in its own right (a shared-boundary node).
    pub folded_master: Option<Symbol>,
}

/// A signal classified as a level-sensitive latch on one declared clock.
struct Latch<B: Brand, C: ManagerCell> {
    clock: Symbol,
    /// Transparency phase: the clock value at which the latch is transparent.
    phase: bool,
    /// The transparent cofactor `T = δ|clock=phase` (does not reference the signal).
    transparent: Bdd<B, C>,
    /// The hold cofactor `H = δ|clock=¬phase` (references the signal).
    hold: Bdd<B, C>,
}

/// Recognise every master-slave latch pair in the minimised model as an edge register. Strictly
/// READ-ONLY: it reads the shared per-cell BDD map and mutates nothing.
///
/// `order` is the post-minimise `signals()` order, `outputs` the external-output names, and `clocks` the
/// cell's declared clock pins — recognition keys off these only, so a cell with no declared clock yields
/// an empty result.
pub fn recognise_edge_registers<B: Brand, C: ManagerCell>(
    bdds: &BTreeMap<Symbol, Bdd<B, C>>,
    order: &[Symbol],
    outputs: &BTreeSet<Symbol>,
    clocks: &[Symbol],
) -> Vec<EdgeRegister> {
    // Classify every surviving signal as a latch (or not) once, up front.
    let latches: BTreeMap<Symbol, Latch<B, C>> = order
        .iter()
        .filter_map(|s| classify_latch(s, &bdds[s], clocks).map(|l| (s.clone(), l)))
        .collect();

    // Worklist over the signals in order to a fixpoint: a slave whose master is not yet foldable-or-
    // annotated waits for a later round (e.g. ICM's `enA` waits until `sela2` is annotated).
    let mut annotated: BTreeMap<Symbol, EdgeRegister> = BTreeMap::new();
    loop {
        let mut changed = false;
        for s in order {
            if annotated.contains_key(s) {
                continue;
            }
            let Some(ls) = latches.get(s) else { continue };
            let Some(m) = valid_master(s, ls, &latches, bdds) else {
                continue;
            };
            let lm = &latches[&m];
            let (folded_master, cap) = if foldable(&m, s, &latches, bdds, outputs) {
                // Fold the pure master away: capture the slave-transparent value with the master's own
                // transparent value substituted in.
                (
                    Some(m.clone()),
                    ls.transparent.compose(m.as_str(), &lm.transparent),
                )
            } else if annotated.contains_key(&m) {
                // The master survives as its own register (a shared boundary): keep the slave's
                // transparent cofactor as the capture, folding nothing.
                (None, ls.transparent.clone())
            } else {
                // The master can neither fold nor is (yet) a register — leave `s` for a later round.
                continue;
            };

            // Rise for a transparent-high slave (capture at the rising seam), Fall for transparent-low.
            let edge = if ls.phase { Edge::Rise } else { Edge::Fall };
            // Capture is combinational (empty hold); off-edge carries the async set/clear + quiescent
            // hold. Neither cofactor references the clock (it was projected out).
            let capture = state_regions(s, &cap, false);
            let off_edge = state_regions(s, &ls.hold, true);
            let cols = union_cols(&capture.cols, &off_edge.cols);

            annotated.insert(
                s.clone(),
                EdgeRegister {
                    node: s.clone(),
                    clock: ls.clock.clone(),
                    edge,
                    cols,
                    capture,
                    off_edge,
                    folded_master,
                },
            );
            changed = true;
        }
        if !changed {
            break;
        }
    }

    // Emit the registers in signals order.
    order.iter().filter_map(|s| annotated.remove(s)).collect()
}

/// Classify `name` (with function `f`) as a latch on one declared clock, or `None`. A signal latch-shaped
/// w.r.t. two or more declared clocks is rejected.
fn classify_latch<B: Brand, C: ManagerCell>(
    name: &Symbol,
    f: &Bdd<B, C>,
    clocks: &[Symbol],
) -> Option<Latch<B, C>> {
    let b = f.builder();
    let vars: BTreeSet<Symbol> = f.variables().collect();
    let mut found: Option<Latch<B, C>> = None;
    let mut count = 0usize;
    for c in clocks {
        if !vars.contains(c) {
            continue;
        }
        // At most one phase can be transparent-without-self for a given clock (both would force the
        // hold cofactor to be self-free too), so the first match settles this clock.
        for &p in &[true, false] {
            let transparent = f.compose(c.as_str(), &b.constant(p));
            let hold = f.compose(c.as_str(), &b.constant(!p));
            let t_self = transparent.variables().any(|v| v == *name);
            let h_self = hold.variables().any(|v| v == *name);
            if !t_self && h_self {
                count += 1;
                found = Some(Latch {
                    clock: c.clone(),
                    phase: p,
                    transparent,
                    hold,
                });
                break;
            }
        }
    }
    if count >= 2 {
        return None;
    }
    found
}

/// The master of slave latch `s`, confirmed against all guards (G2, G3, G5, F1, F2), or `None`. Searches
/// `vars(T_s)` for a latch on the same clock with the opposite phase.
fn valid_master<B: Brand, C: ManagerCell>(
    s: &Symbol,
    ls: &Latch<B, C>,
    latches: &BTreeMap<Symbol, Latch<B, C>>,
    bdds: &BTreeMap<Symbol, Bdd<B, C>>,
) -> Option<Symbol> {
    let b = ls.transparent.builder();
    let hs_s0 = ls.hold.compose(s.as_str(), &b.constant(false));
    let hs_s1 = ls.hold.compose(s.as_str(), &b.constant(true));
    for m in ls.transparent.variables().filter(|v| v != s) {
        let Some(lm) = latches.get(&m) else { continue };
        if lm.clock != ls.clock || lm.phase == ls.phase {
            continue; // must be the same clock, opposite phase
        }
        // G2: the master only feeds the transparent path, never the hold.
        if ls.hold.variables().any(|v| v == m) {
            continue;
        }
        // G3: no reverse dependency of the master on the slave.
        if bdds[&m].variables().any(|v| v == *s) {
            continue;
        }
        // G5: monotone hold for both latches (in each latch's own variable).
        if !monotone_hold(&ls.hold, s) || !monotone_hold(&lm.hold, &m) {
            continue;
        }
        // F1: the captured-through value equals the held value.
        let ts_m0 = ls.transparent.compose(m.as_str(), &b.constant(false));
        let ts_m1 = ls.transparent.compose(m.as_str(), &b.constant(true));
        if ts_m0 != hs_s0 || ts_m1 != hs_s1 {
            continue;
        }
        // F2: master and slave hold the same value.
        let hm_m0 = lm.hold.compose(m.as_str(), &b.constant(false));
        let hm_m1 = lm.hold.compose(m.as_str(), &b.constant(true));
        if hm_m0 != hs_s0 || hm_m1 != hs_s1 {
            continue;
        }
        return Some(m);
    }
    None
}

/// Whether the master `m` of slave `s` can be folded away: internal, sole-consumed by `s`, and not
/// itself a slave (no same-clock opposite-phase latch in `vars(T_m)`).
fn foldable<B: Brand, C: ManagerCell>(
    m: &Symbol,
    s: &Symbol,
    latches: &BTreeMap<Symbol, Latch<B, C>>,
    bdds: &BTreeMap<Symbol, Bdd<B, C>>,
    outputs: &BTreeSet<Symbol>,
) -> bool {
    if outputs.contains(m) {
        return false; // an exposed (output) master is tapped externally — never folded
    }
    // Sole consumer is `s`: no other surviving signal references `m` (its own self-hold aside).
    let sole = bdds
        .iter()
        .all(|(k, f)| k == m || k == s || !f.variables().any(|v| v == *m));
    if !sole {
        return false;
    }
    // A master that is itself a slave (shared boundary) must survive as its own register.
    !has_master_structural(m, latches)
}

/// Whether latch `m` is itself a slave: some signal in `vars(T_m)` is a latch on the same clock with the
/// opposite phase. Purely structural — the shared-boundary check, not a full guard confirmation.
fn has_master_structural<B: Brand, C: ManagerCell>(
    m: &Symbol,
    latches: &BTreeMap<Symbol, Latch<B, C>>,
) -> bool {
    let Some(lm) = latches.get(m) else {
        return false;
    };
    lm.transparent.variables().filter(|v| v != m).any(|v| {
        latches
            .get(&v)
            .is_some_and(|l| l.clock == lm.clock && l.phase != lm.phase)
    })
}

/// Whether the hold cofactor `h` is monotone in `x`: `h|x=0 ∧ ¬h|x=1 == false`.
fn monotone_hold<B: Brand, C: ManagerCell>(h: &Bdd<B, C>, x: &Symbol) -> bool {
    let b = h.builder();
    let h0 = h.compose(x.as_str(), &b.constant(false));
    let h1 = h.compose(x.as_str(), &b.constant(true));
    h0.and(&!&h1).is_contradiction()
}

/// The first-appearance union of two column lists (`a` first, then `b`'s new entries).
fn union_cols(a: &[Symbol], b: &[Symbol]) -> Vec<Symbol> {
    let mut cols = a.to_vec();
    for s in b {
        if !cols.contains(s) {
            cols.push(s.clone());
        }
    }
    cols
}

#[cfg(test)]
mod tests {
    use super::*;
    use espresso_logic::bdd_builder;

    /// Build a signal map from `(name, expr)` pairs in a fresh builder, plus the scan order, the output
    /// set and the declared clocks. Mirrors `minimise.rs`'s `system!` fixture idiom, extended with the
    /// clock list recognition keys off.
    macro_rules! system {
        (outputs: [$($out:literal),* $(,)?], clocks: [$($clk:literal),* $(,)?], $($name:literal = $expr:literal),* $(,)?) => {{
            let b = bdd_builder!();
            let mut bdds: BTreeMap<Symbol, _> = BTreeMap::new();
            let mut order: Vec<Symbol> = Vec::new();
            $(
                let nm = Symbol::from($name);
                bdds.insert(nm.clone(), b.parse($expr).unwrap());
                order.push(nm);
            )*
            let outputs: BTreeSet<Symbol> = [$(Symbol::from($out)),*].into_iter().collect();
            let clocks: Vec<Symbol> = vec![$(Symbol::from($clk)),*];
            (b, bdds, order, outputs, clocks)
        }};
    }

    #[test]
    fn dff_collapses_to_one_rise_register_with_folded_master() {
        // Master M (transparent-low) feeds slave Q (transparent-high) on CLK: one rising-edge register
        // Q, folding M away, capturing D.
        let (_b, bdds, order, outputs, clocks) = system! {
            outputs: ["Q"], clocks: ["CLK"],
            "Q" = "CLK*M + !CLK*Q",
            "M" = "!CLK*D + CLK*M",
        };
        let regs = recognise_edge_registers(&bdds, &order, &outputs, &clocks);
        assert_eq!(regs.len(), 1);
        let q = &regs[0];
        assert_eq!(q.node, "Q");
        assert_eq!(q.clock, "CLK");
        assert_eq!(q.edge, Edge::Rise);
        assert_eq!(q.folded_master, Some(Symbol::from("M")));
        // Cap == D: the combinational capture region is exactly D over the single column D.
        assert_eq!(
            q.capture
                .cols
                .iter()
                .map(Symbol::as_str)
                .collect::<Vec<_>>(),
            ["D"]
        );
        assert_eq!(q.capture.on, vec![vec![Some(true)]]);
        assert_eq!(q.capture.off, vec![vec![Some(false)]]);
        assert!(q.capture.hold.is_empty());
    }

    #[test]
    fn icm_synchronisers_recognise_the_shared_boundary_registers() {
        // Post-minimise ICM (sela/selb relays already folded). Each synchroniser is a three-latch chain
        // sela1 → sela2 → enA: sela2 is BOTH a slave of sela1 and the master of enA — the greedy trap.
        let (_b, bdds, order, outputs, clocks) = system! {
            outputs: ["GCLK"], clocks: ["CLKA", "CLKB"],
            "GCLK" = "enA*CLKA+enB*CLKB",
            "sela1" = "!RA*(!CLKA*(!enB*!S)+CLKA*sela1)",
            "sela2" = "!RA*(CLKA*sela1+!CLKA*sela2)",
            "enA"   = "!RA*(!CLKA*sela2+CLKA*enA)",
            "selb1" = "!RB*(!CLKB*(!enA*S)+CLKB*selb1)",
            "selb2" = "!RB*(CLKB*selb1+!CLKB*selb2)",
            "enB"   = "!RB*(!CLKB*selb2+CLKB*enB)",
        };
        let regs = recognise_edge_registers(&bdds, &order, &outputs, &clocks);
        let by: BTreeMap<&str, &EdgeRegister> = regs.iter().map(|r| (r.node.as_str(), r)).collect();

        assert_eq!(regs.len(), 4);
        // sela2 survives as a rising register that folds sela1 — NOT greedily folded into enA.
        let sela2 = by["sela2"];
        assert_eq!(sela2.edge, Edge::Rise);
        assert_eq!(sela2.clock, "CLKA");
        assert_eq!(sela2.folded_master, Some(Symbol::from("sela1")));
        // enA is a falling register that folds NOTHING (its master sela2 is itself a slave).
        let ena = by["enA"];
        assert_eq!(ena.edge, Edge::Fall);
        assert_eq!(ena.clock, "CLKA");
        assert_eq!(ena.folded_master, None);
        // The CLKB synchroniser mirrors it.
        let selb2 = by["selb2"];
        assert_eq!(selb2.edge, Edge::Rise);
        assert_eq!(selb2.folded_master, Some(Symbol::from("selb1")));
        let enb = by["enB"];
        assert_eq!(enb.edge, Edge::Fall);
        assert_eq!(enb.folded_master, None);
        // The folded masters and the output are not themselves registers.
        assert!(!by.contains_key("sela1"));
        assert!(!by.contains_key("selb1"));
        assert!(!by.contains_key("GCLK"));
    }

    #[test]
    fn single_latch_has_no_pair() {
        // A lone transparent latch has no master → no register.
        let (_b, bdds, order, outputs, clocks) = system! {
            outputs: ["Q"], clocks: ["CLK"],
            "Q" = "CLK*D + !CLK*Q",
        };
        assert!(recognise_edge_registers(&bdds, &order, &outputs, &clocks).is_empty());
    }

    #[test]
    fn gated_latch_transparent_referencing_self_is_not_a_latch() {
        // Q|CLK=1 = D+Q still references Q, so no phase is cleanly transparent → not a latch.
        let (_b, bdds, order, outputs, clocks) = system! {
            outputs: ["Q"], clocks: ["CLK"],
            "Q" = "CLK*(D+Q) + !CLK*Q",
        };
        assert!(recognise_edge_registers(&bdds, &order, &outputs, &clocks).is_empty());
    }

    #[test]
    fn multi_clock_latch_is_rejected() {
        // Q is latch-shaped w.r.t. both CLKA and CLKB → rejected, no register.
        let (_b, bdds, order, outputs, clocks) = system! {
            outputs: ["Q"], clocks: ["CLKA", "CLKB"],
            "Q" = "CLKA*D + !CLKA*(CLKB*D + !CLKB*Q)",
        };
        assert!(recognise_edge_registers(&bdds, &order, &outputs, &clocks).is_empty());
    }

    #[test]
    fn exposed_master_output_does_not_fold() {
        // The master M is also an output pin (tapped externally): not foldable, not a slave → the slave
        // Q cannot annotate, so the cell emits unchanged.
        let (_b, bdds, order, outputs, clocks) = system! {
            outputs: ["Q", "M"], clocks: ["CLK"],
            "Q" = "CLK*M + !CLK*Q",
            "M" = "!CLK*D + CLK*M",
        };
        assert!(recognise_edge_registers(&bdds, &order, &outputs, &clocks).is_empty());
    }

    #[test]
    fn tapped_master_with_two_consumers_does_not_fold() {
        // M feeds both Q and the extra output tap T: not the sole consumer → not foldable → no register.
        let (_b, bdds, order, outputs, clocks) = system! {
            outputs: ["Q", "T"], clocks: ["CLK"],
            "Q" = "CLK*M + !CLK*Q",
            "T" = "M",
            "M" = "!CLK*D + CLK*M",
        };
        assert!(recognise_edge_registers(&bdds, &order, &outputs, &clocks).is_empty());
    }

    #[test]
    fn inverting_handoff_fails_f1() {
        // Q = CLK*!M + !CLK*Q inverts the master on capture, so T_Q|m and H_Q|s disagree → F1 fails.
        let (_b, bdds, order, outputs, clocks) = system! {
            outputs: ["Q"], clocks: ["CLK"],
            "Q" = "CLK*!M + !CLK*Q",
            "M" = "!CLK*D + CLK*M",
        };
        assert!(recognise_edge_registers(&bdds, &order, &outputs, &clocks).is_empty());
    }

    #[test]
    fn master_only_reset_fails_f2() {
        // The master holds through an async reset (!R) the slave lacks, so H_M and H_Q diverge → F2 fails.
        let (_b, bdds, order, outputs, clocks) = system! {
            outputs: ["Q"], clocks: ["CLK"],
            "Q" = "CLK*M + !CLK*Q",
            "M" = "!R*(!CLK*D + CLK*M)",
        };
        assert!(recognise_edge_registers(&bdds, &order, &outputs, &clocks).is_empty());
    }

    #[test]
    fn undeclared_clock_never_collapses() {
        // The same DFF shape, but CLK is not a declared clock → nothing is a latch, no register.
        let (_b, bdds, order, outputs, clocks) = system! {
            outputs: ["Q"], clocks: [],
            "Q" = "CLK*M + !CLK*Q",
            "M" = "!CLK*D + CLK*M",
        };
        assert!(recognise_edge_registers(&bdds, &order, &outputs, &clocks).is_empty());
    }

    /// A two-latch DFF with a declared clock, opting collapse in or out via `no_edge_collapse`.
    const DFF_TOML: &str = r#"
[[cell]]
name = "DFF"
inputs = ["CLK", "D"]
clock = ["CLK"]
constraint_arcs = true
[cell.internal]
M = "!CLK*D + CLK*M"
[cell.outputs]
Q = "CLK*M + !CLK*Q"
"#;

    /// The ICM interlock: two three-latch synchronisers on CLKA/CLKB feeding GCLK.
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

    /// Parse a single-cell spec and analyse it, forcing `no_edge_collapse` on every cell.
    fn analyse_toggled(src: &str, no_collapse: bool) -> crate::model::AnalysedCell {
        let mut spec = crate::model::parse_spec(src).unwrap();
        for c in &mut spec.cells {
            c.no_edge_collapse = no_collapse;
        }
        spec.cells[0].analyse().unwrap()
    }

    /// PERMANENT guard on the CRITICAL INVARIANT: the collapse re-expresses already-explored behaviour
    /// and must change ONLY `edge_registers` — every other `AnalysedCell` field (the exploration,
    /// prevector/vector and hazard outputs) is byte-for-byte identical whether collapse is on or off.
    #[test]
    fn collapse_changes_only_the_edge_registers_field() {
        for src in [DFF_TOML, ICM_TOML] {
            let off = analyse_toggled(src, true); // collapse suppressed
            let on = analyse_toggled(src, false); // collapse active

            // Every exploration-derived field is identical (Debug-string equality across all of them
            // except `edge_registers`).
            macro_rules! unchanged {
                ($field:ident) => {
                    assert_eq!(
                        format!("{:?}", off.$field),
                        format!("{:?}", on.$field),
                        concat!("collapse changed AnalysedCell::", stringify!($field)),
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

            // The guard has teeth: collapse is a no-op when suppressed and does recognise registers on
            // these fixtures when active.
            assert!(off.edge_registers.is_empty());
            assert!(!on.edge_registers.is_empty());
        }
    }
}
