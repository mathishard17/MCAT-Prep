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
import android.graphics.Color
import android.webkit.JavascriptInterface
import android.webkit.WebView
import androidx.annotation.Keep
import androidx.compose.foundation.isSystemInDarkTheme
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.key
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.compose.ui.viewinterop.AndroidView
import anki.scheduler.ConceptLesson
import org.json.JSONObject

/**
 * Renders one authored concept lesson (the desktop "lesson-first" panel, mirrored for AnkiDroid). Shows
 * up to seven sections — Overview, Key concepts, Builds on, Diagram, Worked example, Common
 * misconception, and Try it — skipping any the backend left empty. When the backend reports no (ungated)
 * lesson for the concept, a short placeholder is shown instead. Read-only: every value comes from
 * [lesson] and the panel is purely informational (dismiss via the hosting sheet), matching desktop.
 *
 * Callers must wrap this in `AnkiDroidTheme`.
 */
@Composable
fun ConceptLessonScreen(
    lesson: ConceptLesson,
    modifier: Modifier = Modifier,
) {
    Column(
        modifier
            .fillMaxWidth()
            .verticalScroll(rememberScrollState())
            .padding(16.dp),
        verticalArrangement = Arrangement.spacedBy(12.dp),
    ) {
        if (!lesson.exists) {
            Text("No lesson for this concept yet.", style = MaterialTheme.typography.bodyMedium)
            return@Column
        }

        Text(
            lesson.title.ifEmpty { lesson.kc },
            style = MaterialTheme.typography.titleLarge,
            fontWeight = FontWeight.SemiBold,
        )
        val section = lesson.section.removePrefix("MCAT::").replace('_', ' ')
        if (section.isNotBlank()) {
            Text(
                section,
                style = MaterialTheme.typography.labelLarge,
                color = MaterialTheme.colorScheme.onSurfaceVariant,
            )
        }

        LessonProseSection("Overview", lesson.overview)
        LessonListSection("Key concepts", lesson.keyConceptsList)
        LessonProseSection("Builds on", lesson.prerequisiteReminder)
        // A lesson diagram is either a Mermaid flowchart (rendered live in a WebView with the
        // bundled Mermaid library, matching desktop) or an `![alt](path)` SVG we can't load on
        // device (shown as its caption). Mermaid wins when present.
        if (lesson.diagramMermaid.isNotBlank()) {
            Column(verticalArrangement = Arrangement.spacedBy(2.dp)) {
                Text("Diagram", style = MaterialTheme.typography.titleSmall, fontWeight = FontWeight.SemiBold)
                LessonMermaidDiagram(lesson.diagramMermaid)
            }
        } else {
            LessonProseSection("Diagram", diagramCaption(lesson.diagram))
        }
        LessonProseSection("Worked example", lesson.workedExample)
        LessonProseSection("Common misconception", lesson.commonMisconception)
        LessonProseSection("Try it", lesson.firstRetrievalPrompt)
    }
}

/** A titled prose block, omitted entirely when [body] is blank so empty sections don't render. */
@Composable
private fun LessonProseSection(
    title: String,
    body: String,
) {
    if (body.isBlank()) return
    Column(verticalArrangement = Arrangement.spacedBy(2.dp)) {
        Text(title, style = MaterialTheme.typography.titleSmall, fontWeight = FontWeight.SemiBold)
        Text(body, style = MaterialTheme.typography.bodyMedium)
    }
}

/** A titled bulleted list, omitted entirely when [items] is empty. */
@Composable
private fun LessonListSection(
    title: String,
    items: List<String>,
) {
    if (items.isEmpty()) return
    Column(verticalArrangement = Arrangement.spacedBy(2.dp)) {
        Text(title, style = MaterialTheme.typography.titleSmall, fontWeight = FontWeight.SemiBold)
        for (item in items) {
            Row {
                Text("•  ", style = MaterialTheme.typography.bodyMedium)
                Text(item, style = MaterialTheme.typography.bodyMedium)
            }
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
                    setBackgroundColor(Color.TRANSPARENT)
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
