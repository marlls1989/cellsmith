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

use espresso_logic::{bdd_builder, BoolExpr};

use crate::model::AnalysedOutput;

/// One metastable (arbitration) condition of a cell: an interlock group, the primary-input condition
/// under which it is bistable, and the competing stable states.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Arbitration {
    /// The mutually-exclusive output group (a size-≥2 SCC of the coupling graph), in output order.
    pub group: Vec<String>,
    /// Primary-input condition under which the group is metastable, as `(pin, value)` in input order.
    pub condition: Vec<(String, bool)>,
    /// The competing stable states — each a full assignment of the `group` outputs (output order),
    /// sorted for determinism.
    pub stable: Vec<Vec<(String, bool)>>,
}

impl Arbitration {
    /// The condition as a Boolean product of literals (`A*B`, `!R*S`, …).
    pub fn condition_str(&self) -> String {
        literals_str(&self.condition)
    }

    /// A competing stable state as a brace-wrapped literal product (`{Qa=1, Qb=0}`).
    pub fn state_str(state: &[(String, bool)]) -> String {
        let inner: Vec<String> = state
            .iter()
            .map(|(n, v)| format!("{n}={}", if *v { 1 } else { 0 }))
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
    // assignment determines each next-state completely.
    let mut state_outputs: Vec<String> = Vec::new();
    for o in outputs {
        for f in &o.feedback {
            if !state_outputs.contains(f) {
                state_outputs.push(f.clone());
            }
        }
    }

    // The next-state function of each held output.
    let exprs: BTreeMap<&str, &BoolExpr> = state_outputs
        .iter()
        .map(|name| {
            let expr = &outputs
                .iter()
                .find(|o| &o.name == name)
                .expect("state output is a declared output")
                .expr;
            (name.as_str(), expr)
        })
        .collect();

    let mut result = Vec::new();
    for group in &groups {
        for input_asn in assignments(inputs) {
            // Stable fixpoints under this input assignment, projected onto the group.
            let mut projections: BTreeSet<Vec<(String, bool)>> = BTreeSet::new();
            for state_asn in assignments(&state_outputs) {
                if is_fixpoint(&exprs, &state_outputs, &input_asn, &state_asn) {
                    let proj: Vec<(String, bool)> = group
                        .iter()
                        .map(|g| {
                            let v = state_asn
                                .iter()
                                .find(|(n, _)| n == g)
                                .map(|(_, v)| *v)
                                .expect("group member is a held output");
                            (g.clone(), v)
                        })
                        .collect();
                    projections.insert(proj);
                }
            }
            if projections.len() >= 2 {
                result.push(Arbitration {
                    group: group.clone(),
                    condition: input_asn,
                    stable: projections.into_iter().collect(),
                });
            }
        }
    }
    result
}

/// Whether `state_asn` is a stable joint state: every held output's next value equals its current one
/// under `input_asn ∪ state_asn`.
fn is_fixpoint(
    exprs: &BTreeMap<&str, &BoolExpr>,
    state_outputs: &[String],
    input_asn: &[(String, bool)],
    state_asn: &[(String, bool)],
) -> bool {
    state_outputs.iter().all(|name| {
        let next = eval(exprs[name.as_str()], input_asn, state_asn);
        let current = state_asn
            .iter()
            .find(|(n, _)| n == name)
            .map(|(_, v)| *v)
            .expect("state output is in the state assignment");
        next == current
    })
}

/// Evaluate a fully-assigned function to a constant by restricting every input and held-output name.
fn eval(expr: &BoolExpr, input_asn: &[(String, bool)], state_asn: &[(String, bool)]) -> bool {
    let builder = bdd_builder!();
    let mut cur = builder.build(expr);
    for (n, v) in input_asn.iter().chain(state_asn.iter()) {
        cur = cur.restrict(n.as_str(), *v);
    }
    debug_assert!(
        cur.is_tautology() || cur.is_contradiction(),
        "a full input+state assignment must determine the next-state"
    );
    cur.is_tautology()
}

/// The interlock groups: strongly-connected components of size ≥ 2 in the output coupling graph
/// (self-loops excluded), each in the cells' output order.
fn interlock_groups(outputs: &[AnalysedOutput]) -> Vec<Vec<String>> {
    let order: Vec<&str> = outputs.iter().map(|o| o.name.as_str()).collect();

    // edges: o -> p for each *other* output p that o's function references.
    let mut edges: BTreeMap<&str, BTreeSet<&str>> = BTreeMap::new();
    for o in outputs {
        let set = edges.entry(o.name.as_str()).or_default();
        for f in &o.feedback {
            if f != &o.name {
                set.insert(f.as_str());
            }
        }
    }

    // Transitive closure (≥1 step) via repeated relaxation — graphs are tiny.
    let mut reach = edges.clone();
    loop {
        let mut changed = false;
        let keys: Vec<&str> = reach.keys().copied().collect();
        for u in keys {
            let succ: Vec<&str> = reach[u].iter().copied().collect();
            for v in succ {
                let vs: Vec<&str> = reach
                    .get(v)
                    .map(|s| s.iter().copied().collect())
                    .unwrap_or_default();
                for w in vs {
                    if reach.get_mut(u).unwrap().insert(w) {
                        changed = true;
                    }
                }
            }
        }
        if !changed {
            break;
        }
    }

    // u and v (u != v) share an SCC iff each reaches the other.
    let mutual = |u: &str, v: &str| {
        reach.get(u).map(|s| s.contains(v)).unwrap_or(false)
            && reach.get(v).map(|s| s.contains(u)).unwrap_or(false)
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

/// Every assignment of `names` as `(name, value)` pairs in the given order (2^n rows).
fn assignments(names: &[String]) -> Vec<Vec<(String, bool)>> {
    let n = names.len();
    (0..(1u32 << n))
        .map(|mask| {
            names
                .iter()
                .enumerate()
                .map(|(i, nm)| (nm.clone(), (mask >> i) & 1 == 1))
                .collect()
        })
        .collect()
}

/// A product of literals: `A*B`, `!R*S`. Empty ⇒ the tautology `1`.
fn literals_str(lits: &[(String, bool)]) -> String {
    if lits.is_empty() {
        return "1".to_owned();
    }
    lits.iter()
        .map(|(n, v)| if *v { n.clone() } else { format!("!{n}") })
        .collect::<Vec<_>>()
        .join("*")
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
        assert!(a
            .stable
            .contains(&vec![("Qa".into(), true), ("Qb".into(), false)]));
        assert!(a
            .stable
            .contains(&vec![("Qa".into(), false), ("Qb".into(), true)]));
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
