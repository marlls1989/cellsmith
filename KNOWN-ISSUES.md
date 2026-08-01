# Known issues

Things found during other work and deliberately not fixed at the time, so they are not lost and do
not have to be re-derived later. Each entry should carry enough context to act on without
reconstructing the investigation: what was observed, why it matters, and — where the fix is a
judgement rather than a correction — what the choice actually is.

Remove an entry when it is resolved, or when it becomes a pull request of its own.

---

## `espresso-logic` panics on a forced cover cube with 64 don't-care input columns

Exploration seeds its candidate pool from each seed function's forced FR cover
(`cover_inputs` in `src/logic/machine.rs`), expanding every cube over the input names with
`Cube::expand_to`. `expand_to` forwards straight into espresso-logic's
`Minterm::expand_over`, which asserts `k < usize::BITS as usize` on `k`, the don't-care
input columns left after projecting the cube onto those names
(`espresso-logic-5.6.2/src/cover/minterm.rs:1298`) — so 64 don't-cares in a single cube
already panics. The assert fires while the expansion iterator is being built, ahead of the
`n = minterms.len()` charge that would otherwise let `--max-candidates` reject the cube
first.

In practice this is reached by a cell with 65 or more inputs: its forced cover's first
cube pins one input and leaves the remaining 64 don't-care, which is already at the
boundary.

Filed upstream as https://github.com/marlls1989/espresso-logic/issues/24, which confirms
the boundary is exactly 64 don't-cares. Not fixed here — the assert enforces the crate's
own invariant, and the fix belongs in `expand_over`'s construction, upstream of cellsmith.
