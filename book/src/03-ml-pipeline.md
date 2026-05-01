# The Tiny ML Pipeline

## Mental Model

The ML pipeline is a sequence of typed transformations:

```text
TokenSequence -> TrainingSet
TokenId -> Vector
Vector -> Logits
Logits -> Distribution
Distribution x TokenId -> Loss
```

## Read This File

Open [`src/ml.rs`](../../src/ml.rs).

Read only these structs first:

- `DatasetWindowing`
- `Embedding`
- `LinearToLogits`
- `Softmax`
- `CrossEntropy`

## Run the Demo

```bash
cargo run --bin category_ml
```

Look at sections 2 through 5 in the output.

## The Key Detail

`Softmax` validates that its output is a real probability distribution.
`CrossEntropy` validates that the target token is inside the distribution.

Errors happen at the boundary where the bad data is first understood.

## Checkpoint

Where should an out-of-range target token be caught: inside `CrossEntropy`, or
later after loss calculation?
