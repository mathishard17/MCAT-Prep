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

import android.graphics.Paint
import androidx.compose.foundation.Canvas
import androidx.compose.foundation.background
import androidx.compose.foundation.gestures.detectTapGestures
import androidx.compose.foundation.horizontalScroll
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.OutlinedButton
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clipToBounds
import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.drawscope.DrawScope
import androidx.compose.ui.graphics.drawscope.Stroke
import androidx.compose.ui.graphics.nativeCanvas
import androidx.compose.ui.graphics.toArgb
import androidx.compose.ui.input.pointer.pointerInput
import androidx.compose.ui.layout.onSizeChanged
import androidx.compose.ui.platform.LocalDensity
import androidx.compose.ui.unit.IntSize
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import anki.scheduler.ConceptFringe
import anki.scheduler.ConceptGraph
import anki.scheduler.ConceptGraphEdge
import anki.scheduler.ConceptGraphNode
import kotlinx.coroutines.launch
import kotlin.math.hypot
import kotlin.math.max
import kotlin.math.min
import kotlin.math.sqrt

/**
 * Grid coordinate assigned to a KC node: [col] is its prerequisite depth (0 = foundation with no
 * prerequisites) so the graph reads left-to-right, and [row] is its absolute row once discipline
 * lanes are stacked as horizontal bands.
 */
data class LatticeGridPos(
    val col: Int,
    val row: Int,
)

/**
 * A full layered layout: [positions] for every placed node plus the overall [cols]/[rows] extent,
 * so the renderer can size a scrollable/zoomable canvas without re-scanning the map.
 */
data class LatticeLayout(
    val positions: Map<String, LatticeGridPos>,
    val cols: Int,
    val rows: Int,
)

/** MCAT discipline lanes, top-to-bottom. Anything else falls into the trailing [LANE_OTHER] band. */
private val DISCIPLINE_ORDER = listOf("Bio", "Biochem", "GenChem", "Orgo", "Physics", "PsychSoc", "CARS")
private const val LANE_OTHER = "Other"

/** The discipline prefix of a KC id (`Bio::DNA` -> `Bio`); ids without a `::` map to [LANE_OTHER]. */
private fun disciplineOf(id: String): String = id.substringBefore("::").ifEmpty { LANE_OTHER }

/** The lane a KC belongs to: its discipline if it's a known one, else the catch-all [LANE_OTHER]. */
private fun laneOf(id: String): String = disciplineOf(id).let { if (it in DISCIPLINE_ORDER) it else LANE_OTHER }

// MCAT super-section colors (fixed, theme-independent, matching the desktop reviewer) so a concept's
// section identity reads the same everywhere: Bio/Biochem blue, Chem/Phys red, Psych/Soc green.
internal val SectionBioBiochemColor = Color(0xFF4F74D6)
internal val SectionChemPhysColor = Color(0xFFD65F5F)
internal val SectionPsychSocColor = Color(0xFF5AA469)
internal val SectionCarsColor = Color(0xFFC99A3A)
internal val SectionOtherColor = Color(0xFF8A8A8A)

/** Bright green ring marking a "ready to start" (outer-fringe) topic — the frontier to pick from. */
internal val FrontierRingColor = Color(0xFF34C759)

/** Resolves the MCAT super-section color for a KC from its discipline prefix. */
internal fun sectionColorOf(id: String): Color =
    when (disciplineOf(id)) {
        "Bio", "Biochem" -> SectionBioBiochemColor
        "GenChem", "Orgo", "Physics" -> SectionChemPhysColor
        "PsychSoc" -> SectionPsychSocColor
        "CARS" -> SectionCarsColor
        else -> SectionOtherColor
    }

/**
 * Pure, deterministic layout for the knowledge lattice ("linear paths", not a force-directed
 * hairball). Each node's [LatticeGridPos.col] is the length of the longest prerequisite chain
 * leading into it (foundations = 0), so the graph reads left-to-right from foundations to advanced
 * topics. Nodes are grouped into fixed discipline lanes (stacked as horizontal bands with a gap row
 * between lanes); within a lane, nodes sharing a column stack into successive rows, ordered by id
 * for stability. Prerequisite cycles are handled defensively (a node in a cycle collapses toward
 * column 0 rather than recursing forever).
 *
 * @param nodeIds all KC ids to place
 * @param edges prerequisite edges as `(prerequisiteId, targetId)` — target depends on prerequisite
 */
fun computeLatticeLayout(
    nodeIds: List<String>,
    edges: List<Pair<String, String>>,
): LatticeLayout {
    if (nodeIds.isEmpty()) return LatticeLayout(emptyMap(), cols = 0, rows = 0)

    val idSet = nodeIds.toHashSet()
    val prereqsOf = nodeIds.associateWith { mutableListOf<String>() }
    for ((prereq, target) in edges) {
        if (target in idSet && prereq in idSet) {
            prereqsOf.getValue(target).add(prereq)
        }
    }

    val depthCache = HashMap<String, Int>()

    fun depthOf(
        id: String,
        visiting: Set<String>,
    ): Int {
        depthCache[id]?.let { return it }
        // Cycle guard: if we re-enter a node already on the stack, treat it as a foundation.
        if (id in visiting) return 0
        val prereqs = prereqsOf[id].orEmpty()
        val depth = if (prereqs.isEmpty()) 0 else prereqs.maxOf { depthOf(it, visiting + id) } + 1
        depthCache[id] = depth
        return depth
    }
    val depths = nodeIds.associateWith { depthOf(it, emptySet()) }

    val positions = HashMap<String, LatticeGridPos>()
    var rowTop = 0
    var maxCol = 0
    for (lane in DISCIPLINE_ORDER + LANE_OTHER) {
        val members =
            nodeIds
                .filter { laneOf(it) == lane }
                .sortedWith(compareBy({ depths.getValue(it) }, { it }))
        if (members.isEmpty()) continue
        val perColumn = HashMap<Int, Int>()
        var laneRows = 0
        for (id in members) {
            val col = depths.getValue(id)
            val rowInColumn = perColumn.getOrDefault(col, 0)
            perColumn[col] = rowInColumn + 1
            positions[id] = LatticeGridPos(col = col, row = rowTop + rowInColumn)
            if (rowInColumn + 1 > laneRows) laneRows = rowInColumn + 1
            if (col > maxCol) maxCol = col
        }
        // Leave a blank row between lanes so disciplines read as distinct bands.
        rowTop += laneRows + 1
    }
    return LatticeLayout(positions = positions, cols = maxCol + 1, rows = maxOf(rowTop, 1))
}

/**
 * The focused neighbourhood of [focusId]: the node itself plus its DIRECT prerequisites and
 * dependents (one hop) — the clean "builds on -> this -> unlocks" picture shown by default before
 * the learner opts into the full map.
 */
private fun neighbourhoodIds(
    focusId: String,
    edges: List<ConceptGraphEdge>,
): List<String> {
    val ids = linkedSetOf(focusId)
    for (edge in edges) {
        if (edge.targetId == focusId) ids.add(edge.prerequisiteId)
        if (edge.prerequisiteId == focusId) ids.add(edge.targetId)
    }
    return ids.toList()
}

/** Theme colors captured from the call site (a [DrawScope] can't read MaterialTheme directly). */
data class LatticeColors(
    val edge: Color,
    val label: Color,
    val selectedRing: Color,
    val recommended: Color,
    val frontier: Color,
    val background: Color,
)

private const val COL_W_DP = 128f
private const val ROW_H_DP = 46f
private const val PAD_DP = 18f

/** Extra room on the right so a node's floating label isn't clipped at the canvas edge. */
private const val LABEL_PAD_DP = 96f
private const val NODE_RADIUS_DP = 8f
private const val GRAPH_HEIGHT_DP = 360f

private const val MIN_ZOOM = 0.3f
private const val MAX_ZOOM = 2.4f

/** Below this zoom the map is an overview, so labels are hidden to keep it uncluttered. */
private const val LABEL_ZOOM = 0.8f

/**
 * Computes the zoom that fits the whole [cols] x [rows] grid inside [viewport] (bounded by
 * [MIN_ZOOM]/[MAX_ZOOM]). Used to frame the focused neighbourhood on open and by the "Fit" button.
 */
private fun fitZoomFor(
    cols: Int,
    rows: Int,
    viewport: IntSize,
    stepXpx: Float,
    stepYpx: Float,
    padPx: Float,
    labelPadPx: Float,
): Float {
    if (viewport.width == 0 || viewport.height == 0) return 1f
    val availableW = viewport.width - padPx * 2 - labelPadPx
    val availableH = viewport.height - padPx * 2 - stepYpx
    val zoomW = availableW / max((cols - 1) * stepXpx, 1f)
    val zoomH = availableH / max((rows - 1) * stepYpx, 1f)
    return min(zoomW, zoomH).coerceIn(MIN_ZOOM, MAX_ZOOM)
}

/**
 * Renders the concept knowledge lattice as a legible, tap-to-select graph.
 *
 * Nodes are laid out by prerequisite depth into discipline lanes (via [computeLatticeLayout]) and
 * drawn on a large canvas sized `units * step * zoom` that lives inside a both-axes scrollable box,
 * so the ~170-KC MCAT map can be panned and zoomed instead of being crammed into a fixed area.
 * By default only the focused neighbourhood (a selected/recommended KC plus its direct prerequisites
 * and dependents) is shown; a toggle reveals the full map. Dots are colored by MCAT super-section
 * and sized by how many concepts build on them ("lynchpin" importance); labels appear only when
 * zoomed in far enough to read them. Tapping a node reports it via [onNodeSelected] (and, in the
 * focused view, re-centers the neighbourhood on it), preserving the caller's detail card.
 */
@Composable
fun ConceptLatticeGraph(
    graph: ConceptGraph,
    colors: LatticeColors,
    selectedNodeId: String?,
    onNodeSelected: (String?) -> Unit,
    modifier: Modifier = Modifier,
) {
    val nodes = graph.nodesList
    val edges = graph.edgesList
    if (nodes.isEmpty()) return

    // "Lynchpin" importance: how many other KCs list this one as a prerequisite (global, so a node's
    // size reads the same in the focused and full views).
    val dependents =
        remember(edges) {
            val counts = HashMap<String, Int>()
            for (edge in edges) counts[edge.prerequisiteId] = (counts[edge.prerequisiteId] ?: 0) + 1
            counts
        }
    val maxDependents = remember(dependents) { (dependents.values.maxOrNull() ?: 0).coerceAtLeast(1) }

    // Default focus: the tapped node, else a recommended/startable one, so the graph opens on a clean
    // neighbourhood rather than the whole galaxy. Null only when there's nothing sensible to center.
    val recommendedId =
        remember(nodes) {
            nodes.firstOrNull { it.recommended }?.id
                ?: nodes.firstOrNull { it.fringe == ConceptFringe.CONCEPT_FRINGE_OUTER }?.id
        }
    val focusId = selectedNodeId ?: recommendedId
    var showFullMap by remember { mutableStateOf(false) }
    val fullMap = showFullMap || focusId == null

    // Keep the shown set stable in full-map mode so tapping a node (which changes the focus) doesn't
    // needlessly rebuild the layout and re-fit the view. `neighbourhoodKey` is null exactly when we
    // show the whole map, so it doubles as the layout-rebuild key.
    val neighbourhoodKey = if (fullMap) null else focusId
    val shownIds =
        remember(graph, neighbourhoodKey) {
            if (neighbourhoodKey == null) nodes.map { it.id } else neighbourhoodIds(neighbourhoodKey, edges)
        }
    val shownNodes =
        remember(graph, shownIds) {
            val set = shownIds.toHashSet()
            nodes.filter { it.id in set }
        }
    val shownEdges =
        remember(graph, shownIds) {
            val set = shownIds.toHashSet()
            edges.filter { it.prerequisiteId in set && it.targetId in set }
        }
    val layout =
        remember(shownIds, shownEdges) {
            computeLatticeLayout(shownIds, shownEdges.map { it.prerequisiteId to it.targetId })
        }

    val density = LocalDensity.current
    val stepXpx = with(density) { COL_W_DP.dp.toPx() }
    val stepYpx = with(density) { ROW_H_DP.dp.toPx() }
    val padPx = with(density) { PAD_DP.dp.toPx() }
    val labelPadPx = with(density) { LABEL_PAD_DP.dp.toPx() }
    val baseRadiusPx = with(density) { NODE_RADIUS_DP.dp.toPx() }

    var zoom by remember { mutableStateOf(1f) }
    var viewport by remember { mutableStateOf(IntSize.Zero) }
    val hScroll = rememberScrollState()
    val vScroll = rememberScrollState()
    val scope = rememberCoroutineScope()

    // Frame the shown set when it (or the viewport) changes: fit it to the box and reset scroll to the
    // origin. Manual zoom (+/-) doesn't retrigger this, so the learner stays where they panned to.
    LaunchedEffect(layout, viewport) {
        if (viewport != IntSize.Zero) {
            zoom = fitZoomFor(layout.cols, layout.rows, viewport, stepXpx, stepYpx, padPx, labelPadPx)
            hScroll.scrollTo(0)
            vScroll.scrollTo(0)
        }
    }

    // Untransformed pixel centers for the current zoom; dot/label size stays constant so nodes remain
    // legible and tappable at any zoom (only the spacing between them grows/shrinks).
    val positions =
        remember(layout, zoom, stepXpx, stepYpx, padPx) {
            layout.positions.mapValues { (_, pos) ->
                Offset(padPx + pos.col * stepXpx * zoom, padPx + pos.row * stepYpx * zoom)
            }
        }

    fun radiusOf(id: String): Float {
        val dep = dependents[id] ?: 0
        return baseRadiusPx * (0.8f + 0.7f * sqrt(dep.toFloat() / maxDependents))
    }

    val canvasWpx = padPx * 2 + max(layout.cols - 1, 0) * stepXpx * zoom + labelPadPx
    val canvasHpx = padPx * 2 + max(layout.rows - 1, 0) * stepYpx * zoom + stepYpx
    val canvasWdp = with(density) { canvasWpx.toDp() }
    val canvasHdp = with(density) { canvasHpx.toDp() }
    val tapSlopPx = with(density) { 12.dp.toPx() }

    Column(modifier.fillMaxWidth()) {
        val readyCount = shownNodes.count { it.fringe == ConceptFringe.CONCEPT_FRINGE_OUTER }
        Row(Modifier.fillMaxWidth().padding(bottom = 6.dp), verticalAlignment = Alignment.CenterVertically) {
            Text(
                if (fullMap) "Full map · $readyCount ready to start" else "Nearby concepts",
                style = MaterialTheme.typography.labelLarge,
                modifier = Modifier.weight(1f),
            )
            GraphControl("\u2212") { zoom = (zoom / 1.2f).coerceIn(MIN_ZOOM, MAX_ZOOM) }
            Spacer(Modifier.width(4.dp))
            GraphControl("Fit") {
                zoom = fitZoomFor(layout.cols, layout.rows, viewport, stepXpx, stepYpx, padPx, labelPadPx)
                scope.launch {
                    hScroll.scrollTo(0)
                    vScroll.scrollTo(0)
                }
            }
            Spacer(Modifier.width(4.dp))
            GraphControl("+") { zoom = (zoom * 1.2f).coerceIn(MIN_ZOOM, MAX_ZOOM) }
            if (focusId != null) {
                Spacer(Modifier.width(4.dp))
                GraphControl(if (fullMap) "Nearby" else "Map") { showFullMap = !showFullMap }
            }
        }

        Box(
            Modifier
                .fillMaxWidth()
                .height(GRAPH_HEIGHT_DP.dp)
                .clipToBounds()
                .background(colors.background, RoundedCornerShape(8.dp))
                .onSizeChanged { viewport = it }
                .horizontalScroll(hScroll)
                .verticalScroll(vScroll),
        ) {
            Canvas(
                modifier =
                    Modifier
                        .size(canvasWdp, canvasHdp)
                        .pointerInput(positions) {
                            detectTapGestures { tap ->
                                val hit =
                                    positions.entries
                                        .minByOrNull { hypot(it.value.x - tap.x, it.value.y - tap.y) }
                                        ?.takeIf { hypot(it.value.x - tap.x, it.value.y - tap.y) <= radiusOf(it.key) + tapSlopPx }
                                onNodeSelected(hit?.key.takeIf { it != selectedNodeId })
                            }
                        },
            ) {
                val edgeStroke = 1.5.dp.toPx()
                for (edge in shownEdges) {
                    val from = positions[edge.prerequisiteId] ?: continue
                    val to = positions[edge.targetId] ?: continue
                    drawLine(color = colors.edge, start = from, end = to, strokeWidth = edgeStroke)
                }

                val showLabels = zoom >= LABEL_ZOOM
                val labelBudgetPx = (COL_W_DP.dp.toPx() * zoom * 0.82f).coerceAtLeast(baseRadiusPx * 6f)
                for (node in shownNodes) {
                    val center = positions[node.id] ?: continue
                    val radius = radiusOf(node.id)
                    val startable = node.fringe == ConceptFringe.CONCEPT_FRINGE_OUTER

                    if (node.id == selectedNodeId) {
                        drawCircle(color = colors.selectedRing, radius = radius + 6f, center = center, style = Stroke(width = 3f))
                    } else if (startable) {
                        // Highlight the frontier: a green ring marks topics whose prerequisites are met.
                        drawCircle(color = colors.frontier, radius = radius + 3.5f, center = center, style = Stroke(width = 2.5f))
                    }
                    drawCircle(color = nodeFill(node), radius = radius, center = center)
                    if (node.fringe == ConceptFringe.CONCEPT_FRINGE_INNER) {
                        drawMasteredCheck(center, radius)
                    }
                    if (node.recommended) {
                        drawRecommendedStar(center, radius, colors.recommended)
                    }
                    if (showLabels) {
                        drawNodeLabel(node.id, center, radius, colors.label, labelBudgetPx)
                    }
                }
            }
        }
    }
}

/** A compact control chip for the graph header (zoom out / fit / zoom in / focus toggle). */
@Composable
private fun GraphControl(
    label: String,
    onClick: () -> Unit,
) {
    OutlinedButton(
        onClick = onClick,
        contentPadding = PaddingValues(horizontal = 12.dp, vertical = 4.dp),
    ) {
        Text(label, style = MaterialTheme.typography.labelMedium)
    }
}

/**
 * The dot fill for a node: MCAT super-section hue, with opacity carrying its state. Locked topics
 * fade back; startable/mastered topics scale from clearly visible up to solid as mastery grows.
 */
private fun nodeFill(node: ConceptGraphNode): Color {
    val base = sectionColorOf(node.id)
    return if (node.fringe == ConceptFringe.CONCEPT_FRINGE_LOCKED) {
        base.copy(alpha = 0.25f)
    } else {
        base.copy(alpha = (0.55f + 0.45f * node.mastery.coerceIn(0f, 1f)).coerceIn(0.55f, 1f))
    }
}

/** Draws a small ✓ centered on a mastered (inner-fringe) node. */
private fun DrawScope.drawMasteredCheck(
    center: Offset,
    radius: Float,
) {
    val paint =
        Paint().apply {
            color = Color.White.toArgb()
            textSize = radius * 1.6f
            textAlign = Paint.Align.CENTER
            isAntiAlias = true
        }
    // Offset the baseline so the glyph is optically centered in the dot.
    drawContext.canvas.nativeCanvas.drawText("\u2713", center.x, center.y + radius * 0.6f, paint)
}

/**
 * Draws a small star at the node's top-right to badge a backend-"recommended" (suggested next) topic.
 * Kept as a lightweight glyph so it layers cleanly over the node without competing with the selection
 * ring or the label.
 */
private fun DrawScope.drawRecommendedStar(
    center: Offset,
    radius: Float,
    color: Color,
) {
    val paint =
        Paint().apply {
            this.color = color.toArgb()
            textSize = radius * 2.2f
            textAlign = Paint.Align.CENTER
            isAntiAlias = true
        }
    drawContext.canvas.nativeCanvas.drawText("\u2605", center.x + radius * 1.1f, center.y - radius * 0.7f, paint)
}

/**
 * Draws the short KC label (last path segment) floating to the RIGHT of the node and vertically
 * centered on it, so a node's vertical spacing isn't eaten by its label. Truncated with an ellipsis
 * to fit [maxWidthPx] so long names don't overrun the next column. The full id is always available
 * via the detail card.
 */
private fun DrawScope.drawNodeLabel(
    id: String,
    center: Offset,
    radius: Float,
    color: Color,
    maxWidthPx: Float,
) {
    val paint =
        Paint().apply {
            this.color = color.toArgb()
            textSize = 12.sp.toPx()
            textAlign = Paint.Align.LEFT
            isAntiAlias = true
        }
    val text = ellipsize(id.substringAfterLast("::").replace('_', ' '), paint, maxWidthPx)
    drawContext.canvas.nativeCanvas.drawText(text, center.x + radius + 6f, center.y + paint.textSize / 3f, paint)
}

/** Trims [text] with a trailing ellipsis until it fits within [maxWidth] px under [paint]. */
private fun ellipsize(
    text: String,
    paint: Paint,
    maxWidth: Float,
): String {
    if (paint.measureText(text) <= maxWidth) return text
    var end = text.length
    while (end > 1 && paint.measureText(text.substring(0, end) + "\u2026") > maxWidth) {
        end--
    }
    return text.substring(0, end).trimEnd() + "\u2026"
}
