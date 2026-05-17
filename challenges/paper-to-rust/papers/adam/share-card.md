# Share Card

Paper: Adam: A Method for Stochastic Optimization

Compiled idea: optimizer memory is part of the training state.

Rust lesson: `AdamTrainStep` is an endomorphism on `AdamModelState`.

Typed shape: `AdamModelState -> AdamModelState`

Invariant: parameter, gradient, first moment, and second moment dimensions must
match.

Command: `cargo run --example challenge_adam`

Share line: Stop summarizing Adam. Compile optimizer state.
