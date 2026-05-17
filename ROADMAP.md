# Roadmap

This roadmap turns the project from a working draft into a mature public Rust
learning system.

## Current Status

The project already has:

- a compile-checked Rust crate
- runnable examples
- a public book
- source snapshots inside the book
- a full validation script
- GitHub Pages deployment

The next phase is about adoption: clearer first-run experience, contributor
onboarding, public challenges, diagram refinement, issue structure, and
sponsor-worthy milestones.

## Milestone 1: First Visitor Clarity

Goal: a visitor understands the project in 30 seconds and gets a runnable win
in five minutes.

Status: in progress.

Tasks:

- keep README pain-first and action-first
- maintain `START_HERE.md`
- keep `cargo run --example 01_token_sequence` fast and stable
- add screenshots or terminal-output snippets when useful

## Milestone 2: Chapter Completeness

Goal: every major concept has prose, source snapshot, runnable example, and
exercise.

Status: in progress.

Tasks:

- refine diagrams for the text-to-training-pairs and training-loop flows
- expand exercises by difficulty
- add glossary entries when readers report confusion
- keep category-theory terminology precise

## Milestone 3: Contributor System

Goal: a reader can become a contributor without guessing what help is useful.

Status: started.

Tasks:

- maintain issue templates
- maintain label manifest
- keep starter issues specific
- add contributor-ladder guidance
- review first-time contributor friction monthly

## Milestone 4: Public Challenge Tracks

Goal: make the project shareable through small compiler-fix and paper-to-code
loops.

Status: seeded.

Entry point: [challenges/README.md](challenges/README.md).

Tracks:

- Typed AI Rustlings: learn AI by fixing compiler errors
- Paper-To-Rust: stop summarizing papers and compile one idea
- challenge completion reports with concrete command or compiler evidence
- reference solutions and tests for every public challenge
- one runnable seed example for each paper challenge

Current seed:

- `token_id_not_usize`
- `logits_are_not_probabilities`
- Adam optimizer state as `AdamModelState -> AdamModelState`

## Milestone 5: Sponsor-Worthy Work

Goal: give companies and technical sponsors clear reasons to support the work.

Status: planned.

Sponsor-worthy tracks:

- a polished Rust-first AI foundations curriculum
- diagrams and visual explanations
- workshop-ready exercises
- CI-verified learning examples
- advanced modules that turn the current finite-difference checks into richer learner-facing gradient-checking exercises over structured Transformer state
- facilitator notes for teams using the material internally

## Milestone 6: Citation And Reuse Policy

Goal: keep citation, reuse, and permission boundaries explicit before broader
public promotion.

Status: initial policy declared.

The repository now includes [LICENSE.md](LICENSE.md) and [CITATION.cff](CITATION.cff).
The public book will always remain open access at
<https://hghalebi.github.io/category_theory_transformer_rs/>. The source
repository is available at
<https://github.com/hghalebi/category_theory_transformer_rs>. Short quotation,
personal study, links, and noncommercial educational discussion are allowed
with citation. Plain rule: group reuse with more than one person needs written
permission. Commercial or organizational reuse involving more than one person
requires written permission when it reproduces, adapts, distributes, or teaches
material from the book or repository beyond short quotation, linking, review,
and individual-study allowances. This includes company workshops, internal team
workshops, company reading groups based on copied or adapted material, paid
training material, course packs, adapted slide decks, handouts, labs, and
workshop packets. If the material is reused by or for a company, team, class,
workshop, cohort, course, or training program with more than one person,
request written permission first. Citation alone is not permission and does not
replace written permission for group commercial or organizational reuse.
Permission requests should start through the source repository. Future Kindle
or hard copy editions are support editions, not access gates; they will not
remove free public access to the online book.
