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

import anki.scheduler.ConceptFringe
import org.junit.Test
import kotlin.test.assertEquals
import kotlin.test.assertTrue

class ConceptSchedulerHelpersTest {
    @Test
    fun `node state progresses Next up then In progress then Mastered`() {
        // Un-attempted outer-fringe topic is "Next up"; once attempted it becomes "In progress" so it
        // doesn't appear to jump straight to "Mastered".
        assertEquals("Next up", nodeStateLabel(ConceptFringe.CONCEPT_FRINGE_OUTER, answered = 0))
        assertEquals("In progress", nodeStateLabel(ConceptFringe.CONCEPT_FRINGE_OUTER, answered = 1))
        assertEquals("In progress", nodeStateLabel(ConceptFringe.CONCEPT_FRINGE_OUTER, answered = 9))
        // Inner fringe is always "Mastered"; locked is always "Locked".
        assertEquals("Mastered", nodeStateLabel(ConceptFringe.CONCEPT_FRINGE_INNER, answered = 0))
        assertEquals("Mastered", nodeStateLabel(ConceptFringe.CONCEPT_FRINGE_INNER, answered = 5))
        assertEquals("Locked", nodeStateLabel(ConceptFringe.CONCEPT_FRINGE_LOCKED, answered = 0))
    }

    @Test
    fun `unattempted topic shows Not started`() {
        assertEquals("Not started", displayPriority(answered = 0, score = 0.5f))
        assertEquals("Not started", displayPriority(answered = 0, score = 0.25f))
    }

    @Test
    fun `attempted topic shows a formatted number`() {
        val text = displayPriority(answered = 3, score = 0.72f)
        assertTrue(text != "Not started", "attempted topic should show a number, was '$text'")
        // Locale-agnostic: two decimals of 0.72.
        assertTrue(text.matches(Regex("""0[.,]72""")), "unexpected format '$text'")
    }

    @Test
    fun `unattempted topic shows Not started for mastery, never the 0_5 prior`() {
        // The backend hands back a default prior (0.5/0.25) for topics with no evidence; we must not
        // surface that as if it were real mastery.
        assertEquals("Not started", displayMastery(answered = 0, mastery = 0.5f))
        assertEquals("Not started", displayMastery(answered = 0, mastery = 0.25f))
    }

    @Test
    fun `attempted topic shows a formatted mastery number`() {
        val text = displayMastery(answered = 5, mastery = 0.5f)
        assertTrue(text != "Not started", "attempted topic should show a number, was '$text'")
        assertTrue(text.matches(Regex("""0[.,]50""")), "unexpected format '$text'")
    }

    @Test
    fun `lattice layout places foundations at layer 0 and chains deepen`() {
        val nodes =
            listOf(
                "Bio::DNA",
                "Bio::Genetics",
                "Bio::Eukaryotic_Cell",
                "Biochem::Amino_Acids",
                "Biochem::Peptides_and_Proteins",
                "Biochem::Protein_Structure_and_Function",
                "Biochem::Enzymes",
                "Biochem::Bioenergetics",
                "Biochem::Glycolysis",
                "Biochem::Citric_Acid_Cycle",
            )
        val edges =
            listOf(
                "Bio::DNA" to "Bio::Genetics",
                "Biochem::Amino_Acids" to "Biochem::Peptides_and_Proteins",
                "Biochem::Peptides_and_Proteins" to "Biochem::Protein_Structure_and_Function",
                "Biochem::Protein_Structure_and_Function" to "Biochem::Enzymes",
                "Biochem::Enzymes" to "Biochem::Bioenergetics",
                "Biochem::Bioenergetics" to "Biochem::Glycolysis",
                "Biochem::Glycolysis" to "Biochem::Citric_Acid_Cycle",
                "Bio::Eukaryotic_Cell" to "Biochem::Bioenergetics",
            )
        val layout = computeLatticeLayout(nodes, edges)

        // Foundations (no prerequisites) are layer 0.
        assertEquals(0, layout.getValue("Bio::DNA").layer)
        assertEquals(0, layout.getValue("Bio::Eukaryotic_Cell").layer)
        assertEquals(0, layout.getValue("Biochem::Amino_Acids").layer)
        // Chain deepens by 1 per prerequisite edge.
        assertEquals(1, layout.getValue("Bio::Genetics").layer)
        assertEquals(1, layout.getValue("Biochem::Peptides_and_Proteins").layer)
        assertEquals(2, layout.getValue("Biochem::Protein_Structure_and_Function").layer)
        assertEquals(3, layout.getValue("Biochem::Enzymes").layer)
        // Convergence: Bioenergetics depends on Enzymes(3) and Eukaryotic_Cell(0) -> max+1 = 4.
        assertEquals(4, layout.getValue("Biochem::Bioenergetics").layer)
        assertEquals(5, layout.getValue("Biochem::Glycolysis").layer)
        assertEquals(6, layout.getValue("Biochem::Citric_Acid_Cycle").layer)

        // Every node is placed exactly once.
        assertEquals(nodes.size, layout.size)
        // layerSize is consistent within a layer.
        val layer0 = layout.values.filter { it.layer == 0 }
        assertTrue(layer0.all { it.layerSize == layer0.size })
    }

    @Test
    fun `lattice layout tolerates a prerequisite cycle without looping`() {
        val nodes = listOf("A", "B")
        val edges = listOf("A" to "B", "B" to "A")
        val layout = computeLatticeLayout(nodes, edges)
        assertEquals(2, layout.size)
    }
}
