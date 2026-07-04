# Research KC Map: MCAT General Chemistry (Track A)

Machine-usable Knowledge Component (KC) map for MCAT **General Chemistry**, produced
for Track A of the Concept Scheduler expansion (see `next-feature-expansion-plan.md`).
It extends the ad-hoc `GenChem::` entries in `mcat.md` into a coherent, acyclic
prerequisite lattice ready for card generation (Track B) and lesson stubs (Track C).

- Scope: 26 KCs spanning atomic structure/periodic trends, bonding, stoichiometry,
  gases, phases/intermolecular forces, solutions, thermochemistry/thermodynamics,
  kinetics, equilibrium, acids/bases, buffers/titration, electrochemistry, and nuclear.
- All content is synthetic/original (AAMC-style foundational-concept framing, no
  copyrighted text). Uncertain design choices are marked **(verify)**.
- KC ids use the area prefix `GenChem::` and `snake_case` topics, matching the tag
  parser in `rslib/src/scheduler/concept.rs` and the section derivation in
  `qt/aqt/concept_tags.py`.

## Conventions Recap

- KC id form: `GenChem::<Topic_in_snake_case>`; on cards it appears as `KC::GenChem::<Topic>`.
- Prerequisite edges are authored as `Prerequisite KC -> Target KC` (audit-doc
  convention), stored via `add_prerequisite(target, prerequisite)`.
- MCAT sections a KC can feed evidence to: General Chemistry maps **primarily to
  `Chem_Phys`** (`derived_mcat_sections_for_topics` returns `["Chem_Phys"]` for
  `GenChem::`). Many GenChem KCs also **support `Bio_Biochem`** in practice, because
  biochemistry leans on them; this matches `discipline_weight`, where
  `GeneralChemistry` contributes `0.30` to `Chem_Phys` and `0.05` to `Bio_Biochem`.
  Support is marked `+Bio_Biochem` below and should be applied as a card-level
  `MCAT::Bio_Biochem` override only on biochem-relevant items.
- Difficulty ladder = suggested spread of `Difficulty::N` (1-5) cards to author per
  KC, easy calibration first and harder reasoning last.
- Type ∈ {foundation, mechanism, application, detail}.

Legend for the master table: prerequisite entries omit the `GenChem::` prefix
(all are `GenChem::` unless shown as `Physics::`); cross-discipline prereqs are
soft/optional and listed in their own column. Fully-qualified, machine-usable
edges live in the **Prerequisite edge list** section.

## Cluster Overview

| # | Parent cluster | KCs |
|---|----------------|-----|
| 1 | Atomic Structure & Periodicity | Atomic_Structure, Electron_Configuration, Atomic_Spectra_and_Quantum, Periodic_Trends |
| 2 | Bonding & Molecular Structure | Chemical_Bonding, Molecular_Geometry |
| 3 | Stoichiometry & Reactions | Stoichiometry, Reaction_Types, Redox_Reactions |
| 4 | Gases, Phases & Intermolecular Forces | Gas_Phase, Intermolecular_Forces, Phases_and_Phase_Changes |
| 5 | Solutions | Solutions_and_Solubility, Ions_in_Solutions, Spectrophotometry, Colligative_Properties, Solubility_Equilibria |
| 6 | Thermodynamics | Thermochemistry, Thermodynamics |
| 7 | Kinetics | Kinetics |
| 8 | Equilibrium | Equilibrium |
| 9 | Acids & Bases | Acid_Base_Equilibria, Buffers, Titration |
| 10 | Electrochemistry | Electrochemistry |
| 11 | Nuclear | Nuclear_Chemistry |

## Master KC Table

| KC id | Parent cluster | Prerequisites (GenChem) | Cross-discipline prereq (verify) | MCAT sections | Difficulty ladder | Type |
|-------|----------------|-------------------------|----------------------------------|---------------|-------------------|------|
| `GenChem::Atomic_Structure` | Atomic Structure & Periodicity | — (root) | — | Chem_Phys | 1→2→3 | foundation |
| `GenChem::Electron_Configuration` | Atomic Structure & Periodicity | Atomic_Structure | `Physics::Electronic_Structure` | Chem_Phys | 2→3→4 | foundation |
| `GenChem::Atomic_Spectra_and_Quantum` | Atomic Structure & Periodicity | Electron_Configuration | `Physics::Electromagnetic_Radiation`, `Physics::Light` | Chem_Phys | 3→4→5 | detail |
| `GenChem::Periodic_Trends` | Atomic Structure & Periodicity | Electron_Configuration | — | Chem_Phys | 1→2→3→4 | foundation |
| `GenChem::Chemical_Bonding` | Bonding & Molecular Structure | Periodic_Trends | — | Chem_Phys; +Bio_Biochem | 1→2→3 | foundation |
| `GenChem::Molecular_Geometry` | Bonding & Molecular Structure | Chemical_Bonding | — | Chem_Phys; +Bio_Biochem | 2→3→4 | mechanism |
| `GenChem::Stoichiometry` | Stoichiometry & Reactions | Chemical_Bonding | — | Chem_Phys; +Bio_Biochem | 1→2→3→4 | foundation |
| `GenChem::Reaction_Types` | Stoichiometry & Reactions | Stoichiometry | — | Chem_Phys | 2→3→4 | foundation |
| `GenChem::Redox_Reactions` | Stoichiometry & Reactions | Reaction_Types | — | Chem_Phys; +Bio_Biochem | 2→3→4 | mechanism |
| `GenChem::Gas_Phase` | Gases, Phases & IMF | Stoichiometry | `Physics::Thermodynamics` | Chem_Phys; +Bio_Biochem | 2→3→4→5 | mechanism |
| `GenChem::Intermolecular_Forces` | Gases, Phases & IMF | Molecular_Geometry | — | Chem_Phys; +Bio_Biochem | 2→3→4 | mechanism |
| `GenChem::Phases_and_Phase_Changes` | Gases, Phases & IMF | Intermolecular_Forces, Gas_Phase | — | Chem_Phys | 2→3→4→5 | mechanism |
| `GenChem::Solutions_and_Solubility` | Solutions | Intermolecular_Forces | — | Chem_Phys; +Bio_Biochem | 2→3→4 | foundation |
| `GenChem::Ions_in_Solutions` | Solutions | Solutions_and_Solubility | — | Chem_Phys; +Bio_Biochem | 2→3→4 | mechanism |
| `GenChem::Spectrophotometry` | Solutions | Solutions_and_Solubility | `Physics::Light` | Chem_Phys; +Bio_Biochem | 3→4→5 | detail |
| `GenChem::Colligative_Properties` | Solutions | Solutions_and_Solubility, Phases_and_Phase_Changes | — | Chem_Phys; +Bio_Biochem | 3→4→5 | application |
| `GenChem::Solubility_Equilibria` | Solutions | Ions_in_Solutions, Equilibrium | — | Chem_Phys | 3→4→5 | detail |
| `GenChem::Thermochemistry` | Thermodynamics | Stoichiometry | `Physics::Energy`, `Physics::Work` | Chem_Phys; +Bio_Biochem | 2→3→4→5 | mechanism |
| `GenChem::Thermodynamics` | Thermodynamics | Thermochemistry | `Physics::Thermodynamics` | Chem_Phys; +Bio_Biochem | 3→4→5 | mechanism |
| `GenChem::Kinetics` | Kinetics | Stoichiometry, Thermochemistry | — | Chem_Phys; +Bio_Biochem | 3→4→5 | mechanism |
| `GenChem::Equilibrium` | Equilibrium | Kinetics | — | Chem_Phys; +Bio_Biochem | 3→4→5 | mechanism |
| `GenChem::Acid_Base_Equilibria` | Acids & Bases | Equilibrium | — | Chem_Phys; +Bio_Biochem | 2→3→4→5 | mechanism |
| `GenChem::Buffers` | Acids & Bases | Acid_Base_Equilibria | — | Chem_Phys; +Bio_Biochem | 3→4→5 | application |
| `GenChem::Titration` | Acids & Bases | Acid_Base_Equilibria, Buffers | — | Chem_Phys; +Bio_Biochem | 3→4→5 | application |
| `GenChem::Electrochemistry` | Electrochemistry | Redox_Reactions, Thermodynamics | `Physics::Electrostatics`, `Physics::Electrical_Circuits` | Chem_Phys; +Bio_Biochem | 3→4→5 | application |
| `GenChem::Nuclear_Chemistry` | Nuclear | Atomic_Structure | `Physics::Nuclear_Decay`, `Physics::Atoms` | Chem_Phys | 2→3→4 | detail |

## Prerequisite Edge List (GenChem-internal, acyclic)

Canonical `prerequisite -> target` edges. This is the machine-usable source for
graph authoring; a topological order is given afterward to confirm acyclicity.

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

Topological order (each KC appears after all its GenChem prerequisites, so the
graph is a DAG):

```text
1  Atomic_Structure
2  Electron_Configuration
3  Atomic_Spectra_and_Quantum
4  Periodic_Trends
5  Chemical_Bonding
6  Molecular_Geometry
7  Stoichiometry
8  Reaction_Types
9  Redox_Reactions
10 Gas_Phase
11 Intermolecular_Forces
12 Phases_and_Phase_Changes
13 Solutions_and_Solubility
14 Ions_in_Solutions
15 Spectrophotometry
16 Colligative_Properties
17 Thermochemistry
18 Thermodynamics
19 Kinetics
20 Equilibrium
21 Solubility_Equilibria
22 Acid_Base_Equilibria
23 Buffers
24 Titration
25 Electrochemistry
26 Nuclear_Chemistry
```

## Cross-Discipline Prerequisite List (soft, verify)

These reference existing `Physics::` KCs from `mcat.md`. They are **optional soft
prerequisites** capturing shared physical foundations; keep them out of the strict
GenChem ladder if they cause scheduling friction. All are marked **(verify)** and
none introduce a cycle (every edge points from a `Physics::` node into a
`GenChem::` node).

```text
Physics::Electronic_Structure -> GenChem::Electron_Configuration        (verify)
Physics::Electromagnetic_Radiation -> GenChem::Atomic_Spectra_and_Quantum (verify)
Physics::Light -> GenChem::Atomic_Spectra_and_Quantum                    (verify)
Physics::Thermodynamics -> GenChem::Gas_Phase                           (verify)
Physics::Energy -> GenChem::Thermochemistry                            (verify)
Physics::Work -> GenChem::Thermochemistry                              (verify)
Physics::Thermodynamics -> GenChem::Thermodynamics                     (verify)
Physics::Light -> GenChem::Spectrophotometry                           (verify)
Physics::Electrostatics -> GenChem::Electrochemistry                   (verify)
Physics::Electrical_Circuits -> GenChem::Electrochemistry              (verify)
Physics::Nuclear_Decay -> GenChem::Nuclear_Chemistry                   (verify)
Physics::Atoms -> GenChem::Nuclear_Chemistry                           (verify)
```

Distinct cross-discipline prereqs referenced (10 Physics KCs):
`Physics::Electronic_Structure`, `Physics::Electromagnetic_Radiation`,
`Physics::Light`, `Physics::Thermodynamics`, `Physics::Energy`, `Physics::Work`,
`Physics::Electrostatics`, `Physics::Electrical_Circuits`, `Physics::Nuclear_Decay`,
`Physics::Atoms`.

## Per-Cluster Scope and Content

Concrete sub-topics per KC (for Track B item generation and Track C lessons), with
a suggested primary `Reasoning::` emphasis and a common-misconception hook.

### 1. Atomic Structure & Periodicity

- `GenChem::Atomic_Structure` — protons/neutrons/electrons, atomic number, mass
  number, isotopes, average atomic mass, ions, nuclear notation. Reasoning:
  Conceptual. Misconception: conflating mass number with average atomic mass.
- `GenChem::Electron_Configuration` — orbitals (s/p/d/f), quantum numbers, Aufbau,
  Hund's rule, Pauli exclusion, ground vs excited state, valence electrons,
  paramagnetism. Reasoning: Conceptual. Misconception: filling 4s/3d order and
  ignoring exceptions (Cr, Cu) **(verify exceptions list)**.
- `GenChem::Atomic_Spectra_and_Quantum` — Bohr model, quantized energy levels,
  emission/absorption spectra, photoelectric effect, E = hf and E = hc/λ.
  Reasoning: Data/Application. Misconception: treating emission lines as continuous.
- `GenChem::Periodic_Trends` — atomic/ionic radius, ionization energy, electron
  affinity, electronegativity, metallic character, effective nuclear charge.
  Reasoning: Conceptual/Application. Misconception: ranking radius without regard
  to isoelectronic series.

### 2. Bonding & Molecular Structure

- `GenChem::Chemical_Bonding` — ionic vs covalent vs metallic, polarity from
  electronegativity difference, bond order/length/energy, Lewis structures, formal
  charge, resonance, octet exceptions. Reasoning: Conceptual. Misconception:
  assuming all bonds with a polar bond make a polar molecule.
- `GenChem::Molecular_Geometry` — VSEPR shapes, bond angles, hybridization
  (sp/sp2/sp3), sigma vs pi, net molecular dipole. Reasoning: Conceptual/Application.
  Overlaps `Orgo::Hybridization` (shared foundation, not a prereq edge).
  Misconception: lone-pair effect on geometry vs electron domain geometry.

### 3. Stoichiometry & Reactions

- `GenChem::Stoichiometry` — mole concept, molar mass, empirical/molecular formula,
  balancing, limiting reagent, percent yield, percent composition, density/gas
  stoichiometry link. Reasoning: Application. Misconception: using mass ratios
  instead of mole ratios.
- `GenChem::Reaction_Types` — synthesis, decomposition, single/double displacement,
  combustion, neutralization, precipitation, net ionic equations, solubility rules
  (qualitative). Reasoning: Conceptual/Application. Misconception: forgetting
  spectator ions in net ionic equations.
- `GenChem::Redox_Reactions` — oxidation states, identifying oxidation/reduction,
  oxidizing/reducing agents, balancing half-reactions in acid/base. Reasoning:
  Application. Misconception: sign/assignment errors in oxidation-number rules.

### 4. Gases, Phases & Intermolecular Forces

- `GenChem::Gas_Phase` — ideal gas law, combined/Boyle/Charles/Avogadro, Dalton's
  partial pressures, mole fraction, kinetic molecular theory, Graham's effusion,
  real-gas deviations (van der Waals qualitative). Reasoning: Application/Data.
  Supports respiratory physiology (`+Bio_Biochem`). Misconception: unit/temperature
  (K vs °C) errors.
- `GenChem::Intermolecular_Forces` — London dispersion, dipole-dipole, hydrogen
  bonding, ion-dipole; relation to boiling point, viscosity, surface tension; water
  as solvent. Reasoning: Conceptual. Misconception: calling H-bonds covalent bonds.
- `GenChem::Phases_and_Phase_Changes` — phase diagrams, triple/critical point,
  heating curves, heat of fusion/vaporization, vapor pressure, Clausius-Clapeyron
  (qualitative). Reasoning: Data/Application. Misconception: temperature changing
  during a phase transition.

### 5. Solutions

- `GenChem::Solutions_and_Solubility` — solute/solvent, "like dissolves like",
  molarity, molality, mole fraction, mass %, ppm, dilution, saturation, solubility
  vs temperature/pressure (Henry's law qualitative). Reasoning: Application.
  Misconception: molarity vs molality interchangeability.
- `GenChem::Ions_in_Solutions` — strong/weak electrolytes, dissociation,
  conductivity, hydration, activity vs concentration (qualitative). Reasoning:
  Conceptual. Supports membrane/nerve topics (`+Bio_Biochem`). Misconception:
  assuming all ionic compounds are highly soluble.
- `GenChem::Spectrophotometry` — Beer-Lambert law (A = εbc), calibration curves,
  transmittance vs absorbance, concentration determination. Reasoning: Data.
  Supports enzyme/assay analysis (`+Bio_Biochem`). Misconception: linear
  absorbance at very high concentration.
- `GenChem::Colligative_Properties` — boiling-point elevation, freezing-point
  depression, osmotic pressure, vapor-pressure lowering (Raoult), van't Hoff
  factor. Reasoning: Application. Supports osmosis/tonicity (`+Bio_Biochem`).
  Misconception: ignoring the van't Hoff factor for electrolytes.
- `GenChem::Solubility_Equilibria` — Ksp, molar solubility, common-ion effect,
  precipitation (Q vs Ksp), selective precipitation. Reasoning: Application.
  Misconception: equating Ksp magnitude across differing stoichiometries.

### 6. Thermodynamics

- `GenChem::Thermochemistry` — system/surroundings, state functions, enthalpy,
  endo/exothermic, calorimetry (q = mcΔT), Hess's law, heats of formation, bond
  enthalpies. Reasoning: Application. Supports bioenergetics (`+Bio_Biochem`).
  Misconception: sign conventions for q and ΔH.
- `GenChem::Thermodynamics` — entropy, second law, Gibbs free energy
  (ΔG = ΔH − TΔS), spontaneity, temperature dependence, ΔG vs ΔG°, link to K
  (ΔG° = −RT ln K). Reasoning: Application. Supports metabolic spontaneity
  (`+Bio_Biochem`). Misconception: assuming exothermic ⇒ spontaneous.

### 7. Kinetics

- `GenChem::Kinetics` — reaction rate, rate laws and order, rate constant,
  integrated rate laws/half-life (qualitative), Arrhenius/activation energy,
  catalysts, elementary steps, rate-determining step, energy diagrams. Reasoning:
  Data/Application. Supports enzyme kinetics link (`+Bio_Biochem`). Misconception:
  reading reaction order off stoichiometric coefficients.

### 8. Equilibrium

- `GenChem::Equilibrium` — dynamic equilibrium, Keq (Kc/Kp), reaction quotient Q,
  Le Chatelier's principle, ICE tables, effect of T/P/concentration. Reasoning:
  Application. Supports many biochem equilibria (`+Bio_Biochem`). Misconception:
  thinking a catalyst shifts equilibrium position.

### 9. Acids & Bases

- `GenChem::Acid_Base_Equilibria` — Arrhenius/Brønsted-Lowry/Lewis definitions,
  conjugate pairs, strong vs weak, Kw, pH/pOH, Ka/Kb/pKa, amphoterism, polyprotic
  acids. Reasoning: Application. Supports amino-acid/physiological pH
  (`+Bio_Biochem`). Misconception: pH of very dilute strong acids ignoring water.
- `GenChem::Buffers` — conjugate acid/base pairs, Henderson-Hasselbalch, buffer
  region and capacity, common-ion effect. Reasoning: Application. Supports blood
  bicarbonate buffering (`+Bio_Biochem`). Misconception: buffers "prevent any" pH
  change vs resist change.
- `GenChem::Titration` — strong/weak titration curves, equivalence vs half-
  equivalence (pH = pKa), indicators, polyprotic and back-titration. Reasoning:
  Data/Application. Supports amino-acid titration (`+Bio_Biochem`). Misconception:
  equating equivalence point with pH 7 for weak acid/base.

### 10. Electrochemistry

- `GenChem::Electrochemistry` — galvanic vs electrolytic cells, anode/cathode,
  standard reduction potentials, cell potential, ΔG = −nFE°, Nernst equation,
  Faraday/electrolysis stoichiometry, batteries (qualitative). Reasoning:
  Application. Supports electron transport chain/membrane potential
  (`+Bio_Biochem`). Misconception: anode/cathode sign confusion across cell types.

### 11. Nuclear

- `GenChem::Nuclear_Chemistry` — radioactive decay (α, β⁺/β⁻, γ, electron capture),
  balancing nuclear equations, half-life/decay kinetics, mass defect and binding
  energy, fission vs fusion. Reasoning: Application. Overlaps `Physics::Nuclear_Decay`.
  Misconception: mixing up particle emissions and their effect on Z/A.

## Name Reconciliation with `mcat.md`

This map keeps the widely-referenced `GenChem::` names and consolidates the looser
ones. The right column is a **docs-only suggestion** for the integration step; no
files other than this one were modified.

| `mcat.md` GenChem name | This map | Action |
|------------------------|----------|--------|
| `GenChem::Gas_Phase` | `GenChem::Gas_Phase` | keep |
| `GenChem::Stoichiometry` | `GenChem::Stoichiometry` | keep |
| `GenChem::Acid_Base_Equilibria` | `GenChem::Acid_Base_Equilibria` | keep |
| `GenChem::Ions_in_Solutions` | `GenChem::Ions_in_Solutions` | keep |
| `GenChem::Titration` | `GenChem::Titration` | keep |
| `GenChem::Intermolecular_Forces` | `GenChem::Intermolecular_Forces` | keep |
| `GenChem::Kinetics` | `GenChem::Kinetics` | keep |
| `GenChem::Equilibrium` | `GenChem::Equilibrium` | keep |
| `GenChem::Electrochemistry` | `GenChem::Electrochemistry` | keep |
| `GenChem::Thermochemistry` | `GenChem::Thermochemistry` | keep (adds `GenChem::Thermodynamics`) |
| `GenChem::Covalent_Bond` | `GenChem::Chemical_Bonding` | merge |
| `GenChem::Molecules` | `GenChem::Chemical_Bonding` / `GenChem::Molecular_Geometry` | merge |
| `GenChem::Molecular_Structure` | `GenChem::Molecular_Geometry` | rename |
| `GenChem::Water` | `GenChem::Intermolecular_Forces` / `GenChem::Solutions_and_Solubility` | merge |
| `GenChem::Liquid_Phase` | `GenChem::Phases_and_Phase_Changes` | merge |
| `GenChem::Solubility` | `GenChem::Solutions_and_Solubility` / `GenChem::Solubility_Equilibria` | split |

Cross-discipline edges in `mcat.md` that reference superseded names should be
repointed during integration (suggestion only):

- `Biochem::Protein_Structure_and_Function` prereq `GenChem::Covalent_Bond`
  → `GenChem::Chemical_Bonding`.
- `Biochem::Nucleotides_and_Nucleic_Acids` prereq `GenChem::Molecules`
  → `GenChem::Chemical_Bonding`.
- Edges already using `GenChem::Thermochemistry`, `GenChem::Electrochemistry`,
  `GenChem::Ions_in_Solutions`, and `GenChem::Gas_Phase` need no change.

## Items to Verify

- **(verify)** Electron-configuration exception list (e.g. Cr, Cu, and whether the
  MCAT expects them) before writing D4-D5 items.
- **(verify)** Whether cross-discipline `Physics::` soft edges should be authored as
  real graph edges or kept as lesson "related KC" links only; they are optional here.
- **(verify)** Placement of `GenChem::Spectrophotometry` — it can alternatively live
  under Orgo/analytical or Physics::Light; kept in Solutions for the Beer-Lambert
  concentration use-case.
- **(verify)** Whether `GenChem::Thermodynamics` should be a soft prerequisite of
  `GenChem::Equilibrium` (ΔG° = −RT ln K). Left out to keep the ladder pedagogical
  (kinetics → equilibrium) and to avoid over-coupling; the link is noted in scope.
- **(verify)** Support tags `+Bio_Biochem`: apply as per-card `MCAT::Bio_Biochem`
  overrides only on genuinely biochem-relevant items, since the default section
  derivation for `GenChem::` is `Chem_Phys` only.

## Research Notes

- **Framing/source discipline.** KCs follow AAMC-style General Chemistry coverage
  (foundational concepts around physical/chemical properties and reaction
  energetics) reorganized as a prerequisite lattice. All prose is synthetic and
  original; no copyrighted prep text was copied.
- **Granularity.** 26 KCs ≈ one per AAMC content subcategory, chosen to be
  card-generation-ready for Track B (each KC can host ~5-15 items across the
  difficulty ladder) while remaining coarse enough for readable scheduling.
- **Acyclicity.** Every internal edge points forward in the topological order
  above, so `KnowledgeGraph::cycle()` should return `None`. Cross-discipline edges
  only enter `GenChem::` nodes from `Physics::` nodes, so they cannot form a cycle
  within this set.
- **Section overlap rationale.** Default derivation sends `GenChem::` evidence to
  `Chem_Phys`. The `+Bio_Biochem` markers reflect the `0.05` GeneralChemistry weight
  in the Bio/Biochem blueprint and identify which KCs most justify a per-card
  `MCAT::Bio_Biochem` override (bonding, IMF, solutions/ions, thermo, kinetics,
  equilibrium, acid-base, buffers, titration, redox/electrochem, gases, colligative,
  spectrophotometry). Foundations with little biochem payoff (atomic structure,
  spectra, periodic trends, reaction types, phases, solubility equilibria, nuclear)
  stay `Chem_Phys`-only.
- **Difficulty ladder rationale.** Foundations start at `Difficulty::1` for
  calibration sampling; mechanism/application KCs begin at 2-3 and top out at 5 for
  multi-step quantitative and data-reasoning items (e.g. Nernst, Hess's law,
  titration curves, Beer-Lambert calibration). This aligns with the
  foundation-before-detail sequencing note in the expansion plan.
- **Reasoning coverage.** The set intentionally spans `Reasoning::Conceptual`
  (bonding, trends), `Reasoning::Application` (stoichiometry, thermo, equilibrium,
  acid-base), and `Reasoning::Data` (gas laws, kinetics graphs, phase/heating
  curves, spectrophotometry, titration curves). `Reasoning::ResearchDesign` is rare
  in pure GenChem and is left to passage-based items in Track B.
- **Integration guardrail.** Per the graph audit, add these KCs and edges in an
  audited batch, then run the graph-edge regression test and re-check the
  deck-options graph for readability before generating cards.

---

Report summary: 26 General Chemistry KCs across 11 clusters, 31 internal acyclic
prerequisite edges, plus 12 optional cross-discipline `Physics::` soft edges
(10 distinct Physics KCs) marked (verify). File:
`added features/research-kc-general-chemistry.md`.
