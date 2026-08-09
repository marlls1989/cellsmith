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
//! protected nodes — the three components of `Situation`. Several records can read one: two
//! phenomena of a single probe, a pair that both diverges and never settles, a pulse that both rings and
//! lands somewhere its reference does not, and the same claim reached from several probed states. One
//! constraint removes them all, and they meet here as that constraint: they answer to one `Situation`,
//! and the one that survives is the min `discovered`. The records collapsing there are readings of one
//! situation and so equally good; the situation exists to land the fold on one of them, not to prefer
//! any, and the order the surviving constraints come out in states nothing.

use std::collections::{BTreeMap, HashMap};

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

/// One node a constraint protects: a state variable whose settled value the hazard puts at risk — a
/// flop's master latch, for the setup constraint that separates its clock from its data — and the level
/// it holds at the probed state.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProtectedNode {
    /// The state variable itself, which the emitted block names in its `-probe`.
    pub node: Symbol,
    /// The level it holds at the probed state, which the block's `-ic` initialises its column to.
    pub level: bool,
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
    /// The nodes this constraint protects, in signal declaration order. The emitted block gives each a
    /// column of its own and names them all in one Liberate `-probe`, so the characterisation measures
    /// the nodes the constraint is actually about.
    pub nodes: Vec<ProtectedNode>,
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

impl Constraint {
    /// The names alone of the nodes this constraint protects, in `nodes` order — what the emitted
    /// `-probe` lists, and what emission compares by containment when it decides which observation
    /// speaks for the constraint.
    ///
    /// The level beside each name belongs to the ONE probed state this record was measured from, so two
    /// observations of the same constraint carry the same names holding whatever their own states hold.
    /// Whatever identifies a constraint therefore reads the names, and the levels stay here, with the
    /// state they were sampled at.
    pub fn protected_names(&self) -> Vec<Symbol> {
        self.nodes.iter().map(|p| p.node.clone()).collect()
    }
}

/// Generate the constraints that avoid a cell's detected hazards: one [`Constraint`] per detected
/// [`Hazard`] whose cause calls for one, deduplicated per situation (see the module note). `clock_pins`
/// are the cell's declared clocks, which decide a separation's kind and nothing else.
pub(crate) fn constrain(hazards: &[Hazard], clock_pins: &[Symbol]) -> Vec<Constraint> {
    let mut found: HashMap<Situation, Constraint> = HashMap::new();
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
fn protected(group: &[Symbol], levels: &BTreeMap<Symbol, bool>) -> Vec<ProtectedNode> {
    group
        .iter()
        .map(|node| ProtectedNode {
            node: node.clone(),
            level: *levels
                .get(node)
                .expect("a hazard observation samples every node of its own group"),
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
        && c.nodes.iter().map(|p| &p.node).eq(hazard.group.iter())
}

/// A pin, and the edge it makes, as a [`Situation`] names it.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct PinEdge {
    pin: Symbol,
    edge: Edge,
}

/// The pins one situation is about, each with the edge it makes: a constraint's kind carrying EVERY pin
/// it names, the constrained one included, so no pin sits outside the variant that gives it its role.
///
/// A symmetric separation is symmetric in the type. [`SituationKind::NonSeq`] holds its two ends as one
/// sorted pair, so the pair a probe reached one way round and the pair another reached the other way
/// round are the same value by construction, and which end a record calls its constrained pin settles
/// nothing.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum SituationKind {
    /// A directed separation. Its two ends have different roles — data held around a declared clock — so
    /// each keeps a field of its own.
    SetupHold { data: PinEdge, clock: PinEdge },
    /// A symmetric separation, as the unordered pair of the two edges it holds apart.
    NonSeq { pair: [PinEdge; 2] },
    /// A minimum pulse width, on the one pin it holds against that pin's own second edge.
    MinPulseWidth { pin: PinEdge },
}

impl SituationKind {
    fn of(c: &Constraint) -> Self {
        let constrained = PinEdge {
            pin: c.pin.clone(),
            edge: c.pin_edge,
        };
        match &c.kind {
            ConstraintKind::SetupHold { clock, clock_edge } => SituationKind::SetupHold {
                data: constrained,
                clock: PinEdge {
                    pin: clock.clone(),
                    edge: *clock_edge,
                },
            },
            ConstraintKind::NonSeq { other, other_edge } => {
                let mut pair = [
                    constrained,
                    PinEdge {
                        pin: other.clone(),
                        edge: *other_edge,
                    },
                ];
                pair.sort();
                SituationKind::NonSeq { pair }
            }
            ConstraintKind::MinPulseWidth => SituationKind::MinPulseWidth { pin: constrained },
        }
    }
}

/// What two observations share when they are readings of ONE situation, and so state one constraint: the
/// CAUSE (the pins, the edge each makes, and the kind that gives them their roles), the input CONDITION
/// they were observed under, and the NODES they protect.
///
/// Two records agreeing on all three describe one situation and yield one constraint; differing on any
/// one of them they are separate claims, each stated. The condition carries its own weight — the same
/// edges racing with a side input held one way put a different question to the characterisation than
/// with it held the other — and the nodes theirs: the same edges endangering different nodes are
/// different constraints, each characterised from its own state.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Situation {
    kind: SituationKind,
    condition: Minterm<Symbol>,
    nodes: Vec<Symbol>,
}

impl Situation {
    fn of(c: &Constraint) -> Self {
        Situation {
            kind: SituationKind::of(c),
            condition: c.condition.clone(),
            nodes: c.protected_names(),
        }
    }
}

/// Record a generated constraint into the dedup map, keeping the min `discovered` representative per
/// situation.
///
/// The records that meet on one situation are readings of it, so they are equally good and the situation
/// expresses no preference between them: `discovered` is a BFS index, not a quality, and it is not even
/// stable between runs. What it buys is that a fold lands on one answer within a run, and choosing among
/// equally-good representatives is free.
fn record(found: &mut HashMap<Situation, Constraint>, c: Constraint) {
    let situation = Situation::of(&c);
    // The `Option` read here is the incumbent — no entry yet for this situation, or one this candidate
    // beats on `discovered` — nothing to do with a state value's determinacy.
    if found
        .get(&situation)
        .is_none_or(|e| c.discovered < e.discovered)
    {
        found.insert(situation, c);
    }
}
