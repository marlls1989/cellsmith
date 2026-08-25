//! cellsmith — generate Cadence Liberate transition arcs for logic cells,
//! including state-holding/hysteretic cells (C-elements, latches, cross-coupled pairs).
//!
//! **cellsmith is a command-line tool.** This library target exists only as an internal build
//! artifact shared by the `cellsmith` binary and its benchmarks. It is not a supported public API,
//! carries no stability guarantee across any version, and using it as a library is at your own risk.
//!
//! Modules: the input model ([`model`]), the logic core ([`logic`]: signal resolution, the state
//! machine, arc and hazard derivation, and state-table regions), the arcs / Verilog / Liberty
//! emitters ([`emit`]), the diagnostics' rendering vocabulary ([`report`]), and the separators both
//! of those render their lists with (`text`).
#![doc(hidden)]

pub mod emit;
pub mod logic;
pub mod model;
pub mod report;
pub(crate) mod text;

#[cfg(test)]
mod smoke {
    //! Confirms the espresso-logic 5.x public API and its C-FFI build link, and that the two
    //! primitives cellsmith leans on behave as the plan assumes:
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

        // One cube's assignment to the C-element's two inputs, `a` and `b`.
        #[derive(Debug, PartialEq)]
        struct AbValues {
            a: Option<bool>,
            b: Option<bool>,
        }

        // Universal projection onto the inputs as a two-sided FR cover: the on-set is a=b=1
        // (q⁺ forced high regardless of the held q), the off-set is a=b=0, and a≠b lands in
        // NEITHER side — the C-element hold gap is the absence of a cube, not a `D` cube.
        let fr = f.cover_over_fr(["a", "b"]).maximize();
        let side = |t: CubeType| -> Vec<AbValues> {
            fr.cubes()
                .filter(|c| c.cube_type() == t)
                .map(|c| AbValues {
                    a: c.inputs().value_of("a"),
                    b: c.inputs().value_of("b"),
                })
                .collect()
        };
        assert_eq!(
            side(CubeType::F),
            vec![AbValues {
                a: Some(true),
                b: Some(true)
            }],
            "on-set must be a=b=1"
        );
        assert_eq!(
            side(CubeType::R),
            vec![AbValues {
                a: Some(false),
                b: Some(false)
            }],
            "off-set must be a=b=0"
        );
        assert_eq!(
            fr.num_cubes(),
            2,
            "a≠b must land in neither set (the undef/hold gap)"
        );
    }
}
