# Glossary

## Category-Theory Terms

**Object**

A typed thing an arrow can start from or end at. In this course, `TokenId`,
`Vector`, `Logits`, `Distribution`, `Loss`, and `Parameters` are the main
objects.

**Morphism**

A typed transformation from one object to another. In Rust, this course models
that shape with `Morphism<Input, Output>`.

**Identity Morphism**

The arrow that returns its input unchanged. It matters because composition needs
a neutral arrow.

**Composition**

Connecting two arrows when the output type of the first arrow is exactly the
input type of the second arrow.

**Product Object**

A pair of objects. `Product<A, B>` is used for `(Distribution, TokenId)` during
loss calculation and `(TokenId, TokenId)` during training-set construction.

**Endomorphism**

An arrow from a type back to the same type. Training has this shape because one
step maps `Parameters -> Parameters`.

**Functor**

A structure-preserving map over a wrapper. `VecFunctor` and `OptionFunctor`
show the same idea: keep the wrapper shape and transform the inside.

**Natural Transformation**

A consistent conversion from one wrapper shape to another. `VecToFirstOption`
maps `Vec<A>` to `Option<A>` without depending on what `A` is.

**Monoid**

A type with an empty value and an associative combine operation.
`PipelineTrace` uses this shape for combining trace steps.

## Rust Terms

**Newtype**

A small wrapper type around a lower-level representation. `TokenId(usize)` keeps
a token index from being confused with a loop index or a vocabulary size.

**Smart Constructor**

A constructor that validates invariants before a value enters the rest of the
program. `Distribution::new` rejects invalid probability vectors.

**Invariant**

A rule that must stay true for a type to make sense. A `Distribution` must be
non-empty, finite, non-negative, and sum to one.

**Typed Error**

An explicit error enum that names failure cases. This crate uses `CtError`
instead of returning plain strings.

## Machine-Learning Terms

**Token**

A discrete symbol. In this course, a token is represented by `TokenId`.

**Embedding**

A learned vector representation for a token.

**Logits**

Raw model scores before normalization.

**Softmax**

The operation that converts logits into a probability distribution.

**Cross Entropy**

The loss used to measure how surprised the model is by the target token.

**Parameters**

The trainable state of the model: embedding table, linear head, and bias.

**Gradient**

The direction and scale of change used to update parameters.

**Learning Rate**

The step size used when moving parameters along a gradient.

**Chain Rule**

The rule that lets local derivatives compose into a derivative for a larger
calculation.
