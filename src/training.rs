use crate::category::Morphism;
use crate::domain::{LearningRate, Parameters, TrainingSet, Vector};
use crate::error::{CtError, CtResult};
use crate::ml::{LinearToLogits, Softmax};

/// One full-batch optimizer update.
///
/// Categorically, this is an endomorphism:
///
/// `Parameters -> Parameters`
#[derive(Debug, Clone)]
pub struct TrainStep {
    dataset: TrainingSet,
    learning_rate: LearningRate,
}

impl TrainStep {
    pub fn new(dataset: TrainingSet, learning_rate: LearningRate) -> Self {
        Self {
            dataset,
            learning_rate,
        }
    }
}

impl Morphism<Parameters, Parameters> for TrainStep {
    fn name(&self) -> &'static str {
        "train_step_endomorphism"
    }

    fn apply(&self, params: Parameters) -> CtResult<Parameters> {
        let vocab_size = params.vocab_size();
        let d_model = params.d_model();

        if vocab_size == 0 || d_model == 0 {
            return Err(CtError::EmptyInput("parameters"));
        }

        let mut grad_embedding = vec![vec![0.0; d_model]; params.embedding.len()];
        let mut grad_lm_head = vec![vec![0.0; vocab_size]; d_model];
        let mut grad_bias = vec![0.0; vocab_size];

        for example in self.dataset.examples() {
            let input_id = example.first().index();
            let target_id = example.second().index();

            if input_id >= params.embedding.len() {
                return Err(CtError::OutOfRange {
                    kind: "input token",
                    index: input_id,
                    limit: params.embedding.len(),
                });
            }

            if target_id >= vocab_size {
                return Err(CtError::OutOfRange {
                    kind: "target token",
                    index: target_id,
                    limit: vocab_size,
                });
            }

            let x = &params.embedding[input_id];
            let logits = LinearToLogits::from_parts(params.lm_head.clone(), params.bias.clone())
                .apply(Vector::new(x.clone()))?;
            let probs = Softmax.apply(logits)?;

            let mut dlogits = probs.as_slice().to_vec();
            dlogits[target_id] -= 1.0;

            for (vocab_id, dlogit) in dlogits.iter().copied().enumerate() {
                grad_bias[vocab_id] += dlogit;

                for (feature, x_feature) in x.iter().copied().enumerate() {
                    grad_lm_head[feature][vocab_id] += x_feature * dlogit;
                }
            }

            for (feature, grad_feature) in grad_embedding[input_id].iter_mut().enumerate() {
                let dx = params.lm_head[feature]
                    .iter()
                    .zip(dlogits.iter())
                    .map(|(weight, dlogit)| weight * dlogit)
                    .sum::<f32>();

                *grad_feature += dx;
            }
        }

        let batch_scale = 1.0 / self.dataset.len() as f32;
        let learning_rate = self.learning_rate.value();
        let mut updated = params.clone();

        for (row, grad_row) in updated.embedding.iter_mut().zip(grad_embedding.iter()) {
            for (value, grad) in row.iter_mut().zip(grad_row.iter()) {
                *value -= learning_rate * grad * batch_scale;
            }
        }

        for (row, grad_row) in updated.lm_head.iter_mut().zip(grad_lm_head.iter()) {
            for (value, grad) in row.iter_mut().zip(grad_row.iter()) {
                *value -= learning_rate * grad * batch_scale;
            }
        }

        for (bias, grad) in updated.bias.iter_mut().zip(grad_bias.iter()) {
            *bias -= learning_rate * grad * batch_scale;
        }

        Ok(updated)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::category::{StepCount, apply_endomorphism_n_times};
    use crate::domain::{ModelDimension, TokenSequence, VocabSize};
    use crate::ml::{DatasetWindowing, average_loss};

    #[test]
    fn repeated_training_step_reduces_loss() -> CtResult<()> {
        let tokens = TokenSequence::from_indices([1, 2, 3, 4, 1, 2, 3, 4])?;
        let dataset = DatasetWindowing.apply(tokens)?;
        let params = Parameters::init(VocabSize::new(5)?, ModelDimension::new(4)?);
        let before = average_loss(&params, &dataset)?;
        let train_step = TrainStep::new(dataset.clone(), LearningRate::new(1.0)?);
        let trained = apply_endomorphism_n_times(&train_step, params, StepCount::new(80))?;
        let after = average_loss(&trained, &dataset)?;

        assert!(after.value() < before.value());
        Ok(())
    }
}
