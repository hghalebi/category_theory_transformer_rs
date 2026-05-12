#!/usr/bin/env python3
"""Check that the reader feedback loop stays connected and actionable."""

from __future__ import annotations

import re
import sys
from pathlib import Path
from urllib.parse import parse_qs, urlparse


ROOT = Path(__file__).resolve().parents[1]

README = ROOT / "README.md"
START_HERE = ROOT / "START_HERE.md"
TRIAGE = ROOT / "community" / "reader-feedback-triage.md"
REVIEW_GUIDE = ROOT / "community" / "reader-review-guide.md"
REVIEW_PACKET = ROOT / "community" / "reader-review-packet.md"
REVIEWER_OUTREACH = ROOT / "community" / "reviewer-outreach.md"
REVIEW_SPRINT = ROOT / "community" / "reader-review-sprint.md"
REVIEWER_SLOT_TRACKER = ROOT / "community" / "reviewer-slot-tracker.md"
FEEDBACK_INBOX = ROOT / "community" / "reader-feedback-inbox.md"
FEEDBACK_WALL = ROOT / "community" / "feedback-wall.md"
STARTER_ISSUES = ROOT / "community" / "starter-issues.md"
ISSUE_FORM = ROOT / ".github" / "ISSUE_TEMPLATE" / "reader-confusion.yml"
ISSUE_CONFIG = ROOT / ".github" / "ISSUE_TEMPLATE" / "config.yml"
COMPLETION_AUDIT = ROOT / "book" / "TEXTBOOK_COMPLETION_AUDIT.md"

TRIAGE_LINK = "community/reader-feedback-triage.md"
REVIEW_PACKET_LINK = "community/reader-review-packet.md"
REVIEW_GUIDE_LINK = "community/reader-review-guide.md"
EXTERNAL_SYNTHESIS_LINK = "community/external-feedback-synthesis.md"
REVIEWER_OUTREACH_LINK = "community/reviewer-outreach.md"
REVIEW_SPRINT_LINK = "community/reader-review-sprint.md"
REVIEWER_SLOT_TRACKER_LINK = "community/reviewer-slot-tracker.md"
REVIEW_SPRINT_ISSUE_URL = (
    "https://github.com/hghalebi/category_theory_transformer_rs/issues/6"
)
READER_CONFUSION_FORM_URL = (
    "https://github.com/hghalebi/category_theory_transformer_rs/"
    "issues/new?template=reader-confusion.yml"
)
PUBLIC_BOOK_URL = "https://hghalebi.github.io/category_theory_transformer_rs/"
GITHUB_REPO_URL = "https://github.com/hghalebi/category_theory_transformer_rs"

REQUIRED_ISSUE_FORM_IDS = [
    "review_path",
    "reviewer_context",
    "location",
    "command",
    "friction_lens",
    "quote",
    "understood",
    "stuck",
    "expectation",
    "smallest_fix",
]

REQUIRED_ISSUE_FORM_PRIVACY_MARKERS = [
    "Do not include email addresses",
    "private messages",
    "personal notes",
    "contact details",
]

REQUIRED_FRICTION_LENSES = [
    "Entry path",
    "Rust syntax",
    "ML intuition",
    "Category-theory precision",
    "Diagram clarity",
    "Exercise readiness",
    "Reference needed",
]

REQUIRED_REVIEWER_CONTEXT_OPTIONS = [
    "Rust engineer",
    "ML engineer",
    "Category-theory reader",
    "Technical educator",
    "Beginner-adjacent reader",
]

REQUIRED_TRIAGE_MARKERS = [
    "## Inputs",
    "## Triage Steps",
    "### 1. Check For Concrete Evidence",
    "## Report Quality Examples",
    "### Usable direct report",
    "### Insufficient report",
    "This is an example, not direct evidence.",
    "location-bound, command-backed",
    "### 2. Classify The Friction",
    "### 3. Choose A Rewrite Action",
    "### 4. Rewrite From Evidence",
    "### 5. Record The Pass",
    "### 6. Validate Before Closing",
    "## Close-Out Format",
    "## What Not To Do",
    "reader report\n-> affected chapter section\n-> repository code or example\n-> chapter references\n-> rewrite decision\n-> validation\n-> issue close-out",
    "book/CHAPTER_REWRITE_LOG.md",
    "community/reader-feedback-inbox.md",
    "community/feedback-wall.md",
    EXTERNAL_SYNTHESIS_LINK,
    "python3 scripts/collect-reader-feedback-issues.py",
    "collector redacts email-address patterns in issue titles",
    "scripts/reader-feedback-status.sh --live",
]

REQUIRED_INBOX_MARKERS = [
    "## Current Status",
    "No direct project-specific reader reports have been recorded in this file yet.",
    "## Intake Sources",
    "reader-confusion issue form",
    REVIEWER_OUTREACH_LINK,
    REVIEW_SPRINT_LINK,
    REVIEWER_SLOT_TRACKER_LINK,
    "community/reader-feedback-triage.md",
    "python3 scripts/collect-reader-feedback-issues.py",
    "scripts/reader-feedback-status.sh --live",
    "## Entry Format",
    "## Accepted Report Standard",
    "GitHub issue URL:",
    "Review path:",
    "Reviewer context:",
    "Command or file tried:",
    "Friction lens:",
    "Triage action:",
    "Rewrite log entry:",
    "Validation:",
    "Status:",
    "## Triage States",
    "quality example is not direct evidence",
    "location-bound",
    "command-backed",
    "rewrite-backed",
    "Accepted evidence requires both an accepted `Triage action` and an accepted",
    "email-address patterns in report headings",
    "scripts/collect-reader-feedback-issues.py",
    "## Template Only",
    "The block above is a template, not a report.",
]

REQUIRED_OUTREACH_MARKERS = [
    "## Reviewer Profiles",
    "## Short Public Ask",
    "## Rust Reviewer Ask",
    "## ML Reviewer Ask",
    "## Category-Theory Reviewer Ask",
    "## Educator Ask",
    "## Workshop Follow-Up Ask",
    "## Maintainer Close Loop",
    REVIEW_PACKET_LINK,
    REVIEW_SPRINT_LINK,
    REVIEWER_SLOT_TRACKER_LINK,
    "pre-filled report link",
    "Rust report link:",
    "ML report link:",
    "Category-theory report link:",
    "Educator report link:",
    "community/reader-feedback-inbox.md",
    "community/reader-feedback-triage.md",
    "book/CHAPTER_REWRITE_LOG.md",
    "Do not record a sent outreach message as reader evidence.",
    f"Public book: <{PUBLIC_BOOK_URL}>",
    f"GitHub repository: <{GITHUB_REPO_URL}>",
]

REQUIRED_SPRINT_MARKERS = [
    "## Sprint Goal",
    "1 Rust engineer",
    "1 ML engineer",
    "1 category-theory reader",
    "1 technical educator",
    "1 beginner-adjacent reader",
    "## Sprint Inputs",
    REVIEW_SPRINT_ISSUE_URL,
    f"Public book: <{PUBLIC_BOOK_URL}>",
    f"GitHub repository: <{GITHUB_REPO_URL}>",
    REVIEWER_OUTREACH_LINK,
    REVIEW_PACKET_LINK,
    REVIEW_GUIDE_LINK,
    REVIEWER_SLOT_TRACKER_LINK,
    "community/reader-feedback-inbox.md",
    "community/reader-feedback-triage.md",
    "## Reviewer Assignments",
    "If a reviewer will not clone the repository",
    "the public page or section",
    "## Seven-Day Sprint",
    "### Day 1: Invite",
    "### Day 3: Intake",
    "scripts/check-github-feedback-surface.sh",
    "python3 scripts/collect-reader-feedback-issues.py",
    "scripts/reader-feedback-status.sh --live",
    "### Day 6: Rewrite",
    "### Day 7: Validate And Close",
    "## Acceptance Criteria",
    "at least five accepted reports",
    "each report has a source, reviewer context, location, command or file tried",
    "python3 scripts/check-reader-feedback-inbox.py --require-sprint-complete",
    "python3 scripts/collect-reader-feedback-issues.py --self-test",
    "scripts/reader-feedback-status.sh",
    "## Non-Evidence",
    "sent outreach messages",
    "AI-generated critique",
    "## Sprint Close-Out",
    "book/TEXTBOOK_COMPLETION_AUDIT.md",
]

REQUIRED_SLOT_TRACKER_MARKERS = [
    "## Slot Statuses",
    "## Current Slots",
    "## Intake Procedure",
    "## Completion Check",
    "1 Rust engineer",
    "1 ML engineer",
    "1 category-theory reader",
    "1 technical educator",
    "1 beginner-adjacent reader",
    "Rust engineer",
    "ML engineer",
    "Category-theory reader",
    "Technical educator",
    "Beginner-adjacent reader",
    "`open`",
    "`invited`",
    "`received`",
    "`needs clarification`",
    "`accepted`",
    "`rewritten`",
    "Do not count `invited` as evidence.",
    "Keep names, email addresses, private messages, and personal notes out",
    "community/reader-feedback-inbox.md",
    "community/reader-feedback-triage.md",
    "python3 scripts/check-reader-feedback-inbox.py --require-sprint-complete",
]

SURFACES_THAT_MUST_LINK_TRIAGE = [
    REVIEWER_OUTREACH,
    REVIEW_SPRINT,
    REVIEWER_SLOT_TRACKER,
    REVIEW_GUIDE,
    REVIEW_PACKET,
    FEEDBACK_INBOX,
    FEEDBACK_WALL,
    STARTER_ISSUES,
    COMPLETION_AUDIT,
]

SURFACES_THAT_MUST_LINK_READER_FORM = [
    REVIEWER_OUTREACH,
    README,
    START_HERE,
    REVIEW_GUIDE,
    REVIEW_PACKET,
]

REQUIRED_CATEGORY_SHAPE_REVIEW_MARKERS = {
    REVIEW_PACKET: [
        "Also test the category-shape diagnostic",
        "QuerySequence x KeySequence -> AttentionScores",
        "AttentionScores x AttentionMask -> AttentionScores",
        "LayerNormalization : HiddenSequence -> HiddenSequence",
        "TransformerTrainingState -> TransformerTrainingState",
        "HiddenSequence x MultiHeadOutput -> HiddenSequence",
        "product-input morphism, endomorphism",
        "illegal composition felt unclear",
    ],
    REVIEW_GUIDE: [
        "The newest review target is the category-shape diagnostic",
        "ordinary morphism",
        "product-input morphism",
        "shape-preserving endomorphism",
        "state endomorphism",
        "illegal attempted boundary",
        "should not be reviewed as if it had the same shape",
    ],
    REVIEW_SPRINT: [
        "category-shape diagnostic in the roadmap",
        "product-input/endomorphism confusion",
        "boundary classification",
        "The target report is one unclear distinction",
    ],
    REVIEWER_OUTREACH: [
        "roadmap category-shape review",
        "category-shape",
        "separates product-input morphisms from endomorphisms",
        "unclear boundary is enough",
    ],
    COMPLETION_AUDIT: [
        "category-shape review prompt",
    ],
}

REQUIRED_SELF_CROSS_ATTENTION_REVIEW_MARKERS = {
    REVIEW_PACKET: [
        "Also test the self-attention versus cross-attention boundary",
        "same source for Q, K, V -> self-attention case",
        "parallel HiddenSequence -> QuerySequence, KeySequence, ValueSequence projections",
        "not QuerySequence -> KeySequence -> ValueSequence",
        "separate query and key-value sources -> cross-attention case",
        "target length L x source length S",
        "product-input morphism before any endomorphism claim",
    ],
    REVIEW_GUIDE: [
        "The newest review target is the self-attention versus cross-attention boundary",
        "parallel HiddenSequence -> QuerySequence, KeySequence, ValueSequence projections",
        "not QuerySequence -> KeySequence -> ValueSequence",
        "target sequence length L",
        "source sequence length S",
        "`L x S` mask shape",
        "parallel projections",
        "source-side keys",
        "source-side values",
    ],
    REVIEW_SPRINT: [
        "self-attention versus cross-attention boundary",
        "shared Q/K/V source",
        "parallel `HiddenSequence -> QuerySequence`",
        "false\n`QuerySequence -> KeySequence -> ValueSequence` reading",
        "separate query and key-value sources",
        "target length `L`",
        "source length `S`",
        "product-input morphism before endomorphism",
    ],
    REVIEWER_OUTREACH: [
        "self-attention versus cross-attention",
        "shared Q/K/V source",
        "parallel projections from `HiddenSequence` into",
        "false `QuerySequence -> KeySequence ->\nValueSequence` reading",
        "separate query and key-value sources",
        "product-input morphism before any",
    ],
    COMPLETION_AUDIT: [
        "self-vs-cross attention review prompt",
        "parallel Q/K/V projection review prompt",
    ],
}

REQUIRED_PUBLIC_REVIEW_PACKET_MARKERS = [
    f"Public book:\n\n```text\n{PUBLIC_BOOK_URL}\n```",
    f"GitHub repository:\n\n```text\n{GITHUB_REPO_URL}\n```",
    "## Reviewer Context Briefs",
    "Use one brief that matches your background.",
    "| Rust engineer | `cargo run --example 01_token_sequence`",
    "| ML engineer | `cargo run --example 03_training_endomorphism`",
    "| Category-theory reader | `cargo run --example 02_morphism_composition`",
    "| Technical educator | no clone required; start with the public book path",
    "| Beginner-adjacent reader | `cargo run --example 01_token_sequence`",
    "This exact line, output, command, or term broke the mental model.",
    "## Context-Specific Report Links",
    "GitHub supports issue URLs that select a template and fill custom text fields.",
    "Open Rust engineer report",
    "Open ML engineer report",
    "Open category-theory reader report",
    "Open technical educator report",
    "Open beginner-adjacent reader report",
    "title=%5Breader+confusion%5D+Rust+engineer+brief",
    "title=%5Breader+confusion%5D+ML+engineer+brief",
    "title=%5Breader+confusion%5D+category-theory+reader+brief",
    "title=%5Breader+confusion%5D+technical+educator+brief",
    "title=%5Breader+confusion%5D+beginner-adjacent+reader+brief",
    "## Path D: Public Book Review",
    "Use this path if you want to review without cloning the repository first.",
    "Welcome\nCourse Map\nDomain Objects\nMorphism and Composition",
    "public page or section read",
]

REQUIRED_REVIEW_CONTEXT_BRIEF_MARKERS = {
    REVIEW_SPRINT: [
        "Reviewer Context Briefs",
        "one concrete command path and one concrete report target",
        "context-specific report link",
        "Tell each reviewer which row of `Reviewer Context Briefs` to use.",
        "Use the matching context-specific report link",
    ],
    REVIEWER_SLOT_TRACKER: [
        "Reviewer Context Briefs",
        "Context-Specific Report Links",
        "Rust engineer brief plus first-run commands",
        "ML engineer brief plus examples `03` and `06`",
        "Category-theory reader brief plus Morphism, Structure, Seven Sketches, or Roadmap",
        "Technical educator brief plus README, start guide, review packet, Welcome, Course Map, Exercises",
        "Beginner-adjacent reader brief plus first-run or public-book path",
        "use the Rust engineer report link",
        "use the ML engineer report link",
        "use the category-theory reader report link",
        "use the technical educator report link",
        "use the beginner-adjacent reader report link",
    ],
    COMPLETION_AUDIT: [
        "role-specific reviewer briefs",
    ],
}

EXPECTED_CONTEXT_REPORT_LINKS = {
    "Rust engineer": {
        "link_text": "Open Rust engineer report",
        "title": "[reader confusion] Rust engineer brief",
        "location_marker": "book/src/01-domain-objects.md",
        "command_marker": "cargo run --example 01_token_sequence",
    },
    "ML engineer": {
        "link_text": "Open ML engineer report",
        "title": "[reader confusion] ML engineer brief",
        "location_marker": "book/src/03-ml-pipeline.md",
        "command_marker": "cargo run --example 03_training_endomorphism",
    },
    "Category-theory reader": {
        "link_text": "Open category-theory reader report",
        "title": "[reader confusion] category-theory reader brief",
        "location_marker": "book/src/02-morphisms-composition.md",
        "command_marker": "cargo run --example 02_morphism_composition",
    },
    "Technical educator": {
        "link_text": "Open technical educator report",
        "title": "[reader confusion] technical educator brief",
        "location_marker": "README.md",
        "command_marker": "public book review path",
    },
    "Beginner-adjacent reader": {
        "link_text": "Open beginner-adjacent reader report",
        "title": "[reader confusion] beginner-adjacent reader brief",
        "location_marker": "Welcome",
        "command_marker": "cargo run --example 01_token_sequence",
    },
}

SAFE_PREFILL_QUERY_FIELDS = {"template", "title", "location", "command"}
FORBIDDEN_PREFILL_QUERY_FIELDS = {
    "review_path",
    "reviewer_context",
    "friction_lens",
    "quote",
    "understood",
    "stuck",
    "expectation",
    "smallest_fix",
    "body",
}


def read(path: Path) -> str:
    return path.read_text(encoding="utf-8")


def check_issue_form() -> list[str]:
    text = read(ISSUE_FORM)
    issues: list[str] = []

    for marker in REQUIRED_ISSUE_FORM_PRIVACY_MARKERS:
        if marker not in text:
            issues.append(f"{ISSUE_FORM}: missing privacy marker {marker!r}")

    for label in ("reader confusion", "good first feedback"):
        if f"- {label}" not in text:
            issues.append(f"{ISSUE_FORM}: missing label {label!r}")

    for form_id in REQUIRED_ISSUE_FORM_IDS:
        if f"id: {form_id}" not in text:
            issues.append(f"{ISSUE_FORM}: missing issue-form id {form_id!r}")

    for lens in REQUIRED_FRICTION_LENSES:
        if f"- {lens}" not in text:
            issues.append(f"{ISSUE_FORM}: missing friction lens option {lens!r}")

    for context in REQUIRED_REVIEWER_CONTEXT_OPTIONS:
        if f"- {context}" not in text:
            issues.append(f"{ISSUE_FORM}: missing reviewer context option {context!r}")

    return issues


def check_issue_config() -> list[str]:
    text = read(ISSUE_CONFIG)
    issues: list[str] = []

    if "Reader Review Packet" not in text:
        issues.append(f"{ISSUE_CONFIG}: missing Reader Review Packet contact link")

    if REVIEW_PACKET_LINK not in text:
        issues.append(f"{ISSUE_CONFIG}: missing {REVIEW_PACKET_LINK}")

    return issues


def check_triage_protocol() -> list[str]:
    text = read(TRIAGE)
    issues: list[str] = []

    for marker in REQUIRED_TRIAGE_MARKERS:
        if marker not in text:
            issues.append(f"{TRIAGE}: missing triage marker {marker!r}")

    for action in ("Fix now", "Batch theme", "Ask clarification", "Close out of scope"):
        if f"| {action} |" not in text:
            issues.append(f"{TRIAGE}: missing rewrite action {action!r}")

    return issues


def check_linked_surfaces() -> list[str]:
    issues: list[str] = []

    for path in SURFACES_THAT_MUST_LINK_TRIAGE:
        text = read(path)
        if TRIAGE_LINK not in text:
            issues.append(f"{path}: should link to {TRIAGE_LINK}")

    guide = read(REVIEW_GUIDE)
    if REVIEW_PACKET_LINK not in guide:
        issues.append(f"{REVIEW_GUIDE}: should point to {REVIEW_PACKET_LINK}")
    if REVIEWER_OUTREACH_LINK not in guide:
        issues.append(f"{REVIEW_GUIDE}: should point maintainers to {REVIEWER_OUTREACH_LINK}")
    if REVIEW_SPRINT_LINK not in guide:
        issues.append(f"{REVIEW_GUIDE}: should point coordinated reviews to {REVIEW_SPRINT_LINK}")

    packet = read(REVIEW_PACKET)
    if READER_CONFUSION_FORM_URL not in packet:
        issues.append(f"{REVIEW_PACKET}: should direct reviewers to the reader-confusion issue form")

    wall = read(FEEDBACK_WALL)
    if EXTERNAL_SYNTHESIS_LINK not in wall:
        issues.append(f"{FEEDBACK_WALL}: should keep proxy synthesis link visible")
    if REVIEWER_OUTREACH_LINK not in wall:
        issues.append(f"{FEEDBACK_WALL}: should point reviewer requests to {REVIEWER_OUTREACH_LINK}")
    if REVIEW_SPRINT_LINK not in wall:
        issues.append(f"{FEEDBACK_WALL}: should point review cohorts to {REVIEW_SPRINT_LINK}")
    if "community/reader-feedback-inbox.md" not in wall:
        issues.append(f"{FEEDBACK_WALL}: should point direct reports to the reader feedback inbox")

    starter = read(STARTER_ISSUES)
    if REVIEW_GUIDE_LINK not in starter:
        issues.append(f"{STARTER_ISSUES}: should point reader-review issues to {REVIEW_GUIDE_LINK}")

    return issues


def check_feedback_inbox() -> list[str]:
    text = read(FEEDBACK_INBOX)
    issues: list[str] = []

    for marker in REQUIRED_INBOX_MARKERS:
        if marker not in text:
            issues.append(f"{FEEDBACK_INBOX}: missing inbox marker {marker!r}")

    for state in ("new", "needs clarification", "fix now", "batched theme", "rewritten", "closed out of scope"):
        if f"| `{state}` |" not in text:
            issues.append(f"{FEEDBACK_INBOX}: missing triage state {state!r}")

    return issues


def check_reviewer_outreach() -> list[str]:
    text = read(REVIEWER_OUTREACH)
    issues: list[str] = []

    for marker in REQUIRED_OUTREACH_MARKERS:
        if marker not in text:
            issues.append(f"{REVIEWER_OUTREACH}: missing outreach marker {marker!r}")

    for reviewer in ("Rust engineer", "ML engineer", "Category-theory reader", "Technical educator"):
        if reviewer not in text:
            issues.append(f"{REVIEWER_OUTREACH}: missing reviewer profile {reviewer!r}")

    return issues


def check_reader_review_sprint() -> list[str]:
    text = read(REVIEW_SPRINT)
    issues: list[str] = []

    for marker in REQUIRED_SPRINT_MARKERS:
        if marker not in text:
            issues.append(f"{REVIEW_SPRINT}: missing sprint marker {marker!r}")

    for day in range(1, 8):
        if f"### Day {day}:" not in text:
            issues.append(f"{REVIEW_SPRINT}: missing Day {day} plan")

    return issues


def check_reviewer_slot_tracker() -> list[str]:
    text = read(REVIEWER_SLOT_TRACKER)
    issues: list[str] = []

    for marker in REQUIRED_SLOT_TRACKER_MARKERS:
        if marker not in text:
            issues.append(f"{REVIEWER_SLOT_TRACKER}: missing slot tracker marker {marker!r}")

    if "names, email addresses, private messages" not in text:
        issues.append(f"{REVIEWER_SLOT_TRACKER}: must forbid personal contact details")

    for context in REQUIRED_REVIEWER_CONTEXT_OPTIONS:
        if f"| {context} |" not in text:
            issues.append(f"{REVIEWER_SLOT_TRACKER}: missing slot for {context!r}")

    return issues


def check_reader_form_links() -> list[str]:
    issues: list[str] = []

    for path in SURFACES_THAT_MUST_LINK_READER_FORM:
        text = read(path)
        if READER_CONFUSION_FORM_URL not in text:
            issues.append(f"{path}: should link directly to {READER_CONFUSION_FORM_URL}")

    readme_top = read(README).split("## Who this is for", maxsplit=1)[0]
    stale_issue_link = "https://github.com/hghalebi/category_theory_transformer_rs/issues/1"
    if stale_issue_link in readme_top:
        issues.append(
            f"{README}: top-level feedback path should use the issue form, not {stale_issue_link}"
        )

    return issues


def check_category_shape_review_path() -> list[str]:
    issues: list[str] = []

    for path, markers in REQUIRED_CATEGORY_SHAPE_REVIEW_MARKERS.items():
        text = read(path)

        for marker in markers:
            if marker not in text:
                issues.append(f"{path}: missing category-shape review marker {marker!r}")

    return issues


def check_self_cross_attention_review_path() -> list[str]:
    issues: list[str] = []

    for path, markers in REQUIRED_SELF_CROSS_ATTENTION_REVIEW_MARKERS.items():
        text = read(path)

        for marker in markers:
            if marker not in text:
                issues.append(f"{path}: missing self/cross attention review marker {marker!r}")

    return issues


def check_public_book_review_path() -> list[str]:
    text = read(REVIEW_PACKET)
    issues: list[str] = []

    for marker in REQUIRED_PUBLIC_REVIEW_PACKET_MARKERS:
        if marker not in text:
            issues.append(f"{REVIEW_PACKET}: missing public review path marker {marker!r}")

    for path in (REVIEWER_OUTREACH, REVIEW_SPRINT):
        text = read(path)
        if PUBLIC_BOOK_URL not in text:
            issues.append(f"{path}: should link the public book review URL")
        if GITHUB_REPO_URL not in text:
            issues.append(f"{path}: should link the GitHub repository URL")

    return issues


def check_reviewer_context_briefs() -> list[str]:
    issues: list[str] = []

    for path, markers in REQUIRED_REVIEW_CONTEXT_BRIEF_MARKERS.items():
        text = read(path)

        for marker in markers:
            if marker not in text:
                issues.append(f"{path}: missing reviewer-context brief marker {marker!r}")

    return issues


def check_outreach_report_links() -> list[str]:
    outreach = read(REVIEWER_OUTREACH)
    packet_links = context_report_links(read(REVIEW_PACKET))
    issues: list[str] = []

    required_contexts = [
        "Rust engineer",
        "ML engineer",
        "Category-theory reader",
        "Technical educator",
        "Beginner-adjacent reader",
    ]

    for context in required_contexts:
        link = packet_links.get(context)
        if link is None:
            issues.append(
                f"{REVIEW_PACKET}: missing packet report link needed by outreach for {context!r}"
            )
            continue

        _, url = link
        if url not in outreach:
            issues.append(
                f"{REVIEWER_OUTREACH}: should include the packet report URL for {context!r}"
            )

    return issues


def context_report_links(text: str) -> dict[str, tuple[str, str]]:
    section_start = text.find("## Context-Specific Report Links")
    if section_start == -1:
        return {}

    next_section = text.find("\n## ", section_start + len("## Context-Specific Report Links"))
    section = text[section_start:] if next_section == -1 else text[section_start:next_section]
    links: dict[str, tuple[str, str]] = {}

    for line in section.splitlines():
        stripped = line.strip()
        if not stripped.startswith("|") or stripped.startswith("| ---") or "Reviewer context" in stripped:
            continue

        cells = [cell.strip() for cell in stripped.strip("|").split("|")]
        if len(cells) != 2:
            continue

        context, link_cell = cells
        match = re.fullmatch(r"\[(?P<label>[^\]]+)\]\((?P<url>[^)]+)\)", link_cell)
        if match is None:
            continue

        links[context] = (match.group("label"), match.group("url"))

    return links


def single_query_value(params: dict[str, list[str]], key: str) -> str:
    values = params.get(key, [])
    return values[0] if values else ""


def check_context_report_links() -> list[str]:
    text = read(REVIEW_PACKET)
    links = context_report_links(text)
    issues: list[str] = []

    for context, expected in EXPECTED_CONTEXT_REPORT_LINKS.items():
        link = links.get(context)
        if link is None:
            issues.append(f"{REVIEW_PACKET}: missing context-specific report link for {context!r}")
            continue

        link_text, url = link
        if link_text != expected["link_text"]:
            issues.append(
                f"{REVIEW_PACKET}: {context!r} report link text should be "
                f"{expected['link_text']!r}"
            )

        if len(url) > 2000:
            issues.append(f"{REVIEW_PACKET}: {context!r} report URL is too long")

        parsed = urlparse(url)
        if parsed.scheme != "https" or parsed.netloc != "github.com":
            issues.append(f"{REVIEW_PACKET}: {context!r} report URL should use github.com HTTPS")

        expected_path = "/hghalebi/category_theory_transformer_rs/issues/new"
        if parsed.path != expected_path:
            issues.append(
                f"{REVIEW_PACKET}: {context!r} report URL path should be {expected_path!r}"
            )

        params = parse_qs(parsed.query, keep_blank_values=True)
        query_keys = set(params)
        unknown_keys = query_keys - SAFE_PREFILL_QUERY_FIELDS
        if unknown_keys:
            issues.append(
                f"{REVIEW_PACKET}: {context!r} report URL has unsupported query fields "
                f"{sorted(unknown_keys)!r}"
            )

        forbidden_keys = query_keys & FORBIDDEN_PREFILL_QUERY_FIELDS
        if forbidden_keys:
            issues.append(
                f"{REVIEW_PACKET}: {context!r} report URL must not prefill evidence fields "
                f"{sorted(forbidden_keys)!r}"
            )

        if single_query_value(params, "template") != "reader-confusion.yml":
            issues.append(f"{REVIEW_PACKET}: {context!r} report URL must use reader-confusion.yml")

        if single_query_value(params, "title") != expected["title"]:
            issues.append(
                f"{REVIEW_PACKET}: {context!r} report URL title should be "
                f"{expected['title']!r}"
            )

        location = single_query_value(params, "location")
        if expected["location_marker"] not in location:
            issues.append(
                f"{REVIEW_PACKET}: {context!r} report URL location should include "
                f"{expected['location_marker']!r}"
            )

        command = single_query_value(params, "command")
        if expected["command_marker"] not in command:
            issues.append(
                f"{REVIEW_PACKET}: {context!r} report URL command should include "
                f"{expected['command_marker']!r}"
            )

    unexpected_contexts = set(links) - set(EXPECTED_CONTEXT_REPORT_LINKS)
    for context in sorted(unexpected_contexts):
        issues.append(f"{REVIEW_PACKET}: unexpected context-specific report link {context!r}")

    return issues


def main() -> int:
    issues: list[str] = []

    issues.extend(check_issue_form())
    issues.extend(check_issue_config())
    issues.extend(check_triage_protocol())
    issues.extend(check_linked_surfaces())
    issues.extend(check_reader_form_links())
    issues.extend(check_feedback_inbox())
    issues.extend(check_reviewer_outreach())
    issues.extend(check_reader_review_sprint())
    issues.extend(check_reviewer_slot_tracker())
    issues.extend(check_category_shape_review_path())
    issues.extend(check_self_cross_attention_review_path())
    issues.extend(check_public_book_review_path())
    issues.extend(check_reviewer_context_briefs())
    issues.extend(check_context_report_links())
    issues.extend(check_outreach_report_links())

    if issues:
        print("Reader feedback loop check failed:")
        for issue in issues:
            print(f"- {issue}")
        return 1

    print("Reader feedback loop check passed.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
