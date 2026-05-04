# Category Theory for Tiny ML in Rust

This is a small, modular, compile-checked tutorial for learning category-theory
ideas through a tiny machine-learning pipeline in Rust.

The teaching style is short-loop, low-friction, and example-first:

- one concept at a time
- one Rust file per concept cluster
- one runnable example per lesson group
- exact learner source snapshots embedded in the course
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

Build the mdBook:

```bash
bash scripts/build-mdbook.sh
```

The generated HTML will be written to `book/html/`.

The generated course is the canonical tutorial surface. It embeds the Rust
modules, runnable examples, compact lesson notes, a glossary, curated external
references, and a Transformer roadmap. Build and publishing details stay in
repository docs and scripts rather than inside the learner course.

The GitHub Pages workflow publishes the book from `main` to:

```text
https://hghalebi.github.io/category_theory_transformer_rs/
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
- Seven applied sketches: orders, resources, databases, co-design, signal flow, circuits, and behavior logic through `src/sketches.rs`

## Repository Map

The code is split into small parts:

- `src/domain.rs`: typed nouns used by the whole tutorial
- `src/category.rs`: morphisms, identity, composition, endomorphisms
- `src/ml.rs`: token windowing, embedding, linear projection, softmax, cross entropy
- `src/training.rs`: training as a repeated parameter endomorphism
- `src/structure.rs`: functors, natural transformations, and monoids
- `src/calculus.rs`: local derivative and chain-rule example
- `src/sketches.rs`: Rust companion models for the seven applied-category-theory sketches
- `src/demo.rs`: the complete terminal walkthrough
- `examples/`: runnable lesson examples
- `lessons/`: learner-facing reading path
- `book/src/`: mdBook source chapters
- `book/html/`: generated mdBook output, ignored by git
- `scripts/check-prose-style.py`: checks prose density and learning-scaffold markers
- `scripts/check-mdbook-coverage.sh`: verifies course source snapshots and required references
- `.github/workflows/mdbook-pages.yml`: CI plus GitHub Pages deployment

## Lesson Path

Use the generated course lessons in order:

1. [Cover](book/src/cover.md)
2. [Welcome](book/src/welcome.md)
3. [Map of the Course](book/src/00-map.md)
4. [Domain Objects](book/src/01-domain-objects.md)
5. [Morphism and Composition](book/src/02-morphisms-composition.md)
6. [The Tiny ML Pipeline](book/src/03-ml-pipeline.md)
7. [Training as an Endomorphism](book/src/04-training-endomorphism.md)
8. [Functors, Naturality, Monoids, and Chain Rule](book/src/05-structure-and-calculus.md)
9. [Seven Sketches Through Rust](book/src/seven-sketches-rust.md)
10. [Exercises](book/src/exercises.md)
11. [Glossary](book/src/glossary.md)
12. [References](book/src/references.md)
13. [Transformer Roadmap](book/src/roadmap.md)
14. [Repository Source Snapshots](book/src/source-snapshots.md)

The `lessons/` folder is kept as a compact reading path, while `book/src/`
contains the complete self-contained course.

## Runnable Examples

Each example is a real Rust file:

```bash
cargo run --example 01_domain_objects
cargo run --example 02_morphism_composition
cargo run --example 03_training_endomorphism
cargo run --example 04_structure_and_calculus
cargo run --example 05_seven_sketches
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

That checks formatting, clippy, unit tests, examples, the full demo, the
prose-style and learning-scaffold rules, the course source snapshot coverage,
the generated-book build, and chapter tests.

The source coverage script also checks that the learner course does not discuss
the documentation-generation tool and does not explicitly name the previously
disallowed instructor reference.

## Serve the Book

```bash
mdbook serve --open
```

If the browser does not open automatically, use the local URL printed by
`mdbook`.

## GitHub Pages Deployment

The workflow in `.github/workflows/mdbook-pages.yml` runs on pull requests,
pushes to `main`, and manual dispatches.

Pull requests run validation only.

Pushes to `main` run validation, upload `book/html`, and deploy through GitHub
Pages. In the GitHub repository settings, set:

```text
Settings -> Pages -> Build and deployment -> Source -> GitHub Actions
```

This is pedagogical code, not a production ML framework.
