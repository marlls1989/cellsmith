//! Constraint-arc derivation from **confluence** of the asynchronous state machine.
//!
//! A delay arc ([`super::arcs`]) records a single input edge that *causes* an output edge. A
//! **constraint** arc instead records that two inputs must not change too close together — a setup/hold
//! (data vs clock) or a non-sequential/arbitration relation (two racing requests). The physical origin
//! of both is the same: for a pair of near-simultaneous input edges the machine is **non-confluent** —
//! the settled state depends on which edge lands first.
//!
//! For a reachable stable state `s` and an unordered input pair `{x, y}` (all other inputs held): settle
//! `x` then `y` (`s_xy`) and `y` then `x` (`s_yx`). If either oscillates or `s_xy == s_yx`, the pair is
//! **confluent** at `s` — no hazard. Otherwise it is **non-confluent**: the settled state depends on
//! which edge lands first — a timing hazard.
//!
//! A hazard's **kind is decided solely by the declared clock**, not by the geometry of the race: a pair
//! containing exactly one declared clock is a directed **setup/hold** (clock ← data — the DFF's `D`
//! around `CLK`); any other pair is a symmetric **non_seq** (a mutex's `A`/`B`, a C-element's `A↓`/`B↑`,
//! an SR latch's simultaneous release). Clocks are *declared*, never inferred: inferring one from the
//! race order is state-dependent — the same pins read one way from one held state and the other way from
//! another — so it distinguishes nothing real and is not used.
//!
//! The reachable states and the prevector into `s` come from the shared [`machine::explore`], the same
//! exploration the delay-arc BFS uses.

use std::collections::BTreeMap;

use espresso_logic::{bdd_builder, Minterm, Symbol};

use crate::logic::arcs::Edge;
use crate::logic::{machine, resolve};
use crate::model::{AnalysedCell, AnalysedOutput};

/// The kind of a constraint arc: a directed setup/hold (clock ← data) or a symmetric non-sequential
/// (arbitration / mutual-exclusion) relation between two request inputs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConstraintKind {
    SetupHold,
    NonSeq,
}

/// One constraint arc between two **primary inputs**. For [`ConstraintKind::SetupHold`], `related` is
/// the clock and `pin` the data pin; for [`ConstraintKind::NonSeq`], the two are symmetric requests.
#[derive(Debug, Clone)]
pub struct Constraint {
    pub kind: ConstraintKind,
    pub related: String,
    pub related_edge: Edge,
    pub pin: String,
    pub pin_edge: Edge,
    /// The prevector: the input-assignment path that drives every state variable into the state where
    /// the constraint manifests (each node projected onto the inputs).
    pub prevector: Vec<Minterm<Symbol>>,
}

impl Constraint {
    /// The input condition under which this hazard occurs: the two switching edges, plus any other
    /// inputs held at a fixed value in the pre-toggle state (e.g. `A↓ & B↑ with R=0`).
    pub fn condition(&self) -> String {
        let arrow = |e: Edge| {
            if matches!(e, Edge::Rise) {
                "↑"
            } else {
                "↓"
            }
        };
        let mut cond = format!(
            "{}{} & {}{}",
            self.related,
            arrow(self.related_edge),
            self.pin,
            arrow(self.pin_edge)
        );
        if let Some(state) = self.prevector.last() {
            let others: Vec<String> = state
                .vars()
                .iter()
                .zip(state.iter())
                .filter_map(|(n, v)| {
                    let name = n.as_str();
                    if name == self.related || name == self.pin {
                        return None;
                    }
                    v.map(|b| format!("{name}={}", if b { 1 } else { 0 }))
                })
                .collect();
            if !others.is_empty() {
                cond.push_str(&format!(" with {}", others.join(", ")));
            }
        }
        cond
    }
}

fn edge_from(node: &Minterm<Symbol>, name: &str) -> Edge {
    // The direction `name` toggles from its current value at `node`.
    if node.value_of(name) == Some(false) {
        Edge::Rise
    } else {
        Edge::Fall
    }
}

fn edge_char(e: Edge) -> char {
    match e {
        Edge::Rise => 'R',
        Edge::Fall => 'F',
    }
}

/// A canonical dedup key: setup/hold is directed; non_seq is unordered over its two pins.
fn constraint_key(c: &Constraint) -> String {
    match c.kind {
        ConstraintKind::SetupHold => format!(
            "SH|{}{}|{}{}",
            c.related,
            edge_char(c.related_edge),
            c.pin,
            edge_char(c.pin_edge)
        ),
        ConstraintKind::NonSeq => {
            let a = format!("{}{}", c.related, edge_char(c.related_edge));
            let b = format!("{}{}", c.pin, edge_char(c.pin_edge));
            let (lo, hi) = if a <= b { (a, b) } else { (b, a) };
            format!("NS|{lo}|{hi}")
        }
    }
}

/// Derive constraint arcs for a cell by testing pairwise input-order confluence of its state machine.
/// Empty for confluent cells (ordinary combinational / self-holding gates without arbitration).
pub fn cell_constraints(cell: &AnalysedCell) -> Vec<Constraint> {
    let inputs = &cell.inputs;
    let n = inputs.len();
    if n < 2 {
        return Vec::new(); // a constraint relates two inputs
    }

    let signals: Vec<&AnalysedOutput> = cell.signals().collect();
    let deps = resolve::dependency_map(&signals);
    let state_set = resolve::state_variables(&signals);
    let state_vars: Vec<String> = signals
        .iter()
        .map(|s| s.name.clone())
        .filter(|nm| state_set.contains(nm))
        .collect();
    let k = state_vars.len();
    if k == 0 {
        return Vec::new(); // no state to latch ⇒ always confluent
    }
    if n + k > 22 {
        return Vec::new(); // combinatorial blow-up guard (matches arcs::cell_arcs)
    }

    let builder = bdd_builder!();
    let bdds: BTreeMap<String, _> = signals
        .iter()
        .map(|s| (s.name.clone(), builder.build(&s.expr)))
        .collect();
    let deltas: Vec<machine::Delta<_, _>> = state_vars
        .iter()
        .map(|v| (v.clone(), resolve::delta(v, &bdds, &deps, &state_set)))
        .collect();

    let full_names: Vec<String> = inputs.iter().cloned().chain(state_vars.clone()).collect();
    let full_header = machine::header(&full_names);
    let input_header = machine::header(inputs);

    let ex = machine::explore(&deltas, &full_header, inputs, &state_vars);

    let settle_toggle = |node: &Minterm<Symbol>, name: &str| -> Option<Minterm<Symbol>> {
        let toggled = machine::node_from(&full_header, |nm| {
            let cur = node
                .value_of(nm)
                .expect("a header variable is fixed in the node");
            if nm == name {
                !cur
            } else {
                cur
            }
        });
        machine::settle(&deltas, &full_header, &toggled)
    };

    let path_to = |node: &Minterm<Symbol>| -> Vec<Minterm<Symbol>> {
        let mut chain = vec![node.clone()];
        let mut cur = node.clone();
        while let Some(Some(p)) = ex.prev.get(&cur) {
            chain.push(p.clone());
            cur = p.clone();
        }
        chain.reverse();
        chain
            .iter()
            .map(|m| m.project_onto(&input_header))
            .collect()
    };

    let is_clock = |p: &str| cell.clock_pins.iter().any(|c| c.as_str() == p);

    // Dedup by canonical key, keeping the shortest prevector; BTreeMap gives deterministic output order.
    let mut found: BTreeMap<String, Constraint> = BTreeMap::new();

    for s in &ex.order {
        for i in 0..n {
            for j in (i + 1)..n {
                let x = &inputs[i];
                let y = &inputs[j];

                let (Some(s_x), Some(s_y)) = (settle_toggle(s, x), settle_toggle(s, y)) else {
                    continue; // a single toggle oscillates → treat as confluent (no constraint)
                };
                let (Some(s_xy), Some(s_yx)) = (settle_toggle(&s_x, y), settle_toggle(&s_y, x))
                else {
                    continue;
                };
                if s_xy == s_yx {
                    continue; // confluent at this state — no hazard
                }

                // Non-confluent ⇒ a hazard. Its kind is decided solely by the declared clock: a pair
                // containing exactly one clock is a directed setup/hold (clock ← data); any other pair is
                // a symmetric non_seq. The order-lock geometry is deliberately not used — it is
                // state-dependent (the same pins/edges read asymmetric from one held state and symmetric
                // from another), so it is not an invariant of the hazard and distinguishes nothing.
                let cons = if is_clock(x) ^ is_clock(y) {
                    let (clk, data) = if is_clock(x) { (x, y) } else { (y, x) };
                    Constraint {
                        kind: ConstraintKind::SetupHold,
                        related: clk.clone(),
                        related_edge: edge_from(s, clk),
                        pin: data.clone(),
                        pin_edge: edge_from(s, data),
                        prevector: path_to(s),
                    }
                } else {
                    Constraint {
                        kind: ConstraintKind::NonSeq,
                        related: x.clone(),
                        related_edge: edge_from(s, x),
                        pin: y.clone(),
                        pin_edge: edge_from(s, y),
                        prevector: path_to(s),
                    }
                };

                let key = constraint_key(&cons);
                let shorter = found
                    .get(&key)
                    .is_none_or(|e| cons.prevector.len() < e.prevector.len());
                if shorter {
                    found.insert(key, cons);
                }
            }
        }
    }

    found.into_values().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::parse_spec;

    fn analyse(src: &str) -> AnalysedCell {
        parse_spec(src).unwrap().cells.remove(0).analyse().unwrap()
    }

    #[test]
    fn dff_with_declared_clock_yields_only_setup_hold() {
        // Rising-edge DFF with CLK declared a clock: the CLK↔D hazard is a setup/hold of D w.r.t. CLK,
        // and — because the kind follows the declared clock, not the geometry — nothing on the pair is
        // reported as non_seq.
        let cell = analyse(
            r#"
[[cell]]
name = "DFF"
inputs = ["CLK", "D"]
clock = ["CLK"]
[cell.internal]
M = "!CLK*D + CLK*M"
[cell.outputs]
Q = "CLK*M + !CLK*Q"
"#,
        );
        let cons = cell_constraints(&cell);
        eprintln!("DFF constraints: {cons:#?}");
        assert!(
            cons.iter().all(|c| c.kind == ConstraintKind::SetupHold),
            "a declared-clock DFF yields only setup/hold, got {cons:?}"
        );
        assert!(
            cons.iter()
                .any(|c| c.related == "CLK" && c.related_edge == Edge::Rise && c.pin == "D"),
            "expected a setup/hold of D around CLK↑, got {cons:?}"
        );
    }

    #[test]
    fn dff_without_declared_clock_is_non_seq() {
        // The same DFF with no clock declared: the hazard is real but, with no clock to designate a data
        // pin, it is a symmetric non_seq — the kind is a property of the declaration, not the cell.
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
        let cons = cell_constraints(&cell);
        assert!(!cons.is_empty());
        assert!(
            cons.iter().all(|c| c.kind == ConstraintKind::NonSeq),
            "an undeclared DFF yields only non_seq, got {cons:?}"
        );
    }

    #[test]
    fn mutex_has_non_seq_between_requests() {
        // Cross-coupled mutex: A and B race symmetrically — a non-sequential (arbitration) constraint,
        // kept because the divergence is on the interlocked grant outputs.
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
        let cons = cell_constraints(&cell);
        eprintln!("MUT constraints: {cons:#?}");
        assert!(
            cons.iter().any(|c| c.kind == ConstraintKind::NonSeq
                && [c.related.as_str(), c.pin.as_str()]
                    .iter()
                    .all(|p| *p == "A" || *p == "B")),
            "expected a non_seq constraint between A and B, got {cons:?}"
        );
        assert!(
            cons.iter().all(|c| c.kind == ConstraintKind::NonSeq),
            "a mutex yields only non_seq constraints, got {cons:?}"
        );
    }

    #[test]
    fn c_element_has_non_seq_hazard() {
        // A C-element is order-sensitive: A↓ racing B↑ leaves Q history-dependent. That is a real timing
        // hazard, filed as a non_seq constraint between A and B (not an arbitration, but a genuine one).
        let cell = analyse(
            r#"
[[cell]]
name = "C2"
inputs = ["A", "B"]
[cell.outputs]
Q = "A*B + Q*(A+B)"
"#,
        );
        let cons = cell_constraints(&cell);
        eprintln!("C2 constraints: {cons:#?}");
        assert!(
            cons.iter().any(|c| c.kind == ConstraintKind::NonSeq
                && [c.related.as_str(), c.pin.as_str()]
                    .iter()
                    .all(|p| *p == "A" || *p == "B")),
            "expected a non_seq hazard between A and B, got {cons:?}"
        );
    }

    #[test]
    fn sr_latch_has_non_seq_hazard() {
        // The SR latch's simultaneous release (11→00) is a real order-hazard, filed as a non_seq S↔R.
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
        let cons = cell_constraints(&cell);
        eprintln!("SR constraints: {cons:#?}");
        assert!(
            cons.iter().any(|c| c.kind == ConstraintKind::NonSeq),
            "expected a non_seq hazard between S and R, got {cons:?}"
        );
    }

    #[test]
    fn combinational_has_no_constraints() {
        let cell = analyse(
            r#"
[[cell]]
name = "ND2"
inputs = ["A", "B"]
[cell.outputs]
Y = "!(A*B)"
"#,
        );
        assert!(cell_constraints(&cell).is_empty());
    }
}
