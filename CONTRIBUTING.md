# Contributing

This project needs specific feedback more than vague encouragement.

If you want to help, choose one concrete surface and make the reader path
clearer, more executable, or more precise.

## Good First Contributions

Useful first contributions include:

- report the exact sentence where a chapter becomes unclear
- add a small Rust example for one concept
- propose a diagram for one transformation
- review a category-theory term for precision
- review a Rust example for idiom
- add an exercise idea with an expected answer
- improve glossary coverage for one confusing term

For a fast reader signal, use the quick issue form:

<https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=quick-reader-report.yml>

For chapter clarity feedback, use the focused issue form:

<https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=chapter-clarity.yml>

## Open The Right Report

If you already know the perspective you are bringing, use the closest report
link. The link fills the route, not the evidence; the evidence signal should
come from what you personally read, ran, or attempted.

| Perspective | Report link |
| --- | --- |
| Rust engineer | [Open Rust engineer report](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=chapter-clarity.yml&title=%5Bgood+first+feedback%5D+Rust+engineer+brief&location=docs%2Frust-path.md&command=cargo+run+--example+01_domain_objects%0Acargo+run+--example+02_morphism_composition%0Acargo+test+domain%3A%3Atests+--lib%0Acargo+test+category%3A%3Atests+--lib) |
| ML engineer or learner | [Open ML engineer report](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=chapter-clarity.yml&title=%5Bgood+first+feedback%5D+ML+engineer+brief&location=docs%2Fml-path.md&command=cargo+run+--example+01_token_sequence%0Acargo+run+--bin+category_ml%0Acargo+run+--example+03_training_endomorphism%0Acargo+run+--example+07_transformer_training_state) |
| Category-theory reader | [Open category-theory reader report](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=chapter-clarity.yml&title=%5Bgood+first+feedback%5D+category-theory+reader+brief&location=book%2Fsrc%2Froadmap.md+-%3E+Category+Shape+Diagnostic+-%3E+Reader+Evidence+Handoff&command=cargo+run+--example+06_attention_scores) |
| Technical educator | [Open technical educator report](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=chapter-clarity.yml&title=%5Bgood+first+feedback%5D+technical+educator+brief&location=README.md%2C+START_HERE.md%2C+docs%2Feducator-path.md%2C+book%2Fsrc%2Fwelcome.md%2C+book%2Fsrc%2F00-map.md%2C+or+book%2Fsrc%2Fexercises.md&command=public+book+review+path+or+local+file+review) |
| Beginner-adjacent learner | [Open beginner-adjacent learner report](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=chapter-clarity.yml&title=%5Bgood+first+feedback%5D+beginner-adjacent+reader+brief&location=Welcome%2C+Course+Map%2C+Domain+Objects%2C+or+Morphism+and+Composition&command=cargo+run+--example+01_token_sequence+or+public+book+path) |

If you want a guided review route first, use [docs/review-path.md](docs/review-path.md).
If you want to see the shape of useful evidence before opening an issue, use
[docs/review-examples.md](docs/review-examples.md).

Good clarity feedback includes the chapter or file, friction lens, command or
page tried, evidence signal, first unclear sentence or output line, last clear
idea, what you expected, what happened instead, and what would have helped.
It should explicitly name the last clear idea before the path broke.

## Contribution Terms

By opening a pull request, you agree that your contribution may be included in
the project under the terms in [LICENSE.md](LICENSE.md). Do not contribute
material unless you have the right to submit it.

Public contributions should improve the book, executable Rust code, exercises,
or public issue paths. Keep submissions focused on reader-visible material.

## Issue Labels

The project uses these labels to keep contribution work specific:

- `good first feedback`
- `needs Rust example`
- `needs diagram`
- `chapter expansion`
- `ML intuition`
- `category theory precision`
- `Rust idiom review`
- `exercise idea`
- `glossary needed`
- `bug`
- `documentation`
- `sponsor-worthy milestone`
- `FAQ`

The label manifest lives at [.github/labels.yml](.github/labels.yml).

## Starter Issue Ideas

Good starter issues are concrete. Examples:

- `[good first feedback] Where does the book become unclear?`
- `[needs diagram] Text → Tokens → TrainingPairs → ModelState pipeline`
- `[needs Rust example] Morphism as typed transformation`
- `[chapter expansion] Turn bullet sections into full explanations`
- `[FAQ] What does this project unlock?`

Use the labels above to keep new issues focused and actionable.

## Local Validation

Run the full gate before submitting code or book changes:

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
mdbook build
mdbook test
```

For a smaller loop:

```bash
cargo run --example 01_token_sequence
cargo test --all-targets --all-features
mdbook test
```

## Contribution Rules

Keep changes small and reviewable.

For Rust code:

- prefer typed values over raw primitives at module boundaries
- use `CtResult` and `CtError` for fallible tutorial logic
- add or update examples when a concept changes
- avoid hiding important ideas behind clever abstractions

For book material:

- explain one concept at a time
- anchor explanations in real source files
- add a checkpoint or exercise when a new concept appears
- keep the learner-facing book focused on Rust, ML, and category theory

For issues:

- quote the confusing sentence or command
- name the command or public page you tried
- say what you expected
- say what actually happened
- say what would have helped: a smaller example, diagram, glossary entry,
  rewritten paragraph, or expected output
- link the file or chapter when possible

## Pull Request Shape

A useful pull request includes:

- what changed
- why it changed
- how a reader benefits
- commands run
- any remaining gap

Small educational improvements are welcome. The project grows by making the
next reader's path less confusing.
