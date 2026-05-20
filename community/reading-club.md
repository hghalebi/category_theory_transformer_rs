# Reading Club

This project can be used as a small reading club for engineers learning Rust,
ML internals, and applied category theory together.

## Format

Use one session per chapter:

1. Run the chapter's example.
2. Read the source snapshot.
3. Answer the checkpoint.
4. Collect one confusion point.
5. Turn that confusion point into a chapter clarity report.

Use the public form:

<https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=chapter-clarity.yml>

For a reviewer route, use [docs/review-path.md](../docs/review-path.md).
For a multi-reader pass, use the
[public review sprint](../docs/review-sprint.md).
For report calibration, use
[direct reader report examples](../docs/review-examples.md).
For a shareable reviewer invitation, use [REVIEWERS.md](../REVIEWERS.md).

## Minimum Accepted Report

Before the session ends, make sure one participant can fill these fields:

```text
Perspective:
Command or page tried:
Evidence signal:
Last clear idea:
First unclear point:
Smallest useful fix:
```

If the group cannot name a visible evidence signal, return to the page,
terminal output, compiler error, table row, code block, type boundary, or
exercise prompt where the path first became unclear.

## Fast Report Links

Use the closest link at the end of the session. The link fills the route, not
the evidence; the report should still come from what a participant personally
read, ran, or attempted.

| Perspective | Report link |
| --- | --- |
| Rust engineer | [Open Rust engineer report](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=chapter-clarity.yml&title=%5Bgood+first+feedback%5D+Rust+engineer+brief&location=docs%2Frust-path.md&command=cargo+run+--example+01_domain_objects%0Acargo+run+--example+02_morphism_composition%0Acargo+test+domain%3A%3Atests+--lib%0Acargo+test+category%3A%3Atests+--lib) |
| ML engineer or learner | [Open ML engineer report](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=chapter-clarity.yml&title=%5Bgood+first+feedback%5D+ML+engineer+brief&location=docs%2Fml-path.md&command=cargo+run+--example+01_token_sequence%0Acargo+run+--bin+category_ml%0Acargo+run+--example+03_training_endomorphism%0Acargo+run+--example+07_transformer_training_state) |
| Category-theory reader | [Open category-theory reader report](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=chapter-clarity.yml&title=%5Bgood+first+feedback%5D+category-theory+reader+brief&location=docs%2Fcategory-theory-path.md+-%3E+Seven+Sketches+Transfer+Drill%3B+book%2Fsrc%2Froadmap.md+-%3E+Category+Shape+Diagnostic&command=cargo+run+--example+05_seven_sketches%0Acargo+run+--example+06_attention_scores) |
| Technical educator | [Open technical educator report](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=chapter-clarity.yml&title=%5Bgood+first+feedback%5D+technical+educator+brief&location=README.md%2C+START_HERE.md%2C+docs%2Feducator-path.md%2C+book%2Fsrc%2Fwelcome.md%2C+book%2Fsrc%2F00-map.md%2C+or+book%2Fsrc%2Fexercises.md&command=public+book+review+path+or+local+file+review) |
| Beginner-adjacent learner | [Open beginner-adjacent learner report](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=chapter-clarity.yml&title=%5Bgood+first+feedback%5D+beginner-adjacent+reader+brief&location=Welcome%2C+Course+Map%2C+Domain+Objects%2C+or+Morphism+and+Composition&command=cargo+run+--example+01_token_sequence+or+public+book+path) |

One useful report is enough. It should name:

- the command or page the group used
- the last idea that was clear
- the first sentence, output line, example, or exercise that became unclear
- the smallest sentence, diagram, example, or exercise that would help

## Facilitator Transcription

If a participant cannot open GitHub during the session, one facilitator can
transcribe one report into the quick or detailed form after the session.

Transcribe only one participant's concrete reading or command attempt. Do not
merge several people's comments into one report.

Use `reading-session report` in the command or page field, name the public
page, chapter, exercise, command, or output line the participant used, and
preserve the participant's last clear idea and first unclear point. Do not
include names, email addresses, private messages, or contact details.

The report still needs one visible evidence signal. A summary that only says
the group was confused is not direct reader evidence.

## Source And Citation Reports

If the group's confusion is about a reference, citation, source-backed claim,
or missing source, use the optional source-role field in the quick or detailed
report form.

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
example in [direct reader report examples](../docs/review-examples.md).

## First Four Sessions

1. Start Here and Course Map
2. Domain Objects
3. Morphisms and Composition
4. Tiny ML Pipeline

## Public Workshop

The first public workshop for this project is listed in
[workshops.md](workshops.md). Use it as the shared kickoff session for readers
who want a guided introduction before following the chapter path independently.

## Facilitation Rule

Do not let the session become abstract debate too early. Every term should
return to a Rust type, function, example, or test.

## Session Close

End each session by asking one participant to open a report while the context
is still fresh. Broad impressions are less useful than one exact location where the learning path broke.
