# Public Review Sprint

Use this page when several readers want to help improve the book in one short
cycle.

For the shareable public call, use [../REVIEWERS.md](../REVIEWERS.md).
For the one-page fill-in worksheet, use
[review-worksheet.md](review-worksheet.md).

The goal is not a general opinion survey. The goal is five concrete clarity
reports from five different perspectives:

```text
1 Rust engineer
1 ML engineer or learner
1 category-theory reader
1 technical educator
1 beginner-adjacent learner
```

Each report should identify one exact place where the current learning path
stopped working.

## Minimum Accepted Report

Before the sprint starts, show every reviewer this minimum shape:

```text
Perspective:
Command or page tried:
Evidence signal:
Last clear idea:
First unclear point:
Smallest useful fix:
```

The evidence signal must be something the reviewer personally saw: a terminal
output line, compiler error, test name, Rust type boundary, table row,
sentence, diagram, code block, or exercise prompt. A route-only issue, broad
opinion, copied prompt, or private summary does not fill a sprint slot.

## Open Sprint Slots

Each slot needs one evidence-bearing report. A routing issue is only a path
handle; the slot is not filled until the reviewer submits a report with a page
or command, one evidence signal, the last clear idea, and the first unclear
point.

| Needed report | Route | Report link |
| --- | --- | --- |
| Rust engineer | [issue #8](https://github.com/hghalebi/category_theory_transformer_rs/issues/8) | [Open Rust engineer report](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=chapter-clarity.yml&title=%5Bgood+first+feedback%5D+Rust+engineer+brief&location=docs%2Frust-path.md&command=cargo+run+--example+01_domain_objects%0Acargo+run+--example+02_morphism_composition%0Acargo+test+domain%3A%3Atests+--lib%0Acargo+test+category%3A%3Atests+--lib) |
| ML engineer or learner | [issue #9](https://github.com/hghalebi/category_theory_transformer_rs/issues/9) | [Open ML engineer report](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=chapter-clarity.yml&title=%5Bgood+first+feedback%5D+ML+engineer+brief&location=docs%2Fml-path.md&command=cargo+run+--example+01_token_sequence%0Acargo+run+--bin+category_ml%0Acargo+run+--example+03_training_endomorphism%0Acargo+run+--example+07_transformer_training_state) |
| Category-theory reader | [issue #10](https://github.com/hghalebi/category_theory_transformer_rs/issues/10) | [Open category-theory reader report](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=chapter-clarity.yml&title=%5Bgood+first+feedback%5D+category-theory+reader+brief&location=book%2Fsrc%2Froadmap.md+-%3E+Category+Shape+Diagnostic+-%3E+Reader+Evidence+Handoff&command=cargo+run+--example+06_attention_scores) |
| Technical educator | [issue #11](https://github.com/hghalebi/category_theory_transformer_rs/issues/11) | [Open technical educator report](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=chapter-clarity.yml&title=%5Bgood+first+feedback%5D+technical+educator+brief&location=README.md%2C+START_HERE.md%2C+docs%2Feducator-path.md%2C+book%2Fsrc%2Fwelcome.md%2C+book%2Fsrc%2F00-map.md%2C+or+book%2Fsrc%2Fexercises.md&command=public+book+review+path+or+local+file+review) |
| Beginner-adjacent learner | [issue #12](https://github.com/hghalebi/category_theory_transformer_rs/issues/12) | [Open beginner-adjacent learner report](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=chapter-clarity.yml&title=%5Bgood+first+feedback%5D+beginner-adjacent+reader+brief&location=Welcome%2C+Course+Map%2C+Domain+Objects%2C+or+Morphism+and+Composition&command=cargo+run+--example+01_token_sequence+or+public+book+path) |

## Sprint Rule

Each reviewer chooses one perspective, runs or reads the assigned path, and
opens one chapter clarity report.

Use the quick public form when a reviewer has one concrete signal:

<https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=quick-reader-report.yml>

Use the fuller public form when a reviewer can add expected-versus-actual
context:

<https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=chapter-clarity.yml>

Use [direct reader report examples](review-examples.md) to calibrate report
shape before the sprint starts.
Use [the reader review worksheet](review-worksheet.md) during the sprint so
each reviewer can keep the route, evidence signal, and report fields on one
page.

## Source And Citation Reports During A Sprint

If a sprint report is about a reference, citation, source-backed claim, or
missing source, use the optional source-role field in the quick or detailed
report form.

Name:

- source role;
- owned boundary;
- claim this source supports;
- claim this source does not support;
- local Rust file, type, example, or chapter section.

Good source reports do not ask for "more citations" in general. They identify
one source boundary that would make one chapter claim easier to verify.

Use the source-role table in `book/src/references.md` and the source-role
example in [direct reader report examples](review-examples.md) before filing
the report.

## Invite Text

Use this short text when inviting reviewers:

```text
We are testing five reader paths through Category Theory for Tiny ML in Rust.

Pick one perspective: Rust engineer, ML engineer or learner, category-theory
reader, technical educator, or beginner-adjacent learner.

Spend 20 minutes. Run or read one assigned path. Stop at the first unclear
point. Open one report with the page or command tried, one visible evidence
signal, the last clear idea, the first unclear point, and the smallest fix that
would have helped.

Public book:
https://hghalebi.github.io/category_theory_transformer_rs/

Public sprint guide:
https://github.com/hghalebi/category_theory_transformer_rs/blob/main/docs/review-sprint.md

Quick report form:
https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=quick-reader-report.yml

Full chapter-clarity form:
https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=chapter-clarity.yml
```

## No-Clone Sprint Slot

If one reviewer cannot clone the repository during the sprint, assign that
reviewer a public-book path instead of dropping the slot.

Public-book path:

```text
public book -> Welcome -> Course Map -> one core chapter or Exercises
```

The reviewer should stop at the first sentence, heading, diagram, table row,
code block, or exercise prompt that becomes unclear and open the closest report
link. The report must name the public page and one visible evidence signal from
that page.

A no-clone report can help fill the technical educator or beginner-adjacent
slot. It does not replace the Rust, ML, or category-theory command paths when
those command paths are the evidence being tested.

## Direct Evidence Checklist

A sprint result is useful only if each report is direct reader evidence.

Each report should come from one person who read or ran one path and submitted
one concrete blocked learning step. It should name:

- the reviewer perspective;
- the command, example, chapter, public page, or exercise tried;
- one evidence signal such as an output line, table row, compiler error, test
  name, constructor result, sentence, or exercise prompt;
- the last clear idea;
- the first unclear point;
- the smallest helpful change.

These do not count as sprint reports:

- the tracking issue for the sprint;
- context-routing issues for reviewer paths;
- broad praise or broad dislike;
- copied prompts or AI-generated critique;
- private conversation summaries;
- feature requests without a blocked learning step.

## Rewrite-Ready Sprint Check

Before closing the sprint, check each report against this standard:

```text
one reviewer
one route
one evidence signal
last clear idea
first unclear point
smallest useful fix
```

A sprint report is rewrite-ready when the maintainer can open one chapter,
example, table, diagram, or exercise and know what to improve next. If a report
only says that a topic is hard, ask for the first sentence, output line,
command result, or exercise prompt where the path stopped working.

## After The Sprint

Do not treat the sprint as finished just because five people were invited. A
useful sprint produces reports that can be triaged.

For each slot:

- if no report arrived, keep the slot open;
- if the report lacks a concrete signal, ask the reviewer for the first visible
  page, command, output line, sentence, table row, or exercise prompt;
- if the report is specific enough, accept it as reader evidence for that
  perspective;
- if the report leads to a rewrite, record the public file changed and the
  validation command that passed.

The sprint is strongest when at least one accepted report becomes a visible
rewrite. That proves the review loop can change the book, not only collect
opinions.

## Reviewer Paths

| Perspective | Start with | Evidence to look for |
| --- | --- | --- |
| Rust engineer | [Rust path](rust-path.md) | first unclear type, constructor, trait boundary, compiler signal, test name, or example output |
| ML engineer or learner | [ML path](ml-path.md) | first unclear logits, probability, loss, training update, attention role, or state transition |
| Category-theory reader | [Category-theory path](category-theory-path.md) | first overloaded term, missing law, unclear morphism shape, roadmap precision-drill row, or precision concern |
| Technical educator | [Educator path](educator-path.md) | first missing next action, weak practice loop, unclear exercise expectation, or feedback cue |
| Beginner-adjacent learner | [Beginner path](beginner-path.md) | first sentence, command, output line, or term that becomes too compressed |

If none of these fits, use the closest path and select `other` in the issue
form.

## Open The Right Report

Use these links during the sprint so each reviewer starts from the matching
route. The link fills the route, not the evidence. Each reviewer still writes
the evidence signal from their own reading or command run.

| Perspective | Report link |
| --- | --- |
| Rust engineer | [Open Rust engineer report](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=chapter-clarity.yml&title=%5Bgood+first+feedback%5D+Rust+engineer+brief&location=docs%2Frust-path.md&command=cargo+run+--example+01_domain_objects%0Acargo+run+--example+02_morphism_composition%0Acargo+test+domain%3A%3Atests+--lib%0Acargo+test+category%3A%3Atests+--lib) |
| ML engineer or learner | [Open ML engineer report](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=chapter-clarity.yml&title=%5Bgood+first+feedback%5D+ML+engineer+brief&location=docs%2Fml-path.md&command=cargo+run+--example+01_token_sequence%0Acargo+run+--bin+category_ml%0Acargo+run+--example+03_training_endomorphism%0Acargo+run+--example+07_transformer_training_state) |
| Category-theory reader | [Open category-theory reader report](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=chapter-clarity.yml&title=%5Bgood+first+feedback%5D+category-theory+reader+brief&location=book%2Fsrc%2Froadmap.md+-%3E+Category+Shape+Diagnostic+-%3E+Reader+Evidence+Handoff&command=cargo+run+--example+06_attention_scores) |
| Technical educator | [Open technical educator report](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=chapter-clarity.yml&title=%5Bgood+first+feedback%5D+technical+educator+brief&location=README.md%2C+START_HERE.md%2C+docs%2Feducator-path.md%2C+book%2Fsrc%2Fwelcome.md%2C+book%2Fsrc%2F00-map.md%2C+or+book%2Fsrc%2Fexercises.md&command=public+book+review+path+or+local+file+review) |
| Beginner-adjacent learner | [Open beginner-adjacent learner report](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=chapter-clarity.yml&title=%5Bgood+first+feedback%5D+beginner-adjacent+reader+brief&location=Welcome%2C+Course+Map%2C+Domain+Objects%2C+or+Morphism+and+Composition&command=cargo+run+--example+01_token_sequence+or+public+book+path) |

## Current Category-Precision Target

If the category-theory reviewer wants a narrow target, start with the roadmap
`Reader Evidence Handoff`:

```text
book/src/roadmap.md -> Category Shape Diagnostic -> Reader Evidence Handoff
book/src/roadmap.md -> Category Shape Diagnostic -> Source-Target Audit Card
cargo run --example 06_attention_scores
Q/K/V source diagnostic
query rows own score rows; key/value rows own score columns
mask polarity here: true = allowed, false = blocked
AttentionScores x AttentionMask -> AttentionScores
```

Then use the fixed-context attention drill as one concrete row to test:

```text
open boundary:
MaskedMultiHeadTransformerBlock : HiddenSequence x AttentionMask -> HiddenSequence

fixed context:
choose one mask M

induced view:
MaskedMultiHeadTransformerBlock[M] : HiddenSequence -> HiddenSequence
```

The reviewer should run:

```bash
cargo run --example 06_attention_scores
```

First inspect the diagnostic printed before the attention weights:

```text
Q/K/V source diagnostic:
query rows own score rows; key/value rows own score columns
self-attention shares the hidden source before projection; projected roles stay distinct
mask polarity here: true = allowed, false = blocked
```

Then read:

```text
book/src/roadmap.md -> Category Shape Diagnostic -> Reader Evidence Handoff
book/src/roadmap.md -> Category Shape Diagnostic -> Source-Target Audit Card
docs/category-theory-path.md -> Context-Fixing Mini-Drill
book/src/roadmap.md -> Context Fixing Drill
book/src/exercises.md -> Exercise 12
```

First, the report should say whether the Q/K/V diagnostic makes query
ownership, source ownership, and mask polarity visible before attention
weights appear.

Then the report should say whether the Source-Target Audit Card and
closure-capture analogy make the fixed `AttentionMask` visible, or whether
they accidentally hide the open product input. If the handoff itself is
unclear, the report should name the first missing field, audit-card row,
boundary row, output line, or prompt.

## Time Box

Use a 20-minute review window per person:

1. Five minutes: open the assigned path and run or read the first action.
2. Ten minutes: continue until the first real confusion point appears.
3. Five minutes: file one report with the exact command, page, output line,
   sentence, table row, or exercise that made the issue visible.

Stop at the first useful confusion point. Do not try to review the whole book
in one pass.

## Report Shape

A useful report has this shape:

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

The best evidence signal is small and visible.

Examples:

```text
Composition rule: Embedding then Softmax is illegal because Vector != Logits
loss before: 1.609389
TransformerTrainingState owns parameters, learning rate, and step count
the exercise asked for an endomorphism before I knew what object stayed the same
```

Do not copy those examples as reports. A sprint report must come from the
reviewer's own reading, command run, or exercise attempt.

## What Counts As A Good Sprint Result

A good sprint produces:

- one report from each perspective;
- no private contact details;
- one exact blocked learning step per report;
- enough evidence that the next rewrite can target a chapter, example, command,
  table, or exercise.

Broad praise, broad dislike, and new feature ideas are less useful than one
precise place where a reader got stuck.
