# Beginner Exercises

These exercises are for first-time readers.

Use this loop:

```text
run -> change one thing -> run again -> explain the difference
```

Keep answers short. A good answer names the Rust syntax, the ML or software
idea, and the category-theory shape only as far as the code supports it.

## Exercise 1: Change The Input Text

Edit `examples/01_token_sequence.rs` and change the input sentence.

Run:

```bash
cargo run --example 01_token_sequence
```

Explain which token ids changed and why the training-pair shape stayed the
same.

Expected feedback:

The exact token ids may change. The shape should remain:

```text
Text -> TokenSequence -> TrainingPairs
```

If no training pairs appear, check whether the input has at least two tokens.

## Exercise 2: Find One Invariant

Open `src/domain.rs` and find one constructor that rejects invalid input.

Write one sentence:

```text
This constructor prevents ...
```

Useful candidates:

- `Distribution::new`
- `Loss::new`
- `LearningRate::new`
- `TrainingSet::new`

Pass condition:

You can name the raw value and the invalid state that the constructor rejects.

## Exercise 3: Explain One Output Line

Run:

```bash
cargo run --bin category_ml
```

Choose one output line and explain:

```text
Rust syntax:
which type or function produced it?

ML concept:
what learning step does it represent?

Category theory concept:
what object or transformation does it show?
```
