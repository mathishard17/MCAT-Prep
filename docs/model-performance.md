# Performance model — "can the student answer a new exam-style question?"

**Rubric §4 / §9 Step 2 / §12 model description.** The bridge from memory to
new questions. Exam: **MCAT** (section scale 118–132).

## What it estimates

The probability the student answers a **new, exam-style question** on a topic
correctly — including questions they have never seen. This is deliberately
**separate from memory**: remembering the flashcard "mitochondria = powerhouse"
is not the same as answering an MCAT passage on cellular respiration.

## How it is computed (in the Rust engine)

An **IRT 3-parameter logistic (3PL)** item-response model
(`anki/rslib/src/scheduler/concept.rs`):

```
P(correct | θ) = c + (1 − c) · 1 / (1 + e^(−a·(θ − b)))
```

- **b (difficulty)** from the card's `Difficulty::` tag via `difficulty_to_irt_b`:
  {1 → −2, 2 → −1, 3 → 0, 4 → +1, 5 → +2}. `a` (discrimination) and `c`
  (guessing) come from `IRT::…` tags with seeded defaults.
- **Ability θ** per section is updated on each graded answer with a Newton step
  from the Fisher information (`IrtSectionState::record_response`); its standard
  error is `1/√information`.
- **θ → section scale (118–132):** `score = SCALE_B + SCALE_A · θ` with
  `SCALE_B = 125` (median prepared test-taker), `SCALE_A = 2.5`, clamped to
  [118, 132]. The reported band is `center ± 1.96 · SE` (SE also scaled by 2.5),
  clamped to the scale — a **range, never a bare point**.

## Give-up rule (this score)

A section reports a performance range only once it has **enough evidence**:
`answered ≥ irt_min_section_items (20)` **and** `coverage ≥ irt_min_section_coverage (0.60)`.
Below that, `enough_evidence = false` and the UI shows an abstain/"building
evidence" state rather than a number. See `docs/give-up-rule.md`.

## Honest uncertainty & evidence (rubric honesty rule)

Held-out accuracy on 1560 synthetic exam-style attempts
(`evals/performance_eval.py`), with a **pre-committed cutoff** and a **baseline
comparison** (the model must beat a simpler method):

| Model | Accuracy @0.5 | AUC | Brier |
| --- | ---: | ---: | ---: |
| **IRT 3PL (ours)** | **0.712** | **0.769** | **0.1957** |
| Majority-class | 0.572 | 0.500 | 0.2448 |
| Mastery-only | 0.615 | 0.660 | 0.2333 |

Cutoffs: acc ≥ 0.70 PASS · AUC ≥ 0.70 PASS · Brier ≤ 0.22 PASS · beats both
baselines PASS. **Lift:** +0.140 accuracy over majority, +0.097 over
mastery-only.

- **Locked to the engine:** `evals/test_parity.py` reproduces
  `difficulty_to_irt_b` and the 3PL `P(correct|θ)` across a 150-point grid to
  1e-9.
- **What's missing (disclosed):** the ground-truth process includes a
  timing/rushing penalty and carelessness slips; the shipped predictor is **pure
  ability × item** and does **not yet consume timing or coverage**, so there is
  measurable headroom. Held-out attempts are synthetic; real MCQ-outcome
  validation is future work.

## Verify

```sh
python3 evals/performance_eval.py  # recomputes acc/AUC/Brier + baselines, exits non-zero on miss
python3 evals/test_parity.py       # engine↔eval parity for the 3PL formula
cd anki && CARGO_TARGET_DIR=out/rust cargo test -p anki --features rustls \
    irt_section_state_updates_theta_and_error aamc_scale_maps_theta_to_median_and_clamps
```

Source: `concept.rs::IrtItemMetadata::probability_correct`, `::record_response`,
`::difficulty_to_irt_b`, θ→scale (`SCALE_A`/`SCALE_B`).
