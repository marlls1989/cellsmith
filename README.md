# lobsterate

Generate Cadence Liberate **transition arcs** (with prevectors) for logic cells — including
state-holding / hysteretic cells (C-elements, latches, cross-coupled pairs) that Liberate cannot
auto-detect on non-standard nodes (e.g. nMOS).

lobsterate is an arc **generator**, not a characteriser: it derives the arcs, the behavioural model
and a Liberty stub; Liberate still does the actual characterisation inside your existing harness. It
is a focused Rust rebuild of the valuable core of hsNCL's `genLiberateTemplate`, with the entire
YAML/library layer dropped in favour of a minimal, general (any-gate) TOML input.

## What it produces

For every cell in the input spec, lobsterate emits three artifacts:

| Artifact | File | Contents |
|----------|------|----------|
| Liberate arcs | `<name>_arcs.tcl` | `define_arc` blocks with prevector walks and `R/F/1/0/X` vectors |
| Behavioural Verilog | `<name>.v` | one sequential UDP `primitive` per output (three-valued next-state table) + a `celldefine`d wrapper `module` with a `specify` block |
| Liberty stub | `<name>.lib` | a bare `cell (...)` fragment: input `pin`s plus, per output, a `statetable` (hysteretic) or a plain `function` (combinational) |

## The model

A cell is a **name**, an ordered list of **inputs**, and a Boolean **function per output**. The one
rule that makes state-holding cells work: **any output name referenced inside a function is that
output's feedback/delayed value.** This covers self-coupled cells (a C-element referencing `Q`) and
cross-coupled cells (an SR pair referencing each other) with no special declarations.

Each output is split into three regions by projecting out its feedback variables:

- `on`   — forced high regardless of held state,
- `off`  — forced low regardless of held state,
- `hold` — state-dependent, the **hysteretic** region (encoded as `-` no-change in Verilog, `N` in the
  Liberty state table). A purely combinational output has an empty `hold` and degenerates cleanly.

### Interlocked cells (mutexes / arbiters)

When outputs reference a *different* output (genuine cross-coupling, e.g. a mutex `Qa = !Qb·A`,
`Qb = !Qa·B`), the **arc derivation first collapses the coupling**: each output's function has the
other outputs composed away (their functions substituted in) until it is a self-holding function of
primary inputs plus its own feedback only. That collapse is what makes the arcs physically correct:

- **related pins are always primary inputs** — no other output survives to become one (a
  `-related_pin Qb` on `Qa` would be invalid);
- **impossible output→output arcs are never generated** — the mutual-exclusion / deadlock states
  (one grant holds the other low) fall into the `hold` region, so no arc is emitted for them;
- **a forcing input cascades** — a reset that reaches an output only *through* the coupling (with
  `Qb = Sb + !Qa·B`, `Sb` forces `Qb` high which forces `Qa` low) appears in the collapsed function,
  so both `Sb→Qb` and the cascaded `Sb→Qa` arcs are produced.

Such a cell is also **bistable**: under some input condition (`A·B` for the plain mutex) the joint
next-state has two stable states and the physical cell picks one non-deterministically
(metastability). lobsterate **detects** this, annotates the arcs and Liberty stub with the metastable
condition and the mutually-exclusive grants, and warns if the cell did not declare it. Declare the
grants with `arbitrate = ["Qa", "Qb"]` to acknowledge the interlock (validated against detection). The
arbitration *choice* itself is a physical property Liberate characterises separately — it is not, and
cannot be, expressed as a deterministic timing arc.

(The Verilog UDP and Liberty `statetable` keep the *original* function with the other output as an
input column — that is the correct instantaneous functional model, distinct from the collapsed
self-holding view used for timing arcs.)

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
arbitrate = ["Qa", "Qb"]       # optional: declare the mutually-exclusive grants of a mutex/arbiter
[cell.outputs]                 #   -> validated against detection; silences the interlock warning
Qa = "!Qb * A"                 # genuine cross-coupling: each grant references the *other*
Qb = "!Qa * B"
```

Function syntax: `*` (AND), `+` (OR), `!` (NOT), `1`/`0` (constants), parentheses. Every variable in a
function must be a declared input or one of the cell's own outputs.

## Usage

```
lobsterate <SPEC> [OPTIONS]

Arguments:
  <SPEC>              TOML cell spec to read ("-" reads from stdin)

Options:
  -o, --outdir <DIR>  Directory for the generated files [default: .]
  -n, --name <NAME>   Base name for the output files [default: spec file stem]
      --when          Emit `-when` conditions on the arcs (off by default)
      --stdout        Write all three artifacts to stdout (with banners) instead of files
  -h, --help          Print help
  -V, --version       Print version
```

Examples:

```sh
# Write cells_arcs.tcl, cells.v, cells.lib into ./out
lobsterate cells.toml -o out

# Preview everything on stdout
lobsterate cells.toml --stdout

# Pipe a spec in and name the outputs "mylib"
cat cells.toml | lobsterate - -n mylib -o out
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

- [`espresso-logic`](https://crates.io/crates/espresso-logic) `5.1` — BDD feedback projection and
  cover/minterm extraction.
- [`liberty-parse`](https://github.com/marlls1989/liberty-parse) (git) — generic Liberty `Group`
  trees for emitting the `.lib` fragment.

## Status and scope

Deliberate divergences from the hsNCL reference: pins are emitted in **declaration order** (not
alphabetically), the `vclk`/alias/library layer is dropped, and don't-care cubes are factored via BDD
paths rather than Quine–McCluskey — so a function may render correctly but non-minimally.

Interlocked cells (mutexes / arbiters) are supported: the arc path **collapses the cross-coupling**
(composing each output into a self-holding function of primary inputs) so related pins are always
inputs, impossible output→output arcs are never generated, and input-forced transitions cascade
through the coupling. The metastable arbitration point is detected and annotated; the arbitration
*choice* is left to Liberate's physical characterisation — timing arcs cannot express a
non-deterministic next-state, so lobsterate documents it rather than fabricating deterministic
behaviour for it.

## Licence

MIT.
