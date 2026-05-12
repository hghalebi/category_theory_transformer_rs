# Rust Path

Use this path if you know Rust syntax and want to see how types can carry ML
meaning.

The goal is not to learn Rust from scratch. The goal is to use familiar Rust
tools such as structs, constructors, traits, `Result`, and tests to make a tiny
ML pipeline harder to misuse.

## First Pass

Run these commands before reading deeply:

```bash
cargo run --example 01_domain_objects
cargo run --example 02_morphism_composition
cargo test domain::tests --lib
cargo test category::tests --lib
```

Then inspect in this order:

1. [src/domain.rs](../src/domain.rs)
2. [src/error.rs](../src/error.rs)
3. [src/category.rs](../src/category.rs)
4. [src/ml.rs](../src/ml.rs)
5. [Morphism and Composition](../book/src/02-morphisms-composition.md)

The first reading should answer one question:

```text
Which invalid ML state does this Rust boundary make harder to express?
```

## Core Questions

- Why is `TokenId` better than passing a raw `usize` through the public API?
- Which constructors reject invalid state?
- How does `Morphism<Input, Output>` make composition visible?
- Which errors are recoverable in the tutorial pipeline?
- Which type parameter is the middle object in `Compose<F, G, Middle>`?
- Which test would fail if an invariant or composition law were weakened?

## Rust Boundary Map

| Rust feature | Local file | Teaching role |
| --- | --- | --- |
| named structs | `src/domain.rs` | separate `TokenId`, `Vector`, `Logits`, `Distribution`, and `Loss` |
| validating constructors | `src/domain.rs` | reject empty sequences, invalid probabilities, and invalid rates |
| typed errors | `src/error.rs` | keep recoverable failures explicit |
| traits | `src/category.rs` | give transformations a shared interface |
| generic composition | `src/category.rs` | require the first target type to match the second source type |
| unit tests | `src/domain.rs`, `src/category.rs`, `src/ml.rs` | check invariants, laws, and pipeline behavior |

## Active Practice

After running `cargo run --example 02_morphism_composition`, explain these two
lines from the output:

```text
Embedding then LinearToLogits is legal because Vector == Vector
Embedding then Softmax is illegal because Vector != Logits
```

A strong Rust answer should name:

- the first implementation: `Morphism<TokenId, Vector> for Embedding`,
- the second implementation: `Morphism<Vector, Logits> for LinearToLogits`,
- the rejected shortcut: `Softmax` expects `Logits`, not `Vector`,
- the generic bridge: `Compose::<_, _, Vector>` or `Compose::<_, _, Logits>`.

Then try Exercise 4 in [Exercises](../book/src/exercises.md#exercise-4-break-a-composition).

## Common Rust Misreadings

| Misreading | What to say instead |
| --- | --- |
| tuple structs are only cosmetic | wrappers separate domain roles and can hide raw representation |
| every constructor should return `Self` | constructors return `Result` when they enforce an invariant |
| `Morphism<Input, Output>` is just a function pointer | it is a named fallible transformation with a shared composition API |
| `Compose` proves all category laws | it models the typed composition shape used by the examples |
| errors are distractions from the lesson | errors are evidence that a boundary rejected an invalid state |

## Done Criteria

You can point to one type, one invariant, one morphism, and one test that
protects the teaching model.

You can also explain why weakening a type signature is usually the wrong fix
for a failed composition. The right repair is to restore the missing middle
object or the morphism that produces it.

## Feedback

Open the [chapter clarity feedback form](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=chapter-clarity.yml)
when a Rust type, trait bound, constructor, or test stops explaining the ML
idea.

Use this shape:

```text
Perspective: Rust engineer
Command or page tried:
Rust boundary that became unclear:
ML role I expected it to explain:
What would have helped:
```
