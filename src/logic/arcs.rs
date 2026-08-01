//! Transition-arc derivation over the cell's **asynchronous state machine**.
//!
//! A cell is a state machine over `inputs × state-variables` (each output's own feedback and every
//! internal state node; see [`resolve`](super::resolve)). A node is a [`Minterm<Symbol>`] over
//! `inputs…, state_vars…` ([`machine`]); traversal states may be partial — an uninitialised latch
//! leaves its state column a don't-care — but every MEASURED arc comes only from a fully-initialised
//! (determinate) state, per the shared `Machine::arc_eligible` predicate. Arcs are derived by
//! exploring it:
//!
//!   1. Each state variable's δ comes directly from the cell's minimised signal functions; [`machine::settle`] applies them
//!      via [`Bdd::evaluate`](espresso_logic::bdd::Bdd::evaluate) until the state stops changing.
//!   2. BFS from the reachable stable states — which are not assumed but discovered by [`machine::explore`]
//!      from the on/off covers of the signal characteristic functions (never an assumed all-zero state) —
//!      stepping one input at a time and letting the state settle. Oscillating transitions (the state
//!      oscillates instead of settling — an oscillation hazard, e.g. a mutex at simultaneity) yield no
//!      fixpoint and are dropped, so no impossible arc is produced.
//!   3. Wherever a single input toggle flips an **output**, emit an arc: the toggled input is the
//!      `related` pin (arcs are only ever sourced by primary inputs — never an output or internal),
//!      and the prevector is the BFS path — each node projected onto the inputs — that drives every
//!      state variable (internal ones included) into the measured edge's start state.

use std::collections::HashSet;
use std::hash::Hash;

use espresso_logic::bdd::{Brand, ManagerCell};
use espresso_logic::{Minterm, Symbol};
use rayon::prelude::*;

use crate::logic::analysis::Machine;
use crate::logic::machine;
use crate::model::AnalysedOutput;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

/// One exposed internal node across a measured arc: the level it holds before the measured edge and the
/// level it holds after. Equal levels render the held `0`/`1`; a change renders `R`/`F`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExposedLevel {
    pub node: Symbol,
    pub start: bool,
    pub end: bool,
}

/// What the cell's non-input columns hold across a measured arc: each output pin's level at the arc's
/// start state, in `cell.outputs` order, and each exposed internal node's levels at both ends of the
/// measurement, in the machine's exposure order. Liberate reads the start levels as the arc's `-ic`
/// initial condition — the voltage each column starts the measured vector at — while an exposed node,
/// which the measurement observes as well as initialises, needs its end level too: it can move across an
/// arc whose outputs all hold.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ArcLevels {
    pub outputs: Vec<(Symbol, bool)>,
    pub exposed: Vec<ExposedLevel>,
}

impl ArcLevels {
    /// The levels every output and exposed node of `m`'s cell holds at `node`. A single-state sample, so
    /// every exposed level has `start == end`: the hazard probes in [`super::confluence::detect`]
    /// characterise one state, not a transition out of it.
    pub fn at<B: Brand, C: ManagerCell>(m: &Machine<B, C>, node: &Minterm<Symbol>) -> ArcLevels {
        Self::across(m, node, node)
    }

    /// The levels across a measured transition: every output at `start_node`, the state the measurement
    /// begins in, and every exposed node at both `start_node` and `settled`, the state the toggle settles
    /// into. An exposed internal genuinely transitions during an arc that leaves every output alone — a
    /// DFF's master follows D while Q holds — so its two ends are sampled separately.
    ///
    /// TOTAL, never partial: a level is read at a measurement site, and every measurement starts from a
    /// state `Machine::arc_eligible` admits — [`derive`] measures only eligible starts and
    /// [`super::confluence::detect`] probes only eligible states. At such a node every state column is
    /// determinate, and settling from it leaves every state column determinate, so both ends are total.
    /// A combinational output's or exposed node's δ is in turn bounded by the inputs plus the state
    /// variables (minimise invariant I3, asserted in `Machine::build` at `analysis.rs`). An exposed node
    /// is therefore either a state variable, which `arc_eligible` requires to be defined, or a
    /// combinational signal whose support lies within the inputs plus the state variables — resolved
    /// either way, as is every output.
    pub fn across<B: Brand, C: ManagerCell>(
        m: &Machine<B, C>,
        start_node: &Minterm<Symbol>,
        settled: &Minterm<Symbol>,
    ) -> ArcLevels {
        ArcLevels {
            outputs: m
                .cell
                .outputs
                .iter()
                .map(|o| {
                    let v = m
                        .output_value(&o.name, start_node)
                        .expect("every output is defined at a fully-initialised probed state");
                    (o.name.clone(), v)
                })
                .collect(),
            exposed: m
                .exposed
                .iter()
                .map(|node| {
                    let level = |n: &Minterm<Symbol>| {
                        m.exposed_value(node.as_str(), n).expect(
                            "every exposed node is defined at a fully-initialised probed state",
                        )
                    };
                    ExposedLevel {
                        node: node.clone(),
                        start: level(start_node),
                        end: level(settled),
                    }
                })
                .collect(),
        }
    }
}

/// One characterization arc: an input edge on `related` driving `output` in direction `edge`. The
/// related pin is **always a primary input** — outputs and internal state variables are never arc
/// sources; they are established indirectly by the prevector.
#[derive(Debug, Clone)]
pub struct Arc {
    pub edge: Edge,
    pub output: Symbol,
    pub related: Symbol,
    /// Start state of the measured edge (the prevector's target): the FULL machine node, over the
    /// input AND state-variable columns, not just the input projection. This is the arc's context:
    /// two firings that agree on the inputs but differ in internal state are different arcs, each
    /// with its own prevector, and both are emitted.
    pub start: Minterm<Symbol>,
    /// End state of the measured edge (defines the vector and the `-when` condition).
    pub end: Minterm<Symbol>,
    /// The prevector: the input-assignment sequence that drives every state variable into `start`.
    pub prevector: Vec<Minterm<Symbol>>,
    /// The levels the cell's outputs hold at `start` — the arc's `-ic` initial condition.
    pub levels: ArcLevels,
    pub is_async: bool,
}

/// A whole-cell internal-power ('hidden') arc: the input `pin` toggles between two settled
/// states and NO output changes. Used for internal-power characterisation.
#[derive(Debug, Clone)]
pub struct HiddenArc {
    pub pin: Symbol, // the toggled primary input
    pub edge: Edge,  // that input's Rise/Fall
    /// Start state of the measured toggle: the FULL machine node before it (inputs and state
    /// variables), the arc's context — see [`Arc::start`].
    pub start: Minterm<Symbol>,
    pub end: Minterm<Symbol>, // input vector after the toggle
    pub prevector: Vec<Minterm<Symbol>>,
    /// Each output's HELD logic value, in `cell.outputs` order. Every output is defined at both ends of
    /// the toggle and none of them changes across it, so the values held ARE the levels at `start`.
    pub levels: ArcLevels,
}

/// Derive transition arcs for every output of a cell by re-walking its shared asynchronous state machine
/// (see [`machine`] and [`Machine`]). A machine node is a [`Minterm<Symbol>`] over
/// `[inputs…, state_vars…]`; traversal states may be partial, but each arc is measured only from a
/// fully-initialised (determinate) state (see `Machine::arc_eligible`). Also derives the
/// whole-cell internal-power ('hidden') arcs — single input toggles that settle but leave every
/// output unchanged.
pub fn derive<B: Brand, C: ManagerCell + Send + Sync>(
    m: &Machine<B, C>,
) -> (Vec<Arc>, Vec<HiddenArc>) {
    let cell = m.cell;
    let inputs = &cell.inputs;
    let deltas = &m.deltas;
    let ex = &m.explored;

    let async_set: HashSet<&str> = cell.async_pins.iter().map(|s| s.as_str()).collect();
    // Arcs are identified by their FULL context: transition arcs by (output, related, edge-direction,
    // full machine start state), hidden ('hidden') arcs by (toggled pin, edge-direction, full machine
    // start state). Nothing merges — every context a firing can happen in emits its own arc with its
    // own prevector, so contexts differing only in internal state stay distinct: they exercise
    // different internal nodes, which is a different delay path and a different power measurement,
    // even where the input vectors and the held output values coincide.
    //
    // The identities are unique by construction: each reachable stable state appears once in
    // `ex.order` and contributes at most one toggle per input, hence at most one arc per output, so
    // two firings never share an identity. The `debug_assert!`s below read that back off the
    // assembled arcs.
    let (arcs, hidden) = ex
        .order
        .par_iter()
        .fold(
            || (Vec::new(), Vec::new()),
            |mut acc, node| {
                // ELIGIBILITY: only measure from a FULLY-DETERMINATE start — a partially-fixed start
                // carries an uninitialised latch that must not be read as a held value, so it seeds
                // traversal but is never an arc context (see `Machine::arc_eligible`).
                if !m.arc_eligible(node) {
                    return acc;
                }
                for related in inputs {
                    // Toggle one input, hold the (partial) state, and let the state settle.
                    let toggled = machine::toggle(node, &[related.as_str()]);
                    let Some(np) = machine::settle(deltas, &toggled) else {
                        continue;
                    };
                    // The levels of this toggle — the outputs at the start, the exposed nodes at both
                    // ends — sampled once and cloned per arc derived from it. Both paths below take the
                    // same sample: an exposed node can move across a toggle every output holds through.
                    let levels = ArcLevels::across(m, node, &np);
                    // An arc for every output that is defined at both ends and flips across this input toggle.
                    // The end is projected onto the inputs — it is what the `-vector` and `-when` render from —
                    // while the start keeps the full machine node, the arc's context.
                    let end = np.project_to(inputs);
                    let prevector = ex.path_to(node, inputs);
                    // Collect each output's (before, after) once so both the transition and hidden paths read it.
                    let vals: Vec<(&AnalysedOutput, Option<bool>, Option<bool>)> = cell
                        .outputs
                        .iter()
                        .map(|o| {
                            (
                                o,
                                m.output_value(&o.name, node),
                                m.output_value(&o.name, &np),
                            )
                        })
                        .collect();
                    for (o, before, after) in &vals {
                        let (Some(before), Some(after)) = (before, after) else {
                            continue;
                        };
                        if before == after {
                            continue;
                        }
                        let edge = if *after { Edge::Rise } else { Edge::Fall };
                        acc.0.push(Arc {
                            edge,
                            output: o.name.clone(),
                            related: related.clone(),
                            start: node.clone(),
                            end: end.clone(),
                            prevector: prevector.clone(),
                            levels: levels.clone(),
                            is_async: async_set.contains(related.as_str()),
                        });
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
                        acc.1.push(HiddenArc {
                            pin: related.clone(),
                            edge: if rose { Edge::Rise } else { Edge::Fall },
                            start: node.clone(),
                            end: end.clone(),
                            prevector: prevector.clone(),
                            levels: levels.clone(),
                        });
                    }
                }
                acc
            },
        )
        .reduce(
            || (Vec::new(), Vec::new()),
            |(mut a, mut h), (b, hb)| {
                a.extend(b);
                h.extend(hb);
                (a, h)
            },
        );
    debug_assert!(
        all_distinct(&arcs, |a| (&a.output, &a.related, a.edge, &a.start)),
        "arc identities are unique per firing"
    );
    debug_assert!(
        all_distinct(&hidden, |h| (&h.pin, h.edge, &h.start)),
        "hidden arc identities are unique per firing"
    );
    (arcs, hidden)
}

/// Whether every item carries a distinct identity under `key`, which may borrow from the item it keys.
/// The sole caller is a [`debug_assert!`] in [`derive`], so the set is built only where debug
/// assertions are enabled.
fn all_distinct<'a, T, K: Eq + Hash>(items: &'a [T], key: impl Fn(&'a T) -> K) -> bool {
    let mut seen = HashSet::with_capacity(items.len());
    items.iter().all(|item| seen.insert(key(item)))
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

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
        // Every arc's prevector is a real single-step walk into its start state — the prevector is
        // input-only, so it terminates at the start context projected onto the inputs.
        for a in &arcs {
            assert_eq!(
                a.prevector.last().unwrap(),
                &a.start.project_to(&cell.inputs)
            );
            for w in a.prevector.windows(2) {
                assert_eq!(w[0].hamming_distance(&w[1]), 1);
            }
        }
    }

    #[test]
    fn arc_levels_read_every_output_at_a_probed_state() {
        // Q is a state output (its level is its own state column) and QN a combinational one (its level
        // is its δ evaluated at the node): `at` must agree with `output_value` on both, at every
        // measurement-eligible state, and name the outputs in declaration order.
        let cell = analyse(
            r#"
[[cell]]
name = "C2N"
inputs = ["A", "B"]
[cell.outputs]
Q = "A*B + Q*(A+B)"
QN = "!Q"
"#,
        );
        let builder = espresso_logic::sync_bdd_builder!();
        let bdds = crate::model::build_signal_bdds(&cell, &builder);
        let m = crate::logic::analysis::Machine::build(
            &cell,
            &bdds,
            &crate::logic::machine::ExplorationBudget::default(),
        )
        .expect("fixture is explored");
        let names: Vec<&str> = cell.outputs.iter().map(|o| o.name.as_str()).collect();
        let mut eligible = 0;
        for node in m.explored.order.iter().filter(|n| m.arc_eligible(n)) {
            let levels = ArcLevels::at(&m, node);
            assert_eq!(
                levels
                    .outputs
                    .iter()
                    .map(|(n, _)| n.as_str())
                    .collect::<Vec<_>>(),
                names
            );
            for (name, value) in &levels.outputs {
                assert_eq!(Some(*value), m.output_value(name, node));
            }
            eligible += 1;
        }
        assert!(eligible > 0, "the fixture has measurement-eligible states");
    }

    #[test]
    fn exposed_master_moves_across_a_hidden_arc() {
        // Master-slave DFF with the master M exposed. With the clock low the master is transparent, so a
        // D toggle moves M while Q holds — a hidden (internal-power) arc, which by construction is one
        // where every output is unchanged — and the exposed levels must record that move at both ends.
        let cell = analyse(
            r#"
[[cell]]
name = "DFF"
inputs = ["CLK", "D"]
clock = ["CLK"]
expose = ["M"]
[cell.internal]
M = "!CLK*D + CLK*M"
[cell.outputs]
Q = "CLK*M + !CLK*Q"
"#,
        );
        assert_eq!(
            cell.exposed_signals().collect::<Vec<_>>(),
            [&Symbol::from("M")],
            "the master survives the minimisation as an exposed node",
        );
        // Every measured arc, transition or hidden, carries one level per exposed node.
        for levels in cell
            .arcs
            .iter()
            .map(|a| &a.levels)
            .chain(cell.hidden_arcs.iter().map(|h| &h.levels))
        {
            assert_eq!(levels.exposed.len(), 1);
            assert_eq!(levels.exposed[0].node, "M");
        }
        let moved: Vec<&HiddenArc> = cell
            .hidden_arcs
            .iter()
            .filter(|h| h.levels.exposed.iter().any(|e| e.start != e.end))
            .collect();
        assert!(
            moved.iter().any(|h| h.pin == "D"),
            "a D toggle with the clock low moves the exposed master while Q holds",
        );
    }

    #[test]
    fn combinational_exposure_is_sampled_through_its_delta() {
        // An exposed node that is NOT a state variable — the combinational internal W = A*B — has its
        // level read from its δ, as a combinational output does. The minimisation runs here with W
        // preserved (`Preserved::with_exposed`), which is what keeps a combinational exposure in the
        // model at all; the fold composes it into Y regardless, so W keeps a δ over the inputs alone.
        let mut cell = crate::model::parse_spec(
            r#"
[[cell]]
name = "AOI21"
inputs = ["A", "B", "C"]
expose = ["W"]
[cell.internal]
W = "A*B"
[cell.outputs]
Y = "!(W + C)"
"#,
        )
        .unwrap()
        .cells
        .remove(0)
        .analyse_signals()
        .unwrap();
        let builder = espresso_logic::sync_bdd_builder!();
        let mut bdds = crate::model::build_signal_bdds(&cell, &builder);
        let order: Vec<Symbol> = cell.signals().map(|s| s.name.clone()).collect();
        let preserved = crate::logic::minimise::Preserved::with_exposed(
            cell.outputs.iter().map(|o| o.name.clone()).collect(),
            cell.exposed.iter().cloned().collect(),
        );
        let min = crate::logic::minimise::minimise_state_space(&mut bdds, &order, &preserved);
        crate::model::recompute_signal_metadata(&mut cell, &bdds, &min);
        let m = crate::logic::analysis::Machine::build(
            &cell,
            &bdds,
            &crate::logic::machine::ExplorationBudget::default(),
        )
        .expect("fixture is explored");
        assert_eq!(m.exposed, ["W"]);
        assert!(
            !m.state_set.contains(&Symbol::from("W")) && m.exposed_deltas.contains_key("W"),
            "a combinational exposure is read from a δ, not from a state column",
        );

        // At each explored state the sampled level is A*B there, and a single-state sample leaves the
        // two ends equal — the shape the constraint sampling site reads.
        let mut sampled = 0;
        for node in &m.explored.order {
            let levels = ArcLevels::at(&m, node);
            let of = |pin: &str| {
                node.value_of(pin)
                    .expect("every input is fixed in an explored state")
            };
            assert_eq!(levels.exposed[0].start, of("A") && of("B"));
            assert_eq!(levels.exposed[0].start, levels.exposed[0].end);
            sampled += 1;
        }
        assert!(sampled > 0, "the fixture explores at least one state");

        // Across a measured arc the two ends differ wherever the toggled input moves W.
        let (arcs, _) = derive(&m);
        assert!(
            arcs.iter()
                .any(|a| a.levels.exposed[0].start != a.levels.exposed[0].end),
            "an A or B edge that flips Y moves the exposed W with it",
        );
    }

    #[test]
    fn c2_arc_levels_hold_the_pre_edge_output_value() {
        // An arc's levels are read at its START state, so the measured output sits at the value the edge
        // leaves: low before a rise, high before a fall.
        let cell = analyse(
            r#"
[[cell]]
name = "C2"
inputs = ["A", "B"]
[cell.outputs]
Q = "A*B + Q*(A+B)"
"#,
        );
        assert!(!cell.arcs.is_empty());
        for a in &cell.arcs {
            let (_, level) = a
                .levels
                .outputs
                .iter()
                .find(|(n, _)| *n == a.output)
                .expect("the measured output carries a level");
            assert_eq!(
                *level,
                a.edge == Edge::Fall,
                "{} {:?} must start at its pre-edge value",
                a.output,
                a.edge
            );
        }
    }

    #[test]
    fn c2_arc_and_hidden_prevector_walk_depths() {
        // multiset of prevector lengths, one entry per derived arc — pins the walk depth each context
        // costs. C2's only state variable is the output itself, so no two contexts share an identity:
        // the counts are the same ones full-context keying yields. Re-capture only for a deliberate
        // algorithm change.
        let cell = analyse(
            r#"
[[cell]]
name = "C2"
inputs = ["A", "B"]
[cell.outputs]
Q = "A*B + Q*(A+B)"
"#,
        );
        let mut arc_lens: Vec<usize> = cell.arcs.iter().map(|a| a.prevector.len()).collect();
        arc_lens.sort();
        assert_eq!(arc_lens, vec![2, 2, 2, 2]);
        let mut hidden_lens: Vec<usize> =
            cell.hidden_arcs.iter().map(|h| h.prevector.len()).collect();
        hidden_lens.sort();
        assert_eq!(hidden_lens, vec![1, 1, 1, 1, 2, 2, 2, 2]);
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
                && h.levels.outputs.len() == 1
                && h.levels.outputs[0].0.as_str() == "Y"
                && !h.levels.outputs[0].1
        }));
        // Single-output cell: every hidden arc holds exactly one output value.
        assert!(cell.hidden_arcs.iter().all(|h| h.levels.outputs.len() == 1));
        // Every hidden arc's prevector is a real single-step walk into its start state, projected onto
        // the inputs.
        for h in &cell.hidden_arcs {
            assert_eq!(h.prevector.last(), Some(&h.start.project_to(&cell.inputs)));
            for w in h.prevector.windows(2) {
                assert_eq!(w[0].hamming_distance(&w[1]), 1);
            }
        }
    }

    #[test]
    fn dlatch_keeps_both_stored_value_hidden_contexts() {
        // Transparent-high D-latch: in hold (E=0) a D toggle leaves Q unchanged but its held value depends
        // on the stored state. The two stored-value contexts (Q held 0 and Q held 1) are different machine
        // states, hence distinct hidden arcs on D, each carrying the held value it measured.
        let cell = analyse(
            r#"
[[cell]]
name = "DLAT"
inputs = ["E", "D"]
[cell.outputs]
Q = "E*D + !E*Q"
"#,
        );
        let d_rise: Vec<&HiddenArc> = cell
            .hidden_arcs
            .iter()
            .filter(|h| h.pin.as_str() == "D" && h.edge == Edge::Rise)
            .collect();
        assert!(
            d_rise.len() >= 2,
            "expected >=2 D-rise hidden arcs, got {}",
            d_rise.len()
        );
        let q_val = |h: &HiddenArc| {
            h.levels
                .outputs
                .iter()
                .find(|(s, _)| s.as_str() == "Q")
                .map(|(_, v)| *v)
        };
        assert!(d_rise.iter().any(|h| q_val(h) == Some(false)));
        assert!(d_rise.iter().any(|h| q_val(h) == Some(true)));
    }

    /// Two latches, one of them masked out of the output: `K` drives `Y`, while `L` reaches it only
    /// through `S`. At `S=0` the two stored values of `L` are indistinguishable at the pins, so the
    /// same firing happens in two machine contexts that share every input value and every output
    /// value — the minimal shape of the interlocked cells where the arc growth lands.
    const MASKED_PAIR: &str = r#"
[[cell]]
name = "MASKPAIR"
inputs = ["E", "D", "S", "C"]
[cell.internal]
L = "E*D + !E*L"
K = "C*D + !C*K"
[cell.outputs]
Y = "K + S*L"
"#;

    /// The `L` values the arcs in `starts` were measured from, for arcs whose start context agrees
    /// with `at` on every listed pin.
    fn masked_values<'a>(
        starts: impl Iterator<Item = &'a Minterm<Symbol>>,
        at: &[(&str, bool)],
    ) -> BTreeSet<bool> {
        use crate::logic::assignment;
        starts
            .map(assignment)
            .filter(|a| {
                at.iter()
                    .all(|(pin, v)| a.iter().any(|(s, b)| s == pin && b == v))
            })
            .filter_map(|a| a.iter().find(|(s, _)| s.as_str() == "L").map(|(_, b)| *b))
            .collect()
    }

    #[test]
    fn distinct_internal_contexts_split_a_delay_arc() {
        // C rises with D=1 while K holds 0 and S=0: Y rises through K in both stored contexts of the
        // masked latch. Same related pin, same direction, same input vectors at both ends — two arcs,
        // because the internal node the edge travels through is not the same one.
        let cell = analyse(MASKED_PAIR);
        let at = [("E", false), ("D", true), ("S", false), ("C", false)];
        let contexts = masked_values(
            cell.arcs
                .iter()
                .filter(|a| a.output == "Y" && a.related == "C" && a.edge == Edge::Rise)
                .map(|a| &a.start),
            &at,
        );
        assert_eq!(
            contexts,
            BTreeSet::from([false, true]),
            "both stored contexts of the masked latch must emit their own C→Y rise arc"
        );
    }

    #[test]
    fn distinct_internal_contexts_split_a_hidden_arc() {
        // D falls in hold (E=0, C=0) with S=0: nothing moves and Y stays 0 in either stored context of
        // the masked latch, yet the toggle exercises different internal state — two power measurements,
        // so two hidden arcs, though their input vectors and held output values coincide.
        let cell = analyse(MASKED_PAIR);
        let at = [("E", false), ("D", true), ("S", false), ("C", false)];
        let held_low = |h: &HiddenArc| h.levels.outputs.iter().all(|(o, v)| o == "Y" && !v);
        let contexts = masked_values(
            cell.hidden_arcs
                .iter()
                .filter(|h| h.pin == "D" && h.edge == Edge::Fall && held_low(h))
                .map(|h| &h.start),
            &at,
        );
        assert_eq!(
            contexts,
            BTreeSet::from([false, true]),
            "both stored contexts of the masked latch must emit their own D-fall hidden arc"
        );
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
        // The prevector is a real single-step input walk terminating at the measured start state's
        // input projection — the state variables it establishes are not part of the walk's alphabet.
        assert_eq!(
            clk_rise.prevector.last().unwrap(),
            &clk_rise.start.project_to(&cell.inputs)
        );
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
