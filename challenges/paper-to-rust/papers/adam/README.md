# Paper-To-Rust: Adam

Paper:

```text
Adam: A Method for Stochastic Optimization
https://arxiv.org/abs/1412.6980
```

Compiled idea:

```text
An Adam update is not just Parameters -> Parameters.
It is AdamModelState -> AdamModelState because first moment, second moment, and
step count are part of the optimizer boundary.
```

Source-to-code ledger:

| Source claim | Rust handle | What to verify |
| --- | --- | --- |
| Adam uses adaptive estimates of first and second moments. | `AdamFirstMoment`, `AdamSecondMoment` | both moment vectors move forward after a step |
| Bias correction depends on the step number. | `AdamStepCount` | `cargo run --example challenge_adam` prints `step count: 1` |
| Optimizer memory belongs with the update boundary. | `AdamModelState` | the step has shape `AdamModelState -> AdamModelState` |
| A gradient must match the parameter dimension. | `AdamGradientVector` | the mismatch test returns `CtError::ShapeMismatch` |

Run:

```bash
cargo run --example challenge_adam
cargo test --test paper_to_rust_adam
```

Source:

- `src/challenges/papers/adam.rs`
- `examples/challenge_adam.rs`
- `tests/paper_to_rust_adam.rs`

Share line:

```text
Stop summarizing Adam. Compile optimizer state.
```
