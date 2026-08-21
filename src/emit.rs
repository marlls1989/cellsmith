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

use crate::logic::regions::StateCube;

/// One region of a node's behaviour together with the next-state action its cubes stamp: the `on`, `off`
/// or `hold` cube set — of a signal's [`StateRegions`](crate::logic::regions::StateRegions), of an edge
/// register's capture, or of that register's off-edge — and what every cube in the set drives the node
/// to. `A` is the emitter's own next-state vocabulary: a Verilog UDP row's state, or a Liberty statetable
/// action. Both emitters walk a node's regions this way, and both name the two components here.
pub(crate) struct RegionAction<'a, A> {
    /// The region's cubes, each over that region's own `cols`.
    pub(crate) cubes: &'a [StateCube],
    /// The next state every one of those cubes puts the node in.
    pub(crate) action: A,
}
