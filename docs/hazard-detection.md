# Hazard detection and constraint generation

How cellsmith detects the two hazards of a state-holding cell — the **order-dependent hazard** and the
**oscillation hazard** — and, from each detected hazard, **generates** the timing constraint (setup/hold
or non-sequential) that avoids it. Detection and generation are two separate stages: detection names the
risky situation; generation quantifies the timing separation that removes it. Both run on the same
reachable-state machine that drives arc discovery.

This is a companion to `state-machine-arc-engine.md`, which covers the model, the next-state functions δ,
settling, and the reachability exploration; everything here builds on those. The functional state-table
view of the same signals is documented separately in `state-table-regions.md`.

## 1. Two hazards and one remedy

A **delay arc** records that a single input edge causes an output edge. A hazard instead involves *two*
inputs changing too close together. When two signals switch simultaneously — or close enough in time —
they can drive the cell into **metastability**: an unresolved condition the cell cannot leave cleanly.
Detection finds the situations where that risk is real, and the risk takes two shapes:

- **Order-dependent hazard** — the settled state depends on *which* of the two edges lands first. The
  machine is **non-confluent** at that state for that pair.
- **Oscillation hazard** — the two edges landing *at once* drive the state into a **periodic cycle** that
  never settles.

**Metastability is the shared physical risk of both** — the reason a remedy is needed — not a third
hazard and not another name for oscillation.

A **constraint** is that remedy. It is **generated from** a detected hazard; it is never itself a hazard
and never itself detected. It states the timing separation the two inputs need so the risky situation
cannot arise — directed *setup/hold* if the pair contains a declared clock, symmetric *non-sequential*
otherwise (§6). Detection names the situation; the constraint quantifies the separation that removes it.

Two things are deliberately **not** hazards:

- An **undefined state variable is simply uninitialised** — a value not yet driven to a defined level,
  carrying none of the metastability risk a genuine hazard does.
- **Ordinary order-dependence of an arbiter's grants** is its *function*, not its fault: a mutual-exclusion
  element is supposed to grant whichever request arrived first. Its hazard is the oscillation when the
  requests tie — which is why the two hazards must be told apart rather than lumped together as "the
  results differ".

## 2. Everything starts from the reachable states

Detection does not run the exploration itself — it re-walks the *shared* exploration, the same one the
arc discovery uses, built once with the same on/off cover seeding and the same single-input-toggle edges
(the QDI assumption). It probes hazards **only from the reachable stable states**.

That anchoring is the load-bearing design decision. Held state is the product of the cell's own
sequential behaviour; the only joint assignments that mean anything are the ones the dynamics can
actually produce. Reachability here is intrinsic: every probe starts only from a state the exploration
actually reached, so state variables are never coerced to fabricated values and no hazard is manufactured
on a state the cell can never occupy.

The exploration itself, however, never *reports* metastability: when a toggle fails to settle it silently
drops that transition (no impossible arc is fabricated) and moves on. More importantly, it never presents
the transition that makes an arbiter oscillate: its edges toggle one input at a time, and the trigger for
oscillation is precisely the violation of that single-change assumption — **two or more inputs changing
simultaneously** (a mutex's requests co-asserting). So detection must apply that change itself, as a probe
from each reachable state, which is what the next section does.

## Detection

## 3. The probes: single settles once per state, then the per-pair work

For each reachable stable state, detection settles each input's single toggle **once** and reuses it
across every pair — so the per-state single-settle cost is O(n), not O(n²). Every settle either reaches a
fixpoint or reveals a **periodic cycle**: the trajectory it revisits, kept rather than discarded.

For each reachable stable state and each unordered input pair {x, y} (all other inputs held at their
values in that state), the pair-specific work is one simultaneous settle plus, when both single toggles
settled, two order follow-ups:

1. **x alone** — reused from the per-state singles
2. **y alone** — reused from the per-state singles
3. **x and y simultaneously** — the settle done per pair

and, when both single toggles settle, two follow-ups that complete the *orders*:

4. **x then y** — toggle y from x's fixpoint
5. **y then x** — toggle x from y's fixpoint

The two order outcomes are then compared, and the simultaneous settle inspected for a cycle. The outcomes
classify as:

| Observation | Meaning | Detected hazard → generated constraint |
|---|---|---|
| the simultaneous settle returns a cycle | the pair tied and the state oscillates | **oscillation hazard** → its pair's constraint generated (§6) |
| a lone toggle never settles | even one toggle is degenerate | **oscillation hazard** (no competing orders, no pair recorded) |
| the two order outcomes agree | confluent — order does not matter here | nothing |
| the two order outcomes diverge, and the divergence *interacts* (§4) | order matters at this pair | **order-dependent hazard** → constraint generated (§6) |
| the two order outcomes diverge, latch-mediated only (§4) | divergence real but design-tolerated | nothing |

## 4. The order-dependent hazard

When both single toggles settle, detection compares the two order outcomes. If they agree, the pair is
confluent here and nothing is recorded. If they differ, the divergence is a *candidate* order-dependent
hazard — but divergence alone is **not** the verdict.

Global divergence of the joint state only means the two orders left *some* latch somewhere holding
different values — and for a cell with independent domains that is normal operation, not a pin-pair fault.
The order-dependence must **interact with the racing pair in the immediate combinational neighbourhood**:

> Divergence is an order-dependent hazard only if some state variable w that actually differs between the
> two order outcomes has **both** x and y in the **direct support of its own δ_w**.

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
by contrast, meets directly in δ_M1 and survives as a genuine order-dependent hazard. On the ICM dual-clock
synchroniser (the same shape at scale) this filter reduces the reported hazards to the two same-domain
pairs (CLKA, RA) and (CLKB, RB) and removes the meaningless cross-domain clock-vs-clock ones.

Declassifying a relay can legitimately **surface** an order-dependent hazard that used to be latch-masked.
Once a combinational relay is folded into its consumer (`state-space-minimisation.md`), that consumer's δ
directly incorporates the relay's former support — so a pin pair that used to meet only across a latch
boundary can now land in the same direct support. On the ICM cell this is exactly what happens: folding the
selection-interlock relays sela/selb into sela1/selb1 extends each synchroniser's direct support, so the
cell gains the derived setup/hold pairs (CLKA, S) and (CLKB, S) alongside its existing (CLKA, RA) and
(CLKB, RB) — a genuine gain, never a loss, and consistent with the fold's own soundness.

The filter is symmetric in principle: because it iterates over diverging state variables, folding a
cycle-resident relay could in theory also *drop* a pair whose divergence every consumer's settled value
masks — the mirror image of the gain, the tool re-deciding on the minimised model what counts as a
design-tolerated settled snapshot across a latch, a correction in the same sense rather than a regression.

The filter is also why an arbiter's constraint does not come from its divergence: a mutex's diverging
grants each see only *their own* request (δ_Qa depends on {A, Qb}), so its (A, B) divergence fails the
filter — correctly, since for an arbiter that divergence is function, not fault. The pair is instead
carried by the oscillation hazard (§5), from which the next stage generates the constraint.

## 5. The oscillation hazard

A simultaneous settle that returns a **cycle** — a finite, deterministic transition that revisits a
non-fixpoint state, so periodic forever after — is an **oscillation hazard**: the cell never settles,
which is where the metastability risk arises. From the cycle the report is assembled:

- **group** — the state variables that actually oscillate: those whose value differs between any two nodes
  of the cycle (an undefined-vs-defined difference counts). Variables that happen to sit still through the
  cycle are not blamed.
- **condition** — the primary-input assignment of the probe (the toggled state projected onto the inputs),
  rendered as a literal product, e.g. A·B.
- **stable outcomes** — the competing outcomes the oscillation is torn between: the settled results of the
  two *orders* (x then y, and y then x), each projected onto the group. For a mutex these are the two
  grants; simultaneity is exactly the boundary between the two orders, so the order outcomes are the states
  the cycle cannot choose between.

A bare oscillation record cannot by itself name the pins, edges and prevector its constraint needs, so
detection **records the racing pair with the hazard** — the two pins, their edges, and the path to the
probed state — one per pair-probe that oscillated. That record is what the constraining stage (§6) turns
into a constraint, and it is why an arbiter's constraint originates here rather than from its (filtered)
divergence. A degenerate oscillation from a lone toggle has no competing orders, so it records no racing
pair and yields no pair constraint.

Oscillation hazards are deduplicated by (group, condition), keeping the first occurrence in exploration
order — the earliest reachable state at which the condition is observed. A colliding pair observation still
adds its racing pair, so no pair the constraining stage needs is dropped.

Worked example — the mutex Qa = !Qb·A, Qb = !Qa·B, from the idle state (0 0 | 0 0) (notation
(A B | Qa Qb)), pair {A, B}:

- A alone settles to (1 0 | 1 0); B alone to (0 1 | 0 1) — clean single grants, no hazard.
- Both at once: (1 1 | 0 0) → (1 1 | 1 1) → (1 1 | 0 0) → … — a period-2 cycle.
- Detected: an oscillation hazard with group [Qa, Qb] (both oscillate), condition A·B, competing outcomes
  {Qa=1, Qb=0} and {Qa=0, Qb=1} (the two order outcomes, projected onto the group); the racing pair {A, B}
  recorded with it. From that pair the constraining stage generates the non-sequential constraint A↑/B↑.

A C-element, by contrast, never oscillates: it is bistable in its hold region, but that is self-feedback
holding a *settled* value — no probe from a reachable state makes it oscillate. Its A↓ racing B↑ is a
genuine order-dependent hazard (§4), just not an oscillation.

## Constraining

## 6. From a detected hazard to a generated constraint

The constraining stage is separate from detection. It walks every detected hazard — each **order-dependent
hazard**, and each racing pair recorded on an **oscillation hazard** — and generates one constraint per
pair. Every constraint is built the same way, whichever hazard it came from:

- **Kind is decided solely by the declared clock.** A pair containing exactly one pin declared in the
  cell's clock list is a directed **setup/hold** (related = the clock, constrained = the data pin); any
  other pair is a symmetric **non-sequential** constraint. Clocks are *declared, never inferred*: inferring
  one from the race geometry is state-dependent — the same pins read one way from one held state and the
  other way from another — so it distinguishes nothing real.
- **Edges** are the directions the two pins toggle *from their values at the probed state* (a pin at 0
  races rising, at 1 falling).
- **Prevector** is the path from a start state to the probed state, each node projected onto the inputs —
  the same construction as a delay arc's prevector, and it serves the same purpose: it drives every state
  variable, hidden ones included, into the state where the hazard manifests. The rendered human-readable
  form is the two switching edges plus any other inputs held fixed, e.g. A↓ & B↑ with R=0.

Constraints are deduplicated on a canonical key — directed (related, edge, pin, edge) for setup/hold,
unordered for non-sequential — keeping the **shortest prevector** among the states that exhibit the
hazard, with a deterministic tie-break so the generated set is reproducible.

## 7. Reporting and emission

- **stderr.** The detected hazards are reported — the oscillation hazards, the order-dependent hazards
  (grouped per racing input pair, a pair's conditions joined), and — separately — the constraint generated
  as each hazard's remedy, named with its kind and every condition under which it fires. Each hazard states the shared metastability risk; an
  oscillation is flagged as annotated only, never modelled as deterministic timing, because it is a
  property of the cell the user must know about, not an arc. (The exact wording is fixed elsewhere.)
- **Constraint arcs.** Off by default; enabled per cell in the spec or globally with a CLI flag. Each
  generated constraint renders as a *pair* of characterisation arcs — the two sides are characterised
  separately — a setup and a hold arc for a directed clock↔data constraint, the two non-sequential sides
  for a symmetric one. The vector toggles the two racing pins along their recorded edges, holds every other
  input at its prevector value, and marks all outputs unknown (a constraint arc measures no output
  transition).

## 8. Guards and invariants

- Cells with fewer than two inputs, or with no state variables, have no hazards by construction (a hazard
  relates two inputs; with nothing latched, every input order is confluent); detection returns an empty
  result at those early-outs before probing anything.
- A blow-up guard on total machine width (inputs plus state variables) gates the whole machine pass, so a
  pathologically wide cell is never explored and yields neither arcs nor hazards.
- All containers are ordered, so reports come out in a deterministic order.
- The probes never mutate the exploration: they settle *copies* with inputs toggled, so the reachable
  graph the arcs were derived from is exactly the graph the hazards were probed from.
