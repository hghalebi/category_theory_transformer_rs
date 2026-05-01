# Validation and Troubleshooting

## Full Check

From the repository root:

```bash
bash scripts/check.sh
```

This runs:

- `cargo fmt --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test --all-targets --all-features`
- all lesson examples
- the full terminal demo
- the mdBook build
- mdBook chapter tests

## GitHub Pages CI/CD

The workflow file is:

```text
.github/workflows/mdbook-pages.yml
```

Pull requests run validation only.

Pushes to `main` validate the Rust crate and mdBook, upload `book/html`, and
deploy it to GitHub Pages.

The expected project URL is:

```text
https://hghalebi.github.io/category_theory_transformer_rs/
```

In the GitHub repository settings, set:

```text
Settings -> Pages -> Build and deployment -> Source -> GitHub Actions
```

## Build Only the Book

```bash
bash scripts/build-mdbook.sh
```

Generated HTML goes to:

```text
book/html/
```

## Serve the Book Locally

```bash
mdbook serve --open
```

If the browser does not open automatically, use the URL printed by `mdbook`.

## Common Fixes

If a Rust example fails, run the exact example named in the chapter:

```bash
cargo run --example 03_training_endomorphism
```

If the book fails to build, check:

- `book.toml` exists
- `book/src/SUMMARY.md` exists
- every linked chapter file exists under `book/src`
- `mdbook --version` prints a version
