#!/usr/bin/env python3
"""Optionally validate that editorial reference links are still reachable.

The normal publication gate runs this script in deterministic self-test mode.
Run without ``--self-test`` when doing a source refresh that may use the
network.
"""

from __future__ import annotations

import argparse
import sys
import urllib.error
import urllib.request
from dataclasses import dataclass
from pathlib import Path
from typing import Callable, Iterable


ROOT = Path(__file__).resolve().parents[1]
DEFAULT_FILES = (
    ROOT / "book" / "src" / "references.md",
    ROOT / "book" / "EDITORIAL_RESEARCH_NOTES.md",
    ROOT / "community" / "external-feedback-synthesis.md",
)

MARKDOWN_URL_PREFIXES = ("https://", "http://")
TRAILING_URL_PUNCTUATION = ".,;:"
USER_AGENT = "category-theory-transformer-rs-reference-check/1.0"


@dataclass(frozen=True)
class LinkTarget:
    path: Path
    url: str


@dataclass(frozen=True)
class LinkResult:
    target: LinkTarget
    ok: bool
    detail: str


Fetch = Callable[[str, int], tuple[int, str]]


def normalize_url(raw_url: str) -> str:
    return raw_url.strip().rstrip(TRAILING_URL_PUNCTUATION)


def extract_urls(text: str) -> list[str]:
    urls: list[str] = []

    index = 0
    while index < len(text):
        starts = [
            candidate
            for candidate in (text.find(prefix, index) for prefix in MARKDOWN_URL_PREFIXES)
            if candidate != -1
        ]
        if not starts:
            break

        start = min(starts)
        end = start
        while end < len(text) and text[end] not in " \t\r\n<>)\"]}":
            end += 1

        url = normalize_url(text[start:end])
        if url:
            urls.append(url)
        index = end + 1

    return urls


def unique_urls(urls: Iterable[str]) -> list[str]:
    seen: set[str] = set()
    unique: list[str] = []

    for url in urls:
        if url in seen:
            continue

        seen.add(url)
        unique.append(url)

    return unique


def load_targets(paths: Iterable[Path]) -> list[LinkTarget]:
    targets: list[LinkTarget] = []

    for path in paths:
        text = path.read_text(encoding="utf-8")
        for url in unique_urls(extract_urls(text)):
            targets.append(LinkTarget(path=path, url=url))

    return targets


def fetch_url(url: str, timeout_seconds: int) -> tuple[int, str]:
    request = urllib.request.Request(
        url,
        headers={
            "User-Agent": USER_AGENT,
            "Accept": "text/html,application/xhtml+xml,application/pdf,*/*;q=0.8",
        },
        method="GET",
    )

    with urllib.request.urlopen(request, timeout=timeout_seconds) as response:
        return response.status, response.url


def check_target(target: LinkTarget, fetch: Fetch, timeout_seconds: int) -> LinkResult:
    if not target.url.startswith("https://"):
        return LinkResult(target, False, "source URL must use https")

    try:
        status, final_url = fetch(target.url, timeout_seconds)
    except urllib.error.HTTPError as error:
        if error.code in {401, 403}:
            return LinkResult(target, True, f"HTTP {error.code} blocked automated access")
        return LinkResult(target, False, f"HTTP {error.code}")
    except urllib.error.URLError as error:
        return LinkResult(target, False, f"URL error: {error.reason}")
    except TimeoutError:
        return LinkResult(target, False, "timed out")
    except OSError as error:
        return LinkResult(target, False, f"network error: {error}")

    if 200 <= status < 400:
        if final_url.startswith("http://"):
            return LinkResult(target, False, f"redirected to insecure URL: {final_url}")
        return LinkResult(target, True, f"HTTP {status}")

    return LinkResult(target, False, f"HTTP {status}")


def run_live_check(paths: Iterable[Path], timeout_seconds: int) -> int:
    targets = load_targets(paths)
    if not targets:
        print("Reference link live check failed: no URLs found.")
        return 1

    results = [check_target(target, fetch_url, timeout_seconds) for target in targets]
    failures = [result for result in results if not result.ok]

    for result in results:
        status = "ok" if result.ok else "fail"
        relative_path = result.target.path.relative_to(ROOT)
        print(f"{status}: {relative_path}: {result.target.url} ({result.detail})")

    if failures:
        print(f"Reference link live check failed: {len(failures)} unreachable source(s).")
        return 1

    print(f"Reference link live check passed: {len(results)} source URL(s) reachable.")
    return 0


def run_self_test() -> int:
    sample = """
    [Rust](https://doc.rust-lang.org/book/)
    duplicate: https://doc.rust-lang.org/book/.
    insecure: http://example.com/path
    [D2L](https://d2l.ai/chapter_linear-classification/softmax-regression.html)
    """
    urls = unique_urls(extract_urls(sample))
    expected_urls = [
        "https://doc.rust-lang.org/book/",
        "http://example.com/path",
        "https://d2l.ai/chapter_linear-classification/softmax-regression.html",
    ]
    if urls != expected_urls:
        print("Reference link live self-test failed: URL extraction mismatch.")
        print(f"expected: {expected_urls!r}")
        print(f"actual:   {urls!r}")
        return 1

    def fake_fetch(url: str, _timeout_seconds: int) -> tuple[int, str]:
        if "missing" in url:
            return 404, url
        return 200, url

    ok_result = check_target(
        LinkTarget(path=ROOT / "book" / "src" / "references.md", url=expected_urls[0]),
        fake_fetch,
        timeout_seconds=1,
    )
    if not ok_result.ok:
        print("Reference link live self-test failed: expected https URL to pass.")
        return 1

    insecure_result = check_target(
        LinkTarget(path=ROOT / "book" / "src" / "references.md", url=expected_urls[1]),
        fake_fetch,
        timeout_seconds=1,
    )
    if insecure_result.ok or "https" not in insecure_result.detail:
        print("Reference link live self-test failed: expected insecure URL to fail.")
        return 1

    missing_result = check_target(
        LinkTarget(path=ROOT / "book" / "src" / "references.md", url="https://example.com/missing"),
        fake_fetch,
        timeout_seconds=1,
    )
    if missing_result.ok or "HTTP 404" not in missing_result.detail:
        print("Reference link live self-test failed: expected missing URL to fail.")
        return 1

    def blocked_fetch(url: str, _timeout_seconds: int) -> tuple[int, str]:
        raise urllib.error.HTTPError(url, 403, "Forbidden", hdrs=None, fp=None)

    blocked_result = check_target(
        LinkTarget(path=ROOT / "book" / "src" / "references.md", url="https://example.com/blocked"),
        blocked_fetch,
        timeout_seconds=1,
    )
    if not blocked_result.ok or "blocked automated access" not in blocked_result.detail:
        print("Reference link live self-test failed: expected HTTP 403 to be treated as blocked.")
        return 1

    print("Reference link live self-test passed.")
    return 0


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--self-test",
        action="store_true",
        help="run deterministic parser/checker tests without network access",
    )
    parser.add_argument(
        "--timeout",
        type=int,
        default=10,
        help="per-link timeout in seconds for live checks",
    )
    parser.add_argument(
        "files",
        nargs="*",
        type=Path,
        help="markdown files to scan; defaults to the reference and editorial source files",
    )
    return parser.parse_args()


def main() -> int:
    args = parse_args()

    if args.self_test:
        return run_self_test()

    paths = tuple(path.resolve() for path in args.files) if args.files else DEFAULT_FILES
    return run_live_check(paths, args.timeout)


if __name__ == "__main__":
    sys.exit(main())
