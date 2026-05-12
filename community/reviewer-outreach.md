# Reviewer Outreach

Use this file when asking people to review the book. The goal is not praise,
promotion, or a broad opinion. The goal is one direct reader report that can
be triaged into a source-backed rewrite.

Send reviewers to:

- Public book: <https://hghalebi.github.io/category_theory_transformer_rs/>
- GitHub repository: <https://github.com/hghalebi/category_theory_transformer_rs>
- `community/reader-review-packet.md`
- [reader-confusion issue form](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=reader-confusion.yml)

Record accepted reports in `community/reader-feedback-inbox.md` and triage them
with `community/reader-feedback-triage.md`.
Run the first coordinated review round with `community/reader-review-sprint.md`.
Track the five public review slots with `community/reviewer-slot-tracker.md`.

## Reviewer Profiles

Ask different reviewers for different friction.

| Reviewer | Best review path | Main question |
| --- | --- | --- |
| Rust engineer | First-run or core chapter review | Are the types and trait boundaries idiomatic enough to trust? |
| ML engineer | Tiny ML pipeline or attention review | Does the ML intuition appear before abstraction? |
| Category-theory reader | Morphism, structure, Seven Sketches, or roadmap category-shape review | Are terms precise and modest? |
| Technical educator | Entry path and exercises | Does the learner know what to do next? |
| Beginner-adjacent reader | First-run review | Where does the path become too compressed? |

## Short Public Ask

```text
I am looking for direct reader feedback on Category Theory for Tiny ML in Rust.

The project is a public book and Rust lab for understanding tiny ML systems
through typed transformations, runnable examples, and category-theory structure.

The useful review is small:

1. run one command,
2. read one short path,
3. report the first place where the mental model breaks.

Start here:
https://hghalebi.github.io/category_theory_transformer_rs/

Review packet:
https://github.com/hghalebi/category_theory_transformer_rs/blob/main/community/reader-review-packet.md

Open the report here:
https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=reader-confusion.yml

If you are reviewing as a first-run or beginner-adjacent reader, use this
pre-filled report link:
https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=reader-confusion.yml&title=%5Breader+confusion%5D+beginner-adjacent+reader+brief&location=Welcome%2C+Course+Map%2C+Domain+Objects%2C+or+Morphism+and+Composition&command=cargo+run+--example+01_token_sequence+or+public+book+path
```

## Rust Reviewer Ask

```text
Could you review the Rust teaching path in Category Theory for Tiny ML in Rust?

The review should take about 20 to 30 minutes.

Please run:

cargo run --example 01_token_sequence
cargo run --bin category_ml

Then read one chapter:

book/src/01-domain-objects.md
book/src/02-morphisms-composition.md

What I need is one precise report:

- which type, trait, constructor, or example felt unclear,
- what made sense before that point,
- what one sentence, code comment, diagram, or exercise would help.

Review packet:
https://github.com/hghalebi/category_theory_transformer_rs/blob/main/community/reader-review-packet.md

Rust report link:
https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=reader-confusion.yml&title=%5Breader+confusion%5D+Rust+engineer+brief&location=book%2Fsrc%2F01-domain-objects.md+or+book%2Fsrc%2F02-morphisms-composition.md&command=cargo+run+--example+01_token_sequence%0Acargo+run+--bin+category_ml
```

## ML Reviewer Ask

```text
Could you review whether the ML intuition in Category Theory for Tiny ML in Rust
is clear before the abstractions appear?

Please focus on one path:

cargo run --example 01_token_sequence
cargo run --example 03_training_endomorphism
cargo run --example 06_attention_scores

Then read:

book/src/03-ml-pipeline.md
book/src/04-training-endomorphism.md
book/src/roadmap.md

The most useful feedback is the first point where a reader might lose the ML
meaning of logits, probabilities, loss, parameter updates, attention scores,
self-attention versus cross-attention, or training state.

Review packet:
https://github.com/hghalebi/category_theory_transformer_rs/blob/main/community/reader-review-packet.md

ML report link:
https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=reader-confusion.yml&title=%5Breader+confusion%5D+ML+engineer+brief&location=book%2Fsrc%2F03-ml-pipeline.md%2C+book%2Fsrc%2F04-training-endomorphism.md%2C+or+book%2Fsrc%2Froadmap.md&command=cargo+run+--example+03_training_endomorphism%0Acargo+run+--example+06_attention_scores
```

## Category-Theory Reviewer Ask

```text
Could you review the category-theory language in Category Theory for Tiny ML in
Rust?

The project uses category theory as names for executable structures, not as
decoration. I need feedback on whether the terms are precise, modest, and tied
to code.

Please inspect one path:

cargo run --example 02_morphism_composition
cargo run --example 04_structure_and_calculus
cargo run --example 05_seven_sketches
cargo run --example 06_attention_scores

Then read:

book/src/02-morphisms-composition.md
book/src/05-structure-and-calculus.md
book/src/seven-sketches-rust.md
book/src/roadmap.md

Useful feedback names the first term, law, or diagram that overclaims, arrives
too early, or needs a stronger Rust anchor.

If you choose the roadmap path, please test whether the category-shape
diagnostic clearly separates product-input morphisms from endomorphisms. One
unclear boundary is enough.

Also check whether the self-attention versus cross-attention section clearly
separates shared Q/K/V source, parallel projections from `HiddenSequence` into
query/key/value roles, the false `QuerySequence -> KeySequence ->
ValueSequence` reading, separate query and key-value sources, target length
`L`, source length `S`, and product-input morphism before any endomorphism
claim.

Review packet:
https://github.com/hghalebi/category_theory_transformer_rs/blob/main/community/reader-review-packet.md

Category-theory report link:
https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=reader-confusion.yml&title=%5Breader+confusion%5D+category-theory+reader+brief&location=book%2Fsrc%2F02-morphisms-composition.md%2C+book%2Fsrc%2F05-structure-and-calculus.md%2C+book%2Fsrc%2Fseven-sketches-rust.md%2C+or+book%2Fsrc%2Froadmap.md&command=cargo+run+--example+02_morphism_composition%0Acargo+run+--example+04_structure_and_calculus%0Acargo+run+--example+05_seven_sketches
```

## Educator Ask

```text
Could you review the learning path in Category Theory for Tiny ML in Rust?

I am not looking for a broad endorsement. I need one concrete learner-friction
report.

Please check:

README.md
START_HERE.md
community/reader-review-packet.md
book/src/welcome.md
book/src/00-map.md
book/src/exercises.md

If you do not want to clone first, use the public book path:

https://hghalebi.github.io/category_theory_transformer_rs/

The useful question is:

where does the project stop telling the reader what to run, read, explain, or
try next?

Review packet:
https://github.com/hghalebi/category_theory_transformer_rs/blob/main/community/reader-review-packet.md

Educator report link:
https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=reader-confusion.yml&title=%5Breader+confusion%5D+technical+educator+brief&location=README.md%2C+START_HERE.md%2C+community%2Freader-review-packet.md%2C+book%2Fsrc%2Fwelcome.md%2C+book%2Fsrc%2F00-map.md%2C+or+book%2Fsrc%2Fexercises.md&command=public+book+review+path+or+local+file+review
```

## Workshop Follow-Up Ask

```text
Thanks for joining the session.

The most useful follow-up is one precise reader report:

- what you ran or read,
- what made sense,
- where the mental model broke,
- what small edit would help.

Please use this packet:
https://github.com/hghalebi/category_theory_transformer_rs/blob/main/community/reader-review-packet.md

Public book:
https://hghalebi.github.io/category_theory_transformer_rs/

And report the first unclear point here:
https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=reader-confusion.yml

If this was your first pass through the project, this pre-filled report link
starts with the beginner-adjacent path:
https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=reader-confusion.yml&title=%5Breader+confusion%5D+beginner-adjacent+reader+brief&location=Welcome%2C+Course+Map%2C+Domain+Objects%2C+or+Morphism+and+Composition&command=cargo+run+--example+01_token_sequence+or+public+book+path
```

## Maintainer Close Loop

After sending an outreach ask:

1. Wait for a direct issue, workshop note, or review note.
2. Record accepted direct reports in `community/reader-feedback-inbox.md`.
3. Update `community/reviewer-slot-tracker.md` with the slot status only, not
   names or private contact details.
4. Triage the report with `community/reader-feedback-triage.md`.
5. Rewrite the smallest blocked learning step.
6. Record the rewrite in `book/CHAPTER_REWRITE_LOG.md`.
7. Run the relevant focused checks and then `bash scripts/check.sh` when public
   teaching material changed.

Do not record a sent outreach message as reader evidence. Only a real report
from someone reading the project counts.
