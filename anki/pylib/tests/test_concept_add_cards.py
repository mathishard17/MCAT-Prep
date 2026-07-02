# Copyright: Ankitects Pty Ltd and contributors
# License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html

"""Findability contract for Concept Scheduler demo cards.

Track F of the MCAT expansion plan: after a card is added with Concept Scheduler
metadata, it must land in the chosen deck and be locatable afterwards (by KC tag
or by note id, which is what the Add Cards "Browse Added" action uses).
"""

from anki.collection import SearchNode
from tests.shared import getEmptyCol

CONCEPT_TAGS = [
    "KC::Bio::DNA",
    "Difficulty::2",
    "MCAT::Bio_Biochem",
    "IRT::Discrimination::1.0",
    "IRT::Guessing::0.25",
    "Reasoning::Conceptual",
]


def test_concept_card_lands_in_selected_deck_and_is_findable():
    col = getEmptyCol()
    try:
        deck_id = col.decks.id("MCAT Demo")

        note = col.newNote()
        note["Front"] = "What sugar is in the DNA backbone?"
        note["Back"] = "Deoxyribose"
        note.tags = CONCEPT_TAGS[:]

        col.add_note(note, deck_id)

        # the note got a real id and every card landed in the selected deck
        assert note.id != 0
        cards = note.cards()
        assert cards
        assert all(card.did == deck_id for card in cards)

        # locatable by KC tag (browser search) ...
        by_kc = col.find_notes(
            col.build_search_string(SearchNode(tag="KC::Bio::DNA"))
        )
        assert note.id in by_kc

        # ... and by note id, which the "Browse Added" action uses
        by_nid = col.find_notes(col.build_search_string(SearchNode(nid=note.id)))
        assert by_nid == [note.id]

        # every metadata tag survived the add
        for tag in CONCEPT_TAGS:
            assert tag in note.tags
    finally:
        col.close()
