use category_theory_transformer_rs::{
    CtResult, DatasetWindowing, Morphism, TokenId, TokenSequence,
};

fn main() -> CtResult<()> {
    let tokens = TokenSequence::from_indices([1, 2, 3, 4])?;
    let dataset = DatasetWindowing.apply(tokens.clone())?;

    println!("TokenSequence:");
    println!("{}", format_token_sequence(tokens.as_slice()));
    println!();
    println!("TrainingSet:");
    for example in dataset.examples() {
        println!(
            "({} -> {})",
            format_token_id(example.first()),
            format_token_id(example.second())
        );
    }
    println!();
    println!("Typed boundaries:");
    println!("usize -> TokenId");
    println!("Vec<TokenId> -> TokenSequence");
    println!("TokenSequence -> TrainingSet");
    println!("TrainingExample = Product<TokenId, TokenId>");

    Ok(())
}

fn format_token_sequence(tokens: &[TokenId]) -> String {
    let formatted = tokens
        .iter()
        .map(format_token_id)
        .collect::<Vec<_>>()
        .join(", ");

    format!("[{formatted}]")
}

fn format_token_id(token: &TokenId) -> String {
    format!("TokenId({})", token.index())
}
