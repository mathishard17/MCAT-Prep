# MCAT Prep -- study-feature ablation (rubric §8 / SUNDAY-PLAN §4)

> **Auto-generated** by `evals/ablation.py` from `evals/fixtures/ablation.json`.
> Every number is produced end-to-end by the **real Anki Rust engine**
> (`Collection::build_queues` + `Collection::answer_card`); this script only
> renders and re-checks. Do not edit by hand.

## The feature and the pre-registered hypothesis

**Feature under test:** the concept scheduler (topic-aware new-card ordering).

**Hypothesis (written down before running):** Ordering new cards so prerequisites come first (the concept scheduler) reduces how often a learner is served a card whose prerequisites are not yet learned, and under a stated scaffolding assumption makes equal study time more effective.

**Main number (pre-stated):** `prerequisite_violations` -- how often a learner is
served a card whose prerequisites are not yet met. This is model-independent:
it reuses the engine's own `mastery_for` and `outer_fringe_prereq_mastery`
(0.70) threshold, applied identically to all three arms.

**Three builds, same learners, same cards, equal study time:**

- **ON** -- full app, concept scheduler on (readiness-sorted new cards).
- **OFF** -- the same app with the concept scheduler turned off (the ablation).
- **PLAIN** -- plain, unmodified Anki (stock deck config, no concept flag).

**Setup:** 12 synthetic learners (seeds `20260705..20260716`); a 192-card deck of 64 KCs across 4 disciplines (reverse-topological (deepest-first): adversarial for the plain gather order); equal budget of **56 answered cards** per arm.

## Result -- main number (prerequisite violations)

![Prerequisite violations by arm](ablation.svg)

| Build | Prerequisite violations (mean [min, max]) |
| --- | ---: |
| **Concept scheduler ON** | **8.5 [8.0, 9.0]** |
| Feature OFF (ablation) | 56.0 [56.0, 56.0] |
| Plain Anki | 56.0 [56.0, 56.0] |

Turning the concept scheduler on cut prerequisite violations by **85%** vs both the ablation and plain Anki (mean 8.5 vs 56.0), on **every** one of 12 seeds.

## Result -- secondary metrics (scaffolding-conditioned)

These assume the stated scaffolding effect (a card studied with unmet
prerequisites is learned less effectively). Reported with ranges; see the null
control below for the honest counter-case.

| Metric | ON | OFF | PLAIN | ON - PLAIN |
| --- | ---: | ---: | ---: | ---: |
| Held-out exam accuracy | 0.412 [0.409, 0.444] | 0.225 [0.225, 0.225] | 0.225 [0.225, 0.225] | **+0.187** |
| Projected readiness (472-528) | 489.6 [486.9, 490.7] | 480.3 [479.6, 481.2] | 480.3 [479.6, 481.2] | **+9.3** |
| Memory (mean recall) | 0.762 [0.716, 0.789] | 0.371 [0.289, 0.447] | 0.371 [0.289, 0.447] | **+0.391** |
| Coverage (KCs touched) | 0.301 [0.297, 0.344] | 0.297 [0.297, 0.297] | 0.297 [0.297, 0.297] | -- |

Readiness range per arm (95% band, engine-computed): ON 481-499, PLAIN 472-491.

## Honest null result (no prerequisite structure)

On an identically-shaped deck with **no prerequisite edges**, the feature has
nothing to defer, so its headline benefit vanishes:

| Build | Prerequisite violations |
| --- | ---: |
| ON | 0.0 |
| OFF | 0.0 |
| PLAIN | 0.0 |

All three arms record **zero** prerequisite violations -- the concept scheduler
only helps when the deck actually has a prerequisite structure. (Its readiness
sort still spreads study across topics, so coverage/accuracy can differ, but the
prerequisite-deferral effect under test is genuinely inert here.)

## Engine fidelity + honesty

- **Engine cross-check:** the ON-arm violation count reported here equals the engine's OWN `prerequisite_violations` counter for every seed (PASS) -- the eval is measuring the shipped engine, not a model of it.
- Secondary metrics (accuracy/readiness/memory) assume a scaffolding learning effect: a card studied with unmet prerequisites is learned less effectively. The MAIN number (prerequisite violations) does NOT depend on this assumption.
- Learners are synthetic and seeded; real-student validation (rubric Step 4) is
  future work.

## Pre-committed cutoff

| Check | Result |
| --- | :---: |
| per_seed ON < OFF and ON < PLAIN | PASS |
| mean(ON) <= 0.5 * mean(OFF) | PASS |
| ON observer == engine counter | PASS |
| null control: zero violations all arms | PASS |

**Overall: ALL CHECKS PASS.**

Reproduce:

```sh
# 1) emit the engine results
cd anki && CARGO_TARGET_DIR=out/rust cargo test -p anki --features rustls \
    emit_ablation_fixture -- --nocapture
# 2) render + re-check
cd .. && python evals/ablation.py
```
