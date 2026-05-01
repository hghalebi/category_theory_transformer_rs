# Functors, Naturality, Monoids, and Chain Rule

The problem this chapter solves is:

> After seeing individual ML arrows, you need names for common structures that
> appear across many systems: mapping inside containers, converting containers
> consistently, combining traces, and composing local derivatives.

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

## `Functor<A, B>`

The problem this block solves is:

> The code needs a name for "apply a function inside a wrapper while keeping the
> wrapper shape."

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

`Functor<A, B>` is a trait with two type parameters:

```text
A = input item type
B = output item type
```

It has associated types:

```rust,ignore
type WrappedA;
type WrappedB;
```

These say what wrapper shape is used around `A` and `B`.

The method:

```rust,ignore
fn fmap<F>(wrapped: Self::WrappedA, f: F) -> Self::WrappedB
where
    F: Fn(A) -> B;
```

means:

> Given a wrapped `A` and a function `A -> B`, produce a wrapped `B`.

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

The implementation says:

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

## `NaturalTransformation<A>`

The problem this block solves is:

> Sometimes you need to convert one wrapper shape into another without caring
> about the specific item type.

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

The trait:

```rust,ignore
pub trait Monoid: Sized {
    fn empty() -> Self;
    fn combine(&self, other: &Self) -> Self;
}
```

## Rust Syntax

`Sized` means values of this type have a known size at compile time.

The trait requires:

```text
empty() -> Self
combine(&self, other: &Self) -> Self
```

So a monoid can produce an identity value and combine two values into one.

## ML Concept

Common monoid-like values in ML systems include:

- logs
- traces
- metrics
- batches
- accumulated gradients

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

You should see:

- mapping over `Vec`
- mapping over `Option`
- a naturality check
- a combined trace
- monoid law check
- local gradients for multiplication

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

## Further Reading

- [Glossary](glossary.md): functor, natural transformation, monoid, chain rule
- [References](references.md): applied category theory, deep learning math, and attention
