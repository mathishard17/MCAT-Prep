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

import androidx.compose.animation.AnimatedVisibility
import androidx.compose.animation.core.tween
import androidx.compose.animation.expandVertically
import androidx.compose.animation.fadeIn
import androidx.compose.animation.fadeOut
import androidx.compose.animation.shrinkVertically
import androidx.compose.foundation.BorderStroke
import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.BoxWithConstraints
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.ColumnScope
import androidx.compose.foundation.layout.FlowRow
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.layout.widthIn
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.Button
import androidx.compose.material3.Card
import androidx.compose.material3.CardDefaults
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.OutlinedButton
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Brush
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.font.FontFamily
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import anki.scheduler.ConceptEvidenceStatus
import anki.scheduler.ConceptFringe
import anki.scheduler.ConceptGraph
import anki.scheduler.ConceptGraphNode
import anki.scheduler.ConceptSchedulerStatusResponse
import anki.scheduler.ConceptSectionScore
import anki.scheduler.McatSection
import kotlinx.coroutines.delay
import java.time.Instant
import java.time.LocalDate
import java.time.ZoneId
import java.time.format.DateTimeFormatter
import java.time.format.FormatStyle
import java.time.temporal.ChronoUnit
import kotlin.math.roundToInt

/**
 * Formats a topic's study-priority score for display. The backend's per-topic `readiness_score` is
 * `prerequisiteMastery × (1 − mastery)` — i.e. how much studying this topic now would help, NOT how
 * ready you are for the test (that is [displayMastery] per topic, and the MCAT section readiness
 * overall). It is high when prerequisites are solid but mastery still has room to grow, and falls to
 * 0 once the topic is mastered. A topic that has never been attempted has no meaningful score (only
 * the backend's default prior), so we show a consistent "Not started".
 */
fun displayPriority(
    answered: Int,
    score: Float,
): String = if (answered == 0) "Not started" else "%.2f".format(score)

/**
 * Formats a topic's mastery for display. Like [displayPriority], a topic with no evidence carries
 * only the backend's default prior (0.5/0.25), which is not real mastery — so we show "Not started"
 * rather than a misleading 0.50. Attempted topics start from measured mastery (0.00 upward).
 */
fun displayMastery(
    answered: Int,
    mastery: Float,
): String = if (answered == 0) "Not started" else "%.2f".format(mastery)

/**
 * State label for a specific topic that considers whether the topic has been attempted, so an
 * outer-fringe topic you've already started reads "In progress" rather than "Next up" — otherwise a
 * topic appears to jump straight from "Next up" to "Mastered", which is confusing. Progression:
 * Locked → Next up → In progress → Mastered.
 */
fun nodeStateLabel(
    fringe: ConceptFringe,
    answered: Int,
): String =
    when (fringe) {
        ConceptFringe.CONCEPT_FRINGE_INNER -> "Mastered"
        ConceptFringe.CONCEPT_FRINGE_OUTER -> if (answered > 0) "In progress" else "Next up"
        ConceptFringe.CONCEPT_FRINGE_LOCKED -> "Locked"
        else -> "Unknown"
    }

/**
 * Caption shown under the projected MCAT total, mirroring the desktop reviewer's projection block
 * ("projected MCAT · likely {lo}–{hi} · scale 472–528"). Bounds are rounded to whole points like
 * desktop's `Math.round`, and a missing bound (0) falls back to the point estimate, matching
 * desktop's `projectedTotalLower || projectedTotal` behaviour.
 */
fun projectedTotalMeta(
    total: Float,
    lower: Float,
    upper: Float,
): String {
    val lo = (if (lower > 0f) lower else total).roundToInt()
    val hi = (if (upper > 0f) upper else total).roundToInt()
    return "projected MCAT \u00b7 likely $lo\u2013$hi \u00b7 scale 472\u2013528"
}

/** Whole days from today until the exam (negative once it has passed). */
fun examCountdownDays(
    examEpochSec: Long,
    today: LocalDate = LocalDate.now(),
    zone: ZoneId = ZoneId.systemDefault(),
): Long {
    val examDate = Instant.ofEpochSecond(examEpochSec).atZone(zone).toLocalDate()
    return ChronoUnit.DAYS.between(today, examDate)
}

/** Localized medium date (e.g. `Aug 16, 2026`) for an epoch-seconds exam timestamp. */
private fun formatExamDate(
    examEpochSec: Long,
    zone: ZoneId = ZoneId.systemDefault(),
): String =
    Instant
        .ofEpochSecond(examEpochSec)
        .atZone(zone)
        .toLocalDate()
        .format(DateTimeFormatter.ofLocalizedDate(FormatStyle.MEDIUM))

/**
 * The premium "Readiness Dashboard" for the Concept Scheduler read model, rendered both in the
 * reviewer bottom sheet and the deck-picker full page. Callers must wrap this in `AnkiDroidTheme`.
 *
 * The information architecture mirrors the desktop app (`anki/qt/aqt/mcat_ui.py`) one-for-one, so a
 * reviewer flipping between the two feels a single product: brand topline → hero projected-score gauge
 * (with confidence range, how-sure and freshness) → exam countdown + on-target odds → the three
 * separately-reported scores (memory / performance / readiness) → four color-coded section cards →
 * knowledge map → "Continue studying". Colors come from the shared [McatPalette] (canonical desktop
 * hexes, light + dark) and every score carries its honesty fields with an abstain state below the
 * evidence line.
 *
 * Values come from [status] (the backend response); this screen never computes scheduler state. The
 * only writes are user-driven and delegated to the host via [onSelectTopic] (choose the next topic to
 * study), [onStudySection] (focus study on one MCAT section — the section picker), [onOpenLesson]
 * (open a concept's lesson) and [onContinueStudying] (return to studying) — all no-ops by default so
 * read-only hosts can omit them.
 */
@Composable
fun ConceptSchedulerStatusScreen(
    status: ConceptSchedulerStatusResponse,
    modifier: Modifier = Modifier,
    onSelectTopic: (String) -> Unit = {},
    onStudySection: (McatSection) -> Unit = {},
    onOpenLesson: (String) -> Unit = {},
    onContinueStudying: () -> Unit = {},
) {
    McatDashboardTheme {
        val scheme = MaterialTheme.colorScheme
        val palette = LocalMcatPalette.current
        // Data age is measured from when this status object arrived (a new fetch => a new instance).
        val loadedAt = remember(status) { System.currentTimeMillis() }
        LazyColumn(
            modifier =
                modifier
                    .fillMaxWidth()
                    .background(
                        Brush.verticalGradient(listOf(palette.brandSoft, scheme.background)),
                    ),
            contentPadding = PaddingValues(horizontal = 16.dp, vertical = 20.dp),
            verticalArrangement = Arrangement.spacedBy(14.dp),
        ) {
            item { BrandTopline() }

            if (!status.enabled && !status.hasGraph()) {
                item { DisabledCard() }
                return@LazyColumn
            }

            item { Box(Modifier.staggeredEntrance(0)) { ProjectionHeroCard(status, loadedAt, onContinueStudying) } }

            item { Box(Modifier.staggeredEntrance(1)) { ExamTargetRow(status) } }

            if (status.sectionScoresCount > 0) {
                item { Box(Modifier.staggeredEntrance(2)) { ThreeScoresSection(status, loadedAt) } }
                item {
                    Box(Modifier.staggeredEntrance(3)) {
                        SectionReadinessSection(status.sectionScoresList, status.hasProjection, onStudySection)
                    }
                }
            }

            if (status.hasGraph()) {
                val startable = status.graph.nodesList.filter { it.fringe == ConceptFringe.CONCEPT_FRINGE_OUTER }
                val recommended = startable.filter { it.recommended }
                val nextTopics = (if (recommended.isNotEmpty()) recommended else startable).take(6)
                if (nextTopics.isNotEmpty()) {
                    item { NextTopicsCard(nextTopics, anyRecommended = recommended.isNotEmpty(), onSelectTopic = onSelectTopic) }
                }
                item { KnowledgeMapCard(status, onSelectTopic, onOpenLesson) }
                item { TopicsCard(status) }
            }

            if (status.hasSession() || status.hasEvidence() || status.hasCounters()) {
                item { StudySessionCard(status) }
            }

            item { ContinueStudyingCta(onContinueStudying, Modifier.fillMaxWidth()) }
        }
    }
}

/**
 * Skeleton shown while the status loads: shimmering placeholders shaped like the dashboard (topline,
 * hero, mini row, three tiles, a section block) instead of a hard spinner. Wrapped in
 * [McatDashboardTheme] so the shimmer honours the reduced-motion preference. Public so both hosts
 * (reviewer sheet + deck-picker page) share it.
 */
@Composable
fun DashboardSkeleton(modifier: Modifier = Modifier) {
    McatDashboardTheme {
        Column(
            modifier
                .fillMaxWidth()
                .padding(horizontal = 16.dp, vertical = 20.dp),
            verticalArrangement = Arrangement.spacedBy(14.dp),
        ) {
            ShimmerBox(Modifier.fillMaxWidth().height(44.dp))
            ShimmerBox(Modifier.fillMaxWidth().height(300.dp), RoundedCornerShape(26.dp))
            Row(horizontalArrangement = Arrangement.spacedBy(12.dp)) {
                ShimmerBox(Modifier.weight(1f).height(96.dp), RoundedCornerShape(18.dp))
                ShimmerBox(Modifier.weight(1f).height(96.dp), RoundedCornerShape(18.dp))
            }
            Row(horizontalArrangement = Arrangement.spacedBy(10.dp)) {
                repeat(3) { ShimmerBox(Modifier.weight(1f).height(96.dp), RoundedCornerShape(16.dp)) }
            }
            ShimmerBox(Modifier.fillMaxWidth().height(200.dp), RoundedCornerShape(20.dp))
        }
    }
}

// ---- brand topline ----

@Composable
private fun BrandTopline() {
    val palette = LocalMcatPalette.current
    Row(verticalAlignment = Alignment.CenterVertically) {
        Box(
            Modifier
                .size(38.dp)
                .clip(RoundedCornerShape(11.dp))
                .background(Brush.linearGradient(listOf(palette.brand, palette.brandStrong))),
            contentAlignment = Alignment.Center,
        ) {
            Text("M", color = Color.White, fontWeight = FontWeight.ExtraBold, style = MaterialTheme.typography.titleMedium)
        }
        Spacer(Modifier.width(10.dp))
        Column(Modifier.weight(1f)) {
            Text("MCAT Prep", style = MaterialTheme.typography.titleMedium, fontWeight = FontWeight.Bold)
            Text(
                "Readiness dashboard",
                style = MaterialTheme.typography.labelSmall,
                color = MaterialTheme.colorScheme.onSurfaceVariant,
            )
        }
    }
}

@Composable
private fun DisabledCard() {
    PremiumCard {
        Text(
            "Concept Scheduler is off for this deck",
            style = MaterialTheme.typography.titleMedium,
            fontWeight = FontWeight.SemiBold,
        )
        Spacer(Modifier.height(4.dp))
        Text(
            "Enable Concept Scheduler Mode in this deck's options to see your projected MCAT score, " +
                "section readiness, and knowledge map.",
            style = MaterialTheme.typography.bodyMedium,
            color = MaterialTheme.colorScheme.onSurfaceVariant,
        )
    }
}

// ---- hero: projected score ----

@Composable
private fun ProjectionHeroCard(
    status: ConceptSchedulerStatusResponse,
    loadedAt: Long,
    onContinueStudying: () -> Unit,
) {
    val palette = LocalMcatPalette.current
    val hasProjection = status.hasProjection && status.projectedTotal > 0f
    val fraction = fractionOnScale(status.projectedTotal, McatScale.TOTAL_MIN, McatScale.TOTAL_MAX)
    val accent = if (hasProjection) readinessColor(fraction) else palette.brand
    val avgCoverage = overallCoverage(status.sectionScoresList)

    HeroCard(accent = accent) {
        Row(Modifier.fillMaxWidth(), verticalAlignment = Alignment.CenterVertically) {
            Text(
                "MCAT READINESS",
                style = MaterialTheme.typography.labelMedium,
                letterSpacing = 1.5.sp,
                fontWeight = FontWeight.Bold,
                color = palette.brand,
                modifier = Modifier.weight(1f),
            )
            Text(
                updatedAgoText(loadedAt),
                style = MaterialTheme.typography.labelSmall,
                color = MaterialTheme.colorScheme.onSurfaceVariant,
            )
        }
        Spacer(Modifier.height(4.dp))
        Text(
            "Your projected score",
            style = MaterialTheme.typography.titleLarge,
            fontWeight = FontWeight.Bold,
        )
        Spacer(Modifier.height(12.dp))

        if (hasProjection) {
            val lower = if (status.projectedTotalLower > 0f) status.projectedTotalLower else status.projectedTotal
            val upper = if (status.projectedTotalUpper > 0f) status.projectedTotalUpper else status.projectedTotal
            val target = status.projectedTotal.roundToInt()
            ReadinessGauge(
                fraction = fraction,
                progressColor = accent,
                trackColor = MaterialTheme.colorScheme.surfaceVariant,
                bandStart = fractionOnScale(lower, McatScale.TOTAL_MIN, McatScale.TOTAL_MAX),
                bandEnd = fractionOnScale(upper, McatScale.TOTAL_MIN, McatScale.TOTAL_MAX),
                strokeWidth = 16.dp,
                showTip = true,
                modifier = Modifier.size(220.dp),
            ) {
                Column(horizontalAlignment = Alignment.CenterHorizontally) {
                    Text(
                        "PROJECTED TOTAL",
                        style = MaterialTheme.typography.labelSmall,
                        letterSpacing = 1.2.sp,
                        color = MaterialTheme.colorScheme.onSurfaceVariant,
                    )
                    Text(
                        animatedCountUp(target, startFrom = (target - 24).coerceAtLeast(472)).toString(),
                        style =
                            MaterialTheme.typography.displayMedium
                                .copy(fontWeight = FontWeight.Bold)
                                .tabularFigures(),
                    )
                    Text(
                        scoreBandLabel(status.projectedTotal),
                        style = MaterialTheme.typography.labelMedium,
                        fontWeight = FontWeight.SemiBold,
                        color = palette.brand,
                    )
                }
            }
            Spacer(Modifier.height(14.dp))
            ScoreRangeBar(
                lowerFraction = fractionOnScale(lower, McatScale.TOTAL_MIN, McatScale.TOTAL_MAX),
                upperFraction = fractionOnScale(upper, McatScale.TOTAL_MIN, McatScale.TOTAL_MAX),
                centerFraction = fraction,
                color = accent,
                modifier = Modifier.widthIn(max = 420.dp),
            )
            Spacer(Modifier.height(6.dp))
            Row(
                Modifier.widthIn(max = 420.dp).fillMaxWidth(),
                horizontalArrangement = Arrangement.SpaceBetween,
            ) {
                Text("472", style = MaterialTheme.typography.labelSmall, color = MaterialTheme.colorScheme.onSurfaceVariant)
                Text(
                    "likely ${lower.roundToInt()}\u2013${upper.roundToInt()}",
                    style = MaterialTheme.typography.labelSmall,
                    fontWeight = FontWeight.SemiBold,
                )
                Text("528", style = MaterialTheme.typography.labelSmall, color = MaterialTheme.colorScheme.onSurfaceVariant)
            }
            Spacer(Modifier.height(10.dp))
            // Baseline state: a number is shown but it's still the prior — label it, don't imply it's earned.
            if (isBaselineReadiness(status)) {
                BaselineBanner(baselineReadinessLabel(status), Modifier.widthIn(max = 420.dp))
            } else {
                Row(verticalAlignment = Alignment.CenterVertically) {
                    HowSureChip(howSureLabel(avgCoverage))
                    Spacer(Modifier.width(8.dp))
                    Text(
                        "${(avgCoverage * 100).roundToInt()}% of exam covered",
                        style = MaterialTheme.typography.labelMedium,
                        color = MaterialTheme.colorScheme.onSurfaceVariant,
                    )
                }
            }
        } else {
            AbstainHero(status)
        }

        Spacer(Modifier.height(16.dp))
        ContinueStudyingCta(onContinueStudying)
    }
}

/**
 * Give-up state honoring the evidence rule: with no projection yet we show the spec's exact abstain
 * wording plus progress toward the evidence threshold — never a made-up number.
 */
@Composable
private fun AbstainHero(status: ConceptSchedulerStatusResponse) {
    val need =
        if (status.hasEvidence()) {
            (status.evidence.requiredSeenCards - status.evidence.seenCards).coerceAtLeast(0)
        } else {
            0
        }
    val coveragePct = (MIN_COVERAGE_TO_SHOW_SCORES * 100).roundToInt()
    Column(
        Modifier.fillMaxWidth().padding(vertical = 8.dp),
        horizontalAlignment = Alignment.CenterHorizontally,
        verticalArrangement = Arrangement.spacedBy(8.dp),
    ) {
        Text(
            "\u2013\u2013\u2013",
            style = MaterialTheme.typography.displaySmall,
            fontWeight = FontWeight.Bold,
            color = MaterialTheme.colorScheme.onSurfaceVariant,
        )
        Text(
            abstainMessage(need, coveragePct),
            style = MaterialTheme.typography.titleSmall,
            fontWeight = FontWeight.SemiBold,
            color = MaterialTheme.colorScheme.onSurface,
        )
        if (status.hasEvidence() && status.evidence.requiredSeenCards > 0) {
            val e = status.evidence
            MetricBar(
                fraction = (e.seenCards.toFloat() / e.requiredSeenCards).coerceIn(0f, 1f),
                color = MaterialTheme.colorScheme.primary,
                trackColor = MaterialTheme.colorScheme.surfaceVariant,
                modifier = Modifier.width(240.dp),
                height = 8.dp,
            )
        }
        Text(
            "Answer a few questions and your projected MCAT score (472\u2013528) will appear here with a confidence range.",
            style = MaterialTheme.typography.bodySmall,
            color = MaterialTheme.colorScheme.onSurfaceVariant,
        )
    }
}

/** Refreshing "Updated Xm ago" text; ticks every 30s so freshness stays accurate while open. */
@Composable
private fun updatedAgoText(loadedAtMillis: Long): String {
    var now by remember(loadedAtMillis) { mutableStateOf(System.currentTimeMillis()) }
    LaunchedEffect(loadedAtMillis) {
        while (true) {
            delay(30_000)
            now = System.currentTimeMillis()
        }
    }
    return lastUpdatedText((now - loadedAtMillis).coerceAtLeast(0))
}

// ---- exam countdown + on-target odds ----

@Composable
private fun ExamTargetRow(status: ConceptSchedulerStatusResponse) {
    Row(horizontalArrangement = Arrangement.spacedBy(12.dp)) {
        ExamCountdownCard(status, Modifier.weight(1f))
        TargetOddsCard(status, Modifier.weight(1f))
    }
}

@Composable
private fun ExamCountdownCard(
    status: ConceptSchedulerStatusResponse,
    modifier: Modifier = Modifier,
) {
    MiniCard(modifier) {
        MiniHead("\uD83D\uDCC5", "TEST DAY")
        if (status.examTimestamp > 0L) {
            val days = examCountdownDays(status.examTimestamp)
            val (big, small) =
                when {
                    days > 0L -> days.toString() to (if (days == 1L) "day to go" else "days to go")
                    days == 0L -> "Today" to "exam day"
                    else -> "Done" to "exam passed"
                }
            Row(verticalAlignment = Alignment.Bottom) {
                Text(
                    big,
                    style =
                        MaterialTheme.typography.headlineMedium
                            .copy(fontWeight = FontWeight.Bold)
                            .tabularFigures(),
                )
                Spacer(Modifier.width(4.dp))
                Text(
                    small,
                    style = MaterialTheme.typography.bodySmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                    modifier = Modifier.padding(bottom = 4.dp),
                )
            }
            Text(
                formatExamDate(status.examTimestamp),
                style = MaterialTheme.typography.labelSmall,
                color = MaterialTheme.colorScheme.onSurfaceVariant,
            )
        } else {
            Text(
                "Set your exam date in deck options to project retention to test day.",
                style = MaterialTheme.typography.bodySmall,
                color = MaterialTheme.colorScheme.onSurfaceVariant,
            )
        }
    }
}

@Composable
private fun TargetOddsCard(
    status: ConceptSchedulerStatusResponse,
    modifier: Modifier = Modifier,
) {
    MiniCard(modifier) {
        MiniHead("\uD83C\uDFAF", "TARGET")
        if (status.hasTarget) {
            val prob = status.probabilityHitTarget.coerceIn(0f, 1f)
            val color = readinessColor(prob)
            Row(verticalAlignment = Alignment.CenterVertically) {
                ReadinessGauge(
                    fraction = prob,
                    progressColor = color,
                    trackColor = MaterialTheme.colorScheme.surfaceVariant,
                    strokeWidth = 7.dp,
                    modifier = Modifier.size(58.dp),
                ) {
                    Text(
                        "${(prob * 100).roundToInt()}%",
                        style =
                            MaterialTheme.typography.titleSmall
                                .copy(fontWeight = FontWeight.Bold)
                                .tabularFigures(),
                    )
                }
                Spacer(Modifier.width(10.dp))
                Text(
                    "chance of hitting ${status.targetTotalScore} by test day",
                    style = MaterialTheme.typography.bodySmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                )
            }
        } else {
            Text(
                "Set a target score in deck options to track your odds of hitting it.",
                style = MaterialTheme.typography.bodySmall,
                color = MaterialTheme.colorScheme.onSurfaceVariant,
            )
        }
    }
}

// ---- three scores: memory / performance / readiness ----

@Composable
private fun ThreeScoresSection(
    status: ConceptSchedulerStatusResponse,
    loadedAt: Long,
) {
    val tiles = remember(status) { buildScoreTiles(status) }
    var expanded by remember { mutableStateOf<ScoreKind?>(null) }
    Column(verticalArrangement = Arrangement.spacedBy(10.dp)) {
        SectionHeader("Your three scores", "Memory, performance and readiness — reported separately")
        Row(horizontalArrangement = Arrangement.spacedBy(10.dp)) {
            for (tile in tiles) {
                ScoreTileCompact(
                    tile = tile,
                    selected = expanded == tile.kind,
                    onClick = { expanded = if (expanded == tile.kind) null else tile.kind },
                    modifier = Modifier.weight(1f),
                )
            }
        }
        for (tile in tiles) {
            val reduce = LocalReduceMotion.current
            AnimatedVisibility(
                visible = expanded == tile.kind,
                enter = fadeIn(tween(if (reduce) 0 else 200)) + expandVertically(tween(if (reduce) 0 else 220, easing = EmphasizedEasing)),
                exit = fadeOut(tween(if (reduce) 0 else 160)) + shrinkVertically(tween(if (reduce) 0 else 200, easing = EmphasizedEasing)),
            ) {
                ScoreDetailCard(tile, loadedAt)
            }
        }
    }
}

@Composable
private fun ScoreTileCompact(
    tile: ScoreTileModel,
    selected: Boolean,
    onClick: () -> Unit,
    modifier: Modifier = Modifier,
) {
    val palette = LocalMcatPalette.current
    val accent = scoreAccent(tile.kind, palette)
    Card(
        modifier = modifier.pressable(onClick = onClick),
        shape = RoundedCornerShape(16.dp),
        colors =
            CardDefaults.cardColors(
                containerColor =
                    if (selected) MaterialTheme.colorScheme.surfaceContainerHighest else MaterialTheme.colorScheme.surfaceContainer,
            ),
        border = if (selected) BorderStroke(1.dp, accent) else null,
    ) {
        Column(Modifier.padding(12.dp), verticalArrangement = Arrangement.spacedBy(4.dp)) {
            Text(
                tile.title.uppercase(),
                style = MaterialTheme.typography.labelSmall,
                letterSpacing = 0.8.sp,
                fontWeight = FontWeight.Bold,
                color = accent,
            )
            if (tile.subtitle.isNotEmpty()) {
                Text(
                    tile.subtitle,
                    style = MaterialTheme.typography.labelSmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                )
            }
            if (tile.available) {
                Text(
                    tile.estimate,
                    style =
                        MaterialTheme.typography.headlineSmall
                            .copy(fontWeight = FontWeight.Bold)
                            .tabularFigures(),
                )
                if (tile.isBaseline) {
                    BaselineTag()
                } else {
                    Text(tile.range, style = MaterialTheme.typography.labelSmall, color = MaterialTheme.colorScheme.onSurfaceVariant)
                }
            } else {
                Text(
                    "\u2022\u2022\u2022",
                    style = MaterialTheme.typography.headlineSmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                )
                Text("building", style = MaterialTheme.typography.labelSmall, color = MaterialTheme.colorScheme.onSurfaceVariant)
            }
            MetricBar(
                fraction = if (tile.available) tile.fraction else 0f,
                color = accent,
                trackColor = MaterialTheme.colorScheme.surfaceVariant,
                height = 5.dp,
            )
        }
    }
}

@Composable
private fun ScoreDetailCard(
    tile: ScoreTileModel,
    loadedAt: Long,
) {
    val palette = LocalMcatPalette.current
    val accent = scoreAccent(tile.kind, palette)
    Card(
        shape = RoundedCornerShape(16.dp),
        colors = CardDefaults.cardColors(containerColor = MaterialTheme.colorScheme.surfaceContainer),
    ) {
        Column(Modifier.padding(16.dp), verticalArrangement = Arrangement.spacedBy(4.dp)) {
            Row(verticalAlignment = Alignment.CenterVertically) {
                Box(Modifier.size(9.dp).clip(CircleShape).background(accent))
                Spacer(Modifier.width(8.dp))
                Text(
                    "${tile.title} detail",
                    style = MaterialTheme.typography.titleSmall,
                    fontWeight = FontWeight.SemiBold,
                    modifier = Modifier.weight(1f),
                )
                Text(
                    updatedAgoText(loadedAt),
                    style = MaterialTheme.typography.labelSmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                )
            }
            Spacer(Modifier.height(6.dp))
            if (tile.available) {
                if (tile.isBaseline) {
                    BaselineBanner(tile.baselineNote)
                    Spacer(Modifier.height(8.dp))
                }
                LabeledValue("Estimate", "${tile.estimate}  ·  ${tile.scaleCaption}")
                LabeledValue("Range", tile.range)
                LabeledValue("Covered", "${tile.coveragePct}%")
                Row(Modifier.fillMaxWidth().padding(vertical = 2.dp), verticalAlignment = Alignment.CenterVertically) {
                    Text("How sure", style = MaterialTheme.typography.bodyMedium, modifier = Modifier.weight(1f))
                    HowSureChip(tile.howSure)
                }
                if (tile.compare.isNotEmpty()) {
                    Spacer(Modifier.height(8.dp))
                    CompareNote(tile.compare, accent)
                }
            } else {
                Text(
                    tile.abstain,
                    style = MaterialTheme.typography.bodyMedium,
                    fontWeight = FontWeight.SemiBold,
                    color = MaterialTheme.colorScheme.onSurface,
                )
                Spacer(Modifier.height(6.dp))
                LabeledValue("Covered", "${tile.coveragePct}%")
            }
            Spacer(Modifier.height(6.dp))
            DetailNote("Top reason", tile.topReason)
            DetailNote("What's missing", tile.whatsMissing)
        }
    }
}

@Composable
private fun DetailNote(
    label: String,
    value: String,
) {
    Column(Modifier.padding(top = 4.dp)) {
        Text(label, style = MaterialTheme.typography.labelSmall, color = MaterialTheme.colorScheme.onSurfaceVariant)
        Text(value, style = MaterialTheme.typography.bodySmall)
    }
}

private fun scoreAccent(
    kind: ScoreKind,
    palette: McatPalette,
): Color =
    when (kind) {
        ScoreKind.MEMORY -> palette.bio
        ScoreKind.PERFORMANCE -> palette.cars
        ScoreKind.READINESS -> palette.brand
    }

// ---- section readiness grid ----

@Composable
private fun SectionReadinessSection(
    sections: List<ConceptSectionScore>,
    hasProjection: Boolean,
    onStudySection: (McatSection) -> Unit,
) {
    Column(verticalArrangement = Arrangement.spacedBy(12.dp)) {
        SectionHeader("Section readiness", "Tap a section to focus your study \u00b7 scale 118\u2013132")
        var index = 0
        for (row in sections.chunked(2)) {
            Row(horizontalArrangement = Arrangement.spacedBy(12.dp)) {
                for (score in row) {
                    SectionReadinessCard(score, hasProjection, onStudySection, Modifier.weight(1f).staggeredEntrance(index))
                    index++
                }
                if (row.size == 1) Spacer(Modifier.weight(1f))
            }
        }
    }
}

/**
 * One tappable MCAT-section card — the Compose twin of the desktop `_section_card` and the
 * section picker's tap target. Layout mirrors desktop: section dot + name (+ "building evidence"
 * tag until the section clears its gate), the readiness ring with center estimate·range, the
 * "readiness · scale 118–132" caption, then Coverage and Memory as row+bar pairs. Tapping the card
 * focuses study on this section via [onStudySection].
 */
@Composable
private fun SectionReadinessCard(
    score: ConceptSectionScore,
    hasProjection: Boolean,
    onStudySection: (McatSection) -> Unit,
    modifier: Modifier = Modifier,
) {
    val palette = LocalMcatPalette.current
    val color = palette.colorForSection(score.section)
    // Abstain when there's no projection at all (show no number, matching desktop's "––"); otherwise
    // show the estimate. Sections that haven't cleared their evidence gate carry a "building evidence"
    // tag (desktop `_section_card`) so a not-yet-earned number is never presented as final.
    val showNumber = hasProjection
    val fraction = fractionOnScale(score.readinessCenter, McatScale.SECTION_MIN, McatScale.SECTION_MAX)
    Card(
        modifier =
            modifier
                .fillMaxWidth()
                .clip(RoundedCornerShape(20.dp))
                .pressable(onClick = { onStudySection(score.section) }),
        shape = RoundedCornerShape(20.dp),
        colors = CardDefaults.cardColors(containerColor = MaterialTheme.colorScheme.surfaceContainer),
    ) {
        Column(
            Modifier.fillMaxWidth().padding(14.dp),
            horizontalAlignment = Alignment.CenterHorizontally,
            verticalArrangement = Arrangement.spacedBy(8.dp),
        ) {
            Row(Modifier.fillMaxWidth(), verticalAlignment = Alignment.CenterVertically) {
                Box(Modifier.size(10.dp).clip(CircleShape).background(color))
                Spacer(Modifier.width(6.dp))
                Text(
                    mcatSectionShortLabel(score.section),
                    style = MaterialTheme.typography.labelLarge,
                    fontWeight = FontWeight.SemiBold,
                    modifier = Modifier.weight(1f),
                )
                if (showNumber && !score.enoughEvidence) {
                    BuildingEvidenceTag()
                }
            }
            ReadinessGauge(
                fraction = if (showNumber) fraction else 0f,
                progressColor = color,
                trackColor = MaterialTheme.colorScheme.surfaceVariant,
                bandStart = if (showNumber) fractionOnScale(score.readinessLower, McatScale.SECTION_MIN, McatScale.SECTION_MAX) else null,
                bandEnd = if (showNumber) fractionOnScale(score.readinessUpper, McatScale.SECTION_MIN, McatScale.SECTION_MAX) else null,
                strokeWidth = 9.dp,
                animate = showNumber,
                modifier = Modifier.size(108.dp),
            ) {
                Column(horizontalAlignment = Alignment.CenterHorizontally) {
                    if (!showNumber) {
                        Text(
                            "\u2013\u2013",
                            style = MaterialTheme.typography.titleMedium,
                            color = MaterialTheme.colorScheme.onSurfaceVariant,
                        )
                    } else {
                        Text(
                            score.readinessCenter.roundToInt().toString(),
                            style =
                                MaterialTheme.typography.headlineSmall
                                    .copy(fontWeight = FontWeight.Bold)
                                    .tabularFigures(),
                        )
                        Text(
                            "${score.readinessLower.roundToInt()}\u2013${score.readinessUpper.roundToInt()}",
                            style = MaterialTheme.typography.labelSmall,
                            color = MaterialTheme.colorScheme.onSurfaceVariant,
                        )
                    }
                }
            }
            Text(
                "readiness \u00b7 scale 118\u2013132",
                style = MaterialTheme.typography.labelSmall,
                color = MaterialTheme.colorScheme.onSurfaceVariant,
            )
            SectionMetric("Coverage", "${(score.coverage * 100).roundToInt()}%", score.coverage, color)
            val hasMem = score.sectionHasMemory
            SectionMetric(
                "Memory",
                if (hasMem) "${(score.sectionMemory * 100).roundToInt()}%" else "\u2013",
                if (hasMem) score.sectionMemory else 0f,
                color,
            )
            if (showNumber && !score.enoughEvidence) {
                val needCov = (MIN_COVERAGE_TO_SHOW_SCORES * 100).roundToInt()
                val req = if (score.requiredItems > 0) score.requiredItems else 20
                Text(
                    "Solved ${score.answeredItems}/$req questions \u00b7 " +
                        "${(score.coverage * 100).roundToInt()}%/$needCov% coverage " +
                        "\u00b7 need $needCov% + $req to score it",
                    style = MaterialTheme.typography.labelSmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                )
            }
            Text(
                "Study ${mcatSectionShortLabel(score.section)} \u2192",
                style = MaterialTheme.typography.labelMedium,
                fontWeight = FontWeight.SemiBold,
                color = palette.brand,
                modifier = Modifier.padding(top = 2.dp),
            )
        }
    }
}

/** A labelled metric with an aligned value and a fill bar, shared by the section cards. */
@Composable
private fun SectionMetric(
    label: String,
    value: String,
    fraction: Float,
    color: Color,
) {
    Column(Modifier.fillMaxWidth(), verticalArrangement = Arrangement.spacedBy(3.dp)) {
        Row(Modifier.fillMaxWidth(), verticalAlignment = Alignment.CenterVertically) {
            Text(
                label,
                style = MaterialTheme.typography.labelSmall,
                color = MaterialTheme.colorScheme.onSurfaceVariant,
                modifier = Modifier.weight(1f),
            )
            Text(value, style = MaterialTheme.typography.labelSmall, fontWeight = FontWeight.SemiBold)
        }
        MetricBar(
            fraction = fraction,
            color = color,
            trackColor = MaterialTheme.colorScheme.surfaceVariant,
        )
    }
}

// ---- next topics ----

/**
 * Prominent "what to learn next" picker so choosing the next topic is obvious. Lists [topics] (the
 * recommended startable topics, or all outer-fringe topics when none are flagged) as tappable rows —
 * mirroring the desktop reviewer's `concept-next-topics` block. Tapping selects it via [onSelectTopic];
 * recommended topics keep the ★ marker.
 */
@Composable
private fun NextTopicsCard(
    topics: List<ConceptGraphNode>,
    anyRecommended: Boolean,
    onSelectTopic: (String) -> Unit,
) {
    PremiumCard(containerColor = MaterialTheme.colorScheme.secondaryContainer) {
        Text(
            "Study next",
            style = MaterialTheme.typography.titleMedium,
            fontWeight = FontWeight.SemiBold,
            color = MaterialTheme.colorScheme.onSecondaryContainer,
        )
        Spacer(Modifier.height(2.dp))
        Text(
            if (anyRecommended) {
                "Suggested picks — prerequisites are ready. Tap one to start."
            } else {
                "Ready to start — prerequisites are met. Tap one to begin."
            },
            style = MaterialTheme.typography.bodySmall,
            color = MaterialTheme.colorScheme.onSecondaryContainer,
        )
        Spacer(Modifier.height(12.dp))
        Column(verticalArrangement = Arrangement.spacedBy(8.dp)) {
            for (node in topics) {
                NextTopicRow(node, onSelectTopic)
            }
        }
    }
}

@Composable
private fun NextTopicRow(
    node: ConceptGraphNode,
    onSelectTopic: (String) -> Unit,
) {
    val palette = LocalMcatPalette.current
    Row(
        modifier =
            Modifier
                .fillMaxWidth()
                .clip(RoundedCornerShape(14.dp))
                .background(MaterialTheme.colorScheme.surface)
                .pressable(onClick = { onSelectTopic(node.id) })
                .padding(horizontal = 14.dp, vertical = 12.dp),
        verticalAlignment = Alignment.CenterVertically,
    ) {
        Box(Modifier.size(10.dp).clip(CircleShape).background(palette.colorForKc(node.id)))
        Spacer(Modifier.width(10.dp))
        Text(
            node.id.substringAfterLast("::").replace('_', ' '),
            style = MaterialTheme.typography.bodyLarge,
            modifier = Modifier.weight(1f),
        )
        if (node.recommended) {
            Text("★", color = RecommendedStarColor)
            Spacer(Modifier.width(8.dp))
        }
        Text(
            "Start ▸",
            style = MaterialTheme.typography.labelLarge,
            fontWeight = FontWeight.SemiBold,
            color = MaterialTheme.colorScheme.primary,
        )
    }
}

// ---- knowledge map ----

@Composable
private fun KnowledgeMapCard(
    status: ConceptSchedulerStatusResponse,
    onSelectTopic: (String) -> Unit,
    onOpenLesson: (String) -> Unit,
) {
    val palette = LocalMcatPalette.current
    val colors =
        LatticeColors(
            edge = MaterialTheme.colorScheme.outlineVariant,
            edgeStrong = MaterialTheme.colorScheme.primary,
            label = MaterialTheme.colorScheme.onSurface,
            labelBg = MaterialTheme.colorScheme.surfaceContainerHigh,
            selectedRing = MaterialTheme.colorScheme.primary,
            recommended = RecommendedStarColor,
            frontier = palette.good,
            background = MaterialTheme.colorScheme.surface,
        )
    // Default the detail card to the recommended / first startable concept so it opens populated
    // (mirroring desktop, where the node detail defaults to the current card's concept).
    val initialSelected =
        remember(status) {
            status.graph.nodesList.firstOrNull { it.recommended }?.id
                ?: status.graph.nodesList.firstOrNull { it.fringe == ConceptFringe.CONCEPT_FRINGE_OUTER }?.id
        }
    var selected by remember(status) { mutableStateOf(initialSelected) }
    PremiumCard {
        Text("Knowledge map", style = MaterialTheme.typography.titleMedium, fontWeight = FontWeight.SemiBold)
        Spacer(Modifier.height(8.dp))
        FlowRow(horizontalArrangement = Arrangement.spacedBy(10.dp), verticalArrangement = Arrangement.spacedBy(4.dp)) {
            LegendChip("Bio/Biochem", palette.bio)
            LegendChip("Chem/Phys", palette.chem)
            LegendChip("Psych/Soc", palette.psych)
            LegendChip("CARS", palette.cars)
            LegendChip("Ready to start", palette.good)
        }
        Spacer(Modifier.height(6.dp))
        Text(
            "★ suggested · ✓ mastered · ring = ready · size = importance · tap a dot for details",
            style = MaterialTheme.typography.labelSmall,
            color = MaterialTheme.colorScheme.onSurfaceVariant,
        )
        if (status.graph.hasCycle) {
            Text(
                "⚠ prerequisite cycle detected",
                style = MaterialTheme.typography.labelMedium,
                color = MaterialTheme.colorScheme.error,
            )
        }
        Spacer(Modifier.height(8.dp))
        ConceptLatticeGraph(
            graph = status.graph,
            colors = colors,
            selectedNodeId = selected,
            onNodeSelected = { selected = it },
            modifier = Modifier.fillMaxWidth(),
        )
        val node = status.graph.nodesList.firstOrNull { it.id == selected }
        if (node != null) {
            NodeDetail(node, status.graph, onSelectTopic, onOpenLesson)
        } else {
            Spacer(Modifier.height(4.dp))
            Text(
                "Tap a node for details",
                style = MaterialTheme.typography.labelSmall,
                color = MaterialTheme.colorScheme.onSurfaceVariant,
            )
        }
    }
}

/**
 * The tap-to-describe detail card shown below the knowledge graph, themed to the concept's MCAT
 * section colour (tinted fill + border + dot). Mirrors the desktop sidebar detail panel: name +
 * discipline, status (Mastered / Ready to start / Locked), mastery %, answered/accuracy, a
 * "Builds on … · Unlocks …" role line, and a "Start this topic" button for startable concepts.
 */
@Composable
private fun NodeDetail(
    node: ConceptGraphNode,
    graph: ConceptGraph,
    onSelectTopic: (String) -> Unit,
    onOpenLesson: (String) -> Unit,
) {
    val palette = LocalMcatPalette.current
    val color = palette.colorForKc(node.id)
    val roleLine = remember(graph, node.id) { conceptRoleLine(graph, node.id) }
    Column(
        Modifier
            .fillMaxWidth()
            .padding(top = 10.dp)
            .clip(RoundedCornerShape(14.dp))
            .background(color.copy(alpha = 0.08f))
            .border(BorderStroke(1.5.dp, color.copy(alpha = 0.6f)), RoundedCornerShape(14.dp))
            .padding(14.dp),
    ) {
        Row(verticalAlignment = Alignment.CenterVertically) {
            Box(Modifier.size(10.dp).clip(CircleShape).background(color))
            Spacer(Modifier.width(8.dp))
            Column(Modifier.weight(1f)) {
                Text(
                    node.id.substringAfterLast("::").replace('_', ' '),
                    style = MaterialTheme.typography.titleSmall,
                    fontWeight = FontWeight.SemiBold,
                )
                Text(
                    disciplineLabel(node.id),
                    style = MaterialTheme.typography.labelSmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                )
            }
            StateChipForNode(node.fringe, node.answered)
        }
        Spacer(Modifier.height(10.dp))
        if (node.answered > 0) {
            Row(Modifier.fillMaxWidth(), verticalAlignment = Alignment.CenterVertically) {
                Text("Mastery", style = MaterialTheme.typography.labelMedium, modifier = Modifier.weight(1f))
                Text(
                    displayMastery(node.answered, node.mastery),
                    style = MaterialTheme.typography.labelMedium,
                    fontWeight = FontWeight.SemiBold,
                )
            }
            Spacer(Modifier.height(4.dp))
            MetricBar(
                fraction = node.mastery.coerceIn(0f, 1f),
                color = color,
                trackColor = MaterialTheme.colorScheme.surfaceVariant,
            )
            Spacer(Modifier.height(8.dp))
            LabeledValue("Answered", "${node.answered}  (+${node.positive} / −${node.negative})")
            LabeledValue("Accuracy", accuracyLabel(node.answered, node.positive))
        } else {
            Text(
                "Not started yet",
                style = MaterialTheme.typography.bodyMedium,
                color = MaterialTheme.colorScheme.onSurfaceVariant,
            )
        }
        if (roleLine.isNotBlank()) {
            Spacer(Modifier.height(6.dp))
            Text(
                roleLine,
                style = MaterialTheme.typography.labelMedium,
                color = MaterialTheme.colorScheme.onSurfaceVariant,
            )
        }
        Spacer(Modifier.height(10.dp))
        FlowRow(horizontalArrangement = Arrangement.spacedBy(8.dp)) {
            // Only outer-fringe topics are "ready to start" (their prerequisites are met).
            if (node.fringe == ConceptFringe.CONCEPT_FRINGE_OUTER) {
                Button(onClick = { onSelectTopic(node.id) }) {
                    Text("Start this topic")
                }
            }
            OutlinedButton(onClick = { onOpenLesson(node.id) }) {
                Text("View lesson")
            }
        }
    }
}

/** A readable discipline name for a KC id, from its domain prefix (`GenChem::…` -> "General Chemistry"). */
private fun disciplineLabel(id: String): String =
    when (id.substringBefore("::")) {
        "Bio" -> "Biology"
        "Biochem" -> "Biochemistry"
        "GenChem" -> "General Chemistry"
        "Orgo" -> "Organic Chemistry"
        "Physics" -> "Physics"
        "PsychSoc" -> "Psychology / Sociology"
        "CARS" -> "CARS"
        else -> id.substringBefore("::").ifEmpty { "Other" }
    }

/** "N% correct" from a concept's answered/positive counts (blank sentinel when never attempted). */
private fun accuracyLabel(
    answered: Int,
    positive: Int,
): String = if (answered <= 0) "\u2014" else "${(positive.toFloat() / answered * 100f).roundToInt()}% correct"

/**
 * The concept's role in one line — "Builds on {prereqs} · Unlocks {dependents}" using leaf names, each
 * capped at three with an ellipsis — so the detail card shows how a topic sits in the prerequisite graph.
 */
private fun conceptRoleLine(
    graph: ConceptGraph,
    id: String,
): String {
    fun leaf(x: String) = x.substringAfterLast("::").replace('_', ' ')
    fun clip(names: List<String>) = names.take(3).joinToString(", ") + if (names.size > 3) "\u2026" else ""
    val buildsOn = graph.edgesList.filter { it.targetId == id }.map { leaf(it.prerequisiteId) }.distinct()
    val unlocks = graph.edgesList.filter { it.prerequisiteId == id }.map { leaf(it.targetId) }.distinct()
    val parts = mutableListOf<String>()
    if (buildsOn.isNotEmpty()) parts.add("Builds on ${clip(buildsOn)}")
    if (unlocks.isNotEmpty()) parts.add("Unlocks ${clip(unlocks)}")
    return parts.joinToString("  \u00b7  ")
}

// ---- topics list ----

@Composable
private fun TopicsCard(status: ConceptSchedulerStatusResponse) {
    val palette = LocalMcatPalette.current
    PremiumCard {
        Text("All topics", style = MaterialTheme.typography.titleMedium, fontWeight = FontWeight.SemiBold)
        Text(
            "m = mastery (how well you know it) · pri = study priority (what to review next)",
            style = MaterialTheme.typography.labelSmall,
            color = MaterialTheme.colorScheme.onSurfaceVariant,
        )
        Spacer(Modifier.height(6.dp))
        for (node in status.graph.nodesList) {
            Row(
                Modifier.fillMaxWidth().padding(vertical = 5.dp),
                verticalAlignment = Alignment.CenterVertically,
            ) {
                Box(Modifier.size(10.dp).clip(CircleShape).background(palette.colorForKc(node.id)))
                Spacer(Modifier.width(8.dp))
                Text(
                    (if (node.recommended) "★ " else "") + node.id.substringAfterLast("::").replace('_', ' '),
                    style = MaterialTheme.typography.bodyMedium,
                    modifier = Modifier.weight(1f),
                )
                StateChipForNode(node.fringe, node.answered)
                val summary =
                    if (node.answered == 0) {
                        ""
                    } else {
                        "  m ${displayMastery(node.answered, node.mastery)} · pri ${displayPriority(node.answered, node.readinessScore)}"
                    }
                if (summary.isNotEmpty()) {
                    Text(
                        summary,
                        style = MaterialTheme.typography.labelSmall,
                        color = MaterialTheme.colorScheme.onSurfaceVariant,
                    )
                }
            }
        }
    }
}

// ---- study session (secondary detail) ----

@Composable
private fun StudySessionCard(status: ConceptSchedulerStatusResponse) {
    PremiumCard {
        Text("Study session", style = MaterialTheme.typography.titleMedium, fontWeight = FontWeight.SemiBold)
        Spacer(Modifier.height(8.dp))
        if (status.hasSession()) {
            val s = status.session
            Row(Modifier.fillMaxWidth(), verticalAlignment = Alignment.CenterVertically) {
                Text(
                    "Focus budget",
                    style = MaterialTheme.typography.labelMedium,
                    modifier = Modifier.weight(1f),
                )
                Text(
                    "${(s.budgetProgress * 100).roundToInt()}%",
                    style = MaterialTheme.typography.labelMedium,
                    fontWeight = FontWeight.SemiBold,
                )
            }
            Spacer(Modifier.height(4.dp))
            MetricBar(
                fraction = s.budgetProgress.coerceIn(0f, 1f),
                color = MaterialTheme.colorScheme.primary,
                trackColor = MaterialTheme.colorScheme.surfaceVariant,
            )
            Spacer(Modifier.height(8.dp))
            LabeledValue("Active topic", s.activeTopic.ifEmpty { "—" })
            LabeledValue("Slots available", s.slotsAvailable.toString())
            LabeledValue("Next slot", "${s.reviewsTowardNextSlot}/${s.reviewsPerSlot} reviews")
            LabeledValue("Focus block", "${s.blockRemaining}/${s.blockSize} remaining")
        }
        if (status.hasEvidence() || status.hasCounters()) {
            Spacer(Modifier.height(10.dp))
            FlowRow(horizontalArrangement = Arrangement.spacedBy(8.dp), verticalArrangement = Arrangement.spacedBy(8.dp)) {
                if (status.hasEvidence()) {
                    val e = status.evidence
                    val kind = if (e.kind == ConceptEvidenceStatus.Kind.ENOUGH) "Evidence: enough" else "Evidence: building"
                    StatChip("$kind (${e.seenCards}/${e.requiredSeenCards})")
                }
                if (status.hasCounters()) {
                    val c = status.counters
                    StatChip("Today +${c.dailyPositive} / −${c.dailyNegative}")
                    StatChip("Seen ${c.totalSeenCards}")
                    if (c.prerequisiteViolationsTotal > 0) {
                        StatChip("Prereq slips ${c.prerequisiteViolationsToday} today · ${c.prerequisiteViolationsTotal} total")
                    }
                }
            }
        }
    }
}

// ---- small reusable pieces ----

@Composable
private fun PremiumCard(
    modifier: Modifier = Modifier,
    containerColor: Color = MaterialTheme.colorScheme.surfaceContainer,
    content: @Composable ColumnScope.() -> Unit,
) {
    Card(
        modifier = modifier.fillMaxWidth(),
        shape = RoundedCornerShape(20.dp),
        colors = CardDefaults.cardColors(containerColor = containerColor),
    ) {
        Column(Modifier.padding(16.dp), content = content)
    }
}

/** The hero card: a taller, softly-tinted surface that headlines the projected score. */
@Composable
private fun HeroCard(
    accent: Color,
    content: @Composable ColumnScope.() -> Unit,
) {
    val scheme = MaterialTheme.colorScheme
    Card(
        modifier = Modifier.fillMaxWidth(),
        shape = RoundedCornerShape(26.dp),
        colors = CardDefaults.cardColors(containerColor = scheme.surfaceContainerHigh),
    ) {
        Box(
            Modifier
                .fillMaxWidth()
                .background(
                    Brush.verticalGradient(
                        listOf(accent.copy(alpha = 0.12f), scheme.surfaceContainerHigh, scheme.surfaceContainer),
                    ),
                ),
        ) {
            Column(
                Modifier.fillMaxWidth().padding(20.dp),
                horizontalAlignment = Alignment.CenterHorizontally,
                content = content,
            )
        }
    }
}

/** A rounded pill for the primary CTA, filled with the brand gradient and a press-scale + ripple. */
@Composable
private fun ContinueStudyingCta(
    onClick: () -> Unit,
    modifier: Modifier = Modifier,
) {
    val palette = LocalMcatPalette.current
    Box(
        modifier
            .clip(RoundedCornerShape(50))
            .background(Brush.horizontalGradient(listOf(palette.brand, palette.brandStrong)))
            .pressable(onClick = onClick)
            .padding(horizontal = 22.dp, vertical = 13.dp),
        contentAlignment = Alignment.Center,
    ) {
        Text(
            "Continue studying  →",
            color = Color.White,
            fontWeight = FontWeight.Bold,
            style = MaterialTheme.typography.labelLarge,
        )
    }
}

/** A small pill conveying "how sure" (Low / Building / Strong confidence). */
@Composable
private fun HowSureChip(text: String) {
    val palette = LocalMcatPalette.current
    Box(
        Modifier
            .clip(RoundedCornerShape(50))
            .background(palette.brandSoft)
            .padding(horizontal = 10.dp, vertical = 4.dp),
    ) {
        Text(text, style = MaterialTheme.typography.labelMedium, fontWeight = FontWeight.SemiBold, color = palette.brand)
    }
}

/**
 * The explicit "baseline readiness" label (UI-SPEC "Honesty"): a warn-tinted banner carrying the exact
 * shared copy, shown wherever a not-yet-evidence-backed (prior/baseline) score appears so a baseline
 * number is never presented as if it were earned.
 */
@Composable
private fun BaselineBanner(
    note: String = BASELINE_READINESS_LABEL,
    modifier: Modifier = Modifier,
) {
    val palette = LocalMcatPalette.current
    Row(
        modifier
            .clip(RoundedCornerShape(12.dp))
            .background(palette.warn.copy(alpha = 0.16f))
            .padding(horizontal = 12.dp, vertical = 8.dp),
        verticalAlignment = Alignment.CenterVertically,
    ) {
        Box(Modifier.size(8.dp).clip(CircleShape).background(palette.warn))
        Spacer(Modifier.width(8.dp))
        Text(
            note,
            style = MaterialTheme.typography.labelMedium,
            fontWeight = FontWeight.Medium,
            color = MaterialTheme.colorScheme.onSurface,
        )
    }
}

/**
 * The informational "peak today -> exam day" counterfactual note shown in the Readiness detail (the
 * Compose twin of the desktop `.mc-compare`): an accent-tinted panel, distinct from the amber baseline
 * warning so it never reads as an error.
 */
@Composable
private fun CompareNote(
    text: String,
    accent: Color,
    modifier: Modifier = Modifier,
) {
    Row(
        modifier
            .clip(RoundedCornerShape(12.dp))
            .background(accent.copy(alpha = 0.10f))
            .padding(horizontal = 12.dp, vertical = 8.dp),
        verticalAlignment = Alignment.CenterVertically,
    ) {
        Text(
            text,
            style = MaterialTheme.typography.labelMedium,
            fontWeight = FontWeight.Medium,
            color = MaterialTheme.colorScheme.onSurface,
        )
    }
}

/** A compact "Baseline" pill for tight surfaces (tiles / section cards) where the full banner won't fit. */
@Composable
private fun BaselineTag() {
    val palette = LocalMcatPalette.current
    Box(
        Modifier
            .clip(RoundedCornerShape(50))
            .background(palette.warn.copy(alpha = 0.18f))
            .padding(horizontal = 8.dp, vertical = 2.dp),
    ) {
        Text("Baseline", style = MaterialTheme.typography.labelSmall, fontWeight = FontWeight.SemiBold, color = palette.warn)
    }
}

/**
 * The neutral "building evidence" tag on a section card (desktop `.mc-sec-tag`): a muted pill shown
 * until a section clears its evidence gate, so an early section estimate never reads as final.
 */
@Composable
private fun BuildingEvidenceTag() {
    Box(
        Modifier
            .clip(RoundedCornerShape(50))
            .background(MaterialTheme.colorScheme.surfaceContainerHighest)
            .padding(horizontal = 8.dp, vertical = 2.dp),
    ) {
        Text(
            "building evidence",
            style = MaterialTheme.typography.labelSmall,
            color = MaterialTheme.colorScheme.onSurfaceVariant,
        )
    }
}

/**
 * The hero's confidence range: a rounded track with a translucent band spanning the confidence
 * interval and a haloed marker at the point estimate — the Compose twin of the desktop `.mc-scale`.
 */
@Composable
private fun ScoreRangeBar(
    lowerFraction: Float,
    upperFraction: Float,
    centerFraction: Float,
    color: Color,
    modifier: Modifier = Modifier,
) {
    val scheme = MaterialTheme.colorScheme
    BoxWithConstraints(modifier.fillMaxWidth().height(12.dp)) {
        val w = maxWidth
        val lo = lowerFraction.coerceIn(0f, 1f)
        val hi = upperFraction.coerceIn(0f, 1f)
        val mid = centerFraction.coerceIn(0f, 1f)
        Box(
            Modifier
                .fillMaxWidth()
                .height(6.dp)
                .align(Alignment.CenterStart)
                .clip(RoundedCornerShape(50))
                .background(scheme.surfaceVariant),
        )
        Box(
            Modifier
                .padding(start = w * lo)
                .width((w * (hi - lo)).coerceAtLeast(2.dp))
                .height(6.dp)
                .align(Alignment.CenterStart)
                .clip(RoundedCornerShape(50))
                .background(color.copy(alpha = 0.3f)),
        )
        Box(
            Modifier
                .padding(start = (w * mid - 6.dp).coerceAtLeast(0.dp))
                .size(12.dp)
                .align(Alignment.CenterStart)
                .clip(CircleShape)
                .background(color)
                .border(2.dp, scheme.surface, CircleShape),
        )
    }
}

@Composable
private fun MiniCard(
    modifier: Modifier = Modifier,
    content: @Composable ColumnScope.() -> Unit,
) {
    Card(
        modifier = modifier.fillMaxWidth(),
        shape = RoundedCornerShape(18.dp),
        colors = CardDefaults.cardColors(containerColor = MaterialTheme.colorScheme.surfaceContainer),
    ) {
        Column(
            Modifier.padding(14.dp),
            verticalArrangement = Arrangement.spacedBy(6.dp),
            content = content,
        )
    }
}

@Composable
private fun MiniHead(
    icon: String,
    label: String,
) {
    Row(verticalAlignment = Alignment.CenterVertically) {
        Text(icon, style = MaterialTheme.typography.labelMedium)
        Spacer(Modifier.width(6.dp))
        Text(
            label,
            style = MaterialTheme.typography.labelSmall,
            letterSpacing = 0.8.sp,
            fontWeight = FontWeight.Bold,
            color = MaterialTheme.colorScheme.onSurfaceVariant,
        )
    }
}

@Composable
private fun SectionHeader(
    title: String,
    subtitle: String,
) {
    Column {
        Text(title, style = MaterialTheme.typography.titleMedium, fontWeight = FontWeight.SemiBold)
        Text(subtitle, style = MaterialTheme.typography.bodySmall, color = MaterialTheme.colorScheme.onSurfaceVariant)
    }
}

@Composable
private fun LegendChip(
    label: String,
    color: Color,
) {
    Row(verticalAlignment = Alignment.CenterVertically) {
        Box(Modifier.size(10.dp).clip(CircleShape).background(color))
        Spacer(Modifier.width(5.dp))
        Text(label, style = MaterialTheme.typography.labelSmall)
    }
}

@Composable
private fun StatChip(text: String) {
    Box(
        Modifier
            .clip(RoundedCornerShape(50))
            .background(MaterialTheme.colorScheme.surfaceContainerHighest)
            .padding(horizontal = 12.dp, vertical = 6.dp),
    ) {
        Text(text, style = MaterialTheme.typography.labelMedium, color = MaterialTheme.colorScheme.onSurface)
    }
}

@Composable
private fun LabeledValue(
    label: String,
    value: String,
) {
    Row(Modifier.fillMaxWidth().padding(vertical = 2.dp)) {
        Text(label, style = MaterialTheme.typography.bodyMedium, modifier = Modifier.weight(1f))
        Text(
            value,
            style = MaterialTheme.typography.bodyMedium.copy(fontFamily = FontFamily.Monospace),
            color = MaterialTheme.colorScheme.onSurfaceVariant,
        )
    }
}

/**
 * State chip for a topic. "In progress" (outer fringe, already attempted) gets its own tertiary
 * color so the Locked → Next up → In progress → Mastered progression reads distinctly.
 */
@Composable
private fun StateChipForNode(
    fringe: ConceptFringe,
    answered: Int,
) {
    val inProgress = fringe == ConceptFringe.CONCEPT_FRINGE_OUTER && answered > 0
    val (bg, fg) =
        when {
            fringe == ConceptFringe.CONCEPT_FRINGE_INNER ->
                MaterialTheme.colorScheme.primaryContainer to MaterialTheme.colorScheme.onPrimaryContainer
            inProgress ->
                MaterialTheme.colorScheme.tertiaryContainer to MaterialTheme.colorScheme.onTertiaryContainer
            fringe == ConceptFringe.CONCEPT_FRINGE_OUTER ->
                MaterialTheme.colorScheme.secondaryContainer to MaterialTheme.colorScheme.onSecondaryContainer
            else ->
                MaterialTheme.colorScheme.surfaceVariant to MaterialTheme.colorScheme.onSurfaceVariant
        }
    Box(
        Modifier
            .clip(RoundedCornerShape(50))
            .background(bg)
            .padding(horizontal = 10.dp, vertical = 3.dp),
    ) {
        Text(nodeStateLabel(fringe, answered), style = MaterialTheme.typography.labelSmall, color = fg)
    }
}
