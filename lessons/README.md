# Category Theory for Tiny ML: Lesson Path

This folder is the slow, friendly path through the codebase.

Use it in order. Each lesson is intentionally short and points to a real Rust
file that `cargo` checks.

## The Learning Loop

For each lesson:

1. Read the mental model.
2. Open the named Rust module.
3. Run the named example.
4. Answer the checkpoint before moving on.

## Lessons

1. [Map of the Course](00-map.md)
2. [Domain Objects](01-domain-objects.md)
3. [Morphism and Composition](02-morphisms-composition.md)
4. [The Tiny ML Pipeline](03-ml-pipeline.md)
5. [Training as an Endomorphism](04-training-endomorphism.md)
6. [Functors, Naturality, Monoids, and Chain Rule](05-structure-and-calculus.md)

## Validation

Run the full check:

```bash
bash scripts/check.sh
```

Run one lesson example:

```bash
cargo run --example 03_training_endomorphism
```
