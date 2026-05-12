#!/usr/bin/env python3
"""Check that README chapter maturity guidance stays aligned with the book."""

from __future__ import annotations

import sys
from dataclasses import dataclass
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
README = ROOT / "README.md"
SUMMARY = ROOT / "book" / "src" / "SUMMARY.md"


@dataclass(frozen=True)
class ChapterMaturity:
    title: str
    filename: str
    allowed_statuses: tuple[str, ...]


EXPECTED_CHAPTERS = [
    ChapterMaturity("Welcome", "welcome.md", ("Stable draft", "Draft")),
    ChapterMaturity("Course Map", "00-map.md", ("Stable draft", "Draft")),
    ChapterMaturity("Domain Objects", "01-domain-objects.md", ("Stable draft", "Draft")),
    ChapterMaturity(
        "Morphism and Composition",
        "02-morphisms-composition.md",
        ("Draft", "Stable draft"),
    ),
    ChapterMaturity("The Tiny ML Pipeline", "03-ml-pipeline.md", ("Draft",)),
    ChapterMaturity("Training as an Endomorphism", "04-training-endomorphism.md", ("Draft",)),
    ChapterMaturity(
        "Functors, Naturality, Monoids, and Chain Rule",
        "05-structure-and-calculus.md",
        ("Sketch", "Draft"),
    ),
    ChapterMaturity("Seven Sketches Through Rust", "seven-sketches-rust.md", ("Sketch", "Draft")),
    ChapterMaturity("Exercises", "exercises.md", ("Draft", "Stable draft")),
    ChapterMaturity("Transformer Roadmap", "roadmap.md", ("Sketch", "Draft")),
]

REQUIRED_STATUS_LABELS = {
    "Stable": "The explanation and code are unlikely to change heavily",
    "Draft": "The section is readable but still evolving",
    "Sketch": "The idea is present but needs clearer writing or examples",
    "Planned": "The section is part of the roadmap",
}

REQUIRED_STATUS_MARKERS = [
    "This is a working public draft.",
    "Some chapters are stable.",
    "Some chapters are skeletal.",
    "Some sections are intentionally public before they are polished.",
    "public feedback from readers",
    "If something feels too compressed, unclear, or too bullet-point-like",
]

DISALLOWED_FEEDBACK_MARKERS = ("TO" "DO", "TBD", "fix later", "coming soon")


def section_after_marker(text: str, marker: str) -> str:
    start = text.find(marker)
    if start == -1:
        return ""

    next_heading = text.find("\n## ", start + len(marker))
    if next_heading == -1:
        return text[start:]

    return text[start:next_heading]


def parse_table_after_marker(text: str, marker: str) -> list[list[str]]:
    section = section_after_marker(text, marker)
    rows: list[list[str]] = []
    in_table = False

    for line in section.splitlines()[1:]:
        stripped = line.strip()
        if not stripped:
            if in_table:
                break
            continue
        if not stripped.startswith("|"):
            if in_table:
                break
            continue

        in_table = True
        cells = [cell.strip() for cell in stripped.strip("|").split("|")]
        if cells and not all(set(cell) <= {"-", " "} for cell in cells):
            rows.append(cells)

    if rows and rows[0][0] in {"Label", "Chapter"}:
        return rows[1:]

    return rows


def check_status_intro(readme_text: str) -> list[str]:
    issues: list[str] = []
    current_status = section_after_marker(readme_text, "## Current status")

    if not current_status:
        return [f"{README}: missing '## Current status' section"]

    for marker in REQUIRED_STATUS_MARKERS:
        if marker not in current_status:
            issues.append(f"{README}: current status section missing {marker!r}")

    return issues


def check_status_labels(readme_text: str) -> list[str]:
    issues: list[str] = []
    rows = parse_table_after_marker(readme_text, "Status labels:")
    labels = {row[0]: row[1] for row in rows if len(row) >= 2}

    for label, meaning in REQUIRED_STATUS_LABELS.items():
        if labels.get(label) != meaning:
            issues.append(
                f"{README}: status label {label!r} should be present with meaning {meaning!r}"
            )

    return issues


def check_chapter_maturity_table(readme_text: str, summary_text: str) -> list[str]:
    issues: list[str] = []
    rows = parse_table_after_marker(readme_text, "Chapter maturity:")

    if not rows:
        return [f"{README}: missing chapter maturity table"]

    row_map: dict[str, list[str]] = {}
    for row in rows:
        if len(row) != 3:
            issues.append(f"{README}: chapter maturity row should have 3 columns: {row!r}")
            continue

        title = row[0]
        if title in row_map:
            issues.append(f"{README}: duplicate chapter maturity row for {title!r}")
        row_map[title] = row

    expected_titles = {chapter.title for chapter in EXPECTED_CHAPTERS}
    extra_titles = sorted(set(row_map) - expected_titles)
    for title in extra_titles:
        issues.append(f"{README}: unexpected chapter maturity row {title!r}")

    for chapter in EXPECTED_CHAPTERS:
        summary_marker = f"[{chapter.title}]({chapter.filename})"
        if summary_marker not in summary_text:
            issues.append(f"{SUMMARY}: missing summary entry {summary_marker!r}")

        row = row_map.get(chapter.title)
        if row is None:
            issues.append(f"{README}: missing chapter maturity row for {chapter.title!r}")
            continue

        _, status, feedback = row
        if status not in chapter.allowed_statuses:
            issues.append(
                f"{README}: {chapter.title!r} status is {status!r}; "
                f"expected one of {chapter.allowed_statuses!r}"
            )

        label = status.split()[0]
        if label not in REQUIRED_STATUS_LABELS:
            issues.append(f"{README}: {chapter.title!r} status label {label!r} is not defined")

        if len(feedback) < 12:
            issues.append(f"{README}: {chapter.title!r} feedback guidance is too short")

        lowered_feedback = feedback.lower()
        for marker in DISALLOWED_FEEDBACK_MARKERS:
            if marker.lower() in lowered_feedback:
                issues.append(
                    f"{README}: {chapter.title!r} feedback guidance contains stale marker "
                    f"{marker!r}"
                )

    return issues


def main() -> int:
    readme_text = README.read_text(encoding="utf-8")
    summary_text = SUMMARY.read_text(encoding="utf-8")
    issues: list[str] = []

    issues.extend(check_status_intro(readme_text))
    issues.extend(check_status_labels(readme_text))
    issues.extend(check_chapter_maturity_table(readme_text, summary_text))

    if issues:
        print("Chapter maturity check failed:")
        for issue in issues:
            print(f"- {issue}")
        return 1

    print("Chapter maturity check passed.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
