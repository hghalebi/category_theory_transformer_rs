# Chapter Quality Scorecard

Status: Active rewrite support

This scorecard is an editorial tool for the next rewrite pass. It does not
declare the book finished. A chapter can be source-backed and runnable while
still needing direct reader evidence.

Rubric:

- Strong: the current draft has source grounding, code evidence, and a clear
  learner path for this dimension.
- Developing: the dimension exists, but the next rewrite should sharpen it.
- Reader-needed: the dimension cannot be called strong until direct reader
  reports confirm it works for motivated learners.
- Pending: no accepted direct-reader report has been recorded for the chapter.

| Chapter | Source grounding | Rust clarity | ML intuition | Category precision | Learner path | Practice transfer | Direct-reader evidence | Next rewrite focus |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| [Welcome](src/welcome.md) | Strong | Strong | Strong | Strong | Strong | Strong | Pending | Validate the first-output transfer checklist with a beginner-adjacent reader report |
| [Course Map](src/00-map.md) | Strong | Strong | Strong | Strong | Strong | Strong | Pending | Validate the demo-output wayfinding checklist with a first-session reader |
| [Domain Objects](src/01-domain-objects.md) | Strong | Strong | Strong | Strong | Strong | Strong | Pending | Validate the example-output transfer checklist with a Rust reader |
| [Morphism and Composition](src/02-morphisms-composition.md) | Strong | Strong | Strong | Strong | Strong | Strong | Pending | Validate the stage-output transfer checklist with a reader after they run example 02 |
| [The Tiny ML Pipeline](src/03-ml-pipeline.md) | Strong | Strong | Strong | Strong | Strong | Strong | Pending | Validate the demo-output transfer checklist with an ML learner |
| [Training as an Endomorphism](src/04-training-endomorphism.md) | Strong | Strong | Strong | Strong | Strong | Strong | Pending | Validate the example-output transfer checklist with an ML learner |
| [Functors, Naturality, Monoids, and Chain Rule](src/05-structure-and-calculus.md) | Strong | Strong | Strong | Strong | Strong | Strong | Pending | Validate the example-output transfer checklist with a category-theory reader report |
| [Seven Sketches Through Rust](src/seven-sketches-rust.md) | Strong | Strong | Strong | Strong | Strong | Strong | Pending | Validate the example-output transfer checklist with readers who came for tiny ML |
| [Exercises](src/exercises.md) | Strong | Strong | Strong | Strong | Strong | Strong | Pending | Validate the worked mixed-boundary diagnosis with exercise-attempt evidence |
| [Transformer Roadmap](src/roadmap.md) | Strong | Strong | Strong | Reader-needed | Strong | Strong | Pending | Validate the checker-protected self-vs-cross attention boundary, category-shape diagnostic, and linear-vs-nonlinear attention warning with readers after they run example 06, then tighten any product-input, endomorphism, or endofunctor confusion |

## Scorecard Rule

Use this scorecard before the next rewrite pass:

1. Pick the chapter with the most Developing and Reader-needed cells.
2. Re-read its chapter contract, source bucket, and reference-map row.
3. Rewrite only the weakest dimension first.
4. Record the critique and rewrite decision in `book/CHAPTER_REWRITE_LOG.md`.
5. If a direct reader report exists, cite it in
   `community/reader-feedback-inbox.md` and update the Direct-reader evidence
   cell only after the report has been accepted and rewritten.
