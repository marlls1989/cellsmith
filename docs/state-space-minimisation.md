# State-space minimisation: the model rewrite

How cellsmith reduces a cell to the **minimal set of genuine-memory state coordinates** before it
builds the state machine. A signal is a *state coordinate* only if it holds real memory; signals that
sit on a feedback cycle but merely relay or alias another signal carry none, and inflate the machine
and its emitted artifacts. This pass rewrites the cell's function model once so that every surviving
signal is either a primary input or a self-reaching coordinate, and the machine's next-state δ is a
direct lookup — no signal is ever substituted through at analysis time.

It reads the same BDD representation the rest of the engine uses; `state-machine-arc-engine.md` and
`state-table-regions.md` cover the passes that consume its output.

## Where it runs and what it operates on

Cell analysis mints one BDD builder for the cell, builds a map from every signal name to its
function's BDD, and rewrites that map *before* the machine pass and the regions cache run. The same
map — now folded — is what feeds both of those passes, so there is a single source of truth for the
cell's functions.

The rewrite records, for later stages, which internal signals it removed (purged — dead, relay, or
alias; outputs are never purged: a folded output keeps its pin and is re-expressed as a function of
its representative) and which surviving signals' functions changed, so their display expression can be
regenerated.

All substitution is exact and canonical, never an approximation. A signal's *signal support* is its
function's referenced variables restricted to the names still in the map — primary inputs are ignored,
since they are never coordinates.

## The outer loop

The rewrite alternates two passes to a fixpoint: alias/complement collapse, then guarded relay
elimination, repeating until neither pass commits anything. Iteration is bounded as a runaway backstop;
in practice a couple of rounds suffice — a signal made foldable only *after* another substitution is
picked up on the next round (e.g. a relay chain `W1 → W2 → input`).

## Alias/complement collapse

This pass recognises signals that are the **same coordinate**: a signal whose function is *exactly*
one other signal, possibly negated.

1. **Find the wires.** A signal is a *wire* iff its function has exactly one variable, that variable is
   another map key (not itself), i.e. `f == var(t)` (parity 0) or `f == !var(t)` (parity 1). Each wire
   contributes an out-edge `name → (target, parity)`.

2. **Walk the wire graph.** Each wire has out-degree one, so following edges accumulates a complement
   parity and terminates in one of two ways:
   - a **definer root** — the first non-wire signal reached (it may itself be a "wire of input" whose
     function names a primary input, which is *not* a wire here). Every node on the walk is recorded
     as a member of that root's class, at its parity relative to the root.
   - a **revisit** — an all-wire cycle (`a="b", b="a"` or `a="!b", b="a"`). This is **refused**: every
     node walked is left untouched this pass. Such a cycle is genuine emergent memory, so it stays a
     coordinate via ordinary self-reachability.

3. **Group into classes** by root; the root itself is a member at parity 0. A class with a single
   member (a lone root) carries no wire and is skipped.

4. **Choose the representative:** the root if it is an external output, else the first output member
   in scan order, else the root. This guarantees an external pin is preserved wherever the class holds
   one.

5. **Collapse onto the representative:**
   - Build a rename map sending every non-rep member to `var(rep)` or `!var(rep)` by its parity
     relative to the representative.
   - The representative's function is the root's own definer with the class members renamed in, then
     complemented iff the representative is the root's complement.
   - Every *other* surviving signal that references a class member is rewritten with the same rename
     map, so all references now point at the representative.
   - Non-rep members are retired: internals are removed and purged; **outputs are demoted** to
     `±var(rep)` — they keep their pin but become a combinational function of the representative, and
     are marked changed.

The mutex is *not* touched by this pass: `Qa = !Qb·A` has a two-variable support, so it is not a wire.

### Worked example — the gate-level C-element

`IQ = !QN`, `QN = !(A·B + IQ·(A+B))`, `Q = IQ`.

`Q` and `IQ` are wires (`Q → IQ` parity 0; `IQ → QN` parity 1); `QN` is the definer root. The class is
`{Q, IQ, QN}`; the representative is `Q` (the only external output). `QN = !Q`, `IQ = Q`, and the
root's definer renamed onto `Q` and complemented gives the single coordinate `δ_Q = A·B + Q·(A+B)`,
with `IQ`/`QN` purged — one bit, exactly the physical cell.

## Guarded relay elimination

This pass removes signals that hold no memory of their own: a **combinational relay** whose value is
fixed by the current inputs and coordinates. It scans candidates in signal order:

1. **Skip self-holding signals:** if `s` appears in the signal support of `δ_s`, `s` is a genuine
   register — not a relay.

2. **Collect consumers** — the surviving signals whose function references `s`, in signals order.

3. **Dead relay:** no consumers → a dead internal is purged; a dead *output* (e.g. ICM's `GCLK`, which
   nothing consumes) is a legitimate no-op and kept.

4. **The guard: refuse the fold only if it would *fabricate* a register** — a consumer `c` that forms
   an `s ↔ c` 2-cycle (`c` appears in the support of `δ_s`) yet does **not already self-hold**. That is
   the emergent-memory signature: `s` and `c` hold no memory individually, so the fold invents a
   self-loop for `c` and projects an oscillation that lived in their *disagreement* onto a single-node
   fixpoint, hiding it. A consumer that already self-holds is a genuine register — folding the relay
   into it is safe (only a *new* self-reference is forbidden).

5. **Fold:** substitute `δ_s` into every consumer, mark them changed, then drop the relay (internal →
   purged; output → kept but no longer consumed). No consumer may gain a new self-reference; the pass
   checks this holds.

### Worked example — the ICM interlock relays

ICM's `sela = !enB·!S` and `selb = !enA·S` are non-self-holding and each feeds a synchroniser latch
that already self-holds (`sela1`, `selb1`). Neither consumer is in the relay's support (no 2-cycle), so
the guard passes and both fold in: `sela1 = !RA·(!CLKA·(!enB·!S) + CLKA·sela1)`.

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

- **(I1) Alias/complement soundness.** A wire chain to a definer root carries exactly one bit — every
  member is `±` the same underlying signal at every stable state. The rename is exact, and all-wire
  cycles are refused, so no oscillator is ever collapsed.
- **(I2) Relay-elimination soundness.** At any stable state a relay satisfies `s = δ_s(state)` with `s`
  absent from the support of `δ_s`, so the reduced machine's stable states are exactly the projections
  of the original's, with `s` recoverable as `δ_s`. The guard refuses only folds that would *fabricate*
  a register — a 2-cycle consumer that does not already self-hold — the sole way a fold can turn an
  oscillation into a stable self-hold; folding a relay into an existing register preserves the
  dynamics.
- **(I3) Fixpoint invariant.** At termination every surviving signal's signal support is a subset of
  the primary inputs plus the self-reaching signals, so state-variable classification identifies
  exactly them and the machine's δ is a direct map lookup.
- **(I4) Termination.** Every commit either purges a signal or demotes an alias output idempotently, so
  the fixpoint is reached within the asserted bound.

The safety boundary is about *behaviour*, not names: a cell's derived arcs, hidden arcs, and the
**existence and condition** of every oscillation group must match the un-reduced cell — a folded relay
leaving a group's membership is not a regression (see the ring-oscillator case above). Gained
constraints are permitted (a relay can have been masking a genuine hazard); losses are not. This is
locked by the behaviour-preservation golden tests.

## Known limits

The guard is a structural proxy for "removing `s` preserves the reachable-state cycle structure", and
it inspects only `s ↔ c` **2-cycles**. A longer *emergent* all-relay loop where no node self-holds — an
odd ring `X1="!X3·A", X2="!X1", X3="!X2"` (no stable states, no committed fixture) — can admit a fold
before any 2-cycle appears, shrinking a would-be oscillation group. No committed or mandated cell is
affected: MUT and SR are 2-cycles the guard catches, and ICM's folded relays feed synchroniser latches
that already self-hold. A fully general criterion would carry a BDD check that the projected cycle
structure survives; the structural guard is accepted per the decided enforcement level.
