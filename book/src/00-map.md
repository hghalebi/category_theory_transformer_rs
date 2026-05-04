# Course Map

The problem this chapter solves is:

> Before reading individual Rust files, you need one map of how the whole
> machine-learning pipeline, Rust type system, and category-theory vocabulary
> fit together.

The repository is small, but it contains several layers:

```text
domain objects
  -> typed morphisms
  -> concrete ML morphisms
  -> training endomorphism
  -> reusable structure patterns
  -> applied category-theory sketches
```

This chapter is the index of those layers.

> Reader orientation:
> The map is not a list of things to memorize. It is a promise about how the
> book will move: first name the values, then name the arrows, then compose the
> arrows into a tiny learning system.

## What You Already Know

If you already read programs from top to bottom, you know how to follow a flow.
If you know Rust function signatures, you know that each step has an input type
and an output type. If you know ML pipelines, you know that raw data becomes
features, predictions, loss, and updates. This chapter puts those familiar
habits on one map.

## The Whole Pipeline

The central pipeline is:

```text
TokenSequence -> TrainingSet
TokenId       -> Vector
Vector        -> Logits
Logits        -> Distribution
Distribution x TokenId -> Loss
Parameters    -> Parameters
```

Read this as three stories at once.

In ML terms:

```text
tokenized text
  -> prediction examples
  -> embeddings
  -> vocabulary scores
  -> probabilities
  -> error measurement
  -> updated weights
```

In Rust terms:

```text
validated input types
  -> trait implementations
  -> explicit error handling
  -> private fields
  -> read-only accessors
  -> tests
```

In category-theory terms:

```text
objects
  -> morphisms
  -> products
  -> composition
  -> endomorphisms
  -> laws
```

The course is about learning to see the same pipeline through all three views.

## Worked Example: A Tiny Typed Movement

Here is the smallest Rust idea behind that map. A function has an input type and
an output type:

```rust
fn token_to_vector_id(token_id: usize) -> usize {
    token_id + 100
}

assert_eq!(token_to_vector_id(7), 107);
```

The real code does not leave those values as raw `usize` forever. It gives each
pipeline stage a domain type, then uses morphisms to make the connections
explicit.

## Self-Check

Before moving into the file map, explain why `TokenId -> Vector` is easier to
reason about than `usize -> Vec<f32>`.

## Code Map

Each Rust file owns one part of the idea.

### `src/domain.rs`

This file defines the nouns.

The main examples are `TokenId`, `TokenSequence`, `Vector`, `Logits`,
`Distribution`, `Loss`, `TrainingSet`, and `Parameters`.

The problem this file solves is:

> Raw numbers are too ambiguous for a training pipeline.

For example, these are all machine numbers:

```text
token index
vocabulary size
model dimension
loss value
learning rate
```

But they are not the same concept.

`src/domain.rs` gives each concept a separate type.

### `src/category.rs`

This file defines the arrows.

The central trait is:

```rust,ignore
pub trait Morphism<Input, Output> {
    fn name(&self) -> &'static str;
    fn apply(&self, input: Input) -> CtResult<Output>;
}
```

This says:

> A morphism is something that knows how to transform an `Input` into an
> `Output`, possibly failing with `CtError`.

The rest of the file defines identity, composition, endomorphism, and repeated
application.

### `src/ml.rs`

This file defines concrete ML arrows.

The main transformations are:

```text
DatasetWindowing : TokenSequence -> TrainingSet
Embedding        : TokenId -> Vector
LinearToLogits   : Vector -> Logits
Softmax          : Logits -> Distribution
CrossEntropy     : Distribution x TokenId -> Loss
```

This file is where the abstract `Morphism` trait becomes a tiny learning
system.

### `src/training.rs`

This file defines:

```text
TrainStep : Parameters -> Parameters
```

That shape is important.

Because the output type is the same as the input type, training can be repeated:

```text
Parameters0 -> Parameters1 -> Parameters2 -> ... -> ParametersN
```

That is why training is taught as an endomorphism.

### `src/structure.rs`

This file teaches reusable structure:

- functor: map inside a wrapper
- natural transformation: convert wrapper shape consistently
- monoid: combine values with an empty value

These are not extra theory for decoration. They name patterns that appear in
ordinary ML systems: batches, optional values, traces, logs, and composed
workflows.

### `src/calculus.rs`

This file shows the smallest useful backpropagation idea:

```text
z = x * y
dL/dx = dL/dz * y
dL/dy = dL/dz * x
```

The code does not implement a full automatic differentiation engine. It gives
you the local rule that larger systems compose.

### `src/sketches.rs`

This file connects the course to seven applied category-theory themes: orders,
resources, databases, co-design, signal flow, circuits, and behavior logic.

Each theme is represented as typed Rust values plus law-checking tests.

## Guided Walkthrough Snapshot

The terminal demo is the spine of the course.

The problem this block solves is:

> A learner should be able to run one command and see every major concept used
> once in a concrete order.

<details>
<summary>Source snapshot: src/demo.rs</summary>

```rust,ignore
{{#include ../../src/demo.rs}}
```

</details>

## How To Read The Demo

The demo is not random output. It is a staged proof that the pieces connect.

Section 1 introduces an object:

```text
TokenId(1)
```

Section 2 applies a data-preparation morphism:

```text
TokenSequence -> TrainingSet
```

Section 3 applies identity:

```text
Vector -> Vector
```

Section 4 composes prediction:

```text
TokenId -> Vector -> Logits -> Distribution
```

Section 5 uses a product object:

```text
Distribution x TokenId -> Loss
```

Section 6 repeats an endomorphism:

```text
Parameters -> Parameters
```

Sections 7 through 11 add the structural patterns:

```text
Functor
NaturalTransformation
Monoid
Commutative diagram check
Chain rule
```

So the demo is a miniature course outline in executable form.

## Binary Entrypoint

The binary entrypoint is deliberately tiny:

<details>
<summary>Source snapshot: src/bin/category_ml.rs</summary>

```rust,ignore
{{#include ../../src/bin/category_ml.rs}}
```

</details>

The whole file is:

```rust,ignore
use category_theory_transformer_rs::run_demo;

fn main() {
    run_demo().unwrap();
}
```

Line by line:

`use category_theory_transformer_rs::run_demo;`

This imports the library function that owns the walkthrough.

`fn main()`

This is the process entrypoint. When you run the binary, Rust starts here.

`run_demo().unwrap();`

This runs the walkthrough and panics if it fails. In the library code, fallible
work uses `CtResult`. The binary keeps the entrypoint short because the course
focus is the library, not command-line error reporting.

## First Run

Run:

```bash
cargo run --bin category_ml
```

You should see a tiny language-model pipeline and the loss decreasing after
training.

The important part is not the exact floating-point numbers.

The important part is the shape:

```text
before training: higher loss
after training:  lower loss
```

That means repeated `TrainStep` applications moved the parameters in a useful
direction on the tiny dataset.

## Core Mental Model

Every chapter after this one zooms into one row of the map.

Remember:

```text
object = typed thing
morphism = typed transformation
composition = legal connection of transformations
endomorphism = transformation from a type back to itself
law = property the code checks so composition remains trustworthy
```

## Checkpoint

Explain this line in your own words:

```text
TokenId -> Vector -> Logits -> Distribution
```

A strong answer should mention token lookup, the embedding vector, vocabulary
scores, the probability distribution, and the fact that the whole path is a
composition of typed morphisms.

## Where This Leaves Us

This chapter gave the whole shape before the details. You now know the names of
the source files, the major pipeline objects, and the difference between
objects, morphisms, composition, endomorphisms, and laws.

The next chapter slows down and studies the objects themselves. Before a
pipeline can compose arrows safely, it needs values whose meanings are clear
enough for arrows to start and end at them.

## Further Reading

These pages are the best next stops after the map:

- [Glossary](glossary.md): object, morphism, composition, endomorphism
- [References](references.md): Rust modules and applied category theory
- [Seven Sketches Through Rust](seven-sketches-rust.md): a paper-length concept map made executable

## Retrieval Practice

### Recall

Name the three readings used throughout the course.

### Explain

Why does the course start with a whole-pipeline map before reading individual
source files?

### Apply

Write a one-line diagram for a pipeline you already know, then label the input
object, arrow, and output object.
