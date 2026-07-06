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
import androidx.compose.animation.core.Animatable
import androidx.compose.animation.core.FastOutSlowInEasing
import androidx.compose.animation.core.RepeatMode
import androidx.compose.animation.core.animateFloat
import androidx.compose.animation.core.infiniteRepeatable
import androidx.compose.animation.core.rememberInfiniteTransition
import androidx.compose.animation.core.tween
import androidx.compose.foundation.Canvas
import androidx.compose.foundation.background
import androidx.compose.foundation.gestures.awaitEachGesture
import androidx.compose.foundation.gestures.awaitFirstDown
import androidx.compose.foundation.gestures.calculateCentroid
import androidx.compose.foundation.gestures.calculateZoom
import androidx.compose.foundation.gestures.detectTapGestures
import androidx.compose.foundation.horizontalScroll
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
import androidx.compose.ui.geometry.CornerRadius
import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.geometry.Size
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.Path
import androidx.compose.ui.graphics.StrokeCap
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
import anki.scheduler.McatSection
import kotlinx.coroutines.launch
import kotlin.math.hypot
import kotlin.math.max
import kotlin.math.min
import kotlin.math.roundToInt
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

/**
 * The gold used to badge a backend-"recommended" (suggested next) topic — matches the desktop
 * reviewer's `#f2c200` star so the affordance reads identically across platforms.
 */
internal val RecommendedStarColor = Color(0xFFF2C200)

/**
 * Resolves the canonical MCAT super-section color for a KC from its discipline prefix, drawn from the
 * shared [McatPalette] so the hue matches the desktop app and adapts to light/dark: Bio/Biochem blue,
 * Chem/Phys red, Psych/Soc green, CARS amber.
 */
internal fun McatPalette.colorForKc(id: String): Color =
    when (disciplineOf(id)) {
        "Bio", "Biochem" -> bio
        "GenChem", "Orgo", "Physics" -> chem
        "PsychSoc" -> psych
        "CARS" -> cars
        else -> other
    }

/**
 * The canonical brand color for an MCAT [section], matching [colorForKc] so a section reads the same
 * hue in the readiness cards, the lattice dots, and the legend.
 */
internal fun McatPalette.colorForSection(section: McatSection): Color =
    when (section) {
        McatSection.MCAT_SECTION_BIO_BIOCHEM -> bio
        McatSection.MCAT_SECTION_CHEM_PHYS -> chem
        McatSection.MCAT_SECTION_PSYCH_SOC -> psych
        McatSection.MCAT_SECTION_CARS -> cars
        else -> other
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

/**
 * The startable frontier plus its immediate neighbours: every outer-fringe (ready-to-start) node and
 * their direct prerequisites/dependents. Shown by default when there is no current/recommended focus,
 * so the graph still opens on a small, meaningful slice rather than the whole map.
 */
private fun frontierNeighbourhood(
    frontier: List<String>,
    edges: List<ConceptGraphEdge>,
): List<String> {
    val ids = linkedSetOf<String>().apply { addAll(frontier) }
    val frontierSet = frontier.toHashSet()
    for (edge in edges) {
        if (edge.targetId in frontierSet) ids.add(edge.prerequisiteId)
        if (edge.prerequisiteId in frontierSet) ids.add(edge.targetId)
    }
    return ids.toList()
}

/** The de-dup key for a KC id: its leaf name, case/spacing-normalized (`GenChem::Atomic_Structure` -> `atomic structure`). */
private fun conceptLeafKey(id: String): String = id.substringAfterLast("::").replace('_', ' ').trim().lowercase()

/** The nearby node set + prerequisite edges to render, after neighbourhood selection and de-duplication. */
private data class NearbySet(
    val ids: List<String>,
    val edges: List<Pair<String, String>>,
)

/**
 * Builds the nearby neighbourhood to render and de-dupes concepts that share a leaf name across
 * disciplines (e.g. GenChem vs Physics "atomic structure") into a single representative node, remapping
 * edges onto it so the same label never appears twice. Uses [focusId]'s one-hop neighbourhood when a
 * focus exists, else the startable frontier + neighbours, else a small capped slice (never the galaxy).
 */
private fun buildNearbySet(
    nodes: List<ConceptGraphNode>,
    edges: List<ConceptGraphEdge>,
    focusId: String?,
): NearbySet {
    val byId = nodes.associateBy { it.id }
    val neighbourhood: List<String> =
        when {
            focusId != null -> neighbourhoodIds(focusId, edges)
            else -> {
                val frontier = nodes.filter { it.fringe == ConceptFringe.CONCEPT_FRINGE_OUTER }.map { it.id }
                if (frontier.isNotEmpty()) {
                    frontierNeighbourhood(frontier, edges)
                } else {
                    nodes.map { it.id }.take(NO_FOCUS_FALLBACK_CAP)
                }
            }
        }
    // Prefer keeping the focus, then recommended nodes, then original order as the representative per leaf.
    val priority =
        buildList {
            if (focusId != null && focusId in neighbourhood) add(focusId)
            addAll(neighbourhood.filter { byId[it]?.recommended == true })
            addAll(neighbourhood)
        }.distinct()
    val repByLeaf = HashMap<String, String>()
    val canonical = HashMap<String, String>()
    for (id in priority) {
        canonical[id] = repByLeaf.getOrPut(conceptLeafKey(id)) { id }
    }
    val ids = neighbourhood.map { canonical.getValue(it) }.distinct()
    val idSet = ids.toHashSet()
    val remapped =
        edges
            .mapNotNull { e ->
                val p = canonical[e.prerequisiteId] ?: return@mapNotNull null
                val t = canonical[e.targetId] ?: return@mapNotNull null
                if (p == t || p !in idSet || t !in idSet) null else p to t
            }.distinct()
    return NearbySet(ids, remapped)
}

/** Theme colors captured from the call site (a [DrawScope] can't read MaterialTheme directly). */
data class LatticeColors(
    val edge: Color,
    // Accent used to "trace" the edges of the selected node so its path pops out of the map.
    val edgeStrong: Color,
    val label: Color,
    // Chip background behind a floating node label, so it stays legible over edges in light AND dark.
    val labelBg: Color,
    val selectedRing: Color,
    val recommended: Color,
    val frontier: Color,
    val background: Color,
)

private const val COL_W_DP = 128f
private const val ROW_H_DP = 46f
private const val PAD_DP = 18f

/** Extra room on the right so a node's floating label isn't clipped at the canvas edge. */
private const val LABEL_PAD_DP = 84f
private const val NODE_RADIUS_DP = 8f

/** Shorter than the old full-map view: the nearby neighbourhood is a handful of nodes we fit + center. */
private const val GRAPH_HEIGHT_DP = 300f

private const val MIN_ZOOM = 0.3f
private const val MAX_ZOOM = 3.0f

/** Defensive cap for the rare "no focus and no startable frontier" case, so we never draw the whole galaxy. */
private const val NO_FOCUS_FALLBACK_CAP = 12

/** Eased duration for a button (+/−) zoom step; the paired scroll animates over the same span. */
private const val ZOOM_ANIM_MS = 300

/** Slightly longer eased duration for the "Fit"/open reframe so the whole map settles gently. */
private const val FIT_ANIM_MS = 440

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
 * Always shows the nearby neighbourhood (mirroring the desktop reviewer sidebar): the selected /
 * recommended KC plus its direct prerequisites and dependents, or — when there is no focus — the
 * startable frontier and its neighbours (never the whole galaxy). Nodes are laid out by prerequisite
 * depth into discipline lanes (via [computeLatticeLayout]) and the small set is fit + centered to fill
 * the (shorter) box. Concepts sharing a leaf name across disciplines are de-duped to one node. Dots
 * are colored by MCAT super-section and sized by how many concepts build on them ("lynchpin"
 * importance); labels are always drawn so the neighbourhood stays readable, and the current /
 * recommended node glows strongly. Tapping a node reports it via [onNodeSelected] (and re-centers the
 * neighbourhood on it), driving the caller's themed detail card.
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
    val palette = rememberMcatPalette()
    if (nodes.isEmpty()) {
        // Graceful empty state: keep the framed area so the card doesn't collapse to nothing.
        Box(
            modifier
                .fillMaxWidth()
                .height(GRAPH_HEIGHT_DP.dp)
                .background(colors.background, RoundedCornerShape(8.dp)),
            contentAlignment = Alignment.Center,
        ) {
            Text(
                "No concepts to display yet.",
                style = MaterialTheme.typography.bodyMedium,
                color = colors.label,
            )
        }
        return
    }

    // "Lynchpin" importance: how many other KCs list this one as a prerequisite (global, so a node's
    // size reads the same regardless of which neighbourhood is shown).
    val dependents =
        remember(edges) {
            val counts = HashMap<String, Int>()
            for (edge in edges) counts[edge.prerequisiteId] = (counts[edge.prerequisiteId] ?: 0) + 1
            counts
        }
    val maxDependents = remember(dependents) { (dependents.values.maxOrNull() ?: 0).coerceAtLeast(1) }

    // Default focus: the tapped node, else a recommended/startable one, so the graph opens on a clean
    // neighbourhood. When there is no focus at all we fall back to the startable frontier + neighbours.
    val recommendedId =
        remember(nodes) {
            nodes.firstOrNull { it.recommended }?.id
                ?: nodes.firstOrNull { it.fringe == ConceptFringe.CONCEPT_FRINGE_OUTER }?.id
        }
    val focusId = selectedNodeId ?: recommendedId

    // Always show the nearby neighbourhood, de-duped so a concept shared across disciplines appears once.
    val shown =
        remember(graph, focusId) {
            buildNearbySet(nodes, edges, focusId)
        }
    val shownIds = shown.ids
    val shownEdges = shown.edges
    val shownNodes =
        remember(graph, shownIds) {
            val set = shownIds.toHashSet()
            nodes.filter { it.id in set }
        }
    val layout =
        remember(shownIds, shownEdges) {
            computeLatticeLayout(shownIds, shownEdges)
        }

    val density = LocalDensity.current
    val stepXpx = with(density) { COL_W_DP.dp.toPx() }
    val stepYpx = with(density) { ROW_H_DP.dp.toPx() }
    val padPx = with(density) { PAD_DP.dp.toPx() }
    val labelPadPx = with(density) { LABEL_PAD_DP.dp.toPx() }
    val baseRadiusPx = with(density) { NODE_RADIUS_DP.dp.toPx() }
    // Draw margin (one un-zoomed column) so a partly off-screen node/edge near the viewport still paints.
    val cullMarginPx = stepXpx

    // Zoom is animated so button/fit reframes glide and pinch tracks the fingers; reading `.value`
    // recomputes positions each frame (cheap for <=172 nodes). Pan + fling come from the scroll box.
    val zoomAnim = remember { Animatable(1f) }
    val zoom = zoomAnim.value
    var viewport by remember { mutableStateOf(IntSize.Zero) }
    val hScroll = rememberScrollState()
    val vScroll = rememberScrollState()
    val scope = rememberCoroutineScope()

    // A gentle infinite pulse for the suggested-next halo (read only inside the draw so it repaints
    // just the canvas), mirroring the desktop reviewer's pulsing "recommended" glow.
    val pulse by rememberInfiniteTransition(label = "recommendedPulse").animateFloat(
        initialValue = 0f,
        targetValue = 1f,
        animationSpec = infiniteRepeatable(animation = tween(1300), repeatMode = RepeatMode.Reverse),
        label = "recommendedPulse",
    )

    // Tap ripple: the pressed node id plus a 0..1 expand/fade progress, for Material press feedback.
    var pressedNodeId by remember { mutableStateOf<String?>(null) }
    val pressAnim = remember { Animatable(1f) }

    // Frame the shown set when it (or the viewport) changes: ease the zoom to fill the box (reserving
    // only a little label room so a handful of nodes read large) and reset scroll to the origin. Manual
    // zoom (+/−/pinch) doesn't retrigger this, so the learner stays where they were.
    LaunchedEffect(layout, viewport) {
        if (viewport != IntSize.Zero) {
            val fit = fitZoomFor(layout.cols, layout.rows, viewport, stepXpx, stepYpx, padPx, labelPadPx * 0.5f)
            hScroll.scrollTo(0)
            vScroll.scrollTo(0)
            zoomAnim.animateTo(fit, tween(FIT_ANIM_MS, easing = FastOutSlowInEasing))
        }
    }

    // Content extent at the current zoom, plus the room labels/dots need on the right and bottom. Grow
    // the canvas to at least the viewport and center the (usually small) neighbourhood inside it, so a
    // few nodes fill the box with no dead space instead of clinging to the top-left corner.
    val contentWpx = max(layout.cols - 1, 0) * stepXpx * zoom + labelPadPx
    val contentHpx = max(layout.rows - 1, 0) * stepYpx * zoom + stepYpx
    val boundedWpx = padPx * 2 + contentWpx
    val boundedHpx = padPx * 2 + contentHpx
    val canvasWpx = max(boundedWpx, viewport.width.toFloat())
    val canvasHpx = max(boundedHpx, viewport.height.toFloat())
    val offsetXpx = ((canvasWpx - boundedWpx) / 2f).coerceAtLeast(0f)
    val offsetYpx = ((canvasHpx - boundedHpx) / 2f).coerceAtLeast(0f)
    val canvasWdp = with(density) { canvasWpx.toDp() }
    val canvasHdp = with(density) { canvasHpx.toDp() }

    // Untransformed pixel centers for the current zoom; dot/label size stays constant so nodes remain
    // legible and tappable at any zoom (only the spacing between them grows/shrinks).
    val positions =
        remember(layout, zoom, stepXpx, stepYpx, padPx, offsetXpx, offsetYpx) {
            layout.positions.mapValues { (_, pos) ->
                Offset(padPx + offsetXpx + pos.col * stepXpx * zoom, padPx + offsetYpx + pos.row * stepYpx * zoom)
            }
        }

    fun radiusOf(id: String): Float {
        val dep = dependents[id] ?: 0
        return baseRadiusPx * (0.8f + 0.7f * sqrt(dep.toFloat() / maxDependents))
    }

    val tapSlopPx = with(density) { 12.dp.toPx() }

    // Nearest node to a tap/press within its (importance-scaled) radius plus a small slop, else null.
    fun nodeAt(point: Offset): String? =
        positions.entries
            .minByOrNull { hypot(it.value.x - point.x, it.value.y - point.y) }
            ?.takeIf { hypot(it.value.x - point.x, it.value.y - point.y) <= radiusOf(it.key) + tapSlopPx }
            ?.key

    // Anchor a zoom change so the canvas point under [anchorX]/[anchorY] stays put, then ease the zoom
    // and its paired scroll over the same span so the anchor holds across the whole animation.
    fun zoomToAnimated(
        target: Float,
        anchorX: Float,
        anchorY: Float,
    ) {
        val start = zoomAnim.value
        if (start <= 0f) return
        val clamped = target.coerceIn(MIN_ZOOM, MAX_ZOOM)
        val ratio = clamped / start
        val targetH = (hScroll.value + (anchorX - padPx) * (ratio - 1f)).roundToInt().coerceAtLeast(0)
        val targetV = (vScroll.value + (anchorY - padPx) * (ratio - 1f)).roundToInt().coerceAtLeast(0)
        scope.launch { zoomAnim.animateTo(clamped, tween(ZOOM_ANIM_MS, easing = FastOutSlowInEasing)) }
        scope.launch { hScroll.animateScrollTo(targetH, tween(ZOOM_ANIM_MS, easing = FastOutSlowInEasing)) }
        scope.launch { vScroll.animateScrollTo(targetV, tween(ZOOM_ANIM_MS, easing = FastOutSlowInEasing)) }
    }

    Column(modifier.fillMaxWidth()) {
        val readyCount = shownNodes.count { it.fringe == ConceptFringe.CONCEPT_FRINGE_OUTER }
        Row(Modifier.fillMaxWidth().padding(bottom = 6.dp), verticalAlignment = Alignment.CenterVertically) {
            Text(
                if (readyCount > 0) "Nearby \u00b7 $readyCount ready" else "Nearby concepts",
                style = MaterialTheme.typography.labelLarge,
                modifier = Modifier.weight(1f),
            )
            GraphControl("\u2212") {
                zoomToAnimated(zoom / 1.25f, hScroll.value + viewport.width / 2f, vScroll.value + viewport.height / 2f)
            }
            Spacer(Modifier.width(4.dp))
            GraphControl("Fit") {
                val fit = fitZoomFor(layout.cols, layout.rows, viewport, stepXpx, stepYpx, padPx, labelPadPx * 0.5f)
                scope.launch { zoomAnim.animateTo(fit, tween(FIT_ANIM_MS, easing = FastOutSlowInEasing)) }
                scope.launch { hScroll.animateScrollTo(0, tween(FIT_ANIM_MS, easing = FastOutSlowInEasing)) }
                scope.launch { vScroll.animateScrollTo(0, tween(FIT_ANIM_MS, easing = FastOutSlowInEasing)) }
            }
            Spacer(Modifier.width(4.dp))
            GraphControl("+") {
                zoomToAnimated(zoom * 1.25f, hScroll.value + viewport.width / 2f, vScroll.value + viewport.height / 2f)
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
                        // Two-finger pinch = zoom, anchored at the pinch centroid. Single-finger drags
                        // aren't consumed here, so they fall through to the scroll box for pan + fling.
                        .pointerInput(Unit) {
                            awaitEachGesture {
                                awaitFirstDown(requireUnconsumed = false)
                                do {
                                    val event = awaitPointerEvent()
                                    if (event.changes.count { it.pressed } >= 2) {
                                        val factor = event.calculateZoom()
                                        if (factor != 1f) {
                                            val centroid = event.calculateCentroid(useCurrent = true)
                                            val start = zoomAnim.value
                                            val next = (start * factor).coerceIn(MIN_ZOOM, MAX_ZOOM)
                                            if (next != start && start > 0f) {
                                                val ratio = next / start
                                                val h = (hScroll.value + (centroid.x - padPx) * (ratio - 1f)).roundToInt().coerceAtLeast(0)
                                                val v = (vScroll.value + (centroid.y - padPx) * (ratio - 1f)).roundToInt().coerceAtLeast(0)
                                                // `awaitEachGesture` is a restricted scope, so hop to the
                                                // composable scope to drive the (suspending) zoom + scroll.
                                                scope.launch {
                                                    zoomAnim.snapTo(next)
                                                    hScroll.scrollTo(h)
                                                    vScroll.scrollTo(v)
                                                }
                                            }
                                            event.changes.forEach { it.consume() }
                                        }
                                    }
                                } while (event.changes.any { it.pressed })
                            }
                        }.pointerInput(positions) {
                            detectTapGestures(
                                onPress = { offset ->
                                    val hit = nodeAt(offset)
                                    if (hit != null) {
                                        pressedNodeId = hit
                                        scope.launch {
                                            pressAnim.snapTo(0f)
                                            pressAnim.animateTo(1f, tween(340, easing = FastOutSlowInEasing))
                                        }
                                    }
                                    tryAwaitRelease()
                                },
                                onTap = { tap -> onNodeSelected(nodeAt(tap).takeIf { it != selectedNodeId }) },
                            )
                        },
            ) {
                // Trace the selected node's edges in the accent color. No dimming: the nearby view is
                // already just a handful of nodes, so there is no galaxy to cut through.
                val traceId = selectedNodeId
                val viewLeft = hScroll.value.toFloat()
                val viewTop = vScroll.value.toFloat()
                val viewRight = viewLeft + viewport.width
                val viewBottom = viewTop + viewport.height
                val labelBudgetPx = (COL_W_DP.dp.toPx() * zoom * 0.82f).coerceAtLeast(baseRadiusPx * 6f)
                val labelTextPx = 12.sp.toPx()
                val edgeStrokeThin = 1.4.dp.toPx()
                val edgeStrokeStrong = 2.2.dp.toPx()
                val arrowSizePx = 7.dp.toPx()

                for ((prereqId, targetId) in shownEdges) {
                    val from = positions[prereqId] ?: continue
                    val to = positions[targetId] ?: continue
                    if (max(from.x, to.x) < viewLeft - cullMarginPx || min(from.x, to.x) > viewRight + cullMarginPx ||
                        max(from.y, to.y) < viewTop - cullMarginPx || min(from.y, to.y) > viewBottom + cullMarginPx
                    ) {
                        continue
                    }
                    val traced = traceId != null && (prereqId == traceId || targetId == traceId)
                    drawLatticeEdge(
                        from = from,
                        to = to,
                        targetRadius = radiusOf(targetId),
                        color = if (traced) colors.edgeStrong else colors.edge,
                        strokeWidth = if (traced) edgeStrokeStrong else edgeStrokeThin,
                        arrowSize = arrowSizePx,
                        drawArrow = true,
                    )
                }

                // Nodes: rings + dot + glyphs, only near the viewport so the ~172-node map stays smooth.
                // Labels get a second pass so a chip always sits above every dot and edge.
                val visible =
                    shownNodes.filter { node ->
                        val c = positions[node.id] ?: return@filter false
                        c.x >= viewLeft - cullMarginPx && c.x <= viewRight + cullMarginPx &&
                            c.y >= viewTop - cullMarginPx && c.y <= viewBottom + cullMarginPx
                    }
                for (node in visible) {
                    val center = positions[node.id] ?: continue
                    val radius = radiusOf(node.id)
                    val stateRingR = radius + 8.dp.toPx()
                    val startable = node.fringe == ConceptFringe.CONCEPT_FRINGE_OUTER

                    // Suggested-next / "you are here": a strong, prominent pulsing glow (layered soft
                    // halos + a bright ring) so the current/recommended node clearly stands out.
                    if (node.recommended) {
                        drawCircle(
                            color = colors.recommended.copy(alpha = 0.22f),
                            radius = stateRingR + 12.dp.toPx() + pulse * 6.dp.toPx(),
                            center = center,
                        )
                        drawCircle(
                            color = colors.recommended.copy(alpha = 0.4f),
                            radius = stateRingR + 6.dp.toPx() + pulse * 4.dp.toPx(),
                            center = center,
                        )
                        drawCircle(
                            color = colors.recommended.copy(alpha = 0.95f),
                            radius = stateRingR + 2.dp.toPx(),
                            center = center,
                            style = Stroke(width = 3.dp.toPx()),
                        )
                    }
                    // Selected wins over the green "ready to start" frontier ring.
                    when {
                        node.id == selectedNodeId ->
                            drawCircle(colors.selectedRing, radius = stateRingR, center = center, style = Stroke(width = 2.6.dp.toPx()))
                        startable ->
                            drawCircle(colors.frontier, radius = stateRingR, center = center, style = Stroke(width = 2.4.dp.toPx()))
                    }
                    // Mastery arc hugging the dot — only once attempted, so we never render the backend's
                    // default prior as if it were real mastery.
                    if (node.answered > 0) {
                        drawMasteryRing(
                            center = center,
                            radius = radius,
                            mastery = node.mastery,
                            trackColor = colors.label.copy(alpha = 0.16f),
                            arcColor = colors.label.copy(alpha = 0.85f),
                        )
                    }
                    drawCircle(color = nodeFill(node, palette), radius = radius, center = center)
                    // Press ripple over the tapped dot.
                    if (node.id == pressedNodeId && pressAnim.value < 1f) {
                        drawCircle(
                            color = colors.selectedRing.copy(alpha = 0.28f * (1f - pressAnim.value)),
                            radius = radius + pressAnim.value * 12.dp.toPx(),
                            center = center,
                        )
                    }
                    if (node.fringe == ConceptFringe.CONCEPT_FRINGE_INNER) {
                        drawMasteredCheck(center, radius)
                    }
                    if (node.recommended) {
                        drawRecommendedStar(center, radius, colors.recommended)
                    }
                }
                // Labels are always drawn in the nearby view so the neighbourhood stays readable.
                for (node in visible) {
                    val center = positions[node.id] ?: continue
                    drawNodeLabel(
                        node.id,
                        center,
                        radiusOf(node.id),
                        colors.label,
                        colors.labelBg,
                        colors.edge,
                        labelBudgetPx,
                        labelTextPx,
                    )
                }
            }
        }
    }
}

/** A compact control chip for the graph header (zoom out / fit / zoom in). */
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
private fun nodeFill(
    node: ConceptGraphNode,
    palette: McatPalette,
): Color {
    val base = palette.colorForKc(node.id)
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
 * Draws the short KC label (last path segment) as a rounded chip floating to the RIGHT of the node
 * and vertically centered on it, so a node's vertical spacing isn't eaten by its label. The chip
 * ([bgColor] fill + [borderColor] hairline) keeps the text legible over edges in both light and dark
 * themes. Truncated with an ellipsis to fit [maxWidthPx] so long names don't overrun the next column;
 * the full id is always available via the detail card.
 */
private fun DrawScope.drawNodeLabel(
    id: String,
    center: Offset,
    radius: Float,
    textColor: Color,
    bgColor: Color,
    borderColor: Color,
    maxWidthPx: Float,
    textSizePx: Float,
) {
    val paint =
        Paint().apply {
            this.color = textColor.toArgb()
            textSize = textSizePx
            textAlign = Paint.Align.LEFT
            isAntiAlias = true
        }
    val text = ellipsize(id.substringAfterLast("::").replace('_', ' '), paint, maxWidthPx)
    val textWidth = paint.measureText(text)
    val padH = 5.dp.toPx()
    val padV = 3.dp.toPx()
    val left = center.x + radius + 6.dp.toPx()
    val top = center.y - textSizePx / 2f - padV
    val height = textSizePx + padV * 2
    val corner = CornerRadius(4.dp.toPx(), 4.dp.toPx())
    val chipTopLeft = Offset(left - padH, top)
    val chipSize = Size(textWidth + padH * 2, height)
    drawRoundRect(color = bgColor, topLeft = chipTopLeft, size = chipSize, cornerRadius = corner)
    drawRoundRect(color = borderColor, topLeft = chipTopLeft, size = chipSize, cornerRadius = corner, style = Stroke(width = 1.dp.toPx()))
    drawContext.canvas.nativeCanvas.drawText(text, left, center.y + textSizePx / 3f, paint)
}

/**
 * Draws a mastery arc hugging the dot: a faint full-circle [trackColor] track with an [arcColor]
 * sweep of `mastery × 360°` from the top, so a node's fill (section hue) reads at a glance while the
 * ring shows how far mastery has progressed.
 */
private fun DrawScope.drawMasteryRing(
    center: Offset,
    radius: Float,
    mastery: Float,
    trackColor: Color,
    arcColor: Color,
) {
    val ringRadius = radius + 3.dp.toPx()
    val stroke = 2.2.dp.toPx()
    val topLeft = Offset(center.x - ringRadius, center.y - ringRadius)
    val size = Size(ringRadius * 2f, ringRadius * 2f)
    drawArc(
        color = trackColor,
        startAngle = -90f,
        sweepAngle = 360f,
        useCenter = false,
        topLeft = topLeft,
        size = size,
        style = Stroke(width = stroke),
    )
    val sweep = 360f * mastery.coerceIn(0f, 1f)
    if (sweep > 0f) {
        drawArc(
            color = arcColor,
            startAngle = -90f,
            sweepAngle = sweep,
            useCenter = false,
            topLeft = topLeft,
            size = size,
            style = Stroke(width = stroke, cap = StrokeCap.Round),
        )
    }
}

/**
 * Draws one directional prerequisite edge (prerequisite → target) as a smooth left-to-right cubic
 * whose control points are pulled horizontally: same-row edges read as straight lines and cross-lane
 * edges bow gently, so the map reads as "builds on → this → unlocks". When [drawArrow] is set (i.e.
 * zoomed in enough to be legible) a small arrowhead is placed at the target's boundary so the
 * direction is unambiguous. Because every target sits at least one column right of its prerequisite,
 * the approach is always horizontal, so the arrow points cleanly into the node.
 */
private fun DrawScope.drawLatticeEdge(
    from: Offset,
    to: Offset,
    targetRadius: Float,
    color: Color,
    strokeWidth: Float,
    arrowSize: Float,
    drawArrow: Boolean,
) {
    val handle = (to.x - from.x) * 0.42f
    val path =
        Path().apply {
            moveTo(from.x, from.y)
            cubicTo(from.x + handle, from.y, to.x - handle, to.y, to.x, to.y)
        }
    drawPath(path, color = color, style = Stroke(width = strokeWidth, cap = StrokeCap.Round))
    if (drawArrow && to.x > from.x) {
        // The cubic's control point sits directly left of the target, so the curve approaches
        // horizontally: a rightward arrowhead at the target boundary reads as "into the node".
        val tipX = to.x - targetRadius
        val baseX = tipX - arrowSize
        val half = arrowSize * 0.55f
        val head =
            Path().apply {
                moveTo(tipX, to.y)
                lineTo(baseX, to.y - half)
                lineTo(baseX, to.y + half)
                close()
            }
        drawPath(head, color = color)
    }
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
