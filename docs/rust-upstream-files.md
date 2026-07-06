# Upstream files touched + future-merge difficulty

**Rubric §7a deliverable.** Which files from upstream **Anki**
(`github.com/ankitects/anki`, vendored in `anki/`) and **AnkiDroid**
(`github.com/ankidroid/Anki-Android`, vendored in `Anki-Android/`) the Concept
Scheduler modifies, versus brand-new files it adds, and how hard a future
upstream merge would be for each.

**Guiding principle:** we kept almost all logic in **new files** (which merge
cleanly — no upstream counterpart) and touched existing upstream files only
with **small, localized hooks**. The one exception is the desktop/Android
reviewer, where the dashboard + KC surfacing needed heavier edits.

Merge-difficulty legend: **Trivial** (a few added lines, unlikely to conflict) ·
**Easy** (localized block, self-contained) · **Medium** (interleaved with
upstream logic) · **Hard** (large, in a file upstream changes often).

---

## New files (additive — merge trivially, no upstream counterpart)

| Layer | New file(s) | Role |
| --- | --- | --- |
| Rust engine | `rslib/src/scheduler/concept.rs` (2172 loc), `concept_demo.rs` (1646 loc) | Whole model: tags, graph, mastery, IRT, coverage, readiness, RPC payload |
| Desktop (Qt/Py) | `qt/aqt/mcat_ui.py` (1038), `mcat_ai.py`, `mcat_ai_core.py`, `concept_tags.py`, `cars_practice.py` | Readiness dashboard + AI tagging/rewording + CARS practice |
| Desktop (TS) | `ts/routes/deck-options/ConceptSchedulerOptions.svelte` (1419) | Deck-options "Concept" tab with the three-score snapshot |
| Android | `AnkiDroid/.../conceptscheduler/*` (ConceptSchedulerStatusScreen, McatHonesty, ReadinessGauge, ConceptLatticeGraph, KcBadgeController, McatTheme, AddCardConceptMetadata, ConceptLesson*), `ConceptSchedulerStatusFragment.kt`, `mcat/McatMultipleChoice.kt`, `mcat/OpenAIClient.kt`, `preferences/McatAiSettingsFragment.kt` | Android dashboard, honesty tiles, KC badges, MC quiz, AI settings |
| Tests | `pylib/tests/test_concept_scheduler_engine.py`, `pylib/tests/test_concept_add_cards.py`, `AnkiDroid/.../conceptscheduler/McatHonestyTest.kt` | Python-from-backend + Android honesty tests |

New files carry the only real risk of *silent* breakage: if upstream renames a
trait/function they call (e.g. `Collection` methods, the `fsrs` crate API). They
never cause a textual merge conflict.

---

## Modified upstream files

### Rust engine (`anki/rslib/src`)

| File | Concept-marker lines | Change | Merge difficulty |
| --- | ---: | --- | --- |
| `scheduler/mod.rs` | 2 | `pub(crate) mod concept; mod concept_demo;` | Trivial |
| `scheduler/answering/mod.rs` | 27 | One hook after answering + `enable_concept_scheduler` test helper + undo test | Easy |
| `scheduler/queue/mod.rs` | 77 | Session budget (`ConceptSessionState`) + presentation gate | Medium |
| `scheduler/queue/builder/mod.rs` | 40 | Wire readiness sort into `build`; ablation test | Medium |
| `scheduler/queue/builder/sorting.rs` | 18 | `sort_new_by_concept_readiness` | Easy |
| `scheduler/queue/builder/gathering.rs` | 1 | Call `prepare_concept_new_card_sort` | Trivial |
| `scheduler/queue/undo.rs` | 7 | Restore concept session on undo | Easy |
| `scheduler/service/mod.rs` | 24 | 5 new RPC handlers | Easy |
| `config/deck.rs` | 1 | Per-deck state config key | Trivial |
| `decks/schema11.rs` | 5 | (de)serialize `conceptSchedulerEnabled` | Trivial |
| `deckconfig/update.rs` | 2 | Persist the enable flag | Trivial |

### Protobuf (`anki/proto/anki`)

| File | Change | Merge difficulty |
| --- | --- | --- |
| `scheduler.proto` | +5 RPCs, +~16 messages (append-only at end of file) | Medium (shares one `service SchedulerService {}` block upstream also edits) |
| `decks.proto` | `bool concept_scheduler_enabled = 11;` on `Deck.Normal` | Trivial (new field number) |
| `deck_config.proto` | `bool concept_scheduler_enabled = 8;` on `CurrentDeck.Limits` | Trivial (new field number) |

### Desktop (`anki/qt/aqt`, `anki/ts`)

| File | Change | Merge difficulty |
| --- | --- | --- |
| `qt/aqt/reviewer.py` | Progress sidebar, KC badges, concept graph payload, Ask-AI | **Hard** (large edit to a hot upstream file) |
| `qt/aqt/deckbrowser.py` | Fetch status + render dashboard | Easy |
| `qt/aqt/editor.py` | Concept metadata controls in Add Cards | Medium |
| `qt/aqt/addcards.py` | Route concept cards to the demo deck | Easy |
| `qt/aqt/mediasrv.py` | Expose concept RPCs to the web frontend | Easy |
| `qt/aqt/main.py`, `profiles.py` | AI opt-in key wiring | Easy |
| `ts/routes/deck-options/DeckOptionsPage.svelte` | Mount the Concept tab | Easy |
| `ts/reviewer/reviewer.scss` | Progress/badge styles | Easy |

### Android (`Anki-Android/AnkiDroid`)

| File | Change | Merge difficulty |
| --- | --- | --- |
| `.../AbstractFlashcardViewer.kt`, `Reviewer.kt` | Progress shortcut + bottom sheet + KC badge in review | **Hard** (hot upstream files) |
| `.../DeckPicker.kt`, `dialogs/DeckPickerContextMenu.kt` | Open the dashboard from the deck list | Medium |
| `.../NoteEditorFragment.kt` | Add-card KC metadata | Medium |
| `.../preferences/{HeaderFragment,Preferences}.kt`, `settings/Prefs.kt` | MCAT AI settings screen | Easy |
| `.../pages/PostRequestHandler.kt` | Route concept RPC POSTs | Easy |
| `res/menu/reviewer.xml`, `res/layout/{include_reviewer_topbar,fragment_note_editor}.xml`, `res/xml/preference_headers.xml`, `res/values/{preferences,10-preferences,12-dont-translate,01-core}.xml` | Menu/layout/strings for the above | Easy (localized) |

> The many `res/values-*/01-core.xml` locale files that mention "concept"/"mcat"
> are auto-managed translation mirrors of `values/01-core.xml`; they are not
> hand-edited and are regenerated by AnkiDroid's i18n tooling.

---

## Overall future-merge assessment

- **~70% of the code is in new files** → no textual conflicts on upstream merge.
- **Engine hooks are tiny** (1–2 lines in `mod.rs`, `config/deck.rs`,
  `gathering.rs`, the proto field numbers) → trivial to re-apply.
- **The queue builder + session budget** (`queue/mod.rs`,
  `queue/builder/*`) are the medium-risk engine spots; upstream occasionally
  refactors the queue, so a merge would need re-wiring the two call sites
  (`prepare_concept_new_card_sort`, `sort_new_by_concept_readiness`).
- **The reviewer** (desktop `reviewer.py` and Android
  `AbstractFlashcardViewer.kt`/`Reviewer.kt`) is the hardest merge surface,
  because it is a large, frequently-changed upstream file and we added a lot of
  UI there. Isolating more of that UI into the new files would reduce the risk.
- **`scheduler.proto`** is append-only for us, so conflicts would only be
  positional if upstream also appends RPCs.

## Regenerate this list (verifiable)

```sh
# every source file that references the concept scheduler / MCAT feature
cd /Users/sophiaz/alphaai/MCAT-Prep
rg -l -i 'concept|mcat' anki/rslib/src anki/qt/aqt anki/ts anki/proto \
    Anki-Android/AnkiDroid/src/main/java 2>/dev/null

# concept-marker footprint per Rust engine file (drives the tables above)
rg -c -i 'concept|mcat' anki/rslib/src
```
