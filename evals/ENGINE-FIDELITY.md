# MCAT Prep — engine fidelity

Two evidence artifacts that tie the project's claims to the **REAL shipped Rust
engine** (`anki/rslib/src/scheduler/`), not a Python re-implementation:

1. **Prerequisite-violation ablation** — the study-feature ablation (rubric §8),
   measured inside the engine's actual new-card queue builder.
2. **Python↔Rust parity** — proof that the held-out model evals
   (`performance_eval.py`, `calibration.py`) reproduce the engine's own formulas.

Both are backed by additive Rust tests; nothing in the app or eval logic was
changed to make them pass.

---

## Prerequisite-violation ablation (scheduler ON vs OFF)

**What it demonstrates:** with the concept scheduler **ON**, the new-card queue is
sorted by *readiness*, so cards whose prerequisites are still **unmet** are
**deferred**; with it **OFF**, the queue keeps the plain gather order and serves
those prerequisite-violating cards early. This is the concept-scheduler ablation,
measured end-to-end through the real engine (`Collection::build_queues`), not a
model of it.

**Where it lives:** `anki/rslib/src/scheduler/queue/builder/mod.rs`, test
`concept_scheduler_defers_unmet_prerequisite_new_cards_ablation` (mirrors the
existing `concept_enabled_sorts_new_cards_by_readiness` /
`concept_disabled_preserves_new_card_gather_order`).

**Setup (in-engine):**

- Persisted mastery is seeded around the engine threshold
  `ConceptSchedulerConfig::outer_fringe_prereq_mastery = 0.70`:
  `Biochem::Bioenergetics = 0.85` (a **met** prerequisite, `≥ 0.70`) and
  `Biochem::Glycolysis = 0.20` (an **unmet** prerequisite, `< 0.70`).
- Gather order (`LowestPosition` = insertion order) places **4 unmet-prerequisite
  targets first** (each `Prereq::Biochem::Glycolysis`), then **4 met-prerequisite
  targets** (each `Prereq::Biochem::Bioenergetics`).
- The queue is built with `deck.normal_mut().concept_scheduler_enabled = true`,
  then rebuilt with `= false`.
- Among the first **K = 4** served new cards, we count how many target a KC whose
  prerequisites are unmet. The count reuses the engine's own
  `ConceptSchedulerState::mastery_for` and the `outer_fringe_prereq_mastery`
  threshold — it is not a hand-maintained label.

The engine's readiness score is `prerequisite_mastery × (1 − target_mastery)`, so
met-prerequisite targets score `0.85 × 0.80 = 0.68` and unmet ones score
`0.20 × 0.80 = 0.16`; the ON sort floats the ready cards to the front.

**Result (captured from the test, `-- --nocapture`):**

```
PREREQ-VIOLATION ABLATION (first 4 served new cards): scheduler ON unmet-prereq cards = 0; OFF unmet-prereq cards = 4
```

| Concept scheduler | Unmet-prerequisite cards in first 4 served |
| --- | :---: |
| **ON** | **0** |
| **OFF** | **4** |

The ON count is **strictly less** than the OFF count (`0 < 4`): turning the feature
on removed every prerequisite-violating card from the front of the queue in this
deck, deferring them behind the cards the learner is actually ready for.

**Re-run:**

```sh
cd anki && CARGO_TARGET_DIR=out/rust cargo test -p anki --features rustls \
    concept_scheduler_defers_unmet_prerequisite_new_cards_ablation -- --nocapture
```

---

## 3-build study-session ablation (ON vs OFF vs plain Anki)

**What it demonstrates:** the full rubric §8 ablation — the same synthetic learners
study the same deck for an **equal number of answered cards** under three builds
(concept scheduler **ON**, the feature **OFF**, and **plain Anki**), driven
end-to-end through the real engine (`Collection::build_queues` +
`Collection::answer_card`). The **pre-stated main number** is the count of
**prerequisite violations**; secondary metrics (held-out accuracy, projected
readiness with a range, memory) are reported too, along with an honest **null**.

**Where it lives:** `anki/rslib/src/scheduler/concept_ablation.rs`
(`concept_scheduler_reduces_prerequisite_violations_three_build_ablation`,
`concept_scheduler_ablation_secondary_gains_hold`,
`concept_scheduler_ablation_is_inert_without_prerequisites`, plus stress tests and
the `emit_ablation_fixture` emitter). The engine-emitted results land in
`evals/fixtures/ablation.json`; `evals/ablation.py` renders `evals/ABLATION.md` +
`evals/ablation.svg` and re-asserts the cutoff.

**Result (12 seeds, budget 56 cards; captured in `evals/ABLATION.md`):**

| Build | Prerequisite violations (mean [min, max]) |
| --- | :---: |
| **Concept scheduler ON** | **8.5 [8, 9]** |
| Feature OFF (ablation) | 56.0 [56, 56] |
| Plain Anki | 56.0 [56, 56] |

Turning the feature on cut prerequisite violations by **~85%** vs both OFF and plain
Anki, on **every** seed (pre-committed cutoff: per-seed `ON < OFF` and `ON < PLAIN`,
and `mean(ON) <= 0.5 * mean(OFF)` — both met). Secondary gains under the stated
scaffolding assumption: held-out accuracy **+0.19**, projected readiness **+9.3**
(ON 481–499 vs plain 472–491), memory **+0.39**. **Null:** on a deck with no
prerequisite edges all three arms record **0** violations — the feature is inert
without prerequisite structure.

**Engine cross-check:** the ON-arm violation count reported by the eval equals the
engine's OWN `prerequisite_violations` counter on every seed
(`on_observer_equals_engine_counter` in the fixture; re-checked by `test_parity.py`).

**Re-run:**

```sh
# one command (emit from the engine, then render + re-check):
just ablation
# or the underlying steps:
cd anki && CARGO_TARGET_DIR=out/rust cargo test -p anki --features rustls \
    scheduler::concept_ablation -- --nocapture
python3 evals/ablation.py
```

---

## Python↔Rust parity

**What it demonstrates:** the two held-out model evals recompute, in pure Python,
formulas that live in the Rust engine. This locks them to the engine: a Rust test
emits reference values straight from the shipped functions, and a Python test
asserts the eval modules reproduce every value to **1e-9**. If the engine and the
eval ever drift, the parity test fails.

**Where it lives:**

- Reference emitter (Rust): `anki/rslib/src/scheduler/concept.rs`, test
  `emit_engine_parity_fixture` → writes `evals/fixtures/engine_parity.json`.
- Parity check (Python): `evals/test_parity.py` — imports the pure functions from
  `performance_eval.py` and `calibration.py` and compares against the fixture.

**Formulas checked (tolerance = 1e-9):**

| Category | Engine source | Eval source | Grid size |
| --- | --- | --- | ---: |
| `difficulty_to_irt_b(d)`, d = 1..5 | `concept.rs::difficulty_to_irt_b` | `performance_eval.difficulty_to_irt_b` | 5 |
| IRT 3PL `probability_correct(θ)` | `concept.rs::IrtItemMetadata::probability_correct` | `performance_eval.irt_probability_correct` | 150 |
| FSRS-5 constants (decay + factor) | `fsrs::FSRS5_DEFAULT_DECAY` + `0.9^(1/−decay)−1` | `calibration.FSRS_DECAY` / `FSRS_FACTOR` | 3 |
| `base_recall_for_button(b)`, b = 1..4 | `concept_demo.rs::base_recall_for_button` | `calibration.BASE_RECALL` | 4 |
| Memory rating-decay fallback | `concept_demo.rs::card_memory` fallback branch | `calibration.predict_recall` | 48 |
| FSRS-5 forgetting curve (analytic) | `R(t)=(1+FACTOR·t/S)^(−DECAY)` from engine constants | `calibration.predict_recall` (FSRS path) | 20 |

The IRT grid covers difficulty ∈ {1..5}, θ ∈ {−2,−1,0,1,2}, discrimination ∈
{0.8, 1.0, 1.4}, guessing ∈ {0.2, 0.25}.

**Result (captured from `python3 evals/test_parity.py`):**

```
difficulty_to_irt_b      PASS  pass=   5  fail=   0  max|err|=0.00e+00
irt_probability_correct  PASS  pass= 150  fail=   0  max|err|=0.00e+00
fsrs_constants           PASS  pass=   3  fail=   0  max|err|=0.00e+00
base_recall_for_button   PASS  pass=   4  fail=   0  max|err|=0.00e+00
memory_fallback          PASS  pass=  48  fail=   0  max|err|=0.00e+00
fsrs_curve(analytic)     PASS  pass=  20  fail=   0  max|err|=0.00e+00

TOTAL: pass=230  fail=0  -> ALL PARITY CHECKS PASS
```

**230 / 230** reference values reproduced; the max absolute error was **0.0** —
the shared f64 arithmetic is bit-identical between Rust and CPython for this grid,
comfortably inside the 1e-9 tolerance.

**Honest caveat on the FSRS retrievability path.** The engine's FSRS path calls
the `fsrs` crate (`current_retrievability_seconds`, computed in **f32**). The
Python eval does **not** call the crate; it evaluates the **analytic FSRS-5
default power forgetting curve** `R(t) = (1 + FACTOR · t / S)^(−DECAY)` in **f64**,
using the engine's exact constants (`DECAY = FSRS5_DEFAULT_DECAY = 0.5`,
`FACTOR = 0.9^(1/−0.5) − 1 ≈ 0.234568`). These are the **same curve only at the
default decay**, and this is **not** a bit-identical crate call. So parity is
asserted against the engine-emitted **`recall_analytic`** reference (same analytic
formula, same constants) to 1e-9. The fixture also carries the actual crate value
(`recall_fsrs_crate`), and `test_parity.py` **reports but does not assert** the
analytic-vs-crate gap, which over this grid is at most **9.73e-08** — i.e. the two
agree to f32 precision, as expected, and nothing bit-identical is claimed.

**Re-run:**

```sh
# 1) emit engine references
cd anki && CARGO_TARGET_DIR=out/rust cargo test -p anki --features rustls \
    emit_engine_parity_fixture -- --nocapture
# 2) check the Python evals reproduce them
cd .. && python3 evals/test_parity.py
```
