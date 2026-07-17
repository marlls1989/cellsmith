# Changelog

All notable changes to cellsmith are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0/).

## [Unreleased]

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

[Unreleased]: https://github.com/marlls1989/cellsmith/compare/v0.1.2...HEAD
[0.1.2]: https://github.com/marlls1989/cellsmith/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/marlls1989/cellsmith/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/marlls1989/cellsmith/releases/tag/v0.1.0
