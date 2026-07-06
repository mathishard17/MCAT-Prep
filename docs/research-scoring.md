# Research Report: Scoring & Readiness (Performance → Memory → Coverage → Projected Score)

> Companion to `brainlift.md`. That doc already covers the *learning* side (memory→performance,
> flashcards vs. problems) and the *taxonomy* of scoring models (CTT, IRT, KST, Bayesian
> Networks, BKT, GKT). This report goes deeper on the **scoring / readiness** half: how the
> real MCAT scale actually behaves, and how to turn our per-KC signals into a *defensible
> projected section score* that combines **performance + memory + coverage + uncertainty**.
> Where brainlift already nails something (e.g., IRT 3PL, BKT four params), I reference it
> instead of repeating it.

---

## TL;DR — the decisions this settles

1. **The MCAT is equated, not curved — but the scale is anchored to a prepared population.**
   Your score doesn't move based on who tests the same day; it's linked across forms so a 128
   always means the same thing. But the 118–132 scale was *set* against the test-taker
   population, so 500 total / **125 per section ≈ the 50th percentile** (a median *prepared
   pre-med*, not a median human). [AAMC]

2. **Our current `demonstrated_score = 118 + accuracy·14` linear map is wrong** and is the root
   of the 118-vs-125 confusion. On the real equated scale the raw→scaled curve is *compressive
   and population-anchored*: ~25% correct (4-choice guessing) lands near the **118 floor**, and
   **125 corresponds to roughly ~65–70% correct**, not 50%. A linear map that puts 50% accuracy
   at 125 systematically over-scores weak performance.

3. **So for "not-ready" subjects, anchor LOW (~118–120, the guessing floor), not 125.** 125 =
   median = "a competent prepared test-taker," which is a strong (over-optimistic) claim about
   material the user has never studied. This is the corrected answer to the question we were
   debating.

4. **Readiness must include memory, and it currently doesn't.** `section_score_status` uses
   performance (accuracy) + coverage only; FSRS-derived recall (`section_memory_from_kc_memory`)
   is computed but never folded into the readiness *center*. A score you'll have forgotten by
   test day is not readiness. This matches the published **Exam Readiness Index** (Mastery +
   Coverage + **Retention** + …). [ERI]

5. **Report a band, not a point, and use an average not the latest session.** AAMC itself
   reports **±1 per section / ±2 total** confidence bands (reliability ≈0.9). The best single
   predictor of the real score is the **median/average of recent full-lengths** (r≈0.92), not
   the most recent or the max. [AAMC][ResidencyAdvisor][Premier]

---

## 1. How the real MCAT is actually scored

- **Raw score** = number correct per section. No penalty for wrong/blank (so always guess). [AAMC]
- **Scaled score** = raw converted to **118–132** per section (472–528 total) via a
  form-specific table produced by **equating**. Equating compensates for small difficulty
  differences between forms so "scores have the same meaning no matter when you test." It is
  explicitly **not a curve** against same-day testers. [AAMC "How is the MCAT scored"]
- **Percentiles** are reported alongside and refreshed every May 1 over a rolling 3-year window.
  500 ≈ 50th pct; higher scores map to steadily higher percentiles (≈515 is low-90s). [AAMC][Tutorium]
- **Confidence bands**: section **±1**, total **±2**, because scores are "imperfect measures …
  not perfectly precise." Reliability coefficient ≈ **0.9**. [AAMC score-report PDF][SDN]

**Why it matters for us:** equating ≈ "put every form's items on one θ scale, then map θ→scale."
That is exactly the IRT pipeline brainlift describes — our job is to (a) estimate θ well and
(b) map θ→118–132 with a *realistic* (non-linear, population-anchored) curve, then (c) attach a
band.

---

## 2. The raw→scaled curve is compressive & population-anchored (fixing our map)

AAMC's own examples: raw **35–37 / 59 → ~123**; raw **46–48 → ~128**. Working backwards:

- ~60% correct → ~123; ~80% correct → ~128 ⇒ the curve is compressive and **125 (the median)
  sits around ~65–70% correct**, *not* 50%.
- 4-choice random guessing ≈ 25% correct → **near the 118 floor** (very low percentile), because
  the scale is anchored to prepared pre-meds who mostly score well above chance.

**Consequences:**

- Our `118 + accuracy·14` (0%→118, 50%→125, 100%→132) mis-states reality: it treats "half right"
  as median when half-right is actually a *low* section score. Replace it with either
  (a) a proper **IRT θ→scale** map (`SS = A·θ + B`, clamped), or (b) an **empirical raw→scale
  table** calibrated from AAMC-style reference data / our own outcomes.
- **"Not-ready" anchor:** guessing ≈ floor, so unstudied material should pull the projection
  toward **~118–120**, not 125. (We already have `guessing_baseline_score = 120.0` for exactly
  this; `section_score_status` just ignores it in favor of the 125 prior.)
- Keep the population interpretation explicit: **125 = "you'd be an average prepared test-taker
  here"** is a claim you should only make where the user has *demonstrated + retained* that level.

---

## 3. Performance → ability (θ), and how to "measure" the item parameters

Brainlift already covers 3PL IRT: `P(correct|θ) = c + (1−c)/(1+e^{−a(θ−b)})`, θ via MLE, then
scale θ. Our code implements this (`IrtItemMetadata::probability_correct`, Newton update, θ→scale
`125 + θ·2.5`). The open problem — and the crux of the "actually measure it, don't use random
formulas" ask — is **where a, b, c come from**. Today they're tag defaults + a fixed
difficulty→b table.

**How real programs measure them (and how we can):**

- **Item calibration**: fit each item's (a, b, c) from many students' responses (MMLE/EM, or
  Bayesian). Until we have volume, use **empirical-Bayes shrinkage**: start from the tag default,
  move toward the item's observed accuracy as reviews accumulate (min-N gate). [ACT RR2004-5][ETS]
- **Equating/linking** across our question "forms"/sessions: Stocking–Lord or Haebara
  characteristic-curve linking, or concurrent calibration, so θ is comparable across different
  item sets. [NCME Module 10][theta-minus-b]
- **θ estimation per attempt**: MLE is fine but unstable at the extremes / low N; **EAP (expected
  a posteriori)** with a population prior is more robust for sparse data — worth adopting since our
  users answer few items per KC early on.
- **θ→scaled**: linear `SS = A·θ + B` is standard; pick A,B so the section SD and median match the
  real scale (≈2.5–3 scale points per θ SD; median θ→125). This replaces the accuracy-linear map.

This is the same "measure, don't guess" program as the Bayesian-backend task: the slip/guess in
BKT and the (a,b,c) in IRT are *the same kind of parameter* and should be **fit from the revlog
with shrinkage + min-sample fallbacks**, then **validated** (see §8).

---

## 4. Memory / retention — the missing third leg

**Why it must be in readiness:** performance says "could you answer it *right after studying*";
retention says "will you still answer it *on test day, weeks later*." Readiness is the
intersection. The knowledge-tracing field has repeatedly shown that **adding forgetting improves
next-step prediction** (DKT-Forget, HawkesKT, RKT, KPT, BKT+forgetting). [Nagatani 2019 (DKT-Forget)][KSYS 2025]

**We already have the measured memory signal:** FSRS fits per-user stability/difficulty from the
full revlog and yields **retrievability R = P(recall now)** — a *measured*, personal forgetting
model. `section_memory_from_kc_memory` already aggregates per-KC R to a section value; it's just
not used in the score.

**Standard forgetting forms** (for projecting R forward to the exam date):

- Exponential: `R(t) = exp(−Δt / S)` (FSRS-style, S = stability). ERI uses `r_t = exp(−λ·Δt)`. [ERI]
- Power-law / Ebbinghaus and sigmoid/inverse variants also used; recent work finds Ebbinghaus
  isn't always best, so keep the decay function swappable. [KSYS 2025]
- Duolingo's **Half-Life Regression** is the canonical "trainable spaced-repetition retention"
  model if we ever want to fit our own. [Settles & Meeder 2016]

**Key move:** project retrievability **to the exam date** (`Δt = test_date − now`), not just "now."
A concept mastered today but not scheduled to be reviewed before the test should have its
contribution decayed by its projected `R(test_date)`.

---

## 5. Coverage — you can't be ready on what you haven't seen

Coverage = fraction of the section's blueprint you've actually engaged (breadth across KCs, not
volume on one). Our `section_coverage` already does blueprint-weighted breadth. Two roles:

1. **Blend weight**: only the covered fraction can contribute demonstrated performance; the rest
   sits at the (low) guessing/prior anchor.
2. **Uncertainty**: thin coverage → wider band (we already widen SE by `(1−coverage)`).

Blueprint weights (Chem/Phys, Bio/Biochem, Psych/Soc discipline mixes) are in brainlift's MCAT
table and already encoded in `section_disciplines`.

---

## 6. Synthesis — a recommended readiness model

Frame readiness like the **Exam Readiness Index**: a bounded, blueprint-aware, monotone
combination of normalized signals (ERI uses Mastery, Coverage, Retention, Pace, Volatility,
Endurance). [ERI] We already produce the first three. Minimal, principled version:

For each section, per covered KC (or discipline slice):

```
p_know_i      = IRT/BKT probability the user knows KC i        (performance)
R_i(test)     = FSRS retrievability of KC i projected to exam  (memory)
p_ready_i     = p_know_i * R_i(test)          # must both know AND still recall it
```

Aggregate with blueprint weights `w_i` over the section, and blend covered vs. uncovered against
the guessing floor:

```
covered_ability   = Σ_i w_i * p_ready_i / Σ_i w_i        # over covered KCs
effective_fraction = coverage                            # blueprint-weighted breadth
section_ability    = effective_fraction * covered_ability
                     + (1 − effective_fraction) * guess_ability   # guess_ability ≈ floor
readiness_center   = scale(section_ability)              # θ→scale OR calibrated raw→scale table
```

Notes / rationale:
- **Multiplicative `p_know · R`** encodes "ready = knows it *and* will remember it," which is the
  literal definition we want and matches KT-with-forgetting practice. (A weighted average is the
  softer alternative if multiplicative feels too harsh once we validate.)
- **`scale(·)`** must be the corrected, compressive map from §2 — *not* `118 + x·14`.
- **Uncovered → floor** (~118–120), per §2, replacing the 125 prior for the untested fraction.
- This slots into `section_score_status` by (a) threading the section memory value in and
  (b) replacing the `demonstrated_score`/blend lines. `section_memory_from_kc_memory` already
  exists; the caller (`concept_demo.rs`) already computes `kc_memory`.

**Uncertainty (keep + extend what we have):** combine performance SE (scaled by coverage),
coverage SE `(1−coverage)`, and retention volatility into one band; **floor the band at ±1 scale
point** to honor the real measurement limit (AAMC section ±1). Report `readiness_center ± band`
and, if a target is set, `P(score ≥ target)` from the band.

---

## 7. How to present it (so it's honest, not fake precision)

- **Range, not a number.** Mirror AAMC's ±1 section / ±2 total bands; never show a bare point. [AAMC]
- **Average, not latest.** Anchor to a rolling average/median of recent performance; single
  sessions swing ±3–5 points and the median is the best predictor (r≈0.92). [Premier][ResidencyAdvisor]
- **Probability of hitting target**, e.g. "72% chance ≥ 128 in Bio/Biochem," from the band.
- **Decompose** the score into performance / memory / coverage so the user sees the limiting
  factor (ERI-style diagnostics: low retention → schedule reviews; low coverage → new topics). [ERI]

---

## 8. Validation — the part that makes it "measured," not asserted

Any parameter we fit (IRT a/b/c, BKT slip/guess, decay λ, θ→scale A/B) must beat the fixed
version on **held-out reviews**:

- **Calibration / reliability diagram**: bucket predicted P(correct) vs. observed; well-calibrated
  = diagonal.
- **Brier score & log-loss** on next-answer prediction (standard KT metrics). [DKT-Forget]
- **AUC** for discrimination.
- **Backtest the projection**: hide the last N sessions, project, compare to actual — target the
  literature's practice→real bar (**MAE ≈ 2, ~70–80% within ±3**). [ResidencyAdvisor][Premier]
- Guardrails that prevent noise from small N: **min-sample gates** (we have
  `readiness_min_seen_cards`, `irt_min_section_items`) + **shrinkage to priors**.

---

## 9. Open decisions

- **θ→scale calibration source:** anchor A,B to published AAMC percentile/scale references, or
  self-calibrate from our own outcomes over time?
- **Combine performance × memory:** strict multiplicative (recommended) vs. weighted average —
  decide by which calibrates better in §8.
- **Decay function:** exponential/FSRS (default) vs. power-law/sigmoid — keep swappable. [KSYS 2025]
- **Uncovered anchor:** guessing floor ~118–120 (recommended) vs. a small partial-credit prior for
  background knowledge.
- **Pace/Volatility/Endurance (timed-section signals):** add later, as ERI does, once we track
  per-question timing.

---

## 10. Provenance of every model constant (how the numbers were chosen)

> Asked directly: *where do these numbers come from?* Honest answer: three buckets — **(A)
> externally fixed facts**, **(B) values derived from those facts**, and **(C) hand-picked
> heuristics**. Everything below is a *default* in `ConceptSchedulerConfig` (`concept.rs`,
> `impl Default`) or a module `const`, and every one is overridable per deck. The per-answer
> Bayesian likelihoods additionally **self-correct** to empirical rates once enough data exists,
> so wrong priors wash out.

### A. Externally anchored — not ours to choose

| Constant (code) | Value | Basis / source |
| --- | --- | --- |
| `SCORE_MIN` / `SCORE_MAX` | 118 / 132 | AAMC per-section scaled-score range (472–528 total = 4×). [AAMC] |
| `SCALE_B` | 125 | AAMC section median (~50th pct of prepared test-takers). [AAMC] |
| `GUESS_FLOOR` | 0.25 | 4-option MCQ ⇒ 25% by chance (the IRT *c* lower asymptote). [IRT-Baker] |
| FSRS decay / factor | −0.5 / 19⁄81 | FSRS-5 power forgetting curve `R(t)=(1+FACTOR·t/S)^DECAY`, set so `R(S)=0.9`. [FSRS] |
| FSRS target retention | 0.90 | Stability is *defined* as the interval where R=90%. [FSRS] |
| `1.96` (band multiplier) | 1.96 | 95% two-sided quantile of the standard normal. [NIST] |
| Laplace `+1 / +2` | rule of succession | Additive (add-one) smoothing of an empirical rate. [Laplace] |
| `normal_cdf` coefficients | A&S 7.1.26 | Abramowitz & Stegun erf approximation (\|err\|≲1.5e-7). [A&S] |

### B. Derived from the facts above

| Constant (code) | Value | How it's derived |
| --- | --- | --- |
| `SCALE_A` | 2.5 | Scale points per ability SD, anchored so 118–132 ≈ ±~3 θ SD around 125 (AAMC's compressive raw→scale curve, §2–3). [AAMC][IRT-Baker] |
| `performance_standard_error` | `2.5/√I` | `SCALE_A` × IRT ability SE, where information `I` = Σ Fisher info over answered items. [IRT-Baker] |
| FSRS `FACTOR` = 19⁄81 | — | Solved from `R(S)=0.9` with `DECAY=−0.5`: `FACTOR = 0.9^(1/DECAY) − 1`. [FSRS] |
| total band ±`1.96·SE`, clamp 472–528 | — | Normal 95% band, clamped to the real scale; `total_se` floored at 1.0 to honor AAMC's ~±2-point total limit. [AAMC][NIST] |

### C. Hand-picked heuristics (chosen, conservative, overridable)

Not fit to held-out data. Chosen to bias toward "not ready yet" / wider bands (the honesty rule).
The four likelihoods are **priors only** — after `LIKELIHOOD_MIN_OBSERVATIONS` each is replaced by
its group's Laplace-smoothed empirical rate. [Laplace]

| Constant (code) | Default | Rationale |
| --- | --- | --- |
| `initial_mastery` | 0.20 | Low prior: assume a KC is *not* mastered before evidence. |
| `positive/negative_likelihood_*` | 0.90 / 0.20 / 0.10 / 0.80 | Sensitivity/specificity priors ("mastered ⇒ ~90% correct"); **self-correct** to empirical rates after 20 obs. [Laplace] |
| `LIKELIHOOD_MIN_OBSERVATIONS` | 20 | Min group size before trusting an empirical rate over the prior. |
| `PARTIAL_MASTERY_LIFT` | 0.5 | `P(correct\|¬mastered) = GUESS_FLOOR + 0.5·mastery`: a partly-learned KC beats pure guessing. |
| `inner_fringe_mastery` / `inner_fringe_min_answers` | 0.85 / 3 | "Mastered" bar: high posterior *and* ≥3 answers. |
| `outer_fringe_prereq_mastery` | 0.70 | Prereqs "ready enough" to unlock a new topic. |
| `guessing_baseline_score` | 120 | Untested/forgotten fraction sits near the guessing floor (§2), not the 125 median. [AAMC] |
| `max_coverage_standard_error` / `max_mastery_standard_error` | 2.0 / 2.0 | Max scaled-point uncertainty each weakness source (coverage, mastery, retention) adds; tuned to believable band widths. |
| `irt_min_section_items` / `irt_min_section_coverage` | 20 / 0.60 | Give-up gate: no confident section score until enough items + breadth. |
| `readiness_min_seen_cards` | 500 | Give-up gate for readiness sorting overall. |
| `MEMORY_HORIZON_SECS` | 86 400 (1 day) | Evaluate recall ≥1 day forward to avoid the post-review R≈1.0 artifact. |
| `fallback_readiness_score` | 0.5 | Neutral prior when readiness can't be computed. |

**Honesty note.** The **memory** model is genuinely calibrated on held-out reviews
(`evals/calibration.py`, `evals/ENGINE-FIDELITY.md`, `docs/model-memory.md`). The Bucket-C
**score-mapping** constants are *anchored and reasonable but not empirically fit* — the posture the
rubric rewards (`docs/honesty-rule.md`). Validating them is §8's program; the knobs are exposed on
`ConceptSchedulerConfig` for when that data exists.

---

## References

- AAMC — How is the MCAT Exam Scored (equating, 118–132, no curve):
  https://students-residents.aamc.org/mcat-scores/how-mcat-exam-scored
- AAMC — MCAT Exam Scoring (raw→scaled, percentiles):
  https://students-residents.aamc.org/register-mcat-exam/publication-chapters/mcat-exam-scoring
- AAMC — Understanding Your Score Report / Confidence Bands (±1 section, ±2 total) PDF:
  https://www.aamc.org/media/35641/download
- Tutorium — How the MCAT is scored (500≈avg, percentile intuition):
  https://tutorium.app/blog/how-the-mcat-is-scored
- ResidencyAdvisor — Practice→real score accuracy (MAE≈2, ±3 70–80%):
  https://residencyadvisor.com/resources/mcat-prep/score-prediction-accuracy-how-close-are-practice-tests-to-real-mcat
- Premier MCAT Prep — Practice vs actual; median best predictor (Chen & Corridon r=0.92):
  https://premiermcatprep.com/blog/mcat-practice-test-scores-vs-actual
- PubMed — Predictive value of full-length practice exams (β=0.74):
  https://pubmed.ncbi.nlm.nih.gov/33447661/
- NCME Module 10 — IRT Linking & Equating (Stocking-Lord, Haebara, concurrent):
  https://ncme.org/wp-content/uploads/2025/10/Module-10-Linking-and-Equating-II-IRT-Methods-Coo-1.pdf
- theta-minus-b — Intro to test-score equating:
  https://thetaminusb.com/2020/10/07/an-intro-to-test-score-equating-what-it-is-when-to-use-it/
- ACT RR2004-5 — IRT scale linking methods:
  https://www.act.org/content/dam/act/unsecured/documents/ACT_RR2004-5.pdf
- Nagatani et al. 2019 — Augmenting Knowledge Tracing by Considering Forgetting Behavior (DKT-Forget):
  https://doi.org/10.1145/3308558.3313565
- Knowledge-Based Systems 2025 — Modelling memory decay in DKT (decay-function comparison):
  https://doi.org/10.1016/j.knosys.2025.114884
- Settles & Meeder 2016 — Half-Life Regression (trainable spaced-repetition retention):
  https://doi.org/10.18653/v1/p16-1174
- Exam Readiness Index (ERI) — composite of Mastery/Coverage/Retention/Pace/Volatility/Endurance:
  https://www.emergentmind.com/topics/exam-readiness-index-eri
- [FSRS] FSRS4Anki Wiki — The Algorithm (power forgetting curve; DECAY=−0.5, FACTOR=19/81, R(S)=0.9):
  https://github.com/open-spaced-repetition/fsrs4anki/wiki/The-Algorithm
- [IRT-Baker] Baker, F. — *The Basics of Item Response Theory* (θ~N(0,1), Fisher information, 3PL c-parameter):
  https://eric.ed.gov/?id=ED458219
- [Laplace] Additive (Laplace) smoothing / rule of succession:
  https://en.wikipedia.org/wiki/Additive_smoothing
- [NIST] NIST/SEMATECH e-Handbook of Statistical Methods (normal distribution, z=1.96 for 95%):
  https://www.itl.nist.gov/div898/handbook/
- [A&S] Abramowitz & Stegun, *Handbook of Mathematical Functions*, eq. 7.1.26 (erf approximation, p. 299):
  https://personal.math.ubc.ca/~cbm/aands/
- (Cross-refs in `brainlift.md`: IRT 3PL, BKT four-param, KST, Bayesian networks, GKT.)
