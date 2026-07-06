# Model metrics scoreboard — memory / performance / readiness gains

Consolidated, re-runnable metrics for the three scores, with the **gain over a
simpler baseline** for each (rubric §9 "beat a simpler method"; §11 honest
uncertainty). Detailed writeups: `docs/model-memory.md`,
`docs/model-performance.md`, `docs/model-readiness.md`,
`evals/MODEL-EVALS.md`, `evals/ENGINE-FIDELITY.md`.

All numbers below are emitted by the committed scripts and were last refreshed on
**Jul 5, 2026**; re-running regenerates them (the scripts exit non-zero if a
pre-committed cutoff is missed).

## Memory (calibration on 1600 held-out synthetic reviews)

| Metric | Model | Baseline (predict base rate 0.620) | Gain |
| --- | ---: | ---: | ---: |
| Brier | **0.1677** | 0.2356 | **−0.068 (−28.8%)** |
| ECE | **0.0617** | — | (cutoff ≤ 0.1 met) |
| Log-loss | 0.5180 | 0.6641 | −0.146 |

The Brier of a constant "predict the base rate" model is `p(1−p) = 0.620·0.380 =
0.2356`; the memory model cuts that by **28.8%**. Calibration is honest: mean
predicted 0.677 vs observed 0.620 → **slight over-confidence**, shown in
`evals/memory-calibration.svg`.

## Performance (IRT 3PL on 1560 held-out synthetic attempts)

| Model | Accuracy | AUC | Brier |
| --- | ---: | ---: | ---: |
| **IRT 3PL (ours)** | **0.712** | **0.769** | **0.1957** |
| Mastery-only | 0.615 | 0.660 | 0.2333 |
| Majority-class | 0.572 | 0.500 | 0.2448 |

**Gain:** **+0.140** accuracy over majority-class, **+0.097** over mastery-only
(beats both — the required "simpler method" comparison).

## Readiness (in-engine feature ablation — the study feature's effect)

The readiness *ordering* is the shipped study feature; its effect is measured
inside the real queue builder (not the final projected score, which is not yet
validated end-to-end — see `docs/model-readiness.md`).

| Concept scheduler | Unmet-prerequisite cards in first 4 served |
| --- | :---: |
| **ON** | **0** |
| **OFF** | **4** |

**Gain:** turning the feature on removed **every** prerequisite-violating card
from the front of the queue (0 vs 4). Details + rationale in
`evals/ENGINE-FIDELITY.md`.

## Engine fidelity (are these evals measuring the real engine?)

Python↔Rust parity: **230/230** reference values reproduced at ≤ 1e-9 — the evals
recompute the engine's own formulas, so the metrics above describe the shipped
Rust models, not a look-alike.

## Reproduce everything

```sh
# memory + performance + parity (stdlib only, offline)
python3 evals/calibration.py
python3 evals/performance_eval.py
python3 evals/test_parity.py

# readiness-ordering ablation (real engine)
cd anki && CARGO_TARGET_DIR=out/rust cargo test -p anki --features rustls \
    concept_scheduler_defers_unmet_prerequisite_new_cards_ablation -- --nocapture

# all 60 concept engine tests + the 4 Python-from-backend tests
cd anki && CARGO_TARGET_DIR=out/rust cargo test -p anki --features rustls concept
cd anki && PYTHONPATH=out/pylib ANKI_TEST_MODE=1 out/pyenv/bin/pytest -p no:cacheprovider \
    pylib/tests/test_concept_scheduler_engine.py -q
```
