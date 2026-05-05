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
- `reader confusion`
- `bug`
- `documentation`
- `sponsor-worthy milestone`

The label manifest lives at [.github/labels.yml](.github/labels.yml).

## Starter Issue Ideas

Good starter issues are concrete. Examples:

- `[good first feedback] Where does Chapter 1 become unclear?`
- `[needs diagram] Text -> Tokens -> TrainingPairs`
- `[needs Rust example] Morphism as typed transformation`
- `[ML intuition] Explain loss without framework magic`
- `[category theory precision] Review use of "object" and "morphism"`
- `[Rust idiom review] Improve newtype examples`
- `[exercise idea] Compose two transformations safely`
- `[chapter expansion] Turn bullet outline into full explanation`

See [community/starter-issues.md](community/starter-issues.md) for ready-to-open
issue drafts.

## Local Validation

Run the full gate before submitting code or book changes:

```bash
bash scripts/check.sh
```

For a smaller loop:

```bash
cargo run --example 01_token_sequence
cargo test --all-targets --all-features
bash scripts/check-mdbook-coverage.sh
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
- say what you expected
- say what actually happened
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
