# State-table regions: the functional view

How cellsmith derives, for each signal of a cell, the three-valued next-state table that sits behind
the Verilog sequential UDP and the Liberty `statetable`. This is the **functional** view of a signal —
what value it drives, or holds, as a function of the signals it depends on. It is distinct from the
**timing-arc** view produced by the state machine, which `state-machine-arc-engine.md` and
`hazard-detection.md` cover; the two views are computed independently from the same cell functions.

The code lives in one file:

| File | Role |
|------|------|
| `src/logic/regions.rs` | `state_regions`: derive a signal's on/off/hold regions and column set (module doc `regions.rs:1-23`) |

Downstream: `model.rs::analyse` calls `state_regions` once per signal and caches the result on
`AnalysedCell::regions`, in `signals()` order — outputs then internals (`model.rs:123-125`,
`model.rs:256-261`). The Verilog and Liberty emitters read that cache rather than rebuilding the BDDs
(§7).

## 1. Two views of a signal, and which one this is

The state machine treats a cell as an asynchronous machine over `inputs × state-variables` and reports
how single input edges propagate to output edges — the timing arcs. The regions here answer a different
question: for a given assignment of the signals a function depends on, does the output go high, go low,
or keep its previous value? That three-valued answer is the next-state table of a sequential UDP and of
a Liberty `statetable`.

The entry point is:

```rust
pub fn state_regions(output: &AnalysedOutput) -> StateRegions   // regions.rs:52
```

called once per signal and cached on `AnalysedCell::regions` in `signals()` order
(`model.rs:123-125`, `model.rs:256-261`).

## 2. The column set: BDD support minus self-feedback

`state_regions` builds the pin function into a BDD `f` and takes its column set from the function's own
support, with the signal's self-feedback removed (`regions.rs:60-63`):

```rust
let cols: Vec<Symbol> = f
    .variables()
    .filter(|v| v.as_str() != output.name)
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
`∀self. ¬f` (`regions.rs:12-19`, `regions.rs:70-83`):

- `on   = ∀self. f`   — the `F` side of `f.cover_over_fr(&cols)` (`regions.rs:70`),
- `off  = ∀self. ¬f`  — the `F` side of `(!f).cover_over_fr(&cols)` (`regions.rs:71`),
- `hold = ¬(on ∨ off)` — the gap the two leave behind (`regions.rs:73-77`).

Because a partial function's on-set and off-set are **not** complementary, the gap between them is
non-empty exactly where the output still depends on the projected self variable — that is, where the
next value is state-dependent. That gap is the **hold** set: the hysteretic region, rendered as the
`-`/`N` no-change entry in the emitted tables. The onset and offset are each taken as a clean `F` cover
(`f_side`, `regions.rs:97-102`); the hold set is reconstructed as its own BDD from the onset and offset
covers so that it, too, can be minimised as an independent onset (`regions.rs:75-77`).

## 4. Each region is minimised independently

Each of the three regions is Espresso-minimised on its own, as its own onset (`regions.rs:79-83`):

```rust
let on   = region_cubes(&minimise(on_cover), &cols);
let off  = region_cubes(&minimise(off_cover), &cols);
let hold = region_cubes(&minimise_bdd(&hold_bdd), &cols);
```

This is safe precisely because no region carries a don't-care set. Minimising an onset with no
don't-cares reproduces that exact region, so minimisation cannot absorb the hold gap into `on` or
`off`. An empty cover minimises to empty (`minimise`, `regions.rs:107-109`; `minimise_bdd`,
`regions.rs:112-114`), which preserves region emptiness and therefore both the `hysteretic` flag (§5)
and the emitters' constant detection (§7).

## 5. The result type

```rust
pub struct StateRegions {   // regions.rs:39-49
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
  (`StateCube`, `regions.rs:30-32`).
- `hysteretic = !hold.is_empty()` (`regions.rs:85`): a signal is hysteretic exactly when its hold
  region is non-empty, i.e. when it holds on its own state.

Cubes are read out of a region cover by variable **name** with `Minterm::value_of`, so the read is
order-independent and needs no re-homing of the cube onto `cols` (`region_cubes`, `regions.rs:119-124`).

## 6. Worked examples

Drawn from the tests in `regions.rs`.

**C2 C-element** — `Q = A*B + Q*(A+B)` (`regions.rs:132-152`). `Q` references only `A`, `B`, and
itself, so `Q` is projected out as the `reg` and `cols = [A, B]`. The regions are:

- `on  = A*B`         — one cube `[Some(true), Some(true)]`,
- `off = !A*!B`       — one cube `[Some(false), Some(false)]`,
- `hold = A xor B`    — two cubes,

and `hysteretic` is true (`regions.rs:145-151`).

**Combinational ND2** — `Y = !(A*B)` (`regions.rs:314-329`). `Y` does not reference itself, so the
onset and offset are complementary, the hold region is empty, and `hysteretic` is false. The onset is
non-empty.

**DFF slave** — `Q = CLK*M + !CLK*Q` with internal master `M = !CLK*D + CLK*M` (`regions.rs:177-198`).
`cols = [CLK, M]`: the internal node `M` stays a column because `Q`'s function depends on it; the
primary input `D` drops out because it is not in `Q`'s support; and `Q` itself is projected out as the
`reg`. `hysteretic` is true.

**Equivalence of the minimised regions.** The test `minimised_regions_are_equivalent_to_functions`
(`regions.rs:200-312`) rebuilds a BDD from each region's emitted (minimised) cubes and asserts it is
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
