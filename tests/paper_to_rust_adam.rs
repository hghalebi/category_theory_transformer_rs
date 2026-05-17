use category_theory_transformer_rs::{
    AdamConfig, AdamDecayRate, AdamEpsilon, AdamGradientVector, AdamModelState,
    AdamParameterVector, AdamTrainStep, CtError, CtResult, LearningRate, Morphism,
};

fn standard_config() -> CtResult<AdamConfig> {
    Ok(AdamConfig::new(
        LearningRate::new(0.1)?,
        AdamDecayRate::new(0.9)?,
        AdamDecayRate::new(0.999)?,
        AdamEpsilon::new(1e-8)?,
    ))
}

#[test]
fn paper_to_rust_adam_keeps_optimizer_memory_with_parameters() -> CtResult<()> {
    let state = AdamModelState::from_parameters(AdamParameterVector::new(vec![1.0, -1.0])?);
    let step = AdamTrainStep::new(
        AdamGradientVector::new(vec![0.5, -0.25])?,
        standard_config()?,
    );
    let updated = step.apply(state)?;

    assert_eq!(updated.optimizer().step_count().value(), 1);
    assert_eq!(updated.optimizer().first_moment().as_slice().len(), 2);
    assert_eq!(updated.optimizer().second_moment().as_slice().len(), 2);
    assert!((updated.parameters().as_slice()[0] - 0.9).abs() < 1e-5);
    assert!((updated.parameters().as_slice()[1] + 0.9).abs() < 1e-5);

    Ok(())
}

#[test]
fn paper_to_rust_adam_rejects_dimension_drift() -> CtResult<()> {
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
