//! One-shot **state-space minimisation** of a cell's signal model.
//!
//! `resolve::state_variables` classifies a signal as a state coordinate purely by self-reachability in
//! the dependency graph. That over-counts: a signal that lies on a cycle but holds no genuine memory (an
//! interlock relay, an alias/complement of another signal, or a duplicate of a signal already kept) is
//! flagged as state, inflating the machine and emitting redundant internal nodes. This module rewrites
//! the shared per-cell BDD map **once**, before the machine pass, so that after it every surviving signal
//! is a genuine memory coordinate — a primary input or a self-reaching signal — and the machine's
//! next-state δ is a direct lookup in the map.
//!
//! # Pipeline
//!
//! A single fixpoint loop alternates two **output-preserving** passes — **dedup** (identical-δ merge)
//! then **guarded fold** — over the signals in `signals()` order (outputs first, then internals as
//! parsed): `loop { dedup_pass; fold_pass; if neither committed break }`. Every transformation in either
//! pass prefers to keep an output pin; an output is never purged.
//!
//! * **dedup — plain-BDD-equal merge.** Signals whose functions are the *exact same* BDD (bare ±aliases
//!   included — `!var(x)` is not special) are one coordinate seen more than once, and every such group
//!   is merged. A merged group keeps a single representative (an external output where the group holds
//!   one, so a pin is never lost) and renames the other members onto `var(rep)` everywhere via
//!   [`Bdd::compose_map`]. An **internal** duplicate always retires — it is purged and its consumers
//!   rewritten onto `var(rep)`. A duplicate **output** is never purged — it is only demoted to a bare
//!   `var(rep)` alias, and only when the group is **recurrent** (the rep's current δ references a group
//!   member, so the rep self-holds and the alias stays machine-evaluable — I3/I5); a non-recurrent
//!   all-output group commits nothing, leaving the duplicates as independent full-function signals.
//!   What remains for the fold is arity-1-gated substitution: a signal's function is composed into its
//!   consumers and dropped, refused only when doing so would fabricate a register — and permitted
//!   despite that risk when the folded function has support arity 1.
//! * **guarded fold — relay/alias elimination.** A signal `s` that does not appear in its own support is
//!   a combinational relay: at every stable state `s = δ_s(state)` with `s ∉ support(δ_s)`, so it is
//!   composed into each of its consumers via [`Bdd::compose`] and dropped — *unless* the fold would
//!   fabricate a register out of emergent memory (the arity-aware guard below). A bare ±alias is the
//!   arity-1 case and always folds; when a bare ±alias `s` is an output whose target is a surviving
//!   internal, that internal's definition is folded into `s` and the internal dropped, so the single
//!   coordinate lands on the output pin (its sign carried through the composition — incidental).
//!
//! # Lockstep frame
//!
//! A signal whose transition function depends on only *one* other signal is in **lockstep** with it —
//! the same coordinate, up to complement. A bare ±var alias carries exactly one bit, so no oscillation
//! can hide in the disagreement between the two: they move together at every stable state. The arity
//! clause (`> 1`) in the fold guard is exactly the boundary between "the same coordinate seen through a
//! wire" (arity 1, always safe to collapse) and "two coordinates that can disagree" (arity `> 1`, which
//! may hold emergent memory).
//!
//! # Proof obligations
//!
//! **(I1) alias / arity-1 fold soundness.** A bare ±var alias `s = ±var(t)` carries exactly one bit and
//! is in lockstep with `t`, so it always folds: the old wire-collapse is now simply the **arity-1 case**
//! of the guarded fold, composed away like any other relay. When the alias is an **output** whose target
//! `t` is a *surviving internal*, the same fold lands the coordinate on the output rather than composing
//! the output away: `t`'s definition is folded into `s`, `s` is kept as the coordinate, `t` is purged,
//! every `t` reference is rewritten `t ↦ ±var(s)`, and `t`'s definer is transferred to `s` with the
//! parity carried through — so the pin survives holding the coordinate `t` used to name (the sign is
//! incidental, not a special inversion step). A bare ±alias **ring** is no longer refused: it collapses
//! onto a single self-holding coordinate
//! (`a="b", b="a"` → `b = var(b)`; `a="!b", b="a"` → `b = !var(b)`, a one-node oscillator), preserving
//! the one bit — and any oscillation — the ring carried on that surviving coordinate, exactly as a
//! self-holding `ROSC` register does.
//!
//! **(I2) arity-aware guard soundness.** At any stable state, stability forces `s = δ_s(state)` with
//! `s ∉ support(δ_s)`, so `s` is combinational — its value is fixed by the inputs and the other
//! coordinates — and the reduced machine's stable states are exactly the projections of the original's,
//! with `s` recoverable as `δ_s`. The fold must not **fabricate a register**. The guard is three clauses,
//! refusing the fold of `s` (all-or-nothing) exactly when `arity(δ_s) > 1` **and** some consumer
//! `c ∈ vars(δ_s)` **does not already self-hold**: then the fold invents a self-loop for `c` and projects
//! an oscillation that lived in the *disagreement* of two non-self-holding nodes onto a single-node
//! fixpoint. Mutex (`δ_Qa = {Qb, A}`, arity 2, `Qb` not self-holding): folding `Qa` gives a stable
//! `δ_Qb` at `A=B=1`, collapsing the `(0,0) ↔ (1,1)` oscillation
//! [`machine::settle_or_cycle`](super::machine) reads — refused. `ROSC`'s `Q` already self-holds, so
//! folding the relay `X` re-expresses an existing register rather than inventing one; the oscillation
//! survives in `Q`'s own self-loop (`δ_Q = !Q` at `A·!B`) — allowed. Only a *new* self-reference is
//! forbidden, and only a multi-input (arity `> 1`) relay can fabricate one.
//!
//! **(I3) fixpoint invariant.** At termination neither pass commits, so every surviving signal's
//! signal-name support is a subset of the primary inputs plus the self-reaching signals: any consumed
//! non-self-holding signal is a fold candidate, and a refusal implies a 2-cycle whose members self-reach.
//! Any two signals with an identical **recurrent** δ would already have been deduped onto a self-holding
//! rep; combinational duplicates are left as independent full-function signals — each already lies within
//! inputs plus self-reaching signals, so no alias to a non-state rep is ever emitted. The machine
//! evaluates every signal over the inputs plus the self-reaching signals only, so an alias's target must
//! itself be a state variable — which the recurrence condition guarantees. `resolve::state_variables`
//! therefore counts exactly the genuine coordinates and the machine's δ is a direct map lookup.
//!
//! **(I4) termination.** Every fold commit purges an internal or removes `s` from a support (`s` re-enters
//! a support only via a demotion to `±var(rep)`, bounded by the output count). Every dedup commit purges
//! an **internal** (the map strictly shrinks) or aliases an **output** duplicate to `var(rep)` —
//! terminally: the demotion is idempotent under the `!=` change-check, so a demoted output never
//! re-commits, and the renamed-away member never re-enters any support — folding substitutes `var(rep)`
//! for the member, never the member's own name — so no dedup group can re-form on it. Both measures
//! are bounded, and the outer loop's `2 * order.len() + 2` `debug_assert` backstops against a runaway.
//!
//! **(I5) dedup soundness.** If `δ_a == δ_b` as BDDs, then `a` and `b` are computed by the identical
//! function and take equal values at *every* stable state — lockstep, the I1 wire generalised to any
//! shared function. Merging is sound as a coordinate rename in general, but recurrence now licenses only
//! the **output**-aliasing half of the merge: **internal** retirement is unconditional (an internal
//! never has to keep naming a state variable on its own), while a duplicate **output** demotes to
//! `var(rep)` only when the group is *recurrent* — read from the rep's **current** δ at commit time, not
//! the grouping-time snapshot, since an earlier same-pass group's rewrite can only *remove* references to
//! this group's members, never add one. When recurrent, the renamed-away member never re-enters any
//! support — folding substitutes `var(rep)` for the member, never the member's own name — so no dedup
//! group can re-form on it, and the demotion is idempotent under the `!=` change-check (I4). A
//! non-recurrent group with no internal member commits nothing, leaving the duplicate outputs as
//! independent full-function signals — the behaviour-preserving baseline. A consumer that
//! transiently references a combinational rep (e.g. after an internal in the same group already retired
//! onto it) is resolved before the outer loop's fixpoint: either the same-round fold composes the
//! reference away, or a refusal forms an `s ↔ c` 2-cycle that forces both members to self-reach — so I3
//! holds at the fixpoint either way. Genuine independent memories never collide: a real register
//! self-holds on its **own** variable, so two distinct registers have distinct δ, and two mutex grants
//! differ (`!Qb·A ≠ !Qa·B`).
//!
//! Output/state separation (no output `function:` naming another output pin) is a Liberty-only
//! limitation handled at emission time — see `src/emit/liberty.rs`.
//!
//! # Dedup × fold interaction
//!
//! Dedup can demote one of two identical-δ **output** pins to `var(rep)`, deliberately sharing one
//! coordinate across two pins — but only through the demotion gate, i.e. only when the group is
//! recurrent (I5), so an aliased output only ever aliases a **self-reaching** rep, and the fold skips
//! self-holding candidates: a dedup output-alias is never a fold candidate and can never be re-expanded.
//! No exclusion is needed. Internal retirement carries no such gate: an internal-purge rewrite can rename
//! a consumer's reference onto the rep mid-pass, handing the fold a fresh relay candidate the very same
//! round. Conversely, an output that is a bare ±alias of a surviving internal is just the **arity-1**
//! case of the fold: the substitution keeps the coordinate on the pin (`t` must be internal, I1) and
//! folds the alias away.
//!
//! # Known limit
//!
//! The guard inspects only `s ↔ c` **2-cycles** as a structural proxy for "removing `s` preserves the
//! reachable-state cycle structure". Arity-1 links no longer sit inside this limit — they collapse
//! soundly onto a single coordinate (I1). The residual gap is only an *emergent* all-relay ring whose
//! links are **all** arity `> 1` and no node self-holds: a fold can fire before any 2-cycle forms,
//! shrinking a would-be oscillation group. No committed or mandated cell is affected — MUT and SR are
//! 2-cycles the guard catches, and ICM's folded relays feed synchroniser latches that already self-hold.
//! For an ironclad criterion the fold would carry a BDD check that the projected cycle structure
//! survives; the structural guard is accepted per the decided enforcement level.

use std::collections::{BTreeMap, BTreeSet};

use espresso_logic::bdd::{Bdd, Brand, ManagerCell};
use espresso_logic::Symbol;

/// The outcome of [`minimise_state_space`]: the internal signals that were folded away, and the
/// surviving signals whose function was rewritten (so their display expression must be regenerated).
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Minimised {
    /// Signals removed from the map entirely (dead or relay/alias internals). Outputs are never purged.
    pub purged: BTreeSet<Symbol>,
    /// Surviving signals whose BDD differs from the originally parsed one.
    pub changed: BTreeSet<Symbol>,
}

/// `Some((t, parity))` iff `f` is a bare ±alias of another surviving key.
///
/// `parity` is `0` when `f == var(t)` and `1` when `f == !var(t)`. Serves [`fold_pass`]'s arity-1
/// substitution decision (folding the coordinate onto an output alias): `!var(x)` is just an arity-1
/// function like `var(x)`, so a bare ±alias always collapses.
fn alias_target<B: Brand, C: ManagerCell>(
    name: &Symbol,
    f: &Bdd<B, C>,
    bdds: &BTreeMap<Symbol, Bdd<B, C>>,
) -> Option<(Symbol, u8)> {
    let vars: Vec<Symbol> = f.variables().collect();
    if vars.len() == 1 && vars[0] != *name && bdds.contains_key(&vars[0]) {
        let t = vars[0].clone();
        let b = f.builder();
        let parity = if *f == b.var(t.as_str()) { 0 } else { 1 };
        Some((t, parity))
    } else {
        None
    }
}

/// Reduce `bdds` to a minimal set of genuine-memory coordinates, mutating it in place.
///
/// `order` is the `signals()` order (outputs then internals, as parsed) and `outputs` is the set of
/// external-output names; both drive the scan and the alias-representative choice. The returned
/// [`Minimised`] names the purged internals and the surviving signals whose function changed.
///
/// The dedup/fold fixpoint (see (I4) above; concept in `state-space-minimisation.md`) is bounded at
/// `2 * order.len() + 2` outer iterations — a `debug_assert` backstop against a runaway loop, not a
/// behavioural limit reached in practice.
pub fn minimise_state_space<B: Brand, C: ManagerCell>(
    bdds: &mut BTreeMap<Symbol, Bdd<B, C>>,
    order: &[Symbol],
    outputs: &BTreeSet<Symbol>,
) -> Minimised {
    let mut result = Minimised::default();
    let mut iterations = 0usize;
    loop {
        let d = dedup_pass(bdds, order, outputs, &mut result);
        let f = fold_pass(bdds, order, outputs, &mut result);
        iterations += 1;
        debug_assert!(
            iterations <= 2 * order.len() + 2,
            "minimise_state_space: outer loop exceeded the {} iteration bound",
            2 * order.len() + 2
        );
        if !d && !f {
            break;
        }
    }
    // A signal that was rewritten and then purged is gone; keep `changed` to the survivors.
    result.changed.retain(|n| !result.purged.contains(n));
    result
}

/// One dedup pass: collapse every plain-BDD-equal group onto a single coordinate. Returns whether it
/// committed anything.
///
/// Grouping is pure plain-BDD equality — bare ±aliases included, `!var(x)` is not special. Each group
/// keeps one representative (an external output where the group holds one, so a pin is never lost) and
/// rewrites the retired members' consumers onto `var(rep)`. An **internal** duplicate ALWAYS retires:
/// it is purged and its consumers rewritten. A duplicate **output** is never purged — only aliased
/// (demoted to `var(rep)`, pin preserved so it still emits arcs), and only when the group is
/// **recurrent** (the rep's current δ references a group member, so the rep self-holds and the
/// `var(rep)` aliases stay machine-evaluable — I3/I5). A non-recurrent all-output group commits
/// nothing.
///
/// What is LEFT for [`fold_pass`]: signals whose definition must be SUBSTITUTED into consumers and
/// dropped — fold permits such a substitution unless it would create a self-reference, and permits a
/// self-reference-creating one only when the inserted function has support arity 1.
fn dedup_pass<B: Brand, C: ManagerCell>(
    bdds: &mut BTreeMap<Symbol, Bdd<B, C>>,
    order: &[Symbol],
    outputs: &BTreeSet<Symbol>,
    result: &mut Minimised,
) -> bool {
    let mut groups: Vec<(Bdd<B, C>, Vec<Symbol>)> = Vec::new();
    for s in order {
        let Some(f) = bdds.get(s) else { continue };
        match groups.iter_mut().find(|(g, _)| g == f) {
            Some((_, members)) => members.push(s.clone()),
            None => groups.push((f.clone(), vec![s.clone()])),
        }
    }
    let mut progress = false;
    for (_, members) in groups.into_iter().filter(|(_, m)| m.len() >= 2) {
        let rep = members
            .iter()
            .find(|m| outputs.contains(*m))
            .unwrap_or(&members[0])
            .clone();
        // Recurrence is read from the rep's CURRENT map entry at commit time, not the grouping-time
        // snapshot: an earlier group's compose_map in this same pass can only REMOVE references to
        // this group's members (substitution targets are other groups' disjoint reps), so the
        // snapshot can overstate recurrence and wrongly alias an output onto a non-self-holding rep.
        // A recurrent group's rep self-holds after the rename → var(rep), so the aliases stay
        // machine-evaluable (I3); an internal duplicate always retires regardless.
        let recurrent = members
            .iter()
            .any(|m| bdds[&rep].variables().any(|v| v == *m));
        // Internals always retire; an output retires (demoted to var(rep)) ONLY when recurrent. A
        // non-recurrent all-output group (DUP_COMB) yields an empty retired set and commits nothing.
        let retired: Vec<Symbol> = members
            .iter()
            .filter(|m| **m != rep)
            .filter(|m| !outputs.contains(*m) || recurrent)
            .cloned()
            .collect();
        if retired.is_empty() {
            continue;
        }
        let b = bdds[&rep].builder();
        let rep_var = b.var(rep.as_str());
        // Rename only the RETIRED members' consumers — renaming a non-demoted output's consumers
        // would wrongly rewire them onto the rep.
        let rename: BTreeMap<Symbol, Bdd<B, C>> = retired
            .iter()
            .map(|m| (m.clone(), rep_var.clone()))
            .collect();
        let names: Vec<Symbol> = order
            .iter()
            .filter(|n| bdds.contains_key(*n))
            .cloned()
            .collect();
        for s in names {
            let f = bdds[&s].clone();
            let sup: BTreeSet<Symbol> = f.variables().collect();
            let entries: Vec<(&str, &Bdd<B, C>)> = rename
                .iter()
                .filter(|(k, _)| sup.contains(*k))
                .map(|(k, v)| (k.as_str(), v))
                .collect();
            if entries.is_empty() {
                continue;
            }
            let new = f.compose_map(entries);
            if new != f {
                result.changed.insert(s.clone());
                bdds.insert(s, new);
                progress = true;
            }
        }
        for m in &retired {
            if outputs.contains(m) {
                // Duplicate output: demote to var(rep), pin kept — never purged.
                if bdds[m] != rep_var {
                    result.changed.insert(m.clone());
                    bdds.insert(m.clone(), rep_var.clone());
                    progress = true;
                }
            } else {
                // Internal duplicate: purge. The interface is sacred — result.purged ∩ outputs = ∅.
                debug_assert!(!outputs.contains(m), "dedup must never purge an output pin");
                bdds.remove(m);
                result.purged.insert(m.clone());
                progress = true;
            }
        }
    }
    progress
}

/// One arity-aware fold pass. Returns whether it committed anything.
///
/// For each `s` in scan order: first the **coordinate-on-output fold** (an output that is a bare ±alias
/// of an *internal* key folds that key's definer in and purges it, so the coordinate lands on the output
/// pin, breaking the alias 2-cycle the guard would otherwise refuse); then the **guarded relay
/// elimination** — a signal that does not self-hold is composed into its consumers and dropped, unless
/// the fold would fabricate a register.
fn fold_pass<B: Brand, C: ManagerCell>(
    bdds: &mut BTreeMap<Symbol, Bdd<B, C>>,
    order: &[Symbol],
    outputs: &BTreeSet<Symbol>,
    result: &mut Minimised,
) -> bool {
    let mut progress = false;
    for s in order {
        if !bdds.contains_key(s) {
            continue; // already purged
        }
        let f_s = bdds[s].clone();
        let s_is_output = outputs.contains(s);

        // Coordinate-on-output fold (before the self-hold check). An output `s` that is a bare ±alias of
        // an *internal* key `t` is the keeper of that coordinate: fold `t`'s definer into `s`'s equation
        // (re-expressing `t` as ±s, parity-corrected), rewrite it everywhere `t` was referenced, and
        // purge `t`, so the coordinate lands on the output pin. This resolves the `s ↔ t` alias 2-cycle
        // that the register guard below refuses (e.g. C-element `Q = !QN`); the sign just carries through.
        if s_is_output {
            if let Some((t, parity)) = alias_target(s, &f_s, bdds) {
                if !outputs.contains(&t) {
                    let b = f_s.builder();
                    // `t` expressed as ±s.
                    let g = if parity == 0 {
                        b.var(s.as_str())
                    } else {
                        !&b.var(s.as_str())
                    };
                    let mut new_s = bdds[&t].compose(t.as_str(), &g);
                    if parity == 1 {
                        new_s = !&new_s;
                    }
                    if new_s != f_s {
                        result.changed.insert(s.clone());
                    }
                    bdds.insert(s.clone(), new_s);
                    let others: Vec<Symbol> = bdds
                        .keys()
                        .filter(|k| **k != *s && **k != t)
                        .cloned()
                        .collect();
                    for k in others {
                        if bdds[&k].variables().any(|v| v == t) {
                            let nw = bdds[&k].compose(t.as_str(), &g);
                            if nw != bdds[&k] {
                                result.changed.insert(k.clone());
                                bdds.insert(k, nw);
                            }
                        }
                    }
                    bdds.remove(&t);
                    result.purged.insert(t);
                    progress = true;
                    continue;
                }
            }
        }

        if f_s.variables().any(|v| v == *s) {
            continue; // self-holding ⇒ genuine memory, not a relay
        }

        // Consumers: the surviving signals whose function references s (scanned in signals order).
        let consumers: Vec<Symbol> = order
            .iter()
            .filter(|c| c.as_str() != s.as_str() && bdds.contains_key(*c))
            .filter(|c| bdds[*c].variables().any(|v| v.as_str() == s.as_str()))
            .cloned()
            .collect();

        if consumers.is_empty() {
            // A dead internal relay is purged; a dead output (e.g. ICM's GCLK) is a legitimate no-op.
            if !s_is_output {
                bdds.remove(s);
                result.purged.insert(s.clone());
                progress = true;
            }
            continue;
        }

        // Guard: refuse only a fold that would *fabricate* a register. A consumer `c` that forms an
        // `s ↔ c` 2-cycle (`c ∈ support(δ_s)`) yet does **not** already self-hold is emergent memory:
        // folding `s` into it invents a self-loop and projects a multi-node oscillation onto a
        // single-node fixpoint, hiding it (the mutex — `(0,0) ↔ (1,1)` at `A=B=1` collapses to a
        // stable `δ_Qb = Qb`). A consumer that **already self-holds** (e.g. `ROSC`'s `Q = Q·B + X`) is
        // a genuine register; folding the relay into it preserves the dynamics — the oscillation
        // survives in the register's own self-loop (`δ_Q = !Q` at `A·!B`) — so the fold is allowed
        // even though it is a 2-cycle. Only a *new* self-reference is forbidden, and only a multi-input
        // relay can fabricate one: a bare ±alias (arity 1) always collapses.
        let arity = f_s.variables().count();
        if arity > 1
            && consumers
                .iter()
                .any(|c| f_s.variables().any(|v| v == *c) && !bdds[c].variables().any(|v| v == *c))
        {
            continue;
        }

        for c in &consumers {
            let new = bdds[c].compose(s.as_str(), &f_s);
            if arity > 1 {
                debug_assert!(
                    !new.variables().any(|v| v == *c) || bdds[c].variables().any(|v| v == *c),
                    "fold_pass: folding {s:?} introduced a new self-reference for {c:?}"
                );
            }
            result.changed.insert(c.clone());
            bdds.insert(c.clone(), new);
        }
        // The relay itself is dropped (internal) or kept but no longer consumed (output).
        if !s_is_output {
            bdds.remove(s);
            result.purged.insert(s.clone());
        }
        progress = true;
    }
    progress
}

#[cfg(test)]
mod tests {
    use super::*;
    use espresso_logic::bdd_builder;

    /// Build a signal map from `(name, expr)` pairs in a fresh builder, plus the scan order and the
    /// output set. Every name is parsed in the same builder so the handles share a manager.
    macro_rules! system {
        (outputs: [$($out:literal),* $(,)?], $($name:literal = $expr:literal),* $(,)?) => {{
            let b = bdd_builder!();
            let mut bdds: BTreeMap<Symbol, _> = BTreeMap::new();
            let mut order: Vec<Symbol> = Vec::new();
            $(
                let nm = Symbol::from($name);
                bdds.insert(nm.clone(), b.parse($expr).unwrap());
                order.push(nm);
            )*
            let outputs: BTreeSet<Symbol> = [$(Symbol::from($out)),*].into_iter().collect();
            (b, bdds, order, outputs)
        }};
    }

    fn minimise<B: Brand, C: ManagerCell>(
        bdds: &mut BTreeMap<Symbol, Bdd<B, C>>,
        order: &[Symbol],
        outputs: &BTreeSet<Symbol>,
    ) -> Minimised {
        minimise_state_space(bdds, order, outputs)
    }

    #[test]
    fn c_element_chain_collapses_to_single_output_coordinate() {
        // Q → IQ → QN with QN the definer: the three collapse onto the sole output Q.
        let (b, mut bdds, order, outputs) = system! {
            outputs: ["Q"],
            "Q" = "IQ",
            "IQ" = "!QN",
            "QN" = "!(A*B + IQ*(A+B))",
        };
        let min = minimise(&mut bdds, &order, &outputs);
        assert_eq!(
            min.purged,
            ["IQ", "QN"].map(Symbol::from).into_iter().collect()
        );
        assert!(bdds[&Symbol::from("Q")].equivalent_to(&b.parse("A*B + Q*(A+B)").unwrap()));
        assert!(!bdds.contains_key("IQ"));
        assert!(!bdds.contains_key("QN"));
    }

    #[test]
    fn complement_output_pair_keeps_both_pins() {
        // Both Q and QN are outputs; the definer QN self-holds after the fold (Q = !QN substituted in),
        // leaving the non-cyclic output Q = !QN legally naming the cyclic output QN. No hoist runs —
        // output/state separation is now a Liberty-only concern handled at emission time.
        let (b, mut bdds, order, outputs) = system! {
            outputs: ["Q", "QN"],
            "Q" = "!QN",
            "QN" = "!(A*B + Q*(A+B))",
        };
        let min = minimise(&mut bdds, &order, &outputs);
        assert!(min.purged.is_empty());
        assert_eq!(min.changed, [Symbol::from("QN")].into_iter().collect());
        assert!(bdds[&Symbol::from("Q")] == !&b.var("QN"));
        assert!(bdds[&Symbol::from("QN")].equivalent_to(&b.parse("!(A*B + !QN*(A+B))").unwrap()));
    }

    #[test]
    fn mutex_cross_coupling_is_kept() {
        // Qa ↔ Qb is a 2-cycle: the guard refuses both folds; nothing changes.
        let (_b, mut bdds, order, outputs) = system! {
            outputs: ["Qa", "Qb"],
            "Qa" = "!Qb * A",
            "Qb" = "!Qa * B",
        };
        let min = minimise(&mut bdds, &order, &outputs);
        assert!(min.purged.is_empty());
        assert!(min.changed.is_empty());
    }

    #[test]
    fn sr_nor_latch_is_kept() {
        // Cross-coupled NOR: supports have two variables (not wires) and the fold guard trips on the
        // Q↔Qn 2-cycle.
        let (_b, mut bdds, order, outputs) = system! {
            outputs: ["Q", "Qn"],
            "Q" = "!(R+Qn)",
            "Qn" = "!(S+Q)",
        };
        let min = minimise(&mut bdds, &order, &outputs);
        assert!(min.purged.is_empty());
        assert!(min.changed.is_empty());
    }

    #[test]
    fn dff_master_slave_kept() {
        // Master M and slave Q both self-hold, so neither is a relay.
        let (_b, mut bdds, order, outputs) = system! {
            outputs: ["Q"],
            "M" = "!CLK*D + CLK*M",
            "Q" = "CLK*M + !CLK*Q",
        };
        let min = minimise(&mut bdds, &order, &outputs);
        assert!(min.purged.is_empty());
        assert!(min.changed.is_empty());
    }

    #[test]
    fn icm_relays_fold_into_consumers() {
        // The ICM system: sela/selb are combinational relays that fold into sela1/selb1.
        let (b, mut bdds, order, outputs) = system! {
            outputs: ["GCLK"],
            "sela" = "!enB*!S",
            "selb" = "!enA*S",
            "sela1" = "!RA*(!CLKA*sela+CLKA*sela1)",
            "sela2" = "!RA*(CLKA*sela1+!CLKA*sela2)",
            "enA" = "!RA*(!CLKA*sela2+CLKA*enA)",
            "selb1" = "!RB*(!CLKB*selb+CLKB*selb1)",
            "selb2" = "!RB*(CLKB*selb1+!CLKB*selb2)",
            "enB" = "!RB*(!CLKB*selb2+CLKB*enB)",
            "GCLK" = "enA*CLKA+enB*CLKB",
        };
        let min = minimise(&mut bdds, &order, &outputs);
        assert_eq!(
            min.purged,
            ["sela", "selb"].map(Symbol::from).into_iter().collect()
        );
        assert!(bdds[&Symbol::from("sela1")]
            .equivalent_to(&b.parse("!RA*(!CLKA*(!enB*!S)+CLKA*sela1)").unwrap()));
        assert!(bdds[&Symbol::from("selb1")]
            .equivalent_to(&b.parse("!RB*(!CLKB*(!enA*S)+CLKB*selb1)").unwrap()));
    }

    #[test]
    fn relay_into_self_holding_consumer_folds() {
        // ROSC: X="!Q*A", Q="Q*B+X". `Q` already self-holds, so folding the relay `X` into it does not
        // fabricate a register — it only re-expresses an existing one. The guard allows the fold even
        // though X↔Q is a 2-cycle; `X` is purged and `δ_Q = Q*B + !Q*A` (which oscillates at A*!B,
        // preserving the oscillation in Q's own self-loop).
        let (b, mut bdds, order, outputs) = system! {
            outputs: ["Q"],
            "X" = "!Q*A",
            "Q" = "Q*B + X",
        };
        let min = minimise(&mut bdds, &order, &outputs);
        assert_eq!(min.purged, ["X"].map(Symbol::from).into_iter().collect());
        assert!(bdds[&Symbol::from("Q")].equivalent_to(&b.parse("Q*B + !Q*A").unwrap()));
    }

    #[test]
    fn wire_of_input_folds_through() {
        // W="A" is a wire-of-input: its function targets a primary input, not a signal. Y="W" is a bare
        // alias of the key W, so the fold collapses the {Y, W} chain — W (an internal relay) folds into
        // its consumer Y and is purged, and Y resolves to A.
        let (b, mut bdds, order, outputs) = system! {
            outputs: ["Y"],
            "W" = "A",
            "Y" = "W",
        };
        let min = minimise(&mut bdds, &order, &outputs);
        assert!(min.purged.contains("W"));
        assert!(bdds[&Symbol::from("Y")].equivalent_to(&b.parse("A").unwrap()));
    }

    #[test]
    fn all_wire_cycles_collapse_to_single_coordinate() {
        // Notes point-2 resolution: an all-wire cycle is not refused but collapsed onto a single keeper
        // node whose dynamics are preserved — the surviving coordinate holds the one bit the cycle
        // carried (a lone keeper for a=b, a one-node oscillator for a=!b).
        //
        // a="b", b="a": a folds into b (b=b), a is purged, b is the sole keeper.
        let (b, mut bdds, order, outputs) = system! {
            outputs: [],
            "a" = "b",
            "b" = "a",
        };
        let min = minimise(&mut bdds, &order, &outputs);
        assert_eq!(min.purged, [Symbol::from("a")].into_iter().collect());
        assert!(bdds[&Symbol::from("b")] == b.var("b"));

        // a="!b", b="a": a folds into b (b=!b), a is purged, b is a one-node oscillator.
        let (b2, mut bdds2, order2, outputs2) = system! {
            outputs: [],
            "a" = "!b",
            "b" = "a",
        };
        let min2 = minimise(&mut bdds2, &order2, &outputs2);
        assert_eq!(min2.purged, [Symbol::from("a")].into_iter().collect());
        assert!(bdds2[&Symbol::from("b")] == !&b2.var("b"));
    }

    #[test]
    fn dead_combinational_internal_is_purged() {
        // W="CLK*D" with no consumers is a dead internal — the fold purges it.
        let (_b, mut bdds, order, outputs) = system! {
            outputs: [],
            "W" = "CLK*D",
        };
        let min = minimise(&mut bdds, &order, &outputs);
        assert_eq!(min.purged, [Symbol::from("W")].into_iter().collect());
        assert!(!bdds.contains_key("W"));
    }

    #[test]
    fn relay_chain_folds_to_fixpoint() {
        // W1 → W2 → (input B): a relay chain feeding the self-holding output L. Both internals purge.
        let (b, mut bdds, order, outputs) = system! {
            outputs: ["L"],
            "W1" = "W2*A",
            "W2" = "B",
            "L" = "!R*(W1+L)",
        };
        let min = minimise(&mut bdds, &order, &outputs);
        assert_eq!(
            min.purged,
            ["W1", "W2"].map(Symbol::from).into_iter().collect()
        );
        assert!(bdds[&Symbol::from("L")].equivalent_to(&b.parse("!R*(B*A+L)").unwrap()));
    }

    #[test]
    fn minimisation_is_deterministic() {
        // Two independent builder instances must fold each cell to equivalent results. The two runs
        // carry different brands, so we compare the BDDs by their builder-independent `Cover`: rebuild
        // the second run's function in the first run's builder and use `equivalent_to` — a real BDD
        // equivalence, never a stringified cover.
        fn assert_runs_agree<B1: Brand, C1: ManagerCell, B2: Brand, C2: ManagerCell>(
            a: &BTreeMap<Symbol, Bdd<B1, C1>>,
            b: &BTreeMap<Symbol, Bdd<B2, C2>>,
        ) {
            assert!(a.keys().eq(b.keys()));
            for (name, fa) in a {
                let fb = fa.builder().build_cover(&b[name].cover());
                assert!(
                    fa.equivalent_to(&fb),
                    "signal {name} differs between the two folds"
                );
            }
        }

        // C-element.
        let (_b1, mut a, order, outputs) = system! {
            outputs: ["Q"],
            "Q" = "IQ",
            "IQ" = "!QN",
            "QN" = "!(A*B + IQ*(A+B))",
        };
        let (_b2, mut b, _, _) = system! {
            outputs: ["Q"],
            "Q" = "IQ",
            "IQ" = "!QN",
            "QN" = "!(A*B + IQ*(A+B))",
        };
        assert_eq!(
            minimise(&mut a, &order, &outputs),
            minimise(&mut b, &order, &outputs)
        );
        assert_runs_agree(&a, &b);

        // ICM.
        let (_b1, mut a, order, outputs) = system! {
            outputs: ["GCLK"],
            "sela" = "!enB*!S",
            "selb" = "!enA*S",
            "sela1" = "!RA*(!CLKA*sela+CLKA*sela1)",
            "sela2" = "!RA*(CLKA*sela1+!CLKA*sela2)",
            "enA" = "!RA*(!CLKA*sela2+CLKA*enA)",
            "selb1" = "!RB*(!CLKB*selb+CLKB*selb1)",
            "selb2" = "!RB*(CLKB*selb1+!CLKB*selb2)",
            "enB" = "!RB*(!CLKB*selb2+CLKB*enB)",
            "GCLK" = "enA*CLKA+enB*CLKB",
        };
        let (_b2, mut b, _, _) = system! {
            outputs: ["GCLK"],
            "sela" = "!enB*!S",
            "selb" = "!enA*S",
            "sela1" = "!RA*(!CLKA*sela+CLKA*sela1)",
            "sela2" = "!RA*(CLKA*sela1+!CLKA*sela2)",
            "enA" = "!RA*(!CLKA*sela2+CLKA*enA)",
            "selb1" = "!RB*(!CLKB*selb+CLKB*selb1)",
            "selb2" = "!RB*(CLKB*selb1+!CLKB*selb2)",
            "enB" = "!RB*(!CLKB*selb2+CLKB*enB)",
            "GCLK" = "enA*CLKA+enB*CLKB",
        };
        assert_eq!(
            minimise(&mut a, &order, &outputs),
            minimise(&mut b, &order, &outputs)
        );
        assert_runs_agree(&a, &b);

        // Buffered C-element: dedup of the {Q, IQ} duplicate followed by the output-alias fold.
        let (_b1, mut a, order, outputs) = system! {
            outputs: ["Q"],
            "Q" = "!QN",
            "IQ" = "!QN",
            "QN" = "!(A*B + IQ*(A+B))",
        };
        let (_b2, mut b, _, _) = system! {
            outputs: ["Q"],
            "Q" = "!QN",
            "IQ" = "!QN",
            "QN" = "!(A*B + IQ*(A+B))",
        };
        assert_eq!(
            minimise(&mut a, &order, &outputs),
            minimise(&mut b, &order, &outputs)
        );
        assert_runs_agree(&a, &b);
    }

    #[test]
    fn buffered_c_element_dedups_then_folds_to_single_output_coordinate() {
        // Q and IQ both buffer !QN and are plain-BDD-equal: dedup now retires the internal duplicate IQ
        // outright (purged, consumers rewritten onto var(Q)) inside dedup_pass itself. QN then folds
        // through via the fold landing the coordinate on the output alias, so the whole cell reduces to
        // the single output coordinate Q = A*B + Q*(A+B).
        let (b, mut bdds, order, outputs) = system! {
            outputs: ["Q"],
            "Q" = "!QN",
            "IQ" = "!QN",
            "QN" = "!(A*B + IQ*(A+B))",
        };
        let min = minimise(&mut bdds, &order, &outputs);
        assert_eq!(
            min.purged,
            ["IQ", "QN"].map(Symbol::from).into_iter().collect()
        );
        assert!(bdds[&Symbol::from("Q")].equivalent_to(&b.parse("A*B + Q*(A+B)").unwrap()));
        assert!(!bdds.contains_key("IQ"));
        assert!(!bdds.contains_key("QN"));
    }

    #[test]
    fn duplicate_combinational_output_pins_are_left_independent() {
        // Two output pins carry the identical *combinational* function (no member appears in δ=A*B) — a
        // non-recurrent all-output group, so dedup's retire set is empty: aliasing either pin to a
        // combinational rep the machine cannot evaluate would breach I3, so dedup commits nothing and
        // both pins keep the full function and stay independent.
        let (b, mut bdds, order, outputs) = system! {
            outputs: ["Y1", "Y2"],
            "Y1" = "A*B",
            "Y2" = "A*B",
        };
        let min = minimise(&mut bdds, &order, &outputs);
        assert!(min.purged.is_empty());
        assert!(min.changed.is_empty());
        assert!(bdds[&Symbol::from("Y1")].equivalent_to(&b.parse("A*B").unwrap()));
        assert!(bdds[&Symbol::from("Y2")].equivalent_to(&b.parse("A*B").unwrap()));
    }

    #[test]
    fn recurrent_duplicate_outputs_dedup_to_one_coordinate() {
        // Two output pins carry the identical *recurrent* function (the coordinate self-reaches through
        // Q1). Dedup merges Q2 onto var(Q1), making Q1 self-holding — Q2 = var(Q1) legally names the
        // output Q1; no hoist runs (separation is now an emission-time concern).
        let (b, mut bdds, order, outputs) = system! {
            outputs: ["Q1", "Q2"],
            "Q1" = "!R*(S+Q1)",
            "Q2" = "!R*(S+Q1)",
        };
        let min = minimise(&mut bdds, &order, &outputs);
        assert!(min.purged.is_empty());
        assert!(bdds[&Symbol::from("Q2")] == b.var("Q1"));
        assert!(bdds[&Symbol::from("Q1")].equivalent_to(&b.parse("!R*(S+Q1)").unwrap()));
    }

    #[test]
    fn projections_of_cyclic_output_stay_on_pins() {
        // A cyclic output Q (C-element) named by two non-cyclic outputs: Qn = !Q and Qc = Q. Nothing
        // fires: dead-output aliases are left as-is, on the pins.
        let (b, mut bdds, order, outputs) = system! {
            outputs: ["Q", "Qn", "Qc"],
            "Q" = "A*B + Q*(A+B)",
            "Qn" = "!Q",
            "Qc" = "Q",
        };
        let min = minimise(&mut bdds, &order, &outputs);
        assert!(min.purged.is_empty());
        assert!(min.changed.is_empty());
        assert!(bdds[&Symbol::from("Q")].equivalent_to(&b.parse("A*B + Q*(A+B)").unwrap()));
        assert!(bdds[&Symbol::from("Qn")] == !&b.var("Q"));
        assert!(bdds[&Symbol::from("Qc")] == b.var("Q"));
    }

    #[test]
    fn dedup_pass_retires_plain_equal_internal_alias() {
        // Q and IQ are plain-BDD-equal (both !QN): a PASS-LOCAL dedup_pass call retires the internal
        // duplicate IQ outright, during dedup itself — QN's IQ reference is rewritten onto var(Q) before
        // the fold ever runs. The rep's own bare alias (Q = !QN) is left untouched, still the fold's job.
        let (b, mut bdds, order, outputs) = system! {
            outputs: ["Q"],
            "Q" = "!QN",
            "IQ" = "!QN",
            "QN" = "!(A*B + IQ*(A+B))",
        };
        let mut result = Minimised::default();
        let committed = dedup_pass(&mut bdds, &order, &outputs, &mut result);
        assert!(committed);
        assert!(!bdds.contains_key("IQ"));
        assert!(result.purged.contains("IQ"));
        assert_eq!(result.changed, [Symbol::from("QN")].into_iter().collect());
        assert!(bdds[&Symbol::from("Q")] == !&b.var("QN"));
        assert!(bdds[&Symbol::from("QN")].equivalent_to(&b.parse("!(A*B + Q*(A+B))").unwrap()));
    }

    #[test]
    fn internal_cse_pair_dedups_onto_single_survivor() {
        // I1 and I2 are both internal and plain-BDD-equal (A*B): a PASS-LOCAL dedup_pass call retires
        // I2 onto I1 and rewrites L's I2 reference to var(I1), without ever reaching the fold.
        let (b, mut bdds, order, outputs) = system! {
            outputs: ["L"],
            "I1" = "A*B",
            "I2" = "A*B",
            "L" = "!R*(I1+I2+L)",
        };
        let mut result = Minimised::default();
        dedup_pass(&mut bdds, &order, &outputs, &mut result);
        assert!(result.purged.contains("I2"));
        assert!(bdds.contains_key("I1"));
        assert!(!bdds.contains_key("I2"));
        assert!(bdds[&Symbol::from("L")].equivalent_to(&b.parse("!R*(I1+L)").unwrap()));
    }

    #[test]
    fn internal_cse_duplicates_merge_then_fold_into_consumers() {
        // W1 and W2 are internal duplicates (A*B); dedup retires W2 onto W1, then the fold relays the
        // survivor W1 into both its consumers.
        let (b, mut bdds, order, outputs) = system! {
            outputs: ["Z1", "Z2"],
            "W1" = "A*B",
            "W2" = "A*B",
            "Z1" = "W1+C",
            "Z2" = "W2*D",
        };
        let min = minimise(&mut bdds, &order, &outputs);
        assert_eq!(
            min.purged,
            ["W1", "W2"].map(Symbol::from).into_iter().collect()
        );
        assert!(bdds[&Symbol::from("Z1")].equivalent_to(&b.parse("A*B+C").unwrap()));
        assert!(bdds[&Symbol::from("Z2")].equivalent_to(&b.parse("A*B*D").unwrap()));
    }

    #[test]
    fn internal_duplicate_of_combinational_output_retires() {
        // W is an internal duplicate of the combinational output Y (A*B); the internal always retires
        // regardless of recurrence, and the fold carries Y's function into its consumer Z.
        let (b, mut bdds, order, outputs) = system! {
            outputs: ["Y", "Z"],
            "Y" = "A*B",
            "W" = "A*B",
            "Z" = "W+C",
        };
        let min = minimise(&mut bdds, &order, &outputs);
        assert_eq!(min.purged, [Symbol::from("W")].into_iter().collect());
        assert!(!bdds.contains_key("W"));
        assert!(bdds[&Symbol::from("Y")].equivalent_to(&b.parse("A*B").unwrap()));
        assert!(bdds[&Symbol::from("Z")].equivalent_to(&b.parse("A*B+C").unwrap()));
    }

    #[test]
    fn recurrent_internal_duplicate_of_output_merges() {
        // IQ is an internal duplicate of the recurrent output Q (the shared δ references IQ), so dedup
        // merges it onto Q, making Q self-holding on its own name.
        let (b, mut bdds, order, outputs) = system! {
            outputs: ["Q"],
            "Q" = "!R*(S+IQ)",
            "IQ" = "!R*(S+IQ)",
        };
        let min = minimise(&mut bdds, &order, &outputs);
        assert_eq!(min.purged, [Symbol::from("IQ")].into_iter().collect());
        assert!(min.changed.contains("Q"));
        assert!(bdds[&Symbol::from("Q")].equivalent_to(&b.parse("!R*(S+Q)").unwrap()));
    }

    #[test]
    fn mixed_group_retires_internal_but_keeps_duplicate_outputs() {
        // Y1, Y2 and W are all plain-BDD-equal (A*B); the group is non-recurrent, so the internal W
        // still retires unconditionally while the duplicate output Y2 is left un-aliased, independent.
        let (b, mut bdds, order, outputs) = system! {
            outputs: ["Y1", "Y2", "Z"],
            "Y1" = "A*B",
            "Y2" = "A*B",
            "W" = "A*B",
            "Z" = "!W",
        };
        let min = minimise(&mut bdds, &order, &outputs);
        assert_eq!(min.purged, [Symbol::from("W")].into_iter().collect());
        assert!(min.changed.contains("Z"));
        assert!(bdds[&Symbol::from("Y1")].equivalent_to(&b.parse("A*B").unwrap()));
        assert!(bdds[&Symbol::from("Y2")].equivalent_to(&b.parse("A*B").unwrap()));
        assert!(bdds[&Symbol::from("Y2")] != b.var("Y1"));
        assert!(bdds[&Symbol::from("Z")].equivalent_to(&b.parse("!(A*B)").unwrap()));
    }
}
