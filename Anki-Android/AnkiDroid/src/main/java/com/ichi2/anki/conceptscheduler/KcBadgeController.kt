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

import android.view.View
import android.widget.TextView

/**
 * Shows the current card's Knowledge Component as a small badge over the flashcard during review
 * (Concept Scheduler demo). Mirrors the reviewer's `CardMarker`/`PreviousAnswerIndicator` pattern: a
 * thin controller over a single view, updated when a new card is displayed.
 *
 * The label is derived purely from the note's tags, so it needs no backend call. Tapping the badge
 * invokes [onClick] with the card's full KC id (e.g. `Bio::DNA`) so the host can open its lesson
 * (parity with the desktop's clickable KC badge).
 */
class KcBadgeController(
    private val badge: TextView,
    onClick: (String) -> Unit = {},
) {
    /** Full KC id of the currently shown card (e.g. `Bio::DNA`), or null when the badge is hidden. */
    private var currentKc: String? = null

    init {
        badge.setOnClickListener {
            currentKc?.let(onClick)
        }
    }

    /** Updates the badge from a note's [tags]; hides it when the note has no `KC::` tag. */
    fun displayForTags(tags: List<String>) {
        val label = kcBadgeLabel(tags)
        if (label == null) {
            currentKc = null
            badge.visibility = View.GONE
        } else {
            currentKc = kcIdFromTags(tags)
            badge.text = label
            badge.visibility = View.VISIBLE
        }
    }

    fun hide() {
        badge.visibility = View.GONE
    }
}

/**
 * Extracts the full KC id (without the `KC::` prefix) from a note's tags, e.g. `["KC::Bio::DNA"]` ->
 * `"Bio::DNA"`. This is the id used by the concept graph and lesson lookup. Returns null when there is
 * no `KC::` tag.
 */
fun kcIdFromTags(tags: List<String>): String? {
    val normalized = tags.map { normalizeConceptTag(it) }
    val kc = normalized.firstOrNull { it.startsWith("KC::") } ?: return null
    return kc.removePrefix("KC::")
}

/**
 * Builds the badge text from a note's tags: the KC's leaf name plus its MCAT section when present,
 * e.g. tags `["KC::Bio::DNA", "MCAT::Bio_Biochem"]` -> `"DNA · Bio/Biochem"`. Returns null when there
 * is no `KC::` tag (so the badge stays hidden for untagged cards).
 */
fun kcBadgeLabel(tags: List<String>): String? {
    val normalized = tags.map { normalizeConceptTag(it) }
    val kc = normalized.firstOrNull { it.startsWith("KC::") } ?: return null
    val leaf = kc.removePrefix("KC::").substringAfterLast("::").replace('_', ' ')
    val section =
        normalized
            .firstOrNull { it.startsWith("MCAT::") }
            ?.removePrefix("MCAT::")
            ?.replace('_', '/')
    return if (section.isNullOrBlank()) leaf else "$leaf · $section"
}
