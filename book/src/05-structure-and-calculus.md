# Functors, Naturality, Monoids, and Chain Rule

The problem this chapter solves is:

> After seeing individual ML arrows, you need names for common structures that
> appear across many systems: mapping inside containers, converting containers
> consistently, combining traces, and composing local derivatives.

The previous chapters were mostly about one pipeline. This chapter zooms out
from that pipeline and asks which shapes keep appearing even when the concrete
data changes. Once you can see those repeated shapes, the category-theory
vocabulary stops feeling like a separate subject. It becomes a set of names for
ordinary engineering moves.

This chapter covers four patterns:

```text
Functor
NaturalTransformation
Monoid
Chain rule
```

They are not separate from the ML pipeline.

They explain patterns you already saw:

- mapping over many examples
- converting one wrapper shape to another
- combining pipeline traces
- composing gradients through layers

> Reader orientation:
> This chapter is more abstract than the previous ones. Read each section in
> this order: first the Rust mechanism, then the ML use, then the
> category-theory name. The names are not decoration; they are compression for
> patterns that appear repeatedly in real model code.

## Source Snapshots

`src/structure.rs` covers functors, natural transformations, and monoids.

<details>
<summary>Source snapshot: src/structure.rs</summary>

```rust,ignore
{{#include ../../src/structure.rs}}
```

</details>

`src/calculus.rs` keeps the chain-rule example deliberately small.

<details>
<summary>Source snapshot: src/calculus.rs</summary>

```rust,ignore
{{#include ../../src/calculus.rs}}
```

</details>

## The Whole Structure File

`src/structure.rs` defines:

```text
Functor<A, B>
VecFunctor
OptionFunctor
NaturalTransformation<A>
VecToFirstOption
Monoid
TraceStep
PipelineTrace
```

Each block gives a Rust handle to one abstract pattern.

Before reading the traits, start with the plain Rust operation that motivates
them. Mapping over a vector means taking each item out, applying a function, and
collecting the new values into another vector:

```rust
let values = vec![1, 2, 3];
let doubled: Vec<i32> = values.into_iter().map(|x| x * 2).collect();

assert_eq!(doubled, vec![2, 4, 6]);
```

There is no category theory hidden in that snippet. It is just ordinary Rust.
The category-theory word `functor` appears when we notice the reusable shape:
the code changes the contents while preserving the surrounding container.

## `Functor<A, B>`

The problem this block solves is:

> The code needs a name for "apply a function inside a wrapper while keeping the
> wrapper shape."

First principle: a trait is a contract. It says, "any type that implements this
trait must provide these associated types and this method." Here the contract is
small on purpose. It does not try to model every possible functor in
mathematics; it gives this tutorial one precise place to talk about `fmap`.

The block:

```rust,ignore
/// A minimal functor interface for this tutorial.
pub trait Functor<A, B> {
    type WrappedA;
    type WrappedB;

    fn fmap<F>(wrapped: Self::WrappedA, f: F) -> Self::WrappedB
    where
        F: Fn(A) -> B;
}
```

## Rust Syntax

`Functor<A, B>` is a trait. A trait is Rust's way to name behavior that many
types can implement.

Here, the behavior is:

```text
map a function through some wrapper shape
```

`A` and `B` are generic type parameters:

```text
A = input item type
B = output item type
```

Generic means the trait is not tied to one concrete type like `i32` or
`String`. The same trait can describe `Vec<i32> -> Vec<String>`,
`Option<TokenId> -> Option<Embedding>`, or any other pair of item types.

It has associated types:

```rust,ignore
type WrappedA;
type WrappedB;
```

Associated types are type names chosen by each implementation of the trait.
They let the trait say:

```text
every implementer must tell us what wrapped input and wrapped output mean
```

For `VecFunctor`, those associated types become `Vec<A>` and `Vec<B>`.

For `OptionFunctor`, they become `Option<A>` and `Option<B>`.

The method:

```rust,ignore
fn fmap<F>(wrapped: Self::WrappedA, f: F) -> Self::WrappedB
where
    F: Fn(A) -> B;
```

means:

> Given a wrapped `A` and a function `A -> B`, produce a wrapped `B`.

The `where` clause is a readable place to put a bound. The bound:

```rust,ignore
F: Fn(A) -> B
```

means `F` must be callable like a function that consumes an `A` and returns a
`B`.

Here is the real crate API in the smallest useful form:

```rust,ignore
use category_theory_transformer_rs::{Functor, VecFunctor};

let lengths = VecFunctor::fmap(vec!["cat", "rust"], |word| word.len());

assert_eq!(lengths, vec![3, 4]);
```

> What to notice:
> The call names the structure once: `VecFunctor::fmap`. The closure only
> describes the item-level operation: `&str -> usize`. The vector shape is
> handled by the functor implementation.

## ML Concept

In ML, you often apply the same transformation across a structure:

```text
map preprocessing over a batch
map token conversion over a sequence
map loss computation over examples
```

The wrapper might be:

```text
Vec
Option
Result
batch tensor
```

The idea is:

```text
transform the contents, preserve the surrounding structure
```

## Category-Theory Concept

A functor maps:

```text
objects -> objects
morphisms -> morphisms
```

while preserving identity and composition.

This tutorial's trait is deliberately small. It focuses on the practical
`fmap` operation.

## `VecFunctor`

The problem this block solves is:

> Demonstrate `fmap` for lists.

The block:

```rust,ignore
pub struct VecFunctor;

impl<A, B> Functor<A, B> for VecFunctor {
    type WrappedA = Vec<A>;
    type WrappedB = Vec<B>;

    fn fmap<F>(wrapped: Vec<A>, f: F) -> Vec<B>
    where
        F: Fn(A) -> B,
    {
        wrapped.into_iter().map(f).collect()
    }
}
```

## Rust Syntax

`VecFunctor` is a unit struct. It stores no state.

The `impl` block is a trait implementation:

```rust,ignore
impl<A, B> Functor<A, B> for VecFunctor
```

Read it as:

```text
for every A and B, VecFunctor knows how to behave as Functor<A, B>
```

The implementation chooses the associated types:

```text
WrappedA = Vec<A>
WrappedB = Vec<B>
```

The method consumes the vector:

```rust,ignore
wrapped.into_iter()
```

maps the function over every item:

```rust,ignore
.map(f)
```

and collects the result:

```rust,ignore
.collect()
```

The runnable companion example uses the same real crate API:

```rust,ignore
use category_theory_transformer_rs::{Functor, VecFunctor};

let token_ids = vec![1, 2, 3];
let shifted = VecFunctor::fmap(token_ids, |id| id + 100);

assert_eq!(shifted, vec![101, 102, 103]);
```

## ML Concept

If you have a batch of token IDs:

```text
[TokenId(1), TokenId(2), TokenId(3)]
```

and a function:

```text
TokenId -> Vector
```

mapping over the batch gives:

```text
[Vector, Vector, Vector]
```

That is the same shape as `VecFunctor`.

## Category-Theory Concept

`Vec` is list-like structure.

Mapping preserves the list shape:

```text
List A -> List B
```

The length and order remain structurally meaningful.

## `OptionFunctor`

The problem this block solves is:

> Demonstrate the same functor idea for optional values.

The block:

```rust,ignore
pub struct OptionFunctor;

impl<A, B> Functor<A, B> for OptionFunctor {
    type WrappedA = Option<A>;
    type WrappedB = Option<B>;

    fn fmap<F>(wrapped: Option<A>, f: F) -> Option<B>
    where
        F: Fn(A) -> B,
    {
        wrapped.map(f)
    }
}
```

## Rust Syntax

The wrapper types are:

```text
Option<A>
Option<B>
```

The implementation delegates to Rust's built-in:

```rust,ignore
wrapped.map(f)
```

If the value is `Some(a)`, it becomes `Some(f(a))`.

If the value is `None`, it stays `None`.

The important beginner point is that `Option` makes absence explicit in the
type. You cannot accidentally treat a missing value as a real value without
handling the `None` case.

```rust,ignore
use category_theory_transformer_rs::{Functor, OptionFunctor};

let present = OptionFunctor::fmap(Some(7), |value| value * 2);
let missing = OptionFunctor::fmap(None::<i32>, |value| value * 2);

assert_eq!(present, Some(14));
assert_eq!(missing, None);
```

## ML Concept

Optional values appear when data may be absent:

```text
maybe first token
maybe cached embedding
maybe resolved department
```

Mapping over `Option` lets you transform present values without inventing a
fake value for missing ones.

## Category-Theory Concept

`Option` is a context representing possible absence.

`fmap` lifts:

```text
A -> B
```

to:

```text
Option<A> -> Option<B>
```

## Conceptual Extension: `Distribution<T>::map`

The problem this block solves is:

> A probabilistic value may contain many possible outcomes. Sometimes you want
> to transform every possible outcome while keeping its probability attached.

The current crate's `Distribution` in `src/domain.rs` is a concrete validated
probability vector:

```rust,ignore
pub struct Distribution(Vec<f32>);
```

The block below is a conceptual generic version that explains the functor idea
for probabilistic outcomes:

```rust
pub struct Probability(f32);

pub struct Distribution<T> {
    outcomes: Vec<(T, Probability)>,
}

impl<T> Distribution<T> {
    pub fn map<U>(
        self,
        f: impl Fn(T) -> U,
    ) -> Distribution<U> {
        let outcomes = self
            .outcomes
            .into_iter()
            .map(|(value, probability)| {
                (f(value), probability)
            })
            .collect();

        Distribution { outcomes }
    }
}
```

The core idea is:

```text
map changes the values inside the distribution,
but keeps the probabilities attached to them.
```

## Rust Syntax

Start with the generic struct:

```rust
pub struct Probability(f32);

pub struct Distribution<T> {
    outcomes: Vec<(T, Probability)>,
}
```

This means:

```text
Distribution<T> = many possible T values, each paired with a probability
```

If `T` is `TokenId`, then the type is:

```rust,ignore
Distribution<TokenId>
```

If `T` is `String`, then the type is:

```rust,ignore
Distribution<String>
```

The method introduces a second generic type:

```rust,ignore
pub fn map<U>(...)
```

`T` is the old outcome type.

`U` is the new outcome type.

So the method has this shape:

```text
Distribution<T> -> Distribution<U>
```

The first parameter is:

```rust,ignore
self
```

That means the method consumes the old distribution.

After calling:

```rust,ignore
let text_dist = token_dist.map(decode);
```

the old `token_dist` has been moved and cannot be used again.

That is why the implementation can call:

```rust,ignore
self.outcomes.into_iter()
```

`into_iter()` consumes the vector and yields owned pairs:

```text
(T, Probability)
```

The function parameter is:

```rust,ignore
f: impl Fn(T) -> U
```

This means:

```text
give this method a function or closure that takes T and returns U
```

For example, a decoder might have this shape:

```text
TokenId -> String
```

Then:

```text
Distribution<TokenId> -> Distribution<String>
```

The inner mapping line is:

```rust,ignore
.map(|(value, probability)| {
    (f(value), probability)
})
```

For every pair:

```text
(value, probability)
```

the code applies `f` to the value and leaves the probability unchanged.

So:

```text
(TokenId(2), Probability(0.70))
```

can become:

```text
("Rust", Probability(0.70))
```

Finally:

```rust,ignore
.collect()
```

collects the transformed pairs back into a vector, and:

```rust,ignore
Distribution { outcomes }
```

wraps them in the new distribution.

## ML Concept

Imagine a model returns possible next tokens:

```text
TokenId(2) -> 0.70
TokenId(4) -> 0.20
TokenId(3) -> 0.10
```

Those token IDs are useful to the model, but a learner or UI might need text:

```text
TokenId(2) -> "Rust"
TokenId(4) -> "Python"
TokenId(3) -> "."
```

`map` changes the representation:

```text
Distribution<TokenId> -> Distribution<String>
```

The values change:

```text
TokenId(2) becomes "Rust"
TokenId(4) becomes "Python"
TokenId(3) becomes "."
```

The probabilities do not change:

```text
0.70 stays 0.70
0.20 stays 0.20
0.10 stays 0.10
```

So `map` is for changing the meaning or representation of each possible
outcome, not for changing the probability mass.

## Category Theory Concept

This is the functor pattern for probabilistic context.

Given a normal deterministic function:

```text
f : T -> U
```

`map` lifts it into the distribution context:

```text
Distribution<T> -> Distribution<U>
```

In functional-programming notation:

```text
fmap : (T -> U) -> Distribution<T> -> Distribution<U>
```

The outer structure is preserved:

```text
same number of possible outcomes
same probabilities
same probabilistic context
```

Only the inner values are transformed.

That is the same pattern as:

```text
Option<T> -> Option<U>
Vec<T> -> Vec<U>
Distribution<T> -> Distribution<U>
```

Different context, same functor idea.

## Concrete Example

Here is a complete conceptual example:

```rust
#[derive(Debug, Clone, Copy)]
pub struct TokenId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Probability(pub f32);

#[derive(Debug, Clone)]
pub struct Distribution<T> {
    outcomes: Vec<(T, Probability)>,
}

impl<T> Distribution<T> {
    pub fn new(outcomes: Vec<(T, Probability)>) -> Self {
        Self { outcomes }
    }

    pub fn map<U, F>(self, mut f: F) -> Distribution<U>
    where
        F: FnMut(T) -> U,
    {
        let outcomes = self
            .outcomes
            .into_iter()
            .map(|(value, probability)| {
                (f(value), probability)
            })
            .collect();

        Distribution { outcomes }
    }
}

let vocab = ["I", "love", "Rust", "."];

let token_dist = Distribution::new(vec![
    (TokenId(2), Probability(0.70)),
    (TokenId(3), Probability(0.30)),
]);

let text_dist = token_dist.map(|token| {
    vocab[token.0].to_string()
});

assert_eq!(
    text_dist.outcomes,
    vec![
        ("Rust".to_string(), Probability(0.70)),
        (".".to_string(), Probability(0.30)),
    ],
);
```

Conceptually, the result is:

```text
"Rust" -> 0.70
"."    -> 0.30
```

## Why `Fn(T) -> U`

The signature:

```rust,ignore
f: impl Fn(T) -> U
```

accepts functions and closures.

It is more flexible than:

```rust,ignore
fn(T) -> U
```

because closures can capture values from the surrounding scope:

```rust,ignore
let vocab = ["I", "love", "Rust", "."];

let text_dist = token_dist.map(|token| {
    vocab[token.0].to_string()
});
```

The closure uses `vocab` from outside the closure body.

## Why A Library Might Use `FnMut`

The pedagogical signature:

```rust,ignore
f: impl Fn(T) -> U
```

is easy to read.

A more flexible library signature is often:

```rust,ignore
pub fn map<U, F>(self, mut f: F) -> Distribution<U>
where
    F: FnMut(T) -> U,
```

`FnMut` allows the closure to mutate captured state.

For example:

```rust,ignore
let mut counter = 0;

let numbered = token_dist.map(|token| {
    counter += 1;
    (counter, token)
});
```

The key ownership rule is unchanged:

```text
the method consumes the old distribution and moves each T into f
```

## `map` Versus `flat_map`

Use `map` when each possible value becomes one transformed value:

```text
T -> U
```

So:

```text
Distribution<T> -> Distribution<U>
```

Example:

```text
TokenId -> String
```

Use `flat_map` when each possible value produces another distribution:

```text
T -> Distribution<U>
```

Without flattening, the result would be:

```text
Distribution<Distribution<U>>
```

The simple distinction is:

```text
map:
  one possible value becomes one transformed value

flat_map:
  one possible value becomes many possible future values
```

In language modeling:

```text
map decodes possible tokens into text
flat_map chains uncertainty across another prediction step
```

## Algebra Version

If:

```text
D<T> = [(t1, p1), (t2, p2), ..., (tn, pn)]
```

and:

```text
f : T -> U
```

then:

```text
map(f, D<T>) = [(f(t1), p1), (f(t2), p2), ..., (f(tn), pn)]
```

The probabilities are untouched.

Only the values move through `f`.

## Core Mental Model

In Rust terms:

```text
consume self, move each T into f, preserve each Probability, collect into
Distribution<U>
```

In ML terms:

```text
decode or transform every possible outcome without changing model confidence
```

In category-theory terms:

```text
lift a deterministic morphism T -> U into the probabilistic context
Distribution<T> -> Distribution<U>
```

## `NaturalTransformation<A>`

The problem this block solves is:

> Sometimes you need to convert one wrapper shape into another without caring
> about the specific item type.

The beginner trap is to think a natural transformation is just any conversion.
It is more disciplined than that. The conversion must be compatible with
mapping. If you transform the wrapper first and then map the item, you should get
the same result as mapping first and then transforming the wrapper.

The block:

```rust,ignore
/// A structure-preserving conversion between wrappers.
pub trait NaturalTransformation<A> {
    type From;
    type To;

    fn transform(from: Self::From) -> Self::To;
}
```

## Rust Syntax

The associated types are:

```text
From
To
```

The method:

```rust,ignore
fn transform(from: Self::From) -> Self::To;
```

converts the wrapper.

The type parameter `A` represents the item type inside the wrapper.

This trait has the same shape as `Functor`: the abstract contract is public,
but each implementation chooses the concrete wrapper types through associated
types.

That is why the implementation can be generic over `A` without knowing whether
`A` is a token, a sentence, a loss value, or a trace step.

## ML Concept

Data pipelines often convert shapes:

```text
list of candidates -> optional selected candidate
batch -> first example
many diagnostics -> maybe first failure
```

The conversion should not depend on whether the item is a token, vector, or
loss.

## Category-Theory Concept

A natural transformation converts one functor into another in a way that is
uniform over the inner type.

The important word is uniform.

It should not inspect special details of `A`.

## `VecToFirstOption`

The problem this block solves is:

> Convert a list into an optional first item in a type-uniform way.

The block:

```rust,ignore
/// Natural transformation from `Vec<A>` to `Option<A>` by taking the first item.
pub struct VecToFirstOption;

impl<A> NaturalTransformation<A> for VecToFirstOption {
    type From = Vec<A>;
    type To = Option<A>;

    fn transform(from: Vec<A>) -> Option<A> {
        from.into_iter().next()
    }
}
```

## Rust Syntax

The implementation works for every `A`.

That is the role of:

```rust,ignore
impl<A> NaturalTransformation<A> for VecToFirstOption
```

The implementation does not ask for any bound on `A`. There is no `A: Clone`,
no `A: Debug`, and no `A: PartialEq`. It does not need those capabilities
because it never looks inside the item. It only changes the outer shape.

It consumes a vector:

```rust,ignore
from.into_iter()
```

then takes the first item:

```rust,ignore
.next()
```

If the vector is empty, the result is `None`.

If it has at least one item, the result is `Some(first_item)`.

```rust,ignore
use category_theory_transformer_rs::{NaturalTransformation, VecToFirstOption};

let first = VecToFirstOption::transform(vec!["embed", "linear", "softmax"]);
let empty = VecToFirstOption::transform(Vec::<&str>::new());

assert_eq!(first, Some("embed"));
assert_eq!(empty, None);
```

## ML Concept

This is like selecting the first candidate from a batch while preserving the
possibility that the batch was empty.

It does not care what the candidate type is.

## Category-Theory Concept

This is the example transformation:

```text
Vec<A> -> Option<A>
```

It is natural because it is uniform over `A`.

## Naturality Check

The problem this block solves is:

> Show that mapping first, then converting gives the same result as converting
> first, then mapping.

The function:

```rust,ignore
pub fn naturality_square_holds_for_first_option() -> bool {
    let xs = vec![1, 2, 3];
    let f = |x| x * 10;

    let path_top_then_right = VecToFirstOption::transform(VecFunctor::fmap(xs.clone(), f));
    let path_left_then_bottom = OptionFunctor::fmap(VecToFirstOption::transform(xs), f);

    path_top_then_right == path_left_then_bottom
}
```

## Rust Syntax

There are two paths.

Path one:

```text
Vec<i32> -> Vec<i32> -> Option<i32>
```

Path two:

```text
Vec<i32> -> Option<i32> -> Option<i32>
```

Both should produce the same value.

The crate exposes this as a small law check:

```rust,ignore
use category_theory_transformer_rs::naturality_square_holds_for_first_option;

assert!(naturality_square_holds_for_first_option());
```

## ML Concept

This is a consistency check for pipeline shape conversions.

It says:

```text
transform values, then select
```

matches:

```text
select, then transform the selected value
```

for this operation.

## Category-Theory Concept

This is the naturality square:

```text
Vec<A> ----fmap f----> Vec<B>
  |                     |
  | transform           | transform
  v                     v
Option<A> --fmap f--> Option<B>
```

The square commutes when both paths agree.

## `Monoid`

The problem this block solves is:

> Some values can be combined repeatedly, and there should be an empty value
> that changes nothing.

The first-principles version is string concatenation. The empty string changes
nothing, and grouping does not change the final text:

```rust
let empty = String::new();
let a = String::from("embed");
let b = String::from(" -> linear");
let c = String::from(" -> softmax");

assert_eq!(format!("{empty}{a}"), "embed");
assert_eq!(format!("{}{}", format!("{a}{b}"), c), format!("{a}{}", format!("{b}{c}")));
```

`PipelineTrace` uses the same idea, but with named pipeline steps instead of
raw text.

The trait:

```rust,ignore
pub trait Monoid: Sized {
    fn empty() -> Self;
    fn combine(&self, other: &Self) -> Self;
}
```

## Rust Syntax

`Sized` means values of this type have a known size at compile time. In this
trait, it keeps the return type `Self` straightforward:

```rust,ignore
fn empty() -> Self;
```

`Self` means "the type implementing this trait."

The trait requires:

```text
empty() -> Self
combine(&self, other: &Self) -> Self
```

So a monoid can produce an identity value and combine two values into one.

The `combine` method borrows both traces:

```rust,ignore
fn combine(&self, other: &Self) -> Self;
```

`&self` and `&Self` are references. They let the method read the two existing
values without taking ownership of them. The method returns a new combined
value.

## ML Concept

Common monoid-like values in ML systems include logs, traces, metrics, batches,
and accumulated gradients.

You often need to combine many small values into one larger value.

## Category-Theory Concept

A monoid has:

```text
identity element
associative binary operation
```

The laws are:

```text
empty combine a = a
a combine empty = a
(a combine b) combine c = a combine (b combine c)
```

## `TraceStep` and `PipelineTrace`

The problem this block solves is:

> Pipeline execution steps should be combinable into a larger trace.

The key types:

```rust,ignore
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TraceStep(&'static str);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineTrace(Vec<TraceStep>);
```

## Rust Syntax

`TraceStep` wraps one static string.

`PipelineTrace` wraps a vector of steps.

`PipelineTrace::from_steps` collects any iterable of steps.

`names()` returns the raw names for display:

```rust,ignore
self.0.iter().map(TraceStep::name).collect()
```

These wrappers matter because two values may both be strings but mean different
things. A `TraceStep` is not a model name, a token, or a user-facing sentence.
The type keeps that meaning attached to the value.

## ML Concept

A trace can record:

```text
embedding
linear
softmax
cross_entropy
```

This is useful for understanding which stages ran.

## Category-Theory Concept

A pipeline trace is a sequence-like monoid.

The empty trace is the identity.

Combining traces is concatenation.

## `PipelineTrace` as Monoid

The problem this block solves is:

> Make traces obey the monoid interface.

The implementation:

```rust,ignore
impl Monoid for PipelineTrace {
    fn empty() -> Self {
        PipelineTrace(vec![])
    }

    fn combine(&self, other: &Self) -> Self {
        let mut combined = self.0.clone();
        combined.extend_from_slice(&other.0);
        PipelineTrace(combined)
    }
}
```

## Rust Syntax

`empty` returns an empty vector.

`combine` clones the first trace, appends the second trace, and wraps the result
again as `PipelineTrace`.

The clone is intentional here: `combine` borrows both inputs, so it cannot move
steps out of either trace. Cloning the small `TraceStep` values lets the method
produce a fresh trace while leaving the inputs usable.

```rust,ignore
use category_theory_transformer_rs::{Monoid, PipelineTrace, TraceStep};

let encoder = PipelineTrace::from_steps([TraceStep::new("embedding")]);
let head = PipelineTrace::from_steps([TraceStep::new("softmax")]);
let trace = encoder.combine(&head);

assert_eq!(trace.names(), vec!["embedding", "softmax"]);
```

## ML Concept

This is how many execution logs work:

```text
trace_a + trace_b = longer trace
```

## Category-Theory Concept

This is the list monoid specialized to trace steps.

## Monoid Law Check

The problem this block solves is:

> Verify the identity and associativity laws for the trace type.

The function:

```rust,ignore
pub fn monoid_laws_hold_for_pipeline_trace() -> bool {
    let a = PipelineTrace::from_steps(vec![TraceStep::new("embedding")]);
    let b = PipelineTrace::from_steps(vec![TraceStep::new("linear")]);
    let c = PipelineTrace::from_steps(vec![TraceStep::new("softmax")]);
    let identity = PipelineTrace::empty();

    let left_identity = identity.combine(&a) == a;
    let right_identity = a.combine(&identity) == a;
    let associativity = a.combine(&b).combine(&c) == a.combine(&b.combine(&c));

    left_identity && right_identity && associativity
}
```

## Rust Syntax

The function constructs three traces and checks three booleans.

It returns true only if all monoid laws hold for those examples.

```rust,ignore
use category_theory_transformer_rs::monoid_laws_hold_for_pipeline_trace;

assert!(monoid_laws_hold_for_pipeline_trace());
```

## ML Concept

Grouping trace combination should not change the final trace.

This matters when systems combine logs from nested pipelines.

## Category-Theory Concept

This directly checks the monoid laws:

```text
identity
associativity
```

## The Calculus File

The problem `src/calculus.rs` solves is:

> Backpropagation is easier to understand if you first see one local derivative
> rule in isolation.

> What to notice:
> Backpropagation does not require every operation to know the whole model. Each
> operation only needs to know how its own output changes when its own inputs
> change. Composition does the rest.

The file defines:

```text
Scalar
LocalGradient
MulOp
```

## `Scalar`

The block:

```rust,ignore
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Scalar(f32);
```

## Rust Syntax

`Scalar` wraps one `f32`.

`Scalar::new` rejects non-finite values.

`value()` returns the raw float.

This repeats a pattern from the earlier domain-object chapter:

```text
private field
validating constructor
accessor
```

The raw `f32` is private, so callers cannot construct `Scalar(f32::NAN)`
directly. They must use `Scalar::new`, which returns a `Result`.

```rust,ignore
use category_theory_transformer_rs::Scalar;

let scalar = Scalar::new(2.5)?;

assert_eq!(scalar.value(), 2.5);

# Ok::<(), category_theory_transformer_rs::CtError>(())
```

## ML Concept

This is a single numeric value in a computation graph.

Examples:

```text
activation
loss component
weight
```

## Category-Theory Concept

It is the simple numeric object used in the local derivative example.

## `LocalGradient`

The block:

```rust,ignore
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LocalGradient(f32);
```

## Rust Syntax

This is another `f32` wrapper.

It has the same finite-value validation as `Scalar`.

The semantic difference is important:

```text
Scalar = forward value
LocalGradient = derivative signal
```

This is the same newtype move used throughout the crate. Both wrappers store
an `f32`, but the types prevent accidental mixing at function boundaries.

## ML Concept

A gradient tells how a loss changes when an intermediate value changes.

For example:

```text
dL/dz
```

## Category-Theory Concept

This is information flowing backward through a composed computation.

## `MulOp`

The problem this block solves is:

> Show a forward operation and its local backward rule.

The important methods:

```rust,ignore
pub fn forward(&self, x: Scalar, y: Scalar) -> CtResult<Scalar> {
    Scalar::new(x.value() * y.value())
}

pub fn backward(
    &self,
    x: Scalar,
    y: Scalar,
    upstream: LocalGradient,
) -> CtResult<(LocalGradient, LocalGradient)> {
    let dz_dx = y.value();
    let dz_dy = x.value();

    Ok((
        LocalGradient::new(upstream.value() * dz_dx)?,
        LocalGradient::new(upstream.value() * dz_dy)?,
    ))
}
```

## Rust Syntax

`forward` multiplies two scalars and validates the result.

`backward` takes:

```text
x
y
upstream gradient dL/dz
```

and returns:

```text
(dL/dx, dL/dy)
```

The return type is a Rust tuple.

The `?` operator appears twice in `backward`:

```rust,ignore
LocalGradient::new(upstream.value() * dz_dx)?
```

It means:

```text
if construction succeeded, keep the value
if construction failed, return the error from backward immediately
```

That keeps invalid numeric states at the boundary where they are created.

```rust,ignore
use category_theory_transformer_rs::{LocalGradient, MulOp, Scalar};

let mul = MulOp;
let x = Scalar::new(2.0)?;
let y = Scalar::new(3.0)?;
let z = mul.forward(x, y)?;
let (dl_dx, dl_dy) = mul.backward(x, y, LocalGradient::new(1.0)?)?;

assert_eq!(z.value(), 6.0);
assert_eq!(dl_dx.value(), 3.0);
assert_eq!(dl_dy.value(), 2.0);

# Ok::<(), category_theory_transformer_rs::CtError>(())
```

## ML Concept

For:

```text
z = x * y
```

the local derivatives are:

```text
dz/dx = y
dz/dy = x
```

By the chain rule:

```text
dL/dx = dL/dz * dz/dx = dL/dz * y
dL/dy = dL/dz * dz/dy = dL/dz * x
```

## Category-Theory Concept

The chain rule is composition of local derivative maps.

A big neural network is many small maps composed forward, then many local
gradient rules composed backward.

## Run The Example

<details>
<summary>Source snapshot: examples/04_structure_and_calculus.rs</summary>

```rust,ignore
{{#include ../../examples/04_structure_and_calculus.rs}}
```

</details>

Run:

```bash
cargo run --example 04_structure_and_calculus
```

You should see mapping over `Vec`, mapping over `Option`, a naturality check, a
combined trace, a monoid law check, and local gradients for multiplication.

## Core Mental Model

In Rust terms:

```text
traits name reusable operation shapes
unit structs demonstrate stateless operations
tests check laws
```

In ML terms:

```text
map over structures, combine traces, compose local gradients
```

In category-theory terms:

```text
functors preserve structure
natural transformations commute with mapping
monoids combine associatively with identity
chain rule composes derivative information
```

## Checkpoint

Why is "local rule plus composition" the core idea behind backpropagation?

A strong answer:

> Each operation only needs its local derivative; the chain rule composes those
> local derivatives into the gradient for the whole computation.

## Where This Leaves Us

This chapter named the repeated structures that sit underneath the small ML
pipeline. A functor explains mapping inside a wrapper. A natural transformation
explains changing wrappers consistently. A monoid explains safe accumulation. A
local gradient explains why a large training computation can be assembled from
small derivative rules.

The next chapter uses the same engineering habit on a wider set of ideas from
applied category theory. Instead of adding a larger ML model, it shows how the
same typed-Rust style can model orders, resources, database instances, design
relations, signal flow, circuits, and local-to-global behavior.

## Further Reading

These pages reinforce the structure vocabulary used here:

- [Glossary](glossary.md): functor, natural transformation, monoid, chain rule
- [References](references.md): applied category theory, deep learning math, and attention
