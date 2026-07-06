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

import anki.scheduler.McatSection
import com.ichi2.anki.libanki.Collection
import timber.log.Timber
import java.lang.reflect.InvocationTargetException

/**
 * Focuses the Concept Scheduler on a single MCAT [section] for [deckId] (the section picker), or
 * clears the focus when [section] is null, by invoking the backend `SetConceptSelectedSection` RPC.
 *
 * That RPC (and its request message) is being added to the proto/Rust backend by a parallel change,
 * so it may be **absent** from the generated Kotlin bindings this module currently compiles against.
 * To keep `:AnkiDroid:assemblePlayDebug` green either way, the call is dispatched **reflectively**:
 * when `backend.setConceptSelectedSection` and `anki.scheduler.SetConceptSelectedSectionRequest` are
 * present it runs normally; when they aren't it is a logged no-op, so the section cards' tap feedback
 * and visual parity are never blocked on the backend.
 *
 * Once the binding lands, this whole helper can be replaced with a direct typed call:
 *
 * ```
 * backend.setConceptSelectedSection(
 *     anki.scheduler.setConceptSelectedSectionRequest {
 *         this.deckId = deckId
 *         section?.let { this.section = it }
 *     },
 * )
 * ```
 *
 * @return true if the RPC was dispatched, false if the binding is not generated yet.
 */
fun Collection.trySetConceptSelectedSection(
    deckId: Long,
    section: McatSection?,
): Boolean =
    try {
        val requestClass = Class.forName("anki.scheduler.SetConceptSelectedSectionRequest")
        val builder = requestClass.getMethod("newBuilder").invoke(null)
        val builderClass = builder.javaClass
        builderClass.getMethod("setDeckId", java.lang.Long.TYPE).invoke(builder, deckId)
        // Leaving the section unset (proto `optional`) is how the backend clears the focus.
        if (section != null) {
            builderClass.getMethod("setSection", McatSection::class.java).invoke(builder, section)
        }
        val request = builderClass.getMethod("build").invoke(builder)
        backend.javaClass.getMethod("setConceptSelectedSection", requestClass).invoke(backend, request)
        true
    } catch (e: InvocationTargetException) {
        // The RPC exists but the backend rejected the call — surface the real error to the caller
        // (which runs inside launchCatchingTask) rather than swallowing it as "unavailable".
        throw e.cause ?: e
    } catch (e: ReflectiveOperationException) {
        Timber.i(e, "SetConceptSelectedSection RPC not generated yet; section picker is a visual no-op")
        false
    }
