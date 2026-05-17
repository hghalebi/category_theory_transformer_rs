use category_theory_transformer_rs::{
    CrossEntropy, CtResult, Logits, Morphism, Product, TokenId,
};

fn main() -> CtResult<()> {
    let logits = Logits::new(vec![0.0, 2.0]);
    let target = TokenId::new(1);
    let loss = CrossEntropy.apply(Product::new(logits, target))?;

    println!("target loss: {:.4}", loss.value());
    Ok(())
}
