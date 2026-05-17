# Challenges

The problem this chapter solves is:

> After a chapter explains an idea, how can a reader practice the same boundary
> through compiler feedback and a tiny paper-to-code translation?

The book explains the ideas. The challenge tracks make them public practice.

There are two tracks:

- **Typed AI Rustlings:** learn AI by fixing compiler errors.
- **Paper-To-Rust:** stop summarizing papers. Compile one idea.

## Chapter Outcomes

By the end of this chapter, you should be able to:

- explain how a `TokenId` or `Distribution` challenge uses a compiler or test
  signal to expose one AI boundary,
- translate one paper claim into a Rust boundary, invariant, and evidence
  command without summarizing the whole paper,
- distinguish challenge completion evidence from accepted textbook reader
  feedback.

## Source-Backed Challenge Contract

The challenge track uses external sources as design constraints, not as
decoration.

| Source | What the source supports | Local rule in this chapter | Repository evidence |
| --- | --- | --- | --- |
| [Rustlings Usage](https://rustlings.rust-lang.org/usage/) | Short exercises often ask learners to fix compile errors or pass tests. | A Typed AI Rustlings exercise should make one type boundary fail visibly before the fix. | `challenges/typed-ai-rustlings/exercises/token_id_not_usize.rs` |
| [Rustlings Community Exercises](https://rustlings.rust-lang.org/community-exercises/) | Focused exercise packs can target one topic. | This project targets AI boundary mistakes rather than general Rust syntax. | `challenges/typed-ai-rustlings/metadata.toml` |
| [Adam: A Method for Stochastic Optimization](https://arxiv.org/abs/1412.6980) | Adam is based on adaptive first-moment and second-moment estimates. | The Adam challenge must carry optimizer memory with the update boundary. | `src/challenges/papers/adam.rs` |
| [PyTorch Adam](https://docs.pytorch.org/docs/main/generated/torch.optim.adam.Adam_class.html) | Production Adam exposes optimizer state, moment estimates, `state_dict`, and `step()`. | The tiny Rust challenge is a teaching boundary, not a replacement for a framework optimizer. | `examples/challenge_adam.rs` and `tests/paper_to_rust_adam.rs` |
| [Rust Book: Writing Automated Tests](https://doc.rust-lang.org/stable/book/ch11-00-testing.html) | Tests can verify expected behavior and catch regressions. | Every public challenge needs an evidence command or test signal. | `cargo test --test challenge_typed_ai` and `cargo test --test paper_to_rust_adam` |

The transfer pattern is:

```text
source claim -> local challenge -> compiler, output, or test evidence
```

For the full source map, use [References](references.md).

Rustlings is the model for the first track: a small exercise should force one
syntax or type boundary to become visible before the learner moves on. The
[official Rustlings usage guide](https://rustlings.rust-lang.org/usage/)
describes a command-line loop where learners repair exercises that often fail
to compile or have tests that need to pass. The
[community-exercise guide](https://rustlings.rust-lang.org/community-exercises/)
explains how focused exercise packs can target one topic. This project keeps
that spirit but changes the domain: the failure should teach an AI boundary
such as `usize` versus `TokenId` or `Logits` versus `Distribution`.

## Typed AI Rustlings

The first seed exercises live under `challenges/typed-ai-rustlings/`.

The exercise files are intentionally broken. They are not part of the normal
Cargo build. The reference solutions are compiled by `cargo test --test
challenge_typed_ai`.

Start with:

```text
token_id_not_usize
logits_are_not_probabilities
```

The point is not to memorize types. The point is to feel the compiler reject a
bad AI boundary:

```text
usize is not TokenId
Logits are not Distribution
```

## Paper-To-Rust

The first Paper-To-Rust challenge compiles one idea from Adam:

```text
optimizer memory is part of optimizer state
```

The source paper is
[Adam: A Method for Stochastic Optimization](https://arxiv.org/abs/1412.6980).
The challenge does not claim to reproduce the whole optimizer paper. It uses
one narrow teaching claim from the paper: Adam carries adaptive estimates of
lower-order moments, so the optimizer update must move state as well as
parameters.

Run:

```bash
cargo run --example challenge_adam
cargo test --test paper_to_rust_adam
```

The typed shape is:

```text
AdamModelState -> AdamModelState
```

That shape matters because a real Adam update carries first moment, second
moment, and step count forward with the parameters.

## Worked Paper-To-Rust Ledger: Adam

Use the Adam challenge as the model for future paper-to-code exercises.

The source-backed claim is narrow:

```text
Adam uses adaptive estimates of first and second gradient moments.
Therefore the optimizer boundary must carry optimizer memory forward.
```

That claim is supported by the Adam paper's description of adaptive
lower-moment estimates and by the official PyTorch Adam documentation, which
presents first moment, second moment, step count, and optimizer state as part
of the optimizer update and state dictionary.

The challenge does not ask you to build a production optimizer. It asks you to
compile one boundary:

```text
source claim -> Rust boundary -> invariant -> test signal
```

| Source idea | Rust boundary | Invariant | Evidence |
| --- | --- | --- | --- |
| Adam keeps a first-moment estimate of gradients. | `AdamFirstMoment` inside `AdamOptimizerState` | same dimension as parameters | `adam_step_preserves_complete_optimizer_state` |
| Adam keeps a second-moment estimate of squared gradients. | `AdamSecondMoment` inside `AdamOptimizerState` | same dimension as parameters | `adam_first_step_matches_bias_corrected_update` |
| Bias correction depends on the update count. | `AdamStepCount` | step count advances with every update | `step count: 1` in `cargo run --example challenge_adam` |
| The optimizer update consumes a gradient and returns complete state. | `AdamTrainStep : AdamModelState -> AdamModelState` | gradient dimension matches model state dimension | `adam_rejects_gradient_dimension_mismatch` |

Read the tiny update in Rust as four movements:

```text
previous first moment + gradient -> next first moment
previous second moment + squared gradient -> next second moment
next moments + step count -> bias-corrected moments
parameters + corrected moments -> updated parameters
```

The category-theory shape is deliberately modest. The challenge uses
`AdamTrainStep` as an endomorphism on `AdamModelState` because the public
input and output object are the same complete state. It does not claim that
this small file proves convergence, covers every Adam variant, or replaces the
paper's analysis.

## Challenge Evidence And Textbook Feedback

Challenge completions are useful because they show whether the practice loop
works. They are not automatically accepted reader reports for the textbook.

A completion becomes useful feedback when it includes:

```text
challenge tried
command, output line, compiler error, or test name
AI boundary that became clearer
first unclear point, or none
smallest useful fix for the next reader
```

If the first unclear point belongs to a chapter, example, table, exercise, or
command, open the closest reader report from the public review guide too. The
book improves fastest from a precise blocked learning step, not from broad
approval that a challenge was interesting.

## Contribute A Challenge

For Typed AI Rustlings, contribute one small compiler-fix exercise.

For Paper-To-Rust, choose one paper and compile one idea. Do not submit a
whole-paper summary.

Use this shape:

```text
paper claim -> Rust type -> invariant -> test -> executable example
```

Before adding a challenge, name the source that owns the claim and the Rust file
that owns the executable boundary. The public solution should be small enough
that a reader can inspect it completely.
