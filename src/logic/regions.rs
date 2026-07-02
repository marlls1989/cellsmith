//! State-table / sequential-UDP region derivation.
//!
//! This is the **functional** view of an output, used by the Verilog UDP and the Liberty
//! `statetable` — distinct from the timing-arc view, which is produced by the state machine in
//! [`super::arcs`] and needs no region collapse.
//!
//! A state table keeps each **other** signal the function references (another output *or* an internal
//! state node) as an ordinary input column, and projects out only the pin's **own** self-feedback —
//! which becomes the sequential element's current-state (`reg`) column. This mirrors hsNCL
//! `outPinUDP`, whose column set is `(inPins ∪ otherStateSignals) \ {self}`:
//!
//! For a hysteretic pin the `on`/`off` sets come from a single `Bdd::cover_over_fr` extraction that
//! re-bases `f` onto the column set by **universal** projection of the pin's own self-feedback: the
//! two-sided FR cover's `F` cubes are forced-high (`∀self. f`) and its `R` cubes forced-low
//! (`∀self. ¬f`), both already self-projected. `hold` is the undef gap those two leave behind:
//!
//! - `on   = ∀self. f`   — FR `F` cubes; forced high regardless of held self-state,
//! - `off  = ∀self. ¬f`  — FR `R` cubes; forced low,
//! - `hold = ¬(on ∨ off)` — state-dependent (hysteretic); a `-`/`N` no-change entry.

use std::sync::Arc;

use espresso_logic::bdd::{Bdd, Brand, ManagerCell};
use espresso_logic::{bdd_builder, Anonymous, Cube, CubeType, Symbol, Symbols};

use crate::logic::machine;
use crate::model::AnalysedOutput;

/// Split a signal's feedback into the *other* state signals it references (kept as columns) and its
/// *own* self-feedback (the hysteretic state to project out). The "others" include internal state
/// variables as well as other outputs — both are legitimate state-table columns.
fn feedback_split(output: &AnalysedOutput) -> (Vec<String>, Vec<&str>) {
    let others: Vec<String> = output
        .feedback
        .iter()
        .filter(|f| **f != output.name)
        .cloned()
        .collect();
    let self_state: Vec<&str> = if output.feedback.iter().any(|x| x == &output.name) {
        vec![output.name.as_str()]
    } else {
        vec![]
    };
    (others, self_state)
}

/// One cube over the state-table/UDP column set: `Some(true)`/`Some(false)` for a fixed column,
/// `None` for a don't-care. Aligned position-by-position to [`StateRegions::cols`].
pub type StateCube = Vec<Option<bool>>;

/// The regions of a signal as they appear in a **state table / sequential UDP**.
///
/// Each region is a set of don't-care cubes (from the BDD's prime paths), so a variable a cube does
/// not constrain prints as `?`/`-`.
#[derive(Debug, Clone)]
pub struct StateRegions {
    /// Input columns: the cell's primary inputs, followed by any *other* state signal the function
    /// references (another output or an internal node), in the cell's signal order. The pin's own
    /// feedback is *not* a column.
    pub cols: Vec<String>,
    pub on: Vec<StateCube>,
    pub off: Vec<StateCube>,
    pub hold: Vec<StateCube>,
    /// The pin holds on its own state (self-referential ⇒ hysteretic ⇒ `hold` non-empty).
    pub hysteretic: bool,
}

/// Derive the state-table regions of `output` over `inputs` (see [`StateRegions`]).
pub fn state_regions(output: &AnalysedOutput, inputs: &[String]) -> StateRegions {
    // Columns: primary inputs, then the other state signals this function references (self excluded).
    let (others, self_state) = feedback_split(output);
    let cols: Vec<String> = inputs.iter().cloned().chain(others).collect();

    let builder = bdd_builder!();
    let f = builder.build(&output.expr);
    let not_f = !f.clone();
    let cols_header = machine::header(&cols);

    // Cross-region disjointness holds by construction: for a hysteretic pin the `on`/`off` sets are the
    // `F`/`R` sides of one `cover_over_fr` extraction (mutually exclusive by definition), and `hold` is
    // their complement. Only `hold` is Espresso-minimised — the FR cubes are already in prime form and
    // must *not* be minimised, or the hold gap would be absorbed as don't-care into the on-set.
    let (on, off, hold) = if self_state.is_empty() {
        // Combinational: no self-feedback to project, so `on`/`off` are just `f`/`¬f` minimised.
        let on = minimised(&f, &cols_header);
        let off = minimised(&not_f, &cols_header);
        (on, off, Vec::new())
    } else {
        // Hysteretic: one FR extraction re-bases `f` onto `cols` by universal projection of `self_state`
        // (the only support outside `cols`), so `F ≡ ∀self. f` and `R ≡ ∀self. ¬f`.
        let fr = f.cover_over_fr(&cols);
        let on = realign_cubes(
            fr.cubes().filter(|c| c.cube_type() == CubeType::F),
            &cols_header,
        );
        let off = realign_cubes(
            fr.cubes().filter(|c| c.cube_type() == CubeType::R),
            &cols_header,
        );
        // The undef/hold gap is what `F ∪ R` leaves uncovered; compute it on the BDD and minimise that.
        let hold_bdd = !(f.forall(&self_state).or(&not_f.forall(&self_state)));
        let hold = minimised(&hold_bdd, &cols_header);
        (on, off, hold)
    };
    let hysteretic = !hold.is_empty();

    StateRegions {
        cols,
        on,
        off,
        hold,
        hysteretic,
    }
}

/// Minimise a region's function with Espresso and realign the resulting cover onto `cols`. Falls back
/// to the (non-minimised) prime-path cubes if the minimiser errors. Because `minimize` of a false
/// function yields an empty cover, region emptiness — and thus the hysteretic flag and the emitters'
/// constant-detection — is preserved.
fn minimised<B: Brand, C: ManagerCell>(
    bdd: &Bdd<B, C>,
    cols: &Arc<Symbols<Symbol>>,
) -> Vec<StateCube> {
    let cover = bdd.minimize().unwrap_or_else(|_| bdd.cover());
    realign_cubes(cover.cubes(), cols)
}

/// Realign a cover's cubes onto the `cols` header: for each cube, one `Option<bool>` per column
/// (a column the cube does not constrain — a don't-care or a variable outside its support — is `None`).
/// [`Minterm::project_onto`] re-expresses each cube's minterm over `cols`, so the values are already in
/// column order with the absent columns filled as don't-care.
fn realign_cubes<'a>(
    cubes: impl IntoIterator<Item = &'a Cube<Symbol, Anonymous>>,
    cols: &Arc<Symbols<Symbol>>,
) -> Vec<StateCube> {
    cubes
        .into_iter()
        .map(|cube| cube.inputs().project_onto(cols).iter().collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::analyse_one as analyse;

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
        let sr = state_regions(&cell.outputs[0], &cell.inputs);
        // Self-feedback ⇒ hysteretic; the only columns are the primary inputs (Q is the reg).
        assert!(sr.hysteretic);
        assert_eq!(sr.cols, ["A", "B"]);
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
        let q = state_regions(&cell.outputs[0], &cell.inputs);
        // Q = S + Q*!R references only S, R and itself — no other output, so cols are just inputs.
        assert_eq!(q.cols, ["S", "R"]);
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
        let q = state_regions(&cell.outputs[0], &cell.inputs);
        assert_eq!(q.cols, ["CLK", "D", "M"]); // internal M kept as a column, Q (self) projected
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
            cols: &[String],
            cubes: &[StateCube],
        ) -> Bdd<B, C> {
            let mut cover = builder.constant(false);
            for cube in cubes {
                let mut product = builder.constant(true);
                for (col, val) in cols.iter().zip(cube.iter()) {
                    match val {
                        Some(true) => product = product.and(&builder.var(col)),
                        Some(false) => product = product.and(&!builder.var(col)),
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
        ];

        for src in cells {
            let cell = analyse(src);
            for sig in cell.signals() {
                let sr = state_regions(sig, &cell.inputs);

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
                    cell.name,
                    sig.name
                );
                assert!(
                    reconstruct(&builder, &sr.cols, &sr.off).equivalent_to(&off_bdd),
                    "off region mismatch for {}.{}",
                    cell.name,
                    sig.name
                );
                assert!(
                    reconstruct(&builder, &sr.cols, &sr.hold).equivalent_to(&hold_bdd),
                    "hold region mismatch for {}.{}",
                    cell.name,
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
        let sr = state_regions(&cell.outputs[0], &cell.inputs);
        assert!(!sr.hysteretic);
        assert!(sr.hold.is_empty());
        assert!(!sr.on.is_empty());
    }
}
