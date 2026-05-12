# Chapter Contracts

Status: Active rewrite support

This file is an editorial manifest. It keeps each core chapter connected to its
public maturity status, reference-map question, source bucket, code evidence,
runnable evidence, and practice evidence.

When a chapter changes substantially, update this manifest in the same pass as
the chapter, reference map, practice map, maturity table, and rewrite log.

| Chapter | Maturity | Reference-map question | Source bucket | Code evidence | Run evidence | Practice evidence |
| --- | --- | --- | --- | --- | --- | --- |
| [Welcome](src/welcome.md) | Stable draft | What is the book's promise and reading contract? | Welcome And Course Map | `book/src/welcome.md`, `examples/01_token_sequence.rs`, `src/domain.rs` | `cargo run --example 01_token_sequence` | First-output transfer checklist and Beginner Exercise 3; [Exercises](src/exercises.md) |
| [Course Map](src/00-map.md) | Stable draft | How do the Rust files, ML pipeline, and category-theory vocabulary fit together? | Welcome And Course Map | `book/src/00-map.md`, `src/demo.rs`, `src/bin/category_ml.rs` | `cargo run --bin category_ml` | Demo-output wayfinding checklist, Exercise 2, and Exercise 8; [Exercises](src/exercises.md) |
| [Domain Objects](src/01-domain-objects.md) | Stable draft | Why should meaningful ML values become separate Rust types? | Domain Objects | `book/src/01-domain-objects.md`, `src/domain.rs`, `examples/01_domain_objects.rs` | `cargo run --example 01_domain_objects` | Example-output transfer checklist, Exercise 1, and Exercise 7; [Exercises](src/exercises.md) |
| [Morphism and Composition](src/02-morphisms-composition.md) | Draft | How do typed transformations compose safely? | Morphisms And Composition | `book/src/02-morphisms-composition.md`, `src/category.rs`, `examples/02_morphism_composition.rs` | `cargo run --example 02_morphism_composition` | Stage-output transfer checklist and Exercise 4; [Exercises](src/exercises.md) |
| [The Tiny ML Pipeline](src/03-ml-pipeline.md) | Draft | How do token pairs become prediction, probability, and loss? | Tiny ML Pipeline | `book/src/03-ml-pipeline.md`, `src/ml.rs`, `src/domain.rs`, `src/demo.rs` | `cargo test ml::tests --lib` | Demo-output transfer checklist, Exercise 3, Exercise 9, and Exercise 13; [Exercises](src/exercises.md) |
| [Training as an Endomorphism](src/04-training-endomorphism.md) | Draft | Why is one training step a repeatable `Parameters -> Parameters` update? | Training And Chain Rule | `book/src/04-training-endomorphism.md`, `src/training.rs`, `examples/03_training_endomorphism.rs` | `cargo run --example 03_training_endomorphism` | Example-output transfer checklist and Exercise 5; [Exercises](src/exercises.md) |
| [Functors, Naturality, Monoids, and Chain Rule](src/05-structure-and-calculus.md) | Draft | Which recurring structures appear after the first ML pipeline works? | Structure And Laws | `book/src/05-structure-and-calculus.md`, `src/structure.rs`, `src/calculus.rs` | `cargo run --example 04_structure_and_calculus` | Example-output transfer checklist, Exercise 6, and Exercise 14; [Exercises](src/exercises.md) |
| [Seven Sketches Through Rust](src/seven-sketches-rust.md) | Draft | How can applied category theory become concrete enough to inspect in Rust? | Seven Sketches Through Rust | `book/src/seven-sketches-rust.md`, `src/sketches.rs`, `examples/05_seven_sketches.rs` | `cargo run --example 05_seven_sketches` | Example-output transfer checklist and Exercise 10; [Exercises](src/exercises.md) |
| [Exercises](src/exercises.md) | Draft | How does the reader prove they can transfer the method? | Exercises And Transfer | `book/src/exercises.md`, `exercises/ANSWER_KEY.md`, `exercises/beginner/README.md` | `python3 scripts/check-exercise-alignment.py` | Exercise ladder, evidence map, worked mixed boundary diagnosis, and answer key |
| [Transformer Roadmap](src/roadmap.md) | Sketch | How does the tiny system grow toward attention and Transformer blocks? | Transformer Roadmap | `book/src/roadmap.md`, `src/attention.rs`, `examples/06_attention_scores.rs` | `cargo run --example 06_attention_scores` | Terminal-output checkpoint map, category-shape diagnostic, output-to-boundary transfer checklist, Exercise 12, Exercise 15, and Advanced Exercise 5; [Exercises](src/exercises.md) |

## Contract Rule

Each row must answer six editorial questions:

1. What chapter file owns the teaching path?
2. What public maturity status should readers see?
3. Which central question from the reference map should constrain rewrites?
4. Which editorial source bucket should be consulted before rewriting?
5. Which repository files are the code evidence?
6. Which runnable command or checker proves the chapter is still executable?
7. Which exercise or practice surface proves transfer?
