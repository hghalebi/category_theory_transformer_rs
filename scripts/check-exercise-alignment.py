#!/usr/bin/env python3
"""Check that chapters, exercises, and answer keys stay aligned."""

from __future__ import annotations

import re
import sys
from dataclasses import dataclass
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
BOOK_EXERCISES = ROOT / "book" / "src" / "exercises.md"
ANSWER_KEY = ROOT / "exercises" / "ANSWER_KEY.md"


@dataclass(frozen=True)
class ChapterPracticeTarget:
    title: str
    filename: str


@dataclass(frozen=True)
class ExerciseSet:
    path: Path
    answer_prefix: str


CHAPTER_PRACTICE_TARGETS = [
    ChapterPracticeTarget("Welcome", "welcome.md"),
    ChapterPracticeTarget("Course Map", "00-map.md"),
    ChapterPracticeTarget("Domain Objects", "01-domain-objects.md"),
    ChapterPracticeTarget("Morphism and Composition", "02-morphisms-composition.md"),
    ChapterPracticeTarget("The Tiny ML Pipeline", "03-ml-pipeline.md"),
    ChapterPracticeTarget("Training as an Endomorphism", "04-training-endomorphism.md"),
    ChapterPracticeTarget("Functors, Naturality, Monoids, and Chain Rule", "05-structure-and-calculus.md"),
    ChapterPracticeTarget("Seven Sketches Through Rust", "seven-sketches-rust.md"),
    ChapterPracticeTarget("Transformer Roadmap", "roadmap.md"),
]

EXERCISE_SETS = [
    ExerciseSet(BOOK_EXERCISES, "Exercise"),
    ExerciseSet(ROOT / "exercises" / "beginner" / "README.md", "Beginner"),
    ExerciseSet(ROOT / "exercises" / "intermediate" / "README.md", "Intermediate"),
    ExerciseSet(ROOT / "exercises" / "advanced" / "README.md", "Advanced"),
]

EXERCISE_HEADING_RE = re.compile(r"^## Exercise (?P<number>\d+): (?P<title>.+)$", re.MULTILINE)


def check_chapter_practice_map(book_exercises_text: str) -> list[str]:
    issues: list[str] = []

    if "## Core Chapter Practice Map" not in book_exercises_text:
        issues.append(f"{BOOK_EXERCISES}: missing core chapter practice map")

    for target in CHAPTER_PRACTICE_TARGETS:
        link = f"[{target.title}]({target.filename})"
        if link not in book_exercises_text:
            issues.append(
                f"{BOOK_EXERCISES}: core chapter practice map missing {link}"
            )

    return issues


def expected_answer_heading(prefix: str, number: str, title: str) -> str:
    return f"### {prefix} {number}: {title}"


def check_answer_key_for_set(
    exercise_set: ExerciseSet, answer_key_text: str
) -> list[str]:
    issues: list[str] = []
    text = exercise_set.path.read_text(encoding="utf-8")
    headings = list(EXERCISE_HEADING_RE.finditer(text))

    if not headings:
        issues.append(f"{exercise_set.path}: no exercise headings found")
        return issues

    for heading in headings:
        number = heading.group("number")
        title = heading.group("title").strip()
        expected = expected_answer_heading(exercise_set.answer_prefix, number, title)

        if expected not in answer_key_text:
            issues.append(
                f"{ANSWER_KEY}: missing answer-key heading {expected!r} "
                f"for {exercise_set.path}"
            )

    return issues


def check_exercise_ladder(book_exercises_text: str) -> list[str]:
    issues: list[str] = []

    required_paths = [
        "`exercises/beginner/README.md`",
        "`exercises/intermediate/README.md`",
        "`exercises/advanced/README.md`",
        "`exercises/ANSWER_KEY.md`",
    ]

    for path in required_paths:
        if path not in book_exercises_text:
            issues.append(f"{BOOK_EXERCISES}: exercise ladder should mention {path}")

    return issues


def check_attempt_record(book_exercises_text: str, answer_key_text: str) -> list[str]:
    issues: list[str] = []

    required_book_markers = [
        "## Exercise Attempt Record",
        "Exercise:",
        "Chapter:",
        "Command run:",
        "First failure signal:",
        "Line or concept that caused confusion:",
        "What I expected:",
        "What happened instead:",
        "Answer-key mismatch:",
        "Suggested rewrite:",
    ]

    for marker in required_book_markers:
        if marker not in book_exercises_text:
            issues.append(f"{BOOK_EXERCISES}: missing attempt-record marker {marker!r}")

    required_answer_key_markers = [
        "## Using Attempt Reports",
        "first failure signal",
        "answer-key mismatch",
        "suggested rewrite",
    ]

    for marker in required_answer_key_markers:
        if marker not in answer_key_text:
            issues.append(f"{ANSWER_KEY}: missing attempt-report guidance marker {marker!r}")

    return issues


def main() -> int:
    book_exercises_text = BOOK_EXERCISES.read_text(encoding="utf-8")
    answer_key_text = ANSWER_KEY.read_text(encoding="utf-8")
    issues: list[str] = []

    issues.extend(check_chapter_practice_map(book_exercises_text))
    issues.extend(check_exercise_ladder(book_exercises_text))
    issues.extend(check_attempt_record(book_exercises_text, answer_key_text))

    for exercise_set in EXERCISE_SETS:
        issues.extend(check_answer_key_for_set(exercise_set, answer_key_text))

    if issues:
        print("Exercise alignment check failed:")
        for issue in issues:
            print(f"- {issue}")
        return 1

    print("Exercise alignment check passed.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
