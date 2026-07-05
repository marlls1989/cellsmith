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
/// generation), documents its metastable point, and — after collapse — uses only primary inputs as
/// related pins (a `Qb→Qa` arc would be a physical deadlock).
#[test]
fn mutex_generates_all_three_artifacts() {
    let cell = analyse_one(MUT);

    // Arcs: no crash, oscillation documented, related pins are inputs only.
    let tcl = cell_arcs_tcl(&cell, ArcsTclOptions::default());
    assert!(tcl.contains("# oscillation: A*B metastable"));
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
# selection mutual-exclusion
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

    // The statetable header naming sela1 must reference its folded support: enB and S. Match the
    // header whose *pin* field is sela1 (`, "sela1")`) — the sela2 statetable also lists sela1 as an
    // input, so a bare `contains("sela1")` would be ambiguous.
    let header = lib
        .lines()
        .find(|l| l.contains("statetable") && l.contains("\"sela1\")"))
        .expect("sela1 statetable header line present");
    assert!(header.contains("enB"), "sela1 header missing enB: {header}");
    assert!(header.contains('S'), "sela1 header missing S: {header}");

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
    // Liberty keeps both cross-coupled outputs as distinct output pins (not merged into one). NB: this
    // NOR-latch form is emitted with combinational `function` attributes, not `statetable` groups —
    // each function still references the *other* output, which is what "cross-coupling kept" means.
    let sr_lib = cell_liberty(&sr);
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

    // DFF: the internal master latch M survives as an internal pin.
    let dff = analyse_one(DFF);
    let int_names: Vec<_> = dff.internals.iter().map(|o| o.name.as_str()).collect();
    assert_eq!(int_names, ["M"], "DFF master latch M should survive");
    let dff_lib = cell_liberty(&dff);
    assert!(dff_lib.contains("pin (M)"), "DFF M pin missing");
    assert!(
        dff_lib.contains("direction : internal;"),
        "DFF M should be direction : internal"
    );
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
