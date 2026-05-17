//! Reference helpers for the Typed AI Rustlings challenge.

use crate::category::Morphism;
use crate::domain::{Distribution, Logits, Loss, Product, TokenId, VocabSize};
use crate::error::{CtError, CtResult};
use crate::ml::{CrossEntropy, Softmax};

/// Exposes the integer index only after the value has crossed the `TokenId`
/// boundary.
pub fn token_index(token: TokenId) -> usize {
    token.index()
}

/// Builds a uniform distribution for a non-empty vocabulary.
pub fn uniform_distribution(vocab_size: VocabSize) -> CtResult<Distribution> {
    let probability = 1.0 / vocab_size.value() as f32;
    Distribution::new(vec![probability; vocab_size.value()])
}

/// Computes target loss from raw logits by crossing the softmax boundary first.
pub fn loss_from_logits(logits: Logits, target: TokenId) -> CtResult<Loss> {
    let distribution = Softmax.apply(logits)?;
    CrossEntropy.apply(Product::new(distribution, target))
}

/// Validates that a target token can address a probability distribution.
pub fn require_target_in_distribution(
    distribution: &Distribution,
    target: TokenId,
) -> CtResult<()> {
    if target.index() >= distribution.as_slice().len() {
        return Err(CtError::OutOfRange {
            kind: "target token",
            index: target.index(),
            limit: distribution.as_slice().len(),
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_index_requires_a_token_id_boundary() {
        assert_eq!(token_index(TokenId::new(7)), 7);
    }

    #[test]
    fn uniform_distribution_normalizes_vocab_size() -> CtResult<()> {
        let distribution = uniform_distribution(VocabSize::new(4)?)?;

        assert_eq!(distribution.as_slice(), &[0.25, 0.25, 0.25, 0.25]);
        Ok(())
    }

    #[test]
    fn loss_from_logits_crosses_softmax_before_cross_entropy() -> CtResult<()> {
        let loss = loss_from_logits(Logits::new(vec![0.0, 2.0]), TokenId::new(1))?;

        assert!(loss.value() < 0.2);
        Ok(())
    }

    #[test]
    fn target_validation_rejects_out_of_range_token() -> CtResult<()> {
        let distribution = uniform_distribution(VocabSize::new(2)?)?;

        assert!(matches!(
            require_target_in_distribution(&distribution, TokenId::new(9)),
            Err(CtError::OutOfRange {
                kind: "target token",
                index: 9,
                limit: 2,
            })
        ));

        Ok(())
    }
}
