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

## Mastery Gate Rubric

Use the chapter mastery gates in `book/src/exercises.md` as a lightweight
oral or written check. A strong response does not need polished wording, but it
must contain all three pieces of evidence:

```text
Run evidence:
the learner ran or can point to the exact command, test, or output.

Explain evidence:
the learner can name the protected value, ML role, or category shape without
copying the chapter sentence.

Transfer evidence:
the learner can apply the same distinction to one nearby case, especially an
invalid shortcut or a changed input.
```

Facilitator rule:

```text
If the learner can run but cannot explain, assign the matching exercise.
If the learner can explain but cannot transfer, assign the failure signal.
If the learner can transfer, move on and keep the answer short.
```

Examples:

- A Domain Objects answer passes only if it names both the constructor
  invariant and the bad state rejected by that invariant.
- A Morphism and Composition answer passes only if it names the missing middle
  object in an illegal composition.
- A Transformer Roadmap answer passes only if it counts inputs before naming a
  boundary as ordinary morphism, product-input morphism, endomorphism, or
  illegal composition.

## Checkpoint Quiz Answer Rationales

Use these rationales after attempting the quiz in `book/src/exercises.md`.
Short answers are fine, but each answer should identify the boundary being
protected.

### Question 1

Expected answer:

`TokenId` makes it harder to confuse a vocabulary position with an arbitrary
count, array index, model dimension, or step count.

Rationale:

The Rust type carries the domain role. The ML meaning is "which vocabulary
item." The category-theory reading is that `TokenId` is an object consumed by
specific morphisms such as embedding, not a generic number.

### Question 2

Expected answer:

The Rust evidence is that `Softmax` consumes `Logits`, not `Vector`. The ML
evidence is that probabilities are computed from vocabulary scores, not
directly from the embedding vector.

Rationale:

The missing middle object is `Logits`. Legal composition requires the output
type of one stage to match the input type of the next stage.

### Question 3

Expected answer:

The `0.9` target-probability case should have lower cross-entropy than the
`0.1` case.

Rationale:

Cross-entropy penalizes the model according to the probability it assigned to
the correct target. Higher target probability means lower surprise and lower
loss.

### Question 4

Expected answer:

The update no longer has the reusable shape
`TransformerTrainingState -> TransformerTrainingState` or
`Parameters -> Parameters`.

Rationale:

A training loop must be able to feed the updated object into the next update.
Returning one loose matrix forces callers to reconstruct the rest of the state
and breaks the endomorphism-shaped loop.

### Question 5

Expected answer:

They preserve the wrapper shape while changing the inside value when a value is
available.

Rationale:

For `Vec`, every element is transformed and the result is still a `Vec`. For
`Option`, `Some` is transformed and `None` stays absent. That is the local
meaning of structure-preserving mapping in these examples.

### Question 6

Expected answer:

Both paths should return the same `Option<B>`.

Rationale:

One path maps inside the vector first and then selects the first element. The
other path selects the first element first and then maps inside the option.
Commutativity means the order of these two structure-respecting operations
does not change the final result.

### Question 7

Expected answer:

It has a product input. The mask is extra evidence, so the whole input object
is not just `AttentionScores`.

Rationale:

An endomorphism in this book has shape `A -> A`. A boundary with shape
`A x B -> A` may return the left object, but it is still a product-input
morphism because it needs both inputs.

### Question 8

Expected answer:

The mask must remove illegal source positions before probability mass is
assigned.

Rationale:

Softmax normalizes a row into weights. If the illegal position is still present
during softmax, it competes for probability mass. Masking first means the
weights answer "among legal source positions, how much should each one
contribute?"

### Question 9

Expected answer:

The missing boundary is
`MultiHeadOutput -> ProjectedAttentionOutput`.

Rationale:

Concatenated heads may have width `head_count * head_dimension`. Residual
addition needs the projected output to match the hidden sequence model
dimension before it can return `HiddenSequence`.

### Question 10

Expected answer:

It checks the local sign and scale of the implemented gradient for one selected
parameter. It does not prove every parameter, every dataset, every optimizer,
or every future training loop is correct.

Rationale:

Finite differences give an independent local slope estimate. Agreement with
the inferred update gradient is strong evidence for that parameter path, but
the scope remains local.

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

The composition diagnostic is:

| Question | Answer |
| --- | --- |
| first source | `TokenId` |
| first target | `Vector` |
| second source | `Logits` |
| second target | `Distribution` |
| failed middle-object match | `Vector` is not `Logits` |
| missing morphism | `LinearToLogits : Vector -> Logits` |

The missing middle stage is `LinearToLogits`. In ML terms, the skipped stage is
vocabulary scoring: the model needs logits before `Softmax` can produce a
probability distribution.

Facilitator note:

Do not let learners solve this by weakening types. The lesson is that the type
boundary correctly rejects the skipped prediction stage. A strong answer
debugs source, target, and middle objects before changing code.

### Exercise 5: Change The Training Repetition Count

Expected reasoning:

- `StepCount` controls how many times the same `TrainStep` is applied.
- One step preserves parameter shape but may not reduce loss much.
- More steps usually reduce loss on this tiny dataset until the simple update
  rule reaches its limit.
- Repetition is legal because the shape is `Parameters -> Parameters`.

Expected training diagnostic:

| Question | Expected answer |
| --- | --- |
| what object is updated? | `Parameters` |
| what object measures quality? | `Loss`, computed from `Parameters x TrainingSet` |
| what repeats? | the same `TrainStep : Parameters -> Parameters` |
| what controls update size? | `LearningRate` and averaged gradients |

Loss is evidence about the current parameters. It is not the object returned by
the update. The update returns another `Parameters` value, which is why the next
step can run without reconstructing the model state.

Facilitator note:

Reject answers that say only "more steps make loss go down." A strong answer
separates the measurement arrow from the update arrow and notes that this tiny
dataset can improve while still not proving that more steps are always better.

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

The answer should not only paste a link. It should classify the source role,
name the boundary the source owns, and explain what the larger resource teaches
that the tiny implementation compresses.

Good example:

```text
External resource: Dive into Deep Learning softmax regression.
Source role: open textbook or university material.
Owned boundary: ML intuition for logits, softmax normalization, and
cross-entropy.
Source file: src/ml.rs.
Rust syntax connection: Softmax implements Morphism<Logits, Distribution>.
ML concept connection: logits become normalized probabilities.
Category theory concept connection: Logits -> Distribution is a typed
transformation.
What this source can support: the ML interpretation of the tiny softmax and
loss path.
What this source cannot support: a claim that this repository implements a full
tensor framework, minibatch training stack, or calibrated confidence model.
Difference: the course uses a tiny hand-written example, not a full tensor
library or minibatch training system.
```

Second good example:

```text
External resource: PyTorch CrossEntropyLoss.
Source role: official framework documentation.
Owned boundary: production API shape for unnormalized logits and target class
indices.
Source file: src/ml.rs.
Rust syntax connection: CrossEntropy consumes Product<Distribution, TokenId>
after the teaching path has already built a Distribution.
ML concept connection: production frameworks usually combine log-softmax and
negative log-likelihood behind one loss interface.
Category theory concept connection: the book expands the path into
Logits -> Distribution -> Product<Distribution, TokenId> -> Loss so the
objects are visible.
What this source can support: the distinction between logits and target class
indices in a production API.
What this source cannot support: changing the book's teaching path into a
direct CrossEntropyLoss clone.
Difference: the source documents a framework interface; the repository teaches
the smaller typed boundary step by step.
```

Reject answers that use a learner-friction source as authority for a formal
definition, or an implementation bridge as proof that the tiny Rust code
implements a full research paper.

### Exercise 10: Test One Sketch Law

Expected reasoning:

The learner should name both a positive law and a rejected invalid case.

Examples:

- `InformationLevel` checks reflexivity and transitivity.
- `SignalMatrix::compose_after` rejects mismatched middle dimensions.
- `OpenCircuit::then` rejects incompatible output/input boundaries.
- `FeasibilityRelation::relates` checks whether one implementation offer
  satisfies one requirement.

Facilitator note:

This is the key transfer exercise from tiny ML into applied category theory.
Ask: "What invalid composition does this model prevent?"

Expected co-design answer:

```text
DesignRequirement x ImplementationOffer -> bool
```

This is a relation because one requirement can be satisfied by many offers, and
one offer can satisfy many compatible requirements. It is not a function from a
requirement to one unique implementation.

A passing offer might have higher throughput than required and lower latency
than allowed. A failing offer might have enough throughput but too much latency.

The architecture translation is:

```text
ArchitectureConstraint x CandidateImplementation -> Bool
```

One passing candidate is implementation evidence for that candidate under that
constraint. It is not proof that every future implementation satisfies the
whole architecture constraint space.

Expected PDF-to-Rust contract answer:

```text
source idea from the PDF: schemas and instances
Rust handle: CompanyInstance::new
protected law, relation, or boundary: every EmployeeRecord department must
resolve to an existing DepartmentId
larger source claim not implemented by this code: a general functorial
semantics for database schemas and instances
local evidence command or test: cargo test sketches::tests --lib
```

A complete transfer triage card for that answer is:

```text
source idea: schemas and instances
local Rust handle: CompanyInstance::new
protected law, relation, or boundary: EmployeeRecord -> DepartmentId must
resolve
invalid shortcut rejected: letting a missing department reach feature extraction
tiny ML transfer: validate structured training rows before training
larger claim not implemented: a general categorical database semantics
local evidence command or test: cargo test sketches::tests --lib
```

Another valid answer:

```text
source idea from the PDF: open systems compose through interfaces
Rust handle: OpenCircuit::then
protected law, relation, or boundary: the output ports of the first circuit
must match the input ports of the second circuit
larger source claim not implemented by this code: a full circuit algebra
local evidence command or test:
sketches::tests::open_circuit_serial_composition_rejects_boundary_mismatch
```

Reject answers that say the chapter "implements Seven Sketches." A good answer
says which local Rust handle carries one executable boundary from the larger
source text.

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

Expected Q/K/V diagnostic:

| Printed diagnostic line | Meaning |
| --- | --- |
| `query rows own score rows; key/value rows own score columns` | the score table is query-by-source, so rows belong to the query side and columns belong to the key-value side |
| `self-attention shares the hidden source before projection; projected roles stay distinct` | self-attention can feed one hidden sequence into Q, K, and V projections, but the projected outputs still have different roles |
| `mask polarity here: true = allowed, false = blocked` | this repository's mask convention marks legal source positions with `true`; framework masks with the same shape may use the opposite polarity |

Expected one-sentence explanations:

```text
query rows:
Each score row belongs to one query position, so target/query length counts rows.

key/value rows:
Each score column belongs to one source key-value position, so source length counts columns.

self-attention source:
Self-attention shares the pre-projection hidden sequence, but QuerySequence,
KeySequence, and ValueSequence remain separate typed roles after projection.

mask polarity:
The local AttentionMask uses true for "this query may read this source";
compare framework masks only after translating boolean meaning.
```

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

Expected quick roadmap classification drill:

| Boundary | Classification | Reason |
| --- | --- | --- |
| `HiddenSequence -> QuerySequence` | ordinary morphism | one hidden-state object is projected into the query role object |
| `AttentionScores x AttentionMask -> AttentionScores` | product-input morphism returning the score object | the mask is extra context, so the whole input is not only `AttentionScores` |
| `LayerNormalization : HiddenSequence -> HiddenSequence` | endomorphism | one input object returns the same public object |
| `HiddenSequence x ProjectedAttentionOutput -> HiddenSequence` | product-input morphism returning hidden state | residual addition needs both the old hidden stream and the projected sublayer output |
| `TransformerTrainingState -> TransformerTrainingState` | state endomorphism | the whole training state returns as the same object for the next update |

Trap explanation:

```text
`A x B -> A` is not an endomorphism on `A`; it is a product-input morphism
unless `B` has been fixed as context or the product `A x B` is the object being
studied.
```

If the product is treated as the source object, the arrow can be read as:

```text
(A x B) -> A
```

That is a unary morphism out of the product object, not an endomorphism. An
endomorphism on the product would be:

```text
(A x B) -> (A x B)
```

So the safe name remains "product-input morphism returning `A`" unless the
answer explicitly changes the source and target objects and checks that they
match.

Expected same-output classification:

| Boundary | Classification | Reason |
| --- | --- | --- |
| `LayerNormalization : HiddenSequence -> HiddenSequence` | endomorphism | one input object and the same output object |
| `HiddenSequence x ProjectedAttentionOutput -> HiddenSequence` | product-input morphism returning hidden state | residual addition needs the old hidden stream and the projected sublayer output |
| `HiddenSequence x MultiHeadOutput -> HiddenSequence` | illegal boundary before category naming | the output projection is missing, so raw concatenated heads cannot rejoin the residual stream |

The shared output name is not enough. The answer must count inputs before
naming the category shape.

Expected terminal-output audit:

| Printed output line | What the line proves | Overclaim to reject | Boundary to name |
| --- | --- | --- | --- |
| `projected attention shape: 2 positions x model dimension 2` | raw head output has been projected back to model width | residual addition has already happened | `MultiHeadOutput -> ProjectedAttentionOutput` |
| `residual shape: 2 positions x model dimension 2` | the result has returned to hidden-sequence shape | residual addition was unary | `HiddenSequence x ProjectedAttentionOutput -> HiddenSequence` |
| `masked multi-head block shape: 2 positions x model dimension 2` | the block output can feed the next hidden-sequence layer | the open masked block is a pure endomorphism | `MaskedMultiHeadTransformerBlock : HiddenSequence x AttentionMask -> HiddenSequence` |
| `training state step: 0 -> 1` | the update returns a state that can be updated again | training is a loose `Loss -> Parameters` shortcut | `TransformerTrainingState -> TransformerTrainingState` |

The shape line is target evidence. It is not enough to name the category
boundary. A strong answer also reads the typed transformation line and names
the whole source object before deciding whether the boundary is ordinary,
product-input, an endomorphism, or illegal.

Expected source-ownership diagnostic:

| Case | Query owner | Key owner | Value owner | Score rows | Score columns |
| --- | --- | --- | --- | --- | --- |
| Self-attention | the same hidden sequence before projection | the same hidden sequence before projection | the same hidden sequence before projection | target/query positions | source/key-value positions, equal to the query length in the simple self-attention case |
| Cross-attention | target hidden sequence | source hidden sequence | source hidden sequence | target/query positions | source/key-value positions |

Self-attention shares source ownership before projection. It does not erase the
role split after projection. A strong answer should still name
`QuerySequence`, `KeySequence`, and `ValueSequence` separately, then say that
the simple self-attention case feeds all three projections from the same
`HiddenSequence`.

Expected shape-ledger answer:

| Ledger item | Framework cue | Rust roadmap meaning | Category-shape consequence |
| --- | --- | --- | --- |
| target length | PyTorch `L`, TensorFlow/Keras `T` | number of `QuerySequence` rows | score rows belong to the query-side object |
| source length | PyTorch/Keras `S` | number of `KeySequence` and `ValueSequence` rows | score columns belong to the key-value source object |
| attention mask | PyTorch `L x S`, TensorFlow/Keras `(B, T, S)` | permission table from query rows to source rows | the mask is context over a product boundary |
| attention output | target-side output rows | one `AttentionOutput` row for each query row | value mixing returns information to the query side |

Sanity check:

```text
score table rows == query positions
score table columns == key-value positions
mask cells == query-position/source-position permissions
output rows == query positions after reading values
```

If an answer says the mask is only a vector over tokens, or says output rows
belong to key positions, it has collapsed source ownership or mask context too
early.

Expected mask-role ledger:

| Question | Expected answer |
| --- | --- |
| What does an attention-mask cell select? | Whether one query row may read one source column before softmax. |
| Why is the mask not a shorter token sequence? | The token/source rows still exist; the mask selects legal score cells in the query-by-source score table. |
| Why does the mask not directly produce `AttentionWeights`? | `AttentionScores x AttentionMask -> AttentionScores` happens first; `AttentionScores -> AttentionWeights` is the later softmax boundary. |
| Which block-level boundary keeps the mask visible instead of hidden? | `MaskedMultiHeadTransformerBlock : HiddenSequence x AttentionMask -> HiddenSequence`. |
| In a fixed-mask view, what context was selected first? | One named `AttentionMask`, such as mask `M`, was selected before the remaining `HiddenSequence` call. |
| What does true mean in this repository's `AttentionMask`? | This source position is allowed for that query row. |
| Why can a framework mask with the same shape still need boolean inversion? | Shape and polarity are separate. Some framework APIs use `true` for blocked or padding positions, while this repository uses `true` for allowed positions. |

Three-step rule:

```text
mask cells select legal score cells
softmax turns remaining score rows into weights
weights read value rows
```

Reject answers that say "the mask removes tokens" without naming score cells.
The source sequence still owns the value rows. The mask only says which source
rows each query is allowed to read before probability normalization.

Expected linear-scope diagnostic:

| Question | Expected answer |
| --- | --- |
| Which listed boundaries are the linear Q/K/V projections? | `HiddenSequence -> QuerySequence`, `HiddenSequence -> KeySequence`, and `HiddenSequence -> ValueSequence` |
| Which boundary turns scores into nonlinear normalized weights? | `AttentionScores -> AttentionWeights` |
| Which product-input boundaries must not be collapsed into one unary map? | query-key scoring, mask application, value mixing, and residual addition |
| Which state endomorphism belongs to training rather than forward attention? | `TransformerTrainingState -> TransformerTrainingState` |

Use this to reject an overextended answer such as "the whole attention block is
an endofunctor." A strong answer can say that linear Q/K/V projections are a
safe place to compare with advanced categorical work, while softmax, masking,
residual addition, normalization, feed-forward refinement, and training state
each need their own typed boundary in this book.

Expected source-scope diagnostic:

| Question | Expected answer |
| --- | --- |
| Which source supports decomposing attention into recurring components? | `On the Anatomy of Attention` |
| Which source supports comparing the linear Q/K/V part with advanced category theory? | `Self-Attention as a Parametric Endofunctor` |
| What does neither source license you to claim about the whole Rust roadmap block? | Neither source means the tiny roadmap block is one undifferentiated endofunctor or that the Rust code implements the full paper formalism. |
| What is the local Rust contract for every component in this book? | Each component needs a named type, a boundary shape, and a failure it prevents. |

Use this to keep source roles separate. The anatomy source supports decomposition before comparison. The parametric-endofunctor source supports a narrower comparison around linear self-attention structure. The local teaching claim is smaller than both papers: the Rust roadmap names each component so a reader can inspect the boundary and the invalid connection it blocks.

Expected architecture-constraint diagnostic:

| Question | Expected answer |
| --- | --- |
| What is one architecture constraint in the roadmap? | One valid answer: masked attention should assign probability only to legal source positions. Another valid answer: residual addition should return to the public hidden-sequence shape. |
| Which Rust type, constructor, example, or test is implementation evidence for it? | `AttentionMask::new` rejects fully masked rows; `cargo run --example 06_attention_scores` shows masked weights; `ResidualConnection` rejects mismatched widths before returning `HiddenSequence`. |
| Why is that not proof of the whole future architecture? | It checks one local implementation boundary. A full architecture claim would also need every surrounding boundary, nonlinear step, parameter update, data case, and composition law to be named and tested. |

This is where Categorical Deep Learning is useful as scope control. It reminds
the reader to separate a constraint the architecture should satisfy from the
implementation evidence currently present in the tiny Rust code.

Expected stackability diagnostic:

| Boundary | Can stack directly as `HiddenSequence -> HiddenSequence`? | Reason |
| --- | --- | --- |
| `LayerNormalization : HiddenSequence -> HiddenSequence` | yes, for a fixed layer instance | it has one input and returns the same public object while scale, shift, and epsilon are already stored in the layer |
| `MultiHeadTransformerBlock : HiddenSequence -> HiddenSequence` | yes, for a fixed block instance | the block owns its internal attention, residual, normalization, and feed-forward path behind one unary boundary |
| `MaskedMultiHeadTransformerBlock : HiddenSequence x AttentionMask -> HiddenSequence` | no, not while the mask is open | the full input object is a product, so the mask context is still required |
| fixed-mask view of `MaskedMultiHeadTransformerBlock` | yes, for that named mask context | selecting the mask first induces a unary map over `HiddenSequence` for that run |
| `TransformerTrainingState -> TransformerTrainingState` | yes | the complete training object carries the parameter and update context |

When a layer's parameters are changing, the answer should move from the
forward boundary to the training-state boundary. A strong answer says:

```text
LayerNormalization : HiddenSequence -> HiddenSequence
is an endomorphism for one fixed layer value.

Changing scale, shift, weights, or biases belongs to
TransformerTrainingState -> TransformerTrainingState.
```

The same fixed-value rule applies to positional encodings and whole blocks:

```text
PositionalEncoding : HiddenSequence -> HiddenSequence
is an endomorphism only after one position table has been selected.

MultiHeadTransformerBlock : HiddenSequence -> HiddenSequence
is an endomorphism only for one fixed block value.
```

If the position table, heads, projections, normalization values, or
feed-forward weights are learned, swapped, or rebuilt, the answer should name
the changing context instead of hiding it in the forward arrow. In this
repository's training examples, changing trainable block values belongs to:

```text
TransformerTrainingState -> TransformerTrainingState
```

The precise ways to repeat a masked block are:

```text
1. keep the mask visible and supply HiddenSequence x AttentionMask each time
2. explicitly fix a mask first, then name the induced HiddenSequence -> HiddenSequence view
```

Do not accept "it returns `HiddenSequence`" as enough evidence. A strong answer
counts the full input object, names the mask context, and says whether the
context is open or fixed.

Expected context-fixing drill:

| Case | Expected answer |
| --- | --- |
| Open masked block: whole input object | `HiddenSequence x AttentionMask` |
| Open masked block: safe category shape | product-input morphism returning `HiddenSequence` |
| Open masked block: stackability | it cannot stack unaided as `HiddenSequence -> HiddenSequence` because the mask is still an open input |
| Fixed-mask view: selected first | one named `AttentionMask`, such as mask `M`, is selected before the block call |
| Fixed-mask view: induced boundary | `MaskedMultiHeadTransformerBlock[M] : HiddenSequence -> HiddenSequence` |
| Fixed-mask view: promise while stacking | the same mask context remains fixed, or the prose names when it changes |
| Changing mask per call | the caller must supply `HiddenSequence x AttentionMask` each time, or carry the mask inside a larger state |
| Why changing mask per call differs from fixed mask | the context is not fixed once; every call still depends on a fresh or threaded mask |
| Residual addition | `HiddenSequence` and `ProjectedAttentionOutput` remain visible inputs |
| Why residual addition is not unary | the whole input is still a product, even though the output returns to `HiddenSequence` |
| Residual addition after naming the product as source | `(HiddenSequence x ProjectedAttentionOutput) -> HiddenSequence`, a unary morphism out of a product object, not an endomorphism |
| Rust closure bridge: captured value | the closure captures one chosen `AttentionMask`, such as `fixed_mask` |
| Rust closure bridge: remaining argument | the closure is called with `HiddenSequence` |
| Rust closure bridge: unchanged open boundary | the original block still has type `HiddenSequence x AttentionMask -> HiddenSequence`; the closure is only a fixed-context view |

Fixed context must be named.
Fixing a mask does not erase the source of context; it only creates a smaller
view for one run, proof, or example.
If the mask changes, the boundary has changed back into an open product-input path.
Some larger state object can still carry the mask for the next call, but then
the state object must be named.

Expected add-norm order drill:

| Question | Expected answer |
| --- | --- |
| Current attention sublayer order | projected attention output is added to the old hidden stream, then `attention_norm` normalizes the residual result |
| Current feed-forward sublayer order | feed-forward output is added to the normalized attention stream, then `feed_forward_norm` normalizes the result |
| Local boundaries that show the order | `ResidualConnection` followed by `LayerNormalization`; in the block this appears as `with_attention -> normalized_attention` and `with_feed_forward -> feed_forward_norm` |
| Why same shape is not same morphism | post-norm and pre-norm blocks can both have source and target `HiddenSequence`, but the internal composition order differs |
| Future pre-norm variant | name a separate constructor, type, or mode such as `PreNormMultiHeadTransformerBlock`; do not silently reuse the current post-add block explanation |

The key category-theory point is that source and target equality permits an
endomorphism name for a fixed block. It does not prove two endomorphisms are
the same arrow.

Naming rule:

```text
Count inputs first.

One input returning the same public object can be an endomorphism.
A product input returning the left object is still a product-input morphism.
An attempted product input with the wrong second object is illegal before it
gets a category-theory name.
```

Expected source-target audit card examples:

| Boundary | Whole source object | Target object | Context status | Safe conclusion |
| --- | --- | --- | --- | --- |
| `AttentionScores x AttentionMask -> AttentionScores` | `AttentionScores x AttentionMask` | `AttentionScores` | mask is open context | product-input morphism returning scores |
| fixed-mask view of a masked block | `HiddenSequence` | `HiddenSequence` | one named `AttentionMask` was selected first | induced endomorphism for that mask |
| `HiddenSequence x ProjectedAttentionOutput -> HiddenSequence` | `HiddenSequence x ProjectedAttentionOutput` | `HiddenSequence` | residual input is open context | product-input morphism returning hidden state |
| `TransformerTrainingState -> TransformerTrainingState` | `TransformerTrainingState` | `TransformerTrainingState` | training context is inside the state object | state endomorphism |

Reject any answer that compares only the left side of a product input with the
output. A product-input boundary's whole source object is the product.

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
input. Also reject answers that say self-attention makes query, key, and value
"the same thing." The same source sequence can produce three distinct role
objects. Also reject answers that import an advanced categorical label for the
whole block when the answer only identified the linear projections.

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

### Exercise 16: Trace Transformer Training State

Expected output evidence:

```text
initial state: step=0, learning_rate=0.100, model_dimension=2, vocab_size=3
forward shape: 2 positions x vocabulary size 3
readout update: step 0 -> 1
feed-forward update: step 1 -> 2
composed block update: step 2 -> 3
```

Expected classification:

| Update | What it trains | Public shape |
| --- | --- | --- |
| `TransformerReadoutTrainStep` | sequence readout from hidden states to token logits | `TransformerTrainingState -> TransformerTrainingState` |
| `TransformerFeedForwardTrainStep` | local position-wise feed-forward sublayer against hidden targets | `TransformerTrainingState -> TransformerTrainingState` |
| `TransformerBlockTrainStep` | readout, feed-forward, attention output projection, query/key/value projections, and layer-normalization parameters from token targets | `TransformerTrainingState -> TransformerTrainingState` |

Expected reasoning:

```text
Rust syntax:
Each training-step type implements a morphism that consumes one
TransformerTrainingState and returns another TransformerTrainingState.

ML concept:
The state keeps model parameters, learning rate, and step count together. The
three updates train different parameter subsets, but each one still advances
the same training object.

Category theory concept:
Each update is an endomorphism because its input and output object are the same
state type. The internal gradient path can change without changing the outside
loop shape.
```

Reject answers that say only "the loss goes down." A strong answer names the
state invariant: after every update the next step still has parameters,
learning rate, and step count. Returning only readout weights, only
feed-forward weights, or a bag of changed matrices would make the next update
reconstruct missing context by hand.

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

Official framework practice says the same thing more generally. PyTorch's
`gradcheck` documentation compares small finite differences against analytical
gradients and treats agreement as tolerance-based local evidence. It also warns
about precision, non-differentiable points, and overlapping memory. In this
book, the matching Rust claim is intentionally smaller: a finite-difference
match supports one selected parameter path, not the correctness of every
gradient, dataset, or optimizer setting.
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
