//! The rendering vocabulary of the diagnostics the run writes to standard error.
//!
//! A warning's subjects are the values the analysis already holds — a state is a
//! [`Minterm`](espresso_logic::Minterm) over the cell's signals, a path a sequence of them — and each
//! adapter here borrows one and writes it into the warning's own writer. Nothing is rendered ahead of
//! the write, so a subject travels as itself and becomes text once, where the warning is written.

use std::fmt;

use espresso_logic::{Minterm, Symbol};

/// One state as the values it fixes, in the minterm's variable order: `{A=1, B=0}`. A column the
/// minterm leaves free is no part of the state and is left out.
pub struct State<'a>(pub &'a Minterm<Symbol>);

impl fmt::Display for State<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("{")?;
        let fixed = self
            .0
            .vars()
            .iter()
            .zip(self.0.iter())
            .filter_map(|(name, value)| value.map(|v| (name, v)));
        for (i, (name, value)) in fixed.enumerate() {
            if i > 0 {
                f.write_str(", ")?;
            }
            write!(f, "{name}={}", u8::from(value))?;
        }
        f.write_str("}")
    }
}

/// A walk through the machine as the states it passes through, in order and joined by ` → `:
/// `{A=0, B=0} → {A=1, B=0}`.
pub struct Path<'a>(pub &'a [Minterm<Symbol>]);

impl fmt::Display for Path<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, state) in self.0.iter().enumerate() {
            if i > 0 {
                f.write_str(" → ")?;
            }
            write!(f, "{}", State(state))?;
        }
        Ok(())
    }
}

/// A list written one item after another, separated by `, `.
pub struct Commas<'a, T: fmt::Display>(pub &'a [T]);

impl<T: fmt::Display> fmt::Display for Commas<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, item) in self.0.iter().enumerate() {
            if i > 0 {
                f.write_str(", ")?;
            }
            write!(f, "{item}")?;
        }
        Ok(())
    }
}
