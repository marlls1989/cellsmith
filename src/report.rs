//! The rendering vocabulary of the diagnostics the run writes to standard error, and the layout of a
//! hazard warning.
//!
//! A warning's subjects are the values the analysis already holds — a state is a
//! [`Minterm`](espresso_logic::Minterm) over the cell's signals, a path a sequence of them — and each
//! adapter here borrows one and writes it into the warning's own writer. Nothing is rendered ahead of
//! the write, so a subject travels as itself and becomes text once, where the warning is written.
//!
//! Which warnings a run prints is the caller's to decide; [`hazard_warning`] writes one of them, a
//! header over a [`subblock`] of labelled fields.

use std::collections::HashMap;
use std::fmt;
use std::io;

use espresso_logic::{Minterm, Symbol};

use crate::logic::arcs::PinEdge;
use crate::logic::hazard::{Cause, Hazard, Outcome};
use crate::model::AnalysedCell;
use crate::text::Joined;

/// One state as the values it fixes, in the minterm's variable order: `{A=1, B=0}`. A column the
/// minterm leaves free is no part of the state and is left out.
pub struct State<'a>(pub &'a Minterm<Symbol>);

impl fmt::Display for State<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("{")?;
        // Collected because `Joined::fmt` takes `&self` and clones its stored iterator to walk
        // it, and `Minterm::iter`'s `MintermIter` carries no `Clone`.
        let fixed: Vec<_> = self
            .0
            .vars()
            .iter()
            .zip(self.0.iter())
            .filter_map(|(name, value)| value.map(|v| (name, v)))
            .collect();
        Joined::new(fixed.iter(), ", ", |&(name, value)| Assignment {
            name,
            value,
        })
        .fmt(f)?;
        f.write_str("}")
    }
}

/// One `name=value` pair inside a [`State`]: the variable and the value the minterm fixes it to.
struct Assignment<'a> {
    name: &'a Symbol,
    value: bool,
}

impl fmt::Display for Assignment<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}={}", self.name, u8::from(self.value))
    }
}

/// A walk through the machine as the states it passes through, in order and joined by ` → `:
/// `{A=0, B=0} → {A=1, B=0}`.
pub struct Path<'a>(pub &'a [Minterm<Symbol>]);

impl fmt::Display for Path<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Joined::new(self.0.iter(), " → ", State).fmt(f)
    }
}

/// A list written one item after another, separated by `, `.
pub struct Commas<'a, T: fmt::Display>(pub &'a [T]);

impl<T: fmt::Display> fmt::Display for Commas<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Joined::new(self.0.iter(), ", ", std::convert::identity).fmt(f)
    }
}

/// One field of a warning's subblock: the colon-labelled name written to stderr and the value rendered
/// beside it.
pub struct SubblockField<'a> {
    pub label: &'a str,
    pub value: &'a dyn fmt::Display,
}

/// Write one warning detail block: colon-labelled fields, indented under the header with their values
/// column-aligned. `lead` opens the first line — a hazard warning states one block and opens it at the
/// same indent as the rest, while the masked-arc warning states a block per conflated arc and bullets
/// each so the blocks read apart.
pub fn subblock(w: &mut impl io::Write, lead: &str, fields: &[SubblockField]) -> io::Result<()> {
    for (i, SubblockField { label, value }) in fields.iter().enumerate() {
        let marker = if i == 0 { lead } else { "    " };
        // The colon belongs to the label, so it is what the 16-column field is padded around: the label
        // and its colon go out first, then the padding that would have followed them.
        let padding = 16usize.saturating_sub(label.len() + 1);
        writeln!(w, "{marker}{label}:{:padding$} {value}", "")?;
    }
    Ok(())
}

/// The occasion one hazard warning reports — the CAUSE: a transition, made from one starting state, at
/// the input condition that state stands at. Detection files one record per (cause, outcome), so the
/// records sharing an occasion are the outcomes observed there, and the warning names them together.
#[derive(PartialEq, Eq, Hash)]
pub struct Occasion<'a> {
    cause: &'a Cause,
    condition: &'a Minterm<Symbol>,
    state: &'a Minterm<Symbol>,
}

impl<'a> Occasion<'a> {
    /// The occasion `hazard` was observed on.
    pub fn of(hazard: &'a Hazard) -> Self {
        Self {
            cause: &hazard.cause,
            condition: &hazard.condition,
            state: &hazard.state,
        }
    }
}

/// What one outcome does at an occasion: the nodes that reading puts at risk, and the states the
/// machine lands at once the timing is honoured. Both are gathered over the occasion's records of that
/// outcome — the victims unioned, each kept at the position it was first named, and the landings kept in
/// the order the records state them, since a pulse's are a sequence and a race's alternatives.
#[derive(Default)]
struct Effect<'a> {
    victims: Vec<&'a Symbol>,
    landings: Vec<&'a Minterm<Symbol>>,
}

/// One occasion's warning: a header naming what causes the hazard and the state it is caused from, over
/// a detail block that names the effect. `records` are the occasion's detected hazards, one per outcome
/// observed; the fields that follow from the occasion alone — its condition and the path into its state
/// — are the same in each, so they are read from the first, while each outcome contributes a field of
/// its own naming the nodes THAT reading puts at risk and where it leaves them.
pub fn hazard_warning<'a>(
    w: &mut impl io::Write,
    cell: &AnalysedCell,
    occasion: &Occasion,
    records: &[&'a Hazard],
) -> io::Result<()> {
    let first = records
        .first()
        .expect("an occasion is only entered by a record");
    // One entry per outcome, over the nodes and landing states every record of that outcome names.
    let mut effects: HashMap<Outcome, Effect<'a>> = HashMap::new();
    for h in records {
        let effect = effects.entry(h.outcome).or_default();
        for n in &h.group {
            if !effect.victims.contains(&n) {
                effect.victims.push(n);
            }
        }
        effect.landings.extend(&h.settled);
    }
    // Successive landings naming the same state are one place the machine comes to rest: a pulse's two
    // waypoints coincide wherever the closing edge moves nothing the outcome names, and reporting that
    // state twice would offer the reader two landings to tell apart where there is only one. A race's
    // are already distinct, detection holding them as a set.
    for effect in effects.values_mut() {
        effect.landings.dedup();
    }
    // The values every field is written from, held here so the field list can borrow them. `orders`
    // and `triggered by` each report one outcome, so each is present only where that outcome was
    // observed at this occasion.
    let when = first.condition();
    let path = Path(first.path());
    // A pulse returns its pin to the value it started from, so the pre-pulse input state IS the
    // condition the hazard occurs under — `when` states it, and a separate pre-hazard field would only
    // restate it. A toggle and a race leave their pins where they landed, so the state they started
    // from is worth naming.
    let pre_state = match occasion.cause {
        Cause::Toggle { .. } | Cause::Race { .. } => Some(State(first.pre_state())),
        Cause::Pulse { .. } => None,
    };
    // Which order the edges arrive in is what the settled state depends on, and ordering takes two of
    // them: a lone toggle has no second edge to arrive after.
    let pair = match occasion.cause {
        Cause::Race { pins } => Some(pins),
        Cause::Toggle { .. } | Cause::Pulse { .. } => None,
    };
    let orders = pair
        .filter(|_| effects.contains_key(&Outcome::Indeterminate))
        .map(Orders);
    let trigger =
        Trigger::of(occasion.cause).filter(|_| effects.contains_key(&Outcome::Oscillation));
    let outcomes: Vec<OutcomeField> = effects
        .iter()
        .map(|(outcome, effect)| OutcomeField {
            label: outcome_str(*outcome),
            effect: EffectField {
                cause: occasion.cause,
                effect,
            },
        })
        .collect();

    let mut fields: Vec<SubblockField> = vec![
        SubblockField {
            label: "when",
            value: &when,
        },
        SubblockField {
            label: "reached along",
            value: &path,
        },
    ];
    if let Some(pre_state) = &pre_state {
        fields.push(SubblockField {
            label: "pre-hazard",
            value: pre_state,
        });
    }
    if let Some(orders) = &orders {
        fields.push(SubblockField {
            label: "orders",
            value: orders,
        });
    }
    if let Some(trigger) = &trigger {
        fields.push(SubblockField {
            label: "triggered by",
            value: trigger,
        });
    }
    fields.extend(outcomes.iter().map(|o| SubblockField {
        label: o.label,
        value: &o.effect,
    }));

    writeln!(
        w,
        "cellsmith: warning: cell {:?}: {} causes a hazard at {}",
        cell.repr_name(),
        CauseHeader(occasion.cause),
        State(occasion.state),
    )?;
    subblock(w, "    ", &fields)
}

/// What causes the hazard, as the header names it: the timing that has to be wrong for the cell to be
/// at risk, rather than the transition itself. A pulse is a hazard when it is too SHORT — exactly what
/// the generated minimum pulse width forbids — and a pair of edges when too little separates them, what
/// the generated setup/hold separation forbids. A lone toggle observed not to converge has no second
/// edge to be separated from, and no constraint follows from it, so there the transition is the whole of
/// the condition.
struct CauseHeader<'a>(&'a Cause);

impl fmt::Display for CauseHeader<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            Cause::Toggle { pin } => write!(f, "toggling {pin}"),
            Cause::Race { pins: [a, b] } => write!(f, "too little separation between {a} and {b}"),
            Cause::Pulse { pin } => write!(f, "a short pulse on {pin}"),
        }
    }
}

/// The label an outcome's own field carries — the name it is reported under, beside the nodes that
/// reading puts at risk.
fn outcome_str(outcome: Outcome) -> &'static str {
    match outcome {
        Outcome::Indeterminate => "indeterminate",
        Outcome::Oscillation => "oscillation",
    }
}

/// One outcome's field value: the nodes it puts at risk, and — where the records name any — the states
/// the machine lands at once the timing IS honoured, which for a short pulse is where it would have gone
/// had the pulse been wide enough.
///
/// The landings are joined by what the cause makes them. An input cause's are ALTERNATIVES: either
/// winner is a legitimate result of separating the edges, and nothing orders them among themselves, so
/// they read as `or`. A pulse's are the two waypoints a wide enough one walks through — where the
/// opening edge's own cascade comes to rest, and then where the closing edge leaves the machine — so
/// they read with the same `→` the path field uses for a sequence.
///
/// The clause is absent, rather than empty, where the records name no landing at all: a lone toggle has
/// no second edge to be separated from, and a pair whose every order rings has no timing that brings the
/// machine to rest either. The header and `triggered by` already say which of the two it is.
struct EffectField<'a> {
    cause: &'a Cause,
    effect: &'a Effect<'a>,
}

impl fmt::Display for EffectField<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{{}}}", Commas(&self.effect.victims))?;
        if self.effect.landings.is_empty() {
            return Ok(());
        }
        let separator = match self.cause {
            Cause::Toggle { .. } | Cause::Race { .. } => " or ",
            Cause::Pulse { .. } => " → ",
        };
        f.write_str(" lands at ")?;
        for (i, state) in self.effect.landings.iter().enumerate() {
            if i > 0 {
                f.write_str(separator)?;
            }
            write!(f, "{}", State(state))?;
        }
        Ok(())
    }
}

/// One outcome's landing field: the label the outcome is reported under and the effect written beside
/// it.
struct OutcomeField<'a> {
    label: &'static str,
    effect: EffectField<'a>,
}

/// The triggering transitions of an indeterminate race: the two orders its edges can arrive in, since
/// which lands first is what the settled state depends on (`A↓ then B↑ vs B↑ then A↓`).
struct Orders<'a>(&'a [PinEdge; 2]);

impl fmt::Display for Orders<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let [a, b] = self.0;
        write!(f, "{a} then {b} vs {b} then {a}")
    }
}

/// The triggering transition of an oscillating cause: a pair arrives together, which is what drives the
/// cycle (`simultaneous toggle S↓ & R↓`), and a lone toggle arrives with nothing to coincide with
/// (`toggling A↓`). The variant is which of the two it is, so each carries the edges its own wording
/// names and no other.
enum Trigger<'a> {
    Toggle(&'a PinEdge),
    Simultaneous(&'a [PinEdge; 2]),
}

impl<'a> Trigger<'a> {
    /// The trigger `cause` names, or `None` where it names none: a pulse is its own two edges, which the
    /// header already states in full, so the warning carries no field for it.
    fn of(cause: &'a Cause) -> Option<Self> {
        match cause {
            Cause::Toggle { pin } => Some(Trigger::Toggle(pin)),
            Cause::Race { pins } => Some(Trigger::Simultaneous(pins)),
            Cause::Pulse { .. } => None,
        }
    }
}

impl fmt::Display for Trigger<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Trigger::Toggle(pin) => write!(f, "toggling {pin}"),
            Trigger::Simultaneous([a, b]) => write!(f, "simultaneous toggle {a} & {b}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;
    use crate::logic::machine::ExplorationBudget;
    use crate::model::parse_spec;

    const MULTI: &str = r#"
[[cell]]
name = "C2"
inputs = ["A", "B"]
[cell.outputs]
Q = "A*B + Q*(A+B)"

[[cell]]
name = "MUT"
inputs = ["A", "B"]
[cell.outputs]
Qa = "!Qb * A"
Qb = "!Qa * B"

[[cell]]
name = "DFF"
inputs = ["CLK", "D"]
clock = ["CLK"]
[cell.internal]
M = "!CLK*D + CLK*M"
[cell.outputs]
Q = "CLK*M + !CLK*Q"
"#;

    /// The warning [`hazard_warning`] writes for every occasion of `spec`'s cells, each cell analysed
    /// under the default budget and its arc view's hazards gathered by the occasion they were observed
    /// on.
    fn warnings(spec: &str) -> Vec<String> {
        let cells = parse_spec(spec)
            .unwrap()
            .analyse_with(&ExplorationBudget::default())
            .unwrap();
        let mut warnings = Vec::new();
        for c in &cells {
            let mut occasions: HashMap<Occasion, Vec<&Hazard>> = HashMap::new();
            for h in &c.arc_view().hazards {
                occasions.entry(Occasion::of(h)).or_default().push(h);
            }
            for (occasion, records) in &occasions {
                let mut out = Vec::new();
                hazard_warning(&mut out, c, occasion, records).unwrap();
                warnings.push(String::from_utf8(out).unwrap());
            }
        }
        warnings
    }

    /// The value of the `label:` field in the one hazard warning whose header contains `header`.
    fn hazard_field<'a>(warnings: &'a [String], header: &str, label: &str) -> &'a str {
        let entries: Vec<&str> = warnings
            .iter()
            .map(String::as_str)
            .filter(|e| e.contains(header))
            .collect();
        assert_eq!(entries.len(), 1, "{header} names one entry:\n{warnings:#?}");
        let prefix = format!("{label}:");
        entries[0]
            .lines()
            .find_map(|l| l.trim_start().strip_prefix(&prefix))
            .unwrap_or_else(|| panic!("no {label} field:\n{}", entries[0]))
            .trim_start()
    }

    /// Asserts a `lands at` field names `group`, over a set of race alternatives equal to
    /// `alternatives` — `Hazard::settled`'s alternatives are a set, so the order they render in
    /// carries nothing and either order is a valid rendering.
    fn assert_lands_at_set(field: &str, group: &str, alternatives: &[&str]) {
        let (got_group, landings) = field
            .split_once(" lands at ")
            .expect("a landing field names its group");
        assert_eq!(got_group, group);
        let got: HashSet<&str> = landings.split(" or ").collect();
        assert_eq!(got, alternatives.iter().copied().collect());
    }

    /// Every hazard kind names where the machine lands, beside the nodes it attacks. That landing is
    /// `Hazard::settled` — for a race the results of its two orders, alternatives joined by `or`; for a
    /// pulse the two waypoints one wide enough walks through, in causal order and joined by `→`. Each
    /// expectation below is derived from the cell's own equations, and all four kinds are covered:
    /// race→indeterminate, race→oscillation, pulse→indeterminate and pulse→oscillation.
    #[test]
    fn every_hazard_kind_names_where_the_machine_lands() {
        let warnings = warnings(MULTI);

        // C2 (`Q = A*B + Q*(A+B)`) raced from `{A=1, B=0, Q=0}`: A↓ first leaves both inputs low, so Q stays
        // 0 and the later B↑ cannot lift it; B↑ first co-asserts the pair, which drives Q to 1, and the
        // later A↓ leaves Q holding on B. Either order is a legitimate settling, so the two read as
        // alternatives.
        assert_lands_at_set(
            hazard_field(
                &warnings,
                r#"cell "C2": too little separation between A↓ and B↑ causes a hazard at {A=1, B=0, Q=0}"#,
                "indeterminate",
            ),
            "{Q}",
            &["{Q=0}", "{Q=1}"],
        );

        // MUT (`Qa = !Qb*A`, `Qb = !Qa*B`) with A↑ and B↑ separated from the idle state: whichever request
        // rises first takes its grant and locks the other out, so the ring settles to one grant or the
        // mirror.
        assert_lands_at_set(
            hazard_field(
                &warnings,
                r#"cell "MUT": too little separation between A↑ and B↑ causes a hazard at {A=0, B=0, Qa=0, Qb=0}"#,
                "oscillation",
            ),
            "{Qa, Qb}",
            &["{Qa=0, Qb=1}", "{Qa=1, Qb=0}"],
        );

        // DFF (`M = !CLK*D + CLK*M`, `Q = CLK*M + !CLK*Q`) pulsed low on CLK from `{CLK=1, D=1, Q=0, M=0}`:
        // the opening CLK↓ opens the master and it takes D, resting at `{Q=0, M=1}`; the closing CLK↑ then
        // hands that to the slave, leaving `{Q=1, M=1}`. The two waypoints differ, and the pulse walks the
        // first to reach the second.
        assert_eq!(
            hazard_field(
                &warnings,
                r#"cell "DFF": a short pulse on CLK↓ causes a hazard at {CLK=1, D=1, Q=0, M=0}"#,
                "indeterminate",
            ),
            "{Q, M} lands at {Q=0, M=1} → {Q=1, M=1}",
        );

        // MUT pulsed low on A from `{A=1, B=1, Qa=1, Qb=0}`: A↓ drops A's grant and B's, waiting, takes it;
        // A↑ back finds B holding, so the machine is already where the closing edge leaves it and the two
        // waypoints name one landing. Both outcomes are observed here and both state it.
        for outcome in ["indeterminate", "oscillation"] {
            assert_eq!(
                hazard_field(
                    &warnings,
                    r#"cell "MUT": a short pulse on A↓ causes a hazard at {A=1, B=1, Qa=1, Qb=0}"#,
                    outcome,
                ),
                "{Qa, Qb} lands at {Qa=0, Qb=1}",
                "the {outcome} outcome states where a wide enough pulse lands",
            );
        }
    }
}
