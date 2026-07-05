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
//!   composed into each of its consumers via [`Bdd::compose`] and dropped — *unless* the fold would merge
//!   a genuine feedback loop into a self-hold (see the guard below).
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
//! so the reduced machine's stable states are exactly the projections of the original's, with `s`
//! recoverable as `δ_s`. The fold removes only the relay's one parallel-step lag. The guard refuses
//! precisely the folds that merge an `s ↔ c` 2-cycle into `c`'s self-loop — converting a settle-time
//! oscillation into a stable self-hold. Worked examples: the mutex at `A=B=1` oscillates `(Qa,Qb)`
//! `(0,0) ↔ (1,1)` (what [`machine::settle_or_cycle`](super::machine) reads); folding `Qa` would give
//! `δ_Qb = Qb*B + !A*B`, silently dropping the arbitration. `ROSC` (`X="!Q*A"`, `Q="Q*B+X"`): `Q`
//! already self-holds, so the decided "no new self-reference" criterion alone would admit folding `X`,
//! yet the arbitration group would shrink `{Q,X} → {Q}`. The strengthened 2-cycle guard refuses both,
//! and **subsumes** the new-self-reference criterion because `compose` introduces only `support(δ_s)`.
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
//! **Known limit.** An odd all-relay ring (`X1="!X3*A", X2="!X1", X3="!X2"` — no stable states, no
//! committed fixture) admits one fold before the 2-cycle guard bites, shrinking a would-be arbitration
//! group. The M1 analogue is a wire hanging on a self-inverting definer cycle (`R="!W1*A", W1="R"`):
//! M1 collapses the wire (it is not an all-wire cycle — `R` has a two-variable definer), likewise
//! shrinking the `{R, W1}` group. In both cases inversion parity is preserved by `compose`, so the
//! oscillation itself survives and arbitration is still flagged — only the group can shrink. No
//! committed or mandated cell is affected (MUT/ROSC members all have ≥2-variable supports). Documented
//! and accepted per the decided enforcement level.

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

/// The signal names that `f` still references — its variables restricted to the current map keys.
fn signal_support<B: Brand, C: ManagerCell>(
    f: &Bdd<B, C>,
    bdds: &BTreeMap<Symbol, Bdd<B, C>>,
) -> BTreeSet<Symbol> {
    f.variables().filter(|v| bdds.contains_key(v)).collect()
}

/// Reduce `bdds` to a minimal set of genuine-memory coordinates, mutating it in place.
///
/// `order` is the `signals()` order (outputs then internals, as parsed) and `outputs` is the set of
/// external-output names; both drive the scan and the alias-representative choice. The returned
/// [`Minimised`] names the purged internals and the surviving signals whose function changed.
pub fn minimise_state_space<B: Brand, C: ManagerCell>(
    bdds: &mut BTreeMap<Symbol, Bdd<B, C>>,
    order: &[Symbol],
    outputs: &BTreeSet<Symbol>,
) -> Minimised {
    let mut result = Minimised::default();
    let mut iterations = 0usize;
    loop {
        let m1 = m1_pass(bdds, order, outputs, &mut result);
        let m2 = m2_pass(bdds, order, outputs, &mut result);
        iterations += 1;
        debug_assert!(
            iterations <= 2 * order.len() + 2,
            "minimise_state_space: outer loop exceeded the {} iteration bound",
            2 * order.len() + 2
        );
        if !m1 && !m2 {
            break;
        }
    }
    // A signal that was rewritten and then purged is gone; keep `changed` to the survivors.
    result.changed.retain(|n| !result.purged.contains(n));
    result
}

/// One M1 (alias/complement collapse) pass. Returns whether it committed anything.
fn m1_pass<B: Brand, C: ManagerCell>(
    bdds: &mut BTreeMap<Symbol, Bdd<B, C>>,
    order: &[Symbol],
    outputs: &BTreeSet<Symbol>,
    result: &mut Minimised,
) -> bool {
    // A wire is a signal whose function is exactly one *signal* (a map key), possibly complemented.
    let keys: BTreeSet<Symbol> = bdds.keys().cloned().collect();
    let mut wire_edge: BTreeMap<Symbol, (Symbol, u8)> = BTreeMap::new();
    for (name, f) in bdds.iter() {
        let vars: Vec<Symbol> = f.variables().collect();
        if vars.len() == 1 && vars[0] != *name && keys.contains(&vars[0]) {
            let t = vars[0].clone();
            let b = f.builder();
            let parity = if *f == b.var(t.as_str()) { 0 } else { 1 };
            wire_edge.insert(name.clone(), (t, parity));
        }
    }
    if wire_edge.is_empty() {
        return false;
    }

    // Walk the (out-degree-1) wire graph: each wire resolves to a definer root with a parity relative to
    // it, or lands on an all-wire cycle — in which case every node on the walk is refused this pass.
    let mut member_of: BTreeMap<Symbol, (Symbol, u8)> = BTreeMap::new();
    let mut refused: BTreeSet<Symbol> = BTreeSet::new();
    for start in wire_edge.keys() {
        if member_of.contains_key(start) || refused.contains(start) {
            continue;
        }
        // (node, parity accumulated from `start`).
        let mut walked: Vec<(Symbol, u8)> = Vec::new();
        let mut seen: BTreeSet<Symbol> = BTreeSet::new();
        let mut node = start.clone();
        let mut cum = 0u8;
        loop {
            if !seen.insert(node.clone()) {
                // Revisit ⇒ all-wire cycle. Refuse every node walked.
                for (n, _) in &walked {
                    refused.insert(n.clone());
                }
                break;
            }
            walked.push((node.clone(), cum));
            match wire_edge.get(&node) {
                Some((t, e)) => {
                    cum ^= e;
                    node = t.clone();
                }
                None => {
                    // `node` is the definer root; its accumulated parity is the class's reference frame.
                    let root_par = cum;
                    for (n, c) in &walked {
                        member_of
                            .entry(n.clone())
                            .or_insert((node.clone(), root_par ^ c));
                    }
                    break;
                }
            }
        }
    }

    // Group members (root included, at parity 0) by their root.
    let mut classes: BTreeMap<Symbol, Vec<(Symbol, u8)>> = BTreeMap::new();
    for (m, (root, p)) in &member_of {
        classes
            .entry(root.clone())
            .or_default()
            .push((m.clone(), *p));
    }

    let mut progress = false;
    for (root, members) in classes {
        if members.len() < 2 {
            continue; // a lone root carries no wire; nothing to collapse.
        }
        let member_names: BTreeSet<Symbol> = members.iter().map(|(m, _)| m.clone()).collect();

        // Representative: the root if it is an output, else the first output member in scan order, else
        // the root — so an external pin is preserved wherever the class holds one.
        let rep = if outputs.contains(&root) {
            root.clone()
        } else if let Some(o) = order
            .iter()
            .find(|n| member_names.contains(*n) && outputs.contains(*n))
        {
            o.clone()
        } else {
            root.clone()
        };
        let p_rep = members
            .iter()
            .find(|(m, _)| *m == rep)
            .map(|(_, p)| *p)
            .expect("rep is a class member");

        // The representative's variable, positive and negated, in this cell's builder.
        let b = bdds[&root].builder();
        let rep_pos = b.var(rep.as_str());
        let rep_neg = !&rep_pos;

        // Rename map: every non-rep member expressed in terms of the representative.
        let rename: BTreeMap<Symbol, Bdd<B, C>> = members
            .iter()
            .filter(|(m, _)| *m != rep)
            .map(|(m, p)| {
                let g = if *p == p_rep {
                    rep_pos.clone()
                } else {
                    rep_neg.clone()
                };
                (m.clone(), g)
            })
            .collect();

        // δ_rep is the root's definer with the class renamed in, then complemented iff the rep is the
        // root's complement.
        let root_sup: BTreeSet<Symbol> = bdds[&root].variables().collect();
        let entries: Vec<(&str, &Bdd<B, C>)> = rename
            .iter()
            .filter(|(k, _)| root_sup.contains(*k))
            .map(|(k, v)| (k.as_str(), v))
            .collect();
        let mut delta_rep = bdds[&root].compose_map(entries);
        if p_rep == 1 {
            delta_rep = !&delta_rep;
        }
        if delta_rep != bdds[&rep] {
            result.changed.insert(rep.clone());
            progress = true;
        }
        bdds.insert(rep.clone(), delta_rep);

        // Rewrite every other surviving signal that references a class member.
        let others: Vec<Symbol> = bdds
            .keys()
            .filter(|n| !member_names.contains(*n))
            .cloned()
            .collect();
        for s in others {
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

        // Retire the non-rep members: internals vanish, outputs demote to ±var(rep) but keep their pin.
        for (m, p) in &members {
            if *m == rep {
                continue;
            }
            if outputs.contains(m) {
                let want = if (p ^ p_rep) == 0 {
                    rep_pos.clone()
                } else {
                    rep_neg.clone()
                };
                if want != bdds[m] {
                    result.changed.insert(m.clone());
                    bdds.insert(m.clone(), want);
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

/// One M2 (guarded relay elimination) pass. Returns whether it committed anything.
fn m2_pass<B: Brand, C: ManagerCell>(
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
        let sup_s = signal_support(&f_s, bdds);
        if sup_s.contains(s) {
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
            if !outputs.contains(s) {
                bdds.remove(s);
                result.purged.insert(s.clone());
                progress = true;
            }
            continue;
        }

        // Guard: refuse a fold whose consumer appears in s's support — an s↔c 2-cycle, the emergent
        // memory signature. This subsumes "no new self-reference" (compose adds only support(δ_s)).
        if consumers.iter().any(|c| sup_s.contains(c)) {
            continue;
        }

        for c in &consumers {
            let f_c = bdds[c].clone();
            let sup_c_before = signal_support(&f_c, bdds);
            let new = f_c.compose(s.as_str(), &f_s);
            debug_assert!(
                !signal_support(&new, bdds).contains(c) || sup_c_before.contains(c),
                "m2_pass: folding {s:?} introduced a new self-reference for {c:?}"
            );
            result.changed.insert(c.clone());
            bdds.insert(c.clone(), new);
        }
        // The relay itself is dropped (internal) or kept but no longer consumed (output).
        if !outputs.contains(s) {
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
        // Cross-coupled NOR: supports have two variables (not wires) and the M2 guard trips on the
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
    fn relay_cross_coupled_with_self_holding_consumer_is_refused() {
        // ROSC: X="!Q*A", Q="Q*B+X". Q self-holds, so the decided "no new self-reference" guard alone
        // would fold X. The strengthened 2-cycle guard (X consumes Q, Q consumes X) refuses it.
        let (_b, mut bdds, order, outputs) = system! {
            outputs: ["Q"],
            "X" = "!Q*A",
            "Q" = "Q*B + X",
        };
        let min = minimise_state_space(&mut bdds, &order, &outputs);
        assert!(min.purged.is_empty());
        assert!(min.changed.is_empty());
    }

    #[test]
    fn wire_of_input_folds_through() {
        // W="A" is a wire-of-input: its own function targets a primary input, not a signal, so W is not
        // itself an M1 wire. But Y="W" *is* an M1 wire (its target W is a map key), so the whole {Y, W}
        // class — root W included — is collapsed and purged by m1_pass, not m2_pass: it resolves onto
        // the output Y, purging W, and Y resolves to A.
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
    fn all_wire_cycles_are_refused() {
        // a="b", b="a": a pure alias cycle, no definer — left untouched.
        let (_b, mut bdds, order, outputs) = system! {
            outputs: [],
            "a" = "b",
            "b" = "a",
        };
        let min = minimise_state_space(&mut bdds, &order, &outputs);
        assert!(min.purged.is_empty());
        assert!(min.changed.is_empty());

        // a="!b", b="a": a complement cycle, likewise refused.
        let (_b2, mut bdds2, order2, outputs2) = system! {
            outputs: [],
            "a" = "!b",
            "b" = "a",
        };
        let min2 = minimise_state_space(&mut bdds2, &order2, &outputs2);
        assert!(min2.purged.is_empty());
        assert!(min2.changed.is_empty());
    }

    #[test]
    fn dead_combinational_internal_is_purged() {
        // W="CLK*D" with no consumers is a dead internal — M2 purges it.
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
    }
}
