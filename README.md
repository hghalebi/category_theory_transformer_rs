<p align="center">
  <img src="logo.png" alt="Category Theory for Tiny ML in Rust logo" width="900">
</p>

# Category Theory for Tiny ML in Rust

> Tiny ML, Rust types, and category theory — executable structure, not AI magic.

Python made AI accessible.
Rust can make parts of AI understandable.

This is a first-principles, compile-checked tutorial for engineers who want to
understand the structure underneath machine-learning systems.

```bash
cargo run
```

Read the public draft, run the examples, and star the repo if you want more
executable AI education in Rust.

- Public book: <https://hghalebi.github.io/category_theory_transformer_rs/>
- First-session guide: [START_HERE.md](START_HERE.md)
- Feedback issues: <https://github.com/hghalebi/category_theory_transformer_rs/issues>

## Choose your path

| If you have... | Start here | Goal |
| --- | --- | --- |
| 5 minutes | `cargo run --example 01_token_sequence` | See text become typed training structure |
| 30 minutes | [START_HERE.md](START_HERE.md), then examples 01-03 | Learn the core path through the repo |
| A review session | [community/reader-review-packet.md](community/reader-review-packet.md) | Turn reader friction into a useful issue |
| A chapter question | [Open the reader-confusion form](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=reader-confusion.yml) | Point to the first unclear section |
| A contribution idea | [CONTRIBUTING.md](CONTRIBUTING.md) | Choose a concrete issue or improvement |

```mermaid
flowchart LR
    A[Text] --> B[TokenSequence]
    B --> C[TrainingPairs]
    C --> D[ModelState]
    D --> E[Prediction]
    E --> F[Loss]
    F --> G[Updated ModelState]

    classDef obj fill:#eef6ff,stroke:#2563eb,color:#111827;
    class A,B,C,D,E,F,G obj;
```

The project teaches this pipeline as typed transformations in Rust. The code
uses concrete types such as `TokenId`, `TokenSequence`, `TrainingSet`,
`Parameters`, `Distribution`, and `Loss` so the structure is visible and
compile-checked.

## Who this is for

This project is for:

- Rust developers curious about machine learning
- ML engineers who want stronger mental models
- mathematically curious software engineers
- technical founders who want to understand AI systems below the framework layer
- readers who enjoy small, explicit, executable abstractions

You do **not** need to be a category theorist.

You should be comfortable with basic programming ideas and curious enough to
follow small Rust examples.

## Why this exists

Most machine-learning education starts with frameworks.

That is useful, but it often hides the structure.

A tiny ML system already contains deep ideas:

```text
TokenId -> Vector -> Logits -> Distribution -> Loss
```

Prediction is a transformation.
Evaluation is a transformation.
Training is a transformation over parameters.

Category theory gives names to some of these shapes.
Rust makes the shapes explicit.

This project connects:

```text
English intuition -> tiny math idea -> Rust type -> runnable example
```

The goal is not mathematical decoration.
The goal is to make tiny ML systems easier to reason about.

## Start in 60 seconds

Clone the repository:

```bash
git clone https://github.com/hghalebi/category_theory_transformer_rs.git
cd category_theory_transformer_rs
```

Run the main walkthrough:

```bash
cargo run --bin category_ml
```

Run the five-minute first example:

```bash
cargo run --example 01_token_sequence
```

Expected shape:

```text
Raw input:
"rust makes ai structure visible"

TokenSequence:
[TokenId(12), TokenId(44), TokenId(7), TokenId(19), TokenId(91)]

TrainingPairs:
(TokenId(12) -> TokenId(44))
(TokenId(44) -> TokenId(7))
(TokenId(7) -> TokenId(19))
(TokenId(19) -> TokenId(91))

Typed transformation:
Text -> TokenSequence -> TrainingPairs

No framework magic.
Just explicit structure.
```

Then run the full local gate:

```bash
bash scripts/check.sh
```

The learning artifact is CI-verified: Rust formatting, clippy, tests, all
examples, the main demo, prose checks, source-snapshot coverage, the book build,
and chapter tests run together before publication.

## Read the book

The public draft is available here:

```text
https://hghalebi.github.io/category_theory_transformer_rs/
```

Suggested starting path:

```text
Welcome
-> Course Map
-> Domain Objects
-> Morphism and Composition
-> Tiny ML Pipeline
```

To build the book locally:

```bash
bash scripts/build-mdbook.sh
```

The generated HTML is written to `book/html/`.

If you want live reload while reading locally, run `mdbook serve` from the
repository root and open the URL printed in your terminal.

## The small promise

After the first path, you should be able to explain:

1. why `TokenId` is safer than a raw `usize`
2. why a morphism is a typed transformation
3. why composition means outputs and inputs connect safely
4. why training can be seen as `Parameters -> Parameters`

The full course then gives names to those ideas and connects them to runnable
Rust code.

## What you will learn

By following the project, you will learn how to see tiny ML systems as
composable structures:

| Concept | Plain-English idea | Rust intuition |
| --- | --- | --- |
| Domain object | A meaningful type of thing | `TokenId`, `Vector`, `Logits` |
| Morphism | A typed transformation | `Morphism<Input, Output>` |
| Composition | Connecting transformations safely | output type matches input type |
| Pipeline | A chain of transformations | data flowing through typed stages |
| Training | Updating parameters | state transition |
| Endomorphism | A transformation from a thing to itself | `Parameters -> Parameters` |
| Functor | Structure-preserving mapping | `VecFunctor`, `OptionFunctor` |
| Monoid | Composable accumulation | `PipelineTrace` |
| Chain rule | Local derivatives composed backward | `MulOp` forward and backward pass |

This is not meant to replace full ML frameworks.
It is meant to make their hidden structure easier to see.

## Learning path

### 1. Foundations

Start here if category-theory language is new to you:

```text
1. Course Map
2. Domain Objects
3. Morphism and Composition
4. The Tiny ML Pipeline
```

You will learn the basic language of objects, arrows, and composition through
tiny Rust examples.

### 2. Training

```text
5. Training as an Endomorphism
```

You will see training as a structured update:

```text
Parameters -> Parameters
```

Instead of treating training as magic, the book models it as a controlled state
transition.

### 3. Structure

```text
6. Functors, Naturality, Monoids, and Chain Rule
7. Seven Sketches Through Rust
```

This section connects category-theory vocabulary to engineering patterns.

The goal is not to memorize names.
The goal is to recognize structure when it appears in code.

### 4. Practice

```text
8. Exercises
9. Glossary
10. References
11. Transformer Roadmap
12. Repository Source Snapshots
```

Use this section to test your understanding, review terminology, and connect
the tiny examples to larger AI systems.

## A tiny example

A machine-learning pipeline can be seen as a sequence of typed transformations:

```text
TokenId -> Vector -> Logits -> Distribution
```

In Rust, the teaching goal is to make illegal connections harder to express.

The runnable composition example lives in
[examples/02_morphism_composition.rs](examples/02_morphism_composition.rs):

```rust
use category_theory_transformer_rs::{
    Compose, CtResult, Embedding, LinearToLogits, Logits, ModelDimension, Morphism,
    Parameters, Softmax, TokenId, Vector, VocabSize,
};

fn main() -> CtResult<()> {
    let params = Parameters::init(VocabSize::new(5)?, ModelDimension::new(4)?);
    let token = TokenId::new(1);
    let embedding = Embedding::from_parameters(&params);
    let linear = LinearToLogits::from_parameters(&params);

    let token_to_logits = Compose::<_, _, Vector>::new(
        embedding.clone(),
        linear.clone(),
    );
    let token_to_distribution = Compose::<_, _, Logits>::new(token_to_logits, Softmax);

    let vector = embedding.apply(token)?;
    let logits = linear.apply(vector)?;
    let distribution = Softmax.apply(logits)?;
    let composed_distribution = token_to_distribution.apply(token)?;

    println!("stage output: {:?}", distribution.as_slice());
    println!("composed output: {:?}", composed_distribution.as_slice());

    Ok(())
}
```

Run it:

```bash
cargo run --example 02_morphism_composition
```

This is intentionally tiny.
The point is not performance yet.
The point is structure:

```text
TokenId -> Vector -> Logits -> Distribution
```

The command also prints the middle objects, so the composed arrow does not hide
the `Vector` and `Logits` stages that make it legal.

Once the structure is visible, we can ask better questions about correctness,
composition, training, and eventually performance.

## Project map

```text
.
├── book/                  # public book source and generated HTML
├── community/             # workshop, reading-club, and feedback material
├── docs/                  # learning paths, glossary, and FAQ
├── examples/              # runnable learning examples
├── exercises/             # beginner, intermediate, and advanced practice
├── lessons/               # compact lesson notes
├── scripts/               # validation and book-build scripts
├── src/                   # compile-checked Rust teaching modules
├── START_HERE.md          # first-session path
├── ROADMAP.md             # project milestones
├── CONTRIBUTING.md        # concrete contribution paths
├── SPONSORS.md            # sponsor rationale
├── GOVERNANCE.md          # project decision rules
├── CHANGELOG.md           # visible momentum
└── README.md              # project entry point
```

The repository is designed to be read in two ways:

```text
Book-first:
Read the chapter, then run the matching code.

Code-first:
Run the example, then read the explanation.
```

Both paths are valid.

The best path is probably:

```text
Read a tiny idea -> run a tiny program -> change one line -> observe what breaks
```

The compiler is part of the teacher.

## Rust module map

- `src/domain.rs`: typed nouns used by the whole tutorial
- `src/category.rs`: morphisms, identity, composition, and endomorphisms
- `src/ml.rs`: token windowing, embedding, linear projection, softmax, cross entropy
- `src/training.rs`: training as a repeated parameter endomorphism
- `src/structure.rs`: functors, natural transformations, and monoids
- `src/calculus.rs`: local derivative and chain-rule example
- `src/attention.rs`: typed query-key scoring, masks, attention weights, value mixing, head concatenation, output projection, residual addition, layer normalization, position-wise feed-forward structure, hidden projections, block sketches, sequence readout, structured Transformer state, readout-only training, local feed-forward training, and a composed block training step with query/key/value gradients
- `src/sketches.rs`: Rust models for seven applied-category-theory sketches
- `src/demo.rs`: the complete terminal walkthrough

## Runnable examples

| Command | Concept | Learner win |
| --- | --- | --- |
| `cargo run --example 01_token_sequence` | Text to training pairs | See raw text become typed structure |
| `cargo run --example 01_domain_objects` | Domain objects | See ML nouns as Rust types |
| `cargo run --example 02_morphism_composition` | Morphisms and composition | See typed transformations compose |
| `cargo run --example 03_training_endomorphism` | Training | See training as `Parameters -> Parameters` |
| `cargo run --example 04_structure_and_calculus` | Structure and calculus | See functors, monoids, and chain-rule sketches |
| `cargo run --example 05_seven_sketches` | Applied sketches | See category ideas beyond tiny ML |
| `cargo run --example 06_attention_scores` | Attention roadmap | See query-key scores get masked, normalized, mixed, concatenated, projected, passed through blocks, wrapped in structured state, and updated through readout, feed-forward, attention, and composed block training steps |

These examples are small by design.
Each one is meant to isolate a concept before the book combines it with the
next idea.

## Learn live

If you prefer a guided walkthrough, join the public workshop:

```text
https://luma.com/event/evt-Pb1kYMQvzs8JrQq
```

## Current status

This is a working public draft.

Some chapters are stable.
Some chapters are skeletal.
Some sections are intentionally public before they are polished.

The goal is to build the clearest possible path from tiny ML concepts to typed
Rust implementations, with public feedback from readers.

If something feels too compressed, unclear, or too bullet-point-like, please
open an issue. That feedback is useful.

Status labels:

| Label | Meaning |
| --- | --- |
| Stable | The explanation and code are unlikely to change heavily |
| Draft | The section is readable but still evolving |
| Sketch | The idea is present but needs clearer writing or examples |
| Planned | The section is part of the roadmap |

Chapter maturity:

| Chapter | Status | Best feedback |
| --- | --- | --- |
| Welcome | Stable draft | First-run promise and reader contract |
| Course Map | Stable draft | Path clarity and module-to-pipeline mapping |
| Domain Objects | Stable draft | Clarity and Rust idiom |
| Morphism and Composition | Draft | More examples and composition diagrams |
| The Tiny ML Pipeline | Draft | Diagram review and ML intuition |
| Training as an Endomorphism | Draft | Training-loop intuition |
| Functors, Naturality, Monoids, and Chain Rule | Draft | Law tracing and terminology precision |
| Seven Sketches Through Rust | Draft | Transfer clarity across sketches |
| Exercises | Draft | Evidence quality and transfer difficulty |
| Transformer Roadmap | Sketch | Attention-shape clarity and training-state boundaries |

## Roadmap

Near-term improvements:

- clearer first-session path
- more runnable Rust examples
- better diagrams for the tiny ML pipeline and training loop
- fewer dense bullet-style explanations
- more prose before abstractions
- chapter maturity labels
- contributor-friendly issues
- workshop-ready exercises
- bridge from tiny ML toward transformer components

Longer-term direction:

- learner-facing gradient-checking exercises over the current weight, bias, and normalization checks
- tiny tokenizer examples
- training-loop examples
- category-theory glossary for engineers
- mechanically sympathetic Rust versions after the pedagogical versions
- performance notes where abstraction meets real systems constraints

See [ROADMAP.md](ROADMAP.md) for the living project roadmap.

## How to contribute

The most valuable contributions are not only code.

Good contributions include:

- pointing out unclear explanations
- opening issues where a chapter becomes hard to follow
- suggesting smaller examples
- improving diagrams
- adding exercises
- simplifying Rust code
- fixing terminology
- testing the book as a learner

A great issue looks like this:

```text
Chapter:
Section:
Command or file I tried:
What I understood:
Where I got confused:
What I expected next:
Suggestion:
```

Example:

```text
Chapter: Morphism and Composition
Section: Typed transformations
Command or file I tried: cargo run --example 02_morphism_composition
What I understood: a morphism is a typed transformation
Where I got confused: the word "morphism" appeared before I had an intuition
What I expected next: a concrete Rust function before the abstract explanation
Suggestion: introduce it first as "a typed transformation"
```

That kind of feedback is useful.

For structured reading feedback, use
[community/reader-review-guide.md](community/reader-review-guide.md), then open
the [reader-confusion issue form](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=reader-confusion.yml).
For code or documentation changes, start with [CONTRIBUTING.md](CONTRIBUTING.md).

## Good first issues

Look for issues labeled:

```text
good first feedback
needs Rust example
needs diagram
chapter expansion
ML intuition
category theory precision
Rust idiom review
exercise idea
glossary needed
reader confusion
documentation
```

The initial public issue set gives readers concrete handles:

- [[good first feedback] Where does the book become unclear?](https://github.com/hghalebi/category_theory_transformer_rs/issues/1)
- [[needs diagram] Text -> Tokens -> TrainingPairs -> ModelState pipeline](https://github.com/hghalebi/category_theory_transformer_rs/issues/2)
- [[needs Rust example] Morphism as typed transformation](https://github.com/hghalebi/category_theory_transformer_rs/issues/3)
- [[chapter expansion] Turn bullet sections into full explanations](https://github.com/hghalebi/category_theory_transformer_rs/issues/4)
- [[FAQ] What does this project unlock?](https://github.com/hghalebi/category_theory_transformer_rs/issues/5)

If you are new to the project, the best contribution is usually clarity
feedback.

This project should be understandable before it becomes impressive.

## Community and feedback

This project is connected to a broader learning effort around foundational AI
systems, Rust, and first-principles technical education.

Useful ways to engage:

- star the repository
- read the public draft
- open an issue with feedback
- suggest a tiny Rust example
- share the project with a Rust or ML engineer
- join a reading session or workshop when announced

The project is especially interested in feedback from:

```text
Rust engineers
ML engineers
compiler/type-system people
technical educators
mathematically curious software builders
```

## Design philosophy

### 1. Small before large

Tiny examples reveal structure better than giant frameworks.

### 2. Runnable before impressive

If an idea cannot survive a small program, it may not be understood yet.

### 3. Types as explanations

Rust types are not just implementation details.
They can teach domain structure.

### 4. Category theory as naming, not decoration

The goal is to name useful patterns, not to intimidate the reader.

### 5. Pedagogy before cleverness

A clever abstraction that makes the reader feel lost has failed.

A small example that makes the reader say "I see it now" is doing the work.

## Quality gate

Before trusting changes, run:

```bash
bash scripts/check.sh
```

That checks formatting, clippy, unit tests, examples, the full demo,
prose-style rules, source snapshot coverage, the book build, and chapter tests.

## Citation

If you reference this project, you can cite it as:

```text
Hamze Ghalebi and contributors.
Category Theory for Tiny ML in Rust.
Public draft, GitHub.
```

## License

The repository does not declare a final license yet.

Choosing the code and book licenses is a project-owner decision on the roadmap
before broader community promotion.

Until a license is added, do not assume reuse rights beyond the explicit
reference guidance inside the book draft.
