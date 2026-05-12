#!/usr/bin/env python3
"""Check that source authority and learner-friction signals stay separated."""

from __future__ import annotations

import re
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
REFERENCES = ROOT / "book" / "src" / "references.md"
RESEARCH_NOTES = ROOT / "book" / "EDITORIAL_RESEARCH_NOTES.md"
FRICTION_SYNTHESIS = ROOT / "community" / "external-feedback-synthesis.md"

COMMUNITY_DOMAINS = (
    "reddit.com",
    "stackoverflow.com",
)

REQUIRED_AUTHORITY_MARKERS = (
    "## Source Priority",
    "Official Rust documentation",
    "Peer-reviewed or academic papers",
    "Established open textbooks and university course material",
    "Community posts as friction signals only",
    "authority for definitions, laws, or ML correctness",
)

REQUIRED_REFERENCE_MARKERS = (
    "https://doc.rust-lang.org/book/",
    "https://doc.rust-lang.org/rust-by-example/",
    "https://rust-lang.github.io/api-guidelines/checklist.html",
    "https://arxiv.org/abs/1803.05316",
    "https://arxiv.org/abs/1706.03762",
    "https://arxiv.org/abs/2501.02931",
    "https://d2l.ai/",
    "https://cs231n.github.io/",
    "https://docs.pytorch.org/docs/stable/",
    "https://huggingface.co/docs/",
    "https://www.nationalacademies.org/",
    "https://doi.org/",
)

REQUIRED_FRICTION_MARKERS = (
    "public learner-friction signals",
    "not a substitute for direct feedback on this project",
    "proxy checklist",
    "direct reader reports",
)


def check_https_only(path: Path, text: str) -> list[str]:
    issues: list[str] = []

    for match in re.finditer(r"http://[^\s)>\]]+", text):
        issues.append(f"{path}: use https for source URL {match.group(0)!r}")

    return issues


def check_references(text: str) -> list[str]:
    issues: list[str] = []
    normalized = text.lower()

    for domain in COMMUNITY_DOMAINS:
        if domain in normalized:
            issues.append(
                f"{REFERENCES}: community source {domain!r} belongs in "
                "community/external-feedback-synthesis.md, not the authoritative references"
            )

    for marker in REQUIRED_REFERENCE_MARKERS:
        if marker not in text:
            issues.append(f"{REFERENCES}: missing authoritative source marker {marker!r}")

    return issues


def check_research_notes(text: str) -> list[str]:
    issues: list[str] = []

    for marker in REQUIRED_AUTHORITY_MARKERS:
        if marker not in text:
            issues.append(f"{RESEARCH_NOTES}: missing source-authority marker {marker!r}")

    friction_start = text.find("## Learner Friction Signals")
    if friction_start == -1:
        issues.append(f"{RESEARCH_NOTES}: missing Learner Friction Signals section")
        return issues

    before_friction = text[:friction_start].lower()
    for domain in COMMUNITY_DOMAINS:
        if domain in before_friction:
            issues.append(
                f"{RESEARCH_NOTES}: community source {domain!r} appears before "
                "the learner-friction section"
            )

    return issues


def check_friction_synthesis(text: str) -> list[str]:
    issues: list[str] = []

    for marker in REQUIRED_FRICTION_MARKERS:
        if marker not in text:
            issues.append(f"{FRICTION_SYNTHESIS}: missing friction-boundary marker {marker!r}")

    normalized = text.lower()
    if not any(domain in normalized for domain in COMMUNITY_DOMAINS):
        issues.append(
            f"{FRICTION_SYNTHESIS}: expected community learner-friction sources "
            "to live here, not in the authoritative references"
        )

    return issues


def main() -> int:
    references = REFERENCES.read_text(encoding="utf-8")
    research_notes = RESEARCH_NOTES.read_text(encoding="utf-8")
    friction_synthesis = FRICTION_SYNTHESIS.read_text(encoding="utf-8")
    issues: list[str] = []

    for path, text in (
        (REFERENCES, references),
        (RESEARCH_NOTES, research_notes),
        (FRICTION_SYNTHESIS, friction_synthesis),
    ):
        issues.extend(check_https_only(path, text))

    issues.extend(check_references(references))
    issues.extend(check_research_notes(research_notes))
    issues.extend(check_friction_synthesis(friction_synthesis))

    if issues:
        print("Source authority check failed:")
        for issue in issues:
            print(f"- {issue}")
        return 1

    print("Source authority check passed.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
