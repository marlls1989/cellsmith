//! Input model: a minimal multi-cell TOML spec, plus analysis that classifies each function's
//! variables into **primary inputs** vs **feedback/state** (an output name referenced inside a
//! function is the delayed/feedback value of that output).

use std::collections::BTreeSet;

use espresso_logic::BoolExpr;
use indexmap::IndexMap;
use serde::Deserialize;
use thiserror::Error;

use crate::expr::{self, ParseError};
use crate::logic::interlock::{self, Arbitration};

/// The whole input file: a list of `[[cell]]` tables.
#[derive(Debug, Deserialize)]
pub struct Spec {
    #[serde(rename = "cell", default)]
    pub cells: Vec<Cell>,
}

/// One cell exactly as written in the TOML.
#[derive(Debug, Deserialize)]
pub struct Cell {
    /// Physical cell name used in the emitted arcs.
    pub name: String,
    /// Primary input pins. Order matters: it defines the pinlist/vector order.
    pub inputs: Vec<String>,
    /// Output pin name -> Boolean function. Order preserved (stable output order).
    pub outputs: IndexMap<String, String>,
    /// Optional: internal state variable name -> Boolean function. Order preserved. An internal
    /// signal is referenceable by other functions and is a driven state variable (modelled in the
    /// Verilog and the Liberty state table), but emits **no** external output pin and is never an arc
    /// source or target.
    #[serde(default)]
    pub internal: IndexMap<String, String>,
    /// Optional: input pins that force the output regardless of held state (async set/reset),
    /// so their arcs are emitted as `-type async` rather than combinational.
    #[serde(rename = "async", default)]
    pub async_pins: Vec<String>,
    /// Optional: a set of mutually-exclusive outputs (a mutex/arbiter's grant lines). Declaring it
    /// asserts the cell is interlocked and is validated against the detected arbitration; when
    /// omitted, arbitration is still detected (and a warning surfaced) but not required.
    #[serde(default)]
    pub arbitrate: Vec<String>,
}

#[derive(Debug, Error)]
pub enum ModelError {
    #[error("cell {cell:?}: cannot parse function for output {output:?}: {source}")]
    Function {
        cell: String,
        output: String,
        #[source]
        source: ParseError,
    },
    #[error("cell {cell:?}: duplicate input pin {pin:?}")]
    DuplicateInput { cell: String, pin: String },
    #[error("cell {cell:?}: pin {pin:?} is both an input and an output")]
    InputOutputClash { cell: String, pin: String },
    #[error("cell {cell:?}: internal signal {pin:?} clashes with a declared input or output name")]
    InternalClash { cell: String, pin: String },
    #[error("cell {cell:?}, output {output:?}: variable {var:?} is neither a declared input nor an output of this cell")]
    UnknownVar {
        cell: String,
        output: String,
        var: String,
    },
    #[error("cell {cell:?}: async pin {pin:?} is not a declared input")]
    AsyncNotInput { cell: String, pin: String },
    #[error("cell {cell:?}: arbitrate pin {pin:?} is not a declared output")]
    ArbitrateNotOutput { cell: String, pin: String },
    #[error("cell {cell:?}: declared arbitrate group {declared:?} is not a mutually-coupled (interlocked) set of outputs; detected interlock groups: {detected:?}")]
    ArbitrateNotInterlocked {
        cell: String,
        declared: Vec<String>,
        detected: Vec<Vec<String>>,
    },
}

/// A signal (output **or** internal) after analysis: its function, the variables it references, and
/// the feedback/state variables among them (a signal-name reference = a delayed/feedback value).
#[derive(Debug)]
pub struct AnalysedOutput {
    pub name: String,
    pub expr: BoolExpr,
    pub vars: BTreeSet<String>,
    /// Signal names (outputs then internals) referenced by this function — its feedback/state — in
    /// the cell's signal order.
    pub feedback: Vec<String>,
}

/// A signal after analysis. Alias of [`AnalysedOutput`], which now models any state-bearing signal
/// (an external output or an internal variable).
pub type AnalysedSignal = AnalysedOutput;

/// A cell after validation/analysis.
#[derive(Debug)]
pub struct AnalysedCell {
    pub name: String,
    pub inputs: Vec<String>,
    pub outputs: Vec<AnalysedOutput>,
    /// Internal state variables: driven state signals with no external pin. Referenceable by any
    /// function; never an arc source or target.
    pub internals: Vec<AnalysedOutput>,
    pub async_pins: Vec<String>,
    /// Detected arbitration/metastability conditions (empty for ordinary combinational or
    /// self-holding cells). See [`crate::logic::interlock`].
    pub arbitration: Vec<Arbitration>,
    /// Whether the cell explicitly declared its arbitration set (`arbitrate = [...]`). When an
    /// interlock is detected but not declared, the CLI warns.
    pub arbitrate_declared: bool,
}

impl AnalysedCell {
    /// Every state-bearing signal: outputs first, then internals, in declaration order.
    pub fn signals(&self) -> impl Iterator<Item = &AnalysedOutput> {
        self.outputs.iter().chain(self.internals.iter())
    }
}

impl Cell {
    /// Validate the cell and parse its functions, classifying each referenced variable as a primary
    /// input, an output, or an internal signal (feedback/state = a signal-name reference).
    pub fn analyse(&self) -> Result<AnalysedCell, ModelError> {
        let mut input_set = BTreeSet::new();
        for pin in &self.inputs {
            if !input_set.insert(pin.clone()) {
                return Err(ModelError::DuplicateInput {
                    cell: self.name.clone(),
                    pin: pin.clone(),
                });
            }
        }

        let output_names: Vec<String> = self.outputs.keys().cloned().collect();
        let output_set: BTreeSet<String> = output_names.iter().cloned().collect();
        let internal_names: Vec<String> = self.internal.keys().cloned().collect();
        let internal_set: BTreeSet<String> = internal_names.iter().cloned().collect();

        for pin in &self.inputs {
            if output_set.contains(pin) {
                return Err(ModelError::InputOutputClash {
                    cell: self.name.clone(),
                    pin: pin.clone(),
                });
            }
        }
        for name in &internal_names {
            if input_set.contains(name) || output_set.contains(name) {
                return Err(ModelError::InternalClash {
                    cell: self.name.clone(),
                    pin: name.clone(),
                });
            }
        }
        for pin in &self.async_pins {
            if !input_set.contains(pin) {
                return Err(ModelError::AsyncNotInput {
                    cell: self.name.clone(),
                    pin: pin.clone(),
                });
            }
        }

        // Signal order: outputs first, then internals. Feedback references are classified against it.
        let signal_names: Vec<String> = output_names
            .iter()
            .cloned()
            .chain(internal_names.iter().cloned())
            .collect();

        // Parse every function (outputs then internals) into one signal list.
        let n_outputs = self.outputs.len();
        let mut all: Vec<AnalysedOutput> = Vec::with_capacity(n_outputs + self.internal.len());
        for (name, func) in self.outputs.iter().chain(self.internal.iter()) {
            let parsed = expr::parse(func).map_err(|source| ModelError::Function {
                cell: self.name.clone(),
                output: name.clone(),
                source,
            })?;
            for v in &parsed.vars {
                if !input_set.contains(v) && !output_set.contains(v) && !internal_set.contains(v) {
                    return Err(ModelError::UnknownVar {
                        cell: self.name.clone(),
                        output: name.clone(),
                        var: v.clone(),
                    });
                }
            }
            let feedback: Vec<String> = signal_names
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

        // Detect arbitration/metastability over all state signals, then validate any explicit
        // `arbitrate` declaration against it.
        let arbitration = interlock::detect(&self.inputs, &all);
        let arbitrate_declared = !self.arbitrate.is_empty();
        if arbitrate_declared {
            for pin in &self.arbitrate {
                if !output_set.contains(pin) {
                    return Err(ModelError::ArbitrateNotOutput {
                        cell: self.name.clone(),
                        pin: pin.clone(),
                    });
                }
            }
            let declared: BTreeSet<&String> = self.arbitrate.iter().collect();
            let matches_group = arbitration
                .iter()
                .any(|a| a.group.iter().collect::<BTreeSet<_>>() == declared);
            if !matches_group {
                let mut detected: Vec<Vec<String>> =
                    arbitration.iter().map(|a| a.group.clone()).collect();
                detected.dedup();
                return Err(ModelError::ArbitrateNotInterlocked {
                    cell: self.name.clone(),
                    declared: self.arbitrate.clone(),
                    detected,
                });
            }
        }

        let internals = all.split_off(n_outputs);
        let outputs = all;

        Ok(AnalysedCell {
            name: self.name.clone(),
            inputs: self.inputs.clone(),
            outputs,
            internals,
            async_pins: self.async_pins.clone(),
            arbitration,
            arbitrate_declared,
        })
    }
}

/// Parse a TOML spec into a [`Spec`].
pub fn parse_spec(toml_src: &str) -> Result<Spec, toml::de::Error> {
    toml::from_str(toml_src)
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
        assert_eq!(c2.name, "C2");
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
    fn arbitrate_declaration_validates_against_detection() {
        let s = r#"
[[cell]]
name = "MUT"
inputs = ["A", "B"]
arbitrate = ["Qa", "Qb"]
[cell.outputs]
Qa = "!Qb * A"
Qb = "!Qa * B"
"#;
        let cell = parse_spec(s).unwrap().cells.remove(0).analyse().unwrap();
        assert!(cell.arbitrate_declared);
        assert_eq!(cell.arbitration.len(), 1);
        assert_eq!(cell.arbitration[0].group, ["Qa", "Qb"]);
    }

    #[test]
    fn arbitrate_on_non_interlocked_cell_errors() {
        // A plain C-element is not interlocked; declaring an arbitrate group must be rejected.
        let s = r#"
[[cell]]
name = "C2"
inputs = ["A", "B"]
arbitrate = ["Q"]
[cell.outputs]
Q = "A*B + Q*(A+B)"
"#;
        let err = parse_spec(s).unwrap().cells[0].analyse().unwrap_err();
        assert!(matches!(err, ModelError::ArbitrateNotInterlocked { .. }));
    }

    #[test]
    fn arbitrate_pin_must_be_output() {
        let s = r#"
[[cell]]
name = "MUT"
inputs = ["A", "B"]
arbitrate = ["Qa", "Zz"]
[cell.outputs]
Qa = "!Qb * A"
Qb = "!Qa * B"
"#;
        let err = parse_spec(s).unwrap().cells[0].analyse().unwrap_err();
        assert!(matches!(err, ModelError::ArbitrateNotOutput { .. }));
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
        assert!(cell.outputs[0].feedback.contains(&"M".to_string()));
        assert!(cell.outputs[0].feedback.contains(&"Q".to_string()));
        // signals() yields outputs then internals.
        let sig_names: Vec<_> = cell.signals().map(|s| s.name.as_str()).collect();
        assert_eq!(sig_names, ["Q", "M"]);
        // Not flagged as an arbiter (Q→M is a one-way dependency, no mutual cycle).
        assert!(cell.arbitration.is_empty());
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
