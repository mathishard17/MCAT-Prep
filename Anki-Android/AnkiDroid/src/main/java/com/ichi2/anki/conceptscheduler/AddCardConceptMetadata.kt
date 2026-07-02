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

import java.math.BigDecimal

/**
 * Concept Scheduler metadata for the Add Card editor: the MCAT knowledge-component (KC) topic map, the
 * KC → MCAT-section mapping, and the tag-building logic. Kept free of Android dependencies so it is
 * unit-testable.
 *
 * Cards are tagged for the backend concept scheduler with:
 * `KC::<domain>::<component>`, optional `Prereq::<domain>::<component>`, `MCAT::<section>`,
 * `Difficulty::<1-5>`, optional `IRT::Discrimination::<x>` and `IRT::Guessing::<x>`.
 * The KC list is bundled from `../anki/added features/mcat.md` (there is no backend RPC for it).
 */
object McatTopics {
    /** MCAT section tokens as used in the `MCAT::` tag (e.g. `MCAT::Bio_Biochem`). */
    const val SECTION_BIO_BIOCHEM = "Bio_Biochem"
    const val SECTION_CHEM_PHYS = "Chem_Phys"
    const val SECTION_PSYCH_SOC = "Psych_Soc"
    const val SECTION_CARS = "CARS"

    /** All selectable MCAT sections (for the manual-override dropdown). */
    val SECTIONS = listOf(SECTION_BIO_BIOCHEM, SECTION_CHEM_PHYS, SECTION_PSYCH_SOC, SECTION_CARS)

    /** Every KC id, grouped by domain, sourced from the MCAT topic map. */
    val ALL_KCS: List<String> =
        listOf(
            // Bio
            "Bio::Biotechnology", "Bio::Circulatory_System", "Bio::DNA", "Bio::Digestive_System",
            "Bio::Embryology", "Bio::Endocrine_System", "Bio::Eukaryotic_Cell", "Bio::Evolution",
            "Bio::Genetics", "Bio::Immune_System", "Bio::Lymphatic_System", "Bio::Muscular_System",
            "Bio::Nervous_System", "Bio::Prokaryotes_vs_Eukaryotes", "Bio::Reproductive_System",
            "Bio::Respiratory_System", "Bio::Skeletal_System", "Bio::Skin_System", "Bio::Viruses",
            // Biochem
            "Biochem::Amino_Acids", "Biochem::Bioenergetics", "Biochem::Carbohydrates_and_Lipids",
            "Biochem::Citric_Acid_Cycle", "Biochem::Enzymes", "Biochem::Gluconeogenesis",
            "Biochem::Glycolysis", "Biochem::Lipid_Metabolism", "Biochem::Metabolic_Regulation",
            "Biochem::Nucleotides_and_Nucleic_Acids", "Biochem::Oxidative_Phosphorylation",
            "Biochem::Pentose_Phosphate_Pathway", "Biochem::Peptides_and_Proteins",
            "Biochem::Protein_Structure_and_Function",
            // GenChem
            "GenChem::Acid_Base_Equilibria", "GenChem::Covalent_Bond", "GenChem::Electrochemistry",
            "GenChem::Equilibrium", "GenChem::Gas_Phase", "GenChem::Intermolecular_Forces",
            "GenChem::Ions_in_Solutions", "GenChem::Kinetics", "GenChem::Liquid_Phase",
            "GenChem::Molecular_Structure", "GenChem::Molecules", "GenChem::Solubility",
            "GenChem::Stoichiometry", "GenChem::Thermochemistry", "GenChem::Titration", "GenChem::Water",
            // Orgo
            "Orgo::Acid_Derivatives", "Orgo::Alcohols", "Orgo::Aldehydes_and_Ketones",
            "Orgo::Carboxylic_Acids", "Orgo::Functional_Groups", "Orgo::Hybridization",
            "Orgo::Mass_Spectrometry", "Orgo::Molecular_Structure_and_Absorption_Spectra",
            "Orgo::Nomenclature", "Orgo::Nucleophilic_Substitution", "Orgo::Phenols",
            "Orgo::Polycyclic_and_Heterocyclic_Aromatic_Compounds",
            "Orgo::Separations_and_Purifications", "Orgo::Stereochemistry",
            // Physics
            "Physics::Atomic_and_Chemical_Behavior", "Physics::Atoms", "Physics::Circuit_Elements",
            "Physics::Electrical_Circuits", "Physics::Electromagnetic_Radiation",
            "Physics::Electronic_Structure", "Physics::Electrostatics", "Physics::Energy",
            "Physics::Equilibrium", "Physics::Fluids", "Physics::Force", "Physics::Light",
            "Physics::Magnetism", "Physics::Matter", "Physics::Nuclear_Decay", "Physics::Optics",
            "Physics::Periodic_Motion", "Physics::Sound", "Physics::Thermodynamics",
            "Physics::Translational_Motion", "Physics::Work",
            // Psych/Soc
            "Soc::Attention", "Soc::Attitudes_and_Beliefs", "Soc::Biological_and_Social_Factors",
            "Soc::Cognition", "Soc::Consciousness", "Soc::Culture", "Soc::Emotion",
            "Soc::Health_Disparities", "Soc::Language", "Soc::Memory", "Soc::Motivation",
            "Soc::Perception", "Soc::Personality", "Soc::Poverty", "Soc::Prejudice_and_Bias",
            "Soc::Psychological_Disorders", "Soc::Self_and_Identity", "Soc::Sensory_Processing",
            "Soc::Social_Class", "Soc::Social_Mobility", "Soc::Stereotypes", "Soc::Stratification",
            "Soc::Stress", "Soc::The_Senses",
        )

    /**
     * True only when [kc] is a specific knowledge component from the MCAT map (e.g. `Bio::DNA`), not a
     * whole subject area (`Bio`) or unrecognized free text. A card must target one specific KC, so the
     * Add-Card picker rejects anything that isn't an exact entry in [ALL_KCS]. Tolerates the Unicode
     * `∷` separator and surrounding whitespace.
     */
    fun isSpecificKc(kc: String): Boolean = normalizeConceptTag(kc.trim()) in ALL_KCS

    /**
     * Maps a KC id to its default MCAT section token, from the KC's domain prefix. Used to auto-derive
     * the `MCAT::` tag when the user doesn't manually override the section.
     */
    fun sectionForKc(kc: String): String =
        when (kc.substringBefore("::")) {
            "Bio", "Biochem" -> SECTION_BIO_BIOCHEM
            "GenChem", "Chem", "Orgo", "Physics", "Phys" -> SECTION_CHEM_PHYS
            "Soc", "Psych" -> SECTION_PSYCH_SOC
            "CARS" -> SECTION_CARS
            else -> SECTION_BIO_BIOCHEM
        }
}

/**
 * Normalizes the Unicode "proportion" glyph (∷, U+2237) that some sources use for the KC separator back
 * to ASCII `::`, so manually-typed tags are accepted (parity with the desktop `normalize_concept_tag`).
 */
fun normalizeConceptTag(tag: String): String = tag.replace('∷'.toString(), "::")

/**
 * Builds the Concept Scheduler tag list for a card from Add-Card panel selections.
 *
 * @param kc required target KC id, e.g. `Bio::DNA`
 * @param prereq optional prerequisite KC id
 * @param sectionOverride optional MCAT section token; when null the section is derived from [kc]
 * @param difficulty 1..5
 * @param discrimination optional IRT discrimination
 * @param guessing optional IRT guessing
 */
fun buildConceptTags(
    kc: String,
    prereq: String? = null,
    sectionOverride: String? = null,
    difficulty: Int,
    discrimination: Double? = null,
    guessing: Double? = null,
): List<String> =
    buildList {
        add("KC::$kc")
        if (!prereq.isNullOrBlank()) add("Prereq::$prereq")
        add("MCAT::${sectionOverride ?: McatTopics.sectionForKc(kc)}")
        add("Difficulty::${difficulty.coerceIn(1, 5)}")
        discrimination?.let { add("IRT::Discrimination::${formatIrt(it)}") }
        guessing?.let { add("IRT::Guessing::${formatIrt(it)}") }
    }

/** Formats an IRT double without locale decimal separators or trailing zeros (e.g. 1.0 -> "1", 0.25 -> "0.25"). */
private fun formatIrt(value: Double): String = BigDecimal.valueOf(value).stripTrailingZeros().toPlainString()
