//! Emit Cadence Liberate `define_cell` blocks — the structural cell declaration the transition arcs
//! attach to.
//!
//! Each block names the cell's pins (`-input`/`-clock`/`-async`/`-output`/`-pinlist`) and the
//! characterisation templates the cell carries (`-delay`/`-power`/`-constraint`). Clock and async pins
//! are split out of `-input` into their own flags but still appear verbatim in `-pinlist`; any flag
//! whose pin set is empty is omitted. The drive-strength aliases are bundled by their resolved
//! `(delay, power, constraint)` template triple — each alias inherits the cell-wide `template` unless
//! its `template_overrides` entry supplies a field — so aliases sharing a triple emit as one block, in
//! first-appearance order.
//!
//! Unlike the arcs emitter these blocks carry no `-type`/`-when`/`-related_pin`/`-function`: they are
//! purely the cell's structural declaration.
//!
//! A block travels as the value [`DefineCell`] holds and becomes text once, in
//! [`Display`](fmt::Display), written into the writer the `cells.tcl` is going out on.

use std::collections::BTreeSet;
use std::fmt;

use espresso_logic::Symbol;
use indexmap::IndexMap;

use crate::emit::arcs_tcl::pinlist;
use crate::emit::tcl::Words;
use crate::model::AnalysedCell;

/// A resolved template triple: the `(delay, power, constraint)` names an alias attaches, each `Some`
/// only when the alias override or the cell-wide template supplies it.
type Triple = (Option<Symbol>, Option<Symbol>, Option<Symbol>);

/// One `define_cell` block: the pins the cell declares, split into the flags Liberate reads them under,
/// the characterisation templates the block's aliases resolve to, and the aliases it speaks for.
pub struct DefineCell {
    /// The data inputs — the cell's inputs less its clock and async pins, which carry their own flags.
    inputs: Vec<Symbol>,
    outputs: Vec<Symbol>,
    clocks: Vec<Symbol>,
    async_pins: Vec<Symbol>,
    /// Every pin the arcs address by position: all inputs (clock and async included) then the outputs.
    pinlist: Vec<Symbol>,
    delay: Option<Symbol>,
    power: Option<Symbol>,
    constraint: Option<Symbol>,
    /// The drive-strength aliases this one block speaks for.
    names: Vec<Symbol>,
}

impl fmt::Display for DefineCell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "define_cell \\")?;
        // Data inputs only — an all-clock/all-async cell drops the flag entirely.
        if !self.inputs.is_empty() {
            writeln!(f, "\t-input {{ {} }} \\", Words(&self.inputs))?;
        }
        writeln!(f, "\t-output {{ {} }} \\", Words(&self.outputs))?;
        if !self.clocks.is_empty() {
            writeln!(f, "\t-clock {{ {} }} \\", Words(&self.clocks))?;
        }
        if !self.async_pins.is_empty() {
            writeln!(f, "\t-async {{ {} }} \\", Words(&self.async_pins))?;
        }
        // `-pinlist` is the arcs emitter's source of truth — all inputs (incl. clock + async) then
        // outputs — and is emitted unfiltered.
        writeln!(f, "\t-pinlist {{ {} }} \\", Words(&self.pinlist))?;
        if let Some(d) = &self.delay {
            writeln!(f, "\t-delay {d} \\")?;
        }
        if let Some(p) = &self.power {
            writeln!(f, "\t-power {p} \\")?;
        }
        if let Some(c) = &self.constraint {
            writeln!(f, "\t-constraint {c} \\")?;
        }
        writeln!(f, "\t{{ {} }}", Words(&self.names))?;
        writeln!(f)
    }
}

/// A run's `define_cell` blocks as the text they write: each block in turn, written into the writer the
/// `cells.tcl` is going out on. Every cell's blocks make up the one file, so this holds them all.
pub struct Declarations<'a>(pub &'a [DefineCell]);

impl fmt::Display for Declarations<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for block in self.0 {
            write!(f, "{block}")?;
        }
        Ok(())
    }
}

/// All `define_cell` blocks for a cell, one per resolved template triple. Aliases sharing a triple are
/// bundled into a single block; the blocks follow the first-appearance order of their aliases.
pub fn cell_define_cell(cell: &AnalysedCell) -> Vec<DefineCell> {
    // Pin flags are group-independent, so compute them once. Clock and async pins are lifted out of
    // `-input` into their own flags (they still appear in `-pinlist`, which is untouched).
    let excluded: BTreeSet<&Symbol> = cell.async_pins.iter().chain(&cell.clock_pins).collect();
    let data_inputs: Vec<Symbol> = cell
        .inputs
        .iter()
        .filter(|p| !excluded.contains(p))
        .cloned()
        .collect();
    let outputs: Vec<Symbol> = cell.outputs.iter().map(|o| o.name.clone()).collect();

    // Resolve one alias's template triple: the alias override wins per field, else the cell-wide
    // template, else `None`.
    let resolve = |name: &Symbol| -> Triple {
        let ov = cell.template_overrides.get(name);
        let def = cell.template.as_ref();
        let delay = ov
            .and_then(|o| o.delay.clone())
            .or_else(|| def.and_then(|d| d.delay.clone()));
        let power = ov
            .and_then(|o| o.power.clone())
            .or_else(|| def.and_then(|d| d.power.clone()));
        let constraint = ov
            .and_then(|o| o.constraint.clone())
            .or_else(|| def.and_then(|d| d.constraint.clone()));
        (delay, power, constraint)
    };

    // Bundle the aliases by resolved triple. `IndexMap` insertion order groups by first appearance and
    // keeps each group's aliases in written order.
    let mut groups: IndexMap<Triple, Vec<Symbol>> = IndexMap::new();
    for alias in &cell.name {
        groups
            .entry(resolve(alias))
            .or_default()
            .push(alias.clone());
    }

    // The pin flags and the `-pinlist` are the cell's own, so every block states the same ones; what a
    // block adds of its own is its template triple and the aliases that resolved to it.
    let pinlist = pinlist(cell);
    groups
        .into_iter()
        .map(|((delay, power, constraint), names)| DefineCell {
            inputs: data_inputs.clone(),
            outputs: outputs.clone(),
            clocks: cell.clock_pins.clone(),
            async_pins: cell.async_pins.clone(),
            pinlist: pinlist.clone(),
            delay,
            power,
            constraint,
            names,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::analyse_one as analyse;

    /// Emit the `define_cell` blocks for a single-cell TOML spec, as the text the sink writes.
    fn emit(src: &str) -> String {
        Declarations(&cell_define_cell(&analyse(src))).to_string()
    }

    /// The first `define_cell` block fragment containing `needle` (split on the block keyword).
    fn block_with<'a>(tcl: &'a str, needle: &str) -> &'a str {
        tcl.split("define_cell")
            .find(|b| b.contains(needle))
            .unwrap_or_else(|| panic!("no block containing {needle:?} in:\n{tcl}"))
    }

    /// (a) A single cell-wide template shared by every alias emits ONE block carrying all three
    /// template flags and one braced group naming all aliases.
    #[test]
    fn single_default_triple_multi_name_one_block() {
        let tcl = emit(
            r#"
[[cell]]
name = ["INVX1", "INVX2"]
inputs = ["A"]
[cell.outputs]
Y = "!A"
[cell.template]
delay = "dt"
power = "pt"
constraint = "ct"
"#,
        );
        eprintln!("{tcl}");
        assert_eq!(tcl.matches("define_cell").count(), 1);
        assert!(tcl.contains("-delay dt \\"));
        assert!(tcl.contains("-power pt \\"));
        assert!(tcl.contains("-constraint ct \\"));
        assert!(tcl.contains("{ INVX1 INVX2 }"));
    }

    /// (b) No template at all: ONE block, no template flags, all aliases braced together.
    #[test]
    fn no_template_one_block_no_template_flags() {
        let tcl = emit(
            r#"
[[cell]]
name = ["INVX1", "INVX2"]
inputs = ["A"]
[cell.outputs]
Y = "!A"
"#,
        );
        eprintln!("{tcl}");
        assert_eq!(tcl.matches("define_cell").count(), 1);
        assert!(!tcl.contains("-delay"));
        assert!(!tcl.contains("-power"));
        assert!(!tcl.contains("-constraint"));
        assert!(tcl.contains("{ INVX1 INVX2 }"));
    }

    /// (c) An override that changes a field splits the aliases into TWO blocks with the correct
    /// partition.
    #[test]
    fn override_splits_into_two_blocks() {
        let tcl = emit(
            r#"
[[cell]]
name = ["INVX1", "INVX2"]
inputs = ["A"]
[cell.outputs]
Y = "!A"
[cell.template]
delay = "dt"
[cell.template_overrides.INVX2]
delay = "dt2"
"#,
        );
        eprintln!("{tcl}");
        assert_eq!(tcl.matches("define_cell").count(), 2);
        assert!(block_with(&tcl, "{ INVX1 }").contains("-delay dt \\"));
        assert!(block_with(&tcl, "{ INVX2 }").contains("-delay dt2 \\"));
    }

    /// (d) Two non-adjacent aliases share a triple while the middle one is overridden: they group
    /// together (`{ A C }`) ahead of the odd one (`{ B }`), in first-appearance order.
    #[test]
    fn non_adjacent_same_triple_first_appearance_order() {
        let tcl = emit(
            r#"
[[cell]]
name = ["A", "B", "C"]
inputs = ["I"]
[cell.outputs]
Y = "I"
[cell.template]
delay = "dt"
[cell.template_overrides.B]
delay = "other"
"#,
        );
        eprintln!("{tcl}");
        assert_eq!(tcl.matches("define_cell").count(), 2);
        let ac = tcl.find("{ A C }").expect("A and C share a group");
        let b = tcl.find("{ B }").expect("B is its own group");
        assert!(
            ac < b,
            "first-appearance order puts {{ A C }} before {{ B }}"
        );
    }

    /// (e) A template that sets only delay + power emits no `-constraint` flag.
    #[test]
    fn omitted_constraint_no_constraint_flag() {
        let tcl = emit(
            r#"
[[cell]]
name = "INV"
inputs = ["A"]
[cell.outputs]
Y = "!A"
[cell.template]
delay = "dt"
power = "pt"
"#,
        );
        eprintln!("{tcl}");
        assert!(tcl.contains("-delay dt \\"));
        assert!(tcl.contains("-power pt \\"));
        assert!(!tcl.contains("-constraint"));
    }

    /// (f) An override that supplies only delay MERGES over the cell-wide template: its block keeps the
    /// default power + constraint and takes the overridden delay.
    #[test]
    fn override_merges_over_default_fields() {
        let tcl = emit(
            r#"
[[cell]]
name = ["INVX1", "INVX2"]
inputs = ["A"]
[cell.outputs]
Y = "!A"
[cell.template]
delay = "dt"
power = "pt"
constraint = "ct"
[cell.template_overrides.INVX2]
delay = "dt2"
"#,
        );
        eprintln!("{tcl}");
        let inv2 = block_with(&tcl, "{ INVX2 }");
        assert!(inv2.contains("-delay dt2 \\"));
        assert!(inv2.contains("-power pt \\"));
        assert!(inv2.contains("-constraint ct \\"));
    }

    /// (g) The template key is accepted under both spellings, `constraint` and the older `constrain`,
    /// and either emits the `-constraint` flag.
    #[test]
    fn constrain_spelling_is_accepted_as_constraint() {
        for key in ["constraint", "constrain"] {
            let tcl = emit(&format!(
                r#"
[[cell]]
name = "INV"
inputs = ["A"]
[cell.outputs]
Y = "!A"
[cell.template]
{key} = "ct"
"#,
            ));
            eprintln!("{tcl}");
            assert!(tcl.contains("-constraint ct \\"), "{key} spelling");
        }
    }

    /// (h) A declared async pin is lifted out of `-input` into `-async`, yet still appears in
    /// `-pinlist`.
    #[test]
    fn async_split_excludes_input_keeps_pinlist() {
        let tcl = emit(
            r#"
[[cell]]
name = "RC2"
inputs = ["A", "B", "R"]
async = ["R"]
[cell.outputs]
Q = "(A*B + Q*(A+B))*!R"
"#,
        );
        eprintln!("{tcl}");
        assert!(tcl.contains("-input { A B }"));
        assert!(tcl.contains("-async { R }"));
        assert!(tcl.contains("-pinlist { A B R Q }"));
    }

    /// (i) A cell with no async pins emits no `-async` flag.
    #[test]
    fn no_async_no_async_flag() {
        let tcl = emit(
            r#"
[[cell]]
name = "AND2"
inputs = ["A", "B"]
[cell.outputs]
Y = "A*B"
"#,
        );
        eprintln!("{tcl}");
        assert!(!tcl.contains("-async"));
        assert!(tcl.contains("-input { A B }"));
    }

    /// (j) An internal state node is excluded from both `-output` and `-pinlist`.
    #[test]
    fn internals_excluded_from_output_and_pinlist() {
        let tcl = emit(
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
        eprintln!("{tcl}");
        // Only the external output Q is listed; the internal master latch M never appears.
        assert!(tcl.contains("-output { Q }"));
        assert!(tcl.contains("-pinlist { CLK D Q }"));
    }

    /// (k) When every input is async, `-input` is dropped entirely — but `-pinlist` still lists them.
    #[test]
    fn all_inputs_async_drops_input_keeps_pinlist() {
        let tcl = emit(
            r#"
[[cell]]
name = "AR"
inputs = ["S", "R"]
async = ["S", "R"]
[cell.outputs]
Q = "!R*(S + Q)"
"#,
        );
        eprintln!("{tcl}");
        assert!(!tcl.contains("-input"));
        assert!(tcl.contains("-async { S R }"));
        assert!(tcl.contains("-pinlist { S R Q }"));
    }

    /// (l) A declared clock pin is lifted out of `-input` into `-clock`, yet still appears in
    /// `-pinlist`.
    #[test]
    fn clock_split_excludes_input_keeps_pinlist() {
        let tcl = emit(
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
        eprintln!("{tcl}");
        assert!(tcl.contains("-input { D }"));
        assert!(tcl.contains("-clock { CLK }"));
        assert!(tcl.contains("-pinlist { CLK D Q }"));
    }

    /// (m) A cell with no clock pins emits no `-clock` flag.
    #[test]
    fn no_clock_no_clock_flag() {
        let tcl = emit(
            r#"
[[cell]]
name = "AND2"
inputs = ["A", "B"]
[cell.outputs]
Y = "A*B"
"#,
        );
        eprintln!("{tcl}");
        assert!(!tcl.contains("-clock"));
    }
}
