# Transformer Roadmap

The repository name points toward Transformers, but the current code is a
foundation course. It teaches the typed pieces that a Transformer implementation
will need before adding attention.

## What Exists Now

The current model has:

- token ids
- embeddings
- a linear projection to logits
- softmax
- cross entropy
- full-batch training as `Parameters -> Parameters`
- composition checks
- simple structure examples

That is enough to learn the shape:

```text
typed data -> typed transformations -> composed prediction -> repeated update
```

## What Comes Next

A Transformer-oriented extension should add these ideas in order:

1. **Sequences as first-class objects**

   Current examples predict from one token at a time. Attention needs a typed
   sequence representation with positions and masks.

2. **Query, key, and value projections**

   Each projection is another typed morphism from hidden vectors to attention
   vectors.

3. **Scaled dot-product attention**

   Attention turns query-key similarity scores into a softmax distribution, then
   uses that distribution to mix value vectors.

4. **Multi-head attention**

   Multiple attention heads run in parallel, then their outputs are combined.

5. **Residual blocks and normalization**

   Transformer blocks repeatedly map a hidden sequence back to a hidden
   sequence. That gives another endomorphism shape.

6. **Training and evaluation**

   The existing loss and training patterns can be extended once the parameter
   object has attention weights and feed-forward weights.

## Category-Theory Reading

The interesting categorical shape is still composition:

```text
TokenSequence -> HiddenSequence -> AttentionOutput -> HiddenSequence -> Logits
```

The repeated block shape is another endomorphism:

```text
HiddenSequence -> HiddenSequence
```

The practical rule stays the same: make every intermediate object explicit, then
compose only the arrows whose types actually match.
