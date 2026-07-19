//! Emit a behavioural Verilog model for a cell: one sequential UDP `primitive` per output pin (its
//! three-valued next-state table built from the on/off/hold regions) wrapped in a `celldefine`d
//! `module` that instantiates the primitives and carries a `specify` block of path delays.
//!
//! The UDP for output `x` takes the pin's own state as the `reg`/current-state column and every other
//! signal (primary inputs + other outputs) as an input column, so a self-holding cell keeps its
//! hysteresis as `-` (no-change) rows. Pins are emitted in declaration order.
//!
//! A signal recognised as an edge-triggered register ([`crate::logic::edge`]) emits an
//! **edge-sensitive** UDP instead: the level-latch rows are replaced by clock-edge (`(01)`/`(10)`)
//! capture rows — one group per active `(clock, edge)`, so a dual-edge register captures on both — plus
//! async set/clear level rows, a no-change row for each clock's inactive edge and no-change rows for
//! steady-clock data transitions. The keying clocks are the primitive's LAST ports. A pure master folded
//! into such a register contributes nothing — no primitive, no wire, no instance.

use std::collections::{BTreeMap, BTreeSet};

use espresso_logic::Symbol;

use crate::logic::arcs::Edge;
use crate::logic::edge::EdgeCaptures;
use crate::logic::regions::{StateCube, StateRegions};
use crate::model::AnalysedCell;

/// Fixed rise/fall path delay stamped on every `specify` arc.
const PATH_DELAY: &str = "(0.1, 0.1)";

/// The full Verilog model for a cell: a UDP primitive per signal (outputs **and** internal state
/// nodes) followed by the wrapper module. Internal nodes are modelled exactly like outputs, but appear
/// as internal `wire`s in the wrapper rather than as module ports.
pub fn cell_verilog(cell: &AnalysedCell) -> String {
    // Recognised edge registers, keyed by their output node, and the pure masters folded into them —
    // a folded master emits no primitive, no wire and no instance.
    let edge_by_node: BTreeMap<&str, &EdgeCaptures> = cell
        .edge
        .captures
        .iter()
        .map(|er| (er.node.as_str(), er))
        .collect();
    let folded: BTreeSet<&str> = cell.edge.folded.iter().map(Symbol::as_str).collect();

    let mut out = String::new();
    for (sig, sr) in cell.signal_regions() {
        if folded.contains(sig.name.as_str()) {
            continue; // pure master folded into its edge register
        }
        match edge_by_node.get(sig.name.as_str()) {
            Some(er) => out.push_str(&edge_primitive(&prim_name(cell, &sig.name), &sig.name, er)),
            None => out.push_str(&primitive(&prim_name(cell, &sig.name), &sig.name, sr)),
        }
    }
    // One `celldefine`d wrapper per name; all wrappers instantiate the same shared primitives.
    for name in &cell.name {
        out.push_str(&wrapper_module(cell, name, &edge_by_node, &folded));
    }
    out
}

/// `<cell>_<pin>` — the UDP primitive name for one output pin.
fn prim_name(cell: &AnalysedCell, pin: &str) -> String {
    format!("{}_{}", cell.repr_name(), pin)
}

/// One output pin's UDP. A constant function lowers to a plain `module` with a continuous `assign`;
/// otherwise to a sequential `primitive` whose `table` encodes on (`1`), off (`0`) and hold (`-`).
fn primitive(name: &str, pin: &str, sr: &StateRegions) -> String {
    // Constant pin: no hold and one region empty ⇒ a tautology / contradiction.
    if sr.hold.is_empty() && sr.off.is_empty() {
        return constant_module(name, pin, true);
    }
    if sr.hold.is_empty() && sr.on.is_empty() {
        return constant_module(name, pin, false);
    }

    let ports = std::iter::once(pin)
        .chain(sr.cols.iter().map(|s| s.as_str()))
        .collect::<Vec<_>>()
        .join(", ");

    let mut s = format!("primitive {name}({ports});\n");
    s.push_str(&format!("output {pin};\n"));
    if !sr.cols.is_empty() {
        s.push_str(&format!("input  {};\n", sr.cols.join(", ")));
    }
    s.push_str(&format!("reg    {pin};\n"));
    s.push_str("table\n");
    for line in table_lines(sr) {
        s.push_str(&format!("\t{line}\n"));
    }
    s.push_str("endtable\n");
    s.push_str("endprimitive\n");
    s
}

/// A constant output pin as a `module` with a continuous assignment (`1'b1` / `1'b0`).
fn constant_module(name: &str, pin: &str, value: bool) -> String {
    let bit = if value { "1'b1" } else { "1'b0" };
    format!("module {name}({pin});\noutput {pin};\nassign {pin} = {bit};\nendmodule\n")
}

/// The UDP table rows, one per region cube, in a deterministic (sorted) order. Each row is
/// `<input pattern> : ? : <next>` where `next` is `1` (on), `0` (off) or `-` (hold).
fn table_lines(sr: &StateRegions) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    for cube in &sr.on {
        lines.push(format!("{} : ? : 1;", pattern(cube)));
    }
    for cube in &sr.off {
        lines.push(format!("{} : ? : 0;", pattern(cube)));
    }
    for cube in &sr.hold {
        lines.push(format!("{} : ? : -;", pattern(cube)));
    }
    lines.sort();
    lines
}

/// Render a cube as space-separated `1`/`0`/`?` symbols over the column header order.
fn pattern(cube: &StateCube) -> String {
    cube.iter()
        .map(|c| match c {
            Some(true) => "1",
            Some(false) => "0",
            None => "?",
        })
        .collect::<Vec<_>>()
        .join(" ")
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
/// `(pin, data cols…, clocks…)` with the keying clocks LAST in [`EdgeCaptures::clocks`] order. The `reg`
/// captures on each active clock edge (`(01)` for `Rise`, `(10)` for `Fall`) and honours async set/clear
/// as clock-independent level rows. The register's own symbol (a toggle flop's self-feedback) and the
/// clocks are excluded from the data columns — the self column is the `reg` current-state and the clocks
/// are the dedicated trailing clock columns, not input ports.
fn edge_primitive(name: &str, pin: &Symbol, er: &EdgeCaptures) -> String {
    let clocks = er.clocks();
    let clock_strs: Vec<&str> = clocks.iter().map(|c| c.as_str()).collect();
    let cols = data_cols(er);
    // Ports: the pin, its data columns (self and clocks excluded), then the clocks last in clocks() order.
    let ports = std::iter::once(pin.as_str())
        .chain(cols.iter().map(|c| c.as_str()))
        .chain(clock_strs.iter().copied())
        .collect::<Vec<_>>()
        .join(", ");
    let inputs = cols
        .iter()
        .map(|c| c.as_str())
        .chain(clock_strs.iter().copied())
        .collect::<Vec<_>>()
        .join(", ");
    // Trailing comment naming the clock column(s); reduces to the single-clock wording for one clock.
    let clock_comment = if clock_strs.len() == 1 {
        format!("clock {} is the last port", clock_strs[0])
    } else {
        format!("clocks {} are the last ports", clock_strs.join(", "))
    };

    let mut s = format!("primitive {name}({ports}); // {clock_comment}\n");
    s.push_str(&format!("output {pin};\n"));
    s.push_str(&format!("input  {inputs};\n"));
    s.push_str(&format!("reg    {pin};\n"));
    s.push_str("table\n");
    for line in edge_table_lines(er) {
        s.push_str(&format!("\t{line}\n"));
    }
    s.push_str("endtable\n");
    s.push_str("endprimitive\n");
    s
}

/// The edge-register UDP table rows, sorted for determinism. Column order is the data cols (`er.cols`
/// minus the register's own symbol and the clocks) then the clocks in [`EdgeCaptures::clocks`] order; the
/// current-state (`reg`) field is `?` except on a self-referencing register's capture rows, where it
/// carries that register's own literal. Each capture row carries exactly ONE edge indicator (IEEE 1364);
/// the capturing clock's column holds it while every other clock column carries the conditioning level.
/// For a single clock every rule reduces exactly to the single-clock rows.
fn edge_table_lines(er: &EdgeCaptures) -> Vec<String> {
    let cols = data_cols(er);
    let clocks = er.clocks();
    let mut lines: Vec<String> = Vec::new();

    // (a) Capture rows: the combinational next-state sampled on one active edge of one clock. Each
    // capture carries its own clock; a dual-edge (or multi-clock) register contributes one group per
    // `(clock, edge)`, each keeping its single edge indicator in the capturing clock's column.
    for (clock, edge, capture) in &er.captures {
        let capture_edge = match edge {
            Edge::Rise => "(01)",
            Edge::Fall => "(10)",
        };
        for cube in &capture.on {
            lines.push(region_row(
                er,
                &cols,
                &clocks,
                Some((clock, capture_edge)),
                &capture.cols,
                cube,
                "1",
            ));
        }
        for cube in &capture.off {
            lines.push(region_row(
                er,
                &cols,
                &clocks,
                Some((clock, capture_edge)),
                &capture.cols,
                cube,
                "0",
            ));
        }
    }

    // (b) Async set/clear as LEVEL rows (every clock `?`): by IEEE 1364 a level row dominates the edge
    // rows, and F1/F2 guarantee any overlap agrees, so the set/clear wins independent of the clocks.
    for cube in &er.off_edge.on {
        lines.push(region_row(
            er,
            &cols,
            &clocks,
            None,
            &er.off_edge.cols,
            cube,
            "1",
        ));
    }
    for cube in &er.off_edge.off {
        lines.push(region_row(
            er,
            &cols,
            &clocks,
            None,
            &er.off_edge.cols,
            cube,
            "0",
        ));
    }

    // (c) Opposite-edge ignore: for each clock, each edge face with NO capture entry holds on a
    // transition of that edge — one row carrying that edge indicator in the clock's column and `?`
    // elsewhere. A single-edge clock emits its one inactive edge (as today); a dual-edge clock, both
    // faces captured, emits none.
    for &clock in &clocks {
        for (edge, indicator) in [(Edge::Rise, "(01)"), (Edge::Fall, "(10)")] {
            if er.captures.iter().any(|(c, e, _)| c == clock && *e == edge) {
                continue;
            }
            let mut cells: Vec<&str> = cols.iter().map(|_| "?").collect();
            for &c in &clocks {
                cells.push(if c == clock { indicator } else { "?" });
            }
            lines.push(format!("{} : ? : -;", cells.join(" ")));
        }
    }

    // (d) Steady-clock data-transition ignore: a change on any data column with every clock stable holds.
    for i in 0..cols.len() {
        let mut cells: Vec<&str> = (0..cols.len())
            .map(|j| if i == j { "(??)" } else { "?" })
            .collect();
        cells.extend(clocks.iter().map(|_| "?"));
        lines.push(format!("{} : ? : -;", cells.join(" ")));
    }

    lines.sort();
    lines
}

/// One region row of an edge-register table: the data columns (`cols`, self and clocks excluded) filled
/// from `cube` by column name against `region_cols`, then the clock columns (`clocks`, in order), the
/// current-state (`reg`) field and the `next` action. When `active` is `Some((clock, indicator))` the
/// capturing clock's column carries that edge `indicator` and every OTHER clock column carries its level
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
    active: Option<(&Symbol, &'static str)>,
    region_cols: &[Symbol],
    cube: &StateCube,
    next: &str,
) -> String {
    let mut cells: Vec<&str> = cols
        .iter()
        .map(|&c| level_at(c, region_cols, cube))
        .collect();
    // Clock columns LAST, in clocks() order: the capturing clock carries its edge indicator, every other
    // clock its conditioning level from the cube.
    for &clock in clocks {
        match active {
            Some((active_clock, indicator)) if clock == active_clock => cells.push(indicator),
            _ => cells.push(level_at(clock, region_cols, cube)),
        }
    }
    let reg = if er.cols.contains(&er.node) {
        level_at(&er.node, region_cols, cube)
    } else {
        "?"
    };
    format!("{} : {reg} : {next};", cells.join(" "))
}

/// The `1`/`0`/`?` level of column `col` in `cube`, looked up by name against `region_cols` (a subset
/// of the register's columns). Absent — the region does not constrain this column — reads `?`.
fn level_at(col: &Symbol, region_cols: &[Symbol], cube: &StateCube) -> &'static str {
    match region_cols
        .iter()
        .position(|c| c == col)
        .and_then(|i| cube[i])
    {
        Some(true) => "1",
        Some(false) => "0",
        None => "?",
    }
}

/// The `celldefine`d wrapper module: declares the cell's ports, a `specify` path delay from every
/// input to every output, and instantiates each output pin's UDP with that pin's own column set.
fn wrapper_module(
    cell: &AnalysedCell,
    name: &Symbol,
    edge_by_node: &BTreeMap<&str, &EdgeCaptures>,
    folded: &BTreeSet<&str>,
) -> String {
    let outputs: Vec<Symbol> = cell.outputs.iter().map(|o| o.name.clone()).collect();
    // Folded masters vanish: they are neither a wire nor an instance.
    let internals: Vec<Symbol> = cell
        .internals
        .iter()
        .filter(|o| !folded.contains(o.name.as_str()))
        .map(|o| o.name.clone())
        .collect();
    // Ports are the external face only: outputs and primary inputs. Internal state nodes are not ports.
    let ports = outputs
        .iter()
        .cloned()
        .chain(cell.inputs.iter().cloned())
        .collect::<Vec<_>>()
        .join(", ");

    let mut s = String::from("`celldefine\n");
    s.push_str(&format!("module {name}({ports});\n"));
    s.push_str(&format!("output {};\n", outputs.join(", ")));
    if !cell.inputs.is_empty() {
        s.push_str(&format!("input  {};\n", cell.inputs.join(", ")));
    }
    // Internal state nodes are internal wires driven by their own UDP instance.
    if !internals.is_empty() {
        s.push_str(&format!("wire   {};\n", internals.join(", ")));
    }

    s.push_str("specify\n");
    for input in &cell.inputs {
        for output in &outputs {
            s.push_str(&format!("\t({input} => {output}) = {PATH_DELAY};\n"));
        }
    }
    s.push_str("endspecify\n");

    // Instantiate every surviving signal's UDP (outputs and internals); an internal drives its own
    // wire. A folded master has no instance.
    for (sig, sr) in cell.signal_regions() {
        if folded.contains(sig.name.as_str()) {
            continue;
        }
        let name = prim_name(cell, &sig.name);
        // Edge registers connect in port order `(pin, cols…, clocks…)` with the clocks last in
        // clocks() order; constant pins take just their own port; other sequential pins add their columns.
        let args = if let Some(er) = edge_by_node.get(sig.name.as_str()) {
            let clocks = er.clocks();
            std::iter::once(sig.name.as_str())
                .chain(data_cols(er).iter().map(|c| c.as_str()))
                .chain(clocks.iter().map(|c| c.as_str()))
                .collect::<Vec<_>>()
                .join(", ")
        } else if sr.hold.is_empty() && (sr.on.is_empty() || sr.off.is_empty()) {
            sig.name.to_string()
        } else {
            std::iter::once(sig.name.as_str())
                .chain(sr.cols.iter().map(|s| s.as_str()))
                .collect::<Vec<_>>()
                .join(", ")
        };
        s.push_str(&format!("{name} u_{name} ({args});\n"));
    }

    s.push_str("endmodule\n");
    s.push_str("`endcelldefine\n");
    s
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::analyse_one as analyse;

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
        let v = cell_verilog(&cell);
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
        let v = cell_verilog(&cell);
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
        let v = cell_verilog(&cell);
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
        let v = cell_verilog(&cell);
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
        let v = cell_verilog(&cell);
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

    #[test]
    fn dcmux_udp_keys_both_clocks_last_with_edge_rows() {
        // DCMUX: a genuinely independent two-clock capture. Q's UDP keys off BOTH clocks (as its LAST
        // ports) and captures on each -- no clock-privileging, no per-output suppression.
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
        let v = cell_verilog(&cell);
        eprintln!("{v}");
        // Both clocks are the primitive's trailing ports (clocks LAST, in clocks() order).
        assert!(
            v.contains(", CLKA, CLKB);") && v.contains("primitive DCMUX_Q("),
            "Q UDP keys both clocks last"
        );
        let q = prim_block(&v, "primitive DCMUX_Q(");
        // Both clocks contribute capture rows; each row carries exactly ONE edge indicator, the other
        // keying clock sitting at `?` (a level don't-care).
        let clka_i = q_port_index(&v, "DCMUX_Q", "CLKA");
        let clkb_i = q_port_index(&v, "DCMUX_Q", "CLKB");
        let mut saw_clka_edge = false;
        let mut saw_clkb_edge = false;
        for row in q
            .lines()
            .filter(|l| l.contains("(01)") || l.contains("(10)"))
        {
            let cells: Vec<&str> = row.split(':').next().unwrap().split_whitespace().collect();
            let edges = row.matches("(01)").count() + row.matches("(10)").count();
            assert!(
                edges == 1,
                "each capture row carries exactly one edge token: {row}"
            );
            if matches!(cells.get(clka_i), Some(c) if c.starts_with('(')) {
                saw_clka_edge = true;
            }
            if matches!(cells.get(clkb_i), Some(c) if c.starts_with('(')) {
                saw_clkb_edge = true;
            }
        }
        assert!(saw_clka_edge, "a CLKA edge capture row");
        assert!(saw_clkb_edge, "a CLKB edge capture row");
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
        let v = cell_verilog(&cell);
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
            if field(row, clka_i) == "(01)" {
                saw_clka_rise = true;
            }
            if field(row, clkb_i) == "(10)" {
                saw_clkb_fall = true;
            }
        }
        assert!(saw_clka_rise, "Q captures on CLKA rising");
        assert!(
            saw_clkb_fall,
            "Q captures on CLKB falling, alongside CLKA's rise"
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
        let v = cell_verilog(&cell);
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
        let v = cell_verilog(&cell);
        assert!(v.contains("primitive ND2_Y(Y, A, B);"));
        assert!(!v.contains(": ? : -;")); // no hysteresis
    }

    /// Parse the single-cell `src` and analyse it twice: once as written, once with
    /// `no_edge_collapse` forced true on every cell -- the same blanket mutation the
    /// `--no-edge-collapse` CLI flag applies (main.rs:82-88). Proves the per-cell TOML switch and
    /// the CLI flag are the identical code path, not two independently-tested mechanisms.
    fn analyse_both(src: &str) -> (crate::model::AnalysedCell, crate::model::AnalysedCell) {
        let default = crate::model::parse_spec(src)
            .unwrap()
            .cells
            .remove(0)
            .analyse()
            .unwrap();
        let mut spec = crate::model::parse_spec(src).unwrap();
        for c in &mut spec.cells {
            c.no_edge_collapse = true;
        }
        let forced = spec.cells.remove(0).analyse().unwrap();
        (default, forced)
    }

    /// Four shapes the behavioural classifier recognises as NO edge register even under default (on)
    /// collapse: a single latch, a gated (self-referencing) latch, a master/slave pair split across two
    /// DIFFERENT declared clocks (the slave stays level — its data is transparent in one phase of the
    /// clock that gates it), and a two-latch DFF whose clock is never declared. The exposed-master DFF
    /// (a master surfaced as a second output) now DOES collapse behaviourally and is covered as a
    /// positive fixture in `exposed_master_collapses_slave_over_surviving_master`.
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
        // collapse, a no-op on these shapes) or forced on -- and the two runs emit byte-identical
        // Verilog.
        for src in NON_COLLAPSIBLE {
            let (default, forced) = analyse_both(src);
            let v_default = cell_verilog(&default);
            let v_forced = cell_verilog(&forced);
            for v in [&v_default, &v_forced] {
                assert!(!v.contains("(01)"), "unexpected rising-edge token");
                assert!(!v.contains("(10)"), "unexpected falling-edge token");
            }
            assert_eq!(v_default, v_forced);
        }
    }

    #[test]
    fn dff_opt_out_restores_master_primitive_via_either_switch() {
        // The two-latch DFF, opted out directly (`no_edge_collapse = true` in the TOML) versus opted
        // out via the CLI-flag-equivalent blanket mutation over the whole spec: both switches restore
        // the SAME two-latch Verilog -- a `DFF_M` primitive and wire, absent under default collapse.
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

        let v_direct = cell_verilog(&direct);
        let v_via_flag = cell_verilog(&via_flag);
        for v in [&v_direct, &v_via_flag] {
            assert!(v.contains("primitive DFF_M("));
            assert!(v.contains("wire   M;"));
        }
        assert_eq!(v_direct, v_via_flag);
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
        let v = cell_verilog(&cell);
        eprintln!("{v}");
        // Q is a rising-edge register over the surviving master M; the master keeps its own level UDP.
        assert!(v.contains("primitive EMDFF_Q(Q, M, CLK);"));
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
        let v = cell_verilog(&cell);
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
        let v = cell_verilog(&cell);
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
        // A resettable toggle flip-flop decomposes into TWO edge registers: Q captures the master M on
        // the rising edge, and M captures !Q (== !M over the reachable pre-fall states) on the falling
        // edge. M is SELF-referencing: its own symbol must NOT become a UDP input port -- it is the
        // `reg` current-state field, carrying M's own literal in the capture rows.
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
        let v = cell_verilog(&cell);
        eprintln!("{v}");
        // Two edge registers: Q (rising, captures M) and the self-fed master M (falling).
        assert!(v.contains("primitive TFF_Q(Q, M, R, CLK);"));
        // M's own symbol is excluded from its ports/inputs: the self column is the reg, not an input.
        assert!(v.contains("primitive TFF_M(M, R, CLK);"));
        let m = prim_block(&v, "primitive TFF_M(");
        assert!(m.contains("input  R, CLK;"), "self M is not an input port");
        // The falling capture prints M's own literal in the current-state (reg) field, not `?`.
        assert!(m.contains("0 (10) : 0 : 1;"));
        assert!(m.contains("? (10) : 1 : 0;"));
        // The self-fed master survives as an internal wire, and neither instance duplicates M.
        assert!(v.contains("wire   M;"));
        assert!(v.contains("TFF_M u_TFF_M (M, R, CLK);"));
        assert!(v.contains("TFF_Q u_TFF_Q (Q, M, R, CLK);"));
    }
}
