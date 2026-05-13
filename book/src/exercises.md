# Exercises

The problem this chapter solves is:

> Reading detailed explanations is not enough. You need to practice explaining
> the code through Rust syntax, ML concept, and category-theory concept.

The exercises are deliberately small. A strong answer is not a long essay; it
is a precise explanation that connects a line of Rust to the value it protects,
the ML step it supports, and the categorical shape it names. When an exercise
asks you to edit code, make the smallest change, run the command, and then
explain what changed.

For every exercise, use this answer shape:

```text
Rust syntax:
...

ML concept:
...

Category theory concept:
...
```

The point is not to write long answers.

The point is to connect the same block of code across all three meanings.

The exercise method is:

```text
read one small idea
run the matching command
break one boundary on purpose
explain the failure
restore the working version
```

This matters because the Rust compiler and the test suite are part of the
lesson. The official Rust testing material treats tests as executable checks
for expected behavior. This book uses the same habit for learning: a failed
test, rejected constructor, or compiler error is not only a problem to remove.
It is evidence about which boundary the code protects.

## Source-Backed Practice Contract

This chapter uses sources to keep practice cumulative, testable, and
transfer-oriented. Each source supports one local exercise rule and one kind of
repository evidence.

| Source | What the source supports | Local rule in this chapter | Repository evidence |
| --- | --- | --- | --- |
| [How People Learn II](https://www.nationalacademies.org/projects/DBASSE-BBCSS-13-06/publication/24783) | Learners need practice that connects prior knowledge to new transfer situations. | Move from one small Rust boundary to a new but related boundary. | `TokenId` to `Distribution`, then `TrainStep`, then attention shapes |
| [Test-Enhanced Learning](https://doi.org/10.1111/j.1467-9280.2006.01693.x) | Retrieval practice can improve retention rather than only measure it. | Ask Recall, Explain, and Apply questions before the answer key. | `## Checkpoint Quiz`, `## Retrieval Practice`, `exercises/ANSWER_KEY.md` |
| [Structuring the Transition From Example Study to Problem Solving](https://doi.org/10.1207/S15326985EP3801_3) | Learners benefit from moving from worked examples toward independent problem solving. | Use a worked example, then a partially completed example, then an open transfer exercise. | `## Worked Example`, `## Partially Completed Example`, `## Transfer Exercise` |
| [Rust Book: Writing Automated Tests](https://doc.rust-lang.org/stable/book/ch11-00-testing.html) | Tests check expected behavior that the type system alone cannot prove. | Treat tests, constructor errors, and compiler errors as learning evidence. | `cargo test --all-targets --all-features`, `domain::tests`, `category::tests`, `ml::tests` |
| [Rust By Example: Tests](https://doc.rust-lang.org/rust-by-example/cargo/test.html) | Small test commands and targeted test names make feedback inspectable. | Prefer one named command and one visible signal per exercise attempt. | `cargo test structure::tests --lib`, `cargo test cross_entropy_is_lower_for_more_confident_target_probability --lib` |
| [CS231n Optimization](https://cs231n.github.io/optimization-1/) and [PyTorch gradcheck](https://docs.pytorch.org/docs/stable/generated/torch.autograd.gradcheck.gradcheck.html) | Numerical gradient checks are useful local debugging signals with limits. | Use finite differences to compare one local update path, then state what the check does not prove. | Advanced Exercise 5, `TransformerBlockTrainStep` finite-difference tests |

The transfer pattern is:

```text
worked example -> partial example -> independent attempt -> evidence signal
```

For this chapter, evidence means one of:

```text
command output
constructor error
compiler error
named test result
answer-key mismatch
```

It is not evidence that every exercise works for every reader yet. Direct
exercise-attempt reports are still needed before the exercise ladder can be
called fully validated.

Before starting, make sure the basic Rust feedback loop works:

```bash
cargo test --all-targets --all-features
```

That command is part of the learning method. It proves that the examples in the
book are not only explanatory text; they are tied to code that the compiler can
check.

After attempting an exercise, compare your reasoning with the public answer key
in `exercises/ANSWER_KEY.md`. Use it to check the shape of the explanation, not
to memorize wording.

## Exercise Ladder

Use the exercises in this order:

| Stage | File or chapter | What you practice |
| --- | --- | --- |
| Beginner | `exercises/beginner/README.md` | Change inputs, observe output, name one invariant |
| Core | this chapter | Explain each concept through Rust, ML, and category theory |
| Intermediate | `exercises/intermediate/README.md` | Add one morphism and explain one composition failure |
| Advanced | `exercises/advanced/README.md` | Extend a chapter, diagram, law, or sketch test |

Do not skip the small exercises. How People Learn II emphasizes that learners
need to retrieve and use knowledge in new situations. In this course, transfer
means taking the same explanation method from `TokenId` to `Distribution`, then
from `Distribution` to training, and then from training to the seven applied
sketches.

## Core Chapter Practice Map

Use this map when you finish a chapter and want the matching practice task.

| Chapter | Practice target | Best exercise |
| --- | --- | --- |
| [Welcome](welcome.md) | Explain the three-lens reading contract | Beginner Exercise 3 |
| [Course Map](00-map.md) | Connect terminal output to pipeline stages | Exercise 2 and Exercise 8 |
| [Domain Objects](01-domain-objects.md) | Explain wrappers, invariants, and typed objects | Exercise 1 and Exercise 7 |
| [Morphism and Composition](02-morphisms-composition.md) | Explain legal and illegal composition | Exercise 4 |
| [The Tiny ML Pipeline](03-ml-pipeline.md) | Trace adjacent pairs, prediction, and loss | Exercise 3, Exercise 9, and Exercise 13 |
| [Training as an Endomorphism](04-training-endomorphism.md) | Explain repeated `Parameters -> Parameters` updates | Exercise 5 |
| [Functors, Naturality, Monoids, and Chain Rule](05-structure-and-calculus.md) | Explain mapping, laws, traces, and local gradients | Exercise 6 and Exercise 14 |
| [Seven Sketches Through Rust](seven-sketches-rust.md) | Identify the law or boundary a structure protects | Exercise 10 |
| [Transformer Roadmap](roadmap.md) | Trace attention shapes, classify category shapes, and explain finite-difference checks for structured training state | Exercise 12, Exercise 16, and Advanced Exercise 5 |

The map is not a separate syllabus. It is a repair tool. If a chapter feels
clear while reading but vague one hour later, use the matching exercise to make
the idea active again.

## Chapter Mastery Gates

Use these gates before moving from a chapter into later material. A gate is not
a grade. It is a quick test of whether the idea is active enough to reuse.

| Chapter | Run evidence | Explain evidence | Transfer evidence |
| --- | --- | --- | --- |
| Welcome | `cargo run --example 01_token_sequence` | state the three-lens reading contract without looking back | explain one output line through Rust, ML, and category theory |
| Course Map | `cargo run --bin category_ml` | name the file or module behind three printed sections | choose the next chapter and matching exercise from the output |
| Domain Objects | `cargo run --example 01_domain_objects` | explain one constructor invariant and the bad state it rejects | replace one raw value in an explanation with its domain type |
| Morphism and Composition | `cargo run --example 02_morphism_composition` | name every middle object in `TokenId -> Vector -> Logits -> Distribution` | explain one illegal skipped stage and the missing object |
| The Tiny ML Pipeline | `cargo test ml::tests --lib` | separate logits, probabilities, target token, and loss | compute which prediction should have lower cross-entropy |
| Training as an Endomorphism | `cargo run --example 03_training_endomorphism` | explain why one update has shape `Parameters -> Parameters` | predict what breaks if an update returns only a loose changed field |
| Functors, Naturality, Monoids, and Chain Rule | `cargo run --example 04_structure_and_calculus` | explain one law by tracing both sides of the example | classify a new trace, option, vector, or derivative example |
| Seven Sketches Through Rust | `cargo run --example 05_seven_sketches` | identify the relation, order, schema, circuit, or cover being protected | model one analogous boundary in a small software system |
| Transformer Roadmap | `cargo run --example 06_attention_scores` and `cargo run --example 07_transformer_training_state` | classify attention boundaries by input count and output object | reject one illegal shortcut such as `HiddenSequence x MultiHeadOutput -> HiddenSequence` |

If a gate fails, do not reread the whole chapter first. Start with the matching
exercise, inspect the failure signal, and compare your reasoning with the
answer-key rubric. The smallest useful repair is usually one missing object,
one missing command, or one missing distinction.

## Checkpoint Quiz

Use this after the mastery gates. Answer from memory first, then check the
answer key. The goal is not vocabulary recall alone. The goal is to notice
whether you can connect a Rust boundary, an ML role, and a category-theory
shape without the chapter open.

### Questions

Write one or two sentences for each question.

1. A value has type `TokenId`. What mistake becomes harder than if the same
   value crossed the boundary as `usize`?
2. The path `TokenId -> Vector -> Logits -> Distribution` fails if the middle
   `Logits` stage is skipped. What Rust evidence and ML evidence explain the
   failure?
3. A model gives the target token probability `0.9` in one case and `0.1` in
   another. Which case should have lower cross-entropy, and why?
4. A training update changes weights but returns only the changed readout
   matrix. Which composition shape has been broken?
5. `VecFunctor::fmap` maps every element and `OptionFunctor::fmap` maps only
   when a value is present. What does that preserve?
6. A naturality square has two paths from `Vec<A>` to `Option<B>`. What should
   be true if the square commutes?
7. `AttentionScores x AttentionMask -> AttentionScores` returns the score
   object. Why is this still not a unary endomorphism?
8. Why must the attention mask act before row-wise softmax?
9. `HiddenSequence x MultiHeadOutput -> HiddenSequence` looks tempting after
   concatenating heads. Which missing boundary makes it illegal?
10. A finite-difference test agrees with the inferred gradient for one
    parameter. What has it checked, and what has it not checked?

### Coverage Map

| Question | Chapter or section | Main objective |
| --- | --- | --- |
| 1 | Domain Objects | explain why a wrapper protects a domain role |
| 2 | Morphism and Composition | identify a missing middle object |
| 3 | Tiny ML Pipeline | connect target probability to loss |
| 4 | Training as an Endomorphism | preserve state-update composition |
| 5 | Structure and Laws | explain structure-preserving mapping |
| 6 | Structure and Laws | trace both paths through a naturality square |
| 7 | Transformer Roadmap | count inputs before naming an endomorphism |
| 8 | Transformer Roadmap | separate masked scores from weights |
| 9 | Transformer Roadmap | identify a missing projection boundary |
| 10 | Exercises and Transformer Roadmap | state the scope of a local gradient check |

Score the quiz by evidence, not points. A strong answer names the object or
boundary, explains the ML or software role, and rejects one invalid shortcut.
If an answer only repeats a term, return to the matching exercise.

## Failure Signals

A good exercise often fails before it works. Use the failure signal as part of
the answer.

| Signal | Usually means | What to explain |
| --- | --- | --- |
| Compiler type error | two stages do not connect | the missing middle object |
| Constructor returns `Err(...)` | a value violates an invariant | the bad state rejected at the boundary |
| Test assertion fails | the behavior no longer matches the law | which example stopped preserving the intended structure |
| Command output changes | the data path changed | which typed value moved differently through the pipeline |

When an exercise asks you to break something, do it in a small local edit and
then restore the working version. The final repository should still pass the
validation commands.

## Exercise Evidence Map

Use this table before checking the answer key. It tells you what kind of
evidence should exist when an exercise is complete.

| Exercise | Progress evidence | Failure or output to inspect |
| --- | --- | --- |
| Exercise 1 | written three-lens explanation | raw representation, invariant, and pipeline stage are all named |
| Exercise 2 | `cargo run --bin category_ml` | terminal output includes the new adjacent transition |
| Exercise 3 | handwritten adjacent pairs | three overlapping `TokenId` pairs are present |
| Exercise 4 | temporary broken composition | compiler reports a missing trait bound or middle object |
| Exercise 5 | `cargo run --example 03_training_endomorphism` | loss output changes as `StepCount` changes |
| Exercise 6 | rewritten output distribution | probabilities stay attached to transformed outcomes |
| Exercise 7 | constructor boundary explanation | `Err(...)` is connected to the invalid value |
| Exercise 8 | five-sentence file summary | one command is named as the proof that the file still works |
| Exercise 9 | source-role comparison | one external resource is connected to one local source file, one owned boundary, and one unsupported claim |
| Exercise 10 | `cargo run --example 05_seven_sketches` or a negative test | one law still holds, or one invalid structure is rejected |
| Exercise 11 | block explanation | a beginner-facing Rust explanation and a shape name are both present |
| Exercise 12 | `cargo run --example 06_attention_scores` | first output line and category shape for each attention boundary are recorded |
| Exercise 13 | `cargo test cross_entropy_is_lower_for_more_confident_target_probability --lib` | lower loss is assigned to the higher target probability |
| Exercise 14 | `cargo test structure::tests --lib` | naturality paths and monoid laws are both named |
| Exercise 15 | mixed boundary diagnosis | each failure is classified as an invariant, composition, endomorphism, shape, or local-to-global boundary |
| Exercise 16 | `cargo run --example 07_transformer_training_state` | three different updates preserve `TransformerTrainingState -> TransformerTrainingState` |

This is not extra bureaucracy. Rustlings-style practice works because the
learner gets a concrete feedback signal. This course uses the same idea:
command output, a constructor error, a compiler error, or a named test should
tell you whether the concept is becoming executable.

## Worked Example: Mixed Boundary Diagnosis

Before solving Exercise 15, study one complete diagnosis. The case is:

```text
CrossEntropy receives Logits instead of Product<Distribution, TokenId>.
```

A weak answer says:

```text
The types are wrong.
```

That is true, but it is not precise enough. A useful diagnosis names the Rust
boundary, the ML mistake, and the category-theory shape.

```text
Boundary type:
composition boundary plus product-input boundary

Rust syntax:
CrossEntropy implements Morphism<Product<Distribution, TokenId>, Loss>. The
input must therefore be a product containing a validated Distribution and the
target TokenId. Logits alone have the wrong type.

ML concept:
Logits are unnormalized vocabulary scores. Cross-entropy needs the probability
assigned to the correct target token. The missing work is Softmax followed by
pairing the resulting Distribution with the target TokenId.

Category theory concept:
The legal route is Logits -> Distribution and then
Distribution x TokenId -> Loss. Skipping the product object hides the supervised
part of the loss calculation.

Smallest useful fix:
Run Softmax first, then call CrossEntropy on
Product::new(distribution, target_token).
```

Use this as the standard for Exercise 15. Do not stop at "wrong type." Explain
which object was missing, which morphism should have produced it, and which
shortcut the boundary rejected.

## Exercise Attempt Record

When an exercise feels unclear, record the attempt in this shape before opening
an issue or comparing with the answer key:

```text
Exercise:
Chapter:
Command run:
First failure signal:
Line or concept that caused confusion:
What I expected:
What happened instead:
Answer-key mismatch:
Suggested rewrite:
```

This report is useful because it ties reader feedback to a concrete exercise,
command, failure signal, and chapter location. It also keeps feedback public
and impersonal: do not include private data, local secrets, or personal
background details that are not needed to improve the exercise.

Use the answer key after the attempt record. If the answer key explains the
concept but not the failure you saw, that is evidence that the exercise needs a
better hint, pass condition, or worked example.

Open an
[exercise clarity report](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=chapter-clarity.yml&title=%5Bgood+first+feedback%5D+exercise+attempt+brief&location=book%2Fsrc%2Fexercises.md+or+exercises%2FREADME.md&command=exercise+command+tried)
after you have one concrete attempt record. The link fills the route, not the
evidence; the evidence signal should come from what you personally read, ran,
or attempted.

## Worked Example

First study a complete answer. The exercise is:

```text
Explain why TokenId is not a raw usize.
```

A strong answer:

```text
Rust syntax:
TokenId is a tuple struct around usize. The field is private, so callers use
TokenId::new and index() instead of reaching into the raw value directly.

ML concept:
The number represents a vocabulary position, not an arbitrary count or shape.

Category theory concept:
TokenId is one object in the small category of typed pipeline values. Morphisms
such as Embedding can start from it.
```

Notice the order: name the syntax, connect it to the ML role, then name only the
categorical shape the code supports.

## Worked Example: Gradient Checking

This worked example supports Advanced Exercise 5 in
`exercises/advanced/README.md`.

The exercise asks why a finite-difference test compares:

```text
inferred gradient from one training update
central finite difference of average loss
```

The reason is that these are two independent ways to ask the same local
question:

```text
If I nudge this parameter, how does the loss move?
```

CS231n presents this as the difference between numerical gradients and analytic
gradients: the numerical version is slower and approximate, but useful for
checking whether the analytic implementation is correct. Dive into Deep
Learning explains the matching training shape from the other direction:
backpropagation walks the computation in reverse order, stores intermediate
values, and computes gradients for parameters. This project makes that idea
small enough to inspect in Rust.

PyTorch's `gradcheck` documentation gives the same engineering warning in
framework form: the check compares small finite differences against analytical
gradients and accepts agreement only within tolerance. It also calls out
practical caveats such as precision, non-differentiable points, and overlapping
memory. Translate that into this Rust lab as:

```text
finite-difference match = useful local debugging signal
finite-difference match != proof of every gradient path
```

The code-level test has two paths.

The first path performs one training step:

```text
before parameter
-> TransformerBlockTrainStep
-> after parameter
```

From that update, the test infers the gradient:

```text
inferred_gradient = (before_value - after_value) / learning_rate
```

That matches gradient descent:

```text
parameter <- parameter - learning_rate * gradient
```

The second path does not trust the training step. It clones the same state,
changes one parameter in two directions, and measures the loss:

```text
loss_plus  = loss(parameter + epsilon)
loss_minus = loss(parameter - epsilon)
```

Then it estimates the local slope:

```text
finite_difference = (loss_plus - loss_minus) / (2 * epsilon)
```

A strong answer for one parameter family looks like this:

```text
Rust syntax:
The test selects one feed-forward bias entry, clones the training state twice,
adds epsilon to the entry in one clone, subtracts epsilon in the other clone,
and calls transformer_block_average_loss on both states.

ML concept:
The bias is a trainable parameter. The central finite difference estimates how
the average loss changes around the current bias value. The one-step update
infers the gradient that backpropagation used. If both slopes match, the
implemented update has the right local sign and scale for that parameter.

Category theory concept:
The training step is an endomorphism on TransformerTrainingState. The check
asks whether this state update agrees locally with the loss morphism that it is
supposed to reduce.

What failure would this test catch?
It would catch a missing bias gradient, a reversed update sign, a dropped path
through the feed-forward block, or a mismatch between averaged loss and summed
gradients.
```

The important habit is not the formula by itself. The habit is triangulation:

```text
implementation path
numerical measurement
conceptual explanation
```

When all three agree, the code becomes easier to trust and easier to teach.
If the two numbers disagree, do not immediately change the test tolerance. Ask
which boundary failed first:

```text
wrong sign?
missing path?
wrong averaging scale?
non-smooth point?
parameter aliasing or shared storage?
```

That is why the exercise asks for a specific parameter family. A focused
finite-difference check is a microscope, not a certificate for the whole
training system.

## Partially Completed Example

Complete the missing lines for `Distribution`:

```text
Rust syntax:
Distribution wraps ________ and construction can return ________.

ML concept:
It represents probabilities over possible next tokens, so the values must be
non-negative and sum to ________.

Category theory concept:
It is an object produced by ________ and consumed with a target token by
________.
```

Expected completion:

```text
Vec<f32>
CtResult<Self>
one
Softmax
CrossEntropy
```

## Your Turn

Now solve the same kind of exercise without the filled answer. Pick `Loss`,
`TrainingSet`, or `LearningRate` and explain it through the same three lenses.

## Transfer Exercise

Design a wrapper type for the attention roadmap or a future Transformer chapter,
such as `SequenceLength`, `HeadCount`, or `AttentionScores`. State the raw
representation, the invariant, and one function that should consume or produce
it.

Expected failure to consider:

```text
What should the constructor reject?
```

If the answer is "nothing," the type may be only a semantic wrapper. If the
answer is "zero heads," "empty sequence," or "probability outside the allowed
range," the type needs a validating constructor.

## Exercise 1: Explain One Domain Type

Use [Domain Objects](01-domain-objects.md).

Pick one type:

- `Vector`
- `Logits`
- `Distribution`
- `Loss`
- `TrainingSet`
- `Parameters`

Write:

```text
The problem this solves:

Rust syntax:

ML concept:

Category theory concept:
```

Pass condition:

- You name the raw representation.
- You name the invariant or semantic distinction.
- You name the pipeline stage where the type appears.

First-principles hint:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct LocalTokenId(usize);

impl LocalTokenId {
    fn new(index: usize) -> Self {
        Self(index)
    }

    fn index(self) -> usize {
        self.0
    }
}

assert_eq!(LocalTokenId::new(7).index(), 7);
```

That snippet is intentionally smaller than the real crate. It shows the raw
idea: a named wrapper can make one `usize` mean "token id" instead of "any
number."

## Exercise 2: Add A Token

Use the `src/demo.rs` snapshot in [Course Map](00-map.md).

Add one new vocabulary item and extend the token sequence.

Run:

```bash
cargo run --bin category_ml
```

Pass condition:

- the demo still runs
- the dataset windowing output includes your new transition
- you can explain why a longer `TokenSequence` creates more training examples

Debugging hint:

If the output does not include the new transition, check whether you changed
both the vocabulary and the token sequence. A vocabulary entry alone does not
create a training pair. The pair appears only when two token ids are adjacent in
the sequence.

## Exercise 3: Trace `DatasetWindowing`

Use [The Tiny ML Pipeline](03-ml-pipeline.md).

For this input:

```text
[TokenId(4), TokenId(8), TokenId(15), TokenId(16)]
```

write the training examples produced by `windows(2)`.

Then explain:

```text
Rust syntax:
what does `.windows(2)` do?

ML concept:
why does next-token training need adjacent pairs?

Category theory concept:
why is each example a product object?
```

Check yourself before reading onward:

```text
(TokenId(4), TokenId(8))
(TokenId(8), TokenId(15))
(TokenId(15), TokenId(16))
```

The syntax creates overlapping adjacent windows. The ML idea is next-token
supervision: each input token is paired with the token that follows it. The
category-theory shape is a product object because each training example carries
two typed values together.

## Exercise 4: Break A Composition

Use the `examples/02_morphism_composition.rs` snapshot in
[Morphism and Composition](02-morphisms-composition.md).

Try to compose `Embedding` directly with `Softmax`.

Expected failure shape:

```text
the trait bound ... is not satisfied
```

Then restore the working version.

Explain:

```text
Rust syntax:
which type did the compiler reject?

Composition diagnostic:
first source:
first target:
second source:
second target:
which middle object should connect the stages?

ML concept:
which prediction stage was skipped?

Category theory concept:
which middle object failed to match?
```

Pass condition:

- You name `Embedding : TokenId -> Vector` and
  `Softmax : Logits -> Distribution`.
- You identify `Vector` versus `Logits` as the failed middle-object match.
- You restore `LinearToLogits : Vector -> Logits` instead of weakening
  `Softmax`.
- You explain why the skipped ML stage is vocabulary scoring.

Debugging hint:

Do not fix this by changing the type signatures. Restore the missing stage
instead. The intended path is:

```text
TokenId -> Vector -> Logits -> Distribution
```

## Exercise 5: Change The Training Repetition Count

Use the `examples/03_training_endomorphism.rs` snapshot in
[Training as an Endomorphism](04-training-endomorphism.md).

Change:

```rust,ignore
StepCount::new(80)
```

to:

```rust,ignore
StepCount::new(1)
StepCount::new(10)
StepCount::new(200)
```

Run:

```bash
cargo run --example 03_training_endomorphism
```

Explain the result:

```text
Rust syntax:
where is the count used?

Training diagnostic:
what object is updated?
what object measures quality?
what repeats?
what controls update size?

ML concept:
what happens when training repeats more times?

Category theory concept:
why can the update be repeated?
```

Expected observation:

One step should preserve the shape of the parameters but may not reduce loss
much. More steps usually make the tiny example improve until the hand-written
training rule reaches its limit. The important category-theory point is not
"more is always better"; it is that the same update has the shape:

```text
Parameters -> Parameters
```

Pass condition:

- You distinguish `TrainStep : Parameters -> Parameters` from
  `Parameters x TrainingSet -> Loss`.
- You explain that loss is a measurement, not the updated model state.
- You identify `StepCount` as repetition of the same update shape.
- You avoid claiming that more steps always means better behavior.

## Exercise 6: Explain `Distribution<T>::map`

Use [Functors, Naturality, Monoids, and Chain Rule](05-structure-and-calculus.md).

Explain the conceptual `Distribution<T>::map` example.

Use this input distribution:

```text
TokenId(2) -> 0.70
TokenId(3) -> 0.30
```

and this function:

```text
TokenId -> String
```

where:

```text
TokenId(2) -> "Rust"
TokenId(3) -> "."
```

Write the output distribution.

Then explain:

```text
Rust syntax:
why does `self` plus `into_iter()` move the old outcomes?

ML concept:
why do the probabilities stay the same?

Category theory concept:
what does it mean to lift `T -> U` into `Distribution<T> -> Distribution<U>`?
```

## Exercise 7: Explain One Validation Boundary

Pick one constructor:

- `Distribution::new`
- `Loss::new`
- `LearningRate::new`
- `TrainingSet::new`
- `SignalMatrix::new`
- `OpenCircuit::new`

Write:

```text
The problem this solves:

Rust syntax:
which condition returns `Err(...)`?

ML or software concept:
what bad runtime behavior does this prevent?

Category theory concept:
what intended object or relationship is being protected?
```

## Exercise 8: Trace A Full Source File

Use [Repository Source Snapshots](source-snapshots.md).

Pick one complete source file and write a five-sentence summary:

1. What problem does the file solve?
2. What are the main Rust types or traits?
3. What ML or software concept does it model?
4. What category-theory concept does it teach?
5. Which command proves the file still works?

## Exercise 9: Connect One External Reference

Use [References](references.md).

Pick one external resource and connect it to one source file in this course.
First classify the source using the source-role table in the references
chapter.

Answer:

```text
External resource:
Source role:
Owned boundary:
Source file:
Rust syntax connection:
ML or software concept connection:
Category theory concept connection:
What this source can support:
What this source cannot support:
One difference between the full treatment and this tiny implementation:
```

Pass condition:

- You classify the source as official documentation, academic paper, open
  textbook or university material, implementation bridge, or learner-friction
  signal.
- You name the boundary the source owns.
- You connect it to one concrete source file, type, function, test, or example.
- You state one claim the source does not license this book to make.

## Exercise 10: Test One Sketch Law

Use [Seven Sketches Through Rust](seven-sketches-rust.md).

Pick one law from `src/sketches.rs`:

- preorder laws
- feature/layer Galois law
- resource monotonicity
- foreign-key resolution
- co-design feasibility relation
- signal-flow matrix composition
- local-to-global safety truth

Change one input in `examples/05_seven_sketches.rs`, then run:

```bash
cargo run --example 05_seven_sketches
```

Pass condition:

- you can explain which law still holds
- you can explain which constructor or method prevents invalid structure
- your explanation uses Rust syntax, ML or software concept, and category theory concept

Negative test option:

Instead of changing the runnable example, inspect one of the negative tests in
`src/sketches.rs`:

- missing database reference
- mismatched signal-matrix middle dimension
- open-circuit serial boundary mismatch

Explain what invalid structure the test rejects. This is often the fastest way
to understand what a law or constructor is protecting.

PDF-to-Rust contract option:

Use the chapter's `PDF-To-Rust Reading Contract`. Pick one source idea from
the Seven Sketches chapter and fill this row:

```text
source idea from the PDF:
Rust handle:
protected law, relation, or boundary:
larger source claim not implemented by this code:
local evidence command or test:
```

If the source idea still feels too large, fill the chapter's transfer triage
card before writing the final answer:

```text
source idea:
local Rust handle:
protected law, relation, or boundary:
invalid shortcut rejected:
tiny ML transfer:
larger claim not implemented:
local evidence command or test:
```

Pass condition:

- your Rust handle is one concrete type, method, constructor, example output
  line, or test from `src/sketches.rs`
- your protected claim is smaller than the full source text
- your evidence can be checked with `cargo run --example 05_seven_sketches`
  or `cargo test sketches::tests --lib`
- your transfer card names one invalid shortcut and one non-claim

Co-design option:

Use `DesignRequirement`, `ImplementationOffer`, and `FeasibilityRelation`.
Write the relation as:

```text
DesignRequirement x ImplementationOffer -> bool
```

Then translate it to:

```text
ArchitectureConstraint x CandidateImplementation -> Bool
```

Pass condition:

- you explain why this is a relation rather than a function
- you give one passing offer and one failing offer
- you say why one passing implementation does not prove the whole constraint
  space

## Exercise 11: Write A New Block Explanation

Choose any block from the source snapshots that the chapter did not explain in
enough detail for you.

Write a block explanation using this structure:

```text
The problem this block solves:

The whole block:

Rust syntax:

ML or software concept:

Category theory concept:

Core mental model:
```

Pass condition:

- A beginner can understand the Rust syntax.
- An ML learner can understand why the block exists.
- A category-theory learner can name the shape.

## Exercise 12: Trace Attention Shape Flow

Use [Transformer Roadmap](roadmap.md), `src/attention.rs`, and
`examples/06_attention_scores.rs`.

Run:

```bash
cargo run --example 06_attention_scores
```

Write down the first time the output mentions each shape:

```text
AttentionScores:
AttentionWeights:
AttentionOutput:
MultiHeadOutput:
ProjectedAttentionOutput:
HiddenSequence after residual:
HiddenSequence after normalization:
HiddenSequence after feed-forward:
```

Then explain:

```text
Rust syntax:
which named type or boundary protects each shape?

ML concept:
what changes between scores, weights, value mixing, projection, residual,
normalization, and feed-forward?

Category theory concept:
where does the path use a product input, and where does it return to the same
HiddenSequence object?
```

Then classify these boundaries:

```text
QuerySequence x KeySequence -> AttentionScores:
AttentionScores x AttentionMask -> AttentionScores:
AttentionScores -> AttentionWeights:
AttentionWeights x ValueSequence -> AttentionOutput:
LayerNormalization : HiddenSequence -> HiddenSequence:
TransformerTrainingState -> TransformerTrainingState:
HiddenSequence x MultiHeadOutput -> HiddenSequence:
```

Then repeat the quick roadmap classification drill without looking at the
answer table. For each boundary, count the inputs first, then name the safest
category shape:

```text
HiddenSequence -> QuerySequence:
AttentionScores x AttentionMask -> AttentionScores:
LayerNormalization : HiddenSequence -> HiddenSequence:
HiddenSequence x ProjectedAttentionOutput -> HiddenSequence:
TransformerTrainingState -> TransformerTrainingState:
```

Finally, explain this trap in one sentence:

```text
A product input that returns its left-hand object is not automatically an
endomorphism.
```

Then use the same-output classification rule from the roadmap. These three
lines all end with `HiddenSequence`; explain why they do not have the same
category shape:

```text
LayerNormalization : HiddenSequence -> HiddenSequence:
HiddenSequence x ProjectedAttentionOutput -> HiddenSequence:
HiddenSequence x MultiHeadOutput -> HiddenSequence:
```

Then answer the terminal-output audit. For each printed line, write what the
line proves and what category overclaim it does not prove:

```text
projected attention shape: 2 positions x model dimension 2
residual shape: 2 positions x model dimension 2
masked multi-head block shape: 2 positions x model dimension 2
training state step: 0 -> 1
```

Use this rule:

```text
printed shape line -> target evidence
typed transformation line -> source and target evidence
category name -> only after both are known
```

Then answer the source-ownership diagnostic:

```text
Self-attention:
which sequence owns the query side?
which sequence owns the key side?
which sequence owns the value side?

Cross-attention:
which sequence owns the query side?
which sequence owns the key side?
which sequence owns the value side?

Shape check:
which length counts score rows?
which length counts score columns?
```

Then fill the shape ledger:

```text
target length:
source length:
attention mask:
attention output:
```

For each row, write:

```text
framework cue -> Rust roadmap meaning -> category-shape consequence
```

Then answer the mask-role ledger:

```text
What does an attention-mask cell select?
Why is the mask not a shorter token sequence?
Why does the mask not directly produce AttentionWeights?
Which block-level boundary keeps the mask visible instead of hidden?
In a fixed-mask view, what context was selected first?
What does true mean in this repository's AttentionMask?
Why can a framework mask with the same shape still need boolean inversion?
Write the three-step rule:
  mask cells ...
  softmax ...
  weights ...
```

Then answer the linear-scope diagnostic:

```text
Which listed boundaries are the linear Q/K/V projections?
Which boundary turns scores into nonlinear normalized weights?
Which product-input boundaries must not be collapsed into one unary map?
Which state endomorphism belongs to training rather than forward attention?
```

Then answer the source-scope diagnostic:

```text
Which source supports decomposing attention into recurring components?
Which source supports comparing the linear Q/K/V part with advanced category theory?
What does neither source license you to claim about the whole Rust roadmap block?
What is the local Rust contract for every component in this book?
```

Then answer the architecture-constraint diagnostic:

```text
What is one architecture constraint in the roadmap?
Which Rust type, constructor, example, or test is implementation evidence for it?
Why is that implementation evidence not the same as proving the whole future
Transformer architecture satisfies every intended constraint?
```

Then answer the stackability diagnostic:

```text
Which listed boundaries can stack directly as HiddenSequence -> HiddenSequence?
Why is MaskedMultiHeadTransformerBlock not an endomorphism while the mask is
still an open input?
What are the two precise ways to repeat a masked block?
When is a fixed-mask view allowed to be named HiddenSequence -> HiddenSequence?
When is LayerNormalization allowed to be named HiddenSequence -> HiddenSequence?
When is PositionalEncoding allowed to be named HiddenSequence -> HiddenSequence?
When is MultiHeadTransformerBlock allowed to be named HiddenSequence -> HiddenSequence?
If the layer's scale and shift are being learned, which larger boundary owns
that change?
```

Then answer the context-fixing drill:

```text
Open masked block:
what is the whole input object?
what is the safe category shape?
can it stack unaided as HiddenSequence -> HiddenSequence?

Fixed-mask view:
what was selected first?
what is the induced boundary?
what promise must remain true while stacking?

Changing mask per call:
what must the caller supply or carry?
why is this not the same as a fixed-mask view?

Residual addition:
which two inputs remain visible?
why is this not a unary endomorphism?
if you name the whole product as the source object, why is
  (HiddenSequence x ProjectedAttentionOutput) -> HiddenSequence
  still not an endomorphism?

Rust closure bridge:
what value would a closure capture to create a fixed-mask view?
which argument would remain when the closure is called?
why does the closure analogy still not change the open block boundary?
```

Then answer the add-norm order drill:

```text
Which order does the current Rust block implement around the attention sublayer?
Which order does it implement around the feed-forward sublayer?
Which two local boundaries show the order?
Why can post-norm and pre-norm blocks both have shape
  HiddenSequence -> HiddenSequence
  while still being different morphisms?
If a future pre-norm variant is added, what must be named separately?
```

Before naming each boundary, write the answer to the first diagnostic question:

```text
How many inputs does this boundary require?
```

Then write the source-target audit card for at least three boundaries:

```text
boundary:
whole source object:
target object:
context status:
safe conclusion:
```

Use at least one product-input boundary and one fixed-context boundary.

Pass condition:

- You name at least four concrete Rust types from `src/attention.rs`.
- You distinguish raw attention scores from normalized attention weights.
- You explain why residual addition must return to `HiddenSequence`.
- You connect one terminal output line to one typed boundary.
- You explain why self-attention shares a source before projection without
  collapsing query, key, and value into one role.
- You map target length, source length, attention mask, and attention output
  from framework notation to the Rust roadmap shape ledger.
- You explain that mask cells select legal score cells before softmax, not token rows after probability mass has been assigned.
- You state that this repository's `AttentionMask` uses `true` for an allowed
  source position, while some framework masks use `true` for a blocked or
  padding position.
- You keep claims about linear Q/K/V projections separate from softmax,
  masking, residual addition, normalization, and training state.
- You classify the quick roadmap drill by counting inputs before naming
  endomorphisms.
- You explain that anatomy-of-attention research supports decomposition, while
  parametric-endofunctor research supports a narrower linear self-attention
  comparison.
- You separate architecture constraints from implementation boundaries.
- You do not claim the tiny Rust roadmap implements either full formalism.
- You distinguish an open masked-block product input from a fixed-mask induced
  endomorphism.
- You name what context was fixed before using the fixed-mask
  `HiddenSequence -> HiddenSequence` view.
- You state that a shape-preserving layer is an endomorphism only for a fixed
  module instance, while parameter changes belong to
  `TransformerTrainingState -> TransformerTrainingState`.
- You state that positional encodings and Transformer blocks follow the same
  fixed-value rule: the table or block value must already be selected before
  the forward call is named `HiddenSequence -> HiddenSequence`.
- You state that residual-normalization order is part of the morphism, so
  post-norm and pre-norm blocks can share source and target while remaining
  different implementations.
- You classify at least one product-input morphism, one endomorphism, and one
  illegal boundary.
- You do not call a product-input boundary an endomorphism only because its
  output matches the left input object.
- You write the whole source object and target object before deciding whether a
  row is an endomorphism.
- You explain that naming the whole product as the source object gives a unary
  morphism out of the product, not an endomorphism unless the same product
  object is also returned.
- You explain why two lines that return `HiddenSequence` can still have
  different category shapes.

## Exercise 13: Compute Cross-Entropy From Target Probability

Use [The Tiny ML Pipeline](03-ml-pipeline.md) and `src/ml.rs`.

The `CrossEntropy` morphism uses:

```text
loss = -ln(probability assigned to the target token)
```

For target token `TokenId(0)`, compare these two distributions:

```text
confident = [0.90, 0.10]
surprised = [0.10, 0.90]
```

Compute:

```text
confident loss:
surprised loss:
which one is lower:
```

Then run:

```bash
cargo test cross_entropy_is_lower_for_more_confident_target_probability --lib
```

Explain:

```text
Rust syntax:
which code reads the target probability, and which constructor validates the
loss?

ML concept:
why does the same target token produce different losses under the two
distributions?

Category theory concept:
why is CrossEntropy a morphism from Distribution x TokenId to Loss?
```

Pass condition:

- You compute approximate losses for `0.90` and `0.10`.
- You explain why the target index is `0` in both cases.
- You connect the test name to the learning claim.

## Exercise 14: Trace Naturality And Monoid Laws

Use [Functors, Naturality, Monoids, and Chain Rule](05-structure-and-calculus.md)
and `src/structure.rs`.

Run:

```bash
cargo test structure::tests --lib
```

For the naturality square, write the two paths:

```text
top then right:
left then bottom:
why they should match:
```

For the monoid law check, write the three laws:

```text
left identity:
right identity:
associativity:
```

Then explain:

```text
Rust syntax:
which functions or methods implement each path or law?

ML or software concept:
why do consistent wrapper conversion and trace grouping matter in a pipeline?

Category theory concept:
what does commutativity mean for the square, and what does associativity mean
for the trace monoid?
```

Pass condition:

- You name `naturality_square_holds_for_first_option`.
- You name `monoid_laws_hold_for_pipeline_trace`.
- You explain why both naturality paths return the same `Option` value.
- You explain why changing parentheses in trace combination should not change
  the final trace.

## Exercise 15: Mixed Boundary Diagnosis

Use this exercise after finishing the core chapters. The goal is interleaved
transfer: diagnose which kind of boundary is being protected without being told
which chapter the failure came from.

For each case, classify the boundary:

```text
invariant boundary
composition boundary
endomorphism boundary
shape boundary
local-to-global boundary
```

Then answer with the usual three lenses.

### Cases

```text
1. A raw usize is used where the code expects TokenId.
2. Embedding is followed directly by Softmax.
3. CrossEntropy receives Logits instead of Product<Distribution, TokenId>.
4. A training step returns Loss instead of Parameters.
5. SignalMatrix::compose_after sees mismatched middle dimensions.
6. SafetyCover reports a global claim even though one interval is false.
7. A residual connection tries to add rows with different model dimensions.
```

For each case, write:

```text
Boundary type:

Rust syntax:

ML or software concept:

Category theory concept:

Smallest useful fix:
```

Pass condition:

- You classify all seven cases.
- You name at least five concrete Rust types or functions.
- You explain the smallest useful fix without weakening the type boundary.
- You identify which cases are about invalid values, which are about invalid
  composition, and which are about invalid global claims.

Debugging hint:

Do not answer every case with "the compiler rejects it." Some failures are
constructor errors, some are returned `CtError::ShapeMismatch`, some are
conceptual category-shape failures, and some are law-check failures. The skill
is choosing the right explanation for the right boundary.

## Exercise 16: Trace Transformer Training State

Use [Transformer Roadmap](roadmap.md), `src/attention.rs`, and
`examples/07_transformer_training_state.rs`.

Run:

```bash
cargo run --example 07_transformer_training_state
```

Write down the output lines for:

```text
initial state:
forward shape:
readout update:
feed-forward update:
composed block update:
```

Then classify each update:

```text
TransformerReadoutTrainStep:
TransformerFeedForwardTrainStep:
TransformerBlockTrainStep:
```

For each update, answer with the three lenses:

```text
Rust syntax:
which named type performs the update, and what state does it return?

ML concept:
which parameters or sublayer does this update train?

Category theory concept:
why is the outside shape an endomorphism?
```

Finally, explain why this shortcut would be weaker:

```text
readout update returns readout weights
feed-forward update returns feed-forward weights
block update returns a bag of changed matrices
```

Pass condition:

- You name `TransformerTrainingState`, `TinyTransformerParameters`, and all
  three training-step types.
- You explain that the state owns parameters, learning rate, and step count.
- You distinguish readout-only, local feed-forward, and composed block updates.
- You connect each printed step increment to
  `TransformerTrainingState -> TransformerTrainingState`.
- You explain why returning loose weights would make the next update rebuild
  context by hand.

## Retrieval Practice

Close the source file before answering these prompts.

### Recall

Name three kinds of feedback this course uses:

```text
compiler error
constructor error
test failure
```

### Explain

Explain why a failed composition is useful evidence, not only an obstacle.

### Apply

Pick one exercise you solved and rewrite it for a different type or module.
Keep the same answer shape:

```text
Rust syntax:
ML or software concept:
Category theory concept:
```

## Where This Leaves Us

If you can complete these exercises, you can read the project without treating
category theory, Rust, and ML as three disconnected subjects. You can start from
a line of code, name the syntax, identify the software or ML role, and then
describe the categorical shape only as far as the code justifies it.
