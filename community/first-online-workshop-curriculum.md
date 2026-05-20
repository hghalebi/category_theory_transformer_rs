# First Online Workshop Curriculum

This is the public curriculum for the first introduction workshop for
Category Theory for Tiny ML in Rust.

Registration:

[Register for the public workshop](https://luma.com/event/evt-Pb1kYMQvzs8JrQq)

Public book:

<https://hghalebi.github.io/category_theory_transformer_rs/>

Repository:

<https://github.com/hghalebi/category_theory_transformer_rs>

## Workshop Promise

In one online session, participants should see the central idea of the book:

```text
tiny ML systems become easier to understand when their stages are named,
typed, runnable, and composed explicitly
```

The first session is not a full category-theory course and not a replacement
for an ML framework. It is an introduction to the learning path.

## Audience

This workshop is for:

- Rust developers curious about machine learning;
- ML engineers who want to inspect what frameworks hide;
- mathematically curious software engineers;
- technical educators evaluating the book as a teaching path;
- beginner-adjacent learners who can follow terminal output and small Rust
  snippets.

Participants do not need prior category-theory knowledge. They should be
comfortable reading short code examples and following a terminal command.

## Learning Objectives

By the end, participants should be able to:

1. Explain the project thesis in one sentence.
2. Run or inspect the first tiny ML pipeline.
3. Name the stages:

```text
Text -> TokenSequence -> TrainingPairs -> ModelState -> Prediction -> Loss -> Updated ModelState
```

4. Explain why `TokenId`, `TokenSequence`, `Logits`, `Distribution`, `Loss`,
   and `Parameters` are not interchangeable raw values.
5. Explain a morphism as a typed transformation between named objects.
6. Point to one place where the book, code, output, or exercise became clear or
   unclear.

## Format

Default duration: 60 minutes.

Mode: online, live, screen-share friendly.

Participant setup options:

| Setup | What participants do | Best for |
| --- | --- | --- |
| Run locally | clone the repository and run the commands | Rust developers and hands-on learners |
| Read online | follow the public book and terminal output shown by the facilitator | learners without local setup |
| Review route | inspect one path and submit feedback | educators, reviewers, and first-time readers |

Do not block the workshop on local setup. The first learning goal is to see the
pipeline and typed boundaries. Running locally is valuable, but reading the
public output path is acceptable for the first session.

## Pre-Workshop Checklist

Participants who want to run the code locally can prepare with:

```bash
git clone https://github.com/hghalebi/category_theory_transformer_rs.git
cd category_theory_transformer_rs
cargo check
```

Optional first command:

```bash
cargo run --example 01_token_sequence
```

If local setup fails, participants can still follow through the public book and
the facilitator's shared terminal.

## 60-Minute Agenda

| Time | Segment | Goal | Artifact |
| --- | --- | --- | --- |
| 0-5 min | Welcome and thesis | explain why the project exists | one-sentence thesis |
| 5-12 min | First runnable win | show raw text becoming typed structure | `cargo run --example 01_token_sequence` |
| 12-22 min | Domain objects | separate raw values from typed ML roles | `TokenId`, `TokenSequence`, `Distribution`, `Parameters` |
| 22-32 min | Typed transformations | explain morphisms as typed transformations | `TokenId -> Vector -> Logits -> Distribution` |
| 32-42 min | Training loop intuition | show training as a state update | `Parameters -> Parameters` |
| 42-50 min | Choose your reader route | split attention by Rust, ML, category theory, educator, or beginner path | one route selected |
| 50-57 min | Evidence closeout | capture one clear or unclear point | report shape filled |
| 57-60 min | Next steps | send participants to the book, examples, and feedback links | one next command or page |

## Segment Plan

### 1. Welcome And Thesis

Use this opening:

```text
Frameworks made AI accessible.
Tiny typed systems make parts of AI understandable.
```

Then state the workshop rule:

```text
Every abstract term must return to a Rust type, function, command, or output line.
```

### 2. First Runnable Win

Run:

```bash
cargo run --example 01_token_sequence
```

Point to this output shape:

```text
Raw input
TokenSequence
TrainingPairs
Typed transformation
Text -> TokenSequence -> TrainingPairs
```

Learning prompt:

```text
What became easier to see because the output used names instead of only raw values?
```

### 3. Domain Objects

Open:

```text
book/src/01-domain-objects.md
src/domain.rs
```

Teach the core distinction:

| Type | Raw representation | Teaching role |
| --- | --- | --- |
| `TokenId` | `usize` | semantic role label |
| `TokenSequence` | `Vec<TokenId>` | validated non-empty sequence |
| `Logits` | `Vec<f32>` | raw scores |
| `Distribution` | `Vec<f32>` | validated probabilities |
| `Parameters` | matrices and bias | model state owner |

Run or point to:

```bash
cargo run --example 01_domain_objects
cargo test domain::tests --lib
```

Learning prompt:

```text
Which type only names a role, and which type rejects an invalid state?
```

### 4. Typed Transformations

Open:

```text
book/src/02-morphisms-composition.md
src/category.rs
src/ml.rs
```

Run:

```bash
cargo run --example 02_morphism_composition
```

Use this path:

```text
TokenId -> Vector -> Logits -> Distribution
```

Explain:

```text
A morphism is a typed transformation.
Composition is legal only when the first output type matches the next input type.
```

Learning prompt:

```text
Why is Embedding then Softmax illegal without LinearToLogits?
```

### 5. Training Loop Intuition

Open:

```text
book/src/04-training-endomorphism.md
src/training.rs
```

Run:

```bash
cargo run --example 03_training_endomorphism
```

Use this shape:

```text
TrainStep : Parameters -> Parameters
Parameters x TrainingSet -> Loss
```

Learning prompt:

```text
Why is loss a measurement, not the object returned by training?
```

### 6. Choose A Reader Route

Use the last active segment to send participants to the route that fits them.

| Participant | Route | First command or page |
| --- | --- | --- |
| Rust engineer | Rust path | `cargo run --example 01_domain_objects` |
| ML engineer | ML path | `cargo run --example 03_training_endomorphism` |
| Category-theory reader | category-theory path | `cargo run --example 05_seven_sketches` |
| Technical educator | educator path | `docs/educator-path.md` |
| Beginner-adjacent learner | beginner path | public Welcome -> Course Map |

Each participant should leave with one next action, not a long task list.

## Participant Lab Card

Use this card during the session:

```text
Command or page:
What I expected:
What I saw:
Last clear idea:
First unclear point:
Smallest useful fix:
Next command or page:
```

The card is useful even if the participant does not submit feedback.

## Facilitator Notes

Keep the workshop concrete.

If discussion becomes abstract, return to one of these anchors:

```text
TokenId
Distribution::new
Morphism<Input, Output>
TokenId -> Vector -> Logits -> Distribution
TrainStep : Parameters -> Parameters
cargo run --example 01_token_sequence
```

Common pitfalls:

| Pitfall | Intervention |
| --- | --- |
| category theory becomes the first topic | return to the typed pipeline output |
| local Rust setup consumes too much time | switch that participant to the public book path |
| learners treat every wrapper as full validation | compare `TokenId::new` with `Distribution::new` |
| learners treat training as "loss goes down" only | separate `Parameters x TrainingSet -> Loss` from `Parameters -> Parameters` |
| feedback becomes broad opinion | ask for one command, one output line, or one sentence |

## Assessment

The workshop is successful when each participant can answer at least three of
these:

1. What is the first runnable command?
2. What does `TokenSequence` represent?
3. Why are `Logits` not probabilities?
4. What does a morphism mean in this project?
5. Why does composition need matching middle types?
6. What object does a training step update?
7. Where should the participant continue after the workshop?
8. What was the first clear or unclear evidence signal?

## Closeout

Reserve the final minutes for evidence capture.

Minimum shape:

```text
Perspective:
Command or public page tried:
Evidence signal:
Last clear idea:
First unclear point:
Smallest useful fix:
```

If the participant wants to submit feedback, use the
[chapter clarity form](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=chapter-clarity.yml).

If the participant cannot use GitHub during the session, one facilitator may
transcribe one concrete report after the session. Do not merge several people's
comments into one report.

## After The Workshop

Suggested next path:

1. Read [START_HERE.md](../START_HERE.md).
2. Run `cargo run --example 01_domain_objects`.
3. Read [Domain Objects](../book/src/01-domain-objects.md).
4. Run `cargo run --example 02_morphism_composition`.
5. Try [Exercise 1](../book/src/exercises.md#exercise-1-explain-one-domain-type).
6. Open one precise feedback issue if something became unclear.

For group follow-up, use:

- [Reading Club](reading-club.md)
- [Review Sprint](../docs/review-sprint.md)
- [Review Worksheet](../docs/review-worksheet.md)
