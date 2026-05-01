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

## Code Map

Each concept has a small Rust file:

- [`src/domain.rs`](../../src/domain.rs): nouns, also called objects
- [`src/category.rs`](../../src/category.rs): arrows, identity, composition, endomorphisms
- [`src/ml.rs`](../../src/ml.rs): ML arrows
- [`src/training.rs`](../../src/training.rs): one training step
- [`src/structure.rs`](../../src/structure.rs): functor, natural transformation, monoid
- [`src/calculus.rs`](../../src/calculus.rs): local derivative example
- [`src/demo.rs`](../../src/demo.rs): one guided terminal walkthrough

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
