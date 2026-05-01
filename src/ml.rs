use crate::category::{Compose, Morphism};
use crate::domain::{
    Distribution, Logits, Loss, Parameters, Product, TokenId, TokenSequence, TrainingSet, Vector,
    approx_eq,
};
use crate::error::{CtError, CtResult};

/// Turns adjacent tokens into next-token training examples.
#[derive(Debug, Clone)]
pub struct DatasetWindowing;

impl Morphism<TokenSequence, TrainingSet> for DatasetWindowing {
    fn name(&self) -> &'static str {
        "dataset_windowing"
    }

    fn apply(&self, tokens: TokenSequence) -> CtResult<TrainingSet> {
        if tokens.as_slice().len() < 2 {
            return Err(CtError::EmptyInput(
                "dataset windowing requires at least 2 tokens",
            ));
        }

        TrainingSet::new(
            tokens
                .as_slice()
                .windows(2)
                .map(|pair| Product::new(pair[0], pair[1])),
        )
    }
}

/// Morphism from token id to embedding vector.
#[derive(Debug, Clone)]
pub struct Embedding {
    table: Vec<Vec<f32>>,
}

impl Embedding {
    pub fn from_parameters(params: &Parameters) -> Self {
        Self {
            table: params.embedding_table().to_vec(),
        }
    }
}

impl Morphism<TokenId, Vector> for Embedding {
    fn name(&self) -> &'static str {
        "embedding"
    }

    fn apply(&self, token: TokenId) -> CtResult<Vector> {
        let Some(row) = self.table.get(token.index()) else {
            return Err(CtError::OutOfRange {
                kind: "token",
                index: token.index(),
                limit: self.table.len(),
            });
        };

        Ok(Vector::new(row.clone()))
    }
}

/// Linear projection from hidden vector to vocabulary logits.
#[derive(Debug, Clone)]
pub struct LinearToLogits {
    weight: Vec<Vec<f32>>,
    bias: Vec<f32>,
}

impl LinearToLogits {
    pub fn from_parameters(params: &Parameters) -> Self {
        Self {
            weight: params.lm_head().to_vec(),
            bias: params.bias().to_vec(),
        }
    }

    pub(crate) fn from_parts(weight: Vec<Vec<f32>>, bias: Vec<f32>) -> Self {
        Self { weight, bias }
    }
}

impl Morphism<Vector, Logits> for LinearToLogits {
    fn name(&self) -> &'static str {
        "linear_to_logits"
    }

    fn apply(&self, input: Vector) -> CtResult<Logits> {
        let d_model = input.as_slice().len();
        let vocab_size = self.bias.len();

        if self.weight.len() != d_model {
            return Err(CtError::ShapeMismatch {
                op: "linear layer",
                expected: format!("weight rows == input dim {d_model}"),
                got: format!("weight rows {}", self.weight.len()),
            });
        }

        let mut out = self.bias.clone();

        for (feature, input_value) in input.as_slice().iter().enumerate() {
            if self.weight[feature].len() != vocab_size {
                return Err(CtError::ShapeMismatch {
                    op: "linear layer",
                    expected: format!("weight cols == vocab size {vocab_size}"),
                    got: format!("weight cols {}", self.weight[feature].len()),
                });
            }

            for (vocab_id, output_value) in out.iter_mut().enumerate() {
                *output_value += input_value * self.weight[feature][vocab_id];
            }
        }

        Ok(Logits::new(out))
    }
}

/// Converts logits to a probability distribution.
#[derive(Debug, Clone)]
pub struct Softmax;

impl Morphism<Logits, Distribution> for Softmax {
    fn name(&self) -> &'static str {
        "softmax"
    }

    fn apply(&self, logits: Logits) -> CtResult<Distribution> {
        if logits.as_slice().is_empty() {
            return Err(CtError::EmptyInput("softmax"));
        }

        let max_value = logits
            .as_slice()
            .iter()
            .copied()
            .fold(f32::NEG_INFINITY, f32::max);
        let mut exps = Vec::with_capacity(logits.as_slice().len());
        let mut sum = 0.0;

        for value in logits.as_slice() {
            let exp = (*value - max_value).exp();
            exps.push(exp);
            sum += exp;
        }

        if sum <= 0.0 || !sum.is_finite() {
            return Err(CtError::InvalidProbability("softmax"));
        }

        Distribution::new(exps.into_iter().map(|value| value / sum).collect())
    }
}

/// Negative log likelihood for `(distribution, target_token)`.
#[derive(Debug, Clone)]
pub struct CrossEntropy;

impl Morphism<Product<Distribution, TokenId>, Loss> for CrossEntropy {
    fn name(&self) -> &'static str {
        "cross_entropy"
    }

    fn apply(&self, input: Product<Distribution, TokenId>) -> CtResult<Loss> {
        let (distribution, target) = input.into_parts();

        let Some(probability) = distribution.as_slice().get(target.index()).copied() else {
            return Err(CtError::OutOfRange {
                kind: "target",
                index: target.index(),
                limit: distribution.as_slice().len(),
            });
        };

        Loss::new(-probability.max(1e-9).ln())
    }
}

/// Direct path used for a commutative-diagram check.
#[derive(Debug, Clone)]
pub struct DirectPredict {
    params: Parameters,
}

impl DirectPredict {
    pub fn new(params: Parameters) -> Self {
        Self { params }
    }
}

impl Morphism<TokenId, Distribution> for DirectPredict {
    fn name(&self) -> &'static str {
        "direct_predict"
    }

    fn apply(&self, token: TokenId) -> CtResult<Distribution> {
        let embedding = Embedding::from_parameters(&self.params).apply(token)?;
        let logits = LinearToLogits::from_parameters(&self.params).apply(embedding)?;
        Softmax.apply(logits)
    }
}

/// Average cross-entropy over a training set.
pub fn average_loss(params: &Parameters, dataset: &TrainingSet) -> CtResult<Loss> {
    let embedding = Embedding::from_parameters(params);
    let linear = LinearToLogits::from_parameters(params);
    let predict = Compose::<_, _, Vector>::new(embedding, linear);
    let predict = Compose::<_, _, Logits>::new(predict, Softmax);
    let loss_fn = CrossEntropy;

    let mut total = 0.0;

    for example in dataset.examples() {
        let distribution = predict.apply(*example.first())?;
        let loss = loss_fn.apply(Product::new(distribution, *example.second()))?;
        total += loss.value();
    }

    Loss::new(total / dataset.len() as f32)
}

/// Verifies that the composed path and direct path produce the same result.
pub fn composed_prediction_matches_direct_prediction(params: &Parameters) -> CtResult<bool> {
    let token = TokenId::new(1);

    let composed = Compose::<_, _, Vector>::new(
        Embedding::from_parameters(params),
        LinearToLogits::from_parameters(params),
    );
    let composed = Compose::<_, _, Logits>::new(composed, Softmax);
    let direct = DirectPredict::new(params.clone());

    let left_path = composed.apply(token)?;
    let right_path = direct.apply(token)?;

    Ok(left_path
        .as_slice()
        .iter()
        .zip(right_path.as_slice().iter())
        .all(|(a, b)| approx_eq(*a, *b, 1e-6)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{ModelDimension, VocabSize};

    #[test]
    fn dataset_windowing_builds_adjacent_pairs() -> CtResult<()> {
        let tokens = TokenSequence::from_indices([1, 2, 3])?;
        let dataset = DatasetWindowing.apply(tokens)?;

        assert_eq!(dataset.len(), 2);
        assert_eq!(dataset.examples()[0].first().index(), 1);
        assert_eq!(dataset.examples()[0].second().index(), 2);
        Ok(())
    }

    #[test]
    fn composed_and_direct_prediction_match() -> CtResult<()> {
        let params = Parameters::init(VocabSize::new(5)?, ModelDimension::new(4)?);

        assert!(composed_prediction_matches_direct_prediction(&params)?);
        Ok(())
    }
}
