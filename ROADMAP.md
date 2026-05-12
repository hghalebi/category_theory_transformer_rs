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
onboarding, diagram refinement, issue structure, and sponsor-worthy milestones.

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

## Milestone 4: Sponsor-Worthy Work

Goal: give companies and technical sponsors clear reasons to support the work.

Status: planned.

Sponsor-worthy tracks:

- a polished Rust-first AI foundations curriculum
- diagrams and visual explanations
- workshop-ready exercises
- CI-verified learning examples
- advanced modules that turn the current finite-difference checks into richer learner-facing gradient-checking exercises over structured Transformer state
- facilitator notes for teams using the material internally

## Milestone 5: License Decision

Goal: choose explicit licenses for code and book material before serious public
promotion.

Status: needed.

This is a project-owner decision. The likely shape is separate handling for
Rust code and prose/book content, but no license is declared until the owners
choose it.
