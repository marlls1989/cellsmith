//! lobsterate — generate Cadence Liberate transition arcs (with prevectors) for logic cells,
//! including state-holding/hysteretic cells (C-elements, latches, cross-coupled pairs).
//!
//! Modules are filled in per the implementation plan: input model + parser, region derivation
//! (feedback projection), the prevector walk, and the arcs / Verilog / Liberty emitters.

pub mod emit;
pub mod expr;
pub mod logic;
pub mod model;

#[cfg(test)]
mod smoke {
    //! Confirms the espresso-logic 5.x public API and its C-FFI build link, and that the two
    //! primitives lobsterate leans on behave as the plan assumes:
    //!   * feedback projection via universal quantification (`forall`), and
    //!   * universal projection to a two-sided FR cover (`cover_over_fr`), whose F/R cubes are the
    //!     on/off sets and whose absent cubes are the undef/hold gap.

    use espresso_logic::{bdd_builder, expr, CubeType};

    #[test]
    fn projects_feedback_and_extracts_fr_sides() {
        // C-element next-state: q is the delayed feedback of the output.
        //   next_q = a*b + q*(a+b)
        let next_q = expr!(("a" & "b") | ("q" & ("a" | "b")));

        let builder = bdd_builder!();
        let f = builder.build(&next_q);

        // Cover extraction works => FFI + BDD are linked.
        assert!(f.cover().num_cubes() >= 1);

        // Project the feedback variable q out (complement the BDD directly rather than rebuilding a
        // negated expression).
        //   on  = ∀q. f   == a*b
        //   off = ∀q. !f  == !a*!b
        let on = f.forall(["q"]);
        let off = (!f.clone()).forall(["q"]);
        assert!(
            on.equivalent_to(&builder.build(&expr!("a" & "b"))),
            "on-set of a C-element must be a*b"
        );
        assert!(
            off.equivalent_to(&builder.build(&expr!(!"a" & !"b"))),
            "off-set of a C-element must be !a*!b"
        );

        // Universal projection onto the inputs as a two-sided FR cover: the on-set is a=b=1
        // (q⁺ forced high regardless of the held q), the off-set is a=b=0, and a≠b lands in
        // NEITHER side — the C-element hold gap is the absence of a cube, not a `D` cube.
        let fr = f.cover_over_fr(&["a", "b"]).maximize();
        let side = |t: CubeType| -> Vec<(Option<bool>, Option<bool>)> {
            fr.cubes()
                .filter(|c| c.cube_type() == t)
                .map(|c| (c.inputs().value_of("a"), c.inputs().value_of("b")))
                .collect()
        };
        assert_eq!(side(CubeType::F), vec![(Some(true), Some(true))], "on-set must be a=b=1");
        assert_eq!(side(CubeType::R), vec![(Some(false), Some(false))], "off-set must be a=b=0");
        assert_eq!(fr.num_cubes(), 2, "a≠b must land in neither set (the undef/hold gap)");
    }
}
