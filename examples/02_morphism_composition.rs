use category_theory_transformer_rs::{
    Compose, CtResult, Embedding, LinearToLogits, Logits, ModelDimension, Morphism, Parameters,
    Softmax, TokenId, Vector, VocabSize,
};

fn main() -> CtResult<()> {
    let params = Parameters::init(VocabSize::new(5)?, ModelDimension::new(4)?);

    let token_to_logits = Compose::<_, _, Vector>::new(
        Embedding::from_parameters(&params),
        LinearToLogits::from_parameters(&params),
    );
    let token_to_distribution = Compose::<_, _, Logits>::new(token_to_logits, Softmax);

    let distribution = token_to_distribution.apply(TokenId::new(1))?;

    println!("next-token probabilities: {:?}", distribution.as_slice());

    Ok(())
}
