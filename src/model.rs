//! Input model: a minimal multi-cell TOML spec, plus analysis that classifies each function's
//! variables into **primary inputs** vs **feedback/state** (an output name referenced inside a
//! function is the delayed/feedback value of that output).

use std::collections::{BTreeMap, BTreeSet};

use espresso_logic::bdd::{Bdd, BddBuilder, Brand, ManagerCell};
use espresso_logic::{sync_bdd_builder, BoolExpr, Symbol};
use indexmap::IndexMap;
use rayon::prelude::*;
use serde::Deserialize;
use thiserror::Error;

use espresso_logic::expression::ParseBoolExprError;

use crate::expr;
use crate::logic::arcs::{Arc, HiddenArc};
use crate::logic::confluence::Constraint;
use crate::logic::hazard::{OrderDependence, Oscillation};
use crate::logic::leakage::LeakageState;

/// The whole input file: a list of `[[cell]]` tables.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Spec {
    #[serde(rename = "cell", default)]
    pub cells: Vec<Cell>,
}

/// One cell exactly as written in the TOML.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Cell {
    /// Physical cell name(s) used in the emitted arcs. A scalar or a list; the first entry is the
    /// representative name for single-name contexts.
    #[serde(deserialize_with = "de_name_list")]
    pub name: Vec<Symbol>,
    /// Primary input pins. Order matters: it defines the pinlist/vector order.
    #[serde(deserialize_with = "de_symbol_vec")]
    pub inputs: Vec<Symbol>,
    /// Output pin name -> Boolean function. Order preserved (stable output order).
    #[serde(deserialize_with = "de_symbol_map")]
    pub outputs: IndexMap<Symbol, String>,
    /// Optional: internal state variable name -> Boolean function. Order preserved. An internal
    /// signal is referenceable by other functions and is a driven state variable (modelled in the
    /// Verilog and the Liberty state table), but emits **no** external output pin and is never an arc
    /// source or target.
    #[serde(default, deserialize_with = "de_symbol_map")]
    pub internal: IndexMap<Symbol, String>,
    /// Optional: input pins that force the output regardless of held state (async set/reset),
    /// so their arcs are emitted as `-type async` rather than combinational.
    #[serde(rename = "async", default, deserialize_with = "de_symbol_vec")]
    pub async_pins: Vec<Symbol>,
    /// Optional: input pins that are clocks. A hazard on a pin pair holding a declared clock yields a
    /// directed setup/hold constraint (clock ← data); any other pair yields a symmetric non_seq. See
    /// [`crate::logic::confluence`].
    #[serde(default, deserialize_with = "de_symbol_vec")]
    pub clock: Vec<Symbol>,
    /// Optional: opt in to emitting derived constraint arcs (setup/hold, non_seq) for this cell. Off by
    /// default; also enabled globally by the `--constraints` CLI flag.
    #[serde(default)]
    pub constraint_arcs: bool,
}

/// Deserialize the cell `name` field as a non-empty `Vec<Symbol>` (order preserving). Accepts either a
/// scalar (`name = "INV"`) or a list (`name = ["INVX1", "INVX2"]`); `Symbol` has no `serde` impl, so
/// each entry is read as a `String` and interned (Display/Debug/Ord delegate to `str`, so the emitted
/// bytes are unchanged). Duplicates are dropped keeping the first occurrence, and an empty list is a
/// hard error.
fn de_name_list<'de, D: serde::Deserializer<'de>>(d: D) -> Result<Vec<Symbol>, D::Error> {
    // String variant FIRST so a TOML scalar matches `One` rather than being probed as a sequence.
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum OneOrMany {
        One(String),
        Many(Vec<String>),
    }
    let names = match OneOrMany::deserialize(d)? {
        OneOrMany::One(s) => vec![s],
        OneOrMany::Many(v) => v,
    };
    let mut out: Vec<Symbol> = Vec::new();
    for s in names {
        let sym = Symbol::from(s);
        if !out.contains(&sym) {
            out.push(sym);
        }
    }
    if out.is_empty() {
        return Err(serde::de::Error::custom("cell name list must be non-empty"));
    }
    Ok(out)
}

/// Deserialize a list of name fields as `Vec<Symbol>` (order preserved), interning each entry.
fn de_symbol_vec<'de, D: serde::Deserializer<'de>>(d: D) -> Result<Vec<Symbol>, D::Error> {
    Vec::<String>::deserialize(d).map(|v| v.into_iter().map(Symbol::from).collect())
}

/// Deserialize a `name -> function` table as `IndexMap<Symbol, String>`, interning the keys and keeping
/// the function text (and the insertion order) as-is.
fn de_symbol_map<'de, D: serde::Deserializer<'de>>(
    d: D,
) -> Result<IndexMap<Symbol, String>, D::Error> {
    IndexMap::<String, String>::deserialize(d)
        .map(|m| m.into_iter().map(|(k, v)| (Symbol::from(k), v)).collect())
}

#[derive(Debug, Error)]
pub enum ModelError {
    #[error("cannot parse spec: {0}")]
    Spec(#[from] toml::de::Error),
    #[error("cell name list must be non-empty")]
    EmptyName,
    #[error("duplicate cell name {name:?} used by more than one cell")]
    DuplicateCellName { name: Symbol },
    #[error("cell {cell:?}: cannot parse function for output {output:?}: {source}")]
    Function {
        cell: Symbol,
        output: Symbol,
        #[source]
        source: ParseBoolExprError,
    },
    #[error("cell {cell:?}: duplicate input pin {pin:?}")]
    DuplicateInput { cell: Symbol, pin: Symbol },
    #[error("cell {cell:?}: pin {pin:?} is both an input and an output")]
    InputOutputClash { cell: Symbol, pin: Symbol },
    #[error("cell {cell:?}: internal signal {pin:?} clashes with a declared input or output name")]
    InternalClash { cell: Symbol, pin: Symbol },
    #[error("cell {cell:?}, output {output:?}: variable {var:?} is neither a declared input nor an output of this cell")]
    UnknownVar {
        cell: Symbol,
        output: Symbol,
        var: Symbol,
    },
    #[error("cell {cell:?}: async pin {pin:?} is not a declared input")]
    AsyncNotInput { cell: Symbol, pin: Symbol },
    #[error("cell {cell:?}: clock pin {pin:?} is not a declared input")]
    ClockNotInput { cell: Symbol, pin: Symbol },
}

/// A signal (output **or** internal) after analysis: its function, the variables it references, and
/// the feedback/state variables among them (a signal-name reference = a delayed/feedback value).
#[derive(Debug)]
pub struct AnalysedOutput {
    pub name: Symbol,
    /// The parsed function, regenerated from the minimised BDD when the rewrite changed it.
    /// DISPLAY-ONLY — analysis reads the shared BDD map, never this field.
    pub expr: BoolExpr,
    pub vars: BTreeSet<Symbol>,
    /// Signal names (outputs then internals) referenced by this function — its feedback/state — in
    /// the cell's signal order.
    pub feedback: Vec<Symbol>,
}

/// A cell after validation/analysis.
#[derive(Debug)]
pub struct AnalysedCell {
    pub name: Vec<Symbol>,
    pub inputs: Vec<Symbol>,
    pub outputs: Vec<AnalysedOutput>,
    /// Internal state variables: driven state signals with no external pin. Referenceable by any
    /// function; never an arc source or target. Relay/alias internals are folded away by the
    /// state-space minimisation in [`Cell::analyse`], so only genuine-memory internals survive here.
    pub internals: Vec<AnalysedOutput>,
    pub async_pins: Vec<Symbol>,
    /// The transition arcs derived for the cell's outputs, precomputed once by the shared machine pass
    /// ([`crate::logic::analysis::analyse_machine`]) and consumed by the arcs emitter.
    pub arcs: Vec<Arc>,
    /// The whole-cell internal-power ('hidden') arcs — single input toggles that settle but leave every
    /// output unchanged — precomputed once by the shared machine pass
    /// ([`crate::logic::analysis::analyse_machine`]) and consumed by the arcs emitter.
    pub hidden_arcs: Vec<HiddenArc>,
    /// The cell's static leakage states — the settled seed states of the machine exploration —
    /// precomputed once by the shared machine pass
    /// ([`crate::logic::analysis::analyse_machine`]) and consumed by the arcs emitter.
    pub leakage: Vec<LeakageState>,
    /// Detected order-dependent hazards — pairs whose settled state depends on which edge lands first
    /// (empty for confluent cells). A detected hazard, sibling to `oscillation`; the constraints that
    /// avoid it are generated separately into `constraints`. See [`crate::logic::hazard`].
    pub order_dependence: Vec<OrderDependence>,
    /// Detected oscillation hazards — pairs (or single toggles) that drive a periodic, non-settling
    /// cycle (empty for ordinary combinational or self-holding cells). See [`crate::logic::hazard`].
    pub oscillation: Vec<Oscillation>,
    /// Declared clock input pins (`clock = [...]`). See [`crate::logic::confluence`].
    pub clock_pins: Vec<Symbol>,
    /// The constraints generated to avoid the cell's detected hazards (setup/hold and non_seq). Emission
    /// is gated by the CLI flag or `constraint_arcs_declared`; the kind of each constraint follows the
    /// declared clock.
    pub constraints: Vec<Constraint>,
    /// Whether the cell opted in to constraint-arc emission (`constraint_arcs = true`).
    pub constraint_arcs_declared: bool,
    /// Each signal's state-table regions, precomputed once and cached in `signals()` order (outputs
    /// then internals), so emitters don't rebuild the BDDs per call site.
    pub regions: Vec<crate::logic::regions::StateRegions>,
}

impl AnalysedCell {
    /// The representative (first-as-written) cell name, for single-name contexts (diagnostics and the
    /// still-single-name emitter paths). Safe to index: `de_name_list` rejects empty name lists.
    pub fn repr_name(&self) -> &Symbol {
        &self.name[0]
    }

    /// Every state-bearing signal: outputs first, then internals, in declaration order.
    pub fn signals(&self) -> impl Iterator<Item = &AnalysedOutput> {
        self.outputs.iter().chain(self.internals.iter())
    }

    /// Each signal paired with its cached state-table regions, in `signals()` order (outputs then
    /// internals).
    pub fn signal_regions(
        &self,
    ) -> impl Iterator<Item = (&AnalysedOutput, &crate::logic::regions::StateRegions)> {
        self.signals().zip(self.regions.iter())
    }
}

impl Spec {
    /// Validate cross-cell name uniqueness, then analyse every cell.
    ///
    /// The union of all cells' name lists must contain no name twice: a collision would emit duplicate
    /// Liberty `cell()` groups, duplicate Verilog modules and conflicting `define_arc` trailers. Intra-cell
    /// duplicates are already deduped by `de_name_list`, so the set-insert here catches inter-cell
    /// collisions (an alias colliding with another cell's name included). The per-cell analyses then run
    /// in parallel, matching the single machine pass minted per cell in [`Cell::analyse`].
    pub fn analyse(&self) -> Result<Vec<AnalysedCell>, ModelError> {
        let mut seen: BTreeSet<Symbol> = BTreeSet::new();
        for cell in &self.cells {
            for name in &cell.name {
                if !seen.insert(name.clone()) {
                    return Err(ModelError::DuplicateCellName { name: name.clone() });
                }
            }
        }
        self.cells.par_iter().map(|c| c.analyse()).collect()
    }
}

impl Cell {
    /// Validate the cell and parse its functions, classifying each referenced variable as a primary
    /// input, an output, or an internal signal (feedback/state = a signal-name reference).
    pub fn analyse(&self) -> Result<AnalysedCell, ModelError> {
        let mut analysed = self.analyse_signals()?;

        // One-shot state-space rewrite: mint the cell's single builder, build every signal's BDD once,
        // and run the minimisation (identical-δ dedup + guarded relay/alias fold, alternated to a
        // fixpoint). It rewrites the map in place so every surviving signal is a genuine-memory
        // coordinate; the same map is then shared by the machine pass, the region cache and emission —
        // no signal function is ever rebuilt.
        let builder = sync_bdd_builder!();
        let mut bdds = build_signal_bdds(&analysed, &builder);
        let order: Vec<Symbol> = analysed.signals().map(|s| s.name.clone()).collect();
        let output_set: BTreeSet<Symbol> =
            analysed.outputs.iter().map(|o| o.name.clone()).collect();
        let min = crate::logic::minimise::minimise_state_space(&mut bdds, &order, &output_set);

        recompute_signal_metadata(&mut analysed, &bdds, &min);

        // Build the cell's state machine once and derive both its transition arcs and its hazards from
        // the shared exploration over the minimised model: the two detected hazards (order-dependence,
        // oscillation) and the constraints — setup/hold, non_seq — generated to avoid them. Clock
        // suppression and emission gating are applied downstream.
        let analysis = crate::logic::analysis::analyse_machine(&analysed, &bdds);
        analysed.arcs = analysis.arcs;
        analysed.hidden_arcs = analysis.hidden_arcs;
        analysed.leakage = analysis.leakage;
        analysed.constraints = analysis.constraints;
        analysed.order_dependence = analysis.order_dependence;
        analysed.oscillation = analysis.oscillation;

        // Cache each signal's state-table regions once, in `signals()` order, from the shared folded
        // BDDs, so downstream emitters don't rebuild the BDDs per call site.
        analysed.regions = derive_regions(&analysed, &bdds);
        Ok(analysed)
    }

    /// Validate the cell and parse its functions into the pre-minimise [`AnalysedCell`]: every signal's
    /// parse-time support and feedback classification, with all derived analysis fields
    /// (arcs/hidden_arcs/leakage/order_dependence/oscillation/constraints/regions) still empty. The
    /// state-space rewrite and machine/region passes are layered on by [`Cell::analyse`].
    pub fn analyse_signals(&self) -> Result<AnalysedCell, ModelError> {
        // Programmatic guard: `de_name_list` rejects an empty name list on deserialisation, but `Cell`
        // has all-pub fields, so a hand-built `Cell { name: vec![], .. }` bypasses it and would panic on
        // the pervasive `self.name[0]` / `repr_name()` indexing. Reject it here, before the first index.
        if self.name.is_empty() {
            return Err(ModelError::EmptyName);
        }

        let mut input_set = BTreeSet::new();
        for pin in &self.inputs {
            if !input_set.insert(pin.clone()) {
                return Err(ModelError::DuplicateInput {
                    cell: self.name[0].clone(),
                    pin: pin.clone(),
                });
            }
        }

        let output_names: Vec<Symbol> = self.outputs.keys().cloned().collect();
        let output_set: BTreeSet<Symbol> = output_names.iter().cloned().collect();
        let internal_names: Vec<Symbol> = self.internal.keys().cloned().collect();
        let internal_set: BTreeSet<Symbol> = internal_names.iter().cloned().collect();

        for pin in &self.inputs {
            if output_set.contains(pin) {
                return Err(ModelError::InputOutputClash {
                    cell: self.name[0].clone(),
                    pin: pin.clone(),
                });
            }
        }
        for name in &internal_names {
            if input_set.contains(name) || output_set.contains(name) {
                return Err(ModelError::InternalClash {
                    cell: self.name[0].clone(),
                    pin: name.clone(),
                });
            }
        }
        for pin in &self.async_pins {
            if !input_set.contains(pin) {
                return Err(ModelError::AsyncNotInput {
                    cell: self.name[0].clone(),
                    pin: pin.clone(),
                });
            }
        }
        for pin in &self.clock {
            if !input_set.contains(pin) {
                return Err(ModelError::ClockNotInput {
                    cell: self.name[0].clone(),
                    pin: pin.clone(),
                });
            }
        }

        // Signal order: outputs first, then internals. Feedback references are classified against it.
        let signal_names: Vec<Symbol> = output_names
            .iter()
            .cloned()
            .chain(internal_names.iter().cloned())
            .collect();

        // Parse every function (outputs then internals) into one signal list.
        let n_outputs = self.outputs.len();
        let mut all: Vec<AnalysedOutput> = Vec::with_capacity(n_outputs + self.internal.len());
        for (name, func) in self.outputs.iter().chain(self.internal.iter()) {
            let parsed = expr::parse(func).map_err(|source| ModelError::Function {
                cell: self.name[0].clone(),
                output: name.clone(),
                source,
            })?;
            for v in &parsed.vars {
                if !input_set.contains(v) && !output_set.contains(v) && !internal_set.contains(v) {
                    return Err(ModelError::UnknownVar {
                        cell: self.name[0].clone(),
                        output: name.clone(),
                        var: v.clone(),
                    });
                }
            }
            let feedback: Vec<Symbol> = signal_names
                .iter()
                .filter(|s| parsed.vars.contains(*s))
                .cloned()
                .collect();
            all.push(AnalysedOutput {
                name: name.clone(),
                expr: parsed.expr,
                vars: parsed.vars,
                feedback,
            });
        }

        let internals = all.split_off(n_outputs);
        let outputs = all;

        let analysed = AnalysedCell {
            name: self.name.clone(),
            inputs: self.inputs.clone(),
            outputs,
            internals,
            async_pins: self.async_pins.clone(),
            arcs: Vec::new(),
            hidden_arcs: Vec::new(),
            leakage: Vec::new(),
            order_dependence: Vec::new(),
            oscillation: Vec::new(),
            clock_pins: self.clock.clone(),
            constraints: Vec::new(),
            constraint_arcs_declared: self.constraint_arcs,
            regions: Vec::new(),
        };
        Ok(analysed)
    }
}

/// Build every signal's BDD once from the shared per-cell `builder`, keyed by signal name in
/// `signals()` order (outputs then internals).
///
/// Pure over `signals()`/`expr`, so it yields the same map whether `cell` is pre-minimise (parse-time
/// functions) or post-minimise (folded functions) — the caller re-derives from whichever `expr` each
/// signal currently holds. The builder is minted exactly once per cell in [`Cell::analyse`].
pub fn build_signal_bdds<B: Brand, C: ManagerCell>(
    cell: &AnalysedCell,
    builder: &BddBuilder<B, C>,
) -> BTreeMap<Symbol, Bdd<B, C>> {
    cell.signals()
        .map(|s| (s.name.clone(), builder.build(&s.expr)))
        .collect()
}

/// Recompute each surviving signal's metadata from the minimised `bdds` after
/// [`crate::logic::minimise::minimise_state_space`].
///
/// First drop the internals the fold purged (outputs are never purged), then recompute every surviving
/// signal from its folded BDD: its support (now semantic, not the parse-time syntactic support) and the
/// feedback/state references among the survivors. The display expression is regenerated only when the
/// rewrite actually changed the function.
pub fn recompute_signal_metadata<B: Brand, C: ManagerCell>(
    cell: &mut AnalysedCell,
    bdds: &BTreeMap<Symbol, Bdd<B, C>>,
    min: &crate::logic::minimise::Minimised,
) {
    cell.internals.retain(|s| !min.purged.contains(&s.name));

    let surviving: Vec<Symbol> = cell.signals().map(|s| s.name.clone()).collect();
    for sig in cell.outputs.iter_mut().chain(cell.internals.iter_mut()) {
        sig.vars = bdds[&sig.name].variables().collect();
        sig.feedback = surviving
            .iter()
            .filter(|n| sig.vars.contains(n.as_str()))
            .cloned()
            .collect();
        if min.changed.contains(&sig.name) {
            sig.expr = bdds[&sig.name].to_expr();
        }
    }
}

/// Derive each signal's state-table regions from the shared folded `bdds`, in `signals()` order
/// (outputs then internals).
///
/// The cyclic state-variable set (over the recomputed feedback) decides each region's `hysteretic`
/// flag — a state variable must emit a `statetable`, never a combinational `function`. This is the
/// cheap pure-graph classifier, so it holds even for cells the machine-width guard skips. Returns the
/// region vector; it does not mutate `cell`.
pub fn derive_regions<B: Brand, C: ManagerCell>(
    cell: &AnalysedCell,
    bdds: &BTreeMap<Symbol, Bdd<B, C>>,
) -> Vec<crate::logic::regions::StateRegions> {
    let signals: Vec<&AnalysedOutput> = cell.signals().collect();
    let state_set = crate::logic::resolve::state_variables(&signals);
    cell.signals()
        .map(|s| {
            crate::logic::regions::state_regions(
                &s.name,
                &bdds[&s.name],
                state_set.contains(&s.name),
            )
        })
        .collect()
}

/// Parse a TOML spec into a [`Spec`].
pub fn parse_spec(toml_src: &str) -> Result<Spec, ModelError> {
    Ok(toml::from_str(toml_src)?)
}

/// Parse a single-cell TOML `src` and return its analysed form. The one canonical test helper, shared
/// by the in-crate `#[cfg(test)]` modules.
#[cfg(test)]
pub(crate) fn analyse_one(src: &str) -> AnalysedCell {
    parse_spec(src).unwrap().cells.remove(0).analyse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"
[[cell]]
name = "C2"
inputs = ["A", "B"]
[cell.outputs]
Q = "A*B + Q*(A+B)"

[[cell]]
name = "INV"
inputs = ["A"]
[cell.outputs]
Y = "!A"
"#;

    #[test]
    fn loads_and_classifies_feedback() {
        let spec = parse_spec(SAMPLE).unwrap();
        assert_eq!(spec.cells.len(), 2);

        let c2 = spec.cells[0].analyse().unwrap();
        assert_eq!(c2.name[0], "C2");
        assert_eq!(c2.inputs, ["A", "B"]);
        assert_eq!(c2.outputs.len(), 1);
        assert_eq!(c2.outputs[0].feedback, ["Q"]); // Q references itself => feedback/state

        let inv = spec.cells[1].analyse().unwrap();
        assert!(inv.outputs[0].feedback.is_empty()); // purely combinational
    }

    #[test]
    fn preserves_output_order() {
        let s = r#"
[[cell]]
name = "SR"
inputs = ["S", "R"]
[cell.outputs]
Q = "S + Q*!R"
Qn = "R + Qn*!S"
"#;
        let cell = parse_spec(s).unwrap().cells.remove(0).analyse().unwrap();
        let names: Vec<_> = cell.outputs.iter().map(|o| o.name.as_str()).collect();
        assert_eq!(names, ["Q", "Qn"]);
    }

    #[test]
    fn rejects_unknown_var() {
        let s = r#"
[[cell]]
name = "X"
inputs = ["A"]
[cell.outputs]
Y = "A*Z"
"#;
        let err = parse_spec(s).unwrap().cells[0].analyse().unwrap_err();
        assert!(matches!(err, ModelError::UnknownVar { .. }));
    }

    #[test]
    fn rejects_unknown_var_in_internal() {
        // An undefined variable is rejected wherever it appears — an internal function, not just an output.
        let s = r#"
[[cell]]
name = "X"
inputs = ["A"]
[cell.internal]
W = "A*Z"
[cell.outputs]
Y = "W"
"#;
        let err = parse_spec(s).unwrap().cells[0].analyse().unwrap_err();
        assert!(matches!(err, ModelError::UnknownVar { var, .. } if var == "Z"));
    }

    #[test]
    fn multiple_errors_report_the_first_deterministically() {
        // Two outputs each reference an undefined variable. Analysis short-circuits on the first in a
        // fixed traversal order (outputs in declaration order), so the reported error is stable across
        // repeated parses — never dependent on hash-map iteration.
        let s = r#"
[[cell]]
name = "MULTI"
inputs = ["A"]
[cell.outputs]
Y1 = "A*Z1"
Y2 = "A*Z2"
"#;
        let first = parse_spec(s).unwrap().cells[0]
            .analyse()
            .unwrap_err()
            .to_string();
        for _ in 0..8 {
            let again = parse_spec(s).unwrap().cells[0]
                .analyse()
                .unwrap_err()
                .to_string();
            assert_eq!(again, first, "error reporting must be deterministic");
        }
        assert!(
            first.contains("Z1") && !first.contains("Z2"),
            "the first-declared offending output is reported first: {first}",
        );
    }

    #[test]
    fn rejects_unknown_cell_key() {
        // A misspelt or stale spec key must be a hard error, not silently ignored.
        let s = r#"
[[cell]]
name = "X"
inputs = ["A"]
oscillate = ["Q"]
[cell.outputs]
Y = "A"
"#;
        assert!(matches!(parse_spec(s), Err(ModelError::Spec(_))));
    }

    #[test]
    fn internal_signal_is_classified_and_kept_off_the_output_list() {
        // A DFF: internal master latch M, external slave output Q referencing M.
        let s = r#"
[[cell]]
name = "DFF"
inputs = ["CLK", "D"]
[cell.internal]
M = "!CLK*D + CLK*M"
[cell.outputs]
Q = "CLK*M + !CLK*Q"
"#;
        let cell = parse_spec(s).unwrap().cells.remove(0).analyse().unwrap();
        // M is internal, not an output.
        let out_names: Vec<_> = cell.outputs.iter().map(|o| o.name.as_str()).collect();
        assert_eq!(out_names, ["Q"]);
        let int_names: Vec<_> = cell.internals.iter().map(|o| o.name.as_str()).collect();
        assert_eq!(int_names, ["M"]);
        // Q references the internal M as feedback/state.
        assert!(cell.outputs[0].feedback.iter().any(|s| s == "M"));
        assert!(cell.outputs[0].feedback.iter().any(|s| s == "Q"));
        // signals() yields outputs then internals.
        let sig_names: Vec<_> = cell.signals().map(|s| s.name.as_str()).collect();
        assert_eq!(sig_names, ["Q", "M"]);
        // Not flagged as an arbiter (Q→M is a one-way dependency, no mutual cycle).
        assert!(cell.oscillation.is_empty());
    }

    #[test]
    fn internal_referenced_by_function_is_a_known_var() {
        // An internal name used in an output function must not be rejected as UnknownVar.
        let s = r#"
[[cell]]
name = "X"
inputs = ["A"]
[cell.internal]
W = "A"
[cell.outputs]
Y = "W"
"#;
        assert!(parse_spec(s).unwrap().cells[0].analyse().is_ok());
    }

    #[test]
    fn internal_clashing_with_output_errors() {
        let s = r#"
[[cell]]
name = "X"
inputs = ["A"]
[cell.internal]
Q = "A"
[cell.outputs]
Q = "A + Q"
"#;
        let err = parse_spec(s).unwrap().cells[0].analyse().unwrap_err();
        assert!(matches!(err, ModelError::InternalClash { .. }));
    }

    #[test]
    fn name_scalar_parses_as_single_entry() {
        let s = r#"
[[cell]]
name = "INV"
inputs = ["A"]
[cell.outputs]
Y = "!A"
"#;
        let cell = parse_spec(s).unwrap().cells.remove(0).analyse().unwrap();
        assert_eq!(cell.name, vec![Symbol::from("INV")]);
    }

    #[test]
    fn name_list_preserves_written_order() {
        let s = r#"
[[cell]]
name = ["INVX1", "INVX2"]
inputs = ["A"]
[cell.outputs]
Y = "!A"
"#;
        let cell = parse_spec(s).unwrap().cells.remove(0).analyse().unwrap();
        assert_eq!(
            cell.name,
            vec![Symbol::from("INVX1"), Symbol::from("INVX2")]
        );
    }

    #[test]
    fn name_list_dedups_preserving_order() {
        let s = r#"
[[cell]]
name = ["A", "A", "B"]
inputs = ["I"]
[cell.outputs]
Y = "I"
"#;
        let cell = parse_spec(s).unwrap().cells.remove(0).analyse().unwrap();
        assert_eq!(cell.name, vec![Symbol::from("A"), Symbol::from("B")]);
    }

    #[test]
    fn empty_name_list_is_rejected() {
        let s = r#"
[[cell]]
name = []
inputs = ["A"]
[cell.outputs]
Y = "A"
"#;
        let err = parse_spec(s).unwrap_err().to_string();
        assert!(
            err.contains("cell name list must be non-empty"),
            "unexpected error: {err}",
        );
    }

    #[test]
    fn duplicate_cell_name_across_cells_is_rejected() {
        // The same physical name declared by two cells would emit duplicate Liberty/Verilog groups.
        let s = r#"
[[cell]]
name = "DUP"
inputs = ["A"]
[cell.outputs]
Y = "A"

[[cell]]
name = "DUP"
inputs = ["A"]
[cell.outputs]
Z = "A"
"#;
        let err = parse_spec(s).unwrap().analyse().unwrap_err();
        assert!(matches!(err, ModelError::DuplicateCellName { name } if name == "DUP"));
    }

    #[test]
    fn alias_colliding_with_another_cell_name_is_rejected() {
        // An alias in one cell's list colliding with a second cell's scalar name is still a collision.
        let s = r#"
[[cell]]
name = ["FOO", "BAR"]
inputs = ["A"]
[cell.outputs]
Y = "A"

[[cell]]
name = "BAR"
inputs = ["A"]
[cell.outputs]
Z = "A"
"#;
        let err = parse_spec(s).unwrap().analyse().unwrap_err();
        assert!(matches!(err, ModelError::DuplicateCellName { name } if name == "BAR"));
    }

    #[test]
    fn programmatic_empty_name_errors_instead_of_panicking() {
        // `Cell` has all-pub fields, so a hand-built empty name list bypasses `de_name_list` and would
        // otherwise panic on `self.name[0]`. The guard returns `EmptyName` rather than panicking.
        let mut outputs = IndexMap::new();
        outputs.insert(Symbol::from("Y"), "A".to_string());
        let cell = Cell {
            name: vec![],
            inputs: vec![Symbol::from("A")],
            outputs,
            internal: IndexMap::new(),
            async_pins: vec![],
            clock: vec![],
            constraint_arcs: false,
        };
        let spec = Spec { cells: vec![cell] };
        let err = spec.analyse().unwrap_err();
        assert!(matches!(err, ModelError::EmptyName));
    }

    #[test]
    fn distinct_multi_name_cells_analyse_ok() {
        // A valid spec of several cells whose name lists are all distinct still analyses cleanly.
        let s = r#"
[[cell]]
name = ["INVX1", "INVX2"]
inputs = ["A"]
[cell.outputs]
Y = "!A"

[[cell]]
name = "BUF"
inputs = ["A"]
[cell.outputs]
Z = "A"
"#;
        let cells = parse_spec(s).unwrap().analyse().unwrap();
        assert_eq!(cells.len(), 2);
    }

    #[test]
    fn async_must_be_input() {
        let s = r#"
[[cell]]
name = "X"
inputs = ["A"]
async = ["R"]
[cell.outputs]
Y = "A"
"#;
        let err = parse_spec(s).unwrap().cells[0].analyse().unwrap_err();
        assert!(matches!(err, ModelError::AsyncNotInput { .. }));
    }
}
