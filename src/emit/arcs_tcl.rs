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
//!
//! Every block of a state-holding cell — transition, hidden and constraint alike — also carries an
//! `-ic` line giving each `-pinlist` pin the voltage it starts the measured vector at (see [`ic_str`]).
//! Liberate discards the `-prevector` simulation instead of carrying its settled values into the
//! vector, so a cell with memory would otherwise begin measuring from state nothing established.
//!
//! A cell that exposes internal nodes (`expose = [...]`) is rendered from its ARC VIEW
//! ([`crate::model::AnalysedCell::arc_view`]), the analysis that keeps those nodes as model coordinates.
//! An exposed node is not a pin, so it earns a `-pinlist` column of its own (see [`arc_pinlist_str`])
//! between the inputs and the outputs, which `-vector` and `-ic` line up with. That column reads `X` in
//! every `-vector`: the vector is stimulus, and a node the cell drives cannot be forced without
//! overriding the behaviour being measured — `-ic` carries its start level instead. Only the arc emitter
//! reads that view — the `define_cell` pinlist ([`pinlist_str`]) and every other artifact keep to the
//! cell's actual pins.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::hash::Hash;

use espresso_logic::Symbol;
use indexmap::IndexMap;

use crate::logic::arcs::{Arc, ArcLevels, Edge, ExposedLevel, HiddenArc};
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

/// All `define_arc` blocks for a cell, concatenated. The general arcs are ALWAYS emitted — one
/// representative per transition, rendered without a `-when` line; an arc class the cell selected in
/// its resolved `when` set ADDS its conditioned blocks on top, so the same arc can appear twice. A cell
/// with a detected oscillation hazard is prefixed with a comment recording the racing condition and the
/// competing settled outcomes — the metastability risk timing arcs cannot express. Any derived
/// constraint arcs (setup/hold, non_seq) the cell opted into — its `constraint_arcs` was set, so
/// generation populated `cell.constraints` — follow the delay arcs.
pub fn cell_arcs_tcl(cell: &AnalysedCell, opts: ArcsTclOptions) -> String {
    // Everything below renders the arc view: for a cell that exposes internal nodes that is the analysis
    // carrying them as model coordinates, so its arcs, hazards and leakage states are the ones an
    // exposed column can be read off; for every other cell it IS the cell.
    let cell = cell.arc_view();
    let mut out = oscillation_comment(cell);
    // Each arc class is emitted in two passes. The GENERAL pass comes out ALWAYS: one representative per
    // transition — a related pin's edge driving an output pin's edge — rendered with no `-when` line, so
    // the block generalises over the side inputs' held levels, the held outputs and the internal state
    // the transition was measured from, keeping the member with the shortest prevector (see
    // `generalised`, which also returns how many firings each representative stands for). A class the
    // cell selected in its resolved `when` set then ADDS a `-when` block for every one of its arcs, on
    // top of the general ones, so one arc can appear twice: once as its transition's general
    // representative, once carrying its own condition. The conditioned pass skips a representative whose
    // conditioned block adds nothing over the general one: the transition has a single case (its one
    // context is already what the general block stands for), or the representative renders no `-when` (so
    // the two blocks would be identical). Any non-representative firing renders its own prevector and is
    // emitted whether or not it carries a condition.
    //
    // Two redundancies survive here BY DESIGN — do not "optimise" either away. On a transition with more
    // than one context, the representative's OWN conditioned block restates the context its general block
    // already pins, yet it is emitted: the conditioned pass names every context of a multi-context
    // transition explicitly and symmetrically, the representative's included. And two firings that agree
    // on vector and `-when` but reach it from different internal states both emit identical conditioned
    // blocks — the emitted form cannot express the internal state that makes them distinct arcs. Neither
    // repeat drops an arc or misstates timing; the duplication is harmless.
    //
    // A measured block addresses its exposed columns by name, so the aliases it may name together are
    // those whose netlist agrees on them — its [`Group`]. Every pass below therefore runs per group,
    // each group's blocks contiguous. A cell that maps no node per alias has exactly one group holding
    // every alias, and emits what it always did.
    let general = generalised(
        &cell.arcs,
        |arc| ArcIdentity::of(cell, arc),
        |arc| arc.prevector.len(),
    );
    let general_hidden = generalised(&cell.hidden_arcs, ArcIdentity::of_hidden, |h| {
        h.prevector.len()
    });
    let groups = groups(cell);
    for group in &groups {
        for (_, arc) in cell
            .arcs
            .iter()
            .enumerate()
            .filter(|(i, _)| general.contains_key(i))
        {
            out.push_str(&format_arc(cell, group, arc, false));
        }
        if cell.when.contains(ArcClass::Transition) {
            for (i, arc) in cell.arcs.iter().enumerate() {
                let redundant = general
                    .get(&i)
                    .is_some_and(|&cases| cases == 1 || when_str(&arc.end, &arc.related).is_none());
                if !redundant {
                    out.push_str(&format_arc(cell, group, arc, true));
                }
            }
        }
        if opts.emit_internal {
            for (_, h) in cell
                .hidden_arcs
                .iter()
                .enumerate()
                .filter(|(i, _)| general_hidden.contains_key(i))
            {
                out.push_str(&format_hidden_arc(cell, group, h, false));
            }
            if cell.when.contains(ArcClass::Hidden) {
                // The same rule as the transition pass, over the hidden class's own condition (which
                // carries every held output on top of the other inputs): a representative whose pin
                // toggles from a single context, or which renders no `-when`, is fully characterised by
                // its general block.
                for (i, h) in cell.hidden_arcs.iter().enumerate() {
                    let redundant = general_hidden
                        .get(&i)
                        .is_some_and(|&cases| cases == 1 || hidden_when_str(h).is_none());
                    if !redundant {
                        out.push_str(&format_hidden_arc(cell, group, h, true));
                    }
                }
            }
        }
    }
    // Leakage carries the cell's own pins and no exposed column, so no group divides it: one block per
    // rest state, naming every alias.
    if opts.emit_leakage {
        for l in &cell.leakage {
            out.push_str(&format_leakage(cell, l));
        }
    }
    // Constraint arcs emit whatever generation produced: `cell.constraints` is populated only when the
    // cell opted in (per-cell `constraint_arcs`, or the global `--constraints` flag), and is empty
    // otherwise — so this loop is its own gate.
    for group in &groups {
        for c in &cell.constraints {
            out.push_str(&format_constraint(cell, group, c));
        }
    }
    out
}

/// The general arcs of one class: each identity's representative index into `items`, mapped to the
/// number of firings that identity has. `key` groups the firings — every firing carrying one
/// [`ArcIdentity`] falls in a single group and one block comes out of it, generalising over the contexts
/// the firings differed in. The representative is one with the SHORTEST prevector: only a strictly
/// shorter prevector displaces the incumbent, so where several firings tie at the minimum any one may be
/// kept. The conditioned pass reads this map by index: `contains_key` recognises a representative, and
/// the firing count tells it whether the representative stands for a single context (see
/// [`cell_arcs_tcl`]).
fn generalised<T, K: Hash + Eq>(
    items: &[T],
    key: impl Fn(&T) -> K,
    prevector_len: impl Fn(&T) -> usize,
) -> HashMap<usize, usize> {
    // Per identity: the strictly-shortest-prevector winner and the firing count.
    let mut groups: HashMap<
        K,
        (
            usize, /* winner len */
            usize, /* winner index */
            usize, /* count */
        ),
    > = HashMap::new();
    for (i, item) in items.iter().enumerate() {
        let len = prevector_len(item);
        groups
            .entry(key(item))
            .and_modify(|g| {
                g.2 += 1;
                if len < g.0 {
                    g.0 = len;
                    g.1 = i;
                }
            })
            .or_insert((len, i, 1));
    }
    groups
        .into_values()
        .map(|(_, i, count)| (i, count))
        .collect()
}

/// The event a transition arc measures: the output pin and the edge it makes, the related pin and the
/// edge IT makes. The side inputs' held levels, the held outputs and the internal state are the firing's
/// CONDITION rather than part of the event, so they are absent — one transition yields ONE general block
/// however many contexts it was measured from, and every one of those contexts returns as its own
/// conditioned block under `--when`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Transition {
    output: Symbol,
    edge: Edge,
    related: Symbol,
    related_edge: Edge,
}

/// An emitted arc's identity. The variant IS Liberate's `-type` taxonomy: the three transition kinds
/// carry the [`Transition`] event they measure, while a hidden arc — an input toggle no output follows —
/// carries the toggled pin and its edge and structurally holds no related pin.
///
/// The kind is part of the identity because `-type` declares the arc's nature to Liberate and is decided
/// per firing, from the full machine start state (see [`ArcIdentity::of`]): a transition that classifies
/// differently from different start states is two arc kinds, and collapsing across it would delete one
/// of them from the output.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum ArcIdentity {
    Async(Transition),
    Edge(Transition),
    Combinational(Transition),
    Hidden { pin: Symbol, edge: Edge },
}

impl ArcIdentity {
    /// A transition arc's identity: [`ArcIdentity::Async`] for a declared-async related pin, else
    /// [`ArcIdentity::Edge`] when the arc's FULL context `(output, related, direction, machine start)` is
    /// labelled a clock-edge timing arc in [`crate::logic::edge::EdgeArcs::labels`], else
    /// [`ArcIdentity::Combinational`]. There is ONE edge category, so two firings that differ only in
    /// internal state can classify differently.
    fn of(cell: &AnalysedCell, arc: &Arc) -> Self {
        let related_edge = related_edge(arc);
        let transition = Transition {
            output: arc.output.clone(),
            edge: arc.edge,
            related: arc.related.clone(),
            related_edge,
        };
        if arc.is_async {
            ArcIdentity::Async(transition)
        } else if cell.edge.labels.contains(&(
            arc.output.clone(),
            arc.related.clone(),
            related_edge,
            arc.start.clone(),
        )) {
            ArcIdentity::Edge(transition)
        } else {
            ArcIdentity::Combinational(transition)
        }
    }

    /// A hidden arc's identity: the toggled pin and the edge it makes. That pair IS the event; the other
    /// inputs' held levels and the held outputs are its condition and ride in [`hidden_when_str`], so
    /// they are absent here for the same reason as in [`Transition`].
    fn of_hidden(h: &HiddenArc) -> Self {
        ArcIdentity::Hidden {
            pin: h.pin.clone(),
            edge: h.edge,
        }
    }

    /// The `-type` word Liberate reads, and the ONE source of it: every `define_arc` block the emitter
    /// renders takes its type line from here.
    fn type_token(&self) -> &'static str {
        match self {
            ArcIdentity::Async(_) => "async",
            ArcIdentity::Edge(_) => "edge",
            ArcIdentity::Combinational(_) => "combinational",
            ArcIdentity::Hidden { .. } => "hidden",
        }
    }
}

/// Cell names braced as a Tcl list: `{ C2 }` for a single name, `{ C2A C2B }` for several.
fn name_block(names: &[Symbol]) -> String {
    format!("{{ {} }}", names.join(" "))
}

/// The drive-strength aliases a block can name together, with the netlist nodes its exposed columns
/// address.
///
/// A block addresses an exposed node by ONE name, so it can only name aliases that agree on it. Where
/// `[cell.nodes]` maps a node per alias — the same held state sitting on a differently-spelled node in
/// each alias's netlist — the aliases that disagree cannot share a block, and the arcs fan out into one
/// set per group, as `define_cell` fans out per template triple. A cell that maps nothing, or exposes
/// nothing, has one group holding every alias, which is the whole cell.
struct Group {
    /// The aliases of this group, in the cell's declared order.
    names: Vec<Symbol>,
    /// What this group's exposed nodes are called in its netlist, in `cell.exposed` order.
    exposed: Vec<Symbol>,
}

/// The cell's alias groups, in first-appearance order — aliases bundled by the netlist nodes their
/// exposed columns resolve to.
fn groups(cell: &AnalysedCell) -> Vec<Group> {
    let mut by_nodes: IndexMap<Vec<Symbol>, Vec<Symbol>> = IndexMap::new();
    for alias in &cell.name {
        let resolved: Vec<Symbol> = cell
            .exposed
            .iter()
            .map(|node| cell.nodes.of(alias, node))
            .collect();
        by_nodes.entry(resolved).or_default().push(alias.clone());
    }
    by_nodes
        .into_iter()
        .map(|(exposed, names)| Group { names, exposed })
        .collect()
}

/// A constraint arc as a pair of `define_arc` blocks — the setup member and the hold member (Liberate
/// characterises them as separate arcs): `setup`/`hold` for a directed clock↔data constraint,
/// `non_seq_setup`/`non_seq_hold` for a symmetric (oscillation / mutual-exclusion) one.
fn format_constraint(cell: &AnalysedCell, group: &Group, c: &Constraint) -> String {
    let (setup, hold) = match c.kind {
        ConstraintKind::SetupHold => ("setup", "hold"),
        ConstraintKind::NonSeq => ("non_seq_setup", "non_seq_hold"),
    };
    let mut s = constraint_block(cell, group, c, setup);
    s.push_str(&constraint_block(cell, group, c, hold));
    s
}

/// One constraint `define_arc` of the given `-type`. Liberate cannot infer how to prepare these
/// non-standard state-holding cells, so every pin is listed and fully specified: the `-prevector`
/// drives the cell (inputs + internal state) into the pre-toggle state, and the full `-vector` carries
/// the two switching pins as `R`/`F`, the other inputs at their held value, and the outputs as `X`.
fn constraint_block(cell: &AnalysedCell, group: &Group, c: &Constraint, arc_type: &str) -> String {
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
    s.push_str(&format!(
        "\t-pinlist {{{}}} \\\n",
        arc_pinlist_str(cell, &group.exposed)
    ));
    if cell.state_holding {
        s.push_str(&format!(
            "\t-ic \"{}\" \\\n",
            ic_str(cell, &c.prevector, &c.levels)
        ));
    }
    s.push_str(&format!(
        "\t-vector {{{}}} \\\n",
        constraint_vector_str(cell, c)
    ));
    s.push_str(&format!("\t-related_pin {} \\\n", c.related));
    s.push_str(&format!("\t-pin {} \\\n", c.pin));
    s.push_str(&format!("\t{}\n", name_block(&group.names)));
    s.push('\n');
    s
}

/// The full constraint vector over [`arc_pinlist_str`] order: the related and pin pins as their `R`/`F`
/// edges, every other input at its held value in the pre-toggle state (the prevector's last step), and
/// every output as `X` (a constraint arc measures no output transition). An exposed node reads `X`
/// alongside them: the block constrains WHEN the two input edges may land relative to each other and
/// measures nothing the cell does in response, so the same column that leaves the outputs unstated
/// leaves the internals unstated too. The node's start level still reaches Liberate — the `-ic` line
/// below carries it, as it does for the outputs.
fn constraint_vector_str(cell: &AnalysedCell, c: &Constraint) -> String {
    let held = assignment(
        c.prevector
            .last()
            .expect("path_to seeds its chain with the probed node itself"),
    );
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

/// One transition `define_arc`. `with_when` selects which of the two passes in [`cell_arcs_tcl`] the
/// block belongs to: the conditioned one, carrying the arc's `-when`, or the general one. Either way the
/// block renders THIS arc's own concrete `-prevector` and `-vector`: `-vector` is the stimulus Liberate
/// drives and `X` is legal only in the unmonitored-output columns (see [`vector_str`]), so a general
/// block's generality lives in the ABSENCE of the `-when` line, not in a relaxed vector.
fn format_arc(cell: &AnalysedCell, group: &Group, arc: &Arc, with_when: bool) -> String {
    let type_line = format!("\t-type {} \\\n", ArcIdentity::of(cell, arc).type_token());
    let prevector_pinlist = format!("\t-prevector_pinlist {{{}}} \\\n", cell.inputs.join(" "));
    let prevector = format!(
        "\t-prevector {{{}}} \\\n",
        prevector_str(cell, &arc.prevector)
    );
    let pinlist = format!(
        "\t-pinlist {{{}}} \\\n",
        arc_pinlist_str(cell, &group.exposed)
    );
    // The `-ic` VALUES are one double-quoted word, never a braced one: Tcl substitutes no variable
    // inside braces, so a braced `$VDD` would reach Liberate as that literal text instead of the supply
    // voltage. A single column within the word carries braces of its own where [`ic_column`] wraps it.
    let ic = if cell.state_holding {
        format!(
            "\t-ic \"{}\" \\\n",
            ic_str(cell, &arc.prevector, &arc.levels)
        )
    } else {
        String::new()
    };
    let vector = format!("\t-vector {{{}}} \\\n", vector_str(cell, arc));
    let when = match (with_when, when_str(&arc.end, &arc.related)) {
        (true, Some(w)) => format!("\t-when \"{w}\" \\\n"),
        _ => String::new(),
    };
    let related = format!("\t-related_pin {} \\\n", arc.related);
    let pin = format!("\t-pin {} \\\n", arc.output);
    let name = format!("\t{}\n", name_block(&group.names));

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
    s.push_str(&ic);
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
/// arc measures no output transition). An exposed node reads `X`: a `-vector` column FORCES its value
/// for the measurement, and an internal node driven by the cell must be left to follow the cell. Its
/// start level reaches Liberate through `-ic` instead. Mirrors [`vector_str`] for [`Arc`], and is the
/// ONE source of `format_hidden_arc`'s `-vector` line.
fn hidden_vector_str(cell: &AnalysedCell, h: &HiddenArc) -> String {
    let held: BTreeMap<&str, bool> = h
        .levels
        .outputs
        .iter()
        .map(|(s, b)| (s.as_str(), *b))
        .collect();

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
        |_| "X".to_string(),
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
/// `with_when` selects which of the two passes in [`cell_arcs_tcl`] the block belongs to: the
/// conditioned one, carrying the arc's `-when`, or the general one, which keeps its own concrete
/// `-prevector` and `-vector` and generalises solely by omitting the `-when` line.
fn format_hidden_arc(cell: &AnalysedCell, group: &Group, h: &HiddenArc, with_when: bool) -> String {
    let vec = hidden_vector_str(cell, h);

    let mut s = String::from("define_arc \\\n");
    s.push_str(&format!(
        "\t-type {} \\\n",
        ArcIdentity::of_hidden(h).type_token()
    ));
    s.push_str(&format!(
        "\t-prevector_pinlist {{{}}} \\\n",
        cell.inputs.join(" ")
    ));
    s.push_str(&format!(
        "\t-prevector {{{}}} \\\n",
        prevector_str(cell, &h.prevector)
    ));
    s.push_str(&format!(
        "\t-pinlist {{{}}} \\\n",
        arc_pinlist_str(cell, &group.exposed)
    ));
    if cell.state_holding {
        s.push_str(&format!(
            "\t-ic \"{}\" \\\n",
            ic_str(cell, &h.prevector, &h.levels)
        ));
    }
    s.push_str(&format!("\t-vector {{{vec}}} \\\n"));
    if let (true, Some(w)) = (with_when, hidden_when_str(h)) {
        s.push_str(&format!("\t-when \"{w}\" \\\n"));
    }
    s.push_str(&format!("\t-pin {} \\\n", h.pin.as_str()));
    s.push_str(&format!("\t{}\n", name_block(&group.names)));
    s.push('\n');
    s
}

/// The cell's pins: inputs then outputs, in declaration order. This is what `define_cell` declares the
/// cell by, so it names PINS only — an exposed internal node has no pin and never appears here.
pub(crate) fn pinlist_str(cell: &AnalysedCell) -> String {
    let mut pins = cell.inputs.clone();
    pins.extend(cell.outputs.iter().map(|o| o.name.clone()));
    pins.join(" ")
}

/// The `-pinlist` of a measured block: the inputs, then the exposed internal nodes in declared order,
/// then the outputs. A `-vector` cannot address a node with no column and an `-ic` cannot initialise
/// one, which is what an exposed internal needs a column FOR; it is still no pin of the cell, so it
/// stays out of [`pinlist_str`] and hence out of `define_cell`.
///
/// An exposed node is listed under the name its netlist holds it on ([`Group::exposed`]), which is what
/// Liberate has to be handed; the pins keep the cell's own names, which the netlist shares. Every other
/// artifact reads the spec's names throughout.
fn arc_pinlist_str(cell: &AnalysedCell, exposed: &[Symbol]) -> String {
    let mut pins = cell.inputs.clone();
    pins.extend(exposed.iter().cloned());
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

/// One symbol per input (cell.inputs order), then one per exposed internal node (declared order), then
/// one per output (cell.outputs order), joined by " ". This walk is [`arc_pinlist_str`]'s order, and
/// every line whose columns Liberate reads against the pinlist — `-vector` and `-ic` — comes through it,
/// so the three agree by construction rather than by three renderers happening to walk alike.
fn vector(
    cell: &AnalysedCell,
    input_sym: impl Fn(&str) -> String,
    exposed_sym: impl Fn(&str) -> String,
    output_sym: impl Fn(&str) -> String,
) -> String {
    let mut parts = Vec::with_capacity(cell.inputs.len() + cell.exposed.len() + cell.outputs.len());
    for input in &cell.inputs {
        parts.push(input_sym(input));
    }
    for node in &cell.exposed {
        parts.push(exposed_sym(node));
    }
    for output in &cell.outputs {
        parts.push(output_sym(&output.name));
    }
    parts.join(" ")
}

/// The exposed levels of one measured block, by node name — the lookup the `-vector` and `-ic`
/// renderers index as [`vector`] walks `cell.exposed`.
fn exposed_levels(levels: &ArcLevels) -> BTreeMap<&str, &ExposedLevel> {
    levels
        .exposed
        .iter()
        .map(|e| (e.node.as_str(), e))
        .collect()
}

/// One `-ic` column: a `logic_low`/`logic_high` expression rendered so Liberate reads it as a single
/// list element.
///
/// The `-ic` values leave as one double-quoted Tcl word, so Tcl runs command, variable and backslash
/// substitution over them and Liberate splits the SUBSTITUTED text into columns by the Tcl list rules:
/// whitespace separates the elements, and an element opening with a brace runs to the matching close
/// brace whatever lies between. An expression that is already one element is written as it stands — a
/// bare word (`GND`), a number (`0.99`), a variable reference in either form (`$VDD`, `${VDD}`), or a
/// value the spec itself wrote as one balanced brace group. Anything else is wrapped in a brace pair,
/// which makes it one element whatever whitespace the substitution leaves in it: `$VDD * 0.9` reaches
/// Liberate as the single column `1.08 * 0.9`, and `[expr $VDD*0.9]` as the one its command
/// substitution resolves to. What that column then means to Liberate is the spec author's affair — the
/// wrap is here to keep the columns aligned with the `-pinlist`.
///
/// The characters that would end the word or shift the split are escaped by [`escape_ic`], so every
/// expression — whatever it holds — comes out as exactly one column of a parseable line.
fn ic_column(value: &str) -> String {
    // The form is recognised in the expression as written; the escaping is about the word it is
    // emitted into, so it applies to a recognised expression and a wrapped one alike.
    let escaped = escape_ic(value);
    if is_one_list_element(value) {
        escaped
    } else {
        format!("{{{escaped}}}")
    }
}

/// One expression's own characters, escaped for the two stages its text crosses: Tcl's backslash
/// substitution as the double-quoted `-ic` word is read, and then the list split Liberate applies to the
/// substituted result. An escape meant for the second stage has to survive the first, so it goes out
/// doubled — `\\{` leaves the word as `\{`, which the list parser reads as a quoted brace and does not
/// count (Tcl(n), "Braces": a brace quoted with a backslash is not counted in locating the matching
/// close brace).
///
/// - A double quote ends the `-ic` word wherever it sits — braces are ordinary characters to the word
///   parser, so a wrap is no shield — and each one therefore goes out as `\"`. Substitution turns it
///   back into a quote before the list is read.
/// - A backslash goes out as `\\\\`, which the substitution halves into the `\\` the list then reads as
///   one quoted backslash. Spending both stages on it is what makes it inert: to the list parser a lone
///   backslash quotes whatever follows — the wrap's own closing brace where the expression ends in one,
///   or a brace whose partner sits elsewhere — so doubling every backslash is what lets each brace be
///   counted whatever precedes it. Left unescaped it would be spent on the first stage instead, either
///   substituting (a `\n` becoming the newline that splits the column in two) or quoting the character
///   after it (the `\` of `$V\"X` consuming the escape that was to tame the quote, and the live quote
///   then ending the word). The consequence is that a backslash sequence in a `logic_low`/`logic_high`
///   expression is literal text rather than the character it names.
/// - A brace with no match — a close brace reached at depth zero, an open brace never closed — goes out
///   as `\\}`/`\\{`, so the list parser passes over it and the wrap still closes on its own brace.
///   `{$VDD` would otherwise leave a group with nothing to close it, and Liberate would reject the line
///   (`unmatched open brace in list`) instead of reading a column count that has shifted. A matched
///   pair is left as it stands: braces are how Tcl groups, and a group written inside a command
///   substitution or a spaced variable reference (`${a b}`) has to reach the first stage intact.
/// - An open bracket that no close bracket reaches goes out as `\[`, a single backslash and not a
///   doubled one: a bracket means nothing to the list parser, so the escape is spent entirely on the
///   first stage, where it stops the command substitution that would otherwise run past the end of the
///   word (`missing close-bracket`). It reaches Liberate as the bracket alone, no backslash beside it. A
///   bracket that does close is left as it stands, command substitution being how an expression such as
///   `[expr $VDD*0.9]` names its level; and a close bracket standing on its own needs nothing, starting
///   no substitution to run away with.
///
/// An escape made here reaches Liberate carrying its backslash, the list parser performing no
/// substitution inside a braced element: the expression `a{b` arrives as the column `a\{b`, and a
/// backslash as the pair it was doubled into. The columns stay aligned and the line parses, which is
/// what the escaping is for; what the column then holds is the spec author's affair.
fn escape_ic(value: &str) -> String {
    // The braces and brackets are decided over the expression as written, before anything is emitted,
    // so the one pass that builds the text reads only its input and never re-escapes its own output.
    let unmatched = unmatched_braces(value);
    let unclosed = unclosed_brackets(value);
    let mut escaped = String::with_capacity(value.len());
    for (i, c) in value.char_indices() {
        match c {
            '\\' => escaped.push_str("\\\\\\\\"),
            '"' => escaped.push_str("\\\""),
            '{' | '}' if unmatched.contains(&i) => {
                escaped.push_str("\\\\");
                escaped.push(c);
            }
            '[' if unclosed.contains(&i) => escaped.push_str("\\["),
            _ => escaped.push(c),
        }
    }
    escaped
}

/// The byte offsets of the braces in `value` that have no partner: a close brace reached at depth zero,
/// and every open brace still standing at the end. Each brace counts whatever precedes it, because
/// [`escape_ic`] escapes every backslash and so leaves none quoting the brace that follows.
fn unmatched_braces(value: &str) -> HashSet<usize> {
    // Each open brace waits on the stack for the close brace that takes it off again; a close brace
    // arriving with the stack empty has none to take, and whatever is still waiting at the end never
    // found one.
    let mut open = Vec::new();
    let mut unmatched = HashSet::new();
    for (i, c) in value.char_indices() {
        if c == '{' {
            open.push(i);
        } else if c == '}' && open.pop().is_none() {
            unmatched.insert(i);
        }
    }
    unmatched.extend(open);
    unmatched
}

/// The byte offsets of the open brackets in `value` that no close bracket ever reaches. A close bracket
/// standing on its own is not among them: it is only the opening one that starts a command
/// substitution, and one left running off the end of the word is what the word parser refuses.
fn unclosed_brackets(value: &str) -> HashSet<usize> {
    let mut open = Vec::new();
    for (i, c) in value.char_indices() {
        if c == '[' {
            open.push(i);
        } else if c == ']' {
            open.pop();
        }
    }
    open.into_iter().collect()
}

/// Whether the expression already reaches Liberate as one list element: a bare word, a number, a
/// variable reference (`$VDD` or `${VDD}`), or one balanced brace group. A reference whose name falls
/// outside the ordinary character set — Tcl(n) allows the braced form any character but a close brace —
/// is left to the wrap, which carries it just as well.
fn is_one_list_element(value: &str) -> bool {
    if let Some(reference) = value.strip_prefix('$') {
        let name = reference
            .strip_prefix('{')
            .and_then(|n| n.strip_suffix('}'))
            .unwrap_or(reference);
        return is_bare_word(name);
    }
    is_bare_word(value) || value.parse::<f64>().is_ok() || is_one_brace_group(value)
}

/// Whether `s` is a run of Tcl's variable-name characters: ASCII letters, digits and underscore, plus
/// namespace separators of two or more colons (Tcl(n), "Variable substitution"). Such a run names the
/// variable of a `$` reference, and standing alone it is a literal holding nothing for Tcl to
/// substitute, group or split.
fn is_bare_word(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    // A single colon separates nothing — it takes two to make a namespace separator.
    let mut colons = 0usize;
    for c in s.chars() {
        match c {
            ':' => colons += 1,
            _ if c.is_ascii_alphanumeric() || c == '_' => {
                if colons == 1 {
                    return false;
                }
                colons = 0;
            }
            _ => return false,
        }
    }
    colons != 1
}

/// Whether the whole value is one balanced brace group — it opens with a brace whose match is its last
/// character. `{a} {b}` is not: its group closes early, leaving two elements. Every brace counts,
/// backslash or no backslash, matching how [`escape_ic`] doubles each backslash and so leaves none
/// quoting the brace that follows it.
fn is_one_brace_group(value: &str) -> bool {
    if !value.starts_with('{') {
        return false;
    }
    let mut depth = 0usize;
    for (i, c) in value.char_indices() {
        match c {
            '{' => depth += 1,
            // The depth reaches zero at the opening brace's match and the scan returns there, so this
            // never runs at depth zero.
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return i + 1 == value.len();
                }
            }
            _ => {}
        }
    }
    false
}

/// The `-ic` initial condition over [`arc_pinlist_str`] order: each column's starting voltage, written
/// as the cell's `logic_low`/`logic_high` expression for the level it starts at (through
/// [`ic_column`]). Inputs start where the
/// prevector leaves them (its last step); outputs and exposed nodes start at the levels measured at the
/// arc's start state. Liberate discards the `-prevector` simulation rather than carrying its settled
/// values forward, so this is what actually establishes a state-holding cell's start condition — and
/// the exposed columns are the reason it can establish an internal one at all, an internal node having
/// no pin to drive it through. Every block kind renders every column, the constraint block included,
/// where `-vector` states no behaviour but the start level is real all the same. Rendered through the
/// same [`vector`] helper as `-pinlist` and `-vector`, so the three lines' columns line up by
/// construction.
fn ic_str(
    cell: &AnalysedCell,
    prevector: &[espresso_logic::Minterm<espresso_logic::Symbol>],
    levels: &ArcLevels,
) -> String {
    let held = assignment(
        prevector
            .last()
            .expect("path_to seeds its chain with the probed node itself"),
    );
    let start: BTreeMap<&str, bool> = levels
        .outputs
        .iter()
        .map(|(s, b)| (s.as_str(), *b))
        .collect();
    let exposed = exposed_levels(levels);
    let column = |level: bool| ic_column(cell.voltages.of(level));
    vector(
        cell,
        |input| {
            let level = *held
                .get(input)
                .expect("every input has a held value in the arc's prevector");
            column(level)
        },
        |node| {
            let level = exposed
                .get(node)
                .expect("the arc's levels define every exposed node")
                .start;
            column(level)
        },
        |name| {
            let level = *start
                .get(name)
                .expect("the arc's levels define every output");
            column(level)
        },
    )
}

/// The measured vector: the related input pin and the measured output as `R`/`F`, the other inputs
/// as their `1`/`0` value in the end state, and every exposed node and other output as `X`. A column
/// here is a stimulus Liberate holds the node to, so an exposed internal — which the cell drives — is
/// left unstated and starts from its `-ic` level.
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
        |_| "X".to_string(),
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
    lits.extend(h.levels.outputs.iter().map(|(s, v)| (s.clone(), *v)));
    if lits.is_empty() {
        return None;
    }
    lits.sort();
    Some(crate::logic::literal_product(&lits))
}

/// One `define_leakage` block for a static leakage state. The `-prevector` drives the cell into the
/// state — priming the internal nodes, which is what distinguishes two rest states sharing an input
/// assignment — and the `-vector` then holds every pin at the level it rests at. The pins are the cell's
/// own, inputs then outputs ([`pinlist_str`]): an exposed internal is no pin of the cell and the
/// prevector has already put it where it belongs, so it takes no column here.
///
/// The walk is rendered whole. Its last step is the state itself — `Explored::path_to` ends its chain
/// there — and so restates the input assignment `-vector` carries, which is not redundancy to remove:
/// Liberate requires a prevector to END at the vector's value. What a walk-free state has is no walk at
/// all, its path being that single step, and there the prevector goes entirely: nothing is primed by
/// stating where the cell already is.
fn format_leakage(cell: &AnalysedCell, l: &LeakageState) -> String {
    let mut lits: Vec<(Symbol, bool)> = assignment(&l.inputs).into_iter().collect();
    lits.extend(l.outputs.iter().cloned());
    lits.sort();

    let mut s = String::from("define_leakage \\\n");
    if l.prevector.len() > 1 {
        s.push_str(&format!(
            "\t-prevector_pinlist {{{}}} \\\n",
            cell.inputs.join(" ")
        ));
        s.push_str(&format!(
            "\t-prevector {{{}}} \\\n",
            prevector_str(cell, &l.prevector)
        ));
    }
    s.push_str(&format!("\t-pinlist {{{}}} \\\n", pinlist_str(cell)));
    s.push_str(&format!(
        "\t-vector {{{}}} \\\n",
        leakage_vector_str(cell, l)
    ));
    if !lits.is_empty() {
        s.push_str(&format!("\t-when \"{}\" \\\n", literal_product(&lits)));
    }
    s.push_str(&format!("\t{}\n", name_block(&cell.name)));
    s.push('\n');
    s
}

/// The leakage `-vector` over [`pinlist_str`] order: every input and output at the constant level it
/// holds in this rest state. A leakage state is static, so no column ever carries an edge.
fn leakage_vector_str(cell: &AnalysedCell, l: &LeakageState) -> String {
    let inputs = assignment(&l.inputs);
    let outputs: BTreeMap<&str, bool> = l.outputs.iter().map(|(s, b)| (s.as_str(), *b)).collect();
    let level = |v: bool| if v { "1" } else { "0" };

    let mut parts = Vec::with_capacity(cell.inputs.len() + cell.outputs.len());
    for input in &cell.inputs {
        let v = *inputs
            .get(input)
            .expect("every input is assigned in a leakage state");
        parts.push(level(v));
    }
    for output in &cell.outputs {
        let v = *outputs
            .get(output.name.as_str())
            .expect("every output is defined at a fully-initialised leakage state");
        parts.push(level(v));
    }
    parts.join(" ")
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeSet, HashSet};

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
        // The default emits the general arcs only, so no ARC block carries a `-when` line — read off a
        // leakage-free render, since `define_leakage` is inherently `-when`-conditioned.
        let arcs_only = cell_arcs_tcl(&cell, NO_LEAKAGE);
        assert!(!arcs_only
            .lines()
            .any(|l| l.trim_start().starts_with("-when")));
    }

    #[test]
    fn and2_hidden_toggles_are_single_context() {
        // A stateless AND has no stored value, so each input toggle that holds `Y` fires from exactly one
        // context. Under `when = "hidden"` those single-context toggles are fully characterised by their
        // general block — the held output is already pinned in its `-vector` — so their conditioned copies
        // are suppressed and no hidden block carries a `-when`.
        let cell = analyse(
            r#"
[[cell]]
name = "AND2"
inputs = ["A", "B"]
when = "hidden"
[cell.outputs]
Y = "A*B"
"#,
        );
        let tcl = cell_arcs_tcl(&cell, NO_LEAKAGE);
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
            // Single-context toggle: only the general block, no conditioned copy.
            assert!(
                !has_when(frag),
                "single-context hidden toggle emits no -when: {frag}"
            );
        }
        // The A-falls-while-B=0 general block holds Y at 0 — the held output rides in the `-vector`,
        // which is why the conditioned `-when` would add nothing.
        assert!(tcl
            .split("define_arc")
            .any(|frag| frag.contains("-type hidden")
                && frag.contains("-vector {F 0 0}")
                && frag.contains("-pin A")));
    }

    #[test]
    fn dlatch_hidden_when_carries_held_output() {
        // Transparent-high D-latch: a D toggle in hold (E=0) leaves Q unchanged, but the two stored-value
        // contexts differ in the held Q. Both must be emitted as hidden `-pin D` arcs and disambiguated by
        // the held Q literal folded into `-when` — which the hidden class's conditioned pass renders, so
        // the cell opts in with `when = "hidden"`.
        let cell = analyse(
            r#"
[[cell]]
name = "DLAT"
inputs = ["E", "D"]
when = "hidden"
[cell.outputs]
Q = "E*D + !E*Q"
"#,
        );
        let tcl = cell_arcs_tcl(&cell, NO_LEAKAGE);
        eprintln!("{tcl}");
        let d_hidden: Vec<&str> = tcl
            .split("define_arc")
            .filter(|frag| frag.contains("-type hidden") && frag.contains("-pin D"))
            .collect();
        // The `-when` of one context holds Q true (`* Q`, not `!Q`) and another holds Q false (`!Q`). Only
        // an arc's own `-when` line counts (a leakage `-when` rides on its `define_leakage` line).
        let when_of = |frag: &str| {
            frag.lines()
                .find(|l| l.trim_start().starts_with("-when"))
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

    /// ONE PER PIN EDGE: the general pass emits exactly one hidden block per distinct `(pin, edge)` —
    /// the toggle event — whatever the other inputs and held outputs were when it was measured.
    #[test]
    fn hidden_general_arcs_are_one_per_pin_edge() {
        let cell = analyse(
            r#"
[[cell]]
name = "AND2"
inputs = ["A", "B"]
[cell.outputs]
Y = "A*B"
"#,
        );
        let default = cell_arcs_tcl(&cell, NO_LEAKAGE);
        eprintln!("{default}");
        assert!(!default.lines().any(|l| l.trim_start().starts_with("-when")));
        let events: HashSet<ArcIdentity> = cell
            .hidden_arcs
            .iter()
            .map(ArcIdentity::of_hidden)
            .collect();
        assert!(!events.is_empty(), "AND2 emits hidden arcs");
        let hidden = blocks(&default)
            .iter()
            .filter(|b| b.contains("-type hidden"))
            .count();
        assert_eq!(
            hidden,
            events.len(),
            "one general hidden block per distinct (pin, edge)"
        );
    }

    #[test]
    fn when_flag_adds_the_when_clause() {
        let cell = analyse(
            r#"
[[cell]]
name = "C2"
inputs = ["A", "B"]
[cell.outputs]
Q = "A*B + Q*(A+B)"
"#,
        );
        // The same cell but for `when = true`, which selects every arc class's conditioned blocks.
        let selected = analyse(
            r#"
[[cell]]
name = "C2"
inputs = ["A", "B"]
when = true
[cell.outputs]
Q = "A*B + Q*(A+B)"
"#,
        );
        let arc_when = |tcl: &str| {
            tcl.lines()
                .filter(|l| l.trim_start().starts_with("-when"))
                .count()
        };
        let off = cell_arcs_tcl(&cell, NO_LEAKAGE);
        let on = cell_arcs_tcl(&selected, NO_LEAKAGE);
        assert_eq!(arc_when(&off), 0);
        assert!(arc_when(&on) >= 1);
    }

    /// NOTHING IS LOST: a 3-input majority gate fires each of its transitions from several side-input
    /// contexts, so the default output carries one block per transition — strictly fewer than the
    /// discovered firings — and selecting every class brings every one of those firings back, each with
    /// its own `-when`.
    #[test]
    fn when_restores_every_discovered_firing() {
        let cell = analyse(MAJ3);
        let selected = analyse(&when_variant(MAJ3, "true"));
        let default = cell_arcs_tcl(&cell, NO_LEAKAGE);
        let on = cell_arcs_tcl(&selected, NO_LEAKAGE);
        eprintln!("{default}");

        let transitions: HashSet<_> = cell
            .arcs
            .iter()
            .map(|a| ArcIdentity::of(&cell, a))
            .collect();
        let events: HashSet<_> = cell
            .hidden_arcs
            .iter()
            .map(ArcIdentity::of_hidden)
            .collect();
        let firings = cell.arcs.len() + cell.hidden_arcs.len();
        assert_eq!(
            blocks(&default).len(),
            transitions.len() + events.len(),
            "the default output is one block per transition and per hidden event"
        );
        assert!(
            blocks(&default).len() < firings,
            "premise: MAJ3 fires a transition from several contexts, so the general pass collapses \
             {firings} firings"
        );

        // Every discovered firing is emitted with its own condition once the class is selected.
        for a in &selected.arcs {
            let conditioned = format_arc(&selected, &whole(&selected), a, true);
            assert!(
                has_when(&conditioned) && on.contains(&conditioned),
                "a discovered firing is missing under `when`:\n{conditioned}"
            );
        }
    }

    // ---- General-arc contract: shortest prevector, class selectivity ----

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

    /// A 3-input majority gate: stateless, so a transition's context is entirely the other two inputs'
    /// held levels, and each transition fires from several of them.
    const MAJ3: &str = r#"
[[cell]]
name = "MAJ3"
inputs = ["A", "B", "C"]
[cell.outputs]
Y = "A*B + B*C + A*C"
"#;

    /// An OR-AND 2-2: `A` rising drives `Y` rising from every side-input context that holds `B` low and
    /// the other OR term satisfied — `(C,D)` at `01`, `10` or `11`. Three discovered firings of ONE
    /// transition.
    const OA22: &str = r#"
[[cell]]
name = "OA22"
inputs = ["A", "B", "C", "D"]
[cell.outputs]
Y = "(A+B)*(C+D)"
"#;

    /// A 2-input AND: each output edge needs the other input held high, so every transition fires from
    /// exactly one context — a single conditioned firing apiece.
    const AND2: &str = r#"
[[cell]]
name = "AND2"
inputs = ["A", "B"]
[cell.outputs]
Y = "A*B"
"#;

    /// `src` with a `when = <value>` key spliced into its `[[cell]]` table — just before its FIRST
    /// `[cell.…]` sub-table, so a fixture declaring `[cell.internal]` gets the key in the cell table
    /// rather than as an internal function. `value` is the raw TOML: `true`, `"hidden"`, `"transition"`.
    fn when_variant(src: &str, value: &str) -> String {
        let at = src
            .find("\n[cell.")
            .expect("the fixture declares a [cell.…] sub-table");
        format!("{}\nwhen = {value}{}", &src[..at], &src[at..])
    }

    /// The cell as the single group it is when no node is mapped per alias: every alias named together,
    /// every exposed node under its own name. Tests that render one block directly go through this.
    fn whole(cell: &AnalysedCell) -> Group {
        let mut g = groups(cell);
        assert_eq!(g.len(), 1, "fixture has one alias group");
        g.remove(0)
    }

    /// Isolate the arc passes from `define_leakage`, which is inherently `-when`-conditioned.
    const NO_LEAKAGE: ArcsTclOptions = ArcsTclOptions {
        emit_internal: true,
        emit_leakage: false,
    };

    /// The emitted `define_arc` blocks: the text following each `define_arc`, truncated at its trailing
    /// blank line (the block separator) so the `define_leakage` section that follows the last arc block
    /// stays out of it, and trimmed. Block identity is the whole text, so the same arc emitted twice —
    /// once as its transition's general representative, once carrying its `-when` — yields two blocks
    /// differing by that one line.
    fn blocks(tcl: &str) -> Vec<String> {
        tcl.split("define_arc")
            .skip(1)
            .map(|b| match b.find("\n\n") {
                Some(off) => &b[..off],
                None => b,
            })
            .map(str::trim)
            .map(String::from)
            .collect()
    }

    /// [`blocks`], sorted — for comparing two code paths on the same input by the arcs they emit,
    /// independent of emission order.
    /// A block reduced to what it characterises: the arc kind, the pins it names, and which columns
    /// transition in which direction. The `-prevector`, the `-ic` levels and the vector's held `0`/`1`
    /// digits all name the state the run measured the arc at — a representative of that arc's context,
    /// and a walk free to claim a level in any order may reach one representative before another. Two
    /// runs emitting the same arcs need not agree on those.
    fn arc_shape(block: &str) -> String {
        let mut parts: Vec<String> = block
            .lines()
            .map(|l| l.trim().trim_end_matches('\\').trim_end())
            .filter_map(|l| {
                if let Some(v) = l.strip_prefix("-vector ") {
                    Some(format!("-vector {}", v.replace(['0', '1'], "_")))
                } else if l.starts_with("-type ")
                    || l.starts_with("-pin ")
                    || l.starts_with("-related_pin ")
                {
                    Some(l.to_string())
                } else {
                    None
                }
            })
            .collect();
        parts.sort();
        parts.join(" ")
    }

    /// Every block's [`arc_shape`], sorted — the set of arcs a run emits, blind to which state each was
    /// measured at.
    fn shaped_blocks(tcl: &str) -> Vec<String> {
        let mut b: Vec<String> = blocks(tcl).iter().map(|x| arc_shape(x)).collect();
        b.sort();
        b
    }

    fn sorted_blocks(tcl: &str) -> Vec<String> {
        let mut b = blocks(tcl);
        b.sort();
        b
    }

    /// Whether a block carries an ARC `-when` line. The line start is the discriminator: an arc's `-when`
    /// is its own indented line, whereas `define_leakage` — inherently `-when`-conditioned — rides its
    /// condition on the `define_leakage` line itself.
    fn has_when(block: &str) -> bool {
        block.lines().any(|l| l.trim_start().starts_with("-when"))
    }

    /// [`blocks`], each with its own `-when` line dropped — what maps a conditioned block onto the
    /// general block it otherwise duplicates.
    fn strip_when(tcl: &str) -> Vec<String> {
        blocks(tcl)
            .iter()
            .map(|b| {
                b.lines()
                    .filter(|l| !l.trim_start().starts_with("-when"))
                    .collect::<Vec<_>>()
                    .join("\n")
            })
            .collect()
    }

    /// The number of arcs the conditioned pass renders a block for, per class: every arc except one the
    /// general pass already emitted in the identical form — its transition's representative, which
    /// renders no `-when` to tell the two blocks apart.
    fn conditioned_counts(cell: &AnalysedCell) -> (usize, usize) {
        let general = generalised(
            &cell.arcs,
            |a| ArcIdentity::of(cell, a),
            |a| a.prevector.len(),
        );
        let general_hidden = generalised(&cell.hidden_arcs, ArcIdentity::of_hidden, |h| {
            h.prevector.len()
        });
        // Mirror the emitter's skip rule: a representative is redundant — its conditioned block is not
        // emitted — when its identity has a single firing or it renders no condition.
        let redundant = |map: &HashMap<usize, usize>, i: usize, no_when: bool| {
            map.get(&i).is_some_and(|&cases| cases == 1 || no_when)
        };
        (
            cell.arcs
                .iter()
                .enumerate()
                .filter(|(i, a)| !redundant(&general, *i, when_str(&a.end, &a.related).is_none()))
                .count(),
            cell.hidden_arcs
                .iter()
                .enumerate()
                .filter(|(i, h)| !redundant(&general_hidden, *i, hidden_when_str(h).is_none()))
                .count(),
        )
    }

    /// The A→Y arcs of `cell`, grouped by [`ArcIdentity`] — the source the general pass groups, so a
    /// premise read from it fails loudly on a fixture where nothing collides.
    fn ay_groups(cell: &AnalysedCell) -> HashMap<ArcIdentity, Vec<&Arc>> {
        let mut groups: HashMap<_, Vec<&Arc>> = HashMap::new();
        for a in cell
            .arcs
            .iter()
            .filter(|a| a.output == "Y" && a.related == "A")
        {
            groups.entry(ArcIdentity::of(cell, a)).or_default().push(a);
        }
        groups
    }

    /// COLLISION: the general pass collapses a transition measured from several contexts to a single
    /// block, on the DEFAULT output — the pass is unconditional, so no opt-in is involved.
    #[test]
    fn general_collapses_a_collision_to_one_block() {
        // PREMISE: at least two A→Y arcs share a transition key.
        let cell = analyse(TWO);
        let groups = ay_groups(&cell);
        let (_, group) = groups
            .iter()
            .find(|(_, g)| g.len() >= 2)
            .expect("premise: two A→Y arcs must share a transition key");

        // Exactly one member of the group is emitted, verbatim as its own arc.
        let default = cell_arcs_tcl(&cell, NO_LEAKAGE);
        eprintln!("{default}");
        let survivors = group
            .iter()
            .filter(|a| default.contains(&format_arc(&cell, &whole(&cell), a, false)))
            .count();
        assert_eq!(survivors, 1, "the colliding A→Y arcs collapse to one block");
    }

    /// SHORTEST PREVECTOR: the surviving member of a collapsed group keeps the shortest prevector, on the
    /// DEFAULT output. The minimum is read FROM `cell.arcs` (never hardcoded), so a length tie cannot
    /// make the assertion vacuous.
    #[test]
    fn general_keeps_the_shortest_prevector() {
        let cell = analyse(TWO);
        let groups = ay_groups(&cell);
        let (_, group) = groups
            .iter()
            .find(|(_, g)| g.len() >= 2)
            .expect("premise: a colliding A→Y group");
        let min_len = group
            .iter()
            .map(|a| a.prevector.len())
            .min()
            .expect("a non-empty group");

        // In the default output, the group's emitted member carries exactly `min_len` steps.
        let default = cell_arcs_tcl(&cell, NO_LEAKAGE);
        let survivor = group
            .iter()
            .find(|a| default.contains(&format_arc(&cell, &whole(&cell), a, false)))
            .expect("the surviving A→Y block");
        assert_eq!(
            survivor.prevector.len(),
            min_len,
            "the shortest-prevector member survives"
        );
    }

    /// CONTRACT: the general block count equals the number of DISTINCT transitions — fixture independent,
    /// and the primary pin on the hidden side where a hand-picked collision is not reliably
    /// constructible. Asserted on the DEFAULT output, where the general blocks are all there is.
    #[test]
    fn general_count_equals_distinct_transitions() {
        for src in [TWO, MAJ3] {
            let cell = analyse(src);
            let tcl = cell_arcs_tcl(&cell, NO_LEAKAGE);

            // Transition side: one block per distinct transition.
            let non_hidden = tcl
                .split("define_arc")
                .skip(1)
                .filter(|b| !b.contains("-type hidden"))
                .count();
            let transitions: HashSet<_> = cell
                .arcs
                .iter()
                .map(|a| ArcIdentity::of(&cell, a))
                .collect();
            assert_eq!(
                non_hidden,
                transitions.len(),
                "transition block count equals distinct transitions"
            );

            // Hidden side: one block per distinct (pin, edge) toggle event.
            let hidden = tcl.matches("-type hidden").count();
            let events: HashSet<_> = cell
                .hidden_arcs
                .iter()
                .map(ArcIdentity::of_hidden)
                .collect();
            assert_eq!(
                hidden,
                events.len(),
                "hidden block count equals distinct hidden events"
            );
        }
    }

    /// HELD-OUTPUT CONTEXTS COLLAPSE: a transparent-high D-latch holds Q at 0 or 1 across its two
    /// D-toggle contexts, but the toggle is ONE hidden event either way, so the general pass emits one
    /// `-pin D` block per edge. Selecting the class brings both contexts back, told apart by the held Q
    /// literal in their `-when` lines.
    #[test]
    fn hidden_general_arc_collapses_held_output_contexts() {
        const DLAT: &str = r#"
[[cell]]
name = "DLAT"
inputs = ["E", "D"]
[cell.outputs]
Q = "E*D + !E*Q"
"#;
        let cell = analyse(DLAT);
        // PREMISE: a D toggle is measured from several held-Q contexts, each of which renders a
        // condition.
        let mut contexts: HashMap<ArcIdentity, usize> = HashMap::new();
        for h in cell.hidden_arcs.iter().filter(|h| h.pin == "D") {
            assert!(
                hidden_when_str(h).is_some(),
                "premise: every D hidden arc renders a condition"
            );
            *contexts.entry(ArcIdentity::of_hidden(h)).or_default() += 1;
        }
        assert!(
            contexts.values().any(|n| *n >= 2),
            "premise: a D toggle fires from several held-Q contexts"
        );

        let d_hidden = |tcl: &str| -> Vec<String> {
            blocks(tcl)
                .into_iter()
                .filter(|b| b.contains("-type hidden") && b.contains("-pin D \\"))
                .collect()
        };
        let default = cell_arcs_tcl(&cell, NO_LEAKAGE);
        eprintln!("{default}");
        assert_eq!(
            d_hidden(&default).len(),
            contexts.len(),
            "one general -pin D block per edge"
        );
        assert!(
            d_hidden(&default).iter().all(|b| !has_when(b)),
            "a general hidden block carries no -when"
        );

        // Under `when = "hidden"` every held-Q context returns as its own conditioned block. Pinlist
        // order is {E D Q}, so D's own vector field is the toggle's edge.
        let selected = cell_arcs_tcl(&analyse(&when_variant(DLAT, "\"hidden\"")), NO_LEAKAGE);
        let d_field = |b: &str| -> String {
            b.lines()
                .find(|l| l.contains("-vector"))
                .and_then(|l| l.split('{').nth(1))
                .and_then(|v| v.split_whitespace().nth(1))
                .expect("a hidden block renders a -vector")
                .to_string()
        };
        let when_line = |b: &str| -> String {
            b.lines()
                .find(|l| l.trim_start().starts_with("-when"))
                .expect("a conditioned block renders a -when")
                .trim()
                .to_string()
        };
        for (event, n) in &contexts {
            let ArcIdentity::Hidden { edge, .. } = event else {
                panic!("a hidden arc's identity is ArcIdentity::Hidden: {event:?}");
            };
            let rf = edge.rf().to_string();
            let conditioned: Vec<String> = d_hidden(&selected)
                .into_iter()
                .filter(|b| has_when(b) && d_field(b) == rf)
                .collect();
            assert_eq!(
                conditioned.len(),
                *n,
                "every held-Q context of a D {rf} toggle returns as its own conditioned block"
            );
            let whens: BTreeSet<String> = conditioned.iter().map(|b| when_line(b)).collect();
            assert_eq!(
                whens.len(),
                conditioned.len(),
                "each held-Q context carries a distinct -when line"
            );
        }
    }

    // ---- Generalisation invariants: one per transition, a measured representative, shortest prevector ----

    /// The fixtures the generalisation invariants are asserted over: an OR-AND with several side-input
    /// contexts per transition, a stateless majority gate, a two-output cell whose contexts differ in
    /// INTERNAL state, and a latch whose contexts differ in the held output.
    const GENERALISED_FIXTURES: [&str; 4] = [OA22, MAJ3, TWO, DLAT];

    /// A block's text with the `define_arc` keyword stripped — the form [`blocks`] yields, so a rendering
    /// produced by [`format_arc`] can be compared against an emitted block.
    fn body(rendered: &str) -> String {
        rendered.trim_start_matches("define_arc").trim().to_string()
    }

    /// Each transition block of `tcl` mapped back to the arc in `cell.arcs` whose OWN rendering it is. A
    /// block matching no discovered arc panics: the general pass may only promote a measured firing, and
    /// a synthesised stimulus would be one nothing characterises.
    fn general_transition_arcs<'a>(cell: &'a AnalysedCell, tcl: &str) -> Vec<&'a Arc> {
        blocks(tcl)
            .iter()
            .filter(|b| !b.contains("-type hidden"))
            .map(|b| {
                cell.arcs
                    .iter()
                    .find(|a| body(&format_arc(cell, &whole(cell), a, false)) == *b)
                    .unwrap_or_else(|| {
                        panic!("a general block is no discovered arc's own rendering:\n{b}")
                    })
            })
            .collect()
    }

    /// [`general_transition_arcs`] for the hidden class.
    fn general_hidden_arcs<'a>(cell: &'a AnalysedCell, tcl: &str) -> Vec<&'a HiddenArc> {
        blocks(tcl)
            .iter()
            .filter(|b| b.contains("-type hidden"))
            .map(|b| {
                cell.hidden_arcs
                    .iter()
                    .find(|h| body(&format_hidden_arc(cell, &whole(cell), h, false)) == *b)
                    .unwrap_or_else(|| {
                        panic!("a general hidden block is no discovered arc's own rendering:\n{b}")
                    })
            })
            .collect()
    }

    /// ONE PER TRANSITION: on the DEFAULT output each of the cell's transitions emits exactly one general
    /// block, and every general block belongs to one of them — nothing lost, nothing duplicated. The
    /// hidden class carries the same invariant over its `(pin, edge)` toggle events.
    #[test]
    fn general_arcs_are_one_per_transition() {
        for src in GENERALISED_FIXTURES {
            let cell = analyse(src);
            let transitions: HashSet<_> = cell
                .arcs
                .iter()
                .map(|a| ArcIdentity::of(&cell, a))
                .collect();
            let events: HashSet<_> = cell
                .hidden_arcs
                .iter()
                .map(ArcIdentity::of_hidden)
                .collect();
            // PREMISE: the fixture fires some transition or toggle from several contexts, so there is
            // something to generalise over.
            assert!(
                transitions.len() + events.len() < cell.arcs.len() + cell.hidden_arcs.len(),
                "premise: {} fires from several contexts",
                cell.name.join(" ")
            );

            let tcl = cell_arcs_tcl(&cell, NO_LEAKAGE);
            let mut emitted: HashMap<_, usize> = HashMap::new();
            for a in general_transition_arcs(&cell, &tcl) {
                *emitted.entry(ArcIdentity::of(&cell, a)).or_default() += 1;
            }
            assert!(
                emitted.values().all(|n| *n == 1),
                "no transition emits two general blocks"
            );
            assert_eq!(
                emitted.into_keys().collect::<HashSet<_>>(),
                transitions,
                "one general block per transition"
            );

            let mut emitted_h: HashMap<_, usize> = HashMap::new();
            for h in general_hidden_arcs(&cell, &tcl) {
                *emitted_h.entry(ArcIdentity::of_hidden(h)).or_default() += 1;
            }
            assert!(
                emitted_h.values().all(|n| *n == 1),
                "no hidden event emits two general blocks"
            );
            assert_eq!(
                emitted_h.into_keys().collect::<HashSet<_>>(),
                events,
                "one general block per hidden toggle event"
            );
        }
    }

    /// REPRESENTATIVE IS A REAL FIRING: every general block is the rendering of an arc the pipeline
    /// discovered, so its `-prevector`/`-vector` pair is a stimulus that was measured together — the
    /// general pass promotes a firing, it never synthesises one.
    #[test]
    fn general_arcs_are_measured_firings() {
        for src in GENERALISED_FIXTURES {
            let cell = analyse(src);
            assert!(
                !cell.arcs.is_empty() && !cell.hidden_arcs.is_empty(),
                "premise: {} discovers arcs of both classes",
                cell.name.join(" ")
            );
            let tcl = cell_arcs_tcl(&cell, NO_LEAKAGE);
            // Both lookups panic on a block that is no discovered arc's own rendering.
            let transition = general_transition_arcs(&cell, &tcl).len();
            let hidden = general_hidden_arcs(&cell, &tcl).len();
            assert_eq!(
                transition + hidden,
                blocks(&tcl).len(),
                "every default block is a general arc of one of the two classes"
            );
        }
    }

    /// SHORTEST PREVECTOR AT THE TRANSITION GRAIN: each emitted representative carries the minimum
    /// prevector length of its whole transition group — the minimum read FROM `cell.arcs`, over the
    /// larger groups the transition grain forms.
    #[test]
    fn general_arcs_keep_the_shortest_prevector_per_transition() {
        for src in GENERALISED_FIXTURES {
            let cell = analyse(src);
            let mut shortest: HashMap<_, usize> = HashMap::new();
            for a in &cell.arcs {
                let best = shortest
                    .entry(ArcIdentity::of(&cell, a))
                    .or_insert(usize::MAX);
                *best = (*best).min(a.prevector.len());
            }
            let mut shortest_h: HashMap<_, usize> = HashMap::new();
            for h in &cell.hidden_arcs {
                let best = shortest_h
                    .entry(ArcIdentity::of_hidden(h))
                    .or_insert(usize::MAX);
                *best = (*best).min(h.prevector.len());
            }
            // PREMISE: some group holds several firings, so its minimum is a real choice between them.
            assert!(
                shortest.len() + shortest_h.len() < cell.arcs.len() + cell.hidden_arcs.len(),
                "premise: {} fires from several contexts",
                cell.name.join(" ")
            );

            let tcl = cell_arcs_tcl(&cell, NO_LEAKAGE);
            for a in general_transition_arcs(&cell, &tcl) {
                assert_eq!(
                    a.prevector.len(),
                    shortest[&ArcIdentity::of(&cell, a)],
                    "the representative carries its transition group's shortest prevector"
                );
            }
            for h in general_hidden_arcs(&cell, &tcl) {
                assert_eq!(
                    h.prevector.len(),
                    shortest_h[&ArcIdentity::of_hidden(h)],
                    "the representative carries its toggle event's shortest prevector"
                );
            }
        }
    }

    /// OA22: `A` rising drives `Y` rising from three side-input contexts. That is ONE transition, so the
    /// default output carries ONE unconditioned `A`→`Y` rise block; selecting every class brings all
    /// three firings back, each with its own condition.
    #[test]
    fn oa22_collapses_side_input_contexts_to_one_general_arc() {
        let cell = analyse(OA22);
        let a_rise_y_rise = |c: &AnalysedCell| -> Vec<usize> {
            c.arcs
                .iter()
                .enumerate()
                .filter(|(_, a)| {
                    a.output == "Y"
                        && a.related == "A"
                        && a.edge == Edge::Rise
                        && related_edge(a) == Edge::Rise
                })
                .map(|(i, _)| i)
                .collect()
        };
        // PREMISE: at least three arcs share the (Y rise, A rise) transition — `B` low with `(C,D)` at
        // `01`, `10` and `11`.
        let firings = a_rise_y_rise(&cell);
        assert!(
            firings.len() >= 3,
            "premise: A-rise→Y-rise fires from at least three side-input contexts, got {}",
            firings.len()
        );

        // Pinlist order is {A B C D Y}: A's vector field is index 0, Y's is index 4.
        let field = |b: &str, i: usize| -> String {
            b.lines()
                .find(|l| l.contains("-vector"))
                .and_then(|l| l.split('{').nth(1))
                .and_then(|v| v.split('}').next())
                .and_then(|v| v.split_whitespace().nth(i))
                .expect("a transition block renders a -vector")
                .to_string()
        };
        let default = cell_arcs_tcl(&cell, NO_LEAKAGE);
        eprintln!("{default}");
        let general: Vec<String> = blocks(&default)
            .into_iter()
            .filter(|b| {
                b.contains("-related_pin A \\")
                    && b.contains("-pin Y \\")
                    && field(b, 0) == "R"
                    && field(b, 4) == "R"
            })
            .collect();
        assert_eq!(general.len(), 1, "one general A-rise→Y-rise block");
        assert!(
            !has_when(&general[0]),
            "the general block carries no -when:\n{}",
            general[0]
        );

        // Under `when = true` every firing returns, each with its own distinct condition.
        let selected = analyse(&when_variant(OA22, "true"));
        let on = cell_arcs_tcl(&selected, NO_LEAKAGE);
        let mut whens: BTreeSet<String> = BTreeSet::new();
        for i in a_rise_y_rise(&selected) {
            let arc = &selected.arcs[i];
            let block = format_arc(&selected, &whole(&selected), arc, true);
            assert!(
                on.contains(&block),
                "a discovered A-rise→Y-rise firing is missing under `when`:\n{block}"
            );
            whens.insert(
                when_str(&arc.end, &arc.related).expect("each firing fixes the other inputs"),
            );
        }
        assert_eq!(
            whens.len(),
            firings.len(),
            "each firing carries its own distinct -when"
        );
    }

    /// GENERALISES OVER INTERNAL STATE: two `A`→`Y` firings of `TWO` agree on every input — the
    /// C-element `Z` renders as `X` in `Y`'s vector — and differ only in `Z`'s held state. They are one
    /// transition and collapse to one general block.
    #[test]
    fn general_arc_collapses_internal_state_contexts() {
        let cell = analyse(TWO);
        // PREMISE: two A→Y firings share a transition AND a rendered vector — every input agrees and `Z`
        // is `X` — and differ only in the internal state they were measured from.
        let mut contexts: HashMap<_, Vec<&Arc>> = HashMap::new();
        for a in cell
            .arcs
            .iter()
            .filter(|a| a.output == "Y" && a.related == "A")
        {
            contexts
                .entry((ArcIdentity::of(&cell, a), vector_str(&cell, a)))
                .or_default()
                .push(a);
        }
        let ((key, _), pair) = contexts
            .iter()
            .find(|(_, g)| g.len() >= 2)
            .expect("premise: two A→Y firings share a transition and a rendered vector");
        assert!(
            pair.windows(2).any(|w| w[0].start != w[1].start),
            "premise: the firings differ in the internal state they start from"
        );

        // The transition those two belong to emits ONE general block.
        let default = cell_arcs_tcl(&cell, NO_LEAKAGE);
        eprintln!("{default}");
        let emitted = ay_groups(&cell)[key]
            .iter()
            .filter(|a| default.contains(&format_arc(&cell, &whole(&cell), a, false)))
            .count();
        assert_eq!(
            emitted, 1,
            "the internal-state contexts collapse to one general block"
        );
    }

    /// ADDITIVE: selecting every class ADDS one conditioned block per conditioned arc on top of the
    /// general arcs — it removes nothing and rewrites nothing. Every default block is still there, each
    /// added block carries a `-when`, and at least one emitted vector appears twice, the two blocks
    /// differing solely by that line.
    #[test]
    fn when_all_adds_conditioned_blocks_on_top_of_the_general_arcs() {
        let cell = analyse(&when_variant(TWO, "true"));
        let (cond_t, cond_h) = conditioned_counts(&cell);
        assert!(
            cond_t >= 1 && cond_h >= 1,
            "premise: TWO conditions arcs of both classes"
        );
        let default = cell_arcs_tcl(&analyse(TWO), NO_LEAKAGE);
        let enabled = cell_arcs_tcl(&cell, NO_LEAKAGE);
        eprintln!("{enabled}");

        let default_blocks = blocks(&default);
        assert_eq!(
            blocks(&enabled).len(),
            default_blocks.len() + cond_t + cond_h,
            "one added block per conditioned arc"
        );

        // Multiset difference over sorted Vecs: every default block survives, and what remains is
        // exactly the conditioned blocks.
        let mut remaining = blocks(&enabled);
        remaining.sort();
        for b in &default_blocks {
            let i = remaining
                .iter()
                .position(|r| r == b)
                .unwrap_or_else(|| panic!("a default block is missing under `when`:\n{b}"));
            remaining.remove(i);
        }
        assert_eq!(remaining.len(), cond_t + cond_h);
        for b in &remaining {
            assert!(has_when(b), "every added block carries a -when:\n{b}");
        }

        // Every arc that renders a condition is emitted with it, whether or not the general pass already
        // emitted the same arc unconditioned — UNLESS it is the sole conditioned firing of its
        // transition, which the general block alone already characterises, so its conditioned copy is
        // suppressed. A conditioned block is present exactly when it is not suppressed.
        let general = generalised(
            &cell.arcs,
            |a| ArcIdentity::of(&cell, a),
            |a| a.prevector.len(),
        );
        for (i, a) in cell.arcs.iter().enumerate() {
            let conditioned = format_arc(&cell, &whole(&cell), a, true);
            if !has_when(&conditioned) {
                continue;
            }
            // A representative that stands for a single case has its conditioned copy suppressed.
            let suppressed = general.get(&i).is_some_and(|&cases| cases == 1);
            assert_eq!(
                enabled.contains(&conditioned),
                !suppressed,
                "a conditioned arc is emitted iff its transition has more than one case:\n{conditioned}"
            );
        }

        // At least one arc is emitted twice — once as its transition's general representative, once carrying its
        // condition — the two blocks differing only by the `-when` line.
        let stripped = strip_when(&enabled);
        let doubled = stripped
            .iter()
            .any(|s| stripped.iter().filter(|o| *o == s).count() >= 2);
        assert!(
            doubled,
            "an arc appears both as a general arc and with its -when"
        );
    }

    /// SELECTIVITY: `--when=transition` adds the transition class's conditioned blocks and leaves the
    /// hidden class at its general arcs, and its mirror `--when=hidden` leaves the transition class at
    /// its general arcs.
    #[test]
    fn when_one_class_adds_only_that_class() {
        let cell = analyse(TWO);
        let (cond_t, cond_h) = conditioned_counts(&cell);
        assert!(
            cond_t >= 1 && cond_h >= 1,
            "premise: TWO conditions arcs of both classes"
        );

        let non_hidden = |tcl: &str| {
            blocks(tcl)
                .iter()
                .filter(|b| !b.contains("-type hidden"))
                .count()
        };
        let hidden = |tcl: &str| {
            blocks(tcl)
                .iter()
                .filter(|b| b.contains("-type hidden"))
                .count()
        };
        let non_hidden_when = |tcl: &str| {
            blocks(tcl)
                .iter()
                .filter(|b| !b.contains("-type hidden") && has_when(b))
                .count()
        };
        let hidden_when = |tcl: &str| {
            blocks(tcl)
                .iter()
                .filter(|b| b.contains("-type hidden") && has_when(b))
                .count()
        };

        let default = cell_arcs_tcl(&cell, NO_LEAKAGE);
        assert_eq!(non_hidden_when(&default), 0);
        assert_eq!(hidden_when(&default), 0);

        // --when=transition: the transition class gains its conditioned blocks; the hidden class is untouched.
        let t_on = cell_arcs_tcl(&analyse(&when_variant(TWO, "\"transition\"")), NO_LEAKAGE);
        assert_eq!(
            non_hidden(&t_on),
            non_hidden(&default) + cond_t,
            "one added transition block per conditioned transition arc"
        );
        assert_eq!(non_hidden_when(&t_on), cond_t);
        assert_eq!(
            hidden(&t_on),
            hidden(&default),
            "hidden block count is unchanged by --when=transition"
        );
        assert_eq!(hidden_when(&t_on), 0, "no hidden -when is added");

        // Mirror --when=hidden: the transition class is untouched.
        let h_on = cell_arcs_tcl(&analyse(&when_variant(TWO, "\"hidden\"")), NO_LEAKAGE);
        assert_eq!(
            hidden(&h_on),
            hidden(&default) + cond_h,
            "one added hidden block per conditioned hidden arc"
        );
        assert_eq!(hidden_when(&h_on), cond_h);
        assert_eq!(
            non_hidden(&h_on),
            non_hidden(&default),
            "transition block count is unchanged by --when=hidden"
        );
        assert_eq!(non_hidden_when(&h_on), 0, "no transition -when is added");
    }

    /// A transition that fires from a single context is fully characterised by its general block, so
    /// selecting the transition class adds no conditioned copy for it — the general block already stands
    /// for that one context, and the `-when` would tell Liberate nothing more.
    #[test]
    fn single_context_transition_suppresses_its_conditioned_copy() {
        let cell = analyse(&when_variant(AND2, "\"transition\""));
        // premise: every AND2 transition fires from exactly one context.
        let mut firings: HashMap<ArcIdentity, usize> = HashMap::new();
        for a in &cell.arcs {
            *firings.entry(ArcIdentity::of(&cell, a)).or_default() += 1;
        }
        assert!(
            !firings.is_empty() && firings.values().all(|&n| n == 1),
            "premise: AND2's transitions each fire from a single context"
        );

        let enabled = cell_arcs_tcl(&cell, NO_LEAKAGE);
        eprintln!("{enabled}");
        let transition_when = blocks(&enabled)
            .iter()
            .filter(|b| !b.contains("-type hidden") && has_when(b))
            .count();
        assert_eq!(
            transition_when, 0,
            "no conditioned transition block is emitted for a single-context transition"
        );

        // Nothing is dropped — the transition class still emits exactly its general blocks.
        let default = cell_arcs_tcl(&analyse(AND2), NO_LEAKAGE);
        let transition_blocks = |t: &str| {
            let mut v: Vec<_> = blocks(t)
                .into_iter()
                .filter(|b| !b.contains("-type hidden"))
                .collect();
            v.sort();
            v
        };
        assert_eq!(
            transition_blocks(&enabled),
            transition_blocks(&default),
            "the transition class equals its general arcs: only the redundant conditioned copies are gone"
        );
    }

    /// `--no-internal` outranks the class selection: with the hidden class selected but internal-power
    /// arcs suppressed, neither the general nor the conditioned hidden pass emits anything.
    #[test]
    fn no_internal_suppresses_hidden_blocks_under_when() {
        let cell = analyse(&when_variant(TWO, "true"));
        let tcl = cell_arcs_tcl(
            &cell,
            ArcsTclOptions {
                emit_internal: false,
                emit_leakage: false,
            },
        );
        assert_eq!(tcl.matches("-type hidden").count(), 0);
        assert!(
            !blocks(&tcl).is_empty(),
            "the transition blocks are still emitted"
        );
    }

    /// SINGLE INPUT: an arc whose related pin is the cell's only input renders NO condition
    /// (`when_str` is `None`), so where it is also the general pass's representative the conditioned
    /// pass skips it — the block it would add is the one already emitted. Every INV arc is its own
    /// transition's representative, so selecting every class changes nothing.
    #[test]
    fn when_skips_a_conditionless_general_representative() {
        const INV: &str = r#"
[[cell]]
name = "INV"
inputs = ["A"]
[cell.outputs]
Y = "!A"
"#;
        let cell = analyse(&when_variant(INV, "true"));
        let (cond_t, cond_h) = conditioned_counts(&cell);
        assert_eq!(
            (cond_t, cond_h),
            (0, 0),
            "premise: no INV arc renders a condition"
        );
        // PREMISE: nothing collides, so every arc IS its transition's representative — the case the skip
        // is for. A colliding fixture would exercise the other branch instead.
        let keys: HashSet<_> = cell
            .arcs
            .iter()
            .map(|a| ArcIdentity::of(&cell, a))
            .collect();
        assert_eq!(
            keys.len(),
            cell.arcs.len(),
            "premise: no INV arc shares a transition with another"
        );
        let default = cell_arcs_tcl(&analyse(INV), NO_LEAKAGE);
        let enabled = cell_arcs_tcl(&cell, NO_LEAKAGE);
        eprintln!("{enabled}");
        assert_eq!(
            sorted_blocks(&enabled),
            sorted_blocks(&default),
            "an unconditional arc is emitted once, by the general pass alone"
        );
        let emitted = blocks(&default);
        assert!(!emitted.is_empty(), "INV emits arcs");
        let unique: BTreeSet<&String> = emitted.iter().collect();
        assert_eq!(unique.len(), emitted.len(), "no block is emitted twice");
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

    /// A rising-edge DFF opting into constraint arcs: one state-holding cell emitting all three block
    /// kinds — transition, hidden and constraint — so one pass over its blocks covers every `-ic`
    /// emission site.
    const IC_DFF: &str = r#"
[[cell]]
name = "DFF"
inputs = ["CLK", "D"]
clock = ["CLK"]
constraint_arcs = true
[cell.internal]
M = "!CLK*D + CLK*M"
[cell.outputs]
Q = "CLK*M + !CLK*Q"
"#;

    /// The columns on a block's `-ic` line, or `None` when the block renders no `-ic`. The word between
    /// the line's double quotes is split the way Liberate reads it — by the Tcl list rules, so
    /// whitespace inside a brace group belongs to its element rather than separating two — and each
    /// column comes back as the text emitted for it, braces and all. The double quoting is asserted
    /// here: bracing the word instead would stop Tcl substituting a `$VDD`-style expression.
    fn ic_values(block: &str) -> Option<Vec<&str>> {
        let line = block.lines().find(|l| l.trim_start().starts_with("-ic "))?;
        let open = line.find('"').expect("-ic values are double-quoted");
        let close = line.rfind('"').expect("-ic renders a closing quote");
        assert!(open < close, "-ic renders a pair of quotes: {line}");
        let word = &line[open + 1..close];
        let mut columns = Vec::new();
        let mut depth = 0usize;
        let mut start = None;
        for (i, c) in word.char_indices() {
            match c {
                '{' => depth += 1,
                '}' => depth = depth.saturating_sub(1),
                _ if c.is_whitespace() && depth == 0 => {
                    if let Some(s) = start.take() {
                        columns.push(&word[s..i]);
                    }
                    continue;
                }
                _ => {}
            }
            start.get_or_insert(i);
        }
        if let Some(s) = start {
            columns.push(&word[s..]);
        }
        Some(columns)
    }

    /// The pins on a block's `-pinlist` line, in column order.
    fn pinlist_of(block: &str) -> Vec<&str> {
        braced(block, "-pinlist")
            .expect("every block renders a -pinlist")
            .split_whitespace()
            .collect()
    }

    /// The text between the braces on the block's `-<tag>` line.
    fn braced<'a>(block: &'a str, tag: &str) -> Option<&'a str> {
        let line = block
            .lines()
            .find(|l| l.trim_start().starts_with(&format!("{tag} ")))?;
        let open = line.find('{')?;
        let close = line.rfind('}')?;
        Some(&line[open + 1..close])
    }

    #[test]
    fn state_holding_blocks_carry_a_pinlist_aligned_ic() {
        // Every block a state-holding cell emits states its start condition: one voltage per `-pinlist`
        // pin, in the same column order. The input columns are read back against the prevector's last
        // step — the state the cell is in when the measured vector begins — which is what pins the
        // alignment rather than merely the count.
        let cell = analyse(IC_DFF);
        assert!(cell.state_holding);
        let tcl = cell_arcs_tcl(&cell, ArcsTclOptions::default());
        eprintln!("{tcl}");
        let voltage = |level: bool| cell.voltages.of(level);
        let (mut transitions, mut hidden, mut constraints) = (0, 0, 0);
        for block in blocks(&tcl) {
            let pins = pinlist_of(&block);
            let ic = ic_values(&block)
                .unwrap_or_else(|| panic!("a state-holding cell's block carries an -ic:\n{block}"));
            assert_eq!(ic.len(), pins.len(), "one -ic entry per pin:\n{block}");
            let last = braced(&block, "-prevector")
                .expect("every block renders a -prevector")
                .split_whitespace()
                .last()
                .expect("a prevector has at least one step")
                .to_owned();
            assert_eq!(last.len(), cell.inputs.len());
            for (i, step) in last.chars().enumerate() {
                assert_eq!(
                    ic[i],
                    voltage(step == '1'),
                    "{} starts where the prevector leaves it:\n{block}",
                    pins[i]
                );
            }
            for entry in &ic[cell.inputs.len()..] {
                assert!(
                    [voltage(false), voltage(true)].contains(entry),
                    "an output starts at a logic voltage:\n{block}"
                );
            }
            if block.contains("-type hidden") {
                hidden += 1;
            } else if block.contains("-type setup") || block.contains("-type hold") {
                constraints += 1;
            } else {
                transitions += 1;
            }
        }
        assert!(
            transitions > 0 && hidden > 0 && constraints > 0,
            "the fixture covers all three block kinds, got {transitions}/{hidden}/{constraints}"
        );
    }

    #[test]
    fn combinational_cell_emits_no_ic() {
        // A cell with no state loses nothing when Liberate discards the prevector simulation, so no
        // block states a start condition.
        for src in [
            AND2,
            r#"
[[cell]]
name = "INV"
inputs = ["A"]
[cell.outputs]
Y = "!A"
"#,
        ] {
            let cell = analyse(src);
            assert!(!cell.state_holding);
            let tcl = cell_arcs_tcl(&cell, ArcsTclOptions::default());
            assert!(
                !tcl.contains("-ic"),
                "combinational cell emitted -ic:\n{tcl}"
            );
        }
    }

    #[test]
    fn ic_is_the_only_line_the_gate_adds() {
        // `-ic` is purely additive: dropping the `-ic` lines from a state-holding cell's blocks yields
        // exactly what the same cell renders with the gate clear, line for line.
        let mut cell = analyse(IC_DFF);
        let gated = cell_arcs_tcl(&cell, ArcsTclOptions::default());
        cell.state_holding = false;
        let ungated = cell_arcs_tcl(&cell, ArcsTclOptions::default());
        assert!(gated.contains("-ic \""));
        assert!(!ungated.contains("-ic"));
        let stripped: String = gated
            .lines()
            .filter(|l| !l.trim_start().starts_with("-ic "))
            .map(|l| format!("{l}\n"))
            .collect();
        assert_eq!(stripped, ungated);
    }

    #[test]
    fn logic_voltage_overrides_reach_the_ic_text() {
        // The level expressions are written into `-ic` verbatim — they are Tcl value fragments, so a
        // variable reference reaches Liberate as one.
        let cell = analyse(&IC_DFF.replace(
            "constraint_arcs = true",
            "constraint_arcs = true\nlogic_low = \"GND\"\nlogic_high = \"$VDDH\"",
        ));
        let tcl = cell_arcs_tcl(&cell, ArcsTclOptions::default());
        eprintln!("{tcl}");
        let mut entries: BTreeSet<String> = BTreeSet::new();
        for block in blocks(&tcl) {
            let ic = ic_values(&block).expect("a state-holding cell's block carries an -ic");
            entries.extend(ic.into_iter().map(str::to_owned));
        }
        assert_eq!(
            entries,
            BTreeSet::from(["GND".to_owned(), "$VDDH".to_owned()])
        );
    }

    #[test]
    fn a_one_element_logic_voltage_is_written_as_it_stands() {
        // The forms Liberate's list split already reads as one column: a bare word, a number in either
        // notation, a variable reference either way round, a namespaced one, and a value the spec wrote
        // as one balanced brace group. `0` and `$VDD` are the defaults, so this is also what keeps every
        // emitted artifact free of braces it never had.
        for value in [
            "0",
            "0.99",
            "-0.5",
            "1e-3",
            "GND",
            "VDD_H",
            "$VDD",
            "${VDD}",
            "$::VDD",
            "{$VDD * 0.9}",
            "{}",
        ] {
            assert_eq!(ic_column(value), value, "{value:?} is already one column");
        }
    }

    #[test]
    fn any_other_logic_voltage_is_wrapped_into_one_element() {
        // Whitespace splits the column, a bracket resolves to whatever the command returns, two groups
        // are two elements and an empty value is none: the wrap makes each of them exactly one.
        for (value, column) in [
            ("$VDD * 0.9", "{$VDD * 0.9}"),
            ("$VDD\t0.9", "{$VDD\t0.9}"),
            ("[expr $VDD*0.9]", "{[expr $VDD*0.9]}"),
            ("{a} {b}", "{{a} {b}}"),
            ("${a b}", "{${a b}}"),
            ("", "{}"),
        ] {
            assert_eq!(ic_column(value), column, "{value:?} is wrapped");
        }
    }

    #[test]
    fn a_double_quote_in_a_logic_voltage_is_escaped() {
        // A quote would close the `-ic` word wherever it sat, so it goes out escaped — inside the wrap
        // where the expression's own text goes, and inside a group the spec braced itself.
        assert_eq!(ic_column("a\"b"), "{a\\\"b}");
        assert_eq!(ic_column("{a\"b}"), "{a\\\"b}");
        let cell = analyse(&IC_DFF.replace(
            "constraint_arcs = true",
            "constraint_arcs = true\nlogic_high = \"a\\\"b\"",
        ));
        let tcl = cell_arcs_tcl(&cell, ArcsTclOptions::default());
        eprintln!("{tcl}");
        for block in blocks(&tcl) {
            let line = block
                .lines()
                .map(str::trim)
                .find(|l| l.starts_with("-ic "))
                .unwrap_or_else(|| panic!("a state-holding cell's block carries an -ic:\n{block}"));
            assert_eq!(
                line.matches('"').count() - line.matches("\\\"").count(),
                2,
                "the escaped quotes leave the word its own pair:\n{line}"
            );
        }
    }

    /// The expressions that need more than the wrap to hold their column, each with the text the
    /// emitter writes between the `-ic` quotes. Every one of them is read back through real Tcl by
    /// [`tclsh_reads_an_awkward_logic_voltage_as_one_column_per_pin`], which is where the doubling was
    /// established; the pairs here pin it without an interpreter to hand.
    const AWKWARD_VOLTAGES: [(&str, &str); 17] = [
        // The backslash the word's substitution would otherwise spend on the quote after it.
        (r#"$V\"X"#, r#"{$V\\\\\"X}"#),
        // A brace with nothing to close it, either way round, alone or amid text.
        (r"{$VDD", r"{\\{$VDD}"),
        (r"$VDD}", r"{$VDD\\}}"),
        (r"{", r"{\\{}"),
        (r"}", r"{\\}}"),
        (r"{{{", r"{\\{\\{\\{}"),
        (r"}{", r"{\\}\\{}"),
        // A matched pair stands: only the stray close brace is escaped.
        (r"{a}}", r"{{a}\\}}"),
        // A backslash of the expression's own, before a brace and standing alone.
        (r"a\{b", r"{a\\\\\\{b}"),
        (r"\", r"{\\\\}"),
        // The escape reaching the column as text: `\n` is a backslash and an `n`, not a newline.
        (r"x\ny", r"{x\\\\ny}"),
        // Braces and nothing else, balanced: one empty element, written as it stands.
        (r"{}", r"{}"),
        // A command substitution with nothing to close it, alone and amid text: one backslash, the
        // bracket reaching the column on its own.
        (r"[expr", r"{\[expr}"),
        (r"a[b", r"{a\[b}"),
        (r"[[", r"{\[\[}"),
        // The close bracket starts nothing, so it stands as written.
        (r"a]b", r"{a]b}"),
        // A substitution that does close is how an expression names its level: left alone.
        (r"[expr $VDD*0.9]", r"{[expr $VDD*0.9]}"),
    ];

    #[test]
    fn an_awkward_logic_voltage_is_escaped_into_one_element() {
        for (value, column) in AWKWARD_VOLTAGES {
            assert_eq!(ic_column(value), column, "{value:?} holds its column");
        }
    }

    /// Run `script` through `tclsh`, or `None` where the interpreter is not installed.
    fn tclsh(script: &str) -> Option<std::process::Output> {
        let mut child = match std::process::Command::new("tclsh")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
        {
            Ok(child) => child,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return None,
            Err(e) => panic!("spawn tclsh: {e}"),
        };
        use std::io::Write;
        child
            .stdin
            .as_mut()
            .expect("piped stdin")
            .write_all(script.as_bytes())
            .expect("feed tclsh its script");
        Some(child.wait_with_output().expect("tclsh runs to completion"))
    }

    #[test]
    fn tclsh_reads_an_awkward_logic_voltage_as_one_column_per_pin() {
        // The emitter's own output, read the way Liberate reads it: Tcl parses the `define_arc` command
        // — substituting the word, which is where an unescaped quote or backslash ends the line early —
        // and the stub then splits the `-ic` argument by the list rules that decide the columns. Both
        // logic levels carry the awkward expression, so every column is one of the cases under test.
        // The run is skipped where `tclsh` is not installed, `an_awkward_logic_voltage_is_escaped_into_
        // one_element` pinning the emitted text there.
        //
        // The verdict is read off the interpreter's own output rather than its exit status: a script fed
        // through stdin leaves tclsh exiting zero whether or not a command in it failed. Anything on
        // stderr is a Tcl complaint, and the closing `end` marker is what says the whole script ran.
        for (value, _) in AWKWARD_VOLTAGES {
            let cell = analyse(&IC_DFF.replace(
                "constraint_arcs = true",
                &format!("constraint_arcs = true\nlogic_low = {value:?}\nlogic_high = {value:?}"),
            ));
            let script = format!(
                "set VDD 1.08\n\
                 set V 0.5\n\
                 proc define_arc args {{\n\
                 \x20   set ic [lindex $args [expr {{[lsearch -exact $args -ic] + 1}}]]\n\
                 \x20   set pins [lindex $args [expr {{[lsearch -exact $args -pinlist] + 1}}]]\n\
                 \x20   puts \"[llength $ic] [llength $pins]\"\n\
                 }}\n\
                 proc unknown args {{ return }}\n\
                 {}\n\
                 puts end\n",
                cell_arcs_tcl(&cell, ArcsTclOptions::default())
            );
            let Some(out) = tclsh(&script) else {
                eprintln!("tclsh is not installed: skipping the Tcl read-back");
                return;
            };
            let stderr = String::from_utf8_lossy(&out.stderr);
            assert!(
                stderr.is_empty(),
                "Tcl reads the {value:?} line: {stderr}\n{script}"
            );
            let stdout = String::from_utf8_lossy(&out.stdout);
            let counts = stdout
                .strip_suffix("end\n")
                .unwrap_or_else(|| panic!("the {value:?} script runs to its end:\n{stdout}"));
            let mut arcs = 0;
            for line in counts.lines() {
                let (ic, pins) = line.split_once(' ').expect("one count pair per arc");
                assert_eq!(
                    ic, pins,
                    "{value:?} leaves one -ic column per pin:\n{script}"
                );
                arcs += 1;
            }
            assert!(arcs > 0, "the fixture emits arcs to read back:\n{script}");
        }
    }

    #[test]
    fn a_wrapped_logic_voltage_still_leaves_one_column_per_pin() {
        // The point of the wrap, on the cell that has the most columns to shift: an exposed node sits
        // between the inputs and the outputs, so a voltage that split would move it off its pin.
        let cell = analyse(&C2_EXPOSED.replace(
            "constraint_arcs = true",
            "constraint_arcs = true\nlogic_high = \"$VDD * 0.9\"",
        ));
        let tcl = cell_arcs_tcl(&cell, NO_LEAKAGE);
        eprintln!("{tcl}");
        let mut wrapped = 0;
        for block in blocks(&tcl) {
            let ic = ic_values(&block).expect("a state-holding cell's block carries an -ic");
            assert_eq!(
                ic.len(),
                pinlist_of(&block).len(),
                "one -ic column per pin:\n{block}"
            );
            wrapped += ic.iter().filter(|c| **c == "{$VDD * 0.9}").count();
        }
        assert!(wrapped > 0, "the high level reaches the -ic lines:\n{tcl}");
    }

    // ---- Exposed internal nodes: their own pinlist column, and what each block kind puts in it ----

    /// The worked C-element written around its internal node — `QN = !(A*B + Q*(A+B))`, `Q = !QN` — with
    /// `QN` exposed and constraint arcs opted into, so one fixture emits all three block kinds. The
    /// output inverts the exposed node, so every measured arc moves it.
    const C2_EXPOSED: &str = r#"
[[cell]]
name = "C2EXP"
inputs = ["A", "B"]
expose = ["QN"]
constraint_arcs = true
[cell.internal]
QN = "!(A*B + Q*(A+B))"
[cell.outputs]
Q = "!QN"
"#;

    /// A master-slave DFF exposing its master latch `M`, with constraint arcs opted into. `M` is the
    /// case an output column cannot stand in for: with the clock low the master is transparent, so a `D`
    /// toggle moves it while `Q` holds.
    const DFF_EXPOSED_MASTER: &str = r#"
[[cell]]
name = "DFFM"
inputs = ["CLK", "D"]
clock = ["CLK"]
constraint_arcs = true
expose = ["M"]
[cell.internal]
M = "!CLK*D + CLK*M"
[cell.outputs]
Q = "CLK*M + !CLK*Q"
"#;

    /// An AND term exposed on its way to an inverter — `W = A*B`, `Y = !W` — holding no state, so `W`
    /// is evaluated fresh on every arc rather than read off a state column.
    const AN2_EXPOSED: &str = r#"
[[cell]]
name = "AN2"
inputs = ["A", "B"]
expose = ["W"]
[cell.internal]
W = "A*B"
[cell.outputs]
Y = "!W"
"#;

    /// The column `pin` occupies in the block's `-pinlist` — the shared position `-vector` and `-ic` are
    /// read at, all three lines being one walk over the same order.
    fn column_of(block: &str, pin: &str) -> usize {
        pinlist_of(block)
            .iter()
            .position(|p| *p == pin)
            .unwrap_or_else(|| panic!("{pin} appears in the block's -pinlist:\n{block}"))
    }

    /// The block's `-vector` symbols, in column order.
    fn vector_values(block: &str) -> Vec<&str> {
        braced(block, "-vector")
            .expect("every block renders a -vector")
            .split_whitespace()
            .collect()
    }

    #[test]
    fn an_exposed_node_renders_its_own_pinlist_vector_and_ic_columns() {
        // The authoritative form: `B` rising out of `{A=1, B=0}` drives `Q` up. `QN` falls with it in
        // the cell, but its `-vector` column is `X` — a column there forces the node, and the cell is
        // what drives it — while `-ic` states the level it starts from.
        let cell = analyse(C2_EXPOSED);
        let tcl = cell_arcs_tcl(&cell, NO_LEAKAGE);
        eprintln!("{tcl}");
        let block = blocks(&tcl)
            .into_iter()
            .find(|b| {
                !b.contains("-type hidden")
                    && b.contains("-related_pin B")
                    && b.contains("-pin Q")
                    && vector_values(b)[column_of(b, "B")] == "R"
            })
            .expect("the B-rise → Q-rise block");
        assert_eq!(pinlist_of(&block), ["A", "B", "QN", "Q"]);
        assert_eq!(vector_values(&block), ["1", "R", "X", "R"]);
        assert_eq!(
            ic_values(&block).expect("a state-holding cell's block carries an -ic"),
            ["$VDD", "0", "$VDD", "0"],
        );
        assert!(block.contains("-prevector {00 10}"));
    }

    #[test]
    fn a_combinational_exposed_node_renders_its_pinlist_column_with_no_ic() {
        // AN2 holds no state, so its exposed AND term is evaluated fresh on every arc rather than read
        // off a state column — the machinery a state-variable exposed node never exercises. Held at
        // A=1, B rising drives the AND term up with it and the inverted output down; the AND term's own
        // column is `X` all the same, the cell being what drives it.
        let cell = analyse(AN2_EXPOSED);
        assert!(!cell.state_holding, "AN2 is plain combinational logic");
        let tcl = cell_arcs_tcl(&cell, NO_LEAKAGE);
        eprintln!("{tcl}");
        let block = blocks(&tcl)
            .into_iter()
            .find(|b| {
                b.contains("-related_pin B")
                    && b.contains("-pin Y")
                    && vector_values(b)[column_of(b, "B")] == "R"
            })
            .expect("the B-rise → Y-fall block");
        assert_eq!(pinlist_of(&block), ["A", "B", "W", "Y"]);
        assert_eq!(vector_values(&block), ["1", "R", "X", "F"]);
        assert!(
            ic_values(&block).is_none(),
            "a cell holding no state renders no -ic:\n{block}"
        );
    }

    #[test]
    fn a_hidden_arc_leaves_the_exposed_master_unstated_and_still_initialises_it() {
        // A hidden arc is one no output follows. The transparent master DOES move across it, tracking
        // `D` while the clock is low — but a `-vector` column forces the node it names, and forcing an
        // internal one would override the behaviour the arc exists to measure. So the column reads `X`
        // and `-ic` carries the level the master starts from. The general pass keeps one representative
        // per toggle event, so the hidden class is selected to bring every measured firing out.
        let cell = analyse(&when_variant(DFF_EXPOSED_MASTER, "\"hidden\""));
        let tcl = cell_arcs_tcl(&cell, NO_LEAKAGE);
        eprintln!("{tcl}");
        let hidden: Vec<String> = blocks(&tcl)
            .into_iter()
            .filter(|b| b.contains("-type hidden"))
            .collect();
        assert!(!hidden.is_empty(), "the fixture emits hidden arcs:\n{tcl}");
        for block in &hidden {
            assert_eq!(
                vector_values(block)[column_of(block, "M")],
                "X",
                "the exposed master is never forced by the vector:\n{block}"
            );
            assert!(
                ["0", "1"].contains(&vector_values(block)[column_of(block, "Q")]),
                "every output stays pinned at its held level across a hidden arc:\n{block}"
            );
            let start = ic_values(block).expect("a state-holding cell's block carries an -ic")
                [column_of(block, "M")];
            assert!(
                [cell.voltages.of(false), cell.voltages.of(true)].contains(&start),
                "the unstated column still starts at a stated level:\n{block}"
            );
        }
    }

    #[test]
    fn a_constraint_block_leaves_the_exposed_column_unstated_and_still_initialises_it() {
        // A constraint block measures nothing the cell does in response to its two edges, so it renders
        // the exposed column the same `X` it renders every output — while `-ic` carries the level the
        // node actually starts at, which is what Liberate needs to prepare the cell either way.
        let cell = analyse(C2_EXPOSED);
        let arc = cell.arc_view();
        assert!(
            !arc.constraints.is_empty(),
            "premise: the C-element's racing inputs are constrained"
        );
        let tcl = cell_arcs_tcl(&cell, NO_LEAKAGE);
        eprintln!("{tcl}");
        for c in &arc.constraints {
            let rendered = format_constraint(arc, &whole(arc), c);
            assert!(
                tcl.contains(&rendered),
                "the emitted Tcl carries this constraint's blocks:\n{rendered}"
            );
            let level = c
                .levels
                .exposed
                .iter()
                .find(|e| e.node == "QN")
                .expect("the constraint's levels define the exposed node");
            for block in blocks(&rendered) {
                let i = column_of(&block, "QN");
                assert_eq!(
                    vector_values(&block)[i],
                    "X",
                    "the exposed column is unstated:\n{block}"
                );
                assert_eq!(
                    ic_values(&block).expect("a state-holding cell's block carries an -ic")[i],
                    cell.voltages.of(level.start),
                    "the exposed column starts at its measured level:\n{block}"
                );
            }
        }
    }

    #[test]
    fn every_block_kind_aligns_its_vector_and_ic_with_the_exposed_pinlist() {
        // `-pinlist`, `-vector` and `-ic` are one walk over one order, so each block of each kind renders
        // exactly as many symbols and voltages as it lists columns, the exposed ones included.
        for src in [C2_EXPOSED, DFF_EXPOSED_MASTER] {
            let cell = analyse(src);
            assert!(
                cell.state_holding,
                "the fixture holds state, so it emits -ic"
            );
            let exposed: Vec<&str> = cell.exposed.iter().map(Symbol::as_str).collect();
            let tcl = cell_arcs_tcl(&cell, ArcsTclOptions::default());
            eprintln!("{tcl}");
            let (mut transitions, mut hidden, mut constraints) = (0, 0, 0);
            for block in blocks(&tcl) {
                let pins = pinlist_of(&block);
                assert_eq!(
                    pins.len(),
                    cell.inputs.len() + exposed.len() + cell.outputs.len(),
                    "the pinlist is inputs, exposed nodes and outputs:\n{block}"
                );
                for node in &exposed {
                    assert!(pins.contains(node), "{node} has a column:\n{block}");
                }
                assert_eq!(
                    vector_values(&block).len(),
                    pins.len(),
                    "one -vector symbol per column:\n{block}"
                );
                assert_eq!(
                    ic_values(&block)
                        .unwrap_or_else(|| panic!("every block carries an -ic:\n{block}"))
                        .len(),
                    pins.len(),
                    "one -ic voltage per column:\n{block}"
                );
                if block.contains("-type hidden") {
                    hidden += 1;
                } else if ["setup", "hold"]
                    .iter()
                    .any(|k| block.contains(&format!("-type {k} ")))
                    || block.contains("non_seq")
                {
                    constraints += 1;
                } else {
                    transitions += 1;
                }
            }
            assert!(
                transitions > 0 && hidden > 0 && constraints > 0,
                "the fixture covers all three block kinds, got {transitions}/{hidden}/{constraints}"
            );
        }
    }

    #[test]
    fn a_cell_exposing_nothing_keeps_a_pin_only_pinlist() {
        // Nothing exposed, nothing added: the arc pinlist is the cell's pins, so every rendered block
        // keeps the shape it had before exposed columns existed.
        for src in [AND2, MAJ3, TWO, OA22, IC_DFF] {
            let cell = analyse(src);
            assert!(cell.exposed.is_empty());
            assert_eq!(
                arc_pinlist_str(&cell, &whole(&cell).exposed),
                pinlist_str(&cell)
            );
            let tcl = cell_arcs_tcl(&cell, ArcsTclOptions::default());
            for block in blocks(&tcl) {
                assert_eq!(
                    pinlist_of(&block).len(),
                    cell.inputs.len() + cell.outputs.len(),
                    "an exposure-free block lists the cell's pins and nothing else:\n{block}"
                );
            }
        }
    }

    #[test]
    fn an_exposed_node_is_never_an_arc_source_or_target() {
        // `-related_pin` and `-pin` are drawn from the primary inputs and the output pins alone, so an
        // exposed internal cannot appear in either however many columns it earns.
        for src in [C2_EXPOSED, DFF_EXPOSED_MASTER] {
            let cell = analyse(src);
            let tcl = cell_arcs_tcl(&cell, ArcsTclOptions::default());
            for line in tcl.lines().map(str::trim) {
                let Some(rest) = line
                    .strip_prefix("-related_pin ")
                    .or_else(|| line.strip_prefix("-pin "))
                else {
                    continue;
                };
                let name = rest
                    .split_whitespace()
                    .next()
                    .expect("the field names a pin");
                assert!(
                    !cell.exposed.iter().any(|e| e == name),
                    "an exposed node reached {line:?}"
                );
            }
        }
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
        // One block per rest state: the two forcing inputs, and the two hold inputs at BOTH Q levels.
        assert_eq!(tcl.matches("define_leakage").count(), 6);
        assert!(tcl.contains("-when \"A*B*Q\""));
        assert!(tcl.contains("-when \"!A*!B*!Q\""));

        // The pair that shares an input assignment and differs only in what the cell holds: same
        // -pinlist, opposite Q column, and a prevector walking in from the forcing input that set Q.
        // Each walk ends at the rest state, as Liberate requires of a prevector.
        let block = |needle: &str| {
            tcl.split("define_leakage")
                .find(|b| b.contains(needle))
                .unwrap_or_else(|| panic!("no leakage block containing {needle:?} in:\n{tcl}"))
        };
        let high = block("-when \"A*!B*Q\"");
        let low = block("-when \"A*!B*!Q\"");
        assert!(high.contains("-pinlist {A B Q}") && low.contains("-pinlist {A B Q}"));
        assert!(high.contains("-vector {1 0 1}"), "held high: {high}");
        assert!(low.contains("-vector {1 0 0}"), "held low: {low}");
        assert!(high.contains("-prevector {11 10}"), "held high: {high}");
        assert!(low.contains("-prevector {00 10}"), "held low: {low}");

        // A forcing input drives the cell into its state on its own — no walk, so nothing to prime and
        // no prevector, unlike the hold states above.
        for needle in ["-when \"A*B*Q\"", "-when \"!A*!B*!Q\""] {
            let forced = block(needle);
            assert!(
                !forced.contains("-prevector"),
                "a forced rest state needs no priming: {forced}"
            );
        }

        // Liberate requires a prevector to END at the vector's value, so every rendered walk's last
        // step is the rest state's own input assignment — the two inputs of the {A B Q} vector.
        let field = |b: &str, tag: &str| -> String {
            b.lines()
                .find(|l| l.trim_start().starts_with(tag))
                .and_then(|l| l.split('{').nth(1))
                .and_then(|v| v.split('}').next())
                .unwrap_or_else(|| panic!("block renders a {tag}: {b}"))
                .to_string()
        };
        for b in tcl.split("define_leakage").skip(1) {
            let b = b.split("\n\n").next().unwrap_or(b);
            if !b.contains("-prevector ") {
                continue;
            }
            let last = field(b, "-prevector ")
                .split_whitespace()
                .last()
                .expect("a rendered prevector has a step")
                .to_string();
            let inputs: String = field(b, "-vector ")
                .split_whitespace()
                .take(2)
                .collect::<Vec<_>>()
                .concat();
            assert_eq!(last, inputs, "the walk ends at the vector's value: {b}");
        }
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
        // A combinational cell holds nothing, so its rest states are just the input square. Each is
        // reached with no walk, and a walk-free state has nothing to prime: no block carries a
        // prevector, and the vector alone states every pin's level.
        assert_eq!(tcl.matches("define_leakage").count(), 4);
        assert!(tcl.contains("-when \"A*B*Y\""));
        assert!(tcl.contains("-when \"!A*!B*!Y\""));
        assert!(tcl.contains("define_leakage \\\n\t-pinlist {A B Y} \\\n\t-vector {1 1 1} \\"));
        assert!(tcl.contains("define_leakage \\\n\t-pinlist {A B Y} \\\n\t-vector {0 0 0} \\"));
        for block in tcl.split("define_leakage").skip(1) {
            let block = block.split("\n\n").next().unwrap_or(block);
            assert!(
                !block.contains("-prevector"),
                "a combinational rest state needs no priming: {block}"
            );
        }
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
        // A leakage block fans the names into the same single trailer an arc block does.
        assert!(tcl.contains("-when \"A*B*Q\" \\\n\t{ C2A C2B }"));
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
    fn mcdff_two_clock_releases_are_edge_type() {
        // `when = "transition"`. Each clock release fires from a single context (the other clock open),
        // so the conditioned copy is suppressed and only the general `-type edge` block is emitted; the
        // classification, not the `-when`, is what this test is about.
        let (default, forced) = analyse_both(&when_variant(MCDFF, "\"transition\""));
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
        let mut saw_a_release = false;
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
                // A single-context release: the general edge block is the only one, with no -when.
                assert!(
                    !has_when(frag),
                    "single-context release emits no conditioned copy: {frag}"
                );
                saw_a_release = true;
            }
        }
        assert!(saw_b_release, "CLKB rising release Q arc is -type edge");
        assert!(saw_a_release, "CLKA falling release Q arc is -type edge");
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
        // these shapes) or forced on -- and the two runs emit the same arcs.
        for src in NON_COLLAPSIBLE {
            let (default, forced) = analyse_both(src);
            let tcl_default = cell_arcs_tcl(&default, ArcsTclOptions::default());
            let tcl_forced = cell_arcs_tcl(&forced, ArcsTclOptions::default());
            assert_eq!(tcl_default.matches("-type edge").count(), 0);
            assert_eq!(tcl_forced.matches("-type edge").count(), 0);
            assert_eq!(shaped_blocks(&tcl_default), shaped_blocks(&tcl_forced));
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
        // the SAME arcs -- zero `-type edge` blocks.
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
        assert_eq!(shaped_blocks(&tcl_direct), shaped_blocks(&tcl_via_flag));
    }
}
