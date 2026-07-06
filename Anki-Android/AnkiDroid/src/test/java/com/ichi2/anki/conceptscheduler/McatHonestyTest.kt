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

import anki.scheduler.ConceptEvidenceStatus
import anki.scheduler.McatSection
import anki.scheduler.conceptEvidenceStatus
import anki.scheduler.conceptSchedulerStatusResponse
import anki.scheduler.conceptSectionScore
import org.junit.Test
import kotlin.test.assertEquals
import kotlin.test.assertFalse
import kotlin.test.assertTrue

class McatHonestyTest {
    @Test
    fun `abstain wording matches the shared spec exactly`() {
        // Identical phrasing on both platforms (UI-SPEC "Honesty"): em dash, "reviews / X% coverage".
        assertEquals(
            "Not enough data yet \u2014 need 190 reviews / 60% coverage",
            abstainMessage(reviewsNeeded = 190, coveragePct = 60),
        )
    }

    @Test
    fun `how sure is driven by coverage, never reading high on thin evidence`() {
        assertEquals("Low confidence", howSureLabel(0.2f))
        assertEquals("Building confidence", howSureLabel(0.5f))
        assertEquals("Strong confidence", howSureLabel(0.8f))
    }

    @Test
    fun `last updated reports data freshness`() {
        assertEquals("Updated just now", lastUpdatedText(5_000))
        assertEquals("Updated 1m ago", lastUpdatedText(60_000))
        assertEquals("Updated 3m ago", lastUpdatedText(200_000))
    }

    @Test
    fun `score band mirrors the desktop copy`() {
        assertEquals("Elite", scoreBandLabel(521f))
        assertEquals("Competitive", scoreBandLabel(512f))
        assertEquals("On track", scoreBandLabel(507f))
        assertEquals("Foundation", scoreBandLabel(490f))
    }

    @Test
    fun `section labels match the desktop legend`() {
        assertEquals("Bio/Biochem", mcatSectionShortLabel(McatSection.MCAT_SECTION_BIO_BIOCHEM))
        assertEquals("Chem/Phys", mcatSectionShortLabel(McatSection.MCAT_SECTION_CHEM_PHYS))
        assertEquals("Psych/Soc", mcatSectionShortLabel(McatSection.MCAT_SECTION_PSYCH_SOC))
        assertEquals("CARS", mcatSectionShortLabel(McatSection.MCAT_SECTION_CARS))
    }

    @Test
    fun `whats missing lists only sections below the coverage line, worst first`() {
        val sections =
            listOf(
                sectionScore(McatSection.MCAT_SECTION_BIO_BIOCHEM, coverage = 0.8f),
                sectionScore(McatSection.MCAT_SECTION_CARS, coverage = 0.5f),
                sectionScore(McatSection.MCAT_SECTION_PSYCH_SOC, coverage = 0.3f),
            )
        assertEquals("Below 60% coverage: Psych/Soc 30%, CARS 50%", whatsMissingText(sections))
    }

    @Test
    fun `three tiles are memory, performance and readiness in order`() {
        val tiles = buildScoreTiles(fullStatus())
        assertEquals(listOf(ScoreKind.MEMORY, ScoreKind.PERFORMANCE, ScoreKind.READINESS), tiles.map { it.kind })
    }

    @Test
    fun `readiness tile shows the projected total with its range`() {
        val readiness = buildScoreTiles(fullStatus()).first { it.kind == ScoreKind.READINESS }
        assertTrue(readiness.available)
        assertEquals("505", readiness.estimate)
        assertEquals("498\u2013512", readiness.range)
        assertEquals(65, readiness.coveragePct)
    }

    @Test
    fun `memory tile shows recall percent and the per-section spread`() {
        val memory = buildScoreTiles(fullStatus()).first { it.kind == ScoreKind.MEMORY }
        assertTrue(memory.available)
        assertEquals("72%", memory.estimate)
        assertEquals("60\u201380%", memory.range)
    }

    @Test
    fun `performance tile sums the section IRT estimates with a combined band`() {
        val performance = buildScoreTiles(fullStatus()).first { it.kind == ScoreKind.PERFORMANCE }
        assertTrue(performance.available)
        // 4 sections × 127 center = 508; combined SE = sqrt(4 × 2^2) = 4 → 508 ± 1.96×4 => 500–516.
        assertEquals("508", performance.estimate)
        assertEquals("500\u2013516", performance.range)
    }

    @Test
    fun `top reason names the strongest section and the thinnest coverage`() {
        val readiness = buildScoreTiles(fullStatus()).first { it.kind == ScoreKind.READINESS }
        assertTrue(readiness.topReason.contains("Bio/Biochem"), readiness.topReason)
        assertTrue(readiness.topReason.contains("CARS"), readiness.topReason)
    }

    @Test
    fun `below the evidence line every score abstains with the exact wording`() {
        val tiles = buildScoreTiles(emptyStatus())
        assertFalse(tiles.first { it.kind == ScoreKind.READINESS }.available)
        assertFalse(tiles.first { it.kind == ScoreKind.PERFORMANCE }.available)
        assertFalse(tiles.first { it.kind == ScoreKind.MEMORY }.available)
        assertEquals(
            "Not enough data yet \u2014 need 190 reviews / 60% coverage",
            tiles.first { it.kind == ScoreKind.READINESS }.abstain,
        )
    }

    @Test
    fun `baseline readiness label matches the shared spec exactly`() {
        assertEquals(
            "Using baseline readiness \u2014 need 60% coverage and at least 20 problems",
            BASELINE_READINESS_LABEL,
        )
    }

    @Test
    fun `baseline state flags a shown-but-not-earned projection`() {
        val tiles = buildScoreTiles(baselineStatus())
        val readiness = tiles.first { it.kind == ScoreKind.READINESS }
        assertTrue(readiness.available, "a baseline number is still shown")
        assertTrue(readiness.isBaseline, "but it must be flagged baseline")
        assertTrue(tiles.first { it.kind == ScoreKind.PERFORMANCE }.isBaseline)
    }

    @Test
    fun `an evidence-backed projection is not baseline`() {
        val readiness = buildScoreTiles(fullStatus()).first { it.kind == ScoreKind.READINESS }
        assertTrue(readiness.available)
        assertFalse(readiness.isBaseline)
    }

    // -- fixtures ----------------------------------------------------------

    private fun sectionScore(
        sec: McatSection,
        coverage: Float,
        performanceCenterValue: Float = 125f,
        readinessCenterValue: Float = 125f,
        readinessLowerValue: Float = 123f,
        readinessUpperValue: Float = 127f,
        memory: Float? = null,
        enough: Boolean = true,
    ) = conceptSectionScore {
        section = sec
        enoughEvidence = enough
        this.coverage = coverage
        performanceCenter = performanceCenterValue
        performanceStandardError = 2f
        readinessCenter = readinessCenterValue
        readinessLower = readinessLowerValue
        readinessUpper = readinessUpperValue
        if (memory != null) {
            sectionMemory = memory
            sectionHasMemory = true
        }
    }

    /** A fully-populated status: projection + memory + four evidenced sections (avg coverage 65%). */
    private fun fullStatus() =
        conceptSchedulerStatusResponse {
            enabled = true
            hasProjection = true
            projectedTotal = 505f
            projectedTotalLower = 498f
            projectedTotalUpper = 512f
            hasMemory = true
            overallMemory = 0.72f
            evidence =
                conceptEvidenceStatus {
                    seenCards = 120
                    requiredSeenCards = 200
                    kind = ConceptEvidenceStatus.Kind.ENOUGH
                }
            sectionScores.add(
                sectionScore(
                    McatSection.MCAT_SECTION_BIO_BIOCHEM,
                    coverage = 0.8f,
                    performanceCenterValue = 127f,
                    readinessCenterValue = 128f,
                    memory = 0.8f,
                ),
            )
            sectionScores.add(
                sectionScore(
                    McatSection.MCAT_SECTION_CHEM_PHYS,
                    coverage = 0.7f,
                    performanceCenterValue = 127f,
                    readinessCenterValue = 120f,
                    memory = 0.6f,
                ),
            )
            sectionScores.add(
                sectionScore(
                    McatSection.MCAT_SECTION_PSYCH_SOC,
                    coverage = 0.6f,
                    performanceCenterValue = 127f,
                    readinessCenterValue = 122f,
                ),
            )
            sectionScores.add(
                sectionScore(
                    McatSection.MCAT_SECTION_CARS,
                    coverage = 0.5f,
                    performanceCenterValue = 127f,
                    readinessCenterValue = 119f,
                ),
            )
        }

    /** A projection exists, but no section clears the evidence gate — the number is baseline/prior. */
    private fun baselineStatus() =
        conceptSchedulerStatusResponse {
            enabled = true
            hasProjection = true
            projectedTotal = 500f
            projectedTotalLower = 480f
            projectedTotalUpper = 520f
            hasMemory = false
            sectionScores.add(sectionScore(McatSection.MCAT_SECTION_BIO_BIOCHEM, coverage = 0.3f, enough = false))
            sectionScores.add(sectionScore(McatSection.MCAT_SECTION_CHEM_PHYS, coverage = 0.3f, enough = false))
            sectionScores.add(sectionScore(McatSection.MCAT_SECTION_PSYCH_SOC, coverage = 0.3f, enough = false))
            sectionScores.add(sectionScore(McatSection.MCAT_SECTION_CARS, coverage = 0.3f, enough = false))
        }

    /** No projection, no memory, thin evidence: everything must abstain. */
    private fun emptyStatus() =
        conceptSchedulerStatusResponse {
            enabled = true
            hasProjection = false
            hasMemory = false
            evidence =
                conceptEvidenceStatus {
                    seenCards = 10
                    requiredSeenCards = 200
                    kind = ConceptEvidenceStatus.Kind.INSUFFICIENT
                }
        }
}
