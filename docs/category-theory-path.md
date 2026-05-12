# Category Theory Path

Use this path if you are curious about category theory but want executable
examples first.

The goal is not to memorize category-theory vocabulary in isolation. The goal
is to attach each word to a Rust type, a command output, and a boundary that
prevents one invalid composition.

## First Pass

Run the examples before trying to make the terms formal:

```bash
cargo run --example 02_morphism_composition
cargo run --example 04_structure_and_calculus
cargo run --example 05_seven_sketches
```

Then read in this order:

1. [Course Map](../book/src/00-map.md)
2. [Morphism and Composition](../book/src/02-morphisms-composition.md)
3. [Functors, Naturality, Monoids, and Chain Rule](../book/src/05-structure-and-calculus.md)
4. [Seven Sketches Through Rust](../book/src/seven-sketches-rust.md)
5. [Transformer Roadmap](../book/src/roadmap.md)

The first reading should answer one question:

```text
Which Rust boundary makes this category-theory word concrete?
```

## Core Questions

- What is an object in this tiny system?
- What is a morphism?
- Why does composition require matching types?
- Why is a training step an endomorphism?
- What law does each example check?

## Precision Rules

Use these rules while reading the roadmap and the attention examples:

| Shape | Careful name | Example | Common overclaim |
| --- | --- | --- | --- |
| `A -> B` | ordinary morphism | `AttentionScores -> AttentionWeights` | calling every arrow an endomorphism |
| `A -> A` | endomorphism | `LayerNormalization : HiddenSequence -> HiddenSequence` | ignoring whether the input and output object are identical |
| `A x B -> C` | product-input morphism | `AttentionWeights x ValueSequence -> AttentionOutput` | hiding the second input |
| `A x B -> A` | product-input morphism returning `A` | `HiddenSequence x ProjectedAttentionOutput -> HiddenSequence` | calling it a unary endomorphism |
| failed middle type | illegal composition | `HiddenSequence x MultiHeadOutput -> HiddenSequence` | skipping the output projection |

The practical rule is:

```text
count inputs before naming the arrow
```

If a boundary needs two objects, keep both objects visible. If a boundary
returns the same public object but also needs extra context, name the context
instead of forcing the word `endomorphism`.

## Active Practice

Run the attention example:

```bash
cargo run --example 06_attention_scores
```

Then open [Exercise 12](../book/src/exercises.md#exercise-12-trace-attention-shape-flow)
and answer the quick roadmap classification drill before checking the answer
key.

Classify these five boundaries:

```text
HiddenSequence -> QuerySequence
AttentionScores x AttentionMask -> AttentionScores
LayerNormalization : HiddenSequence -> HiddenSequence
HiddenSequence x ProjectedAttentionOutput -> HiddenSequence
TransformerTrainingState -> TransformerTrainingState
```

The practice target is narrow:

```text
count inputs -> name the category shape -> reject one overclaim
```

In particular, explain why `A x B -> A` is not automatically an endomorphism
on `A`.

## Checkpoint

Explain the difference between these two boundaries:

```text
LayerNormalization : HiddenSequence -> HiddenSequence
ResidualConnection : HiddenSequence x ProjectedAttentionOutput -> HiddenSequence
```

A strong answer should say:

- the first is a unary endomorphism on `HiddenSequence`
- the second is a product-input morphism returning `HiddenSequence`
- the second should not be called a unary endomorphism unless the whole product
  input is treated as the source object

## Source Discipline

Advanced category-theory papers are useful for precision, but this project
keeps every claim tied to the tiny Rust implementation. When a chapter mentions
a law, diagram, or categorical name, ask for one of these forms of evidence:

```text
named Rust type
typed boundary
constructor invariant
test that checks a law or failure case
runnable example output
```

If none of those is nearby, the prose is probably moving too fast.

## Done Criteria

You can explain category-theory vocabulary without leaving the Rust code:

```text
object -> morphism -> composition -> law
```

You can also classify a Transformer boundary without overnaming it:

```text
ordinary morphism
product-input morphism
endomorphism
illegal attempted composition
```

## Feedback

If a term, law, diagram, product-input boundary, endomorphism claim, or
endofunctor warning becomes unclear, open the
[chapter clarity feedback form](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=chapter-clarity.yml)
and include:

```text
Perspective: category-theory reader
Command or page tried:
First unclear term, law, diagram, or boundary:
Last idea that was clear:
What would have helped:
```
