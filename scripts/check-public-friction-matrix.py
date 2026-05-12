#!/usr/bin/env python3
"""Check that public learner-friction synthesis stays chapter-actionable."""

from __future__ import annotations

import re
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SYNTHESIS = ROOT / "community" / "external-feedback-synthesis.md"
AUDIT = ROOT / "book" / "TEXTBOOK_COMPLETION_AUDIT.md"
RESEARCH_NOTES = ROOT / "book" / "EDITORIAL_RESEARCH_NOTES.md"
REFERENCES = ROOT / "book" / "src" / "references.md"
WELCOME = ROOT / "book" / "src" / "welcome.md"
COURSE_MAP = ROOT / "book" / "src" / "00-map.md"
DOMAIN_OBJECTS = ROOT / "book" / "src" / "01-domain-objects.md"
MORPHISMS = ROOT / "book" / "src" / "02-morphisms-composition.md"
ML_PIPELINE = ROOT / "book" / "src" / "03-ml-pipeline.md"
TRAINING = ROOT / "book" / "src" / "04-training-endomorphism.md"
STRUCTURE = ROOT / "book" / "src" / "05-structure-and-calculus.md"
SEVEN_SKETCHES = ROOT / "book" / "src" / "seven-sketches-rust.md"
EXERCISES = ROOT / "book" / "src" / "exercises.md"
ROADMAP = ROOT / "book" / "src" / "roadmap.md"
GLOSSARY = ROOT / "book" / "src" / "glossary.md"

REQUIRED_SECTIONS = [
    "## Sources Reviewed",
    "## Friction Themes",
    "## Chapter Friction Matrix",
    "## Rewrite Checklist",
    "## Direct Feedback Still Needed",
]

REQUIRED_PUBLIC_SOURCES = [
    "https://rust-book.cs.brown.edu/",
    "https://rustlings.rust-lang.org/usage/",
    "https://rust-exercises.com/100-exercises/01_intro/00_welcome",
    "https://www.johndcook.com/blog/2020/11/02/category-theory-for-programmers/",
    "https://stackoverflow.com/questions/62838070/category-theory-to-computer-programming-do-objects-map-to-types-or-instances-o",
    "https://cs231n.github.io/optimization-1/",
    "https://cs231n.github.io/neural-networks-3/",
    "https://arxiv.org/abs/1802.01528",
    "https://jalammar.github.io/illustrated-transformer/",
]

REQUIRED_THEMES = [
    "### 1. Reading Alone Is Not Enough",
    "### 2. Learners Need Failure Signals",
    "### 3. Category Theory Should Start With Arrows",
    "### 4. Gradient Checking Needs Two Independent Views",
    "### 5. Transformer Explanations Need Shape And Role Separation",
]

REQUIRED_CHAPTER_ROWS = [
    "Welcome",
    "Course Map",
    "Domain Objects",
    "Morphism and Composition",
    "The Tiny ML Pipeline",
    "Training as an Endomorphism",
    "Functors, Naturality, Monoids, and Chain Rule",
    "Seven Sketches Through Rust",
    "Exercises",
    "Transformer Roadmap",
]

REQUIRED_AUDIT_MARKERS = [
    "community/external-feedback-synthesis.md",
    "scripts/check-public-friction-matrix.py",
    "public learner-friction",
]

REQUIRED_RESEARCH_MARKERS = [
    "Chapter Friction Matrix",
    "The Matrix Calculus You Need For Deep Learning",
    "Transformer explanations need shape and role separation",
]

REQUIRED_REFERENCE_MARKERS = [
    "The Matrix Calculus You Need For Deep Learning",
    "CS231n: Neural Networks Part 3",
    "The Illustrated Transformer",
]


def read(path: Path) -> str:
    return path.read_text(encoding="utf-8")


def matrix_rows(text: str) -> dict[str, list[str]]:
    rows: dict[str, list[str]] = {}
    in_matrix = False

    for line in text.splitlines():
        if line.strip() == "## Chapter Friction Matrix":
            in_matrix = True
            continue

        if in_matrix and line.startswith("## "):
            break

        stripped = line.strip()
        if not in_matrix or not stripped.startswith("|") or stripped.startswith("| ---"):
            continue

        cells = [cell.strip() for cell in stripped.strip("|").split("|")]
        if cells and cells[0] != "Chapter":
            rows[cells[0]] = cells

    return rows


def check_synthesis(text: str) -> list[str]:
    issues: list[str] = []

    for section in REQUIRED_SECTIONS:
        if section not in text:
            issues.append(f"{SYNTHESIS}: missing section {section!r}")

    for source in REQUIRED_PUBLIC_SOURCES:
        if source not in text:
            issues.append(f"{SYNTHESIS}: missing public source {source}")

    for theme in REQUIRED_THEMES:
        if theme not in text:
            issues.append(f"{SYNTHESIS}: missing friction theme {theme!r}")

    rows = matrix_rows(text)

    for chapter in REQUIRED_CHAPTER_ROWS:
        row = rows.get(chapter)
        if row is None:
            issues.append(f"{SYNTHESIS}: missing Chapter Friction Matrix row for {chapter!r}")
            continue

        if len(row) != 4:
            issues.append(f"{SYNTHESIS}: row for {chapter!r} should have 4 cells")
            continue

        _, friction, sources, review_check = row
        if not friction:
            issues.append(f"{SYNTHESIS}: row for {chapter!r} has empty friction cell")
        if len(re.findall(r"https?://", sources)) < 1:
            issues.append(f"{SYNTHESIS}: row for {chapter!r} should cite at least one URL")
        if "Does" not in review_check and "Can" not in review_check:
            issues.append(
                f"{SYNTHESIS}: row for {chapter!r} should phrase the review check as a question"
            )

    if "direct reader reports" not in text:
        issues.append(f"{SYNTHESIS}: should say proxy synthesis is not direct reader reports")

    return issues


def check_cross_links() -> list[str]:
    issues: list[str] = []
    audit = read(AUDIT)
    research_notes = read(RESEARCH_NOTES)
    references = read(REFERENCES)

    for marker in REQUIRED_AUDIT_MARKERS:
        if marker not in audit:
            issues.append(f"{AUDIT}: missing public-friction marker {marker!r}")

    for marker in REQUIRED_RESEARCH_MARKERS:
        if marker not in research_notes:
            issues.append(f"{RESEARCH_NOTES}: missing research marker {marker!r}")

    for marker in REQUIRED_REFERENCE_MARKERS:
        if marker not in references:
            issues.append(f"{REFERENCES}: missing reference marker {marker!r}")

    return issues


def check_welcome_first_win() -> list[str]:
    text = read(WELCOME)
    issues: list[str] = []

    if "## First Win" not in text:
        issues.append(f"{WELCOME}: missing early First Win section")

    first_win = text.find("## First Win")
    command = text.find("cargo run --example 01_token_sequence")
    book_about = text.find("## What This Book Is About")
    central_thesis = text.find("## The Central Thesis")

    if first_win == -1 or command == -1:
        return issues

    if book_about != -1 and not (first_win < command < book_about):
        issues.append(
            f"{WELCOME}: first runnable command should appear before the broader framing"
        )

    if central_thesis != -1 and command > central_thesis:
        issues.append(f"{WELCOME}: first runnable command should appear before the thesis")

    first_win_section = text[first_win : book_about if book_about != -1 else len(text)]
    for marker in ("Text", "-> TokenSequence", "-> TrainingPairs"):
        if marker not in first_win_section:
            issues.append(f"{WELCOME}: First Win section missing pipeline marker {marker!r}")

    return issues


def check_course_map_paths() -> list[str]:
    text = read(COURSE_MAP)
    issues: list[str] = []

    if "## Choose Your Path" not in text:
        issues.append(f"{COURSE_MAP}: missing Choose Your Path section")

    choose_path = text.find("## Choose Your Path")
    module_map = text.find("## Module Map")

    if choose_path == -1:
        return issues

    if module_map != -1 and choose_path > module_map:
        issues.append(f"{COURSE_MAP}: Choose Your Path should appear before Module Map")

    path_section = text[choose_path : module_map if module_map != -1 else len(text)]
    required_markers = [
        "book-first path",
        "code-first path",
        "Welcome",
        "Course Map",
        "Domain Objects",
        "Morphism and Composition",
        "Tiny ML Pipeline",
        "Training as an Endomorphism",
        "cargo run --example 01_token_sequence",
        "cargo run --bin category_ml",
        "domain value",
        "typed transformation",
        "training update",
    ]

    for marker in required_markers:
        if marker not in path_section:
            issues.append(f"{COURSE_MAP}: Choose Your Path missing marker {marker!r}")

    return issues


def check_domain_object_mistakes() -> list[str]:
    text = read(DOMAIN_OBJECTS)
    issues: list[str] = []

    if "## Mistakes These Types Prevent" not in text:
        issues.append(f"{DOMAIN_OBJECTS}: missing mistake-prevention section")

    mistake_section_start = text.find("## Mistakes These Types Prevent")
    source_snapshot = text.find("## Source Snapshot")

    if mistake_section_start == -1:
        return issues

    if source_snapshot != -1 and mistake_section_start > source_snapshot:
        issues.append(
            f"{DOMAIN_OBJECTS}: mistake-prevention section should appear before Source Snapshot"
        )

    mistake_section = text[
        mistake_section_start : source_snapshot if source_snapshot != -1 else len(text)
    ]

    required_rows = {
        "`TokenId`": "vocabulary index",
        "`TokenSequence`": "empty sequence",
        "`Vector`": "hidden features",
        "`Logits`": "raw scores",
        "`Distribution`": "non-normalized probabilities",
        "`Loss`": "non-finite objective value",
        "`VocabSize`": "zero-token vocabulary",
        "`ModelDimension`": "zero width",
        "`LearningRate`": "negative",
        "`TrainingSet`": "no examples",
        "`Parameters`": "model state",
    }

    for domain_type, mistake_marker in required_rows.items():
        if domain_type not in mistake_section:
            issues.append(
                f"{DOMAIN_OBJECTS}: mistake-prevention section missing {domain_type}"
            )
        if mistake_marker not in mistake_section:
            issues.append(
                f"{DOMAIN_OBJECTS}: mistake-prevention section missing marker "
                f"{mistake_marker!r}"
            )

    return issues


def check_morphism_term_bridge() -> list[str]:
    text = read(MORPHISMS)
    issues: list[str] = []

    if "## Category Terms As Rust Shapes" not in text:
        issues.append(f"{MORPHISMS}: missing term-to-Rust-shape bridge")

    bridge_start = text.find("## Category Terms As Rust Shapes")
    source_snapshot = text.find("## Source Snapshot")

    if bridge_start == -1:
        return issues

    if source_snapshot != -1 and bridge_start > source_snapshot:
        issues.append(
            f"{MORPHISMS}: term-to-Rust-shape bridge should appear before Source Snapshot"
        )

    bridge = text[bridge_start : source_snapshot if source_snapshot != -1 else len(text)]
    required_rows = {
        "Object": "`TokenId`, `Vector`, `Logits`, `Distribution`",
        "Morphism": "impl Morphism<TokenId, Vector> for Embedding",
        "Source object": "The `Input` type parameter",
        "Target object": "The `Output` type parameter",
        "Identity morphism": "`Identity<T>`",
        "Composition": "`Compose<F, G, Middle>`",
        "Middle object": "`Vector` between embedding and projection",
        "Endomorphism": "`TrainStep : Parameters -> Parameters`",
        "Repeated endomorphism": "`apply_endomorphism_n_times`",
    }

    for term, rust_marker in required_rows.items():
        if term not in bridge:
            issues.append(f"{MORPHISMS}: term bridge missing {term!r}")
        if rust_marker not in bridge:
            issues.append(f"{MORPHISMS}: term bridge missing marker {rust_marker!r}")

    return issues


def check_ml_pipeline_trace() -> list[str]:
    text = read(ML_PIPELINE)
    issues: list[str] = []

    if "## Prediction Trace Before Source" not in text:
        issues.append(f"{ML_PIPELINE}: missing prediction trace before source section")

    trace_start = text.find("## Prediction Trace Before Source")
    source_snapshot = text.find("## Source Snapshot")

    if trace_start == -1:
        return issues

    if source_snapshot != -1 and trace_start > source_snapshot:
        issues.append(
            f"{ML_PIPELINE}: prediction trace should appear before Source Snapshot"
        )

    trace = text[trace_start : source_snapshot if source_snapshot != -1 else len(text)]
    required_markers = [
        "`TokenId`",
        "`Vector`",
        "`Logits`",
        "`Distribution`",
        "`Product<Distribution, TokenId>`",
        "`Loss`",
        "raw scores, not probabilities",
        "Do probabilities sum to one?",
        "Which index is the target token?",
        "cross entropy use the target probability",
        "loss = -ln(probability assigned to target)",
        "Logits\n  -> Distribution\n  -> target probability\n  -> Loss",
    ]

    for marker in required_markers:
        if marker not in trace:
            issues.append(f"{ML_PIPELINE}: prediction trace missing marker {marker!r}")

    return issues


def check_training_update_trace() -> list[str]:
    text = read(TRAINING)
    issues: list[str] = []

    if "## Update Trace Before Source" not in text:
        issues.append(f"{TRAINING}: missing update trace before source section")

    trace_start = text.find("## Update Trace Before Source")
    source_snapshot = text.find("## Source Snapshot")

    if trace_start == -1:
        return issues

    if source_snapshot != -1 and trace_start > source_snapshot:
        issues.append(f"{TRAINING}: update trace should appear before Source Snapshot")

    trace = text[trace_start : source_snapshot if source_snapshot != -1 else len(text)]
    required_markers = [
        "`Parameters`",
        "`TrainingSet`",
        "`TokenId -> Vector -> Logits -> Distribution`",
        "`dlogits[target_id] -= 1.0`",
        "`grad_embedding`, `grad_lm_head`, `grad_bias`",
        "`batch_scale` and `LearningRate`",
        "Parameters\n  -> predictions on TrainingSet\n  -> gradients\n  -> Parameters",
        "Parameters0 -> Parameters1 -> Parameters2 -> ... -> ParametersN",
        "`Parameters -> Loss` measures the model",
        "`Parameters -> Parameters` updates the model",
        "parameter = parameter - learning_rate * average_gradient",
        "*value -= learning_rate * grad * batch_scale;",
        "`apply_endomorphism_n_times`",
        "one full-batch update",
    ]

    for marker in required_markers:
        if marker not in trace:
            issues.append(f"{TRAINING}: update trace missing marker {marker!r}")

    return issues


def check_structure_two_paths() -> list[str]:
    text = read(STRUCTURE)
    issues: list[str] = []

    if "## Trace Both Paths Before The Names" not in text:
        issues.append(f"{STRUCTURE}: missing two-path trace section")

    trace_start = text.find("## Trace Both Paths Before The Names")
    source_snapshot = text.find("## Source Snapshots")

    if trace_start == -1:
        return issues

    if source_snapshot != -1 and trace_start > source_snapshot:
        issues.append(f"{STRUCTURE}: two-path trace should appear before Source Snapshots")

    trace = text[trace_start : source_snapshot if source_snapshot != -1 else len(text)]
    required_markers = [
        "Path 1:\nVec<A> --map f--> Vec<B> --first--> Option<B>",
        "Path 2:\nVec<A> --first--> Option<A> --map f--> Option<B>",
        "same `Option<B>`",
        "`naturality_square_holds_for_first_option`",
        "(embedding <> linear) <> softmax",
        "embedding <> (linear <> softmax)",
        "same `PipelineTrace`",
        "Forward:\nx, y -> z = x * y -> L",
        "Backward:\ndL/dz -> dL/dx and dL/dy",
        "dz/dx = y",
        "dz/dy = x",
        "dL/dx = dL/dz * y",
        "dL/dy = dL/dz * x",
        "same result by two structural paths",
        "same gradient signal carried through local paths",
    ]

    for marker in required_markers:
        if marker not in trace:
            issues.append(f"{STRUCTURE}: two-path trace missing marker {marker!r}")

    return issues


def section_between(text: str, start_heading: str, end_headings: list[str]) -> str:
    start = text.find(start_heading)
    if start == -1:
        return ""

    end_positions = [text.find(heading, start + len(start_heading)) for heading in end_headings]
    end_positions = [position for position in end_positions if position != -1]
    end = min(end_positions) if end_positions else len(text)

    return text[start:end]


def check_seven_sketches_transfer_tasks() -> list[str]:
    text = read(SEVEN_SKETCHES)
    issues: list[str] = []

    required_tasks = [
        (
            "## Sketch 1: Information Order",
            ["## Sketch 1 Continued: Feature And Layer Galois Law"],
            "### Transfer Task: Ordered States",
            ["Draft can flow to Published", "Published cannot flow to Draft"],
        ),
        (
            "## Sketch 1 Continued: Feature And Layer Galois Law",
            ["## Sketch 2: Resources"],
            "### Transfer Task: Concrete And Abstract Capacity",
            ["abstract(concrete) -> abstract budget", "concrete fits concretize(budget)"],
        ),
        (
            "## Sketch 2: Resources",
            ["## Sketch 3: Database Instance"],
            "### Transfer Task: Resource Bundle",
            ["compute\nmemory\ndisk", "componentwise addition"],
        ),
        (
            "## Sketch 3: Database Instance",
            ["## Sketch 4: Co-Design Feasibility"],
            "### Transfer Task: Foreign Key Boundary",
            ["Task -> Project", "TaskRecord { id: TaskId, project: ProjectId }"],
        ),
        (
            "## Sketch 4: Co-Design Feasibility",
            ["## Sketch 5: Signal Matrices"],
            "### Transfer Task: Feasibility Relation",
            ["Requirement x Offer -> Bool", "many offers may satisfy one"],
        ),
        (
            "## Sketch 5: Signal Matrices",
            ["## Sketch 6: Open Circuits"],
            "### Transfer Task: Shape-Safe Composition",
            ["A -> B\n+B -> C", "A -> B\n+D -> C", "middle dimension"],
        ),
        (
            "## Sketch 6: Open Circuits",
            ["## Sketch 7: Logic Of Behavior"],
            "### Transfer Task: Interface Boundary",
            ["Tokenizer: Text -> TokenSequence", "which output boundary failed to match"],
        ),
        (
            "## Sketch 7: Logic Of Behavior",
            ["## Tests As Exercise Solutions"],
            "### Transfer Task: Local Checks To Global Claim",
            ["loss is finite", "one false local check"],
        ),
    ]

    for sketch_heading, end_headings, task_heading, markers in required_tasks:
        section = section_between(text, sketch_heading, end_headings)
        if not section:
            issues.append(f"{SEVEN_SKETCHES}: missing sketch section {sketch_heading!r}")
            continue

        if task_heading not in section:
            issues.append(f"{SEVEN_SKETCHES}: {sketch_heading} missing {task_heading!r}")

        for marker in markers:
            normalized_marker = marker.replace("\n+", "\n")
            if normalized_marker not in section:
                issues.append(
                    f"{SEVEN_SKETCHES}: {task_heading} missing marker "
                    f"{normalized_marker!r}"
                )

    return issues


def check_exercise_evidence_map() -> list[str]:
    text = read(EXERCISES)
    issues: list[str] = []

    if "## Exercise Evidence Map" not in text:
        issues.append(f"{EXERCISES}: missing exercise evidence map")

    evidence_start = text.find("## Exercise Evidence Map")
    worked_example = text.find("## Worked Example")

    if evidence_start == -1:
        return issues

    if worked_example != -1 and evidence_start > worked_example:
        issues.append(f"{EXERCISES}: evidence map should appear before Worked Example")

    evidence = text[evidence_start : worked_example if worked_example != -1 else len(text)]

    for exercise_number in range(1, 15):
        marker = f"Exercise {exercise_number}"
        if marker not in evidence:
            issues.append(f"{EXERCISES}: evidence map missing {marker}")

    required_markers = [
        "`cargo run --bin category_ml`",
        "compiler reports a missing trait bound or middle object",
        "`cargo run --example 03_training_endomorphism`",
        "`Err(...)` is connected to the invalid value",
        "`cargo run --example 05_seven_sketches` or a negative test",
        "`cargo run --example 06_attention_scores`",
        "`cargo test cross_entropy_is_lower_for_more_confident_target_probability --lib`",
        "`cargo test structure::tests --lib`",
        "command output, a constructor error, a compiler error, or a named test",
    ]

    for marker in required_markers:
        if marker not in evidence:
            issues.append(f"{EXERCISES}: evidence map missing marker {marker!r}")

    return issues


def check_transformer_role_ownership_map() -> list[str]:
    text = read(ROADMAP)
    issues: list[str] = []

    if "## Transformer Role Ownership Map" not in text:
        issues.append(f"{ROADMAP}: missing Transformer role ownership map")

    map_start = text.find("## Transformer Role Ownership Map")
    status_table = text.find("## What Exists Now")

    if map_start == -1:
        return issues

    if status_table != -1 and map_start > status_table:
        issues.append(f"{ROADMAP}: role ownership map should appear before What Exists Now")

    role_map = text[map_start : status_table if status_table != -1 else len(text)]
    required_markers = [
        "`HiddenSequence`",
        "`QuerySequence` and `HiddenToQuery`",
        "`KeySequence` and `HiddenToKey`",
        "`ValueSequence` and `HiddenToValue`",
        "`AttentionScores`",
        "`AttentionMask`",
        "`AttentionWeights`",
        "`AttentionOutput`",
        "`AttentionHeadOutputs` and `MultiHeadOutput`",
        "`ProjectedAttentionOutput`",
        "`ResidualConnection`",
        "`LayerNormalization`",
        "`PositionWiseFeedForward`",
        "`MaskedMultiHeadTransformerBlock`",
        "`TransformerReadout` and `SequenceLogits`",
        "`TransformerTrainingState`",
        "treating unnormalized scores as probabilities",
        "allowing illegal positions into softmax",
        "forgetting that each query row is a distribution over source positions",
        "Which object owns this role?",
        "Which boundary produces it?",
        "Which invalid connection should fail?",
    ]

    for marker in required_markers:
        if marker not in role_map:
            issues.append(f"{ROADMAP}: role ownership map missing marker {marker!r}")

    return issues


def check_transformer_category_shape_diagnostic() -> list[str]:
    text = read(ROADMAP)
    issues: list[str] = []

    if "## Category Shape Diagnostic" not in text:
        issues.append(f"{ROADMAP}: missing Category Shape Diagnostic")

    diagnostic_start = text.find("## Category Shape Diagnostic")
    retrieval_practice = text.find("## Retrieval Practice")

    if diagnostic_start == -1:
        return issues

    if retrieval_practice != -1 and diagnostic_start > retrieval_practice:
        issues.append(f"{ROADMAP}: Category Shape Diagnostic should appear before Retrieval Practice")

    diagnostic = text[
        diagnostic_start : retrieval_practice if retrieval_practice != -1 else len(text)
    ]
    required_markers = [
        "How many inputs does this boundary require?",
        "Does it return the same public object, or a different object?",
        "Research on self-attention as a parametric endofunctor is useful",
        "linear portions of self-attention",
        "Softmax, masking, residual addition, normalization",
        "Do not call the whole block an endofunctor",
        "Use this naming rule before reading the table:",
        "If there is one input, compare the input object and output object.",
        "If there is a product input, keep the product in the name.",
        "Returning the left object is not enough to make a",
        "`A -> A`",
        "`A x B -> A`",
        "`A x B -> A` with the wrong `B`",
        "`QuerySequence x KeySequence -> AttentionScores`",
        "product-input morphism",
        "`AttentionScores x AttentionMask -> AttentionScores`",
        "calling it a pure endomorphism on scores",
        "`AttentionScores -> AttentionWeights`",
        "ordinary morphism",
        "`HiddenSequence x ProjectedAttentionOutput -> HiddenSequence`",
        "calling the binary residual operation a unary endomorphism",
        "`LayerNormalization : HiddenSequence -> HiddenSequence`",
        "shape-preserving endomorphism",
        "`TransformerTrainingState -> TransformerTrainingState`",
        "state endomorphism",
        "`HiddenSequence x MultiHeadOutput -> HiddenSequence`",
        "not a legal composed boundary",
        "is this a unary morphism, a product-input morphism, an endomorphism, or not",
    ]

    for marker in required_markers:
        if marker not in diagnostic:
            issues.append(
                f"{ROADMAP}: Category Shape Diagnostic missing marker {marker!r}"
            )

    return issues


def check_transformer_self_cross_attention_boundary() -> list[str]:
    text = read(ROADMAP)
    issues: list[str] = []

    if "## Self-Attention And Cross-Attention Boundary" not in text:
        issues.append(f"{ROADMAP}: missing self-attention and cross-attention boundary")

    section_start = text.find("## Self-Attention And Cross-Attention Boundary")
    ml_concept = text.find("## ML Concept", section_start)

    if section_start == -1:
        return issues

    if ml_concept == -1:
        issues.append(f"{ROADMAP}: self/cross attention section should appear before ML Concept")
        section = text[section_start:]
    else:
        section = text[section_start:ml_concept]

    required_markers = [
        "self-attention: query, key, and value roles all",
        "PyTorch's multi-head attention API accepts `query`, `key`, and `value` as separate inputs",
        "target sequence length `L`",
        "source sequence length `S`",
        "Dive into Deep Learning",
        "`n` queries and `m` key-value pairs",
        "Target positions x Source positions -> attention weights",
        "QuerySequence x KeySequence -> AttentionScores",
        "AttentionWeights x ValueSequence -> AttentionOutput",
        "HiddenSequence -> QuerySequence",
        "HiddenSequence -> KeySequence",
        "HiddenSequence -> ValueSequence",
        "These are parallel projections, not a pipeline where queries turn into keys",
        "The shared source is what makes the case",
        "TargetHiddenSequence -> QuerySequence",
        "SourceHiddenSequence -> KeySequence",
        "SourceHiddenSequence -> ValueSequence",
        "same source for Q, K, V  -> self-attention case",
        "separate query and key-value sources -> cross-attention case",
        "Cross-attention makes that product input impossible to ignore",
        "attention mask of shape `L x S`",
        "for each target position, which source positions may be read?",
    ]

    normalized_section = " ".join(section.split())

    for marker in required_markers:
        normalized_marker = " ".join(marker.split())
        if normalized_marker not in normalized_section:
            issues.append(
                f"{ROADMAP}: self/cross attention boundary missing marker {marker!r}"
            )

    return issues


def check_glossary_attention_term_coherence() -> list[str]:
    text = read(GLOSSARY)
    issues: list[str] = []

    required_markers = [
        "| target sequence length | `QuerySequence` row count |",
        "| source sequence length | `KeySequence` and `ValueSequence` row count |",
        "| self-attention | `SelfAttentionHead`, `MultiHeadTransformerBlock` |",
        "| cross-attention | `QuerySequence`, `KeySequence`, `ValueSequence` |",
        "| product-input morphism | `Product<A, B>` at the input boundary |",
        "## Product-Input Morphism",
        "ScaledDotProductScores : QuerySequence x KeySequence -> AttentionScores",
        "WeightedValueMixing : AttentionWeights x ValueSequence -> AttentionOutput",
        "A x B -> C",
        "Do not erase the product",
        "## Target And Source Sequence Length",
        "The target sequence length is the number of query positions",
        "The source sequence length is the number of key-value positions",
        "Target positions x Source positions -> attention weights",
        "A mask of shape `L x S`",
        "## Self-Attention",
        "query, key, and value roles all come from the same",
        "Self-attention is not permission to call every internal step an endomorphism",
        "## Cross-Attention",
        "target-side query sequence reads from a separate",
        "does not yet implement a full cross-attention block",
        "TargetHiddenSequence -> QuerySequence",
        "SourceHiddenSequence -> KeySequence",
        "SourceHiddenSequence -> ValueSequence",
        "keep `L` and `S` separate",
    ]

    normalized_text = " ".join(text.split())

    for marker in required_markers:
        normalized_marker = " ".join(marker.split())
        if normalized_marker not in normalized_text:
            issues.append(
                f"{GLOSSARY}: attention glossary missing marker {marker!r}"
            )

    return issues


def main() -> int:
    issues: list[str] = []
    issues.extend(check_synthesis(read(SYNTHESIS)))
    issues.extend(check_cross_links())
    issues.extend(check_welcome_first_win())
    issues.extend(check_course_map_paths())
    issues.extend(check_domain_object_mistakes())
    issues.extend(check_morphism_term_bridge())
    issues.extend(check_ml_pipeline_trace())
    issues.extend(check_training_update_trace())
    issues.extend(check_structure_two_paths())
    issues.extend(check_seven_sketches_transfer_tasks())
    issues.extend(check_exercise_evidence_map())
    issues.extend(check_transformer_role_ownership_map())
    issues.extend(check_transformer_self_cross_attention_boundary())
    issues.extend(check_glossary_attention_term_coherence())
    issues.extend(check_transformer_category_shape_diagnostic())

    if issues:
        print("Public friction matrix check failed:")
        for issue in issues:
            print(f"- {issue}")
        return 1

    print("Public friction matrix check passed.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
