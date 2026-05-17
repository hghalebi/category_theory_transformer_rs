# Reviewers Needed

This project needs direct reader evidence from people who actually read a page,
run a command, or attempt an exercise.

The goal is not praise, ranking, or a broad opinion survey. The goal is one
precise report that shows where the learning path first stopped working.

## Minimum Accepted Report

A report can be accepted when it contains all six fields below:

```text
Perspective:
Command or page tried:
Evidence signal:
Last clear idea:
First unclear point:
Smallest useful fix:
```

The evidence signal must be visible in the book, terminal output, compiler
output, test output, table row, code block, or exercise prompt. The fastest
useful report is one exact sentence, output line, or boundary row plus the
smallest change that would have helped.

Current target:

```text
1 Rust engineer report
1 ML engineer or learner report
1 category-theory reader report
1 technical educator report
1 beginner-adjacent learner report
```

## Open Review Slots

Each slot needs one direct reader report from someone who personally read a
page, ran a command, or attempted an exercise. Routing issues help reviewers
find the path, but the slot is not filled until a report includes evidence.

| Needed report | Route | Report link |
| --- | --- | --- |
| Rust engineer | [issue #8](https://github.com/hghalebi/category_theory_transformer_rs/issues/8) | [Open Rust engineer report](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=chapter-clarity.yml&title=%5Bgood+first+feedback%5D+Rust+engineer+brief&location=docs%2Frust-path.md&command=cargo+run+--example+01_domain_objects%0Acargo+run+--example+02_morphism_composition%0Acargo+test+domain%3A%3Atests+--lib%0Acargo+test+category%3A%3Atests+--lib) |
| ML engineer or learner | [issue #9](https://github.com/hghalebi/category_theory_transformer_rs/issues/9) | [Open ML engineer report](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=chapter-clarity.yml&title=%5Bgood+first+feedback%5D+ML+engineer+brief&location=docs%2Fml-path.md&command=cargo+run+--example+01_token_sequence%0Acargo+run+--bin+category_ml%0Acargo+run+--example+03_training_endomorphism%0Acargo+run+--example+07_transformer_training_state) |
| Category-theory reader | [issue #10](https://github.com/hghalebi/category_theory_transformer_rs/issues/10) | [Open category-theory reader report](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=chapter-clarity.yml&title=%5Bgood+first+feedback%5D+category-theory+reader+brief&location=book%2Fsrc%2Froadmap.md+-%3E+Category+Shape+Diagnostic+-%3E+Reader+Evidence+Handoff&command=cargo+run+--example+06_attention_scores) |
| Technical educator | [issue #11](https://github.com/hghalebi/category_theory_transformer_rs/issues/11) | [Open technical educator report](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=chapter-clarity.yml&title=%5Bgood+first+feedback%5D+technical+educator+brief&location=README.md%2C+START_HERE.md%2C+docs%2Feducator-path.md%2C+book%2Fsrc%2Fwelcome.md%2C+book%2Fsrc%2F00-map.md%2C+or+book%2Fsrc%2Fexercises.md&command=public+book+review+path+or+local+file+review) |
| Beginner-adjacent learner | [issue #12](https://github.com/hghalebi/category_theory_transformer_rs/issues/12) | [Open beginner-adjacent learner report](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=chapter-clarity.yml&title=%5Bgood+first+feedback%5D+beginner-adjacent+reader+brief&location=Welcome%2C+Course+Map%2C+Domain+Objects%2C+or+Morphism+and+Composition&command=cargo+run+--example+01_token_sequence+or+public+book+path) |

Open a quick report here:

<https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=quick-reader-report.yml>

Open a fuller chapter clarity report here:

<https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=chapter-clarity.yml>

Use the one-page [reader review worksheet](docs/review-worksheet.md) while
reading or running examples. It gives five route cards and a copy/paste report
shape.

If you want to see the shape of a useful report before opening an issue, read
[docs/review-examples.md](docs/review-examples.md).

## Share This Reviewer Call

Use this short text when inviting a reviewer:

```text
Category Theory for Tiny ML in Rust needs one precise reader report.

Pick the closest path: Rust, ML, category theory, technical educator, or
beginner-adjacent learner.

Spend 20 minutes, stop at the first unclear point, and open one report with:
the page or command tried, one visible evidence signal, the last clear idea,
the first unclear point, and the smallest fix that would have helped.

Public book:
https://hghalebi.github.io/category_theory_transformer_rs/

Reviewer guide:
https://github.com/hghalebi/category_theory_transformer_rs/blob/main/REVIEWERS.md

Quick report form:
https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=quick-reader-report.yml

Full chapter-clarity form:
https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=chapter-clarity.yml
```

## No-Clone Review

If you cannot clone the repository right now, you can still give useful direct
reader evidence from the public book.

Use this route:

1. Open the public book:
   <https://hghalebi.github.io/category_theory_transformer_rs/>
2. Read one short path: Welcome, Course Map, Domain Objects, or Morphism and
   Composition.
3. Stop at the first sentence, heading, diagram, table row, code block, or
   exercise prompt that becomes unclear.
4. Open the closest report link below and include the public page you read as
   the command or page tried.

A no-clone report counts when it names a public page and one visible evidence
signal from that page. It does not count if it only says the project looks
interesting, confusing, or useful.

Fastest no-clone report:
[Open beginner-adjacent learner report](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=chapter-clarity.yml&title=%5Bgood+first+feedback%5D+beginner-adjacent+reader+brief&location=Welcome%2C+Course+Map%2C+Domain+Objects%2C+or+Morphism+and+Composition&command=public+book+path).

## Who We Need

The most useful review round has one report from each perspective:

| Perspective | First action | Useful evidence signal |
| --- | --- | --- |
| Rust engineer | Run `cargo run --example 01_domain_objects` and `cargo run --example 02_morphism_composition` | first unclear type, constructor, trait boundary, compiler signal, test name, or output line |
| ML engineer or learner | Run `cargo run --example 03_training_endomorphism` and `cargo run --example 07_transformer_training_state` | first unclear logits, probability, loss, training update, attention role, or state transition |
| Category-theory reader | Run `cargo run --example 02_morphism_composition` and `cargo run --example 06_attention_scores` | first overloaded term, missing law, unclear morphism shape, fixed-mask context issue, or roadmap precision issue |
| Technical educator | Review `README.md`, `START_HERE.md`, `docs/educator-path.md`, and `book/src/exercises.md` | first missing next action, weak practice loop, unclear exercise expectation, or feedback cue |
| Beginner-adjacent learner | Run `cargo run --example 01_token_sequence` or read `START_HERE.md` | first sentence, command, output line, or term that becomes too compressed |

If none of these fits, choose the closest perspective in the issue form.

## Open The Right Report

These links open the same public chapter-clarity form with the title, location,
and command path already filled. Please still write your own evidence signal
from what you personally read, ran, or attempted.

If you only have one concrete signal, use the
[quick reader report form](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=quick-reader-report.yml).
If you want to include expected-versus-actual context, use the detailed report
links below.

| Perspective | Report link |
| --- | --- |
| Rust engineer | [Open Rust engineer report](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=chapter-clarity.yml&title=%5Bgood+first+feedback%5D+Rust+engineer+brief&location=docs%2Frust-path.md&command=cargo+run+--example+01_domain_objects%0Acargo+run+--example+02_morphism_composition%0Acargo+test+domain%3A%3Atests+--lib%0Acargo+test+category%3A%3Atests+--lib) |
| ML engineer or learner | [Open ML engineer report](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=chapter-clarity.yml&title=%5Bgood+first+feedback%5D+ML+engineer+brief&location=docs%2Fml-path.md&command=cargo+run+--example+01_token_sequence%0Acargo+run+--bin+category_ml%0Acargo+run+--example+03_training_endomorphism%0Acargo+run+--example+07_transformer_training_state) |
| Category-theory reader | [Open category-theory reader report](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=chapter-clarity.yml&title=%5Bgood+first+feedback%5D+category-theory+reader+brief&location=book%2Fsrc%2Froadmap.md+-%3E+Category+Shape+Diagnostic+-%3E+Reader+Evidence+Handoff&command=cargo+run+--example+06_attention_scores) |
| Technical educator | [Open technical educator report](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=chapter-clarity.yml&title=%5Bgood+first+feedback%5D+technical+educator+brief&location=README.md%2C+START_HERE.md%2C+docs%2Feducator-path.md%2C+book%2Fsrc%2Fwelcome.md%2C+book%2Fsrc%2F00-map.md%2C+or+book%2Fsrc%2Fexercises.md&command=public+book+review+path+or+local+file+review) |
| Beginner-adjacent learner | [Open beginner-adjacent learner report](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=chapter-clarity.yml&title=%5Bgood+first+feedback%5D+beginner-adjacent+reader+brief&location=Welcome%2C+Course+Map%2C+Domain+Objects%2C+or+Morphism+and+Composition&command=cargo+run+--example+01_token_sequence+or+public+book+path) |

## Current High-Value Target

The most useful category-theory report right now starts at the roadmap
`Reader Evidence Handoff`:

```text
book/src/roadmap.md -> Category Shape Diagnostic -> Reader Evidence Handoff
book/src/roadmap.md -> Category Shape Diagnostic -> Source-Target Audit Card
cargo run --example 06_attention_scores
AttentionScores x AttentionMask -> AttentionScores
```

Use that handoff and audit card to report one boundary row, output line, or
naming rule where the whole source object, target object, or context status
became unclear.

Start with the first four lines of `cargo run --example 06_attention_scores`.
The most useful Q/K/V report says whether this line makes score ownership clear
before any attention weights appear:

```text
query rows own score rows; key/value rows own score columns
```

Then report whether the next line keeps shared self-attention source separate
from projected Q, K, and V roles:

```text
self-attention shares the hidden source before projection; projected roles stay distinct
```

One narrow target inside that handoff is the fixed-mask attention explanation:

```text
open boundary:
MaskedMultiHeadTransformerBlock : HiddenSequence x AttentionMask -> HiddenSequence

fixed context:
choose one mask M

induced view:
MaskedMultiHeadTransformerBlock[M] : HiddenSequence -> HiddenSequence
```

Run `cargo run --example 06_attention_scores`, then read the roadmap handoff
and the Context-Fixing Mini-Drill in
[docs/category-theory-path.md](docs/category-theory-path.md). If the
closure-capture analogy helps or fails, open one report with the exact
sentence, code block, table row, or handoff field where the explanation became
clear or unclear. The highest-value report says whether the Source-Target
Audit Card helped you avoid calling a product-input boundary an endomorphism.

## What Counts

A report counts as direct reader evidence only when it comes from your own
reading, command run, or exercise attempt.

Include:

- your closest perspective;
- the command, example, chapter, exercise, or public page you tried;
- one evidence signal such as an output line, compiler error, constructor
  result, test name, table row, sentence, or exercise prompt;
- the last idea that was clear;
- the first point where the path stopped working;
- the smallest change that would help the next reader.

## Source And Citation Reports

If your issue is about a reference, citation, source-backed claim, or missing
source, use the optional source-role field in the quick or detailed report
form.

Name:

- source role;
- owned boundary;
- claim this source supports;
- claim this source does not support;
- local Rust file, type, example, or chapter section.

The useful report is not "add more citations." The useful report names one
source-backed sentence, table row, chapter reference, code example, or missing
source boundary and says which claim became unclear.

Use the source-role table in `book/src/references.md` and the source-role
example in [docs/review-examples.md](docs/review-examples.md).

## After You File

A maintainer will triage the report against the evidence above.

If the report is missing the exact page, command, output line, sentence, table
row, or exercise prompt, the next step is clarification.

If the report has enough evidence, it becomes accepted reader evidence for that
perspective. Accepted evidence then drives one of two public outcomes:

- a focused rewrite of the affected chapter, example, table, diagram, or
  exercise;
- a recorded theme that waits for a second matching report before a larger
  rewrite.

When a focused rewrite lands, the change should name the affected public file
and the validation command that was run. That makes the feedback loop visible to
future readers instead of turning review into a private note.

## What Does Not Count

These are useful as general encouragement, but they do not count as direct
reader evidence:

- stars, likes, or broad praise;
- broad dislike without a blocked learning step;
- copied prompts or AI-generated critique;
- private conversation summaries;
- feature requests without the first unclear point;
- route-only issues that do not include a reading, command, or exercise signal.

## Twenty-Minute Review

Use this short loop:

1. Pick one perspective from the table.
2. Spend five minutes on the first command or page.
3. Continue until the first real confusion point appears.
4. Open one report with the exact signal and the smallest fix that would have
   helped.

Stop at the first useful confusion point. One concrete blocked step is more
valuable than a full-book opinion.

## Report Shape

```text
Perspective:
Friction lens:
Chapter or file:
Command or page tried:
Evidence signal:
First unclear sentence, output line, table row, code block, or exercise:
Last clear idea:
What you expected:
What happened instead:
What would have helped:
```

Do not include email addresses, private messages, personal notes, or contact
details in public reports.
