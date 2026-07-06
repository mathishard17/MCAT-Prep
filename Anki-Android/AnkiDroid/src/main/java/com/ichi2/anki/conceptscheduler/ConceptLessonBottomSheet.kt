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

import android.app.Dialog
import android.os.Bundle
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.CircularProgressIndicator
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.ComposeView
import androidx.compose.ui.platform.ViewCompositionStrategy
import androidx.compose.ui.unit.dp
import anki.scheduler.ConceptLesson
import com.google.android.material.bottomsheet.BottomSheetBehavior
import com.google.android.material.bottomsheet.BottomSheetDialog
import com.google.android.material.bottomsheet.BottomSheetDialogFragment
import com.ichi2.anki.CollectionManager.withCol
import com.ichi2.anki.launchCatchingTask
import com.ichi2.anki.utils.ext.requireString
import com.ichi2.compose.theme.AnkiDroidTheme

/**
 * Near-fullscreen panel that teaches a concept's lesson over the reviewer or deck picker, so studying
 * is never interrupted. Hosts the shared [ConceptLessonScreen] Compose UI. Open it with [newInstance]
 * and `show(fragmentManager, TAG)`.
 *
 * The sheet is forced fully expanded and non-draggable (it fills the screen and can't be dragged away),
 * mirroring the desktop's large single-column reading overlay; it is read-only and dismissed with the
 * ✕ in the hero header (or the system back / scrim tap).
 */
class ConceptLessonBottomSheet : BottomSheetDialogFragment() {
    private var lesson by mutableStateOf<ConceptLesson?>(null)

    override fun onCreateView(
        inflater: LayoutInflater,
        container: ViewGroup?,
        savedInstanceState: Bundle?,
    ): View =
        ComposeView(requireContext()).apply {
            layoutParams =
                ViewGroup.LayoutParams(
                    ViewGroup.LayoutParams.MATCH_PARENT,
                    ViewGroup.LayoutParams.MATCH_PARENT,
                )
            setViewCompositionStrategy(ViewCompositionStrategy.DisposeOnViewTreeLifecycleDestroyed)
            setContent {
                AnkiDroidTheme {
                    when (val l = lesson) {
                        null -> LoadingBox()
                        else -> ConceptLessonScreen(lesson = l, onClose = { dismiss() })
                    }
                }
            }
        }

    /** Force the sheet to fill the screen (fully expanded, no collapsed state) and disable drag-away. */
    override fun onCreateDialog(savedInstanceState: Bundle?): Dialog {
        val dialog = super.onCreateDialog(savedInstanceState) as BottomSheetDialog
        dialog.setOnShowListener {
            val sheet =
                dialog.findViewById<View>(com.google.android.material.R.id.design_bottom_sheet)
                    ?: return@setOnShowListener
            sheet.layoutParams = sheet.layoutParams.apply { height = ViewGroup.LayoutParams.MATCH_PARENT }
            BottomSheetBehavior.from(sheet).apply {
                state = BottomSheetBehavior.STATE_EXPANDED
                skipCollapsed = true
                isDraggable = false
                peekHeight = 0
            }
        }
        return dialog
    }

    override fun onViewCreated(
        view: View,
        savedInstanceState: Bundle?,
    ) {
        super.onViewCreated(view, savedInstanceState)
        val kc = requireArguments().requireString(ARG_KC)
        launchCatchingTask {
            lesson = withCol { backend.getConceptLesson(kc) }
        }
    }

    companion object {
        const val TAG = "ConceptLessonBottomSheet"
        private const val ARG_KC = "conceptLessonKc"

        fun newInstance(kc: String): ConceptLessonBottomSheet =
            ConceptLessonBottomSheet().apply {
                arguments = Bundle().apply { putString(ARG_KC, kc) }
            }
    }
}

@Composable
private fun LoadingBox() {
    Box(
        Modifier.fillMaxSize().padding(48.dp),
        contentAlignment = Alignment.Center,
    ) {
        CircularProgressIndicator()
    }
}
