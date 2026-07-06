# Readiness model — "what score would the student get today, and how sure are we?"

**Rubric §4 / §9 Step 3 / §12 model description.** The projected MCAT score.
Exam: **MCAT** (total 472–528, sections 118–132).

> **Honesty first (this is the auto-fail line).** The projected total is a
> **method with a stated range**, derived from measured per-section signals — it
> is **not** validated against real MCAT outcomes and we do not claim it is. We
> show the range, the coverage, the confidence, and the give-up rule, and we
> label low-evidence projections as such. A number without that context is a
> guess in a nice font; this page is the "what evidence produced the number"
> the rubric requires.

## How it is computed (in the Rust engine)

Per section, `section_score_status()` (`anki/rslib/src/scheduler/concept.rs`)
blends **performance × memory × coverage** against a low **guessing floor**:

```
retention          = section_memory (projected to exam date), default 1.0
retained_fraction  = clamp(coverage · retention, 0, 1)
readiness_center   = retained_fraction · performance_center
                     + (1 − retained_fraction) · guessing_baseline_score(120.0)
                     clamped to [118, 132]
```

So a section only reaches its performance score to the extent it is **covered
and retained**; uncovered/forgotten material is pulled toward the **~120
guessing floor**, not the 125 median (an unstudied topic is not "average").

**Uncertainty** combines four independent SE terms (performance scaled by
coverage; coverage `(1−coverage)·2.0`; mastery `(1−mastery)·2.0`; memory
`(1−retention)·2.0`) in quadrature; the band is `center ± 1.96 · SE`.

**Projected total** (`concept_demo.rs`): sum of the four section centers, with
`total_se = √Σ section_se²` (floored at 1.0); the reported range is
`total ± 1.96 · total_se`, clamped to [472, 528]. If a target is set,
`probability_hit_target` is the normal-tail `P(score ≥ target)`.

**Coverage** (`section_coverage`): blueprint-weighted breadth —
`Σ_discipline weight · min(answered / required, 1)`, with
`required = round(weight / 0.05)`. It rewards breadth across KCs, not grinding
one card.

## Give-up rule (this score)

- **Global:** the readiness-sorted new-card queue only engages after
  `total_seen_cards ≥ readiness_min_seen_cards` (**500** in production; lowered
  for the demo deck).
- **Projection display:** `has_projection` is false until at least one section
  has a graded item; the dashboard hero then **abstains** ("Not enough data yet
  — need N reviews / X% coverage") instead of a number.
- **Per section:** a section shows a readiness range only with
  `enough_evidence` (≥ 20 items **and** ≥ 60% coverage).

Full statement + enforcement points in `docs/give-up-rule.md`.

## Honest uncertainty & evidence (rubric honesty rule)

Every readiness display carries: **point estimate · likely range · % of exam
covered · how-sure · last-updated · top reasons · what's-missing · give-up
state** (see `docs/honesty-rule.md` for where each is surfaced per platform).

- **Method is written down** here and in `docs/research-scoring.md` (the
  compressive, population-anchored θ→scale rationale; floor at the guessing
  baseline; ±band mirroring AAMC's ±1 section / ±2 total).
- **Its inputs are validated:** memory is calibrated (Brier 0.1677) and
  performance beats baselines (acc 0.712 vs 0.572/0.615) on held-out sets, and
  both are locked to the engine by parity (230/230).
- **What's missing (disclosed):** there is **no held-out validation of the final
  projected score** against real students with practice-test outcomes (rubric §9
  Step 4). Per the rubric, an honest "we calibrated the steps but cannot yet
  prove the projected score" is the claim made here.

## Verify

```sh
# the readiness math (blend, floor, coverage cap, memory discount, target prob):
cd anki && CARGO_TARGET_DIR=out/rust cargo test -p anki --features rustls \
    readiness_is_capped_by_partial_coverage full_coverage_lets_readiness_reach_performance \
    memory_discounts_readiness_toward_the_floor probability_at_least_matches_normal_tails
# the ranged, in-scale projection end-to-end from Python:
cd anki && PYTHONPATH=out/pylib ANKI_TEST_MODE=1 out/pyenv/bin/pytest -p no:cacheprovider \
    pylib/tests/test_concept_scheduler_engine.py::test_scores_are_ranged_and_in_scale_via_rpc -q
```

Source: `concept.rs::section_score_status`, `::section_coverage`;
`concept_demo.rs` projected-total block.
