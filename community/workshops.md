# Workshops

This page lists public sessions for Category Theory for Tiny ML in Rust.

## First Public Workshop

The first public workshop introduces the project as a public book and Rust lab
for engineers who want to understand AI systems below the framework layer.

The session uses the same core path as the repository:

```text
Text -> Tokens -> Training Pairs -> Model State -> Prediction -> Loss -> Updated Model State
```

Register through Luma:

<a href="https://luma.com/event/evt-Pb1kYMQvzs8JrQq" class="luma-checkout--button" data-luma-action="checkout" data-luma-event-id="evt-Pb1kYMQvzs8JrQq">Register for Event</a>

<script id="luma-checkout" src="https://embed.lu.ma/checkout-button.js"></script>

Plain link:

[Register for the public workshop](https://luma.com/event/evt-Pb1kYMQvzs8JrQq)

## Evidence-Producing Run Sheet

Use this run sheet when the goal is both learning and useful public feedback.
The session should leave each participant with one command, page, output line,
or exercise prompt they can point to.

| Time | Activity | Evidence to capture |
| --- | --- | --- |
| 0-5 min | Open the public book, repository, and worksheet | which route each participant chooses |
| 5-15 min | Run the first win: `cargo run --example 01_token_sequence` | one output line that was clear or unclear |
| 15-25 min | Read the matching start path: `START_HERE.md`, Rust path, ML path, category-theory path, educator path, or beginner path | first sentence, table row, command, or term that needs support |
| 25-40 min | Split by perspective and run one route from the worksheet | one route-specific evidence signal |
| 40-50 min | Fill the closeout shape before general discussion | last clear idea, first unclear point, smallest useful fix |
| 50-60 min | Submit or transcribe one report per perspective when possible | one public issue per concrete blocked learning step |

Route assignments:

| Perspective | Suggested route |
| --- | --- |
| Rust engineer | `cargo run --example 01_domain_objects`, then `cargo run --example 02_morphism_composition` |
| ML engineer or learner | `cargo run --example 01_token_sequence`, then `cargo run --example 03_training_endomorphism` |
| Category-theory reader | `cargo run --example 06_attention_scores`, then the roadmap Source-Target Audit Card |
| Technical educator | `README.md`, `START_HERE.md`, `docs/educator-path.md`, and `book/src/exercises.md` |
| Beginner-adjacent learner | public book path: Welcome -> Course Map -> Domain Objects -> Morphism and Composition |

Do not require every participant to finish every route. The workshop is useful
when each person stops at the first precise learning break and records it.
One exact blocked learning step is better than a broad complete-book opinion.

## Minimum Accepted Report

Before opening the issue form, make sure the participant can fill these fields:

```text
Perspective:
Command or page tried:
Evidence signal:
Last clear idea:
First unclear point:
Smallest useful fix:
```

If the participant cannot name a visible evidence signal, send them back to the
page, command output, compiler error, table row, code block, type boundary, or
exercise prompt where the learning path first became unclear.

## After The Workshop

The most useful follow-up is one precise chapter clarity report:

<https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=chapter-clarity.yml>

Report the first place where the session or public book became unclear.
If participants are new to this feedback format, use the
[direct reader report examples](../docs/review-examples.md) before opening an
issue.
For a public call that can be shared with potential reviewers, use
[REVIEWERS.md](../REVIEWERS.md).

## Fast Report Links

Use the closest link after the session. The link fills the route, not the
evidence; the report should still come from what the participant personally
read, ran, or attempted.

| Perspective | Report link |
| --- | --- |
| Rust engineer | [Open Rust engineer report](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=chapter-clarity.yml&title=%5Bgood+first+feedback%5D+Rust+engineer+brief&location=docs%2Frust-path.md&command=cargo+run+--example+01_domain_objects%0Acargo+run+--example+02_morphism_composition%0Acargo+test+domain%3A%3Atests+--lib%0Acargo+test+category%3A%3Atests+--lib) |
| ML engineer or learner | [Open ML engineer report](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=chapter-clarity.yml&title=%5Bgood+first+feedback%5D+ML+engineer+brief&location=docs%2Fml-path.md&command=cargo+run+--example+01_token_sequence%0Acargo+run+--bin+category_ml%0Acargo+run+--example+03_training_endomorphism%0Acargo+run+--example+07_transformer_training_state) |
| Category-theory reader | [Open category-theory reader report](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=chapter-clarity.yml&title=%5Bgood+first+feedback%5D+category-theory+reader+brief&location=book%2Fsrc%2Froadmap.md+-%3E+Category+Shape+Diagnostic+-%3E+Reader+Evidence+Handoff&command=cargo+run+--example+06_attention_scores) |
| Technical educator | [Open technical educator report](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=chapter-clarity.yml&title=%5Bgood+first+feedback%5D+technical+educator+brief&location=README.md%2C+START_HERE.md%2C+docs%2Feducator-path.md%2C+book%2Fsrc%2Fwelcome.md%2C+book%2Fsrc%2F00-map.md%2C+or+book%2Fsrc%2Fexercises.md&command=public+book+review+path+or+local+file+review) |
| Beginner-adjacent learner | [Open beginner-adjacent learner report](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=chapter-clarity.yml&title=%5Bgood+first+feedback%5D+beginner-adjacent+reader+brief&location=Welcome%2C+Course+Map%2C+Domain+Objects%2C+or+Morphism+and+Composition&command=cargo+run+--example+01_token_sequence+or+public+book+path) |

Useful reports include:

- the command or public page you tried
- the first unclear sentence, output line, table row, example, or exercise
- the last idea that still made sense
- what you expected the next explanation to do
- the smallest fix that would help the next reader

## Facilitator Transcription

If a participant cannot open GitHub during the workshop, one facilitator can
transcribe one report into the quick or detailed form after the session.

Transcribe only one participant's concrete reading or command attempt. Do not
merge several people's comments into one report.

Use `workshop report` in the command or page field, name the public page,
chapter, exercise, command, or output line the participant used, and preserve
the participant's last clear idea and first unclear point. Do not include
names, email addresses, private messages, or contact details.

The report still needs one visible evidence signal. A summary that only says
the workshop was confusing is not direct reader evidence.

## Workshop Closeout

Reserve the last five minutes for evidence capture. Do not wait until the next
day, when the exact confusing sentence, output line, or exercise prompt has
already faded.

Use this closeout shape:

```text
Perspective:
Command or public page tried:
Evidence signal:
Last clear idea:
First unclear point:
Smallest useful fix:
```

The closeout rule is:

```text
one participant
one route
one evidence signal
one blocked learning step
```

A report submitted during the workshop is stronger than a memory reconstructed
later. If the participant cannot use GitHub, the facilitator can transcribe the
same fields after the session under the facilitator transcription rule above.

Do not turn the closeout into a poll. Votes can show interest, but a rewrite
needs one exact location where the learning path broke.

## Source And Citation Reports

If a workshop report is about a reference, citation, source-backed claim, or
missing source, use the optional source-role field in the quick or detailed
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

If a workshop group wants to split the review work, use the
[public review sprint](../docs/review-sprint.md) so each perspective produces
one concrete report.
