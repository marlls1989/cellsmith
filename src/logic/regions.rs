//! Region derivation: split an output's **input space** into `on` / `off` / `hold` by projecting
//! out the feedback/state variables.
//!
//! A state-holding output references its own (or another) output as a delayed/feedback variable.
//! Holding those variables at all values (universal quantification) classifies each input
//! assignment:
//!
//! - `on   = ∀state. f`   — forced high regardless of held state,
//! - `off  = ∀state. ¬f`  — forced low regardless of held state,
//! - `hold = ¬(on ∨ off)` — state-dependent, the **hysteretic** region.
//!
//! `hold` is real behaviour, not a don't-care: it is the NULL/transition region the prevector walk
//! routes through, and must never be handed to Espresso as a `CubeType::D`.
//!
//! A purely combinational output (no feedback variables) degenerates to `on = f`, `off = ¬f`,
//! `hold = ∅` through the same code path.

use std::collections::BTreeSet;

use espresso_logic::{bdd_builder, Anonymous, Cover, Minterm, Symbol};

use crate::model::AnalysedOutput;

/// A set of fully-assigned input minterms (every cell input pin fixed).
pub type MintermSet = BTreeSet<Minterm<Symbol>>;

/// The three regions of an output over the cell's input pins.
#[derive(Debug, Clone)]
pub struct Regions {
    pub on: MintermSet,
    pub off: MintermSet,
    pub hold: MintermSet,
}

impl Regions {
    /// Total number of input minterms across the three regions (should equal `2^inputs`).
    pub fn len(&self) -> usize {
        self.on.len() + self.off.len() + self.hold.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Derive the on/off/hold regions of `output` over `inputs` (the cell's pinlist order).
pub fn regions(output: &AnalysedOutput, inputs: &[String]) -> Regions {
    let builder = bdd_builder!();
    let f = builder.build(&output.expr);
    let not_f = builder.build(&!output.expr.clone());

    // Project the feedback/state variables out: hold them at all values.
    let state: Vec<&str> = output.feedback.iter().map(String::as_str).collect();
    let on_bdd = f.forall(&state);
    let off_bdd = not_f.forall(&state);
    let hold_bdd = !on_bdd.or(&off_bdd);

    // Expand each region to fully-assigned minterms over the input pinlist.
    let cols: Vec<&str> = inputs.iter().map(String::as_str).collect();
    Regions {
        on: on_bdd.to_minterms(&cols).into_iter().collect(),
        off: off_bdd.to_minterms(&cols).into_iter().collect(),
        hold: hold_bdd.to_minterms(&cols).into_iter().collect(),
    }
}

/// One cube over the state-table/UDP column set: `Some(true)`/`Some(false)` for a fixed column,
/// `None` for a don't-care. Aligned position-by-position to [`StateRegions::cols`].
pub type StateCube = Vec<Option<bool>>;

/// The regions of an output as they appear in a **state table / sequential UDP**, rather than in the
/// arc space.
///
/// The distinction from [`regions`] is the treatment of feedback. An arc is derived over the primary
/// inputs with *every* feedback variable projected out; a state table instead keeps each **other**
/// output the function references as an ordinary input column, and projects out only the pin's **own**
/// feedback — which becomes the sequential element's current-state (`reg`) column. This mirrors hsNCL
/// `outPinUDP`, whose column set is `(inPins ∪ outPins) \ {self}` (`Circuit/NCLCell.hs`).
///
/// Each region is a set of don't-care cubes (from the BDD's prime paths), so a variable a cube does
/// not constrain prints as `?`/`-`.
#[derive(Debug, Clone)]
pub struct StateRegions {
    /// Input columns: the cell's primary inputs, followed by any *other* outputs the function
    /// references (in the cell's output order). The pin's own feedback is *not* a column.
    pub cols: Vec<String>,
    pub on: Vec<StateCube>,
    pub off: Vec<StateCube>,
    pub hold: Vec<StateCube>,
    /// The pin holds on its own state (self-referential ⇒ hysteretic ⇒ `hold` non-empty).
    pub hysteretic: bool,
}

/// Derive the state-table regions of `output` over `inputs` (see [`StateRegions`]).
pub fn state_regions(output: &AnalysedOutput, inputs: &[String]) -> StateRegions {
    // Columns: primary inputs, then the other outputs this function references (self excluded).
    let others: Vec<String> = output
        .feedback
        .iter()
        .filter(|f| **f != output.name)
        .cloned()
        .collect();
    let cols: Vec<String> = inputs.iter().cloned().chain(others).collect();

    let builder = bdd_builder!();
    let f = builder.build(&output.expr);
    let not_f = builder.build(&!output.expr.clone());

    // Project out only the pin's *own* feedback (its current state); other outputs stay as columns.
    let self_state: Vec<&str> = if output.feedback.iter().any(|x| x == &output.name) {
        vec![output.name.as_str()]
    } else {
        vec![]
    };
    let on_bdd = f.forall(&self_state);
    let off_bdd = not_f.forall(&self_state);
    let hold_bdd = !on_bdd.or(&off_bdd);

    // Extract each region's prime-path cubes and realign them onto the `cols` header.
    let on = realign(&on_bdd.to_cubes(), &cols);
    let off = realign(&off_bdd.to_cubes(), &cols);
    let hold = realign(&hold_bdd.to_cubes(), &cols);
    let hysteretic = !hold.is_empty();

    StateRegions {
        cols,
        on,
        off,
        hold,
        hysteretic,
    }
}

/// Realign a cover's cubes onto the `cols` header: for each cube, one `Option<bool>` per column
/// (a column the cube does not constrain — a don't-care or a variable outside its support — is `None`).
fn realign(cover: &Cover<Symbol, Anonymous>, cols: &[String]) -> Vec<StateCube> {
    cover
        .cubes()
        .map(|cube| {
            let m = cube.inputs();
            let assign: std::collections::BTreeMap<&str, Option<bool>> = m
                .vars()
                .iter()
                .zip(m.iter())
                .map(|(v, val)| (v.as_str(), val))
                .collect();
            cols.iter()
                .map(|c| assign.get(c.as_str()).copied().flatten())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr;
    use crate::model::{parse_spec, AnalysedCell};
    use espresso_logic::bdd_builder;

    fn analyse(src: &str) -> AnalysedCell {
        parse_spec(src).unwrap().cells.remove(0).analyse().unwrap()
    }

    /// The cover of a standalone function over the given columns — the expected region.
    fn cover_of(func: &str, cols: &[&str]) -> MintermSet {
        let builder = bdd_builder!();
        let parsed = expr::parse(func).unwrap();
        builder
            .build(&parsed.expr)
            .to_minterms(cols)
            .into_iter()
            .collect()
    }

    #[test]
    fn c_element_hold_is_two_states() {
        let cell = analyse(
            r#"
[[cell]]
name = "C2"
inputs = ["A", "B"]
[cell.outputs]
Q = "A*B + Q*(A+B)"
"#,
        );
        let r = regions(&cell.outputs[0], &cell.inputs);
        let cols = ["A", "B"];

        assert_eq!(r.on, cover_of("A*B", &cols));
        assert_eq!(r.off, cover_of("!A*!B", &cols));
        assert_eq!(r.on.len(), 1);
        assert_eq!(r.off.len(), 1);
        assert_eq!(r.hold.len(), 2); // A!=B => hold
        assert_eq!(r.len(), 4); // full partition of the 2-input space

        // The three regions are disjoint.
        assert!(r.on.is_disjoint(&r.off));
        assert!(r.on.is_disjoint(&r.hold));
        assert!(r.off.is_disjoint(&r.hold));
    }

    #[test]
    fn inverter_has_no_hold() {
        let cell = analyse(
            r#"
[[cell]]
name = "INV"
inputs = ["A"]
[cell.outputs]
Y = "!A"
"#,
        );
        let r = regions(&cell.outputs[0], &cell.inputs);
        assert!(r.hold.is_empty());
        assert_eq!(r.on, cover_of("!A", &["A"]));
        assert_eq!(r.off, cover_of("A", &["A"]));
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

    #[test]
    fn reset_c_element_reset_forces_off() {
        // Reset-dominant C-element: R forces the output low regardless of held state.
        let cell = analyse(
            r#"
[[cell]]
name = "RC2"
inputs = ["A", "B", "R"]
async = ["R"]
[cell.outputs]
Q = "(A*B + Q*(A+B))*!R"
"#,
        );
        let r = regions(&cell.outputs[0], &cell.inputs);
        // Every state with R=1 is forced off; on-set requires A*B*!R.
        assert_eq!(r.on, cover_of("A*B*!R", &["A", "B", "R"]));
        // Hold can only happen when R=0.
        assert!(r.hold.iter().all(|_m| true));
        assert_eq!(r.len(), 8);
    }
}
