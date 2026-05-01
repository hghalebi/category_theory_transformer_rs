# Functors, Naturality, Monoids, and Chain Rule

## Mental Model

This chapter gives names to patterns you already use:

- `Functor`: map inside a wrapper without changing the wrapper shape
- `NaturalTransformation`: convert one wrapper shape to another in a consistent way
- `Monoid`: an empty value plus an associative combine operation
- Chain rule: local gradients compose into larger gradients

The ML intuition is that training systems are full of structure-preserving
moves:

- map a transformation over a batch
- convert one container shape into another consistently
- combine trace steps into a larger trace
- compose local derivatives into a global derivative

The category-theory names let those moves become inspectable instead of
mysterious.

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

## Code Walkthrough

`VecFunctor` and `OptionFunctor` show the same operation shape: keep the wrapper
and transform the inside.

`VecToFirstOption` is a natural transformation because it changes wrapper shape
from `Vec<A>` to `Option<A>` without caring what `A` is.

`PipelineTrace` is a monoid because it has an empty trace and an associative
combine operation.

`MulOp` shows the smallest possible backpropagation pattern. The forward pass
computes `z = x * y`; the backward pass composes the upstream gradient with the
local derivatives.

## Run the Example

```bash
cargo run --example 04_structure_and_calculus
```

## What to Notice

`VecFunctor` and `OptionFunctor` are not trying to replace real Rust APIs.
They show the shape of the idea:

```text
keep the container, transform the inside
```

`PipelineTrace` is a monoid because:

- there is an empty trace
- traces can be combined
- grouping does not change the final trace

`MulOp` shows the smallest useful backward pass:

```text
z = x * y
dL/dx = dL/dz * y
dL/dy = dL/dz * x
```

## Checkpoint

Why is "local rule plus composition" the core idea behind backpropagation?

## Runnable Example Snapshot

<details>
<summary>Source snapshot: examples/04_structure_and_calculus.rs</summary>

```rust,ignore
{{#include ../../examples/04_structure_and_calculus.rs}}
```

</details>

## Further Reading

- [Glossary](glossary.md): functor, natural transformation, monoid, chain rule
- [References](references.md): applied category theory, deep learning math, and attention
