#!/usr/bin/env bash
set -uo pipefail

status=0

run_required() {
  local label="$1"
  shift

  echo
  echo "== ${label} =="

  if "$@"; then
    echo "pass: ${label}"
  else
    echo "fail: ${label}" >&2
    status=1
  fi
}

run_absence_guard() {
  local label="$1"
  local pattern="$2"
  shift 2

  echo
  echo "== ${label} =="

  if rg -n --glob '!scripts/check-textbook-readiness.sh' "${pattern}" "$@"; then
    echo "fail: ${label}" >&2
    status=1
  else
    echo "pass: ${label}"
  fi
}

if [[ "${1:-}" == "--full" ]]; then
  run_required "Full publication gate" bash scripts/check.sh
else
  echo "Skipping full publication gate. Pass --full to run bash scripts/check.sh first."
fi

run_required "Completion audit" python3 scripts/check-completion-audit.py
run_required "Reader feedback loop" python3 scripts/check-reader-feedback-loop.py
run_required "Reader feedback inbox structure" python3 scripts/check-reader-feedback-inbox.py
run_required "Reader feedback inbox self-test" python3 scripts/check-reader-feedback-inbox.py --self-test
run_required "Reader feedback issue collector self-test" \
  python3 scripts/collect-reader-feedback-issues.py --self-test
run_required "GitHub feedback surface self-test" \
  bash scripts/check-github-feedback-surface.sh --self-test
run_required "Reader feedback status self-test" \
  scripts/reader-feedback-status.sh --self-test
run_required "Reviewer slot tracker" python3 scripts/check-reviewer-slot-tracker.py
run_required "Reviewer slot tracker self-test" python3 scripts/check-reviewer-slot-tracker.py --self-test
run_required "Strict direct-reader completion gate" \
  python3 scripts/check-reader-feedback-inbox.py --require-sprint-complete
run_required "Rewrite log" python3 scripts/check-rewrite-log.py
run_required "Prose style" python3 scripts/check-prose-style.py
run_required "Chapter references" python3 scripts/check-chapter-references.py
run_required "Chapter contracts" python3 scripts/check-chapter-contracts.py
run_required "Chapter scorecard" python3 scripts/check-chapter-scorecard.py
run_required "Chapter scorecard self-test" python3 scripts/check-chapter-scorecard.py --self-test
run_required "Source authority" python3 scripts/check-source-authority.py
run_required "Reference link live self-test" python3 scripts/check-reference-links-live.py --self-test
run_required "Chapter maturity" python3 scripts/check-chapter-maturity.py
run_required "Diagram coverage" python3 scripts/check-diagram-coverage.py
run_required "Public friction matrix" python3 scripts/check-public-friction-matrix.py
run_required "Exercise alignment" python3 scripts/check-exercise-alignment.py
run_required "Exercise commands" python3 scripts/check-exercise-commands.py
run_required "Duplicate prose" python3 scripts/check-duplicate-prose.py
run_required "Whitespace diff check" git diff --check -- . ':!target'

run_absence_guard \
  "Public docs must not name the instructor lens" \
  'Andrew|Andrew[[:space:]]+Ng|\bJay\b' \
  book/src README.md lessons community exercises

run_absence_guard \
  "Learner book content must not mention the book generator" \
  'mdBook|mdbook' \
  book/src lessons

run_absence_guard \
  "No placeholder or unchecked-error patterns in reviewed surfaces" \
  'TODO|todo!|unwrap\(|expect\(' \
  README.md START_HERE.md ROADMAP.md book/src community exercises lessons scripts src examples

if [[ "${status}" -eq 0 ]]; then
  echo
  echo "Textbook readiness check passed."
else
  echo
  echo "Textbook readiness check failed. Do not mark the textbook goal complete."
fi

exit "${status}"
