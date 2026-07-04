# Unified MCAT KC Map (Track A → Track B handoff)

One merged, machine-usable Knowledge Component (KC) map that reconciles the six
per-discipline Wave 1 research maps into a single **globally acyclic**
prerequisite graph. This is the frozen artifact Track B (card generation)
consumes; the paired schema/versioning rules live in
`added features/content-contract.md`.

- **Docs only.** This file changes no code and no other file. It does not edit
  the canonical demo graph in `rslib/src/scheduler/concept_demo.rs`; it specifies
  the target the engine should build toward (see the Content Contract for the
  engine changes that are *specified but not implemented*).
- **Sources merged:** `research-kc-biology.md`, `research-kc-biochemistry.md`,
  `research-kc-general-chemistry.md`, `research-kc-organic-chemistry.md`,
  `research-kc-physics.md`, `research-kc-psych-soc.md`, reconciled against the
  draft lattice in `mcat.md` and the demo graph in `mcat-graph-audit.md` /
  `concept_demo.rs`.
- **Synthetic/original.** No copyrighted prep text is reproduced. Judgment calls
  are marked `(verify)`.

---

## 1. Headline results

| Metric | Value |
| --- | --- |
| **Total KCs** | **172** |
| Biology (`Bio::`) | 34 |
| Biochemistry (`Biochem::`) | 25 |
| General Chemistry (`GenChem::`) | 26 |
| Organic Chemistry (`Orgo::`) | 27 |
| Physics (`Physics::`) | 26 |
| Psychology/Sociology (`PsychSoc::`) | 34 |
| **Total prerequisite edges** | **321** |
| — intra-discipline | 252 |
| — cross-discipline (firm) | 54 |
| — cross-discipline (soft / `(verify)`) | 15 |
| **Globally acyclic?** | **Yes** — full topological order over all 172 KCs exists |
| Cycles found / broken | **None** (no edge had to be dropped or redirected to break a cycle) |
| Longest-path layers | 30 (L0–L29, see §5.3) |
| Dangling cross-references after reconciliation | 0 |
| Canonical demo subgraph preserved | 10/10 KCs, 9/9 edges present, none reversed |

Because every discipline uses a distinct area prefix (`Bio::`, `Biochem::`,
`GenChem::`, `Orgo::`, `Physics::`, `PsychSoc::`), there are **no intra-id
collisions across disciplines**. All deduplication/reconciliation work was on (a)
cross-discipline prerequisite references that named a KC by a superseded id, and
(b) the 10-KC demo overlap. The 172 count therefore equals the sum of the six
per-discipline sets with no merges.

---

## 2. Conventions (frozen)

- **KC id form:** `<Area>::<Topic_in_snake_case>` with ASCII `::`. On a card the
  target concept is tagged `KC::<Area>::<Topic>` and each prerequisite
  `Prereq::<Area>::<Topic>` (parsed by
  `CardConceptMetadata::from_tags` in `rslib/src/scheduler/concept.rs`).
- **Edge direction:** authored as `prerequisite -> target` (the
  `mcat-graph-audit.md` convention). The engine stores it on the target via
  `KnowledgeGraph::add_prerequisite(target, prerequisite)`.
- **Section derivation:** `derived_mcat_sections_for_topics`
  (`qt/aqt/concept_tags.py`) currently derives sections *from the area prefix
  only* — `Bio::` → `[Bio_Biochem, Chem_Phys, Psych_Soc]`, `Biochem::` →
  `[Bio_Biochem, Chem_Phys]`, `GenChem::`/`Physics::`/`Orgo::` → `[Chem_Phys]`,
  `PsychSoc::` → `[Psych_Soc]`. The **Primary section** / **Overlap sections**
  columns below are the *content-real* recommendation for per-card `MCAT::` tags;
  `(minor)` / `*` mark thin or passage-level overlaps that are not auto-derived.
- **Difficulty band:** `Difficulty::1..5` maps to IRT `b` in `concept.rs`
  (`difficulty_to_irt_b`: 1 → −2.0 … 5 → +2.0). The band is the suggested card
  sampling range for the KC, not a hard cap.
- **Type:** `foundation | mechanism | application | detail`, driving the
  foundation-before-detail sequencing rule.
- **Status:** `demo` = one of the 10 canonical demo KCs (reused byte-exact);
  `existing` = already named in `mcat.md`; `new` = introduced by Wave 1;
  `renamed`/`merged`/`split`/`consolidated` = a `mcat.md` id that Wave 1
  restructured (see §3).

---

## 3. Cross-discipline name reconciliation

The only conflicts to resolve were cross-discipline **prerequisite references**
that pointed at a `mcat.md` id which the owning discipline's Wave 1 map renamed,
merged, or split. Each such reference is repointed to the single canonical id
below. These decisions follow the owning maps' own "name reconciliation" /
"alignment" tables; where an owning map offered a choice, the pick is marked
`(verify)`.

### 3.1 Canonical id decisions — General Chemistry

| Superseded id (`mcat.md`) | Canonical id (this map) | Action | Cross-discipline consumers repointed |
| --- | --- | --- | --- |
| `GenChem::Covalent_Bond` | `GenChem::Chemical_Bonding` | merge | `Biochem::Amino_Acids`, `Biochem::Protein_Structure_and_Function`, `Orgo::Hybridization`, `Orgo::Functional_Groups` |
| `GenChem::Molecules` | `GenChem::Chemical_Bonding` | merge | `Biochem::Nucleotides_and_Nucleic_Acids` |
| `GenChem::Molecular_Structure` | `GenChem::Molecular_Geometry` | rename | `Orgo::Hybridization` |
| `GenChem::Solubility` | `GenChem::Solutions_and_Solubility` (+ `GenChem::Solubility_Equilibria`) | split | `Orgo::Separations_and_Purifications` → `Solutions_and_Solubility` |
| `GenChem::Water` | `GenChem::Intermolecular_Forces` / `GenChem::Solutions_and_Solubility` | merge | (none) |
| `GenChem::Liquid_Phase` | `GenChem::Phases_and_Phase_Changes` | merge | (none) |

Kept verbatim (already used by cross-discipline edges, no change):
`GenChem::Gas_Phase`, `GenChem::Ions_in_Solutions`, `GenChem::Acid_Base_Equilibria`,
`GenChem::Thermochemistry`, `GenChem::Kinetics`, `GenChem::Intermolecular_Forces`,
`GenChem::Electrochemistry`, `GenChem::Stoichiometry`, `GenChem::Titration`,
`GenChem::Equilibrium`.

### 3.2 Canonical id decisions — Physics

| Superseded id (`mcat.md`) | Canonical id (this map) | Action | Cross-discipline consumers repointed |
| --- | --- | --- | --- |
| `Physics::Fluids` | `Physics::Fluid_Dynamics` (+ `Physics::Fluid_Statics`) | split | `Bio::Circulatory_System` → `Fluid_Dynamics` |
| `Physics::Energy`, `Physics::Work` | `Physics::Work_And_Energy` | merge | `GenChem::Thermochemistry` (soft) — two edges collapse to one |
| `Physics::Electronic_Structure` | `Physics::Atomic_Structure` | merge | `GenChem::Electron_Configuration` (soft) |
| `Physics::Atoms` | `Physics::Atomic_Structure` | merge | `GenChem::Nuclear_Chemistry` (soft) |
| `Physics::Nuclear_Decay` | `Physics::Nuclear_Physics` | rename | `GenChem::Nuclear_Chemistry` (soft) |
| `Physics::Light` | `Physics::Electromagnetic_Radiation` (spectra/absorption); `Geometric_Optics`/`Physical_Optics` (ray/wave optics) | split | `GenChem::Atomic_Spectra_and_Quantum`, `GenChem::Spectrophotometry` → `Electromagnetic_Radiation` `(verify)` |
| `Physics::Translational_Motion` | `Physics::Kinematics` | rename | (none) |
| `Physics::Force`, `Physics::Equilibrium` | `Physics::Newtons_Laws` / `Physics::Force_Equilibrium` | split/consolidate | (none) |
| `Physics::Optics` | `Physics::Geometric_Optics` / `Physics::Physical_Optics` | split | (none) |
| `Physics::Matter`, `Physics::Atomic_and_Chemical_Behavior` | absorbed into `Fluid_Statics`/`Thermodynamics` / `Atomic_Structure` | absorb | (none) |

Kept verbatim (already used by cross-discipline edges):
`Physics::Electrical_Circuits` (feeds `Bio::Nervous_System`), `Physics::Electrostatics`,
`Physics::Electromagnetic_Radiation`.

### 3.3 Repointed cross-discipline edges (net effect)

| Consuming KC (target) | Original reference | Repointed to | Firm/soft |
| --- | --- | --- | --- |
| `Biochem::Amino_Acids` | `GenChem::Covalent_Bond` | `GenChem::Chemical_Bonding` | firm |
| `Biochem::Protein_Structure_and_Function` | `GenChem::Covalent_Bond` | `GenChem::Chemical_Bonding` | firm |
| `Biochem::Nucleotides_and_Nucleic_Acids` | `GenChem::Molecules` | `GenChem::Chemical_Bonding` | firm |
| `Orgo::Hybridization` | `GenChem::Covalent_Bond`, `GenChem::Molecular_Structure` | `GenChem::Chemical_Bonding`, `GenChem::Molecular_Geometry` | firm |
| `Orgo::Functional_Groups` | `GenChem::Covalent_Bond` | `GenChem::Chemical_Bonding` | firm |
| `Orgo::Separations_and_Purifications` | `GenChem::Solubility` | `GenChem::Solutions_and_Solubility` | firm |
| `Bio::Circulatory_System` | `Physics::Fluids` | `Physics::Fluid_Dynamics` | firm |
| `GenChem::Electron_Configuration` | `Physics::Electronic_Structure` | `Physics::Atomic_Structure` | soft `(verify)` |
| `GenChem::Atomic_Spectra_and_Quantum` | `Physics::Light` (+ `Physics::Electromagnetic_Radiation`) | `Physics::Electromagnetic_Radiation` | soft `(verify)` |
| `GenChem::Thermochemistry` | `Physics::Energy`, `Physics::Work` | `Physics::Work_And_Energy` | soft `(verify)` |
| `GenChem::Spectrophotometry` | `Physics::Light` | `Physics::Electromagnetic_Radiation` | soft `(verify)` |
| `GenChem::Nuclear_Chemistry` | `Physics::Nuclear_Decay`, `Physics::Atoms` | `Physics::Nuclear_Physics`, `Physics::Atomic_Structure` | soft `(verify)` |

### 3.4 Deliberate non-merges (overlapping content, distinct ids)

These pairs cover related content but are intentionally kept as **separate KCs**
(the owning maps chose distinct ownership); do not merge them:

- `Biochem::Membranes_and_Transport` vs `Bio::Cell_Membrane_and_Transport` —
  biochemical (lipid bilayer/transport-protein) emphasis vs cell-biology
  emphasis. `(verify)` dedupe if a single membrane KC is ever preferred.
- `Orgo::Carbohydrate_Chemistry` / `Orgo::Amino_Acid_and_Peptide_Chemistry` vs
  `Biochem::Carbohydrates_and_Lipids` / `Biochem::Amino_Acids` — organic-reaction
  view vs biomolecule-structure view (overlap, not a hard prereq either way).
- `Physics::Atomic_Structure` vs `GenChem::Electron_Configuration` /
  `GenChem::Atomic_Spectra_and_Quantum` — shared electronic-structure content;
  kept split with soft cross-edges. `(verify)` ownership.
- `GenChem::Nuclear_Chemistry` vs `Physics::Nuclear_Physics`,
  `GenChem::Thermodynamics` vs `Physics::Thermodynamics` — coordinated via soft
  cross-edges rather than merged.

---

## 4. Canonical demo subgraph (preserved byte-exact)

The 10 demo KCs and 9 demo edges from `concept_demo.rs` (`DEMO_KCS`,
`DEMO_EDGES`) are preserved verbatim and are a subgraph of this map. All 9 demo
edges are present and **none are reversed**:

```text
Bio::DNA -> Bio::Genetics
Biochem::Amino_Acids -> Biochem::Peptides_and_Proteins
Biochem::Peptides_and_Proteins -> Biochem::Protein_Structure_and_Function
Biochem::Protein_Structure_and_Function -> Biochem::Enzymes
Biochem::Enzymes -> Biochem::Bioenergetics
Biochem::Bioenergetics -> Biochem::Glycolysis
Biochem::Bioenergetics -> Biochem::Citric_Acid_Cycle
Biochem::Glycolysis -> Biochem::Citric_Acid_Cycle
Bio::Eukaryotic_Cell -> Biochem::Bioenergetics
```

> **Additive-only note (integration flag).** The unified map adds *new upstream*
> prerequisites to some demo KCs (e.g. `Biochem::Amino_Acids` gains
> `GenChem::Acid_Base_Equilibria`, `GenChem::Chemical_Bonding`,
> `Orgo::Functional_Groups`; `Biochem::Bioenergetics` gains
> `GenChem::Thermochemistry`). These are additive and never reverse a demo edge,
> but they mean the current demo cards' `Prereq::` tags (which the Rust test
> `demo_card_prereq_tags_match_canonical_graph_edges` pins to the 10-KC graph)
> will not match the unified graph. See the Content Contract §"Engine changes"
> for how to handle this (regenerate demo prereq tags, or keep the 10-KC demo as
> a separate frozen fixture).

---

## 5. Acyclicity verification

### 5.1 Method

The union of all 321 edges (252 intra + 54 firm cross + 15 soft/`(verify)` cross)
over all 172 nodes was run through a Kahn topological sort and an independent
longest-path layering. This mirrors the engine's own guard
`KnowledgeGraph::cycle()` (a DFS three-color cycle detector) in
`rslib/src/scheduler/concept.rs`, which the status builder calls to set
`ConceptGraph.has_cycle`.

### 5.2 Result

- **Acyclic: yes.** A complete topological order of all 172 KCs exists (Kahn
  emptied the queue with 0 nodes left in-cycle).
- **0 dangling references** — every prerequisite id referenced by an edge is a
  node in the map (after the §3 reconciliation).
- **No cycles had to be broken.** No edge was dropped or redirected for
  acyclicity. The reconciliation in §3 removes *dangling references*, not cycles.

**At-risk mutual couplings that were explicitly checked (all safe):**

- **Bio ↔ Biochem** (the case the task called out): Bio depends on Biochem
  (`Biochem::Nucleotides_and_Nucleic_Acids -> Bio::Transcription`, etc.) *and*
  Biochem depends on Bio (`Bio::DNA -> Biochem::Nucleotides_and_Nucleic_Acids`,
  `Bio::Eukaryotic_Cell -> Biochem::Bioenergetics`,
  `Bio::Endocrine_System -> Biochem::Hormonal_Regulation_of_Metabolism`). No node
  is on both sides of a loop: e.g. `Bio::DNA -> Biochem::Nucleotides_and_Nucleic_Acids
  -> Bio::Transcription` is a forward chain (`Bio::DNA` is a root; `Bio::Transcription`
  never reaches back to `Bio::DNA`). Every Biochem→Bio target
  (`Bio::Translation`, `Bio::Cell_Signaling`, `Bio::Muscular_System`, …) is a
  Bio sink relative to the Biochem foundations it uses.
- **GenChem ↔ Physics:** GenChem feeds Physics
  (`GenChem::Thermochemistry -> Physics::Thermodynamics` `(verify)`,
  `GenChem::Gas_Phase -> Physics::Gas_Exchange_And_Respiration_Physics`,
  `GenChem::Ions_in_Solutions -> Physics::Bioelectricity`) and Physics feeds
  GenChem via the soft edges in §3.3. The Physics targets of GenChem
  (`Thermodynamics`, `Gas_Exchange…`, `Bioelectricity`) are **not** ancestors of
  any Physics node that feeds GenChem back (`Atomic_Structure`,
  `Electromagnetic_Radiation`, `Work_And_Energy`, `Electrostatics`,
  `Electrical_Circuits`, `Nuclear_Physics`), so the coupling is layered, not
  circular. Confirmed by the successful topological sort.

If the soft `(verify)` GenChem↔Physics edges are later judged to add scheduling
friction, they can be demoted to lesson-only "related KC" links without affecting
the firm graph (they are already isolated in §7's soft block).

### 5.3 Layer listing (proof of acyclicity)

Longest-path layering: every edge points from a lower layer to a strictly higher
layer, so no back edges exist. 30 layers, 172 KCs.

- **L0** (6): `Bio::DNA`, `Bio::Eukaryotic_Cell`, `GenChem::Atomic_Structure`, `Physics::Units_And_Measurement`, `PsychSoc::Culture`, `PsychSoc::Social_Theory`
- **L1** (10): `Bio::Cytoskeleton`, `Bio::Fungi`, `Bio::Genetics`, `Bio::Prokaryotes_vs_Eukaryotes`, `Bio::Skin_System`, `Bio::Viruses`, `Physics::Kinematics`, `PsychSoc::Social_Institutions`, `PsychSoc::Socialization`, `PsychSoc::Stratification`
- **L2** (6): `Bio::Bacteria`, `Bio::Biotechnology`, `Bio::Evolution`, `Physics::Newtons_Laws`, `PsychSoc::Demographics`, `PsychSoc::Social_Class`
- **L3** (7): `Bio::Biodiversity_and_Phylogeny`, `Physics::Force_Equilibrium`, `Physics::Momentum_And_Impulse`, `Physics::Work_And_Energy`, `PsychSoc::Poverty`, `PsychSoc::Social_Inequality`, `PsychSoc::Social_Mobility`
- **L4** (5): `Physics::Electrostatics`, `Physics::Fluid_Statics`, `Physics::Periodic_Motion`, `Physics::Rotational_Motion`, `PsychSoc::Health_Disparities`
- **L5** (5): `Physics::Electrical_Circuits`, `Physics::Fluid_Dynamics`, `Physics::Magnetism`, `Physics::Waves`, `PsychSoc::Healthcare_Disparities`
- **L6** (5): `Bio::Circulatory_System`, `Physics::Circuit_Elements`, `Physics::Circulatory_Hemodynamics`, `Physics::Electromagnetic_Radiation`, `Physics::Sound`
- **L7** (2): `Physics::Atomic_Structure`, `Physics::Geometric_Optics`
- **L8** (4): `GenChem::Electron_Configuration`, `Physics::Nuclear_Physics`, `Physics::Optics_Of_The_Eye`, `Physics::Physical_Optics`
- **L9** (3): `GenChem::Atomic_Spectra_and_Quantum`, `GenChem::Nuclear_Chemistry`, `GenChem::Periodic_Trends`
- **L10** (1): `GenChem::Chemical_Bonding`
- **L11** (3): `Biochem::Nucleotides_and_Nucleic_Acids`, `GenChem::Molecular_Geometry`, `GenChem::Stoichiometry`
- **L12** (5): `Bio::Transcription`, `GenChem::Intermolecular_Forces`, `GenChem::Reaction_Types`, `GenChem::Thermochemistry`, `Orgo::Hybridization`
- **L13** (5): `GenChem::Kinetics`, `GenChem::Redox_Reactions`, `GenChem::Solutions_and_Solubility`, `Orgo::Functional_Groups`, `Physics::Thermodynamics`
- **L14** (8): `GenChem::Equilibrium`, `GenChem::Gas_Phase`, `GenChem::Ions_in_Solutions`, `GenChem::Spectrophotometry`, `GenChem::Thermodynamics`, `Orgo::IR_Spectroscopy`, `Orgo::NMR_Spectroscopy`, `Orgo::Nomenclature`
- **L15** (9): `Bio::Respiratory_System`, `Bio::Skeletal_System`, `GenChem::Acid_Base_Equilibria`, `GenChem::Electrochemistry`, `GenChem::Phases_and_Phase_Changes`, `GenChem::Solubility_Equilibria`, `Orgo::Isomerism`, `Physics::Bioelectricity`, `Physics::Gas_Exchange_And_Respiration_Physics`
- **L16** (7): `Bio::Excretory_System`, `Biochem::Amino_Acids`, `GenChem::Buffers`, `GenChem::Colligative_Properties`, `Orgo::Acid_Base_Reactions`, `Orgo::Polycyclic_and_Heterocyclic_Aromatic_Compounds`, `Orgo::Stereochemistry`
- **L17** (4): `GenChem::Titration`, `Orgo::Molecular_Structure_and_Absorption_Spectra`, `Orgo::Reaction_Mechanisms_Overview`, `Orgo::Separations_and_Purifications`
- **L18** (4): `Orgo::Mass_Spectrometry`, `Orgo::Nucleophilic_Addition`, `Orgo::Nucleophilic_Substitution`, `Orgo::Oxidation_Reduction_Reactions`
- **L19** (7): `Orgo::Alcohols`, `Orgo::Aldehydes_and_Ketones`, `Orgo::Amines`, `Orgo::Carboxylic_Acids`, `Orgo::Elimination_Reactions`, `Orgo::Laboratory_Techniques`, `Orgo::Nucleophilic_Acyl_Substitution`
- **L20** (5): `Biochem::Carbohydrates_and_Lipids`, `Orgo::Acid_Derivatives`, `Orgo::Amino_Acid_and_Peptide_Chemistry`, `Orgo::Carbohydrate_Chemistry`, `Orgo::Phenols`
- **L21** (2): `Bio::Cell_Membrane_and_Transport`, `Biochem::Peptides_and_Proteins`
- **L22** (5): `Bio::Nervous_System`, `Bio::Translation`, `Biochem::Chromatography_and_Separations`, `Biochem::Electrophoresis_and_Immunoassays`, `Biochem::Protein_Structure_and_Function`
- **L23** (8): `Bio::Cell_Signaling`, `Bio::Gene_Expression_Regulation`, `Bio::Immune_System`, `Biochem::Enzymes`, `Biochem::Membranes_and_Transport`, `Biochem::Nonenzymatic_Protein_Function`, `Biochem::Protein_Folding_and_Stability`, `PsychSoc::Sensory_Processing`
- **L24** (9): `Bio::DNA_Replication`, `Bio::Digestive_System`, `Bio::Endocrine_System`, `Bio::Lymphatic_System`, `Biochem::Bioenergetics`, `Biochem::Enzyme_Kinetics`, `Biochem::Vitamins_and_Cofactors`, `PsychSoc::Attention`, `PsychSoc::The_Senses`
- **L25** (7): `Bio::Cell_Cycle_and_Mitosis`, `Bio::Muscular_System`, `Biochem::Enzyme_Inhibition`, `Biochem::Glycolysis`, `Biochem::Metabolic_Regulation`, `PsychSoc::Biological_and_Social_Factors`, `PsychSoc::Perception`
- **L26** (9): `Bio::Meiosis`, `Biochem::Citric_Acid_Cycle`, `Biochem::Gluconeogenesis`, `Biochem::Glycogen_Metabolism`, `Biochem::Pentose_Phosphate_Pathway`, `PsychSoc::Cognition`, `PsychSoc::Emotion`, `PsychSoc::Learning`, `PsychSoc::Personality`
- **L27** (12): `Bio::Mendelian_Genetics`, `Bio::Reproductive_System`, `Biochem::Amino_Acid_Metabolism`, `Biochem::Lipid_Metabolism`, `Biochem::Oxidative_Phosphorylation`, `PsychSoc::Attitudes_and_Beliefs`, `PsychSoc::Consciousness`, `PsychSoc::Language`, `PsychSoc::Memory`, `PsychSoc::Motivation`, `PsychSoc::Self_and_Identity`, `PsychSoc::Stress`
- **L28** (7): `Bio::Embryology`, `Bio::Population_Genetics`, `Biochem::Hormonal_Regulation_of_Metabolism`, `PsychSoc::Intelligence`, `PsychSoc::Psychological_Disorders`, `PsychSoc::Social_Interaction`, `PsychSoc::Stereotypes`
- **L29** (2): `PsychSoc::Group_Behavior`, `PsychSoc::Prejudice_and_Bias`

---

## 6. Unified KC table

Columns are the machine-usable Track A fields. **Prerequisites** are the exact
canonical ids (union of intra- + cross-discipline edges), computed from §7's edge
list, so they are guaranteed consistent with the acyclic graph above. `kc_code`
is the stable short code Track B uses for deterministic card IDs
(`<AREA>-<TOPIC>-<NNN>`); all 172 are unique. Per-KC prose (summary, learning
objectives, misconceptions) stays in the six `research-kc-*.md` maps — see the
Content Contract for how those feed the harness.

### Biology (Bio::) — 34 KCs

| KC id | kc_code | Cluster | Type | Primary section | Overlap sections | Difficulty | Prerequisites (canonical ids) | Status |
|---|---|---|---|---|---|---|---|---|
| `Bio::DNA` | `BIO-DNA` | Molecular Biology | foundation | Bio_Biochem | — | 1-3 | — | demo |
| `Bio::DNA_Replication` | `BIO-DNAR` | Molecular Biology | mechanism | Bio_Biochem | — | 2-4 | `Bio::DNA`, `Biochem::Enzymes` | new |
| `Bio::Transcription` | `BIO-TRA` | Molecular Biology | mechanism | Bio_Biochem | — | 2-4 | `Bio::DNA`, `Biochem::Nucleotides_and_Nucleic_Acids` | new |
| `Bio::Translation` | `BIO-TRA2` | Molecular Biology | mechanism | Bio_Biochem | — | 2-4 | `Bio::Transcription`, `Biochem::Peptides_and_Proteins` | new |
| `Bio::Gene_Expression_Regulation` | `BIO-GENER` | Molecular Biology | mechanism | Bio_Biochem | — | 3-5 | `Bio::Transcription`, `Bio::Translation` | new |
| `Bio::Biotechnology` | `BIO-BIO` | Molecular Biology | application | Bio_Biochem | Chem_Phys(minor) | 3-5 | `Bio::DNA`, `Bio::Genetics` | existing |
| `Bio::Genetics` | `BIO-GEN` | Genetics & Heredity | mechanism | Bio_Biochem | — | 2-4 | `Bio::DNA` | demo |
| `Bio::Meiosis` | `BIO-MEI` | Genetics & Heredity | mechanism | Bio_Biochem | — | 2-4 | `Bio::Cell_Cycle_and_Mitosis` | new |
| `Bio::Mendelian_Genetics` | `BIO-MENG` | Genetics & Heredity | application | Bio_Biochem | — | 2-5 | `Bio::Genetics`, `Bio::Meiosis` | new |
| `Bio::Eukaryotic_Cell` | `BIO-EUKC` | Cell Biology | foundation | Bio_Biochem | — | 1-3 | — | demo |
| `Bio::Prokaryotes_vs_Eukaryotes` | `BIO-PROE` | Cell Biology | foundation | Bio_Biochem | — | 1-2 | `Bio::Eukaryotic_Cell` | existing |
| `Bio::Cell_Membrane_and_Transport` | `BIO-CELMT` | Cell Biology | mechanism | Bio_Biochem | Chem_Phys(minor) | 2-4 | `Bio::Eukaryotic_Cell`, `Biochem::Carbohydrates_and_Lipids` | new |
| `Bio::Cell_Signaling` | `BIO-CELS` | Cell Biology | mechanism | Bio_Biochem | Psych_Soc(minor) | 3-5 | `Bio::Cell_Membrane_and_Transport`, `Biochem::Protein_Structure_and_Function` | new |
| `Bio::Cytoskeleton` | `BIO-CYT` | Cell Biology | detail | Bio_Biochem | — | 2-3 | `Bio::Eukaryotic_Cell` | new |
| `Bio::Cell_Cycle_and_Mitosis` | `BIO-CELCM` | Cell Biology | mechanism | Bio_Biochem | — | 2-4 | `Bio::DNA_Replication`, `Bio::Eukaryotic_Cell` | new |
| `Bio::Viruses` | `BIO-VIR` | Microbiology | mechanism | Bio_Biochem | — | 2-4 | `Bio::DNA` | existing |
| `Bio::Bacteria` | `BIO-BAC` | Microbiology | mechanism | Bio_Biochem | — | 2-4 | `Bio::Genetics`, `Bio::Prokaryotes_vs_Eukaryotes` | new |
| `Bio::Fungi` | `BIO-FUN` | Microbiology | detail | Bio_Biochem | — | 1-3 | `Bio::Eukaryotic_Cell` | new |
| `Bio::Nervous_System` | `BIO-NERS` | Organ Systems & Physiology | application | Bio_Biochem | Psych_Soc(minor); Chem_Phys(minor) | 3-5 | `Bio::Cell_Membrane_and_Transport`, `GenChem::Ions_in_Solutions`, `Physics::Electrical_Circuits` | existing |
| `Bio::Endocrine_System` | `BIO-ENDS` | Organ Systems & Physiology | application | Bio_Biochem | Psych_Soc(minor) | 3-5 | `Bio::Cell_Signaling` | existing |
| `Bio::Muscular_System` | `BIO-MUSS` | Organ Systems & Physiology | application | Bio_Biochem | Chem_Phys(minor) | 2-4 | `Bio::Eukaryotic_Cell`, `Biochem::Bioenergetics` | existing |
| `Bio::Skeletal_System` | `BIO-SKES` | Organ Systems & Physiology | detail | Bio_Biochem | Chem_Phys(minor) | 1-3 | `Bio::Eukaryotic_Cell`, `GenChem::Ions_in_Solutions` | existing |
| `Bio::Circulatory_System` | `BIO-CIRS` | Organ Systems & Physiology | application | Bio_Biochem | Chem_Phys(minor) | 2-4 | `Bio::Eukaryotic_Cell`, `Physics::Fluid_Dynamics` | existing |
| `Bio::Respiratory_System` | `BIO-RESS` | Organ Systems & Physiology | application | Bio_Biochem | Chem_Phys(minor) | 2-4 | `Bio::Eukaryotic_Cell`, `GenChem::Gas_Phase` | existing |
| `Bio::Digestive_System` | `BIO-DIGS` | Organ Systems & Physiology | application | Bio_Biochem | — | 2-4 | `Bio::Eukaryotic_Cell`, `Biochem::Enzymes` | existing |
| `Bio::Immune_System` | `BIO-IMMS` | Organ Systems & Physiology | application | Bio_Biochem | — | 3-5 | `Bio::Eukaryotic_Cell`, `Biochem::Protein_Structure_and_Function` | existing |
| `Bio::Lymphatic_System` | `BIO-LYMS` | Organ Systems & Physiology | detail | Bio_Biochem | — | 1-3 | `Bio::Circulatory_System`, `Bio::Immune_System` | existing |
| `Bio::Skin_System` | `BIO-SKIS` | Organ Systems & Physiology | detail | Bio_Biochem | — | 1-3 | `Bio::Eukaryotic_Cell` | existing |
| `Bio::Excretory_System` | `BIO-EXCS` | Organ Systems & Physiology | application | Bio_Biochem | Chem_Phys(minor) | 3-5 | `Bio::Circulatory_System`, `GenChem::Acid_Base_Equilibria`, `GenChem::Ions_in_Solutions` | new |
| `Bio::Reproductive_System` | `BIO-REPS` | Reproduction & Development | application | Bio_Biochem | Psych_Soc(minor) | 2-4 | `Bio::Endocrine_System`, `Bio::Meiosis` | existing |
| `Bio::Embryology` | `BIO-EMB` | Reproduction & Development | application | Bio_Biochem | Psych_Soc(minor) | 3-5 | `Bio::Gene_Expression_Regulation`, `Bio::Reproductive_System` | existing |
| `Bio::Evolution` | `BIO-EVO` | Evolution & Diversity | application | Bio_Biochem | Psych_Soc(minor) | 2-4 | `Bio::Genetics` | existing |
| `Bio::Population_Genetics` | `BIO-POPG` | Evolution & Diversity | application | Bio_Biochem | — | 3-5 | `Bio::Evolution`, `Bio::Mendelian_Genetics` | new |
| `Bio::Biodiversity_and_Phylogeny` | `BIO-BIOP` | Evolution & Diversity | detail | Bio_Biochem | — | 1-3 | `Bio::Evolution` | new |

### Biochemistry (Biochem::) — 25 KCs

| KC id | kc_code | Cluster | Type | Primary section | Overlap sections | Difficulty | Prerequisites (canonical ids) | Status |
|---|---|---|---|---|---|---|---|---|
| `Biochem::Amino_Acids` | `BCH-AMIA` | Amino Acids & Proteins | foundation | Bio_Biochem | Chem_Phys | 1-3 | `GenChem::Acid_Base_Equilibria`, `GenChem::Chemical_Bonding`, `Orgo::Functional_Groups` | demo |
| `Biochem::Peptides_and_Proteins` | `BCH-PEPP` | Amino Acids & Proteins | foundation | Bio_Biochem | Chem_Phys | 1-3 | `Biochem::Amino_Acids`, `Orgo::Acid_Derivatives` | demo |
| `Biochem::Protein_Structure_and_Function` | `BCH-PROSF` | Amino Acids & Proteins | mechanism | Bio_Biochem | Chem_Phys | 2-4 | `Biochem::Peptides_and_Proteins`, `GenChem::Chemical_Bonding`, `GenChem::Intermolecular_Forces` | demo |
| `Biochem::Protein_Folding_and_Stability` | `BCH-PROFS` | Amino Acids & Proteins | mechanism | Chem_Phys | Bio_Biochem | 3-5 | `Biochem::Protein_Structure_and_Function`, `GenChem::Intermolecular_Forces`, `GenChem::Thermochemistry` | new |
| `Biochem::Nonenzymatic_Protein_Function` | `BCH-NONPF` | Amino Acids & Proteins | application | Bio_Biochem | Chem_Phys | 3-5 | `Biochem::Protein_Structure_and_Function` | new |
| `Biochem::Enzymes` | `BCH-ENZ` | Enzymes | mechanism | Bio_Biochem | Chem_Phys | 2-4 | `Biochem::Protein_Structure_and_Function` | demo |
| `Biochem::Enzyme_Kinetics` | `BCH-ENZK` | Enzymes | mechanism | Chem_Phys | Bio_Biochem | 3-5 | `Biochem::Enzymes`, `GenChem::Kinetics` | new |
| `Biochem::Enzyme_Inhibition` | `BCH-ENZI` | Enzymes | mechanism | Chem_Phys | Bio_Biochem | 3-5 | `Biochem::Enzyme_Kinetics` | new |
| `Biochem::Bioenergetics` | `BCH-BIO` | Bioenergetics & Regulation | mechanism | Chem_Phys | Bio_Biochem | 2-4 | `Bio::Eukaryotic_Cell`, `Biochem::Enzymes`, `GenChem::Thermochemistry` | demo |
| `Biochem::Metabolic_Regulation` | `BCH-METR` | Bioenergetics & Regulation | mechanism | Bio_Biochem | Chem_Phys | 3-5 | `Biochem::Bioenergetics`, `Biochem::Enzymes` | existing |
| `Biochem::Carbohydrates_and_Lipids` | `BCH-CARL` | Carbohydrates & Lipids (structure) | foundation | Bio_Biochem | Chem_Phys | 1-3 | `Orgo::Aldehydes_and_Ketones`, `Orgo::Functional_Groups`, `Orgo::Stereochemistry` | existing |
| `Biochem::Glycolysis` | `BCH-GLY` | Carbohydrate Metabolism | mechanism | Bio_Biochem | Chem_Phys | 2-4 | `Biochem::Bioenergetics`, `Biochem::Carbohydrates_and_Lipids` | demo |
| `Biochem::Gluconeogenesis` | `BCH-GLU` | Carbohydrate Metabolism | mechanism | Bio_Biochem | Chem_Phys | 3-5 | `Biochem::Glycolysis`, `Biochem::Metabolic_Regulation` | existing |
| `Biochem::Glycogen_Metabolism` | `BCH-GLYM` | Carbohydrate Metabolism | mechanism | Bio_Biochem | Chem_Phys | 3-5 | `Biochem::Carbohydrates_and_Lipids`, `Biochem::Glycolysis`, `Biochem::Metabolic_Regulation` | new |
| `Biochem::Pentose_Phosphate_Pathway` | `BCH-PENPP` | Carbohydrate Metabolism | detail | Bio_Biochem | Chem_Phys | 3-5 | `Biochem::Glycolysis`, `Biochem::Nucleotides_and_Nucleic_Acids` | existing |
| `Biochem::Citric_Acid_Cycle` | `BCH-CITAC` | Citric Acid Cycle & OxPhos | mechanism | Bio_Biochem | Chem_Phys | 2-4 | `Biochem::Bioenergetics`, `Biochem::Glycolysis` | demo |
| `Biochem::Oxidative_Phosphorylation` | `BCH-OXIP` | Citric Acid Cycle & OxPhos | mechanism | Chem_Phys | Bio_Biochem | 3-5 | `Bio::Eukaryotic_Cell`, `Biochem::Citric_Acid_Cycle`, `GenChem::Electrochemistry` | existing |
| `Biochem::Lipid_Metabolism` | `BCH-LIPM` | Lipid & Amino Acid Metabolism | mechanism | Bio_Biochem | Chem_Phys | 3-5 | `Biochem::Bioenergetics`, `Biochem::Carbohydrates_and_Lipids`, `Biochem::Citric_Acid_Cycle` | existing |
| `Biochem::Amino_Acid_Metabolism` | `BCH-AMIAM` | Lipid & Amino Acid Metabolism | detail | Bio_Biochem | Chem_Phys | 3-5 | `Biochem::Amino_Acids`, `Biochem::Citric_Acid_Cycle`, `Biochem::Metabolic_Regulation` | new |
| `Biochem::Nucleotides_and_Nucleic_Acids` | `BCH-NUCNA` | Nucleic Acids | foundation | Bio_Biochem | Chem_Phys | 1-3 | `Bio::DNA`, `GenChem::Chemical_Bonding` | existing |
| `Biochem::Membranes_and_Transport` | `BCH-MEMT` | Membranes & Transport | mechanism | Bio_Biochem | Chem_Phys | 2-4 | `Bio::Eukaryotic_Cell`, `Biochem::Carbohydrates_and_Lipids`, `Biochem::Protein_Structure_and_Function` | new |
| `Biochem::Chromatography_and_Separations` | `BCH-CHRS` | Laboratory Techniques | application | Chem_Phys | Bio_Biochem | 3-5 | `Biochem::Peptides_and_Proteins`, `GenChem::Intermolecular_Forces`, `Orgo::Separations_and_Purifications` | new |
| `Biochem::Electrophoresis_and_Immunoassays` | `BCH-ELEI` | Laboratory Techniques | application | Chem_Phys | Bio_Biochem | 3-5 | `Biochem::Nucleotides_and_Nucleic_Acids`, `Biochem::Peptides_and_Proteins`, `GenChem::Acid_Base_Equilibria` | new |
| `Biochem::Vitamins_and_Cofactors` | `BCH-VITC` | Cofactors & Integration | detail | Bio_Biochem | Chem_Phys | 2-4 | `Biochem::Enzymes` | new |
| `Biochem::Hormonal_Regulation_of_Metabolism` | `BCH-HORRM` | Cofactors & Integration | application | Bio_Biochem | Chem_Phys | 3-5 | `Bio::Endocrine_System`, `Biochem::Gluconeogenesis`, `Biochem::Lipid_Metabolism`, `Biochem::Metabolic_Regulation` | new |

### General Chemistry (GenChem::) — 26 KCs

| KC id | kc_code | Cluster | Type | Primary section | Overlap sections | Difficulty | Prerequisites (canonical ids) | Status |
|---|---|---|---|---|---|---|---|---|
| `GenChem::Atomic_Structure` | `GCH-ATOS` | Atomic Structure & Periodicity | foundation | Chem_Phys | — | 1-3 | — | new |
| `GenChem::Electron_Configuration` | `GCH-ELEC` | Atomic Structure & Periodicity | foundation | Chem_Phys | — | 2-4 | `GenChem::Atomic_Structure`, `Physics::Atomic_Structure` | new |
| `GenChem::Atomic_Spectra_and_Quantum` | `GCH-ATOSQ` | Atomic Structure & Periodicity | detail | Chem_Phys | — | 3-5 | `GenChem::Electron_Configuration`, `Physics::Electromagnetic_Radiation` | new |
| `GenChem::Periodic_Trends` | `GCH-PERT` | Atomic Structure & Periodicity | foundation | Chem_Phys | — | 1-4 | `GenChem::Electron_Configuration` | new |
| `GenChem::Chemical_Bonding` | `GCH-CHEB` | Bonding & Molecular Structure | foundation | Chem_Phys | +Bio_Biochem | 1-3 | `GenChem::Periodic_Trends` | merged |
| `GenChem::Molecular_Geometry` | `GCH-MOLG` | Bonding & Molecular Structure | mechanism | Chem_Phys | +Bio_Biochem | 2-4 | `GenChem::Chemical_Bonding` | renamed |
| `GenChem::Stoichiometry` | `GCH-STO` | Stoichiometry & Reactions | foundation | Chem_Phys | +Bio_Biochem | 1-4 | `GenChem::Chemical_Bonding` | existing |
| `GenChem::Reaction_Types` | `GCH-REAT` | Stoichiometry & Reactions | foundation | Chem_Phys | — | 2-4 | `GenChem::Stoichiometry` | new |
| `GenChem::Redox_Reactions` | `GCH-REDR` | Stoichiometry & Reactions | mechanism | Chem_Phys | +Bio_Biochem | 2-4 | `GenChem::Reaction_Types` | new |
| `GenChem::Gas_Phase` | `GCH-GASP` | Gases, Phases & IMF | mechanism | Chem_Phys | +Bio_Biochem | 2-5 | `GenChem::Stoichiometry`, `Physics::Thermodynamics` | existing |
| `GenChem::Intermolecular_Forces` | `GCH-INTF` | Gases, Phases & IMF | mechanism | Chem_Phys | +Bio_Biochem | 2-4 | `GenChem::Molecular_Geometry` | existing |
| `GenChem::Phases_and_Phase_Changes` | `GCH-PHAPC` | Gases, Phases & IMF | mechanism | Chem_Phys | — | 2-5 | `GenChem::Gas_Phase`, `GenChem::Intermolecular_Forces` | merged |
| `GenChem::Solutions_and_Solubility` | `GCH-SOLS` | Solutions | foundation | Chem_Phys | +Bio_Biochem | 2-4 | `GenChem::Intermolecular_Forces` | merged |
| `GenChem::Ions_in_Solutions` | `GCH-IONS` | Solutions | mechanism | Chem_Phys | +Bio_Biochem | 2-4 | `GenChem::Solutions_and_Solubility` | existing |
| `GenChem::Spectrophotometry` | `GCH-SPE` | Solutions | detail | Chem_Phys | +Bio_Biochem | 3-5 | `GenChem::Solutions_and_Solubility`, `Physics::Electromagnetic_Radiation` | new |
| `GenChem::Colligative_Properties` | `GCH-COLP` | Solutions | application | Chem_Phys | +Bio_Biochem | 3-5 | `GenChem::Phases_and_Phase_Changes`, `GenChem::Solutions_and_Solubility` | new |
| `GenChem::Solubility_Equilibria` | `GCH-SOLE` | Solutions | detail | Chem_Phys | — | 3-5 | `GenChem::Equilibrium`, `GenChem::Ions_in_Solutions` | split |
| `GenChem::Thermochemistry` | `GCH-THE` | Thermodynamics | mechanism | Chem_Phys | +Bio_Biochem | 2-5 | `GenChem::Stoichiometry`, `Physics::Work_And_Energy` | existing |
| `GenChem::Thermodynamics` | `GCH-THE2` | Thermodynamics | mechanism | Chem_Phys | +Bio_Biochem | 3-5 | `GenChem::Thermochemistry`, `Physics::Thermodynamics` | new |
| `GenChem::Kinetics` | `GCH-KIN` | Kinetics | mechanism | Chem_Phys | +Bio_Biochem | 3-5 | `GenChem::Stoichiometry`, `GenChem::Thermochemistry` | existing |
| `GenChem::Equilibrium` | `GCH-EQU` | Equilibrium | mechanism | Chem_Phys | +Bio_Biochem | 3-5 | `GenChem::Kinetics` | existing |
| `GenChem::Acid_Base_Equilibria` | `GCH-ACIBE` | Acids & Bases | mechanism | Chem_Phys | +Bio_Biochem | 2-5 | `GenChem::Equilibrium` | existing |
| `GenChem::Buffers` | `GCH-BUF` | Acids & Bases | application | Chem_Phys | +Bio_Biochem | 3-5 | `GenChem::Acid_Base_Equilibria` | new |
| `GenChem::Titration` | `GCH-TIT` | Acids & Bases | application | Chem_Phys | +Bio_Biochem | 3-5 | `GenChem::Acid_Base_Equilibria`, `GenChem::Buffers` | existing |
| `GenChem::Electrochemistry` | `GCH-ELE` | Electrochemistry | application | Chem_Phys | +Bio_Biochem | 3-5 | `GenChem::Redox_Reactions`, `GenChem::Thermodynamics`, `Physics::Electrical_Circuits`, `Physics::Electrostatics` | existing |
| `GenChem::Nuclear_Chemistry` | `GCH-NUCC` | Nuclear | detail | Chem_Phys | — | 2-4 | `GenChem::Atomic_Structure`, `Physics::Atomic_Structure`, `Physics::Nuclear_Physics` | new |

### Organic Chemistry (Orgo::) — 27 KCs

| KC id | kc_code | Cluster | Type | Primary section | Overlap sections | Difficulty | Prerequisites (canonical ids) | Status |
|---|---|---|---|---|---|---|---|---|
| `Orgo::Hybridization` | `ORG-HYB` | Structure & Bonding | foundation | Chem_Phys | — | 1-3 | `GenChem::Chemical_Bonding`, `GenChem::Molecular_Geometry` | reused |
| `Orgo::Functional_Groups` | `ORG-FUNG` | Structure & Bonding | foundation | Chem_Phys | Bio_Biochem(minor) | 1-3 | `GenChem::Chemical_Bonding`, `Orgo::Hybridization` | reused |
| `Orgo::Nomenclature` | `ORG-NOM` | Structure & Bonding | foundation | Chem_Phys | — | 1-3 | `Orgo::Functional_Groups`, `Orgo::Hybridization` | reused |
| `Orgo::Isomerism` | `ORG-ISO` | Isomerism & Stereochemistry | foundation | Chem_Phys | — | 2-4 | `Orgo::Functional_Groups`, `Orgo::Nomenclature` | new |
| `Orgo::Stereochemistry` | `ORG-STE` | Isomerism & Stereochemistry | foundation | Chem_Phys | Bio_Biochem(minor) | 2-5 | `Orgo::Hybridization`, `Orgo::Isomerism` | reused |
| `Orgo::Acid_Base_Reactions` | `ORG-ACIBR` | Acid-Base & Reactivity Foundations | foundation | Chem_Phys | Bio_Biochem(minor) | 2-4 | `GenChem::Acid_Base_Equilibria`, `Orgo::Functional_Groups` | new |
| `Orgo::Reaction_Mechanisms_Overview` | `ORG-REAMO` | Acid-Base & Reactivity Foundations | foundation | Chem_Phys | — | 2-4 | `GenChem::Kinetics`, `GenChem::Thermochemistry`, `Orgo::Acid_Base_Reactions`, `Orgo::Functional_Groups` | new |
| `Orgo::Nucleophilic_Substitution` | `ORG-NUCS` | Reaction Mechanisms | mechanism | Chem_Phys | — | 2-5 | `GenChem::Kinetics`, `Orgo::Reaction_Mechanisms_Overview`, `Orgo::Stereochemistry` | reused |
| `Orgo::Elimination_Reactions` | `ORG-ELIR` | Reaction Mechanisms | mechanism | Chem_Phys | — | 3-5 | `Orgo::Nucleophilic_Substitution`, `Orgo::Reaction_Mechanisms_Overview` | new |
| `Orgo::Nucleophilic_Addition` | `ORG-NUCA` | Reaction Mechanisms | mechanism | Chem_Phys | — | 2-4 | `Orgo::Acid_Base_Reactions`, `Orgo::Reaction_Mechanisms_Overview` | new |
| `Orgo::Nucleophilic_Acyl_Substitution` | `ORG-NUCAS` | Reaction Mechanisms | mechanism | Chem_Phys | Bio_Biochem(minor) | 3-5 | `Orgo::Acid_Base_Reactions`, `Orgo::Nucleophilic_Addition` | new |
| `Orgo::Oxidation_Reduction_Reactions` | `ORG-OXIRR` | Reaction Mechanisms | mechanism | Chem_Phys | Bio_Biochem(minor) | 2-4 | `GenChem::Electrochemistry`, `Orgo::Functional_Groups`, `Orgo::Reaction_Mechanisms_Overview` | new |
| `Orgo::Alcohols` | `ORG-ALC` | Oxygen-Containing Functional Groups | application | Chem_Phys | Bio_Biochem(minor) | 2-4 | `Orgo::Acid_Base_Reactions`, `Orgo::Functional_Groups`, `Orgo::Nucleophilic_Substitution`, `Orgo::Oxidation_Reduction_Reactions` | reused |
| `Orgo::Aldehydes_and_Ketones` | `ORG-ALDK` | Oxygen-Containing Functional Groups | application | Chem_Phys | Bio_Biochem(minor) | 2-5 | `Orgo::Functional_Groups`, `Orgo::Nucleophilic_Addition`, `Orgo::Oxidation_Reduction_Reactions` | reused |
| `Orgo::Carboxylic_Acids` | `ORG-CARA` | Oxygen-Containing Functional Groups | application | Chem_Phys | Bio_Biochem(minor) | 2-4 | `Orgo::Acid_Base_Reactions`, `Orgo::Functional_Groups`, `Orgo::Oxidation_Reduction_Reactions` | reused |
| `Orgo::Acid_Derivatives` | `ORG-ACID` | Oxygen-Containing Functional Groups | application | Chem_Phys | Bio_Biochem(minor) | 3-5 | `Orgo::Carboxylic_Acids`, `Orgo::Nucleophilic_Acyl_Substitution` | reused |
| `Orgo::Phenols` | `ORG-PHE` | Oxygen-Containing Functional Groups | application | Chem_Phys | Bio_Biochem(minor) | 2-4 | `Orgo::Acid_Base_Reactions`, `Orgo::Alcohols`, `Orgo::Polycyclic_and_Heterocyclic_Aromatic_Compounds` | reused |
| `Orgo::Polycyclic_and_Heterocyclic_Aromatic_Compounds` | `ORG-POLHAC` | Nitrogen & Aromatic Compounds | application | Chem_Phys | Bio_Biochem(minor) | 2-4 | `Orgo::Functional_Groups`, `Orgo::Hybridization`, `Orgo::Isomerism` | reused |
| `Orgo::Amines` | `ORG-AMI` | Nitrogen & Aromatic Compounds | application | Chem_Phys | Bio_Biochem(minor) | 2-4 | `Orgo::Acid_Base_Reactions`, `Orgo::Functional_Groups`, `Orgo::Nucleophilic_Substitution` | new |
| `Orgo::Carbohydrate_Chemistry` | `ORG-CARC` | Biological Molecule Chemistry | application | Chem_Phys | Bio_Biochem | 2-4 | `Orgo::Alcohols`, `Orgo::Nucleophilic_Addition`, `Orgo::Stereochemistry` | new |
| `Orgo::Amino_Acid_and_Peptide_Chemistry` | `ORG-AMIAPC` | Biological Molecule Chemistry | application | Chem_Phys | Bio_Biochem | 2-4 | `Orgo::Acid_Base_Reactions`, `Orgo::Amines`, `Orgo::Nucleophilic_Acyl_Substitution`, `Orgo::Stereochemistry` | new |
| `Orgo::IR_Spectroscopy` | `ORG-IRS` | Spectroscopy & Structure Determination | detail | Chem_Phys | Bio_Biochem(minor) | 1-3 | `Orgo::Functional_Groups`, `Physics::Electromagnetic_Radiation` | new |
| `Orgo::NMR_Spectroscopy` | `ORG-NMRS` | Spectroscopy & Structure Determination | detail | Chem_Phys | Bio_Biochem(minor) | 2-5 | `Orgo::Functional_Groups`, `Orgo::Hybridization`, `Physics::Electromagnetic_Radiation` | new |
| `Orgo::Mass_Spectrometry` | `ORG-MASS` | Spectroscopy & Structure Determination | detail | Chem_Phys | — | 2-4 | `Orgo::Functional_Groups`, `Orgo::Reaction_Mechanisms_Overview` | reused |
| `Orgo::Molecular_Structure_and_Absorption_Spectra` | `ORG-MOLSAS` | Spectroscopy & Structure Determination | detail | Chem_Phys | — | 2-4 | `Orgo::Hybridization`, `Orgo::Polycyclic_and_Heterocyclic_Aromatic_Compounds`, `Physics::Electromagnetic_Radiation` | reused |
| `Orgo::Separations_and_Purifications` | `ORG-SEPP` | Separations & Lab Techniques | application | Chem_Phys | Bio_Biochem(minor) | 1-4 | `GenChem::Intermolecular_Forces`, `GenChem::Solutions_and_Solubility`, `Orgo::Acid_Base_Reactions`, `Orgo::Functional_Groups` | reused |
| `Orgo::Laboratory_Techniques` | `ORG-LABT` | Separations & Lab Techniques | application | Chem_Phys | Bio_Biochem(minor) | 3-5 | `Orgo::IR_Spectroscopy`, `Orgo::Mass_Spectrometry`, `Orgo::NMR_Spectroscopy`, `Orgo::Separations_and_Purifications` | new |

### Physics (Physics::) — 26 KCs

| KC id | kc_code | Cluster | Type | Primary section | Overlap sections | Difficulty | Prerequisites (canonical ids) | Status |
|---|---|---|---|---|---|---|---|---|
| `Physics::Units_And_Measurement` | `PHY-UNIM` | Foundations & Math | foundation | Chem_Phys | — | 1-2 | — | new |
| `Physics::Kinematics` | `PHY-KIN` | Classical Mechanics | foundation | Chem_Phys | — | 1-3 | `Physics::Units_And_Measurement` | renamed |
| `Physics::Newtons_Laws` | `PHY-NEWL` | Classical Mechanics | mechanism | Chem_Phys | — | 1-4 | `Physics::Kinematics` | split |
| `Physics::Force_Equilibrium` | `PHY-FORE` | Classical Mechanics | mechanism | Chem_Phys | — | 2-4 | `Physics::Newtons_Laws` | consolidated |
| `Physics::Work_And_Energy` | `PHY-WORE` | Classical Mechanics | mechanism | Chem_Phys | — | 2-4 | `Physics::Newtons_Laws` | merged |
| `Physics::Momentum_And_Impulse` | `PHY-MOMI` | Classical Mechanics | mechanism | Chem_Phys | — | 2-4 | `Physics::Newtons_Laws` | new |
| `Physics::Rotational_Motion` | `PHY-ROTM` | Classical Mechanics | detail | Chem_Phys | — | 3-5 | `Physics::Force_Equilibrium`, `Physics::Work_And_Energy` | new |
| `Physics::Periodic_Motion` | `PHY-PERM` | Oscillations, Waves & Sound | mechanism | Chem_Phys | — | 3-5 | `Physics::Force_Equilibrium`, `Physics::Work_And_Energy` | existing |
| `Physics::Waves` | `PHY-WAV` | Oscillations, Waves & Sound | mechanism | Chem_Phys | — | 3-5 | `Physics::Periodic_Motion` | new |
| `Physics::Sound` | `PHY-SOU` | Oscillations, Waves & Sound | application | Chem_Phys | Bio_Biochem*; Psych_Soc* | 3-5 | `Physics::Waves` | existing |
| `Physics::Fluid_Statics` | `PHY-FLUS` | Fluids | mechanism | Chem_Phys | Bio_Biochem* | 2-4 | `Physics::Force_Equilibrium` | split |
| `Physics::Fluid_Dynamics` | `PHY-FLUD` | Fluids | mechanism | Chem_Phys | Bio_Biochem* | 3-5 | `Physics::Fluid_Statics`, `Physics::Work_And_Energy` | split |
| `Physics::Thermodynamics` | `PHY-THE` | Thermodynamics | mechanism | Chem_Phys | Bio_Biochem* | 2-4 | `GenChem::Thermochemistry`, `Physics::Work_And_Energy` | existing |
| `Physics::Electrostatics` | `PHY-ELE` | Electricity & Magnetism | mechanism | Chem_Phys | — | 2-4 | `Physics::Work_And_Energy` | existing |
| `Physics::Electrical_Circuits` | `PHY-ELEC` | Electricity & Magnetism | mechanism | Chem_Phys | Bio_Biochem* | 2-4 | `Physics::Electrostatics` | existing |
| `Physics::Circuit_Elements` | `PHY-CIRE` | Electricity & Magnetism | detail | Chem_Phys | Bio_Biochem* | 3-5 | `Physics::Electrical_Circuits` | existing |
| `Physics::Magnetism` | `PHY-MAG` | Electricity & Magnetism | detail | Chem_Phys | — | 3-5 | `Physics::Electrostatics` | existing |
| `Physics::Electromagnetic_Radiation` | `PHY-ELER` | Light & Optics | mechanism | Chem_Phys | — | 2-4 | `Physics::Magnetism`, `Physics::Waves` | existing |
| `Physics::Geometric_Optics` | `PHY-GEOO` | Light & Optics | application | Chem_Phys | Bio_Biochem* | 3-5 | `Physics::Electromagnetic_Radiation` | split |
| `Physics::Physical_Optics` | `PHY-PHYO` | Light & Optics | detail | Chem_Phys | — | 4-5 | `Physics::Geometric_Optics`, `Physics::Waves` | split |
| `Physics::Atomic_Structure` | `PHY-ATOS` | Atomic & Nuclear | mechanism | Chem_Phys | — | 3-5 | `Physics::Electromagnetic_Radiation`, `Physics::Electrostatics` | merged |
| `Physics::Nuclear_Physics` | `PHY-NUCP` | Atomic & Nuclear | detail | Chem_Phys | Bio_Biochem* | 3-5 | `Physics::Atomic_Structure` | renamed |
| `Physics::Circulatory_Hemodynamics` | `PHY-CIRH` | Biological Applications | application | Chem_Phys | Bio_Biochem* | 3-5 | `Physics::Fluid_Dynamics` | new |
| `Physics::Gas_Exchange_And_Respiration_Physics` | `PHY-GASERP` | Biological Applications | application | Chem_Phys | Bio_Biochem* | 3-5 | `GenChem::Gas_Phase`, `Physics::Fluid_Statics`, `Physics::Thermodynamics` | new |
| `Physics::Optics_Of_The_Eye` | `PHY-OPTE` | Biological Applications | application | Chem_Phys | Bio_Biochem*; Psych_Soc* | 3-5 | `Physics::Geometric_Optics` | new |
| `Physics::Bioelectricity` | `PHY-BIO` | Biological Applications | application | Chem_Phys | Bio_Biochem* | 4-5 | `GenChem::Ions_in_Solutions`, `Physics::Circuit_Elements` | new |

### Psychology / Sociology (PsychSoc::) — 34 KCs

Primary section `Psych_Soc` for all. **Sub-domain** is the proposed
psychology/sociology split for when `McatDiscipline::for_component` is extended
(see the Content Contract engine changes); it does not change today's behavior.

| KC id | kc_code | Cluster | Type | Sub-domain | Primary section | Overlap sections | Difficulty | Prerequisites (canonical ids) | Status |
|---|---|---|---|---|---|---|---|---|---|
| `PsychSoc::Biological_and_Social_Factors` | `PSY-BIOSF` | Bio & Social Foundations | foundation | Psy | Psych_Soc | Bio_Biochem | 1-4 | `Bio::Endocrine_System`, `Bio::Genetics`, `Bio::Nervous_System` | reused |
| `PsychSoc::Sensory_Processing` | `PSY-SENP` | Sensation & Perception | foundation | Psy | Psych_Soc | Bio_Biochem; Chem_Phys(verify) | 1-3 | `Bio::Nervous_System` | reused |
| `PsychSoc::The_Senses` | `PSY-SEN` | Sensation & Perception | mechanism | Psy | Psych_Soc | Bio_Biochem; Chem_Phys(verify) | 1-4 | `PsychSoc::Sensory_Processing` | reused |
| `PsychSoc::Attention` | `PSY-ATT` | Sensation & Perception | mechanism | Psy | Psych_Soc | — | 2-4 | `PsychSoc::Sensory_Processing` | reused |
| `PsychSoc::Perception` | `PSY-PER` | Sensation & Perception | mechanism | Psy | Psych_Soc | Chem_Phys(verify) | 2-4 | `PsychSoc::Attention`, `PsychSoc::The_Senses` | reused |
| `PsychSoc::Cognition` | `PSY-COG` | Cognition & Consciousness | mechanism | Psy | Psych_Soc | — | 2-4 | `PsychSoc::Attention`, `PsychSoc::Perception` | reused |
| `PsychSoc::Memory` | `PSY-MEM` | Cognition & Consciousness | mechanism | Psy | Psych_Soc | Bio_Biochem(verify) | 2-5 | `PsychSoc::Cognition` | reused |
| `PsychSoc::Consciousness` | `PSY-CON` | Cognition & Consciousness | mechanism | Psy | Psych_Soc | Bio_Biochem | 2-4 | `PsychSoc::Attention`, `PsychSoc::Cognition` | reused |
| `PsychSoc::Language` | `PSY-LAN` | Cognition & Consciousness | mechanism | Psy | Psych_Soc | Bio_Biochem(verify) | 2-4 | `PsychSoc::Cognition` | reused |
| `PsychSoc::Intelligence` | `PSY-INT` | Cognition & Consciousness | application | Psy | Psych_Soc | — | 3-5 | `PsychSoc::Cognition`, `PsychSoc::Language`, `PsychSoc::Memory` | new |
| `PsychSoc::Learning` | `PSY-LEA` | Learning | mechanism | Psy | Psych_Soc | Bio_Biochem(verify) | 1-4 | `PsychSoc::Biological_and_Social_Factors` | new |
| `PsychSoc::Emotion` | `PSY-EMO` | Motivation, Emotion & Stress | mechanism | Psy | Psych_Soc | Bio_Biochem | 2-4 | `PsychSoc::Biological_and_Social_Factors` | reused |
| `PsychSoc::Motivation` | `PSY-MOT` | Motivation, Emotion & Stress | mechanism | Psy | Psych_Soc | Bio_Biochem | 2-4 | `PsychSoc::Biological_and_Social_Factors`, `PsychSoc::Emotion` | reused |
| `PsychSoc::Stress` | `PSY-STR` | Motivation, Emotion & Stress | application | Psy | Psych_Soc | Bio_Biochem | 2-4 | `Bio::Endocrine_System`, `PsychSoc::Emotion` | reused |
| `PsychSoc::Personality` | `PSY-PER2` | Identity & Personality | mechanism | Psy | Psych_Soc | — | 2-4 | `PsychSoc::Biological_and_Social_Factors` | reused |
| `PsychSoc::Self_and_Identity` | `PSY-SELI` | Identity & Personality | application | Psy | Psych_Soc | — | 2-4 | `PsychSoc::Personality`, `PsychSoc::Socialization` | reused |
| `PsychSoc::Psychological_Disorders` | `PSY-PSYD` | Psychological Disorders | application | Psy | Psych_Soc | Bio_Biochem | 3-5 | `PsychSoc::Biological_and_Social_Factors`, `PsychSoc::Emotion`, `PsychSoc::Stress` | reused |
| `PsychSoc::Attitudes_and_Beliefs` | `PSY-ATTB` | Social Psychology | mechanism | Psy | Psych_Soc | — | 2-4 | `PsychSoc::Cognition`, `PsychSoc::Learning` | reused |
| `PsychSoc::Stereotypes` | `PSY-STE` | Social Psychology | mechanism | Psy | Psych_Soc | — | 3-5 | `PsychSoc::Attitudes_and_Beliefs`, `PsychSoc::Cognition` | reused |
| `PsychSoc::Prejudice_and_Bias` | `PSY-PREB` | Social Psychology | application | Psy/Soc | Psych_Soc | — | 3-5 | `PsychSoc::Attitudes_and_Beliefs`, `PsychSoc::Stereotypes` | reused |
| `PsychSoc::Social_Interaction` | `PSY-SOCI` | Social Psychology | application | Psy | Psych_Soc | — | 3-5 | `PsychSoc::Attitudes_and_Beliefs`, `PsychSoc::Self_and_Identity` | new |
| `PsychSoc::Group_Behavior` | `PSY-GROB` | Social Psychology | application | Psy/Soc | Psych_Soc | — | 3-5 | `PsychSoc::Social_Interaction`, `PsychSoc::Socialization` | new |
| `PsychSoc::Social_Theory` | `PSY-SOCT` | Social Structure & Culture | foundation | Soc | Psych_Soc | — | 2-4 | — | new |
| `PsychSoc::Culture` | `PSY-CUL` | Social Structure & Culture | foundation | Soc | Psych_Soc | — | 1-3 | — | reused |
| `PsychSoc::Socialization` | `PSY-SOC` | Social Structure & Culture | mechanism | Soc | Psych_Soc | — | 1-3 | `PsychSoc::Culture` | new |
| `PsychSoc::Social_Institutions` | `PSY-SOCI2` | Social Structure & Culture | application | Soc | Psych_Soc | — | 2-4 | `PsychSoc::Culture`, `PsychSoc::Social_Theory` | new |
| `PsychSoc::Demographics` | `PSY-DEM` | Demographics | application | Soc | Psych_Soc | — | 2-4 | `PsychSoc::Culture`, `PsychSoc::Social_Institutions` | new |
| `PsychSoc::Stratification` | `PSY-STR2` | Social Inequality & Stratification | mechanism | Soc | Psych_Soc | — | 2-4 | `PsychSoc::Social_Theory` | reused |
| `PsychSoc::Social_Class` | `PSY-SOCC` | Social Inequality & Stratification | application | Soc | Psych_Soc | — | 2-4 | `PsychSoc::Stratification` | reused |
| `PsychSoc::Social_Mobility` | `PSY-SOCM` | Social Inequality & Stratification | application | Soc | Psych_Soc | — | 2-4 | `PsychSoc::Social_Class`, `PsychSoc::Stratification` | reused |
| `PsychSoc::Poverty` | `PSY-POV` | Social Inequality & Stratification | application | Soc | Psych_Soc | — | 2-4 | `PsychSoc::Social_Class` | reused |
| `PsychSoc::Social_Inequality` | `PSY-SOCI3` | Social Inequality & Stratification | application | Soc | Psych_Soc | — | 3-5 | `PsychSoc::Demographics`, `PsychSoc::Stratification` | new |
| `PsychSoc::Health_Disparities` | `PSY-HEAD` | Health & Healthcare Disparities | application | Soc | Psych_Soc | Bio_Biochem(verify) | 3-5 | `PsychSoc::Social_Class`, `PsychSoc::Social_Inequality` | reused |
| `PsychSoc::Healthcare_Disparities` | `PSY-HEAD2` | Health & Healthcare Disparities | application | Soc | Psych_Soc | — | 3-5 | `PsychSoc::Health_Disparities`, `PsychSoc::Social_Institutions` | new |

---

## 7. Merged prerequisite edge list

Authored `prerequisite -> target` (stored via
`add_prerequisite(target, prerequisite)`). 321 edges total: 252 intra-discipline,
54 firm cross-discipline, 15 soft/`(verify)` cross-discipline. This is the
machine source; the §6 Prerequisites column is derived from it.

#### Biology intra-discipline edges (41)

```text
Bio::DNA -> Bio::DNA_Replication
Bio::DNA -> Bio::Transcription
Bio::DNA -> Bio::Genetics
Bio::DNA -> Bio::Viruses
Bio::DNA -> Bio::Biotechnology
Bio::DNA_Replication -> Bio::Cell_Cycle_and_Mitosis
Bio::Transcription -> Bio::Translation
Bio::Transcription -> Bio::Gene_Expression_Regulation
Bio::Translation -> Bio::Gene_Expression_Regulation
Bio::Genetics -> Bio::Biotechnology
Bio::Genetics -> Bio::Mendelian_Genetics
Bio::Genetics -> Bio::Evolution
Bio::Genetics -> Bio::Bacteria
Bio::Eukaryotic_Cell -> Bio::Prokaryotes_vs_Eukaryotes
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

#### Biochemistry intra-discipline edges (37)

```text
Biochem::Amino_Acids -> Biochem::Peptides_and_Proteins
Biochem::Peptides_and_Proteins -> Biochem::Protein_Structure_and_Function
Biochem::Protein_Structure_and_Function -> Biochem::Enzymes
Biochem::Enzymes -> Biochem::Bioenergetics
Biochem::Bioenergetics -> Biochem::Glycolysis
Biochem::Bioenergetics -> Biochem::Citric_Acid_Cycle
Biochem::Glycolysis -> Biochem::Citric_Acid_Cycle
Biochem::Protein_Structure_and_Function -> Biochem::Protein_Folding_and_Stability
Biochem::Protein_Structure_and_Function -> Biochem::Nonenzymatic_Protein_Function
Biochem::Enzymes -> Biochem::Enzyme_Kinetics
Biochem::Enzyme_Kinetics -> Biochem::Enzyme_Inhibition
Biochem::Enzymes -> Biochem::Metabolic_Regulation
Biochem::Bioenergetics -> Biochem::Metabolic_Regulation
Biochem::Carbohydrates_and_Lipids -> Biochem::Glycolysis
Biochem::Glycolysis -> Biochem::Gluconeogenesis
Biochem::Metabolic_Regulation -> Biochem::Gluconeogenesis
Biochem::Glycolysis -> Biochem::Glycogen_Metabolism
Biochem::Metabolic_Regulation -> Biochem::Glycogen_Metabolism
Biochem::Carbohydrates_and_Lipids -> Biochem::Glycogen_Metabolism
Biochem::Glycolysis -> Biochem::Pentose_Phosphate_Pathway
Biochem::Nucleotides_and_Nucleic_Acids -> Biochem::Pentose_Phosphate_Pathway
Biochem::Citric_Acid_Cycle -> Biochem::Oxidative_Phosphorylation
Biochem::Carbohydrates_and_Lipids -> Biochem::Lipid_Metabolism
Biochem::Bioenergetics -> Biochem::Lipid_Metabolism
Biochem::Citric_Acid_Cycle -> Biochem::Lipid_Metabolism
Biochem::Amino_Acids -> Biochem::Amino_Acid_Metabolism
Biochem::Citric_Acid_Cycle -> Biochem::Amino_Acid_Metabolism
Biochem::Metabolic_Regulation -> Biochem::Amino_Acid_Metabolism
Biochem::Carbohydrates_and_Lipids -> Biochem::Membranes_and_Transport
Biochem::Protein_Structure_and_Function -> Biochem::Membranes_and_Transport
Biochem::Peptides_and_Proteins -> Biochem::Chromatography_and_Separations
Biochem::Peptides_and_Proteins -> Biochem::Electrophoresis_and_Immunoassays
Biochem::Nucleotides_and_Nucleic_Acids -> Biochem::Electrophoresis_and_Immunoassays
Biochem::Enzymes -> Biochem::Vitamins_and_Cofactors
Biochem::Metabolic_Regulation -> Biochem::Hormonal_Regulation_of_Metabolism
Biochem::Gluconeogenesis -> Biochem::Hormonal_Regulation_of_Metabolism
Biochem::Lipid_Metabolism -> Biochem::Hormonal_Regulation_of_Metabolism
```

#### General Chemistry intra-discipline edges (31)

```text
GenChem::Atomic_Structure -> GenChem::Electron_Configuration
GenChem::Electron_Configuration -> GenChem::Atomic_Spectra_and_Quantum
GenChem::Electron_Configuration -> GenChem::Periodic_Trends
GenChem::Periodic_Trends -> GenChem::Chemical_Bonding
GenChem::Chemical_Bonding -> GenChem::Molecular_Geometry
GenChem::Chemical_Bonding -> GenChem::Stoichiometry
GenChem::Stoichiometry -> GenChem::Reaction_Types
GenChem::Reaction_Types -> GenChem::Redox_Reactions
GenChem::Stoichiometry -> GenChem::Gas_Phase
GenChem::Molecular_Geometry -> GenChem::Intermolecular_Forces
GenChem::Intermolecular_Forces -> GenChem::Phases_and_Phase_Changes
GenChem::Gas_Phase -> GenChem::Phases_and_Phase_Changes
GenChem::Intermolecular_Forces -> GenChem::Solutions_and_Solubility
GenChem::Solutions_and_Solubility -> GenChem::Ions_in_Solutions
GenChem::Solutions_and_Solubility -> GenChem::Spectrophotometry
GenChem::Solutions_and_Solubility -> GenChem::Colligative_Properties
GenChem::Phases_and_Phase_Changes -> GenChem::Colligative_Properties
GenChem::Stoichiometry -> GenChem::Thermochemistry
GenChem::Thermochemistry -> GenChem::Thermodynamics
GenChem::Stoichiometry -> GenChem::Kinetics
GenChem::Thermochemistry -> GenChem::Kinetics
GenChem::Kinetics -> GenChem::Equilibrium
GenChem::Ions_in_Solutions -> GenChem::Solubility_Equilibria
GenChem::Equilibrium -> GenChem::Solubility_Equilibria
GenChem::Equilibrium -> GenChem::Acid_Base_Equilibria
GenChem::Acid_Base_Equilibria -> GenChem::Buffers
GenChem::Acid_Base_Equilibria -> GenChem::Titration
GenChem::Buffers -> GenChem::Titration
GenChem::Redox_Reactions -> GenChem::Electrochemistry
GenChem::Thermodynamics -> GenChem::Electrochemistry
GenChem::Atomic_Structure -> GenChem::Nuclear_Chemistry
```

#### Organic Chemistry intra-discipline edges (61)

```text
Orgo::Hybridization -> Orgo::Functional_Groups
Orgo::Functional_Groups -> Orgo::Nomenclature
Orgo::Hybridization -> Orgo::Nomenclature
Orgo::Functional_Groups -> Orgo::Isomerism
Orgo::Nomenclature -> Orgo::Isomerism
Orgo::Isomerism -> Orgo::Stereochemistry
Orgo::Hybridization -> Orgo::Stereochemistry
Orgo::Functional_Groups -> Orgo::Acid_Base_Reactions
Orgo::Functional_Groups -> Orgo::Reaction_Mechanisms_Overview
Orgo::Acid_Base_Reactions -> Orgo::Reaction_Mechanisms_Overview
Orgo::Reaction_Mechanisms_Overview -> Orgo::Nucleophilic_Substitution
Orgo::Stereochemistry -> Orgo::Nucleophilic_Substitution
Orgo::Reaction_Mechanisms_Overview -> Orgo::Elimination_Reactions
Orgo::Nucleophilic_Substitution -> Orgo::Elimination_Reactions
Orgo::Reaction_Mechanisms_Overview -> Orgo::Nucleophilic_Addition
Orgo::Acid_Base_Reactions -> Orgo::Nucleophilic_Addition
Orgo::Nucleophilic_Addition -> Orgo::Nucleophilic_Acyl_Substitution
Orgo::Acid_Base_Reactions -> Orgo::Nucleophilic_Acyl_Substitution
Orgo::Functional_Groups -> Orgo::Oxidation_Reduction_Reactions
Orgo::Reaction_Mechanisms_Overview -> Orgo::Oxidation_Reduction_Reactions
Orgo::Functional_Groups -> Orgo::Alcohols
Orgo::Acid_Base_Reactions -> Orgo::Alcohols
Orgo::Nucleophilic_Substitution -> Orgo::Alcohols
Orgo::Oxidation_Reduction_Reactions -> Orgo::Alcohols
Orgo::Hybridization -> Orgo::Polycyclic_and_Heterocyclic_Aromatic_Compounds
Orgo::Functional_Groups -> Orgo::Polycyclic_and_Heterocyclic_Aromatic_Compounds
Orgo::Isomerism -> Orgo::Polycyclic_and_Heterocyclic_Aromatic_Compounds
Orgo::Alcohols -> Orgo::Phenols
Orgo::Polycyclic_and_Heterocyclic_Aromatic_Compounds -> Orgo::Phenols
Orgo::Acid_Base_Reactions -> Orgo::Phenols
Orgo::Nucleophilic_Addition -> Orgo::Aldehydes_and_Ketones
Orgo::Oxidation_Reduction_Reactions -> Orgo::Aldehydes_and_Ketones
Orgo::Functional_Groups -> Orgo::Aldehydes_and_Ketones
Orgo::Acid_Base_Reactions -> Orgo::Carboxylic_Acids
Orgo::Oxidation_Reduction_Reactions -> Orgo::Carboxylic_Acids
Orgo::Functional_Groups -> Orgo::Carboxylic_Acids
Orgo::Nucleophilic_Acyl_Substitution -> Orgo::Acid_Derivatives
Orgo::Carboxylic_Acids -> Orgo::Acid_Derivatives
Orgo::Acid_Base_Reactions -> Orgo::Amines
Orgo::Functional_Groups -> Orgo::Amines
Orgo::Nucleophilic_Substitution -> Orgo::Amines
Orgo::Stereochemistry -> Orgo::Carbohydrate_Chemistry
Orgo::Nucleophilic_Addition -> Orgo::Carbohydrate_Chemistry
Orgo::Alcohols -> Orgo::Carbohydrate_Chemistry
Orgo::Stereochemistry -> Orgo::Amino_Acid_and_Peptide_Chemistry
Orgo::Acid_Base_Reactions -> Orgo::Amino_Acid_and_Peptide_Chemistry
Orgo::Nucleophilic_Acyl_Substitution -> Orgo::Amino_Acid_and_Peptide_Chemistry
Orgo::Amines -> Orgo::Amino_Acid_and_Peptide_Chemistry
Orgo::Functional_Groups -> Orgo::IR_Spectroscopy
Orgo::Functional_Groups -> Orgo::NMR_Spectroscopy
Orgo::Hybridization -> Orgo::NMR_Spectroscopy
Orgo::Functional_Groups -> Orgo::Mass_Spectrometry
Orgo::Reaction_Mechanisms_Overview -> Orgo::Mass_Spectrometry
Orgo::Hybridization -> Orgo::Molecular_Structure_and_Absorption_Spectra
Orgo::Polycyclic_and_Heterocyclic_Aromatic_Compounds -> Orgo::Molecular_Structure_and_Absorption_Spectra
Orgo::Functional_Groups -> Orgo::Separations_and_Purifications
Orgo::Acid_Base_Reactions -> Orgo::Separations_and_Purifications
Orgo::Separations_and_Purifications -> Orgo::Laboratory_Techniques
Orgo::IR_Spectroscopy -> Orgo::Laboratory_Techniques
Orgo::NMR_Spectroscopy -> Orgo::Laboratory_Techniques
Orgo::Mass_Spectrometry -> Orgo::Laboratory_Techniques
```

#### Physics intra-discipline edges (32)

```text
Physics::Units_And_Measurement -> Physics::Kinematics
Physics::Kinematics -> Physics::Newtons_Laws
Physics::Newtons_Laws -> Physics::Force_Equilibrium
Physics::Newtons_Laws -> Physics::Work_And_Energy
Physics::Newtons_Laws -> Physics::Momentum_And_Impulse
Physics::Force_Equilibrium -> Physics::Rotational_Motion
Physics::Work_And_Energy -> Physics::Rotational_Motion
Physics::Force_Equilibrium -> Physics::Periodic_Motion
Physics::Work_And_Energy -> Physics::Periodic_Motion
Physics::Force_Equilibrium -> Physics::Fluid_Statics
Physics::Fluid_Statics -> Physics::Fluid_Dynamics
Physics::Work_And_Energy -> Physics::Fluid_Dynamics
Physics::Work_And_Energy -> Physics::Thermodynamics
Physics::Periodic_Motion -> Physics::Waves
Physics::Waves -> Physics::Sound
Physics::Work_And_Energy -> Physics::Electrostatics
Physics::Electrostatics -> Physics::Electrical_Circuits
Physics::Electrical_Circuits -> Physics::Circuit_Elements
Physics::Electrostatics -> Physics::Magnetism
Physics::Waves -> Physics::Electromagnetic_Radiation
Physics::Magnetism -> Physics::Electromagnetic_Radiation
Physics::Electromagnetic_Radiation -> Physics::Geometric_Optics
Physics::Geometric_Optics -> Physics::Physical_Optics
Physics::Waves -> Physics::Physical_Optics
Physics::Electromagnetic_Radiation -> Physics::Atomic_Structure
Physics::Electrostatics -> Physics::Atomic_Structure
Physics::Atomic_Structure -> Physics::Nuclear_Physics
Physics::Fluid_Dynamics -> Physics::Circulatory_Hemodynamics
Physics::Fluid_Statics -> Physics::Gas_Exchange_And_Respiration_Physics
Physics::Thermodynamics -> Physics::Gas_Exchange_And_Respiration_Physics
Physics::Geometric_Optics -> Physics::Optics_Of_The_Eye
Physics::Circuit_Elements -> Physics::Bioelectricity
```

#### Psychology/Sociology intra-discipline edges (50)

```text
PsychSoc::Sensory_Processing -> PsychSoc::The_Senses
PsychSoc::Sensory_Processing -> PsychSoc::Attention
PsychSoc::The_Senses -> PsychSoc::Perception
PsychSoc::Attention -> PsychSoc::Perception
PsychSoc::Perception -> PsychSoc::Cognition
PsychSoc::Attention -> PsychSoc::Cognition
PsychSoc::Cognition -> PsychSoc::Memory
PsychSoc::Attention -> PsychSoc::Consciousness
PsychSoc::Cognition -> PsychSoc::Consciousness
PsychSoc::Cognition -> PsychSoc::Language
PsychSoc::Cognition -> PsychSoc::Intelligence
PsychSoc::Memory -> PsychSoc::Intelligence
PsychSoc::Language -> PsychSoc::Intelligence
PsychSoc::Biological_and_Social_Factors -> PsychSoc::Learning
PsychSoc::Biological_and_Social_Factors -> PsychSoc::Emotion
PsychSoc::Biological_and_Social_Factors -> PsychSoc::Motivation
PsychSoc::Emotion -> PsychSoc::Motivation
PsychSoc::Emotion -> PsychSoc::Stress
PsychSoc::Biological_and_Social_Factors -> PsychSoc::Personality
PsychSoc::Biological_and_Social_Factors -> PsychSoc::Psychological_Disorders
PsychSoc::Emotion -> PsychSoc::Psychological_Disorders
PsychSoc::Stress -> PsychSoc::Psychological_Disorders
PsychSoc::Personality -> PsychSoc::Self_and_Identity
PsychSoc::Socialization -> PsychSoc::Self_and_Identity
PsychSoc::Cognition -> PsychSoc::Attitudes_and_Beliefs
PsychSoc::Learning -> PsychSoc::Attitudes_and_Beliefs
PsychSoc::Cognition -> PsychSoc::Stereotypes
PsychSoc::Attitudes_and_Beliefs -> PsychSoc::Stereotypes
PsychSoc::Attitudes_and_Beliefs -> PsychSoc::Prejudice_and_Bias
PsychSoc::Stereotypes -> PsychSoc::Prejudice_and_Bias
PsychSoc::Self_and_Identity -> PsychSoc::Social_Interaction
PsychSoc::Attitudes_and_Beliefs -> PsychSoc::Social_Interaction
PsychSoc::Social_Interaction -> PsychSoc::Group_Behavior
PsychSoc::Socialization -> PsychSoc::Group_Behavior
PsychSoc::Culture -> PsychSoc::Socialization
PsychSoc::Social_Theory -> PsychSoc::Social_Institutions
PsychSoc::Culture -> PsychSoc::Social_Institutions
PsychSoc::Social_Institutions -> PsychSoc::Demographics
PsychSoc::Culture -> PsychSoc::Demographics
PsychSoc::Social_Theory -> PsychSoc::Stratification
PsychSoc::Stratification -> PsychSoc::Social_Class
PsychSoc::Stratification -> PsychSoc::Social_Mobility
PsychSoc::Social_Class -> PsychSoc::Social_Mobility
PsychSoc::Social_Class -> PsychSoc::Poverty
PsychSoc::Stratification -> PsychSoc::Social_Inequality
PsychSoc::Demographics -> PsychSoc::Social_Inequality
PsychSoc::Social_Inequality -> PsychSoc::Health_Disparities
PsychSoc::Social_Class -> PsychSoc::Health_Disparities
PsychSoc::Health_Disparities -> PsychSoc::Healthcare_Disparities
PsychSoc::Social_Institutions -> PsychSoc::Healthcare_Disparities
```

#### Cross-discipline edges — firm (54)

```text
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
Physics::Fluid_Dynamics -> Bio::Circulatory_System
Bio::Eukaryotic_Cell -> Biochem::Bioenergetics
GenChem::Acid_Base_Equilibria -> Biochem::Amino_Acids
GenChem::Chemical_Bonding -> Biochem::Amino_Acids
Orgo::Functional_Groups -> Biochem::Amino_Acids
Orgo::Acid_Derivatives -> Biochem::Peptides_and_Proteins
GenChem::Chemical_Bonding -> Biochem::Protein_Structure_and_Function
GenChem::Intermolecular_Forces -> Biochem::Protein_Structure_and_Function
GenChem::Intermolecular_Forces -> Biochem::Protein_Folding_and_Stability
GenChem::Thermochemistry -> Biochem::Protein_Folding_and_Stability
GenChem::Kinetics -> Biochem::Enzyme_Kinetics
GenChem::Thermochemistry -> Biochem::Bioenergetics
Orgo::Functional_Groups -> Biochem::Carbohydrates_and_Lipids
Orgo::Stereochemistry -> Biochem::Carbohydrates_and_Lipids
Orgo::Aldehydes_and_Ketones -> Biochem::Carbohydrates_and_Lipids
GenChem::Electrochemistry -> Biochem::Oxidative_Phosphorylation
Bio::Eukaryotic_Cell -> Biochem::Oxidative_Phosphorylation
Bio::DNA -> Biochem::Nucleotides_and_Nucleic_Acids
GenChem::Chemical_Bonding -> Biochem::Nucleotides_and_Nucleic_Acids
Bio::Eukaryotic_Cell -> Biochem::Membranes_and_Transport
Orgo::Separations_and_Purifications -> Biochem::Chromatography_and_Separations
GenChem::Intermolecular_Forces -> Biochem::Chromatography_and_Separations
GenChem::Acid_Base_Equilibria -> Biochem::Electrophoresis_and_Immunoassays
Bio::Endocrine_System -> Biochem::Hormonal_Regulation_of_Metabolism
GenChem::Chemical_Bonding -> Orgo::Hybridization
GenChem::Molecular_Geometry -> Orgo::Hybridization
GenChem::Chemical_Bonding -> Orgo::Functional_Groups
GenChem::Acid_Base_Equilibria -> Orgo::Acid_Base_Reactions
GenChem::Kinetics -> Orgo::Reaction_Mechanisms_Overview
GenChem::Thermochemistry -> Orgo::Reaction_Mechanisms_Overview
GenChem::Kinetics -> Orgo::Nucleophilic_Substitution
GenChem::Electrochemistry -> Orgo::Oxidation_Reduction_Reactions
GenChem::Solutions_and_Solubility -> Orgo::Separations_and_Purifications
GenChem::Intermolecular_Forces -> Orgo::Separations_and_Purifications
Physics::Electromagnetic_Radiation -> Orgo::IR_Spectroscopy
Physics::Electromagnetic_Radiation -> Orgo::NMR_Spectroscopy
Physics::Electromagnetic_Radiation -> Orgo::Molecular_Structure_and_Absorption_Spectra
GenChem::Gas_Phase -> Physics::Gas_Exchange_And_Respiration_Physics
GenChem::Ions_in_Solutions -> Physics::Bioelectricity
Bio::Nervous_System -> PsychSoc::Biological_and_Social_Factors
Bio::Nervous_System -> PsychSoc::Sensory_Processing
```

#### Cross-discipline edges — soft / (verify) (15)

These are pedagogically defensible but flagged by the source maps for
human review before entering the canonical graph. They are all confirmed
acyclic together with the firm set. They can be demoted to lesson-only "related
KC" links without changing the firm graph.

```text
Biochem::Enzymes -> Bio::DNA_Replication    (verify)
Physics::Atomic_Structure -> GenChem::Electron_Configuration    (verify)
Physics::Electromagnetic_Radiation -> GenChem::Atomic_Spectra_and_Quantum    (verify)
Physics::Thermodynamics -> GenChem::Gas_Phase    (verify)
Physics::Work_And_Energy -> GenChem::Thermochemistry    (verify)
Physics::Thermodynamics -> GenChem::Thermodynamics    (verify)
Physics::Electromagnetic_Radiation -> GenChem::Spectrophotometry    (verify)
Physics::Electrostatics -> GenChem::Electrochemistry    (verify)
Physics::Electrical_Circuits -> GenChem::Electrochemistry    (verify)
Physics::Nuclear_Physics -> GenChem::Nuclear_Chemistry    (verify)
Physics::Atomic_Structure -> GenChem::Nuclear_Chemistry    (verify)
GenChem::Thermochemistry -> Physics::Thermodynamics    (verify)
Bio::Endocrine_System -> PsychSoc::Biological_and_Social_Factors    (verify)
Bio::Genetics -> PsychSoc::Biological_and_Social_Factors    (verify)
Bio::Endocrine_System -> PsychSoc::Stress    (verify)
```

---

## 8. Open `(verify)` items carried from the research maps

- **Soft cross-discipline edges (15 above).** All acyclic; confirm before adding
  to the canonical graph, especially the GenChem↔Physics thermodynamics/atomic
  couplings and the `Biochem::Enzymes -> Bio::DNA_Replication` enzymology edge.
- **`Physics::Light` spectroscopy repoint.** `GenChem::Atomic_Spectra_and_Quantum`
  and `GenChem::Spectrophotometry` were repointed to
  `Physics::Electromagnetic_Radiation`; confirm EM radiation is the intended
  survivor vs `Physics::Physical_Optics`.
- **Deliberate non-merges (§3.4).** Confirm the membrane, biomolecule, atomic,
  and nuclear/thermo overlaps should remain distinct KCs rather than merge.
- **Psych/Soc sub-domain split.** The `Sub-domain` column is not yet consumed by
  the engine (see Content Contract engine-changes §c).
- **Lower-yield KCs** (`Bio::Fungi`, `Bio::Biodiversity_and_Phylogeny`,
  `Physics::Rotational_Motion`) may be deferred if the card budget is tight.
- **Carbohydrate/lipid split.** `Biochem::Carbohydrates_and_Lipids` may later
  split into `Biochem::Carbohydrates` + `Biochem::Lipids`; deferred to avoid
  churning an id other tracks reference.
