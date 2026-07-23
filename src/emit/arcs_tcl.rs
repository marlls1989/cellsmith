//! Emit Cadence Liberate `define_arc` blocks for a cell's transition arcs.
//!
//! The layout places `-type` first on rise arcs and after the prevector on fall arcs, with pins
//! emitted in declaration order.
//!
//! Arc typing follows the per-arc labels in [`crate::logic::edge`], which are SOURCED FROM the arc
//! pipeline itself: each emitted delay arc looks up its own `(output, related clock, clock direction)`
//! key in [`crate::logic::edge::EdgeArcs::labels`]. A labelled arc is a clock edge after which the value
//! holds independently of the clock level, and Liberate has one token for it: `-type edge`. An
//! unlabelled arc — a data change propagating through an already-transparent latch, or a clock acting by
//! its level rather than being held — stays `-type combinational`, and a declared-async related pin
//! takes precedence with `-type async`.

use std::collections::{BTreeMap, BTreeSet};

use espresso_logic::Symbol;

use crate::logic::arcs::{Arc, Edge, HiddenArc};
use crate::logic::assignment;
use crate::logic::confluence::{Constraint, ConstraintKind};
use crate::logic::hazard::Oscillation;
use crate::logic::leakage::LeakageState;
use crate::logic::literal_product;
use crate::model::{AnalysedCell, ArcClass};

/// Knobs for the arc emitter.
#[derive(Debug, Clone, Copy)]
pub struct ArcsTclOptions {
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
    // `-when` suppression and dedup are ONE behaviour, selected PER ARC CLASS by the cell's resolved
    // `no_when` set. A selected class drops its arcs' `-when` line (in `format_arc`/`format_hidden_arc`)
    // AND collapses the arcs that become indistinguishable once `-when` is gone — same
    // output/related/type/vector, differing only by prevector or internal state — keeping the member with
    // the shortest prevector (see `selected`). An unselected class keeps every `-when` and emits every
    // arc, exactly as before.
    for arc in selected(
        &cell.arcs,
        cell.no_when.contains(ArcClass::Transition),
        |arc| {
            (
                arc.output.clone(),
                arc.related.clone(),
                arc_type_token(cell, arc),
                vector_str(cell, arc),
            )
        },
        |arc| arc.prevector.len(),
    ) {
        out.push_str(&format_arc(cell, arc));
    }
    if opts.emit_internal {
        for h in selected(
            &cell.hidden_arcs,
            cell.no_when.contains(ArcClass::Hidden),
            |h| (h.pin.clone(), hidden_vector_str(cell, h)),
            |h| h.prevector.len(),
        ) {
            out.push_str(&format_hidden_arc(cell, h));
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

/// The arcs of one class to emit. With `-when` kept (`!dedup`), every arc. With `-when` suppressed
/// (`dedup`), one arc per emitted-block identity: `key` is everything the block renders EXCEPT its
/// `-prevector` / `-prevector_pinlist` lines, so two arcs collide exactly when Liberate cannot tell them
/// apart once `-when` is gone. The kept member is the one with the SHORTEST prevector; ties keep the
/// first in `items` order. Survivors keep `items` order.
///
/// DETERMINISM IS A HARD REQUIREMENT: the collapse threads through `BTreeMap`/`BTreeSet`, never a
/// randomly-seeded hash map — a per-process random hash seed would reorder the emitted `.tcl` across
/// separate runs on the identical spec. `cell.arcs` / `cell.hidden_arcs` arrive in
/// `BTreeMap::into_values()` order (src/logic/arcs.rs) and `render`'s
/// `par_iter().map().collect::<Vec<_>>().concat()` (src/main.rs) preserves index order, so first-wins
/// over that order is reproducible run to run.
fn selected<T, K: Ord>(
    items: &[T],
    dedup: bool,
    key: impl Fn(&T) -> K,
    prevector_len: impl Fn(&T) -> usize,
) -> Vec<&T> {
    if !dedup {
        return items.iter().collect();
    }
    // Winning index per key: the earliest STRICTLY-shortest prevector. Replace only on a strictly shorter
    // prevector, so a length tie keeps the earlier index.
    let mut winner: BTreeMap<K, (usize /* len */, usize /* index */)> = BTreeMap::new();
    for (i, item) in items.iter().enumerate() {
        let len = prevector_len(item);
        winner
            .entry(key(item))
            .and_modify(|best| {
                if len < best.0 {
                    *best = (len, i);
                }
            })
            .or_insert((len, i));
    }
    let winners: BTreeSet<usize> = winner.into_values().map(|(_, i)| i).collect();
    items
        .iter()
        .enumerate()
        .filter(|(i, _)| winners.contains(i))
        .map(|(_, t)| t)
        .collect()
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
                if *held
                    .get(input)
                    .expect("every input has a held value in the constraint prevector")
                {
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

/// The edge the arc's `related` clock pin makes, read from its value in the end state — the same
/// derivation the vector uses to render its `R`/`F`. `Rise` when the clock settles high, `Fall` when it
/// settles low. Together with the output and related pin it is the arc's identity in
/// [`crate::logic::edge::EdgeArcs::labels`], the per-arc label map the classifier sourced from these
/// same pipeline arcs.
fn related_edge(arc: &Arc) -> Edge {
    if *assignment(&arc.end)
        .get(&arc.related)
        .expect("the arc's related clock pin is assigned in its end state")
    {
        Edge::Rise
    } else {
        Edge::Fall
    }
}

/// The `-type` token for a transition arc: `async` for a declared-async related pin, else `edge` when the
/// arc's FULL identity `(output, related, direction, machine start)` is labelled a clock-edge timing arc
/// in [`crate::logic::edge::EdgeArcs::labels`], else `combinational`. There is ONE edge category, so two
/// firings that differ only in internal state can type differently. The dedup key and `format_arc`'s
/// `-type` line share this ONE source.
fn arc_type_token(cell: &AnalysedCell, arc: &Arc) -> &'static str {
    let is_edge = !arc.is_async
        && cell.edge.labels.contains(&(
            arc.output.clone(),
            arc.related.clone(),
            related_edge(arc),
            arc.start.clone(),
        ));
    if arc.is_async {
        "async"
    } else if is_edge {
        "edge"
    } else {
        "combinational"
    }
}

fn format_arc(cell: &AnalysedCell, arc: &Arc) -> String {
    let type_line = format!("\t-type {} \\\n", arc_type_token(cell, arc));
    let prevector_pinlist = format!("\t-prevector_pinlist {{{}}} \\\n", cell.inputs.join(" "));
    let prevector = format!(
        "\t-prevector {{{}}} \\\n",
        prevector_str(cell, &arc.prevector)
    );
    let pinlist = format!("\t-pinlist {{{}}} \\\n", pinlist_str(cell));
    let vector = format!("\t-vector {{{}}} \\\n", vector_str(cell, arc));
    let when = match (
        cell.no_when.contains(ArcClass::Transition),
        when_str(&arc.end, &arc.related),
    ) {
        (false, Some(w)) => format!("\t-when \"{w}\" \\\n"),
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

/// The measured hidden-arc vector: the toggled `pin` as its `R`/`F` edge, every other input at its held
/// `1`/`0` value in the end state, and every output pinned at its held `1`/`0` value (never `X` — a hidden
/// arc measures no output transition). Mirrors [`vector_str`] for [`Arc`]; the dedup key and
/// `format_hidden_arc`'s `-vector` line share this ONE source.
fn hidden_vector_str(cell: &AnalysedCell, h: &HiddenArc) -> String {
    let held: BTreeMap<&str, bool> = h.outputs.iter().map(|(s, b)| (s.as_str(), *b)).collect();
    let end = assignment(&h.end);
    vector(
        cell,
        |input| {
            if input == h.pin.as_str() {
                h.edge.rf().to_string()
            } else {
                if *end
                    .get(input)
                    .expect("every input is assigned in the hidden arc's end state")
                {
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
    )
}

/// A hidden (whole-cell internal-power) `define_arc` of `-type hidden`: the toggled input drives an
/// `R`/`F` edge, every other input sits at its held value in the end state, and every output is pinned
/// at its held `1`/`0` value (never `X` — a hidden arc measures no output transition). Unlike transition
/// arcs there is no `-related_pin`, and `-type hidden` always leads regardless of edge direction.
fn format_hidden_arc(cell: &AnalysedCell, h: &HiddenArc) -> String {
    let vec = hidden_vector_str(cell, h);

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
    if let (false, Some(w)) = (cell.no_when.contains(ArcClass::Hidden), hidden_when_str(h)) {
        s.push_str(&format!("\t-when \"{w}\" \\\n"));
    }
    s.push_str(&format!("\t-pin {} \\\n", h.pin.as_str()));
    s.push_str(&format!("\t{}\n", name_block(cell)));
    s.push('\n');
    s
}

pub(crate) fn pinlist_str(cell: &AnalysedCell) -> String {
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
                    if *a
                        .get(i)
                        .expect("every input is assigned in each prevector step")
                    {
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
            let value = *end
                .get(input)
                .expect("every input is assigned in the arc's end state");
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
    fn hidden_arcs_dedup_per_vector_without_when() {
        let cell = analyse(
            r#"
[[cell]]
name = "AND2"
inputs = ["A", "B"]
[cell.outputs]
Y = "A*B"
"#,
        );
        // A second cell identical but for `no_when = true`, which suppresses every arc class's `-when`
        // and deduplicates its arcs down to one per emitted vector.
        let suppressed = analyse(
            r#"
[[cell]]
name = "AND2"
inputs = ["A", "B"]
no_when = true
[cell.outputs]
Y = "A*B"
"#,
        );
        let without_when = cell_arcs_tcl(
            &suppressed,
            // define_leakage is inherently -when-conditioned; disabled here to isolate arc -when
            // suppression.
            ArcsTclOptions {
                emit_leakage: false,
                ..Default::default()
            },
        );
        let with_when = cell_arcs_tcl(&cell, ArcsTclOptions::default());
        assert!(!without_when.contains("-when"));
        // Dedup collapses same-vector hidden siblings, so the suppressed count can only shrink.
        assert!(
            without_when.matches("-type hidden").count()
                <= with_when.matches("-type hidden").count()
        );
        // No two surviving `-type hidden` blocks share a `-vector` line: the class collapsed to one arc
        // per emitted vector.
        let hidden_vectors: Vec<&str> = without_when
            .split("define_arc")
            .filter(|b| b.contains("-type hidden"))
            .map(|b| {
                b.lines()
                    .find(|l| l.contains("-vector"))
                    .expect("a hidden block renders a -vector")
            })
            .collect();
        let unique: BTreeSet<&str> = hidden_vectors.iter().copied().collect();
        assert_eq!(
            hidden_vectors.len(),
            unique.len(),
            "no two surviving hidden blocks share a -vector line"
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
        // The same cell but for `no_when = true`, which suppresses every arc class's `-when`.
        let suppressed = analyse(
            r#"
[[cell]]
name = "C2"
inputs = ["A", "B"]
no_when = true
[cell.outputs]
Q = "A*B + Q*(A+B)"
"#,
        );
        let off = cell_arcs_tcl(
            &suppressed,
            // define_leakage is inherently -when-conditioned; disabled here to isolate arc -when
            // suppression.
            ArcsTclOptions {
                emit_leakage: false,
                ..Default::default()
            },
        );
        let on = cell_arcs_tcl(&cell, ArcsTclOptions::default());
        assert!(!off.contains("-when"));
        assert!(on.contains("-when"));
    }

    #[test]
    fn no_when_keeps_arcs_whose_vectors_differ() {
        // A 3-input majority gate has NO internal state, so every held-input context is already visible
        // in `-vector` as 0/1 on the other two inputs. Each candidate arc therefore gets a distinct dedup
        // key and nothing collides: the selected class drops its `-when` lines but collapses no arc. Both
        // modes carry the same `define_arc` count and differ solely by the absent `-when` lines.
        let cell = analyse(
            r#"
[[cell]]
name = "MAJ3"
inputs = ["A", "B", "C"]
[cell.outputs]
Y = "A*B + B*C + A*C"
"#,
        );
        // The same cell but for `no_when = true`, which selects every arc class for suppression + dedup.
        let suppressed = analyse(
            r#"
[[cell]]
name = "MAJ3"
inputs = ["A", "B", "C"]
no_when = true
[cell.outputs]
Y = "A*B + B*C + A*C"
"#,
        );
        // Isolate arc `-when` suppression from `define_leakage`, which is inherently `-when`-conditioned.
        let opts = ArcsTclOptions {
            emit_leakage: false,
            ..Default::default()
        };
        let with_when = cell_arcs_tcl(&cell, opts);
        let without_when = cell_arcs_tcl(&suppressed, opts);
        assert_eq!(
            with_when.matches("define_arc").count(),
            without_when.matches("define_arc").count(),
            "no MAJ3 arc collides, so every arc emits in both modes"
        );
        assert!(with_when.contains("-when"));
        assert!(!without_when.contains("-when"));
        // Dropping the `-when` lines from the default output reproduces the suppressed output exactly.
        let strip_when = |s: &str| {
            s.lines()
                .filter(|l| !l.contains("-when"))
                .collect::<Vec<_>>()
                .join("\n")
        };
        assert_eq!(strip_when(&with_when), strip_when(&without_when));
    }

    // ---- Dedup contract: shortest prevector, class selectivity, determinism ----

    /// A two-output cell that exhibits a transition-arc collision: `Y = A` is a plain rise/fall, and `Z`
    /// is a C-element whose held value renders as `X` in `Y`'s vector. With `B = 1` both `Z = 0` and
    /// `Z = 1` are reachable settled states, so the `A`-rise → `Y`-rise arc is measured from both and the
    /// two blocks are identical apart from their prevectors — a same-key collision once `-when` is gone.
    const TWO: &str = r#"
[[cell]]
name = "TWO"
inputs = ["A", "B"]
[cell.outputs]
Y = "A"
Z = "A*B + Z*(A+B)"
"#;

    /// A 3-input majority gate: stateless, so every held-input context is already visible in `-vector`
    /// and no two candidate arcs collide.
    const MAJ3: &str = r#"
[[cell]]
name = "MAJ3"
inputs = ["A", "B", "C"]
[cell.outputs]
Y = "A*B + B*C + A*C"
"#;

    /// `src` with a `no_when = <value>` key spliced into its `[[cell]]` table (just before the
    /// `[cell.outputs]` sub-table). `value` is the raw TOML: `true`, `"hidden"`, `"transition"`.
    fn no_when_variant(src: &str, value: &str) -> String {
        src.replace(
            "[cell.outputs]",
            &format!("no_when = {value}\n[cell.outputs]"),
        )
    }

    /// Isolate arc `-when`/dedup from `define_leakage`, which is inherently `-when`-conditioned.
    const NO_LEAKAGE: ArcsTclOptions = ArcsTclOptions {
        emit_internal: true,
        emit_leakage: false,
    };

    /// COLLISION: the transition dedup collapses a genuine same-key collision to a single block. The
    /// premise — two A→Y blocks sharing one `-vector` on the DEFAULT output — is asserted first, so a
    /// non-colliding fixture fails loudly rather than testing nothing.
    #[test]
    fn no_when_transition_collapses_a_collision_to_one_block() {
        // Every A→Y block's `-vector` line, in emission order.
        let ay_vectors = |tcl: &str| -> Vec<String> {
            tcl.split("define_arc")
                .filter(|b| b.contains("-related_pin A") && b.contains("-pin Y \\"))
                .map(|b| {
                    b.lines()
                        .find(|l| l.contains("-vector"))
                        .expect("a transition block renders a -vector")
                        .trim()
                        .to_string()
                })
                .collect()
        };

        // PREMISE on the default output (empty no_when): at least two A→Y blocks share a -vector.
        let default = cell_arcs_tcl(&analyse(TWO), NO_LEAKAGE);
        eprintln!("{default}");
        let default_vectors = ay_vectors(&default);
        let shared = default_vectors
            .iter()
            .find(|v| default_vectors.iter().filter(|w| w == v).count() >= 2)
            .cloned()
            .expect("premise: two A→Y blocks must share a -vector on the default output");

        // With the transition class selected, exactly one block with that shared -vector survives.
        let deduped = cell_arcs_tcl(
            &analyse(&no_when_variant(TWO, "\"transition\"")),
            NO_LEAKAGE,
        );
        let survivors = ay_vectors(&deduped)
            .iter()
            .filter(|v| **v == shared)
            .count();
        assert_eq!(survivors, 1, "the colliding A→Y blocks collapse to one");
    }

    /// SHORTEST PREVECTOR: the surviving member of a collapsed collision keeps the shortest prevector.
    /// The minimum is read FROM `cell.arcs` (never hardcoded), so a length tie cannot make the assertion
    /// vacuous.
    #[test]
    fn no_when_transition_keeps_the_shortest_prevector() {
        let cell = analyse(TWO);
        // Group the A→Y arcs by their emitted transition vector; find the colliding group.
        let mut groups: BTreeMap<String, Vec<&Arc>> = BTreeMap::new();
        for a in cell
            .arcs
            .iter()
            .filter(|a| a.output == "Y" && a.related == "A")
        {
            groups.entry(vector_str(&cell, a)).or_default().push(a);
        }
        let (shared_vec, group) = groups
            .iter()
            .find(|(_, g)| g.len() >= 2)
            .expect("premise: a colliding A→Y group");
        let min_len = group
            .iter()
            .map(|a| a.prevector.len())
            .min()
            .expect("a non-empty group");

        // In the deduped output, the surviving block for that vector carries exactly `min_len` steps.
        let deduped = cell_arcs_tcl(
            &analyse(&no_when_variant(TWO, "\"transition\"")),
            NO_LEAKAGE,
        );
        let block = deduped
            .split("define_arc")
            .find(|b| {
                b.contains("-related_pin A")
                    && b.contains("-pin Y \\")
                    && b.lines()
                        .any(|l| l.contains("-vector") && l.contains(shared_vec.as_str()))
            })
            .expect("the surviving A→Y block");
        let steps = block
            .lines()
            .find(|l| l.trim_start().starts_with("-prevector {"))
            .and_then(|l| l.split('{').nth(1))
            .and_then(|s| s.split('}').next())
            .expect("the surviving block renders a -prevector")
            .split_whitespace()
            .count();
        assert_eq!(steps, min_len, "the shortest-prevector member survives");
    }

    /// CONTRACT: the class-scoped dedup count equals the number of DISTINCT emitted keys — fixture
    /// independent, and the primary pin on the hidden side where a hand-picked collision is not reliably
    /// constructible.
    #[test]
    fn no_when_dedup_count_equals_distinct_keys() {
        for src in [TWO, MAJ3] {
            // Transition side: one block per distinct (output, related, type, vector) key.
            let t_cell = analyse(&no_when_variant(src, "\"transition\""));
            let tcl = cell_arcs_tcl(&t_cell, NO_LEAKAGE);
            let non_hidden = tcl
                .split("define_arc")
                .skip(1)
                .filter(|b| !b.contains("-type hidden"))
                .count();
            let distinct_t: BTreeSet<(Symbol, Symbol, &str, String)> = t_cell
                .arcs
                .iter()
                .map(|a| {
                    (
                        a.output.clone(),
                        a.related.clone(),
                        arc_type_token(&t_cell, a),
                        vector_str(&t_cell, a),
                    )
                })
                .collect();
            assert_eq!(
                non_hidden,
                distinct_t.len(),
                "transition block count equals distinct transition keys"
            );

            // Hidden side: one block per distinct (pin, hidden vector) key.
            let h_cell = analyse(&no_when_variant(src, "\"hidden\""));
            let tcl = cell_arcs_tcl(&h_cell, NO_LEAKAGE);
            let hidden = tcl.matches("-type hidden").count();
            let distinct_h: BTreeSet<(Symbol, String)> = h_cell
                .hidden_arcs
                .iter()
                .map(|h| (h.pin.clone(), hidden_vector_str(&h_cell, h)))
                .collect();
            assert_eq!(
                hidden,
                distinct_h.len(),
                "hidden block count equals distinct hidden keys"
            );
        }
    }

    /// DISTINCT VECTORS SURVIVE: two hidden arcs whose vectors DIFFER both survive dedup — only their
    /// `-when` disappears. The transparent-high D-latch holds Q at 0 or 1 across its two D-toggle
    /// contexts, so the held Q makes the two `-vector` lines distinct.
    #[test]
    fn no_when_hidden_keeps_distinct_vectors() {
        const DLAT: &str = r#"
[[cell]]
name = "DLAT"
inputs = ["E", "D"]
[cell.outputs]
Q = "E*D + !E*Q"
"#;
        let cell = analyse(&no_when_variant(DLAT, "\"hidden\""));
        let tcl = cell_arcs_tcl(&cell, NO_LEAKAGE);
        eprintln!("{tcl}");
        let d_hidden: Vec<&str> = tcl
            .split("define_arc")
            .filter(|b| b.contains("-type hidden") && b.contains("-pin D \\"))
            .collect();
        // Both hold-context D hidden arcs survive: their held Q makes the vectors differ.
        let vectors: BTreeSet<&str> = d_hidden
            .iter()
            .map(|b| {
                b.lines()
                    .find(|l| l.contains("-vector"))
                    .expect("a hidden block renders a -vector")
                    .trim()
            })
            .collect();
        assert!(
            vectors.len() >= 2,
            "distinct-Q D hidden arcs keep their separate vectors, so both survive"
        );
        // Only the hidden class's `-when` disappears.
        let hidden_with_when = tcl
            .split("define_arc")
            .filter(|b| {
                b.contains("-type hidden") && b.lines().any(|l| l.trim_start().starts_with("-when"))
            })
            .count();
        assert_eq!(hidden_with_when, 0, "the hidden -when lines are suppressed");
    }

    /// SELECTIVITY: `--no-when=hidden` leaves the transition class untouched (same block count, every
    /// `-when`), and its mirror `--no-when=transition` leaves the hidden class untouched.
    #[test]
    fn no_when_hidden_only_leaves_transition_arcs_untouched() {
        let default = cell_arcs_tcl(&analyse(TWO), NO_LEAKAGE);

        let non_hidden = |tcl: &str| {
            tcl.split("define_arc")
                .skip(1)
                .filter(|b| !b.contains("-type hidden"))
                .count()
        };
        let transition_when = |tcl: &str| {
            tcl.split("define_arc")
                .skip(1)
                .filter(|b| {
                    !b.contains("-type hidden")
                        && b.lines().any(|l| l.trim_start().starts_with("-when"))
                })
                .count()
        };
        let hidden_when = |tcl: &str| {
            tcl.split("define_arc")
                .filter(|b| {
                    b.contains("-type hidden")
                        && b.lines().any(|l| l.trim_start().starts_with("-when"))
                })
                .count()
        };
        let hidden_count = |tcl: &str| tcl.matches("-type hidden").count();

        // --no-when=hidden: the transition class is untouched.
        let hidden_off = cell_arcs_tcl(&analyse(&no_when_variant(TWO, "\"hidden\"")), NO_LEAKAGE);
        assert_eq!(
            non_hidden(&hidden_off),
            non_hidden(&default),
            "transition block count is unchanged by --no-when=hidden"
        );
        assert_eq!(
            transition_when(&hidden_off),
            transition_when(&default),
            "every transition -when survives --no-when=hidden"
        );
        assert!(
            transition_when(&default) >= 1,
            "transition -when lines are present to be preserved"
        );
        assert_eq!(hidden_when(&hidden_off), 0, "hidden -when is suppressed");
        assert!(
            hidden_when(&default) >= 1,
            "hidden -when lines are present by default"
        );

        // Mirror --no-when=transition: the hidden class is untouched.
        let transition_off = cell_arcs_tcl(
            &analyse(&no_when_variant(TWO, "\"transition\"")),
            NO_LEAKAGE,
        );
        assert_eq!(
            hidden_count(&transition_off),
            hidden_count(&default),
            "hidden block count is unchanged by --no-when=transition"
        );
        assert_eq!(
            hidden_when(&transition_off),
            hidden_when(&default),
            "every hidden -when survives --no-when=transition"
        );
        assert_eq!(
            transition_when(&transition_off),
            0,
            "transition -when is suppressed"
        );
    }

    /// DETERMINISM: dedup emission is BTree-ordered throughout, so repeated calls on the same cell are
    /// byte-identical (the cross-process guard lives in the CLI suite).
    #[test]
    fn no_when_dedup_is_deterministic_across_repeated_calls() {
        let cell = analyse(&no_when_variant(TWO, "true"));
        let first = cell_arcs_tcl(&cell, ArcsTclOptions::default());
        let second = cell_arcs_tcl(&cell, ArcsTclOptions::default());
        assert_eq!(
            first, second,
            "dedup emission is byte-identical across calls"
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
        assert!(cell.edge.captures.is_empty());
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
        assert!(!cell.edge.captures.is_empty());
        // The recognised register captures on the rising clock seam (transparent-high slave).
        assert!(cell
            .edge
            .captures
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

    /// The ICM interlock's capturing nodes are all internal (never a Liberty output), so it has no
    /// output arcs to re-label — its Tcl carries zero `-type edge` blocks even though captures are
    /// recognised on those internal nodes.
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
        assert!(!cell.edge.captures.is_empty());
        let tcl = cell_arcs_tcl(&cell, ArcsTclOptions::default());
        eprintln!("{tcl}");
        // `GCLK` acts by the level of both CLKA and CLKB, not a held transition, so neither produces an
        // edge arc.
        for frag in tcl.split("define_arc") {
            if frag.contains("-pin GCLK") && frag.contains("-related_pin CLK") {
                assert!(frag.contains("-type combinational \\"), "GCLK arc: {frag}");
            }
        }
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
        assert_eq!(cell.edge.captures.len(), 1);
        assert_eq!(
            cell.edge.captures[0].captures.len(),
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

    /// DCMUX: two independently-clocked masters merged into one output. Q collapses to a LEVEL model (its
    /// falls are combinational and the seam fixpoint empties its set), so Q is NOT an edge register, yet
    /// each clock's RISING Q delay arc still renders `-type edge` (generation at Q). Both clocks therefore
    /// carry an edge-labelled Q arc; the falls stay combinational.
    #[test]
    fn dcmux_marks_both_clocks_q_arcs_edge_type() {
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
        // Q is a level model, not an edge register -- the label lives on the delay arc, not a capture.
        assert!(!cell.edge.captures.iter().any(|r| r.node == "Q"));
        let tcl = cell_arcs_tcl(&cell, ArcsTclOptions::default());
        eprintln!("{tcl}");
        // Each clock's RISING Q delay arc is re-labelled edge.
        for clock in ["CLKA", "CLKB"] {
            let related = format!("-related_pin {clock}");
            let saw_edge = tcl.split("define_arc").any(|frag| {
                frag.contains("-pin Q") && frag.contains(&related) && frag.contains("-type edge \\")
            });
            assert!(saw_edge, "a {clock}-related Q rise arc must be -type edge");
        }
    }

    /// Hierarchical master-slave across two clocks (HPIPE): `Q` CAPTURES from CLKA on its rising edge and
    /// is RELEASED by CLKB on its falling edge (CLKB's fall opens the output latch, transmitting the M2
    /// value that changed while it was closed). The two categories are distinct internally but share the
    /// Liberate `-type edge` token, so BOTH clocks' Q arcs render `-type edge` on the SAME output node;
    /// no arc is dropped.
    #[test]
    fn hierarchical_second_clock_fall_alongside_rise_edge_type() {
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
        let q = cell.edge.captures.iter().find(|r| r.node == "Q").unwrap();
        assert!(q
            .captures
            .iter()
            .any(|(c, e, _)| c == "CLKA" && *e == Edge::Rise));
        // Q captures on its own CLKB FALLING edge (the master-slave reveal) alongside the CLKA capture.
        assert!(
            q.captures
                .iter()
                .any(|(c, e, _)| c == "CLKB" && *e == Edge::Fall),
            "Q captures on CLKB's falling (opening) edge"
        );
        assert!(
            cell.edge
                .labels
                .iter()
                .any(|(n, c, e, _)| n == "Q" && c == "CLKB" && *e == Edge::Fall),
            "Q's own latch opens on CLKB's falling edge (an edge arc): {:?}",
            cell.edge.labels
        );
        let tcl = cell_arcs_tcl(&cell, ArcsTclOptions::default());
        eprintln!("{tcl}");
        // The CLKA-rise Q delay arc is -type edge, CONDITIONED on CLKB's level (CLKB appears as a
        // level field in the vector, never as an R/F edge). Pinlist orders CLKA, CLKB, then D, Q.
        let field_of = |frag: &str, clock: &str| -> Option<String> {
            let idx = ["CLKA", "CLKB", "D", "Q"]
                .iter()
                .position(|p| *p == clock)?;
            frag.lines()
                .find(|l| l.contains("-vector"))
                .and_then(|l| l.split('{').nth(1))
                .and_then(|v| v.split_whitespace().nth(idx))
                .map(str::to_string)
        };
        let mut saw_a_rise_edge = false;
        let mut saw_b_fall_edge = false;
        for frag in tcl.split("define_arc") {
            if !frag.contains("-pin Q") {
                continue;
            }
            if frag.contains("-related_pin CLKA") && field_of(frag, "CLKA").as_deref() == Some("R")
            {
                saw_a_rise_edge |= frag.contains("-type edge \\");
            }
            // The CLKB->Q release arcs are `-type edge` too, on CLKB's FALLING (opening) edge.
            if frag.contains("-related_pin CLKB") {
                assert_eq!(
                    field_of(frag, "CLKB").as_deref(),
                    Some("F"),
                    "only CLKB's opening (falling) edge reaches Q: {frag}"
                );
                assert!(
                    frag.contains("-type edge \\"),
                    "CLKB release Q arc must be -type edge: {frag}"
                );
                saw_b_fall_edge = true;
            }
        }
        assert!(saw_a_rise_edge, "CLKA rising Q capture arc is -type edge");
        assert!(saw_b_fall_edge, "CLKB falling Q release arc is -type edge");
    }

    /// COEX: a single output pin carrying edge, combinational AND async arcs at once. CLK's rising edge
    /// captures (`-type edge`); a non-async set B forces Q high (`-type combinational`); an async clear R
    /// forces Q low (`-type async`). All three coexist on pin Q -- no per-output suppression.
    #[test]
    fn coex_edge_combinational_async_coexist_on_one_pin() {
        let cell = analyse(
            r#"
[[cell]]
name = "COEX"
inputs = ["CLK", "D", "B", "R"]
clock = ["CLK"]
async = ["R"]
[cell.internal]
M = "!R*(B + !CLK*D + CLK*M)"
[cell.outputs]
Q = "!R*(B + CLK*M + !CLK*Q)"
"#,
        );
        let tcl = cell_arcs_tcl(&cell, ArcsTclOptions::default());
        eprintln!("{tcl}");
        let q_arc = |related: &str, ty: &str| {
            let rp = format!("-related_pin {related}");
            let ty = format!("-type {ty} \\");
            tcl.split("define_arc")
                .any(|frag| frag.contains("-pin Q") && frag.contains(&rp) && frag.contains(&ty))
        };
        assert!(q_arc("CLK", "edge"), "CLK->Q is -type edge");
        assert!(q_arc("B", "combinational"), "B->Q is -type combinational");
        assert!(q_arc("R", "async"), "R->Q is -type async");
    }

    /// BOTH_RESET: edge and async arcs coexist on one output pin. CLK's rising edge captures
    /// (`-type edge`); the declared async clear R forces Q low (`-type async`).
    #[test]
    fn both_reset_edge_and_async_coexist_on_one_pin() {
        let cell = analyse(
            r#"
[[cell]]
name = "BR"
inputs = ["CLK", "D", "R"]
clock = ["CLK"]
async = ["R"]
[cell.internal]
M = "!R*(!CLK*D + CLK*M)"
[cell.outputs]
Q = "!R*(CLK*M + !CLK*Q)"
"#,
        );
        let tcl = cell_arcs_tcl(&cell, ArcsTclOptions::default());
        eprintln!("{tcl}");
        let has_clk_edge = tcl.split("define_arc").any(|frag| {
            frag.contains("-pin Q")
                && frag.contains("-related_pin CLK")
                && frag.contains("-type edge \\")
        });
        let has_r_async = tcl.split("define_arc").any(|frag| {
            frag.contains("-pin Q")
                && frag.contains("-related_pin R")
                && frag.contains("-type async \\")
        });
        assert!(has_clk_edge, "CLK->Q is -type edge");
        assert!(has_r_async, "R->Q is -type async, coexisting on pin Q");
    }

    /// A lone level-sensitive latch whose ENABLE is a declared clock. A latch has no CAPTURE — nothing
    /// holds independently of the enable's level — but it does have a RELEASE: the enable's rising edge
    /// takes it from opaque to transparent and transmits the `D` value that changed while it was closed.
    /// That release is a timing arc, so the enable->Q arcs render `-type edge` even though `captures` is
    /// empty.
    #[test]
    fn latch_enable_to_q_arcs_are_release_edge_type() {
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
        assert!(cell.edge.captures.is_empty(), "a latch has no capture");
        assert!(
            cell.edge
                .labels
                .iter()
                .any(|(n, c, e, _)| n == "Q" && c == "EN" && *e == Edge::Rise),
            "the enable's rising edge opens the latch (an edge arc): {:?}",
            cell.edge.labels
        );
        let tcl = cell_arcs_tcl(&cell, ArcsTclOptions::default());
        eprintln!("{tcl}");
        let mut saw_release = false;
        for frag in tcl.split("define_arc") {
            if !(frag.contains("-pin Q") && frag.contains("-related_pin EN")) {
                continue;
            }
            assert!(frag.contains("-type edge \\"), "EN->Q release arc: {frag}");
            assert!(!frag.contains("-type combinational"));
            saw_release = true;
        }
        assert!(saw_release, "the EN->Q release arcs are emitted");
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

    /// A single transparent latch, whose enable's rising edge is a RELEASE. It has no capture, but the
    /// release is a real timing arc, so its `CLK`->`Q` arcs render `-type edge`. Opting out
    /// (`no_edge_collapse`) suppresses the classification entirely and restores `-type combinational`.
    const DLAT: &str = r#"
[[cell]]
name = "DLAT"
inputs = ["CLK", "D"]
clock = ["CLK"]
[cell.outputs]
Q = "CLK*D + !CLK*Q"
"#;

    /// A master/slave pair split across two DIFFERENT declared clocks. `Q` never captures — CLKB's rising
    /// edge RELEASES the output latch, and CLKA's falling edge (the master closing) reaches `Q` as a
    /// CONDITIONED release, through the CLKB latch while it is open. Conditioning never reclassifies an
    /// arc: the condition rides in `-when`, the type stays `edge`.
    const MCDFF: &str = r#"
[[cell]]
name = "MCDFF"
inputs = ["CLKA", "CLKB", "D"]
clock = ["CLKA", "CLKB"]
[cell.internal]
M = "!CLKA*D + CLKA*M"
[cell.outputs]
Q = "CLKB*M + !CLKB*Q"
"#;

    #[test]
    fn dlat_enable_release_is_edge_type_and_opts_out() {
        let (default, forced) = analyse_both(DLAT);
        let tcl_default = cell_arcs_tcl(&default, ArcsTclOptions::default());
        eprintln!("{tcl_default}");
        assert!(default.edge.captures.is_empty(), "a latch has no capture");
        // The enable's rising (opening) edge is the only CLK->Q arc, and it is `-type edge`.
        for frag in tcl_default.split("define_arc") {
            if !(frag.contains("-pin Q") && frag.contains("-related_pin CLK")) {
                continue;
            }
            assert!(frag.contains("-type edge \\"), "CLK->Q release: {frag}");
        }
        assert!(tcl_default.matches("-type edge").count() >= 1);
        // Opted out, the same cell falls back to plain combinational arcs.
        let tcl_forced = cell_arcs_tcl(&forced, ArcsTclOptions::default());
        assert_eq!(tcl_forced.matches("-type edge").count(), 0);
    }

    #[test]
    fn mcdff_two_clock_releases_are_edge_type_with_conditions_preserved() {
        let (default, forced) = analyse_both(MCDFF);
        let tcl = cell_arcs_tcl(&default, ArcsTclOptions::default());
        eprintln!("{tcl}");
        assert!(default.edge.captures.is_empty(), "neither clock captures Q");
        // Pinlist order is {CLKA CLKB D Q}.
        let field_of = |frag: &str, idx: usize| -> Option<String> {
            frag.lines()
                .find(|l| l.contains("-vector"))
                .and_then(|l| l.split('{').nth(1))
                .and_then(|v| v.split_whitespace().nth(idx))
                .map(str::to_string)
        };
        let mut saw_b_release = false;
        let mut saw_conditioned_a_release = false;
        for frag in tcl.split("define_arc") {
            if !frag.contains("-pin Q") {
                continue;
            }
            if frag.contains("-related_pin CLKB") {
                assert_eq!(field_of(frag, 1).as_deref(), Some("R"), "{frag}");
                assert!(frag.contains("-type edge \\"), "CLKB release: {frag}");
                saw_b_release = true;
            }
            if frag.contains("-related_pin CLKA") {
                assert_eq!(field_of(frag, 0).as_deref(), Some("F"), "{frag}");
                assert!(frag.contains("-type edge \\"), "CLKA release: {frag}");
                // The condition (CLKB open) rides in `-when`; it does not reclassify the arc.
                assert!(
                    frag.contains("-when \"CLKB"),
                    "conditioned release keeps its -when: {frag}"
                );
                saw_conditioned_a_release = true;
            }
        }
        assert!(saw_b_release, "CLKB rising release Q arc is -type edge");
        assert!(
            saw_conditioned_a_release,
            "CLKA falling conditioned release Q arc is -type edge"
        );
        // Opted out, both fall back to plain combinational arcs.
        let tcl_forced = cell_arcs_tcl(&forced, ArcsTclOptions::default());
        assert_eq!(tcl_forced.matches("-type edge").count(), 0);
    }

    /// Two shapes that carry NO edge arc at all — neither a capture nor a release — even under default
    /// (on) classification: a gated (self-referencing) latch, whose enable's edge transmits nothing that
    /// changed while it was closed, and a two-latch DFF whose clock is never declared.
    const NON_COLLAPSIBLE: [&str; 2] = [
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
        // Zero `-type edge` blocks, whether the flag is left off (default classification, a no-op on
        // these shapes) or forced on -- and the two runs emit byte-identical Tcl.
        for src in NON_COLLAPSIBLE {
            let (default, forced) = analyse_both(src);
            let tcl_default = cell_arcs_tcl(&default, ArcsTclOptions::default());
            let tcl_forced = cell_arcs_tcl(&forced, ArcsTclOptions::default());
            assert_eq!(tcl_default.matches("-type edge").count(), 0);
            assert_eq!(tcl_forced.matches("-type edge").count(), 0);
            assert_eq!(tcl_default, tcl_forced);
        }
    }

    /// The exposed-master DFF: the behavioural pass recognises the slave `Q` as CAPTURING on CLK's rising
    /// edge, while the declared-output master `M` is a latch RELEASED by CLK's falling edge. The two
    /// categories are distinct internally but share the `-type edge` token, so both pins carry an edge
    /// arc -- `Q` on the rise, `M` on the fall.
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
        assert!(!cell.edge.captures.is_empty());
        let tcl = cell_arcs_tcl(&cell, ArcsTclOptions::default());
        eprintln!("{tcl}");
        assert!(tcl.matches("-type edge").count() >= 1);
        assert!(
            cell.edge
                .labels
                .iter()
                .any(|(n, c, e, _)| n == "M" && c == "CLK" && *e == Edge::Fall),
            "the exposed master opens on CLK's fall (an edge arc): {:?}",
            cell.edge.labels
        );
        // Vector order is {CLK D M Q} (inputs then outputs, declaration order): CLK's own field is the
        // arc's edge on the related clock.
        let clk_field = |frag: &str| -> Option<String> {
            frag.lines()
                .find(|l| l.contains("-vector"))
                .and_then(|l| l.split('{').nth(1))
                .and_then(|v| v.split_whitespace().next())
                .map(str::to_string)
        };
        let (mut saw_q_capture, mut saw_m_release) = (false, false);
        for frag in tcl.split("define_arc") {
            if !frag.contains("-related_pin CLK") || frag.contains("-type hidden") {
                continue;
            }
            match (frag.contains("-pin Q \\"), clk_field(frag).as_deref()) {
                (true, Some("R")) => {
                    assert!(frag.contains("-type edge \\"), "Q capture: {frag}");
                    saw_q_capture = true;
                }
                (false, Some("F")) => {
                    assert!(frag.contains("-type edge \\"), "M release: {frag}");
                    saw_m_release = true;
                }
                _ => {}
            }
        }
        assert!(saw_q_capture, "Q's CLK-rise capture is -type edge");
        assert!(saw_m_release, "M's CLK-fall release is -type edge");
    }

    /// RDFF: a both-latch clear `R` that is ALSO declared a clock pin. R's assert arcs are a LEVEL
    /// action — `R=1` alone pins Q low, not a transition that holds independently of R's level — so R's
    /// arcs stay `-type combinational`, byte-for-byte the classification `SYNCR` (the same cell with R
    /// undeclared) gets. Declaring a level-acting pin a clock must never conjure an edge arc that isn't
    /// there.
    #[test]
    fn rdff_clock_declared_reset_arcs_stay_combinational() {
        let cell = analyse(
            r#"
[[cell]]
name = "RDFF"
inputs = ["CLK", "D", "R"]
clock = ["CLK", "R"]
[cell.internal]
M = "!R*(!CLK*D + CLK*M)"
[cell.outputs]
Q = "!R*(CLK*M + !CLK*Q)"
"#,
        );
        let tcl = cell_arcs_tcl(&cell, ArcsTclOptions::default());
        eprintln!("{tcl}");
        let (mut saw_r, mut saw_clk_edge) = (false, false);
        for frag in tcl.split("define_arc") {
            if !frag.contains("-pin Q") || frag.contains("-type hidden") {
                continue;
            }
            if frag.contains("-related_pin R") {
                assert!(
                    frag.contains("-type combinational \\"),
                    "R->Q is a level clear, not a release: {frag}"
                );
                saw_r = true;
            }
            if frag.contains("-related_pin CLK") {
                assert!(frag.contains("-type edge \\"), "CLK->Q capture: {frag}");
                saw_clk_edge = true;
            }
        }
        assert!(saw_r, "the R->Q clear arcs are emitted");
        assert!(saw_clk_edge, "the CLK->Q capture arcs are emitted");
    }

    /// An integrated clock gate: `GCLK` is a gated clock, not a latch output. `GCLK` acts by the level of
    /// CLK (`CLK*EL`) rather than holding a value independently of it, so its arcs stay `-type
    /// combinational` -- on both clock edges. The internal enable latch `EL` does have an edge arc of its
    /// own, but it drives no Liberty output, so no `-type edge` block is emitted.
    #[test]
    fn icg_gclk_arcs_stay_combinational() {
        let cell = analyse(
            r#"
[[cell]]
name = "ICG"
inputs = ["CLK", "EN"]
clock = ["CLK"]
[cell.internal]
EL = "!CLK*EN + CLK*EL"
[cell.outputs]
GCLK = "CLK*EL"
"#,
        );
        let tcl = cell_arcs_tcl(&cell, ArcsTclOptions::default());
        eprintln!("{tcl}");
        for frag in tcl.split("define_arc") {
            if frag.contains("-pin GCLK") && frag.contains("-related_pin CLK") {
                assert!(frag.contains("-type combinational \\"), "GCLK arc: {frag}");
            }
        }
        assert_eq!(tcl.matches("-type edge").count(), 0);
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
