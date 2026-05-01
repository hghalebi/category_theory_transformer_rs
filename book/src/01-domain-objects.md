# Domain Objects

## Mental Model

Objects are the nouns of the system.

In everyday Rust, it is tempting to pass `usize`, `Vec<f32>`, and `f32`
everywhere. That is easy at first, then confusing later.

This repo wraps meaningful values in small domain types.

For the tiny model, the first mistake to avoid is mixing up different meanings
that happen to use the same machine representation. A token index, a vector
length, a loss value, and a learning rate are all numbers, but they play very
different roles.

The category-theory reading is simple: before we talk about arrows, we need
clear objects for arrows to connect.

## Source Snapshot

This is the domain layer used by the whole tutorial.

Focus on:

- `TokenId`
- `TokenSequence`
- `Vector`
- `Logits`
- `Distribution`
- `Loss`
- `TrainingSet`
- `Parameters`

<details>
<summary>Source snapshot: src/domain.rs</summary>

```rust,ignore
{{#include ../../src/domain.rs}}
```

</details>

## Code Walkthrough

`TokenId` is the smallest object in the model. It wraps a vocabulary index so a
token cannot be confused with any other `usize` by accident.

`TokenSequence` validates that the sequence is not empty. That puts the failure
at construction time instead of letting an empty sequence travel into training.

`Distribution` validates the ML invariant that probabilities are finite,
non-negative, non-empty, and sum to one.

`Product<A, B>` gives the course its first categorical product. In ML terms,
`TrainingExample` is `(input_token, target_token)`.

`Parameters` stores the embedding table, language-model head, and bias. It is
the object that training will map back into itself.

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

## Runnable Example Snapshot

<details>
<summary>Source snapshot: examples/01_domain_objects.rs</summary>

```rust,ignore
{{#include ../../examples/01_domain_objects.rs}}
```

</details>

## Checkpoint

What bug becomes harder when `TokenId` is not just a raw `usize`?

## Further Reading

- [Glossary](glossary.md): object, product object, invariant, smart constructor
- [References](references.md): Rust error handling, Rust API design, and Rust documentation
