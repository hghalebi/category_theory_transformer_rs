# 00 - Map of the Course

## Goal

Learn one idea:

> Category theory is a language for typed transformations.

In this repo, the transformations are tiny ML operations:

- token to vector
- vector to logits
- logits to probabilities
- prediction plus target to loss
- parameters to better parameters

## The Code Map

- `src/domain.rs`: nouns, also called objects
- `src/category.rs`: arrows, identity, composition, endomorphisms
- `src/ml.rs`: ML arrows
- `src/training.rs`: one training step
- `src/structure.rs`: functor, natural transformation, monoid
- `src/calculus.rs`: local derivative example
- `src/demo.rs`: one guided terminal walkthrough

## First Run

```bash
cargo run --bin category_ml
```

You should see a tiny language-model pipeline and the loss decreasing after
training.

## Checkpoint

Before moving on, say this in your own words:

> A morphism is a typed function. Composition lets small typed functions become
> one larger typed pipeline.
