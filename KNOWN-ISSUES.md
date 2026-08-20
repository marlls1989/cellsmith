# Known issues

Things found during other work and deliberately not fixed at the time, so they are not lost and do
not have to be re-derived later. Each entry should carry enough context to act on without
reconstructing the investigation: what was observed, why it matters, and — where the fix is a
judgement rather than a correction — what the choice actually is.

Remove an entry when it is resolved, or when it becomes a pull request of its own.

## Leakage blocks conflate when combinational and hidden do not

When a cell has many internal nodes and no `expose` list, the emitter cannot express the cell's
full internal state through its interface. The diagnostic reports this as `N block(s) conflate M
measurements: too few nodes exposed to express the cell state`. Running the ICM cell from
examples/cells.toml with its `expose` list removed and `constraint_arcs = true` produced 38
conflated blocks over 118 measurements; all 38 were `leakage` blocks. The same run emitted 6
`setup`, 6 `hold`, 6 `min_pulse_width`, 10 `hidden` and 6 `combinational` blocks, and not one of
those conflated.

The asymmetry is suspicious: combinational and hidden blocks face the same difficulty as leakage
in distinguishing states at unmeasured internal nodes. For constraint blocks there is a plausible
mechanism — a constraint block gets a column for every victim node whether or not the cell exposes
it, and its `-when` states fixed literals of the measured state — but that argument does not cover
combinational and hidden.

`Description`'s five constraint arms (src/emit/block.rs:351-355) are reached only by the unit test
`a_block_describes_itself_on_one_line` (src/emit/block.rs:582-652), never by an end-to-end run on
real cells: the conflation report renders every block through the same `Description` adapter
(src/main.rs:277) rather than through a constraint-specific arm, so those five arms are exercised
only if a constraint block ever conflates. Whether that is a missing fixture or evidence that those
arms are unreachable is the open question; if they are unreachable, that deserves to be stated at
`Description` rather than left to look merely untested.
