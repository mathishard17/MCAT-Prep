---
name: progress-readme-mistakes
description: Keeps project documentation current by updating progress.md, README.md, and a mistakes log after substantive implementation, verification, debugging, or UX changes. Use when working on this Anki Concept Scheduler project, when the user asks to update progress or README, or when an agent discovers/fixes a mistake.
---

# Progress, README, and Mistakes

Use this skill for the Anki Concept Scheduler project whenever work changes behavior, architecture, tests, UI, formulas, or project direction.

## Required Habits

After substantive work, check whether these files need updates:

- `added features/progress.md`: current implementation status, algorithm behavior, verification results, remaining work.
- `README.md`: stable project explanation, architecture, math formulas, commands, validated behavior.
- `.cursor/mistakes.md`: mistakes, false assumptions, bugs found after the fact, and how to avoid repeating them.

Do not edit these files for tiny wording-only changes unless the change affects project truth.

## Update `added features/progress.md`

Update `added features/progress.md` when:

- A feature is implemented, removed, renamed, or scoped differently.
- Tests pass/fail in a way future agents should know.
- Backend contracts, RPCs, DTOs, formulas, thresholds, or UI behavior change.
- A temporary limitation or known blocker appears.

Prefer concise bullets. Include exact commands and results when relevant:

```text
just test-rust: passed, 569 tests
just test: failed on unrelated qt installer template issue
```

## Update `README.md`

Update `README.md` when:

- The high-level project story changes.
- Math formulas change.
- Architecture or data flow changes.
- A new major component is added.
- User-facing behavior changes in a way a reader needs to understand.

Keep `README.md` more stable than `added features/progress.md`. It should explain the project, not narrate every small step.

## Collect Mistakes

When an agent makes or discovers a mistake, add an entry to `.cursor/mistakes.md`.

Record:

- Date.
- What went wrong.
- Why it happened.
- Fix.
- Rule for next time.

Template:

```markdown
## YYYY-MM-DD - Short Title

- Mistake:
- Cause:
- Fix:
- Next time:
```

Examples of mistakes worth recording:

- Misstated what a backend value meant.
- Displayed a score before the honesty gate was met.
- Counted coverage incorrectly.
- Let stale persisted state override reconstructed state.
- Forgot to update `added features/progress.md` or `README.md` after changing behavior.

## User Style Criteria

Prefer:

- Direct, plain explanations.
- Concrete file paths and commands.
- Clear distinction between implemented, blocked, and planned.
- Honest caveats instead of confident guesses.

Avoid:

- Over-polished marketing language.
- Hiding uncertainty.
- Saying a score or metric is meaningful before validation/coverage gates are met.
- Leaving docs stale after implementation changes.
