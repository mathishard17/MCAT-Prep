#!/usr/bin/env python3
"""Render ONE secondary-metrics chart for the study-feature ablation (rubric section 8).

The MAIN ablation result (prerequisite violations) is drawn by `evals/ablation.py`
into `evals/ablation.svg`. This standalone companion renders the SECONDARY,
scaffolding-conditioned metrics into `evals/ablation-secondary.svg` WITHOUT
touching `ablation.py` (or any other existing script):

  * a grouped bar chart of the three metrics that share a 0-1 axis -- held-out
    exam accuracy, memory (mean recall), and coverage (KCs touched) -- with
    ON / OFF / PLAIN bars per group, per-seed min/max whiskers, and the ON-PLAIN
    delta annotated above each group; and
  * a small separate inset for projected readiness, which lives on the MCAT
    472-528 total-score scale (NOT 0-1) and so must not share the 0-1 axis; its
    engine-computed 95% band is shown as a whisker.

Numbers are read from the committed engine fixture `evals/fixtures/ablation.json`
(emitted by `concept_ablation.rs::tests::emit_ablation_fixture`); the same values
also appear in the table in `evals/ABLATION.md`.

HONESTY (this is graded)
------------------------
These SECONDARY metrics assume a stated *scaffolding* effect (a card studied with
unmet prerequisites is learned less effectively); the MAIN number (prerequisite
violations) does NOT. The honest null -- an identically-shaped deck with no
prerequisite edges -- leaves the feature inert and all three arms equal. Learners
are SYNTHETIC and seeded (12 seeds); real-student validation is future work.

Run:  python3 evals/plot_ablation_secondary.py   (stdlib only; no network, no key)
Writes evals/ablation-secondary.svg and validates it parses as XML.
"""
from __future__ import annotations

import json
import sys
import xml.dom.minidom
from pathlib import Path

HERE = Path(__file__).resolve().parent
FIXTURE = HERE / "fixtures" / "ablation.json"
SVG_OUT = HERE / "ablation-secondary.svg"

ARMS = ("on", "off", "plain")
# Palette (per spec): green = feature ON, greys = the OFF/PLAIN baselines.
COLORS = {"on": "#16a34a", "off": "#9ca3af", "plain": "#6b7280"}
# Whiskers + bar outlines in a darker shade of each series.
DARK = {"on": "#166534", "off": "#6b7280", "plain": "#374151"}
ARM_LEGEND = {
    "on": "Concept scheduler ON",
    "off": "Feature OFF (ablation)",
    "plain": "Plain Anki",
}
# The three metrics that share the 0-1 axis (readiness is handled separately).
GROUPS = [
    ("accuracy", "Held-out accuracy"),
    ("memory", "Memory (recall)"),
    ("coverage", "Coverage (KCs)"),
]
# Readiness lives on the MCAT total-score scale, not 0-1.
READY_MIN, READY_MAX = 472.0, 528.0


def load_fixture() -> dict:
    if not FIXTURE.exists():
        sys.stderr.write(
            f"ERROR: fixture not found: {FIXTURE}\n"
            "Emit it from the real engine first:\n"
            "  cd anki && CARGO_TARGET_DIR=out/rust cargo test -p anki --features rustls "
            "emit_ablation_fixture -- --nocapture\n"
        )
        raise SystemExit(2)
    return json.loads(FIXTURE.read_text(encoding="utf-8"))


def esc(text: str) -> str:
    return text.replace("&", "&amp;").replace("<", "&lt;").replace(">", "&gt;")


def tw(s: str, size: float) -> float:
    """Rough Helvetica advance-width estimate, used only to center text runs."""
    return len(s) * size * 0.55


def whisker(cx: float, ya: float, yb: float, cap: float, color: str) -> list[str]:
    """A min/max (or band) whisker: a vertical stem with a cap at each end."""
    lo, hi = min(ya, yb), max(ya, yb)  # lo = upper point (smaller y), hi = lower
    return [
        f'<line x1="{cx:.1f}" y1="{lo:.1f}" x2="{cx:.1f}" y2="{hi:.1f}" '
        f'stroke="{color}" stroke-width="1.5"/>',
        f'<line x1="{cx-cap:.1f}" y1="{lo:.1f}" x2="{cx+cap:.1f}" y2="{lo:.1f}" '
        f'stroke="{color}" stroke-width="1.5"/>',
        f'<line x1="{cx-cap:.1f}" y1="{hi:.1f}" x2="{cx+cap:.1f}" y2="{hi:.1f}" '
        f'stroke="{color}" stroke-width="1.5"/>',
    ]


# ---- Hand-built SVG (stdlib only; ASCII/entities only), mirroring calibration.py -
def build_svg(main: dict) -> str:
    W, H = 720, 500
    # Main grouped-bar panel on a shared 0-1 axis.
    ML, MT = 60, 86
    MAIN_W, MAIN_H = 380, 266
    MR, MBOT = ML + MAIN_W, MT + MAIN_H  # right edge 440, baseline 352

    def y01(v: float) -> float:
        return MT + (1.0 - v) * MAIN_H

    # Readiness inset on its own 472-528 axis (kept off the 0-1 axis on purpose).
    IL, IW = 520, 120
    IR = IL + IW  # 640
    ICX = (IL + IR) / 2.0  # 580

    def yr(v: float) -> float:
        return MT + (1.0 - (v - READY_MIN) / (READY_MAX - READY_MIN)) * MAIN_H

    parts: list[str] = [
        f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" '
        f'viewBox="0 0 {W} {H}" font-family="Helvetica,Arial,sans-serif">',
        f'<rect x="0" y="0" width="{W}" height="{H}" fill="#ffffff"/>',
        f'<text x="{W/2:.0f}" y="30" text-anchor="middle" font-size="18" '
        f'font-weight="bold" fill="#111827">Study-feature ablation: secondary metrics</text>',
        f'<text x="{W/2:.0f}" y="52" text-anchor="middle" font-size="12" fill="#6b7280">'
        f'SYNTHETIC, 12 seeds; scaffolding-conditioned secondary metrics</text>',
    ]

    # Main panel: horizontal gridlines + 0-1 ticks, border, rotated y-axis title.
    for k in range(6):
        v = k * 0.2
        gy = y01(v)
        parts.append(
            f'<line x1="{ML}" y1="{gy:.1f}" x2="{MR}" y2="{gy:.1f}" '
            f'stroke="#eef2f7" stroke-width="1"/>'
        )
        parts.append(
            f'<text x="{ML-8}" y="{gy+3:.1f}" text-anchor="end" font-size="10" '
            f'fill="#6b7280">{v:.1f}</text>'
        )
    parts.append(
        f'<rect x="{ML}" y="{MT}" width="{MAIN_W}" height="{MAIN_H}" fill="none" '
        f'stroke="#9ca3af" stroke-width="1.5"/>'
    )
    ycx = MT + MAIN_H / 2.0
    parts.append(
        f'<text x="18" y="{ycx:.0f}" text-anchor="middle" font-size="13" fill="#374151" '
        f'transform="rotate(-90 18 {ycx:.0f})">Metric value (0-1)</text>'
    )

    # Grouped bars + whiskers + ON-PLAIN delta + group label, per metric.
    slot = MAIN_W / len(GROUPS)
    cluster = slot * 0.72
    bar_w = cluster / len(ARMS)
    for gi, (key, label) in enumerate(GROUPS):
        metric = main[key]
        gcx = ML + slot * (gi + 0.5)
        x0 = gcx - cluster / 2.0
        for ai, arm in enumerate(ARMS):
            s = metric[arm]
            bx = x0 + bar_w * ai
            bcx = bx + bar_w / 2.0
            by = y01(s["mean"])
            parts.append(
                f'<rect x="{bx:.1f}" y="{by:.1f}" width="{bar_w:.1f}" '
                f'height="{MBOT-by:.1f}" fill="{COLORS[arm]}" stroke="{DARK[arm]}" '
                f'stroke-width="1"/>'
            )
            parts.extend(
                whisker(bcx, y01(s["min"]), y01(s["max"]), bar_w * 0.30, DARK[arm])
            )
        delta = metric["on"]["mean"] - metric["plain"]["mean"]
        top_v = max(metric[a]["max"] for a in ARMS)
        parts.append(
            f'<text x="{gcx:.1f}" y="{y01(top_v)-8:.1f}" text-anchor="middle" '
            f'font-size="10" font-weight="bold" fill="#111827">ON - PLAIN {delta:+.3f}</text>'
        )
        parts.append(
            f'<text x="{gcx:.1f}" y="{MBOT+18:.0f}" text-anchor="middle" font-size="11" '
            f'fill="#374151">{esc(label)}</text>'
        )

    # Readiness inset: header, own gridlines/ticks, border, bars + 95% band whiskers.
    parts.append(
        f'<text x="{ICX:.0f}" y="66" text-anchor="middle" font-size="11" '
        f'fill="#374151">Projected readiness</text>'
    )
    parts.append(
        f'<text x="{ICX:.0f}" y="79" text-anchor="middle" font-size="9" '
        f'fill="#6b7280">(472-528 MCAT scale)</text>'
    )
    for t in (472, 486, 500, 514, 528):
        gy = yr(t)
        parts.append(
            f'<line x1="{IL}" y1="{gy:.1f}" x2="{IR}" y2="{gy:.1f}" '
            f'stroke="#eef2f7" stroke-width="1"/>'
        )
        parts.append(
            f'<text x="{IL-6}" y="{gy+3:.1f}" text-anchor="end" font-size="9" '
            f'fill="#6b7280">{t}</text>'
        )
    parts.append(
        f'<rect x="{IL}" y="{MT}" width="{IW}" height="{MAIN_H}" fill="none" '
        f'stroke="#9ca3af" stroke-width="1.5"/>'
    )
    islot = IW / len(ARMS)
    ibar_w = islot * 0.5
    ready = main["readiness_total"]
    band_lo = main["readiness_lower"]
    band_hi = main["readiness_upper"]
    for ai, arm in enumerate(ARMS):
        bcx = IL + islot * (ai + 0.5)
        bx = bcx - ibar_w / 2.0
        by = yr(ready[arm]["mean"])
        parts.append(
            f'<rect x="{bx:.1f}" y="{by:.1f}" width="{ibar_w:.1f}" '
            f'height="{MBOT-by:.1f}" fill="{COLORS[arm]}" stroke="{DARK[arm]}" '
            f'stroke-width="1"/>'
        )
        parts.extend(
            whisker(bcx, yr(band_lo[arm]["mean"]), yr(band_hi[arm]["mean"]),
                    ibar_w * 0.30, DARK[arm])
        )
    # Value labels on ON and PLAIN only (OFF == PLAIN on readiness).
    on_cx = IL + islot * 0.5
    plain_cx = IL + islot * 2.5
    parts.append(
        f'<text x="{on_cx:.1f}" y="{yr(band_hi["on"]["mean"])-6:.1f}" text-anchor="middle" '
        f'font-size="9" font-weight="bold" fill="#111827">{ready["on"]["mean"]:.1f}</text>'
    )
    parts.append(
        f'<text x="{plain_cx:.1f}" y="{yr(band_hi["plain"]["mean"])-6:.1f}" '
        f'text-anchor="middle" font-size="9" fill="#111827">{ready["plain"]["mean"]:.1f}</text>'
    )
    ready_delta = ready["on"]["mean"] - ready["plain"]["mean"]
    parts.append(
        f'<text x="{ICX:.0f}" y="{MBOT+16:.0f}" text-anchor="middle" font-size="10" '
        f'font-weight="bold" fill="#111827">ON - PLAIN {ready_delta:+.1f}</text>'
    )
    parts.append(
        f'<text x="{ICX:.0f}" y="{MBOT+30:.0f}" text-anchor="middle" font-size="9" '
        f'fill="#6b7280">whisker = 95% band</text>'
    )

    # Legend (color chip + label per arm), centered across the full width.
    legend_y = 404
    chip = 12
    size = 12
    gap_ct = 6
    gap_item = 26
    widths = [chip + gap_ct + tw(ARM_LEGEND[a], size) for a in ARMS]
    total = sum(widths) + gap_item * (len(ARMS) - 1)
    x = (W - total) / 2.0
    for a, wd in zip(ARMS, widths):
        parts.append(
            f'<rect x="{x:.1f}" y="{legend_y-chip+1:.1f}" width="{chip}" height="{chip}" '
            f'fill="{COLORS[a]}" stroke="{DARK[a]}" stroke-width="1"/>'
        )
        parts.append(
            f'<text x="{x+chip+gap_ct:.1f}" y="{legend_y:.1f}" font-size="{size}" '
            f'fill="#374151">{esc(ARM_LEGEND[a])}</text>'
        )
        x += wd + gap_item

    # Bottom caption: whisker meaning + scaffolding assumption + the honest null.
    parts.append(
        f'<text x="{W/2:.0f}" y="{H-30:.0f}" text-anchor="middle" font-size="12" '
        f'fill="#111827">Whiskers = per-seed min/max; readiness whisker = 95% band. '
        f'Assumes a scaffolding effect.</text>'
    )
    parts.append(
        f'<text x="{W/2:.0f}" y="{H-13:.0f}" text-anchor="middle" font-size="11" '
        f'fill="#6b7280">Honest null: with no prerequisite edges the feature is inert and '
        f'all three arms are equal.</text>'
    )
    parts.append("</svg>")
    return "\n".join(parts)


def main() -> int:
    fx = load_fixture()
    m = fx["main"]

    svg = build_svg(m)
    xml.dom.minidom.parseString(svg)  # raises if the hand-built SVG is malformed
    SVG_OUT.write_text(svg + "\n", encoding="utf-8")

    acc_d = m["accuracy"]["on"]["mean"] - m["accuracy"]["plain"]["mean"]
    mem_d = m["memory"]["on"]["mean"] - m["memory"]["plain"]["mean"]
    cov_d = m["coverage"]["on"]["mean"] - m["coverage"]["plain"]["mean"]
    rdy_d = m["readiness_total"]["on"]["mean"] - m["readiness_total"]["plain"]["mean"]

    print("=== Study-feature ablation: secondary-metrics chart ===")
    print(f"data source: {FIXTURE}")
    print(
        f"ON-PLAIN deltas: accuracy {acc_d:+.3f}  memory {mem_d:+.3f}  "
        f"coverage {cov_d:+.3f}  readiness {rdy_d:+.1f}"
    )
    print(f"wrote {SVG_OUT}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
