#!/usr/bin/env python3
"""Check that core learner-facing diagrams and traces stay present."""

from __future__ import annotations

import sys
from dataclasses import dataclass
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]


@dataclass(frozen=True)
class DiagramSpec:
    path: Path
    label: str
    markers: tuple[str, ...]


DIAGRAM_SPECS = [
    DiagramSpec(
        path=ROOT / "README.md",
        label="README Mermaid pipeline",
        markers=(
            "```mermaid",
            "flowchart LR",
            "A[Text] --> B[TokenSequence]",
            "B --> C[TrainingPairs]",
            "C --> D[ModelState]",
            "E --> F[Loss]",
            "F --> G[Updated ModelState]",
        ),
    ),
    DiagramSpec(
        path=ROOT / "book" / "src" / "00-map.md",
        label="Course Map whole-pipeline diagram",
        markers=(
            "## The Whole Pipeline",
            "Text\n  -> TokenSequence",
            "TokenSequence --DatasetWindowing--> TrainingSet",
            "TokenId --Embedding--> Vector --LinearToLogits--> Logits --Softmax--> Distribution",
            "Parameters --TrainStep--> Updated Parameters",
        ),
    ),
    DiagramSpec(
        path=ROOT / "book" / "src" / "02-morphisms-composition.md",
        label="Morphism legal-composition diagram",
        markers=(
            "The legal diagram is:",
            "TokenId",
            "Embedding",
            "Vector",
            "LinearToLogits",
            "Logits",
            "Softmax",
            "Distribution",
            "A diagram that skips `LinearToLogits`",
        ),
    ),
    DiagramSpec(
        path=ROOT / "book" / "src" / "03-ml-pipeline.md",
        label="Tiny ML data-preparation and prediction diagram",
        markers=(
            "full data-preparation and prediction diagram",
            "TokenSequence",
            "Product<TokenId, TokenId>",
            "Distribution ---------------- Product",
            "CrossEntropy",
            "Loss",
        ),
    ),
    DiagramSpec(
        path=ROOT / "book" / "src" / "04-training-endomorphism.md",
        label="Training parameter-update loop diagram",
        markers=(
            "The update can be read as a loop around the same object:",
            "Parameters_t",
            "Average Loss",
            "Gradient Accumulators",
            "Parameters_{t+1}",
        ),
    ),
    DiagramSpec(
        path=ROOT / "book" / "src" / "05-structure-and-calculus.md",
        label="Structure naturality and monoid diagrams",
        markers=(
            "The same square as a data-flow diagram:",
            "VecFunctor::fmap x10",
            "VecToFirstOption::transform",
            "The associativity check can be read as a grouping diagram:",
            "(embedding + linear) + softmax",
            "embedding + (linear + softmax)",
        ),
    ),
    DiagramSpec(
        path=ROOT / "book" / "src" / "roadmap.md",
        label="Transformer attention shape-flow diagram",
        markers=(
            "Read the current roadmap code through this shape trace:",
            "```mermaid",
            'H["HiddenSequence"] --> Q["QuerySequence"]',
            'S --> M["Masked Scores"]',
            'M --> W["AttentionWeights"]',
            'O --> MH["MultiHeadOutput"]',
            'R --> N["Normalized HiddenSequence"]',
            'N --> FF["FeedForward HiddenSequence"]',
        ),
    ),
]


def check_spec(spec: DiagramSpec) -> list[str]:
    text = spec.path.read_text(encoding="utf-8")
    issues: list[str] = []

    for marker in spec.markers:
        if marker not in text:
            issues.append(f"{spec.path}: {spec.label} missing marker {marker!r}")

    return issues


def main() -> int:
    issues: list[str] = []

    for spec in DIAGRAM_SPECS:
        issues.extend(check_spec(spec))

    if issues:
        print("Diagram coverage check failed:")
        for issue in issues:
            print(f"- {issue}")
        return 1

    print("Diagram coverage check passed.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
