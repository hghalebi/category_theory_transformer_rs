use category_theory_transformer_rs::{CtResult, DatasetWindowing, Morphism, TokenSequence};

fn main() -> CtResult<()> {
    let tokens = TokenSequence::from_indices([1, 2, 3, 4])?;
    let dataset = DatasetWindowing.apply(tokens)?;

    println!("training pairs:");
    for example in dataset.examples() {
        println!(
            "{} -> {}",
            example.first().index(),
            example.second().index()
        );
    }

    Ok(())
}
