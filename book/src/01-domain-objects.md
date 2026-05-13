# Domain Objects

The problem this chapter solves is:

> A machine-learning pipeline should not pass raw numbers around and hope
> everyone remembers what each number means.

Before this code talks about arrows, composition, loss, or training, it defines
the objects those arrows will connect.

In this course, a domain object means:

```text
raw representation
  + a meaningful name
  + optional validation
  + controlled access
```

For example:

```text
usize
```

could mean:

- a token index
- a vocabulary size
- a model dimension
- a matrix row count
- a training step count

Those are different concepts.

So the code creates different types.

> Reader orientation:
> In this chapter, focus on why a type exists before focusing on its syntax. A
> tuple struct, private field, constructor, or accessor is not decoration. It is
> a small boundary that tells the rest of the pipeline which states it may trust.

## Chapter Outcomes

By the end of this chapter, you should be able to:

- explain why `TokenId`, `VocabSize`, `ModelDimension`, and `StepCount` should
  not all be raw `usize` values at the teaching boundary,
- separate semantic wrappers from validated objects,
- name one invalid ML state that each constructor prevents before prediction,
  loss, or training sees it.

## What You Already Know

If you have used a Rust struct, you already know that a value can carry a name
instead of floating around as raw data. If you have used an ML pipeline, you
already know that a token index, a vector, a probability distribution, and a
loss value play different roles. This chapter turns that familiar separation
into explicit domain types.

The important move is not "wrap everything because wrappers are nice." The
important move is to ask what the rest of the pipeline is allowed to trust. Some
types only separate meanings. Other types also reject invalid values before
they can enter prediction, loss, or training.

## Worked Example: Naming One Number

The smallest version of the pattern looks like this:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TokenId(usize);

impl TokenId {
    fn new(index: usize) -> Self {
        Self(index)
    }

    fn index(self) -> usize {
        self.0
    }
}

assert_eq!(TokenId::new(3).index(), 3);
```

The real source file repeats that pattern with stronger validation where the
value has an invariant, such as "a distribution must contain probabilities that
sum to one."

## Self-Check

Before reading the full source snapshot, explain why `TokenId(3)` communicates
more than the raw number `3`.

## Two Kinds Of Domain Objects

Read the file with this distinction in mind.

| Kind | Example | What the type gives the pipeline |
| --- | --- | --- |
| Semantic wrapper | `TokenId`, `Vector`, `Logits` | A name that prevents one raw representation from being confused with another |
| Validated object | `TokenSequence`, `Distribution`, `Loss`, `VocabSize`, `LearningRate` | A constructor that rejects states later code should not have to handle |

Both kinds matter. A `TokenId` is useful even though any `usize` can become a
token ID at this layer, because it prevents accidental mixing with dimensions
or row counts. A `Distribution` needs a stronger boundary, because not every
`Vec<f32>` is a valid probability distribution.

This is the Rust API idea behind the chapter: put meaning and validation near
construction, then expose small accessors for the raw representation when lower
level code really needs it.

## Mistakes These Types Prevent

Before reading the whole file, scan the reason each type exists. The point is
not to wrap values for style. The point is to make common pipeline mistakes
harder to express.

| Domain type | Raw representation it replaces | Concrete mistake it prevents |
| --- | --- | --- |
| `TokenId` | `usize` | passing a vocabulary index where a model dimension or row count was expected |
| `TokenSequence` | `Vec<TokenId>` | training on an empty sequence or mutating a validated sequence after construction |
| `Vector` | `Vec<f32>` | treating hidden features as if they were vocabulary scores |
| `Logits` | `Vec<f32>` | treating raw scores as if they were probabilities |
| `Distribution` | `Vec<f32>` | computing loss from negative, non-finite, empty, or non-normalized probabilities |
| `Loss` | `f32` | accumulating a negative or non-finite objective value |
| `VocabSize` | `usize` | constructing parameters for a zero-token vocabulary |
| `ModelDimension` | `usize` | constructing embedding rows with zero width |
| `LearningRate` | `f32` | applying an optimizer step with zero, negative, or non-finite step size |
| `TrainingSet` | `Vec<TrainingExample>` | running training on no examples |
| `Parameters` | loose matrices and bias vectors | scattering model state across unrelated arrays without one named owner |

Use this table as the chapter's review checklist. When a later section shows
syntax, ask which mistake the syntax blocks.

## Source-Backed Precision Rules

This chapter uses Rust sources to keep the "domain object" claim precise. Each
source supports one local teaching rule, and each rule is tied to a concrete
constructor, accessor, example, or test. The chapter does not claim that every
wrapper is fully validated. Some types only separate meanings; other types
reject invalid states at construction.

| Source | What the source supports | Local rule in this chapter | Rust evidence |
| --- | --- | --- | --- |
| [Rust Book: Structs](https://doc.rust-lang.org/book/ch05-01-defining-structs.html) | Structs and tuple structs give data a named type, even when the stored representation is small. | Use `TokenId`, `Vector`, and `Logits` to separate meanings that would otherwise share `usize` or `Vec<f32>`. | `TokenId(usize)`, `Vector(Vec<f32>)`, `Logits(Vec<f32>)` |
| [Rust By Example: New Type Idiom](https://doc.rust-lang.org/rust-by-example/generics/new_types.html) | A wrapper type can make the compiler require the intended semantic role before a value enters a function. | Treat `TokenId`, `VocabSize`, and `ModelDimension` as compile-time role labels before adding heavier validation. | `TokenId`, `VocabSize`, `ModelDimension` |
| [Rust Book: Result](https://doc.rust-lang.org/stable/book/ch09-02-recoverable-errors-with-result.html) | `Result<T, E>` represents an operation that may either return a success value or an error value. | Use fallible constructors when raw input may violate an invariant. | `TokenSequence::new`, `Distribution::new`, `Loss::new`, `LearningRate::new` |
| [Rust API Guidelines: Type Safety](https://rust-lang.github.io/api-guidelines/checklist.html#type-safety-c-newtype) | Newtypes provide static distinctions and arguments should convey meaning through custom types. | Do not let `usize`, `f32`, or `Vec<f32>` cross teaching boundaries when they mean different ML concepts. | `VocabSize`, `ModelDimension`, `LearningRate`, `Product<Distribution, TokenId>` |
| [Rust API Guidelines: Dependability](https://rust-lang.github.io/api-guidelines/checklist.html#dependability-c-validate) | Functions should validate their arguments when invalid values would break later assumptions. | Validate once at construction, then let downstream morphisms trust the object. | `distribution_rejects_non_normalized_values`, `token_sequence_rejects_empty_input` |
| [Rust API Guidelines: Future Proofing](https://rust-lang.github.io/api-guidelines/checklist.html#future-proofing-c-sealed) | Private fields and encapsulated newtypes protect invariants and implementation details. | Expose small accessors such as `as_slice`, `value`, and `index` instead of public mutable fields. | `TokenSequence(Vec<TokenId>)`, `Distribution(Vec<f32>)`, `Parameters` accessors |

The transfer pattern is:

```text
source rule -> local domain type -> constructor, accessor, or test evidence
```

For this chapter, that means reading `cargo run --example 01_domain_objects`
and `cargo test domain::tests` as evidence for the small boundary claims:

```text
TokenSequence is non-empty
Distribution is non-empty, finite, non-negative, and normalized
shape and training configuration values are not interchangeable
```

It is not evidence that every future ML value has already been modeled. It is
evidence that the chapter's first layer of objects has explicit names,
construction boundaries, and validation where the later pipeline depends on an
invariant.

## Source Snapshot

This is the domain layer used by the whole tutorial.

<details>
<summary>Source snapshot: src/domain.rs</summary>

```rust,ignore
{{#include ../../src/domain.rs}}
```

</details>

## The Whole File

`src/domain.rs` defines the nouns in the tiny ML system:

```text
TokenId
TokenSequence
Vector
Logits
Distribution
Loss
VocabSize
ModelDimension
LearningRate
Product
TrainingExample
TrainingSet
Parameters
```

The ML pipeline needs all of them:

```text
TokenSequence -> TrainingSet
TokenId       -> Vector
Vector        -> Logits
Logits        -> Distribution
Distribution x TokenId -> Loss
Parameters    -> Parameters
```

The category-theory reading is:

> These are the objects that morphisms start from and end at.

The Rust reading is:

> These are wrapper types that prevent raw representation from leaking through
> the whole program.

Each major block below is meant to be read through three lenses:

```text
Rust syntax:
what does the code literally declare?

ML concept:
why does the training pipeline need this value?

Category theory concept:
what object, product, list, distribution, or morphism endpoint does it model?
```

The chapter follows the same order as the model pipeline. First it names token
data. Then it names hidden representations and probabilities. Then it names
loss, configuration, paired inputs, training data, and model state.

## `TokenId`

The problem this block solves is:

> A token index should not be confused with any other `usize`.

The block:

```rust,ignore
/// A vocabulary index. It is intentionally not a raw `usize` in public APIs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TokenId(usize);

impl TokenId {
    pub fn new(index: usize) -> Self {
        Self(index)
    }

    pub fn index(&self) -> usize {
        self.0
    }
}

impl From<usize> for TokenId {
    fn from(value: usize) -> Self {
        Self::new(value)
    }
}
```

### Rust Syntax

`TokenId` is a tuple struct with one private field:

```rust,ignore
pub struct TokenId(usize);
```

The struct is public, but the field is private.

That means other modules can name `TokenId`, pass it around, and call its
methods, but they cannot directly reach inside and mutate the raw `usize`.

### Why `new` Cannot Fail

```rust,ignore
pub fn new(index: usize) -> Self
```

Every `usize` is a valid token index at this layer.

The code does not know yet whether the token is inside a particular vocabulary.
That check happens later when a morphism tries to look up an embedding row.

So `TokenId::new` is infallible.

### Why `index` Exists

```rust,ignore
pub fn index(&self) -> usize {
    self.0
}
```

This accessor gives read-only access to the raw index when low-level code needs
it.

The type still prevents accidental mixing at the API boundary.

### ML Concept

In ML terms, `TokenId` is a vocabulary position.

If the vocabulary is:

```text
0 = <pad>
1 = I
2 = love
3 = Rust
4 = .
```

then:

```text
TokenId::new(3)
```

means the token `Rust`.

### Category Theory Concept

`TokenId` is one object in the category of this program's typed values.

Arrows such as `Embedding` start from this object:

```text
TokenId -> Vector
```

## `TokenSequence`

The problem this block solves is:

> A language model does not train directly on raw text. First, text becomes a
> sequence of token IDs. Then that sequence becomes input-target training pairs.

This block represents the middle stage:

```text
raw text
  -> tokens
  -> token sequence
  -> training examples
  -> model training
```

The block:

```rust,ignore
/// A sequence of tokens before it has been converted into training pairs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenSequence(Vec<TokenId>);

impl TokenSequence {
    pub fn new(tokens: impl IntoIterator<Item = TokenId>) -> CtResult<Self> {
        let tokens = tokens.into_iter().collect::<Vec<_>>();

        if tokens.is_empty() {
            return Err(CtError::EmptyInput("token sequence"));
        }

        Ok(Self(tokens))
    }

    pub fn from_indices(indices: impl IntoIterator<Item = usize>) -> CtResult<Self> {
        Self::new(indices.into_iter().map(TokenId::new))
    }

    pub fn as_slice(&self) -> &[TokenId] {
        &self.0
    }
}
```

### Rust Syntax: Documentation Comment

```rust,ignore
/// A sequence of tokens before it has been converted into training pairs.
```

This tells you the pipeline stage.

`TokenSequence` is not raw text.

It is also not yet training data.

It is the ordered token stream before adjacent pairs are created.

Example:

```text
[TokenId(1), TokenId(2), TokenId(3)]
```

can later become:

```text
TokenId(1) -> TokenId(2)
TokenId(2) -> TokenId(3)
```

### Rust Syntax: Derived Traits

```rust,ignore
#[derive(Debug, Clone, PartialEq, Eq)]
```

`Debug` allows test and debugging output.

`Clone` allows an explicit copy of the sequence.

`PartialEq` allows equality checks.

`Eq` says equality is total and well-behaved.

Order matters. These are not equal:

```text
[TokenId(1), TokenId(2)]
[TokenId(2), TokenId(1)]
```

### Rust Syntax: Private Vector

```rust,ignore
pub struct TokenSequence(Vec<TokenId>);
```

This wraps:

```text
Vec<TokenId>
```

but does not expose the vector directly.

That is important because the type's invariant is:

```text
TokenSequence is non-empty.
```

If the field were public, a caller could construct:

```rust,ignore
TokenSequence(vec![])
```

and bypass validation.

The private field forces construction through `TokenSequence::new` or
`TokenSequence::from_indices`.

### Rust Syntax: Constructor

```rust,ignore
pub fn new(tokens: impl IntoIterator<Item = TokenId>) -> CtResult<Self>
```

This accepts any input that can produce `TokenId` values:

- a vector
- an array
- a mapped iterator

The return type is:

```text
CtResult<TokenSequence>
```

So construction can succeed or fail.

### Rust Syntax: Collection

```rust,ignore
let tokens = tokens.into_iter().collect::<Vec<_>>();
```

This turns the flexible input into the concrete representation stored inside
the struct.

The `_` means Rust infers the element type as `TokenId`.

### Rust Syntax: Empty Check

```rust,ignore
if tokens.is_empty() {
    return Err(CtError::EmptyInput("token sequence"));
}
```

This is the invariant boundary.

An empty token stream cannot carry useful sequence information.

The error happens immediately, before invalid data enters the rest of the
pipeline.

### Rust Syntax: Successful Construction

```rust,ignore
Ok(Self(tokens))
```

Inside the `impl`, `Self` means `TokenSequence`.

So this is equivalent to:

```rust,ignore
Ok(TokenSequence(tokens))
```

The vector has already been validated, so the object is safe for later code to
trust.

### Rust Syntax: Convenience Constructor

```rust,ignore
pub fn from_indices(indices: impl IntoIterator<Item = usize>) -> CtResult<Self> {
    Self::new(indices.into_iter().map(TokenId::new))
}
```

This accepts raw indices and converts each one into `TokenId`.

The important design choice is delegation:

```text
from_indices -> new
```

Validation is not duplicated.

All construction still passes through the same non-empty check.

### Rust Syntax: Read-Only Access

```rust,ignore
pub fn as_slice(&self) -> &[TokenId] {
    &self.0
}
```

This returns a borrowed slice.

Callers can inspect the sequence, but they cannot clear it, push to it, or
replace the internal vector.

That preserves the invariant after construction.

### ML Concept

`TokenSequence` is tokenized text before next-token examples are created.

A sequence of length `n` can produce `n - 1` adjacent prediction pairs.

### Category Theory Concept

`TokenSequence` behaves like:

```text
List+ TokenId
```

where `List+` means a non-empty finite list.

Its constructor is not:

```text
List TokenId -> TokenSequence
```

because the empty list is invalid.

It is:

```text
List TokenId -> Result TokenSequence CtError
```

Rust turns the partial construction into a total function by using `Result`.

## `Vector` and `Logits`

The problem these blocks solve is:

> A dense hidden vector and raw vocabulary scores are both `Vec<f32>`, but they
> do not mean the same thing.

The blocks:

```rust,ignore
#[derive(Debug, Clone, PartialEq)]
pub struct Vector(Vec<f32>);

impl Vector {
    pub fn new(values: Vec<f32>) -> Self {
        Self(values)
    }

    pub fn as_slice(&self) -> &[f32] {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Logits(Vec<f32>);

impl Logits {
    pub fn new(values: Vec<f32>) -> Self {
        Self(values)
    }

    pub fn as_slice(&self) -> &[f32] {
        &self.0
    }
}
```

### Rust Syntax

`Vector` means hidden features.

`Logits` means unnormalized scores.

Both wrap `Vec<f32>`.

The distinction matters because only this arrow should produce logits:

```text
Vector -> Logits
```

and only this arrow should normalize logits:

```text
Logits -> Distribution
```

If both were plain `Vec<f32>`, the compiler could not help keep those stages
separate.

These types derive `PartialEq`, but not `Eq`, because they contain `f32`.

Floating-point values do not have total equality because `NaN` is not equal to
itself.

### ML Concept

A `Vector` is the dense representation used after embedding lookup.

Example:

```text
TokenId(3) -> [0.12, -0.44, 0.88, 0.03]
```

`Logits` are raw vocabulary scores.

Example:

```text
[3.0, 1.0, -2.0]
```

They can be negative, larger than one, and do not need to sum to one.

The pipeline is:

```text
TokenId -> Vector -> Logits -> Distribution
```

### Category Theory Concept

If the model dimension is `d`, a vector lives in a vector-space-like object:

```text
R^d
```

If the vocabulary size is `V`, logits live in:

```text
R^V
```

The output projection is an arrow:

```text
R^d -> R^V
```

and softmax maps:

```text
R^V -> probability distributions over TokenId
```

## `Distribution`

The problem this block solves is:

> Probabilities are not just floats. A probability distribution must be
> non-empty, finite, non-negative, and sum to one.

The core block:

```rust,ignore
#[derive(Debug, Clone, PartialEq)]
pub struct Distribution(Vec<f32>);

impl Distribution {
    pub fn new(probabilities: Vec<f32>) -> CtResult<Self> {
        if probabilities.is_empty() {
            return Err(CtError::EmptyInput("distribution"));
        }

        let sum: f32 = probabilities.iter().sum();
        let all_valid = probabilities
            .iter()
            .all(|probability| probability.is_finite() && *probability >= 0.0);

        if !all_valid || !approx_eq(sum, 1.0, 1e-4) {
            return Err(CtError::InvalidProbability("distribution constructor"));
        }

        Ok(Self(probabilities))
    }
}
```

### Rust Syntax: Why Construction Can Fail

This is invalid:

```text
[]
```

This is invalid:

```text
[0.4, 0.4]
```

because it sums to `0.8`, not `1.0`.

This is invalid:

```text
[1.2, -0.2]
```

because probabilities cannot be negative.

So `Distribution::new` returns `CtResult<Self>`.

### Rust Syntax: The Sum Check

```rust,ignore
let sum: f32 = probabilities.iter().sum();
```

This computes the total probability mass.

The code uses approximate equality:

```rust,ignore
approx_eq(sum, 1.0, 1e-4)
```

because floating-point arithmetic is not exact.

### ML Concept

This is the output of softmax:

```text
Logits -> Distribution
```

The rest of the model can treat a `Distribution` as real probabilities because
the constructor checked the rule.

### Category Theory Concept

`Distribution` is an object with a stronger invariant than `Vec<f32>`.

The softmax morphism lands in this object only if it can produce valid
probability mass.

## `Loss`

The problem this block solves is:

> A loss value must be a real, non-negative scalar.

The block:

```rust,ignore
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Loss(f32);

impl Loss {
    pub fn new(value: f32) -> CtResult<Self> {
        if !value.is_finite() || value < 0.0 {
            return Err(CtError::InvalidLoss(value));
        }

        Ok(Self(value))
    }

    pub fn value(&self) -> f32 {
        self.0
    }
}
```

### Rust Syntax

`Loss::new` rejects:

- infinity
- not-a-number values
- negative values

Cross entropy should not produce a negative loss. If it does, something has
gone wrong before or during loss calculation.

`Loss` derives `Copy` because it wraps one small scalar.

Calling `value()` returns the raw `f32` for printing, comparison, or averaging.

### ML Concept

Loss is the training signal.

For next-token prediction:

```text
loss = -log(probability assigned to the correct token)
```

Lower loss means the model assigned more probability to the correct answer.

Training tries to reduce this value.

### Category Theory Concept

Loss is the codomain of an evaluation morphism:

```text
Distribution x TokenId -> Loss
```

It maps prediction plus truth into a non-negative scalar objective.

## Shape and Training Hyperparameter Types

The problem these blocks solve is:

> Dimensions and learning rates need boundary checks before they are used to
> allocate matrices or update parameters.

The types are:

```text
VocabSize
ModelDimension
LearningRate
```

### Rust Syntax

`VocabSize::new(0)` fails because a vocabulary with zero entries is unusable.

`ModelDimension::new(0)` fails because an embedding vector with zero width
cannot carry features.

`LearningRate::new(value)` fails when the value is not finite or is not
positive.

These checks keep bad configuration from becoming strange matrix behavior
later.

### Worked Example: Configuration Values Are Not Interchangeable

The raw representation for all three values is small:

```text
VocabSize       -> usize
ModelDimension  -> usize
LearningRate    -> f32
```

That can make them look like ordinary numbers. They are not ordinary once they
cross the model boundary.

`Parameters::init` in `src/domain.rs` makes the distinction concrete:

```rust,ignore
let parameters = Parameters::init(
    VocabSize::new(5)?,
    ModelDimension::new(2)?,
);
```

The first argument chooses how many vocabulary rows and output scores exist.
The second argument chooses how wide each hidden vector is. Swapping those
meanings would create a different model shape, even though both values are
stored as `usize` underneath.

The same rule applies to `LearningRate`. It is not a loss value, probability,
or model dimension. It controls how far one update moves the parameters:

```text
new parameter = old parameter - learning_rate * gradient
```

If the learning rate were zero, negative, infinite, or not-a-number, the update
would stop being the small controlled movement the training chapter needs.
That is why construction fails early.

ML reading:

```text
VocabSize      -> how many token classes the model can score
ModelDimension -> how much hidden capacity each token receives
LearningRate   -> how large each optimizer step is
```

Category-theory reading:

`VocabSize` helps choose the finite token object, `ModelDimension` helps choose
the intermediate representation object, and `LearningRate` selects one update
from a family of possible training endomorphisms. The values are configuration
for different parts of the typed system, not interchangeable numbers.

Checkpoint question:

```text
If you see the raw value 5, what extra information tells you whether it is a
vocabulary size, model dimension, token id, or step count?
```

### ML Concept

`VocabSize` controls:

```text
embedding rows
logit length
distribution length
bias length
```

`ModelDimension` controls embedding width:

```text
R^d
```

`LearningRate` controls optimizer step size:

```text
parameter = parameter - learning_rate * gradient
```

### Category Theory Concept

`VocabSize` describes the cardinality of the finite token object.

`ModelDimension` chooses the intermediate vector-space-like object.

`LearningRate` chooses one update morphism from a family of training
endomorphisms.

## `Product<A, B>`

The problem this block solves is:

> Some ML operations need two inputs that belong together.

The block:

```rust,ignore
#[derive(Debug, Clone, PartialEq)]
pub struct Product<A, B> {
    first: A,
    second: B,
}
```

This is a generic pair.

It is used in two important places:

```rust,ignore
pub type TrainingExample = Product<TokenId, TokenId>;
```

and:

```text
Product<Distribution, TokenId> -> Loss
```

### Rust Syntax: Why Not A Tuple Everywhere?

Rust tuples like `(A, B)` would work mechanically.

`Product<A, B>` makes the category-theory idea visible:

```text
A x B
```

It also gives named methods:

```rust,ignore
first()
second()
into_parts()
```

Those methods make call sites easier to read during the course.

### ML Concept

`Product<TokenId, TokenId>` is one supervised next-token example:

```text
input token x target token
```

`Product<Distribution, TokenId>` is the input to cross entropy:

```text
prediction x target
```

### Category Theory Concept

`Product<A, B>` is the course's named version of:

```text
A x B
```

The accessors are projection-like operations:

```text
first  ~ pi_1
second ~ pi_2
```

## `TrainingSet`

The problem this block solves is:

> Training should not run on an empty collection of examples.

The block:

```rust,ignore
#[derive(Debug, Clone, PartialEq)]
pub struct TrainingSet(Vec<TrainingExample>);

impl TrainingSet {
    pub fn new(examples: impl IntoIterator<Item = TrainingExample>) -> CtResult<Self> {
        let examples = examples.into_iter().collect::<Vec<_>>();

        if examples.is_empty() {
            return Err(CtError::EmptyInput("training set"));
        }

        Ok(Self(examples))
    }
}
```

This mirrors `TokenSequence`.

The internal vector is private.

Construction validates non-emptiness.

Callers get read-only access through:

```rust,ignore
pub fn examples(&self) -> &[TrainingExample]
```

### Rust Syntax: Why `is_empty` Exists If Empty Is Impossible

`TrainingSet` includes:

```rust,ignore
pub fn is_empty(&self) -> bool {
    self.0.is_empty()
}
```

For values constructed through `TrainingSet::new`, this should always be
false.

The method exists because collection-like types conventionally expose both
`len` and `is_empty`, and tests or generic code may use it.

The invariant is still protected by private storage and the constructor.

### ML Concept

A `TrainingSet` is a non-empty list of next-token examples.

For:

```text
[10, 25, 31, 7]
```

the training set is:

```text
(10, 25)
(25, 31)
(31, 7)
```

### Category Theory Concept

The shape is:

```text
non-empty list of (TokenId x TokenId)
```

or:

```text
List+ (TokenId x TokenId)
```

## `Parameters`

The problem this block solves is:

> Training needs one object that owns all trainable model state.

The block:

```rust,ignore
#[derive(Debug, Clone, PartialEq)]
pub struct Parameters {
    pub(crate) embedding: Vec<Vec<f32>>,
    pub(crate) lm_head: Vec<Vec<f32>>,
    pub(crate) bias: Vec<f32>,
}
```

The model has three pieces:

```text
embedding table
lm head matrix
bias vector
```

The fields are `pub(crate)`, not fully public.

That means code inside this crate can update parameters during training, but
external callers use accessors.

### Rust Syntax: Initialization

```rust,ignore
pub fn init(vocab_size: VocabSize, d_model: ModelDimension) -> Self
```

This takes validated domain values, not raw `usize`.

That means matrix allocation starts from:

```text
non-empty vocabulary
positive model dimension
```

The initialized shapes are:

```text
embedding: vocab_size x d_model
lm_head:   d_model x vocab_size
bias:      vocab_size
```

### ML Concept

`Parameters` is the trainable state.

Prediction reads it.

Training maps it back to a new `Parameters` value:

```text
Parameters -> Parameters
```

### Category Theory Concept

`Parameters` is the object of the training endomorphism.

The important point is not that the numbers change.

The important point is that the type remains the same.

## Utility Functions

The file ends with:

```rust,ignore
pub(crate) fn init_matrix(...)
pub(crate) fn approx_eq(...)
```

`init_matrix` is local deterministic initialization for the teaching model.

`approx_eq` is a small floating-point helper used by probability checks and
composition tests.

Both are crate-internal implementation details, not learner-facing domain
objects.

## Runnable Example

The domain example shows token IDs becoming training pairs:

<details>
<summary>Source snapshot: examples/01_domain_objects.rs</summary>

```rust,ignore
{{#include ../../examples/01_domain_objects.rs}}
```

</details>

Run:

```bash
cargo run --example 01_domain_objects
```

Expected shape:

```text
TokenSequence:
[TokenId(1), TokenId(2), TokenId(3), TokenId(4)]

TrainingSet:
(TokenId(1) -> TokenId(2))
(TokenId(2) -> TokenId(3))
(TokenId(3) -> TokenId(4))

Typed boundaries:
usize -> TokenId
Vec<TokenId> -> TokenSequence
TokenSequence -> TrainingSet
TrainingExample = Product<TokenId, TokenId>
```

## Example Output Transfer Checklist

Use the example output to test whether the chapter's boundary idea is working.

| Example output | Rust reading | ML reading | Category-theory reading | Shortcut to reject |
| --- | --- | --- | --- | --- |
| `TokenSequence` | a private `Vec<TokenId>` has passed the non-empty constructor | tokenized data is ready for adjacent-pair creation | non-empty list-like object | treating any `Vec<TokenId>` as valid sequence data |
| `TrainingSet` | `DatasetWindowing` returned validated examples | adjacent input-target pairs are ready for training | list of product objects | training directly on a raw token list |
| `usize -> TokenId` | a raw index receives a domain name | a number becomes a vocabulary position | raw representation enters a typed object | passing row counts, dimensions, and token IDs as the same `usize` |
| `Vec<TokenId> -> TokenSequence` | a collection crosses a constructor boundary | tokenized text becomes an ordered sequence stage | partial construction into `Result<TokenSequence, CtError>` | allowing an empty sequence downstream |
| `TrainingExample = Product<TokenId, TokenId>` | a pair has a named product shape | input token and target token travel together | `TokenId x TokenId` | using an unlabelled tuple and forgetting which side is the target |

This is why the example prints `TokenId(...)` instead of only `1 -> 2`.
Display can use raw numbers at the edge, but the teaching output should remind
you that the program is moving through named objects.

## Why This Matters

The main design rule is:

> Use raw primitives only at the edge where they are created or displayed.

After that, use domain types.

This prevents mistakes like:

```text
passing a model dimension where a token ID was expected
passing logits where probabilities were expected
training on an empty dataset
using a negative learning rate
```

Types do not prove that a model is good, that optimization will always
converge, or that the tiny implementation is production-ready. They do
something narrower and very useful: they make the wrong wiring harder to write.
That is the first step toward a pipeline that can be explained, tested, and
extended.

## Core Mental Model

`src/domain.rs` turns raw storage into trustworthy objects.

In Rust terms:

```text
private fields + smart constructors + accessors
```

In ML terms:

```text
tokens, vectors, probabilities, loss, and model weights
```

In category-theory terms:

```text
objects that morphisms can safely connect
```

## Checkpoint

Pick one type from this file and explain:

1. What raw representation it wraps.
2. What invalid state it prevents.
3. Which morphism consumes or produces it.

Example:

```text
Distribution wraps Vec<f32>, rejects invalid probability mass, and is produced
by Softmax before CrossEntropy consumes it.
```

## Where This Leaves Us

This chapter gave names to the values in the system. A token id is not a model
dimension, logits are not probabilities, and a training set is not just any
vector of pairs. Each type marks a stage where raw storage becomes a meaningful
object.

The next chapter, [Morphism and Composition](02-morphisms-composition.md), adds
arrows between those objects. Once the arrows exist, the book can talk about
identity, composition, and repeated transformations without falling back to
loose wiring conventions.

## Further Reading

Do not read these sources as generic Rust advice. Read them as a way to answer
one question:

```text
what is the pipeline allowed to trust after this value is constructed?
```

Start from the local Rust evidence:

```text
TokenId::new(index)              -> TokenId
TokenSequence::new(tokens)       -> Result<TokenSequence, CtError>
Distribution::new(probabilities) -> Result<Distribution, CtError>
LearningRate::new(value)         -> Result<LearningRate, CtError>
Parameters::init(vocab, dim)     -> Parameters
```

Then read the sources in this order:

| Source | What to transfer back into this chapter | Local evidence to inspect |
| --- | --- | --- |
| [Rust Book: Structs](https://doc.rust-lang.org/book/ch05-01-defining-structs.html) | A named struct or tuple struct can make two identical raw representations mean different things. | `TokenId(usize)`, `VocabSize(usize)`, `ModelDimension(usize)` |
| [Rust By Example: New Type Idiom](https://doc.rust-lang.org/rust-by-example/generics/new_types.html) | A small wrapper can make the compiler reject the wrong semantic role before runtime logic runs. | `TokenId`, `VocabSize`, `ModelDimension` |
| [Rust Book: Result](https://doc.rust-lang.org/stable/book/ch09-02-recoverable-errors-with-result.html) | A constructor can return either a trusted value or a typed error. | `TokenSequence::new`, `Distribution::new`, `Loss::new`, `LearningRate::new` |
| [Rust API Guidelines: Type Safety](https://rust-lang.github.io/api-guidelines/checklist.html#type-safety-c-newtype) | Newtypes provide static distinctions when raw arguments would hide meaning. | `Product<Distribution, TokenId>`, `LearningRate`, `ModelDimension` |
| [Rust API Guidelines: Dependability](https://rust-lang.github.io/api-guidelines/checklist.html#dependability-c-validate) | Invalid arguments should be rejected at the boundary that owns the invariant. | `distribution_rejects_non_normalized_values`, `token_sequence_rejects_empty_input` |
| [Rust API Guidelines: Future Proofing](https://rust-lang.github.io/api-guidelines/checklist.html#future-proofing-c-sealed) | Private fields and small accessors keep later code from bypassing the boundary. | `as_slice`, `value`, `index`, `Parameters` accessors |

After reading one external source, ask four questions:

1. Which domain type did it clarify?
2. Does that type only separate meaning, or does it also validate an invariant?
3. Which downstream morphism is allowed to trust the value?
4. Which command would you run to see the evidence?

For this chapter, the commands are:

```bash
cargo run --example 01_domain_objects
cargo test domain::tests --lib
```

For terminology recovery, use the [Glossary](glossary.md) entries for object,
product object, invariant, and smart constructor. For source depth, use
[References](references.md) and follow the Rust struct, error-handling, and API
design entries.

If a source does not help you explain why `Distribution::new` rejects invalid
probability mass before `CrossEntropy` sees it, it has not transferred back
into the chapter yet.

## Practice After This Chapter

Use [Exercise 1](exercises.md#exercise-1-explain-one-domain-type) to explain one
domain type and [Exercise 7](exercises.md#exercise-7-explain-one-validation-boundary)
to explain one constructor boundary. Together they test the chapter's main
distinction: some types separate meaning, while others also reject invalid
states.

## Retrieval Practice

### Recall

What is a domain object in this book?

### Explain

Why does `Distribution::new` validate probability mass at construction time
instead of leaving that check to `CrossEntropy`?

### Apply

Design a one-field newtype for a future `SequenceLength`. State one invariant
its constructor should protect.
