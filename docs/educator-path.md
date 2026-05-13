# Educator Path

Use this path if you are reviewing the project as a technical educator,
workshop facilitator, study-group host, or curriculum designer.

The review question is:

```text
Does the learner always know what to run, read, explain, or try next?
```

This is a public learning-path guide for checking learner movement through the
material.

## Thirty-Minute Review

Review these public entry points in order:

1. [README.md](../README.md)
2. [START_HERE.md](../START_HERE.md)
3. [book/src/welcome.md](../book/src/welcome.md)
4. [book/src/00-map.md](../book/src/00-map.md)
5. [book/src/exercises.md](../book/src/exercises.md)

At each stop, ask whether the next action is explicit.

## Learner Action Audit

| Surface | What the learner should know next | Evidence to check |
| --- | --- | --- |
| `README.md` | what the project is and which first command to run | `cargo run --bin category_ml` and `cargo run --example 01_token_sequence` are visible early |
| `START_HERE.md` | which first chapter sequence to follow | Domain Objects, Morphism and Composition, Tiny ML Pipeline, Training, Exercises |
| `book/src/welcome.md` | what the first command proves | text becomes token sequence and training pairs |
| `book/src/00-map.md` | how the source files, objects, and arrows fit together | pipeline diagram plus module map |
| `book/src/exercises.md` | how to practice and check transfer | exercises name a command, expected evidence, and a boundary to inspect |

The audit is about learner movement, not polish. A section can be beautifully
written and still fail if the learner cannot tell what to do next.

## Five-Minute Learner Simulation

Run this sequence as if you were a first-session learner:

```bash
cargo run --example 01_token_sequence
```

Then answer:

```text
What object did the raw text become?
What training examples were created?
Which chapter should I read next?
Which exercise checks the same idea?
```

If any answer requires guessing, that is a useful clarity issue.

## Thirty-Minute Learner Simulation

Use the first three examples:

```bash
cargo run --example 01_token_sequence
cargo run --example 01_domain_objects
cargo run --example 02_morphism_composition
```

Then check whether the learner can explain:

```text
TokenId is not a raw usize.
TokenSequence is not an unvalidated Vec.
A morphism has a source object and target object.
Composition requires the first target to match the second source.
```

The expected assessment is not a formal proof. The expected assessment is a
clear explanation tied to one printed output line or one Rust type.

## Exercise Review

For each exercise, look for four things:

| Requirement | Question |
| --- | --- |
| command | Does the exercise say what to run? |
| concept | Does it name the idea being practiced? |
| evidence | Does it say what output, test, or code boundary proves progress? |
| transfer | Does it ask the learner to explain the idea in a new small case? |

Exercises are strongest when they produce evidence, not only agreement.

## Attention Ledger Facilitation

Use this when a learner reaches [Exercise 12](../book/src/exercises.md#exercise-12-trace-attention-shape-flow).
The goal is not to teach a full Transformer in one session. The goal is to
check whether the learner can preserve roles, shapes, and evidence while the
example grows beyond the first tiny pipeline.

Run:

```bash
cargo run --example 06_attention_scores
```

Then ask for four observable answers:

| Prompt | Ready signal | Intervention if unclear |
| --- | --- | --- |
| Which rows belong to queries? | learner points to `attention shape: 2 query positions x 3 key positions` and says score rows are query positions | ask them to underline the word `query` before naming any category term |
| Which columns belong to keys and values? | learner says source/key-value positions count score columns | compare `QuerySequence x KeySequence -> AttentionScores` with `AttentionWeights x ValueSequence -> AttentionOutput` |
| What does the mask do? | learner says `AttentionMask` removes illegal source positions before softmax | point to `query 0 attends with [0.5, 0.0, 0.5]` and ask why the middle weight is zero |
| What returns to the query side? | learner says `AttentionOutput` has one mixed value row per query position | ask them to trace `query 0 output vector [2.0, 20.0]` back to the value rows |
| Which forward layers are fixed for this pass? | learner says `LayerNormalization` and `PositionWiseFeedForward` preserve `HiddenSequence` shape for their current stored parameters | compare `normalized shape` and `feed-forward shape` with `training state step: 0 -> 1` |
| Where does parameter change belong? | learner says changing scale, shift, weights, or biases belongs to `TransformerTrainingState -> TransformerTrainingState` | ask them to explain why a module forward call is not the same event as a training update |

The educator should listen for this distinction:

```text
scores compare queries with keys
masks remove illegal source positions
weights mix values
outputs return to query positions
fixed module calls preserve shape with current parameters
training-state updates change stored parameters
```

If the learner says "attention is just `HiddenSequence -> HiddenSequence`,"
pause the roadmap and return to the Exercise 12 ledger. The useful correction
is not more theory first. The useful correction is one output line, one Rust
type, and one boundary shape.

## Mastery Rubric

Use this rubric when judging whether the first-session path is working. The
goal is not to grade the learner harshly. The goal is to find the first place
where the material stops producing observable understanding.

| Signal | Ready to continue | Needs repair |
| --- | --- | --- |
| runnable start | learner runs `cargo run --example 01_token_sequence` and names the three printed objects | learner only says the command "worked" |
| domain boundary | learner explains why `TokenId` and `TokenSequence` are separate from raw `usize` and `Vec<TokenId>` | learner treats wrappers as cosmetic |
| composition boundary | learner explains why `Vector == Vector` permits one composition and `Vector != Logits` blocks another | learner says composition means "run functions in order" without naming the middle object |
| ML measurement | learner separates logits, distribution, target token, and loss | learner describes loss as the model update |
| attention shape ledger | learner explains query rows, key-value columns, mask cells, and output rows using one printed line from `cargo run --example 06_attention_scores` | learner collapses scores, masks, weights, and value mixing into one "attention" tensor |
| parameter context | learner distinguishes fixed forward module calls from `TransformerTrainingState -> TransformerTrainingState` updates | learner treats a layer endomorphism as if changing parameters had no boundary |
| transfer evidence | learner points to one output line, type, constructor, test, or exercise that proves the explanation | learner can repeat prose but cannot anchor it to code |

A strong first-session learner does not need formal category theory yet. They
should be able to say:

```text
I can name the object,
I can name the transformation,
I can point to the boundary that made an invalid connection impossible.
```

## Facilitator Interventions

Use these interventions when a learner stalls. Each one should return the
learner to a command, output line, type, constructor, test, or exercise.

| Stall | Intervention | Evidence to recover |
| --- | --- | --- |
| learner says the first command worked but cannot name what happened | point to `Raw input`, `TokenSequence`, `TrainingPairs`, and `Typed transformation`; ask for one sentence per block | one named object and one arrow |
| learner treats wrappers as cosmetic | ask what invariant is lost if `TokenSequence` becomes a plain `Vec<TokenId>` | non-empty validated sequence |
| learner says composition only means "run functions in order" | rerun `cargo run --example 02_morphism_composition` and cover every line except `Vector == Vector` and `Vector != Logits` | the middle object must match |
| learner describes loss as the update | rerun `cargo run --example 03_training_endomorphism` and separate `loss before`, `loss after`, and `TrainStep : Parameters -> Parameters` | measurement is not mutation |
| learner collapses scores, masks, weights, and values | run `cargo run --example 06_attention_scores` and ask for the first output line that proves query rows and key columns are different | attention ledger preserves role ownership |
| learner thinks a layer endomorphism hides parameter learning | compare `normalized shape` with `training state step: 0 -> 1`, then ask which Rust object owns changing scale, shift, weights, or biases | fixed module context versus training-state update |
| learner repeats category words without evidence | ask for one output line, one Rust type, or one test before allowing the vocabulary word | term anchored to executable evidence |
| learner jumps to the Transformer Roadmap too early | send them back to the first three commands and ask for the object, arrow, and protected boundary in each | stable first-session path |

If the intervention does not restore a concrete next action, open a clarity
issue. The useful educator signal is not "the learner struggled." It is the
first stall where a smaller command, output line, or explanation would have
kept the learner moving.

## Report Useful Friction

Open the [chapter clarity feedback form](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=chapter-clarity.yml)
when the learner path loses a next action.

For this path, use
[Open technical educator report](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=chapter-clarity.yml&title=%5Bgood+first+feedback%5D+technical+educator+brief&location=README.md%2C+START_HERE.md%2C+docs%2Feducator-path.md%2C+book%2Fsrc%2Fwelcome.md%2C+book%2Fsrc%2F00-map.md%2C+or+book%2Fsrc%2Fexercises.md&command=public+book+review+path+or+local+file+review).
The link fills the route, not the evidence; the evidence signal should come
from what you personally read, ran, or attempted.

Use this shape:

```text
Perspective: technical educator
Surface reviewed:
Evidence signal:
First place without a clear next action:
Last action that was clear:
What the learner should be told to run, read, explain, or try next:
Smallest useful fix:
```

Use the evidence signal for the missing next action, heading, command, table
row, exercise, or link where the learner path stalled.

Good educator feedback names a heading, command, table row, exercise, or link.
Broad comments are less useful than one concrete blocked learning step.
