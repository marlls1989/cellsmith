//! Dependency-graph resolution of a cell's signals.
//!
//! A cell is an asynchronous state machine over `inputs × state-variables`. Every signal (an external
//! **output** or an **internal** variable) is a Boolean function that may reference primary inputs and
//! other signals (delayed/feedback values). To characterise one signal we first **resolve** it: compose
//! the referenced signals into it so the result is a function of primary inputs plus the *residual*
//! **state variables** only.
//!
//! The one rule: **substitute each signal at most once.** A signal whose name *reappears* after it was
//! already substituted — or the target itself, when self-referential — is a genuine state variable that
//! does not resolve, and is left in the result. Substituting a cross-coupled peer **once** is what surfaces
//! a signal's dependence on a seemingly-unrelated input (substituting `Qb = !Qa·B` into `Qa = !Qb·A` is how
//! `B` appears in `Qa` at all, giving the `B↓ → Qa↑` cascade).
//!
//! Substitution order matters only for the **combinational** (acyclic) part: a dependee must never be
//! substituted before a depender whose definition would reintroduce it (that would strand the dependee as
//! a spurious residual). Reverse-post-order DFS from the target (a signal before every signal it
//! references) is the topological order that avoids this; cycles are broken by the visited guard, and the
//! residual is exactly the state variable. Composition uses the BDD layer's native [`Bdd::compose`].

use std::collections::{BTreeMap, BTreeSet};

use espresso_logic::bdd::{Bdd, Brand, ManagerCell};

use crate::model::AnalysedOutput;

/// `signal name → the signal names its function references` (its feedback/state, self-reference
/// included). Non-signal variables (primary inputs) are not edges.
pub fn dependency_map(signals: &[&AnalysedOutput]) -> BTreeMap<String, Vec<String>> {
    signals
        .iter()
        .map(|s| (s.name.clone(), s.feedback.clone()))
        .collect()
}

/// Reverse-post-order DFS from `target` over the reference graph: a signal appears **before** every
/// signal it (transitively) references. Substituting in this order keeps the acyclic part confluent — a
/// dependee is never substituted before a depender that would reintroduce it. Cycles are broken by the
/// visited guard (the residual is the state variable).
pub fn substitution_order(target: &str, deps: &BTreeMap<String, Vec<String>>) -> Vec<String> {
    let mut post = Vec::new();
    let mut seen = BTreeSet::new();
    dfs_post(target, deps, &mut seen, &mut post);
    post.reverse();
    post
}

fn dfs_post(
    node: &str,
    deps: &BTreeMap<String, Vec<String>>,
    seen: &mut BTreeSet<String>,
    post: &mut Vec<String>,
) {
    if !seen.insert(node.to_string()) {
        return;
    }
    if let Some(children) = deps.get(node) {
        for c in children {
            if c != node {
                dfs_post(c, deps, seen, post);
            }
        }
    }
    post.push(node.to_string());
}

/// Resolve `target` into a BDD over primary inputs plus its residual **state variables**.
///
/// `bdds` maps every signal name to its own (un-substituted) BDD, all built in the same builder.
/// `order` is a [`substitution_order`] for `target`. Each signal named in `order` that is still present
/// in the working function is composed in **once** via [`Bdd::compose`] (`f[v:=g]`); a signal that
/// reappears after being substituted (or `target` itself) is a state variable and is left in the result.
///
/// # Panics
///
/// Panics if `bdds` has no entry for `target` — the caller must ensure `target` names a signal already
/// present in `bdds`.
pub fn resolve<B: Brand, C: ManagerCell>(
    target: &str,
    bdds: &BTreeMap<String, Bdd<B, C>>,
    order: &[String],
) -> Bdd<B, C> {
    let mut f = bdds
        .get(target)
        .unwrap_or_else(|| panic!("resolve: target signal {target:?} absent from bdds map"))
        .clone();
    let mut visited: BTreeSet<&str> = BTreeSet::new();
    visited.insert(target);
    loop {
        // The earliest signal in dependency order that is still present in `f` and not yet substituted.
        let next = order.iter().find(|name| {
            !visited.contains(name.as_str())
                && bdds.contains_key(name.as_str())
                && f.variables().any(|v| v.as_str() == name.as_str())
        });
        let Some(name) = next else { break };
        // Cannot panic: `next` (above) only yields names for which `bdds.contains_key` just held.
        f = f.compose(name.as_str(), &bdds[name]); // f[name := g]
        visited.insert(name.as_str());
    }
    f
}

/// The ≥1-step reachability relation of a directed graph: `node → the nodes reachable from it in one
/// or more edges`. Computed by relaxation (the graphs are tiny). Used by [`state_variables`] to find
/// the signals that reach themselves (the state variables).
fn transitive_closure(edges: &BTreeMap<String, Vec<String>>) -> BTreeMap<String, BTreeSet<String>> {
    let mut reach: BTreeMap<String, BTreeSet<String>> = edges
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

/// The **state variables** of a cell: signals that lie on a dependency cycle — a self-reference or a
/// larger coupling cycle. A signal on no cycle is combinational and resolves away entirely; a state
/// variable is a held coordinate of the cell's state machine. A signal `s` is a state variable iff `s`
/// reaches itself in the reference graph.
pub fn state_variables(signals: &[&AnalysedOutput]) -> BTreeSet<String> {
    let reach = transitive_closure(&dependency_map(signals));
    signals
        .iter()
        .map(|s| s.name.clone())
        .filter(|n| reach.get(n).is_some_and(|r| r.contains(n)))
        .collect()
}

/// The one-step **next-state** function of `target` over primary inputs plus the cell's state
/// variables: [`resolve`] substituting **only combinational** signals (those not in `state_vars`), so
/// every state variable stays as a current-state coordinate rather than being folded away. This is the
/// state machine's transition function δ, distinct from [`resolve`] (which composes state peers too, to
/// expose cascades in the region view).
pub fn delta<B: Brand, C: ManagerCell>(
    target: &str,
    bdds: &BTreeMap<String, Bdd<B, C>>,
    deps: &BTreeMap<String, Vec<String>>,
    state_vars: &BTreeSet<String>,
) -> Bdd<B, C> {
    let order: Vec<String> = substitution_order(target, deps)
        .into_iter()
        .filter(|n| !state_vars.contains(n))
        .collect();
    resolve(target, bdds, &order)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{analyse_one as analyse, AnalysedCell, AnalysedOutput};
    use espresso_logic::bdd_builder;

    /// Whether the resolved function `f` still mentions any of the cell's signal names.
    fn has_signal_residual(
        f: &espresso_logic::bdd::Bdd<impl Brand, impl ManagerCell>,
        cell: &AnalysedCell,
    ) -> bool {
        let names: BTreeSet<String> = cell.signals().map(|s| s.name.clone()).collect();
        f.variables().any(|v| names.contains(v.as_str()))
    }

    #[test]
    fn multilevel_combinational_fully_resolves() {
        // Y → W1 → W2 → A: three levels, each substituted once (multiple iterations), no residual signal.
        let cell = analyse(
            r#"
[[cell]]
name = "CHAIN"
inputs = ["A"]
[cell.internal]
W1 = "W2"
W2 = "A"
[cell.outputs]
Y = "W1"
"#,
        );
        let sigs: Vec<&AnalysedOutput> = cell.signals().collect();
        let deps = dependency_map(&sigs);
        let builder = bdd_builder!();
        let bdds: BTreeMap<String, _> = sigs
            .iter()
            .map(|s| (s.name.clone(), builder.build(&s.expr)))
            .collect();

        let f = resolve("Y", &bdds, &substitution_order("Y", &deps));
        assert!(f.equivalent_to(&builder.parse("A").unwrap()));
        assert!(!has_signal_residual(&f, &cell));
    }

    #[test]
    fn state_variables_are_the_cyclic_signals() {
        // DFF: M (self-holding) and Q (self-holding) are state; a plain combinational internal is not.
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
    }

    #[test]
    fn delta_keeps_state_peers_as_current_state() {
        // Plain mutex: δ_Qa keeps Qb (its current state), unlike `resolve` which folds Qb away.
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
        let sigs: Vec<&AnalysedOutput> = cell.signals().collect();
        let deps = dependency_map(&sigs);
        let sv = state_variables(&sigs);
        let builder = bdd_builder!();
        let bdds: BTreeMap<String, _> = sigs
            .iter()
            .map(|s| (s.name.clone(), builder.build(&s.expr)))
            .collect();
        let d = delta("Qa", &bdds, &deps, &sv);
        // δ_Qa = !Qb·A — Qb retained as current state.
        assert!(d.equivalent_to(&builder.parse("!Qb*A").unwrap()));
        assert!(d.variables().any(|v| v.as_str() == "Qb"));
    }

    #[test]
    fn confluent_despite_shared_dependee() {
        // T = P·Q, P = Q, Q = A (all combinational). If Q were substituted before P, P's definition would
        // reintroduce the already-visited Q and strand it as a residual. Reverse-post-order forbids that:
        // the result is A with no residual, regardless.
        let cell = analyse(
            r#"
[[cell]]
name = "SHARE"
inputs = ["A"]
[cell.internal]
P = "Q"
Q = "A"
[cell.outputs]
T = "P*Q"
"#,
        );
        let sigs: Vec<&AnalysedOutput> = cell.signals().collect();
        let deps = dependency_map(&sigs);
        let builder = bdd_builder!();
        let bdds: BTreeMap<String, _> = sigs
            .iter()
            .map(|s| (s.name.clone(), builder.build(&s.expr)))
            .collect();

        let f = resolve("T", &bdds, &substitution_order("T", &deps));
        assert!(f.equivalent_to(&builder.parse("A").unwrap()));
        assert!(!has_signal_residual(&f, &cell));
    }
}
