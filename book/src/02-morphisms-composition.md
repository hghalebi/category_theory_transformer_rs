# Morphism and Composition

## Mental Model

A morphism is a typed arrow:

```text
Input -> Output
```

In Rust, this repo models that with `Morphism<Input, Output>`.

## Read This File

Open [`src/category.rs`](../../src/category.rs).

Read in this order:

1. `Morphism`
2. `Identity`
3. `Compose`
4. `Endomorphism`

## Run the Example

```bash
cargo run --example 02_morphism_composition
```

## What to Notice

The prediction pipeline is built from small arrows:

```text
TokenId -> Vector -> Logits -> Distribution
```

`Compose` is the glue. If the middle types do not match, Rust rejects the
program before it runs.

## Checkpoint

Why is `TokenId -> Vector -> Logits` easier to debug than one giant
`predict(...)` function?
