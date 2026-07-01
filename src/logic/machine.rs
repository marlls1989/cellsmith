//! The cell's asynchronous state machine, expressed natively over espresso-logic minterms.
//!
//! A cell is a state machine over `inputs × state-variables`. A **node** is a fully-fixed
//! [`Minterm<Symbol>`] over the shared `[inputs…, state_vars…]` header: every input and every state
//! variable carries a concrete value. The next-state map settles the state fields (via each state
//! variable's δ, [`super::resolve::delta`]) while holding the inputs fixed; a node is *stable* when it
//! is its own next-state.
//!
//! Everything here is a thin wrapper over the crate primitives — [`Bdd::evaluate`] to read a δ at a
//! node (a complete assignment yields `Ok(bool)`), [`Minterm::from_symbols`] to build a node, and
//! [`Symbols`] for the shared header — rather than a hand-rolled restrict loop over an integer state
//! bitmask. The functions are generic over the BDD brand, mirroring [`super::resolve`].

use std::collections::HashSet;
use std::sync::Arc;

use espresso_logic::bdd::{Bdd, Brand, ManagerCell};
use espresso_logic::{Minterm, Symbol, Symbols};

/// A state variable paired with its next-state function δ (over inputs + state variables).
pub type Delta<B, C> = (String, Bdd<B, C>);

/// A shared symbol header from an ordered list of variable names.
pub fn header(names: &[String]) -> Arc<Symbols<Symbol>> {
    Symbols::new(names.iter().map(|n| Symbol::from(n.as_str())).collect())
}

/// Build a fully-fixed node over `header` from a `name -> value` lookup (called once per variable).
pub fn node_from<F: Fn(&str) -> bool>(header: &Arc<Symbols<Symbol>>, value: F) -> Minterm<Symbol> {
    Minterm::from_symbols(
        header.clone(),
        header.labels().iter().map(|l| Some(value(l.as_str()))),
    )
}

/// One parallel next-state step: every state variable takes its δ evaluated at `node`; the inputs (and
/// anything else in the header) keep their current value.
fn step<B: Brand, C: ManagerCell>(
    deltas: &[Delta<B, C>],
    header: &Arc<Symbols<Symbol>>,
    node: &Minterm<Symbol>,
) -> Minterm<Symbol> {
    let next: Vec<(&str, bool)> = deltas
        .iter()
        .map(|(name, d)| {
            let v = d
                .evaluate(node)
                .expect("a complete (input+state) assignment determines δ");
            (name.as_str(), v)
        })
        .collect();
    node_from(header, |name| {
        next.iter()
            .find(|(n, _)| *n == name)
            .map(|(_, v)| *v)
            .unwrap_or_else(|| {
                node.value_of(name)
                    .expect("a header variable is fixed in the node")
            })
    })
}

/// Whether `node` is stable: every state variable's δ already equals its current value.
pub fn is_stable<B: Brand, C: ManagerCell>(deltas: &[Delta<B, C>], node: &Minterm<Symbol>) -> bool {
    deltas.iter().all(|(name, d)| {
        d.evaluate(node)
            .expect("a complete (input+state) assignment determines δ")
            == node
                .value_of(name.as_str())
                .expect("a state variable is fixed in the node")
    })
}

/// Settle the state under `node`'s fixed inputs: iterate [`step`] to a fixpoint. Returns `None` if the
/// state oscillates without settling (a metastable / arbitration condition).
pub fn settle<B: Brand, C: ManagerCell>(
    deltas: &[Delta<B, C>],
    header: &Arc<Symbols<Symbol>>,
    node: &Minterm<Symbol>,
) -> Option<Minterm<Symbol>> {
    let mut cur = node.clone();
    let mut seen: HashSet<Minterm<Symbol>> = HashSet::new();
    loop {
        let next = step(deltas, header, &cur);
        if next == cur {
            return Some(cur); // fixpoint
        }
        if !seen.insert(next.clone()) {
            return None; // revisited a non-fixpoint state → oscillation
        }
        cur = next;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use espresso_logic::bdd_builder;

    #[test]
    fn settles_a_c_element_hold() {
        // Q = A*B + Q*(A+B). Over header [A, B, Q], hold state 01/10 keeps Q; 11 forces Q high.
        let builder = bdd_builder!();
        let dq = builder.parse("A*B + Q*(A+B)").unwrap();
        let deltas = vec![("Q".to_string(), dq)];
        let hdr = header(&["A".into(), "B".into(), "Q".into()]);

        // A=1 B=0 Q=1 is a stable hold state.
        let hold = node_from(&hdr, |n| matches!(n, "A" | "Q"));
        assert!(is_stable(&deltas, &hold));
        assert_eq!(settle(&deltas, &hdr, &hold).as_ref(), Some(&hold));

        // A=1 B=1 Q=0 is not stable; it settles to Q=1.
        let forcing = node_from(&hdr, |n| matches!(n, "A" | "B"));
        assert!(!is_stable(&deltas, &forcing));
        let settled = settle(&deltas, &hdr, &forcing).expect("settles");
        assert_eq!(settled.value_of("Q"), Some(true));
    }

    #[test]
    fn metastable_mutex_oscillates_to_none() {
        // Cross-coupled: Qa = !Qb*A, Qb = !Qa*B. Under A=B=1 the joint next-state of {Qa=0,Qb=0}
        // toggles both to 1 then back — no fixpoint reachable from it, so settle yields None.
        let builder = bdd_builder!();
        let da = builder.parse("!Qb*A").unwrap();
        let db = builder.parse("!Qa*B").unwrap();
        let deltas = vec![("Qa".to_string(), da), ("Qb".to_string(), db)];
        let hdr = header(&["A".into(), "B".into(), "Qa".into(), "Qb".into()]);
        let both_low = node_from(&hdr, |n| matches!(n, "A" | "B"));
        assert_eq!(settle(&deltas, &hdr, &both_low), None);
    }
}
