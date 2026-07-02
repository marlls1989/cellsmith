# The state-machine arc engine

How lobsterate derives Liberate timing arcs for state-holding cells (C-elements, latches, SR pairs,
mutexes/arbiters, flip-flops with internal state). This document explains the model, the state machine,
how a state is settled to a fixpoint, and how arcs are discovered — with a full worked example.

The code lives in `src/logic/`:

| File | Role |
|------|------|
| `resolve.rs` | classify state variables; build each state variable's next-state function δ |
| `machine.rs` | the async state machine: nodes as minterms, `settle` / `is_stable` / `settle_or_cycle` |
| `arcs.rs` | the BFS that explores the machine and emits arcs |
| `confluence.rs` | pairwise input-order confluence over the same reachable states: constraint arcs and metastable arbitration (see `hazard-detection.md`) |
| `interlock.rs` | the `Arbitration` report type, populated by `confluence.rs` |

## 1. The model

A cell is a **name**, an ordered list of **inputs**, a Boolean **function per output**, and optionally
some **internal** functions (`[cell.internal]`). Two rules make state work with no special ceremony:

- **Any signal name referenced inside a function is that signal's feedback/delayed value.** A C-element
  referencing `Q`, an SR pair referencing each other, and a flop's slave referencing its master are all
  ordinary references.
- An **internal** signal is a first-class state node that other functions may reference but which emits
  **no external pin** — it models hidden state such as a flip-flop's master latch.

The cell is then treated as an **asynchronous state machine** over `inputs × state-variables`.

### State variables vs. combinational signals

A **state variable** is any signal (output or internal) that lies on a feedback cycle — a self-loop
(`Q = A·B + Q·(A+B)`) or a larger coupling cycle (`Qa = !Qb·A`, `Qb = !Qa·B`). A signal on no cycle is
**combinational**: it is a pure function of inputs and other signals and folds away.

`resolve::state_variables` finds them structurally: build the reference graph (`dependency_map`), take
its transitive closure (`transitive_closure`), and a signal `s` is a state variable iff **`s` reaches
itself**. Only the state variables become coordinates of the machine's state; everything else is
eliminated.

## 2. The next-state function δ

For each state variable `v`, `resolve::delta(v, …)` produces a BDD **δ_v** giving `v`'s next value as a
function of `inputs ∪ state-variables`. It is `resolve` (compose referenced signals in via the Shannon
identity `f[x:=g] = g.ite(f|x=1, f|x=0)`) but substituting **only combinational** signals — every state
variable is *kept* as a current-state coordinate rather than folded away.

For the mutex `Qa = !Qb·A`, `Qb = !Qa·B`, both are state variables, so:

- **δ_Qa = !Qb · A**  (Qb kept; A is an input)
- **δ_Qb = !Qa · B**  (Qa kept; B is an input)

For a combinational output the same `delta` resolves it to a function of inputs + state variables; it is
read but never carried as state.

## 3. The state machine, represented as minterms

A machine **node** is a fully-fixed `Minterm<Symbol>` over the shared header `[inputs…, state_vars…]`:
every input and every state variable carries a concrete `0`/`1`. The header is a shared
`Arc<Symbols<Symbol>>` (`machine::header`), so nodes built from it compare on the fast path and can be
used directly as hash/tree keys — `Minterm<Symbol>` is `Hash + Ord`.

`machine::node_from(header, |name| bool)` builds a node from a `name → value` lookup via
`Minterm::from_symbols`. There is **no** integer bitmask and **no** precomputed next-state table; the
state *is* the minterm, and next-states are computed on demand.

For the mutex a node is written here as `(A B | Qa Qb)`, e.g. `(1 1 | 0 1)`.

## 4. Settling a state, and how "stable" is decided

This is the heart of the engine. Everything rests on one fact: **for a fixed input, one settle round is
a deterministic, total function `step : State → State`.**

### One round: `step`

`machine::step(node)` computes, for each state variable `v`,

```
v' = δ_v.evaluate(node)          // a concrete true/false
```

and returns a new node with the inputs unchanged and each state field replaced by its `v'`.

Two properties make this well-defined:

1. **`Bdd::evaluate` returns a concrete `Ok(bool)`, never `Err`.** `δ_v` depends only on
   `inputs ∪ state_vars`, and a node fixes *all* of them. `evaluate` returns `Ok(true/false)` exactly
   when the fixed variables determine the function — a complete assignment over the support always does.
   So each round yields definite values, with no residual and no ambiguity.
2. All `v'` are read from the **same** current node before the new node is built, so `step` is a genuine
   synchronous update, not order-dependent.

### Fixpoint = `step(node) == node`

`machine::settle` iterates:

```rust
let next = step(cur);
if next == cur { return Some(cur); }   // fixpoint
```

The comparison is plain `Minterm` equality over the shared header. **One match is enough to stop, and
this is exact, not heuristic:** `step` is deterministic and pure, so

```
step(x) = x   ⟹   stepⁿ(x) = x   for all n.
```

A fixed point reproduces itself under every further application — iterating again cannot change
anything. That is the whole reason we don't have to "keep going to be sure."

### `is_stable`: the one-round test

`machine::is_stable(deltas, node)` is the same test without building the new node — it checks each
coordinate is already unchanged:

```
δ_v.evaluate(node) == node.value_of(v)     for every state variable v
```

If that holds for all `v`, then `step(node) == node` by definition, so the node is a fixpoint. This is
used to pick the reset-stable start states that seed `explore` (§5) — the same reachable-state walk
`confluence.rs` re-probes to detect metastable arbitration, not by enumerating joint stable states but by
watching `settle`/`settle_or_cycle` fail to reach one (the oscillation described next).

Example — settled mutex state `(1 0 | 1 0)`:

- δ_Qa = !Qb·A = !0·1 = **1** = current Qa ✓
- δ_Qb = !Qa·B = !1·0 = **0** = current Qb ✓

Both coordinates already agree ⇒ `step` maps it to itself ⇒ stable.

A mid-cascade state `(1 0 | 0 1)` fails, so another round is required:

- δ_Qa = !1·1 = 0 = current 0 ✓, but
- δ_Qb = !0·0 = 0 ≠ current 1 ✗

### Termination and oscillation

What if a state never settles? Two facts bound it:

- for a fixed input there are only `2^k` states (k = number of state variables) — a **finite** space;
- `step` is deterministic.

So the trajectory `s → step(s) → step²(s) → …` must eventually **repeat** a state; once it does it is
periodic and can never reach a fixpoint. `settle` detects that with a visited set:

```rust
if !seen.insert(next.clone()) { return None; }   // revisited → oscillation
```

Returning `None` means "no stable state" — a metastable / deadlock condition — and the BFS simply drops
that transition (so no impossible arc is fabricated). This also guarantees termination: within at most
`2^k` rounds, `settle` either hits `next == cur` or re-inserts a seen state.

`settle` is a thin wrapper over `settle_or_cycle`, which on oscillation returns the periodic cycle itself
rather than discarding it. `confluence.rs` is what turns that cycle into a report: probing a reachable
stable state with a simultaneous multi-input toggle (a mutex's requests co-asserting) and finding no
fixpoint names the varying state variables as an arbitrating group, rather than silently dropping the
transition the way the arc BFS does.

Example — mutex under `A=B=1` from `(1 1 | 0 0)`:

```
(1 1 | 0 0) → (1 1 | 1 1) → (1 1 | 0 0) → …
```

The second `(1 1 | 0 0)` is already in `seen` ⇒ `None` ⇒ no arc. The same equality/visited machinery
that confirms a fixpoint also rejects the states that don't have one.

## 5. Deriving arcs by breadth-first search

With `settle` in hand, `arcs::cell_arcs` explores the reachable settled states:

1. **Reset-stable starts.** Enumerate the `2^k` state assignments under the all-zero input and keep the
   ones that are `is_stable`. (If none is — rare — fall back to every stable node over all inputs.)
2. **BFS.** From each node, toggle **one input at a time**, hold the state, and `settle`. A `None`
   (oscillation) transition is skipped. Newly reached settled nodes are enqueued, with `prev[node]`
   recording the predecessor for path reconstruction. Nodes are keyed by their minterm.
3. **Emit an arc** wherever a single input toggle flips an **output**'s value (a state output reads its
   own field; a combinational output is its δ evaluated at the node). The toggled input is the
   `related` pin — so a related pin is **always a primary input**; outputs and internal nodes never are.
4. **Prevector.** The arc's prevector is the BFS path from a start node to the source node, each node
   projected onto the inputs (`Minterm::project_onto(&input_header)`). Since the path drives every state
   variable — internal ones included — into the measured start state, it establishes hidden state such
   as a flop's master before the clock edge.

A blow-up guard (`n + k > 22`) bails out on pathologically wide cells.

Because arcs are found by *reaching* states and *settling*, the correctness properties are structural,
not bolted on: related pins are inputs, impossible arcs are never reached (they oscillate), and
input-forced transitions cascade naturally through the multi-round settle.

## 6. Worked example: discovering `B↓ → Qa↑` on the mutex

`MUT`: `Qa = !Qb·A`, `Qb = !Qa·B`. We trace the arc `-related_pin B -pin Qa` (rise), whose emitted block
is `-prevector {00 01 11} -vector {1 F R X}`.

**Start.** Under `A=B=0`, both δ force `0`, so the only stable state is `S0 = (0 0 | 0 0)`.

**Walk B in.** From `S0`, toggle `B`: `(0 1 | 0 0)` settles to `N_B = (0 1 | 0 1)` — *B holds the
grant*. `prev[N_B] = S0`. (This step also emits the plain `B → Qb↑` arc.)

**Walk A in.** From `N_B`, toggle `A`: `(1 1 | 0 1)` is already stable, giving
`N_AB2 = (1 1 | 0 1)` — *both requested, B still owns it*. No output flipped, so no arc, but it is a
reachable state. `prev[N_AB2] = N_B`.

The path `S0 → N_B → N_AB2` projected onto the inputs is the prevector **`00 01 11`**: it drives the
machine into "B owns the grant, A contending" — the only precondition under which releasing B can hand
the grant to A.

**The measured edge.** Dequeuing `N_AB2 = (1 1 | 0 1)`, toggle `B` (hold `Qa=0, Qb=1`, set `B=0`) and
settle:

| round | node | δ_Qa = !Qb·A | δ_Qb = !Qa·B |
|-------|------|--------------|--------------|
| start | `(1 0 \| 0 1)` | !1·1 = **0** | !0·0 = **0** |
| →     | `(1 0 \| 0 0)` | !0·1 = **1** | !0·0 = **0** |
| →     | `(1 0 \| 1 0)` | !0·1 = 1 | !1·0 = 0 → **fixpoint** |

Settled `(1 0 | 1 0)`. The two micro-steps are the physical cascade: B drops → Qb falls; with Qb=0 and
A=1, Qa rises.

**Emit.** Across the toggle at `N_AB2 → (1 0 | 1 0)`, `Qa: 0 → 1` (rise). Arc: `related = B`, `pin = Qa`,
edge Rise, `start = 11`, `end = 10`, `prevector = 00 01 11`, vector `{1 F R X}`. (The same step emits
`B → Qb↓`.)

**Why this matters.** `B` never appears in `Qa`'s own function; the old approach had to substitute the
peer `Qb = !Qa·B` into `Qa` symbolically just to surface it, which is what made cross-coupled collapse
fragile. Here nothing symbolic happens — the search *reaches* the state where B owns the grant, toggles
B, and the parallel `settle` discovers `Qb↓ ⟹ Qa↑` through the transient `(Qa=0, Qb=0)`. The dependence
of `Qa` on `B` is an emergent property of the reachable state graph, not a term anyone manufactured.

The mirror arc `A↓ → Qb↑` is discovered symmetrically from `N_AB1 = (1 1 | 1 0)`.
