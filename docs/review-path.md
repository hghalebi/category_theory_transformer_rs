# Review Path

Use this path if you want to help improve the book without writing code.

The goal is one precise chapter clarity report, not a broad review.

```text
run or read one path
name the last clear idea
name the first unclear point
suggest the smallest fix
```

Open the report here:

<https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=chapter-clarity.yml>

## Choose One Path

| Perspective | Public path | First action |
| --- | --- | --- |
| Rust engineer | [Rust engineer first-run path](https://github.com/hghalebi/category_theory_transformer_rs/issues/8) | Run `cargo run --example 01_domain_objects` and `cargo run --example 02_morphism_composition` |
| ML engineer or learner | [ML engineer training and attention path](https://github.com/hghalebi/category_theory_transformer_rs/issues/9) | Run `cargo run --example 03_training_endomorphism` and `cargo run --example 07_transformer_training_state` |
| Category-theory reader | [Category-theory precision path](https://github.com/hghalebi/category_theory_transformer_rs/issues/10) | Run `cargo run --example 02_morphism_composition` and `cargo run --example 06_attention_scores` |
| Technical educator | [Technical educator learning path](https://github.com/hghalebi/category_theory_transformer_rs/issues/11) | Review `README.md`, `START_HERE.md`, `docs/educator-path.md`, and `book/src/exercises.md` |
| Beginner-adjacent learner | [Beginner-adjacent first confusion path](https://github.com/hghalebi/category_theory_transformer_rs/issues/12) | Run `cargo run --example 01_token_sequence` or read the public start path |

If none of those fits, choose the closest one and select `other` in the issue
form.

## What To Report

A useful report includes:

```text
Perspective:
Friction lens:
Chapter or file:
Command or page tried:
First unclear sentence, output line, table row, code block, or exercise:
Last clear idea:
What you expected:
What happened instead:
What would have helped:
```

The strongest report points to one exact location.

Useful:

```text
I ran cargo run --example 02_morphism_composition.
The TokenId -> Vector step made sense.
I got stuck at Vector != Logits because I did not yet know why Softmax needs Logits.
A one-sentence reminder before the illegal composition line would help.
```

Less useful:

```text
The category theory chapter needs more explanation.
```

The second report names a topic, but not the first blocked step.

## What To Avoid

Do not include:

- email addresses
- private messages
- personal notes
- broad praise without a blocked step
- broad strategy requests unrelated to one chapter, example, or exercise

## Good Review Outcome

A good 20-minute review should make one next edit obvious.

The report does not need to solve the problem. It only needs to show exactly
where the current learning path stopped working.
