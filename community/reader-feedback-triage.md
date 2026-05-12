# Reader Feedback Triage

Use this protocol when a reader opens a feedback issue. The goal is to turn
confusion into a source-backed chapter rewrite, not to collect opinions.

## Inputs

Start with one direct reader report from:

```text
.github/ISSUE_TEMPLATE/reader-confusion.yml
community/reader-feedback-inbox.md
community/reader-review-guide.md
community/reader-review-packet.md
```

To convert GitHub issue-form submissions into inbox drafts, run:

```bash
python3 scripts/collect-reader-feedback-issues.py
```

The collector redacts email-address patterns in issue titles and rendered
draft fields. Do not paste names, email addresses, private messages, or
personal notes into the public inbox.

To inspect the current inbox, strict completion gate, GitHub surface, and issue
intake together, run:

```bash
scripts/reader-feedback-status.sh --live
```

Proxy sources such as `community/external-feedback-synthesis.md` can help name
common friction patterns, but they do not replace direct project-specific
reports.

## Triage Steps

### 1. Check For Concrete Evidence

A useful report should name:

```text
review path
reviewer context
command or file tried
friction lens
confusing text or output
what made sense before that point
where the mental model broke
what the reader expected next
smallest useful fix
```

If the issue does not include a concrete location or command, ask for that
before rewriting a chapter.

## Report Quality Examples

These examples are not reports and do not count as direct reader evidence.

### Usable direct report

This is an example, not direct evidence.

```text
Review path: Thirty-minute reader review
Reviewer context: ML engineer
Location: book/src/03-ml-pipeline.md / Prediction
Command or file tried: cargo run --example 01_token_sequence
Friction lens: ML intuition
Confusing text or output: I saw `Logits` become `Distribution`, but did not
know what changed mathematically.
Last clear idea: Tokens becoming training pairs was clear.
Where the mental model broke: Raw scores and probabilities looked like the same
thing with different names.
Expected next step: Show one tiny raw-score vector and its normalized
probabilities before naming softmax.
Smallest useful fix: Add one numeric checkpoint before the typed pipeline.
```

Why this is usable: it is location-bound, command-backed, names the last clear
idea, and gives a smallest useful fix.

### Insufficient report

This is an example, not direct evidence.

```text
The ML chapter is confusing. Please make it better.
```

Why this is not usable yet: it does not name a chapter section, command,
confusing sentence, last clear idea, mental-model break, or smallest useful
fix. Ask for one concrete location before rewriting.

### 2. Classify The Friction

Use the issue's friction lens to route the work.

| Lens | First artifact to inspect | Likely repair |
| --- | --- | --- |
| Entry path | `README.md`, `START_HERE.md`, `book/src/welcome.md` | clearer first command or reading path |
| Rust syntax | chapter source snapshot and matching `src/*.rs` file | smaller syntax explanation or boundary note |
| ML intuition | ML chapter, roadmap, or exercise | concrete numeric trace before abstraction |
| Category-theory precision | glossary, chapter term, law table | narrower claim or stronger code anchor |
| Diagram clarity | nearby diagram and following paragraph | label, remove, or explain one takeaway |
| Exercise readiness | `book/src/exercises.md`, `exercises/*/README.md` | command, expected output, or failure signal |
| Reference needed | `book/src/references.md` | authoritative source and chapter reference |

### 3. Choose A Rewrite Action

Pick one of four actions.

| Action | Use when | Required output |
| --- | --- | --- |
| Fix now | the report names one clear blocked sentence, command, or diagram | one focused patch plus validation |
| Batch theme | multiple reports point to the same chapter or concept | feedback-wall theme plus planned rewrite pass |
| Ask clarification | the report lacks location, command, or concrete friction | one specific follow-up question |
| Close out of scope | the request asks for production-scale scope outside the tiny teaching contract | polite explanation and link to the roadmap |

Do not make a large rewrite from a vague report.

### 4. Rewrite From Evidence

For a real rewrite, use this sequence:

```text
reader report
-> affected chapter section
-> repository code or example
-> chapter references
-> rewrite decision
-> validation
-> issue close-out
```

The rewrite should update only the smallest set of files needed to remove the
blocked learning step. If the change affects an exercise, update
`exercises/ANSWER_KEY.md`. If it changes a term, update `book/src/glossary.md`.
If it adds a claim, update `book/src/references.md`.

### 5. Record The Pass

Every feedback-driven rewrite should add an entry to
`book/CHAPTER_REWRITE_LOG.md` with:

```text
reader issue or report
primary source files
primary references
critique before rewrite
rewrite decision
validation commands
remaining risk
```

If the issue reveals a recurring theme, also update
`community/feedback-wall.md`. If the report is accepted as direct evidence,
record it in `community/reader-feedback-inbox.md` so later rewrite sweeps can
audit the source, triage state, affected chapter, rewrite-log entry, and
validation.

### 6. Validate Before Closing

Run the relevant focused checks first, then the full gate when the public book,
examples, or validation scripts changed.

```bash
python3 scripts/check-prose-style.py
python3 scripts/check-completion-audit.py
python3 scripts/check-rewrite-log.py
python3 scripts/check-chapter-references.py
python3 scripts/check-exercise-alignment.py
python3 scripts/collect-reader-feedback-issues.py --self-test
bash scripts/check-mdbook-coverage.sh
git diff --check -- . ':!target'
bash scripts/check.sh
```

If a full gate cannot run, state exactly what was not verified in the issue
close-out.

## Close-Out Format

Use this shape when responding to the issue:

```text
Thanks. This report identified a concrete break in <chapter/section>.

Changed:
- <file>: <what changed>

Validation:
- <commands run>

Remaining risk:
- <what still needs reader feedback, or "none for this issue">
```

## What Not To Do

Do not:

- treat proxy public feedback as direct reader evidence,
- rewrite a whole chapter when one transition is blocked,
- add unsourced technical claims,
- name private critique lenses in public book text,
- weaken validation to make a rewrite easier,
- close an issue only because the full gate passes.

The useful standard is simple:

```text
one reader confusion report
-> one clearer learning step
-> one validated rewrite
```
