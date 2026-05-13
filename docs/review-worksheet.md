# Reader Review Worksheet

Use this worksheet while reading the book, running an example, or attempting an
exercise. The goal is one precise report, not a broad review.

## Minimum Accepted Report

Fill these six fields before adding anything else:

```text
Perspective:
Command or page tried:
Evidence signal:
Last clear idea:
First unclear point:
Smallest useful fix:
```

If those fields are empty, keep reading or running until you can point to one
visible sentence, output line, compiler error, table row, code block, type
boundary, or exercise prompt. One precise blocked step is enough.

Stop at the first place where the learning path becomes unclear. Then open the
quick report form:

<https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=quick-reader-report.yml>

If you can also explain what you expected and what happened instead, use the
detailed form:

<https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=chapter-clarity.yml>

## Copy/Paste Report Shape

```text
Perspective:
Chapter, section, or file:
Command or page tried:
Evidence signal:
First unclear point:
Last clear idea:
What would have helped:
```

## Live Session Closeout

During a workshop, reading session, or review sprint, fill the report shape
before the group moves to general discussion.

Use the closeout rule:

```text
one participant
one route
one evidence signal
one blocked learning step
```

A report written during the session is stronger than a memory reconstructed
later. If GitHub is unavailable, keep the same fields and let a facilitator
transcribe one participant's report after the session.

Do not turn the closeout into a poll. The book changes from precise blocked
learning steps, not from broad sentiment.

Good evidence is small and visible:

- one terminal output line;
- one compiler error;
- one test name;
- one Rust type boundary;
- one table row;
- one sentence, heading, diagram, code block, or exercise prompt.

Do not report broad praise, broad dislike, copied prompts, private notes, or
AI-generated critique. A useful report should come from your own reading,
command run, or exercise attempt.

## Group Facilitator Transcription

If a reading-club or workshop participant cannot open GitHub during the
session, a facilitator can transcribe one report into the quick or detailed
form.

Transcribe only one participant's concrete reading or command attempt. Do not
merge several people's comments into one report.

Use:

- `workshop report` or `reading-session report` in the command or page field;
- the public page, chapter, exercise, command, or output line the participant
  used;
- the participant's last clear idea and first unclear point;
- no names, email addresses, private messages, or contact details.

The report still needs one visible evidence signal. A facilitator summary that
only says the group was confused is not direct reader evidence.

## Optional Source-Role Check

Use this block when your report is about a reference, citation, source-backed
claim, or missing source.

```text
Source role:
Owned boundary:
Claim this source supports:
Claim this source does not support:
Local Rust file, type, example, or chapter section:
```

Use the source-role table in `book/src/references.md`:

```text
repository code and tests
official documentation
academic papers
open textbooks and university material
implementation bridges
learner-friction signals
```

The rule is simple: a source owns a boundary. Rust docs own Rust syntax.
Framework docs own framework API shape. Academic papers own their formal
claims. Learner-friction signals help decide what to explain more slowly, but
they do not prove technical correctness.

## Five Route Cards

Pick the closest route. One useful report from one route is enough.

| Perspective | Run or read | Evidence signal to look for | Report link |
| --- | --- | --- | --- |
| Rust engineer | `cargo run --example 01_domain_objects`; `cargo run --example 02_morphism_composition`; `cargo test domain::tests --lib`; `cargo test category::tests --lib` | first unclear type, constructor, trait boundary, compiler signal, test name, or output line | [Open Rust report](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=chapter-clarity.yml&title=%5Bgood+first+feedback%5D+Rust+engineer+brief&location=docs%2Frust-path.md&command=cargo+run+--example+01_domain_objects%0Acargo+run+--example+02_morphism_composition%0Acargo+test+domain%3A%3Atests+--lib%0Acargo+test+category%3A%3Atests+--lib) |
| ML engineer or learner | `cargo run --example 01_token_sequence`; `cargo run --bin category_ml`; `cargo run --example 03_training_endomorphism`; `cargo run --example 07_transformer_training_state` | first unclear logits, probability, loss, training update, attention role, or state transition | [Open ML report](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=chapter-clarity.yml&title=%5Bgood+first+feedback%5D+ML+engineer+brief&location=docs%2Fml-path.md&command=cargo+run+--example+01_token_sequence%0Acargo+run+--bin+category_ml%0Acargo+run+--example+03_training_endomorphism%0Acargo+run+--example+07_transformer_training_state) |
| Category-theory reader | `cargo run --example 02_morphism_composition`; `cargo run --example 04_structure_and_calculus`; `cargo run --example 06_attention_scores`; read `docs/category-theory-path.md` and the roadmap Source-Target Audit Card | first overloaded term, unclear morphism shape, source-target mismatch, fixed-context boundary, endomorphism claim, or linear-scope warning | [Open category-theory report](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=chapter-clarity.yml&title=%5Bgood+first+feedback%5D+category-theory+reader+brief&location=book%2Fsrc%2Froadmap.md+-%3E+Category+Shape+Diagnostic+-%3E+Reader+Evidence+Handoff&command=cargo+run+--example+06_attention_scores) |
| Technical educator | Read `README.md`, `START_HERE.md`, `docs/educator-path.md`, `book/src/welcome.md`, `book/src/00-map.md`, or `book/src/exercises.md` | first missing next action, weak practice loop, unclear exercise expectation, or missing feedback cue | [Open educator report](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=chapter-clarity.yml&title=%5Bgood+first+feedback%5D+technical+educator+brief&location=README.md%2C+START_HERE.md%2C+docs%2Feducator-path.md%2C+book%2Fsrc%2Fwelcome.md%2C+book%2Fsrc%2F00-map.md%2C+or+book%2Fsrc%2Fexercises.md&command=public+book+review+path+or+local+file+review) |
| Beginner-adjacent learner | `cargo run --example 01_token_sequence`, or public book path: Welcome -> Course Map -> Domain Objects -> Morphism and Composition | first sentence, command, output line, table row, diagram, or term that becomes too compressed | [Open beginner report](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=chapter-clarity.yml&title=%5Bgood+first+feedback%5D+beginner-adjacent+reader+brief&location=Welcome%2C+Course+Map%2C+Domain+Objects%2C+or+Morphism+and+Composition&command=cargo+run+--example+01_token_sequence+or+public+book+path) |

## Fast Triage Check

Before submitting, verify that your report answers these questions:

```text
What did I read or run?
What exact signal did I see?
What was clear immediately before the confusion?
What was the first unclear point?
What small change would have helped?
If this is about a source, what source role and owned boundary did I name?
```

If the report cannot point to one visible signal, it is probably still too
broad. Return to the page, command, output line, table row, code block, or
exercise prompt where the confusion first appeared.
