# Exercises

The problem this chapter solves is:

> Reading detailed explanations is not enough. You need to practice explaining
> the code through Rust syntax, ML concept, and category-theory concept.

For every exercise, use this answer shape:

```text
Rust syntax:
...

ML concept:
...

Category theory concept:
...
```

The point is not to write long answers.

The point is to connect the same block of code across all three meanings.

## Exercise 1: Explain One Domain Type

Use [Domain Objects](01-domain-objects.md).

Pick one type:

- `Vector`
- `Logits`
- `Distribution`
- `Loss`
- `TrainingSet`
- `Parameters`

Write:

```text
The problem this solves:

Rust syntax:

ML concept:

Category theory concept:
```

Pass condition:

- You name the raw representation.
- You name the invariant or semantic distinction.
- You name the pipeline stage where the type appears.

## Exercise 2: Add A Token

Use the `src/demo.rs` snapshot in [Course Map](00-map.md).

Add one new vocabulary item and extend the token sequence.

Run:

```bash
cargo run --bin category_ml
```

Pass condition:

- the demo still runs
- the dataset windowing output includes your new transition
- you can explain why a longer `TokenSequence` creates more training examples

## Exercise 3: Trace `DatasetWindowing`

Use [The Tiny ML Pipeline](03-ml-pipeline.md).

For this input:

```text
[TokenId(4), TokenId(8), TokenId(15), TokenId(16)]
```

write the training examples produced by `windows(2)`.

Then explain:

```text
Rust syntax:
what does `.windows(2)` do?

ML concept:
why does next-token training need adjacent pairs?

Category theory concept:
why is each example a product object?
```

## Exercise 4: Break A Composition

Use the `examples/02_morphism_composition.rs` snapshot in
[Morphism and Composition](02-morphisms-composition.md).

Try to compose `Embedding` directly with `Softmax`.

Expected failure shape:

```text
the trait bound ... is not satisfied
```

Then restore the working version.

Explain:

```text
Rust syntax:
which type did the compiler reject?

ML concept:
which prediction stage was skipped?

Category theory concept:
which middle object failed to match?
```

## Exercise 5: Change The Training Repetition Count

Use the `examples/03_training_endomorphism.rs` snapshot in
[Training as an Endomorphism](04-training-endomorphism.md).

Change:

```rust,ignore
StepCount::new(80)
```

to:

```rust,ignore
StepCount::new(1)
StepCount::new(10)
StepCount::new(200)
```

Run:

```bash
cargo run --example 03_training_endomorphism
```

Explain the result:

```text
Rust syntax:
where is the count used?

ML concept:
what happens when training repeats more times?

Category theory concept:
why can the update be repeated?
```

## Exercise 6: Explain `Distribution<T>::map`

Use [Functors, Naturality, Monoids, and Chain Rule](05-structure-and-calculus.md).

Explain the conceptual `Distribution<T>::map` example.

Use this input distribution:

```text
TokenId(2) -> 0.70
TokenId(3) -> 0.30
```

and this function:

```text
TokenId -> String
```

where:

```text
TokenId(2) -> "Rust"
TokenId(3) -> "."
```

Write the output distribution.

Then explain:

```text
Rust syntax:
why does `self` plus `into_iter()` move the old outcomes?

ML concept:
why do the probabilities stay the same?

Category theory concept:
what does it mean to lift `T -> U` into `Distribution<T> -> Distribution<U>`?
```

## Exercise 7: Explain One Validation Boundary

Pick one constructor:

- `Distribution::new`
- `Loss::new`
- `LearningRate::new`
- `TrainingSet::new`
- `SignalMatrix::new`
- `OpenCircuit::new`

Write:

```text
The problem this solves:

Rust syntax:
which condition returns `Err(...)`?

ML or software concept:
what bad runtime behavior does this prevent?

Category theory concept:
what intended object or relationship is being protected?
```

## Exercise 8: Trace A Full Source File

Use [Repository Source Snapshots](source-snapshots.md).

Pick one complete source file and write a five-sentence summary:

1. What problem does the file solve?
2. What are the main Rust types or traits?
3. What ML or software concept does it model?
4. What category-theory concept does it teach?
5. Which command proves the file still works?

## Exercise 9: Connect One External Reference

Use [References](references.md).

Pick one external resource and connect it to one source file in this course.

Answer:

```text
External resource:
Source file:
Rust syntax connection:
ML or software concept connection:
Category theory concept connection:
One difference between the full treatment and this tiny implementation:
```

## Exercise 10: Test One Sketch Law

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
- your explanation uses Rust syntax, ML or software concept, and category theory concept

## Exercise 11: Write A New Block Explanation

Choose any block from the source snapshots that the chapter did not explain in
enough detail for you.

Write a block explanation using this structure:

```text
The problem this block solves:

The whole block:

Rust syntax:

ML or software concept:

Category theory concept:

Core mental model:
```

Pass condition:

- A beginner can understand the Rust syntax.
- An ML learner can understand why the block exists.
- A category-theory learner can name the shape.
