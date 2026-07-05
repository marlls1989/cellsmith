//! End-to-end parity and validity checks over the three per-cell artifacts.
//!
//! Full byte-parity with the Haskell `genLiberateTemplate` goldens is not a goal: cellsmith emits
//! pins in declaration order (not alphabetical), drops the `vclk`/alias layer, and factors don't-care
//! cubes via BDD paths rather than Quine–McCluskey. What we *can* pin down is that the logic matches —
//! the sequential-UDP next-state table of a 2-input C-element is canonical — and that the emitted
//! Liberty is syntactically valid (parses back through `liberty-parse`).

use cellsmith::emit::arcs_tcl::{cell_arcs_tcl, ArcsTclOptions};
use cellsmith::emit::liberty::cell_liberty;
use cellsmith::emit::verilog::cell_verilog;
use cellsmith::logic::confluence::ConstraintKind;
use cellsmith::model::{parse_spec, AnalysedCell};

fn analyse_one(src: &str) -> AnalysedCell {
    parse_spec(src).unwrap().cells.remove(0).analyse().unwrap()
}

/// Assert an output pin emits at least one transition (`define_arc`) block — i.e. it is defined at
/// both ends of some arc and was not dropped from the machine. A transition arc names the output in
/// `-pin`; a hidden arc names an *input* there, so `-pin <output> ` (trailing space before the line
/// continuation) is an unambiguous marker for the output's own arcs.
fn assert_emits_arc(tcl: &str, output: &str) {
    assert!(
        tcl.contains(&format!("-pin {output} ")),
        "output {output} emits no arc block (dropped from the machine):\n{tcl}"
    );
}

/// Round-trip the emitted Liberty through `liberty-parse` and return the cell's pin-group names.
fn liberty_pins(cell: &AnalysedCell) -> Vec<String> {
    let frag = cell_liberty(cell);
    let wrapped = format!("library (test) {{\n{frag}}}\n");
    let lib = liberty_parse::parse_lib(&wrapped).expect("emitted Liberty must parse");
    lib.iter()
        .flat_map(|g| g.subgroups.iter())
        .find(|g| g.type_ == "cell" && g.name == cell.name.as_str())
        .expect("cell present after round-trip")
        .subgroups
        .iter()
        .filter(|g| g.type_ == "pin")
        .map(|g| g.name.clone())
        .collect()
}

/// Spec-conformance net over a cell's emitted Liberty (replaces the old single-rule R2 net). Round-trips
/// the fragment and asserts the single-table sequential shape the emitter now guarantees:
///  (a) at most ONE `statetable` group per cell;
///  (b) every output pin carries EXACTLY ONE of `function` / `state_function` / `internal_node`;
///  (c) a plain `function` names only primary INPUTS (never another output, never an internal node);
///  (d) a `state_function` names only inputs or pins that themselves carry an `internal_node` (never a
///      bare `_st` node from the isolated table namespace);
///  (e) every `internal_node : "<v>"` value appears in the cell's statetable internal-node header list;
///  (f) if any output uses a plain `function`, the cell has NO statetable.
/// Split out so it can be driven against hand-forged fragments (see `spec_conformance_net_bites`).
fn assert_liberty_spec_conformant(wrapped: &str, cell_name: &str) {
    use std::collections::HashSet;
    let lib = liberty_parse::parse_lib(wrapped).expect("emitted Liberty must parse");
    let cell_group = lib
        .iter()
        .flat_map(|g| g.subgroups.iter())
        .find(|g| g.type_ == "cell" && g.name == cell_name)
        .expect("cell present after round-trip");

    // (a) A sequential cell carries at most one joint statetable.
    let statetables: Vec<_> = cell_group
        .subgroups
        .iter()
        .filter(|g| g.type_ == "statetable")
        .collect();
    assert!(
        statetables.len() <= 1,
        "spec (a): cell `{cell_name}` has {} statetable groups, at most one is allowed",
        statetables.len()
    );
    let has_statetable = !statetables.is_empty();

    // The joint table's internal-node header list: `liberty-parse` joins the two quoted header args
    // (quotes kept) as `"<inputs>", "<nodes>"`, so the trailing comma field, quotes stripped and
    // space-split, is the node set (`"A B", "Qa_st Qb_st"` → {Qa_st, Qb_st}).
    let header_nodes: HashSet<String> = statetables
        .iter()
        .filter_map(|g| g.name.rsplit(", ").next())
        .flat_map(|field| {
            field
                .trim_matches('"')
                .split_whitespace()
                .map(str::to_owned)
        })
        .collect();

    // Primary-input pins, and pins that resolve a state node (carry an `internal_node`): the latter are
    // the only ports besides inputs a `state_function` may name (never a bare `_st` node).
    let input_names: HashSet<&str> = cell_group
        .subgroups
        .iter()
        .filter(|g| g.type_ == "pin")
        .filter(|g| {
            g.simple_attribute("direction")
                .map(|v| v.to_string())
                .as_deref()
                == Some("input")
        })
        .map(|g| g.name.as_str())
        .collect();
    let node_bearing: HashSet<&str> = cell_group
        .subgroups
        .iter()
        .filter(|g| g.type_ == "pin" && g.attributes.contains_key("internal_node"))
        .map(|g| g.name.as_str())
        .collect();

    // (e) Every `internal_node : "<v>"` (on any pin) must name a column of the joint statetable.
    for pin in cell_group.subgroups.iter().filter(|g| g.type_ == "pin") {
        if let Some(node) = pin
            .simple_attribute("internal_node")
            .map(|v| v.to_string().trim_matches('"').to_owned())
        {
            assert!(
                header_nodes.contains(&node),
                "spec (e): pin `{}` internal_node `{node}` is not a statetable column {header_nodes:?}",
                pin.name
            );
        }
    }

    for pin in cell_group
        .subgroups
        .iter()
        .filter(|g| g.type_ == "pin")
        .filter(|g| {
            g.simple_attribute("direction")
                .map(|v| v.to_string())
                .as_deref()
                == Some("output")
        })
    {
        let has_function = pin.attributes.contains_key("function");
        let has_state_function = pin.attributes.contains_key("state_function");
        let has_internal_node = pin.attributes.contains_key("internal_node");

        // (b) Exactly one binding attribute per output pin.
        let bindings = [has_function, has_state_function, has_internal_node]
            .iter()
            .filter(|b| **b)
            .count();
        assert_eq!(
            bindings, 1,
            "spec (b): output pin `{}` must carry exactly one of function / state_function / internal_node",
            pin.name
        );

        if has_function {
            // (f) A plain-function output forbids a statetable anywhere in the cell.
            assert!(
                !has_statetable,
                "spec (f): output pin `{}` uses a plain `function` yet the cell has a statetable",
                pin.name
            );
            let func = pin
                .simple_attribute("function")
                .map(|v| v.to_string().trim_matches('"').to_owned())
                .unwrap();
            // (c) A plain `function` names only primary inputs. Tokenise on identifier runs so `Q1` and
            // `Q1_st` stay distinct (a substring search would conflate them); numeric constants (0/1)
            // are not pin references and are skipped.
            for token in func
                .split(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
                .filter(|t| !t.is_empty() && !t.chars().all(|c| c.is_ascii_digit()))
            {
                assert!(
                    input_names.contains(token),
                    "spec (c): plain-function output `{}` (`{func}`) names `{token}`, not a primary input",
                    pin.name
                );
            }
        } else if has_state_function {
            let func = pin
                .simple_attribute("state_function")
                .map(|v| v.to_string().trim_matches('"').to_owned())
                .unwrap();
            // (d) A `state_function` names only inputs or pins that themselves carry an `internal_node`
            // — never a bare `_st` node from the isolated table namespace. Same identifier tokeniser.
            for token in func
                .split(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
                .filter(|t| !t.is_empty() && !t.chars().all(|c| c.is_ascii_digit()))
            {
                assert!(
                    input_names.contains(token) || node_bearing.contains(token),
                    "spec (d): state_function output `{}` (`{func}`) names `{token}`, not an input or an internal-node pin",
                    pin.name
                );
            }
        }
        // A pure `internal_node` projection output is validated by the (e) header check above.
    }
}

const C2: &str = r#"
[[cell]]
name = "C2"
inputs = ["A", "B"]
[cell.outputs]
Q = "A*B + Q*(A+B)"
"#;

/// The 2-input C-element UDP table is canonical: `00→0`, `11→1`, `01`/`10` hold. This matches the
/// hsNCL golden `CELEM2_Q` primitive body exactly (only the primitive/module names differ).
#[test]
fn c_element_udp_table_matches_golden_logic() {
    let v = cell_verilog(&analyse_one(C2));
    for row in [
        "0 0 : ? : 0;",
        "0 1 : ? : -;",
        "1 0 : ? : -;",
        "1 1 : ? : 1;",
    ] {
        assert!(v.contains(row), "missing UDP row {row:?} in:\n{v}");
    }
    assert!(v.contains("primitive C2_Q(Q, A, B);"));
    assert!(v.contains("reg    Q;"));
}

/// The emitted Liberty fragment must be syntactically valid: wrapped in a `library`, it round-trips
/// through `liberty-parse`, and the cell/pin/statetable groups survive.
#[test]
fn liberty_fragment_parses() {
    let frag = cell_liberty(&analyse_one(C2));
    let wrapped = format!("library (test) {{\n{frag}}}\n");
    let lib = liberty_parse::parse_lib(&wrapped).expect("emitted Liberty must parse");
    let cell = lib
        .iter()
        .flat_map(|g| g.subgroups.iter())
        .find(|g| g.type_ == "cell" && g.name == "C2")
        .expect("C2 cell present after round-trip");
    // The hysteretic output carries a statetable; the inputs are plain pins.
    assert!(cell.subgroups.iter().any(|g| g.type_ == "statetable"));
    assert!(cell
        .subgroups
        .iter()
        .any(|g| g.type_ == "pin" && g.name == "A"));
}

/// Multiple cells concatenate into a single, still-parseable Liberty fragment (no missing separators).
#[test]
fn multi_cell_liberty_concatenates_cleanly() {
    let spec = r#"
[[cell]]
name = "C2"
inputs = ["A", "B"]
[cell.outputs]
Q = "A*B + Q*(A+B)"

[[cell]]
name = "ND2"
inputs = ["A", "B"]
[cell.outputs]
Y = "!(A*B)"
"#;
    let cells: Vec<AnalysedCell> = parse_spec(spec)
        .unwrap()
        .cells
        .iter()
        .map(|c| c.analyse().unwrap())
        .collect();
    let frag: String = cells.iter().map(cell_liberty).collect();
    let wrapped = format!("library (test) {{\n{frag}}}\n");
    let lib = liberty_parse::parse_lib(&wrapped).expect("concatenated Liberty must parse");
    let names: Vec<String> = lib
        .iter()
        .flat_map(|g| g.subgroups.iter())
        .filter(|g| g.type_ == "cell")
        .map(|g| g.name.clone())
        .collect();
    assert_eq!(names, ["C2", "ND2"]);
}

const MUT: &str = r#"
[[cell]]
name = "MUT"
inputs = ["A", "B"]
[cell.outputs]
Qa = "!Qb * A"
Qb = "!Qa * B"
"#;

/// A cross-coupled mutex generates arcs across all three artifacts (it used to abort at arc
/// generation), documents its oscillation, and — after collapse — uses only primary inputs as
/// related pins (a `Qb→Qa` arc would be a physical deadlock).
#[test]
fn mutex_generates_all_three_artifacts() {
    let cell = analyse_one(MUT);

    // Arcs: no crash, oscillation documented, related pins are inputs only.
    let tcl = cell_arcs_tcl(&cell, ArcsTclOptions::default());
    assert!(tcl.contains("# oscillation: A*B risks metastability"));
    assert!(!tcl.contains("-related_pin Qa"));
    assert!(!tcl.contains("-related_pin Qb"));
    assert!(tcl.contains("-related_pin A"));
    assert!(tcl.contains("-related_pin B"));
    assert!(tcl.contains("-prevector_pinlist {A B}"));

    // Verilog: each grant's UDP still keeps the other grant as an input column (functional model), but
    // only the primary input its own function depends on — Qa = !Qb*A drops B, Qb = !Qa*B drops A. The
    // module still exposes both primary inputs as ports.
    let v = cell_verilog(&cell);
    assert!(v.contains("primitive MUT_Qa(Qa, Qb, A);"));
    assert!(v.contains("primitive MUT_Qb(Qb, Qa, B);"));
    assert!(v.contains("module MUT(Qa, Qb, A, B);"));

    // Liberty: annotated and still syntactically valid (round-trips through liberty-parse).
    let frag = cell_liberty(&cell);
    assert!(frag.contains("oscillation:"));
    // Both grants are state variables on the Qa↔Qb coupling cycle, so the cell emits ONE joint
    // statetable over both aliased grant nodes; each grant binds to its own `_st` alias node. A state
    // variable must never emit a combinational `function` naming another output.
    assert!(frag.contains(r#"statetable ("A B", "Qa_st Qb_st") {"#));
    // The race rows (minimal per-output cover): each grant independently drives high when its own
    // request is asserted and the OTHER grant currently reads low — one per-output row per Liberty's
    // per-output-priority multi-output statetable, not a single joint "both go high" row.
    assert!(
        frag.contains("H - : - L : H -"),
        "Qa's race-to-high row missing:\n{frag}"
    );
    assert!(
        frag.contains("- H : L - : - H"),
        "Qb's race-to-high row missing:\n{frag}"
    );
    assert!(frag.contains(r#"internal_node : "Qa_st";"#));
    assert!(frag.contains(r#"internal_node : "Qb_st";"#));
    assert!(frag.contains("inverted_output : false;"));
    // No plain (nor state) `function` anywhere — both outputs bind their `_st` nodes.
    assert!(
        !frag.contains("function : "),
        "a state variable must never emit a combinational function:\n{frag}"
    );
    let wrapped = format!("library (test) {{\n{frag}}}\n");
    let lib = liberty_parse::parse_lib(&wrapped).expect("emitted Liberty must parse");
    assert!(lib
        .iter()
        .flat_map(|g| g.subgroups.iter())
        .any(|g| g.type_ == "cell" && g.name == "MUT"));
}

// ---------------------------------------------------------------------------------------------
// Behaviour-preservation goldens for the state-space minimisation (Wave 3).
//
// These pin the *externally observable* behaviour of the fold via the public artifacts, using
// substring / semantic assertions (no byte snapshots): a cell whose internals are pure relays or
// complementary aliases must analyse to the same machine as its hand-minimised twin, while genuine
// memory must never be collapsed.
// ---------------------------------------------------------------------------------------------

/// Gate-level C-element: complementary internals (`IQ = !QN`, `QN = !(A*B + IQ*(A+B))`) collapse to
/// the single coordinate `Q = A*B + Q*(A+B)` — i.e. exactly the `C2` machine.
const C2GATE: &str = r#"
[[cell]]
name = "C2GATE"
inputs = ["A", "B"]
[cell.internal]
IQ = "!QN"
QN = "!(A*B + IQ*(A+B))"
[cell.outputs]
Q = "IQ"
"#;

/// Interlocked clock-mux with two relay internals (`sela`/`selb`) feeding CLKA/CLKB synchronisers.
/// Verbatim copy of the `examples/cells.toml` fixture.
const ICM: &str = r#"
[[cell]]
name = "ICM"
inputs = ["CLKA", "CLKB", "RA", "RB", "S"]
clock = ["CLKA", "CLKB"]

[cell.internal]
# selection interlock
sela = "!enB*!S"
selb = "!enA*S"
# CLKA synchroniser
sela1 = "!RA*(!CLKA*sela+CLKA*sela1)"
sela2 = "!RA*(CLKA*sela1+!CLKA*sela2)"
enA   = "!RA*(!CLKA*sela2+CLKA*enA)"
# CLKB synchroniser
selb1 = "!RB*(!CLKB*selb+CLKB*selb1)"
selb2 = "!RB*(CLKB*selb1+!CLKB*selb2)"
enB   = "!RB*(!CLKB*selb2+CLKB*enB)"
[cell.outputs]
GCLK = "enA*CLKA+enB*CLKB"
"#;

/// `ICM` with the `sela`/`selb` relays hand-folded into their sole consumers (`sela1`/`selb1`). Same
/// cell name so the analysed artifacts compare directly against the auto-minimised `ICM`.
const ICM_FOLDED: &str = r#"
[[cell]]
name = "ICM"
inputs = ["CLKA", "CLKB", "RA", "RB", "S"]
clock = ["CLKA", "CLKB"]

[cell.internal]
# CLKA synchroniser
sela1 = "!RA*(!CLKA*(!enB*!S)+CLKA*sela1)"
sela2 = "!RA*(CLKA*sela1+!CLKA*sela2)"
enA   = "!RA*(!CLKA*sela2+CLKA*enA)"
# CLKB synchroniser
selb1 = "!RB*(!CLKB*(!enA*S)+CLKB*selb1)"
selb2 = "!RB*(CLKB*selb1+!CLKB*selb2)"
enB   = "!RB*(!CLKB*selb2+CLKB*enB)"
[cell.outputs]
GCLK = "enA*CLKA+enB*CLKB"
"#;

/// A ring-oscillator relay whose 2-cycle guard is strengthened: the relay `X` and the output `Q`
/// oscillate at `A*!B` rather than folding away.
const ROSC: &str = r#"
[[cell]]
name = "ROSC"
inputs = ["A", "B"]
[cell.internal]
X = "!Q*A"
[cell.outputs]
Q = "Q*B + X"
"#;

/// Cross-coupled NOR latch — genuine memory. Verbatim copy of the `examples/cells.toml` fixture.
const SR: &str = r#"
[[cell]]
name = "SR"
inputs = ["S", "R"]
[cell.outputs]
Q  = "!(R+Qn)"
Qn = "!(S+Q)"
"#;

/// D flip-flop with an internal master latch `M` — genuine memory that must survive the fold.
/// Verbatim copy of the `examples/cells.toml` fixture.
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

/// C2GATE's complementary internals minimise away, leaving a cell externally isomorphic to `C2`: the
/// three artifacts are byte-identical after normalising the cell name, and no trace of the purged
/// internals (`IQ`/`QN`) survives.
#[test]
fn c2gate_is_externally_isomorphic_to_c2() {
    let c2gate = analyse_one(C2GATE);
    let c2 = analyse_one(C2);

    // The internals collapsed to the single output coordinate; no oscillation in a plain C-element.
    assert!(c2gate.internals.is_empty(), "C2GATE internals not folded");
    assert!(c2gate.oscillation.is_empty());

    // All three artifacts are byte-identical to C2's once the physical name is normalised.
    let opts = ArcsTclOptions::default();
    assert_eq!(
        cell_arcs_tcl(&c2gate, opts).replace("C2GATE", "C2"),
        cell_arcs_tcl(&c2, opts),
        "arcs tcl diverges from C2"
    );
    assert_eq!(
        cell_verilog(&c2gate).replace("C2GATE", "C2"),
        cell_verilog(&c2),
        "verilog diverges from C2"
    );
    let c2gate_lib = cell_liberty(&c2gate);
    assert_eq!(
        c2gate_lib.replace("C2GATE", "C2"),
        cell_liberty(&c2),
        "liberty diverges from C2"
    );

    // No purged-internal names leak into any artifact.
    for art in [
        cell_arcs_tcl(&c2gate, opts),
        cell_verilog(&c2gate),
        c2gate_lib.clone(),
    ] {
        assert!(!art.contains("IQ"), "IQ leaked into artifact:\n{art}");
        assert!(!art.contains("QN"), "QN leaked into artifact:\n{art}");
    }

    // The Liberty fragment still round-trips through liberty-parse.
    let wrapped = format!("library (test) {{\n{c2gate_lib}}}\n");
    let lib = liberty_parse::parse_lib(&wrapped).expect("C2GATE Liberty must parse");
    assert!(lib
        .iter()
        .flat_map(|g| g.subgroups.iter())
        .any(|g| g.type_ == "cell" && g.name == "C2GATE"));
}

/// The `sela`/`selb` relays fold away automatically, and the auto-minimised `ICM` machine is
/// preserved field-for-field against the hand-folded `ICM_FOLDED`.
#[test]
fn icm_relays_fold_and_machine_is_preserved() {
    let cell = analyse_one(ICM);
    let folded = analyse_one(ICM_FOLDED);

    // Compose is exact: every machine-analysis field must match the hand-folded twin.
    assert_eq!(
        format!("{:?}", cell.arcs),
        format!("{:?}", folded.arcs),
        "arcs differ from hand-folded ICM"
    );
    assert_eq!(
        format!("{:?}", cell.hidden_arcs),
        format!("{:?}", folded.hidden_arcs),
        "hidden_arcs differ from hand-folded ICM"
    );
    assert_eq!(
        format!("{:?}", cell.order_dependence),
        format!("{:?}", folded.order_dependence),
        "order_dependence differs from hand-folded ICM"
    );
    assert_eq!(
        format!("{:?}", cell.oscillation),
        format!("{:?}", folded.oscillation),
        "oscillation differs from hand-folded ICM"
    );
    assert_eq!(
        format!("{:?}", cell.constraints),
        format!("{:?}", folded.constraints),
        "constraints differ from hand-folded ICM"
    );
    assert_eq!(
        format!("{:?}", cell.leakage),
        format!("{:?}", folded.leakage),
        "leakage differs from hand-folded ICM"
    );

    // No oscillation: a synchroniser chain settles, it does not oscillate.
    assert!(cell.oscillation.is_empty());

    // Only the genuine-memory internals survive (relays sela/selb purged); machine width drops 13→11.
    let int_names: Vec<_> = cell.internals.iter().map(|o| o.name.as_str()).collect();
    assert_eq!(
        int_names,
        ["enA", "enB", "sela1", "sela2", "selb1", "selb2"]
    );

    let lib = cell_liberty(&cell);
    assert!(lib.contains("pin (sela1)"), "sela1 pin missing");
    // Trailing space/brace matters: bare `pin (sela)` is a prefix of `pin (sela1)`.
    assert!(
        !lib.contains("pin (sela) "),
        "purged relay sela still a pin"
    );
    assert!(
        !lib.contains("pin (selb) "),
        "purged relay selb still a pin"
    );

    // A SINGLE joint statetable lists all six surviving state nodes of the cell; each is anchored by a
    // `direction : internal` pin naming its own node.
    assert_eq!(
        lib.matches("statetable").count(),
        1,
        "ICM emits one joint statetable"
    );
    assert!(
        lib.contains(r#"statetable ("CLKA CLKB RA RB S", "enA enB sela1 sela2 selb1 selb2") {"#),
        "single joint statetable over all six state nodes missing:\n{lib}"
    );
    assert!(
        lib.contains(r#"internal_node : "sela1";"#),
        "sela1 internal-node pin missing"
    );
    // The gated-clock output is combinational over pins, so it renders as `state_function` (naming the
    // internal-node pins enA/enB and the clock inputs) — never an internal_node projection nor a plain
    // `function`.
    assert!(
        lib.contains(r#"state_function : "enB*CLKB + enA*CLKA";"#),
        "GCLK should render as a state_function over pin ports:\n{lib}"
    );

    // The arcs reference the clock output but never the purged relay token.
    let tcl = cell_arcs_tcl(&cell, ArcsTclOptions::default());
    assert!(tcl.contains("-pin GCLK"), "GCLK arc pin missing");
    assert!(!tcl.contains("sela"), "purged relay token sela in arcs tcl");

    // DELIBERATE LOCK — R2 deviation, coordinator SIGNED OFF. Declassifying the `sela` relay exposes
    // `S` at the CLKA latch boundary, so `confluence::detect`'s combinational-neighbourhood
    // direct-support filter now detects an order-dependent hazard on CLKA/S — from which
    // `confluence::constrain` generates a setup/hold constraint relating CLKA and S that the un-folded
    // model did not have. This is a *gained* constraint: gains are accepted ('may be gained, never
    // lost'), losses are not. Do NOT weaken this away — it is the R2-deviation sentinel.
    assert!(
        cell.constraints
            .iter()
            .any(|c| c.kind == ConstraintKind::SetupHold && c.related == "CLKA" && c.pin == "S"),
        "expected the gained SetupHold(CLKA, S) constraint; constraints = {:?}",
        cell.constraints
    );
}

/// `ROSC`'s relay `X` folds into the already-self-holding `Q`. Folding a relay into a genuine
/// register preserves the dynamics: the oscillation survives in `Q`'s own self-loop (`δ_Q = !Q` at
/// `A*!B`), so `{Q}` still oscillates at the same condition — only the folded-away relay leaves the
/// reported group.
#[test]
fn rosc_relay_folds_and_oscillation_survives() {
    let cell = analyse_one(ROSC);

    assert!(
        cell.internals.is_empty(),
        "ROSC relay X folds into the self-holding Q"
    );

    assert_eq!(cell.oscillation.len(), 1, "expected one oscillating group");
    let arb = &cell.oscillation[0];
    let group: Vec<_> = arb.group.iter().map(|s| s.as_str()).collect();
    assert_eq!(group, ["Q"]);
    assert_eq!(arb.condition_str(), "A*!B");
}

/// Genuine memory is never collapsed: the SR latch keeps both cross-coupled primitives/statetables,
/// and the DFF keeps its internal master latch `M`.
#[test]
fn genuine_memory_is_never_collapsed() {
    // SR: cross-coupling kept — each grant's support has two variables, so neither is an alias. The
    // Verilog primitives are the load-bearing evidence: `SR_Q(Q, R, Qn)` / `SR_Qn(Qn, S, Q)` each
    // keep a 2-input sequential UDP, so neither output was aliased to the other.
    let sr = analyse_one(SR);
    let v = cell_verilog(&sr);
    assert!(v.contains("primitive SR_Q("), "SR_Q primitive missing");
    assert!(v.contains("primitive SR_Qn("), "SR_Qn primitive missing");
    // Liberty keeps both cross-coupled outputs as distinct output pins (not merged into one). Both are
    // state variables on the Q↔Qn coupling cycle, so the cell emits ONE joint statetable over both
    // aliased grant nodes; each output binds to its own `_st` alias node. A state variable must NEVER
    // emit a combinational `function` naming the other output.
    let sr_lib = cell_liberty(&sr);
    assert_eq!(
        sr_lib.matches("statetable").count(),
        1,
        "SR emits one joint statetable"
    );
    assert!(sr_lib.contains(r#"statetable ("S R", "Q_st Qn_st") {"#));
    assert!(sr_lib.contains(r#"internal_node : "Q_st";"#));
    assert!(sr_lib.contains(r#"internal_node : "Qn_st";"#));
    assert!(
        !sr_lib.contains("function : "),
        "a state variable must never emit a combinational function:\n{sr_lib}"
    );
    let wrapped = format!("library (test) {{\n{sr_lib}}}\n");
    let lib = liberty_parse::parse_lib(&wrapped).expect("SR Liberty must parse");
    let sr_cell = lib
        .iter()
        .flat_map(|g| g.subgroups.iter())
        .find(|g| g.type_ == "cell" && g.name == "SR")
        .expect("SR cell present");
    let out_pins: Vec<&str> = sr_cell
        .subgroups
        .iter()
        .filter(|g| g.type_ == "pin")
        .map(|g| g.name.as_str())
        .collect();
    assert!(
        out_pins.contains(&"Q"),
        "SR output Q collapsed: {out_pins:?}"
    );
    assert!(
        out_pins.contains(&"Qn"),
        "SR output Qn collapsed: {out_pins:?}"
    );

    // DFF: the internal master latch M survives as a genuine internal node. One joint statetable over
    // both state nodes (aliased Q_st and the un-aliased internal M); Q projects its alias, M is a
    // `direction : internal` pin anchoring its own same-named node.
    let dff = analyse_one(DFF);
    let int_names: Vec<_> = dff.internals.iter().map(|o| o.name.as_str()).collect();
    assert_eq!(int_names, ["M"], "DFF master latch M should survive");
    let dff_lib = cell_liberty(&dff);
    assert_eq!(
        dff_lib.matches("statetable").count(),
        1,
        "DFF emits one joint statetable"
    );
    assert!(dff_lib.contains(r#"statetable ("CLK D", "Q_st M") {"#));
    assert!(dff_lib.contains("pin (M)"), "DFF M pin missing");
    assert!(
        dff_lib.contains("direction : internal;"),
        "DFF M should be direction : internal"
    );
    assert!(
        dff_lib.contains(r#"internal_node : "M";"#),
        "M should anchor its own node"
    );
    assert!(
        dff_lib.contains(r#"internal_node : "Q_st";"#),
        "Q should project its alias node"
    );
}

// ---------------------------------------------------------------------------------------------
// Whole-pipeline output-alias fixtures (Wave 5).
//
// The pass-local minimise tests inspect the `bdds` map directly and never build the machine, so they
// missed a blocker where an output ended up aliased to a *combinational* output — unevaluable by the
// machine (its nodes carry only inputs + state variables), which debug-panics at
// `src/logic/analysis.rs:109` (invariant I3) and in release silently drops the pin's arcs. These
// fixtures drive the SAME full pipeline as the other goldens (`Cell::analyse`, via `analyse_one`,
// which builds the machine in `analyse_machine` at `src/logic/analysis.rs:175`) and assert positively
// on emitted arcs — catching both the debug panic and the release-mode silent drop — for each shape.
// ---------------------------------------------------------------------------------------------

/// (1) Two output pins carry the identical *combinational* function `A*B`; they must stay independent
/// combinational outputs, neither aliased to the other.
const DUP_COMB: &str = r#"
[[cell]]
name = "DUPY"
inputs = ["A", "B"]
[cell.outputs]
Y1 = "A*B"
Y2 = "A*B"
"#;

/// (2) Output buffer of a combinational output: `Y2 = Y1` where `Y1 = A*B`. The alias must resolve so
/// `Y2` carries the composed function `A*B` — never left aliased to the combinational `Y1`.
const BUF_COMB: &str = r#"
[[cell]]
name = "BUFY"
inputs = ["A", "B"]
[cell.outputs]
Y1 = "A*B"
Y2 = "Y1"
"#;

/// (3) Complementary-output gate over an internal `X = A*B`: `Y = X`, `YN = !X`. The internal is
/// purged and its coordinate carried on the two output pins (`Y = A*B`, `YN = !(A*B)`).
const COMP_OUT: &str = r#"
[[cell]]
name = "COMPY"
inputs = ["A", "B"]
[cell.internal]
X = "A*B"
[cell.outputs]
Y = "X"
YN = "!X"
"#;

/// (4) Recurrent duplicate outputs (the positive dedup case): `Q1`, `Q2` both `!R*(S+Q1)`. Dedup
/// merges onto the state variable `Q1`; `Q2` is kept as a plain alias pin of `Q1` (a genuine state
/// variable, so the machine can still evaluate it — no escape).
const DUP_RECUR: &str = r#"
[[cell]]
name = "SRDUP"
inputs = ["S", "R"]
[cell.outputs]
Q1 = "!R*(S+Q1)"
Q2 = "!R*(S+Q1)"
"#;

/// (5) Complement-output C-element: `Q` is the held (cyclic) coordinate, `QN = !Q` its complement. At
/// emission time the coordinate becomes the `Q_st` alias node of the single joint statetable; both `Q`
/// and `QN` bind that alias through `internal_node` (`Q` non-inverted, `QN` inverted), so neither output
/// names the other.
const CLATCH: &str = r#"
[[cell]]
name = "CLATCH"
inputs = ["A", "B"]
[cell.outputs]
Q = "A*B + Q*(A+B)"
QN = "!Q"
"#;

/// (1) Duplicate combinational outputs survive as two independent combinational outputs — each emits
/// its arcs, neither is aliased to the other, and no internal appears.
#[test]
fn duplicate_combinational_outputs_stay_independent_and_both_emit_arcs() {
    let cell = analyse_one(DUP_COMB);
    assert!(cell.internals.is_empty(), "no internal expected");

    let tcl = cell_arcs_tcl(&cell, ArcsTclOptions::default());
    assert_emits_arc(&tcl, "Y1");
    assert_emits_arc(&tcl, "Y2");

    let pins = liberty_pins(&cell);
    assert!(pins.contains(&"Y1".to_string()), "Y1 pin missing: {pins:?}");
    assert!(pins.contains(&"Y2".to_string()), "Y2 pin missing: {pins:?}");

    // Both pins carry the full A*B function — neither was demoted to an alias of the other.
    let lib = cell_liberty(&cell);
    assert_eq!(
        lib.matches("function : \"A*B\";").count(),
        2,
        "both outputs must carry the independent A*B function:\n{lib}"
    );
}

/// (2) A combinational output buffered by another output resolves: `Y2` carries the composed `A*B`
/// and never stays aliased to `Y1`. Both pins emit arcs.
#[test]
fn output_buffer_of_combinational_output_resolves_and_both_emit_arcs() {
    let cell = analyse_one(BUF_COMB);
    assert!(cell.internals.is_empty(), "no internal expected");

    let tcl = cell_arcs_tcl(&cell, ArcsTclOptions::default());
    assert_emits_arc(&tcl, "Y1");
    assert_emits_arc(&tcl, "Y2");
    // Y2's arcs relate to the primary inputs, not to the buffered output Y1.
    assert!(
        !tcl.contains("-related_pin Y1"),
        "Y2 arc relates to the buffered output Y1:\n{tcl}"
    );

    let pins = liberty_pins(&cell);
    assert!(pins.contains(&"Y1".to_string()), "Y1 pin missing: {pins:?}");
    assert!(pins.contains(&"Y2".to_string()), "Y2 pin missing: {pins:?}");

    // Y2 carries the resolved A*B, not a `function : "Y1"` alias; both outputs share the same function.
    let lib = cell_liberty(&cell);
    assert!(
        !lib.contains("function : \"Y1\";"),
        "Y2 still aliased to Y1:\n{lib}"
    );
    assert_eq!(
        lib.matches("function : \"A*B\";").count(),
        2,
        "both outputs must carry the resolved A*B function:\n{lib}"
    );
}

/// (3) A complementary-output gate over an internal: the internal is purged, `Y` carries `A*B` and
/// `YN` its complement, and neither output is left aliased. Both pins emit arcs; the internal never
/// leaks as a pin.
#[test]
fn complementary_outputs_over_internal_purge_internal_and_both_emit_arcs() {
    let cell = analyse_one(COMP_OUT);
    assert!(
        cell.internals.is_empty(),
        "internal X should be purged: {:?}",
        cell.internals
            .iter()
            .map(|o| o.name.as_str())
            .collect::<Vec<_>>()
    );

    let tcl = cell_arcs_tcl(&cell, ArcsTclOptions::default());
    assert_emits_arc(&tcl, "Y");
    assert_emits_arc(&tcl, "YN");
    // The purged internal never leaks as an arc pin.
    assert!(
        !tcl.contains("-pin X "),
        "internal X leaked into arcs:\n{tcl}"
    );

    let pins = liberty_pins(&cell);
    assert!(pins.contains(&"Y".to_string()), "Y pin missing: {pins:?}");
    assert!(pins.contains(&"YN".to_string()), "YN pin missing: {pins:?}");
    assert!(
        !pins.contains(&"X".to_string()),
        "purged internal X still a pin: {pins:?}"
    );

    // Y carries A*B; YN carries the complement (the SOP form of !(A*B): a sum of the inverted inputs).
    let lib = cell_liberty(&cell);
    assert!(
        lib.contains("function : \"A*B\";"),
        "Y should carry A*B:\n{lib}"
    );
    let yn_func = lib
        .lines()
        .skip_while(|l| !l.contains("pin (YN)"))
        .find(|l| l.contains("function"))
        .expect("YN function present");
    assert!(
        yn_func.contains("!A") && yn_func.contains("!B") && !yn_func.contains('*'),
        "YN should carry the complement of A*B (a sum of !A and !B), got: {yn_func}"
    );
}

/// (4) Recurrent duplicate outputs dedup onto one coordinate. At emission time that coordinate becomes
/// the `Q1_st` alias node of the single joint statetable, and BOTH `Q1` and `Q2` bind to it through
/// `internal_node` — so neither output names another output. The alias is NOT a genuine internal (the
/// machine has no internal), and it is never a pin of its own. Both outputs still emit arcs.
#[test]
fn recurrent_duplicate_outputs_dedup_and_both_emit_arcs() {
    let cell = analyse_one(DUP_RECUR);
    // The recurrent coordinate is an emission-time `_st` alias, not a genuine internal.
    assert!(
        cell.internals.is_empty(),
        "no genuine internal expected: {:?}",
        cell.internals
            .iter()
            .map(|o| o.name.as_str())
            .collect::<Vec<_>>()
    );

    let tcl = cell_arcs_tcl(&cell, ArcsTclOptions::default());
    assert_emits_arc(&tcl, "Q1");
    assert_emits_arc(&tcl, "Q2");
    // The alias node is never an arc pin — arcs range over inputs and outputs only.
    assert!(
        !tcl.contains("Q1_st"),
        "the `_st` alias leaked into the arcs:\n{tcl}"
    );

    let pins = liberty_pins(&cell);
    assert!(pins.contains(&"Q1".to_string()), "Q1 pin missing: {pins:?}");
    assert!(pins.contains(&"Q2".to_string()), "Q2 pin missing: {pins:?}");
    // The `_st` alias is a table node anchored by the outputs' `internal_node` — never a pin group.
    assert!(
        !pins.contains(&"Q1_st".to_string()),
        "the `_st` alias must not be a pin: {pins:?}"
    );

    // A single joint statetable on the aliased node; both outputs bind the SAME alias.
    let lib = cell_liberty(&cell);
    assert_eq!(lib.matches("statetable").count(), 1, "one joint statetable");
    assert!(
        lib.contains(r#"statetable ("S R", "Q1_st") {"#),
        "Q1_st should carry the recurrent statetable:\n{lib}"
    );
    assert_eq!(
        lib.matches(r#"internal_node : "Q1_st";"#).count(),
        2,
        "both Q1 and Q2 must bind the shared Q1_st alias:\n{lib}"
    );
    // No output names another output, and there is no combinational function anywhere.
    assert!(
        !lib.contains("function : "),
        "the deduped outputs project the alias — no `function` should appear:\n{lib}"
    );
}

/// (5) Complement-output C-element: `Q` is the held (cyclic) coordinate, `QN = !Q`. There is no minted
/// internal — the coordinate is the emission-time `Q_st` alias of the single joint statetable, and BOTH
/// outputs bind it through `internal_node`: `Q` non-inverted, `QN` inverted. Neither output names the
/// other. Both outputs emit arcs and the Liberty round-trips. In the Verilog the coordinate lives on the
/// output `Q` (a sequential UDP over the inputs) and `QN` inverts `Q` directly — the intended
/// post-hoist-removal no-mint shape.
#[test]
fn complement_c_element_pair_hoists_coordinate_to_internal() {
    let cell = analyse_one(CLATCH);
    // No hoist: the shared cyclic coordinate is an emission-time `_st` alias, not a genuine internal.
    assert!(
        cell.internals.is_empty(),
        "no genuine internal expected: {:?}",
        cell.internals
            .iter()
            .map(|o| o.name.as_str())
            .collect::<Vec<_>>()
    );

    let tcl = cell_arcs_tcl(&cell, ArcsTclOptions::default());
    assert_emits_arc(&tcl, "Q");
    assert_emits_arc(&tcl, "QN");
    assert!(
        !tcl.contains("Q_st"),
        "the `_st` alias leaked into the arcs:\n{tcl}"
    );

    // The `_st` alias is a table node, never a pin of its own.
    let pins = liberty_pins(&cell);
    assert!(pins.contains(&"Q".to_string()), "Q pin missing: {pins:?}");
    assert!(pins.contains(&"QN".to_string()), "QN pin missing: {pins:?}");
    assert!(
        !pins.contains(&"Q_st".to_string()),
        "the `_st` alias must not be a pin: {pins:?}"
    );

    // A single joint statetable on the aliased node; Q binds it non-inverted, QN binds the SAME alias
    // inverted — neither output names the other, and no `function`/`state_function` appears.
    let lib = cell_liberty(&cell);
    assert_eq!(lib.matches("statetable").count(), 1, "one joint statetable");
    assert!(
        lib.contains(r#"statetable ("A B", "Q_st") {"#),
        "Q_st should carry the statetable:\n{lib}"
    );
    assert_eq!(
        lib.matches(r#"internal_node : "Q_st";"#).count(),
        2,
        "both Q and QN must bind the shared Q_st alias:\n{lib}"
    );
    assert!(
        lib.contains("inverted_output : false;") && lib.contains("inverted_output : true;"),
        "Q reads the alias non-inverted, QN inverted:\n{lib}"
    );
    assert!(
        !lib.contains("function : "),
        "the complement pair binds the alias — no `function` should appear:\n{lib}"
    );

    // Verilog: post-hoist-removal no-mint shape. The coordinate is the output `Q` (a sequential UDP over
    // the inputs); `QN` inverts the output `Q` directly. No minted `_st` internal exists.
    let v = cell_verilog(&cell);
    assert!(
        v.contains("primitive CLATCH_Q(Q, A, B);"),
        "Q should be a sequential UDP over the inputs:\n{v}"
    );
    assert!(
        v.contains("primitive CLATCH_QN(QN, Q);"),
        "QN should invert the output Q directly:\n{v}"
    );
    assert!(
        v.contains("module CLATCH(Q, QN, A, B);"),
        "module port list:\n{v}"
    );
    assert!(
        !v.contains("Q_st"),
        "no minted `_st` internal should appear in the Verilog:\n{v}"
    );
}

/// Spec-conformance net over every fixture: the single-table sequential shape holds for each cell (at
/// most one statetable; every output binds exactly one of function/state_function/internal_node; a plain
/// `function` names only inputs; a `state_function` names only inputs or internal-node pins; every
/// `internal_node` value is a statetable column; a plain-function cell has no statetable). The statetable
/// cells (MUT/SR/DFF/ICM) bind their outputs to `_st` alias or genuine-internal nodes; the deduped/
/// complement cells (DUP_RECUR/CLATCH) bind the shared alias; the combinational cells reference only
/// inputs. ND2 has no standalone const — it lives inline in the multi-cell fixture — so it is
/// reproduced here verbatim.
#[test]
fn liberty_is_spec_conformant_across_fixtures() {
    const ND2: &str = r#"
[[cell]]
name = "ND2"
inputs = ["A", "B"]
[cell.outputs]
Y = "!(A*B)"
"#;
    for src in [
        C2, ND2, MUT, SR, DFF, ICM, ROSC, C2GATE, DUP_COMB, BUF_COMB, COMP_OUT, DUP_RECUR, CLATCH,
    ] {
        let cell = analyse_one(src);
        let frag = cell_liberty(&cell);
        let wrapped = format!("library (test) {{\n{frag}}}\n");
        assert_liberty_spec_conformant(&wrapped, cell.name.as_str());
    }
}

/// `-when` is emitted by default; disabling it (the `--no-when` path) drops it from the arc text.
#[test]
fn when_default_on_and_suppressible() {
    let cell = analyse_one(C2);
    let on = cell_arcs_tcl(&cell, ArcsTclOptions::default());
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
    assert!(on.contains("-when"));
    assert!(!off.contains("-when"));
}

/// The spec net actually bites: three hand-forged shapes the emitter never produces must each be
/// REJECTED, proving the net has teeth on rules (c), (d) and (a). The offending Liberty is fed straight
/// to the check core.
#[test]
fn spec_conformance_net_bites() {
    // (c) a plain `function` output naming another output (Q1) rather than an input.
    let bad_c = r#"library (test) {
cell (BAD) {
pin (A) { direction : input; }
pin (Q1) { direction : output; function : "A"; }
pin (Q2) { direction : output; function : "Q1"; }
}
}
"#;
    // (d) a `state_function` naming a pin (P) that carries no `internal_node`.
    let bad_d = r#"library (test) {
cell (BAD) {
pin (A) { direction : input; }
statetable ("A", "L") { table : "- : - : L"; }
pin (L) { direction : internal; internal_node : "L"; }
pin (P) { direction : output; state_function : "A"; }
pin (Y) { direction : output; state_function : "P"; }
}
}
"#;
    // (a) two `statetable` groups in one cell.
    let bad_a = r#"library (test) {
cell (BAD) {
pin (A) { direction : input; }
statetable ("A", "L") { table : "- : - : L"; }
statetable ("A", "M") { table : "- : - : L"; }
pin (L) { direction : internal; internal_node : "L"; }
pin (M) { direction : internal; internal_node : "M"; }
}
}
"#;
    for (label, forged) in [
        ("(c) plain function names another output", bad_c),
        ("(d) state_function names a non-internal-node pin", bad_d),
        ("(a) two statetables in one cell", bad_a),
    ] {
        let rejected = std::panic::catch_unwind(|| {
            assert_liberty_spec_conformant(forged, "BAD");
        })
        .is_err();
        assert!(
            rejected,
            "spec net failed to reject forged fragment: {label}"
        );
    }
}
