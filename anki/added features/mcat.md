# MCAT Knowledge Component Draft

This file is a working topic map for the Concept Scheduler MVP. It is not the final MCAT taxonomy yet. The goal is to turn the flat topic list into a prerequisite-friendly Knowledge Component (KC) graph that can support calibration, inner/outer fringe tracking, and a fake 50-card demo deck.

## MVP Tagging Convention

Use structured Anki tags while the final input format is still being researched:

- `KC::Bio::Eukaryotic_Cell`
- `Prereq::Bio::DNA`
- `MCAT::Bio_Biochem`
- `Difficulty::1` through `Difficulty::5`

Each demo card should have at least one `KC::...` tag. Cards that depend on earlier concepts should also include one or more `Prereq::...` tags.

## Biology and Biochemistry KC Lattice

### Layer 1: Foundations

These are calibration-friendly prerequisite KCs. Early cards should sample these first, with some randomness.

- `Bio::Eukaryotic_Cell`
- `Bio::Prokaryotes_vs_Eukaryotes`
- `Bio::Viruses`
- `Bio::DNA`
- `Biochem::Amino_Acids`
- `Biochem::Carbohydrates_and_Lipids`
- `GenChem::Molecules`
- `GenChem::Covalent_Bond`
- `GenChem::Acid_Base_Equilibria`

### Layer 2: Molecular and Cellular Mechanisms

These become outer-fringe candidates once the relevant foundations are strong enough.

- `Bio::Genetics`
  - Prereqs: `Bio::DNA`
- `Bio::Evolution`
  - Prereqs: `Bio::Genetics`
- `Bio::Biotechnology`
  - Prereqs: `Bio::DNA`, `Bio::Genetics`
- `Biochem::Nucleotides_and_Nucleic_Acids`
  - Prereqs: `Bio::DNA`, `GenChem::Molecules`
- `Biochem::Peptides_and_Proteins`
  - Prereqs: `Biochem::Amino_Acids`
- `Biochem::Protein_Structure_and_Function`
  - Prereqs: `Biochem::Peptides_and_Proteins`, `GenChem::Covalent_Bond`
- `Biochem::Enzymes`
  - Prereqs: `Biochem::Protein_Structure_and_Function`
- `Biochem::Bioenergetics`
  - Prereqs: `GenChem::Thermochemistry`, `Biochem::Enzymes`
- `Biochem::Metabolic_Regulation`
  - Prereqs: `Biochem::Enzymes`, `Biochem::Bioenergetics`

### Layer 3: Core Metabolism

These should usually appear after enzymes and bioenergetics.

- `Biochem::Glycolysis`
  - Prereqs: `Biochem::Carbohydrates_and_Lipids`, `Biochem::Bioenergetics`
- `Biochem::Gluconeogenesis`
  - Prereqs: `Biochem::Glycolysis`, `Biochem::Metabolic_Regulation`
- `Biochem::Citric_Acid_Cycle`
  - Prereqs: `Biochem::Glycolysis`, `Biochem::Bioenergetics`
- `Biochem::Oxidative_Phosphorylation`
  - Prereqs: `Biochem::Citric_Acid_Cycle`, `GenChem::Electrochemistry`
- `Biochem::Pentose_Phosphate_Pathway`
  - Prereqs: `Biochem::Glycolysis`, `Biochem::Nucleotides_and_Nucleic_Acids`
- `Biochem::Lipid_Metabolism`
  - Prereqs: `Biochem::Carbohydrates_and_Lipids`, `Biochem::Bioenergetics`

### Layer 4: Organ Systems

These are later biology KCs. They depend on cellular biology, basic chemistry, and sometimes specific physics concepts.

- `Bio::Nervous_System`
  - Prereqs: `Bio::Eukaryotic_Cell`, `GenChem::Ions_in_Solutions`, `Physics::Electrical_Circuits`
- `Bio::Endocrine_System`
  - Prereqs: `Bio::Eukaryotic_Cell`, `Biochem::Protein_Structure_and_Function`
- `Bio::Muscular_System`
  - Prereqs: `Bio::Eukaryotic_Cell`, `Biochem::Bioenergetics`
- `Bio::Skeletal_System`
  - Prereqs: `Bio::Eukaryotic_Cell`, `GenChem::Ions_in_Solutions`
- `Bio::Circulatory_System`
  - Prereqs: `Bio::Eukaryotic_Cell`, `Physics::Fluids`
- `Bio::Respiratory_System`
  - Prereqs: `Bio::Eukaryotic_Cell`, `GenChem::Gas_Phase`
- `Bio::Digestive_System`
  - Prereqs: `Bio::Eukaryotic_Cell`, `Biochem::Enzymes`
- `Bio::Immune_System`
  - Prereqs: `Bio::Eukaryotic_Cell`, `Biochem::Protein_Structure_and_Function`
- `Bio::Lymphatic_System`
  - Prereqs: `Bio::Immune_System`, `Bio::Circulatory_System`
- `Bio::Skin_System`
  - Prereqs: `Bio::Eukaryotic_Cell`
- `Bio::Reproductive_System`
  - Prereqs: `Bio::Genetics`, `Bio::Endocrine_System`
- `Bio::Embryology`
  - Prereqs: `Bio::Reproductive_System`, `Bio::Genetics`

## Supporting Chemistry and Physics KCs

These are not the first demo focus, but they are useful prerequisites for biology and biochemistry.

### General Chemistry

- `GenChem::Gas_Phase`
- `GenChem::Electrochemistry`
- `GenChem::Molecular_Structure`
- `GenChem::Stoichiometry`
- `GenChem::Acid_Base_Equilibria`
- `GenChem::Solubility`
- `GenChem::Ions_in_Solutions`
- `GenChem::Titration`
- `GenChem::Covalent_Bond`
- `GenChem::Liquid_Phase`
- `GenChem::Intermolecular_Forces`
- `GenChem::Kinetics`
- `GenChem::Equilibrium`
- `GenChem::Water`
- `GenChem::Molecules`
- `GenChem::Thermochemistry`

### Physics

- `Physics::Translational_Motion`
- `Physics::Force`
- `Physics::Equilibrium`
- `Physics::Work`
- `Physics::Energy`
- `Physics::Periodic_Motion`
- `Physics::Fluids`
- `Physics::Electrostatics`
- `Physics::Electromagnetic_Radiation`
- `Physics::Electrical_Circuits`
- `Physics::Circuit_Elements`
- `Physics::Light`
- `Physics::Magnetism`
- `Physics::Sound`
- `Physics::Matter`
- `Physics::Atoms`
- `Physics::Nuclear_Decay`
- `Physics::Electronic_Structure`
- `Physics::Atomic_and_Chemical_Behavior`
- `Physics::Thermodynamics`
- `Physics::Optics`

### Organic Chemistry

- `Orgo::Functional_Groups`
- `Orgo::Nomenclature`
- `Orgo::Stereochemistry`
- `Orgo::Hybridization`
- `Orgo::Nucleophilic_Substitution`
- `Orgo::Molecular_Structure_and_Absorption_Spectra`
- `Orgo::Aldehydes_and_Ketones`
- `Orgo::Alcohols`
- `Orgo::Carboxylic_Acids`
- `Orgo::Acid_Derivatives`
- `Orgo::Mass_Spectrometry`
- `Orgo::Polycyclic_and_Heterocyclic_Aromatic_Compounds`
- `Orgo::Phenols`
- `Orgo::Separations_and_Purifications`

## Psychology and Sociology KCs

These can become a separate lattice after the Bio/Biochem demo works.

- `PsychSoc::Sensory_Processing`
- `PsychSoc::The_Senses`
- `PsychSoc::Perception`
- `PsychSoc::Attention`
- `PsychSoc::Cognition`
- `PsychSoc::Consciousness`
- `PsychSoc::Memory`
- `PsychSoc::Language`
- `PsychSoc::Emotion`
- `PsychSoc::Stress`
- `PsychSoc::Personality`
- `PsychSoc::Psychological_Disorders`
- `PsychSoc::Motivation`
- `PsychSoc::Attitudes_and_Beliefs`
- `PsychSoc::Biological_and_Social_Factors`
- `PsychSoc::Self_and_Identity`
- `PsychSoc::Prejudice_and_Bias`
- `PsychSoc::Stereotypes`
- `PsychSoc::Social_Class`
- `PsychSoc::Stratification`
- `PsychSoc::Social_Mobility`
- `PsychSoc::Poverty`
- `PsychSoc::Culture`
- `PsychSoc::Health_Disparities`

## Fake 50-Card Demo Seed

Use this small graph first so the scheduler can be tested without needing a full MCAT deck.

### Demo KCs

- `Bio::DNA`
- `Bio::Genetics`
- `Bio::Eukaryotic_Cell`
- `Biochem::Amino_Acids`
- `Biochem::Peptides_and_Proteins`
- `Biochem::Protein_Structure_and_Function`
- `Biochem::Enzymes`
- `Biochem::Bioenergetics`
- `Biochem::Glycolysis`
- `Biochem::Citric_Acid_Cycle`

### Demo Prerequisite Edges

- `Bio::DNA` -> `Bio::Genetics`
- `Biochem::Amino_Acids` -> `Biochem::Peptides_and_Proteins`
- `Biochem::Peptides_and_Proteins` -> `Biochem::Protein_Structure_and_Function`
- `Biochem::Protein_Structure_and_Function` -> `Biochem::Enzymes`
- `Biochem::Enzymes` -> `Biochem::Bioenergetics`
- `Biochem::Bioenergetics` -> `Biochem::Glycolysis`
- `Biochem::Glycolysis` -> `Biochem::Citric_Acid_Cycle`
- `Bio::Eukaryotic_Cell` -> `Biochem::Bioenergetics`

### Demo Card Allocation

Create about 5 cards per demo KC for a 50-card deck. Difficulty should increase within each KC:

- 2 easy prerequisite/calibration cards
- 2 medium application cards
- 1 harder reasoning card

This makes the demo large enough to show calibration, prerequisite violations, outer-fringe recommendations, and fallback review behavior without requiring real medical expertise yet.
