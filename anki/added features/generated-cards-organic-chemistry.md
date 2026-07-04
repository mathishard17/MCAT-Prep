# MCAT Organic Chemistry Generated Cards

Synthetic, original multiple-choice cards for the Concept Scheduler, covering
every Organic Chemistry (`Orgo::`) Knowledge Component in the frozen unified KC
map (`added features/kc-map-unified.md` §6/§7). One card per KC (27 KCs → 27
cards). Block format is identical to `added features/mcat_demo_cards.md` so these
import through the same parser (`rslib/src/scheduler/concept_demo.rs`).

- IDs use the deterministic scheme `MCAT-ORG-<TOPIC>-NNN`, where `<TOPIC>` is the
  KC's stable `kc_code` suffix from the unified map (unique per KC).
- Every `KC::` tag equals the KC id exactly; every `Prereq::` set equals the
  frozen graph's prerequisite edges for that KC (including cross-discipline
  `GenChem::` / `Physics::` prerequisites).
- Primary section is `MCAT::Chem_Phys` for all `Orgo::` KCs (the unified map's
  primary section for the whole discipline).
- Content is synthetic/original and not copied from any copyrighted prep material.

## Tags

- Section tag: `MCAT::Chem_Phys`
- KC tag: `KC::Orgo::<Topic>`
- Prerequisite tag: `Prereq::<Area>::<Topic>`
- Difficulty tag: `Difficulty::<1-5>`

---

## Orgo::Hybridization

### MCAT-ORG-HYB-001

- **KC:** `Orgo::Hybridization`
- **Prereqs:** `Prereq::GenChem::Chemical_Bonding` `Prereq::GenChem::Molecular_Geometry`
- **Difficulty:** 2
- **Question:** The carbonyl carbon of an amide is best described by which hybridization and approximate geometry?
- **A:** sp3, about 109.5 degrees, tetrahedral
- **B:** sp, about 180 degrees, linear
- **C:** sp2, about 120 degrees, trigonal planar
- **D:** sp3d, about 90 degrees, trigonal bipyramidal
- **Correct:** C
- **Explanation:** A carbon forming one double bond and two single bonds has three sigma bonds and one pi bond, so it is sp2 hybridized with trigonal planar geometry near 120 degrees.
- **Tags:** `KC::Orgo::Hybridization` `Prereq::GenChem::Chemical_Bonding` `Prereq::GenChem::Molecular_Geometry` `MCAT::Chem_Phys` `Difficulty::2`

## Orgo::Functional_Groups

### MCAT-ORG-FUNG-001

- **KC:** `Orgo::Functional_Groups`
- **Prereqs:** `Prereq::GenChem::Chemical_Bonding` `Prereq::Orgo::Hybridization`
- **Difficulty:** 2
- **Question:** A molecule contains a single carbon that is double-bonded to one oxygen and single-bonded to a hydroxyl (-OH) group. Which functional group is present?
- **A:** Ketone
- **B:** Aldehyde
- **C:** Carboxylic acid
- **D:** Ester
- **Correct:** C
- **Explanation:** A carbon bearing both a carbonyl (C=O) and a hydroxyl (-OH) on the same atom defines a carboxylic acid.
- **Tags:** `KC::Orgo::Functional_Groups` `Prereq::GenChem::Chemical_Bonding` `Prereq::Orgo::Hybridization` `MCAT::Chem_Phys` `Difficulty::2`

## Orgo::Nomenclature

### MCAT-ORG-NOM-001

- **KC:** `Orgo::Nomenclature`
- **Prereqs:** `Prereq::Orgo::Functional_Groups` `Prereq::Orgo::Hybridization`
- **Difficulty:** 2
- **Question:** When numbering the parent chain of a substituted alkane, what is the guiding IUPAC rule?
- **A:** Number from the end that gives substituents the lowest set of locants
- **B:** Number from the end nearest the substituent that comes first alphabetically
- **C:** Always number the chain from left to right as drawn
- **D:** Number so that substituents receive the highest possible locants
- **Correct:** A
- **Explanation:** IUPAC numbering assigns locants so the substituents or principal characteristic group receive the lowest set of numbers.
- **Tags:** `KC::Orgo::Nomenclature` `Prereq::Orgo::Functional_Groups` `Prereq::Orgo::Hybridization` `MCAT::Chem_Phys` `Difficulty::2`

## Orgo::Isomerism

### MCAT-ORG-ISO-001

- **KC:** `Orgo::Isomerism`
- **Prereqs:** `Prereq::Orgo::Functional_Groups` `Prereq::Orgo::Nomenclature`
- **Difficulty:** 3
- **Question:** Two compounds share the molecular formula C4H10, but one is a straight chain and the other is branched. What is their relationship?
- **A:** Conformational isomers
- **B:** Enantiomers
- **C:** Diastereomers
- **D:** Constitutional (structural) isomers
- **Correct:** D
- **Explanation:** Molecules with the same molecular formula but different atomic connectivity, such as branched versus straight chains, are constitutional isomers.
- **Tags:** `KC::Orgo::Isomerism` `Prereq::Orgo::Functional_Groups` `Prereq::Orgo::Nomenclature` `MCAT::Chem_Phys` `Difficulty::3`

## Orgo::Acid_Base_Reactions

### MCAT-ORG-ACIBR-001

- **KC:** `Orgo::Acid_Base_Reactions`
- **Prereqs:** `Prereq::GenChem::Acid_Base_Equilibria` `Prereq::Orgo::Functional_Groups`
- **Difficulty:** 3
- **Question:** Why is a carboxylic acid substantially more acidic than a comparable alcohol?
- **A:** The O-H bond in the acid is essentially nonpolar
- **B:** Alcohols cannot lose a proton under any conditions
- **C:** The acidic proton in the acid is bonded directly to carbon
- **D:** The carboxylate conjugate base is resonance-stabilized over two oxygens
- **Correct:** D
- **Explanation:** Deprotonating a carboxylic acid gives a carboxylate whose negative charge is delocalized over two oxygens, stabilizing the conjugate base and strengthening the acid.
- **Tags:** `KC::Orgo::Acid_Base_Reactions` `Prereq::GenChem::Acid_Base_Equilibria` `Prereq::Orgo::Functional_Groups` `MCAT::Chem_Phys` `Difficulty::3`

## Orgo::Stereochemistry

### MCAT-ORG-STE-001

- **KC:** `Orgo::Stereochemistry`
- **Prereqs:** `Prereq::Orgo::Hybridization` `Prereq::Orgo::Isomerism`
- **Difficulty:** 3
- **Question:** A solution contains equal amounts of both enantiomers of a chiral compound. What optical activity is observed?
- **A:** Rotation equal to twice that of a single enantiomer
- **B:** No net rotation of plane-polarized light
- **C:** Rotation observed only at low temperature
- **D:** Rotation whose sign depends on the achiral solvent
- **Correct:** B
- **Explanation:** In a racemic mixture the equal and opposite rotations of the two enantiomers cancel, so no net optical rotation is measured.
- **Tags:** `KC::Orgo::Stereochemistry` `Prereq::Orgo::Hybridization` `Prereq::Orgo::Isomerism` `MCAT::Chem_Phys` `Difficulty::3`

## Orgo::Reaction_Mechanisms_Overview

### MCAT-ORG-REAMO-001

- **KC:** `Orgo::Reaction_Mechanisms_Overview`
- **Prereqs:** `Prereq::GenChem::Kinetics` `Prereq::GenChem::Thermochemistry` `Prereq::Orgo::Acid_Base_Reactions` `Prereq::Orgo::Functional_Groups`
- **Difficulty:** 3
- **Question:** In curved-arrow notation for an organic mechanism, what does a curved arrow represent?
- **A:** The movement of a proton only
- **B:** The movement of a whole atom
- **C:** The movement of an electron pair from an electron-rich site to an electron-poor site
- **D:** The transfer of heat between reactant molecules
- **Correct:** C
- **Explanation:** Curved arrows depict the flow of an electron pair from a nucleophilic, electron-rich source toward an electrophilic, electron-poor sink.
- **Tags:** `KC::Orgo::Reaction_Mechanisms_Overview` `Prereq::GenChem::Kinetics` `Prereq::GenChem::Thermochemistry` `Prereq::Orgo::Acid_Base_Reactions` `Prereq::Orgo::Functional_Groups` `MCAT::Chem_Phys` `Difficulty::3`

## Orgo::Polycyclic_and_Heterocyclic_Aromatic_Compounds

### MCAT-ORG-POLHAC-001

- **KC:** `Orgo::Polycyclic_and_Heterocyclic_Aromatic_Compounds`
- **Prereqs:** `Prereq::Orgo::Functional_Groups` `Prereq::Orgo::Hybridization` `Prereq::Orgo::Isomerism`
- **Difficulty:** 3
- **Question:** Which set of conditions must be satisfied for a molecule to be aromatic?
- **A:** Cyclic, planar, fully conjugated, and containing 4n+2 pi electrons
- **B:** Containing at least one double bond somewhere in a ring
- **C:** Cyclic and containing exactly 4n pi electrons
- **D:** Planar and completely saturated
- **Correct:** A
- **Explanation:** Aromaticity requires a cyclic, planar, continuously conjugated system with a Huckel count of 4n+2 pi electrons.
- **Tags:** `KC::Orgo::Polycyclic_and_Heterocyclic_Aromatic_Compounds` `Prereq::Orgo::Functional_Groups` `Prereq::Orgo::Hybridization` `Prereq::Orgo::Isomerism` `MCAT::Chem_Phys` `Difficulty::3`

## Orgo::Nucleophilic_Substitution

### MCAT-ORG-NUCS-001

- **KC:** `Orgo::Nucleophilic_Substitution`
- **Prereqs:** `Prereq::GenChem::Kinetics` `Prereq::Orgo::Reaction_Mechanisms_Overview` `Prereq::Orgo::Stereochemistry`
- **Difficulty:** 3
- **Question:** A primary alkyl halide reacts with a strong, unhindered nucleophile in a polar aprotic solvent. Which result is most consistent with an SN2 mechanism?
- **A:** A reaction rate independent of nucleophile concentration
- **B:** A second-order rate law with inversion of configuration at the reacting carbon
- **C:** Racemization through a planar carbocation intermediate
- **D:** A rate that increases as the substrate becomes more branched
- **Correct:** B
- **Explanation:** SN2 is a concerted bimolecular reaction whose rate depends on both substrate and nucleophile, and backside attack inverts configuration at the carbon.
- **Tags:** `KC::Orgo::Nucleophilic_Substitution` `Prereq::GenChem::Kinetics` `Prereq::Orgo::Reaction_Mechanisms_Overview` `Prereq::Orgo::Stereochemistry` `MCAT::Chem_Phys` `Difficulty::3`

## Orgo::Nucleophilic_Addition

### MCAT-ORG-NUCA-001

- **KC:** `Orgo::Nucleophilic_Addition`
- **Prereqs:** `Prereq::Orgo::Acid_Base_Reactions` `Prereq::Orgo::Reaction_Mechanisms_Overview`
- **Difficulty:** 3
- **Question:** An aldehyde reacts with one equivalent of an alcohol under acid catalysis. What is the initial nucleophilic-addition product?
- **A:** A hemiacetal
- **B:** A carboxylic acid
- **C:** An amide
- **D:** A terminal alkene
- **Correct:** A
- **Explanation:** Addition of one alcohol across the aldehyde carbonyl gives a hemiacetal, which can react with a second alcohol to form an acetal.
- **Tags:** `KC::Orgo::Nucleophilic_Addition` `Prereq::Orgo::Acid_Base_Reactions` `Prereq::Orgo::Reaction_Mechanisms_Overview` `MCAT::Chem_Phys` `Difficulty::3`

## Orgo::Oxidation_Reduction_Reactions

### MCAT-ORG-OXIRR-001

- **KC:** `Orgo::Oxidation_Reduction_Reactions`
- **Prereqs:** `Prereq::GenChem::Electrochemistry` `Prereq::Orgo::Functional_Groups` `Prereq::Orgo::Reaction_Mechanisms_Overview`
- **Difficulty:** 3
- **Question:** In organic chemistry, oxidation of a carbon atom is best recognized by which change?
- **A:** An increase in the number of C-H bonds at that carbon
- **B:** An increase in the number of bonds to more electronegative atoms such as oxygen
- **C:** The carbon gaining a full formal negative charge
- **D:** Converting one sigma bond into a pi bond with no other change
- **Correct:** B
- **Explanation:** Organic oxidation is judged by bonding changes: gaining bonds to electronegative atoms like oxygen, or losing C-H bonds, corresponds to oxidation of carbon.
- **Tags:** `KC::Orgo::Oxidation_Reduction_Reactions` `Prereq::GenChem::Electrochemistry` `Prereq::Orgo::Functional_Groups` `Prereq::Orgo::Reaction_Mechanisms_Overview` `MCAT::Chem_Phys` `Difficulty::3`

## Orgo::Elimination_Reactions

### MCAT-ORG-ELIR-001

- **KC:** `Orgo::Elimination_Reactions`
- **Prereqs:** `Prereq::Orgo::Nucleophilic_Substitution` `Prereq::Orgo::Reaction_Mechanisms_Overview`
- **Difficulty:** 4
- **Question:** A secondary alkyl halide is treated with a strong, sterically bulky base. Which outcome is most favored?
- **A:** The more substituted Zaitsev alkene, because sterics never matter
- **B:** Exclusive SN2 substitution with no elimination
- **C:** The less substituted Hofmann alkene, because the bulky base abstracts a more accessible proton
- **D:** A carbocation-derived alcohol
- **Correct:** C
- **Explanation:** A bulky strong base is hindered from reaching protons at crowded positions, so it removes a less hindered proton and favors the less substituted Hofmann alkene.
- **Tags:** `KC::Orgo::Elimination_Reactions` `Prereq::Orgo::Nucleophilic_Substitution` `Prereq::Orgo::Reaction_Mechanisms_Overview` `MCAT::Chem_Phys` `Difficulty::4`

## Orgo::Nucleophilic_Acyl_Substitution

### MCAT-ORG-NUCAS-001

- **KC:** `Orgo::Nucleophilic_Acyl_Substitution`
- **Prereqs:** `Prereq::Orgo::Acid_Base_Reactions` `Prereq::Orgo::Nucleophilic_Addition`
- **Difficulty:** 4
- **Question:** Which ranking of carboxylic acid derivatives from most to least reactive toward nucleophilic acyl substitution is correct?
- **A:** Amide > ester > anhydride > acyl chloride
- **B:** Ester > amide > acyl chloride > anhydride
- **C:** All four react at essentially equal rates
- **D:** Acyl chloride > anhydride > ester > amide
- **Correct:** D
- **Explanation:** Reactivity tracks leaving-group ability and how strongly the substituent donates electron density into the carbonyl, giving acyl chloride > anhydride > ester > amide.
- **Tags:** `KC::Orgo::Nucleophilic_Acyl_Substitution` `Prereq::Orgo::Acid_Base_Reactions` `Prereq::Orgo::Nucleophilic_Addition` `MCAT::Chem_Phys` `Difficulty::4`

## Orgo::Alcohols

### MCAT-ORG-ALC-001

- **KC:** `Orgo::Alcohols`
- **Prereqs:** `Prereq::Orgo::Acid_Base_Reactions` `Prereq::Orgo::Functional_Groups` `Prereq::Orgo::Nucleophilic_Substitution` `Prereq::Orgo::Oxidation_Reduction_Reactions`
- **Difficulty:** 3
- **Question:** A primary alcohol is treated with the mild oxidant PCC. What is the expected product?
- **A:** An aldehyde
- **B:** A carboxylic acid
- **C:** A ketone
- **D:** An alkane
- **Correct:** A
- **Explanation:** PCC is a mild oxidant that stops at the aldehyde for a primary alcohol, whereas stronger oxidants would carry it on to a carboxylic acid.
- **Tags:** `KC::Orgo::Alcohols` `Prereq::Orgo::Acid_Base_Reactions` `Prereq::Orgo::Functional_Groups` `Prereq::Orgo::Nucleophilic_Substitution` `Prereq::Orgo::Oxidation_Reduction_Reactions` `MCAT::Chem_Phys` `Difficulty::3`

## Orgo::Aldehydes_and_Ketones

### MCAT-ORG-ALDK-001

- **KC:** `Orgo::Aldehydes_and_Ketones`
- **Prereqs:** `Prereq::Orgo::Functional_Groups` `Prereq::Orgo::Nucleophilic_Addition` `Prereq::Orgo::Oxidation_Reduction_Reactions`
- **Difficulty:** 4
- **Question:** Why are aldehydes oxidized to carboxylic acids far more readily than ketones undergo further oxidation?
- **A:** Ketones contain no carbonyl group
- **B:** Aldehydes are aromatic while ketones are not
- **C:** Ketones have much more acidic alpha hydrogens
- **D:** The aldehyde carbonyl carbon bears a hydrogen that is lost during oxidation
- **Correct:** D
- **Explanation:** The aldehyde carbonyl carbon carries an H that is removed on oxidation to a carboxylic acid, whereas a ketone carbonyl carbon has no such hydrogen and resists further oxidation.
- **Tags:** `KC::Orgo::Aldehydes_and_Ketones` `Prereq::Orgo::Functional_Groups` `Prereq::Orgo::Nucleophilic_Addition` `Prereq::Orgo::Oxidation_Reduction_Reactions` `MCAT::Chem_Phys` `Difficulty::4`

## Orgo::Carboxylic_Acids

### MCAT-ORG-CARA-001

- **KC:** `Orgo::Carboxylic_Acids`
- **Prereqs:** `Prereq::Orgo::Acid_Base_Reactions` `Prereq::Orgo::Functional_Groups` `Prereq::Orgo::Oxidation_Reduction_Reactions`
- **Difficulty:** 3
- **Question:** Introducing electron-withdrawing chlorine atoms near the carboxyl group of a carboxylic acid has what effect on its acidity?
- **A:** It decreases acidity by destabilizing the carboxylate
- **B:** It has no measurable effect on acidity
- **C:** It increases acidity by inductively stabilizing the carboxylate
- **D:** It converts the compound into a base
- **Correct:** C
- **Explanation:** Electron-withdrawing groups stabilize the negative charge of the carboxylate through induction, lowering the pKa and increasing acidity.
- **Tags:** `KC::Orgo::Carboxylic_Acids` `Prereq::Orgo::Acid_Base_Reactions` `Prereq::Orgo::Functional_Groups` `Prereq::Orgo::Oxidation_Reduction_Reactions` `MCAT::Chem_Phys` `Difficulty::3`

## Orgo::Amines

### MCAT-ORG-AMI-001

- **KC:** `Orgo::Amines`
- **Prereqs:** `Prereq::Orgo::Acid_Base_Reactions` `Prereq::Orgo::Functional_Groups` `Prereq::Orgo::Nucleophilic_Substitution`
- **Difficulty:** 3
- **Question:** Why is methylamine, an aliphatic amine, generally more basic than aniline, an aromatic amine?
- **A:** Aniline has no lone pair on its nitrogen
- **B:** Methylamine is itself aromatic
- **C:** In aniline the nitrogen lone pair is delocalized into the ring, making it less available to bind a proton
- **D:** Aniline is always the stronger nucleophile in every reaction
- **Correct:** C
- **Explanation:** The aniline nitrogen lone pair is delocalized into the aromatic ring by resonance, making it less available to accept a proton, so the aliphatic amine is more basic.
- **Tags:** `KC::Orgo::Amines` `Prereq::Orgo::Acid_Base_Reactions` `Prereq::Orgo::Functional_Groups` `Prereq::Orgo::Nucleophilic_Substitution` `MCAT::Chem_Phys` `Difficulty::3`

## Orgo::Phenols

### MCAT-ORG-PHE-001

- **KC:** `Orgo::Phenols`
- **Prereqs:** `Prereq::Orgo::Acid_Base_Reactions` `Prereq::Orgo::Alcohols` `Prereq::Orgo::Polycyclic_and_Heterocyclic_Aromatic_Compounds`
- **Difficulty:** 3
- **Question:** Why is phenol significantly more acidic than cyclohexanol?
- **A:** Phenol has a much stronger O-H bond
- **B:** Cyclohexanol cannot be deprotonated at all
- **C:** Phenol does not actually contain a hydroxyl group
- **D:** The phenoxide anion is stabilized by resonance delocalization into the aromatic ring
- **Correct:** D
- **Explanation:** The phenoxide negative charge is delocalized into the aromatic ring by resonance, stabilizing the conjugate base and making phenol more acidic than a comparable non-aromatic alcohol.
- **Tags:** `KC::Orgo::Phenols` `Prereq::Orgo::Acid_Base_Reactions` `Prereq::Orgo::Alcohols` `Prereq::Orgo::Polycyclic_and_Heterocyclic_Aromatic_Compounds` `MCAT::Chem_Phys` `Difficulty::3`

## Orgo::Acid_Derivatives

### MCAT-ORG-ACID-001

- **KC:** `Orgo::Acid_Derivatives`
- **Prereqs:** `Prereq::Orgo::Carboxylic_Acids` `Prereq::Orgo::Nucleophilic_Acyl_Substitution`
- **Difficulty:** 4
- **Question:** Basic hydrolysis (saponification) of an ester produces which pair of products?
- **A:** An aldehyde and an alcohol
- **B:** A carboxylate salt and an alcohol
- **C:** An amide and water
- **D:** Two molecules of alcohol
- **Correct:** B
- **Explanation:** Saponification cleaves an ester with hydroxide to give a carboxylate salt and the corresponding alcohol.
- **Tags:** `KC::Orgo::Acid_Derivatives` `Prereq::Orgo::Carboxylic_Acids` `Prereq::Orgo::Nucleophilic_Acyl_Substitution` `MCAT::Chem_Phys` `Difficulty::4`

## Orgo::Carbohydrate_Chemistry

### MCAT-ORG-CARC-001

- **KC:** `Orgo::Carbohydrate_Chemistry`
- **Prereqs:** `Prereq::Orgo::Alcohols` `Prereq::Orgo::Nucleophilic_Addition` `Prereq::Orgo::Stereochemistry`
- **Difficulty:** 3
- **Question:** The cyclic form of glucose forms through an intramolecular reaction between which groups?
- **A:** Two carboxylic acid groups
- **B:** A hydroxyl group and the aldehyde carbonyl, giving a cyclic hemiacetal
- **C:** An amine and an ester group
- **D:** Two phosphate groups
- **Correct:** B
- **Explanation:** A hydroxyl oxygen attacks the open-chain aldehyde carbonyl in an intramolecular nucleophilic addition, forming a cyclic hemiacetal.
- **Tags:** `KC::Orgo::Carbohydrate_Chemistry` `Prereq::Orgo::Alcohols` `Prereq::Orgo::Nucleophilic_Addition` `Prereq::Orgo::Stereochemistry` `MCAT::Chem_Phys` `Difficulty::3`

## Orgo::Amino_Acid_and_Peptide_Chemistry

### MCAT-ORG-AMIAPC-001

- **KC:** `Orgo::Amino_Acid_and_Peptide_Chemistry`
- **Prereqs:** `Prereq::Orgo::Acid_Base_Reactions` `Prereq::Orgo::Amines` `Prereq::Orgo::Nucleophilic_Acyl_Substitution` `Prereq::Orgo::Stereochemistry`
- **Difficulty:** 3
- **Question:** The peptide bond linking two amino acids is formed by which type of reaction?
- **A:** Nucleophilic acyl substitution between an amino group and an activated carboxyl carbon
- **B:** Electrophilic aromatic substitution on a side chain
- **C:** Free-radical chain addition
- **D:** Oxidative coupling of two thiol groups
- **Correct:** A
- **Explanation:** A peptide (amide) bond forms when an amino group attacks an activated carboxyl carbon in a nucleophilic acyl substitution, releasing water.
- **Tags:** `KC::Orgo::Amino_Acid_and_Peptide_Chemistry` `Prereq::Orgo::Acid_Base_Reactions` `Prereq::Orgo::Amines` `Prereq::Orgo::Nucleophilic_Acyl_Substitution` `Prereq::Orgo::Stereochemistry` `MCAT::Chem_Phys` `Difficulty::3`

## Orgo::IR_Spectroscopy

### MCAT-ORG-IRS-001

- **KC:** `Orgo::IR_Spectroscopy`
- **Prereqs:** `Prereq::Orgo::Functional_Groups` `Prereq::Physics::Electromagnetic_Radiation`
- **Difficulty:** 2
- **Question:** A strong infrared absorption near 1700 reciprocal centimeters most strongly indicates the presence of which group?
- **A:** An isolated carbon-carbon single bond
- **B:** Only saturated alkane C-H bonds
- **C:** An ionic metal-oxygen bond
- **D:** A carbonyl (C=O) group
- **Correct:** D
- **Explanation:** The carbonyl C=O stretch produces a characteristic strong absorption near 1700 reciprocal centimeters, a key diagnostic band in IR spectroscopy.
- **Tags:** `KC::Orgo::IR_Spectroscopy` `Prereq::Orgo::Functional_Groups` `Prereq::Physics::Electromagnetic_Radiation` `MCAT::Chem_Phys` `Difficulty::2`

## Orgo::NMR_Spectroscopy

### MCAT-ORG-NMRS-001

- **KC:** `Orgo::NMR_Spectroscopy`
- **Prereqs:** `Prereq::Orgo::Functional_Groups` `Prereq::Orgo::Hybridization` `Prereq::Physics::Electromagnetic_Radiation`
- **Difficulty:** 4
- **Question:** In a 1H NMR spectrum, a signal appearing as a triplet is most consistent with coupling to how many equivalent protons on the adjacent carbon?
- **A:** One
- **B:** Two
- **C:** Three
- **D:** Zero
- **Correct:** B
- **Explanation:** By the n+1 rule, a triplet arises from coupling to two equivalent neighboring protons, since two plus one gives three peaks.
- **Tags:** `KC::Orgo::NMR_Spectroscopy` `Prereq::Orgo::Functional_Groups` `Prereq::Orgo::Hybridization` `Prereq::Physics::Electromagnetic_Radiation` `MCAT::Chem_Phys` `Difficulty::4`

## Orgo::Mass_Spectrometry

### MCAT-ORG-MASS-001

- **KC:** `Orgo::Mass_Spectrometry`
- **Prereqs:** `Prereq::Orgo::Functional_Groups` `Prereq::Orgo::Reaction_Mechanisms_Overview`
- **Difficulty:** 3
- **Question:** Which statement about the base peak in a mass spectrum is correct?
- **A:** It always corresponds to the molecular ion
- **B:** It always has the highest mass-to-charge ratio
- **C:** It is the most intense peak and need not be the molecular ion
- **D:** It represents the intact neutral molecule before ionization
- **Correct:** C
- **Explanation:** The base peak is simply the most intense peak, often a stable fragment, and is not necessarily the molecular ion.
- **Tags:** `KC::Orgo::Mass_Spectrometry` `Prereq::Orgo::Functional_Groups` `Prereq::Orgo::Reaction_Mechanisms_Overview` `MCAT::Chem_Phys` `Difficulty::3`

## Orgo::Molecular_Structure_and_Absorption_Spectra

### MCAT-ORG-MOLSAS-001

- **KC:** `Orgo::Molecular_Structure_and_Absorption_Spectra`
- **Prereqs:** `Prereq::Orgo::Hybridization` `Prereq::Orgo::Polycyclic_and_Heterocyclic_Aromatic_Compounds` `Prereq::Physics::Electromagnetic_Radiation`
- **Difficulty:** 3
- **Question:** Increasing the extent of conjugation in a molecule generally has what effect on its UV-Vis absorption?
- **A:** It shifts absorption to longer wavelengths as the HOMO-LUMO gap narrows
- **B:** It shifts absorption to shorter wavelengths as the gap widens
- **C:** It eliminates UV-Vis absorption entirely
- **D:** It has no relationship to the electronic energy gap
- **Correct:** A
- **Explanation:** Extended conjugation lowers the HOMO-LUMO energy gap, so lower-energy, longer-wavelength light is absorbed.
- **Tags:** `KC::Orgo::Molecular_Structure_and_Absorption_Spectra` `Prereq::Orgo::Hybridization` `Prereq::Orgo::Polycyclic_and_Heterocyclic_Aromatic_Compounds` `Prereq::Physics::Electromagnetic_Radiation` `MCAT::Chem_Phys` `Difficulty::3`

## Orgo::Separations_and_Purifications

### MCAT-ORG-SEPP-001

- **KC:** `Orgo::Separations_and_Purifications`
- **Prereqs:** `Prereq::GenChem::Intermolecular_Forces` `Prereq::GenChem::Solutions_and_Solubility` `Prereq::Orgo::Acid_Base_Reactions` `Prereq::Orgo::Functional_Groups`
- **Difficulty:** 2
- **Question:** Acid-base extraction separates a carboxylic acid from a neutral organic compound mainly by exploiting a difference in which property?
- **A:** Molecular size
- **B:** Color of the compounds
- **C:** Radioactive decay rate
- **D:** Solubility, since the acid becomes a water-soluble salt when deprotonated
- **Correct:** D
- **Explanation:** Aqueous base converts the carboxylic acid into a charged, water-soluble carboxylate salt, moving it into the aqueous layer and away from the neutral compound.
- **Tags:** `KC::Orgo::Separations_and_Purifications` `Prereq::GenChem::Intermolecular_Forces` `Prereq::GenChem::Solutions_and_Solubility` `Prereq::Orgo::Acid_Base_Reactions` `Prereq::Orgo::Functional_Groups` `MCAT::Chem_Phys` `Difficulty::2`

## Orgo::Laboratory_Techniques

### MCAT-ORG-LABT-001

- **KC:** `Orgo::Laboratory_Techniques`
- **Prereqs:** `Prereq::Orgo::IR_Spectroscopy` `Prereq::Orgo::Mass_Spectrometry` `Prereq::Orgo::NMR_Spectroscopy` `Prereq::Orgo::Separations_and_Purifications`
- **Difficulty:** 4
- **Question:** When identifying an unknown compound, why does a chemist combine mass spectrometry, IR, and NMR data?
- **A:** Each technique alone reveals the full three-dimensional crystal structure
- **B:** Together they give complementary constraints that converge on one consistent structure
- **C:** They are useful only for simple inorganic salts
- **D:** Each one redundantly measures the same molecular property
- **Correct:** B
- **Explanation:** Mass spectrometry gives formula and mass, IR identifies functional groups, and NMR reveals connectivity and environment, so together they constrain a single structure no method could establish alone.
- **Tags:** `KC::Orgo::Laboratory_Techniques` `Prereq::Orgo::IR_Spectroscopy` `Prereq::Orgo::Mass_Spectrometry` `Prereq::Orgo::NMR_Spectroscopy` `Prereq::Orgo::Separations_and_Purifications` `MCAT::Chem_Phys` `Difficulty::4`
