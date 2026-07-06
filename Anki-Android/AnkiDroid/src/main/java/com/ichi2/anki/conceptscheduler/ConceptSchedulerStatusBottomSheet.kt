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
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.platform.ComposeView
import androidx.compose.ui.platform.ViewCompositionStrategy
import androidx.compose.ui.unit.dp
import anki.scheduler.ConceptSchedulerStatusResponse
import anki.scheduler.McatSection
import com.google.android.material.bottomsheet.BottomSheetBehavior
import com.google.android.material.bottomsheet.BottomSheetDialog
import com.google.android.material.bottomsheet.BottomSheetDialogFragment
import com.ichi2.anki.CollectionManager.withCol
import com.ichi2.anki.launchCatchingTask
import com.ichi2.anki.libanki.DeckId
import com.ichi2.anki.utils.ext.requireLong
import com.ichi2.compose.theme.AnkiDroidTheme

/**
 * Non-blocking panel that shows the Concept Scheduler read model over the reviewer, so studying is
 * never interrupted. Hosts the shared [ConceptSchedulerStatusScreen] Compose UI. Open it with
 * [newInstance] and `show(fragmentManager, TAG)`.
 *
 * Presented as a premium sheet: it opens fully expanded (so the readiness dashboard is visible at a
 * glance rather than peeking), carries a drag handle, and dragging down dismisses it.
 */
class ConceptSchedulerStatusBottomSheet : BottomSheetDialogFragment() {
    private var status by mutableStateOf<ConceptSchedulerStatusResponse?>(null)
    private val deckId: DeckId by lazy { requireArguments().requireLong(ARG_DECK_ID) }

    override fun onCreateDialog(savedInstanceState: Bundle?): Dialog {
        val dialog = super.onCreateDialog(savedInstanceState) as BottomSheetDialog
        dialog.setOnShowListener {
            val sheet = dialog.findViewById<View>(com.google.android.material.R.id.design_bottom_sheet)
            sheet?.let {
                BottomSheetBehavior.from(it).apply {
                    state = BottomSheetBehavior.STATE_EXPANDED
                    skipCollapsed = true
                }
            }
        }
        return dialog
    }

    override fun onCreateView(
        inflater: LayoutInflater,
        container: ViewGroup?,
        savedInstanceState: Bundle?,
    ): View =
        ComposeView(requireContext()).apply {
            setViewCompositionStrategy(ViewCompositionStrategy.DisposeOnViewTreeLifecycleDestroyed)
            setContent {
                AnkiDroidTheme {
                    Surface(color = MaterialTheme.colorScheme.background) {
                        Box {
                            when (val s = status) {
                                null -> DashboardSkeleton()
                                else ->
                                    ConceptSchedulerStatusScreen(
                                        s,
                                        onSelectTopic = { selectTopic(it) },
                                        onStudySection = { studySection(it) },
                                        onOpenLesson = { openLesson(it) },
                                        // "Continue studying" over the reviewer simply returns to the cards.
                                        onContinueStudying = { dismiss() },
                                    )
                            }
                            DragHandle(Modifier.align(Alignment.TopCenter))
                        }
                    }
                }
            }
        }

    override fun onViewCreated(
        view: View,
        savedInstanceState: Bundle?,
    ) {
        super.onViewCreated(view, savedInstanceState)
        launchCatchingTask {
            status = withCol { backend.getConceptSchedulerStatus(deckId) }
        }
    }

    /** Selects [kc] as the next topic (reorders the study queue), then refreshes the read model. */
    private fun selectTopic(kc: String) {
        // Bind to a distinct name so the request DSL's own `deckId` doesn't shadow the fragment's.
        val selectedDeckId = deckId
        launchCatchingTask {
            withCol {
                backend.setConceptSelectedTopic(
                    anki.scheduler.setConceptSelectedTopicRequest {
                        this.deckId = selectedDeckId
                        topic = kc
                    },
                )
            }
            status = withCol { backend.getConceptSchedulerStatus(selectedDeckId) }
        }
    }

    /** Focuses study on one MCAT [section] (the section picker), then refreshes the read model. */
    private fun studySection(section: McatSection) {
        val selectedDeckId = deckId
        launchCatchingTask {
            withCol { trySetConceptSelectedSection(selectedDeckId, section) }
            status = withCol { backend.getConceptSchedulerStatus(selectedDeckId) }
        }
    }

    private fun openLesson(kc: String) {
        ConceptLessonBottomSheet
            .newInstance(kc)
            .show(parentFragmentManager, ConceptLessonBottomSheet.TAG)
    }

    companion object {
        const val TAG = "ConceptSchedulerStatusBottomSheet"
        private const val ARG_DECK_ID = "conceptSchedulerDeckId"

        fun newInstance(deckId: DeckId): ConceptSchedulerStatusBottomSheet {
            // Build args before the fragment's `apply` to keep `deckId` unambiguously the parameter and
            // clear of the fragment's same-named lazy property (which reads not-yet-set arguments).
            val args = Bundle().apply { putLong(ARG_DECK_ID, deckId) }
            return ConceptSchedulerStatusBottomSheet().apply { arguments = args }
        }
    }
}

/** A small centered grabber at the top of the sheet, the standard affordance for drag-to-dismiss. */
@Composable
private fun DragHandle(modifier: Modifier = Modifier) {
    Box(modifier.fillMaxWidth().padding(top = 8.dp), contentAlignment = Alignment.TopCenter) {
        Box(
            Modifier
                .width(36.dp)
                .height(4.dp)
                .clip(RoundedCornerShape(50))
                .background(MaterialTheme.colorScheme.onSurfaceVariant.copy(alpha = 0.4f)),
        )
    }
}
