//! Constraint-arc and arbitration derivation from **confluence** of the asynchronous state machine.
//!
//! A delay arc ([`super::arcs`]) records a single input edge that *causes* an output edge. A
//! **constraint** arc instead records that two inputs must not change too close together — a setup/hold
//! (data vs clock) or a non-sequential/arbitration relation (two racing requests). The physical origin
//! of both is the same: for a pair of near-simultaneous input edges the machine is **non-confluent** —
//! the settled state depends on which edge lands first.
//!
//! For a reachable stable state `s` and an unordered input pair `{x, y}` (all other inputs held): settle
//! `x` then `y` (`s_xy`) and `y` then `x` (`s_yx`). If either oscillates or `s_xy == s_yx`, the pair is
//! **confluent** at `s` — no hazard. Otherwise the state has diverged, but global divergence alone is not
//! the verdict: it must *interact* with the racing pair in the immediate combinational neighbourhood —
//! some diverging state variable `w` (`s_xy.value_of(w) != s_yx.value_of(w)`) must have **both** `x` and
//! `y` in the direct support of its transition function `δ_w`. [`resolve::delta`] composes through
//! combinational logic only, so a state variable stays a variable in `δ_w` — both pins in `δ_w`'s direct
//! support means the pins meet within one combinational neighbourhood. A divergence mediated only across
//! a latch boundary — `δ_w` does not itself see both pins — is a settled snapshot carried across that
//! latch (e.g. the two domains of a dual-clock synchroniser), design-tolerated rather than a pin-pair
//! hazard.
//!
//! The same pass also detects **arbitration/metastability**: probed from `s`, the pair applied
//! *simultaneously* (or, degenerately, a single input toggle) can drive the state into a **periodic
//! oscillation** rather than a fixpoint ([`machine::settle_or_cycle`] returning the cycle instead of
//! settling). That is reported as an [`Arbitration`] — distinct from order-dependence: a mutex *is*
//! order-dependent by design (that is its function as an arbiter); its hazard is the oscillation at
//! simultaneity, not the ordinary settling of one request before the other. For the two-input case, that
//! same simultaneous-toggle oscillation is *also* filed as the pair's [`Constraint`] (kind still decided
//! by the declared-clock rule below): metastability at simultaneity **is** the physical origin of the
//! pair's timing constraint, and it is what supplies one for an arbitrating pair (a mutex's `A`/`B`)
//! whose order-divergence the combinational-neighbourhood filter above discards — the grant latches that
//! diverge do not have both racing pins in their own `δ`'s direct support.
//!
//! A constraint's **kind is decided solely by the declared clock**, not by the geometry of the race: a pair
//! containing exactly one declared clock is a directed **setup/hold** (clock ← data — the DFF's `D`
//! around `CLK`); any other pair is a symmetric **non_seq** (a mutex's `A`/`B`, a C-element's `A↓`/`B↑`,
//! an SR latch's simultaneous release). Clocks are *declared*, never inferred: inferring one from the
//! race order is state-dependent — the same pins read one way from one held state and the other way from
//! another — so it distinguishes nothing real and is not used.
//!
//! The reachable states and the prevector into `s` come from the shared [`machine::explore`], the same
//! exploration the delay-arc BFS uses.

use std::collections::{BTreeMap, BTreeSet};

use espresso_logic::{bdd_builder, Minterm, Symbol};

use crate::logic::arcs::Edge;
use crate::logic::interlock::{self, Arbitration};
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
    /// The input condition under which the hazard this constraint avoids occurs: the two switching
    /// edges, plus any other
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

/// The outcome of one hazard-analysis pass over the reachable state machine: the constraints derived
/// to avoid the cell's hazards (order-dependence and oscillation), and the arbitrations annotating its
/// metastable oscillations.
#[derive(Debug, Default)]
pub struct HazardAnalysis {
    pub constraints: Vec<Constraint>,
    pub arbitration: Vec<Arbitration>,
}

/// The state variables that oscillate across a `settle_or_cycle` cycle (`value_of` differs between any
/// two cycle nodes — `Some(v)` vs `None` counts as differing), in `state_vars` declaration order.
fn oscillating_group(cycle: &[Minterm<Symbol>], state_vars: &[String]) -> Vec<String> {
    state_vars
        .iter()
        .filter(|v| {
            let mut vals = cycle.iter().map(|m| m.value_of(v.as_str()));
            let first = vals.next();
            vals.any(|val| Some(val) != first)
        })
        .cloned()
        .collect()
}

/// Analyse a cell's hazards by testing pairwise input-order confluence of its state machine, deriving
/// the constraints that avoid them and the arbitration annotations. Empty for confluent cells (ordinary
/// combinational / self-holding gates without arbitration).
pub fn analyse_hazards(cell: &AnalysedCell) -> HazardAnalysis {
    let inputs = &cell.inputs;
    let n = inputs.len();
    if n < 2 {
        return HazardAnalysis::default(); // a hazard relates two inputs
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
        return HazardAnalysis::default(); // no state to latch ⇒ always confluent
    }
    if n + k > 22 {
        return HazardAnalysis::default(); // combinatorial blow-up guard (matches arcs::cell_arcs)
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
    // The direct support of every state variable's δ — precomputed once, used by the
    // combinational-neighbourhood divergence filter below (see the module doc).
    let support: BTreeMap<String, BTreeSet<String>> = deltas
        .iter()
        .map(|(n, d)| (n.clone(), d.variables().map(|v| v.as_str().to_string()).collect()))
        .collect();

    let full_names: Vec<String> = inputs.iter().cloned().chain(state_vars.clone()).collect();
    let full_header = machine::header(&full_names);
    let input_header = machine::header(inputs);

    // Seed candidates from the on/off covers of every signal function (state δ plus the combinational
    // outputs), matching [`super::arcs`].
    let seed_funcs: Vec<_> = deltas
        .iter()
        .map(|(_, d)| d.clone())
        .chain(
            cell.outputs
                .iter()
                .filter(|o| !state_set.contains(&o.name))
                .map(|o| resolve::delta(&o.name, &bdds, &deps, &state_set)),
        )
        .collect();
    let ex = machine::explore(&deltas, &seed_funcs, &full_header, inputs, &state_vars);

    let settle_toggle = |node: &Minterm<Symbol>,
                          names: &[&str]|
     -> Result<Minterm<Symbol>, Vec<Minterm<Symbol>>> {
        let toggled = machine::node_from_opt(&full_header, |nm| {
            let cur = node.value_of(nm);
            if names.contains(&nm) {
                cur.map(|v| !v)
            } else {
                cur
            }
        });
        machine::settle_or_cycle(&deltas, &full_header, &toggled)
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

    // Dedup by `group|condition`, keeping the FIRST insertion (BFS order over `ex.order` → the earliest
    // reachable state at which the arbitration is observed).
    let mut arbitration: BTreeMap<String, Arbitration> = BTreeMap::new();
    let mut record_arbitration =
        |node: &Minterm<Symbol>, names: &[&str], group: Vec<String>, stable: Vec<Minterm<Symbol>>| {
            let toggled = machine::node_from_opt(&full_header, |nm| {
                let cur = node.value_of(nm);
                if names.contains(&nm) {
                    cur.map(|v| !v)
                } else {
                    cur
                }
            });
            let condition = toggled.project_onto(&input_header);
            let key = format!("{}|{}", group.join(","), interlock::literals_str(&condition));
            arbitration
                .entry(key)
                .or_insert_with(|| Arbitration {
                    group,
                    condition,
                    stable,
                });
        };

    for s in &ex.order {
        for i in 0..n {
            for j in (i + 1)..n {
                let x = &inputs[i];
                let y = &inputs[j];

                let r_x = settle_toggle(s, &[x.as_str()]);
                let r_y = settle_toggle(s, &[y.as_str()]);

                // Single-toggle oscillation capture: a lone input toggle that never settles is itself an
                // arbitration (no competing order to report — `stable` is empty).
                if let Err(cycle) = &r_x {
                    let group = oscillating_group(cycle, &state_vars);
                    record_arbitration(s, &[x.as_str()], group, Vec::new());
                }
                if let Err(cycle) = &r_y {
                    let group = oscillating_group(cycle, &state_vars);
                    record_arbitration(s, &[y.as_str()], group, Vec::new());
                }

                // Simultaneous probe: x and y toggled together. Oscillation here is the mutex/arbiter
                // case proper — the pair asserted at once, driving the state into a periodic cycle.
                let r_sim = settle_toggle(s, &[x.as_str(), y.as_str()]);
                if let Err(cycle) = &r_sim {
                    let group = oscillating_group(cycle, &state_vars);
                    let group_header = machine::header(&group);
                    let mut stable_set: BTreeSet<Minterm<Symbol>> = BTreeSet::new();
                    if let Ok(sx) = &r_x {
                        if let Ok(sxy) = settle_toggle(sx, &[y.as_str()]) {
                            stable_set.insert(sxy.project_onto(&group_header));
                        }
                    }
                    if let Ok(sy) = &r_y {
                        if let Ok(syx) = settle_toggle(sy, &[x.as_str()]) {
                            stable_set.insert(syx.project_onto(&group_header));
                        }
                    }
                    record_arbitration(
                        s,
                        &[x.as_str(), y.as_str()],
                        group,
                        stable_set.into_iter().collect(),
                    );

                    // Metastability at simultaneity is itself the pair's timing-constraint origin: file
                    // it into the same dedup map the divergence path below uses, built exactly the same
                    // way (kind by the declared-clock rule, edges at `s`, prevector = path_to(s)). This
                    // supplies an arbitrating pair's (e.g. a mutex's) constraint, replacing the
                    // divergence-derived one the combinational-neighbourhood filter discards for it.
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

                let (Some(s_x), Some(s_y)) = (r_x.as_ref().ok(), r_y.as_ref().ok()) else {
                    continue; // a single toggle oscillates → treat as confluent (no constraint)
                };
                let (Some(s_xy), Some(s_yx)) = (
                    settle_toggle(s_x, &[y.as_str()]).ok(),
                    settle_toggle(s_y, &[x.as_str()]).ok(),
                ) else {
                    continue;
                };
                if s_xy == s_yx {
                    continue; // confluent at this state — no hazard
                }

                // Global divergence is not enough: it must interact with {x, y} in the immediate
                // combinational neighbourhood — some state variable that actually diverges between the
                // two settle orders must have BOTH x and y in the direct support of its own δ. Otherwise
                // the divergence is a settled snapshot mediated across a latch boundary (e.g. a
                // dual-clock synchroniser's two domains), not a pin-pair hazard.
                let interacts = state_vars.iter().any(|w| {
                    s_xy.value_of(w.as_str()) != s_yx.value_of(w.as_str())
                        && support[w].contains(x.as_str())
                        && support[w].contains(y.as_str())
                });
                if !interacts {
                    continue; // divergence real but latch-mediated — no constraint
                }

                // Non-confluent and interacting ⇒ order-dependence: file the constraint that avoids it.
                // The constraint's kind is decided solely by the declared clock: a pair containing
                // exactly one clock is a directed setup/hold (clock ← data); any other pair is a
                // symmetric non_seq. The order-lock geometry is deliberately not used — it is
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

    HazardAnalysis {
        constraints: found.into_values().collect(),
        arbitration: arbitration.into_values().collect(),
    }
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
        // Rising-edge DFF with CLK declared a clock: the CLK↔D hazard yields a setup/hold constraint of
        // D w.r.t. CLK, and — because the kind follows the declared clock, not the geometry — nothing on
        // the pair is reported as non_seq.
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
        let cons = analyse_hazards(&cell).constraints;
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
        // pin, its constraint is a symmetric non_seq — the kind is a property of the declaration, not
        // the cell.
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
        let cons = analyse_hazards(&cell).constraints;
        assert!(!cons.is_empty());
        assert!(
            cons.iter().all(|c| c.kind == ConstraintKind::NonSeq),
            "an undeclared DFF yields only non_seq, got {cons:?}"
        );
    }

    #[test]
    fn mutex_has_non_seq_between_requests() {
        // Cross-coupled mutex: A and B race symmetrically. Their order-divergence is on the interlocked
        // grant outputs (Qa/Qb), neither of which has *both* A and B in its own δ's direct support, so
        // the combinational-neighbourhood filter discards that divergence-derived constraint — but the
        // simultaneous A*B toggle drives the state into oscillation (arbitration), and that metastability
        // is itself filed as the pair's non_seq constraint.
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
        let cons = analyse_hazards(&cell).constraints;
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
    fn c_element_has_non_seq_constraint() {
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
        let cons = analyse_hazards(&cell).constraints;
        eprintln!("C2 constraints: {cons:#?}");
        assert!(
            cons.iter().any(|c| c.kind == ConstraintKind::NonSeq
                && [c.related.as_str(), c.pin.as_str()]
                    .iter()
                    .all(|p| *p == "A" || *p == "B")),
            "expected a non_seq constraint between A and B, got {cons:?}"
        );
    }

    #[test]
    fn sr_latch_has_non_seq_constraint() {
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
        let cons = analyse_hazards(&cell).constraints;
        eprintln!("SR constraints: {cons:#?}");
        assert!(
            cons.iter().any(|c| c.kind == ConstraintKind::NonSeq),
            "expected a non_seq constraint between S and R, got {cons:?}"
        );
    }

    #[test]
    fn latch_mediated_divergence_is_not_a_constraint() {
        // Two-domain sampling chain: M1 (transparent when C1 is low) samples D; Q (transparent when C2
        // is low) samples M1. No clocks declared, so every derived constraint here is NonSeq. A (C1, C2)
        // order-divergence is real (e.g. whether Q ends up latching M1's old value or D's new one
        // depends on whether C2 or C1 closes first) but is mediated only across the M1↔Q latch
        // boundary: neither δ_M1 (support {C1, D, M1}) nor δ_Q (support {C2, M1, Q}) has both C1 and C2
        // in its own direct support, so it must be filtered. The (C1, D) hazard is direct — δ_M1 has
        // both C1 and D — and must survive.
        let cell = analyse(
            r#"
[[cell]]
name = "SYNC2"
inputs = ["C1", "C2", "D"]
[cell.internal]
M1 = "!C1*D + C1*M1"
[cell.outputs]
Q = "!C2*M1 + C2*Q"
"#,
        );
        let cons = analyse_hazards(&cell).constraints;
        eprintln!("SYNC2 constraints: {cons:#?}");
        assert!(
            !cons.iter().any(|c| [c.related.as_str(), c.pin.as_str()]
                .iter()
                .all(|p| *p == "C1" || *p == "C2")),
            "the C1/C2 divergence is latch-mediated and must be filtered, got {cons:?}"
        );
        assert!(
            cons.iter().any(|c| [c.related.as_str(), c.pin.as_str()]
                .iter()
                .all(|p| *p == "C1" || *p == "D")),
            "expected a constraint for the genuine C1/D hazard (direct support of δ_M1), got {cons:?}"
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
        assert!(analyse_hazards(&cell).constraints.is_empty());
    }
}
