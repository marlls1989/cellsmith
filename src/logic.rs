//! The logic core: signal resolution, the state machine, region derivation, and arc derivation.

pub mod analysis;
pub mod arcs;
pub mod confluence;
pub mod edge;
pub mod hazard;
pub mod leakage;
pub mod machine;
pub mod minimise;
pub mod regions;
pub mod resolve;
pub mod width;

use std::collections::BTreeMap;

use espresso_logic::{Minterm, Symbol};

/// The fixed (non-don't-care) assignments of a minterm as a `name -> value` map. Used by the arcs
/// emitter to read an arc's input vectors.
pub fn assignment(m: &Minterm<Symbol>) -> BTreeMap<Symbol, bool> {
    m.vars()
        .iter()
        .zip(m.iter())
        .filter_map(|(var, val)| val.map(|b| (var.clone(), b)))
        .collect()
}

/// A minterm's fixed values as a product of literals: `A*B`, `!R*S` (in the minterm's variable order).
/// No fixed value ⇒ the tautology `1`.
pub(crate) fn literals_str(m: &Minterm<Symbol>) -> String {
    let pairs: Vec<(Symbol, bool)> = m
        .vars()
        .iter()
        .zip(m.iter())
        .filter_map(|(n, v)| v.map(|b| (n.clone(), b)))
        .collect();
    if pairs.is_empty() {
        "1".to_owned()
    } else {
        literal_product(&pairs)
    }
}

/// A minterm's fixed values as `name=1`/`name=0` strings (minterm variable order), skipping any name
/// in `skip`. Used to render competing states and hazard conditions.
pub(crate) fn fixed_pairs(m: &Minterm<Symbol>, skip: &[&str]) -> Vec<String> {
    m.vars()
        .iter()
        .zip(m.iter())
        .filter_map(|(n, v)| {
            let name = n.as_str();
            if skip.contains(&name) {
                return None;
            }
            v.map(|b| format!("{name}={}", if b { 1 } else { 0 }))
        })
        .collect()
}

/// Mint a state-node name for `base`: `<base>_st`, escalating to `<base>_st2`, `<base>_st3`, … until
/// `taken` no longer reports it. `taken` answers for every name already in use in the cell — its pins
/// and its previously minted nodes alike — so a spec that legitimately declares a signal called
/// `Q_st` simply pushes the minted node to `Q_st2` rather than colliding with it.
///
/// The single minting convention for both node-minting sites: a state OUTPUT's table node
/// ([`crate::emit::statetable::build_state_model`]) and a register factored out of a read-gated output
/// ([`edge`]).
pub(crate) fn mint_state_node(base: &str, taken: impl Fn(&str) -> bool) -> String {
    let mut name = format!("{base}_st");
    let mut k = 2;
    while taken(&name) {
        name = format!("{base}_st{k}");
        k += 1;
    }
    name
}

/// A product of literals `k`/`!k` joined by `*` (no tautology fallback, no sorting — the caller decides).
pub(crate) fn literal_product(lits: &[(Symbol, bool)]) -> String {
    lits.iter()
        .map(|(k, v)| if *v { k.to_string() } else { format!("!{k}") })
        .collect::<Vec<_>>()
        .join("*")
}

#[cfg(test)]
mod tests {
    use super::*;
    use espresso_logic::bdd_builder;

    #[test]
    fn assignment_reads_fixed_values() {
        // A single full minterm over A,B from a maximal cover.
        let builder = bdd_builder!();
        let f = builder.parse("A*!B").unwrap();
        let cover = f.maximize();
        let m = cover.cubes().next().unwrap().inputs().clone();
        let a = assignment(&m);
        assert_eq!(a.get("A"), Some(&true));
        assert_eq!(a.get("B"), Some(&false));
    }
}
