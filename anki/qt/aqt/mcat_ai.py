# Copyright: Ankitects Pty Ltd and contributors
# License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html

"""Bring-your-own OpenAI key for MCAT AI features (Track H).

The user pastes their own OpenAI API key; it is stored per profile only (never
synced, logged, or committed). The reviewer uses this to reword MCAT question
stems on the fly, with a second verification call to confirm the reworded stem
means the same thing as the original.
"""

from __future__ import annotations

import json
import os
import re
import urllib.error
import urllib.request
from pathlib import Path
from typing import Any

from aqt.qt import *
from aqt.utils import showWarning, tooltip

OPENAI_BASE = "https://api.openai.com/v1"
ENV_KEY_NAME = "OPENAI_API_KEY"
ENV_MODEL_NAME = "OPENAI_MODEL"

REWORD_SYSTEM = (
    "You are helping an MCAT student by rephrasing a multiple-choice question "
    "STEM so it tests the same concept with fresh wording. Rewrite the stem so "
    "it asks exactly the same thing and keeps the same correct answer, but uses "
    "different sentence structure or scenario framing. Do NOT change any facts, "
    "numbers, or what is being asked. Do NOT include or reference the answer "
    "choices (no 'A', 'B', 'C', 'D'). Do NOT add hints or new information. "
    "Return ONLY the reworded question stem, with no preamble or quotation marks."
)

EQUIV_SYSTEM = (
    "You verify whether two versions of an MCAT question stem are semantically "
    "equivalent: given identical answer choices, they must ask the same thing "
    "and have the same correct answer. Reply with exactly 'yes' if they are "
    "equivalent, or 'no' otherwise. Reply with only 'yes' or 'no'."
)

REASONING_TYPES = ("Conceptual", "Application", "Data", "ResearchDesign")

SUGGEST_TAGS_SYSTEM = (
    "You tag MCAT flashcards for a spaced-repetition study app. Given a card's "
    "text and a fixed list of allowed Knowledge Component (KC) ids, pick the "
    "single best KC id for the card (or at most two if it genuinely spans two), "
    "an integer difficulty from 1 (easiest) to 5 (hardest), and the dominant "
    "reasoning type. Choose KC ids ONLY from the allowed list and copy them "
    "exactly; never invent an id. Respond with STRICT JSON and nothing else: "
    '{"kcs": ["<KC id>"], "difficulty": 3, "reasoning": "Conceptual", '
    '"discrimination": 1.0, "guessing": 0.25}. The reasoning value must be one '
    "of: Conceptual, Application, Data, ResearchDesign."
)


def _env_search_dirs() -> list[Path]:
    """Directories to look for a `.env` in: cwd and the package tree, plus parents."""
    dirs: list[Path] = []
    seen: set[Path] = set()
    starts = [Path.cwd(), Path(__file__).resolve().parent]
    try:
        import anki

        starts.append(Path(anki.__file__).resolve().parent)
    except Exception:
        pass
    for start in starts:
        try:
            resolved = start.resolve()
        except OSError:
            continue
        for directory in [resolved, *resolved.parents]:
            if directory not in seen:
                seen.add(directory)
                dirs.append(directory)
    return dirs


def read_dotenv_value(key: str) -> str | None:
    """Read `key` from the nearest `.env` file (walking up from cwd / the package).

    The `.env` file is gitignored, so this lets the desktop app pick up a key
    (e.g. OPENAI_API_KEY) without pasting it into settings. Never logs the value.
    """
    for directory in _env_search_dirs():
        env_path = directory / ".env"
        try:
            if not env_path.is_file():
                continue
            for raw in env_path.read_text(encoding="utf-8").splitlines():
                line = raw.strip()
                if not line or line.startswith("#"):
                    continue
                if line.startswith("export "):
                    line = line[len("export ") :].strip()
                name, sep, value = line.partition("=")
                if not sep or name.strip() != key:
                    continue
                value = value.strip().strip('"').strip("'")
                if value:
                    return value
        except OSError:
            continue
    return None


def resolve_openai_key(pm: Any) -> str | None:
    """Resolve the OpenAI key: profile setting → OPENAI_API_KEY env var → `.env`."""
    profile_key = pm.mcat_openai_key()
    if profile_key:
        return profile_key
    env_key = os.environ.get(ENV_KEY_NAME)
    if env_key and env_key.strip():
        return env_key.strip()
    return read_dotenv_value(ENV_KEY_NAME)


def resolve_openai_model(pm: Any) -> str:
    """Resolve the model: profile setting → OPENAI_MODEL env/`.env` → default."""
    profile = getattr(pm, "profile", None)
    profile_model = profile.get("mcatOpenaiModel") if profile else None
    if profile_model:
        return str(profile_model).strip()
    env_model = os.environ.get(ENV_MODEL_NAME) or read_dotenv_value(ENV_MODEL_NAME)
    return (env_model or "gpt-4o-mini").strip()


class OpenAIError(Exception):
    pass


def _extract_error(body: str) -> str | None:
    try:
        data = json.loads(body)
        return data.get("error", {}).get("message")
    except Exception:
        return None


def parse_tag_suggestion(raw: str, allowed_kcs: list[str]) -> dict[str, Any]:
    """Parse and validate the model's JSON tag suggestion.

    Only KC ids present in ``allowed_kcs`` survive (hallucinated ids are dropped);
    difficulty is clamped to 1-5; reasoning must be a known type; IRT values are
    clamped to the panel's ranges. Raises ``OpenAIError`` when no usable JSON or
    no valid KC is found. Pure/offline so it can be unit tested without a network.
    """
    text = raw.strip()
    if text.startswith("```"):
        text = re.sub(r"^```[a-zA-Z]*\s*", "", text)
        text = re.sub(r"\s*```$", "", text).strip()
    if not text.startswith("{"):
        match = re.search(r"\{.*\}", text, re.DOTALL)
        if match:
            text = match.group(0)
    try:
        data = json.loads(text)
    except Exception as exc:
        raise OpenAIError("Could not parse the AI tag suggestion.") from exc
    if not isinstance(data, dict):
        raise OpenAIError("Unexpected AI tag suggestion format.")

    allowed = set(allowed_kcs)
    raw_kcs = data.get("kcs", data.get("kc", []))
    if isinstance(raw_kcs, str):
        raw_kcs = [raw_kcs]
    kcs: list[str] = []
    for kc in raw_kcs or []:
        candidate = str(kc).strip()
        if candidate in allowed and candidate not in kcs:
            kcs.append(candidate)
    if not kcs:
        raise OpenAIError("The AI did not return a KC from the allowed list.")

    try:
        difficulty = int(round(float(data.get("difficulty", 3))))
    except (TypeError, ValueError):
        difficulty = 3
    difficulty = max(1, min(5, difficulty))

    reasoning = str(data.get("reasoning", "Conceptual")).strip()
    if reasoning not in REASONING_TYPES:
        reasoning = "Conceptual"

    def _clamp(value: Any, low: float, high: float, default: float) -> float:
        try:
            return max(low, min(high, float(value)))
        except (TypeError, ValueError):
            return default

    return {
        "kcs": kcs[:2],
        "difficulty": difficulty,
        "reasoning": reasoning,
        "discrimination": round(
            _clamp(data.get("discrimination", 1.0), 0.1, 3.0, 1.0), 1
        ),
        "guessing": round(_clamp(data.get("guessing", 0.25), 0.0, 0.95, 0.25), 2),
    }


class OpenAIClient:
    def __init__(
        self, key: str, model: str = "gpt-4o-mini", timeout: int = 30
    ) -> None:
        self.key = (key or "").strip()
        self.model = (model or "gpt-4o-mini").strip() or "gpt-4o-mini"
        self.timeout = timeout

    def _headers(self) -> dict[str, str]:
        return {
            "Authorization": f"Bearer {self.key}",
            "Content-Type": "application/json",
        }

    def test_connection(self) -> None:
        """Validate the key with a cheap GET /models. Raises OpenAIError on failure."""
        if not self.key:
            raise OpenAIError("No API key set.")
        req = urllib.request.Request(
            f"{OPENAI_BASE}/models", headers=self._headers(), method="GET"
        )
        try:
            with urllib.request.urlopen(req, timeout=self.timeout) as resp:
                if resp.status != 200:
                    raise OpenAIError(f"Unexpected status {resp.status}")
        except urllib.error.HTTPError as e:
            body = e.read().decode("utf-8", "replace")
            raise OpenAIError(_extract_error(body) or f"HTTP {e.code}") from e
        except urllib.error.URLError as e:
            raise OpenAIError(f"Network error: {e.reason}") from e

    def _chat(
        self,
        messages: list[dict[str, str]],
        *,
        max_tokens: int = 400,
        temperature: float = 0.4,
    ) -> str:
        if not self.key:
            raise OpenAIError("No API key set.")
        payload = json.dumps(
            {
                "model": self.model,
                "messages": messages,
                "temperature": temperature,
                "max_tokens": max_tokens,
            }
        ).encode("utf-8")
        req = urllib.request.Request(
            f"{OPENAI_BASE}/chat/completions",
            data=payload,
            headers=self._headers(),
            method="POST",
        )
        try:
            with urllib.request.urlopen(req, timeout=self.timeout) as resp:
                data: dict[str, Any] = json.loads(resp.read().decode("utf-8"))
        except urllib.error.HTTPError as e:
            body = e.read().decode("utf-8", "replace")
            raise OpenAIError(_extract_error(body) or f"HTTP {e.code}") from e
        except urllib.error.URLError as e:
            raise OpenAIError(f"Network error: {e.reason}") from e
        try:
            return data["choices"][0]["message"]["content"].strip()
        except (KeyError, IndexError) as e:
            raise OpenAIError("Malformed response from OpenAI.") from e

    def reword_question(self, question: str) -> str:
        return self._chat(
            [
                {"role": "system", "content": REWORD_SYSTEM},
                {"role": "user", "content": question},
            ],
            max_tokens=400,
            temperature=0.7,
        )

    def questions_equivalent(self, original: str, reworded: str) -> bool:
        """Second call: confirm the reworded stem means the same as the original."""
        answer = self._chat(
            [
                {"role": "system", "content": EQUIV_SYSTEM},
                {
                    "role": "user",
                    "content": f"ORIGINAL:\n{original}\n\nREWORDED:\n{reworded}",
                },
            ],
            max_tokens=3,
            temperature=0.0,
        )
        return answer.strip().lower().startswith("yes")

    def chat(
        self,
        messages: list[dict[str, str]],
        *,
        max_tokens: int = 600,
        temperature: float = 0.3,
    ) -> str:
        """Multi-turn chat completion for the in-reviewer 'Ask AI' tutor. Pass the
        full message list (system prompt + prior turns)."""
        return self._chat(messages, max_tokens=max_tokens, temperature=temperature)

    def suggest_card_tags(
        self, card_text: str, allowed_kcs: list[str]
    ) -> dict[str, Any]:
        """Suggest Concept Scheduler tags for a card, constrained to `allowed_kcs`.

        Returns a validated dict: ``kcs`` (subset of allowed), ``difficulty`` (1-5),
        ``reasoning``, ``discrimination``, ``guessing``.
        """
        allowed_block = "\n".join(allowed_kcs)
        user = (
            "ALLOWED KC IDS (choose only from these, copy exactly):\n"
            f"{allowed_block}\n\nCARD TEXT:\n{card_text}"
        )
        raw = self._chat(
            [
                {"role": "system", "content": SUGGEST_TAGS_SYSTEM},
                {"role": "user", "content": user},
            ],
            max_tokens=200,
            temperature=0.0,
        )
        return parse_tag_suggestion(raw, allowed_kcs)


class McatAiDialog(QDialog):
    def __init__(self, mw: Any) -> None:
        super().__init__(mw)
        self.mw = mw
        self.setWindowTitle("MCAT AI (OpenAI)")
        self.setMinimumWidth(460)
        self._build()
        self._load()

    def _build(self) -> None:
        layout = QVBoxLayout(self)
        info = QLabel(
            "Paste your own OpenAI API key to enable AI question rewording while "
            "you study. The key is stored only on this device (this profile) and "
            "is never synced, shared, or committed. Everything works offline "
            "without a key."
        )
        info.setWordWrap(True)
        layout.addWidget(info)

        form = QFormLayout()
        self.key_edit = QLineEdit()
        self.key_edit.setEchoMode(QLineEdit.EchoMode.Password)
        self.key_edit.setPlaceholderText("sk-...")
        show = QCheckBox("Show")
        qconnect(
            show.toggled,
            lambda on: self.key_edit.setEchoMode(
                QLineEdit.EchoMode.Normal if on else QLineEdit.EchoMode.Password
            ),
        )
        key_row = QHBoxLayout()
        key_row.addWidget(self.key_edit)
        key_row.addWidget(show)
        key_container = QWidget()
        key_container.setLayout(key_row)
        form.addRow("API key:", key_container)

        self.model_edit = QLineEdit()
        self.model_edit.setPlaceholderText("gpt-4o-mini")
        form.addRow("Model:", self.model_edit)
        layout.addLayout(form)

        self.reword_cb = QCheckBox("Reword MCAT questions with AI as I study")
        self.shuffle_cb = QCheckBox("Shuffle answer choices")
        layout.addWidget(self.reword_cb)
        layout.addWidget(self.shuffle_cb)

        self.source_hint = QLabel("")
        self.source_hint.setWordWrap(True)
        self.source_hint.setStyleSheet("color: palette(mid);")
        layout.addWidget(self.source_hint)

        actions = QHBoxLayout()
        self.test_btn = QPushButton("Test connection")
        qconnect(self.test_btn.clicked, self._on_test)
        clear_btn = QPushButton("Clear key")
        qconnect(clear_btn.clicked, lambda: self.key_edit.setText(""))
        actions.addWidget(self.test_btn)
        actions.addWidget(clear_btn)
        actions.addStretch()
        layout.addLayout(actions)

        buttons = QDialogButtonBox(
            QDialogButtonBox.StandardButton.Save
            | QDialogButtonBox.StandardButton.Close
        )
        qconnect(buttons.accepted, self._on_save)
        qconnect(buttons.rejected, self.reject)
        layout.addWidget(buttons)

    def _load(self) -> None:
        pm = self.mw.pm
        self.key_edit.setText(pm.mcat_openai_key() or "")
        self.model_edit.setText(pm.mcat_openai_model())
        self.reword_cb.setChecked(pm.mcat_reword_enabled())
        self.shuffle_cb.setChecked(pm.mcat_shuffle_choices_enabled())
        # If no key is set in this profile but one is available from the
        # environment / a .env file, tell the user it will be used automatically.
        if not pm.mcat_openai_key() and resolve_openai_key(pm):
            self.source_hint.setText(
                "A key from OPENAI_API_KEY (environment / .env) is detected and will "
                "be used automatically. Leave the field blank to keep using it, or "
                "paste a key here to override it for this profile."
            )
        else:
            self.source_hint.setText("")

    def _client(self) -> OpenAIClient:
        # Test the typed key if present, else whatever is resolved (env / .env).
        key = self.key_edit.text().strip() or resolve_openai_key(self.mw.pm) or ""
        model = self.model_edit.text().strip() or resolve_openai_model(self.mw.pm)
        return OpenAIClient(key, model, self.mw.pm.network_timeout())

    def _on_test(self) -> None:
        client = self._client()
        if not client.key:
            showWarning("Enter an API key first.", parent=self)
            return
        self.test_btn.setEnabled(False)

        def task() -> bool:
            client.test_connection()
            return True

        def on_done(fut: Any) -> None:
            self.test_btn.setEnabled(True)
            try:
                fut.result()
            except Exception as exc:
                showWarning(f"Connection failed: {exc}", parent=self)
                return
            tooltip("OpenAI connection OK.", parent=self)

        self.mw.taskman.run_in_background(task, on_done)

    def _on_save(self) -> None:
        pm = self.mw.pm
        pm.set_mcat_openai_key(self.key_edit.text())
        pm.set_mcat_openai_model(self.model_edit.text())
        pm.set_mcat_reword_enabled(self.reword_cb.isChecked())
        pm.set_mcat_shuffle_choices_enabled(self.shuffle_cb.isChecked())
        tooltip("Saved MCAT AI settings.", parent=self.mw)
        self.accept()


def show_mcat_ai_dialog(mw: Any) -> None:
    McatAiDialog(mw).exec()
