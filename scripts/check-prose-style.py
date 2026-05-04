#!/usr/bin/env python3
"""Check authored mdBook prose for list-heavy or fragment-heavy drift."""

from __future__ import annotations

import re
import sys
from dataclasses import dataclass
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
BOOK_SRC = ROOT / "book" / "src"

IGNORED_FILES = {
    "SUMMARY.md",
    "glossary.md",
    "references.md",
    "source-snapshots.md",
}

LIST_DENSITY_IGNORED_FILES = {
    "exercises.md",
}

VERY_SHORT_WORDS = 8
MAX_SHORT_PARAGRAPH_RUN = 5
MAX_SECTION_LIST_DENSITY = 0.48
MAX_FILE_LIST_DENSITY = 0.30
MIN_SECTION_LIST_LINES = 4
MIN_FILE_LIST_LINES = 12

FENCE_RE = re.compile(r"^\s*(```+|~~~+)")
HEADING_RE = re.compile(r"^(#{1,6})\s+\S")
LIST_RE = re.compile(r"^\s*(?:[-*+]\s+|\d+\.\s+)")
TABLE_RE = re.compile(r"^\s*\|.+\|\s*$")
HTML_BLOCK_RE = re.compile(r"^\s*</?(?:div|p|h1|a|br)\b", re.IGNORECASE)
WORD_RE = re.compile(r"[A-Za-z0-9_']+")


@dataclass(frozen=True)
class Paragraph:
    line: int
    text: str
    segment: int

    @property
    def word_count(self) -> int:
        return len(WORD_RE.findall(self.text))


@dataclass
class SectionStats:
    heading: str
    line: int
    content_lines: int = 0
    list_lines: int = 0


@dataclass
class FileStats:
    paragraphs: list[Paragraph]
    sections: list[SectionStats]
    content_lines: int
    list_lines: int
    heading_followed_by_list: list[tuple[int, str]]


def is_ignored(path: Path) -> bool:
    return path.name in IGNORED_FILES


def flush_paragraph(
    paragraphs: list[Paragraph],
    paragraph_lines: list[str],
    start_line: int | None,
    segment: int,
) -> tuple[list[str], int | None]:
    if paragraph_lines and start_line is not None:
        text = " ".join(line.strip() for line in paragraph_lines)
        paragraphs.append(Paragraph(start_line, text, segment))
    return [], None


def classify_content(path: Path) -> FileStats:
    lines = path.read_text(encoding="utf-8").splitlines()
    paragraphs: list[Paragraph] = []
    sections: list[SectionStats] = []
    heading_followed_by_list: list[tuple[int, str]] = []

    in_fence = False
    detail_depth = 0
    paragraph_lines: list[str] = []
    paragraph_start: int | None = None
    prose_segment = 0
    content_lines = 0
    list_lines = 0
    current_section = SectionStats("<file>", 1)
    sections.append(current_section)
    pending_heading: tuple[int, str] | None = None

    for line_no, line in enumerate(lines, start=1):
        stripped = line.strip()

        if FENCE_RE.match(line):
            in_fence = not in_fence
            paragraph_lines, paragraph_start = flush_paragraph(
                paragraphs, paragraph_lines, paragraph_start, prose_segment
            )
            prose_segment += 1
            pending_heading = None
            continue

        if in_fence:
            continue

        if stripped.lower().startswith("<details"):
            detail_depth += 1
            paragraph_lines, paragraph_start = flush_paragraph(
                paragraphs, paragraph_lines, paragraph_start, prose_segment
            )
            prose_segment += 1
            pending_heading = None
            continue

        if detail_depth:
            if stripped.lower().startswith("</details"):
                detail_depth = max(0, detail_depth - 1)
            continue

        if not stripped:
            paragraph_lines, paragraph_start = flush_paragraph(
                paragraphs, paragraph_lines, paragraph_start, prose_segment
            )
            continue

        heading_match = HEADING_RE.match(line)
        if heading_match:
            paragraph_lines, paragraph_start = flush_paragraph(
                paragraphs, paragraph_lines, paragraph_start, prose_segment
            )
            prose_segment += 1
            current_section = SectionStats(stripped, line_no)
            sections.append(current_section)
            pending_heading = (line_no, stripped)
            continue

        is_list = bool(LIST_RE.match(line))
        is_table = bool(TABLE_RE.match(line))
        is_html = bool(HTML_BLOCK_RE.match(line))
        is_blockquote = stripped.startswith(">")

        if is_list:
            if pending_heading is not None:
                heading_followed_by_list.append(pending_heading)
            content_lines += 1
            list_lines += 1
            current_section.content_lines += 1
            current_section.list_lines += 1
            paragraph_lines, paragraph_start = flush_paragraph(
                paragraphs, paragraph_lines, paragraph_start, prose_segment
            )
            prose_segment += 1
            pending_heading = None
            continue

        pending_heading = None

        if is_table or is_html or is_blockquote:
            paragraph_lines, paragraph_start = flush_paragraph(
                paragraphs, paragraph_lines, paragraph_start, prose_segment
            )
            prose_segment += 1
            continue

        content_lines += 1
        current_section.content_lines += 1
        if paragraph_start is None:
            paragraph_start = line_no
        paragraph_lines.append(line)

    flush_paragraph(paragraphs, paragraph_lines, paragraph_start, prose_segment)

    return FileStats(
        paragraphs=paragraphs,
        sections=sections,
        content_lines=content_lines,
        list_lines=list_lines,
        heading_followed_by_list=heading_followed_by_list,
    )


def short_paragraph_issues(path: Path, stats: FileStats) -> list[str]:
    issues: list[str] = []
    run: list[Paragraph] = []
    current_segment: int | None = None

    for paragraph in stats.paragraphs:
        if current_segment != paragraph.segment:
            if len(run) > MAX_SHORT_PARAGRAPH_RUN:
                first = run[0]
                issues.append(
                    f"{path}:{first.line}: {len(run)} consecutive very short "
                    "paragraphs; combine nearby sentence fragments into fuller prose."
                )
            run = []
            current_segment = paragraph.segment

        if paragraph.word_count <= VERY_SHORT_WORDS:
            run.append(paragraph)
        else:
            if len(run) > MAX_SHORT_PARAGRAPH_RUN:
                first = run[0]
                issues.append(
                    f"{path}:{first.line}: {len(run)} consecutive very short "
                    "paragraphs; combine nearby sentence fragments into fuller prose."
                )
            run = []

    if len(run) > MAX_SHORT_PARAGRAPH_RUN:
        first = run[0]
        issues.append(
            f"{path}:{first.line}: {len(run)} consecutive very short "
            "paragraphs; combine nearby sentence fragments into fuller prose."
        )

    return issues


def list_density_issues(path: Path, stats: FileStats) -> list[str]:
    issues: list[str] = []

    if path.name in LIST_DENSITY_IGNORED_FILES:
        return issues

    if (
        stats.list_lines >= MIN_FILE_LIST_LINES
        and stats.content_lines
        and stats.list_lines / stats.content_lines > MAX_FILE_LIST_DENSITY
    ):
        issues.append(
            f"{path}:1: list density is {stats.list_lines}/{stats.content_lines}; "
            "convert dense teaching lists into paragraphs or diagrams."
        )

    for section in stats.sections:
        if section.heading == "<file>" or section.content_lines == 0:
            continue

        if (
            section.list_lines >= MIN_SECTION_LIST_LINES
            and section.list_lines / section.content_lines > MAX_SECTION_LIST_DENSITY
        ):
            issues.append(
                f"{path}:{section.line}: section list density is "
                f"{section.list_lines}/{section.content_lines}; introduce prose "
                "or split the section."
            )

    return issues


def heading_list_issues(path: Path, stats: FileStats) -> list[str]:
    return [
        f"{path}:{line}: heading is followed immediately by a list; add an "
        f"introductory sentence after {heading!r}."
        for line, heading in stats.heading_followed_by_list
    ]


def check_file(path: Path) -> list[str]:
    stats = classify_content(path)
    return [
        *heading_list_issues(path, stats),
        *short_paragraph_issues(path, stats),
        *list_density_issues(path, stats),
    ]


def main() -> int:
    markdown_files = sorted(BOOK_SRC.glob("*.md"))
    issues: list[str] = []

    for path in markdown_files:
        if is_ignored(path):
            continue
        issues.extend(check_file(path.relative_to(ROOT)))

    if issues:
        print("Prose style check failed:")
        for issue in issues:
            print(f"- {issue}")
        return 1

    print("Prose style check passed.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
