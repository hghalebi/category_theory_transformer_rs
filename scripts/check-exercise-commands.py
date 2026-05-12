#!/usr/bin/env python3
"""Check that learner-facing exercise commands stay runnable and intentional."""

from __future__ import annotations

import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]

COMMAND_FILES = [
    ROOT / "book" / "src" / "exercises.md",
    ROOT / "exercises" / "README.md",
    ROOT / "exercises" / "beginner" / "README.md",
    ROOT / "exercises" / "intermediate" / "README.md",
    ROOT / "exercises" / "advanced" / "README.md",
    ROOT / "lessons" / "README.md",
    ROOT / "lessons" / "00-map.md",
    ROOT / "lessons" / "01-domain-objects.md",
    ROOT / "lessons" / "02-morphisms-composition.md",
    ROOT / "lessons" / "03-ml-pipeline.md",
    ROOT / "lessons" / "04-training-endomorphism.md",
    ROOT / "lessons" / "05-structure-and-calculus.md",
    ROOT / "lessons" / "06-seven-sketches.md",
]

ALLOWED_COMMANDS = {
    "bash scripts/check.sh",
    "cargo run --bin category_ml",
    "cargo run --example 01_token_sequence",
    "cargo run --example 01_domain_objects",
    "cargo run --example 02_morphism_composition",
    "cargo run --example 03_training_endomorphism",
    "cargo run --example 04_structure_and_calculus",
    "cargo run --example 05_seven_sketches",
    "cargo run --example 06_attention_scores",
    "cargo test --all-targets --all-features",
    "cargo test cross_entropy_is_lower_for_more_confident_target_probability --lib",
    "cargo test finite_difference --lib",
    "cargo test structure::tests --lib",
}

REQUIRED_COMMANDS = {
    "bash scripts/check.sh",
    "cargo test --all-targets --all-features",
    "cargo test cross_entropy_is_lower_for_more_confident_target_probability --lib",
    "cargo test finite_difference --lib",
    "cargo test structure::tests --lib",
    "cargo run --example 06_attention_scores",
}


def extract_bash_commands(path: Path) -> list[tuple[int, str]]:
    commands: list[tuple[int, str]] = []
    in_bash = False

    for line_number, line in enumerate(path.read_text(encoding="utf-8").splitlines(), start=1):
        stripped = line.strip()

        if stripped.startswith("```"):
            if in_bash:
                in_bash = False
            elif stripped == "```bash":
                in_bash = True
            continue

        if not in_bash or not stripped or stripped.startswith("#"):
            continue

        commands.append((line_number, stripped))

    return commands


def looks_like_multiple_cargo_test_filters(command: str) -> bool:
    parts = command.split()

    if len(parts) < 3 or parts[0:2] != ["cargo", "test"]:
        return False

    filters = [part for part in parts[2:] if not part.startswith("-")]

    return len(filters) > 1


def main() -> int:
    issues: list[str] = []
    seen_commands: set[str] = set()

    for path in COMMAND_FILES:
        if not path.exists():
            issues.append(f"{path}: command file does not exist")
            continue

        for line_number, command in extract_bash_commands(path):
            seen_commands.add(command)

            if command not in ALLOWED_COMMANDS:
                issues.append(
                    f"{path}:{line_number}: command is not in the exercise allowlist: {command!r}"
                )

            if looks_like_multiple_cargo_test_filters(command):
                issues.append(
                    f"{path}:{line_number}: cargo test should use one test-name filter: {command!r}"
                )

    for command in sorted(REQUIRED_COMMANDS):
        if command not in seen_commands:
            issues.append(f"required learner command is missing: {command!r}")

    if issues:
        print("Exercise command check failed:")
        for issue in issues:
            print(f"- {issue}")
        return 1

    print("Exercise command check passed.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
