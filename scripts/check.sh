#!/usr/bin/env bash
set -euo pipefail

cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features

cargo run --example 01_domain_objects
cargo run --example 02_morphism_composition
cargo run --example 03_training_endomorphism
cargo run --example 04_structure_and_calculus
cargo run --example 05_seven_sketches
cargo run --bin category_ml

bash scripts/check-mdbook-coverage.sh
bash scripts/build-mdbook.sh
mdbook test
