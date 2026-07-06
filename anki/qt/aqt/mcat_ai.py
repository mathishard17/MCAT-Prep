# Copyright: Ankitects Pty Ltd and contributors
# License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html

"""Bring-your-own OpenAI key for MCAT AI features (Track H).

The user pastes their own OpenAI API key; it is stored per profile only (never
synced, logged, or committed). The reviewer uses this to reword MCAT question
stems on the fly, with a second verification call to confirm the reworded stem
means the same thing as the original.

The network client, key/model resolution, `.env` reading, and prompt strings
live in the stdlib-only :mod:`aqt.mcat_ai_core` module (no Qt imports) so they
can be imported and unit tested offline. They are re-exported here so existing
imports such as ``from aqt.mcat_ai import OpenAIClient`` keep working unchanged;
this module adds the Qt settings dialog on top.
"""

from __future__ import annotations

from typing import Any

from aqt.mcat_ai_core import (
    ENV_KEY_NAME,
    ENV_MODEL_NAME,
    EQUIV_SYSTEM,
    OPENAI_BASE,
    REASONING_TYPES,
    REWORD_SYSTEM,
    SUGGEST_TAGS_SYSTEM,
    OpenAIClient,
    OpenAIError,
    parse_tag_suggestion,
    read_dotenv_value,
    resolve_openai_key,
    resolve_openai_model,
)
from aqt.qt import *
from aqt.utils import showWarning, tooltip

# Re-exported from the stdlib-only core so callers can keep importing these from
# ``aqt.mcat_ai``. Listing them here also marks them as intentional re-exports.
__all__ = [
    "ENV_KEY_NAME",
    "ENV_MODEL_NAME",
    "EQUIV_SYSTEM",
    "OPENAI_BASE",
    "REASONING_TYPES",
    "REWORD_SYSTEM",
    "SUGGEST_TAGS_SYSTEM",
    "OpenAIClient",
    "OpenAIError",
    "parse_tag_suggestion",
    "read_dotenv_value",
    "resolve_openai_key",
    "resolve_openai_model",
    "McatAiDialog",
    "show_mcat_ai_dialog",
]


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

        eval_note = QLabel(
            "AI features are validated on a held-out eval with pre-committed "
            "cutoffs (see evals/RESULTS.md): KC-tagging 85% (cutoff 80%), "
            "rewording 98%, prompt-injection resistance 100%."
        )
        eval_note.setWordWrap(True)
        eval_note.setStyleSheet("color: palette(mid);")
        layout.addWidget(eval_note)

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
