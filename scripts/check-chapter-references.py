#!/usr/bin/env python3
"""Check that major chapters stay connected to the reference map."""

from __future__ import annotations

import re
import sys
from dataclasses import dataclass
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
BOOK_SRC = ROOT / "book" / "src"
REFERENCES = BOOK_SRC / "references.md"
RESEARCH_NOTES = ROOT / "book" / "EDITORIAL_RESEARCH_NOTES.md"


@dataclass(frozen=True)
class Chapter:
    title: str
    filename: str
    min_sources: int = 3


CHAPTERS = [
    Chapter("Welcome", "welcome.md"),
    Chapter("Course Map", "00-map.md"),
    Chapter("Domain Objects", "01-domain-objects.md"),
    Chapter("Morphism and Composition", "02-morphisms-composition.md"),
    Chapter("The Tiny ML Pipeline", "03-ml-pipeline.md"),
    Chapter("Training as an Endomorphism", "04-training-endomorphism.md"),
    Chapter("Functors, Naturality, Monoids, and Chain Rule", "05-structure-and-calculus.md"),
    Chapter("Seven Sketches Through Rust", "seven-sketches-rust.md"),
    Chapter("Exercises", "exercises.md"),
    Chapter("Transformer Roadmap", "roadmap.md"),
]

REQUIRED_REFERENCE_SECTIONS = [
    "## Rust",
    "## Category Theory",
    "## Machine Learning",
    "## Category Theory And Learning Systems",
    "## Transformers",
    "## Learning Design",
]

REQUIRED_RESEARCH_BUCKETS = [
    "### Welcome And Course Map",
    "### Domain Objects",
    "### Morphisms And Composition",
    "### Tiny ML Pipeline",
    "### Training And Chain Rule",
    "### Structure And Laws",
    "### Seven Sketches Through Rust",
    "### Exercises And Transfer",
    "### Transformer Roadmap",
]


def chapter_row_pattern(chapter: Chapter) -> re.Pattern[str]:
    return re.compile(
        rf"^\|\s+\[{re.escape(chapter.title)}\]\({re.escape(chapter.filename)}\)\s+\|(?P<rest>.+)$",
        re.MULTILINE,
    )


def check_reference_sections(reference_text: str) -> list[str]:
    issues: list[str] = []

    for heading in REQUIRED_REFERENCE_SECTIONS:
        if heading not in reference_text:
            issues.append(f"{REFERENCES}: missing reference section {heading!r}")

    return issues


def check_chapter_row(reference_text: str, chapter: Chapter) -> list[str]:
    issues: list[str] = []
    match = chapter_row_pattern(chapter).search(reference_text)

    if match is None:
        return [
            f"{REFERENCES}: missing chapter reference-map row for "
            f"[{chapter.title}]({chapter.filename})"
        ]

    row = match.group(0)
    cells = [cell.strip() for cell in row.strip().strip("|").split("|")]

    if len(cells) != 3:
        issues.append(
            f"{REFERENCES}: reference-map row for {chapter.filename} should have "
            "chapter, central question, and references columns"
        )
    elif not cells[1]:
        issues.append(f"{REFERENCES}: {chapter.filename} has an empty central question")

    source_count = len(re.findall(r"https?://", row))
    if source_count < chapter.min_sources:
        issues.append(
            f"{REFERENCES}: {chapter.filename} has {source_count} external sources; "
            f"expected at least {chapter.min_sources}"
        )

    return issues


def check_chapter_bridge(chapter: Chapter) -> list[str]:
    path = BOOK_SRC / chapter.filename
    text = path.read_text(encoding="utf-8")
    has_reference_link = "[References](references.md)" in text
    has_roadmap_reference_path = "## Roadmap Reference Path" in text

    if has_reference_link or has_roadmap_reference_path:
        return []

    return [
        f"{path}: chapter should point readers to references.md or include a "
        "Roadmap Reference Path section"
    ]


def check_research_buckets(research_text: str) -> list[str]:
    issues: list[str] = []

    for heading in REQUIRED_RESEARCH_BUCKETS:
        start = research_text.find(heading)
        if start == -1:
            issues.append(f"{RESEARCH_NOTES}: missing source bucket {heading!r}")
            continue

        next_heading = research_text.find("\n### ", start + len(heading))
        bucket = research_text[start:] if next_heading == -1 else research_text[start:next_heading]
        source_count = len(re.findall(r"https?://", bucket))
        if source_count < 3:
            issues.append(
                f"{RESEARCH_NOTES}: source bucket {heading!r} has {source_count} "
                "external sources; expected at least 3"
            )

        if "Rewrite check:" not in bucket:
            issues.append(f"{RESEARCH_NOTES}: source bucket {heading!r} is missing a rewrite check")

    return issues


def main() -> int:
    reference_text = REFERENCES.read_text(encoding="utf-8")
    research_text = RESEARCH_NOTES.read_text(encoding="utf-8")
    issues: list[str] = []

    issues.extend(check_reference_sections(reference_text))
    issues.extend(check_research_buckets(research_text))

    for chapter in CHAPTERS:
        issues.extend(check_chapter_row(reference_text, chapter))
        issues.extend(check_chapter_bridge(chapter))

    if issues:
        print("Chapter reference coverage check failed:")
        for issue in issues:
            print(f"- {issue}")
        return 1

    print("Chapter reference coverage check passed.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
