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

import org.junit.Test
import kotlin.test.assertEquals
import kotlin.test.assertFalse
import kotlin.test.assertTrue

class AddCardConceptMetadataTest {
    @Test
    fun `section is derived from KC domain`() {
        assertEquals(McatTopics.SECTION_BIO_BIOCHEM, McatTopics.sectionForKc("Bio::DNA"))
        assertEquals(McatTopics.SECTION_BIO_BIOCHEM, McatTopics.sectionForKc("Biochem::Glycolysis"))
        assertEquals(McatTopics.SECTION_CHEM_PHYS, McatTopics.sectionForKc("GenChem::Kinetics"))
        assertEquals(McatTopics.SECTION_CHEM_PHYS, McatTopics.sectionForKc("Physics::Optics"))
        assertEquals(McatTopics.SECTION_CHEM_PHYS, McatTopics.sectionForKc("Orgo::Alcohols"))
        assertEquals(McatTopics.SECTION_PSYCH_SOC, McatTopics.sectionForKc("Soc::Memory"))
    }

    @Test
    fun `minimal tags are KC MCAT and Difficulty with auto-derived section`() {
        val tags = buildConceptTags(kc = "Bio::DNA", difficulty = 2)
        assertEquals(listOf("KC::Bio::DNA", "MCAT::Bio_Biochem", "Difficulty::2"), tags)
    }

    @Test
    fun `optional prereq section override and IRT are included`() {
        val tags =
            buildConceptTags(
                kc = "Biochem::Glycolysis",
                prereq = "Biochem::Enzymes",
                sectionOverride = McatTopics.SECTION_CHEM_PHYS,
                difficulty = 4,
                discrimination = 1.0,
                guessing = 0.25,
            )
        assertEquals(
            listOf(
                "KC::Biochem::Glycolysis",
                "Prereq::Biochem::Enzymes",
                "MCAT::Chem_Phys",
                "Difficulty::4",
                "IRT::Discrimination::1",
                "IRT::Guessing::0.25",
            ),
            tags,
        )
    }

    @Test
    fun `difficulty is clamped to 1 to 5`() {
        assertTrue(buildConceptTags(kc = "Bio::DNA", difficulty = 9).contains("Difficulty::5"))
        assertTrue(buildConceptTags(kc = "Bio::DNA", difficulty = 0).contains("Difficulty::1"))
    }

    @Test
    fun `blank prereq is omitted`() {
        val tags = buildConceptTags(kc = "Bio::DNA", prereq = "  ", difficulty = 1)
        assertTrue(tags.none { it.startsWith("Prereq::") })
    }

    @Test
    fun `normalizeConceptTag maps unicode proportion glyph to ascii colons`() {
        assertEquals("KC::Biochem::Glycolysis", normalizeConceptTag("KC∷Biochem∷Glycolysis"))
        assertEquals("Difficulty::1", normalizeConceptTag("Difficulty∷1"))
    }

    @Test
    fun `isSpecificKc accepts only exact leaf KCs, not subjects or free text`() {
        // Specific knowledge components are accepted.
        assertTrue(McatTopics.isSpecificKc("Bio::DNA"))
        assertTrue(McatTopics.isSpecificKc("Biochem::Glycolysis"))
        // Whole subject areas are rejected — you can't just "check all of Biology".
        assertFalse(McatTopics.isSpecificKc("Bio"))
        assertFalse(McatTopics.isSpecificKc("Biochem"))
        // Unrecognized free text and blanks are rejected.
        assertFalse(McatTopics.isSpecificKc("Biology"))
        assertFalse(McatTopics.isSpecificKc("Bio::NotAThing"))
        assertFalse(McatTopics.isSpecificKc(""))
        // Whitespace and the Unicode proportion separator are tolerated.
        assertTrue(McatTopics.isSpecificKc("  Bio::DNA  "))
        assertTrue(McatTopics.isSpecificKc("Bio∷DNA"))
    }

    @Test
    fun `topic list is non-empty and canonical demo KCs are present`() {
        assertTrue(McatTopics.ALL_KCS.size > 50)
        assertTrue(McatTopics.ALL_KCS.contains("Bio::DNA"))
        assertTrue(McatTopics.ALL_KCS.contains("Biochem::Citric_Acid_Cycle"))
    }

    @Test
    fun `kc badge label combines KC leaf and section, and hides when no KC`() {
        assertEquals("DNA · Bio/Biochem", kcBadgeLabel(listOf("KC::Bio::DNA", "MCAT::Bio_Biochem")))
        assertEquals("Amino Acids", kcBadgeLabel(listOf("KC::Biochem::Amino_Acids")))
        assertEquals(null, kcBadgeLabel(listOf("leech", "marked")))
    }
}
