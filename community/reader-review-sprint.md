# Reader Review Sprint

Use this sprint when the project is ready to collect direct reader reports.
The goal is a small set of concrete reports that can drive the next rewrite
pass.

This is not a marketing campaign. It is an editorial evidence-gathering loop.

## Sprint Goal

Collect at least five accepted direct reader reports:

```text
1 Rust engineer
1 ML engineer
1 category-theory reader
1 technical educator
1 beginner-adjacent reader
```

Each accepted report must name:

```text
source
review path
reviewer context
location
command or file tried
friction lens
confusing text or output
last clear idea
where the mental model broke
smallest useful fix
triage action
status
```

Record accepted reports in `community/reader-feedback-inbox.md`.
Triage them with `community/reader-feedback-triage.md`.

## Sprint Inputs

Use these files:

- `community/reviewer-outreach.md`
- `community/reader-review-packet.md`
- `community/reader-review-guide.md`
- `community/reader-feedback-inbox.md`
- `community/reader-feedback-triage.md`
- `community/reviewer-slot-tracker.md`
- [reader-confusion issue form](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=reader-confusion.yml)

Public review surfaces:

- Public book: <https://hghalebi.github.io/category_theory_transformer_rs/>
- GitHub repository: <https://github.com/hghalebi/category_theory_transformer_rs>

Public tracking issue:

- [reader review sprint issue](https://github.com/hghalebi/category_theory_transformer_rs/issues/6)

Before sending an ask, point the reviewer to the matching
`Reviewer Context Briefs` row in `community/reader-review-packet.md`. That row
turns the five required reviewer contexts into one concrete command path and one concrete report target.
Then send the matching context-specific report link from the same packet so
the issue opens with the intended location and command already filled.

## Reviewer Assignments

| Reviewer type | Ask to send | Required path | Accepted report target |
| --- | --- | --- | --- |
| Rust engineer | Rust Reviewer Ask | `cargo run --example 01_token_sequence`, `cargo run --bin category_ml`, one Rust-heavy chapter | first unclear type, trait, constructor, or example |
| ML engineer | ML Reviewer Ask | examples `01`, `03`, `06`, then ML pipeline, training, or roadmap | first unclear logits, probability, loss, update, attention, self-vs-cross attention, state transition, or role separation |
| Category-theory reader | Category-Theory Reviewer Ask | examples `02`, `04`, `05`, the self-vs-cross boundary, or the category-shape diagnostic in the roadmap | first overclaim, early term, missing law, product-input/endomorphism confusion, self-vs-cross boundary confusion, or weak Rust anchor |
| Technical educator | Educator Ask | README, start guide, review packet, Welcome, Course Map, Exercises | first missing next action or practice signal |
| Beginner-adjacent reader | Short Public Ask | first-run review path | first place where the path becomes too compressed |

If a reviewer will not clone the repository, assign the public book path from
`community/reader-review-packet.md`. That report can still count when it names
the public page or section, the last clear idea, and the exact unclear step.

## Slot Tracker

Use `community/reviewer-slot-tracker.md` to keep the five reviewer contexts
visible during the sprint.

The tracker is public and should contain only slot status:

```text
open
invited
received
needs clarification
accepted
rewritten
```

Do not add names, emails, private notes, or messages. A slot is evidence only
after an accepted direct report is recorded in
`community/reader-feedback-inbox.md`.

## Seven-Day Sprint

### Day 1: Invite

Send the relevant asks from `community/reviewer-outreach.md`.

Do not ask for a full review. Ask for one concrete friction report.

Tell each reviewer which row of `Reviewer Context Briefs` to use. The target is
one location-bound report from their context, not a general endorsement.
Use the matching context-specific report link so the reviewer does not have to
retype the path.

If the reviewer is comfortable with category theory, ask them to test one
boundary classification from the Transformer Roadmap category-shape diagnostic.
The target report is one unclear distinction, not a review of the whole
chapter.

If the reviewer is comfortable with attention mechanisms, ask them to test the
self-attention versus cross-attention boundary in the Transformer Roadmap. The
target report is one unclear distinction between shared Q/K/V source,
parallel `HiddenSequence -> QuerySequence`, `HiddenSequence -> KeySequence`,
and `HiddenSequence -> ValueSequence` projections, the false
`QuerySequence -> KeySequence -> ValueSequence` reading, separate query and
key-value sources, target length `L`, source length `S`, or product-input
morphism before endomorphism.

The same ask should still name separate query and key-value sources and
product-input morphism before endomorphism when either phrase is the point that
breaks.

Update `community/reviewer-slot-tracker.md` from `open` to `invited` only if an
ask has actually been sent.

### Day 2: Remind

Send one short reminder with the review packet and issue form.

Do not change the scope.

### Day 3: Intake

Check direct reports from:

```bash
gh issue list --repo hghalebi/category_theory_transformer_rs \
  --state all \
  --label "reader confusion" \
  --limit 50 \
  --json number,title,state,labels,createdAt,updatedAt,url
```

Then render issue-form submissions into inbox drafts:

```bash
python3 scripts/collect-reader-feedback-issues.py
```

Also check workshop notes or direct review notes if they name the exact
chapter, command, and confusing step.

To verify that the GitHub labels and public sprint issue still exist, run:

```bash
scripts/check-github-feedback-surface.sh
```

To see the local inbox state, strict completion gate, GitHub feedback surface,
and issue intake together, run:

```bash
scripts/reader-feedback-status.sh --live
```

Update `community/reviewer-slot-tracker.md` from `invited` to `received` only
when a direct report exists.

### Day 4: Accept Or Clarify

For each report:

1. If it names location, command or file tried, friction lens, and mental-model
   break, record it in `community/reader-feedback-inbox.md`.
2. If it lacks one of those fields, mark it `needs clarification` and ask one
   specific follow-up.
3. If it is vague praise, do not count it as evidence.

Keep `community/reviewer-slot-tracker.md` aligned with this decision.

### Day 5: Triage

Use `community/reader-feedback-triage.md`.

Choose one action:

```text
fix now
batched theme
needs clarification
closed out of scope
```

Update `community/feedback-wall.md` only when repeated reports point to the
same theme.

### Day 6: Rewrite

Pick the smallest blocked learning step with the clearest evidence.

Before editing, inspect:

```text
affected chapter section
matching Rust file or example
book/src/references.md
book/src/glossary.md
book/src/exercises.md
exercises/ANSWER_KEY.md
```

Rewrite only the files needed to remove the blocked step.

### Day 7: Validate And Close

Run focused checks first:

```bash
python3 scripts/check-prose-style.py
python3 scripts/check-completion-audit.py
python3 scripts/check-reader-feedback-loop.py
python3 scripts/check-reader-feedback-inbox.py
python3 scripts/check-reader-feedback-inbox.py --self-test
python3 scripts/collect-reader-feedback-issues.py --self-test
python3 scripts/check-rewrite-log.py
python3 scripts/check-chapter-references.py
python3 scripts/check-exercise-alignment.py
scripts/check-github-feedback-surface.sh
scripts/reader-feedback-status.sh
git diff --check -- . ':!target'
```

Before claiming the sprint closed the external-reader gap, run:

```bash
python3 scripts/check-reader-feedback-inbox.py --require-sprint-complete
```

When public teaching material changed, run:

```bash
bash scripts/check.sh
```

Record the rewrite in `book/CHAPTER_REWRITE_LOG.md`.

## Acceptance Criteria

The sprint has produced usable evidence only when:

- at least five accepted reports are recorded in
  `community/reader-feedback-inbox.md`,
- those reports cover a Rust engineer, ML engineer, category-theory reader,
  technical educator, and beginner-adjacent reader,
- each report has a source, reviewer context, location, command or file tried,
  friction lens, triage action, and status,
- at least one report is converted into a source-backed rewrite,
- the rewrite is recorded in `book/CHAPTER_REWRITE_LOG.md`,
- `python3 scripts/check-reader-feedback-inbox.py --require-sprint-complete`
  passes,
- focused validation passes,
- the full gate passes if public teaching material changed.

## Non-Evidence

Do not count these as accepted direct reports:

- stars,
- reposts,
- vague praise,
- broad opinions,
- sent outreach messages,
- proxy public discussions,
- AI-generated critique not tied to a human reader running or reading the
  project.

Those signals may inform outreach, but they do not close the direct-feedback
gap in `book/TEXTBOOK_COMPLETION_AUDIT.md`.

## Sprint Close-Out

At the end of the sprint, update:

- `community/reader-feedback-inbox.md`
- `community/reviewer-slot-tracker.md`
- `community/feedback-wall.md`
- `book/CHAPTER_REWRITE_LOG.md`
- `book/TEXTBOOK_COMPLETION_AUDIT.md`

If fewer than five reports arrive, keep the audit open and record what was
missing.
