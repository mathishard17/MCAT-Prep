# Content Contract (Track A ↔ Track B)

The frozen interface that connects **Track A** (the KC map,
`added features/kc-map-unified.md`) to **Track B** (the card/problem generation
harness, `added features/generation-harness.md`). It pins the per-KC schema the
harness consumes, the snapshot/versioning handshake that makes batches
reproducible, the A→B acceptance criteria, and the engine changes required to
actually schedule the expanded map.

- **Docs only.** No code, builds, or commits. The "Engine changes" in §5 are
  **specified, not implemented**.
- **Scope.** Tracks E (memory) and F (add-cards reliability) are already
  implemented; this contract records how they satisfy the criteria and are not
  blockers (§4.3).
- Judgment calls are marked `(verify)`.

---

## 1. The frozen KC-map schema Track B consumes

Track B needs 10 fields per KC (the handshake in `generation-harness.md` §5).
Each maps onto a column of the unified map (`kc-map-unified.md` §6) or its edge
list (§7). "Present now" means it is authored in the unified map today for all
172 KCs; "needed for stretch" means it must be added/consolidated before the
500-card push.

| # | Harness field | Unified-map source | Present now? | Notes |
| --- | --- | --- | --- | --- |
| 1 | **Canonical KC id** (`<Area>::<Topic>`, ASCII `::`) | `KC id` column | ✅ all 172 | Join key; byte-exact to the scheduler graph. |
| 2 | **`kc_code`** (stable short code for IDs) | `kc_code` column | ✅ all 172 | Track A-owned, globally unique; drives `<AREA>-<TOPIC>-<NNN>` IDs. Derivation rule in §1.1. |
| 3 | **Parent area / primary MCAT section** | area prefix of `KC id` + `Primary section` column | ✅ all 172 | Emits `MCAT::<Section>` (primary first). |
| 4 | **Prerequisite list** (exact prereq ids) | `Prerequisites (canonical ids)` column / §7 edge list | ✅ all 172 | Must equal the frozen graph edges for the KC (validator §4.6 / Rust `demo_card_prereq_tags_match_canonical_graph_edges`). |
| 5 | **Overlapping sections** | `Overlap sections` column | ✅ all 172 | Secondary `MCAT::` tags; `(minor)`/`*` = thin/passage-level (author per card). |
| 6 | **Suggested difficulty ladder / band** | `Difficulty` column (min–max band) | ✅ all 172 | Band only. The exact per-KC difficulty **histogram** is a manifest input (harness §2.2), seeded from the band + `Type`. |
| 7 | **KC classification** (foundation/mechanism/application/detail) | `Type` column | ✅ all 172 | Biases difficulty skew, reasoning mix, IRT discrimination. |
| 8 | **KC summary** (1–3 sentence scope) | per-KC "scope/notes" prose in the six `research-kc-*.md` maps | ⚠️ prose only | Not yet in a machine-readable per-KC field in the unified map. **Needed for stretch** — consolidate into a `kc_summary` column/JSON. |
| 9 | **Learning objectives / sub-points** | per-KC "Per-KC scope / sub-skills" bullets in `research-kc-*.md` | ⚠️ partial | Present for most KCs as prose; **needed for stretch** in a parseable form. |
| 10 | **Common misconceptions** | misconception hooks in `research-kc-general-chemistry.md` / `-organic-chemistry.md`; sparse elsewhere | ⚠️ partial | **Needed for stretch**; several disciplines lack explicit per-KC misconception lists. |

**Bottom line:** fields **1–7 are complete now** for all 172 KCs — enough to
start the initial 200-card generation (harness §3.2, foundation/mechanism-first).
Fields **8–10 exist as prose** in the research maps and must be lifted into a
per-KC machine-readable companion (see §1.2) before the 500-card stretch, because
the harness prompt template injects `{{kc_summary}}`, `{{kc_objectives}}`, and
`{{kc_misconceptions}}` (harness §2.3).

### 1.1 `kc_code` derivation rule (frozen)

`kc_code = <AREA>-<TOPIC>` where `<AREA> ∈ {BIO, BCH, GCH, ORG, PHY, PSY}` and
`<TOPIC>` is derived deterministically from the snake_case topic: first three
letters of the first significant word (stop-words `and/of/the/in/vs/a` dropped) +
first letter of each remaining word, uppercased, truncated to 6 chars; collisions
within an area resolved by a numeric suffix (`TRA` → `TRA2`). The resulting 172
codes are enumerated in `kc-map-unified.md` §6 and are the **authoritative frozen
set** — treat the table as the source of truth, not the algorithm (so a future
topic rename cannot silently shift a code).

### 1.2 Machine-readable companion (recommended)

The harness (§5) prefers a table/JSON companion over prose parsing. Recommended:
Track A publishes `added features/generated/kc-map.json` (or a fenced table)
derived from `kc-map-unified.md`, one record per KC with keys:
`kc`, `kc_code`, `area`, `primary_section`, `overlap_sections[]`, `prereqs[]`,
`difficulty_min`, `difficulty_max`, `type`, `status`, and (for the stretch)
`summary`, `objectives[]`, `misconceptions[]`. Until that exists, §6/§7 of the
unified map are the parseable source for fields 1–7.

### 1.3 Card emission contract (unchanged from harness)

Track B emits cards in the exact block format of `generation-harness.md` §1.2.
The engine only reads the **`Tags` line** (`note.tags = card.tags`), so machine
values must appear there: `KC::<id>`, zero+ `Prereq::<id>`, `MCAT::<section>`,
`Difficulty::<1-5>`, `IRT::Discrimination::<f>`, `IRT::Guessing::<f>`,
`Reasoning::<type>` (parsed by `CardConceptMetadata::from_tags` in
`rslib/src/scheduler/concept.rs`). Human-readable `- **IRT:**` / `- **Reasoning:**`
/ `- **Misconception:**` bullets are ignored by the importer and are for
authoring/validation only.

---

## 2. Snapshot / versioning handshake

Reproducible batches require both sides to agree on an immutable snapshot.

1. **Frozen snapshot = content hash of the unified map.** Track A freezes
   `added features/kc-map-unified.md` (superseding per-discipline
   `research-kc-*.md` as the single generation input). Compute:

   ```bash
   shasum -a 256 "added features/kc-map-unified.md"
   ```

   Track B records `kc-map-unified.md@<sha256-prefix>` in the batch manifest
   `kc_map_snapshot` field (harness §2.2) and in the provenance sidecar
   (`generated/manifests/<batch>.run.json`, harness §2.5). If a companion
   `kc-map.json` (§1.2) is used, hash and record that too.

2. **Byte-exact naming.** Every `KC::` and `Prereq::` id a generated card emits
   must be **byte-identical** to `kc-map-unified.md` §6/§7 (ASCII `::`, exact
   snake_case). Any rename in Track A is a **breaking change** that invalidates
   affected batches and forces a re-run. The reconciliations in
   `kc-map-unified.md` §3 (e.g. `GenChem::Covalent_Bond` → `GenChem::Chemical_Bonding`,
   `Physics::Fluids` → `Physics::Fluid_Dynamics`) are already applied — Track B
   must use the canonical right-hand-side ids only; the superseded ids must never
   appear on a card.

3. **Change ⇒ explicit re-run.** A new/edited KC or edge changes the hash; only
   the changed KCs are regenerated (harness §3.5). Unchanged KCs keep identical
   IDs (per-KC `<AREA>-<TOPIC>-<NNN>` numbering) and, at `temperature 0`,
   identical items. No silent regeneration; no hand-editing generated cards.

4. **Soft `(verify)` edges are versioned separately.** The 15 soft/`(verify)`
   cross-discipline edges (`kc-map-unified.md` §7) may be excluded from a given
   frozen snapshot without affecting the firm graph. The manifest should record
   whether soft edges are `included` or `excluded` so `Prereq::` sets stay
   consistent with the validator's expectation.

---

## 3. Naming / direction invariants (must hold in every batch)

- Edge direction is **`prerequisite -> target`**, stored via
  `add_prerequisite(target, prerequisite)`. A card for KC `T` carries
  `Prereq::P` for each `P -> T` edge, and its `Prereq::` set must **equal** the
  frozen graph's prerequisite set for `T` (no more, no fewer).
- Exactly **one primary `KC::`** per card.
- `MCAT::` ∈ {`Bio_Biochem`, `Chem_Phys`, `Psych_Soc`, `CARS`} (emit
  underscore/`CARS` forms; `McatSection::from_tag` also accepts camelCase).
- `Difficulty::` integer in the KC's band and equal to the `Difficulty` bullet.
- The combined graph implied by all emitted `KC::`/`Prereq::` tags stays acyclic
  (guaranteed if Track B uses only frozen edges; `KnowledgeGraph::cycle()` is the
  runtime guard).

---

## 4. Acceptance criteria (A → B)

### 4.1 Track A is "done enough to start B" when

- [x] Unified KC map exists, deduplicated and reconciled (`kc-map-unified.md`).
- [x] Combined graph is **globally acyclic** (172 KCs, 321 edges, full topo order;
      `kc-map-unified.md` §5).
- [x] Fields **1–7** present for every KC (§1).
- [x] Canonical demo subgraph (10 KCs, 9 edges) preserved byte-exact
      (`kc-map-unified.md` §4).
- [x] `kc_code` unique across all 172 KCs (§1.1).
- [x] A hashable frozen snapshot is publishable (§2).

### 4.2 A batch is acceptance-passing when (per harness §4)

- [ ] Every card's `KC::` exists in the frozen unified set.
- [ ] Every card's `Prereq::` set equals the frozen edges for that KC
      (mirrors Rust `demo_card_prereq_tags_match_canonical_graph_edges`).
- [ ] `MCAT::`, `Difficulty::`, `IRT::*`, `Reasoning::` valid and on the `Tags`
      line; `Difficulty` bullet == `Difficulty::` tag.
- [ ] IDs unique, contiguous per KC, no collision with `mcat_demo_cards.md`.
- [ ] Per-KC ladder (≥1 item ≤2 and ≥1 item ≥4 when ≥5 items) and per-area
      difficulty distribution within tolerance (harness §3.3).
- [ ] Synthetic-attestation present; no copyrighted-source flags.
- [ ] Manifest + provenance sidecar record the `kc_map_snapshot` hash.

### 4.3 Tracks E and F (already implemented — not blockers)

- **Track E (memory):** recall/memory surfacing is implemented —
  `Collection::card_memory`, `concept_kc_memory_for_deck`, and
  `section_memory_from_kc_memory` feed `overall_memory` / per-node `memory` /
  per-section `section_memory` in `concept_scheduler_status`
  (`rslib/src/scheduler/concept_demo.rs`). New KCs get memory automatically once
  their cards are studied — **no A/B change required**, but memory is only
  populated for KCs that (a) appear in the status graph and (b) have studied
  cards, so it depends on the §5(a) read-model change to show the expanded map.
- **Track F (add-cards reliability):** implemented — `concept_tags.py`
  (`concept_tags_meet_add_requirements`, metadata prefixes) plus the Add Cards
  rule in `mcat-graph-audit.md` (users supply `KC::`/`Difficulty::`/optional
  `MCAT::`/IRT; prerequisite direction stays in canonical authoring). Generated
  cards from Track B use the same tag surface, so they import identically — **no
  A/B change required**.

---

## 5. Engine changes needed to consume the expanded map (specify only)

These are **not implemented here.** They are the known gaps between the current
demo-only engine and consuming the 172-KC unified map.

### (a) Status read model is hardcoded to the 10-node demo graph

`Collection::concept_scheduler_status` (`rslib/src/scheduler/concept_demo.rs`)
builds its response from `canonical_mcat_demo_graph()` (the fixed `DEMO_KCS` /
`DEMO_EDGES`, 10 nodes / 9 edges) and `canonical_mcat_demo_edges()`. Nodes,
edges, `recommendations`, `section_scores`, and `has_cycle` are all derived from
that hardcoded graph, so **new KCs never appear** no matter how many tagged cards
exist.

**Required change (spec):** build the `KnowledgeGraph` used by the status/read
model from the expanded map instead of the hardcoded demo graph. Two sources,
ideally combined:

1. **Authored structure** — load the frozen unified map (`kc-map-unified.md`
   §6/§7, or the §1.2 `kc-map.json`) so KCs/edges exist even before any card is
   studied (needed for locked/outer-fringe display and recommendations).
2. **Card-derived structure** — `KnowledgeGraph::from_card_metadata(...)` already
   exists (`concept.rs`) and builds a graph from each card's
   `target_components` + `prerequisites`; use it to overlay the KCs/edges the
   deck's cards actually carry.

Recommended: construct from the authored frozen map, then attach mastery/memory
from card metadata + revlogs (the existing `reconstruct_*` / `concept_kc_memory_for_deck`
paths already key off `CardConceptMetadata`, so they generalize once the graph
has the nodes). Emit `has_cycle` from `graph.cycle()` on the real graph.

**Coupled test/data implication:** the unified map adds *new upstream* prereqs to
some demo KCs (`kc-map-unified.md` §4), so the current demo cards'
`Prereq::` tags no longer equal the (new) canonical edges for those KCs. The
regression test `demo_card_prereq_tags_match_canonical_graph_edges` would fail.
Resolve by either (i) regenerating the demo cards' `Prereq::` tags to match the
unified edges, or (ii) keeping the 10-KC demo as a separate frozen fixture
(`canonical_mcat_demo_graph()`) distinct from the production unified graph.
`(verify)` which approach the team prefers.

### (b) `McatDiscipline::for_component` collapses all `PsychSoc::` to Psychology

`McatDiscipline::for_component` (`rslib/src/scheduler/concept.rs`) maps **every**
`PsychSoc::` id to `McatDiscipline::Psychology`, so the `Sociology` weight in the
`Psych_Soc` blueprint (`PSYCH_SOC_DISCIPLINES`: Psychology 0.65 / **Sociology
0.30** / Biology 0.05) is **unreachable** — no card can ever contribute to the
Sociology discipline, so `Psych_Soc` coverage/IRT is skewed.

**Required change (spec):** split `PsychSoc::` into Psychology vs Sociology when
computing the discipline. The unified map already carries the intended
assignment in the PsychSoc **Sub-domain** column (`Psy` / `Soc` / `Psy/Soc`;
`kc-map-unified.md` §6). Options: (i) an explicit id→sub-domain lookup seeded
from that column; (ii) a `PsychSoc::Soc_...` naming convention; or (iii) a
per-card `Discipline::Sociology` override tag. `(verify)` the mechanism; the
Sub-domain column is the data source either way. `Psy/Soc` KCs
(`Prejudice_and_Bias`, `Group_Behavior`) need a tie-break rule.

### (c) Area-prefix coverage in `for_component` / `derived_mcat_sections_for_topics`

Both functions branch on the six area prefixes plus `CARS::`:

- `McatDiscipline::for_component` handles `Bio::`, `Biochem::`, `GenChem::`,
  `Orgo::`, `Physics::`, `PsychSoc::`, `CARS::`.
- `derived_mcat_sections_for_topics` (`qt/aqt/concept_tags.py`) handles the same
  prefixes.

**Finding:** **every one of the 172 unified KC ids uses one of these six handled
prefixes** — there are **no unhandled Area prefixes**, so no KC would be dropped
by `for_component` (returns `None`) or `derived_mcat_sections_for_topics`
(`continue`). The unified map introduces **no CARS KCs** (Psych/Soc research
intentionally excludes CARS content KCs). So the only prefix-level gap is the
Psychology/Sociology sub-split in (b); there is nothing to add for (c) beyond
keeping both functions in sync if a new prefix (e.g. `CARS::`) is ever
introduced. `(verify)` if a `CARS::` KC lattice is later added — both functions
already accept the prefix, but no such KCs exist yet.

### (d) (Related) section derivation is prefix-based, not per-KC overlap

`derived_mcat_sections_for_topics` derives sections **from the area prefix
only** (e.g. all `Physics::` → `Chem_Phys`), so the content-real overlaps the
unified map records (`Overlap sections`, e.g. `Physics::Bioelectricity` →
`Bio_Biochem*`, or `Bio::Nervous_System` → `Psych_Soc(minor)`) are **not**
auto-applied. To make those overlaps contribute IRT evidence, Track B must emit
the extra `MCAT::` tag **per card**, or the engine must consult a per-KC overlap
table. This is an authoring choice, not a blocker for fields 1–7. `(verify)`.

---

## 6. One-paragraph summary

Track A's frozen deliverable is `added features/kc-map-unified.md`: a single,
globally acyclic, machine-usable graph of **172 KCs / 321 edges** across all six
disciplines, with the 10-KC demo subgraph preserved byte-exact and cross-discipline
naming reconciled to one canonical id per concept. This Content Contract freezes
the per-KC schema Track B consumes — the 10-field handshake, of which **fields
1–7 (id, `kc_code`, section, prereqs, overlaps, difficulty band, type) are
complete now** and **fields 8–10 (summary, objectives, misconceptions) exist as
research-map prose that must be lifted into a machine-readable companion before
the 500-card stretch** — plus the content-hash snapshot/versioning handshake
(byte-exact ids; any rename is a breaking re-run) and the A→B acceptance
criteria (with Tracks E memory and F add-cards reliability already implemented
and non-blocking). It also specifies, without implementing, the three engine
changes required to actually schedule the expanded map: (a) rebuild the status
read model from the unified map / card metadata instead of the hardcoded
`canonical_mcat_demo_graph()` so new KCs surface, (b) split `PsychSoc::` into
Psychology vs Sociology in `McatDiscipline::for_component` so the Sociology
blueprint weight becomes reachable (using the map's Sub-domain column), and (c)
confirmation that all 172 KC prefixes are already handled by the discipline/
section helpers, so no Area prefix is dropped and only the Psych/Soc sub-split
remains.
