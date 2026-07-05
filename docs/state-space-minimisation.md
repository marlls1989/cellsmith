# State-space minimisation: the model rewrite

How cellsmith reduces a cell to the **minimal set of genuine-memory state coordinates** before it
builds the state machine. A signal is a *state coordinate* only if it holds real memory; signals that
sit on a feedback cycle but merely relay or alias another signal carry none, and inflate the machine
and its emitted artifacts. This pass rewrites the cell's function model once so that every surviving
signal is either a primary input or a self-reaching coordinate, and the machine's next-state δ is a
direct lookup — no signal is ever substituted through at analysis time.

The code lives in one file:

| File | Role |
|------|------|
| `src/logic/minimise.rs` | `minimise_state_space`: the staged rewrite (module doc `minimise.rs:1-64`) |

It reads the same `Bdd` representation the rest of the engine uses; `state-machine-arc-engine.md` and
`state-table-regions.md` cover the passes that consume its output.

## Where it runs and what it operates on

`Cell::analyse` (`model.rs`) mints **one BDD builder for the cell**, builds a map from every signal
name to its function's BDD, and calls `minimise_state_space` on that map *before* the machine pass and
the regions cache. The same map — now folded — is what feeds `analyse_machine` and `state_regions`, so
there is a single source of truth for the cell's functions.

```rust
pub fn minimise_state_space<B: Brand, C: ManagerCell>(
    bdds: &mut BTreeMap<Symbol, Bdd<B, C>>,   // signal name → its function, mutated in place
    order: &[Symbol],                         // signals() order: outputs first, then internals
    outputs: &BTreeSet<Symbol>,               // the external-output names
) -> Minimised
```

`Minimised` records the outcome (`minimise.rs:71-79`):

- `purged` — internal signals removed from the map (dead, relay, or alias). **Outputs are never
  purged**; a folded output keeps its pin and is re-expressed as a function of its representative.
- `changed` — surviving signals whose BDD differs from the one originally parsed, so their display
  expression must be regenerated. `purged` names are pruned from `changed` at the end
  (`minimise.rs:115`).

All substitution is exact and canonical — `Bdd::compose` / `Bdd::compose_map` only. A signal's
*signal support* is the helper `signal_support` (`minimise.rs:82-87`): `f.variables()` restricted to
the names still in the map (primary inputs are ignored — they are never coordinates).

## The outer loop

`minimise_state_space` alternates two passes to a fixpoint (`minimise.rs:101-113`):

```
loop { changed_m1 = m1_pass();  changed_m2 = m2_pass();  if neither changed, stop }
```

Each pass returns whether it committed anything. Iteration is bounded by a `debug_assert`
(`2 * order.len() + 2`) as a runaway backstop; in practice a couple of rounds suffice (a signal made
foldable only *after* another substitution is picked up on the next round — e.g. a relay chain
`W1 → W2 → input`).

## M1 — alias/complement collapse (`m1_pass`)

M1 recognises signals that are the **same coordinate**: a signal whose function is *exactly* one other
signal, possibly negated.

1. **Find the wires** (`minimise.rs:127-140`). A signal is a *wire* iff its BDD has exactly one
   variable, that variable is another map key (not itself), i.e. `f == var(t)` (parity 0) or
   `f == !var(t)` (parity 1). Each wire contributes an out-edge `name → (target, parity)`.

2. **Walk the wire graph** (`minimise.rs:144-181`). Each wire has out-degree one, so following edges
   accumulates a complement parity and terminates in one of two ways:
   - a **definer root** — the first non-wire signal reached (it may itself be a "wire of input" whose
     function names a primary input, which is *not* a wire here). Every node on the walk is recorded
     as a member of that root's class, at its parity relative to the root.
   - a **revisit** — an all-wire cycle (`a="b", b="a"` or `a="!b", b="a"`). This is **refused**: every
     node walked is left untouched this pass. Such a cycle is genuine emergent memory, so it stays a
     coordinate via ordinary self-reachability.

3. **Group into classes** by root (`minimise.rs:184-190`); the root itself is a member at parity 0. A
   class with a single member (a lone root) carries no wire and is skipped.

4. **Choose the representative** (`minimise.rs:201-215`): the root if it is an external output, else
   the first output member in scan order, else the root. This guarantees an external pin is preserved
   wherever the class holds one.

5. **Collapse onto the representative** (`minimise.rs:217-300`):
   - Build a rename map sending every non-rep member to `var(rep)` or `!var(rep)` by its parity
     relative to the representative.
   - `δ_rep` is the root's own definer with the class members renamed in via `compose_map`, then
     complemented iff the representative is the root's complement (`p_rep == 1`).
   - Every *other* surviving signal that references a class member is rewritten with the same rename
     map, so all references now point at the representative.
   - Non-rep members are retired: internals are removed and added to `purged`; **outputs are demoted**
     to `±var(rep)` — they keep their pin but become a combinational function of the representative,
     and are added to `changed`.

The mutex is *not* touched by M1: `Qa = !Qb·A` has a two-variable support, so it is not a wire.

### Worked example — the gate-level C-element

```
IQ = "!QN"                 QN = "!(A*B + IQ*(A+B))"                Q = "IQ"
```

`Q` and `IQ` are wires (`Q → IQ` parity 0; `IQ → QN` parity 1); `QN` is the definer root. The class is
`{Q, IQ, QN}`; the representative is `Q` (the only external output). `QN = !Q`, `IQ = Q`, and the
root's definer renamed onto `Q` and complemented gives the single coordinate

```
δ_Q = A*B + Q*(A+B)
```

with `IQ`/`QN` purged — one bit, exactly the physical cell.

## M2 — guarded relay elimination (`m2_pass`)

M2 removes signals that hold no memory of their own: a **combinational relay** whose value is fixed by
the current inputs and coordinates. It scans candidates in `order` (`minimise.rs:314-365`):

1. **Skip self-holding signals** (`minimise.rs:320-322`): if `s ∈ signal_support(δ_s)`, `s` is a
   genuine register — not a relay.

2. **Collect consumers** — the surviving signals whose function references `s`, in signals order
   (`minimise.rs:325-330`).

3. **Dead relay** (`minimise.rs:332-340`): no consumers → a dead internal is purged; a dead *output*
   (e.g. ICM's `GCLK`, which nothing consumes) is a legitimate no-op and kept.

4. **The guard** (`minimise.rs:346-358`): **refuse the fold only if it would *fabricate* a register** —
   a consumer `c` that forms an `s ↔ c` 2-cycle (`c ∈ support(δ_s)`) yet does **not already
   self-hold**. That is the emergent-memory signature: `s` and `c` hold no memory individually, so the
   fold invents a self-loop for `c` and projects an oscillation that lived in their *disagreement* onto
   a single-node fixpoint, hiding it. A consumer that already self-holds is a genuine register — folding
   the relay into it is safe (only a *new* self-reference is forbidden).

5. **Fold** (`minimise.rs:348-364`): substitute `δ_s` into every consumer via `Bdd::compose`, mark
   them `changed`, then drop the relay (internal → purged; output → kept but no longer consumed). A
   `debug_assert` confirms no consumer gained a new self-reference.

### Worked example — the ICM interlock relays

ICM's `sela = !enB·!S` and `selb = !enA·S` are non-self-holding and each feeds a synchroniser latch
that already self-holds (`sela1`, `selb1`). Neither consumer is in the relay's support (no 2-cycle), so
the guard passes and both fold in:

```
sela1 = !RA·(!CLKA·(!enB·!S) + CLKA·sela1)
```

`sela`/`selb` are purged; the machine width drops from 13 to 11 coordinates, and `sela1`/`selb1` now
carry `enB`/`enA` and `S` in their statetable columns.

### What the guard keeps — and what it lets fold

- **Mutex** `Qa = !Qb·A`, `Qb = !Qa·B`: neither self-holds, so folding either fabricates a register.
  Folding `Qa` gives `δ_Qb = Qb·B + !A·B`, which at `A=B=1` is `δ_Qb = Qb` — the `(0,0) ↔ (1,1)`
  oscillation collapses to two stable states and is **lost**. Refused; both coordinates kept.
- **SR NOR latch**, **master/slave DFF**: self-holding (or become so on fold) → kept.
- **Ring oscillator** `X = !Q·A`, `Q = Q·B + X`: `Q` **already self-holds**, so folding `X` re-expresses
  an existing register rather than inventing one. `X` folds; `δ_Q = Q·B + !Q·A` still oscillates
  (`δ_Q = !Q` at `A·!B`), so the machine still flags the oscillation — the group is just `{Q}` instead
  of `{Q, X}`. The reported group is the genuine memory coordinates that oscillate, not the relays
  feeding them.

## Why the rewrite is behaviour-preserving

The module doc states the four invariants in full (`minimise.rs:27-64`); in brief:

- **(I1) M1 soundness.** A wire chain to a definer root carries exactly one bit — every member is
  `±` the same underlying signal at every stable state. The rename is exact, and all-wire cycles are
  refused, so no oscillator is ever collapsed.
- **(I2) M2 soundness.** At any stable state a relay satisfies `s = δ_s(state)` with `s ∉ support(δ_s)`,
  so the reduced machine's stable states are exactly the projections of the original's, with `s`
  recoverable as `δ_s`. The guard refuses only folds that would *fabricate* a register — a 2-cycle
  consumer that does not already self-hold — the sole way a fold can turn an oscillation into a stable
  self-hold; folding a relay into an existing register preserves the dynamics.
- **(I3) fixpoint invariant.** At termination every surviving signal's signal support is a subset of
  the primary inputs plus the self-reaching signals, so `state_variables` classifies exactly them and
  the machine's δ is a direct map lookup.
- **(I4) termination.** Every commit either purges a signal or demotes an alias output idempotently, so
  the fixpoint is reached within the asserted bound.

The safety boundary is about *behaviour*, not names: a cell's derived arcs, hidden arcs, and the
**existence and condition** of every oscillating (arbitration) group must match the un-reduced cell — a
folded relay leaving a group's membership is not a regression (see the ROSC case above). Gained
constraints are permitted (a relay can have been masking a genuine hazard); losses are not. This is
locked by the behaviour-preservation golden tests in `tests/golden.rs`.

## Known limits

The guard is a structural proxy for "removing `s` preserves the reachable-state cycle structure", and
it inspects only `s ↔ c` **2-cycles**. A longer *emergent* all-relay loop where no node self-holds — an
odd ring `X1="!X3·A", X2="!X1", X3="!X2"` (no stable states, no committed fixture) — can admit a fold
before any 2-cycle appears, shrinking a would-be oscillation group. No committed or mandated cell is
affected: MUT and SR are 2-cycles the guard catches, and ICM's folded relays feed synchroniser latches
that already self-hold. An ironclad criterion would carry a BDD check that the projected cycle structure
survives; the structural guard is accepted per the decided enforcement level (`minimise.rs:57-64`).
