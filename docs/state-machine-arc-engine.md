# The state-machine arc engine

How cellsmith derives Liberate timing arcs for state-holding cells (C-elements, latches, SR pairs,
mutexes/arbiters, flip-flops with internal state). This document explains the model, the state machine,
how a state is settled to a fixpoint, and how arcs are discovered — with a full worked example.

The code lives in `src/logic/`:

| File | Role |
|------|------|
| `resolve.rs` | classify state variables; build each state variable's next-state function δ |
| `machine.rs` | the async state machine: nodes as minterms, `settle` / `settle_or_cycle`, and `explore` |
| `analysis.rs` | the shared machine pass — builds the machine once and derives both arcs and hazards from it |
| `arcs.rs` | arc emission by re-walking the shared exploration |
| `confluence.rs` | pairwise input-order confluence over the same reachable states: constraint arcs and metastable arbitration (see `hazard-detection.md`) |
| `interlock.rs` | the `Arbitration` report type, populated by `confluence.rs` |

The functional state-table view of the same signals (`regions.rs`) is documented separately in
`state-table-regions.md`.

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

`resolve::state_variables` (`resolve.rs:134`) finds them structurally: build the reference graph
(`dependency_map`, `resolve.rs:29`), take its transitive closure (`transitive_closure`, `resolve.rs:107`,
private), and a signal `s` is a state variable iff **`s` reaches itself**. Only the state variables
become coordinates of the machine's state; everything else is eliminated.

## 2. The next-state function δ

For each state variable `v`, `resolve::delta(v, …)` (`resolve.rs:148-159`) produces a BDD **δ_v** giving
`v`'s next value as a function of `inputs ∪ state-variables`. It is `resolve` (`resolve.rs:78`) composing
referenced signals in via the BDD layer's native `Bdd::compose` — the substitution `f[v:=g]`
(`resolve.rs:98`; the module comment records that composition uses `Bdd::compose`, `resolve.rs:19`) — but
substituting **only combinational** signals: every state variable is *kept* as a current-state coordinate
rather than folded away.

For the mutex `Qa = !Qb·A`, `Qb = !Qa·B`, both are state variables, so:

- **δ_Qa = !Qb · A**  (Qb kept; A is an input)
- **δ_Qb = !Qa · B**  (Qa kept; B is an input)

For a combinational output the same `delta` resolves it to a function of inputs + state variables; it is
read but never carried as state.

## 3. The state machine, represented as minterms

A machine **node** is a self-describing `Minterm<Symbol>`: it carries its own ordered columns
(`Minterm::vars`), so **there is no shared header object** (`machine.rs:3-6`). Every input carries a
concrete `0`/`1`, and each state variable is either **defined** (a concrete `0`/`1`) or **absent** —
encoded as the don't-care `-`, never a placeholder value. The power-on state is the inputs-only node,
with every state variable absent: no state fixed (`machine.rs:5-11`).

Because a node is a plain `Minterm<Symbol>` (`Hash + Ord`), it can be used directly as a hash/tree key,
and there is **no** integer bitmask and **no** precomputed next-state table: the state *is* the minterm,
and next-states are computed on demand.

For the mutex a node is written here as `(A B | Qa Qb)`, e.g. `(1 1 | 0 1)`; an absent state variable is
shown as `-`. (The test-only helpers `machine::node_from` / `node_from_opt`, `machine.rs:29-30,43-44`,
build nodes in unit tests and are `#[cfg(test)]`, not a production API.)

## 4. Settling a state, and how "stable" is decided

This is the heart of the engine. Everything rests on one fact: **for fixed inputs, one settle round is a
deterministic function `step : State → State`.**

### One round: `step`

`machine::step(deltas, node)` (`machine.rs:64`, private) computes, for each state variable `v`,

```
v' = δ_v.evaluate(node).ok()      // Some(true/false), or None
```

and returns a new node with the inputs unchanged and each state field replaced by its `v'`.

Two properties make this well-defined:

1. **`Bdd::evaluate` returns `Ok(v)` only when the node's fixed columns force δ_v; otherwise it returns
   `Err`, and that `Err` is the expected, load-bearing case.** A node need not fix all of δ_v's support:
   an absent state variable leaves the value undetermined, so `evaluate` returns `Err`, and `step` writes
   that column **absent** (`.evaluate(node).ok()`, `None` ⇒ `-`, `machine.rs:79-87`). Returning `Err` is
   not an error path — it is how a state variable that the inputs (and the resolved state so far) do not
   yet determine stays absent (`machine.rs:6-11`).
2. All `v'` are read from the **same** current node before the new node is built, so `step` is a genuine
   parallel update, not order-dependent.

### Fixpoint = `step(node) == node`

`machine::settle_or_cycle` (`machine.rs:108-128`) iterates:

```rust
let next = step(deltas, &cur);
if next == cur { return Ok(cur); }   // fixpoint
```

The comparison is plain `Minterm` equality. **One match is enough to stop, and this is exact, not
heuristic:** `step` is deterministic and pure, so

```
step(x) = x   ⟹   stepⁿ(x) = x   for all n.
```

A fixed point reproduces itself under every further application — iterating again cannot change anything.
That is the whole reason we don't have to "keep going to be sure." The fixpoint **may still leave state
variables absent** — those the inputs and resolved state do not determine (`machine.rs:96-98`).

`machine::settle` (`machine.rs:99-104`) is `settle_or_cycle(…).ok()`: it returns `Option`, discarding the
oscillation detail and yielding `None` when the state never settles.

### `is_stable`: the one-round test

`machine::is_stable(deltas, node)` (`machine.rs:91-92`, `#[cfg(test)]`) is the same test without building
the new node: it checks `step(deltas, node) == node`. It is a unit-test helper, not part of the
production walk.

Example — settled mutex state `(1 0 | 1 0)`:

- δ_Qa = !Qb·A = !0·1 = **1** = current Qa ✓
- δ_Qb = !Qa·B = !1·0 = **0** = current Qb ✓

Both coordinates already agree ⇒ `step` maps it to itself ⇒ stable.

A mid-cascade state `(1 0 | 0 1)` fails, so another round is required:

- δ_Qa = !1·1 = 0 = current 0 ✓, but
- δ_Qb = !0·0 = 0 ≠ current 1 ✗

### Termination and oscillation

What if a state never settles? For fixed inputs the reachable state space is **finite** (each state
variable is `0`, `1`, or absent) and `step` is deterministic, so the trajectory
`s → step(s) → step²(s) → …` must eventually **repeat** a state; once it does it is periodic and can
never reach a fixpoint.

`settle_or_cycle` detects that with a `pos: HashMap<Minterm<Symbol>, usize>` (the index at which each
visited state first appeared) alongside a `trace: Vec<Minterm<Symbol>>` of the states in order. When
`step` produces a `next` already in `pos` at index `p`, it returns `Err(trace[p..].to_vec())` — the
periodic cycle slice (`machine.rs:112-127`):

```rust
if let Some(&p) = pos.get(&next) {
    return Err(trace[p..].to_vec());   // revisited a non-fixpoint → the oscillating cycle
}
```

`settle` maps that `Err` to `None`, meaning "no stable state" — a metastable / deadlock condition — and
the BFS simply drops that transition (so no impossible arc is fabricated). `confluence.rs` instead keeps
the cycle: probing a reachable stable state with a simultaneous multi-input toggle (a mutex's requests
co-asserting) and finding a cycle rather than a fixpoint names the varying state variables as an
arbitrating group.

Example — mutex under `A=B=1` from `(1 1 | 0 0)`:

```
(1 1 | 0 0) → (1 1 | 1 1) → (1 1 | 0 0) → …
```

The second `(1 1 | 0 0)` is already in `pos` ⇒ `Err` ⇒ `settle` yields `None` ⇒ no arc. The same
equality/visited machinery that confirms a fixpoint also rejects the states that don't have one.

## 5. Deriving arcs by re-walking the shared exploration

Arc derivation does not run its own search. The reachable stable states are discovered once, by
`machine::explore`, and `arcs::derive` re-walks them.

### The exploration: `machine::explore`

`machine::explore` (`machine.rs:172-308`) both discovers the start states and runs the BFS:

1. **Start candidates — never an assumed all-zero state.** For each seed function (each state variable's
   δ plus the combinational outputs' δ) it takes the forced on/off cover over the inputs via
   `Bdd::cover_over_fr` (`machine.rs:154-155,191-211`): input assignments that force the signal
   *regardless of* the undefined power-on state. These input minterms are pooled, then **ranked by how
   many state variables they settle** (ties broken toward state nearest the inputs). Each candidate input
   is widened onto the full `[inputs…, state_vars…]` columns — the state columns arrive **absent** — and
   settled with `settle` (`machine.rs:246-288`). A state-holding cell whose reset is an input *sequence*
   rather than a level is therefore initialised by the sequence that actually resolves it, not by an
   arbitrary held combination.
2. **BFS.** From each stable node, toggle **one input at a time** (`machine::toggle`), hold the state,
   and `settle`. A `None` (oscillation) transition is skipped. Newly reached settled nodes are enqueued,
   with the predecessor recorded in `Explored::prev` for path reconstruction (`machine.rs:292-305`).
   Nodes are keyed by their minterm.

`explore` returns an `Explored { order, prev }`, shared by both `arcs::derive` and `confluence::derive`.

### The arc emission: `arcs::derive`

`arcs::derive` (`arcs.rs:70`) re-walks `ex.order` only and emits arcs (`arcs.rs:96-146`):

- **Emit an arc** wherever a single input toggle flips an **output**'s value (a state output reads its
  own field; a combinational output is its δ evaluated at the node). The toggled input is the `related`
  pin — so a related pin is **always a primary input**; outputs and internal nodes never are.
- **Prevector.** The arc's prevector is the BFS path from a start node to the source node, each node
  projected onto the inputs. It is reconstructed by `Explored::path_to` (`machine.rs:142-151`), which
  walks predecessors back to a start, reverses, and projects each step onto the input names via
  `Minterm::project_to(input_names)` (`arcs.rs:111`). Since the path drives every state variable —
  internal ones included — into the measured start state, it establishes hidden state such as a flop's
  master before the clock edge.
- **Dedup.** The same arc can be reached from several start candidates; `derive` keeps the one with the
  **shortest prevector** (`arcs.rs:96-141`).

Because arcs are found by *reaching* states and *settling*, the correctness properties are structural,
not bolted on: related pins are inputs, impossible arcs are never reached (they oscillate), and
input-forced transitions cascade naturally through the multi-round settle.

### The shared machine pass: `analysis.rs`

The whole setup happens once, in `analysis.rs`. `Machine::build` (`analysis.rs:67-132`) builds every
signal's BDD, each state variable's δ, the combinational outputs' δ, and runs the **one**
`machine::explore` BFS seeded from all of them. `analyse_machine` (`analysis.rs:138-150`) mints a
per-cell BDD builder — a fresh brand for each cell, so handles from two cells cannot be mixed — and
derives **both** `arcs::derive` and `confluence::derive` from the shared `Machine`.

A combinatorial blow-up guard, `MAX_MACHINE_VARS = 22` (`analysis.rs:45`), gates the whole shared pass:
`Machine::build` returns `None` — leaving the cell unexplored, so arcs *and* hazards come back empty —
when `inputs + state variables` exceeds it (`analysis.rs:86`).

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

`B` does not appear in `Qa`'s own function. The `B↓ → Qa↑` dependence arises from *reaching* the state
where `B` holds the grant and *settling* through the transient `(Qa=0, Qb=0)`: the search toggles `B` and
the parallel `settle` propagates `Qb↓ ⟹ Qa↑`. The dependence is an emergent property of the reachable
state graph.

The mirror arc `A↓ → Qb↑` is discovered symmetrically from `N_AB1 = (1 1 | 1 0)`.
