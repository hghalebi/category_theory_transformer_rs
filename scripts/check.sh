#!/usr/bin/env bash
set -euo pipefail

cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features

cargo run --example 01_token_sequence
cargo run --example 01_domain_objects
cargo run --example 02_morphism_composition
cargo run --example 03_training_endomorphism
cargo run --example 04_structure_and_calculus
cargo run --example 05_seven_sketches
cargo run --example 06_attention_scores
cargo run --bin category_ml

python3 scripts/check-prose-style.py
python3 scripts/check-completion-audit.py
python3 scripts/check-reader-feedback-loop.py
python3 scripts/check-reader-feedback-inbox.py
python3 scripts/check-reader-feedback-inbox.py --self-test
python3 scripts/check-reviewer-slot-tracker.py
python3 scripts/check-reviewer-slot-tracker.py --self-test
python3 scripts/check-public-friction-matrix.py
python3 scripts/collect-reader-feedback-issues.py --self-test
bash scripts/check-github-feedback-surface.sh --self-test
scripts/reader-feedback-status.sh --self-test
python3 scripts/check-rewrite-log.py
python3 scripts/check-chapter-references.py
python3 scripts/check-chapter-contracts.py
python3 scripts/check-chapter-scorecard.py
python3 scripts/check-chapter-scorecard.py --self-test
python3 scripts/check-source-authority.py
python3 scripts/check-reference-links-live.py --self-test
python3 scripts/check-chapter-maturity.py
python3 scripts/check-diagram-coverage.py
python3 scripts/check-exercise-alignment.py
python3 scripts/check-exercise-commands.py
python3 scripts/check-duplicate-prose.py
bash scripts/check-mdbook-coverage.sh
bash scripts/build-mdbook.sh
mdbook test
