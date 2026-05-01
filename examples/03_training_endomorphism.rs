use category_theory_transformer_rs::{
    CtResult, DatasetWindowing, LearningRate, ModelDimension, Morphism, Parameters, StepCount,
    TokenSequence, TrainStep, VocabSize, apply_endomorphism_n_times, average_loss,
};

fn main() -> CtResult<()> {
    let tokens = TokenSequence::from_indices([1, 2, 3, 4, 1, 2, 3, 4])?;
    let dataset = DatasetWindowing.apply(tokens)?;
    let params = Parameters::init(VocabSize::new(5)?, ModelDimension::new(4)?);

    let before = average_loss(&params, &dataset)?;
    let train_step = TrainStep::new(dataset.clone(), LearningRate::new(1.0)?);
    let trained = apply_endomorphism_n_times(&train_step, params, StepCount::new(80))?;
    let after = average_loss(&trained, &dataset)?;

    println!("loss before: {:.6}", before.value());
    println!("loss after:  {:.6}", after.value());

    Ok(())
}
