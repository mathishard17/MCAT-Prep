# Copyright: Ankitects Pty Ltd and contributors
# License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html

import pytest

from aqt.mcat_ai import OpenAIError, parse_tag_suggestion

ALLOWED = ["Bio::DNA", "Biochem::Glycolysis", "Physics::Kinematics"]


def test_parses_valid_suggestion():
    out = parse_tag_suggestion(
        '{"kcs": ["Bio::DNA"], "difficulty": 2, "reasoning": "Application", '
        '"discrimination": 1.4, "guessing": 0.2}',
        ALLOWED,
    )
    assert out["kcs"] == ["Bio::DNA"]
    assert out["difficulty"] == 2
    assert out["reasoning"] == "Application"
    assert out["discrimination"] == 1.4
    assert out["guessing"] == 0.2


def test_drops_hallucinated_kcs_but_keeps_valid_ones():
    out = parse_tag_suggestion(
        '{"kcs": ["Bio::Made_Up", "Biochem::Glycolysis"], "difficulty": 3}', ALLOWED
    )
    assert out["kcs"] == ["Biochem::Glycolysis"]


def test_raises_when_no_valid_kc():
    with pytest.raises(OpenAIError):
        parse_tag_suggestion('{"kcs": ["Bogus::Nope"], "difficulty": 3}', ALLOWED)


def test_clamps_difficulty_and_irt_ranges():
    out = parse_tag_suggestion(
        '{"kcs": ["Bio::DNA"], "difficulty": 9, "discrimination": 5, "guessing": -1}',
        ALLOWED,
    )
    assert out["difficulty"] == 5
    assert out["discrimination"] == 3.0
    assert out["guessing"] == 0.0
    assert parse_tag_suggestion('{"kcs": ["Bio::DNA"], "difficulty": 0}', ALLOWED)[
        "difficulty"
    ] == 1


def test_unknown_reasoning_defaults_to_conceptual():
    out = parse_tag_suggestion(
        '{"kcs": ["Bio::DNA"], "difficulty": 3, "reasoning": "Vibes"}', ALLOWED
    )
    assert out["reasoning"] == "Conceptual"


def test_strips_code_fences_and_surrounding_prose():
    fenced = '```json\n{"kcs": ["Bio::DNA"], "difficulty": 3}\n```'
    assert parse_tag_suggestion(fenced, ALLOWED)["kcs"] == ["Bio::DNA"]
    prose = 'Sure: {"kcs": ["Physics::Kinematics"], "difficulty": 4} — hope that helps'
    assert parse_tag_suggestion(prose, ALLOWED)["kcs"] == ["Physics::Kinematics"]


def test_coerces_singular_kc_and_string_difficulty():
    out = parse_tag_suggestion('{"kc": "Bio::DNA", "difficulty": "2"}', ALLOWED)
    assert out["kcs"] == ["Bio::DNA"]
    assert out["difficulty"] == 2


def test_caps_at_two_kcs():
    out = parse_tag_suggestion(
        '{"kcs": ["Bio::DNA", "Biochem::Glycolysis", "Physics::Kinematics"], '
        '"difficulty": 3}',
        ALLOWED,
    )
    assert len(out["kcs"]) == 2


def test_raises_on_non_json_reply():
    with pytest.raises(OpenAIError):
        parse_tag_suggestion("I can't help with that.", ALLOWED)
