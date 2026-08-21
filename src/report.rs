//! The rendering vocabulary of the diagnostics the run writes to standard error.
//!
//! A warning's subjects are the values the analysis already holds — a state is a
//! [`Minterm`](espresso_logic::Minterm) over the cell's signals, a path a sequence of them — and each
//! adapter here borrows one and writes it into the warning's own writer. Nothing is rendered ahead of
//! the write, so a subject travels as itself and becomes text once, where the warning is written.

use std::fmt;

use espresso_logic::{Minterm, Symbol};

use crate::text::Joined;

/// One state as the values it fixes, in the minterm's variable order: `{A=1, B=0}`. A column the
/// minterm leaves free is no part of the state and is left out.
pub struct State<'a>(pub &'a Minterm<Symbol>);

impl fmt::Display for State<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("{")?;
        // Collected because `Joined::fmt` takes `&self` and clones its stored iterator to walk
        // it, and `Minterm::iter`'s `MintermIter` carries no `Clone`.
        let fixed: Vec<_> = self
            .0
            .vars()
            .iter()
            .zip(self.0.iter())
            .filter_map(|(name, value)| value.map(|v| (name, v)))
            .collect();
        Joined::new(fixed.iter(), ", ", |&(name, value)| Assignment(name, value)).fmt(f)?;
        f.write_str("}")
    }
}

/// One `name=value` pair inside a [`State`]: the variable and the value the minterm fixes it to.
struct Assignment<'a>(&'a Symbol, bool);

impl fmt::Display for Assignment<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}={}", self.0, u8::from(self.1))
    }
}

/// A walk through the machine as the states it passes through, in order and joined by ` → `:
/// `{A=0, B=0} → {A=1, B=0}`.
pub struct Path<'a>(pub &'a [Minterm<Symbol>]);

impl fmt::Display for Path<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Joined::new(self.0.iter(), " → ", State).fmt(f)
    }
}

/// A list written one item after another, separated by `, `.
pub struct Commas<'a, T: fmt::Display>(pub &'a [T]);

impl<T: fmt::Display> fmt::Display for Commas<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Joined::new(self.0.iter(), ", ", std::convert::identity).fmt(f)
    }
}
