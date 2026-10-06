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
//! steady-clock data transitions. A pure master folded into such a register contributes nothing — no
//! primitive, no wire, no instance.
//!
//! A cell's declarations travel as the values [`cell_verilog`] states — one [`Item`] apiece — and become
//! text once, in [`Display`](fmt::Display), written into the writer the model is going out on.

use std::collections::{HashMap, HashSet};
use std::fmt;

use espresso_logic::{Anonymous, BoolExpr, Cover, Minterm, Symbol};

use crate::emit::RegionAction;
use crate::logic::arcs::{Edge, PinEdge};
use crate::logic::edge::EdgeCaptures;
use crate::logic::regions::StateRegions;
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
    let edge_by_node: HashMap<&str, &EdgeCaptures> = cell
        .edge
        .captures
        .iter()
        .map(|er| (er.node.as_str(), er))
        .collect();
    let folded: HashSet<&str> = cell.edge.folded.iter().map(Symbol::as_str).collect();
    // Read-gated outputs read a factored register combinationally: they emit a continuous `assign` in the
    // wrapper, no UDP of their own. Their factored register (minted, not a declared signal) emits an
    // edge-sensitive UDP like any register.
    let signal_names: HashSet<&str> = cell
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
fn read_functions(cell: &AnalysedCell) -> HashMap<&str, &StateRegions> {
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
    // Constant pin: no hold and one region empty ⇒ a tautology / contradiction. A region states nothing
    // exactly where its cover holds no cube.
    if sr.hold.num_cubes() == 0 && sr.off.num_cubes() == 0 {
        return Item::Constant(Constant { name, value: true });
    }
    if sr.hold.num_cubes() == 0 && sr.on.num_cubes() == 0 {
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
        rows.extend(cubes.cubes().map(|cube| TableRow {
            row: cube.inputs(),
            next: action,
            cols: &sr.cols,
        }));
    }
    // IEEE 1364 matches a UDP row by its pattern and resolves an overlap by rule, never by a row's
    // position, so a consumer reads the table as a set of rows and the order they come out in is free.
    rows
}

/// One row of a level-sensitive UDP table: the region cube it matches, as the row of tri-state values
/// that cube states over the signal's columns, and the state the pin takes there. The current-state
/// (`reg`) field is `?` — a level row matches on the input columns alone, and a hold row is what carries
/// the pin's prior state forward.
#[derive(PartialEq, Eq)]
struct TableRow<'a> {
    /// The cube's input pattern: the value it fixes at each column it constrains, every other column
    /// being don't-care by the row not naming it.
    row: &'a Minterm<Symbol>,
    next: Next,
    /// The signal's columns, which the row is projected onto as the line is written — the UDP being a
    /// columnar format, its columns are what the row has to be laid out over.
    cols: &'a [Symbol],
}

impl fmt::Display for TableRow<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let pattern = Pattern {
            row: self.row,
            cols: self.cols,
        };
        write!(f, "{pattern} : ? : {};", self.next)
    }
}

/// Where a UDP table row leaves the pin: driven high (`1`), driven low (`0`) or unchanged (`-`).
#[derive(Clone, Copy, PartialEq, Eq)]
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

/// A cube as space-separated UDP table columns: the row's value at each column, in header order. A
/// column the row does not name is absent from it and so reads as don't-care.
struct Pattern<'a> {
    row: &'a Minterm<Symbol>,
    cols: &'a [Symbol],
}

impl fmt::Display for Pattern<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Joined::new(self.cols.iter(), " ", |col| Level(self.row.value_of(col))).fmt(f)
    }
}

/// One column value as a Verilog UDP table symbol: `1` high, `0` low, `?` any.
#[derive(Clone, Copy, PartialEq, Eq)]
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
/// register carries the OTHER clocks' levels in a conditioned capture's cols; those clocks are clock
/// columns of the primitive, not data ports. Both are excluded here (for a single clock the clock is
/// never in `er.cols`, so this reduces to removing the register's own symbol).
fn data_cols(er: &EdgeCaptures) -> Vec<&Symbol> {
    let clocks = er.clocks();
    er.cols
        .iter()
        .filter(|c| **c != er.node && !clocks.contains(c))
        .collect()
}

/// One edge-register signal's UDP: an edge-sensitive sequential `primitive` whose ports are the pin, its
/// data columns (`data_cols`) and its keying clocks (`EdgeCaptures::clocks`). The header, every table row
/// and the wrapper's instance list those columns in one order: Verilog matches a row's fields and an
/// instance's connections to the header by position. The `reg` captures on each active clock edge (`(01)`
/// for `Rise`, `(10)` for `Fall`) and honours async set/clear as clock-independent level rows. The
/// register's own symbol (a toggle flop's self-feedback) and the clocks are excluded from the data
/// columns — the self column is the `reg` current-state, and each clock is a clock column of its own.
pub struct EdgePrimitive<'a> {
    name: PrimName<'a>,
    captures: &'a EdgeCaptures,
}

impl fmt::Display for EdgePrimitive<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (name, pin, er) = (self.name, self.name.pin, self.captures);
        let clocks = er.clocks();
        let cols = data_cols(er);
        // Ports: the pin, then its data columns (self and clocks excluded) and its clocks, in the order the
        // rows and the wrapper's instance lay them out in.
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
        writeln!(f, "primitive {name}({ports});")?;
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

/// The edge-register UDP table rows, each laid out over the data columns ([`data_cols`]) and the clocks
/// ([`EdgeCaptures::clocks`]) in the primitive's port order (see [`EdgePrimitive`]); the current-state
/// (`reg`) field is `?` except on a self-referencing register's capture rows, where it carries that
/// register's own literal. Each capture row carries exactly ONE edge indicator (IEEE 1364); the
/// capturing clock's column holds it while every other clock column carries the conditioning level. For
/// a single clock every rule reduces exactly to the single-clock rows.
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
            rows.extend(cubes.cubes().map(|cube| {
                region_row(
                    er,
                    &cols,
                    &clocks,
                    Some(&capture.clock),
                    cube.inputs(),
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
                .cubes()
                .map(|cube| region_row(er, &cols, &clocks, None, cube.inputs(), action)),
        );
    }

    // (c) Opposite-edge ignore: for each clock, each edge face with NO capture entry holds on a
    // transition of that edge — one row carrying that edge indicator in the clock's column and `?`
    // elsewhere. A single-edge clock emits its one inactive edge; a dual-edge clock, both
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

    // A UDP consumer reads these rows as a set, as it does `table_rows`'s above, so their order is free.
    rows
}

/// One region row of an edge-register table: the data columns (`cols`, self and clocks excluded) read
/// out of `row` by name and the clock columns (`clocks`), in the primitive's port order, then the
/// current-state (`reg`) field and the `next` action. When `active` names a clock edge, that clock's
/// column carries the edge and every OTHER clock column carries its level from `row` (a conditioned
/// capture references the other clock's level); when `active` is `None` (a clock-independent level row)
/// every clock column reads its `row` level, which is `?` for an off-edge region since it never
/// references a clock. A data or clock column the row does not name is a don't-care in it and reads `?`.
/// The `reg` field is `?` unless the register is self-referencing (its own symbol in `er.cols`), in which
/// case it carries that node's literal from `row` — the capture's dependence on the register's own prior
/// state.
fn region_row(
    er: &EdgeCaptures,
    cols: &[&Symbol],
    clocks: &[&Symbol],
    active: Option<&PinEdge>,
    row: &Minterm<Symbol>,
    next: Next,
) -> EdgeRow {
    let mut cells: Vec<EdgeColumn> = cols
        .iter()
        .map(|&c| EdgeColumn::Level(Level(row.value_of(c))))
        .collect();
    // Clock columns: the capturing clock carries its edge indicator, every other clock its conditioning
    // level from the row.
    cells.extend(clocks.iter().map(|&clock| match active {
        Some(active) if clock == &active.pin => EdgeColumn::Edge(active.edge),
        _ => EdgeColumn::Level(Level(row.value_of(clock))),
    }));
    let reg = if er.cols.contains(&er.node) {
        Level(row.value_of(&er.node))
    } else {
        Level(None)
    };
    EdgeRow { cells, reg, next }
}

/// One row of an edge-sensitive UDP table: its columns in the primitive's port order, the current-state
/// (`reg`) field and the state the register takes.
#[derive(PartialEq, Eq)]
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
#[derive(PartialEq, Eq)]
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
    edge_by_node: &HashMap<&str, &'a EdgeCaptures>,
    folded: &HashSet<&str>,
) -> Wrapper<'a> {
    let outputs: Vec<&Symbol> = cell.outputs.iter().map(|o| &o.name).collect();
    // Read-gated outputs (continuous assigns) and their minted factored registers (internal wires driven
    // by an edge UDP).
    let read_of: HashMap<&str, &StateRegions> = read_functions(cell);
    let signal_names: HashSet<&str> = cell
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
        // Each instance connects in its primitive's port order: an edge register its pin, data columns and
        // clocks as `EdgePrimitive` declares them; a constant pin just its own port; any other sequential
        // pin its own port and then its columns.
        let args: Vec<&Symbol> = if let Some(er) = edge_by_node.get(sig.name.as_str()) {
            std::iter::once(&sig.name)
                .chain(data_cols(er))
                .chain(er.clocks())
                .collect()
        } else if sr.hold.num_cubes() == 0 && (sr.on.num_cubes() == 0 || sr.off.num_cubes() == 0) {
            vec![&sig.name]
        } else {
            std::iter::once(&sig.name).chain(sr.cols.iter()).collect()
        };
        instances.push(Instance {
            name: PrimName::new(cell, &sig.name),
            args,
        });
    }
    // The minted factored registers: an edge UDP instance driving the register's own wire, connected in
    // its primitive's port order as any edge register is.
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
            read_of.get(sig.name.as_str()).map(|&reads| Assign {
                pin: &sig.name,
                function: on_expr(&reads.on),
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

/// One read-gated output's continuous assignment: the output pin and the Boolean function it reads off
/// the factored register and its gate pins.
struct Assign<'a> {
    pin: &'a Symbol,
    function: BoolExpr,
}

impl fmt::Display for Assign<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "assign {} = {};", self.pin, self.function)
    }
}

/// The function a region's on-set states, lowered from the minimised cover that holds it. A cover with
/// no cube has no output to lower and is the constant `false` (`BddBuilder::build_cover`: "an empty
/// cover is `false`").
fn on_expr(cover: &Cover<Symbol, Anonymous>) -> BoolExpr {
    if cover.num_cubes() == 0 {
        return BoolExpr::constant(false);
    }
    cover
        .to_expr_by_index(0)
        .expect("a region cover carries its single anonymous output")
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::emit::liberty::tests::same_multiset;
    use crate::model::{analyse_both, analyse_one as analyse, AnalysedPair};
    use espresso_logic::{bdd_builder, CoverType, Cube, CubeType, ExprNode, OutputSet};

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
        let rows = udp_rows_over(&v, "C2_Q", &["A", "B"]);
        assert!(v.contains("reg    Q;"));
        // Hysteresis appears as no-change rows, on/off as 1/0.
        assert!(v.contains(": ? : -;"));
        assert!(has_row(&rows, "1 1 : ? : 1;"));
        assert!(has_row(&rows, "0 0 : ? : 0;"));
        // Wrapper module + specify + instantiation.
        assert!(v.contains("`celldefine"));
        assert!(v.contains("module C2(Q, A, B);"));
        assert!(v.contains("(A => Q) = (0.1, 0.1);"));
        assert_instance_follows_ports(&v, "C2_Q");
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
        // `no_edge_collapse` keeps the two-latch form, so the master M is a level latch of its own.
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
        assert_eq!(multiset(udp_ports(&v, "DFF_M")), ["CLK", "D"]);
        assert_eq!(multiset(udp_ports(&v, "DFF_Q")), ["CLK", "M"]);
        // Module ports are the external face only; M is an internal wire, both UDPs instantiated.
        assert!(v.contains("module DFF(Q, CLK, D);"));
        assert!(v.contains("wire   M;"));
        assert_instance_follows_ports(&v, "DFF_M");
        assert_instance_follows_ports(&v, "DFF_Q");
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
        // One edge-sensitive UDP over D and the clock; captures on the rising edge.
        let rows = udp_rows_over(&v, "DFF_Q", &["D", "CLK"]);
        assert!(v.contains("reg    Q;"));
        assert!(has_row(&rows, "1 (01) : ? : 1;"));
        assert!(has_row(&rows, "0 (01) : ? : 0;"));
        // The folded master leaves no trace: no primitive, no wire, no instance.
        assert!(!v.contains("DFF_M"));
        assert!(!v.contains("wire   M;"));
        assert_instance_follows_ports(&v, "DFF_Q");
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
        let rows = udp_rows_over(&v, "BDET_Y_st", &["D", "CLK"]);
        assert!(has_row(&rows, "0 (01) : ? : 1;") && has_row(&rows, "1 (01) : ? : 0;"));
        assert!(has_row(&rows, "0 (10) : ? : 1;") && has_row(&rows, "1 (10) : ? : 0;"));
        // The read-gated output is a continuous assign over Y_st and A — never a UDP of its own.
        assert!(v.contains("assign Y = "));
        assert!(
            !v.contains("primitive BDET_Y("),
            "Y is an assign, not a primitive"
        );
        // Y_st is an internal wire, instantiated; Y is the module output. Folded masters leave no trace.
        assert!(v.contains("wire   Y_st;"));
        assert_instance_follows_ports(&v, "BDET_Y_st");
        assert!(v.contains("module BDET(Y, CLK, D, A);"));
        assert!(!v.contains("BDET_L1") && !v.contains("BDET_L2"));
    }

    /// Whether an expression is in sum-of-products form: built from variables, constants, negation,
    /// conjunction and disjunction, with no exclusive-or. Factoring a shared literal out of two products
    /// keeps that form and so does negating a whole block, so this accepts `A & (B | C)` and `!(A | B)`
    /// as readily as `A & B | A & C`, and says nothing about which the lowering picks.
    fn is_sop(expr: &BoolExpr) -> bool {
        expr.fold(|node: ExprNode<'_, bool>| match node {
            ExprNode::Variable(_) | ExprNode::Constant(_) => true,
            ExprNode::Not(inner) => inner,
            ExprNode::And(left, right) | ExprNode::Or(left, right) => left && right,
            ExprNode::Xor(..) => false,
        })
    }

    #[test]
    fn a_lowered_region_is_a_sum_of_products() {
        // The emitter states a pin's logic as whatever `Cover::to_expr_by_index` lowers its on-region to,
        // so nothing here fixes the text. What it must not stop being is a sum of products — an XOR would
        // be a change in the upstream rendering rather than in this cell, and the point of asserting the
        // form rather than the text is to catch that without pinning a rendering the tool is free to
        // change.
        //
        // Two regions, one that cannot factor and one that must be free to: AOI2's primes `A*B` and `!C`
        // share no literal, while AOA's `A*B` and `A*C` share `A`. Both are sums of products either way.
        for (name, function) in [("AOI2", "A*B + !C"), ("AOA", "A*B + A*C")] {
            let cell = analyse(&format!(
                r#"
[[cell]]
name = "{name}"
inputs = ["A", "B", "C"]
[cell.outputs]
Y = "{function}"
"#
            ));
            let expr = on_expr(&cell.regions[0].on);
            assert!(
                is_sop(&expr),
                "{name}: `{function}` lowers to a sum of products, got `{expr}`"
            );
        }
    }

    #[test]
    fn read_gate_assign_states_the_read_function_over_its_columns() {
        // BDET again: the factored register `Y_st` captures `!D`, so the read gate `Y = !(X * A)` over
        // the register's own value `X = !Y_st` is `Y_st + !A`. Reading the emitted expression back and
        // comparing the two functions in one manager checks the text denotes that, whichever equivalent
        // form the cover lowers to.
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
        let line = v
            .lines()
            .find_map(|l| l.trim().strip_prefix("assign Y = "))
            .expect("the read-gated output emits a continuous assign");
        let emitted = line.strip_suffix(';').expect("the assign is terminated");
        let builder = bdd_builder!();
        let expected = builder.parse("Y_st + !A").expect("the reference parses");
        let actual = builder
            .parse(emitted)
            .expect("the emitted expression parses");
        assert!(
            actual.equivalent_to(&expected),
            "emitted `{emitted}` must denote Y_st + !A"
        );
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
        // The async reset RA emits a clock-independent LEVEL clear row (next 0) in enA's table: read over
        // enA's columns `sela2, RA, CLKA`, the clear pattern is `? 1 ?`.
        let en_a = udp_rows_over(&v, "ICM_enA", &["sela2", "RA", "CLKA"]);
        assert!(has_row(&en_a, "? 1 ? : ? : 0;"));

        // The surviving registers instantiate in their primitives' own port order.
        assert_eq!(
            multiset(udp_ports(&v, "ICM_sela2")),
            ["CLKA", "RA", "S", "enB"]
        );
        assert_instance_follows_ports(&v, "ICM_sela2");
        assert_instance_follows_ports(&v, "ICM_enA");
    }

    /// The table body of one named `primitive` (from its header up to `endprimitive`), for asserting
    /// per-primitive row content without matching the same token in a sibling UDP.
    fn prim_block<'a>(v: &'a str, head: &str) -> &'a str {
        let start = v.find(head).expect("primitive present");
        let rest = &v[start..];
        let end = rest.find("endprimitive").expect("endprimitive terminator");
        &rest[..end]
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
        // rising edge AND from CLKB on its falling edge -- both keying clocks are ports, no arc dropped.
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
            "Q captures on CLKB falling (its own latch opening) -- both keying clocks are ports, no arc dropped"
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
        assert_eq!(multiset(udp_ports(&v, "ND2_Y")), ["A", "B"]);
        assert!(!v.contains(": ? : -;")); // no hysteresis
    }

    /// Whether `a` and `b` are the same level UDP, reading no order: the same name, the same input columns
    /// matched by name, and the same table rows as a multiset. A row holds its pattern as a `Minterm`, which
    /// compares by variable name, so two rows agree however each run ordered the columns they are written
    /// over.
    fn same_primitive(a: &Primitive, b: &Primitive) -> bool {
        a.name.cell == b.name.cell
            && a.name.pin == b.name.pin
            && same_multiset(&a.regions.cols, &b.regions.cols, |x, y| x == y)
            && same_multiset(&table_rows(a.regions), &table_rows(b.regions), |x, y| {
                x.row == y.row && x.next == y.next
            })
    }

    /// Whether `a` and `b` are the same wrapper, reading no order: the same declared name, and the same
    /// ports, wires and instances as multisets. An instance's connections follow its primitive's column
    /// order, which [`same_primitive`] reads by name, so they too are compared as a multiset.
    fn same_wrapper(a: &Wrapper, b: &Wrapper) -> bool {
        a.name == b.name
            && same_multiset(&a.outputs, &b.outputs, |x, y| x == y)
            && same_multiset(a.inputs, b.inputs, |x, y| x == y)
            && same_multiset(&a.internals, &b.internals, |x, y| x == y)
            && same_multiset(&a.instances, &b.instances, |x, y| {
                x.name.cell == y.name.cell
                    && x.name.pin == y.name.pin
                    && same_multiset(&x.args, &y.args, |p, q| p == q)
            })
    }

    /// Whether `a` and `b` are the same declaration, under [`same_primitive`] or [`same_wrapper`], or as a
    /// constant pin of the same value.
    fn same_item(a: &Item, b: &Item) -> bool {
        match (a, b) {
            (Item::Primitive(x), Item::Primitive(y)) => same_primitive(x, y),
            (Item::Constant(x), Item::Constant(y)) => {
                x.name.cell == y.name.cell && x.name.pin == y.name.pin && x.value == y.value
            }
            (Item::Wrapper(x), Item::Wrapper(y)) => same_wrapper(x, y),
            _ => false,
        }
    }

    /// Assert `a` and `b` state the same level-sensitive Verilog, reading no order: the same declarations
    /// as a multiset under [`same_item`]. Which order the connections of one run take is a correspondence
    /// within that run — an instance connects to its primitive's ports by position — and is pinned by
    /// `level_shapes_state_each_state_signal_as_a_level_udp`. An edge register's rows are positional over
    /// its data and clock columns, and a read-gated output is a continuous assignment; neither is read
    /// here, so a run stating either fails rather than passing uncompared.
    fn assert_same_level_verilog(a: &AnalysedCell, b: &AnalysedCell) {
        let items_a = cell_verilog(a);
        let items_b = cell_verilog(b);
        for item in items_a.iter().chain(&items_b) {
            match item {
                Item::EdgeRegister(p) => {
                    panic!(
                        "{} is an edge register, which this reading leaves out",
                        p.name
                    )
                }
                Item::Wrapper(w) => assert!(
                    w.assigns.is_empty(),
                    "{} reads a register through a gate, which this reading leaves out",
                    w.name
                ),
                Item::Primitive(_) | Item::Constant(_) => {}
            }
        }
        assert!(
            same_multiset(&items_a, &items_b, same_item),
            "{}\nis not\n{}",
            Verilog(&items_a),
            Verilog(&items_b)
        );
    }

    /// The names a rendered declaration connects, in its own order: those listed between the first `(` of
    /// `line` and the `)` after it — a primitive header's ports, or an instance's connections.
    fn connections(line: &str) -> Vec<&str> {
        let open = line.find('(').expect("a connection list");
        let close = open + line[open..].find(')').expect("a closed connection list");
        line[open + 1..close].split(',').map(str::trim).collect()
    }

    /// `names` as a multiset, held sorted so two compare with `==`: which names a list holds, without the
    /// order the run picked for them.
    pub(crate) fn multiset(mut names: Vec<&str>) -> Vec<&str> {
        names.sort_unstable();
        names
    }

    /// The header line of the rendered UDP `name`.
    fn udp_header<'a>(v: &'a str, name: &str) -> &'a str {
        let head = format!("primitive {name}(");
        v.lines()
            .find(|l| l.starts_with(&head))
            .unwrap_or_else(|| panic!("primitive {name} is declared"))
    }

    /// The input ports the rendered UDP `name` declares after its output pin, in its own order: the columns
    /// its table rows line up with.
    fn udp_ports<'a>(v: &'a str, name: &str) -> Vec<&'a str> {
        connections(udp_header(v, name))[1..].to_vec()
    }

    /// The table rows of the rendered UDP `name`, each read by column name: `cols` lists the UDP's input
    /// ports in the order the caller writes rows against, and each row's fields are re-laid in that order
    /// through where each column sits among the ports the primitive declares, so a row the caller writes
    /// compares with `==` whatever port order the run picked. Asserts the ports are `cols` as a multiset. A
    /// row comes back as `<fields> : <reg> : <next>;`.
    fn udp_rows_over(v: &str, name: &str, cols: &[&str]) -> Vec<String> {
        let ports = udp_ports(v, name);
        assert_eq!(
            multiset(ports.clone()),
            multiset(cols.to_vec()),
            "the input ports of {name}"
        );
        let at: Vec<usize> = cols
            .iter()
            .map(|col| ports.iter().position(|p| p == col).expect("a port"))
            .collect();
        prim_block(v, &format!("primitive {name}("))
            .lines()
            .skip_while(|l| l.trim() != "table")
            .skip(1)
            .take_while(|l| l.trim() != "endtable")
            .map(|line| {
                let fields: Vec<&str> = line.split(':').collect();
                let cells: Vec<&str> = fields[0].split_whitespace().collect();
                let relaid: Vec<&str> = at.iter().map(|&i| cells[i]).collect();
                format!(
                    "{} : {} : {}",
                    relaid.join(" "),
                    fields[1].trim(),
                    fields[2].trim()
                )
            })
            .collect()
    }

    /// Whether `rows` holds `row`.
    fn has_row(rows: &[String], row: &str) -> bool {
        rows.iter().any(|r| r == row)
    }

    /// Assert every rendered instance of the UDP `name` connects in the order the primitive declares its
    /// ports, Verilog connecting an instance by position.
    fn assert_instance_follows_ports(v: &str, name: &str) {
        let ports = connections(udp_header(v, name));
        let head = format!("{name} u_{name} (");
        let instances: Vec<&str> = v.lines().filter(|l| l.starts_with(&head)).collect();
        assert!(!instances.is_empty(), "{name} is instantiated");
        for instance in instances {
            assert_eq!(
                connections(instance),
                ports,
                "u_{name} connects in the order its primitive declares its ports"
            );
        }
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

    /// `no_edge_collapse` suppresses the behavioural edge classification, "leaving every arc in its
    /// combinational form" (`Cell::no_edge_collapse`), and the annotation it suppresses,
    /// `AnalysedCell::edge`, "never alters the exploration". The Verilog reads four parts of that
    /// annotation — the recognised registers, the masters folded into them, the derived read-gate
    /// registers and the outputs factored over them — so where classification finds none of the four,
    /// the switch permits no change to the Verilog. What the Verilog holds is pinned directly by
    /// `level_shapes_state_each_state_signal_as_a_level_udp`.
    #[test]
    fn non_collapsible_suite_verilog_matches_the_no_edge_collapse_flag() {
        // No clock-edge indicator (`(01)`/`(10)`) appears, whether the flag is left off (default
        // collapse, a no-op on these shapes) or forced on -- and the two runs state the same Verilog.
        for src in NON_COLLAPSIBLE {
            let AnalysedPair { default, forced } = analyse_both(src);
            let name = default.repr_name();
            assert!(
                default.edge.captures.is_empty(),
                "unexpected edge register recognised in {name}"
            );
            assert!(
                default.edge.folded.is_empty(),
                "unexpected master folded in {name}"
            );
            assert!(
                default.edge.derived.is_empty(),
                "unexpected read-gate register derived in {name}"
            );
            assert!(
                default.edge.factored.is_empty(),
                "unexpected output factored in {name}"
            );
            let v_default = emit(&default);
            let v_forced = emit(&forced);
            for v in [&v_default, &v_forced] {
                assert!(!v.contains("(01)"), "unexpected rising-edge token");
                assert!(!v.contains("(10)"), "unexpected falling-edge token");
            }
            assert_same_level_verilog(&default, &forced);
        }
    }

    /// What each level shape states in Verilog, read off its spec. Each state signal — the output `Q`, and
    /// in the two-latch shapes the master `M` — is a level UDP of its own, whose input ports are the
    /// signals its function reads, its own state aside. Read over those ports, its `1`, `0` and `-` rows
    /// are where the function drives the pin high whatever its prior state (`∀self. f`), drives it low
    /// whatever its prior state (`∀self. ¬f`), and depends on that prior state (the gap the two leave) —
    /// the regions `crate::logic::regions` derives. The wrapper's ports are `Q` and the declared inputs, `M`
    /// is an internal wire, and each UDP is instantiated once with its connections in the order its
    /// primitive declares its ports, Verilog connecting an instance by position. The shapes state nothing
    /// else: no edge register, constant or continuous assignment.
    #[test]
    fn level_shapes_state_each_state_signal_as_a_level_udp() {
        for src in NON_COLLAPSIBLE {
            let cell = analyse(src);
            let name = cell.repr_name().as_str();
            let mut signals: Vec<&str> = match name {
                "DLAT" | "GLAT" => vec!["Q"],
                "MCDFF" | "UCDFF" => vec!["Q", "M"],
                other => panic!("no expectation stated for {other}"),
            };
            signals.sort_unstable();
            // `M`, in the shapes that have it, is the internal the spec declares.
            let wires: Vec<&str> = signals.iter().copied().filter(|s| *s == "M").collect();

            let items = cell_verilog(&cell);
            let mut primitives: Vec<&Primitive> = Vec::new();
            let mut wrappers: Vec<&Wrapper> = Vec::new();
            for item in &items {
                match item {
                    Item::Primitive(p) => primitives.push(p),
                    Item::Wrapper(w) => wrappers.push(w),
                    Item::EdgeRegister(p) => panic!("{name} states an edge register {}", p.name),
                    Item::Constant(c) => panic!("{name} states a constant pin {}", c.name),
                }
            }
            let mut pins: Vec<&str> = primitives.iter().map(|p| p.name.pin.as_str()).collect();
            pins.sort_unstable();
            assert_eq!(
                pins, signals,
                "{name} states one level UDP per state signal"
            );

            for p in &primitives {
                let pin = p.name.pin;
                let (sig, _) = cell
                    .signal_regions()
                    .find(|(sig, _)| sig.name == *pin)
                    .expect("a UDP models one of the cell's signals");
                // The reference regions, built from the signal's own function in the manager the rows are
                // rebuilt in below, so the two compare.
                let builder = bdd_builder!();
                let f = builder.build(&sig.expr);
                let own: Vec<&str> = if sig.feedback.contains(pin) {
                    vec![pin.as_str()]
                } else {
                    vec![]
                };
                let on = f.forall(&own);
                let off = (!f.clone()).forall(&own);
                let hold = !on.or(&off);
                let reads: Vec<Symbol> = f.variables().filter(|v| v != pin).collect();
                assert!(
                    same_multiset(&p.regions.cols, &reads, |x, y| x == y),
                    "{name}.{pin}'s input ports are the signals its function reads"
                );
                let rows = table_rows(p.regions);
                for next in [Next::On, Next::Off, Next::Hold] {
                    let region = match next {
                        Next::On => &on,
                        Next::Off => &off,
                        Next::Hold => &hold,
                    };
                    // Each row as it reads over the UDP's ports, a port it does not name being `?`.
                    let cubes = rows.iter().filter(|r| r.next == next).map(|r| {
                        Cube::new(
                            r.row.project_to_labels(p.regions.cols.iter().cloned()),
                            OutputSet::anonymous(&[true]),
                            CubeType::F,
                        )
                    });
                    let stated = builder.build_cover(&Cover::from_cubes(CoverType::F, cubes));
                    assert!(
                        stated.equivalent_to(region),
                        "{name}.{pin}: the `{next}` rows state the region they stand for"
                    );
                }
            }

            let [w] = wrappers.as_slice() else {
                panic!("{name} declares one name, so states one wrapper");
            };
            let outputs: Vec<&str> = w.outputs.iter().map(|s| s.as_str()).collect();
            assert_eq!(outputs, ["Q"], "{name}'s one output port");
            assert!(
                same_multiset(w.inputs, &cell.inputs, |x, y| x == y),
                "{name}'s input ports are the declared inputs"
            );
            let mut internals: Vec<&str> = w.internals.iter().map(|s| s.as_str()).collect();
            internals.sort_unstable();
            assert_eq!(internals, wires, "{name}'s internal wires");
            assert!(
                w.assigns.is_empty(),
                "{name} reads no register through a gate"
            );
            assert_eq!(
                w.instances.len(),
                primitives.len(),
                "{name} instantiates each UDP once"
            );
            let v = emit(&cell);
            for p in &primitives {
                assert_instance_follows_ports(&v, &p.name.to_string());
            }
        }
    }

    #[test]
    fn dff_opt_out_restores_master_primitive_via_either_switch() {
        // The two-latch DFF, opted out directly (`no_edge_collapse = true` in the TOML) and opted out
        // via the CLI-flag-equivalent blanket mutation over the whole spec. Each run states the
        // two-latch model the spec writes: a `DFF_M` master transparent while CLK is low and holding
        // while it is high, a `DFF_Q` slave keyed off M -- holding while CLK is low and transparent
        // while it is high -- M an internal wire rather than a module port, and no edge row anywhere,
        // the collapse being off. The flag acts "exactly as if each had declared
        // `no_edge_collapse = true`" (`apply_overrides`), so the two switches permit no difference at all.
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
        const DECLARED: &str = r#"
[[cell]]
name = "DFF"
inputs = ["CLK", "D"]
clock = ["CLK"]
no_edge_collapse = true
[cell.internal]
M = "!CLK*D + CLK*M"
[cell.outputs]
Q = "CLK*M + !CLK*Q"
"#;
        let direct = analyse(DECLARED);
        let via_flag = {
            // Mirrors apply_overrides's blanket application of `--no-edge-collapse` over every cell.
            let mut spec = crate::model::parse_spec(DFF).unwrap();
            for c in &mut spec.cells {
                c.no_edge_collapse = true;
            }
            spec.cells.remove(0).analyse().unwrap()
        };

        let v_direct = emit(&direct);
        let v_via_flag = emit(&via_flag);
        for v in [&v_direct, &v_via_flag] {
            eprintln!("{v}");
            // The master is the negative-level latch the spec writes: it passes D while CLK is low and
            // holds while CLK is high. Its rows read against the port list the same run wrote, the UDP
            // being a columnar format.
            let m = udp_rows_over(v, "DFF_M", &["CLK", "D"]);
            assert!(has_row(&m, "0 0 : ? : 0;"));
            assert!(has_row(&m, "0 1 : ? : 1;"));
            assert!(has_row(&m, "1 ? : ? : -;"));
            // The slave is the positive-level latch, and it keys off M rather than D -- which is what
            // the opt-out preserves: a collapsed Q would capture D on the clock edge instead.
            let q = udp_rows_over(v, "DFF_Q", &["CLK", "M"]);
            assert!(has_row(&q, "0 ? : ? : -;"));
            assert!(has_row(&q, "1 0 : ? : 0;"));
            assert!(has_row(&q, "1 1 : ? : 1;"));
            // M is the cell's internal node, so it is a wire the master drives, not a module port.
            assert!(v.contains("module DFF(Q, CLK, D);"));
            assert!(v.contains("wire   M;"));
            // Neither UDP carries an edge row: nothing collapsed.
            assert!(!v.contains("(01)"), "unexpected rising-edge token");
            assert!(!v.contains("(10)"), "unexpected falling-edge token");
        }
        assert_same_level_verilog(&direct, &via_flag);
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
        let q = udp_rows_over(&v, "EMDFF_Q", &["D", "CLK"]);
        assert!(has_row(&q, "0 (01) : ? : 0;"));
        assert!(has_row(&q, "1 (01) : ? : 1;"));
        assert_eq!(multiset(udp_ports(&v, "EMDFF_M")), ["CLK", "D"]);
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
        let q = udp_rows_over(&v, "DET_Q", &["D", "CLK"]);
        // Both edges capture D; each row carries exactly one edge indicator.
        assert!(has_row(&q, "0 (01) : ? : 0;"));
        assert!(has_row(&q, "1 (01) : ? : 1;"));
        assert!(has_row(&q, "0 (10) : ? : 0;"));
        assert!(has_row(&q, "1 (10) : ? : 1;"));
        for row in &q {
            let edges = row.matches("(01)").count() + row.matches("(10)").count();
            assert!(edges <= 1, "row carries more than one edge token: {row}");
        }
        // No opposite-edge no-change row: the only `-` rows are the steady-clock data-ignore rows.
        assert!(!has_row(&q, "? (10) : ? : -;"));
        assert!(!has_row(&q, "? (01) : ? : -;"));
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
        let q = udp_rows_over(&v, "IDFF_Q", &["D", "CLK"]);
        assert!(has_row(&q, "0 (01) : ? : 1;"));
        assert!(has_row(&q, "1 (01) : ? : 0;"));
        // Single-edge register keeps the opposite-edge no-change row and folds its master.
        assert!(has_row(&q, "? (10) : ? : -;"));
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
        let q = udp_rows_over(&v, "TFF_Q", &["R", "CLK"]);
        let declared: Vec<&str> = prim_block(&v, "primitive TFF_Q(")
            .lines()
            .find_map(|l| l.strip_prefix("input"))
            .expect("an input declaration")
            .trim()
            .trim_end_matches(';')
            .split(',')
            .map(str::trim)
            .collect();
        assert_eq!(
            multiset(declared),
            ["CLK", "R"],
            "self Q is not an input port"
        );
        // The rising capture prints Q's own literal in the current-state (reg) field, not `?`.
        assert!(has_row(&q, "0 (01) : 0 : 1;"));
        assert!(has_row(&q, "? (01) : 1 : 0;"));
        // M captures the same toggle on the falling edge, keying off the surviving Q (an input to M's UDP).
        assert_eq!(multiset(udp_ports(&v, "TFF_M")), ["CLK", "Q", "R"]);
        // The self-fed master survives as an internal wire, and neither instance duplicates M.
        assert!(v.contains("wire   M;"));
        assert_instance_follows_ports(&v, "TFF_Q");
        assert_instance_follows_ports(&v, "TFF_M");
    }
}
