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
//! - `on   = ∀self. f`   — forced high regardless of held self-state,
//! - `off  = ∀self. ¬f`  — forced low,
//! - `hold = ¬(on ∨ off)` — state-dependent (hysteretic); a `-`/`N` no-change entry.

use espresso_logic::{bdd_builder, Anonymous, Cover, Symbol};

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
    let not_f = builder.build(&!output.expr.clone());

    // Project out only the pin's *own* feedback (its current state); other signals stay as columns.
    let on_bdd = f.forall(&self_state);
    let off_bdd = not_f.forall(&self_state);
    let hold_bdd = !on_bdd.or(&off_bdd);

    // Extract each region's prime-path cubes and realign them onto the `cols` header.
    let cols_header = machine::header(&cols);
    let on = realign(&on_bdd.to_cubes(), &cols_header);
    let off = realign(&off_bdd.to_cubes(), &cols_header);
    let hold = realign(&hold_bdd.to_cubes(), &cols_header);
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
/// [`Minterm::project_onto`] re-expresses each cube's minterm over `cols`, so the values are already in
/// column order with the absent columns filled as don't-care.
fn realign(
    cover: &Cover<Symbol, Anonymous>,
    cols: &std::sync::Arc<espresso_logic::Symbols<Symbol>>,
) -> Vec<StateCube> {
    cover
        .cubes()
        .map(|cube| cube.inputs().project_onto(cols).iter().collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{parse_spec, AnalysedCell};

    fn analyse(src: &str) -> AnalysedCell {
        parse_spec(src).unwrap().cells.remove(0).analyse().unwrap()
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
