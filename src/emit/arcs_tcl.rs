//! Emit Cadence Liberate `define_arc` blocks for a cell's transition arcs.
//!
//! The layout mirrors hsNCL `genRiseTransition`/`genFallTransition`, including the quirk that rise
//! arcs place `-type` first while fall arcs place it after the prevector. Pins are emitted in
//! declaration order (lobsterate's deliberate divergence from hsNCL's alphabetical sort).

use crate::logic::arcs::{cell_arcs, Arc, Edge};
use crate::logic::interlock::Arbitration;
use crate::logic::walk::{assignment, WalkError};
use crate::model::AnalysedCell;

/// Knobs for the arc emitter.
#[derive(Debug, Clone, Copy, Default)]
pub struct ArcsTclOptions {
    /// Emit a `-when` condition (the other inputs' fixed values in the end state) on each arc.
    /// Off by default — Liberate can usually infer the conditioning from the vector, and hsNCL's
    /// `-when` differs from ours in pin ordering. Kept available for cells that need it.
    pub emit_when: bool,
}

/// All `define_arc` blocks for a cell, concatenated. Interlocked (mutex/arbiter) cells are prefixed
/// with a comment documenting the metastable condition, which timing arcs cannot express.
pub fn cell_arcs_tcl(cell: &AnalysedCell, opts: ArcsTclOptions) -> Result<String, WalkError> {
    let mut out = arbitration_comment(cell);
    for arc in &cell_arcs(cell)? {
        out.push_str(&format_arc(cell, arc, opts));
    }
    Ok(out)
}

/// A `#` comment block describing each detected arbitration condition (empty for ordinary cells).
fn arbitration_comment(cell: &AnalysedCell) -> String {
    let mut s = String::new();
    for a in &cell.arbitration {
        let states: Vec<String> = a
            .stable
            .iter()
            .map(|st| Arbitration::state_str(st))
            .collect();
        s.push_str(&format!(
            "# arbitration: {} metastable; grants {{{}}} mutually exclusive ({})\n",
            a.condition_str(),
            a.group.join(", "),
            states.join(" | "),
        ));
    }
    s
}

fn format_arc(cell: &AnalysedCell, arc: &Arc, opts: ArcsTclOptions) -> String {
    let type_line = format!(
        "\t-type {} \\\n",
        if arc.is_async {
            "async"
        } else {
            "combinational"
        }
    );
    let prevector_pinlist = format!("\t-prevector_pinlist {{{}}} \\\n", cell.inputs.join(" "));
    let prevector = format!(
        "\t-prevector {{{}}} \\\n",
        prevector_str(cell, &arc.prevector)
    );
    let pinlist = format!("\t-pinlist {{{}}} \\\n", pinlist_str(cell));
    let vector = format!("\t-vector {{{}}} \\\n", vector_str(cell, arc));
    let when = match (opts.emit_when, when_str(arc)) {
        (true, Some(w)) => format!("\t-when \"{w}\" \\\n"),
        _ => String::new(),
    };
    let related = format!("\t-related_pin {} \\\n", arc.related);
    let pin = format!("\t-pin {} \\\n", arc.output);
    let name = format!("\t{{ {} }}\n", cell.name);

    let mut s = String::from("define_arc \\\n");
    match arc.edge {
        // Rise: -type, then prevector. Fall: prevector, then -type (matches hsNCL).
        Edge::Rise => {
            s.push_str(&type_line);
            s.push_str(&prevector_pinlist);
            s.push_str(&prevector);
        }
        Edge::Fall => {
            s.push_str(&prevector_pinlist);
            s.push_str(&prevector);
            s.push_str(&type_line);
        }
    }
    s.push_str(&pinlist);
    s.push_str(&vector);
    s.push_str(&when);
    s.push_str(&related);
    s.push_str(&pin);
    s.push_str(&name);
    s.push('\n');
    s
}

fn pinlist_str(cell: &AnalysedCell) -> String {
    let mut pins = cell.inputs.clone();
    pins.extend(cell.outputs.iter().map(|o| o.name.clone()));
    pins.join(" ")
}

/// Render the prevector: one bit-string per walk step (a `0`/`1` per input pin, in declaration
/// order), steps separated by spaces.
fn prevector_str(
    cell: &AnalysedCell,
    path: &[espresso_logic::Minterm<espresso_logic::Symbol>],
) -> String {
    path.iter()
        .map(|m| {
            let a = assignment(m);
            cell.inputs
                .iter()
                .map(|i| {
                    if *a.get(i).unwrap_or(&false) {
                        '1'
                    } else {
                        '0'
                    }
                })
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// The measured vector: the related input pin and the measured output as `R`/`F`, the other inputs
/// as their `1`/`0` value in the end state, and the other outputs as `X`.
fn vector_str(cell: &AnalysedCell, arc: &Arc) -> String {
    let end = assignment(&arc.end);
    let mut parts = Vec::with_capacity(cell.inputs.len() + cell.outputs.len());
    for input in &cell.inputs {
        let value = *end.get(input).unwrap_or(&false);
        if *input == arc.related {
            parts.push(if value { "R" } else { "F" }.to_string());
        } else {
            parts.push(if value { "1" } else { "0" }.to_string());
        }
    }
    for output in &cell.outputs {
        if output.name == arc.output {
            parts.push(
                match arc.edge {
                    Edge::Rise => "R",
                    Edge::Fall => "F",
                }
                .to_string(),
            );
        } else {
            parts.push("X".to_string());
        }
    }
    parts.join(" ")
}

/// The `-when` condition: the other inputs' fixed values in the end state, as a product of literals
/// (`*` AND, `!` NOT). `None` when no other input is fixed (the arc is unconditional).
fn when_str(arc: &Arc) -> Option<String> {
    let mut lits: Vec<(String, bool)> = assignment(&arc.end)
        .into_iter()
        .filter(|(k, _)| *k != arc.related)
        .collect();
    if lits.is_empty() {
        return None;
    }
    lits.sort();
    Some(
        lits.iter()
            .map(|(k, v)| if *v { k.clone() } else { format!("!{k}") })
            .collect::<Vec<_>>()
            .join("*"),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::parse_spec;

    fn analyse(src: &str) -> AnalysedCell {
        parse_spec(src).unwrap().cells.remove(0).analyse().unwrap()
    }

    #[test]
    fn c_element_emits_well_formed_arcs() {
        let cell = analyse(
            r#"
[[cell]]
name = "C2"
inputs = ["A", "B"]
[cell.outputs]
Q = "A*B + Q*(A+B)"
"#,
        );
        let tcl = cell_arcs_tcl(&cell, ArcsTclOptions::default()).unwrap();
        eprintln!("{tcl}"); // visible with `cargo test -- --nocapture`

        assert!(tcl.contains("define_arc \\"));
        assert!(tcl.contains("-related_pin A"));
        assert!(tcl.contains("-related_pin B"));
        assert!(tcl.contains("-pin Q"));
        assert!(tcl.contains("-prevector_pinlist {A B}"));
        assert!(tcl.contains("-pinlist {A B Q}"));
        assert!(tcl.contains("{ C2 }"));
        // every block is balanced and combinational here
        assert_eq!(
            tcl.matches("define_arc").count(),
            tcl.matches("-pin Q").count()
        );
        assert!(!tcl.contains("-type async"));
        // -when is off by default.
        assert!(!tcl.contains("-when"));
    }

    #[test]
    fn when_flag_toggles_when_clause() {
        let cell = analyse(
            r#"
[[cell]]
name = "C2"
inputs = ["A", "B"]
[cell.outputs]
Q = "A*B + Q*(A+B)"
"#,
        );
        let off = cell_arcs_tcl(&cell, ArcsTclOptions { emit_when: false }).unwrap();
        let on = cell_arcs_tcl(&cell, ArcsTclOptions { emit_when: true }).unwrap();
        assert!(!off.contains("-when"));
        assert!(on.contains("-when"));
    }

    #[test]
    fn mutex_emits_arbitration_comment_and_input_only_related_pins() {
        let cell = analyse(
            r#"
[[cell]]
name = "MUT"
inputs = ["A", "B"]
[cell.outputs]
Qa = "!Qb * A"
Qb = "!Qa * B"
"#,
        );
        let tcl = cell_arcs_tcl(&cell, ArcsTclOptions::default()).unwrap();
        eprintln!("{tcl}");
        // Arbitration documented up front.
        assert!(tcl.contains("# arbitration: A*B metastable"));
        assert!(tcl.contains("Qa, Qb"));
        // Related pins are primary inputs only — never an output (a Qb→Qa arc is a deadlock).
        assert!(!tcl.contains("-related_pin Qa"));
        assert!(!tcl.contains("-related_pin Qb"));
        assert!(tcl.contains("-related_pin A"));
        assert!(tcl.contains("-related_pin B"));
        assert!(tcl.contains("-prevector_pinlist {A B}"));
        assert!(tcl.contains("-pinlist {A B Qa Qb}"));
    }

    #[test]
    fn async_reset_emits_async_type() {
        let cell = analyse(
            r#"
[[cell]]
name = "RC2"
inputs = ["A", "B", "R"]
async = ["R"]
[cell.outputs]
Q = "(A*B + Q*(A+B))*!R"
"#,
        );
        let tcl = cell_arcs_tcl(&cell, ArcsTclOptions::default()).unwrap();
        assert!(tcl.contains("-type async"));
        assert!(tcl.contains("-related_pin R"));
    }
}
