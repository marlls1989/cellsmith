//! The vocabulary a Liberate command is written in: what one `-vector` or `-ic` column holds, and the
//! wrappers Tcl's own syntax puts around a list of them.
//!
//! Each is a typed value carrying a [`fmt::Display`], so a block is assembled from the values it states
//! and the characters are produced once, into the writer it is being written to, where the value is
//! displayed. [`Indented`] is the same idea for a whole artifact: a [`fmt::Write`] that nests everything
//! passing through it under a line prefix.

use std::collections::HashSet;
use std::fmt;

use crate::logic::arcs::Edge;

/// One column of a Liberate `-vector`: what the measurement does to that column's node over the block's
/// own window. It either moves, or is held at a level, or is left for the cell itself to drive. These
/// five are the whole alphabet a column is written in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum VectorValue {
    /// Driven up across the measurement: `R`.
    Rise,
    /// Driven down across the measurement: `F`.
    Fall,
    /// Held high throughout: `1`.
    High,
    /// Held low throughout: `0`.
    Low,
    /// Stated as nothing: `X`. A column forces the node it addresses, so a node the cell drives is left
    /// to follow the cell rather than be overridden mid-measurement.
    Unstated,
}

impl fmt::Display for VectorValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            VectorValue::Rise => "R",
            VectorValue::Fall => "F",
            VectorValue::High => "1",
            VectorValue::Low => "0",
            VectorValue::Unstated => "X",
        })
    }
}

/// The column a measured edge writes: the direction it moves the node in.
impl From<Edge> for VectorValue {
    fn from(edge: Edge) -> Self {
        match edge {
            Edge::Rise => VectorValue::Rise,
            Edge::Fall => VectorValue::Fall,
        }
    }
}

/// The column a held node writes: the level it stays at for the whole measurement.
impl From<bool> for VectorValue {
    fn from(level: bool) -> Self {
        match level {
            true => VectorValue::High,
            false => VectorValue::Low,
        }
    }
}

/// Several values as one Tcl list body: each written in turn, separated by a single space. The
/// separator is all this adds — the braces a `-pinlist`, `-vector` or `-probe` puts around the list are
/// [`Braced`]'s, and the `-ic` word's quotes are the line's own.
pub(crate) struct Words<'a, T: fmt::Display>(pub(crate) &'a [T]);

impl<T: fmt::Display> fmt::Display for Words<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, word) in self.0.iter().enumerate() {
            if i > 0 {
                f.write_str(" ")?;
            }
            write!(f, "{word}")?;
        }
        Ok(())
    }
}

/// One value inside a Tcl brace group, which is what makes it a single argument however much
/// whitespace it holds.
pub(crate) struct Braced<T: fmt::Display>(pub(crate) T);

impl<T: fmt::Display> fmt::Display for Braced<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{{}}}", self.0)
    }
}

/// A [`fmt::Write`] that nests what is written through it one level deeper: every line reaches `writer`
/// behind `prefix`.
///
/// The prefix falls due at a line's first character and nowhere else, which is what lets a line arrive
/// across several writes and still be prefixed once. An EMPTY line — a newline with no content before it
/// — is passed through bare, so nesting leaves no line holding nothing but trailing whitespace.
pub(crate) struct Indented<'w, W: fmt::Write> {
    writer: &'w mut W,
    prefix: &'w str,
    /// Whether the next character written begins a line, and so has the prefix due before it.
    at_line_start: bool,
}

impl<'w, W: fmt::Write> Indented<'w, W> {
    /// Nest everything written here under `prefix`, writing it on to `writer`.
    pub(crate) fn new(writer: &'w mut W, prefix: &'w str) -> Self {
        Indented {
            writer,
            prefix,
            at_line_start: true,
        }
    }
}

impl<W: fmt::Write> fmt::Write for Indented<'_, W> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        // `split_inclusive` keeps each terminator with the line it ends, so a chunk that is exactly a
        // newline is an empty line and a chunk without one leaves its line still open.
        for line in s.split_inclusive('\n') {
            if self.at_line_start && line != "\n" {
                self.writer.write_str(self.prefix)?;
            }
            self.writer.write_str(line)?;
            self.at_line_start = line.ends_with('\n');
        }
        Ok(())
    }
}

/// One `-ic` column as the spec wrote it: the `logic_low`/`logic_high` expression for the level that
/// column starts at. It is a Tcl VALUE fragment rather than a name, so it is carried as a `String`, and
/// carried rather than borrowed because the block holding it is an owned value the emitter hashes.
/// Displaying it is [`ic_column`] — the wrap and the escaping that keep an arbitrary expression to
/// exactly one column of the line.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct IcColumn(pub(crate) String);

impl fmt::Display for IcColumn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&ic_column(&self.0))
    }
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
pub(crate) fn ic_column(value: &str) -> String {
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

#[cfg(test)]
pub(crate) mod tests {
    use std::fmt::Write;

    use super::*;

    /// Write `chunks` through an [`Indented`] under `prefix`, one write per chunk — which is how a line
    /// comes to arrive in pieces.
    fn indented(prefix: &str, chunks: &[&str]) -> String {
        let mut out = String::new();
        let mut nested = Indented::new(&mut out, prefix);
        for chunk in chunks {
            nested
                .write_str(chunk)
                .expect("writing into a String cannot fail");
        }
        out
    }

    #[test]
    fn every_line_reaches_the_writer_behind_the_prefix() {
        assert_eq!(indented("  ", &["a\nb\n"]), "  a\n  b\n");
        // The prefix falls due at the next line's first character, so a text left mid-line ends there
        // and one ending on its terminator leaves no prefix dangling after it.
        assert_eq!(indented("  ", &["a\nb"]), "  a\n  b");
    }

    #[test]
    fn a_line_arriving_in_several_writes_is_prefixed_once() {
        assert_eq!(
            indented("\t", &["ce", "ll", " (C2) {\n"]),
            "\tcell (C2) {\n"
        );
        assert_eq!(indented("\t", &["a\n", "b\n"]), "\ta\n\tb\n");
    }

    #[test]
    fn an_empty_line_is_passed_through_bare() {
        // A line with nothing on it takes no prefix: an indent there would be trailing whitespace and
        // nothing else.
        assert_eq!(indented("  ", &["a\n\nb\n"]), "  a\n\n  b\n");
        assert_eq!(indented("  ", &["\n"]), "\n");
        assert_eq!(indented("  ", &[]), "");
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
    /// The expressions that need more than the wrap to hold their column, each with the text the
    /// emitter writes between the `-ic` quotes. Every one of them is read back through real Tcl by
    /// [`tclsh_reads_an_awkward_logic_voltage_as_one_column_per_pin`], which is where the doubling was
    /// established; the pairs here pin it without an interpreter to hand.
    pub(crate) const AWKWARD_VOLTAGES: [(&str, &str); 17] = [
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
}
