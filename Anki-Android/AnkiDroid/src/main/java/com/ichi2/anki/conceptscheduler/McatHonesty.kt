/*
 *  Copyright (c) 2026 AnkiDroid Open Source Team
 *
 *  This program is free software; you can redistribute it and/or modify it under
 *  the terms of the GNU General Public License as published by the Free Software
 *  Foundation; either version 3 of the License, or (at your option) any later
 *  version.
 *
 *  This program is distributed in the hope that it will be useful, but WITHOUT ANY
 *  WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A
 *  PARTICULAR PURPOSE. See the GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License along with
 *  this program.  If not, see <http://www.gnu.org/licenses/>.
 */
package com.ichi2.anki.conceptscheduler

import anki.scheduler.ConceptSchedulerStatusResponse
import anki.scheduler.ConceptSectionScore
import anki.scheduler.McatSection
import kotlin.math.roundToInt
import kotlin.math.sqrt

/**
 * Coverage below which MCAT section score ranges are hidden (matches the backend/desktop give-up gate).
 * Above this fraction the estimate is shown; below it the UI abstains rather than print a number.
 */
const val MIN_COVERAGE_TO_SHOW_SCORES = 0.60f

private const val EN_DASH = "\u2013"

/** The projected-total scale (whole MCAT). */
private const val TOTAL_MIN = 472f
private const val TOTAL_MAX = 528f

/** Which of the three separately-reported scores a tile shows. Never blended into one number. */
enum class ScoreKind { MEMORY, PERFORMANCE, READINESS }

/**
 * A single honest score tile. Carries every rubric-required honesty field so the UI can show estimate ·
 * range · % covered · how-sure · last-updated · top reason · what's-missing — or, below the give-up
 * line ([available] = false), the [abstain] message instead of a number.
 */
data class ScoreTileModel(
    val kind: ScoreKind,
    val title: String,
    val available: Boolean,
    val estimate: String,
    val range: String,
    val fraction: Float,
    val coveragePct: Int,
    val howSure: String,
    val topReason: String,
    val whatsMissing: String,
    val abstain: String,
    val scaleCaption: String,
    // One-line plain-language gloss under the title ("if you tested today" vs "on your exam day") so the
    // performance-vs-readiness distinction is legible at a glance.
    val subtitle: String = "",
    // For readiness only: the concrete "peak today -> exam day" counterfactual; empty otherwise.
    val compare: String = "",
    // Baseline copy, enriched with live progress; falls back to the static [BASELINE_READINESS_LABEL].
    val baselineNote: String = BASELINE_READINESS_LABEL,
    // True when [available] but the number is still the prior/baseline (see [isBaselineReadiness]).
    val isBaseline: Boolean,
)

/**
 * A friendly readiness band for a 472–528 total, matching the desktop `_score_band` copy exactly so the
 * hero's word ("Competitive", "On track", …) reads identically on both platforms.
 */
fun scoreBandLabel(total: Float): String =
    when {
        total >= 520f -> "Elite"
        total >= 515f -> "Highly competitive"
        total >= 511f -> "Competitive"
        total >= 506f -> "On track"
        total >= 500f -> "Building"
        else -> "Foundation"
    }

/** Short MCAT section label, matching the desktop legend copy exactly ("Bio/Biochem", "Chem/Phys", …). */
fun mcatSectionShortLabel(section: McatSection): String =
    when (section) {
        McatSection.MCAT_SECTION_BIO_BIOCHEM -> "Bio/Biochem"
        McatSection.MCAT_SECTION_CHEM_PHYS -> "Chem/Phys"
        McatSection.MCAT_SECTION_PSYCH_SOC -> "Psych/Soc"
        McatSection.MCAT_SECTION_CARS -> "CARS"
        else -> "Section"
    }

/**
 * The give-up wording shown when a score is below the evidence line. Identical phrasing on both
 * platforms (UI-SPEC "Honesty"): "Not enough data yet — need N reviews / X% coverage".
 */
fun abstainMessage(
    reviewsNeeded: Int,
    coveragePct: Int,
): String = "Not enough data yet \u2014 need $reviewsNeeded reviews / $coveragePct% coverage"

/**
 * The label shown when a score is still the model's prior/baseline rather than earned from evidence
 * (`has_projection` true but not yet evidence-backed). Exact shared copy (UI-SPEC "Honesty"); the two
 * numbers are the backend evidence gate (≥60% coverage and ≥20 problems per section). A baseline number
 * must always carry this label — never presented as if it were earned.
 */
const val BASELINE_READINESS_LABEL = "Using baseline readiness \u2014 need 60% coverage and at least 20 problems"

/**
 * The baseline label enriched with live progress: what the gate needs (60% coverage + N problems per
 * section) *and* how far the learner already is (questions answered, coverage, sections ready). Mirrors
 * the desktop `_baseline_note`. Falls back to [BASELINE_READINESS_LABEL] when no sections are scored.
 */
fun baselineReadinessLabel(status: ConceptSchedulerStatusResponse): String {
    val sections = status.sectionScoresList
    if (sections.isEmpty()) return BASELINE_READINESS_LABEL
    val answered = sections.sumOf { it.answeredItems }
    val required = (sections.maxOfOrNull { it.requiredItems } ?: 0).let { if (it > 0) it else 20 }
    val coveragePct = (overallCoverage(sections) * 100).roundToInt()
    val ready = sections.count { it.enoughEvidence }
    return "Using baseline readiness \u2014 you've answered $answered questions so far " +
        "($coveragePct% covered, $ready/${sections.size} sections ready). " +
        "Need 60% coverage and at least $required questions per section."
}

/**
 * True when the projection is fully evidence-backed — every section clears the backend evidence gate
 * (`enough_evidence` = ≥20 answered problems and ≥60% coverage).
 */
fun isEvidenceBacked(status: ConceptSchedulerStatusResponse): Boolean =
    status.sectionScoresList.isNotEmpty() && status.sectionScoresList.all { it.enoughEvidence }

/**
 * True when a projected number is shown but it is still the prior/baseline: `has_projection` is true so
 * a number exists, yet the evidence gate is not met ([isEvidenceBacked] false). Such scores must be
 * labeled with [BASELINE_READINESS_LABEL].
 */
fun isBaselineReadiness(status: ConceptSchedulerStatusResponse): Boolean = status.hasProjection && !isEvidenceBacked(status)

/**
 * A plain-language "how sure" word driven purely by how much of the exam is covered, so a thin evidence
 * base can never read as "high". Mirrors the desktop's confidence phrasing.
 */
fun howSureLabel(coverage: Float): String =
    when {
        coverage < 0.4f -> "Low confidence"
        coverage < 0.7f -> "Building confidence"
        else -> "Strong confidence"
    }

/**
 * Freshness of the last live status fetch, e.g. "Updated just now" / "Updated 3m ago". The status is
 * recomputed on every fetch, so this honestly reports data age rather than an invented timestamp.
 */
fun lastUpdatedText(elapsedMillis: Long): String {
    val minutes = elapsedMillis / 60_000L
    return when {
        minutes <= 0L -> "Updated just now"
        minutes == 1L -> "Updated 1m ago"
        else -> "Updated ${minutes}m ago"
    }
}

/** Mean section coverage (0..1), used for the overall "% of exam covered" and the how-sure label. */
fun overallCoverage(sections: List<ConceptSectionScore>): Float =
    if (sections.isEmpty()) 0f else sections.map { it.coverage }.average().toFloat()

/**
 * A one-line "top reason" for a score: the strongest section by [valueOf] and the section whose thin
 * coverage is holding the estimate back. Honest and derived entirely from the per-section fields.
 */
private fun topReasonFor(
    sections: List<ConceptSectionScore>,
    valueOf: (ConceptSectionScore) -> Float,
): String {
    val strongest = sections.maxByOrNull(valueOf)
    val thinnest = sections.minByOrNull { it.coverage }
    if (strongest == null || thinnest == null) return "Answer a few cards to see what's driving this"
    return "Driven by ${mcatSectionShortLabel(strongest.section)} \u00b7 thinnest coverage in ${mcatSectionShortLabel(thinnest.section)}"
}

/**
 * The "what's missing" line: high-value sections still under the coverage line, worst first, e.g.
 * "Below 60% coverage: CARS 20%, Psych/Soc 45%". When everything clears the bar, says so.
 */
fun whatsMissingText(
    sections: List<ConceptSectionScore>,
    threshold: Float = MIN_COVERAGE_TO_SHOW_SCORES,
): String {
    val thresholdPct = (threshold * 100).roundToInt()
    val low = sections.filter { it.coverage < threshold }.sortedBy { it.coverage }
    if (low.isEmpty()) {
        return if (sections.isEmpty()) "No sections scored yet" else "All sections above $thresholdPct% coverage"
    }
    val list = low.joinToString(", ") { "${mcatSectionShortLabel(it.section)} ${(it.coverage * 100).roundToInt()}%" }
    return "Below $thresholdPct% coverage: $list"
}

/** Reviews still needed to clear the overall evidence bar (0 when already met or unknown). */
private fun reviewsNeeded(status: ConceptSchedulerStatusResponse): Int =
    if (status.hasEvidence()) {
        (status.evidence.requiredSeenCards - status.evidence.seenCards).coerceAtLeast(0)
    } else {
        0
    }

/**
 * Builds the three separately-reported score tiles (Memory / Performance / Readiness) from [status],
 * each carrying the full set of honesty fields. Uses only existing response fields — no backend change.
 *
 * - Memory: [ConceptSchedulerStatusResponse.overallMemory] recall of studied cards; range = the spread
 *   of per-section recall.
 * - Performance: the demonstrated IRT total = sum of the four sections' `performance_center`, with a
 *   combined confidence band (sqrt of summed variances) — the same construction the backend uses for
 *   the readiness projection, applied to the performance sub-score.
 * - Readiness: the backend's projected MCAT total (472–528) with its confidence band.
 */
fun buildScoreTiles(status: ConceptSchedulerStatusResponse): List<ScoreTileModel> {
    val sections = status.sectionScoresList
    val coverage = overallCoverage(sections)
    val coveragePct = (coverage * 100).roundToInt()
    val howSure = howSureLabel(coverage)
    val whatsMissing = whatsMissingText(sections)
    val abstain = abstainMessage(reviewsNeeded(status), (MIN_COVERAGE_TO_SHOW_SCORES * 100).roundToInt())
    // A projected number that isn't yet evidence-backed is the prior/baseline; the IRT scores (readiness,
    // performance) must be flagged so a baseline is never shown as earned. Memory is measured recall, not
    // a prior, so it has no baseline state.
    val baseline = isBaselineReadiness(status)

    // ---- Memory ----
    val memPct = (status.overallMemory * 100).roundToInt()
    val sectionMems = sections.filter { it.sectionHasMemory }.map { it.sectionMemory }
    val memoryRange =
        if (sectionMems.size >= 2) {
            "${(sectionMems.min() * 100).roundToInt()}$EN_DASH${(sectionMems.max() * 100).roundToInt()}%"
        } else {
            EN_DASH
        }
    val memory =
        ScoreTileModel(
            kind = ScoreKind.MEMORY,
            title = "Memory",
            available = status.hasMemory,
            estimate = "$memPct%",
            range = memoryRange,
            fraction = status.overallMemory.coerceIn(0f, 1f),
            coveragePct = coveragePct,
            howSure = howSure,
            topReason = topReasonFor(sections) { it.sectionMemory },
            whatsMissing = whatsMissing,
            abstain = abstain,
            scaleCaption = "avg recall of studied cards",
            subtitle = "recall right now",
            isBaseline = false,
        )

    // ---- Performance (demonstrated IRT total) ----
    val perfCenter = sections.sumOf { it.performanceCenter.toDouble() }.toFloat()
    val perfSe = sqrt(sections.sumOf { it.performanceStandardError.toDouble() * it.performanceStandardError }).toFloat()
    val perfLo = (perfCenter - 1.96f * perfSe).coerceIn(TOTAL_MIN, TOTAL_MAX)
    val perfHi = (perfCenter + 1.96f * perfSe).coerceIn(TOTAL_MIN, TOTAL_MAX)
    val performance =
        ScoreTileModel(
            kind = ScoreKind.PERFORMANCE,
            title = "Performance",
            available = status.hasProjection && sections.isNotEmpty(),
            estimate = perfCenter.roundToInt().toString(),
            range = "${perfLo.roundToInt()}$EN_DASH${perfHi.roundToInt()}",
            fraction = fractionOnScale(perfCenter, TOTAL_MIN, TOTAL_MAX),
            coveragePct = coveragePct,
            howSure = howSure,
            topReason = topReasonFor(sections) { it.performanceCenter },
            whatsMissing = whatsMissing,
            abstain = abstain,
            scaleCaption = "your ceiling if tested today \u00b7 ${TOTAL_MIN.roundToInt()}$EN_DASH${TOTAL_MAX.roundToInt()}",
            subtitle = "if you tested today",
            isBaseline = baseline,
        )

    // ---- Readiness (projected total) ----
    val rLo = if (status.projectedTotalLower > 0f) status.projectedTotalLower else status.projectedTotal
    val rHi = if (status.projectedTotalUpper > 0f) status.projectedTotalUpper else status.projectedTotal
    val readiness =
        ScoreTileModel(
            kind = ScoreKind.READINESS,
            title = "Readiness",
            available = status.hasProjection && status.projectedTotal > 0f,
            estimate = status.projectedTotal.roundToInt().toString(),
            range = "${rLo.roundToInt()}$EN_DASH${rHi.roundToInt()}",
            fraction = fractionOnScale(status.projectedTotal, TOTAL_MIN, TOTAL_MAX),
            coveragePct = coveragePct,
            howSure = howSure,
            topReason = topReasonFor(sections) { it.readinessCenter },
            whatsMissing = whatsMissing,
            abstain = abstain,
            scaleCaption = "projected to your exam day \u00b7 ${TOTAL_MIN.roundToInt()}$EN_DASH${TOTAL_MAX.roundToInt()}",
            subtitle = "on your exam day",
            compare =
                if (perfCenter > 0f && status.projectedTotal > 0f) {
                    "If tested today ${perfCenter.roundToInt()} \u2192 projected exam day " +
                        "${status.projectedTotal.roundToInt()} \u2014 the gap is what expected forgetting " +
                        "costs; keep reviews up to close it"
                } else {
                    ""
                },
            baselineNote = baselineReadinessLabel(status),
            isBaseline = baseline,
        )

    return listOf(memory, performance, readiness)
}
