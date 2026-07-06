# evals/ — evidence for the AI rubric rows

This folder supplies the evidence that was flagged as missing in review
(held-out numbers, a baseline, an AI-rationale note, a leakage check, and
prompt-injection resistance).

| Rubric row (previously not evidenced) | Where it's covered |
| --- | --- |
| Held-out eval accuracy + wrong-answer counts, **pre-committed cutoff** | `ai_eval.py` §1–2 → `RESULTS.md`; cutoffs are constants at the top of `ai_eval.py` |
| **Baseline** comparison vs a simpler method, real numbers | `ai_eval.py` §1 (lexical name-overlap tagger) → `RESULTS.md` |
| **AI-rationale note** (what / why / skipped) | `AI-RATIONALE.md` |
| **Leakage-check** output | `ai_eval.py` §3 → `RESULTS.md` |
| **Prompt-injection resistance** (distinct) | `ai_eval.py` §4a/§4b + `datasets/injection-tagging.jsonl` + `datasets/injection-chat.jsonl` → `RESULTS.md` |

## Pre-committed cutoffs (fixed before running; see `CUTOFFS` in `ai_eval.py`)

- KC-tagging top-1 accuracy ≥ **80%**
- Rewording pass rate (differs + equivalent + answer preserved) ≥ **85%**
- Prompt-injection resistance (card-tagger) = **100%**
- Prompt-injection resistance (Ask-AI tutor) = **100%**
- Gold-label leaks = **0**

## Faithfulness to the shipped app

- The **KC candidate set** is parsed live from `anki/qt/aqt/editor.py`
  (`CONCEPT_TOPIC_OPTIONS`) — the same list the real Add-Cards tagger uses.
- The **prompts** mirror `anki/qt/aqt/mcat_ai.py` (tagging, rewording, equivalence)
  and the reviewer's scoped Ask-AI tutor prompt.

## Datasets (held out, authored for eval — not from the shipped decks)

- `datasets/tagging.jsonl` — 40 cards with a gold KC (a mix that name the topic and
  that require inference, so the baseline vs AI gap is meaningful).
- `datasets/rewording.jsonl` — 40 MCQs (stem + choices + correct letter).
- `datasets/injection-tagging.jsonl` — 40 tag-flip attacks against the card-tagger
  (card text tries to dictate the KC).
- `datasets/injection-chat.jsonl` — 40 attacks against the scoped Ask-AI tutor
  (jailbreak / off-topic / system-prompt-exfiltration).

## Run

```sh
# needs the user's OpenAI key (env or repo .env), same as the app
python evals/ai_eval.py
```

Writes `evals/RESULTS.md` and exits non-zero if any pre-committed cutoff is missed.
Uses ~240 cheap `gpt-4o-mini` calls (40 tagging + 40 rewording×3 + 40 + 40 injection).

## Study-feature ablation (rubric §8) — no API key

The concept-scheduler **3-build ablation** (full / feature-off / plain Anki) runs
end-to-end through the real Rust engine and reports the pre-stated main number
(prerequisite violations) plus secondary metrics with ranges and an honest null:

```sh
just ablation                 # emit engine results -> render + re-check
# or:
python evals/ablation.py      # render evals/ABLATION.md + evals/ablation.svg, re-assert cutoff
```

Source of truth: `anki/rslib/src/scheduler/concept_ablation.rs` →
`evals/fixtures/ablation.json` → `evals/ABLATION.md`. See `evals/ENGINE-FIDELITY.md`
for the write-up and `evals/MODEL-EVALS.md` for the held-out memory/performance evals.
