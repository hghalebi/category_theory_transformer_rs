# Category Theory for Tiny ML in Rust

Frameworks made AI accessible.
Tiny typed systems make parts of AI understandable.

This is a public book and Rust lab for rebuilding tiny ML systems from first
principles using:

- Rust types
- typed transformations
- composition
- training loops
- category theory as an engineering tool

Not abstraction cosplay.
Executable structure.

## Who This Is For

This project is for engineers who:

- can use AI frameworks but want to understand what they hide
- know Rust and want a serious path into AI
- are curious about category theory but want executable examples
- prefer small systems they can inspect completely

## What You Will Build

A tiny ML pipeline:

```text
Text
-> Tokens
-> Training Pairs
-> Model State
-> Prediction
-> Loss
-> Updated Model State
```

The implementation is intentionally small. The point is not to compete with AI
frameworks. The point is to make the structure that frameworks hide visible,
typed, and runnable.

## Start Here

1. Read [START_HERE.md](START_HERE.md).
2. Run the first Rust example.
3. Try one beginner exercise.
4. Open a specific feedback issue.

## Five-Minute Win

Run:

```bash
cargo run --example 01_token_sequence
```

Expected shape:

```text
Raw input:
"rust makes ai structure visible"

TokenSequence:
[TokenId(12), TokenId(44), TokenId(7), TokenId(19), TokenId(91)]

TrainingPairs:
(TokenId(12) -> TokenId(44))
(TokenId(44) -> TokenId(7))
(TokenId(7) -> TokenId(19))
(TokenId(19) -> TokenId(91))

Typed transformation:
Text -> TokenSequence -> TrainingPairs

No framework magic.
Just explicit structure.
```

That is the whole project promise in one run: raw text becomes typed structure
you can inspect.

## Quickstart

```bash
cargo run --example 01_token_sequence
cargo run --example 01_domain_objects
cargo run --example 02_morphism_composition
cargo run --example 03_training_endomorphism
cargo run --example 04_structure_and_calculus
cargo run --example 05_seven_sketches
```

Run the full validation gate:

```bash
bash scripts/check.sh
```

Build the public book locally:

```bash
bash scripts/build-mdbook.sh
```

The generated HTML is written to `book/html/`.

## Current status

This is a working draft.

Some chapters are stable.
Some chapters are skeletal.
Some sections are intentionally public before they are polished.

The goal is to build the clearest possible path from tiny ML concepts to typed
Rust implementations, with public feedback from readers.

If something feels too compressed, unclear, or too bullet-point-like, please
open an issue. That feedback is useful.

The GitHub Pages version is published at:

```text
https://hghalebi.github.io/category_theory_transformer_rs/
```

## Public Workshop

The first public workshop for this project is hosted through AI Reading Club.
It introduces the tiny ML pipeline as typed Rust structure and invites reader
feedback while the public draft is still evolving.

[Register for the public workshop](https://luma.com/event/evt-Pb1kYMQvzs8JrQq)

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

The repository is organized as a learning product:

- `START_HERE.md`: first-session path
- `ROADMAP.md`: project milestones and sponsor-worthy work
- `CONTRIBUTING.md`: specific ways to help
- `SPONSORS.md`: why support matters
- `GOVERNANCE.md`: project decision rules
- `CHANGELOG.md`: visible momentum
- `src/`: compile-checked teaching modules
- `examples/`: runnable lesson examples
- `book/src/`: book chapters
- `docs/`: learning paths and project FAQ
- `exercises/`: beginner, intermediate, and advanced practice tracks
- `community/`: workshops, reading-club, and contributor-growth materials
- `.github/`: issue templates and workflow automation

The current Rust modules are:

- `src/domain.rs`: typed nouns used by the whole tutorial
- `src/category.rs`: morphisms, identity, composition, endomorphisms
- `src/ml.rs`: token windowing, embedding, linear projection, softmax, cross entropy
- `src/training.rs`: training as a repeated parameter endomorphism
- `src/structure.rs`: functors, natural transformations, and monoids
- `src/calculus.rs`: local derivative and chain-rule example
- `src/sketches.rs`: Rust companion models for the seven applied-category-theory sketches
- `src/demo.rs`: the complete terminal walkthrough

## Book Path

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

## How To Help

Do not send vague feedback if you can send specific feedback.

Useful first contributions:

- tell us where a chapter becomes unclear
- add or improve a Rust example
- propose a diagram for one pipeline step
- review terminology for category-theory precision
- review examples for Rust idiom
- turn one outline into a fuller explanation
- add an exercise idea with an expected answer

Start with [CONTRIBUTING.md](CONTRIBUTING.md), then choose an issue template
that matches the help you want to give.

## Quality Gate

Before trusting changes, run:

```bash
bash scripts/check.sh
```

That checks formatting, clippy, unit tests, examples, the full demo,
prose-style rules, course source snapshot coverage, the generated-book build,
and chapter tests.

## License

The repository does not declare a final license yet. Choosing the code and book
licenses is a project-owner decision on the roadmap before broader community
promotion.

Until a license is added, do not assume reuse rights beyond the explicit
reference guidance inside the book draft.
