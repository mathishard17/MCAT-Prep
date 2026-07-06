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

import android.provider.Settings
import androidx.compose.animation.core.Animatable
import androidx.compose.animation.core.CubicBezierEasing
import androidx.compose.animation.core.LinearEasing
import androidx.compose.animation.core.animateFloat
import androidx.compose.animation.core.animateFloatAsState
import androidx.compose.animation.core.infiniteRepeatable
import androidx.compose.animation.core.rememberInfiniteTransition
import androidx.compose.animation.core.tween
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.interaction.MutableInteractionSource
import androidx.compose.foundation.interaction.collectIsPressedAsState
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.ripple
import androidx.compose.runtime.Composable
import androidx.compose.runtime.CompositionLocalProvider
import androidx.compose.runtime.Immutable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.remember
import androidx.compose.runtime.staticCompositionLocalOf
import androidx.compose.ui.Modifier
import androidx.compose.ui.composed
import androidx.compose.ui.draw.clip
import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.graphics.Brush
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.Shape
import androidx.compose.ui.graphics.graphicsLayer
import androidx.compose.ui.graphics.luminance
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.text.TextStyle
import androidx.compose.ui.unit.dp
import kotlinx.coroutines.delay
import kotlin.math.roundToInt

/**
 * The shared MCAT design tokens, mirrored 1:1 from the desktop app's canonical CSS custom properties
 * in `anki/qt/aqt/mcat_ui.py` (and `anki/ts/reviewer/reviewer.scss`) so the two apps read as one
 * product. Every value here is the exact hex the desktop uses; when the desktop defines a light and a
 * dark value, both are captured in [lightMcatPalette]/[darkMcatPalette].
 *
 * These are the brand + per-MCAT-section hues + semantic (good/warn/bad) colors that must match across
 * platforms. Neutrals (surface/border/text) come from the host [MaterialTheme] so the dashboard sits
 * naturally inside AnkiDroid's own light/dark themes.
 */
@Immutable
data class McatPalette(
    val brand: Color,
    val brandStrong: Color,
    val brandSoft: Color,
    val bio: Color,
    val chem: Color,
    val psych: Color,
    val cars: Color,
    val other: Color,
    val good: Color,
    val warn: Color,
    val bad: Color,
    val track: Color,
    val isDark: Boolean,
)

/** Light-mode tokens, copied verbatim from `mcat_ui.py` `:root`. */
private val lightMcatPalette =
    McatPalette(
        brand = Color(0xFF6366F1),
        brandStrong = Color(0xFF4F46E5),
        brandSoft = Color(0xFF6366F1).copy(alpha = 0.12f),
        bio = Color(0xFF4F74D6),
        chem = Color(0xFFE0555F),
        psych = Color(0xFF2FA96A),
        cars = Color(0xFFDDA032),
        other = Color(0xFF8A90A2),
        good = Color(0xFF21A866),
        warn = Color(0xFFE0A020),
        bad = Color(0xFFE0555F),
        track = Color(0xFFECEEF6),
        isDark = false,
    )

/** Dark-mode tokens, copied verbatim from `mcat_ui.py` `.nightMode`. */
private val darkMcatPalette =
    McatPalette(
        brand = Color(0xFF8B8FF8),
        brandStrong = Color(0xFFA5A8FB),
        brandSoft = Color(0xFF8B8FF8).copy(alpha = 0.16f),
        bio = Color(0xFF6F92EC),
        chem = Color(0xFFF0767F),
        psych = Color(0xFF4CC389),
        cars = Color(0xFFEAB857),
        other = Color(0xFF8A90A2),
        good = Color(0xFF43C98A),
        warn = Color(0xFFEAB857),
        bad = Color(0xFFF0767F),
        track = Color(0xFF262C3B),
        isDark = true,
    )

/** The active MCAT palette. Defaults to light; [McatDashboardTheme] swaps to dark to match the host. */
val LocalMcatPalette = staticCompositionLocalOf { lightMcatPalette }

/**
 * Whether the OS "remove animations" accessibility setting is on. When true, animated arcs, count-ups
 * and staggered entrances snap to their final state so the dashboard respects the user's preference
 * (the Android equivalent of `prefers-reduced-motion`).
 */
val LocalReduceMotion = staticCompositionLocalOf { false }

/**
 * The spec's "emphasized" easing (`cubic-bezier(0.2, 0, 0, 1)`) — the shared motion curve for gauges,
 * transitions and presses across both platforms.
 */
val EmphasizedEasing = CubicBezierEasing(0.2f, 0f, 0f, 1f)

/** Picks the light or dark [McatPalette] from the host theme's surface luminance. */
@Composable
fun rememberMcatPalette(): McatPalette = if (MaterialTheme.colorScheme.surface.luminance() < 0.5f) darkMcatPalette else lightMcatPalette

/** Reads the system "remove animations" setting (animator duration scale == 0). */
@Composable
fun rememberReduceMotion(): Boolean {
    val context = LocalContext.current
    return remember(context) {
        Settings.Global.getFloat(
            context.contentResolver,
            Settings.Global.ANIMATOR_DURATION_SCALE,
            1f,
        ) == 0f
    }
}

/**
 * Provides the MCAT palette + reduced-motion flag to [content]. Wrap the dashboard in this (inside
 * `AnkiDroidTheme`) so every piece reads the same canonical colors and honours the motion preference.
 */
@Composable
fun McatDashboardTheme(content: @Composable () -> Unit) {
    CompositionLocalProvider(
        LocalMcatPalette provides rememberMcatPalette(),
        LocalReduceMotion provides rememberReduceMotion(),
        content = content,
    )
}

/** Applies tabular (monospaced) figures so score numerals don't jitter while counting up. */
fun TextStyle.tabularFigures(): TextStyle = copy(fontFeatureSettings = "tnum")

/**
 * Counts an integer up to [target] from [startFrom] with the emphasized easing, for the "number count
 * up" motion. Snaps straight to [target] when reduced motion is on.
 */
@Composable
fun animatedCountUp(
    target: Int,
    startFrom: Int,
    durationMillis: Int = 900,
): Int {
    if (LocalReduceMotion.current) return target
    val anim = remember(target) { Animatable(startFrom.toFloat()) }
    LaunchedEffect(target) {
        anim.animateTo(target.toFloat(), tween(durationMillis, easing = EmphasizedEasing))
    }
    return anim.value.roundToInt()
}

/**
 * Makes [this] a tappable surface with a Material ripple and a ~0.97 press-scale (the shared "press"
 * response). The scale is skipped under reduced motion, but the ripple always fires so no tap feels
 * dead.
 */
fun Modifier.pressable(
    onClick: () -> Unit,
    enabled: Boolean = true,
): Modifier =
    composed {
        val interaction = remember { MutableInteractionSource() }
        val pressed by interaction.collectIsPressedAsState()
        val reduce = LocalReduceMotion.current
        val scale by animateFloatAsState(
            targetValue = if (pressed && !reduce) 0.97f else 1f,
            animationSpec = tween(durationMillis = 120, easing = EmphasizedEasing),
            label = "pressScale",
        )
        graphicsLayer {
            scaleX = scale
            scaleY = scale
        }.clickable(
            interactionSource = interaction,
            indication = ripple(),
            enabled = enabled,
            onClick = onClick,
        )
    }

/**
 * Fades + lifts [this] in on first composition, offset by `index × [stepMillis]` so a column of cards
 * enters in a gentle stagger. A no-op under reduced motion.
 */
fun Modifier.staggeredEntrance(
    index: Int,
    stepMillis: Int = 50,
    durationMillis: Int = 320,
): Modifier =
    composed {
        if (LocalReduceMotion.current) return@composed this
        val progress = remember { Animatable(0f) }
        LaunchedEffect(Unit) {
            delay(index.toLong() * stepMillis)
            progress.animateTo(1f, tween(durationMillis, easing = EmphasizedEasing))
        }
        graphicsLayer {
            alpha = progress.value
            translationY = (1f - progress.value) * 20.dp.toPx()
        }
    }

/**
 * A skeleton placeholder: a rounded block with a sweeping shimmer, used while the status loads instead
 * of a hard spinner. Falls back to a static tinted block under reduced motion.
 */
@Composable
fun ShimmerBox(
    modifier: Modifier,
    shape: Shape = RoundedCornerShape(12.dp),
) {
    val base = MaterialTheme.colorScheme.surfaceVariant.copy(alpha = 0.5f)
    if (LocalReduceMotion.current) {
        Box(modifier.clip(shape).background(base))
        return
    }
    val highlight = MaterialTheme.colorScheme.surfaceContainerHighest
    val transition = rememberInfiniteTransition(label = "shimmer")
    val shift by transition.animateFloat(
        initialValue = 0f,
        targetValue = 1f,
        animationSpec = infiniteRepeatable(tween(durationMillis = 1200, easing = LinearEasing)),
        label = "shimmerShift",
    )
    // Sweep a soft highlight band left-to-right across a wide span so it reads on any block width.
    val span = 900f
    val start = -span + shift * (span * 2f)
    Box(
        modifier
            .clip(shape)
            .background(
                Brush.linearGradient(
                    colors = listOf(base, highlight, base),
                    start = Offset(start, 0f),
                    end = Offset(start + span, 0f),
                ),
            ),
    )
}
