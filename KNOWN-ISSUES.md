# Known issues

The known limitations of cellsmith. Each entry carries enough context to act on without
reconstructing the investigation: what was observed, why it matters, and — where the fix is a
judgement rather than a correction — what the choice actually is.

Remove an entry when it is resolved, or when it becomes a pull request of its own.

## The conflation warning is silent for general arcs

`-ic` and `-vector` reach exactly a block's `-pinlist`, so a block cannot state an internal node
the cell does not `expose`. The warning that reports this counts the firings that collide on one
emitted block, and for the measured classes the general pass has already chosen ONE representative
firing per transition before the block sink sees it, so a general block always arrives carrying a
single firing and never counts as a conflation. Leakage states one block per rest state with no
such choice, which is why it is the only class that conflates in a run without `--when`: the ICM
cell from examples/cells.toml with its `expose` list removed and `constraint_arcs = true` reports
38 conflated leakage blocks over 118 measurements and nothing else, while the same run under
`--when` reports 330 blocks over 898 measurements — 40 combinational, 158 hidden, 10 setup, 10
hold, 74 min_pulse_width and the same 38 leakage.

## Seed settling runs sequentially

`explore`'s seeding phase settles each pooled candidate one at a time, and nothing depends on the order
the seeds are settled in. `settle` is the expensive part — one walk per candidate — and the BFS levels
below already run their toggles in parallel, so the seeding phase is the odd one out.

The parallel form mirrors the level pipeline directly: collect
`pool.par_iter().filter_map(|input| settle(&stepped, &input.project_to(&full_names)))` into a
`HashSet`, then drain it into `prev` and the frontier. Same seed set — the set dedups candidates
settling to one state, which is what the `Vacant` entry does — and frontier order is free, as
within-level order already is.

The benefit is unmeasured: what share of analyse time the seeding phase holds is not known. The
criterion benches can measure it.

## Which tied observation supplies an unconditioned block varies from run to run

Every arc kind emits one unconditioned block per identity, rendered from one of the observations that
carry it, and where several tie the one picked is decided by exploration order:

- **Delay and hidden arcs** are keyed on their pins and edges. The representative is a firing with the
  shortest prevector, and among firings tied at that length the first in exploration order is kept
  (`generalised` in `src/emit/arcs_tcl.rs`).
- **Constraint arcs** are keyed on the constraint kind, the constrained pin with its edge, and the
  victim nodes with the level each holds. An observation is dominated, and supplies no unconditioned
  block, when another of the same kind and pin fixes every victim node it fixes at the same level and
  at least one more. Among the rest, the representative is the minimum `(discovered, ordinal)`:
  `discovered` is the probed state's index in exploration order, and `ordinal` numbers the (cause,
  outcome) rank the observation was read from (`constraint_selection` in `src/emit/arcs_tcl.rs`).

Exploration order comes from std hash containers: the candidate pool the exploration seeds from is a
`HashSet`, and each BFS level collects the states it reaches into a `HashMap` (`explore` in
`src/logic/machine.rs`). Their iteration order follows std's per-process random hash seed, so the order
varies from run to run even at one thread — thread scheduling is not what drives it. Two runs over one
cell can therefore render an identity's unconditioned block from different tied observations, whose
blocks differ in what names the observation: the `-ic` levels and the `-vector`'s held digits. Within
one analysis the order is fixed, which is what
`ic_is_the_only_line_the_gate_adds` rests on: it emits a single analysis twice.

Nothing outside the crate requires a particular pick — Liberate receives whichever block is written, and
detection files a record for every observation regardless.
