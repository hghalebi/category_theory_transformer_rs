# ML Path

Use this path if you can use AI frameworks but want to understand the small
pieces they hide.

## Order

1. Run `cargo run --example 01_token_sequence`.
2. Read [book/src/03-ml-pipeline.md](../book/src/03-ml-pipeline.md).
3. Run `cargo run --example 03_training_endomorphism`.
4. Read [book/src/04-training-endomorphism.md](../book/src/04-training-endomorphism.md).
5. Try Exercise 3 in [book/src/exercises.md](../book/src/exercises.md).

## Core Questions

- What is the difference between tokens and training pairs?
- Why do logits become probabilities before loss?
- Why is training modeled as `Parameters -> Parameters`?
- What does the tiny pipeline leave out compared with real frameworks?

## Done Criteria

You can explain the path:

```text
TokenSequence -> TrainingSet -> Prediction -> Loss -> Updated Parameters
```
