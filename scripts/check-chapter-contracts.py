#!/usr/bin/env python3
"""Check that the editorial chapter contracts stay aligned."""

from __future__ import annotations

import re
import shlex
import sys
from dataclasses import dataclass
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
CONTRACTS = ROOT / "book" / "CHAPTER_CONTRACTS.md"
REFERENCES = ROOT / "book" / "src" / "references.md"
RESEARCH_NOTES = ROOT / "book" / "EDITORIAL_RESEARCH_NOTES.md"
README = ROOT / "README.md"
EXERCISES = ROOT / "book" / "src" / "exercises.md"


@dataclass(frozen=True)
class ChapterContract:
    title: str
    filename: str
    allowed_run_prefixes: tuple[str, ...]


EXPECTED_CONTRACTS = [
    ChapterContract("Welcome", "welcome.md", ("cargo run --example 01_token_sequence",)),
    ChapterContract("Course Map", "00-map.md", ("cargo run --bin category_ml",)),
    ChapterContract("Domain Objects", "01-domain-objects.md", ("cargo run --example 01_domain_objects",)),
    ChapterContract(
        "Morphism and Composition",
        "02-morphisms-composition.md",
        ("cargo run --example 02_morphism_composition",),
    ),
    ChapterContract("The Tiny ML Pipeline", "03-ml-pipeline.md", ("cargo test ml::tests --lib",)),
    ChapterContract(
        "Training as an Endomorphism",
        "04-training-endomorphism.md",
        ("cargo run --example 03_training_endomorphism",),
    ),
    ChapterContract(
        "Functors, Naturality, Monoids, and Chain Rule",
        "05-structure-and-calculus.md",
        ("cargo run --example 04_structure_and_calculus",),
    ),
    ChapterContract(
        "Seven Sketches Through Rust",
        "seven-sketches-rust.md",
        ("cargo run --example 05_seven_sketches",),
    ),
    ChapterContract(
        "Exercises",
        "exercises.md",
        ("python3 scripts/check-exercise-alignment.py",),
    ),
    ChapterContract(
        "Transformer Roadmap",
        "roadmap.md",
        ("cargo run --example 06_attention_scores",),
    ),
]

REQUIRED_SECTIONS = (
    "# Chapter Contracts",
    "Status: Active rewrite support",
    "## Contract Rule",
)


def parse_markdown_table_after_marker(text: str, marker: str) -> list[list[str]]:
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

    if rows and rows[0][0] in {"Chapter", "Requirement", "Label"}:
        return rows[1:]

    return rows


def chapter_link(title: str, filename: str, prefix: str = "src") -> str:
    return f"[{title}]({prefix}/{filename})"


def reference_row_map(reference_text: str) -> dict[str, str]:
    rows = parse_markdown_table_after_marker(reference_text, "## Chapter Reference Map")
    mapping: dict[str, str] = {}

    for row in rows:
        if len(row) >= 2:
            mapping[row[0]] = row[1]

    return mapping


def maturity_row_map(readme_text: str) -> dict[str, str]:
    rows = parse_markdown_table_after_marker(readme_text, "Chapter maturity:")
    mapping: dict[str, str] = {}

    for row in rows:
        if len(row) >= 2:
            mapping[row[0]] = row[1]

    return mapping


def practice_map_text(exercise_text: str) -> str:
    start = exercise_text.find("## Core Chapter Practice Map")
    if start == -1:
        return ""

    end = exercise_text.find("\n## ", start + len("## Core Chapter Practice Map"))
    return exercise_text[start:] if end == -1 else exercise_text[start:end]


def source_bucket_exists(research_text: str, bucket_name: str) -> bool:
    return f"### {bucket_name}" in research_text


def validate_paths(cell: str) -> list[str]:
    issues: list[str] = []

    for raw_path in re.findall(r"`([^`]+)`", cell):
        path = ROOT / raw_path
        if not path.exists():
            issues.append(f"{CONTRACTS}: contract references missing path {raw_path!r}")

    return issues


def validate_run_command(command: str, allowed_prefixes: tuple[str, ...]) -> bool:
    try:
        shlex.split(command)
    except ValueError:
        return False

    return command in allowed_prefixes


def check_required_sections(text: str) -> list[str]:
    return [f"{CONTRACTS}: missing section marker {section!r}" for section in REQUIRED_SECTIONS if section not in text]


def check_contract_rows(
    contract_text: str,
    reference_text: str,
    research_text: str,
    readme_text: str,
    exercise_text: str,
) -> list[str]:
    issues: list[str] = []
    rows = parse_markdown_table_after_marker(contract_text, "| Chapter | Maturity |")
    references = reference_row_map(reference_text)
    maturities = maturity_row_map(readme_text)
    practice_text = practice_map_text(exercise_text)
    row_map: dict[str, list[str]] = {}

    if not rows:
        return [f"{CONTRACTS}: missing chapter contract table"]

    for row in rows:
        if len(row) != 7:
            issues.append(f"{CONTRACTS}: contract row should have 7 columns: {row!r}")
            continue

        title_match = re.match(r"\[(?P<title>.+)\]\(src/(?P<filename>.+)\)", row[0])
        if title_match is None:
            issues.append(f"{CONTRACTS}: first cell should link to book/src chapter: {row[0]!r}")
            continue

        title = title_match.group("title")
        if title in row_map:
            issues.append(f"{CONTRACTS}: duplicate contract row for {title!r}")
        row_map[title] = row

    expected_titles = {contract.title for contract in EXPECTED_CONTRACTS}
    for title in sorted(set(row_map) - expected_titles):
        issues.append(f"{CONTRACTS}: unexpected chapter contract row {title!r}")

    for expected in EXPECTED_CONTRACTS:
        row = row_map.get(expected.title)
        if row is None:
            issues.append(f"{CONTRACTS}: missing chapter contract for {expected.title!r}")
            continue

        link, maturity, question, source_bucket, code_evidence, run_evidence, practice_evidence = row
        expected_link = chapter_link(expected.title, expected.filename)
        if link != expected_link:
            issues.append(f"{CONTRACTS}: {expected.title!r} link should be {expected_link!r}")

        readme_maturity = maturities.get(expected.title)
        if readme_maturity is None:
            issues.append(f"{README}: missing maturity row for {expected.title!r}")
        elif maturity != readme_maturity:
            issues.append(
                f"{CONTRACTS}: {expected.title!r} maturity {maturity!r} does not match "
                f"README maturity {readme_maturity!r}"
            )

        reference_link = chapter_link(expected.title, expected.filename, prefix="")
        reference_link = reference_link.replace("](/", "](")
        reference_question = references.get(reference_link)
        if reference_question is None:
            issues.append(f"{REFERENCES}: missing reference-map row for {expected.title!r}")
        elif question != reference_question:
            issues.append(
                f"{CONTRACTS}: {expected.title!r} question does not match reference map"
            )

        if not source_bucket_exists(research_text, source_bucket):
            issues.append(
                f"{CONTRACTS}: {expected.title!r} source bucket {source_bucket!r} "
                f"is missing in {RESEARCH_NOTES}"
            )

        issues.extend(validate_paths(code_evidence))

        if not validate_run_command(run_evidence.strip("`"), expected.allowed_run_prefixes):
            issues.append(
                f"{CONTRACTS}: {expected.title!r} run evidence {run_evidence!r} "
                f"should be one of {expected.allowed_run_prefixes!r}"
            )

        exercise_link = f"[{expected.title}]({expected.filename})"
        if expected.title != "Exercises" and exercise_link not in practice_text:
            issues.append(
                f"{EXERCISES}: core chapter practice map missing {exercise_link!r}"
            )

        if not practice_evidence or practice_evidence == "-":
            issues.append(f"{CONTRACTS}: {expected.title!r} has empty practice evidence")

    return issues


def main() -> int:
    contract_text = CONTRACTS.read_text(encoding="utf-8")
    reference_text = REFERENCES.read_text(encoding="utf-8")
    research_text = RESEARCH_NOTES.read_text(encoding="utf-8")
    readme_text = README.read_text(encoding="utf-8")
    exercise_text = EXERCISES.read_text(encoding="utf-8")
    issues: list[str] = []

    issues.extend(check_required_sections(contract_text))
    issues.extend(
        check_contract_rows(
            contract_text,
            reference_text,
            research_text,
            readme_text,
            exercise_text,
        )
    )

    if issues:
        print("Chapter contract check failed:")
        for issue in issues:
            print(f"- {issue}")
        return 1

    print("Chapter contract check passed.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
