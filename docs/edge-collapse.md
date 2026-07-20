# Behavioural per-arc edge classification

How cellsmith decides, for every arc a cell can present, whether that arc is a **capture**, a
**release** (a latch opening), or plain **combinational** propagation — and re-expresses the capturing
ones as edge seams across the emitted artifacts. Classification reads only the cell's already-explored
toggle-and-settle behaviour: it observes how each node reacts to a single input toggle from every
reachable stable state and infers the arc's category from those observations, never from the topology
of the source equations. This is a **post-exploration** re-expression, not a new analysis: the state
machine stays the source of truth for every derived behaviour (arcs, hazards, constraints, regions),
and classification only chooses the *form* an arc is annotated in, never what it does.
`state-machine-arc-engine.md`, `hazard-detection.md`, and `state-table-regions.md` cover the passes
whose output this one reads; `state-space-minimisation.md` covers the model rewrite that runs before
all of them.

## 1. The three categories

Classification is **per arc**, not per node. There is no register verdict on a node: a single output
pin can carry arcs of all three categories at once (an async-reset flop does), and every arc is decided
independently of every other.

1. **CAPTURE** (edge). A clock edge makes the node capture-and-hold a value, and the held value is then
   independent of the clock's LEVEL until that clock's next edge. This is the flop seam.
2. **RELEASE / OPENING** (edge). A clock edge takes a latch from OPAQUE to TRANSPARENT, so data that
   changed while the latch was closed is transmitted to the node **by the clock edge**. The delivered
   value then TRACKS its data rather than holding.
3. **COMBINATIONAL**. A data change propagating to the node while the latch is already transparent —
   ordinary propagation, no clock edge involved.

Categories 1 and 2 are distinct internally — they differ in what the delivered value does afterwards,
hold versus track — but both are timing arcs measured from a clock edge, and Liberate has one token for
both: they emit `-type edge`. A latch therefore has **no capture but a real edge arc**, its opening; it
is not timing-invisible. A **conditioned** release (a clock edge reaching an output only through a
second, currently-open latch) is the same category, with its condition carried in the arc's `-when`:
conditioning never reclassifies an arc.

## 2. Where it runs

`classify` (`src/logic/edge.rs`) is the **last** step of `Cell::analyse`, after the state-space
minimisation rewrite, the machine/hazard pass, and the per-signal region cache have all already run.
It takes the shared `Machine` and is strictly **read-only**: it re-walks the exploration with
`machine::toggle`/`machine::settle` — exactly mirroring `arcs::derive`'s per-node walk — mutates no
BDD, and feeds nothing back into the machine, the arcs, or the hazard detectors. Its entire output is
one field, `AnalysedCell::edge` (`EdgeArcs`): the per-node captures, the per-arc `-type` labels of the
cell's clock-related delay arcs (keyed by the arc's `(output, clock, direction)` identity in the arc
pipeline), and the cell-level set of internal capture-less masters folded away.

The **candidates** are every output (so a combinational output is considered and simply keeps no edge
arc) plus every internal state variable that is not itself an output.

Everything below is derived **behaviourally**, from observed machine toggle-and-settle transitions —
never from the shape of an equation, and never by branching on a declared input class. An async pin
need not be declared to be handled: its effect is classified from its own observed moves
(`forcing_pins`). The characterisation is consequently **implementation-style invariant**: the
NAND-implemented `NDLAT` / `NDFF` / `NHPIPE` fixtures in `src/logic/edge.rs` characterise identically to
their pass-transistor twins `DLAT` / `DFF` / `HPIPE` — same arcs, same covers, same captures.

## 3. The decision pipeline

Per candidate node, over the aggregated walk observations:

1. **SEED by CONTENT.** A `(clock, direction)` is seeded when the edge carries *content* over all its
   firings, changed or not: two firings from equal non-clock input projections deliver different values
   (state content), or a pin outside the eliminated set changes the delivered value (pin content). The
   ELIMINATED set is the non-clock inputs whose toggle moves the node — coexisting combinational arcs
   (async resets, latch data). They contribute no edge content but never disqualify the clock.
2. **LEVEL-INDEPENDENCE VETO** (`pinned_by_clock_levels`). A clock is vetoed on the node when some cube
   of CLOCK LITERALS ALONE pins the node to a constant, that clock's literal being necessary to the
   pinning. In such a phase the clock LEVEL alone decides the node and any captured content is
   irrelevant: this is the **clock-gate class**, combinational by nature. It is what keeps `ICG`'s and
   `ICM`'s `GCLK` arcs `-type combinational` — a gated clock is neither a capture nor a release.
3. **CAPTURE RULE**, per arc and independent of every other arc: keep `(clock, direction)` iff the
   direction CHANGED the node in some firing (a real effect) **and** the delivered phase is QUIET
   (`phase_quiet`) — no live data reaches the node inside that phase, so the delivered value holds
   independently of the clock level. Quietness is judged with the node's behaviourally-classified
   forcing pins exempted (a reset asserting across a closed phase is a coexisting combinational arc,
   not transparency), and with co-resident clock movers admitted unless they change a phase-wide
   **carrier** the node tracks (a mux switch between held values is not transparency; tracking a live
   carrier is).
4. **PER-ARC LABELS, SOURCED FROM THE ARC PIPELINE.** Every timing arc is one of the delay arcs
   `arcs::derive` observed — nothing else exists to label, and internal nodes, which carry no delay
   arc, are never labelled. Each clock-related arc key
   `(output, clock, direction)` is labelled **Capture** when the capture rule kept that direction,
   **nothing** when the clock is vetoed on the node (a level-acting clock's arcs stay combinational —
   `RDFF`'s clock-declared reset is the witness: its assert pins the node by level and emits
   `-type combinational`, never a release), and **Release** otherwise — the arc's existence attests the
   change, and an edge that moves the node without holding afterwards released a latch. The veto is
   judged for every clock whose edge moved the node, seeded or not.

Everything a candidate presents that falls in none of these is left to `super::arcs` as an ordinary
combinational data arc.

### Masking is not a separate mechanism

There is no rule that suppresses an arc a clock edge "should" have had. An arc exists exactly where a
single-input toggle between reachable stable states actually changes an output, so an edge whose effect
never reaches an output produces **no arc to label** — masking is already done by the time
classification runs, and it is done by the machine walk rather than by any edge-specific reasoning:

- A **flop master's release** is stopped by its closed slave. The master opens on one clock phase, but
  the slave is opaque in that phase, so the output never moves and the arc pipeline observes nothing;
  only the phase that reaches the output survives, as `Q`'s single capture arc.
- A **gated clock's** edge is cancelled by the gating condition it controls: the falling edge that would
  close the gate arrives in a state the gate's own condition has already settled, so the toggle leaves
  the output where it was and no arc is derived for it.

A `-when` condition is the opposite case and is **not** masking: a conditioned arc is still an arc.
Conditioning on data, on state, on another clock's level or on clock phase narrows the context the arc
is measured in — it never suppresses the arc nor moves it to another category.

## 4. Capture synthesis

A capture is the next-state value the node latches at an active edge, synthesised per edge from that
edge's `(pre-state, post-value)` samples through the `regions` FR cover pipeline. The witnessed
on-samples are the ON-set, the witnessed off-samples the OFF-set, and every unwitnessed projection a
**don't-care**: the capture is the ON-set generalised by incompletely-specified minimisation, so it
lands on the underlying function rather than only the sampled pre-states — reachability need not
exercise every projection. The generalised on-set is total (its off is the exact complement, empty
hold).

The capture is recorded **verbatim** as an ordinary combinational function. An inverting flop captures
`!D`; a toggle flop's master captures `!Q`; these are just the functions they are, never special-cased
— inversion carries no dedicated attribute or branch. A projection carrying both an on- and an
off-sample under the current header is a **conflict**; the synthesis escalates the header from tier 1
(inputs plus capture-less candidates) to tier 2 (inputs plus every candidate), and a conflict that
survives tier 2 drops the arc.

## 5. Off-edge synthesis

The **off-edge** is the node's behaviour while its clocks are stable: quiescent hold plus any set/clear
forcing. It is synthesised over the **non-clock inputs** from the stable-state samples, grouped by
projection and split by the clock set's phase vector. A projection forced high in a phase becomes a set
(`on`), forced low a clear (`off`), and a projection that merely holds (or is unobserved) lands in
`hold` and drops out of the columns — a data input that never forces the node does not appear. A
phase-AGREED forcing makes each clock a don't-care in every forcing cube, so the clocks drop out of the
cover support; a phase-CONDITIONED one keeps its gating clock pinned to the forcing level (`CLK*R`).

## 6. Fold

Folding is decided at **cell level** (`EdgeArcs::folded`), after classification. An internal
capture-less master is folded away when nothing surviving still references it: no capture or off-edge
cover names it, no other surviving signal's raw function depends on it, and it was not pulled back into
a tier-2 header. A folded master's own pin, UDP primitive, and statetable row are elided from every
artifact, leaving only the edge form; its internal-power characterisation via its primary-input hidden
arcs is unchanged. A **toggle flop** is self-fed, so its ring cannot fold: it decomposes into two
opposite-edge captures instead, each keeping the other as a live reference.

**Known follow-up (contained, fold rule only).** A set of *mutually-referencing* capture-less nodes is
not group-folded: each is "referenced elsewhere" by the other, so the per-node rule strands both as
surviving level internals. `NDFF`'s NAND master pair `M`/`Mn` is the witness — its arcs, covers and `Q`
characterisation are invariant against the pass-transistor `DFF`, only the folding differs (asserted,
not hidden, in `edge_nand_master_slave_matches_the_pass_gate_flop`). The fix is to group-fold a set of
mutually-referencing capture-less nodes when the set as a whole has no reference from outside it.

## 7. Emission

`AnalysedCell::edge` is consumed downstream, but the **per-arc label is read by exactly one emitter**.
`EdgeArcs::captures` and `EdgeArcs::folded` shape the behavioural models; `EdgeArcs::labels` types the
Liberate arcs and nothing else:

- **Liberate** — `src/emit/arcs_tcl.rs` is the only consumer of `labels` and the only emitter that
  types arcs. Each delay arc looks up its own `(output, related clock, clock direction)` key: a capture
  arc and a release arc both render `-type edge`; a declared-async related pin takes precedence with
  `-type async`; an unlabelled arc — a transparent-mode data change, or a level-vetoed clock — stays
  `-type combinational`. No visibility filtering is needed: the label domain is the emitted arcs
  themselves, and outputs never fold.
- **Liberty** — the joint `statetable` carries the capture's edge rows and drops a folded master's row.
  It does not read `labels` at all: release arcs need nothing here, because the statetable's level rows
  already model the latch and Liberate derives the timing from the Tcl.
- **Verilog** — the sequential UDP is written in edge-triggered form for captures and elides folded
  masters; likewise independent of `labels`, the level rows already carrying the latch.

## 8. Retained restrictions

These bounds are deliberate and user-approved:

- **Declared clocks only.** A cell with no declared clock carries no edge arc.
- **Capture conflict ⇒ no capture.** A capture conflict surviving the tier-2 header drops the arc.
- **Never-changing ⇒ no arc.** A direction that never changes the node is neither a capture nor a
  release.
- **Surviving non-state internals** are not candidates.
- **Explored machine required.** Classification needs an explored machine, so a cell wider than
  `MAX_MACHINE_VARS` (= 22) gets no annotation. Lifting the 22-variable cap is a separate, tool-wide
  change.
- **No group fold.** A set of mutually-referencing capture-less nodes — a NAND master pair — is not
  folded (§6): the characterisation is unaffected, only the internals stay visible.

## 9. The exploration is unchanged

Classification is read-only by construction, and a permanent regression guard
(`edge_classification_changes_only_the_edge_annotation` in `src/logic/edge.rs`) checks it directly:
for both the DFF and ICM fixtures, analysing the same spec with `no_edge_collapse` forced true and
false produces byte-for-byte identical `AnalysedCell` fields for everything except `edge` — `arcs`,
`hidden_arcs`, `leakage`, `order_dependence`, `oscillation`, `constraints`, and `regions` included.
Classification changes only which form an arc is annotated in; the state-machine exploration, the
discovered arcs and their prevectors, and hazard detection never see it.

## 10. Opt-outs

Classification is on by default. A cell opts out individually with `no_edge_collapse = true` in its
TOML table; the global `--no-edge-collapse` CLI flag does the same for every cell in the run, applied
before analysis so it is indistinguishable from each cell having declared the field itself. Either
way, `edge` stays the default empty `EdgeArcs` and every arc is emitted in its combinational form.
