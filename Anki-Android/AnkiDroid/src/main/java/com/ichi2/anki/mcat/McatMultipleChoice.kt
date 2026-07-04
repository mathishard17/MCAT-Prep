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

import com.ichi2.anki.conceptscheduler.normalizeConceptTag
import org.json.JSONArray
import org.json.JSONObject
import kotlin.random.Random

/**
 * Structured multiple-choice data parsed from a KC-tagged "Basic" MCAT note. Mirrors the payload built
 * by the desktop `Reviewer._mcat_mc_payload`.
 *
 * @param kc human-readable KC label, e.g. `Bio · DNA` (empty if none)
 * @param kcId raw KC id, e.g. `Bio::DNA`
 * @param question the question stem (HTML)
 * @param choices the answer choices (HTML), in display order
 * @param correctIndex index into [choices] of the correct answer
 * @param explanation the explanation shown after answering (HTML, may be empty)
 * @param reworded whether [question] has been AI-reworded
 */
data class McatMcPayload(
    val kc: String,
    val kcId: String,
    val question: String,
    val choices: List<String>,
    val correctIndex: Int,
    val explanation: String,
    val reworded: Boolean = false,
) {
    fun toJson(): JSONObject =
        JSONObject()
            .put("kc", kc)
            .put("kcId", kcId)
            .put("question", question)
            .put("choices", JSONArray(choices))
            .put("correctIndex", correctIndex)
            .put("explanation", explanation)
            .put("reworded", reworded)
}

/**
 * Parses KC-tagged "Basic" notes into clickable multiple-choice quizzes and builds the JavaScript that
 * renders them (and the "Ask AI" tutor chat) inside the card WebView. Port of the desktop
 * `Reviewer._mcat_mc_payload` / `_mcat_enhance`, the injected `_renderMcatQuiz` / `_mcatChoose` /
 * `_mcatFinishReword` scripts, the `_mcat_chat_open` scoped system prompt, and the chat UI JS.
 *
 * Kept free of Android dependencies so the parsing/shuffle/prompt logic is unit-testable, exactly like
 * [com.ichi2.anki.conceptscheduler.McatTopics].
 */
object McatMultipleChoice {
    // Field 0 (front): `QUESTION<br><br>A. choiceA<br>B. choiceB<br>...`
    // Field 1 (back):  `<b>Correct:</b> X<br><br>EXPLANATION`
    private val BREAK_RE = Regex("""<br\s*/?>\s*<br\s*/?>""", RegexOption.IGNORE_CASE)
    private val LINE_RE = Regex("""<br\s*/?>""", RegexOption.IGNORE_CASE)
    private val CHOICE_RE = Regex("""\s*([A-Z])[.)]\s*(.*\S)\s*""", RegexOption.DOT_MATCHES_ALL)
    private val CORRECT_RE =
        Regex("""correct:?\s*(?:</b>|</strong>)?\s*([A-Z])\b""", RegexOption.IGNORE_CASE)

    /** Intro line shown when the "Ask AI" chat opens. Mirrors the desktop `intro` text verbatim. */
    const val CHAT_INTRO =
        "Ask me anything about this question or the concept behind it \u2014 I'll walk through examples too."

    /**
     * Parse a concept (KC-tagged) card into structured multiple-choice data. Returns null for anything
     * that is not a recognisable MC card, so normal AnkiDroid study is untouched.
     *
     * @param tags the note's tags
     * @param fields the note's fields (field 0 = front, field 1 = back)
     */
    fun parse(
        tags: List<String>,
        fields: List<String>,
    ): McatMcPayload? {
        val normalizedTags = tags.map { normalizeConceptTag(it) }
        if (normalizedTags.none { it.startsWith("KC::") }) return null
        if (fields.size < 2) return null

        val front = fields[0]
        val back = fields[1]

        val frontParts = BREAK_RE.split(front, limit = 2)
        if (frontParts.size != 2) return null
        val question = frontParts[0].trim()

        val choices = mutableListOf<String>()
        val letters = mutableListOf<String>()
        for (raw in LINE_RE.split(frontParts[1])) {
            val text = raw.trim()
            if (text.isEmpty()) continue
            val match = CHOICE_RE.matchEntire(text) ?: continue
            letters.add(match.groupValues[1].uppercase())
            choices.add(match.groupValues[2].trim())
        }
        if (choices.size < 2 || question.isEmpty()) return null

        val correctMatch = CORRECT_RE.find(back) ?: return null
        val correctLetter = correctMatch.groupValues[1].uppercase()
        val correctIndex = letters.indexOf(correctLetter)
        if (correctIndex < 0) return null

        val backParts = BREAK_RE.split(back, limit = 2)
        val explanation = if (backParts.size == 2) backParts[1].trim() else ""

        val kcId =
            normalizedTags
                .firstOrNull { it.startsWith("KC::") }
                ?.removePrefix("KC::")
                .orEmpty()
        val kcLabel = kcId.replace("::", " · ")

        return McatMcPayload(
            kc = kcLabel,
            kcId = kcId,
            question = question,
            choices = choices,
            correctIndex = correctIndex,
            explanation = explanation,
        )
    }

    /**
     * Shuffle the answer choices, tracking the new correct index. Mirrors the desktop `_mcat_enhance`
     * shuffle. Rewording is handled separately (asynchronously) so a slow network call never blocks
     * showing the card.
     */
    fun shuffle(
        payload: McatMcPayload,
        random: Random = Random.Default,
    ): McatMcPayload {
        val choices = payload.choices
        if (choices.size <= 1) return payload
        val order = choices.indices.toMutableList().apply { shuffle(random) }
        return payload.copy(
            choices = order.map { choices[it] },
            correctIndex = order.indexOf(payload.correctIndex),
        )
    }

    /**
     * Build the scoped "Ask AI" system prompt for [payload], mirroring the desktop `_mcat_chat_open`
     * prompt VERBATIM. The tutor must (a) always include a concrete example/analogy and (b) decline
     * off-topic requests.
     *
     * @param selectedIndex the choice the student picked, or null if not recorded
     */
    fun buildChatSystemPrompt(
        payload: McatMcPayload,
        selectedIndex: Int?,
    ): String {
        val choices = payload.choices
        val letters = choices.indices.map { ('A' + it).toString() }
        val choiceLines = choices.indices.joinToString("\n") { "${letters[it]}. ${choices[it]}" }
        val correctIndex = payload.correctIndex
        val correctLine =
            if (correctIndex in choices.indices) {
                "${letters[correctIndex]}. ${choices[correctIndex]}"
            } else {
                "unknown"
            }
        val selectedDesc =
            if (selectedIndex != null && selectedIndex in choices.indices) {
                val verdict = if (selectedIndex == correctIndex) "correct" else "incorrect"
                "${letters[selectedIndex]}. ${choices[selectedIndex]} ($verdict)"
            } else {
                "not recorded"
            }
        return "You are an MCAT tutor helping a student with ONE specific practice " +
            "question. Be concise, accurate, and encouraging. ALWAYS illustrate " +
            "your explanations with at least one concrete example or analogy \u2014 a " +
            "short worked example, a realistic MCAT-style scenario, or a memorable " +
            "analogy \u2014 so the concept sticks. When helpful, give a couple of quick " +
            "contrasting examples (e.g. what would change the answer).\n\n" +
            "Topic (knowledge component): ${payload.kc}\n" +
            "Question: ${payload.question}\n" +
            "Choices:\n$choiceLines\n" +
            "Correct answer: $correctLine\n" +
            "Student selected: $selectedDesc\n" +
            "Explanation: ${payload.explanation}\n\n" +
            "Only discuss THIS question and the directly related MCAT concept and " +
            "science. If the student asks anything off-topic (unrelated subjects, " +
            "chit-chat, personal tasks, coding, current events, etc.), politely " +
            "decline in one sentence and steer them back to this question. Do not " +
            "answer off-topic requests."
    }

    /**
     * Build the JavaScript that renders [payload] as a clickable quiz inside the card WebView. When
     * [loading] is true a spinner ("Rewording this question…") is shown instead of the question and
     * choices, until [finishRewordScript] resolves it. Grading is sent via the `mcatGrade:` URL
     * scheme; the "Ask AI about this" button sends `mcatChatOpen` (see `AbstractFlashcardViewer`).
     */
    fun renderQuizScript(
        payload: McatMcPayload,
        loading: Boolean = false,
        aiEnabled: Boolean = false,
    ): String =
        QUIZ_JS_TEMPLATE
            .replace("__CSS__", JSONObject.quote(QUIZ_CSS))
            .replace(
                "__PAYLOAD__",
                payload.toJson().put("loading", loading).put("aiEnabled", aiEnabled).toString(),
            )

    /**
     * Build the JavaScript that resolves the reword loading state: swaps in the reworded (or original)
     * stem and reveals the choices. Mirrors the desktop `_mcatFinishReword` call. Always call this once
     * a reword request settles (success, non-equivalent, error, or timeout) so the spinner never hangs.
     */
    fun finishRewordScript(
        question: String,
        reworded: Boolean,
    ): String =
        "if (window._mcatFinishReword) { window._mcatFinishReword(${JSONObject.quote(question)}, $reworded); }"

    /** Build the JavaScript that opens the "Ask AI" chat panel with the given [intro] message. */
    fun renderChatScript(intro: String): String =
        "if (window._renderMcatChat) { window._renderMcatChat(${JSONObject.quote(intro)}); }"

    /** Build the JavaScript that appends a chat message from [role] ("ai" or "user"). */
    fun chatAddMessageScript(
        role: String,
        text: String,
    ): String =
        "if (window._mcatChatAddMessage) { window._mcatChatAddMessage(${JSONObject.quote(role)}, ${JSONObject.quote(text)}); }"

    /** Build the JavaScript that shows/hides the chat "thinking" indicator and disables the input. */
    fun chatSetPendingScript(pending: Boolean): String =
        "if (window._mcatChatSetPending) { window._mcatChatSetPending($pending); }"

    // Self-contained CSS, robust for both light and night mode (uses neutral rgba + inherited color
    // rather than the desktop's `--canvas`/`color-mix`, which aren't defined in the AnkiDroid WebView).
    @Suppress("ktlint:standard:max-line-length")
    private val QUIZ_CSS =
        """
        body.mcat-quiz-active #qa { display: none; }
        #_mcat_quiz { padding: 0 0.5rem; }
        .mcat-quiz-card { width: min(92vw, 52rem); margin: 0.5rem auto 2rem; text-align: start; font-size: 1.05rem; line-height: 1.5; }
        .mcat-quiz-kc { display: inline-block; font-size: 0.72rem; letter-spacing: 0.03em; text-transform: uppercase; opacity: 0.75; border: 1px solid rgba(128,128,128,0.35); padding: 0.15rem 0.5rem; border-radius: 0.5rem; margin-bottom: 0.6rem; }
        .mcat-quiz-question { margin-bottom: 1rem; font-weight: 500; }
        .mcat-quiz-reworded { display: inline-block; font-size: 0.68rem; opacity: 0.7; margin: -0.6rem 0 0.8rem; }
        .mcat-quiz-loading { display: flex; flex-direction: column; align-items: center; justify-content: center; gap: 0.9rem; min-height: 12rem; opacity: 0.85; }
        .mcat-quiz-spinner { width: 2.4rem; height: 2.4rem; border-radius: 50%; border: 3px solid rgba(128,128,128,0.35); border-top-color: #4f74d6; animation: mcat-spin 0.8s linear infinite; }
        .mcat-quiz-loading-text { font-size: 0.95rem; }
        @keyframes mcat-spin { to { transform: rotate(360deg); } }
        .mcat-quiz-choices { display: flex; flex-direction: column; gap: 0.5rem; }
        .mcat-quiz-choice { display: flex; align-items: flex-start; gap: 0.6rem; width: 100%; text-align: start; padding: 0.7rem 0.9rem; border: 1.5px solid rgba(128,128,128,0.4); border-radius: 0.7rem; background: transparent; color: inherit; font: inherit; cursor: pointer; -webkit-tap-highlight-color: transparent; }
        .mcat-quiz-choice:active:not(:disabled) { transform: translateY(1px); }
        .mcat-quiz-choice:disabled { cursor: default; opacity: 0.95; }
        .mcat-quiz-letter { flex: 0 0 auto; width: 1.6rem; height: 1.6rem; display: inline-flex; align-items: center; justify-content: center; border-radius: 50%; font-size: 0.8rem; font-weight: 600; border: 1.5px solid rgba(128,128,128,0.4); }
        .mcat-quiz-choice-text { flex: 1 1 auto; }
        .mcat-quiz-choice.correct { border-color: #3fae6e; background: rgba(63,174,110,0.18); }
        .mcat-quiz-choice.correct .mcat-quiz-letter { background: #3fae6e; color: #fff; border-color: #3fae6e; }
        .mcat-quiz-choice.wrong { border-color: #dd6666; background: rgba(221,102,102,0.18); }
        .mcat-quiz-choice.wrong .mcat-quiz-letter { background: #dd6666; color: #fff; border-color: #dd6666; }
        .mcat-quiz-feedback { margin-top: 1.1rem; }
        .mcat-quiz-verdict { font-weight: 700; margin-bottom: 0.4rem; }
        .mcat-quiz-verdict.correct { color: #37a862; }
        .mcat-quiz-verdict.wrong { color: #d1524f; }
        .mcat-quiz-explanation { border: 1px solid rgba(128,128,128,0.3); border-radius: 0.6rem; padding: 0.7rem 0.9rem; margin-bottom: 0.8rem; font-size: 0.98rem; }
        .mcat-quiz-rate-prompt { font-size: 0.85rem; opacity: 0.8; margin-bottom: 0.4rem; }
        .mcat-quiz-actions { display: flex; flex-wrap: wrap; gap: 0.5rem; }
        .mcat-quiz-rate { flex: 1 1 auto; min-width: 4.5rem; padding: 0.6rem 0.8rem; border: 1.5px solid rgba(128,128,128,0.4); border-radius: 0.6rem; background: transparent; color: inherit; font: inherit; font-weight: 600; cursor: pointer; }
        .mcat-quiz-rate:disabled { opacity: 0.6; cursor: default; }
        .mcat-quiz-rate.ease1 { border-color: rgba(221,102,102,0.6); }
        .mcat-quiz-rate.ease4 { border-color: rgba(63,174,110,0.6); }
        .mcat-quiz-reword-row { margin: -0.4rem 0 0.8rem; }
        .mcat-quiz-reword { font: inherit; font-size: 0.85rem; padding: 0.35rem 0.8rem; border-radius: 999px; border: 1px solid rgba(128,128,128,0.45); background: transparent; color: inherit; cursor: pointer; }
        .mcat-quiz-reword:disabled { opacity: 0.6; cursor: default; }
        .mcat-quiz-ask-row { margin-top: 0.8rem; display: flex; flex-wrap: wrap; gap: 0.5rem; }
        .mcat-quiz-ask, .mcat-quiz-lesson { padding: 0.5rem 0.9rem; border: 1px solid rgba(79,116,214,0.5); border-radius: 0.6rem; background: rgba(79,116,214,0.12); color: inherit; font: inherit; font-weight: 600; cursor: pointer; }
        .mcat-quiz-lesson.glow { border-color: #d1524f; background: rgba(209,82,79,0.14); }
        #_mcat_chat[hidden] { display: none; }
        #_mcat_chat { position: fixed; left: 0.75rem; right: 0.75rem; bottom: 0.75rem; max-width: 32rem; margin: 0 auto; max-height: 60vh; display: flex; flex-direction: column; background: #ffffff; color: #1a1a1a; border: 1px solid rgba(128,128,128,0.45); border-radius: 0.85rem; box-shadow: 0 8px 30px rgba(0,0,0,0.35); z-index: 200; overflow: hidden; }
        .nightMode #_mcat_chat { background: #2b2b2d; color: #ececec; }
        .mcat-chat-head { display: flex; align-items: center; justify-content: space-between; padding: 0.6rem 0.8rem; border-bottom: 1px solid rgba(128,128,128,0.3); font-size: 0.95rem; }
        .mcat-chat-close { border: none; background: transparent; color: inherit; cursor: pointer; font-size: 1.1rem; }
        .mcat-chat-messages { flex: 1 1 auto; overflow-y: auto; padding: 0.7rem; display: flex; flex-direction: column; gap: 0.5rem; min-height: 6rem; }
        .mcat-chat-msg { max-width: 85%; padding: 0.5rem 0.7rem; border-radius: 0.7rem; font-size: 0.92rem; line-height: 1.4; white-space: pre-wrap; overflow-wrap: anywhere; }
        .mcat-chat-msg.ai { align-self: flex-start; background: rgba(128,128,128,0.2); }
        .mcat-chat-msg.user { align-self: flex-end; background: rgba(79,116,214,0.28); }
        .mcat-chat-pending { opacity: 0.6; font-weight: 700; letter-spacing: 0.15em; }
        .mcat-chat-inputrow { display: flex; gap: 0.4rem; padding: 0.6rem; border-top: 1px solid rgba(128,128,128,0.3); }
        .mcat-chat-input { flex: 1 1 auto; resize: none; padding: 0.45rem 0.6rem; border: 1px solid rgba(128,128,128,0.45); border-radius: 0.5rem; background: transparent; color: inherit; font: inherit; }
        .mcat-chat-send { padding: 0 0.9rem; border: none; border-radius: 0.5rem; background: #3fae6e; color: #fff; font: inherit; font-weight: 600; cursor: pointer; }
        .mcat-chat-send:disabled { opacity: 0.5; cursor: default; }
        """.trimIndent()

    // JS injected after the card page finishes loading. No `$` may appear in this template (Kotlin raw
    // string interpolation). JS -> Kotlin uses the `mcatChoice:` / `mcatGrade:` / `mcatChatOpen` /
    // `mcatChatSend:` URL schemes; Kotlin -> JS uses the window.* functions defined below.
    @Suppress("ktlint:standard:max-line-length")
    private val QUIZ_JS_TEMPLATE =
        """
        (function () {
            try {
                var payload = __PAYLOAD__;
                if (!payload) { return; }
                if (!document.getElementById("_mcat_style")) {
                    var st = document.createElement("style");
                    st.id = "_mcat_style";
                    st.textContent = __CSS__;
                    (document.head || document.documentElement).appendChild(st);
                }
                var state = {
                    answered: false,
                    graded: false,
                    correctIndex: payload.correctIndex,
                    explanation: payload.explanation || "",
                    choices: payload.choices || [],
                };
                window._mcatState = state;

                function typeset(el) {
                    if (window.MathJax && window.MathJax.typesetPromise) {
                        try { window.MathJax.typesetPromise([el]); } catch (e) { /* ignore */ }
                    }
                }

                function signal(u) {
                    // JS -> Kotlin bridge; handled and cancelled in filterUrl()
                    window.location.href = u;
                }

                // Build the question + reworded badge + choices + feedback placeholder into the card
                // shell. Choices come from state so this can also run after an async reword resolves.
                function buildBody(cardEl, question, reworded) {
                    var q = document.createElement("div");
                    q.className = "mcat-quiz-question";
                    q.innerHTML = question;
                    cardEl.appendChild(q);

                    if (reworded) {
                        var rb = document.createElement("div");
                        rb.className = "mcat-quiz-reworded";
                        rb.textContent = "\u270e Reworded for you";
                        cardEl.appendChild(rb);
                    } else if (payload.aiEnabled) {
                        // On-demand reword: rewrite this stem in different words (same meaning).
                        var rewordRow = document.createElement("div");
                        rewordRow.className = "mcat-quiz-reword-row";
                        var rewordBtn = document.createElement("button");
                        rewordBtn.type = "button";
                        rewordBtn.className = "mcat-quiz-reword";
                        rewordBtn.textContent = "Reword with AI";
                        rewordBtn.addEventListener("click", function () {
                            if (state.answered) { return; }
                            rewordBtn.disabled = true;
                            rewordBtn.textContent = "Rewording\u2026";
                            signal("mcatreword:reword");
                        });
                        rewordRow.appendChild(rewordBtn);
                        cardEl.appendChild(rewordRow);
                    }

                    var choicesEl = document.createElement("div");
                    choicesEl.className = "mcat-quiz-choices";
                    (state.choices || []).forEach(function (choice, i) {
                        var btn = document.createElement("button");
                        btn.type = "button";
                        btn.className = "mcat-quiz-choice";
                        var letter = document.createElement("span");
                        letter.className = "mcat-quiz-letter";
                        letter.textContent = String.fromCharCode(65 + i);
                        var txt = document.createElement("span");
                        txt.className = "mcat-quiz-choice-text";
                        txt.innerHTML = choice;
                        btn.appendChild(letter);
                        btn.appendChild(txt);
                        btn.addEventListener("click", function () { choose(i); });
                        choicesEl.appendChild(btn);
                    });
                    cardEl.appendChild(choicesEl);

                    var feedback = document.createElement("div");
                    feedback.className = "mcat-quiz-feedback";
                    feedback.hidden = true;
                    cardEl.appendChild(feedback);
                }

                var panel = document.getElementById("_mcat_quiz");
                if (!panel) {
                    panel = document.createElement("div");
                    panel.id = "_mcat_quiz";
                    document.body.appendChild(panel);
                }
                panel.innerHTML = "";
                document.body.classList.add("mcat-quiz-active");

                var card = document.createElement("div");
                card.className = "mcat-quiz-card";

                if (payload.kc) {
                    var kc = document.createElement("div");
                    kc.className = "mcat-quiz-kc";
                    kc.textContent = payload.kc;
                    card.appendChild(kc);
                }

                if (payload.loading) {
                    // Rewording in flight: show a spinner instead of the raw question/choices. The body
                    // is built by _mcatFinishReword when the call settles (success/fallback/error/timeout).
                    var loading = document.createElement("div");
                    loading.className = "mcat-quiz-loading";
                    var spinner = document.createElement("div");
                    spinner.className = "mcat-quiz-spinner";
                    var ltext = document.createElement("div");
                    ltext.className = "mcat-quiz-loading-text";
                    ltext.textContent = "Rewording this question\u2026";
                    loading.appendChild(spinner);
                    loading.appendChild(ltext);
                    card.appendChild(loading);
                } else {
                    buildBody(card, payload.question, !!payload.reworded);
                }

                panel.appendChild(card);
                typeset(panel);

                // Called when an async reword settles. If the spinner is still up, build the body now;
                // otherwise swap the stem text in place, keeping the choices/selection untouched.
                window._mcatFinishReword = function (question, reworded) {
                    if (!state || state.answered) { return; }
                    var c = panel.querySelector(".mcat-quiz-card");
                    if (!c) { return; }
                    var loadingEl = c.querySelector(".mcat-quiz-loading");
                    if (loadingEl) { loadingEl.remove(); }
                    if (!c.querySelector(".mcat-quiz-choices")) {
                        buildBody(c, question, !!reworded);
                        typeset(c);
                        return;
                    }
                    var q = c.querySelector(".mcat-quiz-question");
                    if (q) { q.innerHTML = question; }
                    var rewordRow = c.querySelector(".mcat-quiz-reword-row");
                    if (reworded) {
                        if (rewordRow) { rewordRow.remove(); }
                        if (q && !c.querySelector(".mcat-quiz-reworded")) {
                            var badge = document.createElement("div");
                            badge.className = "mcat-quiz-reworded";
                            badge.textContent = "\u270e Reworded for you";
                            q.insertAdjacentElement("afterend", badge);
                        }
                    } else if (rewordRow) {
                        var rewordBtn = rewordRow.querySelector(".mcat-quiz-reword");
                        if (rewordBtn) { rewordBtn.disabled = false; rewordBtn.textContent = "Reword with AI"; }
                    }
                    typeset(c);
                };

                function choose(index) {
                    if (!state || state.answered) { return; }
                    state.answered = true;
                    var correct = state.correctIndex;
                    var isCorrect = index === correct;

                    var btns = panel.querySelectorAll(".mcat-quiz-choice");
                    for (var k = 0; k < btns.length; k++) {
                        btns[k].disabled = true;
                        if (k === correct) { btns[k].classList.add("correct"); }
                        if (k === index && !isCorrect) { btns[k].classList.add("wrong"); }
                    }

                    // Records the selected choice and reveals the card (also disables auto-advance).
                    // NOTE: scheme MUST be lowercase — Android WebView lowercases URL schemes, so a
                    // camelCase scheme never matches filterUrl and falls through to an external Intent
                    // ("no app can perform this action").
                    signal("mcatchoice:" + index);

                    var fb = card.querySelector(".mcat-quiz-feedback");
                    fb.hidden = false;
                    fb.innerHTML = "";

                    var verdict = document.createElement("div");
                    verdict.className = "mcat-quiz-verdict " + (isCorrect ? "correct" : "wrong");
                    verdict.textContent = isCorrect ? "\u2713 Correct" : "\u2717 Not quite";
                    fb.appendChild(verdict);

                    if (state.explanation) {
                        var exp = document.createElement("div");
                        exp.className = "mcat-quiz-explanation";
                        exp.innerHTML = state.explanation;
                        fb.appendChild(exp);
                    }

                    // Always ask how well they knew it - even on a correct answer, so an honest
                    // learner can flag a lucky guess and get it rescheduled sooner.
                    var prompt = document.createElement("div");
                    prompt.className = "mcat-quiz-rate-prompt";
                    prompt.textContent = isCorrect
                        ? "How well did you know it? Be honest \u2014 a lucky guess isn\u0027t mastery."
                        : "How well did you know it?";
                    fb.appendChild(prompt);

                    var actions = document.createElement("div");
                    actions.className = "mcat-quiz-actions";
                    var rateButtons = isCorrect
                        ? [[1, "Guessed"], [2, "Shaky"], [3, "Knew it"], [4, "Easy"]]
                        : [[1, "Again"], [2, "Hard"], [3, "Good"], [4, "Easy"]];
                    rateButtons.forEach(function (pair) {
                        var b = document.createElement("button");
                        b.type = "button";
                        b.className = "mcat-quiz-rate ease" + pair[0];
                        b.textContent = pair[1];
                        b.addEventListener("click", function () {
                            if (state.graded) { return; }
                            state.graded = true;
                            var rateBtns = actions.querySelectorAll(".mcat-quiz-rate");
                            for (var m = 0; m < rateBtns.length; m++) { rateBtns[m].disabled = true; }
                            signal("mcatgrade:" + pair[0]);
                        });
                        actions.appendChild(b);
                    });
                    fb.appendChild(actions);

                    // Post-answer actions. "Lesson" is always available (opens the concept's lesson,
                    // like the KC badge). "Ask AI" only shows when AI is enabled (a key is set).
                    // Schemes MUST be lowercase (see note above).
                    var askRow = document.createElement("div");
                    askRow.className = "mcat-quiz-ask-row";
                    if (payload.aiEnabled) {
                        var askBtn = document.createElement("button");
                        askBtn.type = "button";
                        askBtn.className = "mcat-quiz-ask";
                        askBtn.textContent = "Ask AI about this";
                        askBtn.addEventListener("click", function () { signal("mcatchatopen:open"); });
                        askRow.appendChild(askBtn);
                    }
                    var lessonBtn = document.createElement("button");
                    lessonBtn.type = "button";
                    lessonBtn.className = "mcat-quiz-lesson" + (isCorrect ? "" : " glow");
                    lessonBtn.textContent = "Lesson";
                    lessonBtn.addEventListener("click", function () { signal("mcatlesson:open"); });
                    askRow.appendChild(lessonBtn);
                    fb.appendChild(askRow);

                    typeset(fb);
                    try { fb.scrollIntoView({ behavior: "smooth", block: "nearest" }); } catch (e) { /* ignore */ }
                }

                // -- Ask AI chat ----------------------------------------------------
                function chatPanel() {
                    var p = document.getElementById("_mcat_chat");
                    if (!p) {
                        p = document.createElement("div");
                        p.id = "_mcat_chat";
                        p.hidden = true;
                        document.body.appendChild(p);
                    }
                    return p;
                }

                window._hideMcatChat = function () {
                    var p = document.getElementById("_mcat_chat");
                    if (p) { p.hidden = true; p.innerHTML = ""; }
                };

                window._mcatChatAddMessage = function (role, text) {
                    var p = document.getElementById("_mcat_chat");
                    if (!p) { return; }
                    var list = p.querySelector(".mcat-chat-messages");
                    if (!list) { return; }
                    var pending = list.querySelector(".mcat-chat-pending");
                    if (pending) { pending.remove(); }
                    var msg = document.createElement("div");
                    msg.className = "mcat-chat-msg " + (role === "user" ? "user" : "ai");
                    msg.textContent = text;
                    list.appendChild(msg);
                    list.scrollTop = list.scrollHeight;
                    typeset(msg);
                };

                window._mcatChatSetPending = function (on) {
                    var p = document.getElementById("_mcat_chat");
                    if (!p) { return; }
                    var list = p.querySelector(".mcat-chat-messages");
                    var input = p.querySelector(".mcat-chat-input");
                    var send = p.querySelector(".mcat-chat-send");
                    if (send) { send.disabled = on; }
                    if (input) { input.disabled = on; }
                    if (on && list && !list.querySelector(".mcat-chat-pending")) {
                        var pend = document.createElement("div");
                        pend.className = "mcat-chat-msg ai mcat-chat-pending";
                        pend.textContent = "\u2026";
                        list.appendChild(pend);
                        list.scrollTop = list.scrollHeight;
                    } else if (!on && list) {
                        var pend2 = list.querySelector(".mcat-chat-pending");
                        if (pend2) { pend2.remove(); }
                        if (input) { try { input.focus(); } catch (e) { /* ignore */ } }
                    }
                };

                window._renderMcatChat = function (intro) {
                    var p = chatPanel();
                    p.hidden = false;
                    p.innerHTML = "";

                    var head = document.createElement("div");
                    head.className = "mcat-chat-head";
                    var title = document.createElement("strong");
                    title.textContent = "Ask AI about this question";
                    head.appendChild(title);
                    var close = document.createElement("button");
                    close.type = "button";
                    close.className = "mcat-chat-close";
                    close.textContent = "\u2715";
                    close.addEventListener("click", function () { window._hideMcatChat(); });
                    head.appendChild(close);
                    p.appendChild(head);

                    var list = document.createElement("div");
                    list.className = "mcat-chat-messages";
                    p.appendChild(list);

                    var row = document.createElement("div");
                    row.className = "mcat-chat-inputrow";
                    var input = document.createElement("textarea");
                    input.className = "mcat-chat-input";
                    input.rows = 2;
                    input.placeholder = "Ask about this question or concept\u2026";
                    var send = document.createElement("button");
                    send.type = "button";
                    send.className = "mcat-chat-send";
                    send.textContent = "Send";
                    var submit = function () {
                        var text = input.value.trim();
                        if (!text) { return; }
                        window._mcatChatAddMessage("user", text);
                        input.value = "";
                        window._mcatChatSetPending(true);
                        signal("mcatchatsend:" + encodeURIComponent(text));
                    };
                    send.addEventListener("click", submit);
                    input.addEventListener("keydown", function (e) {
                        if (e.key === "Enter" && !e.shiftKey) { e.preventDefault(); submit(); }
                    });
                    row.appendChild(input);
                    row.appendChild(send);
                    p.appendChild(row);

                    if (intro) { window._mcatChatAddMessage("ai", intro); }
                    try { input.focus(); } catch (e) { /* ignore */ }
                };
            } catch (e) {
                if (window.console) { console.log("mcat quiz error", e); }
            }
        })();
        """.trimIndent()
}
