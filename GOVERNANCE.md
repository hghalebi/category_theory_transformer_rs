# Governance

This project is a working public book and Rust lab. Governance should keep the
material coherent while still making outside contribution possible.

## Maintainer Responsibilities

Maintainers are responsible for:

- preserving the project scope
- keeping examples runnable
- reviewing category-theory terminology carefully
- keeping the README and first-run path clear
- deciding which suggestions become roadmap work
- maintaining citation, reuse, and permission boundaries before broader
  promotion

## Decision Rules

Changes are accepted when they improve at least one of these outcomes:

- a reader understands the project faster
- a learner gets a runnable result sooner
- a chapter becomes clearer or more precise
- a Rust example becomes more idiomatic
- an exercise becomes easier to attempt and verify
- the public contribution path becomes more obvious

Changes should be rejected or revised when they:

- add theory without executable grounding
- add code that is not used by a lesson, example, or test
- make the repository harder to navigate
- weaken terminology precision
- break the full validation gate

## Scope Boundaries

This is not a production ML framework.

It is a learning system for tiny ML, Rust types, typed transformations,
composition, training loops, and applied category-theory intuition.

## Contributor Path

Contributors move through the ladder in
[community/contributor-ladder.md](community/contributor-ladder.md):

1. reader feedback
2. documentation clarification
3. example or exercise contribution
4. reviewer for Rust, ML, or category-theory precision
5. maintainer-level roadmap work

## Citation And Reuse Policy

The project declares citation and reuse terms in [LICENSE.md](LICENSE.md), with
machine-readable citation metadata in [CITATION.cff](CITATION.cff).

The public book will always remain open access at
<https://hghalebi.github.io/category_theory_transformer_rs/>. The source
repository is available at
<https://github.com/hghalebi/category_theory_transformer_rs>.

Maintainers should keep that policy visible in reader-facing docs and should
not accept changes that imply unrestricted commercial or organizational
multi-person reproduction of the book, exercises, diagrams, or substantial
repository material. Citation should remain mandatory where reuse is allowed,
but citation alone is not permission and should not be presented as a substitute
for written permission when substantial material is reused by or for a company,
team, workshop, course, cohort, class, training program, or other group setting
with more than one person.
