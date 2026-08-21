# Known issues

Things found during other work and deliberately not fixed at the time, so they are not lost and do
not have to be re-derived later. Each entry should carry enough context to act on without
reconstructing the investigation: what was observed, why it matters, and — where the fix is a
judgement rather than a correction — what the choice actually is.

Remove an entry when it is resolved, or when it becomes a pull request of its own.

## The conflation warning is silent for general arcs

`-ic` and `-vector` reach exactly a block's `-pinlist`, so a block cannot state an internal node the cell does not `expose`. The warning that reports this counts the firings that collide on one emitted block, and for the measured classes the general pass has already chosen ONE representative firing per transition before the block sink sees it, so a general block always arrives carrying a single firing and never counts as a conflation. Leakage states one block per rest state with no such choice, which is why it is the only class that conflates in a run without `--when`: the ICM cell from examples/cells.toml with its `expose` list removed and `constraint_arcs = true` reports 38 conflated leakage blocks over 118 measurements and nothing else, while the same run under `--when` reports 330 blocks over 898 measurements — 40 combinational, 158 hidden, 10 setup, 10 hold, 74 min_pulse_width and the same 38 leakage.
