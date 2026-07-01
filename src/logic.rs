//! The logic core: signal resolution, the state machine, region derivation, and arc derivation.

pub mod arcs;
pub mod interlock;
pub mod machine;
pub mod regions;
pub mod resolve;

use std::collections::BTreeMap;

use espresso_logic::{Minterm, Symbol};

/// The fixed (non-don't-care) assignments of a minterm as a `name -> value` map. Used by the arcs
/// emitter to read an arc's input vectors.
pub fn assignment(m: &Minterm<Symbol>) -> BTreeMap<String, bool> {
    m.vars()
        .iter()
        .zip(m.iter())
        .filter_map(|(var, val)| val.map(|b| (var.as_str().to_string(), b)))
        .collect()
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
        let cover = f.maximize(&["A", "B"]);
        let m = cover.cubes().next().unwrap().inputs().clone();
        let a = assignment(&m);
        assert_eq!(a.get("A"), Some(&true));
        assert_eq!(a.get("B"), Some(&false));
    }
}
