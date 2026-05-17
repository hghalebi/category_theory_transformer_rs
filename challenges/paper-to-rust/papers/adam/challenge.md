# Adam Challenge

## Paper Claim

Adam keeps exponential moving averages of gradients and squared gradients, then
uses bias correction during each update.

## Rust Boundary

The challenge represents that as:

```text
AdamModelState -> AdamModelState
```

The state owns:

- `AdamParameterVector`
- `AdamFirstMoment`
- `AdamSecondMoment`
- `AdamStepCount`

## Invariant

Parameters, gradient, first moment, and second moment must all have the same
dimension before an update can run.

## Translation Ledger

Use this as the expected shape for future Paper-To-Rust challenges:

```text
source claim -> Rust boundary -> test signal
```

| Adam idea | Rust syntax | Test or output signal |
| --- | --- | --- |
| first-moment memory | `AdamFirstMoment` | `first moment: [...]` in the example output |
| second-moment memory | `AdamSecondMoment` | `second moment: [...]` in the example output |
| bias-correction time index | `AdamStepCount` | `step count: 1` |
| complete optimizer update | `AdamTrainStep : AdamModelState -> AdamModelState` | `adam_step_preserves_complete_optimizer_state` |
| shape-safety boundary | `AdamGradientVector::new(...)` plus `AdamTrainStep::apply(...)` | `CtError::ShapeMismatch` on wrong gradient length |

The challenge is intentionally smaller than the paper. It compiles the state
boundary and one bias-corrected update, but it does not claim convergence,
AMSGrad support, weight decay support, fused kernels, or production optimizer
ergonomics.

## Passing Signal

Run:

```bash
cargo test --test paper_to_rust_adam
```

The dimension-drift test must fail with `CtError::ShapeMismatch` if a gradient
has the wrong length.
