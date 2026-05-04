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
