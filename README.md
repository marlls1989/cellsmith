# cellsmith

Generate Cadence Liberate **transition arcs** (with prevectors) for logic cells — including
state-holding / hysteretic cells (C-elements, latches, cross-coupled pairs, mutexes, and flip-flops
with internal state) that Liberate cannot auto-detect on non-standard nodes (e.g. nMOS).

cellsmith is an arc **generator**, not a characteriser: it derives the arcs, the behavioural model
and a Liberty stub; Liberate still does the actual characterisation inside your existing harness. It
is a focused Rust rebuild of the core of hsNCL's `genLiberateTemplate`, with the entire
YAML/library layer dropped in favour of a minimal, general (any-gate) TOML input.

## What it produces

For every cell in the input spec, cellsmith emits three artifacts:

| Artifact | File | Contents |
|----------|------|----------|
| Liberate arcs | `<name>_arcs.tcl` | `define_arc` blocks with prevector walks and `R/F/1/0/X` vectors, plus `define_leakage` blocks — one static leakage state per settled seed state, conditioned on inputs and settled outputs |
| Behavioural Verilog | `<name>.v` | one sequential UDP `primitive` per signal (outputs + internal state nodes, three-valued next-state table) + a `celldefine`d wrapper `module` (internals as internal `wire`s) with a `specify` block |
| Liberty stub | `<name>.lib` | a self-contained `library (<name>) { ... }` file (Liberate can consume it directly) wrapping one `cell (...)` per cell: input `pin`s; a sequential cell gets one joint `statetable` listing every state node — emission-minted `_st` aliases for state outputs plus genuine internals — with output pins expressed as spec projections onto it (`internal_node` + `inverted_output`, or `state_function`) and `direction : internal` pins for the hidden nodes; a cell with no state nodes gets a plain `function` per output instead |

## The model

A cell is a **name**, an ordered list of **inputs**, a Boolean **function per output**, and optionally
some **internal** functions. Two rules make state-holding cells work with no special ceremony:

- **any signal name referenced inside a function is that signal's feedback/delayed value** — so a
  C-element referencing `Q`, an SR pair referencing each other, and a flop's slave referencing its
  master are all just ordinary references;
- an **internal** signal (declared under `[cell.internal]`) *may* become a **state node**, but is not
  automatically one: before the machine is built, cellsmith **minimises** the model — a pure alias or
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
cell's 8 relay/synchroniser internals fold to 6, taking the machine's width from 13 raw declared
signals down to 11.

### Arcs by state-machine exploration

Timing arcs are derived by exploring that state machine:

1. each state variable's next-state δ is built (folding away combinational signals but **keeping every
   state cycle** — a tight loop is legitimate held state, never substituted);
2. a breadth-first search runs from stable start states discovered from the signals' forced on/off
   covers, stepping **one input at a time** and letting the state settle;
3. wherever a single input toggle flips an **output**, an arc is emitted.

Three properties follow from this construction:

- **related pins are always primary inputs** — outputs and internal nodes are never arc sources
  (a `-related_pin Qb` on `Qa` would be invalid); they are established *indirectly* by the prevector,
  whose input sequence drives every state variable — internal ones included — into the measured edge's
  start state (e.g. a flop's `CLK→Q` prevector first drives `D` to load the master);
- **impossible arcs are never generated** — a mutex's colliding states oscillate (an oscillation
  hazard) instead of settling, so the search drops them, and no `Qb→Qa` arc is produced;
- **input-forced transitions cascade through settling** — with `Qb = Sb + !Qa·B`, toggling `Sb` flips
  both `Qb` (rise) and, through the coupling, `Qa` (fall); the search discovers both.

A cross-coupled cell is also **bistable**: under some input condition (`A·B` for the plain mutex) the
joint next-state has two stable states, and the physical cell picks one non-deterministically instead
of settling — an **oscillation hazard**, whose physical risk is metastability. cellsmith detects this
during analysis and annotates both the arcs and the Liberty stub with a generic comment naming the
condition, the group of nodes involved and the states it can settle to:

```
# oscillation: A*B risks metastability in {Qa, Qb}, settling to one of {Qa=0, Qb=1} | {Qa=1, Qb=0}
```

(and the equivalent `/* oscillation: ... */` form in the `.lib`). cellsmith always emits a stderr
warning for the same hazard, noting that it is annotated only, not modelled as timing. The hazard is
derived from the functions themselves; there is no spec key to declare or silence it. The arbitration
*choice* itself is a physical property Liberate characterises separately — it is not, and cannot be,
expressed as a deterministic timing arc.

The Verilog UDP and Liberty `statetable` are both the **functional** view, but Liberty's spec forces a
different shape. Verilog keeps one sequential UDP per signal, and an output's table may reference
another output directly — the UDP columns are simply that signal's support, projecting out only its own
feedback. The Liberty spec, in contrast, disallows an output pin's own table from referencing another
output pin, so no output pin ever carries state directly there: instead the emitter merges every
sequential cell's state into **one joint `statetable`**, whose rows give the joint next-state of every
state node (genuine internals plus an emission-minted `_st` alias for each state output), and each
output pin is re-expressed as a spec-legal projection onto that one table. Internal nodes appear as
internal `wire`s in the Verilog and `direction : internal` pins in the Liberty — modelled, but not
exposed as ports.

## Input format

A TOML file describing many cells:

```toml
[[cell]]
name = "C2"                    # physical cell name used in the arcs
inputs = ["A", "B"]            # ordered: defines pinlist/vector order
[cell.outputs]
Q = "A*B + Q*(A+B)"            # Q on the RHS => feedback/delayed Q (a 2-input C-element)

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
Qb = "!Qa * B"                 #   the resulting oscillation hazard is auto-detected (no spec key)

[[cell]]
name = "DFF"
inputs = ["CLK", "D"]
clock = ["CLK"]                # optional: input pins that are clocks. A hazard pair holding exactly
                               #   one clock yields a directed setup/hold constraint (clock <- data);
                               #   any other pair yields a symmetric non_seq
constraint_arcs = true         # optional: opt this cell in to emitting the derived constraint arcs
                               #   (equivalent to the global --constraints flag, per cell)
[cell.internal]                # internal state node: referenceable, but emits no external pin
M = "!CLK*D + CLK*M"           #   the master latch (transparent low)
[cell.outputs]
Q = "CLK*M + !CLK*Q"           # the slave references the internal master; CLK→Q arcs are discovered,
                               #   and each prevector drives D to load the master first
```

Function syntax: the primary operators are `*` (AND), `+` (OR), `!` (NOT), the constants `1`/`0`, and
parentheses for grouping. The parser is a superset of that form: it also accepts `&` for AND, `|` for
OR, `~` for NOT, `^` for XOR, and `true`/`false` as constants. Precedence, tightest first, is
NOT > AND > XOR > OR. Every variable in a function must be a declared input, an output, or an internal
signal of the cell.

## Usage

```
cellsmith [OPTIONS] <SPEC>

Arguments:
  <SPEC>              TOML cell spec to read ("-" reads from stdin)

Options:
  -o, --outdir <OUTDIR>  Directory for the generated files [default: .]
  -n, --name <NAME>      Base name for the output files [default: spec file stem, or "cells" for stdin]
      --no-when          Suppress the `-when` conditions on arcs (emitted by default); with them
                         suppressed, arcs sharing a (related, pin, edge) collapse to one
      --no-internal      Suppress hidden (internal-power) arcs — input toggles where no output
                         changes (emitted by default)
      --no-leakage       Suppress `define_leakage` blocks — static leakage states derived from the
                         machine's settled seed states (emitted by default)
      --constraints      Emit derived setup/hold & non_seq constraint arcs (off by default; a cell can
                         opt in with `constraint_arcs = true`)
      --stdout           Write all three artifacts to stdout (with banners) instead of files
  -h, --help             Print help
  -V, --version          Print version
```

Examples:

```sh
# Write cells_arcs.tcl, cells.v, cells.lib into ./out
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

## Build

```sh
cargo build --release
cargo test
```

Requirements:

- A C toolchain and **libclang** — [`espresso-logic`](https://crates.io/crates/espresso-logic) (the
  BDD / cover engine) builds a C FFI.
- Git dependencies are fetched through the system `git` (configured in `.cargo/config.toml` via
  `net.git-fetch-with-cli`), so a working `git` on `PATH` is needed for the first build.

## Benchmarks

The [Criterion](https://crates.io/crates/criterion) suite treats every benchmark target as a pipeline
stage measured across a rayon thread sweep. Multithreaded execution is cellsmith's production mode, so
the swept numbers are the primary signal; `n=1` is simply the baseline point of that same sweep, not a
separate single-thread mode. A per-stage regression under parallelism can be invisible at `n=1` — intra-cell
BDD parallelism once regressed ~3.7x from write-lock contention while single-thread timings stayed flat —
which is why the swept multithread measurements, not a single-thread snapshot, are what the suite reports.

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

- [`espresso-logic`](https://crates.io/crates/espresso-logic) `5.4` — the maintainer's own crate; it
  provides the BDD and cover/minterm engine cellsmith is built on (BDD feedback projection and
  cover/minterm extraction).
- [`liberty-parse`](https://github.com/marlls1989/liberty-parse) (git) — generic Liberty `Group`
  trees for emitting the `.lib` file.

## Status and scope

Deliberate divergences from the hsNCL reference: pins are emitted in **declaration order** (not
alphabetically), the `vclk`/alias/library layer is dropped, and don't-care cubes are factored via BDD
paths rather than Quine–McCluskey — so a function may render correctly but non-minimally.

The **state-machine** arc engine supports state-holding cells of these shapes: self-holding
C-elements and latches, cross-coupled SR pairs, mutexes / arbiters, and cells with **internal state
nodes** (a master/slave flip-flop). Arcs are found by exploring the settled state machine, so related
pins are always primary inputs, impossible arcs are never reached, input-forced transitions cascade
through settling, and a prevector drives every state variable (internal ones included) into the
measured start state.

The engine detects two kinds of hazard: an **order-dependent** hazard (non-confluence — the settled
state depends on which of a racing input pair's edges lands first; seen on C-elements, DFFs and SR
latches) and an **oscillation** hazard (a bistable condition where the machine picks a settled state
non-deterministically instead of converging on one, as in a mutex/arbiter). Metastability is the
shared *physical risk* of both — never a name for either hazard on its own. From a detected hazard,
cellsmith can **generate** a timing constraint (setup/hold for a pair holding a declared clock,
otherwise a symmetric `non_seq`) to avoid it, gated by the `--constraints` flag or a cell's
`constraint_arcs = true`; the constraint is the remedy, not the hazard. cellsmith emits three
kinds of per-cell stderr diagnostic: the oscillation hazards, the order-dependent hazards (grouped
per racing input pair, a pair's conditions joined), and the constraints generated to avoid them.

## Licence

MIT.
