# cellsmith

Generate Cadence Liberate **transition arcs** for logic cells — including
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
| Liberate arcs | `<name>_arcs.tcl` | `define_arc` blocks with `R/F/1/0/X` vectors and an `-ic` start condition, plus `define_leakage` blocks — one static leakage state per fully-initialised reachable rest state, run from a prevector walk where the cell must be walked into it, and stated as the bare condition where the inputs alone drive it there |
| Behavioural Verilog | `<name>.v` | one sequential UDP `primitive` per signal (outputs + internal state nodes — signals that hold memory — with a three-valued next-state table) + a `celldefine`d wrapper `module` (internals as internal `wire`s) with a `specify` block |
| Liberty stub | `<name>.lib` | a self-contained `library (<name>) { ... }` file (Liberate can consume it directly) wrapping one `cell (...)` per cell: input `pin`s; a sequential cell gets one joint `statetable` whose columns live in their own namespace, separate from the pins: a state output **mints** its node (`Q` → `Q_st`, escalating past any real signal of that name), while a genuine internal node keeps its own name. Every node is anchored by a `direction : internal` pin carrying its `internal_node`. Each output pin is then classified against the table: an output that **is** a state node carries a `state_function` naming its minted node, an output that **depends on** state nodes carries a `state_function` over them, and an output over primary inputs alone carries a plain `function`. A cell with no state nodes gets a plain `function` per output and no `statetable` |
| Liberate cell declaration | `<name>_cells.tcl` | `define_cell` blocks: the structural pin declaration (`-input`/`-clock`/`-async`/`-output`/`-pinlist`) and characterisation-template references (`-delay`/`-power`/`-constraint`) from `[cell.template]`/`[cell.template_overrides]` — no logic or timing; one block per distinct resolved `(delay, power, constraint)` triple, bundling the drive-strength aliases that share it. Suppressed by `--no-cells` |

## The model

A cell is a **name**, an ordered list of **inputs**, a Boolean **function per output**, and optionally
some **internal** functions. Two rules make state-holding cells work with no special ceremony:

- **any signal name referenced inside a function is that signal's feedback/delayed value** — so a
  C-element referencing `Q`, an SR pair referencing each other, and a flop's slave referencing its
  master are all ordinary references;
- whether an **internal** signal (declared under `[cell.internal]`) becomes a **state node** is decided
  by minimisation: before the machine is built, cellsmith **minimises** the model — a pure alias or
  complement of another signal collapses onto that signal's coordinate, and a non-self-holding
  combinational relay is composed into its consumers and dropped. Only internals that survive as
  genuine memory (e.g. a flip-flop's master latch) remain first-class state nodes with **no external
  pin**.

cellsmith treats a cell as an **asynchronous state machine** over `inputs × state-variables` — Huffman's
model, in which settling means reaching a **stable state** (`docs/state-machine-arc-engine.md` §5) —
where a *state variable* is any signal (output or internal) that sits on a feedback cycle. This
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
  *indirectly* by `-ic`, which states the level every column starts the measured vector at. cellsmith
  finds that start state by walking its own model — a flop's `CLK→Q` start is reached by first driving
  `D` to load the master — but the walk is not emitted;
- **impossible arcs are never generated** — a mutex's colliding states oscillate (an oscillation
  hazard) instead of settling, so the search drops them, and no arc between its two grants is produced;
- **input-forced transitions cascade through settling** — in a settable cross-coupled pair, toggling a
  set input flips both the output it forces (rise) and, through the coupling, that output's partner
  (fall); the search discovers both;
- **a state-holding cell's arcs carry `-ic`**, the start-state voltage of every `-pinlist` entry, and
  that is the whole of how the start condition reaches Liberate: no `define_arc` emits a `-prevector`.
  A purely combinational cell has no state to establish and carries neither.

A cross-coupled cell is also **bistable**: co-asserting a mutex's two requests walks the joint
next-state around a cycle it never leaves, so the machine reaches no **stable state** — an **oscillation
hazard**, whose physical risk is metastability. An oscillation is annotated on the constraint it
motivated: the comment leads the blocks generated to separate the racing pair, naming the condition the
pair toggles OUT of, the group of nodes involved and the states honouring the timing settles them to:

```
# oscillation: !A*!B risks metastability in {Qa, Qb}, settling to one of {Qa=0, Qb=1} | {Qa=1, Qb=0}
```

(and the equivalent `/* oscillation: ... */` form heading the cell's Liberty stub). A comment explains
what it accompanies, so a ring with no constraint beside it carries none — constraint arcs are opt-in,
through a cell's `constraint_arcs` key or `--constraints` for every input pin of every cell, and a ring
observed under a lone toggle names one pin, which has nothing to be separated from. Detection does not
depend on that selection: the hazard report on stderr carries every hazard, one entry per cause — what
the timing is between — and the state it goes wrong from. The hazard is derived from the functions
themselves; there is no spec key to declare or silence it. The arbitration *choice* itself is a physical
property Liberate characterises separately, outside cellsmith's deterministic timing arcs.

The Verilog UDP and Liberty `statetable` are both the **functional** view, but Liberty's spec forces a
different shape. Verilog keeps one sequential UDP per signal, and an output's table may reference
another output directly — the UDP columns are that signal's support, projecting out only its own
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
constraint_arcs = true         # optional: the input pins this cell's derived constraint arcs are
                               #   generated for — true/"D"/["CLK", "D"]; --constraints selects every
                               #   input pin of every cell. A pin left out is still probed and its
                               #   hazards still reported; it gets no constraint block
# no_edge_collapse = true      # optional: opt this cell OUT of the edge classification below
                               #   (equivalent to the global --no-edge-collapse flag, per cell)
# when = true                  # optional: also emit the `-when`-conditioned arcs, per arc class —
                               #   true/"hidden"/["hidden", "transition"]; unioned with the CLI's --when
                               #   selection. One general arc per transition — a related pin's edge
                               #   driving an output pin's edge — is always emitted, without a `-when`
                               #   line; a selected class adds its `-when` arcs on top
[cell.internal]                # internal state node: referenceable, but emits no external pin
M = "!CLK*D + CLK*M"           #   the master latch (transparent low)
[cell.outputs]
Q = "CLK*M + !CLK*Q"           # the slave references the internal master; CLK→Q arcs are discovered
                               #   by walking D to load the master, and state the start with -ic
```

`M` and `Q` are an opposite-phase latch pair on the declared clock `CLK`, so by default cellsmith
recognises, after exploration, the `CLK` rising arc on `Q` as an **edge arc**: `M`'s pin, UDP, and
`statetable` row are elided from every emitted artifact, while its internal-power characterisation
(carried by its primary-input hidden arcs) is unchanged; `Q`'s next state is re-expressed
combinationally in terms of `D`, and its Liberate arc carries `-type edge`. Setting
`no_edge_collapse = true` (or passing `--no-edge-collapse`) keeps the two-latch form written above
exactly as it stands, with `M` staying a separate internal node and `Q`'s arcs discovered by the same
walk as before.

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

### Exposing internal nodes

`expose` names an ordered list of `[cell.internal]` nodes to carry into the emitted Liberate arcs. Each
listed node gains its own `-pinlist`, `-vector` and `-ic` column, positioned between the inputs and the
outputs in declared order, so the arcs can state the level it starts from, and is preserved through the
state-space minimisation that would otherwise fold it away. Its `-vector` column reads `X` throughout:
that line is the stimulus Liberate holds each named node to, and an internal node the cell drives must
be free to follow the cell rather than be forced against it. `-ic` is where its start level is stated. An exposed node is
never a `-related_pin` or a `-pin` — arc sources and targets remain primary inputs. The Liberty, Verilog,
statetable and `define_cell` artifacts render from the fully minimised model, so exposure does not change
the behaviour they describe. Where the minimisation collapses a group of coordinates that hold the same
value into one, exposure can change which member of the group supplies the surviving name, so an internal
node and the `state_function` that reads it may be written under a different name of that group.

```toml
[[cell]]
name = "DFF"
inputs = ["CLK", "D"]
clock = ["CLK"]
expose = ["M"]                 # carry the master latch into the arcs' -pinlist and -ic
[cell.internal]
M = "!CLK*D + CLK*M"
[cell.outputs]
Q = "CLK*M + !CLK*Q"
```

`[cell.nodes]` says which netlist node an internal signal stands for, for the artifacts Liberate reads.
A spec is written in names that read well in the behavioural model; the netlist may hold that state on a
node spelled otherwise, and Liberate has to be handed its spelling. A signal with no entry stands for
itself, and only the Liberate arcs are affected — the Verilog, Liberty and `define_cell` artifacts carry
the spec's names throughout.

A drive-strength alias may override any of it, since the same signal can sit on a different node in each
alias's netlist. A block addresses an exposed node by one name, so where aliases disagree on it the arcs
fan out into one set per group, as `define_cell` fans out per template triple.

```toml
[[cell]]
name = ["DFFX1", "DFFX4"]
inputs = ["CLK", "D"]
clock = ["CLK"]
expose = ["sela0"]
[cell.internal]
sela0 = "!CLK*D + CLK*sela0"
[cell.outputs]
Q = "CLK*sela0 + !CLK*Q"
[cell.nodes]
sela0 = "XI7/m"                # every alias, unless overridden below
[cell.nodes.DFFX4]
sela0 = "XI4/m"                # this alias only
```

`logic_low` and `logic_high` name the voltage expressions a cell's `-ic` line renders for the two logic
levels, defaulting to `0` and `$VDD`. Either is a Tcl value fragment, so a Tcl variable works as well as
a literal; a cell's own key wins over the `--logic-low`/`--logic-high` command-line value.

`-ic` lists one voltage per `-pinlist` entry and Liberate reads it by position, so each expression has
to occupy a single column — one that splits shifts every column after it. The values go out as one
double-quoted Tcl word, which is what lets `$VDD` reach Liberate as the supply voltage rather than as
that literal text, and Liberate splits the substituted result by Tcl's list rules. An expression that is
already one list element is written as it stands: a bare word (`GND`), a number (`0`, `0.99`), a
variable reference (`$VDD`, `${VDD}`), or a value written as one balanced brace group. Anything else is
wrapped in a brace pair, which makes it one column whatever whitespace the substitution leaves in it —
`--logic-high='$VDD * 0.9'` emits `{$VDD * 0.9}`, a column reading `1.08 * 0.9` where `$VDD` is `1.08`,
and `[expr $VDD*0.9]` emits `{[expr $VDD*0.9]}`, whose command substitution runs before the split and
leaves the column holding the result.

The characters that would end the word or shift the split are escaped, so an expression carrying them
still comes out as one column of a line Tcl reads. A double quote goes out as `\"`, since it would
otherwise close the `-ic` word wherever it sat. A backslash goes out doubled, and a brace with no
partner — the `{` of `--logic-high='{$VDD'`, a stray `}` — goes out backslashed, so the list parser
passes over it instead of looking for a group that is not there. Both escapes are written to survive the
substitution the word goes through first, and the list parser performs no substitution of its own inside
a braced element, so they reach Liberate as text: the expression `a{b` arrives as the column `a\{b`,
backslash and all, and a `\n` written for a newline arrives as a backslash and an `n`. A matched pair of
braces is left as it stands, keeping a group written inside a command substitution or a spaced variable
reference (`${a b}`) intact.

An open bracket that no close bracket reaches goes out as `\[`, one backslash rather than two: a bracket
means nothing to the list, so the escape is spent on the word alone, and the bracket reaches Liberate
without it. A bracket that does close is left as it stands, command substitution being what makes
`[expr $VDD*0.9]` name a level at all.

What a column then means to Liberate is yours to get right: cellsmith keeps the columns aligned with the
`-pinlist`, and no check on the text can tell you what a variable will hold when Liberate runs the
script. A variable that holds whitespace still splits its own column, since the substitution runs after
the escaping and before the split.

### Characterisation templates

`[cell.template]` names the characterisation templates the `<name>_cells.tcl` artifact's
`define_cell` blocks attach to the cell: `delay`, `power` and `constraint`, each an optional template
name taken verbatim from the spec (cellsmith never generates or validates the names — Liberate is
the consumer). `constraint` is also accepted spelled `constrain`. `[cell.template_overrides.<ALIAS>]`
overrides these for one drive-strength alias (a
name from the cell's `name` list); the alias key must be one of the cell's declared names, or it is a
hard error. Overriding merges **per field**: a field set on the override wins, otherwise it falls back
to the cell-wide `[cell.template]` value; a field unset on both means the corresponding
`-delay`/`-power`/`-constraint` flag is omitted for that alias.

Aliases that resolve to the same `(delay, power, constraint)` triple after merging are bundled into one
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
constraint = "inv_constraint"
[cell.template_overrides.INVX2]
delay = "inv_delay_x2"         # only `delay` differs; power/constraint still inherit the default
```

`INVX1` and `INVX3` both resolve to `(inv_delay, inv_power, inv_constraint)` and share one
`define_cell` block naming both; `INVX2` resolves to `(inv_delay_x2, inv_power, inv_constraint)` and
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

The `Options:` block below is a hand-maintained, condensed rendering of `cellsmith --help`. The
standard it is held to is **content** parity, not byte parity: clap lays its columns out against the
terminal width, so wrapping and column positions legitimately differ from any given run.

```
cellsmith [OPTIONS] <SPEC>

Arguments:
  <SPEC>              TOML cell spec ("-" reads stdin)

Options:
  -o, --outdir <OUTDIR>       Output directory [default: .]
  -n, --name <NAME>           Output base name [default: the spec file stem]
      --when[=<CLASS>]        Also emit `-when`-conditioned arcs; bare = every class, repeatable
                              [possible values: transition, hidden, constraint]
      --no-internal           Suppress hidden (internal-power) arcs
      --no-leakage            Suppress `define_leakage` blocks
      --no-cells              Suppress the `<base>_cells.tcl` artifact
      --constraints           Emit derived constraint arcs; every input pin
      --no-edge-collapse      Suppress the edge-register annotation
      --logic-low <VOLTAGE>   Voltage for logic `0` [default: 0]
      --logic-high <VOLTAGE>  Voltage for logic `1` [default: $VDD]
      --stdout                Write the artifacts to stdout instead of to files
      --max-candidates <N>    Ceiling on pooled seed minterms [default: 4194304]
      --max-states <N>        Ceiling on recorded stable states [default: 1048576]
  -h, --help                  Print help
  -V, --version               Print version
```

Exceeding either exploration ceiling is a hard error: cellsmith names every cell whose exploration
stopped there and exits without writing any artifacts, rather than presenting an unexplored cell's
absent arcs and hazards as if that were its behaviour. Raise a ceiling for a run with
`--max-candidates`/`--max-states`.

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
through settling, and the walk into an arc's start state drives every state variable (internal ones
included) to its value there; a state-holding cell's `-ic` line is what carries that start condition
into the measured vector.

A hazard is read on two independent axes. Its **cause** is what the timing is between: a **race**, two
input edges landing close enough together that which of them lands first changes where the cell ends up
(a C-element's `A↓` against `B↑`, a DFF's data against its clock, an SR latch's simultaneous release),
or a **pulse**, the two edges of one input racing each other (a clock pulse too narrow to carry a flop's
master through to its slave leaves the flop somewhere a wider pulse does not). Its **outcome** is what
the machine then does: **indeterminate** — it settles, but which state it settles to is not determined —
or **oscillation** — it never settles, walking a periodic cycle instead of reaching a **stable state**
(`docs/state-machine-arc-engine.md` §5), as a mutex/arbiter does when both requests arrive together. The
axes are independent, so one cause showing both outcomes is reported under each.

A **cause** is a starting state and a transition. From a detected hazard, cellsmith can **generate** a
timing constraint to remove it, and the constraint follows the cause and not the outcome — the same
timing removes a race whether it settles indeterminately or never settles. A racing pair holding a
declared clock gives a setup/hold, any other pair a symmetric `non_seq`, and a pulse a single-pin
`min_pulse_width`. Which pins get one is selected by a cell's `constraint_arcs`, or by `--constraints`
for every input pin of every cell. Selecting pins narrows what is GENERATED: every hazard is detected
and reported whichever pins are selected, so a pin left out keeps its stderr diagnostic and only loses
its blocks.

Naming a pin brings back the constraints that pin has a **role** in, and the roles are the kind's. A
`non_seq` is symmetric — its two pins are equals — so naming **either end** selects the separation that
holds them apart. A setup/hold is directed, the data pin being constrained with respect to the clock, so
it is selected by its **data pin**: naming the clock asks for what the clock is itself subject to — its
own minimum pulse width — and not for the separations other pins are held around it by. A
`min_pulse_width` is selected by the **pin it pulses**.

cellsmith emits two kinds of per-cell stderr diagnostic. The detected hazards come out one entry per
cause — a header naming the timing and the state it goes wrong from, over a body that names the
condition, the walk into that state, and one field per outcome observed listing the nodes THAT reading
puts at risk. The masked arcs come out as the blocks that conflate several cell states — firings a
block cannot tell apart because the state that separates them has no column, which naming those nodes
in `expose` would fix.

Each emitted constraint arc names its **victim nodes** — the state variables whose settled value the
hazard puts at risk, over every outcome the cause showed — in a single `-probe`, so Liberate measures
the nodes the constraint is about. A victim node with no pin of its own, such as a flop's master latch,
is given a `-pinlist` column on that block alone, which its `-ic` states the start level through.

## Known issues

Cells wide enough to panic the espresso-logic dependency during cover expansion are
tracked in [`KNOWN-ISSUES.md`](KNOWN-ISSUES.md).

## Licence

MIT.
