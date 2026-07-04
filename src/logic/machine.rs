//! The cell's asynchronous state machine, expressed natively over espresso-logic minterms.
//!
//! A cell is a state machine over `inputs × state-variables`. A **node** is a self-describing
//! [`Minterm<Symbol>`] — it carries its own ordered columns ([`Minterm::vars`]), so there is no shared
//! header object. Every input carries a concrete value and each state variable is either **defined** (a
//! concrete `0`/`1`) or **absent** — encoded as the don't-care `-`, never a placeholder value. Power-on
//! is the inputs-only node: no state fixed. The next-state map settles the state columns (via each state
//! variable's δ, [`super::resolve::delta`]) using [`Bdd::evaluate`], which reads a δ under the node's
//! fixed columns and returns `Ok(v)` only when they force it — an absent state variable stays absent
//! (its δ provably does not depend on it yet, so `evaluate` returns `Err`). A node is *stable* when it
//! is its own next-state.
//!
//! Start states are not assumed: [`explore`] discovers them from the forced on/off covers of the signal
//! functions over the cell inputs ([`Bdd::cover_over_fr`]) — input vectors that force a signal
//! regardless of the undefined power-on state — so a state-holding cell whose state is undefined
//! at the all-zero input (its reset is an input sequence, not a level) is initialised by the sequence
//! that actually resolves it — the async pins, a clock edge, both requests high — rather than by an
//! arbitrary held combination.

use std::collections::{BTreeSet, HashMap, VecDeque};

use espresso_logic::bdd::{Bdd, Brand, ManagerCell};
use espresso_logic::{Minterm, Symbol};

/// A state variable paired with its next-state function δ (over inputs + state variables).
pub type Delta<B, C> = (String, Bdd<B, C>);

/// Build a fully-fixed node over `names` from a `name -> value` lookup (called once per variable).
#[cfg(test)]
pub fn node_from(names: &[&str], value: impl Fn(&str) -> bool) -> Minterm<Symbol> {
    Minterm::with_labels(
        &names
            .iter()
            .map(|n| (*n, Some(value(n))))
            .collect::<Vec<_>>(),
    )
    .expect("distinct labels")
}

/// Build a node over `names` from a `name -> Option<value>` lookup: `None` leaves that variable
/// **absent** (unresolved, encoded `-`), `Some(v)` fixes it. Inputs are always `Some`, state variables
/// may be either.
#[cfg(test)]
pub fn node_from_opt(names: &[&str], value: impl Fn(&str) -> Option<bool>) -> Minterm<Symbol> {
    Minterm::with_labels(&names.iter().map(|n| (*n, value(n))).collect::<Vec<_>>())
        .expect("distinct labels")
}

/// `node` with each name in `names` flipped in value; an absent field (`-`) stays absent, everything
/// else keeps its current field. Clones the node and mutates the named columns in place.
pub fn toggle(node: &Minterm<Symbol>, names: &[&str]) -> Minterm<Symbol> {
    let mut next = node.clone();
    for nm in names {
        if let Some(v) = node.value_of(*nm) {
            next.set_value_of(*nm, Some(!v)).expect("present label");
        }
    }
    next
}

/// One parallel next-state step: every state variable takes its δ evaluated at `node` — `Ok(v)` fixes
/// it, `Err` (δ still depends on an absent variable) leaves it **absent** (`-`). Inputs (and anything
/// else in the node) keep their current field.
fn step<B: Brand, C: ManagerCell>(
    deltas: &[Delta<B, C>],
    node: &Minterm<Symbol>,
) -> Minterm<Symbol> {
    // The deltas are exactly the trailing state-variable columns of the node, in order (a node is
    // `[inputs…, state_vars…]`), so index them positionally rather than scanning by name on this hot path.
    let vars = node.vars();
    let split = vars.len() - deltas.len();
    debug_assert!(
        vars[split..]
            .iter()
            .zip(deltas)
            .all(|(l, (name, _))| l.as_str() == name.as_str()),
        "step: deltas must be the trailing state-variable columns of the node, in order"
    );
    // Each δ is evaluated against the pre-mutation `node` (a parallel next-state), and an absent
    // dependency (`Err` → `None`) writes `-` = absent.
    let mut next = node.clone();
    for (j, (_, d)) in deltas.iter().enumerate() {
        next.set_value_at(split + j, d.evaluate(node).ok())
            .expect("state column in range");
    }
    next
}

/// Whether `node` is stable: one [`step`] leaves it unchanged (every defined state variable already
/// equals its δ, and no absent one has become forced).
#[cfg(test)]
pub fn is_stable<B: Brand, C: ManagerCell>(deltas: &[Delta<B, C>], node: &Minterm<Symbol>) -> bool {
    step(deltas, node) == *node
}

/// Settle the state under `node`'s fixed inputs: iterate [`step`] to a fixpoint. The fixpoint may still
/// leave state variables absent — those the inputs (and resolved state) do not determine. Returns `None`
/// if the state oscillates without settling (a metastable / arbitration condition).
pub fn settle<B: Brand, C: ManagerCell>(
    deltas: &[Delta<B, C>],
    node: &Minterm<Symbol>,
) -> Option<Minterm<Symbol>> {
    settle_or_cycle(deltas, node).ok()
}

/// Like [`settle`], but on oscillation returns the periodic cycle itself — the sequence of states
/// from the first repeated state back around — so callers can name the oscillating variables.
pub fn settle_or_cycle<B: Brand, C: ManagerCell>(
    deltas: &[Delta<B, C>],
    node: &Minterm<Symbol>,
) -> Result<Minterm<Symbol>, Vec<Minterm<Symbol>>> {
    let mut trace: Vec<Minterm<Symbol>> = vec![node.clone()];
    let mut pos: HashMap<Minterm<Symbol>, usize> = HashMap::new();
    pos.insert(node.clone(), 0);
    let mut cur = node.clone();
    loop {
        let next = step(deltas, &cur);
        if next == cur {
            return Ok(cur); // fixpoint
        }
        if let Some(&p) = pos.get(&next) {
            return Err(trace[p..].to_vec()); // revisited a non-fixpoint state → the oscillating cycle
        }
        pos.insert(next.clone(), trace.len());
        trace.push(next.clone());
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

impl Explored {
    /// The input-projected BFS path into `node` (the prevector): walk predecessors back to a start,
    /// reverse, and project each step onto `input_names`.
    pub fn path_to(&self, node: &Minterm<Symbol>, input_names: &[String]) -> Vec<Minterm<Symbol>> {
        let mut chain = vec![node.clone()];
        let mut cur = node.clone();
        while let Some(Some(p)) = self.prev.get(&cur) {
            chain.push(p.clone());
            cur = p.clone();
        }
        chain.reverse();
        chain.iter().map(|m| m.project_to(input_names)).collect()
    }
}

/// Explore the reachable **stable** states of the machine, starting from initialisation candidates
/// discovered from the signal covers (never an assumed all-zero state).
///
/// `state_deltas` are the state variables' δ (used to settle and to build each state variable's on/off
/// sets); `seed_funcs` are the characteristic functions whose on/off covers over the inputs seed the
/// candidate pool (the state δ plus the combinational outputs, so combinational cells seed too). Both
/// on- and off-set candidates come from a single FR extraction per seed (see `cover_inputs`).
///
/// Pre-step: for each candidate input `x` — an input minterm drawn from the pooled on/off covers — its
/// **settlement map** records, per state variable `w`, the value the fixed inputs force on `w`'s δ via
/// [`Bdd::evaluate`]: `Some(true)` if they force `w=1`, `Some(false)` if `w=0`, else absent (the δ still
/// depends on unresolved state). Candidates are ranked by
/// how many state variables they settle, ties broken toward state nearest the inputs. Exploration then
/// seeds the BFS from the ranked candidates in parallel: each candidate input is widened onto the full
/// `[inputs…, state_vars…]` columns (the state columns come in absent) and settled with [`settle`],
/// refining further state as inputs toggle.
///
/// Shared by [`super::arcs`] and [`super::confluence`], which re-walk `order`.
pub fn explore<B: Brand, C: ManagerCell>(
    state_deltas: &[Delta<B, C>],
    seed_funcs: &[Bdd<B, C>],
    input_names: &[String],
    state_names: &[String],
) -> Explored {
    // The full node columns: inputs then state variables, in state-variable order (see analysis.rs).
    let full_names: Vec<String> = input_names
        .iter()
        .cloned()
        .chain(state_names.iter().cloned())
        .collect();
    let k = state_names.len();
    let state_index: HashMap<&str, usize> = state_names
        .iter()
        .enumerate()
        .map(|(i, n)| (n.as_str(), i))
        .collect();

    // Forced on/off cover of a function over the inputs. `cover_over_fr(input_names)` re-bases the
    // function onto the inputs by universal projection — each cube is an input assignment that forces
    // the function's value regardless of the (undefined) power-on state — yielding both the on-set (F)
    // and off-set (R) in one FR cover. `.maximize()` expands every don't-care, so each cube is a
    // complete input assignment; `project_to` re-homes each cube's inputs onto the input columns for
    // canonical membership tests.
    let cover_inputs = |f: &Bdd<B, C>| -> BTreeSet<Minterm<Symbol>> {
        f.cover_over_fr(input_names)
            .maximize()
            .cubes()
            .map(|c| c.inputs().project_to(input_names))
            .collect()
    };

    // Candidate pool: the forced on/off input minterms of every seed function (one FR extraction each).
    let mut pool: BTreeSet<Minterm<Symbol>> = BTreeSet::new();
    for f in seed_funcs {
        // ¬f's FR cover is f's with the F/R sides swapped, and `cover_inputs` pools `.cubes()`
        // type-blind, so `cover_inputs(&!f) == cover_inputs(f)` as a minterm set — no complement call.
        pool.extend(cover_inputs(f));
    }

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

    // Settlement map of a candidate input: per state variable, the value its δ takes when the fixed
    // inputs already determine it (evaluate → `Ok`), or absent when the δ still depends on unresolved
    // state (`Err`). This is the membership test on(w)/off(w) done directly against each δ, and is used
    // only to RANK the candidates below (the seed itself is extracted and widened, not rebuilt from it).
    let settlement = |x: &Minterm<Symbol>| -> Vec<Option<bool>> {
        state_deltas
            .iter()
            .map(|(_, d)| d.evaluate(x).ok())
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

    // Seed the BFS from the ranked candidates in parallel: widen each candidate input onto the full
    // columns (the state columns arrive absent) and settle to a fixpoint. Metastable seeds (no
    // fixpoint) are dropped.
    let mut prev: HashMap<Minterm<Symbol>, Option<Minterm<Symbol>>> = HashMap::new();
    let mut queue: VecDeque<Minterm<Symbol>> = VecDeque::new();
    for (x, _) in &ranked {
        let seed = x.project_to(&full_names);
        let Some(st) = settle(state_deltas, &seed) else {
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
            let toggled = toggle(&node, &[related.as_str()]);
            let Some(np) = settle(state_deltas, &toggled) else {
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
        // Q = A*B + Q*(A+B). Over columns [A, B, Q], hold state 01/10 keeps Q; 11 forces Q high.
        let builder = bdd_builder!();
        let dq = builder.parse("A*B + Q*(A+B)").unwrap();
        let deltas = vec![("Q".to_string(), dq)];

        // A=1 B=0 Q=1 is a stable hold state.
        let hold = node_from(&["A", "B", "Q"], |n| matches!(n, "A" | "Q"));
        assert!(is_stable(&deltas, &hold));
        assert_eq!(settle(&deltas, &hold).as_ref(), Some(&hold));

        // A=1 B=1 Q=0 is not stable; it settles to Q=1.
        let forcing = node_from(&["A", "B", "Q"], |n| matches!(n, "A" | "B"));
        assert!(!is_stable(&deltas, &forcing));
        let settled = settle(&deltas, &forcing).expect("settles");
        assert_eq!(settled.value_of("Q"), Some(true));
    }

    #[test]
    fn hold_state_leaves_output_absent() {
        // Under a hold input (A=1 B=0) with Q undefined, Q is not forced: it stays absent.
        let builder = bdd_builder!();
        let dq = builder.parse("A*B + Q*(A+B)").unwrap();
        let deltas = vec![("Q".to_string(), dq)];
        let node = node_from_opt(&["A", "B", "Q"], |n| match n {
            "A" => Some(true),
            "B" => Some(false),
            _ => None,
        });
        let settled = settle(&deltas, &node).expect("settles");
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
        let both_low = node_from(&["A", "B", "Qa", "Qb"], |n| matches!(n, "A" | "B"));
        assert_eq!(settle(&deltas, &both_low), None);
    }

    #[test]
    fn settle_or_cycle_names_the_oscillating_pair() {
        // Same cross-coupled mutex as `metastable_mutex_oscillates_to_none`, but probed through
        // settle_or_cycle: the returned cycle should have length 2, with Qa and Qb each taking both
        // values across it (the pair genuinely oscillates, rather than one of them staying fixed).
        let builder = bdd_builder!();
        let da = builder.parse("!Qb*A").unwrap();
        let db = builder.parse("!Qa*B").unwrap();
        let deltas = vec![("Qa".to_string(), da), ("Qb".to_string(), db)];
        let both_low = node_from(&["A", "B", "Qa", "Qb"], |n| matches!(n, "A" | "B"));

        let cycle = settle_or_cycle(&deltas, &both_low).expect_err("oscillates, no fixpoint");
        assert_eq!(cycle.len(), 2, "expected a length-2 cycle, got {cycle:?}");

        let qa_values: BTreeSet<_> = cycle.iter().map(|m| m.value_of("Qa")).collect();
        let qb_values: BTreeSet<_> = cycle.iter().map(|m| m.value_of("Qb")).collect();
        assert_eq!(
            qa_values.len(),
            2,
            "Qa should differ across the cycle, got {cycle:?}"
        );
        assert_eq!(
            qb_values.len(),
            2,
            "Qb should differ across the cycle, got {cycle:?}"
        );
    }
}
