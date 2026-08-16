# Changelog

All notable changes to cellsmith are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0/).

## [Unreleased]

### Added

- **cellsmith detects a pulse too short for the cell to honour, and constrains its minimum width.**
  A pulse is one input's own two edges, driven out of a settled state and driven back. Where the
  cell lands depends on how far the cascade got in between, so a pulse can leave it in a state a
  fully-settled one would never reach, or leave it ringing. Every such pulse yields a
  minimum-pulse-width constraint, emitted as a `-type min_pulse_width` block naming the pulsed pin
  and probing the nodes whose settled value the width decides. Emission is opt-in, as it is for
  every constraint arc: a block is written for the pins a cell's `constraint_arcs` selects, or for
  every input pin of every cell under `--constraints`, and a spec that asks for neither gets none.
  The vector states one edge, so Liberate performs the pulse and searches the width itself, and it
  writes the characterised `min_pulse_width_high`/`_low` into its own output library — the `.lib`
  cellsmith generates is unchanged.

- **`--when=constraint` characterises a constraint in each input context it was observed in.** The
  `-when`-conditioned pass covered the delay/transition and hidden arcs; it now covers the derived
  constraint arcs too, so `--when=constraint` — or `when = "constraint"` on a cell — adds one
  conditioned block per observation. A hazard reachable from ten states used to be kept from one of
  them, the one reached along the shortest walk in, and the other nine were discarded before
  emission saw them; all ten now survive. The observation attacking the widest set of nodes supplies
  the general block that stands for the constraint however it was reached, and every remaining one
  adds a conditioned block over its own nodes, so a conditioned block can probe less than the
  general one beside it.

### Changed

- **A hazard is reported under its cause and its outcome, one report entry per cause.** A cause is
  what the timing is between — two inputs racing each other, or one input's own two edges — and an
  outcome is what the machine then does, settling indeterminately or oscillating. Every combination
  of the two is detected, and a cause observed to do both is reported as both, so a cell that drew
  one warning can draw several. The entry's header names the timing and the state it goes wrong
  from; its body names the condition, the walk into that state, and then one field per outcome
  observed, each listing the victim nodes that reading names and where the machine lands on them
  when the timing is honoured — for a race the alternatives it may settle to, for a pulse the rest
  states an adequately wide one walks through. The header carries no node set, because two outcomes
  of one cause need not attack the same nodes — an SR latch's set pulse rings over `{Q, Qn}` and
  settles indeterminately over `{Q, Qn, L}`. The constraint follows the cause alone: a directed
  setup/hold or a symmetric non-sequential separation for a race, a minimum pulse width for a pulse.

- **A cell that reaches one input assignment in several stored states is constrained in each of
  them.** A state-holding cell arrives at one input assignment in more than one stored state — a
  C-element holds either value under `A*!B` — and those used to be folded into a single constraint
  before emission could see them, so such a cell now emits more constraint blocks. Where a block's
  `-ic` and `-vector` cannot tell two of them apart, the run warns that too few nodes are exposed
  for `-ic` to express the cell state, naming the arc and every state that block conflates, rather
  than the difference vanishing silently. A constraint covers on `-probe` everything its cause
  endangers, since the timing that removes the cause removes every consequence at once.

- **`when` is the input assignment a transition happens FROM, on every arc of every kind.** A
  block's `-when` states the standing assignment its measured transition starts at, and the pins it
  switches are written as edges rather than as literals of the condition. Race-cause hazards used to
  carry the assignment the machine is left in once both edges have landed, which disagreed with the
  `-when` the block beside them wrote: a mutex's ring is now reported and annotated under `!A*!B`,
  the idle state its two requests rise out of, rather than under the `A*B` they land in.

- **A cell with a single input is reported when it rings.** A race is between two pins and needs
  both, while a toggle that leaves the cell ringing around its own feedback needs only the one, so a
  one-input ring — `Q = "!(EN*Q)"` — is reported as an oscillation instead of passing silently.

- **`constraint_arcs` takes a pin name or a list of them as well as `true`, and a name selects the
  constraints that pin has a role in.** The roles are the kind's. A non-sequential separation is
  symmetric — its two pins are equals — so naming either end selects the separation that holds them
  apart. A setup/hold is directed, the data pin being constrained with respect to the clock, so its
  data pin selects it: naming the clock asks for what that clock is itself subject to, its own
  minimum pulse width, and not for the separations other pins are held around it by. A minimum pulse
  width is selected by the pin it pulses. A name that is not one of the cell's declared inputs is a
  spec error — `constraint pin "Q" is not a declared input` — rather than a selection that matches
  nothing.

- **An oscillation is annotated on the constraint it motivated.** The `# oscillation:` comment in
  the emitted Tcl, and the `/* oscillation: … */` form in the `.lib`, lead the constraint block
  generated from the ringing observation instead of heading the file. A comment explains what it
  accompanies, so a ring with no constraint beside it carries none — a ring observed under a lone
  toggle names one pin, and one edge has nothing to be separated from. Every detected hazard reaches
  the user through the report on stderr either way.

## [0.5.1] - 2026-08-08

### Added

- **A warning names every arc no block could state.** An arc should express the cell state it measures
  from, and `-ic` and `-vector` reach exactly the `-pinlist`: firings differing only in an internal
  node with no column all render one block, which then expresses none of them. The warning names the
  arc — `hidden S↑`, `combinational A↑ -> Q↓`, `setup CLK↑ & D↑` — and every cell state that one block
  conflates. They agree on what the block states and differ on what it cannot, so what varies across
  them is the node to expose.

### Changed

- **No `define_arc` carries a `-prevector`, because `-ic` is cheaper.** A prevector is a simulation the
  characterisation run must perform to arrive at the start state; `-ic` states that state outright, and
  every block of a state-holding cell already carried it. A purely combinational cell has no state to
  establish and carries neither. cellsmith still walks its own model to find the state an arc is measured
  from — that walk is what `-ic` and the vector's held columns are read off — it is simply no longer
  emitted. The price is that an internal node left unexposed has no column and so goes unsaid, which is
  what the new conflation warning reports.

- **A `define_leakage` states its condition, and the walk in where the cell needs one.** The `-pinlist`
  and `-vector` are gone from every leakage block: the `-when` already names the inputs held there and
  every output's settled level. A rest state the inputs drive the cell into on their own has nothing to
  prime and so nothing to run — the block is `define_leakage -when "…" { … }` — while a state the cell
  must be walked into runs its `-prevector` to prime the internal nodes.

- **A cell states each `define_arc` once.** Firings that differ only in state no block can carry — an
  internal node with no column, since `-ic` and `-vector` reach exactly the `-pinlist` — render the same
  block, and a repeat hands Liberate the same measurement over again rather than characterising the
  contexts apart. They remain distinct arcs in the model; what is deduplicated is the emitted block,
  keyed on everything it states.

## [0.5.0] - 2026-08-06

### Added

- **A constraint arc names the nodes it protects.** Each emitted setup/hold or non_seq block carries a
  single `-probe` naming the state variables whose settled value the hazard puts at risk, so Liberate
  measures the node the constraint is about — a flop's master latch, for the setup that separates its
  clock from its data. A protected node with no pin of its own is given a `-pinlist` column on that
  block alone, which its `-ic` states the start level through.

- **`[cell.nodes]` says which netlist node an internal signal stands for.** A spec is written in names
  that read well in the behavioural model, while the netlist may hold that state on a node spelled
  otherwise; this hands Liberate the netlist's spelling and leaves the Verilog and Liberty artifacts in
  the spec's. A drive-strength alias may override any of the map under `[cell.nodes.<NAME>]`, the same
  signal being free to sit on a different node in each alias's netlist; where aliases disagree on an
  exposed node the arcs fan out into one set per group.

### Changed

- **A cell's internal signals must resolve to distinct netlist nodes.** `[cell.nodes]` may not put two
  signals on one node, nor a signal on a node named after one of the cell's pins: a netlist holds each
  signal on a node of its own, and a signal sitting on a pin's net is that pin. Both are analyse-time
  errors naming the drive strength they occur under. A spec mapping a signal onto a name already
  spoken for is now rejected where it was previously accepted.

- **`define_leakage` states every state the cell can rest in.** One block per fully-initialised
  reachable rest state, carrying the `-prevector` that primes the cell's internal nodes into it and a
  `-vector` holding the cell's pins at the levels they rest at. A cell leaks differently in each state
  it rests in, and two rest states can share an input assignment while differing in what the cell holds
  — a C-element at `A=1,B=0` with `Q` either way, a mutex at `A=B=1` in whichever grant it arbitrated
  into — so each is its own block.

- **`define_cell` names its characterisation template with `-constraint`.** The spec key is
  `constraint`, also accepted spelled `constrain`.

- **`--help` gives one line per flag.** The behaviour behind the flags is described in the README.

## [0.4.1] - 2026-08-02

### Changed

- **An exposed internal node's `-vector` column reads `X` in every arc block.** The line is the stimulus
  Liberate holds each named node to for the measurement, so stating a level or an edge there forces a
  node the cell itself drives, against the behaviour the arc exists to measure. The node keeps its
  `-pinlist` column and its `-ic` entry, which is where the level it starts from is stated. Transition
  and hidden arcs previously rendered the edge the node made or the level it held; constraint blocks
  already read `X`.

- **A cell's state machine is explored one BFS level at a time, with the level settled in parallel.**
  Settling a toggle is the whole cost of the walk, and the toggles of one frontier are independent, so
  the level is the unit of work: every toggle of every node in it settles at once and the states it
  reaches are collected into one map. On the ICM example this is 149 ms against 184 ms at eight threads
  and 174 ms against 223 ms at four. A node's distance from a seed is unchanged, so every prevector is
  the same length it was. Which of several toggles reaching one state supplies its predecessor is a
  free choice made afresh each run, so an arc may be measured from a different start state — and carry
  a different `-prevector`, `-ic` and `-vector` — between two runs over the same spec. The arcs
  themselves, and the Liberty, Verilog and `define_cell` artifacts, are unaffected.

## [0.4.0] - 2026-08-02

### Added

- **`expose` names internal nodes to carry into the Liberate arcs.** Each node listed, in declared
  order, gains its own `-pinlist`, `-vector` and `-ic` column, positioned between the inputs and the
  outputs, and is preserved through the state-space minimisation so the arcs can state its level. An
  exposed node is never a `-related_pin` or a `-pin`; the Liberty, Verilog, statetable and `define_cell`
  artifacts render from the fully minimised model and are unaffected.
- **A state-holding cell's `define_arc`, hidden-arc and constraint blocks carry `-ic`, the start-state
  voltage of every `-pinlist` entry.** Liberate discards the `-prevector` simulation instead of carrying
  its settled values into the measured vector, so `-ic` states the start condition directly. Purely
  combinational cells carry no `-ic`.
- **`logic_low`/`logic_high` (per cell) and `--logic-low`/`--logic-high` (per run) name the voltage
  expressions `-ic` renders for the two logic levels**, defaulting to `0` and `$VDD`. Recognised simple
  forms are emitted as written; any other value is escaped and wrapped so it occupies exactly one
  `-ic` column. A cell's own key wins over the command-line value.
- **`--max-candidates` and `--max-states` bound a cell's exploration**, charged against the seed
  minterms pooled as initialisation candidates and the reachable stable states the search records — the
  work actually performed, not the cell's declared width. Exceeding either is a hard error naming every
  offending cell; no arcs, hazards, leakage states or constraints are written for it.

### Changed

- **A cell exposing internal nodes now explores its state machine once.** The arc view performs the
  exploration; the model view obtains its own by projecting that exploration onto the coordinates that
  survive the outputs-only minimisation, keyed by label. Both minimisation passes still run — the second
  produces the model view's surviving coordinates and its recomputed state functions — only the second
  exploration is gone. The emitted arcs, hazards, constraints and leakage states are unchanged; the
  difference is in analysis time.

### Fixed

- **Hazards and constraints are now detected only from fully-initialised states.** A probe drawn from a
  state that still carries an absent (uninitialised) column is excluded before confluence detection
  runs, so no hazard or constraint is concluded from a value the machine never actually resolved.

## [0.3.3] - 2026-07-28

### Changed

- **A state output's statetable column is now a minted node with its own internal pin.** The table's
  columns and the cell's pins are separate namespaces; a state output holds its name as a port, so its
  column is minted — `Q` becomes node `Q_st`, escalating to `Q_st2` if the cell already declares a
  signal of that name — and the node gets its own `direction : internal` pin. The output pin then reads
  it with `state_function : "Q_st"`. A genuine internal state node keeps its own name, having no
  competing port. An output that depends on state nodes names those nodes in its `state_function` too,
  so a bare alias of `Q` prints `"Q_st"` and an inverting one `"!Q_st"`.
- **The register factored out of a read-gated output is renamed `Y_st`** (from `Yst`), so both
  node-minting sites share one convention. This changes the node, its internal pin, and the generated
  Verilog UDP/wire names for the affected cells.

### Fixed

- **A Liberty output pin that is a state variable no longer carries an `internal_node`.** Its logic is
  stated in full by `state_function`; the node is anchored by its own internal pin instead.

## [0.3.2] - 2026-07-27

### Fixed

- **A Liberty output pin that is a state variable now carries a `state_function` naming its state-table
  node, alongside the `internal_node` that binds the node to the port.** `internal_node` anchors the
  node; it does not state the pin's output logic, which the pin needs in its own right. An output that
  depends on state nodes, and one combinational over primary inputs, are unaffected.

## [0.3.1] - 2026-07-25

### Changed

- **Under `--when`, an arc whose transition — or hidden-pin toggle — fires from a single context no
  longer emits a conditioned copy.** That context's general block already pins its input levels and
  held outputs in its `-vector`, so a `-when` block on top adds nothing. An arc measured from several
  contexts still emits every discovered firing with its own `-when`. The default output — one general
  arc per transition — is unchanged, so the committed `examples/*_arcs.tcl` are byte-identical.

## [0.3.0] - 2026-07-24

### Changed

- **`--no-when` and the per-cell `no_when` key are removed, with no alias.** `--when` and the
  per-cell `when` key take their place. This is a breaking CLI change against the released 0.2.1: a
  spec that still carries `no_when` fails to parse.
- **The default output carries one general arc per transition — a related pin's edge driving an output
  pin's edge — emitted without a `-when` line**, rather than one arc per condition, so the generated
  `.tcl` changes for every existing spec. Each transition is now characterised in a single context;
  `--when` restores the full set of discovered firings.
- **`--when` and the per-cell `when` key add the selected class's `-when`-conditioned arcs on top of
  the general arcs**, so an arc can appear both with and without its condition. Bare `--when` selects
  every class; `--when=hidden` / `--when=transition` select one; repeat the flag to select several. A
  cell can select classes itself with `when = ...`, unioned with whatever the command line selects.

## [0.2.1] - 2026-07-22

### Added

- **`<base>_cells.tcl`, a fourth generated artifact.** cellsmith now emits Cadence Liberate
  `define_cell` blocks — the structural cell declaration the transition arcs attach to: pins
  (`-input`/`-clock`/`-async`/`-output`/`-pinlist`) and characterisation-template references
  (`-delay`/`-power`/`-constrain`), with no logic or timing. On by default; suppress with the
  `--no-cells` flag.
- **`[cell.template]` and `[cell.template_overrides.<ALIAS>]`.** A cell names its characterisation
  templates (`delay`/`power`/`constrain`, each optional) under `[cell.template]`; a drive-strength
  alias can override them under `[cell.template_overrides.<ALIAS>]`, merged per field (an override
  field wins, otherwise the cell-wide template's field is inherited). Aliases sharing a resolved
  `(delay, power, constrain)` triple are bundled into one `define_cell` block; an override key must
  be one of the cell's declared names.

### Changed

- Boolean expressions for output and internal nodes are now parsed when the cell specification is
  read, reporting any errors at load time with the offending line and column rather than during
  analysis.

## [0.2.0] - 2026-07-22

### Added

- **Support for edge-sensitive flip-flops.** cellsmith now recognises edge-triggered sequential cells
  and emits Cadence Liberate `-type edge` timing arcs for their clock→output transitions, so Liberate
  characterises their edge-triggered timing. Each clock-related arc is classified per arc, from the
  cell's toggle-and-settle behaviour, as an edge arc or ordinary combinational propagation; the decision
  is behavioural, so a cell characterises the same however it is built — a NAND-implemented flop matches
  its pass-transistor equivalent. On by default; opt out per-cell with `no_edge_collapse = true` or
  per-run with `--no-edge-collapse`.
- **Edge-triggered statetable and UDP.** Flip-flops and latches are re-expressed in edge-triggered form
  in the Liberty joint `statetable` and the Verilog sequential UDP: an inverting flop captures `!D`, a
  toggle flop decomposes into two opposite-edge captures, and a phase-conditioned clear carries its
  gating clock literal (`CLK*R`) so it clears in that clock phase alone.
- **Read-gated registers preserved.** When a register output is read through an enable pin, the register
  is emitted as its own edge-triggered node and the output as a combinational `state_function` (a Verilog
  continuous assign) over it, so an output-enabled register is modelled with its held content intact.
- **Internal state nodes folded out of the artifacts.** An internal node that no longer influences an
  output is dropped from the emitted pins, UDP primitives and statetable rows — its power
  characterisation, via its primary-input arcs, unchanged — leaving only the edge-triggered form.
- **`examples/sequentials.toml`**, a sequential-cell example set (latches and flip-flops) alongside
  `examples/cells.toml`, with its generated `.tcl`/`.v`/`.lib` outputs.

### Changed

- The Liberty emitter classifies each sequential cell's output pins independently.
  An output that **is** a state variable emits `internal_node`, one that **depends
  on** a state variable emits `state_function`, and one combinational over inputs
  only emits plain `function` — previously any cell with a state-holding element
  routed all of its outputs through `state_function`, so an inputs-only output
  alongside a state element (e.g. `Z = A*B` next to a latch) was mislabelled. State
  variables now use their own name as their state-table node, dropping the
  emission-time `{name}_st` alias, and the `inverted_output` attribute and its
  projection special-case are removed: a feedthrough or inversion of a single state
  node (`Qn = "!Q"`) renders as `state_function` like any other function of state.
  The emitted SOP strings are unchanged; only the attribute key each output carries
  is chosen correctly. Confined to the Liberty emitter.
- `--no-when` is suppression-only: it omits the `-when` line from each arc and does nothing else, so
  every derived arc emits in both modes and the output differs from the default solely by the absent
  `-when` conditions. Overlapping arcs and same-vector siblings that differ only in internal state or
  prevector are legal in Liberate.

## [0.1.2] - 2026-07-17

### Added

- The cell `name` field now accepts either a single string or a list of strings. A list generates
  arcs and models for multiple physical cell variants that share the same function and interface.
  Multiple names fan across the emitted Tcl arcs (a braced list in `define_arc` and `define_leakage`
  trailers), Liberty (one `cell(...)` group per name with shared pin bodies), and Verilog (one
  wrapper `module` per name over a single shared set of UDP primitives). The scalar form remains
  backward-compatible.
- Reject a cell name that appears in more than one cell (or as an alias colliding with another
  cell's name), which would otherwise emit duplicate Liberty groups and Verilog modules.

### Changed

- cellsmith is a command-line tool. The library target is now documented as an internal build
  artifact shared by the binary and its benchmarks — it is not a supported public API and carries
  no stability guarantee; using cellsmith as a library is at the caller's own risk.

## [0.1.1] - 2026-07-09

### Changed

- State-space minimisation composes over streams of BDDs. The dedup pass unions
  every group's rename and applies it in one `compose_map` stream at pass end;
  the fold pass composes each relay into all its consumers in one stream per
  fold event. Both share a single memo across the stream. The pass is 3–19%
  faster depending on the cell.
- Require espresso-logic 5.6.2, for its `bdd::Composer` trait. Releases 5.6.0
  and 5.6.1 are yanked.

### Fixed

- Intra-doc links that pointed at private or out-of-scope items, so
  `cargo doc` is clean under `-D warnings`.

## [0.1.0] - 2026-07-06

Initial release.

### Added

- Generate Cadence Liberate transition arcs (with prevectors) for logic cells,
  including state-holding and hysteretic cells.
- Detect order-dependent and oscillation hazards and generate the timing
  constraints that resolve them.
- Emit Liberty, Verilog, and Tcl (arc) artefacts from a TOML cell specification.
- Multi-threaded analysis with rayon.

[Unreleased]: https://github.com/marlls1989/cellsmith/compare/v0.5.1...HEAD
[0.5.1]: https://github.com/marlls1989/cellsmith/compare/v0.5.0...v0.5.1
[0.5.0]: https://github.com/marlls1989/cellsmith/compare/v0.4.1...v0.5.0
[0.4.1]: https://github.com/marlls1989/cellsmith/compare/v0.4.0...v0.4.1
[0.4.0]: https://github.com/marlls1989/cellsmith/compare/v0.3.3...v0.4.0
[0.3.3]: https://github.com/marlls1989/cellsmith/compare/v0.3.2...v0.3.3
[0.3.2]: https://github.com/marlls1989/cellsmith/compare/v0.3.1...v0.3.2
[0.3.1]: https://github.com/marlls1989/cellsmith/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/marlls1989/cellsmith/compare/v0.2.1...v0.3.0
[0.2.1]: https://github.com/marlls1989/cellsmith/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/marlls1989/cellsmith/compare/v0.1.2...v0.2.0
[0.1.2]: https://github.com/marlls1989/cellsmith/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/marlls1989/cellsmith/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/marlls1989/cellsmith/releases/tag/v0.1.0
