# Reader Review Packet

Use this packet when someone wants to review the book but does not know where
to start.

The goal is one useful issue, not a complete review of the whole project.
Maintainers inviting reviewers can use `community/reviewer-outreach.md`.

Public book:

```text
https://hghalebi.github.io/category_theory_transformer_rs/
```

GitHub repository:

```text
https://github.com/hghalebi/category_theory_transformer_rs
```

## Reviewer Goal

Spend 20 to 30 minutes and report the first place where the learning path
becomes unclear.

A useful report names:

```text
what you ran or read
what made sense
where the mental model broke
what small edit would help
```

## Reviewer Context Briefs

Use one brief that matches your background. The project needs one concrete
report from each context, not a full review from every reader.

| Reviewer context | Start with | Then inspect | One useful report names |
| --- | --- | --- | --- |
| Rust engineer | `cargo run --example 01_token_sequence` and `cargo run --bin category_ml` | `book/src/01-domain-objects.md` or `book/src/02-morphisms-composition.md` | the first type, trait, constructor, error, or example that feels unidiomatic or underexplained |
| ML engineer | `cargo run --example 03_training_endomorphism` and `cargo run --example 06_attention_scores` | `book/src/03-ml-pipeline.md`, `book/src/04-training-endomorphism.md`, or `book/src/roadmap.md` | the first point where logits, probabilities, loss, updates, attention roles, self-attention versus cross-attention, or training state loses its ML meaning |
| Category-theory reader | `cargo run --example 02_morphism_composition`, `04_structure_and_calculus`, or `05_seven_sketches` | `book/src/02-morphisms-composition.md`, `book/src/05-structure-and-calculus.md`, `book/src/seven-sketches-rust.md`, or the roadmap category-shape diagnostic | the first term, law, diagram, product-input boundary, self-vs-cross attention boundary, endomorphism, or endofunctor warning that needs more precision |
| Technical educator | no clone required; start with the public book path | README, start guide, Welcome, Course Map, Exercises, and this review packet | the first place where the learner is not told what to run, read, explain, or try next |
| Beginner-adjacent reader | `cargo run --example 01_token_sequence` if comfortable, otherwise the public book path | Welcome, Course Map, Domain Objects, and Morphism and Composition | the first sentence, command, output, or term that becomes too compressed |

For every context, the strongest report has this shape:

```text
I tried this path.
This part made sense.
This exact line, output, command, or term broke the mental model.
This one small edit would help.
```

Do not report a broad opinion such as "more examples are needed" unless you can
name the first exact place where an example would remove confusion.

## Context-Specific Report Links

GitHub supports issue URLs that select a template and fill custom text fields.
These links only prefill the title, location, and command or file path. Please
still select your reviewer context in the form and write the actual point where
the mental model broke.

| Reviewer context | Report link |
| --- | --- |
| Rust engineer | [Open Rust engineer report](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=reader-confusion.yml&title=%5Breader+confusion%5D+Rust+engineer+brief&location=book%2Fsrc%2F01-domain-objects.md+or+book%2Fsrc%2F02-morphisms-composition.md&command=cargo+run+--example+01_token_sequence%0Acargo+run+--bin+category_ml) |
| ML engineer | [Open ML engineer report](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=reader-confusion.yml&title=%5Breader+confusion%5D+ML+engineer+brief&location=book%2Fsrc%2F03-ml-pipeline.md%2C+book%2Fsrc%2F04-training-endomorphism.md%2C+or+book%2Fsrc%2Froadmap.md&command=cargo+run+--example+03_training_endomorphism%0Acargo+run+--example+06_attention_scores) |
| Category-theory reader | [Open category-theory reader report](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=reader-confusion.yml&title=%5Breader+confusion%5D+category-theory+reader+brief&location=book%2Fsrc%2F02-morphisms-composition.md%2C+book%2Fsrc%2F05-structure-and-calculus.md%2C+book%2Fsrc%2Fseven-sketches-rust.md%2C+or+book%2Fsrc%2Froadmap.md&command=cargo+run+--example+02_morphism_composition%0Acargo+run+--example+04_structure_and_calculus%0Acargo+run+--example+05_seven_sketches) |
| Technical educator | [Open technical educator report](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=reader-confusion.yml&title=%5Breader+confusion%5D+technical+educator+brief&location=README.md%2C+START_HERE.md%2C+community%2Freader-review-packet.md%2C+book%2Fsrc%2Fwelcome.md%2C+book%2Fsrc%2F00-map.md%2C+or+book%2Fsrc%2Fexercises.md&command=public+book+review+path+or+local+file+review) |
| Beginner-adjacent reader | [Open beginner-adjacent reader report](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=reader-confusion.yml&title=%5Breader+confusion%5D+beginner-adjacent+reader+brief&location=Welcome%2C+Course+Map%2C+Domain+Objects%2C+or+Morphism+and+Composition&command=cargo+run+--example+01_token_sequence+or+public+book+path) |

## Path A: First-Run Review

Use this path if you are new to the project.

Run:

```bash
cargo run --example 01_token_sequence
```

Then read:

```text
README.md
START_HERE.md
book/src/welcome.md
book/src/00-map.md
```

Open the
[reader-confusion issue form](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=reader-confusion.yml)
if any step does not make the next step obvious.

## Path B: Core Chapter Review

Use this path if you can spend a little more time.

Run:

```bash
cargo run --bin category_ml
```

Then choose one chapter:

```text
book/src/01-domain-objects.md
book/src/02-morphisms-composition.md
book/src/03-ml-pipeline.md
book/src/04-training-endomorphism.md
book/src/05-structure-and-calculus.md
```

Report the first place where one of these explanations becomes too compressed:

```text
Rust syntax
ML concept
category-theory concept
```

## Path C: Attention And Gradient Review

Use this path if you are comfortable reading Rust code.

Run:

```bash
cargo run --example 06_attention_scores
cargo test finite_difference --lib
```

Then inspect:

```text
src/attention.rs
book/src/roadmap.md
book/src/exercises.md
exercises/advanced/README.md
```

Report whether the attention and gradient-checking path clearly separates:

```text
implemented tiny structure
teaching approximation
future production-scale work
```

Also test the self-attention versus cross-attention boundary in
`book/src/roadmap.md`. After running the example, check whether these ideas are
separate:

```text
same source for Q, K, V -> self-attention case
parallel HiddenSequence -> QuerySequence, KeySequence, ValueSequence projections
not QuerySequence -> KeySequence -> ValueSequence
separate query and key-value sources -> cross-attention case
target length L x source length S
product-input morphism before any endomorphism claim
```

Also test the category-shape diagnostic in `book/src/roadmap.md`. After
running the example, classify these boundaries:

```text
QuerySequence x KeySequence -> AttentionScores
AttentionScores x AttentionMask -> AttentionScores
LayerNormalization : HiddenSequence -> HiddenSequence
TransformerTrainingState -> TransformerTrainingState
HiddenSequence x MultiHeadOutput -> HiddenSequence
```

Report the first boundary where product-input morphism, endomorphism, or
illegal composition felt unclear.

## Path D: Public Book Review

Use this path if you want to review without cloning the repository first.

Open:

```text
https://hghalebi.github.io/category_theory_transformer_rs/
```

Then read one short path:

```text
Welcome
Course Map
Domain Objects
Morphism and Composition
```

Keep the GitHub repository open for source links and examples:

```text
https://github.com/hghalebi/category_theory_transformer_rs
```

Report the first place where the public book stops giving you one of these:

```text
the concept being learned
the Rust object or function that anchors it
the next command, file, or exercise to try
```

This path can still count as direct reader evidence if the report names the
public page or section read, the last clear idea, and the exact point where the
mental model broke.

## Open The Issue

Use the
[reader-confusion issue form](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=reader-confusion.yml).

The form asks for:

```text
review path
reviewer context
location
command or file tried
friction lens
confusing text or output
what made sense before that point
where the mental model broke
what you expected next
smallest useful fix
```

## High-Signal Examples

Good:

```text
I ran cargo run --example 02_morphism_composition.
The TokenId -> Vector step made sense.
I got stuck when Logits became Distribution.
The text did not yet explain raw scores versus probabilities.
One sentence before Softmax would help.
```

Good:

```text
I read the finite-difference worked example.
The parameter + epsilon path made sense.
I did not understand why the one-step update gives an inferred gradient.
A short reminder of parameter <- parameter - learning_rate * gradient would help.
```

Not useful:

```text
This should explain everything more.
```

Not useful:

```text
This is not a production Transformer.
```

The project already says the Transformer material is a tiny typed teaching
path. Useful feedback points to the first unclear boundary inside that path.

## Review Outcome

One strong issue should make one next edit obvious.

That is enough.

Maintainers should triage submitted issues with
`community/reader-feedback-triage.md` and record accepted direct reports in
`community/reader-feedback-inbox.md`.
