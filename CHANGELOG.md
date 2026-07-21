# Changelog

All notable changes to cellsmith are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0/).

## [Unreleased]

## [0.2.0] - 2026-07-21

### Added

- Behavioural per-arc edge-arc classification inside the arc engine. After exploration, every
  clock-related timing arc a cell presents is classified — from the cell's own toggle-and-settle
  behaviour, per arc — as an **edge** arc or ordinary **combinational** propagation. There is one
  category: an edge arc is a clock toggle that takes a latch from opaque to transparent and delivers
  latched content rather than a value that arrives regardless. It is BORN two ways — by generation (a
  latch going opaque→transparent) or by closer-exposure (a mux switch exposing the latch it just closed,
  possible even at an internal node) — and then PROPAGATES transitively to the output. An arc that meets
  neither — a data change through an already-transparent latch, or a clock acting by its level (a clock
  gate) — stays combinational. Edge and combinational arcs coexist freely on one output pin (an
  async-reset flop carries both), and a conditioned edge arc keeps its condition in `-when` rather than
  changing category. Classification is per arc at its full `(output, related clock, direction, machine
  start minterm)` identity, so two firings of one arc that differ only in internal state can type
  differently, and only fully-determinate reachable stable states are measured for arc eligibility. Edge
  arcs emit Liberate `-type edge`, so Liberate characterises them as edge-triggered.
- Read-gated-register factorisation. A register output whose forcing pin merely READS the held state (an
  output-enable) rather than CHANGING it (a reset) is factored: the state-holding register is pulled out
  as its own node with native edge capture and the output becomes a combinational read function over it —
  a Liberty `state_function`, a Verilog continuous assign — so the register's masters can fold without
  destroying the content the output re-acquires when the gate releases. A matching declared register is
  reused up to inversion; otherwise a fresh register node is minted.
- Edge registers re-expressed in edge-triggered form in the Liberty joint `statetable` and the Verilog
  sequential UDP. A captured next-state function is recorded verbatim, so an inverting flop captures `!D`
  and a toggle flop decomposes into two opposite-edge captures. The off-edge of a phase-conditioned
  edge-register clear carries its gating clock literal (`CLK*R`) in the statetable, clearing in that
  clock phase alone. Folding is decided at emission as a reachability question — does this value still
  influence an output once collapsed? — computed as a liveness fixpoint over the candidates'
  raw-function references, so a mutually- or transitively-referencing set of capture-less internal nodes
  folds together whenever the set as a whole reaches no output, exactly as a single self-holding master
  folds; a NAND-implemented flop's cross-coupled master pair therefore folds identically to its
  pass-transistor twin's lone master. A folded node's own pin, UDP primitive, and `statetable` row are
  elided from every artifact; its internal-power characterisation, carried by its primary-input hidden
  arcs, is unchanged. Everything is derived behaviourally, never from equation shape and never by
  branching on a declared input class, so a NAND-implemented cell characterises identically to its
  pass-transistor twin. Classification changes only which form an arc is emitted in — the state-machine
  exploration, the discovered arcs' prevectors, and hazard detection are untouched. On by default; a
  cell can opt out with `no_edge_collapse = true`, and `--no-edge-collapse` opts out every cell in the
  run.
- `examples/sequentials.toml`, a sequential-cell example set (latches and flip-flops) alongside the
  existing `examples/cells.toml`, with its generated `.tcl`/`.v`/`.lib` outputs.

### Changed

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
