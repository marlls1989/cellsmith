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
//! A single loop, run until neither pass commits, alternates two passes that both honour the caller's
//! [`Preserved`] set — **dedup** (identical-δ merge) then **guarded fold** — over the signals in
//! `signals()` order (outputs first, then internals as parsed):
//! `loop { dedup_pass; fold_pass; if neither committed break }`.
//! [`Preserved`] carries the two roles the passes read apart: the cell's external **output** pins, and
//! the wider **preserved** set no pass may purge — the outputs plus any internal the caller marks
//! *exposed*, a node that must keep its name in the minimised model because something downstream
//! addresses it by name. Every transformation prefers to keep an output pin, then a preserved one; a
//! preserved signal is never purged. With nothing exposed the two roles coincide and every rule below
//! reads exactly as it does for the outputs alone.
//!
//! * **dedup — plain-BDD-equal merge.** Signals whose functions are the *exact same* BDD (bare ±aliases
//!   included — `!var(x)` is not special) are one coordinate seen more than once, and every such group
//!   is merged. A merged group keeps a single representative — an external output where the group holds
//!   one, so a pin is never lost, else a preserved member, so an exposed name is never lost — and
//!   renames the other members onto `var(rep)` everywhere. Every
//!   group's rename is unioned across the whole pass and applied at pass end in one
//!   [`Composer::compose_map`] stream over the surviving functions, sharing a single memo rather than
//!   re-walking the map per group, per signal. A **non-preserved** internal duplicate always retires —
//!   it is purged and its consumers
//!   rewritten onto `var(rep)`. A **preserved** duplicate is never purged — it is only demoted to a bare
//!   `var(rep)` alias, and only when the group is **recurrent** (the rep's current δ references a group
//!   member, so the rep self-holds and the alias stays machine-evaluable — I3/I5); a non-recurrent
//!   all-preserved group commits nothing, leaving the duplicates as independent full-function signals.
//!   What remains for the fold is arity-1-gated substitution: a signal's function is composed into its
//!   consumers and dropped, refused only when doing so would fabricate a register — and permitted
//!   despite that risk when the folded function has support arity 1.
//! * **guarded fold — relay/alias elimination.** A signal `s` that does not appear in its own support is
//!   a combinational relay: at every stable state `s = δ_s(state)` with `s ∉ support(δ_s)`, so it is
//!   composed into all of its consumers at once — one [`Composer::compose`] stream sharing a single
//!   memo — and then dropped, or kept as a consumer-free entry when it is preserved. The composition
//!   itself is refused only when it would
//!   fabricate a register out of emergent memory (the arity-aware guard below). A bare ±alias is the
//!   arity-1 case and always folds; when a bare ±alias `s` is an output whose target is a surviving
//!   non-preserved internal, that internal's definition is folded into `s` and the internal dropped, so
//!   the single coordinate lands on the output pin (its sign carried through the composition —
//!   incidental).
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
//! is in lockstep with `t`, so it always folds — the **arity-1 case** of the guarded fold, composed away
//! like any other relay. When the alias is an **output** whose target `t` is a *surviving non-preserved
//! internal*, the same fold lands the coordinate on the output rather than composing the output away:
//! `t`'s definition is folded into `s`, `s` is kept as the coordinate, `t` is purged, every `t`
//! reference is rewritten `t ↦ ±var(s)`, and `t`'s definer is transferred to `s` with the parity carried
//! through — so the pin survives holding the coordinate `t` used to name (the sign is incidental, not a
//! special inversion step). That landing purges `t`, so it is available only while `t` is not preserved.
//! When `t` **is** preserved — a second output pin, or an exposed internal — the landing is refused and
//! the pair settles in the **complement-pair** shape instead: `s` stays the bare alias, and the plain
//! arity-1 relay fold composes `s` into all its consumers, `t`'s own definer among them, so `t` picks up
//! the self-reference and becomes the self-holding coordinate while `s` survives on its pin as a
//! consumer-free alias of it (C-element with `QN` exposed: `Q = !var(QN)` and `QN = !(A·B + !QN·(A+B))`,
//! the shape a complement output pair reaches). Either way the one bit ends up on exactly one
//! coordinate; the two landings differ only in which name carries it. A bare ±alias **ring** collapses
//! onto a single self-holding coordinate
//! (`a="b", b="a"` → `b = var(b)`; `a="!b", b="a"` → `b = !var(b)`, a one-node oscillator), preserving
//! the one bit — and any oscillation — the ring carried on that surviving coordinate, exactly as a
//! self-holding `ROSC` register does; with a ring member exposed, the fold leaves the two in lockstep
//! and dedup's preserved-first representative choice settles the coordinate on that member.
//!
//! **(I2) arity-aware guard soundness.** At any stable state, stability forces `s = δ_s(state)` with
//! `s ∉ support(δ_s)`, so `s` is combinational — its value is fixed by the inputs and the other
//! coordinates — and the reduced machine's stable states are exactly the projections of the original's,
//! with `s` recoverable as `δ_s`. The fold must not **fabricate a register**. The guard is three clauses,
//! refusing the fold of `s` (all-or-nothing) exactly when `arity(δ_s) > 1` **and** some consumer
//! `c ∈ vars(δ_s)` **does not already self-hold**: then the fold invents a self-loop for `c` and projects
//! an oscillation that lived in the *disagreement* of two non-self-holding nodes onto a single-node
//! stable state. Mutex (`δ_Qa = {Qb, A}`, arity 2, `Qb` not self-holding): folding `Qa` gives a stable
//! `δ_Qb` at `A=B=1`, collapsing the `(0,0) ↔ (1,1)` oscillation
//! [`machine::settle_or_cycle`](super::machine) reads — refused. `ROSC`'s `Q` already self-holds, so
//! folding the relay `X` re-expresses an existing register rather than inventing one; the oscillation
//! survives in `Q`'s own self-loop (`δ_Q = !Q` at `A·!B`) — allowed. Only a *new* self-reference is
//! forbidden, and only a multi-input (arity `> 1`) relay can fabricate one.
//!
//! **(I3) minimised-model support invariant.** At termination neither pass commits, so every surviving
//! signal's signal-name support is a subset of the primary inputs plus the self-reaching signals: any
//! consumed non-self-holding signal is a fold candidate, and a refusal implies a 2-cycle whose members
//! self-reach. Preservation does not weaken that. The fold composes a preserved relay into **all** of
//! its consumers exactly as it does a purgeable one and skips only the removal, so a preserved
//! non-self-reaching signal **survives to the minimised model** with no consumers at all — its name is
//! in no other signal's support — as an entry whose own support already lies within the inputs plus the
//! self-reaching signals. The one way a name re-enters a support is a dedup demotion to `var(rep)`, and that gate
//! fires only on a recurrent group, whose rep self-holds. An exposed internal is therefore either a
//! state variable itself or referenced by nobody, which is what lets the machine's I3 `debug_assert`
//! (`Machine::build` in [`analysis`](super::analysis) — every signal's support within the state set)
//! stand unchanged. Any two signals with an identical **recurrent** δ would already have been deduped
//! onto a self-holding
//! rep; combinational duplicates are left as independent full-function signals — each already lies within
//! inputs plus self-reaching signals, so no alias to a non-state rep is ever emitted. The machine
//! evaluates every signal over the inputs plus the self-reaching signals only, so an alias's target must
//! itself be a state variable — which the recurrence condition guarantees. `resolve::state_variables`
//! therefore counts exactly the genuine coordinates and the machine's δ is a direct map lookup.
//!
//! **(I4) termination.** The measure that strictly decreases is the triple *(signals in the map;
//! preserved signals not yet demoted to `var(rep)`; signals still named in some other signal's support
//! that the fold could take)*, compared lexicographically — a well-founded order on ℕ³, so no run of
//! commits descends forever. Every fold commit purges a non-preserved signal (first component) or, for a
//! preserved relay that is kept, removes `s` from every support (third component); the names the
//! composition inserts in `s`'s place were already read by `s`, so none of them gains a consumer it
//! lacked, and `s` re-enters a support only via a demotion to `±var(rep)`, which drops the second
//! component and so is bounded by the preserved count `|outputs| + |exposed|`. Every dedup commit purges
//! a **non-preserved** duplicate (the map strictly shrinks) or aliases a **preserved** duplicate to
//! `var(rep)` — terminally: the demotion is idempotent under the `!=` change-check, so a demoted signal
//! never re-commits, and the renamed-away member never re-enters any support — folding substitutes
//! `var(rep)` for the member, never the member's own name — so no dedup group can re-form on it.
//! Exposing an internal moves its retirement from the purge disjunct to the demotion one. The outer
//! loop's `2 * order.len() + 2` `debug_assert` backstops against a runaway.
//!
//! **(I5) dedup soundness.** If `δ_a == δ_b` as BDDs, then `a` and `b` are computed by the identical
//! function and take equal values at *every* stable state — lockstep, the I1 wire generalised to any
//! shared function. Merging is sound as a coordinate rename in general, but recurrence now licenses only
//! the **preserved**-aliasing half of the merge: **non-preserved** retirement is unconditional (an
//! internal no one addresses by name never has to keep naming a state variable on its own), while a
//! **preserved** duplicate demotes to `var(rep)` only when the group is *recurrent* — read from the
//! rep's **current** δ at commit time, not
//! the grouping-time snapshot, since an earlier same-pass group's rewrite can only *remove* references to
//! this group's members, never add one. When recurrent, the renamed-away member never re-enters any
//! support — folding substitutes `var(rep)` for the member, never the member's own name — so no dedup
//! group can re-form on it, and the demotion is idempotent under the `!=` change-check (I4). A
//! non-recurrent group with no non-preserved member commits nothing, leaving the duplicates as
//! independent full-function signals — the behaviour-preserving baseline. The two roles are read apart:
//! the demotion gate asks `is_preserved` (may this name go?), the representative preference asks
//! `is_output` first and `is_preserved` only after (which name should carry the coordinate?), so an
//! exposed internal never outranks a real output pin but does outrank a plain internal. A consumer that
//! transiently references a combinational rep (e.g. after an internal in the same group already retired
//! onto it) is resolved before the outer loop stops: either the same-round fold composes
//! the reference away, or a refusal forms an `s ↔ c` 2-cycle that forces both members to self-reach — so
//! I3 holds in the minimised model either way. Genuine independent memories never collide: a real
//! register self-holds on its **own** variable, so two distinct registers have distinct δ, and two mutex
//! grants differ (`!Qb·A ≠ !Qa·B`).
//!
//! Output/state separation (no output `function:` naming another output pin) is a Liberty-only
//! limitation handled at emission time — see `src/emit/liberty.rs`.
//!
//! # Dedup × fold interaction
//!
//! Dedup can demote one of two identical-δ **preserved** signals to `var(rep)`, deliberately sharing one
//! coordinate across two names — but only through the demotion gate, i.e. only when the group is
//! recurrent (I5), so a demoted signal only ever aliases a **self-reaching** rep, and the fold skips
//! self-holding candidates: a dedup alias is never a fold candidate and can never be re-expanded.
//! No exclusion is needed. Non-preserved retirement carries no such gate: a purge rewrite can rename
//! a consumer's reference onto the rep mid-pass, handing the fold a fresh relay candidate the very same
//! round. Conversely, an output that is a bare ±alias of a surviving internal is just the **arity-1**
//! case of the fold: the substitution keeps the coordinate on the pin (`t` must be a non-preserved
//! internal, I1) and folds the alias away.
//!
//! # Known limit
//!
//! The guard inspects only `s ↔ c` **2-cycles** as a structural proxy for "removing `s` preserves the
//! reachable-state cycle structure". Arity-1 links never sit inside this limit — they collapse
//! soundly onto a single coordinate (I1). The residual gap is only an *emergent* all-relay ring whose
//! links are **all** arity `> 1` and no node self-holds: a fold can fire before any 2-cycle forms,
//! shrinking a would-be oscillation group. No committed or mandated cell is affected — MUT and SR are
//! 2-cycles the guard catches, and ICM's folded relays feed synchroniser latches that already self-hold.
//! For an ironclad criterion the fold would carry a BDD check that the projected cycle structure
//! survives; the structural guard is accepted per the decided enforcement level.

use std::collections::{BTreeMap, BTreeSet};

use espresso_logic::bdd::{Bdd, Brand, Composer, ManagerCell};
use espresso_logic::Symbol;

/// The outcome of [`minimise_state_space`]: the internal signals that were folded away, and the
/// surviving signals whose function was rewritten (so their display expression must be regenerated).
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Minimised {
    /// Signals removed from the map entirely (dead or relay/alias internals). A preserved signal — an
    /// output pin or an exposed internal — is never purged.
    pub(crate) purged: BTreeSet<Symbol>,
    /// Surviving signals whose BDD differs from the originally parsed one.
    pub(crate) changed: BTreeSet<Symbol>,
}

impl Minimised {
    /// The composition of two successive minimisations of the same map: `self`, then `next` run over the
    /// map `self` left behind. The result reports the pair as one rewrite of the map they started from,
    /// which is what [`crate::model::recompute_signal_metadata`] needs to recover a signal's metadata
    /// from the original parse.
    ///
    /// Both fields union, then the closing rule [`minimise_state_space`] applies at its own end applies
    /// again: a signal the first run rewrote and the second purged is gone, so `changed` keeps only the
    /// survivors.
    pub(crate) fn then(mut self, next: Minimised) -> Minimised {
        self.purged.extend(next.purged);
        self.changed.extend(next.changed);
        let purged = &self.purged;
        self.changed.retain(|n| !purged.contains(n));
        self
    }
}

/// The names [`minimise_state_space`] may not remove, in the two roles its passes read apart.
///
/// `outputs` are the cell's external pins. `preserved` is the wider set no pass may purge: the outputs
/// plus any internal the caller marks **exposed** — a node that must keep its name in the minimised
/// model because something downstream addresses it by name. `outputs` is always a subset of
/// `preserved`, and with nothing exposed the two are equal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Preserved {
    outputs: BTreeSet<Symbol>,
    preserved: BTreeSet<Symbol>,
}

impl Preserved {
    /// Preserve the external output pins and nothing else.
    pub fn outputs(outputs: BTreeSet<Symbol>) -> Self {
        Self {
            preserved: outputs.clone(),
            outputs,
        }
    }

    /// Preserve the external output pins plus `exposed`, the internal nodes that must keep their names.
    pub(crate) fn with_exposed(outputs: BTreeSet<Symbol>, exposed: BTreeSet<Symbol>) -> Self {
        let preserved: BTreeSet<Symbol> = outputs.union(&exposed).cloned().collect();
        debug_assert!(
            outputs.is_subset(&preserved),
            "Preserved: the outputs must be a subset of the preserved set"
        );
        Self { outputs, preserved }
    }

    /// Whether `name` is an external output pin — the first choice for a merged group's coordinate.
    pub(crate) fn is_output(&self, name: &Symbol) -> bool {
        self.outputs.contains(name)
    }

    /// Whether `name` must survive the minimisation, as an output pin or as an exposed internal.
    pub(crate) fn is_preserved(&self, name: &Symbol) -> bool {
        self.preserved.contains(name)
    }
}

/// Which of the two bare forms a ±alias took: the target variable itself, or its complement.
enum Parity {
    Direct,
    Inverted,
}

/// A bare ±alias of another surviving key: the key aliased to, and the parity the alias took.
struct Alias {
    target: Symbol,
    parity: Parity,
}

/// `Some(alias)` iff `f` is a bare ±alias of another surviving key.
///
/// Serves [`fold_pass`]'s arity-1 substitution decision (folding the coordinate onto an output alias):
/// `!var(x)` is just an arity-1 function like `var(x)`, so a bare ±alias always collapses.
fn alias_target<B: Brand, C: ManagerCell>(
    name: &Symbol,
    f: &Bdd<B, C>,
    bdds: &BTreeMap<Symbol, Bdd<B, C>>,
) -> Option<Alias> {
    let vars: Vec<Symbol> = f.variables().collect();
    if vars.len() == 1 && vars[0] != *name && bdds.contains_key(&vars[0]) {
        let t = vars[0].clone();
        let b = f.builder();
        let parity = if *f == b.var(t.as_str()) {
            Parity::Direct
        } else {
            Parity::Inverted
        };
        Some(Alias { target: t, parity })
    } else {
        None
    }
}

/// Reduce `bdds` to a minimal set of genuine-memory coordinates, mutating it in place.
///
/// `order` is the `signals()` order (outputs then internals, as parsed) and `p` names what must
/// survive, in both of its roles ([`Preserved`]); together they drive the scan and the
/// alias-representative choice. The returned
/// [`Minimised`] names the purged internals and the surviving signals whose function changed.
///
/// The dedup/fold loop runs until neither pass commits (see (I4) above; concept in
/// `state-space-minimisation.md`) and is bounded at
/// `2 * order.len() + 2` outer iterations — a `debug_assert` backstop against a runaway loop, not a
/// behavioural limit reached in practice.
pub fn minimise_state_space<B: Brand, C: ManagerCell>(
    bdds: &mut BTreeMap<Symbol, Bdd<B, C>>,
    order: &[Symbol],
    p: &Preserved,
) -> Minimised {
    let mut result = Minimised::default();
    let mut iterations = 0usize;
    loop {
        let d = dedup_pass(bdds, order, p, &mut result);
        let f = fold_pass(bdds, order, p, &mut result);
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
/// keeps one representative — an external output where the group holds one, so a pin is never lost,
/// else a preserved member, so an exposed name is never lost — and
/// rewrites the retired members' consumers onto `var(rep)`. A **non-preserved** duplicate ALWAYS
/// retires: it is purged and its consumers rewritten. A **preserved** duplicate is never purged — only
/// aliased (demoted to `var(rep)`, the name kept so a pin still emits arcs), and only when the group is
/// **recurrent** (the rep's current δ references a group member, so the rep self-holds and the
/// `var(rep)` aliases stay machine-evaluable — I3/I5). A non-recurrent all-preserved group commits
/// nothing.
///
/// What is LEFT for [`fold_pass`]: signals whose definition must be SUBSTITUTED into consumers and
/// dropped — fold permits such a substitution unless it would create a self-reference, and permits a
/// self-reference-creating one only when the inserted function has support arity 1.
fn dedup_pass<B: Brand, C: ManagerCell>(
    bdds: &mut BTreeMap<Symbol, Bdd<B, C>>,
    order: &[Symbol],
    p: &Preserved,
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
    // Accumulate every committed group's equation-field edits, then apply them ONCE at pass end. The
    // `var(rep)` rename of the retired members' consumers is unioned into a single simultaneous
    // substitution and pushed through one stream compose ([`Composer::compose_map`]) that shares a
    // single memo across all surviving functions, instead of re-walking the map per group, per signal.
    let mut rename: BTreeMap<Symbol, Bdd<B, C>> = BTreeMap::new();
    let mut demoted: Vec<(Symbol, Bdd<B, C>)> = Vec::new();
    let mut purged: Vec<Symbol> = Vec::new();
    for (_, members) in groups.into_iter().filter(|(_, m)| m.len() >= 2) {
        // The coordinate lands on an output pin where the group holds one; failing that on a preserved
        // member, so an exposed name keeps it; failing that on the first member in scan order. An
        // exposed internal never outranks a real output pin.
        let rep = members
            .iter()
            .find(|m| p.is_output(m))
            .or_else(|| members.iter().find(|m| p.is_preserved(m)))
            .unwrap_or(&members[0])
            .clone();
        // Recurrence reads the grouping-time snapshot — equal here to the commit-time read the former
        // incremental pass took. Groups are DISJOINT and a group's rename substitutes only its own
        // members with its own rep, which can neither add nor remove a reference to another group's
        // members inside that group's rep, so no earlier group's edit can flip this predicate:
        // deferring the whole pass keeps the value the incremental read produced. A recurrent group's
        // rep self-holds after the rename → var(rep), so the aliases stay machine-evaluable (I3); a
        // non-preserved duplicate always retires regardless.
        let recurrent = members
            .iter()
            .any(|m| bdds[&rep].variables().any(|v| v == *m));
        // A non-preserved member always retires; a preserved one retires (demoted to var(rep)) ONLY
        // when recurrent. A non-recurrent all-preserved group (DUP_COMB) yields an empty retired set
        // and commits nothing.
        let retired: Vec<Symbol> = members
            .iter()
            .filter(|m| **m != rep)
            .filter(|m| !p.is_preserved(m) || recurrent)
            .cloned()
            .collect();
        if retired.is_empty() {
            continue;
        }
        let b = bdds[&rep].builder();
        let rep_var = b.var(rep.as_str());
        for m in retired {
            // Rename only the RETIRED members' consumers — renaming a member that was not retired
            // would wrongly rewire its consumers onto the rep. Members are disjoint across groups, so
            // no key collides in the unioned map.
            rename.insert(m.clone(), rep_var.clone());
            if p.is_preserved(&m) {
                // Preserved duplicate: demoted to var(rep) at pass end, name kept — never purged.
                demoted.push((m, rep_var.clone()));
            } else {
                // Plain internal duplicate: purged. The interface and the exposed names are sacred —
                // result.purged ∩ preserved = ∅.
                debug_assert!(
                    !p.is_preserved(&m),
                    "dedup must never purge a preserved signal"
                );
                purged.push(m);
            }
        }
    }

    if rename.is_empty() {
        return false;
    }

    // Purge the retired non-preserved members first, so the rewrite stream excludes them.
    let mut progress = !purged.is_empty();
    for m in &purged {
        bdds.remove(m);
        result.purged.insert(m.clone());
    }

    // Rewrite every surviving consumer of a retired member in one shared-memo stream pass. Functions
    // referencing no retired member are held out (an untouched no-op); demoted signals are held out too
    // — their whole entry is overwritten with var(rep) below, not composed.
    let demoted_names: BTreeSet<&Symbol> = demoted.iter().map(|(m, _)| m).collect();
    let names: Vec<Symbol> = order
        .iter()
        .filter(|n| bdds.contains_key(*n) && !demoted_names.contains(*n))
        .filter(|n| bdds[*n].variables().any(|v| rename.contains_key(&v)))
        .cloned()
        .collect();
    let originals: Vec<Bdd<B, C>> = names.iter().map(|n| bdds[n].clone()).collect();
    let entries: Vec<(&str, Bdd<B, C>)> = rename
        .iter()
        .map(|(k, v)| (k.as_str(), v.clone()))
        .collect();
    let composed: Vec<Bdd<B, C>> = originals.clone().into_iter().compose_map(entries).collect();
    for (name, (orig, new)) in names.iter().zip(originals.into_iter().zip(composed)) {
        if new != orig {
            result.changed.insert(name.clone());
            bdds.insert(name.clone(), new);
            progress = true;
        }
    }

    // Demote each preserved duplicate to var(rep) (name kept). Its entry is still the snapshot original,
    // so the change-check is against the pre-pass function.
    for (m, rep_var) in demoted {
        if bdds[&m] != rep_var {
            result.changed.insert(m.clone());
            bdds.insert(m, rep_var);
            progress = true;
        }
    }
    progress
}

/// One arity-aware fold pass. Returns whether it committed anything.
///
/// For each `s` in scan order: first the **coordinate-on-output fold** (an output that is a bare ±alias
/// of a *non-preserved internal* key folds that key's definer in and purges it, so the coordinate lands
/// on the output pin, breaking the alias 2-cycle the guard would otherwise refuse); then the **guarded
/// relay elimination** — a signal that does not self-hold is composed into its consumers and dropped,
/// unless it is preserved (kept, then consumer-free) or the fold would fabricate a register.
fn fold_pass<B: Brand, C: ManagerCell>(
    bdds: &mut BTreeMap<Symbol, Bdd<B, C>>,
    order: &[Symbol],
    p: &Preserved,
    result: &mut Minimised,
) -> bool {
    let mut progress = false;
    for s in order {
        if !bdds.contains_key(s) {
            continue; // already purged
        }
        let f_s = bdds[s].clone();

        // Coordinate-on-output fold (before the self-hold check). An output `s` that is a bare ±alias of
        // a *non-preserved internal* key `t` is the keeper of that coordinate: fold `t`'s definer into
        // `s`'s equation (re-expressing `t` as ±s, parity-corrected), rewrite it everywhere `t` was
        // referenced, and purge `t`, so the coordinate lands on the output pin. This resolves the
        // `s ↔ t` alias 2-cycle that the register guard below refuses (e.g. C-element `Q = !QN`); the
        // sign just carries through. The landing purges `t`, so a preserved `t` refuses it and the pair
        // settles in the complement-pair shape instead — the arity-1 relay fold below composes `s` into
        // `t`'s definer, leaving `t` self-holding and `s` a consumer-free alias on its pin (I1).
        if p.is_output(s) {
            if let Some(Alias { target: t, parity }) = alias_target(s, &f_s, bdds) {
                if !p.is_preserved(&t) {
                    let b = f_s.builder();
                    // `t` expressed as ±s.
                    let g = match parity {
                        Parity::Direct => b.var(s.as_str()),
                        Parity::Inverted => !&b.var(s.as_str()),
                    };
                    let mut new_s = bdds[&t].compose(t.as_str(), &g);
                    if let Parity::Inverted = parity {
                        new_s = !&new_s;
                    }
                    if new_s != f_s {
                        result.changed.insert(s.clone());
                    }
                    bdds.insert(s.clone(), new_s);
                    // Rewrite every other function that references `t` (`t := g`) in one shared-memo
                    // stream compose — the same substitution across all of them, composed once.
                    let others: Vec<Symbol> = bdds
                        .keys()
                        .filter(|k| **k != *s && **k != t)
                        .filter(|k| bdds[*k].variables().any(|v| v == t))
                        .cloned()
                        .collect();
                    let originals: Vec<Bdd<B, C>> =
                        others.iter().map(|k| bdds[k].clone()).collect();
                    let rewritten: Vec<Bdd<B, C>> = originals
                        .clone()
                        .into_iter()
                        .compose(t.as_str(), g)
                        .collect();
                    for (k, (orig, nw)) in others.iter().zip(originals.into_iter().zip(rewritten)) {
                        if nw != orig {
                            result.changed.insert(k.clone());
                            bdds.insert(k.clone(), nw);
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
            // A dead internal relay is purged; a dead preserved signal (e.g. ICM's GCLK output, or an
            // exposed internal already composed into everything that read it) is a legitimate no-op.
            if !p.is_preserved(s) {
                bdds.remove(s);
                result.purged.insert(s.clone());
                progress = true;
            }
            continue;
        }

        // Guard: refuse only a fold that would *fabricate* a register. A consumer `c` that forms an
        // `s ↔ c` 2-cycle (`c ∈ support(δ_s)`) yet does **not** already self-hold is emergent memory:
        // folding `s` into it invents a self-loop and projects a multi-node oscillation onto a
        // single-node stable state, hiding it (the mutex — `(0,0) ↔ (1,1)` at `A=B=1` collapses to a
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

        // Fold the relay into all its consumers (`s := f_s`) in one shared-memo stream compose — the
        // same substitution across every consumer, composed once.
        let originals: Vec<Bdd<B, C>> = consumers.iter().map(|c| bdds[c].clone()).collect();
        let folded: Vec<Bdd<B, C>> = originals
            .clone()
            .into_iter()
            .compose(s.as_str(), f_s)
            .collect();
        for (c, (orig, new)) in consumers.iter().zip(originals.into_iter().zip(folded)) {
            if arity > 1 {
                debug_assert!(
                    !new.variables().any(|v| v == *c) || orig.variables().any(|v| v == *c),
                    "fold_pass: folding {s:?} introduced a new self-reference for {c:?}"
                );
            }
            result.changed.insert(c.clone());
            bdds.insert(c.clone(), new);
        }
        // The relay itself is dropped, or — when preserved — kept with no consumers left (I3).
        if !p.is_preserved(s) {
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
    use espresso_logic::bdd::BddBuilder;
    use espresso_logic::bdd_builder;

    /// Parse `(name, expr)` definitions into a signal map plus the scan order, all in one builder so
    /// the handles share a manager.
    fn parse_system<B: Brand, C: ManagerCell>(
        b: &BddBuilder<B, C>,
        defs: &[(&str, &str)],
    ) -> (BTreeMap<Symbol, Bdd<B, C>>, Vec<Symbol>) {
        let bdds = defs
            .iter()
            .map(|(n, e)| (Symbol::from(*n), b.parse(e).unwrap()))
            .collect();
        let order = defs.iter().map(|(n, _)| Symbol::from(*n)).collect();
        (bdds, order)
    }

    /// Build a signal map from `(name, expr)` pairs in a fresh builder, plus the scan order and the
    /// [`Preserved`] set — the outputs alone, or the outputs plus an `exposed:` list.
    macro_rules! system {
        (
            outputs: [$($out:literal),* $(,)?],
            exposed: [$($exp:literal),* $(,)?],
            $($name:literal = $expr:literal),* $(,)?
        ) => {{
            let b = bdd_builder!();
            let (bdds, order) = parse_system(&b, &[$(($name, $expr)),*]);
            let outputs: BTreeSet<Symbol> = [$(Symbol::from($out)),*].into_iter().collect();
            let exposed: BTreeSet<Symbol> = [$(Symbol::from($exp)),*].into_iter().collect();
            (b, bdds, order, Preserved::with_exposed(outputs, exposed))
        }};
        (outputs: [$($out:literal),* $(,)?], $($name:literal = $expr:literal),* $(,)?) => {{
            let b = bdd_builder!();
            let (bdds, order) = parse_system(&b, &[$(($name, $expr)),*]);
            let outputs: BTreeSet<Symbol> = [$(Symbol::from($out)),*].into_iter().collect();
            (b, bdds, order, Preserved::outputs(outputs))
        }};
    }

    fn minimise<B: Brand, C: ManagerCell>(
        bdds: &mut BTreeMap<Symbol, Bdd<B, C>>,
        order: &[Symbol],
        p: &Preserved,
    ) -> Minimised {
        minimise_state_space(bdds, order, p)
    }

    #[test]
    fn composing_two_runs_reports_the_pair_as_one_rewrite() {
        // The exposed run followed by the run that releases the exposure is ONE rewrite of the map they
        // started from — the very rewrite a single outputs-only run performs. QN is rewritten by the
        // first and purged by the second, and the closing rule drops it from `changed`, so the caller
        // never asks a purged signal for a regenerated expression.
        let (_b, mut staged, order, exposed) = system! {
            outputs: ["Q"], exposed: ["QN"],
            "Q" = "!QN",
            "QN" = "!(A*B + Q*(A+B))",
        };
        let first = minimise(&mut staged, &order, &exposed);
        assert!(first.purged.is_empty(), "an exposed node is never purged");
        assert_eq!(first.changed, [Symbol::from("QN")].into_iter().collect());

        let released = Preserved::outputs([Symbol::from("Q")].into_iter().collect());
        let composed = first.then(minimise(&mut staged, &order, &released));

        let (_b2, mut direct, direct_order, p) = system! {
            outputs: ["Q"],
            "Q" = "!QN",
            "QN" = "!(A*B + Q*(A+B))",
        };
        assert_eq!(composed, minimise(&mut direct, &direct_order, &p));
        assert!(
            !composed.changed.contains("QN"),
            "a purged signal is no survivor"
        );
    }

    #[test]
    fn c_element_chain_collapses_to_single_output_coordinate() {
        // Q → IQ → QN with QN the definer: the three collapse onto the sole output Q.
        let (b, mut bdds, order, p) = system! {
            outputs: ["Q"],
            "Q" = "IQ",
            "IQ" = "!QN",
            "QN" = "!(A*B + IQ*(A+B))",
        };
        let min = minimise(&mut bdds, &order, &p);
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
        let (b, mut bdds, order, p) = system! {
            outputs: ["Q", "QN"],
            "Q" = "!QN",
            "QN" = "!(A*B + Q*(A+B))",
        };
        let min = minimise(&mut bdds, &order, &p);
        assert!(min.purged.is_empty());
        assert_eq!(min.changed, [Symbol::from("QN")].into_iter().collect());
        assert!(bdds[&Symbol::from("Q")] == !&b.var("QN"));
        assert!(bdds[&Symbol::from("QN")].equivalent_to(&b.parse("!(A*B + !QN*(A+B))").unwrap()));
    }

    #[test]
    fn mutex_cross_coupling_is_kept() {
        // Qa ↔ Qb is a 2-cycle: the guard refuses both folds; nothing changes.
        let (_b, mut bdds, order, p) = system! {
            outputs: ["Qa", "Qb"],
            "Qa" = "!Qb * A",
            "Qb" = "!Qa * B",
        };
        let min = minimise(&mut bdds, &order, &p);
        assert!(min.purged.is_empty());
        assert!(min.changed.is_empty());
    }

    #[test]
    fn sr_nor_latch_is_kept() {
        // Cross-coupled NOR: supports have two variables (not wires) and the fold guard trips on the
        // Q↔Qn 2-cycle.
        let (_b, mut bdds, order, p) = system! {
            outputs: ["Q", "Qn"],
            "Q" = "!(R+Qn)",
            "Qn" = "!(S+Q)",
        };
        let min = minimise(&mut bdds, &order, &p);
        assert!(min.purged.is_empty());
        assert!(min.changed.is_empty());
    }

    #[test]
    fn dff_master_slave_kept() {
        // Master M and slave Q both self-hold, so neither is a relay.
        let (_b, mut bdds, order, p) = system! {
            outputs: ["Q"],
            "M" = "!CLK*D + CLK*M",
            "Q" = "CLK*M + !CLK*Q",
        };
        let min = minimise(&mut bdds, &order, &p);
        assert!(min.purged.is_empty());
        assert!(min.changed.is_empty());
    }

    #[test]
    fn icm_relays_fold_into_consumers() {
        // The ICM system: sela/selb are combinational relays that fold into sela1/selb1.
        let (b, mut bdds, order, p) = system! {
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
        let min = minimise(&mut bdds, &order, &p);
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
        let (b, mut bdds, order, p) = system! {
            outputs: ["Q"],
            "X" = "!Q*A",
            "Q" = "Q*B + X",
        };
        let min = minimise(&mut bdds, &order, &p);
        assert_eq!(min.purged, ["X"].map(Symbol::from).into_iter().collect());
        assert!(bdds[&Symbol::from("Q")].equivalent_to(&b.parse("Q*B + !Q*A").unwrap()));
    }

    #[test]
    fn wire_of_input_folds_through() {
        // W="A" is a wire-of-input: its function targets a primary input, not a signal. Y="W" is a bare
        // alias of the key W, so the fold collapses the {Y, W} chain — W (an internal relay) folds into
        // its consumer Y and is purged, and Y resolves to A.
        let (b, mut bdds, order, p) = system! {
            outputs: ["Y"],
            "W" = "A",
            "Y" = "W",
        };
        let min = minimise(&mut bdds, &order, &p);
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
        let (b, mut bdds, order, p) = system! {
            outputs: [],
            "a" = "b",
            "b" = "a",
        };
        let min = minimise(&mut bdds, &order, &p);
        assert_eq!(min.purged, [Symbol::from("a")].into_iter().collect());
        assert!(bdds[&Symbol::from("b")] == b.var("b"));

        // a="!b", b="a": a folds into b (b=!b), a is purged, b is a one-node oscillator.
        let (b2, mut bdds2, order2, p2) = system! {
            outputs: [],
            "a" = "!b",
            "b" = "a",
        };
        let min2 = minimise(&mut bdds2, &order2, &p2);
        assert_eq!(min2.purged, [Symbol::from("a")].into_iter().collect());
        assert!(bdds2[&Symbol::from("b")] == !&b2.var("b"));
    }

    #[test]
    fn dead_combinational_internal_is_purged() {
        // W="CLK*D" with no consumers is a dead internal — the fold purges it.
        let (_b, mut bdds, order, p) = system! {
            outputs: [],
            "W" = "CLK*D",
        };
        let min = minimise(&mut bdds, &order, &p);
        assert_eq!(min.purged, [Symbol::from("W")].into_iter().collect());
        assert!(!bdds.contains_key("W"));
    }

    #[test]
    fn relay_chain_folds_until_no_pass_commits() {
        // W1 → W2 → (input B): a relay chain feeding the self-holding output L. Both internals purge.
        let (b, mut bdds, order, p) = system! {
            outputs: ["L"],
            "W1" = "W2*A",
            "W2" = "B",
            "L" = "!R*(W1+L)",
        };
        let min = minimise(&mut bdds, &order, &p);
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
        let (_b1, mut a, order, p) = system! {
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
        assert_eq!(minimise(&mut a, &order, &p), minimise(&mut b, &order, &p));
        assert_runs_agree(&a, &b);

        // ICM.
        let (_b1, mut a, order, p) = system! {
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
        assert_eq!(minimise(&mut a, &order, &p), minimise(&mut b, &order, &p));
        assert_runs_agree(&a, &b);

        // Buffered C-element: dedup of the {Q, IQ} duplicate followed by the output-alias fold.
        let (_b1, mut a, order, p) = system! {
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
        assert_eq!(minimise(&mut a, &order, &p), minimise(&mut b, &order, &p));
        assert_runs_agree(&a, &b);
    }

    #[test]
    fn buffered_c_element_dedups_then_folds_to_single_output_coordinate() {
        // Q and IQ both buffer !QN and are plain-BDD-equal: dedup now retires the internal duplicate IQ
        // outright (purged, consumers rewritten onto var(Q)) inside dedup_pass itself. QN then folds
        // through via the fold landing the coordinate on the output alias, so the whole cell reduces to
        // the single output coordinate Q = A*B + Q*(A+B).
        let (b, mut bdds, order, p) = system! {
            outputs: ["Q"],
            "Q" = "!QN",
            "IQ" = "!QN",
            "QN" = "!(A*B + IQ*(A+B))",
        };
        let min = minimise(&mut bdds, &order, &p);
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
        let (b, mut bdds, order, p) = system! {
            outputs: ["Y1", "Y2"],
            "Y1" = "A*B",
            "Y2" = "A*B",
        };
        let min = minimise(&mut bdds, &order, &p);
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
        let (b, mut bdds, order, p) = system! {
            outputs: ["Q1", "Q2"],
            "Q1" = "!R*(S+Q1)",
            "Q2" = "!R*(S+Q1)",
        };
        let min = minimise(&mut bdds, &order, &p);
        assert!(min.purged.is_empty());
        assert!(bdds[&Symbol::from("Q2")] == b.var("Q1"));
        assert!(bdds[&Symbol::from("Q1")].equivalent_to(&b.parse("!R*(S+Q1)").unwrap()));
    }

    #[test]
    fn projections_of_cyclic_output_stay_on_pins() {
        // A cyclic output Q (C-element) named by two non-cyclic outputs: Qn = !Q and Qc = Q. Nothing
        // fires: dead-output aliases are left as-is, on the pins.
        let (b, mut bdds, order, p) = system! {
            outputs: ["Q", "Qn", "Qc"],
            "Q" = "A*B + Q*(A+B)",
            "Qn" = "!Q",
            "Qc" = "Q",
        };
        let min = minimise(&mut bdds, &order, &p);
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
        let (b, mut bdds, order, p) = system! {
            outputs: ["Q"],
            "Q" = "!QN",
            "IQ" = "!QN",
            "QN" = "!(A*B + IQ*(A+B))",
        };
        let mut result = Minimised::default();
        let committed = dedup_pass(&mut bdds, &order, &p, &mut result);
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
        let (b, mut bdds, order, p) = system! {
            outputs: ["L"],
            "I1" = "A*B",
            "I2" = "A*B",
            "L" = "!R*(I1+I2+L)",
        };
        let mut result = Minimised::default();
        dedup_pass(&mut bdds, &order, &p, &mut result);
        assert!(result.purged.contains("I2"));
        assert!(bdds.contains_key("I1"));
        assert!(!bdds.contains_key("I2"));
        assert!(bdds[&Symbol::from("L")].equivalent_to(&b.parse("!R*(I1+L)").unwrap()));
    }

    #[test]
    fn dedup_pass_merges_two_disjoint_groups_in_one_pass() {
        // Two independent plain-BDD-equal internal pairs — {I1,I2}=A*B and {J1,J2}=C+D — retire in a
        // SINGLE dedup_pass. Their renames {I2 → var(I1), J2 → var(J1)} are unioned and applied to every
        // survivor in one end-of-pass stream compose, rewriting Z1's I2 reference and Z2's J2 reference
        // together — the combined-map path a single group could not exercise.
        let (b, mut bdds, order, p) = system! {
            outputs: ["Z1", "Z2"],
            "I1" = "A*B",
            "I2" = "A*B",
            "J1" = "C+D",
            "J2" = "C+D",
            "Z1" = "I2 + X",
            "Z2" = "J2 * Y",
        };
        let mut result = Minimised::default();
        let committed = dedup_pass(&mut bdds, &order, &p, &mut result);
        assert!(committed);
        assert_eq!(
            result.purged,
            ["I2", "J2"].map(Symbol::from).into_iter().collect()
        );
        assert!(bdds.contains_key("I1") && bdds.contains_key("J1"));
        assert!(!bdds.contains_key("I2") && !bdds.contains_key("J2"));
        assert!(bdds[&Symbol::from("Z1")].equivalent_to(&b.parse("I1 + X").unwrap()));
        assert!(bdds[&Symbol::from("Z2")].equivalent_to(&b.parse("J1 * Y").unwrap()));
        assert_eq!(
            result.changed,
            ["Z1", "Z2"].map(Symbol::from).into_iter().collect()
        );
    }

    #[test]
    fn internal_cse_duplicates_merge_then_fold_into_consumers() {
        // W1 and W2 are internal duplicates (A*B); dedup retires W2 onto W1, then the fold relays the
        // survivor W1 into both its consumers.
        let (b, mut bdds, order, p) = system! {
            outputs: ["Z1", "Z2"],
            "W1" = "A*B",
            "W2" = "A*B",
            "Z1" = "W1+C",
            "Z2" = "W2*D",
        };
        let min = minimise(&mut bdds, &order, &p);
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
        let (b, mut bdds, order, p) = system! {
            outputs: ["Y", "Z"],
            "Y" = "A*B",
            "W" = "A*B",
            "Z" = "W+C",
        };
        let min = minimise(&mut bdds, &order, &p);
        assert_eq!(min.purged, [Symbol::from("W")].into_iter().collect());
        assert!(!bdds.contains_key("W"));
        assert!(bdds[&Symbol::from("Y")].equivalent_to(&b.parse("A*B").unwrap()));
        assert!(bdds[&Symbol::from("Z")].equivalent_to(&b.parse("A*B+C").unwrap()));
    }

    #[test]
    fn recurrent_internal_duplicate_of_output_merges() {
        // IQ is an internal duplicate of the recurrent output Q (the shared δ references IQ), so dedup
        // merges it onto Q, making Q self-holding on its own name.
        let (b, mut bdds, order, p) = system! {
            outputs: ["Q"],
            "Q" = "!R*(S+IQ)",
            "IQ" = "!R*(S+IQ)",
        };
        let min = minimise(&mut bdds, &order, &p);
        assert_eq!(min.purged, [Symbol::from("IQ")].into_iter().collect());
        assert!(min.changed.contains("Q"));
        assert!(bdds[&Symbol::from("Q")].equivalent_to(&b.parse("!R*(S+Q)").unwrap()));
    }

    #[test]
    fn mixed_group_retires_internal_but_keeps_duplicate_outputs() {
        // Y1, Y2 and W are all plain-BDD-equal (A*B); the group is non-recurrent, so the internal W
        // still retires unconditionally while the duplicate output Y2 is left un-aliased, independent.
        let (b, mut bdds, order, p) = system! {
            outputs: ["Y1", "Y2", "Z"],
            "Y1" = "A*B",
            "Y2" = "A*B",
            "W" = "A*B",
            "Z" = "!W",
        };
        let min = minimise(&mut bdds, &order, &p);
        assert_eq!(min.purged, [Symbol::from("W")].into_iter().collect());
        assert!(min.changed.contains("Z"));
        assert!(bdds[&Symbol::from("Y1")].equivalent_to(&b.parse("A*B").unwrap()));
        assert!(bdds[&Symbol::from("Y2")].equivalent_to(&b.parse("A*B").unwrap()));
        assert!(bdds[&Symbol::from("Y2")] != b.var("Y1"));
        assert!(bdds[&Symbol::from("Z")].equivalent_to(&b.parse("!(A*B)").unwrap()));
    }

    #[test]
    fn exposed_alias_target_keeps_both_names_in_the_complement_pair_shape() {
        // C-element with the internal QN exposed. The coordinate-on-output landing would purge QN, so
        // it is refused; instead the arity-1 relay fold composes Q = !QN into QN's own definer, which
        // leaves QN self-holding and Q a consumer-free alias on its pin — the same shape a complement
        // *output* pair reaches (I1). Nothing is purged.
        let (b, mut bdds, order, p) = system! {
            outputs: ["Q"],
            exposed: ["QN"],
            "Q" = "!QN",
            "QN" = "!(A*B + Q*(A+B))",
        };
        let min = minimise(&mut bdds, &order, &p);
        assert!(min.purged.is_empty());
        assert_eq!(min.changed, [Symbol::from("QN")].into_iter().collect());
        assert!(bdds[&Symbol::from("Q")] == !&b.var("QN"));
        assert!(bdds[&Symbol::from("QN")].equivalent_to(&b.parse("!(A*B + !QN*(A+B))").unwrap()));
    }

    #[test]
    fn non_recurrent_exposed_duplicate_of_an_output_stays_independent() {
        // W is an exposed internal duplicate of the combinational output Y (A*B). The group is
        // non-recurrent, so aliasing W to a combinational rep the machine cannot evaluate would breach
        // I3 — dedup commits nothing and both names keep the full function.
        let (b, mut bdds, order, p) = system! {
            outputs: ["Y"],
            exposed: ["W"],
            "Y" = "A*B",
            "W" = "A*B",
        };
        let min = minimise(&mut bdds, &order, &p);
        assert!(min.purged.is_empty());
        assert!(min.changed.is_empty());
        assert!(bdds[&Symbol::from("Y")].equivalent_to(&b.parse("A*B").unwrap()));
        assert!(bdds[&Symbol::from("W")].equivalent_to(&b.parse("A*B").unwrap()));
        assert!(bdds[&Symbol::from("W")] != b.var("Y"));
    }

    #[test]
    fn recurrent_exposed_duplicate_demotes_onto_the_output_representative() {
        // IQ is an exposed internal duplicate of the recurrent output Q (the shared δ references Q, so
        // the rep self-holds). The group is recurrent, so IQ retires — by demotion to var(Q), the
        // exposed name kept, rather than by the purge a plain internal would take.
        let (b, mut bdds, order, p) = system! {
            outputs: ["Q"],
            exposed: ["IQ"],
            "Q" = "!R*(S+Q)",
            "IQ" = "!R*(S+Q)",
        };
        let min = minimise(&mut bdds, &order, &p);
        assert!(min.purged.is_empty());
        assert_eq!(min.changed, [Symbol::from("IQ")].into_iter().collect());
        assert!(bdds[&Symbol::from("IQ")] == b.var("Q"));
        assert!(bdds[&Symbol::from("Q")].equivalent_to(&b.parse("!R*(S+Q)").unwrap()));
    }

    #[test]
    fn exposed_relay_folds_into_its_consumers_and_survives() {
        // W is an exposed combinational relay. The fold composes it into every consumer exactly as it
        // would a plain internal and skips only the removal, so W survives to the minimised model with
        // no consumers left — the I3 shape that keeps the machine's support assert intact.
        let (b, mut bdds, order, p) = system! {
            outputs: ["Z"],
            exposed: ["W"],
            "W" = "A*B",
            "Z" = "W+C",
        };
        let min = minimise(&mut bdds, &order, &p);
        assert!(min.purged.is_empty());
        assert!(bdds[&Symbol::from("W")].equivalent_to(&b.parse("A*B").unwrap()));
        assert!(bdds[&Symbol::from("Z")].equivalent_to(&b.parse("A*B+C").unwrap()));
    }

    #[test]
    fn dedup_representative_prefers_output_then_exposed() {
        // An output pin outranks an exposed internal: W, X and Y are plain-BDD-equal (A*B) with X
        // exposed, and the coordinate lands on the output Y — the retiring W's consumer is rewritten
        // onto var(Y), and the exposed X (non-recurrent, so not retired) is left independent.
        let (b, mut bdds, order, p) = system! {
            outputs: ["Y", "Z"],
            exposed: ["X"],
            "W" = "A*B",
            "X" = "A*B",
            "Y" = "A*B",
            "Z" = "!W",
        };
        let mut result = Minimised::default();
        dedup_pass(&mut bdds, &order, &p, &mut result);
        assert_eq!(result.purged, [Symbol::from("W")].into_iter().collect());
        assert!(bdds.contains_key("X"));
        assert!(bdds[&Symbol::from("Z")].equivalent_to(&b.parse("!Y").unwrap()));

        // With no output in the group, the exposed member outranks the plain internal: the same
        // duplicate pair lands the coordinate on X and W retires onto var(X).
        let (b2, mut bdds2, order2, p2) = system! {
            outputs: ["Z"],
            exposed: ["X"],
            "W" = "A*B",
            "X" = "A*B",
            "Z" = "!W",
        };
        let mut result2 = Minimised::default();
        dedup_pass(&mut bdds2, &order2, &p2, &mut result2);
        assert_eq!(result2.purged, [Symbol::from("W")].into_iter().collect());
        assert!(bdds2.contains_key("X"));
        assert!(bdds2[&Symbol::from("Z")].equivalent_to(&b2.parse("!X").unwrap()));
    }

    #[test]
    fn exposed_dead_alias_is_never_purged() {
        // QN names the complement of the self-holding output Q and nothing reads it. A plain internal
        // in that position is purged as dead; exposed, it is kept on its own name.
        let (b, mut bdds, order, p) = system! {
            outputs: ["Q"],
            exposed: ["QN"],
            "Q" = "A*B + Q*(A+B)",
            "QN" = "!Q",
        };
        let min = minimise(&mut bdds, &order, &p);
        assert!(min.purged.is_empty());
        assert!(min.changed.is_empty());
        assert!(bdds[&Symbol::from("QN")] == !&b.var("Q"));

        // The same cell without the exposure: the dead alias is purged.
        let (_b2, mut bdds2, order2, p2) = system! {
            outputs: ["Q"],
            "Q" = "A*B + Q*(A+B)",
            "QN" = "!Q",
        };
        let min2 = minimise(&mut bdds2, &order2, &p2);
        assert_eq!(min2.purged, [Symbol::from("QN")].into_iter().collect());
    }

    /// One cell shape the differential gate below replays: its label, its output pins and its signal
    /// definitions in scan order. The definitions are in `signals()` order — outputs first where the
    /// shape comes from a spec, since that is the order the pipeline hands the passes.
    struct Fixture {
        label: &'static str,
        outputs: &'static [&'static str],
        defs: &'static [(&'static str, &'static str)],
    }

    const DIFFERENTIAL_FIXTURES: &[Fixture] = &[
        Fixture {
            label: "c_element_chain",
            outputs: &["Q"],
            defs: &[("Q", "IQ"), ("IQ", "!QN"), ("QN", "!(A*B + IQ*(A+B))")],
        },
        Fixture {
            label: "buffered_c_element",
            outputs: &["Q"],
            defs: &[("Q", "!QN"), ("IQ", "!QN"), ("QN", "!(A*B + IQ*(A+B))")],
        },
        // `examples/cells.toml`'s C2GATE, in the outputs-then-internals order `signals()` yields.
        Fixture {
            label: "C2GATE",
            outputs: &["Q"],
            defs: &[("Q", "!QN"), ("IQ", "!QN"), ("QN", "!(A*B + IQ*(A+B))")],
        },
        Fixture {
            label: "complement_output_pair",
            outputs: &["Q", "QN"],
            defs: &[("Q", "!QN"), ("QN", "!(A*B + Q*(A+B))")],
        },
        Fixture {
            label: "mutex",
            outputs: &["Qa", "Qb"],
            defs: &[("Qa", "!Qb * A"), ("Qb", "!Qa * B")],
        },
        Fixture {
            label: "sr_nor_latch",
            outputs: &["Q", "Qn"],
            defs: &[("Q", "!(R+Qn)"), ("Qn", "!(S+Q)")],
        },
        Fixture {
            label: "dff_master_slave",
            outputs: &["Q"],
            defs: &[("M", "!CLK*D + CLK*M"), ("Q", "CLK*M + !CLK*Q")],
        },
        Fixture {
            label: "icm",
            outputs: &["GCLK"],
            defs: &[
                ("sela", "!enB*!S"),
                ("selb", "!enA*S"),
                ("sela1", "!RA*(!CLKA*sela+CLKA*sela1)"),
                ("sela2", "!RA*(CLKA*sela1+!CLKA*sela2)"),
                ("enA", "!RA*(!CLKA*sela2+CLKA*enA)"),
                ("selb1", "!RB*(!CLKB*selb+CLKB*selb1)"),
                ("selb2", "!RB*(CLKB*selb1+!CLKB*selb2)"),
                ("enB", "!RB*(!CLKB*selb2+CLKB*enB)"),
                ("GCLK", "enA*CLKA+enB*CLKB"),
            ],
        },
        Fixture {
            label: "relay_chain",
            outputs: &["L"],
            defs: &[("W1", "W2*A"), ("W2", "B"), ("L", "!R*(W1+L)")],
        },
        Fixture {
            label: "rosc",
            outputs: &["Q"],
            defs: &[("X", "!Q*A"), ("Q", "Q*B + X")],
        },
        // `arcs.rs`'s MASKPAIR: two latches, one masked out of the output by S.
        Fixture {
            label: "maskpair",
            outputs: &["Y"],
            defs: &[("L", "E*D + !E*L"), ("K", "C*D + !C*K"), ("Y", "K + S*L")],
        },
        // A gated latch behind a relay: the enable is combinational, the internal latch self-holds
        // and the output is a bare alias of it, so the coordinate travels two hops to reach the pin.
        Fixture {
            label: "latch_behind_enable_relay",
            outputs: &["Q"],
            defs: &[("EN", "G*!CLR"), ("IL", "EN*D + !EN*IL"), ("Q", "IL")],
        },
        // One internal register tapped by two output pins of opposite parity — the coordinate lands
        // on the first pin and the second becomes a bare alias of it.
        Fixture {
            label: "register_with_true_and_complement_taps",
            outputs: &["Q", "QB"],
            defs: &[("IL", "!R*(S+IL)"), ("Q", "IL"), ("QB", "!IL")],
        },
    ];

    #[test]
    fn exposing_a_signal_reaches_the_same_minimised_model_once_it_is_released() {
        // D5: minimising with a wider preserved set and then re-minimising with the outputs alone must
        // land on the result a single outputs-only run reaches. This is a falsification test — that the
        // dedup/fold minimised model is reachable from a partly-minimised start is NOT one of the
        // module's proved obligations, so a failure here is a finding about the design, not about the
        // fixture.
        //
        // Every signal of every shape is exposed in turn. For an internal that is the exposure the
        // feature exists for; for an output the union is a no-op, which replays the outputs-only run
        // against its own minimised model and so keeps the internal-free shapes (mutex, SR, the
        // complement pair) in the gate.
        for Fixture {
            label,
            outputs: pins,
            defs,
        } in DIFFERENTIAL_FIXTURES
        {
            let outputs: BTreeSet<Symbol> = pins.iter().copied().map(Symbol::from).collect();
            let plain = Preserved::outputs(outputs.clone());
            for (name, _) in defs.iter() {
                let exposed = Preserved::with_exposed(
                    outputs.clone(),
                    [Symbol::from(*name)].into_iter().collect(),
                );
                // Both runs share one builder, so their BDDs are directly comparable.
                let b = bdd_builder!();
                let (mut direct, order) = parse_system(&b, defs);
                minimise_state_space(&mut direct, &order, &plain);

                let (mut released, _) = parse_system(&b, defs);
                minimise_state_space(&mut released, &order, &exposed);
                minimise_state_space(&mut released, &order, &plain);

                assert!(
                    direct.keys().eq(released.keys()),
                    "{label}, {name} exposed: survivors {:?} differ from the direct run's {:?}",
                    released.keys().collect::<Vec<_>>(),
                    direct.keys().collect::<Vec<_>>()
                );
                for (n, f) in &direct {
                    assert!(
                        f.equivalent_to(&released[n]),
                        "{label}, {name} exposed: {n} differs from the direct run"
                    );
                }
            }
        }
    }
}
