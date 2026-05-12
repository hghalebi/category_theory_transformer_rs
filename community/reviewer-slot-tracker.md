# Reviewer Slot Tracker

Use this tracker during a reader-review sprint. It records review slots, not
people. Keep names, email addresses, private messages, and personal notes out of
this public file.

The completion gate needs five accepted direct reader reports:

```text
1 Rust engineer
1 ML engineer
1 category-theory reader
1 technical educator
1 beginner-adjacent reader
```

Use:

- `community/reviewer-outreach.md` for copy-and-paste asks,
- `community/reader-review-packet.md` for the short reviewer path,
- the `Reviewer Context Briefs` section in `community/reader-review-packet.md`
  for the exact path assigned to each reviewer context,
- the `Context-Specific Report Links` section in
  `community/reader-review-packet.md` for issue links with the title,
  location, and command or file path already filled,
- `community/reader-feedback-triage.md` for accepting or clarifying reports,
- `community/reader-feedback-inbox.md` for accepted direct evidence,
- [reader-confusion issue form](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=reader-confusion.yml),
- [reader review sprint issue](https://github.com/hghalebi/category_theory_transformer_rs/issues/6).

## Slot Statuses

| Status | Meaning |
| --- | --- |
| `open` | no usable report has arrived yet |
| `invited` | an ask was sent, but no report has arrived |
| `received` | a report arrived and needs triage |
| `needs clarification` | the report is real but missing required evidence |
| `accepted` | the report is recorded in `community/reader-feedback-inbox.md` |
| `rewritten` | an accepted report has driven a validated rewrite |

Do not count `invited` as evidence. A slot closes only when the inbox contains
an accepted report for that reviewer context.

## Current Slots

| Reviewer context | Current status | Ask to send | Review path | Accepted report target | Next action |
| --- | --- | --- | --- | --- | --- |
| Rust engineer | `open` | Rust Reviewer Ask | Rust engineer brief plus first-run commands | first unclear type, trait, constructor, error, or example | send the Rust ask, point to the Rust engineer brief, and use the Rust engineer report link |
| ML engineer | `open` | ML Reviewer Ask | ML engineer brief plus examples `03` and `06` | first unclear logits, probability, loss, update, attention role, or state transition | send the ML ask, point to the ML engineer brief, and use the ML engineer report link |
| Category-theory reader | `open` | Category-Theory Reviewer Ask | Category-theory reader brief plus Morphism, Structure, Seven Sketches, or Roadmap | first overclaim, early term, missing law, product-input confusion, endomorphism confusion, or endofunctor warning | send the category-theory ask, point to the category-theory reader brief, and use the category-theory reader report link |
| Technical educator | `open` | Educator Ask | Technical educator brief plus README, start guide, review packet, Welcome, Course Map, Exercises | first missing next action, practice signal, or feedback cue | send the educator ask, point to the technical educator brief, and use the technical educator report link |
| Beginner-adjacent reader | `open` | Short Public Ask | Beginner-adjacent reader brief plus first-run or public-book path | first sentence, command, output, or term that becomes too compressed | send the short public ask, point to the beginner-adjacent reader brief, and use the beginner-adjacent reader report link |

## Intake Procedure

When a report arrives:

1. Run `scripts/reader-feedback-status.sh --live`.
2. If the report came through GitHub, run
   `python3 scripts/collect-reader-feedback-issues.py`.
3. Triage the report with `community/reader-feedback-triage.md`.
4. Record accepted reports in `community/reader-feedback-inbox.md`.
5. Update this tracker by changing only the slot status and next action.

## Completion Check

The sprint is not complete until this command passes:

```bash
python3 scripts/check-reader-feedback-inbox.py --require-sprint-complete
```

If the command still reports missing contexts, leave the corresponding slot open
or in its real current state.
