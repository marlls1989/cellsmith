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
| Liberate arcs | `<name>_arcs.tcl` | `define_arc` blocks with prevector walks and `R/F/1/0/X` vectors |
| Behavioural Verilog | `<name>.v` | one sequential UDP `primitive` per signal (outputs + internal state nodes, three-valued next-state table) + a `celldefine`d wrapper `module` (internals as internal `wire`s) with a `specify` block |
| Liberty stub | `<name>.lib` | a self-contained `library (<name>) { ... }` file (Liberate can consume it directly) wrapping one `cell (...)` per cell: input `pin`s, output/internal `pin`s (`direction : internal` for state nodes), each with a `statetable` (hysteretic) or a plain `function` (combinational) |

## The model

A cell is a **name**, an ordered list of **inputs**, a Boolean **function per output**, and optionally
some **internal** functions. Two rules make state-holding cells work with no special ceremony:

- **any signal name referenced inside a function is that signal's feedback/delayed value** — so a
  C-element referencing `Q`, an SR pair referencing each other, and a flop's slave referencing its
  master are all just ordinary references;
- an **internal** signal (declared under `[cell.internal]`) is a first-class **state node** that other
  functions may reference, but which has **no external pin** — it models hidden state like a
  flip-flop's master latch.

cellsmith treats a cell as an **asynchronous state machine** over `inputs × state-variables`, where a
*state variable* is any signal (output or internal) that sits on a feedback cycle. Combinational
signals are folded away; only true state remains as machine state.

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
- **impossible arcs are never generated** — a mutex's deadlock/metastable states oscillate instead of
  settling, so the search drops them, and no `Qb→Qa` arc is produced;
- **input-forced transitions cascade through settling** — with `Qb = Sb + !Qa·B`, toggling `Sb` flips
  both `Qb` (rise) and, through the coupling, `Qa` (fall); the search discovers both.

A cross-coupled cell is also **bistable**: under some input condition (`A·B` for the plain mutex) the
joint next-state has two stable states and the physical cell picks one non-deterministically
(metastability). cellsmith detects this during analysis, annotates the arcs and the Liberty stub with
the metastable condition and the mutually-exclusive grants, and always emits a stderr warning naming
the interlocked nodes and the metastable condition, noting that it is annotated only, not modelled as
timing. This interlock is derived from the functions themselves; there is no spec key to declare or
silence it. The arbitration *choice* itself is a physical property Liberate characterises separately —
it is not, and cannot be, expressed as a deterministic timing arc.

The Verilog UDP and Liberty `statetable` are the **functional** view: each keeps the other referenced
state signals (other outputs *and* internal nodes) as input/internal-node columns and projects out
only the signal's own feedback. Internal nodes appear as internal `wire`s in the Verilog and
`direction : internal` pins in the Liberty — modelled, but not exposed as ports.

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
Q  = "S + Q*!R"                # each output references only its own held state
Qn = "R + Qn*!S"

[[cell]]
name = "MUT"
inputs = ["A", "B"]
[cell.outputs]
Qa = "!Qb * A"                 # genuine cross-coupling: each grant references the *other*
Qb = "!Qa * B"                 #   arbitration is auto-detected during analysis (no spec key)

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
measured start state. The metastable arbitration point is detected and annotated; the arbitration
*choice* is left to Liberate's physical characterisation, since timing arcs cannot express a
non-deterministic next-state.

## Licence

MIT.
