//! Transition-arc derivation over the cell's **asynchronous state machine**.
//!
//! A cell is a state machine over `inputs × state-variables` (each output's own feedback and every
//! internal state node; see [`resolve`]). A node is a fully-fixed [`Minterm<Symbol>`] over
//! `inputs…, state_vars…` ([`machine`]). Arcs are derived by exploring it:
//!
//!   1. Build each state variable's next-state δ ([`resolve::delta`]); [`machine::settle`] applies them
//!      via [`Bdd::evaluate`] until the state stops changing.
//!   2. BFS from the reachable stable states — which are not assumed but discovered by [`machine::explore`]
//!      from the on/off covers of the signal characteristic functions (never an assumed all-zero state) —
//!      stepping one input at a time and letting the state settle. Metastable transitions (the state
//!      oscillates instead of settling — a mutex's deadlock) yield no fixpoint and are dropped, so no
//!      impossible arc is produced.
//!   3. Wherever a single input toggle flips an **output**, emit an arc: the toggled input is the
//!      `related` pin (arcs are only ever sourced by primary inputs — never an output or internal),
//!      and the prevector is the BFS path — each node projected onto the inputs — that drives every
//!      state variable (internal ones included) into the measured edge's start state.

use std::collections::{BTreeMap, BTreeSet};

use espresso_logic::bdd::{Brand, ManagerCell};
use espresso_logic::{Minterm, Symbol};

use crate::logic::analysis::Machine;
use crate::logic::machine;
use crate::model::AnalysedOutput;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Edge {
    Rise,
    Fall,
}

impl Edge {
    /// The `R`/`F` symbol for this edge (Liberate vector notation).
    pub fn rf(self) -> char {
        match self {
            Edge::Rise => 'R',
            Edge::Fall => 'F',
        }
    }
    /// The `↑`/`↓` arrow for this edge (human-readable condition notation).
    pub fn arrow(self) -> char {
        match self {
            Edge::Rise => '↑',
            Edge::Fall => '↓',
        }
    }
}

/// One characterization arc: an input edge on `related` driving `output` in direction `edge`. The
/// related pin is **always a primary input** — outputs and internal state variables are never arc
/// sources; they are established indirectly by the prevector.
#[derive(Debug, Clone)]
pub struct Arc {
    pub edge: Edge,
    pub output: String,
    pub related: String,
    /// Start state of the measured edge (the prevector's target), over the primary inputs.
    pub start: Minterm<Symbol>,
    /// End state of the measured edge (defines the vector and the `-when` condition).
    pub end: Minterm<Symbol>,
    /// The prevector: the input-assignment sequence that drives every state variable into `start`.
    pub prevector: Vec<Minterm<Symbol>>,
    pub is_async: bool,
}

/// A whole-cell internal-power ('hidden') arc: the input `pin` toggles between two settled
/// states and NO output changes. Used for internal-power characterisation.
#[derive(Debug, Clone)]
pub struct HiddenArc {
    pub pin: Symbol,                    // the toggled primary input
    pub edge: Edge,                     // that input's Rise/Fall
    pub start: Minterm<Symbol>,         // input vector before the toggle
    pub end: Minterm<Symbol>,           // input vector after the toggle
    pub prevector: Vec<Minterm<Symbol>>,
    pub outputs: Vec<(Symbol, bool)>,   // each output's HELD logic value, in cell.outputs order
}

/// Derive transition arcs for every output of a cell by re-walking its shared asynchronous state machine
/// (see [`machine`] and [`Machine`]). A machine node is a fully-fixed [`Minterm<Symbol>`] over
/// `[inputs…, state_vars…]`. Also derives the whole-cell internal-power ('hidden') arcs — single input
/// toggles that settle but leave every output unchanged.
pub(crate) fn derive<B: Brand, C: ManagerCell>(m: &Machine<B, C>) -> (Vec<Arc>, Vec<HiddenArc>) {
    let cell = m.cell;
    let inputs = &cell.inputs;
    let state_set = &m.state_set;
    let deltas = &m.deltas;
    let out_delta = &m.out_deltas;
    let ex = &m.explored;

    // The value of `output` at a node, or `None` when the node does not define it: a state output reads
    // its state field (absent ⇒ undefined); a combinational output is its δ evaluated at the node
    // (`Err` ⇒ still depends on absent state ⇒ undefined). An arc is only measured where the output is
    // defined at both ends.
    let output_value = |name: &str, node: &Minterm<Symbol>| -> Option<bool> {
        if state_set.contains(name) {
            node.value_of(name)
        } else {
            // Every non-state output has a δ in `out_deltas` (one is computed for each of `cell.outputs`
            // when the machine is built), so this lookup cannot miss.
            debug_assert!(
                out_delta.contains_key(name),
                "derive: output {name:?} has no entry in out_deltas"
            );
            out_delta[name].evaluate(node).ok()
        }
    };

    let async_set: BTreeSet<&str> = cell.async_pins.iter().map(String::as_str).collect();
    // The same arc can be reached from several start candidates; keep the one with the shortest
    // prevector. Keyed by (output, related, edge-direction, start over the inputs).
    let mut best_arc: BTreeMap<(String, String, bool, Minterm<Symbol>), Arc> = BTreeMap::new();
    // Hidden ('hidden') arcs, deduped like `best_arc`: keyed by (toggled pin, edge-direction, start over
    // the inputs), keeping the one with the shortest prevector.
    let mut best_hidden: BTreeMap<(Symbol, bool, Minterm<Symbol>), HiddenArc> = BTreeMap::new();

    // Re-walk the reachable stable states in BFS order; wherever a single input toggle flips an output,
    // emit an arc.
    for node in &ex.order {
        for related in inputs {
            // Toggle one input, hold the (partial) state, and let the state settle.
            let toggled = machine::toggle(node, &[related.as_str()]);
            let Some(np) = machine::settle(deltas, &toggled) else {
                continue;
            };
            // An arc for every output that is defined at both ends and flips across this input toggle.
            let start = node.project_to(inputs);
            let end = np.project_to(inputs);
            let prevector = ex.path_to(node, inputs);
            // Collect each output's (before, after) once so both the transition and hidden paths read it.
            let vals: Vec<(&AnalysedOutput, Option<bool>, Option<bool>)> = cell
                .outputs
                .iter()
                .map(|o| (o, output_value(&o.name, node), output_value(&o.name, &np)))
                .collect();
            for (o, before, after) in &vals {
                let (Some(before), Some(after)) = (before, after) else {
                    continue;
                };
                if before == after {
                    continue;
                }
                let edge = if *after { Edge::Rise } else { Edge::Fall };
                let key = (o.name.clone(), related.clone(), *after, start.clone());
                let arc = Arc {
                    edge,
                    output: o.name.clone(),
                    related: related.clone(),
                    start: start.clone(),
                    end: end.clone(),
                    prevector: prevector.clone(),
                    is_async: async_set.contains(related.as_str()),
                };
                match best_arc.entry(key) {
                    std::collections::btree_map::Entry::Vacant(e) => {
                        e.insert(arc);
                    }
                    std::collections::btree_map::Entry::Occupied(mut e) => {
                        if arc.prevector.len() < e.get().prevector.len() {
                            e.insert(arc);
                        }
                    }
                }
            }

            // Hidden path: a settled input toggle where every output is defined at both ends and none of
            // them changed — internal-power characterisation.
            if !vals.is_empty()
                && vals
                    .iter()
                    .all(|(_, b, a)| matches!((b, a), (Some(b), Some(a)) if b == a))
            {
                let rose = end
                    .value_of(related.as_str())
                    .expect("toggled input is fully fixed in the settled end state");
                let pin = Symbol::from(related.as_str());
                let hidden = HiddenArc {
                    pin: pin.clone(),
                    edge: if rose { Edge::Rise } else { Edge::Fall },
                    start: start.clone(),
                    end: end.clone(),
                    prevector: prevector.clone(),
                    outputs: vals
                        .iter()
                        .map(|(o, _, a)| (Symbol::from(o.name.as_str()), a.unwrap()))
                        .collect(),
                };
                let key = (pin, rose, start.clone());
                match best_hidden.entry(key) {
                    std::collections::btree_map::Entry::Vacant(e) => {
                        e.insert(hidden);
                    }
                    std::collections::btree_map::Entry::Occupied(mut e) => {
                        if hidden.prevector.len() < e.get().prevector.len() {
                            e.insert(hidden);
                        }
                    }
                }
            }
        }
    }

    (
        best_arc.into_values().collect(),
        best_hidden.into_values().collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::analyse_one as analyse;

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
        let arcs = cell.arcs.clone();
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
    fn and2_has_hidden_arc_when_output_held() {
        // A falling while B=0 settles with Y held at 0: an internal-power hidden arc, not a transition.
        let cell = analyse(
            r#"
[[cell]]
name = "AND2"
inputs = ["A", "B"]
[cell.outputs]
Y = "A*B"
"#,
        );
        assert!(!cell.hidden_arcs.is_empty());
        assert!(cell.hidden_arcs.iter().any(|h| {
            h.pin.as_str() == "A"
                && h.edge == Edge::Fall
                && h.outputs.len() == 1
                && h.outputs[0].0.as_str() == "Y"
                && !h.outputs[0].1
        }));
        // Single-output cell: every hidden arc holds exactly one output value.
        assert!(cell.hidden_arcs.iter().all(|h| h.outputs.len() == 1));
        // Every hidden arc's prevector is a real single-step walk into its start state.
        for h in &cell.hidden_arcs {
            assert_eq!(h.prevector.last(), Some(&h.start));
            for w in h.prevector.windows(2) {
                assert_eq!(w[0].hamming_distance(&w[1]), 1);
            }
        }
    }

    #[test]
    fn inverter_has_no_hidden_arc() {
        // Toggling A always flips Y, so no toggle leaves the output unchanged.
        let cell = analyse(
            r#"
[[cell]]
name = "INV"
inputs = ["A"]
[cell.outputs]
Y = "!A"
"#,
        );
        assert!(cell.hidden_arcs.is_empty());
    }

    #[test]
    fn c_element_hidden_arcs_on_inputs_only() {
        // A toggle that keeps the C-element in hold leaves Q unchanged: a hidden arc on each input, but
        // never sourced by the output.
        let cell = analyse(
            r#"
[[cell]]
name = "C2"
inputs = ["A", "B"]
[cell.outputs]
Q = "A*B + Q*(A+B)"
"#,
        );
        assert!(cell.hidden_arcs.iter().any(|h| h.pin.as_str() == "A"));
        assert!(cell.hidden_arcs.iter().any(|h| h.pin.as_str() == "B"));
        assert!(cell.hidden_arcs.iter().all(|h| h.pin.as_str() != "Q"));
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
        let arcs = cell.arcs.clone();
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
        let arcs = cell.arcs.clone();
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
    fn dff_clk_to_q_arc_relates_only_inputs_and_prevector_sets_master() {
        // Rising-edge DFF: internal master M, external slave Q. The measured CLK→Q edge relates only
        // primary inputs (M is never a related pin); its prevector — inputs only — must first establish
        // the master (drive D so M holds the captured value) before the clock edge.
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
        let arcs = cell.arcs.clone();
        assert!(!arcs.is_empty());
        // Internal M is never an arc source or target; only Q is a target, only CLK/D are sources.
        assert!(arcs.iter().all(|a| a.output == "Q"));
        assert!(arcs.iter().all(|a| a.related == "CLK" || a.related == "D"));
        // A CLK-driven rise and fall of Q exist (the flop captures D through the clock edge).
        let clk_rise = arcs
            .iter()
            .find(|a| a.related == "CLK" && a.edge == Edge::Rise)
            .expect("a CLK→Q rise arc");
        assert!(arcs
            .iter()
            .any(|a| a.related == "CLK" && a.edge == Edge::Fall));
        // The prevector is a real single-step input walk terminating at the measured start state.
        assert_eq!(clk_rise.prevector.last().unwrap(), &clk_rise.start);
        for w in clk_rise.prevector.windows(2) {
            assert_eq!(w[0].hamming_distance(&w[1]), 1);
        }
        // Establishing the master requires driving D high somewhere along the prevector (Q rises only
        // if the captured master value is 1) — inputs alone set the internal state.
        use crate::logic::assignment;
        assert!(
            clk_rise
                .prevector
                .iter()
                .any(|m| *assignment(m).get("D").unwrap_or(&false)),
            "prevector must drive D high to load the master before the CLK edge"
        );
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
        let arcs = cell.arcs.clone();
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
        let arcs = cell.arcs.clone();
        assert!(arcs.iter().any(|a| a.related == "R" && a.is_async));
        assert!(arcs
            .iter()
            .filter(|a| a.related != "R")
            .all(|a| !a.is_async));
    }
}
