//! Emit a behavioural Verilog model for a cell: one sequential UDP `primitive` per output pin (its
//! three-valued next-state table built from the on/off/hold regions) wrapped in a `celldefine`d
//! `module` that instantiates the primitives and carries a `specify` block of path delays.
//!
//! Mirrors hsNCL `outPinUDP`/`cellAliasModule` (`Circuit/NCLCell.hs`), including its structure: the
//! UDP for output `x` takes the pin's own state as the `reg`/current-state column and every other
//! signal (primary inputs + other outputs) as an input column, so a self-holding cell keeps its
//! hysteresis as `-` (no-change) rows. Pins are emitted in declaration order (lobsterate's deliberate
//! divergence from hsNCL's alphabetical sort).

use crate::logic::regions::{StateCube, StateRegions};
use crate::model::AnalysedCell;

/// Fixed rise/fall path delay stamped on every `specify` arc, matching the hsNCL template.
const PATH_DELAY: &str = "(0.1, 0.1)";

/// The full Verilog model for a cell: a UDP primitive per signal (outputs **and** internal state
/// nodes) followed by the wrapper module. Internal nodes are modelled exactly like outputs, but appear
/// as internal `wire`s in the wrapper rather than as module ports.
pub fn cell_verilog(cell: &AnalysedCell) -> String {
    let mut out = String::new();
    for (sig, sr) in cell.signal_regions() {
        out.push_str(&primitive(&prim_name(cell, &sig.name), &sig.name, sr));
    }
    out.push_str(&wrapper_module(cell));
    out
}

/// `<cell>_<pin>` — the UDP primitive name for one output pin.
fn prim_name(cell: &AnalysedCell, pin: &str) -> String {
    format!("{}_{}", cell.name, pin)
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

/// The `celldefine`d wrapper module: declares the cell's ports, a `specify` path delay from every
/// input to every output, and instantiates each output pin's UDP with that pin's own column set.
fn wrapper_module(cell: &AnalysedCell) -> String {
    let outputs: Vec<String> = cell.outputs.iter().map(|o| o.name.clone()).collect();
    let internals: Vec<String> = cell.internals.iter().map(|o| o.name.clone()).collect();
    // Ports are the external face only: outputs and primary inputs. Internal state nodes are not ports.
    let ports = outputs
        .iter()
        .cloned()
        .chain(cell.inputs.iter().cloned())
        .collect::<Vec<_>>()
        .join(", ");

    let mut s = String::from("`celldefine\n");
    s.push_str(&format!("module {}({ports});\n", cell.name));
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

    // Instantiate every signal's UDP (outputs and internals); an internal drives its own wire.
    for (sig, sr) in cell.signal_regions() {
        let name = prim_name(cell, &sig.name);
        // Constant pins instantiate with just their own port; sequential pins add their columns.
        let args = if sr.hold.is_empty() && (sr.on.is_empty() || sr.off.is_empty()) {
            sig.name.clone()
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
        let cell = analyse(
            r#"
[[cell]]
name = "DFF"
inputs = ["CLK", "D"]
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
}
