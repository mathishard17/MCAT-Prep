# The give-up rule (write yours down)

**Rubric §4 give-up rule / §7c.** "A good system knows when it does not know."
This is our line, stated once, and where it is enforced. Exam: **MCAT**.

## The rule (as shipped)

The app **refuses to show a score** when it lacks evidence, at three levels:

1. **Readiness projection (whole exam).** No projected 472–528 total is shown
   until at least one section has a graded item (`has_projection`). The
   readiness-*ordering* of new cards additionally waits until the deck has
   **≥ 500 graded card-answers** (`readiness_min_seen_cards`, production
   default; the bundled demo deck lowers this so the feature is demoable).
2. **Per-section performance & readiness range.** A section shows a number only
   with **≥ 20 graded items** (`irt_min_section_items`) **and** **≥ 60% blueprint
   coverage** (`irt_min_section_coverage`). Otherwise `enough_evidence = false`.
3. **Memory.** Abstains per KC/section when no studied card is tagged there
   (`has_memory = false`).

Below the line the UI shows **"Not enough data yet — need N reviews / X%
coverage"**, never a number.

> **Note on the threshold choice.** `SUNDAY-PLAN.md` floated an *example* line of
> "200 reviews / 50% coverage". The shipped line is **stricter** (500 answers
> globally; 20 items + 60% coverage per section). Either is a valid "clear line";
> this doc records the one actually enforced in code, so the numbers on screen
> match the numbers here.

## Where it is enforced

### Backend (Rust — single source of truth)

| Gate | Code | Emitted to UI as |
| --- | --- | --- |
| Global readiness evidence | `concept.rs::readiness_evidence_status` (`readiness_min_seen_cards = 500`) | `evidence.seen_cards`, `evidence.required_seen_cards` |
| Queue readiness sort gated | `scheduler/queue/builder/sorting.rs` (skips the sort when insufficient) | (queue behavior) |
| Per-section evidence | `concept.rs::section_score_status` → `enough_evidence = answered ≥ 20 && coverage ≥ 0.60` | `ConceptSectionScore.enough_evidence` |
| Any projection at all | `concept_demo.rs` → `has_projection = any section answered_items > 0` | `has_projection` |
| Memory present | `concept_demo.rs` → `has_memory` | `overall_memory`, `section_has_memory` |

### Desktop (`anki/qt/aqt/mcat_ui.py`, `ts/.../ConceptSchedulerOptions.svelte`)

- Dashboard **hero abstains** when `has_projection` is false (`mcat_ui.py` ~L225–230).
- Section cards tag low-evidence sections **"building evidence"** when
  `enough_evidence` is false (`mcat_ui.py` ~L394).
- The deck-options Concept tab shows performance/readiness ranges **only** when
  `enough_evidence && coverage ≥ 0.6` (`ConceptSchedulerOptions.svelte`).

### Android (`.../conceptscheduler/McatHonesty.kt`, `ConceptSchedulerStatusScreen.kt`)

- Shared abstain copy: `abstainMessage(reviewsNeeded, coveragePct)` →
  `"Not enough data yet — need N reviews / X% coverage"`.
- `MIN_COVERAGE_TO_SHOW_SCORES = 0.60f`; the hero renders `AbstainHero` when
  `!hasProjection`; each score tile abstains via its `available` flag
  (memory: `hasMemory`; performance: any section `enoughEvidence`; readiness:
  `hasProjection`).

### Known gap (disclosed for honesty)

On the **desktop dashboard**, per-section cards currently still *print* a
readiness number below 60% coverage with only a "building evidence" tag, whereas
**Android** hides the number below 60%. The hero and the deck-options tab abstain
correctly on both. This desktop section-card behavior is the remaining item in
`SUNDAY-PLAN.md` §10b.

## Verify

```sh
# Backend refuses when evidence is thin:
cd anki && CARGO_TARGET_DIR=out/rust cargo test -p anki --features rustls \
    readiness_status_refuses_when_evidence_is_thin section_coverage_is_capped_by_discipline_weight
# Fresh deck starts below the line (abstain precondition), from Python:
cd anki && PYTHONPATH=out/pylib ANKI_TEST_MODE=1 out/pyenv/bin/pytest -p no:cacheprovider \
    pylib/tests/test_concept_scheduler_engine.py::test_import_demo_deck_rpc_builds_knowledge_graph -q
# Android abstain wording + below-line behavior:
cd Anki-Android && ./gradlew :AnkiDroid:testPlayDebugUnitTest \
    --tests "com.ichi2.anki.conceptscheduler.McatHonestyTest"
```
