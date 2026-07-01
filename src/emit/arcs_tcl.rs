//! Emit Cadence Liberate `define_arc` blocks for a cell's transition arcs.
//!
//! The layout mirrors hsNCL `genRiseTransition`/`genFallTransition`, including the quirk that rise
//! arcs place `-type` first while fall arcs place it after the prevector. Pins are emitted in
//! declaration order (lobsterate's deliberate divergence from hsNCL's alphabetical sort).

use crate::logic::arcs::{cell_arcs, Arc, Edge};
use crate::logic::assignment;
use crate::logic::confluence::{Constraint, ConstraintKind};
use crate::logic::interlock::Arbitration;
use crate::model::AnalysedCell;

/// Knobs for the arc emitter.
#[derive(Debug, Clone, Copy, Default)]
pub struct ArcsTclOptions {
    /// Emit a `-when` condition (the other inputs' fixed values in the end state) on each arc.
    /// Off by default — Liberate can usually infer the conditioning from the vector, and hsNCL's
    /// `-when` differs from ours in pin ordering. Kept available for cells that need it.
    pub emit_when: bool,
    /// Emit derived constraint arcs (setup/hold, non_seq) from state-machine confluence. Off by default;
    /// a cell can opt in individually via `constraint_arcs = true`. See [`crate::logic::confluence`].
    pub emit_constraints: bool,
}

/// All `define_arc` blocks for a cell, concatenated. Interlocked (mutex/arbiter) cells are prefixed
/// with a comment documenting the metastable condition, which timing arcs cannot express. When enabled,
/// derived constraint arcs (setup/hold, non_seq) follow the delay arcs.
pub fn cell_arcs_tcl(cell: &AnalysedCell, opts: ArcsTclOptions) -> String {
    let mut out = arbitration_comment(cell);
    for arc in &cell_arcs(cell) {
        out.push_str(&format_arc(cell, arc, opts));
    }
    if opts.emit_constraints || cell.constraint_arcs_declared {
        for c in &cell.constraints {
            out.push_str(&format_constraint(cell, c));
        }
    }
    out
}

/// A constraint arc as a pair of `define_arc` blocks — the setup member and the hold member (Liberate
/// characterises them as separate arcs): `setup`/`hold` for a directed clock↔data constraint,
/// `non_seq_setup`/`non_seq_hold` for a symmetric (arbitration / mutual-exclusion) one.
fn format_constraint(cell: &AnalysedCell, c: &Constraint) -> String {
    let (setup, hold) = match c.kind {
        ConstraintKind::SetupHold => ("setup", "hold"),
        ConstraintKind::NonSeq => ("non_seq_setup", "non_seq_hold"),
    };
    let mut s = constraint_block(cell, c, setup);
    s.push_str(&constraint_block(cell, c, hold));
    s
}

/// One constraint `define_arc` of the given `-type`. Liberate cannot infer how to prepare these
/// non-standard state-holding cells, so every pin is listed and fully specified: the `-prevector`
/// drives the cell (inputs + internal state) into the pre-toggle state, and the full `-vector` carries
/// the two switching pins as `R`/`F`, the other inputs at their held value, and the outputs as `X`.
fn constraint_block(cell: &AnalysedCell, c: &Constraint, arc_type: &str) -> String {
    let mut s = String::from("define_arc \\\n");
    s.push_str(&format!("\t-type {arc_type} \\\n"));
    s.push_str(&format!(
        "\t-prevector_pinlist {{{}}} \\\n",
        cell.inputs.join(" ")
    ));
    s.push_str(&format!(
        "\t-prevector {{{}}} \\\n",
        prevector_str(cell, &c.prevector)
    ));
    s.push_str(&format!("\t-pinlist {{{}}} \\\n", pinlist_str(cell)));
    s.push_str(&format!(
        "\t-vector {{{}}} \\\n",
        constraint_vector_str(cell, c)
    ));
    s.push_str(&format!("\t-related_pin {} \\\n", c.related));
    s.push_str(&format!("\t-pin {} \\\n", c.pin));
    s.push_str(&format!("\t{{ {} }}\n", cell.name));
    s.push('\n');
    s
}

/// The full constraint vector over `pinlist_str` order (inputs then outputs): the related and pin pins
/// as their `R`/`F` edges, every other input at its held value in the pre-toggle state (the prevector's
/// last step), and every output as `X` (a constraint arc measures no output transition).
fn constraint_vector_str(cell: &AnalysedCell, c: &Constraint) -> String {
    let edge = |e: Edge| match e {
        Edge::Rise => "R",
        Edge::Fall => "F",
    };
    let held = c.prevector.last().map(assignment).unwrap_or_default();
    let mut parts = Vec::with_capacity(cell.inputs.len() + cell.outputs.len());
    for input in &cell.inputs {
        if *input == c.related {
            parts.push(edge(c.related_edge).to_string());
        } else if *input == c.pin {
            parts.push(edge(c.pin_edge).to_string());
        } else {
            parts.push(
                if *held.get(input).unwrap_or(&false) {
                    "1"
                } else {
                    "0"
                }
                .to_string(),
            );
        }
    }
    for _ in &cell.outputs {
        parts.push("X".to_string());
    }
    parts.join(" ")
}

/// A `#` comment block describing each detected arbitration condition (empty for ordinary cells).
fn arbitration_comment(cell: &AnalysedCell) -> String {
    let mut s = String::new();
    for a in &cell.arbitration {
        let states: Vec<String> = a.stable.iter().map(Arbitration::state_str).collect();
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
        let tcl = cell_arcs_tcl(&cell, ArcsTclOptions::default());
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
        let off = cell_arcs_tcl(
            &cell,
            ArcsTclOptions {
                emit_when: false,
                ..Default::default()
            },
        );
        let on = cell_arcs_tcl(
            &cell,
            ArcsTclOptions {
                emit_when: true,
                ..Default::default()
            },
        );
        assert!(!off.contains("-when"));
        assert!(on.contains("-when"));
    }

    #[test]
    fn dff_constraint_arcs_gated_and_setup_hold_under_declared_clock() {
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
        // Off by default: no constraint arcs.
        let off = cell_arcs_tcl(&cell, ArcsTclOptions::default());
        assert!(!off.contains("-type setup"));
        assert!(!off.contains("-type hold"));

        // Enabled: separate setup and hold blocks of D w.r.t. CLK. With CLK declared a clock the CLK/D
        // hazard is a setup/hold, so no non_seq is produced for the pair.
        let on = cell_arcs_tcl(
            &cell,
            ArcsTclOptions {
                emit_constraints: true,
                ..Default::default()
            },
        );
        eprintln!("{on}");
        assert!(on.contains("-type setup \\"));
        assert!(on.contains("-type hold \\"));
        assert!(on.contains("-related_pin CLK"));
        assert!(on.contains("-pin D"));
        assert!(!on.contains("non_seq"));
    }

    #[test]
    fn mutex_emits_non_seq_constraint_arcs_when_enabled() {
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
        let on = cell_arcs_tcl(
            &cell,
            ArcsTclOptions {
                emit_constraints: true,
                ..Default::default()
            },
        );
        eprintln!("{on}");
        assert!(on.contains("-type non_seq_setup \\"));
        assert!(on.contains("-type non_seq_hold \\"));
        // Both request pins appear as related/pin of the constraint.
        assert!(on.contains("-related_pin A"));
        assert!(on.contains("-pin B"));
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
        let tcl = cell_arcs_tcl(&cell, ArcsTclOptions::default());
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
        let tcl = cell_arcs_tcl(&cell, ArcsTclOptions::default());
        assert!(tcl.contains("-type async"));
        assert!(tcl.contains("-related_pin R"));
    }
}
