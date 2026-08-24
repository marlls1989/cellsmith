//! The logic core: signal resolution, the state machine, region derivation, and arc derivation.

pub mod analysis;
pub mod arcs;
pub mod confluence;
pub(crate) mod constraint;
pub(crate) mod edge;
pub mod hazard;
pub mod leakage;
pub mod machine;
pub mod minimise;
pub mod regions;
pub(crate) mod resolve;
pub mod width;

use espresso_logic::{BoolExpr, Minterm, Symbol};

/// A minterm's fixed values as a product of literals, in the minterm's variable order: `A & B`,
/// `!R & S`. No fixed value ⇒ the tautology `1`.
pub fn condition(m: &Minterm<Symbol>) -> BoolExpr {
    let lits: Vec<Literal> = m
        .vars()
        .iter()
        .zip(m.iter())
        .filter_map(|(n, v)| {
            v.map(|positive| Literal {
                var: n.clone(),
                positive,
            })
        })
        .collect();
    product(&lits)
}

/// Mint a state-node name for `base`: `<base>_st`, escalating to `<base>_st2`, `<base>_st3`, … until
/// `taken` no longer reports it. `taken` answers for every name already in use in the cell — its pins
/// and its previously minted nodes alike — so a spec that legitimately declares a signal called
/// `Q_st` pushes the minted node to `Q_st2` rather than colliding with it.
///
/// The single minting convention for both node-minting sites: a state OUTPUT's table node
/// ([`crate::emit::statetable::build_state_model`]) and a register factored out of a read-gated output
/// ([`edge`]).
pub(crate) fn mint_state_node(base: &str, taken: impl Fn(&Symbol) -> bool) -> Symbol {
    let mut name = Symbol::from(format!("{base}_st"));
    let mut k = 2;
    while taken(&name) {
        name = Symbol::from(format!("{base}_st{k}"));
        k += 1;
    }
    name
}

/// One literal of a product: a variable and whether it appears positive (`k`) or negated (`!k`).
/// The derived order exists so a caller may sort a slice of literals: two conditions naming the same literal set then sort to the same sequence and compare structurally equal.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Literal {
    pub(crate) var: Symbol,
    pub(crate) positive: bool,
}

/// A product of literals `k`/`!k` — `A & !B & C` — as one expression. No literal ⇒ the tautology `1`.
/// Neither sorted nor deduplicated: what the product ranges over, and in which order, is the caller's.
///
/// The literals are folded LEFT, and the rendering rests on that: `BoolExpr`'s `Display` writes minimal
/// parentheses over the syntactic tree, so a left fold reads `A & B & C` where a right fold would read
/// `A & (B & C)`. The fold runs inside [`BoolExpr::build`], espresso-logic's constructor for an
/// expression assembled from data — it serialises the whole product in one pass, where composing with
/// `&` reallocates the token stream at every operator.
pub(crate) fn product(lits: &[Literal]) -> BoolExpr {
    BoolExpr::build(|b| {
        lits.iter()
            .map(|lit| {
                let expr = b.var(&lit.var);
                if lit.positive {
                    expr
                } else {
                    !expr
                }
            })
            .reduce(|acc, lit| acc & lit)
            .unwrap_or_else(|| b.constant(true))
    })
}
