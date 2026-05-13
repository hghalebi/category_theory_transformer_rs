# Category Theory for Tiny ML in Rust

This is a public working draft. The current edition is published so readers
can learn from it now, run the examples, and send precise feedback. It is not
the completed textbook yet; later passes will keep revising the chapters from
source review, exercises, and direct reader reports.

## First Win

From the repository root, run:

```bash
cargo run --example 01_token_sequence
```

That command turns one sentence into typed training material:

```text
Text
  -> TokenSequence
  -> TrainingPairs
```

The point of starting there is practical. Before the book asks you to care
about category theory, it lets you run a small program and inspect the shape it
prints.

## First Output Transfer Checklist

Use the first run as a reading test. Do not treat the output as a demo banner.
Treat each printed block as evidence for a boundary.

| Printed output | Rust reading | ML reading | Category-theory reading |
| --- | --- | --- | --- |
| `Raw input` | an ordinary `&str` enters the program | text before tokenization | source object before the first transformation |
| `TokenSequence` | a validated domain object built from `TokenId` values | tokenized data the tiny system can inspect | object with named structure |
| `TrainingPairs` | adjacent `Product<TokenId, TokenId>` values | input-target examples for next-token learning | product-shaped training examples |
| `Typed transformation` | the command names the path the code took | text became examples for learning | a short chain of morphisms |

The first win is not that the tokenizer is impressive. It is deliberately tiny.
The win is that you can point at the output and say:

```text
this raw value became this domain object,
then this domain object became training examples,
and the transformation path is visible.
```

That is the reading habit the rest of the book repeats at larger scales.

## Source-Backed Reading Contract

This welcome chapter uses sources to keep the first session practical. Each
source supports one local rule for how the reader should move from the first
command to the rest of the book.

| Source | What the source supports | Local rule in this chapter | Repository evidence |
| --- | --- | --- | --- |
| [How People Learn II](https://www.nationalacademies.org/projects/DBASSE-BBCSS-13-06/publication/24783) | New learning works better when it connects to prior knowledge, learner context, and transfer. | Start from what many readers already know: a function has an input, an output, and a visible transformation. | `fn token_to_position(token_id: usize) -> usize`, `## What You Already Know`, `## Self-check` |
| [Rust By Example](https://doc.rust-lang.org/rust-by-example/index.html) | Small runnable programs make syntax inspectable before the explanation gets abstract. | Make the first proof a command the reader can run before reading theory. | `cargo run --example 01_token_sequence`, `examples/01_token_sequence.rs` |
| [Seven Sketches](https://arxiv.org/abs/1803.05316) | Applied category theory becomes learnable through concrete compositional examples. | Name objects, morphisms, products, composition, and laws only after the reader can point to a tiny typed pipeline. | `Text -> TokenSequence -> TrainingPairs`, `Distribution x TokenId -> Loss`, `Parameters -> Parameters` |

The transfer pattern is:

```text
run one small example -> name the visible boundary -> reuse the reading habit
```

For this chapter, the first command is evidence for a small claim:

```text
Text becomes TokenSequence.
TokenSequence becomes TrainingPairs.
The path is visible in terminal output.
```

It is not evidence that the whole book is easy for every reader yet. That is
why the public review path asks for exact evidence signals when a sentence,
output line, table row, code block, or exercise breaks the learning path.

Then run the guided walkthrough:

```bash
cargo run --bin category_ml
```

That command walks through the larger pipeline: domain objects, morphisms,
composition, prediction, loss, repeated training, functors, monoids, and the
small chain-rule example.

The repository is public at
[github.com/hghalebi/category_theory_transformer_rs](https://github.com/hghalebi/category_theory_transformer_rs).
Use it for source files, runnable examples, issues, and contribution work.

## Help Improve This Book

If you want to help as a reader, use the public
[review path](https://github.com/hghalebi/category_theory_transformer_rs/blob/main/docs/review-path.md).
If you want the shareable public call for the five reviewer perspectives, use
[Reviewers Needed](https://github.com/hghalebi/category_theory_transformer_rs/blob/main/REVIEWERS.md).
If you are reviewing with a group, use the
[public review sprint](https://github.com/hghalebi/category_theory_transformer_rs/blob/main/docs/review-sprint.md)
to collect one report from each reader perspective.

The most useful report is small:

```text
Command or page tried:
Evidence signal:
Last clear idea:
First unclear sentence, output line, table row, code block, or exercise:
What would have helped:
```

If you are reading the online book and cannot clone the repository right now,
use the same shape. Put `public book path` in `Command or page tried`, name the
page you read, and quote or describe one visible evidence signal: a sentence,
heading, diagram, table row, code block, or exercise prompt.

Open the
[chapter clarity form](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=chapter-clarity.yml)
when the learning path breaks. One exact blocked step is more useful than a
broad review.

## What This Book Is About

Most machine-learning education starts with frameworks.

Frameworks are useful. They let us train real models quickly. But they also hide
the small structure underneath: the types of values moving through the system,
the transformations between those values, the loss that measures error, and the
update step that changes the model.

This book takes the opposite path.

It builds a tiny learning system slowly enough that every important shape can be
read in Rust:

```text
Text
  -> TokenSequence
  -> TrainingSet
  -> Prediction
  -> Loss
  -> Updated Parameters
```

The goal is to make the hidden structure easier to see.

> Executable structure, not AI magic.

## The Central Thesis

This book is built around one claim:

> A useful ML system is a chain of typed transformations.

Rust gives those transformations compile-checked boundaries. Category theory
gives names to recurring shapes such as objects, morphisms, products,
composition, endomorphisms, functors, and monoids. Tiny ML keeps the system
small enough to inspect completely.

The book uses all three, but in this order:

```text
intuition
  -> small Rust example
  -> ML meaning
  -> category-theory name
  -> runnable exercise
```

The category-theory words should arrive after the reader has seen the shape in
code.

## What You Already Know

If you have written a Rust function, you already know the first shape. A
function has an input type, an output type, and a body that explains how to move
from one to the other.

## Worked Example: Naming One Raw Value

Start with the deliberately unsafe version:

```rust
fn token_to_position(token_id: usize) -> usize {
    token_id + 100
}

assert_eq!(token_to_position(3), 103);
```

This is a transformation from one type to another:

```text
usize -> usize
```

The problem is that both sides are too vague. A raw `usize` might mean a token
index, a vocabulary size, a vector dimension, a training step, or a row number.
Those are different concepts, even if the machine representation is the same.

The book's first move is to give those concepts names.

```rust,ignore
pub struct TokenId(usize);
pub struct VocabSize(usize);
pub struct ModelDimension(usize);
```

Now the reader can ask better questions:

```text
Can this token be embedded?
Does this vector have the expected dimension?
Is this probability distribution valid?
Can this loss be accumulated?
Can this training update be repeated?
```

That is where the Rust type system starts to become part of the explanation.

## Self-check

Before continuing, explain what changed when `token_id: usize` became
`TokenId`. Did the machine representation change, or did the program gain a
clearer boundary?

## The Three Readings

Every important idea in the book is read three ways.

### Rust Reading

The Rust reading asks:

```text
What type is this?
What function or trait connects it to another type?
What invariant does the constructor protect?
What error can happen at the boundary?
```

For example:

```rust,ignore
pub struct TokenSequence(Vec<TokenId>);
```

This is not only "a struct containing a vector." In the real source, it is a
controlled domain object. Other code can use a `TokenSequence`, but it cannot
freely reach inside and mutate the raw representation.

### ML Reading

The ML reading asks:

```text
What stage of the learning pipeline is this?
Is it data, prediction, loss, or an update?
What would a larger framework usually hide here?
```

A token sequence is not the model yet. It is data after tokenization and before
training pairs. A distribution is not just a vector of floats. It is a vector of
non-negative probabilities that should sum to one.

### Category-Theory Reading

The category-theory reading asks:

```text
What object is this?
What morphism starts here or ends here?
Can two transformations compose?
Is this update an endomorphism?
Which law is the code trying to make visible?
```

The point is not to make the code sound more abstract. The point is to name the
same shape that the Rust and ML readings already revealed.

## The Main Picture

The tiny model is organized around this chain:

```text
TokenSequence -> TrainingSet
TokenId       -> Vector
Vector        -> Logits
Logits        -> Distribution
Distribution x TokenId -> Loss
Parameters    -> Parameters
```

Read it left to right.

The first line prepares examples.

The middle lines make a prediction and measure error.

The last line updates the model.

The Rust reading is:

```text
types + constructors + traits + errors + tests
```

The ML reading is:

```text
data + scores + probabilities + loss + training
```

The category-theory reading is:

```text
objects + morphisms + products + composition + laws
```

## Learning Contract

Use the same loop in every chapter. Start with the practical problem, read the
smallest example, and then inspect the relevant source snapshot. Translate the
Rust type or function into plain English before connecting it to the ML
pipeline. Only after the code is concrete should the chapter name the
category-theory shape. Then run the example and answer the retrieval questions
without looking back.

The chapters are deliberately repetitive in structure. That repetition is part
of the learning design. The pattern should become familiar:

```text
raw representation
  -> validated domain object
  -> typed transformation
  -> composed pipeline
  -> checked law
```

## What This Book Is Not

This is not a production ML framework.

This is not a performance-first Rust implementation.

This is not category theory as decoration.

This is not a promise that every advanced mathematical idea has been fully
formalized in the code.

The examples are intentionally small. They are designed to make structure
visible before speed, scale, or completeness enter the conversation.

## Reading Path

Read the chapters in order on the first pass.

The [Course Map](00-map.md) gives the whole pipeline shape.

[Domain Objects](01-domain-objects.md) names the typed nouns.

[Morphism and Composition](02-morphisms-composition.md) names the typed arrows
between them.

[The Tiny ML Pipeline](03-ml-pipeline.md) turns those arrows into prediction and
loss.

[Training as an Endomorphism](04-training-endomorphism.md) shows why one
optimizer step has the repeatable shape:

```text
Parameters -> Parameters
```

After the core pipeline, [Functors, Naturality, Monoids, and Chain
Rule](05-structure-and-calculus.md) introduces reusable structure, and
[Seven Sketches Through Rust](seven-sketches-rust.md) widens the method to
applied category theory.

Use the [Exercises](exercises.md) for practice, the [Glossary](glossary.md) for
terms, the [References](references.md) for chapter-specific sources, and the
[Transformer Roadmap](roadmap.md) for the path toward attention.

## Live Study

The first public workshop for the project is available through
[Luma registration](https://luma.com/event/evt-Pb1kYMQvzs8JrQq).

The workshop is a guided study path through the same tiny pipeline. It is useful
if you want to see the code, diagrams, and vocabulary connected live.

## What To Remember

The central discipline is:

> Do not let raw values travel farther than they should.

A raw `usize` becomes `TokenId`.

A raw `Vec<TokenId>` becomes `TokenSequence`.

A raw `Vec<f32>` becomes `Distribution` only after probability validation.

A raw optimizer update becomes `TrainStep`, a typed endomorphism:

```text
Parameters -> Parameters
```

The result is a small codebase where every concept has a name, every boundary
has a type, and every composition has to make sense before Rust lets it run.

## Where This Leaves Us

This welcome chapter sets the reading contract. You will see the same idea
through Rust syntax, tiny ML behavior, and category-theory shape. The next
chapter, [Course Map](00-map.md), gives the full map before the book starts
reading individual source files.

## Practice After This Chapter

Do one small check before moving on: run `cargo run --example 01_token_sequence`
and explain one output line using the three-lens shape from this chapter. If
you want a written prompt, use the first-output transfer checklist above and
Beginner Exercise 3 in [Exercises](exercises.md).

## Retrieval Practice

### Recall

What is the central pipeline shape this book keeps returning to?

### Explain

Why does the book connect every concept to Rust syntax, ML meaning, and
category-theory shape?

### Apply

Pick one raw value from the pipeline, such as a token index or probability
vector. Give it a domain-type name and explain what confusion the name prevents.
