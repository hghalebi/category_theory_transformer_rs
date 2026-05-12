#!/usr/bin/env python3
"""Check that the textbook completion audit remains evidence based."""

from __future__ import annotations

import re
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
AUDIT = ROOT / "book" / "TEXTBOOK_COMPLETION_AUDIT.md"
READINESS_SCRIPT = ROOT / "scripts" / "check-textbook-readiness.sh"

REQUIRED_SECTIONS = [
    "## Objective",
    "## Prompt-To-Artifact Checklist",
    "## Latest Validation Evidence",
    "## Current Missing Work",
    "## Completion Rule",
]

REQUIRED_OBJECTIVE_ITEMS = [
    "researching authoritative sources",
    "organizing chapter references",
    "drafting chapters",
    "critiquing chapters from expert and learner perspectives",
    "rewriting chapters iteratively",
    "validating the book against real repository code",
]

REQUIRED_CHECKLIST_REQUIREMENTS = [
    "Research authoritative sources",
    "Reference link freshness workflow",
    "Organize references by chapter",
    "Keep chapter contracts aligned",
    "Keep chapter quality scorecard aligned",
    "Ground the practice model in learning science",
    "Draft every current chapter",
    "Critique before rewrite",
    "Rewrite iteratively",
    "Explain Rust syntax",
    "Explain ML concepts",
    "Explain category-theory concepts",
    "Tie book to repository code",
    "Add runnable examples",
    "Add tests for claims",
    "Add expected exercise reasoning",
    "Align chapters to exercises",
    "Avoid learner-facing instructor naming",
    "Avoid discussing the book generator in learner content",
    "Add diagrams",
    "Add law/boundary summary tables",
    "Core terminology consistency",
    "Public chapter maturity status",
    "Core chapter practice alignment",
    "Core duplicate prose sweep",
    "Add typed attention roadmap material",
    "Synthesize public learner-friction signals",
    "Prepare external reader review loop",
    "Validate final artifact",
    "Check textbook readiness before completion claims",
]

REQUIRED_VALIDATION_MARKERS = [
    "bash scripts/check.sh",
    "python3 scripts/check-prose-style.py",
    "python3 scripts/check-rewrite-log.py",
    "python3 scripts/check-chapter-references.py",
    "python3 scripts/check-chapter-contracts.py",
    "python3 scripts/check-chapter-scorecard.py",
    "python3 scripts/check-chapter-scorecard.py --self-test",
    "python3 scripts/check-source-authority.py",
    "python3 scripts/check-reference-links-live.py --self-test",
    "python3 scripts/check-chapter-maturity.py",
    "python3 scripts/check-diagram-coverage.py",
    "python3 scripts/check-exercise-alignment.py",
    "python3 scripts/check-exercise-commands.py",
    "python3 scripts/check-duplicate-prose.py",
    "python3 scripts/check-reader-feedback-loop.py",
    "python3 scripts/check-reader-feedback-inbox.py",
    "python3 scripts/check-reader-feedback-inbox.py --self-test",
    "python3 scripts/check-reader-feedback-inbox.py --require-sprint-complete",
    "python3 scripts/check-reviewer-slot-tracker.py",
    "python3 scripts/check-reviewer-slot-tracker.py --self-test",
    "python3 scripts/check-public-friction-matrix.py",
    "python3 scripts/collect-reader-feedback-issues.py --self-test",
    "bash scripts/check-github-feedback-surface.sh --self-test",
    "scripts/reader-feedback-status.sh --self-test",
    "python3 scripts/collect-reader-feedback-issues.py",
    "scripts/reader-feedback-status.sh --live",
    "scripts/check-github-feedback-surface.sh",
    "scripts/check-textbook-readiness.sh",
    "bash scripts/check-mdbook-coverage.sh",
    "git diff --check -- . ':!target'",
]

REQUIRED_MISSING_WORK_MARKERS = [
    "external reader confusion reports",
    "community/reviewer-outreach.md",
    "community/reader-review-sprint.md",
    "community/reader-review-guide.md",
    "community/reader-feedback-inbox.md",
    "community/reader-feedback-triage.md",
    "https://github.com/hghalebi/category_theory_transformer_rs/issues/6",
    "python3 scripts/check-reader-feedback-inbox.py --require-sprint-complete",
    "chapter rewrites",
]

REQUIRED_READINESS_MARKERS = [
    "bash scripts/check.sh",
    "python3 scripts/check-completion-audit.py",
    "python3 scripts/check-reader-feedback-loop.py",
    "python3 scripts/check-reader-feedback-inbox.py",
    "python3 scripts/check-reader-feedback-inbox.py --self-test",
    "python3 scripts/collect-reader-feedback-issues.py --self-test",
    "bash scripts/check-github-feedback-surface.sh --self-test",
    "scripts/reader-feedback-status.sh --self-test",
    "python3 scripts/check-reviewer-slot-tracker.py",
    "python3 scripts/check-reviewer-slot-tracker.py --self-test",
    "python3 scripts/check-reader-feedback-inbox.py --require-sprint-complete",
    "python3 scripts/check-rewrite-log.py",
    "python3 scripts/check-prose-style.py",
    "python3 scripts/check-chapter-references.py",
    "python3 scripts/check-chapter-contracts.py",
    "python3 scripts/check-chapter-scorecard.py",
    "python3 scripts/check-chapter-scorecard.py --self-test",
    "python3 scripts/check-source-authority.py",
    "python3 scripts/check-reference-links-live.py --self-test",
    "python3 scripts/check-chapter-maturity.py",
    "python3 scripts/check-diagram-coverage.py",
    "python3 scripts/check-public-friction-matrix.py",
    "python3 scripts/check-exercise-alignment.py",
    "python3 scripts/check-exercise-commands.py",
    "python3 scripts/check-duplicate-prose.py",
    "git diff --check -- . ':!target'",
]


def checklist_rows(text: str) -> list[list[str]]:
    rows: list[list[str]] = []

    for line in text.splitlines():
        stripped = line.strip()
        if not stripped.startswith("|") or stripped.startswith("| ---"):
            continue

        cells = [cell.strip() for cell in stripped.strip("|").split("|")]
        if cells and cells[0] != "Requirement":
            rows.append(cells)

    return rows


def section_text(text: str, heading: str) -> str:
    start = text.find(heading)
    if start == -1:
        return ""

    next_heading = re.search(r"^## ", text[start + len(heading) :], re.MULTILINE)
    if next_heading is None:
        return text[start:]

    return text[start : start + len(heading) + next_heading.start()]


def check_required_sections(text: str) -> list[str]:
    return [f"{AUDIT}: missing section {section!r}" for section in REQUIRED_SECTIONS if section not in text]


def check_objective(text: str) -> list[str]:
    objective = section_text(text, "## Objective")
    issues: list[str] = []

    for item in REQUIRED_OBJECTIVE_ITEMS:
        if item not in objective:
            issues.append(f"{AUDIT}: objective missing {item!r}")

    return issues


def check_checklist(text: str) -> list[str]:
    issues: list[str] = []
    rows = checklist_rows(section_text(text, "## Prompt-To-Artifact Checklist"))
    row_requirements = [row[0] for row in rows if row]

    for requirement in REQUIRED_CHECKLIST_REQUIREMENTS:
        if requirement not in row_requirements:
            issues.append(f"{AUDIT}: checklist missing requirement {requirement!r}")

    for row in rows:
        if len(row) != 4:
            issues.append(f"{AUDIT}: checklist row should have 4 columns: {row!r}")
            continue

        requirement, evidence, status, gap = row
        if not evidence:
            issues.append(f"{AUDIT}: checklist row {requirement!r} has empty evidence")
        if not status:
            issues.append(f"{AUDIT}: checklist row {requirement!r} has empty current status")
        if not gap:
            issues.append(f"{AUDIT}: checklist row {requirement!r} has empty gap")

    return issues


def check_validation(text: str) -> list[str]:
    validation = section_text(text, "## Latest Validation Evidence")
    issues: list[str] = []

    for marker in REQUIRED_VALIDATION_MARKERS:
        if marker not in validation:
            issues.append(f"{AUDIT}: validation evidence missing {marker!r}")

    if "106 Rust tests passed" not in validation:
        issues.append(f"{AUDIT}: validation evidence should record the Rust test count")

    return issues


def check_missing_work(text: str) -> list[str]:
    missing = section_text(text, "## Current Missing Work")
    issues: list[str] = []

    for marker in REQUIRED_MISSING_WORK_MARKERS:
        if marker not in missing:
            issues.append(f"{AUDIT}: current missing work missing {marker!r}")

    if re.search(r"^\d+\.\s+", missing, re.MULTILINE) is None:
        issues.append(f"{AUDIT}: current missing work should include a numbered list")

    return issues


def check_completion_rule(text: str) -> list[str]:
    completion_rule = section_text(text, "## Completion Rule")
    normalized_rule = " ".join(completion_rule.split())
    issues: list[str] = []

    required_phrases = [
        "Do not mark the textbook goal complete while this audit lists missing work.",
        "Passing validation means the current state is coherent.",
        "It does not mean the book is world-class or finished.",
    ]

    for phrase in required_phrases:
        if phrase not in normalized_rule:
            issues.append(f"{AUDIT}: completion rule missing {phrase!r}")

    return issues


def check_readiness_script() -> list[str]:
    script = READINESS_SCRIPT.read_text(encoding="utf-8")
    issues: list[str] = []

    for marker in REQUIRED_READINESS_MARKERS:
        if marker not in script:
            issues.append(f"{READINESS_SCRIPT}: readiness gate missing {marker!r}")

    if "Textbook readiness check failed. Do not mark the textbook goal complete." not in script:
        issues.append(f"{READINESS_SCRIPT}: readiness gate should fail honestly before completion")

    return issues


def main() -> int:
    text = AUDIT.read_text(encoding="utf-8")
    issues: list[str] = []

    issues.extend(check_required_sections(text))
    issues.extend(check_objective(text))
    issues.extend(check_checklist(text))
    issues.extend(check_validation(text))
    issues.extend(check_missing_work(text))
    issues.extend(check_completion_rule(text))
    issues.extend(check_readiness_script())

    if issues:
        print("Completion audit check failed:")
        for issue in issues:
            print(f"- {issue}")
        return 1

    print("Completion audit check passed.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
