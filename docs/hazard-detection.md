# Hazard detection and constraint generation

How cellsmith detects the hazards of a state-holding cell and generates the timing constraint
(setup/hold, non-sequential, or minimum pulse width) that avoids each one. A hazard is classified on two
independent axes — what its timing is between, one of three causes, and what the machine then does,
settling indeterminately or oscillating. Detection and emission are two separate layers: detection walks
the reachable-state machine and reports every hazard it observes there; emission decides which of those
reports becomes a rendered block, and states the timing that removes it.
Both run on the same reachable-state machine that drives arc discovery.

This is a companion to `state-machine-arc-engine.md`, which covers the model, the next-state functions δ,
settling, and the reachability exploration; everything here builds on those. The functional state-table
view of the same signals is documented separately in `state-table-regions.md`.

## 1. Two axes, three causes

A **delay arc** records that a single input edge causes an output edge. A hazard instead involves timing
the cell cannot settle around cleanly: a single input's own edge whose cascade never converges, one edge
on each of two inputs landing close together, or the two edges a single input makes on its own. Such
timing can drive the cell into **metastability**: an unresolved condition the cell cannot leave cleanly.
Detection finds the occasions where that risk is real, and classifies each on two independent axes.

**What the timing is between** — the *cause*:

- **A toggle** — one input's edge observed on its own, its cascade ringing around the cell's own feedback
  instead of settling: there is nothing else for it to race, so there is no separation for a constraint to
  state, only the ring itself. The pin involved is its **racer**, carrying the edge it makes.
- **A race** — two inputs' edges landing close together. The pins involved are its **racers**, each
  carrying the edge it makes.
- **A pulse** — one input racing itself. A pulse on input `p`, applied from a stable state, is `p` toggled
  (the **opening edge**), the cascade that toggle opens left to run some distance, and `p` toggled back
  (the **closing edge**) before the cell settles again.

**What the machine then does** — the *outcome*:

- **Indeterminate** — the cell settles, but which state it settles to is not determined by the cause
  alone.
- **Oscillation** — the cell never settles: instead of reaching a **stable state** — Huffman's term,
  defined in `state-machine-arc-engine.md` §5 — it walks a periodic cycle.

The two axes are independent, so a hazard is one of the three causes settling indeterminately or
oscillating instead. A toggle has no second edge to disagree with, so it is recorded only when it
oscillates — the ring around the cell's own feedback never dies out. A race settling indeterminately has
the settled state depend on which of its two edges lands first; a race that oscillates has the two edges
landing at once drive the state into a periodic cycle. A pulse settling indeterminately has the settled
state depend on how far apart the pulse's two edges are — a pulse too narrow to carry the cell through
settles it somewhere a wider one does not; a pulse that oscillates has closing the pulse mid-cascade leave
the cell ringing rather than at rest.

**Metastability is the shared physical risk every cause and outcome carries** — the reason a remedy is
needed — not a hazard of its own and not a name for any one of them alone.

A **constraint** is that remedy. It is **generated from** a detected hazard; it is never itself a hazard
and never itself detected. It states the timing the risky situation cannot arise under — for a race, the
separation between the two edges, directed *setup/hold* if the pair contains a declared clock and
symmetric *non-sequential* otherwise; for a pulse, the *minimum pulse width* its two edges must stand
apart by (§7). Detection names what puts the cell at risk; the constraint quantifies the timing that
removes it.

Two things are deliberately **not** hazards:

- An **undefined state variable is uninitialised** — a bistable at an *unknown* state, not a value
  and not a third logic level, carrying none of the metastability risk a genuine hazard does. Such a state
  seeds traversal only: nothing is concluded from it, so no probe starts there (§2).
- **An arbiter's grants settling differently depending on which request arrived first is its function, not
  its fault**: a mutual-exclusion element is supposed to grant whichever request arrived first. Its hazard
  is the oscillation when the requests tie — which is why a race settling indeterminately and a race that
  oscillates must be told apart rather than lumped together as "the results differ" (§4).

## 2. Everything starts from the reachable states

Detection does not run the exploration itself — it re-walks the *shared* exploration, the same one
the arc discovery uses, built once with the same on/off cover seeding and the same
single-input-toggle edges. It probes hazards **only from the fully-initialised reachable stable
states** — the same measurement eligibility the arc derivation applies: every state column
determinate, no don't-care.

That anchoring is the load-bearing design decision, and it has two halves. Held state is the product of
the cell's own sequential behaviour; the only joint assignments that mean anything are the ones the
dynamics can actually produce. Reachability here is intrinsic: every probe starts only from a state the
exploration actually reached, so state variables are never coerced to fabricated values and no hazard is
manufactured on a state the cell can never occupy. Determinacy is the other half: a race or an
oscillation is a property of a valid, fully-initialised machine, and a state carrying an uninitialised
bistable does not describe one — it is unknown, so nothing follows from it either way. Such a state stays
in the explored order as a traversal seed; it is never a probe's starting point. From an eligible
start the whole probe stays determinate: settling evaluates each δ over concrete inputs and state values,
so a total state steps to a total state and every outcome the probes compare is a value.

**The condition — what an emitted block calls its `when` — is the pre-transition input projection.** A
record's condition is the probed state projected onto the inputs: the standing input assignment the
transition happens *from*. That holds of every arc cellsmith writes, a delay arc and a hidden arc as much
as a constraint over a racing pair or a minimum pulse width, because the pins a probe toggles are the ones
the block writes as edges, and an edge is not part of the condition it fires under. So a mutex's ring is
filed under A=0·B=0, the idle state its two requests rise out of, not under the A·B they land in.

The exploration itself, however, never *reports* metastability: when a toggle fails to settle it silently
drops that transition (no impossible arc is fabricated) and moves on. More importantly, it never presents
the transition that makes an arbiter oscillate: its edges toggle one input at a time, and the trigger for
oscillation is precisely the violation of that single-change assumption — **two or more inputs changing
simultaneously** (a mutex's requests co-asserting). So detection must apply that change itself, as a probe
from each reachable state, which is what the next section does.

## Detection

## 3. The probes: single settles once per state, then the per-pair work

For each reachable stable state, detection settles each input's single toggle **once** and reuses it
across every pair — so the per-state single-settle cost is O(n) in the cell's input count, not
O(n²). Every settle either reaches a stable state or reveals a **periodic cycle**: the trajectory it
revisits, kept rather than discarded.

For each reachable stable state and each unordered input pair {x, y} (all other inputs held at their
values in that state), the pair-specific work is one simultaneous settle plus, when both single toggles
settled, two order follow-ups:

1. **x alone** — reused from the per-state singles
2. **y alone** — reused from the per-state singles
3. **x and y simultaneously** — the settle done per pair

and, when both single toggles settle, two follow-ups that complete the *orders*:

4. **x then y** — toggle y from x's stable state
5. **y then x** — toggle x from y's stable state

**Confluent** is term rewriting's word — the Church–Rosser property — and here it ranges over the
cell's settled machine states, reached from one settled state under a near-simultaneous pair of
input edges; the operation is settling after toggling x then y, in each order. Nothing beyond that
definition is relied on. The pair is confluent at that state when both orders land the machine in
the same state, so which of its two edges arrives first does not matter there.

The two order outcomes are then compared, and the simultaneous settle inspected for a cycle. The outcomes
classify as:

| Observation | Meaning | Reported as |
|---|---|---|
| the simultaneous settle returns a cycle | the pair tied and the state oscillates | a race that oscillates (§5) |
| a lone toggle never settles | nothing to race, ringing on its own | a toggle that oscillates, the one pin named (§5) |
| the two order outcomes agree | confluent — order does not matter here | nothing |
| the two order outcomes diverge, and the divergence *interacts* (§4) | order matters at this pair | a race settling indeterminately (§4) |
| the two order outcomes diverge, latch-mediated only (§4) | divergence real but design-tolerated | nothing |

These probes relate two inputs to each other. A pulse relates one input to itself, and is probed by
pulsing that input rather than by toggling a pair — §6 has those probes.

## 4. Race divergence and the combinational-neighbourhood filter

When both single toggles settle, detection compares the two order outcomes. If they agree, the pair is
confluent here and nothing is recorded. If they differ, the pair diverges — it might turn out to be a race
settling indeterminately — but divergence alone is **not** the verdict.

Global divergence of the joint state only means the two orders left *some* latch somewhere holding
different values — and for a cell with independent domains that is normal operation, not a pin-pair fault.
The divergence must **interact with the racing pair in the immediate combinational neighbourhood**:

> Divergence is reported as a race settling indeterminately only if some state variable w that actually
> differs between the two order outcomes has **both** x and y in the **direct support of its own δ_w**.

Why direct support is the right notion of "immediate neighbourhood": the model minimisation
(`state-space-minimisation.md`) composes through *combinational* logic only — a state variable is kept as
a variable, never substituted through. So both pins appearing in δ_w's support means they meet within one
combinational cone in front of a single latch: the race is physically present at that latch's input. If no
diverging w sees both pins, the divergence was mediated **across a latch boundary** — what crossed the
boundary is a *settled snapshot* of the earlier domain, not the live race — and the pin pair is not at
fault.

Worked example — a two-domain sampling chain: M1 = !C1·D + C1·M1 (so δ_M1 depends on {C1, D, M1}) and
Q = !C2·M1 + C2·Q (so δ_Q depends on {C2, M1, Q}). The (C1, C2) order-divergence is *real* — whether Q
latches M1's old value or D's new one depends on which latch closes first — but no single δ sees both C1
and C2: the divergence is carried across the M1 → Q latch boundary, so it is filtered. The (C1, D) race,
by contrast, meets directly in δ_M1 and survives as a genuine race settling indeterminately. On the ICM
dual-clock synchroniser (see `state-machine-arc-engine.md` §3 for the cell and its internal signals; the
same shape at scale) this filter reduces the reported hazards to the two same-domain pairs (CLKA, RA) and
(CLKB, RB) and removes the meaningless cross-domain clock-vs-clock ones.

Declassifying a relay can legitimately **surface** a race settling indeterminately that used to be
latch-masked. Once a combinational relay is folded into its consumer (`state-space-minimisation.md`), that
consumer's δ directly incorporates the relay's former support — so a pin pair that used to meet only
across a latch boundary can now land in the same direct support. On the ICM cell this is exactly what
happens: folding the selection-interlock relays sela/selb into sela1/selb1 extends each synchroniser's
direct support, so the cell gains the derived setup/hold pairs (CLKA, S) and (CLKB, S) alongside its
existing (CLKA, RA) and (CLKB, RB) — a genuine gain, never a loss, and consistent with the fold's own
soundness.

The filter is symmetric in principle: because it iterates over diverging state variables, folding a
cycle-resident relay could in theory also *drop* a pair whose divergence every consumer's settled value
masks — the mirror image of the gain, the tool re-deciding on the minimised model what counts as a
design-tolerated settled snapshot across a latch, a correction in the same sense rather than a regression.

The filter is also why an arbiter's constraint does not come from its divergence: a mutex's diverging
grants each see only *their own* request (δ_Qa depends on {A, Qb}), so its (A, B) divergence fails the
filter — correctly, since for an arbiter that divergence is function, not fault. The pair is instead
carried by the race that oscillates (§5), from which emission generates the constraint (§7).

## 5. When the pair ties: races that oscillate

A simultaneous settle that returns a **cycle** — a finite, deterministic transition that revisits a state
that is not stable, so periodic forever after — is reported as a race that oscillates: the cell never
settles, which is where the metastability risk arises. From the cycle the report is assembled:

- **group** — the state variables that actually oscillate: those whose *value* differs between any two
  nodes of the cycle. Variables that happen to sit still through the cycle are not blamed.
- **condition** — the probed state projected onto the inputs (§2): the assignment the pair toggles out of,
  rendered as a literal product, e.g. !A·!B.
- **stable outcomes** — the competing outcomes the oscillation is torn between: the settled results of the
  two *orders* (x then y, and y then x), each projected onto the group. For a mutex these are the two
  grants; simultaneity is exactly the boundary between the two orders, so the order outcomes are the states
  the cycle cannot choose between.

A bare oscillation record cannot by itself name the pins, edges and prevector its constraint needs, so
detection **records the racing pair with the hazard** — the two pins, their edges, and the path to the
probed state — one per pair-probe that oscillates. Emission (§7) turns that record into a constraint, and
it is why an arbiter's constraint originates here rather than from its (filtered) divergence. A toggle's
oscillation has no competing order to name a pair from — the record carries the one pin and its edge alone
— so it yields no constraint (§7).

Detection reports every reachable state at which a pair or a lone toggle is observed oscillating — one
record per probed state, not a single representative standing for the pair. That holds of every record
detection files, under either cause and either outcome: **detection deduplicates, ranks and selects
nothing.** Which of those records becomes a rendered block, and which of the rest are rendered alongside
it, is emission's decision (§8).

Worked example — the mutex Qa = !Qb·A, Qb = !Qa·B, from the idle state (0 0 | 0 0) (notation
(A B | Qa Qb)), pair {A, B}:

- A alone settles to (1 0 | 1 0); B alone to (0 1 | 0 1) — clean single grants, no hazard.
- Both at once: (1 1 | 0 0) → (1 1 | 1 1) → (1 1 | 0 0) → … — a period-2 cycle.
- Detected: a race that oscillates with group [Qa, Qb] (both oscillate), condition !A·!B — the idle state
  the pair rises out of — competing outcomes
  {Qa=1, Qb=0} and {Qa=0, Qb=1} (the two order outcomes, projected onto the group); the racing pair {A, B}
  recorded with it. From that pair emission generates the non-sequential constraint A↑/B↑.

A C-element, by contrast, never oscillates: it is bistable in its hold region, but that is self-feedback
holding a *settled* value — no probe from a reachable state makes it oscillate. Its A↓ racing B↑ is a
genuine race settling indeterminately (§4), not an oscillation.

## 6. Pulses: the reference, the candidates, and what the width decides

A pulse, as §1 defines it, is measured by its **width**, counted in settling rounds — one round being one
evaluation of every δ. Settling the opening toggle yields the trace t[0…last]: t[0] is the toggled state
itself, t[last] the stable state it settles to. Closing the pulse at **cut** i means toggling p back
at t[i] and settling from there, so cut i is the pulse i rounds wide and a wider pulse is a later cut. What
the cuts produce are the pulse's **outcomes**: the states they settle to, projected onto the nodes at
risk.

The cuts are not peers. The close at cut last — the one placed once the opening cascade has reached its
stable state — is the **reference**: after the cell has settled, closing now and closing three days
later are the same event, so that close is the behaviour a minimum pulse width is defined RELATIVE TO
rather than one outcome among several. Every earlier close is a **candidate**, the narrowest of them the
zero-width close, whose outcome is s itself. A hazard is a candidate that disagrees with the reference, or
a candidate that does not converge.

**The zero-width anchor.** Cut 0 is derived rather than probed. Closing there is toggling p back at the
state the opening toggle produced, and a toggle writes only the named input's column, so the closed state
is s itself — and s is stable, so the zero-width pulse settles to s. It is a no-op, and s enters the
outcome set as its member: one cut settling anywhere other than s is therefore already two outcomes, and
already the hazard.

**Every cut is walked, not the two ends alone.** The narrowest and the widest pulse are the trace's two
ends; an **interior** cut — one closing while the opening cascade is still in flight — closes onto a state
neither end reaches, and the outcomes found there take two shapes. One is a close that leaves the cell
with no rest state at all: a cross-NOR SR pair re-released onto the illegal both-low state rings, and a
mutex whose request is re-asserted after the first grant has dropped and before the second has been taken
rings the same way. The other is a partial capture — a cascade of latches on one clock phase where the
pulse was wide enough for the first stage to take the value and not for the second, settling between the
outcomes the two ends reach (`TCASC` in `examples/sequentials.toml`: three widths, three outcomes).

**A cut that never converges is its own record.** A cut whose close leaves the machine in a periodic
cycle reaches no rest state at that width — the cell is left ringing rather than at one of the states the
other widths settle to. It is filed as a pulse that oscillates, over the nodes the cycle moves, beside the
pulse settling indeterminately over the nodes the width decides. The two are one cause with two outcomes,
so one constraint covers both and probes the union of what each names (§7): a hazard whose only interior
cut rings still names what to probe.

Such a cut is **not** filed as a race that oscillates (§5). The cause is what the timing is between, and
here it is one pin's two edges: the ring is reached by closing the pulse partway through the opening
cascade, which no separation between two pins can forbid and only a wide enough pulse can. It is a pulse
that oscillates, its own cell of the grid (§1); the ringing is a property of the width, and the width is
what the constraint (§7) states.

An **opening** toggle that never settles is a different thing: it is the toggle that oscillates, which §5
already records — the machine never comes to rest for a second edge to be placed against, so there is no
pulse here to widen. No hazard is recorded for a pulse on that pin at that state; §5's record already
covers it.

Worked example — the master-slave flop M = !CLK·D + CLK·M (the master, transparent while CLK is low) and
Q = CLK·M + !CLK·Q (the slave, transparent while CLK is high), notation (CLK D | Q M):

- **CLK↑ from (0 0 | 1 0)**, a state whose slave holds a value its master does not. The opening toggle
  opens the slave; one round copies M into Q, reaching the stable state (1 0 | 0 0). Closing there
  leaves Q at 0, where the zero-width pulse leaves it at 1. The master holds through both (δ_M = M while
  CLK is high), so the width decides Q alone.
- **CLK↓ from (1 0 | 1 1)**, a state whose master holds a value D does not. The opening toggle opens the
  master; one round takes D into M, reaching (0 0 | 1 0). Closing there re-opens the slave, which then
  copies the new M into Q — so the wide pulse moves both nodes and the zero-width pulse moves neither,
  and the width decides {Q, M}.

## Emission

## 7. Situations, and the constraints generated from them

Emission is what turns detection's raw report into rendered blocks: first by generating a constraint for
every situation whose cause states a timing to hold, then by deciding how each of those constraints
renders — stating a constraint generally, or only in the context it was observed in (§8).

An **observation** is one hazard record: one probe, from one reachable stable state. A **situation** is
what one constraint is generated per, and a situation is the **cause**: the kind with the pins it names
and the edge each makes, plus the **starting state** the probe acted from. A cause is a starting state and
a transition; an *effect* is which node suffers what, and the effect is deliberately not part of the key.
Observations agreeing on the cause are independent readings of one probe — a pair that both diverges and
never settles, a pulse that both rings at one cut and disagrees with its reference at another — and
**collapse into one constraint**: the timing that removes one removes the other, so nothing is gained by
stating them twice. The victim nodes of the readings that meet there are **merged**, so the one constraint
probes every node any outcome of its cause attacks.

**Why the state is the key, and not the input condition it projects to.** Every arc kind in this tool is
keyed on the state its measurement is taken from, and a state-holding cell reaches one input assignment in
several stored states: a C-element sits at A=1·B=0 with its output held high or held low, and a flop sits
at CLK=0 with its master loaded and its slave either side of the value. Keying a constraint on the
condition would fold those together here, before emission ever saw them. Emission is where that fold
belongs: two constraints whose `-ic` and `-vector` cannot tell one state from the other render one block,
and the masked-arc warning names them — which is how a spec author learns that the nodes the cell exposes
do not distinguish two situations it is being asked to characterise. What to do about it is the author's
call, and it can only be made if the two reach that point separately.

Every situation whose cause states a timing to hold — a race naming two pins, or a pulse — generates a
constraint. Every **pair** constraint is built the same way, whichever outcome its situation shows:

- **Kind is decided solely by the declared clock.** A pair containing exactly one pin declared in the
  cell's clock list is a directed **setup/hold** (related = the clock, constrained = the data pin); any
  other pair is a symmetric **non-sequential** constraint. Clocks are *declared, never inferred*: inferring
  one from the race geometry is state-dependent — the same pins read one way from one held state and the
  other way from another — so it distinguishes nothing real.
- **Edges** are the directions the two pins toggle *from their values at the probed state* (a pin at 0
  races rising, at 1 falling).
- **Prevector** is the path from a start state to the probed state, each node projected onto the inputs —
  the same construction as a delay arc's prevector. It is a model quantity and reaches no emitted block:
  the constraint arc states its start condition through `-ic`, as every `define_arc` does. What the walk
  gives is the state itself, which the constraint carries as `Constraint::state` — every input and state
  variable at the level it holds there, including the internal nodes no column carries. The rendered
  human-readable form is the two switching edges plus any other inputs held fixed, e.g. A↓ & B↑ with R=0.

- **Victim nodes** are the state variables the hazard attacks — its `group`, the nodes whose settled
  value depends on the arrival order, sampled with the level each holds at the probed state, and unioned
  over every outcome the cause showed. A constraint carries them because the arc it renders measures them:
  they are what the constraint is about. *Victim* names what the hazard does to the node; whether the
  constraint succeeds in keeping it safe is Liberate's measurement to make, not this record's claim.

A race settling indeterminately takes its cause from the racing pins and the edge each makes; a race that
oscillates takes it the same way, from the racing pair its record carries (§5). Whether the constraints so
generated all reach the output, and as what, is §8's.

A **minimum-pulse-width** constraint relates one pin to itself, and carries the pulsed pin, the pulse's
**opening** edge alone — Liberate searches the width, so no closing edge is stated — the nodes the
hazard's width decides as its victim nodes, and the prevector, probed state and output levels sampled
just as a pair constraint's are. Two things a pair constraint has to decide are not decisions here:

- **The kind is not one.** A declared clock directs a pair by naming which of its two members is the
  clock; a pulse has no pair to direct, so the declaration decides nothing and a cell generates the same
  minimum-pulse-width constraints whether or not the pulsed pin is declared a clock.
- **The node sets are not resolved here.** A pulse observed from different states can decide different,
  even nested, node sets for the same pin and edge — each such situation still generates its own
  constraint, over its own nodes, from its own state. Which of those constraints stands for the pulse
  generally, and which is stated only in its own context, is the containment rule, next (§8).

## 8. General blocks, conditioned blocks, and the maximal node sets

A cell's delay and hidden arcs render on a split this section carries over. A **general block** carries no
`-when` and stands for the thing it renders however it was reached; a **conditioned block** carries a
`-when` naming one context's own condition, and is added on top where the cell opts the class in. One
transition yields one general block however many contexts it was measured from, and every one of those
contexts can return as its own conditioned block. Constraint blocks split the same way, over the
constraints generated from situations (§7) rather than over an arc's contexts.

A constraint's **identity** is what its block states of the arc it renders: the kind — which decides
the Liberate `-type`s the block fans out to and which pin it relates — the pins it holds apart with
the edge each makes, and the **victim nodes** it names in a single `-probe`. Everything else a block
carries (its `-ic` levels, the held digits of its `-vector`, its `-when`) names the state that
constraint was measured from.

**The maximal node sets.** One constraint decides different nodes from different states — on a cascade
whose second stage is gated, a CLK↓ pulse moves the master alone from one state and walks master and
slave from another; a clock racing its data endangers an internal latch alone where a side input holds
the output still, and the latch and the output together where it does not. So the constraints are
grouped by everything that identifies them EXCEPT their victim nodes, and within a group their node
sets are ordered by **containment**. Each set that no other in its group strictly contains — each
**maximal** set — supplies a general block. Two sets that nest neither way are both maximal, and each
gets one.

What makes containment the right order is how a block is measured. The block names its victim nodes in
Liberate's `-probe` (§9), so a block probing a strict superset states everything the contained set's does
and more, and it is the one that stands for the constraint however it was reached. A set that
neither contains nor is contained asks a different question, so it is asked in its own right.

**Being contained is a demotion, not a drop.** A constraint whose node set another one strictly
contains supplies no general block, and it still renders its own conditioned block, over its own
nodes, in the input context it was observed in. Every context that was observed is characterised in
its own right — that is what the conditioned pass is for — so a conditioned block can carry a
`-probe` narrower than any general block's.

**The tie-break.** Several constraints can be equally dominant: the same identity, the same maximal
node set, reached from different probed states. One of them supplies the general block, chosen by
the probed state's index in exploration order and then by the lowest-numbered of the four ranks its
readings' (cause, outcome) pairs fall into — three causes crossed with two outcomes give six such
pairs, and a toggle and a race sharing a rank at the same outcome brings the count to four. Neither
component states a preference — the index is a breadth-first position, not stable between runs — so
this is no quality judgement. What the pair buys is a total order: a parallel fold lands on one
answer within a run, and choosing among equally-good alternatives is free. Nothing is lost to the
tie-break either, since every other constraint is its own conditioned block's `-when` away.

§4's combinational-neighbourhood filter remains the one place the engine decides what *not* to
report: it says a divergence is not the pin pair's fault. Nothing in this section suppresses a
constraint — it decides how each one renders, generally or in its own context.

## 9. Reporting and rendering

- **stderr.** The detected hazards are reported one entry per cause and starting state. The header
  names the cause — the racing pins, or the pulsed pin with its opening edge — and the state it goes
  wrong from; the body names the condition, the walk into that state, and then one field per outcome
  observed there, each listing the victim nodes THAT reading names and where they land once the
  timing is honoured. The two outcomes need not agree on either, which is why neither the header nor
  a shared field carries a node set. A reading with nowhere to land states its nodes alone — a lone
  toggle has no second edge to be separated from. A constraint is the remedy for a hazard reported
  here, so it carries no diagnostic of its own. This report is the run's whole account of what was
  detected: the emitted artifacts state the timing that removes a hazard and say nothing of the hazard
  itself, so the metastability risk every cause and outcome shares (§1) is named here and nowhere else —
  including for a ring the run states no constraint for, which is one observed under a lone toggle (one
  pin, and one edge has nothing to be separated from — §5) or one in a cell that did not opt into
  constraint arcs.
- **Constraint arcs.** Off by default; enabled per cell in the spec or globally with a CLI flag. A
  constraint over a pin pair renders its general block as a *pair* of characterisation arcs — the two
  sides are characterised separately — a setup and a hold arc for a directed clock↔data constraint, the
  two non-sequential sides for a symmetric one; a conditioned block, where rendered, follows the same
  split, one `-when` per constraint. The vector toggles the two racing pins along their recorded edges,
  holds every other input at its prevector value — the general block's from its representative situation,
  a conditioned block's from its own — and marks all outputs unknown (a constraint arc measures no output
  transition). Each block names its victim nodes in a single `-probe`, so the characterisation measures
  them rather than inferring the violation from the pins; a victim node with no pin of its own — a flop's
  master latch — is given a column on that block alone, which is what its `-ic` states the start level
  through.
- **Minimum-pulse-width arcs.** Under the same opt-in, and one `define_arc` of
  `-type min_pulse_width` rather than a pair: the two members of a setup/hold pair are the two sides
  of a separation between two pins, and a pulse has one pin. The general block's `-vector` switches
  the constrained pin alone, along the pulse's opening edge, with every other input held at the
  level it takes in the probed state and the internals and outputs marked unknown; `-pin` and
  `-related_pin` both name that one pin; a conditioned block states the same construction under its
  own situation's `-when`. A general block's single `-probe` names a maximal node set (§8), a
  conditioned block's the nodes its own constraint decided, and `-ic` states the start condition of
  every column, as on every block of a state-holding cell. Liberate narrows the pulse until the
  probed nodes stop behaving, and that measurement is what puts the
  `min_pulse_width_high`/`min_pulse_width_low` groups in Liberate's own output library — cellsmith
  writes no timing group for them into the `.lib` it emits.

## 10. Guards and invariants

- A cell with **no state variables** has none of the three causes' hazards: every coordinate is a
  function of the inputs alone, so every input toggle settles cleanly with nothing to ring around,
  every input order is confluent, and returning a pulsed pin to its pre-pulse value returns the whole
  machine to the state it started from — a pulse can leave no net effect for its width to decide.
  Detection returns an empty result at that early-out before probing anything.
- A cell with **fewer than two inputs** has no pair to race, so the *pair* probes (§3) early out there
  too. That rule is theirs alone. The single-toggle probes run at any input count — one pin races the
  cell's own feedback, and a lone toggle that never settles is a hazard however few other pins the cell
  has — and so do the pulse probes (§6), which relate one pin to itself. §6's reference close rests on
  that: it is the closing edge alone toggled from a stable state, which is exactly the toggle §5
  records, and the record has to be there at one input as much as at ten.
- Within a cell that clears those early-outs, the probed population is filtered per state: only a
  fully-initialised reachable stable state is probed from (§2). The filter is applied after the states are
  numbered, so a hazard's `discovered` index — the tie-break emission's general-block selection reads (§7,
  §8) — remains its position in exploration order.
- Two budgets bound the machine pass, each charged against work the exploration actually performs rather
  than against the cell's declared shape: the seed minterms pooled as initialisation candidates (a forced
  cover cube contributes `2^d` of them for its `d` unconstrained input columns) and the reachable stable
  states the exploration records. A cell that passes either ceiling yields neither arcs nor hazards, so
  the analysis fails there rather than handing an empty cell on: the run stops with an error naming the
  cell and the flag that raises that ceiling (`--max-candidates`, `--max-states`).
- The set of reported hazards and constraints is determined by the machine analysis; the order in which
  entries are emitted is not.
- The probes never mutate the exploration: they settle *copies* with inputs toggled, so the reachable
  graph the arcs were derived from is exactly the graph the hazards were probed from.
