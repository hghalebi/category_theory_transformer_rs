# Beginner-Adjacent Path

Use this path if one part of the project feels new:

- you know Rust but not ML
- you know ML but not Rust
- you are curious about category theory but do not know the vocabulary yet
- you can run examples but lose the mental model after the output scrolls by

The goal is not to understand everything on the first pass. The goal is to
finish one small loop:

```text
run one command -> name the output objects -> read one short section -> explain one boundary
```

## First Ten Minutes

Run:

```bash
cargo run --example 01_token_sequence
```

Look for four printed blocks:

| Printed block | Plain meaning | Word to keep |
| --- | --- | --- |
| `Raw input` | the original sentence | text |
| `TokenSequence` | token IDs in order | object |
| `TrainingPairs` | input token followed by next-token target | product-shaped examples |
| `Typed transformation` | the path the code took | morphism chain |

Do not worry about the hash-like token IDs yet. The important point is:

```text
raw text became a named object, then that object became training examples
```

## First Output Diagnostic

If the first command runs but the output does not click, do not read the whole
project again. Find the first printed block that feels confusing.

| First confusing block | What it usually means | Next move |
| --- | --- | --- |
| `Raw input` | this is still plain text before the model sees structure | say the sentence out loud, then look at the next block |
| `TokenSequence` | each `TokenId(...)` is a named token index, not a meaningful word by itself | read the `TokenId` paragraph in Domain Objects |
| `TrainingPairs` | each line is one input token and the next token to predict | trace one pair by hand from the token sequence |
| `Typed transformation` | this is the path from raw text to training examples | copy `Text -> TokenSequence -> TrainingPairs` and label each object |
| `No framework magic` | the example is intentionally small so the structure stays visible | resist looking for a hidden model call |

You are ready to continue when you can point to one output line and say:

```text
this is the object, and this arrow made the next object
```

## First Thirty Minutes

Read and run in this order:

1. Read [Welcome](../book/src/welcome.md).
2. Read [Course Map](../book/src/00-map.md).
3. Run `cargo run --example 01_domain_objects`.
4. Read [Domain Objects](../book/src/01-domain-objects.md).
5. Run `cargo run --example 02_morphism_composition`.
6. Read [Morphism and Composition](../book/src/02-morphisms-composition.md).

Stop after each command and answer one question:

```text
What object did I just see, and what boundary did the code protect?
```

## Stop Signs

Use this table when the path becomes too compressed.

| If this is unclear | Do this next | What you should recover |
| --- | --- | --- |
| `TokenId` | read the first half of Domain Objects | a token index has a meaning, not only a number |
| `TokenSequence` | run `cargo run --example 01_domain_objects` | a sequence is validated before it becomes training data |
| `TrainingPairs` | compare the printed adjacent pairs | each pair is input token -> target token |
| `morphism` | run `cargo run --example 02_morphism_composition` | a morphism is a typed transformation |
| `composition` | inspect the `Vector == Vector` and `Vector != Logits` lines | arrows compose only when the middle type matches |
| `endomorphism` | wait until Training as an Endomorphism | it means the update returns the same kind of object it consumes |

## What Not To Do First

Do not start by reading every source file.
Do not start with the Transformer Roadmap.
Do not memorize the category-theory words before running the examples.

The early path is:

```text
see output -> name object -> name transformation -> explain why one connection is legal
```

## First Checkpoint

After the first session, explain this without looking back:

```text
Text -> TokenSequence -> TrainingPairs
```

A strong answer says:

- `Text` is the raw sentence
- `TokenSequence` is a validated ordered list of token IDs
- `TrainingPairs` are adjacent input-target token pairs
- the arrows are typed transformations

## Ask For Help

Open the [chapter clarity feedback form](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=chapter-clarity.yml)
when the first-session path becomes unclear.

For this path, use
[Open beginner-adjacent learner report](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=chapter-clarity.yml&title=%5Bgood+first+feedback%5D+beginner-adjacent+reader+brief&location=Welcome%2C+Course+Map%2C+Domain+Objects%2C+or+Morphism+and+Composition&command=cargo+run+--example+01_token_sequence+or+public+book+path).
The link fills the route, not the evidence; the evidence signal should come
from what you personally read, ran, or attempted.

Use this shape:

```text
Perspective: beginner-adjacent learner
Command or page tried:
Evidence signal:
First unclear sentence, command output, or term:
Last idea that was clear:
What would have helped:
```

Use the evidence signal for the exact output line, page section, term, or
command result that made the first-session path hard to follow.

The most useful report names the first exact sentence, command output line, or
term where the mental model broke.
