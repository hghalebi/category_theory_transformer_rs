# Editorial Style Guide

This book teaches category theory, Rust, and tiny machine-learning systems
through one concrete codebase. The prose should feel like a careful instructor
walking beside the reader, not like a checklist generated from headings.

## Core Shape

Every authored chapter should begin with the practical problem the chapter
solves. From there, move through the real code, explain why the code exists, and
then connect it to the category-theory shape. The reader should always know
which layer they are looking at: Rust syntax, ML or software behavior, or the
mathematical structure.

The existing three-lens pattern is still useful, but it should not become a
machine rhythm. Use it when the section is doing real explanatory work. Between
those lens labels, write in full paragraphs that connect ideas instead of
stacking isolated one-line statements.

## Terminology Contract

Use public phrases, Rust names, and category-theory names deliberately.

| Public phrase | Rust name | Category-theory reading |
| --- | --- | --- |
| training pairs | `TrainingExample` values inside `TrainingSet` | product objects |
| model state | `Parameters` | object transformed by an endomorphism |
| probabilities | `Distribution` | object produced by `Softmax` |
| typed transformation | `Morphism<Input, Output>` | morphism |
| update step | `TrainStep` | endomorphism on `Parameters` |

When a chapter is explaining intuition, the public phrase is fine. When it is
explaining code, use the Rust name. When it is explaining structure, use the
category-theory name and point back to the Rust handle.

## Learning Path

The book should manage difficulty instead of pretending the material is easy.
Each major chapter should make the learner's path visible:

1. Activate prior knowledge.
2. Introduce one new hard idea.
3. Give a tiny worked example before the formal abstraction.
4. Explain the Rust shape.
5. Explain the invariant or boundary the code protects.
6. Explain the ML or software meaning.
7. Explain the category-theory meaning without overclaiming.
8. Prompt self-explanation.
9. End with retrieval practice.
10. Connect to the next chapter.

Use this section pattern when a concept is difficult enough to need scaffolding:

```text
What You Already Know
Why This Exists
Smallest Concrete Example
Rust Shape
What The Code Protects
ML Meaning
Category-Theory Meaning
Self-Check
Next Step
```

Do not force every small paragraph into this exact heading structure. Do make
sure every chapter contains the pattern in substance.

## Evidence-Backed Practice

The practice design should be grounded in learning-science sources, not only in
editorial preference. Use the learning-design references in
`book/src/references.md` as the source base for these decisions.

The practical rule is:

```text
worked example -> self-explanation -> faded example -> retrieval -> transfer
```

A chapter should first show one complete example. Then it should ask the reader
to explain why the example works. Then it should remove part of the support:
a blank in a trace, a missing explanation, a changed input, or a small code
edit. Finally it should ask the reader to transfer the same idea to a nearby
boundary.

This matters because rereading can feel fluent without proving durable
understanding. Retrieval practice should therefore ask for recall, explanation,
and application before the reader checks the answer key.

## Reference-Backed Rewrite Loop

Every major chapter should be rewritten from sources, code, and reader friction
rather than from memory alone. Use the [References](src/references.md) chapter
map before editing a chapter.

For each chapter pass:

1. Name the central question the chapter must answer.
2. Choose the Rust, ML, category-theory, and learning-design sources attached to
   that chapter.
3. Re-read the repository file or example the chapter teaches.
4. Draft the chapter as a learner path: intuition, smallest example, Rust shape,
   ML meaning, category-theory meaning, practice.
5. Critique the draft through four lenses:
   - ML teaching lens: is the intuition concrete, accurate, and free of hidden
     framework magic?
   - Visual/tutorial lens: can the reader see the data flow before meeting the
     abstraction?
   - Category-theory lens: are the terms precise, modest, and connected to laws
     only where the code actually supports them?
   - Learner lens: where would a motivated reader become lost, overloaded, or
     unsure what to run next?
6. Rewrite the chapter from the critique.
7. Update chapter references, retrieval practice, glossary links, and exercises
   when the rewrite changes what the reader is expected to know.
8. Run the repository validation gates.

The reference map itself is part of the gate. When a new major chapter is added,
update `scripts/check-chapter-references.py` so the chapter has a central
question, enough external sources, and a bridge from the chapter back to
`references.md`.

The chapter contract manifest is part of the gate. When a core chapter's
source bucket, maturity, central question, runnable command, code evidence, or
practice route changes, update `book/CHAPTER_CONTRACTS.md` and keep
`scripts/check-chapter-contracts.py` green. The manifest is the compact
editorial surface for deciding whether a chapter rewrite is still grounded in
the right sources, code, and exercises.

The chapter scorecard is part of the rewrite loop. Use
`book/CHAPTER_QUALITY_SCORECARD.md` to choose the next weakest dimension in a
chapter before rewriting. Keep `scripts/check-chapter-scorecard.py` green so
the scorecard stays aligned with chapter contracts and does not claim direct
reader evidence before accepted reader reports exist.

The public maturity map is part of the gate. When a core chapter is added or a
chapter moves from sketch to draft or stable draft, update the README maturity
table and `scripts/check-chapter-maturity.py` in the same pass. The table should
tell public readers where feedback is most useful, not merely decorate the
project with status words.

The practice map is also part of the gate. When a chapter, exercise, or answer
key entry changes, update `scripts/check-exercise-alignment.py` so the core
chapter practice map and `exercises/ANSWER_KEY.md` cannot drift apart.

Exercise commands are part of the gate. When a learner-facing exercise command
changes, update `scripts/check-exercise-commands.py` so commands stay
intentional and use runnable project entry points.

Duplicate prose is part of the gate. When major teaching chapters are rewritten,
keep `scripts/check-duplicate-prose.py` green so repeated paragraphs do not
accumulate across sweeps.

The rewrite log is part of the gate too. When a major chapter is rewritten,
update `book/CHAPTER_REWRITE_LOG.md` with the source file, critique before
rewrite, rewrite decision, validation, and the relevant expert and learner
critique lenses. `scripts/check-rewrite-log.py` protects that editorial record.

The completion audit is the guardrail against false finish lines. When the
objective, evidence, validation gate, or missing work changes, update
`book/TEXTBOOK_COMPLETION_AUDIT.md` and keep
`scripts/check-completion-audit.py` green.

The reader feedback loop is part of the gate. When the issue form, review
guide, review packet, reviewer outreach packet, review sprint, feedback wall,
starter issues, feedback inbox, or triage protocol changes, keep
`scripts/check-reader-feedback-loop.py` green so reader reports remain
actionable rewrite inputs.

Direct reader evidence should be recorded in
`community/reader-feedback-inbox.md`. The inbox must distinguish between
accepted reports and templates or proxy signals. Do not count the book as
reader-driven until the sprint has at least five accepted reports across the
Rust engineer, ML engineer, category-theory reader, technical educator, and
beginner-adjacent reader contexts, and at least one report has been converted
into a rewritten section. Keep `scripts/check-reader-feedback-inbox.py` green
so accepted reports cannot silently lose the fields needed for source-backed
rewrites. Run `python3 scripts/check-reader-feedback-inbox.py
--require-sprint-complete` before making any completion claim.
When GitHub issue-form submissions arrive, use
`python3 scripts/collect-reader-feedback-issues.py` to render inbox drafts, then
triage them before counting anything as accepted evidence.
Use `scripts/reader-feedback-status.sh --live` during completion audits to see
the local inbox state, strict completion gate, GitHub surface, and issue intake
in one pass.

Community posts, reader issues, and course reviews may be used as friction
signals. They can tell us where readers get confused, but they should not carry
technical authority. Technical claims must be grounded in the cited papers,
official documentation, course material, or repository code.

Source freshness is an editorial workflow, not a mandatory offline build step.
Run `python3 scripts/check-reference-links-live.py --timeout 10` during a
source-refresh pass to catch moved or unreachable external references. Keep
`python3 scripts/check-reference-links-live.py --self-test` in the full gate so
the parser and checker behavior remain covered without making CI depend on
external websites.

## Cognitive Load

Introduce one new hard idea per section. If the section is teaching Rust syntax,
keep the ML example familiar. If the section is teaching category-theory
vocabulary, keep the Rust code small. If the section is teaching an ML operation,
use already-known Rust patterns.

When a paragraph contains Rust syntax, ML vocabulary, and category-theory
vocabulary all at once, check whether the reader already has enough context. If
not, split it into a worked example followed by the three lenses.

Use signposts such as:

```text
What to notice
Do not worry about training yet
Read this left to right
Self-check
```

These are navigation aids, not apologies for difficulty.

## Worked Examples And Practice

Hard abstractions should move through this progression:

1. A complete worked example.
2. The same example explained.
3. A partially completed example.
4. A reader exercise.
5. A transfer exercise.

The early chapters should show more complete code and explain it directly. Later
chapters may ask the reader to explain why a type, reference, error, or
composition boundary has the shape it has.

Every major chapter should end with retrieval practice:

```text
Recall: ask for the key term or shape.
Explain: ask why the design choice matters.
Apply: ask the reader to use or modify the idea.
```

These questions should make the reader retrieve and reason, not reread passively.

## Diagrams

Diagrams must stay attached to nearby explanation. After a diagram, explain how
to read it. For pipeline diagrams, use the same three readings:

```text
Rust reading
ML reading
Category-theory reading
```

Do not let diagrams float away from code or prose.

## Paragraphs

Prefer paragraphs that carry a complete thought. Short paragraphs are useful
for emphasis, but long runs of sentence fragments make the book feel thin.

Good paragraphs usually answer at least two of these questions:

- what the reader is seeing in the code
- why the pipeline needs this shape
- what invariant is protected
- how this connects to the previous stage
- what category-theory word names the same pattern

When a section starts to read as many separate slogans, combine nearby sentences
into a single explanation.

## Lists

Use lists only when the reader benefits from scanning alternatives, steps, or
named items. A list should not be the default way to write an explanation.

Avoid heading directly into a bullet list. After a heading, first give the
reader a sentence that explains why the list exists and how to use it.

Dense lists are acceptable in `SUMMARY.md`, references, source snapshots,
glossary pages, and exercises where the page is deliberately a reference or
practice surface. In teaching chapters, a dense list should usually become
prose, a diagram, or a smaller list introduced by a paragraph.

## Code Blocks

Every substantial code block should have enough surrounding prose for the reader
to understand why it is there before reading it. Do not rely on the code block
alone to carry the explanation.

When explaining Rust syntax, name the actual ownership, type, trait, or error
boundary involved. When explaining ML behavior, connect the block to prediction,
loss, training, validation, or evaluation. When explaining category theory,
state the object, morphism, product, endomorphism, functor, natural
transformation, monoid, or law being modeled.

## Source Snapshots

Source snapshots are intentionally mechanical. Keep them inside `<details>`
blocks and do not rewrite included code into prose by hand. The book should stay
grounded in the real repository files, while the authored prose around those
snapshots should remain readable.

## Tone

Write directly and concretely. Prefer "this constructor rejects an empty token
sequence before training starts" over "this ensures robustness." Prefer "the
middle type is `Vector`, so `Embedding` can compose with `LinearToLogits`" over
"this enables seamless composition."

The target is precise, friendly technical writing: not decorative abstraction,
not marketing copy, and not a checklist.
