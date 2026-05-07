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

Open an issue for:

- unclear explanations
- missing Rust examples
- missing diagrams
- overloaded math terms
- Rust idiom improvements
- ML intuition gaps
