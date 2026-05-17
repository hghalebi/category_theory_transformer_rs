# Paper-To-Rust Challenge

Hook:

```text
Stop summarizing papers. Compile them.
```

This track turns one idea from one ML or category-theory paper into a small
typed Rust invariant.

The challenge format is:

```text
paper claim -> Rust type -> invariant -> test -> executable example
```

## Seed Paper

| Paper | Challenge | Command |
| --- | --- | --- |
| [Adam: A Method for Stochastic Optimization](papers/adam/README.md) | optimizer state must move with parameters | `cargo run --example challenge_adam` |

## Submission Shape

A good Paper-To-Rust submission adds:

- one paper link
- one isolated idea from the paper
- one Rust type or boundary that makes the idea executable
- one regression test for the invariant
- one runnable example
- one short share card

Do not submit a whole-paper summary. Compile one idea.
