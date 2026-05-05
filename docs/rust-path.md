# Rust Path

Use this path if you know Rust syntax and want to see how types can carry ML
meaning.

## Order

1. Read [src/domain.rs](../src/domain.rs).
2. Run `cargo run --example 01_domain_objects`.
3. Read [src/category.rs](../src/category.rs).
4. Run `cargo run --example 02_morphism_composition`.
5. Read [src/error.rs](../src/error.rs).

## Core Questions

- Why is `TokenId` better than passing a raw `usize` through the public API?
- Which constructors reject invalid state?
- How does `Morphism<Input, Output>` make composition visible?
- Which errors are recoverable in the tutorial pipeline?

## Done Criteria

You can point to one type, one invariant, one morphism, and one test that
protects the teaching model.
