# Known issues

Things found during other work and deliberately not fixed at the time, so they are not lost and do
not have to be re-derived later. Each entry should carry enough context to act on without
reconstructing the investigation: what was observed, why it matters, and — where the fix is a
judgement rather than a correction — what the choice actually is.

Remove an entry when it is resolved, or when it becomes a pull request of its own.

---

## `ex.order` contains partially-fixed nodes, and `arcs.rs` claims otherwise

**Found:** Wave 1 of the unified edge analysis, confirmed independently by the wave verifier.
**Status:** left unfixed, out of that wave's scope.

`ex.order` — the exploration's stable-state list — contains nodes with a **state column left
don't-care**. Observed example:

```
Minterm { E:0, D:1, S:0, L:- }      alongside the L:0 and L:1 nodes
```

So a *partially-fixed* context sits in the order beside the fully-fixed ones for the same input
projection.

### Why it matters

Since commit `456f862` ("Key arcs by their full machine start context…"), arcs are keyed on the full
machine start context — so a partially-fixed context is **its own arc identity** and produces its
own arc. Those arcs are included in the current counts:

```
sequentials   delay  331 → 515      hidden  1263 → 2101
cells         delay  146 → 306      hidden   750 → 1508
```

This is also why an independent implementation reproduced the design prototype's figures exactly:
both behaved this way.

### The inaccuracy

`src/logic/arcs.rs:4-5` and `:93` describe a node as "fully-fixed". That is false, and it predates
the arc-identity work.

### The fix is a decision, not a doc edit

Either:

- exploration should yield only fully-fixed nodes — the partially-fixed entries disappear and the
  arc counts **drop**; or
- partially-fixed contexts are legitimate distinct characterisation contexts — the module doc is
  corrected to say so and the counts stand.

Either way the arc counts move, so this is not cosmetic. Decide deliberately before touching it.
