use category_theory_transformer_rs::{
    AdamConfig, AdamDecayRate, AdamEpsilon, AdamGradientVector, AdamModelState,
    AdamParameterVector, AdamTrainStep, CtResult, LearningRate, Morphism,
};

fn main() -> CtResult<()> {
    let state = AdamModelState::from_parameters(AdamParameterVector::new(vec![1.0, -1.0])?);
    let config = AdamConfig::new(
        LearningRate::new(0.1)?,
        AdamDecayRate::new(0.9)?,
        AdamDecayRate::new(0.999)?,
        AdamEpsilon::new(1e-8)?,
    );
    let step = AdamTrainStep::new(AdamGradientVector::new(vec![0.5, -0.25])?, config);
    let updated = step.apply(state)?;

    println!("Paper-To-Rust: Adam");
    println!("paper idea: optimizer state is part of the update boundary");
    println!("typed shape: AdamModelState -> AdamModelState");
    println!(
        "step count: {}, first moment: {:?}, second moment: {:?}",
        updated.optimizer().step_count().value(),
        updated.optimizer().first_moment().as_slice(),
        updated.optimizer().second_moment().as_slice()
    );
    println!("updated parameters: {:?}", updated.parameters().as_slice());
    println!("share line: Stop summarizing Adam. Compile optimizer state.");

    Ok(())
}
