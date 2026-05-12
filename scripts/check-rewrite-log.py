#!/usr/bin/env python3
"""Check that chapter rewrite passes stay auditable."""

from __future__ import annotations

import re
import sys
from dataclasses import dataclass
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
REWRITE_LOG = ROOT / "book" / "CHAPTER_REWRITE_LOG.md"


@dataclass(frozen=True)
class ChapterEvidence:
    filename: str
    required_lenses: tuple[str, ...] = ()


MAJOR_CHAPTERS = [
    ChapterEvidence("book/src/welcome.md"),
    ChapterEvidence(
        "book/src/00-map.md",
        (
            "ML teaching critique:",
            "Visual/tutorial critique:",
            "Category-theory critique:",
            "Learner critique:",
        ),
    ),
    ChapterEvidence(
        "book/src/01-domain-objects.md",
        (
            "ML teaching critique:",
            "Category-theory critique:",
            "Learner critique:",
        ),
    ),
    ChapterEvidence(
        "book/src/02-morphisms-composition.md",
        (
            "ML teaching critique:",
            "Category-theory critique:",
            "Learner critique:",
        ),
    ),
    ChapterEvidence(
        "book/src/03-ml-pipeline.md",
        (
            "ML teaching critique:",
            "Category-theory critique:",
            "Learner critique:",
        ),
    ),
    ChapterEvidence(
        "book/src/04-training-endomorphism.md",
        (
            "ML teaching critique:",
            "Category-theory critique:",
            "Learner critique:",
        ),
    ),
    ChapterEvidence(
        "book/src/05-structure-and-calculus.md",
        (
            "ML teaching critique:",
            "Category-theory critique:",
            "Learner critique:",
        ),
    ),
    ChapterEvidence(
        "book/src/seven-sketches-rust.md",
        (
            "ML teaching critique:",
            "Category-theory critique:",
            "Learner critique:",
        ),
    ),
    ChapterEvidence("book/src/exercises.md"),
    ChapterEvidence("book/src/glossary.md"),
    ChapterEvidence("book/src/roadmap.md"),
    ChapterEvidence("book/src/source-snapshots.md"),
]

REQUIRED_GLOBAL_MARKERS = [
    "## Success Criteria",
    "Critique before rewrite:",
    "Rewrite decision:",
    "Validation:",
    "bash scripts/check.sh",
]

STALE_MARKERS = [
    "## Next Pass",
]


def split_sections(text: str) -> list[str]:
    matches = list(re.finditer(r"^## .+$", text, re.MULTILINE))
    sections: list[str] = []

    for index, match in enumerate(matches):
        next_start = matches[index + 1].start() if index + 1 < len(matches) else len(text)
        sections.append(text[match.start() : next_start])

    return sections


def section_for_filename(sections: list[str], filename: str) -> str | None:
    for section in sections:
        if filename in section:
            return section

    return None


def check_global_markers(text: str) -> list[str]:
    issues: list[str] = []

    for marker in REQUIRED_GLOBAL_MARKERS:
        if marker not in text:
            issues.append(f"{REWRITE_LOG}: missing global marker {marker!r}")

    for marker in STALE_MARKERS:
        if marker in text:
            issues.append(f"{REWRITE_LOG}: stale planning marker remains: {marker!r}")

    return issues


def check_chapter_evidence(sections: list[str]) -> list[str]:
    issues: list[str] = []

    for chapter in MAJOR_CHAPTERS:
        section = section_for_filename(sections, chapter.filename)

        if section is None:
            issues.append(f"{REWRITE_LOG}: no rewrite-log section mentions {chapter.filename}")
            continue

        for marker in ("Critique before rewrite:", "Rewrite decision:", "Validation:"):
            if marker not in section:
                issues.append(
                    f"{REWRITE_LOG}: section for {chapter.filename} missing {marker!r}"
                )

        for lens in chapter.required_lenses:
            if lens not in section:
                issues.append(
                    f"{REWRITE_LOG}: section for {chapter.filename} missing {lens!r}"
                )

    return issues


def main() -> int:
    text = REWRITE_LOG.read_text(encoding="utf-8")
    sections = split_sections(text)
    issues: list[str] = []

    issues.extend(check_global_markers(text))
    issues.extend(check_chapter_evidence(sections))

    if issues:
        print("Rewrite log check failed:")
        for issue in issues:
            print(f"- {issue}")
        return 1

    print("Rewrite log check passed.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
