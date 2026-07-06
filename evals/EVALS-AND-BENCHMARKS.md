# Evals & benchmarks — consolidated index (all metrics)

One place for every quantitative result in the project: model evals, engine
fidelity, AI checks, the study-feature ablation, the speed benchmark, coverage,
and test suites. Exam: **MCAT** (472–528).

**Snapshot: 2026-07-05.** Numbers are quoted from the canonical, script-generated
files linked per section — re-running a script updates its own file, and those
files are authoritative if a narrative doc rounds differently.

## Scoreboard

| Area | Headline | Cutoff | Pass? | Source | Re-run |
| --- | --- | --- | :--: | --- | --- |
| Memory calibration | Brier **0.1677**, ECE **0.0617** | Brier ≤0.25, ECE ≤0.1 | PASS | `MODEL-EVALS.md` | `python3 evals/calibration.py` |
| Performance (IRT 3PL) | acc **0.712**, AUC **0.769**, Brier **0.196** | acc ≥0.7, AUC ≥0.7, Brier ≤0.22 | PASS | `MODEL-EVALS.md` | `python3 evals/performance_eval.py` |
| Engine↔eval parity | **230/230** @ 1e-9 | all pass | PASS | `ENGINE-FIDELITY.md` | parity (below) |
| Prereq queue ablation | ON **0** vs OFF **4** unmet-prereq in first 4 | ON < OFF | PASS | `ENGINE-FIDELITY.md` | cargo (below) |
| Study-feature ablation (3-build) | violations ON **8.5** vs OFF **56** vs plain **56** (~85% fewer) | per-seed ON<OFF & ON<plain | PASS | `ABLATION.md` | `just ablation` |
| AI — tagging | **88%** vs **35%** baseline (+52%) | ≥80% | PASS | `RESULTS.md` | `python evals/ai_eval.py` |
| AI — rewording | **95%** | ≥85% | PASS | `RESULTS.md` | same |
| AI — injection | tagger **98%** (1 breach), tutor **100%** | 100% | PARTIAL | `RESULTS.md` | same |
| AI — leakage | **0** leaks, **0** contamination | 0 | PASS | `RESULTS.md` | same |
| Speed benchmark (50k deck, desktop engine) | next-card p95 **0.06 ms**, grade p95 **0.48 ms**; **dashboard_refresh p95 680 ms > 500 ms** | §10: ack <50, next <100, load <1000, refresh <500 ms | PARTIAL | `BENCHMARK.md` | `just bench` |
| Crash + offline (desktop) | crash **20/20 clean, 0 corrupted**; offline: AI degrades cleanly, scores still render | rubric §7g | PASS | `CRASH-OFFLINE.md` | `just crash-test` / `just offline-test` |
| Tests | **60** Rust + **4** Python-RPC + **12** Android honesty | — | PASS | `docs/rust-change-note.md` | below |

Legend: PASS = meets cutoff · PARTIAL = one sub-target missed (disclosed).

---

## Charts

Hand-rolled stdlib SVGs; regenerate with `python3 evals/plots.py` (plus
`calibration.py` / `just ablation` / `just bench`). Full catalog: `evals/PLOTS.md`.

**Memory calibration** &nbsp; **Performance ROC (AUC 0.769)** &nbsp; **Performance calibration**

![Memory reliability diagram](memory-calibration.svg)
![Performance ROC curve](performance-roc.svg)
![Performance reliability diagram](performance-reliability.svg)

**Performance vs baselines** &nbsp; **Ablation — main (violations)** &nbsp; **Ablation — secondary**

![Performance vs baselines](performance-baselines.svg)
![Ablation prerequisite violations](ablation.svg)
![Ablation secondary metrics](ablation-secondary.svg)

**AI eval vs cutoffs** &nbsp; **Readiness as a range** &nbsp; **Engine latency (50k deck)**

![AI eval pass rates vs cutoffs](ai-eval.svg)
![Readiness projection range](readiness-projection.svg)
![Engine latency p95 vs target](bench-latency.svg)

---

## 1. Memory calibration — `evals/MODEL-EVALS.md`

Held-out: **1600** synthetic reviews (seed `20260705`; **1234** FSRS-path + **366** rating-decay fallback). Model: the app's `card_memory` FSRS-5 curve + fallback.

| Metric | Value | Cutoff | Result |
| --- | ---: | --- | :--: |
| Brier | **0.1677** | ≤ 0.25 | PASS |
| ECE | **0.0617** | ≤ 0.1 | PASS |
| Log-loss | 0.5180 | (reported) | — |
| MCE | 0.0898 | (reported) | — |
| Calibration-in-the-large (mean pred vs obs) | 0.677 vs 0.620 | (reported) | — |

**Gain vs base-rate predictor (p=0.620):** Brier 0.2356 → **0.1677 (−28.8%)**; log-loss 0.6641 → 0.5180 (−0.146). Honest read: slightly over-confident (`evals/memory-calibration.svg`).

Reliability by decile (predicted → observed; full table in `MODEL-EVALS.md`): worst-calibrated bins are 0.7–0.8 (pred 0.754 / obs 0.664 / gap +0.090) and 0.9–1.0 (n=443, pred 0.933 / obs 0.851 / gap +0.082). FSRS analytic-vs-crate gap over the parity grid: max **9.73e-08** (reported, not asserted).

## 2. Performance / IRT — `evals/MODEL-EVALS.md`

Held-out: **1560** synthetic attempts (130 students × 12 items; seed `20260705`). Base rate correct **0.572**. Model: IRT 3PL `P = c + (1−c)·logistic(a(θ−b))`.

| Model | Accuracy @0.5 | Wrong | AUC | Brier |
| --- | ---: | ---: | ---: | ---: |
| **IRT 3PL (ours)** | **0.712** (1111/1560) | 449 | **0.769** | **0.1957** |
| Mastery-only | 0.615 | 601 | 0.660 | 0.2333 |
| Majority-class | 0.572 | 667 | 0.500 | 0.2448 |

Cutoffs: acc ≥0.70 **PASS**, AUC ≥0.70 **PASS**, Brier ≤0.22 **PASS**, beats both baselines **PASS**. **Lift:** +0.140 acc over majority, +0.097 over mastery-only. Disclosed: the ground-truth process uses timing/carelessness the pure-IRT predictor ignores → headroom.

## 3. Readiness + give-up — `docs/model-readiness.md`, `docs/give-up-rule.md`

Method (not validated end-to-end; honest per rubric §9 Step 3/4). Constants: guessing floor **120.0**, θ→scale `SCALE_B` **125** / `SCALE_A` **2.5**, band **±1.96·SE**, total range clamped **472–528**. Give-up (enforced): global readiness sort ≥ **500** graded answers; per-section ≥ **20** items **and ≥ 60%** coverage; memory abstains when `has_memory=false`. Android abstains fully (12/12 `McatHonestyTest`); desktop hero + deck-options abstain, desktop section cards partial.

## 4. Engine fidelity / parity — `evals/ENGINE-FIDELITY.md`

**230/230** reference values reproduced at tolerance **1e-9**, max |err| **0.0**:

| Category | Checks | Result |
| --- | ---: | :--: |
| `difficulty_to_irt_b` (d=1..5) | 5/5 | PASS |
| IRT 3PL `probability_correct(θ)` grid | 150/150 | PASS |
| FSRS-5 constants (decay + factor) | 3/3 | PASS |
| `base_recall_for_button` (b=1..4) | 4/4 | PASS |
| Memory rating-decay fallback | 48/48 | PASS |
| FSRS-5 analytic forgetting curve | 20/20 | PASS |

Plus the **in-engine prereq-violation ablation**: with the scheduler **ON**, **0** of the first 4 served new cards violate a prerequisite; **OFF**, **4** (readiness scores: met-prereq 0.85×0.80=0.68 vs unmet 0.20×0.80=0.16; threshold 0.70).

## 5. Study-feature ablation, 3 builds — `evals/ABLATION.md`

12 seeds (`20260705..20260716`); **192**-card deck / **64** KCs (reverse-topological, adversarial to plain order); equal **56** answered cards/arm. Main pre-stated number = `prerequisite_violations`.

| Build | Prerequisite violations (mean [min, max]) |
| --- | ---: |
| **Concept scheduler ON** | **8.5 [8.0, 9.0]** |
| Feature OFF (ablation) | 56.0 [56.0, 56.0] |
| Plain Anki | 56.0 [56.0, 56.0] |

**~85% fewer** than OFF/plain, on every seed. Secondary (scaffolding-conditioned):

| Metric | ON | OFF | PLAIN | ON − PLAIN |
| --- | ---: | ---: | ---: | ---: |
| Held-out exam accuracy | 0.412 [0.409, 0.444] | 0.225 | 0.225 | **+0.187** |
| Projected readiness (472–528) | 489.6 [486.9, 490.7] | 480.3 | 480.3 | **+9.3** |
| Memory (mean recall) | 0.762 [0.716, 0.789] | 0.371 | 0.371 | **+0.391** |
| Coverage (KCs touched) | 0.301 [0.297, 0.344] | 0.297 | 0.297 | — |

Readiness 95% band per arm: ON **481–499**, plain **472–491**. **Honest null:** on a deck with no prerequisite edges, all three arms record **0** violations (feature inert without prereq structure). Cutoffs (all PASS): per-seed ON<OFF & ON<PLAIN; mean(ON) ≤ 0.5·mean(OFF); ON observer == engine counter; null control zero.

## 6. AI checking & safety — `evals/RESULTS.md`

`gpt-4o-mini`, run **2026-07-06T00:31:09Z**, KC candidate set **108** ids (live from `editor.py`), **40** held-out items per set (authored for eval, not in shipped decks). Pre-committed cutoffs.

| Check | Result | Baseline | Cutoff | Pass? |
| --- | --- | --- | --- | :--: |
| KC-tagging top-1 accuracy | **88%** (35/40; 5 wrong) | lexical **35%** (26 wrong), +52% | ≥80% | PASS |
| Rewording faithfulness | **95%** (38/40; 2 wrong: EVAL-RW-09, EVAL-RW-15) | — | ≥85% | PASS |
| Injection — card tagger (40 attacks) | **98%** (39/40; 1 breach: EVAL-INJT-31) | — | 100% | MISS |
| Injection — Ask-AI tutor (40 attacks) | **100%** (40/40) | — | 100% | PASS |
| Gold-label leaks | **0** | — | 0 | PASS |
| Train/test contamination | **0** eval cards in shipped decks | — | 0 | PASS |

Every AI output has a traceable named source (`evals/AI-RATIONALE.md`; catalog `docs/ai-features.md`).

## 7. Speed / latency benchmark — `evals/BENCHMARK.md`

`just bench` on a **50,000-card** deck (120 graph nodes / 114 edges), macOS arm64 (10 CPU), Python 3.13.13, 50 timed iters (warmup 5), 2026-07-05 15:59 CDT. Engine-level backend timings (not UI paint). **Desktop (macOS) host only** — §10 also asks for phone-side targets; the phone runs the same engine but is not timed on-device here (see §11).

| Action | p50 | p95 | worst | Target | Result |
| --- | ---: | ---: | ---: | --- | :--: |
| collection_open | 2.35 ms | 2.81 ms | 3.39 ms | p95 < 5000 ms | PASS |
| next_card | 0.06 ms | 0.06 ms | 0.06 ms | p95 < 100 ms | PASS |
| grade | 0.25 ms | 0.48 ms | 5.90 ms | p95 < 50 ms | PASS |
| dashboard_load | 656.69 ms | 686.83 ms | 882.49 ms | p95 < 1000 ms | PASS |
| **dashboard_refresh** | 640.50 ms | **680.09 ms** | 856.91 ms | p95 < 500 ms | **FAIL** |
| memory_rss | 166.4 MB | 166.4 MB | 166.4 MB | < 1500 MB | PASS |
| sync | — | — | — | N/A | N/A |

**Overall: SOME TARGETS NOT MET** — `dashboard_refresh` misses (disclosed cost of the serial KC-map recompute on 50k cards; the benchmark is the regression guard for the fix). Reported as a FAIL, not hidden.

## 8. Coverage map — `docs/kc-map-unified.md`

Frozen unified prerequisite graph: **172 KCs, 321 edges, 30 layers**, globally acyclic; blueprint-weighted section coverage in the engine. Benchmark deck graph: **120 nodes / 114 edges** on 50k cards. Legacy demo graph: **10 nodes / 9 edges**. Dashboard "% of AAMC outline covered" not yet surfaced (rubric §8 open).

## 9. Test suites (re-runnable)

| Suite | Count | Command |
| --- | ---: | --- |
| Rust concept engine | **60** | `cd anki && CARGO_TARGET_DIR=out/rust cargo test -p anki --features rustls concept` |
| Python-from-backend RPC | **4** | `cd anki && PYTHONPATH=out/pylib ANKI_TEST_MODE=1 out/pyenv/bin/pytest -p no:cacheprovider pylib/tests/test_concept_scheduler_engine.py -q` |
| Android honesty (JDK 21) | **12** | `cd Anki-Android && ./gradlew :AnkiDroid:testPlayDebugUnitTest --tests "com.ichi2.anki.conceptscheduler.McatHonestyTest"` |

## 10. All re-run commands

```sh
# Model evals (stdlib only, offline)
python3 evals/calibration.py
python3 evals/performance_eval.py

# Engine ↔ eval parity (two steps)
cd anki && CARGO_TARGET_DIR=out/rust cargo test -p anki --features rustls emit_engine_parity_fixture -- --nocapture
cd .. && python3 evals/test_parity.py

# In-engine prereq-violation ablation
cd anki && CARGO_TARGET_DIR=out/rust cargo test -p anki --features rustls \
    concept_scheduler_defers_unmet_prerequisite_new_cards_ablation -- --nocapture

# 3-build study-feature ablation
just ablation

# Speed / latency benchmark (50k deck)
just bench

# AI evals (needs OPENAI_API_KEY in env or repo .env)
python evals/ai_eval.py

# Robustness -- crash + offline (results in CRASH-OFFLINE.md)
just crash-test
just offline-test
```

## 11. Honest gaps (not yet measured)

- **Phone-side §10 speed/reliability**: `just bench` runs the shared Rust engine on the desktop (macOS) host. Button-ack, cold start, and memory on a mid-range phone are **not measured on-device** — the same engine ships to Android via `Anki-Android-Backend`, but an instrumented on-device benchmark is not wired up.
- **Crash + offline results recorded (desktop)**: crash **20/20 clean, 0 corrupted collections**; offline degrades cleanly with scores still computed locally (`evals/CRASH-OFFLINE.md`). Remaining: running the **same tests on an Android device/emulator** (the shared Rust write path carries the guarantee, but it isn't exercised on-device yet).
- **Sync conflict test** (10+10 offline, same-card winner rule): not yet documented.
- **Paraphrase test** (30 cards × 2 reworded) and **AI card-check gold set** (50+50): not yet run.
- **Dashboard "% of AAMC outline covered"**: coverage math exists in-engine but is not surfaced yet.
- **Readiness end-to-end**: calibrated *inputs* (memory, performance) are validated; the final projected score is not validated against real students (disclosed).
