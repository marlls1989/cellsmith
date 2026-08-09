# The state-machine arc engine

How cellsmith derives Liberate timing arcs for state-holding cells (C-elements, latches, SR pairs,
mutexes/arbiters, flip-flops with internal state). This document explains the model, the state machine,
how a state is settled to a fixpoint, and how arcs are discovered — with a full worked example.

The functional state-table view of the same signals is documented separately in
`state-table-regions.md`. The hazard vocabulary this document leans on — the two detected hazards
(order-dependent and oscillation) and the timing constraints generated from them — is set out in
`hazard-detection.md`, and the one-shot state-space minimisation it relies on in
`state-space-minimisation.md`.

## 1. The model

A cell is a **name**, an ordered list of **inputs**, a Boolean **function per output**, and optionally
some **internal** functions. Two rules make state work with no special ceremony:

- **Any signal name referenced inside a function is that signal's feedback/delayed value.** A C-element
  referencing `Q`, an SR pair referencing each other, and a flop's slave referencing its master are all
  ordinary references.
- An **internal** signal is a first-class signal that other functions may reference but which emits
  **no external pin** — it models hidden state such as a flip-flop's master latch.

From these functions the cell is treated as an **asynchronous state machine** over
`inputs × coordinates`, the coordinates being the signals that survive the minimisation (§2.1). The rest
of this document makes that machine precise: §2 gives the whole
picture and defines the vocabulary; §3–§5 detail how the machine is built and run; §6–§7 derive the arcs.

## 2. The machine at a glance

Before any construction detail, the whole pipeline in one view. For a cell, cellsmith:

1. **minimises** the cell's signal model once — collapsing alias/complement chains and folding guarded
   combinational relays — so only genuine memory coordinates remain (§3, §3.1);
2. takes every surviving signal as a **coordinate** of the machine, classifying each as a **state
   variable** (it holds state) or **combinational** (it does not);
3. reads, directly from the minimised model, a fixed **transition function** δ — one component δ_v per
   coordinate;
4. **settles** a state by repeatedly *evaluating* δ until the state stops changing (a fixpoint);
5. **explores** the reachable settled states, toggling one input at a time;
6. **derives arcs** by re-walking that exploration and watching which input toggles flip an output.

### 2.1 Definitions

Every term the later sections lean on, pinned here before first use.

- **Signal** — an **output** (emits an external pin) or an **internal** (no pin). Both are Boolean
  functions that may reference inputs and other signals; a referenced signal is that signal's
  feedback/delayed value (§1).
- **Reference graph** — each signal mapped to the signals its function names.
- **State variable** — a signal that lies on a **feedback cycle**: it reaches itself in the reference
  graph, whether by a self-loop (`Q = A·B + Q·(A+B)`) or a larger coupling cycle (`Qa = !Qb·A`,
  `Qb = !Qa·B`). Classification runs on the **minimised** model: minimisation (§3.1) first collapses
  every alias/complement chain onto one representative coordinate and composes every non-self-holding
  relay into its consumers — refusing only a fold that would *fabricate* a register (an emergent-memory
  2-cycle) — so self-reachability afterwards counts only genuine memory, never a spent wire or relay.
  The state variables are then found structurally over that minimised map: take the reference graph's
  transitive closure, and a signal `s` is a state variable iff **`s` reaches itself**.
- **Combinational signal** — a signal on **no** feedback cycle. Its value is fixed by the inputs and the
  state variables, so the minimisation composes it away (§3) unless something downstream addresses it by
  name — an external output pin, or an internal node the spec lists in `expose`. **"Combinational" means
  "off every cycle," not "a function of inputs only"**: a combinational signal may reference state
  variables and so depend on the current state — it simply is not itself a piece of held state (see the
  ICM cell's `GCLK` output in §3).
- **Coordinate** — a signal surviving the minimisation, hence a column of the machine's state: the state
  variables together with the combinational signals kept beside them. Both kinds are stepped by the same
  round (§5) and read the same way — a signal's value at a state is that state's column. They part
  company only in what *holds*: a state variable is memory and may be **uninitialised**, while a
  combinational coordinate is in lockstep with the state variables, taking whatever value they force. So
  only the state variables are counted where the question is how much of the cell's memory a candidate
  start state resolves (§6).
- **State (of the machine)** — an assignment of a concrete `0`/`1` to every input and a concrete
  `0`/`1`-or-**absent** to every coordinate, represented as a minterm over the columns
  `[inputs…, coordinates…]` (§4).
- **Transition function δ** — the machine's next-state map, given as one **component δ_v per
  coordinate**: a fixed Boolean function of `inputs ∪ state-variables` yielding v's next value (§3). δ is
  the standard automata-theory transition function, not a difference/delta of states.
- The verbs, in increasing scope:
  - **substitute** — the atomic step: replace one referenced signal name by its definition, **at most
    once**.
  - **compose** — perform one substitution, `f[v := g]` — used directly for a guarded relay fold and,
    batched, for an alias/complement class rename (§3.1).
  - **minimise** — drive substitution to a fixpoint **once**, before the machine is built: collapse
    alias/complement chains and fold non-self-holding relays into their consumers. Each δ_v is then the
    minimised model's own function for `v`, read directly; no per-signal composition remains once the
    machine is built.

## 3. Constructing the transition function δ_v

Every state variable's δ_v is already determined by the time the machine is built. Minimisation rewrites
the signal model **once**, before the machine is constructed, so that every surviving signal's function
is expressed purely over primary inputs and the surviving state variables: building the machine then
reads each state variable's function and each combinational output's function directly, and the settle
and explore passes (§5–§6) only evaluate them. Constructing δ_v is therefore a direct lookup, not a
per-analysis composition.

The model was folded by two staged discriminators run to a fixpoint (the algorithm is documented in full
in `state-space-minimisation.md`; §3.1 below covers the safety guard): **M1** collapses an
alias/complement chain — a signal whose function is *exactly* another signal or its negation — onto one
representative coordinate; **M2** composes a non-self-holding relay into each of its consumers and drops
it, refusing only a fold that would *fabricate* a register — an `s ↔ c` 2-cycle whose consumer does not
already self-hold (emergent memory).

Three examples, each isolating one point.

**A combinational signal is composed away.** Take a flop whose data is an internal AND cone (an
illustrative cell), DFFG, with inputs `CLK`, `A`, `B`:

- `D = A·B` — on no cycle → combinational → folded away
- `M = !CLK·D + CLK·M` — references `M` → self-loop → state variable → kept
- `Q = CLK·M + !CLK·Q` — references `Q` → self-loop → state variable → kept

State variables are `{M, Q}`; `D` is combinational — and, being a relay whose only consumer is `M`, it is
folded away entirely before the machine is even built (an M2 fold: `D` does not self-hold, and `M` is not
in `D`'s own support). `M`'s entry in the minimised model already reads:

- **δ_M = !CLK·(A·B) + CLK·M**  (`D` folded away; `M` kept; `CLK`, `A`, `B` are inputs)

`D` has vanished from δ_M — that is a composition actually firing, just done once, upstream of the
machine pass, rather than per analysis.

**A state variable is kept.** The plain mutex `Qa = !Qb·A`, `Qb = !Qa·B` — both signals are state
variables, so nothing is folded away and each δ keeps its cross-coupled peer:

- **δ_Qa = !Qb · A**  (`Qb` kept; `A` is an input)
- **δ_Qb = !Qa · B**  (`Qa` kept; `B` is an input)

**A combinational output that depends on state.** The real `ICM` cell is written with eight internal
signals, all mutually coupled on one dependency cycle — but two of them, `sela` and `selb`, are
combinational relays on the enable loop (a selection interlock ahead of each synchroniser), each with a
single consumer and no self-reference. Minimisation folds both into their consumers (`sela` into `sela1`,
`selb` into `selb1`) before the machine is built, leaving **six** surviving coordinates — `sela1`,
`sela2`, `enA`, `selb1`, `selb2`, `enB` — so the machine width drops from 5 inputs + 8 state vars = 13 to
5 + 6 = **11**. One output:

- `GCLK = enA·CLKA + enB·CLKB`

Nothing references `GCLK`, so it is on no cycle → **combinational**. Yet it references the state variables
`enA`, `enB`, so its δ keeps them:

- **δ_GCLK = enA·CLKA + enB·CLKB**  (`enA`, `enB` kept as state coordinates; `CLKA`, `CLKB` are inputs)

This is the concrete proof that "combinational" ≠ "function of inputs only": δ_GCLK genuinely depends on
the current state. `GCLK` carries an output pin, so it survives the minimisation and is a coordinate of
the machine: its column is stepped by δ_GCLK alongside `enA` and `enB`, and arc derivation reads its
value from that column exactly as it reads a state output's.

### 3.1 The safety guard: why folding must not fabricate a register

Collapsing a non-self-holding relay is safe — *unless* the fold would invent memory that wasn't there.
The machine detects **oscillating states** (an oscillation hazard); the guard exists to keep an
oscillation from being projected away, not to freeze the set of nodes that participate in it.

- **`MUT`** (§7 below) — the case the fold must **not** perform: `Qa = !Qb·A`, `Qb = !Qa·B`. Neither
  signal self-holds, yet each is the other's consumer — an `s ↔ c` 2-cycle. Folding `Qa` into `Qb`
  gives `δ_Qb = Qb·B + !A·B`, which at `A=B=1` is `δ_Qb = Qb` — a *fabricated* register. The oscillating
  `(0,0) ↔ (1,1)` cycle (`hazard-detection.md` §5) lived in the *disagreement* of the two nodes;
  projected onto `Qb` alone it lands on `Qb`'s stable states and disappears. The pair must stay two
  coordinates.
- **`ROSC`** (`X = !Q·A`, `Q = Q·B + X`) — the case the fold **does** perform: `Q` already self-holds,
  so folding the relay `X` into it re-expresses an existing register rather than inventing one. The
  oscillation survives in `Q`'s own self-loop (`δ_Q = !Q` at `A·!B`) and is still flagged; only the
  folded-away relay leaves the reported group (`{Q, X} → {Q}`). That is correct — the group is the
  genuine memory coordinates that oscillate, not the relays feeding them.

So the guard refuses a fold **only** when the consumer forms an `s ↔ c` 2-cycle *and does not already
self-hold* — i.e. only when the fold would create a *new* self-reference (the emergent-memory case). A
relay folded into an existing register is allowed. The full algorithm is documented in
`state-space-minimisation.md`, and the soundness argument (I1–I4) accompanies the minimisation itself —
neither is repeated here.

## 4. The machine's state as a minterm

A machine state assigns a concrete `0`/`1` to every input and, to every coordinate, either a concrete
`0`/`1` or **absent** — the don't-care `-`, an as-yet-undetermined coordinate. It is a minterm over the
columns `[inputs…, coordinates…]`, the state variables first and the combinational survivors after them.
The power-on state fixes the inputs and leaves every coordinate absent.

A state's successor is derived on demand by evaluating the fixed δ_v (§3) against it (§5).

For the mutex a state is written here as `(A B | Qa Qb)`, e.g. `(1 1 | 0 1)`; an absent state variable is
shown as `-`.

## 5. Settling a state, and how "stable" is decided

Everything rests on one fact: **for fixed inputs, one settle round is a deterministic map on states.**
Call that round *step*.

### One round: *step*

One round computes, for each coordinate v — state variable and combinational survivor alike — the value

- v′ = δ_v evaluated at the current state — a definite `0` or `1` when the state's fixed columns force
  δ_v, otherwise undetermined

and returns a new state with the inputs unchanged and each coordinate field replaced by its v′.

Two properties make this well-defined:

1. **Evaluation yields a definite value only when the state's fixed columns force δ_v; otherwise the
   value is undetermined, and that undetermined case is expected, not an error.** A state need not fix
   all of δ_v's support: an absent state variable leaves the value undetermined, so the round writes that
   column **absent**. Leaving a column undetermined is not an error path — it is how a coordinate
   that the inputs (and the resolved state so far) do not yet determine stays absent. A combinational
   coordinate lands there whenever the state variables its δ reads are themselves still absent.
2. All v′ are read from the **same** current state before the new one is built, so the round is a genuine
   parallel update, not order-dependent.

### Fixpoint = *step*(state) == state

Settling iterates the round: compute the next state; if it equals the current state, that state is a
fixpoint and settling stops. The comparison is plain minterm equality. **One match is enough to stop, and
this is exact, not heuristic:** the round is deterministic and pure, so

> *step*(x) = x   ⟹   *step*ⁿ(x) = x   for all n.

A fixed point reproduces itself under every further application — iterating again cannot change anything.
That is the whole reason settling need not "keep going to be sure." The fixpoint **may still leave state
variables absent** — those the inputs and resolved state do not determine.

Settling may also be requested in a form that discards the cycle detail and simply yields no state when a
state never settles.

### The one-round stability test

The same test without building the new state — does one round map the state to itself? — decides
stability directly. It is a checking aid, not part of the production walk.

Example — settled mutex state `(1 0 | 1 0)`:

- δ_Qa = !Qb·A = !0·1 = **1** = current Qa ✓
- δ_Qb = !Qa·B = !1·0 = **0** = current Qb ✓

Both coordinates already agree ⇒ the round maps it to itself ⇒ stable.

A mid-cascade state `(1 0 | 0 1)` fails, so another round is required:

- δ_Qa = !1·1 = 0 = current 0 ✓, but
- δ_Qb = !0·0 = 0 ≠ current 1 ✗

### Termination and oscillation

What if a state never settles? For fixed inputs the reachable state space is **finite** (each state
variable is `0`, `1`, or absent) and the round is deterministic, so the trajectory
`s → step(s) → step²(s) → …` must eventually **repeat** a state; once it does it is periodic and can
never reach a fixpoint.

Settling detects that by recording the index at which each visited state first appeared alongside the
sequence of visited states in order. When a round produces a next state already seen at index p, the
trajectory slice from p onwards — from the first revisited non-fixpoint — is the periodic cycle.

When settling is asked only for a stable state, that periodic outcome means "no stable state": the
trajectory is an **oscillation**, and the BFS simply drops that transition (so no impossible arc is
fabricated). This is distinct from a settled state that still carries an absent coordinate — an absent
coordinate is an **uninitialised** state variable, not a non-settling trajectory. Hazard detection
instead keeps the cycle: probing a reachable stable state with a simultaneous multi-input toggle (a
mutex's requests co-asserting) and finding a cycle rather than a fixpoint names the varying state
variables as an oscillating group — an oscillation hazard (`hazard-detection.md`).

Example — mutex under `A=B=1` from `(1 1 | 0 0)`:

`(1 1 | 0 0) → (1 1 | 1 1) → (1 1 | 0 0) → …`

The second `(1 1 | 0 0)` is already recorded ⇒ periodic ⇒ no stable state ⇒ no arc. The same
equality/visited machinery that confirms a fixpoint also rejects the states that don't have one.

## 6. Deriving arcs by re-walking the shared exploration

Arc derivation does not run its own search. The reachable stable states are discovered once, by a single
exploration, and arc derivation re-walks them.

### The exploration

The exploration both discovers the start states and runs the BFS:

1. **Start candidates — never an assumed all-zero state.** For each seed function (each state variable's
   δ plus the combinational outputs' δ) it takes the forced on/off cover over the inputs: input
   assignments that force the signal *regardless of* the undefined power-on state. These input minterms
   are pooled, then **ranked by how many state variables they settle** — the state variables alone, since
   what the ranking measures is how much of the cell's memory a candidate resolves (ties broken toward
   state nearest the inputs). Each candidate input is widened onto the full `[inputs…, coordinates…]`
   columns — every coordinate column arrives **absent**, and settling is what first gives a combinational
   one its value — and settled. A state-holding cell whose reset is an input *sequence*
   rather than a level is therefore initialised by the sequence that actually resolves it, not by an
   arbitrary held combination.
2. **BFS.** From each stable state, toggle **one input at a time**, hold the state, and settle. An
   oscillation transition (no stable state) is skipped. Newly reached settled states are enqueued, with
   the predecessor recorded for path reconstruction.

The exploration returns the discovery order and the predecessor map, shared by both arc derivation and
hazard detection.

### The arc emission

Arc emission re-walks the discovery order only and emits arcs:

- **Emit an arc** wherever a single input toggle flips an **output**'s value (every output is a
  coordinate, so its value at a state is that state's column). The toggled input is the
  `related` pin — so a related pin is **always a primary input**; outputs and internal signals never are.
- **Prevector.** The arc's prevector is the BFS path from a start state to the source state, each state
  projected onto the inputs. It is reconstructed by walking predecessors back to a start, reversing, and
  projecting each step onto the input names. The path reaches the measured start state in cellsmith's own
  model; the start condition reaches Liberate through `-ic`, which names the level of every `-pinlist`
  entry — including any node listed in `expose`, such as a flop's master.

Every context a firing can happen in yields its own arc: an arc's identity here is its output, its
related pin, the direction, and the **full machine start state**, so two firings that agree on the
inputs but differ in internal state are two arcs, each with its own prevector, and both are derived.

Whether both are *emitted* is a separate question. A block reaches only its `-pinlist` columns, so two
such arcs render the same block wherever the state that separates them has no column of its own; the
cell states that block once and reports the arcs it conflates, naming each of their states so the node
worth adding to `expose` can be read off them. A cell's rest states conflate the same way: two rest
states sharing every column a `define_leakage` block reaches still differ in a state variable no column
names, render one block between them, and are reported through the same channel, naming each rest
state's full machine state.

Because arcs are found by *reaching* states and *settling*, the correctness properties are structural:
related pins are inputs, impossible arcs are never reached (they oscillate), and input-forced transitions
cascade naturally through the multi-round settle.

### The general pass

Derivation hands the emitter (`src/emit/arcs_tcl.rs`) every firing it found; the emitter is where the
grain of the generated `.tcl` is decided. Each arc class is emitted in two passes.

The **general pass** always runs. It groups the class's derived arcs by **transition** — the output pin
and the edge it makes, the related pin and the edge IT makes — together with the `-type` the arc
classifies as, and emits **one representative** per group, with no `-when` line. The block so emitted
generalises over what the group's members differ in: the side inputs' held levels, the held outputs, and
the internal state the firing was measured from. The representative is a member with the **strictly
shortest prevector** — where several tie at the minimum, any one of them is an equally valid
representative of the group at this grain — and it renders its own concrete `-ic` and `-vector`, those
of one real firing rather than a synthesised context.

Selecting the class (`--when`, or the per-cell `when` key) adds a **conditioned pass** on top: every
derived arc of that class comes back as its own block carrying its own condition, so a firing can appear
twice — once as its transition's general representative, once with its `-when` line.

Because `-type` is part of the grouping, a transition that classifies **`edge` from one machine start
state and `combinational` from another** falls in two groups and emits **both** blocks unconditionally.
That is the intended output: `-type` declares the arc's nature to Liberate and is decided per firing, so
collapsing across it would drop one of the two kinds from the generated library.

### The leakage blocks

A `define_leakage` block measures the cell's **static leakage at one rest state** — a single settled
point of the exploration, not a transition into it. The unit a block states is the rest state itself,
not the input assignment that reaches it: two rest states can share an input assignment while differing
in what the cell holds internally — a bistable's whole point — so stating the input assignment alone
would conflate them.

A rest state the inputs alone drive the cell into is stated by its `-when`: the inputs held there and
the level every output settles at, and the block is that one line. A rest state the cell reaches only
through a sequence of input changes — one it must be walked into — states itself through the block's own
columns instead. `-pinlist` names the inputs, then the cell's exposed internal nodes, then the outputs,
and `-vector` holds every one of those columns at the level the rest state carries. Every column is a
level, because what the block measures is a settled point rather than a transition through one.

The exposed columns are what tell apart two rest states sharing an input assignment: an internal node
has no pin, so a `-when` cannot name it, and its `-vector` column states it. Two rest states differing
only in a node the cell does not expose render one block between them, conflated and reported as above.

The BFS walk that reaches a rest state is internal to the model: `LeakageState` identifies the state by
it, and it is what distinguishes a rest state the inputs drive the cell into from one the cell is
walked into.

### The shared machine pass

The whole setup happens once, over the **minimised** model. Building the machine takes the cell's shared
per-cell signal map (minted once when the cell is analysed, and reused here — no rebuild): each state
variable's δ and each combinational output's δ are **direct lookups** into that map, and it runs the
**one** exploration BFS seeded from all of them. The one shared machine is what `analyse_machine` draws
every derivation off: the transition and hidden arcs, the detected hazards, the constraints that remedy
them, the edge-register classification, and the leakage states.

Two exploration budgets gate the whole shared pass, each charged against work the pass actually performs
rather than the cell's declared shape (a cell is not turned away for having many inputs or many state
variables): the **candidate** budget bounds the seed minterms the candidate pool expands the signals'
forced on/off covers into, before ranking and seeding the BFS; the **state** budget bounds the reachable
stable states the BFS records in `Explored::order`. Exceeding either leaves the cell unexplored — arcs
*and* hazards come back empty for it — and is reported as a hard error naming the cell; each budget is
raised for a run with its own flag, `--max-candidates` or `--max-states`.

## 7. Worked example: discovering `B↓ → Qa↑` on the mutex

`MUT`: `Qa = !Qb·A`, `Qb = !Qa·B`. We trace the arc from related pin `B` to pin `Qa` (rise), whose
emitted block carries `-vector {1 F R X}`, its start condition stated by `-ic`.

**Start.** Start states are discovered from the signals' forced covers (§6), not from an all-zero reset.
`δ_Qb = !Qa·B` is forced high by `B` alone, so `N_B = (0 1 | 0 1)` — *B holds the grant* — is one of the
seeded start states, reached directly as a start with no predecessor. (When the BFS later tries to enter
it from another state the entry is already occupied, so it stays a start.) Seeding this state also emits
the plain `B → Qb↑` arc.

**Walk A in.** From `N_B`, toggle `A`: `(1 1 | 0 1)` is already stable, giving
`N_AB2 = (1 1 | 0 1)` — *both requested, B still owns it*. No output flipped, so no arc, but it is a
reachable state, and its predecessor is `N_B`.

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
edge Rise, `start = 11`, `end = 10`, prevector `01 11` (model only), vector `{1 F R X}`. (The same step emits
`B → Qb↓`.)

`B` does not appear in `Qa`'s own function. The `B↓ → Qa↑` dependence arises from *reaching* the state
where `B` holds the grant and *settling* through the transient `(Qa=0, Qb=0)`: the search toggles `B` and
the parallel settle propagates `Qb↓ ⟹ Qa↑`. The dependence is an emergent property of the reachable
state graph.

The mirror arc `A↓ → Qb↑` is discovered symmetrically: `N_A = (1 0 | 1 0)` is a seeded start, `A` walks in
to `N_AB1 = (1 1 | 1 0)`, and dropping `A` there gives prevector `10 11`.
