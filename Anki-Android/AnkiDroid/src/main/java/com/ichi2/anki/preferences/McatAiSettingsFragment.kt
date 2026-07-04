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
package com.ichi2.anki.preferences

import android.text.InputType
import androidx.lifecycle.lifecycleScope
import androidx.preference.EditTextPreference
import androidx.preference.Preference
import com.ichi2.anki.R
import com.ichi2.anki.mcat.OpenAIClient
import com.ichi2.anki.mcat.OpenAIException
import com.ichi2.anki.settings.Prefs
import com.ichi2.anki.snackbar.showSnackbar
import kotlinx.coroutines.launch
import timber.log.Timber

/**
 * Per-install "bring your own OpenAI key" settings for the MCAT AI reviewer features (Feature B).
 * Port of the desktop `McatAiDialog` (`aqt/mcat_ai.py`).
 *
 * The API key is stored only in the app's local SharedPreferences; it is never synced, logged, or
 * committed. All features degrade gracefully without a key.
 */
class McatAiSettingsFragment : SettingsFragment() {
    override val preferenceResource = R.xml.preferences_mcat_ai
    override val analyticsScreenNameConstant = "prefs.mcat_ai"

    override fun initSubscreen() {
        // The API key: keep it out of predictive keyboards, and never show the raw value in the
        // settings list - only whether it is set.
        requirePreference<EditTextPreference>(R.string.mcat_openai_key_key).apply {
            setOnBindEditTextListener { editText ->
                editText.inputType = InputType.TYPE_CLASS_TEXT or InputType.TYPE_TEXT_FLAG_NO_SUGGESTIONS
                editText.setSelection(editText.text.length)
            }
            summaryProvider =
                Preference.SummaryProvider<EditTextPreference> { pref ->
                    if (pref.text.isNullOrBlank()) {
                        getString(R.string.mcat_openai_key_summary_none)
                    } else {
                        getString(R.string.mcat_openai_key_summary_set)
                    }
                }
        }

        requirePreference<Preference>(R.string.mcat_ai_test_connection_key).setOnPreferenceClickListener {
            testConnection()
            true
        }

        requirePreference<Preference>(R.string.mcat_ai_clear_key_key).setOnPreferenceClickListener {
            requirePreference<EditTextPreference>(R.string.mcat_openai_key_key).text = ""
            showSnackbar(R.string.mcat_ai_key_cleared)
            true
        }
    }

    private fun testConnection() {
        val key = Prefs.mcatOpenAiKey
        if (key.isNullOrBlank()) {
            showSnackbar(R.string.mcat_ai_test_no_key)
            return
        }
        val client = OpenAIClient(key, Prefs.mcatOpenAiModel, Prefs.networkTimeoutSecs)
        showSnackbar(R.string.mcat_ai_test_in_progress)
        lifecycleScope.launch {
            try {
                client.testConnection()
                showSnackbar(R.string.mcat_ai_test_success)
            } catch (e: OpenAIException) {
                Timber.d(e, "MCAT AI test connection failed")
                showSnackbar(getString(R.string.mcat_ai_test_failed, e.message ?: ""))
            }
        }
    }
}
