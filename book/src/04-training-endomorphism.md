# Training as an Endomorphism

## Mental Model

An endomorphism maps a thing back to the same kind of thing:

```text
A -> A
```

Training does exactly that:

```text
Parameters -> Parameters
```

The model changes, but it is still a model.

The ML intuition is a training step:

1. Run the current parameters on the examples.
2. Measure the error.
3. Compute simple gradients.
4. Move the parameters a small amount.

The category-theory intuition is that the whole training step has the shape
`Parameters -> Parameters`. Because the output has the same type as the input,
the step can be repeated.

## Source Snapshot

This file implements one full-batch optimizer update.

Read in this order:

1. `TrainStep`
2. `impl Morphism<Parameters, Parameters> for TrainStep`
3. The unit test at the bottom

<details>
<summary>Source snapshot: src/training.rs</summary>

```rust,ignore
{{#include ../../src/training.rs}}
```

</details>

## Code Walkthrough

`TrainStep::new` stores the dataset and learning rate. Both are validated
domain values before the update runs.

The `Morphism<Parameters, Parameters>` implementation makes the training step a
real endomorphism in this crate.

Inside `apply`, gradients are accumulated over the training set, averaged by
batch size, and then applied to a cloned parameter object. That keeps the
operation deterministic and makes the returned value the new model state.

The regression test checks the learner-visible promise: applying the training
endomorphism repeatedly reduces loss on the tiny dataset.

## Run the Example

```bash
cargo run --example 03_training_endomorphism
```

Expected pattern:

```text
loss before: ...
loss after:  ...
```

The second number should be smaller.

## Why This Is Category-Theoretic

`TrainStep` can be repeated because its input type and output type are both
`Parameters`.

That makes this loop type-correct:

```text
Parameters0 -> Parameters1 -> Parameters2 -> ... -> ParametersN
```

## Runnable Example Snapshot

<details>
<summary>Source snapshot: examples/03_training_endomorphism.rs</summary>

```rust,ignore
{{#include ../../examples/03_training_endomorphism.rs}}
```

</details>

## Checkpoint

Why would training be harder to compose if it returned raw vectors instead of
`Parameters`?

## Further Reading

- [Glossary](glossary.md): endomorphism, parameters, learning rate, gradient
- [References](references.md): gradient descent, softmax regression, and Rust error handling
