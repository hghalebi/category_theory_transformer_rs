use category_theory_transformer_rs::{
    AttentionHeadOutputs, AttentionMask, AttentionOutput, AttentionOutputProjection,
    AttentionSoftmax, ConcatenateHeads, CtResult, HiddenSequence, HiddenToKey, HiddenToQuery,
    HiddenToValue, KeySequence, LayerNormParameters, LayerNormalization, LearningRate,
    MaskedAttentionScores, MaskedMultiHeadTransformerBlock, Morphism, MultiHeadTransformerBlock,
    PositionWiseFeedForward, PositionalEncoding, Product, QuerySequence, ResidualConnection,
    ScaledDotProductScores, SelfAttentionHead, SingleHeadTransformerBlock,
    TinyTransformerParameters, TokenSequence, TransformerBlockTrainStep,
    TransformerBlockTrainingExample, TransformerBlockTrainingSet, TransformerFeedForwardTrainStep,
    TransformerFeedForwardTrainingExample, TransformerFeedForwardTrainingSet, TransformerReadout,
    TransformerReadoutTrainStep, TransformerReadoutTrainingExample, TransformerReadoutTrainingSet,
    TransformerTrainingState, ValueSequence, WeightedValueMixing, transformer_block_average_loss,
    transformer_feed_forward_average_loss, transformer_readout_average_loss,
};

fn main() -> CtResult<()> {
    let queries = QuerySequence::new(vec![vec![1.0, 0.0], vec![0.0, 1.0]])?;
    let keys = KeySequence::new(vec![vec![1.0, 0.0], vec![0.0, 1.0], vec![1.0, 1.0]])?;
    let values = ValueSequence::new(vec![vec![1.0, 10.0], vec![2.0, 20.0], vec![3.0, 30.0]])?;
    let mask = AttentionMask::new(vec![vec![true, false, true], vec![true, true, true]])?;

    let scores = ScaledDotProductScores.apply(Product::new(queries, keys))?;
    let masked_scores = MaskedAttentionScores.apply(Product::new(scores, mask))?;
    let weights = AttentionSoftmax.apply(masked_scores)?;

    println!(
        "attention shape: {} query positions x {} key positions",
        weights.query_len().value(),
        weights.key_len().value()
    );

    for (query_position, row) in weights.rows().iter().enumerate() {
        println!("query {query_position} attends with {:?}", row.as_slice());
    }

    let output = WeightedValueMixing.apply(Product::new(weights, values))?;

    for (query_position, row) in output.rows().iter().enumerate() {
        println!("query {query_position} output vector {:?}", row.as_slice());
    }

    let second_head = AttentionOutput::new(vec![vec![10.0, 1.0], vec![20.0, 2.0]])?;
    let head_outputs = AttentionHeadOutputs::new(vec![output, second_head])?;
    let multi_head = ConcatenateHeads.apply(head_outputs)?;

    println!(
        "multi-head shape: {} heads x {} features -> model dimension {}",
        multi_head.head_count().value(),
        multi_head.head_dimension().value(),
        multi_head.model_dimension().value()
    );

    for (query_position, row) in multi_head.rows().iter().enumerate() {
        println!("query {query_position} multi-head row {:?}", row.as_slice());
    }

    let output_projection = AttentionOutputProjection::new(
        vec![
            vec![1.0, 0.0],
            vec![0.0, 0.1],
            vec![0.5, 0.0],
            vec![0.0, 1.0],
        ],
        vec![0.0, 0.0],
    )?;
    let projected = output_projection.apply(multi_head)?;

    println!(
        "projected attention shape: {} positions x model dimension {}",
        projected.sequence_len().value(),
        projected.model_dimension().value()
    );

    for (query_position, row) in projected.rows().iter().enumerate() {
        println!(
            "query {query_position} projected attention row {:?}",
            row.as_slice()
        );
    }

    let hidden_input = HiddenSequence::new(vec![vec![0.5, 0.5], vec![1.0, 1.0]])?;
    let residual = ResidualConnection.apply(Product::new(hidden_input, projected))?;

    println!(
        "residual shape: {} positions x model dimension {}",
        residual.sequence_len().value(),
        residual.model_dimension().value()
    );

    for (query_position, row) in residual.rows().iter().enumerate() {
        println!("query {query_position} residual row {:?}", row.as_slice());
    }

    let layer_norm =
        LayerNormalization::new(LayerNormParameters::identity(residual.model_dimension()));
    let normalized = layer_norm.apply(residual)?;

    println!(
        "normalized shape: {} positions x model dimension {}",
        normalized.sequence_len().value(),
        normalized.model_dimension().value()
    );

    for (query_position, row) in normalized.rows().iter().enumerate() {
        println!("query {query_position} normalized row {:?}", row.as_slice());
    }

    let feed_forward = PositionWiseFeedForward::new(
        vec![vec![1.0, -1.0, 0.5], vec![0.0, 1.0, 0.5]],
        vec![0.0, 0.0, 0.0],
        vec![vec![1.0, 0.0], vec![0.0, 1.0], vec![0.5, 0.5]],
        vec![0.0, 0.0],
    )?;
    let fed_forward = feed_forward.apply(normalized)?;

    println!(
        "feed-forward shape: {} positions x model dimension {}",
        fed_forward.sequence_len().value(),
        fed_forward.model_dimension().value()
    );

    for (query_position, row) in fed_forward.rows().iter().enumerate() {
        println!(
            "query {query_position} feed-forward row {:?}",
            row.as_slice()
        );
    }

    let positional_encoding = PositionalEncoding::new(vec![vec![0.1, 0.0], vec![0.0, 0.1]])?;
    let positioned_hidden =
        positional_encoding.apply(HiddenSequence::new(vec![vec![1.0, 0.0], vec![0.0, 1.0]])?)?;

    println!(
        "positioned hidden shape: {} positions x model dimension {}",
        positioned_hidden.sequence_len().value(),
        positioned_hidden.model_dimension().value()
    );

    let block = SingleHeadTransformerBlock::new(
        HiddenToQuery::new(vec![vec![1.0, 0.0], vec![0.0, 1.0]], vec![0.0, 0.0])?,
        HiddenToKey::new(vec![vec![1.0, 0.0], vec![0.0, 1.0]], vec![0.0, 0.0])?,
        HiddenToValue::new(vec![vec![1.0, 0.0], vec![0.0, 1.0]], vec![0.0, 0.0])?,
        AttentionOutputProjection::new(vec![vec![1.0, 0.0], vec![0.0, 1.0]], vec![0.0, 0.0])?,
        LayerNormalization::new(LayerNormParameters::identity(fed_forward.model_dimension())),
        PositionWiseFeedForward::new(
            vec![vec![1.0, 0.0], vec![0.0, 1.0]],
            vec![0.0, 0.0],
            vec![vec![1.0, 0.0], vec![0.0, 1.0]],
            vec![0.0, 0.0],
        )?,
        LayerNormalization::new(LayerNormParameters::identity(fed_forward.model_dimension())),
    )?;
    let block_output = block.apply(positioned_hidden.clone())?;

    println!(
        "single-head block shape: {} positions x model dimension {}",
        block_output.sequence_len().value(),
        block_output.model_dimension().value()
    );

    let multi_head_block = MultiHeadTransformerBlock::new(
        vec![
            SelfAttentionHead::new(
                HiddenToQuery::new(vec![vec![1.0], vec![0.0]], vec![0.0])?,
                HiddenToKey::new(vec![vec![1.0], vec![0.0]], vec![0.0])?,
                HiddenToValue::new(vec![vec![1.0], vec![0.0]], vec![0.0])?,
            )?,
            SelfAttentionHead::new(
                HiddenToQuery::new(vec![vec![0.0], vec![1.0]], vec![0.0])?,
                HiddenToKey::new(vec![vec![0.0], vec![1.0]], vec![0.0])?,
                HiddenToValue::new(vec![vec![0.0], vec![1.0]], vec![0.0])?,
            )?,
        ],
        AttentionOutputProjection::new(vec![vec![1.0, 0.0], vec![0.0, 1.0]], vec![0.0, 0.0])?,
        LayerNormalization::new(LayerNormParameters::identity(
            block_output.model_dimension(),
        )),
        PositionWiseFeedForward::new(
            vec![vec![1.0, 0.0], vec![0.0, 1.0]],
            vec![0.0, 0.0],
            vec![vec![1.0, 0.0], vec![0.0, 1.0]],
            vec![0.0, 0.0],
        )?,
        LayerNormalization::new(LayerNormParameters::identity(
            block_output.model_dimension(),
        )),
    )?;
    let multi_head_output = multi_head_block.apply(positioned_hidden)?;

    println!(
        "multi-head block shape: {} positions x {} heads x value dimension {} -> model dimension {}",
        multi_head_output.sequence_len().value(),
        multi_head_block.head_count().value(),
        multi_head_block.value_dimension().value(),
        multi_head_output.model_dimension().value()
    );

    let masked_multi_head_block = MaskedMultiHeadTransformerBlock::new(
        vec![
            SelfAttentionHead::new(
                HiddenToQuery::new(vec![vec![1.0], vec![0.0]], vec![0.0])?,
                HiddenToKey::new(vec![vec![1.0], vec![0.0]], vec![0.0])?,
                HiddenToValue::new(vec![vec![1.0], vec![0.0]], vec![0.0])?,
            )?,
            SelfAttentionHead::new(
                HiddenToQuery::new(vec![vec![0.0], vec![1.0]], vec![0.0])?,
                HiddenToKey::new(vec![vec![0.0], vec![1.0]], vec![0.0])?,
                HiddenToValue::new(vec![vec![0.0], vec![1.0]], vec![0.0])?,
            )?,
        ],
        AttentionOutputProjection::new(vec![vec![1.0, 0.0], vec![0.0, 1.0]], vec![0.0, 0.0])?,
        LayerNormalization::new(LayerNormParameters::identity(
            multi_head_output.model_dimension(),
        )),
        PositionWiseFeedForward::new(
            vec![vec![1.0, 0.0], vec![0.0, 1.0]],
            vec![0.0, 0.0],
            vec![vec![1.0, 0.0], vec![0.0, 1.0]],
            vec![0.0, 0.0],
        )?,
        LayerNormalization::new(LayerNormParameters::identity(
            multi_head_output.model_dimension(),
        )),
    )?;
    let masked_block_output = masked_multi_head_block.apply(Product::new(
        HiddenSequence::new(vec![vec![1.0, 0.0], vec![0.0, 1.0]])?,
        AttentionMask::new(vec![vec![true, false], vec![true, true]])?,
    ))?;

    println!(
        "masked multi-head block shape: {} positions x model dimension {}",
        masked_block_output.sequence_len().value(),
        masked_block_output.model_dimension().value()
    );

    let transformer_parameters = TinyTransformerParameters::new(
        PositionalEncoding::new(vec![vec![0.1, 0.0], vec![0.0, 0.1]])?,
        masked_multi_head_block,
        TransformerReadout::new(
            vec![vec![1.0, 0.0, 0.5], vec![0.0, 1.0, -0.5]],
            vec![0.0, 0.0, 0.0],
        )?,
    )?;
    let transformer_state =
        TransformerTrainingState::new(transformer_parameters, LearningRate::new(0.1)?);
    let training_set =
        TransformerReadoutTrainingSet::new([TransformerReadoutTrainingExample::new(
            HiddenSequence::new(vec![vec![1.0, 0.0], vec![0.0, 1.0]])?,
            AttentionMask::new(vec![vec![true, false], vec![true, true]])?,
            TokenSequence::from_indices([0, 1])?,
        )?])?;
    let sequence_logits = transformer_state.apply(Product::new(
        HiddenSequence::new(vec![vec![1.0, 0.0], vec![0.0, 1.0]])?,
        AttentionMask::new(vec![vec![true, false], vec![true, true]])?,
    ))?;
    let loss_before = transformer_readout_average_loss(&transformer_state, &training_set)?;
    let train_step = TransformerReadoutTrainStep::new(training_set.clone());
    let next_state = train_step.apply(transformer_state.clone())?;
    let loss_after = transformer_readout_average_loss(&next_state, &training_set)?;

    println!(
        "structured transformer logits shape: {} positions x vocabulary size {}",
        sequence_logits.sequence_len().value(),
        sequence_logits.vocab_size().value()
    );
    println!(
        "training state step: {} -> {}",
        transformer_state.step_count().value(),
        next_state.step_count().value()
    );
    println!(
        "readout loss after one update: {:.6} -> {:.6}",
        loss_before.value(),
        loss_after.value()
    );
    let feed_forward_training_set =
        TransformerFeedForwardTrainingSet::new([TransformerFeedForwardTrainingExample::new(
            HiddenSequence::new(vec![vec![1.0, 0.0], vec![0.0, 1.0]])?,
            HiddenSequence::new(vec![vec![2.0, 0.0], vec![0.0, 2.0]])?,
        )?])?;
    let feed_forward_loss_before =
        transformer_feed_forward_average_loss(&next_state, &feed_forward_training_set)?;
    let feed_forward_train_step =
        TransformerFeedForwardTrainStep::new(feed_forward_training_set.clone());
    let feed_forward_state = feed_forward_train_step.apply(next_state.clone())?;
    let feed_forward_loss_after =
        transformer_feed_forward_average_loss(&feed_forward_state, &feed_forward_training_set)?;

    println!(
        "feed-forward loss after one local update: {:.6} -> {:.6}",
        feed_forward_loss_before.value(),
        feed_forward_loss_after.value()
    );
    let block_training_set =
        TransformerBlockTrainingSet::new([TransformerBlockTrainingExample::new(
            HiddenSequence::new(vec![vec![1.0, 0.0], vec![0.0, 1.0]])?,
            AttentionMask::new(vec![vec![true, false], vec![true, true]])?,
            TokenSequence::from_indices([0, 1])?,
        )?])?;
    let block_loss_before =
        transformer_block_average_loss(&feed_forward_state, &block_training_set)?;
    let block_train_step = TransformerBlockTrainStep::new(block_training_set.clone());
    let block_trained_state = block_train_step.apply(feed_forward_state)?;
    let block_loss_after =
        transformer_block_average_loss(&block_trained_state, &block_training_set)?;

    println!(
        "block loss after one composed update: {:.6} -> {:.6}",
        block_loss_before.value(),
        block_loss_after.value()
    );

    println!();
    println!("Typed transformation:");
    println!("HiddenSequence -> QuerySequence");
    println!("HiddenSequence -> KeySequence");
    println!("HiddenSequence -> ValueSequence");
    println!("QuerySequence x KeySequence -> AttentionScores");
    println!("AttentionScores x AttentionMask -> AttentionScores");
    println!("AttentionScores -> AttentionWeights");
    println!("AttentionWeights x ValueSequence -> AttentionOutput");
    println!("AttentionHeadOutputs -> MultiHeadOutput");
    println!("MultiHeadOutput -> ProjectedAttentionOutput");
    println!("HiddenSequence x ProjectedAttentionOutput -> HiddenSequence");
    println!("LayerNormalization : HiddenSequence -> HiddenSequence");
    println!("PositionWiseFeedForward : HiddenSequence -> HiddenSequence");
    println!("PositionalEncoding : HiddenSequence -> HiddenSequence");
    println!("SingleHeadTransformerBlock : HiddenSequence -> HiddenSequence");
    println!("MultiHeadTransformerBlock : HiddenSequence -> HiddenSequence");
    println!("MaskedMultiHeadTransformerBlock : HiddenSequence x AttentionMask -> HiddenSequence");
    println!("TinyTransformerParameters : HiddenSequence x AttentionMask -> SequenceLogits");
    println!("TransformerTrainingState owns parameters, learning rate, and step count");
    println!("TransformerReadoutTrainStep : TransformerTrainingState -> TransformerTrainingState");
    println!(
        "TransformerFeedForwardTrainStep : TransformerTrainingState -> TransformerTrainingState"
    );
    println!("TransformerBlockTrainStep : TransformerTrainingState -> TransformerTrainingState");

    Ok(())
}
