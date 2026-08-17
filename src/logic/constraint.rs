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
//! Which pins a NAMED selection reaches a constraint by is the kind's to state as well, and
//! [`Constraint::selected_by`] states it.
//!
//! **One constraint per situation, and a situation is a CAUSE.** A cause is a starting state and a
//! transition, so `Situation` is the kind with its pins and the edge each makes, plus the state the
//! probe acted from. What is NOT in it is the effect: which node suffers what. Several records read one
//! situation — a pair that both diverges and never settles, a pulse that both rings and lands somewhere
//! its reference does not — and one constraint removes them all, because the timing that removes a
//! cause removes every consequence of it. They meet here as that constraint, whose `nodes` are the
//! UNION of the victims each of them named: one block, probing every node any outcome of the cause
//! attacks.
//!
//! The STATE is the key, not the input condition it projects to. Every other arc kind in this tool keys
//! on the state a measurement is taken from, and a state-holding cell reaches one input assignment in
//! several stored states: keying on the condition would fold those together here, before emission had a
//! chance to see them. Emission is where that fold belongs — two constraints its `-ic` and `-vector`
//! cannot tell apart render one block, and the masked-arc warning names them, which is how a spec author
//! learns that the nodes the cell exposes do not distinguish two situations it is being asked to
//! characterise.

use std::collections::hash_map::Entry;
use std::collections::{BTreeMap, HashMap};

use espresso_logic::{Minterm, Symbol};

use crate::logic::arcs::{ArcLevels, Edge};
use crate::logic::hazard::{Cause, Hazard, Racer};
use crate::model::ConstraintPins;

/// What a constraint relates its pin to: the other pin of a separation, or the pin itself.
///
/// Picking the variant IS the classification — a minimum pulse width holds one pin against its own
/// second edge, so there is no second pin for it to name.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum ConstraintKind {
    /// A directed separation: the constrained pin is data, and this is the declared clock it is held
    /// around.
    SetupHold { clock: Symbol, clock_edge: Edge },
    /// A symmetric separation between two requests, neither of which is a declared clock.
    NonSeq { other: Symbol, other_edge: Edge },
    /// A minimum pulse width on the constrained pin, which the emitted block names on both `-pin` and
    /// `-related_pin`: the constraint relates the pin to itself.
    MinPulseWidth,
}

/// One node a hazard attacks: a state variable whose settled value it puts at risk — a flop's master
/// latch, under the race between its clock and its data — and the level that node holds at the probed
/// state.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct VictimNode {
    /// The state variable itself, which the emitted block names in its `-probe`.
    pub(crate) node: Symbol,
    /// The level it holds at the probed state, which the block's `-ic` initialises its column to.
    pub(crate) level: bool,
}

/// One constraint generated to remove a detected hazard, rendered by the arcs emitter as the
/// `define_arc` block(s) its kind calls for.
#[derive(Debug, Clone)]
pub(crate) struct Constraint {
    pub(crate) kind: ConstraintKind,
    /// The pin this constraint constrains — the block's `-pin`.
    pub(crate) pin: Symbol,
    /// The edge that pin makes: the data edge of a separation, or the pulse's OPENING polarity, rise
    /// meaning the pulse is high and fall low. A minimum-pulse-width block states that one edge, and
    /// Liberate searches the width itself.
    pub(crate) pin_edge: Edge,
    /// The prevector: the input-assignment path that drives every state variable into the state where
    /// the constraint manifests (each node projected onto the inputs).
    pub(crate) prevector: Vec<Minterm<Symbol>>,
    /// The levels the cell's outputs hold in that state — the constraint arc's `-ic` initial condition,
    /// sampled at the same probed state as `prevector`.
    pub(crate) levels: ArcLevels,
    /// The nodes the constrained cause attacks, in signal declaration order: the union of the victims
    /// every outcome of that cause named. The emitted block gives each a column of its own and names them
    /// all in one Liberate `-probe`, so the characterisation measures every node the cause puts at risk.
    pub(crate) nodes: Vec<VictimNode>,
    /// The probed state itself: every input and state variable at the level it holds there. The
    /// prevector reaches it and the levels sample its pins, but only this names the internal nodes no
    /// emitted column carries.
    pub(crate) state: Minterm<Symbol>,
    /// Index of the probed state in the sequential BFS exploration order — the tie-break key the
    /// situation collapse and emission's general-block choice both land on one representative by.
    pub(crate) discovered: usize,
    /// Which of the four (cause, outcome) cells the observations this constraint was generated from
    /// occupy, as [`Hazard::ordinal`] numbers them — the lowest, where a cause showed more than one
    /// outcome. The constraint follows the cause alone, so nothing here decides what is constrained; it
    /// is the last component of the total order emission picks a representative by.
    pub(crate) ordinal: u8,
}

impl Constraint {
    /// The names alone of the nodes this constraint's cause attacks, in `nodes` order — what the emitted
    /// `-probe` lists, and what emission compares by containment when it decides which observation
    /// speaks for the constraint.
    ///
    /// The level beside each name belongs to the ONE probed state this record was measured from, so two
    /// observations of the same constraint carry the same names holding whatever their own states hold.
    /// Whatever identifies a constraint therefore reads the names, and the levels stay here, with the
    /// state they were sampled at.
    pub(crate) fn victim_names(&self) -> Vec<Symbol> {
        self.nodes.iter().map(|v| v.node.clone()).collect()
    }

    /// Does `selection` ask for this constraint? The pins that reach it are the ones its KIND gives a
    /// role to, so the answer is read off the variant rather than off whichever end happens to be
    /// stored in [`Constraint::pin`].
    ///
    /// A [`ConstraintKind::NonSeq`] is symmetric — its two pins are equals — so naming EITHER end names
    /// the separation that holds them apart. A [`ConstraintKind::SetupHold`] is directed: the data pin
    /// is constrained with respect to the clock, so the data pin selects it and the clock does not.
    /// Naming a clock asks for what that clock is itself subject to — its own minimum pulse width — and
    /// not for the separations other pins are held around it by. A [`ConstraintKind::MinPulseWidth`]
    /// names one pin, which is the pin that selects it.
    pub(crate) fn selected_by(&self, selection: &ConstraintPins) -> bool {
        match &self.kind {
            ConstraintKind::NonSeq { other, .. } => {
                selection.selects(&self.pin) || selection.selects(other)
            }
            ConstraintKind::SetupHold { .. } | ConstraintKind::MinPulseWidth => {
                selection.selects(&self.pin)
            }
        }
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
        prevector: hazard.prevector.clone(),
        levels: hazard.levels.clone(),
        nodes: victims(&hazard.group, &hazard.node_levels),
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

/// The nodes a hazard attacks, each with the level the observation sampled for it. A record samples its
/// levels for its own group, at the state it was probed from, so every entry is there.
fn victims(group: &[Symbol], levels: &BTreeMap<Symbol, bool>) -> Vec<VictimNode> {
    group
        .iter()
        .map(|node| VictimNode {
            node: node.clone(),
            level: *levels
                .get(node)
                .expect("a hazard observation samples every node of its own group"),
        })
        .collect()
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
    SetupHold { data: Racer, clock: Racer },
    /// A symmetric separation, as the unordered pair of the two edges it holds apart.
    NonSeq { pair: [Racer; 2] },
    /// A minimum pulse width, on the one pin it holds against that pin's own second edge.
    MinPulseWidth { pin: Racer },
}

impl SituationKind {
    fn of(c: &Constraint) -> Self {
        let constrained = Racer {
            pin: c.pin.clone(),
            edge: c.pin_edge,
        };
        match &c.kind {
            ConstraintKind::SetupHold { clock, clock_edge } => SituationKind::SetupHold {
                data: constrained,
                clock: Racer {
                    pin: clock.clone(),
                    edge: *clock_edge,
                },
            },
            ConstraintKind::NonSeq { other, other_edge } => {
                let mut pair = [
                    constrained,
                    Racer {
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

/// The CAUSE two observations share when they are readings of ONE situation, and so state one
/// constraint: the pins with the edge each makes and the kind that gives them their roles, plus the
/// state the probe acted from.
///
/// The effect is deliberately absent. Two readings of one cause endanger their own nodes — a ring is not
/// a disagreement between landing points — and the constraint that removes the cause removes both, so
/// the victims merge into the one record rather than splitting it in two.
///
/// The starting state carries the discrimination the input condition would: a condition is that state's
/// input projection, and a state-holding cell reaches one condition in several stored states, which the
/// state tells apart and the condition does not.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Situation {
    kind: SituationKind,
    state: Minterm<Symbol>,
}

impl Situation {
    fn of(c: &Constraint) -> Self {
        Situation {
            kind: SituationKind::of(c),
            state: c.state.clone(),
        }
    }
}

/// Record a generated constraint into the dedup map, merging it into any reading of the same situation
/// already there.
///
/// A situation keys on the probed state, so every reading of one was measured at that single state and
/// carries the same prevector, output levels, input condition and exploration index. What two readings
/// genuinely differ in is the victims they name and which (cause, outcome) cell they were read from: the
/// victims merge, so the block probes every node any outcome of the cause attacks, and the ordinal keeps
/// the lower, which lands emission's representative choice on one answer. Neither states a preference —
/// the merge is a union and the ordinal a fixed numbering of the four cells — and the order the surviving
/// constraints come out in states nothing.
fn record(found: &mut HashMap<Situation, Constraint>, c: Constraint) {
    match found.entry(Situation::of(&c)) {
        Entry::Occupied(mut e) => {
            let kept = e.get_mut();
            kept.nodes = merged_victims(&kept.nodes, &c.nodes, &kept.state);
            kept.ordinal = kept.ordinal.min(c.ordinal);
        }
        Entry::Vacant(e) => {
            e.insert(c);
        }
    }
}

/// The victims of two readings of one situation, merged: every node either names, in the probed state's
/// own column order — which for a state variable is signal declaration order, the order each reading's
/// own list is already in.
///
/// Both readings were measured at `state`, since the situation keys on it, so a node they share holds one
/// level and the merge cannot disagree with itself.
fn merged_victims(
    kept: &[VictimNode],
    other: &[VictimNode],
    state: &Minterm<Symbol>,
) -> Vec<VictimNode> {
    let levels: HashMap<&Symbol, bool> = kept
        .iter()
        .chain(other)
        .map(|v| (&v.node, v.level))
        .collect();
    state
        .vars()
        .iter()
        .filter_map(|node| {
            levels.get(node).map(|level| VictimNode {
                node: node.clone(),
                level: *level,
            })
        })
        .collect()
}
