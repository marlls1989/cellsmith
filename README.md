# cellsmith

Generate Cadence Liberate **transition arcs** (with prevectors) for logic cells — including
state-holding / hysteretic cells (C-elements, latches, cross-coupled pairs, mutexes, and flip-flops
with internal state) that Liberate cannot auto-detect on non-standard nodes (e.g. nMOS or pMOS only, dynamic logic).

cellsmith is an arc **generator**: it derives the arcs, the behavioural model and a Liberty stub;
Liberate performs the characterisation inside your existing harness. It is a focused Rust tool driven
by a minimal, general (any-gate) TOML input.

> **cellsmith is a command-line tool.** It is distributed as a CLI binary. A library target exists
> only as an internal build artifact (shared by the binary and its benchmarks) and is **not a stable
> or supported API** — it carries no compatibility guarantee across versions, and using cellsmith as a
> library is entirely at your own risk.

## What it produces

For every cell in the input spec, cellsmith emits four artifacts:

| Artifact | File | Contents |
|----------|------|----------|
| Liberate arcs | `<name>_arcs.tcl` | `define_arc` blocks with prevector walks and `R/F/1/0/X` vectors, plus `define_leakage` blocks — one static leakage state per settled seed state (an input assignment that forces the cell into a defined state on its own), conditioned on inputs and settled outputs |
| Behavioural Verilog | `<name>.v` | one sequential UDP `primitive` per signal (outputs + internal state nodes — signals that hold memory — with a three-valued next-state table) + a `celldefine`d wrapper `module` (internals as internal `wire`s) with a `specify` block |
| Liberty stub | `<name>.lib` | a self-contained `library (<name>) { ... }` file (Liberate can consume it directly) wrapping one `cell (...)` per cell: input `pin`s; a sequential cell gets one joint `statetable` listing every state node — emission-minted `_st` aliases for state outputs plus genuine internals — with output pins expressed as spec projections onto it (`internal_node` + `inverted_output`, or `state_function`) and `direction : internal` pins for the hidden nodes; a cell with no state nodes gets a plain `function` per output instead |
| Liberate cell declaration | `<name>_cells.tcl` | `define_cell` blocks: the structural pin declaration (`-input`/`-clock`/`-async`/`-output`/`-pinlist`) and characterisation-template references (`-delay`/`-power`/`-constrain`) from `[cell.template]`/`[cell.template_overrides]` — no logic or timing; one block per distinct resolved `(delay, power, constrain)` triple, bundling the drive-strength aliases that share it. Suppressed by `--no-cells` |

## The model

A cell is a **name**, an ordered list of **inputs**, a Boolean **function per output**, and optionally
some **internal** functions. Two rules make state-holding cells work with no special ceremony:

- **any signal name referenced inside a function is that signal's feedback/delayed value** — so a
  C-element referencing `Q`, an SR pair referencing each other, and a flop's slave referencing its
  master are all just ordinary references;
- whether an **internal** signal (declared under `[cell.internal]`) becomes a **state node** is decided
  by minimisation: before the machine is built, cellsmith **minimises** the model — a pure alias or
  complement of another signal collapses onto that signal's coordinate, and a non-self-holding
  combinational relay is composed into its consumers and dropped. Only internals that survive as
  genuine memory (e.g. a flip-flop's master latch) remain first-class state nodes with **no external
  pin**.

cellsmith treats a cell as an **asynchronous state machine** over `inputs × state-variables`, where a
*state variable* is any signal (output or internal) that sits on a feedback cycle. This
self-reachability check runs over the **already-minimised** model — the raw feedback-cycle rule over
the declared signals would over-count: a one-shot minimisation pass folds aliases/relays first, so what
remains afterwards is genuine memory. For example, a gate-level C-element built from complementary
internals (`IQ = !QN`, `QN = !(A*B + IQ*(A+B))`) collapses from 3 signals to 1 coordinate, and the ICM
cell — a dual-clock synchroniser fixture — has its 8 relay/synchroniser internals fold to 6, taking the
machine's width from 13 raw declared signals down to 11.

### Arcs by state-machine exploration

Timing arcs are derived by exploring that state machine:

1. each state variable's next-state δ is built (folding away combinational signals but **keeping every
   state cycle** — a tight loop is legitimate held state, kept through folding);
2. a breadth-first search runs from stable start states discovered from the signals' forced on/off
   covers, stepping **one input at a time** and letting the state settle;
3. wherever a single input toggle flips an **output**, an arc is emitted.

Three properties follow from this construction:

- **related pins are always primary inputs** — outputs and internal nodes are never arc sources
  (naming one cross-coupled output as the related pin of another would be invalid); they are established
  *indirectly* by the prevector, whose input sequence drives every state variable — internal ones
  included — into the measured edge's start state (e.g. a flop's `CLK→Q` prevector first drives `D` to
  load the master);
- **impossible arcs are never generated** — a mutex's colliding states oscillate (an oscillation
  hazard) instead of settling, so the search drops them, and no arc between its two grants is produced;
- **input-forced transitions cascade through settling** — in a settable cross-coupled pair, toggling a
  set input flips both the output it forces (rise) and, through the coupling, that output's partner
  (fall); the search discovers both.

A cross-coupled cell is also **bistable**: under some input condition (`A·B` for a mutex) the
joint next-state has two stable states, and the physical cell picks one non-deterministically instead
of settling — an **oscillation hazard**, whose physical risk is metastability. cellsmith detects this
during analysis and annotates both the arcs and the Liberty stub with a generic comment naming the
condition, the group of nodes involved and the states it can settle to:

```
# oscillation: A*B risks metastability in {Qa, Qb}, settling to one of {Qa=0, Qb=1} | {Qa=1, Qb=0}
```

(and the equivalent `/* oscillation: ... */` form in the `.lib`). cellsmith always emits a stderr
warning for the same hazard, noting that the hazard is recorded as a comment annotation only. The
hazard is derived from the functions themselves; there is no spec key to declare or silence it. The
arbitration *choice* itself is a physical property Liberate characterises separately, outside
cellsmith's deterministic timing arcs.

The Verilog UDP and Liberty `statetable` are both the **functional** view, but Liberty's spec forces a
different shape. Verilog keeps one sequential UDP per signal, and an output's table may reference
another output directly — the UDP columns are simply that signal's support, projecting out only its own
feedback. The Liberty spec, in contrast, disallows an output pin's own table from referencing another
output pin, so no output pin ever carries state directly there: instead the emitter merges every
sequential cell's state into **one joint `statetable`**, whose rows give the joint next-state of every
state node (genuine internals plus an emission-minted `_st` alias for each state output), and each
output pin is re-expressed as a spec-legal projection onto that one table. Internal nodes appear as
internal `wire`s in the Verilog and `direction : internal` pins in the Liberty, kept off the port
list.

## Input format

A TOML file describing many cells. The `name` field accepts either a single string or a list of
strings; a list generates arcs and models for multiple physical cell variants that share the same
function and interface but differ in drive strength or electrical properties. Cadence Liberate groups
these as a braced list in the arc trailer (e.g. `define_arc ... { INVX1 INVX2 }`).

```toml
[[cell]]
name = "C2"                    # physical cell name used in the arcs
inputs = ["A", "B"]            # ordered: defines pinlist/vector order
[cell.outputs]
Q = "A*B + Q*(A+B)"            # Q on the RHS => feedback/delayed Q (a 2-input C-element)

[[cell]]
name = ["C2X1", "C2X2", "C2X4"] # list form: generates arcs for multiple drive strengths
inputs = ["A", "B"]
[cell.outputs]
Q = "A*B + Q*(A+B)"

[[cell]]
name = "RCELEM2"
inputs = ["A", "B", "R"]
async = ["R"]                  # optional: pins that force the output (async set/reset)
[cell.outputs]                 #   -> their arcs are emitted as `-type async`
Q = "(A*B + Q*(A+B))*!R"

[[cell]]
name = "SR"
inputs = ["S", "R"]
[cell.outputs]
Q  = "!(R + Qn)"               # cross-coupling: each output references the *other*
Qn = "!(S + Q)"

[[cell]]
name = "MUT"
inputs = ["A", "B"]
[cell.outputs]
Qa = "!Qb * A"                 # genuine cross-coupling: each grant references the *other*
Qb = "!Qa * B"                 #   the resulting oscillation hazard is auto-detected

[[cell]]
name = "DFF"
inputs = ["CLK", "D"]
clock = ["CLK"]                # optional: input pins that are clocks. A hazard pair holding exactly
                               #   one clock yields a directed setup/hold constraint (clock <- data);
                               #   any other pair yields a symmetric non_seq
constraint_arcs = true         # optional: opt this cell in to emitting the derived constraint arcs
                               #   (equivalent to the global --constraints flag, per cell)
# no_edge_collapse = true      # optional: opt this cell OUT of the edge classification below
                               #   (equivalent to the global --no-edge-collapse flag, per cell)
# when = true                  # optional: also emit the `-when`-conditioned arcs, per arc class —
                               #   true/"hidden"/["hidden", "transition"]; unioned with the CLI's --when
                               #   selection. The deduplicated arcs without `-when` are always emitted
                               #   regardless; a selected class adds its `-when` arcs on top
[cell.internal]                # internal state node: referenceable, but emits no external pin
M = "!CLK*D + CLK*M"           #   the master latch (transparent low)
[cell.outputs]
Q = "CLK*M + !CLK*Q"           # the slave references the internal master; CLK→Q arcs are discovered,
                               #   and each prevector drives D to load the master first
```

`M` and `Q` are an opposite-phase latch pair on the declared clock `CLK`, so by default cellsmith
recognises, after exploration, the `CLK` rising arc on `Q` as an **edge arc**: `M`'s pin, UDP, and
`statetable` row are elided from every emitted artifact, while its internal-power characterisation
(carried by its primary-input hidden arcs) is unchanged; `Q`'s next state is re-expressed
combinationally in terms of `D`, and its Liberate arc carries `-type edge`. Setting
`no_edge_collapse = true` (or passing `--no-edge-collapse`) keeps the two-latch form written above
exactly as it stands, with `M` staying a separate internal node and `Q`'s arcs discovered by prevector
as before.

Classification is **per arc**: every arc is labelled independently, and the label is the **edge arc**: a
clock toggle that takes a latch from opaque to transparent and whose delivered value depends on retained
latch content. The physical event may be a capture (the value then holds until the next edge) or a latch
opening (the value then tracks its data); both are timing arcs on a clock edge and both emit
`-type edge`, so a plain latch with no capture still carries its opening arc. An arc that does not meet
the definition — a data change through an already-transparent latch, or a clock acting by its level —
stays `-type combinational`. Edge and combinational arcs coexist freely on one output pin (an
async-reset flop carries both). See `docs/edge-collapse.md` for the decision pipeline and its
invariants.

Function syntax: the primary operators are `*` (AND), `+` (OR), `!` (NOT), the constants `1`/`0`, and
parentheses for grouping. The parser is a superset of that form: it also accepts `&` for AND, `|` for
OR, `~` for NOT, `^` for XOR, and `true`/`false` as constants. Precedence, tightest first, is
NOT > AND > XOR > OR. Identifiers are a letter or `_` followed by letters, digits, or `_`, so pin
names like `M1`, `P2`, and `Q` are fine. Every variable in a function must be a declared input, an
output, or an internal signal of the cell.

### Characterisation templates

`[cell.template]` names the characterisation templates the `<name>_cells.tcl` artifact's
`define_cell` blocks attach to the cell: `delay`, `power` and `constrain`, each an optional template
name taken verbatim from the spec (cellsmith never generates or validates the names — Liberate is
the consumer). `[cell.template_overrides.<ALIAS>]` overrides these for one drive-strength alias (a
name from the cell's `name` list); the alias key must be one of the cell's declared names, or it is a
hard error. Overriding merges **per field**: a field set on the override wins, otherwise it falls back
to the cell-wide `[cell.template]` value; a field unset on both means the corresponding
`-delay`/`-power`/`-constrain` flag is omitted for that alias.

Aliases that resolve to the same `(delay, power, constrain)` triple after merging are bundled into one
`define_cell` block, in first-appearance order; an alias whose override changes even one field splits
off into its own block.

```toml
[[cell]]
name = ["INVX1", "INVX2", "INVX3"]
inputs = ["A"]
[cell.outputs]
Y = "!A"
[cell.template]
delay = "inv_delay"
power = "inv_power"
constrain = "inv_constrain"
[cell.template_overrides.INVX2]
delay = "inv_delay_x2"         # only `delay` differs; power/constrain still inherit the default
```

`INVX1` and `INVX3` both resolve to `(inv_delay, inv_power, inv_constrain)` and share one
`define_cell` block naming both; `INVX2` resolves to `(inv_delay_x2, inv_power, inv_constrain)` and
gets its own block.

This cell is a runnable example in `examples/cells.toml`, and its two generated `define_cell` blocks
are in `examples/cells_cells.tcl`.

`define_cell`'s pin flags follow the same clock/async split as the arcs: `-input` lists the plain
data inputs only — clock pins (`clock`) and async pins (`async`) are excluded and instead get their
own `-clock` and `-async` flags, each omitted (like `-input` itself) when its pin set is empty.
`-pinlist` is unaffected by the split: it always lists every pin — inputs, clock pins and async pins,
in declaration order — followed by the outputs. `define_cell` is purely structural: it carries no
`-type`, `-when`, `-related_pin` or `-function`; timing and function live in `<name>_arcs.tcl`, not
here.

## Usage

```
cellsmith [OPTIONS] <SPEC>

Arguments:
  <SPEC>              TOML cell spec to read ("-" reads from stdin)

Options:
  -o, --outdir <OUTDIR>   Directory for the generated files [default: .]
  -n, --name <NAME>       Base name for the output files (defaults to the spec file stem, or "cells" for stdin)
      --when[=<CLASS>]    Also emit the `-when`-conditioned arcs, per arc class (off by default). The
                          deduplicated arcs without `-when` — one per output/related-pin/arc-type/vector
                          combination, keeping the shortest prevector — are always emitted; a selected
                          class adds its `-when` arcs on top, so an arc can appear both with and without
                          its condition. Bare `--when` selects every class; `--when=hidden` /
                          `--when=transition` select one; repeat the flag to select several. A value
                          must be attached with `=` (the space form is not accepted). A cell can select
                          classes itself with `when = ...`, and the two selections are unioned

                          Possible values:
                          - transition: The `define_arc` delay/transition arcs: an input edge on a
                                        related pin driving an output edge
                          - hidden:     The hidden (internal-power) arcs: an input toggle that settles
                                        without changing any output
      --no-internal       Suppress hidden (internal-power) arcs — input toggles where no output changes
                          (emitted by default)
      --no-leakage        Suppress `define_leakage` blocks — static leakage states derived from the
                          machine's settled seed states (emitted by default)
      --no-cells          Suppress the `<base>_cells.tcl` define_cell artifact (emitted by default)
      --constraints       Emit derived setup/hold & non_seq constraint arcs (off by default; a cell can
                          opt in with `constraint_arcs = true`)
      --no-edge-collapse  Suppress the behavioural edge-register annotation (on by default); a cell can
                          opt out individually with `no_edge_collapse = true`
      --stdout            Write all four artifacts to stdout (with banners) instead of writing files
  -h, --help              Print help
  -V, --version           Print version
```

Examples:

```sh
# Write cells_arcs.tcl, cells.v, cells.lib, cells_cells.tcl into ./out
cellsmith cells.toml -o out

# Preview everything on stdout
cellsmith cells.toml --stdout

# Pipe a spec in and name the outputs "mylib"
cat cells.toml | cellsmith - -n mylib -o out
```

Sample Verilog for the C-element above:

```verilog
primitive C2_Q(Q, A, B);
output Q;
input  A, B;
reg    Q;
table
	0 0 : ? : 0;
	0 1 : ? : -;
	1 0 : ? : -;
	1 1 : ? : 1;
endtable
endprimitive
`celldefine
module C2(Q, A, B);
output Q;
input  A, B;
specify
	(A => Q) = (0.1, 0.1);
	(B => Q) = (0.1, 0.1);
endspecify
C2_Q u_C2_Q (Q, A, B);
endmodule
`endcelldefine
```

## Install

```sh
cargo install cellsmith
```

cellsmith's one external requirement is **`clang-devel`** (it provides `libclang`):
[`espresso-logic`](https://crates.io/crates/espresso-logic), the BDD/cover engine, compiles a C FFI at
build time and needs `libclang`. Every Rust dependency is fetched by cargo.

## Build

From a clone, for development:

```sh
cargo build --release
cargo test
```

`cargo install` and a source build both compile espresso-logic's C FFI, so both need the same
`clang-devel` / `libclang` toolchain described under [Install](#install).

## Benchmarks

The [Criterion](https://crates.io/crates/criterion) suite times every pipeline stage across a rayon
thread sweep, from a serial `n=1` baseline up to `max` threads (`rayon::current_num_threads()`).
cellsmith runs multithreaded, and parallelism can regress a stage's cost — intra-cell BDD parallelism
once slowed ~3.7x under write-lock contention — so each stage is reported across the full sweep.

Two targets cover the pipeline at different granularities, both driven off the 9 cells in
`examples/cells.toml`:

- `benches/stages.rs` — per-stage timings, grouped by fixture: `signal` (`parse`, `build_signal_bdds`,
  `minimise`), `machine` (`machine_build`, `arcs_derive`, `confluence_detect`, `analyse_machine`,
  `leakage_derive`, `derive_regions`), and `emit` (`cell_arcs_tcl`, `cell_verilog`, `cell_liberty`).
- `benches/aggregate.rs` — whole-pipeline timings: `whole_cell` (`Cell::analyse` per cell) and
  `whole_run` (the full 9-cell run: `analyse` plus all three emitters and `library_liberty`).

Sweep width follows each stage's cost and parallelism, via `benches/common/mod.rs::sweep`: internally
parallel stages (`machine_build`, `arcs_derive`, `confluence_detect`, `analyse_machine`, and both
aggregate targets) sweep the full `{1, 2, 4, 8, max}` range on the two `HEAVY` cells (`ICM`,
`RACELEM21`); serial stages sweep the flat `{1, max}` on those same cells as a flatness check; every
stage on every cell is additionally measured at `n=max` so the cost gradient across cells is visible
(`max` is `rayon::current_num_threads()`, e.g. `{1, 2, 4, 8}` on an 8-core host).

```sh
cargo bench                    # both targets
cargo bench --bench stages     # per-stage only
cargo bench --bench aggregate  # whole-pipeline only
```

Results (with HTML reports) land under `target/criterion`. To compare before/after a change:

```sh
cargo bench -- --save-baseline before
# make the change
cargo bench -- --baseline before
```

## Dependencies

cargo resolves these Rust crates automatically — the only *external* requirement is `clang-devel`
(see [Install](#install)):

- [`espresso-logic`](https://crates.io/crates/espresso-logic) `5.6.2` — the maintainer's own crate; it
  provides the BDD and cover/minterm engine cellsmith is built on (BDD feedback projection and
  cover/minterm extraction).
- [`liberty-parser`](https://crates.io/crates/liberty-parser) `0.3` — the published Liberty parser
  crate (used as `liberty_parse`); its generic Liberty `Group` trees back the `.lib` emitter.

Plus the standard ecosystem crates: `serde`/`toml` (spec parsing), `clap` (CLI), `indexmap`,
`thiserror`, and `rayon` (parallelism).

## Status and scope

Pins are emitted in **declaration order**. Don't-care cubes are factored via BDD paths, so a function
may render correctly but non-minimally.

The **state-machine** arc engine supports state-holding cells of these shapes: self-holding
C-elements and latches, cross-coupled SR pairs, mutexes / arbiters, and cells with **internal state
nodes** (a master/slave flip-flop). Arcs are found by exploring the settled state machine, so related
pins are always primary inputs, impossible arcs are never reached, input-forced transitions cascade
through settling, and a prevector drives every state variable (internal ones included) into the
measured start state.

The engine detects two kinds of hazard: an **order-dependent** hazard (non-confluence — the settled
state depends on which of a racing input pair's edges lands first; seen on C-elements, DFFs and SR
latches) and an **oscillation** hazard (a bistable condition where the machine picks a settled state
non-deterministically instead of converging on one, as in a mutex/arbiter). From a detected hazard,
cellsmith can **generate** a timing constraint (setup/hold for a pair holding a declared clock,
otherwise a symmetric `non_seq`) to avoid it, gated by the `--constraints` flag or a cell's
`constraint_arcs = true`. cellsmith emits three kinds of per-cell stderr diagnostic: the oscillation 
hazards, the order-dependent hazards (grouped per racing input pair, a pair's conditions joined), 
and the constraints generated to avoid them.

## Licence

MIT.
