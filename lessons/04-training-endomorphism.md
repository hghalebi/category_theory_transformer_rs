# 04 - Training as an Endomorphism

## Mental Model

An endomorphism maps a thing back to the same kind of thing:

```text
A -> A
```

Training does exactly that:

```text
Parameters -> Parameters
```

The model changes, but it is still a model.

## Read This File

Open `src/training.rs`.

Read in this order:

1. `TrainStep`
2. `impl Morphism<Parameters, Parameters> for TrainStep`
3. The unit test at the bottom

## Run the Example

```bash
cargo run --example 03_training_endomorphism
```

Expected pattern:

```text
loss before: ...
loss after:  ...
```

The second number should be smaller.

## Why This Is Category-Theoretic

`TrainStep` can be repeated because its input type and output type are both
`Parameters`.

That makes this loop type-correct:

```text
Parameters0 -> Parameters1 -> Parameters2 -> ... -> ParametersN
```

## Checkpoint

Why would training be harder to compose if it returned raw vectors instead of
`Parameters`?
