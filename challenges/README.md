# Public Challenges

This directory turns the book into public, shareable practice.

The rule for both tracks is:

```text
broken idea -> typed Rust boundary -> checked invariant -> shareable lesson
```

## Typed AI Rustlings

[Typed AI Rustlings](typed-ai-rustlings/README.md) is a rustlings-style exercise
pack.

Hook:

```text
Learn AI by fixing compiler errors.
```

The exercise files under `typed-ai-rustlings/exercises/` are intentionally
broken and are not compiled by the normal Cargo gate. The reference solutions
under `typed-ai-rustlings/solutions/` are compiled through integration tests.

## Paper-To-Rust

[Paper-To-Rust](paper-to-rust/README.md) turns one idea from one ML or
category-theory paper into executable Rust.

Hook:

```text
Stop summarizing papers. Compile them.
```

The seed challenge is [Adam optimizer state](paper-to-rust/papers/adam/README.md).

## Challenge Evidence And Reader Evidence

A challenge completion proves that one reader crossed one executable boundary.
It becomes useful textbook feedback when it also names the first unclear point
and the smallest fix that would have helped.

Use a challenge completion issue for:

```text
I ran the challenge.
This output, compiler error, or test was the evidence.
This AI boundary became clearer.
This was the first unclear point, or none.
This small edit would help the next reader.
```

If the challenge exposed a chapter, exercise, or command that blocked learning,
also open the closest reader report from [REVIEWERS.md](../REVIEWERS.md). The
strict textbook completion gate counts accepted direct-reader reports, not broad
challenge enthusiasm.

## Validation

Run:

```bash
cargo fmt --check
cargo test --test challenge_typed_ai
cargo test --all-targets --all-features
cargo run --example challenge_adam
mdbook build
mdbook test
```

The repo intentionally does not track a public `scripts/` directory. Use the
explicit commands above for now.
