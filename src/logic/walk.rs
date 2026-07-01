//! The prevector walk — the crux of the tool.
//!
//! Ported from hsNCL `Data/Logic/Cover/Minimization.hs` (`coverDistances`,
//! `coverTransitionsPath`, `coverTransitionsPath'`). Given a `from` cover, a `through` cover (the
//! hold region), and a `to` cover, it builds a sequence of input minterms that steps **one variable
//! at a time** (Hamming distance 1) from `to` back to `from`, routed through `through`. That
//! sequence is the prevector: the preconditioning walk that drives a state-holding cell into the
//! start state of a measured edge without glitching the hold loop.
//!
//! All covers here are sets of fully-assigned minterms over the **same** input header (produced by
//! [`super::regions`]), so hsNCL's `expandCover`/`mergeCovers` reduce to plain set union and its
//! `Map.differenceWith` reduces to a per-position compare.

use std::collections::BTreeMap;

use espresso_logic::{Minterm, Symbol};
use thiserror::Error;

use super::regions::MintermSet;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum WalkError {
    #[error("no single-step path between covers found")]
    NoPath,
    #[error("a cover endpoint was empty")]
    EmptyCover,
}

/// A difference map: on the positions where two minterms disagree, the value taken from the
/// *source* minterm. Its length is the Hamming distance. (hsNCL's `dvars`.)
type Diff = BTreeMap<Symbol, bool>;

/// The fixed (non-don't-care) assignments of a minterm as a `var -> value` map.
fn value_map(m: &Minterm<Symbol>) -> BTreeMap<Symbol, bool> {
    m.vars()
        .iter()
        .cloned()
        .zip(m.iter())
        .filter_map(|(var, val)| val.map(|b| (var, b)))
        .collect()
}

/// Disagreement of `src` against `dst`, carrying `src`'s values (hsNCL `dvars s d`).
fn diff(src: &Minterm<Symbol>, dst: &Minterm<Symbol>) -> Diff {
    let dst_vals = value_map(dst);
    value_map(src)
        .into_iter()
        .filter(|(var, sv)| dst_vals.get(var) != Some(sv))
        .collect()
}

fn is_submap(a: &Diff, b: &Diff) -> bool {
    a.iter().all(|(k, v)| b.get(k) == Some(v))
}

fn is_proper_submap(a: &Diff, b: &Diff) -> bool {
    a.len() < b.len() && is_submap(a, b)
}

/// The fixed (non-don't-care) assignments of a full minterm as a `name -> value` map.
pub fn assignment(m: &Minterm<Symbol>) -> BTreeMap<String, bool> {
    m.vars()
        .iter()
        .zip(m.iter())
        .filter_map(|(var, val)| val.map(|b| (var.as_str().to_string(), b)))
        .collect()
}

/// A single-variable (Hamming distance 1) transition between two covers: `src` and `dst` differ in
/// exactly the variable `var`.
#[derive(Debug, Clone)]
pub struct Transition {
    pub var: String,
    pub src: Minterm<Symbol>,
    pub dst: Minterm<Symbol>,
}

/// All distance-1 transitions from the `src` cover to the `dst` cover. (hsNCL's
/// `filter (size == 1) . coverDistances`.)
pub fn single_var_transitions(src: &MintermSet, dst: &MintermSet) -> Vec<Transition> {
    cover_distances(src, dst)
        .into_iter()
        .filter(|(d, _, _)| d.len() == 1)
        .map(|(d, s, t)| Transition {
            var: d.keys().next().expect("len == 1").as_str().to_string(),
            src: s,
            dst: t,
        })
        .collect()
}

/// Every `(diff, src, dst)` pair across the two covers, sorted by ascending Hamming distance — so
/// the head is a nearest-neighbour pair. (hsNCL `coverDistances`.)
fn cover_distances(
    src: &MintermSet,
    dst: &MintermSet,
) -> Vec<(Diff, Minterm<Symbol>, Minterm<Symbol>)> {
    let mut pairs: Vec<_> = src
        .iter()
        .flat_map(|s| dst.iter().map(move |d| (diff(s, d), s.clone(), d.clone())))
        .collect();
    pairs.sort_by_key(|(d, _, _)| d.len());
    pairs
}

/// Build a single-step path from `to` back to `from`, routed through `through`.
///
/// Returns the minterm sequence with `to`'s endpoint first; consecutive minterms differ in exactly
/// one variable. (hsNCL `coverTransitionsPath`.)
pub fn transitions_path(
    from: &MintermSet,
    through: &MintermSet,
    to: &MintermSet,
) -> Result<Vec<Minterm<Symbol>>, WalkError> {
    if from.is_empty() || to.is_empty() {
        return Err(WalkError::EmptyCover);
    }

    // Closest (from', to') endpoints.
    let (dist, from1, to1) = cover_distances(from, to)
        .into_iter()
        .next()
        .ok_or(WalkError::NoPath)?;

    // Trim `through` to the corridor between from1 and to1: take (through ∪ {from1}) \ {to1}, keep
    // the minterms whose disagreement with to1 stays within `dist`.
    let mut corridor: MintermSet = through.clone();
    corridor.insert(from1.clone());
    corridor.remove(&to1);
    let to1_set: MintermSet = std::iter::once(to1.clone()).collect();
    let trimmed: MintermSet = cover_distances(&corridor, &to1_set)
        .into_iter()
        .filter(|(d, _, _)| is_submap(d, &dist))
        .map(|(_, p, _)| p)
        .collect();

    walk(&from1, trimmed, vec![to1])
}

/// Greedy nearest-neighbour worker. `acc` holds the path so far (the head is the frontier); each
/// step prepends the path minterm nearest the frontier and shrinks `path` to those strictly closer
/// to `from1`. (hsNCL `coverTransitionsPath'`.)
fn walk(
    from1: &Minterm<Symbol>,
    mut path: MintermSet,
    mut acc: Vec<Minterm<Symbol>>,
) -> Result<Vec<Minterm<Symbol>>, WalkError> {
    loop {
        if acc.iter().any(|m| m == from1) {
            return Ok(acc);
        }
        if path.is_empty() {
            return Err(WalkError::NoPath);
        }

        let acc_set: MintermSet = acc.iter().cloned().collect();
        let (_, new, _) = cover_distances(&path, &acc_set)
            .into_iter()
            .next()
            .ok_or(WalkError::NoPath)?;

        let dist = diff(from1, &new);
        let from1_set: MintermSet = std::iter::once(from1.clone()).collect();
        path = cover_distances(&from1_set, &path)
            .into_iter()
            .filter(|(d, _, _)| is_proper_submap(d, &dist))
            .map(|(_, _, p)| p)
            .collect();

        acc.insert(0, new);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::logic::regions::regions;
    use crate::model::{parse_spec, AnalysedCell};

    fn analyse(src: &str) -> AnalysedCell {
        parse_spec(src).unwrap().cells.remove(0).analyse().unwrap()
    }

    fn hamming(a: &Minterm<Symbol>, b: &Minterm<Symbol>) -> usize {
        a.hamming_distance(b)
    }

    /// Every adjacent pair in a walk must differ in exactly one variable.
    fn assert_single_step(path: &[Minterm<Symbol>]) {
        for pair in path.windows(2) {
            assert_eq!(hamming(&pair[0], &pair[1]), 1, "non-adjacent step in path");
        }
    }

    #[test]
    fn c_element_off_through_hold_to_on() {
        let cell = analyse(
            r#"
[[cell]]
name = "C2"
inputs = ["A", "B"]
[cell.outputs]
Q = "A*B + Q*(A+B)"
"#,
        );
        let r = regions(&cell.outputs[0], &cell.inputs);
        // Rise preconditioning: from off (00), through hold (01/10), to on (11).
        let path = transitions_path(&r.off, &r.hold, &r.on).unwrap();

        assert!(path.len() >= 2);
        assert_single_step(&path);

        // Endpoints: one end is in `on`, the other in `off`; interior in hold.
        let first = &path[0];
        let last = &path[path.len() - 1];
        let ends_on_off = (r.on.contains(first) && r.off.contains(last))
            || (r.off.contains(first) && r.on.contains(last));
        assert!(ends_on_off, "walk must connect on and off endpoints");
        for mid in &path[1..path.len() - 1] {
            assert!(
                r.hold.contains(mid),
                "interior steps must lie in the hold region"
            );
        }
    }

    #[test]
    fn empty_corridor_returns_endpoints_directly() {
        // With no `through` region the walk degenerates to just the two endpoints — faithful to
        // hsNCL (`walk` with `path = {from1}` terminates immediately). Adjacency is therefore only
        // guaranteed when a corridor is supplied; real arc generation always passes the hold region.
        let cell = analyse(
            r#"
[[cell]]
name = "C2"
inputs = ["A", "B"]
[cell.outputs]
Q = "A*B + Q*(A+B)"
"#,
        );
        let r = regions(&cell.outputs[0], &cell.inputs);
        let empty = MintermSet::new();
        let path = transitions_path(&r.off, &empty, &r.on).unwrap();
        assert_eq!(path.len(), 2);
        let set: MintermSet = path.iter().cloned().collect();
        assert_eq!(set, r.off.union(&r.on).cloned().collect());
    }

    #[test]
    fn empty_endpoint_is_an_error() {
        let cell = analyse(
            r#"
[[cell]]
name = "C2"
inputs = ["A", "B"]
[cell.outputs]
Q = "A*B + Q*(A+B)"
"#,
        );
        let r = regions(&cell.outputs[0], &cell.inputs);
        let empty = MintermSet::new();
        assert_eq!(
            transitions_path(&empty, &r.hold, &r.on).unwrap_err(),
            WalkError::EmptyCover
        );
    }

    #[test]
    fn adjacent_endpoints_need_no_intermediate() {
        // A 3-input C-element: off(000) to a hold state at distance 1 is a direct step.
        let cell = analyse(
            r#"
[[cell]]
name = "C3"
inputs = ["A", "B", "C"]
[cell.outputs]
Q = "A*B*C + Q*(A+B+C)"
"#,
        );
        let r = regions(&cell.outputs[0], &cell.inputs);
        // on = ABC=111 ; off = 000 ; hold = everything else (6 states)
        assert_eq!(r.on.len(), 1);
        assert_eq!(r.off.len(), 1);
        assert_eq!(r.hold.len(), 6);
        let path = transitions_path(&r.off, &r.hold, &r.on).unwrap();
        assert_single_step(&path);
        // 000 -> ... -> 111 over single-bit steps needs at least 4 nodes.
        assert!(path.len() >= 4);
    }
}
