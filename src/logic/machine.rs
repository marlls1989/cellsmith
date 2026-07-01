//! The cell's asynchronous state machine, expressed natively over espresso-logic minterms.
//!
//! A cell is a state machine over `inputs × state-variables`. A **node** is a [`Minterm<Symbol>`] over
//! the shared `[inputs…, state_vars…]` header in which every input carries a concrete value and each
//! state variable is either **defined** (a concrete value) or **absent** — an unresolved state variable
//! is simply not fixed in the minterm, never a placeholder value. Power-on is the inputs-only node: no
//! state fixed. The next-state map settles the state fields (via each state variable's δ,
//! [`super::resolve::delta`]) using [`Bdd::evaluate`], which reads a δ under the node's fixed fields and
//! returns `Ok(v)` only when they force it — an absent state variable stays absent (it provably does not
//! influence that δ yet). A node is *stable* when it is its own next-state.
//!
//! Start states are not assumed: [`explore`] discovers them from the on/off covers of the signal
//! functions over the cell inputs ([`Bdd::maximize`]), so a state-holding cell whose state is undefined
//! at the all-zero input (its reset is an input sequence, not a level) is initialised by the sequence
//! that actually resolves it — the async pins, a clock edge, both requests high — rather than by an
//! arbitrary held combination.

use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};
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

/// Build a node over `header` from a `name -> Option<value>` lookup: `None` leaves that variable
/// **absent** (unresolved), `Some(v)` fixes it. This is the general node constructor — inputs are
/// always `Some`, state variables may be either.
pub fn node_from_opt<F: Fn(&str) -> Option<bool>>(
    header: &Arc<Symbols<Symbol>>,
    value: F,
) -> Minterm<Symbol> {
    Minterm::from_symbols(
        header.clone(),
        header.labels().iter().map(|l| value(l.as_str())),
    )
}

/// One parallel next-state step: every state variable takes its δ evaluated at `node` — `Ok(v)` fixes
/// it, `Err` (δ still depends on an absent variable) leaves it **absent**. Inputs (and anything else in
/// the header) keep their current field.
fn step<B: Brand, C: ManagerCell>(
    deltas: &[Delta<B, C>],
    header: &Arc<Symbols<Symbol>>,
    node: &Minterm<Symbol>,
) -> Minterm<Symbol> {
    let next: Vec<(&str, Option<bool>)> = deltas
        .iter()
        .map(|(name, d)| (name.as_str(), d.evaluate(node).ok()))
        .collect();
    node_from_opt(header, |name| match next.iter().find(|(n, _)| *n == name) {
        Some((_, v)) => *v,
        None => node.value_of(name),
    })
}

/// Whether `node` is stable: one [`step`] leaves it unchanged (every defined state variable already
/// equals its δ, and no absent one has become forced).
pub fn is_stable<B: Brand, C: ManagerCell>(deltas: &[Delta<B, C>], node: &Minterm<Symbol>) -> bool {
    let header = node.symbols();
    step(deltas, header, node) == *node
}

/// Settle the state under `node`'s fixed inputs: iterate [`step`] to a fixpoint. The fixpoint may still
/// leave state variables absent — those the inputs (and resolved state) do not determine. Returns `None`
/// if the state oscillates without settling (a metastable / arbitration condition).
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

/// Explore the reachable **stable** states of the machine, starting from initialisation candidates
/// discovered from the signal covers (never an assumed all-zero state).
///
/// `state_deltas` are the state variables' δ (used to settle and to build each state variable's on/off
/// sets); `seed_funcs` are the characteristic functions whose on/off covers over the inputs seed the
/// candidate pool (the state δ plus the combinational outputs, so combinational cells seed too).
///
/// Pre-step (no `evaluate`): for each candidate input `x` — an input minterm drawn from the pooled
/// on/off covers — its **settlement map** records, per state variable `w`, `Some(true)` if `x` forces
/// `w=1` (in on(w), not off(w)), `Some(false)` if it forces `w=0`, else absent. Candidates are ranked by
/// how many state variables they settle, ties broken toward state nearest the inputs. Exploration then
/// seeds the BFS from the ranked candidates in parallel, each start being the candidate's inputs plus
/// its settled state, and refines further state with [`settle`] as inputs toggle.
///
/// Shared by [`super::arcs`] and [`super::confluence`], which re-walk `order`.
pub fn explore<B: Brand, C: ManagerCell>(
    state_deltas: &[Delta<B, C>],
    seed_funcs: &[Bdd<B, C>],
    full_header: &Arc<Symbols<Symbol>>,
    input_names: &[String],
    state_names: &[String],
) -> Explored {
    let input_header = header(input_names);
    let k = state_names.len();
    let state_index: HashMap<&str, usize> = state_names
        .iter()
        .enumerate()
        .map(|(i, n)| (n.as_str(), i))
        .collect();

    // On/off cover of a function over the inputs, as a set of full input minterms (maximize expands
    // every don't-care, so each cube is a complete input assignment). Projected onto the shared input
    // header so membership tests compare canonically.
    let cover_inputs = |f: &Bdd<B, C>| -> BTreeSet<Minterm<Symbol>> {
        f.maximize(input_names)
            .cubes()
            .map(|c| c.inputs().project_onto(&input_header))
            .collect()
    };

    // Candidate pool: on and off cover minterms of every seed function.
    let mut pool: BTreeSet<Minterm<Symbol>> = BTreeSet::new();
    for f in seed_funcs {
        pool.extend(cover_inputs(f));
        pool.extend(cover_inputs(&!f));
    }

    // Per-state-variable on/off sets, for settlement by membership.
    type InputSet = BTreeSet<Minterm<Symbol>>;
    let on_off: Vec<(InputSet, InputSet)> = state_deltas
        .iter()
        .map(|(_, d)| (cover_inputs(d), cover_inputs(&!d)))
        .collect();

    // Depth of each state variable from the inputs (shallowest dependency chain), for the ranking
    // tie-break. A variable driven purely by inputs is depth 1; others are 1 + the shallowest state
    // variable they reference. Pure cycles (no input-only base) stay at the max.
    let support: Vec<BTreeSet<usize>> = state_deltas
        .iter()
        .map(|(_, d)| {
            d.variables()
                .filter_map(|v| state_index.get(v.as_str()).copied())
                .collect()
        })
        .collect();
    let mut depth = vec![u32::MAX; k];
    for _ in 0..=k {
        for i in 0..k {
            let others = support[i].iter().copied().filter(|j| *j != i);
            let base = others
                .filter_map(|j| (depth[j] != u32::MAX).then_some(depth[j]))
                .min();
            let d = match (support[i].iter().all(|j| *j == i), base) {
                (true, _) => 1,            // driven only by inputs (and possibly itself)
                (false, Some(m)) => 1 + m, // one hop past its shallowest resolved dependency
                (false, None) => u32::MAX, // not yet reachable from the inputs
            };
            if d < depth[i] {
                depth[i] = d;
            }
        }
    }

    // Settlement map of a candidate input: per state variable, the value it forces (or absent).
    let settlement = |x: &Minterm<Symbol>| -> Vec<Option<bool>> {
        on_off
            .iter()
            .map(|(on, off)| match (on.contains(x), off.contains(x)) {
                (true, false) => Some(true),
                (false, true) => Some(false),
                _ => None,
            })
            .collect()
    };
    let settle_count = |m: &[Option<bool>]| m.iter().filter(|o| o.is_some()).count();
    let depth_sum = |m: &[Option<bool>]| -> u64 {
        m.iter()
            .enumerate()
            .filter(|(_, o)| o.is_some())
            .map(|(i, _)| depth[i] as u64)
            .sum()
    };

    // Rank the candidates: most state variables settled first, ties toward state nearest the inputs,
    // then by minterm order for determinism.
    let mut ranked: Vec<(Minterm<Symbol>, Vec<Option<bool>>)> = pool
        .into_iter()
        .map(|x| (x.clone(), settlement(&x)))
        .collect();
    ranked.sort_by(|a, b| {
        settle_count(&b.1)
            .cmp(&settle_count(&a.1))
            .then_with(|| depth_sum(&a.1).cmp(&depth_sum(&b.1)))
            .then_with(|| a.0.cmp(&b.0))
    });

    // Seed the BFS from the ranked candidates in parallel: each start is the candidate's inputs plus
    // its settled state, then settled to a fixpoint. Metastable seeds (no fixpoint) are dropped.
    let mut prev: HashMap<Minterm<Symbol>, Option<Minterm<Symbol>>> = HashMap::new();
    let mut queue: VecDeque<Minterm<Symbol>> = VecDeque::new();
    for (x, map) in &ranked {
        let seed = node_from_opt(full_header, |name| {
            x.value_of(name)
                .or_else(|| state_index.get(name).and_then(|i| map[*i]))
        });
        let Some(st) = settle(state_deltas, full_header, &seed) else {
            continue;
        };
        if let std::collections::hash_map::Entry::Vacant(e) = prev.entry(st.clone()) {
            e.insert(None);
            queue.push_back(st);
        }
    }

    // BFS: from each node toggle one input at a time, hold the state, and settle. Metastable toggles
    // (no fixpoint) are dropped.
    let mut order: Vec<Minterm<Symbol>> = Vec::new();
    while let Some(node) = queue.pop_front() {
        order.push(node.clone());
        for related in input_names {
            let toggled = node_from_opt(full_header, |name| {
                let cur = node.value_of(name);
                if name == related.as_str() {
                    cur.map(|v| !v)
                } else {
                    cur
                }
            });
            let Some(np) = settle(state_deltas, full_header, &toggled) else {
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
    fn hold_state_leaves_output_absent() {
        // Under a hold input (A=1 B=0) with Q undefined, Q is not forced: it stays absent.
        let builder = bdd_builder!();
        let dq = builder.parse("A*B + Q*(A+B)").unwrap();
        let deltas = vec![("Q".to_string(), dq)];
        let hdr = header(&["A".into(), "B".into(), "Q".into()]);
        let node = node_from_opt(&hdr, |n| match n {
            "A" => Some(true),
            "B" => Some(false),
            _ => None,
        });
        let settled = settle(&deltas, &hdr, &node).expect("settles");
        assert_eq!(settled.value_of("Q"), None);
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
