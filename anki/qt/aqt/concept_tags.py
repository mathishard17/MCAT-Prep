# Copyright: Ankitects Pty Ltd and contributors
# License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html

"""Pure helpers for Concept Scheduler card metadata tags.

These are intentionally free of Qt imports so the tag rules can be unit tested
without constructing an editor. The Add Cards UI in ``aqt.editor`` builds the
metadata panel on top of them.
"""

from __future__ import annotations

from collections.abc import Iterable

# Tag prefixes owned by the Concept Scheduler metadata panel. Tags with these
# prefixes are managed by the panel and are stripped when starting a fresh note.
CONCEPT_METADATA_TAG_PREFIXES = (
    "KC::",
    "MCAT::",
    "Difficulty::",
    "IRT::Discrimination::",
    "IRT::Guessing::",
    "Reasoning::",
)


def normalize_concept_tag(tag: str) -> str:
    """Normalize the Unicode double-colon separator to ASCII ``::``."""
    return tag.replace("∷", "::")


def concept_tags_meet_add_requirements(tags: Iterable[str]) -> bool:
    """True when tags include at least one non-empty ``KC::`` and ``Difficulty::`` tag."""
    normalized_tags = [normalize_concept_tag(tag) for tag in tags]
    return any(
        tag.startswith("KC::") and tag.removeprefix("KC::") for tag in normalized_tags
    ) and any(
        tag.startswith("Difficulty::") and tag.removeprefix("Difficulty::")
        for tag in normalized_tags
    )


def derived_mcat_sections_for_topics(topics: Iterable[str]) -> list[str]:
    """Map KC topics to the MCAT sections they can contribute evidence to.

    Overlap is intentional: e.g. ``Bio::`` topics can support Bio/Biochem,
    Chem/Phys, and Psych/Soc. Order is preserved and duplicates are dropped.
    """
    sections: list[str] = []
    for topic in topics:
        if topic.startswith("Bio::"):
            topic_sections = ["Bio_Biochem", "Chem_Phys", "Psych_Soc"]
        elif topic.startswith("Biochem::"):
            topic_sections = ["Bio_Biochem", "Chem_Phys"]
        elif topic.startswith(("GenChem::", "Physics::", "Orgo::")):
            topic_sections = ["Chem_Phys"]
        elif topic.startswith("PsychSoc::"):
            topic_sections = ["Psych_Soc"]
        elif topic.startswith("CARS::"):
            topic_sections = ["CARS"]
        else:
            continue

        for section in topic_sections:
            if section not in sections:
                sections.append(section)

    return sections
