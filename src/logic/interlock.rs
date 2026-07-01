//! Arbitration / interlock detection for mutually cross-coupled cells (mutexes, arbiters).
//!
//! A state-holding output references another output as a delayed/feedback value. When two (or more)
//! outputs reference *each other* — a coupling cycle spanning **distinct** outputs — the cell can be
//! **bistable**: under some primary-input condition the joint next-state relation has more than one
//! stable state, and which one the physical cell settles into is a non-deterministic (metastable)
//! choice. That is arbitration, and it cannot be expressed by a deterministic timing arc.
//!
//! We *detect and report* it rather than fabricate arc behaviour for it. Detection is structural
//! plus a small brute-force fixpoint search (cell pin counts are tiny):
//!
//! 1. Build the output coupling graph (edge `o → p` iff `o`'s function references the *other* output
//!    `p`). Self-loops — a C-element holding its own state — are ordinary hysteresis, not arbitration,
//!    so they are excluded. Strongly-connected components of size ≥ 2 are the **interlock groups**.
//! 2. For each primary-input assignment, enumerate the joint current-state over the held outputs and
//!    keep the **stable fixpoints** (`next == current`). An input assignment whose fixpoints, projected
//!    onto an interlock group, take ≥ 2 distinct values is a **metastable (arbitration) condition**.

use std::collections::{BTreeMap, BTreeSet};

use espresso_logic::{bdd_builder, Minterm, Symbol};

use crate::logic::{machine, resolve};
use crate::model::AnalysedOutput;

/// One metastable (arbitration) condition of a cell: an interlock group, the primary-input condition
/// under which it is bistable, and the competing stable states.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Arbitration {
    /// The mutually-exclusive output group (a size-≥2 SCC of the coupling graph), in output order.
    pub group: Vec<String>,
    /// Primary-input condition under which the group is metastable, as a full input assignment.
    pub condition: Minterm<Symbol>,
    /// The competing stable states — each a group-projected minterm (group order), sorted for
    /// determinism.
    pub stable: Vec<Minterm<Symbol>>,
}

impl Arbitration {
    /// The condition as a Boolean product of literals (`A*B`, `!R*S`, …).
    pub fn condition_str(&self) -> String {
        literals_str(&self.condition)
    }

    /// A competing stable state as a brace-wrapped literal product (`{Qa=1, Qb=0}`).
    pub fn state_str(state: &Minterm<Symbol>) -> String {
        let inner: Vec<String> = state
            .vars()
            .iter()
            .zip(state.iter())
            .filter_map(|(n, v)| v.map(|b| format!("{}={}", n.as_str(), if b { 1 } else { 0 })))
            .collect();
        format!("{{{}}}", inner.join(", "))
    }
}

/// Detect every arbitration condition of a cell. Empty for ordinary combinational or self-holding
/// (C-element / latch / non-mutual SR) cells.
pub fn detect(inputs: &[String], outputs: &[AnalysedOutput]) -> Vec<Arbitration> {
    let groups = interlock_groups(outputs);
    if groups.is_empty() {
        return Vec::new();
    }

    // The held outputs: any output referenced as feedback by some function (self or other). These are
    // the joint-state coordinates; every referenced output is one of them, so a full input+state
    // assignment determines each next-state completely. A held output's own function *is* its
    // next-state as a function of inputs + held outputs, so these double as the machine's δ.
    let mut state_outputs: Vec<String> = Vec::new();
    for o in outputs {
        for f in &o.feedback {
            if !state_outputs.contains(f) {
                state_outputs.push(f.clone());
            }
        }
    }

    let builder = bdd_builder!();
    let deltas: Vec<machine::Delta<_, _>> = state_outputs
        .iter()
        .map(|name| {
            let expr = &outputs
                .iter()
                .find(|o| &o.name == name)
                .expect("state output is a declared output")
                .expr;
            (name.clone(), builder.build(expr))
        })
        .collect();

    // Shared headers: the full node (inputs + held outputs) and the input-only condition header.
    let full_names: Vec<String> = inputs
        .iter()
        .cloned()
        .chain(state_outputs.clone())
        .collect();
    let full_header = machine::header(&full_names);
    let input_header = machine::header(inputs);

    let bit = |mask: usize, list: &[String], name: &str| -> Option<bool> {
        list.iter()
            .position(|v| v == name)
            .map(|i| (mask >> i) & 1 == 1)
    };
    let n_in = 1usize << inputs.len();
    let n_st = 1usize << state_outputs.len();

    let mut result = Vec::new();
    for group in &groups {
        let group_header = machine::header(group);
        for x in 0..n_in {
            // Stable joint states under this input assignment, projected onto the group.
            let mut projections: BTreeSet<Minterm<Symbol>> = BTreeSet::new();
            for s in 0..n_st {
                let node = machine::node_from(&full_header, |name| {
                    bit(x, inputs, name)
                        .or_else(|| bit(s, &state_outputs, name))
                        .expect("every header variable is an input or a held output")
                });
                if machine::is_stable(&deltas, &node) {
                    projections.insert(node.project_onto(&group_header));
                }
            }
            if projections.len() >= 2 {
                let condition =
                    machine::node_from(&input_header, |name| bit(x, inputs, name).unwrap_or(false));
                result.push(Arbitration {
                    group: group.clone(),
                    condition,
                    stable: projections.into_iter().collect(),
                });
            }
        }
    }
    result
}

/// The interlock groups: strongly-connected components of size ≥ 2 in the output coupling graph
/// (self-loops excluded), each in the cells' output order.
fn interlock_groups(outputs: &[AnalysedOutput]) -> Vec<Vec<String>> {
    let order: Vec<&str> = outputs.iter().map(|o| o.name.as_str()).collect();

    // edges: o -> p for each *other* output p that o's function references (self-loops excluded).
    let edges: BTreeMap<String, Vec<String>> = outputs
        .iter()
        .map(|o| {
            let succ = o
                .feedback
                .iter()
                .filter(|f| **f != o.name)
                .cloned()
                .collect();
            (o.name.clone(), succ)
        })
        .collect();
    let reach = resolve::transitive_closure(&edges);

    // u and v (u != v) share an SCC iff each reaches the other.
    let mutual = |u: &str, v: &str| {
        reach.get(u).is_some_and(|s| s.contains(v)) && reach.get(v).is_some_and(|s| s.contains(u))
    };

    let mut groups: Vec<Vec<String>> = Vec::new();
    let mut placed: BTreeSet<&str> = BTreeSet::new();
    for &u in &order {
        if placed.contains(u) {
            continue;
        }
        let members: Vec<&str> = order
            .iter()
            .copied()
            .filter(|&v| v == u || mutual(u, v))
            .collect();
        if members.len() >= 2 {
            for &m in &members {
                placed.insert(m);
            }
            groups.push(members.into_iter().map(String::from).collect());
        }
    }
    groups
}

/// A minterm's fixed values as a product of literals: `A*B`, `!R*S` (in the minterm's variable order).
/// No fixed value ⇒ the tautology `1`.
fn literals_str(m: &Minterm<Symbol>) -> String {
    let lits: Vec<String> = m
        .vars()
        .iter()
        .zip(m.iter())
        .filter_map(|(n, v)| {
            v.map(|b| {
                if b {
                    n.as_str().to_string()
                } else {
                    format!("!{}", n.as_str())
                }
            })
        })
        .collect();
    if lits.is_empty() {
        "1".to_owned()
    } else {
        lits.join("*")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{parse_spec, AnalysedCell};

    fn analyse(src: &str) -> AnalysedCell {
        parse_spec(src).unwrap().cells.remove(0).analyse().unwrap()
    }

    #[test]
    fn mutex_has_one_arbitration_point() {
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
        let arb = detect(&cell.inputs, &cell.outputs);
        assert_eq!(arb.len(), 1, "exactly one metastable condition");
        let a = &arb[0];
        assert_eq!(a.group, ["Qa", "Qb"]);
        assert_eq!(a.condition_str(), "A*B");
        // Competing stable states: Qa high / Qb low, and the mirror.
        assert_eq!(a.stable.len(), 2);
        let states: BTreeSet<String> = a.stable.iter().map(Arbitration::state_str).collect();
        assert_eq!(
            states,
            ["{Qa=1, Qb=0}".to_string(), "{Qa=0, Qb=1}".to_string()]
                .into_iter()
                .collect()
        );
    }

    #[test]
    fn c_element_self_hold_is_not_arbitration() {
        // A C-element is bistable in the hold region, but that is self-feedback, not mutual coupling.
        let cell = analyse(
            r#"
[[cell]]
name = "C2"
inputs = ["A", "B"]
[cell.outputs]
Q = "A*B + Q*(A+B)"
"#,
        );
        assert!(detect(&cell.inputs, &cell.outputs).is_empty());
    }

    #[test]
    fn non_mutual_sr_is_not_arbitration() {
        // These SR functions each reference only their own state (no mutual edge), so no interlock.
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
        assert!(detect(&cell.inputs, &cell.outputs).is_empty());
    }

    #[test]
    fn combinational_is_not_arbitration() {
        let cell = analyse(
            r#"
[[cell]]
name = "ND2"
inputs = ["A", "B"]
[cell.outputs]
Y = "!(A*B)"
"#,
        );
        assert!(detect(&cell.inputs, &cell.outputs).is_empty());
    }
}
