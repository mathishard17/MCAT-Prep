# Mistakes Log

Use this file to record project-specific mistakes and the rule that prevents repeating them.

## 2026-07-01 - Coverage Was An Additive Nudge, Not A Ceiling

- Mistake: Readiness subtracted a small additive coverage penalty (`(1 - coverage) * 4`), so a section with only 50% coverage could still show ~130 and 90% coverage could show ~131.6. Coverage did not actually cap the score, and performance ignored coverage entirely, so scores approached 132 without covering the blueprint.
- Cause: Coverage and mastery were modeled as fixed-size point penalties on the center instead of as a limit on how much of the section had been demonstrated. A 14-point scale with a 4-point max penalty cannot express "you have not been tested on 10% of this section."
- Fix: Readiness is now a coverage-weighted blend toward a guessing baseline: `clamp(120 + coverage * (performance - 120), 118, 132)`. Coverage is a hard ceiling (`max = 120 + coverage * 12`). Performance stays a conditional ability estimate on tested content. The mastery penalty was dropped from the center; mastery now only widens the uncertainty band. Also fixed the mastery-SE term, which was scaling by `1 - coverage` instead of `1 - section_mastery`.
- Next time: Model "how much have you demonstrated" as a ceiling/coverage weight, not a small additive penalty, when the metric has a bounded scale. Keep performance (conditional) and readiness (whole-domain projection) as distinct, clearly-labeled quantities.

## 2026-07-01 - Coverage Counted From The Wrong Source

- Mistake: IRT coverage was first computed from section-tagged IRT responses only, so overlapping biology evidence did not contribute to the small Bio slice in Chem/Phys or Psych/Soc.
- Cause: I mixed up performance evidence, which should come from section-tagged items, with blueprint coverage, which should come from KC evidence by discipline.
- Fix: Coverage now comes from KC evidence by discipline and is capped by each MCAT section's discipline weights.
- Next time: Keep performance and coverage separate. Performance is section-specific IRT evidence; coverage is blueprint-topic evidence.

## 2026-07-01 - Stale Persisted State Hid New Coverage Buckets

- Mistake: Existing persisted IRT states had answered counts but no `discipline_counts`, causing coverage to show as `0%`.
- Cause: Reconstructed revlog state was not merged when answered counts were equal, even if the reconstructed state had newer fields.
- Fix: Merge now backfills reconstructed IRT state when existing discipline coverage buckets are empty.
- Next time: When adding persisted fields, handle migration/backfill for existing state, not just fresh state.

## 2026-07-01 - Add Cards Metadata Used Tags Widget Too Early

- Mistake: Adding a card crashed with `AttributeError: 'Editor' object has no attribute 'tags'`.
- Cause: Concept metadata tags were applied before the editor's Qt tag widget was guaranteed to exist.
- Fix: Guard tag-widget updates with `hasattr(self, "tags")`, and apply concept metadata right before adding the note.
- Next time: When extending legacy Qt editor internals, do not assume optional UI widgets exist in every lifecycle callback.

## 2026-07-01 - Moved Checkout Broke The Build/Test Env

- Mistake: After moving the checkout under `MCAT-Prep/anki` (and flattening away the nested `anki/.git`), `just run`/`just test-py` failed in several ways: `import anki` -> `cannot import name '_rsbridge'`; then `ninja: '.git' ... missing`; then `protoc-gen-mypy: program not found`.
- Cause: `out/` and `out/pyenv` bake in the original absolute path `/Users/sophiaz/alphaai/anki`, and the flatten removed the local `.git` the build referenced. Specifically:
  1. `out/pylib/anki/_rsbridge.so` symlinked to the old `out/rust/debug/librsbridge.dylib`.
  2. `out/pyenv` editable `.pth` files pointed at the old `pylib`/`qt`.
  3. `out/build.ninja` `builddir` was the old `out`.
  4. `build/ninja_gen/src/configure.rs` always added `.git` as a build.ninja regen input, but there is no local `.git` in a flattened monorepo.
  5. 45 console scripts in `out/pyenv/bin` had `#!` shebangs pointing at the old venv interpreter.
- Fix: Repointed the `_rsbridge.so` symlink and the `.pth` files to the current path (a `uv sync` during the build also re-registers the editable installs), fixed `build.ninja` `builddir`, guarded the `.git` input in `configure.rs` with an existence check, and rewrote the 45 stale shebangs. `just build` then succeeded.
- Next time: After moving/copying an Anki checkout, assume `out/` and the venv are location-bound. Prefer a clean rebuild; if repairing in place, fix symlinks, `.pth`, `build.ninja` `builddir`, and `out/pyenv/bin` shebangs, and do not depend on a local `.git` that a flattened monorepo does not have.

## 2026-07-02 - `SYNC_ENDPOINT` Is Legacy, Not Read By The Current Client

- Mistake: The Track G plan (and initial task framing) assumed the desktop app points at a custom sync server via a `SYNC_ENDPOINT` env var (`SYNC_ENDPOINT=... just run`). It does not — searching the tree, `SYNC_ENDPOINT` only appears in `docs-site/manual/sync-server.mdx` (describing *older* clients) and the plan doc.
- Cause: `SYNC_ENDPOINT`/`SYNC_ENDPOINT_MEDIA` were how pre-2.1.57-era clients were pointed at a server. The current client resolves the endpoint from the profile: `Collection.sync_endpoint()` in `qt/aqt/profiles.py` returns `_current_sync_url() or custom_sync_url()`, where `custom_sync_url()` reads the `customSyncUrl` profile key set in Preferences → Syncing → "Self-hosted sync server". No env override exists.
- Fix: Documented the real mechanism (Preferences base URL, trailing slash, e.g. `http://127.0.0.1:8080/`) in the plan and progress docs and in `scripts/run-local-sync-server.sh` output; the script prints the base URL to paste into Preferences and explicitly flags `SYNC_ENDPOINT` as legacy/ignored. Did not add an env override (kept changes to script + docs only).
- Next time: To point the desktop client at a local/custom server, set the Preferences self-hosted URL (`customSyncUrl`), not `SYNC_ENDPOINT`. The client is handed a *base* URL and appends `sync/`/`msync/` itself, so the configured value must end in `/`. AnkiDroid is analogous: SharedPreference `syncBaseUrl` (Settings → Sync → Custom sync server).

## 2026-07-03 - AnkiDroid JS→Kotlin Bridge Schemes Must Be Lowercase (and have a colon)

- Mistake: Tapping the MCAT quiz's rating (Again/Hard/Good/Easy) and "Ask AI" buttons on Android showed "no app installed that can perform this action" (ActivityNotFoundException). A first fix — adding a colon to `mcatChatOpen` → `mcatChatOpen:` — did NOT help, and the grade buttons (which already had colons) failed too.
- Cause: The Android reviewer bridge is `window.location.href = "<scheme>:<value>"`, intercepted in `AbstractFlashcardViewer.filterUrl`. The Chromium WebView **lowercases URL schemes**, so `mcatChoice:0` / `mcatGrade:3` / `mcatChatOpen:` reach `filterUrl` as `mcatchoice:0` / `mcatgrade:3` / `mcatchatopen:`. The handlers matched camelCase (`url.startsWith("mcatChoice:")`), so they never matched and fell through to `filterUrl`'s catch-all, which does `startActivity(Intent(ACTION_VIEW, url))` → no handler → "activity_start_failed" snackbar. (A scheme with no colon at all is separately broken — it becomes a relative URL — but the deeper issue was case.)
- Fix: use lowercase schemes on BOTH sides — JS `signal("mcatchoice:"…)` / `mcatgrade:` / `mcatchatopen:open` / `mcatchatsend:`, and `filterUrl` `url.startsWith("mcatchoice:")` etc. Every built-in AnkiDroid scheme (`signal:`, `playsound:`, `state-mutation-error:`, …) is lowercase for exactly this reason.
- Next time: an AnkiDroid `window.location.href` bridge command must be a real URI scheme that is **lowercase** and has a colon (`name:value`). Desktop `pycmd` strings are exact and case-preserving, so bridge names can't be copied 1:1 from desktop to Android — lowercase them.

## 2026-07-03 - Android Backend Build Failed On JDK 20 (Needs 17/21/25)

- Mistake: `Anki-Android-Backend/build.sh` completed the Rust cross-compile + web-artifact steps, then failed at the Gradle stage: "ERROR: Anki-Android-Backend builds with JVM version 17, 21 and 25. Incompatible major version detected: '20'." The default `java` on PATH was OpenJDK 20.
- Cause: The backend's Gradle build rejects JDK 20; the machine's default JDK (`/usr/libexec/java_home`) was 20, and `build.sh` doesn't pin a JDK.
- Fix: Re-ran with `JAVA_HOME=/opt/homebrew/opt/openjdk@21/libexec/openjdk.jdk/Contents/Home` (and `ANDROID_HOME=~/Library/Android/sdk`) on PATH → BUILD SUCCESSFUL (the Rust/web steps were cached, so the rerun was mostly the Gradle stage). Same JDK 21 works for `./gradlew :AnkiDroid:compilePlayDebugKotlin`.
- Next time: For `Anki-Android-Backend/build.sh` and AnkiDroid gradle, export `JAVA_HOME` to an installed JDK 17/21/25 first (JDK 20 is rejected). Homebrew `openjdk@21` is available at `/opt/homebrew/opt/openjdk@21/libexec/openjdk.jdk/Contents/Home`.

## 2026-07-03 - FSRS Enable Is A String-Key Config, Not A `Config.Bool`

- Mistake: Assumed FSRS could be toggled via `Config.Bool.FSRS`. The proto `Config.Bool` enum has no plain `FSRS` (only `FSRS_SHORT_TERM_WITH_STEPS_ENABLED` and `FSRS_LEGACY_EVALUATE`).
- Cause: In rslib the global FSRS toggle is `BoolKey::Fsrs`, which serializes camelCase to the collection-config string key `"fsrs"` — it is not exposed as a typed proto bool.
- Fix: Enable via the generic JSON config: desktop `col.set_config("fsrs", True)`, Android `col.config.set("fsrs", true)`. Guarded both with a one-time `mcatFsrsDefaulted` config flag so a later manual toggle-off sticks.
- Next time: For collection-wide FSRS on/off use the string config key `"fsrs"` (get/set_config), not `Config.Bool`.

## 2026-07-03 - Author `display` Overrode The `hidden` Attribute (Invisible Click-Blocking Overlay)

- Mistake: After adding the lesson panel, the reviewer's whole page "got blacker" and nothing was clickable. Cause: `#_concept_lesson_panel { display: flex; position: fixed; inset: 0; background: rgba(0 0 0 / .45); z-index: 100 }` in `ts/reviewer/reviewer.scss`. An author `display: flex` **beats** the UA `[hidden] { display: none }`, so the element with the `hidden` attribute was still rendered — a transparent full-screen overlay that darkened everything and swallowed every pointer event.
- Fix: Add `#_concept_lesson_panel[hidden] { display: none; }` (higher specificity: id+attr) so the panel is truly hidden when closed. Toggling `el.hidden = true/false` now works as intended.
- Next time: Any element toggled via the `hidden` attribute must NOT be given an unconditional author `display` in CSS. Either gate it with `:not([hidden])`, add an explicit `[hidden]{display:none}` rule, or toggle a class instead of `hidden`. A full-viewport `position:fixed` overlay that's accidentally visible will silently block all clicks even if nearly transparent.

## 2026-07-03 - A JS `\n` In A Non-Raw Python String Crashed The Whole Injected Script ("no buttons, can't click")

- Mistake: `reviewer.py`'s `revHtml()` embeds a large JS IIFE inside a **non-raw** `"""..."""`. The mermaid parser had `source.split("\n")` and a regex ending `...(BT)\b/i`. Python processed `\n` → a real newline (illegal inside a JS `"..."` string literal → `SyntaxError: Invalid or unexpected token`) and `\b` → a backspace char (broke the word-boundary regex). The SyntaxError aborts the **entire** IIFE, so `_renderConceptGraphSidebar`, `_renderMcatQuiz`, `_hideConceptGraphSidebar`, etc. were never defined — the quiz/answer buttons never render and nothing is clickable.
- Why it hid: `py_compile` only checks *Python* syntax, and `node --check` on the raw `reviewer.py` text also passes (raw `\n`/`\b` are valid JS escapes). The bug only exists in the **Python-processed** served string. The Anki log is the tell: `JS error .../legacyPageData:NNN Uncaught SyntaxError: Invalid or unexpected token` followed by a cascade of `_render… is not defined`.
- Fix: double-escape JS escapes that are *also* valid Python escapes inside `revHtml` — `\n`→`\\n`, `\b`→`\\b` (also `\t \r \f \v \0 \x`). To verify like the browser sees it: `eval()` the `revHtml` triple-quoted literal in Python, extract each `<script>`, and `node --check` the result (see `/tmp/served_js.py`) — raw `node --check` will NOT reproduce it.
- Next time: JS in a non-raw Python string must `\\`-escape `\n \r \t \b \f \v`. Note `\s \d \w \. \|` currently survive only because they're *invalid* Python escapes (kept, with a `SyntaxWarning`) — deprecated and will eventually become errors, so prefer `\\` for all regex metaescapes, or keep large JS in a raw string / separate served file.

## 2026-07-03 - CSS `%` In A `%`-Formatted Python String (`_bottomHTML`) Crashed The Reviewer

- Mistake: Added a CSS `@keyframes lessonPulse { 0%, 100% {...} 50% {...} }` to `reviewer.py`'s `_bottomHTML`, which returns `""" ... """ % dict(edit=..., time=..., ...)`. Python's `%` operator read `0%,`/`100%`/`50%` as format specifiers → `ValueError: unsupported format character ',' at index N`. `_bottomHTML` raised, so the bottom bar (and the whole reviewer page) never rendered — cascading into `Uncaught ReferenceError: _showQuestion is not defined` and a downstream `'utf-8' codec can't encode ... surrogates not allowed`.
- Why it hid: `py_compile` passes (valid Python source); the error is only raised at runtime when `%` formatting runs. `revHtml()` is safe because it uses `.replace(...)`, NOT `%` — so the same CSS is fine there but not in `_bottomHTML`.
- Fix: escape every literal `%` as `%%` in `_bottomHTML` (e.g. `0%%, 100%%`, `50%%`, and note the pre-existing `width=100%%`). Verify by extracting the template and running `tmpl % dict(editkey=..., edit=..., morekey=..., more=..., downArrow=..., time=0)` → expect success, not `ValueError`.
- Next time: `_bottomHTML` is `%`-formatted — any added CSS/JS `%` (keyframes, `width: 100%`, etc.) must be `%%`. `revHtml`/`_bottomHTML` differ: revHtml uses `.replace` (no `%%` needed), `_bottomHTML` uses `%` (needs `%%`).

## 2026-07-03 - Readiness Metric "Stuck At 0" (Multiplying By An Often-Zero Factor, Then Averaging Over All KCs)

- Mistake: The sidebar computed a section's readiness as `mean over ALL section KCs of (accuracy × coverage × memory)`. Two independent bugs each forced it toward 0: (1) it **multiplied by `memory`**, which is `0` for any KC whose cards haven't been studied / has no FSRS memory estimate, so a single zero factor zeroed the whole term; and (2) it **divided by every KC in the section** (~34 for Bio/Biochem), the vast majority untouched and contributing 0, so even a few well-answered KCs averaged to ~0%. Net: it displayed `readiness 0%` almost always.
- Why it hid: each factor is individually "reasonable" (accuracy, coverage, memory are all real signals), and with a fully-studied deck it would eventually be non-zero — so it looked fine in reasoning but collapsed on the realistic sparse-data path a learner actually sees. There was also a **second, correct** readiness (the backend IRT scaled score) that the sidebar simply ignored, so the visible number and the source-of-truth number diverged.
- Fix: make the backend the single source of truth and express readiness as a **shrink-toward-prior score estimate** that never hits 0: `clamp(125_prior + coverage × (demonstrated − 125_prior), 118, 132)` with `demonstrated = 118 + section_accuracy × 14`. Prior (population median) with no data, moves toward demonstrated accuracy as coverage grows. Also added a projected total = sum of the four section centers.
- Next time: a displayed "score/estimate" should degrade to a **sensible prior** (population median), never to 0, when evidence is thin. Avoid `metric = a × b × c` for a headline number when any factor is frequently 0 (one zero nukes it), and don't **average a per-item quantity over the whole population including untouched items** (that's a coverage penalty in disguise, not the metric). Keep one source of truth: don't recompute a metric client-side when the backend already produces the authoritative version.

## 2026-07-03 - A Cluster Of "Metric Measured Wrong" Scheduler Bugs (memory@t≈0, coverage-by-volume, unreachable discipline, per-KC counting)

- Mistake: Several Concept Scheduler metrics were computed against the wrong quantity, so the numbers looked plausible but were wrong on the realistic path:
  1. **Memory at elapsed≈0.** `card_memory` asked FSRS for retrievability at `now - last_review`, but `last_review` is set to *now* on every rating, so elapsed≈0 and retrievability ≈ 1.0 for **any** button — pressing "Again" repeatedly appeared to RAISE memory.
  2. **Coverage by volume.** `section_coverage` summed each KC's `answered` count, so 13 "Again"s on ONE Bio KC maxed the 65% biology slice without touching any other concept.
  3. **Unreachable discipline.** Every `PsychSoc::` KC mapped to `Psychology`, so the 30% `Sociology` slice could never fill and Psych/Soc coverage was structurally capped ~70%.
  4. **Per-KC counting.** `total_seen_cards` (which gates readiness) incremented inside `record_evidence`, called once per target KC — a multi-KC card counted 2–3×.
- Why they hid: each was fine in the happy demo path (single-KC cards answered "Good" once), and unit tests encoded those exact happy values (e.g. the memory test asserted high recall right after Good — consistent with the t≈0 bug). They only broke under real usage (repeated Again, grinding one topic, multi-KC cards, sociology content).
- Fixes: evaluate memory at a forward horizon (`elapsed.max(MEMORY_HORIZON_SECS)`) on the FSRS path only (the fallback's base is already rating-dependent); count coverage as *distinct KCs with evidence* capped by KCs-that-exist; enumerate the sociology KCs (`SOCIOLOGY_PSYCHSOC_KCS`) so `for_component` can split psych/soc; move the card counter into `note_card_answered()` called once per card. Added regression tests for each (`coverage_rewards_breadth_not_grinding_one_kc`, `psychsoc_kcs_split_into_psychology_and_sociology`, `total_seen_counts_cards_not_kcs`).
- Next time: for any behavioral metric, write down "what real quantity is this measuring, and at what time / over what set?" A metric should move in the intended direction under the *adversarial* path (all-Again, grind-one, multi-tag), not just the demo path — and a test that only pins the happy value will happily lock in the bug.
