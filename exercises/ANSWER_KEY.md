# Exercise Answer Key And Facilitator Notes

This file gives expected reasoning for the public exercises. It is not a script
to memorize. Use it after attempting the exercise, or as facilitator guidance
when reviewing reader answers.

Strong answers follow the same three-lens shape:

```text
Rust syntax:
ML or software concept:
Category theory concept:
```

## Using Attempt Reports

When reviewing exercise feedback, prefer reports that name the exercise,
chapter, command run, first failure signal, confusing line or concept, expected
behavior, observed behavior, answer-key mismatch, and suggested rewrite.

Do not treat "I was confused" as enough evidence by itself. Convert the report
into one of these edit decisions:

- add a smaller worked example,
- sharpen the pass condition,
- add a debugging hint,
- clarify the answer-key reasoning,
- move the exercise later in the ladder.

## Book Chapter Exercises

### Exercise 1: Explain One Domain Type

Expected reasoning:

- `Vector`, `Logits`, `Distribution`, `Loss`, `TrainingSet`, and `Parameters`
  are not interchangeable even when they contain ordinary numbers.
- The Rust answer should name the wrapper and its constructor or accessor.
- The ML answer should name the pipeline role.
- The category-theory answer should name the object or product shape.

Example for `Distribution`:

```text
Rust syntax:
Distribution wraps a Vec<f32> and can only be built through Distribution::new.

ML concept:
It represents probabilities over next-token choices.

Category theory concept:
It is the object produced by Softmax and consumed with TokenId by CrossEntropy.
```

### Exercise 2: Add A Token

Expected reasoning:

- A new vocabulary item only matters for training pairs if it appears in the
  token sequence.
- A longer adjacent token sequence creates more overlapping windows.
- The command should still run without constructor errors.

Common correction:

If the learner adds vocabulary data but does not extend the token sequence, the
training-pair output will not change.

### Exercise 3: Trace `DatasetWindowing`

Expected pairs:

```text
(TokenId(4), TokenId(8))
(TokenId(8), TokenId(15))
(TokenId(15), TokenId(16))
```

Expected reasoning:

- `.windows(2)` creates overlapping adjacent slices.
- Next-token training needs the current token and the following token.
- Each example is a product object because it carries two values together:
  input and target.

### Exercise 4: Break A Composition

Expected reasoning:

The legal path is:

```text
TokenId -> Vector -> Logits -> Distribution
```

`Embedding` cannot compose directly with `Softmax` because:

```text
Embedding : TokenId -> Vector
Softmax   : Logits -> Distribution
```

The missing middle stage is `LinearToLogits`.

Facilitator note:

Do not let learners solve this by weakening types. The lesson is that the type
boundary correctly rejects the skipped prediction stage.

### Exercise 5: Change The Training Repetition Count

Expected reasoning:

- `StepCount` controls how many times the same `TrainStep` is applied.
- One step preserves parameter shape but may not reduce loss much.
- More steps usually reduce loss on this tiny dataset until the simple update
  rule reaches its limit.
- Repetition is legal because the shape is `Parameters -> Parameters`.

### Exercise 6: Explain `Distribution<T>::map`

Expected output:

```text
"Rust" -> 0.70
"."    -> 0.30
```

Expected reasoning:

- `into_iter()` consumes the old outcomes.
- The probabilities stay attached to their outcomes.
- The mapping lifts `TokenId -> String` into
  `Distribution<TokenId> -> Distribution<String>`.

### Exercise 7: Explain One Validation Boundary

Expected reasoning:

- The Rust answer should identify the condition returning `Err(...)`.
- The ML or software answer should identify the bad runtime state prevented.
- The category-theory answer should identify the object, relation, or
  composition being protected.

Example:

```text
Distribution::new rejects negative values and values that do not sum to one.
That prevents arbitrary score vectors from being treated as probabilities.
It protects the Distribution object expected by CrossEntropy.
```

### Exercise 8: Trace A Full Source File

Expected reasoning:

A strong five-sentence summary should name:

- the file's problem,
- the primary Rust types or traits,
- the ML or software role,
- the category-theory shape,
- the command or test that validates it.

Example for `src/ml.rs`:

```text
src/ml.rs implements the tiny prediction and loss pipeline.
Its main types are DatasetWindowing, Embedding, LinearToLogits, Softmax, and
CrossEntropy.
It models next-token data preparation, scoring, probability, and loss.
Categorically, it is a chain of morphisms plus a product-to-loss morphism.
cargo test ml::tests --lib validates its core behavior.
```

### Exercise 9: Connect One External Reference

Expected reasoning:

The answer should not only paste a link. It should explain what the larger
resource teaches that the tiny implementation compresses.

Good example:

```text
External resource: Dive into Deep Learning softmax regression.
Source file: src/ml.rs.
Rust syntax connection: Softmax implements Morphism<Logits, Distribution>.
ML concept connection: logits become normalized probabilities.
Category theory concept connection: Logits -> Distribution is a typed
transformation.
Difference: the course uses a tiny hand-written example, not a full tensor
library or minibatch training system.
```

### Exercise 10: Test One Sketch Law

Expected reasoning:

The learner should name both a positive law and a rejected invalid case.

Examples:

- `InformationLevel` checks reflexivity and transitivity.
- `SignalMatrix::compose_after` rejects mismatched middle dimensions.
- `OpenCircuit::then` rejects incompatible output/input boundaries.

Facilitator note:

This is the key transfer exercise from tiny ML into applied category theory.
Ask: "What invalid composition does this model prevent?"

### Exercise 11: Write A New Block Explanation

Expected reasoning:

A good explanation includes:

- the local problem,
- the exact code block,
- the Rust syntax,
- the ML or software role,
- the category-theory shape,
- a one-sentence mental model.

Reject answers that name a category term without pointing to the code that
supports it.

### Exercise 12: Trace Attention Shape Flow

Expected reasoning:

- `ScaledDotProductScores` builds one score row for each query position and
  one score column for each key position.
- `MaskedAttentionScores` preserves the score-table shape while removing
  disallowed positions before normalization.
- `AttentionSoftmax` changes raw scores into row-wise attention weights.
- `WeightedValueMixing` uses those weights to mix `ValueSequence` rows into an
  `AttentionOutput`.
- `ConcatenateHeads` turns `AttentionHeadOutputs` into `MultiHeadOutput`.
- `AttentionOutputProjection` maps `MultiHeadOutput` back into
  `ProjectedAttentionOutput`.
- `ResidualConnection` combines the original `HiddenSequence` with
  `ProjectedAttentionOutput` and returns `HiddenSequence`.
- `LayerNormalization` and `PositionWiseFeedForward` are shape-preserving
  transformations over `HiddenSequence`.

Expected three-lens answer:

```text
Rust syntax:
The example is a chain of named types: QuerySequence, KeySequence,
AttentionScores, AttentionWeights, AttentionOutput, MultiHeadOutput,
ProjectedAttentionOutput, and HiddenSequence. Constructors and morphisms reject
shape mismatches before the path reaches indexing or multiplication.

ML concept:
Scores are unnormalized similarity values. Weights are normalized rows over
allowed source positions. Value mixing uses those weights to read information,
the output projection returns to model width, residual addition preserves the
hidden-state object, and normalization/feed-forward refine the same sequence.

Category theory concept:
Query-key scoring and value mixing use product inputs:
QuerySequence x KeySequence and AttentionWeights x ValueSequence. The enclosing
block then returns to HiddenSequence, so residual, normalization, and
feed-forward can be composed as shape-preserving transformations.
```

Expected category-shape classifications:

| Boundary | Classification |
| --- | --- |
| `QuerySequence x KeySequence -> AttentionScores` | product-input morphism |
| `AttentionScores x AttentionMask -> AttentionScores` | product-input morphism returning the score object |
| `AttentionScores -> AttentionWeights` | ordinary morphism |
| `AttentionWeights x ValueSequence -> AttentionOutput` | product-input morphism |
| `LayerNormalization : HiddenSequence -> HiddenSequence` | shape-preserving endomorphism |
| `TransformerTrainingState -> TransformerTrainingState` | state endomorphism |
| `HiddenSequence x MultiHeadOutput -> HiddenSequence` | illegal boundary; projection is missing |

Naming rule:

```text
Count inputs first.

One input returning the same public object can be an endomorphism.
A product input returning the left object is still a product-input morphism.
An attempted product input with the wrong second object is illegal before it
gets a category-theory name.
```

For example, `HiddenSequence x ProjectedAttentionOutput -> HiddenSequence`
returns `HiddenSequence`, but it is not unary. The projected attention output
is extra evidence required by the residual operation, so the safe name is
product-input morphism returning hidden state.

Facilitator note:

Do not accept "attention calculates importance" as a complete answer. A strong
answer must separate scores, masks, weights, value rows, projection, residual
shape, and the final return to `HiddenSequence`. Also reject answers that call
every returning-to-same-object line an endomorphism. A binary boundary such as
`AttentionScores x AttentionMask -> AttentionScores` still depends on a product
input.

### Exercise 13: Compute Cross-Entropy From Target Probability

Expected computation:

```text
confident loss = -ln(0.90) ~= 0.105
surprised loss = -ln(0.10) ~= 2.303
lower loss = confident
```

Expected reasoning:

```text
Rust syntax:
CrossEntropy receives Product<Distribution, TokenId>, splits it into
distribution and target, reads distribution.as_slice()[target.index()], and
passes -probability.ln() to Loss::new.

ML concept:
The target token is TokenId(0) in both cases. The only difference is how much
probability the model assigned to that correct target. Higher target
probability means less surprise, so the loss is lower.

Category theory concept:
CrossEntropy is a product-to-object morphism:
Distribution x TokenId -> Loss. It cannot compute the loss from the
distribution alone because loss depends on which token was correct.
```

Facilitator note:

If a learner compares the largest probability in each distribution instead of
the probability at `TokenId(0)`, they have missed the supervision signal. Ask
them to point at the target index before calculating the logarithm.

### Exercise 14: Trace Naturality And Monoid Laws

Expected naturality reasoning:

```text
top then right:
VecFunctor::fmap(vec![1, 2, 3], |x| x * 10)
-> VecToFirstOption::transform(...)
-> Some(10)

left then bottom:
VecToFirstOption::transform(vec![1, 2, 3])
-> OptionFunctor::fmap(..., |x| x * 10)
-> Some(10)

why they should match:
the transform takes the first value uniformly, so mapping before selecting and
selecting before mapping both apply the same function to the same first item.
```

Expected monoid reasoning:

```text
left identity:
PipelineTrace::empty().combine(&a) == a

right identity:
a.combine(&PipelineTrace::empty()) == a

associativity:
a.combine(&b).combine(&c) == a.combine(&b.combine(&c))
```

Expected three-lens answer:

```text
Rust syntax:
naturality_square_holds_for_first_option builds two paths using VecFunctor,
OptionFunctor, and VecToFirstOption. monoid_laws_hold_for_pipeline_trace builds
three PipelineTrace values plus PipelineTrace::empty and checks combine.

ML or software concept:
Wrapper conversion should not depend on whether a value was transformed before
or after selecting the first item. Trace grouping should not change the ordered
record of which pipeline stages ran.

Category theory concept:
The naturality square commutes when both paths from Vec<i32> to Option<i32>
agree. PipelineTrace is a monoid because it has an identity trace and
associative trace combination.
```

Facilitator note:

If a learner says "both tests return true" without naming the two paths or the
three laws, send them back to the function bodies. The point is to connect the
law word to the exact executable check.

### Exercise 15: Mixed Boundary Diagnosis

Expected classifications:

| Case | Boundary type | Expected fix |
| --- | --- | --- |
| raw `usize` where `TokenId` is expected | invariant boundary | construct or pass `TokenId` instead of weakening the API |
| `Embedding` followed directly by `Softmax` | composition boundary | restore `LinearToLogits` so the path is `TokenId -> Vector -> Logits -> Distribution` |
| `CrossEntropy` receives `Logits` | composition boundary plus product-input boundary | run `Softmax` and pair `Distribution` with `TokenId` |
| training step returns `Loss` | endomorphism boundary | return updated `Parameters`; use loss only as measurement |
| `SignalMatrix::compose_after` sees mismatched middle dimensions | shape boundary | change rows or columns so the middle object matches |
| `SafetyCover` ignores one false interval | local-to-global boundary | make global truth fold all local checks with conjunction |
| residual connection adds different model dimensions | shape boundary | project or reject so both sides are `HiddenSequence` with the same model dimension |

Expected reasoning:

- Invalid values should be rejected at constructors or semantic wrappers.
- Invalid composition should be fixed by restoring the missing middle object,
  not by erasing types.
- Endomorphism failures should preserve the repeated update shape.
- Shape failures should name the exact dimension or object that failed to line
  up.
- Local-to-global failures should explain why one false local check blocks a
  true global claim.

Facilitator note:

This exercise is intentionally interleaved. If a learner solves only the cases
from the most recent chapter, send them back to the Exercise Evidence Map and
ask them to classify each failure signal before writing the three-lens answer.

Case 3 is the most important ML precision check. Do not accept an answer that
says only "use the right type." A strong answer must say that logits are
unnormalized scores, that `Softmax` must produce a validated `Distribution`,
and that `CrossEntropy` also needs the target token through the product shape
`Distribution x TokenId -> Loss`.

## Beginner Exercises

### Beginner 1: Change The Input Text

Expected reasoning:

Token ids may change, but adjacent-token pair structure remains the same:

```text
Text -> TokenSequence -> TrainingPairs
```

If there are fewer than two tokens, dataset windowing cannot produce a pair.

### Beginner 2: Find One Invariant

Expected examples:

- `Distribution::new` prevents invalid probability vectors.
- `LearningRate::new` prevents non-positive or non-finite learning rates.
- `TrainingSet::new` prevents empty training data.

### Beginner 3: Explain One Output Line

Expected reasoning:

The answer should connect terminal output to a source type or function. For
example, a training-pair line comes from `DatasetWindowing` and represents an
input-target product.

## Intermediate Exercises

### Intermediate 1: Add A Morphism

Expected reasoning:

The new morphism should have a visible shape:

```text
Input -> Output
```

It should use existing patterns from `src/category.rs` and should be validated
by a test or runnable example.

### Intermediate 2: Explain A Composition Failure

Expected reasoning:

The learner should report:

- the compiler error,
- the missing middle type,
- the corrected path.

The correction should add or restore the missing morphism, not erase the type
boundary.

### Intermediate 3: Add One Negative Test

Expected reasoning:

A good negative test proves the boundary. It should fail if the validation
branch is removed.

Examples:

- invalid probability mass returns `Err(...)`,
- empty training set returns `Err(...)`,
- mismatched matrix dimensions return `Err(...)`.

## Advanced Exercises

### Advanced 1: Add A Diagram-Backed Chapter Section

Expected reasoning:

The diagram must sit near prose explaining how to read it. It should name the
Rust values, the ML or software meaning, and the category-theory relationship.

### Advanced 2: Extend The Sketches Module

Expected reasoning:

The extension should add:

- one named typed value,
- one positive example,
- one boundary or law test,
- one chapter explanation.

The new code should match the small, inspectable style of `src/sketches.rs`.

### Advanced 3: Improve One Exercise

Expected reasoning:

The improved exercise should include:

- a file to inspect,
- a command to run,
- an expected output or failure shape,
- a short answer template.

Facilitator note:

The best exercise improvements reduce guessing. A reader should know where to
look, what to try, and how to tell whether the attempt taught the intended
idea.

### Advanced 4: Review The Attention Projection Boundary

Expected reasoning:

- `HeadCount` rejects zero heads.
- `AttentionHeadOutputs` rejects heads with different sequence lengths.
- `AttentionHeadOutputs` rejects heads with different head dimensions.
- `MultiHeadOutput` records the concatenated model dimension as
  `head_count * head_dimension`.
- `ConcatenateHeads` is a typed recombination morphism:

```text
AttentionHeadOutputs -> MultiHeadOutput
```

- `AttentionOutputProjection` validates non-empty weights, non-empty bias,
  finite values, output-column shape, and input width.
- `AttentionOutputProjection` is the current projection morphism:

```text
MultiHeadOutput -> ProjectedAttentionOutput
```

- `ResidualConnection` rejects sequence-length and model-dimension mismatches.
- `ResidualConnection` is the current residual morphism:

```text
HiddenSequence x ProjectedAttentionOutput -> HiddenSequence
```

- `LayerNormParameters` validates scale, shift, and epsilon.
- `LayerNormalization` is the current normalization endomorphism:

```text
HiddenSequence -> HiddenSequence
```

- `PositionWiseFeedForward` validates the first linear layer, the second
  linear layer, the shared internal feed-forward dimension, and the requirement
  that output width returns to input model width.
- `PositionWiseFeedForward` is the current feed-forward endomorphism:

```text
HiddenSequence -> HiddenSequence
```

- `PositionalEncoding` adds position rows while preserving the hidden sequence
  object:

```text
HiddenSequence -> HiddenSequence
```

- `HiddenToQuery`, `HiddenToKey`, and `HiddenToValue` make role projections
  explicit:

```text
HiddenSequence -> QuerySequence
HiddenSequence -> KeySequence
HiddenSequence -> ValueSequence
```

- `SingleHeadTransformerBlock` composes the current single-head attention,
  residual, normalization, and feed-forward boundaries:

```text
HiddenSequence -> HiddenSequence
```

- `SelfAttentionHead` groups the query, key, and value projections for one
  self-attention head.
- `MultiHeadTransformerBlock` composes several heads, concatenation, output
  projection, residual addition, normalization, and feed-forward structure:

```text
HiddenSequence -> HiddenSequence
```

- `MaskedMultiHeadTransformerBlock` makes the mask part of the block input:

```text
HiddenSequence x AttentionMask -> HiddenSequence
```

- `TransformerReadout` maps every hidden position to vocabulary scores:

```text
HiddenSequence -> SequenceLogits
```

- `TinyTransformerParameters` owns position, masked block, and readout pieces:

```text
HiddenSequence x AttentionMask -> SequenceLogits
```

- `TransformerTrainingState` owns the structured parameters, learning rate,
  and step count.
- `TransformerReadoutTrainStep` updates only the sequence readout:

```text
TransformerTrainingState -> TransformerTrainingState
```

- `TransformerFeedForwardTrainStep` updates the local position-wise
  feed-forward sublayer:

```text
TransformerTrainingState -> TransformerTrainingState
```

- `TransformerBlockTrainStep` updates the readout, feed-forward sublayer,
  attention output projection, query/key/value projections, and layer
  normalization parameters together from token targets:

```text
TransformerTrainingState -> TransformerTrainingState
```

The remaining future work is richer diagrams over that state.

Facilitator note:

Reject answers that say only "multi-head attention has many heads." A strong
answer identifies the exact invalid states that the Rust types now make harder
to express, including projection matrix shape, residual shape, layer-normalization
parameter shape, feed-forward layer shape, positional-encoding shape,
single-head block shape, multi-head block shape, masked block shape, readout
shape, parameter-object shape, training-state metadata, the readout-only
training update, the local feed-forward update, and the composed block update
through attention projections.

### Advanced 5: Explain A Finite-Difference Gradient Check

Expected answer shape:

```text
Rust syntax:
The test clones a tiny Transformer training state, perturbs one selected
parameter by epsilon in both directions, evaluates transformer_block_average_loss
for both perturbed states, and compares that central finite difference with the
gradient inferred from the one-step parameter update.

ML concept:
Backpropagation should produce a local slope for the loss with respect to each
trainable parameter. A finite-difference check estimates the same slope from
nearby losses. The two numbers should match closely when the analytic update
has the right sign and scale.

Category theory concept:
The training step is being checked as an endomorphism on TransformerTrainingState.
The test asks whether the update preserves the intended local relationship
between the loss morphism and the parameter update morphism.

What failure would this test catch?
A wrong sign, a missing bias gradient, a dropped projection path, or a scaling
mistake where averaged loss and summed gradients disagree.
```

Facilitator note:

A strong answer does not need to derive all of backpropagation. It should
explain the local contract:

```text
small parameter change -> measured loss change
one training step -> inferred parameter gradient
```

Then it should connect that contract to one concrete parameter family, such as
readout weights, feed-forward bias, layer-normalization scale, or an attention
projection bias.
