# Starter Issues

These are ready-to-open issue drafts. Keep issue titles concrete so new
contributors know exactly what help is useful.

## Good First Feedback

Title:

```text
[good first feedback] Where does the book become unclear?
```

Prompt:

```text
Read from START_HERE.md into the early book chapters. Quote the first sentence,
section, code block, or command where the path slows down. Explain what you
expected next.
```

## Diagram Review

Title:

```text
[needs diagram] Review the first pipeline diagrams
```

Prompt:

```text
Review the diagrams in the Course Map, Morphism and Composition, Tiny ML
Pipeline, and Training as an Endomorphism chapters. Quote the first diagram that
is unclear, then suggest one concrete simplification or missing label.
```

## Reader Review

Opened tracking issue:

```text
https://github.com/hghalebi/category_theory_transformer_rs/issues/6
```

Title:

```text
[reader confusion] Complete a thirty-minute reader review
```

Prompt:

```text
Use community/reader-review-guide.md. Run the first example, read the entry
path, then review one core chapter. Quote the first point where your mental
model becomes uncertain and explain what you expected next.

Maintainers should triage resulting reports with
community/reader-feedback-triage.md.
```

## Attention Roadmap Review

Title:

```text
[ML intuition] Review the query-key attention example
```

Prompt:

```text
Run cargo run --example 06_attention_scores, then review src/attention.rs and
book/src/roadmap.md. Report whether the current example clearly separates the
implemented query-key score, mask, score-to-weight, value-mixing, and
head-concatenation, and output-projection boundaries from the block sketches.
Also report whether residual addition, normalization, position-wise
feed-forward, positional encoding, hidden projections, and the single-head and
multi-head blocks plus the masked block variant are clearly presented as
implemented. Finally, report whether the structured parameter object,
sequence-level readout, and training-state metadata are clear without implying
that a production Transformer trainer already exists. Include whether the
readout-only training step is clearly presented as real but deliberately
narrow, and whether the local feed-forward update is clearly separated from
the composed block update. Then report whether the composed update is clearly
separated from future reader-feedback-driven refinements to gradient checking.
```

## Rust Example

Title:

```text
[needs Rust example] Morphism as typed transformation
```

Prompt:

```text
Add or improve a small example that shows why two transformations compose only
when the output type of the first matches the input type of the second.
```

## Chapter Expansion

Title:

```text
[chapter expansion] Turn bullet sections into full explanations
```

Prompt:

```text
Find one section that reads too much like an outline. Expand it into learner
prose with one concrete Rust reference and one checkpoint question.
```

## FAQ

Title:

```text
[FAQ] What does this project unlock?
```

Prompt:

```text
Write a concise FAQ answer for readers asking what this project helps them do
that a framework-only tutorial does not.
```
