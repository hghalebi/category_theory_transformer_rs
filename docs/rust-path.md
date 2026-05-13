# Rust Path

Use this path if you know Rust syntax and want to see how types can carry ML
meaning.

The goal is not to learn Rust from scratch. The goal is to use familiar Rust
tools such as structs, constructors, traits, `Result`, and tests to make a tiny
ML pipeline harder to misuse.

## First Pass

Run these commands before reading deeply:

```bash
cargo run --example 01_domain_objects
cargo run --example 02_morphism_composition
cargo test domain::tests --lib
cargo test category::tests --lib
```

Then inspect in this order:

1. [src/domain.rs](../src/domain.rs)
2. [src/error.rs](../src/error.rs)
3. [src/category.rs](../src/category.rs)
4. [src/ml.rs](../src/ml.rs)
5. [Morphism and Composition](../book/src/02-morphisms-composition.md)

The first reading should answer one question:

```text
Which invalid ML state does this Rust boundary make harder to express?
```

## Core Questions

- Why is `TokenId` better than passing a raw `usize` through the public API?
- Which constructors reject invalid state?
- How does `Morphism<Input, Output>` make composition visible?
- Which errors are recoverable in the tutorial pipeline?
- Which type parameter is the middle object in `Compose<F, G, Middle>`?
- Which test would fail if an invariant or composition law were weakened?

## Rust Boundary Map

| Rust feature | Local file | Teaching role |
| --- | --- | --- |
| named structs | `src/domain.rs` | separate `TokenId`, `Vector`, `Logits`, `Distribution`, and `Loss` |
| validating constructors | `src/domain.rs` | reject empty sequences, invalid probabilities, and invalid rates |
| typed errors | `src/error.rs` | keep recoverable failures explicit |
| traits | `src/category.rs` | give transformations a shared interface |
| generic composition | `src/category.rs` | require the first target type to match the second source type |
| unit tests | `src/domain.rs`, `src/category.rs`, `src/ml.rs` | check invariants, laws, and pipeline behavior |

## Attention Boundary Practice

After the core composition example works, inspect the typed attention sketch:

```bash
cargo run --example 06_attention_scores
```

Then open [src/attention.rs](../src/attention.rs) and find the types behind
these output lines:

| Output or boundary | Rust object to inspect | Invariant or misuse it should expose |
| --- | --- | --- |
| `attention shape: 2 query positions x 3 key positions` | `QuerySequence`, `KeySequence`, `AttentionScores` | score rows belong to queries and score columns belong to keys |
| `query 0 attends with [0.5, 0.0, 0.5]` | `AttentionMask`, `AttentionWeights` | masked positions are removed before row-wise normalization |
| `query 0 output vector [2.0, 20.0]` | `ValueSequence`, `AttentionOutput` | weights mix values, not keys or raw scores |
| `projected attention shape` | `MultiHeadOutput`, `ProjectedAttentionOutput` | concatenated head width must return to model width before residual addition |
| `residual shape` | `ResidualConnection`, `HiddenSequence` | residual addition needs both the old hidden stream and projected attention output |
| `normalized shape` | `LayerNormalization`, `LayerNormParameters` | a fixed layer value preserves `HiddenSequence` shape while using stored scale, shift, and epsilon |
| `feed-forward shape` | `PositionWiseFeedForward` | a fixed feed-forward value preserves public model width even though it has internal weights |
| `training state step: 0 -> 1` | `TransformerTrainingState` | changing stored parameters belongs to the training-state boundary, not the forward layer boundary |

The Rust question is:

```text
Which constructor, type, or test keeps this boundary from accepting the wrong
role or shape?
```

This is the code-facing version of the attention shape ledger. Do not weaken a
type because two values are both backed by `Vec<Vec<f32>>`. The point of the
roadmap sketch is that `QuerySequence`, `KeySequence`, `ValueSequence`,
`AttentionScores`, `AttentionWeights`, and `AttentionOutput` are different
roles even when their storage looks similar.

Use the same discipline for parameters. A forward call such as
`LayerNormalization : HiddenSequence -> HiddenSequence` is a Rust boundary for
one fixed layer value. If the scale, shift, weights, or biases are changing,
the boundary you should inspect is:

```text
TransformerTrainingState -> TransformerTrainingState
```

Do not hide parameter changes inside a shape-preserving forward arrow.

## Active Practice

After running `cargo run --example 02_morphism_composition`, explain these two
lines from the output:

```text
Embedding then LinearToLogits is legal because Vector == Vector
Embedding then Softmax is illegal because Vector != Logits
```

A strong Rust answer should name:

- the first implementation: `Morphism<TokenId, Vector> for Embedding`,
- the second implementation: `Morphism<Vector, Logits> for LinearToLogits`,
- the rejected shortcut: `Softmax` expects `Logits`, not `Vector`,
- the generic bridge: `Compose::<_, _, Vector>` or `Compose::<_, _, Logits>`.

Then try Exercise 4 in [Exercises](../book/src/exercises.md#exercise-4-break-a-composition).

## Failure Signals

Rust readers should treat the tests as boundary diagnostics, not as ritual
commands. When one fails, ask which invariant stopped being protected.

| Command or test | If it fails, inspect this boundary |
| --- | --- |
| `cargo test domain::tests --lib` | a domain constructor is allowing an invalid ML object or rejecting a valid one |
| `domain::tests::token_sequence_rejects_empty_input` | `TokenSequence::new` no longer protects the non-empty input boundary |
| `domain::tests::distribution_rejects_non_normalized_values` | `Distribution::new` no longer protects the probability boundary |
| `cargo test category::tests --lib` | the arrow layer changed composition, identity, or error propagation |
| `category::tests::composition_applies_first_then_second` | `Compose<F, G, Middle>` no longer applies the first morphism before the second |
| `category::tests::composition_returns_the_first_error` | composition is hiding or replacing the upstream boundary error |

Use the example output as the quick sanity check after a code change:

```text
Embedding then LinearToLogits is legal because Vector == Vector
Embedding then Softmax is illegal because Vector != Logits
```

If those lines stop matching the code, the likely problem is not prose. It is
that the Rust path no longer exposes the middle object that makes composition
safe.

## Common Rust Misreadings

| Misreading | What to say instead |
| --- | --- |
| tuple structs are only cosmetic | wrappers separate domain roles and can hide raw representation |
| every constructor should return `Self` | constructors return `Result` when they enforce an invariant |
| `Morphism<Input, Output>` is just a function pointer | it is a named fallible transformation with a shared composition API |
| `Compose` proves all category laws | it models the typed composition shape used by the examples |
| errors are distractions from the lesson | errors are evidence that a boundary rejected an invalid state |

## Done Criteria

You can point to one type, one invariant, one morphism, and one test that
protects the teaching model.

You can also explain why weakening a type signature is usually the wrong fix
for a failed composition. The right repair is to restore the missing middle
object or the morphism that produces it.

## Feedback

Open the [chapter clarity feedback form](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=chapter-clarity.yml)
when a Rust type, trait bound, constructor, or test stops explaining the ML
idea.

For this path, use
[Open Rust engineer report](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=chapter-clarity.yml&title=%5Bgood+first+feedback%5D+Rust+engineer+brief&location=docs%2Frust-path.md&command=cargo+run+--example+01_domain_objects%0Acargo+run+--example+02_morphism_composition%0Acargo+test+domain%3A%3Atests+--lib%0Acargo+test+category%3A%3Atests+--lib).
The link fills the route, not the evidence; the evidence signal should come
from what you personally read, ran, or attempted.

Use this shape:

```text
Perspective: Rust engineer
Command or page tried:
Evidence signal:
Rust boundary that became unclear:
ML role I expected it to explain:
What would have helped:
```

Use the evidence signal for the compiler error, test name, constructor result,
output line, or type boundary that exposed the confusion.
