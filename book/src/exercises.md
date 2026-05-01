# Exercises

These exercises are intentionally small. Each one changes one idea at a time.

## Exercise 1: Add a Token

Open [`src/demo.rs`](../../src/demo.rs).

Add one new vocabulary item and extend the token sequence.

Run:

```bash
cargo run --bin category_ml
```

Pass condition:

- the demo still runs
- the dataset windowing output includes your new transition

## Exercise 2: Break a Composition

Open [`examples/02_morphism_composition.rs`](../../examples/02_morphism_composition.rs).

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

Open [`examples/03_training_endomorphism.rs`](../../examples/03_training_endomorphism.rs).

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
