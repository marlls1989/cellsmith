//! Transition-arc derivation. For each output, the single-input-edge transitions are found between
//! the on/off/hold regions, and each is paired with its prevector walk.
//!
//! Mirrors hsNCL `genTransitionArcs'`:
//!   * rise edges: distance-1 transitions from `(hold ∪ off)` to `on`, prevector walks `off`→`hold`
//!     →`start`;
//!   * fall edges: distance-1 transitions from `(hold ∪ on)` to `off`, prevector walks `on`→`hold`
//!     →`start`.

use std::collections::BTreeSet;

use espresso_logic::{Minterm, Symbol};

use crate::logic::regions::{regions, MintermSet};
use crate::logic::walk::{single_var_transitions, transitions_path, WalkError};
use crate::model::{AnalysedCell, AnalysedOutput};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Edge {
    Rise,
    Fall,
}

/// One characterization arc: an input edge on `related` driving `output` in direction `edge`. The
/// related pin is always a primary input — for a cross-coupled cell the coupling is collapsed away
/// (see [`regions`]) before arcs are derived, so no output is ever a related pin.
#[derive(Debug, Clone)]
pub struct Arc {
    pub edge: Edge,
    pub output: String,
    pub related: String,
    /// Start state of the measured edge (the prevector's target).
    pub start: Minterm<Symbol>,
    /// End state of the measured edge (defines the vector and the `-when` condition).
    pub end: Minterm<Symbol>,
    /// The prevector: a single-step walk into `start`.
    pub prevector: Vec<Minterm<Symbol>>,
    pub is_async: bool,
}

/// Derive all transition arcs for one output.
pub fn transition_arcs(
    cell: &AnalysedCell,
    output: &AnalysedOutput,
) -> Result<Vec<Arc>, WalkError> {
    let r = regions(output, &cell.outputs, &cell.inputs);
    let async_set: BTreeSet<&str> = cell.async_pins.iter().map(String::as_str).collect();
    let mut arcs = Vec::new();

    let rise_from: MintermSet = r.hold.union(&r.off).cloned().collect();
    for t in single_var_transitions(&rise_from, &r.on) {
        let target: MintermSet = std::iter::once(t.src.clone()).collect();
        let prevector = transitions_path(&r.off, &r.hold, &target)?;
        let is_async = async_set.contains(t.var.as_str());
        arcs.push(Arc {
            edge: Edge::Rise,
            output: output.name.clone(),
            related: t.var,
            start: t.src,
            end: t.dst,
            prevector,
            is_async,
        });
    }

    let fall_from: MintermSet = r.hold.union(&r.on).cloned().collect();
    for t in single_var_transitions(&fall_from, &r.off) {
        let target: MintermSet = std::iter::once(t.src.clone()).collect();
        let prevector = transitions_path(&r.on, &r.hold, &target)?;
        let is_async = async_set.contains(t.var.as_str());
        arcs.push(Arc {
            edge: Edge::Fall,
            output: output.name.clone(),
            related: t.var,
            start: t.src,
            end: t.dst,
            prevector,
            is_async,
        });
    }

    Ok(arcs)
}

/// Derive transition arcs for every output of a cell.
pub fn cell_arcs(cell: &AnalysedCell) -> Result<Vec<Arc>, WalkError> {
    let mut all = Vec::new();
    for output in &cell.outputs {
        all.extend(transition_arcs(cell, output)?);
    }
    Ok(all)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::parse_spec;

    fn analyse(src: &str) -> AnalysedCell {
        parse_spec(src).unwrap().cells.remove(0).analyse().unwrap()
    }

    #[test]
    fn c_element_has_rise_and_fall_per_input() {
        let cell = analyse(
            r#"
[[cell]]
name = "C2"
inputs = ["A", "B"]
[cell.outputs]
Q = "A*B + Q*(A+B)"
"#,
        );
        let arcs = cell_arcs(&cell).unwrap();
        // A rise on A (from hold 01) and on B (from hold 10); likewise two falls. Plus any from the
        // off/on flat states adjacent to a hold state.
        assert!(arcs
            .iter()
            .any(|a| a.edge == Edge::Rise && a.related == "A"));
        assert!(arcs
            .iter()
            .any(|a| a.edge == Edge::Rise && a.related == "B"));
        assert!(arcs
            .iter()
            .any(|a| a.edge == Edge::Fall && a.related == "A"));
        assert!(arcs
            .iter()
            .any(|a| a.edge == Edge::Fall && a.related == "B"));
        // Every arc's prevector is a real single-step walk into its start state.
        for a in &arcs {
            assert_eq!(a.prevector.last().unwrap(), &a.start);
            for w in a.prevector.windows(2) {
                assert_eq!(w[0].hamming_distance(&w[1]), 1);
            }
        }
    }

    #[test]
    fn cross_coupled_mutex_related_pins_are_inputs_only() {
        // After collapse, related pins are ALWAYS primary inputs — never the other output. A `Qb→Qa`
        // arc is a physical deadlock and must not exist. Both A and B drive each grant (B releasing
        // lets A take the grant, and vice versa — the cascade).
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
        let arcs = cell_arcs(&cell).unwrap();
        assert!(!arcs.is_empty());
        // No output is ever a related pin.
        assert!(
            arcs.iter().all(|a| a.related == "A" || a.related == "B"),
            "related pins must be primary inputs, got {:?}",
            arcs.iter().map(|a| a.related.as_str()).collect::<Vec<_>>()
        );
        assert!(arcs.iter().all(|a| a.related != "Qa" && a.related != "Qb"));
        // Both inputs drive Qa (A directly, B via the cascade) and symmetrically both drive Qb.
        assert!(arcs.iter().any(|a| a.output == "Qa" && a.related == "A"));
        assert!(arcs.iter().any(|a| a.output == "Qa" && a.related == "B"));
        assert!(arcs.iter().any(|a| a.output == "Qb" && a.related == "B"));
        assert!(arcs.iter().any(|a| a.output == "Qb" && a.related == "A"));
    }

    #[test]
    fn reset_cascade_propagates_to_both_grants() {
        // Qb = Sb + !Qa*B: Sb forces Qb high, which forces Qa low. The Sb arc must propagate to BOTH
        // outputs — directly to Qb (rise) and, cascaded via Qb, to Qa (fall).
        let cell = analyse(
            r#"
[[cell]]
name = "MUTS"
inputs = ["A", "B", "Sb"]
async = ["Sb"]
[cell.outputs]
Qa = "!Qb * A"
Qb = "Sb + !Qa * B"
"#,
        );
        let arcs = cell_arcs(&cell).unwrap();
        // Related pins are still inputs only.
        assert!(arcs
            .iter()
            .all(|a| ["A", "B", "Sb"].contains(&a.related.as_str())));
        // Sb rises Qb.
        assert!(arcs
            .iter()
            .any(|a| a.output == "Qb" && a.related == "Sb" && a.edge == Edge::Rise));
        // Sb cascades to Qa (falls) — the required propagation via Qb.
        assert!(arcs
            .iter()
            .any(|a| a.output == "Qa" && a.related == "Sb" && a.edge == Edge::Fall));
    }

    #[test]
    fn combinational_arcs_have_trivial_prevectors() {
        // 2-input NAND: no hold, every state is on/off; arcs still derived.
        let cell = analyse(
            r#"
[[cell]]
name = "ND2"
inputs = ["A", "B"]
[cell.outputs]
Y = "!(A*B)"
"#,
        );
        let arcs = cell_arcs(&cell).unwrap();
        assert!(!arcs.is_empty());
        assert!(arcs.iter().all(|a| !a.is_async));
    }

    #[test]
    fn async_reset_pin_marked() {
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
        let arcs = cell_arcs(&cell).unwrap();
        assert!(arcs.iter().any(|a| a.related == "R" && a.is_async));
        assert!(arcs
            .iter()
            .filter(|a| a.related != "R")
            .all(|a| !a.is_async));
    }
}
