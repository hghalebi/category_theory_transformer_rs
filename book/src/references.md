# References

The problem this chapter solves is:

> The course uses small Rust examples. These references point to the larger
> Rust, ML, and category-theory treatments behind those examples.

Use each reference with the same three questions:

```text
Rust syntax:
which source file in this course uses the idea?

ML concept:
which model or training behavior does the reference explain?

Category theory concept:
which object, morphism, composition, or law does it deepen?
```

## Rust

- [The Rust Programming Language: Packages, Crates, and Modules](https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html) explains how Rust packages are organized into library and binary crates. Use it with `src/lib.rs`, `src/bin/category_ml.rs`, and the `examples/` files.
- [The Rust Programming Language: Recoverable Errors with `Result`](https://doc.rust-lang.org/stable/book/ch09-02-recoverable-errors-with-result.html) explains the error pattern behind `CtResult<T>` and constructors such as `Distribution::new`.
- [The rustdoc book: How to write documentation](https://doc.rust-lang.org/rustdoc/how-to-write-documentation.html) explains the documentation comments used above public types and methods.
- [Rust API Guidelines Checklist](https://rust-lang.github.io/api-guidelines/checklist.html) is a practical review checklist for naming, documentation, type conversions, and error design.

## Category Theory

- [Seven Sketches in Compositionality: An Invitation to Applied Category Theory](https://arxiv.org/abs/1803.05316) is the larger applied-category-theory text behind the companion chapter. Use it with `src/sketches.rs`.
- [Seven Sketches in Compositionality PDF](https://arxiv.org/pdf/1803.05316) is the direct paper file for offline reading and page-by-page study.

## Machine Learning

- [Dive into Deep Learning: Softmax Regression](https://d2l.ai/chapter_linear-classification/softmax-regression.html) explains multiclass classification, logits, softmax, and cross entropy. Use it with `src/ml.rs`.
- [Dive into Deep Learning: Softmax Regression Implementation from Scratch](https://d2l.ai/chapter_linear-classification/softmax-regression-scratch.html) shows the implementation path behind this course's smaller Rust version.
- [Stanford CS231n: Linear Classification](https://cs231n.github.io/linear-classify/) explains linear classifiers, scores, losses, and the softmax classifier from a widely used university course.

## Transformers

- [Attention Is All You Need on arXiv](https://arxiv.org/abs/1706.03762) is the original Transformer paper.
- [Attention Is All You Need on the NeurIPS proceedings site](https://papers.nips.cc/paper/7181-attention-is-all-you-need) is the archival conference listing.
- [Dive into Deep Learning: Attention Mechanisms and Transformers](https://d2l.ai/chapter_attention-mechanisms-and-transformers/index.html) is a practical bridge from softmax and vector operations to attention and Transformer blocks.
