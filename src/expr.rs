//! Parse a Boolean function's surface syntax into an espresso-logic [`BoolExpr`].
//!
//! Delegates to [`BoolExpr::parse`], whose grammar is a superset of the `a*b+!c` form hsNCL uses:
//! `*`/`&` AND, `+`/`|` OR, `!`/`~` NOT, `^` XOR, `0`/`1`/`true`/`false` constants, and parentheses
//! for grouping. Precedence, tightest first: NOT > AND > XOR > OR. Identifiers are a letter/`_`
//! followed by letters/digits/`_` (so pin names like `M1`, `P2`, `Q` are fine).

use std::collections::BTreeSet;

use espresso_logic::expression::ParseBoolExprError;
use espresso_logic::BoolExpr;

/// A successfully parsed function: the espresso [`BoolExpr`] plus the set of variable names it
/// references (used by the model to classify primary inputs vs feedback/state variables).
#[derive(Debug)]
pub struct Parsed {
    pub expr: BoolExpr,
    pub vars: BTreeSet<String>,
}

/// Parse a Boolean function into a [`BoolExpr`] and the set of variables it syntactically references.
pub fn parse(input: &str) -> Result<Parsed, ParseBoolExprError> {
    let expr = BoolExpr::parse(input)?;
    let vars = expr.variables().map(|s| s.to_string()).collect();
    Ok(Parsed { expr, vars })
}

#[cfg(test)]
mod tests {
    use super::*;
    use espresso_logic::{bdd_builder, expr};

    fn vars(p: &Parsed) -> Vec<String> {
        p.vars.iter().cloned().collect()
    }

    #[test]
    fn parses_c_element_and_collects_vars() {
        let p = parse("A*B + Q*(A+B)").unwrap();
        assert_eq!(vars(&p), ["A", "B", "Q"]);

        let builder = bdd_builder!();
        let got = builder.build(&p.expr);
        let want = builder.build(&expr!(("A" & "B") | ("Q" & ("A" | "B"))));
        assert!(got.equivalent_to(&want));
    }

    #[test]
    fn precedence_not_over_and_over_or() {
        // a + b*c  ==  a | (b & c)
        let builder = bdd_builder!();
        let got = builder.build(&parse("a + b*c").unwrap().expr);
        let want = builder.build(&expr!("a" | ("b" & "c")));
        assert!(got.equivalent_to(&want));

        // !a*b  ==  (!a) & b
        let got = builder.build(&parse("!a*b").unwrap().expr);
        let want = builder.build(&expr!(!"a" & "b"));
        assert!(got.equivalent_to(&want));
    }

    #[test]
    fn constants_and_pin_names_with_digits() {
        let p = parse("M1*P2 + 1").unwrap();
        assert_eq!(vars(&p), ["M1", "P2"]);
        // x + 1 is a tautology
        let builder = bdd_builder!();
        assert!(builder.build(&p.expr).is_tautology());
    }

    #[test]
    fn accepts_superset_syntax() {
        // espresso's grammar also accepts `&`/`|`/`~`/`^` and `true`/`false`; precedence NOT > AND >
        // XOR > OR, so `a & b | ~c ^ d` == `(a&b) | ((~c)^d)`.
        let builder = bdd_builder!();
        let got = builder.build(&parse("a & b | ~c ^ d").unwrap().expr);
        let want = builder.build(&expr!(("a" & "b") | (!"c" ^ "d")));
        assert!(got.equivalent_to(&want));
        assert!(builder.build(&parse("true").unwrap().expr).is_tautology());
        assert!(builder.build(&parse("false").unwrap().expr).is_contradiction());
    }

    #[test]
    fn rejects_garbage() {
        assert!(parse("").is_err());
        assert!(parse("a +").is_err());
        assert!(parse("a b").is_err());
        assert!(parse("(a").is_err());
        assert!(parse("a @ b").is_err());
    }
}
