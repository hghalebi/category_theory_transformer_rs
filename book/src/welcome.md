# Category Theory for Tiny ML in Rust

This course teaches category-theory ideas through a tiny Rust language-model
pipeline.

The goal is not to memorize abstract vocabulary.

The goal is to connect each abstract word to a concrete machine-learning
operation, a Rust type or trait, an invariant the code protects, and a command
you can run.

The whole course follows one central idea:

> A useful ML system is a chain of typed transformations.

## What You Already Know

If you have written a Rust function, you already know the informal shape behind
much of this book. A function receives a value of one type and returns a value
of another type. If you have seen an ML pipeline, you already know that data
moves through staged transformations. Category theory asks us to look at that
movement structurally.

## Worked Example: One Typed Transformation

Start with the smallest version:

```rust
fn token_to_position(token_id: usize) -> usize {
    token_id + 100
}

assert_eq!(token_to_position(3), 103);
```

Rust reads this as a function from `usize` to `usize`. The book's real examples
make the same movement safer by replacing raw `usize` values with named domain
types such as `TokenId`, `VocabSize`, and `ModelDimension`.

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

That one type carries several meanings at once. In Rust, it is a private tuple
struct wrapping `Vec<TokenId>`. In the ML pipeline, it is tokenized text before
it becomes examples. At the API boundary, it prevents callers from constructing
an empty sequence directly. Categorically, it behaves like a non-empty list-like
object.

This is the level of reading used throughout the course.

## Self-Check

Before continuing, explain this in your own words: what changes when a raw
number becomes a named type such as `TokenId`?

## Learning Contract

Use the same loop for every chapter. Start with the concrete problem, study the
code block or source snapshot, translate each type into plain English, and then
translate each method into the pipeline stage it serves. After that, run the
chapter command and answer the checkpoint without looking back.

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

You should see token IDs becoming training pairs, a prediction path built from
embedding, linear projection, and softmax, cross entropy producing a loss,
repeated training lowering the loss, and small examples for functors,
naturality, monoids, and the chain rule.

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

Read the chapters in order. The [Course Map](00-map.md) gives the whole
pipeline shape. [Domain Objects](01-domain-objects.md) names the typed nouns,
and [Morphism and Composition](02-morphisms-composition.md) names the typed
arrows between them. [The Tiny ML Pipeline](03-ml-pipeline.md) turns those
arrows into prediction and loss, while
[Training as an Endomorphism](04-training-endomorphism.md) shows why repeated
updates have the shape `Parameters -> Parameters`.

After the core pipeline, [Functors, Naturality, Monoids, and Chain
Rule](05-structure-and-calculus.md) introduces reusable structure, and
[Seven Sketches Through Rust](seven-sketches-rust.md) widens the same style to
applied category theory. The [Exercises](exercises.md),
[Glossary](glossary.md), [References](references.md), and
[Transformer Roadmap](roadmap.md) are there for practice, review, deeper
reading, and the path toward attention.

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

The welcome page sets the reading contract. You will see the same idea through
three lenses: Rust syntax, tiny ML behavior, and category-theory shape. The next
chapter gives the full map before the book starts reading individual source
files.

## Retrieval Practice

### Recall

What is the central pipeline shape this book keeps returning to?

### Explain

Why does the book connect every concept to Rust syntax, ML meaning, and
category-theory shape?

### Apply

Pick one raw value from the pipeline, such as a token index or probability
vector. Give it a domain-type name and explain what confusion the name prevents.
