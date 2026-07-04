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

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.FlowRow
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.Button
import androidx.compose.material3.Card
import androidx.compose.material3.CardDefaults
import androidx.compose.material3.LinearProgressIndicator
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.OutlinedButton
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.font.FontFamily
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import anki.scheduler.ConceptEvidenceStatus
import anki.scheduler.ConceptFringe
import anki.scheduler.ConceptGraphNode
import anki.scheduler.ConceptSchedulerStatusResponse
import anki.scheduler.McatSection
import kotlin.math.roundToInt

/** Coverage below which MCAT section score ranges are hidden (matches the backend/desktop gate). */
private const val MIN_COVERAGE_TO_SHOW_SCORES = 0.60f

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

// "Mastered" (not "Ready") for the inner fringe: the backend marks it once mastery clears ~0.85, so
// it means "you know this", which would otherwise clash with the test-readiness meaning of "ready".
private fun fringeLabel(fringe: ConceptFringe): String =
    when (fringe) {
        ConceptFringe.CONCEPT_FRINGE_INNER -> "Mastered"
        ConceptFringe.CONCEPT_FRINGE_OUTER -> "Next up"
        ConceptFringe.CONCEPT_FRINGE_LOCKED -> "Locked"
        else -> "Unknown"
    }

/**
 * State label for a specific topic. Unlike [fringeLabel] this also considers whether the topic has
 * been attempted, so an outer-fringe topic you've already started reads "In progress" rather than
 * "Next up" — otherwise a topic appears to jump straight from "Next up" to "Mastered", which is
 * confusing. Progression: Locked → Next up → In progress → Mastered.
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
    return "projected MCAT · likely $lo–$hi · scale 472–528"
}

private fun sectionLabel(section: McatSection): String =
    when (section) {
        McatSection.MCAT_SECTION_BIO_BIOCHEM -> "Bio / Biochem"
        McatSection.MCAT_SECTION_CHEM_PHYS -> "Chem / Phys"
        McatSection.MCAT_SECTION_PSYCH_SOC -> "Psych / Soc"
        McatSection.MCAT_SECTION_CARS -> "CARS"
        else -> "Section"
    }

/**
 * The single source-of-truth Compose UI for the Concept Scheduler read model. Rendered both in the
 * reviewer bottom sheet and the deck-picker full page. Callers must wrap this in `AnkiDroidTheme`.
 *
 * Values come from [status] (the backend response); this screen never computes scheduler state. The
 * only writes are user-driven and delegated to the host via [onSelectTopic] (choose the next topic to
 * study) and [onOpenLesson] (open a concept's lesson) — both no-ops by default so read-only hosts can
 * omit them.
 */
@Composable
fun ConceptSchedulerStatusScreen(
    status: ConceptSchedulerStatusResponse,
    modifier: Modifier = Modifier,
    onSelectTopic: (String) -> Unit = {},
    onOpenLesson: (String) -> Unit = {},
) {
    LazyColumn(
        modifier = modifier.fillMaxWidth(),
        contentPadding = PaddingValues(16.dp),
        verticalArrangement = Arrangement.spacedBy(12.dp),
    ) {
        item { HeaderRow(status.enabled, status.active) }

        if (!status.enabled && !status.hasGraph()) {
            item {
                InfoCard {
                    Text(
                        "Concept Scheduler Mode is not enabled for this deck.",
                        style = MaterialTheme.typography.bodyMedium,
                    )
                }
            }
            return@LazyColumn
        }

        // Prominent "what to learn next" picker at the very top, so choosing the next topic is obvious
        // (mirrors the desktop reviewer's `concept-next-topics` block).
        if (status.hasGraph()) {
            val startable = status.graph.nodesList.filter { it.fringe == ConceptFringe.CONCEPT_FRINGE_OUTER }
            val recommended = startable.filter { it.recommended }
            val nextTopics = (if (recommended.isNotEmpty()) recommended else startable).take(6)
            if (nextTopics.isNotEmpty()) {
                item { NextTopicsCard(nextTopics, anyRecommended = recommended.isNotEmpty(), onSelectTopic = onSelectTopic) }
            }
        }

        if (status.hasSession()) {
            item { SessionCard(status) }
        }

        item { EvidenceCountersCard(status) }

        // MCAT score (projected total + per-section breakdown) sits ABOVE the graph, mirroring the
        // desktop reviewer where the projected MCAT total headlines the score block.
        if (status.hasProjection || status.sectionScoresCount > 0) {
            item { SectionsCard(status) }
        }

        if (status.hasGraph()) {
            item { LatticeCard(status, onSelectTopic, onOpenLesson) }
            item { TopicsCard(status) }
        }
    }
}

@Composable
private fun HeaderRow(
    enabled: Boolean,
    active: Boolean,
) {
    Row(horizontalArrangement = Arrangement.spacedBy(8.dp)) {
        StatusChip(if (enabled) "Enabled" else "Disabled", enabled)
        StatusChip(if (active) "Active" else "Inactive", active)
    }
}

/**
 * Prominent "what to learn next" picker shown at the top of the screen so choosing the next topic is
 * obvious. Lists [topics] (the recommended startable topics, or all outer-fringe topics when none are
 * flagged) as large full-width buttons — mirroring the desktop reviewer's `concept-next-topics` block.
 * Tapping a topic selects it via [onSelectTopic]; recommended topics keep the ★ marker.
 */
@Composable
private fun NextTopicsCard(
    topics: List<ConceptGraphNode>,
    anyRecommended: Boolean,
    onSelectTopic: (String) -> Unit,
) {
    Card(
        modifier = Modifier.fillMaxWidth(),
        colors = CardDefaults.cardColors(containerColor = MaterialTheme.colorScheme.secondaryContainer),
    ) {
        Column(Modifier.padding(16.dp)) {
            Text(
                "Pick your next topic",
                style = MaterialTheme.typography.titleMedium,
                fontWeight = FontWeight.SemiBold,
                color = MaterialTheme.colorScheme.onSecondaryContainer,
            )
            Spacer(Modifier.height(4.dp))
            Text(
                if (anyRecommended) {
                    "Suggested next — prerequisites are ready. Tap one to start."
                } else {
                    "Ready to start — prerequisites are met. Tap one to begin."
                },
                style = MaterialTheme.typography.bodySmall,
                color = MaterialTheme.colorScheme.onSecondaryContainer,
            )
            Spacer(Modifier.height(12.dp))
            Column(verticalArrangement = Arrangement.spacedBy(8.dp)) {
                for (node in topics) {
                    Button(
                        onClick = { onSelectTopic(node.id) },
                        modifier = Modifier.fillMaxWidth(),
                    ) {
                        Text(
                            node.id.substringAfterLast("::").replace('_', ' '),
                            modifier = Modifier.weight(1f),
                        )
                        Text(if (node.recommended) "★ Start" else "Start ▸")
                    }
                }
            }
        }
    }
}

@Composable
private fun SessionCard(status: ConceptSchedulerStatusResponse) {
    val s = status.session
    SectionCard(title = "Session — card selection") {
        LabeledValue("Active topic", s.activeTopic.ifEmpty { "—" })
        if (s.selectedTopic.isNotEmpty()) LabeledValue("Selected topic", s.selectedTopic)
        LabeledValue("Slots available", s.slotsAvailable.toString())
        LabeledValue("Next slot", "${s.reviewsTowardNextSlot}/${s.reviewsPerSlot} reviews")
        LabeledValue("Focus block", "${s.blockRemaining}/${s.blockSize} remaining")
        Spacer(Modifier.height(6.dp))
        Text("Budget ${(s.budgetProgress * 100).toInt()}%", style = MaterialTheme.typography.labelMedium)
        LinearProgressIndicator(
            progress = { s.budgetProgress.coerceIn(0f, 1f) },
            modifier = Modifier.fillMaxWidth().padding(top = 4.dp),
        )
    }
}

@Composable
private fun EvidenceCountersCard(status: ConceptSchedulerStatusResponse) {
    SectionCard(title = "Evidence & counters") {
        if (status.hasEvidence()) {
            val e = status.evidence
            val kind = if (e.kind == ConceptEvidenceStatus.Kind.ENOUGH) "Enough" else "Building"
            LabeledValue("Evidence", "$kind (${e.seenCards}/${e.requiredSeenCards})")
        }
        if (status.hasCounters()) {
            val c = status.counters
            LabeledValue("Prereq violations", "${c.prerequisiteViolationsToday} today · ${c.prerequisiteViolationsTotal} total")
            LabeledValue("Today's evidence", "+${c.dailyPositive}  −${c.dailyNegative}")
            LabeledValue("Total seen cards", c.totalSeenCards.toString())
        }
    }
}

@Composable
private fun LatticeCard(
    status: ConceptSchedulerStatusResponse,
    onSelectTopic: (String) -> Unit,
    onOpenLesson: (String) -> Unit,
) {
    val colors =
        LatticeColors(
            edge = MaterialTheme.colorScheme.outlineVariant,
            label = MaterialTheme.colorScheme.onSurface,
            selectedRing = MaterialTheme.colorScheme.primary,
            recommended = Color(0xFFFFB300),
            frontier = FrontierRingColor,
            background = MaterialTheme.colorScheme.surface,
        )
    var selected by remember { mutableStateOf<String?>(null) }
    SectionCard(title = "Knowledge lattice") {
        // Dots are colored by MCAT super-section; state reads from the ring/mark (see hints).
        FlowRow(horizontalArrangement = Arrangement.spacedBy(12.dp), verticalArrangement = Arrangement.spacedBy(4.dp)) {
            LegendDot("Bio / Biochem", SectionBioBiochemColor)
            LegendDot("Chem / Phys", SectionChemPhysColor)
            LegendDot("Psych / Soc", SectionPsychSocColor)
            LegendDot("Ready to start", FrontierRingColor)
        }
        Text(
            "★ suggested next · ✓ mastered · ⬤ ring = ready · size = importance · tap a dot for details",
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
        ConceptLatticeGraph(
            graph = status.graph,
            colors = colors,
            selectedNodeId = selected,
            onNodeSelected = { selected = it },
            modifier = Modifier.fillMaxWidth(),
        )
        val node = status.graph.nodesList.firstOrNull { it.id == selected }
        if (node != null) {
            NodeDetail(node, onSelectTopic, onOpenLesson)
        } else {
            Text(
                "Tap a node for details",
                style = MaterialTheme.typography.labelSmall,
                color = MaterialTheme.colorScheme.onSurfaceVariant,
            )
        }
    }
}

@Composable
private fun NodeDetail(
    node: ConceptGraphNode,
    onSelectTopic: (String) -> Unit,
    onOpenLesson: (String) -> Unit,
) {
    Column(
        Modifier
            .fillMaxWidth()
            .padding(top = 8.dp)
            .background(MaterialTheme.colorScheme.surfaceContainerHigh, RoundedCornerShape(8.dp))
            .padding(12.dp),
    ) {
        Row(verticalAlignment = Alignment.CenterVertically) {
            Text(node.id, style = MaterialTheme.typography.titleSmall, modifier = Modifier.weight(1f))
            if (node.recommended) {
                Spacer(Modifier.width(8.dp))
                SuggestedNextChip()
            }
        }
        LabeledValue("State", nodeStateLabel(node.fringe, node.answered))
        LabeledValue("Mastery", displayMastery(node.answered, node.mastery))
        LabeledValue("Study priority", displayPriority(node.answered, node.readinessScore))
        LabeledValue("Prereq mastery", "%.2f".format(node.prerequisiteMastery))
        LabeledValue("Answered", "${node.answered}  (+${node.positive} / −${node.negative})")
        Spacer(Modifier.height(6.dp))
        Text(
            "Mastery = how well you know this topic. Study priority = how much studying it now would " +
                "help (high when prerequisites are solid but mastery isn't yet); it drops to 0 once mastered.",
            style = MaterialTheme.typography.labelSmall,
            color = MaterialTheme.colorScheme.onSurfaceVariant,
        )
        Spacer(Modifier.height(10.dp))
        FlowRow(horizontalArrangement = Arrangement.spacedBy(8.dp)) {
            // Only outer-fringe topics are "ready to start" (their prerequisites are met), so limit the
            // primary "start" action to them — matching the desktop's clickable outer fringe.
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

/** A small "★ Suggested next" chip flagging a backend-recommended topic (parity with the graph star). */
@Composable
private fun SuggestedNextChip() {
    Box(
        Modifier
            .background(MaterialTheme.colorScheme.tertiaryContainer, RoundedCornerShape(50))
            .padding(horizontal = 10.dp, vertical = 2.dp),
    ) {
        Text(
            "★ Suggested next",
            style = MaterialTheme.typography.labelSmall,
            color = MaterialTheme.colorScheme.onTertiaryContainer,
        )
    }
}

@Composable
private fun TopicsCard(status: ConceptSchedulerStatusResponse) {
    SectionCard(title = "Topics") {
        Text(
            "m = mastery (how well you know it) · pri = study priority (what to review next)",
            style = MaterialTheme.typography.labelSmall,
            color = MaterialTheme.colorScheme.onSurfaceVariant,
        )
        Spacer(Modifier.height(4.dp))
        for (node in status.graph.nodesList) {
            Row(
                Modifier.fillMaxWidth().padding(vertical = 4.dp),
                verticalAlignment = Alignment.CenterVertically,
            ) {
                Text(
                    // Star-prefix backend-recommended topics so the suggested-next set stands out in the list.
                    (if (node.recommended) "★ " else "") + node.id.substringAfterLast("::").replace('_', ' '),
                    style = MaterialTheme.typography.bodyMedium,
                    modifier = Modifier.weight(1f),
                )
                StateChipForNode(node.fringe, node.answered)
                Spacer(Modifier.height(0.dp))
                val summary =
                    if (node.answered == 0) {
                        "  Not started"
                    } else {
                        "  m ${displayMastery(node.answered, node.mastery)}  ·  pri ${displayPriority(node.answered, node.readinessScore)}"
                    }
                Text(
                    summary,
                    style = MaterialTheme.typography.labelMedium,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                )
            }
        }
    }
}

@Composable
private fun SectionsCard(status: ConceptSchedulerStatusResponse) {
    SectionCard(title = "MCAT score") {
        // Projected MCAT total headline (472–528 scale), matching the desktop reviewer's projection
        // block. has_projection is false until some cards are answered; until then only the
        // per-section coverage rows below appear.
        if (status.hasProjection && status.projectedTotal > 0f) {
            Text(
                status.projectedTotal.roundToInt().toString(),
                style = MaterialTheme.typography.headlineMedium,
                fontWeight = FontWeight.Bold,
            )
            Text(
                projectedTotalMeta(status.projectedTotal, status.projectedTotalLower, status.projectedTotalUpper),
                style = MaterialTheme.typography.labelMedium,
                color = MaterialTheme.colorScheme.onSurfaceVariant,
            )
            Spacer(Modifier.height(12.dp))
        }
        for (section in status.sectionScoresList) {
            Column(Modifier.fillMaxWidth().padding(vertical = 4.dp)) {
                Text(sectionLabel(section.section), style = MaterialTheme.typography.bodyMedium)
                val coverage = "Coverage ${(section.coverage * 100).toInt()}%"
                // Coverage alone gates the scores: once a section is ≥60% covered we surface the
                // estimate, even if the backend's stricter min-items `enoughEvidence` flag isn't set yet.
                val gated = section.coverage < MIN_COVERAGE_TO_SHOW_SCORES
                if (gated) {
                    Text(
                        "$coverage · need ${(MIN_COVERAGE_TO_SHOW_SCORES * 100).toInt()}% coverage",
                        style = MaterialTheme.typography.labelMedium,
                        color = MaterialTheme.colorScheme.onSurfaceVariant,
                    )
                } else {
                    Text(
                        "$coverage · perf ${scaledRange(section.performanceLower, section.performanceCenter, section.performanceUpper)}" +
                            " · readiness ${scaledRange(section.readinessLower, section.readinessCenter, section.readinessUpper)}",
                        style = MaterialTheme.typography.labelMedium,
                        color = MaterialTheme.colorScheme.onSurfaceVariant,
                    )
                }
            }
        }
    }
}

private fun scaledRange(
    lower: Float,
    center: Float,
    upper: Float,
): String = "${center.roundToInt()} (${lower.roundToInt()}–${upper.roundToInt()})"

// ---- small reusable pieces ----

@Composable
private fun SectionCard(
    title: String,
    content: @Composable () -> Unit,
) {
    Card(
        modifier = Modifier.fillMaxWidth(),
        colors = CardDefaults.cardColors(containerColor = MaterialTheme.colorScheme.surfaceContainer),
    ) {
        Column(Modifier.padding(16.dp)) {
            Text(title, style = MaterialTheme.typography.titleMedium, fontWeight = FontWeight.SemiBold)
            Spacer(Modifier.height(8.dp))
            content()
        }
    }
}

@Composable
private fun InfoCard(content: @Composable () -> Unit) {
    Card(
        modifier = Modifier.fillMaxWidth(),
        colors = CardDefaults.cardColors(containerColor = MaterialTheme.colorScheme.surfaceContainer),
    ) {
        Column(Modifier.padding(16.dp)) { content() }
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

@Composable
private fun StatusChip(
    text: String,
    on: Boolean,
) {
    val bg = if (on) MaterialTheme.colorScheme.secondaryContainer else MaterialTheme.colorScheme.surfaceVariant
    val fg = if (on) MaterialTheme.colorScheme.onSecondaryContainer else MaterialTheme.colorScheme.onSurfaceVariant
    Box(
        Modifier
            .background(bg, RoundedCornerShape(50))
            .padding(horizontal = 12.dp, vertical = 4.dp),
    ) {
        Text(text, style = MaterialTheme.typography.labelMedium, color = fg)
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
            .background(bg, RoundedCornerShape(50))
            .padding(horizontal = 10.dp, vertical = 2.dp),
    ) {
        Text(nodeStateLabel(fringe, answered), style = MaterialTheme.typography.labelSmall, color = fg)
    }
}

@Composable
private fun LegendDot(
    label: String,
    color: Color,
) {
    Row(verticalAlignment = Alignment.CenterVertically) {
        Box(Modifier.height(10.dp).width(10.dp).background(color, RoundedCornerShape(50)))
        Spacer(Modifier.width(4.dp))
        Text(label, style = MaterialTheme.typography.labelSmall)
    }
}
