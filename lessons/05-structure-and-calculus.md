# 05 - Functors, Naturality, Monoids, and Chain Rule

## Mental Model

This lesson gives names to patterns you already use:

- `Functor`: map inside a wrapper without changing the wrapper shape
- `NaturalTransformation`: convert one wrapper shape to another in a consistent way
- `Monoid`: an empty value plus an associative combine operation
- Chain rule: local gradients compose into larger gradients

## Read These Files

Open:

- `src/structure.rs`
- `src/calculus.rs`

## Run the Example

```bash
cargo run --example 04_structure_and_calculus
```

## What to Notice

The examples are tiny on purpose.

`VecFunctor` and `OptionFunctor` are not trying to replace real Rust APIs. They
show the shape of the idea:

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
