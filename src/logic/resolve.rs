//! Reference-graph analysis of a cell's signals.
//!
//! A cell is an asynchronous state machine over `inputs × state-variables`. Every signal (an external
//! **output** or an **internal** variable) is a Boolean function that may reference primary inputs and
//! other signals (delayed/feedback values). This module provides only the reference graph
//! ([`dependency_map`], [`transitive_closure`]) and the **state-variable** classifier
//! ([`state_variables`]) over the already-**minimised** model — a signal on a dependency cycle
//! (self-reference or a larger coupling cycle) is a held coordinate of the state machine, and one on
//! no cycle is purely combinational.
//!
//! Substitution itself happens once, up front, in [`super::minimise`]: each signal's function is folded
//! down to a fixpoint over the minimal residual set before this module ever sees it. δ (a state
//! variable's next-state function) is then a direct lookup in the shared BDD map — there is no
//! resolve/substitution step left to perform here.

use std::collections::{BTreeMap, BTreeSet};

use espresso_logic::Symbol;

use crate::model::AnalysedOutput;

/// `signal name → the signal names its function references` (its feedback/state, self-reference
/// included). Non-signal variables (primary inputs) are not edges.
pub fn dependency_map(signals: &[&AnalysedOutput]) -> BTreeMap<Symbol, Vec<Symbol>> {
    signals
        .iter()
        .map(|s| (s.name.clone(), s.feedback.clone()))
        .collect()
}

/// The ≥1-step reachability relation of a directed graph: `node → the nodes reachable from it in one
/// or more edges`. Computed by relaxation (the graphs are tiny). Used by [`state_variables`] to find
/// the signals that reach themselves (the state variables).
fn transitive_closure(edges: &BTreeMap<Symbol, Vec<Symbol>>) -> BTreeMap<Symbol, BTreeSet<Symbol>> {
    let mut reach: BTreeMap<Symbol, BTreeSet<Symbol>> = edges
        .iter()
        .map(|(k, vs)| (k.clone(), vs.iter().cloned().collect()))
        .collect();
    loop {
        let mut changed = false;
        for u in reach.keys().cloned().collect::<Vec<_>>() {
            for v in reach[&u].iter().cloned().collect::<Vec<_>>() {
                for w in reach.get(&v).cloned().unwrap_or_default() {
                    if reach.get_mut(&u).unwrap().insert(w) {
                        changed = true;
                    }
                }
            }
        }
        if !changed {
            break;
        }
    }
    reach
}

/// The nodes of a directed graph that reach **themselves** through one or more edges — the signals on
/// a dependency cycle (a self-reference or a larger coupling cycle). This is the pure graph kernel of
/// [`state_variables`]; [`super::minimise`]'s hoist pass reuses it over the folded BDD support graph to
/// find the cyclic (state-variable) outputs.
pub(crate) fn self_reaching(edges: &BTreeMap<Symbol, Vec<Symbol>>) -> BTreeSet<Symbol> {
    let reach = transitive_closure(edges);
    edges
        .keys()
        .filter(|n| reach.get(*n).is_some_and(|r| r.contains(*n)))
        .cloned()
        .collect()
}

/// The **state variables** of a cell: signals that lie on a dependency cycle — a self-reference or a
/// larger coupling cycle. A signal on no cycle is combinational and resolves away entirely; a state
/// variable is a held coordinate of the cell's state machine. A signal `s` is a state variable iff `s`
/// reaches itself in the reference graph.
pub fn state_variables(signals: &[&AnalysedOutput]) -> BTreeSet<Symbol> {
    self_reaching(&dependency_map(signals))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{analyse_one as analyse, AnalysedOutput};

    #[test]
    fn state_variables_are_the_cyclic_signals() {
        // DFF: M (self-holding) and Q (self-holding) are state; a plain combinational internal is not.
        // The combinational internal W ("CLK*D", unconsumed) is purged by the minimisation before
        // classification ever sees it.
        let cell = analyse(
            r#"
[[cell]]
name = "DFF"
inputs = ["CLK", "D"]
[cell.internal]
M = "!CLK*D + CLK*M"
W = "CLK*D"
[cell.outputs]
Q = "CLK*M + !CLK*Q"
"#,
        );
        let sigs: Vec<&AnalysedOutput> = cell.signals().collect();
        let sv = state_variables(&sigs);
        assert!(sv.contains("M"));
        assert!(sv.contains("Q"));
        assert!(!sv.contains("W")); // combinational internal
        assert!(!cell.internals.iter().any(|s| s.name == "W"));
    }
}
