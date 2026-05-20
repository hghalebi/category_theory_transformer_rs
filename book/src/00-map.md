# Course Map

The problem this chapter solves is:

> Before reading individual source files, you need one map that connects the
> tiny ML pipeline, the Rust modules, and the category-theory vocabulary.

The repository is intentionally small, but it still has layers. One layer names
the values. Another layer names transformations between values. A third layer
uses those transformations to make predictions, measure loss, and update model
parameters.

This chapter gives you the whole map before the book zooms in.

## Chapter Outcomes

By the end of this chapter, you should be able to:

- place each printed line from `cargo run --bin category_ml` into domain
  value, typed transformation, or training update,
- explain how `src/domain.rs`, `src/category.rs`, `src/ml.rs`, and
  `src/training.rs` divide responsibility,
- translate the book's first pipeline into objects, morphisms, product input,
  loss, and endomorphism language.

## Choose Your Path

Use the book-first path if you want the concepts introduced in order:

```text
Welcome
  -> Course Map
  -> Domain Objects
  -> Morphism and Composition
  -> Tiny ML Pipeline
  -> Training as an Endomorphism
```

Use the code-first path if you learn faster by running something first:

```bash
cargo run --example 01_token_sequence
cargo run --bin category_ml
```

Then come back to this map and place each printed line in one of three
locations:

```text
domain value
typed transformation
training update
```

Both paths are valid. The book-first path reduces surprise. The code-first path
reduces abstraction anxiety. The important thing is not to open every file at
once. Start with one path, run one command, and attach each new word to one
visible Rust shape.

## What You Already Know

If you read a program from top to bottom, you already know how to follow a
flow. If you read a Rust function signature, you already know that a step has
an input type and an output type. If you have seen any ML pipeline, you already
know that raw data eventually becomes predictions, loss, and updates.

The map in this chapter puts those familiar habits together:

```text
value
  -> transformation
  -> composed transformations
  -> measured error
  -> repeated update
```

The category-theory vocabulary is not a separate layer pasted on top. It names
shapes that are already present in the Rust and ML readings.

## Worked Example: From One Function To A Pipeline

Start with one ordinary function:

```rust
fn token_to_vector_id(token_id: usize) -> usize {
    token_id + 100
}

assert_eq!(token_to_vector_id(7), 107);
```

This has the shape:

```text
usize -> usize
```

That is a transformation, but it is not yet a good teaching boundary. Both
sides use the same raw type, so the signature does not tell us whether the
number is a token, a vector row, a dimension, or something else.

The book replaces that vague movement with named stages:

```text
TokenId -> Vector
```

Then it composes more stages:

```text
TokenId -> Vector -> Logits -> Distribution
```

That is the basic move for the whole book. Start with a familiar function,
give the meaningful values names, then ask which typed transformations can
compose safely.

## Self-check

Before continuing, explain why `TokenId -> Vector` carries more information
than `usize -> Vec<f32>`. A strong answer should mention both reader clarity
and compiler-checked boundaries.

## The Whole Pipeline

The first mental model is:

```text
Text -> Tokens -> TrainingPairs -> ModelState -> Prediction -> Loss -> Updated ModelState
```

Read it as one question:

> What object do we have now, and what typed transformation moves us to the next object?

The same diagram with the first concrete Rust names is:

```text
Text
  |
  | tokenize
  v
TokenSequence
  |
  | adjacent pairs
  v
TrainingSet
  |
  | train with current Parameters
  v
Parameters
  |
  | predict
  v
Distribution
  |
  | compare with target token
  v
Loss
  |
  | optimizer step
  v
Parameters
```

The public names and Rust names are close, but not identical:

| Reader-facing name | Rust name in this project | Why the distinction matters |
| --- | --- | --- |
| `Tokens` | `TokenSequence` | the code preserves order, not only a bag of token IDs |
| `TrainingPairs` | `TrainingSet` of `Product<TokenId, TokenId>` | each example has an input token and the next-token target |
| `ModelState` | `Parameters` | this tiny model's trainable state is its embedding and projection parameters |
| `Updated ModelState` | updated `Parameters` | training is a state update, not a new kind of object |

The central book pipeline is:

```text
Text
  -> TokenSequence
  -> TrainingSet
  -> Prediction
  -> Loss
  -> Updated Parameters
```

The concrete Rust shape is slightly more detailed:

```text
TokenSequence -> TrainingSet
TokenId       -> Vector
Vector        -> Logits
Logits        -> Distribution
Distribution x TokenId -> Loss
Parameters    -> Parameters
```

The same map can be drawn as a learner-facing flow:

```text
raw text
   |
   v
TokenSequence --DatasetWindowing--> TrainingSet
   |
   v
TokenId --Embedding--> Vector --LinearToLogits--> Logits --Softmax--> Distribution
                                                                        |
                                                                        v
                                                       Product<Distribution, TokenId>
                                                                        |
                                                                        v
                                                               CrossEntropy -> Loss

Parameters --TrainStep--> Updated Parameters
```

The same course map as a compact rendered math view:

\[
\begin{array}{ccccccccc}
\mathrm{Text}
& \to &
\mathrm{TokenSequence}
& \xrightarrow{\mathrm{DatasetWindowing}} &
\mathrm{TrainingSet}
& \leadsto &
\mathrm{Product}\langle\mathrm{Distribution},\mathrm{TokenId}\rangle
& \xrightarrow{\mathrm{CrossEntropy}} &
\mathrm{Loss} \\
&&&
&&&
\uparrow \mathrm{Softmax \circ LinearToLogits \circ Embedding}
&& \\
&&&
&&&
\mathrm{TokenId}
&& \\
\mathrm{Parameters}
& \xrightarrow{\mathrm{TrainStep}}
& \mathrm{UpdatedParameters}
&&&&&&
\end{array}
\]

Read the top path as prediction and evaluation. Read the bottom path as
training state. The two meet because `TrainStep` uses the training set, current
parameters, prediction path, and loss to produce updated parameters.

If the text diagram is easier to read first, use it first. If the rendered
view is easier to track, redraw it and label the Rust object behind every
mathematical name. Both views are teaching aids; the proof that the map is
real is still the code and the commands.

Read that map in three ways.

The Rust reading is about named types, trait implementations, constructors,
fallible boundaries, and tests. The ML reading is about data preparation,
embeddings, scores, probabilities, error measurement, and parameter updates.
The category-theory reading is about objects, morphisms, products,
composition, endomorphisms, and laws.

These are not three different books. They are three readings of the same small
program.

## Module Map

The source tree follows the learning path. Each file owns one part of the
conceptual load, so the reader does not have to learn every abstraction at the
same time.

| File | What it teaches | Main shape |
| --- | --- | --- |
| `src/domain.rs` | Meaningful values | `TokenId`, `Vector`, `Distribution`, `Parameters` |
| `src/category.rs` | Typed arrows | `Morphism<Input, Output>` |
| `src/ml.rs` | Concrete ML transformations | `TokenId -> Vector -> Logits -> Distribution` |
| `src/training.rs` | Repeated updates | `Parameters -> Parameters` |
| `src/structure.rs` | Reusable structure | functor, natural transformation, monoid |
| `src/calculus.rs` | Local derivative flow | chain rule for `z = x * y` |
| `src/sketches.rs` | Applied category-theory sketches | typed models plus law checks |
| `src/demo.rs` | Full guided walkthrough | executable course outline |

This table is not something to memorize. Use it as a navigation tool. When a
later chapter names a concept, you should be able to place it in one file and
one row of the pipeline.

## The Library Surface

The library root collects the modules and re-exports the teaching types. That
is why the examples can import clear names instead of reaching through deep
paths.

The important design is:

```text
domain nouns
category arrows
ML arrows
training update
structure patterns
calculus rule
applied sketches
demo
```

That order is also the reading path. The book first asks "what are the values?"
Then it asks "what transformations are allowed?" Only after those two questions
does it build prediction, loss, and training.

## How The Files Fit Together

`src/domain.rs` defines the nouns. A `TokenId` is not a `VocabSize`, a
`Distribution` is not a raw vector, and a `LearningRate` is not any other
floating-point number. This file protects meaning at the boundary where raw
machine values enter the tutorial.

`src/category.rs` defines the arrows. The central trait is:

```rust,ignore
pub trait Morphism<Input, Output> {
    fn name(&self) -> &'static str;
    fn apply(&self, input: Input) -> CtResult<Output>;
}
```

That trait says: a morphism is something that transforms an `Input` into an
`Output`, and the transformation may fail with a typed course error.

`src/ml.rs` makes the arrows concrete. It implements dataset windowing,
embedding lookup, linear projection, softmax, and cross entropy. This is where
the abstract phrase "typed transformation" becomes a tiny learning pipeline.

`src/training.rs` defines the update step:

```text
TrainStep : Parameters -> Parameters
```

Because the output type is the same as the input type, the update can be
repeated:

```text
Parameters0 -> Parameters1 -> Parameters2 -> ... -> ParametersN
```

That is why the training chapter teaches one optimizer step as an
endomorphism. It is a transformation from a type back to itself.

`src/structure.rs` gives names to reusable patterns that appear after the
pipeline works: mapping inside a container, converting one wrapper shape to
another, and combining traces with an identity value. These ideas are useful
because real systems accumulate logs, batches, optional results, gradients, and
workflow traces.

`src/calculus.rs` keeps backpropagation deliberately small. It shows the local
rule for:

```text
z = x * y
dL/dx = dL/dz * y
dL/dy = dL/dz * x
```

This is not a full automatic-differentiation engine. It is the smallest local
chain-rule shape the later training story can point at.

`src/sketches.rs` connects the tutorial to applied category theory beyond the
tiny ML pipeline. It models orders, resources, databases, co-design, signal
flow, circuits, and behavior logic as typed Rust values with law-checking
tests.

## Guided Walkthrough Snapshot

The terminal demo is the spine of the book. It gives a learner one command that
uses every major idea in a concrete order.

<details>
<summary>Source snapshot: src/demo.rs</summary>

```rust,ignore
{{#include ../../src/demo.rs}}
```

</details>

## How To Read The Demo

The demo output is a miniature course outline.

It starts with an object:

```text
TokenId(1)
```

Then it applies a data-preparation morphism:

```text
TokenSequence -> TrainingSet
```

Then it shows identity and composition:

```text
Vector -> Vector
TokenId -> Vector -> Logits -> Distribution
```

Then it uses a product object to measure loss:

```text
Distribution x TokenId -> Loss
```

Then it repeats an endomorphism:

```text
Parameters -> Parameters
```

The later demo sections add functors, naturality, monoids, a commutative
diagram check, and a local chain-rule example. By the time you finish the
demo, you have seen each major term at least once in executable form.

## Demo Output Wayfinding Checklist

After running `cargo run --bin category_ml`, use the numbered output as a map
instead of reading it as one long printout.

| Demo section | Source file to inspect next | Rust reading | ML reading | Category-theory reading |
| --- | --- | --- | --- | --- |
| `1. Object examples` | `src/domain.rs` | `TokenId` gives a raw index a domain name | tokens are data, not model state | object |
| `2. Dataset morphism` | `src/ml.rs` | `DatasetWindowing` turns a sequence into pairs | text becomes supervised examples | morphism |
| `3. Identity morphism` | `src/category.rs` | `Identity<Vector>` returns the same value | a neutral transformation should not change features | identity law |
| `4. Composition` | `src/ml.rs` and `src/category.rs` | `Compose` connects matching output and input types | embedding, logits, and softmax form prediction | composition |
| `5. Product object` | `src/domain.rs` and `src/ml.rs` | `Product<Distribution, TokenId>` pairs prediction with target | loss needs both prediction and correct next token | product object |
| `6. Endomorphism` | `src/training.rs` | `TrainStep` returns `Parameters` | training updates model state | endomorphism |
| `7-9. Structure patterns` | `src/structure.rs` | traits and tests name reusable operations | batches, options, and traces recur in ML systems | functor, naturality, monoid |
| `10. Commutative diagram check` | `src/ml.rs` | two code paths are compared | direct and composed prediction should agree | commutative diagram |
| `11. Chain rule` | `src/calculus.rs` | `MulOp::backward` returns local gradients | backprop starts from local derivative rules | chain rule |

This table gives you a safe next action. If the output line is clear, continue
reading. If it is not clear, open the source file in the second column and look
for the type or function named by the line. The goal is not to memorize the
demo. The goal is to use it as a routing table from terminal output to chapter,
source file, ML role, and category-theory shape.

## Source-Backed Wayfinding Rules

This chapter uses sources to keep the opening map practical. Each source
supports one local rule for how a first session should move from command output
to source files and then to vocabulary.

| Source | What the source supports | Local rule in this chapter | Repository evidence |
| --- | --- | --- | --- |
| [How People Learn II](https://www.nationalacademies.org/projects/DBASSE-BBCSS-13-06/publication/24783) | Learning should connect new ideas to prior knowledge and learner context. | Start from a familiar function, then attach Rust, ML, and category-theory names to one visible pipeline. | `## Worked Example: From One Function To A Pipeline`, `## The Three Readings` |
| [Rust Book: Packages, Crates, and Modules](https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html) | Rust packages organize code into crates and modules with separate responsibilities. | Treat the source tree as the learning map: nouns, arrows, ML arrows, training, structure, calculus, sketches, and demo. | `src/domain.rs`, `src/category.rs`, `src/ml.rs`, `src/training.rs`, `src/demo.rs` |
| [Rust By Example](https://doc.rust-lang.org/rust-by-example/index.html) | Small runnable examples make syntax inspectable before a larger explanation. | Run one command, inspect its output, then route the output line to the matching source file. | `cargo run --example 01_token_sequence`, `cargo run --bin category_ml` |
| [Seven Sketches](https://arxiv.org/abs/1803.05316) | Applied category theory is taught through compositional examples and recurring shapes. | Introduce object, morphism, product, composition, endomorphism, and law as names for shapes already visible in the tiny pipeline. | `TokenId -> Vector -> Logits -> Distribution`, `Distribution x TokenId -> Loss`, `Parameters -> Parameters` |
| [Category Theory for Programming](https://arxiv.org/abs/2209.01259) | Programming examples can make category-theory vocabulary less detached from code. | Translate from Rust file and function evidence to category vocabulary only after the typed path is visible. | `## Demo Output Wayfinding Checklist`, `Morphism<Input, Output>`, `Compose<F, G, Middle>` |

The transfer pattern is:

```text
source rule -> route through files -> command/output evidence
```

For this chapter, that means using `cargo run --bin category_ml` as more than a
demo. Treat it as a table of contents whose output routes you to
`src/domain.rs`, `src/category.rs`, `src/ml.rs`, `src/training.rs`,
`src/structure.rs`, `src/calculus.rs`, and `src/sketches.rs`.

The table is not evidence that the book has solved every learner's route
through the material. It is evidence that the first-session map is grounded in
named sources, real files, and executable output.

## Binary Entrypoint

The binary entrypoint is deliberately tiny:

<details>
<summary>Source snapshot: src/bin/category_ml.rs</summary>

```rust,ignore
{{#include ../../src/bin/category_ml.rs}}
```

</details>

The whole file delegates to the library walkthrough:

```rust,ignore
fn main() -> category_theory_transformer_rs::CtResult<()> {
    category_theory_transformer_rs::run_demo()
}
```

The binary returns `CtResult`, so fallible work can propagate through Rust's
ordinary `Result` path. The binary stays short because this book is teaching
the typed pipeline, not command-line interface design.

## First Run

Start with the smallest visible pipeline:

```bash
cargo run --example 01_token_sequence
```

That command turns text into token IDs and next-token training pairs before any
model weights appear.

Then run the full guided demo:

```bash
cargo run --bin category_ml
```

The exact floating-point values are less important than the shape. You should
see a loss before training, a lower loss after repeated training, and the same
typed pipeline used throughout the walkthrough.

## Core Mental Model

Every chapter after this one zooms into one row of the map.

An object is a typed thing the program can talk about precisely.

A morphism is a typed transformation from one object to another.

Composition is a legal connection between transformations, where the output
type of one step matches the input type of the next.

An endomorphism is a transformation from a type back to itself.

A law is a property the code checks so the reader can trust the shape, not only
the example output.

## Checkpoint

Explain this line in your own words:

```text
TokenId -> Vector -> Logits -> Distribution
```

A strong answer should mention token lookup, the embedding vector, vocabulary
scores, the probability distribution, and the fact that the whole path is a
composition of typed morphisms.

## Where This Leaves Us

This chapter gave the whole shape before the details. You now know the names of
the source files, the major pipeline objects, and the difference between
objects, morphisms, composition, endomorphisms, and laws.

The next chapter, [Domain Objects](01-domain-objects.md), slows down and
studies the objects themselves. Before a pipeline can compose arrows safely, it
needs values whose meanings are clear enough for arrows to start and end at
them.

## Further Reading

Do not leave this chapter with only a list of links. Use the next sources to
practice the map.

Start from this local evidence:

```text
cargo run --example 01_token_sequence
cargo run --bin category_ml
src/domain.rs
src/category.rs
src/ml.rs
src/training.rs
```

Then read the sources in this order:

| Source | What to transfer back into this chapter | Local evidence to inspect |
| --- | --- | --- |
| [How People Learn II](https://www.nationalacademies.org/projects/DBASSE-BBCSS-13-06/publication/24783) | New ideas should connect to prior knowledge, context, and visible learner activity. | `## What You Already Know`, `## Demo Output Wayfinding Checklist` |
| [Rust Book: Packages, Crates, and Modules](https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html) | A Rust package can expose a library crate, binary crates, and modules with separate responsibilities. | `src/bin/category_ml.rs`, `src/lib.rs`, `src/domain.rs`, `src/category.rs` |
| [Rust By Example](https://doc.rust-lang.org/rust-by-example/index.html) | Small runnable examples make syntax inspectable before a larger explanation. | `examples/01_token_sequence.rs`, `examples/02_morphism_composition.rs` |
| [Seven Sketches](https://arxiv.org/abs/1803.05316) | Applied category theory can be introduced through concrete examples before abstraction. | `TokenId -> Vector -> Logits -> Distribution` |
| [Category Theory for Programming](https://arxiv.org/abs/2209.01259) | Programming-shaped examples can keep category vocabulary attached to code. | `Morphism<Input, Output>`, `Compose<F, G, Middle>` |

After reading one external source, ask four questions:

1. Which command output line did it make easier to place?
2. Which source file should you inspect next?
3. Which category word did it clarify?
4. Which later chapter should you read after this map?

For this chapter, the commands are:

```bash
cargo run --example 01_token_sequence
cargo run --bin category_ml
```

For terminology recovery, use the [Glossary](glossary.md) entries for object,
morphism, composition, and endomorphism. For source depth, use
[References](references.md) and the [Seven Sketches Through Rust](seven-sketches-rust.md)
companion chapter after you can already route the first demo output.

If an external source does not help you connect one terminal line to one source
file and one category-theory word, it has not transferred back into the map
yet.

## Practice After This Chapter

Use [Exercise 2](exercises.md#exercise-2-add-a-token) to change the demo input
and [Exercise 8](exercises.md#exercise-8-trace-a-full-source-file) to connect
one source file back to the course map. Use the demo-output wayfinding
checklist above to decide which file to inspect. Those exercises check whether
the map is active knowledge rather than only a diagram you read once.

## Retrieval Practice

### Recall

Name the three readings used throughout the book.

### Explain

Why does the book start with a whole-pipeline map before reading individual
source files?

### Apply

Write a one-line diagram for a pipeline you already know, then label the input
object, arrow, and output object.
