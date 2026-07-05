//! One-shot **state-space minimisation** of a cell's signal model.
//!
//! `resolve::state_variables` classifies a signal as a state coordinate purely by self-reachability in
//! the dependency graph. That over-counts: a signal that lies on a cycle but holds no genuine memory (an
//! interlock relay, an alias/complement of another signal) is flagged as state, inflating the machine and
//! emitting redundant internal nodes. This module rewrites the shared per-cell BDD map **once**, before
//! the machine pass, so that after it every surviving signal is a genuine memory coordinate — a primary
//! input or a self-reaching signal — and the machine's next-state δ is a direct lookup in the map.
//!
//! The rewrite is two staged, structural discriminators run to a fixpoint over the signals in
//! `signals()` order (outputs first, then internals as parsed):
//!
//! * **M1 — alias/complement collapse.** A signal whose function is *exactly* another signal (a map key)
//!   or its negation carries no memory of its own; it is the same coordinate as that signal. A chain of
//!   such wires is walked (tracking the accumulated complement parity) to its **definer root** — the
//!   first non-wire signal reached, which may itself be a wire-of-input (its own function targets a
//!   primary input, not a signal) — and the whole class, root included, collapses onto one
//!   representative — preferring an external output so a pin is never lost. All references are renamed
//!   onto the representative via [`Bdd::compose_map`]. This is M1's job even when the class's root is not
//!   itself a wire (e.g. `W="A"`, `Y="W"`: `Y` is the M1 wire, `W` is only its non-wire root, and both are
//!   retired by this pass — M2 never sees either).
//! * **M2 — guarded relay elimination.** A signal `s` that does not appear in its own support is a
//!   combinational relay: at every stable state `s = δ_s(state)` with `s ∉ support(δ_s)`, so it can be
//!   composed into each of its consumers via [`Bdd::compose`] and dropped — *unless* the fold would
//!   fabricate a register out of emergent memory (see the guard below).
//!
//! # Proof obligations
//!
//! **(I1) M1 soundness.** A wire chain terminating in a definer root carries exactly one bit: each
//! member's stability equation is `m = ±(next)`, so at every stable state all members are determined by
//! the root. The `compose_map` rewrite is exact renaming onto the representative (parity-corrected), and
//! an all-wire cycle (`a="b", b="a"`; `a="!b", b="a"`) is **refused** — every node on it is left
//! untouched — so no oscillator is ever collapsed. The representative's own function is the root's
//! definer with the class renamed in and the parity applied, so its behaviour is unchanged.
//!
//! **(I2) M2 soundness.** At any stable state, stability forces `s = δ_s(state)` and `s ∉ support(δ_s)`,
//! so `s` is combinational — its value is fixed by the inputs and the other coordinates — and the
//! reduced machine's stable states are exactly the projections of the original's, with `s` recoverable
//! as `δ_s`. The fold must not, however, **fabricate a register**. The guard refuses folding `s` into a
//! consumer `c` exactly when `c` forms an `s ↔ c` 2-cycle (`c ∈ support(δ_s)`) *and does not already
//! self-hold* — because then the fold invents a self-loop for `c` and projects an oscillation that
//! lived in the *disagreement* of two non-self-holding nodes onto a single-node fixpoint. Mutex:
//! neither `Qa` nor `Qb` self-holds; folding `Qa` gives `δ_Qb = Qb*B + !A*B`, which at `A=B=1` is
//! `δ_Qb = Qb` — the `(0,0) ↔ (1,1)` oscillation (what [`machine::settle_or_cycle`](super::machine)
//! reads) collapses to two stable states and is lost. Refused. `ROSC` (`X="!Q*A"`, `Q="Q*B+X"`): `Q`
//! **already self-holds**, so folding `X` re-expresses an existing register rather than inventing one;
//! the oscillation survives in `Q`'s own self-loop (`δ_Q = !Q` at `A*!B`) and is still flagged. The
//! fold is allowed — only a *new* self-reference is forbidden. (A folded relay simply leaves the
//! reported oscillation group; it is not a memory coordinate.)
//!
//! **(I3) fixpoint invariant.** At termination, every surviving signal's signal-name support is a subset
//! of the primary inputs plus the self-reaching signals: any consumed non-self-holding signal is an M2
//! candidate, and a refusal implies a 2-cycle whose members self-reach, so `resolve::state_variables`
//! counts them and the machine's δ is a direct map lookup.
//!
//! **(I4) termination.** Every M1 commit purges a member or demotes it to `±var(rep)` (demotion is
//! idempotent under the changed-check, so a re-classified alias output produces no further commit), and
//! every M2 commit purges an internal or removes `s` from every support (`s` re-enters a support only via
//! an M1 demotion, bounded by the output count). The outer-loop `debug_assert` backstops the bound.
//!
//! **Known limit.** The guard is a structural proxy for "removing `s` preserves the reachable-state
//! cycle structure", and it inspects only `s ↔ c` **2-cycles**. A longer *emergent* all-relay loop in
//! which no node self-holds — an odd ring `X1="!X3*A", X2="!X1", X3="!X2"` (no stable states, no
//! committed fixture) — can admit a fold before any 2-cycle appears, shrinking a would-be oscillation
//! group. No committed or mandated cell is affected: MUT and SR are 2-cycles the guard catches, and
//! ICM's folded relays feed synchroniser latches that already self-hold. For an ironclad criterion the
//! fold would carry a BDD check that the projected cycle structure survives; the structural guard is
//! accepted per the decided enforcement level.

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
/// `parity` is `0` when `f == var(t)` and `1` when `f == !var(t)`. Used both to detect the aliases
/// [`dedup_pass`] leaves alone and to drive [`fold_pass`]'s output-alias inversion.
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

/// One dedup pass: collapse signals that share the *same* function onto a single coordinate. Returns
/// whether it committed anything.
///
/// Bare aliases are skipped (they are [`fold_pass`]'s job) so the two passes cannot fight over them.
/// Each group of duplicate functions keeps one representative — an external output where the group holds
/// one, so a pin is never lost — renames the others onto `var(rep)` everywhere, and retires them
/// (internals purged, outputs demoted to `var(rep)`).
fn dedup_pass<B: Brand, C: ManagerCell>(
    bdds: &mut BTreeMap<Symbol, Bdd<B, C>>,
    order: &[Symbol],
    outputs: &BTreeSet<Symbol>,
    result: &mut Minimised,
) -> bool {
    let mut groups: Vec<(Bdd<B, C>, Vec<Symbol>)> = Vec::new();
    for s in order {
        let Some(f) = bdds.get(s) else { continue };
        if alias_target(s, f, bdds).is_some() {
            continue;
        }
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
        let b = bdds[&rep].builder();
        let rep_var = b.var(rep.as_str());
        let rename: BTreeMap<Symbol, Bdd<B, C>> = members
            .iter()
            .filter(|m| **m != rep)
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
        for m in &members {
            if *m == rep {
                continue;
            }
            if outputs.contains(m) {
                if bdds[m] != rep_var {
                    result.changed.insert(m.clone());
                    bdds.insert(m.clone(), rep_var.clone());
                    progress = true;
                }
            } else {
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
/// For each `s` in scan order: first an **output-alias inversion** (an output that is a bare ±alias of
/// an *internal* key absorbs that key's definer and purges it, breaking the alias 2-cycle the guard
/// would otherwise refuse); then the **guarded relay elimination** — a signal that does not self-hold
/// is composed into its consumers and dropped, unless the fold would fabricate a register.
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

        // Output-alias inversion (before the self-hold check). An output `s` that is a bare ±alias of an
        // *internal* key `t` is the keeper of that coordinate: re-express `t`'s definer in terms of `s`
        // (parity-corrected), fold it everywhere `t` was referenced, and purge `t`. This resolves the
        // `s ↔ t` alias 2-cycle that the register guard below refuses (e.g. C-element `Q = !QN`).
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
        // An output→output alias `c = ±var(s)` is left alone: [`dedup_pass`] created it deliberately to
        // share a coordinate across two pins, so folding `s`'s function back into it would re-expand the
        // alias and oscillate against dedup. Inversion never resolves such a pair (both are outputs).
        let consumers: Vec<Symbol> = order
            .iter()
            .filter(|c| c.as_str() != s.as_str() && bdds.contains_key(*c))
            .filter(|c| bdds[*c].variables().any(|v| v.as_str() == s.as_str()))
            .filter(|c| {
                !(s_is_output
                    && outputs.contains(*c)
                    && alias_target(c, &bdds[*c], bdds).is_some_and(|(t, _)| t == *s))
            })
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

    #[test]
    fn c_element_chain_collapses_to_single_output_coordinate() {
        // Q → IQ → QN with QN the definer: the three collapse onto the sole output Q.
        let (b, mut bdds, order, outputs) = system! {
            outputs: ["Q"],
            "Q" = "IQ",
            "IQ" = "!QN",
            "QN" = "!(A*B + IQ*(A+B))",
        };
        let min = minimise_state_space(&mut bdds, &order, &outputs);
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
        // Both Q and QN are outputs; the definer QN is itself an output, so it is the representative and
        // nothing is purged — Q simply demotes to !QN.
        let (b, mut bdds, order, outputs) = system! {
            outputs: ["Q", "QN"],
            "Q" = "!QN",
            "QN" = "!(A*B + Q*(A+B))",
        };
        let min = minimise_state_space(&mut bdds, &order, &outputs);
        assert!(min.purged.is_empty());
        assert!(bdds[&Symbol::from("Q")] == !&b.var("QN"));
        // QN self-holds and equals the QN-based delta (Q = !QN substituted into its definer).
        assert!(bdds[&Symbol::from("QN")]
            .variables()
            .any(|v| v.as_str() == "QN"));
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
        let min = minimise_state_space(&mut bdds, &order, &outputs);
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
        let min = minimise_state_space(&mut bdds, &order, &outputs);
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
        let min = minimise_state_space(&mut bdds, &order, &outputs);
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
        let min = minimise_state_space(&mut bdds, &order, &outputs);
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
        let min = minimise_state_space(&mut bdds, &order, &outputs);
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
        let min = minimise_state_space(&mut bdds, &order, &outputs);
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
        let min = minimise_state_space(&mut bdds, &order, &outputs);
        assert_eq!(min.purged, [Symbol::from("a")].into_iter().collect());
        assert!(bdds[&Symbol::from("b")] == b.var("b"));

        // a="!b", b="a": a folds into b (b=!b), a is purged, b is a one-node oscillator.
        let (b2, mut bdds2, order2, outputs2) = system! {
            outputs: [],
            "a" = "!b",
            "b" = "a",
        };
        let min2 = minimise_state_space(&mut bdds2, &order2, &outputs2);
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
        let min = minimise_state_space(&mut bdds, &order, &outputs);
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
        let min = minimise_state_space(&mut bdds, &order, &outputs);
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
            minimise_state_space(&mut a, &order, &outputs),
            minimise_state_space(&mut b, &order, &outputs)
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
            minimise_state_space(&mut a, &order, &outputs),
            minimise_state_space(&mut b, &order, &outputs)
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
            minimise_state_space(&mut a, &order, &outputs),
            minimise_state_space(&mut b, &order, &outputs)
        );
        assert_runs_agree(&a, &b);
    }

    #[test]
    fn buffered_c_element_dedups_then_folds_to_single_output_coordinate() {
        // Q and IQ both buffer !QN. IQ (an internal duplicate/alias) is retired and QN folds through, so
        // the whole cell reduces to the single output coordinate Q = A*B + Q*(A+B).
        let (b, mut bdds, order, outputs) = system! {
            outputs: ["Q"],
            "Q" = "!QN",
            "IQ" = "!QN",
            "QN" = "!(A*B + IQ*(A+B))",
        };
        let min = minimise_state_space(&mut bdds, &order, &outputs);
        assert_eq!(
            min.purged,
            ["IQ", "QN"].map(Symbol::from).into_iter().collect()
        );
        assert!(bdds[&Symbol::from("Q")].equivalent_to(&b.parse("A*B + Q*(A+B)").unwrap()));
        assert!(!bdds.contains_key("IQ"));
        assert!(!bdds.contains_key("QN"));
    }

    #[test]
    fn duplicate_output_pins_dedup_to_one_coordinate() {
        // Two output pins carry the identical function: dedup keeps Y1 and points Y2 at it (both pins are
        // retained, sharing one coordinate); nothing is purged and the fold leaves the alias intact.
        let (b, mut bdds, order, outputs) = system! {
            outputs: ["Y1", "Y2"],
            "Y1" = "A*B",
            "Y2" = "A*B",
        };
        let min = minimise_state_space(&mut bdds, &order, &outputs);
        assert!(min.purged.is_empty());
        assert_eq!(min.changed, [Symbol::from("Y2")].into_iter().collect());
        assert!(bdds[&Symbol::from("Y2")] == b.var("Y1"));
        assert!(bdds[&Symbol::from("Y1")].equivalent_to(&b.parse("A*B").unwrap()));
    }
}
