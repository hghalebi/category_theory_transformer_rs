#!/usr/bin/env bash
set -euo pipefail

repo="hghalebi/category_theory_transformer_rs"

required_labels=(
  "good first feedback"
  "reader confusion"
  "needs Rust example"
  "needs diagram"
  "chapter expansion"
  "ML intuition"
  "category theory precision"
  "Rust idiom review"
  "exercise idea"
  "glossary needed"
  "sponsor-worthy milestone"
  "reader review sprint"
)

required_issue_body_markers=(
  "Open Rust engineer report"
  "Open ML engineer report"
  "Open category-theory reader report"
  "Open technical educator report"
  "Open beginner-adjacent reader report"
  "title=%5Breader+confusion%5D+Rust+engineer+brief"
  "title=%5Breader+confusion%5D+ML+engineer+brief"
  "title=%5Breader+confusion%5D+category-theory+reader+brief"
  "title=%5Breader+confusion%5D+technical+educator+brief"
  "title=%5Breader+confusion%5D+beginner-adjacent+reader+brief"
  "## Latest roadmap review targets"
  "self-attention versus cross-attention boundary"
  "parallel Q/K/V projections are not a role-to-role pipeline"
  "Do not count this tracking issue as direct reader evidence"
)

check_labels() {
  local labels="$1"

  for label in "${required_labels[@]}"; do
    if ! grep -Fxq "$label" <<<"$labels"; then
      echo "Missing GitHub label: $label" >&2
      return 1
    fi
  done
}

check_review_sprint_issue() {
  local issue_json="$1"
  local issue_title
  local issue_state
  local issue_labels
  local issue_body

  issue_title="$(jq -r '.title' <<<"$issue_json")"
  issue_state="$(jq -r '.state' <<<"$issue_json")"
  issue_labels="$(jq -r '.labels[].name' <<<"$issue_json")"
  issue_body="$(jq -r '.body // ""' <<<"$issue_json")"

  if [[ "$issue_title" != "[reader review sprint] Collect five direct reader reports" ]]; then
    echo "Issue #6 has unexpected title: $issue_title" >&2
    return 1
  fi

  if [[ "$issue_state" != "OPEN" ]]; then
    echo "Issue #6 should remain open while direct reader reports are pending" >&2
    return 1
  fi

  if ! grep -Fxq "reader review sprint" <<<"$issue_labels"; then
    echo "Issue #6 is missing the reader review sprint label" >&2
    return 1
  fi

  for marker in "${required_issue_body_markers[@]}"; do
    if [[ "$issue_body" != *"$marker"* ]]; then
      echo "Issue #6 body is missing marker: $marker" >&2
      return 1
    fi
  done

  local duplicate_latest_target_count
  duplicate_latest_target_count="$(grep -Fxc "## Latest roadmap review target" <<<"$issue_body" || true)"
  if [[ "$duplicate_latest_target_count" != "0" ]]; then
    echo "Issue #6 should use one plural latest-roadmap-targets section" >&2
    return 1
  fi

  local latest_targets_count
  latest_targets_count="$(grep -Fxc "## Latest roadmap review targets" <<<"$issue_body" || true)"
  if [[ "$latest_targets_count" != "1" ]]; then
    echo "Issue #6 should have exactly one latest-roadmap-targets section" >&2
    return 1
  fi
}

run_self_test() {
  local fixture_labels
  local missing_label_fixture
  local issue_json
  local closed_issue_json

  fixture_labels="$(printf "%s\n" "${required_labels[@]}")"
  missing_label_fixture="$(grep -Fxv "reader confusion" <<<"$fixture_labels")"
  local body

  body="$(printf "%s\n" "${required_issue_body_markers[@]}")"
  issue_json="$(jq -n --arg body "$body" '{"title":"[reader review sprint] Collect five direct reader reports","state":"OPEN","labels":[{"name":"reader review sprint"}],"body":$body}')"
  closed_issue_json="$(jq -n --arg body "$body" '{"title":"[reader review sprint] Collect five direct reader reports","state":"CLOSED","labels":[{"name":"reader review sprint"}],"body":$body}')"

  check_labels "$fixture_labels"

  if check_labels "$missing_label_fixture" 2>/dev/null; then
    echo "GitHub feedback surface self-test failed: missing label fixture passed" >&2
    return 1
  fi

  check_review_sprint_issue "$issue_json"

  if check_review_sprint_issue "$closed_issue_json" 2>/dev/null; then
    echo "GitHub feedback surface self-test failed: closed issue fixture passed" >&2
    return 1
  fi

  echo "GitHub feedback surface self-test passed."
}

if [[ "${1:-}" == "--self-test" ]]; then
  run_self_test
  exit 0
fi

labels="$(gh label list --repo "$repo" --limit 200 --json name --jq '.[].name')"
check_labels "$labels"

  issue_json="$(gh issue view 6 --repo "$repo" --json number,title,state,labels,url,body)"
check_review_sprint_issue "$issue_json"

echo "GitHub feedback surface check passed."
