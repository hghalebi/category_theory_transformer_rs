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

require_reference "https://github.com/hghalebi/category_theory_transformer_rs"
require_reference "https://luma.com/event/evt-Pb1kYMQvzs8JrQq"
require_reference "https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html"
require_reference "https://doc.rust-lang.org/book/ch05-01-defining-structs.html"
require_reference "https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html"
require_reference "https://doc.rust-lang.org/book/ch10-01-syntax.html"
require_reference "https://doc.rust-lang.org/book/ch10-02-traits.html"
require_reference "https://doc.rust-lang.org/stable/book/ch09-02-recoverable-errors-with-result.html"
require_reference "https://doc.rust-lang.org/stable/book/ch11-00-testing.html"
require_reference "https://doc.rust-lang.org/rust-by-example/index.html"
require_reference "https://doc.rust-lang.org/rust-by-example/cargo/test.html"
require_reference "https://doc.rust-lang.org/rustdoc/how-to-write-documentation.html"
require_reference "https://rust-lang.github.io/api-guidelines/checklist.html"
require_reference "https://arxiv.org/abs/1803.05316"
require_reference "https://arxiv.org/pdf/1803.05316"
require_reference "https://arxiv.org/abs/2209.01259"
require_reference "https://github.com/hmemcpy/milewski-ctfp-pdf"
require_reference "https://d2l.ai/chapter_linear-classification/softmax-regression.html"
require_reference "https://d2l.ai/chapter_linear-classification/softmax-regression-scratch.html"
require_reference "https://d2l.ai/chapter_optimization/gd.html"
require_reference "https://d2l.ai/chapter_multilayer-perceptrons/backprop.html"
require_reference "https://d2l.ai/chapter_multilayer-perceptrons/numerical-stability-and-init.html"
require_reference "https://cs231n.github.io/linear-classify/"
require_reference "https://www.deeplearningbook.org/"
require_reference "https://arxiv.org/abs/1711.10455"
require_reference "https://arxiv.org/abs/1907.08292"
require_reference "https://arxiv.org/abs/1706.03762"
require_reference "https://arxiv.org/abs/1607.06450"
require_reference "https://papers.nips.cc/paper/7181-attention-is-all-you-need"
require_reference "https://d2l.ai/chapter_attention-mechanisms-and-transformers/index.html"
require_reference "https://d2l.ai/chapter_attention-mechanisms-and-transformers/queries-keys-values.html"
require_reference "https://d2l.ai/chapter_attention-mechanisms-and-transformers/attention-scoring-functions.html"
require_reference "https://d2l.ai/chapter_attention-mechanisms-and-transformers/multihead-attention.html"
require_reference "https://d2l.ai/chapter_attention-mechanisms-and-transformers/transformer.html"
require_reference "https://d2l.ai/chapter_builders-guide/parameters.html"
require_reference "https://nlp.seas.harvard.edu/2018/04/03/attention.html"
require_reference "https://jalammar.github.io/illustrated-transformer/"
require_reference "https://www.nationalacademies.org/projects/DBASSE-BBCSS-13-06/publication/24783"
require_reference "https://journals.sagepub.com/doi/abs/10.1177/1529100612453266"
require_reference "https://doi.org/10.1111/j.1467-9280.2006.01693.x"
require_reference "https://doi.org/10.1207/S15326985EP3801_3"
require_reference "https://doi.org/10.1207/s15516709cog1302_1"

if grep -R -E -n -- 'Andrew[[:space:]]+Ng' book/src README.md lessons; then
  printf 'book and learner-facing docs should not name the instructor explicitly\n' >&2
  exit 1
fi

if grep -R -E -n -- 'mdBook|mdbook' book/src lessons; then
  printf 'learner book content should not discuss the book-generation tool\n' >&2
  exit 1
fi
