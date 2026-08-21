//! Emit a behavioural Verilog model for a cell: one sequential UDP `primitive` per output pin (its
//! three-valued next-state table built from the on/off/hold regions) wrapped in a `celldefine`d
//! `module` that instantiates the primitives and carries a `specify` block of path delays.
//!
//! The UDP for output `x` takes the pin's own state as the `reg`/current-state column and every other
//! signal (primary inputs + other outputs) as an input column, so a self-holding cell keeps its
//! hysteresis as `-` (no-change) rows. Pins are emitted in declaration order.
//!
//! A signal recognised as an edge-triggered register (`crate::logic::edge`) emits an
//! **edge-sensitive** UDP instead: the level-latch rows are replaced by clock-edge (`(01)`/`(10)`)
//! capture rows — one group per active `(clock, edge)`, so a dual-edge register captures on both — plus
//! async set/clear level rows, a no-change row for each clock's inactive edge and no-change rows for
//! steady-clock data transitions. The keying clocks are the primitive's LAST ports. A pure master folded
//! into such a register contributes nothing — no primitive, no wire, no instance.
//!
//! A cell's declarations travel as the values [`cell_verilog`] states — one [`Item`] apiece — and become
//! text once, in [`Display`](fmt::Display), written into the writer the model is going out on.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use espresso_logic::Symbol;

use crate::emit::RegionAction;
use crate::logic::arcs::{Edge, PinEdge};
use crate::logic::edge::EdgeCaptures;
use crate::logic::regions::{StateCube, StateRegions};
use crate::model::AnalysedCell;
use crate::text::Joined;

/// Fixed rise/fall path delay stamped on every `specify` arc.
const PATH_DELAY: &str = "(0.1, 0.1)";

/// One top-level declaration of a cell's Verilog model, the variant being what that declaration is: a
/// signal's UDP, a constant pin's module, or a wrapper module instantiating them.
pub enum Item<'a> {
    /// One signal's sequential UDP, its table the signal's on/off/hold regions.
    Primitive(Primitive<'a>),
    /// One edge register's edge-sensitive UDP, its table that register's captures.
    EdgeRegister(EdgePrimitive<'a>),
    /// A constant output pin, which is a `module` with a continuous assignment rather than a UDP.
    Constant(Constant<'a>),
    /// The `celldefine`d wrapper for one of the cell's declared names.
    Wrapper(Wrapper<'a>),
}

impl fmt::Display for Item<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Item::Primitive(p) => p.fmt(f),
            Item::EdgeRegister(p) => p.fmt(f),
            Item::Constant(c) => c.fmt(f),
            Item::Wrapper(w) => w.fmt(f),
        }
    }
}

/// A run's Verilog declarations as the text they write: each item in turn, written into the writer the
/// `.v` is going out on. Every cell's declarations make up the one model file, so this holds them all.
pub struct Verilog<'a>(pub &'a [Item<'a>]);

impl fmt::Display for Verilog<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for item in self.0 {
            write!(f, "{item}")?;
        }
        Ok(())
    }
}

/// The full Verilog model for a cell: a UDP primitive per signal (outputs **and** internal state
/// nodes) followed by the wrapper module. Internal nodes are modelled exactly like outputs, but appear
/// as internal `wire`s in the wrapper rather than as module ports.
pub fn cell_verilog(cell: &AnalysedCell) -> Vec<Item<'_>> {
    // Recognised edge registers, keyed by their output node, and the pure masters folded into them —
    // a folded master emits no primitive, no wire and no instance.
    let edge_by_node: BTreeMap<&str, &EdgeCaptures> = cell
        .edge
        .captures
        .iter()
        .map(|er| (er.node.as_str(), er))
        .collect();
    let folded: BTreeSet<&str> = cell.edge.folded.iter().map(Symbol::as_str).collect();
    // Read-gated outputs read a factored register combinationally: they emit a continuous `assign` in the
    // wrapper, no UDP of their own. Their factored register (minted, not a declared signal) emits an
    // edge-sensitive UDP like any register.
    let signal_names: BTreeSet<&str> = cell
        .signal_regions()
        .map(|(s, _)| s.name.as_str())
        .collect();

    let mut items: Vec<Item> = Vec::new();
    for (sig, sr) in cell.signal_regions() {
        if folded.contains(sig.name.as_str()) {
            continue; // pure master folded into its edge register
        }
        if cell.edge.factored.contains(&sig.name) {
            continue; // a read-gated output is a continuous assign, not a UDP
        }
        let name = PrimName::new(cell, &sig.name);
        items.push(match edge_by_node.get(sig.name.as_str()) {
            Some(er) => Item::EdgeRegister(EdgePrimitive { name, captures: er }),
            None => signal_item(name, sr),
        });
    }
    // The minted derived registers: an edge-sensitive UDP from their EdgeCaptures.
    for d in &cell.edge.derived {
        if signal_names.contains(d.name.as_str()) {
            continue; // a reused declared register already emitted its UDP above
        }
        if let Some(er) = edge_by_node.get(d.name.as_str()) {
            items.push(Item::EdgeRegister(EdgePrimitive {
                name: PrimName::new(cell, &d.name),
                captures: er,
            }));
        }
    }
    // One `celldefine`d wrapper per name; all wrappers instantiate the same shared primitives.
    for name in &cell.name {
        items.push(Item::Wrapper(wrapper(cell, name, &edge_by_node, &folded)));
    }
    items
}

/// The cell's read-gated outputs mapped to their combinational read function over the factored register
/// and gate pins (the read-gate factorisation). Empty for a cell with no such output.
fn read_functions(cell: &AnalysedCell) -> BTreeMap<&str, &StateRegions> {
    cell.edge
        .derived
        .iter()
        .flat_map(|d| d.reads.iter().map(|r| (r.output.as_str(), &r.function)))
        .collect()
}

/// `<cell>_<pin>` — one signal's UDP primitive name, held as the two names it is made of: the cell's
/// representative name and the pin the UDP models. That pin is the primitive's own port, so a
/// declaration reads it from here rather than carrying a second copy that could disagree.
#[derive(Clone, Copy)]
struct PrimName<'a> {
    cell: &'a Symbol,
    pin: &'a Symbol,
}

impl<'a> PrimName<'a> {
    fn new(cell: &'a AnalysedCell, pin: &'a Symbol) -> Self {
        PrimName {
            cell: cell.repr_name(),
            pin,
        }
    }
}

impl fmt::Display for PrimName<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}_{}", self.cell, self.pin)
    }
}

/// One level-sensitive signal's declaration: a constant function is a plain `module` with a continuous
/// assignment, anything else the sequential UDP whose table encodes on (`1`), off (`0`) and hold (`-`).
fn signal_item<'a>(name: PrimName<'a>, sr: &'a StateRegions) -> Item<'a> {
    // Constant pin: no hold and one region empty ⇒ a tautology / contradiction.
    if sr.hold.is_empty() && sr.off.is_empty() {
        return Item::Constant(Constant { name, value: true });
    }
    if sr.hold.is_empty() && sr.on.is_empty() {
        return Item::Constant(Constant { name, value: false });
    }
    Item::Primitive(Primitive { name, regions: sr })
}

/// One output pin's sequential UDP: the pin is the `reg`/current-state column and the regions' columns
/// are its input ports.
pub struct Primitive<'a> {
    name: PrimName<'a>,
    regions: &'a StateRegions,
}

impl fmt::Display for Primitive<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (name, pin, sr) = (self.name, self.name.pin, self.regions);
        let ports = Joined::new(
            std::iter::once(pin).chain(sr.cols.iter()),
            ", ",
            std::convert::identity,
        );
        writeln!(f, "primitive {name}({ports});")?;
        writeln!(f, "output {pin};")?;
        if !sr.cols.is_empty() {
            let cols = Joined::new(sr.cols.iter(), ", ", std::convert::identity);
            writeln!(f, "input  {cols};")?;
        }
        writeln!(f, "reg    {pin};")?;
        writeln!(f, "table")?;
        for row in table_rows(sr) {
            writeln!(f, "\t{row}")?;
        }
        writeln!(f, "endtable")?;
        writeln!(f, "endprimitive")
    }
}

/// A constant output pin as a `module` with a continuous assignment (`1'b1` / `1'b0`).
pub struct Constant<'a> {
    name: PrimName<'a>,
    value: bool,
}

impl fmt::Display for Constant<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (name, pin) = (self.name, self.name.pin);
        let bit = if self.value { "1'b1" } else { "1'b0" };
        write!(
            f,
            "module {name}({pin});\noutput {pin};\nassign {pin} = {bit};\nendmodule\n"
        )
    }
}

/// The UDP table rows: one per region cube of the signal's on, off and hold regions.
fn table_rows(sr: &StateRegions) -> Vec<TableRow<'_>> {
    let mut rows: Vec<TableRow> = Vec::new();
    for RegionAction { cubes, action } in [
        RegionAction {
            cubes: &sr.on,
            action: Next::On,
        },
        RegionAction {
            cubes: &sr.off,
            action: Next::Off,
        },
        RegionAction {
            cubes: &sr.hold,
            action: Next::Hold,
        },
    ] {
        rows.extend(cubes.iter().map(|cube| TableRow { cube, next: action }));
    }
    // IEEE 1364 matches a UDP row by its pattern and resolves an overlap by rule, never by a row's
    // position, so a consumer reads the table as a set of rows. This order over the row values is the
    // tool's own and carries nothing to that consumer.
    rows.sort();
    rows
}

/// One row of a level-sensitive UDP table: the region cube it matches over the signal's columns and the
/// state the pin takes there. The current-state (`reg`) field is `?` — a level row matches on the input
/// columns alone, and a hold row is what carries the pin's prior state forward.
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct TableRow<'a> {
    cube: &'a StateCube,
    next: Next,
}

impl fmt::Display for TableRow<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} : ? : {};", Pattern(self.cube), self.next)
    }
}

/// Where a UDP table row leaves the pin: driven high (`1`), driven low (`0`) or unchanged (`-`).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Next {
    On,
    Off,
    Hold,
}

impl fmt::Display for Next {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Next::On => "1",
            Next::Off => "0",
            Next::Hold => "-",
        })
    }
}

/// A cube as space-separated UDP table columns, one [`Level`] per column of the header order.
struct Pattern<'a>(&'a StateCube);

impl fmt::Display for Pattern<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Joined::new(self.0.iter(), " ", |val: &Option<bool>| Level(*val)).fmt(f)
    }
}

/// One column value as a Verilog UDP table symbol: `1` high, `0` low, `?` any.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Level(Option<bool>);

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self.0 {
            Some(true) => "1",
            Some(false) => "0",
            None => "?",
        })
    }
}

/// The register's DATA columns: `er.cols` with the register's own symbol and every keying clock removed.
/// A self-referencing register (a toggle flop, whose capture depends on its own prior state) carries its
/// own node in `er.cols`; that node is the UDP's `reg` current-state, not an input port. A multi-clock
/// register carries the OTHER clocks' levels in a conditioned capture's cols; those clocks are the
/// primitive's dedicated trailing clock columns, not data ports. Both are excluded here (for a single
/// clock the clock is never in `er.cols`, so this reduces to removing the register's own symbol).
fn data_cols(er: &EdgeCaptures) -> Vec<&Symbol> {
    let clocks = er.clocks();
    er.cols
        .iter()
        .filter(|c| **c != er.node && !clocks.contains(c))
        .collect()
}

/// One edge-register signal's UDP: an edge-sensitive sequential `primitive` whose ports are
/// `(pin, data cols…, clocks…)` with the keying clocks LAST in `EdgeCaptures::clocks` order. The `reg`
/// captures on each active clock edge (`(01)` for `Rise`, `(10)` for `Fall`) and honours async set/clear
/// as clock-independent level rows. The register's own symbol (a toggle flop's self-feedback) and the
/// clocks are excluded from the data columns — the self column is the `reg` current-state and the clocks
/// are the dedicated trailing clock columns, not input ports.
pub struct EdgePrimitive<'a> {
    name: PrimName<'a>,
    captures: &'a EdgeCaptures,
}

impl fmt::Display for EdgePrimitive<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (name, pin, er) = (self.name, self.name.pin, self.captures);
        let clocks = er.clocks();
        let cols = data_cols(er);
        // Ports: the pin, its data columns (self and clocks excluded), then the clocks last in clocks() order.
        let ports = Joined::new(
            std::iter::once(pin)
                .chain(cols.iter().copied())
                .chain(clocks.iter().copied()),
            ", ",
            std::convert::identity,
        );
        let inputs = Joined::new(
            cols.iter().copied().chain(clocks.iter().copied()),
            ", ",
            std::convert::identity,
        );
        let comment = ClockComment(&clocks);
        writeln!(f, "primitive {name}({ports}); // {comment}")?;
        writeln!(f, "output {pin};")?;
        writeln!(f, "input  {inputs};")?;
        writeln!(f, "reg    {pin};")?;
        writeln!(f, "table")?;
        for row in edge_table_rows(er) {
            writeln!(f, "\t{row}")?;
        }
        writeln!(f, "endtable")?;
        writeln!(f, "endprimitive")
    }
}

/// The trailing comment naming an edge UDP's clock column(s), which reduces to the single-clock wording
/// for a register keyed off one clock.
struct ClockComment<'a>(&'a [&'a Symbol]);

impl fmt::Display for ClockComment<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            [one] => write!(f, "clock {one} is the last port"),
            many => {
                let list = Joined::new(many.iter(), ", ", std::convert::identity);
                write!(f, "clocks {list} are the last ports")
            }
        }
    }
}

/// The edge-register UDP table rows. Column order is the data cols (`er.cols` minus the register's own
/// symbol and the clocks) then the clocks in [`EdgeCaptures::clocks`] order; the current-state (`reg`)
/// field is `?` except on a self-referencing register's capture rows, where it carries that register's
/// own literal. Each capture row carries exactly ONE edge indicator (IEEE 1364); the capturing clock's
/// column holds it while every other clock column carries the conditioning level. For a single clock
/// every rule reduces exactly to the single-clock rows.
fn edge_table_rows(er: &EdgeCaptures) -> Vec<EdgeRow> {
    let cols = data_cols(er);
    let clocks = er.clocks();
    let mut rows: Vec<EdgeRow> = Vec::new();

    // (a) Capture rows: the combinational next-state sampled on one active edge of one clock. Each
    // capture carries its own clock; a dual-edge (or multi-clock) register contributes one group per
    // `(clock, edge)`, each keeping its single edge indicator in the capturing clock's column.
    for capture in &er.captures {
        let regions = &capture.regions;
        for RegionAction { cubes, action } in [
            RegionAction {
                cubes: &regions.on,
                action: Next::On,
            },
            RegionAction {
                cubes: &regions.off,
                action: Next::Off,
            },
        ] {
            rows.extend(cubes.iter().map(|cube| {
                region_row(
                    er,
                    &cols,
                    &clocks,
                    Some(&capture.clock),
                    &regions.cols,
                    cube,
                    action,
                )
            }));
        }
    }

    // (b) Async set/clear as LEVEL rows (every clock `?`): by IEEE 1364 a level row dominates the edge
    // rows, and F1/F2 guarantee any overlap agrees, so the set/clear wins independent of the clocks.
    for RegionAction { cubes, action } in [
        RegionAction {
            cubes: &er.off_edge.on,
            action: Next::On,
        },
        RegionAction {
            cubes: &er.off_edge.off,
            action: Next::Off,
        },
    ] {
        rows.extend(
            cubes
                .iter()
                .map(|cube| region_row(er, &cols, &clocks, None, &er.off_edge.cols, cube, action)),
        );
    }

    // (c) Opposite-edge ignore: for each clock, each edge face with NO capture entry holds on a
    // transition of that edge — one row carrying that edge indicator in the clock's column and `?`
    // elsewhere. A single-edge clock emits its one inactive edge (as today); a dual-edge clock, both
    // faces captured, emits none.
    for &clock in &clocks {
        for edge in [Edge::Rise, Edge::Fall] {
            if er
                .captures
                .iter()
                .any(|c| &c.clock.pin == clock && c.clock.edge == edge)
            {
                continue;
            }
            let mut cells: Vec<EdgeColumn> = cols.iter().map(|_| EdgeColumn::any()).collect();
            cells.extend(clocks.iter().map(|&c| {
                if c == clock {
                    EdgeColumn::Edge(edge)
                } else {
                    EdgeColumn::any()
                }
            }));
            rows.push(EdgeRow::holding(cells));
        }
    }

    // (d) Steady-clock data-transition ignore: a change on any data column with every clock stable holds.
    for i in 0..cols.len() {
        let mut cells: Vec<EdgeColumn> = (0..cols.len())
            .map(|j| {
                if i == j {
                    EdgeColumn::Change
                } else {
                    EdgeColumn::any()
                }
            })
            .collect();
        cells.extend(clocks.iter().map(|_| EdgeColumn::any()));
        rows.push(EdgeRow::holding(cells));
    }

    // The tool's own order over the row values, carrying nothing to a UDP consumer, as in
    // `table_rows` above.
    rows.sort();
    rows
}

/// One region row of an edge-register table: the data columns (`cols`, self and clocks excluded) filled
/// from `cube` by column name against `region_cols`, then the clock columns (`clocks`, in order), the
/// current-state (`reg`) field and the `next` action. When `active` names a clock edge, that clock's
/// column carries the edge and every OTHER clock column carries its level
/// from `cube` (a conditioned capture references the other clock's level); when `active` is `None` (a
/// clock-independent level row) every clock column reads its `cube` level, which is `?` for an off-edge
/// region since it never references a clock. A data or clock column the cube does not constrain (or absent
/// from `region_cols`) reads `?`. The `reg` field is `?` unless the register is self-referencing (its own
/// symbol in `er.cols`), in which case it carries that node's literal from `cube` — the capture's
/// dependence on the register's own prior state.
fn region_row(
    er: &EdgeCaptures,
    cols: &[&Symbol],
    clocks: &[&Symbol],
    active: Option<&PinEdge>,
    region_cols: &[Symbol],
    cube: &StateCube,
    next: Next,
) -> EdgeRow {
    let mut cells: Vec<EdgeColumn> = cols
        .iter()
        .map(|&c| EdgeColumn::Level(level_at(c, region_cols, cube)))
        .collect();
    // Clock columns LAST, in clocks() order: the capturing clock carries its edge indicator, every other
    // clock its conditioning level from the cube.
    cells.extend(clocks.iter().map(|&clock| match active {
        Some(active) if clock == &active.pin => EdgeColumn::Edge(active.edge),
        _ => EdgeColumn::Level(level_at(clock, region_cols, cube)),
    }));
    let reg = if er.cols.contains(&er.node) {
        level_at(&er.node, region_cols, cube)
    } else {
        Level(None)
    };
    EdgeRow { cells, reg, next }
}

/// One row of an edge-sensitive UDP table: its columns in the primitive's port order (the data columns
/// then the clocks), the current-state (`reg`) field and the state the register takes.
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct EdgeRow {
    cells: Vec<EdgeColumn>,
    reg: Level,
    next: Next,
}

impl EdgeRow {
    /// A row that leaves the register where it was, its current state unread: the shape of both ignore
    /// rules — an inactive clock face and a data change under steady clocks.
    fn holding(cells: Vec<EdgeColumn>) -> EdgeRow {
        EdgeRow {
            cells,
            reg: Level(None),
            next: Next::Hold,
        }
    }
}

impl fmt::Display for EdgeRow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let cells = Joined::new(self.cells.iter(), " ", std::convert::identity);
        write!(f, "{cells} : {} : {};", self.reg, self.next)
    }
}

/// One column of an [`EdgeRow`]: a steady [`Level`], the clock edge the row fires on (`(01)` for a rise,
/// `(10)` for a fall), or a change to any value (`(??)`), which is what a steady-clock ignore row keys
/// its data column off.
#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum EdgeColumn {
    Level(Level),
    Edge(Edge),
    Change,
}

impl EdgeColumn {
    /// The any-value column `?`, which is what a column the row leaves unconstrained carries.
    fn any() -> EdgeColumn {
        EdgeColumn::Level(Level(None))
    }
}

impl fmt::Display for EdgeColumn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EdgeColumn::Level(level) => write!(f, "{level}"),
            EdgeColumn::Edge(Edge::Rise) => f.write_str("(01)"),
            EdgeColumn::Edge(Edge::Fall) => f.write_str("(10)"),
            EdgeColumn::Change => f.write_str("(??)"),
        }
    }
}

/// The level of column `col` in `cube`, looked up by name against `region_cols` (a subset of the
/// register's columns). Absent — the region does not constrain this column — is the any-value `?`.
fn level_at(col: &Symbol, region_cols: &[Symbol], cube: &StateCube) -> Level {
    Level(
        region_cols
            .iter()
            .position(|c| c == col)
            .and_then(|i| cube[i]),
    )
}

/// The `celldefine`d wrapper module: the cell's ports, an internal `wire` per surviving state node, a
/// `specify` path delay from every input to every output, an instance of each signal's UDP and a
/// continuous assignment for each read-gated output.
pub struct Wrapper<'a> {
    /// The declared name this wrapper carries. A cell with several names emits one wrapper per name,
    /// all instantiating the same shared primitives.
    name: &'a Symbol,
    outputs: Vec<&'a Symbol>,
    inputs: &'a [Symbol],
    /// The internal wires: the surviving internal state nodes then the minted factored registers.
    internals: Vec<&'a Symbol>,
    instances: Vec<Instance<'a>>,
    assigns: Vec<Assign<'a>>,
}

/// Build one declared name's wrapper. Ports are the external face only — outputs and primary inputs — so
/// an internal state node is a wire driven by its own instance, and a folded master vanishes: it is
/// neither a wire nor an instance.
fn wrapper<'a>(
    cell: &'a AnalysedCell,
    name: &'a Symbol,
    edge_by_node: &BTreeMap<&str, &'a EdgeCaptures>,
    folded: &BTreeSet<&str>,
) -> Wrapper<'a> {
    let outputs: Vec<&Symbol> = cell.outputs.iter().map(|o| &o.name).collect();
    // Read-gated outputs (continuous assigns) and their minted factored registers (internal wires driven
    // by an edge UDP).
    let read_of: BTreeMap<&str, &StateRegions> = read_functions(cell);
    let signal_names: BTreeSet<&str> = cell
        .signal_regions()
        .map(|(s, _)| s.name.as_str())
        .collect();
    let derived_minted: Vec<&Symbol> = cell
        .edge
        .derived
        .iter()
        .map(|d| &d.name)
        .filter(|n| !signal_names.contains(n.as_str()))
        .collect();
    // A minted factored register is an internal wire like a surviving internal state node.
    let internals: Vec<&Symbol> = cell
        .internals
        .iter()
        .filter(|o| !folded.contains(o.name.as_str()))
        .map(|o| &o.name)
        .chain(derived_minted.iter().copied())
        .collect();

    // Instantiate every surviving signal's UDP (outputs and internals); an internal drives its own
    // wire. A folded master has no instance; a read-gated output is a continuous assign, added below.
    let mut instances: Vec<Instance> = Vec::new();
    for (sig, sr) in cell.signal_regions() {
        if folded.contains(sig.name.as_str()) || cell.edge.factored.contains(&sig.name) {
            continue;
        }
        // Edge registers connect in port order `(pin, cols…, clocks…)` with the clocks last in
        // clocks() order; constant pins take just their own port; other sequential pins add their columns.
        let args: Vec<&Symbol> = if let Some(er) = edge_by_node.get(sig.name.as_str()) {
            std::iter::once(&sig.name)
                .chain(data_cols(er))
                .chain(er.clocks())
                .collect()
        } else if sr.hold.is_empty() && (sr.on.is_empty() || sr.off.is_empty()) {
            vec![&sig.name]
        } else {
            std::iter::once(&sig.name).chain(sr.cols.iter()).collect()
        };
        instances.push(Instance {
            name: PrimName::new(cell, &sig.name),
            args,
        });
    }
    // The minted factored registers: an edge UDP instance driving the register's own wire, in port order
    // `(pin, data cols…, clocks…)` — the same layout as any edge register.
    for d in &derived_minted {
        let Some(er) = edge_by_node.get(d.as_str()) else {
            continue;
        };
        instances.push(Instance {
            name: PrimName::new(cell, d),
            args: std::iter::once(*d)
                .chain(data_cols(er))
                .chain(er.clocks())
                .collect(),
        });
    }
    // The read-gated outputs: a continuous assign of the read function over the factored register and gate
    // pins.
    let assigns: Vec<Assign> = cell
        .signal_regions()
        .filter_map(|(sig, _)| {
            read_of.get(sig.name.as_str()).map(|&function| Assign {
                pin: &sig.name,
                function,
            })
        })
        .collect();

    Wrapper {
        name,
        outputs,
        inputs: &cell.inputs,
        internals,
        instances,
        assigns,
    }
}

impl fmt::Display for Wrapper<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = self.name;
        let ports = Joined::new(
            self.outputs.iter().copied().chain(self.inputs.iter()),
            ", ",
            std::convert::identity,
        );
        let outputs = Joined::new(self.outputs.iter(), ", ", std::convert::identity);
        writeln!(f, "`celldefine")?;
        writeln!(f, "module {name}({ports});")?;
        writeln!(f, "output {outputs};")?;
        if !self.inputs.is_empty() {
            let inputs = Joined::new(self.inputs.iter(), ", ", std::convert::identity);
            writeln!(f, "input  {inputs};")?;
        }
        if !self.internals.is_empty() {
            let internals = Joined::new(self.internals.iter(), ", ", std::convert::identity);
            writeln!(f, "wire   {internals};")?;
        }

        writeln!(f, "specify")?;
        for input in self.inputs {
            for output in &self.outputs {
                writeln!(f, "\t({input} => {output}) = {PATH_DELAY};")?;
            }
        }
        writeln!(f, "endspecify")?;

        for instance in &self.instances {
            writeln!(f, "{instance}")?;
        }
        for assign in &self.assigns {
            writeln!(f, "{assign}")?;
        }
        writeln!(f, "endmodule")?;
        writeln!(f, "`endcelldefine")
    }
}

/// One UDP instance inside a wrapper: the primitive it instantiates and the signals its ports connect
/// to, in that primitive's own port order.
struct Instance<'a> {
    name: PrimName<'a>,
    args: Vec<&'a Symbol>,
}

impl fmt::Display for Instance<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = self.name;
        let args = Joined::new(self.args.iter(), ", ", std::convert::identity);
        write!(f, "{name} u_{name} ({args});")
    }
}

/// One read-gated output's continuous assignment: the output pin and the read function it takes over the
/// factored register and its gate pins.
struct Assign<'a> {
    pin: &'a Symbol,
    function: &'a StateRegions,
}

impl fmt::Display for Assign<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "assign {} = {};", self.pin, Sop(self.function))
    }
}

/// A read function's on-region as a Verilog sum-of-products expression over its columns: literals joined
/// by `&`, product terms by `|`, negation `~`. An empty on-set is `1'b0`; an on-set holding a cube that
/// constrains no column is a tautology, so the whole expression is `1'b1`.
struct Sop<'a>(&'a StateRegions);

impl fmt::Display for Sop<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sr = self.0;
        if sr.on.is_empty() {
            return f.write_str("1'b0");
        }
        if sr.on.iter().any(|cube| literals(sr, cube).next().is_none()) {
            return f.write_str("1'b1");
        }
        Joined::new(sr.on.iter(), " | ", |cube| Product { regions: sr, cube }).fmt(f)
    }
}

/// One product term of a [`Sop`]: the cube's literals, `&`-joined inside parentheses.
struct Product<'a> {
    regions: &'a StateRegions,
    cube: &'a StateCube,
}

impl fmt::Display for Product<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let literals = Joined::new(
            literals(self.regions, self.cube),
            " & ",
            std::convert::identity,
        );
        write!(f, "({literals})")
    }
}

/// The literals `cube` states over `sr`'s columns, in column order. A column the cube leaves
/// unconstrained states none.
fn literals<'a>(
    sr: &'a StateRegions,
    cube: &'a StateCube,
) -> impl Iterator<Item = Literal<'a>> + Clone {
    sr.cols
        .iter()
        .zip(cube.iter())
        .filter_map(|(col, &value)| value.map(|level| Literal { col, level }))
}

/// One literal of a Verilog expression: the column, negated (`~`) where the cube holds it low.
struct Literal<'a> {
    col: &'a Symbol,
    level: bool,
}

impl fmt::Display for Literal<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.level {
            f.write_str("~")?;
        }
        write!(f, "{}", self.col)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{analyse_both, analyse_one as analyse, AnalysedPair};

    /// A cell's model as the text the sink writes: its declarations, each written in turn.
    fn emit(cell: &AnalysedCell) -> String {
        Verilog(&cell_verilog(cell)).to_string()
    }

    #[test]
    fn c_element_emits_sequential_udp() {
        let cell = analyse(
            r#"
[[cell]]
name = "C2"
inputs = ["A", "B"]
[cell.outputs]
Q = "A*B + Q*(A+B)"
"#,
        );
        let v = emit(&cell);
        eprintln!("{v}");
        assert!(v.contains("primitive C2_Q(Q, A, B);"));
        assert!(v.contains("reg    Q;"));
        // Hysteresis appears as no-change rows, on/off as 1/0.
        assert!(v.contains(": ? : -;"));
        assert!(v.contains("1 1 : ? : 1;"));
        assert!(v.contains("0 0 : ? : 0;"));
        // Wrapper module + specify + instantiation.
        assert!(v.contains("`celldefine"));
        assert!(v.contains("module C2(Q, A, B);"));
        assert!(v.contains("(A => Q) = (0.1, 0.1);"));
        assert!(v.contains("C2_Q u_C2_Q (Q, A, B);"));
        assert!(v.contains("`endcelldefine"));
    }

    #[test]
    fn cross_coupled_keeps_other_output_as_udp_input() {
        let cell = analyse(
            r#"
[[cell]]
name = "SR"
inputs = ["S", "R"]
[cell.outputs]
Q = "S + Q*!R"
Qn = "R + Qn*!S"
"#,
        );
        let v = emit(&cell);
        // Two primitives, one wrapper declaring both outputs.
        assert!(v.contains("primitive SR_Q("));
        assert!(v.contains("primitive SR_Qn("));
        assert!(v.contains("module SR(Q, Qn, S, R);"));
    }

    #[test]
    fn dff_internal_master_is_a_wire_not_a_port() {
        // Opt-out fixture: the declared clock would collapse the master-slave pair, but
        // `no_edge_collapse` keeps the two-latch form — preserving the level-latch coverage.
        let cell = analyse(
            r#"
[[cell]]
name = "DFF"
inputs = ["CLK", "D"]
clock = ["CLK"]
no_edge_collapse = true
[cell.internal]
M = "!CLK*D + CLK*M"
[cell.outputs]
Q = "CLK*M + !CLK*Q"
"#,
        );
        let v = emit(&cell);
        eprintln!("{v}");
        // A UDP for the internal master and for the slave; the slave takes M as an input column. Q's
        // function (CLK*M + !CLK*Q) does not depend on D, so D is not one of DFF_Q's columns.
        assert!(v.contains("primitive DFF_M(M, CLK, D);"));
        assert!(v.contains("primitive DFF_Q(Q, CLK, M);"));
        // Module ports are the external face only; M is an internal wire, both UDPs instantiated.
        assert!(v.contains("module DFF(Q, CLK, D);"));
        assert!(v.contains("wire   M;"));
        assert!(v.contains("DFF_M u_DFF_M (M, CLK, D);"));
        assert!(v.contains("DFF_Q u_DFF_Q (Q, CLK, M);"));
        // M is never declared as a module output.
        assert!(!v.contains("output Q, M"));
        assert!(!v.contains("module DFF(Q, M,"));
    }

    #[test]
    fn dff_collapses_to_edge_register_udp() {
        // Default collapse: the same DFF with a declared clock becomes a single rising-edge register Q
        // that folds the master M away.
        let cell = analyse(
            r#"
[[cell]]
name = "DFF"
inputs = ["CLK", "D"]
clock = ["CLK"]
[cell.internal]
M = "!CLK*D + CLK*M"
[cell.outputs]
Q = "CLK*M + !CLK*Q"
"#,
        );
        let v = emit(&cell);
        eprintln!("{v}");
        // One edge-sensitive UDP with the clock last; captures on the rising edge.
        assert!(v.contains("primitive DFF_Q(Q, D, CLK);"));
        assert!(v.contains("reg    Q;"));
        assert!(v.contains("1 (01) : ? : 1;"));
        assert!(v.contains("0 (01) : ? : 0;"));
        // The folded master leaves no trace: no primitive, no wire, no instance.
        assert!(!v.contains("DFF_M"));
        assert!(!v.contains("wire   M;"));
        // Instance connects in port order (pin, cols…, clock).
        assert!(v.contains("DFF_Q u_DFF_Q (Q, D, CLK);"));
        assert!(v.contains("module DFF(Q, CLK, D);"));
    }

    #[test]
    fn bdet_read_gate_factorisation_verilog() {
        // BDET: the factored register `Y_st` emits an edge UDP; the read-gated output `Y` a continuous
        // assign. The DET masters `L1/L2` fold entirely.
        let cell = analyse(
            r#"
[[cell]]
name = "BDET"
inputs = ["CLK", "D", "A"]
clock = ["CLK"]
[cell.internal]
L1 = "!CLK*D + CLK*L1"
L2 = "CLK*D + !CLK*L2"
[cell.outputs]
Y = "!((CLK*L1 + !CLK*L2)*A)"
"#,
        );
        let v = emit(&cell);
        eprintln!("{v}");
        // The factored register is a dual-edge UDP capturing !D (D=0 -> 1, D=1 -> 0 on both edges).
        assert!(v.contains("primitive BDET_Y_st(Y_st, D, CLK);"));
        assert!(v.contains("0 (01) : ? : 1;") && v.contains("1 (01) : ? : 0;"));
        assert!(v.contains("0 (10) : ? : 1;") && v.contains("1 (10) : ? : 0;"));
        // The read-gated output is a continuous assign over Y_st and A — never a UDP of its own.
        assert!(v.contains("assign Y = "));
        assert!(
            !v.contains("primitive BDET_Y("),
            "Y is an assign, not a primitive"
        );
        // Y_st is an internal wire, instantiated; Y is the module output. Folded masters leave no trace.
        assert!(v.contains("wire   Y_st;"));
        assert!(v.contains("BDET_Y_st u_BDET_Y_st (Y_st, D, CLK);"));
        assert!(v.contains("module BDET(Y, CLK, D, A);"));
        assert!(!v.contains("BDET_L1") && !v.contains("BDET_L2"));
    }

    #[test]
    fn ndff_group_folds_the_mutually_referencing_nand_master_pair() {
        // The cross-coupled-NAND master-slave flop: M/Mn are captureless and mutually referencing, so
        // they fold together exactly as the pass DFF's lone M folds. Q and Qn survive as the two edge
        // registers (Qn carries its own genuine !D capture).
        let cell = analyse(
            r#"
[[cell]]
name = "NDFF"
inputs = ["CLK", "D"]
clock = ["CLK"]
[cell.internal]
Mn = "!( !(!D*!CLK) * M )"
M = "!( !(D*!CLK) * Mn )"
[cell.outputs]
Qn = "!( !(!M*CLK) * Q )"
Q = "!( !(M*CLK) * Qn )"
"#,
        );
        let v = emit(&cell);
        eprintln!("{v}");
        // The folded master pair leaves no trace: no primitive, no wire, no instance.
        assert!(!v.contains("NDFF_M"));
        assert!(!v.contains("wire   M;"));
        assert!(!v.contains("wire   Mn;"));
    }

    #[test]
    fn icm_collapses_masters_into_edge_registers() {
        // The ICM interlock: two three-latch synchronisers. Each chain's head latch (sela1/selb1) is a
        // foldable pure master; sela2/enA (and the CLKB mirror) survive as edge registers.
        let cell = analyse(
            r#"
[[cell]]
name = "ICM"
inputs = ["CLKA", "CLKB", "RA", "RB", "S"]
clock = ["CLKA", "CLKB"]
[cell.internal]
sela = "!enB*!S"
selb = "!enA*S"
sela1 = "!RA*(!CLKA*sela+CLKA*sela1)"
sela2 = "!RA*(CLKA*sela1+!CLKA*sela2)"
enA   = "!RA*(!CLKA*sela2+CLKA*enA)"
selb1 = "!RB*(!CLKB*selb+CLKB*selb1)"
selb2 = "!RB*(CLKB*selb1+!CLKB*selb2)"
enB   = "!RB*(!CLKB*selb2+CLKB*enB)"
[cell.outputs]
GCLK = "enA*CLKA+enB*CLKB"
"#,
        );
        let v = emit(&cell);
        eprintln!("{v}");

        // Folded masters vanish entirely — no primitive, no wire, no instance.
        assert!(!v.contains("ICM_sela1"));
        assert!(!v.contains("ICM_selb1"));
        assert!(!v.contains("wire   sela1"));
        assert!(!v.contains("wire   selb1"));

        // sela2 survives as a rising-edge register (folding sela1); enA as a falling-edge one.
        assert!(prim_block(&v, "primitive ICM_sela2(").contains("(01)"));
        assert!(prim_block(&v, "primitive ICM_enA(").contains("(10)"));
        // The async reset RA emits a clock-independent LEVEL clear row (next 0) in enA's table; RA is
        // enA's second data column (cols `sela2, RA`), so the clear pattern is `? 1 ?`.
        assert!(prim_block(&v, "primitive ICM_enA(").contains("? 1 ? : ? : 0;"));

        // The surviving registers instantiate in port order (pin, cols…, clock).
        assert!(v.contains("ICM_sela2 u_ICM_sela2 (sela2, RA, S, enB, CLKA);"));
        assert!(v.contains("ICM_enA u_ICM_enA (enA, sela2, RA, CLKA);"));
    }

    /// The table body of one named `primitive` (from its header up to `endprimitive`), for asserting
    /// per-primitive row content without matching the same token in a sibling UDP.
    fn prim_block<'a>(v: &'a str, head: &str) -> &'a str {
        let start = v.find(head).expect("primitive present");
        let rest = &v[start..];
        let end = rest.find("endprimitive").expect("endprimitive terminator");
        &rest[..end]
    }

    /// Every UDP table row a model states, each under the `primitive` header it belongs to and the whole
    /// sorted -- the rows a run emits, blind to the order they came out in. This is what
    /// `crate::emit::arcs_tcl`'s `shaped_blocks` is over the Tcl deck: a UDP consumer matches a row by
    /// its pattern rather than by its position, so what two runs of one cell have to agree on is the SET
    /// of rows under each primitive.
    fn table_row_set(v: &str) -> Vec<String> {
        let mut rows: Vec<String> = Vec::new();
        let mut head = "";
        let mut in_table = false;
        for line in v.lines() {
            match line {
                _ if line.starts_with("primitive ") => head = line,
                "table" => in_table = true,
                "endtable" => in_table = false,
                _ if in_table => rows.push(format!("{head}\t{}", line.trim())),
                _ => {}
            }
        }
        rows.sort();
        rows
    }

    #[test]
    fn dcmux_udp_is_a_level_reg() {
        // DCMUX collapses to a LEVEL model (its falls are combinational and the active-edge filter
        // empties Q's set), so Q emits a level `reg` UDP -- it holds while both clocks are low and
        // passes the muxed masters otherwise, with NO edge rows. Both clocks stay UDP ports; the two
        // rise DELAY arcs render `-type edge` (covered in the arcs_tcl emitter tests).
        let cell = analyse(
            r#"
[[cell]]
name = "DCMUX"
inputs = ["CLKA", "CLKB", "DA", "DB"]
clock = ["CLKA", "CLKB"]
[cell.internal]
MA = "!CLKA*DA + CLKA*MA"
MB = "!CLKB*DB + CLKB*MB"
[cell.outputs]
Q = "CLKA*MA + CLKB*MB + !CLKA*!CLKB*Q"
"#,
        );
        let v = emit(&cell);
        eprintln!("{v}");
        assert!(v.contains("primitive DCMUX_Q("), "Q UDP present");
        let q = prim_block(&v, "primitive DCMUX_Q(");
        assert!(q.contains("reg    Q;"), "Q is a level reg");
        // A level model carries no edge rows.
        assert!(
            !q.contains("(01)") && !q.contains("(10)"),
            "a level model carries no edge rows:\n{q}"
        );
        // Both keying clocks remain ports of the UDP.
        let header = q.lines().next().expect("a primitive header");
        assert!(
            header.contains("CLKA") && header.contains("CLKB"),
            "both clocks are UDP ports: {header}"
        );
    }

    #[test]
    fn hierarchical_slave_udp_captures_on_both_clocks() {
        // Hierarchical master-slave across two clocks (HPIPE): the slave Q's UDP captures from CLKA on its
        // rising edge AND from CLKB on its falling edge -- both keying clocks trail, no arc dropped.
        let cell = analyse(
            r#"
[[cell]]
name = "HPIPE"
inputs = ["CLKA", "CLKB", "D"]
clock = ["CLKA", "CLKB"]
[cell.internal]
M1 = "!CLKA*D + CLKA*M1"
M2 = "CLKA*M1 + !CLKA*M2"
[cell.outputs]
Q = "!CLKB*M2 + CLKB*Q"
"#,
        );
        let v = emit(&cell);
        eprintln!("{v}");
        let q = prim_block(&v, "primitive HPIPE_Q(");
        let clka_i = q_port_index(&v, "HPIPE_Q", "CLKA");
        let clkb_i = q_port_index(&v, "HPIPE_Q", "CLKB");
        let field = |row: &str, i: usize| -> String {
            row.split(':')
                .next()
                .unwrap()
                .split_whitespace()
                .nth(i)
                .unwrap_or("")
                .to_string()
        };
        let mut saw_clka_rise = false;
        let mut saw_clkb_fall = false;
        for row in q
            .lines()
            .filter(|l| l.contains("(01)") || l.contains("(10)"))
        {
            // Each edge row keys exactly ONE clock; the other keying clock sits as a level condition.
            if field(row, clka_i) == "(01)"
                && field(row, clkb_i) != "(01)"
                && field(row, clkb_i) != "(10)"
            {
                saw_clka_rise = true;
            }
            if field(row, clkb_i) == "(10)"
                && field(row, clka_i) != "(01)"
                && field(row, clka_i) != "(10)"
            {
                saw_clkb_fall = true;
            }
        }
        assert!(saw_clka_rise, "Q captures on CLKA rising");
        assert!(
            saw_clkb_fall,
            "Q captures on CLKB falling (its own latch opening) -- both keying clocks trail, no arc dropped"
        );
    }

    /// The zero-based position of `port` among the UDP `head`'s ports AFTER the output pin (i.e. the data
    /// and clock columns, aligned to the `table` row cells before the first `:`).
    fn q_port_index(v: &str, head: &str, port: &str) -> usize {
        let decl = v
            .lines()
            .find(|l| l.contains(&format!("primitive {head}(")))
            .expect("primitive decl");
        let ports: Vec<&str> = decl
            .split('(')
            .nth(1)
            .unwrap()
            .split(')')
            .next()
            .unwrap()
            .split(',')
            .map(str::trim)
            .collect();
        // ports[0] is the pin; row cells align to ports[1..], so return the index within that tail.
        ports[1..]
            .iter()
            .position(|p| *p == port)
            .unwrap_or_else(|| panic!("{port} is a UDP port of {head}"))
    }

    #[test]
    fn multiple_names_share_primitives_with_one_wrapper_each() {
        let cell = analyse(
            r#"
[[cell]]
name = ["INVX1", "INVX2"]
inputs = ["A"]
[cell.outputs]
Y = "!A"
"#,
        );
        let v = emit(&cell);
        eprintln!("{v}");
        // The primitive keys off the representative name and is emitted exactly once.
        assert_eq!(v.matches("primitive INVX1_Y(").count(), 1);
        assert!(!v.contains("primitive INVX2_Y("));
        // One wrapper module per name, both instantiating the same shared primitive.
        assert!(v.contains("module INVX1(Y, A);"));
        assert!(v.contains("module INVX2(Y, A);"));
        assert_eq!(v.matches("INVX1_Y u_INVX1_Y (Y, A);").count(), 2);
    }

    #[test]
    fn combinational_gate_has_no_hold_rows() {
        let cell = analyse(
            r#"
[[cell]]
name = "ND2"
inputs = ["A", "B"]
[cell.outputs]
Y = "!(A*B)"
"#,
        );
        let v = emit(&cell);
        assert!(v.contains("primitive ND2_Y(Y, A, B);"));
        assert!(!v.contains(": ? : -;")); // no hysteresis
    }

    /// Four shapes the behavioural classifier recognises as NO edge register even under default (on)
    /// collapse: a single latch, a gated (self-referencing) latch, a master/slave pair split across two
    /// DIFFERENT declared clocks (the slave stays level — its data is transparent in one phase of the
    /// clock that gates it), and a two-latch DFF whose clock is never declared. The exposed-master DFF
    /// — a master surfaced as a second output — collapses behaviourally and is covered as a positive
    /// fixture in `exposed_master_collapses_slave_over_surviving_master`.
    const NON_COLLAPSIBLE: [&str; 4] = [
        r#"
[[cell]]
name = "DLAT"
inputs = ["CLK", "D"]
clock = ["CLK"]
[cell.outputs]
Q = "CLK*D + !CLK*Q"
"#,
        r#"
[[cell]]
name = "GLAT"
inputs = ["CLK", "D"]
clock = ["CLK"]
[cell.outputs]
Q = "CLK*(D+Q) + !CLK*Q"
"#,
        r#"
[[cell]]
name = "MCDFF"
inputs = ["CLKA", "CLKB", "D"]
clock = ["CLKA", "CLKB"]
[cell.internal]
M = "!CLKA*D + CLKA*M"
[cell.outputs]
Q = "CLKB*M + !CLKB*Q"
"#,
        r#"
[[cell]]
name = "UCDFF"
inputs = ["CLK", "D"]
[cell.internal]
M = "!CLK*D + CLK*M"
[cell.outputs]
Q = "CLK*M + !CLK*Q"
"#,
    ];

    #[test]
    fn non_collapsible_suite_verilog_matches_the_no_edge_collapse_flag() {
        // No clock-edge indicator (`(01)`/`(10)`) appears, whether the flag is left off (default
        // collapse, a no-op on these shapes) or forced on -- and the two runs state the same UDP table
        // rows.
        for src in NON_COLLAPSIBLE {
            let AnalysedPair { default, forced } = analyse_both(src);
            let v_default = emit(&default);
            let v_forced = emit(&forced);
            for v in [&v_default, &v_forced] {
                assert!(!v.contains("(01)"), "unexpected rising-edge token");
                assert!(!v.contains("(10)"), "unexpected falling-edge token");
            }
            assert_eq!(table_row_set(&v_default), table_row_set(&v_forced));
        }
    }

    #[test]
    fn dff_opt_out_restores_master_primitive_via_either_switch() {
        // The two-latch DFF, opted out directly (`no_edge_collapse = true` in the TOML) versus opted
        // out via the CLI-flag-equivalent blanket mutation over the whole spec: both switches restore
        // the SAME two-latch model -- a `DFF_M` primitive and wire, absent under default collapse, over
        // the same UDP table rows.
        const DFF: &str = r#"
[[cell]]
name = "DFF"
inputs = ["CLK", "D"]
clock = ["CLK"]
[cell.internal]
M = "!CLK*D + CLK*M"
[cell.outputs]
Q = "CLK*M + !CLK*Q"
"#;
        let direct = {
            let mut spec = crate::model::parse_spec(DFF).unwrap();
            spec.cells[0].no_edge_collapse = true;
            spec.cells.remove(0).analyse().unwrap()
        };
        let via_flag = {
            // Mirrors main.rs:82-88's blanket application of `--no-edge-collapse` over every cell.
            let mut spec = crate::model::parse_spec(DFF).unwrap();
            for c in &mut spec.cells {
                c.no_edge_collapse = true;
            }
            spec.cells.remove(0).analyse().unwrap()
        };

        let v_direct = emit(&direct);
        let v_via_flag = emit(&via_flag);
        for v in [&v_direct, &v_via_flag] {
            assert!(v.contains("primitive DFF_M("));
            assert!(v.contains("wire   M;"));
        }
        assert_eq!(table_row_set(&v_direct), table_row_set(&v_via_flag));
    }

    #[test]
    fn exposed_master_collapses_slave_over_surviving_master() {
        // The exposed-master DFF: the master M is a second OUTPUT, so it survives (never folded) as its
        // own level UDP, while the slave Q collapses to a rising-edge register capturing M.
        let cell = analyse(
            r#"
[[cell]]
name = "EMDFF"
inputs = ["CLK", "D"]
clock = ["CLK"]
[cell.outputs]
Q = "CLK*M + !CLK*Q"
M = "!CLK*D + CLK*M"
"#,
        );
        let v = emit(&cell);
        eprintln!("{v}");
        // Q is a rising-edge register; its capture cover PREFERS the input D over the internal M (D and M
        // coincide over the CLK=0 capture domain), so Q's UDP keys off D. The master M keeps its own level
        // UDP and survives as an output.
        assert!(v.contains("primitive EMDFF_Q(Q, D, CLK);"));
        assert!(prim_block(&v, "primitive EMDFF_Q(").contains("0 (01) : ? : 0;"));
        assert!(prim_block(&v, "primitive EMDFF_Q(").contains("1 (01) : ? : 1;"));
        assert!(v.contains("primitive EMDFF_M(M, CLK, D);"));
        // M is an output, so it is a module port, not folded away.
        assert!(v.contains("module EMDFF(M, Q, CLK, D);"));
        assert!(!v.contains("wire   M;"));
    }

    #[test]
    fn dual_edge_det_captures_on_both_edges_with_no_opposite_row() {
        // A mux-based dual-edge flip-flop: Q captures D on BOTH clock edges. Each capture row carries
        // exactly ONE edge token, and there is no opposite-edge no-change row (a dual-edge register has
        // no inactive edge).
        let cell = analyse(
            r#"
[[cell]]
name = "DET"
inputs = ["CLK", "D"]
clock = ["CLK"]
[cell.internal]
L1 = "!CLK*D + CLK*L1"
L2 = "CLK*D + !CLK*L2"
[cell.outputs]
Q = "CLK*L1 + !CLK*L2"
"#,
        );
        let v = emit(&cell);
        eprintln!("{v}");
        let q = prim_block(&v, "primitive DET_Q(");
        // Both edges capture D; each row carries exactly one edge indicator.
        assert!(q.contains("0 (01) : ? : 0;"));
        assert!(q.contains("1 (01) : ? : 1;"));
        assert!(q.contains("0 (10) : ? : 0;"));
        assert!(q.contains("1 (10) : ? : 1;"));
        for row in q.lines().filter(|l| l.contains("(0") || l.contains("(1")) {
            let edges = row.matches("(01)").count() + row.matches("(10)").count();
            assert!(edges <= 1, "row carries more than one edge token: {row}");
        }
        // No opposite-edge no-change row: the only `-` rows are the steady-clock data-ignore rows.
        assert!(!q.contains("? (10) : ? : -;"));
        assert!(!q.contains("? (01) : ? : -;"));
        // Both internal latches fold away.
        assert!(!v.contains("DET_L1"));
        assert!(!v.contains("DET_L2"));
    }

    #[test]
    fn inverting_dff_captures_not_d() {
        // An inverting DFF: Q captures !D on the rising edge, recorded verbatim (inversion is not
        // special-cased) -- the capture rows map D=0 to next 1 and D=1 to next 0.
        let cell = analyse(
            r#"
[[cell]]
name = "IDFF"
inputs = ["CLK", "D"]
clock = ["CLK"]
[cell.internal]
M = "!CLK*D + CLK*M"
[cell.outputs]
Q = "CLK*!M + !CLK*Q"
"#,
        );
        let v = emit(&cell);
        eprintln!("{v}");
        let q = prim_block(&v, "primitive IDFF_Q(");
        assert!(v.contains("primitive IDFF_Q(Q, D, CLK);"));
        assert!(q.contains("0 (01) : ? : 1;"));
        assert!(q.contains("1 (01) : ? : 0;"));
        // Single-edge register keeps the opposite-edge no-change row and folds its master.
        assert!(q.contains("? (10) : ? : -;"));
        assert!(!v.contains("IDFF_M"));
    }

    #[test]
    fn toggle_flop_self_column_is_reg_field_not_input_port() {
        // A resettable toggle flip-flop decomposes into TWO edge registers over the ring cols [R, Q]: Q
        // captures the toggle `!R*!Q` on the rising edge, and M captures the same toggle on the falling
        // edge (keying off the surviving output Q, since the drop-loop prefers Q over the internal M). Q is
        // SELF-referencing: its own symbol must NOT become a UDP input port -- it is the `reg`
        // current-state field, carrying Q's own literal in the capture rows.
        let cell = analyse(
            r#"
[[cell]]
name = "TFF"
inputs = ["CLK", "R"]
clock = ["CLK"]
async = ["R"]
[cell.internal]
M = "!R*(!CLK*!Q + CLK*M)"
[cell.outputs]
Q = "!R*(CLK*M + !CLK*Q)"
"#,
        );
        let v = emit(&cell);
        eprintln!("{v}");
        // Q is the self-referencing rising-edge register: its own symbol is the reg field, not an input.
        assert!(v.contains("primitive TFF_Q(Q, R, CLK);"));
        let q = prim_block(&v, "primitive TFF_Q(");
        assert!(q.contains("input  R, CLK;"), "self Q is not an input port");
        // The rising capture prints Q's own literal in the current-state (reg) field, not `?`.
        assert!(q.contains("0 (01) : 0 : 1;"));
        assert!(q.contains("? (01) : 1 : 0;"));
        // M captures the same toggle on the falling edge, keying off the surviving Q (an input to M's UDP).
        assert!(v.contains("primitive TFF_M(M, R, Q, CLK);"));
        // The self-fed master survives as an internal wire, and neither instance duplicates M.
        assert!(v.contains("wire   M;"));
        assert!(v.contains("TFF_Q u_TFF_Q (Q, R, CLK);"));
        assert!(v.contains("TFF_M u_TFF_M (M, R, Q, CLK);"));
    }
}
