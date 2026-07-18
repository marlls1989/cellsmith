# Behavioural edge-sensitivity classification

How cellsmith decides, for every node of a cell, whether that node is an **edge-triggered register**,
a **level-sensitive** element, or plain combinational logic — and re-expresses the edge-triggered ones
as edge seams across the emitted artifacts. Classification reads only the cell's already-explored
toggle-and-settle behaviour: it observes how each node reacts to a single input toggle from every
reachable stable state and infers the sequential shape from those observations, never from the
topology of the source equations. This is a **post-exploration** re-expression, not a new analysis:
the state machine stays the source of truth for every derived behaviour (arcs, hazards, constraints,
regions), and classification only chooses the *form* a register is annotated in, never what it does.
`state-machine-arc-engine.md`, `hazard-detection.md`, and `state-table-regions.md` cover the passes
whose output this one reads; `state-space-minimisation.md` covers the model rewrite that runs before
all of them.

## 1. Where it runs

`classify` (`src/logic/edge.rs`) is the **last** step of `Cell::analyse`, after the state-space
minimisation rewrite, the machine/hazard pass, and the per-signal region cache have all already run.
It takes the shared `Machine` and is strictly **read-only**: it re-walks the exploration with
`machine::toggle`/`machine::settle` — exactly mirroring `arcs::derive`'s per-node walk — mutates no
BDD, and feeds nothing back into the machine, the arcs, or the hazard detectors. Its entire output is
one field, `AnalysedCell::edge` (`EdgeSensitivity`): the recognised `EdgeRegister`s and the cell-level
set of internal level-sensitive masters folded away.

The **candidates** are every output (so a combinational output is considered and simply classified as
`none`) plus every internal state variable that is not itself an output. A pin declared both a clock
and an async pin is treated as async-only.

## 2. The transition predicate

For each candidate the walk aggregates, over every reachable stable state, how one input toggle moves
the node. A toggled input is one of three kinds — declared clock, declared async pin, or data input —
and each contributes a different observation:

- **data input.** A data toggle that changes the node is recorded per clock phase: for each declared
  clock, whether the node moved while that clock was low and whether it moved while it was high.
- **clock.** A clock toggle records the `(pre-state, post-value)` sample under that clock's active
  edge, and whether the value changed.
- **async pin.** Async pins are excluded from the hold discipline; their effect is folded into the
  off-edge synthesis (§4), not the level/register classification.

A candidate is then classified:

- **level** — some data input is transparent to the node in one phase of a clock that actually gates
  it (a clock whose own toggle moves the node) but not the other phase. That **phase-asymmetric**
  change is the signature of a transparent latch following its data during one phase. A level node
  emits its ordinary hysteretic regions and takes no annotation; an internal level node is a foldable
  master (§5). Restricting the transparency test to clocks that gate the node stops a uniform reset
  reading as transparent against an unrelated clock it is independent of.
- **register** — exactly one declared clock's edge(s) change the node, and no data input is
  transparent to it: the node holds across data changes while the clock is stable and changes only
  phase-asymmetrically across that one clock's edges. The active edge set is `Rise`, `Fall`, or both
  (a **dual-edge** register, when both edges of the same clock change it).
- **none** — combinational (no clock changes it), or changed by two or more distinct declared clocks:
  no annotation.

## 3. Capture synthesis

A register's **capture** is the next-state value it latches at an active edge, synthesised per edge
from that edge's `(pre-state, post-value)` samples through the `regions` FR cover pipeline. The
witnessed on-samples are the ON-set, the witnessed off-samples the OFF-set, and every unwitnessed
projection a **don't-care**: the capture is the ON-set generalised by incompletely-specified
minimisation, so it lands on the underlying function rather than only the sampled pre-states —
reachability need not exercise every projection. The generalised on-set is total (its off is the exact
complement, empty hold).

The capture is recorded **verbatim** as an ordinary combinational function. An inverting flop captures
`!D`; a toggle flop's master captures `!Q`; these are just the functions they are, never special-cased
— inversion carries no dedicated attribute or branch. A projection that carries both an on- and an
off-sample under the current header is a **conflict**; the synthesis escalates the header from tier 1
(inputs plus non-level candidates) to tier 2 (inputs plus every candidate, level masters re-included),
and a conflict that survives tier 2 falls the node back to level, no annotation.

## 4. Off-edge synthesis

The **off-edge** is the node's behaviour while the clock is stable: quiescent hold plus any async
set/clear. It is synthesised over the **non-clock inputs** from the stable-state samples, grouped by
projection and split into the clock's two phases. A projection forced high in a phase becomes an async
set (`on`), forced low becomes an async clear (`off`), and a projection that merely holds (or is
unobserved) lands in `hold` and drops out of the columns — a data input that never forces the node
does not appear.

A declared-async pin whose forced effect **differs between the two stable clock phases** blocks the
whole annotation and falls the node back to level (**behavioural F2**). This subsumes the master-slave
async-agreement guarantee without any topology matching: a reset that clears both latches agrees
across the two phases and is recognised as an async clear on the register; a reset that clears only one
latch disagrees between the phases and self-excludes.

## 5. Fold and the toggle-flop decomposition

Folding is decided at **cell level** (`EdgeSensitivity.folded`). An internal level master is folded
away when nothing surviving still references it: no register capture or off-edge cover names it, and no
other surviving level signal depends on it. A folded master's own pin, UDP primitive, and statetable
row are elided from every artifact, leaving only the register's edge form; its internal-power
characterisation via its primary-input hidden arcs is unchanged.

A **foldable** master is one that is a pure input function in every pre-edge state. A **toggle flop**
does not meet that bar: its master is self-fed (it captures a function of the register's own prior
state, not of a data input), so the ring cannot fold. It **decomposes into two opposite-edge
registers** instead — the master becomes a register on one edge and the slave a register on the other,
each keeping the other as a live reference in its capture. A **cross-coupled NAND** pair shares one
folded master, recognised as two registers over the same captured value and its inverse.

A **dual-edge** register carries two captures (`Rise` first, then `Fall`); both of its clock→node arcs
are relabelled `-type edge` in the Liberate output.

## 6. Emission

`AnalysedCell::edge` is consumed downstream by the three emitters, each re-expressing a recognised
register in its own edge form and eliding any folded master:

- **Liberty** — the joint `statetable` carries the register's edge rows; a folded master's row is
  dropped.
- **Verilog** — the sequential UDP is written in edge-triggered form.
- **Liberate** — the register-capturing `define_arc` output carries `-type edge`.

How each does so is an emission concern, not part of classification.

## 7. What the behaviour subsumes

Because classification checks the actual settled behaviour rather than matching a topology, the
guarantees an explicit master-slave recogniser would enforce as structural guards fall out for free:

- a genuine **hold** across data changes is checked on real transitions, so a node that follows an
  input during a phase never presents as a register (subsuming the transparent-path / hold guards);
- **async agreement** is the phase-agreement rule of §4;
- a **non-monotone or oscillating** hold never presents the required stable behaviour on the walk, so
  it self-excludes without a separate monotonicity guard.

## 8. Retained restrictions

These bounds are deliberate and user-approved:

- **Declared clocks only.** A cell with no declared clock is never annotated.
- **One clock per node.** A node changed by two or more declared clocks is not annotated — a two-clock
  master-slave stays level.
- **Capture conflict ⇒ level.** A capture conflict that survives the tier-2 header falls the node back
  to level.
- **Never-changing ⇒ no register.** A node that never changes on a clock is not a register on it.
- **Clock/async overlap.** A pin declared both a clock and an async pin is treated async-only.
- **Async must be declared** to be excluded from the hold discipline; an undeclared forcing input is
  read as data.
- **Surviving non-state internals** are not candidates.
- **Explored machine required.** Classification needs an explored machine, so a cell wider than
  `MAX_MACHINE_VARS` (= 22) gets no annotation. Lifting the 22-variable cap is a separate,
  tool-wide change.

## 9. The exploration is unchanged

Classification is read-only by construction, and a permanent regression guard
(`edge_classification_changes_only_the_edge_annotation` in `src/logic/edge.rs`) checks it directly:
for both the DFF and ICM fixtures, analysing the same spec with `no_edge_collapse` forced true and
false produces byte-for-byte identical `AnalysedCell` fields for everything except `edge` — `arcs`,
`hidden_arcs`, `leakage`, `order_dependence`, `oscillation`, `constraints`, and `regions` included.
Classification changes only which form a recognised register is annotated in; the state-machine
exploration, the discovered arcs and their prevectors, and hazard detection never see it.

## 10. Opt-outs

Classification is on by default. A cell opts out individually with `no_edge_collapse = true` in its
TOML table; the global `--no-edge-collapse` CLI flag does the same for every cell in the run, applied
before analysis so it is indistinguishable from each cell having declared the field itself. Either
way, `edge` stays the default empty `EdgeSensitivity` and every node is emitted in its level form.
