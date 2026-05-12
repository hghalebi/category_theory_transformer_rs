use category_theory_transformer_rs::{
    Compose, CtResult, Distribution, Embedding, LinearToLogits, Logits, ModelDimension, Morphism,
    Parameters, Softmax, TokenId, Vector, VocabSize,
};

fn main() -> CtResult<()> {
    let params = Parameters::init(VocabSize::new(5)?, ModelDimension::new(4)?);
    let token = TokenId::new(1);
    let embedding = Embedding::from_parameters(&params);
    let linear = LinearToLogits::from_parameters(&params);

    let token_to_logits = Compose::<_, _, Vector>::new(embedding.clone(), linear.clone());
    let token_to_distribution = Compose::<_, _, Logits>::new(token_to_logits, Softmax);

    let vector = embedding.apply(token)?;
    let logits = linear.apply(vector.clone())?;
    let distribution = Softmax.apply(logits.clone())?;
    let composed_distribution = token_to_distribution.apply(token)?;

    println!("Input object:");
    println!("TokenId({})", token.index());
    println!();
    println!("Stage outputs:");
    println!("Embedding : TokenId -> Vector");
    println!("{}", format_vector(&vector));
    println!("LinearToLogits : Vector -> Logits");
    println!("{}", format_logits(&logits));
    println!("Softmax : Logits -> Distribution");
    println!("{}", format_distribution(&distribution));
    println!();
    println!("Composed morphism:");
    println!("TokenId -> Distribution");
    println!(
        "next-token probabilities: {}",
        format_values(composed_distribution.as_slice())
    );
    println!();
    println!("Middle objects kept visible:");
    println!("Vector");
    println!("Logits");

    Ok(())
}

fn format_vector(vector: &Vector) -> String {
    format!(
        "Vector(dim={}, values={})",
        vector.as_slice().len(),
        format_values(vector.as_slice())
    )
}

fn format_logits(logits: &Logits) -> String {
    format!(
        "Logits(vocab={}, values={})",
        logits.as_slice().len(),
        format_values(logits.as_slice())
    )
}

fn format_distribution(distribution: &Distribution) -> String {
    format!(
        "Distribution(vocab={}, sum={:.6}, values={})",
        distribution.as_slice().len(),
        distribution.as_slice().iter().sum::<f32>(),
        format_values(distribution.as_slice())
    )
}

fn format_values(values: &[f32]) -> String {
    let formatted = values
        .iter()
        .map(|value| format!("{value:.6}"))
        .collect::<Vec<_>>()
        .join(", ");

    format!("[{formatted}]")
}
