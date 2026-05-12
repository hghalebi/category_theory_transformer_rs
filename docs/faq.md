# FAQ

## Is this a production ML framework?

No. This is a teaching crate and public book. The implementation is deliberately
small so the whole system can be inspected.

## Why Rust?

Rust makes data shape, ownership, errors, and invariants explicit. That is a
good fit for teaching category-theory ideas as engineering structure.

## Why category theory?

Category theory gives names to typed transformations, composition, products,
endomorphisms, functors, and laws. Those names help explain why the tiny ML
pipeline is organized the way it is.

## Where should I start?

Run:

```bash
cargo run --example 01_token_sequence
```

Then read [START_HERE.md](../START_HERE.md).

## How can I help?

Open a specific issue. The best first issues point to one unclear sentence, one
missing diagram, one Rust example, or one exercise idea.

## What does this project unlock?

Framework-only tutorials often teach the call:

```text
model(input)
```

This project teaches the structure that call compresses.

After the first path, a reader should be able to:

- see `TokenId`, `TokenSequence`, `TrainingSet`, `Parameters`,
  `Distribution`, and `Loss` as different ML objects, not loose values
- explain a tiny pipeline as typed transformations:

  ```text
  Text -> TokenSequence -> TrainingPairs -> ModelState -> Prediction -> Loss
  ```

- recognize composition as the rule that lets one stage feed the next
- describe training as a controlled update of model state
- read a larger framework API with better questions about inputs, outputs,
  hidden state, shape, loss, and update steps

The unlock is not a production-scale model. The unlock is an inspectable mental
model for what larger frameworks automate.

Run the first concrete example:

```bash
cargo run --example 01_token_sequence
```
