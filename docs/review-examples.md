# Direct Reader Report Examples

Use this page before opening a chapter clarity report.

These are example shapes, not real reader reports. Do not copy them as your own
evidence. A real report must come from your own reading, command run, exercise
attempt, or public-page review.

Open the quick report form here:

<https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=quick-reader-report.yml>

Open the detailed chapter clarity form here:

<https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=chapter-clarity.yml>

## What A Strong Report Does

A strong report names one blocked learning step:

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

The evidence signal should be something visible: an output line, compiler
error, constructor result, test name, table row, sentence, heading, or exercise
prompt.

## Quick Reader Report Example

Use this shape when you have one concrete signal and do not need the fuller
expected-versus-actual form.

```text
Perspective: beginner-adjacent learner
Chapter or file: START_HERE.md
Command or page tried: cargo run --example 01_token_sequence
Evidence signal: TokenSequence: [TokenId(12), TokenId(44), TokenId(7), TokenId(19), TokenId(91)]
First unclear sentence, output line, table row, code block, or exercise:
The output shows TokenId numbers, but I did not know whether the exact numbers
matter outside this example.
Last clear idea:
The raw input text became a typed sequence.
What would have helped:
Add one sentence saying the exact IDs are local to this tiny demo; the important
thing is the typed boundary from text to token structure.
```

Why this is useful: it uses the shorter form, names one command, points to one
visible output line, and asks for one small clarification. It does not need a
full theory review to become actionable.

## Rust Engineer Example

```text
Perspective: Rust engineer
Friction lens: Rust syntax or idiom
Chapter or file: book/src/02-morphisms-composition.md
Command or page tried: cargo run --example 02_morphism_composition
Evidence signal: Embedding then Softmax is illegal because Vector != Logits
First unclear sentence, output line, table row, code block, or exercise:
The output says the composition is illegal before I know why Softmax expects Logits.
Last clear idea:
TokenId -> Vector made sense, and Vector -> Logits made sense.
What you expected:
A one-line reminder that Softmax is defined on scores, not embedding vectors.
What happened instead:
I understood the type mismatch but not the ML reason behind the mismatch.
What would have helped:
Add one sentence before the illegal-composition line: "Softmax converts logits,
not arbitrary vectors, into probabilities."
```

Why this is useful: it names the command, output line, last clear idea, first
unclear point, and a small edit.

## ML Engineer Or Learner Example

```text
Perspective: ML engineer or learner
Friction lens: ML intuition
Chapter or file: book/src/04-training-endomorphism.md
Command or page tried: cargo run --example 03_training_endomorphism
Evidence signal: Measurement: Parameters x TrainingSet -> Loss
First unclear sentence, output line, table row, code block, or exercise:
The example separates measurement from TrainStep, but the chapter did not make
that split concrete early enough for me.
Last clear idea:
loss before and loss after show the model improved.
What you expected:
I expected the chapter to say which part measures loss and which part changes state.
What happened instead:
I had to infer that loss measurement is not the same arrow as the parameter update.
What would have helped:
Add a two-row table: measure uses Parameters x TrainingSet -> Loss; update uses
Parameters -> Parameters.
```

Why this is useful: it does not ask for "more ML"; it names the missing bridge.

## Category-Theory Reader Example

```text
Perspective: category-theory reader
Friction lens: category-theory precision
Chapter or file: book/src/roadmap.md
Command or page tried: cargo run --example 06_attention_scores
Evidence signal: AttentionScores x AttentionMask -> AttentionScores
First unclear sentence, output line, table row, code block, or exercise:
The roadmap calls some updates endomorphisms, but I needed a sharper contrast
between product-input morphisms and endomorphisms.
Last clear idea:
LayerNormalization : HiddenSequence -> HiddenSequence is a unary endomorphism
for one fixed layer instance.
What you expected:
I expected the mask row to say why a two-input boundary is not automatically an
endomorphism even if the output type repeats one input type.
What happened instead:
The count-inputs rule was present, but I did not notice it before the attention table.
What would have helped:
Add a short note beside the mask row: "Count inputs first; this is
AttentionScores x AttentionMask -> AttentionScores, not simply
AttentionScores -> AttentionScores."
```

Why this is useful: it names the exact morphism shape where precision matters.

## Category-Theory Fixed-Context Example

```text
Perspective: category-theory reader
Friction lens: category-theory precision
Chapter or file: docs/category-theory-path.md
Command or page tried: cargo run --example 06_attention_scores, then Context-Fixing Mini-Drill
Evidence signal: MaskedMultiHeadTransformerBlock[M] : HiddenSequence -> HiddenSequence
First unclear sentence, output line, table row, code block, or exercise:
The closure example helped, but I needed the text to say that fixed_mask is the
captured context before the unary view is named.
Last clear idea:
MaskedMultiHeadTransformerBlock : HiddenSequence x AttentionMask -> HiddenSequence
is the open boundary.
What you expected:
I expected the induced view to name exactly which context was fixed.
What happened instead:
I understood the Rust closure shape but was not sure whether the category name
still depended on the same mask for every call.
What would have helped:
Add one sentence after the code block: "This view is valid only while the same
fixed_mask remains the captured context."
```

Why this is useful: it validates the precise bridge between a Rust closure,
the fixed mask context, and the induced unary category shape.

## Category-Theory Residual Contrast Example

```text
Perspective: category-theory reader
Friction lens: category-theory precision
Chapter or file: docs/category-theory-path.md -> Context-Fixing Mini-Drill
Command or page tried: cargo run --example 06_attention_scores
Evidence signal: ResidualConnection : HiddenSequence x ProjectedAttentionOutput -> HiddenSequence
First unclear sentence, output line, table row, code block, or exercise:
The fixed-context drill contrasts masked attention with residual addition, but
I did not know why residual addition was not another fixed-context view.
Last clear idea:
A fixed mask can induce a `HiddenSequence -> HiddenSequence` view for that
chosen mask.
What you expected:
I expected the residual row to say which input, if any, had been selected in
advance.
What happened instead:
I saw that the output is `HiddenSequence` and almost called the boundary unary.
What would have helped:
Add a note that residual addition still receives both inputs: the old hidden
stream and the projected sublayer output. Nothing has been fixed.
```

Why this is useful: it names the exact residual boundary and tests whether the
reader can distinguish fixing context from merely returning the left-hand
object.

## Category-Theory Roadmap Handoff Example

```text
Perspective: category-theory reader
Friction lens: category-theory precision
Chapter or file: book/src/roadmap.md -> Category Shape Diagnostic -> Reader Evidence Handoff
Command or page tried: cargo run --example 06_attention_scores
Evidence signal: AttentionScores x AttentionMask -> AttentionScores
Question tested: did the Source-Target Audit Card make the whole source object visible?
First unclear sentence, output line, table row, code block, or exercise:
The handoff asked for the first failed rule, but I did not know whether my
confusion was "input count" or "fixed context."
Last clear idea:
AttentionScores -> AttentionWeights is an ordinary morphism from scores to
probabilities.
What you expected:
I expected the handoff to map the mask row directly to the rule I should test.
What happened instead:
I could name the row, but not the specific rule that failed for me.
What would have helped:
Add "for the mask row, start with input count; then ask whether a mask has been
fixed as context" beside the handoff checklist.
```

Why this is useful: it tests the exact public handoff that should turn a
roadmap confusion into one actionable chapter-clarity report.

## Category-Theory Seven Sketches Transfer Example

```text
Perspective: category-theory reader
Friction lens: category-theory precision
Chapter or file: docs/category-theory-path.md -> Seven Sketches Transfer Drill
Command or page tried: cargo run --example 05_seven_sketches
Evidence signal: SignalMatrix::compose_after rejects mismatched middle dimension
Question tested: did the transfer row connect the sketch law to a tiny ML shortcut?
First unclear sentence, output line, table row, code block, or exercise:
The drill named matrix composition, but I did not know which tiny ML shortcut
was being rejected.
Last clear idea:
The output feature dimension of one stage must match the input feature
dimension of the next stage.
What you expected:
I expected the row to name the bad shortcut directly.
What happened instead:
I saw the Rust boundary check but did not connect it back to ML pipeline safety.
What would have helped:
Add "bad shortcut rejected: matrix-shaped data can be wired by position alone"
beside the SignalMatrix row.
```

Why this is useful: it checks whether Seven Sketches vocabulary becomes a
small typed ML safety lesson instead of a detached analogy.

## Technical Educator Example

```text
Perspective: technical educator
Friction lens: learner path or next action
Chapter or file: book/src/exercises.md
Command or page tried: read the beginner exercises after START_HERE.md
Evidence signal: which code boundary or output line proves your answer
First unclear sentence, output line, table row, code block, or exercise:
The first exercise asks for an explanation, but a learner may not know what a
complete answer should contain.
Last clear idea:
The first-session checkpoints made the read-run-explain loop clear.
What you expected:
I expected the first exercise to include one tiny model answer or answer shape.
What happened instead:
The learner has to infer how much Rust evidence is enough.
What would have helped:
Add a one-line answer shape: "My answer should name one type, one invalid state
it prevents, and one command output that shows the boundary."
```

Why this is useful: it targets the practice loop instead of giving broad
pedagogy advice.

## Beginner-Adjacent Learner Example

```text
Perspective: beginner-adjacent learner
Friction lens: learner path or next action
Chapter or file: START_HERE.md
Command or page tried: cargo run --example 01_token_sequence
Evidence signal: [TokenId(12), TokenId(44), TokenId(7), TokenId(19), TokenId(91)]
First unclear sentence, output line, table row, code block, or exercise:
I saw TokenId numbers but did not know whether the exact numbers mattered.
Last clear idea:
The raw input sentence became a sequence of tokens.
What you expected:
I expected a quick note explaining that these are stable IDs in this tiny demo,
not universal token IDs from a production tokenizer.
What happened instead:
I wondered whether TokenId(12) had a special global meaning.
What would have helped:
Add one sentence after the output: "The exact numbers are demo-local token IDs;
the important idea is that text has become typed token structure."
```

Why this is useful: it identifies a first-session confusion before the reader
falls into a side question.

## No-Clone Public-Book Example

```text
Perspective: beginner-adjacent learner
Friction lens: learner path or next action
Chapter or file: public book -> Course Map
Command or page tried: public book path
Evidence signal: Text -> TokenSequence -> TrainingPairs
First unclear sentence, output line, table row, code block, or exercise:
The Course Map repeats the pipeline, but I did not know whether this was an
actual command output or only a conceptual diagram.
Last clear idea:
The public book said the project teaches tiny ML as typed transformations.
What you expected:
I expected one sentence telling me which local command prints the same path.
What happened instead:
I could read the diagram, but I did not know how to connect it to a runnable
example.
What would have helped:
Add `cargo run --example 01_token_sequence` immediately after the first
pipeline diagram.
```

Why this is useful: it comes from a public-page review, names the exact page
and evidence signal, and asks for one small bridge from page to command.

## Source-Role Report Example

Use this shape when the confusing point is about what a source can prove.

```text
Perspective: ML engineer or learner
Friction lens: source or reference connection
Chapter or file: book/src/references.md and book/src/03-ml-pipeline.md
Command or page tried: public book path -> References -> How To Read Source Roles
Evidence signal: official documentation row in the source-role table
Optional source-role check:
Source role: official documentation.
Owned boundary: API shape and terminology for a framework loss function.
Claim this source supports: how the framework names logits, targets, and reduction behavior.
Claim this source does not support: that this repository implements the full framework loss operator.
Local Rust file, type, example, or chapter section: src/ml.rs CrossEntropy and book/src/03-ml-pipeline.md source-backed precision table.
First unclear sentence, output line, table row, code block, or exercise:
The chapter cites framework documentation near the tiny Rust loss, but I was
not sure whether the citation supported the framework API shape or the local
Rust implementation.
Last clear idea:
Repository code and tests own the local teaching claim.
What you expected:
I expected the source note to say which claim belongs to the framework docs and
which claim belongs to the Rust example.
What happened instead:
I nearly treated the external documentation as proof that the tiny Rust example
matches every production framework behavior.
What would have helped:
Add one sentence: "The framework documentation supports the API vocabulary; the
local Rust code supports only the smaller teaching implementation."
```

Why this is useful: it does not ask for more citations in general. It names the
source role, the owned boundary, the unsupported overclaim, and the exact local
chapter/code area where a reader needs a clearer citation boundary.

## Quick Check Before Submitting

Before opening the issue, check that your report answers these questions:

- What did you personally read, run, or attempt?
- What exact signal made the issue visible?
- What was the last clear idea?
- Where did the path first stop working?
- What is the smallest change that would help the next reader?
- If this is about a source or citation, what does the source support and what
  does it not support?

If the report only says "this is good," "this is confusing," or "add more
theory," keep reading until you can name one exact blocked step.
