# AI Rationale — MCAT Prep

A short note on what AI was built, why, and what was intentionally skipped.

## What was built, and why

1. **Question rewording + equivalence verification** (`anki/qt/aqt/mcat_ai.py`:
  `reword_question` → `questions_equivalent`). After a miss, re-serving the *same*
   underlying question in fresh wording forces re-derivation instead of surface
   pattern-matching — the core concept-scheduler thesis. It is a **two-call**
   design: reword the stem, then a **second model call verifies** the reworded
   stem is semantically equivalent (same correct answer) before it is shown, so a
   bad rewording can't silently change the question.
2. **Scoped "Ask AI about it" tutor** (`anki/qt/aqt/reviewer.py`). Deliberately a
  *bounded augmentation*, not an open chatbot: the system prompt pins the tutor
   to the current question and its MCAT concept and instructs it to **decline
   off-topic requests**. This keeps the feature aligned with studying and shrinks
   the abuse surface.
3. **AI card tagging in Add Cards** (`mcat_ai.suggest_card_tags` + `editor.py`).
  Speeds authoring by proposing `KC::/Difficulty::/IRT/Reasoning` tags. The model
   must choose from the **exact allowed KC list**; `parse_tag_suggestion` drops any
   hallucinated id and clamps difficulty/IRT. The human confirms before adding.
4. **CARS coaching** (`anki/qt/aqt/cars_practice.py`). CARS is reasoning, not
  recall, so AI feedback on *why a distractor is a trap* is high value and low
   hallucination risk. The coach is grounded strictly in the passage text.

## Traceable named source per AI output (rubric §5 — hard limit)

Every AI output is tied to a **named, inspectable source**; nothing is
free-floating generation. This is the rubric hard limit ("AI claims with no
traceable source → the AI section is zero").

| AI output | Named source | How the tie is enforced (code) |
| --- | --- | --- |
| **Question rewording** (reviewer) | The **source card** being reviewed — its exact question stem | `reword_question(stem)` rephrases only that stem; `questions_equivalent(original, reworded)` is a **second** model call that must return "yes" (same correct answer) before the reworded stem is shown. Choices + correct letter are never sent to the reworder. (`mcat_ai_core.py`; Android `mcat/OpenAIClient.kt`) |
| **"Ask AI" tutor** (reviewer) | The **current question** — its stem, choices, and authored explanation | The scoped system prompt embeds this card's text and pins the tutor to "only discuss THIS question"; off-topic + prompt-injection attempts are declined. (`reviewer.py` `_mcat_chat_system`; Android `OpenAIClient.chat`) |
| **KC / difficulty / IRT tagging** (Add Cards) | The **frozen KC map** — `CONCEPT_TOPIC_OPTIONS` in `anki/qt/aqt/editor.py` | The allowed KC ids are passed in the prompt; `parse_tag_suggestion` **drops any id not in that list** (no invented KCs), clamps difficulty to 1–5 and IRT to panel ranges, and the human confirms before the card is added. (`mcat_ai_core.py`) |
| **CARS coaching** (CARS Practice) | The **passage** and its **cited public-domain source** (Project Gutenberg — e.g. Mill *On Liberty*, Du Bois *The Souls of Black Folk*, Wollstonecraft, Veblen) | `CARS_COACH_SYSTEM` forbids outside knowledge; `cars_coach_messages` grounds the coach in the passage text only; each passage carries a `Source:` line with a Gutenberg eBook id + URL and "no text reproduced verbatim". (`cars_practice.py`, `anki/added features/passages-cars.md`) |

**Verify the grounding** with `python evals/ai_eval.py` → `evals/RESULTS.md`: the
tagger is scored only against ids in the frozen KC map; the leakage scan confirms
the reworder's input is the stem only (no answer key); injection resistance
confirms card/passage text is treated as data, not instructions. Full feature
catalog: `docs/ai-features.md`.

## Safety / trust posture

- **Bring-your-own-key** (Track H): the user's own OpenAI key, stored per profile,
**never synced, logged, or committed**. AI is opt-in; the app is fully usable
offline with no key.
- **Generate → verify**: rewording is gated by an equivalence check; tagging is
validated against the KC list; authored lessons render only when
`source=authored && review_status=approved` (AI-generated text stays hidden).
- **Grounding**: tagger constrained to the KC candidate set; CARS coach to the
passage; tutor to the current question.

## Rejected AI approaches (considered and declined)

These are things we deliberately decided **not** to do — not "ran out of time",
but principled rejections that keep the AI section honest and traceable.

- **AI computing or adjusting the score — REJECTED.** No LLM ever touches a number.
  Memory, performance, and readiness are computed **deterministically in the Rust
  engine** and checked on held-out data. An LLM-estimated score would be an
  unverifiable guess with no traceable source — exactly the rubric's auto-fail
  ("made-up or misleading readiness numbers") and AI-section-zero ("AI claims with
  no traceable source"). Keeping AI out of scoring is *why* every number traces to
  the engine + evals.
- **AI grading whether an answer is correct — REJECTED.** MC correctness is
  **objective** (it matches the card's answer key), and the engine gates the score
  on that, never on an LLM's opinion. This is also what lets the anti-gaming rule
  work (a wrong answer can't grow the score even if self-rated "Good").
- **Auto-applying AI tags without a human — REJECTED.** The tagger only *suggests*
  `KC/Difficulty/IRT`; a human confirms and hallucinated ids are dropped. An
  unsupervised tagger would silently corrupt the knowledge graph the whole
  scheduler depends on.
- **An open-ended chatbot — REJECTED** in favour of a per-question tutor scoped to
  the current card that declines off-topic/jailbreak requests. An open chatbot is a
  large hallucination + prompt-injection surface with no named source.
- **AI-generated cards/passages going live — REJECTED.** Only human-authored,
  approved lessons render, and CARS passages are human-written from cited
  public-domain sources (no verbatim text). A wrong generated card is worse than no
  card, and ungrounded generation has no traceable source.
- **LLM as the difficulty / IRT ground truth — REJECTED.** Item parameters come
  from tags plus (planned) empirical calibration from the revlog, not an LLM's
  guess — IRT parameters must be *measured*, not asserted, for the performance
  model to stay honest.
- **Sending the answer key to the reworder — REJECTED.** The reworder gets the stem
  only (never the choices or correct letter), which the leakage check enforces — so
  a rewording can't leak or drift the answer.

## What was intentionally skipped / deferred

- **Notarized macOS build** — needs a paid Apple Developer ID; shipped ad-hoc
signed with a one-line `xattr` un-quarantine instruction instead.
- **Embedded/shared API key** — BYO-key only, on purpose (no per-call cost or
liability, no key in the repo).
- **AI-generated lesson/problem text going live** — gated off until source/eval rules  
exist; only human-authored+approved lessons render.
- **Automated grading of tutor *prose quality*** — this harness grades
correctness, faithfulness, leakage, and injection resistance; tone/pedagogy is
human-reviewed.
- **Fine-tuning / retrieval over a large corpus** — out of scope; prompt-only with
strict candidate lists and verification.

## How it is evaluated

See `RESULTS.md` (produced by `ai_eval.py`): held-out KC-tagging accuracy vs a
lexical baseline, rewording faithfulness (differs + verifier-equivalent +
answer-preserved), a leakage check, and prompt-injection resistance — each against
a **pre-committed** cutoff.