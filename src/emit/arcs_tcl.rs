//! Emit Cadence Liberate `define_arc` blocks for a cell's transition arcs.
//!
//! The layout places `-type` first on rise arcs and after the prevector on fall arcs, with pins
//! emitted in declaration order.

use espresso_logic::Symbol;

use crate::logic::arcs::{Arc, Edge, HiddenArc};
use crate::logic::assignment;
use crate::logic::confluence::{Constraint, ConstraintKind};
use crate::logic::hazard::Oscillation;
use crate::logic::leakage::LeakageState;
use crate::logic::literal_product;
use crate::model::AnalysedCell;

/// Knobs for the arc emitter.
#[derive(Debug, Clone, Copy)]
pub struct ArcsTclOptions {
    /// Emit a `-when` condition (the other inputs' fixed values in the end state) on each arc, keeping
    /// every held-input context as its own conditioned arc. **On by default.** When off, arcs that share
    /// a (related, pin, edge) collapse to one — a single prevector exercises the transition, so the
    /// distinct held contexts (which only `-when` would distinguish) are redundant.
    pub emit_when: bool,
    /// Emit hidden (whole-cell internal-power) arcs — an input toggles but no output changes — as
    /// `-type hidden` blocks. **On by default.**
    pub emit_internal: bool,
    /// Emit `define_leakage` blocks — one per static leakage state (the settled seed states of the
    /// machine exploration), conditioned on the cell's inputs and settled outputs. **On by default.**
    pub emit_leakage: bool,
}

impl Default for ArcsTclOptions {
    fn default() -> Self {
        Self {
            emit_when: true,
            emit_internal: true,
            emit_leakage: true,
        }
    }
}

/// All `define_arc` blocks for a cell, concatenated. A cell with a detected oscillation hazard is
/// prefixed with a comment recording the racing condition and the competing settled outcomes — the
/// metastability risk timing arcs cannot express. Any derived constraint arcs (setup/hold, non_seq)
/// the cell opted into — its `constraint_arcs` was set, so generation populated `cell.constraints` —
/// follow the delay arcs.
pub fn cell_arcs_tcl(cell: &AnalysedCell, opts: ArcsTclOptions) -> String {
    let mut out = oscillation_comment(cell);
    // Without `-when`, arcs of the same (related, pin, edge) that differ only in the held-input context
    // are the *same* arc — one prevector is enough to exercise it, so collapse them (keeping the
    // shortest prevector). With `-when` each held context is a distinct characterisation condition and
    // is kept.
    let arcs = if opts.emit_when {
        cell.arcs.clone()
    } else {
        collapse_conditions(&cell.arcs)
    };
    let edge_clocks = edge_register_clocks(cell);
    for arc in &arcs {
        out.push_str(&format_arc(cell, arc, opts, &edge_clocks));
    }
    if opts.emit_internal {
        let hidden = if opts.emit_when {
            cell.hidden_arcs.clone()
        } else {
            collapse_hidden(&cell.hidden_arcs)
        };
        for h in &hidden {
            out.push_str(&format_hidden_arc(cell, h, opts));
        }
    }
    if opts.emit_leakage {
        for l in &cell.leakage {
            out.push_str(&format_leakage(cell, l));
        }
    }
    // Constraint arcs emit whatever generation produced: `cell.constraints` is populated only when the
    // cell opted in (per-cell `constraint_arcs`, or the global `--constraints` flag), and is empty
    // otherwise — so this loop is its own gate.
    for c in &cell.constraints {
        out.push_str(&format_constraint(cell, c));
    }
    out
}

/// The cell's name(s), braced as a Tcl list: `{ C2 }` for a single name, `{ C2A C2B }` for several.
fn name_block(cell: &AnalysedCell) -> String {
    format!("{{ {} }}", cell.name.join(" "))
}

/// A constraint arc as a pair of `define_arc` blocks — the setup member and the hold member (Liberate
/// characterises them as separate arcs): `setup`/`hold` for a directed clock↔data constraint,
/// `non_seq_setup`/`non_seq_hold` for a symmetric (oscillation / mutual-exclusion) one.
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
    s.push_str(&format!("\t{}\n", name_block(cell)));
    s.push('\n');
    s
}

/// The full constraint vector over `pinlist_str` order (inputs then outputs): the related and pin pins
/// as their `R`/`F` edges, every other input at its held value in the pre-toggle state (the prevector's
/// last step), and every output as `X` (a constraint arc measures no output transition).
fn constraint_vector_str(cell: &AnalysedCell, c: &Constraint) -> String {
    let held = c.prevector.last().map(assignment).unwrap_or_default();
    vector(
        cell,
        |input| {
            if input == c.related {
                c.related_edge.rf().to_string()
            } else if input == c.pin {
                c.pin_edge.rf().to_string()
            } else {
                if *held.get(input).unwrap_or(&false) {
                    "1"
                } else {
                    "0"
                }
                .to_string()
            }
        },
        |_| "X".to_string(),
    )
}

/// A `#` comment block describing each detected oscillation condition (empty for ordinary cells).
fn oscillation_comment(cell: &AnalysedCell) -> String {
    let mut s = String::new();
    for a in &cell.oscillation {
        let states: Vec<String> = a.stable.iter().map(Oscillation::state_str).collect();
        s.push_str(&format!(
            "# oscillation: {} risks metastability in {{{}}}, settling to one of {}\n",
            a.condition_str(),
            a.group.join(", "),
            states.join(" | "),
        ));
    }
    s
}

/// Collapse arcs that share a `(pin, related, edge)` — the same physical transition reached under
/// different held-input contexts — to one, keeping the shortest prevector. Used when `-when` is off,
/// where the held context is not emitted, so a single prevector suffices to exercise the arc.
fn collapse_conditions(arcs: &[Arc]) -> Vec<Arc> {
    use std::collections::btree_map::Entry;
    use std::collections::BTreeMap;
    // Keep references while deduping so only the surviving arcs are cloned.
    let mut best: BTreeMap<(Symbol, Symbol, bool), &Arc> = BTreeMap::new();
    for arc in arcs {
        let key = (
            arc.output.clone(),
            arc.related.clone(),
            matches!(arc.edge, Edge::Rise),
        );
        match best.entry(key) {
            Entry::Vacant(e) => {
                e.insert(arc);
            }
            Entry::Occupied(mut e) => {
                if arc.prevector.len() < e.get().prevector.len() {
                    e.insert(arc);
                }
            }
        }
    }
    best.into_values().cloned().collect()
}

/// Collapse hidden arcs that share a `(pin, edge)` — the same physical input toggle reached under
/// different held-input contexts — to one, keeping the shortest prevector. Used when `-when` is off,
/// where the held context is not emitted, so a single prevector suffices to exercise the arc.
fn collapse_hidden(arcs: &[HiddenArc]) -> Vec<HiddenArc> {
    use std::collections::btree_map::Entry;
    use std::collections::BTreeMap;
    // Keep references while deduping so only the surviving arcs are cloned.
    let mut best: BTreeMap<(Symbol, bool), &HiddenArc> = BTreeMap::new();
    for arc in arcs {
        let key = (arc.pin.clone(), matches!(arc.edge, Edge::Rise));
        match best.entry(key) {
            Entry::Vacant(e) => {
                e.insert(arc);
            }
            Entry::Occupied(mut e) => {
                if arc.prevector.len() < e.get().prevector.len() {
                    e.insert(arc);
                }
            }
        }
    }
    best.into_values().cloned().collect()
}

/// Lookup from a register node (an `edge.registers` entry's `node`) to its `(clock, capturing edge)`
/// pairs — one per capture (a single-edge single-clock register has one, a dual-edge register two, Rise
/// and Fall). A delay arc whose output is one of these nodes, whose `-related_pin` is a keying clock, and
/// whose own edge is that clock's capturing edge is one of the register's clock-to-output edge arcs —
/// `format_arc` re-labels it `-type edge` (a Liberate edge-register delay arc) instead of
/// `-type combinational`. An arc on a non-capturing clock edge is level/latch behaviour and stays
/// `-type combinational`.
fn edge_register_clocks(
    cell: &AnalysedCell,
) -> std::collections::BTreeMap<Symbol, Vec<(Symbol, Edge)>> {
    cell.edge
        .registers
        .iter()
        .map(|r| {
            (
                r.node.clone(),
                r.captures
                    .iter()
                    .map(|(clock, edge, _)| (clock.clone(), *edge))
                    .collect(),
            )
        })
        .collect()
}

/// The edge the arc's `related` clock pin makes, read from its value in the end state — the same
/// derivation the vector uses to render its `R`/`F`. `Rise` when the clock settles high, `Fall` when it
/// settles low. Used to gate `-type edge`: only the clock-to-output arc on the register's *capturing*
/// clock edge is the sequential edge arc; an arc on the opposite (non-capturing) clock edge is
/// level/latch behaviour and stays `-type combinational`.
fn related_edge(arc: &Arc) -> Edge {
    if *assignment(&arc.end).get(&arc.related).unwrap_or(&false) {
        Edge::Rise
    } else {
        Edge::Fall
    }
}

fn format_arc(
    cell: &AnalysedCell,
    arc: &Arc,
    opts: ArcsTclOptions,
    edge_clocks: &std::collections::BTreeMap<Symbol, Vec<(Symbol, Edge)>>,
) -> String {
    let is_edge = !arc.is_async
        && edge_clocks.get(&arc.output).is_some_and(|pairs| {
            pairs
                .iter()
                .any(|(clock, edge)| arc.related == *clock && *edge == related_edge(arc))
        });
    let type_line = format!(
        "\t-type {} \\\n",
        if arc.is_async {
            "async"
        } else if is_edge {
            "edge"
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
    let when = match (opts.emit_when, when_str(&arc.end, &arc.related)) {
        (true, Some(w)) => format!("\t-when \"{w}\" \\\n"),
        _ => String::new(),
    };
    let related = format!("\t-related_pin {} \\\n", arc.related);
    let pin = format!("\t-pin {} \\\n", arc.output);
    let name = format!("\t{}\n", name_block(cell));

    let mut s = String::from("define_arc \\\n");
    match arc.edge {
        // Rise: -type, then prevector. Fall: prevector, then -type.
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

/// A hidden (whole-cell internal-power) `define_arc` of `-type hidden`: the toggled input drives an
/// `R`/`F` edge, every other input sits at its held value in the end state, and every output is pinned
/// at its held `1`/`0` value (never `X` — a hidden arc measures no output transition). Unlike transition
/// arcs there is no `-related_pin`, and `-type hidden` always leads regardless of edge direction.
fn format_hidden_arc(cell: &AnalysedCell, h: &HiddenArc, opts: ArcsTclOptions) -> String {
    let held: std::collections::BTreeMap<&str, bool> =
        h.outputs.iter().map(|(s, b)| (s.as_str(), *b)).collect();
    let end = assignment(&h.end);
    let vec = vector(
        cell,
        |input| {
            if input == h.pin.as_str() {
                h.edge.rf().to_string()
            } else {
                if *end.get(input).unwrap_or(&false) {
                    "1"
                } else {
                    "0"
                }
                .to_string()
            }
        },
        |name| {
            if *held.get(name).expect("hidden arc defines every output") {
                "1"
            } else {
                "0"
            }
            .to_string()
        },
    );

    let mut s = String::from("define_arc \\\n");
    s.push_str("\t-type hidden \\\n");
    s.push_str(&format!(
        "\t-prevector_pinlist {{{}}} \\\n",
        cell.inputs.join(" ")
    ));
    s.push_str(&format!(
        "\t-prevector {{{}}} \\\n",
        prevector_str(cell, &h.prevector)
    ));
    s.push_str(&format!("\t-pinlist {{{}}} \\\n", pinlist_str(cell)));
    s.push_str(&format!("\t-vector {{{vec}}} \\\n"));
    if let (true, Some(w)) = (opts.emit_when, hidden_when_str(h)) {
        s.push_str(&format!("\t-when \"{w}\" \\\n"));
    }
    s.push_str(&format!("\t-pin {} \\\n", h.pin.as_str()));
    s.push_str(&format!("\t{}\n", name_block(cell)));
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

/// One symbol per input (cell.inputs order), then one per output (cell.outputs order), joined by " ".
fn vector(
    cell: &AnalysedCell,
    input_sym: impl Fn(&str) -> String,
    output_sym: impl Fn(&str) -> String,
) -> String {
    let mut parts = Vec::with_capacity(cell.inputs.len() + cell.outputs.len());
    for input in &cell.inputs {
        parts.push(input_sym(input));
    }
    for output in &cell.outputs {
        parts.push(output_sym(&output.name));
    }
    parts.join(" ")
}

/// The measured vector: the related input pin and the measured output as `R`/`F`, the other inputs
/// as their `1`/`0` value in the end state, and the other outputs as `X`.
fn vector_str(cell: &AnalysedCell, arc: &Arc) -> String {
    let end = assignment(&arc.end);
    vector(
        cell,
        |input| {
            let value = *end.get(input).unwrap_or(&false);
            if input == arc.related {
                (if value { Edge::Rise } else { Edge::Fall })
                    .rf()
                    .to_string()
            } else {
                if value { "1" } else { "0" }.to_string()
            }
        },
        |name| {
            if name == arc.output {
                arc.edge.rf().to_string()
            } else {
                "X".to_string()
            }
        },
    )
}

/// The `-when` condition: the other inputs' fixed values in the end state, as a product of literals
/// (`*` AND, `!` NOT). `None` when no other input is fixed (the arc is unconditional).
fn when_str(
    end: &espresso_logic::Minterm<espresso_logic::Symbol>,
    exclude: &str,
) -> Option<String> {
    let mut lits: Vec<(Symbol, bool)> = assignment(end)
        .into_iter()
        .filter(|(k, _)| *k != exclude)
        .collect();
    if lits.is_empty() {
        return None;
    }
    lits.sort();
    Some(crate::logic::literal_product(&lits))
}

/// The hidden arc's `-when` condition: the other inputs' fixed values in the end state (excluding the
/// toggled pin) plus every held output value, as a product of literals. The held outputs disambiguate
/// the distinct stored-value contexts of a state-holding cell that share one input vector. `None` when
/// no literal is fixed.
fn hidden_when_str(h: &HiddenArc) -> Option<String> {
    let mut lits: Vec<(Symbol, bool)> = assignment(&h.end)
        .into_iter()
        .filter(|(k, _)| *k != h.pin.as_str())
        .collect();
    lits.extend(h.outputs.iter().map(|(s, v)| (s.clone(), *v)));
    if lits.is_empty() {
        return None;
    }
    lits.sort();
    Some(crate::logic::literal_product(&lits))
}

/// One-line `define_leakage` for a static leakage state: the stable condition over the cell's
/// inputs and its settled (resolved) outputs.
fn format_leakage(cell: &AnalysedCell, l: &LeakageState) -> String {
    let mut lits: Vec<(Symbol, bool)> = assignment(&l.inputs).into_iter().collect();
    lits.extend(l.outputs.iter().cloned());
    if lits.is_empty() {
        return String::new();
    }
    lits.sort();
    format!(
        "define_leakage -when \"{}\" {}\n",
        literal_product(&lits),
        name_block(cell)
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::analyse_one as analyse;

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
        // every transition block is balanced and combinational here
        assert_eq!(
            tcl.matches("-type combinational").count(),
            tcl.matches("-pin Q").count()
        );
        assert_eq!(
            tcl.matches("define_arc").count(),
            tcl.matches("-pin Q").count() + tcl.matches("-type hidden").count()
        );
        assert!(!tcl.contains("-type async"));
        // -when is emitted by default.
        assert!(tcl.contains("-when"));
    }

    #[test]
    fn and2_emits_hidden_arc_blocks() {
        let cell = analyse(
            r#"
[[cell]]
name = "AND2"
inputs = ["A", "B"]
[cell.outputs]
Y = "A*B"
"#,
        );
        let tcl = cell_arcs_tcl(&cell, ArcsTclOptions::default());
        eprintln!("{tcl}");
        for frag in tcl.split("define_arc") {
            if !frag.contains("-type hidden") {
                continue;
            }
            // Hidden arcs never carry a related pin.
            assert!(!frag.contains("-related_pin"));
            // The toggled input is named by `-pin`.
            assert!(frag.contains("-pin A") || frag.contains("-pin B"));
            // Every output is pinned at its held value — never X.
            assert!(!frag.contains("X"));
        }
        // The A-falls-while-B=0 hidden arc: Y held 0. The held output is folded into `-when` (sorted
        // literals over inputs B and output Y).
        assert!(tcl
            .split("define_arc")
            .any(|frag| frag.contains("-type hidden")
                && frag.contains("-vector {F 0 0}")
                && frag.contains("-when \"!B*!Y\"")
                && frag.contains("-pin A")));
    }

    #[test]
    fn dlatch_hidden_when_carries_held_output() {
        // Transparent-high D-latch: a D toggle in hold (E=0) leaves Q unchanged, but the two stored-value
        // contexts differ in the held Q. Both must be emitted as hidden `-pin D` arcs and disambiguated by
        // the held Q literal folded into `-when`.
        let cell = analyse(
            r#"
[[cell]]
name = "DLAT"
inputs = ["E", "D"]
[cell.outputs]
Q = "E*D + !E*Q"
"#,
        );
        let tcl = cell_arcs_tcl(&cell, ArcsTclOptions::default());
        eprintln!("{tcl}");
        let d_hidden: Vec<&str> = tcl
            .split("define_arc")
            .filter(|frag| frag.contains("-type hidden") && frag.contains("-pin D"))
            .collect();
        // The `-when` of one context holds Q true (`* Q`, not `!Q`) and another holds Q false (`!Q`).
        let when_of = |frag: &str| {
            frag.lines()
                .find(|l| l.contains("-when"))
                .unwrap_or("")
                .to_string()
        };
        assert!(
            d_hidden.iter().any(|frag| when_of(frag).contains("*Q")),
            "expected a D hidden arc whose -when holds Q true"
        );
        assert!(
            d_hidden.iter().any(|frag| when_of(frag).contains("!Q")),
            "expected a D hidden arc whose -when holds Q false"
        );
    }

    #[test]
    fn no_internal_option_suppresses_hidden() {
        let cell = analyse(
            r#"
[[cell]]
name = "AND2"
inputs = ["A", "B"]
[cell.outputs]
Y = "A*B"
"#,
        );
        let off = cell_arcs_tcl(
            &cell,
            ArcsTclOptions {
                emit_internal: false,
                ..Default::default()
            },
        );
        let on = cell_arcs_tcl(&cell, ArcsTclOptions::default());
        assert_eq!(off.matches("-type hidden").count(), 0);
        assert!(on.matches("-type hidden").count() >= 1);
    }

    #[test]
    fn hidden_arcs_collapse_without_when() {
        let cell = analyse(
            r#"
[[cell]]
name = "AND2"
inputs = ["A", "B"]
[cell.outputs]
Y = "A*B"
"#,
        );
        let without_when = cell_arcs_tcl(
            &cell,
            ArcsTclOptions {
                emit_when: false,
                // define_leakage is inherently -when-conditioned; disabled here to isolate arc -when
                // suppression.
                emit_leakage: false,
                ..Default::default()
            },
        );
        let with_when = cell_arcs_tcl(&cell, ArcsTclOptions::default());
        assert!(!without_when.contains("-when"));
        assert!(
            without_when.matches("-type hidden").count()
                <= with_when.matches("-type hidden").count()
        );
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
                // define_leakage is inherently -when-conditioned; disabled here to isolate arc -when
                // suppression.
                emit_leakage: false,
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
    fn collapse_conditions_reduces_shared_context_arcs() {
        // A 3-input majority gate: its output rises via one pin under two distinct held contexts (the
        // other two inputs at 10 or 01). With `-when` off those share a (output, related, edge) and
        // collapse to one representative, so the collapsed set has strictly fewer arcs than the raw set.
        let cell = analyse(
            r#"
[[cell]]
name = "MAJ3"
inputs = ["A", "B", "C"]
[cell.outputs]
Y = "A*B + B*C + A*C"
"#,
        );
        let collapsed = collapse_conditions(&cell.arcs);
        assert!(
            collapsed.len() < cell.arcs.len(),
            "collapse should drop redundant held-context duplicates: {} vs {}",
            collapsed.len(),
            cell.arcs.len(),
        );
        // The emitter reflects the collapse: fewer `define_arc` blocks once `-when` is suppressed.
        let with_when = cell_arcs_tcl(
            &cell,
            ArcsTclOptions {
                emit_when: true,
                ..Default::default()
            },
        );
        let without_when = cell_arcs_tcl(
            &cell,
            ArcsTclOptions {
                emit_when: false,
                ..Default::default()
            },
        );
        assert!(
            without_when.matches("define_arc").count() < with_when.matches("define_arc").count()
        );
    }

    #[test]
    fn dff_constraint_arcs_gated_and_setup_hold_under_declared_clock() {
        // Constraint generation is gated on the per-cell opt-in, so gating is exercised by two cells
        // rather than an emit-time toggle. Off: no `constraint_arcs`, so none are generated or emitted.
        let off_cell = analyse(
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
        let off = cell_arcs_tcl(&off_cell, ArcsTclOptions::default());
        assert!(!off.contains("-type setup"));
        assert!(!off.contains("-type hold"));

        // On: the same DFF with `constraint_arcs = true` generates separate setup and hold blocks of D
        // w.r.t. CLK. With CLK declared a clock the CLK/D constraint is a setup/hold, so no non_seq is
        // produced for the pair.
        let on_cell = analyse(
            r#"
[[cell]]
name = "DFF"
inputs = ["CLK", "D"]
clock = ["CLK"]
constraint_arcs = true
[cell.internal]
M = "!CLK*D + CLK*M"
[cell.outputs]
Q = "CLK*M + !CLK*Q"
"#,
        );
        let on = cell_arcs_tcl(&on_cell, ArcsTclOptions::default());
        eprintln!("{on}");
        assert!(on.contains("-type setup \\"));
        assert!(on.contains("-type hold \\"));
        assert!(on.contains("-related_pin CLK"));
        assert!(on.contains("-pin D"));
        assert!(!on.contains("non_seq"));
    }

    /// The same two-latch DFF, with edge collapse explicitly suppressed (`no_edge_collapse = true`) —
    /// preserves the pre-collapse two-latch coverage: every delay arc on Q stays `-type combinational`,
    /// none is re-labelled `-type edge`.
    #[test]
    fn dff_no_edge_collapse_keeps_combinational_type_on_q_arcs() {
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
        assert!(cell.edge.registers.is_empty());
        let tcl = cell_arcs_tcl(&cell, ArcsTclOptions::default());
        eprintln!("{tcl}");
        assert_eq!(tcl.matches("-type edge").count(), 0);
        assert!(tcl.contains("-pin Q"));
    }

    /// The same two-latch DFF under default (on) edge collapse: the CLK-related delay arc(s) on Q are
    /// re-labelled `-type edge`; the D-related hidden arc and the setup/hold constraint blocks are
    /// unaffected by the re-label.
    #[test]
    fn dff_default_collapse_marks_clk_to_q_arcs_edge_type() {
        let cell = analyse(
            r#"
[[cell]]
name = "DFF"
inputs = ["CLK", "D"]
clock = ["CLK"]
constraint_arcs = true
[cell.internal]
M = "!CLK*D + CLK*M"
[cell.outputs]
Q = "CLK*M + !CLK*Q"
"#,
        );
        assert!(!cell.edge.registers.is_empty());
        // The recognised register captures on the rising clock seam (transparent-high slave).
        assert!(cell
            .edge
            .registers
            .iter()
            .all(|r| r.captures.iter().all(|(_, e, _)| *e == Edge::Rise)));
        let tcl = cell_arcs_tcl(&cell, ArcsTclOptions::default());
        eprintln!("{tcl}");
        assert!(tcl.matches("-type edge").count() >= 1);
        // A CLK-related, Q-pinned delay arc is `-type edge` only on the register's *capturing* clock
        // edge (CLK rising here). An arc on the opposite (falling) clock edge is level behaviour and
        // must stay `-type combinational`. The vector renders CLK first (pinlist {CLK D Q}): `R` is the
        // capturing edge, `F` the non-capturing one.
        for frag in tcl.split("define_arc") {
            if !(frag.contains("-pin Q") && frag.contains("-related_pin CLK")) {
                continue;
            }
            let clk_field = frag
                .lines()
                .find(|l| l.contains("-vector"))
                .and_then(|l| l.split('{').nth(1))
                .and_then(|v| v.split_whitespace().next())
                .expect("delay arc renders a CLK vector field");
            if clk_field == "R" {
                assert!(
                    frag.contains("-type edge"),
                    "capturing-edge CLK->Q arc: {frag}"
                );
                assert!(!frag.contains("-type combinational"));
            } else {
                assert!(
                    frag.contains("-type combinational"),
                    "opposite-edge CLK->Q arc must stay combinational: {frag}"
                );
                assert!(!frag.contains("-type edge"));
            }
        }
        // The D-related hidden arc(s) are untouched: still `-type hidden`, never `-type edge`.
        for frag in tcl.split("define_arc") {
            if frag.contains("-type hidden") {
                assert!(!frag.contains("-type edge"));
            }
        }
        // Setup/hold constraint blocks are unaffected by the re-label.
        assert!(tcl.contains("-type setup \\"));
        assert!(tcl.contains("-type hold \\"));
    }

    /// The ICM interlock's registers are all internal nodes (never a Liberty output), so it has no
    /// output arcs to re-label — its Tcl carries zero `-type edge` blocks even though it recognises
    /// edge registers.
    #[test]
    fn icm_internal_registers_emit_zero_edge_type_arcs() {
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
        assert!(!cell.edge.registers.is_empty());
        let tcl = cell_arcs_tcl(&cell, ArcsTclOptions::default());
        eprintln!("{tcl}");
        assert_eq!(tcl.matches("-type edge").count(), 0);
    }

    /// A dual-edge mux-DET: two complementary-phase master latches muxed straight into the output, with
    /// no slave stage. `Q` captures `D` on both the rising and falling edge of `CLK`, so both CLK-related
    /// `Q` delay arcs (rise and fall) are re-labelled `-type edge`, while the `D`-related arcs stay
    /// `-type combinational`.
    #[test]
    fn det_dual_edge_marks_both_clk_to_q_arcs_edge_type() {
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
        assert_eq!(cell.edge.registers.len(), 1);
        assert_eq!(
            cell.edge.registers[0].captures.len(),
            2,
            "dual-edge register"
        );
        let tcl = cell_arcs_tcl(&cell, ArcsTclOptions::default());
        eprintln!("{tcl}");
        // Every CLK-related, Q-pinned delay arc is `-type edge` -- both the rising and the falling
        // capture -- with no combinational survivor among them (held-context duplicates under `-when`
        // notwithstanding).
        let mut saw_rise = false;
        let mut saw_fall = false;
        for frag in tcl.split("define_arc") {
            if !(frag.contains("-pin Q") && frag.contains("-related_pin CLK")) {
                continue;
            }
            assert!(frag.contains("-type edge \\"), "CLK->Q arc: {frag}");
            assert!(!frag.contains("-type combinational"));
            let clk_field = frag
                .lines()
                .find(|l| l.contains("-vector"))
                .and_then(|l| l.split('{').nth(1))
                .and_then(|v| v.split_whitespace().next())
                .expect("delay arc renders a CLK vector field");
            match clk_field {
                "R" => saw_rise = true,
                "F" => saw_fall = true,
                other => panic!("unexpected CLK vector field: {other}"),
            }
        }
        assert!(
            saw_rise && saw_fall,
            "both rise and fall CLK->Q arcs present"
        );
        // Data (D-related) arcs stay combinational -- toggling D alone never changes Q here (Q is a
        // function of CLK and the internal latches only), so D's arcs are all `-type hidden`, never
        // re-labelled edge.
        for frag in tcl.split("define_arc") {
            if frag.contains("-type hidden") {
                assert!(!frag.contains("-type edge"));
            }
        }
    }

    /// A lone level-sensitive latch whose ENABLE is a declared clock, driving an output node. A single
    /// latch is not a master-slave pair, so nothing collapses (no edge registers) and its level
    /// enable->output arcs stay `-type combinational` — never `-type edge` — even though the enable is a
    /// declared clock. Confirms a declared-clock enable does not, on its own, collapse a level latch or
    /// emit an edge arc: recognition finds no master-slave pair, so `edge_registers` is empty.
    #[test]
    fn latch_with_declared_clock_enable_emits_zero_edge_type_arcs() {
        let cell = analyse(
            r#"
[[cell]]
name = "DLAT"
inputs = ["EN", "D"]
clock = ["EN"]
[cell.outputs]
Q = "EN*D + !EN*Q"
"#,
        );
        assert!(cell.edge.registers.is_empty());
        let tcl = cell_arcs_tcl(&cell, ArcsTclOptions::default());
        eprintln!("{tcl}");
        assert_eq!(tcl.matches("-type edge").count(), 0);
        assert!(tcl.contains("-pin Q"));
    }

    #[test]
    fn mutex_emits_non_seq_constraint_arcs_when_enabled() {
        let cell = analyse(
            r#"
[[cell]]
name = "MUT"
inputs = ["A", "B"]
constraint_arcs = true
[cell.outputs]
Qa = "!Qb * A"
Qb = "!Qa * B"
"#,
        );
        let on = cell_arcs_tcl(&cell, ArcsTclOptions::default());
        eprintln!("{on}");
        assert!(on.contains("-type non_seq_setup \\"));
        assert!(on.contains("-type non_seq_hold \\"));
        // Both request pins appear as related/pin of the constraint.
        assert!(on.contains("-related_pin A"));
        assert!(on.contains("-pin B"));
    }

    #[test]
    fn mutex_emits_oscillation_comment_and_input_only_related_pins() {
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
        // Oscillation documented up front.
        assert!(tcl.contains("# oscillation: A*B risks metastability"));
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
    fn c_element_emits_leakage_states() {
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
        eprintln!("{tcl}");
        assert_eq!(tcl.matches("define_leakage").count(), 2);
        assert!(tcl.contains("define_leakage -when \"A*B*Q\" { C2 }"));
        assert!(tcl.contains("define_leakage -when \"!A*!B*!Q\" { C2 }"));
    }

    #[test]
    fn and2_emits_leakage_states() {
        let cell = analyse(
            r#"
[[cell]]
name = "AND2"
inputs = ["A", "B"]
[cell.outputs]
Y = "A*B"
"#,
        );
        let tcl = cell_arcs_tcl(&cell, ArcsTclOptions::default());
        eprintln!("{tcl}");
        assert_eq!(tcl.matches("define_leakage").count(), 4);
        assert!(tcl.contains("-when \"A*B*Y\""));
        assert!(tcl.contains("-when \"!A*!B*!Y\""));
    }

    #[test]
    fn no_leakage_option_suppresses_leakage() {
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
                emit_leakage: false,
                ..Default::default()
            },
        );
        assert_eq!(off.matches("define_leakage").count(), 0);
    }

    #[test]
    fn leakage_section_follows_hidden_arcs() {
        let cell = analyse(
            r#"
[[cell]]
name = "AND2"
inputs = ["A", "B"]
[cell.outputs]
Y = "A*B"
"#,
        );
        let tcl = cell_arcs_tcl(&cell, ArcsTclOptions::default());
        let last_hidden = tcl.rfind("-type hidden").expect("hidden arc present");
        let first_leakage = tcl.find("define_leakage").expect("leakage present");
        assert!(first_leakage > last_hidden);
    }

    #[test]
    fn multi_name_cell_fans_names_into_one_trailer() {
        // A cell with several names emits one braced list carrying all of them per arc trailer and
        // per define_leakage — not one arc per name.
        let cell = analyse(
            r#"
[[cell]]
name = ["C2A", "C2B"]
inputs = ["A", "B"]
[cell.outputs]
Q = "A*B + Q*(A+B)"
"#,
        );
        let single = analyse(
            r#"
[[cell]]
name = "C2"
inputs = ["A", "B"]
[cell.outputs]
Q = "A*B + Q*(A+B)"
"#,
        );
        let tcl = cell_arcs_tcl(&cell, ArcsTclOptions::default());
        let single_tcl = cell_arcs_tcl(&single, ArcsTclOptions::default());
        eprintln!("{tcl}");
        assert!(single_tcl.contains("{ C2 }"));
        assert!(tcl.contains("{ C2A C2B }"));
        assert!(!tcl.contains("{ C2A }"));
        assert!(!tcl.contains("{ C2B }"));
        assert!(tcl.contains("define_leakage -when \"A*B*Q\" { C2A C2B }"));
        // Same arc count regardless of how many names the cell carries — one arc per transition, a
        // single trailer names both.
        assert_eq!(
            tcl.matches("define_arc").count(),
            single_tcl.matches("define_arc").count()
        );
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

    /// Four shapes that recognise NO edge register even under default (on) collapse: a single latch, a
    /// gated (self-referencing) latch, a master/slave pair split across two DIFFERENT declared clocks
    /// (a genuine two-clock master-slave: a node changing on >=2 declared clocks is not annotated), and
    /// a two-latch DFF whose clock is never declared. The exposed-master DFF (EMDFF) is NO LONGER here
    /// -- its slave Q is now a recognised edge register (see `emdff_marks_only_the_slave_qs_clk_arc_edge_type`),
    /// matching the sibling emitters' fixture suites.
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
    fn non_collapsible_suite_tcl_matches_the_no_edge_collapse_flag() {
        // Zero `-type edge` blocks, whether the flag is left off (default collapse, a no-op on these
        // shapes) or forced on -- and the two runs emit byte-identical Tcl.
        for src in NON_COLLAPSIBLE {
            let (default, forced) = analyse_both(src);
            let tcl_default = cell_arcs_tcl(&default, ArcsTclOptions::default());
            let tcl_forced = cell_arcs_tcl(&forced, ArcsTclOptions::default());
            assert_eq!(tcl_default.matches("-type edge").count(), 0);
            assert_eq!(tcl_forced.matches("-type edge").count(), 0);
            assert_eq!(tcl_default, tcl_forced);
        }
    }

    /// The exposed-master DFF: the behavioural pass recognises the slave `Q` as a rising-edge register
    /// while the declared-output master `M` survives as a level node. `Q`'s CLK-related delay arc is
    /// re-labelled `-type edge` (default collapse, no TOML opt-out); `M`'s own arcs are unaffected --
    /// never re-labelled edge.
    #[test]
    fn emdff_marks_only_the_slave_qs_clk_arc_edge_type() {
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
        assert!(!cell.edge.registers.is_empty());
        let tcl = cell_arcs_tcl(&cell, ArcsTclOptions::default());
        eprintln!("{tcl}");
        assert!(tcl.matches("-type edge").count() >= 1);
        for frag in tcl.split("define_arc") {
            if frag.contains("-type edge") {
                assert!(frag.contains("-pin Q"), "edge type only on Q: {frag}");
            }
            if frag.contains("-pin M") {
                assert!(!frag.contains("-type edge"), "M stays non-edge: {frag}");
            }
        }
    }

    #[test]
    fn dff_opt_out_restores_combinational_type_via_either_switch() {
        // The two-latch DFF, opted out directly (`no_edge_collapse = true` in the TOML) versus opted
        // out via the CLI-flag-equivalent blanket mutation over the whole spec: both switches restore
        // the SAME Tcl -- zero `-type edge` blocks.
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

        let tcl_direct = cell_arcs_tcl(&direct, ArcsTclOptions::default());
        let tcl_via_flag = cell_arcs_tcl(&via_flag, ArcsTclOptions::default());
        for tcl in [&tcl_direct, &tcl_via_flag] {
            assert_eq!(tcl.matches("-type edge").count(), 0);
            assert!(tcl.contains("-pin Q"));
        }
        assert_eq!(tcl_direct, tcl_via_flag);
    }
}
