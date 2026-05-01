use std::fmt::{self, Display};

/// Shared error type for the tutorial crate.
#[derive(Debug, Clone, PartialEq)]
pub enum CtError {
    EmptyInput(&'static str),
    OutOfRange {
        kind: &'static str,
        index: usize,
        limit: usize,
    },
    ShapeMismatch {
        op: &'static str,
        expected: String,
        got: String,
    },
    InvalidProbability(&'static str),
    InvalidLoss(f32),
    InvalidLearningRate(f32),
}

impl Display for CtError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CtError::EmptyInput(context) => write!(f, "empty input in {context}"),
            CtError::OutOfRange { kind, index, limit } => {
                write!(f, "{kind} index {index} out of range; limit is {limit}")
            }
            CtError::ShapeMismatch { op, expected, got } => {
                write!(f, "shape mismatch in {op}: expected {expected}, got {got}")
            }
            CtError::InvalidProbability(context) => {
                write!(f, "invalid probability distribution in {context}")
            }
            CtError::InvalidLoss(value) => write!(f, "invalid loss value {value}"),
            CtError::InvalidLearningRate(value) => write!(f, "invalid learning rate {value}"),
        }
    }
}

impl std::error::Error for CtError {}

pub type CtResult<T> = Result<T, CtError>;
