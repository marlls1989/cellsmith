# Changelog

All notable changes to cellsmith are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0/).

## [Unreleased]

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

[Unreleased]: https://github.com/marlls1989/cellsmith/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/marlls1989/cellsmith/compare/v0.1.2...v0.2.0
[0.1.2]: https://github.com/marlls1989/cellsmith/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/marlls1989/cellsmith/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/marlls1989/cellsmith/releases/tag/v0.1.0
