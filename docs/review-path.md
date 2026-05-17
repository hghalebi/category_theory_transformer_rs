# Review Path

Use this path if you want to help improve the book without writing code.

The goal is one precise chapter clarity report, not a broad review.

For the shareable reviewer call, use [../REVIEWERS.md](../REVIEWERS.md).
For a one-page fill-in guide, use the
[reader review worksheet](review-worksheet.md).

If several readers are reviewing together, use the
[public review sprint](review-sprint.md) to split the work across five reader
perspectives.

```text
run or read one path
name the command or page tried
name one visible evidence signal
name the last clear idea
name the first unclear point
suggest the smallest fix
```

## Minimum Accepted Report

A report can be accepted when it includes:

```text
Perspective:
Command or page tried:
Evidence signal:
Last clear idea:
First unclear point:
Smallest useful fix:
```

If one of those fields is missing, keep reading or running until you can point
to one visible sentence, output line, compiler error, table row, code block,
type boundary, or exercise prompt.

Open a quick report here:

<https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=quick-reader-report.yml>

Open the fuller chapter clarity report here:

<https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=chapter-clarity.yml>

If you are not sure what a useful report looks like, read
[direct reader report examples](review-examples.md) first.

## No-Clone Review

You can review without cloning the repository.

1. Open the public book:
   <https://hghalebi.github.io/category_theory_transformer_rs/>
2. Read one page: Welcome, Course Map, Domain Objects, Morphism and
   Composition, or Exercises.
3. Stop at the first sentence, heading, diagram, table row, code block, or
   exercise prompt that becomes unclear.
4. Open the closest report link and put the public page in `Command or page
   tried`.

A no-clone report counts as direct reader evidence only when it names the
public page and one visible evidence signal from that page. It does not count
when it only says the project looks interesting, confusing, or useful.

Fastest no-clone path:
[Open beginner-adjacent learner report](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=chapter-clarity.yml&title=%5Bgood+first+feedback%5D+beginner-adjacent+reader+brief&location=Welcome%2C+Course+Map%2C+Domain+Objects%2C+or+Morphism+and+Composition&command=public+book+path).

## What Counts As Direct Reader Evidence

A report counts as direct reader evidence only when it comes from your own
reading or command run and names the exact signal that made the issue visible.

It should include:

- your closest perspective;
- the command, example, exercise, chapter, or public page you tried;
- one evidence signal such as an output line, compiler error, constructor
  result, test name, table row, sentence, or exercise prompt;
- the last idea that was clear;
- the first point where the path stopped working;
- the smallest change that would have helped.

These do not count as direct reader evidence:

- stars, likes, or broad praise;
- broad dislike without a blocked learning step;
- routing issues that only point to a path;
- copied prompts or AI-generated critique;
- summaries of private conversations;
- feature requests that do not name the first unclear point.

The project can use broad feedback as context, but the textbook completion gate
requires accepted direct reports from readers who actually read or ran part of
the project.

## If The Issue Is A Source Or Citation

Some reports are about the boundary between an external source and the local
Rust teaching claim. Use the optional source-role field in either report form
when the issue is about a reference, citation, source-backed claim, or missing
source.

Name:

- source role;
- owned boundary;
- claim this source supports;
- claim this source does not support;
- local Rust file, type, example, or chapter section.

Good source reports do not ask for "more citations" in general. They point to
one source-backed sentence, table row, chapter reference, code example, or
missing source boundary and say what claim became unclear.

Use the source-role table in `book/src/references.md` and the source-role
example in [direct reader report examples](review-examples.md).

## Rewrite-Ready Checklist

Before submitting, check that the report is specific enough to become a small
rewrite.

It is ready when it has:

- one person reading or running one concrete path;
- one named command, public page, chapter, file, or exercise;
- one visible evidence signal from the book, terminal output, compiler output,
  test output, table row, diagram, or exercise prompt;
- the last clear idea separated from the first unclear point;
- a suggested fix small enough to apply to one chapter, example, table,
  diagram, or exercise.

The report does not need to solve the issue. It should give enough evidence
that a maintainer can open the affected section and rewrite without guessing.

## After Submission

The next maintainer action depends on how specific the report is.

| Report state | Maintainer action |
| --- | --- |
| Missing location or evidence signal | Ask for the first visible page, command, output line, sentence, table row, or exercise prompt |
| Specific and rewrite-ready | Accept it as reader evidence and target one public chapter, example, table, diagram, or exercise |
| Repeated theme across readers | Batch it as a theme until the affected section is clear enough to rewrite |
| Rewritten | Link the landed change and validation command so the report has a visible outcome |

This is why the form asks for the last clear idea and the first unclear point:
those two fields tell the maintainer where the rewrite should begin.

## Public Report Lifecycle

A direct reader report is useful because it can become a visible public
change. The path should stay small:

```text
submit one evidence-backed report
-> maintainer triage
-> clarification, accepted evidence, or out-of-scope close-out
-> focused rewrite or batched theme
-> validation command linked back to the report
```

Accepted evidence does not mean the whole book is finished. It means one
reader found one concrete blocked step and the project can either rewrite that
spot or record a repeated theme for a later pass.

When a report is rewritten, the public close-out should name:

- the changed public file;
- the reader evidence signal that drove the change;
- the validation command that passed;
- any remaining risk that still needs reader feedback.

This keeps the review loop inspectable without exposing private notes,
maintainer planning, or private maintenance details.

## Choose One Path

| Perspective | Public path | First action |
| --- | --- | --- |
| Rust engineer | [Rust engineer first-run path](https://github.com/hghalebi/category_theory_transformer_rs/issues/8) | Run `cargo run --example 01_domain_objects` and `cargo run --example 02_morphism_composition` |
| ML engineer or learner | [ML engineer training and attention path](https://github.com/hghalebi/category_theory_transformer_rs/issues/9) | Run `cargo run --example 03_training_endomorphism` and `cargo run --example 07_transformer_training_state` |
| Category-theory reader | [Category-theory precision path](https://github.com/hghalebi/category_theory_transformer_rs/issues/10) | Run `cargo run --example 02_morphism_composition` and `cargo run --example 06_attention_scores` |
| Technical educator | [Technical educator learning path](https://github.com/hghalebi/category_theory_transformer_rs/issues/11) | Review `README.md`, `START_HERE.md`, `docs/educator-path.md`, and `book/src/exercises.md` |
| Beginner-adjacent learner | [Beginner-adjacent first confusion path](https://github.com/hghalebi/category_theory_transformer_rs/issues/12) | Run `cargo run --example 01_token_sequence` or read the public start path |

If none of those fits, choose the closest one and select `other` in the issue
form.

## Open The Right Report

These links open the public chapter-clarity form with the route already filled.
Use one, then add the evidence signal from your own reading or command run.
If you have one signal but do not need the prefilled route, use the
[quick reader report form](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=quick-reader-report.yml).

| Perspective | Report link |
| --- | --- |
| Rust engineer | [Open Rust engineer report](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=chapter-clarity.yml&title=%5Bgood+first+feedback%5D+Rust+engineer+brief&location=docs%2Frust-path.md&command=cargo+run+--example+01_domain_objects%0Acargo+run+--example+02_morphism_composition%0Acargo+test+domain%3A%3Atests+--lib%0Acargo+test+category%3A%3Atests+--lib) |
| ML engineer or learner | [Open ML engineer report](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=chapter-clarity.yml&title=%5Bgood+first+feedback%5D+ML+engineer+brief&location=docs%2Fml-path.md&command=cargo+run+--example+01_token_sequence%0Acargo+run+--bin+category_ml%0Acargo+run+--example+03_training_endomorphism%0Acargo+run+--example+07_transformer_training_state) |
| Category-theory reader | [Open category-theory reader report](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=chapter-clarity.yml&title=%5Bgood+first+feedback%5D+category-theory+reader+brief&location=book%2Fsrc%2Froadmap.md+-%3E+Category+Shape+Diagnostic+-%3E+Reader+Evidence+Handoff&command=cargo+run+--example+06_attention_scores) |
| Technical educator | [Open technical educator report](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=chapter-clarity.yml&title=%5Bgood+first+feedback%5D+technical+educator+brief&location=README.md%2C+START_HERE.md%2C+docs%2Feducator-path.md%2C+book%2Fsrc%2Fwelcome.md%2C+book%2Fsrc%2F00-map.md%2C+or+book%2Fsrc%2Fexercises.md&command=public+book+review+path+or+local+file+review) |
| Beginner-adjacent learner | [Open beginner-adjacent learner report](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=chapter-clarity.yml&title=%5Bgood+first+feedback%5D+beginner-adjacent+reader+brief&location=Welcome%2C+Course+Map%2C+Domain+Objects%2C+or+Morphism+and+Composition&command=cargo+run+--example+01_token_sequence+or+public+book+path) |

## Current High-Value Review Target

The weakest remaining non-code review target is category precision in the
Transformer Roadmap. Start with the roadmap `Reader Evidence Handoff`:

```text
book/src/roadmap.md -> Category Shape Diagnostic -> Reader Evidence Handoff
book/src/roadmap.md -> Category Shape Diagnostic -> Source-Target Audit Card
cargo run --example 06_attention_scores
AttentionScores x AttentionMask -> AttentionScores
```

A useful report can focus on this single question:

```text
Does the Source-Target Audit Card make the fixed context visible?
```

Run:

```bash
cargo run --example 06_attention_scores
```

Then read:

```text
book/src/roadmap.md -> Category Shape Diagnostic -> Reader Evidence Handoff
book/src/roadmap.md -> Category Shape Diagnostic -> Source-Target Audit Card
docs/category-theory-path.md -> Context-Fixing Mini-Drill
book/src/roadmap.md -> Context Fixing Drill
book/src/exercises.md -> Exercise 12
```

A strong report names one exact signal:

```text
Evidence signal: query rows own score rows; key/value rows own score columns
Question tested: did I know which side owns score rows and score columns before reading attention weights?
Last clear idea: the diagnostic says query rows own score rows
First unclear point: I did not know whether self-attention makes Q, K, and V the same role
What would have helped: one sentence saying shared source before projection does not erase projected roles
```

Or:

```text
Evidence signal: AttentionScores x AttentionMask -> AttentionScores
Question tested: did I name the whole source object before deciding whether the row was an endomorphism?
Last clear idea: the open block needs both hidden state and mask
First unclear point: the closure-capture analogy did not make it clear which value was fixed
What would have helped: name fixed_mask directly before the code block
```

If the explanation works, report the first boundary row, output line, or
sentence that made the difference. If it fails, report the first handoff field,
diagnostic line, audit-card row, or sentence where source ownership, mask
polarity, or mask context disappeared.

## What To Report

A useful report includes:

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

The strongest report points to one exact location.

Useful:

```text
Command or page tried: cargo run --example 02_morphism_composition
Evidence signal: Embedding then Softmax is illegal because Vector != Logits
Last clear idea: The TokenId -> Vector step made sense.
First unclear point: I did not yet know why Softmax needs Logits.
What would have helped: a one-sentence reminder before the illegal composition line.
```

Less useful:

```text
The category theory chapter needs more explanation.
```

The second report names a topic, but not the first blocked step.

## Report Quality Rubric

Use this rubric before submitting. A report is easiest to act on when it points
to evidence, not only a reaction.

| Report quality | What it contains | Example signal |
| --- | --- | --- |
| ready to act on | exact location, command or page tried, evidence signal, last clear idea, first unclear point, and smallest useful fix | `Vector != Logits` appeared before the reason `Softmax` needs `Logits` |
| needs clarification | real confusion, but missing the exact sentence, output line, command, evidence signal, or expected next step | "composition was confusing" |
| not useful yet | broad praise, broad dislike, strategy advice, or a new feature request without a blocked learning step | "add more category theory" |

A ready report does not need to be long. It should let a maintainer open one
chapter or example and know what to improve next.

For five concrete shapes, see
[direct reader report examples](review-examples.md). The examples are not real
reports and should not be copied as your own evidence.

## What To Avoid

Do not include:

- email addresses
- private messages
- personal notes
- broad praise without a blocked step
- broad strategy requests unrelated to one chapter, example, or exercise

## Good Review Outcome

A good 20-minute review should make one next edit obvious.

The report does not need to solve the problem. It only needs to show exactly
where the current learning path stopped working.
