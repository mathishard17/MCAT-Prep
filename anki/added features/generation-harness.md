# Replayable Card/Problem Generation Harness (Track B)

Design-only specification for a repeatable, auditable way to generate synthetic
MCAT multiple-choice cards at scale (initial 200, stretch 500). This document
defines the item schema, a deterministic prompt/output contract, a
batch-by-KC-area generation strategy, a validation-pass specification, and the
inputs this harness needs from Track A's KC map.

**Status:** design only. No code, no generated cards, and no changes to any
other file are part of this deliverable. The 50-card demo deck in
`added features/mcat_demo_cards.md` remains the canonical source of truth for
the current demo; everything below is *additive* and format-compatible with it.

## 0. Grounding in existing code

The schema and output format below are pinned to how demo cards are already
parsed and imported, so generated cards can be ingested the same way.

- Markdown parser: `rslib/src/scheduler/concept_demo.rs`
  - `### <ID>` starts a new card; the text after `### ` is the card `id`.
  - Body bullets must be exactly `- **<Key>:** <value>` (the parser strips
    `- **` and splits on the literal `":** "`).
  - Recognized keys: `KC`, `Prereqs`, `Difficulty`, `Question`, `A`, `B`, `C`,
    `D`, `Correct`, `Explanation`, `Tags`. **Unknown keys are silently
    ignored** (`_ => ()`), which is what makes new bullets like `Reasoning`,
    `IRT`, and `Misconception` safe to add without breaking import.
  - Required at build time (the builder `.unwrap()`s these): `Difficulty`
    (parses to `u8`, must be `1..=5`), `Question`, `A`, `B`, `C`, `D`,
    `Correct` (must be one of `A|B|C|D`), `Explanation`.
  - `Prereqs` is either the literal `none` or a space-separated list of
    backtick-wrapped `` `Prereq::...` `` values.
  - `Tags` is a space-separated list of backtick-wrapped tags; **every tag on
    this line becomes the note's tags** (`note.tags = card.tags`).
- Tag consumption: `rslib/src/scheduler/concept.rs`
  (`CardConceptMetadata::from_tags`) and `qt/aqt/concept_tags.py`.
  - Prefixes read by the scheduler: `KC::`, `Overview::`, `Prereq::`, `MCAT::`,
    `Difficulty::`, `IRT::Discrimination::`, `IRT::Guessing::`, `Reasoning::`.
  - `IRT::Discrimination::` parses to `f64` and must be `> 0.0`; if absent the
    engine defaults to `1.0` (floored at `0.1`).
  - `IRT::Guessing::` parses to `f64` and is clamped to `[0,1]`; if absent the
    engine defaults to `0.25` (clamped to `[0.0, 0.95]`).
  - `Difficulty::` outside `1..=5` is dropped.
  - `Reasoning::` is stored as a free string; use the four canonical values.
  - Sections accepted by `McatSection::from_tag`: `Bio_Biochem`, `Chem_Phys`,
    `Psych_Soc`, `CARS` (camelCase variants also accepted; emit the
    underscore/`CARS` forms).

**Key consequence:** IRT and Reasoning metadata are *only* functional when they
appear on the `Tags` line (that is what flows into `note.tags`). Human-readable
`- **IRT:**` / `- **Reasoning:**` / `- **Misconception:**` bullets are for
authoring/validation clarity and are currently ignored by the importer. The
`Tags` line is authoritative for the engine.

---

## 1. Item schema

### 1.1 Field table

| Field | Required | Source of truth in card block | Format / constraint |
| --- | --- | --- | --- |
| `id` | yes | `### <ID>` header | Unique across all cards; deterministic (see 1.4). |
| `KC::` | yes | `- **KC:**` bullet **and** `Tags` line | Exactly one primary `KC::<Area>::<Topic>` that exists in the frozen Track A graph. |
| `Prereq::` | yes (may be `none`) | `- **Prereqs:**` bullet **and** `Tags` line | Zero or more `Prereq::<KC>`; must equal the frozen graph's prerequisite set for this KC (see 4.5). |
| `MCAT::` | yes | `Tags` line | One or more of `MCAT::Bio_Biochem`, `MCAT::Chem_Phys`, `MCAT::Psych_Soc`, `MCAT::CARS`. Primary section first. |
| `Difficulty::` | yes | `- **Difficulty:**` bullet **and** `Tags` line | Integer `1`–`5`. |
| `IRT::Discrimination::` | yes | `- **IRT:**` bullet **and** `Tags` line | Float `> 0.0`, recommended `0.4`–`2.0`. |
| `IRT::Guessing::` | yes | `- **IRT:**` bullet **and** `Tags` line | Float in `[0,1]`; for 4-choice items use `0.20`–`0.30` (default `0.25`). |
| `Reasoning::` | yes | `- **Reasoning:**` bullet **and** `Tags` line | One of `Conceptual`, `Application`, `Data`, `ResearchDesign`. |
| `question` | yes | `- **Question:**` bullet | Single line, self-contained, no external figure dependency for MVP. |
| `choices[4]` | yes | `- **A:**`…`- **D:**` bullets | Exactly four options; each non-empty; mutually exclusive; one unambiguously correct. |
| `answer` | yes | `- **Correct:**` bullet | Single letter `A`\|`B`\|`C`\|`D` pointing at the correct choice. |
| `explanation` | yes | `- **Explanation:**` bullet | Why the key is correct; single line. |
| `misconception` | yes | `- **Misconception:**` bullet | Names the specific wrong idea a strong distractor targets. |

Notes:
- `question`, choices, and `explanation` must be a single physical line each
  (the parser is line-based). Use `<br>` for internal breaks if ever needed.
- Do **not** use backticks inside the `Question`/`A`–`D`/`Explanation`/
  `Misconception` values; backticks are reserved for the `KC`, `Prereqs`, and
  `Tags` lines because the parser treats backtick pairs as delimiters there.

### 1.2 Canonical card block (authoring format)

This is the exact block a generator must emit. It is a strict superset of the
demo format: the extra `IRT`, `Reasoning`, and `Misconception` bullets are
ignored by the current importer and the machine values are duplicated onto the
`Tags` line where the engine actually reads them.

```text
## KC::<Area>::<Topic>

### <ID>

- **KC:** `<Area>::<Topic>`
- **Prereqs:** none
- **Difficulty:** 3
- **Reasoning:** Application
- **IRT:** discrimination=1.1, guessing=0.25
- **Question:** <single-line stem ending in a question>
- **A:** <option A>
- **B:** <option B>
- **C:** <option C>
- **D:** <option D>
- **Correct:** B
- **Explanation:** <one-line rationale for the key>
- **Misconception:** <the specific wrong idea distractor(s) target>
- **Tags:** `KC::<Area>::<Topic>` `MCAT::<Section>` `Difficulty::3` `IRT::Discrimination::1.1` `IRT::Guessing::0.25` `Reasoning::Application`
```

With prerequisites present, both the `Prereqs` bullet and the `Tags` line carry
them (mirroring `MCAT-BIOCHEM-BIOEN-001` in the demo):

```text
- **Prereqs:** `Prereq::Biochem::Enzymes` `Prereq::Bio::Eukaryotic_Cell`
...
- **Tags:** `KC::Biochem::Bioenergetics` `Prereq::Biochem::Enzymes` `Prereq::Bio::Eukaryotic_Cell` `MCAT::Bio_Biochem` `Difficulty::3` `IRT::Discrimination::1.1` `IRT::Guessing::0.25` `Reasoning::Application`
```

### 1.3 Overview / lesson-gate card variant (optional, from graph audit)

Overview cards (see `mcat-graph-audit.md`) introduce a KC without scoring
mastery. They add `Overview::<KC>` alongside `KC::<KC>` on the `Tags` line:

```text
- **Tags:** `Overview::<Area>::<Topic>` `KC::<Area>::<Topic>` `MCAT::<Section>` `Difficulty::1`
```

Overview cards are out of scope for the 200/500 practice-item targets but the
schema reserves the shape so the harness can emit them later without change.

### 1.4 Deterministic ID scheme

Replayability requires stable IDs: regenerating a KC must reproduce the same
IDs. IDs are:

```text
<AREA>-<TOPIC>-<NNN>
```

- `<AREA>`: fixed discipline code — `BIO`, `BCH` (Biochem), `GCH` (GenChem),
  `ORG` (Orgo), `PHY` (Physics), `PSY` (PsychSoc), `CAR` (CARS).
- `<TOPIC>`: the stable per-KC short code (`kc_code`) supplied by Track A
  (see §5). Example mapping: `Biochem::Peptides_and_Proteins` → `BCH-PEP`.
  Track A owning `kc_code` avoids ambiguous ad-hoc abbreviation and guarantees
  global uniqueness of the `<AREA>-<TOPIC>` prefix.
- `<NNN>`: zero-padded 3-digit sequence **within the KC**, starting at `001`,
  assigned in generation order.

Example: `BCH-PEP-001`, `BCH-PEP-002`, … Because sequence is per-KC and KC codes
are unique, IDs are globally unique and a KC batch re-run reproduces identical
IDs.

Legacy demo IDs (`MCAT-BIO-DNA-001`, `BIOCHEM-PEP-001`, `MCAT-BB-GLY-001`, …)
remain valid — the importer only needs uniqueness and the `### ` prefix — but
new generated content uses the canonical scheme above so the validator can
enforce it.

### 1.5 Output file layout

Generated cards are written to per-area files, one KC group (`## KC::…`) per
section, block format identical to §1.2:

```text
added features/generated/bio_biochem.md
added features/generated/chem_phys.md
added features/generated/psych_soc.md
added features/generated/cars.md
```

Rationale: keeps `mcat_demo_cards.md` as the untouched canonical demo, keeps
batches diff-friendly and re-runnable per area, and lets a future importer
change include these files exactly like the demo file. Wiring the Rust importer
(currently `include_str!(".../mcat_demo_cards.md")`) to also ingest
`generated/` is an integration step and is **out of scope** here; the format
guarantees drop-in compatibility when that happens.

---

## 2. Replayable prompt template and strict output format

### 2.1 Determinism principles

A "replay" = same inputs → same cards. The harness pins every input:

1. **Batch manifest** (checked in, human-authored) is the only variable input.
   It names the KC set, per-KC counts, difficulty targets, model, decoding
   params, and a seed. Same manifest ⇒ same batch.
2. **Pinned model + decoding**: fixed model name/version, `temperature = 0`
   (or a fixed seed if a nonzero temperature is ever needed), fixed
   `max_tokens`. Recorded in the manifest and the provenance sidecar.
3. **Frozen KC context**: the exact Track A `research-kc-*.md` snapshot
   (content hash) the batch was generated against is recorded, so drift in the
   KC map forces a new, explicit re-run rather than silent divergence.
4. **One KC per prompt call.** Generating per KC (not whole-deck) keeps the
   prompt small, makes IDs/counts deterministic, and isolates re-runs.
5. **Deterministic post-processing**: IDs assigned by KC + order (§1.4); tag
   strings emitted in a fixed field order (§1.2).

### 2.2 Batch manifest (input)

A small, reviewable file per run (proposed `added features/generated/manifests/<batch>.md`
or `.yaml`). Illustrative fields:

```yaml
batch_id: bio-foundations-01
model: <pinned-model-name-and-version>
decoding: { temperature: 0, top_p: 1, max_tokens: 1200, seed: 7 }
kc_map_snapshot: research-kc-biology.md@<sha256-prefix>
synthetic_attestation: true          # author asserts original, non-copyrighted
kcs:
  - kc: Bio::DNA
    kc_code: BIO-DNA
    section: Bio_Biochem
    prereqs: []
    count: 6
    difficulty: { 1: 1, 2: 2, 3: 2, 4: 1, 5: 0 }
    reasoning_mix: { Conceptual: 3, Application: 2, Data: 1, ResearchDesign: 0 }
  - kc: Bio::Genetics
    kc_code: BIO-GEN
    section: Bio_Biochem
    prereqs: [Bio::DNA]
    count: 6
    difficulty: { 1: 0, 2: 2, 3: 2, 4: 1, 5: 1 }
    reasoning_mix: { Conceptual: 2, Application: 2, Data: 1, ResearchDesign: 1 }
```

### 2.3 Prompt template (one call per KC)

Placeholders in `{{...}}` are filled from the manifest + Track A KC entry.

**System message (fixed across all calls):**

```text
You are generating original, synthetic MCAT-style multiple-choice practice items
for a spaced-repetition study tool. Rules:
- Write only ORIGINAL content. Do NOT copy, paraphrase, or reconstruct questions
  from AAMC materials, commercial prep (Kaplan, Princeton Review, UWorld, etc.),
  textbooks, or any copyrighted question bank. No brand names, no passage text.
- Each item is standalone conceptual reasoning, answerable without a figure.
- Exactly four options (A–D); exactly one unambiguously correct; three plausible
  distractors, each targeting a real misconception.
- Test understanding/application, not trivia or memorized numbers, unless the KC
  is inherently quantitative.
- Output MUST follow the exact block format specified. Emit NOTHING else:
  no preamble, no commentary, no markdown fences, no blank lines inside a block.
```

**User message (per KC):**

```text
KC: {{kc}}                         # e.g. Biochem::Enzymes
Section: {{section}}               # e.g. Bio_Biochem
Prerequisites: {{prereqs_or_none}} # e.g. Prereq::Biochem::Protein_Structure_and_Function
KC summary (from KC map): {{kc_summary}}
Learning objectives / sub-points: {{kc_objectives}}
Common misconceptions to target: {{kc_misconceptions}}

Generate {{count}} items for this KC with this difficulty distribution:
{{difficulty_histogram}}          # e.g. D1x1, D2x2, D3x2, D4x1
and this reasoning-type mix:
{{reasoning_mix}}                  # e.g. Conceptual x3, Application x2, Data x1

For every item, choose IRT values consistent with its difficulty:
- discrimination in [0.4, 2.0] (higher for cleaner, more diagnostic items)
- guessing = 0.25 for four-choice items unless strong distractors lower it.

Use id prefix "{{kc_code}}-" and number items 001, 002, ... in output order.
Set every Difficulty:: tag equal to the item's stated difficulty.
Set Prereq:: tags on BOTH the Prereqs bullet and the Tags line to exactly:
{{prereqs_or_none}}.

Output each item in EXACTLY this block, blocks separated by a single blank line:

### {{kc_code}}-<NNN>
- **KC:** `{{kc}}`
- **Prereqs:** {{prereqs_or_none}}
- **Difficulty:** <1-5>
- **Reasoning:** <Conceptual|Application|Data|ResearchDesign>
- **IRT:** discrimination=<0.4-2.0>, guessing=<0.0-1.0>
- **Question:** <one line>
- **A:** <one line>
- **B:** <one line>
- **C:** <one line>
- **D:** <one line>
- **Correct:** <A|B|C|D>
- **Explanation:** <one line>
- **Misconception:** <one line>
- **Tags:** `KC::{{kc}}` {{prereq_tags}} `MCAT::{{section}}` `Difficulty::<1-5>` `IRT::Discrimination::<x>` `IRT::Guessing::<x>` `Reasoning::<type>`
```

### 2.4 Strict output contract

- The model returns **only** a sequence of card blocks in §1.2 format, in ID
  order, separated by exactly one blank line. No `## ` header (the harness adds
  the KC group header deterministically), no fences, no prose.
- The harness performs a mechanical self-check before writing: block count ==
  requested count; IDs contiguous from `001`; every required bullet present;
  `Correct ∈ {A,B,C,D}`; `Difficulty` bullet == `Difficulty::` tag; `Prereq`
  bullet set == `Prereq::` tag set. Any failure ⇒ discard and re-run that KC
  (deterministic, since inputs are fixed), never hand-patch.
- The harness then emits the `## KC::…` group header and appends the validated
  blocks to the correct `generated/<section>.md` file, and writes a provenance
  sidecar (see 2.5).

### 2.5 Provenance sidecar (output)

Alongside each generated file, record how it was produced so a run is auditable
and reproducible (proposed `generated/manifests/<batch>.run.json`):

```json
{
  "batch_id": "bio-foundations-01",
  "generated_utc": "<timestamp>",
  "model": "<pinned-model>",
  "decoding": { "temperature": 0, "seed": 7 },
  "kc_map_snapshot": "research-kc-biology.md@<sha256>",
  "kcs": [{ "kc": "Bio::DNA", "kc_code": "BIO-DNA", "ids": ["BIO-DNA-001", "..."] }],
  "synthetic_attestation": true,
  "validator": { "status": "pass", "report": "reports/bio-foundations-01.json" }
}
```

---

## 3. Batch-by-KC-area generation strategy

### 3.1 Generate per KC area, in audited order

Follow the expansion order already agreed in `mcat-graph-audit.md` (never
whole-deck random):

1. Bio/Biochem molecular foundations (extends the current 10-KC demo graph).
2. General Chemistry KCs + edges.
3. Physics KCs + edges.
4. Organic Chemistry KCs + edges.
5. Psych/Soc KCs + edges.
6. CARS only if a meaningful KC structure exists beyond passage practice.

Per area: generate one KC at a time, validate the KC batch, then move on. A KC
is only eligible once it exists in the **frozen** Track A graph with its
prerequisites (so `Prereq::` tags can be emitted correctly and pass §4.5).

### 3.2 Targets and allocation

- **Initial target: 200 cards.** Prioritize foundation + mechanism KCs first;
  do not spread thin across all 100–150 planned KCs. Practical shapes:
  - ~6 cards/KC × ~33 KCs ≈ 200, **or**
  - 5–8 cards/KC across the highest-value ~25–35 KCs.
- **Stretch target: 500 cards**, unlocked only after validation quality holds
  (clean validator runs + human spot-checks). Reach 500 by (a) covering more
  KCs and (b) deepening high-yield KCs to 8–10 items.
- **Per-KC minimum: 5 items** (matches the demo's ~5/KC), so each KC can show
  calibration → application → stretch behavior in the scheduler.

Suggested first-200 split (indicative, tune from Track A weights): Bio ≈ 60,
Biochem ≈ 70, GenChem ≈ 35, Physics ≈ 20, Orgo ≈ 15; defer Psych/Soc + CARS to
the push toward 500.

### 3.3 Difficulty distribution guidance

Two layers of guidance:

- **Per KC (ladder):** every KC includes at least one calibration item
  (Difficulty ≤ 2) and at least one stretch item (Difficulty ≥ 4), difficulty
  rising within the KC — the same shape as the demo (`2 easy / 2 medium /
  1 harder`).
- **Per area (aggregate target), center-weighted:**

  | Difficulty | Target share |
  | --- | --- |
  | 1 | ~10% |
  | 2 | ~25% |
  | 3 | ~35% |
  | 4 | ~20% |
  | 5 | ~10% |

  Foundation KCs skew easier (more D1–D2); mechanism/application/detail KCs
  skew harder (more D3–D5), guided by Track A's foundation/mechanism/
  application/detail classification (§5).

### 3.4 IRT and reasoning guidance

- **Discrimination** rises with item quality/difficulty: ~`0.8`–`1.2` for
  foundational D1–D2, ~`1.2`–`1.8` for well-crafted D3–D5. Keep `> 0`.
- **Guessing** = `0.25` for four-choice items; lower toward `0.15`–`0.20` only
  when distractors are strong enough to suppress blind guessing.
- **Reasoning mix** shifts with depth: foundations lean `Conceptual`;
  mechanism/application lean `Application` and `Data`; experiment/graph items
  use `Data`; study-design items use `ResearchDesign`. Higher-difficulty items
  should over-index on `Application`/`Data`/`ResearchDesign`.

### 3.5 Re-run / regeneration workflow

1. Edit the batch manifest (add/adjust KCs, counts, difficulty/reasoning mix).
2. Re-run per changed KC only. Fixed inputs ⇒ identical IDs and (with
   `temperature 0`) identical items; unchanged KCs are untouched.
3. Run the validator (§4). On failure, fix the manifest or KC map — never
   hand-edit generated cards — and re-run.
4. Commit generated file(s) + manifest + provenance sidecar + validator report
   together so any batch is fully reproducible from what's in the repo.

---

## 4. Validation-pass specification

A validator runs on the generated files after every batch and in CI. This
section specifies **what it checks and its I/O** — it is **not implemented here**.

### 4.1 Checks

1. **Structural / parse-compat**: each `### <ID>` block parses under the same
   rules as `concept_demo.rs`; all required bullets present; `Question`/`A`–`D`/
   `Explanation`/`Misconception` are single-line and backtick-free.
2. **Tag validity**:
   - `KC::` present, exactly one primary, and exists in the frozen Track A KC set.
   - `MCAT::` ∈ {`Bio_Biochem`, `Chem_Phys`, `Psych_Soc`, `CARS`}.
   - `Difficulty::` integer `1..=5` and equals the `Difficulty` bullet.
   - `IRT::Discrimination::` parses to float `> 0` (warn if outside
     `[0.4, 2.0]`); `IRT::Guessing::` parses to float in `[0,1]` (warn if
     outside `[0.15, 0.30]` for 4-choice).
   - `Reasoning::` ∈ {`Conceptual`, `Application`, `Data`, `ResearchDesign`}.
   - No unknown `KC::`/`Prereq::` topics; separator is ASCII `::`.
3. **Duplicate IDs**: IDs unique across all generated files *and* not colliding
   with `mcat_demo_cards.md`; IDs match the `<AREA>-<TOPIC>-<NNN>` scheme with
   contiguous per-KC numbering.
4. **Answer format**: `Correct ∈ {A,B,C,D}`; the named option is non-empty; all
   four options non-empty and pairwise distinct; exactly one key.
5. **Difficulty distribution**: per-KC ladder satisfied (≥1 item ≤2 and ≥1 item
   ≥4 when KC has ≥5 items); per-area aggregate within tolerance of §3.3
   targets (warn, not fail, on modest deviation).
6. **Prerequisite consistency**: the set of `Prereq::` tags on each card equals
   the frozen graph's prerequisite set for that card's `KC::` (the exact
   invariant enforced by the Rust test
   `demo_card_prereq_tags_match_canonical_graph_edges`), and the `Prereqs`
   bullet matches the `Tags` line. Edge direction is prerequisite → target.
7. **Synthetic / no-copyrighted-material review** (defense in depth; cannot be
   fully automated):
   - Heuristic flags: brand/source names (AAMC, Kaplan, Princeton Review,
     UWorld, Examkrackers, etc.), "passage"-style prose, verbatim n-gram
     overlap against any provided reference corpus, and near-duplicate stems
     within the generated set (high similarity ⇒ flag).
   - Require `synthetic_attestation: true` in the manifest.
   - Emit a human spot-check sample (e.g. N random items/batch) into the report
     for manual sign-off before promotion to import.

### 4.2 Proposed location, inputs, outputs

- **Location:** `tools/mcat/validate_cards.py`, with fixtures/tests under
  `tools/tests/`. This matches the repo's Python tooling convention
  (`tools/…`, `tools/tests/…`). Per `AGENTS.md`, it is invoked via a **`just`
  recipe** (e.g. `just validate-mcat-cards`), not run directly, and wired into
  `just check`/CI.
  - Alternative if a non-build utility is preferred: `scripts/validate_mcat_cards.py`
    (sibling to `scripts/run-local-sync-server.sh`). `tools/mcat/` is preferred
    for CI integration.
- **Inputs:**
  - Generated card files: `added features/generated/*.md`.
  - Frozen KC map + edges from Track A: `added features/research-kc-*.md`
    (or the machine-readable export in §5).
  - Optional batch manifest(s) for expected counts/distribution.
  - Optional reference corpus path for overlap heuristics.
- **Outputs:**
  - Human-readable summary to stdout (counts, pass/fail per check, per-area
    difficulty histogram, prereq-mismatch list, flagged items).
  - Machine-readable report `added features/generated/reports/<batch>.json`
    (per-item results + aggregate stats) for CI and the provenance sidecar.
  - **Exit code**: `0` = pass, non-zero = at least one hard failure. Warnings
    (e.g. distribution drift, IRT out of recommended band) do not fail the run
    but are surfaced.

The validator is specified only. Implementation is a later step and must not be
built as part of this design task.

---

## 5. Content Contract inputs (what Track B needs from Track A)

Track A produces the `research-kc-*.md` KC map in parallel. For the two tracks
to connect, Track B needs the following **per KC**, in a stable, machine-usable
form. Track A's markdown is fine as long as these fields are present and
parseable (a small companion export — e.g. a table or JSON block — is preferred
so the validator/harness can consume it without brittle prose parsing).

**Required per-KC fields the harness consumes:**

1. **Canonical KC id** — exact `<Area>::<Topic>` string (ASCII `::`), matching
   what the scheduler expects (e.g. `Biochem::Enzymes`). This is the join key.
2. **`kc_code`** — the stable short code for IDs (§1.4), e.g.
   `Biochem::Enzymes` → `BCH-ENZ`. Must be unique across all KCs. (If Track A
   does not own this, Track B needs an agreed deterministic derivation rule
   instead — but a Track A-owned code is preferred.)
3. **Parent area / primary MCAT section** — to emit the correct `MCAT::` tag(s).
4. **Prerequisite list** — exact prerequisite KC ids for this KC, matching the
   canonical graph edges (prerequisite → target). This is required for the
   `Prereq::` tags and must satisfy the §4.5 consistency check.
5. **Overlapping sections** — any secondary `MCAT::` sections the KC can also
   provide evidence toward (Track A is charged with making overlap explicit;
   this mirrors `derived_mcat_sections_for_topics` in `concept_tags.py`).
6. **Suggested difficulty ladder / band** — the difficulty range appropriate
   for the KC, to seed per-KC and per-area distribution (§3.3).
7. **KC classification** — one of foundation / mechanism / application / detail,
   to bias difficulty, reasoning mix, and IRT discrimination (§3.3–3.4).
8. **KC summary** — 1–3 sentence scope statement used as `{{kc_summary}}` in
   the prompt so items stay on-topic and standalone.
9. **Learning objectives / sub-points** — short bullet list of what the KC
   covers (`{{kc_objectives}}`), to spread items across the KC's real content.
10. **Common misconceptions** — known wrong ideas for the KC
    (`{{kc_misconceptions}}`), to seed strong distractors and the
    `Misconception` field.

**Handshake / freeze criteria (so batches are reproducible):**

- Track A publishes a **frozen snapshot** of the KC map for the KCs in a batch;
  the harness records its content hash in the manifest and sidecar (§2). KC-map
  changes require an explicit new batch/run, not silent regeneration.
- **Naming must be exact.** KC ids and prerequisite ids must be byte-identical
  to what the scheduler graph uses; any rename in Track A is a breaking change
  that invalidates affected batches.
- Minimum to start Bio/Biochem generation: fields 1–7 for the KCs in that batch,
  with prerequisites consistent with the canonical graph. Fields 8–10 strongly
  improve item quality and are required for the stretch (500) target.

Nice-to-have (not required): per-KC blueprint weight (to weight area
allocation), and a curated list of high-yield sub-topics to prioritize first.
