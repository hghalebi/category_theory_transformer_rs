use crate::error::{CtError, CtResult};

/// A vocabulary index. It is intentionally not a raw `usize` in public APIs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TokenId(usize);

impl TokenId {
    pub fn new(index: usize) -> Self {
        Self(index)
    }

    pub fn index(&self) -> usize {
        self.0
    }
}

impl From<usize> for TokenId {
    fn from(value: usize) -> Self {
        Self::new(value)
    }
}

/// A sequence of tokens before it has been converted into training pairs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenSequence(Vec<TokenId>);

impl TokenSequence {
    pub fn new(tokens: impl IntoIterator<Item = TokenId>) -> CtResult<Self> {
        let tokens = tokens.into_iter().collect::<Vec<_>>();

        if tokens.is_empty() {
            return Err(CtError::EmptyInput("token sequence"));
        }

        Ok(Self(tokens))
    }

    pub fn from_indices(indices: impl IntoIterator<Item = usize>) -> CtResult<Self> {
        Self::new(indices.into_iter().map(TokenId::new))
    }

    pub fn as_slice(&self) -> &[TokenId] {
        &self.0
    }
}

/// A dense feature vector.
#[derive(Debug, Clone, PartialEq)]
pub struct Vector(Vec<f32>);

impl Vector {
    pub fn new(values: Vec<f32>) -> Self {
        Self(values)
    }

    pub fn as_slice(&self) -> &[f32] {
        &self.0
    }
}

/// Unnormalized model scores.
#[derive(Debug, Clone, PartialEq)]
pub struct Logits(Vec<f32>);

impl Logits {
    pub fn new(values: Vec<f32>) -> Self {
        Self(values)
    }

    pub fn as_slice(&self) -> &[f32] {
        &self.0
    }
}

/// A validated probability distribution.
#[derive(Debug, Clone, PartialEq)]
pub struct Distribution(Vec<f32>);

impl Distribution {
    pub fn new(probabilities: Vec<f32>) -> CtResult<Self> {
        if probabilities.is_empty() {
            return Err(CtError::EmptyInput("distribution"));
        }

        let sum: f32 = probabilities.iter().sum();
        let all_valid = probabilities
            .iter()
            .all(|probability| probability.is_finite() && *probability >= 0.0);

        if !all_valid || !approx_eq(sum, 1.0, 1e-4) {
            return Err(CtError::InvalidProbability("distribution constructor"));
        }

        Ok(Self(probabilities))
    }

    pub fn as_slice(&self) -> &[f32] {
        &self.0
    }
}

/// A non-negative scalar objective value.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Loss(f32);

impl Loss {
    pub fn new(value: f32) -> CtResult<Self> {
        if !value.is_finite() || value < 0.0 {
            return Err(CtError::InvalidLoss(value));
        }

        Ok(Self(value))
    }

    pub fn value(&self) -> f32 {
        self.0
    }
}

/// Number of vocabulary entries.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VocabSize(usize);

impl VocabSize {
    pub fn new(value: usize) -> CtResult<Self> {
        if value == 0 {
            return Err(CtError::EmptyInput("vocabulary"));
        }

        Ok(Self(value))
    }

    pub fn value(&self) -> usize {
        self.0
    }
}

/// Width of each embedding vector.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModelDimension(usize);

impl ModelDimension {
    pub fn new(value: usize) -> CtResult<Self> {
        if value == 0 {
            return Err(CtError::EmptyInput("model dimension"));
        }

        Ok(Self(value))
    }

    pub fn value(&self) -> usize {
        self.0
    }
}

/// Positive optimizer step size.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LearningRate(f32);

impl LearningRate {
    pub fn new(value: f32) -> CtResult<Self> {
        if !value.is_finite() || value <= 0.0 {
            return Err(CtError::InvalidLearningRate(value));
        }

        Ok(Self(value))
    }

    pub fn value(&self) -> f32 {
        self.0
    }
}

/// Categorical product object: `A x B`.
#[derive(Debug, Clone, PartialEq)]
pub struct Product<A, B> {
    first: A,
    second: B,
}

impl<A, B> Product<A, B> {
    pub fn new(first: A, second: B) -> Self {
        Self { first, second }
    }

    pub fn first(&self) -> &A {
        &self.first
    }

    pub fn second(&self) -> &B {
        &self.second
    }

    pub fn into_parts(self) -> (A, B) {
        (self.first, self.second)
    }
}

pub type TrainingExample = Product<TokenId, TokenId>;

/// Non-empty next-token training pairs.
#[derive(Debug, Clone, PartialEq)]
pub struct TrainingSet(Vec<TrainingExample>);

impl TrainingSet {
    pub fn new(examples: impl IntoIterator<Item = TrainingExample>) -> CtResult<Self> {
        let examples = examples.into_iter().collect::<Vec<_>>();

        if examples.is_empty() {
            return Err(CtError::EmptyInput("training set"));
        }

        Ok(Self(examples))
    }

    pub fn examples(&self) -> &[TrainingExample] {
        &self.0
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

/// Tiny model parameters for an embedding plus language-model head.
#[derive(Debug, Clone, PartialEq)]
pub struct Parameters {
    pub(crate) embedding: Vec<Vec<f32>>,
    pub(crate) lm_head: Vec<Vec<f32>>,
    pub(crate) bias: Vec<f32>,
}

impl Parameters {
    pub fn init(vocab_size: VocabSize, d_model: ModelDimension) -> Self {
        let vocab_size = vocab_size.value();
        let d_model = d_model.value();

        Self {
            embedding: init_matrix(vocab_size, d_model, 0.2, 1),
            lm_head: init_matrix(d_model, vocab_size, 0.2, 2),
            bias: vec![0.0; vocab_size],
        }
    }

    pub fn vocab_size(&self) -> usize {
        self.bias.len()
    }

    pub fn d_model(&self) -> usize {
        self.embedding.first().map_or(0, Vec::len)
    }

    pub fn embedding_table(&self) -> &[Vec<f32>] {
        &self.embedding
    }

    pub fn lm_head(&self) -> &[Vec<f32>] {
        &self.lm_head
    }

    pub fn bias(&self) -> &[f32] {
        &self.bias
    }
}

pub(crate) fn init_matrix(rows: usize, cols: usize, scale: f32, seed: usize) -> Vec<Vec<f32>> {
    let mut out = vec![vec![0.0; cols]; rows];

    for (row_index, row) in out.iter_mut().enumerate() {
        for (col_index, value) in row.iter_mut().enumerate() {
            let raw = ((row_index * cols + col_index) * 37 + seed * 101) % 1000;
            let unit = raw as f32 / 1000.0;
            *value = (unit - 0.5) * scale;
        }
    }

    out
}

pub(crate) fn approx_eq(a: f32, b: f32, eps: f32) -> bool {
    (a - b).abs() <= eps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distribution_rejects_non_normalized_values() {
        let result = Distribution::new(vec![0.4, 0.4]);

        assert!(matches!(result, Err(CtError::InvalidProbability(_))));
    }

    #[test]
    fn token_sequence_rejects_empty_input() {
        let result = TokenSequence::new(vec![]);

        assert!(matches!(result, Err(CtError::EmptyInput("token sequence"))));
    }
}
