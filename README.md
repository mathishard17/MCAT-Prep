# MCAT Prep Workspace

Parent workspace for the MCAT desktop app, Android backend, and Android app.

Expected layout:

```text
MCAT-Prep/
  anki/
  Anki-Android-Backend/
  Anki-Android/
  scripts/
```

## Migration Status

The three child repositories now live under this parent folder as nested Git
repositories. Their uncommitted work is preserved inside each child repo.

Before committing this parent repo, decide whether it should track the children
as Git submodules or whether this should become a full monorepo import. For now,
do not flatten the child repos while they still have uncommitted work.

## Local Commands

Use `just --list` from this folder after the three child repos are in place.

