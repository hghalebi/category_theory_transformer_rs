use crate::calculus::{LocalGradient, MulOp, Scalar};
use crate::category::{Compose, StepCount, apply_endomorphism_n_times};
use crate::domain::{
    LearningRate, Logits, ModelDimension, Parameters, Product, TokenId, TokenSequence, Vector,
    VocabSize,
};
use crate::error::CtResult;
use crate::ml::{
    CrossEntropy, DatasetWindowing, Embedding, LinearToLogits, Softmax, average_loss,
    composed_prediction_matches_direct_prediction,
};
use crate::structure::{
    Functor, Monoid, OptionFunctor, PipelineTrace, TraceStep, VecFunctor,
    monoid_laws_hold_for_pipeline_trace, naturality_square_holds_for_first_option,
};
use crate::training::TrainStep;
use crate::{Identity, Morphism};

/// Run the full terminal walkthrough used by `cargo run --bin category_ml`.
pub fn run_demo() -> CtResult<()> {
    println!("Category theory concepts implemented in Rust 2024");
    println!("=================================================\n");

    let vocab = ["<pad>", "I", "love", "Rust", "."];
    let raw_text = TokenSequence::from_indices([1, 2, 3, 4, 1, 2, 3, 4])?;

    println!("1. Object examples");
    println!("   TokenId(1) means {:?}\n", vocab[1]);

    println!("2. Dataset morphism: TokenSequence -> TrainingSet");
    let dataset = DatasetWindowing.apply(raw_text)?;
    for example in dataset.examples() {
        println!(
            "   {:?} -> {:?}",
            vocab[example.first().index()],
            vocab[example.second().index()]
        );
    }
    println!();

    println!("3. Identity morphism: id_Vector : Vector -> Vector");
    let v = Vector::new(vec![1.0, 2.0, 3.0]);
    let same_v = Identity::<Vector>::new().apply(v.clone())?;
    println!("   input  = {:?}", v);
    println!("   output = {:?}\n", same_v);

    println!("4. Composition: Softmax after Linear after Embedding");
    let params = Parameters::init(VocabSize::new(vocab.len())?, ModelDimension::new(4)?);
    let embedding = Embedding::from_parameters(&params);
    let linear = LinearToLogits::from_parameters(&params);
    let token_to_logits = Compose::<_, _, Vector>::new(embedding, linear);
    let token_to_distribution = Compose::<_, _, Logits>::new(token_to_logits, Softmax);
    let distribution = token_to_distribution.apply(TokenId::new(1))?;
    println!("   P(next token | 'I') = {:?}\n", distribution.as_slice());

    println!("5. Product object: Prediction x Target -> Loss");
    let loss = CrossEntropy.apply(Product::new(distribution, TokenId::new(2)))?;
    println!("   loss for target 'love' = {:.6}\n", loss.value());

    println!("6. Endomorphism: TrainStep : Parameters -> Parameters");
    let before = average_loss(&params, &dataset)?;
    let train_step = TrainStep::new(dataset.clone(), LearningRate::new(1.0)?);
    let trained_params =
        apply_endomorphism_n_times(&train_step, params.clone(), StepCount::new(80))?;
    let after = average_loss(&trained_params, &dataset)?;
    println!("   average loss before training = {:.6}", before.value());
    println!("   average loss after  training = {:.6}\n", after.value());

    println!("7. Functor: fmap over Vec and Option");
    let xs = vec![1, 2, 3];
    let ys = VecFunctor::fmap(xs, |x| x * x);
    let maybe = OptionFunctor::fmap(Some(7), |x| x + 1);
    println!("   VecFunctor fmap square: {:?}", ys);
    println!("   OptionFunctor fmap +1: {:?}\n", maybe);

    println!("8. Natural transformation: Vec<A> -> Option<A>");
    println!(
        "   naturality square holds: {}\n",
        naturality_square_holds_for_first_option()
    );

    println!("9. Monoid: pipeline traces compose associatively with identity");
    let trace = PipelineTrace::from_steps(vec![TraceStep::new("embedding")])
        .combine(&PipelineTrace::from_steps(vec![TraceStep::new("linear")]))
        .combine(&PipelineTrace::from_steps(vec![TraceStep::new("softmax")]));
    println!("   trace = {:?}", trace.names());
    println!(
        "   monoid laws hold for this trace type: {}\n",
        monoid_laws_hold_for_pipeline_trace()
    );

    println!("10. Commutative diagram check");
    println!(
        "   composed prediction == direct prediction: {}\n",
        composed_prediction_matches_direct_prediction(&params)?
    );

    println!("11. Chain rule / local derivative morphism");
    let mul = MulOp;
    let x = Scalar::new(2.0)?;
    let y = Scalar::new(3.0)?;
    let z = mul.forward(x, y)?;
    let upstream = LocalGradient::new(1.0)?;
    let (dl_dx, dl_dy) = mul.backward(x, y, upstream)?;
    println!("   z = x * y = {}", z.value());
    println!(
        "   if dL/dz = {}, then dL/dx = {}, dL/dy = {}\n",
        upstream.value(),
        dl_dx.value(),
        dl_dy.value()
    );

    println!("Compressed categorical training view:");
    println!("   Dataset x Parameters -> Prediction -> Loss -> Gradients -> Updated Parameters");
    println!("   TrainStep is repeated as Parameters0 -> Parameters1 -> ... -> ParametersN");

    Ok(())
}
