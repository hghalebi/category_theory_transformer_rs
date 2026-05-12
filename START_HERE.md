# Start Here

Category Theory for Tiny ML in Rust is a public book and Rust lab for engineers
who want to understand AI systems below the framework layer.

Frameworks made AI accessible.
Tiny typed systems can make parts of AI understandable.

## Who this is for

This project is for:

- ML engineers who can use frameworks but want to understand what they hide
- Rust engineers who want a serious path into AI
- category-theory-curious engineers who want executable examples
- serious learners tired of shallow tutorials

## What you will build

A tiny ML pipeline:

```text
Text
→ Tokens
→ Training Pairs
→ Model State
→ Prediction
→ Loss
→ Updated Model State
```

## What this project is not

This is not a PyTorch replacement.
This is not category theory as decoration.
This is not abstraction cosplay.

The goal is executable structure.

## Best reading path

If Rust, ML, or category theory is new for you, start with
[docs/beginner-path.md](docs/beginner-path.md). It gives a slower first-session
route with stop signs for the first confusing terms.

If you already know one side of the project, use a focused route:

- [docs/rust-path.md](docs/rust-path.md) for Rust engineers who want to see how
  types, traits, constructors, and tests carry ML meaning
- [docs/ml-path.md](docs/ml-path.md) for ML engineers who want to map framework
  habits to tiny explicit objects and transformations
- [docs/category-theory-path.md](docs/category-theory-path.md) for readers who
  want category-theory vocabulary anchored to runnable Rust boundaries
- [docs/review-path.md](docs/review-path.md) if you want to help by filing one
  precise clarity report

1. [Domain Objects](book/src/01-domain-objects.md)
2. [Morphism and Composition](book/src/02-morphisms-composition.md)
3. [The Tiny ML Pipeline](book/src/03-ml-pipeline.md)
4. [Training as an Endomorphism](book/src/04-training-endomorphism.md)
5. [Exercises](book/src/exercises.md)

## First command

Run:

```bash
cargo run --example 01_token_sequence
```

The output shows raw text becoming token ids and next-token training pairs:

```text
Text -> TokenSequence -> TrainingPairs
```

## First public workshop

The first public workshop for this project is open for registration:

[Register for the public workshop](https://luma.com/event/evt-Pb1kYMQvzs8JrQq)

## How to give feedback

Open the [chapter clarity feedback form](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=chapter-clarity.yml)
for:

- unclear explanations
- missing Rust examples
- missing diagrams
- overloaded math terms
- Rust idiom improvements
- ML intuition gaps

The most useful feedback names:

```text
Chapter or file:
Friction lens:
Command or page tried:
First unclear sentence, output line, or exercise prompt:
Last clear idea:
What you expected:
What happened instead:
What would have helped:
```
