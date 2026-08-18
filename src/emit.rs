//! Per-cell output emitters: Liberate arcs (`arcs.tcl`), the structural `define_cell` blocks
//! (`cells.tcl`), behavioural Verilog, and Liberty snippets — over the shared rendering vocabulary in
//! `tcl`, which is what a Liberate command's columns and lists are written in, and `block`, which is
//! one emitted Liberate command as the value it is.

pub mod arcs_tcl;
pub mod block;
pub mod define_cell;
pub mod liberty;
pub(crate) mod statetable;
pub(crate) mod tcl;
pub mod verilog;
