# MCAT Prep

Study tooling for the **MCAT** (Medical College Admission Test), the AAMC's
standardized exam for U.S. and Canadian medical-school admissions. The MCAT is
reported on a total scaled score of **472–528** (midpoint 500), combining four
sections that are each scored **118–132** (midpoint 125):

- Biological and Biochemical Foundations of Living Sokystems (Bio/Biochem)
- Chemical and Physical Foundations of Biological Systems (Chem/Phys)
- Psychological, Social, and Biological Foundations of Behavior (Psych/Soc)
- Critical Analysis and Reasoning Skills (CARS)

This is the parent workspace for the MCAT prep desktop, Android, and backend
projects.

## Repository Layout

```text
MCAT-Prep/
  anki/                 Desktop app + the shared Rust engine (forked from Anki); the Concept Scheduler lives here.
  Anki-Android/         Android companion app (forked from AnkiDroid).
  Anki-Android-Backend/ Bridge that runs the shared Rust engine on Android. (not modified)
  docs/                 Model descriptions, the Rust-change note, give-up/honesty rules, AI features, KC map.
  evals/                Re-runnable evals + benchmarks (model calibration, ablation, AI checks, speed) + generated charts.
  scripts/              Workspace helper scripts.
```

Feature progress is tracked in `anki/added features/progress.md`. Design and
model docs live in `docs/`; all evidence lives in `evals/` (see
`evals/EVALS-AND-BENCHMARKS.md` for the full index and `evals/PLOTS.md` for the charts).

## What's new vs upstream Anki (MCAT features)

This fork adds an MCAT study system on top of Anki/AnkiDroid. The engine change is
real Rust; the desktop and phone apps share it and sync.

- **Concept Scheduler (real Rust engine change).** Topic-aware scheduling that
parses `KC::`/`Prereq::`/`MCAT::`/`Difficulty::` tags, tracks Bayesian per-KC
mastery, **defers cards whose prerequisites are unmet**, sorts new cards by
readiness, and enforces a review-first session budget — with 5 new proto RPCs
that ship to both apps. Why Rust (not Python): `docs/rust-change-note.md`; files
touched + future-merge difficulty: `docs/rust-upstream-files.md`.
- **Three separate scores, each with a range.** **Memory** (FSRS recall),
**Performance** (IRT 3PL → 118–132), **Readiness** (projected 472–528 with a
band). Each carries a **give-up rule** (abstains instead of guessing) and honesty
fields. Docs: `docs/model-memory.md`, `docs/model-performance.md`,
`docs/model-readiness.md`, `docs/give-up-rule.md`, `docs/honesty-rule.md`.
- **Two apps, one engine.** Desktop (`anki/`) and Android (`Anki-Android/`) run the
same Rust backend and sync; the same status RPC drives both dashboards.
- **AI features (opt-in, bring-your-own-key, offline-safe).** Question rewording
with a second equivalence-verify call, a scoped "Ask AI" tutor, KC/difficulty
tagging, and CARS coaching — every output tied to a **named source**. Catalog:
`docs/ai-features.md`; rationale + source traceability: `evals/AI-RATIONALE.md`.
- **Readiness dashboards.** A desktop deck-browser dashboard and an Android Compose
dashboard with honest score tiles, a projected-score gauge, and a knowledge lattice.
- **Evidence, all re-runnable.** Held-out model evals, a 3-build study-feature
ablation, a 50k-deck speed benchmark, and Python↔Rust engine parity — full index
in `evals/EVALS-AND-BENCHMARKS.md` (summary below).

## Features (desktop + Android)

Desktop (`anki/`) and Android (`Anki-Android/`) run the **same Rust engine** and
sync, so they share the majority of features. Each row lists the feature, the
learning-science reasoning behind it, and how it's implemented. **Platform** is
*Both* unless noted (a few authoring/coaching surfaces are desktop-first). Full
learning-science sourcing lives in `docs/brainlift.md`; the AI features are
cataloged in `docs/ai-features.md`.


| Feature                                                                               | Platform                                     | Learning science / reasoning                                                                                                                                                                                                                     | Implementation                                                                                                                                                                                                                            |
| ------------------------------------------------------------------------------------- | -------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Concept Scheduler** — topic-aware scheduling + per-KC Bayesian mastery              | Both                                         | Knowledge-Space / prerequisite structure + mastery learning + Bayesian (continuous) knowledge tracing: hold back cards whose prerequisites are unmet, sort new cards by readiness, and keep a review-first budget so the graph is built in order | Real Rust engine change in `anki/rslib/src/scheduler/concept.rs`; parses `KC::`/`Prereq::`/`MCAT::`/`Difficulty::` tags; new concept proto RPCs (`GetConceptSchedulerStatus`, …) ship to both apps and update on every graded tagged card |
| **Three honest scores, each a range** — Memory / Performance / Readiness              | Both                                         | These are *different questions*: memory ≠ performance ≠ readiness. Use IRT for ability and report a range, not a point (AAMC itself reports score bands)                                                                                         | Memory = FSRS recall; Performance = IRT 3PL → 118–132; Readiness = projected 472–528 with a ±band; all computed in the shared engine                                                                                                      |
| **Give-up rule + honesty fields**                                                     | Both (Android full; desktop partial)         | "A good system knows when it doesn't know" — abstain instead of showing false precision                                                                                                                                                          | Enforced thresholds (e.g. ≥500 graded answers for readiness sort; per-section ≥20 items **and** ≥60% coverage); honesty fields in `McatHonesty.kt`                                                                                        |
| **Readiness dashboard**                                                               | Both (desktop deck-browser; Android Compose) | Show the projected score with its range, % of the exam covered, and the single best next thing to study — not one blurred number                                                                                                                 | Desktop `deckbrowser.py` + `mcat_ui.py` (`build_dashboard_html`); Android `ConceptSchedulerStatusScreen.kt` + `ReadinessGauge.kt`                                                                                                         |
| **Knowledge lattice / concept-graph view**                                            | Both                                         | Memory is a schema/graph, not a flat list; surface mastered / in-progress / next-up / locked topics (Knowledge-Space inner & outer fringe)                                                                                                       | Android `ConceptLatticeGraph.kt`; desktop deck-options `ConceptSchedulerOptions.svelte`                                                                                                                                                   |
| **KC badges on review cards**                                                         | Both                                         | Tie each retrieval to its knowledge component so practice reinforces structure; untagged cards keep the normal review experience                                                                                                                 | Reviewer renders e.g. `DNA · Bio/Biochem` from the card's tags                                                                                                                                                                            |
| **Non-blocking reviewer Progress view**                                               | Both                                         | Metacognition / self-monitoring *without* interrupting the retrieval session                                                                                                                                                                     | Android `Progress` shortcut + `ConceptSchedulerStatusBottomSheet.kt`; desktop reviewer progress + Deck Options                                                                                                                            |
| **Add-Cards metadata controls** (KC, prerequisite, MCAT section, difficulty, IRT a/c) | Both                                         | Item modeling: tag every item with its knowledge component and IRT parameters so scoring is per-item, not per-deck                                                                                                                               | Desktop `editor.py`; Android `ConceptSectionSelect.kt`                                                                                                                                                                                    |
| **Topic picker** — "Pick your next topic"                                             | Desktop                                      | Learner agency / self-regulated learning (adult learners want control) over an opaque queue, bounded by the ready outer-fringe                                                                                                                   | Reviewer topic chooser (`reviewer.py::_start_concept_topic`)                                                                                                                                                                              |
| **Lessons-before-retrieval**                                                          | Desktop                                      | Retrieval practice needs something already encoded; novices need worked examples/guidance before being quizzed (Kirschner/Sweller/Clark; Hendrick's "lethal mutation of retrieval")                                                              | Authored lesson gate before the multiple-choice step (`reviewer.py::_open_lesson_for_kc`, `GetConceptLesson` RPC)                                                                                                                         |
| **Interactive multiple-choice + honest self-rating**                                  | Both                                         | Measure *performance / transfer* via applied MCQs, not verbatim recognition; an honest self-report counters the Good/Easy "illusion of knowing" (Karpicke)                                                                                       | Reviewer MC mode; Android `mcat/McatMultipleChoice.kt`; grades feed the engine                                                                                                                                                            |
| **AI — question rewording + equivalence verify**                                      | Both                                         | The memory→performance bridge: fresh wording forces *re-derivation* (a desirable difficulty that builds transfer) instead of recognizing memorized card text                                                                                     | Two-call generate→verify (`mcat_ai_core.py`, Android `OpenAIClient.kt`); answer choices/key are never sent to the reworder                                                                                                                |
| **AI — scoped "Ask AI" tutor**                                                        | Both                                         | On-demand guidance + feedback, deliberately *bounded* to the current item to prevent hallucination and off-task drift                                                                                                                            | Reviewer chat (`reviewer.py::_mcat_chat_open`); injection-resistant prompt grounded in the current question                                                                                                                               |
| **AI — KC / difficulty / IRT tagging** (Add Cards)                                    | Desktop (+ Android metadata)                 | Automatic item generation to speed *valid* tagging; measured to beat a simpler lexical baseline                                                                                                                                                  | Suggestions validated against the frozen KC list in `editor.py`; a human confirms before the card is added                                                                                                                                |
| **AI — CARS coaching**                                                                | Desktop                                      | Reading/reasoning strategy: name the trap type and give one transferable takeaway (feedback reduces the downsides of MCQs — Butler & Roediger)                                                                                                   | Passage-grounded (`cars_practice.py`); cites a public-domain source, never outside knowledge                                                                                                                                              |


Notes:

- **AI is opt-in, bring-your-own-key, and offline-safe.** Every study path works with  
AI off; keys are stored per-profile/per-install and never synced or logged (also an  
equity/access choice — the app can't assume constant connectivity).

## Mobile App

`Anki-Android/` is the Android client (pulled from AnkiDroid), and
`Anki-Android-Backend/` is the backend bridge that connects the Anki backend
code to the Android app. Its features are in the shared
[Features](#features-desktop--android) table above.

### How to Run

Requires Java 21 and the Android SDK (see
`scripts/rebuild-ankidroid-local-backend.sh` for environment details).

```bash
# Build the local backend bridge and install the app on a device/emulator
just rebuild-local-backend

# Or run the steps individually
just backend          # build the local rsdroid backend
just android-install  # install the Play debug build
just android-check    # run Android unit tests
```

## Desktop App

`anki/` is the desktop application and upstream backend code pulled from Anki. Its
features are in the shared [Features](#features-desktop--android) table above.

### How to Run

```bash
just desktop        # run the desktop app
just desktop-check  # run desktop checks
```

Current caveats: the 50k-deck benchmark's dashboard-refresh p95 is over target (see
`evals/BENCHMARK.md`), and full `just desktop-check` is still blocked by a known
unrelated Qt installer test.

### Build a Downloadable Installer

To get a standalone desktop app you can download and double-click to install
(no source checkout needed on the target machine), build an installer for your
current OS:

```bash
just desktop-installer   # wraps anki/tools/build-installer (RELEASE=2 ./ninja installer)
```

This writes a platform-specific package to `anki/out/installer/dist/`:

- macOS: a `.dmg` disk image
- Windows: an `.msi` installer
- Linux: a tarball (`.tar.zst`)

The installer is built with [Briefcase](https://briefcase.readthedocs.io/),
configured under `anki/qt/installer/`. It requires the full desktop build
toolchain (see `anki/docs/development.md`), and only targets the OS you run it
on.

To produce signed installers for every platform at once (macOS Intel/Apple
Silicon, Windows x64/ARM, Linux x86/ARM), the `anki/` project ships a GitHub
Actions release workflow driven by the `release` just module:

```bash
cd anki
just release build --ref <branch>   # unsigned installers for all platforms via CI
just release public --ref <branch>  # signed build + draft GitHub Release
```

The `public` recipe uploads the installers to a draft GitHub Release, which is
the download page users actually get the desktop app from.

## Models, evals & benchmarks

Every number is produced by a committed script and is re-runnable; the full index
is `evals/EVALS-AND-BENCHMARKS.md`. Headlines:


| Area                                       | Result                                                                                              | Cutoff                                                              | Pass?   | Source                     |
| ------------------------------------------ | --------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------- | ------- | -------------------------- |
| Memory calibration                         | Brier 0.1677, ECE 0.0617                                                                            | Brier ≤0.25, ECE ≤0.1                                               | PASS    | `evals/MODEL-EVALS.md`     |
| Performance (IRT 3PL)                      | acc 0.712, AUC 0.769 (beats 0.572 / 0.615 baselines)                                                | acc ≥0.7, AUC ≥0.7                                                  | PASS    | `evals/MODEL-EVALS.md`     |
| Engine ↔ eval parity                       | 230/230 @ 1e-9                                                                                      | all pass                                                            | PASS    | `evals/ENGINE-FIDELITY.md` |
| Study-feature ablation (3-build)           | prereq violations ON 8.5 vs OFF/plain 56 (~85% fewer)                                               | per-seed ON<OFF & ON<plain                                          | PASS    | `evals/ABLATION.md`        |
| Speed benchmark (50k deck, desktop engine) | next-card p95 0.06 ms, grade p95 0.48 ms; **dashboard-refresh p95 680 ms misses the 500 ms target** | §10: ack <50, next-card <100, dash-load <1000, dash-refresh <500 ms | PARTIAL | `evals/BENCHMARK.md`       |
| Tests                                      | 60 Rust + 4 Python-via-RPC + 12 Android honesty                                                     | —                                                                   | PASS    | `docs/rust-change-note.md` |


**Benchmark scope + honest gap (§10 asks for "desktop and phone").** `just bench`
times the **shared Rust engine as run on the desktop (macOS arm64) host** — it drives
the exact backend calls the app makes (`get_queued_cards`, `build_answer`/`answer_card`,
`get_concept_scheduler_status`), not the phone. Because the *same* engine ships to Android
via `Anki-Android-Backend`, the algorithmic cost carries over (next-card/grade are
O(queue), not O(deck)), but §10's explicit **phone-side** numbers — button-ack, cold start,
and memory on a mid-range device — are **not yet measured on-device**. That requires an
instrumented Android benchmark (an on-device `androidTest`/macrobenchmark that times the
same RPCs through `rsdroid`), which is not wired up. Disclosed as a gap, not claimed.

```bash
python3 evals/calibration.py        # memory calibration
python3 evals/performance_eval.py   # performance / IRT
python3 evals/test_parity.py        # Python <-> Rust parity
just ablation                       # 3-build study-feature ablation
just bench                          # 50k-deck speed benchmark
```

Model write-ups (memory / performance / readiness) and the give-up + honesty rules
live in `docs/model-*.md`, `docs/give-up-rule.md`, and `docs/honesty-rule.md`. AI
feature evals are detailed just below.

### Charts

Every chart is **hand-rolled SVG (pure Python stdlib — no matplotlib/numpy)**, so it
regenerates offline and versions cleanly. Full catalog + style notes: `evals/PLOTS.md`.

Reproduce them all:

```sh
python3 evals/plots.py         # performance-roc / -reliability / -baselines, ablation-secondary, ai-eval, readiness-projection
python3 evals/calibration.py   # memory-calibration.svg
just ablation                  # ablation.svg       (+ evals/ABLATION.md)
just bench                     # bench-latency.svg  (+ evals/BENCHMARK.md)
```

**Memory & performance models**

![Memory calibration reliability diagram: predicted vs observed recall by decile (Brier 0.1677, ECE 0.0617)](evals/memory-calibration.svg)

![Performance ROC curve: IRT 3PL performance model, AUC 0.769](evals/performance-roc.svg)

![Performance reliability: P(correct) calibration, Brier 0.1957](evals/performance-reliability.svg)

![Performance vs baselines: IRT 3PL vs mastery-only vs majority, with the 0.70 accuracy cutoff](evals/performance-baselines.svg)

**Study-feature ablation (3 builds, same learners)**

![Ablation, prerequisite violations: scheduler ON 8.5 vs OFF / plain-Anki 56 (~85% fewer)](evals/ablation.svg)

![Ablation, secondary metrics: held-out accuracy / memory / coverage across ON / OFF / plain](evals/ablation-secondary.svg)

**Readiness & speed**

![Readiness as a range, not a point: projected total 472-528 with per-section 118-132 bands and one abstain](evals/readiness-projection.svg)

![Engine latency p95 vs target on a 50k-card deck (dashboard-refresh over target)](evals/bench-latency.svg)

The AI-eval pass-rate chart is embedded in the **AI evaluation** section below.

## AI evaluation (held-out, pre-committed cutoffs)

The AI features (question rewording + equivalence verification, KC card tagging,
and the scoped "Ask AI about it" tutor) are measured against a **held-out**
dataset with cutoffs that were **committed before the run** (the `CUTOFFS` dict
in `evals/ai_eval.py`). The numbers below are quoted from the latest run in
`evals/RESULTS.md` (`gpt-4o-mini`, 2026-07-06).

**Overall this run: 4 of 5 cutoffs met — reported honestly, misses included.**
KC-tagging beats its baseline and clears the cutoff, rewording faithfulness and the
leakage check pass, and the Ask-AI tutor resists every attack. The card-tagger
injection surface resisted **39/40** — a single breach (`EVAL-INJT-31`) lands just
under the strict **100%** cutoff. We report it as measured rather than tune it away:
one adversarial miss out of 40 is the held-out test doing its job.


| Metric                         | Result                | Baseline                            | Pre-committed cutoff | Pass? |
| ------------------------------ | --------------------- | ----------------------------------- | -------------------- | ----- |
| KC-tagging top-1 accuracy      | 88% (35/40; 5 wrong)  | 35% lexical name-overlap (26 wrong) | >= 80%               | PASS  |
| Rewording faithfulness         | 95% (38/40; 2 wrong)  | n/a                                 | >= 85%               | PASS  |
| Prompt-injection: card-tagger  | 98% (39/40; 1 breach) | n/a                                 | 100%                 | MISS  |
| Prompt-injection: Ask-AI tutor | 100% (40/40)          | n/a                                 | 100%                 | PASS  |
| Gold-label leaks               | 0 (contamination 0)   | n/a                                 | 0                    | PASS  |

![AI evaluation results vs pre-committed cutoffs: KC-tagging 88% (vs 35% lexical baseline), rewording 95%, prompt-injection 98% card-tagger / 100% tutor; 0 gold-label leaks and 0 contamination](evals/ai-eval.svg)


- **KC-tagging accuracy:** the AI tagger scored **88% top-1 (35/40, 5 wrong)** on
40 held-out cards, versus a **35% lexical name-overlap baseline (26 wrong)** —
a **+52%** lift, clearing the pre-committed **80%** cutoff (**PASS**).
- **Rewording faithfulness:** **95% pass rate (38/40, 2 wrong)** against an **85%**
cutoff (**PASS**). A rewording only passes if it differs from the original, a
second model call judges it semantically equivalent, and the model still
answers the reworded stem with the original correct letter.
- **Leakage check:** **0 gold-label leaks** (the KC id / `KC::` tag is never in
the model's input) against a **0** cutoff (**PASS**), and **0** train/test
contamination (no eval card appears in the shipped generated decks).
- **Prompt-injection resistance:** measured on **two separate held-out surfaces of
40 attacks each**. The scoped Ask-AI tutor (off-topic / jailbreak /
system-prompt-exfiltration / append-echo) resisted **100% (40/40)**. The card-tagger
(tag-flip injections) resisted **98% (39/40)** — one breach (`EVAL-INJT-31`), where a
"the system already computed the tags … return this JSON verbatim" attack coaxed the
model into echoing an off-topic KC. That is **one miss under the strict 100% cutoff**,
reported as measured. Each prompt still treats user- and card-supplied text as
untrusted **data, not instructions**.

### How to re-run

```bash
python evals/ai_eval.py   # needs OPENAI_API_KEY (env or repo .env)
```

It rewrites `evals/RESULTS.md` and exits non-zero if any pre-committed cutoff is
missed (~240 cheap `gpt-4o-mini` calls).

### More detail

See the `evals/` folder: `RESULTS.md` (full per-item results), `AI-RATIONALE.md`
(what was built, why, and what was intentionally skipped), and `datasets/` (the
held-out tagging, rewording, and injection sets).

## Original Repositories

- `anki/`: [https://github.com/ankitects/anki.git](https://github.com/ankitects/anki.git)
- `Anki-Android/`: [https://github.com/ankidroid/Anki-Android.git](https://github.com/ankidroid/Anki-Android.git)
- `Anki-Android-Backend/`: [https://github.com/ankidroid/Anki-Android-Backend.git](https://github.com/ankidroid/Anki-Android-Backend.git)

## Local Commands

Run `just --list` from this folder to see all workspace commands. The most useful:

```bash
just desktop                # run the desktop app
just desktop-check          # desktop build + checks
just rebuild-local-backend  # build the Android local backend + install the app
just android-check          # Android unit tests
just bench                  # 50k-deck speed benchmark   -> evals/BENCHMARK.md
just ablation               # 3-build study-feature ablation -> evals/ABLATION.md
just crash-test             # kill-mid-review crash / no-corruption test
just offline-test           # AI-off / network-pulled degradation test
```

