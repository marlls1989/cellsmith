# Known issues

Things found during other work and deliberately not fixed at the time, so they are not lost and do
not have to be re-derived later. Each entry should carry enough context to act on without
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

## Seed settling runs sequentially, and no longer has a reason to

`explore`'s seeding phase settles each pooled candidate one at a time. That shape was justified by an
ordering it no longer has: the comment read "Sequential: the Vacant-insertion order into `prev` fixes
the order seeds are pushed onto the BFS queue", and the candidate ranking that order-fixing served has
been removed as a leftover of a superseded algorithm. `settle` is the expensive part — one walk per
candidate — and the BFS levels below already run their toggles in parallel, so the seeding phase is the
odd one out.

The parallel form mirrors the level pipeline directly: collect
`pool.par_iter().filter_map(|input| settle(&stepped, &input.project_to(&full_names)))` into a
`HashSet`, then drain it into `prev` and the frontier. Same seed set — the set dedups candidates
settling to one state, which is what the `Vacant` entry does today — and frontier order is free, as
within-level order already is.

Not taken because aligning code on a critical path is its own pass, and the benefit is unmeasured: no
one has established what share of analyse time the seeding phase holds. The criterion benches can
answer that first if a number is wanted before the change.

## Which observation supplies a constraint's general block is picked by a key that could go

Where several observations of one probed state are equally dominant, emission picks one by the
`(discovered, ordinal)` key on `Hazard`. Nothing outside the crate requires that pick — Liberate
receives whichever block is written, and detection files a record for every observation regardless — so
the key is not a determinism guarantee owed to anyone. It is the mechanism of a free choice, and a pick
needs some rule.

What the key settles is the pick within one analysis, not across runs. The candidate pool the
exploration seeds from is a `HashSet` (`machine.rs:415`), so the order the candidates are settled in is
schedule-dependent, and with it the seed order, the `Explored::order` indices and the `discovered` each
hazard carries. Two runs over one cell can therefore read the same observations under different indices
and promote a different one of the equally dominant to the general block. Within one analysis the
indices are the one set, which is what `ic_is_the_only_line_the_gate_adds` rests on: it emits a single
analysis twice.

The judgement, should the key be revisited: deleting it does not remove the choice, it changes who
makes it. The pick becomes schedule-dependent rather than fixed per emission, which reaches
`ic_is_the_only_line_the_gate_adds` — that test emits one analysis twice and compares the two decks as
multisets of `-ic`-stripped blocks, so it relies on the two emissions agreeing on the representative,
and on nothing about the order they state their blocks in — and it reaches `Constraint`'s
own `discovered`/`ordinal` fields and the merge code in `merged_victims`. Those are the sites a
removal has to answer for; the key itself carries no meaning worth preserving.
