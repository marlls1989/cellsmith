//! Transition-arc derivation over the cell's **asynchronous state machine**.
//!
//! A cell is a state machine over `inputs × state-variables` (each output's own feedback and every
//! internal state node; see [`resolve`]). A node is a fully-fixed [`Minterm<Symbol>`] over the shared
//! `[inputs…, state_vars…]` header ([`machine`]). Arcs are derived by exploring it:
//!
//!   1. Build each state variable's next-state δ ([`resolve::delta`]); [`machine::settle`] applies them
//!      via [`Bdd::evaluate`] until the state stops changing.
//!   2. BFS from the reset-stable states (state stable under the all-zero input), stepping one input
//!      at a time and letting the state settle. Metastable transitions (the state oscillates instead
//!      of settling — a mutex's deadlock) yield no fixpoint and are dropped, so no impossible arc is
//!      produced.
//!   3. Wherever a single input toggle flips an **output**, emit an arc: the toggled input is the
//!      `related` pin (arcs are only ever sourced by primary inputs — never an output or internal),
//!      and the prevector is the BFS path — each node projected onto the inputs — that drives every
//!      state variable (internal ones included) into the measured edge's start state.

use std::collections::{BTreeMap, BTreeSet, HashMap, VecDeque};

use espresso_logic::{bdd_builder, Minterm, Symbol};

use crate::logic::{machine, resolve};
use crate::model::AnalysedCell;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Edge {
    Rise,
    Fall,
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

/// Derive transition arcs for every output of a cell by exploring its asynchronous state machine
/// (see [`machine`]). A machine node is a fully-fixed [`Minterm<Symbol>`] over `[inputs…, state_vars…]`.
pub fn cell_arcs(cell: &AnalysedCell) -> Vec<Arc> {
    let inputs = &cell.inputs;
    let n = inputs.len();

    let signals: Vec<&crate::model::AnalysedOutput> = cell.signals().collect();
    let deps = resolve::dependency_map(&signals);
    let state_set = resolve::state_variables(&signals);
    // State variables in signal order (outputs first, then internals).
    let state_vars: Vec<String> = signals
        .iter()
        .map(|s| s.name.clone())
        .filter(|nm| state_set.contains(nm))
        .collect();
    let k = state_vars.len();

    // Guard against a combinatorial blow-up on pathologically wide cells.
    if n + k > 22 {
        return Vec::new();
    }

    let builder = bdd_builder!();
    let bdds: BTreeMap<String, _> = signals
        .iter()
        .map(|s| (s.name.clone(), builder.build(&s.expr)))
        .collect();

    // δ of each state variable (the machine's transition functions), and of each *combinational*
    // output (a state output instead reads its own state field of the node).
    let deltas: Vec<machine::Delta<_, _>> = state_vars
        .iter()
        .map(|v| (v.clone(), resolve::delta(v, &bdds, &deps, &state_set)))
        .collect();
    let out_delta: BTreeMap<String, _> = cell
        .outputs
        .iter()
        .filter(|o| !state_set.contains(&o.name))
        .map(|o| {
            (
                o.name.clone(),
                resolve::delta(&o.name, &bdds, &deps, &state_set),
            )
        })
        .collect();

    // The shared headers: the full node header (inputs + state variables) and the input-only header the
    // arcs are expressed over.
    let full_names: Vec<String> = inputs.iter().cloned().chain(state_vars.clone()).collect();
    let full_header = machine::header(&full_names);
    let input_header = machine::header(inputs);

    // The value of `output` at a node: a state output reads its state field; a combinational output is
    // its δ evaluated at the node.
    let output_value = |name: &str, node: &Minterm<Symbol>| -> bool {
        if state_set.contains(name) {
            node.value_of(name)
                .expect("a state variable is fixed in the node")
        } else {
            out_delta[name]
                .evaluate(node)
                .expect("a complete assignment determines a combinational output")
        }
    };

    // A full node from an input assignment `x` (bit i = inputs[i]) and a state assignment `s`
    // (bit i = state_vars[i]).
    let bit = |mask: usize, list: &[String], name: &str| -> Option<bool> {
        list.iter()
            .position(|v| v == name)
            .map(|i| (mask >> i) & 1 == 1)
    };
    let make_node = |x: usize, s: usize| -> Minterm<Symbol> {
        machine::node_from(&full_header, |name| {
            bit(x, inputs, name)
                .or_else(|| bit(s, &state_vars, name))
                .expect("every header variable is an input or a state variable")
        })
    };

    let n_st = 1usize << k;

    // Reset-stable states: state stable under the all-zero input. Fall back to every stable node if the
    // all-zero input has no stable state.
    let mut starts: Vec<Minterm<Symbol>> = (0..n_st)
        .map(|s| make_node(0, s))
        .filter(|node| machine::is_stable(&deltas, node))
        .collect();
    if starts.is_empty() {
        starts = (0..(1usize << n))
            .flat_map(|x| (0..n_st).map(move |s| (x, s)))
            .map(|(x, s)| make_node(x, s))
            .filter(|node| machine::is_stable(&deltas, node))
            .collect();
    }

    // BFS over stable nodes; `prev[node] = predecessor` for prevector reconstruction.
    let mut prev: HashMap<Minterm<Symbol>, Option<Minterm<Symbol>>> = HashMap::new();
    let mut queue: VecDeque<Minterm<Symbol>> = VecDeque::new();
    for st in &starts {
        prev.entry(st.clone()).or_insert(None);
    }
    queue.extend(starts.iter().cloned());

    let async_set: BTreeSet<&str> = cell.async_pins.iter().map(String::as_str).collect();
    let mut seen_arc: BTreeSet<(String, String, bool, Minterm<Symbol>)> = BTreeSet::new();
    let mut arcs: Vec<Arc> = Vec::new();

    // The prevector: the input assignments from a start node to `node`, each projected onto the inputs.
    let path_to = |prev: &HashMap<Minterm<Symbol>, Option<Minterm<Symbol>>>,
                   node: &Minterm<Symbol>|
     -> Vec<Minterm<Symbol>> {
        let mut chain = vec![node.clone()];
        let mut cur = node.clone();
        while let Some(Some(p)) = prev.get(&cur) {
            chain.push(p.clone());
            cur = p.clone();
        }
        chain.reverse();
        chain
            .iter()
            .map(|m| m.project_onto(&input_header))
            .collect()
    };

    while let Some(node) = queue.pop_front() {
        for related in inputs {
            // Toggle one input, hold the state, and let the state settle.
            let toggled = machine::node_from(&full_header, |name| {
                let cur = node
                    .value_of(name)
                    .expect("a header variable is fixed in the node");
                if name == related.as_str() {
                    !cur
                } else {
                    cur
                }
            });
            let Some(np) = machine::settle(&deltas, &full_header, &toggled) else {
                continue;
            };
            if let std::collections::hash_map::Entry::Vacant(e) = prev.entry(np.clone()) {
                e.insert(Some(node.clone()));
                queue.push_back(np.clone());
            }
            // An arc for every output whose value flips across this input toggle.
            let start = node.project_onto(&input_header);
            let end = np.project_onto(&input_header);
            for o in &cell.outputs {
                let before = output_value(&o.name, &node);
                let after = output_value(&o.name, &np);
                if before == after {
                    continue;
                }
                let edge = if after { Edge::Rise } else { Edge::Fall };
                let key = (o.name.clone(), related.clone(), after, start.clone());
                if !seen_arc.insert(key) {
                    continue;
                }
                arcs.push(Arc {
                    edge,
                    output: o.name.clone(),
                    related: related.clone(),
                    start: start.clone(),
                    end: end.clone(),
                    prevector: path_to(&prev, &node),
                    is_async: async_set.contains(related.as_str()),
                });
            }
        }
    }

    arcs
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
        let arcs = cell_arcs(&cell);
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
        let arcs = cell_arcs(&cell);
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
        let arcs = cell_arcs(&cell);
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
        let arcs = cell_arcs(&cell);
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
        let arcs = cell_arcs(&cell);
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
        let arcs = cell_arcs(&cell);
        assert!(arcs.iter().any(|a| a.related == "R" && a.is_async));
        assert!(arcs
            .iter()
            .filter(|a| a.related != "R")
            .all(|a| !a.is_async));
    }
}
