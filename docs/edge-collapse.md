# Behavioural per-arc edge classification

How cellsmith decides, for every timing arc a cell can present, whether that arc is an **edge arc** —
a clock edge whose delivered value depends on retained latch content — or ordinary **combinational**
propagation, and re-expresses the edge arcs as seams across the emitted artifacts. Classification reads
only the cell's already-explored toggle-and-settle behaviour: it observes how each node reacts to a
single input toggle from every reachable stable state and decides each arc's type from those
observations and from the cell's own next-state functions, never from the topology of the source
equations. This is a **post-exploration** re-expression, not a new analysis: the state machine stays the
source of truth for every derived behaviour (arcs, hazards, constraints, regions), and classification
only chooses the *form* an arc is annotated in, never what it does. `state-machine-arc-engine.md`,
`hazard-detection.md`, and `state-table-regions.md` cover the passes whose output this one reads;
`state-space-minimisation.md` covers the model rewrite that runs before all of them.

## 1. One category: the edge arc

Classification is **per arc**, not per node. There is no register verdict on a node: a single output
pin can carry edge arcs and combinational arcs at once (an async-reset flop does), and every arc is
typed independently of every other.

> **A clock toggle that takes a latch from opaque to transparent, and whose resulting output value
> depends on latch content rather than arriving regardless, is an edge arc on that output.**

There is one category. An edge arc emits Liberate `-type edge`; an arc that does not meet the
definition — a data change reaching a node while a latch is already transparent, or a clock acting by
its LEVEL (a clock gate) — carries no label and stays an ordinary combinational data arc. The physical
event behind an edge arc may be a *capture* (the delivered value then holds independent of the clock
level until the next edge, a flop seam) or a latch *opening* (the delivered value then tracks its data);
both are timing arcs measured from a clock edge and both emit the one `-type edge` token, so the
distinction changes nothing in the annotation and is not a second kind of arc. A latch therefore has a
real edge arc, its opening, and is not timing-invisible.

A **conditioned** edge arc — a clock edge reaching an output only through a second, currently-open latch
— is the same category, with its condition carried in the arc's `-when`: conditioning never reclassifies
an arc.

## 2. Where it runs

`classify` (`src/logic/edge.rs`) runs inside `analyse_machine` (`src/logic/analysis.rs`), **after** arc
derivation (`arcs::derive`) and hazard detection (`confluence::detect`) and **before** the per-signal
region cache. The region cache (`AnalysedCell::regions`) is built later still, back in `Cell::analyse`
(`src/model.rs`) once `analyse_machine` has returned — so classification sees the arcs and the machine
but never the region cache. It takes the shared `Machine` and is strictly **read-only**: it re-walks the
exploration with `machine::toggle`/`machine::settle` — exactly mirroring `arcs::derive`'s per-node walk
— mutates no BDD, and feeds nothing back into the machine, the arcs, or the hazard detectors. Its entire
output is one field, `AnalysedCell::edge` (`EdgeArcs`): the per-node captures, the per-arc `-type`
labels of the cell's clock-related delay arcs (keyed by the arc's full
`(output, clock, direction, machine start minterm)` identity in the arc pipeline), and the cell-level
set of internal non-seam masters folded away.

The **candidates** are every output (so a combinational output is considered and simply keeps no edge
arc) plus every internal state variable that is not itself an output.

Everything below is measured only from **fully-determinate** reachable stable states — a state with a
don't-care (uninitialised) state column is arc-ineligible, a don't-care being a *missing* variable never
coerced to 0/1, in the `Minterm` and in BDD evaluation alike. Traversal is untouched: partial states
remain seeds, they are simply never measured from. No machine state is ever perturbed, defaulted or
re-settled under a held value; an oscillating configuration is an invalid state and takes part in no
test. And everything is derived **behaviourally**, from observed toggle-and-settle transitions and the
cell's own next-state functions — never from the shape of an equation, and never by branching on a
declared input class. An async pin need not be declared to be handled: its effect is classified from its
own observed moves (see `forcing_pins`, §5). The characterisation is consequently **implementation-style
invariant**: the NAND-implemented `NDLAT` / `NDFF` / `NHPIPE` fixtures in `src/logic/edge.rs`
characterise identically to their pass-transistor twins `DLAT` / `DFF` / `HPIPE` — same arcs, same
seams, same covers, same folds.

## 3. The decision pipeline

One analysis over the machine's `toggle`/`settle` observations produces both the arc types and the state
model:

> arc typing (generation + propagation, per arc) → per-node seam set `S` (greatest fixpoint) → edge
> functions (uniform header + drop-loop) → off-edge → fold (reachability).

### 3.1 Arc typing — generation and propagation

Each clock-related delay arc, at its full identity `(output, clock, direction, machine start minterm)`,
is typed edge iff its firing changed the output (vacuity), some latch in the output's cone **generates**
on `(clock, direction)`, and the change **propagates** from a surviving source. Generation at the output
itself types the arc directly.

**Generation — a latch going opaque→transparent.** Opacity is read from the live dependency loops at the
eligible stable states. At a stable state the dependency edge `n → m` between state variables is **live**
iff `δ_m`, restricted (`Bdd::restrict_to`) to all of its support *except* `n` at the values they take in
the state, still depends on `n` — the residual is non-constant in `n`. A latch is **opaque** in a phase
iff some eligible stable state of that phase carries a live dependency cycle through it, and
**transparent** iff none does *and* its value **varies** across the phase's eligible stable states. That
variation clause matters: a phase pinned to one constant everywhere — by a reset, or by the toggled
clock's own level — delivers its value regardless of latch content, which is a forcing, not an opening,
so generation never fires into it. A latch **generates** on `(clock, direction)` iff it is opaque at the
source level and transparent at the delivered level.

This lands the hard cases with no per-node witness machinery. In a pass-gate DFF, `δ_Q` at `CLK=0` is
`Q` — a live self-loop, opaque — while at `CLK=1` it is the master `M` with no path back — transparent —
so the slave generates the rise. In a cross-coupled-NAND flop the `Q ↔ Qn` loop is live at `CLK=0` and
broken at every `CLK=1` stable state, so the pair generates the rise identically to the pass-gate twin.
`SETLR` (`Y = !R*(CLK + !CLK*Y)`) generates on the rise: its `CLK=1` phase delivers `!R`, which varies
with the clear, so the phase is live and the latched constant is a value like any other. Its hard twin
(`Y = CLK + !CLK*!R*Y`) pins `Y` to 1 across the whole `CLK=1` phase — no variation — so nothing
generates and it stays combinational. `RDFF`'s clock-declared `R` generates nothing because its `R=1`
phase pins `Q` at 0 everywhere, while its `CLK` arcs are unaffected. The pseudo-latch probe
`Y = CLK*A + !CLK*B + Y*A*B` never sits on a live cycle that opens across an edge, so it never generates
and stays combinational — generation means *behaviourally bistable*, not *self-referential in the
equation*.

**Propagation — restriction-survival from the arc's source.** For an observed arc with post-generation
stable state `s'`, restrict `δ_output` on all of its support *minus* the edge's source variable `W` to
the values they take at `s'`; the arc propagates iff the residual still depends on `W`. The source is the
latch the edge arrived from — the generated latch, or the exposed content donor whose value the change
delivered (`DET`'s rise delivers `L1`, so the source is `L1`). Candidate sources are the clock-associated
latches in the output's cone (the generator and the closer). The test is applied transitively through
intermediate nodes and **per arc**: different firings restrict to different `s'` and decide
independently, which is exactly how the `MASKL` fall splits — with the mask admitting the latch the
residual is `L` (edge), with it masking the latch the residual is constant (combinational). A masked
source never survives the restriction: `ICG`'s `EL` is swallowed by the `CLK*EL` gate at the destination,
and `ICM`'s competing enable is not the toggled clock's associate, so a clock gate's `GCLK` reaches no
surviving source on either edge and stays combinational — the exclusion is causal, a plain absence of any
clock-associated source.

### 3.2 The seam set — the seam fixpoint

A candidate node carries an **edge seam** on `(clock, direction)` iff the arc typing holds **and** the
delivered value holds through the phase, the second computed as a greatest fixpoint over the node's own
seam set `S`. Start `S` at every `(clock, direction)` the node types edge on at some eligible changed
firing, then remove `(clock, direction)` whenever some non-forcing change of the node inside its
delivered phase occurs at a toggle that is **not** itself an edge of `S` — live data, or a non-seam
clock — and iterate until stable. A node with a non-empty `S` is an edge register: its per-edge
next-state functions and off-edge are synthesised into `EdgeArcs::captures`. An empty `S` is a level
node (a latch that merely tracks, or a clock gate). `DCMUX` collapses to level in two steps — its falls
are combinational, so each in-phase fall is a non-seam change and both rises' seams die — leaving a level
model whose two rises still carry `-type edge` labels; `DLAT` empties immediately on live data. `ICG` and
`ICM`'s `GCLK` have no edge arc and therefore no seam, causally.

## 4. Cover synthesis

A capture is the next-state value the node latches at an active edge, synthesised per edge from that
edge's `(pre-state, post-value)` samples through the `regions` FR cover pipeline. The witnessed
on-samples are the ON-set, the witnessed off-samples the OFF-set, and every unwitnessed projection a
**don't-care**: the capture is the ON-set generalised by incompletely-specified minimisation, so it
lands on the underlying function rather than only the sampled pre-states — reachability need not
exercise every projection. The generalised on-set is total (its off is the exact complement, empty
hold).

The synthesis runs over **one uniform header** — all inputs except the keying clock plus every candidate
— for every arc. A single header is sufficient by an impossibility argument rather than a retry: within a
`(clock, direction)` sample group the clock's pre-level is fixed, the header carries every other input
and all state variables, so two samples with equal header projections are the same machine minterm, and
`settle` is deterministic — a projection cannot carry both an on- and an off-sample. Between sample
collection and generalisation, an ordered **drop-loop** shrinks the header a column at a time, keeping a
drop whenever the `(pre-projection → post-value)` samples stay conflict-free after it. The drop test is a
sample-level grouping — no BDD quantification, no second synthesis pass. Drop order is fold-eligibility
order: inputs are never dropped, fold-eligible level internals (the `S = ∅` nodes, settled before
synthesis) are attempted first, edge-form nodes and outputs last, in reverse header order within a class.
This is the stated rationale applied uniformly — a cover prefers columns that survive emission, because
eliminating useless internals is what the post-derivation fold is for — so `generalise` and `regions_from`
run over the surviving columns and cannot choose an internal the fold wants gone. `ICM` forces the
ordering: at its sample states two internals are inter-determined, so an unordered drop-loop could keep
the one that must fold.

The capture is recorded **verbatim** as an ordinary combinational function. An inverting flop's next
state is `!D`; a toggle flop's master's is `!Q`; these are just the functions they are, never
special-cased — inversion carries no dedicated attribute or branch.

## 5. Off-edge synthesis

The **off-edge** is the node's behaviour while its clocks are stable: quiescent hold plus any set/clear
forcing. A node's **forcing pins** are the inputs whose toggle drives the node to a constant regardless
of the clock (a reset, a preset) — classified behaviourally from the node's own moves, never from a
declared class. The off-edge is synthesised over **all** the cell's inputs, the node's clocks included,
from the stable-state samples, grouped by projection and split by the clock set's phase vector. A
projection forced high in a phase becomes a set (`on`), forced low a clear (`off`), and a projection that
merely holds (or is unobserved) lands in `hold` and drops out of the columns — a data input that never
forces the node does not appear. A phase-AGREED forcing makes each clock a don't-care in every forcing
cube, so the clocks drop out of the cover support; a phase-CONDITIONED forcing keeps its gating clock
pinned to the forcing level, which is why the off-edge of a phase-conditioned reset carries the clock
literal (`CLK*R`). Because the clocks are in the header, that literal is available: synthesising over the
non-clock inputs alone could not express it.

## 6. Fold

Folding is decided at **cell level** (`EdgeArcs::folded`), after classification, as a **reachability**
question: does this value still influence an output once collapsed? It is computed as a liveness fixpoint
over the graph of raw-function references among the internal non-seam survivors. The seeds are what must
stay visible: capture-less outputs (never folded) and any candidate named by a surviving capture or
off-edge cover column — the sinks whose raw function is actually emitted. Liveness then propagates along
each live node's own function support (semantic BDD support, never equation shape). An internal non-seam
node folds unless that propagation reaches it; a *mutually-referencing* — or transitively-referencing —
set of such nodes that reaches no sink folds together, because the set as a whole influences nothing a
survivor still names. This single reachability rule covers both the single-node case and the group:
`NDFF`'s NAND master pair `M`/`Mn` and `NHPIPE`'s inner NAND master pair `M1`/`M1n` are capture-less and
mutually referencing, so both fold together — exactly as the pass-transistor `DFF` and `HPIPE` fold their
lone `M`/`M1`, pinned by `edge_nand_master_slave_matches_the_pass_gate_flop` and
`edge_nand_hierarchical_two_clocks_matches_the_pass_gate_pipe`. A folded node's own pin, UDP primitive,
and statetable row are elided from every artifact, leaving only the edge form; its internal-power
characterisation via its primary-input hidden arcs is unchanged. A **toggle flop** is self-fed, so its
ring cannot fold: its master carries a real capture, which excludes it from the non-seam candidate
population regardless of this rule; it decomposes into two opposite-edge captures instead, each keeping
the other as a live reference.

This criterion is deliberately **narrower than the one used during early minimisation**. There,
self-referential loops are preserved on purpose, because they carry the oscillation detection. At
emission that concern does not apply: the only question is whether a value affects the output, so a
self-referential set that reaches no output may be collapsed even though minimisation had to keep it.

## 7. Emission

`AnalysedCell::edge` is consumed downstream, but the **per-arc label is read by exactly one emitter**.
`EdgeArcs::captures` and `EdgeArcs::folded` shape the behavioural models; `EdgeArcs::labels` types the
Liberate arcs and nothing else:

- **Liberate** — `src/emit/arcs_tcl.rs` is the only consumer of `labels` and the only emitter that types
  arcs. Each delay arc looks up its own `(output, related clock, clock direction, start minterm)` key: an
  edge arc renders `-type edge`; a declared-async related pin takes precedence with `-type async`; an
  unlabelled arc — a transparent-mode data change, or a clock acting by its level — stays
  `-type combinational`. No visibility filtering is needed: the label domain is the emitted arcs
  themselves, and outputs never fold.
- **Liberty** — the joint `statetable` carries the edge rows and drops a folded master's row. It does not
  read `labels` at all: a latch opening needs nothing here, because the statetable's level rows already
  model the latch and Liberate derives the timing from the Tcl.
- **Verilog** — the sequential UDP is written in edge-triggered form for the seams and elides folded
  masters; likewise independent of `labels`, the level rows already carrying the latch.

## 8. Retained restrictions

- **Declared clocks only.** A cell with no declared clock carries no edge arc.
- **Never-changing ⇒ no arc.** A direction that never changes the node presents no arc to type.
- **Surviving non-state internals** are not candidates.
- **Explored machine required.** Classification needs an explored machine, so a cell wider than
  `MAX_MACHINE_VARS` (= 22) gets no annotation. Lifting the 22-variable cap is a separate, tool-wide
  change.

## 9. The exploration is unchanged

Classification is read-only by construction, and a permanent regression guard
(`edge_classification_changes_only_the_edge_annotation` in `src/logic/edge.rs`) checks it directly: for
both the DFF and ICM fixtures, analysing the same spec with `no_edge_collapse` forced true and false
produces byte-for-byte identical `AnalysedCell` fields for everything except `edge` — `arcs`,
`hidden_arcs`, `leakage`, `order_dependence`, `oscillation`, `constraints`, and `regions` included.
Classification changes only which form an arc is annotated in; the state-machine exploration, the
discovered arcs and their prevectors, and hazard detection never see it.

## 10. Opt-outs

Classification is on by default. A cell opts out individually with `no_edge_collapse = true` in its TOML
table; the global `--no-edge-collapse` CLI flag does the same for every cell in the run, applied before
analysis so it is indistinguishable from each cell having declared the field itself. Either way, `edge`
stays the default empty `EdgeArcs` and every arc is emitted in its combinational form.
