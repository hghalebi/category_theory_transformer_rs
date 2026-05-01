# Training as an Endomorphism

The problem this chapter solves is:

> A model is not only used for prediction. It must also be updated by training,
> and one update should produce the same kind of object it consumed.

The key shape is:

```text
Parameters -> Parameters
```

This is an endomorphism.

In ordinary ML terms:

```text
old parameters
  -> compute predictions
  -> compute loss gradients
  -> subtract learning-rate-scaled gradients
  -> new parameters
```

In category-theory terms:

```text
A -> A
```

Because the input and output type are the same, the step can be repeated.

## Source Snapshot

This file implements one full-batch optimizer update.

<details>
<summary>Source snapshot: src/training.rs</summary>

```rust,ignore
{{#include ../../src/training.rs}}
```

</details>

## The Whole File

`src/training.rs` defines:

```text
TrainStep
TrainStep::new
impl Morphism<Parameters, Parameters> for TrainStep
unit test proving repeated training reduces loss
```

The whole file is about one idea:

```text
training is a repeatable typed transformation of model state
```

## `TrainStep`

The problem this block solves is:

> A training update needs a dataset and a learning rate, and those values should
> travel together as one configured operation.

The block:

```rust,ignore
/// One full-batch optimizer update.
///
/// Categorically, this is an endomorphism:
///
/// `Parameters -> Parameters`
#[derive(Debug, Clone)]
pub struct TrainStep {
    dataset: TrainingSet,
    learning_rate: LearningRate,
}
```

## Rust Syntax

This is a named-field struct.

It stores:

```text
dataset: TrainingSet
learning_rate: LearningRate
```

Both fields are private.

That means callers cannot directly replace the dataset or learning rate after
construction.

The derived traits mean:

```text
Debug -> can be printed for debugging
Clone -> can be explicitly duplicated
```

`TrainingSet` is already non-empty.

`LearningRate` is already finite and positive.

So `TrainStep` stores validated inputs.

## ML Concept

A training step needs:

- examples to learn from
- a step size for parameter updates

The dataset gives the input-target pairs.

The learning rate controls how far the update moves.

## Category-Theory Concept

`TrainStep` is the value that will implement:

```text
Parameters -> Parameters
```

That makes it an endomorphism on the object `Parameters`.

## `TrainStep::new`

The problem this block solves is:

> Construct a configured training step from already validated pieces.

The block:

```rust,ignore
impl TrainStep {
    pub fn new(dataset: TrainingSet, learning_rate: LearningRate) -> Self {
        Self {
            dataset,
            learning_rate,
        }
    }
}
```

## Rust Syntax

`impl TrainStep` defines methods for `TrainStep`.

The constructor takes ownership of:

```text
dataset
learning_rate
```

and stores them.

It returns `Self`, not `CtResult<Self>`, because the inputs are already
validated domain objects.

No extra validation is needed here.

## ML Concept

This is like configuring an optimizer step:

```text
use this dataset
use this learning rate
```

The actual update happens later in `apply`.

## Category-Theory Concept

The constructor chooses one specific endomorphism from a family:

```text
TrainStep(dataset, learning_rate) : Parameters -> Parameters
```

Different datasets or learning rates create different update morphisms.

## Morphism Implementation

The problem this block solves is:

> Make `TrainStep` a real typed arrow from model parameters back to model
> parameters.

The header:

```rust,ignore
impl Morphism<Parameters, Parameters> for TrainStep {
```

## Rust Syntax

This says:

```text
TrainStep implements Morphism<Input = Parameters, Output = Parameters>
```

So the `apply` method must have this effective shape:

```text
Parameters -> CtResult<Parameters>
```

The name method:

```rust,ignore
fn name(&self) -> &'static str {
    "train_step_endomorphism"
}
```

returns a static label for the transformation.

## ML Concept

The input `Parameters` are the current model weights.

The output `Parameters` are the updated weights after one full-batch step.

## Category-Theory Concept

Because the input and output object are the same, `TrainStep` is an
endomorphism.

That is what lets this work:

```text
Parameters0 -> Parameters1 -> Parameters2 -> ... -> ParametersN
```

## `apply`: Shape Checks

The problem this block solves is:

> Before computing gradients, verify that the parameter object has usable
> dimensions.

The block:

```rust,ignore
let vocab_size = params.vocab_size();
let d_model = params.d_model();

if vocab_size == 0 || d_model == 0 {
    return Err(CtError::EmptyInput("parameters"));
}
```

## Rust Syntax

The code asks the parameter object for two dimensions.

Then it rejects zero-sized parameters.

This uses an explicit error instead of panicking.

## ML Concept

Training cannot run if:

- there are zero possible vocabulary outputs
- hidden vectors have zero width

Those shapes would make the gradient arrays meaningless.

## Category-Theory Concept

The endomorphism is only defined on valid `Parameters`.

Invalid parameter state is rejected before the morphism performs the update.

## Gradient Buffers

The problem this block solves is:

> Accumulate gradients for every trainable parameter before applying the update.

The block:

```rust,ignore
let mut grad_embedding = vec![vec![0.0; d_model]; params.embedding.len()];
let mut grad_lm_head = vec![vec![0.0; vocab_size]; d_model];
let mut grad_bias = vec![0.0; vocab_size];
```

## Rust Syntax

These are mutable matrices and vectors initialized to zero.

Their shapes mirror the trainable parameters:

```text
grad_embedding: same row count as embedding, d_model columns
grad_lm_head:   d_model x vocab_size
grad_bias:      vocab_size
```

## ML Concept

Gradients accumulate how each parameter should change to reduce loss.

The code uses full-batch training: it processes every example, accumulates all
gradients, averages them, then updates once.

## Category-Theory Concept

The gradient buffers are not the endomorphism itself.

They are internal machinery used to construct the output object in:

```text
Parameters -> Parameters
```

## Example Loop

The problem this block solves is:

> For each training example, compute the local contribution to the parameter
> gradients.

The loop begins:

```rust,ignore
for example in self.dataset.examples() {
    let input_id = example.first().index();
    let target_id = example.second().index();
    ...
}
```

## Rust Syntax

`self.dataset.examples()` returns a slice of `TrainingExample`.

Each example is a `Product<TokenId, TokenId>`.

So:

```rust,ignore
example.first()
```

is the input token.

```rust,ignore
example.second()
```

is the target token.

The code extracts raw indices because matrix indexing needs `usize`.

## ML Concept

Each example says:

```text
given input token, predict target token
```

The training loop calculates how wrong the current model is for that example.

## Category-Theory Concept

The example is an element of:

```text
TokenId x TokenId
```

The training morphism consumes many such product values while building the
parameter update.

## Token Bounds Checks

The problem this block solves is:

> Training examples must refer to tokens that exist in the current parameter
> shapes.

The checks:

```rust,ignore
if input_id >= params.embedding.len() {
    return Err(CtError::OutOfRange { ... });
}

if target_id >= vocab_size {
    return Err(CtError::OutOfRange { ... });
}
```

## Rust Syntax

These are ordinary bounds checks with typed errors.

They prevent invalid indexing into:

- the embedding table
- the vocabulary-sized output vector

## ML Concept

An input token must have an embedding row.

A target token must be one of the possible prediction classes.

If either token is outside the model vocabulary, training cannot continue.

## Category-Theory Concept

The example must belong to the finite token object that the parameters are
currently modeling.

This check keeps the training morphism inside the intended domain.

## Forward Pass Inside Training

The problem this block solves is:

> To compute gradients, the training step first needs the current prediction.

The block:

```rust,ignore
let x = &params.embedding[input_id];
let logits = LinearToLogits::from_parts(params.lm_head.clone(), params.bias.clone())
    .apply(Vector::new(x.clone()))?;
let probs = Softmax.apply(logits)?;
```

## Rust Syntax

`x` borrows the embedding row for the input token.

`LinearToLogits::from_parts(...)` builds a linear projection from the current
weights.

`Vector::new(x.clone())` wraps the embedding row as a `Vector`.

Then:

```text
Vector -> Logits -> Distribution
```

runs through the same morphism interface as prediction.

## ML Concept

This computes the model's current predicted distribution for one input token.

The gradient depends on the difference between that distribution and the true
target.

## Category-Theory Concept

Even inside training, prediction is still a composed path:

```text
TokenId -> Vector -> Logits -> Distribution
```

Training uses that path as part of a larger endomorphism:

```text
Parameters -> Parameters
```

## Logit Gradient

The problem this block solves is:

> For softmax plus cross entropy, the gradient with respect to logits is
> predicted probability minus one-hot target.

The block:

```rust,ignore
let mut dlogits = probs.as_slice().to_vec();
dlogits[target_id] -= 1.0;
```

## Rust Syntax

The probabilities are copied into a mutable vector.

Then the target class is adjusted by subtracting `1.0`.

If:

```text
probs = [0.70, 0.20, 0.10]
target = 1
```

then:

```text
dlogits = [0.70, -0.80, 0.10]
```

## ML Concept

This is the standard simplified gradient for softmax cross entropy.

It says:

- decrease the scores that are too high
- increase the target score if it was too low

## Category-Theory Concept

This is local derivative information for one part of the composed prediction
path.

The next loops compose that local derivative back into parameter gradients.

## Output-Head And Bias Gradients

The problem this block solves is:

> Convert the logit gradient into gradients for the output matrix and bias.

The core loop:

```rust,ignore
for (vocab_id, dlogit) in dlogits.iter().copied().enumerate() {
    grad_bias[vocab_id] += dlogit;

    for (feature, x_feature) in x.iter().copied().enumerate() {
        grad_lm_head[feature][vocab_id] += x_feature * dlogit;
    }
}
```

## Rust Syntax

The outer loop visits every vocabulary output.

The inner loop visits every feature of the input vector.

The bias gradient is just the logit gradient.

The weight gradient is:

```text
input feature * output gradient
```

## ML Concept

For a linear layer:

```text
logits = xW + b
```

the gradient of a weight is:

```text
input activation * output gradient
```

This is the same pattern used in larger neural networks.

## Category-Theory Concept

This is the local backward map for the affine projection stage.

It translates changes needed at the output object `Logits` into changes in the
parameter object.

## Embedding Gradient

The problem this block solves is:

> Move the output error backward through the language-model head to the input
> embedding row.

The block:

```rust,ignore
for (feature, grad_feature) in grad_embedding[input_id].iter_mut().enumerate() {
    let dx = params.lm_head[feature]
        .iter()
        .zip(dlogits.iter())
        .map(|(weight, dlogit)| weight * dlogit)
        .sum::<f32>();

    *grad_feature += dx;
}
```

## Rust Syntax

The loop mutates the gradient row for the input token.

For each feature, it pairs:

```text
weights from that feature to every vocab output
dlogits for every vocab output
```

Then it sums:

```text
weight * dlogit
```

## ML Concept

This is backpropagation through the linear head.

It tells the embedding row how it should change so the future logits improve.

Only the row for the current input token receives an embedding gradient.

## Category-Theory Concept

This is another local backward map.

The training endomorphism is built by composing local derivative information
from output back toward parameters.

## Parameter Update

The problem this block solves is:

> Turn accumulated gradients into new parameters.

The code computes:

```rust,ignore
let batch_scale = 1.0 / self.dataset.len() as f32;
let learning_rate = self.learning_rate.value();
let mut updated = params.clone();
```

Then it subtracts scaled gradients from every parameter.

## Rust Syntax

`batch_scale` averages the accumulated gradients.

`learning_rate` extracts the raw scalar.

`updated = params.clone()` creates the output parameter object.

The following loops mutate `updated`, not the original `params`.

Finally:

```rust,ignore
Ok(updated)
```

returns the new model state.

## ML Concept

The update rule is:

```text
parameter_new = parameter_old - learning_rate * average_gradient
```

This is gradient descent.

## Category-Theory Concept

The final result has the same object type as the input:

```text
Parameters -> Parameters
```

That completes the endomorphism.

## Regression Test

The problem this block solves is:

> Prove the learner-visible promise that repeated training reduces loss on the
> tiny dataset.

The test:

```rust,ignore
#[test]
fn repeated_training_step_reduces_loss() -> CtResult<()> {
    let tokens = TokenSequence::from_indices([1, 2, 3, 4, 1, 2, 3, 4])?;
    let dataset = DatasetWindowing.apply(tokens)?;
    let params = Parameters::init(VocabSize::new(5)?, ModelDimension::new(4)?);
    let before = average_loss(&params, &dataset)?;
    let train_step = TrainStep::new(dataset.clone(), LearningRate::new(1.0)?);
    let trained = apply_endomorphism_n_times(&train_step, params, StepCount::new(80))?;
    let after = average_loss(&trained, &dataset)?;

    assert!(after.value() < before.value());
    Ok(())
}
```

## Rust Syntax

The test returns `CtResult<()>`, so it can use `?`.

It builds:

- a token sequence
- a training set
- initial parameters
- a training step

Then it applies the endomorphism 80 times and checks the loss decreased.

## ML Concept

This is not a benchmark.

It is a sanity check:

```text
training should make the tiny model better on the tiny data
```

## Category-Theory Concept

The test exercises repeated endomorphism application:

```text
Parameters0 -> Parameters1 -> ... -> Parameters80
```

## Run The Example

<details>
<summary>Source snapshot: examples/03_training_endomorphism.rs</summary>

```rust,ignore
{{#include ../../examples/03_training_endomorphism.rs}}
```

</details>

Run:

```bash
cargo run --example 03_training_endomorphism
```

Expected pattern:

```text
loss before: ...
loss after:  ...
```

The second number should be smaller.

## Core Mental Model

In Rust terms:

```text
TrainStep implements Morphism<Parameters, Parameters>
```

In ML terms:

```text
one full-batch gradient descent update
```

In category-theory terms:

```text
an endomorphism that can be iterated
```

## Checkpoint

Why is it useful that training returns `Parameters` instead of a raw matrix?

A strong answer:

> Because the output can immediately be used as the input to the next
> `TrainStep`, preserving the `Parameters -> Parameters` endomorphism shape.

## Further Reading

- [Glossary](glossary.md): endomorphism, parameters, learning rate, gradient
- [References](references.md): gradient descent, softmax regression, and Rust error handling
