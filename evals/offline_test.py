#!/usr/bin/env python3
# Copyright: Ankitects Pty Ltd and contributors
# License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html

"""Section 7 offline test: AI degrades cleanly + scores stay present with NO network.

Two independent, hermetic checks (neither one makes a real network call):

1. AI degrades cleanly. With ``urllib.request.urlopen`` monkeypatched to raise
   ``URLError`` (forced offline), the OpenAI client's ``test_connection`` /
   ``reword_question`` / ``questions_equivalent`` must each fail FAST and CLEANLY:
   raise ``OpenAIError`` inside a bounded time (no hang), never leaking a raw
   traceback. The client is loaded from the stdlib-only ``aqt/mcat_ai_core.py``
   directly by file path, so Qt (and the rest of ``aqt``) is never imported.

2. Scores work offline. Via the bench fixture, ``status_for(col)`` returns a
   concept-scheduler status with a populated graph and score projection while a
   guard makes any Python socket raise, proving the computation is local.

Run under the app's pyenv (the scores check needs the built anki backend):

    cd anki && PYTHONPATH=out/pylib out/pyenv/bin/python ../evals/offline_test.py

or via the recipe from the repo root:

    just offline-test

Prints "AI degrades cleanly + scores present offline" and exits 0 on success;
prints the failure and exits non-zero on any failure.
"""

from __future__ import annotations

import importlib.util
import os
import socket
import sys
import time
import urllib.error
import urllib.request
from types import ModuleType

REPO = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
ANKI = os.path.join(REPO, "anki")
# The stdlib-only AI core. We load it by file path (not `import aqt.mcat_ai_core`)
# on purpose: importing it through the `aqt` package would run aqt/__init__.py,
# which imports Qt. This file has no Qt / no `aqt` imports, so it loads clean.
CORE_PATH = os.path.join(ANKI, "qt", "aqt", "mcat_ai_core.py")

# A clean offline failure is milliseconds; anything approaching this bound means
# the call hung instead of degrading fast.
FAIL_FAST_BOUND_S = 15.0


def _load_mcat_ai_core() -> ModuleType:
    """Import ONLY the stdlib-only mcat_ai_core module, by file path, so we never
    trigger aqt/__init__.py (which pulls in Qt). No Qt needed for the AI part."""
    spec = importlib.util.spec_from_file_location("mcat_ai_core", CORE_PATH)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"cannot load mcat_ai_core from {CORE_PATH}")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


class _BlockNetwork:
    """Context manager: any new Python socket raises, so a block can prove it does
    no network I/O. (The Rust status computation is local SQLite math and does not
    use Python sockets, so this only fires if something unexpectedly reaches out.)"""

    def __init__(self, label: str) -> None:
        self.label = label
        self._orig = socket.socket

    def __enter__(self) -> "_BlockNetwork":
        self._orig = socket.socket

        def _blocked(*_args: object, **_kwargs: object) -> socket.socket:
            raise AssertionError(f"network access attempted during {self.label}")

        socket.socket = _blocked  # type: ignore[assignment,misc]
        return self

    def __exit__(self, *_exc: object) -> bool:
        socket.socket = self._orig  # type: ignore[assignment,misc]
        return False


def check_ai_degrades_offline() -> None:
    """Force offline and assert every network-touching client method fails fast and
    cleanly with OpenAIError (no hang, no unhandled traceback)."""
    core = _load_mcat_ai_core()
    open_ai_client = core.OpenAIClient
    open_ai_error = core.OpenAIError

    def _offline_urlopen(*_args: object, **_kwargs: object) -> object:
        raise urllib.error.URLError("forced offline by offline_test")

    original_urlopen = urllib.request.urlopen
    urllib.request.urlopen = _offline_urlopen  # type: ignore[assignment]
    try:
        # A real-looking key so we get past the "no key" guard and actually reach
        # the (now offline) network path. The value is fake and never sent.
        client = open_ai_client("sk-offline-test-fake", "gpt-4o-mini", timeout=5)
        cases = [
            ("test_connection", client.test_connection),
            (
                "reword_question",
                lambda: client.reword_question(
                    "A 0.10 M solution of a strong monoprotic acid is prepared. "
                    "What is its pH?"
                ),
            ),
            (
                "questions_equivalent",
                lambda: client.questions_equivalent(
                    "What is the pH of the solution?",
                    "Determine the solution's pH.",
                ),
            ),
        ]
        for name, call in cases:
            start = time.monotonic()
            try:
                call()
            except open_ai_error as exc:
                elapsed = time.monotonic() - start
                if elapsed > FAIL_FAST_BOUND_S:
                    raise AssertionError(
                        f"{name} took {elapsed:.1f}s (> {FAIL_FAST_BOUND_S}s): it hung"
                    )
                print(
                    f"  ok  {name}: clean OpenAIError in {elapsed * 1000:.0f} ms "
                    f"({exc})"
                )
            except Exception as exc:  # noqa: BLE001 - any non-OpenAIError is a failure
                raise AssertionError(
                    f"{name} raised {type(exc).__name__}, not OpenAIError: {exc}"
                ) from exc
            else:
                raise AssertionError(
                    f"{name} did not fail offline (expected OpenAIError)"
                )
    finally:
        urllib.request.urlopen = original_urlopen  # type: ignore[assignment]


def check_scores_offline() -> None:
    """Assert the concept-scheduler status (graph + score projection) computes with
    no network, using a small cached bench deck."""
    sys.path.insert(0, os.path.join(REPO, "evals"))
    import _bench_fixture as fx

    path = fx.build(500)  # cached after the first run; local SQLite only
    col = fx.open_collection(path)
    try:
        with _BlockNetwork("offline scores computation"):
            status = fx.status_for(col)
            enabled = bool(status.enabled)
            nodes = len(status.graph.nodes)
            edges = len(status.graph.edges)
            sections = len(status.section_scores)
            projected = float(status.projected_total)
        assert enabled, "concept scheduler should be enabled on the bench deck"
        assert nodes > 0, "expected a non-empty concept graph offline"
        assert sections > 0, "expected a per-section score projection offline"
        assert projected > 0, "expected a projected MCAT total offline"
        print(
            f"  ok  scores offline: graph={nodes} nodes / {edges} edges, "
            f"{sections} section projections, projected_total={projected:.0f}"
        )
    finally:
        col.close()


def main() -> int:
    print("Section 7 offline test (no network): AI degradation + offline scores")
    try:
        print("[1/2] AI degrades cleanly when offline ...")
        check_ai_degrades_offline()
        print("[2/2] Scores present offline (network blocked) ...")
        check_scores_offline()
    except Exception as exc:  # noqa: BLE001 - report any failure and exit non-zero
        print(f"\nFAIL: {exc}")
        return 1
    print("\nAI degrades cleanly + scores present offline")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
