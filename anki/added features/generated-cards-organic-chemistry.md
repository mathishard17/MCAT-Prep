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

**Tags**

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

### MCAT-ORG-HYB-002

- **KC:** `Orgo::Hybridization`
- **Prereqs:** `Prereq::GenChem::Chemical_Bonding` `Prereq::GenChem::Molecular_Geometry`
- **Difficulty:** 1
- **Question:** Which hybridization is expected for each carbon in a carbon-carbon triple bond, as found in an alkyne?
- **A:** sp
- **B:** sp2
- **C:** sp3
- **D:** sp3d
- **Correct:** A
- **Explanation:** Each triple-bonded carbon forms two sigma bonds and two pi bonds, using two sp hybrid orbitals (linear, about 180 degrees) plus two unhybridized p orbitals.
- **Tags:** `KC::Orgo::Hybridization` `Prereq::GenChem::Chemical_Bonding` `Prereq::GenChem::Molecular_Geometry` `MCAT::Chem_Phys` `Difficulty::1`

### MCAT-ORG-HYB-003

- **KC:** `Orgo::Hybridization`
- **Prereqs:** `Prereq::GenChem::Chemical_Bonding` `Prereq::GenChem::Molecular_Geometry`
- **Difficulty:** 3
- **Question:** In an amide the carbon-nitrogen bond shows restricted rotation and partial double-bond character. Which description of the nitrogen best explains this?
- **A:** An sp3 nitrogen holding a localized lone pair in a tetrahedral orbital
- **B:** An sp nitrogen bearing two lone pairs
- **C:** An sp2 nitrogen whose lone pair is delocalized into the carbonyl pi system
- **D:** An sp3d nitrogen using an expanded octet
- **Correct:** C
- **Explanation:** The amide nitrogen is sp2 hybridized, and its lone pair sits in a p orbital that conjugates with the carbonyl, giving partial C-N double-bond character and hindered rotation.
- **Tags:** `KC::Orgo::Hybridization` `Prereq::GenChem::Chemical_Bonding` `Prereq::GenChem::Molecular_Geometry` `MCAT::Chem_Phys` `Difficulty::3`

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

### MCAT-ORG-FUNG-002

- **KC:** `Orgo::Functional_Groups`
- **Prereqs:** `Prereq::GenChem::Chemical_Bonding` `Prereq::Orgo::Hybridization`
- **Difficulty:** 1
- **Question:** In which functional group is an oxygen atom bonded to two carbon atoms and to no hydrogen?
- **A:** Alcohol
- **B:** Aldehyde
- **C:** Carboxylic acid
- **D:** Ether
- **Correct:** D
- **Explanation:** An ether has an oxygen bonded to two carbon groups (R-O-R) and no O-H bond, which distinguishes it from an alcohol.
- **Tags:** `KC::Orgo::Functional_Groups` `Prereq::GenChem::Chemical_Bonding` `Prereq::Orgo::Hybridization` `MCAT::Chem_Phys` `Difficulty::1`

### MCAT-ORG-FUNG-003

- **KC:** `Orgo::Functional_Groups`
- **Prereqs:** `Prereq::GenChem::Chemical_Bonding` `Prereq::Orgo::Hybridization`
- **Difficulty:** 3
- **Question:** A compound has the connectivity R-C(=O)-O-R', where R and R' are both carbon-containing groups. Which functional-group class is this?
- **A:** Ketone
- **B:** Ester
- **C:** Amide
- **D:** Carboxylic acid
- **Correct:** B
- **Explanation:** A carbonyl carbon bonded to an alkoxy (-OR') group defines an ester; a carboxylic acid would bear -OH and an amide would bear nitrogen on the carbonyl.
- **Tags:** `KC::Orgo::Functional_Groups` `Prereq::GenChem::Chemical_Bonding` `Prereq::Orgo::Hybridization` `MCAT::Chem_Phys` `Difficulty::3`

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

### MCAT-ORG-NOM-002

- **KC:** `Orgo::Nomenclature`
- **Prereqs:** `Prereq::Orgo::Functional_Groups` `Prereq::Orgo::Hybridization`
- **Difficulty:** 1
- **Question:** In IUPAC substitutive nomenclature, the suffix -ol at the end of a name indicates which principal characteristic group?
- **A:** A hydroxyl (alcohol) group
- **B:** An aldehyde group
- **C:** A carboxylic acid group
- **D:** An amine group
- **Correct:** A
- **Explanation:** The suffix -ol denotes a hydroxyl group, so a compound named as an -ol is an alcohol.
- **Tags:** `KC::Orgo::Nomenclature` `Prereq::Orgo::Functional_Groups` `Prereq::Orgo::Hybridization` `MCAT::Chem_Phys` `Difficulty::1`

### MCAT-ORG-NOM-003

- **KC:** `Orgo::Nomenclature`
- **Prereqs:** `Prereq::Orgo::Functional_Groups` `Prereq::Orgo::Hybridization`
- **Difficulty:** 3
- **Question:** What is the correct IUPAC name of the alcohol CH3-CH(CH3)-CH2-CH2-OH?
- **A:** 2-methylbutan-1-ol
- **B:** 3-methylbutanal
- **C:** 3-methylbutan-1-ol
- **D:** 2-methylbutan-4-ol
- **Correct:** C
- **Explanation:** Numbering from the hydroxyl end gives the -OH the lowest locant (C1); the methyl branch then falls on C3, giving 3-methylbutan-1-ol.
- **Tags:** `KC::Orgo::Nomenclature` `Prereq::Orgo::Functional_Groups` `Prereq::Orgo::Hybridization` `MCAT::Chem_Phys` `Difficulty::3`

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

### MCAT-ORG-ISO-002

- **KC:** `Orgo::Isomerism`
- **Prereqs:** `Prereq::Orgo::Functional_Groups` `Prereq::Orgo::Nomenclature`
- **Difficulty:** 2
- **Question:** 1,2-dichloroethene has restricted rotation about its carbon-carbon double bond. The form with both chlorine atoms on the same side of that double bond is best described as which isomer?
- **A:** The trans isomer
- **B:** An enantiomer of the trans isomer
- **C:** A constitutional isomer of the trans isomer
- **D:** The cis isomer
- **Correct:** D
- **Explanation:** With both chlorines on the same side of the non-rotating C=C double bond, the molecule is the cis geometric isomer; cis and trans share connectivity, so they are stereoisomers rather than constitutional isomers.
- **Tags:** `KC::Orgo::Isomerism` `Prereq::Orgo::Functional_Groups` `Prereq::Orgo::Nomenclature` `MCAT::Chem_Phys` `Difficulty::2`

### MCAT-ORG-ISO-003

- **KC:** `Orgo::Isomerism`
- **Prereqs:** `Prereq::Orgo::Functional_Groups` `Prereq::Orgo::Nomenclature`
- **Difficulty:** 4
- **Question:** Glucose and galactose share the same molecular formula and connectivity but differ in configuration at a single stereocenter. This specific kind of diastereomer is called what?
- **A:** Epimers
- **B:** Enantiomers
- **C:** Anomers
- **D:** Constitutional isomers
- **Correct:** A
- **Explanation:** Diastereomers that differ in configuration at exactly one of several stereocenters are epimers; glucose and galactose differ only at carbon 4.
- **Tags:** `KC::Orgo::Isomerism` `Prereq::Orgo::Functional_Groups` `Prereq::Orgo::Nomenclature` `MCAT::Chem_Phys` `Difficulty::4`

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

### MCAT-ORG-ACIBR-002

- **KC:** `Orgo::Acid_Base_Reactions`
- **Prereqs:** `Prereq::GenChem::Acid_Base_Equilibria` `Prereq::Orgo::Functional_Groups`
- **Difficulty:** 2
- **Question:** Which of the following compounds is the strongest Bronsted-Lowry acid?
- **A:** Ethane
- **B:** Acetic acid
- **C:** Ethanol
- **D:** Ethylamine
- **Correct:** B
- **Explanation:** Acetic acid is far more acidic than an alcohol, amine, or alkane because deprotonation gives a resonance-stabilized carboxylate.
- **Tags:** `KC::Orgo::Acid_Base_Reactions` `Prereq::GenChem::Acid_Base_Equilibria` `Prereq::Orgo::Functional_Groups` `MCAT::Chem_Phys` `Difficulty::2`

### MCAT-ORG-ACIBR-003

- **KC:** `Orgo::Acid_Base_Reactions`
- **Prereqs:** `Prereq::GenChem::Acid_Base_Equilibria` `Prereq::Orgo::Functional_Groups`
- **Difficulty:** 4
- **Question:** Sodium ethoxide (whose conjugate acid is ethanol, pKa about 16) is added to acetic acid (pKa about 4.8). Which way does the acid-base equilibrium lie?
- **A:** No reaction occurs because both species are neutral
- **B:** Toward reactants, because acetic acid is the weaker acid
- **C:** Toward products: ethoxide deprotonates acetic acid to give acetate and ethanol
- **D:** Exactly balanced, with equal amounts of all species
- **Correct:** C
- **Explanation:** The stronger acid (acetic acid, lower pKa) protonates the stronger base (ethoxide), forming the weaker acid (ethanol) and weaker base (acetate), so products are favored.
- **Tags:** `KC::Orgo::Acid_Base_Reactions` `Prereq::GenChem::Acid_Base_Equilibria` `Prereq::Orgo::Functional_Groups` `MCAT::Chem_Phys` `Difficulty::4`

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

### MCAT-ORG-STE-002

- **KC:** `Orgo::Stereochemistry`
- **Prereqs:** `Prereq::Orgo::Hybridization` `Prereq::Orgo::Isomerism`
- **Difficulty:** 2
- **Question:** What is the defining requirement for a tetrahedral carbon to be a stereocenter (chirality center)?
- **A:** It must be part of a carbon-carbon double bond
- **B:** It must bear at least one hydrogen atom
- **C:** It must be bonded to an oxygen atom
- **D:** It must be bonded to four different groups
- **Correct:** D
- **Explanation:** A tetrahedral carbon bonded to four different substituents makes the molecule and its mirror image non-superimposable, which defines a stereocenter.
- **Tags:** `KC::Orgo::Stereochemistry` `Prereq::Orgo::Hybridization` `Prereq::Orgo::Isomerism` `MCAT::Chem_Phys` `Difficulty::2`

### MCAT-ORG-STE-003

- **KC:** `Orgo::Stereochemistry`
- **Prereqs:** `Prereq::Orgo::Hybridization` `Prereq::Orgo::Isomerism`
- **Difficulty:** 4
- **Question:** When assigning R or S configuration, the four groups on a stereocenter are ranked by Cahn-Ingold-Prelog priority. On what basis is priority assigned?
- **A:** By decreasing atomic number at the first point of difference from the stereocenter
- **B:** By alphabetical order of the substituent names
- **C:** By increasing molecular weight of each whole substituent
- **D:** By the order in which the groups happen to be drawn
- **Correct:** A
- **Explanation:** Cahn-Ingold-Prelog rules give higher priority to the atom of higher atomic number at the first point of difference moving out from the stereocenter.
- **Tags:** `KC::Orgo::Stereochemistry` `Prereq::Orgo::Hybridization` `Prereq::Orgo::Isomerism` `MCAT::Chem_Phys` `Difficulty::4`

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

### MCAT-ORG-REAMO-002

- **KC:** `Orgo::Reaction_Mechanisms_Overview`
- **Prereqs:** `Prereq::GenChem::Kinetics` `Prereq::GenChem::Thermochemistry` `Prereq::Orgo::Acid_Base_Reactions` `Prereq::Orgo::Functional_Groups`
- **Difficulty:** 2
- **Question:** In an organic reaction, a species that donates an electron pair to form a new covalent bond is best classified as what?
- **A:** An electrophile
- **B:** A nucleophile
- **C:** A leaving group
- **D:** A free radical
- **Correct:** B
- **Explanation:** A nucleophile is electron-rich and donates an electron pair to an electron-poor electrophile to form a new bond.
- **Tags:** `KC::Orgo::Reaction_Mechanisms_Overview` `Prereq::GenChem::Kinetics` `Prereq::GenChem::Thermochemistry` `Prereq::Orgo::Acid_Base_Reactions` `Prereq::Orgo::Functional_Groups` `MCAT::Chem_Phys` `Difficulty::2`

### MCAT-ORG-REAMO-003

- **KC:** `Orgo::Reaction_Mechanisms_Overview`
- **Prereqs:** `Prereq::GenChem::Kinetics` `Prereq::GenChem::Thermochemistry` `Prereq::Orgo::Acid_Base_Reactions` `Prereq::Orgo::Functional_Groups`
- **Difficulty:** 4
- **Question:** A reaction's experimentally measured rate law is first order in substrate and zero order in the nucleophile. What does this most directly reveal about the rate-determining step?
- **A:** The nucleophile is consumed during the rate-determining step
- **B:** The reaction must be a single concerted step involving both species
- **C:** The rate-determining step involves only the substrate, before the nucleophile participates
- **D:** The reaction proceeds without any rate-determining step
- **Correct:** C
- **Explanation:** Because the rate is independent of nucleophile concentration, the nucleophile enters after the slow step, which involves only the substrate (as in a unimolecular ionization).
- **Tags:** `KC::Orgo::Reaction_Mechanisms_Overview` `Prereq::GenChem::Kinetics` `Prereq::GenChem::Thermochemistry` `Prereq::Orgo::Acid_Base_Reactions` `Prereq::Orgo::Functional_Groups` `MCAT::Chem_Phys` `Difficulty::4`

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

### MCAT-ORG-POLHAC-002

- **KC:** `Orgo::Polycyclic_and_Heterocyclic_Aromatic_Compounds`
- **Prereqs:** `Prereq::Orgo::Functional_Groups` `Prereq::Orgo::Hybridization` `Prereq::Orgo::Isomerism`
- **Difficulty:** 2
- **Question:** Naphthalene is made of two benzene rings fused along a shared edge and contains only carbon and hydrogen. It is best classified as which of the following?
- **A:** A heterocyclic aromatic compound
- **B:** A nonaromatic cycloalkane
- **C:** An aliphatic amine
- **D:** A polycyclic aromatic hydrocarbon
- **Correct:** D
- **Explanation:** Two fused all-carbon aromatic rings make naphthalene a polycyclic aromatic hydrocarbon (PAH); it contains no ring heteroatom, so it is not heterocyclic.
- **Tags:** `KC::Orgo::Polycyclic_and_Heterocyclic_Aromatic_Compounds` `Prereq::Orgo::Functional_Groups` `Prereq::Orgo::Hybridization` `Prereq::Orgo::Isomerism` `MCAT::Chem_Phys` `Difficulty::2`

### MCAT-ORG-POLHAC-003

- **KC:** `Orgo::Polycyclic_and_Heterocyclic_Aromatic_Compounds`
- **Prereqs:** `Prereq::Orgo::Functional_Groups` `Prereq::Orgo::Hybridization` `Prereq::Orgo::Isomerism`
- **Difficulty:** 4
- **Question:** Pyridine and pyrrole are both nitrogen-containing aromatic heterocycles, yet pyridine is a far stronger base. Which explanation accounts for this difference?
- **A:** In pyridine the nitrogen lone pair sits in an in-plane sp2 orbital and is not part of the aromatic sextet, so it is free to accept a proton
- **B:** Pyridine is not aromatic, while pyrrole is
- **C:** Pyrrole contains no nitrogen atom
- **D:** Pyridine's lone pair is delocalized into the ring, unlike pyrrole's
- **Correct:** A
- **Explanation:** Pyridine's basic lone pair occupies an sp2 orbital in the ring plane and is not needed for aromaticity, whereas pyrrole's nitrogen lone pair is part of the aromatic sextet and is unavailable to bind a proton.
- **Tags:** `KC::Orgo::Polycyclic_and_Heterocyclic_Aromatic_Compounds` `Prereq::Orgo::Functional_Groups` `Prereq::Orgo::Hybridization` `Prereq::Orgo::Isomerism` `MCAT::Chem_Phys` `Difficulty::4`

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

### MCAT-ORG-NUCS-002

- **KC:** `Orgo::Nucleophilic_Substitution`
- **Prereqs:** `Prereq::GenChem::Kinetics` `Prereq::Orgo::Reaction_Mechanisms_Overview` `Prereq::Orgo::Stereochemistry`
- **Difficulty:** 2
- **Question:** The rate of a typical SN1 reaction depends on the concentration of which species?
- **A:** Both the substrate and the nucleophile
- **B:** The substrate only
- **C:** The nucleophile only
- **D:** The solvent only
- **Correct:** B
- **Explanation:** SN1 is unimolecular; its rate-determining step is ionization of the substrate to a carbocation, so the rate depends only on substrate concentration and not on the nucleophile.
- **Tags:** `KC::Orgo::Nucleophilic_Substitution` `Prereq::GenChem::Kinetics` `Prereq::Orgo::Reaction_Mechanisms_Overview` `Prereq::Orgo::Stereochemistry` `MCAT::Chem_Phys` `Difficulty::2`

### MCAT-ORG-NUCS-003

- **KC:** `Orgo::Nucleophilic_Substitution`
- **Prereqs:** `Prereq::GenChem::Kinetics` `Prereq::Orgo::Reaction_Mechanisms_Overview` `Prereq::Orgo::Stereochemistry`
- **Difficulty:** 4
- **Question:** A tertiary alkyl halide is solvolyzed in warm aqueous ethanol with no strong nucleophile added. Which observation is most consistent with the expected SN1 pathway?
- **A:** A second-order rate law with clean inversion of configuration
- **B:** A faster reaction for the analogous primary halide
- **C:** Racemization at the reacting carbon via a planar carbocation intermediate
- **D:** A rate that is independent of carbocation stability
- **Correct:** C
- **Explanation:** SN1 forms a planar carbocation that can be attacked from either face, giving racemization; tertiary substrates react fastest because they form the most stable carbocation.
- **Tags:** `KC::Orgo::Nucleophilic_Substitution` `Prereq::GenChem::Kinetics` `Prereq::Orgo::Reaction_Mechanisms_Overview` `Prereq::Orgo::Stereochemistry` `MCAT::Chem_Phys` `Difficulty::4`

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

### MCAT-ORG-NUCA-002

- **KC:** `Orgo::Nucleophilic_Addition`
- **Prereqs:** `Prereq::Orgo::Acid_Base_Reactions` `Prereq::Orgo::Reaction_Mechanisms_Overview`
- **Difficulty:** 2
- **Question:** For a typical aldehyde and a typical ketone of comparable size, which is generally more reactive toward nucleophilic addition at the carbonyl carbon?
- **A:** The ketone, because its extra alkyl group withdraws electron density
- **B:** They react at exactly the same rate
- **C:** The ketone, because it is less sterically hindered
- **D:** The aldehyde, because it is less hindered and less electronically stabilized
- **Correct:** D
- **Explanation:** An aldehyde carbonyl carries only one alkyl group, so it is less sterically hindered and its partial positive carbon is less stabilized by electron donation, making it more reactive than a ketone.
- **Tags:** `KC::Orgo::Nucleophilic_Addition` `Prereq::Orgo::Acid_Base_Reactions` `Prereq::Orgo::Reaction_Mechanisms_Overview` `MCAT::Chem_Phys` `Difficulty::2`

### MCAT-ORG-NUCA-003

- **KC:** `Orgo::Nucleophilic_Addition`
- **Prereqs:** `Prereq::Orgo::Acid_Base_Reactions` `Prereq::Orgo::Reaction_Mechanisms_Overview`
- **Difficulty:** 4
- **Question:** A ketone is treated with excess ethylene glycol (a diol) and an acid catalyst while water is removed. What is the product, often used to protect the carbonyl?
- **A:** A cyclic acetal (a 1,3-dioxolane)
- **B:** A carboxylic acid
- **C:** A primary amine
- **D:** A stable enol that does not revert
- **Correct:** A
- **Explanation:** A ketone plus a diol under acid with water removal forms a cyclic acetal (a 1,3-dioxolane), a protecting group that can later be hydrolyzed back to the ketone with aqueous acid.
- **Tags:** `KC::Orgo::Nucleophilic_Addition` `Prereq::Orgo::Acid_Base_Reactions` `Prereq::Orgo::Reaction_Mechanisms_Overview` `MCAT::Chem_Phys` `Difficulty::4`

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

### MCAT-ORG-OXIRR-002

- **KC:** `Orgo::Oxidation_Reduction_Reactions`
- **Prereqs:** `Prereq::GenChem::Electrochemistry` `Prereq::Orgo::Functional_Groups` `Prereq::Orgo::Reaction_Mechanisms_Overview`
- **Difficulty:** 2
- **Question:** Treatment of a ketone with sodium borohydride (NaBH4) followed by aqueous workup gives which product?
- **A:** A primary alcohol
- **B:** A secondary alcohol
- **C:** A carboxylic acid
- **D:** An alkane
- **Correct:** B
- **Explanation:** NaBH4 delivers hydride to the ketone carbonyl, reducing it to a secondary alcohol.
- **Tags:** `KC::Orgo::Oxidation_Reduction_Reactions` `Prereq::GenChem::Electrochemistry` `Prereq::Orgo::Functional_Groups` `Prereq::Orgo::Reaction_Mechanisms_Overview` `MCAT::Chem_Phys` `Difficulty::2`

### MCAT-ORG-OXIRR-003

- **KC:** `Orgo::Oxidation_Reduction_Reactions`
- **Prereqs:** `Prereq::GenChem::Electrochemistry` `Prereq::Orgo::Functional_Groups` `Prereq::Orgo::Reaction_Mechanisms_Overview`
- **Difficulty:** 4
- **Question:** A primary alcohol is treated with an excess of aqueous chromic acid, a strong oxidant. What is the expected final product?
- **A:** An aldehyde that resists any further reaction
- **B:** A ketone
- **C:** A carboxylic acid
- **D:** A tertiary alcohol
- **Correct:** C
- **Explanation:** A strong oxidant such as aqueous chromic acid oxidizes a primary alcohol past the aldehyde stage all the way to a carboxylic acid.
- **Tags:** `KC::Orgo::Oxidation_Reduction_Reactions` `Prereq::GenChem::Electrochemistry` `Prereq::Orgo::Functional_Groups` `Prereq::Orgo::Reaction_Mechanisms_Overview` `MCAT::Chem_Phys` `Difficulty::4`

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

### MCAT-ORG-ELIR-002

- **KC:** `Orgo::Elimination_Reactions`
- **Prereqs:** `Prereq::Orgo::Nucleophilic_Substitution` `Prereq::Orgo::Reaction_Mechanisms_Overview`
- **Difficulty:** 2
- **Question:** Using a small, strong base, the major product of a typical E2 elimination follows Zaitsev's rule. Which alkene is favored?
- **A:** The least substituted (Hofmann) alkene
- **B:** An alkane formed by reduction
- **C:** A carbocation rearrangement product
- **D:** The more highly substituted, more stable alkene
- **Correct:** D
- **Explanation:** With a small base, Zaitsev's rule predicts the more substituted, more thermodynamically stable alkene as the major product.
- **Tags:** `KC::Orgo::Elimination_Reactions` `Prereq::Orgo::Nucleophilic_Substitution` `Prereq::Orgo::Reaction_Mechanisms_Overview` `MCAT::Chem_Phys` `Difficulty::2`

### MCAT-ORG-ELIR-003

- **KC:** `Orgo::Elimination_Reactions`
- **Prereqs:** `Prereq::Orgo::Nucleophilic_Substitution` `Prereq::Orgo::Reaction_Mechanisms_Overview`
- **Difficulty:** 3
- **Question:** The concerted E2 mechanism has a strong geometric preference for the orientation of the C-H bond being broken relative to the leaving group. What is that preferred geometry?
- **A:** Anti-periplanar, with the two bonds roughly 180 degrees apart
- **B:** Syn-periplanar, with the two bonds eclipsed on the same side
- **C:** Exactly 90 degrees apart
- **D:** Random; geometry does not matter for E2
- **Correct:** A
- **Explanation:** The E2 transition state is lowest in energy when the C-H and C-leaving-group bonds are anti-periplanar, allowing the developing orbitals to overlap into the new pi bond.
- **Tags:** `KC::Orgo::Elimination_Reactions` `Prereq::Orgo::Nucleophilic_Substitution` `Prereq::Orgo::Reaction_Mechanisms_Overview` `MCAT::Chem_Phys` `Difficulty::3`

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

### MCAT-ORG-NUCAS-002

- **KC:** `Orgo::Nucleophilic_Acyl_Substitution`
- **Prereqs:** `Prereq::Orgo::Acid_Base_Reactions` `Prereq::Orgo::Nucleophilic_Addition`
- **Difficulty:** 2
- **Question:** A nucleophile attacks the carbonyl carbon of both a ketone and an ester, but only the ester typically undergoes net substitution. Why?
- **A:** The ester carbonyl is aromatic and cannot add a nucleophile
- **B:** The ester's tetrahedral intermediate can expel a leaving group (the alkoxy group), regenerating a carbonyl
- **C:** The ester has no carbonyl carbon to attack
- **D:** The nucleophile always adds to an ester exactly twice
- **Correct:** B
- **Explanation:** Carboxylic acid derivatives carry a leaving group on the carbonyl carbon, so their tetrahedral intermediate collapses by expelling that group, giving substitution rather than the simple addition seen with ketones.
- **Tags:** `KC::Orgo::Nucleophilic_Acyl_Substitution` `Prereq::Orgo::Acid_Base_Reactions` `Prereq::Orgo::Nucleophilic_Addition` `MCAT::Chem_Phys` `Difficulty::2`

### MCAT-ORG-NUCAS-003

- **KC:** `Orgo::Nucleophilic_Acyl_Substitution`
- **Prereqs:** `Prereq::Orgo::Acid_Base_Reactions` `Prereq::Orgo::Nucleophilic_Addition`
- **Difficulty:** 3
- **Question:** Which reagent converts a carboxylic acid into the corresponding acyl chloride, the most reactive acid derivative?
- **A:** Aqueous sodium bicarbonate
- **B:** Sodium borohydride
- **C:** Thionyl chloride (SOCl2)
- **D:** Excess water
- **Correct:** C
- **Explanation:** Thionyl chloride converts the carboxylic acid -OH into a good leaving group and installs chloride, giving the highly reactive acyl chloride.
- **Tags:** `KC::Orgo::Nucleophilic_Acyl_Substitution` `Prereq::Orgo::Acid_Base_Reactions` `Prereq::Orgo::Nucleophilic_Addition` `MCAT::Chem_Phys` `Difficulty::3`

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

### MCAT-ORG-ALC-002

- **KC:** `Orgo::Alcohols`
- **Prereqs:** `Prereq::Orgo::Acid_Base_Reactions` `Prereq::Orgo::Functional_Groups` `Prereq::Orgo::Nucleophilic_Substitution` `Prereq::Orgo::Oxidation_Reduction_Reactions`
- **Difficulty:** 2
- **Question:** Alcohols boil at notably higher temperatures than hydrocarbons of similar molecular weight. Which intermolecular interaction is chiefly responsible?
- **A:** London dispersion forces alone
- **B:** Ion-ion (ionic) bonding
- **C:** Covalent network bonding
- **D:** Hydrogen bonding through the O-H group
- **Correct:** D
- **Explanation:** The polar O-H group lets alcohol molecules hydrogen bond to one another, raising the boiling point relative to comparably sized hydrocarbons.
- **Tags:** `KC::Orgo::Alcohols` `Prereq::Orgo::Acid_Base_Reactions` `Prereq::Orgo::Functional_Groups` `Prereq::Orgo::Nucleophilic_Substitution` `Prereq::Orgo::Oxidation_Reduction_Reactions` `MCAT::Chem_Phys` `Difficulty::2`

### MCAT-ORG-ALC-003

- **KC:** `Orgo::Alcohols`
- **Prereqs:** `Prereq::Orgo::Acid_Base_Reactions` `Prereq::Orgo::Functional_Groups` `Prereq::Orgo::Nucleophilic_Substitution` `Prereq::Orgo::Oxidation_Reduction_Reactions`
- **Difficulty:** 4
- **Question:** A secondary alcohol is heated with concentrated sulfuric acid. Which reaction and major product are expected?
- **A:** Acid-catalyzed dehydration to an alkene
- **B:** Oxidation to a carboxylic acid
- **C:** Reduction to an alkane
- **D:** Substitution to give a primary amine
- **Correct:** A
- **Explanation:** Hot concentrated acid protonates the hydroxyl so water can leave, driving E1 dehydration to an alkene (favoring the more substituted alkene).
- **Tags:** `KC::Orgo::Alcohols` `Prereq::Orgo::Acid_Base_Reactions` `Prereq::Orgo::Functional_Groups` `Prereq::Orgo::Nucleophilic_Substitution` `Prereq::Orgo::Oxidation_Reduction_Reactions` `MCAT::Chem_Phys` `Difficulty::4`

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

### MCAT-ORG-ALDK-002

- **KC:** `Orgo::Aldehydes_and_Ketones`
- **Prereqs:** `Prereq::Orgo::Functional_Groups` `Prereq::Orgo::Nucleophilic_Addition` `Prereq::Orgo::Oxidation_Reduction_Reactions`
- **Difficulty:** 3
- **Question:** Hydrogens on the carbon directly adjacent to a carbonyl (the alpha carbon) are unusually acidic for C-H bonds. Why?
- **A:** Because the carbonyl oxygen is always protonated first
- **B:** Because removing an alpha proton gives an enolate stabilized by resonance onto the carbonyl oxygen
- **C:** Because the alpha carbon carries a full formal positive charge
- **D:** Because alpha C-H bonds are essentially ionic
- **Correct:** B
- **Explanation:** Deprotonating the alpha carbon yields an enolate whose negative charge is delocalized onto the electronegative carbonyl oxygen by resonance, stabilizing the conjugate base.
- **Tags:** `KC::Orgo::Aldehydes_and_Ketones` `Prereq::Orgo::Functional_Groups` `Prereq::Orgo::Nucleophilic_Addition` `Prereq::Orgo::Oxidation_Reduction_Reactions` `MCAT::Chem_Phys` `Difficulty::3`

### MCAT-ORG-ALDK-003

- **KC:** `Orgo::Aldehydes_and_Ketones`
- **Prereqs:** `Prereq::Orgo::Functional_Groups` `Prereq::Orgo::Nucleophilic_Addition` `Prereq::Orgo::Oxidation_Reduction_Reactions`
- **Difficulty:** 5
- **Question:** In aqueous base, an aldehyde bearing alpha hydrogens can react with a second molecule of itself so that an enolate adds to the other's carbonyl carbon. This carbon-carbon bond-forming reaction is called what?
- **A:** A Fischer esterification
- **B:** A saponification
- **C:** An aldol addition
- **D:** An electrophilic aromatic substitution
- **Correct:** C
- **Explanation:** In the aldol reaction an enolate nucleophile adds to the carbonyl of a second carbonyl compound, forming a beta-hydroxy carbonyl product and a new carbon-carbon bond.
- **Tags:** `KC::Orgo::Aldehydes_and_Ketones` `Prereq::Orgo::Functional_Groups` `Prereq::Orgo::Nucleophilic_Addition` `Prereq::Orgo::Oxidation_Reduction_Reactions` `MCAT::Chem_Phys` `Difficulty::5`

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

### MCAT-ORG-CARA-002

- **KC:** `Orgo::Carboxylic_Acids`
- **Prereqs:** `Prereq::Orgo::Acid_Base_Reactions` `Prereq::Orgo::Functional_Groups` `Prereq::Orgo::Oxidation_Reduction_Reactions`
- **Difficulty:** 2
- **Question:** Carboxylic acids often boil even higher than alcohols of similar mass. This is largely because two carboxylic acid molecules can do what?
- **A:** Ionize completely into free ions in the pure liquid
- **B:** Form extended covalent networks
- **C:** Hydrogen bond only to water molecules
- **D:** Form a hydrogen-bonded dimer joined by two O-H to O interactions
- **Correct:** D
- **Explanation:** Two carboxylic acid molecules pair into a cyclic dimer held by two hydrogen bonds, effectively doubling the mass that must vaporize and raising the boiling point.
- **Tags:** `KC::Orgo::Carboxylic_Acids` `Prereq::Orgo::Acid_Base_Reactions` `Prereq::Orgo::Functional_Groups` `Prereq::Orgo::Oxidation_Reduction_Reactions` `MCAT::Chem_Phys` `Difficulty::2`

### MCAT-ORG-CARA-003

- **KC:** `Orgo::Carboxylic_Acids`
- **Prereqs:** `Prereq::Orgo::Acid_Base_Reactions` `Prereq::Orgo::Functional_Groups` `Prereq::Orgo::Oxidation_Reduction_Reactions`
- **Difficulty:** 4
- **Question:** Rank acetic acid, chloroacetic acid, and trichloroacetic acid from MOST to LEAST acidic.
- **A:** Trichloroacetic acid > chloroacetic acid > acetic acid
- **B:** Acetic acid > chloroacetic acid > trichloroacetic acid
- **C:** Chloroacetic acid > acetic acid > trichloroacetic acid
- **D:** All three are equally acidic
- **Correct:** A
- **Explanation:** Each added electron-withdrawing chlorine further stabilizes the carboxylate by induction, so trichloroacetic acid is the most acidic and unsubstituted acetic acid the least.
- **Tags:** `KC::Orgo::Carboxylic_Acids` `Prereq::Orgo::Acid_Base_Reactions` `Prereq::Orgo::Functional_Groups` `Prereq::Orgo::Oxidation_Reduction_Reactions` `MCAT::Chem_Phys` `Difficulty::4`

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

### MCAT-ORG-AMI-002

- **KC:** `Orgo::Amines`
- **Prereqs:** `Prereq::Orgo::Acid_Base_Reactions` `Prereq::Orgo::Functional_Groups` `Prereq::Orgo::Nucleophilic_Substitution`
- **Difficulty:** 2
- **Question:** An amine acts as a base primarily because of which feature of its nitrogen atom?
- **A:** An empty p orbital that accepts an electron pair
- **B:** A nonbonding lone pair that can accept a proton
- **C:** A permanent full positive charge on nitrogen
- **D:** A carbon-nitrogen triple bond
- **Correct:** B
- **Explanation:** The nitrogen lone pair can accept a proton, so amines behave as Bronsted-Lowry bases (and as nucleophiles).
- **Tags:** `KC::Orgo::Amines` `Prereq::Orgo::Acid_Base_Reactions` `Prereq::Orgo::Functional_Groups` `Prereq::Orgo::Nucleophilic_Substitution` `MCAT::Chem_Phys` `Difficulty::2`

### MCAT-ORG-AMI-003

- **KC:** `Orgo::Amines`
- **Prereqs:** `Prereq::Orgo::Acid_Base_Reactions` `Prereq::Orgo::Functional_Groups` `Prereq::Orgo::Nucleophilic_Substitution`
- **Difficulty:** 4
- **Question:** Rank ammonia, methylamine, and acetamide (an amide) in order of DECREASING basicity at nitrogen.
- **A:** Acetamide > methylamine > ammonia
- **B:** Ammonia > methylamine > acetamide
- **C:** Methylamine > ammonia > acetamide
- **D:** All three are equally basic
- **Correct:** C
- **Explanation:** The electron-donating alkyl group makes methylamine more basic than ammonia, while an amide is barely basic because its nitrogen lone pair is delocalized into the carbonyl.
- **Tags:** `KC::Orgo::Amines` `Prereq::Orgo::Acid_Base_Reactions` `Prereq::Orgo::Functional_Groups` `Prereq::Orgo::Nucleophilic_Substitution` `MCAT::Chem_Phys` `Difficulty::4`

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

### MCAT-ORG-PHE-002

- **KC:** `Orgo::Phenols`
- **Prereqs:** `Prereq::Orgo::Acid_Base_Reactions` `Prereq::Orgo::Alcohols` `Prereq::Orgo::Polycyclic_and_Heterocyclic_Aromatic_Compounds`
- **Difficulty:** 2
- **Question:** Which description best matches the structure of phenol?
- **A:** A hydroxyl group on a saturated cyclohexane ring
- **B:** A carboxylic acid group attached to a benzene ring
- **C:** An amino group attached to a benzene ring
- **D:** A hydroxyl group attached directly to a benzene ring
- **Correct:** D
- **Explanation:** Phenol is a hydroxyl group bonded directly to an aromatic benzene ring carbon, the structural feature behind its enhanced acidity.
- **Tags:** `KC::Orgo::Phenols` `Prereq::Orgo::Acid_Base_Reactions` `Prereq::Orgo::Alcohols` `Prereq::Orgo::Polycyclic_and_Heterocyclic_Aromatic_Compounds` `MCAT::Chem_Phys` `Difficulty::2`

### MCAT-ORG-PHE-003

- **KC:** `Orgo::Phenols`
- **Prereqs:** `Prereq::Orgo::Acid_Base_Reactions` `Prereq::Orgo::Alcohols` `Prereq::Orgo::Polycyclic_and_Heterocyclic_Aromatic_Compounds`
- **Difficulty:** 4
- **Question:** How does adding a nitro group at the para position of phenol affect its acidity relative to unsubstituted phenol?
- **A:** It increases acidity by stabilizing the phenoxide through resonance and induction
- **B:** It decreases acidity by donating electron density into the ring
- **C:** It has no effect because the group is too far from the -OH
- **D:** It converts phenol into a base
- **Correct:** A
- **Explanation:** A para-nitro group withdraws electron density and delocalizes the phenoxide negative charge onto its own oxygens by resonance, stabilizing the conjugate base and making para-nitrophenol much more acidic than phenol.
- **Tags:** `KC::Orgo::Phenols` `Prereq::Orgo::Acid_Base_Reactions` `Prereq::Orgo::Alcohols` `Prereq::Orgo::Polycyclic_and_Heterocyclic_Aromatic_Compounds` `MCAT::Chem_Phys` `Difficulty::4`

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

### MCAT-ORG-ACID-002

- **KC:** `Orgo::Acid_Derivatives`
- **Prereqs:** `Prereq::Orgo::Carboxylic_Acids` `Prereq::Orgo::Nucleophilic_Acyl_Substitution`
- **Difficulty:** 2
- **Question:** Fischer esterification combines a carboxylic acid and an alcohol under acid catalysis. Besides the ester, what is the other product of this equilibrium?
- **A:** Hydrogen gas
- **B:** Water
- **C:** Carbon dioxide
- **D:** Ammonia
- **Correct:** B
- **Explanation:** Acid-catalyzed condensation of a carboxylic acid with an alcohol yields an ester and water; removing the water drives the equilibrium toward the ester.
- **Tags:** `KC::Orgo::Acid_Derivatives` `Prereq::Orgo::Carboxylic_Acids` `Prereq::Orgo::Nucleophilic_Acyl_Substitution` `MCAT::Chem_Phys` `Difficulty::2`

### MCAT-ORG-ACID-003

- **KC:** `Orgo::Acid_Derivatives`
- **Prereqs:** `Prereq::Orgo::Carboxylic_Acids` `Prereq::Orgo::Nucleophilic_Acyl_Substitution`
- **Difficulty:** 3
- **Question:** Complete acidic hydrolysis of a nitrile (a group containing a carbon-nitrogen triple bond) ultimately produces which functional group?
- **A:** A primary amine
- **B:** An aldehyde
- **C:** A carboxylic acid
- **D:** An ether
- **Correct:** C
- **Explanation:** Full hydrolysis adds water across the carbon-nitrogen triple bond and proceeds through an amide intermediate to give a carboxylic acid, releasing ammonia or ammonium.
- **Tags:** `KC::Orgo::Acid_Derivatives` `Prereq::Orgo::Carboxylic_Acids` `Prereq::Orgo::Nucleophilic_Acyl_Substitution` `MCAT::Chem_Phys` `Difficulty::3`

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

### MCAT-ORG-CARC-002

- **KC:** `Orgo::Carbohydrate_Chemistry`
- **Prereqs:** `Prereq::Orgo::Alcohols` `Prereq::Orgo::Nucleophilic_Addition` `Prereq::Orgo::Stereochemistry`
- **Difficulty:** 2
- **Question:** In its open-chain form, glucose contains an aldehyde group. Monosaccharides whose carbonyl is an aldehyde are classified as what?
- **A:** Ketoses
- **B:** Disaccharides
- **C:** Amino sugars
- **D:** Aldoses
- **Correct:** D
- **Explanation:** A monosaccharide with an aldehyde carbonyl is an aldose (glucose), whereas one with a ketone carbonyl is a ketose (fructose).
- **Tags:** `KC::Orgo::Carbohydrate_Chemistry` `Prereq::Orgo::Alcohols` `Prereq::Orgo::Nucleophilic_Addition` `Prereq::Orgo::Stereochemistry` `MCAT::Chem_Phys` `Difficulty::2`

### MCAT-ORG-CARC-003

- **KC:** `Orgo::Carbohydrate_Chemistry`
- **Prereqs:** `Prereq::Orgo::Alcohols` `Prereq::Orgo::Nucleophilic_Addition` `Prereq::Orgo::Stereochemistry`
- **Difficulty:** 4
- **Question:** When open-chain glucose cyclizes, a new stereocenter forms at the former carbonyl carbon, producing alpha and beta forms. These two cyclic forms are specifically called what?
- **A:** Anomers
- **B:** Enantiomers
- **C:** Constitutional isomers
- **D:** Epimers at carbon 3
- **Correct:** A
- **Explanation:** The two ring forms differing only in configuration at the new anomeric carbon (the former carbonyl carbon) are anomers, labeled alpha and beta.
- **Tags:** `KC::Orgo::Carbohydrate_Chemistry` `Prereq::Orgo::Alcohols` `Prereq::Orgo::Nucleophilic_Addition` `Prereq::Orgo::Stereochemistry` `MCAT::Chem_Phys` `Difficulty::4`

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

### MCAT-ORG-AMIAPC-002

- **KC:** `Orgo::Amino_Acid_and_Peptide_Chemistry`
- **Prereqs:** `Prereq::Orgo::Acid_Base_Reactions` `Prereq::Orgo::Amines` `Prereq::Orgo::Nucleophilic_Acyl_Substitution` `Prereq::Orgo::Stereochemistry`
- **Difficulty:** 2
- **Question:** At physiological pH (about 7.4), a typical amino acid with a nonionizable side chain exists predominantly as which species?
- **A:** A neutral molecule with intact -NH2 and -COOH groups
- **B:** A zwitterion with an -NH3+ group and a -COO- group
- **C:** A fully protonated cation
- **D:** A fully deprotonated anion
- **Correct:** B
- **Explanation:** Near neutral pH the carboxyl group is deprotonated (-COO-) and the amino group is protonated (-NH3+), giving a net-neutral zwitterion.
- **Tags:** `KC::Orgo::Amino_Acid_and_Peptide_Chemistry` `Prereq::Orgo::Acid_Base_Reactions` `Prereq::Orgo::Amines` `Prereq::Orgo::Nucleophilic_Acyl_Substitution` `Prereq::Orgo::Stereochemistry` `MCAT::Chem_Phys` `Difficulty::2`

### MCAT-ORG-AMIAPC-003

- **KC:** `Orgo::Amino_Acid_and_Peptide_Chemistry`
- **Prereqs:** `Prereq::Orgo::Acid_Base_Reactions` `Prereq::Orgo::Amines` `Prereq::Orgo::Nucleophilic_Acyl_Substitution` `Prereq::Orgo::Stereochemistry`
- **Difficulty:** 4
- **Question:** For an amino acid whose only ionizable groups are the alpha-carboxyl and alpha-amino groups, how is the isoelectric point (pI) calculated?
- **A:** As the sum of the two pKa values
- **B:** As the difference between pH 7 and the lower pKa
- **C:** As the average of the two pKa values
- **D:** It is always exactly 7.0
- **Correct:** C
- **Explanation:** With two ionizable groups, the pI is the average of the two pKa values, the pH at which the zwitterion predominates and the net charge is zero.
- **Tags:** `KC::Orgo::Amino_Acid_and_Peptide_Chemistry` `Prereq::Orgo::Acid_Base_Reactions` `Prereq::Orgo::Amines` `Prereq::Orgo::Nucleophilic_Acyl_Substitution` `Prereq::Orgo::Stereochemistry` `MCAT::Chem_Phys` `Difficulty::4`

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

### MCAT-ORG-IRS-002

- **KC:** `Orgo::IR_Spectroscopy`
- **Prereqs:** `Prereq::Orgo::Functional_Groups` `Prereq::Physics::Electromagnetic_Radiation`
- **Difficulty:** 1
- **Question:** Absorption of infrared radiation by a molecule primarily excites which kind of motion?
- **A:** Flipping of nuclear spins in a magnetic field
- **B:** Ejection of core (inner-shell) electrons
- **C:** Radioactive decay of the nucleus
- **D:** Bond vibrations such as stretching and bending
- **Correct:** D
- **Explanation:** Infrared photon energies match vibrational energy gaps, so IR absorption excites bond stretching and bending motions.
- **Tags:** `KC::Orgo::IR_Spectroscopy` `Prereq::Orgo::Functional_Groups` `Prereq::Physics::Electromagnetic_Radiation` `MCAT::Chem_Phys` `Difficulty::1`

### MCAT-ORG-IRS-003

- **KC:** `Orgo::IR_Spectroscopy`
- **Prereqs:** `Prereq::Orgo::Functional_Groups` `Prereq::Physics::Electromagnetic_Radiation`
- **Difficulty:** 3
- **Question:** A broad, strong infrared absorption spanning roughly 3200 to 3550 reciprocal centimeters most likely indicates which group?
- **A:** An O-H (hydroxyl) stretch, as in an alcohol
- **B:** A carbonyl (C=O) stretch
- **C:** An isolated carbon-carbon single bond
- **D:** A carbon-fluorine bond
- **Correct:** A
- **Explanation:** A broad band near 3200 to 3550 reciprocal centimeters is characteristic of an alcohol O-H stretch, broadened by hydrogen bonding.
- **Tags:** `KC::Orgo::IR_Spectroscopy` `Prereq::Orgo::Functional_Groups` `Prereq::Physics::Electromagnetic_Radiation` `MCAT::Chem_Phys` `Difficulty::3`

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

### MCAT-ORG-NMRS-002

- **KC:** `Orgo::NMR_Spectroscopy`
- **Prereqs:** `Prereq::Orgo::Functional_Groups` `Prereq::Orgo::Hybridization` `Prereq::Physics::Electromagnetic_Radiation`
- **Difficulty:** 3
- **Question:** In proton (1H) NMR, what does the integrated area (integration) of a signal indicate?
- **A:** The chemical shift of the protons in ppm
- **B:** The relative number of protons producing that signal
- **C:** The number of neighboring carbon atoms
- **D:** The molecular mass of the compound
- **Correct:** B
- **Explanation:** The integrated area of a 1H NMR signal is proportional to the number of equivalent protons giving rise to it.
- **Tags:** `KC::Orgo::NMR_Spectroscopy` `Prereq::Orgo::Functional_Groups` `Prereq::Orgo::Hybridization` `Prereq::Physics::Electromagnetic_Radiation` `MCAT::Chem_Phys` `Difficulty::3`

### MCAT-ORG-NMRS-003

- **KC:** `Orgo::NMR_Spectroscopy`
- **Prereqs:** `Prereq::Orgo::Functional_Groups` `Prereq::Orgo::Hybridization` `Prereq::Physics::Electromagnetic_Radiation`
- **Difficulty:** 5
- **Question:** In the 1H NMR spectrum of pure ethanol, the CH2 protons appear as a quartet. By the n+1 rule, this indicates coupling to how many equivalent neighbors, and to which group do they belong?
- **A:** Two neighbors, belonging to the OH proton
- **B:** One neighbor, belonging to the OH proton
- **C:** Three neighbors, belonging to the adjacent CH3 group
- **D:** Four neighbors, belonging to a second CH2 group
- **Correct:** C
- **Explanation:** A quartet corresponds to n+1 = 4, so three equivalent neighboring protons, which are the three hydrogens of the adjacent methyl (CH3) group.
- **Tags:** `KC::Orgo::NMR_Spectroscopy` `Prereq::Orgo::Functional_Groups` `Prereq::Orgo::Hybridization` `Prereq::Physics::Electromagnetic_Radiation` `MCAT::Chem_Phys` `Difficulty::5`

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

### MCAT-ORG-MASS-002

- **KC:** `Orgo::Mass_Spectrometry`
- **Prereqs:** `Prereq::Orgo::Functional_Groups` `Prereq::Orgo::Reaction_Mechanisms_Overview`
- **Difficulty:** 2
- **Question:** In a typical electron-ionization mass spectrum, the molecular ion peak (M+) corresponds to what?
- **A:** The largest fragment left after the molecule breaks apart
- **B:** A doubly charged fragment ion
- **C:** The solvent used to dissolve the sample
- **D:** The intact molecule after it loses a single electron, a radical cation
- **Correct:** D
- **Explanation:** The molecular ion forms when the intact molecule loses one electron, giving a radical cation whose mass-to-charge ratio equals the molecular mass.
- **Tags:** `KC::Orgo::Mass_Spectrometry` `Prereq::Orgo::Functional_Groups` `Prereq::Orgo::Reaction_Mechanisms_Overview` `MCAT::Chem_Phys` `Difficulty::2`

### MCAT-ORG-MASS-003

- **KC:** `Orgo::Mass_Spectrometry`
- **Prereqs:** `Prereq::Orgo::Functional_Groups` `Prereq::Orgo::Reaction_Mechanisms_Overview`
- **Difficulty:** 4
- **Question:** A compound shows a molecular ion at m/z = 72 and a strong fragment at m/z = 57. The neutral loss of 15 mass units most likely corresponds to loss of which fragment?
- **A:** A methyl radical (CH3)
- **B:** A water molecule
- **C:** A molecule of carbon monoxide
- **D:** A hydroxyl group plus a carbon
- **Correct:** A
- **Explanation:** A neutral loss of 15 mass units corresponds to a methyl radical (CH3, mass 15), a common fragmentation that leaves a more stabilized cation.
- **Tags:** `KC::Orgo::Mass_Spectrometry` `Prereq::Orgo::Functional_Groups` `Prereq::Orgo::Reaction_Mechanisms_Overview` `MCAT::Chem_Phys` `Difficulty::4`

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

### MCAT-ORG-MOLSAS-002

- **KC:** `Orgo::Molecular_Structure_and_Absorption_Spectra`
- **Prereqs:** `Prereq::Orgo::Hybridization` `Prereq::Orgo::Polycyclic_and_Heterocyclic_Aromatic_Compounds` `Prereq::Physics::Electromagnetic_Radiation`
- **Difficulty:** 2
- **Question:** Absorption of ultraviolet-visible light by an organic molecule corresponds to which kind of transition?
- **A:** A nuclear spin flip in a magnetic field
- **B:** Promotion of an electron from a lower to a higher energy molecular orbital
- **C:** A change in a bond's vibrational state only
- **D:** Ejection of a neutron from the nucleus
- **Correct:** B
- **Explanation:** UV-Vis photons promote valence electrons between molecular orbitals, such as a pi to pi-star transition.
- **Tags:** `KC::Orgo::Molecular_Structure_and_Absorption_Spectra` `Prereq::Orgo::Hybridization` `Prereq::Orgo::Polycyclic_and_Heterocyclic_Aromatic_Compounds` `Prereq::Physics::Electromagnetic_Radiation` `MCAT::Chem_Phys` `Difficulty::2`

### MCAT-ORG-MOLSAS-003

- **KC:** `Orgo::Molecular_Structure_and_Absorption_Spectra`
- **Prereqs:** `Prereq::Orgo::Hybridization` `Prereq::Orgo::Polycyclic_and_Heterocyclic_Aromatic_Compounds` `Prereq::Physics::Electromagnetic_Radiation`
- **Difficulty:** 4
- **Question:** The part of a molecule responsible for absorbing UV-Vis light, such as an extended conjugated pi system, is given which name?
- **A:** A leaving group
- **B:** An isotope pattern
- **C:** A chromophore
- **D:** A stereocenter
- **Correct:** C
- **Explanation:** The light-absorbing region of a molecule is its chromophore; conjugated pi systems are common chromophores, and extending conjugation shifts absorption toward longer wavelengths.
- **Tags:** `KC::Orgo::Molecular_Structure_and_Absorption_Spectra` `Prereq::Orgo::Hybridization` `Prereq::Orgo::Polycyclic_and_Heterocyclic_Aromatic_Compounds` `Prereq::Physics::Electromagnetic_Radiation` `MCAT::Chem_Phys` `Difficulty::4`

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

### MCAT-ORG-SEPP-002

- **KC:** `Orgo::Separations_and_Purifications`
- **Prereqs:** `Prereq::GenChem::Intermolecular_Forces` `Prereq::GenChem::Solutions_and_Solubility` `Prereq::Orgo::Acid_Base_Reactions` `Prereq::Orgo::Functional_Groups`
- **Difficulty:** 1
- **Question:** Which technique separates two miscible liquids primarily by exploiting their different boiling points?
- **A:** Gravity filtration
- **B:** Recrystallization
- **C:** Centrifugation
- **D:** Distillation
- **Correct:** D
- **Explanation:** Distillation separates liquids by differences in boiling point (volatility), vaporizing the more volatile component preferentially.
- **Tags:** `KC::Orgo::Separations_and_Purifications` `Prereq::GenChem::Intermolecular_Forces` `Prereq::GenChem::Solutions_and_Solubility` `Prereq::Orgo::Acid_Base_Reactions` `Prereq::Orgo::Functional_Groups` `MCAT::Chem_Phys` `Difficulty::1`

### MCAT-ORG-SEPP-003

- **KC:** `Orgo::Separations_and_Purifications`
- **Prereqs:** `Prereq::GenChem::Intermolecular_Forces` `Prereq::GenChem::Solutions_and_Solubility` `Prereq::Orgo::Acid_Base_Reactions` `Prereq::Orgo::Functional_Groups`
- **Difficulty:** 3
- **Question:** In thin-layer chromatography on polar silica gel developed with a relatively nonpolar solvent, which compound generally travels FARTHEST up the plate (highest Rf)?
- **A:** The least polar compound
- **B:** The most polar compound
- **C:** The compound of highest molecular weight
- **D:** An ionic compound
- **Correct:** A
- **Explanation:** On polar silica, polar compounds bind the stationary phase and move little; the least polar compound interacts least with silica and spends more time in the mobile phase, giving the highest Rf.
- **Tags:** `KC::Orgo::Separations_and_Purifications` `Prereq::GenChem::Intermolecular_Forces` `Prereq::GenChem::Solutions_and_Solubility` `Prereq::Orgo::Acid_Base_Reactions` `Prereq::Orgo::Functional_Groups` `MCAT::Chem_Phys` `Difficulty::3`

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

### MCAT-ORG-LABT-002

- **KC:** `Orgo::Laboratory_Techniques`
- **Prereqs:** `Prereq::Orgo::IR_Spectroscopy` `Prereq::Orgo::Mass_Spectrometry` `Prereq::Orgo::NMR_Spectroscopy` `Prereq::Orgo::Separations_and_Purifications`
- **Difficulty:** 3
- **Question:** To quickly gauge the identity and purity of a crystalline organic solid, a chemist commonly measures which physical property and compares it with a literature value?
- **A:** The sign of its optical rotation only
- **B:** Its melting point
- **C:** The refractive index of its vapor
- **D:** Its radioactive half-life
- **Correct:** B
- **Explanation:** A pure crystalline solid melts over a narrow range at a characteristic temperature; a sharp value matching the literature supports identity and purity, while impurities broaden and lower the melting range.
- **Tags:** `KC::Orgo::Laboratory_Techniques` `Prereq::Orgo::IR_Spectroscopy` `Prereq::Orgo::Mass_Spectrometry` `Prereq::Orgo::NMR_Spectroscopy` `Prereq::Orgo::Separations_and_Purifications` `MCAT::Chem_Phys` `Difficulty::3`

### MCAT-ORG-LABT-003

- **KC:** `Orgo::Laboratory_Techniques`
- **Prereqs:** `Prereq::Orgo::IR_Spectroscopy` `Prereq::Orgo::Mass_Spectrometry` `Prereq::Orgo::NMR_Spectroscopy` `Prereq::Orgo::Separations_and_Purifications`
- **Difficulty:** 5
- **Question:** In recrystallization, why is the impure solid dissolved in a minimum volume of hot solvent and then cooled slowly?
- **A:** So the compound remains permanently dissolved and is recovered only by evaporation
- **B:** So the impurities crystallize first and can be collected
- **C:** So the major component becomes supersaturated and crystallizes on cooling while trace impurities stay in solution
- **D:** So the solvent evaporates completely, leaving all solutes behind
- **Correct:** C
- **Explanation:** A hot saturated solution becomes supersaturated in the major component as it cools, so that compound crystallizes selectively while small amounts of impurity remain dissolved and are removed by filtration.
- **Tags:** `KC::Orgo::Laboratory_Techniques` `Prereq::Orgo::IR_Spectroscopy` `Prereq::Orgo::Mass_Spectrometry` `Prereq::Orgo::NMR_Spectroscopy` `Prereq::Orgo::Separations_and_Purifications` `MCAT::Chem_Phys` `Difficulty::5`
