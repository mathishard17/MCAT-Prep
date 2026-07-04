# MCAT Prep Workspace

Study tooling for the **MCAT** (Medical College Admission Test), the AAMC's
standardized exam for U.S. and Canadian medical-school admissions. The MCAT is
reported on a total scaled score of **472–528** (midpoint 500), combining four
sections that are each scored **118–132** (midpoint 125):

- Biological and Biochemical Foundations of Living Systems (Bio/Biochem)
- Chemical and Physical Foundations of Biological Systems (Chem/Phys)
- Psychological, Social, and Biological Foundations of Behavior (Psych/Soc)
- Critical Analysis and Reasoning Skills (CARS)

This is the parent workspace for the MCAT prep desktop, Android, and backend
projects.

## Repository Layout

```text
MCAT-Prep/
  anki/                 Desktop app and upstream backend code pulled from Anki.
  Anki-Android/         Android app pulled from AnkiDroid.
  Anki-Android-Backend/ Backend bridge used to connect Anki backend code to Android.
  scripts/              Workspace helper scripts.
```

Feature progress is tracked in `anki/added features/progress.md`.

## Mobile App

`Anki-Android/` is the Android client (pulled from AnkiDroid), and
`Anki-Android-Backend/` is the backend bridge that connects the Anki backend
code to the Android app.

### Features

- Adds an Android Concept Scheduler status view for each deck, opened from the
deck picker or reviewer, showing backend-provided progress, evidence counters,
session budget, topic states, and MCAT section readiness.
- Adds a reviewer `Progress` shortcut and non-blocking bottom sheet so students
can inspect Concept Scheduler progress without leaving review.
- Shows KC badges on tagged review cards, such as `DNA · Bio/Biochem`, while
untagged cards keep the normal review experience.
- Adds demo Add Card metadata controls for target KC, prerequisite KC, MCAT
section, difficulty, and optional IRT values.
- Adds a mobile knowledge lattice with mastered, in-progress, next-up, and locked
topics plus tap-to-view topic details.
- Current caveat: the Android UI reads scheduler state from backend RPCs and does
not compute or write scheduler state itself yet.

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

`anki/` is the desktop application and upstream backend code pulled from Anki.

### Features

- Adds a desktop-first Concept Scheduler Mode for MCAT decks using `KC::`,
`Prereq::`, `MCAT::`, and `Difficulty::` tags to track topic mastery and
recommend what concept to study next.
- Updates the scheduler so answered tagged cards adjust mastery, evidence
counters, prerequisite-violation counts, IRT section state, and concept-aware
new-card ordering while normal reviews still use Anki scheduling.
- Adds Deck Options and reviewer progress views with KC badges, topic graph
status, section coverage, readiness, and score ranges once enough evidence is
available.
- Adds local MCAT demo deck/import support plus Add Cards metadata controls for
KC topics, MCAT sections, difficulty, discrimination, and guessing tags.
- Current caveats: no reviewer topic picker yet, no Android sync companion yet,
and full checks are still blocked by a known unrelated Qt installer test issue.

### How to Run

```bash
just desktop        # run the desktop app
just desktop-check  # run desktop checks
```

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

## Original Repositories

- `anki/`: [https://github.com/ankitects/anki.git](https://github.com/ankitects/anki.git)
- `Anki-Android/`: [https://github.com/ankidroid/Anki-Android.git](https://github.com/ankidroid/Anki-Android.git)
- `Anki-Android-Backend/`: [https://github.com/ankidroid/Anki-Android-Backend.git](https://github.com/ankidroid/Anki-Android-Backend.git)

## Local Commands

Run `just --list` from this folder to see all workspace commands.