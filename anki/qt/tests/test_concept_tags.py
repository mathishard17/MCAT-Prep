# Copyright: Ankitects Pty Ltd and contributors
# License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html

from aqt.concept_tags import (
    concept_tags_meet_add_requirements,
    derived_mcat_sections_for_topics,
    normalize_concept_tag,
)


def test_normalize_unicode_double_colon():
    assert normalize_concept_tag("KC∷Bio∷DNA") == "KC::Bio::DNA"
    assert normalize_concept_tag("KC::Bio::DNA") == "KC::Bio::DNA"


def test_requirements_need_kc_and_difficulty():
    assert concept_tags_meet_add_requirements(["KC::Bio::DNA", "Difficulty::2"])
    # unicode separators are normalized before checking
    assert concept_tags_meet_add_requirements(["KC∷Bio∷DNA", "Difficulty∷2"])


def test_requirements_reject_incomplete_metadata():
    assert not concept_tags_meet_add_requirements(["KC::Bio::DNA"])
    assert not concept_tags_meet_add_requirements(["Difficulty::2"])
    # empty values after the prefix do not count
    assert not concept_tags_meet_add_requirements(["KC::", "Difficulty::"])
    assert not concept_tags_meet_add_requirements([])


def test_mcat_section_derivation_allows_overlap():
    assert derived_mcat_sections_for_topics(["Bio::DNA"]) == [
        "Bio_Biochem",
        "Chem_Phys",
        "Psych_Soc",
    ]
    assert derived_mcat_sections_for_topics(["Biochem::Glycolysis"]) == [
        "Bio_Biochem",
        "Chem_Phys",
    ]
    assert derived_mcat_sections_for_topics(["GenChem::Acids_and_Bases"]) == [
        "Chem_Phys"
    ]
    assert derived_mcat_sections_for_topics(["CARS::Reasoning"]) == ["CARS"]


def test_mcat_section_derivation_dedupes_and_skips_unknown():
    assert derived_mcat_sections_for_topics(["Bio::DNA", "Biochem::Glycolysis"]) == [
        "Bio_Biochem",
        "Chem_Phys",
        "Psych_Soc",
    ]
    assert derived_mcat_sections_for_topics(["Unknown::Thing"]) == []
