# Category Theory for Tiny ML in Rust

This book teaches category-theory ideas through a small Rust machine-learning
pipeline.

The goal is not to memorize abstract words. The goal is to connect each word to
working Rust code.

## How to Use This Book

Use this loop:

1. Read one short chapter.
2. Open the linked Rust file.
3. Run the command in that chapter.
4. Answer the checkpoint.
5. Move on only when the code and the idea both make sense.

## Fast Start

From the repository root:

```bash
cargo run --bin category_ml
```

Build this book:

```bash
bash scripts/build-mdbook.sh
```

Open the generated book at:

```text
book/html/index.html
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
