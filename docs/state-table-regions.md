# State-table regions: the functional view

How cellsmith derives, for each signal of a cell, the three-valued next-state table that sits behind
the Verilog sequential UDP and the Liberty `statetable`. This is the **functional** view of a signal —
what value it drives, or holds, as a function of the signals it depends on. It is distinct from the
**timing-arc** view produced by the state machine, which `state-machine-arc-engine.md` and
`hazard-detection.md` cover; the two views are computed independently from the same, minimised, cell
functions. How that model is minimised — the alias/complement collapse and guarded relay fold that
produce the shared per-cell map both views read — is documented in `state-space-minimisation.md`.

Regions are derived once per signal, over the shared minimised BDD map, and cached in `signals()` order
— outputs then internals — and stay **per-signal** at this stage regardless of consumer. The Verilog
sequential UDP emitter reads that cache directly, one region set per signal. The Liberty emitter reads
the same per-signal cache but does not stop there: the Liberty spec forbids an output pin's table from
referencing another output pin, so at emission time (`src/emit/statetable.rs`) it **joins** every
sequential cell's state nodes' regions into one cell-wide `statetable` by cube intersection (§7).

## 1. Two views of a signal, and which one this is

The state machine treats a cell as an asynchronous machine over `inputs × state-variables` and reports
how single input edges propagate to output edges — the timing arcs. The regions here answer a different
question: for a given assignment of the signals a function depends on, does the output go high, go low,
or keep its previous value? That three-valued answer is the next-state table of a sequential UDP row,
and — after emission joins the per-signal regions into one cell-wide table (§7) — a row of the Liberty
`statetable`.

Region derivation is not handed the parsed expression directly — it works from the shared per-cell map's
already-**minimised** entry for the signal (the same map the minimisation pass rewrote and the machine
pass reads from; see `state-machine-arc-engine.md` §3). Columns therefore reflect the **folded** support:
on the real `ICM` cell, the relay `sela` folds into `sela1` before regions are derived, so `sela1`'s
columns gain `enB` and `S` — `sela`'s own referenced signals — in place of `sela` itself. A purged
relay/alias internal (like `sela` here) has no surviving output, so it has no region entry and
contributes no row to the emitted table at all.

## 2. The column set: BDD support minus self-feedback

The column set is the signal's function's own support, with the signal's self-feedback removed. The
consequences are exact:

- **Every signal the function actually depends on becomes a column** — a primary input, another output,
  or an internal state node — because it appears in the function's support.
- **An input the function ignores never appears.** Support comes from the BDD, so a pin the function
  does not reference is simply absent; it is not carried as a spurious don't-care column.
- **The signal's own self-feedback is projected out** and becomes the sequential element's
  current-state (`reg`) column, rather than an input column. It is the only support variable left
  outside the column set, which is what makes the projection in §3 well defined.

## 3. The three regions by universal projection of the self variable

The regions come from re-basing the function `f` onto the column set by **universally** projecting away
the self variable `self`:

- `on   = ∀self. f`
- `off  = ∀self. ¬f`
- `hold = ¬(on ∨ off)`

Because a partial function's on-set and off-set are **not** complementary, the gap between them is
non-empty exactly where the output still depends on the projected self variable — that is, where the
next value is state-dependent. That gap is the **hold** set: the hysteretic region (formalised as the
`hysteretic` flag in §5), rendered as the `-`/`N` no-change entry in the emitted tables. The onset and
offset are each taken as a clean cover; the hold set is reconstructed as its own function from the onset
and offset covers so that it, too, can be minimised as an independent onset.

## 4. Each region is minimised independently

Each of the three regions is Espresso-minimised on its own, as its own onset. This is safe precisely
because no region carries a don't-care set. Minimising an onset with no don't-cares reproduces that
exact region, so minimisation cannot absorb the hold gap into `on` or `off`. An empty cover minimises to
empty, which preserves region emptiness and therefore both the `hysteretic` flag (§5) and the emitters'
constant detection (§7).

## 5. The result

For each signal, region derivation produces: the column set of §2; a set of cubes for each of `on`,
`off`, and `hold`, each cube fixing some columns to true/false and leaving the rest don't-care; and a
`hysteretic` flag, true exactly when the hold region is non-empty, i.e. when the signal holds on its own
state.

## 6. Worked examples

**C2 C-element** — `Q = A*B + Q*(A+B)`. `Q` references only `A`, `B`, and itself, so `Q` is projected
out as the `reg` and the column set is `[A, B]`. The regions are:

| Region | Equation | Cubes |
|---|---|---|
| `on` | `A*B` | `A=1, B=1` |
| `off` | `!A*!B` | `A=0, B=0` |
| `hold` | `A xor B` | `A=1,B=0` and `A=0,B=1` |

`hysteretic` is true.

**Combinational ND2** — `Y = !(A*B)`. `Y` does not reference itself, so the onset and offset are
complementary, the hold region is empty, and `hysteretic` is false. The onset is non-empty.

**DFF slave** — `Q = CLK*M + !CLK*Q` with internal master `M = !CLK*D + CLK*M`. The column set is
`[CLK, M]`: the internal node `M` stays a column because `Q`'s function depends on it; the primary input
`D` drops out because it is not in `Q`'s support; and `Q` itself is projected out as the `reg`.
`hysteretic` is true.

**Equivalence of the minimised regions.** Rebuilding a function from each region's emitted (minimised)
cubes and comparing it against the reference region computed directly by universal projection confirms
that minimisation preserves every region's function even though the cube set changes — checked against
the C2, ND2, DFF, a cross-coupled mutex, and six-input arbiter-latch cells.

## 7. Consumers

The emitters read these regions rather than rebuilding them, but consume them differently.

The Verilog sequential UDP consumes regions **per-signal**, unchanged: `on` becomes `1`, `off` becomes
`0`, and `hold` becomes `-`, with internal state nodes appearing as internal `wire`s and as columns in
the tables of the outputs that reference them — an output's UDP may reference another output directly.

The Liberty `statetable` cannot work the same way: the Liberty spec disallows an output pin's own table
from referencing another output pin. So emission (`src/emit/statetable.rs`) builds one cell-wide
`statetable` from the per-signal regions by **cover algebra**, not a cube-intersection cross-product,
in five steps:

1. **Rename.** Each state signal's minimised `on`/`off`/`hold` cover is renamed (`rename_outputs`) so
   its single output column carries that node's own name, rather than an anonymous placeholder.
2. **Normalise.** Each renamed cover is re-based (`over_vars`) onto one **shared header**: the union of
   every state signal's own transition function's input support (the primary-input columns from every
   signal's region, in cell input order), followed by the state signals' original names in node order.
   Normalising onto this shared header is what makes the next step's stacking well defined.
3. **Extend.** For a given action, every state signal's renamed, normalised cover is folded
   (`Cover::extend`) into one another, giving a single multi-output F cover for that action. Doing this
   once per action produces three multi-output covers — ON, OFF, HOLD.
4. **Minimise.** Each of the three multi-output covers is Espresso-minimised independently, as a joint
   multi-output problem, so cubes can be shared across nodes that agree on the same action over the
   same input/current pattern.
5. **Keyed fold.** Every resulting cube, from all three minimised covers, is folded into the joint row
   table keyed by its input/current minterm (a `BTreeMap` keyed on the pattern): for each node the cube
   asserts, that node's slot in the row for that key is stamped with the pass's action (`H`/`L`/`N`);
   cubes that land on the same key — whether from the same action's cover or a different one — stamp
   different nodes' slots of the *same* row rather than producing separate rows.

A `-` in a row's next-state field for some node means this row does not define that node's next value at
all. Liberty resolves each output's next state independently — **per-output priority** (Vol. 1 §5) — by
reading the first row that pins a definite (non-`-`) value for it, in table order; `-` simply defers
that node to whichever lower-priority row does pin it (the master-slave `CP(R)`/`CPN(F)` split-row
example — one row for the rising-clock phase, another for the falling-clock phase — is exactly this
deferral). The construction is sound only because, per node, its `on`, `off`,
and `hold` regions are mutually disjoint: a row can therefore never need to stamp the same node with two
different definite actions, however many passes contribute cubes to it.

The joint table's node set is every state variable of the cell — state outputs and genuine internals
alike — each column carrying the signal's own name. Each output pin is then expressed against that
table: an output that **is** a node carries `internal_node` binding the node to the port plus a
`state_function` naming it, an output that **depends on** nodes carries a `state_function` over them,
and an output over primary inputs alone carries a plain `function` even inside a cell that has a
statetable. A genuine internal node has no output pin; it is anchored by a `direction : internal` pin
carrying its `internal_node`.
