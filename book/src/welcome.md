# Category Theory for Tiny ML in Rust

This course teaches category-theory ideas through a small Rust machine-learning
pipeline. The important Rust files and runnable examples are included as source
snapshots so the concept and implementation stay together.

The goal is not to memorize abstract words. The goal is to connect each word to
working Rust code.

## Learning Contract

Every chapter follows the same loop:

1. Start with the concrete ML move.
2. Name the category-theory shape only after the example is visible.
3. Read the exact Rust source snapshot included in the chapter.
4. Run the command from the chapter.
5. Use the checkpoint to explain the idea in your own words.

The style is intentionally example-first. When a term is abstract, the chapter
pulls it back to a tiny next-token model and a specific Rust type.

## How to Study

Use this loop:

1. Read one short chapter.
2. Study the included source snapshot.
3. Run the command in that chapter.
4. Answer the checkpoint.
5. Move on only when the code and the idea both make sense.

## Fast Start

From the repository root:

```bash
cargo run --bin category_ml
```

## Mental Picture

The tiny model is a chain of typed arrows:

```text
TokenId -> Vector -> Logits -> Distribution
Distribution x TokenId -> Loss
Parameters -> Parameters
```

Rust checks that the arrows connect.

Category theory gives names to the shapes.

## Reading Path

Read the chapters in order. Each one adds one idea:

- [Course Map](00-map.md): the whole pipeline shape
- [Domain Objects](01-domain-objects.md): typed nouns
- [Morphism and Composition](02-morphisms-composition.md): typed arrows
- [The Tiny ML Pipeline](03-ml-pipeline.md): prediction and loss
- [Training as an Endomorphism](04-training-endomorphism.md): repeated updates
- [Functors, Naturality, Monoids, and Chain Rule](05-structure-and-calculus.md): reusable structure
- [Glossary](glossary.md): short definitions
- [References](references.md): credible external resources
- [Transformer Roadmap](roadmap.md): how this foundation points toward attention
