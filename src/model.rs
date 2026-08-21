//! Input model: a minimal multi-cell TOML spec, plus analysis that classifies each function's
//! variables into **primary inputs** vs **feedback/state** (an output name referenced inside a
//! function is the delayed/feedback value of that output).

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::io;

use espresso_logic::bdd::{Bdd, BddBuilder, Brand, ManagerCell};
use espresso_logic::{sync_bdd_builder, BoolExpr, Symbol};
use indexmap::IndexMap;
use rayon::prelude::*;
use serde::Deserialize;

use crate::logic::analysis::{Derivations, Exploration};
use crate::logic::arcs::{Arc, HiddenArc};
use crate::logic::constraint::Constraint;
use crate::logic::hazard::Hazard;
use crate::logic::leakage::LeakageState;
use crate::logic::machine::{ExplorationBudget, ExplorationLimit, Explored};

/// The whole input file: a list of `[[cell]]` tables.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Spec {
    #[serde(rename = "cell", default)]
    pub cells: Vec<Cell>,
}

/// One cell exactly as written in the TOML.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Cell {
    /// Physical cell name(s) used in the emitted arcs. A scalar or a list; the first entry is the
    /// representative name for single-name contexts.
    #[serde(deserialize_with = "de_name_list")]
    pub name: Vec<Symbol>,
    /// Primary input pins. Order matters: it defines the pinlist/vector order.
    #[serde(deserialize_with = "de_symbol_vec")]
    pub(crate) inputs: Vec<Symbol>,
    /// Output pin name -> Boolean function, parsed at deserialise time. Entries arrive in the order
    /// the TOML parser yields them — sorted by name, not as written in the file — and that order is
    /// stable from then on.
    ///
    /// The function text's grammar is a superset of the `a*b+!c` form: `*`/`&` AND, `+`/`|` OR,
    /// `!`/`~` NOT, `^` XOR, `0`/`1`/`true`/`false` constants, and parentheses for grouping.
    /// Precedence, tightest first: NOT > AND > XOR > OR. Identifiers are a letter/`_` followed by
    /// letters/digits/`_` (so pin names like `M1`, `P2`, `Q` are fine).
    #[serde(deserialize_with = "de_symbol_expr_map")]
    pub(crate) outputs: IndexMap<Symbol, BoolExpr>,
    /// Optional: internal state variable name -> Boolean function, parsed at deserialise time (same
    /// grammar and name-sorted ordering as [`Cell::outputs`]). An internal signal is referenceable by other
    /// functions and is a driven state variable (modelled in the Verilog and the Liberty state
    /// table), but emits **no** external output pin and is never an arc source or target.
    #[serde(default, deserialize_with = "de_symbol_expr_map")]
    pub(crate) internal: IndexMap<Symbol, BoolExpr>,
    /// Optional: the internal nodes listed in the Liberate arcs' `-pinlist`, in declared order (the
    /// declared order fixes their pinlist position). Each is preserved through the state-space
    /// minimisation so the arcs can drive it (`-ic`) and observe it (`-vector`). Spec-only: like
    /// `template`/`template_overrides` above, there is no CLI counterpart.
    #[serde(default, deserialize_with = "de_symbol_vec")]
    pub(crate) expose: Vec<Symbol>,
    /// Optional: input pins that force the output regardless of held state (async set/reset),
    /// so their arcs are emitted as `-type async` rather than combinational.
    #[serde(rename = "async", default, deserialize_with = "de_symbol_vec")]
    pub(crate) async_pins: Vec<Symbol>,
    /// Optional: input pins that are clocks. A hazard on a pin pair holding a declared clock yields a
    /// directed setup/hold constraint (clock ← data); any other pair yields a symmetric non_seq. See
    /// [`crate::logic::confluence`].
    #[serde(default, deserialize_with = "de_symbol_vec")]
    pub(crate) clock: Vec<Symbol>,
    /// Optional: which of this cell's input pins derived constraint arcs (setup/hold, non_seq,
    /// min_pulse_width) are generated for. Accepts a bool (`true` = every pin, `false` = none), a scalar
    /// pin name, or a list of them; absent = none. Unioned with the global `--constraints` CLI flag, and
    /// every named pin is checked against this cell's inputs at analyse time.
    #[serde(default, deserialize_with = "de_constraint_pins")]
    pub constraint_arcs: ConstraintPins,
    /// Optional: opt OUT of the behavioural per-arc edge classification for this cell (see
    /// `crate::logic::edge`). Classification is ON by default; setting this true (or the global
    /// `--no-edge-collapse` CLI flag) suppresses it, leaving every arc in its combinational form.
    #[serde(default)]
    pub no_edge_collapse: bool,
    /// Optional: the per-cell mirror of `--when` — the arc classes whose `-when`-conditioned arcs are
    /// also emitted, unioned with the global flag. One general arc per transition — a related pin's edge
    /// driving an output pin's edge — is always emitted, without a `-when` line, regardless of this set;
    /// a selected class ADDS that class's `-when` arcs on top, so an arc can appear both with and without
    /// its condition. Accepts a bool (`true` = every class, `false` = none), a scalar class name, or a
    /// list of them. Absent = the empty set = only the general arcs, no `-when`.
    #[serde(default, deserialize_with = "de_when")]
    pub when: ArcClasses,
    /// Optional: the cell-wide characterisation-template references for the `define_cell` emitter
    /// (delay/power/constraint). Structural only — the template names come from the spec, never
    /// generated. `None` fields carry through unset.
    #[serde(default)]
    pub(crate) template: Option<TemplateSpec>,
    /// Optional: per-drive-strength-alias template overrides, keyed by a name from this cell's `name`
    /// list. Each alias's [`TemplateSpec`] is merged per-field over the cell-wide `template`. Keys are
    /// validated against the cell's declared names at analyse time.
    #[serde(default, deserialize_with = "de_template_overrides")]
    pub(crate) template_overrides: IndexMap<Symbol, TemplateSpec>,
    /// Optional: the netlist node each internal signal stands for. A spec is written in names that read
    /// well in the behavioural model, while the node a cell actually holds its state on may be spelled
    /// however the netlist spells it (`XI7/m`); this says which is which. See [`NodeNames`].
    #[serde(default, deserialize_with = "de_nodes")]
    pub(crate) nodes: NodeNames,
    /// Optional: override the low-logic-level (`0`) voltage expression the exposed nodes' `-ic` renders.
    /// Falls back to the `--logic-low` CLI default, then to `LogicVoltages::default`'s `"0"`. A Tcl
    /// variable is as good as a literal: the arcs emitter renders the value as one `-ic` column.
    #[serde(default)]
    pub logic_low: Option<String>,
    /// Optional: override the high-logic-level (`1`) voltage expression, mirroring `logic_low`. Falls
    /// back to the `--logic-high` CLI default, then to `LogicVoltages::default`'s `"$VDD"`.
    #[serde(default)]
    pub logic_high: Option<String>,
}

/// Which netlist node each of a cell's internal signals stands for, as declared under `[cell.nodes]`.
///
/// A signal's name in the spec is the one the behavioural model reads well in and is what the Verilog
/// and Liberty artifacts carry; the netlist may hold that state on a node spelled quite differently,
/// and it is that spelling Liberate has to be handed. A signal with no entry stands for itself.
///
/// The map is per cell, and a drive-strength alias may override any of it — the same signal can sit on
/// a different node in each alias's netlist. Resolution is per signal: the alias's own entry if it has
/// one, else the cell-wide entry, else the signal's own name ([`Self::of`]).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct NodeNames {
    /// The cell-wide mapping, in declared order.
    pub(crate) cell: IndexMap<Symbol, Symbol>,
    /// Per-alias mappings, keyed by a name from the cell's `name` list, each in declared order.
    pub(crate) aliases: IndexMap<Symbol, IndexMap<Symbol, Symbol>>,
}

impl NodeNames {
    /// The netlist node `signal` stands for under drive-strength alias `alias`: the alias's own entry,
    /// else the cell-wide entry, else `signal` itself.
    pub(crate) fn of(&self, alias: &Symbol, signal: &Symbol) -> Symbol {
        self.aliases
            .get(alias)
            .and_then(|m| m.get(signal))
            .or_else(|| self.cell.get(signal))
            .cloned()
            .unwrap_or_else(|| signal.clone())
    }
}

/// The characterisation-template references for a cell (or a drive-strength alias override): the
/// `delay`, `power` and `constraint` template names the `define_cell` emitter attaches. Structural
/// only — each name is taken verbatim from the spec, never generated; an absent field is `None`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct TemplateSpec {
    #[serde(default, deserialize_with = "de_opt_symbol")]
    pub(crate) delay: Option<Symbol>,
    #[serde(default, deserialize_with = "de_opt_symbol")]
    pub(crate) power: Option<Symbol>,
    /// Also accepted under the `constrain` spelling.
    #[serde(default, alias = "constrain", deserialize_with = "de_opt_symbol")]
    pub(crate) constraint: Option<Symbol>,
}

/// The voltage expressions the Liberate arcs' `-ic` renders for the two logic levels (`low` for `0`,
/// `high` for `1`). A Tcl variable (`$VDD`) is as good as a literal (`0`) — these are user-supplied Tcl
/// VALUE fragments, not name fields, so this holds `String` rather than the `Symbol` the rest of the
/// model uses for names. Each is carried here exactly as written; making it one `-ic` column is the arcs
/// emitter's job, that being a question about the Tcl line it renders rather than about the cell.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LogicVoltages {
    pub(crate) low: String,
    pub(crate) high: String,
}

impl Default for LogicVoltages {
    /// The Liberate defaults: `0` for low, `$VDD` for high.
    fn default() -> Self {
        Self {
            low: "0".to_owned(),
            high: "$VDD".to_owned(),
        }
    }
}

impl LogicVoltages {
    /// The voltage expression for `level` (`false` → low, `true` → high).
    pub(crate) fn of(&self, level: bool) -> &str {
        if level {
            &self.high
        } else {
            &self.low
        }
    }

    /// Fill each side from its optional override, falling back to [`Default`] where absent.
    fn from_options(low: Option<&str>, high: Option<&str>) -> Self {
        let default = Self::default();
        Self {
            low: low.map(str::to_owned).unwrap_or(default.low),
            high: high.map(str::to_owned).unwrap_or(default.high),
        }
    }
}

/// A class of emitted arc, the granularity at which `-when` arcs are opted into. The `clap::ValueEnum`
/// derive kebab-cases the variants, so the tokens `transition`, `hidden` and `constraint` name the
/// classes on both the CLI (`--when=<CLASS>`) and in the spec (`when = ...`) — one token table, shared by
/// both surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, clap::ValueEnum)]
pub enum ArcClass {
    /// The `define_arc` delay/transition arcs: an input edge on a related pin driving an output edge.
    Transition,
    /// The hidden (internal-power) arcs: an input toggle that settles without changing any output.
    Hidden,
    /// The derived constraint arcs: the setup/hold, non_seq and min_pulse_width blocks generated to
    /// remove a detected hazard.
    Constraint,
}

/// The set of arc classes whose `-when` arcs are also emitted, on top of the always-emitted general
/// arcs. `Default` is the EMPTY set — only the general arcs, no `-when`.
///
/// Membership is a bitmask indexed by [`ArcClass`]'s discriminant: every operation reads a class's bit
/// through `bit`, so the class a bit stands for is fixed by the variant itself.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ArcClasses {
    bits: u8,
}

impl ArcClasses {
    /// Every class selected.
    pub const ALL: Self = Self {
        bits: Self::bit(ArcClass::Transition)
            | Self::bit(ArcClass::Hidden)
            | Self::bit(ArcClass::Constraint),
    };

    /// The bit standing for `class`, its discriminant being the bit index.
    const fn bit(class: ArcClass) -> u8 {
        1 << (class as u8)
    }

    /// Whether `class`'s `-when` arcs are also emitted.
    pub fn contains(self, class: ArcClass) -> bool {
        self.bits & Self::bit(class) != 0
    }

    /// The union of two sets: a class is selected iff either set selects it.
    pub fn union(self, other: Self) -> Self {
        Self {
            bits: self.bits | other.bits,
        }
    }
}

impl FromIterator<ArcClass> for ArcClasses {
    fn from_iter<I: IntoIterator<Item = ArcClass>>(iter: I) -> Self {
        iter.into_iter().fold(Self::default(), |set, class| Self {
            bits: set.bits | Self::bit(class),
        })
    }
}

/// Which of a cell's input pins constraint arcs are generated for, as `constraint_arcs` and the
/// `--constraints` flag select them.
///
/// Picking the variant IS the selection: `Off` asks for none, `All` for every pin the cell's hazards
/// constrain, and `Named` for the listed pins alone. The selection reaches GENERATION only — detection is
/// never gated on it, so a pin left out is still probed and its hazards still detected and reported; what
/// it loses is the constraint block.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum ConstraintPins {
    /// No constraint arcs at all.
    #[default]
    Off,
    /// Constraint arcs for every pin the cell's hazards constrain.
    All,
    /// Constraint arcs for the listed input pins only, in declared order. A named pin brings back the
    /// constraints it has a ROLE in: either end of a symmetric separation, since its two pins are
    /// equals; the data pin of a directed setup/hold, the clock it is held around being what OTHER pins
    /// are constrained against rather than what the clock is subject to; and the pin a minimum pulse
    /// width is on.
    Named(Vec<Symbol>),
}

impl ConstraintPins {
    /// Whether this selection names `pin`. Which of a cell's constraints that reaches is the
    /// constraint's own to answer, from the roles its kind gives its pins — `Constraint::selected_by`.
    pub(crate) fn selects(&self, pin: &Symbol) -> bool {
        match self {
            Self::Off => false,
            Self::All => true,
            Self::Named(pins) => pins.contains(pin),
        }
    }

    /// The pins this selection names, which is none under `Off` and `All`: neither states a pin — one
    /// asks for no constraint, the other for every pin there is. Read by the analyse-time check that a
    /// named pin is a declared input, the one place a pin's NAME rather than the selection it makes is
    /// of interest.
    pub(crate) fn named(&self) -> &[Symbol] {
        match self {
            Self::Off | Self::All => &[],
            Self::Named(pins) => pins,
        }
    }
}

/// Deserialize the cell `name` field as a non-empty `Vec<Symbol>` (order preserving). Accepts either a
/// scalar (`name = "INV"`) or a list (`name = ["INVX1", "INVX2"]`); `Symbol` has no `serde` impl, so
/// each entry is read as a `String` and interned (Display/Debug/Ord delegate to `str`, so the emitted
/// bytes are unchanged). Duplicates are dropped keeping the first occurrence, and an empty list is a
/// hard error.
fn de_name_list<'de, D: serde::Deserializer<'de>>(d: D) -> Result<Vec<Symbol>, D::Error> {
    // String variant FIRST so a TOML scalar matches `One` rather than being probed as a sequence.
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum OneOrMany {
        One(String),
        Many(Vec<String>),
    }
    let names = match OneOrMany::deserialize(d)? {
        OneOrMany::One(s) => vec![s],
        OneOrMany::Many(v) => v,
    };
    let mut out: Vec<Symbol> = Vec::new();
    for s in names {
        let sym = Symbol::from(s);
        if !out.contains(&sym) {
            out.push(sym);
        }
    }
    if out.is_empty() {
        return Err(serde::de::Error::custom("cell name list must be non-empty"));
    }
    Ok(out)
}

/// Deserialize a list of name fields as `Vec<Symbol>` (order preserved), interning each entry.
fn de_symbol_vec<'de, D: serde::Deserializer<'de>>(d: D) -> Result<Vec<Symbol>, D::Error> {
    Vec::<String>::deserialize(d).map(|v| v.into_iter().map(Symbol::from).collect())
}

/// A private newtype around [`BoolExpr`] with a hand-written `Deserialize` impl: `BoolExpr` has no
/// `serde` impl of its own (espresso-logic has no serde dependency), so this parses the TOML string
/// value directly via [`BoolExpr::parse`] — a bad function is a hard error surfaced at file-load, at
/// the value's own TOML span.
struct DeBoolExpr(BoolExpr);

impl<'de> Deserialize<'de> for DeBoolExpr {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct FuncVisitor;

        impl<'de> serde::de::Visitor<'de> for FuncVisitor {
            type Value = DeBoolExpr;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str("a Boolean function string")
            }

            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                BoolExpr::parse(v)
                    .map(DeBoolExpr)
                    .map_err(serde::de::Error::custom)
            }

            fn visit_borrowed_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                BoolExpr::parse(v)
                    .map(DeBoolExpr)
                    .map_err(serde::de::Error::custom)
            }
        }

        d.deserialize_str(FuncVisitor)
    }
}

/// Deserialize a `name -> function` table as `IndexMap<Symbol, BoolExpr>`, interning the keys and
/// parsing each function's text into a [`BoolExpr`] (insertion order preserved). A malformed function
/// is a hard error at deserialise time, reported at the value's TOML span.
fn de_symbol_expr_map<'de, D: serde::Deserializer<'de>>(
    d: D,
) -> Result<IndexMap<Symbol, BoolExpr>, D::Error> {
    IndexMap::<String, DeBoolExpr>::deserialize(d)
        .map(|m| m.into_iter().map(|(k, v)| (Symbol::from(k), v.0)).collect())
}

/// Deserialize an optional template-name field as `Option<Symbol>`, interning the name when present.
/// `Symbol` has no `serde` impl, so the value is read as `Option<String>` and interned via
/// `Symbol::from` (a template name is created once at parse time and shared across many cells).
fn de_opt_symbol<'de, D: serde::Deserializer<'de>>(d: D) -> Result<Option<Symbol>, D::Error> {
    Ok(Option::<String>::deserialize(d)?.map(Symbol::from))
}

/// Deserialize `[cell.nodes]`: one table carrying both halves of the mapping, told apart by the value's
/// TYPE. A string value maps a signal to the netlist node it stands for, cell-wide; a table value is a
/// drive-strength alias's own map, overriding the cell-wide one per signal. So
///
/// ```toml
/// [cell.nodes]
/// sela0 = "XI7/m"      # every alias, unless overridden below
/// [cell.nodes.DFFX4]
/// sela0 = "XI4/m"      # this alias only
/// ```
///
/// A cell name can therefore never be mistaken for a signal name here, whatever it is spelled: the two
/// live in different value positions. Both halves keep their insertion order, and both are validated at
/// analyse time — an alias key against the cell's declared names, a signal key against its internals.
fn de_nodes<'de, D: serde::Deserializer<'de>>(d: D) -> Result<NodeNames, D::Error> {
    /// One entry's value. Read through a hand-written visitor rather than an untagged enum so a value
    /// of neither shape reports the two shapes a spec author can write, in their vocabulary, instead of
    /// naming this type: serde's untagged form can only say the value matched no variant.
    enum Entry {
        /// `signal = "netlist/node"` — the cell-wide mapping.
        Node(String),
        /// `[cell.nodes.<ALIAS>]` — one alias's own mapping.
        Alias(IndexMap<String, String>),
    }
    impl<'de> Deserialize<'de> for Entry {
        fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
            struct V;
            impl<'de> serde::de::Visitor<'de> for V {
                type Value = Entry;
                fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    f.write_str(
                        "a netlist node name, or a table of them keyed by drive-strength name",
                    )
                }
                fn visit_str<E: serde::de::Error>(self, s: &str) -> Result<Entry, E> {
                    Ok(Entry::Node(s.to_owned()))
                }
                fn visit_map<M: serde::de::MapAccess<'de>>(
                    self,
                    mut m: M,
                ) -> Result<Entry, M::Error> {
                    let mut map = IndexMap::new();
                    while let Some((k, v)) = m.next_entry::<String, String>()? {
                        map.insert(k, v);
                    }
                    Ok(Entry::Alias(map))
                }
            }
            d.deserialize_any(V)
        }
    }
    let mut nodes = NodeNames::default();
    for (key, entry) in IndexMap::<String, Entry>::deserialize(d)? {
        match entry {
            Entry::Node(node) => {
                nodes.cell.insert(Symbol::from(key), Symbol::from(node));
            }
            Entry::Alias(map) => {
                nodes.aliases.insert(
                    Symbol::from(key),
                    map.into_iter()
                        .map(|(k, v)| (Symbol::from(k), Symbol::from(v)))
                        .collect(),
                );
            }
        }
    }
    Ok(nodes)
}

/// Deserialize an `alias -> template overrides` table as `IndexMap<Symbol, TemplateSpec>`, interning
/// the keys and keeping the insertion order. `Symbol` has no `serde` impl, so the keys are read as
/// `String` and interned via `Symbol::from`.
fn de_template_overrides<'de, D: serde::Deserializer<'de>>(
    d: D,
) -> Result<IndexMap<Symbol, TemplateSpec>, D::Error> {
    IndexMap::<String, TemplateSpec>::deserialize(d)
        .map(|m| m.into_iter().map(|(k, v)| (Symbol::from(k), v)).collect())
}

/// Deserialize the per-cell `when` field as an [`ArcClasses`] set. Accepts a bool (`true` = every
/// class, `false` = none), a scalar class name (`"hidden"`), or a list of names (`["hidden",
/// "transition"]`). Each name is validated through [`ArcClass`]'s `ValueEnum` parser, so the CLI and the
/// spec share one token table. A bad name is a hard error at the value's own TOML span.
fn de_when<'de, D: serde::Deserializer<'de>>(d: D) -> Result<ArcClasses, D::Error> {
    // Bool and scalar-string variants FIRST so a TOML bool or scalar matches `All`/`One` rather than
    // being probed as a sequence. Any string matches `One`, so the class-name validation happens AFTER
    // this untagged match — a bad name surfaces the `custom` message below, not "did not match any
    // variant".
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum BoolOrClasses {
        All(bool),
        One(String),
        Many(Vec<String>),
    }
    let names: Vec<String> = match BoolOrClasses::deserialize(d)? {
        BoolOrClasses::All(true) => return Ok(ArcClasses::ALL),
        BoolOrClasses::All(false) => return Ok(ArcClasses::default()),
        BoolOrClasses::One(s) => vec![s],
        BoolOrClasses::Many(v) => v,
    };
    names
        .iter()
        .map(|s| {
            <ArcClass as clap::ValueEnum>::from_str(s, false).map_err(|_| {
                serde::de::Error::custom(format!(
                    "unknown when arc class {s:?}: expected \"hidden\" or \"transition\", a bool, or a list of them"
                ))
            })
        })
        .collect::<Result<ArcClasses, _>>()
}

/// Deserialize the per-cell `constraint_arcs` field as a [`ConstraintPins`] selection. Accepts a bool
/// (`true` = every pin, `false` = none), a scalar pin name (`"D"`), or a list of them (`["CLK", "D"]`).
/// A name is validated against the cell's inputs at analyse time ([`ModelError::ConstraintNotInput`]),
/// which is where they are known.
fn de_constraint_pins<'de, D: serde::Deserializer<'de>>(d: D) -> Result<ConstraintPins, D::Error> {
    // Bool and scalar-string variants FIRST so a TOML bool or scalar matches `Every`/`One` rather than
    // being probed as a sequence.
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum BoolOrPins {
        Every(bool),
        One(String),
        Many(Vec<String>),
    }
    Ok(match BoolOrPins::deserialize(d)? {
        BoolOrPins::Every(true) => ConstraintPins::All,
        BoolOrPins::Every(false) => ConstraintPins::Off,
        BoolOrPins::One(s) => ConstraintPins::Named(vec![Symbol::from(s)]),
        BoolOrPins::Many(v) => ConstraintPins::Named(v.into_iter().map(Symbol::from).collect()),
    })
}

/// Why a spec could not be turned into analysed cells.
///
/// Every variant but [`Spec`](Self::Spec) reports a rule the cell model imposes on top of what TOML
/// can express — a name used twice, a pin referenced that was never declared — and names the cell it
/// was checking, so a spec holding many cells says which one is at fault.
// `Clone` and `#[non_exhaustive]` follow the espresso-logic error idiom this crate's errors are written
// in — `CoverError` and its siblings carry the same pair. Nothing in cellsmith clones a `ModelError` or
// matches one from outside the crate; the derives are here so the two crates' errors present one shape.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ModelError {
    /// The source text is not well-formed TOML, or does not deserialise into the spec's shape.
    Spec(toml::de::Error),
    /// A cell's `name` list is empty, so the cell has no name to be reported or emitted under.
    EmptyName,
    /// One name is claimed by two cells. Names — the drive-strength aliases beyond the first included
    /// — are unique across the whole spec, each standing for one cell in the emitted artifacts.
    DuplicateCellName {
        /// The name claimed twice.
        name: Symbol,
    },
    /// A cell lists the same input pin twice.
    DuplicateInput {
        /// The cell being validated.
        cell: Symbol,
        /// The pin repeated in the input list.
        pin: Symbol,
    },
    /// A name is declared as both an input pin and an output, leaving a reference to it ambiguous.
    InputOutputClash {
        /// The cell being validated.
        cell: Symbol,
        /// The name declared on both sides.
        pin: Symbol,
    },
    /// An internal signal reuses a declared input or output name. Internals share the one namespace
    /// with the pins, a function referencing all three alike.
    InternalClash {
        /// The cell being validated.
        cell: Symbol,
        /// The internal signal whose name is already taken.
        pin: Symbol,
    },
    /// A function references a name the cell does not declare. A variable is an input pin, an output
    /// or an internal signal; anything else has no value to read.
    UnknownVar {
        /// The cell being validated.
        cell: Symbol,
        /// The signal whose function holds the reference.
        output: Symbol,
        /// The undeclared variable that was referenced.
        var: Symbol,
    },
    /// The `async` list names a pin that is not a declared input. Asynchronous control arrives on an
    /// input pin.
    AsyncNotInput {
        /// The cell being validated.
        cell: Symbol,
        /// The name listed as asynchronous.
        pin: Symbol,
    },
    /// The `clock` list names a pin that is not a declared input.
    ClockNotInput {
        /// The cell being validated.
        cell: Symbol,
        /// The name listed as a clock.
        pin: Symbol,
    },
    /// `constraint_arcs` selects a pin that is not a declared input. A constraint holds input pins
    /// apart, so a selection names one.
    ConstraintNotInput {
        /// The cell being validated.
        cell: Symbol,
        /// The name selected for constraint arcs.
        pin: Symbol,
    },
    /// A per-alias `template` override is keyed on a name the cell does not carry in its `name` list.
    UnknownTemplateOverride {
        /// The cell being validated.
        cell: Symbol,
        /// The key matching none of the cell's names.
        alias: Symbol,
    },
    /// A `nodes` mapping renames a signal that is not a declared internal. Only an internal stands for
    /// a netlist node; a pin is addressed by its own name.
    NodeNotInternal {
        /// The cell being validated.
        cell: Symbol,
        /// The signal the mapping tried to rename.
        signal: Symbol,
    },
    /// A per-alias `nodes` mapping is keyed on a name the cell does not carry in its `name` list.
    UnknownNodeAlias {
        /// The cell being validated.
        cell: Symbol,
        /// The key matching none of the cell's names.
        alias: Symbol,
    },
    /// Two internal signals map onto one netlist node under an alias, leaving a block that names the
    /// node unable to say which signal it measures.
    DuplicateNode {
        /// The cell being validated.
        cell: Symbol,
        /// The drive-strength alias the mapping belongs to.
        alias: Symbol,
        /// The netlist node reached from two signals.
        node: Symbol,
    },
    /// An internal signal maps onto a netlist node bearing a pin's name, which already addresses the
    /// pin.
    NodeClashesWithPin {
        /// The cell being validated.
        cell: Symbol,
        /// The drive-strength alias the mapping belongs to.
        alias: Symbol,
        /// The netlist node colliding with a pin.
        node: Symbol,
    },
    /// The `expose` list names a signal that is not a declared internal. Exposure preserves an internal
    /// node as a machine coordinate; an output is already one.
    ExposeNotInternal {
        /// The cell being validated.
        cell: Symbol,
        /// The name listed for exposure.
        node: Symbol,
    },
    /// The `expose` list names the same internal signal twice.
    DuplicateExpose {
        /// The cell being validated.
        cell: Symbol,
        /// The node repeated in the expose list.
        node: Symbol,
    },
    /// An exploration budget stopped the cell's machine pass, so the cell has no arcs, hazards,
    /// leakage states or constraints to emit and the run ends here. The counter and its ceiling are
    /// the inner error's to state; this layer walks the cells, so it adds the cell's name and the flag
    /// that raises that ceiling.
    Exploration {
        /// The cell whose exploration was stopped.
        cell: Symbol,
        /// Which budget stopped it, and at what ceiling.
        source: ExplorationLimit,
    },
}

impl fmt::Display for ModelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ModelError::Spec(e) => write!(f, "cannot parse spec: {e}"),
            ModelError::EmptyName => write!(f, "cell name list must be non-empty"),
            ModelError::DuplicateCellName { name } => {
                write!(f, "duplicate cell name {name:?} used by more than one cell")
            }
            ModelError::DuplicateInput { cell, pin } => {
                write!(f, "cell {cell:?}: duplicate input pin {pin:?}")
            }
            ModelError::InputOutputClash { cell, pin } => {
                write!(f, "cell {cell:?}: pin {pin:?} is both an input and an output")
            }
            ModelError::InternalClash { cell, pin } => write!(
                f,
                "cell {cell:?}: internal signal {pin:?} clashes with a declared input or output name"
            ),
            ModelError::UnknownVar { cell, output, var } => write!(
                f,
                "cell {cell:?}, output {output:?}: variable {var:?} is neither a declared input nor an output of this cell"
            ),
            ModelError::AsyncNotInput { cell, pin } => {
                write!(f, "cell {cell:?}: async pin {pin:?} is not a declared input")
            }
            ModelError::ClockNotInput { cell, pin } => {
                write!(f, "cell {cell:?}: clock pin {pin:?} is not a declared input")
            }
            ModelError::ConstraintNotInput { cell, pin } => write!(
                f,
                "cell {cell:?}: constraint pin {pin:?} is not a declared input"
            ),
            ModelError::UnknownTemplateOverride { cell, alias } => write!(
                f,
                "cell {cell:?}: template override alias {alias:?} is not a declared cell name"
            ),
            ModelError::NodeNotInternal { cell, signal } => write!(
                f,
                "cell {cell:?}: node mapping for {signal:?} is not a declared internal signal"
            ),
            ModelError::UnknownNodeAlias { cell, alias } => write!(
                f,
                "cell {cell:?}: node mapping alias {alias:?} is not a declared cell name"
            ),
            ModelError::DuplicateNode { cell, alias, node } => write!(
                f,
                "cell {cell:?}: two nodes resolve to {node:?} under {alias:?}"
            ),
            ModelError::NodeClashesWithPin { cell, alias, node } => write!(
                f,
                "cell {cell:?}: a node resolves to the pin {node:?} under {alias:?}"
            ),
            ModelError::ExposeNotInternal { cell, node } => write!(
                f,
                "cell {cell:?}: exposed node {node:?} is not a declared internal signal"
            ),
            ModelError::DuplicateExpose { cell, node } => {
                write!(f, "cell {cell:?}: duplicate exposed node {node:?}")
            }
            // Which command-line flag raises the ceiling belongs beside the counter that passed it, and
            // this is the outermost layer that still knows which counter that was: `main` prints the
            // error and nothing else. The match is over the crate's own enum, so a third budget cannot
            // be added without a flag being named for it here.
            ModelError::Exploration { cell, source } => {
                let flag = match source {
                    ExplorationLimit::Candidates(_) => "--max-candidates",
                    ExplorationLimit::States(_) => "--max-states",
                };
                write!(f, "cell {cell:?}: {source} — raise it with {flag}")
            }
        }
    }
}

impl std::error::Error for ModelError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ModelError::Spec(e) => Some(e),
            ModelError::EmptyName => None,
            ModelError::DuplicateCellName { .. } => None,
            ModelError::DuplicateInput { .. } => None,
            ModelError::InputOutputClash { .. } => None,
            ModelError::InternalClash { .. } => None,
            ModelError::UnknownVar { .. } => None,
            ModelError::AsyncNotInput { .. } => None,
            ModelError::ClockNotInput { .. } => None,
            ModelError::ConstraintNotInput { .. } => None,
            ModelError::UnknownTemplateOverride { .. } => None,
            ModelError::NodeNotInternal { .. } => None,
            ModelError::UnknownNodeAlias { .. } => None,
            ModelError::DuplicateNode { .. } => None,
            ModelError::NodeClashesWithPin { .. } => None,
            ModelError::ExposeNotInternal { .. } => None,
            ModelError::DuplicateExpose { .. } => None,
            ModelError::Exploration { source, .. } => Some(source),
        }
    }
}

impl From<toml::de::Error> for ModelError {
    fn from(err: toml::de::Error) -> Self {
        ModelError::Spec(err)
    }
}

impl From<ModelError> for io::Error {
    fn from(err: ModelError) -> Self {
        io::Error::new(io::ErrorKind::InvalidData, err)
    }
}

/// A signal (output **or** internal) after analysis: its function, the variables it references, and
/// the feedback/state variables among them (a signal-name reference = a delayed/feedback value).
#[derive(Debug)]
pub struct AnalysedOutput {
    pub name: Symbol,
    /// The parsed function, regenerated from the minimised BDD when the rewrite changed it.
    /// DISPLAY-ONLY — analysis reads the shared BDD map, never this field.
    pub(crate) expr: BoolExpr,
    pub(crate) vars: BTreeSet<Symbol>,
    /// Signal names (outputs then internals) referenced by this function — its feedback/state — in
    /// the cell's signal order.
    pub(crate) feedback: Vec<Symbol>,
}

/// A cell after validation/analysis.
#[derive(Debug)]
pub struct AnalysedCell {
    pub(crate) name: Vec<Symbol>,
    pub(crate) inputs: Vec<Symbol>,
    pub outputs: Vec<AnalysedOutput>,
    /// Internal state variables: driven state signals with no external pin. Referenceable by any
    /// function; never an arc source or target. Relay/alias internals are folded away by the
    /// state-space minimisation in [`Cell::analyse`], so only genuine-memory internals survive here.
    pub(crate) internals: Vec<AnalysedOutput>,
    /// The internal nodes named in the spec's `expose` list, in declared order, carried verbatim from
    /// `Cell::expose` (validated to be declared internal signals). Some may later be folded away by the
    /// state-space minimisation — see [`AnalysedCell::exposed_signals`] for the survivors in this view.
    pub(crate) exposed: Vec<Symbol>,
    pub(crate) async_pins: Vec<Symbol>,
    /// The transition arcs derived for the cell's outputs, precomputed once by the shared machine pass
    /// ([`crate::logic::analysis::analyse_machine`]) and consumed by the arcs emitter.
    pub(crate) arcs: Vec<Arc>,
    /// The whole-cell internal-power ('hidden') arcs — single input toggles that settle but leave every
    /// output unchanged — precomputed once by the shared machine pass
    /// ([`crate::logic::analysis::analyse_machine`]) and consumed by the arcs emitter.
    pub(crate) hidden_arcs: Vec<HiddenArc>,
    /// The cell's static leakage states — one per fully-initialised reachable rest state of the machine
    /// exploration — precomputed once by the shared machine pass
    /// ([`crate::logic::analysis::analyse_machine`]) and consumed by the arcs emitter.
    pub(crate) leakage: Vec<LeakageState>,
    /// The cell's detected hazards — one [`Hazard`] per (cause, outcome) pair a probe observes: a race
    /// or a pulse, settling indeterminately or oscillating (empty for cells with no such risk). The
    /// constraints that avoid them are generated separately into `constraints`.
    /// See [`crate::logic::hazard`].
    pub hazards: Vec<Hazard>,
    /// Declared clock input pins (`clock = [...]`). See [`crate::logic::constraint`], which reads them
    /// to direct a separation, and [`crate::logic::edge`].
    pub(crate) clock_pins: Vec<Symbol>,
    /// The constraints generated to avoid the cell's detected hazards: the separations (setup/hold and
    /// non_seq) holding two pins apart, and the minimum pulse widths holding one pin against itself.
    /// Only the pins `constraint_arcs_declared` selects are represented here; each constraint's kind
    /// follows the cause of the hazard it avoids, a separation's being directed by the declared clock.
    /// See [`crate::logic::constraint`].
    pub(crate) constraints: Vec<Constraint>,
    /// Which input pins the cell asked for constraint arcs on (`constraint_arcs`, unioned with the
    /// global `--constraints` flag). Read by generation, which keeps the constraints on a selected pin
    /// and drops the rest; detection runs whatever it selects.
    pub(crate) constraint_arcs_declared: ConstraintPins,
    /// The arc classes whose `-when` arcs are also emitted (per-cell `when` unioned with the global
    /// `--when`), read by the arcs emitter. One general arc per transition — a related pin's edge driving
    /// an output pin's edge — is always emitted, without a `-when` line, regardless of this set; a
    /// selected class adds that class's `-when` arcs on top, so an arc can appear both with and without
    /// its condition. Raw carry — analysis never reads it.
    pub(crate) when: ArcClasses,
    /// Each signal's state-table regions, precomputed once and cached in `signals()` order (outputs
    /// then internals), so emitters don't rebuild the BDDs per call site.
    pub(crate) regions: Vec<crate::logic::regions::StateRegions>,
    /// The cell's behavioural edge classification ([`crate::logic::edge::EdgeArcs`]): the per-node
    /// active-edge sets (`captures`), the per-arc `-type edge` labels (`labels`) — the field the Liberate
    /// arc emitter reads to type each arc — the cell-level set of internal level master nodes folded away
    /// (`folded`), and the read-gate factorisations recognised across the cell's outputs (`derived`), which
    /// the Liberty, Verilog and state-table emitters read to render a read-gated register as its own
    /// internal node. Default (empty) when the cell opted out (`no_edge_collapse`). Computed purely from
    /// the already-explored machine — it never alters the exploration.
    pub(crate) edge: crate::logic::edge::EdgeArcs,
    /// The cell-wide characterisation-template references (delay/power/constraint) carried verbatim from
    /// the spec for the `define_cell` emitter. `None` when the cell declares no `template`. Raw carry —
    /// analysis never reads or synthesises it.
    pub(crate) template: Option<TemplateSpec>,
    /// Per-drive-strength-alias template overrides carried verbatim from the spec, keyed by an alias
    /// from `name`. Merged per-field over `template` by the `define_cell` emitter. Keys are validated
    /// against the declared names in [`Cell::analyse_signals`]; raw carry otherwise.
    ///
    /// (`nodes` below is validated in the same place and carried the same way.)
    pub(crate) template_overrides: IndexMap<Symbol, TemplateSpec>,
    /// Which netlist node each internal signal stands for, carried verbatim from the spec for the arcs
    /// emitter. Analysis never reads it: the machine is built and explored in the spec's own names, and
    /// only the Liberate artifacts are rendered in the netlist's.
    pub(crate) nodes: NodeNames,
    /// The voltage expressions the Liberate arcs' `-ic` renders for the two logic levels, resolved from
    /// the cell's `logic_low`/`logic_high` overrides (falling back to [`LogicVoltages::default`]). Raw
    /// carry — analysis never reads it, like `template`.
    pub(crate) voltages: LogicVoltages,
    /// The arc view of this cell: the same cell analysed with its exposed nodes preserved as model
    /// coordinates, so an arc can drive one (`-ic`) and observe it (`-vector`). `None` means the model
    /// view IS the arc view, which is how [`AnalysedCell::arc_view`] reads it. Presence follows
    /// exposure: the single write site is the branch of `Cell::analyse` a cell reaches only by exposing
    /// something, and that branch analyses the view with the exposure already applied, so the view set
    /// here never carries a further view of its own.
    ///
    /// Exposure is arcs-only. The cell holding this field is the MODEL view — minimised to the outputs
    /// alone, exactly as a cell that exposes nothing — and it is the one the Liberty, Verilog,
    /// statetable and `define_cell` emitters render, so those artifacts are unaffected by what a cell
    /// exposes. Only the Liberate arc emitter reads the view here, through
    /// [`AnalysedCell::arc_view`].
    ///
    /// The cell explores once, in the arc view; the model view's machine is that same exploration
    /// projected onto the coordinates surviving the outputs-only minimisation
    /// ([`crate::logic::analysis::Exploration`]). The two views' derivations then differ because their
    /// machines carry different coordinates: the arc view's `arcs`, `hidden_arcs` and `edge.labels` are
    /// keyed on ITS `arc.start` minterms — carrying a column per exposed node, which is what the arcs
    /// emitter's arc identity reads — while the model view's own derivations feed
    /// [`crate::emit::liberty`], [`crate::emit::statetable`] and [`crate::emit::verilog`] over the
    /// minimised coordinates. The model view's machine sees only the exposed nodes that survived its own
    /// minimisation ([`AnalysedCell::exposed_signals`], empty in the usual case where the release retires
    /// them), which costs it nothing: it never emits an exposure.
    pub(crate) exposed_view: Option<Box<AnalysedCell>>,
}

impl AnalysedCell {
    /// The representative (first-as-written) cell name, for single-name contexts (diagnostics and the
    /// still-single-name emitter paths). Safe to index: `de_name_list` rejects empty name lists.
    pub fn repr_name(&self) -> &Symbol {
        &self.name[0]
    }

    /// Every state-bearing signal: outputs first, then internals, in declaration order.
    pub fn signals(&self) -> impl Iterator<Item = &AnalysedOutput> {
        self.outputs.iter().chain(self.internals.iter())
    }

    /// Whether the cell holds state: one of its signals is a state variable, a signal on a dependency
    /// cycle. That classification is already in the `regions` cache — [`crate::logic::regions`] marks a
    /// signal's region view `hysteretic` exactly when
    /// [`crate::logic::resolve::state_variables`] names it — and the cache holds one entry per signal,
    /// so a hysteretic entry exists exactly when that set is non-empty.
    ///
    /// The Liberate arc emitter gates the `-ic` initial condition on this: no block renders a walk, so
    /// `-ic` is the whole of how a cell with memory is told what state its measured vector begins in. A
    /// combinational cell has no state to establish and gets no `-ic`.
    pub(crate) fn state_holding(&self) -> bool {
        self.regions.iter().any(|r| r.hysteretic)
    }

    /// The view the Liberate arcs are emitted from: [`AnalysedCell::exposed_view`] where the cell
    /// exposes internal nodes, else the cell itself. Every other emitter reads the cell directly.
    pub fn arc_view(&self) -> &AnalysedCell {
        self.exposed_view.as_deref().unwrap_or(self)
    }

    /// The exposed nodes surviving in this view, in declared order. `exposed` keeps every declared
    /// node (they are preserved through the state-space minimisation), while this keeps only those that
    /// also survive the outputs-only minimisation reflected in `internals`.
    pub(crate) fn exposed_signals(&self) -> impl Iterator<Item = &Symbol> {
        self.exposed
            .iter()
            .filter(|e| self.internals.iter().any(|s| s.name == **e))
    }

    /// Each signal paired with its cached state-table regions, in `signals()` order (outputs then
    /// internals).
    pub(crate) fn signal_regions(
        &self,
    ) -> impl Iterator<Item = (&AnalysedOutput, &crate::logic::regions::StateRegions)> {
        self.signals().zip(self.regions.iter())
    }
}

impl Spec {
    /// Validate cross-cell name uniqueness, then analyse every cell under `budget`.
    ///
    /// The union of all cells' name lists must contain no name twice: a collision would emit duplicate
    /// Liberty `cell()` groups, duplicate Verilog modules and conflicting `define_arc` trailers. Intra-cell
    /// duplicates are already deduped by `de_name_list`, so the set-insert here catches inter-cell
    /// collisions (an alias colliding with another cell's name included). The per-cell analyses then run
    /// in parallel, matching the single machine pass minted per cell in [`Cell::analyse`]. `budget` is the
    /// CLI's `--max-candidates` / `--max-states` ceilings, applied to every cell alike.
    pub fn analyse_with(
        &self,
        budget: &ExplorationBudget,
    ) -> Result<Vec<AnalysedCell>, ModelError> {
        let mut seen: BTreeSet<Symbol> = BTreeSet::new();
        for cell in &self.cells {
            for name in &cell.name {
                if !seen.insert(name.clone()) {
                    return Err(ModelError::DuplicateCellName { name: name.clone() });
                }
            }
        }
        self.cells
            .par_iter()
            .map(|c| c.analyse_with(budget))
            .collect()
    }
}

impl Cell {
    /// Validate the cell and parse its functions, classifying each referenced variable as a primary
    /// input, an output, or an internal signal (feedback/state = a signal-name reference). The machine
    /// is explored under the default [`ExplorationBudget`].
    pub fn analyse(&self) -> Result<AnalysedCell, ModelError> {
        self.analyse_with(&ExplorationBudget::default())
    }

    /// [`Cell::analyse`] with an explicit exploration budget: the ceilings bound the machine pass, and
    /// an exploration stopped by one of them fails the analysis with [`ModelError::Exploration`].
    pub(crate) fn analyse_with(
        &self,
        budget: &ExplorationBudget,
    ) -> Result<AnalysedCell, ModelError> {
        let analysed = self.analyse_signals()?;
        // `machine::explore` reports which ceiling stopped it and nothing about whose exploration it
        // was; this is the layer holding the cell, so it names it.
        let stopped = |source| ModelError::Exploration {
            cell: self.name[0].clone(),
            source,
        };

        // One-shot state-space rewrite: mint the cell's single builder, build every signal's BDD once,
        // and run the minimisation (identical-δ dedup + guarded relay/alias fold, alternated until
        // neither pass commits). It rewrites the map in place so every surviving signal is a genuine-memory
        // coordinate; the same map is then shared by the machine pass, the region cache and emission —
        // no signal function is ever rebuilt.
        let builder = sync_bdd_builder!();
        let mut bdds = build_signal_bdds(&analysed, &builder);
        let order: Vec<Symbol> = analysed.signals().map(|s| s.name.clone()).collect();
        let outputs: BTreeSet<Symbol> = analysed.outputs.iter().map(|o| o.name.clone()).collect();
        // An exposed node must keep its name to be addressable by the arcs, so it joins the outputs in
        // the set the minimisation may not remove.
        let preserved = if self.expose.is_empty() {
            crate::logic::minimise::Preserved::outputs(outputs.clone())
        } else {
            crate::logic::minimise::Preserved::with_exposed(
                outputs.clone(),
                self.expose.iter().cloned().collect(),
            )
        };
        let min = crate::logic::minimise::minimise_state_space(&mut bdds, &order, &preserved);

        // Nothing exposed: the minimised map already IS the model, and the cell carries a single view.
        if self.expose.is_empty() {
            return Ok(self
                .finish_view(analysed, &bdds, &min, Exploration::Fresh(budget))
                .map_err(stopped)?
                .view);
        }

        // The exposed nodes survived the run above, so this view carries them as machine coordinates:
        // the arc view (see `AnalysedCell::exposed_view`). Only plain data — arcs, hazards, regions,
        // the explored states — comes back out of the map, so the map is free to be minimised further
        // beneath it. This is the cell's one exploration; the model view below reads it.
        let FinishedView {
            view: arc_view,
            explored,
        } = self
            .finish_view(analysed, &bdds, &min, Exploration::Fresh(budget))
            .map_err(stopped)?;

        // Release the exposure and carry the SAME map on to the outputs-only minimised model. The
        // composition reaches the reduced system a single outputs-only run reaches, save for which
        // member of a collapsed group of equal-valued coordinates supplies the surviving name:
        // protecting a member through the first run leaves that one the representative, and the second
        // run has no remaining member of the group to reconsider. Which name survives carries no
        // meaning — the group holds one value. The model view is re-derived from a fresh parse against
        // the twice-minimised map, under the composition of the two runs, so its display expressions
        // are those of an exposure-free analysis.
        let released = crate::logic::minimise::minimise_state_space(
            &mut bdds,
            &order,
            &crate::logic::minimise::Preserved::outputs(outputs),
        );
        // The model view reads the arc view's exploration, carried onto its own coordinates rather than
        // discovered again. A ceiling that stopped that one exploration already returned above, so the
        // states are here to be reused.
        let reused = Exploration::Reused(
            explored
                .as_ref()
                .expect("a fresh exploration that returned hands its states back"),
        );
        let FinishedView {
            view: mut model, ..
        } = self
            .finish_view(self.analyse_signals()?, &bdds, &min.then(released), reused)
            .map_err(stopped)?;
        debug_assert!(
            arc_view.exposed_view.is_none(),
            "the arc view is analysed with the exposure already applied and never carries a view of its own"
        );
        model.exposed_view = Some(Box::new(arc_view));
        Ok(model)
    }

    /// Complete one view of the cell over `bdds` — the minimised map `min` reports the rewrite of, back
    /// to `view`'s parse-time functions. Recomputes each surviving signal's metadata, builds the machine
    /// over `exploration` and copies its derivations in, then caches the region view.
    ///
    /// This is everything downstream of the minimisation, and it runs once PER VIEW over that view's own
    /// signals: a cell that exposes internal nodes carries two of them (see
    /// [`AnalysedCell::exposed_view`]), whose machines differ in their coordinates. The exploration
    /// itself is per CELL — the second element of the result is the states this view explored, for the
    /// other view to project onto its own coordinates, and is `None` when this view reused an
    /// exploration. A budget ceiling that stops the exploration is the error, cell-free: the caller
    /// names the cell.
    fn finish_view<B: Brand, C: ManagerCell + Send + Sync>(
        &self,
        mut view: AnalysedCell,
        bdds: &BTreeMap<Symbol, Bdd<B, C>>,
        min: &crate::logic::minimise::Minimised,
        exploration: Exploration<'_>,
    ) -> Result<FinishedView, ExplorationLimit> {
        recompute_signal_metadata(&mut view, bdds, min);

        // Build the cell's state machine once and derive both its transition arcs and its hazards from
        // the shared exploration over the minimised model: the detected hazards — one per (cause,
        // outcome) pair — and the constraints — setup/hold, non_seq, min_pulse_width — generated to
        // avoid them. Clock suppression and emission gating are applied downstream.
        // The opt-out (`no_edge_collapse`, also set for every cell by the global `--no-edge-collapse`)
        // gates the classify() call itself, not just its result — no wasted work when collapse is off.
        let Derivations {
            arcs,
            hidden_arcs,
            constraints,
            hazards,
            leakage,
            edge,
            explored,
        } = crate::logic::analysis::analyse_machine(
            &view,
            bdds,
            !self.no_edge_collapse,
            exploration,
        )?;
        view.arcs = arcs;
        view.hidden_arcs = hidden_arcs;
        view.leakage = leakage;
        view.constraints = constraints;
        view.hazards = hazards;
        view.edge = edge;

        // Cache each signal's state-table regions once, in `signals()` order, from the shared folded
        // BDDs, so downstream emitters don't rebuild the BDDs per call site. Whether the minimised model
        // holds any state at all is read back off this cache by `AnalysedCell::state_holding`, the
        // regions carrying the same cyclic classifier's verdict per signal.
        view.regions = derive_regions(&view, bdds);

        Ok(FinishedView { view, explored })
    }

    /// Validate the cell and parse its functions into the pre-minimise [`AnalysedCell`]: every signal's
    /// parse-time support and feedback classification, with all derived analysis fields
    /// (arcs/hidden_arcs/leakage/hazards/constraints/regions)
    /// still empty. The state-space rewrite and machine/region passes are layered on by
    /// [`Cell::analyse`].
    pub fn analyse_signals(&self) -> Result<AnalysedCell, ModelError> {
        // Programmatic guard: `de_name_list` rejects an empty name list on deserialisation, but `Cell`
        // has all-pub fields, so a hand-built `Cell { name: vec![], .. }` bypasses it and would panic on
        // the pervasive `self.name[0]` / `repr_name()` indexing. Reject it here, before the first index.
        if self.name.is_empty() {
            return Err(ModelError::EmptyName);
        }

        let mut input_set = BTreeSet::new();
        for pin in &self.inputs {
            if !input_set.insert(pin.clone()) {
                return Err(ModelError::DuplicateInput {
                    cell: self.name[0].clone(),
                    pin: pin.clone(),
                });
            }
        }

        let output_names: Vec<Symbol> = self.outputs.keys().cloned().collect();
        let output_set: BTreeSet<Symbol> = output_names.iter().cloned().collect();
        let internal_names: Vec<Symbol> = self.internal.keys().cloned().collect();
        let internal_set: BTreeSet<Symbol> = internal_names.iter().cloned().collect();

        for pin in &self.inputs {
            if output_set.contains(pin) {
                return Err(ModelError::InputOutputClash {
                    cell: self.name[0].clone(),
                    pin: pin.clone(),
                });
            }
        }
        for name in &internal_names {
            if input_set.contains(name) || output_set.contains(name) {
                return Err(ModelError::InternalClash {
                    cell: self.name[0].clone(),
                    pin: name.clone(),
                });
            }
        }
        for pin in &self.async_pins {
            if !input_set.contains(pin) {
                return Err(ModelError::AsyncNotInput {
                    cell: self.name[0].clone(),
                    pin: pin.clone(),
                });
            }
        }
        for pin in &self.clock {
            if !input_set.contains(pin) {
                return Err(ModelError::ClockNotInput {
                    cell: self.name[0].clone(),
                    pin: pin.clone(),
                });
            }
        }
        // A constraint constrains an input: a block names it on `-pin`, and an output or an internal
        // node is not something a run can hold to a timing. So a selection naming one is a spec error,
        // not a selection that quietly matches nothing.
        for pin in self.constraint_arcs.named() {
            if !input_set.contains(pin) {
                return Err(ModelError::ConstraintNotInput {
                    cell: self.name[0].clone(),
                    pin: pin.clone(),
                });
            }
        }

        // Every template-override key must name one of this cell's (de_name_list-deduped) drive-strength
        // aliases. Iterating in insertion order keeps the reported error deterministic.
        let name_set: BTreeSet<Symbol> = self.name.iter().cloned().collect();
        for alias in self.template_overrides.keys() {
            if !name_set.contains(alias) {
                return Err(ModelError::UnknownTemplateOverride {
                    cell: self.name[0].clone(),
                    alias: alias.clone(),
                });
            }
        }

        // Every `[cell.nodes]` entry must name a declared internal, and every per-alias table a declared
        // cell name. A mistyped key would otherwise map nothing and hand Liberate the spec's own name,
        // which reads as a working spec right up to characterisation.
        for signal in self.nodes.cell.keys() {
            if !internal_set.contains(signal) {
                return Err(ModelError::NodeNotInternal {
                    cell: self.name[0].clone(),
                    signal: signal.clone(),
                });
            }
        }
        for (alias, map) in &self.nodes.aliases {
            if !name_set.contains(alias) {
                return Err(ModelError::UnknownNodeAlias {
                    cell: self.name[0].clone(),
                    alias: alias.clone(),
                });
            }
            for signal in map.keys() {
                if !internal_set.contains(signal) {
                    return Err(ModelError::NodeNotInternal {
                        cell: self.name[0].clone(),
                        signal: signal.clone(),
                    });
                }
            }
        }

        // Every exposed node must be a declared internal signal, checked in declaration order against a
        // running set so a duplicate is caught deterministically.
        let mut expose_seen: BTreeSet<Symbol> = BTreeSet::new();
        for node in &self.expose {
            if !internal_set.contains(node) {
                return Err(ModelError::ExposeNotInternal {
                    cell: self.name[0].clone(),
                    node: node.clone(),
                });
            }
            if !expose_seen.insert(node.clone()) {
                return Err(ModelError::DuplicateExpose {
                    cell: self.name[0].clone(),
                    node: node.clone(),
                });
            }
        }

        // ONE NODE, ONE NAME. A netlist holds each signal on a node of its own, and a signal that sits
        // on a pin's net IS that pin, so what the internals resolve to must be distinct from each other
        // and from every pin the cell declares. The rule is the netlist's, and it is checked per drive
        // strength, each having its own map and so its own names.
        //
        // Every declared internal is checked, mapped or not: an unmapped one stands for itself, which
        // is a name another may not be mapped onto. This is also what keeps the emitted columns
        // straight — `-vector` and `-ic` are positional against `-pinlist`, so two columns under one
        // name shift every column after them — but the columns are the consequence, not the rule.
        // Which internals earn one is not even known here: a constraint arc gives each victim node a
        // column of its own, and where the hazards are is settled only by exploring the machine.
        let pin_set: BTreeSet<Symbol> = self
            .inputs
            .iter()
            .chain(self.outputs.keys())
            .cloned()
            .collect();
        for alias in &self.name {
            let mut resolved_seen: BTreeSet<Symbol> = BTreeSet::new();
            for node in self.internal.keys() {
                let resolved = self.nodes.of(alias, node);
                if pin_set.contains(&resolved) {
                    return Err(ModelError::NodeClashesWithPin {
                        cell: self.name[0].clone(),
                        alias: alias.clone(),
                        node: resolved,
                    });
                }
                if !resolved_seen.insert(resolved.clone()) {
                    return Err(ModelError::DuplicateNode {
                        cell: self.name[0].clone(),
                        alias: alias.clone(),
                        node: resolved,
                    });
                }
            }
        }

        // A `--logic-low`/`--logic-high` value reaches analysis as a spec key: `main` folds the
        // command-line defaults into the cell's own keys, which win where both are set.
        let voltages =
            LogicVoltages::from_options(self.logic_low.as_deref(), self.logic_high.as_deref());

        // Signal order: outputs first, then internals. Feedback references are classified against it.
        let signal_names: Vec<Symbol> = output_names
            .iter()
            .cloned()
            .chain(internal_names.iter().cloned())
            .collect();

        // Every function (outputs then internals) is already parsed into a `BoolExpr` at deserialise
        // time; classify its support here into one signal list.
        let n_outputs = self.outputs.len();
        let mut all: Vec<AnalysedOutput> = Vec::with_capacity(n_outputs + self.internal.len());
        for (name, func) in self.outputs.iter().chain(self.internal.iter()) {
            let vars: BTreeSet<Symbol> = func.variables().collect();
            for v in &vars {
                if !input_set.contains(v) && !output_set.contains(v) && !internal_set.contains(v) {
                    return Err(ModelError::UnknownVar {
                        cell: self.name[0].clone(),
                        output: name.clone(),
                        var: v.clone(),
                    });
                }
            }
            let feedback: Vec<Symbol> = signal_names
                .iter()
                .filter(|s| vars.contains(*s))
                .cloned()
                .collect();
            all.push(AnalysedOutput {
                name: name.clone(),
                expr: func.clone(),
                vars,
                feedback,
            });
        }

        let internals = all.split_off(n_outputs);
        let outputs = all;

        let analysed = AnalysedCell {
            name: self.name.clone(),
            inputs: self.inputs.clone(),
            outputs,
            internals,
            exposed: self.expose.clone(),
            async_pins: self.async_pins.clone(),
            arcs: Vec::new(),
            hidden_arcs: Vec::new(),
            leakage: Vec::new(),
            hazards: Vec::new(),
            clock_pins: self.clock.clone(),
            constraints: Vec::new(),
            constraint_arcs_declared: self.constraint_arcs.clone(),
            when: self.when,
            regions: Vec::new(),
            edge: Default::default(),
            template: self.template.clone(),
            template_overrides: self.template_overrides.clone(),
            nodes: self.nodes.clone(),
            voltages,
            exposed_view: None,
        };
        Ok(analysed)
    }
}

/// The result of [`Cell::finish_view`]: the finished view, and the states this view explored for the
/// other view to project — `None` when it reused an exploration.
struct FinishedView {
    view: AnalysedCell,
    explored: Option<Explored>,
}

/// Build every signal's BDD once from the shared per-cell `builder`, keyed by signal name in
/// `signals()` order (outputs then internals).
///
/// Pure over `signals()`/`expr`, so it yields the same map whether `cell` is pre-minimise (parse-time
/// functions) or post-minimise (folded functions) — the caller re-derives from whichever `expr` each
/// signal currently holds. The builder is minted exactly once per cell in [`Cell::analyse`].
pub fn build_signal_bdds<B: Brand, C: ManagerCell>(
    cell: &AnalysedCell,
    builder: &BddBuilder<B, C>,
) -> BTreeMap<Symbol, Bdd<B, C>> {
    cell.signals()
        .map(|s| (s.name.clone(), builder.build(&s.expr)))
        .collect()
}

/// Recompute each surviving signal's metadata from the minimised `bdds` after
/// [`crate::logic::minimise::minimise_state_space`].
///
/// First drop the internals the fold purged (outputs are never purged), then recompute every surviving
/// signal from its folded BDD: its support (now semantic, not the parse-time syntactic support) and the
/// feedback/state references among the survivors. The display expression is regenerated only when the
/// rewrite actually changed the function.
pub(crate) fn recompute_signal_metadata<B: Brand, C: ManagerCell>(
    cell: &mut AnalysedCell,
    bdds: &BTreeMap<Symbol, Bdd<B, C>>,
    min: &crate::logic::minimise::Minimised,
) {
    cell.internals.retain(|s| !min.purged.contains(&s.name));

    let surviving: Vec<Symbol> = cell.signals().map(|s| s.name.clone()).collect();
    for sig in cell.outputs.iter_mut().chain(cell.internals.iter_mut()) {
        sig.vars = bdds[&sig.name].variables().collect();
        sig.feedback = surviving
            .iter()
            .filter(|n| sig.vars.contains(n.as_str()))
            .cloned()
            .collect();
        if min.changed.contains(&sig.name) {
            sig.expr = bdds[&sig.name].to_expr();
        }
    }
}

/// Derive each signal's state-table regions from the shared folded `bdds`, in `signals()` order
/// (outputs then internals).
///
/// The cyclic state-variable set (over the recomputed feedback) decides each region's `hysteretic`
/// flag — a state variable must emit a `statetable`, never a combinational `function`. This is the
/// cheap pure-graph classifier, so it holds even for cells the machine-width guard skips. Returns the
/// region vector; it does not mutate `cell`.
pub fn derive_regions<B: Brand, C: ManagerCell>(
    cell: &AnalysedCell,
    bdds: &BTreeMap<Symbol, Bdd<B, C>>,
) -> Vec<crate::logic::regions::StateRegions> {
    let signals: Vec<&AnalysedOutput> = cell.signals().collect();
    let state_set = crate::logic::resolve::state_variables(&signals);
    cell.signals()
        .map(|s| {
            crate::logic::regions::state_regions(
                &s.name,
                &bdds[&s.name],
                state_set.contains(&s.name),
            )
        })
        .collect()
}

/// Parse a TOML spec into a [`Spec`].
pub fn parse_spec(toml_src: &str) -> Result<Spec, ModelError> {
    Ok(toml::from_str(toml_src)?)
}

/// Parse a single-cell TOML `src` and return its analysed form. The one canonical test helper, shared
/// by the in-crate `#[cfg(test)]` modules.
#[cfg(test)]
pub(crate) fn analyse_one(src: &str) -> AnalysedCell {
    parse_spec(src).unwrap().cells.remove(0).analyse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::emit::arcs_tcl::{cell_arcs, ArcsTclOptions, Deck};
    use crate::logic::arcs::{HeldLevel, PinEdge};
    use crate::logic::constraint::ConstraintKind;
    use crate::logic::hazard::{Cause, Outcome};
    use espresso_logic::{bdd_builder, expr, Minterm};

    const SAMPLE: &str = r#"
[[cell]]
name = "C2"
inputs = ["A", "B"]
[cell.outputs]
Q = "A*B + Q*(A+B)"

[[cell]]
name = "INV"
inputs = ["A"]
[cell.outputs]
Y = "!A"
"#;

    #[test]
    fn loads_and_classifies_feedback() {
        let spec = parse_spec(SAMPLE).unwrap();
        assert_eq!(spec.cells.len(), 2);

        let c2 = spec.cells[0].analyse().unwrap();
        assert_eq!(c2.name[0], "C2");
        assert_eq!(c2.inputs, ["A", "B"]);
        assert_eq!(c2.outputs.len(), 1);
        assert_eq!(c2.outputs[0].feedback, ["Q"]); // Q references itself => feedback/state

        let inv = spec.cells[1].analyse().unwrap();
        assert!(inv.outputs[0].feedback.is_empty()); // purely combinational
    }

    #[test]
    fn preserves_output_order() {
        let s = r#"
[[cell]]
name = "SR"
inputs = ["S", "R"]
[cell.outputs]
Q = "S + Q*!R"
Qn = "R + Qn*!S"
"#;
        let cell = parse_spec(s).unwrap().cells.remove(0).analyse().unwrap();
        let names: Vec<_> = cell.outputs.iter().map(|o| o.name.as_str()).collect();
        assert_eq!(names, ["Q", "Qn"]);
    }

    #[test]
    fn rejects_unknown_var() {
        let s = r#"
[[cell]]
name = "X"
inputs = ["A"]
[cell.outputs]
Y = "A*Z"
"#;
        let err = parse_spec(s).unwrap().cells[0].analyse().unwrap_err();
        assert!(matches!(err, ModelError::UnknownVar { .. }));
    }

    #[test]
    fn rejects_unknown_var_in_internal() {
        // An undefined variable is rejected wherever it appears — an internal function, not just an output.
        let s = r#"
[[cell]]
name = "X"
inputs = ["A"]
[cell.internal]
W = "A*Z"
[cell.outputs]
Y = "W"
"#;
        let err = parse_spec(s).unwrap().cells[0].analyse().unwrap_err();
        assert!(matches!(err, ModelError::UnknownVar { var, .. } if var == "Z"));
    }

    #[test]
    fn multiple_errors_report_the_first_deterministically() {
        // Two outputs each reference an undefined variable. Analysis short-circuits on the first in a
        // fixed traversal order (outputs in declaration order), so the reported error is stable across
        // repeated parses — never dependent on hash-map iteration.
        let s = r#"
[[cell]]
name = "MULTI"
inputs = ["A"]
[cell.outputs]
Y1 = "A*Z1"
Y2 = "A*Z2"
"#;
        let first = parse_spec(s).unwrap().cells[0]
            .analyse()
            .unwrap_err()
            .to_string();
        for _ in 0..8 {
            let again = parse_spec(s).unwrap().cells[0]
                .analyse()
                .unwrap_err()
                .to_string();
            assert_eq!(again, first, "error reporting must be deterministic");
        }
        assert!(
            first.contains("Z1") && !first.contains("Z2"),
            "the first-declared offending output is reported first: {first}",
        );
    }

    #[test]
    fn rejects_unknown_cell_key() {
        // A misspelt or stale spec key must be a hard error, not silently ignored.
        let s = r#"
[[cell]]
name = "X"
inputs = ["A"]
oscillate = ["Q"]
[cell.outputs]
Y = "A"
"#;
        assert!(matches!(parse_spec(s), Err(ModelError::Spec(_))));
    }

    /// `ALL` names its classes one by one, so it is checked against `ValueEnum`'s derived list of
    /// every variant.
    #[test]
    fn all_selects_every_arc_class() {
        for class in <ArcClass as clap::ValueEnum>::value_variants() {
            assert!(ArcClasses::ALL.contains(*class), "{class:?} is not in ALL");
        }
    }

    #[test]
    fn when_absent_is_the_empty_set() {
        let s = r#"
[[cell]]
name = "X"
inputs = ["A"]
[cell.outputs]
Y = "A"
"#;
        let spec = parse_spec(s).unwrap();
        assert_eq!(spec.cells[0].when, ArcClasses::default());
    }

    #[test]
    fn when_true_selects_every_class() {
        let s = r#"
[[cell]]
name = "X"
inputs = ["A"]
when = true
[cell.outputs]
Y = "A"
"#;
        let spec = parse_spec(s).unwrap();
        assert_eq!(spec.cells[0].when, ArcClasses::ALL);
    }

    #[test]
    fn when_false_is_the_empty_set() {
        let s = r#"
[[cell]]
name = "X"
inputs = ["A"]
when = false
[cell.outputs]
Y = "A"
"#;
        let spec = parse_spec(s).unwrap();
        assert_eq!(spec.cells[0].when, ArcClasses::default());
    }

    #[test]
    fn when_scalar_selects_only_that_class() {
        let s = r#"
[[cell]]
name = "X"
inputs = ["A"]
when = "hidden"
[cell.outputs]
Y = "A"
"#;
        let spec = parse_spec(s).unwrap();
        let when = spec.cells[0].when;
        assert!(when.contains(ArcClass::Hidden));
        assert!(!when.contains(ArcClass::Transition));
    }

    #[test]
    fn when_list_selects_every_named_class() {
        let s = r#"
[[cell]]
name = "X"
inputs = ["A"]
when = ["hidden", "transition", "constraint"]
[cell.outputs]
Y = "A"
"#;
        let spec = parse_spec(s).unwrap();
        assert_eq!(spec.cells[0].when, ArcClasses::ALL);
    }

    #[test]
    fn when_rejects_an_unknown_class() {
        let s = r#"
[[cell]]
name = "X"
inputs = ["A"]
when = "propagation"
[cell.outputs]
Y = "A"
"#;
        let err = parse_spec(s).unwrap_err();
        assert!(matches!(err, ModelError::Spec(_)));
        assert!(
            err.to_string().contains("unknown when arc class"),
            "unexpected error: {err}",
        );
    }

    /// A two-input cell carrying `constraint_arcs = {selection}`, or nothing where the selection is
    /// empty.
    fn constraint_spec(selection: &str) -> Spec {
        let s = format!(
            r#"
[[cell]]
name = "X"
inputs = ["A", "B"]
{selection}[cell.internal]
M = "A*B"
[cell.outputs]
Y = "M + A"
"#
        );
        parse_spec(&s).unwrap()
    }

    #[test]
    fn constraint_arcs_absent_selects_no_pin() {
        assert_eq!(
            constraint_spec("").cells[0].constraint_arcs,
            ConstraintPins::Off
        );
    }

    #[test]
    fn constraint_arcs_true_selects_every_pin() {
        let spec = constraint_spec("constraint_arcs = true\n");
        assert_eq!(spec.cells[0].constraint_arcs, ConstraintPins::All);
        assert!(spec.cells[0].constraint_arcs.selects(&Symbol::from("A")));
    }

    #[test]
    fn constraint_arcs_false_selects_no_pin() {
        let spec = constraint_spec("constraint_arcs = false\n");
        assert_eq!(spec.cells[0].constraint_arcs, ConstraintPins::Off);
        assert!(!spec.cells[0].constraint_arcs.selects(&Symbol::from("A")));
    }

    #[test]
    fn constraint_arcs_scalar_names_one_pin() {
        let selection = constraint_spec("constraint_arcs = \"A\"\n")
            .cells
            .remove(0)
            .constraint_arcs;
        assert_eq!(selection, ConstraintPins::Named(vec![Symbol::from("A")]));
        assert!(selection.selects(&Symbol::from("A")));
        assert!(!selection.selects(&Symbol::from("B")));
    }

    #[test]
    fn constraint_arcs_list_names_every_pin_in_it() {
        let selection = constraint_spec("constraint_arcs = [\"B\", \"A\"]\n")
            .cells
            .remove(0)
            .constraint_arcs;
        assert_eq!(
            selection,
            ConstraintPins::Named(vec![Symbol::from("B"), Symbol::from("A")]),
        );
        assert!(selection.selects(&Symbol::from("A")));
        assert!(selection.selects(&Symbol::from("B")));
    }

    #[test]
    fn constraint_arcs_rejects_an_output_pin() {
        // A constraint constrains an input: `Y` is an output, so naming it is a spec error rather than a
        // selection that matches nothing.
        let err = constraint_spec("constraint_arcs = [\"A\", \"Y\"]\n")
            .analyse_with(&ExplorationBudget::default())
            .unwrap_err();
        assert!(
            matches!(&err, ModelError::ConstraintNotInput { cell, pin } if cell == "X" && pin == "Y"),
            "unexpected error: {err}",
        );
    }

    #[test]
    fn constraint_arcs_rejects_an_internal_signal() {
        // `M` is an internal node, which no block can name on `-pin` either.
        let err = constraint_spec("constraint_arcs = \"M\"\n")
            .analyse_with(&ExplorationBudget::default())
            .unwrap_err();
        assert!(
            matches!(&err, ModelError::ConstraintNotInput { cell, pin } if cell == "X" && pin == "M"),
            "unexpected error: {err}",
        );
    }

    #[test]
    fn internal_signal_is_classified_and_kept_off_the_output_list() {
        // A DFF: internal master latch M, external slave output Q referencing M.
        let s = r#"
[[cell]]
name = "DFF"
inputs = ["CLK", "D"]
[cell.internal]
M = "!CLK*D + CLK*M"
[cell.outputs]
Q = "CLK*M + !CLK*Q"
"#;
        let cell = parse_spec(s).unwrap().cells.remove(0).analyse().unwrap();
        // M is internal, not an output.
        let out_names: Vec<_> = cell.outputs.iter().map(|o| o.name.as_str()).collect();
        assert_eq!(out_names, ["Q"]);
        let int_names: Vec<_> = cell.internals.iter().map(|o| o.name.as_str()).collect();
        assert_eq!(int_names, ["M"]);
        // Q references the internal M as feedback/state.
        assert!(cell.outputs[0].feedback.iter().any(|s| s == "M"));
        assert!(cell.outputs[0].feedback.iter().any(|s| s == "Q"));
        // signals() yields outputs then internals.
        let sig_names: Vec<_> = cell.signals().map(|s| s.name.as_str()).collect();
        assert_eq!(sig_names, ["Q", "M"]);
        // No oscillation hazard (Q→M is a one-way dependency, no mutual cycle).
        assert!(!cell.hazards.iter().any(|h| {
            matches!(h.cause, Cause::Toggle { .. } | Cause::Race { .. })
                && h.outcome == Outcome::Oscillation
        }));
    }

    #[test]
    fn internal_referenced_by_function_is_a_known_var() {
        // An internal name used in an output function must not be rejected as UnknownVar.
        let s = r#"
[[cell]]
name = "X"
inputs = ["A"]
[cell.internal]
W = "A"
[cell.outputs]
Y = "W"
"#;
        assert!(parse_spec(s).unwrap().cells[0].analyse().is_ok());
    }

    #[test]
    fn internal_clashing_with_output_errors() {
        let s = r#"
[[cell]]
name = "X"
inputs = ["A"]
[cell.internal]
Q = "A"
[cell.outputs]
Q = "A + Q"
"#;
        let err = parse_spec(s).unwrap().cells[0].analyse().unwrap_err();
        assert!(matches!(err, ModelError::InternalClash { .. }));
    }

    #[test]
    fn name_scalar_parses_as_single_entry() {
        let s = r#"
[[cell]]
name = "INV"
inputs = ["A"]
[cell.outputs]
Y = "!A"
"#;
        let cell = parse_spec(s).unwrap().cells.remove(0).analyse().unwrap();
        assert_eq!(cell.name, vec![Symbol::from("INV")]);
    }

    #[test]
    fn name_list_preserves_written_order() {
        let s = r#"
[[cell]]
name = ["INVX1", "INVX2"]
inputs = ["A"]
[cell.outputs]
Y = "!A"
"#;
        let cell = parse_spec(s).unwrap().cells.remove(0).analyse().unwrap();
        assert_eq!(
            cell.name,
            vec![Symbol::from("INVX1"), Symbol::from("INVX2")]
        );
    }

    #[test]
    fn name_list_dedups_preserving_order() {
        let s = r#"
[[cell]]
name = ["A", "A", "B"]
inputs = ["I"]
[cell.outputs]
Y = "I"
"#;
        let cell = parse_spec(s).unwrap().cells.remove(0).analyse().unwrap();
        assert_eq!(cell.name, vec![Symbol::from("A"), Symbol::from("B")]);
    }

    #[test]
    fn empty_name_list_is_rejected() {
        let s = r#"
[[cell]]
name = []
inputs = ["A"]
[cell.outputs]
Y = "A"
"#;
        let err = parse_spec(s).unwrap_err().to_string();
        assert!(
            err.contains("cell name list must be non-empty"),
            "unexpected error: {err}",
        );
    }

    #[test]
    fn duplicate_cell_name_across_cells_is_rejected() {
        // The same physical name declared by two cells would emit duplicate Liberty/Verilog groups.
        let s = r#"
[[cell]]
name = "DUP"
inputs = ["A"]
[cell.outputs]
Y = "A"

[[cell]]
name = "DUP"
inputs = ["A"]
[cell.outputs]
Z = "A"
"#;
        let err = parse_spec(s)
            .unwrap()
            .analyse_with(&ExplorationBudget::default())
            .unwrap_err();
        assert!(matches!(err, ModelError::DuplicateCellName { name } if name == "DUP"));
    }

    #[test]
    fn alias_colliding_with_another_cell_name_is_rejected() {
        // An alias in one cell's list colliding with a second cell's scalar name is still a collision.
        let s = r#"
[[cell]]
name = ["FOO", "BAR"]
inputs = ["A"]
[cell.outputs]
Y = "A"

[[cell]]
name = "BAR"
inputs = ["A"]
[cell.outputs]
Z = "A"
"#;
        let err = parse_spec(s)
            .unwrap()
            .analyse_with(&ExplorationBudget::default())
            .unwrap_err();
        assert!(matches!(err, ModelError::DuplicateCellName { name } if name == "BAR"));
    }

    #[test]
    fn programmatic_empty_name_errors_instead_of_panicking() {
        // `Cell` has all-pub fields, so a hand-built empty name list bypasses `de_name_list` and would
        // otherwise panic on `self.name[0]`. The guard returns `EmptyName` rather than panicking.
        let mut outputs = IndexMap::new();
        outputs.insert(Symbol::from("Y"), BoolExpr::parse("A").unwrap());
        let cell = Cell {
            name: vec![],
            inputs: vec![Symbol::from("A")],
            outputs,
            internal: IndexMap::new(),
            expose: vec![],
            async_pins: vec![],
            clock: vec![],
            constraint_arcs: ConstraintPins::Off,
            no_edge_collapse: false,
            when: ArcClasses::default(),
            template: None,
            template_overrides: IndexMap::new(),
            nodes: NodeNames::default(),
            logic_low: None,
            logic_high: None,
        };
        let spec = Spec { cells: vec![cell] };
        let err = spec
            .analyse_with(&ExplorationBudget::default())
            .unwrap_err();
        assert!(matches!(err, ModelError::EmptyName));
    }

    #[test]
    fn distinct_multi_name_cells_analyse_ok() {
        // A valid spec of several cells whose name lists are all distinct still analyses cleanly.
        let s = r#"
[[cell]]
name = ["INVX1", "INVX2"]
inputs = ["A"]
[cell.outputs]
Y = "!A"

[[cell]]
name = "BUF"
inputs = ["A"]
[cell.outputs]
Z = "A"
"#;
        let cells = parse_spec(s)
            .unwrap()
            .analyse_with(&ExplorationBudget::default())
            .unwrap();
        assert_eq!(cells.len(), 2);
    }

    #[test]
    fn async_must_be_input() {
        let s = r#"
[[cell]]
name = "X"
inputs = ["A"]
async = ["R"]
[cell.outputs]
Y = "A"
"#;
        let err = parse_spec(s).unwrap().cells[0].analyse().unwrap_err();
        assert!(matches!(err, ModelError::AsyncNotInput { .. }));
    }

    #[test]
    fn template_override_alias_must_be_a_declared_name() {
        // An override keyed by a name absent from the cell's `name` list is rejected at analyse time.
        let s = r#"
[[cell]]
name = ["INVX1", "INVX2"]
inputs = ["A"]
[cell.outputs]
Y = "!A"
[cell.template_overrides.NOPE]
delay = "delay_template"
"#;
        let err = parse_spec(s).unwrap().cells[0].analyse().unwrap_err();
        assert!(matches!(err, ModelError::UnknownTemplateOverride { .. }));
    }

    #[test]
    fn template_override_on_declared_alias_analyses_ok() {
        // An override keyed by a declared drive-strength alias parses and analyses cleanly.
        let s = r#"
[[cell]]
name = ["INVX1", "INVX2"]
inputs = ["A"]
[cell.outputs]
Y = "!A"
[cell.template_overrides.INVX2]
delay = "delay_template"
"#;
        assert!(parse_spec(s).unwrap().cells[0].analyse().is_ok());
    }

    /// A DFF over two drive strengths whose master sits on a differently-spelled netlist node in each.
    const NODES_SRC: &str = r#"
[[cell]]
name = ["DFFX1", "DFFX4"]
inputs = ["CLK", "D"]
clock = ["CLK"]
expose = ["sela0"]
[cell.internal]
sela0 = "!CLK*D + CLK*sela0"
[cell.outputs]
Q = "CLK*sela0 + !CLK*Q"
[cell.nodes]
sela0 = "XI7/m"
[cell.nodes.DFFX4]
sela0 = "XI4/m"
"#;

    #[test]
    fn nodes_reads_both_halves_of_one_table() {
        // The section carries the cell-wide mapping and the per-alias tables together, told apart by
        // the value's type, and each resolves per signal: the alias's own entry, else the cell-wide.
        let cell = parse_spec(NODES_SRC).unwrap().cells.remove(0);
        let (sela0, x1, x4) = (
            Symbol::from("sela0"),
            Symbol::from("DFFX1"),
            Symbol::from("DFFX4"),
        );
        assert_eq!(cell.nodes.cell[&sela0], Symbol::from("XI7/m"));
        assert_eq!(cell.nodes.aliases[&x4][&sela0], Symbol::from("XI4/m"));
        assert_eq!(cell.nodes.of(&x1, &sela0), Symbol::from("XI7/m"));
        assert_eq!(cell.nodes.of(&x4, &sela0), Symbol::from("XI4/m"));
        // An unmapped signal stands for itself, under every alias.
        let q = Symbol::from("Q");
        assert_eq!(cell.nodes.of(&x4, &q), q);
    }

    #[test]
    fn node_mapping_must_name_a_declared_internal() {
        // A mistyped signal key would map nothing and hand Liberate the spec's own name, so it is a
        // hard error rather than a silent identity.
        let s = r#"
[[cell]]
name = "DFF"
inputs = ["CLK", "D"]
[cell.internal]
M = "!CLK*D + CLK*M"
[cell.outputs]
Q = "CLK*M + !CLK*Q"
[cell.nodes]
NOPE = "XI7/m"
"#;
        let err = parse_spec(s).unwrap().cells[0].analyse().unwrap_err();
        assert!(matches!(err, ModelError::NodeNotInternal { .. }), "{err:?}");
    }

    #[test]
    fn node_mapping_alias_must_be_a_declared_name() {
        let s = r#"
[[cell]]
name = ["DFFX1", "DFFX4"]
inputs = ["CLK", "D"]
[cell.internal]
M = "!CLK*D + CLK*M"
[cell.outputs]
Q = "CLK*M + !CLK*Q"
[cell.nodes.NOPE]
M = "XI7/m"
"#;
        let err = parse_spec(s).unwrap().cells[0].analyse().unwrap_err();
        assert!(
            matches!(err, ModelError::UnknownNodeAlias { .. }),
            "{err:?}"
        );
    }

    /// A two-drive-strength cell exposing two internals, with `nodes` spliced in.
    fn two_exposed_src(nodes: &str) -> String {
        format!(
            r#"
[[cell]]
name = ["DFFX1", "DFFX4"]
inputs = ["CLK", "D"]
clock = ["CLK"]
expose = ["m", "n"]
[cell.internal]
m = "!CLK*D + CLK*m"
n = "CLK*m + !CLK*n"
[cell.outputs]
Q = "n"
{nodes}
"#
        )
    }

    #[test]
    fn two_nodes_may_not_resolve_to_one() {
        // The resolved names are `-pinlist` columns and the vector is positional against that list, so
        // two columns under one name shift every column after them. `expose` rejects the same collision
        // before resolution; the mapping may not reintroduce it.
        let err = parse_spec(&two_exposed_src(
            "[cell.nodes]\nm = \"XI7/m\"\nn = \"XI7/m\"",
        ))
        .unwrap()
        .cells[0]
            .analyse()
            .unwrap_err();
        assert!(matches!(err, ModelError::DuplicateNode { .. }), "{err:?}");
    }

    #[test]
    fn two_mapped_nodes_may_not_share_a_name() {
        // One node, one name: two internals mapped onto the same netlist node say the netlist holds
        // both on one, which it does not — neither is exposed nor probed here, so this is the rule
        // itself rather than the columns it keeps straight.
        let s = r#"
[[cell]]
name = "DFF"
inputs = ["CLK", "D"]
clock = ["CLK"]
[cell.internal]
M = "!CLK*D + CLK*M"
N = "CLK*M + !CLK*N"
[cell.outputs]
Q = "N"
[cell.nodes]
M = "xuxu"
N = "xuxu"
"#;
        let err = parse_spec(s).unwrap().cells[0].analyse().unwrap_err();
        assert!(matches!(err, ModelError::DuplicateNode { .. }), "{err:?}");
    }

    #[test]
    fn a_mapping_may_not_take_an_unmapped_node_s_name() {
        // An unmapped internal stands for itself, so its own name is taken: mapping another onto it
        // collides even though nothing in the spec mentions it twice.
        let s = r#"
[[cell]]
name = "DFF"
inputs = ["CLK", "D"]
clock = ["CLK"]
expose = ["N"]
constraint_arcs = true
[cell.internal]
M = "!CLK*D + CLK*M"
N = "CLK*M + !CLK*N"
[cell.outputs]
Q = "N"
[cell.nodes]
N = "M"
"#;
        let err = parse_spec(s).unwrap().cells[0].analyse().unwrap_err();
        assert!(matches!(err, ModelError::DuplicateNode { .. }), "{err:?}");
    }

    #[test]
    fn an_unexposed_node_is_checked_for_collisions_too() {
        // Which nodes take a column is not settled by `expose`: a constraint arc gives each victim node
        // one of its own, and that is known only after exploration. So every MAPPED node is checked,
        // exposed or not — here a flop's master, the victim of the setup/hold pair it earns.
        let s = r#"
[[cell]]
name = "DFF"
inputs = ["CLK", "D"]
clock = ["CLK"]
constraint_arcs = true
[cell.internal]
M = "!CLK*D + CLK*M"
[cell.outputs]
Q = "CLK*M + !CLK*Q"
[cell.nodes]
M = "CLK"
"#;
        let err = parse_spec(s).unwrap().cells[0].analyse().unwrap_err();
        assert!(
            matches!(err, ModelError::NodeClashesWithPin { .. }),
            "{err:?}"
        );
    }

    #[test]
    fn a_mapped_node_may_not_land_on_a_pin() {
        for onto in ["CLK", "Q"] {
            let err = parse_spec(&two_exposed_src(&format!("[cell.nodes]\nm = \"{onto}\"")))
                .unwrap()
                .cells[0]
                .analyse()
                .unwrap_err();
            assert!(
                matches!(err, ModelError::NodeClashesWithPin { .. }),
                "mapping onto {onto}: {err:?}"
            );
        }
    }

    #[test]
    fn a_collision_is_judged_per_drive_strength() {
        // Resolution is per alias, so the columns one alias emits are what its own check reads: a
        // cell-wide map that is fine stays fine where an alias's override collides.
        let src = two_exposed_src(
            "[cell.nodes]\nm = \"XI7/m\"\nn = \"XI7/n\"\n[cell.nodes.DFFX4]\nn = \"XI7/m\"",
        );
        let err = parse_spec(&src).unwrap().cells[0].analyse().unwrap_err();
        let ModelError::DuplicateNode { alias, .. } = &err else {
            panic!("expected a duplicate-node error, got {err:?}");
        };
        assert_eq!(
            alias.as_str(),
            "DFFX4",
            "the colliding drive strength is named"
        );

        // Without that override the same cell analyses cleanly.
        let ok = two_exposed_src("[cell.nodes]\nm = \"XI7/m\"\nn = \"XI7/n\"");
        assert!(parse_spec(&ok).unwrap().cells[0].analyse().is_ok());
    }

    #[test]
    fn a_malformed_node_value_names_the_shapes_it_accepts() {
        // The error a spec author reads names the two shapes an entry may take, in their vocabulary —
        // not the deserialiser's own type.
        let err = parse_spec(&two_exposed_src("[cell.nodes]\nm = 7"))
            .unwrap_err()
            .to_string();
        assert!(
            err.contains("a netlist node name, or a table of them keyed by drive-strength name"),
            "{err}"
        );
    }

    #[test]
    fn a_mapped_node_reaches_liberate_alone() {
        // The netlist spelling is for Liberate: it addresses the exposed column in the arcs, and
        // appears in no other artifact. The behavioural artifacts keep the spec's own name, which is
        // what the model was built and explored in.
        let cell = analyse_one(NODES_SRC);
        let arcs = Deck(&[cell_arcs(&cell, ArcsTclOptions::default())]).to_string();
        assert!(arcs.contains("-pinlist {CLK D XI7/m Q}"), "{arcs}");
        assert!(arcs.contains("-pinlist {CLK D XI4/m Q}"), "{arcs}");
        assert!(
            !arcs.contains("sela0"),
            "the spec name is not Liberate's:\n{arcs}"
        );

        for other in [
            crate::emit::verilog::Verilog(&crate::emit::verilog::cell_verilog(&cell)).to_string(),
            crate::emit::liberty::library_liberty("lib", crate::emit::liberty::cell_liberty(&cell))
                .to_string(),
            crate::emit::define_cell::Declarations(&crate::emit::define_cell::cell_define_cell(
                &cell,
            ))
            .to_string(),
        ] {
            assert!(
                !other.contains("XI7/m") && !other.contains("XI4/m"),
                "a netlist node reached a behavioural artifact:\n{other}"
            );
        }
    }

    #[test]
    fn per_alias_nodes_fan_the_arcs_out_by_group() {
        // A block addresses its exposed column by one name, so aliases whose netlists disagree on it
        // cannot share a block: each group names only its own aliases. A bare leakage block carries no
        // exposed column, so it stays one block naming both; a walked one carries the column and fans
        // out by group exactly as a measured block does.
        let cell = analyse_one(NODES_SRC);
        let arcs = Deck(&[cell_arcs(&cell, ArcsTclOptions::default())]).to_string();
        for block in arcs.split("define_arc").skip(1) {
            let block = block.split("\n\n").next().unwrap_or(block);
            let (x1, x4) = (block.contains("XI7/m"), block.contains("XI4/m"));
            assert!(x1 ^ x4, "a block addresses one netlist node:\n{block}");
            let named = if x1 { "{ DFFX1 }" } else { "{ DFFX4 }" };
            assert!(
                block.contains(named),
                "the block names only the aliases that agree on it:\n{block}"
            );
        }
        for block in arcs.split("define_leakage").skip(1) {
            let block = block.split("\n\n").next().unwrap_or(block);
            let (x1, x4) = (block.contains("XI7/m"), block.contains("XI4/m"));
            if !block.contains("-pinlist") {
                assert!(
                    !x1 && !x4,
                    "a bare block carries no exposed column:\n{block}"
                );
                assert!(
                    block.contains("{ DFFX1 DFFX4 }"),
                    "no column divides a bare block, so it names every alias:\n{block}"
                );
                continue;
            }
            assert!(
                x1 ^ x4,
                "a walked block addresses one netlist node:\n{block}"
            );
            let named = if x1 { "{ DFFX1 }" } else { "{ DFFX4 }" };
            assert!(
                block.contains(named),
                "the block names only the aliases that agree on it:\n{block}"
            );
        }
    }

    #[test]
    fn an_unmapped_cell_emits_one_group() {
        // The common case: nothing mapped, so every alias is named together and the exposed node keeps
        // its own name — the output a cell had before any of this existed.
        let cell = analyse_one(
            r#"
[[cell]]
name = ["DFFX1", "DFFX4"]
inputs = ["CLK", "D"]
clock = ["CLK"]
expose = ["sela0"]
[cell.internal]
sela0 = "!CLK*D + CLK*sela0"
[cell.outputs]
Q = "CLK*sela0 + !CLK*Q"
"#,
        );
        let arcs = Deck(&[cell_arcs(&cell, ArcsTclOptions::default())]).to_string();
        assert!(arcs.contains("-pinlist {CLK D sela0 Q}"), "{arcs}");
        assert!(!arcs.contains("{ DFFX1 }"), "one group names both:\n{arcs}");
        assert!(arcs.contains("{ DFFX1 DFFX4 }"), "{arcs}");
    }

    #[test]
    fn expose_of_declared_internal_analyses_ok_with_order_preserved() {
        let s = r#"
[[cell]]
name = "DFF"
inputs = ["CLK", "D"]
expose = ["QN", "M"]
[cell.internal]
M = "!CLK*D + CLK*M"
QN = "!M"
[cell.outputs]
Q = "CLK*M + !CLK*Q"
"#;
        let cell = parse_spec(s).unwrap().cells.remove(0).analyse().unwrap();
        assert_eq!(cell.exposed, vec![Symbol::from("QN"), Symbol::from("M")]);
    }

    /// The worked C-element `QN = !(A*B + Q*(A+B))`, `Q = !QN`, in the two spellings the view split is
    /// read against: `{expose}` is the `expose = ["QN"]` line, or nothing at all.
    fn c_element_src(expose: &str) -> String {
        format!(
            r#"
[[cell]]
name = "C2"
inputs = ["A", "B"]
{expose}
[cell.internal]
QN = "!(A*B + Q*(A+B))"
[cell.outputs]
Q = "!QN"
"#
        )
    }

    #[test]
    fn a_cell_exposing_nothing_carries_a_single_view() {
        // With nothing exposed there is one minimisation, one machine pass and one view: `arc_view`
        // hands back the cell itself, so every emitter reads the same analysis.
        let cell = analyse_one(&c_element_src(""));
        assert!(cell.exposed_view.is_none());
        assert!(std::ptr::eq(cell.arc_view(), &cell));
        assert!(cell.exposed.is_empty());
        // The C-element's coordinate lands on its output pin: QN folds away and Q self-holds.
        assert!(cell.internals.is_empty());
        assert!(!cell.arcs.is_empty());
    }

    #[test]
    fn exposing_an_internal_node_yields_an_arc_view_beside_the_model_view() {
        let cell = analyse_one(&c_element_src(r#"expose = ["QN"]"#));

        // The model view is minimised to the output alone, so QN is folded away — though `exposed`
        // still names it, as the spec's declared list is carried verbatim into both views.
        assert!(
            cell.internals.is_empty(),
            "the model view keeps only genuine-memory coordinates: {:?}",
            cell.internals.iter().map(|s| &s.name).collect::<Vec<_>>(),
        );
        assert_eq!(cell.exposed, vec![Symbol::from("QN")]);
        assert_eq!(cell.exposed_signals().count(), 0);

        // The arc view keeps QN as a machine coordinate, so every measured start state carries a QN
        // column for the arcs to drive (`-ic`) and observe (`-vector`).
        let arc = cell.arc_view();
        assert!(!std::ptr::eq(arc, &cell), "the arc view is a distinct cell");
        assert!(arc.internals.iter().any(|s| s.name == "QN"));
        assert_eq!(
            arc.exposed_signals().collect::<Vec<_>>(),
            [&Symbol::from("QN")],
        );
        assert!(!arc.arcs.is_empty());
        assert!(
            arc.arcs.iter().all(|a| a.start.value_of("QN").is_some()),
            "an exposed node is a state column of every arc's start state",
        );
        assert!(
            arc.exposed_view.is_none(),
            "the arc view never carries a view of its own"
        );
    }

    #[test]
    fn the_model_view_carries_its_own_coordinates() {
        // The C-element's coordinate MOVES NAME between the views: the arc view holds QN as its
        // self-holding machine coordinate (kept alive by the exposure), while releasing the exposure
        // lets Q self-hold instead. So a model-view arc's start must be read against the model view's
        // OWN coordinates, never the arc view's — projecting onto the wrong view's names is exactly
        // what this guards against.
        let cell = analyse_one(&c_element_src(r#"expose = ["QN"]"#));
        assert!(!cell.arcs.is_empty(), "the model view emits arcs");
        for a in &cell.arcs {
            assert!(
                a.start.value_of("Q").is_some(),
                "a model-view arc's start must define its own coordinate Q: {:?}",
                a.start
            );
            assert!(
                a.start.value_of("QN").is_none(),
                "a model-view arc's start must not carry the arc view's QN column: {:?}",
                a.start
            );
        }

        let arc = cell.arc_view();
        assert!(!arc.arcs.is_empty(), "the arc view emits arcs");
        assert!(
            arc.arcs.iter().all(|a| a.start.value_of("QN").is_some()),
            "an arc-view arc's start must carry QN as its own coordinate"
        );
    }

    #[test]
    fn every_arc_carries_the_prevector_that_reaches_it() {
        // Every arc, hidden arc and constraint of BOTH views must carry a real prevector: non-empty,
        // and ending at the record's own start state projected onto the inputs (the pattern at
        // arcs.rs:618). A rebuilt `prev` that breaks `path_to` either empties the prevector — panicking
        // the `.expect` the constraint columns read their held levels through
        // (`arcs_tcl::constraint_columns`) — or misaligns the chain, corrupting the `prevector.len()`
        // constraint-dedup tie-break in `constraint::record`.
        let cell = analyse_one(&c_element_src(r#"expose = ["QN"]"#));
        for view in [cell.arc_view(), &cell] {
            assert!(!view.arcs.is_empty(), "the view emits arcs");
            for a in &view.arcs {
                assert!(
                    !a.prevector.is_empty(),
                    "arc {a:?} carries an empty prevector"
                );
                assert_eq!(
                    a.prevector.last().unwrap(),
                    &a.start.project_to(&view.inputs),
                    "arc {a:?}: the prevector must end at its own start"
                );
            }
            assert!(!view.hidden_arcs.is_empty(), "the view emits hidden arcs");
            for h in &view.hidden_arcs {
                assert!(
                    !h.prevector.is_empty(),
                    "hidden arc {h:?} carries an empty prevector"
                );
                assert_eq!(
                    h.prevector.last().unwrap(),
                    &h.start.project_to(&view.inputs),
                    "hidden arc {h:?}: the prevector must end at its own start"
                );
            }
            for c in &view.constraints {
                assert!(
                    !c.prevector.is_empty(),
                    "constraint {c:?} carries an empty prevector"
                );
            }
        }
    }

    #[test]
    fn the_model_view_is_what_an_expose_free_analysis_yields() {
        // Exposure is arcs-only: the view the Liberty, Verilog and statetable emitters read must be the
        // fully-minimised model, signal for signal, expression for expression and record for record.
        // The model view carries the arc view's exploration onto its own coordinates instead of
        // exploring for itself, so this also holds that projection to what a plain analysis discovers —
        // across the C-element, where releasing the exposure moves the coordinate from `QN` to `Q`, and
        // the DFF, where the exposed master survives both views.
        for (with, without, _) in exposure_pairs() {
            let exposed = analyse_one(&with);
            let plain = analyse_one(&without);
            let cell = exposed.repr_name();

            let names = |c: &AnalysedCell| c.signals().map(|s| s.name.clone()).collect::<Vec<_>>();
            assert_eq!(names(&exposed), names(&plain), "cell {cell}: signals");
            for (a, b) in exposed.signals().zip(plain.signals()) {
                assert_eq!(
                    a.expr, b.expr,
                    "cell {cell}: signal {} display expression",
                    a.name
                );
                assert_eq!(a.vars, b.vars, "cell {cell}: signal {} support", a.name);
                assert_eq!(
                    a.feedback, b.feedback,
                    "cell {cell}: signal {} feedback",
                    a.name
                );
            }
            assert_eq!(
                exposed.state_holding(),
                plain.state_holding(),
                "cell {cell}: state_holding"
            );
            assert_same_cell_records(&exposed, &plain);
        }
    }

    /// A master-slave DFF, in the two spellings the view split is read against: `{expose}` is the
    /// `expose = ["M"]` line, or nothing at all. Unlike the C-element the master survives the
    /// outputs-only minimisation, so it is the shape where an exposed node is still a signal of the
    /// MODEL view — the one an artifact could leak it into.
    fn dff_src(expose: &str) -> String {
        format!(
            r#"
[[cell]]
name = "DFF"
inputs = ["CLK", "D"]
clock = ["CLK"]
{expose}
[cell.internal]
M = "!CLK*D + CLK*M"
[cell.outputs]
Q = "CLK*M + !CLK*Q"
"#
        )
    }

    /// The two exposing fixtures paired with their exposure-free spelling, and the node each exposes.
    fn exposure_pairs() -> Vec<(String, String, &'static str)> {
        vec![
            (c_element_src(r#"expose = ["QN"]"#), c_element_src(""), "QN"),
            (dff_src(r#"expose = ["M"]"#), dff_src(""), "M"),
        ]
    }

    /// One transition arc reduced to the identity [`crate::logic::arcs::derive`] keys it on: the driven
    /// output with the edge it makes, the related pin — which holds no edge of its own — and the state
    /// the arc is measured from. The rest of an arc follows from that identity — its end state and levels
    /// are read off the start state, and its prevector is one path into it.
    #[derive(Debug, PartialEq, Eq)]
    struct ArcRecord {
        output: PinEdge,
        related: Symbol,
        start: Minterm<Symbol>,
    }

    /// One internal-power ('hidden') arc reduced to its identity: the toggled pin with the edge it makes,
    /// and the state the toggle is measured from.
    #[derive(Debug, PartialEq, Eq)]
    struct HiddenArcRecord {
        pin: PinEdge,
        start: Minterm<Symbol>,
    }

    /// One static leakage state reduced to the rest state it records: the inputs held there and every
    /// output's settled level. Its prevector is one path into that state, so it is left out with every
    /// other representative — two rest states differing only in an internal node still reduce to two
    /// records, which the record COUNT holds even where the two read alike.
    #[derive(Debug, PartialEq, Eq)]
    struct LeakageRecord {
        inputs: Minterm<Symbol>,
        outputs: Vec<HeldLevel>,
    }

    /// One generated constraint by its identity: the pin it constrains with the edge that pin makes,
    /// the kind — which carries the other pin of a separation, where there is one — and the victim nodes
    /// it probes.
    #[derive(Debug, PartialEq, Eq)]
    struct ConstraintRecord {
        kind: ConstraintKind,
        pin: PinEdge,
        nodes: Vec<Symbol>,
    }

    /// One detected hazard, reduced to what identifies it: the (cause, outcome) cell it occupies and the
    /// state variables it decides. `cause` alone already carries a race's pins or a pulse's pin, so no
    /// separate pins/pin record is needed.
    #[derive(Debug, PartialEq, Eq)]
    struct HazardRecord {
        cause: Cause,
        outcome: Outcome,
        group: Vec<Symbol>,
    }

    /// The behavioural edge classification reduced to the node names it carries: the folded masters, the
    /// nodes holding an edge capture, and the derived registers.
    #[derive(Debug, PartialEq, Eq)]
    struct EdgeRecord {
        folded: BTreeSet<Symbol>,
        captures: BTreeSet<Symbol>,
        derived: BTreeSet<Symbol>,
    }

    /// Everything one analysed view emits, each record reduced to what identifies it. Two views agreeing
    /// here emit the same arcs, hazards and constraints.
    ///
    /// A record's prevector, the levels sampled alongside it, its exploration-order index and a
    /// hazard's settled state are left out. Each names WHICH of several equally-good reachable states
    /// the pipeline chose to observe the record from, and that choice follows the BFS order — a
    /// representative, not behaviour.
    #[derive(Debug)]
    struct CellRecords {
        arcs: Vec<ArcRecord>,
        hidden_arcs: Vec<HiddenArcRecord>,
        leakage: Vec<LeakageRecord>,
        constraints: Vec<ConstraintRecord>,
        hazards: Vec<HazardRecord>,
        edge: EdgeRecord,
    }

    /// Reduce a view to [`CellRecords`]. Takes the whole shipped [`AnalysedCell`] rather than any piece
    /// of the analysis, so a view routed to the wrong place reads back as a record-set difference.
    fn records(cell: &AnalysedCell) -> CellRecords {
        CellRecords {
            arcs: cell
                .arcs
                .iter()
                .map(|a| ArcRecord {
                    output: a.output.clone(),
                    related: a.related.clone(),
                    start: a.start.clone(),
                })
                .collect(),
            hidden_arcs: cell
                .hidden_arcs
                .iter()
                .map(|h| HiddenArcRecord {
                    pin: h.pin.clone(),
                    start: h.start.clone(),
                })
                .collect(),
            leakage: cell
                .leakage
                .iter()
                .map(|l| LeakageRecord {
                    inputs: l.inputs.clone(),
                    outputs: l.levels.outputs.clone(),
                })
                .collect(),
            constraints: cell
                .constraints
                .iter()
                .map(|c| ConstraintRecord {
                    kind: c.kind.clone(),
                    pin: c.pin.clone(),
                    nodes: c.victim_names(),
                })
                .collect(),
            hazards: cell
                .hazards
                .iter()
                .map(|h| HazardRecord {
                    cause: h.cause.clone(),
                    outcome: h.outcome,
                    group: h.group.clone(),
                })
                .collect(),
            edge: EdgeRecord {
                folded: cell.edge.folded.iter().cloned().collect(),
                captures: cell.edge.captures.iter().map(|c| c.node.clone()).collect(),
                derived: cell.edge.derived.iter().map(|d| d.name.clone()).collect(),
            },
        }
    }

    /// Assert two runs emitted the same records of one kind, in any order. A record carries `Eq` and not
    /// `Ord`, so membership is a scan — free at fixture size, and a mismatch names the record itself
    /// rather than the position it sits at.
    fn assert_same_records<T: PartialEq + std::fmt::Debug>(
        exposing: &[T],
        plain: &[T],
        what: &str,
    ) {
        for r in exposing {
            assert!(
                plain.contains(r),
                "{what}: only the exposing run emits {r:?}"
            );
        }
        for r in plain {
            assert!(
                exposing.contains(r),
                "{what}: only the exposure-free run emits {r:?}"
            );
        }
        assert_eq!(exposing.len(), plain.len(), "{what}: record counts differ");
    }

    /// Assert an exposing cell's model view and an exposure-free analysis of the same cell emit the same
    /// records — the whole of what exposure is not allowed to reach.
    fn assert_same_cell_records(exposing: &AnalysedCell, plain: &AnalysedCell) {
        let cell = exposing.repr_name();
        let (a, b) = (records(exposing), records(plain));
        assert_same_records(&a.arcs, &b.arcs, &format!("cell {cell}: arcs"));
        assert_same_records(
            &a.hidden_arcs,
            &b.hidden_arcs,
            &format!("cell {cell}: hidden arcs"),
        );
        assert_same_records(&a.leakage, &b.leakage, &format!("cell {cell}: leakage"));
        assert_same_records(
            &a.constraints,
            &b.constraints,
            &format!("cell {cell}: constraints"),
        );
        assert_same_records(&a.hazards, &b.hazards, &format!("cell {cell}: hazards"));
        assert_eq!(a.edge, b.edge, "cell {cell}: edge classification");
    }

    #[test]
    fn exposure_changes_the_arcs_and_nothing_else() {
        // The arcs-only claim, as an invariance of THIS binary rather than against a recorded baseline:
        // analyse each fixture twice, once exposing and once not, and the model view every emitter but
        // the arcs one reads emits the same records either way. The arcs are where the difference lands,
        // as the exposed node's own column.
        for (with, without, node) in exposure_pairs() {
            let exposed = analyse_one(&with);
            let plain = analyse_one(&without);
            assert_eq!(exposed.exposed, [Symbol::from(node)]);
            assert!(plain.exposed.is_empty());

            let cell = exposed.repr_name();
            assert_same_cell_records(&exposed, &plain);

            // The arcs differ in exactly one way: every ARC block of the exposing run lists the node
            // among its columns, and no block of the other run does. Leakage is switched off for this
            // comparison and asserted on its own below, where a walked block states the exposed node
            // through its own `-pinlist`.
            let opts = ArcsTclOptions {
                emit_leakage: false,
                ..Default::default()
            };
            let arcs = |c| Deck(&[cell_arcs(c, opts)]).to_string();
            let pinlists = |tcl: String| -> Vec<String> {
                tcl.lines()
                    .map(str::trim)
                    .filter(|l| l.starts_with("-pinlist "))
                    .map(str::to_owned)
                    .collect()
            };
            let (with_node, without_node) = (pinlists(arcs(&exposed)), pinlists(arcs(&plain)));
            assert!(!with_node.is_empty(), "cell {cell}: the fixture emits arcs");
            let names = |l: &str| {
                l.split_whitespace()
                    .any(|t| t.trim_matches(['{', '}']) == node)
            };
            assert!(
                with_node.iter().all(|l| names(l)),
                "cell {cell}: every exposing block lists {node}: {with_node:?}"
            );
            assert!(
                !without_node.iter().any(|l| names(l)),
                "cell {cell}: no exposure-free block lists {node}: {without_node:?}"
            );

            // A walked leakage block states the exposed node through its own `-pinlist`, exactly as a
            // measured block does — so it names the node under the exposing run and none does under the
            // exposure-free one. A bare block carries no column under either run.
            let all = ArcsTclOptions::default();
            for (c, expect_named) in [(&exposed, true), (&plain, false)] {
                let tcl = Deck(&[cell_arcs(c, all)]).to_string();
                let leakage: Vec<String> = tcl
                    .split("define_leakage")
                    .skip(1)
                    .map(|b| match b.find("\n\n") {
                        Some(off) => b[..off].to_owned(),
                        None => b.to_owned(),
                    })
                    .collect();
                assert!(!leakage.is_empty(), "cell {cell}: the fixture leaks");
                let columns = |b: &str| -> Vec<String> {
                    b.lines()
                        .map(str::trim)
                        .filter(|l| l.starts_with("-pinlist "))
                        .map(str::to_owned)
                        .collect()
                };
                assert!(
                    leakage.iter().any(|b| !columns(b).is_empty()),
                    "cell {cell}: a walked leakage block carries columns to check"
                );
                for block in &leakage {
                    let cols = columns(block);
                    if cols.is_empty() {
                        continue; // a bare block is column-free under either run
                    }
                    assert_eq!(
                        cols.iter().any(|l| names(l)),
                        expect_named,
                        "cell {cell}: a walked leakage column names {node} exactly when the run exposes it: {cols:?}"
                    );
                }
            }
        }
    }

    #[test]
    fn define_cell_never_declares_an_exposed_node() {
        // `define_cell` declares the cell's PINS, and an exposed internal is not one however many arc
        // columns it earns — including the DFF's master, which survives the model view as a signal.
        for (with, _, node) in exposure_pairs() {
            let cell = analyse_one(&with);
            let tcl = crate::emit::define_cell::Declarations(
                &crate::emit::define_cell::cell_define_cell(&cell),
            )
            .to_string();
            assert!(
                !tcl.split_whitespace().any(|t| t == node),
                "define_cell declared the exposed {node}:\n{tcl}"
            );
        }
    }

    #[test]
    fn both_views_agree_on_state_holding() {
        // Minimisation re-labels where a cell's memory lives; it never creates or destroys it. So the
        // view that keeps the exposed nodes as coordinates holds state exactly when the model view does
        // — including for a cell whose exposed node is plain combinational logic and has none.
        let combinational = r#"
[[cell]]
name = "AN2"
inputs = ["A", "B"]
expose = ["W"]
[cell.internal]
W = "A*B"
[cell.outputs]
Y = "!W"
"#;
        let mut holding = 0;
        for src in [
            c_element_src(r#"expose = ["QN"]"#),
            dff_src(r#"expose = ["M"]"#),
            combinational.to_owned(),
        ] {
            let cell = analyse_one(&src);
            assert_eq!(
                cell.state_holding(),
                cell.arc_view().state_holding(),
                "cell {}: the two views disagree on state_holding",
                cell.repr_name(),
            );
            holding += usize::from(cell.state_holding());
        }
        assert_eq!(holding, 2, "the fixtures cover both verdicts");
    }

    #[test]
    fn expose_of_output_is_rejected() {
        let s = r#"
[[cell]]
name = "X"
inputs = ["A"]
expose = ["Y"]
[cell.outputs]
Y = "A"
"#;
        let err = parse_spec(s).unwrap().cells[0].analyse().unwrap_err();
        assert!(matches!(err, ModelError::ExposeNotInternal { .. }));
    }

    #[test]
    fn expose_of_input_is_rejected() {
        let s = r#"
[[cell]]
name = "X"
inputs = ["A"]
expose = ["A"]
[cell.outputs]
Y = "A"
"#;
        let err = parse_spec(s).unwrap().cells[0].analyse().unwrap_err();
        assert!(matches!(err, ModelError::ExposeNotInternal { .. }));
    }

    #[test]
    fn expose_of_unknown_name_is_rejected() {
        let s = r#"
[[cell]]
name = "X"
inputs = ["A"]
expose = ["Z"]
[cell.outputs]
Y = "A"
"#;
        let err = parse_spec(s).unwrap().cells[0].analyse().unwrap_err();
        assert!(matches!(err, ModelError::ExposeNotInternal { .. }));
    }

    #[test]
    fn duplicate_expose_is_rejected() {
        let s = r#"
[[cell]]
name = "DFF"
inputs = ["CLK", "D"]
expose = ["M", "M"]
[cell.internal]
M = "!CLK*D + CLK*M"
[cell.outputs]
Q = "CLK*M + !CLK*Q"
"#;
        let err = parse_spec(s).unwrap().cells[0].analyse().unwrap_err();
        assert!(matches!(err, ModelError::DuplicateExpose { .. }));
    }

    #[test]
    fn logic_voltages_default_is_0_and_vdd() {
        let v = LogicVoltages::default();
        assert_eq!(v.low, "0");
        assert_eq!(v.high, "$VDD");
    }

    #[test]
    fn logic_voltages_from_options_fills_each_side_independently() {
        let v = LogicVoltages::from_options(Some("GND"), None);
        assert_eq!(v.low, "GND");
        assert_eq!(v.high, "$VDD");

        let v = LogicVoltages::from_options(None, Some("$VDDH"));
        assert_eq!(v.low, "0");
        assert_eq!(v.high, "$VDDH");
    }

    /// A cell carrying `low`/`high` verbatim, for the analysis of the resolved voltages.
    fn cell_with_voltages(low: &str, high: &str) -> Cell {
        let s = format!(
            r#"
[[cell]]
name = "X"
inputs = ["A"]
logic_low = {low:?}
logic_high = {high:?}
[cell.outputs]
Q = "A"
"#
        );
        parse_spec(&s).unwrap().cells.remove(0)
    }

    #[test]
    fn any_logic_voltage_is_carried_through_analysis() {
        // Analysis turns no voltage expression away and rewrites none of them: whatever the spec says
        // reaches the arcs emitter as written, which is where it is rendered into one `-ic` column.
        for value in [
            "$VDD * 0.9",
            "[expr $VDD*0.9]",
            "{$VDD * 0.9}",
            "",
            "$V\"X",
            "{$VDD",
        ] {
            let cell = cell_with_voltages("0", value)
                .analyse()
                .unwrap_or_else(|e| panic!("expected {value:?} to analyse, got {e}"));
            assert_eq!(cell.voltages.high, value);
            assert_eq!(cell.voltages.low, "0");
        }
    }

    #[test]
    fn expose_and_logic_voltages_round_trip_through_toml() {
        let s = r#"
[[cell]]
name = "DFF"
inputs = ["CLK", "D"]
expose = ["M"]
logic_low = "GND"
logic_high = "VDDH"
[cell.internal]
M = "!CLK*D + CLK*M"
[cell.outputs]
Q = "CLK*M + !CLK*Q"
"#;
        let spec = parse_spec(s).unwrap();
        let cell = &spec.cells[0];
        assert_eq!(cell.expose, vec![Symbol::from("M")]);
        assert_eq!(cell.logic_low.as_deref(), Some("GND"));
        assert_eq!(cell.logic_high.as_deref(), Some("VDDH"));

        let analysed = cell.analyse().unwrap();
        assert_eq!(analysed.exposed, vec![Symbol::from("M")]);
        assert_eq!(analysed.voltages.low, "GND");
        assert_eq!(analysed.voltages.high, "VDDH");
    }

    #[test]
    fn invalid_output_function_fails_at_parse_spec() {
        // A malformed function under `[cell.outputs]` is a hard error at TOML deserialise time
        // (parse_spec), never reaching `.analyse()` — parse failures now surface at LOAD, at the
        // value's own TOML span, carrying the underlying BoolExpr parse error through.
        let s = r#"
[[cell]]
name = "BAD"
inputs = ["A"]
[cell.outputs]
Q = "A +"
"#;
        let err = parse_spec(s).unwrap_err();
        assert!(matches!(err, ModelError::Spec(_)));
    }

    #[test]
    fn output_function_builds_to_expected_bdd() {
        // Preserves the c-element / NOT>AND>OR precedence grammar coverage that lived in the removed
        // src/expr.rs, now exercised at the deserialise-time parse boundary: a function parsed once
        // into a BoolExpr at load must build to the same BDD as the equivalent hand-built expression.
        const SRC: &str = r#"
[[cell]]
name = "GRAMMAR"
inputs = ["A", "B", "a", "b", "c"]
[cell.outputs]
Q = "A*B + Q*(A+B)"
Y1 = "a + b*c"
Y2 = "!a*b"
"#;

        // Direct check on the raw, parse-time field: the c-element function is parsed and stored on
        // `Cell.outputs` at deserialise time, before any minimisation write-back can touch it.
        let raw = parse_spec(SRC).unwrap();
        assert_eq!(
            raw.cells[0].outputs[&Symbol::from("Q")],
            BoolExpr::parse("A*B + Q*(A+B)").unwrap()
        );

        let cell = analyse_one(SRC);

        // Look each output up by name. Positions would only happen to line up because the TOML
        // parser hands the table over sorted by key; a renamed output would silently shift them.
        let out = |n: &str| {
            let name = Symbol::from(n);
            &cell
                .outputs
                .iter()
                .find(|o| o.name == name)
                .unwrap_or_else(|| panic!("output {n} missing"))
                .expr
        };

        let builder = bdd_builder!();

        let got = builder.build(out("Q"));
        let want = builder.build(&expr!(("A" & "B") | ("Q" & ("A" | "B"))));
        assert!(got.equivalent_to(&want), "c-element function must match");

        // NOT > AND > OR: `a + b*c` == `a | (b & c)`.
        let got = builder.build(out("Y1"));
        let want = builder.build(&expr!("a" | ("b" & "c")));
        assert!(
            got.equivalent_to(&want),
            "precedence: a + b*c == a | (b & c)"
        );

        // `!a*b` == `(!a) & b`.
        let got = builder.build(out("Y2"));
        let want = builder.build(&expr!(!"a" & "b"));
        assert!(got.equivalent_to(&want), "precedence: !a*b == (!a) & b");
    }

    #[test]
    fn accepts_superset_operator_syntax_at_parse_spec() {
        // espresso's grammar also accepts `&`/`|`/`~`/`^` and `true`/`false`; precedence NOT > AND >
        // XOR > OR, so `a & b | ~c ^ d` == `(a&b) | ((~c)^d)`. Preserves the coverage that lived in the
        // removed src/expr.rs's `accepts_superset_syntax`, now exercised at the deserialise-time parse
        // boundary: the raw, parse-time field on `Cell.outputs` (not the post-pipeline `AnalysedCell`).
        let s = r#"
[[cell]]
name = "SUPERSET"
inputs = ["a", "b", "c", "d"]
[cell.outputs]
Y = "a & b | ~c ^ d"
"#;
        let raw = parse_spec(s).unwrap();

        let builder = bdd_builder!();
        let got = builder.build(&raw.cells[0].outputs[&Symbol::from("Y")]);
        let want = builder.build(&expr!(("a" & "b") | (!"c" ^ "d")));
        assert!(got.equivalent_to(&want));
    }

    #[test]
    fn xor_precedence_pinned_between_and_and_or_at_parse_spec() {
        // Pins the one precedence boundary left uncovered: NOT > AND > XOR > OR, so XOR binds looser
        // than AND but tighter than OR. Each case asserts both an equivalence to the correctly
        // parenthesised reading and a non-equivalence to the wrongly parenthesised reading — the
        // non-equivalence is what actually pins the boundary, since a merely-equivalent pair would
        // pass under either precedence.
        let s = r#"
[[cell]]
name = "XORPREC"
inputs = ["a", "b", "c"]
[cell.outputs]
AndBeforeXor = "a*b ^ c"
XorBeforeOr = "a ^ b + c"
"#;
        let raw = parse_spec(s).unwrap();
        let builder = bdd_builder!();

        // AND binds tighter than XOR: `a*b ^ c` == `(a*b)^c`, not `a*(b^c)`.
        let got = builder.build(&raw.cells[0].outputs[&Symbol::from("AndBeforeXor")]);
        let want_and_first = builder.build(&expr!(("a" & "b") ^ "c"));
        let want_xor_first = builder.build(&expr!("a" & ("b" ^ "c")));
        assert!(
            got.equivalent_to(&want_and_first),
            "precedence: a*b ^ c == (a*b)^c"
        );
        assert!(
            !got.equivalent_to(&want_xor_first),
            "precedence: a*b ^ c must NOT equal a*(b^c)"
        );

        // XOR binds tighter than OR: `a ^ b + c` == `(a^b)+c`, not `a^(b+c)`.
        let got = builder.build(&raw.cells[0].outputs[&Symbol::from("XorBeforeOr")]);
        let want_xor_first = builder.build(&expr!(("a" ^ "b") | "c"));
        let want_or_first = builder.build(&expr!("a" ^ ("b" | "c")));
        assert!(
            got.equivalent_to(&want_xor_first),
            "precedence: a ^ b + c == (a^b)+c"
        );
        assert!(
            !got.equivalent_to(&want_or_first),
            "precedence: a ^ b + c must NOT equal a^(b+c)"
        );
    }

    #[test]
    fn accepts_constant_literals_at_parse_spec() {
        // Preserves the constant-literal coverage that lived in the removed src/expr.rs (the bare
        // numeral `1` from `constants_and_pin_names_with_digits`, and the `true`/`false` word literals
        // from `accepts_superset_syntax`), now exercised at the deserialise-time parse boundary: the
        // raw, parse-time field on `Cell.outputs`.
        let s = r#"
[[cell]]
name = "CONST"
inputs = ["A"]
[cell.outputs]
Y = "A + 1"
T = "true"
F = "false"
"#;
        let raw = parse_spec(s).unwrap();

        let builder = bdd_builder!();
        assert!(
            builder
                .build(&raw.cells[0].outputs[&Symbol::from("Y")])
                .is_tautology(),
            "A + 1 is a tautology"
        );
        assert!(builder
            .build(&raw.cells[0].outputs[&Symbol::from("T")])
            .is_tautology());
        assert!(builder
            .build(&raw.cells[0].outputs[&Symbol::from("F")])
            .is_contradiction());
    }

    #[test]
    fn accepts_digit_and_underscore_identifiers_at_parse_spec() {
        // Pins the identifier rule stated as a guarantee on [`Cell::outputs`] and in the README: an
        // identifier is a letter or `_` followed by letters, digits or `_`. Preserves the pin-name
        // half of the removed src/expr.rs's `constants_and_pin_names_with_digits`, now exercised at
        // the deserialise-time parse boundary.
        let s = r#"
[[cell]]
name = "IDENT"
inputs = ["M1", "P2", "_x"]
[cell.outputs]
Y = "M1*P2 + 1"
Z = "_x*M1"
"#;
        let raw = parse_spec(s).unwrap();
        let outputs = &raw.cells[0].outputs;

        let vars: BTreeSet<Symbol> = outputs[&Symbol::from("Y")].variables().collect();
        assert_eq!(
            vars,
            BTreeSet::from([Symbol::from("M1"), Symbol::from("P2")]),
            "digit-bearing pin names parse as identifiers"
        );

        let vars: BTreeSet<Symbol> = outputs[&Symbol::from("Z")].variables().collect();
        assert_eq!(
            vars,
            BTreeSet::from([Symbol::from("_x"), Symbol::from("M1")]),
            "a leading underscore is a valid identifier start"
        );
    }

    #[test]
    fn rejects_malformed_output_function_at_parse_spec() {
        // Preserves the malformed-input coverage that lived in the removed src/expr.rs's
        // `rejects_garbage`, now exercised at the deserialise-time parse boundary: each of these must
        // fail `parse_spec` itself, with no `.analyse()` call reached. (`"a +"` is already covered by
        // `invalid_output_function_fails_at_parse_spec`.)
        for bad in ["", "a b", "(a", "a @ b"] {
            let s = format!(
                r#"
[[cell]]
name = "X"
inputs = ["a", "b"]
[cell.outputs]
Y = "{bad}"
"#
            );
            assert!(
                parse_spec(&s).is_err(),
                "expected parse_spec to reject {bad:?}"
            );
        }
    }
}
