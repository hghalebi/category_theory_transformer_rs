#!/usr/bin/env python3
"""Validate accepted direct-reader report records."""

from __future__ import annotations

import argparse
import re
import sys
from dataclasses import dataclass
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
INBOX = ROOT / "community" / "reader-feedback-inbox.md"

NO_DIRECT_REPORTS_MARKER = (
    "No direct project-specific reader reports have been recorded in this file yet."
)

REQUIRED_REPORT_FIELDS = [
    "Source",
    "GitHub issue URL",
    "Review path",
    "Reviewer context",
    "Location",
    "Command or file tried",
    "Friction lens",
    "Confusing text or output",
    "Last clear idea",
    "Where the mental model broke",
    "Expected next step",
    "Smallest useful fix",
    "Triage action",
    "Affected chapter section",
    "Repository code or example",
    "References checked",
    "Rewrite log entry",
    "Validation",
    "Status",
]

ALLOWED_STATUSES = {
    "new",
    "needs clarification",
    "fix now",
    "batched theme",
    "rewritten",
    "closed out of scope",
}

ALLOWED_TRIAGE_ACTIONS = {
    "fix now",
    "batched theme",
    "needs clarification",
    "closed out of scope",
}

REPORT_HEADING = re.compile(r"^### Report \d{4}-\d{2}-\d{2}: .+$")
EMAIL_ADDRESS_PATTERN = re.compile(
    r"\b[A-Z0-9._%+-]+@[A-Z0-9.-]+\.[A-Z]{2,}\b",
    re.IGNORECASE,
)

REQUIRED_REVIEWER_CONTEXTS = {
    "rust engineer": "Rust engineer",
    "ml engineer": "ML engineer",
    "category-theory reader": "category-theory reader",
    "technical educator": "technical educator",
    "beginner-adjacent reader": "beginner-adjacent reader",
}

ACCEPTED_STATUSES = {
    "fix now",
    "batched theme",
    "rewritten",
}

ACCEPTED_TRIAGE_ACTIONS = {
    "fix now",
    "batched theme",
}


@dataclass(frozen=True)
class ReportSection:
    heading: str
    body: str


@dataclass(frozen=True)
class ReportValidation:
    heading: str
    values: dict[str, str]
    issues: list[str]

    @property
    def normalized_status(self) -> str:
        return self.values.get("Status", "").strip().lower()

    @property
    def normalized_triage_action(self) -> str:
        return self.values.get("Triage action", "").strip().lower()

    @property
    def is_accepted(self) -> bool:
        return (
            not self.issues
            and self.normalized_status in ACCEPTED_STATUSES
            and self.normalized_triage_action in ACCEPTED_TRIAGE_ACTIONS
        )


def strip_fenced_blocks(text: str) -> str:
    lines: list[str] = []
    in_fence = False

    for line in text.splitlines():
        if line.startswith("```"):
            in_fence = not in_fence
            continue

        if not in_fence:
            lines.append(line)

    return "\n".join(lines)


def report_sections(text: str) -> list[ReportSection]:
    unfenced = strip_fenced_blocks(text)
    matches = list(re.finditer(r"^### Report .+$", unfenced, re.MULTILINE))
    reports: list[ReportSection] = []

    for index, match in enumerate(matches):
        next_match = matches[index + 1].start() if index + 1 < len(matches) else len(unfenced)
        section = unfenced[match.start() : next_match]
        lines = section.splitlines()
        heading = lines[0]
        body = "\n".join(lines[1:])
        reports.append(ReportSection(heading=heading, body=body))

    return reports


def field_values(body: str) -> dict[str, str]:
    values: dict[str, str] = {}
    field_names = "|".join(re.escape(field) for field in REQUIRED_REPORT_FIELDS)
    field_pattern = re.compile(rf"^({field_names}):\s*(.*)$", re.MULTILINE)

    matches = list(field_pattern.finditer(body))
    for index, match in enumerate(matches):
        field = match.group(1)
        first_line = match.group(2).strip()
        value_start = match.end()
        value_end = matches[index + 1].start() if index + 1 < len(matches) else len(body)
        continuation = body[value_start:value_end].strip()
        value = "\n".join(part for part in (first_line, continuation) if part).strip()
        values[field] = value

    return values


def is_placeholder(value: str) -> bool:
    normalized = value.strip().lower()
    return not normalized or normalized in {"tbd", "todo", "n/a?"} or "<" in normalized or ">" in normalized


def contains_email_address(value: str) -> bool:
    return EMAIL_ADDRESS_PATTERN.search(value) is not None


def check_no_reports_state(text: str, reports: list[ReportSection]) -> list[str]:
    issues: list[str] = []

    if not reports and NO_DIRECT_REPORTS_MARKER not in text:
        issues.append(
            f"{INBOX}: no report entries found, but missing no-direct-reports marker"
        )

    if reports and NO_DIRECT_REPORTS_MARKER in text:
        issues.append(
            f"{INBOX}: direct reports are present, so remove the no-direct-reports marker"
        )

    return issues


def check_report(report: ReportSection) -> ReportValidation:
    issues: list[str] = []
    values = field_values(report.body)

    if REPORT_HEADING.match(report.heading) is None:
        issues.append(
            f"{INBOX}: report heading should match '### Report YYYY-MM-DD: <short title>': {report.heading!r}"
        )

    for field in REQUIRED_REPORT_FIELDS:
        if field not in values:
            issues.append(f"{INBOX}: {report.heading}: missing field {field!r}")
            continue

        if is_placeholder(values[field]):
            issues.append(f"{INBOX}: {report.heading}: field {field!r} is empty or placeholder")

    if contains_email_address(report.heading) or contains_email_address(report.body):
        issues.append(
            f"{INBOX}: {report.heading}: report heading or body should not include email addresses"
        )

    status = values.get("Status", "").strip().lower()
    if status and status not in ALLOWED_STATUSES:
        issues.append(
            f"{INBOX}: {report.heading}: Status should be one of {sorted(ALLOWED_STATUSES)}"
        )

    triage_action = values.get("Triage action", "").strip().lower()
    if triage_action and triage_action not in ALLOWED_TRIAGE_ACTIONS:
        issues.append(
            f"{INBOX}: {report.heading}: Triage action should be one of {sorted(ALLOWED_TRIAGE_ACTIONS)}"
        )

    if (
        status in {"new", "needs clarification", "closed out of scope"}
        and triage_action in ACCEPTED_TRIAGE_ACTIONS
    ):
        issues.append(
            f"{INBOX}: {report.heading}: accepted triage actions require an accepted Status"
        )

    if status in ACCEPTED_STATUSES and triage_action not in ACCEPTED_TRIAGE_ACTIONS:
        issues.append(
            f"{INBOX}: {report.heading}: accepted statuses require an accepted Triage action"
        )

    if status == "rewritten" and not values.get("Rewrite log entry", "").startswith("book/CHAPTER_REWRITE_LOG.md"):
        issues.append(
            f"{INBOX}: {report.heading}: rewritten reports should link to book/CHAPTER_REWRITE_LOG.md"
        )

    return ReportValidation(heading=report.heading, values=values, issues=issues)


def reviewer_context_coverage(validations: list[ReportValidation]) -> set[str]:
    covered: set[str] = set()

    for validation in validations:
        if not validation.is_accepted:
            continue

        context = validation.values.get("Reviewer context", "").lower()
        for marker in REQUIRED_REVIEWER_CONTEXTS:
            if marker in context:
                covered.add(marker)

    return covered


def check_sprint_completion(validations: list[ReportValidation]) -> list[str]:
    issues: list[str] = []
    accepted = [validation for validation in validations if validation.is_accepted]
    rewritten = [
        validation
        for validation in validations
        if validation.normalized_status == "rewritten"
    ]
    covered_contexts = reviewer_context_coverage(validations)

    if len(accepted) < 5:
        issues.append(
            f"{INBOX}: completion requires at least 5 accepted direct-reader reports; found {len(accepted)}"
        )

    for marker, label in REQUIRED_REVIEWER_CONTEXTS.items():
        if marker not in covered_contexts:
            issues.append(f"{INBOX}: completion requires an accepted report from {label}")

    if not rewritten:
        issues.append(
            f"{INBOX}: completion requires at least one accepted report with Status: rewritten"
        )

    return issues


def print_status(validations: list[ReportValidation]) -> None:
    accepted = [validation for validation in validations if validation.is_accepted]
    rewritten = [
        validation
        for validation in validations
        if validation.normalized_status == "rewritten"
    ]
    covered_contexts = reviewer_context_coverage(validations)
    missing_contexts = [
        label
        for marker, label in REQUIRED_REVIEWER_CONTEXTS.items()
        if marker not in covered_contexts
    ]

    print(
        "Reader feedback sprint status: "
        f"{len(accepted)}/5 accepted reports, "
        f"{len(rewritten)} rewritten report(s)."
    )

    if missing_contexts:
        print("Missing accepted reviewer contexts:")
        for label in missing_contexts:
            print(f"- {label}")


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Validate direct-reader feedback inbox records."
    )
    parser.add_argument(
        "--require-sprint-complete",
        action="store_true",
        help=(
            "Fail unless the inbox contains the direct-reader evidence required "
            "before claiming the textbook goal is complete."
        ),
    )
    parser.add_argument(
        "--self-test",
        action="store_true",
        help="Run offline validator fixtures without reading the project inbox.",
    )
    return parser.parse_args()


def validate_inbox_text(text: str, require_sprint_complete: bool) -> tuple[list[str], list[ReportValidation]]:
    reports = report_sections(text)
    issues: list[str] = []
    validations: list[ReportValidation] = []

    issues.extend(check_no_reports_state(text, reports))

    for report in reports:
        validation = check_report(report)
        validations.append(validation)
        issues.extend(validation.issues)

    if require_sprint_complete:
        issues.extend(check_sprint_completion(validations))

    return issues, validations


def fixture_report(
    date: str,
    title: str,
    issue_number: int,
    reviewer_context: str,
    status: str,
) -> str:
    return f"""### Report {date}: {title}

Source: GitHub issue #{issue_number}
GitHub issue URL: https://github.com/hghalebi/category_theory_transformer_rs/issues/{issue_number}
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
Validation: python3 scripts/check-reader-feedback-inbox.py --self-test
Status: {status}
"""


def complete_fixture_text() -> str:
    contexts = [
        "Rust engineer",
        "ML engineer",
        "category-theory reader",
        "technical educator",
        "beginner-adjacent reader",
    ]
    reports = [
        fixture_report(
            date=f"2026-05-{12 + index:02d}",
            title=f"fixture report {index + 1}",
            issue_number=40 + index,
            reviewer_context=context,
            status="rewritten" if index == 0 else "fix now",
        )
        for index, context in enumerate(contexts)
    ]
    return "# Reader Feedback Inbox Fixture\n\n" + "\n".join(reports)


def incomplete_fixture_text() -> str:
    return "# Reader Feedback Inbox Fixture\n\n" + fixture_report(
        date="2026-05-12",
        title="single incomplete sprint fixture",
        issue_number=90,
        reviewer_context="ML engineer",
        status="fix now",
    )


def private_contact_fixture_text() -> str:
    report = fixture_report(
        date="2026-05-12",
        title="private contact fixture from reader@example.com",
        issue_number=91,
        reviewer_context="ML engineer",
        status="fix now",
    ).replace(
        "Source: GitHub issue #91",
        "Source: GitHub issue #91 from reader@example.com",
    )
    return "# Reader Feedback Inbox Fixture\n\n" + report


def inconsistent_acceptance_fixture_text() -> str:
    return "# Reader Feedback Inbox Fixture\n\n" + fixture_report(
        date="2026-05-12",
        title="inconsistent acceptance fixture",
        issue_number=92,
        reviewer_context="ML engineer",
        status="new",
    )


def run_self_test() -> int:
    complete_issues, _ = validate_inbox_text(
        complete_fixture_text(),
        require_sprint_complete=True,
    )
    if complete_issues:
        print("Reader feedback inbox self-test failed: complete fixture should pass.")
        for issue in complete_issues:
            print(f"- {issue}")
        return 1

    incomplete_issues, _ = validate_inbox_text(
        incomplete_fixture_text(),
        require_sprint_complete=True,
    )
    if not incomplete_issues:
        print("Reader feedback inbox self-test failed: incomplete fixture should fail.")
        return 1

    private_contact_issues, _ = validate_inbox_text(
        private_contact_fixture_text(),
        require_sprint_complete=False,
    )
    if not any("email addresses" in issue for issue in private_contact_issues):
        print(
            "Reader feedback inbox self-test failed: "
            "private contact fixture should fail."
        )
        return 1

    inconsistent_issues, _ = validate_inbox_text(
        inconsistent_acceptance_fixture_text(),
        require_sprint_complete=False,
    )
    if not any("accepted triage actions require" in issue for issue in inconsistent_issues):
        print(
            "Reader feedback inbox self-test failed: "
            "inconsistent acceptance fixture should fail."
        )
        return 1

    print("Reader feedback inbox self-test passed.")
    return 0


def main() -> int:
    args = parse_args()

    if args.self_test:
        return run_self_test()

    text = INBOX.read_text(encoding="utf-8")
    issues, validations = validate_inbox_text(
        text,
        require_sprint_complete=args.require_sprint_complete,
    )

    if issues:
        print("Reader feedback inbox check failed:")
        for issue in issues:
            print(f"- {issue}")
        return 1

    if validations:
        print(f"Reader feedback inbox check passed for {len(validations)} report(s).")
    else:
        print("Reader feedback inbox check passed with no direct reports recorded.")

    print_status(validations)

    return 0


if __name__ == "__main__":
    sys.exit(main())
