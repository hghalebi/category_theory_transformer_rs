# Domain Objects

## Mental Model

Objects are the nouns of the system.

In everyday Rust, it is tempting to pass `usize`, `Vec<f32>`, and `f32`
everywhere. That is easy at first, then confusing later.

This repo wraps meaningful values in small domain types.

## Read This File

Open [`src/domain.rs`](../../src/domain.rs).

Focus on:

- `TokenId`
- `TokenSequence`
- `Vector`
- `Logits`
- `Distribution`
- `Loss`
- `TrainingSet`
- `Parameters`

## Run the Example

```bash
cargo run --example 01_domain_objects
```

Expected shape:

```text
training pairs:
1 -> 2
2 -> 3
3 -> 4
```

## Why This Matters

`TokenId` and `Loss` are not interchangeable, even if both could be represented
by numbers.

Rust keeps those ideas separate.

## Checkpoint

What bug becomes harder when `TokenId` is not just a raw `usize`?
