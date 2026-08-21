//! Per-cell output emitters: Liberate arcs (`arcs.tcl`), the structural `define_cell` blocks
//! (`cells.tcl`), behavioural Verilog, and Liberty snippets — over `tcl`, which is what a Liberate
//! command's columns and brace groups are written in, `block`, which is one emitted Liberate command as
//! the value it is, and the crate's own `text`, whose separators every one of the formats writes its
//! lists with.

pub mod arcs_tcl;
pub mod block;
pub mod define_cell;
pub mod liberty;
pub(crate) mod statetable;
pub(crate) mod tcl;
pub mod verilog;
