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

## Language and style

- British spelling everywhere it is written by us — identifiers, comments and user-facing
  output (`analyse`, `serialise`, `behaviour`, `optimisation`).
- Name fields carry `espresso_logic::Symbol`, not `String`.

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
rows are matched first-to-last. Those are external constraints and must be preserved. The
test for any ordering rule you meet is: does the consuming format impose it, or did
we impose it on ourselves to make a check easier? Only the former survives. Given a choice
between clearer or faster code that reorders output and more awkward code that keeps the
order stable, take the former.

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

Where the parts of one output must correspond, that correspondence is itself a property and
is asserted as one. The pin order a run produced is the same order in its `-pinlist`, its
`-vector`'s characters and its `-ic`'s columns; a Liberty statetable's rows carry their
values in the order that table's own header declares its pin columns. Neither fixes which
order the run picks — the claim is that the parts agree with each other — so it holds for
every output the tool is free to produce, and it needs no second run to state.

One relation between two runs does survive, because there the relation is the documented
behaviour: where a switch is specified to change one named thing and nothing else, or two
spellings of one cell are specified to classify alike, the agreement is the property. Such a
test names the delta it permits, and the content both sides share is pinned by a direct test
of its own.

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
  membership, lookup or grouping; an ordered map only where the iteration order is claimed by
  an external format (see above). Don't reach for an ordered container just for its
  iteration order when nothing reads that order, and don't hold a field at a weaker type
  because a better one would sort differently.
- Negation is an ordinary Boolean function. Don't give inversion a special case — no
  dedicated flag, guard or branch for it.

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
