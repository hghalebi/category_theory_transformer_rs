# Glossary

The canonical glossary is in [book/src/glossary.md](../book/src/glossary.md).

This file exists so GitHub visitors browsing `docs/` can recover quickly when a
term becomes unclear.

Start with these terms:

- object
- morphism
- composition
- product object
- endomorphism
- functor
- natural transformation
- monoid
- calibrated confidence
- loss
- training step

## Source-Backed Recovery Rule

Do not stop at a formal definition. Recover the term through this path:

```text
term -> trusted source anchor -> Rust handle -> evidence signal
```

For this repository, an evidence signal is usually one command output, one
constructor error, one compiler error, one named test result, or one table row
you can point to in the book.

## Term Recovery Map

Use this table when a term blocks the next step. The goal is not to memorize the
word. The goal is to connect the word to one command, one chapter, and one
plain explanation.

| If this term is unclear | Run or read | Evidence signal | Explain before moving on |
| --- | --- | --- | --- |
| object | `cargo run --example 01_domain_objects`; [Domain Objects](../book/src/01-domain-objects.md) | a named type or constructor rejection | which named Rust type protects one ML meaning |
| morphism | `cargo run --example 02_morphism_composition`; [Morphism and Composition](../book/src/02-morphisms-composition.md) | an input type, output type, and returned value | what the source object and target object are |
| composition | `cargo run --example 02_morphism_composition` | the output line explaining legal and illegal composition | why `Vector == Vector` composes and `Vector != Logits` does not |
| product object | [Tiny ML Pipeline](../book/src/03-ml-pipeline.md) | a boundary with two inputs | why prediction plus target is different from prediction alone |
| calibrated confidence | [Tiny ML Pipeline](../book/src/03-ml-pipeline.md#scores-probabilities-and-loss) | `Distribution::new` validates a probability vector, but no calibration test is run | why normalized model probability is not the same as empirical reliability |
| product-input morphism returning the left object | [Transformer Roadmap](../book/src/roadmap.md#category-shape-diagnostic) | `A x B -> A` | why returning `A` does not make the boundary an endomorphism on `A` |
| product-as-source view | [Category-theory path](category-theory-path.md#context-fixing-mini-drill) | `(A x B) -> A` versus `(A x B) -> (A x B)` | why a unary morphism out of a product is still not an endomorphism unless it returns the same product |
| endomorphism | `cargo run --example 03_training_endomorphism`; [Training as an Endomorphism](../book/src/04-training-endomorphism.md) | a before/after state with the same object type | why the update has shape `Parameters -> Parameters` |
| functor | `cargo run --example 04_structure_and_calculus` | same wrapper shape before and after mapping | what changes inside the wrapper and what wrapper shape remains |
| natural transformation | `cargo run --example 04_structure_and_calculus` | `naturality square holds: true` | why two paths to `Option<B>` agree in the tiny square |
| monoid | `cargo run --example 04_structure_and_calculus` | `monoid laws hold: true` | what the empty trace is and how traces combine |
| loss | `cargo run --example 03_training_endomorphism` | loss before and loss after, plus the separate update arrow | why loss measures the current model instead of being the update |
| training step | `cargo run --example 07_transformer_training_state` | `step 0 -> 1` or another state transition | why each update returns a full training state |
| fixed module instance | `cargo run --example 06_attention_scores`; [Transformer Roadmap](../book/src/roadmap.md#category-shape-diagnostic) | `LayerNormalization : HiddenSequence -> HiddenSequence` beside `TransformerTrainingState -> TransformerTrainingState` | why a forward layer call is an endomorphism only for one fixed module value |
| parameter-changing update | `cargo run --example 07_transformer_training_state` | readout, feed-forward, or block update returns `TransformerTrainingState` | why changing scale, shift, weights, or biases belongs to training state |
| attention mask | `cargo run --example 06_attention_scores`; [Transformer Roadmap](../book/src/roadmap.md#self-attention-and-cross-attention-boundary) | `query 0 attends with [0.5, 0.0, 0.5]` and `AttentionScores x AttentionMask -> AttentionScores` | why mask cells select legal score cells before softmax, not token rows after probability mass has been assigned |
| mask polarity | [Transformer Roadmap](../book/src/roadmap.md#mask-polarity-ledger) | `true -> allowed`, `false -> blocked` in `AttentionMask` | why two masks can have the same shape and opposite boolean meaning across APIs |

If a term still feels abstract after the recovery step, open the canonical
glossary and look for the Rust handle first. Do not start from the most formal
definition.

## Fast Misreading Check

Before asking for more theory, check whether one of these smaller mistakes is
the real blocker:

| Misreading | Repair question |
| --- | --- |
| a wrapper is only cosmetic | what invalid value would a constructor reject? |
| a morphism is any function | what are the exact input and output objects? |
| composition means only "run in order" | what is the middle object? |
| loss performs training | which line measures loss and which line updates state? |
| returning the same visible object is always an endomorphism | how many inputs did the boundary require? |
| a layer endomorphism hides changing parameters | which fixed module value is being applied, or is this really a training-state update? |
| an attention mask removes tokens | which query-source score cells were made illegal before softmax? |
