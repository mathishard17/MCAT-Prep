# Mistakes Log

Use this file to record project-specific mistakes and the rule that prevents repeating them.

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
