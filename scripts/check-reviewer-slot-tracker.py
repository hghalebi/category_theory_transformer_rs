#!/usr/bin/env python3
"""Validate reviewer slot status against direct-reader inbox evidence."""

from __future__ import annotations

import importlib.util
import re
import sys
from pathlib import Path

sys.dont_write_bytecode = True


ROOT = Path(__file__).resolve().parents[1]
TRACKER = ROOT / "community" / "reviewer-slot-tracker.md"
INBOX = ROOT / "community" / "reader-feedback-inbox.md"
INBOX_CHECKER = ROOT / "scripts" / "check-reader-feedback-inbox.py"

REQUIRED_CONTEXTS = {
    "rust engineer": "Rust engineer",
    "ml engineer": "ML engineer",
    "category-theory reader": "Category-theory reader",
    "technical educator": "Technical educator",
    "beginner-adjacent reader": "Beginner-adjacent reader",
}

ALLOWED_SLOT_STATUSES = {
    "open",
    "invited",
    "received",
    "needs clarification",
    "accepted",
    "rewritten",
}

EVIDENCE_STATUSES = {
    "accepted",
    "rewritten",
}


def load_inbox_checker():
    spec = importlib.util.spec_from_file_location("reader_feedback_inbox", INBOX_CHECKER)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"could not load {INBOX_CHECKER}")

    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


def normalize_status(value: str) -> str:
    return value.strip().strip("`").lower()


def normalize_context(value: str) -> str:
    return value.strip().lower()


def tracker_rows(text: str) -> dict[str, str]:
    rows: dict[str, str] = {}
    in_slots = False

    for line in text.splitlines():
        if line.strip() == "## Current Slots":
            in_slots = True
            continue

        if in_slots and line.startswith("## "):
            break

        stripped = line.strip()
        if not in_slots or not stripped.startswith("|") or stripped.startswith("| ---"):
            continue

        cells = [cell.strip() for cell in stripped.strip("|").split("|")]
        if not cells or cells[0] == "Reviewer context":
            continue

        if len(cells) < 2:
            continue

        rows[normalize_context(cells[0])] = normalize_status(cells[1])

    return rows


def context_marker(raw_context: str) -> str | None:
    normalized = raw_context.lower()

    for marker in REQUIRED_CONTEXTS:
        if marker in normalized:
            return marker

    return None


def inbox_contexts() -> tuple[list[str], dict[str, bool], dict[str, bool]]:
    checker = load_inbox_checker()
    text = INBOX.read_text(encoding="utf-8")
    issues, validations = checker.validate_inbox_text(
        text,
        require_sprint_complete=False,
    )

    accepted, rewritten = evidence_contexts(validations)

    return issues, accepted, rewritten


def evidence_contexts(validations) -> tuple[dict[str, bool], dict[str, bool]]:
    accepted: dict[str, bool] = {}
    rewritten: dict[str, bool] = {}

    for validation in validations:
        marker = context_marker(validation.values.get("Reviewer context", ""))
        if marker is None:
            continue

        if validation.is_accepted:
            accepted[marker] = True

        if validation.normalized_status == "rewritten":
            rewritten[marker] = True

    return accepted, rewritten


def check_rows_against_evidence(
    rows: dict[str, str],
    accepted_contexts: dict[str, bool],
    rewritten_contexts: dict[str, bool],
    issue_prefix: str,
) -> list[str]:
    issues: list[str] = []

    for marker, label in REQUIRED_CONTEXTS.items():
        status = rows.get(marker)

        if status is None:
            issues.append(f"{issue_prefix}: missing reviewer slot for {label}")
            continue

        if status not in ALLOWED_SLOT_STATUSES:
            issues.append(
                f"{issue_prefix}: slot for {label} has invalid status {status!r}; "
                f"expected one of {sorted(ALLOWED_SLOT_STATUSES)}"
            )
            continue

        has_accepted = accepted_contexts.get(marker, False)
        has_rewritten = rewritten_contexts.get(marker, False)

        if status in EVIDENCE_STATUSES and not has_accepted:
            issues.append(
                f"{issue_prefix}: slot for {label} is {status!r}, but inbox has no "
                "accepted report for that context"
            )

        if status == "rewritten" and not has_rewritten:
            issues.append(
                f"{issue_prefix}: slot for {label} is 'rewritten', but inbox has no "
                "rewritten report for that context"
            )

        if has_rewritten and status != "rewritten":
            issues.append(
                f"{issue_prefix}: slot for {label} should be 'rewritten' because the "
                "inbox has a rewritten report for that context"
            )
        elif has_accepted and status not in EVIDENCE_STATUSES:
            issues.append(
                f"{issue_prefix}: slot for {label} should be 'accepted' or 'rewritten' "
                "because the inbox has accepted evidence for that context"
            )

    return issues


def check_tracker_consistency(text: str) -> list[str]:
    issues: list[str] = []
    rows = tracker_rows(text)
    inbox_issues, accepted_contexts, rewritten_contexts = inbox_contexts()

    for issue in inbox_issues:
        issues.append(f"{INBOX}: inbox evidence must be valid before tracker consistency: {issue}")

    issues.extend(
        check_rows_against_evidence(
            rows=rows,
            accepted_contexts=accepted_contexts,
            rewritten_contexts=rewritten_contexts,
            issue_prefix=str(TRACKER),
        )
    )

    return issues


def fixture_report(
    date: str,
    title: str,
    reviewer_context: str,
    status: str,
) -> str:
    return f"""### Report {date}: {title}

Source: GitHub issue #77
GitHub issue URL: https://github.com/hghalebi/category_theory_transformer_rs/issues/77
Review path: Thirty-minute reader review
Reviewer context: {reviewer_context}
Location: book/src/03-ml-pipeline.md
Command or file tried: cargo run --example 01_token_sequence
Friction lens: ML intuition
Confusing text or output: The transition from logits to probabilities needs one more bridge sentence.
Last clear idea: TokenSequence to TrainingPairs was clear.
Where the mental model broke: The reader did not know why raw scores became a normalized distribution.
Expected next step: Show softmax as raw-score normalization before naming it.
Smallest useful fix: Add one sentence and a numeric checkpoint before the softmax call.
Triage action: fix now
Affected chapter section: Tiny ML Pipeline / Prediction
Repository code or example: src/ml.rs and examples/02_morphism_composition.rs
References checked: book/src/references.md#tiny-ml-pipeline
Rewrite log entry: book/CHAPTER_REWRITE_LOG.md#reader-feedback-fixture
Validation: python3 scripts/check-reviewer-slot-tracker.py --self-test
Status: {status}
"""


def run_self_test() -> int:
    complete_tracker = """# Reviewer Slot Tracker

## Current Slots

| Reviewer context | Current status | Ask to send | Review path | Accepted report target | Next action |
| --- | --- | --- | --- | --- | --- |
| Rust engineer | `rewritten` | Rust Reviewer Ask | path | target | done |
| ML engineer | `accepted` | ML Reviewer Ask | path | target | done |
| Category-theory reader | `accepted` | Category-Theory Reviewer Ask | path | target | done |
| Technical educator | `accepted` | Educator Ask | path | target | done |
| Beginner-adjacent reader | `accepted` | Short Public Ask | path | target | done |
"""
    complete_inbox = "# Fixture\n\n" + "\n".join(
        [
            fixture_report("2026-05-12", "rust", "Rust engineer", "rewritten"),
            fixture_report("2026-05-13", "ml", "ML engineer", "fix now"),
            fixture_report(
                "2026-05-14",
                "category",
                "category-theory reader",
                "fix now",
            ),
            fixture_report(
                "2026-05-15",
                "educator",
                "technical educator",
                "fix now",
            ),
            fixture_report(
                "2026-05-16",
                "beginner",
                "beginner-adjacent reader",
                "fix now",
            ),
        ]
    )

    checker = load_inbox_checker()
    complete_issues, validations = checker.validate_inbox_text(
        complete_inbox,
        require_sprint_complete=False,
    )
    if complete_issues:
        print("Reviewer slot tracker self-test failed: fixture inbox invalid.")
        for issue in complete_issues:
            print(f"- {issue}")
        return 1

    accepted, rewritten = evidence_contexts(validations)
    rows = tracker_rows(complete_tracker)
    if len(rows) != len(REQUIRED_CONTEXTS):
        print("Reviewer slot tracker self-test failed: fixture tracker rows missing.")
        return 1

    complete_consistency_issues = check_rows_against_evidence(
        rows=rows,
        accepted_contexts=accepted,
        rewritten_contexts=rewritten,
        issue_prefix="fixture tracker",
    )
    if complete_consistency_issues:
        print("Reviewer slot tracker self-test failed: complete fixture should pass.")
        for issue in complete_consistency_issues:
            print(f"- {issue}")
        return 1

    bad_tracker = complete_tracker.replace("| ML engineer | `accepted` |", "| ML engineer | `open` |")
    bad_rows = tracker_rows(bad_tracker)
    bad_status_issues = check_rows_against_evidence(
        rows=bad_rows,
        accepted_contexts=accepted,
        rewritten_contexts=rewritten,
        issue_prefix="fixture tracker",
    )
    if not bad_status_issues:
        print("Reviewer slot tracker self-test failed: accepted evidence with open slot should fail.")
        return 1

    missing_accepted = dict(accepted)
    missing_accepted.pop("technical educator", None)
    missing_evidence_issues = check_rows_against_evidence(
        rows=rows,
        accepted_contexts=missing_accepted,
        rewritten_contexts=rewritten,
        issue_prefix="fixture tracker",
    )
    if not missing_evidence_issues:
        print("Reviewer slot tracker self-test failed: accepted slot without evidence should fail.")
        return 1

    missing_rewritten = dict(rewritten)
    missing_rewritten.pop("rust engineer", None)
    missing_rewrite_issues = check_rows_against_evidence(
        rows=rows,
        accepted_contexts=accepted,
        rewritten_contexts=missing_rewritten,
        issue_prefix="fixture tracker",
    )
    if not missing_rewrite_issues:
        print("Reviewer slot tracker self-test failed: rewritten slot without rewritten evidence should fail.")
        return 1

    stale_rewrite_tracker = complete_tracker.replace("| Rust engineer | `rewritten` |", "| Rust engineer | `accepted` |")
    stale_rewrite_rows = tracker_rows(stale_rewrite_tracker)
    stale_rewrite_issues = check_rows_against_evidence(
        rows=stale_rewrite_rows,
        accepted_contexts=accepted,
        rewritten_contexts=rewritten,
        issue_prefix="fixture tracker",
    )
    if not stale_rewrite_issues:
        print("Reviewer slot tracker self-test failed: rewritten evidence should require rewritten slot status.")
        return 1

    print("Reviewer slot tracker self-test passed.")
    return 0


def main() -> int:
    if "--self-test" in sys.argv:
        return run_self_test()

    text = TRACKER.read_text(encoding="utf-8")
    issues = check_tracker_consistency(text)

    if issues:
        print("Reviewer slot tracker check failed:")
        for issue in issues:
            print(f"- {issue}")
        return 1

    print("Reviewer slot tracker check passed.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
