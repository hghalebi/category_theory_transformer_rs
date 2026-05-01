# Repository Source Snapshots

This appendix collects the learner-facing source files used throughout the
course. The Rust snapshots are complete files, so they are marked `ignore` as
standalone snippets. The real source files are still validated by the repository
checks.

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

### `src/demo.rs`

```rust,ignore
{{#include ../../src/demo.rs}}
```

### `src/bin/category_ml.rs`

```rust,ignore
{{#include ../../src/bin/category_ml.rs}}
```

## Runnable Examples

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
