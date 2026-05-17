//! Public challenge implementations that back the learner-facing challenge files.
//!
//! The intentionally broken exercise templates live under `challenges/` and are
//! not part of the normal Cargo build. This module contains the validated
//! reference behavior that keeps the public challenge honest.

pub mod papers;
pub mod typed_ai;
