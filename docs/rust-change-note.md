# Rust change — the Concept Scheduler (why it lives in Rust, not Python)

**Rubric §7a deliverable.** Exam: **MCAT** (472–528).

## What the change is

The **Concept Scheduler** is a real modification of Anki's Rust engine
(`anki/rslib`). It adds MCAT-aware scheduling and scoring *inside the engine*:

- **Bayesian per-KC mastery** updated on every answer
  (`maybe_update_concept_mastery_from_answer`), from note tags
  (`KC::…`, `Prereq::…`, `MCAT::…`, `Difficulty::…`, `IRT::…`).
- **A new-card queue order** that sorts by a *readiness* score
  (`prerequisite_mastery × (1 − target_mastery)`) and **defers cards whose
  prerequisites are unmet**, integrated into the real queue builder
  (`scheduler/queue/builder/`), not a wrapper around it.
- **A 4:1 review-to-new session budget** and topic-focused blocks
  (`CONCEPT_REVIEWS_PER_NEW_SLOT = 4`, `CONCEPT_FOCUSED_BLOCK_SIZE = 3`).
- **Three scores** computed in-engine: Memory (FSRS retrievability),
  Performance (IRT 3PL θ → 118–132), Readiness (coverage×retention blend →
  472–528, with a range).
- **Five new protobuf RPCs** on `SchedulerService`
  (`GetConceptSchedulerStatus`, `ImportMcatDemoDeck`, `SetConceptSelectedTopic`,
  `SetConceptExamSettings`, `GetConceptLesson`).

Core files (new): `rslib/src/scheduler/concept.rs` (2172 lines),
`rslib/src/scheduler/concept_demo.rs` (1646 lines). Integration hooks live in
existing engine files (see `docs/rust-upstream-files.md`).

## Why this belongs in Rust, not Python

1. **One engine, two apps (rubric §2/§3).** The desktop app and the Android
   companion share the *same* Rust backend (AnkiDroid runs it through the
   `rsdroid` bridge). Putting scheduling + scoring in Rust means the change
   ships to **both** platforms from one source of truth. The identical
   `get_concept_scheduler_status` RPC drives the desktop dashboard
   (`qt/aqt/mcat_ui.py`) and the Android dashboard
   (`conceptscheduler/ConceptSchedulerStatusScreen.kt`). A Python implementation
   would never run on the phone — the rubric explicitly says reimplementing the
   scheduler in Swift/JS/Python "does not count".

2. **Scheduling *is* the engine.** New-card ordering, burying, and the review
   queue are built in `rslib/src/scheduler/queue/`. To change *which card comes
   next* you must change the queue builder itself. Doing it in Python could only
   re-sort cards after the fact, which fights the engine and breaks its
   invariants (limits, burying, learning steps).

3. **Undo and no-corruption are Rust guarantees.** Anki's undo and transactional
   writes are implemented in Rust (`OpChanges` / undo stack). Because the
   mastery update is recorded through the same mechanism, **undo restores it for
   free** and the collection stays consistent. See the undo proof below. A
   Python-side side-table would not participate in undo and could desync from the
   collection (i.e. corruption).

4. **Speed on 50k cards.** Mastery updates on answer and the per-section
   readiness aggregation run in the hot review/queue-build path. Native Rust over
   the collection keeps button-ack and next-card latency inside the rubric §10
   targets without a Python round-trip per card.

5. **Faithfulness is provable.** Because the math lives in one Rust place, the
   held-out Python evals can be **locked to the engine**: a Rust test emits
   reference values and a Python test reproduces them to 1e-9
   (`evals/test_parity.py`, 230/230). That parity is only meaningful because the
   engine is the single source of truth.

## Tests (≥3 Rust unit tests + 1 test from Python)

- **60 Rust unit tests** cover the change (tags, graph, Bayesian mastery, fringe
  classification, IRT, coverage, readiness, session budget, queue sorting,
  undo, and the RPC). Verify the count:

```sh
cd anki && CARGO_TARGET_DIR=out/rust cargo test -p anki --features rustls concept 2>&1 | tail -1
# => test result: ok. 60 passed; 0 failed; ...
```

- **4 Python tests call the change through the real backend RPCs**
  (`anki/pylib/tests/test_concept_scheduler_engine.py`): import the demo deck via
  `import_mcat_demo_deck`, answer cards through `col.sched.answerCard`, and read
  `get_concept_scheduler_status` — asserting mastery updates, ranged in-scale
  scores, undo, and collection integrity.

```sh
cd anki && PYTHONPATH=out/pylib ANKI_TEST_MODE=1 out/pyenv/bin/pytest \
    -p no:cacheprovider pylib/tests/test_concept_scheduler_engine.py -q
# => 4 passed
```

## Undo-safe + no-corruption proof

Two independent, re-runnable proofs that a review can be undone with the
collection left intact:

- **Rust:** `scheduler::answering::test::undo_restores_concept_mastery_update`
  answers a card (mastery + `total_seen_cards` move), calls `col.undo()`, and
  asserts the concept state is emptied again.
- **Python (end-to-end):**
  `test_review_then_undo_keeps_collection_intact` reviews one card, undoes it,
  asserts `total_seen_cards` returns to 0 and every KC's `answered` returns to 0,
  then runs `col.fix_integrity()` and asserts `ok is True` (no corruption).

```sh
# Rust undo test
cd anki && CARGO_TARGET_DIR=out/rust cargo test -p anki --features rustls \
    undo_restores_concept_mastery_update -- --nocapture
# Python review+undo+integrity
cd anki && PYTHONPATH=out/pylib ANKI_TEST_MODE=1 out/pyenv/bin/pytest \
    -p no:cacheprovider \
    pylib/tests/test_concept_scheduler_engine.py::test_review_then_undo_keeps_collection_intact -q
```

`queue/undo.rs` is also part of the touched surface (the queue's session state
participates in undo), so undoing an answer restores both the collection *and*
the concept-scheduler session.

## See also

- `docs/rust-upstream-files.md` — every upstream file touched + future-merge difficulty.
- `evals/ENGINE-FIDELITY.md` — the in-engine prerequisite-violation ablation and Python↔Rust parity.
- `docs/model-memory.md`, `docs/model-performance.md`, `docs/model-readiness.md` — the three score models + give-up rule.
