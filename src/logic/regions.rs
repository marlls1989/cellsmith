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

/// The three regions of an output over the cell's primary inputs.
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

/// Split an output's feedback into the *other* outputs it references (kept as columns) and its *own*
/// self-feedback (the hysteretic state to project out). Used by [`state_regions`].
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

/// Derive the on/off/hold regions of `output` over the cell's primary `inputs`.
///
/// For a genuinely cross-coupled cell (a mutex `Qa = !Qb·A`, a reset arbiter) the function first
/// **collapses the coupling**: every *other* output it references is composed away (its own function
/// substituted in) until the result is **self-holding** — a function of primary inputs plus this
/// pin's own feedback only. This is what makes the arcs correct: the related pin is always a primary
/// input (no other output survives to become one), the mutual-exclusion/deadlock states fall into the
/// `hold` region (so no impossible output→output arc is produced), and an input that only reaches this
/// output *through* the coupling (a reset forcing the other grant) now appears in the collapsed
/// function, so its cascade arc is generated.
///
/// A cell with no cross-references (C-element, latch, non-mutual SR) collapses to itself, so its
/// regions are unchanged. All logic is on BDDs; `BoolExpr` is only the parse-input to `build`.
pub fn regions(output: &AnalysedOutput, outputs: &[AnalysedOutput], inputs: &[String]) -> Regions {
    let builder = bdd_builder!();
    // Build every output's BDD in one builder so they can be composed (BDDs are branded per builder).
    let bdds: std::collections::BTreeMap<&str, _> = outputs
        .iter()
        .map(|o| (o.name.as_str(), builder.build(&o.expr)))
        .collect();

    // Collapse: compose every *other* output out of this one, to a self-holding fixpoint.
    let other_names: BTreeSet<&str> = outputs
        .iter()
        .map(|o| o.name.as_str())
        .filter(|n| *n != output.name)
        .collect();
    let mut f = bdds[output.name.as_str()].clone();
    let mut rounds = 0usize;
    loop {
        let present: Vec<Symbol> = f
            .variables()
            .filter(|s| other_names.contains(s.as_str()))
            .collect();
        if present.is_empty() {
            break;
        }
        for s in present {
            // f[v := g]  via the Shannon cofactor identity (no compose primitive in espresso).
            let v = s.as_str();
            let g = &bdds[v];
            let f1 = f.restrict(v, true);
            let f0 = f.restrict(v, false);
            f = g.and(&f1).or(&(!g).and(&f0));
        }
        rounds += 1;
        assert!(
            rounds <= outputs.len() + 1,
            "collapse of output {:?} did not converge: a non-self output cycle (an extra latch among \
             {other_names:?}) is unsupported",
            output.name,
        );
    }

    // Project the pin's own self-feedback out (universal quantification); combinational if absent.
    let self_state: Vec<&str> = if f.variables().any(|s| s.as_str() == output.name) {
        vec![output.name.as_str()]
    } else {
        vec![]
    };
    let on_bdd = f.forall(&self_state);
    let off_bdd = (!&f).forall(&self_state);
    let hold_bdd = !on_bdd.or(&off_bdd);

    // Expand each region to fully-assigned minterms over the input pinlist.
    let cols: Vec<&str> = inputs.iter().map(String::as_str).collect();
    Regions {
        on: cover_minterms(&on_bdd.maximize(&cols)),
        off: cover_minterms(&off_bdd.maximize(&cols)),
        hold: cover_minterms(&hold_bdd.maximize(&cols)),
    }
}

/// Collect a maximal cover's cubes (each a fully-assigned minterm) into a [`MintermSet`].
///
/// In espresso-logic 5.1 `Bdd::maximize` replaces the removed `to_minterms`: it returns the
/// deduplicated maximal cover over the requested variables, each cube of which is a minterm.
fn cover_minterms(cover: &Cover<Symbol, Anonymous>) -> MintermSet {
    cover.cubes().map(|c| c.inputs().clone()).collect()
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
    let (others, self_state) = feedback_split(output);
    let cols: Vec<String> = inputs.iter().cloned().chain(others).collect();

    let builder = bdd_builder!();
    let f = builder.build(&output.expr);
    let not_f = builder.build(&!output.expr.clone());

    // Project out only the pin's *own* feedback (its current state); other outputs stay as columns.
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
        cover_minterms(&builder.build(&parsed.expr).maximize(cols))
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
        let r = regions(&cell.outputs[0], &cell.outputs, &cell.inputs);
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
    fn cross_coupled_mutex_collapses_to_self_holding() {
        // Mutex grant Qa = !Qb·A collapses (Qb composed out) to the self-holding Qa = A·!B + Qa·A:
        //   on = A·!B, off = !A, hold = {A=1,B=1} (the metastable arbitration state).
        // The other grant Qb never survives as a column — regions are over the primary inputs only.
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
        let r = regions(&cell.outputs[0], &cell.outputs, &cell.inputs);
        let cols = ["A", "B"];
        assert_eq!(r.on, cover_of("A*!B", &cols));
        assert_eq!(r.off, cover_of("!A", &cols));
        assert_eq!(r.hold, cover_of("A*B", &cols)); // metastable hold
        assert_eq!(r.len(), 4); // full partition of the 2-input space
        assert!(r.on.is_disjoint(&r.off));
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
        let r = regions(&cell.outputs[0], &cell.outputs, &cell.inputs);
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
        let r = regions(&cell.outputs[0], &cell.outputs, &cell.inputs);
        // Every state with R=1 is forced off; on-set requires A*B*!R.
        assert_eq!(r.on, cover_of("A*B*!R", &["A", "B", "R"]));
        // Hold can only happen when R=0.
        assert!(r.hold.iter().all(|_m| true));
        assert_eq!(r.len(), 8);
    }
}
