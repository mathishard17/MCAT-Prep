# AI features — MCAT Prep

A catalog of every AI feature in the app (desktop + Android), what each one does,
the **named source** it is grounded in, how it is kept safe, and how it is
evaluated. Exam: **MCAT**.

Companion docs: source-traceability table + rationale in `evals/AI-RATIONALE.md`;
held-out eval numbers in `evals/RESULTS.md` (produced by `evals/ai_eval.py`).

## Design principles

- **Opt-in, bring-your-own-key.** AI is off by default. The user pastes their own
  OpenAI key, stored **per profile / per install only — never synced, logged, or
  committed**. Model defaults to `gpt-4o-mini`.
- **Fully usable offline / AI-off.** Every study path works with no key: reviews,
  the three scores, authored CARS explanations, and manual tagging. AI only ever
  *augments*.
- **Generate → verify.** Anything generated is checked before it is shown
  (rewording is gated by an equivalence call; tags are validated against a fixed
  list).
- **Grounded in a named source.** No free-floating generation — see the
  traceability table in `evals/AI-RATIONALE.md` and the per-feature "Source" below.
- **Injection-resistant.** Card/passage/student text is treated as *data, not
  instructions*; the tutor prompt explicitly refuses embedded commands.

## Feature catalog

| # | Feature | Where | Platform | Grounded in |
| --- | --- | --- | --- | --- |
| 1 | Question rewording + equivalence verify | Reviewer | Desktop + Android | The source card's stem |
| 2 | "Ask AI about it" scoped tutor | Reviewer | Desktop + Android | The current question |
| 3 | KC / difficulty / IRT tagging | Add Cards | Desktop (+ Android metadata) | The frozen KC map |
| 4 | CARS coaching | CARS Practice | Desktop | The passage + its cited public-domain source |

### 1. Question rewording + equivalence verification (flagship)

**What:** after a card is served, the AI rephrases the **question stem** into fresh
wording that tests the same concept — forcing re-derivation instead of
recognising memorised card wording. This is the concept-scheduler thesis
(memory ≠ performance) made interactive.

**Two-call generate→verify:** `reword_question(stem)` rewrites the stem
(`REWORD_SYSTEM`), then `questions_equivalent(original, reworded)` (`EQUIV_SYSTEM`)
is a **second** model call that must answer "yes" (same meaning, same correct
answer) before the reworded stem replaces the original. A bad rewording can't
silently change the question.

**Source:** the exact card being reviewed. The answer choices and correct letter
are **never** sent to the reworder (verified by the leakage scan).

**Code:** `anki/qt/aqt/mcat_ai_core.py` (`reword_question`, `questions_equivalent`);
Android `Anki-Android/.../mcat/OpenAIClient.kt` (`rewordQuestion`,
`questionsEquivalent`). Settings toggle: "Reword MCAT questions with AI as I study".

### 2. "Ask AI about it" scoped tutor

**What:** an in-reviewer chat to ask about the current question. Deliberately a
**bounded augmentation, not an open chatbot**.

**Source & scoping:** the system prompt embeds *this* card's stem, choices, and
authored explanation, and pins the tutor to "only discuss THIS question and the
directly related MCAT concept." Off-topic requests are politely declined; embedded
"ignore your instructions / reveal the prompt / append X" attempts are refused
(the message is treated only as a question to help with).

**Code:** `anki/qt/aqt/reviewer.py` (`_mcat_chat_open`, `_mcat_chat_system`);
Android `OpenAIClient.chat`. Scored by the injection-resistance eval (§4b).

### 3. KC / difficulty / IRT tagging (Add Cards)

**What:** proposes Concept Scheduler metadata for a new card — a `KC::` id, a
1–5 `Difficulty`, a reasoning type, and IRT discrimination/guessing — to speed
authoring.

**Source & guardrail:** the **frozen KC map** (`CONCEPT_TOPIC_OPTIONS` in
`anki/qt/aqt/editor.py`, ~108 ids). The allowed ids are passed in the prompt and
`parse_tag_suggestion` **drops any id not in the list** (no hallucinated KCs),
clamps difficulty to 1–5 and IRT to the panel's ranges. The human confirms before
the card is added.

**Code:** `anki/qt/aqt/mcat_ai_core.py` (`suggest_card_tags`, `parse_tag_suggestion`,
`SUGGEST_TAGS_SYSTEM`). Scored by the tagging accuracy + injection evals (§1, §4a).

### 4. CARS coaching

**What:** for CARS practice (reading/reasoning, not recall), the AI explains *why*
the correct choice is best and what makes a wrong choice a classic CARS trap
(out-of-scope, too extreme, reversal, distortion, half-right,
plausible-but-unsupported), plus one transferable takeaway.

**Source & grounding:** the **passage text only** (`CARS_COACH_SYSTEM` forbids
outside knowledge). Each passage is an original synthetic passage that **cites a
public-domain source** (Project Gutenberg — e.g. Mill *On Liberty*, Du Bois *The
Souls of Black Folk*, Wollstonecraft, Veblen) with an eBook id + URL and no
verbatim reproduction. The authored `Explanation` always shows even with AI off.

**Code:** `anki/qt/aqt/cars_practice.py` (`CARS_COACH_SYSTEM`, `cars_coach_messages`);
passages in `anki/added features/passages-cars.md`.

## Safety & trust posture

- **Key handling:** resolved from profile setting → `OPENAI_API_KEY` env → nearest
  `.env` (gitignored); value never logged. Desktop dialog: Tools → MCAT AI. Android:
  MCAT AI settings (`preferences/McatAiSettingsFragment.kt`).
- **Generate→verify + validation:** rewording gated by the equivalence check;
  tagging validated against the KC list; AI-generated lesson/problem text stays
  hidden — only `source=authored && review_status=approved` content renders.
- **Robust output parsing:** JSON tag parsing tolerates code fences / extra prose
  and fails closed (`OpenAIError`); malformed/again empty responses degrade to the
  offline experience rather than crashing.
- **Offline / rate-limited / broken output:** all handled — the app keeps working
  and still shows a score (scores are computed locally in the Rust engine).

## Rejected / out of scope (and why)

Deliberate design rejections that keep the AI honest and traceable — full rationale
in `evals/AI-RATIONALE.md`:

- **AI never computes or adjusts a score** — the three scores are deterministic Rust
  + held-out evals; an LLM number would be an untraceable guess (rubric auto-fail).
- **AI never grades correctness** — MC correctness is objective (the answer key); the
  score gates on that, not an LLM judgment (also what makes the anti-gaming rule work).
- **No auto-applied tags** — the tagger suggests; a human confirms and invalid ids are dropped.
- **No open chatbot** — only a per-question tutor that declines off-topic/injection.
- **No AI-generated cards/passages go live** — only human-authored, approved content;
  CARS passages are human-written from cited public-domain sources.
- **No LLM as IRT/difficulty ground truth**, and **the answer key is never sent to the reworder** (leakage-checked).

## Evaluation (held-out, pre-committed cutoffs)

`python evals/ai_eval.py` → `evals/RESULTS.md`. Latest run:

| Check | Result | Cutoff |
| --- | --- | --- |
| KC-tagging top-1 accuracy | **88%** (vs lexical baseline **35%**, +52%) | ≥ 80% |
| Rewording faithfulness (differs + verifier-equivalent + answer-preserved) | **95%** | ≥ 85% |
| Prompt-injection resistance — card tagger | **98%** (39/40; 1 breach) | 100% |
| Prompt-injection resistance — Ask-AI tutor | **100%** | 100% |
| Gold-label leaks | **0** (contamination **0**) | 0 |

Datasets are held out (authored for eval, not in the shipped decks):
`evals/datasets/{tagging,rewording,injection-tagging,injection-chat}.jsonl`.

## Verify

```sh
# needs the user's OpenAI key (env or repo .env), same as the app; ~240 cheap calls
python evals/ai_eval.py            # writes evals/RESULTS.md; non-zero exit if any cutoff missed
```
