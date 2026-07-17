# Master-slave latch → edge-register collapse

How cellsmith recognises a cell modelled as two opposite-phase level-sensitive latches in series on
the same declared clock — a master feeding a slave — and re-expresses that pair as a single
edge-triggered register. This is a **post-exploration** re-expression, not a new analysis: the
two-latch model stays the source of truth for every derived behaviour (arcs, hazards, constraints,
regions), and the collapse only changes the *form* a register is annotated in, never what it does.
`state-machine-arc-engine.md`, `hazard-detection.md`, and `state-table-regions.md` cover the passes
whose output this one reads; `state-space-minimisation.md` covers the model rewrite that runs before
all of them.

## 1. Where it runs

Recognition (`src/logic/collapse.rs::recognise_edge_registers`) is the **last** step of
`Cell::analyse`, after the state-space minimisation rewrite, the machine/hazard pass, and the
per-signal region cache have all already run over the shared per-cell BDD map. It reads that map, the
post-minimise `signals()` order, the output set, and the cell's declared clock pins — nothing else —
and is strictly **read-only**: it mutates no BDD and feeds nothing back into the machine, the arcs, or
the hazard detectors. Its entire output is one new field, `AnalysedCell::edge_registers`, a
`Vec<EdgeRegister>` in `signals()` order.

## 2. Recognition rule

A signal `s` with transition function `δ_s` is a **latch** with respect to a declared clock `c` at
transparency phase `p` iff `c` appears in `δ_s`'s support, the transparent cofactor `T_s = δ_s|c=p`
does **not** reference `s`, and the hold cofactor `H_s = δ_s|c=¬p` **does**. Recognition is
**declared-clocks-only**: a cell with no declared clock never collapses, and a signal that is
latch-shaped with respect to two or more declared clocks is rejected outright — it is not classified
as a latch on either.

A candidate pair `(m master, s slave)` is nominated when both are latches on the same clock with
opposite phases and `m` appears in `vars(T_s)`, then confirmed by five guards, all exact BDD
identities computed in the one shared per-cell builder (so `==` is a genuine function-equality test):

- **G2** `m ∉ vars(H_s)` — the master feeds only the slave's transparent path, never its hold.
- **G3** `s ∉ vars(δ_m)` — no reverse dependency of the master on the slave.
- **G5** monotone hold for both latches: `H|x=0 ∧ ¬H|x=1 == false` in each latch's own variable.
- **F1** `T_s|m=0 == H_s|s=0 ∧ T_s|m=1 == H_s|s=1` — the value captured through the master equals the
  value the slave itself would have held.
- **F2** `H_m|m=0 == H_s|s=0 ∧ H_m|m=1 == H_s|s=1` — master and slave hold the same value.

A master is **foldable** away when it is internal (not an output pin), `s` is its sole surviving
consumer, and it is not itself a slave (no same-clock opposite-phase latch feeds it). A confirmed pair
whose master is foldable annotates the slave with `folded_master = Some(m)` and a capture function that
substitutes the master's own transparent cofactor in for `m`. A confirmed pair whose master is instead
already annotated as a register in its own right annotates the slave with `folded_master = None`,
keeping `m` as a live reference in the capture. Recognition runs this pairing as a worklist over the
signals to a fixpoint, so a slave whose master is not yet resolved (foldable or annotated) waits for a
later round. A chain head that can neither fold nor be annotated — an exposed, tapped, multi-consumer,
or undeclared-clock master — propagates nothing, leaving the whole cell unchanged.

Each recognised `EdgeRegister` carries the slave's name as `node`, the paired `clock`, the active
`edge` (`Rise` for a transparent-high slave, `Fall` for transparent-low), a `cols` column set (the
first-appearance union of the capture's and off-edge's own columns), the `capture` function as
combinational state-table regions (empty hold — it never references the clock), and the `off_edge`
function as state-table regions carrying the async set/clear and quiescent-hold behaviour.

## 3. The N−1 edge-element invariant

Recognition never increases signal count and, wherever a chain of `N` alternating-phase latches
confirms all its pairwise guards, folds it down to `N − 1` edge elements: every master but the last
gets elided, and the boundary node — the one node that is both somebody's slave and somebody else's
master — survives as its own register instead of vanishing.

The simplest case is `N = 2`: a plain master/slave DFF (`M` transparent-low, `Q` transparent-high on
`CLK`) collapses to the single rising-edge register `Q`, folding `M` away entirely — `2 − 1 = 1`.

## 4. The ICM shared-boundary case

The ICM synchroniser's `CLKA` chain is `sela1 → sela2 → enA`, three latches in series (`N = 3`), so
the invariant predicts `2` surviving registers — and that is exactly what recognition produces, but
not by folding greedily from the front. `sela2` is simultaneously the **slave** of `sela1` and the
**master** of `enA`: a greedy fold would absorb `sela2` into `enA`'s capture and leave nothing marking
the shared boundary. The `foldable` guard's "not itself a slave" clause exists precisely to prevent
that — `sela2` is rejected as a foldable master for `enA` because it is itself latch-paired with
`sela1` on the same clock. So `sela2` is instead annotated as its **own** rising-edge register, folding
`sela1` away (`folded_master = Some(sela1)`); and `enA` is annotated as a falling-edge register whose
master does not fold (`folded_master = None`), keeping `sela2` as a live reference in `enA`'s capture.
`sela1` and `sela2`'s original two-latch shape both vanish from the emitted count, but `sela2` itself
remains a genuine coordinate — the chain's `3` latches become `2` registers, `sela2` and `enA`, with
`sela1` alone elided. The `CLKB` chain (`selb1 → selb2 → enB`) mirrors it.

## 5. The exploration is unchanged

Recognition is read-only by construction, and a permanent regression guard
(`collapse_changes_only_the_edge_registers_field` in `src/logic/collapse.rs`) checks it directly: for
both the DFF and ICM fixtures, analysing the same spec with `no_edge_collapse` forced true and false
produces byte-for-byte identical `AnalysedCell` fields for everything except `edge_registers` —
`arcs`, `hidden_arcs`, `leakage`, `order_dependence`, `oscillation`, `constraints`, and `regions`
included. The collapse changes only which form a recognised register is annotated in; the state-machine
exploration, the discovered arcs and their prevectors, and hazard detection never see it.

## 6. Opt-outs

Collapse is on by default. A cell opts out individually with `no_edge_collapse = true` in its TOML
table; the global `--no-edge-collapse` CLI flag does the same for every cell in the run, applied before
analysis so it is indistinguishable from each cell having declared the field itself. Either way,
`edge_registers` stays empty and the cell's two-latch model is emitted exactly as written.

The annotation itself is consumed downstream by the Liberty, Verilog, and Liberate-arc emitters, each
re-expressing a recognised register in its own edge form and eliding any folded master; how each does
so is an emission concern, not part of recognition.
