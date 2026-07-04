# Hazard detection and constraint derivation

How cellsmith detects the hazard phenomena of a state-holding cell — **order-dependence** and
**oscillation** (metastability) — and derives the timing **constraints** (setup/hold, non_seq) that
avoid them, from the same reachable-state machine that drives arc discovery. This is a companion to `state-machine-arc-engine.md`, which covers the model,
the next-state functions δ, settling, and the reachability BFS; everything here builds on those.

The code lives in `src/logic/`:

| File | Role |
|------|------|
| `confluence.rs` | `confluence::derive`: one pass over the reachable states probing every input pair — produces both constraints and arbitrations |
| `interlock.rs` | the `Arbitration` report type (and its string helpers); no detection logic |
| `machine.rs` | `settle_or_cycle`, the settle variant that returns the periodic cycle instead of discarding it |

Downstream: `Cell::analyse` calls `analysis::analyse_machine`, which calls `confluence::derive` once
per cell, and stores the results on `AnalysedCell` (`constraints`, `arbitration`); `main.rs` prints one
stderr warning per hazard; `emit/arcs_tcl.rs` renders constraints as Liberate `define_arc` blocks when
enabled.

## 1. Phenomena and remedy: what is a hazard, what is a constraint

A **delay arc** records that a single input edge causes an output edge. A **hazard** instead involves
*two inputs* changing too close together. Two distinct phenomena model the potential hazards, and the
same pass surfaces both:

- **Order-dependence** — the settled state depends on *which* of the two edges lands first. The machine
  is **non-confluent** at that state for that pair.
- **Oscillation at simultaneity** — the two edges landing *at once* drive the state into a **periodic
  cycle** that never settles: metastability.

A **constraint** (`Constraint`) is not a kind of hazard — it is the *remedy*: the requirement that the
two inputs keep enough separation, which is how **both** phenomena are avoided. Whenever a phenomenon is
attributed to a pin pair, the engine files a constraint for that pair — directed *setup/hold* if the
pair contains a declared clock, symmetric *non_seq* otherwise (§5). Oscillation is additionally
*reported* as an **arbitration** (`Arbitration`, §4): the constraint says how to stay out of the
metastable window; the arbitration annotates that the window exists and what oscillates inside it.

Two things are deliberately **not** hazards:

- An **undefined state variable is simply uninitialised** — a state that has not yet been driven to a
  defined value, not a metastable one. Only a periodic oscillation is metastability.
- **Ordinary order-dependence of an arbiter's grants** is its *function*, not its fault: a mutex is
  supposed to grant whichever request arrived first. Its hazard is the oscillation when the requests tie,
  which is why the two phenomena must be told apart rather than lumped as "the results differ".

## 2. Everything starts from the reachable states

`confluence::derive` does not run `machine::explore` itself — it re-walks the *shared* exploration
(`let ex = &m.explored;`), the same one the arc BFS uses, built once by `Machine::build` with the same
on/off cover seeding and the same single-input-toggle edges (the QDI assumption). It probes hazards
**only from the reachable stable states** in `ex.order`.

That anchoring is the load-bearing design decision. Held state is the product of the cell's own
sequential behaviour; the only joint assignments that mean anything are the ones the dynamics can
actually produce. Reachability here is intrinsic: every probe starts only from a state the exploration
actually reached, so state variables are never coerced to fabricated values and no hazard is
manufactured on a state the cell can never occupy.

The BFS itself, however, never *reports* metastability: when a toggle fails to settle it silently drops
that transition (no impossible arc is fabricated) and moves on. More importantly, it never presents the
transition that makes an arbiter oscillate: its edges toggle one input at a time, and the trigger for
arbitration is precisely the violation of that single-change assumption — **two or more inputs changing
simultaneously** (a mutex's requests co-asserting). So the detector must apply that change itself, as a
probe from each reachable state, which is what the next section does.

## 3. The probes: single settles once per state, then the per-pair work

For each reachable stable state, `confluence::derive` settles each input's single toggle **once**, into
the `single` vector, and reuses it across every pair — so the per-state single-settle cost is O(n), not
O(n²). Every settle goes through `machine::settle_or_cycle` — `settle`'s underlying form, which returns
`Ok(fixpoint)` or `Err(cycle)`, the periodic trajectory it revisited instead of discarding it.

For each reachable stable state `s` and each unordered input pair `{x, y}` (all other inputs held at
their values in `s`), the pair-specific work is one simultaneous settle plus, when both single toggles
settled, two order follow-ups:

1. **`x` alone** → `r_x` — indexed from `single`
2. **`y` alone** → `r_y` — indexed from `single`
3. **`x` and `y` simultaneously** → `r_sim` — the settle done per pair

and, when both single toggles settle, two follow-ups that complete the *orders*:

4. **`x` then `y`**: toggle `y` from `r_x`'s fixpoint → `s_xy`
5. **`y` then `x`**: toggle `x` from `r_y`'s fixpoint → `s_yx`

The outcomes classify as:

| Observation | Meaning | Report |
|---|---|---|
| `r_sim` is `Err(cycle)` | the pair tied and the state oscillates | **Arbitration**, plus the pair's **Constraint** (§4) |
| `r_x` or `r_y` is `Err(cycle)` | even a lone toggle never settles (degenerate) | **Arbitration** (no competing orders to report) |
| `s_xy == s_yx` | confluent — order does not matter here | nothing |
| `s_xy != s_yx`, and the divergence *interacts* (§6) | order-dependence | **Constraint** |
| `s_xy != s_yx`, latch-mediated only (§6) | divergence real but design-tolerated | nothing |

## 4. Arbitration: oscillation as the report — and as the constraint's origin

When `settle_or_cycle` returns `Err(cycle)`, the cycle *is* the metastability: a finite, deterministic
`step` that revisits a non-fixpoint state is periodic forever after. From it the report is assembled:

- **`group`** — the state variables that actually oscillate: those whose value differs between any two
  nodes of the cycle (an undefined-vs-defined difference counts). Variables that happen to sit still
  through the cycle are not blamed.
- **`condition`** — the primary-input assignment of the probe (the toggled node projected onto the
  inputs), rendered as a literal product, e.g. `A*B`.
- **`stable`** — the competing outcomes the oscillation is torn between: the settled results of the two
  *orders* (`x` then `y`, and `y` then `x`), each projected onto the group. For a mutex these are the
  two grants; simultaneity is exactly the boundary between the two orders, so the order outcomes are the
  states the cycle cannot choose between.

Arbitrations are deduplicated by `(group, condition)`, keeping the first occurrence in BFS order — the
earliest reachable state at which the condition is observed.

**The same oscillation also files the pair's timing constraint.** Metastability at simultaneity is the
physical origin of the requirement that the two inputs not change too close together, so the probe
constructs a `Constraint` for `{x, y}` exactly as the divergence path does (same kind rule, edges, and
prevector — §5) and inserts it into the same dedup map. This matters because an arbiter's
order-divergence is discarded by the interaction filter (§6) — its cross-coupled grants do not have both
requests in their own δ's direct support — so the oscillation probe is what supplies, for example, a
mutex's `non_seq (A, B)`.

Worked example — the mutex `Qa = !Qb·A`, `Qb = !Qa·B`, from the idle state `S0 = (0 0 | 0 0)` (notation
`(A B | Qa Qb)`), pair `{A, B}`:

- `A` alone settles to `(1 0 | 1 0)`; `B` alone to `(0 1 | 0 1)` — clean single grants, no hazard.
- Both at once: `(1 1 | 0 0) → (1 1 | 1 1) → (1 1 | 0 0) → …` — a period-2 cycle.
- Report: `group = [Qa, Qb]` (both oscillate), `condition = A*B`, `stable = {Qa=1, Qb=0}` and
  `{Qa=0, Qb=1}` (the two order outcomes, projected onto the group), plus the constraint
  `non_seq A↑/B↑`.

A C-element, by contrast, never arbitrates: it is bistable in its hold region, but that is self-feedback
holding a *settled* value — no probe from a reachable state makes it oscillate. Its `A↓` racing `B↑`
order-dependence is a genuine constraint (§6), just not an arbitration.

## 5. Constraint shape, kind, and dedup

Every constraint — whether it came from the divergence path or from the oscillation probe — is built the
same way:

- **Kind is decided solely by the declared clock.** A pair containing exactly one pin declared in the
  cell's `clock = [...]` list is a directed **setup/hold** (`related` = the clock, `pin` = the data pin);
  any other pair is a symmetric **non_seq**. Clocks are *declared, never inferred*: inferring one from
  the race geometry is state-dependent — the same pins read one way from one held state and the other
  way from another — so it distinguishes nothing real.
- **Edges** are the directions the two pins toggle *from their values at `s`* (a pin at 0 in `s` races
  rising, at 1 falling).
- **Prevector** is the BFS path from a start state to `s` (via `ex.prev`), each node projected onto the
  inputs — the same construction as a delay arc's prevector, and it serves the same purpose: it drives
  every state variable, hidden ones included, into the state where the hazard manifests.
  `Constraint::condition()` renders the human-readable form: the two switching edges plus any other
  inputs held fixed, e.g. `A↓ & B↑ with R=0`.

Constraints are deduplicated on a canonical key — directed `(related, edge, pin, edge)` for setup/hold,
unordered for non_seq — keeping the **shortest prevector** among the states that exhibit the hazard.

## 6. Not every divergence is a hazard: the interaction filter

`s_xy != s_yx` alone is *not* the verdict. Global divergence of the joint state only means the two
orders left *some* latch somewhere holding different values — and for a cell with independent domains
that is normal operation, not a pin-pair fault. The order-dependence must **interact with the racing
pair in the immediate combinational neighbourhood**:

> Divergence yields a constraint only if some state variable `w` that actually differs between `s_xy`
> and `s_yx` has **both** `x` and `y` in the **direct support of its own δ_w**.

Why direct support is the right notion of "immediate neighbourhood": `resolve::delta` composes through
*combinational* logic only — a state variable is kept as a variable, never substituted through. So both
pins appearing in `δ_w`'s support means they meet within one combinational cone in front of a single
latch: the race is physically present at that latch's input. If no diverging `w` sees both pins, the
divergence was mediated **across a latch boundary** — what crossed the boundary is a *settled snapshot*
of the earlier domain, not the live race — and the pin pair is not at fault.

Worked example — a two-domain sampling chain (the `SYNC2` test fixture; the `ICM` dual-clock
synchroniser in `examples/cells.toml` is the same shape at scale):

```
M1 = !C1·D + C1·M1        δ_M1 support: {C1, D, M1}
Q  = !C2·M1 + C2·Q        δ_Q  support: {C2, M1, Q}
```

The `(C1, C2)` order-divergence is *real* — whether `Q` latches `M1`'s old value or `D`'s new one
depends on which latch closes first — but no single δ sees both `C1` and `C2`: the divergence is carried
across the `M1 → Q` latch boundary, so it is filtered. The `(C1, D)` race, by contrast, meets directly
in `δ_M1` and survives as a genuine hazard. On the `ICM` cell this filter is what reduces the reported
constraints to the two same-domain pairs (`CLKA`,`RA`) and (`CLKB`,`RB`) and removes the meaningless
cross-domain clock-vs-clock ones.

The filter is also why the arbitration probe must file the arbiter's constraint itself (§4): a mutex's
diverging grants each see only *their own* request (`δ_Qa` support is `{A, Qb}`), so its `(A, B)`
divergence fails the filter — correctly, since for an arbiter that divergence is function, not fault —
and the pair's constraint instead comes from the oscillation, which *is* its fault.

## 7. Reporting and emission

- **stderr warnings** (`main.rs`): one line per arbitration —
  `nodes {Qa, Qb} arbitrate (metastable at A*B) — annotated only, not modelled as timing` — and one line
  per constrained pin pair, its kind and every condition under which it fires. Arbitration is *never*
  expressed as deterministic timing; it is a property of the cell the user must know about, not an arc.
- **Constraint arcs** (`emit/arcs_tcl.rs`): off by default; enabled per cell with
  `constraint_arcs = true` in the spec or globally with the `--constraints` CLI flag. Each `Constraint`
  renders as a *pair* of `define_arc` blocks — Liberate characterises the two sides separately —
  `setup` + `hold` for a directed clock↔data constraint, `non_seq_setup` + `non_seq_hold` for a
  symmetric one. The vector toggles the two racing pins along their recorded edges, holds every other
  input at its prevector value, and marks all outputs `X` (a constraint arc measures no output
  transition).

## 8. Guards and invariants

- Cells with fewer than two inputs (`n < 2`), or with no state variables (`k == 0`), have no hazards by
  construction (a hazard relates two inputs; with nothing latched, every input order is confluent);
  `confluence::derive` returns an empty `HazardAnalysis` at those early-outs before probing anything.
- The `n + k > 22` blow-up guard is the shared `MAX_MACHINE_VARS`: it gates the whole machine pass in
  `Machine::build`, so a pathologically wide cell is never explored and yields neither arcs nor hazards.
- All containers are `BTreeMap`/`BTreeSet`, so reports come out in a deterministic order.
- The probes never mutate the exploration: they settle *copies* with inputs toggled, so the reachable
  graph the arcs were derived from is exactly the graph the hazards were probed from.
