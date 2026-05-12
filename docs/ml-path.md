# ML Path

Use this path if you can use AI frameworks but want to understand the small
pieces they hide.

The goal is not to replace framework practice. The goal is to make the hidden
objects and transformations easier to name:

```text
tokens -> training pairs -> prediction -> loss -> updated state
```

## First Pass

Run these commands before reading deeply:

```bash
cargo run --example 01_token_sequence
cargo run --bin category_ml
cargo run --example 03_training_endomorphism
```

Then read in this order:

1. [The Tiny ML Pipeline](../book/src/03-ml-pipeline.md)
2. [Training as an Endomorphism](../book/src/04-training-endomorphism.md)
3. [Exercises](../book/src/exercises.md#exercise-13-compute-cross-entropy-from-target-probability)
4. [Transformer Roadmap](../book/src/roadmap.md)

The first reading should answer one question:

```text
Which framework operation is being made explicit by this tiny Rust object?
```

## Core Questions

- What is the difference between tokens and training pairs?
- Why do logits become probabilities before loss?
- Why is training modeled as `Parameters -> Parameters`?
- Why is loss a measurement instead of the updated model state?
- Which objects would a framework usually keep inside tensors or module state?
- What does the tiny pipeline leave out compared with real frameworks?

## Framework Translation Table

Use this table when a tiny type feels too small compared with a real framework:

| Tiny object or arrow | ML meaning | Framework habit made explicit |
| --- | --- | --- |
| `TokenSequence -> TrainingSet` | build adjacent next-token examples | dataset/windowing logic |
| `TokenId -> Vector` | embedding lookup | row selection from an embedding table |
| `Vector -> Logits` | vocabulary scoring | linear readout |
| `Logits -> Distribution` | normalize scores | softmax over vocabulary |
| `Distribution x TokenId -> Loss` | score the target token | supervised cross-entropy |
| `Parameters -> Parameters` | update model state | one optimizer step |
| `TransformerTrainingState -> TransformerTrainingState` | update structured transformer state | parameter update plus training metadata |

## Active Practice

After reading the pipeline chapter, do Exercise 13:

```text
Distribution x TokenId -> Loss
```

A strong answer should explain why the target token is part of the input. Loss
does not come from a probability distribution alone; it comes from the
probability assigned to the correct target.

After reading the training chapter, run:

```bash
cargo run --example 07_transformer_training_state
```

Then explain why three different updates still expose the same public shape:

```text
TransformerTrainingState -> TransformerTrainingState
```

## Common ML Misreadings

| Misreading | What to say instead |
| --- | --- |
| logits are probabilities | logits are unnormalized scores; softmax produces a distribution |
| loss updates the model | loss measures the current model; a training step returns updated state |
| a target token is only metadata | the target is part of the supervised loss input |
| more steps are always better | more steps repeat the update shape; behavior depends on data, rate, and model |
| a tiny example is a framework replacement | the tiny example isolates structure that frameworks automate |

## Done Criteria

You can explain the path:

```text
TokenSequence -> TrainingSet -> Prediction -> Loss -> Updated Parameters
```

You can also point to:

- one command that prints the pipeline,
- one source file that computes loss,
- one exercise that checks target probability,
- one training example where the update returns the same state type.

## Feedback

Open the [chapter clarity feedback form](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=chapter-clarity.yml)
when the ML path stops matching your framework intuition.

Use this shape:

```text
Perspective: ML engineer
Command or page tried:
Framework concept I expected:
Tiny Rust object or arrow that became unclear:
What would have helped:
```
