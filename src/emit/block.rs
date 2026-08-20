//! One emitted Liberate block, as the value it is.
//!
//! A [`Block`] is one command — a `define_arc` under one of its nine `-type`s, or a `define_leakage`
//! under one of its two forms — and the variant IS that form: it carries what that form states and
//! nothing another form would need, so the `-type` word, the pins the block names and the lines it
//! writes are one choice rather than several that have to agree. What varies WITHIN a form is a field:
//! a measured block may or may not carry a `-when`, and that is the `Option` inside the variant.
//!
//! The value is what the emitter compares and hashes to decide whether the cell has already stated the
//! block (see [`crate::emit::arcs_tcl`]), so everything a block says is here and nothing else is. It
//! becomes text once, in [`Display`](fmt::Display), written into the writer the deck is going out on.

use std::fmt;

use espresso_logic::{BoolExpr, Symbol};

use crate::emit::tcl::{Braced, IcColumn, VectorValue, Words};
use crate::logic::arcs::{Edge, PinEdge};

/// The two pins one constraint block switches, and the edge each makes: the pin it names on
/// `-related_pin` and the pin it names on `-pin`, in that order. A separation carries the pair it holds
/// apart; a minimum pulse width carries its one pin and relates it to itself (`RacingPins::pulse`).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RacingPins {
    pub(crate) related: PinEdge,
    pub(crate) pin: PinEdge,
}

impl RacingPins {
    /// The pins a minimum-pulse-width block switches: the one pin it constrains, raced against its own
    /// second edge, so the block names that pin on both `-related_pin` and `-pin`.
    pub(crate) fn pulse(pin: &PinEdge) -> Self {
        RacingPins {
            related: pin.clone(),
            pin: pin.clone(),
        }
    }

    /// The edge `input` makes in the block's `-vector`, or `None` where it is neither of the switching
    /// pins — the vector then holding it at the level it starts from.
    pub(crate) fn edge_of(&self, input: &str) -> Option<Edge> {
        [&self.related, &self.pin]
            .into_iter()
            .find(|p| p.pin.as_str() == input)
            .map(|p| p.edge)
    }
}

/// One column of a measured block: the node the `-pinlist` names it by, the voltage `-ic` starts it at
/// and the value `-vector` drives it to.
///
/// Liberate reads those three lines against each other by POSITION, and here they are three projections
/// of one list, so a column cannot reach one of them and miss another. The name is the one the block's
/// netlist holds the node on, which is what Liberate has to be handed.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Column {
    pub(crate) name: Symbol,
    /// The voltage the column starts at, as the cell's `logic_low`/`logic_high` expression for that
    /// level. `None` where the block states no start condition at all: a cell that holds no state has
    /// none to state, and then no column of it carries one — the `-ic` line is the whole list or
    /// nothing.
    pub(crate) ic: Option<IcColumn>,
    pub(crate) value: VectorValue,
}

/// One column of the `define_leakage` form that states a rest state through its own columns: the node
/// and the level it rests at. A rest state is static — the block measures no transition — so there is
/// no start condition for an `-ic` to establish, and the form that never carries one cannot hold one.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LevelColumn {
    pub(crate) name: Symbol,
    pub(crate) level: bool,
}

/// What a `define_arc` measuring a transition states: the columns it drives, the condition it was
/// measured under, and the two pins the measurement runs between — the related pin it starts at and the
/// output it lands on, each with the edge that pin makes.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Transition {
    pub(crate) columns: Vec<Column>,
    /// The context the block is characterised in, or `None` where it generalises over every context the
    /// transition was measured from.
    pub(crate) when: Option<BoolExpr>,
    pub(crate) related: PinEdge,
    pub(crate) output: PinEdge,
    /// The drive-strength aliases this one block speaks for.
    pub(crate) names: Vec<Symbol>,
}

/// What a hidden `define_arc` states: an input toggle that settles with no output following it. The
/// toggle is the whole of the event, so this is the one form naming no related pin.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Toggle {
    pub(crate) columns: Vec<Column>,
    pub(crate) when: Option<BoolExpr>,
    pub(crate) pin: PinEdge,
    pub(crate) names: Vec<Symbol>,
}

/// What one member of a constraint separation states: the pair of pins it holds apart, and the nodes it
/// watches while they switch.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Separation {
    pub(crate) columns: Vec<Column>,
    pub(crate) when: Option<BoolExpr>,
    pub(crate) pins: RacingPins,
    /// The victim nodes the constraint is measured against, which Liberate is handed in one `-probe`.
    pub(crate) probe: Vec<Symbol>,
    pub(crate) names: Vec<Symbol>,
}

/// What a minimum-pulse-width `define_arc` states: the ONE pin whose pulse is constrained, named on both
/// `-related_pin` and `-pin`, and the nodes watched across it. Holding a single pin is what makes a
/// block naming two different pins here unrepresentable.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Pulse {
    pub(crate) columns: Vec<Column>,
    pub(crate) when: Option<BoolExpr>,
    pub(crate) pin: PinEdge,
    pub(crate) probe: Vec<Symbol>,
    pub(crate) names: Vec<Symbol>,
}

/// What a `define_leakage` states through its own columns: every column of the block held at the level
/// the rest state carries. Those columns are how it states an internal node, which a condition cannot
/// name because an internal node has no pin.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Held {
    pub(crate) columns: Vec<LevelColumn>,
    /// The condition the cell rests under. A leakage block always carries one — it is the whole of what
    /// the bare form states — so a cell fixing no literal rests under the tautology `1`.
    pub(crate) when: BoolExpr,
    pub(crate) names: Vec<Symbol>,
}

/// What a `define_leakage` states as a bare condition: the inputs drive the cell into the state on their
/// own, so naming them states it and the block carries no column. Carrying none, it is divided by no
/// alias group either — the one block names every alias of the cell.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Resting {
    pub(crate) when: BoolExpr,
    pub(crate) names: Vec<Symbol>,
}

/// One block a cell states. The nine `define_arc` variants ARE Liberate's `-type` taxonomy — the three
/// measured transitions, the hidden toggle and the five constraint types — and the last two are the two
/// forms `define_leakage` takes, a command that carries no `-type`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Block {
    /// A transition an asynchronous pin drives, that pin being declared such by the spec.
    Async(Transition),
    /// A transition a clock edge drives.
    Edge(Transition),
    /// A transition the cell's logic drives directly.
    Combinational(Transition),
    /// An input toggle that settles with no output following it, drawn for its internal power.
    Hidden(Toggle),
    /// The setup member of a directed clock↔data separation.
    Setup(Separation),
    /// The hold member of a directed clock↔data separation.
    Hold(Separation),
    /// The setup member of a symmetric separation — an oscillation or a mutual exclusion, where neither
    /// pin is the clock of the other.
    NonSeqSetup(Separation),
    /// The hold member of a symmetric separation.
    NonSeqHold(Separation),
    /// The width a pulse must keep for the probed nodes to go on behaving.
    MinPulseWidth(Pulse),
    /// A rest state stated through the block's own columns.
    LeakageHeld(Held),
    /// A rest state the inputs alone drive the cell into, stated as the bare condition.
    LeakageResting(Resting),
}

/// The whole block, written into the caller's writer. Each arm writes its `-type` word as a literal of
/// its own, so the word Liberate reads and the variant the emitter grouped the block under cannot come
/// apart; what follows is the arm's payload writing the lines its form states.
impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Block::Async(t) => t.write(f, "async"),
            Block::Edge(t) => t.write(f, "edge"),
            Block::Combinational(t) => t.write(f, "combinational"),
            Block::Hidden(t) => t.write(f, "hidden"),
            Block::Setup(s) => s.write(f, "setup"),
            Block::Hold(s) => s.write(f, "hold"),
            Block::NonSeqSetup(s) => s.write(f, "non_seq_setup"),
            Block::NonSeqHold(s) => s.write(f, "non_seq_hold"),
            Block::MinPulseWidth(p) => p.write(f, "min_pulse_width"),
            Block::LeakageHeld(h) => h.write(f),
            Block::LeakageResting(r) => r.write(f),
        }
    }
}

impl Transition {
    fn write(&self, f: &mut fmt::Formatter<'_>, kind: &str) -> fmt::Result {
        write_type(f, kind)?;
        write_columns(f, &self.columns)?;
        write_when(f, self.when.as_ref())?;
        writeln!(f, "\t-related_pin {} \\", self.related.pin)?;
        writeln!(f, "\t-pin {} \\", self.output.pin)?;
        write_names(f, &self.names)
    }
}

impl Toggle {
    fn write(&self, f: &mut fmt::Formatter<'_>, kind: &str) -> fmt::Result {
        write_type(f, kind)?;
        write_columns(f, &self.columns)?;
        write_when(f, self.when.as_ref())?;
        writeln!(f, "\t-pin {} \\", self.pin.pin)?;
        write_names(f, &self.names)
    }
}

impl Separation {
    fn write(&self, f: &mut fmt::Formatter<'_>, kind: &str) -> fmt::Result {
        write_type(f, kind)?;
        write_columns(f, &self.columns)?;
        write_when(f, self.when.as_ref())?;
        writeln!(f, "\t-related_pin {} \\", self.pins.related.pin)?;
        writeln!(f, "\t-pin {} \\", self.pins.pin.pin)?;
        writeln!(f, "\t-probe {} \\", Braced(Words(&self.probe)))?;
        write_names(f, &self.names)
    }
}

impl Pulse {
    fn write(&self, f: &mut fmt::Formatter<'_>, kind: &str) -> fmt::Result {
        write_type(f, kind)?;
        write_columns(f, &self.columns)?;
        write_when(f, self.when.as_ref())?;
        // The one pin is related to itself: what the block holds apart is the pulse's two edges.
        writeln!(f, "\t-related_pin {} \\", self.pin.pin)?;
        writeln!(f, "\t-pin {} \\", self.pin.pin)?;
        writeln!(f, "\t-probe {} \\", Braced(Words(&self.probe)))?;
        write_names(f, &self.names)
    }
}

impl Held {
    fn write(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "define_leakage \\")?;
        writeln!(
            f,
            "\t-pinlist {} \\",
            Braced(Projected(&self.columns, |c: &LevelColumn| &c.name))
        )?;
        writeln!(
            f,
            "\t-vector {} \\",
            Braced(Projected(&self.columns, |c: &LevelColumn| {
                VectorValue::from(c.level)
            }))
        )?;
        writeln!(f, "\t-when \"{}\" \\", self.when)?;
        write_names(f, &self.names)
    }
}

impl Resting {
    fn write(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "define_leakage -when \"{}\" {{ {} }}",
            self.when,
            Words(&self.names)
        )?;
        writeln!(f)
    }
}

/// The two lines every `define_arc` opens with: the command, and the `-type` its arm names.
fn write_type(f: &mut fmt::Formatter<'_>, kind: &str) -> fmt::Result {
    writeln!(f, "define_arc \\")?;
    writeln!(f, "\t-type {kind} \\")
}

/// The `-pinlist`, `-ic` and `-vector` a column list writes, in the order Liberate reads them against
/// each other.
///
/// The `-ic` line is the whole list or nothing at all, which is what the columns' own `Option` states: a
/// state-holding cell starts every column of the block somewhere and says so, and a cell holding no
/// state has no start condition to state.
fn write_columns(f: &mut fmt::Formatter<'_>, columns: &[Column]) -> fmt::Result {
    writeln!(
        f,
        "\t-pinlist {} \\",
        Braced(Projected(columns, |c: &Column| &c.name))
    )?;
    if let Some(ic) = columns
        .iter()
        .map(|c| c.ic.as_ref())
        .collect::<Option<Vec<_>>>()
    {
        // The `-ic` values are one double-quoted word, never a braced one: Tcl substitutes no variable
        // inside braces, so a braced `$VDD` would reach Liberate as that literal text instead of the
        // supply voltage. A single column within the word carries braces of its own where the expression
        // it holds needs them to stay one column.
        writeln!(f, "\t-ic \"{}\" \\", Words(&ic))?;
    }
    writeln!(
        f,
        "\t-vector {} \\",
        Braced(Projected(columns, |c: &Column| c.value))
    )
}

/// The `-when` line, or nothing where the block states no condition.
fn write_when(f: &mut fmt::Formatter<'_>, when: Option<&BoolExpr>) -> fmt::Result {
    match when {
        Some(when) => writeln!(f, "\t-when \"{when}\" \\"),
        None => Ok(()),
    }
}

/// The line a block closes on — the aliases it speaks for — and the blank line separating it from the
/// next block.
fn write_names(f: &mut fmt::Formatter<'_>, names: &[Symbol]) -> fmt::Result {
    writeln!(f, "\t{{ {} }}", Words(names))?;
    writeln!(f)
}

/// One projection of a column list as a Tcl list body: what each column contributes to one line, written
/// in turn and separated by a single space. This is [`Words`] over a list whose items are read out of a
/// larger value, and it is what makes `-pinlist`, `-ic` and `-vector` three walks of the same columns in
/// the same order.
struct Projected<'a, C, T, F: Fn(&'a C) -> T>(&'a [C], F);

impl<'a, C, T: fmt::Display, F: Fn(&'a C) -> T> fmt::Display for Projected<'a, C, T, F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, column) in self.0.iter().enumerate() {
            if i > 0 {
                f.write_str(" ")?;
            }
            write!(f, "{}", (self.1)(column))?;
        }
        Ok(())
    }
}

/// One block on a single line, its form leading the pins it relates: `combinational A↑ -> Q↓`,
/// `hidden S↑`, `setup CLK↑ & D↑`, `min_pulse_width CLK↑`, `leakage`. This is how a diagnostic names the
/// block it reports on, and it is read off the same variant that writes the block's `-type`.
pub struct Description<'a>(pub &'a Block);

impl fmt::Display for Description<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            Block::Async(t) => write!(f, "async {} -> {}", t.related, t.output),
            Block::Edge(t) => write!(f, "edge {} -> {}", t.related, t.output),
            Block::Combinational(t) => write!(f, "combinational {} -> {}", t.related, t.output),
            Block::Hidden(t) => write!(f, "hidden {}", t.pin),
            Block::Setup(s) => write!(f, "setup {} & {}", s.pins.related, s.pins.pin),
            Block::Hold(s) => write!(f, "hold {} & {}", s.pins.related, s.pins.pin),
            Block::NonSeqSetup(s) => write!(f, "non_seq_setup {} & {}", s.pins.related, s.pins.pin),
            Block::NonSeqHold(s) => write!(f, "non_seq_hold {} & {}", s.pins.related, s.pins.pin),
            Block::MinPulseWidth(p) => write!(f, "min_pulse_width {}", p.pin),
            // A `define_leakage` names no pin, so its form is the whole of the description.
            Block::LeakageHeld(_) | Block::LeakageResting(_) => f.write_str("leakage"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::logic::product;

    fn sym(name: &str) -> Symbol {
        Symbol::from(name)
    }

    fn names(of: &[&str]) -> Vec<Symbol> {
        of.iter().map(|n| sym(n)).collect()
    }

    fn pin(name: &str, edge: Edge) -> PinEdge {
        PinEdge {
            pin: sym(name),
            edge,
        }
    }

    /// One measured column: the node, the voltage it starts at where the cell holds state, and the value
    /// the vector drives it to.
    fn column(name: &str, ic: Option<&str>, value: VectorValue) -> Column {
        Column {
            name: sym(name),
            ic: ic.map(|v| IcColumn(v.to_owned())),
            value,
        }
    }

    /// A condition as the product of literals every `-when` here is built from.
    fn when(lits: &[(&str, bool)]) -> BoolExpr {
        let lits: Vec<(Symbol, bool)> = lits.iter().map(|(n, v)| (sym(n), *v)).collect();
        product(&lits)
    }

    /// The columns of a state-holding cell's `A B Q` block: `A` rises and takes `Q` with it, `B` is held
    /// high, and every column states the voltage it starts at.
    fn measured() -> Vec<Column> {
        vec![
            column("A", Some("0"), VectorValue::Rise),
            column("B", Some("$VDD"), VectorValue::High),
            column("Q", Some("0"), VectorValue::Rise),
        ]
    }

    #[test]
    fn a_transition_block_states_its_type_columns_condition_and_pins() {
        let block = Block::Combinational(Transition {
            columns: measured(),
            when: Some(when(&[("B", true)])),
            related: pin("A", Edge::Rise),
            output: pin("Q", Edge::Rise),
            names: names(&["C2"]),
        });
        assert_eq!(
            block.to_string(),
            "define_arc \\\n\
             \t-type combinational \\\n\
             \t-pinlist {A B Q} \\\n\
             \t-ic \"0 $VDD 0\" \\\n\
             \t-vector {R 1 R} \\\n\
             \t-when \"B\" \\\n\
             \t-related_pin A \\\n\
             \t-pin Q \\\n\
             \t{ C2 }\n\n"
        );
    }

    #[test]
    fn a_general_block_carries_no_when_line() {
        // The general pass generalises by carrying no condition, so the line is absent rather than
        // empty — and the `-type` word is the arm's own.
        let block = Block::Edge(Transition {
            columns: measured(),
            when: None,
            related: pin("A", Edge::Rise),
            output: pin("Q", Edge::Rise),
            names: names(&["C2", "C2X4"]),
        });
        assert_eq!(
            block.to_string(),
            "define_arc \\\n\
             \t-type edge \\\n\
             \t-pinlist {A B Q} \\\n\
             \t-ic \"0 $VDD 0\" \\\n\
             \t-vector {R 1 R} \\\n\
             \t-related_pin A \\\n\
             \t-pin Q \\\n\
             \t{ C2 C2X4 }\n\n"
        );
    }

    #[test]
    fn a_hidden_block_names_the_toggled_pin_and_no_related_pin() {
        // A cell that holds no state has no start condition to state, so no column carries one and the
        // whole `-ic` line is absent.
        let block = Block::Hidden(Toggle {
            columns: vec![
                column("A", None, VectorValue::Rise),
                column("B", None, VectorValue::Low),
                column("Y", None, VectorValue::Low),
            ],
            when: None,
            pin: pin("A", Edge::Rise),
            names: names(&["AND2"]),
        });
        assert_eq!(
            block.to_string(),
            "define_arc \\\n\
             \t-type hidden \\\n\
             \t-pinlist {A B Y} \\\n\
             \t-vector {R 0 0} \\\n\
             \t-pin A \\\n\
             \t{ AND2 }\n\n"
        );
    }

    #[test]
    fn a_separation_block_names_both_pins_and_its_probe() {
        let block = Block::Setup(Separation {
            columns: vec![
                column("CLK", Some("0"), VectorValue::Rise),
                column("D", Some("$VDD"), VectorValue::Rise),
                column("M", Some("0"), VectorValue::Unstated),
                column("Q", Some("0"), VectorValue::Unstated),
            ],
            when: Some(when(&[("Q", false)])),
            pins: RacingPins {
                related: pin("CLK", Edge::Rise),
                pin: pin("D", Edge::Rise),
            },
            probe: names(&["M", "Q"]),
            names: names(&["DFF"]),
        });
        assert_eq!(
            block.to_string(),
            "define_arc \\\n\
             \t-type setup \\\n\
             \t-pinlist {CLK D M Q} \\\n\
             \t-ic \"0 $VDD 0 0\" \\\n\
             \t-vector {R R X X} \\\n\
             \t-when \"!Q\" \\\n\
             \t-related_pin CLK \\\n\
             \t-pin D \\\n\
             \t-probe {M Q} \\\n\
             \t{ DFF }\n\n"
        );
    }

    #[test]
    fn a_minimum_pulse_width_block_relates_its_one_pin_to_itself() {
        let block = Block::MinPulseWidth(Pulse {
            columns: vec![
                column("CLK", Some("$VDD"), VectorValue::Fall),
                column("D", Some("0"), VectorValue::Low),
                column("Q", Some("0"), VectorValue::Unstated),
            ],
            when: None,
            pin: pin("CLK", Edge::Fall),
            probe: names(&["Q"]),
            names: names(&["DFF"]),
        });
        assert_eq!(
            block.to_string(),
            "define_arc \\\n\
             \t-type min_pulse_width \\\n\
             \t-pinlist {CLK D Q} \\\n\
             \t-ic \"$VDD 0 0\" \\\n\
             \t-vector {F 0 X} \\\n\
             \t-related_pin CLK \\\n\
             \t-pin CLK \\\n\
             \t-probe {Q} \\\n\
             \t{ DFF }\n\n"
        );
    }

    #[test]
    fn a_held_rest_state_states_every_column_at_its_level() {
        let block = Block::LeakageHeld(Held {
            columns: vec![
                LevelColumn {
                    name: sym("A"),
                    level: true,
                },
                LevelColumn {
                    name: sym("B"),
                    level: false,
                },
                LevelColumn {
                    name: sym("Q"),
                    level: true,
                },
            ],
            when: when(&[("A", true), ("B", false), ("Q", true)]),
            names: names(&["C2"]),
        });
        assert_eq!(
            block.to_string(),
            "define_leakage \\\n\
             \t-pinlist {A B Q} \\\n\
             \t-vector {1 0 1} \\\n\
             \t-when \"A & !B & Q\" \\\n\
             \t{ C2 }\n\n"
        );
    }

    #[test]
    fn a_resting_rest_state_is_the_bare_condition() {
        let block = Block::LeakageResting(Resting {
            when: when(&[("A", false), ("B", false), ("Q", false)]),
            names: names(&["C2"]),
        });
        assert_eq!(
            block.to_string(),
            "define_leakage -when \"!A & !B & !Q\" { C2 }\n\n"
        );
    }

    #[test]
    fn a_block_describes_itself_on_one_line() {
        // The description leads with the same word the block's `-type` does, and names the pins the
        // variant itself carries. A `define_leakage` names none, so its form is the whole of it.
        let transition = || Transition {
            columns: measured(),
            when: None,
            related: pin("A", Edge::Rise),
            output: pin("Q", Edge::Fall),
            names: names(&["C2"]),
        };
        let separation = || Separation {
            columns: measured(),
            when: None,
            pins: RacingPins {
                related: pin("CLK", Edge::Rise),
                pin: pin("D", Edge::Rise),
            },
            probe: names(&["M"]),
            names: names(&["DFF"]),
        };
        let described: Vec<String> = [
            Block::Async(transition()),
            Block::Edge(transition()),
            Block::Combinational(transition()),
            Block::Hidden(Toggle {
                columns: measured(),
                when: None,
                pin: pin("S", Edge::Rise),
                names: names(&["SR"]),
            }),
            Block::Setup(separation()),
            Block::Hold(separation()),
            Block::NonSeqSetup(separation()),
            Block::NonSeqHold(separation()),
            Block::MinPulseWidth(Pulse {
                columns: measured(),
                when: None,
                pin: pin("CLK", Edge::Rise),
                probe: names(&["Q"]),
                names: names(&["DFF"]),
            }),
            Block::LeakageHeld(Held {
                columns: Vec::new(),
                when: when(&[("A", true)]),
                names: names(&["C2"]),
            }),
            Block::LeakageResting(Resting {
                when: when(&[("A", true)]),
                names: names(&["C2"]),
            }),
        ]
        .iter()
        .map(|b| Description(b).to_string())
        .collect();
        assert_eq!(
            described,
            [
                "async A↑ -> Q↓",
                "edge A↑ -> Q↓",
                "combinational A↑ -> Q↓",
                "hidden S↑",
                "setup CLK↑ & D↑",
                "hold CLK↑ & D↑",
                "non_seq_setup CLK↑ & D↑",
                "non_seq_hold CLK↑ & D↑",
                "min_pulse_width CLK↑",
                "leakage",
                "leakage",
            ]
        );
    }
}
