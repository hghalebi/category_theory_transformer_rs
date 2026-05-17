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

## Where This Leaves Us

The challenge track turns the book's recurring method into public practice:

```text
name the boundary
make the failure visible
repair the smallest Rust shape
record the evidence
```

The next reference tool is the [Glossary](glossary.md). Use it when a challenge
exposes a term you can run but cannot yet explain. For example:

```text
TokenId
Distribution
endomorphism
optimizer state
evidence signal
```

If the term is still unclear after the glossary, report the exact command,
output line, compiler error, or table row that exposed the confusion.

## Further Reading

Use these sources only after you have run at least one challenge command:

| Source | Use it to clarify | Bring it back to this evidence |
| --- | --- | --- |
| [Rustlings Usage](https://rustlings.rust-lang.org/usage/) | why a small exercise can be built around a compiler error or failing test | one Typed AI Rustlings exercise file |
| [Rustlings Community Exercises](https://rustlings.rust-lang.org/community-exercises/) | how a focused exercise pack can target one topic | `challenges/typed-ai-rustlings/metadata.toml` |
| [Adam](https://arxiv.org/abs/1412.6980) | why Adam carries first-moment and second-moment estimates | `AdamFirstMoment`, `AdamSecondMoment`, and `AdamStepCount` |
| [PyTorch Adam](https://docs.pytorch.org/docs/main/generated/torch.optim.adam.Adam_class.html) | how a production optimizer exposes state and `step()` | `AdamModelState -> AdamModelState` |
| [Rust Book: Writing Automated Tests](https://doc.rust-lang.org/stable/book/ch11-00-testing.html) | why tests are part of the learning artifact | `cargo test --test challenge_typed_ai` and `cargo test --test paper_to_rust_adam` |

The safe reading rule is:

```text
read one source -> improve one challenge boundary -> run one command
```

Do not use a source link as proof that the challenge is correct. Use the source
to refine the local Rust claim, then use a command or test as local evidence.

## Practice After This Chapter

Run one command from each track:

```bash
cargo test --test challenge_typed_ai
cargo run --example challenge_adam
```

Then fill this challenge evidence card:

```text
challenge tried:
command:
visible evidence signal:
AI boundary that became clearer:
first unclear point, or none:
smallest useful fix for the next reader:
```

For a Typed AI Rustlings challenge, the evidence signal should be a compiler
error, type mismatch, test name, or solution test.

For a Paper-To-Rust challenge, the evidence signal should be a source claim,
Rust type, invariant, and passing test or output line.

## Retrieval Practice

### Recall

Name the two challenge tracks without looking back.

Name one boundary that Typed AI Rustlings should make visible.

Name the complete Adam challenge shape.

### Explain

Explain why `Logits` should not be accepted where a `Distribution` is required.

Explain why Adam's first moment, second moment, and step count belong to the
optimizer state instead of being loose helper values.

Explain why challenge completion evidence is not automatically accepted
textbook reader evidence.

### Apply

Choose one paper, tutorial, or framework documentation page and write only this
much:

```text
source claim:
Rust boundary:
invariant:
test or output evidence:
larger claim not implemented:
```

The answer is strong only if the Rust boundary is small enough for another
reader to inspect completely.

### Debug

For each weak challenge design, name the missing piece:

```text
1. The challenge links to a paper but names no Rust type.
2. The exercise fails, but the failure does not teach an AI boundary.
3. The Adam challenge updates parameters but drops moment state.
4. The completion report says "I liked it" but gives no output line.
5. The challenge claims to implement a whole paper from one small test.
```

A useful answer should say whether the problem is a missing source claim,
missing Rust boundary, missing invariant, missing evidence signal, or
overclaim.
