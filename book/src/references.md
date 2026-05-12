# References

The problem this chapter solves is:

> The course uses small Rust examples. These references point to the larger
> Rust, ML, category-theory, Transformer, and learning-science treatments behind
> those examples.

Use references as a writing tool, not as decoration. Before a chapter is
rewritten, choose the sources that answer the chapter's central question. After
the rewrite, check whether the chapter explains the same idea through the
book's three lenses:

```text
Rust syntax:
which source file in this course uses the idea?

ML concept:
which model, training, or learning behavior does the source explain?

Category theory concept:
which object, morphism, composition, product, endomorphism, functor, monoid, or law does it deepen?
```

## Chapter Reference Map

| Chapter | Central question | Best references to use while rewriting |
| --- | --- | --- |
| [Welcome](welcome.md) | What is the book's promise and reading contract? | [How People Learn II](https://www.nationalacademies.org/projects/DBASSE-BBCSS-13-06/publication/24783), [Rust By Example](https://doc.rust-lang.org/rust-by-example/index.html), [Seven Sketches](https://arxiv.org/abs/1803.05316) |
| [Course Map](00-map.md) | How do the Rust files, ML pipeline, and category-theory vocabulary fit together? | [Rust modules](https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html), [Seven Sketches](https://arxiv.org/abs/1803.05316), [Category Theory for Programming](https://arxiv.org/abs/2209.01259) |
| [Domain Objects](01-domain-objects.md) | Why should meaningful ML values become separate Rust types? | [Rust structs](https://doc.rust-lang.org/book/ch05-01-defining-structs.html), [Rust enums](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html), [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/checklist.html), [Result error handling](https://doc.rust-lang.org/stable/book/ch09-02-recoverable-errors-with-result.html) |
| [Morphism and Composition](02-morphisms-composition.md) | How do typed transformations compose safely? | [Rust traits](https://doc.rust-lang.org/book/ch10-02-traits.html), [Rust generics](https://doc.rust-lang.org/book/ch10-01-syntax.html), [Seven Sketches](https://arxiv.org/abs/1803.05316), [Category Theory for Programming](https://arxiv.org/abs/2209.01259) |
| [The Tiny ML Pipeline](03-ml-pipeline.md) | How do token pairs become prediction, probability, and loss? | [Dive into Deep Learning: Softmax Regression](https://d2l.ai/chapter_linear-classification/softmax-regression.html), [Softmax from Scratch](https://d2l.ai/chapter_linear-classification/softmax-regression-scratch.html), [CS231n Linear Classification](https://cs231n.github.io/linear-classify/), [Deep Learning](https://www.deeplearningbook.org/) |
| [Training as an Endomorphism](04-training-endomorphism.md) | Why is one training step a repeatable `Parameters -> Parameters` update? | [D2L Gradient Descent](https://d2l.ai/chapter_optimization/gd.html), [CS231n Optimization](https://cs231n.github.io/optimization-1/), [D2L Backpropagation and Computational Graphs](https://d2l.ai/chapter_multilayer-perceptrons/backprop.html), [The Matrix Calculus You Need For Deep Learning](https://arxiv.org/abs/1802.01528), [Backprop as Functor](https://arxiv.org/abs/1711.10455) |
| [Functors, Naturality, Monoids, and Chain Rule](05-structure-and-calculus.md) | Which recurring structures appear after the first ML pipeline works? | [Backprop as Functor](https://arxiv.org/abs/1711.10455), [Seven Sketches](https://arxiv.org/abs/1803.05316), [Category Theory for Programming](https://arxiv.org/abs/2209.01259), [D2L Backpropagation and Computational Graphs](https://d2l.ai/chapter_multilayer-perceptrons/backprop.html), [The Matrix Calculus You Need For Deep Learning](https://arxiv.org/abs/1802.01528) |
| [Seven Sketches Through Rust](seven-sketches-rust.md) | How can applied category theory become concrete enough to inspect in Rust? | [Seven Sketches](https://arxiv.org/abs/1803.05316), [Compositional Deep Learning](https://arxiv.org/abs/1907.08292), [Category Theory for Programming](https://arxiv.org/abs/2209.01259) |
| [Exercises](exercises.md) | How does the reader prove they can transfer the method? | [How People Learn II](https://www.nationalacademies.org/projects/DBASSE-BBCSS-13-06/publication/24783), [Improving Students' Learning With Effective Learning Techniques](https://journals.sagepub.com/doi/abs/10.1177/1529100612453266), [Test-Enhanced Learning](https://doi.org/10.1111/j.1467-9280.2006.01693.x), [Structuring the Transition From Example Study to Problem Solving](https://doi.org/10.1207/S15326985EP3801_3), [Writing Automated Tests](https://doc.rust-lang.org/stable/book/ch11-00-testing.html), [Rust By Example: Tests](https://doc.rust-lang.org/rust-by-example/cargo/test.html), [CS231n Optimization: numerical gradients](https://cs231n.github.io/optimization-1/), [CS231n: Neural Networks Part 3](https://cs231n.github.io/neural-networks-3/), [D2L Backpropagation and Computational Graphs](https://d2l.ai/chapter_multilayer-perceptrons/backprop.html), [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/checklist.html) |
| [Transformer Roadmap](roadmap.md) | How does the tiny system grow toward attention and Transformer blocks? | [Attention Is All You Need](https://arxiv.org/abs/1706.03762), [NeurIPS proceedings page](https://papers.nips.cc/paper/7181-attention-is-all-you-need), [D2L Attention and Transformers](https://d2l.ai/chapter_attention-mechanisms-and-transformers/index.html), [D2L Transformer Architecture](https://d2l.ai/chapter_attention-mechanisms-and-transformers/transformer.html), [D2L Queries, Keys, and Values](https://d2l.ai/chapter_attention-mechanisms-and-transformers/queries-keys-values.html), [D2L Attention Scoring Functions](https://d2l.ai/chapter_attention-mechanisms-and-transformers/attention-scoring-functions.html), [D2L Parameter Management](https://d2l.ai/chapter_builders-guide/parameters.html), [D2L Softmax From Scratch](https://d2l.ai/chapter_linear-classification/softmax-regression-scratch.html), [D2L Gradient Descent](https://d2l.ai/chapter_optimization/gd.html), [Hugging Face Course: How do Transformers work?](https://huggingface.co/docs/course/en/chapter1/4), [Hugging Face Transformers Model Outputs](https://huggingface.co/docs/transformers/main_classes/output), [PyTorch MultiheadAttention](https://docs.pytorch.org/docs/stable/generated/torch.nn.MultiheadAttention.html), [PyTorch TransformerEncoderLayer](https://docs.pytorch.org/docs/stable/generated/torch.nn.TransformerEncoderLayer.html), [Self-Attention as a Parametric Endofunctor](https://arxiv.org/abs/2501.02931), [Seven Sketches](https://arxiv.org/abs/1803.05316), [The Annotated Transformer](https://nlp.seas.harvard.edu/2018/04/03/attention.html), [The Illustrated Transformer](https://jalammar.github.io/illustrated-transformer/) |

## Rust

- [Category Theory for Tiny ML in Rust GitHub repository](https://github.com/hghalebi/category_theory_transformer_rs) is the public source for this book, including Rust modules, examples, exercises, and issue templates.
- [Category Theory for Tiny ML in Rust public workshop](https://luma.com/event/evt-Pb1kYMQvzs8JrQq) is the first public workshop for discussing the draft and the tiny ML pipeline.
- [The Rust Programming Language: Packages, Crates, and Modules](https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html) explains how Rust packages are organized into library and binary crates. Use it with `src/lib.rs`, `src/bin/category_ml.rs`, and the `examples/` files.
- [The Rust Programming Language: Defining and Instantiating Structs](https://doc.rust-lang.org/book/ch05-01-defining-structs.html) supports the domain-object chapter's use of named Rust structs.
- [The Rust Programming Language: Defining an Enum](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html) supports enum-based modeling in `src/sketches.rs` and future Transformer state modeling.
- [The Rust Programming Language: Generic Data Types](https://doc.rust-lang.org/book/ch10-01-syntax.html) supports the generic shapes in `Product<A, B>`, `Compose<F, G, Middle>`, and the functor examples.
- [The Rust Programming Language: Defining Shared Behavior with Traits](https://doc.rust-lang.org/book/ch10-02-traits.html) explains the trait contract behind `Morphism<Input, Output>`, `Functor<A, B>`, and `Monoid`.
- [The Rust Programming Language: Recoverable Errors with `Result`](https://doc.rust-lang.org/stable/book/ch09-02-recoverable-errors-with-result.html) explains the error pattern behind `CtResult<T>` and constructors such as `Distribution::new`.
- [The Rust Programming Language: Writing Automated Tests](https://doc.rust-lang.org/stable/book/ch11-00-testing.html) supports the exercise design where tests act as executable feedback.
- [Rust By Example](https://doc.rust-lang.org/rust-by-example/index.html) is useful when a chapter needs a smaller runnable Rust example before the real crate code.
- [Rust By Example: Tests](https://doc.rust-lang.org/rust-by-example/cargo/test.html) gives a compact view of unit and integration test organization for readers turning exercises into checks.
- [The rustdoc book: How to write documentation](https://doc.rust-lang.org/rustdoc/how-to-write-documentation.html) explains the documentation comments used above public types and methods.
- [Rust API Guidelines Checklist](https://rust-lang.github.io/api-guidelines/checklist.html) is a practical review checklist for naming, documentation, type conversions, and error design.

## Category Theory

- [Seven Sketches in Compositionality: An Invitation to Applied Category Theory](https://arxiv.org/abs/1803.05316) is the larger applied-category-theory text behind the companion chapter. Use it with `src/sketches.rs`.
- [Seven Sketches in Compositionality PDF](https://arxiv.org/pdf/1803.05316) is the direct paper file for offline reading and page-by-page study.
- [MIT OpenCourseWare: Applied Category Theory](https://ocw.mit.edu/courses/18-s097-applied-category-theory-january-iap-2019/) is a university course built around applied category theory and the Seven Sketches text. Use it when a chapter needs more examples before a formal definition.
- [Categories for the Working Mathematician](https://link.springer.com/book/10.1007/978-1-4757-4721-8) is the classic formal reference for category, functor, natural transformation, duality, adjunctions, limits, monoids, and related structures. Use it as precision support, not as prerequisite reading.
- [Category Theory for Programming](https://arxiv.org/abs/2209.01259) is a programming-oriented academic reference for connecting category-theory ideas to datatype and functional-programming structure.
- [Category Theory for Programmers PDF source repository](https://github.com/hmemcpy/milewski-ctfp-pdf) is a programmer-friendly bridge for readers who want a longer informal route from programming to category theory.

## Machine Learning

- [Dive into Deep Learning: Softmax Regression](https://d2l.ai/chapter_linear-classification/softmax-regression.html) explains multiclass classification, logits, softmax, and cross entropy. Use it with `src/ml.rs`.
- [Dive into Deep Learning: Softmax Regression Implementation from Scratch](https://d2l.ai/chapter_linear-classification/softmax-regression-scratch.html) shows the implementation path behind this course's smaller Rust version.
- [Dive into Deep Learning: Gradient Descent](https://d2l.ai/chapter_optimization/gd.html) gives the optimization background for `TrainStep`.
- [Dive into Deep Learning: Forward Propagation, Backward Propagation, and Computational Graphs](https://d2l.ai/chapter_multilayer-perceptrons/backprop.html) supports the chain-rule and training chapters.
- [Dive into Deep Learning: Numerical Stability and Initialization](https://d2l.ai/chapter_multilayer-perceptrons/numerical-stability-and-init.html) is useful when explaining why `Softmax` subtracts the maximum score before exponentiation.
- [Stanford CS231n: Optimization](https://cs231n.github.io/optimization-1/) explains finite differences, numerical gradients, analytic gradients, and gradient checks. Use it with the finite-difference exercise and the `TransformerBlockTrainStep` tests.
- [Stanford CS231n: Neural Networks Part 3](https://cs231n.github.io/neural-networks-3/) explains gradient-checking cautions, learning-rate checks, and small-data sanity checks. Use it when exercises ask readers to interpret a failed training or gradient-check signal.
- [Stanford CS231n: Linear Classification](https://cs231n.github.io/linear-classify/) explains linear classifiers, scores, losses, and the softmax classifier from a widely used university course.
- [Deep Learning](https://www.deeplearningbook.org/) by Goodfellow, Bengio, and Courville is a standard textbook reference for the broader ML vocabulary behind the tiny examples.
- [The Matrix Calculus You Need For Deep Learning](https://arxiv.org/abs/1802.01528) gives a compact bridge from scalar calculus to the matrix shapes behind neural-network training. Use it as an advanced support reference for the chain-rule and gradient-check sections, not as a prerequisite.

## Category Theory And Learning Systems

- [Backprop as Functor: A compositional perspective on supervised learning](https://arxiv.org/abs/1711.10455) connects supervised learning, parameter updates, gradient descent, and compositional structure. Use it carefully: the book's `TrainStep` is a tiny executable analogy, not a full implementation of the paper.
- [Compositional Deep Learning](https://arxiv.org/abs/1907.08292) is a research reference for neural-network composition and categorical schemas. Use it as advanced context, not as prerequisite reading.

## Transformers

- [Attention Is All You Need on arXiv](https://arxiv.org/abs/1706.03762) is the original Transformer paper.
- [Attention Is All You Need on the NeurIPS proceedings site](https://papers.nips.cc/paper/7181-attention-is-all-you-need) is the archival conference listing.
- [Dive into Deep Learning: Attention Mechanisms and Transformers](https://d2l.ai/chapter_attention-mechanisms-and-transformers/index.html) is a practical bridge from softmax and vector operations to attention and Transformer blocks. Use it with `src/attention.rs` for the query-key scoring, mask, score-to-weight, value-mixing, head-concatenation, output-projection, residual, normalization, and feed-forward boundaries.
- [Dive into Deep Learning: Queries, Keys, and Values](https://d2l.ai/chapter_attention-mechanisms-and-transformers/queries-keys-values.html) supports the role distinction between queries, keys, and values before the code names `QuerySequence`, `KeySequence`, and `ValueSequence`.
- [Dive into Deep Learning: Attention Scoring Functions](https://d2l.ai/chapter_attention-mechanisms-and-transformers/attention-scoring-functions.html) supports the scaled dot-product, masked-softmax, and value-mixing path used by `ScaledDotProductScores`, `MaskedAttentionScores`, `WeightedValueMixing`, and `MaskedMultiHeadTransformerBlock`.
- [Dive into Deep Learning: Multi-Head Attention](https://d2l.ai/chapter_attention-mechanisms-and-transformers/multihead-attention.html) supports the roadmap distinction between separate attention heads, concatenated head outputs, the output projection, and the `MultiHeadTransformerBlock` shape.
- [PyTorch `MultiheadAttention`](https://docs.pytorch.org/docs/stable/generated/torch.nn.MultiheadAttention.html) is a framework documentation reference for query, key, and value as separate forward inputs, separate source and target sequence shapes, and total embedding dimension split across attention heads. Use it as an API-shape sanity check for the book's typed role split and multi-head shape arithmetic.
- [PyTorch `scaled_dot_product_attention`](https://docs.pytorch.org/docs/stable/generated/torch.nn.functional.scaled_dot_product_attention.html) is a framework documentation reference for the implementation order: score, apply mask or bias, row-wise softmax, dropout if used, then value mixing. Use it as an implementation sanity check, not as the book's primary API target.
- [PyTorch `TransformerEncoderLayer`](https://docs.pytorch.org/docs/stable/generated/torch.nn.TransformerEncoderLayer.html) is an official framework reference for the original Transformer encoder layer shape. Use it to keep the roadmap's teaching boundary honest: the book can model foundational components while still being explicit that production libraries expose broader and faster variants.
- [Dive into Deep Learning: Self-Attention and Positional Encoding](https://d2l.ai/chapter_attention-mechanisms-and-transformers/self-attention-and-positional-encoding.html) supports the need for position information before sequence attention and the `PositionalEncoding` boundary.
- [Dive into Deep Learning: Transformer Architecture](https://d2l.ai/chapter_attention-mechanisms-and-transformers/transformer.html) supports the residual-connection, layer-normalization, position-wise feed-forward, block, decoder masking, readout, and training-loop shape requirements used by `ResidualConnection`, `LayerNormalization`, `PositionWiseFeedForward`, `SingleHeadTransformerBlock`, `MultiHeadTransformerBlock`, `MaskedMultiHeadTransformerBlock`, `TransformerReadout`, and `TransformerTrainingState`.
- [Dive into Deep Learning: Parameter Management](https://d2l.ai/chapter_builders-guide/parameters.html) supports the idea that model parameters should be managed as explicit named components rather than scattered unnamed arrays. Use it with `TinyTransformerParameters` and `TransformerTrainingState`.
- [Dive into Deep Learning: Softmax Regression Implementation from Scratch](https://d2l.ai/chapter_linear-classification/softmax-regression-scratch.html) supports the readout-only gradient step used by `TransformerReadoutTrainStep`.
- [Dive into Deep Learning: Backpropagation and Computational Graphs](https://d2l.ai/chapter_multilayer-perceptrons/backprop.html) supports the forward-cache and reverse-computation order used by `TransformerBlockTrainStep`.
- [Dive into Deep Learning: Gradient Descent](https://d2l.ai/chapter_optimization/gd.html) supports the learning-rate update shape used by `TransformerReadoutTrainStep`, `TransformerFeedForwardTrainStep`, and `TransformerBlockTrainStep`.
- [Hugging Face Course: How do Transformers work?](https://huggingface.co/docs/course/en/chapter1/4) is a practitioner-facing course reference for architecture families, attention layers, masks, and the distinction between architecture, checkpoint, and model. Use it when the roadmap needs to explain why this repository builds tiny architecture pieces rather than loading pretrained checkpoints.
- [Hugging Face Transformers: Model outputs](https://huggingface.co/docs/transformers/main_classes/output) is official framework documentation for returned hidden states, attentions, and output structures. Use it as an API-shape sanity check for the roadmap's `HiddenSequence`, `AttentionWeights`, and `SequenceLogits` boundaries.
- [Layer Normalization](https://arxiv.org/abs/1607.06450) by Ba, Kiros, and Hinton supports the layer-normalization boundary and the per-example mean-and-variance normalization used by the roadmap code.
- [Self-Attention as a Parametric Endofunctor](https://arxiv.org/abs/2501.02931) is an advanced research reference for categorical structure in the linear query, key, and value portions of self-attention. Use it as precision support when discussing linear attention structure, iterated layers, positional encodings, and the limit of the book's claims around softmax and layer normalization.
- [The Annotated Transformer](https://nlp.seas.harvard.edu/2018/04/03/attention.html) is useful when the roadmap needs an implementation-oriented bridge from paper notation to code.
- [The Illustrated Transformer](https://jalammar.github.io/illustrated-transformer/) is useful when the roadmap needs visual explanation of attention, encoder/decoder structure, and token-to-vector flow.

## Learning Design

- [How People Learn II: Learners, Contexts, and Cultures](https://www.nationalacademies.org/projects/DBASSE-BBCSS-13-06/publication/24783) supports the book's learning design: prior knowledge activation, worked examples, practice, retrieval, and attention to learner context.
- [Improving Students' Learning With Effective Learning Techniques](https://journals.sagepub.com/doi/abs/10.1177/1529100612453266) by Dunlosky, Rawson, Marsh, Nathan, and Willingham is useful when deciding whether a chapter asks readers to practice durable techniques instead of only rereading.
- [Test-Enhanced Learning: Taking Memory Tests Improves Long-Term Retention](https://doi.org/10.1111/j.1467-9280.2006.01693.x) by Roediger and Karpicke supports retrieval-practice prompts that ask readers to recall, explain, and apply without looking back first.
- [Structuring the Transition From Example Study to Problem Solving in Cognitive Skill Acquisition](https://doi.org/10.1207/S15326985EP3801_3) by Renkl and Atkinson supports the book's progression from worked examples to partially completed examples and then transfer exercises.
- [Self-Explanations: How Students Study and Use Examples in Learning to Solve Problems](https://doi.org/10.1207/s15516709cog1302_1) by Chi, Bassok, Lewis, Reimann, and Glaser supports self-check prompts that ask readers to explain why a worked example has the shape it has.

Use this section while rewriting chapters. The goal is not to cite learning
science on every page. The goal is to make each chapter easier to enter,
practice, remember, and transfer.
