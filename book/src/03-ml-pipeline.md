# The Tiny ML Pipeline

The problem this chapter solves is:

> The abstract `Morphism` trait needs concrete machine-learning arrows that
> turn token data into predictions and loss.

The whole prediction-and-loss path is:

```text
TokenSequence -> TrainingSet
TokenId       -> Vector
Vector        -> Logits
Logits        -> Distribution
Distribution x TokenId -> Loss
```

In ordinary ML language, the path turns a token stream into adjacent training
pairs, looks up an embedding vector for the current token, uses a linear layer
to score every possible next token, normalizes those scores with softmax, and
then measures surprise with cross entropy.

In category-theory language:

> Each stage is a morphism, and the legal stages compose.

> Reader orientation:
> This is the first chapter where all three subjects meet at once. When the code
> feels dense, follow the pipeline order: data preparation first, prediction
> second, loss third.

## What You Already Know

If you know ML, you already know the rough path: prepare data, make a
prediction, and measure the error. If you know Rust, you already know that each
step can have a concrete input and output type. This chapter combines those two
habits by making each ML step implement the same morphism interface.

## Source Snapshot

This file owns the concrete ML arrows.

<details>
<summary>Source snapshot: src/ml.rs</summary>

```rust,ignore
{{#include ../../src/ml.rs}}
```

</details>

## The Whole File

`src/ml.rs` defines:

```text
DatasetWindowing
Embedding
LinearToLogits
Softmax
CrossEntropy
DirectPredict
average_loss
composed_prediction_matches_direct_prediction
```

The chapter reads them in pipeline order.

Read each block through the same three lenses:

```text
Rust syntax:
what struct, trait implementation, loop, or error branch does the code use?

ML concept:
which prediction, loss, or data-preparation step does the block implement?

Category theory concept:
which object, product, morphism, composition, or commutative check appears?
```

## Worked Example: Normalizing Scores

The smallest first-principles version of "normalize scores into probabilities"
does not need a model yet:

```rust
let scores = [1.0_f32, 2.0, 3.0];
let total: f32 = scores.iter().sum();
let probabilities: Vec<f32> = scores.iter().map(|score| score / total).collect();

let probability_sum: f32 = probabilities.iter().sum();
assert!((probability_sum - 1.0).abs() < 1e-6);
```

The real `Softmax` implementation is more careful than this toy normalization:
it uses exponentials, subtracts the maximum score for numerical stability, and
validates the result through `Distribution::new`.

## Self-Check

Why is it useful for the probability-validation boundary to live in
`Distribution::new` instead of in every caller that uses probabilities?

## `DatasetWindowing`

The problem this block solves is:

> A token sequence must become input-target pairs before supervised
> next-token training can happen.

The block:

```rust,ignore
/// Turns adjacent tokens into next-token training examples.
#[derive(Debug, Clone)]
pub struct DatasetWindowing;

impl Morphism<TokenSequence, TrainingSet> for DatasetWindowing {
    fn name(&self) -> &'static str {
        "dataset_windowing"
    }

    fn apply(&self, tokens: TokenSequence) -> CtResult<TrainingSet> {
        if tokens.as_slice().len() < 2 {
            return Err(CtError::EmptyInput(
                "dataset windowing requires at least 2 tokens",
            ));
        }

        TrainingSet::new(
            tokens
                .as_slice()
                .windows(2)
                .map(|pair| Product::new(pair[0], pair[1])),
        )
    }
}
```

### Rust Syntax: Unit Struct

```rust,ignore
pub struct DatasetWindowing;
```

This is a unit struct.

It stores no fields because the operation has no configuration.

The value itself represents the transformation.

### Rust Syntax: Morphism Shape

```rust,ignore
impl Morphism<TokenSequence, TrainingSet> for DatasetWindowing
```

This says:

```text
DatasetWindowing : TokenSequence -> TrainingSet
```

So it consumes the raw sequence stage and produces the training-example stage.

### Rust Syntax: Why It Requires At Least Two Tokens

```rust,ignore
if tokens.as_slice().len() < 2 {
    return Err(CtError::EmptyInput(
        "dataset windowing requires at least 2 tokens",
    ));
}
```

`TokenSequence` only guarantees at least one token.

But next-token training requires at least one adjacent pair.

One token:

```text
[7]
```

produces zero pairs.

Two tokens:

```text
[7, 8]
```

produce one pair:

```text
7 -> 8
```

So this morphism owns the stronger validation rule.

### Rust Syntax: `windows(2)`

```rust,ignore
tokens.as_slice().windows(2)
```

This walks adjacent pairs:

```text
[1, 2, 3, 4]
```

becomes:

```text
[1, 2]
[2, 3]
[3, 4]
```

Each pair becomes:

```rust,ignore
Product::new(pair[0], pair[1])
```

That is a `TrainingExample`.

### ML Concept

This is the data-preparation step for next-token prediction.

### Category Theory Concept

This is a morphism between two structured objects:

```text
non-empty token list -> non-empty product list
```

The output examples are product objects:

```text
TokenId x TokenId
```

## `Embedding`

The problem this block solves is:

> A discrete token ID needs to become a dense vector before the model can use
> linear algebra.

The core block:

```rust,ignore
#[derive(Debug, Clone)]
pub struct Embedding {
    table: Vec<Vec<f32>>,
}

impl Embedding {
    pub fn from_parameters(params: &Parameters) -> Self {
        Self {
            table: params.embedding_table().to_vec(),
        }
    }
}

impl Morphism<TokenId, Vector> for Embedding {
    fn name(&self) -> &'static str {
        "embedding"
    }

    fn apply(&self, token: TokenId) -> CtResult<Vector> {
        let Some(row) = self.table.get(token.index()) else {
            return Err(CtError::OutOfRange {
                kind: "token",
                index: token.index(),
                limit: self.table.len(),
            });
        };

        Ok(Vector::new(row.clone()))
    }
}
```

### Rust Syntax: Stored Table

```rust,ignore
table: Vec<Vec<f32>>
```

The embedding table has shape:

```text
vocab_size x model_dimension
```

Each row is the vector for one token.

### Rust Syntax: Constructor From Parameters

```rust,ignore
pub fn from_parameters(params: &Parameters) -> Self
```

The embedding morphism is built from model parameters.

It copies the table out of `Parameters`:

```rust,ignore
params.embedding_table().to_vec()
```

This keeps the morphism simple and owned for the tiny tutorial.

### Rust Syntax: Morphism Shape

```rust,ignore
impl Morphism<TokenId, Vector> for Embedding
```

This says:

```text
Embedding : TokenId -> Vector
```

### Rust Syntax: Bounds Check

```rust,ignore
let Some(row) = self.table.get(token.index()) else {
    return Err(CtError::OutOfRange { ... });
};
```

The code does not assume every `TokenId` is valid for every embedding table.

It checks the row lookup at the boundary where the table is used.

### Rust Syntax: Why Clone The Row

```rust,ignore
Ok(Vector::new(row.clone()))
```

The morphism returns an owned `Vector`.

The row inside the table is borrowed, so the code clones it into the output
object.

This is a deliberate ownership boundary.

### ML Concept

An embedding converts a symbolic token into numerical features.

### Category Theory Concept

It is an arrow:

```text
TokenId -> Vector
```

## `LinearToLogits`

The problem this block solves is:

> A hidden vector must be projected into one raw score per vocabulary item.

The shape is:

```text
Vector -> Logits
```

The core implementation stores:

```rust,ignore
pub struct LinearToLogits {
    weight: Vec<Vec<f32>>,
    bias: Vec<f32>,
}
```

The dimensions are:

```text
weight: d_model x vocab_size
bias: vocab_size
input: d_model
output: vocab_size
```

### Rust Syntax: Shape Validation

Inside `apply`, the code checks:

```rust,ignore
if self.weight.len() != d_model {
    return Err(CtError::ShapeMismatch { ... });
}
```

This catches a matrix whose row count does not match the input vector length.

Then each row checks:

```rust,ignore
if self.weight[feature].len() != vocab_size {
    return Err(CtError::ShapeMismatch { ... });
}
```

This catches rows whose column count does not match the output vocabulary size.

### Rust Syntax: Linear Computation

The output begins as the bias:

```rust,ignore
let mut out = self.bias.clone();
```

Then each input feature contributes to every vocabulary score:

```rust,ignore
for (feature, input_value) in input.as_slice().iter().enumerate() {
    for (vocab_id, output_value) in out.iter_mut().enumerate() {
        *output_value += input_value * self.weight[feature][vocab_id];
    }
}
```

Mathematically:

```text
logits = input x weight + bias
```

### ML Concept

This is the language-model head.

It scores each possible next token.

### Category Theory Concept

It is a morphism:

```text
Vector -> Logits
```

It can compose after `Embedding` because `Embedding` returns `Vector`.

## `Softmax`

The problem this block solves is:

> Raw scores are not probabilities. They must be normalized into a valid
> distribution.

The block:

```rust,ignore
#[derive(Debug, Clone)]
pub struct Softmax;

impl Morphism<Logits, Distribution> for Softmax {
    fn name(&self) -> &'static str {
        "softmax"
    }

    fn apply(&self, logits: Logits) -> CtResult<Distribution> {
        if logits.as_slice().is_empty() {
            return Err(CtError::EmptyInput("softmax"));
        }

        let max_value = logits
            .as_slice()
            .iter()
            .copied()
            .fold(f32::NEG_INFINITY, f32::max);
        let mut exps = Vec::with_capacity(logits.as_slice().len());
        let mut sum = 0.0;

        for value in logits.as_slice() {
            let exp = (*value - max_value).exp();
            exps.push(exp);
            sum += exp;
        }

        if sum <= 0.0 || !sum.is_finite() {
            return Err(CtError::InvalidProbability("softmax"));
        }

        Distribution::new(exps.into_iter().map(|value| value / sum).collect())
    }
}
```

### Rust Syntax: Unit Struct

`Softmax` stores no state.

It is the operation itself.

### Rust Syntax: Morphism Shape

```rust,ignore
impl Morphism<Logits, Distribution> for Softmax
```

This says:

```text
Softmax : Logits -> Distribution
```

### Rust Syntax: Empty Check

Softmax over no scores is meaningless.

So the code rejects empty logits.

### Rust Syntax: Numerical Stability

```rust,ignore
let max_value = ...
let exp = (*value - max_value).exp();
```

Subtracting the maximum value keeps exponentials smaller and more stable.

It does not change the final probabilities because softmax is invariant under
adding or subtracting the same constant from every logit.

### Rust Syntax: Normalization

```rust,ignore
Distribution::new(exps.into_iter().map(|value| value / sum).collect())
```

The raw exponentials are divided by their sum.

Then the `Distribution` constructor validates the probability invariant.

This is good boundary design: softmax computes, and `Distribution::new`
enforces the distribution contract.

### ML Concept

Softmax turns raw model scores into probabilities.

High logits become high probabilities.

Low logits become low probabilities.

The output can be interpreted as:

```text
P(next token | current token)
```

### Category Theory Concept

Softmax is a morphism-like transformation:

```text
Logits -> Distribution
```

It changes the object from an unconstrained score vector into a probability
simplex-like object.

## `CrossEntropy`

The problem this block solves is:

> A model prediction must be compared to the actual target token to produce a
> scalar loss.

The block:

```rust,ignore
#[derive(Debug, Clone)]
pub struct CrossEntropy;

impl Morphism<Product<Distribution, TokenId>, Loss> for CrossEntropy {
    fn name(&self) -> &'static str {
        "cross_entropy"
    }

    fn apply(&self, input: Product<Distribution, TokenId>) -> CtResult<Loss> {
        let (distribution, target) = input.into_parts();

        let Some(probability) = distribution.as_slice().get(target.index()).copied() else {
            return Err(CtError::OutOfRange {
                kind: "target",
                index: target.index(),
                limit: distribution.as_slice().len(),
            });
        };

        Loss::new(-probability.max(1e-9).ln())
    }
}
```

### Rust Syntax: Input Type

```rust,ignore
Product<Distribution, TokenId>
```

Cross entropy needs both:

- the predicted distribution
- the correct target token

That pair is a product object.

### Rust Syntax: Splitting The Product

```rust,ignore
let (distribution, target) = input.into_parts();
```

This consumes the product and extracts both values.

### Rust Syntax: Target Bounds Check

```rust,ignore
distribution.as_slice().get(target.index())
```

The target token must be inside the probability vector.

If the distribution has 5 entries, target index 7 is invalid.

This error belongs here because this is the first place the target is used as
an index into the predicted distribution.

### Rust Syntax: Negative Log Likelihood

```rust,ignore
Loss::new(-probability.max(1e-9).ln())
```

The loss is:

```text
-ln(probability assigned to the correct token)
```

The `max(1e-9)` avoids taking the log of zero.

Then `Loss::new` validates the loss scalar.

### ML Concept

Cross entropy measures how surprised the model was by the true target.

If the model assigns high probability to the target, the loss is small.

If the model assigns low probability to the target, the loss is large.

### Category Theory Concept

Cross entropy consumes a product object:

```text
Distribution x TokenId
```

and maps it into:

```text
Loss
```

So its shape is:

```text
Product<Distribution, TokenId> -> Loss
```

## `DirectPredict`

The problem this block solves is:

> The course needs a direct implementation to compare against the composed
> prediction path.

`DirectPredict` stores parameters and implements:

```text
TokenId -> Distribution
```

Internally, it still performs:

```text
Embedding
LinearToLogits
Softmax
```

but it writes the steps directly.

This allows the code to test:

```text
composed path == direct path
```

That is the program's tiny commutative diagram check.

### Rust Syntax

`DirectPredict` is a struct that owns `Parameters`.

Its `apply` method calls the prediction steps directly instead of using
`Compose`.

### ML Concept

This is the direct prediction implementation.

It exists so the composed path can be checked against a straightforward path.

### Category Theory Concept

It provides the second path in a commutative diagram:

```text
composed path
direct path
```

## `average_loss`

The problem this function solves is:

> Training needs one scalar loss over the whole training set.

The function builds the composed prediction path:

```rust,ignore
let embedding = Embedding::from_parameters(params);
let linear = LinearToLogits::from_parameters(params);
let predict = Compose::<_, _, Vector>::new(embedding, linear);
let predict = Compose::<_, _, Logits>::new(predict, Softmax);
```

The resulting shape is:

```text
TokenId -> Distribution
```

Then each training example is evaluated:

```rust,ignore
let distribution = predict.apply(*example.first())?;
let loss = loss_fn.apply(Product::new(distribution, *example.second()))?;
```

Finally, the average is wrapped in `Loss::new`.

The function does not return a raw `f32`.

It returns a validated `Loss`.

### Rust Syntax

The function takes borrowed parameters and a borrowed dataset:

```rust,ignore
pub fn average_loss(params: &Parameters, dataset: &TrainingSet) -> CtResult<Loss>
```

It does not consume either one.

The function loops through examples, accumulates scalar losses, and divides by
the dataset length.

### ML Concept

Average loss summarizes model performance over the full training set.

### Category Theory Concept

It folds many example-level loss morphism results into one scalar object.

## `composed_prediction_matches_direct_prediction`

The problem this function solves is:

> The code should prove that the composed prediction pipeline and the direct
> implementation agree.

The composed path is:

```text
TokenId -> Vector -> Logits -> Distribution
```

The direct path is:

```text
TokenId -> Distribution
```

The function runs both on the same token and compares every probability with a
small floating-point tolerance.

Category-theoretically, this is a commutative diagram test:

```text
          composed
TokenId ------------> Distribution
   \                      ^
    \ direct              |
     ---------------------
```

The exact drawing is less important than the idea:

> Two paths through the system should produce the same meaning.

### Rust Syntax

The function builds one composed path with `Compose` and one direct path with
`DirectPredict`.

It compares probabilities pairwise with `approx_eq`.

### ML Concept

This verifies that refactoring the prediction path into smaller stages did not
change the predicted probabilities.

### Category Theory Concept

This is a commutative-diagram check in code.

## Run The Demo

Run:

```bash
cargo run --bin category_ml
```

Look at sections 2 through 5 in the output.

You should see:

```text
TokenSequence -> TrainingSet
prediction probabilities
loss for a target token
```

## Why This Matters

This chapter is where the course stops being abstract.

The code implements a real, tiny version of the common language-model training
path:

```text
context token -> hidden vector -> next-token probabilities -> loss
```

The implementation is small, but the boundaries are real. Invalid token lookup
returns `OutOfRange`, invalid matrix shape returns `ShapeMismatch`, empty
logits return `EmptyInput`, invalid probabilities return `InvalidProbability`,
and invalid loss returns `InvalidLoss`.

Errors are caught where the invalid data first becomes meaningful.

## Core Mental Model

In Rust terms:

```text
each ML operation implements Morphism<Input, Output>
```

In ML terms:

```text
prediction is embedding + linear projection + softmax
loss is negative log probability of the target
```

In category-theory terms:

```text
prediction is composition of arrows
loss consumes a product object
the direct and composed paths should commute
```

## Checkpoint

Where should an out-of-range target token be caught?

Correct answer:

> Inside `CrossEntropy`, because that is where the target is used to index the
> predicted distribution.

## Where This Leaves Us

This chapter assembled the first complete tiny ML path. A token sequence becomes
training examples, a token becomes a vector, a vector becomes logits, logits
become probabilities, and a probability distribution plus a target token becomes
loss.

The next chapter changes the question from "how do we evaluate one prediction?"
to "how do repeated updates change the model state?" That is where training
enters as an endomorphism.

## Further Reading

These pages connect the tiny pipeline to the surrounding vocabulary:

- [Glossary](glossary.md): logits, softmax, probability distribution, cross entropy
- [References](references.md): softmax regression and linear classifiers

## Retrieval Practice

### Recall

What are the concrete ML arrows in the prediction-and-loss path?

### Explain

Why does `CrossEntropy` consume a product of `Distribution` and `TokenId`?

### Apply

Given `TokenId -> Vector -> Logits -> Distribution`, write the Rust type that
must appear between `Embedding` and `Softmax`.
