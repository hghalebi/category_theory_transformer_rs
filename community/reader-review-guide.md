# Reader Review Guide

This guide turns reader feedback into concrete textbook improvements.

The most useful review is not a general opinion. It is a precise report of
where a learner's mental model becomes weaker than the chapter expects.

If you want the shortest path, use `community/reader-review-packet.md`.
If you want to invite reviewers, use `community/reviewer-outreach.md`.
If you want a coordinated review round, use `community/reader-review-sprint.md`.

## Thirty-Minute Review Path

Use this path when you want to help but do not know where to start.

1. Run the first example:

   ```bash
   cargo run --example 01_token_sequence
   ```

2. Read these entry points:

   ```text
   README.md
   START_HERE.md
   book/src/welcome.md
   book/src/00-map.md
   ```

3. Read one core chapter:

   ```text
   book/src/01-domain-objects.md
   book/src/02-morphisms-composition.md
   book/src/03-ml-pipeline.md
   book/src/04-training-endomorphism.md
   ```

4. Open one issue with the first point of friction.

## What To Look For

Good reader feedback answers one of these questions.

| Review lens | Useful question |
| --- | --- |
| Entry path | Did the first command and first chapter tell you what to do next? |
| ML intuition | Did the chapter explain the machine-learning idea before naming the abstraction? |
| Rust syntax | Did the code explain what each type or trait is protecting? |
| Category-theory precision | Did a term appear before the code shape made it concrete? |
| Diagram clarity | Did a diagram reduce confusion, or add another thing to decode? |
| Exercise readiness | Could you attempt the suggested exercise without guessing the goal? |
| Reference need | Would an external source make the claim easier to trust? |

## Good Feedback Format

Use the
[reader-confusion GitHub issue form](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=reader-confusion.yml)
when possible. It asks for the same information in a structured format so
repeated friction can be compared across reports.

Maintainers should process submitted reports with
`community/reader-feedback-triage.md` and record accepted direct reports in
`community/reader-feedback-inbox.md`.

If you are writing the report manually, use this structure.

```text
Chapter:
Section:
Command or file I tried:
What I understood:
Where I got stuck:
What I expected next:
Suggested fix:
```

Example:

```text
Chapter: Morphism and Composition
Section: The Middle Type Is The Contract
Command or file I tried: cargo run --example 02_morphism_composition
What I understood: TokenId becomes Vector, then Vector becomes Logits.
Where I got stuck: I was not sure why Logits and Distribution need separate types.
What I expected next: one sentence explaining raw scores versus probabilities.
Suggested fix: add a short contrast before the Softmax example.
```

## Diagram Review

When reviewing diagrams, quote the exact diagram and check three things.

```text
1. Does every node name a real concept from the code or chapter?
2. Does every arrow represent a typed transformation?
3. Is there one clear takeaway sentence immediately after the diagram?
```

If a diagram fails one of these checks, open an issue with the title:

```text
[diagram review] Clarify <chapter or diagram name>
```

## Attention Roadmap Review

The current attention material is intentionally small. It models:

```text
HiddenSequence -> QuerySequence
HiddenSequence -> KeySequence
HiddenSequence -> ValueSequence
QuerySequence x KeySequence -> AttentionScores
AttentionScores x AttentionMask -> AttentionScores
AttentionScores -> AttentionWeights
AttentionWeights x ValueSequence -> AttentionOutput
AttentionHeadOutputs -> MultiHeadOutput
MultiHeadOutput -> ProjectedAttentionOutput
HiddenSequence x ProjectedAttentionOutput -> HiddenSequence
HiddenSequence -> HiddenSequence
PositionWiseFeedForward : HiddenSequence -> HiddenSequence
PositionalEncoding : HiddenSequence -> HiddenSequence
SingleHeadTransformerBlock : HiddenSequence -> HiddenSequence
MultiHeadTransformerBlock : HiddenSequence -> HiddenSequence
MaskedMultiHeadTransformerBlock : HiddenSequence x AttentionMask -> HiddenSequence
```

Run:

```bash
cargo run --example 06_attention_scores
```

Then review:

```text
src/attention.rs
examples/06_attention_scores.rs
book/src/roadmap.md
book/src/source-snapshots.md
```

Useful feedback should separate current claims from future work.

The newest review target is the self-attention versus cross-attention boundary
in `book/src/roadmap.md`. Ask whether the chapter makes these distinctions
clear:

```text
same source for Q, K, V -> self-attention case
parallel HiddenSequence -> QuerySequence, KeySequence, ValueSequence projections
not QuerySequence -> KeySequence -> ValueSequence
separate query and key-value sources -> cross-attention case
target sequence length L
source sequence length S
product-input morphism before any endomorphism claim
```

High-signal feedback names the first point where target-side queries,
source-side keys, source-side values, parallel projections, or the `L x S` mask shape became unclear.

The newest review target is the category-shape diagnostic in
`book/src/roadmap.md`. Ask whether the chapter makes these distinctions clear:

```text
ordinary morphism
product-input morphism
shape-preserving endomorphism
state endomorphism
illegal attempted boundary
```

High-signal feedback names one concrete boundary and says which classification
felt unclear. For example, `AttentionScores x AttentionMask -> AttentionScores`
should not be reviewed as if it had the same shape as
`LayerNormalization : HiddenSequence -> HiddenSequence`.

Good issue:

```text
The attention example clearly teaches masking, row-wise score normalization,
value mixing, head concatenation, output projection, residual addition,
normalization, feed-forward structure, and the single-head and multi-head block
sketches. The next useful review is whether the masked block boundary is clear.
```

Not useful:

```text
This is not a full Transformer.
```

The project already treats feed-forward structure, positional encoding, the
single-head block, the multi-head block sketch, and the masked block variant as
implemented tiny boundaries. It also now treats structured Transformer
parameters, sequence readout, and training-state metadata as implemented tiny
boundaries. The readout-only training step is implemented as the first state
endomorphism. The local feed-forward training step is also implemented as a
scoped sublayer update. The composed block training step now updates the
readout, feed-forward sublayer, attention output projection, query/key/value
projections, and layer-normalization scale/shift parameters from token targets
through residual, normalization, and attention paths. The first
gradient-checking worked solution is present; report whether it is enough or
where another worked example is needed.

## Review Boundaries

This project is a public book and Rust lab. Reviews should improve the learning
artifact for general readers.

Prioritize:

- precise confusion reports,
- clearer examples,
- smaller exercises,
- terminology fixes,
- source-backed corrections,
- runnable code checks.

Avoid:

- vague encouragement,
- private notes,
- unsourced technical claims,
- requests for large rewrites without a specific blocked reader path.

## Suggested Issue Titles

```text
[reader confusion] <chapter>: <short friction point>
[diagram review] <chapter>: <diagram name>
[ML intuition] <chapter>: <concept needing clearer explanation>
[Rust idiom review] <file>: <type or trait>
[category theory precision] <term>: <possible overclaim or ambiguity>
[exercise readiness] <exercise>: <missing instruction or expected result>
[reference needed] <chapter>: <claim needing source support>
```

## Direct Review Issue Form

The repository includes `.github/ISSUE_TEMPLATE/reader-confusion.yml` and the
[reader-confusion issue form](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=reader-confusion.yml)
for direct reader reports. A useful report names:

```text
review path
command or file tried
friction lens
confusing text or output
what made sense before that point
where the mental model broke
smallest useful fix
```

## Success Signal

A good review should make one next edit obvious.

The best feedback helps the book move from:

```text
interesting idea
```

to:

```text
clear path, runnable proof, inspectable structure
```
