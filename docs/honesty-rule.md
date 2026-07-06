# Honesty-rule completeness (per score)

**Rubric §1 "the honesty rule" / §4 line 79.** Every score must ship with:
point estimate · likely range · **% of exam covered** · **how-sure** ·
**last-updated** · **main reasons** · **what's missing** · **give-up state** ·
the **single best next thing to study** · and **how accurate past guesses were**
(calibration). This page audits, honestly, which of those are present for each of
the three scores and on which surface.

Legend: ✅ present · 🟡 partial / derived elsewhere · ❌ not present.
Surfaces: **BE** = Rust backend (proto) · **DDash** = desktop dashboard
(`mcat_ui.py`) · **DRev** = desktop reviewer sidebar (`reviewer.py`) ·
**DOpt** = deck-options Concept tab (`ConceptSchedulerOptions.svelte`) ·
**AND** = Android (`McatHonesty.kt` / `ConceptSchedulerStatusScreen.kt`).

## Memory

| Field | BE | DDash | DRev | DOpt | AND |
| --- | :-: | :-: | :-: | :-: | :-: |
| Point estimate | ✅ | ✅ | ✅ | ✅ | ✅ |
| Range | 🟡 (min–max of sections) | ❌ | ❌ | ❌ | ✅ |
| % covered | ✅ | ✅ | ❌ | ✅ | ✅ |
| How-sure | ❌ (UI-derived) | ❌ | ❌ | ❌ | ✅ `howSureLabel` |
| Last-updated | 🟡 (recomputed per fetch) | ❌ | ❌ | ❌ | ✅ `lastUpdatedText` |
| Top reasons | ❌ | ❌ | ❌ | ❌ | ✅ `topReasonFor` |
| What's missing | ❌ | ❌ | ❌ | ❌ | ✅ `whatsMissingText` |
| Give-up / abstain | ✅ `has_memory` | 🟡 "—" | 🟡 omits | 🟡 "—" | ✅ full |

## Performance

| Field | BE | DDash | DRev | DOpt | AND |
| --- | :-: | :-: | :-: | :-: | :-: |
| Point estimate | ✅ `performance_center`, `theta` | ❌ | 🟡 (shows accuracy) | ✅ (gated) | ✅ |
| Range | ✅ lower/upper/SE | ❌ | ❌ | ✅ (gated) | ✅ |
| % covered | ✅ | ✅ | ❌ | ✅ | ✅ |
| How-sure / last-updated / reasons / missing | ❌ | ❌ | ❌ | ❌ | ✅ |
| Give-up / abstain | ✅ `enough_evidence` | 🟡 | ❌ | ✅ | ✅ |

## Readiness

| Field | BE | DDash | DRev | DOpt | AND |
| --- | :-: | :-: | :-: | :-: | :-: |
| Point estimate (472–528 + per section) | ✅ | ✅ hero + sections | ✅ | ✅ (gated) | ✅ hero + tile |
| Range | ✅ ±1.96·SE | ✅ | ✅ | ✅ (gated) | ✅ |
| % covered | ✅ | ✅ | ❌ | ✅ | ✅ |
| How-sure | ❌ | ❌ | ❌ | ❌ | ✅ |
| Last-updated | 🟡 | ❌ | ❌ | ❌ | ✅ |
| Top reasons | ❌ | ❌ | ❌ | ❌ | ✅ |
| What's missing | ❌ | ❌ | ❌ | ❌ | ✅ |
| Target probability | ✅ `probability_hit_target` | ✅ | ❌ | ❌ | ✅ |
| Give-up / abstain | ✅ `has_projection` | 🟡 hero only | 🟡 | ✅ | ✅ |

## Cross-cutting honesty fields

- **Best next thing to study.** Present as backend **recommendations**
  (`recommended_topics` → `ConceptTopicRecommendation`), surfaced on both
  dashboards ("Recommended next" / knowledge-map "next-up").
- **How accurate past guesses were (calibration).** Global, not per-user:
  memory Brier **0.1677** / ECE **0.0617** and performance acc **0.712** / AUC
  **0.769** with baseline lift, from the held-out evals (`evals/MODEL-EVALS.md`,
  `evals/memory-calibration.svg`). Linked from the model pages.

## Honest summary

- **Android is complete** — all rubric honesty fields render for all three
  scores (see `McatHonesty.kt` builders and `McatHonestyTest.kt`).
- **Desktop dashboard is now complete too** — the per-score detail cards
  (tap-to-expand) plus the readiness hero render **how-sure, last-updated, top
  reasons, and what's-missing** (`_how_sure` / `_top_reason` / `_whats_missing` /
  `_score_detail_html`, and the hero how-sure chip + "% of exam covered" in
  `anki/qt/aqt/mcat_ui.py`), with wording matching `McatHonesty.kt`.
  > Note: the per-score tables above predate this and still mark the **DDash**
  > column ❌ for how-sure / last-updated / top-reasons / what's-missing — treat
  > this summary as the current state (those cells are now ✅ via the detail cards).
- **Remaining lighter surface:** the reviewer **sidebar** shows the scores +
  ranges but not the full honesty fields — the dashboard is the complete view.
- The backend emits the raw fields (`coverage`, SE/range, evidence flags), so this
  was UI wiring, not new modeling.

## Verify

```sh
# Android honesty fields + wording:
cd Anki-Android && ./gradlew :AnkiDroid:testPlayDebugUnitTest \
    --tests "com.ichi2.anki.conceptscheduler.McatHonestyTest"
# Backend emits ranged, in-scale, coverage-bearing scores:
cd anki && PYTHONPATH=out/pylib ANKI_TEST_MODE=1 out/pyenv/bin/pytest -p no:cacheprovider \
    pylib/tests/test_concept_scheduler_engine.py::test_scores_are_ranged_and_in_scale_via_rpc -q
```
