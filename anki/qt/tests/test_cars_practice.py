# Copyright: Ankitects Pty Ltd and contributors
# License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html

from aqt.cars_practice import cars_coach_messages, parse_cars_passages

SAMPLE = """# CARS Practice

## CARS-P01
- **Discipline:** CARS
- **Source:** Original synthetic passage from Foo, *Bar* (1900), public domain.
- **Passage:** First paragraph, sentence one. Sentence two.

  Second paragraph, sentence three.

### CARS-P01-Q1
- **Stem:** What is the main idea?
- **A:** Alpha.
- **B:** Beta.
- **C:** Gamma.
- **D:** Delta.
- **Correct:** B
- **Explanation:** B is best; A is out of scope.

### CARS-P01-Q2
- **Stem:** Which follows?
- **A:** One.
- **B:** Two.
- **C:** Three.
- **D:** Four.
- **Correct:** C
- **Explanation:** C follows from the second paragraph.

## CARS-P02
- **Source:** Another public-domain source.
- **Passage:** A lone passage with no usable questions.
"""


def test_parses_passages_and_questions():
    passages = parse_cars_passages(SAMPLE)
    # P02 has no complete question, so it is filtered out.
    assert len(passages) == 1
    passage = passages[0]
    assert passage["id"] == "CARS-P01"
    assert passage["source"].startswith("Original synthetic")
    assert len(passage["questions"]) == 2


def test_passage_paragraphs_joined_and_cleaned():
    passage = parse_cars_passages(SAMPLE)[0]["passage"]
    assert "First paragraph, sentence one. Sentence two." in passage
    assert "Second paragraph, sentence three." in passage
    # paragraphs are separated by a blank line
    assert "\n\n" in passage


def test_question_fields():
    q1 = parse_cars_passages(SAMPLE)[0]["questions"][0]
    assert q1["id"] == "CARS-P01-Q1"
    assert set(q1["choices"]) == {"A", "B", "C", "D"}
    assert q1["choices"]["A"] == "Alpha."
    assert q1["correct"] == "B"
    assert "out of scope" in q1["explanation"]


def test_coach_prompt_flags_wrong_choice_as_trap():
    msgs = cars_coach_messages(
        "passage text", "the stem", {"A": "a", "B": "b", "C": "c", "D": "d"}, "B", "A"
    )
    assert msgs[0]["role"] == "system"
    assert "CARS" in msgs[0]["content"]
    user = msgs[1]["content"]
    assert "PASSAGE:" in user and "QUESTION:" in user and "CHOICES:" in user
    assert "chose A" in user and "correct answer is B" in user


def test_coach_prompt_handles_correct_and_blank_choices():
    correct_msg = cars_coach_messages("p", "s", {"A": "a", "B": "b"}, "A", "A")[1][
        "content"
    ]
    assert "which is correct" in correct_msg
    blank_msg = cars_coach_messages("p", "s", {"A": "a", "B": "b"}, "A", "")[1][
        "content"
    ]
    assert "correct answer is A" in blank_msg
