use category_theory_transformer_rs::{
    CrossEntropy, CtResult, Logits, Morphism, Product, Softmax, TokenId,
};

pub fn target_loss() -> CtResult<category_theory_transformer_rs::Loss> {
    let logits = Logits::new(vec![0.0, 2.0]);
    let target = TokenId::new(1);
    let distribution = Softmax.apply(logits)?;

    CrossEntropy.apply(Product::new(distribution, target))
}
