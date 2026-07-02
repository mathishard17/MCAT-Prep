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
