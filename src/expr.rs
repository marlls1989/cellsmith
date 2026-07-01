//! Parser for the `a*b+!c` Boolean surface syntax (the one hsNCL uses) into an espresso-logic
//! [`BoolExpr`].
//!
//! Operators: `*` AND, `+` OR, `!` NOT; `0`/`1` constants; parentheses for grouping.
//! Precedence, tightest first: `!`  >  `*`  >  `+`. Identifiers are a letter/`_` followed by
//! letters/digits/`_` (so pin names like `M1`, `P2`, `Q` are fine; a bare `0`/`1` is a constant).

use std::collections::BTreeSet;

use espresso_logic::BoolExpr;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ParseError {
    #[error("empty expression")]
    Empty,
    #[error("unexpected character {0:?} at byte offset {1}")]
    UnexpectedChar(char, usize),
    #[error("unexpected end of expression")]
    UnexpectedEof,
    #[error("expected an operand")]
    ExpectedOperand,
    #[error("unclosed '('")]
    UnclosedParen,
    #[error("unexpected trailing tokens")]
    TrailingTokens,
}

/// A successfully parsed function: the espresso [`BoolExpr`] plus the set of variable names it
/// references (used by the model to classify primary inputs vs feedback/state variables).
#[derive(Debug)]
pub struct Parsed {
    pub expr: BoolExpr,
    pub vars: BTreeSet<String>,
}

/// Parse a Boolean function in the `a*b+!c` surface syntax.
pub fn parse(input: &str) -> Result<Parsed, ParseError> {
    let tokens = lex(input)?;
    if tokens.is_empty() {
        return Err(ParseError::Empty);
    }
    let mut parser = Parser {
        tokens: &tokens,
        pos: 0,
        vars: BTreeSet::new(),
    };
    let expr = parser.parse_or()?;
    if parser.pos != parser.tokens.len() {
        return Err(ParseError::TrailingTokens);
    }
    Ok(Parsed {
        expr,
        vars: parser.vars,
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Tok {
    And,
    Or,
    Not,
    LParen,
    RParen,
    Ident(String),
    Const(bool),
}

fn lex(input: &str) -> Result<Vec<Tok>, ParseError> {
    let mut tokens = Vec::new();
    let mut chars = input.char_indices().peekable();
    while let Some((i, c)) = chars.next() {
        match c {
            c if c.is_whitespace() => {}
            '*' => tokens.push(Tok::And),
            '+' => tokens.push(Tok::Or),
            '!' => tokens.push(Tok::Not),
            '(' => tokens.push(Tok::LParen),
            ')' => tokens.push(Tok::RParen),
            '0' => tokens.push(Tok::Const(false)),
            '1' => tokens.push(Tok::Const(true)),
            c if c.is_alphabetic() || c == '_' => {
                let mut name = String::from(c);
                while let Some(&(_, n)) = chars.peek() {
                    if n.is_alphanumeric() || n == '_' {
                        name.push(n);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Tok::Ident(name));
            }
            _ => return Err(ParseError::UnexpectedChar(c, i)),
        }
    }
    Ok(tokens)
}

struct Parser<'a> {
    tokens: &'a [Tok],
    pos: usize,
    vars: BTreeSet<String>,
}

impl Parser<'_> {
    fn peek(&self) -> Option<&Tok> {
        self.tokens.get(self.pos)
    }

    // or := and ('+' and)*
    fn parse_or(&mut self) -> Result<BoolExpr, ParseError> {
        let mut left = self.parse_and()?;
        while matches!(self.peek(), Some(Tok::Or)) {
            self.pos += 1;
            let right = self.parse_and()?;
            left = left | right;
        }
        Ok(left)
    }

    // and := unary ('*' unary)*
    fn parse_and(&mut self) -> Result<BoolExpr, ParseError> {
        let mut left = self.parse_unary()?;
        while matches!(self.peek(), Some(Tok::And)) {
            self.pos += 1;
            let right = self.parse_unary()?;
            left = left & right;
        }
        Ok(left)
    }

    // unary := '!' unary | atom
    fn parse_unary(&mut self) -> Result<BoolExpr, ParseError> {
        if matches!(self.peek(), Some(Tok::Not)) {
            self.pos += 1;
            let inner = self.parse_unary()?;
            Ok(!inner)
        } else {
            self.parse_atom()
        }
    }

    // atom := Ident | Const | '(' or ')'
    fn parse_atom(&mut self) -> Result<BoolExpr, ParseError> {
        match self.peek().cloned() {
            Some(Tok::Ident(name)) => {
                self.pos += 1;
                let e = BoolExpr::var(name.as_str());
                self.vars.insert(name);
                Ok(e)
            }
            Some(Tok::Const(b)) => {
                self.pos += 1;
                Ok(BoolExpr::constant(b))
            }
            Some(Tok::LParen) => {
                self.pos += 1;
                let inner = self.parse_or()?;
                if !matches!(self.peek(), Some(Tok::RParen)) {
                    return Err(ParseError::UnclosedParen);
                }
                self.pos += 1;
                Ok(inner)
            }
            Some(_) => Err(ParseError::ExpectedOperand),
            None => Err(ParseError::UnexpectedEof),
        }
    }
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
    fn rejects_garbage() {
        assert_eq!(parse("").unwrap_err(), ParseError::Empty);
        assert_eq!(parse("a +").unwrap_err(), ParseError::UnexpectedEof);
        assert_eq!(parse("a b").unwrap_err(), ParseError::TrailingTokens);
        assert_eq!(parse("(a").unwrap_err(), ParseError::UnclosedParen);
        assert!(matches!(
            parse("a @ b").unwrap_err(),
            ParseError::UnexpectedChar('@', _)
        ));
    }
}
