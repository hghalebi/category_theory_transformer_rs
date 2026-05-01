# Category Theory for Tiny ML in Rust

This is a small, modular, compile-checked tutorial for learning category-theory
ideas through a tiny machine-learning pipeline in Rust.

The teaching style is short-loop and ADHD-friendly:

- one concept at a time
- one Rust file per concept cluster
- one runnable example per lesson group
- fast feedback from `cargo`
- no hidden pseudo-code examples

## Start Here

Run the guided demo:

```bash
cargo run --bin category_ml
```

Run the full validation gate:

```bash
bash scripts/check.sh
```

## What You Will Learn

By the end, you should be able to explain and run:

- Object: domain types such as `TokenId`, `Vector`, `Distribution`, `Loss`, and `Parameters`
- Morphism: a typed transformation using `Morphism<Input, Output>`
- Identity: `Identity<T>`
- Composition: `Compose<F, G, Middle>`
- Product object: `Product<A, B>` and `TrainingExample`
- Endomorphism: `TrainStep : Parameters -> Parameters`
- Functor: `VecFunctor` and `OptionFunctor`
- Natural transformation: `VecToFirstOption`
- Monoid: `PipelineTrace`
- Commutative diagram: composed prediction path equals direct prediction path
- Chain rule: `MulOp` forward and backward local derivatives

## Repository Map

The code is split into small parts:

- `src/domain.rs`: typed nouns used by the whole tutorial
- `src/category.rs`: morphisms, identity, composition, endomorphisms
- `src/ml.rs`: token windowing, embedding, linear projection, softmax, cross entropy
- `src/training.rs`: training as a repeated parameter endomorphism
- `src/structure.rs`: functors, natural transformations, and monoids
- `src/calculus.rs`: local derivative and chain-rule example
- `src/demo.rs`: the complete terminal walkthrough
- `examples/`: runnable lesson examples
- `lessons/`: learner-facing reading path

## Lesson Path

Use the lessons in order:

1. [Map of the Course](lessons/00-map.md)
2. [Domain Objects](lessons/01-domain-objects.md)
3. [Morphism and Composition](lessons/02-morphisms-composition.md)
4. [The Tiny ML Pipeline](lessons/03-ml-pipeline.md)
5. [Training as an Endomorphism](lessons/04-training-endomorphism.md)
6. [Functors, Naturality, Monoids, and Chain Rule](lessons/05-structure-and-calculus.md)

## Runnable Examples

Each example is a real Rust file:

```bash
cargo run --example 01_domain_objects
cargo run --example 02_morphism_composition
cargo run --example 03_training_endomorphism
cargo run --example 04_structure_and_calculus
```

`cargo test --all-targets` also compiles the examples.

## Fast Mental Model

Think of the tiny model as arrows:

```text
TokenId -> Vector -> Logits -> Distribution
Distribution x TokenId -> Loss
Parameters -> Parameters
```

The first line is prediction.

The second line is evaluation.

The third line is training.

Category theory gives names to those shapes. Rust checks that the shapes connect.

## Quality Gate

Before trusting changes, run:

```bash
bash scripts/check.sh
```

That checks formatting, clippy, unit tests, examples, and the full demo.

This is pedagogical code, not a production ML framework.
# category_theory_transformer_rs
