#!/usr/bin/env python3
"""Render GitHub reader-confusion issues as inbox report drafts."""

from __future__ import annotations

import argparse
import json
import re
import subprocess
import sys
from dataclasses import dataclass
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
INBOX = ROOT / "community" / "reader-feedback-inbox.md"
DEFAULT_REPO = "hghalebi/category_theory_transformer_rs"

SECTION_TO_FIELD = {
    "Review path": "Review path",
    "Reviewer context": "Reviewer context",
    "Location": "Location",
    "Command or file tried": "Command or file tried",
    "Friction lens": "Friction lens",
    "Confusing text or output": "Confusing text or output",
    "What made sense before that point?": "Last clear idea",
    "Where did the mental model break?": "Where the mental model broke",
    "What did you expect next?": "Expected next step",
    "Smallest useful fix": "Smallest useful fix",
}

REPORT_FIELDS = [
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

EMAIL_ADDRESS_PATTERN = re.compile(
    r"\b[A-Z0-9._%+-]+@[A-Z0-9.-]+\.[A-Z]{2,}\b",
    re.IGNORECASE,
)


@dataclass(frozen=True)
class Issue:
    number: int
    title: str
    url: str
    created_at: str
    body: str


def parse_issue_form_body(body: str) -> dict[str, str]:
    sections: dict[str, str] = {}
    matches = list(re.finditer(r"^### (?P<label>.+?)\s*$", body, re.MULTILINE))

    for index, match in enumerate(matches):
        label = match.group("label").strip()
        start = match.end()
        end = matches[index + 1].start() if index + 1 < len(matches) else len(body)
        value = body[start:end].strip()
        if value == "_No response_":
            value = ""
        sections[label] = value

    return sections


def report_title(issue: Issue) -> str:
    title = re.sub(r"^\[reader confusion\]\s*", "", issue.title).strip()
    return redact_private_contact_details(title or f"reader feedback issue {issue.number}")


def report_date(issue: Issue) -> str:
    if re.match(r"^\d{4}-\d{2}-\d{2}", issue.created_at):
        return issue.created_at[:10]
    return "YYYY-MM-DD"


def value_or_note(value: str, note: str) -> str:
    normalized = value.strip()
    return normalized if normalized else note


def redact_private_contact_details(value: str) -> str:
    return EMAIL_ADDRESS_PATTERN.sub("[redacted email]", value)


def issue_to_report(issue: Issue) -> str:
    sections = parse_issue_form_body(issue.body)
    values: dict[str, str] = {
        "Source": f"GitHub issue #{issue.number}",
        "GitHub issue URL": issue.url,
        "Triage action": "needs clarification",
        "Affected chapter section": "not triaged yet",
        "Repository code or example": "not triaged yet",
        "References checked": "not triaged yet",
        "Rewrite log entry": "not rewritten yet",
        "Validation": "not run yet",
        "Status": "new",
    }

    for section_label, inbox_field in SECTION_TO_FIELD.items():
        values[inbox_field] = value_or_note(
            redact_private_contact_details(sections.get(section_label, "")),
            "not supplied in issue form",
        )

    lines = [f"### Report {report_date(issue)}: {report_title(issue)}", ""]
    for field in REPORT_FIELDS:
        lines.append(f"{field}: {values[field]}")
    return "\n".join(lines)


def fetch_issues(repo: str) -> list[Issue]:
    command = [
        "gh",
        "issue",
        "list",
        "--repo",
        repo,
        "--state",
        "all",
        "--label",
        "reader confusion",
        "--limit",
        "100",
        "--json",
        "number,title,url,createdAt,body",
    ]
    result = subprocess.run(command, check=True, text=True, capture_output=True)
    issues = json.loads(result.stdout)

    return [
        Issue(
            number=issue["number"],
            title=issue["title"],
            url=issue["url"],
            created_at=issue["createdAt"],
            body=issue.get("body") or "",
        )
        for issue in issues
    ]


def inbox_contains_issue(issue: Issue) -> bool:
    return issue.url in INBOX.read_text(encoding="utf-8")


def print_reports(issues: list[Issue], include_existing: bool) -> int:
    pending = [
        issue
        for issue in issues
        if include_existing or not inbox_contains_issue(issue)
    ]

    if not pending:
        print("No reader confusion issues need inbox drafts.")
        return 0

    for index, issue in enumerate(pending):
        if index:
            print()
        print(issue_to_report(issue))

    return 0


def run_self_test() -> int:
    issue = Issue(
        number=42,
        title="[reader confusion] logits became probabilities too fast from reader@example.com",
        url="https://github.com/hghalebi/category_theory_transformer_rs/issues/42",
        created_at="2026-05-12T10:00:00Z",
        body="""### Review path

One core chapter

### Reviewer context

ML engineer

### Location

book/src/03-ml-pipeline.md

### Command or file tried

cargo run --example 02_morphism_composition

### Friction lens

ML intuition

### Confusing text or output

Logits became Distribution before I knew why.

### What made sense before that point?

TokenId to Vector made sense.

### Where did the mental model break?

I did not know why exponentials appeared.

### What did you expect next?

A one-sentence softmax bridge.

### Smallest useful fix

Add a short raw-score-to-probability checkpoint. Contact me at reader@example.com.
""",
    )
    report = issue_to_report(issue)
    required = [
        "### Report 2026-05-12: logits became probabilities too fast from [redacted email]",
        "Source: GitHub issue #42",
        "Review path: One core chapter",
        "Reviewer context: ML engineer",
        "Friction lens: ML intuition",
        "Smallest useful fix: Add a short raw-score-to-probability checkpoint. Contact me at [redacted email].",
        "Status: new",
    ]
    forbidden = ["reader@example.com"]

    missing = [item for item in required if item not in report]
    leaked = [item for item in forbidden if item in report]
    if missing or leaked:
        print("Self-test failed:")
        for item in missing:
            print(f"- missing {item!r}")
        for item in leaked:
            print(f"- leaked {item!r}")
        return 1

    print("Reader feedback issue collector self-test passed.")
    return 0


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description=(
            "Fetch GitHub reader-confusion issues and render inbox-ready "
            "report drafts."
        )
    )
    parser.add_argument("--repo", default=DEFAULT_REPO)
    parser.add_argument(
        "--include-existing",
        action="store_true",
        help="Render issues even if their URL is already present in the inbox.",
    )
    parser.add_argument(
        "--require-issues",
        action="store_true",
        help="Fail when there are no reader-confusion issues to render.",
    )
    parser.add_argument(
        "--self-test",
        action="store_true",
        help="Run an offline parser self-test without calling GitHub.",
    )
    return parser.parse_args()


def main() -> int:
    args = parse_args()

    if args.self_test:
        return run_self_test()

    issues = fetch_issues(args.repo)
    if not issues:
        print("No reader confusion issues found.")
        return 1 if args.require_issues else 0

    return print_reports(issues, include_existing=args.include_existing)


if __name__ == "__main__":
    sys.exit(main())
