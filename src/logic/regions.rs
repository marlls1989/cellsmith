//! State-table / sequential-UDP region derivation.
//!
//! This is the **functional** view of an output, used by the Verilog UDP and the Liberty
//! `statetable` — distinct from the timing-arc view, which is produced by the state machine in
//! [`super::arcs`].
//!
//! The column set is exactly the pin function's **BDD support minus its own self-feedback** — every
//! signal (primary input, other output, or internal state node) the function actually depends on, and
//! nothing else. An input the function ignores never becomes a column. The pin's own self-feedback is
//! projected out and becomes the sequential element's current-state (`reg`) column.
//!
//! The three regions come from re-basing `f` onto that column set by **universal** projection of the
//! self var (`Bdd::cover_over_fr`, whose `F` side is `∀self. f` and `R` side `∀self. ¬f`). Because a
//! partial function's on- and off-sets are *not* complementary, the gap between them is the hold set:
//!
//! - `on   = ∀self. f`    — the `F` side of `f.cover_over_fr(cols)`,
//! - `off  = ∀self. ¬f`   — the `F` side of `(!f).cover_over_fr(cols)`,
//! - `hold = ¬(on ∨ off)` — the undef gap the two leave behind; a `-`/`N` no-change entry. A signal can
//!   be hysteretic (a state variable) yet have an *empty* hold set: cross-coupled emergent memory (a
//!   mutex or SR latch) holds via its coupling cycle. Hysteresis is decided by the machine's
//!   state-variable set (see `is_state` below), independent of hold emptiness.
//!
//! Each region is then Espresso-minimised **independently** as its own onset — safe because none
//! carries a don't-care set, so minimisation reproduces that exact region and never absorbs the hold
//! gap into on/off.
//!
//! The region view is derived from the **minimised model's** folded BDD, taken from the shared per-cell
//! builder — a purged relay/alias internal has no region.
//!
//! The two-view split, the column/self-feedback rule and the independent-region minimisation argument are
//! described concept-first in `state-table-regions.md`; this module records only the implementation
//! shape.
//!
//! **Emission plumbing:** `state_regions` is called once per signal from `Cell::analyse`, cached on
//! `AnalysedCell::regions` in `signals()` order (see `model.rs`); the Verilog and Liberty emitters read
//! that cache rather than rebuilding the BDDs.

use espresso_logic::bdd::{Bdd, Brand, ManagerCell};
use espresso_logic::{Anonymous, Cover, CoverType, CubeType, Minimizable, Symbol};

/// One cube over the state-table/UDP column set: `Some(true)`/`Some(false)` for a fixed column,
/// `None` for a don't-care. Aligned position-by-position to [`StateRegions::cols`].
pub(crate) type StateCube = Vec<Option<bool>>;

/// The regions of a signal as they appear in a **state table / sequential UDP**.
///
/// Each region is a set of don't-care cubes (from the BDD's prime paths), so a variable a cube does
/// not constrain prints as `?`/`-`.
#[derive(Debug, Clone)]
pub struct StateRegions {
    /// Input columns: the pin function's BDD support minus its own self-feedback, in BDD variable
    /// order. Every other signal the function references (another output or an internal node) that it
    /// actually depends on appears here; inputs the function ignores do not.
    pub(crate) cols: Vec<Symbol>,
    pub(crate) on: Vec<StateCube>,
    pub(crate) off: Vec<StateCube>,
    pub(crate) hold: Vec<StateCube>,
    /// The minimised on-region as a single-output F cover over `cols` (the same cover the `on` cubes
    /// are read from). Kept so the joint state-table build can re-base and stack it into a
    /// multi-output cover without rebuilding any BDD.
    pub(crate) on_cover: Cover<Symbol, Anonymous>,
    /// The minimised off-region cover, twin of [`on_cover`](Self::on_cover).
    pub(crate) off_cover: Cover<Symbol, Anonymous>,
    /// The minimised hold-region cover, twin of [`on_cover`](Self::on_cover). Empty for a signal with
    /// no self-hold gap (cross-coupled emergent memory).
    pub(crate) hold_cover: Cover<Symbol, Anonymous>,
    /// The signal is a state variable — it lies on a dependency cycle (a direct self-hold or a larger
    /// coupling cycle, as classified by [`super::resolve::state_variables`]) — and so must emit a
    /// `statetable`, never a combinational `function`. This is independent of whether the signal has a
    /// direct self-hold gap: cross-coupled emergent memory (mutex, SR latch) is hysteretic with an
    /// empty `hold`.
    pub(crate) hysteretic: bool,
}

/// Derive the state-table regions of the signal `name` from its folded BDD `f` (see [`StateRegions`]),
/// taken from the shared per-cell BDD map. `is_state` is the machine's cyclic classification — whether
/// `name` is in the cell's [`super::resolve::state_variables`] set — and decides `hysteretic` (and thus
/// statetable-vs-function emission), independent of the region maths.
pub(crate) fn state_regions<B: Brand, C: ManagerCell>(
    name: &Symbol,
    f: &Bdd<B, C>,
    is_state: bool,
) -> StateRegions {
    // Columns = the function's BDD support minus the pin's own self-feedback, in BDD variable order.
    // Inputs the function does not depend on are simply absent from `variables()`, so they never
    // become columns. `cover_over_fr(&cols)` then universally projects the self var (the only support
    // variable left outside `cols`) away, re-basing `f` onto the partial function over `cols`.
    let cols: Vec<Symbol> = f
        .variables()
        .filter(|v| v.as_str() != name.as_str())
        .collect();

    // Onset and offset as independent single-output F covers. For a two-sided FR cover the F side is
    // the force-high assignments and the R side the force-low ones; the undef gap between them (where
    // the output still depends on the projected self var) is *not* emitted as cubes. Taking the F side
    // of `f` gives the onset and the F side of `!f` the offset — each already a clean F cover we can
    // minimise on its own without collapsing the gap.
    // Minimise each region independently, keeping the minimised covers. This is safe precisely because
    // each is its own onset with no don't-care set: Espresso reproduces that exact region and cannot
    // bleed the hold gap into on/off.
    let on_cover = minimise(f_side(&f.cover_over_fr(&cols)));
    let off_cover = minimise(f_side(&(!f).cover_over_fr(&cols)));

    // The hold set is the undef gap = complement of (onset ∪ offset), reconstructed from the two region
    // covers as its own function so it, too, can be minimised as an independent onset. The shared per-cell
    // builder that minted `f` mints these too.
    let builder = f.builder();
    let on_bdd = builder.build_cover(&on_cover);
    let off_bdd = builder.build_cover(&off_cover);
    let hold_bdd = !(on_bdd.or(&off_bdd));
    let hold_cover = minimise_bdd(&hold_bdd);

    // Derive the `StateCube` views from the SAME minimised covers, so cube output is bit-identical.
    let on = region_cubes(&on_cover, &cols);
    let off = region_cubes(&off_cover, &cols);
    let hold = region_cubes(&hold_cover, &cols);

    let hysteretic = is_state;

    StateRegions {
        cols,
        on,
        off,
        hold,
        on_cover,
        off_cover,
        hold_cover,
        hysteretic,
    }
}

/// The F (ON-set) cubes of an FR cover, re-collected as an independent single-output F cover.
pub(crate) fn f_side(fr: &Cover<Symbol, Anonymous>) -> Cover<Symbol, Anonymous> {
    Cover::from_cubes(
        CoverType::F,
        fr.cubes().filter(|c| c.cube_type() == CubeType::F).cloned(),
    )
}

/// Espresso-minimise a region's own F cover, falling back to the un-minimised cover on error. An empty
/// cover minimises to empty, so region emptiness — and thus the emitters' constant-detection — is
/// preserved.
pub(crate) fn minimise(cover: Cover<Symbol, Anonymous>) -> Cover<Symbol, Anonymous> {
    cover.minimize().unwrap_or(cover)
}

/// Espresso-minimise a region given as a BDD (the hold gap), as an F cover.
pub(crate) fn minimise_bdd<B: Brand, C: ManagerCell>(bdd: &Bdd<B, C>) -> Cover<Symbol, Anonymous> {
    bdd.minimize().unwrap_or_else(|_| bdd.cover())
}

/// Read each cube of a region cover as a [`StateCube`] aligned to `cols` by variable name: a column
/// the cube does not constrain (absent from its support) reads as `None` (don't-care). Reading by name
/// with [`Minterm::value_of`] is order-independent, so no re-homing/projection of the cube is needed.
pub(crate) fn region_cubes(cover: &Cover<Symbol, Anonymous>, cols: &[Symbol]) -> Vec<StateCube> {
    cover
        .cubes()
        .map(|cube| cols.iter().map(|v| cube.inputs().value_of(v)).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::logic::resolve;
    use crate::model::{analyse_one as analyse, AnalysedCell, AnalysedOutput};
    use espresso_logic::bdd_builder;

    /// Derive the regions of a signal straight from its parsed expression, in a throwaway builder. The
    /// in-file test cells are all untouched by minimisation, so `sig.expr` is still the parsed truth and
    /// this reproduces exactly what the shared-map `state_regions` computes. `is_state` is the cell's
    /// cyclic classification (`resolve::state_variables`), matching how `model.rs` calls `state_regions`.
    fn regions_of(cell: &AnalysedCell, sig: &AnalysedOutput) -> StateRegions {
        let b = bdd_builder!();
        let f = b.build(&sig.expr);
        let signals: Vec<&AnalysedOutput> = cell.signals().collect();
        let state_set = resolve::state_variables(&signals);
        state_regions(&sig.name, &f, state_set.contains(&sig.name))
    }

    #[test]
    fn state_regions_c_element_self_holds() {
        let cell = analyse(
            r#"
[[cell]]
name = "C2"
inputs = ["A", "B"]
[cell.outputs]
Q = "A*B + Q*(A+B)"
"#,
        );
        let sr = regions_of(&cell, &cell.outputs[0]);
        // Self-feedback ⇒ hysteretic; the only columns are the primary inputs (Q is the reg).
        assert!(sr.hysteretic);
        assert_eq!(
            sr.cols.iter().map(Symbol::as_str).collect::<Vec<_>>(),
            ["A", "B"]
        );
        assert_eq!(sr.on, vec![vec![Some(true), Some(true)]]); // A*B
        assert_eq!(sr.off, vec![vec![Some(false), Some(false)]]); // !A*!B
        assert_eq!(sr.hold.len(), 2); // A xor B
    }

    #[test]
    fn state_regions_keep_other_output_as_column() {
        // Cross-coupled: Q references the *other* output Qn, so Qn is a column, not projected away.
        let cell = analyse(
            r#"
[[cell]]
name = "SR"
inputs = ["S", "R"]
[cell.outputs]
Q = "S + Q*!R"
Qn = "R + Qn*!S"
"#,
        );
        let q = regions_of(&cell, &cell.outputs[0]);
        // Q = S + Q*!R references only S, R and itself — no other output, so cols are just inputs.
        assert_eq!(
            q.cols.iter().map(Symbol::as_str).collect::<Vec<_>>(),
            ["S", "R"]
        );
        assert!(q.hysteretic);
    }

    #[test]
    fn state_regions_keeps_internal_node_as_column() {
        // A DFF slave Q references the internal master M — M must appear as a state-table column.
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
        let q = regions_of(&cell, &cell.outputs[0]);
        // Q = CLK*M + !CLK*Q depends on CLK and the internal M only — D is not in its support, so it is
        // no longer a column (Q, its self-feedback, is projected out as the reg).
        assert_eq!(
            q.cols.iter().map(Symbol::as_str).collect::<Vec<_>>(),
            ["CLK", "M"]
        );
        assert!(q.hysteretic);
    }

    /// The crux of the minimisation step: for every region of every signal, the BDD reconstructed
    /// from the emitted (minimised) cubes must be *logically equivalent* to the reference region BDD
    /// computed exactly as [`state_regions`] does. This proves minimisation preserved every region's
    /// function even though the cube set changed.
    #[test]
    fn minimised_regions_are_equivalent_to_functions() {
        use espresso_logic::bdd::BddBuilder;

        // Rebuild a region BDD from its emitted cubes: OR of cubes, each cube the AND of its fixed
        // literals. An empty cube list is the constant `false`; a cube with no fixed literal (all
        // don't-care) is the constant `true`.
        fn reconstruct<B: Brand, C: ManagerCell>(
            builder: &BddBuilder<B, C>,
            cols: &[Symbol],
            cubes: &[StateCube],
        ) -> Bdd<B, C> {
            let mut cover = builder.constant(false);
            for cube in cubes {
                let mut product = builder.constant(true);
                for (col, val) in cols.iter().zip(cube.iter()) {
                    match val {
                        Some(true) => product = product.and(&builder.var(col.as_str())),
                        Some(false) => product = product.and(&!builder.var(col.as_str())),
                        None => {}
                    }
                }
                cover = cover.or(&product);
            }
            cover
        }

        let cells = [
            r#"
[[cell]]
name = "C2"
inputs = ["A", "B"]
[cell.outputs]
Q = "A*B + Q*(A+B)"
"#,
            r#"
[[cell]]
name = "ND2"
inputs = ["A", "B"]
[cell.outputs]
Y = "!(A*B)"
"#,
            r#"
[[cell]]
name = "DFF"
inputs = ["CLK", "D"]
[cell.internal]
M = "!CLK*D + CLK*M"
[cell.outputs]
Q = "CLK*M + !CLK*Q"
"#,
            // Cross-coupled mutex: each grant drops the primary input it does not depend on.
            r#"
[[cell]]
name = "MUT"
inputs = ["A", "B"]
[cell.outputs]
Qa = "!Qb * A"
Qb = "!Qa * B"
"#,
            // Multi-input hysteretic cell that keeps all its inputs but re-minimises each region.
            r#"
[[cell]]
name = "RACELEM21"
inputs = ["M1", "M2", "P1", "P2", "C", "R"]
[cell.outputs]
Q = "(P1*P2*C+Q*(M1+M2+C))*!R"
"#,
        ];

        for src in cells {
            let cell = analyse(src);
            for sig in cell.signals() {
                let sr = regions_of(&cell, sig);

                // Reference region BDDs, built exactly as `state_regions` does. One builder for both
                // the references and the reconstruction so `equivalent_to` shares a manager.
                let builder = bdd_builder!();
                let f = builder.build(&sig.expr);
                let self_state: Vec<&str> = if sig.feedback.contains(&sig.name) {
                    vec![sig.name.as_str()]
                } else {
                    vec![]
                };
                let on_bdd = f.forall(&self_state);
                let off_bdd = (!f.clone()).forall(&self_state);
                let hold_bdd = !on_bdd.or(&off_bdd);

                assert!(
                    reconstruct(&builder, &sr.cols, &sr.on).equivalent_to(&on_bdd),
                    "on region mismatch for {}.{}",
                    cell.repr_name(),
                    sig.name
                );
                assert!(
                    reconstruct(&builder, &sr.cols, &sr.off).equivalent_to(&off_bdd),
                    "off region mismatch for {}.{}",
                    cell.repr_name(),
                    sig.name
                );
                assert!(
                    reconstruct(&builder, &sr.cols, &sr.hold).equivalent_to(&hold_bdd),
                    "hold region mismatch for {}.{}",
                    cell.repr_name(),
                    sig.name
                );
            }
        }
    }

    #[test]
    fn state_regions_combinational_has_no_hold() {
        let cell = analyse(
            r#"
[[cell]]
name = "ND2"
inputs = ["A", "B"]
[cell.outputs]
Y = "!(A*B)"
"#,
        );
        let sr = regions_of(&cell, &cell.outputs[0]);
        assert!(!sr.hysteretic);
        assert!(sr.hold.is_empty());
        assert!(!sr.on.is_empty());
    }

    #[test]
    fn state_regions_mutex_is_hysteretic_with_empty_hold() {
        // Cross-coupled mutex: each grant holds via the *coupling* cycle (Qa↔Qb), not a direct
        // self-hold. Both are state variables, so both classify hysteretic — yet each region maths
        // leaves an empty hold set (the function is total once the other grant is a column).
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
        let qa = regions_of(&cell, &cell.outputs[0]);
        let qb = regions_of(&cell, &cell.outputs[1]);
        assert!(qa.hysteretic);
        assert!(qb.hysteretic);
        assert!(qa.hold.is_empty());
        assert!(qb.hold.is_empty());
    }

    #[test]
    fn state_regions_sr_nor_latch_is_hysteretic() {
        // SR NOR latch: cross-coupled, each output on the coupling cycle ⇒ both hysteretic.
        let cell = analyse(
            r#"
[[cell]]
name = "SR"
inputs = ["S", "R"]
[cell.outputs]
Q = "!(R + Qn)"
Qn = "!(S + Q)"
"#,
        );
        let q = regions_of(&cell, &cell.outputs[0]);
        let qn = regions_of(&cell, &cell.outputs[1]);
        assert!(q.hysteretic);
        assert!(qn.hysteretic);
    }

    #[test]
    fn state_regions_plain_combinational_stays_non_hysteretic() {
        // A pure combinational gate is on no dependency cycle ⇒ never hysteretic.
        let cell = analyse(
            r#"
[[cell]]
name = "ND2"
inputs = ["A", "B"]
[cell.outputs]
Y = "!(A*B)"
"#,
        );
        let sr = regions_of(&cell, &cell.outputs[0]);
        assert!(!sr.hysteretic);
    }

    #[test]
    fn state_regions_self_holding_keeper_stays_hysteretic() {
        // A C-element keeper self-holds directly (Q on its own cycle) ⇒ hysteretic, as before.
        let cell = analyse(
            r#"
[[cell]]
name = "C2"
inputs = ["A", "B"]
[cell.outputs]
Q = "A*B + Q*(A+B)"
"#,
        );
        let sr = regions_of(&cell, &cell.outputs[0]);
        assert!(sr.hysteretic);
    }
}
