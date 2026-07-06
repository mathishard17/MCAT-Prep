#!/usr/bin/env python3
"""Held-out CALIBRATION eval for the MCAT Prep *memory* model (rubric Step 1).

What this proves
----------------
The rubric's Sunday Step 1 (required): "show your memory model is calibrated.
When it says 80%, the student should recall about 80% of the time. Prove it on
held-back reviews." This script does exactly that against a PRE-COMMITTED cutoff.

The model under test
--------------------
This reimplements, in pure Python, the app's `card_memory` recall probability
(`anki/rslib/src/scheduler/concept_demo.rs`):

  * FSRS path (a memory state / stability is available) -- the FSRS-5 power
    forgetting curve  R(t) = (1 + FACTOR * t / S) ** (-DECAY)  with
    DECAY = 0.5 and FACTOR = 0.9 ** (1 / -DECAY) - 1, evaluated at a forward
    horizon of at least one day (t = max(elapsed_days, 1)), exactly like the
    app's MEMORY_HORIZON.
  * Fallback path (no FSRS state) -- base_from_last_rating * exp(-elapsed_days /
    max(interval_days, 1)), with Again=0.20 Hard=0.55 Good=0.80 Easy=0.90
    (see Track E of `docs/next-feature-expansion-plan.md`).

Metrics: a reliability table (predicted vs observed by decile), Brier score,
log-loss, and Expected Calibration Error (ECE), plus a hand-built SVG
reliability diagram at evals/memory-calibration.svg. Pass/fail is decided
against the PRE-COMMITTED CUTOFFS constant below.

HONESTY (this is graded)
------------------------
The held-out review set is SYNTHETIC / SIMULATED. No real longitudinal student
recall data is obtainable in a week, so `datasets/memory_reviews.jsonl` is a
seeded, reproducible simulation. The ground-truth `recalled` outcome is drawn
from a process that is DELIBERATELY DIFFERENT from the model's own prediction
(the app's forgetting curve on NOISELESS latent features, passed through a
deliberate over-confidence warp + occasional hidden lapses + Bernoulli sampling,
while the model only sees NOISY observed features), so the calibration numbers
can actually reveal miscalibration instead of being circular. This evaluates the
CALIBRATION OF THE MEMORY MODELING STEP, not a validated final readiness score.
Validation against real students (rubric Step 4) is future work.

Run:  python evals/calibration.py           (stdlib only; no network, no key)
      python evals/calibration.py --regenerate   (rebuild the held-out dataset)
Writes evals/memory-calibration.svg and the MEMORY section of evals/MODEL-EVALS.md,
and exits non-zero if any pre-committed cutoff fails.
"""
from __future__ import annotations

import json
import math
import random
import sys
import xml.dom.minidom
from pathlib import Path

HERE = Path(__file__).resolve().parent
DATASETS = HERE / "datasets"
DATASET = DATASETS / "memory_reviews.jsonl"
SVG_OUT = HERE / "memory-calibration.svg"
REPORT = HERE / "MODEL-EVALS.md"

# ---- PRE-COMMITTED cutoffs (fixed before running; not tuned post hoc) --------
CUTOFFS = {
    "brier": 0.25,   # Brier score must be <= this (0.25 == an all-0.5 coin flip)
    "ece": 0.10,     # Expected Calibration Error must be <= this
}

# ---- The app's memory model (mirrors concept_demo.rs::card_memory) -----------
FSRS_DECAY = 0.5                                  # FSRS5_DEFAULT_DECAY
FSRS_FACTOR = 0.9 ** (1.0 / -FSRS_DECAY) - 1.0    # = 19/81 ~= 0.2345679
MEMORY_HORIZON_DAYS = 1.0                         # MEMORY_HORIZON_SECS / 86400
BASE_RECALL = {1: 0.20, 2: 0.55, 3: 0.80, 4: 0.90}  # base_recall_for_button


def clamp01(x: float) -> float:
    return max(0.0, min(1.0, x))


def predict_recall(row: dict) -> float:
    """Predicted P(recall) for one review, using the shipped app's formula."""
    if row.get("has_fsrs") and row.get("stability") is not None:
        # FSRS-5 power forgetting curve at a >= 1 day forward horizon.
        t = max(float(row["elapsed_days"]), MEMORY_HORIZON_DAYS)
        s = max(float(row["stability"]), 1e-6)
        return clamp01((1.0 + FSRS_FACTOR * t / s) ** (-FSRS_DECAY))
    # Rating-decay fallback (no FSRS memory state): uses the REAL elapsed time.
    base = BASE_RECALL.get(int(row["last_rating"]))
    if base is None:
        return 0.5
    interval = max(float(row.get("interval_days", 1)), 1.0)
    decay_factor = math.exp(-float(row["elapsed_days"]) / interval)
    return clamp01(base * decay_factor)


# ---- Held-out dataset generation (SYNTHETIC, seeded, reproducible) -----------
# Ground truth is drawn from a process DIFFERENT from the model above so the
# calibration metric is meaningful (see the HONESTY note in the module docstring).
# The "true" recall prob is the model's OWN forgetting curve evaluated on the
# NOISELESS latent features, then passed through a deliberate miscalibration warp
# (a temperature + bias overconfidence transform) plus occasional hidden lapses;
# the outcome is a Bernoulli draw. The model, at eval time, only sees NOISY
# observed features (FSRS stability is estimated with error). So the reported gap
# comes from (a) the deliberate warp, (b) stability-estimation noise, (c) lapses
# the model cannot see, and (d) sampling noise -- none of which the model can
# undo, which is exactly what a calibration metric should surface.
GEN_SEED = 20260705
GEN_N = 1600
# Rating mix roughly like real review logs (mostly Good, some lapses).
RATING_WEIGHTS = {1: 0.15, 2: 0.20, 3: 0.45, 4: 0.20}
# Median "true" stability (days) implied by the last rating.
TRUE_STABILITY_MEDIAN = {1: 1.0, 2: 3.5, 3: 14.0, 4: 40.0}
TRUE_STABILITY_SIGMA = 0.55       # lognormal spread of true stability
FSRS_AVAILABLE_PROB = 0.78        # fraction of reviews with an FSRS memory state
STABILITY_NOISE_SIGMA = 0.20      # FSRS stability-ESTIMATION error (log-normal)
# Deliberate miscalibration: the model is mildly OVER-confident. The true recall
# logit is the model's logit divided by a temperature (>1 => softer/less extreme
# than predicted) plus a small negative bias. This is the "deliberate
# miscalibration" the rubric example calls for; the model cannot see it.
TRUE_TEMPERATURE = 1.18
TRUE_BIAS = -0.12
LAPSE_PROB = 0.05                 # "tapped Good without reading" / a bad day
LAPSE_FACTOR = 0.50               # recall multiplier when a hidden lapse happens
# Elapsed time as a multiple of true stability: a mixture of on-schedule reviews
# (most reviews sit near the FSRS target retrievability) and overdue/lapsed ones,
# so predicted recall spans the whole [0,1] range instead of clumping near 0.9.
OVERDUE_PROB = 0.32
ON_SCHEDULE_LOG_MU, ON_SCHEDULE_LOG_SIGMA = -0.10, 0.60   # median ~0.9x interval
OVERDUE_LOG_MU, OVERDUE_LOG_SIGMA = 1.60, 0.85            # median ~5x interval


def logit(p: float) -> float:
    p = min(1.0 - 1e-9, max(1e-9, p))
    return math.log(p / (1.0 - p))


def sigmoid(z: float) -> float:
    return 1.0 / (1.0 + math.exp(-z))


def ideal_model_recall(
    rating: int, elapsed_days: float, true_stability: float, interval_days: int, has_fsrs: bool
) -> float:
    """The app's formula on NOISELESS latent features (the truth's starting point)."""
    if has_fsrs:
        t = max(elapsed_days, MEMORY_HORIZON_DAYS)
        return clamp01((1.0 + FSRS_FACTOR * t / max(true_stability, 1e-6)) ** (-FSRS_DECAY))
    base = BASE_RECALL[rating]
    return clamp01(base * math.exp(-elapsed_days / max(interval_days, 1)))


def generate_dataset(path: Path, seed: int = GEN_SEED, n: int = GEN_N) -> None:
    rng = random.Random(seed)
    ratings = list(RATING_WEIGHTS)
    weights = [RATING_WEIGHTS[r] for r in ratings]

    rows = []
    for i in range(n):
        rating = rng.choices(ratings, weights=weights, k=1)[0]
        true_stability = TRUE_STABILITY_MEDIAN[rating] * math.exp(
            rng.gauss(0.0, TRUE_STABILITY_SIGMA)
        )
        if rng.random() < OVERDUE_PROB:
            elapsed_multiple = math.exp(rng.gauss(OVERDUE_LOG_MU, OVERDUE_LOG_SIGMA))
        else:
            elapsed_multiple = math.exp(rng.gauss(ON_SCHEDULE_LOG_MU, ON_SCHEDULE_LOG_SIGMA))
        elapsed_days = max(true_stability * elapsed_multiple, 0.02)
        interval_days = max(1, round(true_stability))
        has_fsrs = rng.random() < FSRS_AVAILABLE_PROB

        # Ground-truth: model curve on true features -> deliberate warp -> lapse -> draw.
        p_ideal = ideal_model_recall(
            rating, elapsed_days, true_stability, interval_days, has_fsrs
        )
        p_true = sigmoid(logit(p_ideal) / TRUE_TEMPERATURE + TRUE_BIAS)
        if rng.random() < LAPSE_PROB:
            p_true *= LAPSE_FACTOR
        p_true = min(0.999, max(0.001, p_true))
        recalled = 1 if rng.random() < p_true else 0

        # Observed features the MODEL is allowed to see (no p_true / true_stability leak).
        if has_fsrs:
            stability_obs = round(
                true_stability * math.exp(rng.gauss(0.0, STABILITY_NOISE_SIGMA)), 4
            )
        else:
            stability_obs = None

        rows.append(
            {
                "id": f"MEM-{i + 1:04d}",
                "last_rating": rating,
                "elapsed_days": round(elapsed_days, 4),
                "interval_days": interval_days,
                "stability": stability_obs,
                "has_fsrs": has_fsrs,
                "recalled": recalled,
            }
        )

    header = (
        "# SYNTHETIC / SIMULATED held-out review set for memory-model calibration.\n"
        "# Reproducible: generated by evals/calibration.py "
        f"(random seed {seed}, n={n}).\n"
        "# Ground-truth `recalled` is drawn from a process DIFFERENT from the model's\n"
        "# prediction: the app's forgetting curve on NOISELESS latent features, passed\n"
        "# through a deliberate overconfidence warp (temperature+bias) + hidden lapses,\n"
        "# then Bernoulli-sampled; the model only sees NOISY observed features. So the\n"
        "# calibration metric is meaningful (not circular). NOT real student data.\n"
        "# Regenerate: python evals/calibration.py --regenerate\n"
    )
    path.parent.mkdir(parents=True, exist_ok=True)
    lines = [header.rstrip("\n")]
    lines.extend(json.dumps(r, sort_keys=True) for r in rows)
    path.write_text("\n".join(lines) + "\n", encoding="utf-8")


def load_dataset(path: Path) -> list[dict]:
    rows = []
    for raw in path.read_text(encoding="utf-8").splitlines():
        line = raw.strip()
        if not line or line.startswith("#"):
            continue
        rows.append(json.loads(line))
    return rows


# ---- Metrics ----------------------------------------------------------------
def brier_score(preds: list[float], ys: list[int]) -> float:
    return sum((p - y) ** 2 for p, y in zip(preds, ys)) / len(ys)


def log_loss(preds: list[float], ys: list[int], eps: float = 1e-12) -> float:
    total = 0.0
    for p, y in zip(preds, ys):
        p = min(1.0 - eps, max(eps, p))
        total += -(y * math.log(p) + (1 - y) * math.log(1.0 - p))
    return total / len(ys)


def reliability_bins(preds: list[float], ys: list[int], n_bins: int = 10) -> list[dict]:
    """Equal-width probability bins (deciles of the [0,1] range)."""
    bins = []
    for b in range(n_bins):
        lo = b / n_bins
        hi = (b + 1) / n_bins
        idx = [
            i
            for i, p in enumerate(preds)
            if (p >= lo and (p < hi or (b == n_bins - 1 and p <= hi)))
        ]
        count = len(idx)
        mean_pred = sum(preds[i] for i in idx) / count if count else 0.0
        obs_freq = sum(ys[i] for i in idx) / count if count else 0.0
        bins.append(
            {"lo": lo, "hi": hi, "count": count, "mean_pred": mean_pred, "obs_freq": obs_freq}
        )
    return bins


def expected_calibration_error(bins: list[dict], total: int) -> float:
    return sum(
        (b["count"] / total) * abs(b["mean_pred"] - b["obs_freq"]) for b in bins if b["count"]
    )


def max_calibration_error(bins: list[dict]) -> float:
    populated = [abs(b["mean_pred"] - b["obs_freq"]) for b in bins if b["count"]]
    return max(populated) if populated else 0.0


# ---- Hand-built SVG reliability diagram (stdlib only, ASCII/entities only) ---
def esc(text: str) -> str:
    return text.replace("&", "&amp;").replace("<", "&lt;").replace(">", "&gt;")


def build_svg(bins: list[dict], brier: float, ece: float, total: int) -> str:
    # Canvas + plot geometry.
    W, H = 640, 500
    ml, mr, mt, mb = 70, 30, 70, 90
    pw = W - ml - mr
    ph = H - mt - mb

    def px(x: float) -> float:
        return ml + x * pw

    def py(y: float) -> float:
        return mt + (1.0 - y) * ph

    parts = []
    parts.append(
        f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" '
        f'viewBox="0 0 {W} {H}" font-family="Helvetica,Arial,sans-serif">'
    )
    parts.append(f'<rect x="0" y="0" width="{W}" height="{H}" fill="#ffffff"/>')
    parts.append(
        f'<text x="{W/2:.0f}" y="28" text-anchor="middle" font-size="18" '
        f'font-weight="bold" fill="#111827">Memory model reliability diagram</text>'
    )
    parts.append(
        f'<text x="{W/2:.0f}" y="48" text-anchor="middle" font-size="12" '
        f'fill="#6b7280">SYNTHETIC held-out reviews (n={total}) -- predicted vs '
        f'observed recall by decile</text>'
    )

    # Gridlines + ticks (0.0 .. 1.0 on both axes).
    for k in range(11):
        v = k / 10.0
        gx, gy = px(v), py(v)
        parts.append(
            f'<line x1="{gx:.1f}" y1="{mt}" x2="{gx:.1f}" y2="{mt+ph}" '
            f'stroke="#eef2f7" stroke-width="1"/>'
        )
        parts.append(
            f'<line x1="{ml}" y1="{gy:.1f}" x2="{ml+pw}" y2="{gy:.1f}" '
            f'stroke="#eef2f7" stroke-width="1"/>'
        )
        parts.append(
            f'<text x="{gx:.1f}" y="{mt+ph+18:.0f}" text-anchor="middle" '
            f'font-size="10" fill="#6b7280">{v:.1f}</text>'
        )
        parts.append(
            f'<text x="{ml-8:.0f}" y="{gy+3:.1f}" text-anchor="end" '
            f'font-size="10" fill="#6b7280">{v:.1f}</text>'
        )

    # Plot border.
    parts.append(
        f'<rect x="{ml}" y="{mt}" width="{pw}" height="{ph}" fill="none" '
        f'stroke="#9ca3af" stroke-width="1.5"/>'
    )
    # Perfect-calibration diagonal (dashed).
    parts.append(
        f'<line x1="{px(0):.1f}" y1="{py(0):.1f}" x2="{px(1):.1f}" y2="{py(1):.1f}" '
        f'stroke="#9ca3af" stroke-width="1.5" stroke-dasharray="6,5"/>'
    )
    parts.append(
        f'<text x="{px(0.83):.1f}" y="{py(0.90):.1f}" font-size="10" '
        f'fill="#9ca3af" transform="rotate(-33 {px(0.83):.1f} {py(0.90):.1f})">'
        f'perfect calibration</text>'
    )

    # Reliability curve through populated bins + point markers sized by count.
    pts = [(px(b["mean_pred"]), py(b["obs_freq"])) for b in bins if b["count"]]
    if len(pts) >= 2:
        poly = " ".join(f"{x:.1f},{y:.1f}" for x, y in pts)
        parts.append(
            f'<polyline points="{poly}" fill="none" stroke="#2563eb" stroke-width="2"/>'
        )
    max_count = max((b["count"] for b in bins if b["count"]), default=1)
    for b in bins:
        if not b["count"]:
            continue
        cx, cy = px(b["mean_pred"]), py(b["obs_freq"])
        r = 3.0 + 6.0 * math.sqrt(b["count"] / max_count)
        parts.append(
            f'<circle cx="{cx:.1f}" cy="{cy:.1f}" r="{r:.1f}" fill="#2563eb" '
            f'fill-opacity="0.65" stroke="#1e3a8a" stroke-width="1"/>'
        )

    # Axis labels.
    parts.append(
        f'<text x="{ml+pw/2:.0f}" y="{H-42:.0f}" text-anchor="middle" '
        f'font-size="13" fill="#374151">Predicted recall probability</text>'
    )
    parts.append(
        f'<text x="20" y="{mt+ph/2:.0f}" text-anchor="middle" font-size="13" '
        f'fill="#374151" transform="rotate(-90 20 {mt+ph/2:.0f})">'
        f'Observed recall frequency</text>'
    )
    # Metric caption (ASCII only; "&lt;=" for the cutoff comparator).
    verdict = "PASS" if (brier <= CUTOFFS["brier"] and ece <= CUTOFFS["ece"]) else "FAIL"
    caption = (
        f'Brier {brier:.4f} (cutoff &lt;= {CUTOFFS["brier"]})   '
        f'ECE {ece:.4f} (cutoff &lt;= {CUTOFFS["ece"]})   [{verdict}]'
    )
    parts.append(
        f'<text x="{W/2:.0f}" y="{H-14:.0f}" text-anchor="middle" font-size="12" '
        f'fill="#111827">{caption}</text>'
    )
    parts.append("</svg>")
    return "\n".join(parts)


# ---- Report writing (updates the MEMORY section of MODEL-EVALS.md) -----------
MEMORY_BEGIN = "<!-- BEGIN:MEMORY -->"
MEMORY_END = "<!-- END:MEMORY -->"
PERF_BEGIN = "<!-- BEGIN:PERFORMANCE -->"
PERF_END = "<!-- END:PERFORMANCE -->"


def report_scaffold() -> str:
    return "\n".join(
        [
            "# MCAT Prep -- held-out MODEL evaluations",
            "",
            "Two reproducible, held-out evaluations of the **modeling steps** behind the",
            "app's three scores (rubric Sunday, Steps 1 and 2):",
            "",
            "1. **Memory-model calibration** (`calibration.py`) -- rubric Step 1.",
            "2. **Performance-model accuracy** (`performance_eval.py`) -- rubric Step 2.",
            "",
            "> **HONESTY (read this first).** Both held-out datasets are **synthetic /",
            "> simulated** and seeded for reproducibility -- there is no real longitudinal",
            "> student practice-test data obtainable in a week. These evals therefore",
            "> measure the **calibration / accuracy of the modeling steps**, not a",
            "> validated final readiness score. Each ground-truth label is drawn from a",
            "> process **deliberately different** from the model being scored, so the",
            "> numbers can reveal real miscalibration rather than being circular.",
            "> Validation against **real students** with study history + practice-test",
            "> scores (rubric Step 4) is **future work**. An honest \"calibrated on",
            "> synthetic held-out reviews; real-student validation pending\" is the claim",
            "> being made here -- nothing more.",
            "",
            "Reproduce (stdlib only, offline, no API key):",
            "",
            "```sh",
            "python evals/calibration.py        # memory calibration + SVG chart",
            "python evals/performance_eval.py   # performance accuracy + baselines",
            "```",
            "",
            "Each script re-derives its numbers from the committed seeded dataset and",
            "rewrites its section below; both exit non-zero if a pre-committed cutoff fails.",
            "",
            "## 1. Memory-model calibration (Step 1)",
            "",
            MEMORY_BEGIN,
            "_Not run yet -- run `python evals/calibration.py`._",
            MEMORY_END,
            "",
            "## 2. Performance-model accuracy (Step 2)",
            "",
            PERF_BEGIN,
            "_Not run yet -- run `python evals/performance_eval.py`._",
            PERF_END,
            "",
        ]
    )


def update_report_section(begin: str, end: str, body: str) -> None:
    text = REPORT.read_text(encoding="utf-8") if REPORT.exists() else report_scaffold()
    if begin not in text or end not in text:
        text = report_scaffold()
    head = text[: text.index(begin) + len(begin)]
    tail = text[text.index(end):]
    REPORT.write_text(head + "\n" + body + "\n" + tail, encoding="utf-8")


def fmt_reliability_table(bins: list[dict]) -> list[str]:
    rows = [
        "| Predicted bin | n | Mean predicted | Observed recall | Gap |",
        "| --- | ---: | ---: | ---: | ---: |",
    ]
    for b in bins:
        if not b["count"]:
            rows.append(
                f"| {b['lo']:.1f}-{b['hi']:.1f} | 0 | -- | -- | -- |"
            )
            continue
        gap = b["mean_pred"] - b["obs_freq"]
        rows.append(
            f"| {b['lo']:.1f}-{b['hi']:.1f} | {b['count']} | {b['mean_pred']:.3f} "
            f"| {b['obs_freq']:.3f} | {gap:+.3f} |"
        )
    return rows


def main(argv: list[str]) -> int:
    regenerate = "--regenerate" in argv[1:]
    if regenerate or not DATASET.exists():
        generate_dataset(DATASET)
        print(f"{'Regenerated' if regenerate else 'Generated'} held-out dataset: {DATASET}")

    rows = load_dataset(DATASET)
    preds = [predict_recall(r) for r in rows]
    ys = [int(r["recalled"]) for r in rows]
    n = len(rows)

    brier = brier_score(preds, ys)
    ll = log_loss(preds, ys)
    bins = reliability_bins(preds, ys)
    ece = expected_calibration_error(bins, n)
    mce = max_calibration_error(bins)
    mean_pred = sum(preds) / n
    obs_rate = sum(ys) / n
    n_fsrs = sum(1 for r in rows if r.get("has_fsrs") and r.get("stability") is not None)

    checks = {"brier": brier <= CUTOFFS["brier"], "ece": ece <= CUTOFFS["ece"]}
    all_pass = all(checks.values())

    # Emit the SVG and validate it parses as XML.
    svg = build_svg(bins, brier, ece, n)
    SVG_OUT.write_text(svg + "\n", encoding="utf-8")
    xml.dom.minidom.parseString(svg)  # raises if the hand-built SVG is malformed

    # Console summary.
    print("\n=== Memory-model calibration (SYNTHETIC held-out reviews) ===")
    print(f"reviews: {n}  (FSRS path: {n_fsrs}, rating-decay fallback: {n - n_fsrs})")
    print(f"observed recall rate: {obs_rate:.3f}   mean predicted: {mean_pred:.3f}")
    print(f"Brier: {brier:.4f}  (cutoff <= {CUTOFFS['brier']}) -> "
          f"{'PASS' if checks['brier'] else 'FAIL'}")
    print(f"log-loss: {ll:.4f}")
    print(f"ECE: {ece:.4f}  (cutoff <= {CUTOFFS['ece']}) -> "
          f"{'PASS' if checks['ece'] else 'FAIL'}")
    print(f"MCE: {mce:.4f}")
    print("reliability by decile (predicted -> observed):")
    for b in bins:
        if not b["count"]:
            print(f"  {b['lo']:.1f}-{b['hi']:.1f}: (empty)")
            continue
        print(f"  {b['lo']:.1f}-{b['hi']:.1f}: n={b['count']:4d}  "
              f"pred={b['mean_pred']:.3f}  obs={b['obs_freq']:.3f}  "
              f"gap={b['mean_pred'] - b['obs_freq']:+.3f}")
    print(f"\nwrote {SVG_OUT}")
    print(f"overall: {'ALL CUTOFFS MET' if all_pass else 'SOME CUTOFFS NOT MET'}")

    # Markdown section for MODEL-EVALS.md.
    body = [
        "**Model under test:** the app's `card_memory` recall probability",
        "(`anki/rslib/src/scheduler/concept_demo.rs`) reimplemented in Python -- the",
        "FSRS-5 power forgetting curve `R(t) = (1 + FACTOR * t / S) ** (-DECAY)` "
        "(`DECAY = 0.5`)",
        "for cards with an FSRS stability, and the `base_from_last_rating * "
        "exp(-elapsed / max(interval, 1))`",
        "rating-decay fallback otherwise (Track E of `next-feature-expansion-plan.md`).",
        "",
        f"**Held-out set:** `datasets/memory_reviews.jsonl` -- **{n}** SYNTHETIC reviews "
        f"(seed `{GEN_SEED}`; "
        f"{n_fsrs} FSRS-path, {n - n_fsrs} rating-decay-fallback). Ground truth is drawn from a "
        "process distinct from the model: the app's forgetting curve on *noiseless* latent "
        "features, passed through a deliberate over-confidence warp (temperature+bias) plus "
        "hidden lapses, then Bernoulli-sampled; the model only sees *noisy* observed features.",
        "",
        "| Metric | Value | Pre-committed cutoff | Result |",
        "| --- | ---: | --- | :---: |",
        f"| Brier score | **{brier:.4f}** | <= {CUTOFFS['brier']} | "
        f"{'PASS' if checks['brier'] else 'FAIL'} |",
        f"| Expected Calibration Error (ECE) | **{ece:.4f}** | <= {CUTOFFS['ece']} | "
        f"{'PASS' if checks['ece'] else 'FAIL'} |",
        f"| Log-loss | {ll:.4f} | (reported) | -- |",
        f"| Max Calibration Error (MCE) | {mce:.4f} | (reported) | -- |",
        f"| Calibration-in-the-large (mean pred vs obs) | {mean_pred:.3f} vs {obs_rate:.3f} "
        f"| (reported) | -- |",
        "",
        f"**Overall: {'ALL CUTOFFS MET' if all_pass else 'SOME CUTOFFS NOT MET'}.**",
        "",
        "Reliability diagram (predicted vs observed recall; diagonal = perfect "
        "calibration):",
        "",
        "![Memory calibration reliability diagram](memory-calibration.svg)",
        "",
        "Reliability by decile:",
        "",
        *fmt_reliability_table(bins),
        "",
        "_Reading it:_ points on the dashed diagonal are perfectly calibrated; points "
        "**below** it mean the model was **over-confident** in that bin (predicted recall "
        "exceeded observed). Marker area is proportional to the number of reviews in the bin.",
    ]
    update_report_section(MEMORY_BEGIN, MEMORY_END, "\n".join(body))
    print(f"updated {REPORT} (MEMORY section)")

    return 0 if all_pass else 1


if __name__ == "__main__":
    sys.exit(main(sys.argv))
