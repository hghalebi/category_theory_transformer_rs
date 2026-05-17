//! A small Paper-To-Rust translation of Adam optimizer state.
//!
//! The challenge compiles one idea from the Adam paper: an optimizer step is not
//! just a parameter vector update. It carries first-moment, second-moment, and
//! step-count state forward.

use crate::category::Morphism;
use crate::domain::LearningRate;
use crate::error::{CtError, CtResult};

/// Number of coordinates in an Adam parameter or gradient vector.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdamVectorDimension(usize);

impl AdamVectorDimension {
    pub fn new(value: usize) -> CtResult<Self> {
        if value == 0 {
            return Err(CtError::EmptyInput("Adam vector dimension"));
        }

        Ok(Self(value))
    }

    pub fn value(&self) -> usize {
        self.0
    }
}

/// Model parameters owned by the Adam challenge.
#[derive(Debug, Clone, PartialEq)]
pub struct AdamParameterVector(Vec<f32>);

impl AdamParameterVector {
    pub fn new(values: Vec<f32>) -> CtResult<Self> {
        validate_finite_non_empty("Adam parameter vector", &values)?;
        Ok(Self(values))
    }

    pub fn dimension(&self) -> AdamVectorDimension {
        AdamVectorDimension(self.0.len())
    }

    pub fn as_slice(&self) -> &[f32] {
        &self.0
    }
}

/// Gradient vector for one Adam update.
#[derive(Debug, Clone, PartialEq)]
pub struct AdamGradientVector(Vec<f32>);

impl AdamGradientVector {
    pub fn new(values: Vec<f32>) -> CtResult<Self> {
        validate_finite_non_empty("Adam gradient vector", &values)?;
        Ok(Self(values))
    }

    pub fn dimension(&self) -> AdamVectorDimension {
        AdamVectorDimension(self.0.len())
    }

    pub fn as_slice(&self) -> &[f32] {
        &self.0
    }
}

/// Exponential moving average of gradients.
#[derive(Debug, Clone, PartialEq)]
pub struct AdamFirstMoment(Vec<f32>);

impl AdamFirstMoment {
    pub fn zeros(dimension: AdamVectorDimension) -> Self {
        Self(vec![0.0; dimension.value()])
    }

    pub fn as_slice(&self) -> &[f32] {
        &self.0
    }
}

/// Exponential moving average of squared gradients.
#[derive(Debug, Clone, PartialEq)]
pub struct AdamSecondMoment(Vec<f32>);

impl AdamSecondMoment {
    pub fn zeros(dimension: AdamVectorDimension) -> Self {
        Self(vec![0.0; dimension.value()])
    }

    pub fn as_slice(&self) -> &[f32] {
        &self.0
    }
}

/// A decay coefficient in the half-open range `[0, 1)`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AdamDecayRate(f32);

impl AdamDecayRate {
    pub fn new(value: f32) -> CtResult<Self> {
        if !value.is_finite() || !(0.0..1.0).contains(&value) {
            return Err(CtError::InvalidScalar {
                kind: "Adam decay rate",
                value,
            });
        }

        Ok(Self(value))
    }

    pub fn value(&self) -> f32 {
        self.0
    }
}

/// Small positive stabilizer in the Adam denominator.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AdamEpsilon(f32);

impl AdamEpsilon {
    pub fn new(value: f32) -> CtResult<Self> {
        if !value.is_finite() || value <= 0.0 {
            return Err(CtError::InvalidScalar {
                kind: "Adam epsilon",
                value,
            });
        }

        Ok(Self(value))
    }

    pub fn value(&self) -> f32 {
        self.0
    }
}

/// Count of Adam updates already applied.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdamStepCount(usize);

impl AdamStepCount {
    pub fn zero() -> Self {
        Self(0)
    }

    pub fn value(&self) -> usize {
        self.0
    }

    fn next(self) -> Self {
        Self(self.0 + 1)
    }
}

/// Adam hyperparameters that affect one optimizer transition.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AdamConfig {
    learning_rate: LearningRate,
    beta1: AdamDecayRate,
    beta2: AdamDecayRate,
    epsilon: AdamEpsilon,
}

impl AdamConfig {
    pub fn new(
        learning_rate: LearningRate,
        beta1: AdamDecayRate,
        beta2: AdamDecayRate,
        epsilon: AdamEpsilon,
    ) -> Self {
        Self {
            learning_rate,
            beta1,
            beta2,
            epsilon,
        }
    }

    pub fn learning_rate(&self) -> LearningRate {
        self.learning_rate
    }

    pub fn beta1(&self) -> AdamDecayRate {
        self.beta1
    }

    pub fn beta2(&self) -> AdamDecayRate {
        self.beta2
    }

    pub fn epsilon(&self) -> AdamEpsilon {
        self.epsilon
    }
}

/// Adam optimizer memory that must move with the parameters.
#[derive(Debug, Clone, PartialEq)]
pub struct AdamOptimizerState {
    first_moment: AdamFirstMoment,
    second_moment: AdamSecondMoment,
    step_count: AdamStepCount,
}

impl AdamOptimizerState {
    pub fn zeros(dimension: AdamVectorDimension) -> Self {
        Self {
            first_moment: AdamFirstMoment::zeros(dimension),
            second_moment: AdamSecondMoment::zeros(dimension),
            step_count: AdamStepCount::zero(),
        }
    }

    pub fn first_moment(&self) -> &AdamFirstMoment {
        &self.first_moment
    }

    pub fn second_moment(&self) -> &AdamSecondMoment {
        &self.second_moment
    }

    pub fn step_count(&self) -> AdamStepCount {
        self.step_count
    }
}

/// Complete Adam state at an optimizer boundary.
#[derive(Debug, Clone, PartialEq)]
pub struct AdamModelState {
    parameters: AdamParameterVector,
    optimizer: AdamOptimizerState,
}

impl AdamModelState {
    pub fn new(parameters: AdamParameterVector, optimizer: AdamOptimizerState) -> CtResult<Self> {
        validate_matching_dimension(
            "Adam model state",
            parameters.dimension(),
            optimizer.first_moment.as_slice().len(),
        )?;
        validate_matching_dimension(
            "Adam model state",
            parameters.dimension(),
            optimizer.second_moment.as_slice().len(),
        )?;

        Ok(Self {
            parameters,
            optimizer,
        })
    }

    pub fn from_parameters(parameters: AdamParameterVector) -> Self {
        let optimizer = AdamOptimizerState::zeros(parameters.dimension());
        Self {
            parameters,
            optimizer,
        }
    }

    pub fn parameters(&self) -> &AdamParameterVector {
        &self.parameters
    }

    pub fn optimizer(&self) -> &AdamOptimizerState {
        &self.optimizer
    }
}

/// One Adam optimizer step as an endomorphism on `AdamModelState`.
#[derive(Debug, Clone, PartialEq)]
pub struct AdamTrainStep {
    gradient: AdamGradientVector,
    config: AdamConfig,
}

impl AdamTrainStep {
    pub fn new(gradient: AdamGradientVector, config: AdamConfig) -> Self {
        Self { gradient, config }
    }
}

impl Morphism<AdamModelState, AdamModelState> for AdamTrainStep {
    fn name(&self) -> &'static str {
        "adam_train_step"
    }

    fn apply(&self, state: AdamModelState) -> CtResult<AdamModelState> {
        let dimension = state.parameters.dimension();
        validate_matching_dimension("Adam gradient", dimension, self.gradient.as_slice().len())?;
        validate_matching_dimension(
            "Adam first moment",
            dimension,
            state.optimizer.first_moment.as_slice().len(),
        )?;
        validate_matching_dimension(
            "Adam second moment",
            dimension,
            state.optimizer.second_moment.as_slice().len(),
        )?;

        let next_step = state.optimizer.step_count.next();
        let beta1 = self.config.beta1.value();
        let beta2 = self.config.beta2.value();
        let learning_rate = self.config.learning_rate.value();
        let epsilon = self.config.epsilon.value();
        let bias_correction_1 = 1.0 - beta1.powf(next_step.value() as f32);
        let bias_correction_2 = 1.0 - beta2.powf(next_step.value() as f32);

        let mut next_parameters = Vec::with_capacity(dimension.value());
        let mut next_first_moment = Vec::with_capacity(dimension.value());
        let mut next_second_moment = Vec::with_capacity(dimension.value());

        for (((parameter, gradient), first_moment), second_moment) in state
            .parameters
            .as_slice()
            .iter()
            .copied()
            .zip(self.gradient.as_slice().iter().copied())
            .zip(state.optimizer.first_moment.as_slice().iter().copied())
            .zip(state.optimizer.second_moment.as_slice().iter().copied())
        {
            let updated_first_moment = beta1 * first_moment + (1.0 - beta1) * gradient;
            let updated_second_moment = beta2 * second_moment + (1.0 - beta2) * gradient * gradient;
            let corrected_first_moment = updated_first_moment / bias_correction_1;
            let corrected_second_moment = updated_second_moment / bias_correction_2;
            let updated_parameter = parameter
                - learning_rate * corrected_first_moment
                    / (corrected_second_moment.sqrt() + epsilon);

            next_parameters.push(updated_parameter);
            next_first_moment.push(updated_first_moment);
            next_second_moment.push(updated_second_moment);
        }

        AdamModelState::new(
            AdamParameterVector::new(next_parameters)?,
            AdamOptimizerState {
                first_moment: AdamFirstMoment(next_first_moment),
                second_moment: AdamSecondMoment(next_second_moment),
                step_count: next_step,
            },
        )
    }
}

fn validate_finite_non_empty(kind: &'static str, values: &[f32]) -> CtResult<()> {
    if values.is_empty() {
        return Err(CtError::EmptyInput(kind));
    }

    if let Some(value) = values.iter().copied().find(|value| !value.is_finite()) {
        return Err(CtError::InvalidScalar { kind, value });
    }

    Ok(())
}

fn validate_matching_dimension(
    op: &'static str,
    expected: AdamVectorDimension,
    got: usize,
) -> CtResult<()> {
    if expected.value() != got {
        return Err(CtError::ShapeMismatch {
            op,
            expected: format!("dimension {}", expected.value()),
            got: format!("dimension {got}"),
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn standard_config() -> CtResult<AdamConfig> {
        Ok(AdamConfig::new(
            LearningRate::new(0.1)?,
            AdamDecayRate::new(0.9)?,
            AdamDecayRate::new(0.999)?,
            AdamEpsilon::new(1e-8)?,
        ))
    }

    #[test]
    fn adam_first_step_matches_bias_corrected_update() -> CtResult<()> {
        let state = AdamModelState::from_parameters(AdamParameterVector::new(vec![1.0, -1.0])?);
        let step = AdamTrainStep::new(
            AdamGradientVector::new(vec![0.5, -0.25])?,
            standard_config()?,
        );
        let updated = step.apply(state)?;

        assert_eq!(updated.optimizer().step_count().value(), 1);
        assert!((updated.parameters().as_slice()[0] - 0.9).abs() < 1e-5);
        assert!((updated.parameters().as_slice()[1] + 0.9).abs() < 1e-5);
        assert!((updated.optimizer().first_moment().as_slice()[0] - 0.05).abs() < 1e-6);
        assert!((updated.optimizer().second_moment().as_slice()[0] - 0.00025).abs() < 1e-7);

        Ok(())
    }

    #[test]
    fn adam_rejects_bad_decay_rate() {
        assert!(matches!(
            AdamDecayRate::new(1.0),
            Err(CtError::InvalidScalar {
                kind: "Adam decay rate",
                value: 1.0,
            })
        ));
    }

    #[test]
    fn adam_rejects_gradient_dimension_mismatch() -> CtResult<()> {
        let state = AdamModelState::from_parameters(AdamParameterVector::new(vec![1.0, -1.0])?);
        let step = AdamTrainStep::new(AdamGradientVector::new(vec![0.5])?, standard_config()?);

        assert!(matches!(
            step.apply(state),
            Err(CtError::ShapeMismatch {
                op: "Adam gradient",
                ..
            })
        ));

        Ok(())
    }

    #[test]
    fn adam_step_preserves_complete_optimizer_state() -> CtResult<()> {
        let state = AdamModelState::from_parameters(AdamParameterVector::new(vec![1.0])?);
        let step = AdamTrainStep::new(AdamGradientVector::new(vec![0.5])?, standard_config()?);
        let once = step.apply(state)?;
        let twice = step.apply(once)?;

        assert_eq!(twice.optimizer().step_count().value(), 2);
        assert_eq!(twice.parameters().dimension().value(), 1);
        assert_eq!(twice.optimizer().first_moment().as_slice().len(), 1);
        assert_eq!(twice.optimizer().second_moment().as_slice().len(), 1);

        Ok(())
    }
}
