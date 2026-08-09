//! The **constraints** generated from a cell's detected hazards: the record every constraint arc is
//! rendered from, and the pass that derives them.
//!
//! Detection ([`super::confluence`] and [`super::width`]) reports what the machine does under
//! closely-timed input changes; generation here states the timing that removes it. A constraint follows
//! the hazard's [`Cause`] — what the timing is between — and nothing else: the
//! [`Outcome`](super::hazard::Outcome) is what told detection there was a hazard at all, and the same
//! timing removes a race whether it settles indeterminately or never settles.
//!
//! - [`Cause::Race`] naming two pins is a **separation** between them: a directed
//!   [`ConstraintKind::SetupHold`] (clock ← data — the DFF's `D` around `CLK`) where exactly one of the
//!   pair is a declared clock, else a symmetric [`ConstraintKind::NonSeq`] (a mutex's `A`/`B`, a
//!   C-element's `A↓`/`B↑`, an SR latch's simultaneous release). Clocks are *declared* inputs; the race
//!   geometry is left out of the decision because inferring a clock from race order would be
//!   state-dependent — the same pins read one way from one held state and the other way from another —
//!   so it would distinguish nothing real. A race naming ONE pin yields no constraint: a separation
//!   states that two edges stay apart, and one edge has nothing to be separated from.
//! - [`Cause::Pulse`] is a [`ConstraintKind::MinPulseWidth`]: the width a pulse on that one pin must
//!   have for the nodes the record names to reach the outcome the reference close settles to (see
//!   [`super::width`] for the reference). Liberate measures that width off the emitted block, narrowing
//!   the pulse until the probed behaviour fails.
//!
//! [`Constraint::pin`] is the pin the constraint constrains — the emitted block's `-pin` — under every
//! kind. What a kind adds is the OTHER pin of a separation, which a minimum pulse width does not have.
//!
//! **One constraint per situation.** A situation is a cause, at one input condition, over one set of
//! protected nodes — the three components of `constraint_key`. Several records can read one: two
//! phenomena of a single probe, a pair that both diverges and never settles, a pulse that both rings and
//! lands somewhere its reference does not, and the same claim reached from several probed states. One
//! constraint removes them all, and they meet here as that constraint: they key alike, and the one that
//! survives is the min `discovered`. The records collapsing there are readings of one situation and so
//! equally good; the key exists to land the fold on one of them, not to prefer any. The dedup map is a
//! [`BTreeMap`], so the generated order is deterministic independent of any hash map's.

use std::collections::BTreeMap;

use espresso_logic::{Minterm, Symbol};

use crate::logic::arcs::{ArcLevels, Edge};
use crate::logic::hazard::{Cause, Hazard, Racer};

/// What a constraint relates its pin to: the other pin of a separation, or the pin itself.
///
/// Picking the variant IS the classification — a minimum pulse width holds one pin against its own
/// second edge, so there is no second pin for it to name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConstraintKind {
    /// A directed separation: the constrained pin is data, and this is the declared clock it is held
    /// around.
    SetupHold { clock: Symbol, clock_edge: Edge },
    /// A symmetric separation between two requests, neither of which is a declared clock.
    NonSeq { other: Symbol, other_edge: Edge },
    /// A minimum pulse width on the constrained pin, which the emitted block names on both `-pin` and
    /// `-related_pin`: the constraint relates the pin to itself.
    MinPulseWidth,
}

/// One constraint generated to remove a detected hazard, rendered by the arcs emitter as the
/// `define_arc` block(s) its kind calls for.
#[derive(Debug, Clone)]
pub struct Constraint {
    pub kind: ConstraintKind,
    /// The pin this constraint constrains — the block's `-pin`.
    pub pin: Symbol,
    /// The edge that pin makes: the data edge of a separation, or the pulse's OPENING polarity, rise
    /// meaning the pulse is high and fall low. A minimum-pulse-width block states that one edge, and
    /// Liberate searches the width itself.
    pub pin_edge: Edge,
    /// Primary-input condition under which the hazard this constraint avoids occurs, as a full input
    /// assignment. A pulse returns every input to its pre-pulse value, so for
    /// [`ConstraintKind::MinPulseWidth`] this is the pre-pulse input state.
    pub condition: Minterm<Symbol>,
    /// The prevector: the input-assignment path that drives every state variable into the state where
    /// the constraint manifests (each node projected onto the inputs).
    pub prevector: Vec<Minterm<Symbol>>,
    /// The levels the cell's outputs hold in that state — the constraint arc's `-ic` initial condition,
    /// sampled at the same probed state as `prevector`.
    pub levels: ArcLevels,
    /// The nodes this constraint protects, each with the level it holds at the probed state: the state
    /// variables whose settled value the hazard puts at risk — a flop's master latch, for the setup
    /// constraint that separates its clock from its data — in signal declaration order. The emitted
    /// block gives each a column of its own and names them all in one Liberate `-probe`, so the
    /// characterisation measures the nodes the constraint is actually about.
    pub nodes: Vec<(Symbol, bool)>,
    /// The probed state itself: every input and state variable at the level it holds there. The
    /// prevector reaches it and the levels sample its pins, but only this names the internal nodes no
    /// emitted column carries.
    pub state: Minterm<Symbol>,
    /// Index of the probed state in the sequential BFS exploration order — the tie-break key the
    /// situation collapse and emission's general-block choice both land on one representative by.
    pub discovered: usize,
    /// Which of the four (cause, outcome) cells the observation this constraint was generated from
    /// occupies, as [`Hazard::ordinal`] numbers them. The constraint follows the cause alone, so nothing
    /// here decides what is constrained; emission reads it to tell two observations of one cause apart —
    /// a ring and a divergence are different phenomena over their own nodes — and as the last component
    /// of the total order it picks a representative by.
    pub ordinal: u8,
}

/// Generate the constraints that avoid a cell's detected hazards: one [`Constraint`] per detected
/// [`Hazard`] whose cause calls for one, deduplicated per situation (see the module note). `clock_pins`
/// are the cell's declared clocks, which decide a separation's kind and nothing else.
pub(crate) fn constrain(hazards: &[Hazard], clock_pins: &[Symbol]) -> Vec<Constraint> {
    let mut found: BTreeMap<String, Constraint> = BTreeMap::new();
    for hazard in hazards {
        if let Some(c) = remedy(hazard, clock_pins) {
            record(&mut found, c);
        }
    }
    found.into_values().collect()
}

/// The constraint that removes `hazard`, or `None` where its cause states no timing to constrain — a
/// race observed under a single pin, which names one edge and so no separation.
fn remedy(hazard: &Hazard, clock_pins: &[Symbol]) -> Option<Constraint> {
    let (kind, pin, pin_edge) = match &hazard.cause {
        Cause::Race { pins } => {
            let [x, y] = pins.as_slice() else {
                return None; // fewer pins than a separation can relate
            };
            separation(x, y, clock_pins)
        }
        Cause::Pulse { pin, edge } => (ConstraintKind::MinPulseWidth, pin.clone(), *edge),
    };
    Some(Constraint {
        kind,
        pin,
        pin_edge,
        condition: hazard.condition.clone(),
        prevector: hazard.prevector.clone(),
        levels: hazard.levels.clone(),
        nodes: protected(&hazard.group, &hazard.node_levels),
        state: hazard.state.clone(),
        discovered: hazard.discovered,
        ordinal: hazard.ordinal(),
    })
}

/// The separation that holds two racing pins apart, as the kind and the pin it constrains: a directed
/// setup/hold when exactly one of the pair is a declared clock — the other pin being the data the clock
/// is constrained against — else a symmetric non_seq of the two as they were probed.
fn separation(x: &Racer, y: &Racer, clock_pins: &[Symbol]) -> (ConstraintKind, Symbol, Edge) {
    let is_clock = |r: &Racer| clock_pins.contains(&r.pin);
    if is_clock(x) ^ is_clock(y) {
        let (clk, data) = if is_clock(x) { (x, y) } else { (y, x) };
        (
            ConstraintKind::SetupHold {
                clock: clk.pin.clone(),
                clock_edge: clk.edge,
            },
            data.pin.clone(),
            data.edge,
        )
    } else {
        (
            ConstraintKind::NonSeq {
                other: x.pin.clone(),
                other_edge: x.edge,
            },
            y.pin.clone(),
            y.edge,
        )
    }
}

/// The nodes a hazard puts at risk, each with the level the observation sampled for it. A record samples
/// its levels for its own group, at the state it was probed from, so every entry is there.
fn protected(group: &[Symbol], levels: &BTreeMap<Symbol, bool>) -> Vec<(Symbol, bool)> {
    group
        .iter()
        .map(|node| {
            let level = *levels
                .get(node)
                .expect("a hazard observation samples every node of its own group");
            (node.clone(), level)
        })
        .collect()
}

/// Does `c` remove `hazard` — is the constraint the remedy generated for that observation?
///
/// The three fields compared name the observation between them: the probed state and the input condition
/// fix the cause (a race's condition is the probed state with the racing pins toggled, a pulse's is the
/// state's own input assignment), and a record's `group` is what its constraint protects. So it holds of
/// the record `c` carries the probed state of, and of every other reading of that same probe: a ring and
/// a divergence of one situation answer to it alike, whichever of them supplied the representative. An
/// emitter asks it to annotate a constraint with the phenomenon that motivated it, which is why the
/// probed state is compared — the annotation describes the very context the block renders.
pub(crate) fn constrains(c: &Constraint, hazard: &Hazard) -> bool {
    c.state == hazard.state
        && c.condition == hazard.condition
        && c.nodes.iter().map(|(node, _)| node).eq(hazard.group.iter())
}

/// A constraint's protected nodes as one key fragment, in their own order.
fn names_of(nodes: &[(Symbol, bool)]) -> String {
    nodes
        .iter()
        .map(|(n, _)| n.as_str())
        .collect::<Vec<_>>()
        .join(",")
}

/// A canonical dedup key, one keyspace per kind: setup/hold is directed, non_seq is unordered over its
/// two pins, and a minimum pulse width names the one pin it constrains.
///
/// Every kind keys on the same three components, which together ARE the situation: the CAUSE (the pins
/// and edges above), the input CONDITION it was observed under, and the NODES it protects. Two records
/// agreeing on all three describe one situation and yield one constraint; differing on any one of them
/// they are separate claims, each stated. The condition carries its own weight — the same edges racing
/// with a side input held one way put a different question to the characterisation than with it held the
/// other — and the nodes theirs: the same edges endangering different nodes are different constraints,
/// each characterised from its own state.
fn constraint_key(c: &Constraint) -> String {
    let constrained = format!("{}{}", c.pin, c.pin_edge.rf());
    let condition = crate::logic::literals_str(&c.condition);
    match &c.kind {
        ConstraintKind::SetupHold { clock, clock_edge } => format!(
            "SH|{clock}{}|{constrained}|{condition}|{}",
            clock_edge.rf(),
            names_of(&c.nodes)
        ),
        ConstraintKind::NonSeq { other, other_edge } => {
            let a = format!("{other}{}", other_edge.rf());
            let (lo, hi) = if a <= constrained {
                (a, constrained)
            } else {
                (constrained, a)
            };
            format!("NS|{lo}|{hi}|{condition}|{}", names_of(&c.nodes))
        }
        ConstraintKind::MinPulseWidth => {
            format!("MPW|{constrained}|{condition}|{}", names_of(&c.nodes))
        }
    }
}

/// Record a generated constraint into the dedup map, keeping the min `discovered` representative per
/// canonical key.
///
/// The records that meet on one key are readings of ONE situation, so they are equally good and the key
/// expresses no preference between them: `discovered` is a BFS index, not a quality, and it is not even
/// stable between runs. What it buys is that a fold lands on one answer within a run, and choosing among
/// equally-good representatives is free.
fn record(found: &mut BTreeMap<String, Constraint>, c: Constraint) {
    let key = constraint_key(&c);
    // The `Option` read here is the incumbent — no entry yet for this constraint, or one this candidate
    // beats on `discovered` — nothing to do with a state value's determinacy.
    if found.get(&key).is_none_or(|e| c.discovered < e.discovered) {
        found.insert(key, c);
    }
}
