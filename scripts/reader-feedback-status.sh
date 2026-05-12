#!/usr/bin/env bash
set -euo pipefail

if [[ "${1:-}" == "--self-test" ]]; then
  python3 scripts/check-reader-feedback-inbox.py --self-test
  python3 scripts/collect-reader-feedback-issues.py --self-test
  bash scripts/check-github-feedback-surface.sh --self-test
  python3 scripts/check-reviewer-slot-tracker.py --self-test
  echo "Reader feedback status self-test passed."
  exit 0
fi

echo "== Local reader feedback inbox =="
python3 scripts/check-reader-feedback-inbox.py

echo
echo "== Strict completion gate =="
if python3 scripts/check-reader-feedback-inbox.py --require-sprint-complete; then
  echo "Strict completion gate passed."
else
  echo "Strict completion gate is still open."
fi

if [[ "${1:-}" == "--live" ]]; then
  echo
  echo "== GitHub feedback surface =="
  scripts/check-github-feedback-surface.sh

  echo
  echo "== GitHub reader-confusion issue intake =="
  python3 scripts/collect-reader-feedback-issues.py
fi
