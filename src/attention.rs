//! Tiny typed attention boundary for the Transformer roadmap.
//!
//! This module does not implement a full Transformer. It makes the first small
//! attention-specific shapes explicit:
//!
//! - projected queries and keys become query-by-key scores,
//! - masks turn illegal score positions into negligible softmax inputs,
//! - query-by-key scores become row-wise attention probabilities,
//! - attention probabilities mix value vectors into output vectors,
//! - multiple head outputs concatenate into a multi-head output,
//! - the concatenated heads project back into a hidden sequence width,
//! - residual addition preserves the hidden sequence boundary,
//! - layer normalization preserves the hidden sequence boundary,
//! - a position-wise feed-forward map preserves the hidden sequence boundary,
//! - positional encoding adds position information while preserving shape,
//! - a single-head block sketch composes those boundaries end to end,
//! - a masked multi-head block accepts attention masks at the block boundary,
//! - a structured parameter object owns position, block, and readout pieces,
//! - a training-state object owns parameters, learning rate, and step count,
//! - a composed block train step updates readout, feed-forward,
//!   normalization, and attention projection parameters from sequence targets.

use crate::category::{Morphism, StepCount};
use crate::domain::{
    Distribution, LearningRate, Logits, Loss, ModelDimension, Product, TokenSequence, Vector,
    VocabSize,
};
use crate::error::{CtError, CtResult};
use crate::ml::Softmax;

const MASKED_SCORE: f32 = -1_000_000.0;

/// Number of positions in a sequence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SequenceLength(usize);

impl SequenceLength {
    pub fn new(value: usize) -> CtResult<Self> {
        if value == 0 {
            return Err(CtError::EmptyInput("sequence length"));
        }

        Ok(Self(value))
    }

    pub fn value(&self) -> usize {
        self.0
    }
}

/// Number of parallel attention heads.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HeadCount(usize);

impl HeadCount {
    pub fn new(value: usize) -> CtResult<Self> {
        if value == 0 {
            return Err(CtError::EmptyInput("head count"));
        }

        Ok(Self(value))
    }

    pub fn value(&self) -> usize {
        self.0
    }
}

/// Width of one attention head.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HeadDimension(usize);

impl HeadDimension {
    pub fn new(value: usize) -> CtResult<Self> {
        if value == 0 {
            return Err(CtError::EmptyInput("head dimension"));
        }

        Ok(Self(value))
    }

    pub fn value(&self) -> usize {
        self.0
    }
}

/// Positive stabilizer used in layer normalization.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NormalizationEpsilon(f32);

impl NormalizationEpsilon {
    pub fn new(value: f32) -> CtResult<Self> {
        if !value.is_finite() || value <= 0.0 {
            return Err(CtError::ShapeMismatch {
                op: "normalization epsilon",
                expected: "positive finite epsilon".to_string(),
                got: format!("epsilon {value}"),
            });
        }

        Ok(Self(value))
    }

    pub fn value(&self) -> f32 {
        self.0
    }
}

/// Projected query vectors for one attention head.
#[derive(Debug, Clone, PartialEq)]
pub struct QuerySequence {
    sequence_len: SequenceLength,
    head_dimension: HeadDimension,
    rows: Vec<Vector>,
}

impl QuerySequence {
    pub fn new(rows: Vec<Vec<f32>>) -> CtResult<Self> {
        let matrix = AttentionVectorRows::new("query sequence", rows)?;

        Ok(Self {
            sequence_len: matrix.sequence_len,
            head_dimension: matrix.head_dimension,
            rows: matrix.rows,
        })
    }

    pub fn sequence_len(&self) -> SequenceLength {
        self.sequence_len
    }

    pub fn head_dimension(&self) -> HeadDimension {
        self.head_dimension
    }

    pub fn rows(&self) -> &[Vector] {
        &self.rows
    }
}

/// Projected key vectors for one attention head.
#[derive(Debug, Clone, PartialEq)]
pub struct KeySequence {
    sequence_len: SequenceLength,
    head_dimension: HeadDimension,
    rows: Vec<Vector>,
}

impl KeySequence {
    pub fn new(rows: Vec<Vec<f32>>) -> CtResult<Self> {
        let matrix = AttentionVectorRows::new("key sequence", rows)?;

        Ok(Self {
            sequence_len: matrix.sequence_len,
            head_dimension: matrix.head_dimension,
            rows: matrix.rows,
        })
    }

    pub fn sequence_len(&self) -> SequenceLength {
        self.sequence_len
    }

    pub fn head_dimension(&self) -> HeadDimension {
        self.head_dimension
    }

    pub fn rows(&self) -> &[Vector] {
        &self.rows
    }
}

/// Projected value vectors for one attention head.
#[derive(Debug, Clone, PartialEq)]
pub struct ValueSequence {
    sequence_len: SequenceLength,
    head_dimension: HeadDimension,
    rows: Vec<Vector>,
}

impl ValueSequence {
    pub fn new(rows: Vec<Vec<f32>>) -> CtResult<Self> {
        let matrix = AttentionVectorRows::new("value sequence", rows)?;

        Ok(Self {
            sequence_len: matrix.sequence_len,
            head_dimension: matrix.head_dimension,
            rows: matrix.rows,
        })
    }

    pub fn sequence_len(&self) -> SequenceLength {
        self.sequence_len
    }

    pub fn head_dimension(&self) -> HeadDimension {
        self.head_dimension
    }

    pub fn rows(&self) -> &[Vector] {
        &self.rows
    }
}

/// Hidden vectors over sequence positions.
#[derive(Debug, Clone, PartialEq)]
pub struct HiddenSequence {
    sequence_len: SequenceLength,
    model_dimension: ModelDimension,
    rows: Vec<Vector>,
}

impl HiddenSequence {
    pub fn new(rows: Vec<Vec<f32>>) -> CtResult<Self> {
        let matrix = AttentionVectorRows::new("hidden sequence", rows)?;

        Ok(Self {
            sequence_len: matrix.sequence_len,
            model_dimension: ModelDimension::new(matrix.head_dimension.value())?,
            rows: matrix.rows,
        })
    }

    pub fn sequence_len(&self) -> SequenceLength {
        self.sequence_len
    }

    pub fn model_dimension(&self) -> ModelDimension {
        self.model_dimension
    }

    pub fn rows(&self) -> &[Vector] {
        &self.rows
    }
}

/// A finite table of position vectors added to hidden states.
#[derive(Debug, Clone, PartialEq)]
pub struct PositionalEncoding {
    max_sequence_len: SequenceLength,
    model_dimension: ModelDimension,
    rows: Vec<Vector>,
}

impl PositionalEncoding {
    pub fn new(rows: Vec<Vec<f32>>) -> CtResult<Self> {
        let matrix = AttentionVectorRows::new("positional encoding", rows)?;

        Ok(Self {
            max_sequence_len: matrix.sequence_len,
            model_dimension: ModelDimension::new(matrix.head_dimension.value())?,
            rows: matrix.rows,
        })
    }

    pub fn max_sequence_len(&self) -> SequenceLength {
        self.max_sequence_len
    }

    pub fn model_dimension(&self) -> ModelDimension {
        self.model_dimension
    }
}

#[derive(Debug, Clone, PartialEq)]
struct AttentionVectorRows {
    sequence_len: SequenceLength,
    head_dimension: HeadDimension,
    rows: Vec<Vector>,
}

impl AttentionVectorRows {
    fn new(kind: &'static str, rows: Vec<Vec<f32>>) -> CtResult<Self> {
        if rows.is_empty() {
            return Err(CtError::EmptyInput(kind));
        }

        let head_dimension = rows[0].len();

        if head_dimension == 0 {
            return Err(CtError::EmptyInput("attention vector row"));
        }

        for row in &rows {
            if row.len() != head_dimension {
                return Err(CtError::ShapeMismatch {
                    op: kind,
                    expected: format!("all rows have {head_dimension} columns"),
                    got: format!("row with {} columns", row.len()),
                });
            }

            if row.iter().any(|value| !value.is_finite()) {
                return Err(CtError::ShapeMismatch {
                    op: kind,
                    expected: "all vector values are finite".to_string(),
                    got: "non-finite vector value".to_string(),
                });
            }
        }

        Ok(Self {
            sequence_len: SequenceLength::new(rows.len())?,
            head_dimension: HeadDimension::new(head_dimension)?,
            rows: rows.into_iter().map(Vector::new).collect(),
        })
    }
}

/// Query-by-key scores before row-wise softmax.
#[derive(Debug, Clone, PartialEq)]
pub struct AttentionScores {
    query_len: SequenceLength,
    key_len: SequenceLength,
    rows: Vec<Logits>,
}

impl AttentionScores {
    pub fn new(rows: Vec<Vec<f32>>) -> CtResult<Self> {
        if rows.is_empty() {
            return Err(CtError::EmptyInput("attention scores"));
        }

        let key_len = rows[0].len();

        if key_len == 0 {
            return Err(CtError::EmptyInput("attention score row"));
        }

        for row in &rows {
            if row.len() != key_len {
                return Err(CtError::ShapeMismatch {
                    op: "attention scores",
                    expected: format!("all rows have {key_len} columns"),
                    got: format!("row with {} columns", row.len()),
                });
            }

            if row.iter().any(|value| !value.is_finite()) {
                return Err(CtError::ShapeMismatch {
                    op: "attention scores",
                    expected: "all score values are finite".to_string(),
                    got: "non-finite score value".to_string(),
                });
            }
        }

        Ok(Self {
            query_len: SequenceLength::new(rows.len())?,
            key_len: SequenceLength::new(key_len)?,
            rows: rows.into_iter().map(Logits::new).collect(),
        })
    }

    pub fn query_len(&self) -> SequenceLength {
        self.query_len
    }

    pub fn key_len(&self) -> SequenceLength {
        self.key_len
    }

    pub fn rows(&self) -> &[Logits] {
        &self.rows
    }
}

/// Allowed query-by-key positions before attention softmax.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttentionMask {
    query_len: SequenceLength,
    key_len: SequenceLength,
    rows: Vec<Vec<bool>>,
}

impl AttentionMask {
    pub fn new(rows: Vec<Vec<bool>>) -> CtResult<Self> {
        if rows.is_empty() {
            return Err(CtError::EmptyInput("attention mask"));
        }

        let key_len = rows[0].len();

        if key_len == 0 {
            return Err(CtError::EmptyInput("attention mask row"));
        }

        for row in &rows {
            if row.len() != key_len {
                return Err(CtError::ShapeMismatch {
                    op: "attention mask",
                    expected: format!("all rows have {key_len} columns"),
                    got: format!("row with {} columns", row.len()),
                });
            }

            if !row.iter().any(|allowed| *allowed) {
                return Err(CtError::EmptyInput("attention mask row allows no keys"));
            }
        }

        Ok(Self {
            query_len: SequenceLength::new(rows.len())?,
            key_len: SequenceLength::new(key_len)?,
            rows,
        })
    }

    pub fn query_len(&self) -> SequenceLength {
        self.query_len
    }

    pub fn key_len(&self) -> SequenceLength {
        self.key_len
    }

    pub fn rows(&self) -> &[Vec<bool>] {
        &self.rows
    }
}

/// Computes scaled query-key dot-product scores.
#[derive(Debug, Clone)]
pub struct ScaledDotProductScores;

impl Morphism<Product<QuerySequence, KeySequence>, AttentionScores> for ScaledDotProductScores {
    fn name(&self) -> &'static str {
        "scaled_dot_product_scores"
    }

    fn apply(&self, input: Product<QuerySequence, KeySequence>) -> CtResult<AttentionScores> {
        let (queries, keys) = input.into_parts();
        let query_dimension = queries.head_dimension();
        let key_dimension = keys.head_dimension();

        if query_dimension != key_dimension {
            return Err(CtError::ShapeMismatch {
                op: "scaled dot-product attention scores",
                expected: format!("query head dimension {}", query_dimension.value()),
                got: format!("key head dimension {}", key_dimension.value()),
            });
        }

        let scale = (query_dimension.value() as f32).sqrt();
        let rows = queries
            .rows()
            .iter()
            .map(|query| {
                keys.rows()
                    .iter()
                    .map(|key| dot_product(query.as_slice(), key.as_slice()) / scale)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        AttentionScores::new(rows)
    }
}

fn dot_product(left: &[f32], right: &[f32]) -> f32 {
    left.iter()
        .zip(right.iter())
        .map(|(left, right)| left * right)
        .sum()
}

/// Applies a boolean attention mask to score rows before softmax.
#[derive(Debug, Clone)]
pub struct MaskedAttentionScores;

impl Morphism<Product<AttentionScores, AttentionMask>, AttentionScores> for MaskedAttentionScores {
    fn name(&self) -> &'static str {
        "masked_attention_scores"
    }

    fn apply(&self, input: Product<AttentionScores, AttentionMask>) -> CtResult<AttentionScores> {
        let (scores, mask) = input.into_parts();

        if scores.query_len() != mask.query_len() || scores.key_len() != mask.key_len() {
            return Err(CtError::ShapeMismatch {
                op: "masked attention scores",
                expected: format!(
                    "{} query rows x {} key columns",
                    scores.query_len().value(),
                    scores.key_len().value()
                ),
                got: format!(
                    "{} query rows x {} key columns",
                    mask.query_len().value(),
                    mask.key_len().value()
                ),
            });
        }

        let rows = scores
            .rows()
            .iter()
            .zip(mask.rows())
            .map(|(score_row, mask_row)| {
                score_row
                    .as_slice()
                    .iter()
                    .zip(mask_row)
                    .map(|(score, allowed)| if *allowed { *score } else { MASKED_SCORE })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        AttentionScores::new(rows)
    }
}

/// Row-wise attention probabilities over key positions.
#[derive(Debug, Clone, PartialEq)]
pub struct AttentionWeights {
    query_len: SequenceLength,
    key_len: SequenceLength,
    rows: Vec<Distribution>,
}

impl AttentionWeights {
    pub fn new(rows: Vec<Distribution>) -> CtResult<Self> {
        if rows.is_empty() {
            return Err(CtError::EmptyInput("attention weights"));
        }

        let key_len = rows[0].as_slice().len();

        if key_len == 0 {
            return Err(CtError::EmptyInput("attention weight row"));
        }

        for row in &rows {
            if row.as_slice().len() != key_len {
                return Err(CtError::ShapeMismatch {
                    op: "attention weights",
                    expected: format!("all rows have {key_len} columns"),
                    got: format!("row with {} columns", row.as_slice().len()),
                });
            }
        }

        Ok(Self {
            query_len: SequenceLength::new(rows.len())?,
            key_len: SequenceLength::new(key_len)?,
            rows,
        })
    }

    pub fn query_len(&self) -> SequenceLength {
        self.query_len
    }

    pub fn key_len(&self) -> SequenceLength {
        self.key_len
    }

    pub fn rows(&self) -> &[Distribution] {
        &self.rows
    }
}

/// Weighted value vectors, one output row per query position.
#[derive(Debug, Clone, PartialEq)]
pub struct AttentionOutput {
    sequence_len: SequenceLength,
    head_dimension: HeadDimension,
    rows: Vec<Vector>,
}

impl AttentionOutput {
    pub fn new(rows: Vec<Vec<f32>>) -> CtResult<Self> {
        let matrix = AttentionVectorRows::new("attention output", rows)?;

        Ok(Self {
            sequence_len: matrix.sequence_len,
            head_dimension: matrix.head_dimension,
            rows: matrix.rows,
        })
    }

    pub fn sequence_len(&self) -> SequenceLength {
        self.sequence_len
    }

    pub fn head_dimension(&self) -> HeadDimension {
        self.head_dimension
    }

    pub fn rows(&self) -> &[Vector] {
        &self.rows
    }
}

/// Validated collection of single-head attention outputs.
#[derive(Debug, Clone, PartialEq)]
pub struct AttentionHeadOutputs {
    head_count: HeadCount,
    sequence_len: SequenceLength,
    head_dimension: HeadDimension,
    heads: Vec<AttentionOutput>,
}

impl AttentionHeadOutputs {
    pub fn new(heads: Vec<AttentionOutput>) -> CtResult<Self> {
        if heads.is_empty() {
            return Err(CtError::EmptyInput("attention head outputs"));
        }

        let sequence_len = heads[0].sequence_len();
        let head_dimension = heads[0].head_dimension();

        for head in &heads {
            if head.sequence_len() != sequence_len {
                return Err(CtError::ShapeMismatch {
                    op: "attention head outputs",
                    expected: format!("all heads have {} sequence rows", sequence_len.value()),
                    got: format!("head with {} sequence rows", head.sequence_len().value()),
                });
            }

            if head.head_dimension() != head_dimension {
                return Err(CtError::ShapeMismatch {
                    op: "attention head outputs",
                    expected: format!("all heads have dimension {}", head_dimension.value()),
                    got: format!("head dimension {}", head.head_dimension().value()),
                });
            }
        }

        Ok(Self {
            head_count: HeadCount::new(heads.len())?,
            sequence_len,
            head_dimension,
            heads,
        })
    }

    pub fn head_count(&self) -> HeadCount {
        self.head_count
    }

    pub fn sequence_len(&self) -> SequenceLength {
        self.sequence_len
    }

    pub fn head_dimension(&self) -> HeadDimension {
        self.head_dimension
    }

    pub fn heads(&self) -> &[AttentionOutput] {
        &self.heads
    }
}

/// Concatenated output of several attention heads.
#[derive(Debug, Clone, PartialEq)]
pub struct MultiHeadOutput {
    sequence_len: SequenceLength,
    head_count: HeadCount,
    head_dimension: HeadDimension,
    model_dimension: ModelDimension,
    rows: Vec<Vector>,
}

impl MultiHeadOutput {
    fn new(
        rows: Vec<Vec<f32>>,
        head_count: HeadCount,
        head_dimension: HeadDimension,
    ) -> CtResult<Self> {
        let matrix = AttentionVectorRows::new("multi-head output", rows)?;
        let expected_dimension = head_count.value() * head_dimension.value();

        if matrix.head_dimension.value() != expected_dimension {
            return Err(CtError::ShapeMismatch {
                op: "multi-head output",
                expected: format!("row dimension {expected_dimension}"),
                got: format!("row dimension {}", matrix.head_dimension.value()),
            });
        }

        Ok(Self {
            sequence_len: matrix.sequence_len,
            head_count,
            head_dimension,
            model_dimension: ModelDimension::new(expected_dimension)?,
            rows: matrix.rows,
        })
    }

    pub fn sequence_len(&self) -> SequenceLength {
        self.sequence_len
    }

    pub fn head_count(&self) -> HeadCount {
        self.head_count
    }

    pub fn head_dimension(&self) -> HeadDimension {
        self.head_dimension
    }

    pub fn model_dimension(&self) -> ModelDimension {
        self.model_dimension
    }

    pub fn rows(&self) -> &[Vector] {
        &self.rows
    }
}

/// Output sequence after the multi-head output projection.
#[derive(Debug, Clone, PartialEq)]
pub struct ProjectedAttentionOutput {
    sequence_len: SequenceLength,
    model_dimension: ModelDimension,
    rows: Vec<Vector>,
}

impl ProjectedAttentionOutput {
    pub fn new(rows: Vec<Vec<f32>>) -> CtResult<Self> {
        let matrix = AttentionVectorRows::new("projected attention output", rows)?;

        Ok(Self {
            sequence_len: matrix.sequence_len,
            model_dimension: ModelDimension::new(matrix.head_dimension.value())?,
            rows: matrix.rows,
        })
    }

    pub fn sequence_len(&self) -> SequenceLength {
        self.sequence_len
    }

    pub fn model_dimension(&self) -> ModelDimension {
        self.model_dimension
    }

    pub fn rows(&self) -> &[Vector] {
        &self.rows
    }
}

/// Vocabulary logits for every position in a hidden sequence.
#[derive(Debug, Clone, PartialEq)]
pub struct SequenceLogits {
    sequence_len: SequenceLength,
    vocab_size: VocabSize,
    rows: Vec<Logits>,
}

impl SequenceLogits {
    pub fn new(rows: Vec<Vec<f32>>) -> CtResult<Self> {
        if rows.is_empty() {
            return Err(CtError::EmptyInput("sequence logits"));
        }

        let vocab_size = rows[0].len();

        if vocab_size == 0 {
            return Err(CtError::EmptyInput("sequence logits row"));
        }

        for row in &rows {
            if row.len() != vocab_size {
                return Err(CtError::ShapeMismatch {
                    op: "sequence logits",
                    expected: format!("all rows have {vocab_size} columns"),
                    got: format!("row with {} columns", row.len()),
                });
            }

            if row.iter().any(|value| !value.is_finite()) {
                return Err(CtError::ShapeMismatch {
                    op: "sequence logits",
                    expected: "all logit values are finite".to_string(),
                    got: "non-finite logit value".to_string(),
                });
            }
        }

        Ok(Self {
            sequence_len: SequenceLength::new(rows.len())?,
            vocab_size: VocabSize::new(vocab_size)?,
            rows: rows.into_iter().map(Logits::new).collect(),
        })
    }

    pub fn sequence_len(&self) -> SequenceLength {
        self.sequence_len
    }

    pub fn vocab_size(&self) -> VocabSize {
        self.vocab_size
    }

    pub fn rows(&self) -> &[Logits] {
        &self.rows
    }
}

/// Learned language-model readout applied to each hidden position.
#[derive(Debug, Clone, PartialEq)]
pub struct TransformerReadout {
    input_dimension: ModelDimension,
    vocab_size: VocabSize,
    weight: Vec<Vec<f32>>,
    bias: Vec<f32>,
}

impl TransformerReadout {
    pub fn new(weight: Vec<Vec<f32>>, bias: Vec<f32>) -> CtResult<Self> {
        let (input_dimension, output_dimension) =
            validate_linear_parts("transformer readout", &weight, &bias)?;

        Ok(Self {
            input_dimension,
            vocab_size: VocabSize::new(output_dimension.value())?,
            weight,
            bias,
        })
    }

    pub fn input_dimension(&self) -> ModelDimension {
        self.input_dimension
    }

    pub fn vocab_size(&self) -> VocabSize {
        self.vocab_size
    }

    pub fn weight(&self) -> &[Vec<f32>] {
        &self.weight
    }

    pub fn bias(&self) -> &[f32] {
        &self.bias
    }
}

/// Learned output projection after head concatenation.
#[derive(Debug, Clone, PartialEq)]
pub struct AttentionOutputProjection {
    input_dimension: ModelDimension,
    output_dimension: ModelDimension,
    weight: Vec<Vec<f32>>,
    bias: Vec<f32>,
}

impl AttentionOutputProjection {
    pub fn new(weight: Vec<Vec<f32>>, bias: Vec<f32>) -> CtResult<Self> {
        if weight.is_empty() {
            return Err(CtError::EmptyInput("attention output projection weight"));
        }

        if bias.is_empty() {
            return Err(CtError::EmptyInput("attention output projection bias"));
        }

        let output_dimension = bias.len();

        if bias.iter().any(|value| !value.is_finite()) {
            return Err(CtError::ShapeMismatch {
                op: "attention output projection",
                expected: "finite bias values".to_string(),
                got: "non-finite bias value".to_string(),
            });
        }

        for row in &weight {
            if row.len() != output_dimension {
                return Err(CtError::ShapeMismatch {
                    op: "attention output projection",
                    expected: format!("weight rows have {output_dimension} columns"),
                    got: format!("weight row with {} columns", row.len()),
                });
            }

            if row.iter().any(|value| !value.is_finite()) {
                return Err(CtError::ShapeMismatch {
                    op: "attention output projection",
                    expected: "finite weight values".to_string(),
                    got: "non-finite weight value".to_string(),
                });
            }
        }

        Ok(Self {
            input_dimension: ModelDimension::new(weight.len())?,
            output_dimension: ModelDimension::new(output_dimension)?,
            weight,
            bias,
        })
    }

    pub fn input_dimension(&self) -> ModelDimension {
        self.input_dimension
    }

    pub fn output_dimension(&self) -> ModelDimension {
        self.output_dimension
    }

    pub fn weight(&self) -> &[Vec<f32>] {
        &self.weight
    }

    pub fn bias(&self) -> &[f32] {
        &self.bias
    }
}

/// Scale, shift, and epsilon parameters for layer normalization.
#[derive(Debug, Clone, PartialEq)]
pub struct LayerNormParameters {
    model_dimension: ModelDimension,
    scale: Vec<f32>,
    shift: Vec<f32>,
    epsilon: NormalizationEpsilon,
}

impl LayerNormParameters {
    pub fn new(scale: Vec<f32>, shift: Vec<f32>, epsilon: NormalizationEpsilon) -> CtResult<Self> {
        if scale.is_empty() {
            return Err(CtError::EmptyInput("layer norm scale"));
        }

        if shift.is_empty() {
            return Err(CtError::EmptyInput("layer norm shift"));
        }

        if scale.len() != shift.len() {
            return Err(CtError::ShapeMismatch {
                op: "layer norm parameters",
                expected: format!("scale and shift length {}", scale.len()),
                got: format!("shift length {}", shift.len()),
            });
        }

        if scale.iter().any(|value| !value.is_finite()) {
            return Err(CtError::ShapeMismatch {
                op: "layer norm parameters",
                expected: "finite scale values".to_string(),
                got: "non-finite scale value".to_string(),
            });
        }

        if shift.iter().any(|value| !value.is_finite()) {
            return Err(CtError::ShapeMismatch {
                op: "layer norm parameters",
                expected: "finite shift values".to_string(),
                got: "non-finite shift value".to_string(),
            });
        }

        Ok(Self {
            model_dimension: ModelDimension::new(scale.len())?,
            scale,
            shift,
            epsilon,
        })
    }

    pub fn identity(model_dimension: ModelDimension) -> Self {
        Self {
            model_dimension,
            scale: vec![1.0; model_dimension.value()],
            shift: vec![0.0; model_dimension.value()],
            epsilon: NormalizationEpsilon(1e-5),
        }
    }

    pub fn model_dimension(&self) -> ModelDimension {
        self.model_dimension
    }

    pub fn scale(&self) -> &[f32] {
        &self.scale
    }

    pub fn shift(&self) -> &[f32] {
        &self.shift
    }

    pub fn epsilon(&self) -> NormalizationEpsilon {
        self.epsilon
    }
}

/// Layer normalization over each hidden vector independently.
#[derive(Debug, Clone, PartialEq)]
pub struct LayerNormalization {
    parameters: LayerNormParameters,
}

impl LayerNormalization {
    pub fn new(parameters: LayerNormParameters) -> Self {
        Self { parameters }
    }

    pub fn model_dimension(&self) -> ModelDimension {
        self.parameters.model_dimension()
    }

    pub fn parameters(&self) -> &LayerNormParameters {
        &self.parameters
    }
}

#[derive(Debug, Clone, PartialEq)]
struct FeedForwardRowCache {
    input: Vec<f32>,
    pre_activation: Vec<f32>,
    activation: Vec<f32>,
    output: Vec<f32>,
}

#[derive(Debug, Clone, PartialEq)]
struct AttentionHeadTrainingCache {
    queries: QuerySequence,
    keys: KeySequence,
    values: ValueSequence,
    weights: AttentionWeights,
    output: AttentionOutput,
}

#[derive(Debug, Clone, PartialEq)]
struct MaskedBlockTrainingCache {
    output: HiddenSequence,
    with_feed_forward: HiddenSequence,
    with_attention: HiddenSequence,
    multi_head_output: MultiHeadOutput,
    attention_heads: Vec<AttentionHeadTrainingCache>,
    feed_forward_rows: Vec<FeedForwardRowCache>,
}

/// Position-wise two-layer feed-forward sublayer.
#[derive(Debug, Clone, PartialEq)]
pub struct PositionWiseFeedForward {
    input_dimension: ModelDimension,
    hidden_dimension: ModelDimension,
    output_dimension: ModelDimension,
    first_weight: Vec<Vec<f32>>,
    first_bias: Vec<f32>,
    second_weight: Vec<Vec<f32>>,
    second_bias: Vec<f32>,
}

impl PositionWiseFeedForward {
    pub fn new(
        first_weight: Vec<Vec<f32>>,
        first_bias: Vec<f32>,
        second_weight: Vec<Vec<f32>>,
        second_bias: Vec<f32>,
    ) -> CtResult<Self> {
        let (input_dimension, hidden_dimension) = validate_linear_parts(
            "position-wise feed-forward first layer",
            &first_weight,
            &first_bias,
        )?;
        let (second_input_dimension, output_dimension) = validate_linear_parts(
            "position-wise feed-forward second layer",
            &second_weight,
            &second_bias,
        )?;

        if second_input_dimension != hidden_dimension {
            return Err(CtError::ShapeMismatch {
                op: "position-wise feed-forward",
                expected: format!("second input dimension {}", hidden_dimension.value()),
                got: format!("second input dimension {}", second_input_dimension.value()),
            });
        }

        if output_dimension != input_dimension {
            return Err(CtError::ShapeMismatch {
                op: "position-wise feed-forward",
                expected: format!("output dimension {}", input_dimension.value()),
                got: format!("output dimension {}", output_dimension.value()),
            });
        }

        Ok(Self {
            input_dimension,
            hidden_dimension,
            output_dimension,
            first_weight,
            first_bias,
            second_weight,
            second_bias,
        })
    }

    pub fn input_dimension(&self) -> ModelDimension {
        self.input_dimension
    }

    pub fn hidden_dimension(&self) -> ModelDimension {
        self.hidden_dimension
    }

    pub fn output_dimension(&self) -> ModelDimension {
        self.output_dimension
    }

    pub fn first_weight(&self) -> &[Vec<f32>] {
        &self.first_weight
    }

    pub fn first_bias(&self) -> &[f32] {
        &self.first_bias
    }

    pub fn second_weight(&self) -> &[Vec<f32>] {
        &self.second_weight
    }

    pub fn second_bias(&self) -> &[f32] {
        &self.second_bias
    }
}

#[derive(Debug, Clone, PartialEq)]
struct HiddenProjection {
    op: &'static str,
    input_dimension: ModelDimension,
    head_dimension: HeadDimension,
    weight: Vec<Vec<f32>>,
    bias: Vec<f32>,
}

impl HiddenProjection {
    fn new(op: &'static str, weight: Vec<Vec<f32>>, bias: Vec<f32>) -> CtResult<Self> {
        let (input_dimension, output_dimension) = validate_linear_parts(op, &weight, &bias)?;

        Ok(Self {
            op,
            input_dimension,
            head_dimension: HeadDimension::new(output_dimension.value())?,
            weight,
            bias,
        })
    }

    fn project(&self, input: &HiddenSequence) -> CtResult<Vec<Vec<f32>>> {
        if input.model_dimension() != self.input_dimension {
            return Err(CtError::ShapeMismatch {
                op: self.op,
                expected: format!("input dimension {}", self.input_dimension.value()),
                got: format!("input dimension {}", input.model_dimension().value()),
            });
        }

        Ok(input
            .rows()
            .iter()
            .map(|row| project_row(row.as_slice(), &self.weight, &self.bias))
            .collect::<Vec<_>>())
    }

    fn input_dimension(&self) -> ModelDimension {
        self.input_dimension
    }

    fn head_dimension(&self) -> HeadDimension {
        self.head_dimension
    }

    fn weight(&self) -> &[Vec<f32>] {
        &self.weight
    }

    fn bias(&self) -> &[f32] {
        &self.bias
    }
}

/// Learned projection from hidden states to query vectors.
#[derive(Debug, Clone, PartialEq)]
pub struct HiddenToQuery {
    projection: HiddenProjection,
}

impl HiddenToQuery {
    pub fn new(weight: Vec<Vec<f32>>, bias: Vec<f32>) -> CtResult<Self> {
        Ok(Self {
            projection: HiddenProjection::new("hidden-to-query projection", weight, bias)?,
        })
    }

    pub fn input_dimension(&self) -> ModelDimension {
        self.projection.input_dimension()
    }

    pub fn head_dimension(&self) -> HeadDimension {
        self.projection.head_dimension()
    }

    pub fn weight(&self) -> &[Vec<f32>] {
        self.projection.weight()
    }

    pub fn bias(&self) -> &[f32] {
        self.projection.bias()
    }
}

/// Learned projection from hidden states to key vectors.
#[derive(Debug, Clone, PartialEq)]
pub struct HiddenToKey {
    projection: HiddenProjection,
}

impl HiddenToKey {
    pub fn new(weight: Vec<Vec<f32>>, bias: Vec<f32>) -> CtResult<Self> {
        Ok(Self {
            projection: HiddenProjection::new("hidden-to-key projection", weight, bias)?,
        })
    }

    pub fn input_dimension(&self) -> ModelDimension {
        self.projection.input_dimension()
    }

    pub fn head_dimension(&self) -> HeadDimension {
        self.projection.head_dimension()
    }

    pub fn weight(&self) -> &[Vec<f32>] {
        self.projection.weight()
    }

    pub fn bias(&self) -> &[f32] {
        self.projection.bias()
    }
}

/// Learned projection from hidden states to value vectors.
#[derive(Debug, Clone, PartialEq)]
pub struct HiddenToValue {
    projection: HiddenProjection,
}

impl HiddenToValue {
    pub fn new(weight: Vec<Vec<f32>>, bias: Vec<f32>) -> CtResult<Self> {
        Ok(Self {
            projection: HiddenProjection::new("hidden-to-value projection", weight, bias)?,
        })
    }

    pub fn input_dimension(&self) -> ModelDimension {
        self.projection.input_dimension()
    }

    pub fn head_dimension(&self) -> HeadDimension {
        self.projection.head_dimension()
    }

    pub fn weight(&self) -> &[Vec<f32>] {
        self.projection.weight()
    }

    pub fn bias(&self) -> &[f32] {
        self.projection.bias()
    }
}

/// A tiny single-head Transformer block sketch.
#[derive(Debug, Clone, PartialEq)]
pub struct SingleHeadTransformerBlock {
    model_dimension: ModelDimension,
    query_projection: HiddenToQuery,
    key_projection: HiddenToKey,
    value_projection: HiddenToValue,
    output_projection: AttentionOutputProjection,
    attention_norm: LayerNormalization,
    feed_forward: PositionWiseFeedForward,
    feed_forward_norm: LayerNormalization,
}

impl SingleHeadTransformerBlock {
    pub fn new(
        query_projection: HiddenToQuery,
        key_projection: HiddenToKey,
        value_projection: HiddenToValue,
        output_projection: AttentionOutputProjection,
        attention_norm: LayerNormalization,
        feed_forward: PositionWiseFeedForward,
        feed_forward_norm: LayerNormalization,
    ) -> CtResult<Self> {
        let model_dimension = query_projection.input_dimension();

        validate_projection_input(
            "single-head block key projection",
            model_dimension,
            key_projection.input_dimension(),
        )?;
        validate_projection_input(
            "single-head block value projection",
            model_dimension,
            value_projection.input_dimension(),
        )?;

        if query_projection.head_dimension() != key_projection.head_dimension() {
            return Err(CtError::ShapeMismatch {
                op: "single-head block",
                expected: format!(
                    "query/key head dimension {}",
                    query_projection.head_dimension().value()
                ),
                got: format!(
                    "key head dimension {}",
                    key_projection.head_dimension().value()
                ),
            });
        }

        if output_projection.input_dimension().value() != value_projection.head_dimension().value()
        {
            return Err(CtError::ShapeMismatch {
                op: "single-head block",
                expected: format!(
                    "output projection input dimension {}",
                    value_projection.head_dimension().value()
                ),
                got: format!(
                    "output projection input dimension {}",
                    output_projection.input_dimension().value()
                ),
            });
        }

        validate_projection_input(
            "single-head block output projection",
            model_dimension,
            output_projection.output_dimension(),
        )?;
        validate_projection_input(
            "single-head block attention normalization",
            model_dimension,
            attention_norm.model_dimension(),
        )?;
        validate_projection_input(
            "single-head block feed-forward",
            model_dimension,
            feed_forward.input_dimension(),
        )?;
        validate_projection_input(
            "single-head block feed-forward normalization",
            model_dimension,
            feed_forward_norm.model_dimension(),
        )?;

        Ok(Self {
            model_dimension,
            query_projection,
            key_projection,
            value_projection,
            output_projection,
            attention_norm,
            feed_forward,
            feed_forward_norm,
        })
    }

    pub fn model_dimension(&self) -> ModelDimension {
        self.model_dimension
    }
}

/// Learned query, key, and value projections for one self-attention head.
#[derive(Debug, Clone, PartialEq)]
pub struct SelfAttentionHead {
    query_projection: HiddenToQuery,
    key_projection: HiddenToKey,
    value_projection: HiddenToValue,
}

impl SelfAttentionHead {
    pub fn new(
        query_projection: HiddenToQuery,
        key_projection: HiddenToKey,
        value_projection: HiddenToValue,
    ) -> CtResult<Self> {
        let input_dimension = query_projection.input_dimension();

        validate_projection_input(
            "self-attention head key projection",
            input_dimension,
            key_projection.input_dimension(),
        )?;
        validate_projection_input(
            "self-attention head value projection",
            input_dimension,
            value_projection.input_dimension(),
        )?;

        if query_projection.head_dimension() != key_projection.head_dimension() {
            return Err(CtError::ShapeMismatch {
                op: "self-attention head",
                expected: format!(
                    "query/key head dimension {}",
                    query_projection.head_dimension().value()
                ),
                got: format!(
                    "key head dimension {}",
                    key_projection.head_dimension().value()
                ),
            });
        }

        Ok(Self {
            query_projection,
            key_projection,
            value_projection,
        })
    }

    pub fn input_dimension(&self) -> ModelDimension {
        self.query_projection.input_dimension()
    }

    pub fn query_key_dimension(&self) -> HeadDimension {
        self.query_projection.head_dimension()
    }

    pub fn value_dimension(&self) -> HeadDimension {
        self.value_projection.head_dimension()
    }

    pub fn query_projection(&self) -> &HiddenToQuery {
        &self.query_projection
    }

    pub fn key_projection(&self) -> &HiddenToKey {
        &self.key_projection
    }

    pub fn value_projection(&self) -> &HiddenToValue {
        &self.value_projection
    }
}

/// A tiny multi-head Transformer block sketch.
#[derive(Debug, Clone, PartialEq)]
pub struct MultiHeadTransformerBlock {
    model_dimension: ModelDimension,
    head_count: HeadCount,
    value_dimension: HeadDimension,
    heads: Vec<SelfAttentionHead>,
    output_projection: AttentionOutputProjection,
    attention_norm: LayerNormalization,
    feed_forward: PositionWiseFeedForward,
    feed_forward_norm: LayerNormalization,
}

impl MultiHeadTransformerBlock {
    pub fn new(
        heads: Vec<SelfAttentionHead>,
        output_projection: AttentionOutputProjection,
        attention_norm: LayerNormalization,
        feed_forward: PositionWiseFeedForward,
        feed_forward_norm: LayerNormalization,
    ) -> CtResult<Self> {
        if heads.is_empty() {
            return Err(CtError::EmptyInput("multi-head block heads"));
        }

        let model_dimension = heads[0].input_dimension();
        let value_dimension = heads[0].value_dimension();

        for head in &heads {
            validate_projection_input(
                "multi-head block head projection",
                model_dimension,
                head.input_dimension(),
            )?;

            if head.value_dimension() != value_dimension {
                return Err(CtError::ShapeMismatch {
                    op: "multi-head block",
                    expected: format!("value head dimension {}", value_dimension.value()),
                    got: format!("value head dimension {}", head.value_dimension().value()),
                });
            }
        }

        let head_count = HeadCount::new(heads.len())?;
        let concatenated_dimension =
            ModelDimension::new(head_count.value() * value_dimension.value())?;

        validate_projection_input(
            "multi-head block output projection input",
            concatenated_dimension,
            output_projection.input_dimension(),
        )?;
        validate_projection_input(
            "multi-head block output projection",
            model_dimension,
            output_projection.output_dimension(),
        )?;
        validate_projection_input(
            "multi-head block attention normalization",
            model_dimension,
            attention_norm.model_dimension(),
        )?;
        validate_projection_input(
            "multi-head block feed-forward",
            model_dimension,
            feed_forward.input_dimension(),
        )?;
        validate_projection_input(
            "multi-head block feed-forward normalization",
            model_dimension,
            feed_forward_norm.model_dimension(),
        )?;

        Ok(Self {
            model_dimension,
            head_count,
            value_dimension,
            heads,
            output_projection,
            attention_norm,
            feed_forward,
            feed_forward_norm,
        })
    }

    pub fn model_dimension(&self) -> ModelDimension {
        self.model_dimension
    }

    pub fn head_count(&self) -> HeadCount {
        self.head_count
    }

    pub fn value_dimension(&self) -> HeadDimension {
        self.value_dimension
    }

    pub fn heads(&self) -> &[SelfAttentionHead] {
        &self.heads
    }

    fn with_heads(self, heads: Vec<SelfAttentionHead>) -> CtResult<Self> {
        Self::new(
            heads,
            self.output_projection,
            self.attention_norm,
            self.feed_forward,
            self.feed_forward_norm,
        )
    }

    fn with_feed_forward(self, feed_forward: PositionWiseFeedForward) -> CtResult<Self> {
        Self::new(
            self.heads,
            self.output_projection,
            self.attention_norm,
            feed_forward,
            self.feed_forward_norm,
        )
    }

    fn with_output_projection(
        self,
        output_projection: AttentionOutputProjection,
    ) -> CtResult<Self> {
        Self::new(
            self.heads,
            output_projection,
            self.attention_norm,
            self.feed_forward,
            self.feed_forward_norm,
        )
    }

    fn with_layer_norms(
        self,
        attention_norm: LayerNormalization,
        feed_forward_norm: LayerNormalization,
    ) -> CtResult<Self> {
        Self::new(
            self.heads,
            self.output_projection,
            attention_norm,
            self.feed_forward,
            feed_forward_norm,
        )
    }
}

/// A tiny masked multi-head Transformer block sketch.
#[derive(Debug, Clone, PartialEq)]
pub struct MaskedMultiHeadTransformerBlock {
    block: MultiHeadTransformerBlock,
}

impl MaskedMultiHeadTransformerBlock {
    pub fn new(
        heads: Vec<SelfAttentionHead>,
        output_projection: AttentionOutputProjection,
        attention_norm: LayerNormalization,
        feed_forward: PositionWiseFeedForward,
        feed_forward_norm: LayerNormalization,
    ) -> CtResult<Self> {
        Ok(Self {
            block: MultiHeadTransformerBlock::new(
                heads,
                output_projection,
                attention_norm,
                feed_forward,
                feed_forward_norm,
            )?,
        })
    }

    pub fn model_dimension(&self) -> ModelDimension {
        self.block.model_dimension()
    }

    pub fn head_count(&self) -> HeadCount {
        self.block.head_count()
    }

    pub fn value_dimension(&self) -> HeadDimension {
        self.block.value_dimension()
    }

    pub fn heads(&self) -> &[SelfAttentionHead] {
        self.block.heads()
    }

    pub fn feed_forward(&self) -> &PositionWiseFeedForward {
        &self.block.feed_forward
    }

    fn with_feed_forward(self, feed_forward: PositionWiseFeedForward) -> CtResult<Self> {
        Ok(Self {
            block: self.block.with_feed_forward(feed_forward)?,
        })
    }

    fn with_heads(self, heads: Vec<SelfAttentionHead>) -> CtResult<Self> {
        Ok(Self {
            block: self.block.with_heads(heads)?,
        })
    }

    fn with_output_projection(
        self,
        output_projection: AttentionOutputProjection,
    ) -> CtResult<Self> {
        Ok(Self {
            block: self.block.with_output_projection(output_projection)?,
        })
    }

    fn with_layer_norms(
        self,
        attention_norm: LayerNormalization,
        feed_forward_norm: LayerNormalization,
    ) -> CtResult<Self> {
        Ok(Self {
            block: self
                .block
                .with_layer_norms(attention_norm, feed_forward_norm)?,
        })
    }

    fn apply_with_training_cache(
        &self,
        hidden: HiddenSequence,
        mask: AttentionMask,
    ) -> CtResult<MaskedBlockTrainingCache> {
        if hidden.model_dimension() != self.block.model_dimension {
            return Err(CtError::ShapeMismatch {
                op: "masked multi-head block",
                expected: format!("model dimension {}", self.block.model_dimension.value()),
                got: format!("model dimension {}", hidden.model_dimension().value()),
            });
        }

        let head_caches = self
            .block
            .heads
            .iter()
            .map(|head| apply_self_attention_head_with_mask_cache(&hidden, head, Some(&mask)))
            .collect::<CtResult<Vec<_>>>()?;
        let attention_outputs = head_caches
            .iter()
            .map(|cache| cache.output.clone())
            .collect::<Vec<_>>();
        let head_outputs = AttentionHeadOutputs::new(attention_outputs)?;
        let multi_head_output = ConcatenateHeads.apply(head_outputs)?;
        let projected_attention = self
            .block
            .output_projection
            .apply(multi_head_output.clone())?;
        let with_attention = ResidualConnection.apply(Product::new(hidden, projected_attention))?;
        let normalized_attention = self.block.attention_norm.apply(with_attention.clone())?;
        let (feed_forward_output, feed_forward_rows) =
            feed_forward_with_cache(&self.block.feed_forward, &normalized_attention)?;
        let with_feed_forward =
            ResidualConnection.apply(Product::new(normalized_attention, feed_forward_output))?;
        let output = self
            .block
            .feed_forward_norm
            .apply(with_feed_forward.clone())?;

        Ok(MaskedBlockTrainingCache {
            output,
            with_feed_forward,
            with_attention,
            multi_head_output,
            attention_heads: head_caches,
            feed_forward_rows,
        })
    }
}

/// Tiny structured Transformer parameter object for the roadmap.
#[derive(Debug, Clone, PartialEq)]
pub struct TinyTransformerParameters {
    positional_encoding: PositionalEncoding,
    block: MaskedMultiHeadTransformerBlock,
    readout: TransformerReadout,
}

impl TinyTransformerParameters {
    pub fn new(
        positional_encoding: PositionalEncoding,
        block: MaskedMultiHeadTransformerBlock,
        readout: TransformerReadout,
    ) -> CtResult<Self> {
        let model_dimension = positional_encoding.model_dimension();

        validate_projection_input(
            "tiny transformer parameters block",
            model_dimension,
            block.model_dimension(),
        )?;
        validate_projection_input(
            "tiny transformer parameters readout",
            model_dimension,
            readout.input_dimension(),
        )?;

        Ok(Self {
            positional_encoding,
            block,
            readout,
        })
    }

    pub fn model_dimension(&self) -> ModelDimension {
        self.positional_encoding.model_dimension()
    }

    pub fn max_sequence_len(&self) -> SequenceLength {
        self.positional_encoding.max_sequence_len()
    }

    pub fn vocab_size(&self) -> VocabSize {
        self.readout.vocab_size()
    }

    pub fn encode(&self, hidden: HiddenSequence, mask: AttentionMask) -> CtResult<HiddenSequence> {
        let positioned = self.positional_encoding.apply(hidden)?;

        self.block.apply(Product::new(positioned, mask))
    }

    pub fn readout(&self) -> &TransformerReadout {
        &self.readout
    }

    pub fn feed_forward(&self) -> &PositionWiseFeedForward {
        self.block.feed_forward()
    }

    pub fn output_projection(&self) -> &AttentionOutputProjection {
        &self.block.block.output_projection
    }

    pub fn attention_heads(&self) -> &[SelfAttentionHead] {
        self.block.heads()
    }

    pub fn attention_norm(&self) -> &LayerNormalization {
        &self.block.block.attention_norm
    }

    pub fn feed_forward_norm(&self) -> &LayerNormalization {
        &self.block.block.feed_forward_norm
    }

    fn with_readout(self, readout: TransformerReadout) -> CtResult<Self> {
        Self::new(self.positional_encoding, self.block, readout)
    }

    fn with_feed_forward(self, feed_forward: PositionWiseFeedForward) -> CtResult<Self> {
        Self::new(
            self.positional_encoding,
            self.block.with_feed_forward(feed_forward)?,
            self.readout,
        )
    }

    fn with_attention_heads(self, heads: Vec<SelfAttentionHead>) -> CtResult<Self> {
        Self::new(
            self.positional_encoding,
            self.block.with_heads(heads)?,
            self.readout,
        )
    }

    fn with_output_projection(
        self,
        output_projection: AttentionOutputProjection,
    ) -> CtResult<Self> {
        Self::new(
            self.positional_encoding,
            self.block.with_output_projection(output_projection)?,
            self.readout,
        )
    }

    fn with_layer_norms(
        self,
        attention_norm: LayerNormalization,
        feed_forward_norm: LayerNormalization,
    ) -> CtResult<Self> {
        Self::new(
            self.positional_encoding,
            self.block
                .with_layer_norms(attention_norm, feed_forward_norm)?,
            self.readout,
        )
    }
}

/// Structured state owned by a future Transformer training loop.
#[derive(Debug, Clone, PartialEq)]
pub struct TransformerTrainingState {
    parameters: TinyTransformerParameters,
    learning_rate: LearningRate,
    step_count: StepCount,
}

impl TransformerTrainingState {
    pub fn new(parameters: TinyTransformerParameters, learning_rate: LearningRate) -> Self {
        Self::from_parts(parameters, learning_rate, StepCount::new(0))
    }

    pub fn from_parts(
        parameters: TinyTransformerParameters,
        learning_rate: LearningRate,
        step_count: StepCount,
    ) -> Self {
        Self {
            parameters,
            learning_rate,
            step_count,
        }
    }

    pub fn parameters(&self) -> &TinyTransformerParameters {
        &self.parameters
    }

    pub fn learning_rate(&self) -> LearningRate {
        self.learning_rate
    }

    pub fn step_count(&self) -> StepCount {
        self.step_count
    }

    pub fn record_updated_parameters(self, parameters: TinyTransformerParameters) -> Self {
        Self {
            parameters,
            learning_rate: self.learning_rate,
            step_count: StepCount::new(self.step_count.value() + 1),
        }
    }
}

/// One supervised sequence example for a readout-only Transformer update.
#[derive(Debug, Clone, PartialEq)]
pub struct TransformerReadoutTrainingExample {
    hidden: HiddenSequence,
    mask: AttentionMask,
    targets: TokenSequence,
}

impl TransformerReadoutTrainingExample {
    pub fn new(
        hidden: HiddenSequence,
        mask: AttentionMask,
        targets: TokenSequence,
    ) -> CtResult<Self> {
        let sequence_len = hidden.sequence_len();

        if targets.as_slice().len() != sequence_len.value() {
            return Err(CtError::ShapeMismatch {
                op: "transformer readout training targets",
                expected: format!("{} target tokens", sequence_len.value()),
                got: format!("{} target tokens", targets.as_slice().len()),
            });
        }

        if mask.query_len() != sequence_len || mask.key_len() != sequence_len {
            return Err(CtError::ShapeMismatch {
                op: "transformer readout training mask",
                expected: format!(
                    "{} query rows x {} key columns",
                    sequence_len.value(),
                    sequence_len.value()
                ),
                got: format!(
                    "{} query rows x {} key columns",
                    mask.query_len().value(),
                    mask.key_len().value()
                ),
            });
        }

        Ok(Self {
            hidden,
            mask,
            targets,
        })
    }

    pub fn hidden(&self) -> &HiddenSequence {
        &self.hidden
    }

    pub fn mask(&self) -> &AttentionMask {
        &self.mask
    }

    pub fn targets(&self) -> &TokenSequence {
        &self.targets
    }
}

/// Non-empty set of supervised readout examples.
#[derive(Debug, Clone, PartialEq)]
pub struct TransformerReadoutTrainingSet(Vec<TransformerReadoutTrainingExample>);

impl TransformerReadoutTrainingSet {
    pub fn new(
        examples: impl IntoIterator<Item = TransformerReadoutTrainingExample>,
    ) -> CtResult<Self> {
        let examples = examples.into_iter().collect::<Vec<_>>();

        if examples.is_empty() {
            return Err(CtError::EmptyInput("transformer readout training set"));
        }

        Ok(Self(examples))
    }

    pub fn examples(&self) -> &[TransformerReadoutTrainingExample] {
        &self.0
    }
}

/// One full-batch update of the sequence readout parameters.
#[derive(Debug, Clone, PartialEq)]
pub struct TransformerReadoutTrainStep {
    dataset: TransformerReadoutTrainingSet,
}

impl TransformerReadoutTrainStep {
    pub fn new(dataset: TransformerReadoutTrainingSet) -> Self {
        Self { dataset }
    }
}

impl Morphism<TransformerTrainingState, TransformerTrainingState> for TransformerReadoutTrainStep {
    fn name(&self) -> &'static str {
        "transformer_readout_train_step"
    }

    fn apply(&self, state: TransformerTrainingState) -> CtResult<TransformerTrainingState> {
        let input_dimension = state.parameters.readout().input_dimension().value();
        let vocab_size = state.parameters.vocab_size().value();
        let mut grad_weight = vec![vec![0.0; vocab_size]; input_dimension];
        let mut grad_bias = vec![0.0; vocab_size];
        let mut position_count = 0usize;

        for example in self.dataset.examples() {
            let encoded = state
                .parameters
                .encode(example.hidden().clone(), example.mask().clone())?;
            let logits = state.parameters.readout().apply(encoded.clone())?;

            for ((hidden_row, logit_row), target) in encoded
                .rows()
                .iter()
                .zip(logits.rows())
                .zip(example.targets().as_slice())
            {
                let target_index = target.index();

                if target_index >= vocab_size {
                    return Err(CtError::OutOfRange {
                        kind: "sequence target",
                        index: target_index,
                        limit: vocab_size,
                    });
                }

                let probabilities = Softmax.apply(logit_row.clone())?;
                let mut dlogits = probabilities.as_slice().to_vec();
                dlogits[target_index] -= 1.0;

                for (vocab_id, dlogit) in dlogits.iter().copied().enumerate() {
                    grad_bias[vocab_id] += dlogit;

                    for (feature, hidden_value) in hidden_row.as_slice().iter().copied().enumerate()
                    {
                        grad_weight[feature][vocab_id] += hidden_value * dlogit;
                    }
                }

                position_count += 1;
            }
        }

        let scale = state.learning_rate().value() / position_count as f32;
        let mut updated_weight = state.parameters.readout().weight().to_vec();
        let mut updated_bias = state.parameters.readout().bias().to_vec();

        for (row, grad_row) in updated_weight.iter_mut().zip(&grad_weight) {
            for (weight, grad) in row.iter_mut().zip(grad_row) {
                *weight -= scale * grad;
            }
        }

        for (bias, grad) in updated_bias.iter_mut().zip(&grad_bias) {
            *bias -= scale * grad;
        }

        let updated_readout = TransformerReadout::new(updated_weight, updated_bias)?;
        let updated_parameters = state.parameters.clone().with_readout(updated_readout)?;

        Ok(state.record_updated_parameters(updated_parameters))
    }
}

/// One supervised sequence example for local feed-forward sublayer training.
#[derive(Debug, Clone, PartialEq)]
pub struct TransformerFeedForwardTrainingExample {
    input: HiddenSequence,
    target: HiddenSequence,
}

impl TransformerFeedForwardTrainingExample {
    pub fn new(input: HiddenSequence, target: HiddenSequence) -> CtResult<Self> {
        if input.sequence_len() != target.sequence_len() {
            return Err(CtError::ShapeMismatch {
                op: "transformer feed-forward training sequence",
                expected: format!("{} target rows", input.sequence_len().value()),
                got: format!("{} target rows", target.sequence_len().value()),
            });
        }

        if input.model_dimension() != target.model_dimension() {
            return Err(CtError::ShapeMismatch {
                op: "transformer feed-forward training dimension",
                expected: format!("target dimension {}", input.model_dimension().value()),
                got: format!("target dimension {}", target.model_dimension().value()),
            });
        }

        Ok(Self { input, target })
    }

    pub fn input(&self) -> &HiddenSequence {
        &self.input
    }

    pub fn target(&self) -> &HiddenSequence {
        &self.target
    }
}

/// Non-empty set of local feed-forward training examples.
#[derive(Debug, Clone, PartialEq)]
pub struct TransformerFeedForwardTrainingSet(Vec<TransformerFeedForwardTrainingExample>);

impl TransformerFeedForwardTrainingSet {
    pub fn new(
        examples: impl IntoIterator<Item = TransformerFeedForwardTrainingExample>,
    ) -> CtResult<Self> {
        let examples = examples.into_iter().collect::<Vec<_>>();

        if examples.is_empty() {
            return Err(CtError::EmptyInput("transformer feed-forward training set"));
        }

        Ok(Self(examples))
    }

    pub fn examples(&self) -> &[TransformerFeedForwardTrainingExample] {
        &self.0
    }
}

/// One full-batch update of the position-wise feed-forward sublayer.
#[derive(Debug, Clone, PartialEq)]
pub struct TransformerFeedForwardTrainStep {
    dataset: TransformerFeedForwardTrainingSet,
}

impl TransformerFeedForwardTrainStep {
    pub fn new(dataset: TransformerFeedForwardTrainingSet) -> Self {
        Self { dataset }
    }
}

impl Morphism<TransformerTrainingState, TransformerTrainingState>
    for TransformerFeedForwardTrainStep
{
    fn name(&self) -> &'static str {
        "transformer_feed_forward_train_step"
    }

    fn apply(&self, state: TransformerTrainingState) -> CtResult<TransformerTrainingState> {
        let feed_forward = state.parameters.feed_forward();
        let mut gradients = FeedForwardGradients::new(feed_forward);
        let mut row_count = 0usize;

        for example in self.dataset.examples() {
            if example.input().model_dimension() != feed_forward.input_dimension() {
                return Err(CtError::ShapeMismatch {
                    op: "transformer feed-forward train step",
                    expected: format!("input dimension {}", feed_forward.input_dimension().value()),
                    got: format!(
                        "input dimension {}",
                        example.input().model_dimension().value()
                    ),
                });
            }

            let (_output, cache_rows) = feed_forward_with_cache(feed_forward, example.input())?;

            for (cache, target_row) in cache_rows.iter().zip(example.target().rows()) {
                let d_output = cache
                    .output
                    .iter()
                    .zip(target_row.as_slice())
                    .map(|(output_value, target_value)| output_value - target_value)
                    .collect::<Vec<_>>();
                gradients.accumulate(feed_forward, cache, &d_output);

                row_count += 1;
            }
        }

        let updated_feed_forward =
            gradients.apply_to(feed_forward, state.learning_rate(), row_count)?;
        let updated_parameters = state
            .parameters
            .clone()
            .with_feed_forward(updated_feed_forward)?;

        Ok(state.record_updated_parameters(updated_parameters))
    }
}

/// One supervised sequence example for a composed block-level update.
#[derive(Debug, Clone, PartialEq)]
pub struct TransformerBlockTrainingExample {
    hidden: HiddenSequence,
    mask: AttentionMask,
    targets: TokenSequence,
}

impl TransformerBlockTrainingExample {
    pub fn new(
        hidden: HiddenSequence,
        mask: AttentionMask,
        targets: TokenSequence,
    ) -> CtResult<Self> {
        let sequence_len = hidden.sequence_len();

        if targets.as_slice().len() != sequence_len.value() {
            return Err(CtError::ShapeMismatch {
                op: "transformer block training targets",
                expected: format!("{} target tokens", sequence_len.value()),
                got: format!("{} target tokens", targets.as_slice().len()),
            });
        }

        if mask.query_len() != sequence_len || mask.key_len() != sequence_len {
            return Err(CtError::ShapeMismatch {
                op: "transformer block training mask",
                expected: format!(
                    "{} query rows x {} key columns",
                    sequence_len.value(),
                    sequence_len.value()
                ),
                got: format!(
                    "{} query rows x {} key columns",
                    mask.query_len().value(),
                    mask.key_len().value()
                ),
            });
        }

        Ok(Self {
            hidden,
            mask,
            targets,
        })
    }

    pub fn hidden(&self) -> &HiddenSequence {
        &self.hidden
    }

    pub fn mask(&self) -> &AttentionMask {
        &self.mask
    }

    pub fn targets(&self) -> &TokenSequence {
        &self.targets
    }
}

/// Non-empty set of supervised block-level training examples.
#[derive(Debug, Clone, PartialEq)]
pub struct TransformerBlockTrainingSet(Vec<TransformerBlockTrainingExample>);

impl TransformerBlockTrainingSet {
    pub fn new(
        examples: impl IntoIterator<Item = TransformerBlockTrainingExample>,
    ) -> CtResult<Self> {
        let examples = examples.into_iter().collect::<Vec<_>>();

        if examples.is_empty() {
            return Err(CtError::EmptyInput("transformer block training set"));
        }

        Ok(Self(examples))
    }

    pub fn examples(&self) -> &[TransformerBlockTrainingExample] {
        &self.0
    }
}

/// One full-batch update through the readout, block sublayers, and attention heads.
#[derive(Debug, Clone, PartialEq)]
pub struct TransformerBlockTrainStep {
    dataset: TransformerBlockTrainingSet,
}

impl TransformerBlockTrainStep {
    pub fn new(dataset: TransformerBlockTrainingSet) -> Self {
        Self { dataset }
    }
}

impl Morphism<TransformerTrainingState, TransformerTrainingState> for TransformerBlockTrainStep {
    fn name(&self) -> &'static str {
        "transformer_block_train_step"
    }

    fn apply(&self, state: TransformerTrainingState) -> CtResult<TransformerTrainingState> {
        let readout = state.parameters.readout();
        let feed_forward = state.parameters.feed_forward();
        let output_projection = state.parameters.output_projection();
        let attention_norm = state.parameters.attention_norm();
        let feed_forward_norm = state.parameters.feed_forward_norm();
        let mut readout_gradients = ReadoutGradients::new(readout);
        let mut feed_forward_gradients = FeedForwardGradients::new(feed_forward);
        let mut output_projection_gradients =
            AttentionOutputProjectionGradients::new(output_projection);
        let mut attention_norm_gradients = LayerNormGradients::new(attention_norm.parameters());
        let mut feed_forward_norm_gradients =
            LayerNormGradients::new(feed_forward_norm.parameters());
        let mut attention_head_gradients = state
            .parameters
            .attention_heads()
            .iter()
            .map(AttentionHeadGradients::new)
            .collect::<Vec<_>>();
        let mut position_count = 0usize;

        for example in self.dataset.examples() {
            let positioned = state
                .parameters
                .positional_encoding
                .apply(example.hidden().clone())?;
            let block_cache = state
                .parameters
                .block
                .apply_with_training_cache(positioned.clone(), example.mask().clone())?;
            let logits = readout.apply(block_cache.output.clone())?;
            let vocab_size = logits.vocab_size().value();
            let mut d_multi_head_rows = vec![
                vec![0.0; output_projection.input_dimension().value()];
                block_cache.multi_head_output.sequence_len().value()
            ];

            for (
                position,
                (
                    (((encoded_row, logit_row), with_feed_forward_row), with_attention_row),
                    ((feed_forward_cache, multi_head_row), target),
                ),
            ) in block_cache
                .output
                .rows()
                .iter()
                .zip(logits.rows())
                .zip(block_cache.with_feed_forward.rows())
                .zip(block_cache.with_attention.rows())
                .zip(
                    block_cache
                        .feed_forward_rows
                        .iter()
                        .zip(block_cache.multi_head_output.rows())
                        .zip(example.targets().as_slice()),
                )
                .enumerate()
            {
                let dlogits =
                    softmax_cross_entropy_logits_gradient(logit_row, target.index(), vocab_size)?;
                let d_encoded =
                    readout_gradients.accumulate(readout, encoded_row.as_slice(), &dlogits);
                let d_with_feed_forward = feed_forward_norm_gradients.accumulate(
                    &d_encoded,
                    with_feed_forward_row.as_slice(),
                    feed_forward_norm.parameters(),
                );

                let d_feed_forward_input = feed_forward_gradients.accumulate(
                    feed_forward,
                    feed_forward_cache,
                    &d_with_feed_forward,
                );
                let d_normalized_attention = add_rows(&d_with_feed_forward, &d_feed_forward_input);
                let d_with_attention = attention_norm_gradients.accumulate(
                    &d_normalized_attention,
                    with_attention_row.as_slice(),
                    attention_norm.parameters(),
                );
                d_multi_head_rows[position] = output_projection_gradients.accumulate(
                    output_projection,
                    multi_head_row.as_slice(),
                    &d_with_attention,
                );
                position_count += 1;
            }

            let value_dimension = block_cache.multi_head_output.head_dimension().value();
            for (head_index, ((head_gradient, head), head_cache)) in attention_head_gradients
                .iter_mut()
                .zip(state.parameters.attention_heads())
                .zip(&block_cache.attention_heads)
                .enumerate()
            {
                let start = head_index * value_dimension;
                let end = start + value_dimension;
                let d_head_output_rows = d_multi_head_rows
                    .iter()
                    .map(|row| row[start..end].to_vec())
                    .collect::<Vec<_>>();

                head_gradient.accumulate(
                    head,
                    &positioned,
                    head_cache,
                    example.mask(),
                    &d_head_output_rows,
                )?;
            }
        }

        let updated_readout =
            readout_gradients.apply_to(readout, state.learning_rate(), position_count)?;
        let updated_feed_forward =
            feed_forward_gradients.apply_to(feed_forward, state.learning_rate(), position_count)?;
        let updated_output_projection = output_projection_gradients.apply_to(
            output_projection,
            state.learning_rate(),
            position_count,
        )?;
        let updated_attention_norm = LayerNormalization::new(attention_norm_gradients.apply_to(
            attention_norm.parameters(),
            state.learning_rate(),
            position_count,
        )?);
        let updated_feed_forward_norm =
            LayerNormalization::new(feed_forward_norm_gradients.apply_to(
                feed_forward_norm.parameters(),
                state.learning_rate(),
                position_count,
            )?);
        let updated_heads = state
            .parameters
            .attention_heads()
            .iter()
            .zip(attention_head_gradients)
            .map(|(head, gradients)| {
                gradients.apply_to(head, state.learning_rate(), position_count)
            })
            .collect::<CtResult<Vec<_>>>()?;
        let updated_parameters = state
            .parameters
            .clone()
            .with_readout(updated_readout)?
            .with_feed_forward(updated_feed_forward)?
            .with_output_projection(updated_output_projection)?
            .with_layer_norms(updated_attention_norm, updated_feed_forward_norm)?
            .with_attention_heads(updated_heads)?;

        Ok(state.record_updated_parameters(updated_parameters))
    }
}

/// Average sequence cross-entropy for the structured Transformer readout.
pub fn transformer_readout_average_loss(
    state: &TransformerTrainingState,
    dataset: &TransformerReadoutTrainingSet,
) -> CtResult<Loss> {
    let mut total = 0.0;
    let mut position_count = 0usize;

    for example in dataset.examples() {
        let logits = state.apply(Product::new(
            example.hidden().clone(),
            example.mask().clone(),
        ))?;
        let vocab_size = logits.vocab_size().value();

        for (logit_row, target) in logits.rows().iter().zip(example.targets().as_slice()) {
            let target_index = target.index();

            if target_index >= vocab_size {
                return Err(CtError::OutOfRange {
                    kind: "sequence target",
                    index: target_index,
                    limit: vocab_size,
                });
            }

            let probabilities = Softmax.apply(logit_row.clone())?;
            let probability = probabilities.as_slice()[target_index].max(1e-9);
            total += -probability.ln();
            position_count += 1;
        }
    }

    Loss::new(total / position_count as f32)
}

/// Average squared error for local feed-forward sublayer training.
pub fn transformer_feed_forward_average_loss(
    state: &TransformerTrainingState,
    dataset: &TransformerFeedForwardTrainingSet,
) -> CtResult<Loss> {
    let feed_forward = state.parameters.feed_forward();
    let mut total = 0.0;
    let mut value_count = 0usize;

    for example in dataset.examples() {
        let output = feed_forward.apply(example.input().clone())?;

        for (output_row, target_row) in output.rows().iter().zip(example.target().rows()) {
            for (output_value, target_value) in
                output_row.as_slice().iter().zip(target_row.as_slice())
            {
                let error = output_value - target_value;
                total += 0.5 * error * error;
                value_count += 1;
            }
        }
    }

    Loss::new(total / value_count as f32)
}

/// Average sequence cross-entropy for the composed block-level training set.
pub fn transformer_block_average_loss(
    state: &TransformerTrainingState,
    dataset: &TransformerBlockTrainingSet,
) -> CtResult<Loss> {
    let mut total = 0.0;
    let mut position_count = 0usize;

    for example in dataset.examples() {
        let logits = state.apply(Product::new(
            example.hidden().clone(),
            example.mask().clone(),
        ))?;
        let vocab_size = logits.vocab_size().value();

        for (logit_row, target) in logits.rows().iter().zip(example.targets().as_slice()) {
            let target_index = target.index();

            if target_index >= vocab_size {
                return Err(CtError::OutOfRange {
                    kind: "sequence target",
                    index: target_index,
                    limit: vocab_size,
                });
            }

            let probabilities = Softmax.apply(logit_row.clone())?;
            let probability = probabilities.as_slice()[target_index].max(1e-9);
            total += -probability.ln();
            position_count += 1;
        }
    }

    Loss::new(total / position_count as f32)
}

/// Applies softmax independently to each query row.
#[derive(Debug, Clone)]
pub struct AttentionSoftmax;

impl Morphism<AttentionScores, AttentionWeights> for AttentionSoftmax {
    fn name(&self) -> &'static str {
        "attention_softmax"
    }

    fn apply(&self, scores: AttentionScores) -> CtResult<AttentionWeights> {
        let rows = scores
            .rows
            .into_iter()
            .map(|row| Softmax.apply(row))
            .collect::<CtResult<Vec<_>>>()?;

        AttentionWeights::new(rows)
    }
}

/// Mixes value vectors with row-wise attention weights.
#[derive(Debug, Clone)]
pub struct WeightedValueMixing;

impl Morphism<Product<AttentionWeights, ValueSequence>, AttentionOutput> for WeightedValueMixing {
    fn name(&self) -> &'static str {
        "weighted_value_mixing"
    }

    fn apply(&self, input: Product<AttentionWeights, ValueSequence>) -> CtResult<AttentionOutput> {
        let (weights, values) = input.into_parts();

        if weights.key_len() != values.sequence_len() {
            return Err(CtError::ShapeMismatch {
                op: "weighted value mixing",
                expected: format!("{} value rows", weights.key_len().value()),
                got: format!("{} value rows", values.sequence_len().value()),
            });
        }

        let value_width = values.head_dimension().value();
        let rows = weights
            .rows()
            .iter()
            .map(|weight_row| weighted_sum(weight_row.as_slice(), values.rows(), value_width))
            .collect::<Vec<_>>();

        AttentionOutput::new(rows)
    }
}

fn weighted_sum(weights: &[f32], values: &[Vector], value_width: usize) -> Vec<f32> {
    let mut output = vec![0.0; value_width];

    for (weight, value) in weights.iter().zip(values.iter()) {
        for (output_value, value_component) in output.iter_mut().zip(value.as_slice()) {
            *output_value += weight * value_component;
        }
    }

    output
}

/// Concatenates single-head outputs along the feature dimension.
#[derive(Debug, Clone)]
pub struct ConcatenateHeads;

impl Morphism<AttentionHeadOutputs, MultiHeadOutput> for ConcatenateHeads {
    fn name(&self) -> &'static str {
        "concatenate_heads"
    }

    fn apply(&self, heads: AttentionHeadOutputs) -> CtResult<MultiHeadOutput> {
        let rows = (0..heads.sequence_len().value())
            .map(|position| {
                heads
                    .heads()
                    .iter()
                    .flat_map(|head| head.rows()[position].as_slice().iter().copied())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        MultiHeadOutput::new(rows, heads.head_count(), heads.head_dimension())
    }
}

impl Morphism<MultiHeadOutput, ProjectedAttentionOutput> for AttentionOutputProjection {
    fn name(&self) -> &'static str {
        "attention_output_projection"
    }

    fn apply(&self, input: MultiHeadOutput) -> CtResult<ProjectedAttentionOutput> {
        if input.model_dimension() != self.input_dimension {
            return Err(CtError::ShapeMismatch {
                op: "attention output projection",
                expected: format!("input dimension {}", self.input_dimension.value()),
                got: format!("input dimension {}", input.model_dimension().value()),
            });
        }

        let rows = input
            .rows()
            .iter()
            .map(|row| project_row(row.as_slice(), &self.weight, &self.bias))
            .collect::<Vec<_>>();

        ProjectedAttentionOutput::new(rows)
    }
}

/// Adds a same-shaped sublayer output back to the hidden sequence.
#[derive(Debug, Clone)]
pub struct ResidualConnection;

impl Morphism<Product<HiddenSequence, ProjectedAttentionOutput>, HiddenSequence>
    for ResidualConnection
{
    fn name(&self) -> &'static str {
        "residual_connection"
    }

    fn apply(
        &self,
        input: Product<HiddenSequence, ProjectedAttentionOutput>,
    ) -> CtResult<HiddenSequence> {
        let (hidden, sublayer_output) = input.into_parts();

        if hidden.sequence_len() != sublayer_output.sequence_len() {
            return Err(CtError::ShapeMismatch {
                op: "residual connection",
                expected: format!("{} sequence rows", hidden.sequence_len().value()),
                got: format!("{} sequence rows", sublayer_output.sequence_len().value()),
            });
        }

        if hidden.model_dimension() != sublayer_output.model_dimension() {
            return Err(CtError::ShapeMismatch {
                op: "residual connection",
                expected: format!("model dimension {}", hidden.model_dimension().value()),
                got: format!(
                    "model dimension {}",
                    sublayer_output.model_dimension().value()
                ),
            });
        }

        let rows = hidden
            .rows()
            .iter()
            .zip(sublayer_output.rows())
            .map(|(left, right)| add_rows(left.as_slice(), right.as_slice()))
            .collect::<Vec<_>>();

        HiddenSequence::new(rows)
    }
}

impl Morphism<HiddenSequence, HiddenSequence> for LayerNormalization {
    fn name(&self) -> &'static str {
        "layer_normalization"
    }

    fn apply(&self, input: HiddenSequence) -> CtResult<HiddenSequence> {
        if input.model_dimension() != self.parameters.model_dimension() {
            return Err(CtError::ShapeMismatch {
                op: "layer normalization",
                expected: format!(
                    "model dimension {}",
                    self.parameters.model_dimension().value()
                ),
                got: format!("model dimension {}", input.model_dimension().value()),
            });
        }

        let rows = input
            .rows()
            .iter()
            .map(|row| normalize_row(row.as_slice(), &self.parameters))
            .collect::<Vec<_>>();

        HiddenSequence::new(rows)
    }
}

impl Morphism<HiddenSequence, HiddenSequence> for PositionWiseFeedForward {
    fn name(&self) -> &'static str {
        "position_wise_feed_forward"
    }

    fn apply(&self, input: HiddenSequence) -> CtResult<HiddenSequence> {
        let (output, _cache) = feed_forward_with_cache(self, &input)?;

        Ok(output)
    }
}

impl Morphism<HiddenSequence, HiddenSequence> for PositionalEncoding {
    fn name(&self) -> &'static str {
        "positional_encoding"
    }

    fn apply(&self, input: HiddenSequence) -> CtResult<HiddenSequence> {
        if input.sequence_len().value() > self.max_sequence_len.value() {
            return Err(CtError::ShapeMismatch {
                op: "positional encoding",
                expected: format!("at most {} sequence rows", self.max_sequence_len.value()),
                got: format!("{} sequence rows", input.sequence_len().value()),
            });
        }

        if input.model_dimension() != self.model_dimension {
            return Err(CtError::ShapeMismatch {
                op: "positional encoding",
                expected: format!("model dimension {}", self.model_dimension.value()),
                got: format!("model dimension {}", input.model_dimension().value()),
            });
        }

        let rows = input
            .rows()
            .iter()
            .zip(&self.rows)
            .map(|(hidden_row, position_row)| {
                add_rows(hidden_row.as_slice(), position_row.as_slice())
            })
            .collect::<Vec<_>>();

        HiddenSequence::new(rows)
    }
}

impl Morphism<HiddenSequence, SequenceLogits> for TransformerReadout {
    fn name(&self) -> &'static str {
        "transformer_readout"
    }

    fn apply(&self, input: HiddenSequence) -> CtResult<SequenceLogits> {
        if input.model_dimension() != self.input_dimension {
            return Err(CtError::ShapeMismatch {
                op: "transformer readout",
                expected: format!("input dimension {}", self.input_dimension.value()),
                got: format!("input dimension {}", input.model_dimension().value()),
            });
        }

        let rows = input
            .rows()
            .iter()
            .map(|row| project_row(row.as_slice(), &self.weight, &self.bias))
            .collect::<Vec<_>>();

        SequenceLogits::new(rows)
    }
}

impl Morphism<HiddenSequence, QuerySequence> for HiddenToQuery {
    fn name(&self) -> &'static str {
        "hidden_to_query"
    }

    fn apply(&self, input: HiddenSequence) -> CtResult<QuerySequence> {
        QuerySequence::new(self.projection.project(&input)?)
    }
}

impl Morphism<HiddenSequence, KeySequence> for HiddenToKey {
    fn name(&self) -> &'static str {
        "hidden_to_key"
    }

    fn apply(&self, input: HiddenSequence) -> CtResult<KeySequence> {
        KeySequence::new(self.projection.project(&input)?)
    }
}

impl Morphism<HiddenSequence, ValueSequence> for HiddenToValue {
    fn name(&self) -> &'static str {
        "hidden_to_value"
    }

    fn apply(&self, input: HiddenSequence) -> CtResult<ValueSequence> {
        ValueSequence::new(self.projection.project(&input)?)
    }
}

fn apply_self_attention_head(
    input: &HiddenSequence,
    head: &SelfAttentionHead,
) -> CtResult<AttentionOutput> {
    apply_self_attention_head_with_mask(input, head, None)
}

fn apply_self_attention_head_with_mask(
    input: &HiddenSequence,
    head: &SelfAttentionHead,
    mask: Option<&AttentionMask>,
) -> CtResult<AttentionOutput> {
    Ok(apply_self_attention_head_with_mask_cache(input, head, mask)?.output)
}

fn apply_self_attention_head_with_mask_cache(
    input: &HiddenSequence,
    head: &SelfAttentionHead,
    mask: Option<&AttentionMask>,
) -> CtResult<AttentionHeadTrainingCache> {
    let queries = head.query_projection.apply(input.clone())?;
    let keys = head.key_projection.apply(input.clone())?;
    let values = head.value_projection.apply(input.clone())?;
    let scores = ScaledDotProductScores.apply(Product::new(queries.clone(), keys.clone()))?;
    let scores = if let Some(mask) = mask {
        MaskedAttentionScores.apply(Product::new(scores, mask.clone()))?
    } else {
        scores
    };
    let weights = AttentionSoftmax.apply(scores)?;
    let output = WeightedValueMixing.apply(Product::new(weights.clone(), values.clone()))?;

    Ok(AttentionHeadTrainingCache {
        queries,
        keys,
        values,
        weights,
        output,
    })
}

impl Morphism<Product<HiddenSequence, HiddenSequence>, HiddenSequence> for ResidualConnection {
    fn name(&self) -> &'static str {
        "hidden_residual_connection"
    }

    fn apply(&self, input: Product<HiddenSequence, HiddenSequence>) -> CtResult<HiddenSequence> {
        let (left, right) = input.into_parts();

        if left.sequence_len() != right.sequence_len() {
            return Err(CtError::ShapeMismatch {
                op: "hidden residual connection",
                expected: format!("{} sequence rows", left.sequence_len().value()),
                got: format!("{} sequence rows", right.sequence_len().value()),
            });
        }

        if left.model_dimension() != right.model_dimension() {
            return Err(CtError::ShapeMismatch {
                op: "hidden residual connection",
                expected: format!("model dimension {}", left.model_dimension().value()),
                got: format!("model dimension {}", right.model_dimension().value()),
            });
        }

        let rows = left
            .rows()
            .iter()
            .zip(right.rows())
            .map(|(left, right)| add_rows(left.as_slice(), right.as_slice()))
            .collect::<Vec<_>>();

        HiddenSequence::new(rows)
    }
}

impl Morphism<HiddenSequence, HiddenSequence> for SingleHeadTransformerBlock {
    fn name(&self) -> &'static str {
        "single_head_transformer_block"
    }

    fn apply(&self, input: HiddenSequence) -> CtResult<HiddenSequence> {
        if input.model_dimension() != self.model_dimension {
            return Err(CtError::ShapeMismatch {
                op: "single-head block",
                expected: format!("model dimension {}", self.model_dimension.value()),
                got: format!("model dimension {}", input.model_dimension().value()),
            });
        }

        let head = SelfAttentionHead::new(
            self.query_projection.clone(),
            self.key_projection.clone(),
            self.value_projection.clone(),
        )?;
        let attention_output = apply_self_attention_head(&input, &head)?;
        let head_outputs = AttentionHeadOutputs::new(vec![attention_output])?;
        let multi_head_output = ConcatenateHeads.apply(head_outputs)?;
        let projected_attention = self.output_projection.apply(multi_head_output)?;
        let with_attention = ResidualConnection.apply(Product::new(input, projected_attention))?;
        let normalized_attention = self.attention_norm.apply(with_attention)?;
        let feed_forward_output = self.feed_forward.apply(normalized_attention.clone())?;
        let with_feed_forward =
            ResidualConnection.apply(Product::new(normalized_attention, feed_forward_output))?;

        self.feed_forward_norm.apply(with_feed_forward)
    }
}

impl Morphism<HiddenSequence, HiddenSequence> for MultiHeadTransformerBlock {
    fn name(&self) -> &'static str {
        "multi_head_transformer_block"
    }

    fn apply(&self, input: HiddenSequence) -> CtResult<HiddenSequence> {
        if input.model_dimension() != self.model_dimension {
            return Err(CtError::ShapeMismatch {
                op: "multi-head block",
                expected: format!("model dimension {}", self.model_dimension.value()),
                got: format!("model dimension {}", input.model_dimension().value()),
            });
        }

        let attention_outputs = self
            .heads
            .iter()
            .map(|head| apply_self_attention_head(&input, head))
            .collect::<CtResult<Vec<_>>>()?;
        let head_outputs = AttentionHeadOutputs::new(attention_outputs)?;
        let multi_head_output = ConcatenateHeads.apply(head_outputs)?;
        let projected_attention = self.output_projection.apply(multi_head_output)?;
        let with_attention = ResidualConnection.apply(Product::new(input, projected_attention))?;
        let normalized_attention = self.attention_norm.apply(with_attention)?;
        let feed_forward_output = self.feed_forward.apply(normalized_attention.clone())?;
        let with_feed_forward =
            ResidualConnection.apply(Product::new(normalized_attention, feed_forward_output))?;

        self.feed_forward_norm.apply(with_feed_forward)
    }
}

impl Morphism<Product<HiddenSequence, AttentionMask>, HiddenSequence>
    for MaskedMultiHeadTransformerBlock
{
    fn name(&self) -> &'static str {
        "masked_multi_head_transformer_block"
    }

    fn apply(&self, input: Product<HiddenSequence, AttentionMask>) -> CtResult<HiddenSequence> {
        let (hidden, mask) = input.into_parts();

        Ok(self.apply_with_training_cache(hidden, mask)?.output)
    }
}

impl Morphism<Product<HiddenSequence, AttentionMask>, SequenceLogits>
    for TinyTransformerParameters
{
    fn name(&self) -> &'static str {
        "tiny_transformer_parameters"
    }

    fn apply(&self, input: Product<HiddenSequence, AttentionMask>) -> CtResult<SequenceLogits> {
        let (hidden, mask) = input.into_parts();
        let positioned = self.positional_encoding.apply(hidden)?;
        let encoded = self.block.apply(Product::new(positioned, mask))?;

        self.readout.apply(encoded)
    }
}

impl Morphism<Product<HiddenSequence, AttentionMask>, SequenceLogits> for TransformerTrainingState {
    fn name(&self) -> &'static str {
        "transformer_training_state_forward"
    }

    fn apply(&self, input: Product<HiddenSequence, AttentionMask>) -> CtResult<SequenceLogits> {
        self.parameters.apply(input)
    }
}

#[derive(Debug, Clone, PartialEq)]
struct ReadoutGradients {
    weight: Vec<Vec<f32>>,
    bias: Vec<f32>,
}

impl ReadoutGradients {
    fn new(readout: &TransformerReadout) -> Self {
        Self {
            weight: vec![
                vec![0.0; readout.vocab_size().value()];
                readout.input_dimension().value()
            ],
            bias: vec![0.0; readout.vocab_size().value()],
        }
    }

    fn accumulate(
        &mut self,
        readout: &TransformerReadout,
        hidden_row: &[f32],
        dlogits: &[f32],
    ) -> Vec<f32> {
        let mut d_hidden = vec![0.0; readout.input_dimension().value()];

        for (vocab_id, dlogit) in dlogits.iter().copied().enumerate() {
            self.bias[vocab_id] += dlogit;

            for (feature, hidden_value) in hidden_row.iter().copied().enumerate() {
                self.weight[feature][vocab_id] += hidden_value * dlogit;
                d_hidden[feature] += readout.weight()[feature][vocab_id] * dlogit;
            }
        }

        d_hidden
    }

    fn apply_to(
        self,
        readout: &TransformerReadout,
        learning_rate: LearningRate,
        position_count: usize,
    ) -> CtResult<TransformerReadout> {
        if position_count == 0 {
            return Err(CtError::EmptyInput("readout gradient positions"));
        }

        let scale = learning_rate.value() / position_count as f32;
        let mut updated_weight = readout.weight().to_vec();
        let mut updated_bias = readout.bias().to_vec();

        for (row, grad_row) in updated_weight.iter_mut().zip(&self.weight) {
            for (weight, grad) in row.iter_mut().zip(grad_row) {
                *weight -= scale * grad;
            }
        }

        for (bias, grad) in updated_bias.iter_mut().zip(&self.bias) {
            *bias -= scale * grad;
        }

        TransformerReadout::new(updated_weight, updated_bias)
    }
}

#[derive(Debug, Clone, PartialEq)]
struct FeedForwardGradients {
    first_weight: Vec<Vec<f32>>,
    first_bias: Vec<f32>,
    second_weight: Vec<Vec<f32>>,
    second_bias: Vec<f32>,
}

impl FeedForwardGradients {
    fn new(feed_forward: &PositionWiseFeedForward) -> Self {
        Self {
            first_weight: vec![
                vec![0.0; feed_forward.hidden_dimension().value()];
                feed_forward.input_dimension().value()
            ],
            first_bias: vec![0.0; feed_forward.hidden_dimension().value()],
            second_weight: vec![
                vec![0.0; feed_forward.output_dimension().value()];
                feed_forward.hidden_dimension().value()
            ],
            second_bias: vec![0.0; feed_forward.output_dimension().value()],
        }
    }

    fn accumulate(
        &mut self,
        feed_forward: &PositionWiseFeedForward,
        cache: &FeedForwardRowCache,
        d_output: &[f32],
    ) -> Vec<f32> {
        let mut d_activation = vec![0.0; feed_forward.hidden_dimension().value()];

        for (output_id, d_output_value) in d_output.iter().copied().enumerate() {
            self.second_bias[output_id] += d_output_value;

            for (hidden_id, activation_value) in cache.activation.iter().copied().enumerate() {
                self.second_weight[hidden_id][output_id] += activation_value * d_output_value;
                d_activation[hidden_id] +=
                    feed_forward.second_weight()[hidden_id][output_id] * d_output_value;
            }
        }

        let mut d_input = vec![0.0; feed_forward.input_dimension().value()];

        for (hidden_id, pre_activation_value) in cache.pre_activation.iter().copied().enumerate() {
            let d_pre_activation = if pre_activation_value > 0.0 {
                d_activation[hidden_id]
            } else {
                0.0
            };
            self.first_bias[hidden_id] += d_pre_activation;

            for (input_id, input_value) in cache.input.iter().copied().enumerate() {
                self.first_weight[input_id][hidden_id] += input_value * d_pre_activation;
                d_input[input_id] +=
                    feed_forward.first_weight()[input_id][hidden_id] * d_pre_activation;
            }
        }

        d_input
    }

    fn apply_to(
        self,
        feed_forward: &PositionWiseFeedForward,
        learning_rate: LearningRate,
        row_count: usize,
    ) -> CtResult<PositionWiseFeedForward> {
        if row_count == 0 {
            return Err(CtError::EmptyInput("feed-forward gradient rows"));
        }

        let scale = learning_rate.value() / row_count as f32;
        let mut updated_first_weight = feed_forward.first_weight().to_vec();
        let mut updated_first_bias = feed_forward.first_bias().to_vec();
        let mut updated_second_weight = feed_forward.second_weight().to_vec();
        let mut updated_second_bias = feed_forward.second_bias().to_vec();

        for (row, grad_row) in updated_first_weight.iter_mut().zip(&self.first_weight) {
            for (weight, grad) in row.iter_mut().zip(grad_row) {
                *weight -= scale * grad;
            }
        }

        for (bias, grad) in updated_first_bias.iter_mut().zip(&self.first_bias) {
            *bias -= scale * grad;
        }

        for (row, grad_row) in updated_second_weight.iter_mut().zip(&self.second_weight) {
            for (weight, grad) in row.iter_mut().zip(grad_row) {
                *weight -= scale * grad;
            }
        }

        for (bias, grad) in updated_second_bias.iter_mut().zip(&self.second_bias) {
            *bias -= scale * grad;
        }

        PositionWiseFeedForward::new(
            updated_first_weight,
            updated_first_bias,
            updated_second_weight,
            updated_second_bias,
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
struct AttentionOutputProjectionGradients {
    weight: Vec<Vec<f32>>,
    bias: Vec<f32>,
}

impl AttentionOutputProjectionGradients {
    fn new(output_projection: &AttentionOutputProjection) -> Self {
        Self {
            weight: vec![
                vec![0.0; output_projection.output_dimension().value()];
                output_projection.input_dimension().value()
            ],
            bias: vec![0.0; output_projection.output_dimension().value()],
        }
    }

    fn accumulate(
        &mut self,
        output_projection: &AttentionOutputProjection,
        multi_head_row: &[f32],
        d_projected_attention: &[f32],
    ) -> Vec<f32> {
        let mut d_multi_head = vec![0.0; output_projection.input_dimension().value()];

        for (output_id, d_value) in d_projected_attention.iter().copied().enumerate() {
            self.bias[output_id] += d_value;

            for (input_id, input_value) in multi_head_row.iter().copied().enumerate() {
                self.weight[input_id][output_id] += input_value * d_value;
                d_multi_head[input_id] += output_projection.weight()[input_id][output_id] * d_value;
            }
        }

        d_multi_head
    }

    fn apply_to(
        self,
        output_projection: &AttentionOutputProjection,
        learning_rate: LearningRate,
        row_count: usize,
    ) -> CtResult<AttentionOutputProjection> {
        if row_count == 0 {
            return Err(CtError::EmptyInput(
                "attention output projection gradient rows",
            ));
        }

        let scale = learning_rate.value() / row_count as f32;
        let mut updated_weight = output_projection.weight().to_vec();
        let mut updated_bias = output_projection.bias().to_vec();

        for (row, grad_row) in updated_weight.iter_mut().zip(&self.weight) {
            for (weight, grad) in row.iter_mut().zip(grad_row) {
                *weight -= scale * grad;
            }
        }

        for (bias, grad) in updated_bias.iter_mut().zip(&self.bias) {
            *bias -= scale * grad;
        }

        AttentionOutputProjection::new(updated_weight, updated_bias)
    }
}

#[derive(Debug, Clone, PartialEq)]
struct HiddenProjectionGradients {
    weight: Vec<Vec<f32>>,
    bias: Vec<f32>,
}

impl HiddenProjectionGradients {
    fn new(projection: &HiddenProjection) -> Self {
        Self {
            weight: vec![
                vec![0.0; projection.head_dimension().value()];
                projection.input_dimension().value()
            ],
            bias: vec![0.0; projection.head_dimension().value()],
        }
    }

    fn accumulate(
        &mut self,
        projection: &HiddenProjection,
        hidden_row: &[f32],
        d_output: &[f32],
    ) -> CtResult<()> {
        if hidden_row.len() != projection.input_dimension().value() {
            return Err(CtError::ShapeMismatch {
                op: projection.op,
                expected: format!("input dimension {}", projection.input_dimension().value()),
                got: format!("input dimension {}", hidden_row.len()),
            });
        }

        if d_output.len() != projection.head_dimension().value() {
            return Err(CtError::ShapeMismatch {
                op: projection.op,
                expected: format!("output dimension {}", projection.head_dimension().value()),
                got: format!("output dimension {}", d_output.len()),
            });
        }

        for (output_id, d_output_value) in d_output.iter().copied().enumerate() {
            self.bias[output_id] += d_output_value;

            for (input_id, input_value) in hidden_row.iter().copied().enumerate() {
                self.weight[input_id][output_id] += input_value * d_output_value;
            }
        }

        Ok(())
    }

    fn updated_parts(
        self,
        projection: &HiddenProjection,
        learning_rate: LearningRate,
        row_count: usize,
    ) -> CtResult<(Vec<Vec<f32>>, Vec<f32>)> {
        if row_count == 0 {
            return Err(CtError::EmptyInput("hidden projection gradient rows"));
        }

        let scale = learning_rate.value() / row_count as f32;
        let mut updated_weight = projection.weight().to_vec();
        let mut updated_bias = projection.bias().to_vec();

        for (row, grad_row) in updated_weight.iter_mut().zip(&self.weight) {
            for (weight, grad) in row.iter_mut().zip(grad_row) {
                *weight -= scale * grad;
            }
        }

        for (bias, grad) in updated_bias.iter_mut().zip(&self.bias) {
            *bias -= scale * grad;
        }

        Ok((updated_weight, updated_bias))
    }
}

#[derive(Debug, Clone, PartialEq)]
struct AttentionHeadGradients {
    query: HiddenProjectionGradients,
    key: HiddenProjectionGradients,
    value: HiddenProjectionGradients,
}

impl AttentionHeadGradients {
    fn new(head: &SelfAttentionHead) -> Self {
        Self {
            query: HiddenProjectionGradients::new(&head.query_projection.projection),
            key: HiddenProjectionGradients::new(&head.key_projection.projection),
            value: HiddenProjectionGradients::new(&head.value_projection.projection),
        }
    }

    fn accumulate(
        &mut self,
        head: &SelfAttentionHead,
        input: &HiddenSequence,
        cache: &AttentionHeadTrainingCache,
        mask: &AttentionMask,
        d_output_rows: &[Vec<f32>],
    ) -> CtResult<()> {
        let sequence_len = input.sequence_len().value();
        let value_dimension = head.value_dimension().value();
        let query_key_dimension = head.query_key_dimension().value();

        if d_output_rows.len() != sequence_len {
            return Err(CtError::ShapeMismatch {
                op: "attention head gradients",
                expected: format!("{sequence_len} output rows"),
                got: format!("{} output rows", d_output_rows.len()),
            });
        }

        if mask.query_len().value() != sequence_len || mask.key_len().value() != sequence_len {
            return Err(CtError::ShapeMismatch {
                op: "attention head gradient mask",
                expected: format!("{sequence_len} query rows x {sequence_len} key columns"),
                got: format!(
                    "{} query rows x {} key columns",
                    mask.query_len().value(),
                    mask.key_len().value()
                ),
            });
        }

        let mut d_weights = vec![vec![0.0; sequence_len]; sequence_len];
        let mut d_values = vec![vec![0.0; value_dimension]; sequence_len];

        for (query_id, d_output) in d_output_rows.iter().enumerate() {
            if d_output.len() != value_dimension {
                return Err(CtError::ShapeMismatch {
                    op: "attention head output gradient",
                    expected: format!("value dimension {value_dimension}"),
                    got: format!("value dimension {}", d_output.len()),
                });
            }

            for key_id in 0..sequence_len {
                let value_row = cache.values.rows()[key_id].as_slice();
                let weight = cache.weights.rows()[query_id].as_slice()[key_id];

                for value_id in 0..value_dimension {
                    d_weights[query_id][key_id] += d_output[value_id] * value_row[value_id];
                    d_values[key_id][value_id] += weight * d_output[value_id];
                }
            }
        }

        let mut d_scores = vec![vec![0.0; sequence_len]; sequence_len];

        for query_id in 0..sequence_len {
            let weight_row = cache.weights.rows()[query_id].as_slice();
            let row_dot = d_weights[query_id]
                .iter()
                .zip(weight_row)
                .map(|(grad, weight)| grad * weight)
                .sum::<f32>();

            for key_id in 0..sequence_len {
                if mask.rows()[query_id][key_id] {
                    d_scores[query_id][key_id] =
                        weight_row[key_id] * (d_weights[query_id][key_id] - row_dot);
                }
            }
        }

        let score_scale = (query_key_dimension as f32).sqrt();
        let mut d_queries = vec![vec![0.0; query_key_dimension]; sequence_len];
        let mut d_keys = vec![vec![0.0; query_key_dimension]; sequence_len];

        for query_id in 0..sequence_len {
            let query_row = cache.queries.rows()[query_id].as_slice();

            for key_id in 0..sequence_len {
                let score_gradient = d_scores[query_id][key_id] / score_scale;
                let key_row = cache.keys.rows()[key_id].as_slice();

                for feature in 0..query_key_dimension {
                    d_queries[query_id][feature] += score_gradient * key_row[feature];
                    d_keys[key_id][feature] += score_gradient * query_row[feature];
                }
            }
        }

        for position in 0..sequence_len {
            let hidden_row = input.rows()[position].as_slice();

            self.query.accumulate(
                &head.query_projection.projection,
                hidden_row,
                &d_queries[position],
            )?;
            self.key.accumulate(
                &head.key_projection.projection,
                hidden_row,
                &d_keys[position],
            )?;
            self.value.accumulate(
                &head.value_projection.projection,
                hidden_row,
                &d_values[position],
            )?;
        }

        Ok(())
    }

    fn apply_to(
        self,
        head: &SelfAttentionHead,
        learning_rate: LearningRate,
        row_count: usize,
    ) -> CtResult<SelfAttentionHead> {
        let (query_weight, query_bias) = self.query.updated_parts(
            &head.query_projection.projection,
            learning_rate,
            row_count,
        )?;
        let (key_weight, key_bias) =
            self.key
                .updated_parts(&head.key_projection.projection, learning_rate, row_count)?;
        let (value_weight, value_bias) = self.value.updated_parts(
            &head.value_projection.projection,
            learning_rate,
            row_count,
        )?;

        SelfAttentionHead::new(
            HiddenToQuery::new(query_weight, query_bias)?,
            HiddenToKey::new(key_weight, key_bias)?,
            HiddenToValue::new(value_weight, value_bias)?,
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
struct LayerNormGradients {
    scale: Vec<f32>,
    shift: Vec<f32>,
}

impl LayerNormGradients {
    fn new(parameters: &LayerNormParameters) -> Self {
        Self {
            scale: vec![0.0; parameters.model_dimension().value()],
            shift: vec![0.0; parameters.model_dimension().value()],
        }
    }

    fn accumulate(
        &mut self,
        d_output: &[f32],
        input: &[f32],
        parameters: &LayerNormParameters,
    ) -> Vec<f32> {
        let stats = layer_norm_stats(input, parameters);

        for (feature, (grad, normalized_value)) in
            d_output.iter().zip(&stats.normalized).enumerate()
        {
            self.scale[feature] += grad * normalized_value;
            self.shift[feature] += grad;
        }

        layer_norm_backward_from_stats(d_output, &stats, parameters)
    }

    fn apply_to(
        self,
        parameters: &LayerNormParameters,
        learning_rate: LearningRate,
        row_count: usize,
    ) -> CtResult<LayerNormParameters> {
        if row_count == 0 {
            return Err(CtError::EmptyInput("layer norm gradient rows"));
        }

        let scale = learning_rate.value() / row_count as f32;
        let mut updated_scale = parameters.scale().to_vec();
        let mut updated_shift = parameters.shift().to_vec();

        for (value, grad) in updated_scale.iter_mut().zip(&self.scale) {
            *value -= scale * grad;
        }

        for (value, grad) in updated_shift.iter_mut().zip(&self.shift) {
            *value -= scale * grad;
        }

        LayerNormParameters::new(updated_scale, updated_shift, parameters.epsilon())
    }
}

fn feed_forward_with_cache(
    feed_forward: &PositionWiseFeedForward,
    input: &HiddenSequence,
) -> CtResult<(HiddenSequence, Vec<FeedForwardRowCache>)> {
    if input.model_dimension() != feed_forward.input_dimension() {
        return Err(CtError::ShapeMismatch {
            op: "position-wise feed-forward",
            expected: format!("input dimension {}", feed_forward.input_dimension().value()),
            got: format!("input dimension {}", input.model_dimension().value()),
        });
    }

    let mut output_rows = Vec::with_capacity(input.rows().len());
    let mut cache_rows = Vec::with_capacity(input.rows().len());

    for row in input.rows() {
        let input_row = row.as_slice().to_vec();
        let pre_activation = project_row(
            &input_row,
            feed_forward.first_weight(),
            feed_forward.first_bias(),
        );
        let activation = pre_activation
            .iter()
            .map(|value| value.max(0.0))
            .collect::<Vec<_>>();
        let output = project_row(
            &activation,
            feed_forward.second_weight(),
            feed_forward.second_bias(),
        );

        output_rows.push(output.clone());
        cache_rows.push(FeedForwardRowCache {
            input: input_row,
            pre_activation,
            activation,
            output,
        });
    }

    Ok((HiddenSequence::new(output_rows)?, cache_rows))
}

fn softmax_cross_entropy_logits_gradient(
    logits: &Logits,
    target_index: usize,
    vocab_size: usize,
) -> CtResult<Vec<f32>> {
    if target_index >= vocab_size {
        return Err(CtError::OutOfRange {
            kind: "sequence target",
            index: target_index,
            limit: vocab_size,
        });
    }

    let probabilities = Softmax.apply(logits.clone())?;
    let mut dlogits = probabilities.as_slice().to_vec();
    dlogits[target_index] -= 1.0;

    Ok(dlogits)
}

#[derive(Debug, Clone, PartialEq)]
struct LayerNormStats {
    dimension: f32,
    inverse_std: f32,
    normalized: Vec<f32>,
}

fn layer_norm_stats(input: &[f32], parameters: &LayerNormParameters) -> LayerNormStats {
    let dimension = input.len() as f32;
    let mean = input.iter().sum::<f32>() / dimension;
    let variance = input
        .iter()
        .map(|value| {
            let centered = value - mean;
            centered * centered
        })
        .sum::<f32>()
        / dimension;
    let inverse_std = 1.0 / (variance + parameters.epsilon().value()).sqrt();
    let normalized = input
        .iter()
        .map(|value| (value - mean) * inverse_std)
        .collect::<Vec<_>>();

    LayerNormStats {
        dimension,
        inverse_std,
        normalized,
    }
}

fn layer_norm_backward_from_stats(
    d_output: &[f32],
    stats: &LayerNormStats,
    parameters: &LayerNormParameters,
) -> Vec<f32> {
    let d_normalized = d_output
        .iter()
        .zip(parameters.scale())
        .map(|(grad, scale)| grad * scale)
        .collect::<Vec<_>>();
    let sum_d_normalized = d_normalized.iter().sum::<f32>();
    let sum_d_normalized_times_normalized = d_normalized
        .iter()
        .zip(&stats.normalized)
        .map(|(grad, normalized_value)| grad * normalized_value)
        .sum::<f32>();

    d_normalized
        .iter()
        .zip(&stats.normalized)
        .map(|(grad, normalized_value)| {
            (stats.dimension * grad
                - sum_d_normalized
                - normalized_value * sum_d_normalized_times_normalized)
                * stats.inverse_std
                / stats.dimension
        })
        .collect()
}

fn validate_projection_input(
    op: &'static str,
    expected: ModelDimension,
    got: ModelDimension,
) -> CtResult<()> {
    if expected != got {
        return Err(CtError::ShapeMismatch {
            op,
            expected: format!("model dimension {}", expected.value()),
            got: format!("model dimension {}", got.value()),
        });
    }

    Ok(())
}

fn add_rows(left: &[f32], right: &[f32]) -> Vec<f32> {
    left.iter()
        .zip(right.iter())
        .map(|(left, right)| left + right)
        .collect()
}

fn validate_linear_parts(
    op: &'static str,
    weight: &[Vec<f32>],
    bias: &[f32],
) -> CtResult<(ModelDimension, ModelDimension)> {
    if weight.is_empty() {
        return Err(CtError::EmptyInput("linear weight"));
    }

    if bias.is_empty() {
        return Err(CtError::EmptyInput("linear bias"));
    }

    let output_dimension = bias.len();

    if bias.iter().any(|value| !value.is_finite()) {
        return Err(CtError::ShapeMismatch {
            op,
            expected: "finite bias values".to_string(),
            got: "non-finite bias value".to_string(),
        });
    }

    for row in weight {
        if row.len() != output_dimension {
            return Err(CtError::ShapeMismatch {
                op,
                expected: format!("weight rows have {output_dimension} columns"),
                got: format!("weight row with {} columns", row.len()),
            });
        }

        if row.iter().any(|value| !value.is_finite()) {
            return Err(CtError::ShapeMismatch {
                op,
                expected: "finite weight values".to_string(),
                got: "non-finite weight value".to_string(),
            });
        }
    }

    Ok((
        ModelDimension::new(weight.len())?,
        ModelDimension::new(output_dimension)?,
    ))
}

fn normalize_row(input: &[f32], parameters: &LayerNormParameters) -> Vec<f32> {
    let mean = input.iter().sum::<f32>() / input.len() as f32;
    let variance = input
        .iter()
        .map(|value| {
            let centered = value - mean;
            centered * centered
        })
        .sum::<f32>()
        / input.len() as f32;
    let denominator = (variance + parameters.epsilon.value()).sqrt();

    input
        .iter()
        .zip(parameters.scale.iter().zip(&parameters.shift))
        .map(|(value, (scale, shift))| ((value - mean) / denominator) * scale + shift)
        .collect()
}

fn project_row(input: &[f32], weight: &[Vec<f32>], bias: &[f32]) -> Vec<f32> {
    let mut output = bias.to_vec();

    for (feature, input_value) in input.iter().enumerate() {
        for (output_value, weight_value) in output.iter_mut().zip(&weight[feature]) {
            *output_value += input_value * weight_value;
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scaled_dot_product_scores_build_query_by_key_rows() -> CtResult<()> {
        let queries = QuerySequence::new(vec![vec![1.0, 0.0], vec![0.0, 1.0]])?;
        let keys = KeySequence::new(vec![vec![1.0, 0.0], vec![0.0, 1.0], vec![1.0, 1.0]])?;

        let scores = ScaledDotProductScores.apply(Product::new(queries, keys))?;

        assert_eq!(scores.query_len().value(), 2);
        assert_eq!(scores.key_len().value(), 3);
        assert!(crate::domain::approx_eq(
            scores.rows()[0].as_slice()[0],
            std::f32::consts::FRAC_1_SQRT_2,
            1e-4
        ));
        assert!(crate::domain::approx_eq(
            scores.rows()[0].as_slice()[1],
            0.0,
            1e-4
        ));
        assert!(crate::domain::approx_eq(
            scores.rows()[1].as_slice()[2],
            std::f32::consts::FRAC_1_SQRT_2,
            1e-4
        ));

        Ok(())
    }

    #[test]
    fn weighted_value_mixing_builds_one_output_per_query() -> CtResult<()> {
        let queries = QuerySequence::new(vec![vec![1.0, 0.0], vec![0.0, 1.0]])?;
        let keys = KeySequence::new(vec![vec![1.0, 0.0], vec![0.0, 1.0], vec![1.0, 1.0]])?;
        let values = ValueSequence::new(vec![vec![1.0, 10.0], vec![2.0, 20.0], vec![3.0, 30.0]])?;

        let scores = ScaledDotProductScores.apply(Product::new(queries, keys))?;
        let weights = AttentionSoftmax.apply(scores)?;
        let output = WeightedValueMixing.apply(Product::new(weights, values))?;

        assert_eq!(output.sequence_len().value(), 2);
        assert_eq!(output.head_dimension().value(), 2);
        assert!(crate::domain::approx_eq(
            output.rows()[0].as_slice()[0],
            2.0,
            1e-4
        ));
        assert!(crate::domain::approx_eq(
            output.rows()[0].as_slice()[1],
            20.0,
            1e-4
        ));

        Ok(())
    }

    #[test]
    fn concatenate_heads_preserves_sequence_and_concatenates_features() -> CtResult<()> {
        let head_a = AttentionOutput::new(vec![vec![1.0, 10.0], vec![2.0, 20.0]])?;
        let head_b = AttentionOutput::new(vec![vec![3.0, 30.0], vec![4.0, 40.0]])?;

        let heads = AttentionHeadOutputs::new(vec![head_a, head_b])?;
        let output = ConcatenateHeads.apply(heads)?;

        assert_eq!(output.sequence_len().value(), 2);
        assert_eq!(output.head_count().value(), 2);
        assert_eq!(output.head_dimension().value(), 2);
        assert_eq!(output.model_dimension().value(), 4);
        assert_eq!(output.rows()[0].as_slice(), &[1.0, 10.0, 3.0, 30.0]);
        assert_eq!(output.rows()[1].as_slice(), &[2.0, 20.0, 4.0, 40.0]);

        Ok(())
    }

    #[test]
    fn attention_output_projection_maps_multi_head_rows() -> CtResult<()> {
        let head_a = AttentionOutput::new(vec![vec![1.0, 10.0], vec![2.0, 20.0]])?;
        let head_b = AttentionOutput::new(vec![vec![3.0, 30.0], vec![4.0, 40.0]])?;
        let multi_head =
            ConcatenateHeads.apply(AttentionHeadOutputs::new(vec![head_a, head_b])?)?;
        let projection = AttentionOutputProjection::new(
            vec![
                vec![1.0, 0.0],
                vec![0.0, 0.1],
                vec![0.5, 0.0],
                vec![0.0, 0.01],
            ],
            vec![0.0, 1.0],
        )?;

        let output = projection.apply(multi_head)?;

        assert_eq!(output.sequence_len().value(), 2);
        assert_eq!(output.model_dimension().value(), 2);
        assert!(crate::domain::approx_eq(
            output.rows()[0].as_slice()[0],
            2.5,
            1e-4
        ));
        assert!(crate::domain::approx_eq(
            output.rows()[0].as_slice()[1],
            2.3,
            1e-4
        ));
        assert!(crate::domain::approx_eq(
            output.rows()[1].as_slice()[0],
            4.0,
            1e-4
        ));
        assert!(crate::domain::approx_eq(
            output.rows()[1].as_slice()[1],
            3.4,
            1e-4
        ));

        Ok(())
    }

    #[test]
    fn residual_connection_adds_matching_sequences() -> CtResult<()> {
        let hidden = HiddenSequence::new(vec![vec![1.0, 2.0], vec![3.0, 4.0]])?;
        let sublayer_output = ProjectedAttentionOutput::new(vec![vec![0.5, 1.5], vec![2.5, 3.5]])?;

        let output = ResidualConnection.apply(Product::new(hidden, sublayer_output))?;

        assert_eq!(output.sequence_len().value(), 2);
        assert_eq!(output.model_dimension().value(), 2);
        assert_eq!(output.rows()[0].as_slice(), &[1.5, 3.5]);
        assert_eq!(output.rows()[1].as_slice(), &[5.5, 7.5]);

        Ok(())
    }

    #[test]
    fn layer_normalization_preserves_shape_and_normalizes_each_row() -> CtResult<()> {
        let hidden = HiddenSequence::new(vec![vec![1.0, 3.0], vec![2.0, 4.0]])?;
        let norm = LayerNormalization::new(LayerNormParameters::identity(ModelDimension::new(2)?));

        let output = norm.apply(hidden)?;

        assert_eq!(output.sequence_len().value(), 2);
        assert_eq!(output.model_dimension().value(), 2);
        assert!(crate::domain::approx_eq(
            output.rows()[0].as_slice()[0],
            -0.999995,
            1e-4
        ));
        assert!(crate::domain::approx_eq(
            output.rows()[0].as_slice()[1],
            0.999995,
            1e-4
        ));
        assert!(crate::domain::approx_eq(
            output.rows()[1].as_slice()[0],
            -0.999995,
            1e-4
        ));
        assert!(crate::domain::approx_eq(
            output.rows()[1].as_slice()[1],
            0.999995,
            1e-4
        ));

        Ok(())
    }

    #[test]
    fn layer_normalization_applies_scale_and_shift() -> CtResult<()> {
        let hidden = HiddenSequence::new(vec![vec![1.0, 3.0]])?;
        let params = LayerNormParameters::new(
            vec![2.0, 0.5],
            vec![1.0, -1.0],
            NormalizationEpsilon::new(1e-5)?,
        )?;
        let norm = LayerNormalization::new(params);

        let output = norm.apply(hidden)?;

        assert!(crate::domain::approx_eq(
            output.rows()[0].as_slice()[0],
            -0.99999,
            1e-4
        ));
        assert!(crate::domain::approx_eq(
            output.rows()[0].as_slice()[1],
            -0.5000025,
            1e-4
        ));

        Ok(())
    }

    #[test]
    fn position_wise_feed_forward_maps_each_row_and_preserves_shape() -> CtResult<()> {
        let hidden = HiddenSequence::new(vec![vec![1.0, 2.0], vec![3.0, 4.0]])?;
        let feed_forward = PositionWiseFeedForward::new(
            vec![vec![1.0, -1.0, 0.5], vec![0.0, 1.0, 0.5]],
            vec![0.0, 0.0, 0.0],
            vec![vec![1.0, 0.0], vec![0.0, 1.0], vec![0.5, 0.5]],
            vec![0.0, 0.0],
        )?;

        let output = feed_forward.apply(hidden)?;

        assert_eq!(output.sequence_len().value(), 2);
        assert_eq!(output.model_dimension().value(), 2);
        assert!(crate::domain::approx_eq(
            output.rows()[0].as_slice()[0],
            1.75,
            1e-4
        ));
        assert!(crate::domain::approx_eq(
            output.rows()[0].as_slice()[1],
            1.75,
            1e-4
        ));
        assert!(crate::domain::approx_eq(
            output.rows()[1].as_slice()[0],
            4.75,
            1e-4
        ));
        assert!(crate::domain::approx_eq(
            output.rows()[1].as_slice()[1],
            2.75,
            1e-4
        ));

        Ok(())
    }

    #[test]
    fn attention_mask_removes_disallowed_positions_before_softmax() -> CtResult<()> {
        let scores = AttentionScores::new(vec![vec![2.0, 1.0, 2.0]])?;
        let mask = AttentionMask::new(vec![vec![true, false, true]])?;
        let masked_scores = MaskedAttentionScores.apply(Product::new(scores, mask))?;
        let weights = AttentionSoftmax.apply(masked_scores)?;

        assert!(crate::domain::approx_eq(
            weights.rows()[0].as_slice()[0],
            0.5,
            1e-4
        ));
        assert!(crate::domain::approx_eq(
            weights.rows()[0].as_slice()[1],
            0.0,
            1e-4
        ));
        assert!(crate::domain::approx_eq(
            weights.rows()[0].as_slice()[2],
            0.5,
            1e-4
        ));

        Ok(())
    }

    #[test]
    fn attention_softmax_normalizes_each_query_row() -> CtResult<()> {
        let scores = AttentionScores::new(vec![vec![2.0, 1.0], vec![0.0, 3.0]])?;
        let weights = AttentionSoftmax.apply(scores)?;

        assert_eq!(weights.query_len().value(), 2);
        assert_eq!(weights.key_len().value(), 2);

        for row in weights.rows() {
            let sum: f32 = row.as_slice().iter().sum();
            assert!(crate::domain::approx_eq(sum, 1.0, 1e-4));
        }

        Ok(())
    }

    #[test]
    fn attention_scores_reject_non_finite_values() {
        assert!(matches!(
            AttentionScores::new(vec![vec![1.0, f32::NAN]]),
            Err(CtError::ShapeMismatch {
                op: "attention scores",
                ..
            })
        ));
    }

    #[test]
    fn attention_scores_reject_ragged_rows() {
        assert!(matches!(
            AttentionScores::new(vec![vec![1.0, 2.0], vec![3.0]]),
            Err(CtError::ShapeMismatch {
                op: "attention scores",
                ..
            })
        ));
    }

    #[test]
    fn attention_mask_rejects_rows_with_no_allowed_keys() {
        assert!(matches!(
            AttentionMask::new(vec![vec![false, false]]),
            Err(CtError::EmptyInput("attention mask row allows no keys"))
        ));
    }

    #[test]
    fn masked_attention_scores_reject_shape_mismatch() -> CtResult<()> {
        let scores = AttentionScores::new(vec![vec![1.0, 2.0]])?;
        let mask = AttentionMask::new(vec![vec![true, true], vec![true, true]])?;

        assert!(matches!(
            MaskedAttentionScores.apply(Product::new(scores, mask)),
            Err(CtError::ShapeMismatch {
                op: "masked attention scores",
                ..
            })
        ));

        Ok(())
    }

    #[test]
    fn query_sequence_rejects_ragged_rows() {
        assert!(matches!(
            QuerySequence::new(vec![vec![1.0, 2.0], vec![3.0]]),
            Err(CtError::ShapeMismatch {
                op: "query sequence",
                ..
            })
        ));
    }

    #[test]
    fn value_sequence_rejects_empty_rows() {
        assert!(matches!(
            ValueSequence::new(vec![Vec::new()]),
            Err(CtError::EmptyInput("attention vector row"))
        ));
    }

    #[test]
    fn key_sequence_rejects_non_finite_values() {
        assert!(matches!(
            KeySequence::new(vec![vec![1.0, f32::NAN]]),
            Err(CtError::ShapeMismatch {
                op: "key sequence",
                ..
            })
        ));
    }

    #[test]
    fn scaled_dot_product_rejects_mismatched_head_dimensions() -> CtResult<()> {
        let queries = QuerySequence::new(vec![vec![1.0, 0.0]])?;
        let keys = KeySequence::new(vec![vec![1.0, 0.0, 1.0]])?;

        assert!(matches!(
            ScaledDotProductScores.apply(Product::new(queries, keys)),
            Err(CtError::ShapeMismatch {
                op: "scaled dot-product attention scores",
                ..
            })
        ));

        Ok(())
    }

    #[test]
    fn weighted_value_mixing_rejects_value_length_mismatch() -> CtResult<()> {
        let weights = AttentionWeights::new(vec![Distribution::new(vec![0.5, 0.5])?])?;
        let values = ValueSequence::new(vec![vec![1.0, 10.0]])?;

        assert!(matches!(
            WeightedValueMixing.apply(Product::new(weights, values)),
            Err(CtError::ShapeMismatch {
                op: "weighted value mixing",
                ..
            })
        ));

        Ok(())
    }

    #[test]
    fn sequence_and_head_dimensions_reject_zero() {
        assert!(matches!(
            SequenceLength::new(0),
            Err(CtError::EmptyInput("sequence length"))
        ));
        assert!(matches!(
            HeadDimension::new(0),
            Err(CtError::EmptyInput("head dimension"))
        ));
        assert!(matches!(
            HeadCount::new(0),
            Err(CtError::EmptyInput("head count"))
        ));
    }

    #[test]
    fn attention_head_outputs_reject_sequence_mismatch() -> CtResult<()> {
        let head_a = AttentionOutput::new(vec![vec![1.0, 10.0]])?;
        let head_b = AttentionOutput::new(vec![vec![2.0, 20.0], vec![3.0, 30.0]])?;

        assert!(matches!(
            AttentionHeadOutputs::new(vec![head_a, head_b]),
            Err(CtError::ShapeMismatch {
                op: "attention head outputs",
                ..
            })
        ));

        Ok(())
    }

    #[test]
    fn attention_head_outputs_reject_head_dimension_mismatch() -> CtResult<()> {
        let head_a = AttentionOutput::new(vec![vec![1.0, 10.0]])?;
        let head_b = AttentionOutput::new(vec![vec![2.0, 20.0, 200.0]])?;

        assert!(matches!(
            AttentionHeadOutputs::new(vec![head_a, head_b]),
            Err(CtError::ShapeMismatch {
                op: "attention head outputs",
                ..
            })
        ));

        Ok(())
    }

    #[test]
    fn attention_output_projection_rejects_input_dimension_mismatch() -> CtResult<()> {
        let head_a = AttentionOutput::new(vec![vec![1.0, 10.0]])?;
        let head_b = AttentionOutput::new(vec![vec![2.0, 20.0]])?;
        let multi_head =
            ConcatenateHeads.apply(AttentionHeadOutputs::new(vec![head_a, head_b])?)?;
        let projection =
            AttentionOutputProjection::new(vec![vec![1.0], vec![1.0], vec![1.0]], vec![0.0])?;

        assert!(matches!(
            projection.apply(multi_head),
            Err(CtError::ShapeMismatch {
                op: "attention output projection",
                ..
            })
        ));

        Ok(())
    }

    #[test]
    fn attention_output_projection_rejects_bad_weight_shapes() {
        assert!(matches!(
            AttentionOutputProjection::new(vec![vec![1.0, 0.0], vec![1.0]], vec![0.0, 0.0]),
            Err(CtError::ShapeMismatch {
                op: "attention output projection",
                ..
            })
        ));
    }

    #[test]
    fn attention_output_projection_rejects_non_finite_values() {
        assert!(matches!(
            AttentionOutputProjection::new(vec![vec![1.0]], vec![f32::NAN]),
            Err(CtError::ShapeMismatch {
                op: "attention output projection",
                ..
            })
        ));
        assert!(matches!(
            AttentionOutputProjection::new(vec![vec![f32::INFINITY]], vec![0.0]),
            Err(CtError::ShapeMismatch {
                op: "attention output projection",
                ..
            })
        ));
    }

    #[test]
    fn residual_connection_rejects_sequence_mismatch() -> CtResult<()> {
        let hidden = HiddenSequence::new(vec![vec![1.0, 2.0]])?;
        let sublayer_output = ProjectedAttentionOutput::new(vec![vec![0.5, 1.5], vec![2.5, 3.5]])?;

        assert!(matches!(
            ResidualConnection.apply(Product::new(hidden, sublayer_output)),
            Err(CtError::ShapeMismatch {
                op: "residual connection",
                ..
            })
        ));

        Ok(())
    }

    #[test]
    fn residual_connection_rejects_model_dimension_mismatch() -> CtResult<()> {
        let hidden = HiddenSequence::new(vec![vec![1.0, 2.0]])?;
        let sublayer_output = ProjectedAttentionOutput::new(vec![vec![0.5, 1.5, 2.5]])?;

        assert!(matches!(
            ResidualConnection.apply(Product::new(hidden, sublayer_output)),
            Err(CtError::ShapeMismatch {
                op: "residual connection",
                ..
            })
        ));

        Ok(())
    }

    #[test]
    fn layer_normalization_rejects_model_dimension_mismatch() -> CtResult<()> {
        let hidden = HiddenSequence::new(vec![vec![1.0, 2.0, 3.0]])?;
        let norm = LayerNormalization::new(LayerNormParameters::identity(ModelDimension::new(2)?));

        assert!(matches!(
            norm.apply(hidden),
            Err(CtError::ShapeMismatch {
                op: "layer normalization",
                ..
            })
        ));

        Ok(())
    }

    #[test]
    fn layer_norm_parameters_reject_bad_shapes_and_values() {
        assert!(matches!(
            LayerNormParameters::new(vec![1.0, 1.0], vec![0.0], NormalizationEpsilon(1e-5)),
            Err(CtError::ShapeMismatch {
                op: "layer norm parameters",
                ..
            })
        ));
        assert!(matches!(
            LayerNormParameters::new(vec![f32::NAN], vec![0.0], NormalizationEpsilon(1e-5)),
            Err(CtError::ShapeMismatch {
                op: "layer norm parameters",
                ..
            })
        ));
        assert!(matches!(
            LayerNormParameters::new(vec![1.0], vec![f32::INFINITY], NormalizationEpsilon(1e-5)),
            Err(CtError::ShapeMismatch {
                op: "layer norm parameters",
                ..
            })
        ));
        assert!(matches!(
            NormalizationEpsilon::new(0.0),
            Err(CtError::ShapeMismatch {
                op: "normalization epsilon",
                ..
            })
        ));
    }

    #[test]
    fn position_wise_feed_forward_rejects_input_dimension_mismatch() -> CtResult<()> {
        let hidden = HiddenSequence::new(vec![vec![1.0, 2.0, 3.0]])?;
        let feed_forward = PositionWiseFeedForward::new(
            vec![vec![1.0, 0.0], vec![0.0, 1.0]],
            vec![0.0, 0.0],
            vec![vec![1.0, 0.0], vec![0.0, 1.0]],
            vec![0.0, 0.0],
        )?;

        assert!(matches!(
            feed_forward.apply(hidden),
            Err(CtError::ShapeMismatch {
                op: "position-wise feed-forward",
                ..
            })
        ));

        Ok(())
    }

    #[test]
    fn position_wise_feed_forward_rejects_incompatible_layer_shapes() {
        assert!(matches!(
            PositionWiseFeedForward::new(
                vec![vec![1.0, 0.0]],
                vec![0.0, 0.0],
                vec![vec![1.0], vec![1.0], vec![1.0]],
                vec![0.0],
            ),
            Err(CtError::ShapeMismatch {
                op: "position-wise feed-forward",
                ..
            })
        ));
        assert!(matches!(
            PositionWiseFeedForward::new(
                vec![vec![1.0, 0.0]],
                vec![0.0, 0.0],
                vec![vec![1.0, 0.0], vec![1.0, 0.0]],
                vec![0.0, 0.0],
            ),
            Err(CtError::ShapeMismatch {
                op: "position-wise feed-forward",
                ..
            })
        ));
    }

    #[test]
    fn position_wise_feed_forward_rejects_non_finite_values() {
        assert!(matches!(
            PositionWiseFeedForward::new(
                vec![vec![f32::NAN]],
                vec![0.0],
                vec![vec![1.0]],
                vec![0.0],
            ),
            Err(CtError::ShapeMismatch {
                op: "position-wise feed-forward first layer",
                ..
            })
        ));
        assert!(matches!(
            PositionWiseFeedForward::new(
                vec![vec![1.0]],
                vec![0.0],
                vec![vec![1.0]],
                vec![f32::INFINITY],
            ),
            Err(CtError::ShapeMismatch {
                op: "position-wise feed-forward second layer",
                ..
            })
        ));
    }

    #[test]
    fn positional_encoding_adds_position_rows_and_preserves_shape() -> CtResult<()> {
        let hidden = HiddenSequence::new(vec![vec![1.0, 2.0], vec![3.0, 4.0]])?;
        let positions = PositionalEncoding::new(vec![vec![0.1, 0.2], vec![0.3, 0.4]])?;

        let output = positions.apply(hidden)?;

        assert_eq!(output.sequence_len().value(), 2);
        assert_eq!(output.model_dimension().value(), 2);
        assert!(crate::domain::approx_eq(
            output.rows()[0].as_slice()[0],
            1.1,
            1e-4
        ));
        assert!(crate::domain::approx_eq(
            output.rows()[1].as_slice()[1],
            4.4,
            1e-4
        ));

        Ok(())
    }

    #[test]
    fn positional_encoding_rejects_model_dimension_mismatch() -> CtResult<()> {
        let hidden = HiddenSequence::new(vec![vec![1.0, 2.0]])?;
        let positions = PositionalEncoding::new(vec![vec![0.1, 0.2, 0.3]])?;

        assert!(matches!(
            positions.apply(hidden),
            Err(CtError::ShapeMismatch {
                op: "positional encoding",
                ..
            })
        ));

        Ok(())
    }

    #[test]
    fn positional_encoding_rejects_sequence_too_long() -> CtResult<()> {
        let hidden = HiddenSequence::new(vec![vec![1.0], vec![2.0]])?;
        let positions = PositionalEncoding::new(vec![vec![0.1]])?;

        assert!(matches!(
            positions.apply(hidden),
            Err(CtError::ShapeMismatch {
                op: "positional encoding",
                ..
            })
        ));

        Ok(())
    }

    #[test]
    fn hidden_to_query_projects_hidden_rows() -> CtResult<()> {
        let hidden = HiddenSequence::new(vec![vec![1.0, 2.0], vec![3.0, 4.0]])?;
        let projection = HiddenToQuery::new(vec![vec![1.0, 0.0], vec![0.0, 1.0]], vec![0.5, -0.5])?;

        let queries = projection.apply(hidden)?;

        assert_eq!(queries.sequence_len().value(), 2);
        assert_eq!(queries.head_dimension().value(), 2);
        assert!(crate::domain::approx_eq(
            queries.rows()[0].as_slice()[0],
            1.5,
            1e-4
        ));
        assert!(crate::domain::approx_eq(
            queries.rows()[1].as_slice()[1],
            3.5,
            1e-4
        ));

        Ok(())
    }

    #[test]
    fn hidden_projection_rejects_input_dimension_mismatch() -> CtResult<()> {
        let hidden = HiddenSequence::new(vec![vec![1.0, 2.0, 3.0]])?;
        let projection = HiddenToValue::new(vec![vec![1.0, 0.0], vec![0.0, 1.0]], vec![0.0, 0.0])?;

        assert!(matches!(
            projection.apply(hidden),
            Err(CtError::ShapeMismatch {
                op: "hidden-to-value projection",
                ..
            })
        ));

        Ok(())
    }

    #[test]
    fn residual_connection_adds_hidden_sequences() -> CtResult<()> {
        let left = HiddenSequence::new(vec![vec![1.0, 2.0]])?;
        let right = HiddenSequence::new(vec![vec![3.0, 4.0]])?;

        let output = ResidualConnection.apply(Product::new(left, right))?;

        assert_eq!(output.rows()[0].as_slice(), &[4.0, 6.0]);

        Ok(())
    }

    #[test]
    fn single_head_transformer_block_preserves_hidden_sequence_shape() -> CtResult<()> {
        let block = tiny_single_head_block()?;
        let hidden = HiddenSequence::new(vec![vec![1.0, 0.0], vec![0.0, 1.0]])?;

        let output = block.apply(hidden)?;

        assert_eq!(output.sequence_len().value(), 2);
        assert_eq!(output.model_dimension().value(), 2);
        assert!(
            output
                .rows()
                .iter()
                .flat_map(|row| row.as_slice())
                .all(|value| value.is_finite())
        );

        Ok(())
    }

    #[test]
    fn single_head_transformer_block_rejects_constructor_dimension_mismatch() -> CtResult<()> {
        let query = HiddenToQuery::new(vec![vec![1.0, 0.0], vec![0.0, 1.0]], vec![0.0, 0.0])?;
        let key = HiddenToKey::new(
            vec![vec![1.0, 0.0], vec![0.0, 1.0], vec![0.0, 0.0]],
            vec![0.0, 0.0],
        )?;
        let value = HiddenToValue::new(vec![vec![1.0, 0.0], vec![0.0, 1.0]], vec![0.0, 0.0])?;
        let output_projection =
            AttentionOutputProjection::new(vec![vec![1.0, 0.0], vec![0.0, 1.0]], vec![0.0, 0.0])?;
        let model_dimension = ModelDimension::new(2)?;
        let attention_norm =
            LayerNormalization::new(LayerNormParameters::identity(model_dimension));
        let feed_forward = PositionWiseFeedForward::new(
            vec![vec![1.0, 0.0], vec![0.0, 1.0]],
            vec![0.0, 0.0],
            vec![vec![1.0, 0.0], vec![0.0, 1.0]],
            vec![0.0, 0.0],
        )?;
        let feed_forward_norm =
            LayerNormalization::new(LayerNormParameters::identity(model_dimension));

        assert!(matches!(
            SingleHeadTransformerBlock::new(
                query,
                key,
                value,
                output_projection,
                attention_norm,
                feed_forward,
                feed_forward_norm,
            ),
            Err(CtError::ShapeMismatch {
                op: "single-head block key projection",
                ..
            })
        ));

        Ok(())
    }

    #[test]
    fn single_head_transformer_block_rejects_apply_dimension_mismatch() -> CtResult<()> {
        let block = tiny_single_head_block()?;
        let hidden = HiddenSequence::new(vec![vec![1.0, 0.0, 0.0]])?;

        assert!(matches!(
            block.apply(hidden),
            Err(CtError::ShapeMismatch {
                op: "single-head block",
                ..
            })
        ));

        Ok(())
    }

    #[test]
    fn self_attention_head_rejects_query_key_head_mismatch() -> CtResult<()> {
        let query = HiddenToQuery::new(vec![vec![1.0], vec![0.0]], vec![0.0])?;
        let key = HiddenToKey::new(vec![vec![1.0, 0.0], vec![0.0, 1.0]], vec![0.0, 0.0])?;
        let value = HiddenToValue::new(vec![vec![1.0], vec![0.0]], vec![0.0])?;

        assert!(matches!(
            SelfAttentionHead::new(query, key, value),
            Err(CtError::ShapeMismatch {
                op: "self-attention head",
                ..
            })
        ));

        Ok(())
    }

    #[test]
    fn multi_head_transformer_block_preserves_hidden_sequence_shape() -> CtResult<()> {
        let block = tiny_multi_head_block()?;
        let hidden = HiddenSequence::new(vec![vec![1.0, 0.0], vec![0.0, 1.0]])?;

        let output = block.apply(hidden)?;

        assert_eq!(block.head_count().value(), 2);
        assert_eq!(block.value_dimension().value(), 1);
        assert_eq!(output.sequence_len().value(), 2);
        assert_eq!(output.model_dimension().value(), 2);
        assert!(
            output
                .rows()
                .iter()
                .flat_map(|row| row.as_slice())
                .all(|value| value.is_finite())
        );

        Ok(())
    }

    #[test]
    fn multi_head_transformer_block_rejects_value_dimension_mismatch() -> CtResult<()> {
        let head_a = tiny_self_attention_head_first_feature()?;
        let head_b = SelfAttentionHead::new(
            HiddenToQuery::new(vec![vec![0.0], vec![1.0]], vec![0.0])?,
            HiddenToKey::new(vec![vec![0.0], vec![1.0]], vec![0.0])?,
            HiddenToValue::new(vec![vec![0.0, 1.0], vec![1.0, 0.0]], vec![0.0, 0.0])?,
        )?;
        let model_dimension = ModelDimension::new(2)?;

        assert!(matches!(
            MultiHeadTransformerBlock::new(
                vec![head_a, head_b],
                AttentionOutputProjection::new(
                    vec![vec![1.0, 0.0], vec![0.0, 1.0]],
                    vec![0.0, 0.0],
                )?,
                LayerNormalization::new(LayerNormParameters::identity(model_dimension)),
                identity_feed_forward()?,
                LayerNormalization::new(LayerNormParameters::identity(model_dimension)),
            ),
            Err(CtError::ShapeMismatch {
                op: "multi-head block",
                ..
            })
        ));

        Ok(())
    }

    #[test]
    fn multi_head_transformer_block_rejects_output_projection_input_mismatch() -> CtResult<()> {
        let model_dimension = ModelDimension::new(2)?;

        assert!(matches!(
            MultiHeadTransformerBlock::new(
                vec![
                    tiny_self_attention_head_first_feature()?,
                    tiny_self_attention_head_second_feature()?,
                ],
                AttentionOutputProjection::new(
                    vec![vec![1.0, 0.0], vec![0.0, 1.0], vec![0.0, 0.0]],
                    vec![0.0, 0.0],
                )?,
                LayerNormalization::new(LayerNormParameters::identity(model_dimension)),
                identity_feed_forward()?,
                LayerNormalization::new(LayerNormParameters::identity(model_dimension)),
            ),
            Err(CtError::ShapeMismatch {
                op: "multi-head block output projection input",
                ..
            })
        ));

        Ok(())
    }

    #[test]
    fn multi_head_transformer_block_rejects_apply_dimension_mismatch() -> CtResult<()> {
        let block = tiny_multi_head_block()?;
        let hidden = HiddenSequence::new(vec![vec![1.0, 0.0, 0.0]])?;

        assert!(matches!(
            block.apply(hidden),
            Err(CtError::ShapeMismatch {
                op: "multi-head block",
                ..
            })
        ));

        Ok(())
    }

    #[test]
    fn masked_multi_head_transformer_block_preserves_hidden_sequence_shape() -> CtResult<()> {
        let block = tiny_masked_multi_head_block()?;
        let hidden = HiddenSequence::new(vec![vec![1.0, 0.0], vec![0.0, 1.0]])?;
        let mask = AttentionMask::new(vec![vec![true, false], vec![true, true]])?;

        let output = block.apply(Product::new(hidden, mask))?;

        assert_eq!(block.head_count().value(), 2);
        assert_eq!(output.sequence_len().value(), 2);
        assert_eq!(output.model_dimension().value(), 2);
        assert!(
            output
                .rows()
                .iter()
                .flat_map(|row| row.as_slice())
                .all(|value| value.is_finite())
        );

        Ok(())
    }

    #[test]
    fn masked_multi_head_transformer_block_rejects_mask_shape_mismatch() -> CtResult<()> {
        let block = tiny_masked_multi_head_block()?;
        let hidden = HiddenSequence::new(vec![vec![1.0, 0.0], vec![0.0, 1.0]])?;
        let mask = AttentionMask::new(vec![vec![true, true, true], vec![true, true, true]])?;

        assert!(matches!(
            block.apply(Product::new(hidden, mask)),
            Err(CtError::ShapeMismatch {
                op: "masked attention scores",
                ..
            })
        ));

        Ok(())
    }

    #[test]
    fn transformer_readout_maps_each_hidden_position_to_logits() -> CtResult<()> {
        let readout = TransformerReadout::new(
            vec![vec![1.0, 0.0, 0.5], vec![0.0, 1.0, -0.5]],
            vec![0.0, 0.1, -0.1],
        )?;
        let hidden = HiddenSequence::new(vec![vec![2.0, 3.0], vec![4.0, 5.0]])?;

        let logits = readout.apply(hidden)?;

        assert_eq!(logits.sequence_len().value(), 2);
        assert_eq!(logits.vocab_size().value(), 3);
        assert_eq!(logits.rows()[0].as_slice(), &[2.0, 3.1, -0.6]);
        assert_eq!(logits.rows()[1].as_slice(), &[4.0, 5.1, -0.6]);
        Ok(())
    }

    #[test]
    fn tiny_transformer_parameters_forward_maps_hidden_and_mask_to_sequence_logits() -> CtResult<()>
    {
        let parameters = tiny_transformer_parameters()?;
        let hidden = HiddenSequence::new(vec![vec![1.0, 0.0], vec![0.0, 1.0]])?;
        let mask = AttentionMask::new(vec![vec![true, false], vec![true, true]])?;

        let logits = parameters.apply(Product::new(hidden, mask))?;

        assert_eq!(parameters.model_dimension().value(), 2);
        assert_eq!(parameters.max_sequence_len().value(), 2);
        assert_eq!(logits.sequence_len().value(), 2);
        assert_eq!(logits.vocab_size().value(), 3);
        assert!(
            logits
                .rows()
                .iter()
                .flat_map(|row| row.as_slice())
                .all(|value| value.is_finite())
        );

        Ok(())
    }

    #[test]
    fn tiny_transformer_parameters_rejects_readout_dimension_mismatch() -> CtResult<()> {
        let positional_encoding = PositionalEncoding::new(vec![vec![0.1, 0.0], vec![0.0, 0.1]])?;
        let block = tiny_masked_multi_head_block()?;
        let readout = TransformerReadout::new(vec![vec![1.0], vec![0.0], vec![0.5]], vec![0.0])?;

        assert!(matches!(
            TinyTransformerParameters::new(positional_encoding, block, readout),
            Err(CtError::ShapeMismatch {
                op: "tiny transformer parameters readout",
                ..
            })
        ));

        Ok(())
    }

    #[test]
    fn transformer_training_state_records_updated_parameters_and_step_count() -> CtResult<()> {
        let initial_parameters = tiny_transformer_parameters()?;
        let updated_parameters = tiny_transformer_parameters()?;
        let state = TransformerTrainingState::new(initial_parameters, LearningRate::new(0.25)?);

        let next_state = state.record_updated_parameters(updated_parameters.clone());

        assert_eq!(next_state.parameters(), &updated_parameters);
        assert_eq!(next_state.learning_rate().value(), 0.25);
        assert_eq!(next_state.step_count().value(), 1);
        Ok(())
    }

    #[test]
    fn transformer_training_state_forward_uses_structured_parameters() -> CtResult<()> {
        let state =
            TransformerTrainingState::new(tiny_transformer_parameters()?, LearningRate::new(0.1)?);
        let hidden = HiddenSequence::new(vec![vec![1.0, 0.0], vec![0.0, 1.0]])?;
        let mask = AttentionMask::new(vec![vec![true, false], vec![true, true]])?;

        let logits = state.apply(Product::new(hidden, mask))?;

        assert_eq!(logits.sequence_len().value(), 2);
        assert_eq!(logits.vocab_size().value(), 3);
        assert_eq!(state.step_count().value(), 0);
        Ok(())
    }

    #[test]
    fn transformer_readout_training_example_rejects_target_length_mismatch() -> CtResult<()> {
        let hidden = HiddenSequence::new(vec![vec![1.0, 0.0], vec![0.0, 1.0]])?;
        let mask = AttentionMask::new(vec![vec![true, false], vec![true, true]])?;
        let targets = TokenSequence::from_indices([0])?;

        assert!(matches!(
            TransformerReadoutTrainingExample::new(hidden, mask, targets),
            Err(CtError::ShapeMismatch {
                op: "transformer readout training targets",
                ..
            })
        ));

        Ok(())
    }

    #[test]
    fn transformer_readout_training_example_rejects_mask_shape_mismatch() -> CtResult<()> {
        let hidden = HiddenSequence::new(vec![vec![1.0, 0.0], vec![0.0, 1.0]])?;
        let mask = AttentionMask::new(vec![vec![true, true, true], vec![true, true, true]])?;
        let targets = TokenSequence::from_indices([0, 1])?;

        assert!(matches!(
            TransformerReadoutTrainingExample::new(hidden, mask, targets),
            Err(CtError::ShapeMismatch {
                op: "transformer readout training mask",
                ..
            })
        ));

        Ok(())
    }

    #[test]
    fn transformer_readout_train_step_reduces_sequence_loss() -> CtResult<()> {
        let dataset = tiny_transformer_training_set()?;
        let state =
            TransformerTrainingState::new(tiny_transformer_parameters()?, LearningRate::new(0.5)?);
        let before = transformer_readout_average_loss(&state, &dataset)?;
        let train_step = TransformerReadoutTrainStep::new(dataset.clone());

        let trained =
            crate::category::apply_endomorphism_n_times(&train_step, state, StepCount::new(40))?;
        let after = transformer_readout_average_loss(&trained, &dataset)?;

        assert!(after.value() < before.value());
        assert_eq!(trained.step_count().value(), 40);
        Ok(())
    }

    #[test]
    fn transformer_readout_train_step_rejects_target_outside_vocabulary() -> CtResult<()> {
        let example = TransformerReadoutTrainingExample::new(
            HiddenSequence::new(vec![vec![1.0, 0.0], vec![0.0, 1.0]])?,
            AttentionMask::new(vec![vec![true, false], vec![true, true]])?,
            TokenSequence::from_indices([0, 9])?,
        )?;
        let dataset = TransformerReadoutTrainingSet::new([example])?;
        let state =
            TransformerTrainingState::new(tiny_transformer_parameters()?, LearningRate::new(0.1)?);
        let train_step = TransformerReadoutTrainStep::new(dataset);

        assert!(matches!(
            train_step.apply(state),
            Err(CtError::OutOfRange {
                kind: "sequence target",
                index: 9,
                limit: 3,
            })
        ));

        Ok(())
    }

    #[test]
    fn transformer_feed_forward_training_example_rejects_target_shape_mismatch() -> CtResult<()> {
        let input = HiddenSequence::new(vec![vec![1.0, 0.0], vec![0.0, 1.0]])?;
        let target = HiddenSequence::new(vec![vec![1.0, 0.0, 0.0], vec![0.0, 1.0, 0.0]])?;

        assert!(matches!(
            TransformerFeedForwardTrainingExample::new(input, target),
            Err(CtError::ShapeMismatch {
                op: "transformer feed-forward training dimension",
                ..
            })
        ));

        Ok(())
    }

    #[test]
    fn transformer_feed_forward_training_set_rejects_empty_input() {
        assert!(matches!(
            TransformerFeedForwardTrainingSet::new([]),
            Err(CtError::EmptyInput("transformer feed-forward training set"))
        ));
    }

    #[test]
    fn transformer_feed_forward_train_step_reduces_local_hidden_loss() -> CtResult<()> {
        let dataset = tiny_feed_forward_training_set()?;
        let state =
            TransformerTrainingState::new(tiny_transformer_parameters()?, LearningRate::new(0.2)?);
        let before = transformer_feed_forward_average_loss(&state, &dataset)?;
        let train_step = TransformerFeedForwardTrainStep::new(dataset.clone());

        let trained =
            crate::category::apply_endomorphism_n_times(&train_step, state, StepCount::new(60))?;
        let after = transformer_feed_forward_average_loss(&trained, &dataset)?;

        assert!(after.value() < before.value());
        assert_eq!(trained.step_count().value(), 60);
        Ok(())
    }

    #[test]
    fn transformer_block_training_example_rejects_mask_shape_mismatch() -> CtResult<()> {
        let hidden = HiddenSequence::new(vec![vec![1.0, 0.0], vec![0.0, 1.0]])?;
        let mask = AttentionMask::new(vec![vec![true, true, true], vec![true, true, true]])?;
        let targets = TokenSequence::from_indices([0, 1])?;

        assert!(matches!(
            TransformerBlockTrainingExample::new(hidden, mask, targets),
            Err(CtError::ShapeMismatch {
                op: "transformer block training mask",
                ..
            })
        ));

        Ok(())
    }

    #[test]
    fn transformer_block_training_set_rejects_empty_input() {
        assert!(matches!(
            TransformerBlockTrainingSet::new([]),
            Err(CtError::EmptyInput("transformer block training set"))
        ));
    }

    #[test]
    fn transformer_block_train_step_rejects_target_outside_vocabulary() -> CtResult<()> {
        let example = TransformerBlockTrainingExample::new(
            HiddenSequence::new(vec![vec![1.0, 0.0], vec![0.0, 1.0]])?,
            AttentionMask::new(vec![vec![true, false], vec![true, true]])?,
            TokenSequence::from_indices([0, 9])?,
        )?;
        let dataset = TransformerBlockTrainingSet::new([example])?;
        let state =
            TransformerTrainingState::new(tiny_transformer_parameters()?, LearningRate::new(0.1)?);
        let train_step = TransformerBlockTrainStep::new(dataset);

        assert!(matches!(
            train_step.apply(state),
            Err(CtError::OutOfRange {
                kind: "sequence target",
                index: 9,
                limit: 3,
            })
        ));

        Ok(())
    }

    #[test]
    fn transformer_block_train_step_reduces_sequence_loss() -> CtResult<()> {
        let dataset = tiny_transformer_block_training_set()?;
        let state =
            TransformerTrainingState::new(tiny_transformer_parameters()?, LearningRate::new(0.2)?);
        let before = transformer_block_average_loss(&state, &dataset)?;
        let train_step = TransformerBlockTrainStep::new(dataset.clone());

        let trained =
            crate::category::apply_endomorphism_n_times(&train_step, state, StepCount::new(40))?;
        let after = transformer_block_average_loss(&trained, &dataset)?;

        assert!(after.value() < before.value());
        assert_eq!(trained.step_count().value(), 40);
        Ok(())
    }

    #[test]
    fn transformer_block_train_step_updates_attention_output_projection() -> CtResult<()> {
        let dataset = tiny_transformer_block_training_set()?;
        let state =
            TransformerTrainingState::new(tiny_transformer_parameters()?, LearningRate::new(0.2)?);
        let before = state.parameters().output_projection().weight().to_vec();
        let train_step = TransformerBlockTrainStep::new(dataset);

        let trained = train_step.apply(state)?;
        let after = trained.parameters().output_projection().weight().to_vec();

        assert_ne!(before, after);
        assert_eq!(trained.step_count().value(), 1);
        Ok(())
    }

    #[test]
    fn transformer_block_train_step_updates_layer_norm_parameters() -> CtResult<()> {
        let dataset = tiny_transformer_block_training_set()?;
        let state =
            TransformerTrainingState::new(tiny_transformer_parameters()?, LearningRate::new(0.2)?);
        let before_attention_scale = state
            .parameters()
            .attention_norm()
            .parameters()
            .scale()
            .to_vec();
        let before_feed_forward_shift = state
            .parameters()
            .feed_forward_norm()
            .parameters()
            .shift()
            .to_vec();
        let train_step = TransformerBlockTrainStep::new(dataset);

        let trained = train_step.apply(state)?;
        let after_attention_scale = trained
            .parameters()
            .attention_norm()
            .parameters()
            .scale()
            .to_vec();
        let after_feed_forward_shift = trained
            .parameters()
            .feed_forward_norm()
            .parameters()
            .shift()
            .to_vec();

        assert_ne!(before_attention_scale, after_attention_scale);
        assert_ne!(before_feed_forward_shift, after_feed_forward_shift);
        assert_eq!(trained.step_count().value(), 1);
        Ok(())
    }

    #[test]
    fn transformer_block_train_step_updates_query_key_value_projections() -> CtResult<()> {
        let dataset = tiny_transformer_block_training_set()?;
        let state =
            TransformerTrainingState::new(tiny_transformer_parameters()?, LearningRate::new(0.2)?);
        let before_query = state
            .parameters()
            .attention_heads()
            .iter()
            .map(|head| head.query_projection().weight().to_vec())
            .collect::<Vec<_>>();
        let before_key = state
            .parameters()
            .attention_heads()
            .iter()
            .map(|head| head.key_projection().weight().to_vec())
            .collect::<Vec<_>>();
        let before_value = state
            .parameters()
            .attention_heads()
            .iter()
            .map(|head| head.value_projection().weight().to_vec())
            .collect::<Vec<_>>();
        let train_step = TransformerBlockTrainStep::new(dataset);

        let trained = train_step.apply(state)?;
        let after_query = trained
            .parameters()
            .attention_heads()
            .iter()
            .map(|head| head.query_projection().weight().to_vec())
            .collect::<Vec<_>>();
        let after_key = trained
            .parameters()
            .attention_heads()
            .iter()
            .map(|head| head.key_projection().weight().to_vec())
            .collect::<Vec<_>>();
        let after_value = trained
            .parameters()
            .attention_heads()
            .iter()
            .map(|head| head.value_projection().weight().to_vec())
            .collect::<Vec<_>>();

        assert_ne!(before_query, after_query);
        assert_ne!(before_key, after_key);
        assert_ne!(before_value, after_value);
        assert_eq!(trained.step_count().value(), 1);
        Ok(())
    }

    #[test]
    fn transformer_block_train_step_matches_finite_difference_for_attention_projection()
    -> CtResult<()> {
        let dataset = tiny_transformer_block_training_set()?;
        let state =
            TransformerTrainingState::new(tiny_transformer_parameters()?, LearningRate::new(0.2)?);
        let train_step = TransformerBlockTrainStep::new(dataset.clone());
        let trained = train_step.apply(state.clone())?;
        let selection = largest_changed_attention_projection(&state, &trained)?;
        let before_value = attention_projection_weight(&state, selection)?;
        let after_value = attention_projection_weight(&trained, selection)?;
        let inferred_gradient = (before_value - after_value) / state.learning_rate().value();
        let epsilon = 1e-3;
        let loss_plus = transformer_block_average_loss(
            &state_with_attention_projection_weight(&state, selection, before_value + epsilon)?,
            &dataset,
        )?
        .value();
        let loss_minus = transformer_block_average_loss(
            &state_with_attention_projection_weight(&state, selection, before_value - epsilon)?,
            &dataset,
        )?
        .value();
        let finite_difference = (loss_plus - loss_minus) / (2.0 * epsilon);

        assert!(
            (inferred_gradient - finite_difference).abs() < 1e-2,
            "inferred gradient {inferred_gradient} should match finite difference {finite_difference}"
        );
        Ok(())
    }

    #[test]
    fn transformer_block_train_step_matches_finite_difference_for_readout_weight() -> CtResult<()> {
        let dataset = tiny_transformer_block_training_set()?;
        let state =
            TransformerTrainingState::new(tiny_transformer_parameters()?, LearningRate::new(0.2)?);
        let train_step = TransformerBlockTrainStep::new(dataset.clone());
        let trained = train_step.apply(state.clone())?;
        let selection = largest_changed_readout_weight(&state, &trained)?;
        let before_value = readout_weight(&state, selection);
        let after_value = readout_weight(&trained, selection);

        assert_block_gradient_matches_finite_difference(
            &state,
            &dataset,
            before_value,
            after_value,
            |value| state_with_readout_weight(&state, selection, value),
        )
    }

    #[test]
    fn transformer_block_train_step_matches_finite_difference_for_feed_forward_weight()
    -> CtResult<()> {
        let dataset = tiny_transformer_block_training_set()?;
        let state =
            TransformerTrainingState::new(tiny_transformer_parameters()?, LearningRate::new(0.2)?);
        let train_step = TransformerBlockTrainStep::new(dataset.clone());
        let trained = train_step.apply(state.clone())?;
        let selection = largest_changed_feed_forward_weight(&state, &trained)?;
        let before_value = feed_forward_weight(&state, selection);
        let after_value = feed_forward_weight(&trained, selection);

        assert_block_gradient_matches_finite_difference(
            &state,
            &dataset,
            before_value,
            after_value,
            |value| state_with_feed_forward_weight(&state, selection, value),
        )
    }

    #[test]
    fn transformer_block_train_step_matches_finite_difference_for_layer_norm_parameter()
    -> CtResult<()> {
        let dataset = tiny_transformer_block_training_set()?;
        let state =
            TransformerTrainingState::new(tiny_transformer_parameters()?, LearningRate::new(0.2)?);
        let train_step = TransformerBlockTrainStep::new(dataset.clone());
        let trained = train_step.apply(state.clone())?;
        let selection = largest_changed_layer_norm_parameter(&state, &trained)?;
        let before_value = layer_norm_parameter_value(&state, selection);
        let after_value = layer_norm_parameter_value(&trained, selection);

        assert_block_gradient_matches_finite_difference(
            &state,
            &dataset,
            before_value,
            after_value,
            |value| state_with_layer_norm_parameter(&state, selection, value),
        )
    }

    #[test]
    fn transformer_block_train_step_matches_finite_difference_for_readout_bias() -> CtResult<()> {
        let dataset = tiny_transformer_block_training_set()?;
        let state =
            TransformerTrainingState::new(tiny_transformer_parameters()?, LearningRate::new(0.2)?);
        let train_step = TransformerBlockTrainStep::new(dataset.clone());
        let trained = train_step.apply(state.clone())?;
        let selection = largest_changed_vector_index(
            state.parameters().readout().bias(),
            trained.parameters().readout().bias(),
            "changed readout bias",
        )?;
        let before_value = state.parameters().readout().bias()[selection];
        let after_value = trained.parameters().readout().bias()[selection];

        assert_block_gradient_matches_finite_difference(
            &state,
            &dataset,
            before_value,
            after_value,
            |value| state_with_readout_bias(&state, selection, value),
        )
    }

    #[test]
    fn transformer_block_train_step_matches_finite_difference_for_feed_forward_bias() -> CtResult<()>
    {
        let dataset = tiny_transformer_block_training_set()?;
        let state =
            TransformerTrainingState::new(tiny_transformer_parameters()?, LearningRate::new(0.2)?);
        let train_step = TransformerBlockTrainStep::new(dataset.clone());
        let trained = train_step.apply(state.clone())?;
        let selection = largest_changed_feed_forward_bias(&state, &trained)?;
        let before_value = feed_forward_bias(&state, selection);
        let after_value = feed_forward_bias(&trained, selection);

        assert_block_gradient_matches_finite_difference(
            &state,
            &dataset,
            before_value,
            after_value,
            |value| state_with_feed_forward_bias(&state, selection, value),
        )
    }

    #[test]
    fn transformer_block_train_step_matches_finite_difference_for_output_projection_bias()
    -> CtResult<()> {
        let dataset = tiny_transformer_block_training_set()?;
        let state =
            TransformerTrainingState::new(tiny_transformer_parameters()?, LearningRate::new(0.2)?);
        let train_step = TransformerBlockTrainStep::new(dataset.clone());
        let trained = train_step.apply(state.clone())?;
        let selection = largest_changed_vector_index(
            state.parameters().output_projection().bias(),
            trained.parameters().output_projection().bias(),
            "changed attention output projection bias",
        )?;
        let before_value = state.parameters().output_projection().bias()[selection];
        let after_value = trained.parameters().output_projection().bias()[selection];

        assert_block_gradient_matches_finite_difference(
            &state,
            &dataset,
            before_value,
            after_value,
            |value| state_with_output_projection_bias(&state, selection, value),
        )
    }

    #[test]
    fn transformer_block_train_step_matches_finite_difference_for_attention_projection_bias()
    -> CtResult<()> {
        let dataset = tiny_transformer_block_training_set()?;
        let state =
            TransformerTrainingState::new(tiny_transformer_parameters()?, LearningRate::new(0.2)?);
        let train_step = TransformerBlockTrainStep::new(dataset.clone());
        let trained = train_step.apply(state.clone())?;
        let selection = largest_changed_attention_projection_bias(&state, &trained)?;
        let before_value = attention_projection_bias(&state, selection)?;
        let after_value = attention_projection_bias(&trained, selection)?;

        assert_block_gradient_matches_finite_difference(
            &state,
            &dataset,
            before_value,
            after_value,
            |value| state_with_attention_projection_bias(&state, selection, value),
        )
    }

    fn assert_block_gradient_matches_finite_difference(
        state: &TransformerTrainingState,
        dataset: &TransformerBlockTrainingSet,
        before_value: f32,
        after_value: f32,
        mut state_with_value: impl FnMut(f32) -> CtResult<TransformerTrainingState>,
    ) -> CtResult<()> {
        let inferred_gradient = (before_value - after_value) / state.learning_rate().value();
        let epsilon = 1e-3;
        let loss_plus =
            transformer_block_average_loss(&state_with_value(before_value + epsilon)?, dataset)?
                .value();
        let loss_minus =
            transformer_block_average_loss(&state_with_value(before_value - epsilon)?, dataset)?
                .value();
        let finite_difference = (loss_plus - loss_minus) / (2.0 * epsilon);

        assert!(
            (inferred_gradient - finite_difference).abs() < 1e-2,
            "inferred gradient {inferred_gradient} should match finite difference {finite_difference}"
        );
        Ok(())
    }

    fn largest_changed_vector_index(
        before: &[f32],
        after: &[f32],
        label: &'static str,
    ) -> CtResult<usize> {
        let mut selected = None;
        let mut largest_delta = 0.0;

        for (index, (before_value, after_value)) in before.iter().zip(after).enumerate() {
            let delta = (before_value - after_value).abs();

            if delta > largest_delta {
                largest_delta = delta;
                selected = Some(index);
            }
        }

        selected.ok_or(CtError::EmptyInput(label))
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct MatrixSelection {
        input_index: usize,
        output_index: usize,
    }

    fn largest_changed_matrix_weight(
        before: &[Vec<f32>],
        after: &[Vec<f32>],
        label: &'static str,
    ) -> CtResult<MatrixSelection> {
        let mut selected = None;
        let mut largest_delta = 0.0;

        for (input_index, (before_row, after_row)) in before.iter().zip(after).enumerate() {
            for (output_index, (before_value, after_value)) in
                before_row.iter().zip(after_row).enumerate()
            {
                let delta = (before_value - after_value).abs();

                if delta > largest_delta {
                    largest_delta = delta;
                    selected = Some(MatrixSelection {
                        input_index,
                        output_index,
                    });
                }
            }
        }

        selected.ok_or(CtError::EmptyInput(label))
    }

    fn largest_changed_readout_weight(
        before: &TransformerTrainingState,
        after: &TransformerTrainingState,
    ) -> CtResult<MatrixSelection> {
        largest_changed_matrix_weight(
            before.parameters().readout().weight(),
            after.parameters().readout().weight(),
            "changed readout weight",
        )
    }

    fn readout_weight(state: &TransformerTrainingState, selection: MatrixSelection) -> f32 {
        state.parameters().readout().weight()[selection.input_index][selection.output_index]
    }

    fn state_with_readout_weight(
        state: &TransformerTrainingState,
        selection: MatrixSelection,
        value: f32,
    ) -> CtResult<TransformerTrainingState> {
        let readout = state.parameters().readout();
        let mut weight = readout.weight().to_vec();
        weight[selection.input_index][selection.output_index] = value;
        let readout = TransformerReadout::new(weight, readout.bias().to_vec())?;
        let parameters = state.parameters().clone().with_readout(readout)?;

        Ok(TransformerTrainingState::from_parts(
            parameters,
            state.learning_rate(),
            state.step_count(),
        ))
    }

    fn state_with_readout_bias(
        state: &TransformerTrainingState,
        selection: usize,
        value: f32,
    ) -> CtResult<TransformerTrainingState> {
        let readout = state.parameters().readout();
        let mut bias = readout.bias().to_vec();
        bias[selection] = value;
        let readout = TransformerReadout::new(readout.weight().to_vec(), bias)?;
        let parameters = state.parameters().clone().with_readout(readout)?;

        Ok(TransformerTrainingState::from_parts(
            parameters,
            state.learning_rate(),
            state.step_count(),
        ))
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum FeedForwardWeightKind {
        First,
        Second,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct FeedForwardWeightSelection {
        kind: FeedForwardWeightKind,
        matrix: MatrixSelection,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct FeedForwardBiasSelection {
        kind: FeedForwardWeightKind,
        index: usize,
    }

    fn largest_changed_feed_forward_weight(
        before: &TransformerTrainingState,
        after: &TransformerTrainingState,
    ) -> CtResult<FeedForwardWeightSelection> {
        let first = largest_changed_matrix_weight(
            before.parameters().feed_forward().first_weight(),
            after.parameters().feed_forward().first_weight(),
            "changed first feed-forward weight",
        );
        let second = largest_changed_matrix_weight(
            before.parameters().feed_forward().second_weight(),
            after.parameters().feed_forward().second_weight(),
            "changed second feed-forward weight",
        );

        match (first, second) {
            (Ok(first), Ok(second)) => {
                let first_delta = feed_forward_weight_delta(
                    before,
                    after,
                    FeedForwardWeightSelection {
                        kind: FeedForwardWeightKind::First,
                        matrix: first,
                    },
                );
                let second_delta = feed_forward_weight_delta(
                    before,
                    after,
                    FeedForwardWeightSelection {
                        kind: FeedForwardWeightKind::Second,
                        matrix: second,
                    },
                );

                if first_delta >= second_delta {
                    Ok(FeedForwardWeightSelection {
                        kind: FeedForwardWeightKind::First,
                        matrix: first,
                    })
                } else {
                    Ok(FeedForwardWeightSelection {
                        kind: FeedForwardWeightKind::Second,
                        matrix: second,
                    })
                }
            }
            (Ok(first), Err(_)) => Ok(FeedForwardWeightSelection {
                kind: FeedForwardWeightKind::First,
                matrix: first,
            }),
            (Err(_), Ok(second)) => Ok(FeedForwardWeightSelection {
                kind: FeedForwardWeightKind::Second,
                matrix: second,
            }),
            (Err(_), Err(_)) => Err(CtError::EmptyInput("changed feed-forward weight")),
        }
    }

    fn feed_forward_weight_delta(
        before: &TransformerTrainingState,
        after: &TransformerTrainingState,
        selection: FeedForwardWeightSelection,
    ) -> f32 {
        (feed_forward_weight(before, selection) - feed_forward_weight(after, selection)).abs()
    }

    fn feed_forward_weight(
        state: &TransformerTrainingState,
        selection: FeedForwardWeightSelection,
    ) -> f32 {
        let feed_forward = state.parameters().feed_forward();

        match selection.kind {
            FeedForwardWeightKind::First => feed_forward.first_weight()
                [selection.matrix.input_index][selection.matrix.output_index],
            FeedForwardWeightKind::Second => feed_forward.second_weight()
                [selection.matrix.input_index][selection.matrix.output_index],
        }
    }

    fn state_with_feed_forward_weight(
        state: &TransformerTrainingState,
        selection: FeedForwardWeightSelection,
        value: f32,
    ) -> CtResult<TransformerTrainingState> {
        let feed_forward = state.parameters().feed_forward();
        let mut first_weight = feed_forward.first_weight().to_vec();
        let mut second_weight = feed_forward.second_weight().to_vec();

        match selection.kind {
            FeedForwardWeightKind::First => {
                first_weight[selection.matrix.input_index][selection.matrix.output_index] = value;
            }
            FeedForwardWeightKind::Second => {
                second_weight[selection.matrix.input_index][selection.matrix.output_index] = value;
            }
        }

        let feed_forward = PositionWiseFeedForward::new(
            first_weight,
            feed_forward.first_bias().to_vec(),
            second_weight,
            feed_forward.second_bias().to_vec(),
        )?;
        let parameters = state.parameters().clone().with_feed_forward(feed_forward)?;

        Ok(TransformerTrainingState::from_parts(
            parameters,
            state.learning_rate(),
            state.step_count(),
        ))
    }

    fn largest_changed_feed_forward_bias(
        before: &TransformerTrainingState,
        after: &TransformerTrainingState,
    ) -> CtResult<FeedForwardBiasSelection> {
        let first = largest_changed_vector_index(
            before.parameters().feed_forward().first_bias(),
            after.parameters().feed_forward().first_bias(),
            "changed first feed-forward bias",
        );
        let second = largest_changed_vector_index(
            before.parameters().feed_forward().second_bias(),
            after.parameters().feed_forward().second_bias(),
            "changed second feed-forward bias",
        );

        match (first, second) {
            (Ok(first), Ok(second)) => {
                let first_delta = feed_forward_bias_delta(
                    before,
                    after,
                    FeedForwardBiasSelection {
                        kind: FeedForwardWeightKind::First,
                        index: first,
                    },
                );
                let second_delta = feed_forward_bias_delta(
                    before,
                    after,
                    FeedForwardBiasSelection {
                        kind: FeedForwardWeightKind::Second,
                        index: second,
                    },
                );

                if first_delta >= second_delta {
                    Ok(FeedForwardBiasSelection {
                        kind: FeedForwardWeightKind::First,
                        index: first,
                    })
                } else {
                    Ok(FeedForwardBiasSelection {
                        kind: FeedForwardWeightKind::Second,
                        index: second,
                    })
                }
            }
            (Ok(first), Err(_)) => Ok(FeedForwardBiasSelection {
                kind: FeedForwardWeightKind::First,
                index: first,
            }),
            (Err(_), Ok(second)) => Ok(FeedForwardBiasSelection {
                kind: FeedForwardWeightKind::Second,
                index: second,
            }),
            (Err(_), Err(_)) => Err(CtError::EmptyInput("changed feed-forward bias")),
        }
    }

    fn feed_forward_bias_delta(
        before: &TransformerTrainingState,
        after: &TransformerTrainingState,
        selection: FeedForwardBiasSelection,
    ) -> f32 {
        (feed_forward_bias(before, selection) - feed_forward_bias(after, selection)).abs()
    }

    fn feed_forward_bias(
        state: &TransformerTrainingState,
        selection: FeedForwardBiasSelection,
    ) -> f32 {
        let feed_forward = state.parameters().feed_forward();

        match selection.kind {
            FeedForwardWeightKind::First => feed_forward.first_bias()[selection.index],
            FeedForwardWeightKind::Second => feed_forward.second_bias()[selection.index],
        }
    }

    fn state_with_feed_forward_bias(
        state: &TransformerTrainingState,
        selection: FeedForwardBiasSelection,
        value: f32,
    ) -> CtResult<TransformerTrainingState> {
        let feed_forward = state.parameters().feed_forward();
        let mut first_bias = feed_forward.first_bias().to_vec();
        let mut second_bias = feed_forward.second_bias().to_vec();

        match selection.kind {
            FeedForwardWeightKind::First => {
                first_bias[selection.index] = value;
            }
            FeedForwardWeightKind::Second => {
                second_bias[selection.index] = value;
            }
        }

        let feed_forward = PositionWiseFeedForward::new(
            feed_forward.first_weight().to_vec(),
            first_bias,
            feed_forward.second_weight().to_vec(),
            second_bias,
        )?;
        let parameters = state.parameters().clone().with_feed_forward(feed_forward)?;

        Ok(TransformerTrainingState::from_parts(
            parameters,
            state.learning_rate(),
            state.step_count(),
        ))
    }

    fn state_with_output_projection_bias(
        state: &TransformerTrainingState,
        selection: usize,
        value: f32,
    ) -> CtResult<TransformerTrainingState> {
        let output_projection = state.parameters().output_projection();
        let mut bias = output_projection.bias().to_vec();
        bias[selection] = value;
        let output_projection =
            AttentionOutputProjection::new(output_projection.weight().to_vec(), bias)?;
        let parameters = state
            .parameters()
            .clone()
            .with_output_projection(output_projection)?;

        Ok(TransformerTrainingState::from_parts(
            parameters,
            state.learning_rate(),
            state.step_count(),
        ))
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum LayerNormParameterKind {
        AttentionScale,
        AttentionShift,
        FeedForwardScale,
        FeedForwardShift,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct LayerNormParameterSelection {
        kind: LayerNormParameterKind,
        feature_index: usize,
    }

    fn largest_changed_layer_norm_parameter(
        before: &TransformerTrainingState,
        after: &TransformerTrainingState,
    ) -> CtResult<LayerNormParameterSelection> {
        let mut selected = None;
        let mut largest_delta = 0.0;

        for kind in [
            LayerNormParameterKind::AttentionScale,
            LayerNormParameterKind::AttentionShift,
            LayerNormParameterKind::FeedForwardScale,
            LayerNormParameterKind::FeedForwardShift,
        ] {
            let before_values = layer_norm_parameter_values(before, kind);
            let after_values = layer_norm_parameter_values(after, kind);

            for (feature_index, (before_value, after_value)) in
                before_values.iter().zip(after_values).enumerate()
            {
                let delta = (before_value - after_value).abs();

                if delta > largest_delta {
                    largest_delta = delta;
                    selected = Some(LayerNormParameterSelection {
                        kind,
                        feature_index,
                    });
                }
            }
        }

        selected.ok_or(CtError::EmptyInput("changed layer norm parameter"))
    }

    fn layer_norm_parameter_values(
        state: &TransformerTrainingState,
        kind: LayerNormParameterKind,
    ) -> &[f32] {
        match kind {
            LayerNormParameterKind::AttentionScale => {
                state.parameters().attention_norm().parameters().scale()
            }
            LayerNormParameterKind::AttentionShift => {
                state.parameters().attention_norm().parameters().shift()
            }
            LayerNormParameterKind::FeedForwardScale => {
                state.parameters().feed_forward_norm().parameters().scale()
            }
            LayerNormParameterKind::FeedForwardShift => {
                state.parameters().feed_forward_norm().parameters().shift()
            }
        }
    }

    fn layer_norm_parameter_value(
        state: &TransformerTrainingState,
        selection: LayerNormParameterSelection,
    ) -> f32 {
        layer_norm_parameter_values(state, selection.kind)[selection.feature_index]
    }

    fn state_with_layer_norm_parameter(
        state: &TransformerTrainingState,
        selection: LayerNormParameterSelection,
        value: f32,
    ) -> CtResult<TransformerTrainingState> {
        let attention_parameters = state.parameters().attention_norm().parameters();
        let feed_forward_parameters = state.parameters().feed_forward_norm().parameters();
        let mut attention_scale = attention_parameters.scale().to_vec();
        let mut attention_shift = attention_parameters.shift().to_vec();
        let mut feed_forward_scale = feed_forward_parameters.scale().to_vec();
        let mut feed_forward_shift = feed_forward_parameters.shift().to_vec();

        match selection.kind {
            LayerNormParameterKind::AttentionScale => {
                attention_scale[selection.feature_index] = value;
            }
            LayerNormParameterKind::AttentionShift => {
                attention_shift[selection.feature_index] = value;
            }
            LayerNormParameterKind::FeedForwardScale => {
                feed_forward_scale[selection.feature_index] = value;
            }
            LayerNormParameterKind::FeedForwardShift => {
                feed_forward_shift[selection.feature_index] = value;
            }
        }

        let attention_norm = LayerNormalization::new(LayerNormParameters::new(
            attention_scale,
            attention_shift,
            attention_parameters.epsilon(),
        )?);
        let feed_forward_norm = LayerNormalization::new(LayerNormParameters::new(
            feed_forward_scale,
            feed_forward_shift,
            feed_forward_parameters.epsilon(),
        )?);
        let parameters = state
            .parameters()
            .clone()
            .with_layer_norms(attention_norm, feed_forward_norm)?;

        Ok(TransformerTrainingState::from_parts(
            parameters,
            state.learning_rate(),
            state.step_count(),
        ))
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum AttentionProjectionKind {
        Query,
        Key,
        Value,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct AttentionProjectionSelection {
        head_index: usize,
        kind: AttentionProjectionKind,
        input_index: usize,
        output_index: usize,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct AttentionProjectionBiasSelection {
        head_index: usize,
        kind: AttentionProjectionKind,
        output_index: usize,
    }

    fn largest_changed_attention_projection(
        before: &TransformerTrainingState,
        after: &TransformerTrainingState,
    ) -> CtResult<AttentionProjectionSelection> {
        let head_count = before.parameters().attention_heads().len();
        let mut selected = None;
        let mut largest_delta = 0.0;

        for head_index in 0..head_count {
            for kind in [
                AttentionProjectionKind::Query,
                AttentionProjectionKind::Key,
                AttentionProjectionKind::Value,
            ] {
                let before_weight = attention_projection_weight_matrix(before, head_index, kind)?;
                let after_weight = attention_projection_weight_matrix(after, head_index, kind)?;

                for (input_index, (before_row, after_row)) in
                    before_weight.iter().zip(after_weight).enumerate()
                {
                    for (output_index, (before_value, after_value)) in
                        before_row.iter().zip(after_row).enumerate()
                    {
                        let delta = (before_value - after_value).abs();

                        if delta > largest_delta {
                            largest_delta = delta;
                            selected = Some(AttentionProjectionSelection {
                                head_index,
                                kind,
                                input_index,
                                output_index,
                            });
                        }
                    }
                }
            }
        }

        selected.ok_or(CtError::EmptyInput("changed attention projection"))
    }

    fn attention_projection_weight(
        state: &TransformerTrainingState,
        selection: AttentionProjectionSelection,
    ) -> CtResult<f32> {
        let weight =
            attention_projection_weight_matrix(state, selection.head_index, selection.kind)?;

        Ok(weight[selection.input_index][selection.output_index])
    }

    fn attention_projection_weight_matrix(
        state: &TransformerTrainingState,
        head_index: usize,
        kind: AttentionProjectionKind,
    ) -> CtResult<&[Vec<f32>]> {
        let head =
            state
                .parameters()
                .attention_heads()
                .get(head_index)
                .ok_or(CtError::OutOfRange {
                    kind: "attention head",
                    index: head_index,
                    limit: state.parameters().attention_heads().len(),
                })?;

        Ok(match kind {
            AttentionProjectionKind::Query => head.query_projection().weight(),
            AttentionProjectionKind::Key => head.key_projection().weight(),
            AttentionProjectionKind::Value => head.value_projection().weight(),
        })
    }

    fn largest_changed_attention_projection_bias(
        before: &TransformerTrainingState,
        after: &TransformerTrainingState,
    ) -> CtResult<AttentionProjectionBiasSelection> {
        let head_count = before.parameters().attention_heads().len();
        let mut selected = None;
        let mut largest_delta = 0.0;

        for head_index in 0..head_count {
            for kind in [
                AttentionProjectionKind::Query,
                AttentionProjectionKind::Key,
                AttentionProjectionKind::Value,
            ] {
                let before_bias = attention_projection_bias_values(before, head_index, kind)?;
                let after_bias = attention_projection_bias_values(after, head_index, kind)?;

                for (output_index, (before_value, after_value)) in
                    before_bias.iter().zip(after_bias).enumerate()
                {
                    let delta = (before_value - after_value).abs();

                    if delta > largest_delta {
                        largest_delta = delta;
                        selected = Some(AttentionProjectionBiasSelection {
                            head_index,
                            kind,
                            output_index,
                        });
                    }
                }
            }
        }

        selected.ok_or(CtError::EmptyInput("changed attention projection bias"))
    }

    fn attention_projection_bias(
        state: &TransformerTrainingState,
        selection: AttentionProjectionBiasSelection,
    ) -> CtResult<f32> {
        let bias = attention_projection_bias_values(state, selection.head_index, selection.kind)?;

        Ok(bias[selection.output_index])
    }

    fn attention_projection_bias_values(
        state: &TransformerTrainingState,
        head_index: usize,
        kind: AttentionProjectionKind,
    ) -> CtResult<&[f32]> {
        let head =
            state
                .parameters()
                .attention_heads()
                .get(head_index)
                .ok_or(CtError::OutOfRange {
                    kind: "attention head",
                    index: head_index,
                    limit: state.parameters().attention_heads().len(),
                })?;

        Ok(match kind {
            AttentionProjectionKind::Query => head.query_projection().bias(),
            AttentionProjectionKind::Key => head.key_projection().bias(),
            AttentionProjectionKind::Value => head.value_projection().bias(),
        })
    }

    fn state_with_attention_projection_weight(
        state: &TransformerTrainingState,
        selection: AttentionProjectionSelection,
        value: f32,
    ) -> CtResult<TransformerTrainingState> {
        let mut heads = state.parameters().attention_heads().to_vec();
        let head = heads.get(selection.head_index).ok_or(CtError::OutOfRange {
            kind: "attention head",
            index: selection.head_index,
            limit: heads.len(),
        })?;
        let mut query_weight = head.query_projection().weight().to_vec();
        let mut key_weight = head.key_projection().weight().to_vec();
        let mut value_weight = head.value_projection().weight().to_vec();

        match selection.kind {
            AttentionProjectionKind::Query => {
                query_weight[selection.input_index][selection.output_index] = value;
            }
            AttentionProjectionKind::Key => {
                key_weight[selection.input_index][selection.output_index] = value;
            }
            AttentionProjectionKind::Value => {
                value_weight[selection.input_index][selection.output_index] = value;
            }
        }

        heads[selection.head_index] = SelfAttentionHead::new(
            HiddenToQuery::new(query_weight, head.query_projection().bias().to_vec())?,
            HiddenToKey::new(key_weight, head.key_projection().bias().to_vec())?,
            HiddenToValue::new(value_weight, head.value_projection().bias().to_vec())?,
        )?;
        let parameters = state.parameters().clone().with_attention_heads(heads)?;

        Ok(TransformerTrainingState::from_parts(
            parameters,
            state.learning_rate(),
            state.step_count(),
        ))
    }

    fn state_with_attention_projection_bias(
        state: &TransformerTrainingState,
        selection: AttentionProjectionBiasSelection,
        value: f32,
    ) -> CtResult<TransformerTrainingState> {
        let mut heads = state.parameters().attention_heads().to_vec();
        let head = heads.get(selection.head_index).ok_or(CtError::OutOfRange {
            kind: "attention head",
            index: selection.head_index,
            limit: heads.len(),
        })?;
        let mut query_bias = head.query_projection().bias().to_vec();
        let mut key_bias = head.key_projection().bias().to_vec();
        let mut value_bias = head.value_projection().bias().to_vec();

        match selection.kind {
            AttentionProjectionKind::Query => {
                query_bias[selection.output_index] = value;
            }
            AttentionProjectionKind::Key => {
                key_bias[selection.output_index] = value;
            }
            AttentionProjectionKind::Value => {
                value_bias[selection.output_index] = value;
            }
        }

        heads[selection.head_index] = SelfAttentionHead::new(
            HiddenToQuery::new(head.query_projection().weight().to_vec(), query_bias)?,
            HiddenToKey::new(head.key_projection().weight().to_vec(), key_bias)?,
            HiddenToValue::new(head.value_projection().weight().to_vec(), value_bias)?,
        )?;
        let parameters = state.parameters().clone().with_attention_heads(heads)?;

        Ok(TransformerTrainingState::from_parts(
            parameters,
            state.learning_rate(),
            state.step_count(),
        ))
    }

    fn tiny_single_head_block() -> CtResult<SingleHeadTransformerBlock> {
        let query = HiddenToQuery::new(vec![vec![1.0, 0.0], vec![0.0, 1.0]], vec![0.0, 0.0])?;
        let key = HiddenToKey::new(vec![vec![1.0, 0.0], vec![0.0, 1.0]], vec![0.0, 0.0])?;
        let value = HiddenToValue::new(vec![vec![1.0, 0.0], vec![0.0, 1.0]], vec![0.0, 0.0])?;
        let output_projection =
            AttentionOutputProjection::new(vec![vec![1.0, 0.0], vec![0.0, 1.0]], vec![0.0, 0.0])?;
        let model_dimension = ModelDimension::new(2)?;
        let attention_norm =
            LayerNormalization::new(LayerNormParameters::identity(model_dimension));
        let feed_forward = PositionWiseFeedForward::new(
            vec![vec![1.0, 0.0], vec![0.0, 1.0]],
            vec![0.0, 0.0],
            vec![vec![1.0, 0.0], vec![0.0, 1.0]],
            vec![0.0, 0.0],
        )?;
        let feed_forward_norm =
            LayerNormalization::new(LayerNormParameters::identity(model_dimension));

        SingleHeadTransformerBlock::new(
            query,
            key,
            value,
            output_projection,
            attention_norm,
            feed_forward,
            feed_forward_norm,
        )
    }

    fn tiny_multi_head_block() -> CtResult<MultiHeadTransformerBlock> {
        let model_dimension = ModelDimension::new(2)?;

        MultiHeadTransformerBlock::new(
            vec![
                tiny_self_attention_head_first_feature()?,
                tiny_self_attention_head_second_feature()?,
            ],
            AttentionOutputProjection::new(vec![vec![1.0, 0.0], vec![0.0, 1.0]], vec![0.0, 0.0])?,
            LayerNormalization::new(LayerNormParameters::identity(model_dimension)),
            identity_feed_forward()?,
            LayerNormalization::new(LayerNormParameters::identity(model_dimension)),
        )
    }

    fn tiny_masked_multi_head_block() -> CtResult<MaskedMultiHeadTransformerBlock> {
        let model_dimension = ModelDimension::new(2)?;

        MaskedMultiHeadTransformerBlock::new(
            vec![
                tiny_self_attention_head_first_feature()?,
                tiny_self_attention_head_second_feature()?,
            ],
            AttentionOutputProjection::new(vec![vec![1.0, 0.0], vec![0.0, 1.0]], vec![0.0, 0.0])?,
            LayerNormalization::new(LayerNormParameters::identity(model_dimension)),
            identity_feed_forward()?,
            LayerNormalization::new(LayerNormParameters::identity(model_dimension)),
        )
    }

    fn tiny_transformer_parameters() -> CtResult<TinyTransformerParameters> {
        TinyTransformerParameters::new(
            PositionalEncoding::new(vec![vec![0.1, 0.0], vec![0.0, 0.1]])?,
            tiny_masked_multi_head_block()?,
            TransformerReadout::new(
                vec![vec![1.0, 0.0, 0.5], vec![0.0, 1.0, -0.5]],
                vec![0.0, 0.0, 0.0],
            )?,
        )
    }

    fn tiny_transformer_training_set() -> CtResult<TransformerReadoutTrainingSet> {
        let example = TransformerReadoutTrainingExample::new(
            HiddenSequence::new(vec![vec![1.0, 0.0], vec![0.0, 1.0]])?,
            AttentionMask::new(vec![vec![true, false], vec![true, true]])?,
            TokenSequence::from_indices([0, 1])?,
        )?;

        TransformerReadoutTrainingSet::new([example])
    }

    fn tiny_feed_forward_training_set() -> CtResult<TransformerFeedForwardTrainingSet> {
        let example = TransformerFeedForwardTrainingExample::new(
            HiddenSequence::new(vec![vec![1.0, 0.0], vec![0.0, 1.0]])?,
            HiddenSequence::new(vec![vec![2.0, 0.0], vec![0.0, 2.0]])?,
        )?;

        TransformerFeedForwardTrainingSet::new([example])
    }

    fn tiny_transformer_block_training_set() -> CtResult<TransformerBlockTrainingSet> {
        let example = TransformerBlockTrainingExample::new(
            HiddenSequence::new(vec![vec![1.0, 0.0], vec![0.0, 1.0]])?,
            AttentionMask::new(vec![vec![true, false], vec![true, true]])?,
            TokenSequence::from_indices([0, 1])?,
        )?;

        TransformerBlockTrainingSet::new([example])
    }

    fn tiny_self_attention_head_first_feature() -> CtResult<SelfAttentionHead> {
        SelfAttentionHead::new(
            HiddenToQuery::new(vec![vec![1.0], vec![0.0]], vec![0.0])?,
            HiddenToKey::new(vec![vec![1.0], vec![0.0]], vec![0.0])?,
            HiddenToValue::new(vec![vec![1.0], vec![0.0]], vec![0.0])?,
        )
    }

    fn tiny_self_attention_head_second_feature() -> CtResult<SelfAttentionHead> {
        SelfAttentionHead::new(
            HiddenToQuery::new(vec![vec![0.0], vec![1.0]], vec![0.0])?,
            HiddenToKey::new(vec![vec![0.0], vec![1.0]], vec![0.0])?,
            HiddenToValue::new(vec![vec![0.0], vec![1.0]], vec![0.0])?,
        )
    }

    fn identity_feed_forward() -> CtResult<PositionWiseFeedForward> {
        PositionWiseFeedForward::new(
            vec![vec![1.0, 0.0], vec![0.0, 1.0]],
            vec![0.0, 0.0],
            vec![vec![1.0, 0.0], vec![0.0, 1.0]],
            vec![0.0, 0.0],
        )
    }
}
