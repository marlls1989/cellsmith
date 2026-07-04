//! The logic core: signal resolution, the state machine, region derivation, and arc derivation.

pub mod analysis;
pub mod arcs;
pub mod confluence;
pub mod interlock;
pub mod machine;
pub mod regions;
pub mod resolve;

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
