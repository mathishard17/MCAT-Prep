# Add Cards Progress

## Issues
- when you successfully add, clear and hide the tags part
- what happens when a prereq is chosen and it's not in the graph yet? like what if the direction is wrong? maybe remove the prereq part and use only the manual prereq orderings put into the graph
- update the demo deck to have 200 cards
but before doing that must do it step by step by researching proper knowledge graph and directions and topics and prereqs. may need to updatr te UI of the graph to be movable and exapndable/shrinkable for different mmore major areas of bio/biochem/chem etc to maintain allllll the need knowledge components
- add learning sections before every new topic have a section overviewing important parts of that knowledge component




## Current Goal

Make Add Cards usable for Concept Scheduler demo cards without breaking normal Anki note creation.

The Add Cards workflow should support:

- Normal Front/Back card content entry.
- Required Concept Scheduler metadata:
  - at least one `KC::...` tag
  - at least one `Difficulty::1` through `Difficulty::5` tag
- Optional `Prereq::...` tags.
- Optional `MCAT::...` override tags.
- Automatic `MCAT::...` tags derived from selected KCs when the user does not manually override sections.
- Optional IRT metadata:
  - `IRT::Discrimination::...`
  - `IRT::Guessing::...`

## Implemented UI Changes

The Add Cards editor now has a Concept Scheduler tag panel in `qt/aqt/editor.py`.

The panel includes:

- Target KC checklist with all current MCAT topic KCs, including Bio, Biochem, GenChem, Physics, Orgo, Psych/Soc, and CARS.
- Prerequisite KC checklist using the same KC list.
- MCAT section checklist for manual overrides.
- Difficulty checklist.
- IRT discrimination and guessing controls.

The panel is now collapsed by default behind a `Show Concept Scheduler tags` button so it does not hide the normal card fields. This was changed after the UI made it look like there was no visible Front/Back field and Anki reported the first field was empty.

## Current Expected Add Cards Behavior

The user should first fill the normal card content fields, especially the first field such as `Front`.

Then they should add concept metadata either by:

- Expanding `Show Concept Scheduler tags` and selecting KC/Difficulty tags, or
- Typing tags manually in the regular tag editor.

The Add button stays clickable when there is a current note. On click:

- If KC or Difficulty metadata is missing, it shows `Choose target KC and difficulty tags first.`
- If the first note field is empty, Anki shows its normal empty-first-field warning.
- If required metadata and card fields are present, the note proceeds through the normal add path.

## Manual Tag Support

Manual tags are accepted when they include at least:

```text
KC::Biochem::Glycolysis
Difficulty::1
```

The parser also normalizes the Unicode double-colon character to ASCII `::`, so tags typed like this are accepted:

```text
KC∷Biochem∷Glycolysis
Difficulty∷1
```

The normalization helper is:

```text
normalize_concept_tag()
```

The requirement helper is:

```text
concept_tags_meet_add_requirements()
```

## MCAT Section Derivation

MCAT sections are auto-derived from KCs unless the user manually selects MCAT section overrides.

Current mapping:

- `Bio::*` -> `MCAT::Bio_Biochem`, `MCAT::Chem_Phys`, `MCAT::Psych_Soc`
- `Biochem::*` -> `MCAT::Bio_Biochem`, `MCAT::Chem_Phys`
- `GenChem::*` -> `MCAT::Chem_Phys`
- `Physics::*` -> `MCAT::Chem_Phys`
- `Orgo::*` -> `MCAT::Chem_Phys`
- `PsychSoc::*` -> `MCAT::Psych_Soc`
- `CARS::*` -> `MCAT::CARS`

The helper is:

```text
derived_mcat_sections_for_topics()
```

## Bugs Fixed

### Add Button Stayed Greyed Out

Problem:

- The Add button was gated by Concept Scheduler tag checks.
- Manual tag edits happened through the web tag editor.
- The web editor saved tags through the `saveTags` bridge command.
- That path updated `note.tags` but did not refresh Add button state.

Fix:

- `Editor.onBridgeCmd()` now calls `on_concept_metadata_changed()` after `saveTags` in Add mode.
- `AddCards.setupEditor()` wires `editor.on_concept_metadata_changed` to `_update_add_button_enabled()`.
- The Add button is no longer disabled solely because concept metadata is incomplete; missing metadata is validated on click instead.

### Concept Checklist Panel Was Not Mounted

Problem:

- The checklist panel was originally attached through `setupTags()`.
- `setupTags()` is not called in the active Add Cards editor path.
- Users only saw the web editor tag bar, not the new checklist UI.

Fix:

- Add mode now mounts the Concept Scheduler panel directly through `_setup_concept_metadata_panel()`.
- The panel is collapsed by default so the normal Front/Back fields remain visible.

### Unicode Tag Separators Broke Validation

Problem:

- Tags typed with the Unicode double-colon character were not recognized as structured tags.

Fix:

- Concept tag parsing normalizes that character to ASCII `::` before validation.

### MCAT Section Auto-Tagging Was Too Narrow

Problem:

- MCAT section derivation initially picked only the most obvious section.

Fix:

- KC-to-MCAT mapping now emits all plausible sections, while still allowing user overrides.

## Files Touched

- `qt/aqt/editor.py`
  - Adds Concept Scheduler tag options.
  - Adds pure helper functions for tag normalization, requirement checks, and MCAT derivation.
  - Adds the collapsible metadata panel.
  - Wires `saveTags` to refresh Add state.
  - Applies concept metadata tags before adding.

- `qt/aqt/addcards.py`
  - Wires editor concept metadata changes to Add button refresh.
  - Keeps Add clickable when a note exists.
  - Blocks add on click if KC/Difficulty tags are missing.
  - Uses `set_note()` on notetype changes so editor state syncs correctly.

## Verification So Far

Targeted checks passed:

```text
collapsed add panel checks passed
concept add-card checks passed
```

Full lint passed after the Add Cards fixes:

```text
just lint
```

Result:

```text
All checks passed.
Success: no issues found in 296 source files.
Build succeeded.
```

## Current User-Facing Instruction

After code changes, restart the running Anki process started by `just run`.

In Add Cards:

1. Fill the normal card field, especially `Front`.
2. Add at least one `KC::...` tag.
3. Add one `Difficulty::...` tag.
4. Leave MCAT empty for automatic derivation, or manually choose/enter `MCAT::...` overrides.
5. Click Add.

If Anki says the first field is empty, that is about the card content field, not the Concept Scheduler metadata.

