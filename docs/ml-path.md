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
| `LayerNormalization : HiddenSequence -> HiddenSequence` | apply one fixed normalization layer | module forward call with current parameters |
| `PositionWiseFeedForward : HiddenSequence -> HiddenSequence` | apply one fixed feed-forward sublayer | module forward call with current weights and biases |

## Attention Shape Ledger

When you reach the Transformer roadmap, use framework shape notation as a
translation aid, not as a replacement for the Rust types.

Run:

```bash
cargo run --example 06_attention_scores
```

Then compare the output with the same source/target split used by
[PyTorch MultiheadAttention](https://docs.pytorch.org/docs/stable/generated/torch.nn.MultiheadAttention.html)
and
[TensorFlow/Keras MultiHeadAttention](https://www.tensorflow.org/api_docs/python/tf/keras/layers/MultiHeadAttention):

| Framework cue | Rust roadmap object | ML meaning |
| --- | --- | --- |
| target length `L` or `T` | `QuerySequence` rows | positions asking for information |
| source length `S` | `KeySequence` and `ValueSequence` rows | positions that can be compared and read |
| mask shape `L x S` or `(B, T, S)` | `AttentionMask` | which source positions each query may read |
| attention output rows | `AttentionOutput` | one mixed value row per query position |

The mask shape tells you which score cells are affected. The mask polarity
tells you whether `true` means "allowed" or "blocked." This project uses:

```text
true  -> allowed source position
false -> blocked source position
```

Some framework APIs use the opposite boolean convention for attention masks or
padding masks. Translate polarity before comparing a framework call with the
Rust roadmap.

The local sanity check is:

```text
scores: QuerySequence x KeySequence -> AttentionScores
mask:   AttentionScores x AttentionMask -> AttentionScores
mix:    AttentionWeights x ValueSequence -> AttentionOutput
```

The mask should be read as a permissions table, not as a shorter token list:

```text
mask cells select legal score cells
softmax turns remaining score rows into weights
weights read value rows
```

This is the ML reading of the category rule. Attention is not one vague
`HiddenSequence -> HiddenSequence` operation. It first separates the side that
asks from the side that can be read, then returns information to the query
side.

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

Then separate forward calls from learning:

```text
LayerNormalization : HiddenSequence -> HiddenSequence
```

means the current layer parameters are already fixed inside the module for this
forward pass. If those parameters are being changed, the ML story has moved
back to the training-state update. This is the tiny Rust version of a familiar
framework distinction: a module's forward pass uses current parameters;
optimizer or training code changes them.

## Loss vs Update Trace

When you run:

```bash
cargo run --example 03_training_endomorphism
```

read the output in two different roles:

| Output line | ML role | Rust shape |
| --- | --- | --- |
| `loss before: ...` | measure the current model | `Parameters x TrainingSet -> Loss` |
| `loss after: ...` | measure the updated model | `Parameters x TrainingSet -> Loss` |
| `TrainStep : Parameters -> Parameters` | change the model state | same input and output object |

The loss value is evidence about a model. It is not the model update itself.
The update is the transformation that takes one `Parameters` object and returns
another `Parameters` object.

Use the same distinction when reading the Transformer training-state example:

```text
readout update: step 0 -> 1, loss 0.499085 -> 0.456495
feed-forward update: step 1 -> 2, loss 0.250000 -> 0.160633
composed block update: step 2 -> 3, loss 0.456495 -> 0.409737
```

The step count marks state movement. The two loss numbers measure state before
and after that movement.

## Gradient Evidence

Finite-difference checks compare two views of one local question:

```text
small parameter change -> measured loss change
training update        -> inferred gradient
```

They do not prove every future optimizer or Transformer block. They support
one selected parameter path when the numerical slope and the inferred update
gradient agree within tolerance. If they disagree, inspect the sign, averaging
scale, dropped path, nonsmooth point, or tolerance before trusting the update.

## Common ML Misreadings

| Misreading | What to say instead |
| --- | --- |
| logits are probabilities | logits are unnormalized scores; softmax produces a distribution |
| loss updates the model | loss measures the current model; a training step returns updated state |
| a target token is only metadata | the target is part of the supervised loss input |
| one gradient check proves training | a finite-difference check is local evidence for one selected parameter path |
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

For this path, use
[Open ML engineer report](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=chapter-clarity.yml&title=%5Bgood+first+feedback%5D+ML+engineer+brief&location=docs%2Fml-path.md&command=cargo+run+--example+01_token_sequence%0Acargo+run+--bin+category_ml%0Acargo+run+--example+03_training_endomorphism%0Acargo+run+--example+07_transformer_training_state).
The link fills the route, not the evidence; the evidence signal should come
from what you personally read, ran, or attempted.

Use this shape:

```text
Perspective: ML engineer
Command or page tried:
Evidence signal:
Framework concept I expected:
Tiny Rust object or arrow that became unclear:
What would have helped:
```

Use the evidence signal for the trace line, loss value,
framework-translation row, diagram row, or state-update output where the ML
meaning stopped matching the tiny Rust path.
