#!/usr/bin/env python3
"""Check that the chapter quality scorecard remains usable for rewrites."""

from __future__ import annotations

import argparse
import importlib.util
import re
import sys
from dataclasses import dataclass
from pathlib import Path

sys.dont_write_bytecode = True


ROOT = Path(__file__).resolve().parents[1]
SCORECARD = ROOT / "book" / "CHAPTER_QUALITY_SCORECARD.md"
CONTRACTS = ROOT / "book" / "CHAPTER_CONTRACTS.md"
INBOX = ROOT / "community" / "reader-feedback-inbox.md"
INBOX_CHECKER = ROOT / "scripts" / "check-reader-feedback-inbox.py"


@dataclass(frozen=True)
class Chapter:
    title: str
    filename: str


EXPECTED_CHAPTERS = [
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

REQUIRED_SECTIONS = (
    "# Chapter Quality Scorecard",
    "Status: Active rewrite support",
    "Rubric:",
    "## Scorecard Rule",
)

QUALITY_VALUES = {"Strong", "Developing", "Reader-needed"}
DIRECT_READER_VALUES = {"Pending", "Accepted", "Rewritten"}

CHAPTER_EVIDENCE_MARKERS = {
    "Welcome": ("welcome.md", "welcome"),
    "Course Map": ("00-map.md", "course map"),
    "Domain Objects": ("01-domain-objects.md", "domain objects"),
    "Morphism and Composition": (
        "02-morphisms-composition.md",
        "morphism",
        "composition",
    ),
    "The Tiny ML Pipeline": ("03-ml-pipeline.md", "tiny ml pipeline", "ml pipeline"),
    "Training as an Endomorphism": (
        "04-training-endomorphism.md",
        "training as an endomorphism",
        "training",
    ),
    "Functors, Naturality, Monoids, and Chain Rule": (
        "05-structure-and-calculus.md",
        "structure and calculus",
        "functor",
        "naturality",
        "monoid",
        "chain rule",
    ),
    "Seven Sketches Through Rust": ("seven-sketches-rust.md", "seven sketches"),
    "Exercises": ("exercises.md", "exercise"),
    "Transformer Roadmap": ("roadmap.md", "transformer roadmap", "attention", "roadmap"),
}


def parse_table_after_marker(text: str, marker: str) -> list[list[str]]:
    start = text.find(marker)
    if start == -1:
        return []

    rows: list[list[str]] = []
    in_table = False

    for line in text[start:].splitlines()[1:]:
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

    if rows and rows[0][0] == "Chapter":
        return rows[1:]

    return rows


def contract_chapter_links(contract_text: str) -> set[str]:
    rows = parse_table_after_marker(contract_text, "| Chapter | Maturity |")
    return {row[0] for row in rows if row}


def load_inbox_checker():
    spec = importlib.util.spec_from_file_location("reader_feedback_inbox", INBOX_CHECKER)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"could not load {INBOX_CHECKER}")

    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


def accepted_direct_reports(inbox_text: str) -> tuple[int, list[str]]:
    checker = load_inbox_checker()
    inbox_issues, validations = checker.validate_inbox_text(
        inbox_text,
        require_sprint_complete=False,
    )
    accepted_count = sum(1 for validation in validations if validation.is_accepted)

    return accepted_count, [
        f"{INBOX}: inbox evidence must be valid before scorecard consistency: {issue}"
        for issue in inbox_issues
    ]


def chapter_report_counts(inbox_text: str) -> tuple[dict[str, int], dict[str, int], list[str]]:
    checker = load_inbox_checker()
    inbox_issues, validations = checker.validate_inbox_text(
        inbox_text,
        require_sprint_complete=False,
    )
    accepted_counts = {chapter.title: 0 for chapter in EXPECTED_CHAPTERS}
    rewritten_counts = {chapter.title: 0 for chapter in EXPECTED_CHAPTERS}

    for validation in validations:
        if not validation.is_accepted:
            continue

        evidence_text = "\n".join(
            [
                validation.values.get("Location", ""),
                validation.values.get("Affected chapter section", ""),
            ]
        ).lower()

        for chapter_title, markers in CHAPTER_EVIDENCE_MARKERS.items():
            if not any(marker in evidence_text for marker in markers):
                continue

            accepted_counts[chapter_title] += 1
            if validation.normalized_status == "rewritten":
                rewritten_counts[chapter_title] += 1

    return accepted_counts, rewritten_counts, [
        f"{INBOX}: inbox evidence must be valid before scorecard consistency: {issue}"
        for issue in inbox_issues
    ]


def check_required_sections(text: str) -> list[str]:
    return [f"{SCORECARD}: missing section marker {section!r}" for section in REQUIRED_SECTIONS if section not in text]


def check_rubric(text: str) -> list[str]:
    issues: list[str] = []
    normalized_text = " ".join(text.split())

    for marker in (
        "Strong:",
        "Developing:",
        "Reader-needed:",
        "Pending:",
    ):
        if marker not in text:
            issues.append(f"{SCORECARD}: rubric missing marker {marker!r}")

    prose_marker = "A chapter can be source-backed and runnable while still needing direct reader evidence."
    if prose_marker not in normalized_text:
        issues.append(f"{SCORECARD}: rubric missing marker {prose_marker!r}")

    return issues


def check_rows(scorecard_text: str, contract_text: str, inbox_text: str) -> list[str]:
    issues: list[str] = []
    rows = parse_table_after_marker(scorecard_text, "| Chapter | Source grounding |")
    row_map: dict[str, list[str]] = {}
    contract_links = contract_chapter_links(contract_text)
    direct_reports, inbox_issues = accepted_direct_reports(inbox_text)
    issues.extend(inbox_issues)
    chapter_accepted, chapter_rewritten, chapter_inbox_issues = chapter_report_counts(inbox_text)
    issues.extend(chapter_inbox_issues)

    if not rows:
        return [f"{SCORECARD}: missing chapter quality table"]

    for row in rows:
        if len(row) != 9:
            issues.append(f"{SCORECARD}: scorecard row should have 9 columns: {row!r}")
            continue

        link = row[0]
        match = re.match(r"\[(?P<title>.+)\]\(src/(?P<filename>.+)\)", link)
        if match is None:
            issues.append(f"{SCORECARD}: first cell should link to book/src chapter: {link!r}")
            continue

        title = match.group("title")
        if title in row_map:
            issues.append(f"{SCORECARD}: duplicate scorecard row for {title!r}")
        row_map[title] = row

    expected_titles = {chapter.title for chapter in EXPECTED_CHAPTERS}
    for title in sorted(set(row_map) - expected_titles):
        issues.append(f"{SCORECARD}: unexpected scorecard row {title!r}")

    for chapter in EXPECTED_CHAPTERS:
        expected_link = f"[{chapter.title}](src/{chapter.filename})"
        row = row_map.get(chapter.title)
        if row is None:
            issues.append(f"{SCORECARD}: missing scorecard row for {chapter.title!r}")
            continue

        if row[0] != expected_link:
            issues.append(f"{SCORECARD}: {chapter.title!r} link should be {expected_link!r}")

        if expected_link not in contract_links:
            issues.append(f"{CONTRACTS}: missing chapter contract for {expected_link!r}")

        quality_cells = row[1:7]
        for value in quality_cells:
            if value not in QUALITY_VALUES:
                issues.append(
                    f"{SCORECARD}: {chapter.title!r} quality value {value!r} "
                    f"must be one of {sorted(QUALITY_VALUES)!r}"
                )

        direct_reader_value = row[7]
        if direct_reader_value not in DIRECT_READER_VALUES:
            issues.append(
                f"{SCORECARD}: {chapter.title!r} direct-reader value {direct_reader_value!r} "
                f"must be one of {sorted(DIRECT_READER_VALUES)!r}"
            )

        if direct_reports == 0 and direct_reader_value != "Pending":
            issues.append(
                f"{SCORECARD}: {chapter.title!r} cannot claim direct-reader evidence "
                "while the inbox has no accepted reports"
            )

        if direct_reader_value == "Accepted" and chapter_accepted[chapter.title] == 0:
            issues.append(
                f"{SCORECARD}: {chapter.title!r} cannot claim Accepted direct-reader "
                "evidence without an accepted inbox report for that chapter"
            )

        if direct_reader_value == "Rewritten":
            if chapter_accepted[chapter.title] == 0:
                issues.append(
                    f"{SCORECARD}: {chapter.title!r} cannot claim Rewritten direct-reader "
                    "evidence without an accepted inbox report for that chapter"
                )
            if chapter_rewritten[chapter.title] == 0:
                issues.append(
                    f"{SCORECARD}: {chapter.title!r} cannot claim Rewritten direct-reader "
                    "evidence without a rewritten inbox report for that chapter"
                )

        focus = row[8]
        if len(focus) < 24:
            issues.append(f"{SCORECARD}: {chapter.title!r} next rewrite focus is too short")

    if all(value == "Strong" for row in row_map.values() for value in row[1:7]):
        issues.append(f"{SCORECARD}: at least one chapter dimension should name remaining rewrite work")

    if "direct reader report" not in scorecard_text and "reader feedback" not in scorecard_text:
        issues.append(f"{SCORECARD}: scorecard should name direct reader evidence as a completion gap")

    return issues


def fixture_contract_text() -> str:
    rows = [
        f"| [{chapter.title}](src/{chapter.filename}) | Stable draft | question | bucket | code | run | practice |"
        for chapter in EXPECTED_CHAPTERS
    ]

    return "\n".join(
        [
            "# Fixture Contracts",
            "",
            "| Chapter | Maturity | Reference-map question | Source bucket | Code evidence | Run evidence | Practice evidence |",
            "| --- | --- | --- | --- | --- | --- | --- |",
            *rows,
        ]
    )


def fixture_scorecard_text(direct_reader_overrides: dict[str, str]) -> str:
    rows: list[str] = []

    for chapter in EXPECTED_CHAPTERS:
        quality_values = ["Strong", "Strong", "Strong", "Strong", "Strong", "Strong"]
        if chapter.title == "Transformer Roadmap":
            quality_values[3] = "Reader-needed"

        direct_reader_value = direct_reader_overrides.get(chapter.title, "Pending")
        rows.append(
            "| "
            + " | ".join(
                [
                    f"[{chapter.title}](src/{chapter.filename})",
                    *quality_values,
                    direct_reader_value,
                    f"Validate {chapter.title} with direct reader report evidence",
                ]
            )
            + " |"
        )

    return "\n".join(
        [
            "# Chapter Quality Scorecard",
            "",
            "A chapter can be source-backed and runnable while still needing direct reader evidence.",
            "",
            "| Chapter | Source grounding | Rust clarity | ML intuition | Category precision | Learner path | Practice transfer | Direct-reader evidence | Next rewrite focus |",
            "| --- | --- | --- | --- | --- | --- | --- | --- | --- |",
            *rows,
        ]
    )


def run_self_test() -> int:
    checker = load_inbox_checker()
    accepted_fixture = "# Fixture\n\n" + checker.fixture_report(
        "2026-05-12",
        "accepted scorecard evidence",
        88,
        "ML engineer",
        "fix now",
    )
    accepted_count, accepted_issues = accepted_direct_reports(accepted_fixture)

    if accepted_issues or accepted_count != 1:
        print("Chapter scorecard self-test failed: accepted fixture should count.")
        for issue in accepted_issues:
            print(f"- {issue}")
        print(f"- accepted_count={accepted_count}")
        return 1

    inconsistent_fixture = "# Fixture\n\n" + checker.fixture_report(
        "2026-05-12",
        "inconsistent scorecard evidence",
        89,
        "ML engineer",
        "new",
    )
    inconsistent_count, inconsistent_issues = accepted_direct_reports(inconsistent_fixture)

    if inconsistent_count != 0 or not inconsistent_issues:
        print("Chapter scorecard self-test failed: inconsistent fixture should fail.")
        print(f"- inconsistent_count={inconsistent_count}")
        for issue in inconsistent_issues:
            print(f"- {issue}")
        return 1

    accepted_by_chapter, rewritten_by_chapter, chapter_issues = chapter_report_counts(
        accepted_fixture
    )
    if (
        chapter_issues
        or accepted_by_chapter["The Tiny ML Pipeline"] != 1
        or accepted_by_chapter["Welcome"] != 0
    ):
        print("Chapter scorecard self-test failed: chapter-specific counts are wrong.")
        for issue in chapter_issues:
            print(f"- {issue}")
        print(f"- tiny_ml_pipeline={accepted_by_chapter['The Tiny ML Pipeline']}")
        print(f"- welcome={accepted_by_chapter['Welcome']}")
        return 1

    rewritten_fixture = "# Fixture\n\n" + checker.fixture_report(
        "2026-05-12",
        "rewritten scorecard evidence",
        90,
        "ML engineer",
        "rewritten",
    )
    _, rewritten_by_chapter, rewritten_issues = chapter_report_counts(rewritten_fixture)
    if rewritten_issues or rewritten_by_chapter["The Tiny ML Pipeline"] != 1:
        print("Chapter scorecard self-test failed: rewritten fixture should count by chapter.")
        for issue in rewritten_issues:
            print(f"- {issue}")
        print(f"- tiny_ml_pipeline_rewritten={rewritten_by_chapter['The Tiny ML Pipeline']}")
        return 1

    contract_fixture = fixture_contract_text()
    tiny_ml_accepted_scorecard = fixture_scorecard_text(
        {"The Tiny ML Pipeline": "Accepted"}
    )
    tiny_ml_accepted_issues = check_rows(
        scorecard_text=tiny_ml_accepted_scorecard,
        contract_text=contract_fixture,
        inbox_text=accepted_fixture,
    )
    if tiny_ml_accepted_issues:
        print("Chapter scorecard self-test failed: matching accepted chapter should pass.")
        for issue in tiny_ml_accepted_issues:
            print(f"- {issue}")
        return 1

    wrong_chapter_scorecard = fixture_scorecard_text({"Welcome": "Accepted"})
    wrong_chapter_issues = check_rows(
        scorecard_text=wrong_chapter_scorecard,
        contract_text=contract_fixture,
        inbox_text=accepted_fixture,
    )
    if not any("Welcome" in issue and "without an accepted inbox report" in issue for issue in wrong_chapter_issues):
        print("Chapter scorecard self-test failed: wrong chapter Accepted claim should fail.")
        for issue in wrong_chapter_issues:
            print(f"- {issue}")
        return 1

    not_rewritten_scorecard = fixture_scorecard_text(
        {"The Tiny ML Pipeline": "Rewritten"}
    )
    not_rewritten_issues = check_rows(
        scorecard_text=not_rewritten_scorecard,
        contract_text=contract_fixture,
        inbox_text=accepted_fixture,
    )
    if not any("without a rewritten inbox report" in issue for issue in not_rewritten_issues):
        print("Chapter scorecard self-test failed: Rewritten claim without rewritten evidence should fail.")
        for issue in not_rewritten_issues:
            print(f"- {issue}")
        return 1

    print("Chapter scorecard self-test passed.")
    return 0


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Check that the chapter quality scorecard remains usable for rewrites."
    )
    parser.add_argument(
        "--self-test",
        action="store_true",
        help="Run offline scorecard evidence fixtures without reading project files.",
    )
    return parser.parse_args()


def main() -> int:
    args = parse_args()

    if args.self_test:
        return run_self_test()

    scorecard_text = SCORECARD.read_text(encoding="utf-8")
    contract_text = CONTRACTS.read_text(encoding="utf-8")
    inbox_text = INBOX.read_text(encoding="utf-8")
    issues: list[str] = []

    issues.extend(check_required_sections(scorecard_text))
    issues.extend(check_rubric(scorecard_text))
    issues.extend(check_rows(scorecard_text, contract_text, inbox_text))

    if issues:
        print("Chapter scorecard check failed:")
        for issue in issues:
            print(f"- {issue}")
        return 1

    print("Chapter scorecard check passed.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
