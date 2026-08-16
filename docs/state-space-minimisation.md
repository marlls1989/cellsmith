# State-space minimisation: the model rewrite

How cellsmith reduces a cell to the **minimal set of genuine-memory state coordinates** before it
builds the state machine. A signal is a *state coordinate* only if it holds real memory; signals that
sit on a feedback cycle but merely relay (defined in "Guarded relay elimination" below), alias, or
duplicate another signal carry none, and inflate the machine and its emitted artifacts. This pass
rewrites the cell's function model once so that every surviving signal is either a primary input or a
self-reaching coordinate, and the machine's next-state δ is a direct lookup — no signal is ever
substituted through at analysis time.

It reads the same BDD representation the rest of the engine uses; `state-machine-arc-engine.md` and
`state-table-regions.md` cover the passes that consume its output.

## Where it runs and what it operates on

Cell analysis mints one BDD builder for the cell, builds a map from every signal name to its
function's BDD, and rewrites that map *before* the machine pass and the regions cache run. The same
map — now minimised — is what feeds both of those passes, so there is a single source of truth for the
cell's functions.

The rewrite records, for later stages, which internal signals it removed (purged — dead, relay,
alias, or duplicate; outputs are never purged: a folded or deduped output keeps its pin and is
re-expressed as a function of its representative) and which surviving signals' functions changed, so
their display expression can be regenerated.

All substitution is exact and canonical, never an approximation. A signal's *signal support* is its
function's referenced variables restricted to the names still in the map — primary inputs are ignored,
since they are never coordinates.

## The outer loop

The rewrite runs a single convergence-point loop of two output-preserving passes: **dedup first, then fold**,
repeating until neither pass commits anything. Both prefer to keep output pins and never purge an
output — a retired output keeps its pin and is re-expressed as a function of its representative.

Iteration is bounded as a runaway backstop; in practice a couple of rounds suffice — a signal made
foldable only *after* another substitution is picked up on the next round (e.g. a relay chain
`W1 → W2 → input`, or a bare-alias chain resolved one link per round).

### Output/state separation is not this pass's job

The convergence point above is behaviour-preserving, not Liberty-spec-preserving: it may legitimately leave the
minimised model with a cyclic output referenced by another output's function (two genuine coordinates,
each an external pin). That shape is exactly what a Liberty `statetable`/`function` cannot express for
an output pin — the spec forbids an output referencing another output. Separating output pins from the
state they depend on is a Liberty-specific emission concern, not a property of the minimised model, so
it is handled entirely at emission time (alias minting in `src/emit/statetable.rs`), never by a
minimisation pass here: a minimisation-time rewrite would mutate the shared model the Verilog emitter
also reads, and a Verilog UDP is free to have an output reference another output directly, so keeping
the cyclic-output shape in the minimised model preserves that distinction for Verilog.

## Identical-δ merge (dedup)

This pass recognises signals that are the **same coordinate** because they compute the *same
transition function*: two signals whose BDDs are equal. BDD equality is a cheap canonical handle
compare — structurally identical functions share one node — so the grouping is exact, not a heuristic.

1. **Group by function.** Scan the signals in order and bucket each one by its BDD — plain BDD
   equality only, no inverse/complement matching. Bare ±aliases are grouped exactly like any other
   signal: a bare alias that happens to share its BDD with another key falls into that key's group like
   any duplicate. A lone bare alias with no BDD-equal peer forms a singleton group, which carries no
   duplicate and falls through to the fold pass.

2. **Split by interface: internals always retire, outputs only when recurrent.** A group with more
   than one member holds a duplicate, and every duplicate that is an **internal** (non-output) signal
   is unconditionally retired onto the representative — the cell interface is sacred, but an internal
   has none to protect, so plain BDD equality alone is enough to purge it and rewrite its consumers onto
   `var(rep)`. A duplicate that is an **output**, by contrast, is never purged — its pin always survives
   — and is only *aliased* (demoted to `var(rep)`) when the group is **recurrent**: its shared function
   references one of the group's own members. Recurrence is evaluated against the representative's
   *current* function at commit time, so an internal retirement earlier in the same pass (which can only
   remove member references, never add one) is already reflected when an output's recurrence is judged.
   Once every aliased member is renamed to `var(rep)`, the representative is self-referential and so a
   genuine **state variable**, which is what makes the resulting `var(rep)` aliases machine-evaluable. A
   purely **combinational** output duplicate — no member in the shared δ, e.g. two output pins both
   computing `A·B` — is **left independent**: an alias's target must be a state variable, because the
   machine evaluates each signal over the primary inputs plus the state variables only (invariant
   **I3**), so aliasing an output to a combinational signal would make it unevaluable. Those output
   duplicates stay independent full-function signals; an *internal* duplicate in the same group would
   still retire regardless of recurrence.

3. **Choose the representative:** the first external output in the group, else the first member in
   scan order. This guarantees an external pin is preserved wherever the group holds one.

4. **Merge onto the representative:**
   - Build a rename map covering exactly the *retired* members: every non-rep **internal** (always),
     plus every non-rep **output** that is retired because the group is recurrent. A non-recurrent
     output duplicate is not in the map — it stays independent and its consumers are not rewritten.
   - Every surviving signal that references a retired member is rewritten with that rename map, so all
     references now point at the representative.
   - Retired members are handled per kind: internals are removed and purged; **outputs are demoted** to
     `var(rep)` — they keep their pin but become a combinational function of the representative, and
     are marked changed.

This catches **truly-parallel recurrent aliases** that folding need not collapse: two output pins
wired to the same self-reaching logic are one coordinate even though neither is a bare ±var of the
other and neither relays through the other.

### Worked example — recurrent duplicate output pins

`Q1 = !R·(S+Q1)`, `Q2 = !R·(S+Q1)`.

Both pins compute the *identical* function, and that function references `Q1` — a group member — so
the coordinate is recurrent. Dedup groups `{Q1, Q2}`, keeps `Q1` as representative (the first external
output), and demotes `Q2` to `var(Q1)`: the pin survives as a combinational function of the
representative, and `Q1` is left self-holding — a genuine state variable, so the `var(Q1)` alias is
machine-evaluable.

Two output pins that instead both compute a purely **combinational** `A·B` share no member in their δ,
so dedup leaves them independent full-function signals rather than alias one pin to a non-state rep —
outputs stay independent. Had one member of that pair instead been an *internal* signal computing the
same `A·B`, dedup would still retire it onto the other regardless of recurrence: internal retirement is
unconditional.

### Worked example — the buffered C-element

`Q = !QN`, `IQ = !QN`, `QN = !(A·B + IQ·(A+B))`. `Q` and `IQ` are plain-BDD-equal — both compute
`!var(QN)` — so dedup groups `{Q, IQ}`. `Q` is the external output and so the representative; `IQ` is
internal, so it retires unconditionally: it is purged, and `QN`'s reference to `IQ` is rewritten to
`var(Q)`, leaving `QN = !(A·B + Q·(A+B))`. Dedup's job is done; the (untouched) fold pass then
eliminates the internal `QN` by folding its definition `QN = !(A·B + Q·(A+B))` into the alias
`Q = !QN` and purging `QN`. The double negation cancels — `Q = !(!(A·B + Q·(A+B)))` — collapsing onto
the single coordinate `δ_Q = A·B + Q·(A+B)` on the output `Q`: one bit, exactly the physical cell.

## Guarded arity-aware fold

This pass eliminates signals that hold no memory of their own — a bare alias, a combinational relay
whose value is fixed by the current inputs and coordinates — while refusing any fold that would
*fabricate* a register out of emergent memory. It scans candidates in signal order and does three
things.

### Landing the coordinate on an output alias

A bare ±alias `s = ±var(t)` is exactly one coordinate shared by `s` and `t`. When `s` is an **external
output** and `t` is an **internal** key, `s` is the keeper: fold `t`'s definer into `s`'s equation
(re-expressing `t` as ±s, parity-corrected), fold that everywhere `t` was referenced, and purge `t`, so
the coordinate lands on the output pin `s`. The sign of the alias simply carries through the composition
arithmetic — there is no separate inversion step. This breaks the `s ↔ t` alias 2-cycle that the
register guard below would otherwise refuse.

This is what collapses the **gate-level C-element chain** `Q = IQ`, `IQ = !QN`, `QN = …`. Neither
`Q = var(IQ)` nor `IQ = !var(QN)` is a duplicate, so dedup leaves them; the fold folds each internal's
definer into its consumer one link per round, the coordinate ending on the output `Q`. Round one keeps
output `Q`, folds `IQ`'s definer into it and purges `IQ`, leaving `Q = !QN`, `QN = !(A·B + Q·(A+B))`.
Round two folds `QN`'s definer into `Q = !QN`, purging the internal `QN`; the double negation cancels in
the composition, leaving the single coordinate `δ_Q = A·B + Q·(A+B)` on the output. (A **complement
output pair** — where the alias target `t` is *itself* an output — is left to dedup/demotion instead:
the fold never retires a pin.)

### Guarded relay elimination

A signal `s` that does not appear in the signal support of `δ_s` is a **combinational relay**: at every
stable state `s = δ_s(state)` with `s ∉ support(δ_s)`, so `δ_s` can be composed into each consumer and
`s` dropped (internal → purged; output → kept but no longer consumed). A relay with no consumers is a
dead internal (purged) or a legitimate dead output (e.g. ICM's `GCLK`, which nothing consumes — kept).

The fold is refused by a **three-clause arity guard**. It declines to fold `s` iff **all** of:

- `arity(δ_s) > 1` — `δ_s` names more than one variable; and
- a consumer `c ∈ vars(δ_s)` forms an `s ↔ c` 2-cycle; and
- that `c` does **not already self-hold**.

That triple is the emergent-memory signature: `s` and `c` hold no memory individually, so the fold
would invent a self-loop for `c` and project an oscillation that lived in their *disagreement* onto a
single-node stable state, hiding it. If `c` already self-holds it is a genuine register and folding the
relay into it is safe (only a *new* self-reference is forbidden). **Mutex** is refused; a **ring
oscillator** whose register already self-holds is allowed.

### Arity-1 is lockstep

The first guard clause is the crux: **only a multi-input relay can fabricate a register**. A bare ±var
alias (`arity(δ_s) == 1`) is in lockstep with its single target — it carries exactly that one bit at
every state — so it *always* folds, 2-cycle or not. The guard can only ever trip on a
relay with two or more inputs.

### Worked example — the ICM interlock relays

ICM's `sela = !enB·!S` and `selb = !enA·S` are non-self-holding, multi-input relays that each feed a
synchroniser latch that already self-holds (`sela1`, `selb1`). Neither consumer is in the relay's
support (no 2-cycle), so the guard passes and both fold in:
`sela1 = !RA·(!CLKA·(!enB·!S) + CLKA·sela1)`.

`sela`/`selb` are purged; the machine width drops from 13 to 11 coordinates, and `sela1`/`selb1` now
carry `enB`/`enA` and `S` in their statetable columns.

### What the guard keeps — and what it lets fold

- **Mutex** `Qa = !Qb·A`, `Qb = !Qa·B`: arity 2, neither self-holds, so folding either fabricates a
  register. Folding `Qa` gives `δ_Qb = Qb·B + !A·B`, which at `A=B=1` is `δ_Qb = Qb` — the
  `(0,0) ↔ (1,1)` oscillation collapses to two stable states and is **lost**. Refused; both
  coordinates kept.
- **SR NOR latch**, **master/slave DFF**: self-holding (or become so on fold) → kept.
- **Ring oscillator** `X = !Q·A`, `Q = Q·B + X`: `Q` **already self-holds**, so folding `X`
  re-expresses an existing register rather than inventing one. `X` folds; `δ_Q = Q·B + !Q·A` still
  oscillates (`δ_Q = !Q` at `A·!B`), so the machine still flags the oscillation — the group is just
  `{Q}` instead of `{Q, X}`. The reported group is the genuine memory coordinates that oscillate, not
  the relays feeding them.

## How dedup and fold interact

The two passes partition the aliasing they resolve by a hard interface rule, not by function shape:

- **Dedup owns every plain-BDD equality, including between bare ±aliases, and never removes an output
  pin.** Dedup groups signals by plain BDD equality with no special case for bare ±aliases — a bare
  alias that is BDD-equal to another key is a duplicate like any other and is grouped with it. Within a
  group, every internal duplicate is purged unconditionally; a duplicate output is never purged, only
  aliased (demoted to `var(rep)`), and only when the group is recurrent. Fold owns everything dedup
  leaves as a singleton: substitute-and-drop for a signal not in its own support. A substitution that
  would create a self-reference is permitted **only** when the inserted function has support arity 1 —
  that includes a bare ±alias `s = ±var(t)` (arity 1, `t` internal), which the fold resolves by landing
  the coordinate on the output alias, but arity 1 is just the general gate, not special "inverse
  handling": any arity-1 function is lockstep with its sole input and always folds. So a signal is never
  contested between the passes.
- **No output-output exclusion is needed.** Dedup may share a single coordinate between *two output
  pins* — it keeps one pin as representative and aliases the other to `var(rep)` when recurrent, and the
  pin is preserved either way. But dedup only ever aliases to a **self-reaching** representative (the
  recurrence condition above), and the fold skips self-holding candidates, so a dedup alias is never a
  fold candidate and can never be re-expanded — no special exclusion is required. Conversely, an
  output-alias fold that *resolves* a combinational alias — an output buffer or inverter of a
  combinational output — has no shared coordinate to protect and proceeds normally.

## Why the rewrite is behaviour-preserving

- **(I1) Arity-1 fold soundness.** A bare ±var alias carries exactly one bit — it equals
  `±` its target at every state — so folding it, or folding an internal target's definition into an
  output alias of it, is exact renaming (parity-corrected via the BDD compose; the sign is incidental).
  Arity-1 is always lockstep, so this is unconditional. An all-wire cycle **collapses** to a single
  keeper coordinate (`a="b"` → `b = var(b)`, a lone self-holding keeper) or a one-node oscillator
  (`a="!b"` → `b = !var(b)`), and the surviving node holds exactly the one bit the cycle carried, so
  its dynamics are preserved.
- **(I2) Arity guard.** Only a multi-input relay can fabricate a register, and the guard refuses
  exactly the fold that would: a 2-cycle consumer `c ∈ vars(δ_s)` that does not already self-hold, the
  sole way a fold can turn a multi-node oscillation into a stable self-hold. Mutex is refused; folding
  a relay into a consumer that already self-holds (ROSC) preserves the dynamics and is allowed.
- **(I3) Convergence-point invariant.** At termination every surviving signal's signal support is a subset of
  the primary inputs plus the self-reaching signals: any consumed non-self-holding signal is a fold
  candidate, and a refusal implies a 2-cycle whose members self-reach. So state-variable
  classification identifies exactly the coordinates and the machine's δ is a direct map lookup.
- **(I4) Termination.** Every dedup commit either purges an **internal** duplicate (the signal map
  strictly shrinks) or idempotently aliases an **output** duplicate onto an output representative (the
  output is never purged, so a re-classified output produces no further commit); every fold commit
  removes a signal from every support (a signal re-enters a support only via an alias/demotion, bounded
  by the output count). So the convergence point is reached within the asserted bound.
- **(I5) Dedup soundness.** Two signals with the *same* BDD compute the same transition function, so
  they are `=` the same underlying coordinate at every state; renaming the retired members onto
  `var(rep)` is exact. Internal retirement is unconditional and purges the internal; output aliasing is
  licensed only by recurrence — read against the representative's function at commit time — and never
  purges the pin, so the output-preferring representative keeps a pin wherever the group holds one, and
  an aliased output remains a combinational function of the representative.

The safety boundary is about *behaviour*, not names: a cell's derived arcs, hidden arcs, and the
**existence and condition** of every oscillation group must match the un-reduced cell — a folded relay
leaving a group's membership is not a regression (see the ring-oscillator case above). Gained
constraints are permitted (a relay can have been masking a genuine hazard); losses are not. This is
locked by the behaviour-preservation golden tests.

## Known limits

The arity guard is a structural proxy for "removing `s` preserves the reachable-state cycle
structure", and it inspects only `s ↔ c` **2-cycles**. The only residual case is an *emergent
all-relay ring whose links are all arity > 1* — a longer relay/fold loop where **every** node is a
multi-input relay and none self-holds (e.g. `X1="!X3·A", X2="!X1·B", X3="!X2·C"`: no stable states, no
committed fixture). Such a ring can admit a fold before any 2-cycle appears, shrinking a would-be
oscillation group.

Arity-1 links never contribute to this limit: a bare ±var alias always collapses soundly (I1), so
any ring with even one wire link is resolved rather than mis-folded. No committed or mandated cell is
affected: MUT and SR (an SR NOR latch, as above) are 2-cycles the guard catches, and ICM's folded
relays feed synchroniser latches that already self-hold. A fully general criterion would carry a BDD
check that the projected cycle structure survives; the structural guard is accepted per the decided
enforcement level.
