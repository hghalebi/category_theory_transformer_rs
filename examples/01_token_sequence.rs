use category_theory_transformer_rs::{CtResult, Product, TokenId, TokenSequence, TrainingExample};

fn main() -> CtResult<()> {
    let raw_input = "rust makes ai structure visible";
    let token_ids = tokenize_visible_structure(raw_input);
    let sequence = TokenSequence::new(token_ids)?;
    let training_pairs = training_pairs(sequence.as_slice())?;

    println!("Raw input:");
    println!("\"{raw_input}\"");
    println!();
    println!("TokenSequence:");
    println!("{}", format_token_sequence(sequence.as_slice()));
    println!();
    println!("TrainingPairs:");
    for pair in &training_pairs {
        println!("{}", format_training_pair(pair));
    }
    println!();
    println!("Typed transformation:");
    println!("Text -> TokenSequence -> TrainingPairs");
    println!();
    println!("No framework magic.");
    println!("Just explicit structure.");

    Ok(())
}

fn tokenize_visible_structure(input: &str) -> Vec<TokenId> {
    input
        .split_whitespace()
        .map(|word| match word {
            "rust" => TokenId::new(12),
            "makes" => TokenId::new(44),
            "ai" => TokenId::new(7),
            "structure" => TokenId::new(19),
            "visible" => TokenId::new(91),
            _ => TokenId::new(0),
        })
        .collect()
}

fn training_pairs(tokens: &[TokenId]) -> CtResult<Vec<TrainingExample>> {
    TokenSequence::new(tokens.iter().copied())?;

    Ok(tokens
        .windows(2)
        .map(|pair| Product::new(pair[0], pair[1]))
        .collect())
}

fn format_token_sequence(tokens: &[TokenId]) -> String {
    let formatted = tokens
        .iter()
        .map(format_token_id)
        .collect::<Vec<_>>()
        .join(", ");

    format!("[{formatted}]")
}

fn format_training_pair(pair: &TrainingExample) -> String {
    format!(
        "({} -> {})",
        format_token_id(pair.first()),
        format_token_id(pair.second())
    )
}

fn format_token_id(token: &TokenId) -> String {
    format!("TokenId({})", token.index())
}
