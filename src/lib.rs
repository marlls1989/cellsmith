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
    //!   * minterm expansion that *widens* to variables absent from the function.

    use espresso_logic::{bdd_builder, expr};

    #[test]
    fn projects_feedback_and_widens_minterms() {
        // C-element next-state: q is the delayed feedback of the output.
        //   next_q = a*b + q*(a+b)
        let next_q = expr!(("a" & "b") | ("q" & ("a" | "b")));
        let not_next_q = expr!(!(("a" & "b") | ("q" & ("a" | "b"))));

        let builder = bdd_builder!();
        let f = builder.build(&next_q);
        let nf = builder.build(&not_next_q);

        // Cover extraction works => FFI + BDD are linked.
        assert!(f.to_cubes().num_cubes() >= 1);

        // Project the feedback variable q out.
        //   on  = ∀q. f   == a*b
        //   off = ∀q. !f  == !a*!b
        let on = f.forall(&["q"]);
        let off = nf.forall(&["q"]);
        assert!(
            on.equivalent_to(&builder.build(&expr!("a" & "b"))),
            "on-set of a C-element must be a*b"
        );
        assert!(
            off.equivalent_to(&builder.build(&expr!(!"a" & !"b"))),
            "off-set of a C-element must be !a*!b"
        );

        // Maximum-cover / minterm expansion must widen to variables absent from the function:
        // adding an unused variable `z` doubles the minterm count (z split both polarities).
        let m_abq = f.to_minterms(&["a", "b", "q"]);
        let m_abqz = f.to_minterms(&["a", "b", "q", "z"]);
        assert_eq!(
            m_abqz.len(),
            m_abq.len() * 2,
            "absent variable in `vars` must be split into both polarities"
        );
    }
}
