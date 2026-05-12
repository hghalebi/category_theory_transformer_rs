# Glossary

The problem this chapter solves is:

> Abstract terms are easier to remember when each term is tied to a Rust type,
> an ML role, and a category-theory shape.

Use this glossary as a lookup table while reading the source snapshots.

Do not read it as a separate dictionary. Each entry is deliberately anchored to
the codebase. If a definition sounds abstract, jump from the term to the Rust
syntax and then back to the chapter where the type or trait appears.

> Reader orientation:
> The glossary uses compact entries, but the entries still follow the book's
> main discipline: first the Rust handle, then the ML or software role, then the
> categorical shape.

## How To Use This Glossary

Use each entry as a bridge, not as a final definition.

```text
term -> Rust handle -> ML or software role -> category-theory shape
```

If a term has no Rust handle in this repository, it is not a core term for this
book yet. The goal is not to collect impressive vocabulary. The goal is to make
the vocabulary already used by the chapters easier to retrieve and transfer.

When a term appears in a chapter, ask:

```text
What value, function, trait, constructor, method, test, or command makes this
term concrete?
```

That question keeps the glossary grounded.

## Core Term Alignment

Some ideas have a public phrase, a Rust type, and a category-theory reading.
Use this table to keep them separate.

| Public phrase | Rust handle | Use this wording when precision matters |
| --- | --- | --- |
| training pairs | `TrainingExample` values inside `TrainingSet` | "adjacent input-target pairs" for the examples, `TrainingSet` for the validated Rust object |
| model state | `Parameters` | "parameters" when naming the Rust object, "model state" when explaining the ML role |
| probabilities | `Distribution` | "probabilities" for intuition, `Distribution` when the constructor invariant matters |
| query sequence | `QuerySequence` | "queries" for intuition, `QuerySequence` when the attention role matters |
| key sequence | `KeySequence` | "keys" for intuition, `KeySequence` when score construction needs the matching head dimension |
| value sequence | `ValueSequence` | "values" for intuition, `ValueSequence` when attention weights need source rows to mix |
| target sequence length | `QuerySequence` row count | "target length" for intuition, `L` when contrasting target positions with source positions |
| source sequence length | `KeySequence` and `ValueSequence` row count | "source length" for intuition, `S` when the positions being read may differ from target positions |
| attention score rows | `AttentionScores` | "scores" for intuition, `AttentionScores` when the row shape must be validated |
| attention mask | `AttentionMask` | "allowed positions" for intuition, `AttentionMask` when illegal score positions must be removed before softmax |
| attention weights | `AttentionWeights` | "weights" for intuition, `AttentionWeights` when each query row must sum to one |
| attention output | `AttentionOutput` | "mixed values" for intuition, `AttentionOutput` when one output row per query matters |
| head count | `HeadCount` | "number of heads" for intuition, `HeadCount` when zero heads must be rejected |
| head outputs | `AttentionHeadOutputs` | "outputs from several heads" for intuition, `AttentionHeadOutputs` when all heads must share sequence length and width |
| multi-head output | `MultiHeadOutput` | "concatenated heads" for intuition, `MultiHeadOutput` when the combined model dimension matters |
| attention output projection | `AttentionOutputProjection` | "projection after head concatenation" for intuition, `AttentionOutputProjection` when matrix shape must be validated |
| projected attention output | `ProjectedAttentionOutput` | "projected attention sequence" for intuition, `ProjectedAttentionOutput` when the post-projection width matters |
| hidden sequence | `HiddenSequence` | "sequence of hidden vectors" for intuition, `HiddenSequence` when residual shape must be protected |
| hidden-to-query projection | `HiddenToQuery` | "make query vectors from hidden rows" for intuition, `HiddenToQuery` when projection shape must be validated |
| hidden-to-key projection | `HiddenToKey` | "make key vectors from hidden rows" for intuition, `HiddenToKey` when projection shape must be validated |
| hidden-to-value projection | `HiddenToValue` | "make value vectors from hidden rows" for intuition, `HiddenToValue` when projection shape must be validated |
| residual connection | `ResidualConnection` | "add the sublayer output back" for intuition, `ResidualConnection` when sequence length and width must match |
| layer normalization | `LayerNormalization` | "normalize each hidden vector" for intuition, `LayerNormalization` when feature-wise normalization must preserve shape |
| layer norm parameters | `LayerNormParameters` | "scale, shift, epsilon" for intuition, `LayerNormParameters` when parameter dimensions must be validated |
| position-wise feed-forward | `PositionWiseFeedForward` | "same non-linear map at each sequence position" for intuition, `PositionWiseFeedForward` when two-layer shape checks must preserve hidden width |
| positional encoding | `PositionalEncoding` | "add position rows" for intuition, `PositionalEncoding` when sequence length and model width must be checked |
| self-attention | `SelfAttentionHead`, `MultiHeadTransformerBlock` | "same hidden sequence supplies query, key, and value roles" for intuition, self-attention when source ownership matters |
| cross-attention | `QuerySequence`, `KeySequence`, `ValueSequence` | "target sequence reads a separate source sequence" for intuition; the repository names the boundary but does not implement a full cross-attention block yet |
| single-head block | `SingleHeadTransformerBlock` | "one block-shaped sketch" for intuition, `SingleHeadTransformerBlock` when the whole boundary should preserve hidden sequence shape |
| self-attention head | `SelfAttentionHead` | "one query/key/value projection triple" for intuition, `SelfAttentionHead` when one head's role dimensions must be validated |
| multi-head block | `MultiHeadTransformerBlock` | "several heads as one block" for intuition, `MultiHeadTransformerBlock` when head count and output-projection shape must be validated |
| masked multi-head block | `MaskedMultiHeadTransformerBlock` | "block with allowed attention positions" for intuition, `MaskedMultiHeadTransformerBlock` when the mask joins hidden state at the block boundary |
| sequence logits | `SequenceLogits` | "vocabulary scores at each sequence position" for intuition, `SequenceLogits` when sequence length and vocabulary width must be explicit |
| Transformer readout | `TransformerReadout` | "sequence language-model head" for intuition, `TransformerReadout` when hidden width and vocabulary width must be validated |
| tiny Transformer parameters | `TinyTransformerParameters` | "position plus block plus readout" for intuition, `TinyTransformerParameters` when named model roles should move together |
| Transformer training state | `TransformerTrainingState` | "parameters plus optimizer metadata" for intuition, `TransformerTrainingState` when step count and learning rate matter |
| Transformer readout training example | `TransformerReadoutTrainingExample` | "one fixed hidden sequence with target tokens" for intuition, `TransformerReadoutTrainingExample` when hidden, mask, and target lengths must match |
| Transformer readout train step | `TransformerReadoutTrainStep` | "readout-only update" for intuition, `TransformerReadoutTrainStep` when the state endomorphism matters |
| Transformer feed-forward training example | `TransformerFeedForwardTrainingExample` | "one hidden-sequence input and target" for intuition, `TransformerFeedForwardTrainingExample` when feed-forward local training shape must match |
| Transformer feed-forward train step | `TransformerFeedForwardTrainStep` | "local feed-forward update" for intuition, `TransformerFeedForwardTrainStep` when the state endomorphism matters |
| Transformer block training example | `TransformerBlockTrainingExample` | "one sequence-to-token supervised example" for intuition, `TransformerBlockTrainingExample` when hidden, mask, and target lengths must match |
| Transformer block train step | `TransformerBlockTrainStep` | "composed readout-plus-feed-forward update" for intuition, `TransformerBlockTrainStep` when a sequence loss updates more than one parameter group |
| typed transformation | `Morphism<Input, Output>` | "typed transformation" before abstraction, "morphism" once the Rust trait is in view |
| product-input morphism | `Product<A, B>` at the input boundary | "needs two named inputs" before abstraction, product-input morphism when the arrow shape is `A x B -> C` |
| update step | `TrainStep` | "training step" for ML behavior, "endomorphism" for the `Parameters -> Parameters` shape |

This alignment prevents two common confusions. First, not every prose phrase is
a Rust type. Second, not every Rust type is a new mathematical concept. The book
uses plain phrases for intuition, Rust names for exact code, and
category-theory words only when the shape is visible.

## Category-Theory Terms

## Object

Rust syntax:

```text
TokenId
Vector
Logits
Distribution
Loss
Parameters
```

ML concept:

An object is one kind of value in the pipeline, such as a token, vector,
probability distribution, loss, or model state.

Category theory concept:

An object is something a morphism can start from or end at.

First-principles reading:

An object is the kind of thing an arrow is allowed to receive or return. In this
book, `TokenId` and `Vector` are different objects because the pipeline should
not confuse a vocabulary index with a dense numeric representation.

## Morphism

Rust syntax:

```rust,ignore
pub trait Morphism<Input, Output>
```

ML concept:

A morphism is one transformation stage, such as embedding lookup or softmax.

Category theory concept:

A morphism is a typed arrow:

```text
Input -> Output
```

First-principles reading:

In this book, "morphism" usually means "a named transformation with an input
type, an output type, and a possible typed error." The abstract name is useful
only because the Rust code makes the boundary inspectable.

## Identity Morphism

Rust syntax:

```rust,ignore
Identity<T>
```

ML concept:

Identity is a stage that leaves a value unchanged. It is useful for testing the
idea of neutral transformations.

Category theory concept:

Every object has an identity arrow:

```text
id_A : A -> A
```

## Composition

Rust syntax:

```rust,ignore
Compose<F, G, Middle>
```

ML concept:

Composition connects stages:

```text
Embedding then LinearToLogits then Softmax
```

Category theory concept:

If:

```text
f : A -> B
g : B -> C
```

then:

```text
g after f : A -> C
```

First-principles reading:

Composition is the reason the middle type matters. If the first stage produces
`Vector`, the next stage must accept `Vector`. A compiler error at this point is
useful evidence: the pipeline is missing or misordering a stage.

## Product Object

Rust syntax:

```rust,ignore
Product<A, B>
```

ML concept:

A product stores paired values, such as:

```text
input token x target token
prediction distribution x target token
```

Category theory concept:

The product object is written:

```text
A x B
```

Its projections correspond to `first()` and `second()`.

## Product-Input Morphism

Rust syntax:

```rust,ignore
Product<A, B> -> C
ScaledDotProductScores : QuerySequence x KeySequence -> AttentionScores
WeightedValueMixing : AttentionWeights x ValueSequence -> AttentionOutput
```

ML or software concept:

Some transformations need two meaningful inputs at the boundary. Attention
scoring needs target-side queries and source-side keys. Value mixing needs
attention weights and the source values being mixed.

Category theory concept:

A product-input morphism has a product object as its input:

```text
A x B -> C
```

First-principles reading:

Do not erase the product just because the output has a familiar type. The
product names the fact that two inputs must agree before the transformation is
legal.

## Law

Rust syntax:

```rust,ignore
assert_eq!(...)
information_order_obeys_preorder_laws()
pipeline_trace_obeys_monoid_laws()
```

ML or software concept:

A law is expected behavior that should keep working after implementation
details change.

Category theory concept:

A law states the structure a model must preserve, such as identity,
associativity, reflexivity, transitivity, or composition preservation.

First-principles reading:

A law is not decoration. In this repository, a law should have a nearby test or
check. Otherwise the reader has no executable reason to trust the word.

## Endomorphism

Rust syntax:

```rust,ignore
Endomorphism<T>
TrainStep : Parameters -> Parameters
```

ML concept:

A training step updates parameters and returns parameters again.

Category theory concept:

An endomorphism is an arrow from an object back to itself:

```text
A -> A
```

## Functor

Rust syntax:

```rust,ignore
Functor<A, B>
VecFunctor
OptionFunctor
```

ML concept:

Apply a transformation inside a wrapper such as a batch or optional value.

Category theory concept:

A functor maps objects and arrows while preserving structure.

First-principles reading:

For this book, the simplest functor intuition is `map`: apply a function inside
a context without destroying the context. `VecFunctor` preserves the list
shape. `OptionFunctor` preserves the difference between `Some` and `None`.

## Functor Map

Rust syntax:

```rust,ignore
fn map<U>(self, f: impl Fn(T) -> U) -> Distribution<U>
```

ML concept:

For a probabilistic output, `map` transforms every possible outcome while
leaving the attached probabilities unchanged.

Category theory concept:

`map` lifts a deterministic function:

```text
T -> U
```

into a context-aware transformation:

```text
Distribution<T> -> Distribution<U>
```

## Natural Transformation

Rust syntax:

```rust,ignore
VecToFirstOption : Vec<A> -> Option<A>
```

ML concept:

Convert one container shape into another consistently, such as many candidates
to maybe one selected candidate.

Category theory concept:

A natural transformation converts one functor shape into another and commutes
with mapping.

## Monoid

Rust syntax:

```rust,ignore
PipelineTrace
Monoid::empty()
Monoid::combine()
```

ML concept:

Traces, logs, batches, and metric accumulators often need an empty value and a
combine operation.

Category theory concept:

A monoid has an identity element and an associative binary operation.

First-principles reading:

A monoid is the structure behind "start empty, then combine many pieces." That
is why traces, logs, resource bundles, and accumulated updates are good
software examples.

## Preorder

Rust syntax:

```rust,ignore
InformationLevel::can_flow_to
```

ML or software concept:

Information can flow from observation to feature to score to decision.

Category theory concept:

A preorder is reflexive and transitive.

First-principles reading:

In code, a preorder often appears as a "can flow to," "can supply," or "is no
more than" relation. The important part is not sorting. The important part is
that repeated comparisons remain coherent.

## Galois Connection

Rust syntax:

```rust,ignore
abstract_to_layer_budget
concretize_layer_budget
```

ML or software concept:

Concrete feature counts and abstract layer budgets can be coordinated.

Category theory concept:

Two order-preserving views are connected by a law:

```text
abstract(x) <= y iff x <= concretize(y)
```

## Monoidal Preorder

Rust syntax:

```rust,ignore
ResourceBundle::tensor
ResourceBundle::can_supply
```

ML or software concept:

Independent compute and memory resources can be combined.

Category theory concept:

A preorder with a product-like composition operation that preserves order.

## Profunctor

Rust syntax:

```rust,ignore
FeasibilityRelation::relates(requirement, offer)
```

ML or software concept:

A requirement and implementation offer are related if constraints are
satisfied.

Category theory concept:

A profunctor generalizes a relationship between categories. This course uses a
small Bool-valued relation as the practical handle.

## Functorial Semantics

Rust syntax:

```rust,ignore
SignalMatrix::compose_after
```

ML or software concept:

Composed signal-flow stages should have the same meaning as composing their
matrix interpretations.

Category theory concept:

Interpretation preserves composition.

## Open System

Rust syntax:

```rust,ignore
OpenCircuit
OpenCircuit::then
OpenCircuit::parallel
```

ML or software concept:

A component has an external interface plus internal implementation details.

Category theory concept:

An open system composes through typed boundaries.

## Commutative Diagram

Rust syntax:

```rust,ignore
composed_and_direct_prediction_match()
naturality_square_commutes()
```

ML or software concept:

Two different implementation paths should produce the same result.

Category theory concept:

A commutative diagram says that following one route through a diagram has the
same meaning as following another route with the same start and end.

First-principles reading:

In this book, do not imagine a diagram first. Imagine two Rust expressions that
should agree. The diagram is the picture of that agreement.

## Sheaf-Style Locality

Rust syntax:

```rust,ignore
SafetyCover::global_truth
```

ML or software concept:

Local safety checks over time intervals combine into a global safety result.

Category theory concept:

Local facts can determine a global fact when they glue coherently.

## Boundary

Rust syntax:

```rust,ignore
Distribution::new
TrainingSet::new
SignalMatrix::compose_after
OpenCircuit::then
```

ML or software concept:

A boundary is where invalid structure should be rejected before it spreads
through the pipeline.

Category theory concept:

A boundary protects the intended object, morphism, relation, or composition
from accepting values outside its domain.

First-principles reading:

Many exercises ask what a type or method prevents. That is a boundary question.
Good boundaries make wrong connections hard to express.

## Rust Terms

## Newtype

Rust syntax:

```rust,ignore
pub struct TokenId(usize);
```

ML concept:

The same raw number type can represent different concepts. Newtypes prevent
accidental mixing.

Category theory concept:

A newtype names a specific object instead of treating all raw representations
as the same object.

First-principles reading:

A newtype is the smallest move from "just data" to "data with a role." The
runtime representation can stay cheap, but the type checker now knows that a
token id, vocabulary size, and model dimension are not the same concept.

## Smart Constructor

Rust syntax:

```rust,ignore
pub fn new(value: Raw) -> CtResult<Self>
```

ML concept:

Invalid training inputs, probabilities, dimensions, or hyperparameters should
be rejected early.

Category theory concept:

A smart constructor maps raw data into a validated subobject, using `Result`
when the mapping can fail.

## Invariant

Rust syntax:

```text
Distribution must be non-empty, finite, non-negative, and sum to one.
```

ML concept:

The model can trust a value only if the type protects the rule that makes it
meaningful.

Category theory concept:

An invariant describes the subset or structure the object is meant to inhabit.

## Typed Error

Rust syntax:

```rust,ignore
CtError
CtResult<T>
```

ML concept:

Bad data should fail with a meaningful cause, not with a vague panic later.

Category theory concept:

`Result` turns a partial construction or morphism into a total error-aware
mapping.

## Negative Test

Rust syntax:

```rust,ignore
assert!(matches!(..., Err(...)))
```

ML or software concept:

A negative test proves that invalid data or an invalid connection is rejected.

Category theory concept:

It checks that a proposed object, relation, or composition is not admitted when
the required structure is missing.

First-principles reading:

Positive tests show what works. Negative tests show what the boundary protects.
Both are needed when a chapter claims that types make structure explicit.

## Machine-Learning Terms

## Token

Rust syntax:

```rust,ignore
TokenId
```

ML concept:

A token is a discrete symbol from a vocabulary.

Category theory concept:

The vocabulary is a finite discrete set of possible token objects.

## Training Example

Rust syntax:

```rust,ignore
pub type TrainingExample = Product<TokenId, TokenId>;
```

ML concept:

A training example pairs an input token with the target token that follows it.

Category theory concept:

It is a product object:

```text
TokenId x TokenId
```

First-principles reading:

The product matters because the loss function needs both parts: the prediction
derived from the first token and the target represented by the second token.

## Training Set

Rust syntax:

```rust,ignore
TrainingSet
DatasetWindowing : TokenSequence -> TrainingSet
```

ML concept:

A training set is a non-empty collection of adjacent next-token examples.

Category theory concept:

It is an object produced by a data-preparation morphism and consumed by the
training update.

## Embedding

Rust syntax:

```rust,ignore
Embedding : TokenId -> Vector
```

ML concept:

An embedding maps a discrete token to a dense numerical representation.

Category theory concept:

It is a morphism from a finite token object into a vector-space-like object.

## Logits

Rust syntax:

```rust,ignore
Logits(Vec<f32>)
```

ML concept:

Logits are raw scores before softmax.

Category theory concept:

They live in a vector-space-like object:

```text
R^vocab_size
```

## Softmax

Rust syntax:

```rust,ignore
Softmax : Logits -> Distribution
```

ML concept:

Softmax turns raw scores into probabilities.

Category theory concept:

It maps from a score vector into the probability simplex.

## Distribution

Rust syntax:

```rust,ignore
Distribution
Distribution::new
```

ML concept:

A distribution is a probability vector over possible next tokens. Its values
must be finite, non-negative, non-empty, and sum to one.

Category theory concept:

It is the object produced by softmax and consumed with a target token to produce
loss.

First-principles reading:

A raw vector can contain any numbers. A `Distribution` is a vector that has
earned the right to be read as probabilities.

## Cross Entropy

Rust syntax:

```rust,ignore
CrossEntropy : Product<Distribution, TokenId> -> Loss
```

ML concept:

Cross entropy measures how much probability the model assigned to the correct
target.

Category theory concept:

It is a morphism from prediction-target product into non-negative scalar loss.

## Loss

Rust syntax:

```rust,ignore
Loss
Loss::new
```

ML concept:

Loss is a scalar penalty. Lower loss means the model assigned more probability
to the correct target in this tiny pipeline.

Category theory concept:

Loss is the output object of the evaluation morphism:

```text
Distribution x TokenId -> Loss
```

## Parameters

Rust syntax:

```rust,ignore
Parameters
```

ML concept:

The trainable state of the model: embedding table, output head, and bias.

Category theory concept:

The object transformed by the training endomorphism.

First-principles reading:

The word "state" can be vague. In this book, the model state is concrete:
embedding table, output head, and bias. Training means returning a new value of
the same `Parameters` type.

## Gradient

Rust syntax:

```rust,ignore
LocalGradient
grad_embedding
grad_lm_head
grad_bias
```

ML concept:

A gradient tells how parameters should change to reduce loss.

Category theory concept:

Gradient flow is local derivative information composed backward through a
composed computation.

## Learning Rate

Rust syntax:

```rust,ignore
LearningRate
```

ML concept:

The scalar step size in gradient descent.

Category theory concept:

It chooses a specific update morphism from a family of parameter endomorphisms.

## End-To-End Pipeline

Rust syntax:

```text
TokenSequence -> TrainingSet
TokenId -> Vector -> Logits -> Distribution
Distribution x TokenId -> Loss
Parameters -> Parameters
```

ML concept:

The full tiny system turns text into training examples, predicts a next-token
distribution, evaluates loss, and updates parameters.

Category theory concept:

The full pipeline is a collection of composable typed transformations, with
training represented as a repeatable endomorphism on model state.

## Chain Rule

Rust syntax:

```rust,ignore
MulOp::backward
```

ML concept:

The chain rule lets local derivatives combine into gradients for a larger
computation.

Category theory concept:

It is composition of local derivative maps.

## Target And Source Sequence Length

Rust syntax:

```rust,ignore
QuerySequence
KeySequence
ValueSequence
AttentionScores
AttentionMask
```

ML concept:

The target sequence length is the number of query positions that ask for
information. The source sequence length is the number of key-value positions
that can be read. In self-attention they are often the same sequence. In
cross-attention they can come from different sequences.

Category theory concept:

The attention boundary keeps two roles visible:

```text
Target positions x Source positions -> attention weights
```

First-principles reading:

This is why the book uses role-specific names instead of one generic matrix
name. A mask of shape `L x S` answers a concrete question: for each target
position, which source positions may be read?

## Attention Scores

Rust syntax:

```rust,ignore
QuerySequence
KeySequence
ScaledDotProductScores : QuerySequence x KeySequence -> AttentionScores
AttentionScores
```

ML concept:

Attention scores are query-by-key compatibility values before softmax. The
scaled dot-product boundary computes one score for each query and key pair.

Category theory concept:

`ScaledDotProductScores` is a morphism from a product of role-specific sequence
objects into a score table. `AttentionScores` is an object whose rows can be
transformed into probability-like attention weights.

First-principles reading:

The shape matters. Query and key sequences may have different lengths, but they
must share the same head dimension before dot products make sense. A score
table must have at least one query row, at least one key column, and the same
number of key columns in every row.

## Hidden-To-Role Projections

Rust syntax:

```rust,ignore
HiddenToQuery : HiddenSequence -> QuerySequence
HiddenToKey : HiddenSequence -> KeySequence
HiddenToValue : HiddenSequence -> ValueSequence
```

ML concept:

Self-attention begins by projecting hidden states into query, key, and value
roles. The rows may all be numbers, but the roles are not interchangeable.

Category theory concept:

These are parallel morphisms from one source object:

```text
HiddenSequence -> QuerySequence
HiddenSequence -> KeySequence
HiddenSequence -> ValueSequence
```

First-principles reading:

The projection constructors validate matrix shape and finite values. The
application step checks that the hidden sequence width matches the projection
input width before producing role-specific sequence objects.

## Self-Attention

Rust syntax:

```rust,ignore
SelfAttentionHead
MultiHeadTransformerBlock : HiddenSequence -> HiddenSequence
```

ML concept:

Self-attention means the query, key, and value roles all come from the same
hidden sequence. The roles are still distinct after projection, but their
source ownership is shared.

Category theory concept:

The internal attention path still contains product-input boundaries:

```text
QuerySequence x KeySequence -> AttentionScores
AttentionWeights x ValueSequence -> AttentionOutput
```

The surrounding block can have endomorphism shape only after the internal
composition returns to the same public object:

```text
HiddenSequence -> HiddenSequence
```

First-principles reading:

Self-attention is not permission to call every internal step an endomorphism.
It is the case where one source hidden sequence is projected into the query,
key, and value roles before scoring and mixing.

## Cross-Attention

Rust syntax:

```rust,ignore
QuerySequence
KeySequence
ValueSequence
```

ML concept:

Cross-attention means the target-side query sequence reads from a separate
source-side key-value sequence. The current repository names this boundary for
precision, but it does not yet implement a full cross-attention block.

Category theory concept:

The source split makes the product input impossible to hide:

```text
TargetHiddenSequence -> QuerySequence
SourceHiddenSequence -> KeySequence
SourceHiddenSequence -> ValueSequence
QuerySequence x KeySequence -> AttentionScores
AttentionWeights x ValueSequence -> AttentionOutput
```

First-principles reading:

When the target sequence and source sequence are not the same object, the
attention map has target rows and source columns. That is the shape reason to
keep `L` and `S` separate in explanations, masks, and tests.

## Attention Mask

Rust syntax:

```rust,ignore
AttentionMask
MaskedAttentionScores : AttentionScores x AttentionMask -> AttentionScores
```

ML concept:

An attention mask marks which key positions each query is allowed to attend to.
Disallowed score positions become a large negative value before softmax, so
their probability becomes negligible.

Category theory concept:

`MaskedAttentionScores` is a typed morphism from a product object back to the
score object:

```text
AttentionScores x AttentionMask -> AttentionScores
```

First-principles reading:

Every mask row must allow at least one key. Otherwise softmax would be asked to
choose among no legal positions.

## Attention Weights

Rust syntax:

```rust,ignore
AttentionWeights
AttentionSoftmax : AttentionScores -> AttentionWeights
```

ML concept:

Attention weights are row-wise probabilities over key positions. Each query
position receives its own distribution over the positions it can attend to.

Category theory concept:

`AttentionSoftmax` is a typed morphism from raw score rows to validated
probability rows.

First-principles reading:

This is one Transformer-roadmap boundary made executable in the crate. It
validates the probability-like score-to-weight step after query-key scoring and
masking have produced legal score rows.

## Value Mixing

Rust syntax:

```rust,ignore
ValueSequence
WeightedValueMixing : AttentionWeights x ValueSequence -> AttentionOutput
AttentionOutput
```

ML concept:

Value mixing uses each query row of attention weights to compute a weighted sum
of value vectors. The result has one output vector per query position.

Category theory concept:

`WeightedValueMixing` is a morphism from a product object to an output object:

```text
AttentionWeights x ValueSequence -> AttentionOutput
```

First-principles reading:

The key length of the weights must match the number of value rows. If a query
has weights over three source positions, the value sequence must provide three
source vectors to mix.

## Multi-Head Concatenation

Rust syntax:

```rust,ignore
HeadCount
AttentionHeadOutputs
ConcatenateHeads : AttentionHeadOutputs -> MultiHeadOutput
MultiHeadOutput
```

ML concept:

Several attention heads can produce one output sequence each. Concatenation
combines the feature vectors at each sequence position so later layers can read
all head outputs together.

Category theory concept:

`ConcatenateHeads` is a recombination morphism:

```text
AttentionHeadOutputs -> MultiHeadOutput
```

First-principles reading:

The constructor checks that every head has the same sequence length and head
dimension before concatenation. The resulting model dimension is the head count
multiplied by the head dimension. This is the typed boundary where separate
head outputs become one combined object.

## Attention Output Projection

Rust syntax:

```rust,ignore
AttentionOutputProjection
AttentionOutputProjection : MultiHeadOutput -> ProjectedAttentionOutput
ProjectedAttentionOutput
```

ML concept:

After head outputs are concatenated, a learned linear projection mixes features
across heads and returns the sequence to the width expected by the surrounding
model block.

Category theory concept:

`AttentionOutputProjection` is a morphism:

```text
MultiHeadOutput -> ProjectedAttentionOutput
```

First-principles reading:

The projection validates its matrix and bias before use. It also checks that
the `MultiHeadOutput` width matches the projection input width. This keeps the
post-concatenation linear map from becoming an untyped matrix multiply hidden
inside the example.

## Residual Connection

Rust syntax:

```rust,ignore
HiddenSequence
ResidualConnection : HiddenSequence x ProjectedAttentionOutput -> HiddenSequence
```

ML concept:

A residual connection adds a sublayer output back to the hidden sequence it
came from. The addition is only meaningful when every sequence position has the
same hidden width on both sides.

Category theory concept:

`ResidualConnection` is a product-to-object morphism:

```text
HiddenSequence x ProjectedAttentionOutput -> HiddenSequence
```

The larger Transformer block can still have endomorphism shape:

```text
HiddenSequence -> HiddenSequence
```

First-principles reading:

Residual addition is not just vector arithmetic. It is a shape contract. The
sequence length and model dimension must match before addition can preserve the
hidden sequence object.

## Layer Normalization

Rust syntax:

```rust,ignore
LayerNormParameters
LayerNormalization : HiddenSequence -> HiddenSequence
```

ML concept:

Layer normalization normalizes each hidden vector across its feature dimension.
It keeps the sequence length and model dimension unchanged.

Category theory concept:

`LayerNormalization` is an endomorphism:

```text
HiddenSequence -> HiddenSequence
```

First-principles reading:

The operation changes values, not the object type. The parameter object protects
the scale, shift, and epsilon invariants before a hidden sequence can be
normalized.

## Position-Wise Feed-Forward

Rust syntax:

```rust,ignore
PositionWiseFeedForward : HiddenSequence -> HiddenSequence
```

ML concept:

A position-wise feed-forward network applies the same two-layer non-linear map
to every hidden vector in the sequence. It can expand the feature dimension
internally, apply an activation, then project back to the original model
dimension.

Category theory concept:

`PositionWiseFeedForward` is an endomorphism:

```text
HiddenSequence -> HiddenSequence
```

First-principles reading:

The internal feed-forward width is allowed to differ from the model dimension,
but the public output must return to the same hidden sequence shape. The type
protects that shape before later blocks try to compose with it.

## Positional Encoding

Rust syntax:

```rust,ignore
PositionalEncoding : HiddenSequence -> HiddenSequence
```

ML concept:

Position information lets a sequence model distinguish the first token from the
second token even when their content vectors are otherwise similar.

Category theory concept:

`PositionalEncoding` is an endomorphism:

```text
HiddenSequence -> HiddenSequence
```

First-principles reading:

The encoding table must have enough rows for the hidden sequence and the same
model width. Adding position changes the values at each row, not the public
shape of the hidden sequence.

## Single-Head Transformer Block

Rust syntax:

```rust,ignore
SingleHeadTransformerBlock : HiddenSequence -> HiddenSequence
```

ML concept:

The single-head block sketch composes hidden-to-role projections, attention,
output projection, residual addition, normalization, and a feed-forward
sublayer. It is intentionally small: one head and no production training
machinery.

Category theory concept:

`SingleHeadTransformerBlock` is an endomorphism:

```text
HiddenSequence -> HiddenSequence
```

First-principles reading:

The block is useful because it hides internal steps without hiding shape
contracts. The caller sees one sequence-preserving transformation; the
constructor still checks the dimensions that make the internal composition
legal.

## Multi-Head Transformer Block

Rust syntax:

```rust,ignore
SelfAttentionHead
MultiHeadTransformerBlock : HiddenSequence -> HiddenSequence
```

ML concept:

A multi-head block applies several self-attention heads in parallel, concatenates
their outputs, projects back to the model dimension, then applies the same
residual, normalization, and feed-forward shape-preserving pattern.

Category theory concept:

`MultiHeadTransformerBlock` is an endomorphism:

```text
HiddenSequence -> HiddenSequence
```

First-principles reading:

The block checks that every head accepts the same hidden width, every value head
has the same output width, and the output projection expects exactly:

```text
head_count * value_head_dimension
```

Those checks keep multi-head attention as explicit structure rather than an
unlabeled matrix pile.

## Masked Multi-Head Transformer Block

Rust syntax:

```rust,ignore
MaskedMultiHeadTransformerBlock : HiddenSequence x AttentionMask -> HiddenSequence
```

ML concept:

A masked block runs the same multi-head path while preventing disallowed
query-key positions from receiving attention probability.

Category theory concept:

`MaskedMultiHeadTransformerBlock` consumes a product object:

```text
HiddenSequence x AttentionMask -> HiddenSequence
```

First-principles reading:

The mask is not a side channel. It is an explicit input to the block. The mask
shape must match the query-by-key score table produced inside each head.

## Sequence Logits

Rust syntax:

```rust,ignore
SequenceLogits
```

ML concept:

Sequence logits are unnormalized vocabulary scores for each position in a
hidden sequence.

Category theory concept:

They are the output object of a sequence-level readout morphism:

```text
HiddenSequence -> SequenceLogits
```

First-principles reading:

The object keeps sequence length and vocabulary size explicit. That prevents a
sequence readout from becoming an unlabeled table of floats.

## Transformer Readout

Rust syntax:

```rust,ignore
TransformerReadout : HiddenSequence -> SequenceLogits
```

ML concept:

A readout maps each final hidden vector to vocabulary scores. It is the
sequence-level version of the earlier `Vector -> Logits` language-model head.

Category theory concept:

`TransformerReadout` is a morphism from hidden sequence object to sequence
logit object:

```text
HiddenSequence -> SequenceLogits
```

First-principles reading:

The readout validates the input model dimension and vocabulary width before
projecting rows. The model should fail at the boundary, not inside an indexing
loop.

## Tiny Transformer Parameters

Rust syntax:

```rust,ignore
TinyTransformerParameters : HiddenSequence x AttentionMask -> SequenceLogits
```

ML concept:

The parameter object owns the position table, masked block, and sequence
readout needed for the tiny Transformer forward path.

Category theory concept:

It is a product-to-object morphism:

```text
HiddenSequence x AttentionMask -> SequenceLogits
```

First-principles reading:

The object groups named roles. The point is not to claim a production
Transformer; the point is to stop passing unrelated matrices as loose
arguments.

## Transformer Training State

Rust syntax:

```rust,ignore
TransformerTrainingState
```

ML concept:

A training state owns parameters, a learning rate, and a step count. The current
code can evaluate through the structured state and record that a new parameter
object belongs to the next step.

Category theory concept:

The forward path has shape:

```text
HiddenSequence x AttentionMask -> SequenceLogits
```

The optimizer updates the state with endomorphism shape:

```text
TransformerTrainingState -> TransformerTrainingState
```

First-principles reading:

This is honest scaffolding. It models the state boundary the current tiny
optimizer updates, without pretending that the teaching implementation is a
production Transformer trainer.

## Transformer Readout Training Example

Rust syntax:

```rust,ignore
TransformerReadoutTrainingExample
```

ML concept:

A readout training example pairs one hidden sequence and attention mask with a
target token at every sequence position.

Category theory concept:

It is a validated product-like learning object that feeds a training
endomorphism:

```text
TransformerTrainingState -> TransformerTrainingState
```

First-principles reading:

The hidden sequence length, mask shape, and target-token count must agree
before a training step can compute a meaningful loss.

## Transformer Readout Train Step

Rust syntax:

```rust,ignore
TransformerReadoutTrainStep : TransformerTrainingState -> TransformerTrainingState
```

ML concept:

This step updates only the sequence readout. It keeps the position table and
attention block fixed, computes softmax cross-entropy gradients at each
sequence position, updates the readout weights and bias, and increments the
step count.

Category theory concept:

The update is an endomorphism:

```text
TransformerTrainingState -> TransformerTrainingState
```

First-principles reading:

This is a real update with a narrow scope. It teaches how the structured state
can change without claiming that gradients already flow through every
Transformer block parameter.

## Transformer Feed-Forward Training Example

Rust syntax:

```rust,ignore
TransformerFeedForwardTrainingExample
```

ML concept:

A local feed-forward training example pairs a hidden-sequence input with a
hidden-sequence target. It trains the feed-forward sublayer as a small supervised
map before the book attempts full block gradients.

Category theory concept:

It is a validated training object for an endomorphism:

```text
TransformerTrainingState -> TransformerTrainingState
```

First-principles reading:

The input and target must have the same sequence length and model dimension.
Otherwise the squared-error training signal would compare incompatible hidden
objects.

## Transformer Feed-Forward Train Step

Rust syntax:

```rust,ignore
TransformerFeedForwardTrainStep : TransformerTrainingState -> TransformerTrainingState
```

ML concept:

This step updates the position-wise feed-forward sublayer. It computes a local
squared-error gradient through the second linear layer, the ReLU gate, and the
first linear layer. It leaves attention and readout parameters fixed.

Category theory concept:

The update is another endomorphism:

```text
TransformerTrainingState -> TransformerTrainingState
```

First-principles reading:

This is one layer deeper than readout-only training, but it is still not full
Transformer backpropagation. It is a deliberately scoped way to show that a
structured state can update an internal block component without erasing the
roles of the other components.

## Transformer Block Training Example

Rust syntax:

```rust,ignore
TransformerBlockTrainingExample
```

ML concept:

A block training example pairs an input hidden sequence and attention mask with
target tokens. The loss starts at sequence logits, not at a hand-written hidden
target.

Category theory concept:

It is a supervised object for a state endomorphism:

```text
TransformerTrainingState -> TransformerTrainingState
```

First-principles reading:

The hidden sequence length, mask shape, and target-token count must agree
because the training signal is position-wise. Every row in the hidden sequence
produces one vocabulary score row and expects one target token.

## Transformer Block Train Step

Rust syntax:

```rust,ignore
TransformerBlockTrainStep : TransformerTrainingState -> TransformerTrainingState
```

ML concept:

This step updates the sequence readout, position-wise feed-forward sublayer,
and attention output projection from the same token-level loss. It computes the
softmax cross-entropy gradient at the readout, backpropagates through the final
layer-normalization boundary and residual addition, updates the feed-forward
layers through the ReLU gate, then carries the signal through the attention
normalization and residual boundary to the attention output projection.

Category theory concept:

The update is a composed endomorphism:

```text
TransformerTrainingState -> TransformerTrainingState
```

First-principles reading:

This is the first update in the repository where a token prediction loss
reaches inside the Transformer block and updates the attention output
projection, query/key/value projections, and both layer-normalization
scale/shift parameter sets. It still keeps position encodings fixed. That
boundary is deliberate: the implemented step is real, but it is not pretending
to be a production training algorithm.

## Where This Leaves Us

The glossary is not a substitute for the chapters. It is the index of the
book's repeated translation habit. When a term feels unfamiliar, connect it back
to one of three things: the Rust syntax that names it, the ML or software role
that motivates it, and the categorical shape that explains how it composes.
