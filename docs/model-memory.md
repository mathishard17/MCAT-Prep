# Memory model — "can the student recall this fact right now?"

**Rubric §4 / §9 Step 1 / §12 model description.** One of the three separately
reported scores. Exam: **MCAT**.

## What it estimates

The probability the student **recalls a fact they have already studied**, at a
given point in time. This is *memory only* — not whether they can answer a new
exam question (that is the [performance model](model-performance.md)).

## How it is computed (in the Rust engine)

Per card, in `card_memory()` (`anki/rslib/src/scheduler/concept_demo.rs`):

- **FSRS path (studied cards with a memory state):** the FSRS-5 retrievability
  from Anki's own `fsrs` crate — the power forgetting curve

  ```
  R(t) = (1 + FACTOR · t / S) ^ (−DECAY),   DECAY = 0.5,  FACTOR = 0.9^(1/−DECAY) − 1
  ```

  evaluated at a **1-day horizon floor** (`MEMORY_HORIZON_SECS = 86_400`) so a
  card reviewed seconds ago doesn't read as a fake ~100% recall.
- **Fallback (studied, no FSRS state yet):**
  `base_recall_for_button(last_rating) · exp(−elapsed / max(interval, 1))`,
  with bases Again = 0.20, Hard = 0.55, Good = 0.80, Easy = 0.90.
- **Unseen cards (`reps == 0`) return nothing** — they never inflate the score.

Aggregation:

- **Per KC:** mean recall over that KC's studied cards.
- **Per section:** blueprint-weighted mean of its KC memories
  (`section_memory_from_kc_memory`, `concept.rs`).
- **Overall:** mean of per-KC memories (`overall_memory`, `has_memory`).

Memory is a probability in **[0, 1]**, shown as a percentage. It is **not**
mapped onto the 118–132 scale.

## Give-up rule (this score)

Show nothing when there is no memory evidence: if no studied card is tagged to a
KC, `has_memory = false` and the UI abstains ("—" on desktop, the abstain tile
on Android) instead of printing a number. See `docs/give-up-rule.md`.

## Honest uncertainty & evidence (rubric honesty rule)

- **Held-out calibration** (`evals/calibration.py`, 1600 synthetic reviews):

  | Metric | Value | Cutoff | Result |
  | --- | ---: | --- | :---: |
  | Brier | **0.1677** | ≤ 0.25 | PASS |
  | ECE | **0.0617** | ≤ 0.1 | PASS |
  | Log-loss | 0.5180 | (reported) | — |
  | MCE | 0.0898 | (reported) | — |

  Calibration-in-the-large: mean predicted **0.677** vs observed **0.620** — an
  **honestly disclosed slight over-confidence** (see the reliability diagram
  `evals/memory-calibration.svg`), not hidden.
- **Locked to the engine:** `evals/test_parity.py` proves the Python curve
  reproduces the engine's FSRS-5 constants + fallback to 1e-9 (230/230).
- **What's missing:** the held-out reviews are **synthetic/seeded** — no real
  longitudinal revlog was available in a week. Real-revlog validation is future
  work (rubric §9 Step 4). Stated, not hidden.

## Verify

```sh
python3 evals/calibration.py       # recomputes Brier/ECE/log-loss + SVG, exits non-zero on miss
python3 evals/test_parity.py       # engine↔eval parity (needs evals/fixtures/engine_parity.json)
cd anki && CARGO_TARGET_DIR=out/rust cargo test -p anki --features rustls \
    demo_status_reports_memory_after_reviews -- --nocapture
```

Source: `concept_demo.rs::card_memory`; `concept.rs::section_memory_from_kc_memory`.
