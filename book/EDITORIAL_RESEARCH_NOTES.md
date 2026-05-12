# Editorial Research Notes

Status: Active rewrite support

These notes organize sources and reader-friction signals for chapter rewrites.
They are not a replacement for the learner-facing [References](src/references.md)
chapter. Use them while drafting, critiquing, and rewriting chapters.

## Source Priority

Use sources in this order when making technical claims:

1. Repository code and tests.
2. Official Rust documentation.
3. Peer-reviewed or academic papers.
4. Established open textbooks and university course material.
5. High-quality implementation tutorials.
6. Community posts as friction signals only.

Community posts can reveal where readers get stuck. They should not be used as
authority for definitions, laws, or ML correctness.

## Learning-Science Evidence

Use these sources when deciding how a chapter should teach, practice, and
review an idea. They are not decoration; each one maps to a concrete editorial
decision.

| Source | Use in this book | Editorial check |
| --- | --- | --- |
| [How People Learn II](https://www.nationalacademies.org/projects/DBASSE-BBCSS-13-06/publication/24783) | prior knowledge, transfer, learner context | Does the chapter connect the new concept to something the reader already knows? |
| [Improving Students' Learning With Effective Learning Techniques](https://journals.sagepub.com/doi/abs/10.1177/1529100612453266) | durable study methods | Does the chapter ask for retrieval, practice, or explanation rather than rereading alone? |
| [Test-Enhanced Learning](https://doi.org/10.1111/j.1467-9280.2006.01693.x) | retrieval practice | Can the reader answer Recall, Explain, and Apply prompts without looking back first? |
| [Structuring the Transition From Example Study to Problem Solving](https://doi.org/10.1207/S15326985EP3801_3) | faded examples | Does practice move from complete worked example to partially completed example to transfer? |
| [Self-Explanations](https://doi.org/10.1207/s15516709cog1302_1) | self-check prompts | Does the chapter ask why the example works, not only what it prints? |

Treat these as rewrite constraints for every major chapter. A chapter can pass
without citing all of them directly, but it should not skip the learning moves
they support.

## Chapter Source Buckets

### Welcome And Course Map

Use these sources to keep the opening concrete, progressive, and grounded in
working code:

- [How People Learn II](https://www.nationalacademies.org/projects/DBASSE-BBCSS-13-06/publication/24783)
- [Test-Enhanced Learning](https://doi.org/10.1111/j.1467-9280.2006.01693.x)
- [Rust By Example](https://doc.rust-lang.org/rust-by-example/index.html)
- [Seven Sketches in Compositionality](https://arxiv.org/abs/1803.05316)
- [Rust modules](https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html)

Rewrite check:

- Does the reader know what to run first?
- Does the reader know what the project is not?
- Does every abstract term arrive after a concrete code shape?
- Does the opening ask the reader to retrieve the first pipeline shape before
  continuing?

### Domain Objects

Use these sources to explain why names and boundaries matter:

- [Rust structs](https://doc.rust-lang.org/book/ch05-01-defining-structs.html)
- [Rust enums](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html)
- [Rust `Result`](https://doc.rust-lang.org/stable/book/ch09-02-recoverable-errors-with-result.html)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/checklist.html)
- [Self-Explanations](https://doi.org/10.1207/s15516709cog1302_1)

Rewrite check:

- Does the chapter explain what confusion each newtype prevents?
- Are validation rules attached to constructors?
- Does the reader see why raw `usize` and `Vec<f32>` are too ambiguous at
  public boundaries?
- Does the self-check ask the reader to explain the invariant in their own
  words?

### Morphisms And Composition

Use these sources to make typed transformations precise without overloading the
reader:

- [Rust generics](https://doc.rust-lang.org/book/ch10-01-syntax.html)
- [Rust traits](https://doc.rust-lang.org/book/ch10-02-traits.html)
- [Seven Sketches in Compositionality](https://arxiv.org/abs/1803.05316)
- [Category Theory for Programming](https://arxiv.org/abs/2209.01259)
- [Category Theory for Programmers source repository](https://github.com/hmemcpy/milewski-ctfp-pdf)

Rewrite check:

- Does "morphism" first appear as "typed transformation"?
- Does composition fail for understandable type reasons?
- Are identity and associativity explained as laws the code can test, not
  slogans?

### Tiny ML Pipeline

Use these sources to ground logits, softmax, loss, and classification:

- [Dive into Deep Learning: Softmax Regression](https://d2l.ai/chapter_linear-classification/softmax-regression.html)
- [Dive into Deep Learning: Softmax Regression from Scratch](https://d2l.ai/chapter_linear-classification/softmax-regression-scratch.html)
- [CS231n: Linear Classification](https://cs231n.github.io/linear-classify/)
- [Deep Learning](https://www.deeplearningbook.org/)
- [Structuring the Transition From Example Study to Problem Solving](https://doi.org/10.1207/S15326985EP3801_3)

Rewrite check:

- Does the reader see scores before probabilities?
- Is softmax explained as a normalization step over logits?
- Is cross entropy connected to "the model assigned probability to the target"
  before equations appear?
- Does the practice move from a full trace to a partially completed trace?
- Does a loss exercise force the reader to use the target token index rather
  than simply picking the largest probability?

### Training And Chain Rule

Use these sources to explain updates, computational graphs, and compositional
learning:

- [Dive into Deep Learning: Gradient Descent](https://d2l.ai/chapter_optimization/gd.html)
- [Stanford CS231n: Optimization](https://cs231n.github.io/optimization-1/)
- [Dive into Deep Learning: Backpropagation and Computational Graphs](https://d2l.ai/chapter_multilayer-perceptrons/backprop.html)
- [Dive into Deep Learning: Numerical Stability and Initialization](https://d2l.ai/chapter_multilayer-perceptrons/numerical-stability-and-init.html)
- [The Matrix Calculus You Need For Deep Learning](https://arxiv.org/abs/1802.01528)
- [Backprop as Functor](https://arxiv.org/abs/1711.10455)
- [Test-Enhanced Learning](https://doi.org/10.1111/j.1467-9280.2006.01693.x)

Rewrite check:

- Does the chapter explain one training step before repeated training?
- Does the chapter separate measuring loss from updating parameters?
- Does `Parameters -> Parameters` feel inevitable, not ornamental?
- Does the chapter clearly separate the tiny code analogy from full
  automatic differentiation?
- Does retrieval practice ask for the update rule before giving it again?

### Structure And Laws

Use these sources to ground functors, naturality, monoids, and executable law
checks:

- [Seven Sketches in Compositionality](https://arxiv.org/abs/1803.05316)
- [Category Theory for Programming](https://arxiv.org/abs/2209.01259)
- [Rust Book: Advanced Traits](https://doc.rust-lang.org/stable/book/ch20-02-advanced-traits.html)
- [Rust By Example: Associated Types](https://doc.rust-lang.org/rust-by-example/generics/assoc_items/types.html)

Rewrite check:

- Does every law word point to a concrete test?
- Does the naturality square show both paths before naming commutativity?
- Does the monoid explanation name identity and associativity separately?
- Does the exercise ask the reader to trace values, not only repeat law names?

### Seven Sketches Through Rust

Use these sources to keep the applied-category chapter faithful to the source
paper while still teaching through small Rust boundaries:

- [Seven Sketches in Compositionality](https://arxiv.org/abs/1803.05316)
- [Seven Sketches PDF](https://arxiv.org/pdf/1803.05316)
- [Category Theory for Programming](https://arxiv.org/abs/2209.01259)
- [Compositional Deep Learning](https://arxiv.org/abs/1907.08292)
- [Rust Book: Defining an Enum](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html)
- [Rust Book: Traits](https://doc.rust-lang.org/book/ch10-02-traits.html)

Rewrite check:

- Does each sketch state the source idea before translating it into Rust?
- Does the Rust code make the boundary, relation, order, or composition visible
  without pretending to cover the whole paper?
- Does every sketch end with a transfer task that asks the reader to model one
  analogous system?
- Does the chapter mark speculative extensions as sketches rather than as
  completed ML theory?

### Exercises And Transfer

Use these sources to make practice cumulative, testable, and transfer-oriented
rather than a loose list of prompts:

- [How People Learn II](https://www.nationalacademies.org/publications/24783)
- [Improving Students' Learning With Effective Learning Techniques](https://journals.sagepub.com/doi/abs/10.1177/1529100612453266)
- [Test-Enhanced Learning](https://doi.org/10.1111/j.1467-9280.2006.01693.x)
- [Structuring the Transition From Example Study to Problem Solving](https://doi.org/10.1207/S15326985EP3801_3)
- [Rust Book: Writing Automated Tests](https://doc.rust-lang.org/stable/book/ch11-00-testing.html)
- [Rust By Example: Tests](https://doc.rust-lang.org/rust-by-example/cargo/test.html)
- [CS231n: Optimization](https://cs231n.github.io/optimization-1/)

Rewrite check:

- Does each exercise state the evidence that proves progress?
- Do exercises move from recall to explanation to modification to transfer?
- Does command-based practice use commands that the repository actually
  validates?
- Does at least one exercise make a wrong answer fail through the compiler,
  constructor, or test output instead of only through prose?

### Transformer Roadmap

Use these sources when extending the tiny pipeline toward attention:

- [Attention Is All You Need](https://arxiv.org/abs/1706.03762)
- [NeurIPS proceedings page](https://papers.nips.cc/paper/7181-attention-is-all-you-need)
- [Seven Sketches in Compositionality](https://arxiv.org/abs/1803.05316)
- [MIT OpenCourseWare: Applied Category Theory](https://ocw.mit.edu/courses/18-s097-applied-category-theory-january-iap-2019/)
- [Categories for the Working Mathematician](https://link.springer.com/book/10.1007/978-1-4757-4721-8)
- [Dive into Deep Learning: Attention Mechanisms and Transformers](https://d2l.ai/chapter_attention-mechanisms-and-transformers/index.html)
- [Dive into Deep Learning: Queries, Keys, and Values](https://d2l.ai/chapter_attention-mechanisms-and-transformers/queries-keys-values.html)
- [Dive into Deep Learning: Attention Scoring Functions](https://d2l.ai/chapter_attention-mechanisms-and-transformers/attention-scoring-functions.html)
- [Dive into Deep Learning: Parameter Management](https://d2l.ai/chapter_builders-guide/parameters.html)
- [Dive into Deep Learning: Softmax Regression Implementation from Scratch](https://d2l.ai/chapter_linear-classification/softmax-regression-scratch.html)
- [Dive into Deep Learning: Gradient Descent](https://d2l.ai/chapter_optimization/gd.html)
- [Hugging Face Course: How do Transformers work?](https://huggingface.co/docs/course/en/chapter1/4)
- [Hugging Face Transformers: Model outputs](https://huggingface.co/docs/transformers/main_classes/output)
- [PyTorch MultiheadAttention](https://docs.pytorch.org/docs/stable/generated/torch.nn.MultiheadAttention.html)
- [PyTorch TransformerEncoderLayer](https://docs.pytorch.org/docs/stable/generated/torch.nn.TransformerEncoderLayer.html)
- [PyTorch scaled_dot_product_attention](https://docs.pytorch.org/docs/stable/generated/torch.nn.functional.scaled_dot_product_attention.html)
- [Self-Attention as a Parametric Endofunctor](https://arxiv.org/abs/2501.02931)
- [The Annotated Transformer](https://nlp.seas.harvard.edu/2018/04/03/attention.html)
- [The Illustrated Transformer](https://jalammar.github.io/illustrated-transformer/)
- [Improving Students' Learning With Effective Learning Techniques](https://journals.sagepub.com/doi/abs/10.1177/1529100612453266)

Rewrite check:

- Does the roadmap reuse the book's typed pipeline instead of jumping to a full
  production Transformer?
- Does the roadmap use framework docs as API-shape checks without treating a
  framework layer as the book's teaching target?
- Does attention appear as a structured map over token representations?
- Does the roadmap distinguish tiny architecture pieces from pretrained
  checkpoints and framework output containers?
- Does each future component have a plausible tiny Rust version?
- Does the structured parameter object group named roles without pretending to
  implement full attention backpropagation?
- Does any category-theory claim about attention separate the linear query,
  key, value, and positional parts from nonlinear softmax and layer
  normalization?
- Does the roadmap distinguish the self-attention case from the more general
  query-source and key-value-source boundary before naming an endomorphism?
- Does the readout-only, local feed-forward, and composed block training prose
  clearly separate tiny pedagogical updates from production-scale training?
- Does the roadmap ask the reader to trace related boundaries visually instead
  of only reading a long component list?
- Does a shape-flow exercise force the reader to distinguish scores, masks,
  weights, value mixing, projection, residual shape, normalization, and
  feed-forward refinement?
- Does the roadmap classify each boundary as a unary morphism, product-input
  morphism, shape-preserving endomorphism, state endomorphism, or illegal
  composition before using category vocabulary?

## Learner Friction Signals

These sources are qualitative signals for what motivated readers struggle with.
Use them to improve examples, exercises, and transitions.

- The [Brown experimental Rust Book](https://rust-book.cs.brown.edu/) shows how
  quizzes and misconception-targeted edits can make Rust learning more active
  than reading alone.
- [Coursera supervised learning course page](https://www.coursera.org/learn/machine-learning) shows the durable demand for intuitive, stepwise supervised-learning explanations before advanced machinery.
- [Stanford Engineering Everywhere CS229 material](https://see.stanford.edu/Course/CS229) shows the traditional lecture progression from linear models and gradient descent toward broader ML theory.
- [Reader discussion after Category Theory for Programmers](https://www.reddit.com/r/haskell/comments/1evw5qo) suggests that category-theory learners often need transfer exercises after reading abstract material.
- [Learner confusion around LMS/gradient-descent implementation](https://www.reddit.com/r/learnmachinelearning/comments/m95oxu) suggests that update formulas need concrete code traces and careful normalization explanations.
- The [CS231n gradient-checking notes](https://cs231n.github.io/neural-networks-3/)
  show that a useful gradient check also teaches scale, relative error, and
  small-data sanity checks.
- Public Transformer questions and visual explainers suggest this extra theme:
  Transformer explanations need shape and role separation. Keep query/key/value
  roles, scores, masks, weights, mixed values, residuals, and normalization as
  separate named stages before compressing them into a block.

Editorial response:

- Add transfer exercises after every abstract category-theory concept.
- Show one concrete numeric or typed trace before formalizing an update.
- Distinguish "this tiny code demonstrates the shape" from "this is a full
  production implementation."
- Keep visual pipeline diagrams close to the code they explain.
- Use the Chapter Friction Matrix in
  `community/external-feedback-synthesis.md` before each rewrite pass so public
  learner signals are mapped to chapter-level checks.

## Iteration Log Template

Use this short template when doing a chapter pass:

```text
Chapter:
Central question:
Primary code file/example:
Primary references:
Learner friction:
ML critique:
Visual/tutorial critique:
Category-theory critique:
Learner critique:
Rewrite decision:
Validation:
```
