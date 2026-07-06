# Copyright: Ankitects Pty Ltd and contributors
# License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html

"""Shared MCAT design system + the Readiness Dashboard home experience.

This module renders the premium "Readiness Dashboard" shown on the deck browser
home screen for the MCAT prep fork. It only *consumes* the existing concept
scheduler status (proto ``ConceptSchedulerStatusResponse``); it never mutates the
scoring/scheduler backend.

Everything is emitted as a self-contained ``<style> + markup + <script>`` block so
it applies without a web rebuild, matching the inline-injection pattern used in
``reviewer.py``. Design tokens live in :data:`DESIGN_TOKENS_CSS` and can be reused
by other Qt web surfaces for a cohesive look.
"""

from __future__ import annotations

import json
import math
import time
from typing import Any

# -- small unicode helpers (kept out of f-string braces for <3.12 safety) ---
_EN_DASH = "\u2013"
_EM_DASH = "\u2014"
_ARROW = "\u2192"
_CHECK = "\u2713"
_CROSS = "\u2715"
_MID = "\u00b7"

# Coverage below which section/score ranges are hidden (the give-up gate). Above
# this fraction the estimate is shown; below it the UI abstains rather than print
# a number. Mirrors Android's ``MIN_COVERAGE_TO_SHOW_SCORES`` for cross-platform
# parity. Backend is untouched: this only decides what the UI dares to show.
_MIN_COVERAGE_TO_SHOW = 0.60

# Shown when a number IS displayed but is still the model's baseline/prior: a
# projection exists (``has_projection``) yet the overall evidence gate isn't met
# (``enough_evidence`` false). Distinct from the abstain state, which shows NO
# number. Exact copy is shared with the spec + the reviewer sidebar.
_BASELINE_NOTE = (
    "Using baseline readiness " + _EM_DASH + " need 60% coverage and at least "
    "20 problems"
)

# -- Design system ----------------------------------------------------------
#
# One brand accent (indigo), a per-MCAT-section palette that stays recognisable
# across the graph/sidebar/dashboard, a type scale, spacing, radii and shadows.
# Exposed as CSS custom properties with light + dark values so any Qt web view
# can reuse the same language.

BRAND = "#6366f1"

# key -> (label, short label, accent colour, CSS var suffix).
SECTIONS: list[dict[str, str]] = [
    {"key": "Bio_Biochem", "label": "Bio/Biochem", "color": "#4f74d6", "var": "bio"},
    {"key": "Chem_Phys", "label": "Chem/Phys", "color": "#e0555f", "var": "chem"},
    {"key": "Psych_Soc", "label": "Psych/Soc", "color": "#2fa96a", "var": "psych"},
    {"key": "CARS", "label": "CARS", "color": "#dda032", "var": "cars"},
]
_SEC_VAR = {s["key"]: s["var"] for s in SECTIONS}

# Ring geometry shared by every donut gauge (one radius => one keyframe).
_RING_R = 52.0
_RING_C = 2 * math.pi * _RING_R  # circumference

DESIGN_TOKENS_CSS = """
:root {
    --mc-brand: #6366f1;
    --mc-brand-strong: #4f46e5;
    --mc-brand-soft: rgba(99, 102, 241, 0.12);

    --mc-sec-bio: #4f74d6;
    --mc-sec-chem: #e0555f;
    --mc-sec-psych: #2fa96a;
    --mc-sec-cars: #dda032;

    --mc-good: #21a866;
    --mc-warn: #e0a020;
    --mc-bad: #e0555f;

    --mc-app-bg: #f4f6fb;
    --mc-surface: #ffffff;
    --mc-surface-2: #f7f8fc;
    --mc-border: #e7eaf3;
    --mc-border-strong: #d7dceb;
    --mc-text: #1b2233;
    --mc-text-muted: #5c6780;
    --mc-text-faint: #8b95ac;
    --mc-track: #eceef6;

    --mc-shadow-sm: 0 1px 2px rgba(20, 27, 48, 0.06), 0 1px 3px rgba(20, 27, 48, 0.08);
    --mc-shadow-md: 0 6px 18px rgba(20, 27, 48, 0.08);
    --mc-shadow-lg: 0 18px 44px rgba(20, 27, 48, 0.14);

    --mc-r-sm: 10px;
    --mc-r-md: 14px;
    --mc-r-lg: 20px;
    --mc-r-pill: 999px;

    --mc-font: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica,
        Arial, sans-serif;

    /* Shared "emphasized" easing + motion durations (UI-SPEC Motion). */
    --mc-ease: cubic-bezier(0.2, 0, 0, 1);
    --mc-dur-press: 120ms;
    --mc-dur-move: 240ms;
    --mc-dur-sweep: 620ms;
}

.nightMode {
    --mc-brand: #8b8ff8;
    --mc-brand-strong: #a5a8fb;
    --mc-brand-soft: rgba(139, 143, 248, 0.16);

    --mc-sec-bio: #6f92ec;
    --mc-sec-chem: #f0767f;
    --mc-sec-psych: #4cc389;
    --mc-sec-cars: #eab857;

    --mc-good: #43c98a;
    --mc-warn: #eab857;
    --mc-bad: #f0767f;

    --mc-app-bg: #0d1017;
    --mc-surface: #171b26;
    --mc-surface-2: #1e2331;
    --mc-border: #2a3040;
    --mc-border-strong: #353c50;
    --mc-text: #e9edf7;
    --mc-text-muted: #9aa5bd;
    --mc-text-faint: #6f7a93;
    --mc-track: #262c3b;

    --mc-shadow-sm: 0 1px 2px rgba(0, 0, 0, 0.4);
    --mc-shadow-md: 0 8px 22px rgba(0, 0, 0, 0.45);
    --mc-shadow-lg: 0 22px 50px rgba(0, 0, 0, 0.55);
}
"""


def _clamp01(x: float) -> float:
    return max(0.0, min(1.0, x))


def _frac(value: float, lo: float, hi: float) -> float:
    if hi <= lo:
        return 0.0
    return _clamp01((value - lo) / (hi - lo))


def _score_band(total: float) -> str:
    """A friendly readiness label for a 472-528 total."""
    if total >= 520:
        return "Elite"
    if total >= 515:
        return "Highly competitive"
    if total >= 511:
        return "Competitive"
    if total >= 506:
        return "On track"
    if total >= 500:
        return "Building"
    return "Foundation"


def _prob_color(prob: float) -> str:
    if prob >= 0.66:
        return "var(--mc-good)"
    if prob >= 0.4:
        return "var(--mc-warn)"
    return "var(--mc-bad)"


# -- Honesty model (the three ranged scores) --------------------------------
#
# These mirror the Android app's ``McatHonesty.kt`` 1:1 so both platforms report
# the same numbers, wording and abstain state. Everything is derived from fields
# already present in the status payload; the scoring/scheduler backend is never
# touched.


def _clamp(x: float, lo: float, hi: float) -> float:
    return max(lo, min(hi, x))


def _round_i(x: float) -> int:
    """Round half-up to match Android's ``roundToInt`` (all values here are >= 0)."""
    return int(math.floor(x + 0.5))


def _how_sure(coverage: float) -> str:
    """A plain-language confidence word driven purely by how much of the exam is
    covered, so a thin evidence base can never read as "high"."""
    if coverage < 0.4:
        return "Low confidence"
    if coverage < 0.7:
        return "Building confidence"
    return "Strong confidence"


def _abstain_message(reviews_needed: int, coverage_pct: int) -> str:
    """The give-up wording, identical on desktop + Android (UI-SPEC "Honesty")."""
    return (
        f"Not enough data yet {_EM_DASH} need {reviews_needed} reviews "
        f"/ {coverage_pct}% coverage"
    )


def _overall_coverage(sections: list[dict[str, Any]]) -> float:
    """Mean section coverage (0..1), used for "% of exam covered" and how-sure."""
    if not sections:
        return 0.0
    return sum(float(s.get("coverage") or 0.0) for s in sections) / len(sections)


def _reviews_needed(status: dict[str, Any]) -> int:
    """Reviews still needed to clear the overall evidence bar (0 when met/unknown)."""
    required = int(status.get("evidenceRequiredCards") or 0)
    seen = int(status.get("evidenceSeenCards") or 0)
    return max(0, required - seen)


def _baseline_note(sections: list[dict[str, Any]], coverage_pct: int) -> str:
    """Baseline-readiness copy, enriched with live progress: what the gate needs
    (``60% coverage`` + N questions/section) *and* how far the learner already is
    (questions answered, coverage so far). Falls back to the static note when the
    payload carries no section data."""
    if not sections:
        return _BASELINE_NOTE
    answered = sum(int(s.get("answeredItems") or 0) for s in sections)
    required = (
        max((int(s.get("requiredItems") or 0) for s in sections), default=0) or 20
    )
    need_cov = _round_i(_MIN_COVERAGE_TO_SHOW * 100)
    ready = sum(1 for s in sections if bool(s.get("enoughEvidence")))
    return (
        "Using baseline readiness "
        + _EM_DASH
        + f" you've answered {answered} questions so far ({coverage_pct}% covered, "
        + f"{ready}/{len(sections)} sections ready). "
        + f"Need {need_cov}% coverage and at least {required} questions per section."
    )


def _top_reason(sections: list[dict[str, Any]], value_key: str) -> str:
    """A one-line "top reason": the strongest section by ``value_key`` and the one
    whose thin coverage is holding the estimate back."""
    if not sections:
        return "Answer a few cards to see what's driving this"
    strongest = max(sections, key=lambda s: float(s.get(value_key) or 0.0))
    thinnest = min(sections, key=lambda s: float(s.get("coverage") or 0.0))
    return (
        f"Driven by {strongest.get('section', 'a section')} {_MID} "
        f"thinnest coverage in {thinnest.get('section', 'a section')}"
    )


def _whats_missing(
    sections: list[dict[str, Any]], threshold: float = _MIN_COVERAGE_TO_SHOW
) -> str:
    """High-value sections still under the coverage line, worst first."""
    threshold_pct = _round_i(threshold * 100)
    low = sorted(
        (s for s in sections if float(s.get("coverage") or 0.0) < threshold),
        key=lambda s: float(s.get("coverage") or 0.0),
    )
    if not low:
        if not sections:
            return "No sections scored yet"
        return f"All sections above {threshold_pct}% coverage"
    listing = ", ".join(
        f"{s.get('section', 'Section')} "
        f"{_round_i(float(s.get('coverage') or 0.0) * 100)}%"
        for s in low
    )
    return f"Below {threshold_pct}% coverage: {listing}"


def _score_tiles(status: dict[str, Any]) -> list[dict[str, Any]]:
    """The three separately-reported scores (Memory / Performance / Readiness),
    each carrying its honesty fields. Mirrors Android's ``buildScoreTiles``.

    - Memory: avg recall of studied cards; range = spread of per-section recall.
    - Performance: demonstrated IRT total = sum of the sections' performance
      centers, with a combined confidence band (sqrt of summed variances).
    - Readiness: the backend's projected MCAT total (472-528) with its band.
    """
    sections = status.get("sectionScores") or []
    coverage = _overall_coverage(sections)
    coverage_pct = _round_i(coverage * 100)
    how_sure = _how_sure(coverage)
    whats_missing = _whats_missing(sections)
    evidence_enough = bool(status.get("evidenceEnough"))
    abstain = _abstain_message(
        _reviews_needed(status), _round_i(_MIN_COVERAGE_TO_SHOW * 100)
    )

    mem = float(status.get("overallMemory") or 0.0)
    section_mems = [
        float(s.get("sectionMemory") or 0.0)
        for s in sections
        if s.get("sectionHasMemory")
    ]
    if len(section_mems) >= 2:
        mem_range = (
            f"{_round_i(min(section_mems) * 100)}{_EN_DASH}"
            f"{_round_i(max(section_mems) * 100)}%"
        )
    else:
        mem_range = _EN_DASH
    memory = dict(
        kind="memory",
        title="Memory",
        subtitle="recall right now",
        available=bool(status.get("hasMemory")),
        estimate=f"{_round_i(mem * 100)}%",
        range=mem_range,
        fraction=_clamp01(mem),
        coverage_pct=coverage_pct,
        how_sure=how_sure,
        top_reason=_top_reason(sections, "sectionMemory"),
        whats_missing=whats_missing,
        abstain=abstain,
        baseline=False,
        scale_caption="avg recall of studied cards",
    )

    perf_center = sum(float(s.get("performanceCenter") or 0.0) for s in sections)
    perf_se = math.sqrt(
        sum(float(s.get("performanceStandardError") or 0.0) ** 2 for s in sections)
    )
    perf_lo = _clamp(perf_center - 1.96 * perf_se, 472, 528)
    perf_hi = _clamp(perf_center + 1.96 * perf_se, 472, 528)
    performance = dict(
        kind="performance",
        title="Performance",
        subtitle="if you keep it up",
        available=any(bool(s.get("enoughEvidence")) for s in sections),
        estimate=str(_round_i(perf_center)),
        range=f"{_round_i(perf_lo)}{_EN_DASH}{_round_i(perf_hi)}",
        fraction=_frac(perf_center, 472, 528),
        coverage_pct=coverage_pct,
        how_sure=how_sure,
        top_reason=_top_reason(sections, "performanceCenter"),
        whats_missing=whats_missing,
        abstain=abstain,
        baseline=False,
        scale_caption=f"your level if you keep it up {_MID} 472{_EN_DASH}528",
    )

    total = float(status.get("projectedTotal") or 0.0)
    r_lo = (float(status.get("projectedTotalLower") or 0.0)) or total
    r_hi = (float(status.get("projectedTotalUpper") or 0.0)) or total
    readiness = dict(
        kind="readiness",
        title="Readiness",
        subtitle="on your exam day",
        available=bool(status.get("hasProjection")) and total > 0,
        estimate=str(_round_i(total)),
        range=f"{_round_i(r_lo)}{_EN_DASH}{_round_i(r_hi)}",
        fraction=_frac(total, 472, 528),
        coverage_pct=coverage_pct,
        how_sure=how_sure,
        top_reason=_top_reason(sections, "readinessCenter"),
        whats_missing=whats_missing,
        abstain=abstain,
        baseline=(
            bool(status.get("hasProjection")) and total > 0 and not evidence_enough
        ),
        baseline_note=_baseline_note(sections, coverage_pct),
        scale_caption=f"projected to your exam day {_MID} 472{_EN_DASH}528",
        compare=(
            f"If tested today {_round_i(perf_center)} {_ARROW} "
            f"projected exam day {_round_i(total)} {_EM_DASH} the gap is what "
            "expected forgetting costs; keep reviews up to close it"
            if perf_center > 0 and total > 0
            else ""
        ),
    )

    return [memory, performance, readiness]


def _ring_svg(
    center_frac: float,
    color: str,
    *,
    band: tuple[float, float] | None = None,
    delay: float = 0.0,
    track_width: float = 11.0,
    value_width: float = 11.0,
    grad: bool = False,
) -> str:
    """A donut gauge SVG. ``center_frac`` is the solid fill (best estimate);
    ``band`` optionally draws a fainter arc for the confidence range. Renders the
    final value inline (works without JS) and eases in via a CSS keyframe."""
    c = _RING_C
    arc = _clamp01(center_frac) * c
    offset = c - arc
    defs = ""
    if grad:
        defs = (
            '<defs><linearGradient id="mcHeroGrad" x1="0" y1="0" x2="1" y2="1">'
            '<stop offset="0" stop-color="var(--mc-brand)"></stop>'
            '<stop offset="1" stop-color="var(--mc-good)"></stop>'
            "</linearGradient></defs>"
        )
    band_svg = ""
    if band is not None:
        f0 = _clamp01(band[0])
        f1 = _clamp01(band[1])
        if f1 > f0:
            band_len = (f1 - f0) * c
            band_off = -(f0 * c)
            band_svg = (
                '<circle class="mc-ring-band" cx="60" cy="60" r="52" fill="none" '
                f'stroke="{color}" stroke-width="{value_width}" stroke-linecap="round" '
                f'stroke-dasharray="{band_len:.2f} {c:.2f}" '
                f'stroke-dashoffset="{band_off:.2f}" '
                'transform="rotate(-90 60 60)"></circle>'
            )
    style = f"animation-delay:{delay:.2f}s"
    return (
        '<svg class="mc-ring" viewBox="0 0 120 120" aria-hidden="true">'
        f"{defs}"
        '<circle cx="60" cy="60" r="52" fill="none" stroke="var(--mc-track)" '
        f'stroke-width="{track_width}"></circle>'
        f"{band_svg}"
        '<circle class="mc-ring-val" cx="60" cy="60" r="52" fill="none" '
        f'stroke="{color}" stroke-width="{value_width}" stroke-linecap="round" '
        f'stroke-dasharray="{c:.2f}" stroke-dashoffset="{offset:.2f}" '
        f'style="{style}" transform="rotate(-90 60 60)"></circle>'
        "</svg>"
    )


def _bar(frac: float, color: str, *, delay: float = 0.0) -> str:
    pct = round(_clamp01(frac) * 100)
    style = f"width:{pct}%;background:{color};transition-delay:{delay:.2f}s"
    return f'<div class="mc-bar"><div class="mc-bar-fill" style="{style}"></div></div>'


def _section_by_key(scores: list[dict[str, Any]]) -> dict[str, dict[str, Any]]:
    label_to_key = {s["label"]: s["key"] for s in SECTIONS}
    out: dict[str, dict[str, Any]] = {}
    for score in scores:
        key = label_to_key.get(score.get("section", ""))
        if key:
            out[key] = score
    return out


def _hero(status: dict[str, Any]) -> str:
    has_projection = bool(status.get("hasProjection"))
    total = float(status.get("projectedTotal") or 0.0)
    lo = float(status.get("projectedTotalLower") or total)
    hi = float(status.get("projectedTotalUpper") or total)
    # Baseline: a number is shown but it's still the prior (projection exists yet
    # the evidence gate isn't met). Labelled explicitly; distinct from abstain.
    is_baseline = (
        has_projection and total > 0 and not bool(status.get("evidenceEnough"))
    )
    baseline_note = (
        f'<div class="mc-baseline">{_BASELINE_NOTE}</div>' if is_baseline else ""
    )
    # How-sure chip + "% of exam covered" beneath the range bar, mirroring
    # Android's ``ScoreHeroCard``. Shown only for an evidence-backed projection;
    # while the number is still the prior the baseline banner takes its place.
    howsure_html = ""
    if has_projection and total > 0 and not is_baseline:
        hero_coverage = _overall_coverage(status.get("sectionScores") or [])
        howsure_html = (
            '<div class="mc-hero-howsure">'
            f'<span class="mc-howsure">{_how_sure(hero_coverage)}</span>'
            '<span class="mc-hero-covered">'
            f"{_round_i(hero_coverage * 100)}% of exam covered</span>"
            "</div>"
        )

    if has_projection and total > 0:
        f_center = _frac(total, 472, 528)
        band = (_frac(lo, 472, 528), _frac(hi, 472, 528))
        ring = _ring_svg(
            f_center,
            "url(#mcHeroGrad)",
            band=band,
            track_width=13,
            value_width=13,
            grad=True,
        )
        band_label = _score_band(total)
        total_i = round(total)
        center = (
            '<div class="mc-hero-num">'
            f'<span class="mc-hero-score" data-count="{total_i}">{total_i}</span>'
            '<span class="mc-hero-cap">projected total</span>'
            f'<span class="mc-hero-band">{band_label}</span>'
            "</div>"
        )
        band_left = _frac(lo, 472, 528) * 100
        band_right = (1 - _frac(hi, 472, 528)) * 100
        marker_left = _frac(total, 472, 528) * 100
        likely = f"likely <b>{round(lo)}{_EN_DASH}{round(hi)}</b>"
        rangebar = (
            '<div class="mc-hero-range">'
            '<div class="mc-scale">'
            f'<div class="mc-scale-band" style="left:{band_left:.1f}%;right:{band_right:.1f}%"></div>'
            f'<div class="mc-scale-marker" style="left:{marker_left:.1f}%"></div>'
            "</div>"
            '<div class="mc-scale-ends"><span>472</span>'
            f"<span>{likely}</span><span>528</span></div>"
            "</div>"
        )
    else:
        ring = _ring_svg(0.0, "var(--mc-brand)", track_width=13, value_width=13)
        dashes = _EN_DASH * 3
        center = (
            '<div class="mc-hero-num">'
            f'<span class="mc-hero-score mc-empty">{dashes}</span>'
            '<span class="mc-hero-cap">projected total</span>'
            "</div>"
        )
        need = _reviews_needed(status)
        cov_pct = _round_i(_MIN_COVERAGE_TO_SHOW * 100)
        seen = int(status.get("evidenceSeenCards") or 0)
        required = int(status.get("evidenceRequiredCards") or 0)
        progress = (
            _bar(_clamp01(seen / required), "var(--mc-brand)") if required > 0 else ""
        )
        rangebar = (
            '<div class="mc-hero-range mc-hero-abstain">'
            f'<div class="mc-abstain-line">{_abstain_message(need, cov_pct)}</div>'
            f"{progress}"
            '<div class="mc-hero-hint">'
            "Answer a few questions and your projected MCAT score "
            f"(472{_EN_DASH}528) will appear here with a confidence range."
            "</div>"
            "</div>"
        )

    return (
        '<div class="mc-hero-card">'
        '<div class="mc-hero-ring">'
        f"{ring}"
        f'<div class="mc-ring-center">{center}</div>'
        "</div>"
        '<div class="mc-hero-body">'
        '<div class="mc-eyebrow">MCAT readiness</div>'
        '<div class="mc-hero-title">Your projected score</div>'
        f"{rangebar}"
        f"{howsure_html}"
        f"{baseline_note}"
        '<button class="mc-cta" data-cmd="mcatStudy">Continue studying '
        f'<span class="mc-cta-arrow">{_ARROW}</span></button>'
        "</div>"
        "</div>"
    )


def _exam_card(status: dict[str, Any]) -> str:
    exam_ts = int(status.get("examTimestamp") or 0)
    if exam_ts > 0:
        now = time.time()
        days = max(0, math.ceil((exam_ts - now) / 86400))
        try:
            date_str = time.strftime("%a, %b %-d, %Y", time.localtime(exam_ts))
        except ValueError:
            date_str = time.strftime("%a, %b %d, %Y", time.localtime(exam_ts))
        weeks = days / 7
        sub = f"{weeks:.0f} weeks out" if days >= 14 else "final stretch"
        body = (
            '<div class="mc-stat-big">'
            f'<span class="mc-stat-num">{days}</span>'
            '<span class="mc-stat-unit">days</span></div>'
            f'<div class="mc-stat-sub">{date_str}</div>'
            f'<div class="mc-stat-sub2">{sub}</div>'
            '<button class="mc-chip" data-cmd="mcatPlanOpen">Edit date</button>'
        )
    else:
        body = (
            '<div class="mc-stat-empty">Set your exam date to project memory and '
            "retention to test day.</div>"
            '<button class="mc-chip mc-chip-accent" data-cmd="mcatPlanOpen">'
            "Set exam date</button>"
        )
    return (
        '<div class="mc-card mc-mini">'
        '<div class="mc-mini-head"><span class="mc-mini-ico">\U0001f4c5</span>'
        "Test day</div>"
        f"{body}"
        "</div>"
    )


def _target_card(status: dict[str, Any]) -> str:
    has_target = bool(status.get("hasTarget"))
    target = int(status.get("targetTotalScore") or 0)
    prob = float(status.get("probabilityHitTarget") or 0.0)
    if has_target and target > 0:
        color = _prob_color(prob)
        ring = _ring_svg(prob, color, track_width=9, value_width=9)
        pct = round(prob * 100)
        body = (
            '<div class="mc-mini-ring">'
            f"{ring}"
            f'<div class="mc-ring-center-sm"><span class="mc-mini-pct">{pct}%</span></div>'
            "</div>"
            f'<div class="mc-stat-sub">chance of hitting <b>{target}</b> by test day</div>'
            '<button class="mc-chip" data-cmd="mcatPlanOpen">Edit target</button>'
        )
    else:
        body = (
            '<div class="mc-stat-empty">Set a target score to track your odds of '
            "hitting it by test day.</div>"
            '<button class="mc-chip mc-chip-accent" data-cmd="mcatPlanOpen">'
            "Set target score</button>"
        )
    return (
        '<div class="mc-card mc-mini">'
        '<div class="mc-mini-head"><span class="mc-mini-ico">\U0001f3af</span>'
        "Target</div>"
        f"{body}"
        "</div>"
    )


# Accent per score, matching Android's ``scoreAccent``: memory = blue, performance
# = amber, readiness = brand indigo.
_SCORE_ACCENT = {
    "memory": "var(--mc-sec-bio)",
    "performance": "var(--mc-sec-cars)",
    "readiness": "var(--mc-brand)",
}


def _score_tile_html(tile: dict[str, Any], index: int) -> str:
    """A compact, tappable score tile: title, estimate + range (or the abstain
    placeholder) and a fraction bar. Expands its detail panel via JS."""
    acc = _SCORE_ACCENT[tile["kind"]]
    delay = 0.05 * index
    if tile["available"]:
        est = f'<span class="mc-score-est">{tile["estimate"]}</span>'
        rng = f'<span class="mc-score-range">{tile["range"]}</span>'
        bar = _bar(tile["fraction"], acc, delay=delay + 0.08)
    else:
        dots = "\u2022\u2022\u2022"
        est = f'<span class="mc-score-est mc-empty">{dots}</span>'
        rng = '<span class="mc-score-range">building</span>'
        bar = _bar(0.0, acc, delay=delay + 0.08)
    tag = (
        '<span class="mc-score-tag">baseline</span>'
        if tile["available"] and tile.get("baseline")
        else ""
    )
    return (
        '<button class="mc-score-tile mc-enter" type="button" '
        f'data-score="{tile["kind"]}" style="--acc:{acc};--enter-delay:{delay:.2f}s" '
        'aria-expanded="false">'
        f'<span class="mc-score-name">{tile["title"]}</span>'
        f'<span class="mc-score-sub" style="display:block;font-size:0.64rem;'
        f"font-weight:500;opacity:0.6;margin:0.05rem 0 0.12rem;"
        f'letter-spacing:0.01em">{tile.get("subtitle", "")}</span>'
        f"{est}{rng}{tag}{bar}"
        "</button>"
    )


def _score_detail_html(tile: dict[str, Any]) -> str:
    """The expandable honesty panel for a score: estimate, range, % covered,
    how-sure and last-updated (or the abstain phrase), plus top reason and
    what's-missing. Fields mirror the Android ``ScoreDetailCard`` exactly."""
    acc = _SCORE_ACCENT[tile["kind"]]
    if tile["available"]:
        body = (
            '<div class="mc-kv"><span>Estimate</span>'
            f"<b>{tile['estimate']} {_MID} {tile['scale_caption']}</b></div>"
            f'<div class="mc-kv"><span>Range</span><b>{tile["range"]}</b></div>'
            f'<div class="mc-kv"><span>Covered</span><b>{tile["coverage_pct"]}%</b></div>'
            '<div class="mc-kv"><span>How sure</span>'
            f'<span class="mc-howsure">{tile["how_sure"]}</span></div>'
        )
    else:
        body = (
            f'<div class="mc-score-abstain">{tile["abstain"]}</div>'
            '<div class="mc-kv"><span>Covered</span>'
            f"<b>{tile['coverage_pct']}%</b></div>"
        )
    baseline_html = (
        f'<div class="mc-baseline">{tile.get("baseline_note") or _BASELINE_NOTE}</div>'
        if tile["available"] and tile.get("baseline")
        else ""
    )
    compare_html = (
        '<div class="mc-compare" style="margin-top:0.55rem;font-size:0.76rem;'
        "font-weight:600;line-height:1.35;border-left:3px solid var(--acc);"
        "padding:0.4rem 0.6rem;border-radius:var(--mc-r-sm);"
        'background:color-mix(in srgb, var(--acc) 10%, transparent)">'
        f"{tile['compare']}</div>"
        if tile["available"] and tile.get("compare")
        else ""
    )
    return (
        f'<div class="mc-score-detail" data-detail="{tile["kind"]}" '
        f'style="--acc:{acc}">'
        '<div class="mc-score-detail-head">'
        '<span class="mc-score-dot"></span>'
        f'<span class="mc-score-detail-title">{tile["title"]} detail</span>'
        '<span class="mc-updated" data-updated>Updated just now</span>'
        "</div>"
        f"{body}"
        f"{compare_html}"
        f"{baseline_html}"
        '<div class="mc-note"><span>Top reason</span>'
        f"<p>{tile['top_reason']}</p></div>"
        '<div class="mc-note"><span>What\'s missing</span>'
        f"<p>{tile['whats_missing']}</p></div>"
        "</div>"
    )


def _three_scores(status: dict[str, Any]) -> str:
    """The "three scores" block: Memory / Performance / Readiness, each reported
    separately (never blended) with its honesty fields and an abstain state."""
    tiles = _score_tiles(status)
    tiles_html = "".join(_score_tile_html(t, i) for i, t in enumerate(tiles))
    details_html = "".join(_score_detail_html(t) for t in tiles)
    return (
        '<div class="mc-section-title">Your three scores</div>'
        '<div class="mc-scores-sub">Performance is your level if you keep it up '
        f"{_EM_DASH} readiness projects it to your exam day, after expected "
        "forgetting</div>"
        '<div class="mc-scores">'
        f'<div class="mc-score-tiles">{tiles_html}</div>'
        f"{details_html}"
        "</div>"
    )


def _section_card(
    sec: dict[str, str],
    score: dict[str, Any] | None,
    index: int,
    active: bool = False,
) -> str:
    color = f"var(--mc-sec-{_SEC_VAR[sec['key']]})"
    delay = 0.12 * index
    if score:
        center = float(score.get("readinessCenter") or 0.0)
        lo = float(score.get("readinessLower") or center)
        hi = float(score.get("readinessUpper") or center)
        coverage = float(score.get("coverage") or 0.0)
        has_mem = bool(score.get("sectionHasMemory"))
        mem = float(score.get("sectionMemory") or 0.0)
        enough = bool(score.get("enoughEvidence"))
        if center > 0:
            f_center = _frac(center, 118, 132)
            band = (_frac(lo, 118, 132), _frac(hi, 118, 132))
            ring = _ring_svg(f_center, color, band=band, delay=delay)
            ring_center = (
                '<div class="mc-ring-center-sm">'
                f'<span class="mc-sec-score">{round(center)}</span>'
                f'<span class="mc-sec-range">{round(lo)}{_EN_DASH}{round(hi)}</span>'
                "</div>"
            )
        else:
            ring = _ring_svg(0.0, color, delay=delay)
            twodash = _EN_DASH * 2
            ring_center = (
                '<div class="mc-ring-center-sm">'
                f'<span class="mc-sec-score mc-empty">{twodash}</span>'
                '<span class="mc-sec-range">118' + _EN_DASH + "132</span></div>"
            )
        evidence_note = (
            "" if enough else '<span class="mc-sec-tag">building evidence</span>'
        )
        cov_pct = round(coverage * 100)
        if enough:
            evidence_detail = ""
        else:
            answered = int(score.get("answeredItems") or 0)
            required = int(score.get("requiredItems") or 0) or 20
            need_cov = _round_i(_MIN_COVERAGE_TO_SHOW * 100)
            evidence_detail = (
                '<div class="mc-sec-evidence" style="font-size:0.64rem;opacity:0.72;'
                'margin-top:0.3rem;line-height:1.35">'
                f"{cov_pct}%/{need_cov}% coverage, {answered}/{required} solved"
                "<br>"
                f"need {need_cov}% coverage and {required} solved for accurate scoring"
                "</div>"
            )
        mem_txt = (str(round(mem * 100)) + "%") if has_mem else _EN_DASH
        metrics = (
            '<div class="mc-sec-metric"><span>Coverage</span>'
            f"<b>{cov_pct}%</b></div>"
            f"{_bar(coverage, color, delay=delay + 0.1)}"
            '<div class="mc-sec-metric"><span>Memory</span>'
            f"<b>{mem_txt}</b></div>"
            f"{_bar(mem if has_mem else 0.0, color, delay=delay + 0.16)}"
        )
    else:
        ring = _ring_svg(0.0, color)
        twodash = _EN_DASH * 2
        ring_center = (
            '<div class="mc-ring-center-sm">'
            f'<span class="mc-sec-score mc-empty">{twodash}</span></div>'
        )
        evidence_note = ""
        evidence_detail = ""
        metrics = '<div class="mc-sec-metric"><span>No data yet</span></div>'
    # Tappable "study this section" affordance; reflects the active focus.
    if active:
        cta = f'<div class="mc-sec-cta mc-sec-cta-on">{_CHECK} Studying this section</div>'
    else:
        cta = f'<div class="mc-sec-cta">Study this section {_ARROW}</div>'
    active_cls = " mc-sec-active" if active else ""
    return (
        f'<div class="mc-card mc-sec{active_cls}" style="--sec:{color}" '
        f'role="button" tabindex="0" data-cmd="mcatSection:{index}" '
        f'aria-pressed="{"true" if active else "false"}" '
        f'title="Study only {sec["label"]} questions">'
        '<div class="mc-sec-head"><span class="mc-sec-dot"></span>'
        f'<span class="mc-sec-name">{sec["label"]}</span>{evidence_note}</div>'
        '<div class="mc-sec-ring">'
        f"{ring}{ring_center}</div>"
        f'<div class="mc-sec-rlabel">readiness {_MID} scale 118{_EN_DASH}132</div>'
        f'<div class="mc-sec-metrics">{metrics}</div>'
        f"{evidence_detail}"
        f"{cta}"
        "</div>"
    )


def _section_cards(status: dict[str, Any]) -> str:
    by_key = _section_by_key(status.get("sectionScores") or [])
    has_selected = bool(status.get("hasSelectedSection"))
    selected_label = status.get("selectedSection") or ""
    cards = [
        _section_card(
            sec,
            by_key.get(sec["key"]),
            i,
            active=has_selected and sec["label"] == selected_label,
        )
        for i, sec in enumerate(SECTIONS)
    ]
    # Clear / back-to-all affordance: shown as an active banner when a section is
    # the focus, and as a guiding hint otherwise. "Study all sections" clears it.
    if has_selected:
        control = (
            '<div class="mc-sec-control mc-sec-control-on">'
            f'<span class="mc-sec-control-label">Studying <b>{selected_label}</b> only'
            "</span>"
            '<button class="mc-chip mc-chip-accent" data-cmd="mcatSectionClear">'
            "Study all sections</button>"
            "</div>"
        )
    else:
        control = (
            '<div class="mc-sec-control">'
            '<span class="mc-sec-control-label">Tap a section to study just its '
            "questions</span></div>"
        )
    return control + '<div class="mc-sec-grid">' + "".join(cards) + "</div>"


def _knowledge_map(status: dict[str, Any]) -> str:
    nodes = status.get("nodes") or []
    node_count = len(nodes)
    ready = sum(1 for n in nodes if n.get("fringe") == "outer")
    mastered = sum(1 for n in nodes if n.get("fringe") == "inner")
    legend = "".join(
        f'<span><i style="background:{s["color"]}"></i>{s["label"]}</span>'
        for s in SECTIONS
    )
    return (
        '<div class="mc-card mc-map-card">'
        '<div class="mc-map-head">'
        '<div><div class="mc-eyebrow">Knowledge map</div>'
        '<div class="mc-map-title" data-map-title>Your skill tree</div></div>'
        '<div class="mc-map-stats">'
        f"<span><b>{node_count}</b> concepts</span>"
        f'<span class="mc-dot-ok"><b>{mastered}</b> mastered</span>'
        f'<span class="mc-dot-ready"><b>{ready}</b> ready to start</span>'
        "</div>"
        '<div class="mc-map-controls" data-map-controls></div>'
        "</div>"
        '<div class="mc-map-legend">'
        f"{legend}"
        '<span class="mc-lg-ready"><i></i>ready to start</span>'
        '<span class="mc-lg-lock"><i></i>locked</span>'
        f'<span class="mc-lg-hint">fill = mastery {_MID} size = importance {_MID} '
        "click to explore</span>"
        "</div>"
        '<div class="mc-map-scroll">'
        '<div class="mc-map-skeleton mc-skeleton"></div>'
        '<div class="mc-map-canvas"></div></div>'
        '<div class="mc-map-detail" hidden></div>'
        "</div>"
    )


def _plan_editor(status: dict[str, Any]) -> str:
    exam_ts = int(status.get("examTimestamp") or 0)
    target = int(status.get("targetTotalScore") or 0)
    date_default = time.strftime("%Y-%m-%d", time.localtime(exam_ts)) if exam_ts else ""
    target_default = str(target) if target else ""
    return (
        '<div class="mc-plan">'
        '<div class="mc-plan-head">Study plan'
        f'<button class="mc-plan-x" data-cmd="mcatPlanClose">{_CROSS}</button></div>'
        '<label class="mc-plan-field"><span>Exam date</span>'
        f'<input type="date" value="{date_default}"></label>'
        f'<label class="mc-plan-field"><span>Target score (472{_EN_DASH}528)</span>'
        '<input type="number" min="472" max="528" step="1" placeholder="e.g. 512" '
        f'value="{target_default}"></label>'
        '<div class="mc-plan-actions">'
        '<button class="mc-chip" data-plan="clear">Clear</button>'
        '<button class="mc-cta mc-cta-sm" data-plan="save">Save plan</button>'
        "</div></div>"
    )


def build_dashboard_html(status: dict[str, Any]) -> str:
    """Return the full inline dashboard block (style + markup + script)."""
    payload = json.dumps(
        {"nodes": status.get("nodes") or [], "edges": status.get("edges") or []}
    )
    return (
        "<style>"
        + DESIGN_TOKENS_CSS
        + _DASH_CSS
        + "</style>"
        + '<div class="mcat-dash">'
        + '<div class="mc-topline">'
        + '<div class="mc-brandmark"><span class="mc-logo">M</span>'
        + '<div><div class="mc-brandname">MCAT&nbsp;Prep</div>'
        + '<div class="mc-brandsub">Readiness dashboard</div></div></div>'
        + "</div>"
        + _hero(status)
        + '<div class="mc-mini-row">'
        + _exam_card(status)
        + _target_card(status)
        + "</div>"
        + _plan_editor(status)
        + _three_scores(status)
        + '<div class="mc-section-title">Section readiness</div>'
        + _section_cards(status)
        + _knowledge_map(status)
        + "</div>"
        + "<script>window.__mcatDash = "
        + payload
        + ";"
        + _DASH_JS
        + "</script>"
    )


# The knowledge-map renderer + score count-up + plan editor. Reads window.__mcatDash.
_DASH_JS = r"""
(() => {
    const data = window.__mcatDash || {};
    const root = document.querySelector(".mcat-dash");
    if (!root) { return; }

    const togglePlan = (open) => {
        const panel = root.querySelector(".mc-plan");
        if (!panel) { return; }
        panel.classList.toggle("open", open);
        if (open) {
            const d = panel.querySelector("input[type=date]");
            if (d) { d.focus(); }
            panel.scrollIntoView({ behavior: "smooth", block: "nearest" });
        }
    };

    // command routing: any [data-cmd] element -> pycmd
    root.addEventListener("click", (e) => {
        const el = e.target.closest("[data-cmd]");
        if (!el) { return; }
        const cmd = el.getAttribute("data-cmd");
        if (cmd === "mcatPlanOpen") { togglePlan(true); return; }
        if (cmd === "mcatPlanClose") { togglePlan(false); return; }
        if (typeof pycmd === "function") { pycmd(cmd); }
    });

    // Respect the OS/browser reduced-motion preference for JS-driven motion.
    const reduceMotion = window.matchMedia
        && window.matchMedia("(prefers-reduced-motion: reduce)").matches;

    // count-up the hero score (450-650ms) for a bit of life
    const scoreEl = root.querySelector(".mc-hero-score[data-count]");
    if (scoreEl) {
        const target = parseInt(scoreEl.getAttribute("data-count"), 10) || 0;
        if (reduceMotion) {
            scoreEl.textContent = target;
        } else {
            const from = Math.max(472, target - 24);
            const start = performance.now();
            const dur = 620;
            const tick = (now) => {
                const t = Math.min(1, (now - start) / dur);
                const eased = 1 - Math.pow(1 - t, 3);
                scoreEl.textContent = Math.round(from + (target - from) * eased);
                if (t < 1) { requestAnimationFrame(tick); }
            };
            requestAnimationFrame(tick);
        }
    }

    // three-score tiles: tap a tile to expand its honesty detail (accordion).
    const scoreTiles = Array.from(root.querySelectorAll(".mc-score-tile"));
    const scoreDetails = Array.from(root.querySelectorAll(".mc-score-detail"));
    const closeScores = () => {
        scoreTiles.forEach((t) => t.setAttribute("aria-expanded", "false"));
        scoreDetails.forEach((d) => d.classList.remove("open"));
    };
    scoreTiles.forEach((tile) => {
        tile.addEventListener("click", () => {
            const kind = tile.getAttribute("data-score");
            const detail = root.querySelector('.mc-score-detail[data-detail="' + kind + '"]');
            const isOpen = detail && detail.classList.contains("open");
            closeScores();
            if (detail && !isOpen) {
                tile.setAttribute("aria-expanded", "true");
                detail.classList.add("open");
            }
        });
    });

    // "Updated Xm ago" freshness ticker (status is regenerated on each render).
    const loadedAt = Date.now();
    const updatedEls = Array.from(root.querySelectorAll("[data-updated]"));
    if (updatedEls.length) {
        const fmtAgo = (ms) => {
            const m = Math.floor(ms / 60000);
            if (m <= 0) { return "Updated just now"; }
            if (m === 1) { return "Updated 1m ago"; }
            return "Updated " + m + "m ago";
        };
        const tickUpdated = () => {
            const txt = fmtAgo(Date.now() - loadedAt);
            updatedEls.forEach((el) => { el.textContent = txt; });
        };
        tickUpdated();
        setInterval(tickUpdated, 30000);
    }

    // plan editor save / clear
    const planSave = root.querySelector("[data-plan=save]");
    if (planSave) {
        planSave.addEventListener("click", () => {
            const panel = root.querySelector(".mc-plan");
            const dateStr = panel.querySelector("input[type=date]").value;
            const targetStr = panel.querySelector("input[type=number]").value;
            let epoch = 0;
            if (dateStr) {
                const parts = dateStr.split("-").map(Number);
                epoch = Math.floor(new Date(parts[0], parts[1] - 1, parts[2], 12).getTime() / 1000);
            }
            const target = targetStr ? parseInt(targetStr, 10) : 0;
            if (typeof pycmd === "function") { pycmd("mcatPlanSave:" + epoch + "," + target); }
        });
    }
    const planClear = root.querySelector("[data-plan=clear]");
    if (planClear) {
        planClear.addEventListener("click", () => {
            if (typeof pycmd === "function") { pycmd("mcatPlanSave:0,0"); }
        });
    }

    // knowledge map: cluster-first hybrid (clusters -> discipline -> full)
    const DISCIPLINE_ORDER = ["Bio", "Biochem", "GenChem", "Orgo", "Physics", "PsychSoc", "CARS"];
    const SECTION_COLORS = {
        Bio: "#4f74d6", Biochem: "#4f74d6",
        GenChem: "#e0555f", Orgo: "#e0555f", Physics: "#e0555f",
        PsychSoc: "#2fa96a", CARS: "#dda032", Other: "#8a90a2",
    };
    const laneLabel = { Bio: "Biology", Biochem: "Biochem", GenChem: "Gen Chem",
        Orgo: "Org Chem", Physics: "Physics", PsychSoc: "Psych/Soc", CARS: "CARS", Other: "Other" };
    const disciplineOf = (id) => id.split("::")[0] || "Other";
    const clusterKeyOf = (id) => {
        const d = disciplineOf(id);
        return DISCIPLINE_ORDER.includes(d) ? d : "Other";
    };
    const colorOf = (id) => SECTION_COLORS[clusterKeyOf(id)] || SECTION_COLORS.Other;
    const clamp01 = (x) => Math.max(0, Math.min(1, x || 0));

    const nodes = data.nodes || [];
    const edges = data.edges || [];
    const canvas = root.querySelector(".mc-map-canvas");
    const scroll = root.querySelector(".mc-map-scroll");
    const titleEl = root.querySelector("[data-map-title]");
    const controlsEl = root.querySelector("[data-map-controls]");
    const mapSkel = root.querySelector(".mc-map-skeleton");
    if (mapSkel) { mapSkel.remove(); }
    if (!canvas || !scroll || !controlsEl || !nodes.length) {
        if (canvas) { canvas.innerHTML = '<div class="mc-map-empty">Your concept map appears once the deck is loaded.</div>'; }
        return;
    }

    // longest-path depth (Kahn) over an id set + its edges
    function computeDepth(ids, edgeList, idset) {
        const succ = new Map(ids.map((id) => [id, []]));
        const indeg = new Map(ids.map((id) => [id, 0]));
        for (const e of edgeList) {
            if (idset.has(e.prerequisiteId) && idset.has(e.targetId)) {
                succ.get(e.prerequisiteId).push(e.targetId);
                indeg.set(e.targetId, indeg.get(e.targetId) + 1);
            }
        }
        const depth = new Map(ids.map((id) => [id, 0]));
        const remaining = new Map(indeg);
        const q = ids.filter((id) => remaining.get(id) === 0);
        while (q.length) {
            const u = q.shift();
            for (const v of succ.get(u)) {
                if (depth.get(v) < depth.get(u) + 1) { depth.set(v, depth.get(u) + 1); }
                remaining.set(v, remaining.get(v) - 1);
                if (remaining.get(v) === 0) { q.push(v); }
            }
        }
        return depth;
    }

    // node importance = how many concepts list it as a prerequisite (global)
    const deps = {};
    for (const e of edges) { deps[e.prerequisiteId] = (deps[e.prerequisiteId] || 0) + 1; }
    const maxDep = Math.max(1, ...Object.values(deps));
    const allIds = nodes.map((n) => n.id);
    const allIdset = new Set(allIds);
    const globalDepth = computeDepth(allIds, edges, allIdset);

    const svgNS = "http://www.w3.org/2000/svg";
    let currentView = "clusters";
    let currentDisc = null;
    let activeWheel = null;

    // Node description panel — pops up below the map when you click a concept.
    const mapDetail = root.querySelector(".mc-map-detail");
    const shortName = (id) => id.split("::").slice(-1)[0].replace(/_/g, " ");
    const hideMapDetail = () => {
        if (mapDetail) { mapDetail.hidden = true; mapDetail.replaceChildren(); }
    };
    const showMapNodeDetail = (n) => {
        if (!mapDetail) { return; }
        mapDetail.replaceChildren();
        mapDetail.hidden = false;
        mapDetail.style.setProperty("--sec", colorOf(n.id));
        const name = document.createElement("div");
        name.className = "mc-map-detail-name";
        name.textContent = shortName(n.id);
        mapDetail.appendChild(name);
        const status = n.fringe === "inner" ? "Mastered"
            : (n.fringe === "outer" ? "Ready to start" : "Locked");
        const attempts = (n.positive || 0) + (n.negative || 0);
        const acc = attempts > 0 ? Math.round((n.positive / attempts) * 100) : 0;
        const meta = document.createElement("div");
        meta.className = "mc-map-detail-meta";
        let metaTxt = (laneLabel[disciplineOf(n.id)] || disciplineOf(n.id))
            + " \u00b7 " + status + " \u00b7 " + Math.round(clamp01(n.mastery) * 100) + "% mastered";
        if ((n.answered || 0) > 0) { metaTxt += " \u00b7 " + acc + "% correct (" + n.answered + " answered)"; }
        meta.textContent = metaTxt;
        mapDetail.appendChild(meta);
        const prereqs = edges.filter((e) => e.targetId === n.id).map((e) => shortName(e.prerequisiteId));
        const unlocks = edges.filter((e) => e.prerequisiteId === n.id).map((e) => shortName(e.targetId));
        const roleParts = [];
        if (prereqs.length) { roleParts.push("Builds on " + prereqs.slice(0, 3).join(", ")); }
        if (unlocks.length) { roleParts.push("Unlocks " + unlocks.slice(0, 3).join(", ")); }
        if (roleParts.length) {
            const role = document.createElement("div");
            role.className = "mc-map-detail-role";
            role.textContent = roleParts.join("  \u00b7  ");
            mapDetail.appendChild(role);
        }
        if (n.fringe === "outer") {
            const start = document.createElement("button");
            start.type = "button";
            start.className = "mc-map-detail-start";
            start.textContent = "Start this topic";
            start.addEventListener("click", () => {
                if (typeof pycmd === "function") { pycmd("mcatStart:" + n.id); }
            });
            mapDetail.appendChild(start);
        }
    };

    // ---- clusters view -----------------------------------------------------
    function computeClusters() {
        const map = new Map();
        for (const n of nodes) {
            const key = clusterKeyOf(n.id);
            let c = map.get(key);
            if (!c) { c = { key: key, members: [], mastered: 0, ready: 0, masterySum: 0, depth: Infinity }; map.set(key, c); }
            c.members.push(n);
            c.masterySum += clamp01(n.mastery);
            if (n.fringe === "inner") { c.mastered += 1; }
            if (n.fringe === "outer") { c.ready += 1; }
            const d = globalDepth.get(n.id) || 0;
            if (d < c.depth) { c.depth = d; }
        }
        const out = [];
        for (const key of [...DISCIPLINE_ORDER, "Other"]) {
            const c = map.get(key);
            if (!c || !c.members.length) { continue; }
            c.count = c.members.length;
            c.meanMastery = c.masterySum / c.count;
            c.hasReady = c.ready > 0;
            c.color = SECTION_COLORS[key] || SECTION_COLORS.Other;
            c.label = laneLabel[key] || key;
            if (!isFinite(c.depth)) { c.depth = 0; }
            out.push(c);
        }
        return out;
    }

    function layoutClusters(clusters) {
        const n = clusters.length;
        const CELLW = 224, CELLH = 190, PADX = 22, PADY = 20;
        // primary: columns by dense-ranked min-depth so prereq flow reads L->R
        const depths = Array.from(new Set(clusters.map((c) => c.depth))).sort((a, b) => a - b);
        const colOfDepth = new Map(depths.map((d, i) => [d, i]));
        const colCount = {};
        let cells = clusters.map((c) => {
            const col = colOfDepth.get(c.depth);
            const row = colCount[col] || 0;
            colCount[col] = row + 1;
            return { c: c, col: col, row: row };
        });
        let ncols = depths.length;
        let centered = true;
        const maxColLen = Math.max(1, ...Object.values(colCount));
        // fall back to a tidy wrapped grid when the depth layout is degenerate
        if (ncols < 2 || ncols > 4 || maxColLen > 2) {
            const nrows = n > 4 ? 2 : 1;
            ncols = Math.ceil(n / nrows);
            cells = clusters.map((c, i) => ({ c: c, col: i % ncols, row: Math.floor(i / ncols) }));
            centered = false;
        }
        const colH = {};
        for (const cell of cells) { colH[cell.col] = Math.max(colH[cell.col] || 0, cell.row + 1); }
        const maxRows = Math.max(1, ...Object.values(colH));
        const width = PADX * 2 + ncols * CELLW;
        const height = PADY * 2 + maxRows * CELLH;
        for (const cell of cells) {
            cell.cx = PADX + cell.col * CELLW + CELLW / 2;
            if (centered) {
                const top = (height - colH[cell.col] * CELLH) / 2;
                cell.cy = top + cell.row * CELLH + CELLH / 2;
            } else {
                cell.cy = PADY + cell.row * CELLH + CELLH / 2;
            }
        }
        return { cells: cells, width: width, height: height };
    }

    function renderClusters() {
        const clusters = computeClusters();
        if (!clusters.length) {
            canvas.innerHTML = '<div class="mc-map-empty">Your concept map appears once the deck is loaded.</div>';
            return;
        }
        const layout = layoutClusters(clusters);
        const byKey = {};
        for (const cell of layout.cells) { byKey[cell.c.key] = cell; }
        canvas.style.width = layout.width + "px";
        canvas.style.height = layout.height + "px";

        const svg = document.createElementNS(svgNS, "svg");
        svg.classList.add("mc-map-svg");
        svg.setAttribute("viewBox", "0 0 " + layout.width + " " + layout.height);
        svg.style.width = layout.width + "px";
        svg.style.height = layout.height + "px";
        const defs = document.createElementNS(svgNS, "defs");
        const marker = document.createElementNS(svgNS, "marker");
        marker.setAttribute("id", "mcClusterArrow");
        marker.setAttribute("markerUnits", "userSpaceOnUse");
        marker.setAttribute("viewBox", "0 0 10 10");
        marker.setAttribute("refX", "8");
        marker.setAttribute("refY", "5");
        marker.setAttribute("markerWidth", "10");
        marker.setAttribute("markerHeight", "10");
        marker.setAttribute("orient", "auto");
        const mpath = document.createElementNS(svgNS, "path");
        mpath.setAttribute("d", "M0,0 L10,5 L0,10 z");
        mpath.setAttribute("class", "mc-cluster-arrow");
        marker.appendChild(mpath);
        defs.appendChild(marker);
        svg.appendChild(defs);
        canvas.appendChild(svg);

        // Uniform bubble size for every discipline — cleaner and calmer than
        // sizing by topic count.
        const radius = {};
        for (const c of clusters) { radius[c.key] = 46; }

        // one deduped directed arrow per cross-cluster prerequisite relation
        const pairSeen = new Set();
        for (const e of edges) {
            const from = clusterKeyOf(e.prerequisiteId);
            const to = clusterKeyOf(e.targetId);
            if (from === to) { continue; }
            const pk = from + "|" + to;
            if (pairSeen.has(pk)) { continue; }
            pairSeen.add(pk);
            const a = byKey[from], b = byKey[to];
            if (!a || !b) { continue; }
            const dx = b.cx - a.cx, dy = b.cy - a.cy;
            const dist = Math.hypot(dx, dy) || 1;
            const ux = dx / dist, uy = dy / dist;
            const ln = document.createElementNS(svgNS, "line");
            ln.setAttribute("class", "mc-cluster-edge");
            ln.setAttribute("x1", (a.cx + ux * (radius[from] + 4)).toFixed(1));
            ln.setAttribute("y1", (a.cy + uy * (radius[from] + 4)).toFixed(1));
            ln.setAttribute("x2", (b.cx - ux * (radius[to] + 11)).toFixed(1));
            ln.setAttribute("y2", (b.cy - uy * (radius[to] + 11)).toFixed(1));
            ln.setAttribute("marker-end", "url(#mcClusterArrow)");
            svg.appendChild(ln);
        }

        for (const cell of layout.cells) {
            const c = cell.c;
            const el = document.createElement("div");
            el.className = "mc-cluster" + (c.hasReady ? " ready" : "");
            el.style.left = cell.cx + "px";
            el.style.top = cell.cy + "px";
            el.setAttribute("role", "button");
            el.setAttribute("tabindex", "0");
            el.title = c.label + " \u2014 " + c.mastered + "/" + c.count + " mastered"
                + (c.ready ? " \u00b7 " + c.ready + " ready to start" : "") + " \u00b7 open";
            const dot = document.createElement("div");
            dot.className = "mc-cluster-dot";
        // Percentage = topics mastered / total topics in this discipline, so the
        // number, the ring fill, and the "X/Y mastered" label all agree.
        const masteredPct = c.count ? Math.round((c.mastered / c.count) * 100) : 0;
        dot.style.setProperty("--d", (radius[c.key] * 2).toFixed(1) + "px");
        dot.style.setProperty("--col", c.color);
        dot.style.setProperty("--fill", masteredPct + "%");
        const pct = document.createElement("span");
        pct.className = "mc-cluster-pct";
        pct.textContent = masteredPct + "%";
            dot.appendChild(pct);
            el.appendChild(dot);
            const lab = document.createElement("div");
            lab.className = "mc-cluster-label";
            const nm = document.createElement("span");
            nm.className = "mc-cluster-name";
            nm.textContent = c.label;
            const st = document.createElement("span");
            st.className = "mc-cluster-stat";
            st.textContent = c.mastered + "/" + c.count + " mastered" + (c.ready ? " \u00b7 " + c.ready + " ready" : "");
            lab.appendChild(nm);
            lab.appendChild(st);
            el.appendChild(lab);
            const go = () => setView("discipline", c.key);
            el.addEventListener("click", go);
            el.addEventListener("keydown", (ev) => {
                if (ev.key === "Enter" || ev.key === " ") { ev.preventDefault(); go(); }
            });
            canvas.appendChild(el);
        }
    }

    // ---- discipline + full views (layered "linear paths") ------------------
    function renderLayered(nodeFilter) {
        const subNodes = nodes.filter(nodeFilter);
        if (!subNodes.length) {
            canvas.innerHTML = '<div class="mc-map-empty">No concepts in this view yet.</div>';
            return;
        }
        const ids = subNodes.map((n) => n.id);
        const idset = new Set(ids);
        const subEdges = edges.filter((e) => idset.has(e.prerequisiteId) && idset.has(e.targetId));
        const depth = computeDepth(ids, subEdges, idset);

        const lanes = [...DISCIPLINE_ORDER, "Other"];
        const pos = {};
        const laneRanges = [];
        let rowTop = 0;
        let maxCol = 0;
        let minCol = Infinity;
        for (const lane of lanes) {
            const members = ids.filter((id) => clusterKeyOf(id) === lane);
            if (!members.length) { continue; }
            members.sort((a, b) => (depth.get(a) - depth.get(b)) || (a < b ? -1 : 1));
            const colCount = new Map();
            let laneRows = 0;
            for (const id of members) {
                const c = depth.get(id);
                const r = colCount.get(c) || 0;
                colCount.set(c, r + 1);
                pos[id] = { col: c, row: rowTop + r };
                if (r + 1 > laneRows) { laneRows = r + 1; }
                if (c > maxCol) { maxCol = c; }
                if (c < minCol) { minCol = c; }
            }
            laneRanges.push({ lane: lane, top: rowTop, rows: laneRows });
            rowTop += laneRows + 1;
        }
        if (!isFinite(minCol)) { minCol = 0; }
        if (minCol > 0) {
            for (const id in pos) { pos[id].col -= minCol; }
            maxCol -= minCol;
        }
        const cols = maxCol + 1;
        const rows = Math.max(rowTop, 1);

        const COLW = 134, ROWH = 48, PADX = 96, PADY = 32, LABELPAD = 150;
        let zoom = 1;
        const nx = (u) => PADX + u.col * COLW * zoom;
        const ny = (u) => PADY + u.row * ROWH * zoom;

        const svg = document.createElementNS(svgNS, "svg");
        svg.classList.add("mc-map-svg");
        canvas.appendChild(svg);

        const laneEls = [];
        for (const lr of laneRanges) {
            const band = document.createElement("div");
            band.className = "mc-lane-band";
            const label = document.createElement("div");
            label.className = "mc-lane-label";
            label.textContent = laneLabel[lr.lane] || lr.lane;
            canvas.appendChild(band);
            canvas.appendChild(label);
            laneEls.push({ lr: lr, band: band, label: label });
        }

        const lineEls = [];
        for (const e of subEdges) {
            const a = pos[e.prerequisiteId], b = pos[e.targetId];
            if (!a || !b) { continue; }
            const ln = document.createElementNS(svgNS, "line");
            ln.setAttribute("class", "mc-map-edge");
            svg.appendChild(ln);
            lineEls.push({ ln: ln, a: a, b: b });
        }

        const nodeEls = [];
        for (const n of subNodes) {
            const u = pos[n.id];
            if (!u) { continue; }
            const el = document.createElement("div");
            el.className = "mc-node " + (n.fringe || "locked");
            const color = colorOf(n.id);
            const dep = deps[n.id] || 0;
            const size = 13 + 12 * Math.sqrt(dep / maxDep);
            const mastery = clamp01(n.mastery);
            const dot = document.createElement("span");
            dot.className = "mc-node-dot";
            dot.style.setProperty("--dot", size + "px");
            dot.style.setProperty("--col", color);
            dot.style.setProperty("--fill", (mastery * 100).toFixed(0) + "%");
            el.appendChild(dot);
            if (n.fringe === "inner") {
                const chk = document.createElement("span");
                chk.className = "mc-node-check";
                chk.textContent = "\u2713";
                dot.appendChild(chk);
            }
            const label = document.createElement("span");
            label.className = "mc-node-label";
            label.textContent = n.id.split("::").slice(-1)[0].replace(/_/g, " ");
            el.appendChild(label);
            const disc = disciplineOf(n.id);
            const attempts = (n.positive || 0) + (n.negative || 0);
            const acc = attempts > 0 ? Math.round((n.positive / attempts) * 100) : 0;
            let tip = disc + " \u00b7 " + label.textContent + " \u2014 mastery " + Math.round(mastery * 100) + "%";
            tip += (n.answered || 0) > 0 ? " \u00b7 " + acc + "% correct (" + n.answered + " answered)" : " \u00b7 not started";
            if (n.fringe === "outer") { tip += " \u00b7 ready to start"; }
            else if (n.fringe === "inner") { tip += " \u00b7 mastered"; }
            el.title = tip;
            if (n.fringe === "outer") { el.classList.add("startable"); }
            // Click ANY node to pop its description below the map (starting a topic
            // happens via the "Start this topic" button in that panel).
            el.style.cursor = "pointer";
            el.addEventListener("click", (ev) => {
                ev.stopPropagation();
                showMapNodeDetail(n);
            });
            canvas.appendChild(el);
            nodeEls.push({ el: el, u: u });
        }

        const paint = () => {
            const w = PADX + Math.max(cols - 1, 0) * COLW * zoom + LABELPAD;
            const h = PADY * 2 + Math.max(rows - 1, 0) * ROWH * zoom + ROWH;
            canvas.style.width = w + "px";
            canvas.style.height = h + "px";
            svg.setAttribute("viewBox", "0 0 " + w + " " + h);
            svg.style.width = w + "px";
            svg.style.height = h + "px";
            for (const item of lineEls) {
                item.ln.setAttribute("x1", nx(item.a)); item.ln.setAttribute("y1", ny(item.a));
                item.ln.setAttribute("x2", nx(item.b)); item.ln.setAttribute("y2", ny(item.b));
            }
            const showLabels = zoom >= 0.5;
            for (const item of nodeEls) {
                item.el.style.left = nx(item.u) + "px";
                item.el.style.top = ny(item.u) + "px";
                item.el.classList.toggle("nolabel", !showLabels);
            }
            for (const item of laneEls) {
                const top = PADY + item.lr.top * ROWH * zoom - ROWH * 0.55 * zoom;
                const height = (item.lr.rows - 1) * ROWH * zoom + ROWH * 1.1 * zoom;
                item.band.style.top = top + "px";
                item.band.style.height = Math.max(height, ROWH) + "px";
                item.band.style.width = w + "px";
                item.label.style.top = (top + 6) + "px";
            }
        };
        const setZoom = (z, anchor) => {
            const prev = zoom;
            zoom = Math.max(0.45, Math.min(2.0, z));
            paint();
            if (anchor) {
                const r = zoom / prev;
                scroll.scrollLeft = (scroll.scrollLeft + anchor.x) * r - anchor.x;
                scroll.scrollTop = (scroll.scrollTop + anchor.y) * r - anchor.y;
            }
        };
        const fit = () => {
            const availW = scroll.clientWidth - PADX - LABELPAD;
            const zW = availW / Math.max((cols - 1) * COLW, 1);
            zoom = Math.max(0.45, Math.min(1.3, zW || 1));
            paint();
        };
        const center = { x: scroll.clientWidth / 2, y: scroll.clientHeight / 2 };
        const btnIn = controlsEl.querySelector("[data-map=in]");
        const btnOut = controlsEl.querySelector("[data-map=out]");
        const btnFit = controlsEl.querySelector("[data-map=fit]");
        if (btnIn) { btnIn.addEventListener("click", () => setZoom(zoom * 1.2, center)); }
        if (btnOut) { btnOut.addEventListener("click", () => setZoom(zoom / 1.2, center)); }
        if (btnFit) { btnFit.addEventListener("click", fit); }
        activeWheel = (e) => {
            if (!(e.ctrlKey || e.metaKey)) { return; }
            e.preventDefault();
            const rect = scroll.getBoundingClientRect();
            setZoom(zoom * (e.deltaY < 0 ? 1.12 : 1 / 1.12), { x: e.clientX - rect.left, y: e.clientY - rect.top });
        };
        fit();
    }

    // ---- view switching ----------------------------------------------------
    function updateHead() {
        let title = "Your skill tree";
        if (currentView === "discipline") { title = laneLabel[currentDisc] || currentDisc || "Discipline"; }
        if (titleEl) { titleEl.textContent = title; }
        scroll.classList.toggle("is-clusters", currentView === "clusters");
        if (currentView === "clusters") {
            controlsEl.innerHTML = "";
        } else {
            controlsEl.innerHTML =
                '<button class="mc-map-crumb" data-map="clusters">\u2039 All disciplines</button>'
                + '<div class="mc-map-zoom">'
                + '<button data-map="out" title="Zoom out">\u2212</button>'
                + '<button data-map="fit" title="Fit to view">Fit</button>'
                + '<button data-map="in" title="Zoom in">+</button>'
                + '</div>';
        }
    }

    function setView(view, disc) {
        activeWheel = null;
        hideMapDetail();
        currentView = view;
        currentDisc = disc || null;
        canvas.innerHTML = "";
        canvas.style.width = "";
        canvas.style.height = "";
        scroll.scrollLeft = 0;
        scroll.scrollTop = 0;
        updateHead();
        if (view === "discipline" && currentDisc) {
            renderLayered((n) => clusterKeyOf(n.id) === currentDisc);
        } else {
            currentView = "clusters";
            renderClusters();
        }
    }

    controlsEl.addEventListener("click", (e) => {
        const b = e.target.closest("[data-map]");
        if (!b || !controlsEl.contains(b)) { return; }
        const act = b.getAttribute("data-map");
        if (act === "clusters") { setView("clusters"); }
    });
    scroll.addEventListener("wheel", (e) => {
        if (activeWheel) { activeWheel(e); }
    }, { passive: false });

    setView("clusters");
})();
"""


_DASH_CSS = """
.mcat-dash {
    box-sizing: border-box;
    max-width: 1080px;
    margin: 0 auto 1.5rem;
    padding: 0 0.5rem;
    text-align: left;
    font-family: var(--mc-font);
    color: var(--mc-text);
    -webkit-font-smoothing: antialiased;
}
.mcat-dash *, .mcat-dash *::before, .mcat-dash *::after { box-sizing: border-box; }

.mc-topline { display: flex; align-items: center; justify-content: space-between; margin: 0 0 1rem; }
.mc-brandmark { display: flex; align-items: center; gap: 0.6rem; }
.mc-logo {
    width: 2.1rem; height: 2.1rem; border-radius: 0.6rem;
    display: grid; place-items: center;
    background: linear-gradient(135deg, var(--mc-brand), var(--mc-brand-strong));
    color: #fff; font-weight: 800; font-size: 1.15rem;
    box-shadow: var(--mc-shadow-sm);
}
.mc-brandname { font-weight: 700; font-size: 0.98rem; letter-spacing: -0.01em; }
.mc-brandsub { font-size: 0.72rem; color: var(--mc-text-faint); }

.mc-eyebrow {
    font-size: 0.68rem; font-weight: 700; letter-spacing: 0.09em;
    text-transform: uppercase; color: var(--mc-brand);
}
.mc-section-title {
    font-size: 0.9rem; font-weight: 700; letter-spacing: -0.01em;
    margin: 1.4rem 0.15rem 0.7rem; color: var(--mc-text);
}

/* "All decks" heading shown above the standard Anki deck table on the home
   screen (lives outside .mcat-dash, so it is intentionally unscoped). */
.mcat-decks-title {
    max-width: 1080px;
    margin: 1.8rem auto 0.5rem;
    padding: 0 0.6rem;
    font-family: var(--mc-font);
    font-size: 0.78rem; font-weight: 700; letter-spacing: 0.08em;
    text-transform: uppercase; text-align: left;
    color: var(--mc-text-faint);
}

.mc-card {
    background: var(--mc-surface);
    border: 1px solid var(--mc-border);
    border-radius: var(--mc-r-lg);
    box-shadow: var(--mc-shadow-sm);
}

/* hero */
.mc-hero-card {
    display: flex; align-items: center; gap: clamp(1rem, 4vw, 2.5rem);
    background:
        radial-gradient(120% 140% at 85% -10%, var(--mc-brand-soft), transparent 60%),
        var(--mc-surface);
    border: 1px solid var(--mc-border);
    border-radius: var(--mc-r-lg);
    box-shadow: var(--mc-shadow-md);
    padding: clamp(1.1rem, 3vw, 2rem);
}
.mc-hero-ring { position: relative; flex: 0 0 auto; width: clamp(9rem, 22vw, 12rem); height: clamp(9rem, 22vw, 12rem); }
.mc-ring { width: 100%; height: 100%; display: block; }
.mc-ring-val { animation: mcRingIn var(--mc-dur-sweep) var(--mc-ease) both; }
@keyframes mcRingIn { from { stroke-dashoffset: __RING_C__; } }
.mc-ring-band { opacity: 0.28; }
.mc-ring-center {
    position: absolute; inset: 0; display: flex; flex-direction: column;
    align-items: center; justify-content: center; text-align: center;
}
.mc-hero-num { display: flex; flex-direction: column; align-items: center; line-height: 1; }
.mc-hero-score { font-size: clamp(2.4rem, 7vw, 3.2rem); font-weight: 800; letter-spacing: -0.03em; }
.mc-hero-score.mc-empty { color: var(--mc-text-faint); font-weight: 700; }
.mc-hero-cap { margin-top: 0.35rem; font-size: 0.66rem; text-transform: uppercase; letter-spacing: 0.08em; color: var(--mc-text-faint); }
.mc-hero-band { margin-top: 0.3rem; font-size: 0.72rem; font-weight: 700; color: var(--mc-brand); }
.mc-hero-body { flex: 1 1 auto; min-width: 0; }
.mc-hero-title { font-size: clamp(1.25rem, 3.4vw, 1.75rem); font-weight: 800; letter-spacing: -0.02em; margin: 0.15rem 0 0.9rem; }
.mc-hero-range { max-width: 24rem; }
.mc-hero-hint { font-size: 0.82rem; color: var(--mc-text-muted); line-height: 1.5; margin-bottom: 1rem; }
.mc-scale { position: relative; height: 0.5rem; border-radius: var(--mc-r-pill); background: var(--mc-track); overflow: hidden; }
.mc-scale-band { position: absolute; top: 0; bottom: 0; background: var(--mc-brand-soft); }
.mc-scale-marker { position: absolute; top: -0.15rem; width: 0.5rem; height: 0.8rem; border-radius: var(--mc-r-pill); background: var(--mc-brand); transform: translateX(-50%); box-shadow: 0 0 0 3px var(--mc-surface); }
.mc-scale-ends { display: flex; justify-content: space-between; margin-top: 0.35rem; font-size: 0.72rem; color: var(--mc-text-faint); }
.mc-scale-ends b { color: var(--mc-text); }

.mc-cta {
    display: inline-flex; align-items: center; gap: 0.5rem;
    margin-top: 1.1rem; padding: 0.7rem 1.3rem;
    border: none; border-radius: var(--mc-r-pill);
    background: linear-gradient(135deg, var(--mc-brand), var(--mc-brand-strong));
    color: #fff; font: inherit; font-weight: 700; font-size: 0.9rem; cursor: pointer;
    box-shadow: 0 6px 16px var(--mc-brand-soft);
    transition: transform 0.08s ease, box-shadow 0.15s ease;
}
.mc-cta:hover { transform: translateY(-1px); box-shadow: 0 10px 22px var(--mc-brand-soft); }
.mc-cta:active { transform: translateY(0) scale(0.97); }
.mc-cta-sm { margin-top: 0; padding: 0.5rem 1rem; font-size: 0.82rem; }
.mc-cta-arrow { transition: transform 0.15s ease; }
.mc-cta:hover .mc-cta-arrow { transform: translateX(3px); }

/* mini cards */
.mc-mini-row { display: grid; grid-template-columns: 1fr 1fr; gap: 0.9rem; margin-top: 0.9rem; }
.mc-mini { padding: 1rem 1.1rem; display: flex; flex-direction: column; gap: 0.5rem; }
.mc-mini-head { display: flex; align-items: center; gap: 0.4rem; font-size: 0.72rem; font-weight: 700; text-transform: uppercase; letter-spacing: 0.07em; color: var(--mc-text-faint); }
.mc-mini-ico { font-size: 0.9rem; }
.mc-stat-big { display: flex; align-items: baseline; gap: 0.3rem; }
.mc-stat-num { font-size: 2.3rem; font-weight: 800; letter-spacing: -0.03em; }
.mc-stat-unit { font-size: 0.9rem; font-weight: 600; color: var(--mc-text-muted); }
.mc-stat-sub { font-size: 0.82rem; color: var(--mc-text); font-weight: 600; }
.mc-stat-sub b { color: var(--mc-text); }
.mc-stat-sub2 { font-size: 0.75rem; color: var(--mc-text-faint); }
.mc-stat-empty { font-size: 0.82rem; color: var(--mc-text-muted); line-height: 1.5; }
.mc-mini-ring { position: relative; width: 4.6rem; height: 4.6rem; }
.mc-mini-ring .mc-ring { width: 100%; height: 100%; }
.mc-ring-center-sm { position: absolute; inset: 0; display: flex; flex-direction: column; align-items: center; justify-content: center; }
.mc-mini-pct { font-size: 1.15rem; font-weight: 800; }

.mc-chip {
    align-self: flex-start; margin-top: auto;
    padding: 0.4rem 0.85rem; border-radius: var(--mc-r-pill);
    border: 1px solid var(--mc-border-strong); background: var(--mc-surface-2);
    color: var(--mc-text); font: inherit; font-weight: 600; font-size: 0.78rem; cursor: pointer;
    transition: border-color 0.12s ease, background 0.12s ease, color 0.12s ease;
}
.mc-chip:hover { border-color: var(--mc-brand); color: var(--mc-brand); }
.mc-chip-accent { border-color: var(--mc-brand); color: var(--mc-brand); background: var(--mc-brand-soft); }

/* plan editor */
.mc-plan {
    display: none; margin-top: 0.9rem; padding: 1.1rem 1.2rem;
    background: var(--mc-surface); border: 1px solid var(--mc-border);
    border-radius: var(--mc-r-lg); box-shadow: var(--mc-shadow-md);
    gap: 0.8rem; flex-wrap: wrap; align-items: flex-end;
}
.mc-plan.open { display: flex; }
.mc-plan-head { flex: 1 0 100%; display: flex; justify-content: space-between; align-items: center; font-weight: 700; font-size: 0.9rem; }
.mc-plan-x { border: none; background: transparent; color: var(--mc-text-faint); font-size: 1rem; cursor: pointer; }
.mc-plan-field { display: flex; flex-direction: column; gap: 0.3rem; font-size: 0.75rem; color: var(--mc-text-muted); font-weight: 600; }
.mc-plan-field input {
    padding: 0.5rem 0.7rem; border: 1px solid var(--mc-border-strong);
    border-radius: var(--mc-r-sm); background: var(--mc-surface-2);
    color: var(--mc-text); font: inherit; font-size: 0.9rem; min-width: 11rem;
}
.mc-plan-field input:focus { outline: none; border-color: var(--mc-brand); box-shadow: 0 0 0 3px var(--mc-brand-soft); }
.mc-plan-actions { display: flex; gap: 0.5rem; margin-left: auto; }
.mc-plan-actions .mc-chip { margin-top: 0; }

/* section cards */
.mc-sec-control { display: flex; align-items: center; gap: 0.6rem; flex-wrap: wrap; margin: -0.2rem 0.15rem 0.7rem; }
.mc-sec-control-label { font-size: 0.78rem; color: var(--mc-text-muted); }
.mc-sec-control-label b { color: var(--mc-text); }
.mc-sec-control-on { padding: 0.5rem 0.7rem; border: 1px solid var(--mc-brand); background: var(--mc-brand-soft); border-radius: var(--mc-r-md); }
.mc-sec-control-on .mc-chip { margin-left: auto; margin-top: 0; }
.mc-sec-grid { display: grid; grid-template-columns: repeat(4, 1fr); gap: 0.9rem; }
.mc-sec {
    padding: 1rem; display: flex; flex-direction: column; align-items: center; text-align: center;
    cursor: pointer;
    transition: border-color var(--mc-dur-move) var(--mc-ease),
        box-shadow var(--mc-dur-move) var(--mc-ease),
        transform var(--mc-dur-press) var(--mc-ease);
}
.mc-sec:hover { border-color: var(--sec); box-shadow: var(--mc-shadow-md); transform: translateY(-1px); }
.mc-sec:active { transform: scale(0.98); }
.mc-sec:focus-visible { outline: none; border-color: var(--sec); box-shadow: 0 0 0 3px color-mix(in srgb, var(--sec) 35%, transparent); }
.mc-sec-active { border-color: var(--sec); box-shadow: 0 0 0 1px var(--sec), var(--mc-shadow-md); }
.mc-sec-cta { margin-top: 0.7rem; width: 100%; padding-top: 0.6rem; border-top: 1px solid var(--mc-border); font-size: 0.74rem; font-weight: 700; color: var(--sec); }
.mc-sec-cta-on { color: var(--mc-good); }
.mc-sec-head { display: flex; align-items: center; gap: 0.4rem; width: 100%; justify-content: center; }
.mc-sec-dot { width: 0.55rem; height: 0.55rem; border-radius: 50%; background: var(--sec); }
.mc-sec-name { font-weight: 700; font-size: 0.86rem; }
.mc-sec-tag { font-size: 0.6rem; color: var(--mc-text-faint); background: var(--mc-surface-2); padding: 0.1rem 0.35rem; border-radius: var(--mc-r-pill); }
.mc-sec-ring { position: relative; width: 6.2rem; height: 6.2rem; margin: 0.6rem 0 0.1rem; }
.mc-sec-ring .mc-ring { width: 100%; height: 100%; }
.mc-sec-score { font-size: 1.5rem; font-weight: 800; letter-spacing: -0.02em; }
.mc-sec-score.mc-empty { color: var(--mc-text-faint); }
.mc-sec-range { font-size: 0.68rem; color: var(--mc-text-muted); font-weight: 600; }
.mc-sec-rlabel { font-size: 0.64rem; color: var(--mc-text-faint); margin-bottom: 0.7rem; }
.mc-sec-metrics { width: 100%; display: flex; flex-direction: column; gap: 0.2rem; }
.mc-sec-metric { display: flex; justify-content: space-between; font-size: 0.72rem; color: var(--mc-text-muted); margin-top: 0.35rem; }
.mc-sec-metric b { color: var(--mc-text); }
.mc-bar { height: 0.4rem; border-radius: var(--mc-r-pill); background: var(--mc-track); overflow: hidden; }
.mc-bar-fill { height: 100%; border-radius: var(--mc-r-pill); width: 0; transition: width var(--mc-dur-sweep) var(--mc-ease); }

/* knowledge map */
.mc-map-card { margin-top: 1.4rem; padding: 1.1rem 1.2rem; }
.mc-map-head { display: flex; align-items: center; gap: 1rem; flex-wrap: wrap; }
.mc-map-title { font-size: 1.05rem; font-weight: 800; letter-spacing: -0.02em; }
.mc-map-stats { display: flex; gap: 1rem; font-size: 0.76rem; color: var(--mc-text-muted); margin-left: 0.5rem; }
.mc-map-stats b { color: var(--mc-text); font-size: 0.9rem; }
.mc-map-stats .mc-dot-ok b { color: var(--mc-good); }
.mc-map-stats .mc-dot-ready b { color: var(--mc-brand); }
.mc-map-controls { margin-left: auto; display: flex; gap: 0.3rem; }
.mc-map-controls button {
    min-width: 2rem; padding: 0.3rem 0.6rem; border-radius: var(--mc-r-sm);
    border: 1px solid var(--mc-border-strong); background: var(--mc-surface-2);
    color: var(--mc-text); font: inherit; font-weight: 600; font-size: 0.82rem; cursor: pointer;
}
.mc-map-controls button:hover { border-color: var(--mc-brand); color: var(--mc-brand); }
.mc-map-legend { display: flex; flex-wrap: wrap; gap: 0.4rem 1rem; margin: 0.8rem 0; font-size: 0.72rem; color: var(--mc-text-muted); align-items: center; }
.mc-map-legend span { display: inline-flex; align-items: center; gap: 0.35rem; }
.mc-map-legend i { width: 0.7rem; height: 0.7rem; border-radius: 50%; display: inline-block; }
.mc-lg-ready i { background: transparent; border: 2px solid var(--mc-good); }
.mc-lg-lock i { background: var(--mc-text-faint); opacity: 0.4; }
.mc-lg-hint { color: var(--mc-text-faint); margin-left: auto; }

.mc-map-scroll {
    position: relative; overflow: auto; border: 1px solid var(--mc-border);
    border-radius: var(--mc-r-md); background: var(--mc-surface-2);
    height: 24rem; max-height: 60vh;
}
.mc-map-canvas { position: relative; }
.mc-map-svg { position: absolute; left: 0; top: 0; overflow: visible; }
.mc-map-edge { stroke: var(--mc-border-strong); stroke-width: 1.5; vector-effect: non-scaling-stroke; }
.mc-lane-band { position: absolute; left: 0; background: color-mix(in srgb, var(--mc-surface) 55%, transparent); border-top: 1px dashed var(--mc-border); pointer-events: none; }
.mc-lane-label { position: absolute; left: 0.6rem; font-size: 0.66rem; font-weight: 700; letter-spacing: 0.04em; text-transform: uppercase; color: var(--mc-text-faint); pointer-events: none; background: var(--mc-surface-2); padding: 0 0.25rem; border-radius: 4px; }
.mc-map-empty { padding: 2rem; color: var(--mc-text-faint); font-size: 0.85rem; text-align: center; }

.mc-node { position: absolute; transform: translate(-50%, -50%); display: flex; align-items: center; gap: 0.4rem; }
.mc-node-dot {
    position: relative; flex: 0 0 auto;
    width: var(--dot); height: var(--dot); border-radius: 50%;
    background: var(--mc-track);
    display: grid; place-items: center;
}
.mc-node-dot::after {
    content: ""; position: absolute; inset: 0; border-radius: 50%;
    background: radial-gradient(circle, var(--col) var(--fill), transparent calc(var(--fill) + 1%));
    box-shadow: inset 0 0 0 2px color-mix(in srgb, var(--col) 55%, transparent);
}
.mc-node.locked { opacity: 0.5; }
.mc-node.locked .mc-node-dot { filter: grayscale(0.4); }
.mc-node.startable .mc-node-dot { box-shadow: 0 0 0 3px color-mix(in srgb, var(--mc-good) 55%, transparent); cursor: pointer; }
.mc-node.startable { cursor: pointer; }
.mc-node-check { position: relative; z-index: 1; color: #fff; font-size: calc(var(--dot) * 0.55); font-weight: 900; }
.mc-node-label { font-size: 0.72rem; color: var(--mc-text); white-space: nowrap; font-weight: 500; max-width: 8.5rem; overflow: hidden; text-overflow: ellipsis; }
.mc-node.nolabel .mc-node-label { display: none; }
.mc-node.startable .mc-node-label { font-weight: 700; }

/* knowledge map: cluster bubbles + drill-in / full controls */
.mc-map-controls { align-items: center; }
.mc-map-zoom { display: flex; gap: 0.3rem; }
.mc-map-controls .mc-map-crumb, .mc-map-controls .mc-map-full-btn {
    min-width: 0; padding: 0.32rem 0.8rem; border-radius: var(--mc-r-pill);
    border: 1px solid var(--mc-border-strong); background: var(--mc-surface-2);
    color: var(--mc-text-muted); font-weight: 700; font-size: 0.78rem; white-space: nowrap;
}
.mc-map-controls .mc-map-crumb:hover, .mc-map-controls .mc-map-full-btn:hover {
    border-color: var(--mc-brand); color: var(--mc-brand);
}

/* clusters view grows to fit its bubbles instead of scrolling a fixed box */
.mc-map-scroll.is-clusters { height: auto; max-height: none; }

/* node description panel below the map (click a node to populate); themed to
   the concept's MCAT section colour via the inline --sec variable. */
.mc-map-detail {
    margin-top: 0.9rem;
    padding: 0.8rem 0.95rem;
    border: 1px solid var(--mc-border);
    border-left: 3px solid var(--sec, var(--mc-brand));
    border-radius: var(--mc-r-md);
    background: color-mix(in srgb, var(--mc-surface) 86%, var(--sec, var(--mc-brand)));
}
.mc-map-detail-name {
    font-weight: 800; font-size: 1rem; letter-spacing: -0.01em;
    color: var(--sec, var(--mc-text));
}
.mc-map-detail-meta { margin-top: 0.15rem; font-size: 0.8rem; color: var(--mc-text-muted); }
.mc-map-detail-role { margin-top: 0.3rem; font-size: 0.8rem; color: var(--mc-text-muted); }
.mc-map-detail-start {
    margin-top: 0.6rem; padding: 0.4rem 0.9rem;
    border: 0; border-radius: var(--mc-r-pill); cursor: pointer;
    background: var(--sec, var(--mc-brand)); color: #fff; font-weight: 700; font-size: 0.82rem;
}
.mc-map-detail-start:hover { filter: brightness(1.06); }

.mc-cluster {
    position: absolute; transform: translate(-50%, -50%);
    cursor: pointer; z-index: 1; outline: none;
    transition: transform var(--mc-dur-move) var(--mc-ease);
}
.mc-cluster:hover, .mc-cluster:focus-visible { transform: translate(-50%, -50%) scale(1.06); z-index: 3; }
.mc-cluster-dot {
    position: relative; width: var(--d); height: var(--d); border-radius: 50%;
    background: conic-gradient(var(--col) 0 var(--fill), color-mix(in srgb, var(--col) 16%, transparent) var(--fill) 100%);
    display: grid; place-items: center; box-shadow: var(--mc-shadow-md);
    transition: box-shadow var(--mc-dur-move) var(--mc-ease);
}
.mc-cluster-dot::before {
    content: ""; position: absolute; inset: 15%; border-radius: 50%;
    background: var(--mc-surface); box-shadow: inset 0 0 0 1px var(--mc-border);
}
.mc-cluster:hover .mc-cluster-dot { box-shadow: var(--mc-shadow-lg); }
.mc-cluster:focus-visible .mc-cluster-dot { box-shadow: 0 0 0 3px var(--mc-brand-soft), var(--mc-shadow-lg); }
.mc-cluster.ready .mc-cluster-dot { box-shadow: 0 0 0 4px color-mix(in srgb, var(--mc-good) 42%, transparent), var(--mc-shadow-md); }
.mc-cluster.ready:hover .mc-cluster-dot { box-shadow: 0 0 0 4px color-mix(in srgb, var(--mc-good) 55%, transparent), var(--mc-shadow-lg); }
.mc-cluster-pct {
    position: relative; z-index: 1; font-weight: 800;
    font-size: clamp(0.8rem, calc(var(--d) * 0.2), 1.35rem);
    color: var(--mc-text); font-variant-numeric: tabular-nums;
}
.mc-cluster-label {
    position: absolute; top: calc(100% + 0.45rem); left: 50%; transform: translateX(-50%);
    width: max-content; max-width: 11rem; text-align: center;
    display: flex; flex-direction: column; gap: 0.1rem; pointer-events: none;
}
.mc-cluster-name { font-weight: 800; font-size: 0.85rem; letter-spacing: -0.01em; color: var(--mc-text); }
.mc-cluster-stat { font-size: 0.71rem; color: var(--mc-text-muted); font-weight: 600; }
.mc-cluster-edge { stroke: var(--mc-border-strong); stroke-width: 1.6; fill: none; opacity: 0.7; vector-effect: non-scaling-stroke; }
.mc-cluster-arrow { fill: var(--mc-border-strong); }

/* ---- motion: staggered entrance for the top-level dashboard blocks (~50ms) ---- */
@keyframes mcEnter { from { opacity: 0; transform: translateY(10px); } to { opacity: 1; transform: none; } }
@keyframes mcShimmer { from { background-position: 180% 0; } to { background-position: -180% 0; } }
.mcat-dash > * { animation: mcEnter var(--mc-dur-move) var(--mc-ease) both; }
.mcat-dash > *:nth-child(1) { animation-delay: 0s; }
.mcat-dash > *:nth-child(2) { animation-delay: 0.05s; }
.mcat-dash > *:nth-child(3) { animation-delay: 0.10s; }
.mcat-dash > *:nth-child(4) { animation-delay: 0.15s; }
.mcat-dash > *:nth-child(5) { animation-delay: 0.20s; }
.mcat-dash > *:nth-child(6) { animation-delay: 0.25s; }
.mcat-dash > *:nth-child(7) { animation-delay: 0.30s; }
.mcat-dash > *:nth-child(8) { animation-delay: 0.35s; }
.mcat-dash > *:nth-child(9) { animation-delay: 0.40s; }
.mcat-dash > *:nth-child(n+10) { animation-delay: 0.45s; }

/* Press + hover response for clickables that lacked one (spec: no dead clicks). */
.mc-chip, .mc-map-controls button, .mc-plan-x, .home-theme-toggle button {
    transition: border-color var(--mc-dur-press) var(--mc-ease),
        background var(--mc-dur-press) var(--mc-ease),
        color var(--mc-dur-press) var(--mc-ease),
        transform var(--mc-dur-press) var(--mc-ease);
}
.mc-chip:active, .mc-map-controls button:active, .mc-plan-x:active,
.home-theme-toggle button:active { transform: scale(0.97); }

/* ---- three scores: memory / performance / readiness ---- */
.mc-scores-sub { font-size: 0.78rem; color: var(--mc-text-muted); margin: -0.3rem 0.15rem 0.7rem; }
.mc-scores { display: flex; flex-direction: column; gap: 0.7rem; }
.mc-score-tiles { display: grid; grid-template-columns: repeat(3, 1fr); gap: 0.7rem; }
.mc-score-tile {
    display: flex; flex-direction: column; gap: 0.25rem; text-align: left;
    padding: 0.9rem; border-radius: var(--mc-r-md);
    border: 1px solid var(--mc-border); background: var(--mc-surface);
    box-shadow: var(--mc-shadow-sm); color: var(--mc-text); font: inherit; cursor: pointer;
    transition: border-color var(--mc-dur-move) var(--mc-ease),
        box-shadow var(--mc-dur-move) var(--mc-ease),
        transform var(--mc-dur-press) var(--mc-ease);
}
.mc-score-tile:hover { border-color: var(--acc); box-shadow: var(--mc-shadow-md); }
.mc-score-tile:active { transform: scale(0.97); }
.mc-score-tile[aria-expanded="true"] { border-color: var(--acc); box-shadow: 0 0 0 1px var(--acc), var(--mc-shadow-md); }
.mc-score-name { font-size: 0.64rem; font-weight: 800; letter-spacing: 0.07em; text-transform: uppercase; color: var(--acc); }
.mc-score-est { font-size: 1.5rem; font-weight: 800; letter-spacing: -0.02em; font-variant-numeric: tabular-nums; line-height: 1.05; }
.mc-score-est.mc-empty { color: var(--mc-text-faint); letter-spacing: 0.12em; }
.mc-score-range { font-size: 0.7rem; color: var(--mc-text-muted); font-weight: 600; }
.mc-score-tile .mc-bar { margin-top: 0.4rem; }
.mc-score-tag { align-self: flex-start; font-size: 0.56rem; font-weight: 800; text-transform: uppercase; letter-spacing: 0.05em; color: var(--mc-warn); background: color-mix(in srgb, var(--mc-warn) 16%, transparent); padding: 0.06rem 0.4rem; border-radius: var(--mc-r-pill); }

/* Baseline (prior) label: a number is shown but not yet evidence-backed. Amber,
   distinct from the neutral abstain block (which shows no number). */
.mc-baseline { margin-top: 0.55rem; font-size: 0.76rem; font-weight: 700; color: var(--mc-warn); background: color-mix(in srgb, var(--mc-warn) 12%, transparent); border: 1px solid color-mix(in srgb, var(--mc-warn) 32%, transparent); border-radius: var(--mc-r-sm); padding: 0.45rem 0.6rem; line-height: 1.35; }
.mc-hero-body .mc-baseline { max-width: 26rem; }
.mc-hero-howsure { display: flex; align-items: center; gap: 0.5rem; flex-wrap: wrap; margin-top: 0.65rem; }
.mc-hero-covered { font-size: 0.78rem; color: var(--mc-text-muted); font-weight: 600; }

.mc-score-detail {
    display: none; padding: 0.9rem 1rem; border-radius: var(--mc-r-md);
    border: 1px solid var(--mc-border); border-left: 3px solid var(--acc);
    background: var(--mc-surface); box-shadow: var(--mc-shadow-sm);
}
.mc-score-detail.open { display: block; animation: mcEnter var(--mc-dur-move) var(--mc-ease) both; }
.mc-score-detail-head { display: flex; align-items: center; gap: 0.45rem; margin-bottom: 0.5rem; }
.mc-score-dot { width: 0.55rem; height: 0.55rem; border-radius: 50%; background: var(--acc); flex: 0 0 auto; }
.mc-score-detail-title { font-weight: 700; font-size: 0.86rem; }
.mc-updated { margin-left: auto; font-size: 0.68rem; color: var(--mc-text-faint); }
.mc-kv { display: flex; justify-content: space-between; gap: 0.6rem; font-size: 0.76rem; color: var(--mc-text-muted); padding: 0.18rem 0; }
.mc-kv b { color: var(--mc-text); font-weight: 700; text-align: right; }
.mc-howsure { font-size: 0.68rem; font-weight: 700; color: var(--mc-brand); background: var(--mc-brand-soft); padding: 0.1rem 0.5rem; border-radius: var(--mc-r-pill); }
.mc-score-abstain { font-size: 0.82rem; font-weight: 700; color: var(--mc-text); background: var(--mc-surface-2); border-radius: var(--mc-r-sm); padding: 0.5rem 0.6rem; margin-bottom: 0.3rem; }
.mc-note { margin-top: 0.5rem; }
.mc-note > span { font-size: 0.62rem; text-transform: uppercase; letter-spacing: 0.06em; color: var(--mc-text-faint); }
.mc-note p { margin: 0.15rem 0 0; font-size: 0.76rem; color: var(--mc-text); line-height: 1.45; }

/* hero abstain (no projection yet) */
.mc-hero-abstain { max-width: 26rem; display: flex; flex-direction: column; gap: 0.55rem; }
.mc-abstain-line { font-size: 0.92rem; font-weight: 700; color: var(--mc-text); }
.mc-hero-abstain .mc-hero-hint { margin-bottom: 0; }

/* skeleton shimmer while the map paints (spec: shimmer, not spinners) */
.mc-skeleton { border-radius: var(--mc-r-md); background: linear-gradient(100deg, var(--mc-track) 30%, var(--mc-surface-2) 50%, var(--mc-track) 70%); background-size: 220% 100%; animation: mcShimmer 1.2s linear infinite; }
.mc-map-skeleton { position: absolute; inset: 0.6rem; }

/* respect the OS / browser reduced-motion preference */
@media (prefers-reduced-motion: reduce) {
    .mcat-dash *, .mcat-dash *::before, .mcat-dash *::after {
        animation-duration: 0.001ms !important;
        animation-iteration-count: 1 !important;
        animation-delay: 0s !important;
        transition-duration: 0.001ms !important;
    }
    .mc-skeleton { animation: none !important; background: var(--mc-track); }
}

@media (max-width: 820px) {
    .mc-hero-card { flex-direction: column; text-align: center; }
    .mc-hero-range { margin: 0 auto; }
    .mc-sec-grid { grid-template-columns: repeat(2, 1fr); }
    .mc-map-stats { width: 100%; margin-left: 0; }
}
@media (max-width: 540px) {
    .mc-mini-row { grid-template-columns: 1fr; }
}
""".replace("__RING_C__", f"{_RING_C:.2f}")
