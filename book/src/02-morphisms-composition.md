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

> A morphism is a typed transformation from one object to another.

The central Rust sentence is:

> A morphism is a trait implementation with an input type, output type, and
> typed error result.

> Reader orientation:
> The previous chapter defined the objects of the tiny ML system. This chapter
> explains how values move between those objects. That movement is the bridge
> between ordinary Rust functions and the categorical idea of morphisms.

## Chapter Outcomes

By the end of this chapter, you should be able to:

- read `Morphism<Input, Output>` as a typed transformation contract,
- explain why `Compose<F, G, Middle>` requires the first target object to match
  the second source object,
- diagnose why `Embedding` followed directly by `Softmax` is illegal without
  weakening either stage.

## What You Already Know

If you know Rust functions, you already know that computation moves from an
input type to an output type. If you know ML pipelines, you already know that a
prediction path is built from stages. This chapter gives that familiar movement
a shared interface: `Morphism<Input, Output>`.

## Category Terms As Rust Shapes

Before reading the generic source file, pin each category-theory word to a
Rust shape and one tiny ML example.

| Category term | Rust shape in this repository | Tiny ML example |
| --- | --- | --- |
| Object | A named type that can appear as an input or output | `TokenId`, `Vector`, `Logits`, `Distribution` |
| Morphism | `impl Morphism<Input, Output> for SomeStage` | `impl Morphism<TokenId, Vector> for Embedding` |
| Source object | The `Input` type parameter | `TokenId` in `Morphism<TokenId, Vector>` |
| Target object | The `Output` type parameter | `Vector` in `Morphism<TokenId, Vector>` |
| Identity morphism | `Identity<T>` implementing `Morphism<T, T>` | `Identity::<Vector>::new()` |
| Composition | `Compose<F, G, Middle>` | `Embedding` followed by `LinearToLogits` |
| Middle object | The type produced by `F` and consumed by `G` | `Vector` between embedding and projection |
| Endomorphism | `Endomorphism<T>` where input and output are the same type | `TrainStep : Parameters -> Parameters` |
| Repeated endomorphism | `apply_endomorphism_n_times` | repeated training updates |

Use the table as a translation layer. When a formal word appears later, ask
which Rust trait, type parameter, or implementation makes it concrete. If no
Rust shape is nearby, the explanation is probably moving too fast.

## Source-Backed Precision Rules

This chapter uses external sources to keep the word `morphism` small enough to
teach. Each source supports a limited claim, and each claim is tied to one
local Rust boundary. The chapter does not claim that this crate implements a
general category-theory library; it models typed transformations, identity,
composition, and one repeated endomorphism helper for the tiny ML system.

| Source | What the source supports | Local rule in this chapter | Rust evidence |
| --- | --- | --- | --- |
| [Rust Book: Generics](https://doc.rust-lang.org/book/ch10-01-syntax.html) | Generic type parameters let one definition describe many concrete types while preserving type relationships. | Read `Input`, `Middle`, and `Output` as type-level objects, not runtime values. | `Morphism<Input, Output>`, `Compose<F, G, Middle>` |
| [Rust Book: Traits](https://doc.rust-lang.org/book/ch10-02-traits.html) | Traits name shared behavior and make a contract that concrete types implement. | Treat a morphism as a trait contract: a named, fallible, typed transformation. | `trait Morphism<Input, Output>`, `impl Morphism<TokenId, Vector> for Embedding` |
| [Stanford Encyclopedia of Philosophy: Category Theory](https://plato.stanford.edu/entries/category-theory/) | A category has morphisms between objects, identity morphisms, composition, and identity/associativity axioms. | Keep the local Rust claim narrow: the chapter models source type, target type, identity, and composition for this teaching crate. | `Identity<T>`, `Compose<F, G, Middle>`, `identity_composes_without_changing_behavior`, `composition_applies_first_then_second` |
| [Seven Sketches](https://arxiv.org/abs/1803.05316) | Category theory introduces objects, arrows, identity, and composition through concrete applied examples. | Use category words only when they point to a visible Rust object, arrow, identity, or composition boundary. | `Identity<T>`, `Compose<F, G, Middle>`, `identity_composes_without_changing_behavior` |
| [Category Theory for Programming](https://arxiv.org/abs/2209.01259) | Programming-shaped category-theory notes connect categorical vocabulary to datatypes, functions, and typed structure. | Explain the ordinary typed-function shape before using the word `morphism`. | `fn add_one(input: i32) -> i32`, `Morphism<Input, Output>` |

The transfer pattern is:

```text
source idea -> local typed boundary -> compiler, output, or test evidence
```

For this chapter, that means reading `cargo run --example
02_morphism_composition`, `cargo test category::tests`, and the failed-shape
diagnostic as evidence for a small claim:

```text
two arrows compose only when the first target object matches the second source
object
```

It is not evidence that every categorical law has been formalized. It is
evidence that this tiny Rust interface makes the relevant middle object hard to
ignore.

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

## Worked Example: Where Composition Breaks

Now look at a tiny ML path:

```text
TokenId -> Vector -> Logits -> Distribution
```

Each arrow has a job:

```text
Embedding       : TokenId -> Vector
LinearToLogits  : Vector -> Logits
Softmax         : Logits -> Distribution
```

A legal composition connects the target of one arrow to the source of the next
arrow:

```text
TokenId --Embedding--> Vector --LinearToLogits--> Logits --Softmax--> Distribution
```

The middle object is not decoration. It is the reason the pipeline is legal.
`Embedding` produces a `Vector`, and `LinearToLogits` consumes a `Vector`.
`LinearToLogits` produces `Logits`, and `Softmax` consumes `Logits`.

Now remove the middle step:

```text
TokenId --Embedding--> Vector --Softmax--> Distribution
```

This looks tempting if you only think in English: "turn the token into
probabilities." But the types say something more precise. `Softmax` does not
consume a `Vector`. It consumes `Logits`. The missing arrow is:

```text
Vector -> Logits
```

That is why composition is not just "run functions in order." Composition means:

```text
the previous output type equals the next input type
```

In this chapter, the word "morphism" gives that rule a handle. A morphism has a
source object and a target object. Two morphisms compose only when the first
target object is the second source object.

## Runnable Example: The Middle Type Is The Contract

Run the example for this chapter:

```bash
cargo run --example 02_morphism_composition
```

The example builds the legal path in two composed steps:

```rust,ignore
let token_to_logits = Compose::<_, _, Vector>::new(embedding.clone(), linear.clone());
let token_to_distribution = Compose::<_, _, Logits>::new(token_to_logits, Softmax);
```

Read the third type argument as the middle object being checked:

```text
Embedding then LinearToLogits:
TokenId -> Vector -> Logits
middle object: Vector

token_to_logits then Softmax:
TokenId -> Logits -> Distribution
middle object: Logits
```

The example output ends with the composition rule:

```text
first target must equal second source
Embedding then LinearToLogits is legal because Vector == Vector
Embedding then Softmax is illegal because Vector != Logits
```

This is the concrete Rust reason a morphism is more than a metaphor. The type
signature tells you which object an arrow produces and which object the next
arrow expects. If those do not match, the composed pipeline is not a valid
pipeline.

## Composition Debugging Checklist

When a composition fails, do not start by changing type signatures. Name the
three objects first:

```text
first source  -> first target
second source -> second target
```

Then ask whether the middle objects match:

```text
first target == second source ?
```

For the legal path:

| Stage | Source object | Target object |
| --- | --- | --- |
| `Embedding` | `TokenId` | `Vector` |
| `LinearToLogits` | `Vector` | `Logits` |
| `Softmax` | `Logits` | `Distribution` |

The legal middle objects are:

```text
Vector
Logits
```

The same legal path as a rendered math view:

\[
\mathrm{TokenId}
  \xrightarrow{\mathrm{Embedding}}
\mathrm{Vector}
  \xrightarrow{\mathrm{LinearToLogits}}
\mathrm{Logits}
  \xrightarrow{\mathrm{Softmax}}
\mathrm{Distribution}
\]

How to read this diagram:

- the objects are the Rust domain types,
- the arrows are morphism implementations,
- composition is legal only when the target object of one arrow is the source
  object of the next arrow,
- the diagram is a reading aid, not a claim that Rust proves every category
  law.

For the broken shortcut:

| Attempted composition | First target | Second source | Result |
| --- | --- | --- | --- |
| `Embedding` then `Softmax` | `Vector` | `Logits` | illegal composition |

The fix is not to make `Softmax` accept `Vector`. That would erase the model
stage that turns hidden features into vocabulary scores. The fix is to restore
the missing morphism:

```text
Vector -> Logits
```

The broken shortcut is useful to draw because it exposes the missing middle
object:

\[
\begin{array}{ccccc}
\mathrm{TokenId} & \xrightarrow{\mathrm{Embedding}} & \mathrm{Vector}
  & \not\!\xrightarrow{\mathrm{Softmax}} & \mathrm{Distribution} \\
&& \downarrow \mathrm{LinearToLogits} && \\
&& \mathrm{Logits} & \xrightarrow{\mathrm{Softmax}} & \mathrm{Distribution}
\end{array}
\]

Reconstruct this diagram by hand when a composition error appears. Label the
first target, the second source, and the repair arrow before changing code.

This checklist is useful beyond this chapter. Most pipeline bugs can be read
as one of three failures:

| Failure | Diagnostic question | Repair |
| --- | --- | --- |
| missing stage | Which middle object should exist but does not? | restore the morphism that produces it |
| wrong stage order | Which target object arrives too early or too late? | reorder the arrows so targets meet sources |
| wrong object name | Which two values have the same raw representation but different roles? | introduce or restore the domain type |

The category-theory word "composition" is doing practical engineering work
here. It tells you to debug the boundary, not the individual matrix
multiplication, softmax formula, or display output first.

## Source-Target-Middle Repair Ledger

When a composition breaks, write a small ledger before changing code. The
ledger forces the abstract word "composition" back into source object, target
object, middle object, and repair.

| Composition attempt | First arrow | Second arrow | Claimed middle | Actual mismatch | Repair | Unsafe shortcut rejected | Validation evidence |
| --- | --- | --- | --- | --- | --- | --- | --- |
| legal embedding then projection | `Embedding : TokenId -> Vector` | `LinearToLogits : Vector -> Logits` | `Vector` | none | keep the order | skipping vocabulary scoring | `Embedding then LinearToLogits is legal because Vector == Vector` |
| illegal embedding then softmax | `Embedding : TokenId -> Vector` | `Softmax : Logits -> Distribution` | `Vector` | `Softmax` needs `Logits`, not `Vector` | restore `LinearToLogits : Vector -> Logits` | making `Softmax` accept hidden features | `Embedding then Softmax is illegal because Vector != Logits` |
| legal projection then softmax | `LinearToLogits : Vector -> Logits` | `Softmax : Logits -> Distribution` | `Logits` | none | keep the order | treating logits as optional decoration | `Compose::<_, _, Logits>` |

Use this audit card when the compiler, a diagram, or a reader's intuition says
two stages should connect:

```text
composition attempt:
first arrow:
second arrow:
claimed middle object:
actual first target:
actual second source:
repair:
unsafe shortcut rejected:
validation command or output:
```

Worked audit:

```text
composition attempt: Embedding then Softmax
first arrow: Embedding : TokenId -> Vector
second arrow: Softmax : Logits -> Distribution
claimed middle object: Vector
actual first target: Vector
actual second source: Logits
repair: insert LinearToLogits : Vector -> Logits
unsafe shortcut rejected: changing Softmax to accept Vector
validation command or output:
  cargo run --example 02_morphism_composition
  Embedding then Softmax is illegal because Vector != Logits
```

The source-backed limit is important. The Rust compiler is not proving every
theorem about categories. It is checking the local trait bounds that make this
pipeline composition legal or illegal.

## Compiler Error As Evidence

The example does not include a broken composition because examples in this
repository are expected to run. But the failed shape is still worth naming.

If you try to compose `Embedding` directly with `Softmax`, the intended shape
would be:

```text
Embedding : TokenId -> Vector
Softmax   : Logits -> Distribution
```

For `Compose<F, G, Middle>` to implement `Morphism<Input, Output>`, Rust needs
these two facts:

```text
F: Morphism<Input, Middle>
G: Morphism<Middle, Output>
```

With `Embedding` followed by `Softmax`, choosing `Middle = Vector` asks Rust
for:

```text
Embedding : Morphism<TokenId, Vector>
Softmax   : Morphism<Vector, Distribution>
```

The first fact is true. The second fact is false. `Softmax` is implemented for
`Logits -> Distribution`, not `Vector -> Distribution`.

That failed trait bound is not noise. It says the missing middle object is:

```text
Logits
```

and the missing morphism is:

```text
LinearToLogits : Vector -> Logits
```

So the repair is not to make `Softmax` accept `Vector`. The repair is to
restore the stage that turns hidden features into vocabulary scores.

## From Function To Morphism

An ordinary Rust function already has the outline:

```text
Input -> Output
```

The course's `Morphism<Input, Output>` trait adds three things to that outline.
It gives the transformation a stable name, makes failure explicit with
`CtResult<Output>`, and lets different transformation structs share one
composition API.

That is why this chapter uses the word "morphism" carefully. In this codebase,
read it first as:

```text
a named, fallible, typed transformation
```

Only after that concrete reading should you attach the category-theory word.

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

### Read One Concrete Implementation

The abstract trait becomes concrete when a stage implements it. In `src/ml.rs`,
the embedding stage has this shape:

```rust,ignore
impl Morphism<TokenId, Vector> for Embedding {
    fn name(&self) -> &'static str {
        "embedding"
    }

    fn apply(&self, token: TokenId) -> CtResult<Vector> {
        // lookup and validation happen here
    }
}
```

Read the first line slowly:

```text
Embedding is a morphism from TokenId to Vector.
```

That one line gives the reader all four pieces requested by the public starter
issue:

| Piece | In the code | Meaning |
| --- | --- | --- |
| typed input | `TokenId` | a vocabulary position, not a raw integer |
| typed output | `Vector` | hidden features for that token |
| transformation | `Embedding` | table lookup from token to feature row |
| explicit failure | `CtResult<Vector>` | out-of-range tokens return an error |

The implementation does not say that every `usize` can become features. It says
that a validated `TokenId` can be applied to this embedding table, and the
result is either a `Vector` or a typed error.

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

The tests in `src/category.rs` check the executable version of this idea:
composing identity on either side of a simple morphism leaves the behavior
unchanged.

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

This is the most important learner habit in the chapter: when composition feels
abstract, look for the middle type.

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

The category tests also check this behavior directly. A composed morphism that
fails in its first step returns that error immediately instead of pretending the
second step ran.

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

The legal diagram is:

```text
TokenId
   |
   | Embedding
   v
Vector
   |
   | LinearToLogits
   v
Logits
   |
   | Softmax
   v
Distribution
```

The important detail is not the vertical layout. The important detail is that
every arrow's output object is exactly the next arrow's input object.

If you try to compose `Embedding` directly with `Softmax`, the middle type does
not match:

```text
Embedding : TokenId -> Vector
Softmax   : Logits -> Distribution
```

`Vector` is not `Logits`, so Rust rejects the composition.

This is the practical win. A diagram that skips `LinearToLogits` is not only
conceptually wrong; it has the wrong type boundary.

### Category Theory Concept

`Compose` is function composition with types made explicit.

It is the course's main example of:

```text
small legal arrows -> larger legal arrow
```

The code is deliberately modest. It models enough composition to make the
pipeline inspectable and testable; it is not claiming to encode every
categorical law in Rust's type system.

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

Expected shape:

```text
Input object:
TokenId(1)

Stage outputs:
Embedding : TokenId -> Vector
Vector(dim=4, values=[...])
LinearToLogits : Vector -> Logits
Logits(vocab=5, values=[...])
Softmax : Logits -> Distribution
Distribution(vocab=5, sum=1.000000, values=[...])

Composed morphism:
TokenId -> Distribution
next-token probabilities: [...]

Middle objects kept visible:
Vector
Logits
```

## Example Output Transfer Checklist

The example prints stage outputs and then prints the composed arrow. Read that
output as a composition report, not only as a numeric demo.

| Example output or code evidence | Rust reading | ML reading | Category-theory reading | Shortcut to reject |
| --- | --- | --- | --- | --- |
| `TokenId(1)` | the input is a named object, not a bare index | choose one context token | source object | passing an unnamed row number through the pipeline |
| `Embedding : TokenId -> Vector` | `Embedding` implements `Morphism<TokenId, Vector>` | look up the token's hidden feature row | arrow from source object to middle object | passing a token directly to projection |
| `Vector(dim=4, values=[...])` | a `Vector` value exists before projection | hidden representation, not vocabulary scores | first middle object | treating features as logits |
| `LinearToLogits : Vector -> Logits` | `LinearToLogits` implements `Morphism<Vector, Logits>` | project hidden features into vocabulary scores | arrow between middle objects | sending a vector directly to `Softmax` |
| `Logits(vocab=5, values=[...])` | unnormalized scores have their own type | one score per vocabulary item | second middle object | treating scores as probabilities |
| `Softmax : Logits -> Distribution` | `Softmax` implements `Morphism<Logits, Distribution>` | normalize scores into probabilities | arrow into the target object | computing loss before a probability object exists |
| `Distribution(vocab=5, sum=1.000000, values=[...])` | constructor validation produced a distribution | next-token probabilities sum to one | target object | treating arbitrary floats as a probability distribution |
| `Compose::<_, _, Vector>` | `Vector` is the first bridge type | embedding must happen before projection | legal composition through a middle object | hiding the bridge type and guessing that stages fit |
| `Compose::<_, _, Logits>` | `Logits` is the second bridge type | projection must happen before softmax | legal composition through a middle object | forgetting that `Softmax` needs logits |
| `TokenId -> Distribution` | the composed value is a larger morphism | the prediction path is now one callable stage | composite arrow | thinking composition erases intermediate obligations |

This is the chapter's most important transfer move. The user-facing output is
compact:

```text
next-token probabilities: [...]
```

The typed explanation is larger:

```text
TokenId -> Vector -> Logits -> Distribution
```

A strong reader can connect both views. The numeric output tells you what the
pipeline produced. The typed path tells you why the pipeline was legal.

The stage outputs also explain the ML meaning of the middle objects:

```text
Vector       = hidden features
Logits       = vocabulary scores
Distribution = normalized next-token probabilities
```

The category-theory discipline is to keep those middle objects visible. A
composite arrow can be named `TokenId -> Distribution`, but the legal route is
still built from the two bridge objects `Vector` and `Logits`.

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

The next chapter, [The Tiny ML Pipeline](03-ml-pipeline.md), fills those arrow
shapes with concrete ML behavior: token windowing, embedding lookup, linear
projection, softmax, and cross entropy.

## Further Reading

Do not use these sources to make the word "morphism" sound larger. Use them to
debug one concrete question:

```text
what is the source object, target object, and middle object?
```

Start from the local Rust evidence:

```text
Morphism<Input, Output>
Compose<F, G, Middle>
F: Morphism<Input, Middle>
G: Morphism<Middle, Output>
Embedding      : TokenId -> Vector
LinearToLogits : Vector -> Logits
Softmax        : Logits -> Distribution
```

Then read the sources in this order:

| Source | What to transfer back into this chapter | Local evidence to inspect |
| --- | --- | --- |
| [Rust Book: Generics](https://doc.rust-lang.org/book/ch10-01-syntax.html) | Generic parameters preserve relationships between input, middle, and output types. | `Compose<F, G, Middle>` |
| [Rust Book: Traits](https://doc.rust-lang.org/book/ch10-02-traits.html) | A trait defines the method signatures each implementation must provide. | `trait Morphism<Input, Output>` |
| [Stanford Encyclopedia of Philosophy: Category Theory](https://plato.stanford.edu/entries/category-theory/) | The formal category shape needs morphisms, identity, composition, associativity, and identity laws. | `Identity<T>`, `Compose<F, G, Middle>`, `composition_applies_first_then_second` |
| [Seven Sketches](https://arxiv.org/abs/1803.05316) | Objects, arrows, identity, and composition can be introduced through concrete applied examples. | `Identity<T>`, `Compose<F, G, Middle>` |
| [Category Theory for Programming](https://arxiv.org/abs/2209.01259) | Category-theory vocabulary can be connected to typed programming structure. | `fn add_one(input: i32) -> i32`, `Morphism<Input, Output>` |

After reading one external source, ask four questions:

1. Which local boundary did it clarify?
2. Which type relationship did it help protect?
3. Which illegal composition does it help reject?
4. Which command would you run to see the evidence?

For this chapter, the commands are:

```bash
cargo run --example 02_morphism_composition
cargo test category::tests --lib
cargo test ml::tests::composed_and_direct_prediction_match --lib
```

Use [Glossary](glossary.md) when a term becomes slippery. Use
[References](references.md) when you want the full source list.

If a source does not help you explain why `Embedding` can compose with
`LinearToLogits` but not directly with `Softmax`, it has not transferred back
into the chapter yet.

## Practice After This Chapter

Use [Exercise 4](exercises.md#exercise-4-break-a-composition) to intentionally
break a composition and explain the missing middle type. This is the chapter's
most important transfer check: a type error should become evidence about the
pipeline boundary.

## Retrieval Practice

### Recall

Recover the shape of the API before explaining the pipeline.

1. What two methods must `Morphism<Input, Output>` provide?
2. Which type in `Compose<F, G, Middle>` records the bridge between two arrows?
3. What shape makes a morphism an endomorphism?

### Explain

Use the middle object to explain why composition is legal or illegal.

1. Why does `Compose<F, G, Middle>` require `F: Morphism<Input, Middle>` and
   `G: Morphism<Middle, Output>`?
2. Why is `TokenId -> Vector -> Distribution` not a legal version of the
   prediction path?
3. Why does composition return the first error instead of trying to run the
   second arrow?

### Apply

Use the output from `cargo run --example 02_morphism_composition` as the
working path.

1. Write the legal path from `TokenId` to `Distribution`, naming both middle
   objects.
2. If you insert `Identity<Vector>` between `Embedding` and `LinearToLogits`,
   why should the behavior stay the same?
3. If you try to repeat `Embedding` with `apply_endomorphism_n_times`, which
   shape rule blocks the attempt?

### Debug

For each invalid shortcut, name the missing or mismatched middle type:

```text
Embedding followed directly by Softmax
Embedding followed by Identity<TokenId>
repeating Embedding as an endomorphism
```

A strong answer should identify the source and target object of each arrow,
then state which object fails to line up. Do not answer only with "the compiler
rejects it"; explain the typed boundary the compiler is protecting.
