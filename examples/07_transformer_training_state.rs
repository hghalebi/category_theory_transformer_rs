use category_theory_transformer_rs::{
    AttentionMask, AttentionOutputProjection, CtResult, HiddenSequence, HiddenToKey, HiddenToQuery,
    HiddenToValue, LayerNormParameters, LayerNormalization, LearningRate,
    MaskedMultiHeadTransformerBlock, ModelDimension, Morphism, PositionWiseFeedForward,
    PositionalEncoding, Product, SelfAttentionHead, TinyTransformerParameters, TokenSequence,
    TransformerBlockTrainStep, TransformerBlockTrainingExample, TransformerBlockTrainingSet,
    TransformerFeedForwardTrainStep, TransformerFeedForwardTrainingExample,
    TransformerFeedForwardTrainingSet, TransformerReadout, TransformerReadoutTrainStep,
    TransformerReadoutTrainingExample, TransformerReadoutTrainingSet, TransformerTrainingState,
    transformer_block_average_loss, transformer_feed_forward_average_loss,
    transformer_readout_average_loss,
};

fn main() -> CtResult<()> {
    let hidden = HiddenSequence::new(vec![vec![1.0, 0.0], vec![0.0, 1.0]])?;
    let mask = AttentionMask::new(vec![vec![true, false], vec![true, true]])?;
    let targets = TokenSequence::from_indices([0, 1])?;
    let initial_state = tiny_training_state()?;

    let logits = initial_state.apply(Product::new(hidden.clone(), mask.clone()))?;

    println!(
        "initial state: step={}, learning_rate={:.3}, model_dimension={}, vocab_size={}",
        initial_state.step_count().value(),
        initial_state.learning_rate().value(),
        initial_state.parameters().model_dimension().value(),
        initial_state.parameters().vocab_size().value()
    );
    println!(
        "forward shape: {} positions x vocabulary size {}",
        logits.sequence_len().value(),
        logits.vocab_size().value()
    );

    let readout_set =
        TransformerReadoutTrainingSet::new([TransformerReadoutTrainingExample::new(
            hidden.clone(),
            mask.clone(),
            targets.clone(),
        )?])?;
    let readout_loss_before = transformer_readout_average_loss(&initial_state, &readout_set)?;
    let readout_state =
        TransformerReadoutTrainStep::new(readout_set.clone()).apply(initial_state)?;
    let readout_loss_after = transformer_readout_average_loss(&readout_state, &readout_set)?;

    print_update(
        "readout update",
        0,
        &readout_state,
        readout_loss_before.value(),
        readout_loss_after.value(),
    );

    let feed_forward_set =
        TransformerFeedForwardTrainingSet::new([TransformerFeedForwardTrainingExample::new(
            hidden.clone(),
            HiddenSequence::new(vec![vec![2.0, 0.0], vec![0.0, 2.0]])?,
        )?])?;
    let feed_forward_loss_before =
        transformer_feed_forward_average_loss(&readout_state, &feed_forward_set)?;
    let feed_forward_state =
        TransformerFeedForwardTrainStep::new(feed_forward_set.clone()).apply(readout_state)?;
    let feed_forward_loss_after =
        transformer_feed_forward_average_loss(&feed_forward_state, &feed_forward_set)?;

    print_update(
        "feed-forward update",
        1,
        &feed_forward_state,
        feed_forward_loss_before.value(),
        feed_forward_loss_after.value(),
    );

    let block_set = TransformerBlockTrainingSet::new([TransformerBlockTrainingExample::new(
        hidden, mask, targets,
    )?])?;
    let block_loss_before = transformer_block_average_loss(&feed_forward_state, &block_set)?;
    let block_state =
        TransformerBlockTrainStep::new(block_set.clone()).apply(feed_forward_state)?;
    let block_loss_after = transformer_block_average_loss(&block_state, &block_set)?;

    print_update(
        "composed block update",
        2,
        &block_state,
        block_loss_before.value(),
        block_loss_after.value(),
    );

    println!();
    println!("Typed transformation:");
    println!("TinyTransformerParameters : HiddenSequence x AttentionMask -> SequenceLogits");
    println!("TransformerTrainingState owns parameters, learning rate, and step count");
    println!("TransformerReadoutTrainStep : TransformerTrainingState -> TransformerTrainingState");
    println!(
        "TransformerFeedForwardTrainStep : TransformerTrainingState -> TransformerTrainingState"
    );
    println!("TransformerBlockTrainStep : TransformerTrainingState -> TransformerTrainingState");
    println!("Every update returns a full training state, not loose changed weights.");

    Ok(())
}

fn print_update(
    label: &str,
    previous_step: usize,
    state: &TransformerTrainingState,
    loss_before: f32,
    loss_after: f32,
) {
    println!(
        "{label}: step {} -> {}, loss {:.6} -> {:.6}",
        previous_step,
        state.step_count().value(),
        loss_before,
        loss_after
    );
}

fn tiny_training_state() -> CtResult<TransformerTrainingState> {
    let model_dimension = ModelDimension::new(2)?;
    let block = MaskedMultiHeadTransformerBlock::new(
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
        LayerNormalization::new(LayerNormParameters::identity(model_dimension)),
        PositionWiseFeedForward::new(
            vec![vec![1.0, 0.0], vec![0.0, 1.0]],
            vec![0.0, 0.0],
            vec![vec![1.0, 0.0], vec![0.0, 1.0]],
            vec![0.0, 0.0],
        )?,
        LayerNormalization::new(LayerNormParameters::identity(model_dimension)),
    )?;
    let parameters = TinyTransformerParameters::new(
        PositionalEncoding::new(vec![vec![0.1, 0.0], vec![0.0, 0.1]])?,
        block,
        TransformerReadout::new(
            vec![vec![1.0, 0.0, 0.5], vec![0.0, 1.0, -0.5]],
            vec![0.0, 0.0, 0.0],
        )?,
    )?;

    Ok(TransformerTrainingState::new(
        parameters,
        LearningRate::new(0.1)?,
    ))
}
