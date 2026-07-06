# Demo video script — MCAT Prep (5–7 min)

A shot-by-shot script for the Sunday demo. Two apps, one Rust engine, three honest
scores, AI, and re-runnable proof. Each segment has **Show** (what's on screen) and
**Say** (voiceover). Total ≈ 6:30.

> Rubric §12 wants: a review session, the Rust change in action, a card synced
> phone→desktop, the three scores with ranges, the AI features, and test results.
> This script hits all of them.

## Pre-flight checklist (set up BEFORE recording)

- [ ] Desktop app running on the **MCAT** deck (`just desktop`), FSRS on, a few cards already reviewed so scores render.
- [ ] Exam date + target score entered on desktop (so the countdown + target ring populate).
- [ ] Android emulator running the latest build, same account, **synced** once.
- [ ] OpenAI key set in **both** apps (Tools → MCAT AI / MCAT AI settings) so AI panels are live.
- [ ] A terminal open at the repo root, ready to run eval commands; `evals/*.svg` charts open in a viewer.
- [ ] One deck with a clear **prerequisite chain** (e.g. Bio::DNA → Genetics) so the "defers unmet prereqs" is visible.
- [ ] Screen recording at 1080p; hide personal tabs/notifications.

---

## 0:00–0:30 — Hook + what this is

- **Show:** Desktop Readiness Dashboard (home), then quickly flip to the Android dashboard side-by-side.
- **Say:**
  - "This is an MCAT prep app — scored 472 to 528, four sections — built on a fork of Anki."
  - "One **Rust engine** powers a **desktop app and an Android companion**, they **sync**, and it answers three different questions: can you *remember* it, can you *use* it on a new question, and are you *ready*."
  - "Everything I show is backed by a re-runnable test — no made-up numbers."

## 0:30–1:50 — The real Rust engine change: the Concept Scheduler

- **Show:** Start a review session on the desktop. Point at the KC badge on a card (e.g. `DNA · Bio/Biochem`). Open Deck Options → **Concept** tab. Then the reviewer **Progress** sidebar with the knowledge graph.
- **Say:**
  - "The core change is in Anki's **Rust** engine, not just the Python screens: a **Concept Scheduler**."
  - "It reads `KC`, `Prereq`, and `MCAT` tags, tracks **Bayesian mastery** per topic, and **defers cards whose prerequisites you haven't learned yet** — so you get foundations before dependents."
  - "It also runs a **review-first budget** — one new topic per four reviews — and computes the scores in-engine, shipped to **both** apps through new backend RPCs."
- **Change callout — the ablation:**
  - **Show:** `evals/ablation.svg` (or `just ablation` output).
  - **Say:** "We tested it fairly: same cards, same study time, three builds — feature **on**, feature **off**, and plain Anki. Prerequisite violations dropped from **56 to about 8 — ~85% fewer** — on every seed. And an honest null: with no prerequisites, all three are equal."
- **Say (tests):** "It's covered by **60 Rust unit tests plus a Python test** that drives it through the real backend, including an **undo + integrity** proof."

## 1:50–3:05 — Three scores, each a range, and the give-up rule

- **Show:** The dashboard hero: **Projected MCAT with a confidence band** (e.g. "508, likely 503–512"), the exam countdown, target-probability ring. Then the four **section cards**: Memory %, Performance range, Readiness range, coverage %.
- **Say:**
  - "Three **separate** scores, never one blended number: **Memory** — FSRS recall; **Performance** — an IRT model on a 118–132 scale; **Readiness** — the projected total **with a range**."
  - "Every score shows its evidence: the range, **percent of the exam covered**, how sure we are, and the top reasons."
- **Change callout — the give-up rule:**
  - **Show:** A low-evidence section showing **"Not enough data yet — need N reviews / X% coverage"** instead of a number (Android shows this cleanly; or the deck-options gate).
  - **Say:** "And it **refuses to guess**: below 500 graded reviews, or under 60% coverage in a section, it **abstains** and tells you what it needs. A good system knows when it doesn't know."

## 3:05–3:45 — Anti-gaming fix (highlight this change)

- **Show:** In review, a KC card renders as **clickable multiple choice**. Pick a **wrong** answer, then tap **"Good"** on the honesty rating. Cut to the dashboard — the score for that topic **did not go up**.
- **Say:**
  - "A subtle but important fix I just made: you **can't cheat the score**. If you get a question **wrong** but tap 'Good', the engine now gates on **actual correctness** — a wrong answer is **neutral** for Good/Easy and a **penalty** for Again/Hard, but it can **never grow** your mastery or performance."
  - "This holds even through the dashboard's history reconstruction — and it's locked in by four new engine tests."

## 3:45–4:45 — AI features (every output has a named source)

- **Show:** In review, click **"Reword with AI"** — the question restates in fresh words. Then **"Ask AI about this"** — ask an off-topic question and show it **declines**. Then Add Cards → AI **KC tagging**. Optionally CARS practice coaching.
- **Say:**
  - "AI is **opt-in, bring-your-own-key, and fully optional** — the app scores you with AI off."
  - "**Rewording** proves performance over memorization — and a **second model call verifies** the reworded question means the same thing. The **tutor is scoped** to the current question and refuses off-topic or injection attempts. **Tagging** is constrained to a frozen topic list; **CARS coaching** is grounded only in the passage."
  - "Every AI output traces to a **named source** — the card, the topic map, or the passage's cited public-domain text."
- **Show:** `evals/ai-eval.svg` or `evals/RESULTS.md`.
- **Say:** "Held-out, pre-committed cutoffs: tagging **88% vs a 35% baseline**, rewording **95%**, prompt-injection **98–100%** across two surfaces, **zero** leaks."

## 4:45–5:45 — Two apps, one engine + sync

- **Show:** Pick up the Android app. Show its dashboard: the **472–528 gauge**, three honesty tiles, the **knowledge lattice**. Review **one card on the phone**. Hit **sync**. Switch to desktop, sync, and show the **review count / that card's progress appears**.
- **Say:**
  - "The phone runs the **same Rust engine** through the AnkiDroid backend — not a re-write. Same cards, same scores, same give-up rule."
  - "I review on the phone, sync, and it **shows up on the desktop** — reviews flow both ways with nothing lost or double-counted."

## 5:45–6:30 — Proof it works, and honesty

- **Show:** Terminal: run `python3 evals/calibration.py` and `python3 evals/performance_eval.py` (fast); show the PASS lines. Flash the calibration + performance charts. Mention `just bench`.
- **Say:**
  - "The models are checked on **held-out** data: memory calibration **Brier 0.168**, performance **accuracy 0.71 / AUC 0.77** — beating simpler baselines. The Python evals are **locked to the Rust engine** at 230-of-230 reference values."
  - "A one-command **50k-card benchmark** shows next-card and grading are sub-millisecond — and I report the **one target we miss** honestly, dashboard refresh, rather than hide it."
  - "Everything here re-runs with one command. That's the point: **honest numbers over flattering ones.**"

## 6:30 — Close

- **Show:** The repo (README) + the desktop installer + the phone build.
- **Say:** "One exam, two apps on one real engine, three scores I can back up, and a fix that stops you from gaming it. Repo, installers, and every eval are in the description."

---

## Cheat-sheet: exact numbers to say

- Ablation: prereq violations **ON ~8.5 vs OFF/plain 56 (~85% fewer)**; honest null on no-prereq decks.
- Memory: **Brier 0.1677, ECE 0.0617** (held-out, synthetic).
- Performance (IRT 3PL): **acc 0.712, AUC 0.769**; beats majority 0.572 / mastery-only 0.615.
- Parity: **230/230** engine↔eval at 1e-9.
- AI: tagging **88% vs 35%** baseline, rewording **95%**, injection tagger **98%** (1 breach) / tutor **100%**, **0** leaks + **0** contamination.
- Give-up rule: **≥500** graded reviews globally; **≥20 items + ≥60% coverage** per section.
- Benchmark (50k deck): next-card p95 **0.06 ms**, grade p95 **0.48 ms**; dashboard-refresh **over** the 500 ms target (disclosed).
- Tests: **60** Rust concept + **4** Python-via-RPC + **12** Android honesty; all green.

## If you want a tight 5-minute cut

Drop these, in order: the CARS coaching beat (4:45), the benchmark detail (5:45), and shorten the AI segment to just rewording + the eval table. Keep the Rust change, the three scores + give-up rule, the anti-gaming fix, and the phone→desktop sync — those are the graded must-haves.
