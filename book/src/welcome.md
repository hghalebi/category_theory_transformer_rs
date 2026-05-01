# Category Theory for Tiny ML in Rust

This course teaches category-theory ideas through a tiny Rust language-model
pipeline.

The goal is not to memorize abstract vocabulary.

The goal is to connect each abstract word to:

- a concrete machine-learning operation
- a Rust type or trait
- an invariant the code protects
- a command you can run

The whole course follows one central idea:

> A useful ML system is a chain of typed transformations.

In this repository, that chain is small enough to read completely:

```text
raw text idea
  -> token ids
  -> token sequence
  -> next-token training pairs
  -> prediction distribution
  -> loss
  -> updated parameters
```

Rust gives those stages names.

Category theory gives those stages shapes.

Machine learning gives those stages a reason to exist.

## The Explanation Standard

Every major chapter now explains code at four levels.

First, it says what problem a block solves.

Second, it places the block in the ML pipeline.

Third, it reads the Rust syntax directly.

Fourth, it explains the category-theory shape behind the code.

For example, when you see:

```rust,ignore
pub struct TokenSequence(Vec<TokenId>);
```

do not read it as only:

```text
a struct containing a vector
```

Read it as:

```text
a validated, owned, non-empty list of token IDs
```

That one type carries several meanings at once:

- Rust meaning: private tuple struct wrapping `Vec<TokenId>`
- ML meaning: tokenized text before it becomes examples
- API meaning: callers cannot construct an empty sequence directly
- category-theory meaning: a non-empty list-like object

This is the level of reading used throughout the course.

## Learning Contract

Use this loop for every chapter:

1. Read the concrete problem first.
2. Study the code block or source snapshot.
3. Translate every type into plain English.
4. Translate every method into the pipeline stage it serves.
5. Run the command from the chapter.
6. Answer the checkpoint without looking back.

The chapters are deliberately repetitive in structure. That repetition is part
of the learning design. You should start to recognize the same pattern:

```text
raw representation
  -> validated domain object
  -> typed morphism
  -> composed pipeline
  -> tested law
```

## Fast Start

From the repository root:

```bash
cargo run --bin category_ml
```

That command runs the full guided walkthrough.

You should see:

- token IDs becoming training pairs
- a prediction path built from embedding, linear projection, and softmax
- cross entropy producing a loss
- repeated training lowering the loss
- functor, naturality, monoid, and chain-rule examples

## The Main Picture

The tiny model is a chain of typed arrows:

```text
TokenSequence -> TrainingSet
TokenId       -> Vector
Vector        -> Logits
Logits        -> Distribution
Distribution x TokenId -> Loss
Parameters    -> Parameters
```

The first line prepares examples.

The middle lines make predictions and measure error.

The last line updates the model.

The category-theory reading is:

```text
objects + morphisms + composition + laws
```

The Rust reading is:

```text
types + traits + smart constructors + tests
```

The ML reading is:

```text
data + model + probabilities + loss + training
```

## Reading Path

Read the chapters in order:

- [Course Map](00-map.md): the whole pipeline shape
- [Domain Objects](01-domain-objects.md): the typed nouns
- [Morphism and Composition](02-morphisms-composition.md): the typed arrows
- [The Tiny ML Pipeline](03-ml-pipeline.md): prediction and loss
- [Training as an Endomorphism](04-training-endomorphism.md): repeated updates
- [Functors, Naturality, Monoids, and Chain Rule](05-structure-and-calculus.md): reusable structure
- [Seven Sketches Through Rust](seven-sketches-rust.md): applied category theory as typed Rust models
- [Exercises](exercises.md): practice tasks
- [Glossary](glossary.md): definitions tied to this codebase
- [References](references.md): deeper external material
- [Transformer Roadmap](roadmap.md): how this foundation points toward attention

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
