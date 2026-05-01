# Course Map

## Goal

Learn one idea:

> Category theory is a language for typed transformations.

In this repo, the transformations are tiny ML operations:

- token to vector
- vector to logits
- logits to probabilities
- prediction plus target to loss
- parameters to better parameters

This is the whole course in one picture:

```text
TokenSequence -> TrainingSet
TokenId       -> Vector -> Logits -> Distribution
Distribution x TokenId -> Loss
Parameters   -> Parameters
```

The ML side says: make training pairs, predict probabilities, measure loss,
then update weights.

The category-theory side says: name each typed transformation, then compose the
legal transformations into a larger path.

## Code Map

Each concept has a small Rust file:

- [`src/domain.rs`](../../src/domain.rs): nouns, also called objects
- [`src/category.rs`](../../src/category.rs): arrows, identity, composition, endomorphisms
- [`src/ml.rs`](../../src/ml.rs): ML arrows
- [`src/training.rs`](../../src/training.rs): one training step
- [`src/structure.rs`](../../src/structure.rs): functor, natural transformation, monoid
- [`src/calculus.rs`](../../src/calculus.rs): local derivative example
- [`src/demo.rs`](../../src/demo.rs): one guided terminal walkthrough

The source snapshots keep each concept close to the code that implements it.

## Guided Walkthrough Snapshot

The terminal demo is the spine of the course. It touches every concept once.

<details>
<summary>Source snapshot: src/demo.rs</summary>

```rust,ignore
{{#include ../../src/demo.rs}}
```

</details>

The binary entrypoint is deliberately tiny:

<details>
<summary>Source snapshot: src/bin/category_ml.rs</summary>

```rust,ignore
{{#include ../../src/bin/category_ml.rs}}
```

</details>

## First Run

```bash
cargo run --bin category_ml
```

You should see a tiny language-model pipeline and the loss decreasing after
training.

## Checkpoint

Say this in your own words before moving on:

> A morphism is a typed function. Composition lets small typed functions become
> one larger typed pipeline.

## Further Reading

- [Glossary](glossary.md): object, morphism, composition, endomorphism
- [References](references.md): Rust modules and applied category theory
