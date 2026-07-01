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
import androidx.compose.foundation.gestures.detectTapGestures
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.drawscope.DrawScope
import androidx.compose.ui.graphics.nativeCanvas
import androidx.compose.ui.graphics.toArgb
import androidx.compose.ui.input.pointer.pointerInput
import androidx.compose.ui.platform.LocalDensity
import androidx.compose.ui.unit.IntSize
import androidx.compose.ui.unit.dp
import anki.scheduler.ConceptFringe
import anki.scheduler.ConceptGraph
import anki.scheduler.ConceptGraphNode
import kotlin.math.hypot

/**
 * Grid coordinate assigned to a KC node: [layer] is its prerequisite depth (0 = foundation with no
 * prerequisites), [indexInLayer] is its slot within the layer, and [layerSize] is how many nodes
 * share that layer (used to spread them vertically).
 */
data class LatticeGridPos(
    val layer: Int,
    val indexInLayer: Int,
    val layerSize: Int,
)

/**
 * Pure, deterministic layout for the knowledge lattice. Each node's [LatticeGridPos.layer] is the
 * length of the longest prerequisite chain leading into it (foundations = 0), so the graph reads
 * left-to-right from foundations to advanced topics. Nodes in the same layer are ordered by id for
 * stability. Prerequisite cycles are handled defensively (a node in a cycle collapses to layer 0
 * rather than recursing forever).
 *
 * @param nodeIds all KC ids to place
 * @param edges prerequisite edges as `(prerequisiteId, targetId)` — target depends on prerequisite
 */
fun computeLatticeLayout(
    nodeIds: List<String>,
    edges: List<Pair<String, String>>,
): Map<String, LatticeGridPos> {
    val prereqsOf = nodeIds.associateWith { mutableListOf<String>() }
    for ((prereq, target) in edges) {
        if (target in prereqsOf && prereq in prereqsOf) {
            prereqsOf.getValue(target).add(prereq)
        }
    }

    val layerCache = HashMap<String, Int>()
    fun layerOf(
        id: String,
        visiting: Set<String>,
    ): Int {
        layerCache[id]?.let { return it }
        // Cycle guard: if we re-enter a node already on the stack, treat it as a foundation.
        if (id in visiting) return 0
        val prereqs = prereqsOf[id].orEmpty()
        val layer = if (prereqs.isEmpty()) 0 else prereqs.maxOf { layerOf(it, visiting + id) } + 1
        layerCache[id] = layer
        return layer
    }

    val byLayer = nodeIds.groupBy { layerOf(it, emptySet()) }
    val result = HashMap<String, LatticeGridPos>()
    for ((layer, ids) in byLayer) {
        val ordered = ids.sorted()
        ordered.forEachIndexed { index, id ->
            result[id] = LatticeGridPos(layer = layer, indexInLayer = index, layerSize = ordered.size)
        }
    }
    return result
}

/** Resolves the display color for a node from its fringe classification and mastery. */
private fun nodeColor(
    node: ConceptGraphNode,
    colors: LatticeColors,
): Color {
    val base =
        when (node.fringe) {
            ConceptFringe.CONCEPT_FRINGE_INNER -> colors.inner
            // An outer-fringe topic that's already been attempted is "in progress" — distinct color.
            ConceptFringe.CONCEPT_FRINGE_OUTER -> if (node.answered > 0) colors.inProgress else colors.outer
            ConceptFringe.CONCEPT_FRINGE_LOCKED -> colors.locked
            else -> colors.locked
        }
    // Stronger mastery -> more opaque fill, so progress reads at a glance.
    val alpha = (0.35f + 0.65f * node.mastery.coerceIn(0f, 1f)).coerceIn(0.2f, 1f)
    return base.copy(alpha = alpha)
}

/** Theme colors captured from the call site (a [DrawScope] can't read MaterialTheme directly). */
data class LatticeColors(
    val inner: Color,
    val outer: Color,
    val inProgress: Color,
    val locked: Color,
    val edge: Color,
    val label: Color,
    val selectedRing: Color,
)

private const val NODE_RADIUS_DP = 10f

/** Fraction of the canvas width/height reserved as empty margin around the node grid. */
private const val PAD_X_FRAC = 0.08f
private const val PAD_Y_FRAC = 0.16f

/**
 * Renders the concept knowledge lattice as a static, tap-to-select 2D graph.
 *
 * Nodes are laid out by prerequisite depth (via [computeLatticeLayout]) to fit the canvas, colored by
 * fringe (inner/outer/locked) with opacity scaled by mastery, and connected by prerequisite edges.
 * The whole graph is fixed in place (no pan/zoom) so it behaves predictably with a mouse on an
 * emulator; tap a node to select it. The selected node id is reported via [onNodeSelected] so the
 * caller can show a detail card.
 */
@Composable
fun ConceptLatticeGraph(
    graph: ConceptGraph,
    colors: LatticeColors,
    selectedNodeId: String?,
    onNodeSelected: (String?) -> Unit,
    modifier: Modifier = Modifier,
) {
    var canvasSize by remember { mutableStateOf(IntSize.Zero) }

    val nodes = graph.nodesList
    val layout =
        remember(nodes, graph.edgesList) {
            computeLatticeLayout(
                nodeIds = nodes.map { it.id },
                edges = graph.edgesList.map { it.prerequisiteId to it.targetId },
            )
        }

    // Base (untransformed) pixel centers for each node, recomputed when the canvas is measured.
    val basePositions =
        remember(layout, canvasSize) {
            computePixelPositions(layout, canvasSize)
        }

    val radiusPx = with(LocalDensity.current) { NODE_RADIUS_DP.dp.toPx() }

    Box(modifier) {
        Canvas(
            modifier =
                Modifier
                    .fillMaxWidth()
                    .height(320.dp)
                    .pointerInput(basePositions) {
                        detectTapGestures { tap ->
                            // Static layout: hit-test the tap directly against the node positions.
                            // The tolerance is padded well beyond the (small) dot so taps are forgiving.
                            val hit =
                                basePositions.entries.firstOrNull { (_, pos) ->
                                    hypot(pos.x - tap.x, pos.y - tap.y) <= radiusPx * 2.4f
                                }
                            onNodeSelected(hit?.key.takeIf { it != selectedNodeId })
                        }
                    },
        ) {
            if (canvasSize != IntSize(size.width.toInt(), size.height.toInt())) {
                canvasSize = IntSize(size.width.toInt(), size.height.toInt())
            }

            // Horizontal room a label may occupy: the gap to the next same-parity column. Because
            // labels alternate above/below by layer, a label only competes with columns two steps
            // away, so it can safely span ~2 layer gaps before it would collide.
            val maxLayer = (layout.values.maxOfOrNull { it.layer } ?: 0).coerceAtLeast(1)
            val usableW = size.width * (1f - 2 * PAD_X_FRAC)
            val labelMaxWidth = (usableW / maxLayer * 1.9f).coerceAtLeast(radiusPx * 5f)

            // Edges first, so nodes draw on top.
            for (edge in graph.edgesList) {
                val from = basePositions[edge.prerequisiteId] ?: continue
                val to = basePositions[edge.targetId] ?: continue
                drawLine(
                    color = colors.edge,
                    start = from,
                    end = to,
                    strokeWidth = 3f,
                )
            }

            for (node in nodes) {
                val center = basePositions[node.id] ?: continue
                if (node.id == selectedNodeId) {
                    drawCircle(color = colors.selectedRing, radius = radiusPx + 5f, center = center)
                }
                drawCircle(color = nodeColor(node, colors), radius = radiusPx, center = center)
                // Alternate label placement by layer so neighbouring columns don't stack labels.
                val labelBelow = (layout[node.id]?.layer ?: 0) % 2 == 0
                drawNodeLabel(node.id, center, radiusPx, colors.label, labelBelow, labelMaxWidth)
            }
        }
    }
}

/**
 * Draws the short KC label (last path segment) centered on the node, either just below it
 * ([below] = true) or just above it, and truncated with an ellipsis to fit [maxWidthPx] so long
 * names don't overrun neighbouring columns. The full id is always available via the detail card.
 */
private fun DrawScope.drawNodeLabel(
    id: String,
    center: Offset,
    radius: Float,
    color: Color,
    below: Boolean,
    maxWidthPx: Float,
) {
    val paint =
        Paint().apply {
            this.color = color.toArgb()
            textSize = 22f
            textAlign = Paint.Align.CENTER
            isAntiAlias = true
        }
    val text = ellipsize(id.substringAfterLast("::").replace('_', ' '), paint, maxWidthPx)
    val baseline = if (below) center.y + radius + 24f else center.y - radius - 12f
    drawContext.canvas.nativeCanvas.drawText(text, center.x, baseline, paint)
}

/** Trims [text] with a trailing ellipsis until it fits within [maxWidth] px under [paint]. */
private fun ellipsize(
    text: String,
    paint: Paint,
    maxWidth: Float,
): String {
    if (paint.measureText(text) <= maxWidth) return text
    var end = text.length
    while (end > 1 && paint.measureText(text.substring(0, end) + "…") > maxWidth) {
        end--
    }
    return text.substring(0, end).trimEnd() + "…"
}

/** Maps grid positions to untransformed pixel centers within [canvasSize]. */
private fun computePixelPositions(
    layout: Map<String, LatticeGridPos>,
    canvasSize: IntSize,
): Map<String, Offset> {
    if (canvasSize.width == 0 || canvasSize.height == 0 || layout.isEmpty()) return emptyMap()
    val padX = canvasSize.width * PAD_X_FRAC
    val padY = canvasSize.height * PAD_Y_FRAC
    val usableW = canvasSize.width - 2 * padX
    val usableH = canvasSize.height - 2 * padY
    val maxLayer = layout.values.maxOf { it.layer }.coerceAtLeast(1)
    return layout.mapValues { (_, pos) ->
        val x = padX + (pos.layer.toFloat() / maxLayer) * usableW
        val y = padY + ((pos.indexInLayer + 0.5f) / pos.layerSize) * usableH
        Offset(x, y)
    }
}
