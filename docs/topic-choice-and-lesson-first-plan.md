# Plan: Choose-Next-Topic (2+1+1 from the outer fringe) + Lesson-First

Implementation plan only. Turns two designs into concrete, buildable steps:

1. **Choose what to learn next** by looking at the *outer fringe* — recommend
   **2 topics in the current section + 1 in each of the other two** super-sections.
2. **Lesson-first** — when a new topic is started, show that KC's lesson (from
   `lessons-<discipline>.md`) **before** its cards, with a "Start retrieval" button.

Grounding docs (already frozen): `topic-picker-design.md` (picker + the shipped
`set_concept_selected_topic` write path), `lessons.md` (7-section lesson schema +
data format), `lesson-contract.md` (the pick → lesson → cards seam + display gate),
`kc-map-unified.md` (172 KCs / edges / sections).

> Line numbers in `lesson-contract.md` / `lessons.md` are stale (reviewer.py has
> changed a lot). Reference reviewer functions by **name**, not line.

---

## Status (implemented)

**M1–M3 done** (see `progress.md`). Decisions taken:

- **Gate:** render all `source == authored` lessons now (any review status), incl. the
  169 `needs_review` ones; `ai_generated` stays gated. (Lessons are being reviewed +
  gaining images in parallel; the gate can tighten to `approved` later without code
  changes.)
- **Parser:** Rust (`parse_lessons_md` + `GetConceptLesson` RPC).
- **Scope:** M1–M3. **M4 deferred** (once-per-KC first-encounter auto-open + encoded
  flag).
- **Diagrams:** lessons now carry a `- **Diagram:**` field (markdown image into
  `lesson-diagrams/` or prose). The panel shows the alt text as a caption; serving
  the SVGs to the webview is a follow-up.

## 0. What already ships (do not rebuild)

- Layered, scroll+zoom concept graph in the reviewer whose **outer-fringe** nodes
  are green "ready to start" and **clickable** (`reviewer.py` injected JS:
  `layeredUnits`, node `available` class, `pycmd("conceptStart:<KC>")`).
- Backend write path: `ConceptSchedulerState.selected_topic`,
  `Collection::set_concept_selected_topic` (validates outer-fringe, persists,
  `clear_study_queues()`), `SetConceptSelectedTopic` RPC, live-session seeding in
  `QueueBuilder::build()`. Python: `Reviewer._start_concept_topic`.
- Lesson content: 172 authored lessons across `lessons-{biology,biochemistry,
  general-chemistry,organic-chemistry,physics,psych-soc}.md`, parseable
  `## <KC id>` / `### LESSON-<slug>` / `- **Key:** value` blocks (same convention
  as `mcat_demo_cards.md`). Demo KCs are `approved`; the rest are `needs_review`.

So this plan adds: (Part 1) a recommendation *policy* + surfacing, and (Part 2) a
lesson *resolver* + reviewer panel + the pick→lesson→cards ordering.

---

## Part 1 — Choose-next-topic recommendation (2 + 1 + 1)

### 1.1 Policy

From the outer fringe (KCs whose prereqs are met, not yet mastered), pick up to 4
**suggested** next topics, spanning sections:

- **2** from the *current* super-section.
- **1** from each of the *other two* super-sections.

Super-sections (mirror `sectionOf` in `reviewer.py` / `recommendation_section`):
`Bio_Biochem` (Bio, Biochem), `Chem_Phys` (GenChem, Orgo, Physics),
`Psych_Soc` (PsychSoc). Within each section, rank by existing
`readiness_score = prerequisite_mastery * (1 - target_mastery)`, highest first.

**"Current section"** (decision — recommend this order):
1. Section of the live `active_topic`/`selected_topic` if set.
2. Else the section with the most *recent* answered concept KCs (most `answered`
   among inner-fringe), i.e. where the learner is actually working.
3. Else (cold start / true baseline) — no current section: take the top pick from
   **each** section (breadth first), then fill the remaining slots by global
   readiness. This is what delivers "a baseline across all sections at the start."

Fill rules: if the current section has fewer than 2 ready topics, or another
section has 0, backfill the empty slots from remaining outer-fringe by global
readiness so the list is still useful (never error, never pad with locked KCs).

### 1.2 Backend

`rslib/src/scheduler/concept_demo.rs`:

- Rewrite `recommended_topics(graph, persisted)` to apply 1.1 (keep the
  `ConceptTopicRecommendation` row shape: `id, readiness_score, mastery,
  prerequisite_mastery, selectable`). Add a `recommendation_section(id) -> &str`
  prefix classifier. Add `current_section(persisted) -> Option<&str>` per 1.1.
- Add a `recommended: bool` (or reuse: mark the top-N ids) to the graph read model
  so the reviewer can badge them. Cheapest: add `bool recommended = N;` to
  `ConceptGraphNode` in `proto/anki/scheduler.proto` and set it in the node builder
  when the id is in `recommended_topics(...)`. (Alternative: the reviewer matches
  `status.recommendations[].id` against nodes — no proto change. **Recommend the
  proto flag** so Android/clients share it.)

Proto: `ConceptGraphNode { … bool recommended = <next>; }`. Regenerate.

### 1.3 Surfacing (reviewer graph)

`reviewer.py` payload + JS:

- Add `recommended` to each node dict in `_concept_graph_payload`.
- In the graph JS, badge recommended nodes on top of the green "available" ring —
  a small star/dot or a bolder ring + always-on label — so among all green
  frontier nodes the 2+1+1 picks are the obvious "do these next". Add a legend row
  ("★ = suggested next").
- Keep every outer-fringe node clickable (the user can still start anything); the
  recommendation only *emphasizes* 4 of them.

Optional (nice, not required now): the explainable picker *card* in
`ConceptSchedulerOptions.svelte` (deck options) per `topic-picker-design.md` §1.5.
Defer unless asked.

### 1.4 Tests (Rust)

- `recommends_two_in_current_section_and_one_each_other`: seed a state where the
  current section has ≥2 ready KCs and the others ≥1; assert 2+1+1 by section.
- `cold_start_recommendations_span_sections`: fresh state → picks span all sections
  (baseline).
- `backfills_when_a_section_has_no_ready_topic`: a section empty → slots filled by
  readiness, total still ≤ 5, no locked KCs.

---

## Part 2 — Lesson-first flow

Goal (per `lesson-contract.md` §2.1): pick topic `T` → **open T's lesson** →
"Start retrieval" → introduce T's cards.

### 2.1 Lesson store + resolver (Rust, mirrors card parsing)

New `rslib/src/scheduler/concept_lessons.rs` (or fold into `concept_demo.rs`):

- `parse_lessons_md(&str) -> Vec<ParsedLesson>` mirroring `parse_cards_md` /
  `parse_generated_mcat_cards`: split on `## <KC id>`, read `- **Key:** value`
  bullets (Overview, Key Concepts (list), Prerequisite Reminder, Worked Example,
  Common Misconception, First Retrieval Prompt, Related KCs, Title, Section,
  Source, Review Status).
- `include_str!` the six `lessons-<discipline>.md` files + `lessons.md` demo stubs;
  build a `BTreeMap<KnowledgeComponentId, Lesson>` (demo `approved` prose wins for
  the demo KCs, matching the note at the top of `lessons-biochemistry.md`).
- Resolver: `Collection::concept_lesson(kc) -> Option<Lesson>` and
  `concept_lesson_exists(kc) -> bool`, applying the **display gate** (see 2.5).
- Derived-field integrity (contract §1.2) can be *validated* in a test but the
  panel just renders whatever is authored.

Proto: new `message ConceptLesson { string kc = 1; string title = 2; string
section = 3; string overview = 4; repeated string key_concepts = 5; string
prerequisite_reminder = 6; string worked_example = 7; string common_misconception
= 8; string first_retrieval_prompt = 9; repeated string related_kcs = 10;
bool exists = 11; }` and RPC `rpc GetConceptLesson(GetConceptLessonRequest) returns
(ConceptLesson);` with `GetConceptLessonRequest { string kc = 1; }`. When no
lesson (or gated), return `exists=false`.

> Decision — parser location: **Rust** (consistent with cards, testable, reusable
> by Android) vs a **Python-only** parser in `reviewer.py` reading the md files
> directly (faster to ship, desktop-only). Recommend **Rust** to match the
> contract's "resolver keyed by KC id" and avoid a second content path.

### 2.2 Reviewer lesson panel (mirrors the concept sidebar)

`reviewer.py`:

- `revHtml()`: inject `<div id="_concept_lesson_panel" hidden></div>` next to
  `#_concept_graph_sidebar`; add `window._renderLessonPanel(payload)` /
  `window._hideLessonPanel()` JS modeled on `_renderConceptGraphSidebar` /
  `_hideConceptGraphSidebar`. Panel renders the 7 sections with a header and a
  primary **"Start retrieval ▶"** button (`pycmd("lessonStart:<KC>")`) + a
  secondary "Close".
- `_lesson_payload(kc)`: call `col._backend.get_concept_lesson(...)`, return a dict
  (or `{exists:false}`), like `_concept_graph_payload`.
- `_open_lesson_for_kc(kc)` and `_open_lesson_for_current_card()` (resolves KC via
  `_concept_labels(self.card)[:1]`).
- Styles: add a `#_concept_lesson_panel` block to `ts/reviewer/reviewer.scss`
  modeled on the sidebar (scrollable, readable prose column).

### 2.3 Pick → lesson → cards wiring

- Change `_start_concept_topic(topic)`: after `set_concept_selected_topic`
  succeeds, if `concept_lesson_exists(topic)` → `_open_lesson_for_kc(topic)`
  (show the lesson) **instead of** just a tooltip. The panel's "Start retrieval"
  (`lessonStart:<KC>`) → hide panel, set the encoded flag (2.4), then `mw.reset()`
  so the queue rebuilds with `selected_topic` and serves T's cards.
- If no lesson → current behavior (tooltip + proceed), i.e. graceful skip
  (`lesson-contract.md` §2.2).

### 2.4 First-encounter fallback + encoded flag

- Encoded flag: per-`(profile, KC)` local boolean via `mw.pm` profile meta (NOT the
  synced deck-config concept state) — `lesson-contract.md` §2.3.
  Helpers: `_kc_encoded(kc) -> bool`, `_mark_kc_encoded(kc)`.
- In `_showQuestion(card)`: after `_update_concept_graph_sidebar(card)`, if the
  card's KC is unencoded AND a lesson exists AND concept scheduler is enabled →
  open the lesson panel before reveal (once). Non-blocking for already-encoded KCs
  and for review cards. `(verify)` keep this behind a setting so it can't surprise
  during heavy review.

### 2.5 Display gate (decision — important for "every topic shows a lesson")

`lesson-contract.md` §4.2 says render iff `source == authored && review_status ==
approved`. But 169/172 lessons are `authored` + `needs_review`, so under that rule
almost nothing shows. The user wants a lesson for every new topic.

- **Recommended:** relax the resolver gate to **`source == authored`** (any
  `review_status`) so all human-authored lessons render now; keep `ai_generated`
  gated (blocked until Track H + eval). Rationale: the gate's real purpose is
  blocking unreviewed *AI* text; these are human-authored. Note the divergence
  from the contract and revisit when a review pass happens.
- Alternatives: (b) bulk-flip authored lessons to `approved`; (c) a config/dev
  toggle "show unreviewed authored lessons" defaulting on for now.

### 2.6 Post-answer "Lesson" button (from `lessons.md` entry (b))

- Add a `Lesson` control shown after `_showAnswer` (next to `Progress` in the
  bottom bar, or in the ease-button row), `pycmd("lesson")` →
  `_open_lesson_for_current_card()`. Empty/gated → "No lesson for this concept yet".

### 2.7 Tests

- Rust: `parses_all_lessons` (172 parse, ids exist in the unified map);
  `lesson_resolver_gates_ai_generated`; `demo_kc_lessons_are_approved`.
- Rust: `concept_lesson_exists` true for a known KC, false for a bare/unknown id.
- Python (optional smoke): `_lesson_payload` returns 7 sections for `Bio::DNA`.

---

## 3. Decisions to confirm before coding

1. **Gate (2.5):** render all `authored` lessons now (recommended) vs approved-only.
2. **Parser (2.1):** Rust resolver + RPC (recommended) vs Python-only md parse.
3. **Current section (1.1):** the active/selected-topic section, with
   most-worked-section then cold-start-breadth fallbacks (recommended).
4. **Recommendation badge (1.3):** star overlay on green nodes (recommended) vs a
   separate "Suggested next" list panel.
5. **First-encounter auto-lesson (2.4):** ship now behind a setting, or defer and
   only do the picker-driven (2.3) lesson-first for v1.

## 4. Milestones (each builds + tests green independently)

- **M1 — Recommendation policy (Part 1):** `recommended_topics` 2+1+1 + proto
  `recommended` flag + graph badge + Rust tests. Visible immediately in the graph.
- **M2 — Lesson resolver (2.1):** parser + `ConceptLesson` proto + `GetConceptLesson`
  RPC + gate + Rust tests. No UI yet.
- **M3 — Lesson panel + pick→lesson→cards (2.2, 2.3, 2.6):** reviewer panel,
  `_start_concept_topic` opens the lesson, "Start retrieval" proceeds, post-answer
  Lesson button.
- **M4 — First-encounter (2.4):** encoded flag + `_showQuestion` gate (optional /
  behind a setting).

## 5. Files touched

- `proto/anki/scheduler.proto` — `ConceptGraphNode.recommended`, `ConceptLesson`,
  `GetConceptLesson` RPC (+ request).
- `rslib/src/scheduler/concept_demo.rs` — `recommended_topics` rewrite, section
  helpers, node `recommended`; lesson resolver + parser (or new
  `concept_lessons.rs`), `import`/`include_str!` of `lessons-*.md`.
- `rslib/src/scheduler/service/mod.rs` — `get_concept_lesson` handler.
- `qt/aqt/reviewer.py` — lesson panel markup/JS, `_lesson_payload`,
  `_open_lesson_for_*`, `_start_concept_topic` change, `_showQuestion` gate,
  encoded-flag helpers, `lesson`/`lessonStart:` link branches, graph badge payload.
- `ts/reviewer/reviewer.scss` — lesson panel styles, recommended-node badge.
- Docs: update `progress.md`, tick `topic-picker-design.md` / `lesson-contract.md`
  acceptance boxes as milestones land.

## 6. Out of scope (now)

- AI-generated lesson text (Track H gate).
- The full explainable deck-options picker card (`topic-picker-design.md` §1.5).
- Lesson content edits / human review pass (separate from wiring).
