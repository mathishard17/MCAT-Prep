# Research KC Map — MCAT Biochemistry (Track A)

Machine-usable Knowledge Component (KC) map for MCAT **Biochemistry**, produced by
the Biochemistry research subagent for Track A of the Concept Scheduler expansion.
This is **docs-only** research: it does not change the canonical graph, scheduler
code, or demo content. It refines and extends the draft lattice in
`added features/mcat.md` and stays consistent with the canonical demo graph in
`added features/mcat-graph-audit.md`.

All content here is synthetic/original phrasing, not copied from copyrighted prep
material. Uncertain items are flagged `(verify)`.

## How to read this file

- **KC id** is shown bare (e.g. `Biochem::Enzymes`). On a card the target concept
  is tagged `KC::Biochem::Enzymes`; each prerequisite is tagged
  `Prereq::<KC id>` (e.g. `Prereq::GenChem::Kinetics`). This matches the tag
  parsing in `rslib/src/scheduler/concept.rs` and `qt/aqt/concept_tags.py`.
- **Prerequisites** list the full intended prerequisite set (intra-Biochem +
  cross-discipline). Edges are authored as `prerequisite -> target`, the same
  convention as the graph audit; the internal store calls
  `add_prerequisite(target, prerequisite)`.
- **Overlapping MCAT sections**: every `Biochem::` KC maps to **both**
  `Bio_Biochem` and `Chem_Phys` (see `derived_mcat_sections_for_topics`, which
  returns `["Bio_Biochem", "Chem_Phys"]` for `Biochem::`). A `*` marks the
  section where the KC contributes the most evidence — the recommended primary
  emphasis when tagging cards. Both tags should still be applied.
- **Difficulty ladder** uses `Difficulty::1..5`. Notation `a→b→c` is the
  recommended spread of card difficulties for that KC, from first-encounter
  calibration cards up to the hardest reasoning cards. Author roughly:
  2 easy (a) → 2 medium (b) → 1 hard (c), matching the demo allocation pattern.
- **Type** is one of `foundation | mechanism | application | detail`, driving the
  foundation-before-detail sequencing rule from the expansion plan.
- **Status**: `demo` = one of the 7 canonical demo KCs (reused **exactly**,
  edges preserved); `existing` = already named in the `mcat.md` draft lattice
  (reused to avoid duplicates); `new` = introduced by this research pass.

## KC count summary

- Total KCs: **25**
- Reused canonical demo Biochem KCs (unchanged): **7**
- Reused existing draft Biochem KCs: **7**
- New Biochem KCs proposed: **11**

---

## KC table

### Cluster: Amino Acids & Proteins

| KC id | Parent cluster | Prerequisites (KC ids) | Overlapping MCAT sections | Difficulty ladder | Type | Status | Scope / notes |
|---|---|---|---|---|---|---|---|
| `Biochem::Amino_Acids` | Amino Acids & Proteins | `GenChem::Acid_Base_Equilibria`, `GenChem::Covalent_Bond`, `Orgo::Functional_Groups` | `Bio_Biochem*`, `Chem_Phys` | 1→2→3 | foundation | demo | 20 standard amino acids, side-chain classification (nonpolar/polar/acidic/basic), zwitterions, pKa and isoelectric point (pI), amino-acid titration curves. |
| `Biochem::Peptides_and_Proteins` | Amino Acids & Proteins | `Biochem::Amino_Acids`, `Orgo::Acid_Derivatives` | `Bio_Biochem*`, `Chem_Phys` | 1→2→3 | foundation | demo | Peptide (amide) bond formation and hydrolysis, primary sequence, N-/C-termini, directionality, peptide nomenclature. |
| `Biochem::Protein_Structure_and_Function` | Amino Acids & Proteins | `Biochem::Peptides_and_Proteins`, `GenChem::Covalent_Bond`, `GenChem::Intermolecular_Forces` | `Bio_Biochem*`, `Chem_Phys` | 2→3→4 | mechanism | demo | Primary/secondary/tertiary/quaternary structure, α-helix and β-sheet, disulfide bonds, domains and motifs, structure–function link. Preserves the `GenChem::Covalent_Bond` prereq from the draft. |
| `Biochem::Protein_Folding_and_Stability` | Amino Acids & Proteins | `Biochem::Protein_Structure_and_Function`, `GenChem::Intermolecular_Forces`, `GenChem::Thermochemistry` | `Bio_Biochem`, `Chem_Phys*` | 3→4→5 | mechanism | new | Folding thermodynamics, hydrophobic effect, chaperones, denaturation/renaturation, misfolding. Chem/Phys primary because it leans on entropy/enthalpy reasoning. |
| `Biochem::Nonenzymatic_Protein_Function` | Amino Acids & Proteins | `Biochem::Protein_Structure_and_Function` | `Bio_Biochem*`, `Chem_Phys` | 3→4→5 | application | new | Binding proteins and cooperativity (myoglobin vs hemoglobin, O2 dissociation, Bohr effect), immunoglobulins, motor proteins, cytoskeletal proteins. |

### Cluster: Enzymes

| KC id | Parent cluster | Prerequisites (KC ids) | Overlapping MCAT sections | Difficulty ladder | Type | Status | Scope / notes |
|---|---|---|---|---|---|---|---|
| `Biochem::Enzymes` | Enzymes | `Biochem::Protein_Structure_and_Function` | `Bio_Biochem*`, `Chem_Phys` | 2→3→4 | mechanism | demo | Catalysis and activation-energy lowering, active site, induced fit vs lock-and-key, enzyme classes (EC families), cofactors/coenzymes overview. |
| `Biochem::Enzyme_Kinetics` | Enzymes | `Biochem::Enzymes`, `GenChem::Kinetics` | `Bio_Biochem`, `Chem_Phys*` | 3→4→5 | mechanism | new | Michaelis–Menten model, Km, Vmax, kcat, catalytic efficiency, Lineweaver–Burk plots, cooperative kinetics / Hill coefficient. Chem/Phys primary because it is quantitative/graphical. |
| `Biochem::Enzyme_Inhibition` | Enzymes | `Biochem::Enzyme_Kinetics` | `Bio_Biochem`, `Chem_Phys*` | 3→4→5 | mechanism | new | Competitive, noncompetitive, uncompetitive, and mixed inhibition (effect on Km/Vmax); allosteric regulation, feedback inhibition, covalent modification, zymogen activation. |

### Cluster: Bioenergetics & Regulation

| KC id | Parent cluster | Prerequisites (KC ids) | Overlapping MCAT sections | Difficulty ladder | Type | Status | Scope / notes |
|---|---|---|---|---|---|---|---|
| `Biochem::Bioenergetics` | Bioenergetics & Regulation | `Biochem::Enzymes`, `Bio::Eukaryotic_Cell`, `GenChem::Thermochemistry` | `Bio_Biochem`, `Chem_Phys*` | 2→3→4 | mechanism | demo | ΔG vs ΔG°′, spontaneity, ATP as energy currency, phosphoryl-transfer potential, reaction coupling, redox carriers (NAD+/NADH, FAD/FADH2). Preserves both existing demo edges into this node. Chem/Phys primary (thermodynamics). |
| `Biochem::Metabolic_Regulation` | Bioenergetics & Regulation | `Biochem::Enzymes`, `Biochem::Bioenergetics` | `Bio_Biochem*`, `Chem_Phys` | 3→4→5 | mechanism | existing | Rate-limiting/committed steps, allosteric and hormonal control points, compartmentalization, energy-charge signals (ATP/AMP, NADH/NAD+), reciprocal regulation concept. Matches draft prereqs. |

### Cluster: Carbohydrates & Lipids (structure)

| KC id | Parent cluster | Prerequisites (KC ids) | Overlapping MCAT sections | Difficulty ladder | Type | Status | Scope / notes |
|---|---|---|---|---|---|---|---|
| `Biochem::Carbohydrates_and_Lipids` | Carbohydrates & Lipids (structure) | `Orgo::Functional_Groups`, `Orgo::Stereochemistry`, `Orgo::Aldehydes_and_Ketones` | `Bio_Biochem*`, `Chem_Phys` | 1→2→3 | foundation | existing | Monosaccharides (aldoses/ketoses, D/L, anomers, epimers), glycosidic bonds, di-/polysaccharides; fatty acids, triacylglycerols, phospholipids, sphingolipids, steroids, terpenes. Existing **combined** structural KC; see Research notes for a possible future split. |

### Cluster: Carbohydrate Metabolism

| KC id | Parent cluster | Prerequisites (KC ids) | Overlapping MCAT sections | Difficulty ladder | Type | Status | Scope / notes |
|---|---|---|---|---|---|---|---|
| `Biochem::Glycolysis` | Carbohydrate Metabolism | `Biochem::Bioenergetics`, `Biochem::Carbohydrates_and_Lipids` | `Bio_Biochem*`, `Chem_Phys` | 2→3→4 | mechanism | demo | Ten-step overview, investment vs payoff phases, PFK-1 regulation, substrate-level phosphorylation, aerobic vs anaerobic (lactate/fermentation) fates, feeder sugars. Preserves demo edge from `Biochem::Bioenergetics`. |
| `Biochem::Gluconeogenesis` | Carbohydrate Metabolism | `Biochem::Glycolysis`, `Biochem::Metabolic_Regulation` | `Bio_Biochem*`, `Chem_Phys` | 3→4→5 | mechanism | existing | Four bypass enzymes, reciprocal regulation with glycolysis, substrates (lactate, glycerol, glucogenic amino acids), Cori cycle, tissue localization. Matches draft prereqs. |
| `Biochem::Glycogen_Metabolism` | Carbohydrate Metabolism | `Biochem::Glycolysis`, `Biochem::Metabolic_Regulation`, `Biochem::Carbohydrates_and_Lipids` | `Bio_Biochem*`, `Chem_Phys` | 3→4→5 | mechanism | new | Glycogenesis and glycogenolysis, glycogen synthase vs phosphorylase, branching/debranching, hormonal control, liver vs muscle roles. |
| `Biochem::Pentose_Phosphate_Pathway` | Carbohydrate Metabolism | `Biochem::Glycolysis`, `Biochem::Nucleotides_and_Nucleic_Acids` | `Bio_Biochem*`, `Chem_Phys` | 3→4→5 | detail | existing | Oxidative phase (NADPH), non-oxidative phase (ribose-5-phosphate for nucleotides), links to biosynthesis and oxidative-stress defense. Matches draft prereqs. |

### Cluster: Citric Acid Cycle & Oxidative Phosphorylation

| KC id | Parent cluster | Prerequisites (KC ids) | Overlapping MCAT sections | Difficulty ladder | Type | Status | Scope / notes |
|---|---|---|---|---|---|---|---|
| `Biochem::Citric_Acid_Cycle` | Citric Acid Cycle & OxPhos | `Biochem::Bioenergetics`, `Biochem::Glycolysis` | `Bio_Biochem*`, `Chem_Phys` | 2→3→4 | mechanism | demo | Acetyl-CoA entry, per-turn yield (NADH, FADH2, GTP, CO2), regulation (isocitrate dehydrogenase etc.), anaplerotic and cataplerotic reactions. Preserves both demo edges into this node. |
| `Biochem::Oxidative_Phosphorylation` | Citric Acid Cycle & OxPhos | `Biochem::Citric_Acid_Cycle`, `GenChem::Electrochemistry`, `Bio::Eukaryotic_Cell` | `Bio_Biochem`, `Chem_Phys*` | 3→4→5 | mechanism | existing | Electron transport chain complexes, proton-motive force, chemiosmosis, ATP synthase, approximate P/O ratios, uncouplers and ETC inhibitors. Chem/Phys primary (redox potentials, energetics). Adds mitochondrial-context prereq `Bio::Eukaryotic_Cell` `(verify)`. |

### Cluster: Lipid & Amino Acid Metabolism

| KC id | Parent cluster | Prerequisites (KC ids) | Overlapping MCAT sections | Difficulty ladder | Type | Status | Scope / notes |
|---|---|---|---|---|---|---|---|
| `Biochem::Lipid_Metabolism` | Lipid & Amino Acid Metabolism | `Biochem::Carbohydrates_and_Lipids`, `Biochem::Bioenergetics`, `Biochem::Citric_Acid_Cycle` | `Bio_Biochem*`, `Chem_Phys` | 3→4→5 | mechanism | existing | β-oxidation, fatty-acid synthesis, ketone-body formation/use, lipid transport (chylomicrons/lipoproteins), cholesterol overview. Extends draft prereqs with `Biochem::Citric_Acid_Cycle` (acetyl-CoA link). |
| `Biochem::Amino_Acid_Metabolism` | Lipid & Amino Acid Metabolism | `Biochem::Amino_Acids`, `Biochem::Citric_Acid_Cycle`, `Biochem::Metabolic_Regulation` | `Bio_Biochem*`, `Chem_Phys` | 3→4→5 | detail | new | Transamination and oxidative deamination, urea cycle overview, glucogenic vs ketogenic amino acids, entry of carbon skeletons into the TCA cycle. |

### Cluster: Nucleic Acids

| KC id | Parent cluster | Prerequisites (KC ids) | Overlapping MCAT sections | Difficulty ladder | Type | Status | Scope / notes |
|---|---|---|---|---|---|---|---|
| `Biochem::Nucleotides_and_Nucleic_Acids` | Nucleic Acids | `Bio::DNA`, `GenChem::Molecules` | `Bio_Biochem*`, `Chem_Phys` | 1→2→3 | foundation | existing | Nucleotide structure, purines vs pyrimidines, phosphodiester backbone, base pairing, DNA vs RNA, denaturation and melting temperature (Tm). Gene expression/replication mechanics stay under `Bio::` (e.g. `Bio::DNA`, `Bio::Genetics`) to avoid overlap. Matches draft prereqs. |

### Cluster: Membranes & Transport

| KC id | Parent cluster | Prerequisites (KC ids) | Overlapping MCAT sections | Difficulty ladder | Type | Status | Scope / notes |
|---|---|---|---|---|---|---|---|
| `Biochem::Membranes_and_Transport` | Membranes & Transport | `Biochem::Carbohydrates_and_Lipids`, `Biochem::Protein_Structure_and_Function`, `Bio::Eukaryotic_Cell` | `Bio_Biochem*`, `Chem_Phys` | 2→3→4 | mechanism | new | Fluid mosaic model, lipid bilayer and fluidity, passive vs facilitated diffusion, primary/secondary active transport, membrane potential, transport/channel proteins. Overlaps `Bio::` cell membrane content; kept in Biochem for the lipid+protein biochemistry emphasis. |

### Cluster: Laboratory Techniques

| KC id | Parent cluster | Prerequisites (KC ids) | Overlapping MCAT sections | Difficulty ladder | Type | Status | Scope / notes |
|---|---|---|---|---|---|---|---|
| `Biochem::Chromatography_and_Separations` | Laboratory Techniques | `Biochem::Peptides_and_Proteins`, `Orgo::Separations_and_Purifications`, `GenChem::Intermolecular_Forces` | `Bio_Biochem`, `Chem_Phys*` | 3→4→5 | application | new | Size-exclusion, ion-exchange, and affinity chromatography, HPLC; centrifugation and dialysis for protein isolation. Chem/Phys primary (content category 5C, separations). Cross-listed with `Orgo::Separations_and_Purifications`. |
| `Biochem::Electrophoresis_and_Immunoassays` | Laboratory Techniques | `Biochem::Peptides_and_Proteins`, `Biochem::Nucleotides_and_Nucleic_Acids`, `GenChem::Acid_Base_Equilibria` | `Bio_Biochem`, `Chem_Phys*` | 3→4→5 | application | new | SDS-PAGE and native PAGE, isoelectric focusing, agarose gel electrophoresis of nucleic acids; blotting (Western/Southern/Northern) and ELISA. Chem/Phys primary (charge/pH-driven migration). |

### Cluster: Cofactors & Integration

| KC id | Parent cluster | Prerequisites (KC ids) | Overlapping MCAT sections | Difficulty ladder | Type | Status | Scope / notes |
|---|---|---|---|---|---|---|---|
| `Biochem::Vitamins_and_Cofactors` | Cofactors & Integration | `Biochem::Enzymes` | `Bio_Biochem*`, `Chem_Phys` | 2→3→4 | detail | new | Water- and fat-soluble vitamins as coenzyme precursors (NAD+/NADP+, FAD, coenzyme A, TPP, PLP, biotin, tetrahydrofolate), plus metal-ion cofactors and their catalytic roles. |
| `Biochem::Hormonal_Regulation_of_Metabolism` | Cofactors & Integration | `Biochem::Metabolic_Regulation`, `Biochem::Gluconeogenesis`, `Biochem::Lipid_Metabolism`, `Bio::Endocrine_System` | `Bio_Biochem*`, `Chem_Phys` | 3→4→5 | application | new | Insulin, glucagon, epinephrine, and cortisol; fed vs fasting vs starvation states; tissue-specific metabolism (liver, muscle, adipose, brain) and pathway integration. `Bio::Endocrine_System` prereq `(verify)`. Capstone integration KC. |

---

## Prerequisite edges

Authored as `prerequisite -> target` (the graph-audit convention). Cross-discipline
edges reference existing `Bio::`, `GenChem::`, and `Orgo::` KC ids from
`added features/mcat.md`.

### Existing demo edges (unchanged — reused exactly)

```text
Biochem::Amino_Acids -> Biochem::Peptides_and_Proteins
Biochem::Peptides_and_Proteins -> Biochem::Protein_Structure_and_Function
Biochem::Protein_Structure_and_Function -> Biochem::Enzymes
Biochem::Enzymes -> Biochem::Bioenergetics
Bio::Eukaryotic_Cell -> Biochem::Bioenergetics
Biochem::Bioenergetics -> Biochem::Glycolysis
Biochem::Bioenergetics -> Biochem::Citric_Acid_Cycle
Biochem::Glycolysis -> Biochem::Citric_Acid_Cycle
```

### New intra-Biochem edges

```text
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

### New cross-discipline edges (into Biochem targets)

```text
GenChem::Acid_Base_Equilibria -> Biochem::Amino_Acids
GenChem::Covalent_Bond -> Biochem::Amino_Acids
Orgo::Functional_Groups -> Biochem::Amino_Acids
Orgo::Acid_Derivatives -> Biochem::Peptides_and_Proteins
GenChem::Covalent_Bond -> Biochem::Protein_Structure_and_Function
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
GenChem::Molecules -> Biochem::Nucleotides_and_Nucleic_Acids
Bio::Eukaryotic_Cell -> Biochem::Membranes_and_Transport
Orgo::Separations_and_Purifications -> Biochem::Chromatography_and_Separations
GenChem::Intermolecular_Forces -> Biochem::Chromatography_and_Separations
GenChem::Acid_Base_Equilibria -> Biochem::Electrophoresis_and_Immunoassays
Bio::Endocrine_System -> Biochem::Hormonal_Regulation_of_Metabolism
```

## Cross-discipline prerequisites referenced

These are the non-Biochem KC ids this map depends on. All already exist in
`added features/mcat.md`; none are newly invented here.

- **GenChem:** `GenChem::Acid_Base_Equilibria`, `GenChem::Covalent_Bond`,
  `GenChem::Intermolecular_Forces`, `GenChem::Thermochemistry`,
  `GenChem::Kinetics`, `GenChem::Electrochemistry`, `GenChem::Molecules`
- **Orgo:** `Orgo::Functional_Groups`, `Orgo::Stereochemistry`,
  `Orgo::Aldehydes_and_Ketones`, `Orgo::Acid_Derivatives`,
  `Orgo::Separations_and_Purifications`
- **Bio:** `Bio::DNA`, `Bio::Eukaryotic_Cell`, `Bio::Endocrine_System` `(verify)`

## Acyclicity check

The graph is a DAG. Structural foundations (`Amino_Acids`,
`Carbohydrates_and_Lipids`, `Nucleotides_and_Nucleic_Acids`) depend only on
out-of-map GenChem/Orgo/Bio roots. Every new KC points *forward only* (each is a
sink or depends solely on earlier-defined KCs), so no back-edges are introduced
and the existing demo spine is untouched. This is consistent with the cycle
detector in `KnowledgeGraph::cycle` and the graph-edge regression test noted in
`mcat-graph-audit.md`.

## Section-overlap rationale

Per `derived_mcat_sections_for_topics`, all `Biochem::` KCs contribute to both
`Bio_Biochem` and `Chem_Phys`, and the discipline weights in `concept.rs`
already give Biochemistry meaningful weight in **both** sections
(`Bio_Biochem`: 0.25; `Chem_Phys`: 0.25). The `*` primary markers above are a
tagging/authoring hint only (they do not change the derived tags):

- **Bio/Biochem-leaning:** structure, metabolism pathways, regulation,
  membranes — the "how the biology works" emphasis.
- **Chem/Phys-leaning:** enzyme kinetics, folding/bioenergetics thermodynamics,
  oxidative phosphorylation redox, and all lab techniques (AAMC content
  category 5C separations and quantitative analysis).

## Research notes

- **Demo KCs preserved exactly.** The 7 canonical demo Biochem KCs
  (`Amino_Acids`, `Peptides_and_Proteins`, `Protein_Structure_and_Function`,
  `Enzymes`, `Bioenergetics`, `Glycolysis`, `Citric_Acid_Cycle`) reuse their ids
  and keep every existing edge, including `Bio::Eukaryotic_Cell -> Bioenergetics`
  and the dual `Bioenergetics/Glycolysis -> Citric_Acid_Cycle` edges. Where this
  map lists extra prerequisites for a demo KC, they are **additive** and never
  reverse an existing edge.
- **Reused draft KCs.** `Metabolic_Regulation`, `Carbohydrates_and_Lipids`,
  `Gluconeogenesis`, `Pentose_Phosphate_Pathway`, `Oxidative_Phosphorylation`,
  `Lipid_Metabolism`, and `Nucleotides_and_Nucleic_Acids` were already named in
  `mcat.md`; ids and draft prereqs are reused to prevent duplicate concepts.
- **Combined carbohydrate/lipid KC.** `Biochem::Carbohydrates_and_Lipids` is kept
  as the single existing structural foundation. A future audited batch could
  split it into `Biochem::Carbohydrates` and `Biochem::Lipids` for finer
  granularity (membranes and lipid metabolism would then depend on the lipid
  half, feeder sugars on the carbohydrate half). Deferred to avoid churning an
  id other tracks may already reference. `(verify)` before splitting.
- **Nucleic-acid boundary.** Only nucleotide/nucleic-acid *chemistry and
  structure* live under `Biochem::`. Replication, transcription, translation, and
  regulation of gene expression are intentionally left under `Bio::` (`Bio::DNA`,
  `Bio::Genetics`, `Bio::Biotechnology`) to keep sections clean and avoid
  double-counting.
- **Membranes overlap.** `Biochem::Membranes_and_Transport` overlaps cell-biology
  membrane content that could also sit under `Bio::`. It is placed in Biochem for
  the lipid-bilayer + transport-protein biochemistry emphasis; if a `Bio::`
  membrane KC is later authored, dedupe rather than duplicate. `(verify)`
- **Difficulty ladders** are pedagogical suggestions for card authoring, not
  IRT `b` parameters. The scheduler maps `Difficulty::1..5` to IRT difficulty in
  `difficulty_to_irt_b` (1→-2.0 … 5→+2.0); ladders here are chosen so each KC
  seeds calibration-friendly easy cards before harder reasoning items.
- **`(verify)` items** are the `Bio::Endocrine_System` prerequisite for hormonal
  integration, the `Bio::Eukaryotic_Cell` prerequisite added to
  `Oxidative_Phosphorylation`, the membranes placement, and the potential future
  carbohydrate/lipid split. These are reasonable modeling choices flagged for a
  human graph-authoring review before they enter the canonical graph.
- **Content is synthetic.** All scope descriptions are original paraphrases of
  widely known biochemistry; no copyrighted prep text is reproduced.
