# State-table regions: the functional view

How cellsmith derives, for each signal of a cell, the three-valued next-state table that sits behind
the Verilog sequential UDP and the Liberty `statetable`. This is the **functional** view of a signal —
what value it drives, or holds, as a function of the signals it depends on. It is distinct from the
**timing-arc** view produced by the state machine, which `state-machine-arc-engine.md` and
`hazard-detection.md` cover; the two views are computed independently from the same, minimised, cell
functions. How that model is minimised — the alias/complement collapse and guarded relay fold that
produce the shared per-cell map both views read — is documented in `state-space-minimisation.md`.

The code lives in one file:

| File | Role |
|------|------|
| `src/logic/regions.rs` | `state_regions`: derive a signal's on/off/hold regions and column set (module doc `regions.rs:1-26`) |

Downstream: `Cell::analyse` calls `state_regions` once per signal, over the **shared minimised BDD map**
(`model.rs:332-337`), and caches the result on `AnalysedCell::regions`, in `signals()` order — outputs
then internals (`model.rs:165-167`). The Verilog and Liberty emitters read that cache rather than
rebuilding the BDDs (§7).

## 1. Two views of a signal, and which one this is

The state machine treats a cell as an asynchronous machine over `inputs × state-variables` and reports
how single input edges propagate to output edges — the timing arcs. The regions here answer a different
question: for a given assignment of the signals a function depends on, does the output go high, go low,
or keep its previous value? That three-valued answer is the next-state table of a sequential UDP and of
a Liberty `statetable`.

The entry point is:

```rust
pub fn state_regions<B: Brand, C: ManagerCell>(name: &Symbol, f: &Bdd<B, C>) -> StateRegions   // regions.rs:54
```

called once per signal and cached on `AnalysedCell::regions` in `signals()` order (`model.rs:165-167`,
`model.rs:332-337`).

`state_regions` does not build its own BDD from the parsed expression — it is simply handed `f`, the
shared per-cell map's already-**minimised** entry for `name` (the same map `logic::minimise` rewrote and
the machine pass reads from; see `state-machine-arc-engine.md` §3). Columns therefore reflect the
**folded** support: on the real `ICM` cell (`examples/cells.toml:55-72`), the relay `sela` folds into
`sela1` before regions are derived, so `sela1`'s columns gain `enB` and `S` — `sela`'s own referenced
signals — in place of `sela` itself. A purged relay/alias internal (like `sela` here) has no surviving
`AnalysedOutput`, so it has no statetable and no region entry at all.

## 2. The column set: BDD support minus self-feedback

`state_regions` takes its column set from `f`'s own support, with the signal's self-feedback removed
(`regions.rs:59-62`):

```rust
let cols: Vec<Symbol> = f
    .variables()
    .filter(|v| v.as_str() != name.as_str())
    .collect();
```

The consequences are exact:

- **Every signal the function actually depends on becomes a column** — a primary input, another output,
  or an internal state node — because it appears in `f.variables()`.
- **An input the function ignores never appears.** Support comes from the BDD, so a pin the function
  does not reference is simply absent; it is not carried as a spurious don't-care column.
- **The signal's own self-feedback is projected out** and becomes the sequential element's
  current-state (`reg`) column, rather than an input column. It is the only support variable left
  outside `cols`, which is what makes the projection in §3 well defined.

## 3. The three regions by universal projection of the self variable

The regions come from re-basing `f` onto `cols` by **universally** projecting away the self variable,
using `Bdd::cover_over_fr`. For a two-sided FR cover the `F` side is `∀self. f` and the `R` side is
`∀self. ¬f` (`regions.rs:12-19`, `regions.rs:69-78`):

- `on   = ∀self. f`   — the `F` side of `f.cover_over_fr(&cols)` (`regions.rs:69`),
- `off  = ∀self. ¬f`  — the `F` side of `(!f).cover_over_fr(&cols)` (`regions.rs:70`),
- `hold = ¬(on ∨ off)` — the gap the two leave behind (`regions.rs:72-78`).

Because a partial function's on-set and off-set are **not** complementary, the gap between them is
non-empty exactly where the output still depends on the projected self variable — that is, where the
next value is state-dependent. That gap is the **hold** set: the hysteretic region, rendered as the
`-`/`N` no-change entry in the emitted tables. The onset and offset are each taken as a clean `F` cover
(`f_side`, `regions.rs:97-103`); the hold set is reconstructed as its own BDD from the onset and offset
covers so that it, too, can be minimised as an independent onset (`regions.rs:72-78`).

## 4. Each region is minimised independently

Each of the three regions is Espresso-minimised on its own, as its own onset (`regions.rs:82-84`):

```rust
let on   = region_cubes(&minimise(on_cover), &cols);
let off  = region_cubes(&minimise(off_cover), &cols);
let hold = region_cubes(&minimise_bdd(&hold_bdd), &cols);
```

This is safe precisely because no region carries a don't-care set. Minimising an onset with no
don't-cares reproduces that exact region, so minimisation cannot absorb the hold gap into `on` or
`off`. An empty cover minimises to empty (`minimise`, `regions.rs:108-110`; `minimise_bdd`,
`regions.rs:113-115`), which preserves region emptiness and therefore both the `hysteretic` flag (§5)
and the emitters' constant detection (§7).

## 5. The result type

```rust
pub struct StateRegions {   // regions.rs:39-50
    pub cols: Vec<Symbol>,
    pub on: Vec<StateCube>,
    pub off: Vec<StateCube>,
    pub hold: Vec<StateCube>,
    pub hysteretic: bool,
}
```

- `cols` is the column set of §2, in BDD variable order.
- `on`, `off`, `hold` are each a set of cubes. A `StateCube` is `Vec<Option<bool>>` aligned
  position-by-position to `cols`: `Some(true)`/`Some(false)` fixes a column, `None` is a don't-care
  (`StateCube`, `regions.rs:31-33`).
- `hysteretic = !hold.is_empty()` (`regions.rs:86`): a signal is hysteretic exactly when its hold
  region is non-empty, i.e. when it holds on its own state.

Cubes are read out of a region cover by variable **name** with `Minterm::value_of`, so the read is
order-independent and needs no re-homing of the cube onto `cols` (`region_cubes`, `regions.rs:120-125`).

## 6. Worked examples

Drawn from the tests in `regions.rs`. Each test builds its own BDD straight from a cell's parsed
expression (`regions_of`, `regions.rs:136-140`) — none of these fixtures are touched by minimisation, so
this reproduces exactly what the shared-map `state_regions` computes for them.

**C2 C-element** — `Q = A*B + Q*(A+B)` (`state_regions_c_element_self_holds`, `regions.rs:143-163`). `Q`
references only `A`, `B`, and itself, so `Q` is projected out as the `reg` and `cols = [A, B]`. The
regions are:

- `on  = A*B`         — one cube `[Some(true), Some(true)]`,
- `off = !A*!B`       — one cube `[Some(false), Some(false)]`,
- `hold = A xor B`    — two cubes,

and `hysteretic` is true.

**Combinational ND2** — `Y = !(A*B)` (`state_regions_combinational_has_no_hold`, `regions.rs:326-340`).
`Y` does not reference itself, so the onset and offset are complementary, the hold region is empty, and
`hysteretic` is false. The onset is non-empty.

**DFF slave** — `Q = CLK*M + !CLK*Q` with internal master `M = !CLK*D + CLK*M`
(`state_regions_keeps_internal_node_as_column`, `regions.rs:188-209`). `cols = [CLK, M]`: the internal
node `M` stays a column because `Q`'s function depends on it; the primary input `D` drops out because it
is not in `Q`'s support; and `Q` itself is projected out as the `reg`. `hysteretic` is true.

**Equivalence of the minimised regions.** The test `minimised_regions_are_equivalent_to_functions`
(`regions.rs:216-323`) rebuilds a BDD from each region's emitted (minimised) cubes and asserts it is
logically equivalent to the reference region BDD computed straight from the function by universal
projection (`f.forall(&self_state)` for the onset, and the corresponding forms for off and hold). This
proves that minimisation preserved every region's function even though the cube set changed, across the
C2, ND2, DFF, a cross-coupled mutex, and the six-input `RACELEM21` cells.

## 7. Consumers

- **Verilog** (`src/emit/verilog.rs`): the emitter builds one sequential UDP per signal from these
  regions, encoding on as `1`, off as `0`, and hold as `-` (module doc `verilog.rs:1-9`). A
  fully-constant function — one with an empty hold region and one empty polarity region — is lowered
  instead to a plain `module` with a continuous `assign` of `1'b1` or `1'b0` (`verilog.rs:34-42`,
  `verilog.rs:66-68`).
- **Liberty** (`src/emit/liberty.rs`): a hysteretic signal is rendered as a `statetable` whose
  next-state column is `H` (on) / `L` (off) / `N` (hold); internal state nodes appear as
  `direction : internal` pins and as internal-node columns in the tables of the outputs that reference
  them (module doc `liberty.rs:1-5`, `push_signal`/`statetable_group`, `liberty.rs:84-124`).
