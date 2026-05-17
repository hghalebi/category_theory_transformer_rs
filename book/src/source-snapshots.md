# Repository Source Snapshots

This appendix collects the learner-facing source files used throughout the
course. The Rust snapshots are complete files, so they are marked `ignore` as
standalone snippets. The real source files are still validated by the repository
checks.

The problem this appendix solves is:

> The explanatory chapters should never drift away from the real code.

Use this appendix as the exact-code lookup layer.

When you inspect any block here, use the same reading pattern as the chapters:

```text
Rust syntax:
what does the code literally declare or execute?

ML or software concept:
why does this block exist in the pipeline or system model?

Category theory concept:
what object, morphism, product, composition, endomorphism, or law does it
represent?
```

The appendix is intentionally less narrative than the chapters. It keeps the
full source available in one place so you can verify every explanation against
the code itself.

## How To Navigate The Snapshots

Use the snapshots in two directions.

When reading chapter-first, start with the explanation, then open the matching
snapshot to verify the exact code. When reading code-first, start with the file
that interests you, then return to the chapter that teaches its role.

| If you want to inspect | Start with | Then read |
| --- | --- | --- |
| public crate surface | `src/lib.rs` | [Course Map](00-map.md) |
| typed values and invariants | `src/domain.rs` | [Domain Objects](01-domain-objects.md) |
| arrows and composition | `src/category.rs` | [Morphism and Composition](02-morphisms-composition.md) |
| prediction pipeline | `src/ml.rs` | [The Tiny ML Pipeline](03-ml-pipeline.md) |
| parameter updates | `src/training.rs` | [Training as an Endomorphism](04-training-endomorphism.md) |
| reusable structure | `src/structure.rs` | [Functors, Naturality, Monoids, and Chain Rule](05-structure-and-calculus.md) |
| local derivative flow | `src/calculus.rs` | [Functors, Naturality, Monoids, and Chain Rule](05-structure-and-calculus.md) |
| typed masked attention, value-mixing, head-concatenation, output-projection, residual, normalization, feed-forward, positional, hidden-projection, single-head block, multi-head block, masked-block, readout, parameter-object, training-state, readout-training, local feed-forward training, and composed block-training boundaries with query/key/value gradients | `src/attention.rs` | [Transformer Roadmap](roadmap.md) |
| applied category-theory sketches | `src/sketches.rs` | [Seven Sketches Through Rust](seven-sketches-rust.md) |
| public challenge reference behavior | `src/challenges/` and `examples/challenge_adam.rs` | [Challenges](challenges.md) |
| runnable end-to-end walkthrough | `src/demo.rs` | [Course Map](00-map.md) |
| command-line entrypoint | `src/bin/category_ml.rs` | [Course Map](00-map.md) |

The useful question is:

```text
Which explanation in the book would become false if this source file changed?
```

That question is why the source snapshots exist. They make drift visible.

## Reading Order By Goal

For a five-minute run, inspect `examples/01_token_sequence.rs`, then compare it
with the beginning of [Course Map](00-map.md).

For the core book path, read `src/domain.rs`, `src/category.rs`, `src/ml.rs`,
and `src/training.rs` in that order.

For the structure path, read `src/structure.rs`, `src/calculus.rs`,
`src/attention.rs`, and `src/sketches.rs` after the tiny ML pipeline is clear.

For contribution work, read the chapter first, then the source file, then the
tests in the same module. The tests often explain the contract more clearly
than the implementation alone.

## Rust Library Surface

### `src/lib.rs`

```rust,ignore
{{#include ../../src/lib.rs}}
```

### `src/error.rs`

```rust,ignore
{{#include ../../src/error.rs}}
```

### `src/domain.rs`

```rust,ignore
{{#include ../../src/domain.rs}}
```

### `src/category.rs`

```rust,ignore
{{#include ../../src/category.rs}}
```

### `src/ml.rs`

```rust,ignore
{{#include ../../src/ml.rs}}
```

### `src/training.rs`

```rust,ignore
{{#include ../../src/training.rs}}
```

### `src/structure.rs`

```rust,ignore
{{#include ../../src/structure.rs}}
```

### `src/calculus.rs`

```rust,ignore
{{#include ../../src/calculus.rs}}
```

### `src/attention.rs`

```rust,ignore
{{#include ../../src/attention.rs}}
```

### `src/sketches.rs`

```rust,ignore
{{#include ../../src/sketches.rs}}
```

### `src/challenges/mod.rs`

```rust,ignore
{{#include ../../src/challenges/mod.rs}}
```

### `src/challenges/typed_ai.rs`

```rust,ignore
{{#include ../../src/challenges/typed_ai.rs}}
```

### `src/challenges/papers/mod.rs`

```rust,ignore
{{#include ../../src/challenges/papers/mod.rs}}
```

### `src/challenges/papers/adam.rs`

```rust,ignore
{{#include ../../src/challenges/papers/adam.rs}}
```

### `src/demo.rs`

```rust,ignore
{{#include ../../src/demo.rs}}
```

### `src/bin/category_ml.rs`

```rust,ignore
{{#include ../../src/bin/category_ml.rs}}
```

## Runnable Examples

### `examples/01_token_sequence.rs`

```rust,ignore
{{#include ../../examples/01_token_sequence.rs}}
```

### `examples/01_domain_objects.rs`

```rust,ignore
{{#include ../../examples/01_domain_objects.rs}}
```

### `examples/02_morphism_composition.rs`

```rust,ignore
{{#include ../../examples/02_morphism_composition.rs}}
```

### `examples/03_training_endomorphism.rs`

```rust,ignore
{{#include ../../examples/03_training_endomorphism.rs}}
```

### `examples/04_structure_and_calculus.rs`

```rust,ignore
{{#include ../../examples/04_structure_and_calculus.rs}}
```

### `examples/05_seven_sketches.rs`

```rust,ignore
{{#include ../../examples/05_seven_sketches.rs}}
```

### `examples/06_attention_scores.rs`

```rust,ignore
{{#include ../../examples/06_attention_scores.rs}}
```

### `examples/07_transformer_training_state.rs`

```rust,ignore
{{#include ../../examples/07_transformer_training_state.rs}}
```

### `examples/challenge_adam.rs`

```rust,ignore
{{#include ../../examples/challenge_adam.rs}}
```

## Project Configuration

### `Cargo.toml`

```toml
{{#include ../../Cargo.toml}}
```

## Companion Lesson Notes

These are the shorter markdown notes kept under `lessons/`.

### `lessons/README.md`

````md
{{#include ../../lessons/README.md}}
````

### `lessons/00-map.md`

````md
{{#include ../../lessons/00-map.md}}
````

### `lessons/01-domain-objects.md`

````md
{{#include ../../lessons/01-domain-objects.md}}
````

### `lessons/02-morphisms-composition.md`

````md
{{#include ../../lessons/02-morphisms-composition.md}}
````

### `lessons/03-ml-pipeline.md`

````md
{{#include ../../lessons/03-ml-pipeline.md}}
````

### `lessons/04-training-endomorphism.md`

````md
{{#include ../../lessons/04-training-endomorphism.md}}
````

### `lessons/05-structure-and-calculus.md`

````md
{{#include ../../lessons/05-structure-and-calculus.md}}
````

### `lessons/06-seven-sketches.md`

````md
{{#include ../../lessons/06-seven-sketches.md}}
````
