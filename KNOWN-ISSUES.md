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

---

## An exposing cell explores its state machine twice, and the second pass is redundant

A cell that lists `expose` nodes is analysed as two views: the arc view, minimised with the
exposed nodes preserved as coordinates, and the model view, minimised again with only the
output pins preserved. `Cell::analyse_with` runs `finish_view` for each, so each performs
its own `machine::explore`. The second explores a state space that is already determined by
the first.

The reason is what the state-space minimisation removes. Dedup retires a signal whose δ is
the *identical* BDD to another's — the same coordinate seen twice. The relay fold retires a
signal that does not appear in its own support, so at every stable state its value is
`δ(state)`, a function of the coordinates that survive. Neither removes a coordinate that
carries a bit of its own.

Oscillation is the case where two coordinates are *not* in lockstep — they disagree over
time — and the fold guard exists to keep it. Its structural test is the condition under
which that disagreement can arise, so a coordinate that could oscillate is never folded
away. An arity-1 ring is folded, but the fold lands the bit and the oscillation on a single
self-holding coordinate (`a = "!b"`, `b = "a"` becomes `b = !var(b)`), so nothing is lost
there either.

That makes the projection from the arc view's reachable stable states onto the model view's
a bijection. It is surjective because the arc view explores a superset of the columns over
the same inputs. It is injective because two arc-view states differing only in a folded
column cannot both be stable: stability forces that column to equal its δ, and the surviving
columns already determine it. Determinacy carries across for the same reason, so
`Machine::arc_eligible` cannot disagree between the views.

So the model view's reachable set, its transitions and its BFS order are the arc view's with
the folded columns dropped. The second `explore` rediscovers what is already held, and it is
the expensive half — the BFS plus the per-candidate settlement ranking. What genuinely
differs per view is everything downstream of exploration (regions, edge classification, the
display expressions), which is cheap by comparison.

The fix keeps both minimisation passes and drops only the second exploration. The first
minimisation runs with the exposed nodes preserved and the single `explore` runs on the
machine it produces. The second minimisation still runs, because the reduced equation system
is what the state tables are built from. The exploration's output is then projected onto the
variables that survive it, rather than searched for again.

Not done here because it reshapes `Cell::analyse_with`, which was outside the change that
found it. It makes explicit a dependency the two-pass form already has: if the minimisation
were not behaviour-preserving, the model view would be wrong whether or not it is
re-explored.
