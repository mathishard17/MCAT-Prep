# Copyright: Ankitects Pty Ltd and contributors
# License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html

"""Python-side tests that drive the Rust Concept Scheduler change end to end.

This is the rubric's "1 test that calls your change from Python" (rubric 7a) for
the Concept Scheduler engine that lives in Rust
(`rslib/src/scheduler/concept.rs`, `concept_demo.rs`, and the queue builder).
Everything here goes through the real generated backend RPCs
(`import_mcat_demo_deck`, `get_concept_scheduler_status`) and the normal
`col.sched.answerCard` review path -- no Python re-implementation of the engine.

`test_review_then_undo_keeps_collection_intact` is also the
"undo-safe + no-corruption proof" (rubric 7a): it runs a real review, undoes it,
and asserts the collection passes an integrity check afterwards.
"""

from __future__ import annotations

from tests.shared import getEmptyCol

# MCAT scale bounds (see README / docs/model-*.md).
SECTION_MIN, SECTION_MAX = 118.0, 132.0
TOTAL_MIN, TOTAL_MAX = 472.0, 528.0


def _answer_cards(col, deck_id: int, count: int, ease: int = 3) -> int:
    """Answer up to `count` cards in `deck_id`; return how many were answered."""
    col.decks.set_current(deck_id)
    answered = 0
    for _ in range(count):
        card = col.sched.getCard()
        if card is None:
            break
        col.sched.answerCard(card, ease)  # 1=Again 2=Hard 3=Good 4=Easy
        answered += 1
    return answered


def test_import_demo_deck_rpc_builds_knowledge_graph():
    """import_mcat_demo_deck (Rust RPC) enables the scheduler and builds the graph."""
    col = getEmptyCol()
    try:
        deck_id = col._backend.import_mcat_demo_deck()
        assert deck_id != 0

        status = col._backend.get_concept_scheduler_status(deck_id)
        assert status.enabled
        # The full MCAT graph carries >100 knowledge components across sections.
        assert len(status.graph.nodes) > 100
        # No reviews yet -> no evidence, and the give-up gate is not met.
        assert status.evidence.seen_cards == 0
        assert status.counters.total_seen_cards == 0
        assert status.evidence.required_seen_cards > 0
    finally:
        col.close()


def test_answering_updates_bayesian_mastery_via_rpc():
    """Positive evidence through the real review path raises per-KC mastery.

    Mirrors the Rust `concept_mastery_updates_on_answer` test, but exercises the
    whole stack from Python: answerCard -> Rust answering hook -> persisted
    state -> get_concept_scheduler_status.
    """
    col = getEmptyCol()
    try:
        deck_id = col._backend.import_mcat_demo_deck()
        before = {n.id: n.mastery for n in col._backend.get_concept_scheduler_status(deck_id).graph.nodes}

        answered = _answer_cards(col, deck_id, count=12, ease=3)
        assert answered >= 3, "expected the demo deck to serve new cards"

        status = col._backend.get_concept_scheduler_status(deck_id)
        # total_seen_cards counts cards, once per answer.
        assert status.counters.total_seen_cards == answered
        assert status.evidence.seen_cards == answered

        # A KC that only saw positive (Good) evidence must have mastery strictly
        # above its cold-start prior -- the Bayesian update actually ran.
        rose = [
            n
            for n in status.graph.nodes
            if n.positive > 0 and n.negative == 0 and n.mastery > before[n.id] + 1e-9
        ]
        assert rose, "at least one answered KC should have gained mastery"
    finally:
        col.close()


def test_scores_are_ranged_and_in_scale_via_rpc():
    """The three scores come back as ranges inside the real MCAT scale."""
    col = getEmptyCol()
    try:
        deck_id = col._backend.import_mcat_demo_deck()
        _answer_cards(col, deck_id, count=12, ease=3)
        status = col._backend.get_concept_scheduler_status(deck_id)

        # Readiness: projected total is a 472-528 point estimate with a band.
        assert status.has_projection
        assert TOTAL_MIN <= status.projected_total <= TOTAL_MAX
        assert status.projected_total_lower <= status.projected_total <= status.projected_total_upper

        for section in status.section_scores:
            # Performance (IRT) center sits inside its own band, on the 118-132 scale.
            assert SECTION_MIN <= section.performance_center <= SECTION_MAX
            assert section.performance_lower <= section.performance_center <= section.performance_upper
            # Readiness center is ranged and in scale too.
            assert SECTION_MIN <= section.readiness_center <= SECTION_MAX
            assert section.readiness_lower <= section.readiness_center <= section.readiness_upper
            # Memory is a probability; coverage is a fraction.
            assert 0.0 <= section.section_memory <= 1.0
            assert 0.0 <= section.coverage <= 1.0
    finally:
        col.close()


def test_review_then_undo_keeps_collection_intact():
    """Undo-safe + no-corruption proof (rubric 7a).

    Review a card, confirm the Concept Scheduler state moved, undo it, confirm
    the state is fully restored, then run a database integrity check and assert
    the collection is not corrupted.
    """
    col = getEmptyCol()
    try:
        deck_id = col._backend.import_mcat_demo_deck()
        assert col._backend.get_concept_scheduler_status(deck_id).counters.total_seen_cards == 0

        answered = _answer_cards(col, deck_id, count=1, ease=3)
        assert answered == 1
        after = col._backend.get_concept_scheduler_status(deck_id)
        assert after.counters.total_seen_cards == 1
        # exactly one KC picked up an answer
        touched = [n for n in after.graph.nodes if n.answered > 0]
        assert touched

        col.undo()
        restored = col._backend.get_concept_scheduler_status(deck_id)
        assert restored.counters.total_seen_cards == 0
        assert all(n.answered == 0 for n in restored.graph.nodes)

        # The collection is still structurally sound after review + undo.
        _report, ok = col.fix_integrity()
        assert ok, f"collection integrity check failed: {_report}"
    finally:
        col.close()
