# Changelog

All notable changes to cellsmith are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0/).

## [Unreleased]

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

[Unreleased]: https://github.com/marlls1989/cellsmith/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/marlls1989/cellsmith/releases/tag/v0.1.0
