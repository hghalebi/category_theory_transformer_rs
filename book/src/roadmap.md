# Transformer Roadmap

The problem this chapter solves is:

> The repository name points toward Transformers, but the current code is a
> foundation course. This chapter explains exactly how the current objects and
> morphisms point toward a future attention-based model.

The current code is not a full Transformer.

It teaches the typed pieces you need first:

```text
tokens
vectors
logits
probabilities
loss
training updates
composition
```

## What Exists Now

The current model has this prediction path:

```text
TokenId -> Vector -> Logits -> Distribution
```

## Rust Syntax

The path is implemented with:

```text
Embedding
LinearToLogits
Softmax
Compose
```

The main domain objects are:

```text
TokenId
Vector
Logits
Distribution
Parameters
```

The training update is:

```text
TrainStep : Parameters -> Parameters
```

## ML Concept

This is a tiny next-token model.

It predicts from one token at a time.

It does not yet model attention across a sequence.

Still, it already teaches the core path:

```text
discrete token
  -> dense representation
  -> vocabulary scores
  -> next-token probabilities
```

## Category Theory Concept

The current system teaches composition:

```text
TokenId -> Vector -> Logits -> Distribution
```

and endomorphism:

```text
Parameters -> Parameters
```

Those two shapes remain central in Transformers.

## Step 1: Sequences As First-Class Objects

The future problem:

> Attention does not operate on one token alone. It operates on a sequence of
> hidden states.

## Rust Syntax

A future extension should introduce types such as:

```rust,ignore
pub struct Position(usize);
pub struct SequenceLength(usize);
pub struct HiddenSequence(Vec<Vector>);
pub struct AttentionMask(/* validated mask representation */);
```

The important rule is the same as this course:

```text
do not pass raw vectors across architectural boundaries
```

## ML Concept

Attention needs a representation like:

```text
[hidden_0, hidden_1, hidden_2, ...]
```

plus position and mask information.

## Category Theory Concept

The object changes from:

```text
Vector
```

to:

```text
Sequence(Vector)
```

The next morphisms operate on structured sequences.

## Step 2: Query, Key, And Value Projections

The future problem:

> Attention compares tokens by projecting hidden states into query, key, and
> value spaces.

## Rust Syntax

Future morphisms might have shapes:

```text
HiddenSequence -> QuerySequence
HiddenSequence -> KeySequence
HiddenSequence -> ValueSequence
```

Each output type should be distinct.

Queries, keys, and values are all vectors underneath, but they have different
roles.

## ML Concept

Queries ask:

```text
what am I looking for?
```

Keys answer:

```text
what do I contain?
```

Values provide:

```text
what information should be mixed?
```

## Category Theory Concept

These are parallel morphisms out of the same object:

```text
HiddenSequence -> QuerySequence
HiddenSequence -> KeySequence
HiddenSequence -> ValueSequence
```

The future attention block combines their results.

## Step 3: Scaled Dot-Product Attention

The future problem:

> Convert query-key similarity into a probability distribution over positions,
> then use it to mix values.

## Rust Syntax

A typed shape could be:

```text
QuerySequence x KeySequence -> AttentionScores
AttentionScores -> AttentionWeights
AttentionWeights x ValueSequence -> AttentionOutput
```

`AttentionWeights` should be validated like `Distribution`, but over sequence
positions instead of vocabulary tokens.

## ML Concept

Attention computes:

```text
scores = QK^T / sqrt(d)
weights = softmax(scores)
output = weights V
```

This is softmax again, but applied to token-to-token interaction scores.

## Category Theory Concept

The attention block is a composition of typed maps with a product input:

```text
(Q, K, V) -> scores -> weights -> mixed values
```

## Step 4: Multi-Head Attention

The future problem:

> One attention head sees one interaction pattern. Multiple heads let the model
> learn several patterns in parallel.

## Rust Syntax

Future types might include:

```rust,ignore
pub struct AttentionHead(/* head parameters */);
pub struct HeadCount(usize);
pub struct MultiHeadOutput(/* concatenated or projected heads */);
```

`HeadCount` should reject zero.

## ML Concept

Each head performs attention separately.

The outputs are combined and projected back into the model dimension.

## Category Theory Concept

This is parallel composition followed by recombination:

```text
head_1 x head_2 x ... x head_n -> combined output
```

## Step 5: Residual Blocks And Normalization

The future problem:

> Transformer blocks repeatedly map a hidden sequence back to a hidden sequence.

## Rust Syntax

A future block should have shape:

```text
HiddenSequence -> HiddenSequence
```

That means it can implement an endomorphism-like trait or reuse the existing
`Morphism<HiddenSequence, HiddenSequence>` shape.

## ML Concept

Transformer blocks use:

```text
attention
residual connection
normalization
feed-forward network
```

The block output has the same shape as the input.

## Category Theory Concept

This is another endomorphism:

```text
HiddenSequence -> HiddenSequence
```

Stacking layers is repeated endomorphism application.

## Step 6: Training And Evaluation

The future problem:

> Once the model has attention parameters, training must update a larger
> parameter object without losing type structure.

## Rust Syntax

The current:

```text
Parameters
```

would need to grow into a structured parameter type:

```rust,ignore
pub struct TransformerParameters {
    token_embedding: ...,
    attention_blocks: ...,
    lm_head: ...,
}
```

The update should still have the shape:

```text
TransformerParameters -> TransformerParameters
```

## ML Concept

The same high-level training loop remains:

```text
predict
compute loss
backpropagate
update parameters
```

The internal model becomes richer.

## Category Theory Concept

The training endomorphism generalizes:

```text
Parameters -> Parameters
```

to:

```text
TransformerParameters -> TransformerParameters
```

## Core Mental Model

The current course teaches the typed skeleton:

```text
TokenId -> Vector -> Logits -> Distribution
Distribution x TokenId -> Loss
Parameters -> Parameters
```

A Transformer extension grows the middle:

```text
TokenSequence
  -> HiddenSequence
  -> AttentionOutput
  -> HiddenSequence
  -> Logits
  -> Distribution
```

The practical rule stays the same:

> Make every intermediate object explicit, then compose only arrows whose types
> actually match.
