#!/usr/bin/env python3
"""Render the READINESS PROJECTION as a *range, not a point* (rubric honesty rule).

What this draws
---------------
One hand-built SVG (`evals/readiness-projection.svg`) that visualizes the honesty
rule from `docs/model-readiness.md`: a projected MCAT score is shown as a RANGE
with per-section bands, never a bare number. Layout:

  * Top -- the projected TOTAL on a 472-528 scale: a point marker at the center
    with a shaded 95% confidence band, labelled "Projected MCAT 489.6
    (likely 481-499)". The 500 median is a faint reference.
  * Below -- four per-section rows on the 118-132 scale (Bio/Biochem, Chem/Phys,
    Psych/Soc, CARS), each a center dot + `center +/- 1.96*SE` band. ONE section
    (CARS) is rendered as an ABSTAIN ("Not enough data yet ...") to show the
    give-up rule (`docs/give-up-rule.md`): a section needs >= 20 items AND
    >= 60% coverage before it commits to a number.
  * A faint dashed guessing-floor line at 120 on the section scale (uncovered /
    forgotten material is pulled toward the ~120 floor, not the 125 median).

HONESTY (this is graded)
------------------------
This is an EXAMPLE / illustrative projection, NOT a validated MCAT score. The
projected total (489.6, 95% band 481-499) is the concept-scheduler ON-arm number
from the study-feature ablation (`evals/ABLATION.md`), which itself uses
SYNTHETIC, seeded learners. The four per-section centers below it are plausible
values *constructed* to sum to that 489.6 total; they are illustrative, not
measured. Per `docs/model-readiness.md`, there is no held-out validation of the
final projected score against real students yet -- that is future work.

Design + palette deliberately mirror `evals/calibration.py::build_svg` /
`evals/memory-calibration.svg` (pure stdlib; ASCII/entities only; no unicode
glyphs; validated with xml.dom.minidom before writing).

Run:  python3 evals/plot_readiness.py     (stdlib only; no network, no key)
Writes evals/readiness-projection.svg and prints its path.
"""
from __future__ import annotations

import json
import sys
import xml.dom.minidom
from pathlib import Path

HERE = Path(__file__).resolve().parent
SVG_OUT = HERE / "readiness-projection.svg"

# ---- The example projection (see the HONESTY note in the module docstring) ----
# TOTAL: the concept-scheduler ON-arm projected readiness from evals/ABLATION.md
# (synthetic learners) -- projected total 489.6 with an engine-computed 95% band
# of 481-499 on the 472-528 MCAT scale.
TOTAL = {
    "center": 489.6,   # ablation ON-arm projected total (472-528 scale)
    "lo": 481.0,       # engine-computed 95% band, from evals/ABLATION.md
    "hi": 499.0,
    "min": 472.0,      # MCAT total scale endpoints
    "max": 528.0,
    "median": 500.0,   # population median total (for reference only)
}

# Per-section give-up thresholds (docs/give-up-rule.md): a section commits to a
# range only with >= MIN_ITEMS graded items AND >= MIN_COVERAGE blueprint coverage.
MIN_ITEMS = 20
MIN_COVERAGE = 0.60
SEC_MIN, SEC_MAX = 118.0, 132.0   # MCAT per-section scale
GUESS_FLOOR = 120.0               # guessing-baseline score (docs/model-readiness.md)
SEC_MEDIAN = 125.0                # per-section population median (reference only)

# Four plausible per-section centers that SUM to the 489.6 total (illustrative,
# NOT measured). `se` is the standard error used for the center +/- 1.96*SE band.
# CARS is below the give-up line (12 items, 38% coverage) -> the display abstains,
# so its center (~120, pulled toward the floor) is shown only as a withheld value
# in the construction note, never as a committed number on the chart.
SECTIONS = [
    {"name": "Bio/Biochem", "center": 124.0, "se": 1.45, "items": 41, "coverage": 0.78},
    {"name": "Chem/Phys", "center": 122.5, "se": 1.68, "items": 33, "coverage": 0.64},
    {"name": "Psych/Soc", "center": 123.1, "se": 1.30, "items": 52, "coverage": 0.86},
    {"name": "CARS", "center": 120.0, "se": 2.10, "items": 12, "coverage": 0.38},
]

# ---- Palette (mirrors evals/calibration.py::build_svg exactly) ---------------
C_TITLE = "#111827"
C_SUB = "#6b7280"
C_GRID = "#eef2f7"
C_SCALE = "#9ca3af"
C_AXIS = "#374151"
C_POINT = "#2563eb"
C_POINT_STROKE = "#1e3a8a"
C_ABSTAIN = "#6b7280"
C_FLOOR = "#d97706"
C_WITHHELD = "#d1d5db"

# ---- Canvas + shared horizontal scale geometry ------------------------------
W, H = 640, 500
SX0, SX1 = 150.0, 605.0   # left / right pixel bounds of every horizontal scale


def esc(text: str) -> str:
    """Escape XML text content; keeps output ASCII (`--`, `+/-`, `&gt;=`)."""
    return text.replace("&", "&amp;").replace("<", "&lt;").replace(">", "&gt;")


def clamp(x: float, lo: float, hi: float) -> float:
    return max(lo, min(hi, x))


def x_total(score: float) -> float:
    """Map a total score (472-528) to an x pixel on the shared scale."""
    return SX0 + (score - TOTAL["min"]) / (TOTAL["max"] - TOTAL["min"]) * (SX1 - SX0)


def x_sec(score: float) -> float:
    """Map a section score (118-132) to an x pixel on the shared scale."""
    return SX0 + (score - SEC_MIN) / (SEC_MAX - SEC_MIN) * (SX1 - SX0)


def enough_evidence(sec: dict) -> bool:
    return sec["items"] >= MIN_ITEMS and sec["coverage"] >= MIN_COVERAGE


def build_svg() -> str:
    p: list[str] = []
    p.append(
        f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" '
        f'viewBox="0 0 {W} {H}" font-family="Helvetica,Arial,sans-serif">'
    )
    p.append(f'<rect x="0" y="0" width="{W}" height="{H}" fill="#ffffff"/>')

    # ---- Title + subtitle ----
    p.append(
        f'<text x="{W/2:.0f}" y="26" text-anchor="middle" font-size="18" '
        f'font-weight="bold" fill="{C_TITLE}">{esc("Readiness: a range, not a number")}</text>'
    )
    p.append(
        f'<text x="{W/2:.0f}" y="45" text-anchor="middle" font-size="12" '
        f'fill="{C_SUB}">'
        f'{esc("EXAMPLE projection from the ablation ON arm -- synthetic; range = center +/- 1.96*SE")}'
        f'</text>'
    )

    # ---- Legend (top-right): band swatch + point marker ----
    p.append(
        f'<rect x="402" y="60" width="20" height="12" fill="{C_POINT}" '
        f'fill-opacity="0.2" stroke="{C_POINT}" stroke-width="1"/>'
    )
    p.append(
        f'<text x="427" y="70" text-anchor="start" font-size="10" '
        f'fill="{C_AXIS}">{esc("95% range")}</text>'
    )
    p.append(
        f'<circle cx="412" cy="85" r="4" fill="{C_POINT}" stroke="{C_POINT_STROKE}" '
        f'stroke-width="1"/>'
    )
    p.append(
        f'<text x="427" y="88" text-anchor="start" font-size="10" '
        f'fill="{C_AXIS}">{esc("point estimate")}</text>'
    )

    # ---- TOTAL: 472-528 scale with band + marker --------------------------
    total_label = (
        f"Projected MCAT {TOTAL['center']:.1f} "
        f"(likely {TOTAL['lo']:.0f}-{TOTAL['hi']:.0f})"
    )
    p.append(
        f'<text x="24" y="72" text-anchor="start" font-size="15" font-weight="bold" '
        f'fill="{C_TITLE}">{esc(total_label)}</text>'
    )
    ty = 104.0
    # baseline scale line
    p.append(
        f'<line x1="{SX0:.1f}" y1="{ty:.1f}" x2="{SX1:.1f}" y2="{ty:.1f}" '
        f'stroke="{C_SCALE}" stroke-width="1.5"/>'
    )
    # faint median (500) reference
    mx = x_total(TOTAL["median"])
    p.append(
        f'<line x1="{mx:.1f}" y1="{ty-14:.1f}" x2="{mx:.1f}" y2="{ty+9:.1f}" '
        f'stroke="{C_GRID}" stroke-width="2"/>'
    )
    # shaded 95% band
    bx0, bx1 = x_total(TOTAL["lo"]), x_total(TOTAL["hi"])
    p.append(
        f'<rect x="{bx0:.1f}" y="{ty-9:.1f}" width="{bx1-bx0:.1f}" height="18" '
        f'fill="{C_POINT}" fill-opacity="0.2"/>'
    )
    for bx in (bx0, bx1):
        p.append(
            f'<line x1="{bx:.1f}" y1="{ty-9:.1f}" x2="{bx:.1f}" y2="{ty+9:.1f}" '
            f'stroke="{C_POINT}" stroke-width="1"/>'
        )
    # point marker
    cx = x_total(TOTAL["center"])
    p.append(
        f'<line x1="{cx:.1f}" y1="{ty-10:.1f}" x2="{cx:.1f}" y2="{ty+10:.1f}" '
        f'stroke="{C_POINT_STROKE}" stroke-width="1.5"/>'
    )
    p.append(
        f'<circle cx="{cx:.1f}" cy="{ty:.1f}" r="5" fill="{C_POINT}" '
        f'stroke="{C_POINT_STROKE}" stroke-width="1"/>'
    )
    total_center_label = f"{TOTAL['center']:.1f}"
    p.append(
        f'<text x="{cx:.1f}" y="{ty-14:.1f}" text-anchor="middle" font-size="10" '
        f'font-weight="bold" fill="{C_POINT_STROKE}">{esc(total_center_label)}</text>'
    )
    # scale endpoint + median tick labels
    for val, label in (
        (TOTAL["min"], f"{TOTAL['min']:.0f}"),
        (TOTAL["median"], f"{TOTAL['median']:.0f} (median)"),
        (TOTAL["max"], f"{TOTAL['max']:.0f}"),
    ):
        vx = x_total(val)
        p.append(
            f'<line x1="{vx:.1f}" y1="{ty+9:.1f}" x2="{vx:.1f}" y2="{ty+14:.1f}" '
            f'stroke="{C_SCALE}" stroke-width="1"/>'
        )
        p.append(
            f'<text x="{vx:.1f}" y="{ty+26:.1f}" text-anchor="middle" font-size="10" '
            f'fill="{C_SUB}">{esc(label)}</text>'
        )

    # ---- SECTIONS: four rows on the 118-132 scale -------------------------
    p.append(
        f'<text x="24" y="158" text-anchor="start" font-size="13" '
        f'fill="{C_AXIS}">{esc("By section (118-132 scale)")}</text>'
    )

    row0, step = 196.0, 52.0
    top_y = row0 - 16
    bot_y = row0 + step * (len(SECTIONS) - 1) + 16
    axis_y = bot_y + 6

    # faint median (125) reference line across the section rows
    smx = x_sec(SEC_MEDIAN)
    p.append(
        f'<line x1="{smx:.1f}" y1="{top_y:.1f}" x2="{smx:.1f}" y2="{axis_y:.1f}" '
        f'stroke="{C_GRID}" stroke-width="2"/>'
    )
    # guessing-floor (120) reference line (faint, dashed)
    fx = x_sec(GUESS_FLOOR)
    p.append(
        f'<line x1="{fx:.1f}" y1="{top_y:.1f}" x2="{fx:.1f}" y2="{axis_y:.1f}" '
        f'stroke="{C_FLOOR}" stroke-width="1" stroke-opacity="0.6" stroke-dasharray="5,4"/>'
    )
    p.append(
        f'<text x="{fx+5:.1f}" y="{top_y-3:.1f}" text-anchor="start" font-size="9" '
        f'fill="{C_FLOOR}">{esc("guessing floor 120")}</text>'
    )

    for i, sec in enumerate(SECTIONS):
        rc = row0 + i * step
        # left label column: name + evidence line
        p.append(
            f'<text x="24" y="{rc-2:.1f}" text-anchor="start" font-size="12" '
            f'fill="{C_AXIS}">{esc(sec["name"])}</text>'
        )
        evidence_label = f"{sec['items']} items, {sec['coverage'] * 100:.0f}% cov"
        p.append(
            f'<text x="24" y="{rc+12:.1f}" text-anchor="start" font-size="9" '
            f'fill="{C_SUB}">{esc(evidence_label)}</text>'
        )

        if enough_evidence(sec):
            # thin baseline track (the abstain row uses a dashed box instead, so
            # its withheld message stays legible)
            p.append(
                f'<line x1="{SX0:.1f}" y1="{rc:.1f}" x2="{SX1:.1f}" y2="{rc:.1f}" '
                f'stroke="{C_SCALE}" stroke-width="1"/>'
            )
            lo = clamp(sec["center"] - 1.96 * sec["se"], SEC_MIN, SEC_MAX)
            hi = clamp(sec["center"] + 1.96 * sec["se"], SEC_MIN, SEC_MAX)
            lx, hx = x_sec(lo), x_sec(hi)
            p.append(
                f'<rect x="{lx:.1f}" y="{rc-8:.1f}" width="{hx-lx:.1f}" height="16" '
                f'fill="{C_POINT}" fill-opacity="0.2"/>'
            )
            for ex in (lx, hx):
                p.append(
                    f'<line x1="{ex:.1f}" y1="{rc-8:.1f}" x2="{ex:.1f}" y2="{rc+8:.1f}" '
                    f'stroke="{C_POINT}" stroke-width="1"/>'
                )
            dx = x_sec(sec["center"])
            center_label = f"{sec['center']:.1f}"
            range_label = f"{lo:.1f}-{hi:.1f}"
            p.append(
                f'<circle cx="{dx:.1f}" cy="{rc:.1f}" r="4" fill="{C_POINT}" '
                f'stroke="{C_POINT_STROKE}" stroke-width="1"/>'
            )
            p.append(
                f'<text x="{dx:.1f}" y="{rc-11:.1f}" text-anchor="middle" font-size="11" '
                f'font-weight="bold" fill="{C_TITLE}">{esc(center_label)}</text>'
            )
            p.append(
                f'<text x="{hx+8:.1f}" y="{rc+3:.1f}" text-anchor="start" font-size="9" '
                f'fill="{C_SUB}">{esc(range_label)}</text>'
            )
        else:
            # ABSTAIN: withhold the number, show the give-up-rule message instead
            p.append(
                f'<rect x="{SX0:.1f}" y="{rc-8:.1f}" width="{SX1-SX0:.1f}" height="16" '
                f'fill="none" stroke="{C_WITHHELD}" stroke-width="1" stroke-dasharray="4,4"/>'
            )
            need = int(MIN_ITEMS)
            msg = (
                f"Not enough data yet -- need {need} items / {MIN_COVERAGE*100:.0f}% coverage "
                f"(have {sec['items']} / {sec['coverage']*100:.0f}%)"
            )
            p.append(
                f'<text x="{(SX0+SX1)/2:.1f}" y="{rc+3:.1f}" text-anchor="middle" '
                f'font-size="11" fill="{C_ABSTAIN}">{esc(msg)}</text>'
            )

    # ---- Section axis line + ticks (shared 118-132 scale) ----
    p.append(
        f'<line x1="{SX0:.1f}" y1="{axis_y:.1f}" x2="{SX1:.1f}" y2="{axis_y:.1f}" '
        f'stroke="{C_SCALE}" stroke-width="1.5"/>'
    )
    for val in (118, 120, 125, 130, 132):
        vx = x_sec(float(val))
        p.append(
            f'<line x1="{vx:.1f}" y1="{axis_y:.1f}" x2="{vx:.1f}" y2="{axis_y+5:.1f}" '
            f'stroke="{C_SCALE}" stroke-width="1"/>'
        )
        p.append(
            f'<text x="{vx:.1f}" y="{axis_y+17:.1f}" text-anchor="middle" font-size="10" '
            f'fill="{C_SUB}">{esc(str(val))}</text>'
        )

    # ---- Bottom captions: give-up rule + honesty + construction ----
    p.append(
        f'<text x="{W/2:.0f}" y="414" text-anchor="middle" font-size="11" '
        f'fill="{C_AXIS}">'
        f'{esc("Give-up rule: a section shows a range only with >= 20 items AND >= 60% coverage; CARS is below the line, so it abstains.")}'
        f'</text>'
    )
    p.append(
        f'<text x="{W/2:.0f}" y="434" text-anchor="middle" font-size="11" '
        f'fill="{C_TITLE}">'
        f'{esc("EXAMPLE / illustrative projection (synthetic learners) -- NOT a validated MCAT score.")}'
        f'</text>'
    )
    shown = " + ".join(
        f"{s['name']} {s['center']:.1f}" for s in SECTIONS if enough_evidence(s)
    )
    construction_label = (
        f"Construction: {shown} + CARS (withheld, ~120 floor) "
        f"= {TOTAL['center']:.1f} total"
    )
    p.append(
        f'<text x="{W/2:.0f}" y="454" text-anchor="middle" font-size="10" '
        f'fill="{C_SUB}">{esc(construction_label)}</text>'
    )
    p.append("</svg>")
    return "\n".join(p)


def main(argv: list[str]) -> int:
    # Honesty self-check: the four constructed section centers must sum to the
    # projected total we display, so the "range not a point" picture is internally
    # consistent (this is an EXAMPLE construction, not measured data).
    center_sum = sum(s["center"] for s in SECTIONS)
    if abs(center_sum - TOTAL["center"]) > 0.05:
        sys.stderr.write(
            f"ERROR: section centers sum to {center_sum:.2f}, "
            f"expected {TOTAL['center']:.2f}\n"
        )
        return 2

    svg = build_svg()
    xml.dom.minidom.parseString(svg)  # raises if the hand-built SVG is malformed
    SVG_OUT.write_text(svg + "\n", encoding="utf-8")

    spec = {
        "total": {
            "center": TOTAL["center"],
            "band_95": [TOTAL["lo"], TOTAL["hi"]],
            "scale": [TOTAL["min"], TOTAL["max"]],
            "source": "ablation ON arm (evals/ABLATION.md; synthetic learners)",
        },
        "sections": [
            {
                "name": s["name"],
                "center": s["center"],
                "band_95": (
                    [
                        round(clamp(s["center"] - 1.96 * s["se"], SEC_MIN, SEC_MAX), 1),
                        round(clamp(s["center"] + 1.96 * s["se"], SEC_MIN, SEC_MAX), 1),
                    ]
                    if enough_evidence(s)
                    else None
                ),
                "items": s["items"],
                "coverage": s["coverage"],
                "abstains": not enough_evidence(s),
            }
            for s in SECTIONS
        ],
        "give_up_rule": {"min_items": MIN_ITEMS, "min_coverage": MIN_COVERAGE},
        "guessing_floor": GUESS_FLOOR,
        "note": "EXAMPLE / illustrative -- synthetic learners; not a validated MCAT score.",
    }

    print("\n=== Readiness projection (EXAMPLE -- a range, not a point) ===")
    print(json.dumps(spec, indent=2))
    abstained = [s["name"] for s in SECTIONS if not enough_evidence(s)]
    print(
        f"\nsection centers sum to {center_sum:.1f} (matches projected total "
        f"{TOTAL['center']:.1f})"
    )
    print(f"abstaining section(s) (give-up rule): {', '.join(abstained) or 'none'}")
    print(f"wrote {SVG_OUT}")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
