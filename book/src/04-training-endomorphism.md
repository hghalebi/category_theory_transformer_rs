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

> Reader orientation:
> Do not read this chapter as a full backpropagation engine. It is a small,
> explicit training step whose purpose is to make the shape `Parameters ->
> Parameters` visible and runnable.

## Chapter Outcomes

By the end of this chapter, you should be able to:

- explain why one training step is modeled as `Parameters -> Parameters`,
- separate loss measurement from parameter update,
- compare the tiny `TrainStep(dataset, learning_rate)` boundary with a
  production optimizer loop that calls `zero_grad`, `backward`, and `step`.

## What You Already Know

If you have seen gradient descent, you already know the informal movement:
parameters are adjusted and then used again. If you know Rust, you already know
that a function can return the same type it receives. This chapter names that
shape precisely: a training step is an endomorphism on `Parameters`.

## Update Trace Before Source

Before reading `src/training.rs`, keep this one-step trace in view. It separates
loss measurement, gradient accumulation, the parameter update, and repetition.

| Stage | Rust shape | Plain meaning | What to check |
| --- | --- | --- | --- |
| Current state | `Parameters` | embeddings, output weights, and bias before the step | What object is being updated? |
| Training data | `TrainingSet` | adjacent input-target examples | Is the update using examples, not one prediction alone? |
| Forward pass | `TokenId -> Vector -> Logits -> Distribution` | predict with the current parameters | Are predictions computed before gradients are accumulated? |
| Error signal | `dlogits[target_id] -= 1.0` | probability minus target indicator | Which target index changes the gradient? |
| Gradient buffers | `grad_embedding`, `grad_lm_head`, `grad_bias` | accumulated directions for each parameter group | Which buffer matches which parameter group? |
| Average step | `batch_scale` and `LearningRate` | scale gradients before subtracting them | Is this one full-batch update? |
| New state | `Parameters` | updated model state with the same shape | Did the output remain reusable as model state? |

One optimizer update has this shape:

```text
Parameters
  -> predictions on TrainingSet
  -> gradients
  -> Parameters
```

Repeated optimization is not a different kind of arrow. It is the same arrow
used again:

```text
Parameters0 -> Parameters1 -> Parameters2 -> ... -> ParametersN
```

That is the chapter's main separation. `Parameters -> Loss` measures the model.
`Parameters -> Parameters` updates the model. The first is diagnostic. The
second is repeatable training.

The local update rule in this chapter is the same first-order shape used in
standard gradient descent:

```text
parameter = parameter - learning_rate * average_gradient
```

In the Rust source, that appears as:

```rust,ignore
*value -= learning_rate * grad * batch_scale;
```

The chapter uses a full-batch step, so one call to `TrainStep::apply` reads all
examples in the `TrainingSet`, averages their gradients with `batch_scale`, and
returns a new `Parameters` value. The tests repeat that one endomorphism with
`apply_endomorphism_n_times`.

## Training Debugging Checklist

When training output looks surprising, separate four questions before changing
the update rule:

| Question | Safe answer in this chapter | Common mistake |
| --- | --- | --- |
| What object is updated? | `Parameters` | treating `Loss` as the updated object |
| What object measures quality? | `Loss` from `Parameters x TrainingSet` | returning loss instead of new parameters |
| What repeats? | the same `TrainStep : Parameters -> Parameters` | inventing a new arrow for every step count |
| What controls update size? | `LearningRate` and the averaged gradient | assuming more steps always means better behavior |

The example prints both roles:

```text
TrainStep : Parameters -> Parameters
Parameters x TrainingSet -> Loss
```

Those lines are deliberately different. `Loss` tells you how the current
parameters perform on the dataset. It is evidence, not the next model state.
`TrainStep` returns the next model state. That is what makes repetition legal:

```text
Parameters0 -> Parameters1 -> ... -> Parameters80
```

Use this diagnostic when changing `StepCount`:

| If you change | You are testing | You are not proving |
| --- | --- | --- |
| `StepCount::new(1)` | one update preserves state shape | that one update is enough training |
| `StepCount::new(10)` | repeated updates can improve the tiny dataset | that all datasets behave the same |
| `StepCount::new(200)` | the same endomorphism can be iterated many times | that more steps can never overshoot or plateau |

The category-theory lesson is stable even when the numeric loss changes in
different ways:

```text
the update remains Parameters -> Parameters
the measurement remains Parameters x TrainingSet -> Loss
```

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

## Source Reading Bridge: One Step Has Four Responsibilities

The short list above names the file's pieces, but it does not yet tell you how
to read the main function. The central method is `TrainStep::apply` in
`src/training.rs`. Read it as four responsibilities in order:

```text
validate the current Parameters
run the current model on each training example
accumulate gradients for embedding, output weights, and bias
subtract a learning-rate-scaled average gradient to create new Parameters
```

The ML intuition is gradient descent. A loss signal does not replace the model.
It tells each parameter which direction would reduce the current error on the
training set. The code makes that visible by separating the diagnostic value
from the state update:

```text
average_loss(&params, &dataset) -> Loss
TrainStep::apply(params)       -> Parameters
```

That difference matters. If `TrainStep::apply` returned `Loss`, it could tell
you how bad the current model is, but it could not be composed with itself for
the next update.

The category-theory connection is the same boundary in a shorter form:

```text
TrainStep(dataset, learning_rate) : Parameters -> Parameters
```

The dataset and learning rate configure which update arrow you have. The
gradient buffers are internal machinery used while building the output object;
they are not the object being returned by the morphism.

Checkpoint:

```text
If `TrainStep::apply` returned `Loss` instead of `Parameters`, what ability
would `apply_endomorphism_n_times` lose?
```

## Production Optimizer Boundary

Production frameworks usually split the training loop across model parameters,
stored gradients, an optimizer object, and an optimizer step. PyTorch's
`torch.optim` documentation describes optimizers as objects that hold current
state and update parameters from computed gradients. The common loop shape is:

```text
optimizer.zero_grad()
output = model(input)
loss = loss_fn(output, target)
loss.backward()
optimizer.step()
```

This book compresses the same teaching shape into one explicit Rust morphism:

```text
TrainStep(dataset, learning_rate) : Parameters -> Parameters
```

The tiny Rust boundary is smaller than a production optimizer. It does not
model momentum, parameter groups, optimizer state dictionaries, closures,
schedulers, mixed precision, or distributed training. It keeps one full-batch
gradient update inspectable.

| Production training responsibility | Tiny Rust teaching boundary |
| --- | --- |
| optimizer owns parameter groups and update state | `TrainStep` owns `TrainingSet` and `LearningRate` |
| `loss.backward()` computes gradients | `TrainStep::apply` accumulates local gradients directly |
| `optimizer.step()` updates parameters | `TrainStep::apply` returns a new `Parameters` value |
| `zero_grad()` manages stored gradient buffers | gradient buffers are local variables inside one update |
| schedulers may change learning rates across epochs | one `LearningRate` configures one repeated endomorphism |

When you return to a framework, the useful transfer question is:

```text
which object owns the update state, and which call turns current parameters
into next parameters?
```

## Source-Backed Precision Rules

This chapter uses external sources to keep the tiny update honest. Each source
supports a limited claim; these citations are not proof that this crate is a
production optimizer or a full automatic-differentiation engine.

| Source | What the source supports | Local rule in this chapter | Rust evidence |
| --- | --- | --- | --- |
| [D2L Gradient Descent](https://d2l.ai/chapter_optimization/gd.html) | First-order gradient descent updates a value by moving against the gradient, and the learning rate controls whether the step is useful or unstable. | The local update is `parameter = parameter - learning_rate * average_gradient`; do not claim every step count or learning rate must improve every dataset. | `*value -= learning_rate * grad * batch_scale;`, `LearningRate`, `StepCount` |
| [D2L Backpropagation and Computational Graphs](https://d2l.ai/chapter_multilayer-perceptrons/backprop.html) | Backpropagation computes gradients through intermediate variables using the chain rule in reverse order. | This chapter hand-computes the local softmax-linear gradients for one tiny model; it is not a general autograd tape. | `dlogits[target_id] -= 1.0`, `grad_lm_head`, `grad_embedding` |
| [Automatic differentiation in machine learning: a survey](https://arxiv.org/abs/1502.05767) | Automatic differentiation is broader than backpropagation and distinct from symbolic differentiation and finite differences. | Do not call this chapter's hand-written gradient buffers an AD engine; they are one visible gradient path for one tiny model. | `TrainStep::apply`, `grad_embedding`, `grad_lm_head`, `grad_bias` |
| [PyTorch `torch.optim`](https://docs.pytorch.org/docs/stable/optim.html) | A production optimizer owns update state and updates parameters after gradients have been computed. | `TrainStep` compresses `zero_grad`, `backward`, and `step` into one inspectable full-batch teaching boundary. | `TrainStep(dataset, learning_rate) : Parameters -> Parameters` |
| [Backprop as Functor](https://arxiv.org/abs/1711.10455) | Parameter-update rules can be studied compositionally under stated assumptions. | The categorical claim here is narrower: one fixed training step is an endomorphism on `Parameters`; the chapter does not prove a monoidal-functor result. | `impl Morphism<Parameters, Parameters> for TrainStep`, `apply_endomorphism_n_times` |

The transfer pattern is:

```text
source claim -> local typed boundary -> validation command or test
```

For this chapter, that means reading `cargo run --example
03_training_endomorphism` and the `src/training.rs` tests as evidence for the
tiny `Parameters -> Parameters` boundary, not as evidence for every production
training system.

## Worked Example: Repeating One Update

The smallest first-principles version of a repeated update is a number being
moved a little at a time:

```rust
fn step_toward_zero(value: f32, learning_rate: f32) -> f32 {
    value - learning_rate * value
}

let once = step_toward_zero(10.0, 0.1);
let twice = step_toward_zero(once, 0.1);

assert!(twice < once);
```

The real training code applies the same repeatable-update idea to `Parameters`,
not to one scalar. The output stays the same kind of object as the input, so the
update can be run again.

## Self-Check

Before reading the full training step, explain why `Parameters -> Parameters`
is repeatable but `Parameters -> Loss` is not.

## One Step Before Many Steps

Training becomes easier to reason about if you separate two ideas.

One training step has the shape:

```text
Parameters -> Parameters
```

It reads the dataset, computes predictions, accumulates gradients, subtracts a
learning-rate-scaled average gradient, and returns updated model state.

Repeated training is just iteration of that same shape:

```text
Parameters0 -> Parameters1 -> Parameters2 -> ... -> ParametersN
```

The chapter's category-theory word for the one-step shape is endomorphism. The
ML word for the update rule is gradient descent. The Rust evidence is the trait
implementation:

```rust,ignore
impl Morphism<Parameters, Parameters> for TrainStep
```

The tests in `src/training.rs` protect the learner-visible claims: one training
step preserves the parameter shape, out-of-range targets fail with a typed
error, and repeated steps reduce loss on the tiny dataset.

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

## Worked Example: Why Subtracting A Negative Gradient Increases The Target

The update rule can feel backwards the first time you see it. The code
subtracts gradients:

```text
parameter = parameter - learning_rate * gradient
```

So how can training increase the target score?

Use the same three-class example:

```text
probs  = [0.70, 0.20, 0.10]
target = 1
```

After the target correction:

```text
dlogits = [0.70, -0.80, 0.10]
```

Now look only at the bias update with learning rate `0.1` and one example:

```text
bias[0] = 0.0 - 0.1 *  0.70 = -0.07
bias[1] = 0.0 - 0.1 * -0.80 =  0.08
bias[2] = 0.0 - 0.1 *  0.10 = -0.01
```

The non-target classes had positive gradients, so subtracting them lowers their
biases. The target class had a negative gradient, so subtracting it raises the
target bias.

That is the local version of gradient descent: move parameters in the direction
that lowers loss. In this tiny classifier, the direction says "make the target
logit larger and make the overconfident non-target logits smaller."

The Rust path is:

```text
dlogits
  -> grad_bias
  -> bias -= learning_rate * grad * batch_scale
```

For output weights, the same sign passes through `x_feature * dlogit`. For the
embedding row, the sign passes backward through the output weights. The full
training step is bigger, but the sign logic starts here.

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

The update can be read as a loop around the same object:

```text
Parameters_t
    |
    | prediction on TrainingSet
    v
Average Loss
    |
    | local gradients
    v
Gradient Accumulators
    |
    | subtract learning_rate * average_gradient
    v
Parameters_{t+1}
```

The diagram has one important boundary: the first and last objects are both
`Parameters`. Everything in the middle explains how one state becomes the next
state.

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

It builds a token sequence, turns it into a training set, initializes
parameters, and configures a training step.

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

The companion tests check the one-step contract too. One update keeps the same
vocabulary size and model dimension, and invalid targets are rejected before an
unsafe index can enter gradient accumulation.

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
Typed transformation:
TrainStep : Parameters -> Parameters
Repeated endomorphism:
Parameters0 -> Parameters1 -> ... -> Parameters80
Measurement:
Parameters x TrainingSet -> Loss
```

The second number should be smaller.

## Example Output Transfer Checklist

The example output is deliberately small. It gives you two measurements and
then names the update shape that produced the second measurement.

Use the printed lines this way:

| Example output | Boundary to own | Shortcut to reject |
| --- | --- | --- |
| `loss before: ...` | measure the initial state with `Parameters x TrainingSet -> Loss` | treating the loss measurement as the training update |
| `loss after: ...` | measure the state after repeated updates | assuming one lower loss proves a full optimizer is correct |
| `TrainStep : Parameters -> Parameters` | one configured step consumes model state and returns model state | returning `Loss`, loose gradients, or one raw matrix from `apply` |
| `Parameters0 -> Parameters1 -> ... -> Parameters80` | the same endomorphism can be applied again | repeating `Parameters -> Loss` as if it were training |
| `Parameters x TrainingSet -> Loss` | evaluation needs both model state and examples | judging the loop from one prediction alone |

This is the same separation used in standard gradient-descent explanations:
compute a loss and its gradient, then update the parameters in the negative
gradient direction. The measurement tells you whether the model improved. The
endomorphism is the repeatable state transition that makes training possible.

If you only remember one distinction from this chapter, remember this:

```text
Parameters -> Loss        measures
Parameters -> Parameters  trains
```

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

## Where This Leaves Us

This chapter turned training into a repeatable typed transformation. The model
state enters as `Parameters`, the training step computes gradients from the tiny
dataset, and the updated model state leaves as `Parameters` again.

The next chapter, [Functors, Naturality, Monoids, and Chain
Rule](05-structure-and-calculus.md), steps back from the training loop and names
reusable structures that appear across the whole course: mapping inside
wrappers, changing wrapper shapes consistently, combining traces, and composing
local derivative rules.

## Further Reading

The problem this section solves is transfer. A framework training loop compresses
several responsibilities into familiar calls. This chapter expands those
responsibilities so the reader can see which object is measured, which object is
updated, and why the update can repeat.

Start from the local Rust evidence:

```text
average_loss(&params, &dataset) -> Loss
TrainStep::apply(params)       -> Parameters
apply_endomorphism_n_times     -> Parameters
```

Then compare that with a framework loop:

```text
optimizer.zero_grad()
loss = loss_fn(model(input), target)
loss.backward()
optimizer.step()
```

The framework loop is compact because the model, gradient buffers, optimizer
state, parameter groups, and update rule live behind framework objects. The
teaching path is expanded because the reader needs to separate four ideas:

| Framework responsibility | Tiny Rust question |
| --- | --- |
| clear old gradients | Which temporary gradient accumulators start empty inside `TrainStep::apply`? |
| compute current loss | Which call has shape `Parameters x TrainingSet -> Loss`? |
| compute gradients | Which local derivative changes `Distribution` into a logit gradient? |
| update parameters | Which call returns the next full `Parameters` object? |

Read the sources in this order:

1. [D2L Gradient Descent](https://d2l.ai/chapter_optimization/gd.html): use it
   for the update direction and learning-rate intuition.
2. [D2L Backpropagation and Computational Graphs](https://d2l.ai/chapter_multilayer-perceptrons/backprop.html):
   use it for the forward-then-reverse gradient story.
3. [Automatic differentiation in machine learning: a survey](https://arxiv.org/abs/1502.05767):
   use it to keep "automatic differentiation", "backpropagation", "symbolic
   differentiation", and "finite differences" separate.
4. [PyTorch `torch.optim`](https://docs.pytorch.org/docs/stable/optim.html):
   use it to recognize `zero_grad`, `backward`, and `step` as production
   boundaries.
5. [PyTorch Autograd mechanics](https://docs.pytorch.org/docs/stable/notes/autograd.html):
   use it to contrast graph-recording autograd with this chapter's hand-written
   gradient path.
6. [Backprop as Functor](https://arxiv.org/abs/1711.10455): use it only as
   advanced context for compositional update rules.

The transfer bridge is:

```text
production loop
  -> measure current model
  -> compute gradients
  -> update optimizer/model state
  -> repeat
```

The category-theory bridge is smaller and stricter:

```text
Parameters x TrainingSet -> Loss
TrainStep(dataset, learning_rate) : Parameters -> Parameters
```

The first boundary measures. The second boundary updates. Only the second one
is the endomorphism that can be repeated by `apply_endomorphism_n_times`.

Checkpoint:

```text
When reading an external optimizer or autograd reference, can you name which
part corresponds to Parameters x TrainingSet -> Loss and which part
corresponds to TrainStep(dataset, learning_rate) : Parameters -> Parameters?
```

These pages connect the tiny update to the surrounding vocabulary and source
material:

- [Glossary](glossary.md): endomorphism, parameters, learning rate, gradient
- [References](references.md): gradient descent, computational graphs, backpropagation, and compositional learning

## Practice After This Chapter

Use [Exercise 5](exercises.md#exercise-5-change-the-training-repetition-count)
to change the number of repeated training steps. The goal is not to tune a real
model. The goal is to see why a `Parameters -> Parameters` update can be
applied again and again.

## Retrieval Practice

### Recall

Recover the update shape before explaining the gradient.

1. What makes `TrainStep` an endomorphism?
2. Which line changes the probability vector into the logit gradient for the
   target class?
3. Which helper repeats the same `Parameters -> Parameters` step many times?

### Explain

Separate measurement from update.

1. Why is `Parameters -> Loss` useful for evaluation but not itself a training
   endomorphism?
2. Why does the training code validate input and target token bounds before
   accumulating gradients?
3. Why does subtracting a negative target gradient increase the target bias or
   target weight?

### Apply

Use the sign trace from this chapter.

1. Suppose:

   ```text
   probs  = [0.65, 0.25, 0.10]
   target = 2
   learning_rate = 0.1
   batch_scale = 1.0
   bias starts at [0.0, 0.0, 0.0]
   ```

   What is `dlogits`, and what is the updated bias?
2. If you changed `StepCount::new(80)` to `StepCount::new(1)`, what would you
   expect to happen to the loss, and why?
3. If the dataset has four examples, why does the code multiply each accumulated
   gradient by `batch_scale = 0.25` before updating parameters?

### Debug

For each invalid shortcut, name the broken shape or missing state:

```text
returning Loss from TrainStep.apply
updating only lm_head and discarding embedding and bias
repeating Parameters -> Loss as if it were Parameters -> Parameters
skipping token bounds checks before indexing gradient buffers
```

A strong answer should mention the outer loop shape:

```text
Parameters_t -> Parameters_{t+1}
```

The loss and gradients explain how the update is computed. They are not the
object that must be returned from the training step.
