# External Feedback Synthesis

This document records public learner-friction signals that can guide future
rewrites. It is not a substitute for direct feedback on this project. Treat it
as a proxy checklist until readers file project-specific issues through
`community/reader-review-guide.md`.

## Sources Reviewed

Rust learning and exercise design:

- [Rust Learn](https://rust-lang.org/learn/)
- [Brown University experimental Rust Book](https://rust-book.cs.brown.edu/experiment-intro.html)
- [Brown University experimental Rust Book overview](https://rust-book.cs.brown.edu/)
- [Rustlings usage guide](https://rustlings.rust-lang.org/usage/)
- [100 Exercises To Learn Rust](https://rust-exercises.com/100-exercises/01_intro/00_welcome)
- [r/learnrust: Is the official book good](https://www.reddit.com/r/learnrust/comments/1l47phk/is_the_official_book_good/)
- [r/rust: Learning Rust through the book](https://www.reddit.com/r/rust/comments/1jbfi5f/learning_rust_through_the_book/)
- [r/rust: When you learned Rust did you take notes or read the whole book](https://www.reddit.com/r/rust/comments/1fc4po5/when_you_learned_rust_did_you_take_notes_or/)

Category-theory entry points:

- [Category theory for programmers made easier](https://www.johndcook.com/blog/2020/11/02/category-theory-for-programmers/)
- [Stack Overflow: Do objects map to types or instances?](https://stackoverflow.com/questions/62838070/category-theory-to-computer-programming-do-objects-map-to-types-or-instances-o)
- [Stack Overflow: What exactly is a category?](https://stackoverflow.com/questions/46510557/what-exactly-is-a-category)

Gradient checking and ML debugging:

- [CS231n Optimization: numerical gradients](https://cs231n.github.io/optimization-1/)
- [CS231n Neural Networks Part 3: gradient checks and babysitting learning](https://cs231n.github.io/neural-networks-3/)
- [CS231n Backpropagation](https://cs231n.github.io/optimization-2/)
- [Dive into Deep Learning: Backpropagation and Computational Graphs](https://d2l.ai/chapter_multilayer-perceptrons/backprop.html)
- [Dive into Deep Learning: Gradient Descent](https://d2l.ai/chapter_optimization/gd.html)
- [The Matrix Calculus You Need For Deep Learning](https://arxiv.org/abs/1802.01528)
- [r/learnmachinelearning: Confusion in gradient descent](https://www.reddit.com/r/learnmachinelearning/comments/1q5tjr4/confusion_in_gradient_descent/)

Transformer understanding:

- [The Illustrated Transformer](https://jalammar.github.io/illustrated-transformer/)
- [r/MLQuestions: How does self-attention work?](https://www.reddit.com/r/MLQuestions/comments/16q8nkr)
- [r/MachineLearning: Most of us just pretend to understand Transformers](https://www.reddit.com/r/MachineLearning/comments/r76igz/discussion_rant_most_of_us_just_pretend_to/)

## Friction Themes

### 1. Reading Alone Is Not Enough

Public Rust learners repeatedly ask for practice, solution references, or
exercise paths. Official Rust material already presents multiple modes: the
main book for first-principles overview, Rust By Example for code-heavy
examples, and Rustlings for small exercises that fail until fixed. The Brown
experimental book also shows that quizzes and visualizations can expose
misconceptions that plain reading misses.

Implication for this project:

- Every core chapter should point to one command, one exercise, or one
  self-check.
- Worked examples should appear before open-ended exercises.
- The answer key should explain reasoning shape, not only final output.

Current coverage:

- `book/src/exercises.md`
- `exercises/README.md`
- `exercises/ANSWER_KEY.md`
- `exercises/advanced/README.md`

Next review question:

```text
Can a reader finish one chapter and immediately know what to run or explain?
```

### 2. Learners Need Failure Signals

Rustlings and 100 Exercises To Learn Rust both use executable feedback: a
compile error, failing test, or verification command tells the learner where
the boundary is. Public learner comments also emphasize a try, fail, research,
retry loop.

Implication for this project:

- Exercises should name the command that proves the answer.
- Exercises should say what failure means.
- Constructor errors and test failures should be framed as teaching signals.

Current coverage:

- `book/src/exercises.md` includes a failure-signal table.
- `scripts/check.sh` runs code, examples, prose checks, source coverage, and
  chapter tests.

Next review question:

```text
Does each exercise say what output, test result, or error shape proves progress?
```

### 3. Category Theory Should Start With Arrows

Programmer-facing category-theory discussions often become confusing when they
begin with formal foundations before the reader has a concrete arrow to hold.
The most useful public advice for programmers is to focus first on arrows,
functions, and patterns, then add rigor when the concrete shape is visible.
The Stack Overflow category question also shows a common confusion: mixing up
types, values, arrows, and the particular category being discussed.

Implication for this project:

- Introduce "morphism" as a typed transformation before formal vocabulary.
- Keep asking: object of which small category, arrow between which objects,
  and composition through which middle type?
- Avoid using category-theory words when a Rust type or function is not nearby.

Current coverage:

- `book/src/02-morphisms-composition.md`
- `book/src/glossary.md`
- `src/category.rs`

Next review question:

```text
Does each category-theory term appear next to a real Rust shape?
```

### 4. Gradient Checking Needs Two Independent Views

CS231n presents numerical gradients as slow and approximate but useful for
checking analytic gradients. D2L explains why forward computation and backward
computation depend on the same computational graph, with backpropagation
walking the graph in reverse order. Together, these sources support the
finite-difference exercise: compare a measured local loss slope with the
gradient inferred from a training update.

Implication for this project:

- The finite-difference section should name both paths:

  ```text
  one training update -> inferred gradient
  two perturbed losses -> numerical slope
  ```

- The exercise should stay small and inspect one selected parameter family at a
  time.
- The worked example should explain why averaged loss affects gradient scale.

Current coverage:

- `src/attention.rs`
- `book/src/exercises.md`
- `exercises/advanced/README.md`
- `exercises/ANSWER_KEY.md`

Next review question:

```text
Can a reader explain why the finite-difference check would catch a wrong sign,
missing bias path, or scale mismatch?
```

### 5. Transformer Explanations Need Shape And Role Separation

Public Transformer discussions often ask what self-attention is doing beyond
the formula. Visual explainers help because they separate token positions,
query/key/value roles, attention scores, weights, value mixing, and the later
block components. The recurring learner problem is not only "what is the
equation?" It is "which tensor role am I looking at, and what shape should it
have now?"

Implication for this project:

- Transformer roadmap prose should name roles before equations.
- Attention examples should keep scores, masks, weights, mixed values, and
  projected outputs as separate typed stages.
- Exercises should ask for shape and role traces before asking for full
  training behavior.

Current coverage:

- `src/attention.rs`
- `examples/06_attention_scores.rs`
- `book/src/roadmap.md`
- `book/src/exercises.md`
- `exercises/advanced/README.md`

Next review question:

```text
Can a reader point to the stage where token-to-token scores become
token-to-value mixing weights, and can they say which Rust type represents it?
```

## Chapter Friction Matrix

This matrix maps public learner-friction signals to chapter review questions.
It is a proxy review scaffold. It does not replace direct reader reports.

| Chapter | Public friction signal | Sources | Review check |
| --- | --- | --- | --- |
| Welcome | New readers need a fast win before theory. | [Rustlings](https://rustlings.rust-lang.org/usage/), [100 Exercises](https://rust-exercises.com/100-exercises/01_intro/00_welcome) | Can the reader run one command and explain the first pipeline shape before the abstract promise grows? |
| Course Map | Learners need one path through many files. | [Rust Learn](https://rust-lang.org/learn/), [Brown Rust Book](https://rust-book.cs.brown.edu/) | Does the map reduce choice overload by naming one first path and one code-first path? |
| Domain Objects | Rust learners confuse raw values with meaningful domain values. | [Brown Rust Book](https://rust-book.cs.brown.edu/), [Rust Learn](https://rust-lang.org/learn/) | Does each newtype explain the concrete mistake it prevents? |
| Morphism and Composition | Category-theory learners confuse types, values, objects, and arrows. | [John D. Cook](https://www.johndcook.com/blog/2020/11/02/category-theory-for-programmers/), [Stack Overflow](https://stackoverflow.com/questions/62838070/category-theory-to-computer-programming-do-objects-map-to-types-or-instances-o) | Does every category term arrive beside a Rust function, trait, or type boundary? |
| The Tiny ML Pipeline | ML beginners often need scores, probabilities, target index, and loss separated. | [CS231n Linear Classification](https://cs231n.github.io/linear-classify/), [D2L Softmax Regression](https://d2l.ai/chapter_linear-classification/softmax-regression.html) | Does the chapter show logits before probabilities and target-index loss before formulas dominate? |
| Training as an Endomorphism | Gradient descent confusion often comes from mixing one local update with full training behavior. | [D2L Gradient Descent](https://d2l.ai/chapter_optimization/gd.html), [CS231n Optimization](https://cs231n.github.io/optimization-1/), [r/learnmachinelearning](https://www.reddit.com/r/learnmachinelearning/comments/1q5tjr4/confusion_in_gradient_descent/) | Does the chapter separate one `Parameters -> Parameters` step from repeated optimization? |
| Functors, Naturality, Monoids, and Chain Rule | Chain-rule learners need two paths through the same computation. | [D2L Backpropagation](https://d2l.ai/chapter_multilayer-perceptrons/backprop.html), [Matrix Calculus](https://arxiv.org/abs/1802.01528) | Does the chapter force the reader to trace both composed paths before naming the law? |
| Seven Sketches Through Rust | Abstract category-theory reading needs transfer exercises. | [Seven Sketches](https://arxiv.org/abs/1803.05316), [John D. Cook](https://www.johndcook.com/blog/2020/11/02/category-theory-for-programmers/) | Does each sketch end with a concrete Rust transfer task rather than only vocabulary? |
| Exercises | Practice must include feedback signals, not only prompts. | [Rustlings](https://rustlings.rust-lang.org/usage/), [CS231n Optimization](https://cs231n.github.io/optimization-1/) | Does each major exercise say what command, output, or failure shape proves progress? |
| Transformer Roadmap | Transformer learners often lose track of Q/K/V roles, shapes, scores, masks, and value mixing. | [The Illustrated Transformer](https://jalammar.github.io/illustrated-transformer/), [r/MLQuestions](https://www.reddit.com/r/MLQuestions/comments/16q8nkr) | Can the reader trace which typed stage owns each Transformer role before full block training appears? |

## Rewrite Checklist

Use this checklist before the next chapter rewrite pass.

| Signal | Check | Evidence to inspect |
| --- | --- | --- |
| Practice gap | Chapter has a command, exercise, or self-check | `book/src/*.md`, `exercises/` |
| Feedback gap | Exercise names expected output or failure shape | `book/src/exercises.md`, `exercises/*/README.md` |
| Abstraction gap | Category term appears beside a Rust value or function | `book/src/*.md`, `src/*.rs` |
| Source gap | Technical claim has an authoritative reference | `book/src/references.md` |
| Transfer gap | Reader must explain Rust, ML, and category-theory meaning | `book/src/exercises.md`, `exercises/ANSWER_KEY.md` |

## Direct Feedback Still Needed

This synthesis only uses public external signals. The project still needs
direct reader reports from people reading this book and running this code.

Direct feedback should answer:

```text
Chapter:
Section:
Command or file tried:
Where the mental model broke:
Smallest edit that would help:
```
