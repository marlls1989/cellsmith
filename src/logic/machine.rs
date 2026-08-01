//! The cell's asynchronous state machine, expressed natively over espresso-logic minterms.
//!
//! A cell is a state machine over `inputs × coordinates`, where the **coordinates** are every signal
//! surviving the minimisation — the state variables plus the combinational signals kept alongside them
//! ([`Coordinates`]). A **node** is a self-describing [`Minterm<Symbol>`] — it carries its own ordered
//! columns ([`Minterm::vars`]), so there is no shared header object. Every input carries a concrete value
//! and each coordinate is either **defined** (a concrete `0`/`1`) or **absent** — encoded as the
//! don't-care `-`. Power-on is the inputs-only node: no coordinate fixed. The next-state map settles the
//! coordinate columns (via each coordinate's minimised next-state function, read directly from the model
//! (see [`super::minimise`])) using [`Bdd::evaluate`], which reads a δ under the node's
//! fixed columns and returns `Ok(v)` only when they force it — an absent coordinate stays absent
//! (its δ provably does not depend on it yet, so `evaluate` returns `Err`). A node is *stable* when it
//! is its own next-state.
//!
//! Start states are not assumed: [`explore`] discovers them from the forced on/off covers of the signal
//! functions over the cell inputs ([`Bdd::cover_over_fr`]) — input vectors that force a signal
//! regardless of the undefined power-on state — so a state-holding cell whose state is undefined
//! at the all-zero input (its reset is an input sequence, not a level) is initialised by the sequence
//! that actually resolves it — the async pins, a clock edge, both requests high — rather than by an
//! arbitrary held combination.
//!
//! The machine model, settling, cycle detection and start-state discovery are described concept-first in
//! `state-machine-arc-engine.md`; this module records only the implementation specifics the concept
//! doesn't need.
//!
//! # Exploration budgets
//!
//! [`explore`] is bounded by two counters, carried together in [`ExplorationBudget`] and charged against
//! the work the call actually performs — never against the cell's declared shape (a cell is not turned
//! away for having many inputs or many state variables). Whichever counter trips is the returned
//! [`ExplorationLimit`] variant, carrying the ceiling it passed.
//!
//! * **`candidates`** counts the **seed minterms** of the candidate pool. The pool expands every seed
//!   function's forced FR cover into complete input assignments, so one cube carrying `d` don't-care
//!   *input* columns is exactly `2^d` minterms — a quantity in the input count alone, not in
//!   inputs + state variables. Every pooled candidate then costs a settlement map (one δ evaluation per
//!   state variable) for the ranking, so the pool sets both the memory and the ranking cost.
//!   [`Cube::expand_to`](espresso_logic::Cube::expand_to) yields a cube's minterms lazily and knows its
//!   own length up front, so each cube is charged before it is expanded and an over-budget pool is
//!   counted without ever being materialised.
//! * **`states`** counts the reachable stable states the BFS records in [`Explored::order`]. That vector
//!   is what the downstream passes re-walk: [`super::arcs::derive`] at O(|order| · inputs) settles and
//!   [`super::confluence::detect`] at O(|order| · inputs²), so a machine that explores unboundedly many
//!   states is one whose hazard detection does not finish.

use std::collections::{BTreeSet, HashMap, VecDeque};
use std::sync::atomic::{AtomicUsize, Ordering};

use espresso_logic::bdd::{Bdd, Brand, ManagerCell};
use espresso_logic::{Minterm, Symbol};
use rayon::prelude::*;

/// A coordinate paired with its next-state function δ (over inputs + state variables).
pub type Delta<B, C> = (Symbol, Bdd<B, C>);

/// The machine's coordinates — every signal surviving the minimisation — split by the role [`explore`]
/// gives each half.
///
/// `state` are the state variables, the signals that hold memory. `combinational` are the remaining
/// survivors: signals off every feedback cycle that the minimisation kept because something addresses
/// them by name (an output pin, an exposed internal node). A combinational survivor is in lockstep with
/// the state variables — its δ is a function of the inputs and the state alone — so it is a coordinate
/// like any other, stepped with the rest and landing absent where its δ is unsatisfied.
///
/// BOTH halves are stepped ([`Self::stepped`]) and both are node columns ([`Self::names`]). Only `state`
/// is measured by [`explore`]'s candidate ranking and its depth tie-break, so a combinational coordinate
/// cannot change which states the BFS reaches, nor in which order.
pub struct Coordinates<'d, B: Brand, C: ManagerCell> {
    /// The state variables' δ, in signal order.
    pub state: &'d [Delta<B, C>],
    /// The combinational survivors' δ, in signal order.
    pub combinational: &'d [Delta<B, C>],
}

impl<B: Brand, C: ManagerCell> Coordinates<'_, B, C> {
    /// The coordinate names in node-column order: the state variables, then the combinational
    /// survivors.
    pub fn names(&self) -> Vec<Symbol> {
        self.state
            .iter()
            .chain(self.combinational)
            .map(|(name, _)| name.clone())
            .collect()
    }

    /// Every coordinate's δ, in the same order as [`Self::names`] — what one [`step`] writes.
    pub fn stepped(&self) -> Vec<Delta<B, C>> {
        self.state
            .iter()
            .chain(self.combinational)
            .cloned()
            .collect()
    }
}

/// What one [`explore`] call may spend, in the two quantities that drive its cost (see the module
/// documentation): the seed minterms pooled as initialisation candidates and the reachable stable states
/// the BFS records.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExplorationBudget {
    /// Ceiling on the candidate pool's seed minterms, charged per expanded FR cube.
    pub candidates: usize,
    /// Ceiling on the reachable stable states recorded in [`Explored::order`].
    pub states: usize,
}

impl Default for ExplorationBudget {
    /// 2^22 seed minterms and 2^20 explored states: the pool ceiling holds the candidate expansion (and
    /// the per-candidate settlement maps ranking it) to a few million rows, and a machine reaching a
    /// million stable states carries a downstream hazard probe that does not finish.
    fn default() -> Self {
        Self {
            candidates: 1 << 22,
            states: 1 << 20,
        }
    }
}

/// The counter that stopped an [`explore`] call, carrying the ceiling it passed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExplorationLimit {
    /// The candidate pool passed this many seed minterms.
    Candidates(usize),
    /// The BFS passed this many reachable stable states.
    States(usize),
}

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

/// One parallel next-state step: every coordinate takes its δ evaluated at `node` — `Some(v)` fixes
/// it, `None` (δ still depends on an absent coordinate) leaves it **absent** (`-`). Inputs (and anything
/// else in the node) keep their current field.
fn step<B: Brand, C: ManagerCell>(
    deltas: &[Delta<B, C>],
    node: &Minterm<Symbol>,
) -> Minterm<Symbol> {
    // Each δ is evaluated against the pre-mutation `node` (a parallel next-state), and an absent
    // dependency (`evaluate_fast` returning `None`) writes `-` = absent. The write is BY NAME:
    // `Minterm::set_value_of` reports a label the node does not carry as an error even when the value
    // written is `None`, so the `expect` is a standing check — in release as well as debug — that every
    // δ handed here names a column of the node.
    let mut next = node.clone();
    for (name, d) in deltas {
        next.set_value_of(name.as_str(), d.evaluate_fast(node))
            .expect("every delta names a column of the node");
    }
    next
}

/// Whether `node` is stable: one [`step`] leaves it unchanged (every defined state variable already
/// equals its δ, and no absent one has become forced).
#[cfg(test)]
pub fn is_stable<B: Brand, C: ManagerCell>(deltas: &[Delta<B, C>], node: &Minterm<Symbol>) -> bool {
    step(deltas, node) == *node
}

/// Settle the state under `node`'s fixed inputs: iterate `step` to a fixpoint. The fixpoint may still
/// leave state variables absent — those the inputs (and resolved state) do not determine. Returns `None`
/// if the state oscillates without settling (an oscillation hazard, which risks metastability).
pub fn settle<B: Brand, C: ManagerCell>(
    deltas: &[Delta<B, C>],
    node: &Minterm<Symbol>,
) -> Option<Minterm<Symbol>> {
    settle_or_cycle(deltas, node).ok()
}

/// Like [`settle`], but on oscillation returns the periodic cycle itself — the sequence of states
/// from the first repeated state back around — so callers can name the oscillating variables. The
/// concept is in `state-machine-arc-engine.md` §5; the mechanics: `pos` maps each visited state to its
/// index in `trace` (an O(1) revisit check), and `trace` preserves visitation order, so a revisit of a
/// state already at index `p` slices the cycle out as `trace[p..]`.
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
#[derive(Debug)]
pub struct Explored {
    /// Reachable stable nodes in BFS dequeue order (each appears once).
    pub order: Vec<Minterm<Symbol>>,
    /// Predecessor of each reachable node (`None` at a start node).
    pub prev: HashMap<Minterm<Symbol>, Option<Minterm<Symbol>>>,
}

impl Explored {
    /// The settled BFS start states (the seeds), in discovery order: reachable stable nodes with no
    /// predecessor. Seeds are inserted into `prev` with `None` before the BFS begins and entries are
    /// never overwritten (Vacant-only insertion), so `prev[n] == None` identifies exactly the
    /// deduplicated seed set.
    pub fn seeds(&self) -> impl Iterator<Item = &Minterm<Symbol>> {
        self.order
            .iter()
            .filter(|n| matches!(self.prev.get(*n), Some(None)))
    }

    /// The input-projected BFS path into `node` (the prevector): walk predecessors back to a start,
    /// reverse, and project each step onto `input_names`.
    pub fn path_to(&self, node: &Minterm<Symbol>, input_names: &[Symbol]) -> Vec<Minterm<Symbol>> {
        let mut chain = vec![node.clone()];
        let mut cur = node.clone();
        while let Some(Some(p)) = self.prev.get(&cur) {
            chain.push(p.clone());
            cur = p.clone();
        }
        chain.reverse();
        chain.iter().map(|m| m.project_to(input_names)).collect()
    }

    /// This exploration carried onto the coordinates of another view of the same cell.
    ///
    /// Every node — each entry of `order`, and both ends of every `prev` edge — is re-homed onto `names`
    /// by [`Minterm::project_to`]: a coordinate `names` does not carry is dropped, a coordinate it names
    /// that this exploration lacks arrives don't-know, and a shared one keeps the value it holds here.
    /// Nothing is re-evaluated, so a don't-know projects to a don't-know.
    ///
    /// The predecessor chain keeps its shape — a projected node's predecessor is the projection of its
    /// own predecessor here, never re-derived — so [`Self::seeds`], [`Self::path_to`] and every
    /// prevector, its length included, read the same on the projection as on this exploration.
    pub fn project_to(&self, names: &[Symbol]) -> Explored {
        // `order` holds each node once, and it still does after the projection: two stable nodes
        // differing only in a released column cannot both be stable, because stability forces that
        // column to equal its δ, which the surviving columns determine. So no two entries meet.
        let order: Vec<Minterm<Symbol>> = self.order.iter().map(|n| n.project_to(names)).collect();
        let prev: HashMap<Minterm<Symbol>, Option<Minterm<Symbol>>> = self
            .prev
            .iter()
            .map(|(node, p)| {
                (
                    node.project_to(names),
                    p.as_ref().map(|p| p.project_to(names)),
                )
            })
            .collect();
        // The container invariant above, checked rather than assumed: a collision would drop a node
        // from `order` or a predecessor chain from `prev` without a word.
        debug_assert_eq!(
            order.iter().collect::<BTreeSet<_>>().len(),
            order.len(),
            "Explored::project_to: the projection put two explored states on one node",
        );
        debug_assert_eq!(
            prev.len(),
            self.prev.len(),
            "Explored::project_to: the projection put two predecessor keys on one node",
        );
        Explored { order, prev }
    }
}

/// Explore the reachable **stable** states of the machine, starting from initialisation candidates
/// discovered from the signal covers (never an assumed all-zero state).
///
/// `coords` are the machine's coordinates — the state variables and the combinational survivors, both
/// stepped to settle a node; `seed_funcs` are the characteristic functions whose on/off covers over the
/// inputs seed the candidate pool (the state δ plus the combinational outputs, so combinational cells
/// seed too). Both on- and off-set candidates come from a single FR extraction per seed (see
/// `cover_inputs`).
///
/// Pre-step: for each candidate input `x` — an input minterm drawn from the pooled on/off covers — its
/// **settlement map** records, per state variable `w`, the value the fixed inputs force on `w`'s δ via
/// [`Bdd::evaluate`]: `Some(true)` if they force `w=1`, `Some(false)` if `w=0`, else absent (the δ still
/// depends on unresolved state). Candidates are ranked by
/// how many state variables they settle, ties broken toward state nearest the inputs. Exploration then
/// seeds the BFS from the ranked candidates in parallel: each candidate input is widened onto the full
/// `[inputs…, coordinates…]` columns (the coordinate columns come in absent) and settled with [`settle`],
/// refining further state as inputs toggle.
///
/// Shared by [`super::arcs`] and [`super::confluence`], which re-walk `order`.
///
/// `budget` bounds the two costs the exploration incurs — the pooled seed minterms and the recorded
/// stable states (see the module documentation) — and the counter that passes its ceiling comes back as
/// the [`ExplorationLimit`] error.
pub fn explore<B: Brand, C: ManagerCell + Send + Sync>(
    coords: Coordinates<'_, B, C>,
    seed_funcs: &[Bdd<B, C>],
    input_names: &[Symbol],
    budget: &ExplorationBudget,
) -> Result<Explored, ExplorationLimit> {
    // The full node columns: inputs, then the coordinates in `Coordinates::names` order (see analysis.rs).
    let full_names: Vec<Symbol> = input_names.iter().cloned().chain(coords.names()).collect();
    // Every coordinate's δ, applied together by each `step` — the state variables and the combinational
    // survivors alike.
    let stepped = coords.stepped();
    let state_deltas = coords.state;
    let k = state_deltas.len();
    let state_index: HashMap<&str, usize> = state_deltas
        .iter()
        .enumerate()
        .map(|(i, (n, _))| (n.as_str(), i))
        .collect();

    // Forced on/off cover of a function over the inputs. `cover_over_fr(input_names)` re-bases the
    // function onto the inputs by universal projection — each cube is an input assignment that forces
    // the function's value regardless of the (undefined) power-on state — yielding both the on-set (F)
    // and off-set (R) in one FR cover. `expand_to(input_names)` then expands one cube into every
    // complete assignment of the input columns, on the canonical header membership tests compare on.
    //
    // The expansion is lazy and knows its length, so each cube's exact minterm count is charged into
    // `charged` before a single minterm is packed: the pool is measured whether or not it is affordable.
    // The charge is saturating, so a cube whose expansion exceeds `usize` still reads as over budget.
    let charged = AtomicUsize::new(0);
    let cover_inputs = |f: &Bdd<B, C>| -> BTreeSet<Minterm<Symbol>> {
        f.cover_over_fr(input_names)
            .cubes()
            .flat_map(|c| {
                let minterms = c.expand_to(input_names);
                let n = minterms.len();
                let total = charged
                    .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |t| {
                        Some(t.saturating_add(n))
                    })
                    .unwrap_or_else(|t| t) // the closure never declines, so this is the observed total
                    .saturating_add(n);
                // Past the ceiling the verdict is already settled, so nothing more is materialised.
                // Every cube is still charged, which keeps the total an interleaving-free sum and the
                // verdict identical under any thread count.
                (total <= budget.candidates)
                    .then_some(minterms)
                    .into_iter()
                    .flatten()
            })
            .collect()
    };

    // Candidate pool: the forced on/off input minterms of every seed function (one FR extraction each),
    // built in parallel across seed functions — set semantics make the union order-free.
    // ¬f's FR cover is f's with the F/R sides swapped, and `cover_inputs` pools `.cubes()`
    // type-blind, so `cover_inputs(&!f) == cover_inputs(f)` as a minterm set — no complement call.
    let pool: BTreeSet<Minterm<Symbol>> =
        seed_funcs.par_iter().flat_map_iter(cover_inputs).collect();
    if charged.load(Ordering::Relaxed) > budget.candidates {
        return Err(ExplorationLimit::Candidates(budget.candidates));
    }

    // Depth of each state variable from the inputs (shallowest dependency chain), for the ranking
    // tie-break. A variable driven purely by inputs is depth 1; others are 1 + the shallowest state
    // variable they reference. Pure cycles (no input-only base) stay at the max.
    // Sequential: this is a relaxation — each pass's `depth` values feed the next pass — so passes
    // cannot be parallelised.
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
    // inputs already determine it (`evaluate_fast` → `Some(v)`), or absent when the δ still depends
    // on unresolved state (`None`). This is the membership test on(w)/off(w) done directly against
    // each δ, and is used only to RANK the candidates below (the seed itself is extracted and widened,
    // not rebuilt from it).
    //
    // The pool, this map, `settle_count`, `depth_sum` and the depth relaxation above are quantities over
    // `coords.state` and `seed_funcs` ALONE — never the combinational coordinates. The ranking fixes the
    // seed order, hence the BFS discovery order, hence every prevector's length, and `prevector.len()` is
    // the tie-break the constraint dedup picks its representative by (see `super::confluence`). Ranking
    // over the combinational δ would move hazard and constraint representatives, and leakage, for EVERY
    // cell — cells that expose nothing included. Both halves are STEPPED; only `state` is RANKED.
    let settlement = |x: &Minterm<Symbol>| -> Vec<Option<bool>> {
        state_deltas
            .iter()
            .map(|(_, d)| d.evaluate_fast(x))
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
        .into_par_iter()
        .map(|x| {
            let m = settlement(&x);
            (x, m)
        })
        .collect();
    ranked.sort_by(|a, b| {
        settle_count(&b.1)
            .cmp(&settle_count(&a.1))
            .then_with(|| depth_sum(&a.1).cmp(&depth_sum(&b.1)))
            .then_with(|| a.0.cmp(&b.0))
    });

    // Seed the BFS from the ranked candidates: widen each candidate input onto the full columns (the
    // coordinate columns arrive absent, target-only labels of the projection) and settle to a fixpoint,
    // which is where a combinational coordinate first takes a value — no separate fill phase. Metastable
    // seeds (no fixpoint) are dropped. Sequential: the Vacant-insertion order into `prev` fixes the order
    // seeds are pushed onto the BFS queue.
    let mut prev: HashMap<Minterm<Symbol>, Option<Minterm<Symbol>>> = HashMap::new();
    let mut queue: VecDeque<Minterm<Symbol>> = VecDeque::new();
    for (x, _) in &ranked {
        let seed = x.project_to(&full_names);
        let Some(st) = settle(&stepped, &seed) else {
            continue;
        };
        if let std::collections::hash_map::Entry::Vacant(e) = prev.entry(st.clone()) {
            e.insert(None);
            queue.push_back(st);
        }
    }

    // BFS: from each node toggle one input at a time, hold the state, and settle. Metastable toggles
    // (no fixpoint) are dropped. Sequential: `queue`/`prev`/`order` and every prevector's shape are
    // derived from discovery order.
    let mut order: Vec<Minterm<Symbol>> = Vec::new();
    while let Some(node) = queue.pop_front() {
        order.push(node.clone());
        // `order` holds every state the exploration has recorded, seeds included (a seed enters here
        // when it is dequeued), so its length is the state counter.
        if order.len() > budget.states {
            return Err(ExplorationLimit::States(budget.states));
        }
        for related in input_names {
            let toggled = toggle(&node, &[related.as_str()]);
            let Some(np) = settle(&stepped, &toggled) else {
                continue;
            };
            if let std::collections::hash_map::Entry::Vacant(e) = prev.entry(np.clone()) {
                e.insert(Some(node.clone()));
                queue.push_back(np);
            }
        }
    }

    Ok(Explored { order, prev })
}

#[cfg(test)]
mod tests {
    use super::*;
    use espresso_logic::{bdd_builder, sync_bdd_builder};

    #[test]
    fn settles_a_c_element_hold() {
        // Q = A*B + Q*(A+B). Over columns [A, B, Q], hold state 01/10 keeps Q; 11 forces Q high.
        let builder = bdd_builder!();
        let dq = builder.parse("A*B + Q*(A+B)").unwrap();
        let deltas = vec![(Symbol::from("Q"), dq)];

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
    fn seeds_are_the_forced_on_off_covers() {
        // C2 (2-input Muller-C): Q = A*B + Q*(A+B). The settled BFS seeds are exactly the two forced
        // covers — the on-set (A=1,B=1,Q=1) and the off-set (A=0,B=0,Q=0), both present.
        let builder = sync_bdd_builder!();
        let dq = builder.parse("A*B + Q*(A+B)").unwrap();
        let deltas = vec![(Symbol::from("Q"), dq.clone())];
        let inputs = [Symbol::from("A"), Symbol::from("B")];
        let explored = explore(
            Coordinates {
                state: &deltas,
                combinational: &[],
            },
            &[dq],
            &inputs,
            &ExplorationBudget::default(),
        )
        .expect("a 2-input C-element is well inside the default budget");

        let seeds: Vec<Minterm<Symbol>> = explored.seeds().cloned().collect();
        let on = node_from(&["A", "B", "Q"], |_| true);
        let off = node_from(&["A", "B", "Q"], |_| false);
        assert_eq!(seeds.len(), 2, "expected exactly two seeds, got {seeds:?}");
        assert!(seeds.contains(&on), "on-set seed (A=1,B=1,Q=1) present");
        assert!(seeds.contains(&off), "off-set seed (A=0,B=0,Q=0) present");
    }

    #[test]
    fn wide_input_cell_trips_the_candidate_budget() {
        // Y = I0 over 24 inputs. Each FR cube of Y carries 23 don't-care input columns, so expanding
        // one packs 2^23 seed minterms — past the default 2^22 ceiling on that cube alone, and this
        // with no state variable at all: the charge reads the input columns, never the machine width.
        // The count comes from the lazy expansion's length before any minterm is packed, so a pool this
        // size is measured without being built (building it would cost gigabytes, and this test would
        // not return).
        let builder = sync_bdd_builder!();
        let f = builder.parse("I0").unwrap();
        let inputs: Vec<Symbol> = (0..24)
            .map(|i| Symbol::from(format!("I{i}").as_str()))
            .collect();
        let budget = ExplorationBudget::default();
        let verdict = |threads: usize| {
            let explored = rayon::ThreadPoolBuilder::new()
                .num_threads(threads)
                .build()
                .expect("thread pool")
                .install(|| {
                    explore(
                        Coordinates {
                            state: &[],
                            combinational: &[],
                        },
                        std::slice::from_ref(&f),
                        &inputs,
                        &budget,
                    )
                });
            let Err(limit) = explored else {
                panic!("the candidate pool passes the ceiling, so exploration must stop");
            };
            limit
        };
        assert_eq!(verdict(1), ExplorationLimit::Candidates(budget.candidates));
        // Every cube is charged whatever the interleaving and the early stop only fires once the
        // ceiling is already passed, so the verdict is the same however many threads expand the seeds.
        assert_eq!(verdict(1), verdict(8));
    }

    #[test]
    #[should_panic(expected = "every delta names a column of the node")]
    fn step_refuses_a_delta_with_no_column() {
        // `step` writes each δ by name, and `Minterm::set_value_of` reports a label the node does not
        // carry as an error even when the value written is absent. Hand it a node whose columns stop at
        // the inputs and the write is refused rather than landing on whatever column sits at that
        // position — the standing check that a δ handed to `step` names a coordinate of the node.
        let builder = bdd_builder!();
        let dq = builder.parse("A*B + Q*(A+B)").unwrap();
        let deltas = vec![(Symbol::from("Q"), dq)];
        let no_q = node_from(&["A", "B"], |n| n == "A");
        let _ = step(&deltas, &no_q);
    }

    #[test]
    fn hold_state_leaves_output_absent() {
        // Under a hold input (A=1 B=0) with Q undefined, Q is not forced: it stays absent.
        let builder = bdd_builder!();
        let dq = builder.parse("A*B + Q*(A+B)").unwrap();
        let deltas = vec![(Symbol::from("Q"), dq)];
        let node = node_from_opt(&["A", "B", "Q"], |n| match n {
            "A" => Some(true),
            "B" => Some(false),
            _ => None,
        });
        let settled = settle(&deltas, &node).expect("settles");
        assert_eq!(settled.value_of("Q"), None);
    }

    #[test]
    fn mutex_oscillates_to_none() {
        // Cross-coupled: Qa = !Qb*A, Qb = !Qa*B. Under A=B=1 the joint next-state of {Qa=0,Qb=0}
        // toggles both to 1 then back — no fixpoint reachable from it, so settle yields None.
        let builder = bdd_builder!();
        let da = builder.parse("!Qb*A").unwrap();
        let db = builder.parse("!Qa*B").unwrap();
        let deltas = vec![(Symbol::from("Qa"), da), (Symbol::from("Qb"), db)];
        let both_low = node_from(&["A", "B", "Qa", "Qb"], |n| matches!(n, "A" | "B"));
        assert_eq!(settle(&deltas, &both_low), None);
    }

    #[test]
    fn settle_or_cycle_names_the_oscillating_pair() {
        // Same cross-coupled mutex as `mutex_oscillates_to_none`, but probed through
        // settle_or_cycle: the returned cycle should have length 2, with Qa and Qb each taking both
        // values across it (the pair genuinely oscillates, rather than one of them staying fixed).
        let builder = bdd_builder!();
        let da = builder.parse("!Qb*A").unwrap();
        let db = builder.parse("!Qa*B").unwrap();
        let deltas = vec![(Symbol::from("Qa"), da), (Symbol::from("Qb"), db)];
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
