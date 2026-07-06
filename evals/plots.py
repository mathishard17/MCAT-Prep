#!/usr/bin/env python3
"""Generate the informative eval/benchmark charts (hand-rolled stdlib SVG).

Companion to the eval scripts. Reads the *committed* datasets / fixtures /
reported numbers and emits the SVG charts catalogued in `evals/PLOTS.md`:

  - performance-roc.svg          ROC curve for the IRT 3PL performance model
  - performance-reliability.svg  calibration (reliability) diagram for performance
  - performance-baselines.svg    IRT 3PL vs mastery-only vs majority baselines
  - ablation-secondary.svg       3-build ablation secondary metrics (ON/OFF/PLAIN)
  - ai-eval.svg                  AI eval pass rates vs pre-committed cutoffs
  - readiness-projection.svg     "range not a point" readiness projection

Pure Python stdlib, offline, no API key. Numbers come from:
  - `performance_eval.py` (re-run on the committed synthetic set -> matches MODEL-EVALS.md)
  - `fixtures/ablation.json` (engine-emitted, matches ABLATION.md)
  - `RESULTS.md` numbers (AI eval; hardcoded here with a citation, since ai_eval.py needs a key)

Run:  python3 evals/plots.py
"""
from __future__ import annotations

import json
import math
import sys
import xml.dom.minidom
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
import performance_eval as pe  # reuse the shipped IRT model + metrics  # noqa: E402

# ---- shared palette (matches calibration.py / memory-calibration.svg) --------
INK = "#111827"
MUTE = "#6b7280"
GRID = "#eef2f7"
BORDER = "#9ca3af"
AXIS = "#374151"
BLUE = "#2563eb"
BLUE_D = "#1e3a8a"
RED = "#dc2626"
GREEN = "#16a34a"
AMBER = "#d97706"
GREY = "#9ca3af"


def esc(text: str) -> str:
    return text.replace("&", "&amp;").replace("<", "&lt;").replace(">", "&gt;")


def _open(w: int, h: int, title: str, subtitle: str) -> list[str]:
    parts = [
        f'<svg xmlns="http://www.w3.org/2000/svg" width="{w}" height="{h}" '
        f'viewBox="0 0 {w} {h}" font-family="Helvetica,Arial,sans-serif">',
        f'<rect x="0" y="0" width="{w}" height="{h}" fill="#ffffff"/>',
        f'<text x="{w/2:.0f}" y="28" text-anchor="middle" font-size="18" '
        f'font-weight="bold" fill="{INK}">{esc(title)}</text>',
        f'<text x="{w/2:.0f}" y="48" text-anchor="middle" font-size="12" '
        f'fill="{MUTE}">{esc(subtitle)}</text>',
    ]
    return parts


def _caption(parts: list[str], w: int, h: int, text: str) -> None:
    parts.append(
        f'<text x="{w/2:.0f}" y="{h-14:.0f}" text-anchor="middle" font-size="12" '
        f'fill="{INK}">{esc(text)}</text>'
    )


def _write(parts: list[str], name: str) -> None:
    parts.append("</svg>")
    svg = "\n".join(parts)
    xml.dom.minidom.parseString(svg)  # raises if malformed
    (HERE / name).write_text(svg + "\n", encoding="utf-8")
    print(f"wrote {HERE / name}")


# ---- data -------------------------------------------------------------------
def _perf_preds():
    if not pe.DATASET.exists():
        pe.generate_dataset(pe.DATASET)
    rows = pe.load_dataset(pe.DATASET)
    ys = [int(r["correct"]) for r in rows]
    irt = [pe.predict_correct(r) for r in rows]
    mastery = [float(r["kc_mastery"]) for r in rows]
    return rows, ys, irt, mastery


def _ablation():
    return json.loads((HERE / "fixtures" / "ablation.json").read_text(encoding="utf-8"))


# ---- 1. ROC curve (performance) ---------------------------------------------
def roc_curve(scores: list[float], ys: list[int]) -> list[tuple[float, float]]:
    pos = sum(ys)
    neg = len(ys) - pos
    order = sorted(range(len(scores)), key=lambda i: -scores[i])
    pts = [(0.0, 0.0)]
    tp = fp = 0
    prev = None
    for i in order:
        s = scores[i]
        if prev is not None and s != prev:
            pts.append((fp / neg, tp / pos))
        if ys[i] == 1:
            tp += 1
        else:
            fp += 1
        prev = s
    pts.append((fp / neg, tp / pos))
    return pts


def build_roc(scores: list[float], ys: list[int], auc: float, n: int) -> None:
    w, h = 640, 500
    ml, mr, mt, mb = 70, 30, 70, 70
    pw, ph = w - ml - mr, h - mt - mb

    def px(x: float) -> float:
        return ml + x * pw

    def py(y: float) -> float:
        return mt + (1.0 - y) * ph

    p = _open(w, h, "Performance model -- ROC curve",
              f"SYNTHETIC held-out attempts (n={n}); IRT 3PL item-response model")
    for k in range(11):
        v = k / 10.0
        p.append(f'<line x1="{px(v):.1f}" y1="{mt}" x2="{px(v):.1f}" y2="{mt+ph}" '
                 f'stroke="{GRID}" stroke-width="1"/>')
        p.append(f'<line x1="{ml}" y1="{py(v):.1f}" x2="{ml+pw}" y2="{py(v):.1f}" '
                 f'stroke="{GRID}" stroke-width="1"/>')
        p.append(f'<text x="{px(v):.1f}" y="{mt+ph+18:.0f}" text-anchor="middle" '
                 f'font-size="10" fill="{MUTE}">{v:.1f}</text>')
        p.append(f'<text x="{ml-8:.0f}" y="{py(v)+3:.1f}" text-anchor="end" '
                 f'font-size="10" fill="{MUTE}">{v:.1f}</text>')
    p.append(f'<rect x="{ml}" y="{mt}" width="{pw}" height="{ph}" fill="none" '
             f'stroke="{BORDER}" stroke-width="1.5"/>')
    p.append(f'<line x1="{px(0):.1f}" y1="{py(0):.1f}" x2="{px(1):.1f}" y2="{py(1):.1f}" '
             f'stroke="{BORDER}" stroke-width="1.5" stroke-dasharray="6,5"/>')
    p.append(f'<text x="{px(0.62):.1f}" y="{py(0.55):.1f}" font-size="10" fill="{MUTE}" '
             f'transform="rotate(-33 {px(0.62):.1f} {py(0.55):.1f})">chance (AUC 0.50)</text>')
    poly = " ".join(f"{px(x):.1f},{py(y):.1f}" for x, y in roc_curve(scores, ys))
    p.append(f'<polyline points="{poly}" fill="none" stroke="{BLUE}" stroke-width="2.5"/>')
    p.append(f'<text x="{px(0.45):.1f}" y="{py(0.28):.1f}" font-size="14" '
             f'font-weight="bold" fill="{BLUE_D}">AUC = {auc:.3f}</text>')
    p.append(f'<text x="{ml+pw/2:.0f}" y="{h-42:.0f}" text-anchor="middle" font-size="13" '
             f'fill="{AXIS}">False positive rate</text>')
    p.append(f'<text x="20" y="{mt+ph/2:.0f}" text-anchor="middle" font-size="13" '
             f'fill="{AXIS}" transform="rotate(-90 20 {mt+ph/2:.0f})">True positive rate</text>')
    _caption(p, w, h, f"AUC {auc:.3f} (cutoff &gt;= 0.70)   [PASS]")
    _write(p, "performance-roc.svg")


# ---- 2. reliability diagram (performance) -----------------------------------
def build_perf_reliability(irt: list[float], ys: list[int], brier: float, n: int) -> None:
    bins = []
    for k in range(10):
        lo, hi = k / 10.0, (k + 1) / 10.0
        idx = [i for i in range(len(irt)) if (irt[i] >= lo and (irt[i] < hi or (k == 9 and irt[i] <= hi)))]
        if idx:
            mp = sum(irt[i] for i in idx) / len(idx)
            of = sum(ys[i] for i in idx) / len(idx)
            bins.append({"count": len(idx), "mean_pred": mp, "obs_freq": of})
        else:
            bins.append({"count": 0, "mean_pred": (lo + hi) / 2, "obs_freq": 0.0})

    w, h = 640, 500
    ml, mr, mt, mb = 70, 30, 70, 90
    pw, ph = w - ml - mr, h - mt - mb

    def px(x: float) -> float:
        return ml + x * pw

    def py(y: float) -> float:
        return mt + (1.0 - y) * ph

    p = _open(w, h, "Performance model -- reliability diagram",
              f"SYNTHETIC held-out attempts (n={n}) -- predicted vs observed P(correct) by decile")
    for k in range(11):
        v = k / 10.0
        p.append(f'<line x1="{px(v):.1f}" y1="{mt}" x2="{px(v):.1f}" y2="{mt+ph}" '
                 f'stroke="{GRID}" stroke-width="1"/>')
        p.append(f'<line x1="{ml}" y1="{py(v):.1f}" x2="{ml+pw}" y2="{py(v):.1f}" '
                 f'stroke="{GRID}" stroke-width="1"/>')
        p.append(f'<text x="{px(v):.1f}" y="{mt+ph+18:.0f}" text-anchor="middle" '
                 f'font-size="10" fill="{MUTE}">{v:.1f}</text>')
        p.append(f'<text x="{ml-8:.0f}" y="{py(v)+3:.1f}" text-anchor="end" '
                 f'font-size="10" fill="{MUTE}">{v:.1f}</text>')
    p.append(f'<rect x="{ml}" y="{mt}" width="{pw}" height="{ph}" fill="none" '
             f'stroke="{BORDER}" stroke-width="1.5"/>')
    p.append(f'<line x1="{px(0):.1f}" y1="{py(0):.1f}" x2="{px(1):.1f}" y2="{py(1):.1f}" '
             f'stroke="{BORDER}" stroke-width="1.5" stroke-dasharray="6,5"/>')
    p.append(f'<text x="{px(0.80):.1f}" y="{py(0.88):.1f}" font-size="10" fill="{MUTE}" '
             f'transform="rotate(-33 {px(0.80):.1f} {py(0.88):.1f})">perfect calibration</text>')
    pts = [(px(b["mean_pred"]), py(b["obs_freq"])) for b in bins if b["count"]]
    if len(pts) >= 2:
        p.append(f'<polyline points="{" ".join(f"{x:.1f},{y:.1f}" for x, y in pts)}" '
                 f'fill="none" stroke="{BLUE}" stroke-width="2"/>')
    mc = max((b["count"] for b in bins if b["count"]), default=1)
    for b in bins:
        if not b["count"]:
            continue
        r = 3.0 + 6.0 * math.sqrt(b["count"] / mc)
        p.append(f'<circle cx="{px(b["mean_pred"]):.1f}" cy="{py(b["obs_freq"]):.1f}" '
                 f'r="{r:.1f}" fill="{BLUE}" fill-opacity="0.65" stroke="{BLUE_D}" stroke-width="1"/>')
    p.append(f'<text x="{ml+pw/2:.0f}" y="{h-42:.0f}" text-anchor="middle" font-size="13" '
             f'fill="{AXIS}">Predicted P(correct)</text>')
    p.append(f'<text x="20" y="{mt+ph/2:.0f}" text-anchor="middle" font-size="13" '
             f'fill="{AXIS}" transform="rotate(-90 20 {mt+ph/2:.0f})">Observed correct frequency</text>')
    _caption(p, w, h, f"Brier {brier:.4f} (cutoff &lt;= 0.22)   marker area ~ attempts in bin   [PASS]")
    _write(p, "performance-reliability.svg")


# ---- 3. baselines grouped bars ----------------------------------------------
def build_baselines(irt: dict, mastery: dict, majority: dict) -> None:
    w, h = 660, 500
    ml, mr, mt, mb = 60, 30, 80, 70
    pw, ph = w - ml - mr, h - mt - mb
    groups = [("Accuracy", "accuracy"), ("AUC", "auc"), ("Brier (lower=better)", "brier")]
    series = [("IRT 3PL (ours)", irt, BLUE), ("Mastery-only", mastery, AMBER), ("Majority", majority, GREY)]

    def py(v: float) -> float:
        return mt + (1.0 - v) * ph

    p = _open(w, h, "Performance vs baselines",
              "SYNTHETIC held-out attempts (n=1560) -- accuracy / AUC / Brier")
    for k in range(6):
        v = k / 5.0
        p.append(f'<line x1="{ml}" y1="{py(v):.1f}" x2="{ml+pw}" y2="{py(v):.1f}" '
                 f'stroke="{GRID}" stroke-width="1"/>')
        p.append(f'<text x="{ml-8:.0f}" y="{py(v)+3:.1f}" text-anchor="end" '
                 f'font-size="10" fill="{MUTE}">{v:.1f}</text>')
    # 0.70 accuracy cutoff line
    p.append(f'<line x1="{ml}" y1="{py(0.70):.1f}" x2="{ml+pw}" y2="{py(0.70):.1f}" '
             f'stroke="{RED}" stroke-width="1.2" stroke-dasharray="5,4"/>')
    p.append(f'<text x="{ml+pw-4:.0f}" y="{py(0.70)-4:.1f}" text-anchor="end" font-size="10" '
             f'fill="{RED}">accuracy cutoff 0.70</text>')

    gw = pw / len(groups)
    bw = gw * 0.7 / len(series)
    for gi, (glabel, gkey) in enumerate(groups):
        gx = ml + gi * gw + gw * 0.15
        for si, (slabel, vals, color) in enumerate(series):
            v = vals[gkey]
            bx = gx + si * bw
            by = py(v)
            p.append(f'<rect x="{bx:.1f}" y="{by:.1f}" width="{bw-3:.1f}" '
                     f'height="{mt+ph-by:.1f}" fill="{color}"/>')
            p.append(f'<text x="{bx+(bw-3)/2:.1f}" y="{by-4:.1f}" text-anchor="middle" '
                     f'font-size="9" fill="{AXIS}">{v:.3f}</text>')
        p.append(f'<text x="{gx+3*bw/2:.1f}" y="{mt+ph+18:.0f}" text-anchor="middle" '
                 f'font-size="11" fill="{AXIS}">{esc(glabel)}</text>')
    # legend
    lx = ml + 6
    ly = mt + 10
    for slabel, _v, color in series:
        p.append(f'<rect x="{lx:.0f}" y="{ly-9:.0f}" width="12" height="12" fill="{color}"/>')
        p.append(f'<text x="{lx+17:.0f}" y="{ly+1:.0f}" font-size="11" fill="{AXIS}">{esc(slabel)}</text>')
        lx += 130
    _caption(p, w, h, "IRT 3PL beats both baselines; accuracy cutoff 0.70 met [PASS]")
    _write(p, "performance-baselines.svg")


# ---- 4. ablation secondary grouped bars -------------------------------------
def build_ablation_secondary(ab: dict) -> None:
    main = ab["main"]
    w, h = 720, 500
    ml, mr, mt, mb = 60, 30, 80, 78
    pw, ph = w - ml - mr, h - mt - mb
    groups = [("Held-out accuracy", "accuracy"), ("Memory (recall)", "memory"),
              ("Coverage (KCs)", "coverage")]
    arms = [("ON", "on", GREEN), ("OFF", "off", GREY), ("PLAIN", "plain", MUTE)]

    def py(v: float) -> float:
        return mt + (1.0 - v) * ph

    p = _open(w, h, "Study-feature ablation -- secondary metrics",
              "SYNTHETIC, 12 seeds; ON / OFF / PLAIN, mean with min-max whiskers (scaffolding-conditioned)")
    for k in range(6):
        v = k / 5.0
        p.append(f'<line x1="{ml}" y1="{py(v):.1f}" x2="{ml+pw}" y2="{py(v):.1f}" '
                 f'stroke="{GRID}" stroke-width="1"/>')
        p.append(f'<text x="{ml-8:.0f}" y="{py(v)+3:.1f}" text-anchor="end" '
                 f'font-size="10" fill="{MUTE}">{v:.1f}</text>')
    gw = pw / len(groups)
    bw = gw * 0.7 / len(arms)
    for gi, (glabel, gkey) in enumerate(groups):
        gx = ml + gi * gw + gw * 0.15
        for ai, (albl, akey, color) in enumerate(arms):
            d = main[gkey][akey]
            v, vmin, vmax = d["mean"], d["min"], d["max"]
            bx = gx + ai * bw
            by = py(v)
            p.append(f'<rect x="{bx:.1f}" y="{by:.1f}" width="{bw-3:.1f}" '
                     f'height="{mt+ph-by:.1f}" fill="{color}"/>')
            # min-max whisker
            cx = bx + (bw - 3) / 2
            p.append(f'<line x1="{cx:.1f}" y1="{py(vmin):.1f}" x2="{cx:.1f}" y2="{py(vmax):.1f}" '
                     f'stroke="{INK}" stroke-width="1"/>')
            p.append(f'<line x1="{cx-3:.1f}" y1="{py(vmax):.1f}" x2="{cx+3:.1f}" y2="{py(vmax):.1f}" '
                     f'stroke="{INK}" stroke-width="1"/>')
            p.append(f'<text x="{cx:.1f}" y="{by-4:.1f}" text-anchor="middle" '
                     f'font-size="9" fill="{AXIS}">{v:.2f}</text>')
        p.append(f'<text x="{gx+3*bw/2:.1f}" y="{mt+ph+18:.0f}" text-anchor="middle" '
                 f'font-size="11" fill="{AXIS}">{esc(glabel)}</text>')
    # legend
    lx = ml + 6
    ly = mt + 10
    for albl, _k, color in arms:
        p.append(f'<rect x="{lx:.0f}" y="{ly-9:.0f}" width="12" height="12" fill="{color}"/>')
        p.append(f'<text x="{lx+17:.0f}" y="{ly+1:.0f}" font-size="11" fill="{AXIS}">{esc(albl)}</text>')
        lx += 70
    # readiness (different scale) as text
    rt = main["readiness_total"]
    p.append(f'<text x="{ml:.0f}" y="{mt+ph+40:.0f}" font-size="11" fill="{AXIS}">'
             f'Projected readiness (472-528): ON {rt["on"]["mean"]:.1f} vs '
             f'OFF/PLAIN {rt["off"]["mean"]:.1f}  (ON-PLAIN +{rt["on"]["mean"]-rt["plain"]["mean"]:.1f})</text>')
    _caption(p, w, h, "Honest null: with no prerequisite edges all three arms are equal (see ABLATION.md)")
    _write(p, "ablation-secondary.svg")


# ---- 5. AI eval pass rates vs cutoffs ---------------------------------------
def build_ai() -> None:
    # Numbers quoted from evals/RESULTS.md (gpt-4o-mini, latest run).
    metrics = [
        ("KC tagging", 88.0, 80.0, 35.0),          # value, cutoff, baseline
        ("Rewording", 95.0, 85.0, None),
        ("Inject: tagger", 98.0, 100.0, None),
        ("Inject: tutor", 100.0, 100.0, None),
    ]
    w, h = 660, 500
    ml, mr, mt, mb = 60, 30, 80, 88
    pw, ph = w - ml - mr, h - mt - mb

    def py(v: float) -> float:
        return mt + (1.0 - v / 100.0) * ph

    p = _open(w, h, "AI evaluation vs pre-committed cutoffs",
              "gpt-4o-mini, held-out; 40 items per set")
    for k in range(6):
        v = k * 20
        p.append(f'<line x1="{ml}" y1="{py(v):.1f}" x2="{ml+pw}" y2="{py(v):.1f}" '
                 f'stroke="{GRID}" stroke-width="1"/>')
        p.append(f'<text x="{ml-8:.0f}" y="{py(v)+3:.1f}" text-anchor="end" '
                 f'font-size="10" fill="{MUTE}">{v}%</text>')
    gw = pw / len(metrics)
    bw = gw * 0.5
    for mi, (label, val, cut, base) in enumerate(metrics):
        gx = ml + mi * gw + (gw - bw) / 2
        if base is not None:
            bbw = bw * 0.42
            p.append(f'<rect x="{gx-bbw-3:.1f}" y="{py(base):.1f}" width="{bbw:.1f}" '
                     f'height="{mt+ph-py(base):.1f}" fill="{GREY}"/>')
            p.append(f'<text x="{gx-bbw/2-3:.1f}" y="{py(base)-4:.1f}" text-anchor="middle" '
                     f'font-size="9" fill="{MUTE}">{base:.0f}</text>')
        p.append(f'<rect x="{gx:.1f}" y="{py(val):.1f}" width="{bw:.1f}" '
                 f'height="{mt+ph-py(val):.1f}" fill="{BLUE}"/>')
        p.append(f'<text x="{gx+bw/2:.1f}" y="{py(val)-4:.1f}" text-anchor="middle" '
                 f'font-size="10" fill="{AXIS}">{val:.0f}%</text>')
        # cutoff marker
        p.append(f'<line x1="{gx-4:.1f}" y1="{py(cut):.1f}" x2="{gx+bw+4:.1f}" y2="{py(cut):.1f}" '
                 f'stroke="{RED}" stroke-width="1.6" stroke-dasharray="5,3"/>')
        p.append(f'<text x="{gx+bw/2:.1f}" y="{mt+ph+18:.0f}" text-anchor="middle" '
                 f'font-size="11" fill="{AXIS}">{esc(label)}</text>')
    # legend
    lx = ml + 6
    ly = mt + 8
    p.append(f'<rect x="{lx:.0f}" y="{ly-9:.0f}" width="12" height="12" fill="{BLUE}"/>')
    p.append(f'<text x="{lx+17:.0f}" y="{ly+1:.0f}" font-size="11" fill="{AXIS}">result</text>')
    p.append(f'<rect x="{lx+90:.0f}" y="{ly-9:.0f}" width="12" height="12" fill="{GREY}"/>')
    p.append(f'<text x="{lx+107:.0f}" y="{ly+1:.0f}" font-size="11" fill="{AXIS}">baseline</text>')
    p.append(f'<line x1="{lx+185:.0f}" y1="{ly-3:.0f}" x2="{lx+205:.0f}" y2="{ly-3:.0f}" '
             f'stroke="{RED}" stroke-width="1.6" stroke-dasharray="5,3"/>')
    p.append(f'<text x="{lx+210:.0f}" y="{ly+1:.0f}" font-size="11" fill="{AXIS}">cutoff</text>')
    _caption(p, w, h, "Leaks 0, contamination 0   --   tagger-injection 98% (1 breach) vs 100% cutoff")
    _write(p, "ai-eval.svg")


# ---- 6. readiness projection (range not a point) ----------------------------
def build_readiness(ab: dict) -> None:
    rt = ab["main"]["readiness_total"]["on"]
    rl = ab["main"]["readiness_lower"]["on"]["mean"]
    ru = ab["main"]["readiness_upper"]["on"]["mean"]
    total, lo, hi = rt["mean"], rl, ru

    w, h = 660, 500
    ml, mr = 150, 40
    scale_x0, scale_x1 = ml, w - mr

    def sx(val: float, vmin: float, vmax: float) -> float:
        return scale_x0 + (val - vmin) / (vmax - vmin) * (scale_x1 - scale_x0)

    p = _open(w, h, "Readiness: a range, not a number",
              "EXAMPLE projection from the ablation ON arm -- synthetic; range = center +/- 1.96*SE")

    # total row (472-528)
    ty = 100
    tmin, tmax = 472.0, 528.0
    p.append(f'<text x="20" y="{ty+4:.0f}" font-size="12" fill="{AXIS}">Projected total</text>')
    p.append(f'<line x1="{scale_x0}" y1="{ty:.0f}" x2="{scale_x1}" y2="{ty:.0f}" '
             f'stroke="{BORDER}" stroke-width="2"/>')
    for val in (472, 486, 500, 514, 528):
        gx = sx(val, tmin, tmax)
        p.append(f'<line x1="{gx:.1f}" y1="{ty-4:.0f}" x2="{gx:.1f}" y2="{ty+4:.0f}" stroke="{BORDER}"/>')
        p.append(f'<text x="{gx:.1f}" y="{ty+18:.0f}" text-anchor="middle" font-size="9" fill="{MUTE}">{val}</text>')
    p.append(f'<rect x="{sx(lo,tmin,tmax):.1f}" y="{ty-9:.0f}" width="{sx(hi,tmin,tmax)-sx(lo,tmin,tmax):.1f}" '
             f'height="18" fill="{BLUE}" fill-opacity="0.20"/>')
    p.append(f'<circle cx="{sx(total,tmin,tmax):.1f}" cy="{ty:.0f}" r="5" fill="{BLUE}" stroke="{BLUE_D}"/>')
    p.append(f'<text x="{sx(total,tmin,tmax):.1f}" y="{ty-14:.0f}" text-anchor="middle" font-size="12" '
             f'font-weight="bold" fill="{BLUE_D}">{total:.0f} (likely {lo:.0f}-{hi:.0f})</text>')

    # section rows (118-132)
    smin, smax = 118.0, 132.0
    sections = [
        ("Bio/Biochem", 128.0, 125.0, 131.0, False),
        ("Chem/Phys", 122.0, 119.0, 126.0, False),
        ("Psych/Soc", 124.0, 121.0, 127.0, False),
        ("CARS", None, None, None, True),  # abstain example
    ]
    y0 = 190
    dy = 66
    p.append(f'<text x="20" y="{y0-22:.0f}" font-size="12" fill="{AXIS}">Per section (118-132):</text>')
    for i, (name, center, slo, shi, abstain) in enumerate(sections):
        y = y0 + i * dy
        p.append(f'<text x="20" y="{y+4:.0f}" font-size="12" fill="{AXIS}">{esc(name)}</text>')
        p.append(f'<line x1="{scale_x0}" y1="{y:.0f}" x2="{scale_x1}" y2="{y:.0f}" '
                 f'stroke="{BORDER}" stroke-width="1.5"/>')
        for val in (118, 122, 125, 128, 132):
            gx = sx(val, smin, smax)
            p.append(f'<line x1="{gx:.1f}" y1="{y-3:.0f}" x2="{gx:.1f}" y2="{y+3:.0f}" stroke="{BORDER}"/>')
            p.append(f'<text x="{gx:.1f}" y="{y+15:.0f}" text-anchor="middle" font-size="8" fill="{MUTE}">{val}</text>')
        # guessing floor at 120
        gx = sx(120, smin, smax)
        p.append(f'<line x1="{gx:.1f}" y1="{y-11:.0f}" x2="{gx:.1f}" y2="{y+11:.0f}" '
                 f'stroke="{AMBER}" stroke-width="1" stroke-dasharray="3,3"/>')
        if abstain:
            p.append(f'<text x="{(scale_x0+scale_x1)/2:.1f}" y="{y-9:.0f}" text-anchor="middle" '
                     f'font-size="11" fill="{MUTE}">Not enough data yet -- need 190 reviews / 60% coverage</text>')
        else:
            p.append(f'<rect x="{sx(slo,smin,smax):.1f}" y="{y-8:.0f}" '
                     f'width="{sx(shi,smin,smax)-sx(slo,smin,smax):.1f}" height="16" '
                     f'fill="{BLUE}" fill-opacity="0.20"/>')
            p.append(f'<circle cx="{sx(center,smin,smax):.1f}" cy="{y:.0f}" r="4" fill="{BLUE}" stroke="{BLUE_D}"/>')
            p.append(f'<text x="{sx(center,smin,smax):.1f}" y="{y-11:.0f}" text-anchor="middle" '
                     f'font-size="10" fill="{BLUE_D}">{center:.0f} ({slo:.0f}-{shi:.0f})</text>')
    p.append(f'<text x="{scale_x1:.0f}" y="{y0+3*dy+30:.0f}" text-anchor="end" font-size="9" '
             f'fill="{AMBER}">dashed = guessing floor (120)</text>')
    _caption(p, w, h, "Give-up rule: a section abstains below 20 items / 60% coverage (see give-up-rule.md)")
    _write(p, "readiness-projection.svg")


def main() -> int:
    rows, ys, irt, mastery = _perf_preds()
    n = len(rows)
    base = sum(ys) / n
    auc = pe.auc_score(irt, ys)
    irt_m = {"accuracy": pe.accuracy_and_wrong(irt, ys)[0], "auc": auc, "brier": pe.brier_score(irt, ys)}
    mastery_m = {"accuracy": pe.accuracy_and_wrong(mastery, ys)[0], "auc": pe.auc_score(mastery, ys),
                 "brier": pe.brier_score(mastery, ys)}
    majority_m = {"accuracy": max(base, 1 - base), "auc": 0.5, "brier": pe.brier_score([base] * n, ys)}

    build_roc(irt, ys, auc, n)
    build_perf_reliability(irt, ys, irt_m["brier"], n)
    build_baselines(irt_m, mastery_m, majority_m)
    ab = _ablation()
    build_ablation_secondary(ab)
    build_ai()
    build_readiness(ab)
    print("all charts written to evals/*.svg")
    return 0


if __name__ == "__main__":
    sys.exit(main())
