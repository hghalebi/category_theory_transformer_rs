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

The plain ML story is:

1. Turn a token stream into adjacent training pairs.
2. Look up an embedding vector for the current token.
3. Use a linear layer to score each possible next token.
4. Use softmax to convert scores into probabilities.
5. Use cross entropy to measure how surprised the model was by the target.

The category-theory story is the same story with stricter names: each step is a
morphism, and the pipeline is a composition of morphisms.

## Source Snapshot

This file owns the concrete ML arrows.

Read only these structs first:

- `DatasetWindowing`
- `Embedding`
- `LinearToLogits`
- `Softmax`
- `CrossEntropy`

<details>
<summary>Source snapshot: src/ml.rs</summary>

```rust,ignore
{{#include ../../src/ml.rs}}
```

</details>

## Code Walkthrough

`DatasetWindowing` maps `TokenSequence -> TrainingSet`. It is the data-prep
arrow.

`Embedding` maps `TokenId -> Vector`. In neural-network language, it turns a
discrete symbol into learned continuous features.

`LinearToLogits` maps `Vector -> Logits`. It produces one unnormalized score per
vocabulary item.

`Softmax` maps `Logits -> Distribution`. It is the boundary where raw scores
become probabilities.

`CrossEntropy` maps `Distribution x TokenId -> Loss`. It compares the predicted
distribution to the actual target token.

`composed_prediction_matches_direct_prediction` is a small commutative-diagram
check: the explicit composition and the direct implementation produce the same
probabilities.

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

## Further Reading

- [Glossary](glossary.md): logits, softmax, probability distribution, cross entropy
- [References](references.md): softmax regression and linear classifiers
