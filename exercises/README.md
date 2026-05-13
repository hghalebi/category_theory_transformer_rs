# Exercises

This directory contains the compact practice ladder for the book.

Use the files in this order:

1. `beginner/README.md`
2. `intermediate/README.md`
3. `advanced/README.md`

## Standalone Attempt Checklist

Use this checklist when you are practicing from the `exercises/` directory
without the book open.

Before checking the answer key, write down:

```text
Exercise:
File inspected:
Command run:
Output, test, or compiler signal:
Rust object or boundary:
ML or software role:
Category-theory shape:
One-line answer:
```

The answer does not need formal language at first. It does need evidence. A
good attempt points to one command output line, test name, constructor,
compiler error, type, or function signature.

If you cannot fill in `Output, test, or compiler signal`, rerun the exercise
command or choose a smaller exercise. If you cannot fill in `Rust object or
boundary`, return to the matching chapter before reading the answer key.

## Standalone Command Map

Use this map when you know the topic you want to practice but not the command
that gives useful evidence.

| Practice focus | Command | Signal to inspect |
| --- | --- | --- |
| first typed pipeline | `cargo run --example 01_token_sequence` | `Text -> TokenSequence -> TrainingPairs` |
| domain objects | `cargo run --example 01_domain_objects` | `TokenSequence`, `TrainingSet`, and `TrainingExample = Product<TokenId, TokenId>` |
| composition | `cargo run --example 02_morphism_composition` | `Vector == Vector` is legal; `Vector != Logits` is illegal |
| training update | `cargo run --example 03_training_endomorphism` | `TrainStep : Parameters -> Parameters` and loss before/after |
| structure and laws | `cargo run --example 04_structure_and_calculus` | functor output, naturality square, monoid trace, and local derivatives |
| seven sketches | `cargo run --example 05_seven_sketches` | relation, order, schema, circuit, and cover checks |
| attention shapes | `cargo run --example 06_attention_scores` | query/key/value scores, masks, weights, residuals, normalization, and blocks |
| training state | `cargo run --example 07_transformer_training_state` | every update returns `TransformerTrainingState -> TransformerTrainingState` |
| full Rust check | `cargo test --all-targets --all-features` | all examples and tests still compile and pass |

If a command produces many lines, choose one line as your evidence before
reading the answer key. The goal is not to run everything; the goal is to
connect one visible signal to one Rust boundary and one concept.

After attempting an exercise, use `ANSWER_KEY.md` to compare reasoning. The
answer key is written as facilitator guidance, so it focuses on expected
conceptual shape rather than exact phrasing.

If an exercise is unclear, fill out the attempt-record template in the book's
Exercises chapter before opening feedback. A useful report names the exercise,
the command run, the first failure signal, the confusing line or concept, and
the answer-key mismatch.

Then open an
[exercise clarity report](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=chapter-clarity.yml&title=%5Bgood+first+feedback%5D+exercise+attempt+brief&location=book%2Fsrc%2Fexercises.md+or+exercises%2FREADME.md&command=exercise+command+tried).
The link fills the route, not the evidence; the evidence signal should come
from what you personally read, ran, or attempted.

Exercise 15 is the mixed transfer check. Use it after the chapter-local
exercises to practice diagnosing invariant, composition, endomorphism, shape,
and local-to-global boundaries without chapter hints.

The advanced track now includes a finite-difference gradient-checking exercise
that matches the tiny Transformer training tests in `src/attention.rs`.

Run the full validation gate before submitting exercise changes:

```bash
cargo test --all-targets --all-features
```
