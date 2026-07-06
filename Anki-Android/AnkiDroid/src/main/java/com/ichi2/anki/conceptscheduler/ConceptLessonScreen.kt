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

import android.annotation.SuppressLint
import android.webkit.JavascriptInterface
import android.webkit.WebView
import androidx.annotation.Keep
import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.isSystemInDarkTheme
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.IntrinsicSize
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxHeight
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.statusBarsPadding
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.HorizontalDivider
import androidx.compose.material3.IconButton
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.key
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.font.FontFamily
import androidx.compose.ui.text.font.FontStyle
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import androidx.compose.ui.viewinterop.AndroidView
import anki.scheduler.ConceptLesson
import org.json.JSONObject

/**
 * Renders one authored concept lesson as a large, near-fullscreen single-column reading overlay (the
 * desktop "lesson-first" premium panel, mirrored for AnkiDroid). A sticky hero header carries the
 * eyebrow, title, section and a ✕ close; the scrolling body shows — in reading order — the pinned
 * "Watch out" misconception callout, a clamped Overview, up to four Key concepts (+"N more"), a
 * one-line "Builds on", a framed Diagram, a Worked example, and a green "Try it" retrieval CTA. Any
 * section the backend left empty is skipped, and when the backend reports no (ungated) lesson a short
 * placeholder is shown instead. Read-only: every value comes from [lesson]; the panel is purely
 * informational and dismissed via [onClose] (the hosting sheet), matching desktop.
 *
 * Callers must wrap this in `AnkiDroidTheme`.
 */
@Composable
fun ConceptLessonScreen(
    lesson: ConceptLesson,
    modifier: Modifier = Modifier,
    onClose: () -> Unit = {},
) {
    val palette = rememberMcatPalette()
    Column(
        modifier
            .fillMaxSize()
            .background(MaterialTheme.colorScheme.surface)
            .statusBarsPadding(),
    ) {
        val title = lesson.title.ifEmpty { lesson.kc }
        val section = lesson.section.removePrefix("MCAT::").replace('_', ' ')
        LessonTopBar(title = title, section = section, onClose = onClose)

        if (!lesson.exists) {
            Text(
                "No lesson for this concept yet.",
                style = MaterialTheme.typography.bodyMedium,
                modifier = Modifier.padding(16.dp),
            )
        } else {
            Column(
                Modifier
                    .fillMaxWidth()
                    .weight(1f)
                    .verticalScroll(rememberScrollState())
                    .padding(16.dp),
                verticalArrangement = Arrangement.spacedBy(14.dp),
            ) {
                // Misconception is pinned near the top (right under the hero), distinct and brief.
                if (lesson.commonMisconception.isNotBlank()) {
                    MisconceptionCallout(lesson.commonMisconception, palette.warn)
                }
                LessonProseSection("Overview", lesson.overview, maxLines = 3)
                LessonKeyConcepts(lesson.keyConceptsList, cap = 4)
                LessonBuildsOn(lesson.prerequisiteReminder)
                // A lesson diagram is either a Mermaid flowchart (rendered live in a WebView with the
                // bundled Mermaid library, matching desktop) or an `![alt](path)` SVG we can't load on
                // device (shown as its caption). Mermaid wins when present.
                if (lesson.diagramMermaid.isNotBlank()) {
                    DiagramFrame(lesson.diagramMermaid)
                } else {
                    LessonProseSection("Diagram", diagramCaption(lesson.diagram))
                }
                LessonProseSection("Worked example", lesson.workedExample)
                if (lesson.firstRetrievalPrompt.isNotBlank()) {
                    TryItCallout(lesson.firstRetrievalPrompt, palette.good)
                }
            }
        }
    }
}

/**
 * The sticky hero header: an eyebrow "LESSON", the big [title], the [section] label and a trailing ✕
 * that dismisses the panel via [onClose] — the Compose twin of the desktop lesson panel's sticky hero.
 */
@Composable
private fun LessonTopBar(
    title: String,
    section: String,
    onClose: () -> Unit,
) {
    val palette = rememberMcatPalette()
    Column(Modifier.fillMaxWidth().background(MaterialTheme.colorScheme.surface)) {
        Row(
            Modifier
                .fillMaxWidth()
                .padding(start = 16.dp, end = 4.dp, top = 10.dp, bottom = 10.dp),
            verticalAlignment = Alignment.Top,
        ) {
            Column(Modifier.weight(1f).padding(end = 8.dp)) {
                Text(
                    "LESSON",
                    style = MaterialTheme.typography.labelSmall,
                    fontWeight = FontWeight.Bold,
                    letterSpacing = 1.5.sp,
                    color = palette.brand,
                )
                Spacer(Modifier.height(2.dp))
                Text(
                    title,
                    style = MaterialTheme.typography.titleLarge,
                    fontWeight = FontWeight.SemiBold,
                )
                if (section.isNotBlank()) {
                    Text(
                        section,
                        style = MaterialTheme.typography.labelLarge,
                        color = MaterialTheme.colorScheme.onSurfaceVariant,
                    )
                }
            }
            IconButton(onClick = onClose) {
                Text(
                    "\u2715",
                    style = MaterialTheme.typography.titleLarge,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                )
            }
        }
        HorizontalDivider(color = MaterialTheme.colorScheme.outlineVariant)
    }
}

/**
 * The "⚠ Watch out" misconception callout: a warm-amber container with a left accent bar, rendered in
 * serif italic and clamped to three lines (full text lives in the authored lesson), so the most
 * common trap is impossible to miss without dominating the panel. Mirrors desktop's amber serif block.
 */
@Composable
private fun MisconceptionCallout(
    text: String,
    amber: Color,
) {
    Row(
        Modifier
            .fillMaxWidth()
            .clip(RoundedCornerShape(12.dp))
            .background(amber.copy(alpha = 0.14f))
            .height(IntrinsicSize.Min),
    ) {
        Box(Modifier.width(4.dp).fillMaxHeight().background(amber))
        Column(Modifier.padding(horizontal = 14.dp, vertical = 12.dp)) {
            Text(
                "\u26A0 Watch out",
                style = MaterialTheme.typography.labelMedium,
                fontWeight = FontWeight.Bold,
                color = amber,
            )
            Spacer(Modifier.height(4.dp))
            Text(
                text,
                style =
                    MaterialTheme.typography.bodyMedium.copy(
                        fontFamily = FontFamily.Serif,
                        fontStyle = FontStyle.Italic,
                    ),
                color = MaterialTheme.colorScheme.onSurface,
                maxLines = 3,
                overflow = TextOverflow.Ellipsis,
            )
        }
    }
}

/**
 * The "Try it" retrieval-practice CTA: a green container with a left accent bar carrying the lesson's
 * first retrieval prompt — the panel's call-to-action footer (desktop's green "Try it" block).
 */
@Composable
private fun TryItCallout(
    text: String,
    good: Color,
) {
    Row(
        Modifier
            .fillMaxWidth()
            .clip(RoundedCornerShape(12.dp))
            .background(good.copy(alpha = 0.14f))
            .height(IntrinsicSize.Min),
    ) {
        Box(Modifier.width(4.dp).fillMaxHeight().background(good))
        Column(Modifier.padding(horizontal = 14.dp, vertical = 12.dp)) {
            Text(
                "\u270E Try it",
                style = MaterialTheme.typography.labelMedium,
                fontWeight = FontWeight.Bold,
                color = good,
            )
            Spacer(Modifier.height(4.dp))
            Text(
                text,
                style = MaterialTheme.typography.bodyMedium,
                color = MaterialTheme.colorScheme.onSurface,
            )
        }
    }
}

/**
 * A titled prose block, omitted entirely when [body] is blank so empty sections don't render. When
 * [maxLines] is set the body is clamped with an ellipsis (used to keep the Overview brief).
 */
@Composable
private fun LessonProseSection(
    title: String,
    body: String,
    maxLines: Int = Int.MAX_VALUE,
) {
    if (body.isBlank()) return
    Column(verticalArrangement = Arrangement.spacedBy(2.dp)) {
        Text(title, style = MaterialTheme.typography.titleSmall, fontWeight = FontWeight.SemiBold)
        Text(
            body,
            style = MaterialTheme.typography.bodyMedium,
            maxLines = maxLines,
            overflow = TextOverflow.Ellipsis,
        )
    }
}

/**
 * The "Key concepts" list, capped at [cap] bullets with a trailing "+N more" line so the reading
 * column stays scannable (desktop caps the same list at four).
 */
@Composable
private fun LessonKeyConcepts(
    items: List<String>,
    cap: Int,
) {
    if (items.isEmpty()) return
    Column(verticalArrangement = Arrangement.spacedBy(2.dp)) {
        Text("Key concepts", style = MaterialTheme.typography.titleSmall, fontWeight = FontWeight.SemiBold)
        for (item in items.take(cap)) {
            Row {
                Text("\u2022  ", style = MaterialTheme.typography.bodyMedium)
                Text(item, style = MaterialTheme.typography.bodyMedium)
            }
        }
        val extra = items.size - cap
        if (extra > 0) {
            Text(
                "+$extra more",
                style = MaterialTheme.typography.labelMedium,
                color = MaterialTheme.colorScheme.onSurfaceVariant,
            )
        }
    }
}

/** The "Builds on" prerequisite reminder as a single muted line (desktop keeps it to one line). */
@Composable
private fun LessonBuildsOn(text: String) {
    if (text.isBlank()) return
    Text(
        "Builds on \u00b7 $text",
        style = MaterialTheme.typography.bodySmall,
        color = MaterialTheme.colorScheme.onSurfaceVariant,
        maxLines = 1,
        overflow = TextOverflow.Ellipsis,
    )
}

/** Frames the [source] diagram in a bordered, tinted surface so it reads as a distinct diagram stage. */
@Composable
private fun DiagramFrame(source: String) {
    Column(verticalArrangement = Arrangement.spacedBy(6.dp)) {
        Text("Diagram", style = MaterialTheme.typography.titleSmall, fontWeight = FontWeight.SemiBold)
        Box(
            Modifier
                .fillMaxWidth()
                .clip(RoundedCornerShape(12.dp))
                .background(MaterialTheme.colorScheme.surfaceContainerHighest)
                .border(1.dp, MaterialTheme.colorScheme.outlineVariant, RoundedCornerShape(12.dp))
                .padding(8.dp),
        ) {
            LessonMermaidDiagram(source)
        }
    }
}

/**
 * The backend diagram field is markdown: either an inline image `![alt](path)` or plain prose. We can't
 * render the referenced SVG on device, so fall back to the image's alt text as a caption (mirroring the
 * desktop caption); non-image prose is shown as-is.
 */
private fun diagramCaption(diagram: String): String {
    val image = Regex("""^!\[(.*?)]\((.*?)\)\s*$""").find(diagram.trim()) ?: return diagram
    return image.groupValues[1]
}

/**
 * Renders a Mermaid flowchart [source] into an SVG using the real Mermaid library bundled at
 * `assets/mermaid/mermaid.min.js`, inside a lightweight [WebView]. The WebView reports the rendered
 * diagram's height back over a JS bridge so the Compose host sizes to fit; until then it shows the raw
 * source as a graceful fallback. Recreated whenever the source or theme changes.
 */
@SuppressLint("SetJavaScriptEnabled")
@Composable
private fun LessonMermaidDiagram(source: String) {
    val dark = isSystemInDarkTheme()
    key(source, dark) {
        var heightDp by remember { mutableStateOf(0) }
        val html = remember { mermaidHtml(source, dark) }
        AndroidView(
            modifier =
                Modifier
                    .fillMaxWidth()
                    .height(if (heightDp > 0) heightDp.dp else 220.dp),
            factory = { context ->
                WebView(context).apply {
                    settings.javaScriptEnabled = true
                    settings.allowFileAccess = true
                    setBackgroundColor(android.graphics.Color.TRANSPARENT)
                    addJavascriptInterface(
                        MermaidHeightBridge { px -> post { heightDp = px.coerceIn(1, 4000) } },
                        "AndroidDiagram",
                    )
                    loadDataWithBaseURL("file:///android_asset/", html, "text/html", "utf-8", null)
                }
            },
        )
    }
}

/** JS→Kotlin bridge that reports the rendered diagram's CSS-pixel height (≈ dp) back to Compose. */
@Keep
private class MermaidHeightBridge(
    private val callback: (Int) -> Unit,
) {
    @JavascriptInterface
    fun onHeight(px: Int) {
        callback(px)
    }
}

/** Builds the self-contained HTML page that renders [source] with the bundled Mermaid library. */
private fun mermaidHtml(
    source: String,
    dark: Boolean,
): String {
    val theme = if (dark) "dark" else "default"
    val fg = if (dark) "#e0e0e0" else "#1b2b39"
    val jsSource = JSONObject.quote(source)
    val escaped =
        source
            .replace("&", "&amp;")
            .replace("<", "&lt;")
            .replace(">", "&gt;")
    return """
        <!DOCTYPE html>
        <html><head><meta charset="utf-8">
        <meta name="viewport" content="width=device-width, initial-scale=1">
        <style>
          html,body{margin:0;padding:0;background:transparent;color:$fg;}
          #d{padding:4px;}
          svg{max-width:100%;height:auto;}
          pre{white-space:pre-wrap;font-size:12px;color:$fg;margin:0;}
        </style>
        <script src="mermaid/mermaid.min.js"></script>
        </head><body>
        <div id="d"><pre>$escaped</pre></div>
        <script>
          function reportHeight(){ try{ if(window.AndroidDiagram){ AndroidDiagram.onHeight(document.body.scrollHeight); } }catch(e){} }
          try {
            mermaid.initialize({startOnLoad:false, securityLevel:'loose', theme:'$theme', flowchart:{htmlLabels:true, useMaxWidth:true}});
            mermaid.render('m0', $jsSource).then(function(r){
              document.getElementById('d').innerHTML = r.svg;
              setTimeout(reportHeight, 30);
            }).catch(function(e){ setTimeout(reportHeight, 30); });
          } catch(e){ setTimeout(reportHeight, 30); }
        </script>
        </body></html>
        """.trimIndent()
}
