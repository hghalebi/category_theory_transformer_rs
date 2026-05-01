use category_theory_transformer_rs::{
    CtResult, Functor, LocalGradient, Monoid, MulOp, OptionFunctor, PipelineTrace, Scalar,
    TraceStep, VecFunctor, monoid_laws_hold_for_pipeline_trace,
    naturality_square_holds_for_first_option,
};

fn main() -> CtResult<()> {
    let squared = VecFunctor::fmap(vec![1, 2, 3], |x| x * x);
    let shifted = OptionFunctor::fmap(Some(7), |x| x + 1);

    println!("Vec fmap square: {squared:?}");
    println!("Option fmap +1: {shifted:?}");
    println!(
        "naturality square holds: {}",
        naturality_square_holds_for_first_option()
    );

    let trace = PipelineTrace::from_steps(vec![TraceStep::new("embedding")])
        .combine(&PipelineTrace::from_steps(vec![TraceStep::new("linear")]))
        .combine(&PipelineTrace::from_steps(vec![TraceStep::new("softmax")]));

    println!("trace: {:?}", trace.names());
    println!(
        "monoid laws hold: {}",
        monoid_laws_hold_for_pipeline_trace()
    );

    let mul = MulOp;
    let x = Scalar::new(2.0)?;
    let y = Scalar::new(3.0)?;
    let upstream = LocalGradient::new(1.0)?;
    let (dl_dx, dl_dy) = mul.backward(x, y, upstream)?;

    println!("dL/dx: {}", dl_dx.value());
    println!("dL/dy: {}", dl_dy.value());

    Ok(())
}
