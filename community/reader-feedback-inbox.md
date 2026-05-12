# Reader Feedback Inbox

This inbox records direct project-specific reader reports after they arrive.
Use it for evidence that came from someone reading this book, running this
repository, or reviewing one chapter with the review packet.

This file is not a place for proxy feedback. Public discussions, other
courses, and general learner-friction signals belong in
`community/external-feedback-synthesis.md` until a reader reports friction on
this project.

## Current Status

No direct project-specific reader reports have been recorded in this file yet.

Latest live check used by the completion audit:

```bash
gh issue list --repo hghalebi/category_theory_transformer_rs \
  --state all \
  --label "reader confusion" \
  --limit 50 \
  --json number,title,state,labels,createdAt,updatedAt,url
```

Observed result:

```text
[]
```

Interpretation: the review loop is ready, but the next reader-driven rewrite
still needs a real report.

## Intake Sources

Direct reports should come from one of these paths:

- [reader-confusion issue form](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=reader-confusion.yml)
- `community/reviewer-outreach.md`
- `community/reader-review-sprint.md`
- `community/reader-review-guide.md`
- `community/reader-review-packet.md`
- `community/reviewer-slot-tracker.md`
- a workshop or reading-session report that names the exact chapter, command,
  and confusing step

Triage every report with `community/reader-feedback-triage.md`.

To render inbox-ready drafts from GitHub issue-form submissions, run:

```bash
python3 scripts/collect-reader-feedback-issues.py
```

This command does not count as evidence by itself. It only turns real
`reader confusion` issues into report drafts that still need triage.

To inspect the whole feedback status, run:

```bash
scripts/reader-feedback-status.sh --live
```

## Entry Format

Use this format for each accepted direct report.

```text
### Report YYYY-MM-DD: <short title>

Source:
GitHub issue URL:
Review path:
Reviewer context:
Location:
Command or file tried:
Friction lens:
Confusing text or output:
Last clear idea:
Where the mental model broke:
Expected next step:
Smallest useful fix:

Triage action:
Affected chapter section:
Repository code or example:
References checked:
Rewrite log entry:
Validation:
Status:
```

## Accepted Report Standard

The template above becomes accepted direct-reader evidence only when it is
filled with a real source and concrete friction from this project. A usable
entry should be:

- location-bound: it names the chapter, section, file, command, or output;
- command-backed: it says what the reader ran, opened, or reviewed;
- mental-model specific: it separates the last clear idea from the point of
  confusion;
- rewrite-backed: it names the smallest useful fix and later links the rewrite
  log when the change lands;
- privacy-safe: it avoids names, email addresses, private messages, and
  personal notes.

Accepted evidence requires both an accepted `Triage action` and an accepted
`Status`. A report with `Status: new` is still intake, even if the likely
triage action looks clear.

The inbox validator rejects email-address patterns in report headings and
bodies. If a review arrives through a private channel, rewrite the source as a
public-safe description such as `workshop report` or
`maintainer-transcribed review` before recording it here.

GitHub issue drafts rendered by `scripts/collect-reader-feedback-issues.py`
redact email-address patterns in issue titles and form fields before printing
inbox-ready text.

This quality example is not direct evidence:

```text
The examples are confusing.
```

It is too broad to rewrite from. Ask for the exact chapter, command, confusing
text or output, and smallest useful fix before recording it as accepted
evidence.

## Triage States

Use one of these states.

| State | Meaning |
| --- | --- |
| `new` | report captured but not triaged |
| `needs clarification` | report is real but lacks location, command, or concrete friction |
| `fix now` | one focused rewrite is clear |
| `batched theme` | useful but should wait for repeated reports |
| `rewritten` | rewrite has landed and validation is recorded |
| `closed out of scope` | request does not fit the tiny teaching contract |

## Template Only

The block above is a template, not a report.

Do not count this inbox as direct reader evidence until it contains at least one
filled report with a source, reviewer context, location, command or file tried,
friction lens, triage action, and status.
