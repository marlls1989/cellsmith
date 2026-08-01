//! Input model: a minimal multi-cell TOML spec, plus analysis that classifies each function's
//! variables into **primary inputs** vs **feedback/state** (an output name referenced inside a
//! function is the delayed/feedback value of that output).

use std::collections::{BTreeMap, BTreeSet};

use espresso_logic::bdd::{Bdd, BddBuilder, Brand, ManagerCell};
use espresso_logic::{sync_bdd_builder, BoolExpr, Symbol};
use indexmap::IndexMap;
use rayon::prelude::*;
use serde::Deserialize;
use thiserror::Error;

use crate::logic::arcs::{Arc, HiddenArc};
use crate::logic::confluence::Constraint;
use crate::logic::hazard::{OrderDependence, Oscillation};
use crate::logic::leakage::LeakageState;
use crate::logic::machine::ExplorationBudget;

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
    pub inputs: Vec<Symbol>,
    /// Output pin name -> Boolean function, parsed at deserialise time. Entries arrive in the order
    /// the TOML parser yields them — sorted by name, not as written in the file — and that order is
    /// stable from then on.
    ///
    /// The function text's grammar is a superset of the `a*b+!c` form: `*`/`&` AND, `+`/`|` OR,
    /// `!`/`~` NOT, `^` XOR, `0`/`1`/`true`/`false` constants, and parentheses for grouping.
    /// Precedence, tightest first: NOT > AND > XOR > OR. Identifiers are a letter/`_` followed by
    /// letters/digits/`_` (so pin names like `M1`, `P2`, `Q` are fine).
    #[serde(deserialize_with = "de_symbol_expr_map")]
    pub outputs: IndexMap<Symbol, BoolExpr>,
    /// Optional: internal state variable name -> Boolean function, parsed at deserialise time (same
    /// grammar and name-sorted ordering as [`Cell::outputs`]). An internal signal is referenceable by other
    /// functions and is a driven state variable (modelled in the Verilog and the Liberty state
    /// table), but emits **no** external output pin and is never an arc source or target.
    #[serde(default, deserialize_with = "de_symbol_expr_map")]
    pub internal: IndexMap<Symbol, BoolExpr>,
    /// Optional: the internal nodes listed in the Liberate arcs' `-pinlist`, in declared order (the
    /// declared order fixes their pinlist position). Each is preserved through the state-space
    /// minimisation so the arcs can drive it (`-ic`) and observe it (`-vector`). Spec-only: like
    /// `template`/`template_overrides` above, there is no CLI counterpart.
    #[serde(default, deserialize_with = "de_symbol_vec")]
    pub expose: Vec<Symbol>,
    /// Optional: input pins that force the output regardless of held state (async set/reset),
    /// so their arcs are emitted as `-type async` rather than combinational.
    #[serde(rename = "async", default, deserialize_with = "de_symbol_vec")]
    pub async_pins: Vec<Symbol>,
    /// Optional: input pins that are clocks. A hazard on a pin pair holding a declared clock yields a
    /// directed setup/hold constraint (clock ← data); any other pair yields a symmetric non_seq. See
    /// [`crate::logic::confluence`].
    #[serde(default, deserialize_with = "de_symbol_vec")]
    pub clock: Vec<Symbol>,
    /// Optional: opt in to emitting derived constraint arcs (setup/hold, non_seq) for this cell. Off by
    /// default; also enabled globally by the `--constraints` CLI flag.
    #[serde(default)]
    pub constraint_arcs: bool,
    /// Optional: opt OUT of the behavioural per-arc edge classification for this cell (see
    /// [`crate::logic::edge`]). Classification is ON by default; setting this true (or the global
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
    /// (delay/power/constrain). Structural only — the template names come from the spec, never
    /// generated. `None` fields carry through unset.
    #[serde(default)]
    pub template: Option<TemplateSpec>,
    /// Optional: per-drive-strength-alias template overrides, keyed by a name from this cell's `name`
    /// list. Each alias's [`TemplateSpec`] is merged per-field over the cell-wide `template`. Keys are
    /// validated against the cell's declared names at analyse time.
    #[serde(default, deserialize_with = "de_template_overrides")]
    pub template_overrides: IndexMap<Symbol, TemplateSpec>,
    /// Optional: override the low-logic-level (`0`) voltage expression the exposed nodes' `-ic` renders.
    /// Falls back to the `--logic-low` CLI default, then to [`LogicVoltages::default`]'s `"0"`. Written
    /// into the emitted Tcl verbatim, so a Tcl variable is as good as a literal.
    #[serde(default)]
    pub logic_low: Option<String>,
    /// Optional: override the high-logic-level (`1`) voltage expression, mirroring `logic_low`. Falls
    /// back to the `--logic-high` CLI default, then to [`LogicVoltages::default`]'s `"$VDD"`.
    #[serde(default)]
    pub logic_high: Option<String>,
}

/// The characterisation-template references for a cell (or a drive-strength alias override): the
/// `delay`, `power` and `constrain` template names the `define_cell` emitter attaches. Structural
/// only — each name is taken verbatim from the spec, never generated; an absent field is `None`.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TemplateSpec {
    #[serde(default, deserialize_with = "de_opt_symbol")]
    pub delay: Option<Symbol>,
    #[serde(default, deserialize_with = "de_opt_symbol")]
    pub power: Option<Symbol>,
    #[serde(default, deserialize_with = "de_opt_symbol")]
    pub constrain: Option<Symbol>,
}

/// The voltage expressions the Liberate arcs' `-ic` renders for the two logic levels (`low` for `0`,
/// `high` for `1`). Each string is written into the emitted Tcl verbatim, so a Tcl variable (`$VDD`) is
/// as good as a literal (`0`) — these are user-supplied Tcl VALUE fragments, not name fields, so this
/// holds `String` rather than the `Symbol` the rest of the model uses for names.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogicVoltages {
    pub low: String,
    pub high: String,
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
    pub fn of(&self, level: bool) -> &str {
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
/// derive kebab-cases the variants, so the tokens `transition` and `hidden` name the classes on both the
/// CLI (`--when=<CLASS>`) and in the spec (`when = ...`) — one token table, shared by both surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, clap::ValueEnum)]
pub enum ArcClass {
    /// The `define_arc` delay/transition arcs: an input edge on a related pin driving an output edge.
    Transition,
    /// The hidden (internal-power) arcs: an input toggle that settles without changing any output.
    Hidden,
}

/// The set of arc classes whose `-when` arcs are also emitted, on top of the always-emitted general
/// arcs. `Default` is the EMPTY set — only the general arcs, no `-when`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ArcClasses {
    transition: bool,
    hidden: bool,
}

impl ArcClasses {
    /// Every class selected.
    pub const ALL: Self = Self {
        transition: true,
        hidden: true,
    };

    /// Whether `class`'s `-when` arcs are also emitted.
    pub fn contains(self, class: ArcClass) -> bool {
        match class {
            ArcClass::Transition => self.transition,
            ArcClass::Hidden => self.hidden,
        }
    }

    /// The field-wise union of two sets: a class is selected iff either set selects it.
    pub fn union(self, other: Self) -> Self {
        Self {
            transition: self.transition || other.transition,
            hidden: self.hidden || other.hidden,
        }
    }
}

impl FromIterator<ArcClass> for ArcClasses {
    fn from_iter<I: IntoIterator<Item = ArcClass>>(iter: I) -> Self {
        let mut set = Self::default();
        for class in iter {
            match class {
                ArcClass::Transition => set.transition = true,
                ArcClass::Hidden => set.hidden = true,
            }
        }
        set
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

#[derive(Debug, Error)]
pub enum ModelError {
    #[error("cannot parse spec: {0}")]
    Spec(#[from] toml::de::Error),
    #[error("cell name list must be non-empty")]
    EmptyName,
    #[error("duplicate cell name {name:?} used by more than one cell")]
    DuplicateCellName { name: Symbol },
    #[error("cell {cell:?}: duplicate input pin {pin:?}")]
    DuplicateInput { cell: Symbol, pin: Symbol },
    #[error("cell {cell:?}: pin {pin:?} is both an input and an output")]
    InputOutputClash { cell: Symbol, pin: Symbol },
    #[error("cell {cell:?}: internal signal {pin:?} clashes with a declared input or output name")]
    InternalClash { cell: Symbol, pin: Symbol },
    #[error("cell {cell:?}, output {output:?}: variable {var:?} is neither a declared input nor an output of this cell")]
    UnknownVar {
        cell: Symbol,
        output: Symbol,
        var: Symbol,
    },
    #[error("cell {cell:?}: async pin {pin:?} is not a declared input")]
    AsyncNotInput { cell: Symbol, pin: Symbol },
    #[error("cell {cell:?}: clock pin {pin:?} is not a declared input")]
    ClockNotInput { cell: Symbol, pin: Symbol },
    #[error("cell {cell:?}: template override alias {alias:?} is not a declared cell name")]
    UnknownTemplateOverride { cell: Symbol, alias: Symbol },
    #[error("cell {cell:?}: exposed node {node:?} is not a declared internal signal")]
    ExposeNotInternal { cell: Symbol, node: Symbol },
    #[error("cell {cell:?}: duplicate exposed node {node:?}")]
    DuplicateExpose { cell: Symbol, node: Symbol },
}

/// A signal (output **or** internal) after analysis: its function, the variables it references, and
/// the feedback/state variables among them (a signal-name reference = a delayed/feedback value).
#[derive(Debug)]
pub struct AnalysedOutput {
    pub name: Symbol,
    /// The parsed function, regenerated from the minimised BDD when the rewrite changed it.
    /// DISPLAY-ONLY — analysis reads the shared BDD map, never this field.
    pub expr: BoolExpr,
    pub vars: BTreeSet<Symbol>,
    /// Signal names (outputs then internals) referenced by this function — its feedback/state — in
    /// the cell's signal order.
    pub feedback: Vec<Symbol>,
}

/// A cell after validation/analysis.
#[derive(Debug)]
pub struct AnalysedCell {
    pub name: Vec<Symbol>,
    pub inputs: Vec<Symbol>,
    pub outputs: Vec<AnalysedOutput>,
    /// Internal state variables: driven state signals with no external pin. Referenceable by any
    /// function; never an arc source or target. Relay/alias internals are folded away by the
    /// state-space minimisation in [`Cell::analyse`], so only genuine-memory internals survive here.
    pub internals: Vec<AnalysedOutput>,
    /// The internal nodes named in the spec's `expose` list, in declared order, carried verbatim from
    /// `Cell::expose` (validated to be declared internal signals). Some may later be folded away by the
    /// state-space minimisation — see [`AnalysedCell::exposed_signals`] for the survivors in this view.
    pub exposed: Vec<Symbol>,
    pub async_pins: Vec<Symbol>,
    /// The transition arcs derived for the cell's outputs, precomputed once by the shared machine pass
    /// ([`crate::logic::analysis::analyse_machine`]) and consumed by the arcs emitter.
    pub arcs: Vec<Arc>,
    /// The whole-cell internal-power ('hidden') arcs — single input toggles that settle but leave every
    /// output unchanged — precomputed once by the shared machine pass
    /// ([`crate::logic::analysis::analyse_machine`]) and consumed by the arcs emitter.
    pub hidden_arcs: Vec<HiddenArc>,
    /// The cell's static leakage states — the settled seed states of the machine exploration —
    /// precomputed once by the shared machine pass
    /// ([`crate::logic::analysis::analyse_machine`]) and consumed by the arcs emitter.
    pub leakage: Vec<LeakageState>,
    /// Detected order-dependent hazards — pairs whose settled state depends on which edge lands first
    /// (empty for confluent cells). A detected hazard, sibling to `oscillation`; the constraints that
    /// avoid it are generated separately into `constraints`. See [`crate::logic::hazard`].
    pub order_dependence: Vec<OrderDependence>,
    /// Detected oscillation hazards — pairs (or single toggles) that drive a periodic, non-settling
    /// cycle (empty for ordinary combinational or self-holding cells). See [`crate::logic::hazard`].
    pub oscillation: Vec<Oscillation>,
    /// Declared clock input pins (`clock = [...]`). See [`crate::logic::confluence`].
    pub clock_pins: Vec<Symbol>,
    /// The constraints generated to avoid the cell's detected hazards (setup/hold and non_seq). Emission
    /// is gated by the CLI flag or `constraint_arcs_declared`; the kind of each constraint follows the
    /// declared clock.
    pub constraints: Vec<Constraint>,
    /// Whether the cell opted in to constraint-arc emission (`constraint_arcs = true`).
    pub constraint_arcs_declared: bool,
    /// The arc classes whose `-when` arcs are also emitted (per-cell `when` unioned with the global
    /// `--when`), read by the arcs emitter. One general arc per transition — a related pin's edge driving
    /// an output pin's edge — is always emitted, without a `-when` line, regardless of this set; a
    /// selected class adds that class's `-when` arcs on top, so an arc can appear both with and without
    /// its condition. Raw carry — analysis never reads it.
    pub when: ArcClasses,
    /// Each signal's state-table regions, precomputed once and cached in `signals()` order (outputs
    /// then internals), so emitters don't rebuild the BDDs per call site.
    pub regions: Vec<crate::logic::regions::StateRegions>,
    /// Whether the cell holds state: at least one of its minimised signals is a state variable — a
    /// signal on a dependency cycle ([`crate::logic::resolve::state_variables`]). The Liberate arc
    /// emitter gates the `-ic` initial condition on it: Liberate discards the `-prevector` simulation
    /// instead of carrying its settled values into the measured vector, so a cell with memory starts
    /// that vector from state the prevector was supposed to establish and did not, and `-ic` states the
    /// start condition outright. A combinational cell has no state to lose and gets no `-ic`.
    pub state_holding: bool,
    /// The cell's behavioural edge classification ([`crate::logic::edge::EdgeArcs`]): the per-node edge
    /// seams (`captures`), the per-arc `-type edge` labels (`labels`) — the field the Liberate arc emitter
    /// reads to type each arc — the cell-level set of internal non-seam master nodes folded away
    /// (`folded`), and the read-gate factorisations recognised across the cell's outputs (`derived`),
    /// which the Liberty, Verilog and state-table emitters read to render a read-gated register as its own
    /// internal node. Default (empty) when the cell opted out (`no_edge_collapse`). Computed purely from
    /// the already-explored machine — it never alters the exploration.
    pub edge: crate::logic::edge::EdgeArcs,
    /// The exploration budget counter that stopped the machine pass, or `None` when the machine was
    /// explored in full. Set ⇒ nothing was derived from the machine: `arcs`, `hidden_arcs`, `leakage`,
    /// `order_dependence`, `oscillation` and `constraints` are all empty and `edge` is the default, so
    /// the CLI reports the cell instead of emitting arc-free artifacts for it.
    pub unexplored: Option<crate::logic::machine::ExplorationLimit>,
    /// The cell-wide characterisation-template references (delay/power/constrain) carried verbatim from
    /// the spec for the `define_cell` emitter. `None` when the cell declares no `template`. Raw carry —
    /// analysis never reads or synthesises it.
    pub template: Option<TemplateSpec>,
    /// Per-drive-strength-alias template overrides carried verbatim from the spec, keyed by an alias
    /// from `name`. Merged per-field over `template` by the `define_cell` emitter. Keys are validated
    /// against the declared names in [`Cell::analyse_signals`]; raw carry otherwise.
    pub template_overrides: IndexMap<Symbol, TemplateSpec>,
    /// The voltage expressions the Liberate arcs' `-ic` renders for the two logic levels, resolved from
    /// the cell's `logic_low`/`logic_high` overrides (falling back to [`LogicVoltages::default`]). Raw
    /// carry — analysis never reads it, like `template`.
    pub voltages: LogicVoltages,
    /// The arc view of this cell: the same cell analysed with its exposed nodes preserved as model
    /// coordinates, so an arc can drive one (`-ic`) and observe it (`-vector`). Present only when the
    /// cell exposes something, and itself never carrying a further view.
    ///
    /// Exposure is arcs-only. The cell holding this field is the MODEL view — minimised to the outputs
    /// alone, exactly as a cell that exposes nothing — and it is the one the Liberty, Verilog,
    /// statetable and `define_cell` emitters render, so those artifacts are unaffected by what a cell
    /// exposes. Only the Liberate arc emitter reads the view here, through
    /// [`AnalysedCell::arc_view`].
    ///
    /// The two views cannot share one machine exploration, because their machines have different
    /// coordinates: an exposed node is a state variable of the arc view's machine and not of the model
    /// view's. So the arc view's `arcs`, `hidden_arcs` and `edge.labels` are keyed on ITS `arc.start`
    /// minterms — carrying a column per exposed node, which is what the arcs emitter's arc identity
    /// reads — while the model view's own machine derivations feed [`crate::emit::liberty`],
    /// [`crate::emit::statetable`] and [`crate::emit::verilog`] over the minimised coordinates. Each
    /// view therefore runs its own pass. The model view's machine sees only the exposed nodes that
    /// survived its own minimisation ([`AnalysedCell::exposed_signals`], empty in the usual case where
    /// the release folds them away), which costs it nothing: it never emits an exposure.
    pub exposed_view: Option<Box<AnalysedCell>>,
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

    /// The view the Liberate arcs are emitted from: [`AnalysedCell::exposed_view`] where the cell
    /// exposes internal nodes, else the cell itself. Every other emitter reads the cell directly.
    pub fn arc_view(&self) -> &AnalysedCell {
        self.exposed_view.as_deref().unwrap_or(self)
    }

    /// The exposed nodes surviving in this view, in declared order. `exposed` keeps every declared
    /// node (they are preserved through the state-space minimisation), while this keeps only those that
    /// also survive the outputs-only minimisation reflected in `internals`.
    pub fn exposed_signals(&self) -> impl Iterator<Item = &Symbol> {
        self.exposed
            .iter()
            .filter(|e| self.internals.iter().any(|s| s.name == **e))
    }

    /// Each signal paired with its cached state-table regions, in `signals()` order (outputs then
    /// internals).
    pub fn signal_regions(
        &self,
    ) -> impl Iterator<Item = (&AnalysedOutput, &crate::logic::regions::StateRegions)> {
        self.signals().zip(self.regions.iter())
    }
}

impl Spec {
    /// Validate cross-cell name uniqueness, then analyse every cell.
    ///
    /// The union of all cells' name lists must contain no name twice: a collision would emit duplicate
    /// Liberty `cell()` groups, duplicate Verilog modules and conflicting `define_arc` trailers. Intra-cell
    /// duplicates are already deduped by `de_name_list`, so the set-insert here catches inter-cell
    /// collisions (an alias colliding with another cell's name included). The per-cell analyses then run
    /// in parallel, matching the single machine pass minted per cell in [`Cell::analyse`].
    pub fn analyse(&self) -> Result<Vec<AnalysedCell>, ModelError> {
        self.analyse_with(&ExplorationBudget::default())
    }

    /// [`Spec::analyse`] with an explicit exploration budget for every cell — the CLI's entry point,
    /// carrying the `--max-candidates` / `--max-states` ceilings.
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
    /// an exploration stopped by one of them leaves every machine-derived field empty with the counter
    /// recorded in [`AnalysedCell::unexplored`].
    pub fn analyse_with(&self, budget: &ExplorationBudget) -> Result<AnalysedCell, ModelError> {
        let analysed = self.analyse_signals()?;

        // One-shot state-space rewrite: mint the cell's single builder, build every signal's BDD once,
        // and run the minimisation (identical-δ dedup + guarded relay/alias fold, alternated to a
        // fixpoint). It rewrites the map in place so every surviving signal is a genuine-memory
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
            return Ok(self.finish_view(analysed, &bdds, &min, budget));
        }

        // The exposed nodes survived the run above, so this view carries them as machine coordinates:
        // the arc view (see `AnalysedCell::exposed_view`). Only plain data — arcs, hazards, regions —
        // comes back out of the map, so the map is free to be minimised further beneath it.
        let arc_view = self.finish_view(analysed, &bdds, &min, budget);

        // Release the exposure and carry the SAME map on to the outputs-only fixpoint, which is where a
        // single outputs-only run would have landed it (the differential gate in
        // `crate::logic::minimise` checks that over every fixture shape). The model view is re-derived
        // from a fresh parse against the twice-minimised map, under the composition of the two runs, so
        // its display expressions are those of an exposure-free analysis.
        let released = crate::logic::minimise::minimise_state_space(
            &mut bdds,
            &order,
            &crate::logic::minimise::Preserved::outputs(outputs),
        );
        let mut model =
            self.finish_view(self.analyse_signals()?, &bdds, &min.then(released), budget);
        debug_assert!(
            arc_view.exposed_view.is_none(),
            "the arc view is analysed with the exposure already applied and never carries a view of its own"
        );
        model.exposed_view = Some(Box::new(arc_view));
        Ok(model)
    }

    /// Complete one view of the cell over `bdds` — the minimised map `min` reports the rewrite of, back
    /// to `view`'s parse-time functions. Recomputes each surviving signal's metadata, explores the
    /// machine once and copies its derivations in, then caches the region view and the state-holding
    /// verdict.
    ///
    /// This is everything downstream of the minimisation, and it runs once PER VIEW over that view's own
    /// signals: a cell that exposes internal nodes carries two of them (see
    /// [`AnalysedCell::exposed_view`]), whose machines differ in their coordinates.
    fn finish_view<B: Brand, C: ManagerCell + Send + Sync>(
        &self,
        mut view: AnalysedCell,
        bdds: &BTreeMap<Symbol, Bdd<B, C>>,
        min: &crate::logic::minimise::Minimised,
        budget: &ExplorationBudget,
    ) -> AnalysedCell {
        recompute_signal_metadata(&mut view, bdds, min);

        // Build the cell's state machine once and derive both its transition arcs and its hazards from
        // the shared exploration over the minimised model: the two detected hazards (order-dependence,
        // oscillation) and the constraints — setup/hold, non_seq — generated to avoid them. Clock
        // suppression and emission gating are applied downstream.
        // The opt-out (`no_edge_collapse`, also set for every cell by the global `--no-edge-collapse`)
        // gates the classify() call itself, not just its result — no wasted work when collapse is off.
        let analysis =
            crate::logic::analysis::analyse_machine(&view, bdds, !self.no_edge_collapse, budget);
        view.arcs = analysis.arcs;
        view.hidden_arcs = analysis.hidden_arcs;
        view.leakage = analysis.leakage;
        view.constraints = analysis.constraints;
        view.order_dependence = analysis.order_dependence;
        view.oscillation = analysis.oscillation;
        view.edge = analysis.edge;
        view.unexplored = analysis.unexplored;

        // Cache each signal's state-table regions once, in `signals()` order, from the shared folded
        // BDDs, so downstream emitters don't rebuild the BDDs per call site, and record whether the
        // minimised model holds any state at all — both read the same cyclic classifier.
        view.regions = derive_regions(&view, bdds);
        view.state_holding = holds_state(&view);

        view
    }

    /// Validate the cell and parse its functions into the pre-minimise [`AnalysedCell`]: every signal's
    /// parse-time support and feedback classification, with all derived analysis fields
    /// (arcs/hidden_arcs/leakage/order_dependence/oscillation/constraints/regions) still empty. The
    /// state-space rewrite and machine/region passes are layered on by [`Cell::analyse`].
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
            order_dependence: Vec::new(),
            oscillation: Vec::new(),
            clock_pins: self.clock.clone(),
            constraints: Vec::new(),
            constraint_arcs_declared: self.constraint_arcs,
            when: self.when,
            regions: Vec::new(),
            state_holding: false,
            edge: Default::default(),
            unexplored: None,
            template: self.template.clone(),
            template_overrides: self.template_overrides.clone(),
            voltages: LogicVoltages::from_options(
                self.logic_low.as_deref(),
                self.logic_high.as_deref(),
            ),
            exposed_view: None,
        };
        Ok(analysed)
    }
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
pub fn recompute_signal_metadata<B: Brand, C: ManagerCell>(
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

/// Whether the minimised model holds state: at least one signal is a state variable — one on a
/// dependency cycle, as classified by [`crate::logic::resolve::state_variables`] over the recomputed
/// feedback — the same pure-graph classifier [`derive_regions`] reads to mark a region hysteretic.
fn holds_state(cell: &AnalysedCell) -> bool {
    let signals: Vec<&AnalysedOutput> = cell.signals().collect();
    !crate::logic::resolve::state_variables(&signals).is_empty()
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
    use espresso_logic::{bdd_builder, expr};

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
when = ["hidden", "transition"]
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
        // Not flagged as an arbiter (Q→M is a one-way dependency, no mutual cycle).
        assert!(cell.oscillation.is_empty());
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
        let err = parse_spec(s).unwrap().analyse().unwrap_err();
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
        let err = parse_spec(s).unwrap().analyse().unwrap_err();
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
            constraint_arcs: false,
            no_edge_collapse: false,
            when: ArcClasses::default(),
            template: None,
            template_overrides: IndexMap::new(),
            logic_low: None,
            logic_high: None,
        };
        let spec = Spec { cells: vec![cell] };
        let err = spec.analyse().unwrap_err();
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
        let cells = parse_spec(s).unwrap().analyse().unwrap();
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
    fn the_model_view_is_what_an_expose_free_analysis_yields() {
        // Exposure is arcs-only: the view the Liberty, Verilog and statetable emitters read must be the
        // fully-minimised model, signal for signal and expression for expression.
        let exposed = analyse_one(&c_element_src(r#"expose = ["QN"]"#));
        let plain = analyse_one(&c_element_src(""));

        let names = |c: &AnalysedCell| c.signals().map(|s| s.name.clone()).collect::<Vec<_>>();
        assert_eq!(names(&exposed), names(&plain));
        for (a, b) in exposed.signals().zip(plain.signals()) {
            assert_eq!(a.expr, b.expr, "signal {} display expression", a.name);
            assert_eq!(a.vars, b.vars, "signal {} support", a.name);
            assert_eq!(a.feedback, b.feedback, "signal {} feedback", a.name);
        }
        assert_eq!(exposed.state_holding, plain.state_holding);
        assert_eq!(exposed.regions.len(), plain.regions.len());
        assert_eq!(exposed.arcs.len(), plain.arcs.len());
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

    #[test]
    fn exposure_changes_the_arcs_and_nothing_else() {
        // The arcs-only claim, as an invariance of THIS binary rather than against a recorded baseline:
        // analyse each fixture twice, once exposing and once not, and every artifact but the arcs comes
        // out the same text — Liberty, the Verilog primitive's state table, and `define_cell`. The arcs
        // are where the difference lands, as the exposed node's own column.
        for (with, without, node) in exposure_pairs() {
            let exposed = analyse_one(&with);
            let plain = analyse_one(&without);
            assert_eq!(exposed.exposed, [Symbol::from(node)]);
            assert!(plain.exposed.is_empty());

            let cell = exposed.repr_name();
            assert_eq!(
                crate::emit::liberty::cell_liberty(&exposed),
                crate::emit::liberty::cell_liberty(&plain),
                "cell {cell}: exposure reaches the Liberty"
            );
            assert_eq!(
                crate::emit::verilog::cell_verilog(&exposed),
                crate::emit::verilog::cell_verilog(&plain),
                "cell {cell}: exposure reaches the Verilog"
            );
            assert_eq!(
                crate::emit::define_cell::cell_define_cell(&exposed),
                crate::emit::define_cell::cell_define_cell(&plain),
                "cell {cell}: exposure reaches define_cell"
            );

            // The arcs differ in exactly one way: every block of the exposing run lists the node among
            // its columns, and no block of the other run does.
            let opts = crate::emit::arcs_tcl::ArcsTclOptions::default();
            let arcs = |c| crate::emit::arcs_tcl::cell_arcs_tcl(c, opts);
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
        }
    }

    #[test]
    fn define_cell_never_declares_an_exposed_node() {
        // `define_cell` declares the cell's PINS, and an exposed internal is not one however many arc
        // columns it earns — including the DFF's master, which survives the model view as a signal.
        for (with, _, node) in exposure_pairs() {
            let cell = analyse_one(&with);
            let tcl = crate::emit::define_cell::cell_define_cell(&cell);
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
                cell.state_holding,
                cell.arc_view().state_holding,
                "cell {}: the two views disagree on state_holding",
                cell.repr_name(),
            );
            holding += usize::from(cell.state_holding);
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
