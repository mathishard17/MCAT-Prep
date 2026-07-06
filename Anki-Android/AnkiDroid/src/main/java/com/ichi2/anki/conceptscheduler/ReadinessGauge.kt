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

import androidx.compose.animation.core.Animatable
import androidx.compose.animation.core.FastOutSlowInEasing
import androidx.compose.animation.core.tween
import androidx.compose.foundation.Canvas
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.BoxScope
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.geometry.CornerRadius
import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.geometry.Size
import androidx.compose.ui.graphics.Brush
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.StrokeCap
import androidx.compose.ui.graphics.drawscope.Stroke
import androidx.compose.ui.graphics.lerp
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.dp
import kotlin.math.cos
import kotlin.math.max
import kotlin.math.sin

/**
 * MCAT scoring scales, used to place a value on a gauge. The whole-exam total runs 472–528 and each
 * section runs 118–132; both are the official AAMC ranges the backend reports against.
 */
object McatScale {
    const val TOTAL_MIN = 472f
    const val TOTAL_MAX = 528f
    const val SECTION_MIN = 118f
    const val SECTION_MAX = 132f
}

/**
 * Where [value] sits on the [min]..[max] scale as a 0..1 fraction (clamped). Used to convert a raw
 * MCAT score into a gauge sweep.
 */
fun fractionOnScale(
    value: Float,
    min: Float,
    max: Float,
): Float {
    if (max <= min) return 0f
    return ((value - min) / (max - min)).coerceIn(0f, 1f)
}

// A calm, premium "temperature" ramp for overall readiness: soft coral (low) → amber (mid) →
// emerald (high). Deliberately mid-luminance so it reads well on both light and dark surfaces.
private val ReadinessLow = Color(0xFFE5675C)
private val ReadinessMid = Color(0xFFE8A13C)
private val ReadinessHigh = Color(0xFF3BA776)

/**
 * The readiness color for a 0..1 [fraction]: a smooth two-segment blend coral → amber → emerald. A
 * fuller gauge therefore trends green, an emptier one coral — encoding "how ready" in hue as well as
 * arc length.
 */
fun readinessColor(fraction: Float): Color {
    val f = fraction.coerceIn(0f, 1f)
    return if (f < 0.5f) {
        lerp(ReadinessLow, ReadinessMid, f / 0.5f)
    } else {
        lerp(ReadinessMid, ReadinessHigh, (f - 0.5f) / 0.5f)
    }
}

/** A lighter tint of [color] (blended toward white) for the leading edge of a gauge gradient. */
fun lighten(
    color: Color,
    amount: Float = 0.22f,
): Color = lerp(color, Color.White, amount.coerceIn(0f, 1f))

/**
 * Animates a 0..1 [target] from 0 on first appearance (and on change) with a gentle ease, so gauges
 * and bars "fill in" rather than snapping. Returns the current animated value to drive drawing.
 */
@Composable
fun animatedFraction(
    target: Float,
    durationMillis: Int = 900,
    delayMillis: Int = 0,
): Float {
    // Respect the system "remove animations" setting: snap straight to the value.
    if (LocalReduceMotion.current) return target.coerceIn(0f, 1f)
    val anim = remember { Animatable(0f) }
    LaunchedEffect(target) {
        anim.animateTo(
            targetValue = target.coerceIn(0f, 1f),
            animationSpec = tween(durationMillis = durationMillis, delayMillis = delayMillis, easing = FastOutSlowInEasing),
        )
    }
    return anim.value
}

/**
 * A polished circular readiness gauge drawn on a [Canvas], with a center [content] slot (usually the
 * headline number).
 *
 * The arc opens at the bottom like a speedometer ([sweepAngle], default 270°). Layers, back to front:
 * a faint full-range track; an optional translucent confidence band spanning [bandStart]..[bandEnd];
 * the value arc from the start up to [fraction], filled with a light→[progressColor] gradient and a
 * rounded cap; and — when [showTip] — a small haloed dot at the arc's tip for a touch of delight.
 *
 * All fractions are 0..1 positions on the same scale. The value arc animates from empty on first show
 * (see [animatedFraction]); pass [animate] = false for static previews.
 */
@Composable
fun ReadinessGauge(
    fraction: Float,
    progressColor: Color,
    trackColor: Color,
    modifier: Modifier = Modifier,
    bandStart: Float? = null,
    bandEnd: Float? = null,
    strokeWidth: Dp = 12.dp,
    sweepAngle: Float = 270f,
    showTip: Boolean = false,
    animate: Boolean = true,
    content: @Composable BoxScope.() -> Unit = {},
) {
    val animated = if (animate) animatedFraction(fraction) else fraction.coerceIn(0f, 1f)
    val startAngle = 90f + (360f - sweepAngle) / 2f
    val gradient = remember(progressColor) { listOf(lighten(progressColor, 0.28f), progressColor) }
    Box(modifier, contentAlignment = Alignment.Center) {
        Canvas(Modifier.fillMaxSize()) {
            val stroke = strokeWidth.toPx()
            val inset = stroke / 2f
            val arcTopLeft = Offset(inset, inset)
            val arcSize = Size(size.width - stroke, size.height - stroke)

            drawArc(
                color = trackColor,
                startAngle = startAngle,
                sweepAngle = sweepAngle,
                useCenter = false,
                topLeft = arcTopLeft,
                size = arcSize,
                style = Stroke(width = stroke, cap = StrokeCap.Round),
            )

            if (bandStart != null && bandEnd != null && bandEnd > bandStart) {
                drawArc(
                    color = progressColor.copy(alpha = 0.20f),
                    startAngle = startAngle + bandStart.coerceIn(0f, 1f) * sweepAngle,
                    sweepAngle = (bandEnd - bandStart).coerceIn(0f, 1f) * sweepAngle,
                    useCenter = false,
                    topLeft = arcTopLeft,
                    size = arcSize,
                    style = Stroke(width = stroke * 1.7f, cap = StrokeCap.Round),
                )
            }

            if (animated > 0f) {
                drawArc(
                    brush = Brush.linearGradient(gradient),
                    startAngle = startAngle,
                    sweepAngle = animated * sweepAngle,
                    useCenter = false,
                    topLeft = arcTopLeft,
                    size = arcSize,
                    style = Stroke(width = stroke, cap = StrokeCap.Round),
                )

                if (showTip) {
                    val cx = arcTopLeft.x + arcSize.width / 2f
                    val cy = arcTopLeft.y + arcSize.height / 2f
                    val radius = arcSize.minDimension / 2f
                    val tipRad = Math.toRadians((startAngle + animated * sweepAngle).toDouble())
                    val tip = Offset(cx + radius * cos(tipRad).toFloat(), cy + radius * sin(tipRad).toFloat())
                    drawCircle(color = Color.White, radius = stroke * 0.62f, center = tip)
                    drawCircle(color = progressColor, radius = stroke * 0.40f, center = tip)
                }
            }
        }
        content()
    }
}

/**
 * A slim, pill-shaped progress bar (rounded track + animated fill) for secondary metrics like
 * coverage or budget, where a full gauge would be too heavy. Fill animates from empty on first show.
 */
@Composable
fun MetricBar(
    fraction: Float,
    color: Color,
    trackColor: Color,
    modifier: Modifier = Modifier,
    height: Dp = 6.dp,
    animate: Boolean = true,
) {
    val animated = if (animate) animatedFraction(fraction) else fraction.coerceIn(0f, 1f)
    Canvas(
        modifier
            .fillMaxWidth()
            .height(height),
    ) {
        val radius = size.height / 2f
        drawRoundRect(color = trackColor, cornerRadius = CornerRadius(radius, radius))
        val fillWidth = size.width * animated
        if (fillWidth > 0f) {
            drawRoundRect(
                color = color,
                size = Size(max(fillWidth, size.height), size.height),
                cornerRadius = CornerRadius(radius, radius),
            )
        }
    }
}
