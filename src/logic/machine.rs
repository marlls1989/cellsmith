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

use std::collections::{HashMap, HashSet, VecDeque};
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

/// The reachable **stable** states of a cell's state machine, in the order [`explore`] discovered them,
/// with a predecessor map for prevector reconstruction.
pub struct Explored {
    /// Reachable stable nodes in BFS dequeue order (each appears once).
    pub order: Vec<Minterm<Symbol>>,
    /// Predecessor of each reachable node (`None` at a start node).
    pub prev: HashMap<Minterm<Symbol>, Option<Minterm<Symbol>>>,
}

/// Explore the reachable **stable** states of the machine. Start from the reset-stable states (stable
/// under the all-zero input, falling back to every stable node if none), then BFS: from each node toggle
/// one input at a time, hold the state, and [`settle`]. Metastable toggles (no fixpoint) are dropped.
///
/// Shared by [`super::arcs`] (which re-walks `order`, re-toggling to measure output edges) and
/// [`super::confluence`] (which re-walks `order`, testing pairwise input-order confluence). `input_names`
/// and `state_names` index into the `full_header` node fields.
pub fn explore<B: Brand, C: ManagerCell>(
    deltas: &[Delta<B, C>],
    full_header: &Arc<Symbols<Symbol>>,
    input_names: &[String],
    state_names: &[String],
) -> Explored {
    let n = input_names.len();
    let k = state_names.len();

    // A node from an input assignment `x` (bit i = input_names[i]) and a state assignment `s`.
    let bit = |mask: usize, list: &[String], name: &str| -> Option<bool> {
        list.iter()
            .position(|v| v == name)
            .map(|i| (mask >> i) & 1 == 1)
    };
    let make_node = |x: usize, s: usize| -> Minterm<Symbol> {
        node_from(full_header, |name| {
            bit(x, input_names, name)
                .or_else(|| bit(s, state_names, name))
                .expect("every header variable is an input or a state variable")
        })
    };

    let n_st = 1usize << k;
    // Reset-stable states: state stable under the all-zero input. Fall back to every stable node if the
    // all-zero input has no stable state.
    let mut starts: Vec<Minterm<Symbol>> = (0..n_st)
        .map(|s| make_node(0, s))
        .filter(|node| is_stable(deltas, node))
        .collect();
    if starts.is_empty() {
        starts = (0..(1usize << n))
            .flat_map(|x| (0..n_st).map(move |s| (x, s)))
            .map(|(x, s)| make_node(x, s))
            .filter(|node| is_stable(deltas, node))
            .collect();
    }

    let mut prev: HashMap<Minterm<Symbol>, Option<Minterm<Symbol>>> = HashMap::new();
    let mut queue: VecDeque<Minterm<Symbol>> = VecDeque::new();
    for st in &starts {
        prev.entry(st.clone()).or_insert(None);
    }
    queue.extend(starts.iter().cloned());

    let mut order: Vec<Minterm<Symbol>> = Vec::new();
    while let Some(node) = queue.pop_front() {
        order.push(node.clone());
        for related in input_names {
            let toggled = node_from(full_header, |name| {
                let cur = node
                    .value_of(name)
                    .expect("a header variable is fixed in the node");
                if name == related.as_str() {
                    !cur
                } else {
                    cur
                }
            });
            let Some(np) = settle(deltas, full_header, &toggled) else {
                continue;
            };
            if let std::collections::hash_map::Entry::Vacant(e) = prev.entry(np.clone()) {
                e.insert(Some(node.clone()));
                queue.push_back(np);
            }
        }
    }

    Explored { order, prev }
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
