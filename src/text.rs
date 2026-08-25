//! Writing a sequence of values one after another with something between them, which is what every
//! list this crate writes comes down to — in any of its formats and on either stream.
//!
//! The same space-separated body is a Tcl `-pinlist` and a Liberty statetable header, and the same
//! separator loop writes a warning's comma list on standard error, so these two adapters belong to no
//! format and sit outside the emitters that use them. Each is a typed value carrying a
//! [`fmt::Display`], so the characters are produced once, into the writer the value is being written
//! to. Whatever a format wraps the list in — a Tcl brace group, an attribute's quotes — is the
//! caller's, and stays with the emitter that speaks that format.

use std::fmt;

/// A sequence of items written one after another with `sep` between consecutive ones and nothing
/// else — the one separator loop the crate's rendering vocabulary needs, shared by every fixed list
/// this crate writes (a Tcl word list, a diagnostic's comma list, a state-table row's node columns).
/// `project` reads the value actually displayed out of what `source` yields: the identity where the
/// source already iterates the displayed values themselves, and a genuine projection where it iterates
/// something a displayed value has to be read out of or picked from — a struct field, a header-aligned
/// pair, an either-or of two renderings.
pub(crate) struct Joined<S, F> {
    source: S,
    sep: &'static str,
    project: F,
}

impl<S, F> Joined<S, F> {
    pub(crate) fn new(source: S, sep: &'static str, project: F) -> Self {
        Joined {
            source,
            sep,
            project,
        }
    }
}

impl<S, F, Item, T> fmt::Display for Joined<S, F>
where
    S: Iterator<Item = Item> + Clone,
    F: Fn(Item) -> T,
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, item) in self.source.clone().enumerate() {
            if i > 0 {
                f.write_str(self.sep)?;
            }
            write!(f, "{}", (self.project)(item))?;
        }
        Ok(())
    }
}

/// Several values as one whitespace-separated list body: each written in turn, separated by a single
/// space. That is a Tcl list body — a `-pinlist`, `-vector` or `-probe` — and equally a Liberty
/// statetable's column header. The separator is all this adds: the braces a Tcl argument puts around
/// the list, and the quotes a Liberty attribute puts around its value, are the caller's.
pub(crate) struct Words<'a, T: fmt::Display>(pub(crate) &'a [T]);

impl<T: fmt::Display> fmt::Display for Words<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Joined::new(self.0.iter(), " ", std::convert::identity).fmt(f)
    }
}
