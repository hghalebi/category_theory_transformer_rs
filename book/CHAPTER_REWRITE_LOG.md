# Chapter Rewrite Log

Status: Active

This log records source-backed chapter rewrite passes. It is an editorial
artifact for keeping the book coherent over multiple iterations.

## Success Criteria

A chapter pass is complete only when the chapter:

- starts from a practical problem,
- activates prior knowledge,
- contains a worked example,
- prompts self-explanation,
- ties claims to repository code and chapter references,
- explains Rust syntax, ML meaning, and category-theory shape,
- ends with retrieval practice,
- passes prose/style, coverage, book, and Rust validation gates.

## Pass 1: Reference Surface And Welcome

Files:

- `book/src/references.md`
- `book/STYLE_GUIDE.md`
- `book/EDITORIAL_RESEARCH_NOTES.md`
- `book/src/welcome.md`
- `scripts/check-mdbook-coverage.sh`

Primary sources:

- [How People Learn II](https://www.nationalacademies.org/projects/DBASSE-BBCSS-13-06/publication/24783)
- [Rust By Example](https://doc.rust-lang.org/rust-by-example/index.html)
- [Seven Sketches in Compositionality](https://arxiv.org/abs/1803.05316)
- [Dive into Deep Learning](https://d2l.ai/)

Critique before rewrite:

The book had the right pedagogical direction, but the source-to-chapter mapping
was not explicit enough. The Welcome chapter introduced the three-lens method,
but it could do more work as the public learning contract: first run, central
thesis, what the book is not, and how to use the repository.

Rewrite decision:

Make the reference chapter a chapter-by-chapter source map. Make the style
guide preserve a repeatable source-backed rewrite loop. Rewrite Welcome as the
first public teaching contract around "executable structure, not AI magic."

Validation:

```bash
python3 scripts/check-prose-style.py
bash scripts/check-mdbook-coverage.sh
git diff --check -- . ':!target'
bash scripts/check.sh
```

All passed.

Full gate rerun:

```bash
bash scripts/check.sh
```

Result: 34 Rust tests passed, all examples ran, the demo binary ran, prose
checks passed, source coverage passed, the book built, and chapter tests passed.

## Pass 2: Course Map

File:

- `book/src/00-map.md`

Primary sources:

- [Rust modules](https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html)
- [Seven Sketches in Compositionality](https://arxiv.org/abs/1803.05316)
- [Category Theory for Programming](https://arxiv.org/abs/2209.01259)
- `src/lib.rs`
- `src/demo.rs`
- `src/bin/category_ml.rs`

Critique before rewrite:

The chapter had the correct source inventory and runnable commands, but it
still read partly like a file map. The learner needed a stronger mental model
before the inventory: one function, one typed movement, one full pipeline, then
the module map.

ML teaching critique:

The original version named the ML stages, but the explanation could better show
why raw data becomes prediction, loss, and repeated updates.

Visual/tutorial critique:

The chapter needed a clearer progression from one small function to the full
pipeline diagram before reading the modules.

Category-theory critique:

The terms object, morphism, composition, endomorphism, and law needed to be
presented as names for already-visible code shapes, not as a glossary list.

Learner critique:

A motivated reader could understand the file list but still ask, "What should I
look for when I run the demo?" The rewrite makes the demo output a miniature
course outline.

Rewrite decision:

Rewrite the chapter around this sequence:

```text
one function
  -> typed movement
  -> whole pipeline
  -> module map
  -> guided demo
  -> first run
  -> retrieval practice
```

Validation:

```bash
python3 scripts/check-prose-style.py
bash scripts/check-mdbook-coverage.sh
git diff --check -- . ':!target'
bash scripts/check.sh
```

All passed.

## Pass 3: Domain Objects

File:

```text
book/src/01-domain-objects.md
```

Primary sources:

- [Rust structs](https://doc.rust-lang.org/book/ch05-01-defining-structs.html)
- [Rust enums](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html)
- [Rust `Result`](https://doc.rust-lang.org/stable/book/ch09-02-recoverable-errors-with-result.html)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/checklist.html)
- `src/domain.rs`
- `examples/01_domain_objects.rs`

Critique before rewrite:

The chapter already had strong source-by-source explanation. The weakness was
not missing detail. The weakness was that the reader could enter a long source
snapshot without a clean distinction between objects that only separate meaning
and objects that enforce invariants.

ML teaching critique:

The chapter should make clear that not every domain type has the same job.
`TokenId` prevents confusion. `Distribution` prevents invalid probability mass.
Those are both useful, but they are not the same kind of boundary.

Category-theory critique:

The chapter should not imply that every wrapper is mathematically deep. Some
objects are simple typed endpoints. Some carry stronger invariants. The
category-theory reading should follow the code's actual boundary.

Learner critique:

A motivated reader could ask, "Why does `TokenId::new` never fail, but
`Distribution::new` can fail?" The rewrite adds that distinction before the
full source snapshot.

Rewrite decision:

- confirm every domain object answers "what confusion does this type prevent?",
- tie smart constructors and private fields to Rust API guidance,
- improve any section where the category-theory reading appears before the code
  shape is concrete,
- keep the chapter long enough for real teaching but reduce avoidable
  repetition.

Implemented as a focused patch: add "Two Kinds Of Domain Objects", strengthen
the chapter opening, clarify chapter order, tighten the "Why This Matters"
claim, and update further-reading wording.

Validation:

```bash
python3 scripts/check-prose-style.py
bash scripts/check-mdbook-coverage.sh
git diff --check -- . ':!target'
```

All passed. The full validation gate should be rerun before publishing the
combined tranche.

## Pass 4: Morphism And Composition

Files:

```text
book/src/02-morphisms-composition.md
src/category.rs
```

Primary sources:

- [Rust generics](https://doc.rust-lang.org/book/ch10-01-syntax.html)
- [Rust traits](https://doc.rust-lang.org/book/ch10-02-traits.html)
- [Seven Sketches in Compositionality](https://arxiv.org/abs/1803.05316)
- [Category Theory for Programming](https://arxiv.org/abs/2209.01259)
- `src/category.rs`
- `examples/02_morphism_composition.rs`

Critique before rewrite:

The chapter already explained `Morphism`, `Identity`, `Compose`, and
`Endomorphism`, but the word "morphism" needed to land first as a concrete
typed transformation. The law discussion also needed more executable support in
`src/category.rs`, not only prose claims.

ML teaching critique:

The prediction path is the central ML use case. The chapter should make it
obvious that skipping `LinearToLogits` is not just a bad diagram; it breaks the
typed boundary between `Vector` and `Logits`.

Category-theory critique:

The chapter should stay modest. It can model typed arrows, identity,
composition, and repeatable endomorphisms, but it should not imply that every
categorical law is encoded in Rust's type system.

Learner critique:

A reader may ask where composition actually checks anything. The rewrite adds
the habit: when composition feels abstract, look for the middle type.

Rewrite decision:

- make "morphism" first read as a typed transformation,
- show composition failure and success through Rust types,
- connect identity and composition behavior to executable tests,
- avoid presenting category-theory laws before the reader has seen the code
  shape.

Implemented as:

- a new `From Function To Morphism` section,
- stronger explanation of `Middle` as the bridge type,
- explicit modesty around what the code does and does not prove,
- new `src/category.rs` tests for identity composition, composition order, and
  first-error propagation.

Validation:

```bash
cargo fmt --check
cargo test category::tests --lib
python3 scripts/check-prose-style.py
bash scripts/check-mdbook-coverage.sh
git diff --check -- . ':!target'
```

All passed. The full validation gate should be rerun before publishing the
combined tranche.

## Pass 5: Tiny ML Pipeline

Files:

```text
book/src/03-ml-pipeline.md
src/ml.rs
```

Primary sources:

- [Dive into Deep Learning: Softmax Regression](https://d2l.ai/chapter_linear-classification/softmax-regression.html)
- [Dive into Deep Learning: Softmax Regression from Scratch](https://d2l.ai/chapter_linear-classification/softmax-regression-scratch.html)
- [CS231n: Linear Classification](https://cs231n.github.io/linear-classify/)
- [Deep Learning](https://www.deeplearningbook.org/)
- `src/ml.rs`
- `examples/02_morphism_composition.rs`

Critique before rewrite:

The chapter already walked through the concrete ML morphisms, but the reader
needed a clearer intuition boundary before the detailed code: logits are raw
scores, distributions are probabilities, and loss is a scalar penalty derived
from the probability assigned to the target.

ML teaching critique:

Softmax and cross entropy should not arrive only as implementation blocks.
The chapter should first separate raw scores, normalized probabilities, and
surprise about the target.

Category-theory critique:

The chapter should continue treating category-theory language as naming the
pipeline shape. The central category-theory claims here are composition,
product input to loss, and the direct-vs-composed path check.

Learner critique:

A reader may understand `Logits -> Distribution` mechanically but still ask why
loss goes down when the target probability goes up. The rewrite adds that
intuition and backs it with tests.

Rewrite decision:

- explain logits before probabilities,
- make softmax and cross entropy concrete before equations,
- connect the Rust morphisms in `src/ml.rs` to the ML pipeline references,
- make the composed prediction path and direct prediction check feel like a
  commutative-diagram sanity check, not a detached theorem.

Implemented as:

- a new `Scores, Probabilities, And Loss` section,
- sharper ML prose in the softmax and cross-entropy sections,
- new `src/ml.rs` tests showing softmax normalization and lower cross entropy
  for higher target probability.

Validation:

```bash
cargo fmt --check
cargo test ml::tests --lib
python3 scripts/check-prose-style.py
```

All passed. The full validation gate should be rerun before publishing the
combined tranche.

## Pass 6: Training As An Endomorphism

Files:

```text
book/src/04-training-endomorphism.md
src/training.rs
```

Primary sources:

- [Dive into Deep Learning: Gradient Descent](https://d2l.ai/chapter_optimization/gd.html)
- [Dive into Deep Learning: Backpropagation and Computational Graphs](https://d2l.ai/chapter_multilayer-perceptrons/backprop.html)
- [Backprop as Functor](https://arxiv.org/abs/1711.10455)
- `src/training.rs`
- `examples/03_training_endomorphism.rs`

Critique before rewrite:

The chapter already explained the training implementation in detail, but it
could better separate one training step from repeated training before entering
the source walkthrough. The endomorphism claim also deserved tests for the
one-step contract, not only the repeated-loss sanity check.

ML teaching critique:

The learner should first see the ordinary gradient-descent story: compute
predictions, compute gradients, subtract a learning-rate-scaled average, and
return updated parameters.

Category-theory critique:

`Parameters -> Parameters` is the real shape. The chapter should present this
as one update step first, then explain iteration as repeated application of the
same endomorphism.

Learner critique:

A reader may believe the chapter only proves that loss goes down after 80
steps. The rewrite adds tests showing a single step preserves parameter shape
and that invalid targets fail at the training boundary.

Rewrite decision:

- explain one update step before repeated training,
- make `Parameters -> Parameters` feel like the natural shape of training,
- connect loss reduction to the actual `TrainStep` implementation,
- separate the tiny hand-coded update from full automatic differentiation.

Implemented as:

- a new `One Step Before Many Steps` section,
- prose that points to the one-step and repeated-step tests,
- new `src/training.rs` tests for parameter-shape preservation and out-of-range
  target rejection.

Validation:

```bash
cargo fmt --check
cargo test training::tests --lib
python3 scripts/check-prose-style.py
```

All passed. The full validation gate should be rerun before publishing the
combined tranche.

## Pass 7: Structure And Calculus

Files:

```text
book/src/05-structure-and-calculus.md
src/structure.rs
src/calculus.rs
```

Primary sources:

- [Backprop as Functor](https://arxiv.org/abs/1711.10455)
- [Dive into Deep Learning: Backpropagation and Computational Graphs](https://d2l.ai/chapter_multilayer-perceptrons/backprop.html)
- [Seven Sketches in Compositionality](https://arxiv.org/abs/1803.05316)
- [Category Theory for Programming](https://arxiv.org/abs/2209.01259)
- `src/structure.rs`
- `src/calculus.rs`
- `examples/04_structure_and_calculus.rs`

Critique before rewrite:

The chapter was comprehensive, but it carried many category-theory names at
once. The learner needed a compact way to distinguish the four patterns before
entering the long detailed walkthrough.

ML teaching critique:

The chain-rule section should make upstream-gradient scaling visible, because
that is the concrete mechanism behind composing local derivatives in larger
models.

Category-theory critique:

The functor section should point to executable law-shaped checks. This avoids
using "preserve identity and composition" as a slogan without nearby Rust
evidence.

Learner critique:

A reader may know `map`, `Option`, traces, and the chain rule separately but not
know why they are in one chapter. The rewrite adds one table that maps each
category word to one engineering question and one code example.

Rewrite decision:

- keep functor, naturality, monoid, and chain-rule sections tied to concrete
  code before names,
- avoid overwhelming the reader with too many category terms at once,
- connect the chapter to tests in `src/structure.rs` and `src/calculus.rs`,
- separate "recognize the pattern" from "fully formalize the theory."

Implemented as:

- a new `Four Patterns, Four Questions` orientation section,
- new `src/structure.rs` tests for functor identity/composition examples and
  `Option` absence preservation,
- a new `src/calculus.rs` test showing local gradients scale with upstream
  gradient,
- updated further-reading wording to point at Backprop as Functor,
  computational graphs, applied category theory, and programming-oriented
  category theory.

Validation:

```bash
cargo fmt --check
cargo test structure::tests --lib
cargo test calculus::tests --lib
python3 scripts/check-prose-style.py
bash scripts/check-mdbook-coverage.sh
git diff --check -- . ':!target'
```

All passed. The full validation gate should be rerun before publishing the
combined tranche.

## Pass 8: Seven Sketches Through Rust

Files:

```text
book/src/seven-sketches-rust.md
src/sketches.rs
```

Primary sources:

- [Seven Sketches in Compositionality](https://arxiv.org/abs/1803.05316)
- [Compositional Deep Learning](https://arxiv.org/abs/1907.08292)
- [Category Theory for Programming](https://arxiv.org/abs/2209.01259)
- `src/sketches.rs`
- `examples/05_seven_sketches.rs`

Critique before rewrite:

The chapter already had the right ambition: show seven applied
category-theory patterns through small Rust models. The weakness was cohesion.
The learner could read the sketches as seven interesting fragments instead of
one repeatable engineering method.

ML teaching critique:

The chapter should preserve the tiny-ML teaching habit: start from an
engineering question, make the values concrete, then name the structure. The
sketches should feel like extensions of the same typed-pipeline method, not a
detour into unrelated mathematics.

Category-theory critique:

Each sketch needs a law, boundary, or relation that can fail. Without negative
tests, compositionality can sound like a slogan instead of a contract.

Learner critique:

A reader may ask, "What am I supposed to do with seven sketches?" The rewrite
adds a common method that applies to all seven: find the engineering problem,
name the Rust values, validate construction, compose or relate the values, and
test the law or boundary.

Rewrite decision:

- add a unifying method before the sketches,
- make the chapter feel like a toolbox for recognizing structure,
- add negative tests where invalid structure should be rejected,
- connect the tests to exercise-style learner feedback.

Implemented as:

- a new `One Method Across Seven Sketches` section,
- learner-facing prose explaining negative boundary tests,
- a `src/sketches.rs` test for missing database references,
- a `src/sketches.rs` test for mismatched signal-matrix composition,
- a `src/sketches.rs` test for open-circuit boundary mismatch.

Validation:

```bash
cargo fmt --check
cargo test sketches::tests --lib
python3 scripts/check-prose-style.py
bash scripts/check-mdbook-coverage.sh
git diff --check -- . ':!target'
```

All passed. The full validation gate should be rerun before publishing the
combined tranche.

## Pass 9: Exercises

Files:

```text
book/src/exercises.md
exercises/beginner/README.md
exercises/intermediate/README.md
exercises/advanced/README.md
book/src/references.md
scripts/check-mdbook-coverage.sh
```

Primary sources:

- [How People Learn II](https://www.nationalacademies.org/projects/DBASSE-BBCSS-13-06/publication/24783)
- [The Rust Programming Language: Writing Automated Tests](https://doc.rust-lang.org/stable/book/ch11-00-testing.html)
- [Rust By Example: Tests](https://doc.rust-lang.org/rust-by-example/cargo/test.html)
- [Rust API Guidelines Checklist](https://rust-lang.github.io/api-guidelines/checklist.html)
- `exercises/beginner/README.md`
- `exercises/intermediate/README.md`
- `exercises/advanced/README.md`

Critique before rewrite:

The chapter already asked readers to explain Rust syntax, ML meaning, and
category-theory shape. The weakness was that the exercise system did not yet
make failure feedback explicit enough. A learner needed clearer guidance for
how compiler errors, constructor errors, test failures, and command-output
changes become part of the learning method.

ML teaching critique:

The exercises should keep returning to the core tiny-ML path: typed values,
adjacent training pairs, prediction, loss, and repeated updates. The reader
should practice transfer, not only recall.

Category-theory critique:

The exercises should show that structures are protected by boundaries and laws.
Asking a learner to break composition or inspect a negative test makes the
category-theory claim concrete.

Learner critique:

A reader could complete the original exercise list but still be unsure how to
know whether an answer was finished. The rewrite adds a ladder, pass
conditions, expected failure signals, debugging hints, and retrieval practice.

Rewrite decision:

- make the exercise method explicit: read, run, break, explain, restore,
- align the root exercise chapter with beginner, intermediate, and advanced
  practice packs,
- add failure-signal guidance so errors become teaching evidence,
- add source-backed testing references to the reference map and coverage guard.

Implemented as:

- a new exercise ladder table,
- a new failure-signal table,
- debugging hints and expected observations for core exercises,
- retrieval-practice prompts,
- expanded beginner, intermediate, and advanced exercise packs,
- Rust testing references in `book/src/references.md`,
- coverage checks for the new testing references.

Validation:

```bash
python3 scripts/check-prose-style.py
bash scripts/check-mdbook-coverage.sh
git diff --check -- . ':!target'
```

All passed. The coverage guard also preserves the public-facing naming and
tooling boundaries for learner content.

## Pass 10: Glossary

File:

```text
book/src/glossary.md
```

Primary sources:

- [Seven Sketches in Compositionality](https://arxiv.org/abs/1803.05316)
- [Category Theory for Programming](https://arxiv.org/abs/2209.01259)
- [Dive into Deep Learning: Softmax Regression](https://d2l.ai/chapter_linear-classification/softmax-regression.html)
- [The Rust Programming Language: Writing Automated Tests](https://doc.rust-lang.org/stable/book/ch11-00-testing.html)
- `src/domain.rs`
- `src/category.rs`
- `src/ml.rs`
- `src/training.rs`
- `src/structure.rs`
- `src/sketches.rs`

Critique before rewrite:

The glossary already followed the book's three-lens shape, but it still read
like a flat dictionary. The rewritten chapters now rely more heavily on terms
such as law, boundary, negative test, distribution, training example, and
end-to-end pipeline. Those terms needed explicit handles.

ML teaching critique:

The glossary should separate score, probability, loss, training example,
training set, and parameters so a learner can recover the tiny ML pipeline
without rereading every chapter.

Category-theory critique:

The glossary should make laws and boundaries executable. If a term names
structure, the entry should point to a type, function, method, or test that
makes the structure inspectable.

Learner critique:

A reader could look up "composition" and still not know why a failed
composition is useful. The rewrite adds first-principles readings that connect
terms to errors, tests, and validation boundaries.

Rewrite decision:

- add a "How To Use This Glossary" section,
- add missing high-use terms from the rewritten chapters,
- keep definitions anchored to repository code,
- avoid adding advanced vocabulary that the book has not yet made executable.

Implemented as:

- new entries for `Law`, `Commutative Diagram`, `Boundary`, `Negative Test`,
  `Training Example`, `Training Set`, `Distribution`, `Loss`, and
  `End-To-End Pipeline`,
- stronger first-principles readings for morphism, composition, functor,
  monoid, preorder, and parameters,
- a usage contract that treats every term as a bridge from vocabulary to code.

Validation:

```bash
python3 scripts/check-prose-style.py
bash scripts/check-mdbook-coverage.sh
mdbook test book
git diff --check -- . ':!target'
```

All passed.

## Pass 11: Transformer Roadmap

File:

```text
book/src/roadmap.md
```

Primary sources:

- [Attention Is All You Need](https://arxiv.org/abs/1706.03762)
- [Dive into Deep Learning: Attention Mechanisms and Transformers](https://d2l.ai/chapter_attention-mechanisms-and-transformers/index.html)
- [The Annotated Transformer](https://nlp.seas.harvard.edu/2018/04/03/attention.html)
- [The Illustrated Transformer](https://jalammar.github.io/illustrated-transformer/)
- `src/domain.rs`
- `src/ml.rs`
- `src/training.rs`

Critique before rewrite:

The chapter already made the most important honesty move: the current code is a
foundation course, not a full Transformer. The weakness was that the roadmap
could do more to distinguish implemented foundation from planned attention
components and to connect future work to the external source path.

ML teaching critique:

The chapter should make the bridge from current softmax to future attention
weights explicit. Both are probability-like objects, but the support differs:
vocabulary tokens for prediction and sequence positions for attention.

Category-theory critique:

Future Transformer work should preserve the book's existing discipline:
objects, typed morphisms, products, endomorphisms, and law or boundary tests.
The roadmap should not invite a large untyped implementation dump.

Learner critique:

A reader may see "Transformer" in the project name and expect full attention
code now. The rewrite adds a status table that separates implemented concepts
from planned concepts.

Rewrite decision:

- add a source path from current Rust pipeline to the original paper and
  implementation/tutorial bridges,
- add an implementation-status table,
- add design contracts for sequence objects, Q/K/V roles, attention weights,
  multi-head dimensions, residual blocks, and future parameters,
- keep planned work honest and tied to typed boundaries.

Implemented as:

- a new roadmap source path,
- a current-versus-planned concept table,
- additional design contracts for each future milestone,
- a reference-path section that tells contributors how to use external sources
  without copying full complexity into one large module.

Validation:

```bash
python3 scripts/check-prose-style.py
bash scripts/check-mdbook-coverage.sh
mdbook test book
git diff --check -- . ':!target'
```

The coverage guard passed before the list-density fix; prose style,
chapter tests, and whitespace checks passed after the fix. The full validation
gate should be rerun before publishing the combined tranche.

## Pass 12: Repository Source Snapshots

File:

```text
book/src/source-snapshots.md
```

Primary sources:

- `src/lib.rs`
- `src/domain.rs`
- `src/category.rs`
- `src/ml.rs`
- `src/training.rs`
- `src/structure.rs`
- `src/calculus.rs`
- `src/sketches.rs`
- `src/demo.rs`
- `examples/*.rs`
- `lessons/*.md`

Critique before rewrite:

The appendix already included the full learner-facing source surface. The
weakness was navigation. A reader could see the files but still not know which
chapter each file verifies or which order to use for different learning goals.

Learner critique:

The appendix needed to answer, "I am here with the code open; where do I go
next?" It also needed to explain why snapshots exist without becoming another
full tutorial.

Rewrite decision:

- keep the appendix as a lookup and audit layer,
- add a chapter-to-source navigation table,
- add reading paths for five-minute, core, structure, and contribution goals,
- preserve the source include coverage contract.

Implemented as:

- a new `How To Navigate The Snapshots` section,
- a file-to-chapter map for the main source modules,
- a `Reading Order By Goal` section,
- a drift-check question that connects source changes back to chapter truth.

Validation:

```bash
python3 scripts/check-prose-style.py
bash scripts/check-mdbook-coverage.sh
mdbook test book
git diff --check -- . ':!target'
```

All passed.

## Sweep Status

The first top-to-bottom source-backed rewrite sweep has now covered:

- Welcome
- Course Map
- Domain Objects
- Morphism and Composition
- The Tiny ML Pipeline
- Training as an Endomorphism
- Functors, Naturality, Monoids, and Chain Rule
- Seven Sketches Through Rust
- Exercises
- Glossary
- References
- Transformer Roadmap
- Repository Source Snapshots

The next sweep should start again at [Welcome](src/welcome.md), this time
looking for cross-chapter consistency, duplicated explanations, missing
diagrams, and exercise-to-test alignment.

## Sweep 2 Pass 1: Companion Lessons

Files:

```text
lessons/README.md
lessons/06-seven-sketches.md
book/src/source-snapshots.md
```

Critique before rewrite:

The compact lesson path covered the core tiny ML pipeline and structure
chapter, but it stopped before the Seven Sketches chapter. That made the
companion notes lag behind the book's expanded source-backed sweep.

Rewrite decision:

- keep compact lessons short,
- add a Seven Sketches lesson that starts from tests and boundaries,
- wire the new lesson into the source snapshot appendix,
- preserve the coverage guard that requires every lesson file to be included.

Implemented as:

- a new `lessons/06-seven-sketches.md`,
- an updated lesson index,
- a source snapshot include for the new lesson.

Validation:

```bash
python3 scripts/check-prose-style.py
bash scripts/check-mdbook-coverage.sh
mdbook test book
git diff --check -- . ':!target'
```

All passed.

## Sweep 2 Pass 2: Core Pipeline Diagrams

Files:

```text
book/src/00-map.md
book/src/02-morphisms-composition.md
book/src/03-ml-pipeline.md
book/src/04-training-endomorphism.md
README.md
ROADMAP.md
community/starter-issues.md
community/feedback-wall.md
```

Critique before rewrite:

The rewritten chapters explained the pipeline in prose and code, but the repo's
own public roadmap and starter issue list still identified diagrams as a
learner-facing gap. The first diagrams needed to reduce cognitive load without
adding a rendering dependency or floating away from code.

Rewrite decision:

- add plain text diagrams that render anywhere the book renders,
- attach each diagram to nearby prose explaining how to read it,
- prioritize the whole pipeline, legal composition, data-preparation-to-loss,
  and training update loop,
- keep diagrams tied to concrete Rust types and morphism names.

Implemented as:

- a full course pipeline diagram in `00-map.md`,
- a legal `TokenId -> Vector -> Logits -> Distribution` composition diagram in
  `02-morphisms-composition.md`,
- a data-preparation and prediction/loss diagram in `03-ml-pipeline.md`,
- a `Parameters_t -> Parameters_{t+1}` training loop diagram in
  `04-training-endomorphism.md`,
- roadmap and community wording changed from "missing diagrams" to diagram
  review and refinement.

Validation:

```bash
python3 scripts/check-prose-style.py
bash scripts/check-mdbook-coverage.sh
mdbook test book
git diff --check -- . ':!target'
```

All passed.

## Sweep 2 Pass 3: Completion Audit

File:

```text
book/TEXTBOOK_COMPLETION_AUDIT.md
```

Decision:

Add an explicit prompt-to-artifact audit before treating the current green state
as anything close to finished. The audit maps the active textbook objective to
real files, validation evidence, and remaining gaps.

Result:

The audit records that the first sweep and early second-sweep improvements are
coherent and validated, but the textbook goal is not complete. Remaining work
includes exercise answer keys or facilitator notes, advanced structure
diagrams, terminology consistency passes, future attention examples, and
external reader feedback.

## Sweep 2 Pass 4: Exercise Answer Key

Files:

```text
exercises/ANSWER_KEY.md
exercises/README.md
book/src/exercises.md
book/TEXTBOOK_COMPLETION_AUDIT.md
```

Critique before rewrite:

The exercise prompts had pass conditions and debugging hints, but practice was
still harder to review consistently because there was no centralized expected
reasoning guide.

Rewrite decision:

- add a public answer key and facilitator notes file,
- focus on reasoning shape rather than exact wording,
- make the root `exercises/` directory navigable,
- link the book exercise chapter to the answer key after readers attempt the
  exercises.

Validation:

```bash
python3 scripts/check-prose-style.py
bash scripts/check-mdbook-coverage.sh
mdbook test book
git diff --check -- . ':!target'
```

All passed.

## Sweep 2 Pass 5: Advanced Law And Boundary Tables

Files:

```text
book/src/05-structure-and-calculus.md
book/src/seven-sketches-rust.md
book/TEXTBOOK_COMPLETION_AUDIT.md
```

Critique before rewrite:

The advanced chapters were detailed, but a learner still had to hold many
abstract terms in memory at once. The next improvement was to add compact
visual indexes that connect every advanced term to a law, boundary, or test.

Rewrite decision:

- add a law/boundary table to the structure chapter,
- add a protected-structure table to the Seven Sketches chapter,
- update the completion audit so this gap is no longer listed as missing.

Validation:

```bash
python3 scripts/check-prose-style.py
bash scripts/check-mdbook-coverage.sh
mdbook test book
git diff --check -- . ':!target'
```

All passed.

## Sweep 2 Pass 6: Core Chapter Practice Alignment

Files:

```text
book/src/welcome.md
book/src/00-map.md
book/src/01-domain-objects.md
book/src/02-morphisms-composition.md
book/src/03-ml-pipeline.md
book/src/04-training-endomorphism.md
book/src/exercises.md
book/TEXTBOOK_COMPLETION_AUDIT.md
```

Critique before rewrite:

The early chapters had retrieval practice, but the connection from chapter
outcomes to the longer exercise set was mostly implicit. A learner could finish
a chapter and still wonder which exercise was the right next check.

Rewrite decision:

- add a chapter-to-exercise practice map in `exercises.md`,
- add short `Practice After This Chapter` sections from Welcome through
  Training,
- keep the sections small and tied to the exact chapter outcome,
- update the completion audit to mark core chapter exercise alignment present.

Validation:

```bash
python3 scripts/check-prose-style.py
bash scripts/check-mdbook-coverage.sh
mdbook test book
git diff --check -- . ':!target'
```

All passed.

## Sweep 2 Pass 7: Core Terminology And Duplicate-Prose Audit

Files:

```text
book/src/glossary.md
book/STYLE_GUIDE.md
book/TEXTBOOK_COMPLETION_AUDIT.md
```

Critique before rewrite:

The core chapters use some ideas in three registers: public phrase, Rust type,
and category-theory term. Without a contract, terms like "training pairs,"
`TrainingSet`, "model state," `Parameters`, "typed transformation," and
`Morphism` can drift.

Rewrite decision:

- add a learner-facing `Core Term Alignment` table to the glossary,
- add an editorial `Terminology Contract` to the style guide,
- run a duplicate-line audit over Welcome through Training,
- update the completion audit after confirming that repeated lines were code
  snapshots, diagrams, or deliberate signatures rather than prose needing
  rewrite.

Validation:

```bash
python3 scripts/check-prose-style.py
bash scripts/check-mdbook-coverage.sh
mdbook test book
git diff --check -- . ':!target'
```

All passed.

## Sweep 2 Pass 8: Typed Attention Roadmap Boundary

Files:

```text
src/attention.rs
examples/06_attention_scores.rs
src/lib.rs
scripts/check.sh
book/src/roadmap.md
book/src/source-snapshots.md
book/src/glossary.md
book/src/references.md
README.md
book/TEXTBOOK_COMPLETION_AUDIT.md
```

Primary sources:

- [Attention Is All You Need](https://arxiv.org/abs/1706.03762)
- [Dive into Deep Learning: Attention Mechanisms and Transformers](https://d2l.ai/chapter_attention-mechanisms-and-transformers/index.html)
- `src/attention.rs`
- `examples/06_attention_scores.rs`

Critique before rewrite:

The Transformer roadmap correctly said that attention work should not be
claimed until the code had typed examples. The next coherent step was not a
full Transformer block. It was the smallest attention-specific boundary that
the current softmax chapter already prepares: score rows becoming probability
rows.

Rewrite decision:

- add a tiny `attention` module rather than overbuilding a Transformer,
- model `SequenceLength`, `HeadDimension`, `AttentionScores`,
  `AttentionWeights`, and `AttentionSoftmax`,
- add tests for row-wise normalization and rejected invalid shapes,
- add a runnable `06_attention_scores` example,
- update the full validation gate so it runs the new example,
- wire the new code into the roadmap, source snapshots, glossary, references,
  and README examples table.

Validation:

```bash
cargo fmt --check
cargo test attention::tests --lib
cargo run --example 06_attention_scores
python3 scripts/check-prose-style.py
bash scripts/check-mdbook-coverage.sh
mdbook test book
git diff --check -- . ':!target'
```

All passed.

## Sweep 2 Pass 9: Public Reader Review Loop

Files:

```text
community/reader-review-guide.md
community/starter-issues.md
community/feedback-wall.md
README.md
book/TEXTBOOK_COMPLETION_AUDIT.md
```

Critique before rewrite:

The project had starter issues and a feedback wall, but the route from a
first-time reader's confusion to a usable issue was still too implicit. The
textbook objective requires external learner friction, not only internal
critique passes.

Rewrite decision:

- add a public thirty-minute reader review path,
- make useful feedback concrete through chapter, section, command, confusion,
  expectation, and suggested-fix fields,
- add diagram and attention-roadmap review prompts,
- link the review guide from the README's path table and contribution section,
- update the completion audit so the review mechanism is present while actual
  external reports remain pending.

Validation:

```bash
python3 scripts/check-prose-style.py
bash scripts/check-mdbook-coverage.sh
git diff --check -- . ':!target'
bash scripts/check.sh
```

All passed.

## Sweep 2 Pass 10: Query-Key Attention Boundary

Files:

```text
src/attention.rs
src/lib.rs
examples/06_attention_scores.rs
book/src/roadmap.md
book/src/glossary.md
book/src/references.md
book/src/source-snapshots.md
README.md
ROADMAP.md
community/reader-review-guide.md
community/starter-issues.md
community/feedback-wall.md
book/TEXTBOOK_COMPLETION_AUDIT.md
```

Primary sources:

- [Attention Is All You Need](https://arxiv.org/abs/1706.03762)
- [Dive into Deep Learning: Attention Mechanisms and Transformers](https://d2l.ai/chapter_attention-mechanisms-and-transformers/index.html)
- [The Annotated Transformer](https://nlp.seas.harvard.edu/2018/04/03/attention.html)

Critique before rewrite:

The attention roadmap had one executable boundary:

```text
AttentionScores -> AttentionWeights
```

That was a useful bridge from softmax, but it skipped the first attention
question a learner expects from the Transformer references: where do the scores
come from? The smallest honest next step was query-key scoring, not value
mixing or a full Transformer block.

Rewrite decision:

- add `QuerySequence` and `KeySequence` as separate role-specific objects,
- validate non-empty sequence length, non-empty head dimension, rectangular
  rows, and finite vector values,
- add `ScaledDotProductScores` as
  `QuerySequence x KeySequence -> AttentionScores`,
- keep value mixing, masks, and multi-head attention explicitly planned,
- update the runnable attention example so the command demonstrates the
  two-step path from query/key roles to attention weights,
- update roadmap, glossary, references, source snapshots, README, community
  review prompts, and the completion audit so public docs match the code.

Validation:

```bash
cargo fmt --check
cargo test attention::tests --lib
cargo run --example 06_attention_scores
python3 scripts/check-prose-style.py
bash scripts/check-mdbook-coverage.sh
git diff --check -- . ':!target'
bash scripts/check.sh
```

All passed. The full gate reported 38 Rust tests, all examples, the main
demo, prose style, source coverage, book build, and chapter tests.

## Sweep 2 Pass 11: Value-Mixing Attention Boundary

Files:

```text
src/attention.rs
src/lib.rs
examples/06_attention_scores.rs
book/src/roadmap.md
book/src/glossary.md
book/src/references.md
book/src/source-snapshots.md
scripts/check-mdbook-coverage.sh
README.md
ROADMAP.md
community/reader-review-guide.md
community/starter-issues.md
community/feedback-wall.md
book/TEXTBOOK_COMPLETION_AUDIT.md
```

Primary sources:

- [Attention Is All You Need](https://arxiv.org/abs/1706.03762)
- [Dive into Deep Learning: Queries, Keys, and Values](https://d2l.ai/chapter_attention-mechanisms-and-transformers/queries-keys-values.html)
- [Dive into Deep Learning: Attention Scoring Functions](https://d2l.ai/chapter_attention-mechanisms-and-transformers/attention-scoring-functions.html)
- [The Annotated Transformer](https://nlp.seas.harvard.edu/2018/04/03/attention.html)

Critique before rewrite:

The query-key pass made score construction explicit, but the learner still
could not see the final step of scaled dot-product attention:

```text
softmax(QK^T / sqrt(d)) V
```

The missing boundary was not masks or multi-head attention. It was the smaller
value-mixing step: attention weights must have the same key/source length as
the value sequence they mix.

Rewrite decision:

- add `ValueSequence` and `AttentionOutput`,
- add `WeightedValueMixing` as
  `AttentionWeights x ValueSequence -> AttentionOutput`,
- reject value-length mismatches before mixing,
- update the attention example so it prints output vectors after weights,
- update roadmap, glossary, reference map, community prompts, README, and audit
  language so masks and multi-head attention remain planned but value mixing is
  implemented.

Validation:

```bash
cargo fmt --check
cargo test attention::tests --lib
cargo run --example 06_attention_scores
python3 scripts/check-prose-style.py
bash scripts/check-mdbook-coverage.sh
git diff --check -- . ':!target'
bash scripts/check.sh
```

All passed. The full gate reported 41 Rust tests, all examples, the main
demo, prose style, source coverage, book build, and chapter tests.

## Sweep 2 Pass 12: Attention Mask Boundary

Files:

```text
src/attention.rs
src/lib.rs
examples/06_attention_scores.rs
book/src/roadmap.md
book/src/glossary.md
book/src/references.md
book/src/source-snapshots.md
README.md
ROADMAP.md
community/reader-review-guide.md
community/starter-issues.md
community/feedback-wall.md
book/TEXTBOOK_COMPLETION_AUDIT.md
```

Primary sources:

- [Attention Is All You Need](https://arxiv.org/abs/1706.03762)
- [Dive into Deep Learning: Attention Scoring Functions](https://d2l.ai/chapter_attention-mechanisms-and-transformers/attention-scoring-functions.html)
- [The Annotated Transformer](https://nlp.seas.harvard.edu/2018/04/03/attention.html)

Critique before rewrite:

The value-mixing pass completed the unmasked single-head path:

```text
QuerySequence x KeySequence -> AttentionScores -> AttentionWeights
AttentionWeights x ValueSequence -> AttentionOutput
```

But the Transformer references use masks before softmax to block illegal
positions. Without a typed mask boundary, the roadmap still had an important
attention concept listed as planned.

Rewrite decision:

- add `AttentionMask` with rectangular shape validation,
- reject mask rows that allow no key positions,
- add `MaskedAttentionScores` as
  `AttentionScores x AttentionMask -> AttentionScores`,
- use a large negative finite score for disallowed positions before softmax,
- update the attention example so it shows a masked score path,
- update roadmap, glossary, reference map, source snapshots, README, community
  prompts, and the completion audit so multi-head attention remains planned
  but masking is implemented.

Validation:

```bash
cargo fmt --check
cargo test attention::tests --lib
cargo run --example 06_attention_scores
python3 scripts/check-prose-style.py
bash scripts/check-mdbook-coverage.sh
git diff --check -- . ':!target'
bash scripts/check.sh
```

All passed. The full gate reported 45 Rust tests, all examples, the main
demo, prose style, source coverage, book build, and chapter tests.

## Sweep 2 Pass 13: Multi-Head Concatenation Boundary

Files:

```text
src/attention.rs
src/lib.rs
examples/06_attention_scores.rs
book/src/roadmap.md
book/src/glossary.md
book/src/references.md
book/src/source-snapshots.md
README.md
ROADMAP.md
community/reader-review-guide.md
community/starter-issues.md
community/feedback-wall.md
book/TEXTBOOK_COMPLETION_AUDIT.md
scripts/check-mdbook-coverage.sh
```

Primary sources:

- [Attention Is All You Need](https://arxiv.org/abs/1706.03762)
- [Dive into Deep Learning: Multi-Head Attention](https://d2l.ai/chapter_attention-mechanisms-and-transformers/multihead-attention.html)
- [The Annotated Transformer](https://nlp.seas.harvard.edu/2018/04/03/attention.html)

Critique before rewrite:

The mask pass made attention legally selective before softmax, but the roadmap
still treated every multi-head idea as future work. That was no longer the
smallest useful next boundary. The book could teach the recombination step
without pretending to implement per-head projections or a full Transformer
block.

Rewrite decision:

- add `HeadCount`, `AttentionHeadOutputs`, `ConcatenateHeads`, and
  `MultiHeadOutput`,
- reject zero heads, empty head collections, sequence-length mismatches, and
  head-dimension mismatches,
- compute the combined model dimension from head count and head dimension,
- update the attention example so it prints the concatenated multi-head rows,
- update roadmap, glossary, reference map, source snapshots, README, community
  prompts, and the completion audit so output projection and full Transformer
  blocks remain planned.

Validation:

```bash
cargo fmt --check
cargo test attention::tests --lib
cargo run --example 06_attention_scores
python3 scripts/check-prose-style.py
bash scripts/check-mdbook-coverage.sh
git diff --check -- . ':!target'
bash scripts/check.sh
```

All passed. The full gate reported 48 Rust tests, all examples, the main
demo, prose style, source coverage, book build, and chapter tests.

## Sweep 2 Pass 14: Attention Output Projection Boundary

Files:

```text
src/attention.rs
src/lib.rs
examples/06_attention_scores.rs
book/src/roadmap.md
book/src/glossary.md
book/src/references.md
book/src/source-snapshots.md
README.md
ROADMAP.md
community/reader-review-guide.md
community/starter-issues.md
community/feedback-wall.md
exercises/advanced/README.md
exercises/ANSWER_KEY.md
book/TEXTBOOK_COMPLETION_AUDIT.md
```

Primary sources:

- [Attention Is All You Need](https://arxiv.org/abs/1706.03762)
- [Dive into Deep Learning: Multi-Head Attention](https://d2l.ai/chapter_attention-mechanisms-and-transformers/multihead-attention.html)
- [The Annotated Transformer](https://nlp.seas.harvard.edu/2018/04/03/attention.html)

Critique before rewrite:

The previous pass made head concatenation explicit:

```text
AttentionHeadOutputs -> MultiHeadOutput
```

That still left one source-backed multi-head step described as future work:
the linear output projection after concatenation. Leaving that boundary out
made the roadmap less precise than the D2L and Transformer references, where
concatenated heads are followed by another learned projection.

Rewrite decision:

- add `ProjectedAttentionOutput`,
- add `AttentionOutputProjection` as
  `MultiHeadOutput -> ProjectedAttentionOutput`,
- validate non-empty projection weights, non-empty bias, finite values,
  output-column shape, and input-width compatibility,
- update the attention example so it prints projected attention rows,
- update roadmap, glossary, reference map, source snapshots, README, community
  prompts, exercises, and the completion audit so residual structure and full
  Transformer blocks remain planned.

Validation:

```bash
cargo fmt --check
cargo test attention::tests --lib
cargo run --example 06_attention_scores
python3 scripts/check-prose-style.py
bash scripts/check-mdbook-coverage.sh
git diff --check -- . ':!target'
bash scripts/check.sh
```

All passed. The full gate reported 52 Rust tests, all examples, the main
demo, prose style, source coverage, book build, and chapter tests.

## Sweep 2 Pass 15: Residual Addition Boundary

Files:

```text
src/attention.rs
src/lib.rs
examples/06_attention_scores.rs
book/src/roadmap.md
book/src/glossary.md
book/src/references.md
book/src/source-snapshots.md
README.md
ROADMAP.md
community/reader-review-guide.md
community/starter-issues.md
community/feedback-wall.md
exercises/advanced/README.md
exercises/ANSWER_KEY.md
book/TEXTBOOK_COMPLETION_AUDIT.md
scripts/check-mdbook-coverage.sh
```

Primary sources:

- [Dive into Deep Learning: Transformer Architecture](https://d2l.ai/chapter_attention-mechanisms-and-transformers/transformer.html)
- [The Annotated Transformer](https://nlp.seas.harvard.edu/2018/04/03/attention.html)
- [Attention Is All You Need](https://arxiv.org/abs/1706.03762)

Critique before rewrite:

The output-projection pass returned the concatenated heads to a coherent
post-attention width:

```text
MultiHeadOutput -> ProjectedAttentionOutput
```

The next source-backed Transformer shape was residual addition. Both D2L and
The Annotated Transformer emphasize that residual connections require the
sublayer output to have the same shape as the input sequence. The book should
turn that requirement into a typed boundary, not leave it as prose.

Rewrite decision:

- add `HiddenSequence`,
- add `ResidualConnection` as
  `HiddenSequence x ProjectedAttentionOutput -> HiddenSequence`,
- reject sequence-length mismatches and model-dimension mismatches,
- update the attention example so it prints residual rows,
- update roadmap, glossary, reference map, source snapshots, README, community
  prompts, exercises, and the completion audit so normalization and full
  Transformer blocks remain planned.

Validation:

```bash
cargo fmt --check
cargo test attention::tests --lib
cargo run --example 06_attention_scores
python3 scripts/check-prose-style.py
bash scripts/check-mdbook-coverage.sh
git diff --check -- . ':!target'
bash scripts/check.sh
```

All passed. The full gate reported 55 Rust tests, all examples, the main
demo, prose style, source coverage, book build, and chapter tests.

## Sweep 2 Pass 16: Layer Normalization Boundary

Files:

```text
src/attention.rs
src/lib.rs
examples/06_attention_scores.rs
book/src/roadmap.md
book/src/glossary.md
book/src/references.md
book/src/source-snapshots.md
README.md
ROADMAP.md
community/reader-review-guide.md
community/starter-issues.md
community/feedback-wall.md
exercises/advanced/README.md
exercises/ANSWER_KEY.md
book/TEXTBOOK_COMPLETION_AUDIT.md
```

Primary sources:

- [Dive into Deep Learning: Transformer Architecture](https://d2l.ai/chapter_attention-mechanisms-and-transformers/transformer.html)
- [The Annotated Transformer](https://nlp.seas.harvard.edu/2018/04/03/attention.html)

Critique before rewrite:

The residual pass made the add step explicit:

```text
HiddenSequence x ProjectedAttentionOutput -> HiddenSequence
```

But the source path describes "add and norm" together. If the book stops after
residual addition, a reader sees why shapes must match but not why the result
still needs a normalization boundary that preserves the hidden sequence object.

Rewrite decision:

- add `NormalizationEpsilon`,
- add `LayerNormParameters`,
- add `LayerNormalization` as `HiddenSequence -> HiddenSequence`,
- validate scale/shift dimensions, finite parameters, positive finite epsilon,
  and hidden-sequence width compatibility,
- update the attention example so it prints normalized rows,
- update roadmap, glossary, reference map, source snapshots, README, community
  prompts, exercises, and the completion audit so the remaining attention work
  stayed explicit.

Validation:

```bash
cargo fmt --check
cargo test attention::tests --lib
cargo run --example 06_attention_scores
python3 scripts/check-prose-style.py
bash scripts/check-mdbook-coverage.sh
git diff --check -- . ':!target'
bash scripts/check.sh
```

All passed. The full gate reported 59 Rust tests, all examples, the main
demo, prose style, source coverage, book build, and chapter tests.

## Sweep 2 Pass 17: Position-Wise Feed-Forward Boundary

Files:

```text
src/attention.rs
src/lib.rs
examples/06_attention_scores.rs
book/src/roadmap.md
book/src/glossary.md
book/src/references.md
book/src/source-snapshots.md
README.md
ROADMAP.md
community/reader-review-guide.md
community/starter-issues.md
community/feedback-wall.md
exercises/advanced/README.md
exercises/ANSWER_KEY.md
book/TEXTBOOK_COMPLETION_AUDIT.md
```

Primary sources:

- [Attention Is All You Need](https://arxiv.org/abs/1706.03762)
- [Dive into Deep Learning: Transformer Architecture](https://d2l.ai/chapter_attention-mechanisms-and-transformers/transformer.html)
- [The Annotated Transformer](https://nlp.seas.harvard.edu/2018/04/03/attention.html)

Critique before rewrite:

The roadmap had attention output projection, residual addition, and layer
normalization as executable boundaries. It still described feed-forward
structure as planned, even though this is the next small shape-preserving
sublayer before a complete block. A learner could see "add and norm" but not
the second half of the standard Transformer encoder block shape.

Rewrite decision:

- add `PositionWiseFeedForward` as `HiddenSequence -> HiddenSequence`,
- validate both linear layers, finite weights and biases, compatible internal
  feed-forward width, and return to the input model dimension,
- apply a tiny ReLU between the two linear projections,
- update the attention example so it prints feed-forward rows,
- update roadmap, glossary, reference map, source snapshots, README, community
  prompts, exercises, and the completion audit so only full block integration
  remains planned.

Validation:

```bash
cargo fmt --check
cargo test attention::tests --lib
cargo run --example 06_attention_scores
python3 scripts/check-prose-style.py
bash scripts/check-mdbook-coverage.sh
git diff --check -- . ':!target'
bash scripts/check.sh
```

All passed. The full gate reported 63 Rust tests, all examples, the main
demo, prose style, source coverage, book build, and chapter tests.

## Sweep 2 Pass 18: Single-Head Block Boundary

Files:

```text
src/attention.rs
src/lib.rs
examples/06_attention_scores.rs
book/src/roadmap.md
book/src/glossary.md
book/src/references.md
book/src/source-snapshots.md
README.md
ROADMAP.md
community/reader-review-guide.md
community/starter-issues.md
community/feedback-wall.md
exercises/advanced/README.md
exercises/ANSWER_KEY.md
book/TEXTBOOK_COMPLETION_AUDIT.md
```

Primary sources:

- [Attention Is All You Need](https://arxiv.org/abs/1706.03762)
- [Dive into Deep Learning: Transformer Architecture](https://d2l.ai/chapter_attention-mechanisms-and-transformers/transformer.html)
- [The Annotated Transformer](https://nlp.seas.harvard.edu/2018/04/03/attention.html)

Critique before rewrite:

The feed-forward pass made every major sublayer boundary executable, but the
roadmap still relied on prose to say that those pieces compose into a
sequence-preserving block. A learner could run the sublayers individually
without seeing the compact block shape that later stacking depends on.

Rewrite decision:

- add `HiddenToQuery`, `HiddenToKey`, and `HiddenToValue`,
- add `SingleHeadTransformerBlock : HiddenSequence -> HiddenSequence`,
- validate projection input widths, query/key head compatibility, value width
  against output-projection input width, normalization dimensions, and
  feed-forward dimensions,
- add a hidden-sequence residual form for the second residual connection,
- update the attention example so it prints the single-head block shape,
- update roadmap, glossary, reference map, source snapshots, README, community
  prompts, exercises, and the completion audit so multi-head block integration
  remains future work.

Validation:

```bash
cargo fmt --check
cargo test attention::tests --lib
cargo run --example 06_attention_scores
python3 scripts/check-prose-style.py
bash scripts/check-mdbook-coverage.sh
git diff --check -- . ':!target'
bash scripts/check.sh
```

All passed. The full gate reported 69 Rust tests, all examples, the main
demo, prose style, source coverage, book build, and chapter tests.

## Sweep 2 Pass 19: Multi-Head Block Boundary

Files:

```text
src/attention.rs
src/lib.rs
examples/06_attention_scores.rs
book/src/roadmap.md
book/src/glossary.md
book/src/references.md
book/src/source-snapshots.md
README.md
ROADMAP.md
community/reader-review-guide.md
community/starter-issues.md
community/feedback-wall.md
exercises/advanced/README.md
exercises/ANSWER_KEY.md
book/TEXTBOOK_COMPLETION_AUDIT.md
```

Primary sources:

- [Attention Is All You Need](https://arxiv.org/abs/1706.03762)
- [Dive into Deep Learning: Multi-Head Attention](https://d2l.ai/chapter_attention-mechanisms-and-transformers/multihead-attention.html)
- [Dive into Deep Learning: Transformer Architecture](https://d2l.ai/chapter_attention-mechanisms-and-transformers/transformer.html)

Critique before rewrite:

The single-head block proved the endomorphism-shaped block boundary, but the
roadmap still left the multi-head version as future work. Since the source path
explicitly uses several attention heads, concatenation, and output projection,
the next coherent step was to make that full tiny boundary executable.

Rewrite decision:

- add `SelfAttentionHead` as a validated query/key/value projection triple,
- add `MultiHeadTransformerBlock : HiddenSequence -> HiddenSequence`,
- validate shared hidden input width, query/key head compatibility per head,
  value head width across heads, output-projection input width equal to
  `head_count * value_head_dimension`, and normalization/feed-forward widths,
- update the attention example so it prints the multi-head block shape,
- update roadmap, glossary, reference map, source snapshots, README, community
  prompts, exercises, and the completion audit so positional encoding, masked
  block variants, and structured Transformer training state are the next gaps.

Validation:

```bash
cargo fmt --check
cargo test attention::tests --lib
cargo run --example 06_attention_scores
python3 scripts/check-prose-style.py
bash scripts/check-mdbook-coverage.sh
git diff --check -- . ':!target'
bash scripts/check.sh
```

All passed. The full gate reported 74 Rust tests, all examples, the main
demo, prose style, source coverage, book build, and chapter tests.

## Sweep 2 Pass 20: Positional Encoding Boundary

Files:

```text
src/attention.rs
src/lib.rs
examples/06_attention_scores.rs
book/src/roadmap.md
book/src/glossary.md
book/src/references.md
book/src/source-snapshots.md
README.md
ROADMAP.md
community/reader-review-guide.md
community/starter-issues.md
community/feedback-wall.md
exercises/advanced/README.md
exercises/ANSWER_KEY.md
book/TEXTBOOK_COMPLETION_AUDIT.md
```

Primary sources:

- [Attention Is All You Need](https://arxiv.org/abs/1706.03762)
- [Dive into Deep Learning: Self-Attention and Positional Encoding](https://d2l.ai/chapter_attention-mechanisms-and-transformers/self-attention-and-positional-encoding.html)
- [Dive into Deep Learning: Transformer Architecture](https://d2l.ai/chapter_attention-mechanisms-and-transformers/transformer.html)

Critique before rewrite:

The multi-head block pass made the sequence block executable, but the roadmap
still said positional information was planned. That left a conceptual gap: the
block could process sequences, but the tiny code had no typed boundary for
adding position before attention.

Rewrite decision:

- add `PositionalEncoding : HiddenSequence -> HiddenSequence`,
- validate finite encoding rows, maximum supported sequence length, and model
  dimension compatibility,
- update the attention example so the block path starts from positioned hidden
  rows,
- update roadmap, glossary, reference map, source snapshots, README, community
  prompts, exercises, and the completion audit so masked block variants and
  structured Transformer training state are the next gaps.

Validation:

```bash
cargo fmt --check
cargo test attention::tests --lib
cargo run --example 06_attention_scores
python3 scripts/check-prose-style.py
bash scripts/check-mdbook-coverage.sh
git diff --check -- . ':!target'
bash scripts/check.sh
```

All passed. The full gate reported 77 Rust tests, all examples, the main
demo, prose style, source coverage, book build, and chapter tests.

## Sweep 2 Pass 21: Masked Multi-Head Block Boundary

Files:

```text
src/attention.rs
src/lib.rs
examples/06_attention_scores.rs
book/src/roadmap.md
book/src/glossary.md
book/src/references.md
book/src/source-snapshots.md
README.md
ROADMAP.md
community/reader-review-guide.md
community/starter-issues.md
community/feedback-wall.md
exercises/advanced/README.md
exercises/ANSWER_KEY.md
book/TEXTBOOK_COMPLETION_AUDIT.md
```

Primary sources:

- [Attention Is All You Need](https://arxiv.org/abs/1706.03762)
- [Dive into Deep Learning: Attention Scoring Functions](https://d2l.ai/chapter_attention-mechanisms-and-transformers/attention-scoring-functions.html)
- [Dive into Deep Learning: Transformer Architecture](https://d2l.ai/chapter_attention-mechanisms-and-transformers/transformer.html)

Critique before rewrite:

The lower-level mask boundary was executable, but the block-level API still
only exposed unmasked self-attention. That made masks look like a detached
operation rather than part of a real block input.

Rewrite decision:

- add `MaskedMultiHeadTransformerBlock` as
  `HiddenSequence x AttentionMask -> HiddenSequence`,
- reuse the validated multi-head block constructor contract,
- apply the mask after query-key scoring and before softmax for every head,
- add tests for shape preservation and mask shape rejection,
- update the attention example and public learning materials so structured
  Transformer training state is now the next technical gap.

Validation:

```bash
cargo fmt --check
cargo test attention::tests --lib
cargo run --example 06_attention_scores
python3 scripts/check-prose-style.py
bash scripts/check-mdbook-coverage.sh
git diff --check -- . ':!target'
bash scripts/check.sh
```

All passed. The full gate reported 79 Rust tests, all examples, the main
demo, prose style, source coverage, book build, and chapter tests.

## Sweep 2 Pass 22: Structured Transformer State Boundary

Files:

```text
src/attention.rs
src/lib.rs
examples/06_attention_scores.rs
book/src/roadmap.md
book/src/glossary.md
book/src/references.md
book/src/source-snapshots.md
README.md
ROADMAP.md
community/reader-review-guide.md
community/starter-issues.md
community/feedback-wall.md
exercises/advanced/README.md
exercises/ANSWER_KEY.md
book/EDITORIAL_RESEARCH_NOTES.md
book/TEXTBOOK_COMPLETION_AUDIT.md
scripts/check-mdbook-coverage.sh
```

Primary sources:

- [Dive into Deep Learning: Transformer Architecture](https://d2l.ai/chapter_attention-mechanisms-and-transformers/transformer.html)
- [Dive into Deep Learning: Parameter Management](https://d2l.ai/chapter_builders-guide/parameters.html)
- [The Annotated Transformer](https://nlp.seas.harvard.edu/2018/04/03/attention.html)

Critique before rewrite:

The masked block pass made the sequence block boundary executable, but the
roadmap still had no single object that owned position, block, readout, and
optimizer metadata. That left the future training path too loose: readers could
see the block, but not the model state that a future optimizer would update.

Rewrite decision:

- add `SequenceLogits` for vocabulary scores at each sequence position,
- add `TransformerReadout : HiddenSequence -> SequenceLogits`,
- add `TinyTransformerParameters : HiddenSequence x AttentionMask ->
  SequenceLogits`,
- add `TransformerTrainingState` to own structured parameters, learning rate,
  and step count,
- make the example print the structured logits shape and step transition,
- update roadmap, glossary, reference map, source snapshots, README,
  community prompts, exercises, and the completion audit so the remaining gap
  is full gradient update over structured state, not state structure itself.

Validation:

```bash
cargo fmt --check
cargo test attention::tests --lib
cargo run --example 06_attention_scores
python3 scripts/check-prose-style.py
bash scripts/check-mdbook-coverage.sh
git diff --check -- . ':!target'
bash scripts/check.sh
```

All passed. The full gate reported 84 Rust tests, all examples, the main
demo, prose style, source coverage, book build, and chapter tests.

## Sweep 2 Pass 23: Readout-Only Training Endomorphism

Files:

```text
src/attention.rs
src/lib.rs
examples/06_attention_scores.rs
book/src/roadmap.md
book/src/glossary.md
book/src/references.md
book/src/source-snapshots.md
README.md
ROADMAP.md
community/reader-review-guide.md
community/starter-issues.md
community/feedback-wall.md
exercises/advanced/README.md
exercises/ANSWER_KEY.md
book/EDITORIAL_RESEARCH_NOTES.md
book/TEXTBOOK_COMPLETION_AUDIT.md
```

Primary sources:

- [Dive into Deep Learning: Softmax Regression Implementation from Scratch](https://d2l.ai/chapter_linear-classification/softmax-regression-scratch.html)
- [Dive into Deep Learning: Gradient Descent](https://d2l.ai/chapter_optimization/gd.html)
- [Dive into Deep Learning: Transformer Architecture](https://d2l.ai/chapter_attention-mechanisms-and-transformers/transformer.html)

Critique before rewrite:

The structured state pass created a parameter object and step counter, but the
only way to advance the state was to manually record replacement parameters.
That made the endomorphism story weaker than the earlier `TrainStep` chapter.
The next teaching boundary needed a real update while still avoiding a false
claim that full attention backpropagation exists.

Rewrite decision:

- add `TransformerReadoutTrainingExample` and
  `TransformerReadoutTrainingSet`,
- add `TransformerReadoutTrainStep : TransformerTrainingState ->
  TransformerTrainingState`,
- add `transformer_readout_average_loss`,
- update only the sequence readout weights and bias with a softmax
  cross-entropy gradient,
- keep position, attention block, normalization, and feed-forward parameters
  fixed,
- update the attention example to show loss decreasing after one readout-only
  update,
- update the public docs and exercises so the remaining gap is full
  block-gradient work.

Validation:

```bash
cargo fmt --check
cargo test attention::tests --lib
cargo run --example 06_attention_scores
```

The targeted Rust path passed with 57 attention tests, and the example showed
readout loss decreasing from `0.499085` to `0.456495` after one update. The
full gate reported 88 Rust tests, all examples, the main demo, prose style,
source coverage, book build, and chapter tests.

## Sweep 2 Pass 24: Local Feed-Forward Training Endomorphism

Files:

```text
src/attention.rs
src/lib.rs
examples/06_attention_scores.rs
book/src/roadmap.md
book/src/glossary.md
book/src/source-snapshots.md
README.md
ROADMAP.md
community/reader-review-guide.md
community/starter-issues.md
community/feedback-wall.md
exercises/advanced/README.md
exercises/ANSWER_KEY.md
book/TEXTBOOK_COMPLETION_AUDIT.md
```

Primary sources:

- [Dive into Deep Learning: Backpropagation and Computational Graphs](https://d2l.ai/chapter_multilayer-perceptrons/backprop.html)
- [Dive into Deep Learning: Gradient Descent](https://d2l.ai/chapter_optimization/gd.html)
- [Dive into Deep Learning: Transformer Architecture](https://d2l.ai/chapter_attention-mechanisms-and-transformers/transformer.html)

Critique before rewrite:

The readout-only update was a real state endomorphism, but it did not touch
any internal block component. The next useful teaching boundary was a local
feed-forward update: one layer deeper than the readout, while still avoiding
the false claim that attention, residual, and normalization gradients are
complete.

Rewrite decision:

- add `TransformerFeedForwardTrainingExample` and
  `TransformerFeedForwardTrainingSet`,
- add `TransformerFeedForwardTrainStep : TransformerTrainingState ->
  TransformerTrainingState`,
- add `transformer_feed_forward_average_loss`,
- update the position-wise feed-forward first and second linear layers with a
  local squared-error gradient through the ReLU,
- rebuild the structured state through existing constructors,
- update the example to show local feed-forward loss decreasing,
- update public docs and exercises so the remaining gap is end-to-end
  block-gradient work.

Validation:

```bash
cargo fmt --check
cargo test attention::tests --lib
cargo run --example 06_attention_scores
```

The targeted Rust path passed with 60 attention tests, and the example showed
local feed-forward loss decreasing from `0.250000` to `0.160633` after one
update. The full publication gate passed with 91 Rust tests, all examples,
the main demo, prose style checks, source coverage checks, book build, and
chapter tests.

## Sweep 2 Pass 25: Composed Block Training Endomorphism

Files:

```text
src/attention.rs
src/lib.rs
examples/06_attention_scores.rs
book/src/roadmap.md
book/src/glossary.md
book/src/references.md
book/src/source-snapshots.md
README.md
ROADMAP.md
community/reader-review-guide.md
community/starter-issues.md
community/feedback-wall.md
exercises/advanced/README.md
exercises/ANSWER_KEY.md
book/TEXTBOOK_COMPLETION_AUDIT.md
```

Primary sources:

- [Attention Is All You Need](https://arxiv.org/abs/1706.03762)
- [Layer Normalization](https://arxiv.org/abs/1607.06450)
- [Dive into Deep Learning: Backpropagation and Computational Graphs](https://d2l.ai/chapter_multilayer-perceptrons/backprop.html)
- [Dive into Deep Learning: Transformer Architecture](https://d2l.ai/chapter_attention-mechanisms-and-transformers/transformer.html)
- [Stanford CS231n: Linear Classification](https://cs231n.github.io/linear-classify/)

Critique before rewrite:

The readout-only and local feed-forward updates were both real, but they taught
two separate local stories. A learner could still miss the more important
training idea: a token-level loss can move backward through the readout, a
normalization boundary, a residual connection, and an internal sublayer while
the whole update remains a state endomorphism.

Rewrite decision:

- add `TransformerBlockTrainingExample` and `TransformerBlockTrainingSet`,
- add `TransformerBlockTrainStep : TransformerTrainingState ->
  TransformerTrainingState`,
- add `transformer_block_average_loss`,
- add private forward caches for the masked block and feed-forward sublayer so
  training and inference share the same forward path,
- add private readout, feed-forward, softmax-loss, and layer-normalization
  backward helpers,
- update the example to show block loss decreasing after one composed update,
- update public docs and exercises so the remaining gap is attention-projection
  and normalization-parameter gradients, not the whole readout-to-feed-forward
  chain.

Validation:

```bash
cargo fmt --check
cargo test attention::tests --lib
cargo run --example 06_attention_scores
```

The targeted Rust path passed with 64 attention tests, and the example showed
block loss decreasing from `0.456495` to `0.419403` after one composed update.
The full publication gate passed with 95 Rust tests, all examples, the
main demo, prose style checks, source coverage checks, book build, and chapter
tests.

## Sweep 2 Pass 26: Attention Output Projection Gradient

Files:

```text
src/attention.rs
README.md
ROADMAP.md
book/src/roadmap.md
book/src/glossary.md
community/reader-review-guide.md
community/starter-issues.md
community/feedback-wall.md
exercises/advanced/README.md
exercises/ANSWER_KEY.md
book/TEXTBOOK_COMPLETION_AUDIT.md
```

Primary sources:

- [Attention Is All You Need](https://arxiv.org/abs/1706.03762)
- [Dive into Deep Learning: Backpropagation and Computational Graphs](https://d2l.ai/chapter_multilayer-perceptrons/backprop.html)
- [Dive into Deep Learning: Multi-Head Attention](https://d2l.ai/chapter_attention-mechanisms-and-transformers/multihead-attention.html)
- [Dive into Deep Learning: Transformer Architecture](https://d2l.ai/chapter_attention-mechanisms-and-transformers/transformer.html)

Critique before rewrite:

The composed block update reached the feed-forward sublayer, but it still left
the attention output projection fixed. That created an awkward teaching gap:
the book already emphasized `MultiHeadOutput -> ProjectedAttentionOutput` as a
typed boundary, but the training step did not yet show that the same boundary
can receive a gradient from token loss.

Rewrite decision:

- add `AttentionOutputProjection` weight and bias accessors,
- extend the masked-block training cache with `MultiHeadOutput` and
  pre-attention-normalization residual rows,
- add private attention-output-projection gradient accumulation and update
  helpers,
- extend `TransformerBlockTrainStep` so the token loss updates readout,
  feed-forward, and attention output projection parameters together,
- add a regression test that verifies the attention output projection weights
  change after one block training step,
- update public docs and exercises so the next gap at that point was attention
  input-projection and normalization-parameter gradients.

Validation:

```bash
cargo fmt --check
cargo test attention::tests --lib
cargo run --example 06_attention_scores
python3 scripts/check-prose-style.py
bash scripts/check-mdbook-coverage.sh
git diff --check -- . ':!target'
bash scripts/check.sh
```

The targeted Rust path passed with 65 attention tests. The full publication
gate passed with 96 Rust tests, all examples, the main demo, prose style
checks, source coverage checks, book build, and chapter tests.

## Sweep 2 Pass 27: Trainable Layer Normalization Gradient

Files:

```text
src/attention.rs
README.md
ROADMAP.md
book/src/roadmap.md
book/src/glossary.md
community/reader-review-guide.md
community/starter-issues.md
community/feedback-wall.md
exercises/advanced/README.md
exercises/ANSWER_KEY.md
book/TEXTBOOK_COMPLETION_AUDIT.md
```

Primary sources:

- [Layer Normalization](https://arxiv.org/abs/1607.06450)
- [Attention Is All You Need](https://arxiv.org/abs/1706.03762)
- [Dive into Deep Learning: Backpropagation and Computational Graphs](https://d2l.ai/chapter_multilayer-perceptrons/backprop.html)
- [Dive into Deep Learning: Transformer Architecture](https://d2l.ai/chapter_attention-mechanisms-and-transformers/transformer.html)

Critique before rewrite:

The composed block update passed gradients through layer normalization, but it
treated the scale and shift vectors as fixed constants. That weakened the
pedagogical contract: the code had a named `LayerNormParameters` object, yet
the training state did not show how those parameters can be updated by the same
token-level loss.

Rewrite decision:

- add explicit `LayerNormParameters` accessors for scale, shift, and epsilon,
- keep `LayerNormalization` as the boundary object that owns validated
  normalization parameters,
- add private layer-normalization gradient accumulation and update helpers,
- extend `TransformerBlockTrainStep` so the token loss updates both
  normalization parameter sets alongside the readout, feed-forward sublayer,
  and attention output projection,
- rebuild the structured parameter object through validated constructors,
- add a regression test that verifies layer-normalization parameters change
  after one block training step,
- update public docs and exercises so the remaining Transformer-training gap is
  query/key/value projection gradients.

Validation:

```bash
cargo fmt --check
cargo test attention::tests --lib
cargo run --example 06_attention_scores
python3 scripts/check-prose-style.py
bash scripts/check-mdbook-coverage.sh
git diff --check -- . ':!target'
bash scripts/check.sh
```

The targeted Rust path passed with 66 attention tests. The attention example
showed block loss decreasing from `0.456495` to `0.409737` after one composed
update. The full publication gate passed with 97 Rust tests, all examples,
the main demo, prose style checks, source coverage checks, book build, and
chapter tests.

## Sweep 2 Pass 28: Query Key Value Gradients

Files:

```text
src/attention.rs
README.md
ROADMAP.md
book/src/roadmap.md
book/src/glossary.md
book/src/source-snapshots.md
community/reader-review-guide.md
community/starter-issues.md
community/feedback-wall.md
exercises/advanced/README.md
exercises/ANSWER_KEY.md
book/TEXTBOOK_COMPLETION_AUDIT.md
```

Primary sources:

- [Attention Is All You Need](https://arxiv.org/abs/1706.03762)
- [Dive into Deep Learning: Multi-Head Attention](https://d2l.ai/chapter_attention-mechanisms-and-transformers/multihead-attention.html)
- [Dive into Deep Learning: Backpropagation and Computational Graphs](https://d2l.ai/chapter_multilayer-perceptrons/backprop.html)
- [Dive into Deep Learning: Transformer Architecture](https://d2l.ai/chapter_attention-mechanisms-and-transformers/transformer.html)

Critique before rewrite:

The composed block update reached the readout, feed-forward sublayer, attention
output projection, and layer-normalization parameters, but the query, key, and
value projections were still fixed. That made the attention section feel
structural but not trainable: the book taught query-key scores and value mixing,
yet the token-level gradient stopped just before those boundaries.

Rewrite decision:

- expose projection weights and biases through read-only accessors,
- store query, key, value, attention weights, and head output rows in the
  masked-block training cache,
- add private hidden-projection gradient accumulation and update helpers,
- backpropagate through head concatenation, value mixing, attention softmax,
  and scaled query-key scores,
- update each `SelfAttentionHead` through validated `HiddenToQuery`,
  `HiddenToKey`, and `HiddenToValue` constructors,
- add a regression test that verifies the composed block step changes the
  query/key/value projection families across heads,
- update public docs and exercises so the next training-quality gap is
  finite-difference gradient checking, not missing Q/K/V updates.

Validation:

```bash
cargo fmt --check
cargo test attention::tests --lib
cargo run --example 06_attention_scores
python3 scripts/check-prose-style.py
bash scripts/check-mdbook-coverage.sh
git diff --check -- . ':!target'
bash scripts/check.sh
```

The targeted Rust path passed with 67 attention tests. The attention example
kept the same visible one-step block loss decrease from `0.456495` to
`0.409737`, while the new regression test verified that query/key/value
projection weights change. The full publication gate passed with 98 Rust tests,
all examples, the main demo, prose style checks, source coverage checks,
book build, and chapter tests.

## Sweep 2 Pass 29: Attention Projection Finite Difference Check

Files:

```text
src/attention.rs
README.md
ROADMAP.md
community/reader-review-guide.md
community/starter-issues.md
community/feedback-wall.md
exercises/advanced/README.md
exercises/ANSWER_KEY.md
book/TEXTBOOK_COMPLETION_AUDIT.md
```

Primary sources:

- [Dive into Deep Learning: Backpropagation and Computational Graphs](https://d2l.ai/chapter_multilayer-perceptrons/backprop.html)
- [Dive into Deep Learning: Gradient Descent](https://d2l.ai/chapter_optimization/gd.html)

Critique before rewrite:

The Q/K/V update had behavioral coverage showing projection weights changed,
but it did not yet compare an inferred analytic gradient with a numerical
derivative. That left a trust gap: the code could update parameters while still
having a sign or scaling mistake inside the attention path.

Rewrite decision:

- infer the analytic gradient for one changed attention projection weight from
  the one-step parameter delta,
- perturb that same weight up and down in cloned training states,
- compare the central finite difference of `transformer_block_average_loss`
  with the inferred gradient,
- select the largest changed attention projection weight so the tiny causal
  fixture avoids a zero-gradient row,
- update public docs so the remaining work is broader gradient-checking
  coverage, not the absence of any finite-difference check.

Validation:

```bash
cargo fmt --check
cargo test attention::tests --lib
python3 scripts/check-prose-style.py
bash scripts/check-mdbook-coverage.sh
git diff --check -- . ':!target'
bash scripts/check.sh
```

The targeted Rust path passed with 68 attention tests. The full publication
gate passed with 99 Rust tests, all examples, the main demo, prose style
checks, source coverage checks, book build, and chapter tests.

## Sweep 2 Pass 30: Full Parameter Finite Difference Checks

Files:

```text
src/attention.rs
README.md
ROADMAP.md
community/reader-review-guide.md
community/starter-issues.md
community/feedback-wall.md
exercises/advanced/README.md
exercises/ANSWER_KEY.md
book/TEXTBOOK_COMPLETION_AUDIT.md
```

Primary sources:

- [Dive into Deep Learning: Backpropagation and Computational Graphs](https://d2l.ai/chapter_multilayer-perceptrons/backprop.html)
- [Dive into Deep Learning: Gradient Descent](https://d2l.ai/chapter_optimization/gd.html)

Critique before rewrite:

The attention projection had one numerical gradient check, but the broader
structured block still lacked comparable checks for readout, feed-forward,
layer-normalization, and bias parameters. That made the training endomorphism
look complete in prose before the code had enough independent evidence for its
sign and scaling.

Rewrite decision:

- add finite-difference tests for readout weights and biases,
- add finite-difference tests for feed-forward weights and biases,
- add finite-difference tests for layer-normalization parameters,
- add finite-difference tests for attention output and hidden-projection biases,
- keep the checks private to the test module so the public teaching API remains
  simple,
- update public docs so the remaining work is learner-facing exercise and
  diagram design, not missing finite-difference coverage for the current tiny
  trainer.

Validation:

```bash
cargo fmt --check
cargo test finite_difference --lib
cargo test attention::tests --lib
python3 scripts/check-prose-style.py
bash scripts/check-mdbook-coverage.sh
git diff --check -- . ':!target'
bash scripts/check.sh
```

The finite-difference subset passed with 8 tests. The targeted attention path
passed with 75 attention tests. The full publication gate passed with 106 Rust
tests, all examples, the main demo, prose style checks, source coverage
checks, book build, and chapter tests.

## Sweep 2 Pass 31: Finite Difference Exercise Bridge

Files:

```text
exercises/advanced/README.md
exercises/ANSWER_KEY.md
exercises/README.md
book/src/exercises.md
book/TEXTBOOK_COMPLETION_AUDIT.md
community/reader-review-guide.md
community/starter-issues.md
community/feedback-wall.md
```

Primary sources:

- [Dive into Deep Learning: Backpropagation and Computational Graphs](https://d2l.ai/chapter_multilayer-perceptrons/backprop.html)
- [Dive into Deep Learning: Gradient Descent](https://d2l.ai/chapter_optimization/gd.html)

Critique before rewrite:

The repository had code-level finite-difference checks, but the learner path
still treated gradient checking as future work. That meant a reader could see
that tests existed without a guided exercise for explaining why the numerical
check is meaningful.

Rewrite decision:

- add an advanced exercise that points directly at the eight finite-difference
  tests in `src/attention.rs`,
- ask the reader to compare an inferred gradient from the one-step update with
  the central finite difference of average loss,
- add a diagram that shows the split between the one-update path and the
  two-perturbation loss-measurement path,
- add facilitator guidance that connects the check to sign errors, missing
  bias gradients, dropped projection paths, and scale mismatches,
- update the audit and community review prompts so the remaining gap is
  worked solutions and external reader feedback, not the absence of a
  learner-facing gradient exercise.

Validation:

```bash
python3 scripts/check-prose-style.py
bash scripts/check-mdbook-coverage.sh
git diff --check -- . ':!target'
bash scripts/check.sh
```

The full publication gate passed with 106 Rust tests, all examples, the main
demo, prose style checks, source coverage checks, book build, and chapter
tests.

## Sweep 2 Pass 38: Chapter Reference Coverage Gate

Files:

```text
scripts/check-chapter-references.py
scripts/check.sh
book/src/roadmap.md
book/STYLE_GUIDE.md
book/TEXTBOOK_COMPLETION_AUDIT.md
```

Critique before rewrite:

The reference chapter had a chapter-by-chapter source map, and the general
coverage guard required important external URLs. The weakness was that no
checker verified the map as a chapter-level contract. A future rewrite could
add a major chapter without a source row, or a chapter could lose its reader
bridge back to the references.

Rewrite decision:

- add `scripts/check-chapter-references.py`,
- require every major chapter to have a reference-map row with a central
  question and at least three external sources,
- require the reference chapter to keep Rust, category theory, ML,
  category-theory-and-learning-systems, Transformer, and learning-design source
  sections,
- require each major chapter to point back to `references.md` or provide a
  roadmap reference path,
- wire the checker into `scripts/check.sh`,
- update the roadmap's reference path to link directly to the references
  chapter,
- update the style guide and completion audit so the checker is part of the
  reference-backed rewrite loop.

Validation:

```bash
python3 scripts/check-chapter-references.py
bash scripts/check.sh
```

Result:

```text
Chapter reference coverage check passed.
```

The full publication gate passed with 106 Rust tests, all examples, the main
demo, prose style checks, chapter reference coverage checks, source coverage
checks, book build, and chapter tests.

## Sweep 2 Pass 39: Exercise Alignment Gate

Files:

```text
scripts/check-exercise-alignment.py
scripts/check.sh
exercises/ANSWER_KEY.md
book/STYLE_GUIDE.md
book/TEXTBOOK_COMPLETION_AUDIT.md
```

Critique before rewrite:

The book had a core chapter practice map and a public answer key, but the
validation gate did not prove that those surfaces stayed aligned. A later edit
could rename an exercise, add a chapter practice target, or change a compact
exercise file without updating facilitator notes.

Rewrite decision:

- normalize the advanced finite-difference answer-key heading to match the
  compact exercise naming scheme,
- add `scripts/check-exercise-alignment.py`,
- verify that the core chapter practice map covers every major chapter,
- verify that the exercise ladder links to beginner, intermediate, advanced,
  and answer-key files,
- verify that every exercise heading in the chapter, beginner, intermediate,
  and advanced exercise files has a matching answer-key heading,
- wire the checker into `scripts/check.sh`,
- update the style guide and completion audit so exercise alignment is part of
  the textbook gate.

Validation:

```bash
python3 scripts/check-exercise-alignment.py
bash scripts/check.sh
```

Result:

```text
Exercise alignment check passed.
```

The full publication gate passed with 106 Rust tests, all examples, the main
demo, prose style checks, chapter reference coverage checks, exercise alignment
checks, source coverage checks, book build, and chapter tests.

## Sweep 2 Pass 40: Rewrite Log Audit Gate

Files:

```text
scripts/check-rewrite-log.py
scripts/check.sh
book/CHAPTER_REWRITE_LOG.md
book/STYLE_GUIDE.md
book/TEXTBOOK_COMPLETION_AUDIT.md
```

Critique before rewrite:

The project had a detailed rewrite log, but the validation gate did not prove
that every major chapter had auditable critique, rewrite, and validation
evidence. Since the active objective depends on iterative expert and learner
critique, the editorial log itself needed a guard.

Rewrite decision:

- remove a stale `Next Pass` heading from the rewrite log,
- add `scripts/check-rewrite-log.py`,
- require every major chapter to be mentioned in a rewrite-log section,
- require each major chapter section to include critique before rewrite,
  rewrite decision, and validation markers,
- require expert and learner critique lenses for the major teaching chapters,
- wire the checker into `scripts/check.sh`,
- update the style guide and completion audit so rewrite-log evidence is part
  of the textbook gate.

Validation:

```bash
python3 scripts/check-rewrite-log.py
bash scripts/check.sh
```

Result:

```text
Rewrite log check passed.
```

The full publication gate passed with 106 Rust tests, all examples, the main
demo, prose style checks, rewrite-log checks, chapter reference coverage
checks, exercise alignment checks, source coverage checks, book build, and
chapter tests.

## Sweep 2 Pass 41: Completion Audit Guard

Files:

```text
scripts/check-completion-audit.py
scripts/check.sh
book/TEXTBOOK_COMPLETION_AUDIT.md
book/STYLE_GUIDE.md
```

Critique before rewrite:

The completion audit was doing important work, but it was still only prose.
The active objective explicitly requires a prompt-to-artifact checklist and
real evidence before any completion claim. A future edit could accidentally
remove a required objective item, validation marker, or missing-work statement
and make the audit less trustworthy.

Rewrite decision:

- add `scripts/check-completion-audit.py`,
- require the audit to keep objective, checklist, validation, missing-work, and
  completion-rule sections,
- require the objective to list the six active deliverables,
- require the checklist to keep the core requirements and non-empty evidence,
  status, and gap cells,
- require validation evidence to mention the current validation scripts,
- require current missing work to keep the direct-reader-feedback blocker,
- wire the checker into `scripts/check.sh`,
- update the style guide and audit so the completion-audit guard is part of
  the textbook gate.

Validation:

```bash
python3 scripts/check-completion-audit.py
bash scripts/check.sh
```

Result:

```text
Completion audit check passed.
```

The full publication gate passed with 106 Rust tests, all examples, the main
demo, prose style checks, completion-audit checks, rewrite-log checks, chapter
reference coverage checks, exercise alignment checks, source coverage checks,
book build, and chapter tests.

## Sweep 2 Pass 42: Reader Feedback Triage Protocol

Files:

```text
community/reader-feedback-triage.md
community/reader-review-guide.md
community/reader-review-packet.md
community/feedback-wall.md
community/starter-issues.md
book/TEXTBOOK_COMPLETION_AUDIT.md
scripts/check-completion-audit.py
```

Critique before rewrite:

The project had a reader review guide, review packet, issue form, and feedback
wall, but it did not yet define how maintainers should convert direct reader
reports into source-backed chapter rewrites. That left a risk that future
feedback could become unstructured discussion instead of auditable textbook
improvement.

Rewrite decision:

- add `community/reader-feedback-triage.md`,
- define the evidence required before rewriting,
- map feedback lenses to likely artifacts and repair types,
- distinguish fix-now, batch-theme, clarification, and out-of-scope outcomes,
- require feedback-driven rewrites to update the rewrite log and validation
  evidence,
- link the triage protocol from existing reader review surfaces,
- update the completion audit and audit checker so the triage protocol remains
  part of the direct-feedback missing-work path.

Validation:

```bash
python3 scripts/check-completion-audit.py
bash scripts/check.sh
```

Result:

```text
Completion audit check passed.
```

The full publication gate passed with 106 Rust tests, all examples, the main
demo, prose style checks, completion-audit checks, rewrite-log checks, chapter
reference coverage checks, exercise alignment checks, source coverage checks,
book build, and chapter tests.

## Sweep 2 Pass 36: Direct Feedback Availability Check

Files:

```text
book/TEXTBOOK_COMPLETION_AUDIT.md
```

Critique before rewrite:

The project now has a reader review guide, review packet, starter issue paths,
and a structured issue form. That makes the direct-feedback loop ready, but it
does not prove that any project-specific reader reports have arrived. The
completion audit needed live evidence so future chapter rewrites do not mistake
prepared feedback infrastructure for actual reader feedback.

Rewrite decision:

- check live GitHub issues for the `reader confusion` label,
- record the exact command and empty result in the completion audit,
- keep direct project-specific reader reports as the next missing textbook
  input instead of treating the world-class-textbook objective as complete.

Validation:

```bash
gh issue list --repo hghalebi/category_theory_transformer_rs --state all --label "reader confusion" --limit 50 --json number,title,state,labels,createdAt,updatedAt,url
```

Result:

```text
[]
```

## Sweep 2 Pass 35: Reader Review Packet

Files:

```text
community/reader-review-packet.md
community/reader-review-guide.md
community/feedback-wall.md
.github/ISSUE_TEMPLATE/config.yml
README.md
book/TEXTBOOK_COMPLETION_AUDIT.md
```

Critique before rewrite:

The project had a detailed review guide and a structured reader-confusion
issue form, but a first-time reviewer still had to decide how much to read and
which command to run. That friction could reduce direct reader reports before
they reach the issue form.

Rewrite decision:

- add a short review packet with three paths: first-run, core-chapter, and
  attention/gradient review,
- make the desired outcome one actionable issue rather than a broad review,
- include command-first instructions and examples of useful and weak feedback,
- link the packet from the README, feedback wall, reader review guide, and
  GitHub issue chooser.

Validation:

```bash
ruby -e 'require "yaml"; YAML.load_file(".github/ISSUE_TEMPLATE/config.yml")'
python3 scripts/check-prose-style.py
bash scripts/check-mdbook-coverage.sh
git diff --check -- . ':!target'
bash scripts/check.sh
```

The full publication gate passed with 106 Rust tests, all examples, the main
demo, prose style checks, source coverage checks, book build, and chapter
tests.

## Sweep 2 Pass 32: Structured Direct Reader Report Form

Files:

```text
.github/ISSUE_TEMPLATE/reader-confusion.yml
community/reader-review-guide.md
book/TEXTBOOK_COMPLETION_AUDIT.md
```

Primary source:

- [GitHub Docs: Syntax for issue forms](https://docs.github.com/en/communities/using-templates-to-encourage-useful-issues-and-pull-requests/syntax-for-issue-forms)

Critique before rewrite:

The repo had a reader-confusion issue form, but it collected only location,
quote, and expectation. That was enough for a single unclear sentence, but too
weak for the textbook rewrite loop because it did not capture the review path,
command tried, friction type, last clear idea, or smallest useful fix.

Rewrite decision:

- expand the existing issue form instead of adding a duplicate template,
- add a review-path dropdown to separate first-run, chapter, attention, and
  exercise feedback,
- add a friction-lens dropdown aligned with the reader review guide,
- require the command or file tried so reports stay tied to runnable evidence,
- ask for the last clear idea and the exact mental-model break,
- update the reader review guide and completion audit to treat the issue form
  as part of the direct-feedback loop.

Validation:

```bash
python3 scripts/check-prose-style.py
bash scripts/check-mdbook-coverage.sh
git diff --check -- . ':!target'
bash scripts/check.sh
```

The full publication gate passed with 106 Rust tests, all examples, the main
demo, prose style checks, source coverage checks, book build, and chapter
tests.

## Sweep 2 Pass 33: External Feedback Proxy Synthesis

Files:

```text
community/external-feedback-synthesis.md
community/feedback-wall.md
book/TEXTBOOK_COMPLETION_AUDIT.md
```

Primary sources:

- [Rust Learn](https://rust-lang.org/learn/)
- [Brown University experimental Rust Book](https://rust-book.cs.brown.edu/experiment-intro.html)
- [Rustlings usage guide](https://rustlings.rust-lang.org/usage/)
- [100 Exercises To Learn Rust](https://rust-exercises.com/100-exercises/01_intro/00_welcome)
- [Category theory for programmers made easier](https://www.johndcook.com/blog/2020/11/02/category-theory-for-programmers/)
- [Stack Overflow: What exactly is a category?](https://stackoverflow.com/questions/46510557/what-exactly-is-a-category)
- [CS231n Optimization: numerical gradients](https://cs231n.github.io/optimization-1/)
- [CS231n Backpropagation](https://cs231n.github.io/optimization-2/)
- [Dive into Deep Learning: Backpropagation and Computational Graphs](https://d2l.ai/chapter_multilayer-perceptrons/backprop.html)
- [Dive into Deep Learning: Gradient Descent](https://d2l.ai/chapter_optimization/gd.html)

Critique before rewrite:

The audit asked for external reader reports, but the repository only had a
review guide. Direct project-specific reports cannot be fabricated. The honest
next step was to separate direct feedback, which is still pending, from public
learner-friction signals that can guide the next rewrite pass.

Rewrite decision:

- add a community-facing proxy synthesis of public learner-friction signals,
- label the synthesis as proxy guidance rather than direct project feedback,
- turn the signals into a rewrite checklist covering practice, failure
  signals, abstraction, source support, and transfer,
- update the audit so direct reader reports remain a missing completion item.

Validation:

```bash
python3 scripts/check-prose-style.py
bash scripts/check-mdbook-coverage.sh
git diff --check -- . ':!target'
bash scripts/check.sh
```

The full publication gate passed with 106 Rust tests, all examples, the main
demo, prose style checks, source coverage checks, book build, and chapter
tests.

## Sweep 2 Pass 34: Source-Backed Gradient-Checking Worked Example

Files:

```text
book/src/exercises.md
book/src/references.md
exercises/advanced/README.md
book/TEXTBOOK_COMPLETION_AUDIT.md
community/reader-review-guide.md
community/starter-issues.md
community/feedback-wall.md
```

Primary sources:

- [CS231n Optimization: numerical gradients](https://cs231n.github.io/optimization-1/)
- [CS231n Backpropagation](https://cs231n.github.io/optimization-2/)
- [Dive into Deep Learning: Backpropagation and Computational Graphs](https://d2l.ai/chapter_multilayer-perceptrons/backprop.html)
- [Dive into Deep Learning: Gradient Descent](https://d2l.ai/chapter_optimization/gd.html)

Critique before rewrite:

The advanced finite-difference exercise had a prompt, diagram, and answer-key
shape, but the main exercise chapter did not walk a reader through one complete
worked example. That left a gap between "run the finite-difference tests" and
"explain why the two slopes should match."

Rewrite decision:

- add a worked example to `book/src/exercises.md` that explains the one-update
  path and the two-perturbation loss-measurement path,
- connect the explanation to centered finite differences, reverse-order
  backpropagation, and gradient descent parameter updates,
- extend the chapter reference map with CS231n optimization and D2L
  backpropagation for the exercise chapter,
- update the advanced exercise prompt to point readers to the worked example,
- update public review prompts so future work is reader-feedback-driven
  refinement, not absence of a first worked solution.

Validation:

```bash
python3 scripts/check-prose-style.py
bash scripts/check-mdbook-coverage.sh
git diff --check -- . ':!target'
bash scripts/check.sh
```

The full publication gate passed with 106 Rust tests, all examples, the main
demo, prose style checks, source coverage checks, book build, and chapter
tests.

## Sweep 2 Pass 37: Learning-Science Practice Contract

Files:

```text
book/src/references.md
book/EDITORIAL_RESEARCH_NOTES.md
book/STYLE_GUIDE.md
book/TEXTBOOK_COMPLETION_AUDIT.md
scripts/check-mdbook-coverage.sh
```

Primary sources:

- [Improving Students' Learning With Effective Learning Techniques](https://journals.sagepub.com/doi/abs/10.1177/1529100612453266)
- [Test-Enhanced Learning](https://doi.org/10.1111/j.1467-9280.2006.01693.x)
- [Structuring the Transition From Example Study to Problem Solving](https://doi.org/10.1207/S15326985EP3801_3)
- [Self-Explanations](https://doi.org/10.1207/s15516709cog1302_1)

Critique before rewrite:

The book had a strong practice shape: worked examples, self-checks, retrieval
practice, and transfer exercises. The weakness was that the learning-science
source base was still too implicit. How People Learn II was present, but the
specific editorial moves for retrieval practice, faded examples, and
self-explanation deserved direct source backing.

Rewrite decision:

- add learning-science sources to the references chapter and exercise chapter
  map,
- add a source-to-editorial-check ledger to the internal research notes,
- make the style guide's practice progression explicit:
  `worked example -> self-explanation -> faded example -> retrieval ->
  transfer`,
- add coverage-guard requirements so these sources do not disappear during
  later rewrites,
- update the completion audit so learning-science grounding becomes a tracked
  requirement.

Validation:

```bash
python3 scripts/check-prose-style.py
bash scripts/check-mdbook-coverage.sh
git diff --check -- . ':!target'
bash scripts/check.sh
```

The full publication gate passed with 106 Rust tests, all examples, the main
demo, prose style checks, source coverage checks, book build, and chapter
tests.

## Sweep 2 Pass 43: Reader Feedback Loop Gate

Files:

```text
scripts/check-reader-feedback-loop.py
scripts/check.sh
scripts/check-completion-audit.py
book/STYLE_GUIDE.md
book/TEXTBOOK_COMPLETION_AUDIT.md
```

Critique before rewrite:

The repository had a structured reader-confusion issue form, review guide,
review packet, feedback wall, starter issues, and triage protocol. The
weakness was that those surfaces were only connected by convention. A future
edit could rename a field, remove a triage link, or hide the reader packet
without breaking the main validation gate.

Rewrite decision:

- add `scripts/check-reader-feedback-loop.py`,
- require the reader-confusion issue form to keep the review path, command,
  friction lens, confusing quote, last clear idea, mental-model break,
  expectation, and smallest-fix fields,
- require the issue chooser, review guide, review packet, feedback wall,
  starter issues, completion audit, and triage protocol to stay cross-linked,
- require the triage protocol to keep evidence, classification, rewrite,
  logging, validation, and close-out steps,
- wire the checker into `scripts/check.sh`,
- update the style guide and completion audit so reader-feedback capture is
  part of the editorial gate.

Validation:

```bash
python3 scripts/check-reader-feedback-loop.py
python3 scripts/check-completion-audit.py
python3 scripts/check-rewrite-log.py
python3 scripts/check-prose-style.py
bash scripts/check.sh
```

Result:

```text
Reader feedback loop check passed.
Completion audit check passed.
Rewrite log check passed.
Prose style check passed.
```

The full publication gate passed with 106 Rust tests, all examples, the main
demo, prose style checks, completion-audit checks, reader-feedback-loop checks,
rewrite-log checks, chapter reference coverage checks, exercise alignment
checks, source coverage checks, book build, and chapter tests.

## Sweep 2 Pass 44: Attention Shape-Flow Exercise

Files:

```text
book/src/roadmap.md
book/src/exercises.md
exercises/ANSWER_KEY.md
exercises/advanced/README.md
book/EDITORIAL_RESEARCH_NOTES.md
book/TEXTBOOK_COMPLETION_AUDIT.md
```

Primary sources:

- [Attention Is All You Need](https://arxiv.org/abs/1706.03762)
- [Dive into Deep Learning: Attention Mechanisms and Transformers](https://d2l.ai/chapter_attention-mechanisms-and-transformers/index.html)
- [Dive into Deep Learning: Multi-Head Attention](https://d2l.ai/chapter_attention-mechanisms-and-transformers/multihead-attention.html)
- [Dive into Deep Learning: Transformer Architecture](https://d2l.ai/chapter_attention-mechanisms-and-transformers/transformer.html)
- [The Annotated Transformer](https://nlp.seas.harvard.edu/annotated-transformer/)
- [Layer Normalization](https://arxiv.org/abs/1607.06450)

Critique before rewrite:

The roadmap had strong typed attention material and the example printed the
full attention path, but a learner still had to infer how the output lines,
Rust types, and Transformer reference path fit together. The completion audit
also still listed Transformer diagrams as a remaining gap. Without a compact
shape-flow exercise, the attention section risked reading as a component list
instead of a trace the reader can actively explain.

ML teaching critique:

The reader needed a concrete distinction between raw scores, masked scores,
normalized weights, mixed values, output projection, residual shape,
normalization, and feed-forward refinement.

Category-theory critique:

The roadmap needed to make product inputs and shape-preserving returns visible:
query-key scoring and value mixing consume paired objects, while the enclosing
block repeatedly returns to `HiddenSequence`.

Learner critique:

A motivated reader could run `cargo run --example 06_attention_scores` and see
many output lines without knowing which line proves which typed boundary. The
new exercise makes the terminal output a checklist.

Rewrite decision:

- add a Mermaid attention shape-flow diagram to the Transformer roadmap,
- add Exercise 12 so readers trace the first output line for each attention
  shape,
- add answer-key reasoning that separates scores, masks, weights, values,
  projection, residual, normalization, and feed-forward,
- update the advanced exercise wording so completed gradient-checking work is
  not described as still planned,
- update the internal research notes and completion audit to record the new
  attention diagram and shape-tracing exercise.

Validation:

```bash
python3 scripts/check-exercise-alignment.py
python3 scripts/check-completion-audit.py
python3 scripts/check-prose-style.py
bash scripts/check-mdbook-coverage.sh
bash scripts/check.sh
```

Focused checks passed before the full gate. The full publication gate then
passed with 106 Rust tests, all examples, the main demo, prose style checks,
completion-audit checks, reader-feedback-loop checks, rewrite-log checks,
chapter reference coverage checks, exercise alignment checks, source coverage
checks, book build, and chapter tests.

## Sweep 2 Pass 45: Cross-Entropy Target-Probability Exercise

Files:

```text
book/src/03-ml-pipeline.md
book/src/exercises.md
exercises/ANSWER_KEY.md
book/EDITORIAL_RESEARCH_NOTES.md
book/TEXTBOOK_COMPLETION_AUDIT.md
```

Primary sources:

- [Dive into Deep Learning: Softmax Regression](https://d2l.ai/chapter_linear-classification/softmax-regression.html)
- [Dive into Deep Learning: Softmax Regression Implementation from Scratch](https://d2l.ai/chapter_linear-classification/softmax-regression-scratch.html)
- [CS231n: Linear Classification](https://cs231n.github.io/linear-classify/)
- [Rust Book: How to Write Tests](https://doc.rust-lang.org/book/ch11-01-writing-tests.html)

Critique before rewrite:

The ML pipeline chapter explained logits, probabilities, and cross entropy,
and `src/ml.rs` had a test proving lower loss for higher target probability.
The weakness was practice: the exercise ladder did not force the learner to
calculate the target-token loss directly. A reader could still confuse "highest
probability" with "probability assigned to the correct target."

ML teaching critique:

The core intuition should be numeric before it is abstract: for the same target
token, `0.90` gives a small loss and `0.10` gives a large loss.

Category-theory critique:

The exercise should emphasize that `CrossEntropy` needs a product input:
`Distribution x TokenId -> Loss`. The distribution alone is not enough because
the target chooses which probability matters.

Learner critique:

A motivated learner might passively accept that cross entropy measures
"surprise" without checking the target index. The new exercise makes the
target index explicit and asks the learner to connect the calculation to the
unit test name.

Rewrite decision:

- add a small numeric cross-entropy trace to the ML pipeline chapter,
- add Exercise 13 for target-probability loss calculation,
- add answer-key reasoning and a facilitator correction for the common
  "choose the largest probability" mistake,
- update the research notes and completion audit so loss intuition is part of
  the tracked exercise surface.

Validation:

```bash
python3 scripts/check-exercise-alignment.py
python3 scripts/check-completion-audit.py
python3 scripts/check-chapter-references.py
python3 scripts/check-prose-style.py
cargo test cross_entropy_is_lower_for_more_confident_target_probability --lib
bash scripts/check-mdbook-coverage.sh
git diff --check -- . ':!target'
```

Focused checks passed. The full publication gate then passed with 106 Rust
tests, all examples, the main demo, prose style checks, completion-audit
checks, reader-feedback-loop checks, rewrite-log checks, chapter reference
coverage checks, exercise alignment checks, source coverage checks, book build,
and chapter tests.

## Sweep 2 Pass 46: Naturality And Monoid Law-Tracing Exercise

Files:

```text
book/src/05-structure-and-calculus.md
book/src/exercises.md
exercises/ANSWER_KEY.md
book/EDITORIAL_RESEARCH_NOTES.md
book/TEXTBOOK_COMPLETION_AUDIT.md
```

Primary sources:

- [Seven Sketches in Compositionality](https://arxiv.org/abs/1803.05316)
- [Category Theory for Programming](https://arxiv.org/abs/2209.01259)
- [Rust Book: Advanced Traits](https://doc.rust-lang.org/stable/book/ch20-02-advanced-traits.html)
- [Rust By Example: Associated Types](https://doc.rust-lang.org/rust-by-example/generics/assoc_items/types.html)

Critique before rewrite:

The structure chapter already explained functors, natural transformations,
monoids, and chain rule, and it had executable law checks in `src/structure.rs`.
The weak point was practice coverage. The exercise map sent readers to a
functor mapping exercise, but did not force them to trace the naturality square
or the monoid laws back to the tests.

ML teaching critique:

The practical pipeline meaning should be visible: wrapper conversion should be
consistent, and trace grouping should not change the ordered record of pipeline
stages.

Category-theory critique:

The terms commutative square, identity, and associativity needed direct
exercise pressure. A learner should not be able to answer only by repeating the
law names.

Learner critique:

A motivated reader could see that the tests return `true` without explaining
the two naturality paths or the three monoid checks. The new exercise asks for
both paths, all three laws, and the exact functions that implement them.

Rewrite decision:

- add a Mermaid naturality-square diagram to the structure chapter,
- add a Mermaid monoid-grouping diagram to the structure chapter,
- add Exercise 14 for tracing naturality and monoid laws,
- add answer-key reasoning that names the two square paths and three monoid
  checks,
- update the research notes and completion audit so law tracing is a tracked
  category-theory practice surface.

Validation:

```bash
cargo test structure::tests --lib
python3 scripts/check-exercise-alignment.py
python3 scripts/check-completion-audit.py
python3 scripts/check-prose-style.py
```

Result:

```text
5 structure tests passed.
Exercise alignment check passed.
Completion audit check passed.
Prose style check passed.
```

The full publication gate then passed with 106 Rust tests, all examples, the
main demo, prose style checks, completion-audit checks, reader-feedback-loop
checks, rewrite-log checks, chapter reference coverage checks, exercise
alignment checks, exercise command checks, duplicate-prose checks, source
coverage checks, book build, and chapter tests.

## Sweep 2 Pass 115: Reader Report Quality Standard

Files:

```text
community/reader-feedback-triage.md
community/reader-feedback-inbox.md
scripts/check-reader-feedback-loop.py
book/TEXTBOOK_COMPLETION_AUDIT.md
```

Primary sources:

- `community/reader-feedback-triage.md`
- `community/reader-feedback-inbox.md`
- `scripts/check-reader-feedback-loop.py`

Critique before rewrite:

The direct-reader loop required five accepted reports before completion, but
the public triage protocol still left one practical ambiguity: a reviewer could
submit feedback that sounded useful while being too broad to drive a focused
rewrite. That would push maintainers back into guessing instead of converting
one concrete confusion point into one clearer learning step.

Rewrite decision:

- add usable and insufficient report examples to the triage protocol,
- define the accepted-report standard in the feedback inbox,
- emphasize location-bound, command-backed, mental-model-specific,
  rewrite-backed, and privacy-safe reports,
- guard those markers in `scripts/check-reader-feedback-loop.py`,
- record the pass in the completion audit.

Validation:

```bash
python3 scripts/check-reader-feedback-loop.py
python3 scripts/check-reader-feedback-inbox.py
git diff --check -- . ':!target'
bash scripts/check.sh
python3 scripts/check-reader-feedback-inbox.py --require-sprint-complete
scripts/check-textbook-readiness.sh
```

Result:

```text
Reader feedback loop check passed.
Reader feedback inbox check passed with no direct reports recorded.
Reader feedback sprint status: 0/5 accepted reports, 0 rewritten report(s).
Diff whitespace check passed.
Full publication gate passed with 106 Rust tests, all examples, the demo
binary, prose/audit/source/reference/exercise checks, book build, and chapter
tests.
Strict completion gate failed as expected because direct-reader reports are
still missing.
Textbook readiness check failed only on the strict direct-reader completion
gate; the remaining readiness checks passed.
```

Remaining risk:

The standard improves intake quality, but it still does not replace the missing
direct-reader reports required by the strict completion gate.

## Sweep 2 Pass 116: Reader Inbox Privacy Guard

Files:

```text
scripts/check-reader-feedback-inbox.py
community/reader-feedback-inbox.md
scripts/check-reader-feedback-loop.py
book/TEXTBOOK_COMPLETION_AUDIT.md
book/CHAPTER_REWRITE_LOG.md
```

Primary sources:

- `community/reader-feedback-inbox.md`
- `scripts/check-reader-feedback-inbox.py`
- `community/reader-feedback-triage.md`

Critique before rewrite:

The inbox standard said accepted reports should be privacy-safe, but that was
only prose. A maintainer could accidentally paste a private contact detail into
a public reader report while trying to preserve evidence. That would weaken the
public review process and make direct-reader acquisition riskier than needed.

Rewrite decision:

- add an email-address detector to `scripts/check-reader-feedback-inbox.py`,
- reject report bodies that contain email-address patterns,
- add a self-test fixture that proves private contact data is rejected,
- document the public-safe transcription rule in
  `community/reader-feedback-inbox.md`,
- guard the new marker in `scripts/check-reader-feedback-loop.py`.

Validation:

```bash
python3 scripts/check-reader-feedback-inbox.py --self-test
python3 scripts/check-reader-feedback-inbox.py
python3 scripts/check-reader-feedback-loop.py
python3 -m py_compile scripts/check-reader-feedback-inbox.py scripts/check-reader-feedback-loop.py
rm -rf scripts/__pycache__
git diff --check -- . ':!target'
```

Result:

```text
Reader feedback inbox self-test passed.
Reader feedback inbox check passed with no direct reports recorded.
Reader feedback sprint status: 0/5 accepted reports, 0 rewritten report(s).
Reader feedback loop check passed.
Python syntax check passed.
Diff whitespace check passed.
```

Remaining risk:

The guard catches email-address patterns. It cannot detect every possible
personal detail, so maintainers still need to avoid names, private messages,
and personal notes when transcribing workshop or private-channel feedback.

## Sweep 2 Pass 117: Reader Issue Draft Redaction

Files:

```text
scripts/collect-reader-feedback-issues.py
community/reader-feedback-triage.md
community/reader-feedback-inbox.md
scripts/check-reader-feedback-loop.py
book/TEXTBOOK_COMPLETION_AUDIT.md
book/CHAPTER_REWRITE_LOG.md
```

Primary sources:

- `.github/ISSUE_TEMPLATE/reader-confusion.yml`
- `scripts/collect-reader-feedback-issues.py`
- `community/reader-feedback-inbox.md`

Critique before rewrite:

The inbox validator rejected email-address patterns after a report was pasted,
but the GitHub issue collector still rendered issue-form values directly. That
made the privacy boundary reactive instead of preventive.

Rewrite decision:

- add email-address redaction to `scripts/collect-reader-feedback-issues.py`,
- apply redaction before issue-form values become inbox fields,
- extend the collector self-test with an email-containing smallest-fix field,
- document the redaction behavior in the triage protocol and inbox,
- guard the new public marker in `scripts/check-reader-feedback-loop.py`.

Validation:

```bash
python3 scripts/collect-reader-feedback-issues.py --self-test
python3 scripts/check-reader-feedback-inbox.py --self-test
python3 scripts/check-reader-feedback-inbox.py
python3 scripts/check-reader-feedback-loop.py
python3 -m py_compile scripts/collect-reader-feedback-issues.py scripts/check-reader-feedback-inbox.py scripts/check-reader-feedback-loop.py
rm -rf scripts/__pycache__
git diff --check -- . ':!target'
```

Result:

```text
Reader feedback issue collector self-test passed.
Reader feedback inbox self-test passed.
Reader feedback inbox check passed with no direct reports recorded.
Reader feedback sprint status: 0/5 accepted reports, 0 rewritten report(s).
Reader feedback loop check passed.
Python syntax check passed.
Diff whitespace check passed.
```

Remaining risk:

The collector redacts email addresses, but it cannot infer every kind of
personal note. Maintainer review is still required before recording accepted
direct-reader evidence.

## Sweep 2 Pass 118: Readiness Gate Coverage Audit

Files:

```text
scripts/check-textbook-readiness.sh
scripts/check-completion-audit.py
book/TEXTBOOK_COMPLETION_AUDIT.md
book/CHAPTER_REWRITE_LOG.md
```

Primary sources:

- `scripts/check.sh`
- `scripts/check-textbook-readiness.sh`
- `scripts/check-completion-audit.py`

Critique before rewrite:

The readiness gate existed and failed honestly on missing direct-reader
evidence, but it was thinner than the normal publication gate. It did not run
some cheap completion checks such as the issue collector self-test, reviewer
slot tracker checks, public-friction matrix, reference-link self-test, diagram
coverage, or duplicate-prose check. That made the readiness command less
representative than the completion audit claimed.

Rewrite decision:

- extend `scripts/check-textbook-readiness.sh` with the missing non-expensive
  completion checks,
- keep the strict direct-reader completion gate inside readiness so completion
  still fails honestly,
- update `scripts/check-completion-audit.py` so it verifies the readiness
  script contains the required command coverage,
- record the expanded readiness transcript in the completion audit.

Validation:

```bash
python3 scripts/check-completion-audit.py
python3 -m py_compile scripts/check-completion-audit.py
rm -rf scripts/__pycache__
scripts/check-textbook-readiness.sh
git diff --check -- . ':!target'
```

Result:

```text
Completion audit check passed.
Python syntax check passed.
Readiness gate ran the expected completion checks.
Readiness gate failed only on the strict direct-reader completion gate.
Diff whitespace check passed.
```

Remaining risk:

The readiness gate is now better aligned with the audit, but it is still
blocked by the same real-world evidence requirement: accepted direct-reader
reports and at least one rewritten report.

## Sweep 2 Pass 119: Reader Report Title Privacy Guard

Files:

```text
scripts/collect-reader-feedback-issues.py
scripts/check-reader-feedback-inbox.py
community/reader-feedback-triage.md
community/reader-feedback-inbox.md
scripts/check-reader-feedback-loop.py
book/TEXTBOOK_COMPLETION_AUDIT.md
book/CHAPTER_REWRITE_LOG.md
```

Primary sources:

- `scripts/collect-reader-feedback-issues.py`
- `scripts/check-reader-feedback-inbox.py`
- `community/reader-feedback-inbox.md`

Critique before rewrite:

The previous privacy guard redacted email-address patterns inside issue-form
fields and rejected inbox report bodies that contained email addresses. The
issue title still became the report heading without redaction, and the inbox
checker did not scan headings. A reader could therefore put private contact
data in an issue title and bypass the body-level guard.

Rewrite decision:

- redact email-address patterns in issue titles before they become report
  headings,
- update the collector self-test so the title contains an email address,
- reject email-address patterns in report headings as well as bodies,
- update the inbox self-test so the report heading contains an email address,
- document that titles, headings, and fields share the privacy boundary.

Validation:

```bash
python3 scripts/collect-reader-feedback-issues.py --self-test
python3 scripts/check-reader-feedback-inbox.py --self-test
python3 scripts/check-reader-feedback-inbox.py
python3 scripts/check-reader-feedback-loop.py
python3 -m py_compile scripts/collect-reader-feedback-issues.py scripts/check-reader-feedback-inbox.py scripts/check-reader-feedback-loop.py
rm -rf scripts/__pycache__
git diff --check -- . ':!target'
```

Result:

```text
Reader feedback issue collector self-test passed.
Reader feedback inbox self-test passed.
Reader feedback inbox check passed with no direct reports recorded.
Reader feedback sprint status: 0/5 accepted reports, 0 rewritten report(s).
Reader feedback loop check passed.
Python syntax check passed.
Diff whitespace check passed.
```

Remaining risk:

This protects email-address patterns in titles, headings, and bodies. It still
does not detect every possible private name or personal note, so maintainer
review remains part of accepting direct-reader reports.

## Sweep 2 Pass 120: Reader Issue Form Privacy Prompt

Files:

```text
.github/ISSUE_TEMPLATE/reader-confusion.yml
scripts/check-reader-feedback-loop.py
book/TEXTBOOK_COMPLETION_AUDIT.md
book/CHAPTER_REWRITE_LOG.md
```

Primary sources:

- `.github/ISSUE_TEMPLATE/reader-confusion.yml`
- `scripts/check-reader-feedback-loop.py`
- `community/reader-feedback-inbox.md`

Critique before rewrite:

The collector and inbox checker now redacted or rejected email-address
patterns, but the reader-facing issue form did not warn reviewers at the point
of submission. That left privacy as a downstream cleanup instead of an intake
instruction.

Rewrite decision:

- add a privacy warning to the reader-confusion issue form,
- ask reviewers not to include email addresses, private messages, personal
  notes, or contact details,
- update `scripts/check-reader-feedback-loop.py` so the warning remains
  mechanically required.

Validation:

```bash
python3 scripts/check-reader-feedback-loop.py
python3 -m py_compile scripts/check-reader-feedback-loop.py
rm -rf scripts/__pycache__
git diff --check -- . ':!target'
```

Result:

```text
Reader feedback loop check passed.
Python syntax check passed.
Diff whitespace check passed.
```

Remaining risk:

The issue form now warns reviewers before submission, but maintainers still
need the collector and inbox guards because public issue titles and bodies can
contain accidental private details.

## Sweep 2 Pass 121: GitHub Feedback Surface Self-Test

Files:

```text
scripts/check-github-feedback-surface.sh
scripts/check.sh
scripts/check-textbook-readiness.sh
scripts/check-completion-audit.py
book/TEXTBOOK_COMPLETION_AUDIT.md
book/CHAPTER_REWRITE_LOG.md
```

Primary sources:

- `scripts/check-github-feedback-surface.sh`
- `scripts/check.sh`
- `scripts/check-textbook-readiness.sh`

Critique before rewrite:

The live GitHub feedback-surface checker verified labels and the public reader
review sprint issue, but it had no offline self-test. If its parsing logic
became stale, the normal publication gate would not catch it without a live
GitHub call.

Rewrite decision:

- refactor `scripts/check-github-feedback-surface.sh` into label and issue
  validation functions,
- add a `--self-test` mode with positive and negative fixtures,
- wire the self-test into `scripts/check.sh`,
- add it to `scripts/check-textbook-readiness.sh`,
- make `scripts/check-completion-audit.py` require the self-test in the
  validation evidence and readiness-script coverage.

Validation:

```bash
bash -n scripts/check-github-feedback-surface.sh scripts/check-textbook-readiness.sh scripts/check.sh
bash scripts/check-github-feedback-surface.sh --self-test
python3 scripts/check-completion-audit.py
python3 -m py_compile scripts/check-completion-audit.py
rm -rf scripts/__pycache__
git diff --check -- . ':!target'
```

Result:

```text
GitHub feedback surface self-test passed.
Completion audit check passed.
Python syntax check passed.
Diff whitespace check passed.
```

Remaining risk:

The self-test verifies parser behavior and expected issue constraints offline.
The actual labels and issue state still require the live
`scripts/check-github-feedback-surface.sh` path.

## Sweep 2 Pass 122: Reader Feedback Status Self-Test

Files:

```text
scripts/reader-feedback-status.sh
scripts/check.sh
scripts/check-textbook-readiness.sh
scripts/check-completion-audit.py
book/TEXTBOOK_COMPLETION_AUDIT.md
book/CHAPTER_REWRITE_LOG.md
```

Primary sources:

- `scripts/reader-feedback-status.sh`
- `scripts/check-reader-feedback-inbox.py`
- `scripts/collect-reader-feedback-issues.py`
- `scripts/check-github-feedback-surface.sh`
- `scripts/check-reviewer-slot-tracker.py`

Critique before rewrite:

The lower-level reader-feedback scripts had self-tests, but the status wrapper
did not. That meant the command maintainers use for a combined reader-feedback
snapshot had no cheap offline verification path.

Rewrite decision:

- add `scripts/reader-feedback-status.sh --self-test`,
- have it run the inbox, issue collector, GitHub feedback surface, and reviewer
  slot tracker self-tests,
- wire it into `scripts/check.sh`,
- add it to `scripts/check-textbook-readiness.sh`,
- make the completion-audit checker require the command in validation evidence
  and readiness-script coverage.

Validation:

```bash
bash -n scripts/reader-feedback-status.sh scripts/check.sh scripts/check-textbook-readiness.sh
scripts/reader-feedback-status.sh --self-test
python3 scripts/check-completion-audit.py
python3 -m py_compile scripts/check-completion-audit.py
rm -rf scripts/__pycache__
git diff --check -- . ':!target'
```

Result:

```text
Reader feedback inbox self-test passed.
Reader feedback issue collector self-test passed.
GitHub feedback surface self-test passed.
Reviewer slot tracker self-test passed.
Reader feedback status self-test passed.
Completion audit check passed.
Python syntax check passed.
Diff whitespace check passed.
```

Remaining risk:

The status self-test is intentionally offline. `scripts/reader-feedback-status.sh
--live` remains necessary to inspect current GitHub labels and reader-confusion
issues.

## Sweep 2 Pass 123: Accepted Report State Consistency

Files:

```text
scripts/check-reader-feedback-inbox.py
community/reader-feedback-inbox.md
scripts/check-reader-feedback-loop.py
book/TEXTBOOK_COMPLETION_AUDIT.md
book/CHAPTER_REWRITE_LOG.md
```

Primary sources:

- `scripts/check-reader-feedback-inbox.py`
- `community/reader-feedback-inbox.md`
- `scripts/reader-feedback-status.sh`

Critique before rewrite:

The inbox validator treated a report as accepted if either its status or its
triage action was accepted. That meant a report with `Status: new` and
`Triage action: fix now` could count toward the direct-reader completion gate
even though the report was still marked as intake.

Rewrite decision:

- require both accepted `Status` and accepted `Triage action` for
  `ReportValidation.is_accepted`,
- reject reports where `Status: new`, `needs clarification`, or
  `closed out of scope` is paired with an accepted triage action,
- reject accepted statuses paired with non-accepted triage actions,
- add an inconsistent-state self-test fixture,
- document the accepted-evidence rule in the inbox.

Validation:

```bash
python3 scripts/check-reader-feedback-inbox.py --self-test
python3 scripts/check-reader-feedback-inbox.py
python3 scripts/check-reader-feedback-loop.py
scripts/reader-feedback-status.sh --self-test
python3 -m py_compile scripts/check-reader-feedback-inbox.py scripts/check-reader-feedback-loop.py
rm -rf scripts/__pycache__
git diff --check -- . ':!target'
```

Result:

```text
Reader feedback inbox self-test passed.
Reader feedback inbox check passed with no direct reports recorded.
Reader feedback sprint status: 0/5 accepted reports, 0 rewritten report(s).
Reader feedback loop check passed.
Reader feedback status self-test passed.
Python syntax check passed.
Diff whitespace check passed.
```

Remaining risk:

The accepted-state rule prevents local bookkeeping from overcounting untriaged
reports. It still depends on real reports arriving before the completion gate
can pass.

## Sweep 2 Pass 124: Transformer Category Diagnostic Guard

Files:

```text
scripts/check-public-friction-matrix.py
book/CHAPTER_QUALITY_SCORECARD.md
book/TEXTBOOK_COMPLETION_AUDIT.md
book/CHAPTER_REWRITE_LOG.md
```

Primary sources:

- [Attention Is All You Need](https://arxiv.org/abs/1706.03762)
- [Dive into Deep Learning: Attention Scoring Functions](https://d2l.ai/chapter_attention-mechanisms-and-transformers/attention-scoring-functions.html)
- [PyTorch scaled dot product attention](https://docs.pytorch.org/docs/stable/generated/torch.nn.functional.scaled_dot_product_attention.html)

Critique before rewrite:

The Transformer Roadmap already had a category-shape diagnostic that separates
ordinary morphisms, product-input morphisms, shape-preserving endomorphisms,
state endomorphisms, and illegal boundaries. The quality scorecard still marked
category precision as `Developing`, and the public-friction checker protected
the role-ownership map but not the category diagnostic itself. That made the
remaining gap look like missing authored content instead of missing direct
reader validation.

Rewrite decision:

- add a category-shape diagnostic guard to
  `scripts/check-public-friction-matrix.py`,
- require the diagnostic to preserve the two classifier questions, the
  product-input versus endomorphism distinction, and the illegal
  `HiddenSequence x MultiHeadOutput -> HiddenSequence` boundary,
- update the Transformer Roadmap scorecard row from `Developing` to
  `Reader-needed`,
- record that the category-shape diagnostic is checker-protected in the
  completion audit.

Validation:

```bash
python3 scripts/check-public-friction-matrix.py
python3 scripts/check-chapter-scorecard.py
python3 scripts/check-completion-audit.py
python3 scripts/check-rewrite-log.py
python3 -m py_compile scripts/check-public-friction-matrix.py scripts/check-chapter-scorecard.py scripts/check-completion-audit.py
rm -rf scripts/__pycache__
git diff --check -- . ':!target'
```

Result:

```text
Public friction matrix check passed.
Chapter scorecard check passed.
Completion audit check passed.
Rewrite log check passed.
Python syntax check passed.
Diff whitespace check passed.
```

Remaining risk:

The guard protects the diagnostic from disappearing, but the scorecard still
correctly requires a direct reader report before the category-precision claim
can become `Strong`.

## Sweep 2 Pass 125: Scorecard Evidence Semantics

Files:

```text
scripts/check-chapter-scorecard.py
scripts/check.sh
scripts/check-textbook-readiness.sh
scripts/check-completion-audit.py
book/TEXTBOOK_COMPLETION_AUDIT.md
book/CHAPTER_REWRITE_LOG.md
```

Primary sources:

- `scripts/check-reader-feedback-inbox.py`
- `community/reader-feedback-inbox.md`
- `book/CHAPTER_QUALITY_SCORECARD.md`

Critique before rewrite:

The scorecard checker guarded chapter rows and prevented direct-reader evidence
claims while the inbox was empty. But its accepted-report detector looked for
`Status: accepted`, while the inbox validator accepts consistent report states
such as `Status: fix now`, `Status: batched theme`, or `Status: rewritten`
paired with an accepted `Triage action`. That meant future valid reports could
still be treated as zero accepted evidence by the scorecard checker.

Rewrite decision:

- make `scripts/check-chapter-scorecard.py` import and reuse
  `scripts/check-reader-feedback-inbox.py`,
- count accepted reports through `ReportValidation.is_accepted`,
- surface inbox validation issues before trusting scorecard evidence,
- add a scorecard self-test proving `Status: fix now` counts when the inbox
  validator accepts it and an inconsistent `Status: new` fixture fails,
- wire the self-test into the publication gate, readiness gate, and completion
  audit.

Validation:

```bash
python3 scripts/check-chapter-scorecard.py
python3 scripts/check-chapter-scorecard.py --self-test
python3 scripts/check-completion-audit.py
python3 scripts/check-rewrite-log.py
python3 -m py_compile scripts/check-chapter-scorecard.py scripts/check-completion-audit.py
rm -rf scripts/__pycache__
git diff --check -- . ':!target'
```

Result:

```text
Chapter scorecard check passed.
Chapter scorecard self-test passed.
Completion audit check passed.
Rewrite log check passed.
Python syntax check passed.
Diff whitespace check passed.
```

Remaining risk:

The scorecard now shares accepted-report semantics with the inbox validator.
It still does not let chapters claim direct-reader evidence until real accepted
reports are recorded.

## Sweep 2 Pass 126: Per-Chapter Scorecard Evidence

Files:

```text
scripts/check-chapter-scorecard.py
book/TEXTBOOK_COMPLETION_AUDIT.md
book/CHAPTER_REWRITE_LOG.md
```

Primary sources:

- `scripts/check-reader-feedback-inbox.py`
- `community/reader-feedback-inbox.md`
- `book/CHAPTER_QUALITY_SCORECARD.md`

Critique before rewrite:

The scorecard checker now used the inbox validator for accepted-report
semantics, but it still treated accepted reader evidence as a global count. One
future accepted report could therefore make every chapter eligible to claim
direct-reader evidence if a row were edited incorrectly.

Rewrite decision:

- add chapter-specific evidence markers for every core chapter,
- count accepted and rewritten reports by matching accepted inbox reports
  against their `Location` and `Affected chapter section`,
- reject `Accepted` or `Rewritten` scorecard claims unless the affected chapter
  has matching accepted evidence,
- reject `Rewritten` claims unless the affected chapter also has a rewritten
  report,
- extend the scorecard self-test so a Tiny ML Pipeline fixture counts for that
  chapter but not for Welcome.

Validation:

```bash
python3 scripts/check-chapter-scorecard.py
python3 scripts/check-chapter-scorecard.py --self-test
python3 -m py_compile scripts/check-chapter-scorecard.py
rm -rf scripts/__pycache__
git diff --check -- . ':!target'
```

Result:

```text
Chapter scorecard check passed.
Chapter scorecard self-test passed.
Python syntax check passed.
Diff whitespace check passed.
```

Remaining risk:

The matcher is intentionally conservative and depends on accepted reports
naming the affected chapter or file. That is compatible with the inbox
template, but vague reports still need clarification before they can support a
scorecard change.

## Sweep 2 Pass 127: Scorecard Claim Self-Test

Files:

```text
scripts/check-chapter-scorecard.py
book/CHAPTER_REWRITE_LOG.md
```

Primary sources:

- `scripts/check-reader-feedback-inbox.py`
- `book/CHAPTER_QUALITY_SCORECARD.md`

Critique before rewrite:

The per-chapter evidence matcher computed accepted and rewritten report counts
by chapter. Its self-test proved the counts were correct, but it did not yet
prove that the actual scorecard row validator rejected a wrong chapter claim.
That left a weaker verifier surface: the helper could work while the row-level
claim check regressed later.

Rewrite decision:

- add fixture scorecard and contract builders to the scorecard checker
  self-test,
- prove a Tiny ML Pipeline accepted report allows only the Tiny ML Pipeline row
  to claim `Accepted`,
- prove a Welcome `Accepted` claim fails when the accepted report is for the
  Tiny ML Pipeline,
- prove a `Rewritten` claim fails when the accepted report has not reached
  `Status: rewritten`.

Validation:

```bash
python3 scripts/check-chapter-scorecard.py --self-test
```

Result:

```text
Chapter scorecard self-test passed.
```

Remaining risk:

The self-test covers scorecard claim semantics offline. Real evidence still
depends on accepted reader reports being recorded with concrete chapter
locations.

## Sweep 2 Pass 132: Structure Chapter Maturity Promotion

Files:

```text
README.md
book/CHAPTER_CONTRACTS.md
book/TEXTBOOK_COMPLETION_AUDIT.md
book/CHAPTER_REWRITE_LOG.md
```

Primary sources:

- `book/CHAPTER_QUALITY_SCORECARD.md`
- `book/CHAPTER_CONTRACTS.md`
- `scripts/check-chapter-maturity.py`
- `scripts/check-chapter-contracts.py`

Critique before rewrite:

The public maturity table still labeled the structure chapter and Seven
Sketches chapter as `Sketch`. That was stale relative to the current evidence:
both chapters now have source grounding, runnable examples, code evidence,
practice evidence, answer-key support, and `Strong` scorecard rows across
non-reader dimensions. Keeping them as `Sketch` understated their actual
public maturity and made the remaining blocker less precise.

Rewrite decision:

- promote `Functors, Naturality, Monoids, and Chain Rule` from `Sketch` to
  `Draft`,
- promote `Seven Sketches Through Rust` from `Sketch` to `Draft`,
- keep `Transformer Roadmap` as `Sketch` because its category-precision row is
  still `Reader-needed`,
- keep all direct-reader evidence cells as `Pending`,
- update the chapter contract manifest and completion audit to match the public
  maturity table.

Validation:

```bash
python3 scripts/check-chapter-maturity.py
python3 scripts/check-chapter-contracts.py
python3 scripts/check-completion-audit.py
python3 scripts/check-chapter-scorecard.py
```

Result:

```text
Chapter maturity check passed.
Chapter contract check passed.
Completion audit check passed.
Chapter scorecard check passed.
```

Remaining risk:

`Draft` means readable and evolving, not complete. External reader evidence is
still required before any chapter can claim direct-reader validation.

## Sweep 2 Pass 131: Hugging Face Architecture Boundary Sources

Files:

```text
book/src/references.md
book/EDITORIAL_RESEARCH_NOTES.md
book/src/roadmap.md
scripts/check-source-authority.py
book/CHAPTER_REWRITE_LOG.md
book/TEXTBOOK_COMPLETION_AUDIT.md
```

Primary sources:

- [Hugging Face Course: How do Transformers work?](https://huggingface.co/docs/course/en/chapter1/4)
- [Hugging Face Transformers: Model outputs](https://huggingface.co/docs/transformers/main_classes/output)

Critique before rewrite:

The Transformer reference surface now included PyTorch framework shape
references, but it did not yet include a current practitioner source for the
architecture, checkpoint, and model-output distinction. That distinction
matters pedagogically: this project builds tiny architecture pieces, not a
pretrained checkpoint or a wrapper over a framework output container.

Rewrite decision:

- add the Hugging Face Transformer course chapter to the Transformer reference
  map and source bucket,
- add the Hugging Face model-output documentation as an API-shape source for
  hidden states, attentions, and output containers,
- add a roadmap paragraph distinguishing architecture pieces from pretrained
  checkpoints and large framework outputs,
- strengthen the source-authority checker so Hugging Face documentation stays
  represented,
- rerun the optional live link checker over the reference and research files.

Validation:

```bash
python3 scripts/check-source-authority.py
python3 scripts/check-chapter-references.py
python3 scripts/check-public-friction-matrix.py
python3 scripts/check-prose-style.py
python3 -m py_compile scripts/check-source-authority.py
python3 scripts/check-reference-links-live.py --timeout 10 book/src/references.md book/EDITORIAL_RESEARCH_NOTES.md
```

Result:

```text
Source authority check passed.
Chapter reference coverage check passed.
Public friction matrix check passed.
Prose style check passed.
Python syntax check passed.
Reference link live check passed: 104 source URL(s) reachable.
```

Remaining risk:

These sources improve the architecture boundary, but they are still source
evidence rather than direct reader evidence. The strict reader-feedback gate
remains open.

## Sweep 2 Pass 130: Live Framework Reference Refresh

Files:

```text
book/src/references.md
book/EDITORIAL_RESEARCH_NOTES.md
book/src/roadmap.md
scripts/check-source-authority.py
book/CHAPTER_REWRITE_LOG.md
book/TEXTBOOK_COMPLETION_AUDIT.md
```

Primary sources:

- [PyTorch MultiheadAttention](https://docs.pytorch.org/docs/stable/generated/torch.nn.MultiheadAttention.html)
- [PyTorch scaled_dot_product_attention](https://docs.pytorch.org/docs/stable/generated/torch.nn.functional.scaled_dot_product_attention.html)
- [PyTorch TransformerEncoderLayer](https://docs.pytorch.org/docs/stable/generated/torch.nn.TransformerEncoderLayer.html)

Critique before rewrite:

The Transformer references were strong, but one PyTorch attention link was
pinned to a specific `/docs/2.11/` path. That can age poorly in a public
textbook. The roadmap also used framework documentation as an implicit sanity
check without naming that boundary: framework docs should verify API shapes,
not become the book's teaching target.

Rewrite decision:

- replace the pinned PyTorch `scaled_dot_product_attention` URL with the
  stable docs URL,
- add official PyTorch `TransformerEncoderLayer` documentation to the
  Transformer reference map and research notes,
- add a short roadmap paragraph that explains framework docs as API-shape
  checks rather than the target abstraction,
- strengthen the source-authority checker so official PyTorch stable docs stay
  represented in the reference surface,
- run the optional live link checker over the reference and research files.

Validation:

```bash
python3 scripts/check-source-authority.py
python3 scripts/check-chapter-references.py
python3 scripts/check-public-friction-matrix.py
python3 scripts/check-prose-style.py
python3 scripts/check-reference-links-live.py --timeout 10 book/src/references.md book/EDITORIAL_RESEARCH_NOTES.md
```

Result:

```text
Source authority check passed.
Chapter reference coverage check passed.
Public friction matrix check passed.
Prose style check passed.
Reference link live check passed: 100 source URL(s) reachable.
```

Remaining risk:

Live external checks are useful source-refresh evidence, but they are not part
of the offline publication gate because external sites can rate-limit or block
automated access. The strict textbook completion blocker remains direct reader
evidence.

## Sweep 2 Pass 129: Transformer Category Naming Rule

Files:

```text
book/src/roadmap.md
book/src/exercises.md
exercises/ANSWER_KEY.md
scripts/check-public-friction-matrix.py
book/CHAPTER_REWRITE_LOG.md
book/TEXTBOOK_COMPLETION_AUDIT.md
```

Primary sources:

- [Seven Sketches in Compositionality](https://arxiv.org/abs/1803.05316)
- [Attention Is All You Need](https://arxiv.org/abs/1706.03762)
- [D2L Attention and Transformers](https://d2l.ai/chapter_attention-mechanisms-and-transformers/index.html)

Critique before rewrite:

The Transformer Roadmap already had a category-shape diagnostic that separated
ordinary morphisms, product-input morphisms, shape-preserving endomorphisms,
state endomorphisms, and illegal boundaries. The weakness was procedural: a
reader saw the example table before receiving a small naming algorithm. That
could still let them memorize classifications instead of learning the first
move: count the inputs before deciding whether an arrow is an endomorphism.

Rewrite decision:

- add a naming rule before the Transformer category-shape table,
- state the five safe cases: `A -> B`, `A -> A`, `A x B -> C`, `A x B -> A`,
  and illegal `A x B -> A` with the wrong second object,
- make the trap explicit: returning the left object is not enough to make a
  product-input boundary an endomorphism,
- extend the exercise prompt so learners count inputs before naming shapes,
- extend the answer key with the same rule,
- strengthen `scripts/check-public-friction-matrix.py` so this procedure stays
  present.

Validation:

```bash
python3 scripts/check-public-friction-matrix.py
python3 scripts/check-exercise-alignment.py
python3 scripts/check-prose-style.py
python3 -m py_compile scripts/check-public-friction-matrix.py
```

Result:

```text
Public friction matrix check passed.
Exercise alignment check passed.
Prose style check passed.
Python syntax check passed.
```

Remaining risk:

This makes the diagnostic more teachable, but the scorecard should keep the
Transformer Roadmap category-precision cell at `Reader-needed` until an
external reader runs example 06 and reports whether the distinction works.

## Sweep 2 Pass 128: Public Book Review Path Guard

Files:

```text
community/reader-review-packet.md
community/reviewer-outreach.md
community/reader-review-sprint.md
scripts/check-reader-feedback-loop.py
book/TEXTBOOK_COMPLETION_AUDIT.md
```

Primary sources:

- `community/reader-feedback-triage.md`
- `community/reader-feedback-inbox.md`
- `scripts/check-reader-feedback-inbox.py`

Critique before rewrite:

The reader-review loop was strict about accepted evidence, but the short review
packet still assumed a reviewer would usually clone the repository or read
local file paths. That is useful for code reviewers, but it leaves a weaker
external-review surface for readers who start from the public book and GitHub
page. The completion blocker is direct reader evidence, so the review path
should not accidentally require local setup before a reader can report a
precise unclear section.

Rewrite decision:

- add the public book URL and GitHub repository URL to the short review packet,
- add a `Path D: Public Book Review` route for no-clone reviewers,
- define when a public-book report still counts as direct reader evidence,
- include the public book URL in outreach and sprint instructions,
- strengthen `scripts/check-reader-feedback-loop.py` so the public-book path,
  URLs, and evidence wording remain present.

Validation:

```bash
python3 scripts/check-reader-feedback-loop.py
python3 -m py_compile scripts/check-reader-feedback-loop.py
```

Result:

```text
Reader feedback loop check passed.
Python syntax check passed.
```

Remaining risk:

This pass improves the intake path but does not create evidence. The textbook
completion gate still requires five accepted direct-reader reports and at
least one rewritten report.

## Sweep 2 Pass 114: Textbook Completion Readiness Gate

Files:

```text
scripts/check-textbook-readiness.sh
scripts/check-completion-audit.py
book/TEXTBOOK_COMPLETION_AUDIT.md
book/CHAPTER_REWRITE_LOG.md
```

Primary sources:

- `scripts/check.sh`
- `scripts/check-reader-feedback-inbox.py`
- `scripts/check-completion-audit.py`
- `book/TEXTBOOK_COMPLETION_AUDIT.md`

Critique before rewrite:

The audit listed the strict reader-feedback gate, but there was no single
command whose purpose was "prove that the textbook goal is ready to close."
That left too much room for treating the ordinary publication gate as if it
were the same thing as completion.

ML teaching critique:

A world-class technical textbook needs learner evidence, not only working
examples. The readiness gate should keep the missing reader reports visible
even when all Rust and book checks pass.

Category-theory critique:

The project's own completion logic should preserve the distinction between
coherent local structure and external validation. Passing local laws and
contracts is necessary evidence, but it is not the same object as direct reader
evidence.

Learner critique:

Future maintainers need one command that fails honestly until the reader sprint
has produced accepted reports and at least one rewritten report. Otherwise a
long green validation transcript can hide the actual remaining blocker.

Rewrite decision:

- add `scripts/check-textbook-readiness.sh` as an optional completion gate,
- make it run the completion audit, feedback-loop checks, strict direct-reader
  gate, chapter/rewrite/reference checks, wording guards, and whitespace guard,
- keep it out of `scripts/check.sh` because it must currently fail,
- update the completion audit and audit checker so the readiness command is
  part of the evidence map.

Validation:

```bash
python3 scripts/check-completion-audit.py
python3 scripts/check-rewrite-log.py
scripts/check-textbook-readiness.sh
bash scripts/check.sh
```

The readiness script failed for the intended reason: the strict direct-reader
completion gate is still open. The normal full gate passed.

## Sweep 2 Pass 113: Reader Feedback Loop Category-Shape Guard

Files:

```text
scripts/check-reader-feedback-loop.py
book/TEXTBOOK_COMPLETION_AUDIT.md
book/CHAPTER_REWRITE_LOG.md
```

Primary sources:

- [Seven Sketches in Compositionality](https://arxiv.org/abs/1803.05316)
- [MIT OpenCourseWare: Applied Category Theory](https://ocw.mit.edu/courses/18-s097-applied-category-theory-january-iap-2019/)
- [Categories for the Working Mathematician](https://link.springer.com/book/10.1007/978-1-4757-4721-8)

Critique before rewrite:

The reader sprint had a category-shape review path, but the guard script only
checked general feedback-loop connectivity. A future edit could remove the
newest product-input versus endomorphism prompt while the loop checker still
passed.

ML teaching critique:

The feedback loop should preserve prompts that ask about concrete attention
boundaries, not just broad chapter opinions. Otherwise the next reader report
may say "attention is hard" without identifying whether the blocked step is
scores, masks, weights, values, projection, residual state, or training state.

Category-theory critique:

The highest-risk terminology distinction is arity plus return object. The
checker should protect prompts that ask reviewers to classify product-input
morphisms, ordinary morphisms, shape-preserving endomorphisms, state
endomorphisms, and illegal attempted boundaries.

Learner critique:

Public review files are part of the learning product. If they stop asking for
one concrete boundary, reviewers have to infer what kind of report is useful.
The checker should keep the review task small and specific.

Rewrite decision:

- add `REQUIRED_CATEGORY_SHAPE_REVIEW_MARKERS` to
  `scripts/check-reader-feedback-loop.py`,
- require category-shape markers in the review packet, review guide, sprint
  plan, outreach ask, and completion audit,
- update the completion audit to record that category-shape review markers are
  mechanically protected.

Validation:

```bash
python3 scripts/check-reader-feedback-loop.py
python3 scripts/check-completion-audit.py
python3 scripts/check-rewrite-log.py
bash scripts/check.sh
```

The feedback-loop guard now fails if the category-shape review path disappears
from the public reviewer surfaces. The direct-reader sprint gate remains open
until accepted human reports arrive.

## Sweep 2 Pass 112: Reader Sprint Category-Shape Review Path

Files:

```text
community/reader-review-packet.md
community/reader-review-guide.md
community/reader-review-sprint.md
community/reviewer-outreach.md
book/TEXTBOOK_COMPLETION_AUDIT.md
book/CHAPTER_REWRITE_LOG.md
```

Primary sources:

- [Seven Sketches in Compositionality](https://arxiv.org/abs/1803.05316)
- [MIT OpenCourseWare: Applied Category Theory](https://ocw.mit.edu/courses/18-s097-applied-category-theory-january-iap-2019/)
- [Categories for the Working Mathematician](https://link.springer.com/book/10.1007/978-1-4757-4721-8)

Critique before rewrite:

The book now had a category-shape diagnostic for the Transformer Roadmap, but
the external reader sprint still asked reviewers mostly for broad chapter-level
feedback. That made the next direct-reader report less likely to test the
newest high-risk distinction: product-input morphism versus endomorphism.

ML teaching critique:

For attention, a useful reader report should name the exact boundary where role
or input separation failed. A broad complaint that attention is hard does not
tell the author whether the reader lost scores, masks, weights, values,
projection, residual state, or training-state updates.

Category-theory critique:

The review prompt should ask category-theory readers to classify one concrete
boundary before judging terminology. That keeps precision grounded in the Rust
artifact and avoids a generic preference for more or less formal language.

Learner critique:

Reviewers are more helpful when the task is small. Asking for one unclear
boundary after running example 06 is more likely to produce actionable evidence
than asking for a full review of the Transformer chapter.

Rewrite decision:

- add a category-shape classification task to the reader review packet,
- fix the attention review guide's boundary order so masking appears before
  row-wise normalization,
- point category-theory outreach at the roadmap diagnostic as an optional path,
- update the sprint plan so Day 1 can ask for one category-shape friction
  report,
- update the completion audit evidence without changing the direct-reader
  status.

Validation:

```bash
python3 scripts/check-reader-feedback-loop.py
python3 scripts/check-completion-audit.py
python3 scripts/check-prose-style.py
python3 scripts/check-rewrite-log.py
scripts/reader-feedback-status.sh --live
bash scripts/check.sh
```

Focused checks and the full gate passed. The strict reader sprint gate remains
open because no accepted direct-reader reports have arrived.

## Sweep 2 Pass 111: Transformer Roadmap Category-Shape Diagnostic

Files:

```text
book/src/roadmap.md
book/src/exercises.md
exercises/ANSWER_KEY.md
book/src/references.md
book/EDITORIAL_RESEARCH_NOTES.md
book/CHAPTER_CONTRACTS.md
book/CHAPTER_QUALITY_SCORECARD.md
book/TEXTBOOK_COMPLETION_AUDIT.md
book/CHAPTER_REWRITE_LOG.md
```

Primary sources:

- [Seven Sketches in Compositionality](https://arxiv.org/abs/1803.05316)
- [MIT OpenCourseWare: Applied Category Theory](https://ocw.mit.edu/courses/18-s097-applied-category-theory-january-iap-2019/)
- [Categories for the Working Mathematician](https://link.springer.com/book/10.1007/978-1-4757-4721-8)
- [Attention Is All You Need](https://arxiv.org/abs/1706.03762)
- [Dive into Deep Learning: Multi-Head Attention](https://d2l.ai/chapter_attention-mechanisms-and-transformers/multihead-attention.html)

Critique before rewrite:

The Transformer Roadmap now had a terminal-output checkpoint map and an
output-transfer checklist. The remaining category-precision weakness was that
several boundaries returned to a familiar object while using different arities.
A reader could still call every `HiddenSequence`-returning or
`AttentionScores`-returning line an endomorphism.

ML teaching critique:

Attention needs role separation, but it also needs input separation. Query-key
scoring, masking, and value mixing require different evidence at different
stages. If a reader compresses those stages into "attention updates attention,"
they miss why masks, values, projections, and residuals have different jobs.

Category-theory critique:

The chapter needed a compact diagnostic for arity and return object:
`A -> B`, `A x B -> C`, `A -> A`, and illegal attempted composition are
different shapes. Returning the same public object is not enough to call a
boundary a unary endomorphism when the boundary also consumes a product input.

Learner critique:

After running example 06, a motivated reader can name shapes from output, but
may still overgeneralize the category words. The chapter should ask them to
classify concrete boundaries before retrieval practice, and the answer key
should reject imprecise "everything is an endomorphism" answers.

Rewrite decision:

- add a `Category Shape Diagnostic` to the Transformer Roadmap,
- classify key attention and training boundaries as product-input morphisms,
  ordinary morphisms, shape-preserving endomorphisms, state endomorphisms, or
  illegal boundaries,
- expand Exercise 12 and its answer key so learners classify at least one
  product-input morphism, one endomorphism, and one illegal boundary,
- add category-theory precision sources to the reference and editorial-source
  maps,
- keep direct-reader evidence Pending and keep the scorecard focus on
  validating the diagnostic with readers.

Validation:

```bash
python3 scripts/check-prose-style.py
python3 scripts/check-chapter-references.py
python3 scripts/check-source-authority.py
python3 scripts/check-chapter-contracts.py
python3 scripts/check-chapter-scorecard.py
python3 scripts/check-completion-audit.py
python3 scripts/check-rewrite-log.py
python3 scripts/check-exercise-alignment.py
cargo run --example 06_attention_scores
mdbook test
bash scripts/check.sh
```

Focused checks and the full gate passed. The direct-reader completion gate
remains intentionally blocked until accepted project-specific reader reports
arrive.

## Sweep 2 Pass 110: Transformer Roadmap Terminal Output Checkpoint Map

Files:

```text
book/src/roadmap.md
book/CHAPTER_CONTRACTS.md
book/CHAPTER_QUALITY_SCORECARD.md
book/TEXTBOOK_COMPLETION_AUDIT.md
book/CHAPTER_REWRITE_LOG.md
```

Primary sources:

- [Attention Is All You Need](https://arxiv.org/abs/1706.03762)
- [Dive into Deep Learning: Attention Mechanisms and Transformers](https://d2l.ai/chapter_attention-mechanisms-and-transformers/index.html)
- [Dive into Deep Learning: Queries, Keys, and Values](https://d2l.ai/chapter_attention-mechanisms-and-transformers/queries-keys-values.html)
- [Dive into Deep Learning: Multi-Head Attention](https://d2l.ai/chapter_attention-mechanisms-and-transformers/multihead-attention.html)
- [Dive into Deep Learning: The Transformer Architecture](https://d2l.ai/chapter_attention-mechanisms-and-transformers/transformer.html)
- [The Annotated Transformer](https://nlp.seas.harvard.edu/annotated-transformer/)
- [The Illustrated Transformer](https://jalammar.github.io/illustrated-transformer/)
- `src/attention.rs`
- `examples/06_attention_scores.rs`

Critique before rewrite:

The Transformer Roadmap chapter had strong role ownership, worked examples for
masking, query/key/value roles, multi-head projection, residual shape,
shape-preserving sublayers, and structured training state. Its remaining
scorecard weakness was the long terminal output from example 06: the chapter
mapped typed transformation lines, but it did not yet teach readers how to
group the earlier numeric output checkpoints.

ML teaching critique:

Transformer learners often compress every vector into the word "attention."
The example output should make them separate where-to-read weights, what-to-read
value mixing, return-to-hidden-state projection/residual structure, and
parameter updates.

Category-theory critique:

The typed transformation list is useful, but the numeric checkpoints are also
boundary evidence. Each printed shape line should say what changed, what stayed
true, and which typed boundary is being protected.

Learner critique:

After running `cargo run --example 06_attention_scores`, a reader sees many
lines before the final typed transformation list. The chapter should provide a
checkpoint map so the reader can pause at each output block and explain the ML
role and categorical shape before moving on.

Rewrite decision:

- add a `Terminal Output Checkpoint Map` before the existing output-transfer
  checklist,
- map attention shape, weights, value output, multi-head width, projection,
  residual, normalization/feed-forward, logits, state-step, and loss-change
  lines to their protected boundaries,
- update the chapter contract, scorecard, and completion audit so the
  checkpoint map is tracked as evidence.

Validation:

```bash
cargo run --example 06_attention_scores
python3 scripts/check-prose-style.py
python3 scripts/check-chapter-contracts.py
python3 scripts/check-chapter-scorecard.py
python3 scripts/check-completion-audit.py
python3 scripts/check-rewrite-log.py
python3 scripts/check-duplicate-prose.py
python3 scripts/check-exercise-alignment.py
git diff --check -- . ':!target'
mdbook test
bash scripts/check.sh
```

All focused checks passed. The roadmap now maps the long terminal output into
checkpoints before the typed transformation list, while the scorecard keeps the
advanced category-shape precision gap open for direct reader feedback.

## Sweep 2 Pass 109: Exercises Mixed Boundary Worked Diagnosis

Files:

```text
book/src/exercises.md
exercises/ANSWER_KEY.md
book/CHAPTER_CONTRACTS.md
book/CHAPTER_QUALITY_SCORECARD.md
book/TEXTBOOK_COMPLETION_AUDIT.md
book/CHAPTER_REWRITE_LOG.md
```

Primary sources:

- [How People Learn II: Knowledge and Reasoning](https://www.nationalacademies.org/read/24783/chapter/7)
- [Rust Book: Writing Automated Tests](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Rustlings](https://rustlings.rust-lang.org/)
- [100 Exercises To Learn Rust](https://rust-exercises.com/100-exercises/01_intro/00_welcome)
- [Dive into Deep Learning: Softmax Regression](https://d2l.ai/chapter_linear-classification/softmax-regression.html)
- `book/src/exercises.md`
- `exercises/ANSWER_KEY.md`

Critique before rewrite:

The Exercises chapter already had a ladder, practice map, failure-signal table,
evidence map, attempt-record template, and mixed boundary diagnosis. The
remaining weakness was that Exercise 15 asked readers to classify seven
interleaved failures without first showing one complete diagnosis at the
required level of ML and category-theory precision.

ML teaching critique:

The most fragile case is `CrossEntropy` receiving `Logits`. A learner can say
"wrong type" while still missing the supervised ML idea: cross-entropy needs
the probability assigned to the target token, not just a vector of scores.

Category-theory critique:

The same case also tests product-object precision. The legal route is
`Logits -> Distribution`, followed by
`Distribution x TokenId -> Loss`. A strong exercise surface should make the
missing product input visible.

Learner critique:

Readers need one model answer before the interleaved exercise. The worked
diagnosis should show the exact answer shape: boundary type, Rust syntax, ML
concept, category-theory concept, and smallest useful fix.

Rewrite decision:

- add a `Worked Example: Mixed Boundary Diagnosis` before the attempt-record
  template,
- use the `CrossEntropy`/`Logits` confusion as the full worked case,
- sharpen the answer key so case 3 is classified as both a composition
  boundary and a product-input boundary,
- update the chapter contract, scorecard, and completion audit so the worked
  diagnosis is tracked as practice evidence.

Validation:

```bash
python3 scripts/check-prose-style.py
python3 scripts/check-chapter-contracts.py
python3 scripts/check-chapter-scorecard.py
python3 scripts/check-completion-audit.py
python3 scripts/check-exercise-alignment.py
python3 scripts/check-exercise-commands.py
python3 scripts/check-duplicate-prose.py
python3 scripts/check-rewrite-log.py
git diff --check -- . ':!target'
mdbook test
bash scripts/check.sh
```

All focused checks passed. The Exercises chapter now includes one full mixed
boundary diagnosis before the interleaved exercise, the answer key sharpens the
CrossEntropy/logits case, and book chapter tests passed.

## Sweep 2 Pass 108: Morphism Stage-Output Transfer Trace

Files:

```text
book/src/02-morphisms-composition.md
examples/02_morphism_composition.rs
lessons/02-morphisms-composition.md
README.md
book/CHAPTER_CONTRACTS.md
book/CHAPTER_QUALITY_SCORECARD.md
book/CHAPTER_REWRITE_LOG.md
```

Primary sources:

- [Rust Book: Defining Shared Behavior with Traits](https://doc.rust-lang.org/book/ch10-02-traits.html)
- [Rust Book: Generic Data Types](https://doc.rust-lang.org/book/ch10-01-syntax.html)
- [Dive into Deep Learning: Softmax Regression](https://d2l.ai/chapter_linear-classification/softmax-regression.html)
- [Seven Sketches in Compositionality](https://arxiv.org/abs/1803.05316)
- [Category Theory for Programming](https://arxiv.org/abs/2209.01259)
- `src/category.rs`
- `src/ml.rs`
- `examples/02_morphism_composition.rs`

Critique before rewrite:

The Morphism and Composition chapter had a strong term-to-code bridge, broken
pipeline example, retrieval practice, and output-to-boundary checklist. The
scorecard still marked ML intuition and category precision as Developing
because the runnable output skipped from final probabilities to typed arrows
without displaying the intermediate values those arrows produced.

ML teaching critique:

Learners need to see `Vector`, `Logits`, and `Distribution` as different ML
objects, not as interchangeable float lists. The example should print hidden
features, vocabulary scores, and normalized probabilities in sequence.

Category-theory critique:

The composite arrow `TokenId -> Distribution` is useful only if readers
remember the legal route through `Vector` and `Logits`. Composition summarizes
the route; it does not erase the middle objects that make the route legal.

Learner critique:

A reader could run the old example and focus only on the probability numbers.
The new trace should make them explain which printed object is the source,
which are middle objects, which is the target, and which shortcut each boundary
rejects.

Rewrite decision:

- make `examples/02_morphism_composition.rs` print the input object, stage
  outputs, composed morphism, final probabilities, and visible middle objects,
- expand the chapter checklist into Rust, ML, and category-theory readings for
  each printed line,
- update the compact lesson and README example so public entry points do not
  show stale composition output,
- update the contract and scorecard so the stage-output transfer checklist is
  tracked as the practice evidence.

Validation:

```bash
cargo run --example 02_morphism_composition
python3 scripts/check-prose-style.py
python3 scripts/check-chapter-contracts.py
python3 scripts/check-chapter-scorecard.py
python3 scripts/check-rewrite-log.py
python3 scripts/check-duplicate-prose.py
python3 scripts/check-exercise-commands.py
git diff --check -- . ':!target'
mdbook test
bash scripts/check.sh
```

All focused checks passed. The example now prints stage outputs before the
composed morphism, the chapter checklist maps each output line to Rust, ML, and
category-theory readings, and book chapter tests passed.

## Sweep 2 Pass 107: Domain Objects Example Output Transfer Checklist

Files:

```text
book/src/01-domain-objects.md
examples/01_domain_objects.rs
book/CHAPTER_CONTRACTS.md
book/CHAPTER_QUALITY_SCORECARD.md
book/CHAPTER_REWRITE_LOG.md
```

Primary sources:

- [Rust Book: Defining and Instantiating Structs](https://doc.rust-lang.org/book/ch05-01-defining-structs.html)
- [Rust Book: Recoverable Errors with Result](https://doc.rust-lang.org/stable/book/ch09-02-recoverable-errors-with-result.html)
- [Rust API Guidelines Checklist](https://rust-lang.github.io/api-guidelines/checklist.html)
- [rustdoc: How to write documentation](https://doc.rust-lang.org/rustdoc/how-to-write-documentation.html)
- `src/domain.rs`
- `examples/01_domain_objects.rs`

Critique before rewrite:

The Domain Objects chapter already explained semantic wrappers, validated
objects, mistake-prevention, and constructor boundaries. Its companion example,
however, printed only raw numeric pairs, which weakened the chapter's claim
that display should remind readers where raw values became domain objects.

ML teaching critique:

Learners need to see that tokenized data and adjacent input-target examples
are separate stages. The output should name `TokenSequence` and `TrainingSet`
before the chapter moves into later ML transformations.

Category-theory critique:

The output should make object and product boundaries visible:
`TokenSequence`, `TrainingSet`, `TokenId x TokenId`, and the partial
constructor path from raw collections into validated objects.

Learner critique:

A reader could run the old example and remember only `1 -> 2`. The new
checklist should force them to say which type boundary each printed line proves
and which shortcut it rejects.

Rewrite decision:

- make `examples/01_domain_objects.rs` print named domain objects and typed
  boundaries,
- add an example-output transfer checklist to the chapter,
- update the chapter contract and scorecard so the checklist is tracked as
  practice evidence.

Validation:

```bash
cargo run --example 01_domain_objects
python3 scripts/check-prose-style.py
python3 scripts/check-chapter-contracts.py
python3 scripts/check-chapter-scorecard.py
python3 scripts/check-rewrite-log.py
python3 scripts/check-duplicate-prose.py
python3 scripts/check-exercise-commands.py
git diff --check -- . ':!target'
mdbook test
bash scripts/check.sh
```

All focused checks passed. The example now prints named domain objects and
constructor/product boundaries, and book chapter tests passed.

## Sweep 2 Pass 106: Course Map Demo Output Wayfinding Checklist

Files:

```text
book/src/00-map.md
book/CHAPTER_CONTRACTS.md
book/CHAPTER_QUALITY_SCORECARD.md
book/CHAPTER_REWRITE_LOG.md
```

Primary sources:

- [Rust Book: Packages and Crates](https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html)
- [Seven Sketches in Compositionality](https://arxiv.org/abs/1803.05316)
- [Category Theory for Programming](https://arxiv.org/abs/2209.01259)
- `src/demo.rs`
- `src/bin/category_ml.rs`

Critique before rewrite:

The Course Map chapter already gave book-first and code-first paths and a
strong module map. Its remaining weakness was wayfinding after the full demo.
A reader could run `cargo run --bin category_ml`, see eleven numbered
sections, and still not know which file or chapter to inspect next.

ML teaching critique:

The demo should be read as a sequence from data preparation to prediction,
loss, training, reusable structure, and local derivatives. Without a routing
table, the ML story can feel like many unrelated terms printed in one command.

Category-theory critique:

Each numbered demo section introduces a categorical shape: object, morphism,
identity, composition, product, endomorphism, functor, naturality, monoid,
commutative diagram, or chain rule. The chapter should make that mapping
explicit before later chapters zoom in.

Learner critique:

First-session readers need a next action for every confusing output line. The
checklist should map output line to source file, Rust reading, ML reading, and
category-theory reading.

Rewrite decision:

- add a demo-output wayfinding checklist after the demo explanation,
- map each numbered demo section to the next source file to inspect,
- connect each output section to Rust, ML, and category-theory readings,
- update the chapter contract and scorecard so the checklist is tracked as
  practice evidence.

Validation:

```bash
cargo run --bin category_ml
python3 scripts/check-prose-style.py
python3 scripts/check-chapter-contracts.py
python3 scripts/check-chapter-scorecard.py
python3 scripts/check-rewrite-log.py
bash scripts/check.sh
```

All focused checks passed. The demo output maps cleanly to the new wayfinding
checklist, and book chapter tests passed.

## Sweep 2 Pass 105: Welcome First Output Transfer Checklist

Files:

```text
book/src/welcome.md
book/CHAPTER_CONTRACTS.md
book/CHAPTER_QUALITY_SCORECARD.md
book/CHAPTER_REWRITE_LOG.md
```

Primary sources:

- [How People Learn II](https://nap.nationalacademies.org/read/24783/chapter/7)
- [Rust By Example](https://doc.rust-lang.org/rust-by-example/index.html)
- [Seven Sketches in Compositionality](https://arxiv.org/abs/1803.05316)
- `examples/01_token_sequence.rs`
- `src/domain.rs`

Critique before rewrite:

The Welcome chapter already gave a strong first command and a clear
three-lens reading contract. Its remaining weakness was transfer from the
first terminal output. A beginner-adjacent reader could run the command and
still treat the output as a banner rather than evidence for Rust, ML, and
category-theory boundaries.

ML teaching critique:

The first run should show why raw text, tokenized data, and adjacent
input-target examples are different stages. The ML idea should appear before
the vocabulary becomes heavier.

Category-theory critique:

The chapter should not ask for abstract words too early. It should first make
the learner point at a source object, a named object, product-shaped examples,
and a visible transformation path.

Learner critique:

The first-session success condition needs to be concrete: run the example,
choose one printed block, and explain what boundary it proves. Without that
check, the first win is runnable but not yet diagnostic.

Rewrite decision:

- add a first-output transfer checklist after the first command,
- map `Raw input`, `TokenSequence`, `TrainingPairs`, and `Typed
  transformation` to the Rust, ML, and category-theory readings,
- update the chapter contract and scorecard so the checklist is tracked as
  practice evidence.

Validation:

```bash
cargo run --example 01_token_sequence
python3 scripts/check-prose-style.py
python3 scripts/check-chapter-contracts.py
python3 scripts/check-chapter-scorecard.py
python3 scripts/check-rewrite-log.py
bash scripts/check.sh
```

All focused checks passed. The first example output matches the new transfer
checklist, and book chapter tests passed.

## Sweep 2 Pass 104: Seven Sketches Example Output Transfer Checklist

Files:

```text
book/src/seven-sketches-rust.md
examples/05_seven_sketches.rs
book/CHAPTER_CONTRACTS.md
book/CHAPTER_QUALITY_SCORECARD.md
book/CHAPTER_REWRITE_LOG.md
```

Primary sources:

- [Seven Sketches in Compositionality](https://arxiv.org/abs/1803.05316)
- [Category Theory for Programming](https://arxiv.org/abs/2209.01259)
- [Compositional Deep Learning](https://arxiv.org/abs/1907.08292)
- `src/sketches.rs`
- `examples/05_seven_sketches.rs`

Critique before rewrite:

The Seven Sketches chapter had a strong paper-to-Rust map and transfer tasks,
but the runnable companion still required readers to infer which boundary each
printed line protected. A broad chapter needs extra output discipline because
the examples jump across orders, resources, databases, relations, matrices,
circuits, and behavior logic.

ML teaching critique:

The chapter should keep the tiny ML thread visible while leaving the ML
pipeline. Each sketch should be read as a model of a protected engineering
boundary: structured data, resources, linear composition, interfaces, and
local checks.

Category-theory critique:

The output should name the protected shape for every sketch: preorder, Galois
law, monoidal resource composition, database instance, feasibility relation,
matrix composition, open-system composition, and local-to-global truth.

Learner critique:

A reader can run the companion and see many true values without knowing what
to transfer. The checklist should make each output line answer: Rust handle,
protected structure, and shortcut to reject.

Rewrite decision:

- make `examples/05_seven_sketches.rs` print the typed form protected by each
  sketch,
- add an example-output transfer checklist to the chapter,
- map each line to a Rust handle and invalid shortcut,
- update the chapter contract and scorecard so the checklist is tracked as
  practice evidence.

Validation:

```bash
cargo run --example 05_seven_sketches
python3 scripts/check-prose-style.py
python3 scripts/check-chapter-contracts.py
python3 scripts/check-chapter-scorecard.py
python3 scripts/check-rewrite-log.py
bash scripts/check.sh
```

All focused checks passed. The companion now prints the protected form for
each sketch after the concrete values. Book chapter tests passed.

## Sweep 2 Pass 103: Structure Example Output Transfer Checklist

Files:

```text
book/src/05-structure-and-calculus.md
examples/04_structure_and_calculus.rs
book/CHAPTER_CONTRACTS.md
book/CHAPTER_QUALITY_SCORECARD.md
book/CHAPTER_REWRITE_LOG.md
```

Primary sources:

- [Backprop as Functor](https://arxiv.org/abs/1711.10455)
- [Seven Sketches in Compositionality](https://arxiv.org/abs/1803.05316)
- [Rust Book: Advanced Traits](https://doc.rust-lang.org/book/ch20-02-advanced-traits.html)
- [D2L Backpropagation and Computational Graphs](https://d2l.ai/chapter_multilayer-perceptrons/backprop.html)
- `src/structure.rs`
- `src/calculus.rs`
- `examples/04_structure_and_calculus.rs`

Critique before rewrite:

The structure chapter already traced functor mapping, naturality squares,
monoid laws, and the local chain rule. Its remaining weakness was transfer
from the runnable example. The example printed values and booleans, but did
not print the typed boundaries that make the laws visible.

ML teaching critique:

The chain-rule output should remind readers that backpropagation is local
rules composed through a computation graph. The trace output should also show
that logs and pipeline traces are accumulated values, not untyped decoration.

Category-theory critique:

Functor, naturality, and monoid should be tied to exact consistency
conditions. The runnable output should distinguish item-level functions from
wrapper-level mapping, one naturality path from the whole square, and a raw
append operation from a monoid law check.

Learner critique:

A reader can run the example and remember only `[1, 4, 9]`, `true`, and
`dL/dx`. The chapter needs a transfer checklist that turns those visible
results into boundary ownership and invalid shortcuts.

Rewrite decision:

- make `examples/04_structure_and_calculus.rs` print the typed mapping,
  naturality, monoid, and chain-rule boundaries,
- add an example-output transfer checklist to the chapter,
- map each output line to the consistency condition it protects,
- update the chapter contract and scorecard so the checklist is tracked as
  practice evidence.

Validation:

```bash
cargo run --example 04_structure_and_calculus
python3 scripts/check-prose-style.py
python3 scripts/check-chapter-contracts.py
python3 scripts/check-chapter-scorecard.py
python3 scripts/check-rewrite-log.py
bash scripts/check.sh
```

All focused checks passed. The example now prints functor, naturality,
monoid, and chain-rule boundaries after the visible values. Book chapter tests
passed.

## Sweep 2 Pass 102: Training Example Output Transfer Checklist

Files:

```text
book/src/04-training-endomorphism.md
examples/03_training_endomorphism.rs
book/CHAPTER_CONTRACTS.md
book/CHAPTER_QUALITY_SCORECARD.md
book/CHAPTER_REWRITE_LOG.md
```

Primary sources:

- [D2L Gradient Descent](https://d2l.ai/chapter_optimization/gd.html)
- [CS231n Optimization](https://cs231n.github.io/optimization-1/)
- [D2L Backpropagation and Computational Graphs](https://d2l.ai/chapter_multilayer-perceptrons/backprop.html)
- `src/training.rs`
- `examples/03_training_endomorphism.rs`

Critique before rewrite:

The Training as an Endomorphism chapter already explained the update trace,
the gradient sign, and the distinction between measurement and update. Its
remaining weakness was the executable bridge. The companion example printed
loss before and loss after, but did not name the typed state transition that
made repeated training legal.

ML teaching critique:

The learner needs to see the standard gradient-descent separation in the
terminal output: measure loss, compute update direction, and return updated
parameters. A lower loss is evidence for the tiny run, not proof that every
optimizer detail is correct.

Category-theory critique:

The chapter's categorical claim is not "loss goes down." The claim is that
`TrainStep` has the repeatable shape `Parameters -> Parameters`. The runnable
output should distinguish this endomorphism from the diagnostic measurement
`Parameters x TrainingSet -> Loss`.

Learner critique:

A reader can run the example and still remember only two numbers. The new
checklist turns each printed line into a boundary to own and an invalid
shortcut to reject.

Rewrite decision:

- make `examples/03_training_endomorphism.rs` print the typed update shape,
  repeated endomorphism trace, and measurement boundary,
- add an example-output transfer checklist to the chapter,
- map each printed line to the Rust boundary and the shortcut it rejects,
- update the chapter contract and scorecard so the checklist is tracked as
  practice evidence.

Validation:

```bash
cargo run --example 03_training_endomorphism
python3 scripts/check-prose-style.py
python3 scripts/check-chapter-contracts.py
python3 scripts/check-chapter-scorecard.py
python3 scripts/check-rewrite-log.py
bash scripts/check.sh
```

All focused checks passed. The example now prints the loss measurements,
typed update boundary, repeated endomorphism trace, and measurement boundary.
Book chapter tests passed.

## Sweep 2 Pass 101: Tiny ML Demo Output Transfer Checklist

Files:

```text
book/src/03-ml-pipeline.md
book/CHAPTER_CONTRACTS.md
book/CHAPTER_QUALITY_SCORECARD.md
book/CHAPTER_REWRITE_LOG.md
```

Primary sources:

- [Dive into Deep Learning: Softmax Regression](https://d2l.ai/chapter_linear-classification/softmax-regression.html)
- [Dive into Deep Learning: Softmax Regression Implementation from Scratch](https://d2l.ai/chapter_linear-classification/softmax-regression-scratch.html)
- [CS231n Linear Classification](https://cs231n.github.io/linear-classify/)
- `src/ml.rs`
- `src/demo.rs`

Critique before rewrite:

The Tiny ML Pipeline chapter already separated logits, distributions, target
probability, and loss. Its remaining weakness was the bridge from the terminal
demo back to typed boundaries. The chapter told readers to inspect demo
sections 2 through 5, but did not yet translate each visible output line into
the Rust object, ML meaning, category shape, and invalid shortcut being
protected.

ML teaching critique:

The demo output should make clear that training data is input paired with a
target, prediction is a distribution rather than a single token, and loss uses
the probability assigned to the target token.

Category-theory critique:

The product boundary needed to appear beside the actual demo line that says
`Prediction x Target -> Loss`. This makes `Distribution x TokenId -> Loss`
feel like the reason the loss arrow is legal, not a glossary phrase.

Learner critique:

A reader can run the demo and still focus only on the numbers. The checklist
should turn the run into a self-explanation task: visible output, typed
boundary, invalid shortcut rejected.

Rewrite decision:

Add a demo-output transfer checklist after the run instructions. Map
`TokenSequence -> TrainingSet`, adjacent pairs, composed prediction,
probabilities, product boundary, and target loss to concrete Rust types and the
wrong shortcuts each boundary rejects. Update the scorecard and chapter
contract to track the checklist as practice evidence.

Validation:

```bash
cargo run --bin category_ml
python3 scripts/check-prose-style.py
python3 scripts/check-chapter-contracts.py
python3 scripts/check-chapter-scorecard.py
python3 scripts/check-rewrite-log.py
bash scripts/check.sh
```

All passed. The full publication gate passed with 106 Rust tests, all
examples, the main demo, prose style checks, completion-audit checks,
reader-feedback-loop checks, rewrite-log checks, chapter reference coverage
checks, chapter contract and scorecard checks, source authority checks, chapter
maturity checks, diagram coverage checks, exercise alignment checks, exercise
command checks, duplicate-prose checks, book build, and chapter tests.

## Sweep 2 Pass 100: Morphism Output Transfer Checklist

Files:

```text
book/src/02-morphisms-composition.md
examples/02_morphism_composition.rs
book/CHAPTER_CONTRACTS.md
book/CHAPTER_QUALITY_SCORECARD.md
book/CHAPTER_REWRITE_LOG.md
```

Primary sources:

- [Rust Book: Generic Data Types](https://doc.rust-lang.org/book/ch10-01-syntax.html)
- [Rust Book: Defining Shared Behavior with Traits](https://doc.rust-lang.org/stable/book/ch10-02-traits.html)
- [Seven Sketches in Compositionality](https://arxiv.org/abs/1803.05316)
- [Category Theory for Programming](https://arxiv.org/abs/2209.01259)
- `src/category.rs`
- `examples/02_morphism_composition.rs`

Critique before rewrite:

The Morphism and Composition chapter had a strong explanation of `Morphism`,
`Identity`, `Compose`, and `Endomorphism`, plus a broken-pipeline worked
example. The runnable companion example, however, printed only final
probabilities. A reader could run the command, see a numeric output, and still
miss the middle objects that make the composed path legal.

ML teaching critique:

The final probabilities should be treated as the end of a prediction path, not
as the whole path. The reader needs to see that `Distribution` is produced only
after `TokenId -> Vector -> Logits` has happened.

Category-theory critique:

The visible output and the categorical arrow should not be confused. A
composed morphism may be summarized as `TokenId -> Distribution`, but that
summary does not erase the typed obligations of the middle objects.

Learner critique:

First-pass readers need one short checklist that translates command output and
code evidence into boundary ownership. The checklist should ask what each line
proves and which shortcut it rejects.

Rewrite decision:

Make `examples/02_morphism_composition.rs` print the typed transformation path
after the probabilities. Add an example-output transfer checklist to the
chapter so readers connect the final numeric output to `TokenId -> Vector ->
Logits -> Distribution`, `Compose::<_, _, Vector>`, and `Compose::<_, _,
Logits>`.

Validation:

```bash
cargo run --example 02_morphism_composition
python3 scripts/check-prose-style.py
python3 scripts/check-chapter-contracts.py
python3 scripts/check-chapter-scorecard.py
python3 scripts/check-rewrite-log.py
bash scripts/check.sh
```

All passed. The full publication gate passed with 106 Rust tests, all
examples, the main demo, prose style checks, completion-audit checks,
reader-feedback-loop checks, rewrite-log checks, chapter reference coverage
checks, chapter contract and scorecard checks, source authority checks, chapter
maturity checks, diagram coverage checks, exercise alignment checks, exercise
command checks, duplicate-prose checks, book build, and chapter tests.

## Sweep 2 Pass 99: Transformer Roadmap Output Transfer Checklist

Files:

```text
book/src/roadmap.md
book/src/references.md
book/CHAPTER_CONTRACTS.md
book/CHAPTER_QUALITY_SCORECARD.md
```

Primary sources:

- [Attention Is All You Need](https://arxiv.org/abs/1706.03762)
- [Dive into Deep Learning: Multi-Head Attention](https://d2l.ai/chapter_attention-mechanisms-and-transformers/multihead-attention.html)
- [Dive into Deep Learning: Transformer Architecture](https://d2l.ai/chapter_attention-mechanisms-and-transformers/transformer.html)
- [PyTorch: `scaled_dot_product_attention`](https://docs.pytorch.org/docs/2.11/generated/torch.nn.functional.scaled_dot_product_attention.html)
- `examples/06_attention_scores.rs`
- `src/attention.rs`

Critique before rewrite:

The Transformer roadmap had enough implementation detail, but the final
retrieval practice was still mostly recall. A reader could run the example,
recognize the printed lines, and still fail to transfer them into exact
ownership of scores, masks, weights, value mixing, multi-head projection,
shape-preserving sublayers, and training-state updates.

ML teaching critique:

The chapter needed a stronger bridge from framework-compressed attention to the
repository's uncompressed path. The key learner risk is treating scores as
weights, masking after softmax, mixing values before weights exist, or adding
unprojected multi-head output back to the residual stream.

Category-theory critique:

The practice needed to make each printed line behave like a typed morphism
boundary. The learner should name source object, target object, preserved
shape, and rejected shortcut instead of giving a generic "shape mismatch"
answer.

Learner critique:

The runnable output is long enough that first-pass readers need a checklist.
The checklist should convert each output line into: Rust object, ML role,
shape condition, and invalid connection prevented.

Rewrite decision:

Add an output-to-boundary transfer checklist before retrieval practice. Expand
recall, explanation, application, and debugging prompts so learners must use
the attention example output to diagnose scores versus weights, mask order,
multi-head projection arithmetic, feed-forward shape preservation,
training-state composition, and the limited scope of finite-difference checks.

Validation:

```bash
python3 scripts/check-prose-style.py
python3 scripts/check-chapter-contracts.py
python3 scripts/check-chapter-scorecard.py
python3 scripts/check-exercise-commands.py
python3 scripts/check-duplicate-prose.py
mdbook test
cargo run --example 06_attention_scores
python3 scripts/check-rewrite-log.py
bash scripts/check.sh
```

All passed. The full publication gate passed with 106 Rust tests, all
examples, the main demo, prose style checks, completion-audit checks,
reader-feedback-loop checks, rewrite-log checks, chapter reference coverage
checks, chapter contract and scorecard checks, source authority checks, chapter
maturity checks, diagram coverage checks, exercise alignment checks, exercise
command checks, duplicate-prose checks, book build, and chapter tests.

## Sweep 2 Pass 98: Mixed Exercise Transfer Upgrade

Files:

```text
book/src/exercises.md
exercises/ANSWER_KEY.md
exercises/README.md
book/CHAPTER_CONTRACTS.md
book/CHAPTER_QUALITY_SCORECARD.md
book/CHAPTER_REWRITE_LOG.md
```

Primary sources:

- [How People Learn II](https://nap.nationalacademies.org/catalog/24783/how-people-learn-ii-learners-contexts-and-cultures)
- [Improving Students' Learning With Effective Learning Techniques](https://pubmed.ncbi.nlm.nih.gov/26173288/)
- [Test-enhanced learning](https://pubmed.ncbi.nlm.nih.gov/16507066/)
- [Rust Book: Testing](https://doc.rust-lang.org/book/ch11-00-testing.html)

Critique before rewrite:

The exercise chapter had a strong ladder, evidence map, attempt-record template,
worked examples, and aligned answer key. Its transfer practice was still mostly
chapter-local. A learner could solve a domain-object exercise, then a training
exercise, then a sketch exercise, while relying on the chapter heading to tell
them which boundary mattered.

Pedagogy critique:

Retrieval practice and transfer improve when learners must pull the right idea
without an immediate label. The exercise set needed one interleaved diagnosis
task where invariant, composition, endomorphism, shape, and local-to-global
failures appear together.

Technical critique:

The best transfer check should not require a new dependency or large code edit.
It should ask readers to classify concrete failure shapes already present in
the book and code: raw IDs, skipped morphisms, loss-product boundaries,
training-state updates, matrix dimensions, safety covers, and residual shapes.

Learner critique:

A motivated reader could still overuse one explanation such as "the compiler
rejects it." The new mixed exercise forces them to distinguish compiler type
errors, constructor errors, returned `ShapeMismatch` errors, endomorphism
contract mistakes, and law-check failures.

Rewrite decision:

- add Exercise 15, Mixed Boundary Diagnosis,
- add seven interleaved failure cases across domain objects, composition, loss,
  training, signal matrices, behavior logic, and attention residuals,
- require a boundary classification plus Rust, ML/software, category-theory,
  and smallest-fix fields,
- add an answer-key classification table and facilitator notes,
- mention the mixed transfer check in `exercises/README.md`,
- update the chapter contract and scorecard so this transfer surface is tracked.

Validation:

```bash
python3 scripts/check-exercise-alignment.py
python3 scripts/check-exercise-commands.py
python3 scripts/check-chapter-contracts.py
python3 scripts/check-chapter-scorecard.py
python3 scripts/check-prose-style.py
python3 scripts/check-duplicate-prose.py
mdbook test
```

Result:

```text
Exercise alignment check passed.
Exercise command check passed.
Chapter contract check passed.
Chapter scorecard check passed.
Prose style check passed.
Duplicate prose check passed.
Book chapter tests passed.
```

## Sweep 2 Pass 97: Seven Sketches Retrieval Practice Upgrade

Files:

```text
book/src/seven-sketches-rust.md
book/CHAPTER_QUALITY_SCORECARD.md
book/CHAPTER_REWRITE_LOG.md
```

Primary sources:

- [Seven Sketches in Compositionality](https://arxiv.org/abs/1803.05316)
- [Category Theory for Programming](https://arxiv.org/abs/2209.01259)
- [Rust Book: Defining Shared Behavior with Traits](https://doc.rust-lang.org/book/ch10-02-traits.html)
- [Rust API Guidelines: Type Safety](https://rust-lang.github.io/api-guidelines/type-safety.html)

Critique before rewrite:

The Seven Sketches chapter had a strong paper-to-Rust map, a core-transfer
guide, transfer tasks after each sketch, and source snapshots for
`src/sketches.rs` and `examples/05_seven_sketches.rs`. Its final retrieval
practice was still too broad. Naming three structures and inventing one law did
not force a reader to connect the companion output to the protected boundary in
each Rust model.

Software teaching critique:

The chapter should train readers to ask what invalid structure each model
rejects. The most useful examples are concrete: missing foreign keys, mismatched
matrix dimensions, mismatched circuit boundaries, and false local behavior
checks.

Category-theory critique:

The breadth of the source paper can overwhelm a first reader. The practice
should preserve breadth while making the repeated method explicit:
name objects, name relationships, control construction, define composition, and
check the law.

Learner critique:

A motivated reader could run the companion example and see true law checks
without being asked to translate a printed line into Rust value, software
meaning, category-theory shape, and rejected boundary. The new practice creates
that transfer pressure.

Rewrite decision:

- expand Retrieval Practice into Recall, Explain, Apply, and Debug sections,
- ask readers to identify sketches by the boundary they protect,
- tie practice to concrete companion-output lines,
- add a four-field transfer template for Rust value, software meaning,
  category-theory shape, and invalid structure prevented,
- add debugging prompts for raw ids, missing references, shape mismatch,
  boundary mismatch, and false local checks,
- update the scorecard focus to validate the expanded practice with readers who
  came for the tiny ML path.

Validation:

```bash
python3 scripts/check-prose-style.py
python3 scripts/check-chapter-scorecard.py
python3 scripts/check-exercise-commands.py
python3 scripts/check-duplicate-prose.py
mdbook test
cargo run --example 05_seven_sketches
```

Result:

```text
Prose style check passed.
Chapter scorecard check passed.
Exercise command check passed.
Duplicate prose check passed.
Book chapter tests passed.
Seven Sketches example passed with preorder, Galois, resource, database, feasibility, signal-flow, circuit, and behavior output.
```

## Sweep 2 Pass 96: Structure Retrieval Practice Upgrade

Files:

```text
book/src/05-structure-and-calculus.md
book/CHAPTER_QUALITY_SCORECARD.md
book/CHAPTER_REWRITE_LOG.md
```

Primary sources:

- [Backprop as Functor](https://arxiv.org/abs/1711.10455)
- [Seven Sketches in Compositionality](https://arxiv.org/abs/1803.05316)
- [Category Theory for Programming](https://arxiv.org/abs/2209.01259)
- [Dive into Deep Learning: Forward Propagation, Backward Propagation, and Computational Graphs](https://d2l.ai/chapter_multilayer-perceptrons/backprop.html)
- [Rust Book: Defining Shared Behavior with Traits](https://doc.rust-lang.org/book/ch10-02-traits.html)

Critique before rewrite:

The structure chapter had a good law-test-analogy table, Mermaid diagrams for
the naturality square and monoid grouping, and concrete source snapshots for
`src/structure.rs` and `src/calculus.rs`. Its final retrieval practice still
asked only one broad question per section, so a reader could leave with the
words `functor`, `natural transformation`, `monoid`, and `chain rule` without
being forced to trace the exact paths the code checks.

ML teaching critique:

The chain-rule section needs active numeric practice. D2L presents
backpropagation as reverse traversal of a computational graph using stored
intermediates and the chain rule; the chapter's tiny version is
`MulOp::backward`. The retrieval practice should make the reader calculate one
new upstream-gradient case.

Category-theory critique:

The strongest transfer target is not vocabulary recall. It is recognizing which
consistency condition is at stake: functor identity and composition,
naturality-square agreement, monoid identity and associativity, or local
derivative composition.

Learner critique:

A motivated reader could run `cargo run --example 04_structure_and_calculus`
and see `true` for the law checks without explaining what would break if a
wrapper conversion inspected special details of `A`, if an empty trace changed
the trace, or if backpropagation were described as one undifferentiated giant
derivative.

Rewrite decision:

- expand Retrieval Practice into Recall, Explain, Apply, and Debug sections,
- ask readers to name both naturality-square paths,
- ask readers to name monoid identity and associativity for `PipelineTrace`,
- add a new upstream-gradient calculation for `MulOp::backward`,
- add debugging prompts for broken functor, naturality, monoid, and chain-rule
  explanations,
- update the scorecard focus to validate the expanded law-tracing practice with
  a category-theory reader.

Validation:

```bash
python3 scripts/check-prose-style.py
python3 scripts/check-chapter-scorecard.py
python3 scripts/check-exercise-commands.py
python3 scripts/check-duplicate-prose.py
mdbook test
cargo run --example 04_structure_and_calculus
```

Result:

```text
Prose style check passed after splitting dense recall and explanation lists.
Chapter scorecard check passed.
Exercise command check passed.
Duplicate prose check passed.
Book chapter tests passed.
Structure and calculus example passed with naturality, monoid, and local-gradient output.
```

## Sweep 2 Pass 95: Training Retrieval Practice Upgrade

Files:

```text
book/src/04-training-endomorphism.md
book/CHAPTER_QUALITY_SCORECARD.md
book/CHAPTER_REWRITE_LOG.md
```

Primary sources:

- [Dive into Deep Learning: Gradient Descent](https://d2l.ai/chapter_optimization/gd.html)
- [Dive into Deep Learning: Forward Propagation, Backward Propagation, and Computational Graphs](https://d2l.ai/chapter_multilayer-perceptrons/backprop.html)
- [CS231n Optimization](https://cs231n.github.io/optimization-1/)
- [CS231n Softmax Classifier Case Study](https://cs231n.github.io/neural-networks-case-study/)
- [Backprop as Functor](https://arxiv.org/abs/1711.10455)

Critique before rewrite:

The Training as an Endomorphism chapter had a concrete gradient-sign worked
example, code snapshots, one-step shape checks, and a repeated-loss regression
test. Its final retrieval practice still asked only three broad questions. That
was too weak for the chapter's central transfer skill: distinguishing a
measurement arrow from an update arrow, then tracing one gradient sign through
the parameter update.

ML teaching critique:

The common learner confusion is reading `parameter -= learning_rate * gradient`
as "all corrected parameters go down." The chapter explains that the target
gradient can be negative, but the practice needed a fresh numeric case where
the learner computes `dlogits` and updated bias without copying the worked
example.

Category-theory critique:

The endomorphism shape should be practiced as the outer contract:
`Parameters_t -> Parameters_{t+1}`. Loss and gradients are internal evidence
for how to build the next state, but returning `Loss` or only one raw matrix
breaks the repeatable object shape.

Learner critique:

A motivated reader could run the example and see lower loss while still
mistaking evaluation for training. The new retrieval section asks them to debug
invalid shortcuts: returning loss, discarding parts of `Parameters`, repeating a
non-endomorphism, and skipping bounds checks before gradient indexing.

Rewrite decision:

- expand Retrieval Practice into Recall, Explain, Apply, and Debug sections,
- ask for the line that performs the softmax-cross-entropy target correction,
- separate `Parameters -> Loss` from `Parameters -> Parameters`,
- add a fresh sign-trace calculation for `dlogits` and bias updates,
- ask why batch scaling averages accumulated gradients,
- add invalid shortcuts that require naming the broken shape or missing state,
- update the scorecard focus to validate the expanded practice with an ML
  learner.

Validation:

```bash
python3 scripts/check-prose-style.py
python3 scripts/check-chapter-scorecard.py
python3 scripts/check-exercise-commands.py
python3 scripts/check-duplicate-prose.py
mdbook test
```

Result:

```text
Prose style check passed.
Chapter scorecard check passed.
Exercise command check passed.
Duplicate prose check passed.
Book chapter tests passed.
Training endomorphism example passed: loss before 1.609389, loss after 0.009720.
```

## Sweep 2 Pass 94: Tiny ML Retrieval Practice Upgrade

Files:

```text
book/src/03-ml-pipeline.md
book/CHAPTER_QUALITY_SCORECARD.md
book/CHAPTER_REWRITE_LOG.md
```

Primary sources:

- [Dive into Deep Learning: Softmax Regression](https://d2l.ai/chapter_linear-classification/softmax-regression.html)
- [Dive into Deep Learning: Softmax Regression From Scratch](https://d2l.ai/chapter_linear-classification/softmax-regression-scratch.html)
- [CS231n Linear Classification](https://cs231n.github.io/linear-classify/)
- [Deep Learning](https://www.deeplearningbook.org/)

Critique before rewrite:

The Tiny ML Pipeline chapter had strong worked examples for logits,
probabilities, target probability, and loss, but the final retrieval practice
still asked only three broad questions. It did not yet force the learner to
name the boundary where logits become a distribution or the boundary where a
target token selects the probability used by cross entropy.

ML teaching critique:

The central misconception is treating loss as a function of the largest
probability or of raw scores. The practice should require the learner to select
the target probability and explain why logits must pass through softmax before
loss.

Category-theory critique:

The product boundary should be practiced explicitly:
`Distribution x TokenId -> Loss`. The learner should see why neither
`Logits -> Loss` nor `Distribution -> Loss` is the chapter's legal loss
boundary.

Learner critique:

After the chapter's numeric examples, a learner needs a short transfer check
with new numbers and a debugging prompt for invalid shortcuts. The old
retrieval section did not create that transfer pressure.

Rewrite decision:

- expand Retrieval Practice into Recall, Explain, Apply, and Debug sections,
- ask for `DatasetWindowing`, prediction arrows, and the loss product object,
- add target-index questions for a new distribution,
- add adjacent-pair practice for a new token sequence,
- add invalid shortcuts that require naming the missing boundary or wrong
  object,
- update the scorecard focus to validate the expanded practice with an ML
  learner.

Validation:

```bash
python3 scripts/check-prose-style.py
python3 scripts/check-chapter-scorecard.py
python3 scripts/check-exercise-commands.py
python3 scripts/check-duplicate-prose.py
mdbook test
```

Result:

```text
Prose style check passed.
Chapter scorecard check passed.
Exercise command check passed.
Duplicate prose check passed.
Book chapter tests passed.
```

## Sweep 2 Pass 93: Composition Retrieval Practice Upgrade

Files:

```text
book/src/02-morphisms-composition.md
book/CHAPTER_QUALITY_SCORECARD.md
book/CHAPTER_REWRITE_LOG.md
```

Primary sources:

- [Rust Book: Traits](https://doc.rust-lang.org/book/ch10-02-traits.html)
- [Rust Book: Generic Data Types](https://doc.rust-lang.org/book/ch10-01-syntax.html)
- [Seven Sketches in Compositionality](https://arxiv.org/abs/1803.05316)
- [Category Theory for Programming](https://arxiv.org/abs/2209.01259)

Critique before rewrite:

The Morphism and Composition chapter had a strong broken-pipeline worked
example, but its final retrieval practice still asked only three broad
questions. That was too weak for the chapter's central transfer skill: reading
typed composition failures as missing or mismatched middle objects.

ML teaching critique:

The practice needed to reinforce the actual tiny ML path:
`TokenId -> Vector -> Logits -> Distribution`. Learners should name `Vector`
and `Logits` as middle objects, not only say that stages compose.

Category-theory critique:

The exercises should connect source object, target object, identity,
composition, and endomorphism. The key transfer move is to inspect the endpoints
of arrows and decide whether composition or repetition is legal.

Learner critique:

A reader who runs `cargo run --example 02_morphism_composition` needs concrete
prompts that turn the printed probability output back into the typed path that
produced it. The old practice did not ask enough debugging questions.

Rewrite decision:

- expand the Retrieval Practice section into Recall, Explain, Apply, and Debug,
- ask for required `Morphism` methods, bridge type, and endomorphism shape,
- add prompts for legal `TokenId -> Distribution` composition, identity
  insertion, and why `Embedding` cannot be repeated as an endomorphism,
- add invalid shortcuts that require naming the missing or mismatched middle
  type,
- update the scorecard focus to validate this practice with direct readers.

Validation:

```bash
python3 scripts/check-prose-style.py
python3 scripts/check-chapter-scorecard.py
python3 scripts/check-exercise-commands.py
python3 scripts/check-duplicate-prose.py
mdbook test
```

Result:

```text
Prose style check passed.
Chapter scorecard check passed.
Exercise command check passed.
Duplicate prose check passed.
Book chapter tests passed.
```

## Sweep 2 Pass 92: Transformer Retrieval Practice Upgrade

Files:

```text
book/src/roadmap.md
book/CHAPTER_QUALITY_SCORECARD.md
book/CHAPTER_REWRITE_LOG.md
```

Primary sources:

- [How People Learn II](https://www.nationalacademies.org/projects/DBASSE-BBCSS-13-06/publication/24783)
- [Test-Enhanced Learning](https://doi.org/10.1111/j.1467-9280.2006.01693.x)
- [Improving Students' Learning With Effective Learning Techniques](https://journals.sagepub.com/doi/abs/10.1177/1529100612453266)

Critique before rewrite:

The Transformer Roadmap had become much more concrete, but the final retrieval
practice still asked only three broad questions. That was too weak for a long
chapter that now covers roles, masks, multi-head shape arithmetic, residuals,
normalization, feed-forward computation, structured state, and three update
types.

ML teaching critique:

The practice section needed to ask about the actual ML boundaries that cause
confusion: mask-before-softmax, output projection before residual addition, and
training-state preservation after different updates.

Category-theory critique:

The exercises should force transfer from named objects and morphisms to invalid
shortcuts. Learners should identify the missing boundary, not only say "shape
mismatch."

Learner critique:

A reader who runs the attention example needs immediate prompts that use its
printed output. The rewrite should start with the command, then ask recall,
explain, apply, and debug questions in increasing depth.

Rewrite decision:

- expand the Transformer Roadmap retrieval practice into Recall, Explain,
  Apply, and Debug sections,
- anchor the practice to `cargo run --example 06_attention_scores`,
- add questions for role objects, shape-preserving boundaries, state updates,
  mask-before-softmax, output projection, and missing-boundary debugging,
- add short purpose sentences under each subheading to satisfy the prose style
  gate,
- update the scorecard focus to validate the expanded practice with direct
  readers.

Validation:

```bash
python3 scripts/check-prose-style.py
python3 scripts/check-chapter-scorecard.py
python3 scripts/check-exercise-commands.py
python3 scripts/check-duplicate-prose.py
mdbook test
```

Result:

```text
Prose style check passed.
Chapter scorecard check passed.
Exercise command check passed.
Duplicate prose check passed.
Book chapter tests passed.
```

## Sweep 2 Pass 91: Structured Training State Trace

Files:

```text
book/src/roadmap.md
book/CHAPTER_QUALITY_SCORECARD.md
book/CHAPTER_REWRITE_LOG.md
```

Primary sources:

- [Dive into Deep Learning: Parameter Management](https://d2l.ai/chapter_builders-guide/parameters.html)
- [Dive into Deep Learning: Gradient Descent](https://d2l.ai/chapter_optimization/gd.html)
- [Dive into Deep Learning: Backpropagation and Computational Graphs](https://d2l.ai/chapter_multilayer-perceptrons/backprop.html)
- [Backprop as Functor](https://arxiv.org/abs/1711.10455)

Critique before rewrite:

The Transformer Roadmap named `TransformerTrainingState` and three train-step
endomorphisms, but the section moved quickly from implementation inventory to
general ML explanation. A reader could miss the central state contract: each
update may touch different internals, but every update must return the same
structured training state.

ML teaching critique:

The source-backed training pattern is forward computation, loss, reverse
gradient flow, and parameter update under a learning rate. The chapter needed
to separate three concrete training questions: readout-only learning, local
feed-forward learning, and composed block learning.

Category-theory critique:

The update examples should make the endomorphism shape visible before the
gradient details expand:
`TransformerTrainingState -> TransformerTrainingState`. The chapter should not
leave readers with the idea that each update returns a different bag of
parameters.

Learner critique:

The attention example already prints three loss reductions. The rewrite should
reuse those numbers so the learner sees actual evidence instead of a new
abstract claim about training loops.

Rewrite decision:

- add `Worked Example: Three Updates, One State Shape`,
- reuse the step-count and loss lines from `examples/06_attention_scores.rs`,
- show readout-only, local feed-forward, and composed block updates as three
  different questions with the same outer state type,
- name the invalid shortcut where updates return loose component weights,
- update the scorecard focus to validate this trace before expanding optimizer
  details.

Validation:

```bash
python3 scripts/check-prose-style.py
python3 scripts/check-chapter-scorecard.py
python3 scripts/check-duplicate-prose.py
mdbook test
```

Result:

```text
Prose style check passed.
Chapter scorecard check passed.
Duplicate prose check passed.
Book chapter tests passed.
```

## Sweep 2 Pass 90: Shape-Preserving Normalization And Feed-Forward

Files:

```text
book/src/roadmap.md
book/CHAPTER_QUALITY_SCORECARD.md
book/CHAPTER_REWRITE_LOG.md
```

Primary sources:

- [Attention Is All You Need](https://arxiv.org/abs/1706.03762)
- [Layer Normalization](https://arxiv.org/abs/1607.06450)
- [Dive into Deep Learning: Transformer Architecture](https://d2l.ai/chapter_attention-mechanisms-and-transformers/transformer.html)

Critique before rewrite:

The Transformer Roadmap explained that both layer normalization and the
position-wise feed-forward sublayer have shape
`HiddenSequence -> HiddenSequence`, but it did not yet show a concrete row
trace after the residual worked example. A reader could believe the type
boundary while still missing the difference between changing values and
changing the public object.

ML teaching critique:

The source-backed Transformer block pattern uses residual addition, layer
normalization, and position-wise feed-forward networks around the hidden
sequence. The chapter needed to make the invariant visible with numbers:
normalization changes per-row feature values, and feed-forward computation
changes per-row feature values, but neither changes sequence length or model
dimension.

Category-theory critique:

The rewrite should keep both sublayers as endomorphisms on the same object:
`HiddenSequence -> HiddenSequence`. Introducing separate learner-facing object
names for "normalized sequence" or "feed-forward sequence" would blur the
composable block boundary that the Rust types are teaching.

Learner critique:

After the residual trace, the learner has real rows in hand. The next section
should reuse those rows so the explanation feels like continuing one example,
not starting another abstraction.

Rewrite decision:

- add `Worked Example: Values Change, Shape Stays HiddenSequence`,
- reuse residual, normalized, and feed-forward rows from
  `examples/06_attention_scores.rs`,
- show that both sublayers preserve `2 positions x model dimension 2`,
- name the invalid mental shortcut that treats normalization and feed-forward
  computation as new public sequence objects,
- update the scorecard focus to validate this trace with direct readers.

Validation:

```bash
python3 scripts/check-prose-style.py
python3 scripts/check-chapter-scorecard.py
python3 scripts/check-duplicate-prose.py
mdbook test
```

Result:

```text
Prose style check passed.
Chapter scorecard check passed.
Duplicate prose check passed.
Book chapter tests passed.
```

## Sweep 2 Pass 89: Residual Shape Trace

Files:

```text
book/src/roadmap.md
book/CHAPTER_QUALITY_SCORECARD.md
book/CHAPTER_REWRITE_LOG.md
```

Primary sources:

- [Attention Is All You Need](https://arxiv.org/abs/1706.03762)
- [Dive into Deep Learning: Transformer Architecture](https://d2l.ai/chapter_attention-mechanisms-and-transformers/transformer.html)

Critique before rewrite:

The Transformer Roadmap explained residual addition as
`HiddenSequence x ProjectedAttentionOutput -> HiddenSequence`, but it did not
yet show why the projection from the previous worked example is required before
the residual boundary. The reader could see the invariant without seeing one
row satisfy it.

ML teaching critique:

Transformer block explanations often say that residual connections preserve
shape, but learners still need to see the addition operate row by row. The
chapter needed to connect the projected attention output to the hidden input
with the same sequence length and model dimension.

Category-theory critique:

The legal path should stay explicit:
`MultiHeadOutput -> ProjectedAttentionOutput` followed by
`HiddenSequence x ProjectedAttentionOutput -> HiddenSequence`. The invalid
shortcut `HiddenSequence x MultiHeadOutput -> HiddenSequence` should be named
as the shape error that the typed path prevents.

Learner critique:

After learning that two heads of width two concatenate into width four, a
reader may reasonably ask why the example then returns to width two. The
rewrite answers that question with a row-level residual trace.

Rewrite decision:

- add `Worked Example: Residual Addition Needs The Same Shape`,
- reuse the projected rows and hidden rows from `examples/06_attention_scores.rs`,
- show two residual row additions,
- state the preserved `2 positions x model dimension 2` shape,
- name the illegal shortcut that would try to add hidden rows to
  unprojected multi-head rows,
- update the scorecard focus to validate the residual-shape trace with readers.

Validation:

```bash
python3 scripts/check-prose-style.py
python3 scripts/check-chapter-scorecard.py
python3 scripts/check-rewrite-log.py
python3 scripts/check-duplicate-prose.py
mdbook test
```

Result:

```text
Prose style check passed.
Chapter scorecard check passed.
Rewrite log check passed.
Duplicate prose check passed.
Book chapter tests passed.
```

## Sweep 2 Pass 88: Multi-Head Shape Arithmetic Worked Example

Files:

```text
book/src/roadmap.md
book/src/references.md
book/CHAPTER_QUALITY_SCORECARD.md
book/CHAPTER_REWRITE_LOG.md
```

Primary sources:

- [Attention Is All You Need](https://arxiv.org/abs/1706.03762)
- [Dive into Deep Learning: Multi-Head Attention](https://d2l.ai/chapter_attention-mechanisms-and-transformers/multihead-attention.html)
- [PyTorch `MultiheadAttention`](https://docs.pytorch.org/docs/stable/generated/torch.nn.MultiheadAttention.html)

Critique before rewrite:

The Transformer Roadmap already explained that several attention heads are
concatenated and then projected. The missing learner bridge was shape
arithmetic. A reader could see `head_count`, `head_dimension`, and
`model_dimension`, but still not trace one row through concatenation and output
projection.

ML teaching critique:

Multi-head attention is often presented as a single block. For this book, the
important learning step is separating the parallel heads, concatenated width,
and learned output projection. The projection is not decoration; it is the
boundary that returns the concatenated head features to the surrounding model
width.

Category-theory critique:

The roadmap needed to keep two morphisms separate:
`AttentionHeadOutputs -> MultiHeadOutput` and
`MultiHeadOutput -> ProjectedAttentionOutput`. Combining them too early hides
which invariant each boundary protects.

Learner critique:

A learner should be able to compute `2 heads x 2 features = 4`, then verify
why the projection accepts a four-feature row and emits a two-feature row in
the tiny example. Without that arithmetic, later residual-shape checks feel
arbitrary.

Rewrite decision:

- add `Worked Example: Concatenate Heads, Then Project`,
- reuse the exact head rows and projection weights from
  `examples/06_attention_scores.rs`,
- show row concatenation for two query positions,
- compute `head_count * head_dimension`,
- compute one projected row by hand,
- keep concatenation and projection as separate typed boundaries,
- update the PyTorch `MultiheadAttention` reference note to mention head-split
  shape arithmetic,
- update the scorecard focus to validate the multi-head trace with readers.

Validation:

```bash
python3 scripts/check-prose-style.py
python3 scripts/check-chapter-references.py
python3 scripts/check-chapter-scorecard.py
python3 scripts/check-rewrite-log.py
python3 scripts/check-duplicate-prose.py
mdbook test
```

Result:

```text
Prose style check passed.
Chapter reference coverage check passed.
Chapter scorecard check passed.
Rewrite log check passed.
Duplicate prose check passed.
Book chapter tests passed.
```

## Sweep 2 Pass 87: Query-Key-Value Role Worked Example

Files:

```text
book/src/roadmap.md
book/src/references.md
book/CHAPTER_QUALITY_SCORECARD.md
book/CHAPTER_REWRITE_LOG.md
```

Primary sources:

- [Attention Is All You Need](https://arxiv.org/abs/1706.03762)
- [Dive into Deep Learning: Queries, Keys, and Values](https://d2l.ai/chapter_attention-mechanisms-and-transformers/queries-keys-values.html)
- [Dive into Deep Learning: Attention Scoring Functions](https://d2l.ai/chapter_attention-mechanisms-and-transformers/attention-scoring-functions.html)
- [PyTorch `MultiheadAttention`](https://docs.pytorch.org/docs/stable/generated/torch.nn.MultiheadAttention.html)

Critique before rewrite:

The Transformer Roadmap named query, key, and value roles, and the code exposed
`HiddenToQuery`, `HiddenToKey`, and `HiddenToValue`, but the prose did not yet
show one hidden row becoming three different role objects. A reader still had
to trust the vocabulary before seeing it execute as a small trace.

ML teaching critique:

Query, key, and value vectors can look numerically similar, especially in
self-attention where they originate from the same hidden sequence. The chapter
needed to separate "same source hidden row" from "same role".

Category-theory critique:

The role split should be introduced as three parallel morphisms from the same
object:
`HiddenSequence -> QuerySequence`, `HiddenSequence -> KeySequence`, and
`HiddenSequence -> ValueSequence`. The later attention composition depends on
that role separation.

Learner critique:

A learner can remember "query asks, key matches, value is mixed" as a slogan
without being able to trace a row. The rewrite gives a row-level numeric trace
and a table that ties each role to the exact downstream operation.

Rewrite decision:

- add `Worked Example: Same Hidden Row, Three Roles`,
- reuse the repository's typed projection names,
- show one hidden row projected into query, key, and value rows,
- add a role table for question and downstream use,
- name the two-phase attention path:
  `QuerySequence x KeySequence -> AttentionScores` and
  `AttentionWeights x ValueSequence -> AttentionOutput`,
- add PyTorch `MultiheadAttention` as an API-shape sanity-check reference,
- update the scorecard focus to validate the query-key-value example with
  readers.

Validation:

```bash
python3 scripts/check-prose-style.py
python3 scripts/check-chapter-references.py
python3 scripts/check-chapter-scorecard.py
python3 scripts/check-rewrite-log.py
python3 scripts/check-duplicate-prose.py
mdbook test
```

Result:

```text
Prose style check passed.
Chapter reference coverage check passed.
Chapter scorecard check passed.
Rewrite log check passed.
Duplicate prose check passed.
Book chapter tests passed.
```

## Sweep 2 Pass 86: Attention Mask-Before-Softmax Worked Example

Files:

```text
book/src/roadmap.md
book/src/references.md
book/CHAPTER_QUALITY_SCORECARD.md
book/CHAPTER_REWRITE_LOG.md
```

Primary sources:

- [Attention Is All You Need](https://arxiv.org/abs/1706.03762)
- [Dive into Deep Learning: Attention Scoring Functions](https://d2l.ai/chapter_attention-mechanisms-and-transformers/attention-scoring-functions.html)
- [Dive into Deep Learning: Multi-Head Attention](https://d2l.ai/chapter_attention-mechanisms-and-transformers/multihead-attention.html)
- [PyTorch `scaled_dot_product_attention`](https://docs.pytorch.org/docs/stable/generated/torch.nn.functional.scaled_dot_product_attention.html)

Critique before rewrite:

The Transformer Roadmap now ranked mask-before-softmax as the highest-risk
worked example, but the chapter still asked the reader to infer the numeric
story from the source snapshot and example output. That left a gap between the
priority table and the runnable code.

ML teaching critique:

Attention masks are easy to misunderstand as something applied to probabilities
after softmax. The chapter needed to show that masking happens at the score
stage so illegal positions never compete for probability mass.

Category-theory critique:

The boundary should stay visible as a typed composition:
`AttentionScores x AttentionMask -> AttentionScores -> AttentionWeights`.
The example should name the product input and the output object rather than
describing attention as one loose matrix operation.

Learner critique:

A reader can run `cargo run --example 06_attention_scores` and see the final
weights, but still not know why the middle position becomes exactly zero. The
rewrite gives a four-line numeric trace from raw scores to masked scores to
softmax weights to value mixing.

Rewrite decision:

- add `Worked Example: Mask Before Softmax`,
- reuse the exact query, key, value, and mask rows from
  `examples/06_attention_scores.rs`,
- show the first query row as raw scores, mask, masked scores, and row-wise
  softmax,
- show the value-mixing arithmetic that produces `[2.0, 20.0]`,
- clarify that the repository uses a very negative finite score to preserve
  constructor invariants while teaching the standard mask-before-softmax
  implementation pattern,
- add the PyTorch scaled-dot-product-attention documentation as an
  implementation sanity-check reference,
- update the scorecard focus to validate the new worked example with readers.

Validation:

```bash
python3 scripts/check-prose-style.py
python3 scripts/check-chapter-references.py
python3 scripts/check-chapter-scorecard.py
python3 scripts/check-rewrite-log.py
python3 scripts/check-duplicate-prose.py
mdbook test
```

Result:

```text
Prose style check passed.
Chapter reference coverage check passed.
Chapter scorecard check passed.
Rewrite log check passed.
Duplicate prose check passed.
Book chapter tests passed.
```

## Sweep 2 Pass 85: Transformer Worked-Example Priority

Files:

```text
book/src/roadmap.md
book/CHAPTER_QUALITY_SCORECARD.md
book/CHAPTER_REWRITE_LOG.md
```

Primary sources:

- [Attention Is All You Need](https://arxiv.org/abs/1706.03762)
- [NeurIPS proceedings: Attention Is All You Need](https://papers.nips.cc/paper/7181-attention-is-all-you-need)
- [Dive into Deep Learning: Attention Scoring Functions](https://d2l.ai/chapter_attention-mechanisms-and-transformers/attention-scoring-functions.html)
- [Dive into Deep Learning: Multi-Head Attention](https://d2l.ai/chapter_attention-mechanisms-and-transformers/multihead-attention.html)

Critique before rewrite:

The Transformer Roadmap already had a role ownership map, an implementation
status table, an attention shape-flow diagram, and detailed sections from
hidden states through training state. The remaining learner-path risk was
priority. A reader could see many implemented boundaries and not know which
ones need worked examples first.

ML teaching critique:

The most fragile attention concepts are the transition from roles to scores,
the mask before softmax, the score-to-weight normalization, and value mixing.
Those should be expanded before residuals, normalization, and training state.

Category-theory critique:

The roadmap should keep the typed-composition discipline visible: each worked
example should identify the boundary, the product input if any, the output
object, and the invalid connection it prevents.

Learner critique:

A motivated reader could ask, "Which section should I ask for a worked example
on first?" The rewrite adds a priority table that turns that vague question
into traceable evidence.

Rewrite decision:

- add `Worked Example Priority`,
- rank attention boundaries by teaching risk,
- put scores, masks, weights, and value mixing before multi-head arithmetic,
  residuals, normalization, feed-forward, and training state,
- add reader-evidence questions for each priority row,
- update the scorecard focus to validate the table with readers tracing
  attention shapes.

Validation:

```bash
python3 scripts/check-prose-style.py
python3 scripts/check-chapter-scorecard.py
python3 scripts/check-duplicate-prose.py
mdbook test
```

Result:

```text
Prose style check passed.
Chapter scorecard check passed.
Duplicate prose check passed.
Rewrite log check passed.
Book chapter tests passed.
```

## Sweep 2 Pass 84: Exercise Attempt Evidence Template

Files:

```text
book/src/exercises.md
exercises/README.md
exercises/ANSWER_KEY.md
scripts/check-exercise-alignment.py
book/CHAPTER_QUALITY_SCORECARD.md
book/CHAPTER_REWRITE_LOG.md
```

Primary sources:

- [How People Learn II](https://www.nationalacademies.org/projects/DBASSE-BBCSS-13-06/publication/24783)
- [Test-Enhanced Learning](https://doi.org/10.1111/j.1467-9280.2006.01693.x)
- [Structuring the Transition From Example Study to Problem Solving](https://doi.org/10.1207/S15326985EP3801_3)
- [Rust Book: Writing Automated Tests](https://doc.rust-lang.org/stable/book/ch11-00-testing.html)

Critique before rewrite:

The Exercises chapter already had a ladder, practice map, failure-signal table,
evidence map, worked examples, and an answer key. The remaining practice
transfer risk was feedback capture. The scorecard correctly said exercise
difficulty should be tuned from attempt evidence, but readers did not yet have
a concrete shape for recording an attempt.

ML teaching critique:

Exercise feedback should record the command, first failure signal, and answer
key mismatch. Otherwise an ML learner's report can become too vague to improve
the worked examples, loss traces, or training-step explanations.

Category-theory critique:

For abstract topics, feedback needs to identify whether the failure was the
Rust boundary, the ML/software role, or the category-theory shape. A structured
attempt report keeps those lenses separate.

Learner critique:

A learner can know that an exercise felt unclear without knowing what to report.
The rewrite gives a short attempt-record template before answer-key comparison.

Rewrite decision:

- add `Exercise Attempt Record` to the book exercise chapter,
- require exercise, chapter, command, first failure signal, confusing line,
  expected behavior, observed behavior, answer-key mismatch, and suggested
  rewrite,
- connect the template from `exercises/README.md`,
- add facilitator guidance in `exercises/ANSWER_KEY.md`,
- strengthen `scripts/check-exercise-alignment.py` so the attempt-record
  markers and answer-key guidance stay present,
- update the scorecard focus to validate the template with real
  exercise-attempt evidence.

Validation:

```bash
python3 scripts/check-exercise-alignment.py
python3 scripts/check-exercise-commands.py
python3 scripts/check-prose-style.py
python3 scripts/check-duplicate-prose.py
mdbook test
```

Result:

```text
Exercise alignment check passed.
Exercise command check passed.
Prose style check passed.
Duplicate prose check passed.
Chapter scorecard check passed.
Rewrite log check passed.
Book chapter tests passed.
```

## Sweep 2 Pass 83: Seven Sketches Core-Transfer Guide

Files:

```text
book/src/seven-sketches-rust.md
book/CHAPTER_QUALITY_SCORECARD.md
book/CHAPTER_REWRITE_LOG.md
```

Primary sources:

- [Seven Sketches in Compositionality](https://arxiv.org/abs/1803.05316)
- [Compositional Deep Learning](https://arxiv.org/abs/1907.08292)
- [Category Theory for Programming](https://arxiv.org/abs/2209.01259)
- [Rust Book: Traits](https://doc.rust-lang.org/book/ch10-02-traits.html)

Critique before rewrite:

The Seven Sketches chapter already had transfer tasks after each sketch and a
law-and-boundary index. The remaining learner-path risk was sequencing. A
reader who came for tiny ML could see eight applied models and not know which
ones are core transfer for ML structure and which ones are useful detours.

ML teaching critique:

The chapter needed a first-pass route that keeps the tiny ML thread visible.
Information order, database validity, feasibility, signal matrices, and
component boundaries transfer directly into ML pipeline thinking. Resource and
local-behavior sketches are useful, but less central to the first model.

Category-theory critique:

The chapter should preserve the breadth of the source material without turning
the source tour into an undifferentiated list. Each sketch should be read as one
kind of compositional boundary, not as a claim that every topic is equally
central to the tiny ML path.

Learner critique:

A motivated learner could ask, "Which of these should I read deeply now?" The
rewrite answers with a core-transfer table and a single exit sentence:
`This Rust model prevents this invalid composition.`

Rewrite decision:

- add `Choose A Sketch Without Losing The Tiny ML Thread`,
- classify each sketch by what it helps with,
- map each sketch back to tiny ML transfer,
- mark safe first-pass treatment as core transfer, optional but useful, or
  optional but practical,
- update the scorecard focus to validate the guide with readers who came for
  tiny ML.

Validation:

```bash
python3 scripts/check-prose-style.py
python3 scripts/check-chapter-scorecard.py
python3 scripts/check-duplicate-prose.py
mdbook test
```

Result:

```text
Prose style check passed.
Chapter scorecard check passed.
Duplicate prose check passed.
Book chapter tests passed.
```

## Sweep 2 Pass 82: Structure Law-Test-Analogy Boundary

Files:

```text
book/src/05-structure-and-calculus.md
book/CHAPTER_QUALITY_SCORECARD.md
book/CHAPTER_REWRITE_LOG.md
```

Primary sources:

- [Seven Sketches in Compositionality](https://arxiv.org/abs/1803.05316)
- [Category Theory for Programming](https://arxiv.org/abs/2209.01259)
- [Dive into Deep Learning: Forward Propagation, Backward Propagation, and Computational Graphs](https://d2l.ai/chapter_multilayer-perceptrons/backprop.html)
- [Backprop as Functor](https://arxiv.org/abs/1711.10455)

Critique before rewrite:

The Structure and Calculus chapter already had a two-path trace, a law and
boundary map, and warnings that tests are not full mathematical proofs. The
remaining category-precision risk was that a reader could still treat every
law-shaped word, executable test, and larger-ML comparison as the same kind of
claim.

ML teaching critique:

The chapter needed to protect transfer without overclaiming. A local chain-rule
test for multiplication can support intuition for backpropagation, but it is
not a full neural-network training proof.

Category-theory critique:

The chapter needed a visible epistemic boundary: which statements are category
laws, which statements are concrete repository tests, and which statements are
engineering analogies for transfer.

Learner critique:

A learner can either distrust the chapter because tests are not proofs, or
overtrust the chapter because tests have names like `naturality` and `monoid`.
The rewrite gives a table for reading the claims at the right strength.

Rewrite decision:

- add `What Is A Law, A Test, Or An Analogy?`,
- map each major claim to repository evidence,
- label concrete tests as examples or checks rather than universal proofs,
- mark larger-ML transfer as an analogy until a larger typed implementation
  exists,
- update the scorecard focus to validate this boundary with a category-theory
  reader report.

Validation:

```bash
python3 scripts/check-prose-style.py
python3 scripts/check-chapter-scorecard.py
python3 scripts/check-duplicate-prose.py
mdbook test
```

Result:

```text
Prose style check passed.
Chapter scorecard check passed.
Duplicate prose check passed.
Book chapter tests passed.
```

## Sweep 2 Pass 81: Training Gradient-Sign Example

Files:

```text
book/src/04-training-endomorphism.md
book/CHAPTER_QUALITY_SCORECARD.md
book/CHAPTER_REWRITE_LOG.md
```

Primary sources:

- [Dive into Deep Learning: Gradient Descent](https://d2l.ai/chapter_optimization/gd.html)
- [Dive into Deep Learning: Forward Propagation, Backward Propagation, and Computational Graphs](https://d2l.ai/chapter_multilayer-perceptrons/backprop.html)
- [Stanford CS231n: Optimization](https://cs231n.github.io/optimization-1/)
- [Backprop as Functor](https://arxiv.org/abs/1711.10455)

Critique before rewrite:

The Training as an Endomorphism chapter already separated measurement
(`Parameters -> Loss`) from update (`Parameters -> Parameters`). It also showed
`dlogits[target_id] -= 1.0` and the parameter update rule. The remaining learner
risk was the sign transition: if the code subtracts gradients, a reader may not
see why the target class can increase.

ML teaching critique:

The chapter needed one numeric example that starts from probabilities, applies
the target correction, then follows the sign into the bias update. This keeps
gradient descent concrete before the full matrix and embedding loops appear.

Category-theory critique:

The endomorphism claim should stay tied to state transition, not to a vague
optimization metaphor. The new example keeps the local derivative as internal
machinery and returns to the visible `Parameters -> Parameters` state change.

Learner critique:

A learner could memorize `parameter - learning_rate * gradient` while missing
that subtracting a negative gradient increases the target parameter. The
rewrite makes that sign flip explicit with one class-probability vector.

Rewrite decision:

- add `Worked Example: Why Subtracting A Negative Gradient Increases The Target`,
- reuse the three-class probability vector from the logit-gradient section,
- compute the target-corrected `dlogits`,
- show the bias update for learning rate `0.1`,
- explain why non-target biases decrease and the target bias increases,
- tie the Rust path to `dlogits -> grad_bias -> bias -= learning_rate * grad * batch_scale`,
- update the scorecard focus to validate the gradient-sign example with an ML
  learner report.

Validation:

```bash
python3 scripts/check-prose-style.py
python3 scripts/check-chapter-scorecard.py
python3 scripts/check-duplicate-prose.py
mdbook test
```

Result:

```text
Prose style check passed.
Chapter scorecard check passed.
Duplicate prose check passed.
Book chapter tests passed.
```

## Sweep 2 Pass 80: Tiny ML Target-Index Loss Example

Files:

```text
book/src/03-ml-pipeline.md
book/CHAPTER_QUALITY_SCORECARD.md
book/CHAPTER_REWRITE_LOG.md
```

Critique before rewrite:

The Tiny ML Pipeline chapter already separated logits, distributions, target
probability, and loss. It also showed that lower target probability produces
higher loss. The remaining learner risk was a common shortcut: using the
largest probability in the distribution instead of the probability assigned to
the actual target token.

ML teaching critique:

The chapter needed to make wrong confidence visible. A model can assign the
largest probability to the wrong token, so a loss explanation must point to the
target token index before applying `-ln`.

Category-theory critique:

The product object claim is strongest when it prevents a concrete confusion.
`Product<Distribution, TokenId>` should read as the boundary that carries both
the predicted distribution and the target needed to select the right
probability.

Learner critique:

A learner could read "loss uses probability" and silently choose the maximum
probability. The rewrite needed a small counterexample where the maximum
probability and the target probability are different.

Rewrite decision:

- add `Worked Example: Do Not Use The Largest Probability`,
- show a distribution where index `0` has probability `0.60` but the target is
  index `1`,
- compute the correct loss from `0.30`,
- show the incorrect shortcut from `0.60`,
- explain that the shortcut rewards confidence in the wrong token,
- tie the Rust boundary to `Product<Distribution, TokenId>` and
  `target.index()`,
- update the scorecard focus to validate the new target-index example with an
  ML learner report.

Validation:

```bash
python3 scripts/check-prose-style.py
python3 scripts/check-chapter-scorecard.py
python3 scripts/check-duplicate-prose.py
mdbook test
```

Result:

```text
Prose style check passed.
Chapter scorecard check passed.
Duplicate prose check passed.
Book chapter tests passed.
```

## Sweep 2 Pass 75: Public Chapter Maturity Gate

Files:

```text
README.md
book/STYLE_GUIDE.md
book/TEXTBOOK_COMPLETION_AUDIT.md
scripts/check-chapter-maturity.py
scripts/check-completion-audit.py
scripts/check.sh
```

Critique before rewrite:

The README already told readers that the project is a working public draft and
that some chapters are stable, draft, or sketch material. It also had a chapter
maturity table. The weak boundary was that the table was not mechanically tied
to the actual core chapter set, so a future chapter addition or status change
could leave public readers with stale feedback guidance.

Rewrite decision:

- expand the README chapter maturity table to cover the core chapter path from
  Welcome through Transformer Roadmap,
- keep every row tied to defined public status labels,
- require useful feedback guidance for each chapter,
- add `scripts/check-chapter-maturity.py`,
- check the README maturity table against `book/src/SUMMARY.md`,
- wire the checker into `scripts/check.sh`,
- add the maturity contract to the completion audit and style guide.

Validation:

```bash
python3 scripts/check-chapter-maturity.py
python3 scripts/check-completion-audit.py
python3 scripts/check-rewrite-log.py
```

Result:

```text
Chapter maturity check passed.
Completion audit check passed.
Rewrite log check passed.
```

## Sweep 2 Pass 76: Reference Link Freshness Workflow

Files:

```text
scripts/check-reference-links-live.py
scripts/check.sh
scripts/check-completion-audit.py
book/STYLE_GUIDE.md
book/TEXTBOOK_COMPLETION_AUDIT.md
book/CHAPTER_REWRITE_LOG.md
```

Critique before rewrite:

The research map now separates authoritative sources from learner-friction
signals, but the audit still described periodic refresh as a manual gap. A full
network check would make the normal publication gate flaky, because external
websites can rate-limit, redirect, or fail independently of this repository.
The missing boundary was an explicit optional live-check command plus a stable
self-test for the parser and failure semantics.

Rewrite decision:

- add `scripts/check-reference-links-live.py`,
- scan the reference chapter, editorial research notes, and public friction
  synthesis for external source URLs,
- require HTTPS and reject insecure redirects during live checks,
- provide a deterministic `--self-test` for URL extraction and failure handling,
- wire only the self-test into `scripts/check.sh`,
- document the optional live command in the style guide and completion audit.

Validation:

```bash
python3 scripts/check-reference-links-live.py --self-test
python3 scripts/check-reference-links-live.py --timeout 8
python3 scripts/check-source-authority.py
python3 scripts/check-chapter-references.py
python3 scripts/check-completion-audit.py
python3 scripts/check-rewrite-log.py
```

Result:

```text
Reference link live self-test passed.
Reference link live check passed: 115 source URL(s) reachable.
Source authority check passed.
Chapter reference coverage check passed.
Completion audit check passed.
Rewrite log check passed.
```

## Sweep 2 Pass 77: Chapter Contract Manifest

Files:

```text
book/CHAPTER_CONTRACTS.md
scripts/check-chapter-contracts.py
scripts/check.sh
scripts/check-completion-audit.py
book/STYLE_GUIDE.md
book/TEXTBOOK_COMPLETION_AUDIT.md
book/CHAPTER_REWRITE_LOG.md
```

Critique before rewrite:

The chapter reference map, maturity table, source buckets, exercises, and
rewrite log were all checked, but they lived as separate editorial surfaces.
A chapter rewrite could still accidentally update one surface while leaving
another stale. The missing boundary was a compact chapter contract manifest:
one row per core chapter naming the maturity, reference-map question, source
bucket, code evidence, runnable evidence, and practice evidence.

Rewrite decision:

- add `book/CHAPTER_CONTRACTS.md`,
- add `scripts/check-chapter-contracts.py`,
- check every core chapter from Welcome through Transformer Roadmap,
- verify contract maturity against the README maturity table,
- verify contract questions against the chapter reference map,
- verify source buckets against `book/EDITORIAL_RESEARCH_NOTES.md`,
- verify code evidence paths exist,
- verify runnable evidence is one of the intended chapter commands,
- verify practice evidence remains connected to the core exercise map,
- wire the checker into `scripts/check.sh`,
- add the chapter-contract row to the completion audit and style guide.

Validation:

```bash
python3 scripts/check-chapter-contracts.py
python3 scripts/check-completion-audit.py
python3 scripts/check-chapter-references.py
python3 scripts/check-exercise-alignment.py
```

Result:

```text
Chapter contract check passed.
Completion audit check passed.
Chapter reference coverage check passed.
Exercise alignment check passed.
```

## Sweep 2 Pass 78: Chapter Quality Scorecard

Files:

```text
book/CHAPTER_QUALITY_SCORECARD.md
scripts/check-chapter-scorecard.py
scripts/check.sh
scripts/check-completion-audit.py
book/STYLE_GUIDE.md
book/TEXTBOOK_COMPLETION_AUDIT.md
book/CHAPTER_REWRITE_LOG.md
```

Critique before rewrite:

The chapter contracts make each chapter auditable, but they do not prioritize
the next rewrite. The rewrite log proves that critique happened, yet a future
editor still has to infer which dimensions are strong and which dimensions need
reader evidence or sharper explanation. The missing boundary was a scorecard
that makes chapter quality explicit without claiming completion.

Rewrite decision:

- add `book/CHAPTER_QUALITY_SCORECARD.md`,
- score every core chapter across source grounding, Rust clarity, ML
  intuition, category precision, learner path, and practice transfer,
- keep direct-reader evidence marked `Pending` while the inbox has no accepted
  direct reports,
- add `scripts/check-chapter-scorecard.py`,
- align scorecard rows with `book/CHAPTER_CONTRACTS.md`,
- reject direct-reader evidence claims before accepted inbox reports exist,
- wire the checker into `scripts/check.sh`,
- add the scorecard row to the completion audit and style guide.

Validation:

```bash
python3 scripts/check-chapter-scorecard.py
python3 scripts/check-completion-audit.py
python3 scripts/check-chapter-contracts.py
python3 scripts/check-reader-feedback-inbox.py
```

Result:

```text
Chapter scorecard check passed.
Completion audit check passed.
Chapter contract check passed.
Reader feedback inbox check passed with no direct reports recorded.
```

## Sweep 2 Pass 79: Morphism Broken-Pipeline Example

Files:

```text
book/src/02-morphisms-composition.md
book/CHAPTER_QUALITY_SCORECARD.md
book/CHAPTER_REWRITE_LOG.md
```

Critique before rewrite:

The Morphism and Composition chapter already explained typed transformations,
but the scorecard still marked Rust clarity, ML intuition, category precision,
learner path, and practice transfer as developing. The chapter had a simple
function-as-arrow example and later explained `Compose<F, G, Middle>`, but a
reader could still meet the formal composition adapter before seeing a concrete
broken ML pipeline.

Rewrite decision:

- add `Worked Example: Where Composition Breaks`,
- show the legal `TokenId -> Vector -> Logits -> Distribution` path,
- name each stage as a typed arrow,
- show the tempting but illegal `TokenId -> Vector -> Distribution` shortcut,
- explain that `Softmax` consumes `Logits`, not `Vector`,
- keep the scorecard direct-reader evidence as `Pending`,
- update the scorecard focus to validate the new example with a reader before
  upgrading clarity.

Validation:

```bash
python3 scripts/check-prose-style.py
python3 scripts/check-chapter-scorecard.py
python3 scripts/check-duplicate-prose.py
mdbook test
python3 scripts/check-chapter-contracts.py
python3 scripts/check-rewrite-log.py
git diff --check -- . ':!target'
```

Result:

```text
Prose style check passed.
Chapter scorecard check passed.
Duplicate prose check passed.
Book chapter tests passed.
Chapter contract check passed.
Rewrite log check passed.
Diff whitespace check passed.
```

Validation note:

```text
mdbook test book/src/02-morphisms-composition.md
```

is not a valid command for this repository because `mdbook test` expects the
book root and looks for `src/SUMMARY.md` under the supplied path. Use
`mdbook test` from the repository root instead.

## Sweep 2 Pass 72: Completion-Audit Checker Coverage

Files:

```text
scripts/check-completion-audit.py
book/CHAPTER_REWRITE_LOG.md
```

Critique before rewrite:

The completion audit text had grown more precise than its validator. It named
newer evidence rows for diagrams, law and boundary tables, glossary consistency,
chapter-practice alignment, duplicate-prose checks, typed attention roadmap
material, and reviewer-slot validation. But the checker did not yet require all
of those rows and commands. That left a maintenance gap: a future edit could
remove important audit evidence while `scripts/check-completion-audit.py` still
passed.

Rewrite decision:

- require the audit checklist rows for diagrams, law/boundary tables,
  terminology consistency, chapter-practice alignment, duplicate-prose sweep,
  and typed attention roadmap material,
- require the latest validation evidence to mention both reviewer-slot tracker
  commands,
- keep direct-reader completion as a separate strict gate rather than treating
  these proxy checks as completion.

Validation:

```bash
python3 scripts/check-completion-audit.py
python3 scripts/check-rewrite-log.py
```

Result:

```text
Completion audit check passed.
Rewrite log check passed.
```

## Sweep 2 Pass 73: Diagram Coverage Gate

Files:

```text
scripts/check-diagram-coverage.py
scripts/check.sh
scripts/check-completion-audit.py
book/TEXTBOOK_COMPLETION_AUDIT.md
book/CHAPTER_REWRITE_LOG.md
```

Critique before rewrite:

The completion audit said diagrams and flow traces were present in the README,
Course Map, Morphism/Composition, Tiny ML Pipeline, Training, Structure, and
Transformer Roadmap. That was true in the authored chapters, but the evidence
was not protected by a focused checker. A future rewrite could remove a core
diagram while the audit row and full gate still passed.

Rewrite decision:

- add `scripts/check-diagram-coverage.py`,
- require the README Mermaid pipeline,
- require the Course Map whole-pipeline flow,
- require the legal composition diagram in Morphism and Composition,
- require the Tiny ML data-preparation and prediction/loss diagram,
- require the Training parameter-update loop,
- require the Structure naturality-square and monoid-grouping diagrams,
- require the Transformer attention shape-flow diagram,
- wire the checker into `scripts/check.sh`,
- add the checker to completion-audit validation evidence.

Validation:

```bash
python3 scripts/check-diagram-coverage.py
python3 scripts/check-completion-audit.py
```

Result:

```text
Diagram coverage check passed.
Completion audit check passed.
```

## Sweep 2 Pass 74: Source Authority Boundary Gate

Files:

```text
scripts/check-source-authority.py
scripts/check.sh
scripts/check-completion-audit.py
book/TEXTBOOK_COMPLETION_AUDIT.md
book/CHAPTER_REWRITE_LOG.md
```

Critique before rewrite:

The book uses two different kinds of external material: authoritative sources
for definitions and technical claims, and public community posts as learner
friction signals. The prose stated that distinction, but there was no focused
guard preventing Reddit or Stack Overflow links from drifting into the
learner-facing authoritative reference chapter.

Rewrite decision:

- add `scripts/check-source-authority.py`,
- require source-priority language in the editorial research notes,
- reject community domains from `book/src/references.md`,
- keep community domains allowed only in the learner-friction synthesis or its
  dedicated editorial section,
- require canonical official, academic, open-textbook, and DOI source markers,
- require HTTPS source URLs,
- wire the checker into `scripts/check.sh`,
- add the checker to completion-audit validation evidence.

Validation:

```bash
python3 scripts/check-source-authority.py
python3 scripts/check-completion-audit.py
```

Result:

```text
Source authority check passed.
Completion audit check passed.
```

## Sweep 2 Pass 53: Editorial Source-Bucket Contracts

Files:

```text
book/EDITORIAL_RESEARCH_NOTES.md
book/TEXTBOOK_COMPLETION_AUDIT.md
scripts/check-chapter-references.py
```

Primary sources:

- [Seven Sketches in Compositionality](https://arxiv.org/abs/1803.05316)
- [Rust Book: Writing Automated Tests](https://doc.rust-lang.org/stable/book/ch11-00-testing.html)
- [How People Learn II](https://www.nationalacademies.org/publications/24783)
- [Structuring the Transition From Example Study to Problem Solving](https://doi.org/10.1207/S15326985EP3801_3)

Critique before rewrite:

The public reference map already named sources for Seven Sketches and Exercises,
but the editorial research notes did not give those chapters their own source
buckets and rewrite contracts. That made future rewrites easier to run from the
table of references alone, without the chapter-specific editorial questions
that keep source use pedagogical rather than decorative.

Rewrite decision:

- add a `Seven Sketches Through Rust` source bucket with applied-category,
  programming-category, compositional-learning, enum, and trait references,
- add an `Exercises And Transfer` source bucket with learning-science, Rust
  testing, and gradient-checking references,
- give both buckets concrete rewrite checks,
- strengthen `scripts/check-chapter-references.py` so editorial source buckets
  are mechanically required, each bucket has at least three external sources,
  and each bucket includes a rewrite check,
- update the completion audit so reference organization names both the public
  reference map and the editorial source-bucket contracts.

Validation:

```bash
python3 scripts/check-chapter-references.py
python3 scripts/check-public-friction-matrix.py
python3 scripts/check-completion-audit.py
```

Result:

```text
Chapter reference coverage check passed.
Public friction matrix check passed.
Completion audit check passed.
```

## Sweep 2 Pass 50: Reviewer Slot Evidence Consistency

Files:

```text
community/reviewer-slot-tracker.md
scripts/check-reviewer-slot-tracker.py
scripts/check.sh
book/TEXTBOOK_COMPLETION_AUDIT.md
```

Critique before rewrite:

The reviewer-slot tracker gave the public project a clean way to organize the
five required external reader contexts, but the status table was still a
human-maintained surface. Without an automated consistency check, a slot could
be marked `accepted` or `rewritten` before the reader-feedback inbox contained
matching direct evidence. That would make the sprint look more complete than it
really is.

Rewrite decision:

- add `scripts/check-reviewer-slot-tracker.py`,
- parse the tracker status table by reviewer context,
- reuse the reader-feedback inbox validator instead of duplicating inbox rules,
- require every required reviewer context to appear in the tracker,
- reject `accepted` and `rewritten` tracker states without matching accepted
  inbox evidence,
- require a `rewritten` tracker state when an inbox report has already driven a
  rewrite,
- add fixture coverage for complete, stale, missing-evidence, and
  missing-rewrite cases,
- wire the checker and its self-test into `scripts/check.sh`.

Validation:

```bash
python3 scripts/check-reviewer-slot-tracker.py
python3 scripts/check-reviewer-slot-tracker.py --self-test
python3 scripts/check-completion-audit.py
```

Result:

```text
Reviewer slot tracker check passed.
Reviewer slot tracker self-test passed.
Completion audit check passed.
```

## Sweep 2 Pass 71: Reviewer Slot Tracker

Files:

```text
community/reviewer-slot-tracker.md
community/reviewer-outreach.md
community/reader-review-sprint.md
community/reader-feedback-inbox.md
community/feedback-wall.md
scripts/check-reader-feedback-loop.py
book/TEXTBOOK_COMPLETION_AUDIT.md
```

Primary sources:

- [GitHub issue forms](https://docs.github.com/en/communities/using-templates-to-encourage-useful-issues-and-pull-requests/syntax-for-issue-forms)
- [reader-confusion issue form](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=reader-confusion.yml)

Critique before rewrite:

The strict completion gate correctly remains open because the project has no
accepted direct reader reports. The review packet, outreach asks, sprint guide,
inbox, and triage protocol were connected, but there was not yet a public
slot-level view of the five required reviewer contexts. That can make the next
operational action less obvious: which context is still open, invited,
received, accepted, or rewritten?

Rewrite decision:

- add `community/reviewer-slot-tracker.md` with one public slot per required
  reviewer context,
- explicitly forbid names, email addresses, private messages, and personal
  notes in the public tracker,
- link the tracker from outreach, sprint, inbox, and feedback-wall surfaces,
- update the sprint procedure so slot status changes follow real evidence,
- strengthen `scripts/check-reader-feedback-loop.py` so the tracker, statuses,
  reviewer contexts, privacy guard, inbox link, triage link, and strict
  completion command stay present,
- update the completion audit evidence and missing-work list.

Validation:

```bash
python3 scripts/check-reader-feedback-loop.py
python3 scripts/check-reader-feedback-inbox.py
python3 scripts/check-completion-audit.py
python3 scripts/check-prose-style.py
```

Result:

```text
Reader feedback loop check passed.
Reader feedback inbox check passed with no direct reports recorded.
Completion audit check passed.
Prose style check passed.
```

The full publication gate then passed with 106 Rust tests, all examples, the
main demo, prose style checks, completion-audit checks, reader-feedback-loop
checks, rewrite-log checks, chapter reference coverage checks, exercise
alignment checks, exercise command checks, duplicate-prose checks, source
coverage checks, book build, and chapter tests.

## Sweep 2 Pass 70: Transformer Role Ownership Map

Files:

```text
book/src/roadmap.md
scripts/check-public-friction-matrix.py
```

Primary sources:

- [The Illustrated Transformer](https://jalammar.github.io/illustrated-transformer/)
- [Attention Is All You Need](https://arxiv.org/abs/1706.03762)
- [Dive into Deep Learning: Attention and Transformers](https://d2l.ai/chapter_attention-mechanisms-and-transformers/index.html)

Critique before rewrite:

The public-friction matrix asks whether the Transformer Roadmap lets a reader
trace which typed stage owns each Transformer role before full block training
appears. The chapter already had a strong implementation-status table and a
detailed attention path, but the first scan point did not explicitly separate
roles from raw vector/matrix representations. That is exactly where Transformer
learners often lose the thread: query, key, value, score, mask, weight, and
hidden-state objects all look numerically similar until the roles are named.

Rewrite decision:

- add `## Transformer Role Ownership Map` before the implementation-status
  table,
- map each Transformer role to the Rust owner, boundary shape, and concrete
  confusion prevented,
- include Q/K/V, scores, masks, weights, value mixing, multi-head output,
  projection, residual, normalization, feed-forward, masked block, readout, and
  training state,
- add a three-question debugging prompt for later formulas,
- strengthen `scripts/check-public-friction-matrix.py` so the role map remains
  before `## What Exists Now` and keeps the key role/confusion markers.

Validation:

```bash
python3 scripts/check-public-friction-matrix.py
python3 scripts/check-prose-style.py
python3 scripts/check-duplicate-prose.py
python3 scripts/check-chapter-references.py
```

Result:

```text
Public friction matrix check passed.
Prose style check passed.
Duplicate prose check passed.
Chapter reference coverage check passed.
```

The full publication gate then passed with 106 Rust tests, all examples, the
main demo, prose style checks, completion-audit checks, reader-feedback-loop
checks, rewrite-log checks, chapter reference coverage checks, exercise
alignment checks, exercise command checks, duplicate-prose checks, source
coverage checks, book build, and chapter tests.

## Sweep 2 Pass 68: Seven Sketches Transfer Tasks

Files:

```text
book/src/seven-sketches-rust.md
scripts/check-public-friction-matrix.py
```

Primary sources:

- [Seven Sketches in Compositionality](https://arxiv.org/abs/1803.05316)
- [John D. Cook: Category Theory for Programmers](https://www.johndcook.com/blog/2020/11/02/category-theory-for-programmers/)

Critique before rewrite:

The public-friction matrix asks whether each applied sketch ends with a concrete
Rust transfer task. The chapter already had a strong protected-structure table,
source walkthrough, and final retrieval practice, but most transfer work was
grouped near the end. That means a reader could finish a sketch with a useful
definition but no immediate prompt to move the pattern into their own Rust
model.

Rewrite decision:

- add a concrete transfer task after the information-order sketch,
- add a transfer task after the feature/layer Galois-law sketch,
- add a resource-bundle transfer task,
- add a database foreign-key transfer task,
- add a feasibility-relation transfer task,
- add a signal-matrix shape-safe-composition transfer task,
- add an open-circuit interface-boundary transfer task,
- add a local-checks-to-global-claim transfer task,
- strengthen `scripts/check-public-friction-matrix.py` so every sketch section
  keeps its task and key concrete markers.

Validation:

```bash
python3 scripts/check-public-friction-matrix.py
python3 scripts/check-prose-style.py
python3 scripts/check-duplicate-prose.py
python3 scripts/check-chapter-references.py
```

Result:

```text
Public friction matrix check passed.
Prose style check passed.
Duplicate prose check passed.
Chapter reference coverage check passed.
```

## Sweep 2 Pass 69: Exercise Evidence Map

Files:

```text
book/src/exercises.md
scripts/check-public-friction-matrix.py
```

Primary sources:

- [Rustlings Usage](https://rustlings.rust-lang.org/usage/)
- [Stanford CS231n: Optimization](https://cs231n.github.io/optimization-1/)
- [How People Learn II](https://www.nationalacademies.org/projects/DBASSE-BBCSS-13-06/publication/24783)

Critique before rewrite:

The public-friction matrix asks whether major exercises include concrete
feedback signals, not only prompts. The chapter already had pass conditions,
failure signals, and commands inside individual exercises, but a reader had to
scan the whole chapter to know what evidence counted for each exercise. That
weakens the Rustlings-style feedback loop.

Rewrite decision:

- add `## Exercise Evidence Map` before the worked examples,
- list Exercise 1 through Exercise 14 with a command, output, failure shape, or
  written evidence target,
- explicitly connect progress evidence to command output, constructor errors,
  compiler errors, and named tests,
- strengthen `scripts/check-public-friction-matrix.py` so every exercise number
  and the important command/failure markers remain present.

Validation:

```bash
python3 scripts/check-public-friction-matrix.py
python3 scripts/check-prose-style.py
python3 scripts/check-duplicate-prose.py
python3 scripts/check-exercise-alignment.py
python3 scripts/check-exercise-commands.py
```

Result:

```text
Public friction matrix check passed.
Prose style check passed.
Duplicate prose check passed.
Exercise alignment check passed.
Exercise command check passed.
```

The full publication gate then passed with 106 Rust tests, all examples, the
main demo, prose style checks, completion-audit checks, reader-feedback-loop
checks, rewrite-log checks, chapter reference coverage checks, exercise
alignment checks, exercise command checks, duplicate-prose checks, source
coverage checks, book build, and chapter tests.

## Sweep 2 Pass 66: Training Update Trace

Files:

```text
book/src/04-training-endomorphism.md
community/external-feedback-synthesis.md
book/src/references.md
book/EDITORIAL_RESEARCH_NOTES.md
scripts/check-public-friction-matrix.py
```

Primary sources:

- [Dive into Deep Learning: Gradient Descent](https://d2l.ai/chapter_optimization/gd.html)
- [Stanford CS231n: Optimization](https://cs231n.github.io/optimization-1/)
- [Dive into Deep Learning: Backpropagation and Computational Graphs](https://d2l.ai/chapter_multilayer-perceptrons/backprop.html)

Critique before rewrite:

The public-friction matrix asks whether the training chapter separates one
`Parameters -> Parameters` update from repeated optimization. The chapter
already said training is an endomorphism, but the source snapshot still arrived
before the reader saw the concrete update trace. That left room for a learner to
mix up measuring loss, computing gradients, applying the learning rate, and
repeating the step.

Rewrite decision:

- add `## Update Trace Before Source` before the `src/training.rs` snapshot,
- map `Parameters`, `TrainingSet`, the forward pass, `dlogits`, gradient
  buffers, `batch_scale`, `LearningRate`, and the returned `Parameters`,
- state that `Parameters -> Loss` measures the model while `Parameters ->
  Parameters` updates the model,
- show the local update rule
  `parameter = parameter - learning_rate * average_gradient`,
- connect the repeated-training claim to `apply_endomorphism_n_times`,
- add CS231n Optimization to the Training chapter reference surface,
- strengthen `scripts/check-public-friction-matrix.py` so the update trace
  remains before the source snapshot and keeps the one-step-vs-repeated-step
  markers.

Validation:

```bash
python3 scripts/check-public-friction-matrix.py
python3 scripts/check-prose-style.py
python3 scripts/check-duplicate-prose.py
python3 scripts/check-chapter-references.py
```

Result:

```text
Public friction matrix check passed.
Prose style check passed.
Duplicate prose check passed.
Chapter reference coverage check passed.
```

The full publication gate then passed with 106 Rust tests, all examples, the
main demo, prose style checks, completion-audit checks, reader-feedback-loop
checks, rewrite-log checks, chapter reference coverage checks, exercise
alignment checks, exercise command checks, duplicate-prose checks, source
coverage checks, book build, and chapter tests.

## Sweep 2 Pass 67: Structure Two-Path Trace

Files:

```text
book/src/05-structure-and-calculus.md
scripts/check-public-friction-matrix.py
```

Primary sources:

- [Dive into Deep Learning: Backpropagation and Computational Graphs](https://d2l.ai/chapter_multilayer-perceptrons/backprop.html)
- [The Matrix Calculus You Need For Deep Learning](https://arxiv.org/abs/1802.01528)
- [Backprop as Functor](https://arxiv.org/abs/1711.10455)

Critique before rewrite:

The public-friction matrix asks whether the structure chapter forces the reader
to trace both paths before the formal law name appears. The chapter had strong
sections on functors, naturality, monoids, and the chain rule, but the source
snapshots still came before one compact two-path orientation. A reader could
therefore encounter `Functor`, `NaturalTransformation`, `Monoid`, and chain-rule
language before seeing the concrete agreement each name is meant to compress.

Rewrite decision:

- add `## Trace Both Paths Before The Names` before the source snapshots,
- show the naturality paths
  `Vec<A> -> Vec<B> -> Option<B>` and `Vec<A> -> Option<A> -> Option<B>`,
- show the two monoid groupings for the pipeline trace,
- show the forward multiplication path and backward derivative path,
- state `dL/dx = dL/dz * y` and `dL/dy = dL/dz * x` before the longer source
  walkthrough,
- strengthen `scripts/check-public-friction-matrix.py` so this two-path trace
  remains before the source snapshots.

Validation:

```bash
python3 scripts/check-public-friction-matrix.py
python3 scripts/check-prose-style.py
python3 scripts/check-duplicate-prose.py
python3 scripts/check-chapter-references.py
```

Result:

```text
Public friction matrix check passed.
Prose style check passed.
Duplicate prose check passed.
Chapter reference coverage check passed.
```

## Sweep 2 Pass 65: Tiny ML Prediction Trace

Files:

```text
book/src/03-ml-pipeline.md
community/external-feedback-synthesis.md
book/TEXTBOOK_COMPLETION_AUDIT.md
scripts/check-public-friction-matrix.py
```

Primary sources:

- [Dive into Deep Learning: Softmax Regression](https://d2l.ai/chapter_linear-classification/softmax-regression.html)
- [Dive into Deep Learning: Softmax Regression from Scratch](https://d2l.ai/chapter_linear-classification/softmax-regression-scratch.html)
- [CS231n: Linear Classification](https://cs231n.github.io/linear-classify/)

Critique before rewrite:

The public-friction matrix asks whether the Tiny ML Pipeline chapter shows
logits before probabilities and target-index loss before formulas dominate.
The chapter already had a useful `Scores, Probabilities, And Loss` section, but
it came after the source snapshot. A reader could enter the implementation
before seeing the most important debugging distinction: logits are not
probabilities, and cross entropy uses the probability assigned to the target
token index.

Rewrite decision:

- add `## Prediction Trace Before Source` before the `src/ml.rs` snapshot,
- map `TokenId`, `Vector`, `Logits`, `Distribution`,
  `Product<Distribution, TokenId>`, and `Loss` to plain meaning and checks,
- state the target-index rule before the longer implementation walkthrough,
- show the compact trace `Logits -> Distribution -> target probability ->
  Loss`,
- strengthen `scripts/check-public-friction-matrix.py` so this trace remains
  present before the source snapshot and keeps the required target-probability
  markers.

Validation:

```bash
python3 scripts/check-public-friction-matrix.py
python3 scripts/check-prose-style.py
python3 scripts/check-duplicate-prose.py
python3 scripts/check-completion-audit.py
python3 scripts/check-rewrite-log.py
```

Result:

```text
Public friction matrix check passed.
Prose style check passed.
Duplicate prose check passed.
Completion audit check passed.
Rewrite log check passed.
```

## Sweep 2 Pass 64: Morphism Term-To-Code Bridge

Files:

```text
book/src/02-morphisms-composition.md
community/external-feedback-synthesis.md
book/TEXTBOOK_COMPLETION_AUDIT.md
scripts/check-public-friction-matrix.py
```

Primary sources:

- [The Rust Programming Language: Traits](https://doc.rust-lang.org/book/ch10-02-traits.html)
- [The Rust Programming Language: Generic Data Types](https://doc.rust-lang.org/book/ch10-01-syntax.html)
- [Category theory for programmers made easier](https://www.johndcook.com/blog/2020/11/02/category-theory-for-programmers/)
- [Stack Overflow: Do objects map to types or instances?](https://stackoverflow.com/questions/62838070/category-theory-to-computer-programming-do-objects-map-to-types-or-instances-o)

Critique before rewrite:

The public-friction matrix asks whether every category-theory term arrives
beside a real Rust function, trait, or type boundary. The Morphism and
Composition chapter already explained `Morphism`, `Identity`, `Compose`, and
`Endomorphism` in detail, but the source snapshot came before a compact
translation table. That meant readers could meet the generic implementation
before seeing a single map from formal term to Rust shape to tiny ML example.

Rewrite decision:

- add `## Category Terms As Rust Shapes` near the top of
  `book/src/02-morphisms-composition.md`,
- map object, morphism, source object, target object, identity morphism,
  composition, middle object, endomorphism, and repeated endomorphism to
  concrete Rust shapes,
- include tiny ML examples such as `TokenId`, `Vector`,
  `impl Morphism<TokenId, Vector> for Embedding`, `Compose<F, G, Middle>`,
  and `TrainStep : Parameters -> Parameters`,
- strengthen `scripts/check-public-friction-matrix.py` so the bridge appears
  before the source snapshot and keeps the required term-to-code markers.

Validation:

```bash
python3 scripts/check-public-friction-matrix.py
python3 scripts/check-prose-style.py
python3 scripts/check-duplicate-prose.py
python3 scripts/check-completion-audit.py
python3 scripts/check-rewrite-log.py
```

Result:

```text
Public friction matrix check passed.
Prose style check passed.
Duplicate prose check passed.
Completion audit check passed.
Rewrite log check passed.
```

## Sweep 2 Pass 63: Domain Object Mistake Map

Files:

```text
book/src/01-domain-objects.md
community/external-feedback-synthesis.md
book/TEXTBOOK_COMPLETION_AUDIT.md
scripts/check-public-friction-matrix.py
```

Primary sources:

- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/checklist.html)
- [The Rust Programming Language: Defining Structs](https://doc.rust-lang.org/book/ch05-01-defining-structs.html)
- [Brown experimental Rust Book](https://rust-book.cs.brown.edu/)

Critique before rewrite:

The public-friction matrix asks whether each newtype explains the concrete
mistake it prevents. The Domain Objects chapter already explained semantic
wrappers, validated objects, constructors, and accessors, but a learner still
had to infer the full mistake map while reading detailed syntax. That leaves
too much load on the first pass: readers see many names before seeing why each
name earns its place.

Rewrite decision:

- add `## Mistakes These Types Prevent` before the source snapshot,
- map `TokenId`, `TokenSequence`, `Vector`, `Logits`, `Distribution`, `Loss`,
  `VocabSize`, `ModelDimension`, `LearningRate`, `TrainingSet`, and
  `Parameters` to the raw representation and concrete mistake each type
  prevents,
- frame the table as the review checklist for the chapter,
- strengthen `scripts/check-public-friction-matrix.py` so the table, key
  domain types, and mistake markers remain present before the source snapshot.

Validation:

```bash
python3 scripts/check-public-friction-matrix.py
python3 scripts/check-prose-style.py
python3 scripts/check-duplicate-prose.py
python3 scripts/check-completion-audit.py
python3 scripts/check-rewrite-log.py
```

Result:

```text
Public friction matrix check passed.
Prose style check passed.
Duplicate prose check passed.
Completion audit check passed.
Rewrite log check passed.
```

## Sweep 2 Pass 62: Course Map Path Choice Rewrite

Files:

```text
book/src/00-map.md
community/external-feedback-synthesis.md
book/TEXTBOOK_COMPLETION_AUDIT.md
scripts/check-public-friction-matrix.py
```

Primary sources:

- [Rust Learn](https://rust-lang.org/learn/)
- [Brown experimental Rust Book](https://rust-book.cs.brown.edu/)
- [Rustlings usage guide](https://rustlings.rust-lang.org/usage/)

Critique before rewrite:

The public-friction matrix asks whether the Course Map reduces choice overload
by naming one first path and one code-first path. The chapter already mapped
the modules, pipeline, and vocabulary, but it still made a motivated reader
infer how to start. The map was accurate, but it did not yet act like a
navigation surface for readers who enter through code or through prose.

Rewrite decision:

- add a `## Choose Your Path` section near the top of `book/src/00-map.md`,
- name a book-first path from Welcome through Training as an Endomorphism,
- name a code-first path using `cargo run --example 01_token_sequence` and
  `cargo run --bin category_ml`,
- tell code-first readers to classify demo output as domain value, typed
  transformation, or training update,
- strengthen `scripts/check-public-friction-matrix.py` so the Course Map must
  keep the path-choice section before the module table and include both entry
  paths.

Validation:

```bash
python3 scripts/check-public-friction-matrix.py
python3 scripts/check-prose-style.py
python3 scripts/check-duplicate-prose.py
python3 scripts/check-completion-audit.py
python3 scripts/check-rewrite-log.py
```

Result:

```text
Public friction matrix check passed.
Prose style check passed.
Duplicate prose check passed.
Completion audit check passed.
Rewrite log check passed.
```

## Sweep 2 Pass 61: Welcome First-Run Rewrite

Files:

```text
book/src/welcome.md
community/external-feedback-synthesis.md
book/TEXTBOOK_COMPLETION_AUDIT.md
scripts/check-public-friction-matrix.py
```

Primary sources:

- [Rustlings usage guide](https://rustlings.rust-lang.org/usage/)
- [100 Exercises To Learn Rust](https://rust-exercises.com/100-exercises/01_intro/00_welcome)
- [Brown experimental Rust Book](https://rust-book.cs.brown.edu/)

Critique before rewrite:

The public-friction matrix asks whether a reader can run one command and
explain the first pipeline shape before the abstract promise grows. The Welcome
chapter already contained the `cargo run --example 01_token_sequence` command,
but it came after several paragraphs of positioning and the central thesis.
That meant the book agreed with the first-run principle in prose while still
asking a new reader to read theory before getting a runnable win.

Rewrite decision:

- move the first runnable command into a new `## First Win` section immediately
  after the title,
- show the short `Text -> TokenSequence -> TrainingPairs` shape before the
  broader `Text -> TokenSequence -> TrainingSet -> Prediction -> Loss ->
  Updated Parameters` path,
- keep the guided `cargo run --bin category_ml` walkthrough near the top,
- move the framework contrast and central thesis after the first command,
- strengthen `scripts/check-public-friction-matrix.py` so the Welcome first
  command must appear before broader framing and thesis sections.

Validation:

```bash
python3 scripts/check-public-friction-matrix.py
python3 scripts/check-prose-style.py
python3 scripts/check-duplicate-prose.py
python3 scripts/check-completion-audit.py
python3 scripts/check-rewrite-log.py
```

Result:

```text
Public friction matrix check passed.
Prose style check passed.
Duplicate prose check passed.
Completion audit check passed.
Rewrite log check passed.
```

## Sweep 2 Pass 60: Public Friction Matrix Gate

Files:

```text
community/external-feedback-synthesis.md
book/EDITORIAL_RESEARCH_NOTES.md
book/src/references.md
book/TEXTBOOK_COMPLETION_AUDIT.md
scripts/check-public-friction-matrix.py
scripts/check-completion-audit.py
scripts/check.sh
```

Primary sources:

- [Brown experimental Rust Book](https://rust-book.cs.brown.edu/)
- [Rustlings usage guide](https://rustlings.rust-lang.org/usage/)
- [100 Exercises To Learn Rust](https://rust-exercises.com/100-exercises/01_intro/00_welcome)
- [Category theory for programmers made easier](https://www.johndcook.com/blog/2020/11/02/category-theory-for-programmers/)
- [Stack Overflow: category theory objects and types](https://stackoverflow.com/questions/62838070/category-theory-to-computer-programming-do-objects-map-to-types-or-instances-o)
- [CS231n: Optimization](https://cs231n.github.io/optimization-1/)
- [CS231n: Neural Networks Part 3](https://cs231n.github.io/neural-networks-3/)
- [The Matrix Calculus You Need For Deep Learning](https://arxiv.org/abs/1802.01528)
- [The Illustrated Transformer](https://jalammar.github.io/illustrated-transformer/)

Critique before rewrite:

The direct reader gate is correctly open, but the proxy public-friction
synthesis was still mostly prose. It named useful signals from Rust learners,
category-theory learners, ML debugging notes, and Transformer explainers, but
it did not force every major chapter to translate those signals into a concrete
review question. That made the synthesis useful to read but easier to forget
during the next rewrite sweep.

Rewrite decision:

- expand `community/external-feedback-synthesis.md` with additional public
  sources for active Rust learning, category-theory type/object confusion,
  gradient checking, matrix-calculus support, and Transformer role/shape
  confusion,
- add a Transformer-specific friction theme about separating query/key/value
  roles, scores, masks, weights, mixed values, and later block components,
- add a Chapter Friction Matrix that maps each major chapter to a public
  friction signal, sources, and one review question,
- add `scripts/check-public-friction-matrix.py` so the matrix keeps required
  sections, sources, themes, chapter rows, and cross-links,
- wire the checker into `scripts/check.sh` and the completion audit,
- update the references and research notes so the new source-backed friction
  lens is part of future chapter rewrites.

Validation:

```bash
python3 scripts/check-public-friction-matrix.py
python3 scripts/check-completion-audit.py
python3 scripts/check-chapter-references.py
python3 scripts/check-rewrite-log.py
```

Result:

```text
Public friction matrix check passed.
Completion audit check passed.
Chapter reference coverage check passed.
Rewrite log check passed.
```

## Sweep 2 Pass 53: Reader Feedback Inbox Validator

Files:

```text
scripts/check-reader-feedback-inbox.py
scripts/check.sh
scripts/check-completion-audit.py
book/TEXTBOOK_COMPLETION_AUDIT.md
book/STYLE_GUIDE.md
community/reader-feedback-inbox.md
```

Primary sources:

- [GitHub Docs: Syntax for issue forms](https://docs.github.com/en/communities/using-templates-to-encourage-useful-issues-and-pull-requests/syntax-for-issue-forms)
- [How People Learn II](https://nap.nationalacademies.org/catalog/24783/how-people-learn-ii-learners-contexts-and-cultures)

Critique before rewrite:

The reader feedback inbox now existed, but it was only structurally protected
by the broader reader-feedback-loop checker. That was enough to keep the inbox
visible, not enough to validate accepted reader reports after they arrive. A
future report could omit the command tried, friction lens, triage action,
status, rewrite-log entry, or validation and still look like evidence.

Rewrite decision:

- add `scripts/check-reader-feedback-inbox.py`,
- ignore the template inside fenced code blocks,
- pass cleanly while no direct reports are recorded and the no-report marker is
  present,
- validate every future `### Report YYYY-MM-DD: <short title>` entry,
- require source, issue URL, review path, location, command or file tried,
  friction lens, confusing text or output, last clear idea, mental-model break,
  expected next step, smallest useful fix, triage action, affected chapter,
  repository code or example, references checked, rewrite-log entry,
  validation, and status,
- constrain status and triage action to known states,
- require rewritten reports to link back to `book/CHAPTER_REWRITE_LOG.md`,
- wire the checker into `scripts/check.sh` and the completion audit.

Validation:

```bash
python3 scripts/check-reader-feedback-inbox.py
python3 scripts/check-completion-audit.py
python3 scripts/check-reader-feedback-loop.py
git diff --check -- . ':!target'
```

Result:

```text
Reader feedback inbox check passed with no direct reports recorded.
Completion audit check passed.
Reader feedback loop check passed.
Diff whitespace check passed.
```

The full publication gate then passed with 106 Rust tests, all examples, the
main demo, prose style checks, completion-audit checks, reader-feedback-loop
checks, reader-feedback-inbox checks, rewrite-log checks, chapter reference
coverage checks, exercise alignment checks, exercise command checks,
duplicate-prose checks, source coverage checks, book build, and chapter tests.

## Sweep 2 Pass 52: Reader Review Sprint

Files:

```text
community/reader-review-sprint.md
community/reviewer-outreach.md
community/reader-review-guide.md
community/reader-feedback-inbox.md
community/feedback-wall.md
book/STYLE_GUIDE.md
book/TEXTBOOK_COMPLETION_AUDIT.md
scripts/check-completion-audit.py
scripts/check-reader-feedback-loop.py
```

Primary sources:

- [How People Learn II](https://nap.nationalacademies.org/catalog/24783/how-people-learn-ii-learners-contexts-and-cultures)
- [GitHub Docs: Syntax for issue forms](https://docs.github.com/en/communities/using-templates-to-encourage-useful-issues-and-pull-requests/syntax-for-issue-forms)

Critique before rewrite:

The project now had outreach asks, a review packet, an issue form, an inbox, and
a triage protocol. The missing operational layer was a bounded first cohort:
how many direct reports are needed, which reviewer profiles should be covered,
what counts as accepted evidence, and how the sprint closes into a rewrite. The
completion audit should not depend on vague "ask for feedback" activity.

Rewrite decision:

- add `community/reader-review-sprint.md`,
- require a five-report target across Rust, ML, category-theory, technical
  educator, and beginner-adjacent readers,
- define accepted-report fields and non-evidence signals,
- define a seven-day intake, clarification, triage, rewrite, and validation
  loop,
- link the sprint from reviewer outreach, the reader review guide, the feedback
  inbox, and the feedback wall,
- update the style guide and completion audit so the review sprint is part of
  the direct-feedback collection path,
- strengthen `scripts/check-reader-feedback-loop.py` so the sprint keeps its
  reviewer profiles, seven-day plan, acceptance criteria, non-evidence
  boundary, close-out artifacts, and required links.

Validation:

```bash
python3 scripts/check-reader-feedback-loop.py
python3 scripts/check-completion-audit.py
git diff --check -- . ':!target'
```

Result:

```text
Reader feedback loop check passed.
Completion audit check passed.
Diff whitespace check passed.
```

The full publication gate then passed with 106 Rust tests, all examples, the
main demo, prose style checks, completion-audit checks, reader-feedback-loop
checks, rewrite-log checks, chapter reference coverage checks, exercise
alignment checks, exercise command checks, duplicate-prose checks, source
coverage checks, book build, and chapter tests.

## Sweep 2 Pass 51: Reviewer Outreach Packet

Files:

```text
community/reviewer-outreach.md
community/reader-review-guide.md
community/reader-review-packet.md
community/reader-feedback-inbox.md
community/feedback-wall.md
book/STYLE_GUIDE.md
book/TEXTBOOK_COMPLETION_AUDIT.md
scripts/check-completion-audit.py
scripts/check-reader-feedback-loop.py
```

Primary sources:

- [GitHub Docs: Syntax for issue forms](https://docs.github.com/en/communities/using-templates-to-encourage-useful-issues-and-pull-requests/syntax-for-issue-forms)
- [How People Learn II](https://nap.nationalacademies.org/catalog/24783/how-people-learn-ii-learners-contexts-and-cultures)

Critique before rewrite:

The project had a review packet, direct issue-form links, a triage protocol, and
an inbox for accepted direct reports. The remaining activation gap was before
the report exists: maintainers still needed a public, reusable way to ask the
right reviewers for the right kind of friction. Without a reviewer-outreach
packet, the project could invite broad praise or vague opinions instead of one
actionable reader report.

Rewrite decision:

- add `community/reviewer-outreach.md`,
- define reviewer profiles for Rust engineers, ML engineers, category-theory
  readers, technical educators, and beginner-adjacent readers,
- add short public, Rust reviewer, ML reviewer, category-theory reviewer,
  educator, and workshop follow-up asks,
- point every ask to the review packet and reader-confusion issue form,
- state that a sent outreach message is not reader evidence,
- link the outreach packet from the review guide, review packet, feedback wall,
  and feedback inbox,
- strengthen `scripts/check-reader-feedback-loop.py` so the outreach packet
  keeps its reviewer profiles, public asks, feedback links, inbox link, triage
  link, rewrite-log link, and evidence boundary,
- update the completion audit and checker so outreach is part of the remaining
  direct-feedback collection path.

Validation:

```bash
python3 scripts/check-reader-feedback-loop.py
python3 scripts/check-completion-audit.py
git diff --check -- . ':!target'
```

Result:

```text
Reader feedback loop check passed.
Completion audit check passed.
Diff whitespace check passed.
```

The full publication gate then passed with 106 Rust tests, all examples, the
main demo, prose style checks, completion-audit checks, reader-feedback-loop
checks, rewrite-log checks, chapter reference coverage checks, exercise
alignment checks, exercise command checks, duplicate-prose checks, source
coverage checks, book build, and chapter tests.

## Sweep 2 Pass 50: Direct Reader Feedback Inbox

Files:

```text
community/reader-feedback-inbox.md
community/reader-feedback-triage.md
community/feedback-wall.md
community/reader-review-guide.md
community/reader-review-packet.md
book/STYLE_GUIDE.md
book/TEXTBOOK_COMPLETION_AUDIT.md
scripts/check-completion-audit.py
scripts/check-reader-feedback-loop.py
```

Primary sources:

- [GitHub Docs: Syntax for issue forms](https://docs.github.com/en/communities/using-templates-to-encourage-useful-issues-and-pull-requests/syntax-for-issue-forms)
- [How People Learn II](https://nap.nationalacademies.org/catalog/24783/how-people-learn-ii-learners-contexts-and-cultures)

Critique before rewrite:

The reader feedback loop had a review packet, issue form, triage protocol, and
direct issue-form links. The remaining gap was evidence custody. Once a real
reader report arrives, the repo needed a stable local place to record the
source, location, command, friction lens, triage action, rewrite-log entry, and
validation. Without that inbox, direct feedback could live only in GitHub issue
state or informal memory, which is weaker than the rest of the source-backed
rewrite loop.

Rewrite decision:

- add `community/reader-feedback-inbox.md`,
- make the inbox explicitly say no direct project-specific reports are recorded
  yet,
- define an accepted-report entry format with source, issue URL, review path,
  location, command or file tried, friction lens, confusing text or output,
  triage action, rewrite-log entry, validation, and status,
- link the inbox from the triage protocol, feedback wall, review guide, and
  review packet,
- update the style guide and completion audit so the inbox is part of the
  reader-feedback loop,
- strengthen `scripts/check-reader-feedback-loop.py` so the inbox cannot lose
  its required fields, states, or triage link,
- update the completion-audit checker so the missing-work section continues to
  name the inbox until real reports arrive.

Validation:

```bash
python3 scripts/check-reader-feedback-loop.py
python3 scripts/check-completion-audit.py
git diff --check -- . ':!target'
```

Result:

```text
Reader feedback loop check passed.
Completion audit check passed.
Diff whitespace check passed.
```

The full publication gate then passed with 106 Rust tests, all examples, the
main demo, prose style checks, completion-audit checks, reader-feedback-loop
checks, rewrite-log checks, chapter reference coverage checks, exercise
alignment checks, exercise command checks, duplicate-prose checks, source
coverage checks, book build, and chapter tests.

## Sweep 2 Pass 47: Duplicate Prose Gate

Files:

```text
scripts/check-duplicate-prose.py
scripts/check.sh
scripts/check-completion-audit.py
book/STYLE_GUIDE.md
book/TEXTBOOK_COMPLETION_AUDIT.md
```

Primary sources:

- [How People Learn II](https://nap.nationalacademies.org/catalog/24783/how-people-learn-ii-learners-contexts-and-cultures)

Critique before rewrite:

The major chapters have grown through several rewrite sweeps. That improves
coverage, but it also creates a quiet textbook risk: repeated explanatory
paragraphs can make the book feel padded instead of cumulative. Manual review
can catch obvious repetition, but a world-class textbook needs a mechanical
guard that catches exact duplicated prose before it reaches the public draft.

Rewrite decision:

- add `scripts/check-duplicate-prose.py`,
- scan the major teaching chapters for exact normalized duplicate prose
  paragraphs,
- ignore headings, tables, lists, code fences, detail blocks, blockquotes, and
  HTML fragments so legitimate repeated structures are not flagged,
- require repeated prose to clear a minimum word threshold before failing,
- wire the checker into `scripts/check.sh`,
- update the completion audit and style guide so repetition control is part of
  the publication gate.

Validation:

```bash
python3 scripts/check-duplicate-prose.py
python3 scripts/check-completion-audit.py
python3 scripts/check-prose-style.py
python3 scripts/check-exercise-commands.py
```

Result:

```text
Duplicate prose check passed.
Completion audit check passed.
Prose style check passed.
Exercise command check passed.
```

The full publication gate then passed with 106 Rust tests, all examples, the
main demo, prose style checks, completion-audit checks, reader-feedback-loop
checks, rewrite-log checks, chapter reference coverage checks, exercise
alignment checks, exercise command checks, duplicate-prose checks, source
coverage checks, book build, and chapter tests.

## Sweep 2 Pass 54: Direct Reader Completion Gate

Files:

```text
scripts/check-reader-feedback-inbox.py
scripts/check-completion-audit.py
scripts/check-reader-feedback-loop.py
community/reader-review-sprint.md
book/STYLE_GUIDE.md
book/TEXTBOOK_COMPLETION_AUDIT.md
```

Primary sources:

- [How People Learn II](https://nap.nationalacademies.org/catalog/24783/how-people-learn-ii-learners-contexts-and-cultures)
- [GitHub Docs: Syntax for issue forms](https://docs.github.com/en/communities/using-templates-to-encourage-useful-issues-and-pull-requests/syntax-for-issue-forms)

Critique before rewrite:

The inbox validator could prove that accepted report entries were well formed,
but it did not give maintainers a single command for the stronger question:
is the direct-reader review sprint complete enough to support a completion
claim? The completion audit named the gap, but the gap was not yet executable.

Rewrite decision:

- add `--require-sprint-complete` to `scripts/check-reader-feedback-inbox.py`,
- keep normal validation green while no reports are recorded,
- make the strict mode require at least five accepted direct-reader reports,
  one accepted report from each sprint reviewer context, and at least one
  report with `Status: rewritten`,
- print the current accepted-report count and missing reviewer contexts in
  normal mode,
- document the strict command in the review sprint, style guide, completion
  audit, and completion-audit checker,
- keep the strict gate out of `scripts/check.sh` so daily validation can pass
  while the audit honestly remains open.

Validation:

```bash
python3 scripts/check-reader-feedback-inbox.py
python3 scripts/check-reader-feedback-inbox.py --require-sprint-complete
python3 scripts/check-reader-feedback-loop.py
python3 scripts/check-completion-audit.py
```

Result:

```text
Reader feedback inbox check passed with no direct reports recorded.
Reader feedback sprint status: 0/5 accepted reports, 0 rewritten report(s).
Reader feedback loop check passed.
Completion audit check passed.
```

The strict sprint-completion command failed as intended because no direct
reader reports are present yet. That failure is the correct completion guard,
not a local validation failure.

The full publication gate then passed with 106 Rust tests, all examples, the
main demo, prose style checks, completion-audit checks, reader-feedback-loop
checks, reader-feedback-inbox status output, rewrite-log checks, chapter
reference coverage checks, exercise alignment checks, exercise command checks,
duplicate-prose checks, source coverage checks, book build, and chapter tests.

## Sweep 2 Pass 55: GitHub Feedback Surface

Files:

```text
scripts/check-github-feedback-surface.sh
scripts/check-completion-audit.py
scripts/check-reader-feedback-loop.py
community/reader-review-sprint.md
community/feedback-wall.md
community/starter-issues.md
book/TEXTBOOK_COMPLETION_AUDIT.md
```

Primary sources:

- [GitHub Docs: Managing labels](https://docs.github.com/en/issues/using-labels-and-milestones-to-track-work/managing-labels)
- [GitHub Docs: Syntax for issue forms](https://docs.github.com/en/communities/using-templates-to-encourage-useful-issues-and-pull-requests/syntax-for-issue-forms)

Critique before rewrite:

The repo-local reader feedback loop was now strong, but a practical public
surface was still weak: the `reader confusion` label used by the issue form did
not exist yet on GitHub, and the five-report review sprint had no public issue
handle. That made the feedback loop correct in files but less actionable for
reviewers and maintainers on GitHub.

Rewrite decision:

- create the missing GitHub routing labels for reader confusion, ML intuition,
  category-theory precision, Rust idiom review, exercise ideas, glossary
  needs, sponsor-worthy milestones, and reader review sprint work,
- create [GitHub issue #6](https://github.com/hghalebi/category_theory_transformer_rs/issues/6)
  as the public tracking issue for collecting five direct reader reports,
- add `scripts/check-github-feedback-surface.sh` as an optional live check for
  the required labels and issue #6,
- keep the live GitHub check out of `scripts/check.sh` because it requires
  network access and `gh`,
- link issue #6 from the reader review sprint, feedback wall, starter issues,
  and completion audit.

Validation:

```bash
scripts/check-github-feedback-surface.sh
python3 scripts/check-reader-feedback-loop.py
python3 scripts/check-completion-audit.py
git diff --check -- . ':!target'
```

Result:

```text
GitHub feedback surface check passed.
Reader feedback loop check passed.
Completion audit check passed.
Diff whitespace check passed.
```

The full publication gate then passed with 106 Rust tests, all examples, the
main demo, prose style checks, completion-audit checks, reader-feedback-loop
checks, reader-feedback-inbox status output, rewrite-log checks, chapter
reference coverage checks, exercise alignment checks, exercise command checks,
duplicate-prose checks, source coverage checks, book build, and chapter tests.

## Sweep 2 Pass 56: Reader Issue Intake Bridge

Files:

```text
scripts/collect-reader-feedback-issues.py
scripts/check.sh
scripts/check-completion-audit.py
scripts/check-reader-feedback-loop.py
community/reader-feedback-inbox.md
community/reader-feedback-triage.md
community/reader-review-sprint.md
book/STYLE_GUIDE.md
book/TEXTBOOK_COMPLETION_AUDIT.md
```

Primary sources:

- [GitHub CLI manual: gh issue list](https://cli.github.com/manual/gh_issue_list)
- [GitHub Docs: Syntax for issue forms](https://docs.github.com/en/communities/using-templates-to-encourage-useful-issues-and-pull-requests/syntax-for-issue-forms)

Critique before rewrite:

The GitHub issue form, labels, sprint issue, inbox, and strict completion gate
were connected, but a future maintainer still had a manual gap: copying a real
`reader confusion` issue into the inbox format. Manual copying risks losing the
source URL, command tried, friction lens, or last-clear-idea field.

Rewrite decision:

- add `scripts/collect-reader-feedback-issues.py`,
- fetch `reader confusion` issues through `gh issue list`,
- parse GitHub issue-form section headings into inbox fields,
- render inbox-ready report drafts without mutating files by default,
- include an offline `--self-test` parser check in `scripts/check.sh`,
- keep live issue collection out of the offline full gate,
- document the intake bridge in the inbox, triage protocol, sprint plan, style
  guide, and completion audit.

Validation:

```bash
python3 scripts/collect-reader-feedback-issues.py --self-test
python3 scripts/collect-reader-feedback-issues.py
python3 scripts/check-reader-feedback-loop.py
python3 scripts/check-completion-audit.py
git diff --check -- . ':!target'
```

Result:

```text
Reader feedback issue collector self-test passed.
No reader confusion issues found.
Reader feedback loop check passed.
Completion audit check passed.
Diff whitespace check passed.
```

The full publication gate then passed with 106 Rust tests, all examples, the
main demo, prose style checks, completion-audit checks, reader-feedback-loop
checks, reader-feedback-inbox status output, reader-issue collector self-test,
rewrite-log checks, chapter reference coverage checks, exercise alignment
checks, exercise command checks, duplicate-prose checks, source coverage
checks, book build, and chapter tests.

## Sweep 2 Pass 57: Reader Feedback Status Command

Files:

```text
scripts/reader-feedback-status.sh
scripts/check-completion-audit.py
scripts/check-reader-feedback-loop.py
community/reader-feedback-inbox.md
community/reader-review-sprint.md
book/STYLE_GUIDE.md
book/TEXTBOOK_COMPLETION_AUDIT.md
```

Primary sources:

- [GitHub CLI manual: gh issue list](https://cli.github.com/manual/gh_issue_list)
- [GitHub Docs: Managing labels](https://docs.github.com/en/issues/using-labels-and-milestones-to-track-work/managing-labels)

Critique before rewrite:

The repo now had separate commands for the local inbox, strict completion gate,
GitHub feedback surface, and issue intake. That was good for validation, but it
made completion audits require remembering several commands and interpreting
one expected strict-gate failure. A maintainer needed one status command that
shows the current direct-feedback state without pretending the goal is closed.

Rewrite decision:

- add `scripts/reader-feedback-status.sh`,
- make default mode show the local inbox status and strict completion gate,
- make `--live` also run the GitHub feedback-surface check and issue-intake
  renderer,
- keep the command exit status successful when the strict gate is open, because
  an open direct-feedback gap is expected until real reports arrive,
- document the command in the inbox, sprint plan, style guide, and completion
  audit,
- make the feedback-loop and completion-audit checkers require the command in
  the relevant status surfaces.

Validation:

```bash
scripts/reader-feedback-status.sh
scripts/reader-feedback-status.sh --live
python3 scripts/check-reader-feedback-loop.py
python3 scripts/check-completion-audit.py
git diff --check -- . ':!target'
```

Result:

```text
Reader feedback sprint status: 0/5 accepted reports, 0 rewritten report(s).
Strict completion gate is still open.
GitHub feedback surface check passed.
No reader confusion issues found.
Reader feedback loop check passed.
Completion audit check passed.
Diff whitespace check passed.
```

The full publication gate then passed with 106 Rust tests, all examples, the
main demo, prose style checks, completion-audit checks, reader-feedback-loop
checks, reader-feedback-inbox status output, reader-issue collector self-test,
rewrite-log checks, chapter reference coverage checks, exercise alignment
checks, exercise command checks, duplicate-prose checks, source coverage
checks, book build, and chapter tests.

## Sweep 2 Pass 58: Reviewer Context Intake

Files:

```text
.github/ISSUE_TEMPLATE/reader-confusion.yml
scripts/collect-reader-feedback-issues.py
scripts/check-reader-feedback-loop.py
community/reader-review-packet.md
community/reader-review-sprint.md
community/reader-feedback-triage.md
community/reader-feedback-inbox.md
book/TEXTBOOK_COMPLETION_AUDIT.md
```

Primary sources:

- [GitHub Docs: Syntax for issue forms](https://docs.github.com/en/communities/using-templates-to-encourage-useful-issues-and-pull-requests/syntax-for-issue-forms)

Critique before rewrite:

The strict sprint gate requires accepted reports from Rust engineer, ML
engineer, category-theory reader, technical educator, and beginner-adjacent
reader contexts. The GitHub issue form did not ask for reviewer context, so
future reports would require manual classification before they could satisfy
the gate.

Rewrite decision:

- add a required `Reviewer context` dropdown to the reader-confusion issue
  form,
- include the five sprint reviewer contexts plus `Other`,
- map the issue-form section into `Reviewer context` in
  `scripts/collect-reader-feedback-issues.py`,
- update the collector self-test to prove the field survives parsing,
- strengthen the feedback-loop checker so the issue form keeps the field and
  required options,
- update review packet, sprint, triage, inbox, and completion audit wording so
  reviewer context is treated as first-class evidence.

Validation:

```bash
python3 scripts/collect-reader-feedback-issues.py --self-test
python3 scripts/check-reader-feedback-loop.py
python3 scripts/check-completion-audit.py
python3 scripts/check-reader-feedback-inbox.py
```

Result:

```text
Reader feedback issue collector self-test passed.
Reader feedback loop check passed.
Completion audit check passed.
Reader feedback inbox check passed with no direct reports recorded.
```

The full publication gate then passed with 106 Rust tests, all examples, the
main demo, prose style checks, completion-audit checks, reader-feedback-loop
checks, reader-feedback-inbox status output, reader-issue collector self-test,
rewrite-log checks, chapter reference coverage checks, exercise alignment
checks, exercise command checks, duplicate-prose checks, source coverage
checks, book build, and chapter tests.

## Sweep 2 Pass 59: Reader Inbox Completion Fixture

Files:

```text
scripts/check-reader-feedback-inbox.py
scripts/check.sh
scripts/check-completion-audit.py
community/reader-review-sprint.md
book/TEXTBOOK_COMPLETION_AUDIT.md
```

Primary sources:

- [How People Learn II](https://nap.nationalacademies.org/catalog/24783/how-people-learn-ii-learners-contexts-and-cultures)

Critique before rewrite:

The strict reader-feedback completion gate now represented the real missing
external evidence, but it only had negative evidence in normal project state:
the empty inbox fails strict mode. Without an offline positive fixture, a future
change could make the strict gate impossible to satisfy or too easy to satisfy
without being noticed.

Rewrite decision:

- add `--self-test` to `scripts/check-reader-feedback-inbox.py`,
- refactor the validator so fixtures and the real inbox use the same logic,
- add a complete five-report fixture covering Rust engineer, ML engineer,
  category-theory reader, technical educator, and beginner-adjacent reader,
- require one fixture report to have `Status: rewritten`,
- add an incomplete fixture that must fail strict mode,
- wire the self-test into `scripts/check.sh` and the completion audit.

Validation:

```bash
python3 scripts/check-reader-feedback-inbox.py --self-test
python3 scripts/check-reader-feedback-inbox.py
python3 scripts/check-reader-feedback-inbox.py --require-sprint-complete
python3 -m py_compile scripts/check-reader-feedback-inbox.py
```

Result:

```text
Reader feedback inbox self-test passed.
Reader feedback inbox check passed with no direct reports recorded.
The strict completion gate still failed for the real empty inbox as intended.
```

The full publication gate then passed with 106 Rust tests, all examples, the
main demo, prose style checks, completion-audit checks, reader-feedback-loop
checks, reader-feedback-inbox status output, reader-feedback-inbox self-test,
reader-issue collector self-test, rewrite-log checks, chapter reference
coverage checks, exercise alignment checks, exercise command checks,
duplicate-prose checks, source coverage checks, book build, and chapter tests.

## Sweep 2 Pass 48: Exercise Command Gate

Files:

```text
scripts/check-exercise-commands.py
scripts/check.sh
scripts/check-completion-audit.py
book/STYLE_GUIDE.md
book/TEXTBOOK_COMPLETION_AUDIT.md
```

Primary sources:

- [Rust Book: How to Write Tests](https://doc.rust-lang.org/book/ch11-01-writing-tests.html)

Critique before rewrite:

The exercise ladder now contains many useful commands, including full-suite
checks, example runs, targeted unit tests, finite-difference tests, and
structure-law tests. A previous manual review caught an invalid command shape
with two `cargo test` filters. That showed a weak validation boundary:
exercise commands were useful but not mechanically checked as intentional
learner entry points.

Rewrite decision:

- add `scripts/check-exercise-commands.py`,
- scan learner-facing exercise and lesson surfaces for fenced `bash` commands,
- require every command to match an explicit allowlist,
- reject `cargo test` invocations with more than one test-name filter,
- require the key exercise commands for full tests, targeted loss tests,
  finite-difference tests, structure-law tests, the attention example, and the
  main quality gate,
- wire the checker into `scripts/check.sh`,
- update the style guide and completion audit so command validity is part of
  the textbook gate.

Validation:

```bash
python3 scripts/check-exercise-commands.py
python3 scripts/check-completion-audit.py
python3 scripts/check-exercise-alignment.py
python3 scripts/check-prose-style.py
```

Result:

```text
Exercise command check passed.
Completion audit check passed.
Exercise alignment check passed.
Prose style check passed.
```

The full publication gate then passed with 106 Rust tests, all examples, the
main demo, prose style checks, completion-audit checks, reader-feedback-loop
checks, rewrite-log checks, chapter reference coverage checks, exercise
alignment checks, exercise command checks, duplicate-prose checks, source
coverage checks, book build, and chapter tests.

## Sweep 2 Pass 49: Direct Reader Issue Path Guard

Files:

```text
README.md
START_HERE.md
community/reader-review-guide.md
community/reader-review-packet.md
scripts/check-reader-feedback-loop.py
book/TEXTBOOK_COMPLETION_AUDIT.md
```

Primary sources:

- [GitHub Docs: Syntax for issue forms](https://docs.github.com/en/communities/using-templates-to-encourage-useful-issues-and-pull-requests/syntax-for-issue-forms)
- [GitHub Docs: Creating an issue with query parameters](https://docs.github.com/en/github/managing-your-work-on-github/about-automation-for-issues-and-pull-requests-with-query-parameters)

Critique before rewrite:

The completion audit correctly says direct project-specific reader reports are
still missing. A live issue check returned no `reader confusion` reports. The
next practical weakness was therefore not chapter prose; it was the activation
path for a reader who wants to submit friction. The README's top choice table
sent chapter questions to issue `#1`, which is a useful starter issue but not
the structured issue form. That can lose exactly the evidence the next
reader-driven rewrite needs.

Rewrite decision:

- change the top README feedback path to the reader-confusion issue form,
- link the same form from `START_HERE.md`,
- link the form from the reader review guide and packet,
- keep the existing starter issue links for readers who want prewritten public
  issue handles,
- strengthen `scripts/check-reader-feedback-loop.py` so README, `START_HERE`,
  the guide, and the packet must link directly to the structured form,
- add a guard that the top README feedback path cannot point to issue `#1`.

Validation:

```bash
python3 scripts/check-reader-feedback-loop.py
python3 scripts/check-completion-audit.py
python3 scripts/check-rewrite-log.py
git diff --check -- . ':!target'
```

Result:

```text
Reader feedback loop check passed.
Completion audit check passed.
Rewrite log check passed.
Diff whitespace check passed.
```

The full publication gate then passed with 106 Rust tests, all examples, the
main demo, prose style checks, completion-audit checks, reader-feedback-loop
checks, rewrite-log checks, chapter reference coverage checks, exercise
alignment checks, exercise command checks, duplicate-prose checks, source
coverage checks, book build, and chapter tests.

## Sweep 2 Pass 133: Transformer Linear Category Precision Source

Files:

```text
book/src/references.md
book/EDITORIAL_RESEARCH_NOTES.md
book/src/roadmap.md
book/CHAPTER_QUALITY_SCORECARD.md
book/TEXTBOOK_COMPLETION_AUDIT.md
scripts/check-public-friction-matrix.py
scripts/check-source-authority.py
```

Primary sources:

- [Self-Attention as a Parametric Endofunctor](https://arxiv.org/abs/2501.02931)
- [Attention Is All You Need](https://arxiv.org/abs/1706.03762)
- [D2L Attention and Transformers](https://d2l.ai/chapter_attention-mechanisms-and-transformers/index.html)
- [PyTorch scaled dot product attention](https://docs.pytorch.org/docs/stable/generated/torch.nn.functional.scaled_dot_product_attention.html)
- [Hugging Face Course: How do Transformers work?](https://huggingface.co/docs/course/en/chapter1/4)

Critique before rewrite:

The Transformer Roadmap had a strong count-inputs-first category-shape
diagnostic, but its source map did not yet include a focused categorical
self-attention reference. That left a precision risk: a reader could see
"endomorphism" near attention and overgeneralize from linear query-key-value
maps to the whole pedagogical block.

ML teaching critique:

The attention chapter should keep query, key, value, softmax, masking,
residual addition, normalization, feed-forward refinement, and training state
as distinct learner objects. Compressing them into one advanced mathematical
name would hide the sequence of operations the chapter is trying to make
visible.

Category-theory critique:

The new source is valuable precisely because it supports a narrower claim:
linear portions of self-attention can be studied through parametric categorical
structure, while nonlinear parts require separate treatment. The roadmap should
use that source to improve caution, not to inflate claims.

Learner critique:

A motivated learner could ask, "If attention can be described as an
endofunctor, why does the roadmap keep product-input morphisms, endomorphisms,
and illegal compositions separate?" The rewrite answers by separating the
advanced research lens from this book's local naming rule.

Rewrite decision:

- add the categorical self-attention paper to the Transformer reference row
  and source bucket,
- add a roadmap warning that linear query-key-value structure is not the same
  as the whole block,
- protect the warning in the public friction checker,
- keep the Transformer Roadmap scorecard category precision as
  `Reader-needed` until direct readers validate the diagnostic.

Validation:

```bash
python3 scripts/check-source-authority.py
python3 scripts/check-chapter-references.py
python3 scripts/check-public-friction-matrix.py
python3 scripts/check-chapter-scorecard.py
```

Result:

```text
Source authority check passed.
Chapter reference coverage check passed.
Public friction matrix check passed.
Chapter scorecard check passed.
```

## Sweep 2 Pass 134: Reviewer Context Briefs

Files:

```text
community/reader-review-packet.md
community/reader-review-sprint.md
community/reviewer-slot-tracker.md
book/TEXTBOOK_COMPLETION_AUDIT.md
scripts/check-reader-feedback-loop.py
```

Primary sources:

- `community/reader-feedback-inbox.md`
- `community/reader-feedback-triage.md`
- `community/reviewer-outreach.md`
- `community/reader-review-sprint.md`

Critique before rewrite:

The direct-reader evidence loop was structurally ready, but the review packet
still required reviewers to infer which path best matched their background.
The strict completion gate needs five distinct reviewer contexts, so the next
practical weakness was not another chapter explanation; it was the lack of a
single public brief per missing context.

ML teaching critique:

An ML reviewer should not need to inspect the whole book to find the relevant
path. Their brief should point directly to training and attention examples and
ask for the first place where logits, probabilities, loss, updates, attention
roles, or training state loses meaning.

Category-theory critique:

A category-theory reviewer should be asked for precision evidence, not broad
approval. Their brief now targets terms, laws, diagrams, product-input
boundaries, endomorphisms, and the newer endofunctor warning.

Learner critique:

Different readers need different entry points. A beginner-adjacent reader can
review first-run compression; a technical educator can review next-action and
practice signals; a Rust engineer can review type and trait clarity. The
packet should make those expectations explicit before a report arrives.

Rewrite decision:

- add `Reviewer Context Briefs` to the public reader review packet,
- connect the sprint plan and slot tracker to those briefs,
- strengthen the reader-feedback-loop checker so the briefs stay present,
- record the context-brief evidence in the completion audit while keeping
  actual direct-reader reports pending.

Validation:

```bash
python3 scripts/check-reader-feedback-loop.py
python3 scripts/check-completion-audit.py
python3 scripts/check-reader-feedback-inbox.py
python3 scripts/check-reviewer-slot-tracker.py
python3 -m py_compile scripts/check-reader-feedback-loop.py scripts/check-completion-audit.py scripts/check-reader-feedback-inbox.py scripts/check-reviewer-slot-tracker.py
```

Result:

```text
Reader feedback loop check passed.
Completion audit check passed.
Reader feedback inbox check passed with no direct reports recorded.
Reader feedback sprint status: 0/5 accepted reports, 0 rewritten report(s).
Reviewer slot tracker check passed.
Python syntax check passed.
```

## Sweep 2 Pass 135: Context-Specific Report Links

Files:

```text
community/reader-review-packet.md
community/reader-review-sprint.md
community/reviewer-slot-tracker.md
book/TEXTBOOK_COMPLETION_AUDIT.md
scripts/check-reader-feedback-loop.py
```

Primary sources:

- [GitHub Docs: Creating an issue from a URL query](https://docs.github.com/en/issues/tracking-your-work-with-issues/using-issues/creating-an-issue#creating-an-issue-from-a-url-query)
- `.github/ISSUE_TEMPLATE/reader-confusion.yml`
- `community/reader-feedback-inbox.md`
- `community/reviewer-slot-tracker.md`

Critique before rewrite:

The packet now had role-specific reviewer briefs, but each reviewer still had
to open a generic issue form and copy the intended path by hand. That adds
avoidable friction exactly where the project needs five concrete reports.

ML teaching critique:

The ML reviewer path should open with the training and attention commands
already visible, so the reviewer spends time reporting the first unclear ML
idea rather than reconstructing the assigned path.

Category-theory critique:

The category-theory reviewer path should open with the morphism, structure,
and Seven Sketches examples already visible. The report still needs the human
reader's judgment, but the form should make the intended review scope obvious.

Learner critique:

A beginner-adjacent reader should not have to decide which command belongs in
the report. Prefilling title, location, and command keeps the report
location-bound without inventing the actual confusion.

Rewrite decision:

- add context-specific report links to `community/reader-review-packet.md`,
- prefill only title, location, and command or file fields,
- keep reviewer context and mental-model break as human-entered fields,
- connect the sprint and slot tracker to the prefilled links,
- guard the links in `scripts/check-reader-feedback-loop.py`.

Validation:

```bash
python3 scripts/check-reader-feedback-loop.py
python3 scripts/check-completion-audit.py
python3 scripts/check-reader-feedback-inbox.py
python3 scripts/check-reviewer-slot-tracker.py
python3 -m py_compile scripts/check-reader-feedback-loop.py scripts/check-completion-audit.py scripts/check-reader-feedback-inbox.py scripts/check-reviewer-slot-tracker.py
```

Result:

```text
Reader feedback loop check passed.
Completion audit check passed.
Reader feedback inbox check passed with no direct reports recorded.
Reader feedback sprint status: 0/5 accepted reports, 0 rewritten report(s).
Reviewer slot tracker check passed.
Python syntax check passed.
```

## Sweep 2 Pass 136: Report Link Safe-Query Guard

Files:

```text
scripts/check-reader-feedback-loop.py
book/TEXTBOOK_COMPLETION_AUDIT.md
```

Primary sources:

- [GitHub Docs: Creating an issue from a URL query](https://docs.github.com/en/issues/tracking-your-work-with-issues/using-issues/creating-an-issue#creating-an-issue-from-a-url-query)
- `.github/ISSUE_TEMPLATE/reader-confusion.yml`
- `community/reader-review-packet.md`

Critique before rewrite:

The context-specific report links reduced reviewer friction, but the checker
only searched for marker strings. It did not parse the links to prove they
prefilled only safe scaffolding. A malformed future link could accidentally
prefill evidence fields and make reports less trustworthy.

ML teaching critique:

The ML report link should steer a reviewer toward the intended examples, but
it must not pre-answer the point where ML intuition broke. The report is only
evidence if the reader supplies that break.

Category-theory critique:

The category-theory report link can prefill the relevant roadmap and structure
paths. It must not prefill a category-theory judgment such as the unclear term,
law, or boundary classification.

Learner critique:

The form should remove mechanical friction without scripting the learner's
experience. Prefill the path; require the reader to describe the confusion.

Rewrite decision:

- parse context-specific report links in `scripts/check-reader-feedback-loop.py`,
- require GitHub HTTPS issue-form URLs with `reader-confusion.yml`,
- allow only `template`, `title`, `location`, and `command` query fields,
- reject evidence fields such as `reviewer_context`, `friction_lens`,
  `quote`, `understood`, `stuck`, `expectation`, `smallest_fix`, and `body`,
- verify each reviewer context has the expected title, location marker, and
  command marker.

Validation:

```bash
python3 scripts/check-reader-feedback-loop.py
python3 -m py_compile scripts/check-reader-feedback-loop.py
```

Result:

```text
Reader feedback loop check passed.
Python syntax check passed.
```

## Sweep 2 Pass 137: Outreach Report Link Synchronization

Files:

```text
community/reviewer-outreach.md
scripts/check-reader-feedback-loop.py
book/TEXTBOOK_COMPLETION_AUDIT.md
```

Primary sources:

- `community/reader-review-packet.md`
- `community/reviewer-slot-tracker.md`
- `community/reader-feedback-inbox.md`

Critique before rewrite:

The review packet had safe context-specific report links, but the
copy-and-paste outreach asks still sent most reviewers to the generic issue
form. That left a gap between the protected reviewer packet and the actual
message a maintainer would send.

ML teaching critique:

The ML ask should link directly to the ML report path, with training and
attention commands already in the issue form. That reduces path setup and
keeps the report focused on the first unclear ML concept.

Category-theory critique:

The category-theory ask should link directly to the category-theory report
path, with the morphism, structure, Seven Sketches, and roadmap files already
named. The reviewer still supplies the precision failure.

Learner critique:

The short public and workshop asks should give a beginner-adjacent prefilled
path for first-pass readers. This lowers the cost of reporting the first
compressed sentence, command, output, or term.

Rewrite decision:

- add the Rust, ML, category-theory, educator, and beginner-adjacent prefilled
  report links to `community/reviewer-outreach.md`,
- keep the generic issue form visible for unclassified reports,
- extend `scripts/check-reader-feedback-loop.py` so outreach must include the
  same context-specific URLs as the review packet.

Validation:

```bash
python3 scripts/check-reader-feedback-loop.py
python3 scripts/check-completion-audit.py
python3 scripts/check-reader-feedback-inbox.py
python3 scripts/check-reviewer-slot-tracker.py
python3 -m py_compile scripts/check-reader-feedback-loop.py scripts/check-completion-audit.py scripts/check-reader-feedback-inbox.py scripts/check-reviewer-slot-tracker.py
```

Result:

```text
Reader feedback loop check passed.
Completion audit check passed.
Reader feedback inbox check passed with no direct reports recorded.
Reader feedback sprint status: 0/5 accepted reports, 0 rewritten report(s).
Reviewer slot tracker check passed.
Python syntax check passed.
```

## Sweep 2 Pass 138: Live Sprint Issue Link Synchronization

Files:

```text
scripts/check-github-feedback-surface.sh
book/TEXTBOOK_COMPLETION_AUDIT.md
```

External surface updated:

```text
https://github.com/hghalebi/category_theory_transformer_rs/issues/6
```

Primary sources:

- `community/reader-review-packet.md`
- `community/reviewer-outreach.md`
- `scripts/check-github-feedback-surface.sh`

Critique before rewrite:

The local review packet and outreach asks had protected context-specific report
links, but the live public sprint issue still pointed readers to the generic
issue form. That meant the public tracking handle could drift away from the
stronger local review flow.

ML teaching critique:

The live sprint issue should expose the ML engineer report path directly, so
reviewers can start from the training and attention commands instead of
reconstructing the route.

Category-theory critique:

The live sprint issue should expose the category-theory reader report path
directly, preserving the focus on morphisms, laws, product-input boundaries,
endomorphisms, and the roadmap precision check.

Learner critique:

The public issue should remain a routing handle, not evidence. It can lower
friction by linking to role-specific forms, but it must explicitly say that the
tracking issue itself does not count as a direct reader report.

Rewrite decision:

- update GitHub issue `#6` with the five context-specific report links,
- keep the public packet and sprint-plan links in the issue body,
- state that the issue is a routing handle only,
- strengthen `scripts/check-github-feedback-surface.sh` so the live issue body
  must contain all five report links and the non-evidence warning.

Validation:

```bash
bash scripts/check-github-feedback-surface.sh --self-test
bash scripts/check-github-feedback-surface.sh
python3 scripts/check-reader-feedback-loop.py
python3 scripts/check-completion-audit.py
```

Result:

```text
GitHub feedback surface self-test passed.
GitHub feedback surface check passed.
Reader feedback loop check passed.
Completion audit check passed.
```

## Sweep 2 Pass 139: Transformer Self-Attention Versus Cross-Attention Boundary

Files:

```text
book/src/roadmap.md
book/src/references.md
book/EDITORIAL_RESEARCH_NOTES.md
book/TEXTBOOK_COMPLETION_AUDIT.md
book/CHAPTER_QUALITY_SCORECARD.md
scripts/check-public-friction-matrix.py
```

Primary sources:

- [PyTorch MultiheadAttention](https://docs.pytorch.org/docs/stable/generated/torch.nn.MultiheadAttention.html)
- [Dive into Deep Learning: Queries, Keys, and Values](https://d2l.ai/chapter_attention-mechanisms-and-transformers/queries-keys-values.html)
- [Dive into Deep Learning: Attention Scoring Functions](https://d2l.ai/chapter_attention-mechanisms-and-transformers/attention-scoring-functions.html)

Critique before rewrite:

The Transformer Roadmap had strong role ownership for query, key, value,
scores, masks, weights, and value mixing. It also had a count-inputs-first
category diagnostic. The missing precision was the distinction between the
self-attention case and the more general query-source versus key-value-source
attention boundary.

ML teaching critique:

A learner can hear "self-attention" and assume attention always has one
sequence object as input. Framework and textbook sources expose a more general
shape: query positions and key-value positions can be separate, with target
length `L` and source length `S`.

Category-theory critique:

The roadmap should make the product-input shape visible before naming any
endomorphism. Self-attention is a special case where query, key, and value
roles come from the same hidden sequence. Cross-attention makes the separate
target and source sides impossible to hide.

Learner critique:

Readers need a short rule they can apply while reading framework docs:
same source for Q, K, V is the self-attention case; separate query and
key-value sources are the cross-attention case.

Rewrite decision:

- add a `Self-Attention And Cross-Attention Boundary` section to the roadmap,
- connect PyTorch's `L x S` mask shape and D2L's `n` queries by `m`
  key-value pairs to the book's typed objects,
- keep the current code claim modest by saying the repository does not yet
  implement full cross-attention,
- strengthen `scripts/check-public-friction-matrix.py` so this boundary cannot
  disappear without a validation failure,
- update the reference map, editorial notes, completion audit, and scorecard
  focus.

Validation:

```bash
python3 scripts/check-public-friction-matrix.py
python3 scripts/check-chapter-references.py
python3 scripts/check-chapter-scorecard.py
python3 scripts/check-completion-audit.py
python3 scripts/check-rewrite-log.py
```

Result:

```text
Public friction matrix check passed.
Chapter reference coverage check passed.
Chapter scorecard check passed.
Completion audit check passed.
Rewrite log check passed.
```

## Sweep 2 Pass 140: Self-Vs-Cross Attention Review Routing

Files:

```text
community/reader-review-packet.md
community/reader-review-guide.md
community/reader-review-sprint.md
community/reviewer-outreach.md
book/TEXTBOOK_COMPLETION_AUDIT.md
scripts/check-reader-feedback-loop.py
scripts/check-github-feedback-surface.sh
```

External surface updated:

```text
https://github.com/hghalebi/category_theory_transformer_rs/issues/6
```

Primary sources:

- `book/src/roadmap.md`
- `community/reader-review-packet.md`
- `community/reader-review-guide.md`
- `community/reader-review-sprint.md`

Critique before rewrite:

The roadmap now teaches a source-backed self-attention versus cross-attention
boundary, but the reader-review workflow still asked reviewers mainly for the
older category-shape diagnostic. That created a gap between the newest
teaching risk and the direct-reader evidence the project is trying to collect.

ML teaching critique:

ML reviewers should be asked whether the distinction between shared Q/K/V
source, separate query and key-value sources, target length, source length, and
attention mask shape stays clear after running the attention example.

Category-theory critique:

Category-theory reviewers should be asked whether the self-vs-cross attention
boundary preserves the count-inputs-first rule before any endomorphism claim.

Learner critique:

A reviewer should not need to infer the newest review target from the rewrite
log. The packet, guide, sprint plan, outreach ask, and live tracking issue
should all point to the same current friction target.

Rewrite decision:

- add self-attention versus cross-attention review prompts to the reader packet,
  review guide, sprint plan, and outreach asks,
- update the completion audit so the review-loop evidence names the new prompt,
- extend `scripts/check-reader-feedback-loop.py` so the prompt cannot
  disappear from local review surfaces,
- extend `scripts/check-github-feedback-surface.sh` so the live sprint issue
  must expose the new review target,
- update GitHub issue `#6` with the latest roadmap review target.

Validation:

```bash
python3 scripts/check-reader-feedback-loop.py
bash scripts/check-github-feedback-surface.sh --self-test
bash scripts/check-github-feedback-surface.sh
python3 scripts/check-completion-audit.py
bash scripts/check.sh
scripts/check-textbook-readiness.sh
scripts/reader-feedback-status.sh --live
```

Result:

```text
Reader feedback loop check passed.
GitHub feedback surface self-test passed.
GitHub feedback surface check passed.
Completion audit check passed.
Full publication gate passed.
Textbook readiness check failed only on the strict direct-reader completion
gate: 0/5 accepted direct-reader reports and 0 rewritten reports.
Live reader feedback status found no reader-confusion issues.
```

## Sweep 2 Pass 143: Parallel Projection Review Routing

Files:

```text
community/reader-review-packet.md
community/reader-review-guide.md
community/reader-review-sprint.md
community/reviewer-outreach.md
book/TEXTBOOK_COMPLETION_AUDIT.md
scripts/check-reader-feedback-loop.py
scripts/check-github-feedback-surface.sh
```

External surface updated:

```text
https://github.com/hghalebi/category_theory_transformer_rs/issues/6
```

Primary sources:

- `book/src/roadmap.md`
- `community/reader-review-packet.md`
- `community/reader-review-guide.md`
- `community/reader-review-sprint.md`

Critique before rewrite:

The roadmap now explicitly says that self-attention's query, key, and value
role projections are parallel morphisms out of a shared `HiddenSequence`, not a
role-to-role pipeline. The reader-review workflow still asked for the broader
self-vs-cross distinction, but it did not force reviewers to test that exact
new false reading.

ML teaching critique:

ML reviewers should be asked whether the shared source sequence is clear
without implying that a query turns into a key or a key turns into a value.

Category-theory critique:

Category-theory reviewers should be asked whether the role projections are
ordinary parallel morphisms before attention scoring introduces the
`QuerySequence x KeySequence` product input.

Learner critique:

A reviewer should not need to infer the newest risk from the rewrite log. The
packet, guide, sprint plan, outreach ask, completion audit, local checker, and
live sprint issue should all name the same report target.

Rewrite decision:

- add the parallel-projection distinction to the reader packet,
  reader-review guide, sprint plan, and category-theory outreach ask,
- update the completion audit so the review-loop evidence names the parallel
  Q/K/V projection review prompt,
- extend `scripts/check-reader-feedback-loop.py` so the local review surfaces
  cannot drop the prompt,
- extend `scripts/check-github-feedback-surface.sh` so the live sprint issue
  must expose the prompt,
- update GitHub issue `#6` with the latest roadmap review target.

Validation:

```bash
python3 scripts/check-reader-feedback-loop.py
bash scripts/check-github-feedback-surface.sh --self-test
bash scripts/check-github-feedback-surface.sh
python3 scripts/check-completion-audit.py
bash scripts/check.sh
scripts/check-textbook-readiness.sh
scripts/reader-feedback-status.sh --live
```

Result:

```text
Reader feedback loop check passed.
GitHub feedback surface self-test passed.
GitHub feedback surface check passed.
Completion audit check passed.
Full publication gate passed.
Textbook readiness check failed only on the strict direct-reader completion
gate: 0/5 accepted direct-reader reports and 0 rewritten reports.
Live reader feedback status found no reader-confusion issues.
```

## Sweep 2 Pass 144: Live Review Issue Target Consolidation

Files:

```text
scripts/check-github-feedback-surface.sh
book/CHAPTER_REWRITE_LOG.md
```

External surface updated:

```text
https://github.com/hghalebi/category_theory_transformer_rs/issues/6
```

Primary sources:

- `community/reader-review-packet.md`
- `community/reader-review-sprint.md`
- GitHub issue `#6`

Critique before rewrite:

The live sprint issue had two repeated `Latest roadmap review target` sections:
one for self-vs-cross attention and one for parallel Q/K/V projections. Both
targets were useful, but duplicate headings make the public review handle look
like an append-only scratchpad instead of a controlled reviewer route.

ML teaching critique:

The issue should present both attention review targets as one short list so an
ML reviewer can see the current scope without deciding which repeated heading
is authoritative.

Category-theory critique:

The self-vs-cross target and the parallel-projection target are related but
distinct. Consolidating them keeps the product-input and parallel-morphism
questions visible without collapsing them.

Learner critique:

A reviewer should see one current section named in the plural, then two
numbered targets. That reduces navigation friction before they open an issue.

Rewrite decision:

- consolidate GitHub issue `#6` into one `Latest roadmap review targets`
  section,
- preserve both current target prompts,
- update `scripts/check-github-feedback-surface.sh` to require exactly one
  plural section and reject the older duplicate singular heading.

Validation:

```bash
bash scripts/check-github-feedback-surface.sh --self-test
bash scripts/check-github-feedback-surface.sh
python3 scripts/check-rewrite-log.py
bash scripts/check.sh
scripts/check-textbook-readiness.sh
scripts/reader-feedback-status.sh --live
```

Result:

```text
GitHub feedback surface self-test passed.
GitHub feedback surface check passed.
Rewrite log check passed.
Full publication gate passed.
Textbook readiness check failed only on the strict direct-reader completion
gate: 0/5 accepted direct-reader reports and 0 rewritten reports.
Live reader feedback status found no reader-confusion issues.
```

## Sweep 2 Pass 141: Glossary Self-Vs-Cross Attention Term Coherence

Files:

```text
book/src/glossary.md
book/TEXTBOOK_COMPLETION_AUDIT.md
scripts/check-public-friction-matrix.py
```

Primary sources:

- [Attention Is All You Need](https://arxiv.org/abs/1706.03762)
- [PyTorch MultiheadAttention](https://docs.pytorch.org/docs/stable/generated/torch.nn.MultiheadAttention.html)
- [Dive into Deep Learning: Queries, Keys, and Values](https://d2l.ai/chapter_attention-mechanisms-and-transformers/queries-keys-values.html)
- [Dive into Deep Learning: Attention Scoring Functions](https://d2l.ai/chapter_attention-mechanisms-and-transformers/attention-scoring-functions.html)

Critique before rewrite:

The roadmap now distinguishes self-attention from the more general target-source
attention boundary, but the glossary still had a smaller term surface: it named
`SelfAttentionHead` and the attention objects without separate entries for
self-attention, cross-attention, target length, source length, or
product-input morphism. That could make the glossary lag behind the strongest
new teaching distinction.

ML teaching critique:

The glossary should make the target/source split easy to retrieve. A learner
should be able to look up why `L x S` masks exist, why self-attention often has
one shared source sequence, and why cross-attention uses a target query source
plus a separate key-value source.

Category-theory critique:

The glossary should protect the count-inputs-first rule. Attention scoring and
value mixing must remain product-input morphisms until a larger composition
returns to the same public object.

Learner critique:

A reader who jumps from the roadmap to the glossary should not find only the
implementation type names. They need the plain distinction first, then the Rust
handles, then the categorical shape.

Rewrite decision:

- add core-alignment rows for target length, source length, self-attention,
  cross-attention, and product-input morphism,
- add a category-theory glossary entry for product-input morphisms,
- add attention glossary entries for target/source sequence length,
  self-attention, and cross-attention,
- explicitly state that the repository names cross-attention as a precision
  boundary but does not yet implement a full cross-attention block,
- guard the new glossary markers in `scripts/check-public-friction-matrix.py`,
- update the completion audit so terminology consistency names the new guard.

Validation:

```bash
python3 scripts/check-public-friction-matrix.py
python3 scripts/check-completion-audit.py
bash scripts/check.sh
scripts/check-textbook-readiness.sh
scripts/reader-feedback-status.sh --live
```

Result:

```text
Public friction matrix check passed.
Completion audit check passed.
Full publication gate passed.
Textbook readiness check failed only on the strict direct-reader completion
gate: 0/5 accepted direct-reader reports and 0 rewritten reports.
Live reader feedback status found no reader-confusion issues.
```

## Sweep 2 Pass 142: Parallel Self-Attention Projection Clarity

Files:

```text
book/src/roadmap.md
scripts/check-public-friction-matrix.py
```

Primary sources:

- [Attention Is All You Need](https://arxiv.org/abs/1706.03762)
- [PyTorch MultiheadAttention](https://docs.pytorch.org/docs/stable/generated/torch.nn.MultiheadAttention.html)
- [Dive into Deep Learning: Queries, Keys, and Values](https://d2l.ai/chapter_attention-mechanisms-and-transformers/queries-keys-values.html)

Critique before rewrite:

The self-attention section correctly said that query, key, and value roles come
from the same hidden sequence, but its code block could be misread as a
pipeline:

```text
HiddenSequence -> QuerySequence -> KeySequence -> ValueSequence
```

That weakens the category-precision goal because query, key, and value are
parallel role projections, not sequential transformations from one role into
the next.

ML teaching critique:

A learner should see that self-attention shares one source sequence while still
creating three distinct roles. The diagram should not imply that a query
becomes a key, or that a key becomes a value.

Category-theory critique:

The section should preserve the product-input reading of attention scoring.
The parallel morphisms out of `HiddenSequence` are ordinary role projections;
the later scoring boundary is where `QuerySequence x KeySequence` appears.

Learner critique:

The smallest useful fix is not another full explanation. It is a clearer shape
trace and one sentence that names the mistake the old trace could invite.

Rewrite decision:

- rewrite the self-attention special-case trace as three parallel
  `HiddenSequence -> RoleSequence` projections,
- add a sentence saying these are parallel projections, not a pipeline where
  queries turn into keys and keys turn into values,
- guard the exact parallel-projection markers in
  `scripts/check-public-friction-matrix.py`.

Validation:

```bash
python3 scripts/check-public-friction-matrix.py
python3 scripts/check-rewrite-log.py
bash scripts/check.sh
scripts/check-textbook-readiness.sh
scripts/reader-feedback-status.sh --live
```

Result:

```text
Public friction matrix check passed.
Rewrite log check passed.
Full publication gate passed.
Textbook readiness check failed only on the strict direct-reader completion
gate: 0/5 accepted direct-reader reports and 0 rewritten reports.
Live reader feedback status found no reader-confusion issues.
```
