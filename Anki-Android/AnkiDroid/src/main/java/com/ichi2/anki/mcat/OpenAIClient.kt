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
package com.ichi2.anki.mcat

import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import okhttp3.MediaType.Companion.toMediaType
import okhttp3.OkHttpClient
import okhttp3.Request
import okhttp3.RequestBody.Companion.toRequestBody
import org.json.JSONArray
import org.json.JSONObject
import java.io.IOException
import java.util.concurrent.TimeUnit

/**
 * Bring-your-own OpenAI key for MCAT AI features. Port of the desktop `aqt/mcat_ai.py`.
 *
 * The user pastes their own OpenAI API key; it is stored per install only (never synced, logged, or
 * committed). The reviewer uses this to reword MCAT question stems on the fly, with a second
 * verification call to confirm the reworded stem means the same thing as the original.
 *
 * All network calls are blocking OkHttp calls wrapped in [Dispatchers.IO]; callers must invoke them
 * from a coroutine and never block the main thread.
 */
class OpenAIClient(
    key: String?,
    model: String?,
    private val timeoutSecs: Int = 30,
) {
    val key: String = (key ?: "").trim()
    val model: String = (model ?: "").trim().ifEmpty { DEFAULT_MODEL }

    private fun client(): OkHttpClient =
        OkHttpClient
            .Builder()
            .connectTimeout(timeoutSecs.toLong(), TimeUnit.SECONDS)
            .readTimeout(timeoutSecs.toLong(), TimeUnit.SECONDS)
            .writeTimeout(timeoutSecs.toLong(), TimeUnit.SECONDS)
            .build()

    private fun authHeader(): String = "Bearer $key"

    /** Validate the key with a cheap `GET /models`. Throws [OpenAIException] on failure. */
    suspend fun testConnection() {
        if (key.isEmpty()) throw OpenAIException("No API key set.")
        withContext(Dispatchers.IO) {
            val request =
                Request
                    .Builder()
                    .url("$OPENAI_BASE/models")
                    .header("Authorization", authHeader())
                    .get()
                    .build()
            try {
                client().newCall(request).execute().use { response ->
                    if (!response.isSuccessful) {
                        val body = response.body.string()
                        throw OpenAIException(extractError(body) ?: "HTTP ${response.code}")
                    }
                }
            } catch (e: IOException) {
                throw OpenAIException("Network error: ${e.message}", e)
            }
        }
    }

    /** Reword a question stem. Returns the reworded stem (may be empty on a malformed response). */
    suspend fun rewordQuestion(question: String): String =
        chatCompletion(
            messages =
                listOf(
                    systemMessage(REWORD_SYSTEM),
                    userMessage(question),
                ),
            maxTokens = 400,
            temperature = 0.7,
        )

    /** Second call: confirm the reworded stem means the same as the original. */
    suspend fun questionsEquivalent(
        original: String,
        reworded: String,
    ): Boolean {
        val answer =
            chatCompletion(
                messages =
                    listOf(
                        systemMessage(EQUIV_SYSTEM),
                        userMessage("ORIGINAL:\n$original\n\nREWORDED:\n$reworded"),
                    ),
                maxTokens = 3,
                temperature = 0.0,
            )
        return answer.trim().lowercase().startsWith("yes")
    }

    /**
     * Multi-turn chat completion for the in-reviewer "Ask AI" tutor. Pass the full message list
     * (system prompt + prior turns). Mirrors the desktop `OpenAIClient.chat`.
     */
    suspend fun chat(
        messages: List<ChatMessage>,
        maxTokens: Int = 600,
        temperature: Double = 0.3,
    ): String =
        chatCompletion(
            messages = messages.map { message(it.role, it.content) },
            maxTokens = maxTokens,
            temperature = temperature,
        )

    private suspend fun chatCompletion(
        messages: List<JSONObject>,
        maxTokens: Int,
        temperature: Double,
    ): String {
        if (key.isEmpty()) throw OpenAIException("No API key set.")
        val payload =
            JSONObject()
                .put("model", model)
                .put("messages", JSONArray(messages))
                .put("temperature", temperature)
                .put("max_tokens", maxTokens)
                .toString()
        return withContext(Dispatchers.IO) {
            val request =
                Request
                    .Builder()
                    .url("$OPENAI_BASE/chat/completions")
                    .header("Authorization", authHeader())
                    .post(payload.toRequestBody(JSON_MEDIA_TYPE))
                    .build()
            val bodyText =
                try {
                    client().newCall(request).execute().use { response ->
                        val body = response.body.string()
                        if (!response.isSuccessful) {
                            throw OpenAIException(extractError(body) ?: "HTTP ${response.code}")
                        }
                        body
                    }
                } catch (e: IOException) {
                    throw OpenAIException("Network error: ${e.message}", e)
                }
            parseChatContent(bodyText)
        }
    }

    private fun parseChatContent(body: String): String =
        try {
            JSONObject(body)
                .getJSONArray("choices")
                .getJSONObject(0)
                .getJSONObject("message")
                .getString("content")
                .trim()
        } catch (e: Exception) {
            throw OpenAIException("Malformed response from OpenAI.", e)
        }

    companion object {
        const val OPENAI_BASE = "https://api.openai.com/v1"
        const val DEFAULT_MODEL = "gpt-4o-mini"

        private val JSON_MEDIA_TYPE = "application/json; charset=utf-8".toMediaType()

        val REWORD_SYSTEM =
            "You are helping an MCAT student by rephrasing a multiple-choice question " +
                "STEM so it tests the SAME underlying concept with fresh wording, forcing " +
                "the student to re-derive the answer instead of recognizing memorized " +
                "phrasing. Rewrite the stem so it asks exactly the same thing and keeps the " +
                "same correct answer, changing only the sentence structure and scenario " +
                "framing. Keep the SAME difficulty: do NOT make it harder or easier, and do " +
                "NOT add extra reading, distracting detail, or extra reasoning steps, which " +
                "would quietly change which concept is being tested. Preserve every fact, " +
                "number, and given the student needs to answer (reword it, never drop it); " +
                "the only thing you remove is the original's recognizable wording. Do NOT " +
                "change any facts, numbers, or what is being asked. Do NOT include or " +
                "reference the answer choices (no 'A', 'B', 'C', 'D'). Do NOT add hints, " +
                "definitions, or new information. Return ONLY the reworded question stem, " +
                "with no preamble or quotation marks."

        val EQUIV_SYSTEM =
            "You verify whether two versions of an MCAT question stem are semantically " +
                "equivalent: given identical answer choices, they must ask the same thing " +
                "and have the same correct answer. Reply with exactly 'yes' if they are " +
                "equivalent, or 'no' otherwise. Reply with only 'yes' or 'no'."

        private fun systemMessage(content: String): JSONObject = message("system", content)

        private fun userMessage(content: String): JSONObject = message("user", content)

        private fun message(
            role: String,
            content: String,
        ): JSONObject = JSONObject().put("role", role).put("content", content)

        /** Best-effort extraction of the `error.message` field from an OpenAI error body. */
        private fun extractError(body: String): String? =
            try {
                JSONObject(body).optJSONObject("error")?.optString("message")?.ifEmpty { null }
            } catch (e: Exception) {
                null
            }
    }
}

/** Raised for any failure talking to OpenAI (no key, network error, HTTP error, bad response). */
class OpenAIException(
    message: String,
    cause: Throwable? = null,
) : Exception(message, cause)

/**
 * A single chat message for the "Ask AI" tutor conversation.
 *
 * @param role one of `system`, `user`, or `assistant`
 * @param content the message text
 */
data class ChatMessage(
    val role: String,
    val content: String,
)
