# Intermediate Exercises

These exercises are for readers who can already run the examples and read the
source files.

Use this loop:

```text
read the existing pattern -> add one typed step -> test the boundary
```

Do not add a large abstraction. The goal is to extend the teaching system by
one small, inspectable transformation.

## Exercise 1: Add A Morphism

Add a small transformation that maps one typed tutorial value to another.

Requirements:

- define the input and output types
- implement `Morphism<Input, Output>`
- add a runnable example or test

Pass condition:

The new transformation can be explained with this shape:

```text
Input object -> named transformation -> output object
```

Validation:

```bash
cargo test --all-targets --all-features
```

## Exercise 2: Explain A Composition Failure

Try to compose two transformations whose types do not match.

Write down:

- the compiler error
- the missing middle type
- the corrected composition path

Do not solve the exercise by weakening types. The intended lesson is that a
failed composition points to a missing stage.

## Exercise 3: Add One Negative Test

Choose a constructor or composition method that rejects invalid structure.

Add one test that proves the rejection happens.

Good candidates:

- `Distribution::new` rejects probabilities that do not sum to one
- `TrainingSet::new` rejects an empty dataset
- `SignalMatrix::compose_after` rejects mismatched dimensions

Pass condition:

The test fails when the validation branch is removed and passes when the branch
is present.
