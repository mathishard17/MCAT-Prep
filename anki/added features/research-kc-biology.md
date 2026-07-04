# Research KC Map: MCAT Biology (Track A)

Machine-usable Knowledge Component (KC) map for MCAT **Biology**. This is a
research artifact for Track A of the feature expansion plan. It expands the
Bio slice of the demo graph toward a card-ready lattice while staying
compatible with the existing tag schema and the canonical prerequisite-edge
direction.

All content here is synthetic/original scope description. No copyrighted prep
text is reproduced. Items whose classification or edge is a judgement call are
marked `(verify)`.

## How to read this map

- **KC id** — Anki tag of the form `KC::Bio::<Topic_in_snake_case>`. The table
  lists the bare id (`Bio::...`); the live tag adds the `KC::` prefix.
- **Cluster** — topic cluster inside the Biology area (area is always `Bio`).
- **Type** — one of:
  - `foundation` — prerequisite anchor; calibration-friendly; sampled early.
  - `mechanism` — a process/how-it-works KC that builds on foundations.
  - `application` — integrates multiple KCs; system-level or problem-solving.
  - `detail` — narrower, lower-yield recall that hangs off a broader KC.
- **Prerequisites** — KC ids that should be reasonably strong first, written so
  that each listed id is a *prerequisite of this KC* (i.e. `prereq -> this KC`).
  May reference other disciplines (`Biochem::...`, `GenChem::...`,
  `Physics::...`). All edges are acyclic (see the edge appendix).
- **MCAT sections** — section tag(s) the KC can supply evidence to
  (`MCAT::<Section>`). Biology is *primarily* `Bio_Biochem`. `(minor)` marks a
  genuine but small content slice into `Psych_Soc` or `Chem_Phys`.
- **Difficulty** — the `Difficulty::1..5` band that a card set for this KC
  typically spans (the suggested ladder). Following the demo allocation in
  `mcat.md`, a full KC card set skews to ~2 lower-band calibration cards, ~2
  mid-band application cards, and ~1 top-band reasoning card.

### Difficulty ladder convention

`Difficulty::1..5` maps to IRT item difficulty in the engine
(`difficulty_to_irt_b`: 1 -> -2.0 up to 5 -> +2.0). The bands below are the
*typical* levels that apply per KC, not a hard cap:

- `foundation` KCs anchor low (usually 1-3).
- `mechanism` KCs sit mid (usually 2-4).
- `application` KCs reach high (usually 3-5).
- `detail` KCs are mostly recall (usually 1-3).

### Reasoning emphasis (for Track B generation)

Suggested `Reasoning::...` skew by type (not a per-KC column here, but a
generation hint): `foundation` -> mostly `Conceptual`; `mechanism` ->
`Conceptual`/`Application`; `application` -> `Application`/`Data`; `detail` ->
`Conceptual`. Quantitative KCs (`Bio::Population_Genetics`,
`Bio::Excretory_System`) should include `Data`; experimental/lab KCs
(`Bio::Biotechnology`) should include some `ResearchDesign`.

---

## Cluster A: Molecular Biology

| KC id | Cluster | Type | Prerequisites (prereq -> this KC) | MCAT sections | Difficulty | Notes |
|---|---|---|---|---|---|---|
| `Bio::DNA` | Molecular Biology | foundation | (none) | `Bio_Biochem` | 1-3 | Reuse (demo KC). Nucleic-acid structure, base pairing, chromosomes. |
| `Bio::DNA_Replication` | Molecular Biology | mechanism | `Bio::DNA`, `Biochem::Enzymes` (verify) | `Bio_Biochem` | 2-4 | Semiconservative model, polymerases, leading/lagging strands, proofreading. |
| `Bio::Transcription` | Molecular Biology | mechanism | `Bio::DNA`, `Biochem::Nucleotides_and_Nucleic_Acids` | `Bio_Biochem` | 2-4 | RNA synthesis, mRNA processing, splicing. |
| `Bio::Translation` | Molecular Biology | mechanism | `Bio::Transcription`, `Biochem::Peptides_and_Proteins` | `Bio_Biochem` | 2-4 | Ribosomes, tRNA, the genetic code, post-translational basics. |
| `Bio::Gene_Expression_Regulation` | Molecular Biology | mechanism | `Bio::Transcription`, `Bio::Translation` | `Bio_Biochem` | 3-5 | Operons, transcription factors, epigenetics, RNA regulation. |
| `Bio::Biotechnology` | Molecular Biology | application | `Bio::DNA`, `Bio::Genetics` | `Bio_Biochem`; `Chem_Phys` (minor, lab techniques) (verify) | 3-5 | Reuse (draft KC). PCR, cloning, sequencing, gels, blotting. |

## Cluster B: Genetics & Heredity

| KC id | Cluster | Type | Prerequisites (prereq -> this KC) | MCAT sections | Difficulty | Notes |
|---|---|---|---|---|---|---|
| `Bio::Genetics` | Genetics & Heredity | mechanism | `Bio::DNA` | `Bio_Biochem` | 2-4 | Reuse (demo KC). Genes, alleles, mutation types, chromosomal basis. |
| `Bio::Meiosis` | Genetics & Heredity | mechanism | `Bio::Cell_Cycle_and_Mitosis` | `Bio_Biochem` | 2-4 | Gamete formation, crossing over, independent assortment, nondisjunction. |
| `Bio::Mendelian_Genetics` | Genetics & Heredity | application | `Bio::Genetics`, `Bio::Meiosis` | `Bio_Biochem` | 2-5 | Inheritance patterns, Punnett squares, pedigrees, linkage/recombination. |

## Cluster C: Cell Biology

| KC id | Cluster | Type | Prerequisites (prereq -> this KC) | MCAT sections | Difficulty | Notes |
|---|---|---|---|---|---|---|
| `Bio::Eukaryotic_Cell` | Cell Biology | foundation | (none) | `Bio_Biochem` | 1-3 | Reuse (demo KC). Organelles, endomembrane system, compartmentalization. |
| `Bio::Prokaryotes_vs_Eukaryotes` | Cell Biology | foundation | `Bio::Eukaryotic_Cell` (verify) | `Bio_Biochem` | 1-2 | Reuse (draft KC). Draft treats it as an independent foundation; the edge is optional. |
| `Bio::Cell_Membrane_and_Transport` | Cell Biology | mechanism | `Bio::Eukaryotic_Cell`, `Biochem::Carbohydrates_and_Lipids` | `Bio_Biochem`; `Chem_Phys` (minor, osmosis/thermodynamics) | 2-4 | Fluid mosaic model, diffusion, osmosis, active/passive transport. |
| `Bio::Cell_Signaling` | Cell Biology | mechanism | `Bio::Cell_Membrane_and_Transport`, `Biochem::Protein_Structure_and_Function` | `Bio_Biochem`; `Psych_Soc` (minor) (verify) | 3-5 | Receptors, second messengers, signal cascades, feedback. |
| `Bio::Cytoskeleton` | Cell Biology | detail | `Bio::Eukaryotic_Cell` | `Bio_Biochem` | 2-3 | Microtubules/microfilaments, motor proteins, cilia/flagella. |
| `Bio::Cell_Cycle_and_Mitosis` | Cell Biology | mechanism | `Bio::Eukaryotic_Cell`, `Bio::DNA_Replication` | `Bio_Biochem` | 2-4 | Interphase/M-phase, checkpoints, apoptosis, cancer link. |

## Cluster D: Microbiology

| KC id | Cluster | Type | Prerequisites (prereq -> this KC) | MCAT sections | Difficulty | Notes |
|---|---|---|---|---|---|---|
| `Bio::Viruses` | Microbiology | mechanism | `Bio::DNA` (verify) | `Bio_Biochem` | 2-4 | Reuse (draft KC). Structure, lytic/lysogenic cycles, retroviruses/prions. Draft treats it as a foundation; edge is optional. |
| `Bio::Bacteria` | Microbiology | mechanism | `Bio::Prokaryotes_vs_Eukaryotes`, `Bio::Genetics` (verify) | `Bio_Biochem` | 2-4 | Growth curves, binary fission, conjugation/plasmids, antibiotic resistance. |
| `Bio::Fungi` | Microbiology | detail | `Bio::Eukaryotic_Cell` | `Bio_Biochem` | 1-3 | Lower yield (verify). Basic structure and reproductive modes. |

## Cluster E: Organ Systems & Physiology

| KC id | Cluster | Type | Prerequisites (prereq -> this KC) | MCAT sections | Difficulty | Notes |
|---|---|---|---|---|---|---|
| `Bio::Nervous_System` | Organ Systems & Physiology | application | `Bio::Cell_Membrane_and_Transport`, `GenChem::Ions_in_Solutions`, `Physics::Electrical_Circuits` | `Bio_Biochem`; `Psych_Soc` (minor); `Chem_Phys` (minor) | 3-5 | Reuse (draft KC). Neurons, resting/action potentials, synapses, CNS/PNS. Draft prereq `Bio::Eukaryotic_Cell` refined to `Bio::Cell_Membrane_and_Transport`. |
| `Bio::Endocrine_System` | Organ Systems & Physiology | application | `Bio::Cell_Signaling`, `Biochem::Protein_Structure_and_Function` | `Bio_Biochem`; `Psych_Soc` (minor) | 3-5 | Reuse (draft KC). Hormone classes, feedback loops, glands. Draft prereq `Bio::Eukaryotic_Cell` refined to `Bio::Cell_Signaling`. |
| `Bio::Muscular_System` | Organ Systems & Physiology | application | `Bio::Eukaryotic_Cell`, `Biochem::Bioenergetics` | `Bio_Biochem`; `Chem_Phys` (minor) | 2-4 | Reuse (draft KC). Sliding-filament model, fiber types, energetics. |
| `Bio::Skeletal_System` | Organ Systems & Physiology | detail | `Bio::Eukaryotic_Cell`, `GenChem::Ions_in_Solutions` | `Bio_Biochem`; `Chem_Phys` (minor) | 1-3 | Reuse (draft KC). Bone structure/remodeling, calcium homeostasis, joints. |
| `Bio::Circulatory_System` | Organ Systems & Physiology | application | `Bio::Eukaryotic_Cell`, `Physics::Fluids` | `Bio_Biochem`; `Chem_Phys` (minor) | 2-4 | Reuse (draft KC). Heart, vessels, hemodynamics, gas/nutrient transport. |
| `Bio::Respiratory_System` | Organ Systems & Physiology | application | `Bio::Eukaryotic_Cell`, `GenChem::Gas_Phase` | `Bio_Biochem`; `Chem_Phys` (minor) | 2-4 | Reuse (draft KC). Gas exchange, ventilation mechanics, control of breathing. |
| `Bio::Digestive_System` | Organ Systems & Physiology | application | `Bio::Eukaryotic_Cell`, `Biochem::Enzymes` | `Bio_Biochem` | 2-4 | Reuse (draft KC). Enzymatic digestion, absorption, accessory organs. |
| `Bio::Immune_System` | Organ Systems & Physiology | application | `Bio::Eukaryotic_Cell`, `Biochem::Protein_Structure_and_Function` | `Bio_Biochem` | 3-5 | Reuse (draft KC). Innate vs adaptive, antibodies, T/B cells. Content links to `Bio::Viruses`/`Bio::Bacteria` (not encoded as prereqs). |
| `Bio::Lymphatic_System` | Organ Systems & Physiology | detail | `Bio::Immune_System`, `Bio::Circulatory_System` | `Bio_Biochem` | 1-3 | Reuse (draft KC). Lymph flow, fluid balance, immune-cell trafficking. |
| `Bio::Skin_System` | Organ Systems & Physiology | detail | `Bio::Eukaryotic_Cell` | `Bio_Biochem` | 1-3 | Reuse (draft KC, integumentary). Barrier function, thermoregulation. |
| `Bio::Excretory_System` | Organ Systems & Physiology | application | `Bio::Circulatory_System`, `GenChem::Ions_in_Solutions`, `GenChem::Acid_Base_Equilibria` | `Bio_Biochem`; `Chem_Phys` (minor) | 3-5 | New KC (renal). Nephron function, osmoregulation, acid-base balance. |

## Cluster F: Reproduction & Development

| KC id | Cluster | Type | Prerequisites (prereq -> this KC) | MCAT sections | Difficulty | Notes |
|---|---|---|---|---|---|---|
| `Bio::Reproductive_System` | Reproduction & Development | application | `Bio::Endocrine_System`, `Bio::Meiosis` | `Bio_Biochem`; `Psych_Soc` (minor) (verify) | 2-4 | Reuse (draft KC). Gametogenesis, reproductive cycles, anatomy. Draft also cited `Bio::Genetics`. |
| `Bio::Embryology` | Reproduction & Development | application | `Bio::Reproductive_System`, `Bio::Gene_Expression_Regulation` | `Bio_Biochem`; `Psych_Soc` (minor, development) (verify) | 3-5 | Reuse (draft KC). Fertilization, cleavage, gastrulation, differentiation. Draft cited `Bio::Genetics`; refined to gene regulation. |

## Cluster G: Evolution & Diversity

| KC id | Cluster | Type | Prerequisites (prereq -> this KC) | MCAT sections | Difficulty | Notes |
|---|---|---|---|---|---|---|
| `Bio::Evolution` | Evolution & Diversity | application | `Bio::Genetics` | `Bio_Biochem`; `Psych_Soc` (minor, evolutionary behavior) (verify) | 2-4 | Reuse (draft KC). Natural selection, fitness, evidence, mechanisms of change. |
| `Bio::Population_Genetics` | Evolution & Diversity | application | `Bio::Mendelian_Genetics`, `Bio::Evolution` | `Bio_Biochem` | 3-5 | New KC. Hardy-Weinberg, allele/genotype frequencies (data-heavy). |
| `Bio::Biodiversity_and_Phylogeny` | Evolution & Diversity | detail | `Bio::Evolution` | `Bio_Biochem` | 1-3 | Lower yield (verify). Classification, phylogenetic trees, speciation. |

---

## Prerequisite edge appendix

Edges are written in the canonical `prerequisite -> target` direction from
`mcat-graph-audit.md` (stored in Rust via `add_prerequisite(target,
prerequisite)`). The graph below is acyclic; `Bio::DNA` and
`Bio::Eukaryotic_Cell` are the two Biology roots.

### Bio-internal edges

```text
Bio::DNA -> Bio::DNA_Replication
Bio::DNA -> Bio::Transcription
Bio::DNA -> Bio::Genetics
Bio::DNA -> Bio::Viruses            (verify)
Bio::DNA -> Bio::Biotechnology
Bio::DNA_Replication -> Bio::Cell_Cycle_and_Mitosis
Bio::Transcription -> Bio::Translation
Bio::Transcription -> Bio::Gene_Expression_Regulation
Bio::Translation -> Bio::Gene_Expression_Regulation
Bio::Genetics -> Bio::Biotechnology
Bio::Genetics -> Bio::Mendelian_Genetics
Bio::Genetics -> Bio::Evolution
Bio::Genetics -> Bio::Bacteria      (verify)
Bio::Eukaryotic_Cell -> Bio::Prokaryotes_vs_Eukaryotes   (verify)
Bio::Eukaryotic_Cell -> Bio::Cell_Membrane_and_Transport
Bio::Eukaryotic_Cell -> Bio::Cytoskeleton
Bio::Eukaryotic_Cell -> Bio::Cell_Cycle_and_Mitosis
Bio::Eukaryotic_Cell -> Bio::Fungi
Bio::Eukaryotic_Cell -> Bio::Muscular_System
Bio::Eukaryotic_Cell -> Bio::Skeletal_System
Bio::Eukaryotic_Cell -> Bio::Circulatory_System
Bio::Eukaryotic_Cell -> Bio::Respiratory_System
Bio::Eukaryotic_Cell -> Bio::Digestive_System
Bio::Eukaryotic_Cell -> Bio::Immune_System
Bio::Eukaryotic_Cell -> Bio::Skin_System
Bio::Cell_Membrane_and_Transport -> Bio::Cell_Signaling
Bio::Cell_Membrane_and_Transport -> Bio::Nervous_System
Bio::Cell_Signaling -> Bio::Endocrine_System
Bio::Cell_Cycle_and_Mitosis -> Bio::Meiosis
Bio::Meiosis -> Bio::Mendelian_Genetics
Bio::Meiosis -> Bio::Reproductive_System
Bio::Prokaryotes_vs_Eukaryotes -> Bio::Bacteria
Bio::Mendelian_Genetics -> Bio::Population_Genetics
Bio::Evolution -> Bio::Population_Genetics
Bio::Evolution -> Bio::Biodiversity_and_Phylogeny
Bio::Immune_System -> Bio::Lymphatic_System
Bio::Circulatory_System -> Bio::Lymphatic_System
Bio::Circulatory_System -> Bio::Excretory_System
Bio::Endocrine_System -> Bio::Reproductive_System
Bio::Gene_Expression_Regulation -> Bio::Embryology
Bio::Reproductive_System -> Bio::Embryology
```

### Cross-discipline edges (non-Bio prerequisite -> Bio target)

These reference KC ids owned by other disciplines. They must be reconciled with
the Biochem / GenChem / Physics research maps so directions stay consistent and
acyclic.

```text
Biochem::Enzymes -> Bio::DNA_Replication        (verify)
Biochem::Nucleotides_and_Nucleic_Acids -> Bio::Transcription
Biochem::Peptides_and_Proteins -> Bio::Translation
Biochem::Carbohydrates_and_Lipids -> Bio::Cell_Membrane_and_Transport
Biochem::Protein_Structure_and_Function -> Bio::Cell_Signaling
Biochem::Protein_Structure_and_Function -> Bio::Immune_System
Biochem::Bioenergetics -> Bio::Muscular_System
Biochem::Enzymes -> Bio::Digestive_System
GenChem::Ions_in_Solutions -> Bio::Nervous_System
GenChem::Ions_in_Solutions -> Bio::Skeletal_System
GenChem::Ions_in_Solutions -> Bio::Excretory_System
GenChem::Acid_Base_Equilibria -> Bio::Excretory_System
GenChem::Gas_Phase -> Bio::Respiratory_System
Physics::Electrical_Circuits -> Bio::Nervous_System
Physics::Fluids -> Bio::Circulatory_System
```

> Direction reconciliation note: `mcat.md` already authors
> `Bio::DNA -> Biochem::Nucleotides_and_Nucleic_Acids` (Bio is upstream of the
> Biochem nucleotide KC). This map then uses
> `Biochem::Nucleotides_and_Nucleic_Acids -> Bio::Transcription`. The resulting
> chain `Bio::DNA -> Biochem::Nucleotides_and_Nucleic_Acids -> Bio::Transcription`
> is acyclic. The Biochem author should keep `Biochem::Nucleotides_and_Nucleic_Acids`
> from depending on any transcription/translation KC to avoid a cycle.

---

## Research notes

### Scope

- Covers AAMC-style Biology content: molecular biology, genetics/heredity, cell
  biology, microbiology, organ systems/physiology, evolution/diversity, and
  reproduction/development. This maps most closely to Biological/Biochemical
  Foundations content categories for the living-systems and organ-systems
  topics, minus the pure biochemistry KCs (owned by the Biochem map).
- 34 KCs total across 7 clusters (A: 6, B: 3, C: 6, D: 3, E: 11, F: 2, G: 3),
  inside the requested ~25-35 range.
- Pure metabolic biochemistry (glycolysis, TCA, oxidative phosphorylation,
  enzymes, amino acids, etc.) is intentionally left to the Biochem map and only
  referenced as cross-discipline prerequisites, to avoid double-owning KCs.

### Reused ids (kept verbatim)

- Demo graph: `Bio::DNA`, `Bio::Genetics`, `Bio::Eukaryotic_Cell`.
- `mcat.md` draft lattice: `Bio::Prokaryotes_vs_Eukaryotes`, `Bio::Viruses`,
  `Bio::Evolution`, `Bio::Biotechnology`, `Bio::Nervous_System`,
  `Bio::Endocrine_System`, `Bio::Muscular_System`, `Bio::Skeletal_System`,
  `Bio::Circulatory_System`, `Bio::Respiratory_System`,
  `Bio::Digestive_System`, `Bio::Immune_System`, `Bio::Lymphatic_System`,
  `Bio::Skin_System`, `Bio::Reproductive_System`, `Bio::Embryology`.

### New ids introduced

`Bio::DNA_Replication`, `Bio::Transcription`, `Bio::Translation`,
`Bio::Gene_Expression_Regulation`, `Bio::Meiosis`, `Bio::Mendelian_Genetics`,
`Bio::Population_Genetics`, `Bio::Cell_Membrane_and_Transport`,
`Bio::Cell_Signaling`, `Bio::Cytoskeleton`, `Bio::Cell_Cycle_and_Mitosis`,
`Bio::Bacteria`, `Bio::Fungi`, `Bio::Excretory_System`,
`Bio::Biodiversity_and_Phylogeny` (15 new).

### Section-overlap assumptions

- The engine's `derived_mcat_sections_for_topics` (`qt/aqt/concept_tags.py`)
  currently routes **every** `Bio::` KC to `Bio_Biochem`, `Chem_Phys`, and
  `Psych_Soc`, and `discipline_weight` in `concept.rs` gives Biology weights of
  0.65 / 0.05 / 0.05 in those sections respectively. So at the code level all
  Bio KCs already nominally touch three sections.
- The `MCAT sections` column above is the **content-real** view: `Bio_Biochem`
  is primary for every KC, and `(minor)` marks only the KCs where a `Psych_Soc`
  or `Chem_Phys` slice is genuinely justified (e.g. nervous/endocrine ->
  Psych_Soc biological-basis-of-behavior; fluids/gas/renal/membrane ->
  Chem_Phys). If per-card `MCAT::` tags are authored more precisely than the
  blanket mapping, this column is the recommendation.

### Prereq deviations from the `mcat.md` draft (for reconciliation)

- `Bio::Nervous_System`: refined draft prereq `Bio::Eukaryotic_Cell` to
  `Bio::Cell_Membrane_and_Transport` (membrane potentials). Transitively still
  covers the cell foundation.
- `Bio::Endocrine_System`: refined draft prereq `Bio::Eukaryotic_Cell` to
  `Bio::Cell_Signaling` (hormone signal transduction).
- `Bio::Reproductive_System`: draft used `Bio::Genetics` + `Bio::Endocrine_System`;
  changed to `Bio::Meiosis` + `Bio::Endocrine_System` (gametogenesis is the
  tighter dependency; meiosis is transitively downstream of the cell/DNA roots).
- `Bio::Embryology`: draft used `Bio::Genetics`; changed to
  `Bio::Gene_Expression_Regulation` (differentiation is driven by regulated
  expression).

### `(verify)` items

- `Biochem::Enzymes -> Bio::DNA_Replication`: replication enzymology is real but
  the enzyme dependency may be light for intro-level cards.
- `Bio::DNA -> Bio::Viruses` and `Bio::Eukaryotic_Cell -> Bio::Prokaryotes_vs_Eukaryotes`:
  the draft treats `Viruses` and `Prokaryotes_vs_Eukaryotes` as standalone
  foundations; the added edges are pedagogically reasonable but optional.
- `Bio::Genetics -> Bio::Bacteria`: included for bacterial genetics
  (plasmids/conjugation); drop if `Bio::Bacteria` cards stay structural.
- `Psych_Soc` minor tags on `Bio::Cell_Signaling`, `Bio::Reproductive_System`,
  `Bio::Embryology`, `Bio::Evolution`: defensible but thin; keep only if cards
  actually probe behavior/development links.
- `Bio::Fungi` and `Bio::Biodiversity_and_Phylogeny`: lower-yield on the exam;
  keep as small `detail` clusters or fold in later if card budget is tight.
