# Exercises

These exercises are intentionally small. Each one changes one idea at a time.

## Exercise 1: Add a Token

Use the `src/demo.rs` snapshot in [Course Map](00-map.md).

Add one new vocabulary item and extend the token sequence.

Run:

```bash
cargo run --bin category_ml
```

Pass condition:

- the demo still runs
- the dataset windowing output includes your new transition

## Exercise 2: Break a Composition

Use the `examples/02_morphism_composition.rs` snapshot in
[Morphism and Composition](02-morphisms-composition.md).

Try to compose `Embedding` directly with `Softmax`.

Expected failure:

```text
the trait bound ... is not satisfied
```

Then restore the working version.

Lesson:

> Composition is only legal when the output type of one arrow matches the input
> type of the next arrow.

## Exercise 3: Change the Training Repetition Count

Use the `examples/03_training_endomorphism.rs` snapshot in
[Training as an Endomorphism](04-training-endomorphism.md).

Change `StepCount::new(80)` to:

- `StepCount::new(1)`
- `StepCount::new(10)`
- `StepCount::new(200)`

Run:

```bash
cargo run --example 03_training_endomorphism
```

Watch how the final loss changes.

## Exercise 4: Explain One Boundary

Pick one constructor:

- `Distribution::new`
- `Loss::new`
- `LearningRate::new`
- `TrainingSet::new`

Write one sentence explaining what invalid state it prevents.

## Exercise 5: Trace a Full Source File

Use [Repository Source Snapshots](source-snapshots.md).

Pick one complete source file and write a three-sentence summary:

1. What object or arrow does this file define?
2. Which invariant does the file protect?
3. Which command proves the file still works?

## Exercise 6: Connect One External Reference

Use [References](references.md).

Pick one external resource and connect it to one source file in this course:

1. Which idea does the external resource explain?
2. Which Rust file implements a tiny version of that idea?
3. What is one difference between the full external treatment and this tiny implementation?

## Exercise 7: Test One Sketch Law

Use [Seven Sketches Through Rust](seven-sketches-rust.md).

Pick one law from `src/sketches.rs`:

- preorder laws
- feature/layer Galois law
- resource monotonicity
- foreign-key resolution
- signal-flow matrix composition
- local-to-global safety truth

Change one input in `examples/05_seven_sketches.rs`, then run:

```bash
cargo run --example 05_seven_sketches
```

Pass condition:

- you can explain which law still holds
- you can explain which constructor or method prevents invalid structure
