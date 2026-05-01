#!/usr/bin/env bash
set -euo pipefail

require_include() {
  local path="$1"
  local include="{{#include ../../${path}"

  if ! grep -R -F -q -- "${include}" book/src; then
    printf 'missing course source snapshot include for %s\n' "${path}" >&2
    return 1
  fi
}

require_summary_entry() {
  local entry="$1"

  if ! grep -F -q -- "${entry}" book/src/SUMMARY.md; then
    printf 'book/src/SUMMARY.md must include %s\n' "${entry}" >&2
    return 1
  fi
}

require_reference() {
  local url="$1"

  if ! grep -F -q -- "${url}" book/src/references.md; then
    printf 'book/src/references.md must include external reference %s\n' "${url}" >&2
    return 1
  fi
}

while IFS= read -r path; do
  require_include "${path}"
done < <(find src -type f -name '*.rs' | sort)

while IFS= read -r path; do
  require_include "${path}"
done < <(find examples -type f -name '*.rs' | sort)

while IFS= read -r path; do
  require_include "${path}"
done < <(find lessons -type f -name '*.md' | sort)

require_include "Cargo.toml"

require_summary_entry "[Glossary](glossary.md)"
require_summary_entry "[References](references.md)"
require_summary_entry "[Transformer Roadmap](roadmap.md)"
require_summary_entry "[Repository Source Snapshots](source-snapshots.md)"

require_reference "https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html"
require_reference "https://doc.rust-lang.org/stable/book/ch09-02-recoverable-errors-with-result.html"
require_reference "https://doc.rust-lang.org/rustdoc/how-to-write-documentation.html"
require_reference "https://rust-lang.github.io/api-guidelines/checklist.html"
require_reference "https://arxiv.org/abs/1803.05316"
require_reference "https://d2l.ai/chapter_linear-classification/softmax-regression.html"
require_reference "https://d2l.ai/chapter_linear-classification/softmax-regression-scratch.html"
require_reference "https://cs231n.github.io/linear-classify/"
require_reference "https://arxiv.org/abs/1706.03762"
require_reference "https://papers.nips.cc/paper/7181-attention-is-all-you-need"
require_reference "https://d2l.ai/chapter_attention-mechanisms-and-transformers/index.html"

if grep -R -E -n -- 'Andrew[[:space:]]+Ng' book/src README.md lessons; then
  printf 'book and learner-facing docs should not name the instructor explicitly\n' >&2
  exit 1
fi

if grep -R -E -n -- 'mdBook|mdbook' book/src lessons; then
  printf 'learner book content should not discuss the book-generation tool\n' >&2
  exit 1
fi
