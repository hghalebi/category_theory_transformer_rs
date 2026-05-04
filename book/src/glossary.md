# Glossary

The problem this chapter solves is:

> Abstract terms are easier to remember when each term is tied to a Rust type,
> an ML role, and a category-theory shape.

Use this glossary as a lookup table while reading the source snapshots.

Do not read it as a separate dictionary. Each entry is deliberately anchored to
the codebase. If a definition sounds abstract, jump from the term to the Rust
syntax and then back to the chapter where the type or trait appears.

> Reader orientation:
> The glossary uses compact entries, but the entries still follow the book's
> main discipline: first the Rust handle, then the ML or software role, then the
> categorical shape.

## Category-Theory Terms

## Object

Rust syntax:

```text
TokenId
Vector
Logits
Distribution
Loss
Parameters
```

ML concept:

An object is one kind of value in the pipeline, such as a token, vector,
probability distribution, loss, or model state.

Category theory concept:

An object is something a morphism can start from or end at.

First-principles reading:

An object is the kind of thing an arrow is allowed to receive or return. In this
book, `TokenId` and `Vector` are different objects because the pipeline should
not confuse a vocabulary index with a dense numeric representation.

## Morphism

Rust syntax:

```rust,ignore
pub trait Morphism<Input, Output>
```

ML concept:

A morphism is one transformation stage, such as embedding lookup or softmax.

Category theory concept:

A morphism is a typed arrow:

```text
Input -> Output
```

## Identity Morphism

Rust syntax:

```rust,ignore
Identity<T>
```

ML concept:

Identity is a stage that leaves a value unchanged. It is useful for testing the
idea of neutral transformations.

Category theory concept:

Every object has an identity arrow:

```text
id_A : A -> A
```

## Composition

Rust syntax:

```rust,ignore
Compose<F, G, Middle>
```

ML concept:

Composition connects stages:

```text
Embedding then LinearToLogits then Softmax
```

Category theory concept:

If:

```text
f : A -> B
g : B -> C
```

then:

```text
g after f : A -> C
```

## Product Object

Rust syntax:

```rust,ignore
Product<A, B>
```

ML concept:

A product stores paired values, such as:

```text
input token x target token
prediction distribution x target token
```

Category theory concept:

The product object is written:

```text
A x B
```

Its projections correspond to `first()` and `second()`.

## Endomorphism

Rust syntax:

```rust,ignore
Endomorphism<T>
TrainStep : Parameters -> Parameters
```

ML concept:

A training step updates parameters and returns parameters again.

Category theory concept:

An endomorphism is an arrow from an object back to itself:

```text
A -> A
```

## Functor

Rust syntax:

```rust,ignore
Functor<A, B>
VecFunctor
OptionFunctor
```

ML concept:

Apply a transformation inside a wrapper such as a batch or optional value.

Category theory concept:

A functor maps objects and arrows while preserving structure.

## Functor Map

Rust syntax:

```rust,ignore
fn map<U>(self, f: impl Fn(T) -> U) -> Distribution<U>
```

ML concept:

For a probabilistic output, `map` transforms every possible outcome while
leaving the attached probabilities unchanged.

Category theory concept:

`map` lifts a deterministic function:

```text
T -> U
```

into a context-aware transformation:

```text
Distribution<T> -> Distribution<U>
```

## Natural Transformation

Rust syntax:

```rust,ignore
VecToFirstOption : Vec<A> -> Option<A>
```

ML concept:

Convert one container shape into another consistently, such as many candidates
to maybe one selected candidate.

Category theory concept:

A natural transformation converts one functor shape into another and commutes
with mapping.

## Monoid

Rust syntax:

```rust,ignore
PipelineTrace
Monoid::empty()
Monoid::combine()
```

ML concept:

Traces, logs, batches, and metric accumulators often need an empty value and a
combine operation.

Category theory concept:

A monoid has an identity element and an associative binary operation.

## Preorder

Rust syntax:

```rust,ignore
InformationLevel::can_flow_to
```

ML or software concept:

Information can flow from observation to feature to score to decision.

Category theory concept:

A preorder is reflexive and transitive.

## Galois Connection

Rust syntax:

```rust,ignore
abstract_to_layer_budget
concretize_layer_budget
```

ML or software concept:

Concrete feature counts and abstract layer budgets can be coordinated.

Category theory concept:

Two order-preserving views are connected by a law:

```text
abstract(x) <= y iff x <= concretize(y)
```

## Monoidal Preorder

Rust syntax:

```rust,ignore
ResourceBundle::tensor
ResourceBundle::can_supply
```

ML or software concept:

Independent compute and memory resources can be combined.

Category theory concept:

A preorder with a product-like composition operation that preserves order.

## Profunctor

Rust syntax:

```rust,ignore
FeasibilityRelation::relates(requirement, offer)
```

ML or software concept:

A requirement and implementation offer are related if constraints are
satisfied.

Category theory concept:

A profunctor generalizes a relationship between categories. This course uses a
small Bool-valued relation as the practical handle.

## Functorial Semantics

Rust syntax:

```rust,ignore
SignalMatrix::compose_after
```

ML or software concept:

Composed signal-flow stages should have the same meaning as composing their
matrix interpretations.

Category theory concept:

Interpretation preserves composition.

## Open System

Rust syntax:

```rust,ignore
OpenCircuit
OpenCircuit::then
OpenCircuit::parallel
```

ML or software concept:

A component has an external interface plus internal implementation details.

Category theory concept:

An open system composes through typed boundaries.

## Sheaf-Style Locality

Rust syntax:

```rust,ignore
SafetyCover::global_truth
```

ML or software concept:

Local safety checks over time intervals combine into a global safety result.

Category theory concept:

Local facts can determine a global fact when they glue coherently.

## Rust Terms

## Newtype

Rust syntax:

```rust,ignore
pub struct TokenId(usize);
```

ML concept:

The same raw number type can represent different concepts. Newtypes prevent
accidental mixing.

Category theory concept:

A newtype names a specific object instead of treating all raw representations
as the same object.

First-principles reading:

A newtype is the smallest move from "just data" to "data with a role." The
runtime representation can stay cheap, but the type checker now knows that a
token id, vocabulary size, and model dimension are not the same concept.

## Smart Constructor

Rust syntax:

```rust,ignore
pub fn new(value: Raw) -> CtResult<Self>
```

ML concept:

Invalid training inputs, probabilities, dimensions, or hyperparameters should
be rejected early.

Category theory concept:

A smart constructor maps raw data into a validated subobject, using `Result`
when the mapping can fail.

## Invariant

Rust syntax:

```text
Distribution must be non-empty, finite, non-negative, and sum to one.
```

ML concept:

The model can trust a value only if the type protects the rule that makes it
meaningful.

Category theory concept:

An invariant describes the subset or structure the object is meant to inhabit.

## Typed Error

Rust syntax:

```rust,ignore
CtError
CtResult<T>
```

ML concept:

Bad data should fail with a meaningful cause, not with a vague panic later.

Category theory concept:

`Result` turns a partial construction or morphism into a total error-aware
mapping.

## Machine-Learning Terms

## Token

Rust syntax:

```rust,ignore
TokenId
```

ML concept:

A token is a discrete symbol from a vocabulary.

Category theory concept:

The vocabulary is a finite discrete set of possible token objects.

## Embedding

Rust syntax:

```rust,ignore
Embedding : TokenId -> Vector
```

ML concept:

An embedding maps a discrete token to a dense numerical representation.

Category theory concept:

It is a morphism from a finite token object into a vector-space-like object.

## Logits

Rust syntax:

```rust,ignore
Logits(Vec<f32>)
```

ML concept:

Logits are raw scores before softmax.

Category theory concept:

They live in a vector-space-like object:

```text
R^vocab_size
```

## Softmax

Rust syntax:

```rust,ignore
Softmax : Logits -> Distribution
```

ML concept:

Softmax turns raw scores into probabilities.

Category theory concept:

It maps from a score vector into the probability simplex.

## Cross Entropy

Rust syntax:

```rust,ignore
CrossEntropy : Product<Distribution, TokenId> -> Loss
```

ML concept:

Cross entropy measures how much probability the model assigned to the correct
target.

Category theory concept:

It is a morphism from prediction-target product into non-negative scalar loss.

## Parameters

Rust syntax:

```rust,ignore
Parameters
```

ML concept:

The trainable state of the model: embedding table, output head, and bias.

Category theory concept:

The object transformed by the training endomorphism.

## Gradient

Rust syntax:

```rust,ignore
LocalGradient
grad_embedding
grad_lm_head
grad_bias
```

ML concept:

A gradient tells how parameters should change to reduce loss.

Category theory concept:

Gradient flow is local derivative information composed backward through a
composed computation.

## Learning Rate

Rust syntax:

```rust,ignore
LearningRate
```

ML concept:

The scalar step size in gradient descent.

Category theory concept:

It chooses a specific update morphism from a family of parameter endomorphisms.

## Chain Rule

Rust syntax:

```rust,ignore
MulOp::backward
```

ML concept:

The chain rule lets local derivatives combine into gradients for a larger
computation.

Category theory concept:

It is composition of local derivative maps.

## Where This Leaves Us

The glossary is not a substitute for the chapters. It is the index of the
book's repeated translation habit. When a term feels unfamiliar, connect it back
to one of three things: the Rust syntax that names it, the ML or software role
that motivates it, and the categorical shape that explains how it composes.
