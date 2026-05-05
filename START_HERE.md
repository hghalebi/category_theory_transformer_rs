# Start Here

This path is designed for a first visit. It should take about five minutes to
get a real result and about thirty minutes to understand the shape of the
project.

## The First Win

Run:

```bash
cargo run --example 01_token_sequence
```

You should see raw text become a `TokenSequence`, then adjacent next-token
training pairs. That is the smallest version of the project:

```text
Text -> TokenSequence -> TrainingPairs
```

The point is not tokenization quality. The point is that the pipeline is
explicit, typed, and inspectable.

## The First Reading Loop

1. Read [book/src/welcome.md](book/src/welcome.md).
2. Read [book/src/00-map.md](book/src/00-map.md).
3. Run `cargo run --example 01_domain_objects`.
4. Open [book/src/exercises.md](book/src/exercises.md) and do Exercise 1.
5. If something is unclear, open a reader-confusion issue.

## The First Code Loop

After the first example, run:

```bash
cargo run --example 02_morphism_composition
cargo run --example 03_training_endomorphism
```

These show the two core ideas:

```text
typed transformations compose
training is repeated model-state update
```

## Choose Your Path

- New to ML internals: [docs/ml-path.md](docs/ml-path.md)
- New to Rust architecture: [docs/rust-path.md](docs/rust-path.md)
- New to category theory: [docs/category-theory-path.md](docs/category-theory-path.md)
- Ready to contribute: [CONTRIBUTING.md](CONTRIBUTING.md)

## Done Criteria

You are ready for the rest of the book when you can explain this line:

```text
TokenId -> Vector -> Logits -> Distribution
```

You do not need advanced math yet. You need the intuition that each arrow has a
typed input, a typed output, and a reason it can compose with the next arrow.
