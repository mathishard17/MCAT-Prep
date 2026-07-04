# Copyright: Ankitects Pty Ltd and contributors
# License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html

"""CARS Practice with AI coaching (sibling of the Add Cards AI tagging).

Loads original, synthetic MCAT CARS passages from ``added features/passages-cars.md``,
lets the student answer the passage questions, and — using the user's own OpenAI
key (Track H, see ``mcat_ai``) — coaches them on *why* the correct answer is best
and what makes a wrong choice a classic CARS trap. All coaching is grounded in the
passage text; studying works offline (the authored Explanation always shows).
"""

from __future__ import annotations

import html
import re
from pathlib import Path
from typing import Any

from aqt.qt import *
from aqt.utils import showInfo, showWarning, tooltip

CHOICE_KEYS = ("A", "B", "C", "D")

CARS_COACH_SYSTEM = (
    "You are an MCAT CARS (Critical Analysis and Reasoning Skills) coach. Base "
    "every claim ONLY on the passage provided; never use outside knowledge or "
    "invent facts. Explain concisely: (a) why the correct choice is best, pointing "
    "to what the passage actually says; and (b) if the student picked a wrong "
    "answer, what makes that choice a classic CARS trap — name the trap type "
    "(e.g. out-of-scope, too extreme/absolute, opposite/reversal, distortion, "
    "half-right, or plausible-but-unsupported). Finish with one transferable "
    "takeaway. Keep it under about 150 words and address the student as 'you'."
)


def parse_cars_passages(md: str) -> list[dict[str, Any]]:
    """Parse ``passages-cars.md`` into passages with their questions.

    Pure/offline so it can be unit tested. Each passage is a dict with ``id``,
    ``source``, ``passage`` (paragraphs joined by blank lines), and ``questions``;
    each question has ``id``, ``stem``, ``choices`` (A-D), ``correct``, ``explanation``.
    """
    passages: list[dict[str, Any]] = []
    passage: dict[str, Any] | None = None
    question: dict[str, Any] | None = None
    capturing_passage = False
    passage_buf: list[str] = []

    def finish_passage_text() -> None:
        nonlocal capturing_passage, passage_buf
        if passage is not None and passage_buf:
            text = "\n".join(passage_buf)
            paras = [
                re.sub(r"[ \t]+", " ", part).strip()
                for part in re.split(r"\n[ \t]*\n", text)
            ]
            passage["passage"] = "\n\n".join(p for p in paras if p)
        capturing_passage = False
        passage_buf = []

    def finish_question() -> None:
        nonlocal question
        if passage is not None and question is not None:
            passage["questions"].append(question)
        question = None

    for raw in md.splitlines():
        line = raw.rstrip()

        if line.startswith("## CARS-P"):
            finish_passage_text()
            finish_question()
            if passage is not None:
                passages.append(passage)
            passage = {
                "id": line[3:].strip(),
                "source": "",
                "passage": "",
                "questions": [],
            }
            continue
        if passage is None:
            continue
        if line.startswith("### "):
            finish_passage_text()
            finish_question()
            question = {
                "id": line[4:].strip(),
                "stem": "",
                "choices": {},
                "correct": "",
                "explanation": "",
            }
            continue

        bullet = re.match(r"- \*\*(.+?):\*\*\s?(.*)$", line)
        if bullet:
            key, value = bullet.group(1).strip(), bullet.group(2)
            if key == "Passage":
                finish_passage_text()
                capturing_passage = True
                passage_buf = [value]
                continue
            capturing_passage = False
            if question is None:
                if key == "Source":
                    passage["source"] = value.strip()
                continue
            if key == "Stem":
                question["stem"] = value.strip()
            elif key in CHOICE_KEYS:
                question["choices"][key] = value.strip()
            elif key == "Correct":
                letter = value.strip().strip("*").strip()
                question["correct"] = letter[:1].upper()
            elif key == "Explanation":
                question["explanation"] = value.strip()
            continue

        if capturing_passage:
            passage_buf.append(line)

    finish_passage_text()
    finish_question()
    if passage is not None:
        passages.append(passage)

    # Keep only well-formed passages (some text and at least one complete question).
    return [
        p
        for p in passages
        if p["passage"]
        and any(q["stem"] and len(q["choices"]) >= 2 and q["correct"] for q in p["questions"])
    ]


def cars_coach_messages(
    passage: str,
    stem: str,
    choices: dict[str, str],
    correct: str,
    chosen: str,
) -> list[dict[str, str]]:
    """Build the grounded coaching prompt for one answered CARS question."""
    choices_block = "\n".join(f"{key}. {choices.get(key, '')}" for key in CHOICE_KEYS)
    if chosen and chosen == correct:
        verdict = f"The student chose {chosen}, which is correct."
    elif chosen:
        verdict = f"The student chose {chosen}. The correct answer is {correct}."
    else:
        verdict = f"The correct answer is {correct}."
    user = (
        f"PASSAGE:\n{passage}\n\n"
        f"QUESTION:\n{stem}\n\n"
        f"CHOICES:\n{choices_block}\n\n"
        f"{verdict}\n\nCoach the student."
    )
    return [
        {"role": "system", "content": CARS_COACH_SYSTEM},
        {"role": "user", "content": user},
    ]


def _find_passages_file() -> Path | None:
    """Locate ``added features/passages-cars.md`` in the repo tree (dev builds).

    Installed builds won't ship the folder; the dialog then reports that no
    passages are available rather than failing.
    """
    starts = [Path(__file__).resolve(), Path.cwd().resolve()]
    seen: set[Path] = set()
    for start in starts:
        for directory in [start, *start.parents]:
            if directory in seen:
                continue
            seen.add(directory)
            candidate = directory / "added features" / "passages-cars.md"
            if candidate.is_file():
                return candidate
    return None


def load_cars_passages() -> list[dict[str, Any]]:
    path = _find_passages_file()
    if path is None:
        return []
    try:
        return parse_cars_passages(path.read_text(encoding="utf-8"))
    except OSError:
        return []


class CarsPracticeDialog(QDialog):
    def __init__(self, mw: Any, passages: list[dict[str, Any]]) -> None:
        super().__init__(mw)
        self.mw = mw
        self.passages = passages
        self._answered = False
        self.setWindowTitle("CARS Practice (AI coaching)")
        self.setMinimumSize(680, 620)
        self._build()
        self._load_passage(0)

    def _build(self) -> None:
        layout = QVBoxLayout(self)

        top = QHBoxLayout()
        top.addWidget(QLabel("Passage:"))
        self.passage_combo = QComboBox()
        for index, passage in enumerate(self.passages):
            self.passage_combo.addItem(passage["id"], index)
        qconnect(self.passage_combo.currentIndexChanged, self._on_passage_changed)
        top.addWidget(self.passage_combo, 1)
        layout.addLayout(top)

        self.source_label = QLabel("")
        self.source_label.setWordWrap(True)
        self.source_label.setStyleSheet("color: palette(mid); font-size: 11px;")
        layout.addWidget(self.source_label)

        self.passage_view = QTextBrowser()
        self.passage_view.setOpenExternalLinks(False)
        layout.addWidget(self.passage_view, 3)

        qrow = QHBoxLayout()
        qrow.addWidget(QLabel("Question:"))
        self.question_combo = QComboBox()
        qconnect(self.question_combo.currentIndexChanged, self._on_question_changed)
        qrow.addWidget(self.question_combo, 1)
        layout.addLayout(qrow)

        self.stem_label = QLabel("")
        self.stem_label.setWordWrap(True)
        self.stem_label.setStyleSheet("font-weight: 600;")
        layout.addWidget(self.stem_label)

        self.choice_group = QButtonGroup(self)
        self.choice_buttons: dict[str, QRadioButton] = {}
        for key in CHOICE_KEYS:
            button = QRadioButton("")
            self.choice_group.addButton(button)
            self.choice_buttons[key] = button
            layout.addWidget(button)

        actions = QHBoxLayout()
        self.check_btn = QPushButton("Check answer")
        qconnect(self.check_btn.clicked, self._on_check)
        self.coach_btn = QPushButton("Coach me with AI")
        qconnect(self.coach_btn.clicked, self._on_coach)
        self.coach_btn.setEnabled(False)
        actions.addWidget(self.check_btn)
        actions.addWidget(self.coach_btn)
        actions.addStretch()
        layout.addLayout(actions)

        self.feedback = QTextBrowser()
        self.feedback.setMinimumHeight(150)
        layout.addWidget(self.feedback, 2)

        buttons = QDialogButtonBox(QDialogButtonBox.StandardButton.Close)
        qconnect(buttons.rejected, self.reject)
        layout.addWidget(buttons)

    # --- data loading ---------------------------------------------------------
    def _current_passage(self) -> dict[str, Any]:
        return self.passages[self.passage_combo.currentData()]

    def _current_question(self) -> dict[str, Any] | None:
        passage = self._current_passage()
        index = self.question_combo.currentData()
        if index is None or index >= len(passage["questions"]):
            return None
        return passage["questions"][index]

    def _load_passage(self, index: int) -> None:
        self.passage_combo.setCurrentIndex(index)

    def _on_passage_changed(self, _index: int = 0) -> None:
        passage = self._current_passage()
        self.source_label.setText(passage.get("source", ""))
        paragraphs = "".join(
            f"<p>{html.escape(par)}</p>" for par in passage["passage"].split("\n\n")
        )
        self.passage_view.setHtml(paragraphs)
        self.question_combo.blockSignals(True)
        self.question_combo.clear()
        for qi, question in enumerate(passage["questions"]):
            self.question_combo.addItem(question["id"], qi)
        self.question_combo.blockSignals(False)
        self.question_combo.setCurrentIndex(0)
        self._on_question_changed(0)

    def _on_question_changed(self, _index: int = 0) -> None:
        self._answered = False
        self.coach_btn.setEnabled(False)
        self.feedback.setHtml("")
        self.choice_group.setExclusive(False)
        for button in self.choice_buttons.values():
            button.setChecked(False)
        self.choice_group.setExclusive(True)
        question = self._current_question()
        if question is None:
            self.stem_label.setText("")
            for key in CHOICE_KEYS:
                self.choice_buttons[key].setText("")
                self.choice_buttons[key].setVisible(False)
            return
        self.stem_label.setText(question["stem"])
        for key in CHOICE_KEYS:
            text = question["choices"].get(key)
            button = self.choice_buttons[key]
            if text:
                button.setText(f"{key}.  {text}")
                button.setVisible(True)
            else:
                button.setText("")
                button.setVisible(False)

    def _selected_choice(self) -> str:
        for key, button in self.choice_buttons.items():
            if button.isChecked():
                return key
        return ""

    # --- actions --------------------------------------------------------------
    def _on_check(self) -> None:
        question = self._current_question()
        if question is None:
            return
        chosen = self._selected_choice()
        if not chosen:
            tooltip("Pick an answer first.", parent=self)
            return
        correct = question["correct"]
        self._answered = True
        self.coach_btn.setEnabled(True)
        verdict = (
            "<b style='color:#2e7d32;'>Correct.</b>"
            if chosen == correct
            else f"<b style='color:#c62828;'>Not quite — you chose {chosen}.</b> "
            f"The correct answer is <b>{correct}</b>."
        )
        explanation = html.escape(question.get("explanation", "")).replace("\n", "<br>")
        self.feedback.setHtml(
            f"<p>{verdict}</p><p><b>Explanation:</b> {explanation}</p>"
            "<p style='color:palette(mid);'>Click <b>Coach me with AI</b> for trap "
            "analysis and a CARS strategy tip.</p>"
        )

    def _on_coach(self) -> None:
        from aqt.mcat_ai import (
            OpenAIClient,
            resolve_openai_key,
            resolve_openai_model,
        )

        question = self._current_question()
        if question is None or not self._answered:
            return
        key = resolve_openai_key(self.mw.pm)
        if not key:
            showInfo(
                "Add your OpenAI API key first (Tools \u2192 MCAT AI). "
                "The written explanation above works without a key.",
                parent=self,
            )
            return
        passage = self._current_passage()
        messages = cars_coach_messages(
            passage["passage"],
            question["stem"],
            question["choices"],
            question["correct"],
            self._selected_choice(),
        )
        client = OpenAIClient(
            key, resolve_openai_model(self.mw.pm), self.mw.pm.network_timeout()
        )
        self.coach_btn.setEnabled(False)
        self.coach_btn.setText("Coaching\u2026")

        def task() -> str:
            return client._chat(messages, max_tokens=500, temperature=0.3)

        def on_done(future: Any) -> None:
            self.coach_btn.setEnabled(True)
            self.coach_btn.setText("Coach me with AI")
            try:
                coaching = future.result()
            except Exception as exc:
                showWarning(f"AI coaching failed: {exc}", parent=self)
                return
            body = html.escape(coaching).replace("\n", "<br>")
            self.feedback.setHtml(
                self.feedback.toHtml()
                + f"<hr><p><b>AI coach:</b><br>{body}</p>"
            )

        self.mw.taskman.run_in_background(task, on_done)


def show_cars_practice_dialog(mw: Any) -> None:
    passages = load_cars_passages()
    if not passages:
        showInfo(
            "No CARS practice passages found. They live in "
            "'added features/passages-cars.md' in the source tree.",
            parent=mw,
        )
        return
    CarsPracticeDialog(mw, passages).exec()
