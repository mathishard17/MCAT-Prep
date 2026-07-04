# Research KC Map — MCAT Psychology / Sociology (Track A)

Machine-usable Knowledge Component (KC) map for the MCAT **Psychological, Social,
and Biological Foundations of Behavior** section (Psych/Soc). It expands the flat
`PsychSoc::` list in `added features/mcat.md` into a prerequisite-friendly,
**acyclic** lattice for the Concept Scheduler.

- Scope: AAMC foundational concepts 6–10 (sensation/perception, learning/memory,
  cognition/language/intelligence, motivation/emotion/stress, identity/personality,
  psychological disorders, social psychology/attitudes/group behavior, social
  structures/institutions, demographics, social inequality/stratification, and
  health/healthcare disparities).
- Content is **synthetic / original** research scaffolding, not copied from any
  copyrighted prep material. Uncertain placements are marked **(verify)**.
- Docs only. No code or other files are modified by this research pass.

## Tag conventions used here

Reused verbatim from `qt/aqt/concept_tags.py` and
`rslib/src/scheduler/concept.rs` so the map is directly taggable:

- KC id: `KC::PsychSoc::<Topic_in_snake_case>` (the `KC id` column below omits the
  leading `KC::` and shows the `PsychSoc::...` component id).
- Prerequisite tag: `Prereq::<KC id>` (e.g. `Prereq::PsychSoc::Cognition`,
  `Prereq::Bio::Nervous_System`). Edges are authored as
  `prerequisite -> target`, matching `mcat-graph-audit.md`.
- Section tag: `MCAT::Psych_Soc` (primary for every KC below). Overlap sections
  use `MCAT::Bio_Biochem` / `MCAT::Chem_Phys`.
- `Difficulty::1..5`, `IRT::Discrimination::x`, `IRT::Guessing::x`,
  `Reasoning::{Conceptual|Application|Data|ResearchDesign}` are set per card, not
  per KC. The **Difficulty ladder** column is the suggested span of card
  difficulties for a KC (easy calibration → harder application/reasoning).

Note on section weighting (`concept.rs`): `Psych_Soc` blueprint weight is
Psychology 0.65 / Sociology 0.30 / Biology 0.05, and `McatDiscipline::for_component`
currently maps every `PsychSoc::` id to `Psychology` (psychology vs. sociology is
not yet split in the engine). The `Psych sub-domain` column below records the
intended psychology-vs-sociology split for when that split is implemented; it does
not change today's behavior.

## Cluster overview

| Cluster | AAMC anchor | KCs |
| --- | --- | --- |
| Biological & Social Foundations | FC 7A | `Biological_and_Social_Factors` |
| Sensation & Perception | FC 6A | `Sensory_Processing`, `The_Senses`, `Perception`, `Attention` |
| Cognition & Consciousness | FC 6B | `Cognition`, `Consciousness`, `Memory`, `Language`, `Intelligence` |
| Learning | FC 7C | `Learning` |
| Motivation, Emotion & Stress | FC 6C / 7A | `Emotion`, `Motivation`, `Stress` |
| Identity & Personality | FC 7A / 8A | `Personality`, `Self_and_Identity` |
| Psychological Disorders | FC 7A | `Psychological_Disorders` |
| Social Psychology | FC 7B / 8B / 8C | `Attitudes_and_Beliefs`, `Stereotypes`, `Prejudice_and_Bias`, `Social_Interaction`, `Group_Behavior` |
| Social Structure & Culture | FC 9A / 7B | `Social_Theory`, `Culture`, `Socialization`, `Social_Institutions` |
| Demographics | FC 9B | `Demographics` |
| Social Inequality & Stratification | FC 10A | `Stratification`, `Social_Class`, `Social_Mobility`, `Poverty`, `Social_Inequality` |
| Health & Healthcare Disparities | FC 10A | `Health_Disparities`, `Healthcare_Disparities` |

## Master KC table

Legend — Type: `foundation` (root/early, calibration-friendly), `mechanism`
(process/model that other KCs build on), `application` (integrative/scenario).
Prereqs are KC ids; unless prefixed `Bio::`, all prereqs are `PsychSoc::`. `—` =
no prerequisite (a graph root). Sub-domain: `Psy` = psychology, `Soc` = sociology.

| # | KC id | Cluster | Type | Sub-domain | Prerequisites (`prereq -> this KC`) | Overlap sections | Difficulty ladder |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `PsychSoc::Biological_and_Social_Factors` | Bio & Social Foundations | foundation | Psy | `Bio::Nervous_System`, `Bio::Endocrine_System` (verify), `Bio::Genetics` (verify) | Psych_Soc, Bio_Biochem | 1–4 |
| 2 | `PsychSoc::Sensory_Processing` | Sensation & Perception | foundation | Psy | `Bio::Nervous_System` | Psych_Soc, Bio_Biochem, Chem_Phys (verify) | 1–3 |
| 3 | `PsychSoc::The_Senses` | Sensation & Perception | mechanism | Psy | `PsychSoc::Sensory_Processing` | Psych_Soc, Bio_Biochem, Chem_Phys (verify) | 1–4 |
| 4 | `PsychSoc::Attention` | Sensation & Perception | mechanism | Psy | `PsychSoc::Sensory_Processing` | Psych_Soc | 2–4 |
| 5 | `PsychSoc::Perception` | Sensation & Perception | mechanism | Psy | `PsychSoc::The_Senses`, `PsychSoc::Attention` | Psych_Soc, Chem_Phys (verify) | 2–4 |
| 6 | `PsychSoc::Cognition` | Cognition & Consciousness | mechanism | Psy | `PsychSoc::Perception`, `PsychSoc::Attention` | Psych_Soc | 2–4 |
| 7 | `PsychSoc::Memory` | Cognition & Consciousness | mechanism | Psy | `PsychSoc::Cognition` | Psych_Soc, Bio_Biochem (verify) | 2–5 |
| 8 | `PsychSoc::Consciousness` | Cognition & Consciousness | mechanism | Psy | `PsychSoc::Attention`, `PsychSoc::Cognition` | Psych_Soc, Bio_Biochem | 2–4 |
| 9 | `PsychSoc::Language` | Cognition & Consciousness | mechanism | Psy | `PsychSoc::Cognition` | Psych_Soc, Bio_Biochem (verify) | 2–4 |
| 10 | `PsychSoc::Intelligence` (verify) | Cognition & Consciousness | application | Psy | `PsychSoc::Cognition`, `PsychSoc::Memory`, `PsychSoc::Language` | Psych_Soc | 3–5 |
| 11 | `PsychSoc::Learning` (new) | Learning | mechanism | Psy | `PsychSoc::Biological_and_Social_Factors` | Psych_Soc, Bio_Biochem (verify) | 1–4 |
| 12 | `PsychSoc::Emotion` | Motivation, Emotion & Stress | mechanism | Psy | `PsychSoc::Biological_and_Social_Factors` | Psych_Soc, Bio_Biochem | 2–4 |
| 13 | `PsychSoc::Motivation` | Motivation, Emotion & Stress | mechanism | Psy | `PsychSoc::Biological_and_Social_Factors`, `PsychSoc::Emotion` | Psych_Soc, Bio_Biochem | 2–4 |
| 14 | `PsychSoc::Stress` | Motivation, Emotion & Stress | application | Psy | `PsychSoc::Emotion`, `Bio::Endocrine_System` (verify) | Psych_Soc, Bio_Biochem | 2–4 |
| 15 | `PsychSoc::Personality` | Identity & Personality | mechanism | Psy | `PsychSoc::Biological_and_Social_Factors` | Psych_Soc | 2–4 |
| 16 | `PsychSoc::Self_and_Identity` | Identity & Personality | application | Psy | `PsychSoc::Personality`, `PsychSoc::Socialization` | Psych_Soc | 2–4 |
| 17 | `PsychSoc::Psychological_Disorders` | Psychological Disorders | application | Psy | `PsychSoc::Biological_and_Social_Factors`, `PsychSoc::Emotion`, `PsychSoc::Stress` | Psych_Soc, Bio_Biochem | 3–5 |
| 18 | `PsychSoc::Attitudes_and_Beliefs` | Social Psychology | mechanism | Psy | `PsychSoc::Cognition`, `PsychSoc::Learning` | Psych_Soc | 2–4 |
| 19 | `PsychSoc::Stereotypes` | Social Psychology | mechanism | Psy | `PsychSoc::Cognition`, `PsychSoc::Attitudes_and_Beliefs` | Psych_Soc | 3–5 |
| 20 | `PsychSoc::Prejudice_and_Bias` | Social Psychology | application | Psy/Soc | `PsychSoc::Attitudes_and_Beliefs`, `PsychSoc::Stereotypes` | Psych_Soc | 3–5 |
| 21 | `PsychSoc::Social_Interaction` (new) | Social Psychology | application | Psy | `PsychSoc::Self_and_Identity`, `PsychSoc::Attitudes_and_Beliefs` | Psych_Soc | 3–5 |
| 22 | `PsychSoc::Group_Behavior` (new) | Social Psychology | application | Psy/Soc | `PsychSoc::Social_Interaction`, `PsychSoc::Socialization` | Psych_Soc | 3–5 |
| 23 | `PsychSoc::Social_Theory` (new, verify) | Social Structure & Culture | foundation | Soc | — | Psych_Soc | 2–4 |
| 24 | `PsychSoc::Culture` | Social Structure & Culture | foundation | Soc | — | Psych_Soc | 1–3 |
| 25 | `PsychSoc::Socialization` (new) | Social Structure & Culture | mechanism | Soc | `PsychSoc::Culture` | Psych_Soc | 1–3 |
| 26 | `PsychSoc::Social_Institutions` (new) | Social Structure & Culture | application | Soc | `PsychSoc::Social_Theory`, `PsychSoc::Culture` | Psych_Soc | 2–4 |
| 27 | `PsychSoc::Demographics` (new) | Demographics | application | Soc | `PsychSoc::Social_Institutions`, `PsychSoc::Culture` | Psych_Soc | 2–4 |
| 28 | `PsychSoc::Stratification` | Social Inequality | mechanism | Soc | `PsychSoc::Social_Theory` | Psych_Soc | 2–4 |
| 29 | `PsychSoc::Social_Class` | Social Inequality | application | Soc | `PsychSoc::Stratification` | Psych_Soc | 2–4 |
| 30 | `PsychSoc::Social_Mobility` | Social Inequality | application | Soc | `PsychSoc::Stratification`, `PsychSoc::Social_Class` | Psych_Soc | 2–4 |
| 31 | `PsychSoc::Poverty` | Social Inequality | application | Soc | `PsychSoc::Social_Class` | Psych_Soc | 2–4 |
| 32 | `PsychSoc::Social_Inequality` (new, verify) | Social Inequality | application | Soc | `PsychSoc::Stratification`, `PsychSoc::Demographics` | Psych_Soc | 3–5 |
| 33 | `PsychSoc::Health_Disparities` | Health & Healthcare Disparities | application | Soc | `PsychSoc::Social_Inequality`, `PsychSoc::Social_Class` | Psych_Soc, Bio_Biochem (verify) | 3–5 |
| 34 | `PsychSoc::Healthcare_Disparities` (new, verify) | Health & Healthcare Disparities | application | Soc | `PsychSoc::Health_Disparities`, `PsychSoc::Social_Institutions` | Psych_Soc | 3–5 |

**Total: 34 KCs** — 24 reused verbatim from `mcat.md`, 10 new
(`Intelligence`, `Learning`, `Social_Interaction`, `Group_Behavior`,
`Social_Theory`, `Socialization`, `Social_Institutions`, `Demographics`,
`Social_Inequality`, `Healthcare_Disparities`).

## Canonical prerequisite edge list

Authored as `prerequisite -> target` (the audit convention; stored in Rust via
`add_prerequisite(target, prerequisite)`). Cross-discipline `Bio::` edges are
listed first.

### Cross-discipline edges (Bio -> PsychSoc)

```text
Bio::Nervous_System   -> PsychSoc::Biological_and_Social_Factors
Bio::Endocrine_System -> PsychSoc::Biological_and_Social_Factors   # (verify)
Bio::Genetics         -> PsychSoc::Biological_and_Social_Factors   # (verify)
Bio::Nervous_System   -> PsychSoc::Sensory_Processing
Bio::Endocrine_System -> PsychSoc::Stress                          # (verify) HPA axis / cortisol
```

### Within-Psych/Soc edges

```text
PsychSoc::Sensory_Processing            -> PsychSoc::The_Senses
PsychSoc::Sensory_Processing            -> PsychSoc::Attention
PsychSoc::The_Senses                    -> PsychSoc::Perception
PsychSoc::Attention                     -> PsychSoc::Perception
PsychSoc::Perception                    -> PsychSoc::Cognition
PsychSoc::Attention                     -> PsychSoc::Cognition
PsychSoc::Cognition                     -> PsychSoc::Memory
PsychSoc::Attention                     -> PsychSoc::Consciousness
PsychSoc::Cognition                     -> PsychSoc::Consciousness
PsychSoc::Cognition                     -> PsychSoc::Language
PsychSoc::Cognition                     -> PsychSoc::Intelligence      # (verify)
PsychSoc::Memory                        -> PsychSoc::Intelligence      # (verify)
PsychSoc::Language                      -> PsychSoc::Intelligence      # (verify)
PsychSoc::Biological_and_Social_Factors -> PsychSoc::Learning
PsychSoc::Biological_and_Social_Factors -> PsychSoc::Emotion
PsychSoc::Biological_and_Social_Factors -> PsychSoc::Motivation
PsychSoc::Emotion                       -> PsychSoc::Motivation
PsychSoc::Emotion                       -> PsychSoc::Stress
PsychSoc::Biological_and_Social_Factors -> PsychSoc::Personality
PsychSoc::Biological_and_Social_Factors -> PsychSoc::Psychological_Disorders
PsychSoc::Emotion                       -> PsychSoc::Psychological_Disorders
PsychSoc::Stress                        -> PsychSoc::Psychological_Disorders
PsychSoc::Personality                   -> PsychSoc::Self_and_Identity
PsychSoc::Socialization                 -> PsychSoc::Self_and_Identity
PsychSoc::Cognition                     -> PsychSoc::Attitudes_and_Beliefs
PsychSoc::Learning                      -> PsychSoc::Attitudes_and_Beliefs
PsychSoc::Cognition                     -> PsychSoc::Stereotypes
PsychSoc::Attitudes_and_Beliefs         -> PsychSoc::Stereotypes
PsychSoc::Attitudes_and_Beliefs         -> PsychSoc::Prejudice_and_Bias
PsychSoc::Stereotypes                   -> PsychSoc::Prejudice_and_Bias
PsychSoc::Self_and_Identity             -> PsychSoc::Social_Interaction
PsychSoc::Attitudes_and_Beliefs         -> PsychSoc::Social_Interaction
PsychSoc::Social_Interaction            -> PsychSoc::Group_Behavior
PsychSoc::Socialization                 -> PsychSoc::Group_Behavior
PsychSoc::Culture                       -> PsychSoc::Socialization
PsychSoc::Social_Theory                 -> PsychSoc::Social_Institutions
PsychSoc::Culture                       -> PsychSoc::Social_Institutions
PsychSoc::Social_Institutions           -> PsychSoc::Demographics
PsychSoc::Culture                       -> PsychSoc::Demographics
PsychSoc::Social_Theory                 -> PsychSoc::Stratification
PsychSoc::Stratification                -> PsychSoc::Social_Class
PsychSoc::Stratification                -> PsychSoc::Social_Mobility
PsychSoc::Social_Class                  -> PsychSoc::Social_Mobility
PsychSoc::Social_Class                  -> PsychSoc::Poverty
PsychSoc::Stratification                -> PsychSoc::Social_Inequality   # (verify)
PsychSoc::Demographics                  -> PsychSoc::Social_Inequality   # (verify)
PsychSoc::Social_Inequality             -> PsychSoc::Health_Disparities
PsychSoc::Social_Class                  -> PsychSoc::Health_Disparities
PsychSoc::Health_Disparities            -> PsychSoc::Healthcare_Disparities  # (verify)
PsychSoc::Social_Institutions           -> PsychSoc::Healthcare_Disparities  # (verify)
```

### Acyclicity

The graph is a DAG. A valid topological layering (every edge points from a lower
layer to a higher one) is:

- **L0 (roots):** `Biological_and_Social_Factors`, `Sensory_Processing`,
  `Social_Theory`, `Culture`
- **L1:** `The_Senses`, `Attention`, `Emotion`, `Learning`, `Socialization`,
  `Stratification`, `Social_Institutions`
- **L2:** `Perception`, `Motivation`, `Stress`, `Personality`, `Social_Class`,
  `Demographics`
- **L3:** `Cognition`, `Psychological_Disorders`, `Self_and_Identity`,
  `Social_Mobility`, `Poverty`, `Social_Inequality`
- **L4:** `Memory`, `Consciousness`, `Language`, `Attitudes_and_Beliefs`,
  `Health_Disparities`
- **L5:** `Intelligence`, `Stereotypes`, `Social_Interaction`,
  `Healthcare_Disparities`
- **L6:** `Prejudice_and_Bias`, `Group_Behavior`

Because a consistent layering exists, there are no back edges and no cycles. This
mirrors the check enforced by `KnowledgeGraph::cycle()` in
`rslib/src/scheduler/concept.rs`.

## Cross-discipline prerequisites referenced

Only three `Bio::` KCs are used as prerequisites (all already present in
`mcat.md`); Chem/Phys appears as **overlap only**, never as a prerequisite:

| Cross-discipline KC | Feeds into | Status |
| --- | --- | --- |
| `Bio::Nervous_System` | `Biological_and_Social_Factors`, `Sensory_Processing` | firm |
| `Bio::Endocrine_System` | `Biological_and_Social_Factors`, `Stress` | (verify) |
| `Bio::Genetics` | `Biological_and_Social_Factors` | (verify, behavioral genetics/heritability) |

Overlap-only (no prereq edge): `Chem_Phys` is tagged as an overlap section for
`Sensory_Processing`, `The_Senses`, and `Perception` because vision/hearing and
psychophysics touch optics/acoustics/signal detection (`Physics::Optics`,
`Physics::Light`, `Physics::Sound`) — but those are intentionally **not** made
hard prerequisites, to keep Psych/Soc study from being gated on physics mastery.

## Difficulty-ladder rationale

- **Foundations** (`Sensory_Processing`, `Culture`) start at Difficulty 1 for
  calibration; `Biological_and_Social_Factors` and `Social_Theory` run 1–4 / 2–4
  because their abstract/integrative facets get hard.
- **Mechanisms** typically span 2–4; `Memory` reaches 5 (many models + biological
  detail) and `Learning`/`The_Senses` start at 1 (conditioning basics, simple
  sensory facts) but climb to 4 (reinforcement schedules, signal transduction
  detail).
- **Applications / integrative KCs** (`Intelligence`, `Psychological_Disorders`,
  `Stereotypes`, `Prejudice_and_Bias`, `Social_Interaction`, `Group_Behavior`,
  `Social_Inequality`, `Health_Disparities`, `Healthcare_Disparities`) run 3–5,
  favoring `Reasoning::Application`, `Reasoning::Data`, and
  `Reasoning::ResearchDesign` cards (study interpretation, confounds, method
  critique) at the top of the ladder.

Foundation-before-detail: the scheduler should sample the easy end of a KC's
ladder first and only surface Difficulty 4–5 cards once earlier cards in that KC
(and its prerequisites) are reasonably mastered.

## Research notes

- **Coverage vs. AAMC.** The 34 KCs map to AAMC Foundational Concepts 6
  (sensing/organizing/responding to the environment), 7 (individual + social
  bases of behavior and attitude/behavior change), 8 (self, social thinking,
  social interaction), 9 (social structure + demographics), and 10 (social
  inequality). Every task-listed topic area has at least one KC.
- **What "detail" maps to.** The four allowed types are `foundation | mechanism |
  application | detail`. Deliberately, **no KC is authored at `detail` grain** —
  KCs are kept at topic grain, and detail-level facts (e.g., a specific
  neurotransmitter in a disorder, one theorist's stage names) live at the
  **card** level as Difficulty 4–5 items tagged to the parent KC. This keeps the
  graph small enough for readable deck-options visualization while still letting
  cards carry fine detail.
- **New KCs added (10).** `Learning` and `Intelligence` fill obvious FC 6B/7C
  gaps in the original list. `Social_Interaction` and `Group_Behavior` give social
  psychology (FC 8C / 7B) explicit homes for self-presentation, attraction,
  aggression, conformity, obedience, and group processes. `Social_Theory`,
  `Socialization`, `Social_Institutions`, and `Demographics` supply the FC 9
  sociology backbone. `Social_Inequality` anchors FC 10A spatial/global inequality
  distinct from the stratification *mechanism*, and `Healthcare_Disparities`
  separates the healthcare-system/access lens from population `Health_Disparities`.
- **`(verify)` items.**
  - `Intelligence` placement/edges — sometimes treated as a sub-topic of
    `Cognition` rather than a standalone KC; kept separate because AAMC tests
    theories/heritability/testing explicitly.
  - `Social_Theory` as a standalone KC — an alternative is to fold sociological
    paradigms (functionalist, conflict, symbolic-interactionist, social-constructionist,
    exchange/rational-choice, feminist) into each downstream KC instead of a root.
  - `Social_Inequality` vs. `Stratification` boundary — treated as
    result/patterns (spatial, environmental, global, intersectional) produced by
    the stratification mechanism; the split is a judgment call.
  - `Healthcare_Disparities` vs. `Health_Disparities` — split into
    system/access/quality vs. population health outcomes; some sources merge them.
  - `Bio::Endocrine_System`, `Bio::Genetics` prereq edges and the `Bio_Biochem`
    overlaps — pedagogically sound but should be confirmed once the Bio track's KC
    ids are frozen (they match `mcat.md` today).
- **Psychology vs. sociology split.** `McatDiscipline::for_component` maps all
  `PsychSoc::` ids to `Psychology`, so the `Sociology` discipline weight (0.30 of
  `Psych_Soc`) is currently unreachable. The `Sub-domain` column above is the
  proposed Psy/Soc assignment to drive that split later; implementing it would let
  section mastery/coverage reflect the AAMC blueprint more faithfully. **(verify)**
- **CARS is intentionally excluded.** The Critical Analysis and Reasoning Skills
  section is skill/reasoning-based (analysis, reasoning within/beyond the text)
  and has **no content KCs**; it is not part of this content-KC research. It is
  better modeled with `Reasoning::{Conceptual|Application|Data|ResearchDesign}`
  tags on passage-practice items (and, if ever needed, a small `CARS::` skill
  lattice) rather than a `PsychSoc::`-style content graph.
- **Consistency with the engine.** All ids use the existing `PsychSoc::`
  namespace and `snake_case` topics; all edges use the `prerequisite -> target`
  convention and pass the acyclicity requirement; every KC's primary section is
  `Psych_Soc`, with `Bio_Biochem`/`Chem_Phys` overlaps recorded but not enforced
  as prerequisites.
```