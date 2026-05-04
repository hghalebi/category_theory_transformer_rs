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
