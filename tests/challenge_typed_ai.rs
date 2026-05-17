use category_theory_transformer_rs::{
    CtError, CtResult, Distribution, Logits, TokenId, VocabSize, loss_from_logits,
    require_target_in_distribution, token_index, uniform_distribution,
};

#[path = "../challenges/typed-ai-rustlings/solutions/token_id_not_usize.rs"]
mod token_id_not_usize_solution;

#[path = "../challenges/typed-ai-rustlings/solutions/logits_are_not_probabilities.rs"]
mod logits_are_not_probabilities_solution;

#[test]
fn token_id_solution_crosses_the_typed_boundary() {
    assert_eq!(token_id_not_usize_solution::visible_token_index(), 3);
    assert_eq!(token_index(TokenId::new(3)), 3);
}

#[test]
fn logits_solution_crosses_softmax_before_loss() -> CtResult<()> {
    let loss = logits_are_not_probabilities_solution::target_loss()?;

    assert!(loss.value() < 0.2);
    assert!(loss_from_logits(Logits::new(vec![0.0, 2.0]), TokenId::new(1))?.value() < 0.2);
    Ok(())
}

#[test]
fn typed_ai_helpers_reject_invalid_distribution_boundary() -> CtResult<()> {
    let distribution = uniform_distribution(VocabSize::new(2)?)?;

    assert!(matches!(
        require_target_in_distribution(&distribution, TokenId::new(3)),
        Err(CtError::OutOfRange {
            kind: "target token",
            index: 3,
            limit: 2,
        })
    ));
    assert!(matches!(
        Distribution::new(vec![0.8, 0.8]),
        Err(CtError::InvalidProbability("distribution constructor"))
    ));

    Ok(())
}
