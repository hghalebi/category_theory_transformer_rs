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

There is also parameter context. When the roadmap calls
`LayerNormalization : HiddenSequence -> HiddenSequence` an endomorphism, it
means one fixed layer instance is being applied. Its scale, shift, and epsilon
are already stored in the Rust object. If those parameters are changing, the
safe boundary is the larger training update:

```text
TransformerTrainingState -> TransformerTrainingState
```

Use the same rule for `PositionWiseFeedForward` and Transformer blocks:
forward calls can be endomorphisms for fixed module values; learning those
module values belongs to training state.

Use this fixed-value checklist when a roadmap row returns
`HiddenSequence -> HiddenSequence`:

| Boundary | What must already be fixed? | If it changes |
| --- | --- | --- |
| `PositionalEncoding : HiddenSequence -> HiddenSequence` | the position table | name the table update or rebuild |
| `LayerNormalization : HiddenSequence -> HiddenSequence` | scale, shift, and epsilon inside one layer value | use `TransformerTrainingState -> TransformerTrainingState` |
| `PositionWiseFeedForward : HiddenSequence -> HiddenSequence` | weights, biases, and activation rule inside one feed-forward value | use `TransformerTrainingState -> TransformerTrainingState` |
| `MultiHeadTransformerBlock : HiddenSequence -> HiddenSequence` | heads, projections, residual path, normalization, and feed-forward layers inside one block value | use `TransformerTrainingState -> TransformerTrainingState` |
| fixed-mask view of a masked block | one named `AttentionMask` | return to `HiddenSequence x AttentionMask -> HiddenSequence`, or name the larger state carrying the mask |

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

If you choose to treat the whole product as one source object, the arrow is:

```text
(A x B) -> A
```

That is a unary morphism out of the product object, not an endomorphism. An
endomorphism on the product would have to return the same product object:

```text
(A x B) -> (A x B)
```

## Seven Sketches Transfer Drill

Use this drill when category-theory vocabulary feels clear in the abstract but
hard to connect back to tiny ML.

Run:

```bash
cargo run --example 05_seven_sketches
```

Then open:

```text
book/src/seven-sketches-rust.md -> Page-To-Rust Decision Ladder
book/src/seven-sketches-rust.md -> Bridge Back To Tiny ML
book/src/exercises.md -> Exercise 10 -> Page-to-Rust decision-ladder option
book/src/exercises.md -> Exercise 10 -> Bridge-back-to-tiny-ML option
```

If you are reading the source text page by page, start with the decision
ladder. Classify the paragraph as a definition, relation, composition rule,
law, worked example, or larger theory paragraph before choosing the Rust
handle. That prevents the common shortcut of creating a broad framework before
one local boundary is visible.

Pick one bridge row and fill this path:

```text
source idea:
Rust handle:
category-theory shape:
tiny ML pressure:
bad shortcut rejected:
safe non-claim:
evidence command or test:
```

The check sentence is:

```text
This sketch helps me reject this ML shortcut: ...
```

Examples of good transfer targets:

| Category word | Rust handle | Tiny ML pressure | Bad shortcut to reject |
| --- | --- | --- | --- |
| order | `InformationLevel::can_flow_to` | observations, features, scores, and decisions should not be interchangeable | treating a score as a decision |
| schema arrow | `CompanyInstance::new` | structured training rows should not contain dangling references | letting malformed source data reach feature extraction |
| matrix composition | `SignalMatrix::compose_after` | linear stages need matching middle dimensions | composing layers before checking shape |
| open interface | `OpenCircuit::then` | components need explicit input and output boundaries | wiring pieces by label while ignoring type shape |
| local-to-global claim | `SafetyCover::global_truth` | global safety depends on local checks | claiming global behavior while one local check failed |

The safe category-theory move is:

```text
name the structure -> name the Rust handle -> name the ML shortcut -> name the
non-claim
```

The non-claim matters. For example, `SignalMatrix::compose_after` checks one
matrix-composition boundary. It does not make this repository a general
automatic-differentiation system. `OpenCircuit::then` checks one interface
composition boundary. It does not implement the full theory of circuits from
the source text.

When this drill is useful for review, report the exact row that stopped
working. A strong report names the bridge row, the command output or test, and
the first place where the tiny ML transfer became unclear.

## Context-Fixing Mini-Drill

Use this drill when an example seems to become unary only after some extra
input has been chosen:

```text
1. Name the open boundary.
2. Name the context that was fixed.
3. Name the induced unary view.
4. State when that view stops being valid.
```

For the masked attention block, the careful sequence is:

```text
open boundary:
MaskedMultiHeadTransformerBlock : HiddenSequence x AttentionMask -> HiddenSequence

fixed context:
choose one mask M

induced view:
MaskedMultiHeadTransformerBlock[M] : HiddenSequence -> HiddenSequence
```

The view stops being valid as soon as the mask is no longer fixed or carried by
a named state object. That is the difference between fixing context and hiding
an input.

Rust readers can use closure capture as the concrete analogy:

```rust,ignore
let fixed_mask = mask.clone();
let fixed_mask_view = move |hidden: HiddenSequence| {
    masked_block.apply(Product::new(hidden, fixed_mask.clone()))
};
```

The closure captures `fixed_mask`; the call still receives `HiddenSequence`.
That helps explain the fixed-context view without pretending the original open
block had only one input.

Use residual addition as the negative contrast:

```text
ResidualConnection : HiddenSequence x ProjectedAttentionOutput -> HiddenSequence
```

This is not a context-fixing example. The hidden stream is still supplied, and
the projected sublayer output is still supplied. Nothing has been selected in
advance. Returning `HiddenSequence` is not enough to make the boundary unary.
It remains a product-input morphism unless one input is actually fixed or the
whole product is named as the source object. If the whole product is named as
the source object, the arrow is unary from that product:

```text
(HiddenSequence x ProjectedAttentionOutput) -> HiddenSequence
```

It still is not an endomorphism, because it does not return the same product
object. That difference is the reason the text keeps "product-input morphism"
visible.

## Roadmap Precision Review Drill

If you want to help with the most useful category-theory review target, use
the Transformer roadmap as a boundary-classification drill.

Run:

```bash
cargo run --example 06_attention_scores
```

Then open [Transformer Roadmap](../book/src/roadmap.md) and test the
`Attention Mental Model Repair Table` before the longer classification table.
Pick one shortcut and decide whether the repair points to a concrete Rust
boundary:

| Shortcut to test | Repair should make visible |
| --- | --- |
| query turns into key, then key turns into value | Q, K, and V are sibling roles, not a role-to-role pipeline |
| raw scores are already attention probabilities | masking and row-wise softmax happen before value mixing |
| same output shape means endomorphism | the whole source object must be counted first |
| fixing a mask means the mask disappeared | the fixed mask remains named context |

A useful report can name one row from that repair table and say whether the
local Rust checkpoint made the safer model inspectable.

Then test this classification table:

| Boundary from the roadmap | Careful classification | Evidence signal to report if unclear |
| --- | --- | --- |
| `AttentionScores -> AttentionWeights` | ordinary morphism | the score-to-probability row felt like an endomorphism |
| `AttentionScores x AttentionMask -> AttentionScores` | product-input morphism returning scores | the mask input disappeared from the name |
| `LayerNormalization : HiddenSequence -> HiddenSequence` | unary endomorphism for one fixed layer instance | the parameter context was not named |
| `HiddenSequence x ProjectedAttentionOutput -> HiddenSequence` | product-input morphism returning hidden state | returning `HiddenSequence` made it tempting to call this an endomorphism |
| `MaskedMultiHeadTransformerBlock : HiddenSequence x AttentionMask -> HiddenSequence` | open product-input block boundary | the fixed-mask view was not distinguished from the open block |
| fixed mask view of a masked block | induced `HiddenSequence -> HiddenSequence` view for one chosen mask | the text did not say which context was fixed |
| whole Transformer training update | state endomorphism | the update object was not clearly `TransformerTrainingState` before and after |
| linear self-attention scope | limited place where advanced endofunctor language may be compared | the warning about softmax, masking, residuals, normalization, or training did not feel bounded |

Then run a second pass over the terminal output. Treat printed shape lines as
evidence about targets, not as category names:

| Printed output line | First question |
| --- | --- |
| `Q/K/V source diagnostic:` | Which source owns score rows, score columns, and local mask polarity before attention weights appear? |
| `projected attention shape: 2 positions x model dimension 2` | Which boundary produced the projected object before residual addition? |
| `residual shape: 2 positions x model dimension 2` | Which two inputs were needed before the result returned to hidden shape? |
| `masked multi-head block shape: 2 positions x model dimension 2` | Is the mask still an open input or was one mask fixed first? |
| `training state step: 0 -> 1` | Which whole state object returned for the next update? |

The audit rule is:

```text
printed shape line -> target evidence
typed transformation line -> source and target evidence
category name -> only after both are known
```

A useful category-theory report names exactly one row and says which naming
rule failed:

```text
count inputs
name the source object
name the target object
reject the overclaim
```

If you get stuck, use the roadmap's `Reader Evidence Handoff` as the report
shape and the `Source-Target Audit Card` as the precision check:

```text
book/src/roadmap.md -> Category Shape Diagnostic -> Reader Evidence Handoff
book/src/roadmap.md -> Category Shape Diagnostic -> Source-Target Audit Card
cargo run --example 06_attention_scores
one boundary row or printed output line
```

This drill does not ask a reviewer to prove the formal theory. It asks whether
the public text keeps its category-theory names proportional to the Rust
boundaries that readers can inspect.

## Checkpoint

Explain the difference between these two boundaries:

```text
LayerNormalization : HiddenSequence -> HiddenSequence
ResidualConnection : HiddenSequence x ProjectedAttentionOutput -> HiddenSequence
```

A strong answer should say:

- the first is a unary endomorphism on `HiddenSequence`
- the second is a product-input morphism returning `HiddenSequence`
- the second should not be called a unary endomorphism unless one input is
  actually fixed or the whole product input is treated as the source object

## Precision Failure Signals

When a category-theory-flavored test fails, do not first ask whether the code
"disproves category theory." Ask which small claim this tutorial was using the
test to protect.

| Command or test | If it fails, inspect this claim |
| --- | --- |
| `cargo test structure::tests --lib` | a functor, naturality, or trace-monoid example no longer matches its stated law |
| `structure::tests::naturality_square_commutes` | the two paths around the `Vec<A> -> Option<A>` square no longer agree |
| `structure::tests::pipeline_trace_obeys_monoid_laws` | trace composition may no longer have identity or associativity in the tiny example |
| `cargo test sketches::tests --lib` | an applied sketch boundary changed without the chapter naming it |
| `sketches::tests::feature_layer_abstraction_obeys_galois_law` | the feature-to-layer abstraction no longer satisfies the stated fit equivalence |
| `sketches::tests::signal_matrix_composition_rejects_mismatched_middle_dimension` | signal-flow composition is no longer enforcing the middle-dimension boundary |
| `sketches::tests::open_circuit_serial_composition_rejects_boundary_mismatch` | open-circuit composition is no longer enforcing matching ports |
| `attention::tests::residual_connection_rejects_model_dimension_mismatch` | residual addition is no longer protecting `HiddenSequence x ProjectedAttentionOutput -> HiddenSequence` shape agreement |
| `attention::tests::masked_multi_head_transformer_block_rejects_mask_shape_mismatch` | the masked block may be hiding mask context instead of enforcing the open product boundary |
| `attention::tests::multi_head_transformer_block_rejects_output_projection_input_mismatch` | the block may be skipping the projection needed before residual composition |

These tests are not a proof of the whole mathematical theory. They are local
precision checks for claims the book makes about one executable model.

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

For product-input rows, name the whole product as the source object before
deciding whether the boundary is an endomorphism.

## Feedback

If a term, law, diagram, product-input boundary, endomorphism claim, or
endofunctor warning becomes unclear, open the
[chapter clarity feedback form](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=chapter-clarity.yml)
and include:

For this path, use
[Open category-theory reader report](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=chapter-clarity.yml&title=%5Bgood+first+feedback%5D+category-theory+reader+brief&location=docs%2Fcategory-theory-path.md+-%3E+Seven+Sketches+Transfer+Drill%3B+book%2Fsrc%2Froadmap.md+-%3E+Category+Shape+Diagnostic&command=cargo+run+--example+05_seven_sketches%0Acargo+run+--example+06_attention_scores).
The link fills the route, not the evidence; the evidence signal should come
from what you personally read, ran, or attempted.

```text
Perspective: category-theory reader
Command or page tried:
Evidence signal:
First unclear term, law, diagram, or boundary:
Last idea that was clear:
What would have helped:
```

Use the evidence signal for the exact law, table row, morphism shape, output
line, or naming rule that became too compressed.
