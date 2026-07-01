# MCAT Prep Workspace

Parent workspace for the MCAT prep desktop, Android, and backend projects.

## Repository Layout

```text
MCAT-Prep/
  anki/                 Desktop app and upstream backend code pulled from Anki.
  Anki-Android/         Android app pulled from AnkiDroid.
  Anki-Android-Backend/ Backend bridge used to connect Anki backend code to Android.
  scripts/              Workspace helper scripts.
```

## Folder Roles

`anki/` contains the desktop application code and backend logic brought in from
Anki. This is the source for the desktop experience and the upstream backend
behavior it depends on.

`Anki-Android/` contains the Android application code brought in from
AnkiDroid. This is the mobile client users run on Android devices.

`Anki-Android-Backend/` contains the backend bridge code used to connect the
Anki backend pieces to the Android app. It is based on the backend work pulled
from AnkiDroid.

The parent workspace ties these pieces together so the desktop/backend code,
Android app, and Android backend can be developed as one repository.

## Local Commands

Use `just --list` from this folder after the three child repos are in place.

