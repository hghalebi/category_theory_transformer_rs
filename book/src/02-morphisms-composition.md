# Morphism and Composition

## Mental Model

A morphism is a typed arrow:

```text
Input -> Output
```

In Rust, this repo models that with `Morphism<Input, Output>`.

The ML intuition is that a model is not one magic operation. It is a sequence:
lookup an embedding, project it to logits, normalize logits into probabilities.
Each step has a clear input type and output type.

The category-theory intuition is that composition is only legal when the middle
types match.

## Source Snapshot

This file defines the typed arrow interface and the composition adapter.

Read in this order:

1. `Morphism`
2. `Identity`
3. `Compose`
4. `Endomorphism`

<details>
<summary>Source snapshot: src/category.rs</summary>

```rust,ignore
{{#include ../../src/category.rs}}
```

</details>

## Code Walkthrough

`Morphism<Input, Output>` is the contract for one typed transformation.

`Identity<T>` is the arrow that returns the same value. It is useful because
identity is what lets composition have a neutral element.

`Compose<F, G, Middle>` stores two arrows. The compiler checks that `F` returns
`Middle` and `G` accepts `Middle`. If that bridge type is wrong, the program
does not compile.

`Endomorphism<T>` is an arrow from `T` back to `T`. Training uses this shape in
the next chapter.

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

## Runnable Example Snapshot

<details>
<summary>Source snapshot: examples/02_morphism_composition.rs</summary>

```rust,ignore
{{#include ../../examples/02_morphism_composition.rs}}
```

</details>

## Checkpoint

Why is `TokenId -> Vector -> Logits` easier to debug than one giant
`predict(...)` function?

## Further Reading

- [Glossary](glossary.md): morphism, identity morphism, composition, endomorphism
- [References](references.md): applied category theory and Rust module structure
