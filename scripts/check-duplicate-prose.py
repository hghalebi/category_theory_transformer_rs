#!/usr/bin/env python3
"""Check authored teaching chapters for repeated long prose paragraphs."""

from __future__ import annotations

import re
import sys
from collections import defaultdict
from dataclasses import dataclass
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]

TEACHING_FILES = [
    ROOT / "book" / "src" / "welcome.md",
    ROOT / "book" / "src" / "00-map.md",
    ROOT / "book" / "src" / "01-domain-objects.md",
    ROOT / "book" / "src" / "02-morphisms-composition.md",
    ROOT / "book" / "src" / "03-ml-pipeline.md",
    ROOT / "book" / "src" / "04-training-endomorphism.md",
    ROOT / "book" / "src" / "05-structure-and-calculus.md",
    ROOT / "book" / "src" / "seven-sketches-rust.md",
    ROOT / "book" / "src" / "roadmap.md",
]

MIN_WORDS = 14
FENCE_RE = re.compile(r"^\s*(```+|~~~+)")
HEADING_RE = re.compile(r"^#{1,6}\s+")
TABLE_RE = re.compile(r"^\s*\|.+\|\s*$")
LIST_RE = re.compile(r"^\s*(?:[-*+]\s+|\d+\.\s+)")
HTML_BLOCK_RE = re.compile(r"^\s*</?(?:details|summary|div|p|h1|a|br|script)\b", re.IGNORECASE)
WORD_RE = re.compile(r"[a-z0-9_']+")


@dataclass(frozen=True)
class Paragraph:
    path: Path
    line: int
    text: str

    @property
    def normalized(self) -> str:
        return " ".join(WORD_RE.findall(self.text.lower()))

    @property
    def word_count(self) -> int:
        return len(WORD_RE.findall(self.text))


def flush_paragraph(
    paragraphs: list[Paragraph],
    path: Path,
    lines: list[str],
    start_line: int | None,
) -> tuple[list[str], int | None]:
    if lines and start_line is not None:
        text = " ".join(line.strip() for line in lines)
        paragraphs.append(Paragraph(path=path, line=start_line, text=text))

    return [], None


def paragraphs_from_file(path: Path) -> list[Paragraph]:
    paragraphs: list[Paragraph] = []
    paragraph_lines: list[str] = []
    paragraph_start: int | None = None
    in_fence = False
    detail_depth = 0

    for line_number, line in enumerate(path.read_text(encoding="utf-8").splitlines(), start=1):
        stripped = line.strip()

        if FENCE_RE.match(line):
            in_fence = not in_fence
            paragraph_lines, paragraph_start = flush_paragraph(
                paragraphs, path, paragraph_lines, paragraph_start
            )
            continue

        if in_fence:
            continue

        if stripped.lower().startswith("<details"):
            detail_depth += 1
            paragraph_lines, paragraph_start = flush_paragraph(
                paragraphs, path, paragraph_lines, paragraph_start
            )
            continue

        if detail_depth:
            if stripped.lower().startswith("</details"):
                detail_depth = max(0, detail_depth - 1)
            continue

        if (
            not stripped
            or HEADING_RE.match(line)
            or TABLE_RE.match(line)
            or LIST_RE.match(line)
            or stripped.startswith(">")
            or HTML_BLOCK_RE.match(line)
        ):
            paragraph_lines, paragraph_start = flush_paragraph(
                paragraphs, path, paragraph_lines, paragraph_start
            )
            continue

        if paragraph_start is None:
            paragraph_start = line_number
        paragraph_lines.append(line)

    flush_paragraph(paragraphs, path, paragraph_lines, paragraph_start)

    return paragraphs


def main() -> int:
    by_text: dict[str, list[Paragraph]] = defaultdict(list)

    for path in TEACHING_FILES:
        if not path.exists():
            print(f"{path}: teaching chapter file does not exist")
            return 1

        for paragraph in paragraphs_from_file(path):
            if paragraph.word_count < MIN_WORDS:
                continue

            by_text[paragraph.normalized].append(paragraph)

    issues: list[str] = []

    for normalized, paragraphs in sorted(by_text.items()):
        if len(paragraphs) < 2 or not normalized:
            continue

        locations = ", ".join(f"{paragraph.path}:{paragraph.line}" for paragraph in paragraphs)
        issues.append(f"duplicate long prose paragraph at {locations}")

    if issues:
        print("Duplicate prose check failed:")
        for issue in issues:
            print(f"- {issue}")
        return 1

    print("Duplicate prose check passed.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
