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
