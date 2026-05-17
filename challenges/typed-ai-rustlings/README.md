# Typed AI Rustlings

Hook:

```text
Learn AI by fixing compiler errors.
```

This challenge is for Rust learners, ML engineers, and mathematically curious
programmers who want AI concepts to become concrete compiler feedback.

Each exercise starts from one broken boundary:

```text
raw value -> compiler error -> stronger type -> ML meaning
```

The seed pack currently teaches two boundaries:

```text
usize is not TokenId
Logits are not Distribution
```

## Current Exercises

| Exercise | Broken idea | Fix |
| --- | --- | --- |
| `token_id_not_usize` | a token id is not a raw `usize` | construct `TokenId` before crossing the lookup boundary |
| `logits_are_not_probabilities` | logits are not probabilities | run `Softmax` before `CrossEntropy` |

## How To Use This Seed Pack

The files in `exercises/` are intentionally broken. Read one file, repair the
typed boundary, then compare with the matching file in `solutions/`.

For now, validate the reference solutions through the repository tests:

```bash
cargo test --test challenge_typed_ai
```

When this grows into a standalone Rustlings pack, the same exercises will become
the first two compiler-fix cards.

## Share Line

```text
I fixed an AI concept by making the compiler reject the wrong boundary.
```
