# MCAT Prep -- Section 7 reliability: crash-safety + offline degradation

Two one-command resilience tests over the **real Rust backend** (Python -> `out/pylib` `_rsbridge.so` -> bundled SQLite), covering the two durability/availability claims a study app must keep: an abrupt kill mid-review never corrupts the collection, and losing the network never breaks the app (AI degrades cleanly; the three scores still render because they are local Rust math). Reproduce with:

```sh
just crash-test      # 20x SIGKILL mid-review -> reopen + backend integrity check
just offline-test    # network forced off -> AI fails fast/clean + scores still compute
# direct (what the recipes run):
# cd anki && PYTHONPATH=out/pylib out/pyenv/bin/python ../evals/crash_test.py
# cd anki && PYTHONPATH=out/pylib out/pyenv/bin/python ../evals/offline_test.py
```

> **HONESTY -- what these cover and what they don't.** Both harnesses exercise the **desktop backend on the macOS host**: the same shared Rust `rslib` + SQLite write path AnkiDroid also ships via `Anki-Android-Backend`, so the algorithmic durability/locality guarantees carry over -- but **neither test runs on an Android device/emulator**, so on-device crash-injection and the Android (Kotlin) AI client are a **disclosed follow-up**, not something claimed here. §10 asks for **both** platforms; this is desktop-only evidence for a cross-platform guarantee. The crash test proves **process-death** durability (the atomicity SQLite transactions give you against SIGKILL / force-quit / OOM-kill / power loss); it does **not** simulate hardware/disk-level corruption (torn writes, bad sectors). The offline test forces offline **in-process** (monkeypatched `urllib.request.urlopen`, blocked `socket.socket`) rather than pulling a real cable, so it proves the **code path** degrades cleanly and that score computation touches no Python socket -- it is not a captive-portal / flaky-DNS field test.

**Machine:** macOS-15.7.4-arm64-arm-64bit-Mach-O (arm64, 10 CPU). **Python:** 3.13.13. **Decks:** 2,000-card fixture (crash), 500-card fixture (offline scores), both cached under `evals/.bench_cache/`; concept graph 120 nodes / 114 edges. **Run date:** 2026-07-05 19:45 CDT.

## 1. Crash test -- durability under abrupt mid-review kill

**What it does.** Builds/uses a cached 2,000-card MCAT deck. For each of **20** iterations it `shutil.copy`s the fixture to a throwaway temp path, spawns a child (the app's pyenv Python running `_crash_child.py`) that opens the copy and continuously answers due cards through the real **v3 scheduler** (`col.sched.answerCard` -> Rust backend committing card + revlog writes), waits for the child's `READY` handshake so the kill lands **mid-review** (not during startup), sleeps a random **0.2-0.8 s**, then `os.kill(pid, SIGKILL)` (uncatchable -- no cleanup/flush). The parent then reopens the killed copy and runs the **real backend integrity check** (`Collection.fix_integrity` -> `check_database`, an fsck of the SQLite collection), asserting no corruption, a clean reopen, and a `card_count` consistent with the pristine deck.

**Result (captured 2026-07-05):**

| check | result | target | status |
| --- | ---: | --- | :---: |
| mid-review SIGKILLs delivered | 20 | 20 | -- |
| kills that landed mid-review (not startup / early exit) | 20 / 20 | 20 / 20 | PASS |
| collections that reopened cleanly | 20 / 20 | 20 / 20 | PASS |
| backend integrity check (`fix_integrity`) | 0 problems | 0 problems | PASS |
| `card_count` consistent after kill | 20 / 20 (= 2000) | = 2000 | PASS |
| **corrupted collections** | **0** | **0** | **PASS** |

Real writes were in flight at kill time: the child had committed **245-876** revlog rows (varies per iteration with the random kill window) before each SIGKILL, so the kill genuinely interrupted active review commits rather than an idle process.

**Overall: PASS -- `20/20 clean`, 0 corrupted collections** (harness exits 0).

**Covers / does not cover.** Covers desktop process-death durability of the shared Rust + SQLite write path. Does **not** cover on-device Android crash-injection (same Rust core, not exercised here -- follow-up) or hardware/media-level corruption. 20 sub-second trials is a strong repeated-trial signal, not an exhaustive proof.

## 2. Offline test -- no network (AI degrades cleanly + scores stay local)

**What it does.** Two independent, hermetic checks, neither of which makes a real network call:

1. **AI degrades cleanly.** Loads the stdlib-only `aqt/mcat_ai_core.py` by file path (so Qt is never imported), monkeypatches `urllib.request.urlopen` to raise `URLError` (forced offline), then calls `test_connection`, `reword_question`, and `questions_equivalent`. Each must raise a clean `OpenAIError` **fast** (bounded at 15 s -- proving no hang) and never leak a raw traceback.
2. **Scores work offline.** Blocks any new Python `socket.socket`, then computes the concept-scheduler status (`get_concept_scheduler_status`) on a cached 500-card deck, asserting a populated graph + a per-section score projection -- proving the three scores are **local Rust/SQLite math**, not a server call.

**Result (captured 2026-07-05):**

| check | result | bound / expectation | status |
| --- | --- | --- | :---: |
| `test_connection` offline | clean `OpenAIError` in 0 ms | fail fast, &lt; 15000 ms, no traceback | PASS |
| `reword_question` offline | clean `OpenAIError` in 0 ms | fail fast, &lt; 15000 ms, no traceback | PASS |
| `questions_equivalent` offline | clean `OpenAIError` in 0 ms | fail fast, &lt; 15000 ms, no traceback | PASS |
| scores compute with sockets blocked | graph 120 nodes / 114 edges, 4 section projections, `projected_total` = 480 | non-empty graph + projection, no network | PASS |

The AI errors surface as a user-facing `OpenAIError` (`Network error: forced offline by offline_test`) -- the app catches this and shows a degraded message instead of crashing; the scores render from local data with the network hard-blocked.

**Overall: PASS -- `AI degrades cleanly + scores present offline`** (harness exits 0).

**Covers / does not cover.** Covers the **desktop** AI client (`mcat_ai_core.py`, Python) degradation path and the fact that score computation uses no Python socket. Does **not** cover the Android (Kotlin) AI client (`OpenAIClient.kt`, separate implementation -- follow-up), nor real-world network-flakiness modes (captive portal, DNS timeout, TLS reset); it forces offline in-process rather than physically pulling the network.
