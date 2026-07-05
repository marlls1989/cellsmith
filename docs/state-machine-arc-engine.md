# The state-machine arc engine

How cellsmith derives Liberate timing arcs for state-holding cells (C-elements, latches, SR pairs,
mutexes/arbiters, flip-flops with internal state). This document explains the model, the state machine,
how a state is settled to a fixpoint, and how arcs are discovered — with a full worked example.

The code lives in `src/logic/`:

| File | Role |
|------|------|
| `minimise.rs` | one-shot state-space minimisation: alias/complement collapse + guarded relay fold on the shared per-cell BDD map, before the machine pass |
| `resolve.rs` | reference graph + state-variable classifier (over the minimised model) |
| `machine.rs` | the async state machine: states as minterms, `settle` / `settle_or_cycle`, and `explore` |
| `analysis.rs` | the shared machine pass — builds the machine once and derives both arcs and hazards from it |
| `arcs.rs` | arc emission by re-walking the shared exploration |
| `confluence.rs` | pairwise input-order confluence over the same reachable states: constraint arcs and metastable oscillation (see `hazard-detection.md`) |
| `interlock.rs` | the `Oscillation` report type, populated by `confluence.rs` |

The functional state-table view of the same signals (`regions.rs`) is documented separately in
`state-table-regions.md`.

## 1. The model

A cell is a **name**, an ordered list of **inputs**, a Boolean **function per output**, and optionally
some **internal** functions (`[cell.internal]`). Two rules make state work with no special ceremony:

- **Any signal name referenced inside a function is that signal's feedback/delayed value.** A C-element
  referencing `Q`, an SR pair referencing each other, and a flop's slave referencing its master are all
  ordinary references.
- An **internal** signal is a first-class signal that other functions may reference but which emits
  **no external pin** — it models hidden state such as a flip-flop's master latch.

From these functions the cell is treated as an **asynchronous state machine** over
`inputs × state-variables`. The rest of this document makes that machine precise: §2 gives the whole
picture and defines the vocabulary; §3–§5 detail how the machine is built and run; §6–§7 derive the arcs.

## 2. The machine at a glance

Before any construction detail, the whole pipeline in one view. For a cell, cellsmith:

1. **minimises** the cell's signal model once — collapsing alias/complement chains and folding guarded
   combinational relays — so only genuine memory coordinates remain (§3, §3.1);
2. classifies each surviving signal as a **state variable** (it holds state) or **combinational** (it does
   not);
3. reads, directly from the minimised model, a fixed **transition function** δ — one component δ_v per
   state variable;
4. **settles** a state by repeatedly *evaluating* δ until the state stops changing (a fixpoint);
5. **explores** the reachable settled states, toggling one input at a time;
6. **derives arcs** by re-walking that exploration and watching which input toggles flip an output.

### 2.1 Definitions

Every term the later sections lean on, pinned here before first use.

- **Signal** — an **output** (emits an external pin) or an **internal** (no pin). Both are Boolean
  functions that may reference inputs and other signals; a referenced signal is that signal's
  feedback/delayed value (§1).
- **Reference graph** — signal → the signals its function names (`dependency_map`, `resolve.rs:24`).
- **State variable** — a signal that lies on a **feedback cycle**: it reaches itself in the reference
  graph, whether by a self-loop (`Q = A·B + Q·(A+B)`) or a larger coupling cycle (`Qa = !Qb·A`,
  `Qb = !Qa·B`). Classification runs on the **minimised** model: `logic::minimise` (§3.1) first collapses
  every alias/complement chain onto one representative coordinate and composes every non-self-holding
  relay into its consumers — refusing only a fold that would *fabricate* a register (an emergent-memory
  2-cycle) — so self-reachability afterwards counts only genuine memory, never a spent wire or relay.
  `resolve::state_variables` (`resolve.rs:61`) then finds the state variables structurally over that
  minimised map: take the reference graph's transitive closure (`transitive_closure`, `resolve.rs:34`,
  private), and a signal `s` is a state variable iff **`s` reaches itself**. The state variables — and
  only they — become the coordinates of the machine's state.
- **Combinational signal** — a signal on **no** feedback cycle. Its value is fixed by the inputs and the
  current state, so it need not be held as a coordinate; it is eliminated (composed away, §3) and
  reconstructed on demand. **"Combinational" means "off every cycle," not "a function of inputs only"**:
  a combinational signal may reference state variables and so depend on the current state — it simply is
  not itself a piece of held state (see `ICM.GCLK` in §3).
- **State (of the machine)** — an assignment of a concrete `0`/`1` to every input and a concrete
  `0`/`1`-or-**absent** to every state variable, represented as a `Minterm<Symbol>` over the columns
  `[inputs…, state-vars…]` (§4).
- **Transition function δ** — the machine's next-state map, given as one **component δ_v per state
  variable**: a fixed Boolean function of `inputs ∪ state-variables` yielding v's next value (§3). δ is
  the standard automata-theory transition function, not a difference/delta of states.
- The verbs, in increasing scope:
  - **substitute** — the atomic step: replace one referenced signal name by its definition, **at most
    once**.
  - **compose** — the BDD primitive that performs one substitution, `f[v := g]` (`Bdd::compose`) — used
    directly for a guarded relay fold and, batched, as `Bdd::compose_map` for an alias/complement class
    rename (`logic::minimise`, §3.1).
  - **minimise** — drive substitution to a fixpoint **once**, before the machine is built:
    `logic::minimise::minimise_state_space` collapses alias/complement chains and folds non-self-holding
    relays into their consumers. δ_v is the minimised model's own function for `v`, read directly from
    the shared BDD map (`analysis.rs`); no per-signal composition remains once the machine is built.

## 3. Constructing the transition function δ_v

Every state variable's δ_v is already sitting in the shared BDD map by the time the machine is built.
`logic::minimise::minimise_state_space` (`minimise.rs:94-117`) rewrites that map **once**, before
`Machine::build` ever runs, so that every surviving signal's function is expressed purely over primary
inputs and the surviving state variables: `Machine::build` reads `bdds[v]` for each state variable `v`
(`analysis.rs:113-116`) and `bdds[o]` for each combinational output `o` (`analysis.rs:117-122`), and
stores them as `Machine.deltas` / `Machine.out_deltas`; the settle and explore passes (§5–§6) only
evaluate them. Constructing δ_v is therefore a direct map lookup, not a per-analysis composition.

The map itself was folded by two staged discriminators run to a fixpoint (the algorithm is documented
in full in `state-space-minimisation.md`; §3.1 below covers the safety guard, and the proof lives in
the `minimise.rs` module doc): **M1** collapses an alias/complement chain — a
signal whose function is *exactly* another signal or its negation — onto one representative coordinate
via `Bdd::compose_map`; **M2** composes a non-self-holding relay into each of its consumers via
`Bdd::compose` and drops it, refusing only a fold that would *fabricate* a register — an `s ↔ c`
2-cycle whose consumer does not already self-hold (emergent memory).

Three examples, each isolating one point.

**A combinational signal is composed away.** Take a flop whose data is an internal AND cone (an
illustrative cell):

```toml
[[cell]]
name = "DFFG"
inputs = ["CLK", "A", "B"]
[cell.internal]
D = "A*B"                 # on no cycle  → combinational → folded away
M = "!CLK*D + CLK*M"      # references M → self-loop     → state variable → kept
[cell.outputs]
Q = "CLK*M + !CLK*Q"      # references Q → self-loop     → state variable → kept
```

State variables are `{M, Q}`; `D` is combinational — and, being a relay whose only consumer is `M`, it is
folded away entirely by `logic::minimise` before the machine is even built (an M2 fold: `D` does not
self-hold, and `M` is not in `D`'s own support). `M`'s entry in the shared map already reads:

- **δ_M = !CLK·(A·B) + CLK·M**  (`D` folded away; `M` kept; `CLK`, `A`, `B` are inputs)

`D` has vanished from δ_M — that is a composition actually firing, just done once, upstream of the
machine pass, rather than per analysis.

**A state variable is kept.** The plain mutex `Qa = !Qb·A`, `Qb = !Qa·B` — both signals are state
variables, so nothing is folded away and each δ keeps its cross-coupled peer:

- **δ_Qa = !Qb · A**  (`Qb` kept; `A` is an input)
- **δ_Qb = !Qa · B**  (`Qa` kept; `B` is an input)

**A combinational output that depends on state.** The real `ICM` cell (`examples/cells.toml:55-72`) is
written with eight internal signals, all mutually coupled on one dependency cycle — but two of them,
`sela` and `selb`, are combinational relays on the enable loop (a selection interlock ahead of each
synchroniser), each with a single consumer and no self-reference. `logic::minimise` folds both into
their consumers (`sela` into `sela1`, `selb` into `selb1`) before the machine is built, leaving **six**
surviving coordinates — `sela1`, `sela2`, `enA`, `selb1`, `selb2`, `enB` — so the machine width drops
from 5 inputs + 8 state vars = 13 to 5 + 6 = **11**. One output:

```toml
GCLK = "enA*CLKA+enB*CLKB"
```

Nothing references `GCLK`, so it is on no cycle → **combinational**. Yet it references the state variables
`enA`, `enB`, so its δ keeps them:

- **δ_GCLK = enA·CLKA + enB·CLKB**  (`enA`, `enB` kept as state coordinates; `CLKA`, `CLKB` are inputs)

This is the concrete proof that "combinational" ≠ "function of inputs only": δ_GCLK genuinely depends on
the current state. A combinational output is resolved the same way as any folded-away signal — it just
never carries the result as a coordinate. Arc derivation reads a combinational output's value by
*evaluating* its δ at a state (`arcs.rs:125-126`); a state output instead reads its own state coordinate.

### 3.1 The safety guard: why folding must not fabricate a register

Collapsing a non-self-holding relay is safe — *unless* the fold would invent memory that wasn't there.
The machine detects **oscillating (non-confluent) states**; the guard exists to keep an oscillation from
being projected away, not to freeze the set of nodes that participate in it.

- **`MUT`** (§7 below) — the case the fold must **not** perform: `Qa = !Qb·A`, `Qb = !Qa·B`. Neither
  signal self-holds, yet each is the other's consumer — an `s ↔ c` 2-cycle. Folding `Qa` into `Qb`
  gives `δ_Qb = Qb·B + !A·B`, which at `A=B=1` is `δ_Qb = Qb` — a *fabricated* register. The oscillating
  `(0,0) ↔ (1,1)` oscillation (`hazard-detection.md` §4) lived in the *disagreement* of the two nodes;
  projected onto `Qb` alone it lands on `Qb`'s stable states and disappears. The pair must stay two
  coordinates.
- **`ROSC`** (`X = "!Q*A"`, `Q = "Q*B+X"`) — the case the fold **does** perform: `Q` already self-holds,
  so folding the relay `X` into it re-expresses an existing register rather than inventing one. The
  oscillation survives in `Q`'s own self-loop (`δ_Q = !Q` at `A·!B`) and is still flagged; only the
  folded-away relay leaves the reported group (`{Q, X} → {Q}`). That is correct — the group is the
  genuine memory coordinates that oscillate, not the relays feeding them.

So the guard refuses a fold **only** when the consumer forms an `s ↔ c` 2-cycle *and does not already
self-hold* — i.e. only when the fold would create a *new* self-reference (the emergent-memory case). A
relay folded into an existing register is allowed. The full algorithm is documented in
`state-space-minimisation.md`, and the soundness argument (I1–I4) lives in the `src/logic/minimise.rs`
module doc — neither is repeated here.

## 4. The machine's state as a minterm

A machine state assigns a concrete `0`/`1` to every input and, to every state variable, either a concrete
`0`/`1` or **absent** — the don't-care `-`, an as-yet-undetermined coordinate. It is a `Minterm<Symbol>`
over the columns `[inputs…, state-vars…]`. The power-on state fixes the inputs and leaves every state
variable absent (`machine.rs:5-11`).

A state's successor is derived on demand by evaluating the fixed δ_v (§3) against it (§5).

For the mutex a state is written here as `(A B | Qa Qb)`, e.g. `(1 1 | 0 1)`; an absent state variable is
shown as `-`.

## 5. Settling a state, and how "stable" is decided

This is the heart of the engine. Everything rests on one fact: **for fixed inputs, one settle round is a
deterministic function `step : State → State`.**

### One round: `step`

`machine::step(deltas, node)` (`machine.rs:64`, private) computes, for each state variable `v`,

```
v' = δ_v.evaluate(node).ok()      // Some(true/false), or None
```

and returns a new state with the inputs unchanged and each state field replaced by its `v'`.

Two properties make this well-defined:

1. **`Bdd::evaluate` returns `Ok(v)` only when the state's fixed columns force δ_v; otherwise it returns
   `Err`, and that `Err` is the expected, load-bearing case.** A state need not fix all of δ_v's support:
   an absent state variable leaves the value undetermined, so `evaluate` returns `Err`, and `step` writes
   that column **absent** (`.evaluate(node).ok()`, `None` ⇒ `-`, `machine.rs:79-87`). Returning `Err` is
   not an error path — it is how a state variable that the inputs (and the resolved state so far) do not
   yet determine stays absent (`machine.rs:6-11`).
2. All `v'` are read from the **same** current state before the new one is built, so `step` is a genuine
   parallel update, not order-dependent.

### Fixpoint = `step(state) == state`

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
the new state: it checks `step(deltas, node) == node`. It is a unit-test helper, not part of the
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
oscillating group.

Example — mutex under `A=B=1` from `(1 1 | 0 0)`:

```
(1 1 | 0 0) → (1 1 | 1 1) → (1 1 | 0 0) → …
```

The second `(1 1 | 0 0)` is already in `pos` ⇒ `Err` ⇒ `settle` yields `None` ⇒ no arc. The same
equality/visited machinery that confirms a fixpoint also rejects the states that don't have one.

## 6. Deriving arcs by re-walking the shared exploration

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
2. **BFS.** From each stable state, toggle **one input at a time** (`machine::toggle`), hold the state,
   and `settle`. A `None` (oscillation) transition is skipped. Newly reached settled states are enqueued,
   with the predecessor recorded in `Explored::prev` for path reconstruction (`machine.rs:292-305`).

`explore` returns an `Explored { order, prev }`, shared by both `arcs::derive` and `confluence::derive`.

### The arc emission: `arcs::derive`

`arcs::derive` (`arcs.rs:70`) re-walks `ex.order` only and emits arcs (`arcs.rs:96-146`):

- **Emit an arc** wherever a single input toggle flips an **output**'s value (a state output reads its
  own coordinate; a combinational output is its δ evaluated at the state). The toggled input is the
  `related` pin — so a related pin is **always a primary input**; outputs and internal signals never are.
- **Prevector.** The arc's prevector is the BFS path from a start state to the source state, each state
  projected onto the inputs. It is reconstructed by `Explored::path_to` (`machine.rs:142-151`), which
  walks predecessors back to a start, reverses, and projects each step onto the input names via
  `Minterm::project_to(input_names)` (`machine.rs:150`). Since the path drives every state variable —
  internal ones included — into the measured start state, it establishes hidden state such as a flop's
  master before the clock edge.
- **Dedup.** The same arc can be reached from several start candidates; `derive` keeps the one with the
  **shortest prevector** (`arcs.rs:96-141`).

Because arcs are found by *reaching* states and *settling*, the correctness properties are structural,
not bolted on: related pins are inputs, impossible arcs are never reached (they oscillate), and
input-forced transitions cascade naturally through the multi-round settle.

### The shared machine pass: `analysis.rs`

The whole setup happens once, in `analysis.rs`, over the **minimised** model. `Machine::build`
(`analysis.rs:74`) takes the cell's shared per-cell BDD map (minted once in `Cell::analyse`,
`model.rs:290`, and reused here — no rebuild): each state variable's δ and each combinational output's
δ are **direct lookups** into that map, and it runs the **one** `machine::explore` BFS seeded from all
of them. `analyse_machine` (`analysis.rs:167`) receives the same shared map and derives **both**
`arcs::derive` and `confluence::derive` from the shared `Machine`.

A combinatorial blow-up guard, `MAX_MACHINE_VARS = 22` (`analysis.rs:50`), gates the whole shared pass:
`Machine::build` returns `None` — leaving the cell unexplored, so arcs *and* hazards come back empty —
when `inputs + state variables` exceeds it (`analysis.rs:94`); the width is now the **minimised**
state count, so folded relays no longer count against the budget.

## 7. Worked example: discovering `B↓ → Qa↑` on the mutex

`MUT`: `Qa = !Qb·A`, `Qb = !Qa·B`. We trace the arc `-related_pin B -pin Qa` (rise), whose emitted block
is `-prevector {01 11} -vector {1 F R X}`.

**Start.** Start states are discovered from the signals' forced covers (§6), not from an all-zero reset.
`δ_Qb = !Qa·B` is forced high by `B` alone, so `N_B = (0 1 | 0 1)` — *B holds the grant* — is one of the
seeded start states, reached directly with `prev[N_B] = None`. (When the BFS later tries to enter it from
another state the entry is already occupied, so it stays a start.) Seeding this state also emits the plain
`B → Qb↑` arc.

**Walk A in.** From `N_B`, toggle `A`: `(1 1 | 0 1)` is already stable, giving
`N_AB2 = (1 1 | 0 1)` — *both requested, B still owns it*. No output flipped, so no arc, but it is a
reachable state. `prev[N_AB2] = N_B`.

The path `N_B → N_AB2` projected onto the inputs is the prevector **`01 11`**: it drives the
machine into "B owns the grant, A contending" — the only precondition under which releasing B can hand
the grant to A.

**The measured edge.** Dequeuing `N_AB2 = (1 1 | 0 1)`, toggle `B` (hold `Qa=0, Qb=1`, set `B=0`) and
settle:

| round | state | δ_Qa = !Qb·A | δ_Qb = !Qa·B |
|-------|-------|--------------|--------------|
| start | `(1 0 \| 0 1)` | !1·1 = **0** | !0·0 = **0** |
| →     | `(1 0 \| 0 0)` | !0·1 = **1** | !0·0 = **0** |
| →     | `(1 0 \| 1 0)` | !0·1 = 1 | !1·0 = 0 → **fixpoint** |

Settled `(1 0 | 1 0)`. The two micro-steps are the physical cascade: B drops → Qb falls; with Qb=0 and
A=1, Qa rises.

**Emit.** Across the toggle at `N_AB2 → (1 0 | 1 0)`, `Qa: 0 → 1` (rise). Arc: `related = B`, `pin = Qa`,
edge Rise, `start = 11`, `end = 10`, `prevector = 01 11`, vector `{1 F R X}`. (The same step emits
`B → Qb↓`.)

`B` does not appear in `Qa`'s own function. The `B↓ → Qa↑` dependence arises from *reaching* the state
where `B` holds the grant and *settling* through the transient `(Qa=0, Qb=0)`: the search toggles `B` and
the parallel `settle` propagates `Qb↓ ⟹ Qa↑`. The dependence is an emergent property of the reachable
state graph.

The mirror arc `A↓ → Qb↑` is discovered symmetrically: `N_A = (1 0 | 1 0)` is a seeded start, `A` walks in
to `N_AB1 = (1 1 | 1 0)`, and dropping `A` there gives prevector `10 11`.
