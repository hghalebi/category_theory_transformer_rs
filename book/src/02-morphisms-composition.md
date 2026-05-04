# Morphism and Composition

The problem this chapter solves is:

> Once the system has typed objects, it needs typed transformations between
> them.

In the previous chapter, the code created objects such as:

```text
TokenId
Vector
Logits
Distribution
Loss
Parameters
```

This chapter explains the arrows that connect them.

The central category-theory sentence is:

> A morphism is a typed arrow from one object to another.

The central Rust sentence is:

> A morphism is a trait implementation with an input type, output type, and
> typed error result.

> Reader orientation:
> The previous chapter defined the objects of the tiny ML system. This chapter
> explains how values move between those objects. That movement is the bridge
> between ordinary Rust functions and the categorical idea of morphisms.

## What You Already Know

If you know Rust functions, you already know that computation moves from an
input type to an output type. If you know ML pipelines, you already know that a
prediction path is built from stages. This chapter gives that familiar movement
a shared interface: `Morphism<Input, Output>`.

## Source Snapshot

This file defines the typed arrow interface and the composition adapter.

<details>
<summary>Source snapshot: src/category.rs</summary>

```rust,ignore
{{#include ../../src/category.rs}}
```

</details>

## The Whole File

`src/category.rs` defines:

```text
Morphism<Input, Output>
Identity<T>
Compose<F, G, Middle>
Endomorphism<T>
StepCount
apply_endomorphism_n_times
```

These are the abstract shapes used by the ML code.

Without this file, prediction could still be written as ordinary functions.

With this file, the course can name and test the structure:

```text
identity
composition
endomorphism
repeated application
```

Read each block through the same three lenses:

```text
Rust syntax:
what trait, struct, generic parameter, or bound is declared?

ML concept:
which model pipeline behavior does the shape support?

Category theory concept:
which arrow, identity, composition, or endomorphism idea is being modeled?
```

## Worked Example: A Function As An Arrow

Before reading the generic trait, start with an ordinary Rust function:

```rust
fn add_one(input: i32) -> i32 {
    input + 1
}

assert_eq!(add_one(41), 42);
```

That function already has an arrow shape:

```text
i32 -> i32
```

The real `Morphism<Input, Output>` trait makes that shape explicit, gives the
arrow a name, and lets the arrow fail with a typed error when the input cannot
be transformed safely.

## Self-Check

Before reading the trait, explain why `i32 -> i32` and `TokenId -> Vector` have
the same arrow shape even though they mean very different things.

## `Morphism<Input, Output>`

The problem this block solves is:

> The code needs one shared contract for typed transformations.

The block:

```rust,ignore
/// A typed category-theory arrow: `Input -> Output`.
pub trait Morphism<Input, Output> {
    fn name(&self) -> &'static str;
    fn apply(&self, input: Input) -> CtResult<Output>;
}
```

### Rust Syntax: Documentation Comment

```rust,ignore
/// A typed category-theory arrow: `Input -> Output`.
```

This tells you how to read the trait.

For example:

```text
Embedding : TokenId -> Vector
```

means:

```rust,ignore
impl Morphism<TokenId, Vector> for Embedding
```

### Rust Syntax: Trait Definition

```rust,ignore
pub trait Morphism<Input, Output>
```

`Input` and `Output` are type parameters.

They are not values.

They describe the type-level shape of the arrow.

This allows the same trait to model:

```text
TokenSequence -> TrainingSet
TokenId -> Vector
Vector -> Logits
Logits -> Distribution
Distribution x TokenId -> Loss
Parameters -> Parameters
```

### Rust Syntax: `name`

```rust,ignore
fn name(&self) -> &'static str;
```

This gives a stable human-readable name.

It is useful for demonstrations, diagnostics, and teaching.

The return type `&'static str` means the string is known for the whole program
lifetime. Names such as `"softmax"` and `"embedding"` are static literals.

### Rust Syntax: `apply`

```rust,ignore
fn apply(&self, input: Input) -> CtResult<Output>;
```

This is the actual transformation.

It consumes an `Input` and returns either:

```text
Ok(Output)
```

or:

```text
Err(CtError)
```

This is important because many arrows can fail. Embedding can receive an
out-of-range token, softmax can receive empty logits, cross entropy can receive
an invalid target, and training can receive malformed parameters. The shared
return type keeps those failures explicit instead of hiding them behind a
panic.

### ML Concept

Every ML stage becomes an implementation of the same contract.

That makes the pipeline inspectable as arrows, not just function calls.

### Category Theory Concept

This trait is the course's concrete model of a morphism.

It is not trying to implement all category theory. It gives enough structure to
talk about typed arrows and composition in ordinary Rust.

## `Identity<T>`

The problem this block solves is:

> Every object should have an arrow that returns the object unchanged.

The block:

```rust,ignore
/// Identity morphism: `id_A : A -> A`.
#[derive(Debug, Clone, Copy)]
pub struct Identity<T> {
    _marker: PhantomData<T>,
}
```

### Rust Syntax: Why The Struct Has No Real Data

`Identity<T>` does not need to store a `T`.

It only needs to remember the type `T`.

That is why it stores:

```rust,ignore
_marker: PhantomData<T>
```

`PhantomData<T>` tells Rust:

> This struct is logically connected to `T`, even though it does not own a real
> `T` value.

### Rust Syntax: Constructor

```rust,ignore
pub fn new() -> Self {
    Self {
        _marker: PhantomData,
    }
}
```

This creates the identity arrow for a type.

Example:

```rust,ignore
Identity::<Vector>::new()
```

means:

```text
id_Vector : Vector -> Vector
```

### Rust Syntax: Default

```rust,ignore
impl<T> Default for Identity<T> {
    fn default() -> Self {
        Self::new()
    }
}
```

This follows Rust convention: if a type has an obvious empty constructor, it can
implement `Default`.

### Rust Syntax: Morphism Implementation

```rust,ignore
impl<T> Morphism<T, T> for Identity<T> {
    fn name(&self) -> &'static str {
        "identity"
    }

    fn apply(&self, input: T) -> CtResult<T> {
        Ok(input)
    }
}
```

This is the key:

```text
T -> T
```

The input and output type are the same.

The implementation simply returns the input.

### ML Concept

Identity is a no-op transformation.

In a model pipeline, no-op stages are useful for tests and for understanding
what it means for composition to have a neutral element.

### Category Theory Concept

Identity matters because composition has laws:

```text
id after f = f
f after id = f
```

This code does not prove those laws generally, but it gives the object you need
to talk about them in Rust.

## `Compose<F, G, Middle>`

The problem this block solves is:

> If one morphism produces the type another morphism consumes, the code should
> be able to build a larger morphism.

The block:

```rust,ignore
/// Composition of two morphisms: if `f : A -> B` and `g : B -> C`, this is
/// `g after f : A -> C`.
#[derive(Debug, Clone)]
pub struct Compose<F, G, Middle> {
    first: F,
    second: G,
    _middle: PhantomData<Middle>,
}
```

### Rust Syntax: The Shape

The category-theory shape is:

```text
f : A -> B
g : B -> C
g after f : A -> C
```

The Rust type is:

```rust,ignore
Compose<F, G, Middle>
```

where:

- `F` is the first morphism
- `G` is the second morphism
- `Middle` is the bridge type

The middle type is explicit because Rust needs to know what connects the two
arrows.

### Rust Syntax: Fields

```rust,ignore
first: F,
second: G,
_middle: PhantomData<Middle>,
```

`first` stores the first arrow.

`second` stores the second arrow.

`_middle` records the bridge type without storing a value of that type.

### Rust Syntax: Constructor

```rust,ignore
pub fn new(first: F, second: G) -> Self
```

This builds the composed morphism.

It does not run the morphisms yet.

It only stores them.

### Rust Syntax: Morphism Implementation

```rust,ignore
impl<Input, Middle, Output, F, G> Morphism<Input, Output>
    for Compose<F, G, Middle>
where
    F: Morphism<Input, Middle>,
    G: Morphism<Middle, Output>,
{
    fn apply(&self, input: Input) -> CtResult<Output> {
        let middle = self.first.apply(input)?;
        self.second.apply(middle)
    }
}
```

This is the most important block in the chapter.

The `where` clause says:

```text
F must be Input -> Middle
G must be Middle -> Output
```

Only then can `Compose<F, G, Middle>` be:

```text
Input -> Output
```

### Rust Syntax: The `?` Operator

```rust,ignore
let middle = self.first.apply(input)?;
```

This applies the first arrow.

If it fails, the error returns immediately.

If it succeeds, the successful value is bound to `middle`.

Then the second arrow runs:

```rust,ignore
self.second.apply(middle)
```

So composition preserves failure.

It does not hide invalid states.

### ML Concept

Prediction uses composition:

```text
TokenId -> Vector -> Logits -> Distribution
```

The code builds that in two steps:

```rust,ignore
let token_to_logits = Compose::<_, _, Vector>::new(embedding, linear);
let token_to_distribution = Compose::<_, _, Logits>::new(token_to_logits, Softmax);
```

The bridge types are:

```text
Vector
Logits
```

If you try to compose `Embedding` directly with `Softmax`, the middle type does
not match:

```text
Embedding : TokenId -> Vector
Softmax   : Logits -> Distribution
```

`Vector` is not `Logits`, so Rust rejects the composition.

### Category Theory Concept

`Compose` is function composition with types made explicit.

It is the course's main example of:

```text
small legal arrows -> larger legal arrow
```

## `Endomorphism<T>`

The problem this block solves is:

> Some arrows start and end at the same type, and those arrows can be repeated.

The block:

```rust,ignore
/// Endomorphism: a morphism from a type back to itself.
pub trait Endomorphism<T>: Morphism<T, T> {}

impl<T, M> Endomorphism<T> for M where M: Morphism<T, T> {}
```

An endomorphism has shape:

```text
T -> T
```

The trait has no methods of its own.

It is a marker trait:

```text
if something implements Morphism<T, T>, it is an Endomorphism<T>
```

The blanket implementation says exactly that:

```rust,ignore
impl<T, M> Endomorphism<T> for M where M: Morphism<T, T> {}
```

### ML Concept

Training has this shape:

```text
Parameters -> Parameters
```

One training step consumes parameters and returns updated parameters.

The model changes, but the type stays the same.

### Category Theory Concept

Endomorphisms are important because they can be iterated:

```text
A -> A -> A -> A
```

That is the categorical shape of repeated training.

## `StepCount`

The problem this block solves is:

> Repetition count should have a semantic name instead of being a random
> `usize` at the call site.

The block:

```rust,ignore
/// How many times to repeat an endomorphism.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StepCount(usize);
```

This wraps a raw `usize`.

It means:

```text
number of repeated endomorphism applications
```

`StepCount::new(80)` reads better than a bare `80` because it names the role of
the number.

### Rust Syntax

`StepCount` is a newtype around `usize`.

It has a constructor and a `value()` accessor.

### ML Concept

It controls how many optimizer steps are applied.

### Category Theory Concept

It controls how many times an endomorphism is iterated.

## `apply_endomorphism_n_times`

The problem this block solves is:

> Given an endomorphism, repeatedly apply it in a type-safe loop.

The block:

```rust,ignore
pub fn apply_endomorphism_n_times<T, E>(
    endo: &E,
    mut value: T,
    count: StepCount,
) -> CtResult<T>
where
    E: Endomorphism<T>,
{
    for _ in 0..count.value() {
        value = endo.apply(value)?;
    }

    Ok(value)
}
```

### Rust Syntax: Type Parameters

`T` is the object being updated.

`E` is the endomorphism type.

The bound:

```rust,ignore
E: Endomorphism<T>
```

means:

```text
E must be a T -> T arrow
```

### Rust Syntax: Mutable Value

```rust,ignore
mut value: T
```

The function owns the current value.

Each loop iteration replaces it with the next value:

```rust,ignore
value = endo.apply(value)?;
```

This is not mutation of shared global state.

It is ownership passing through a repeated transformation.

### Rust Syntax: Failure Behavior

If any application fails, the whole repeated process fails immediately.

This is the correct behavior for training too: if a step discovers invalid
parameters or an out-of-range token, the loop should not pretend everything is
fine.

### ML Concept

For training:

```text
T = Parameters
E = TrainStep
```

The function becomes:

```text
repeat TrainStep on Parameters
```

### Category Theory Concept

This is iteration of an endomorphism:

```text
value0
  -> value1
  -> value2
  -> ...
  -> valueN
```

## Runnable Example

The composition example builds:

```text
TokenId -> Vector -> Logits -> Distribution
```

<details>
<summary>Source snapshot: examples/02_morphism_composition.rs</summary>

```rust,ignore
{{#include ../../examples/02_morphism_composition.rs}}
```

</details>

Run:

```bash
cargo run --example 02_morphism_composition
```

## Why This API Is Good Design

The code does not make composition a loose runtime convention.

It puts composition into the type system.

That means the compiler checks the bridge type:

```text
F output == G input
```

This is the core practical value of the category-theory framing in this repo.

It turns:

```text
remember to wire the stages correctly
```

into:

```text
make invalid wiring fail to compile
```

## Core Mental Model

In Rust terms:

```text
Morphism<Input, Output> = fallible typed transformation
Compose<F, G, Middle> = legal connection of two transformations
Endomorphism<T> = repeatable T -> T transformation
```

In ML terms:

```text
small prediction stages compose into a model path
training is a repeatable update step
```

In category-theory terms:

```text
objects are connected by arrows, arrows compose when their endpoints match
```

## Checkpoint

Why does this composition compile:

```text
TokenId -> Vector -> Logits
```

but this one does not:

```text
TokenId -> Vector -> Distribution
```

A strong answer should mention that `Softmax` expects `Logits`, not `Vector`.

## Where This Leaves Us

This chapter turned ordinary transformations into named arrows. `Identity<T>`
leaves a value unchanged, `Compose<F, G, Middle>` connects compatible arrows,
and `Endomorphism<T>` names the special case where the input and output object
are the same.

The next chapter fills those arrow shapes with concrete ML behavior: token
windowing, embedding lookup, linear projection, softmax, and cross entropy.

## Further Reading

These pages give the supporting vocabulary for the arrow layer:

- [Glossary](glossary.md): morphism, identity morphism, composition, endomorphism
- [References](references.md): applied category theory and Rust module structure

## Retrieval Practice

### Recall

What does `Morphism<Input, Output>` require an implementation to provide?

### Explain

Why does `Compose<F, G, Middle>` need the middle type to match?

### Apply

Write a diagram for the legal path from `TokenId` to `Distribution`, naming the
middle objects.
