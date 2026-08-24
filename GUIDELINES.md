# Contributing guidelines

The standing conventions for working on cellsmith, kept in one place so they need not be
rediscovered from the code or restated on every change. They apply to every contribution,
human or automated; the automated development tooling reads this file verbatim and treats
each rule as binding. If a change genuinely cannot be made without breaking one of these,
that is a decision to raise and settle explicitly — not to work around quietly — because a
rule that keeps obstructing real work is a rule that needs fixing here, not ignoring.

## What cellsmith is

A general-purpose, any-gate Cadence Liberate arc generator — a command-line tool, not a
library. It reads a TOML cell spec, analyses the cell's behaviour with BDDs, and emits
Liberate Tcl (`define_arc`, hidden/internal-power arcs, `define_leakage`, constraint arcs)
alongside Liberty, Verilog and statetable artifacts.

It is **not** NCL/QDI-specific. cellsmith lives inside the Pulsar ecosystem, which is
QDI-focused, but the tool itself characterises arbitrary gates; do not narrow a design to
the NCL ancestry. The `[lib]` target exists only so the benchmarks can link against the
crate — it is not a public API, so version and reason about the tool from its CLI and
config surface, not from library-level compatibility.

## Efficiency and correctness come first

Every rule below about how the code is shaped and tested is a way of serving those two here; none
is an end in itself. Where such a rule and one of these appear to conflict, the conflict is a defect
in the rule — raise it and fix it here rather than working around it. The settled conventions —
British spelling, and the Git section — are choices rather than trade-offs and stand outside this.

Code that compromises efficiency to keep an order is wrong: the order has to earn what it costs.
And a test that checks the output matches a reference is not establishing correctness — the test
has to reason about why that output is right.

Read each rule for what it protects. A legitimate, strong and verifiable reason outranks the
letter of a rule that did not anticipate it; a weak one does not, and "the rule does not quite
cover my case" is not a reason at all. Where such a reason holds, amend the rule here in the same
change: outranking a rule's letter and fixing the rule are one act, not a licence to proceed alone.

## Language and style

- British spelling everywhere it is written by us — identifiers, comments and user-facing
  output (`analyse`, `serialise`, `behaviour`, `optimisation`).

## Writing comments, docs and messages

This covers everything we write in prose: code comments, doc comments, the README, the
CHANGELOG, and commit and PR messages.

- **Describe what the thing is and does, in the present.** State the behaviour and the
  reason for it. Don't explain something by contrast with an approach that was considered
  and dropped — the reader is looking at what exists, not at its history — and don't narrate
  the intermediate states a value moved through to reach the one that matters.
- **Say it plainly; drop the superlatives.** No "powerful", "robust", "simply", "just",
  "significantly", "seamlessly". They add length and confidence without adding information,
  and a claim of ease often turns out to be untrue on the case that doesn't fit. State the
  fact and let it stand.
- **Don't dress up routine detail.** An ordinary implementation choice doesn't need to be
  announced as though it were the point; give it the weight it actually carries.
- **Write for the reader's context, not your own.** Introduce a term specific to this
  project or otherwise non-obvious before leaning on it, and where a name belongs to a
  particular tool or theory, say so. You finish a change holding a great deal of context the
  reader does not have; the prose has to bridge that gap rather than assume it away.
- **State a borrowed framework before its vocabulary, and gloss it afterwards.** Where an
  argument rests on established machinery — a lattice and its fixed points, a rewrite system,
  an automaton — say what it ranges over here, what the operation is, and whose result is
  being relied on, before any term belonging to it appears; then restate it in the terms of
  the cell or the pass at hand. The formal statement is what makes a word like *greatest* mean
  something and be checkable. The restatement is what lets a reader who does not know the
  framework follow the argument. Either half alone fails: a term with no framework behind it
  is a claim nobody can verify, and a framework with no restatement is vocabulary nobody can
  use.
- A comment that only restates the code earns nothing. Spend the words on a non-obvious
  invariant and why it holds.

## Read the documentation before guessing

When you need to know how something behaves — a crate, a library, an external tool, a file
format, a standard, an algorithm — read its documentation first, rather than inferring the
behaviour from source, a type signature, a prototype or observed output. This holds whatever
the thing is: the rustdoc for a dependency, the Liberate/Liberty reference for the formats we
emit, the spec for an input we parse. A signature or a sample tells you what happens to work;
the documentation tells you what is actually guaranteed, and the gap between the two is where
the subtle bugs live.

## Emission order is not significant

The order in which the tool emits its output commands carries no meaning, and nothing in
the design or the tests should depend on it. Two runs that produce the same arcs, hazards
and constraints are equivalent even if the blocks come out in a different order, and a run
is free to pick any equally-good representative where a choice is arbitrary.

The one place ordering *is* real is where a format the output feeds gives a position
meaning — a `-vector`'s characters line up with the `-pinlist`, and Liberty's statetable
rows are matched first-to-last. Those are external constraints and must be preserved. An
order kept for any other reason answers to "Tests assert properties" below, which states
what a motivation has to look like. Given a choice between clearer or faster code that
reorders output and more awkward code that keeps the order stable, take the former.

## Correctness means semantic equivalence, not identical bytes

A change is correct when it produces the same set of arcs, hazards and constraints with the
same content — not when the generated files are byte-for-byte identical.

- Verify by comparing the *set of emitted records*, not the raw file. A gate that demands an
  empty byte-diff is really demanding that the output never change, which is a stronger
  claim than the tool actually makes; don't write one.
- The `examples/*` files are generated artifacts. Regenerate them with the tool; never edit
  one by hand, and never regenerate one just to make a diff go away.

## Tests assert properties

A test states a property of the thing under test and asserts it semantically — on the
records, the values or the fields that carry the claim, against an expectation derived from
the input. A fixture is chosen so the expected content is forced by its own behaviour, and
the assertion holds for every output the tool is free to produce; a test that fails on a
second, equally valid output is asserting a rendering, not a property, and is not a valid
test.

Running the code twice and asserting the two results agree does not test behaviour: it
passes whenever the two paths are self-consistent, including when both are wrong, so it
cannot fail for the reason anyone cares about. The same holds whatever the currency —
rendered text, values, counts or multisets — and byte-identity or whole-text comparison is
this defect in its plainest form. State instead what the output must contain, and derive it
from the input the test controls.

Making a test cheaper is never a reason to assert less; the economies that justify a design choice
in the code justify nothing here. And a test resting on equality often asserts no property at all —
it asserts immutability, that the output has not changed since someone last looked. Immutability is
a fact about the previous run, not about the thing under test. Checking that the output matches a
reference does not establish correctness; the test has to reason about why that output is right.

The property a test asserts is one the code or its documentation states. Where neither states
it, the claim belongs there first — otherwise the test pins something nothing promised, which
is how a test comes to fail on an output that was always valid.

An order is where this bites hardest. Keeping one needs a motivation stated where the order is,
and that motivation has to be checkable — a reason nobody could test settles nothing. Two kinds
qualify. A reader outside this crate requires the position: a consuming format, a tool that parses
by column. Or the order makes the algorithm cheaper than the unordered one would be — sorting to
dedup, so equality walks two sequences instead of comparing every element against every other, is
a reason by itself.

What does not qualify is an order bought only for its own stability. "Held sorted so the
report is stable", "ordered so the fold is well-defined", "sorted so the comparison passes"
each justify the order by our own use of it and answer to nothing. Where nothing outside
reads it and nothing cheaper comes of it, the order is free and nothing may depend on it. The
same reasoning runs the other way: where determinism is not required, take the cheaper
variant — an unstable sort over a stable one. What this rule protects is the freedom not to
pay for determinism nobody needs.

A correspondence between the parts of one output is a property like any other, and so is a
relation between two runs where that relation is the specified behaviour. Examples, not the
list: Liberate reads a block's `-pinlist`, `-vector` and `-ic` as positional columns of one
argument, so those three agreeing on the pin order a run produced is the contract with that
reader; a switch documented to change one named thing leaves the rest alone. Neither fixes
which output the run picks, so both hold for every output the tool is free to produce. A test
resting on a two-run relation names the delta it permits, and the content both sides share is
pinned by a direct test of its own.

## Proper use of support types

The libraries we build on already model the things this tool reasons about. Reaching for a general
container where a library type says the same thing is one mistake whichever type it is: holding a
name in a `String` and holding an assignment in a `BTreeMap` are the same error, not two.

- **A name is an `espresso_logic::Symbol`, not a `String`.** Every name field — pins, nodes, cells,
  templates.
- **A mapping from a name to a Boolean value is an `espresso_logic::Minterm<Symbol>`, not a
  `BTreeMap<Symbol, bool>` or a `HashMap<Symbol, bool>`** — and not a map over a struct whose only
  field is a `bool`, which is the same shape wearing a name. A `Minterm` is a row of tri-state
  values keyed by variable, and a variable it does not define is **by definition** a don't-care. A
  partial assignment therefore needs no separate type and no absent-key convention: the partial
  case is what a `Minterm` already is.
- **A set of tri-state rows over a shared header is a `Cover`.** Not a vector of rows beside a
  vector of column names — a row's header travels with the row, and splitting the two is what
  forces every reader downstream to re-pair them by position.

## Use the library's operations, not your own

The Boolean data model here is espresso-logic's, and its operations come verified. A hand-rolled
equivalent is a second implementation of the same thing that nobody tests, so it is a defect whether
or not it currently works.

Before writing a loop over cubes, rows, variables or literals, find the operation in the library.
**Where no single call does the job, look for the composition of two before concluding there isn't
one** — that step is where this goes wrong. The hand-rolled code in this crate was not written in
ignorance of the library; it was written after failing to find one method that did the whole job,
when the answer was two calls: wrap a minterm in a one-cube cover and build it; take the
disagreement and project it.

Some the crate has needed, so the loop is never written again: `Minterm::value_of` to read one
variable, absent meaning don't-care; `project_to` / `project_to_labels` to re-home onto another
variable set — silently, so it also deletes any check that a variable was defined; `disagreement`,
`is_subset_of`, `is_superset_of`, `is_disjoint_with`, `hamming_distance` and the Kleene `&`, `|`,
`^`, `!` operators to compare and combine two rows; `Cube::expand_to` to expand don't-cares into
full assignments; `BddBuilder::build_cover` to build a BDD from cubes, a single minterm included by
way of a one-cube cover; `Cover::to_expr_by_index` to render a cover as an expression;
`Cover::merge` and `Cover::extend` to assemble a multi-output cover.

Rendering positionally at the sink is not this defect. Liberty's `statetable` and the Verilog UDP
are columnar formats, so projecting a row onto its columns as it is written is the format's demand.
Carrying that projection back up the pipeline as the way the data is held is the defect.

## Prefer real types to stand-ins

- A closed set of kinds is an `enum`, not a string token. Model it so that picking the
  variant *is* the classification, and an impossible combination can't be built in the first
  place.
- **A tuple must not escape the scope that makes it.** Within a function a tuple is an ordinary
  Rust value: a map's own key and value, `enumerate`, `zip`, a chain of iterator adaptors, a buffer
  of pairs assembled to satisfy an external signature, an iterator of pairs collected into a map.
  Use them freely. What a tuple may not do is outlive that scope — it is not a struct field, not a
  map key, and not the element type of a collection that is kept or handed back. A value with more
  than one component that lives beyond the scope building it is a value of ours and takes a name:
  a struct, or an enum variant, with named fields.

  A concrete function does not return a tuple. Where a generic one does — `zip`, `partition`,
  `enumerate` — the components are whatever the caller supplied and there is nothing to name; where
  ours does, each component means something particular here and the tuple leaves the reader to work
  it out. The exception is a return whose tuple is std's protocol rather than our data: an
  `Iterator<Item = (K, V)>` is the shape `collect` consumes to build a map, and a method offering
  that shape keeps it.

  The same holds for a struct or an enum variant that tells its own components apart by position —
  give them names. A single-component newtype is unaffected; one value has no positions to confuse.
  Where two components share a type the cost is immediate — a transposition compiles and changes
  meaning silently — but that is the sharpest case, not the reason: `.0` and `.1` tell a reader
  nothing about what they hold.

  In a corner case the question to ask is whether the shape encodes something meaningful and whether
  it can drift. A pair that carries a fact of this domain, and that two readers could come to
  disagree about, takes a name however local it is. A buffer whose shape a foreign signature
  dictates carries nothing of ours and cannot drift; leave it.
- Choose each collection for how it is used: a hash map or set where the access is
  membership, lookup or grouping; an ordered container only where its order has one of the two
  motivations "Tests assert properties" sets out above. Don't reach for an ordered container just
  for its iteration order when nothing reads that order, and don't hold a field at a weaker type
  because a better one would sort differently.
- Negation is an ordinary Boolean function. Don't give inversion a special case — no
  dedicated flag, guard or branch for it.

## Structure is held until the edge

A value stays structured until the moment it is written. `Display` is how a thing reaches the
output; a function that returns rendered text has handed its caller a string to parse instead of
the value it was holding. Build the expression, the record or the block, and let the sink render
it — that is what keeps a rendering decision in one place and lets everything upstream of the sink
still reason about the value.

## Lints point at missing design

The tree builds clean under `clippy -D warnings`, and it stays that way by fixing what the
lint names rather than silencing it. A `too_many_arguments` or `type_complexity` warning is
telling you a type is missing — introduce it.

Reach for `#[allow]` only as a genuine last resort, and when you do, say in place what the
lint wanted, why the proper form can't be had here, and what the attribute buys. Suppressing
a lint to preserve a shortcut is the same move as loosening a test until it passes. And a
clean clippy run is only evidence of quality once you've checked the green isn't coming from
a suppression.

## Work out the rule before adding a guard

If some input misbehaves and the temptation is to add a check that rejects or narrows it,
first understand why. A guard bolted on to dodge a limitation you haven't pinned down tends
to be wrong, and a rule that turns away otherwise-valid input is a real behaviour change —
raise it as one rather than letting it accrete silently at the edge of the code.

## Git

A commit message ends with the description of the change — never an AI-attribution line or
session trailer.

A pull request is squash-merged and its branch is kept: `master` reads as one commit per
delivered change, and the step-by-step history that produced it stays on the branch. That is
why the branch outlives the merge — `gh pr merge --squash`, never with `--delete-branch`.

The record being kept is the pull request's own branch. Branches a tool cuts for its own
working copies, and the commits it folds together before handing the work back, are
scaffolding; removing them is ordinary cleanup, and a squashed unit of work arriving on the
branch is that tool doing its job. Read the rule for what it protects — the history someone
would go looking for after the merge — rather than as a prohibition on every ref that ever
existed. Restoring scaffolding that was correctly cleaned up costs as much as losing the
record would.
