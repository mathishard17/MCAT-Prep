# MCAT Demo Flashcards

Synthetic 50-card multiple-choice demo deck for Concept Scheduler testing.

## Tags

- Section tag: `MCAT::Bio_Biochem`
- KC tag: `KC::<domain>::<component>`
- Prerequisite tag: `Prereq::<domain>::<component>`
- Difficulty tag: `Difficulty::<1-5>`

---

## Bio::DNA

### MCAT-BIO-DNA-001

- **KC:** `Bio::DNA`
- **Prereqs:** none
- **Difficulty:** 2
- **Question:** A researcher finds that a DNA sample has a higher melting temperature than expected. Which feature most likely explains this observation?
- **A:** A higher proportion of A-T base pairs
- **B:** A higher proportion of G-C base pairs
- **C:** More frequent uracil incorporation
- **D:** Fewer phosphodiester bonds
- **Correct:** B
- **Explanation:** G-C pairs form three hydrogen bonds and stronger base stacking interactions, making DNA strands harder to separate.
- **Tags:** `KC::Bio::DNA` `MCAT::Bio_Biochem` `Difficulty::2`

### MCAT-BIO-DNA-002

- **KC:** `Bio::DNA`
- **Prereqs:** none
- **Difficulty:** 3
- **Question:** During DNA replication, why is a primer required before DNA polymerase can extend a new strand?
- **A:** DNA polymerase can only add nucleotides to an existing 3' hydroxyl group
- **B:** DNA polymerase needs a double-stranded RNA template
- **C:** Primers prevent complementary base pairing
- **D:** Primers remove supercoiling ahead of the replication fork
- **Correct:** A
- **Explanation:** DNA polymerase cannot start synthesis de novo; it extends from a free 3' OH provided by a primer.
- **Tags:** `KC::Bio::DNA` `MCAT::Bio_Biochem` `Difficulty::3`

### MCAT-BIO-DNA-003

- **KC:** `Bio::DNA`
- **Prereqs:** none
- **Difficulty:** 3
- **Question:** A mutation changes one DNA base but does not change the amino acid sequence of the encoded protein. What best explains this outcome?
- **A:** DNA replication is conservative
- **B:** The genetic code is degenerate
- **C:** Introns are translated into protein
- **D:** Ribosomes proofread DNA directly
- **Correct:** B
- **Explanation:** Multiple codons can encode the same amino acid, so some base substitutions are silent.
- **Tags:** `KC::Bio::DNA` `MCAT::Bio_Biochem` `Difficulty::3`

### MCAT-BIO-DNA-004

- **KC:** `Bio::DNA`
- **Prereqs:** none
- **Difficulty:** 4
- **Question:** If topoisomerase activity is inhibited during DNA replication, what problem is most likely to occur?
- **A:** RNA primers cannot be synthesized
- **B:** Okazaki fragments cannot be joined
- **C:** Torsional strain builds up ahead of the replication fork
- **D:** Complementary base pairing is eliminated
- **Correct:** C
- **Explanation:** Topoisomerases relieve supercoiling generated as helicase unwinds DNA ahead of the fork.
- **Tags:** `KC::Bio::DNA` `MCAT::Bio_Biochem` `Difficulty::4`

### MCAT-BIO-DNA-005

- **KC:** `Bio::DNA`
- **Prereqs:** none
- **Difficulty:** 4
- **Question:** A chemical causes deamination of cytosine, converting it to uracil in DNA. Why is this potentially mutagenic if unrepaired?
- **A:** Uracil pairs with adenine instead of guanine
- **B:** Uracil prevents DNA strands from separating
- **C:** Uracil creates an extra phosphate group
- **D:** Uracil stops all transcription permanently
- **Correct:** A
- **Explanation:** Cytosine normally pairs with guanine, but uracil pairs with adenine, which can cause a base-pair change after replication.
- **Tags:** `KC::Bio::DNA` `MCAT::Bio_Biochem` `Difficulty::4`

## Bio::Genetics

### MCAT-BIO-GEN-001

- **KC:** `Bio::Genetics`
- **Prereqs:** `Prereq::Bio::DNA`
- **Difficulty:** 2
- **Question:** In a heterozygote, one allele fully masks the phenotypic effect of another allele. Which inheritance pattern is shown?
- **A:** Codominance
- **B:** Incomplete dominance
- **C:** Complete dominance
- **D:** Mitochondrial inheritance
- **Correct:** C
- **Explanation:** In complete dominance, the dominant allele determines the heterozygous phenotype.
- **Tags:** `KC::Bio::Genetics` `Prereq::Bio::DNA` `MCAT::Bio_Biochem` `Difficulty::2`

### MCAT-BIO-GEN-002

- **KC:** `Bio::Genetics`
- **Prereqs:** `Prereq::Bio::DNA`
- **Difficulty:** 3
- **Question:** Two genes are located very close together on the same chromosome. What inheritance pattern is most likely?
- **A:** They assort independently in every meiosis
- **B:** They are inherited together more often than expected by chance
- **C:** They always mutate at the same rate
- **D:** They are expressed only in gametes
- **Correct:** B
- **Explanation:** Linked genes are less likely to be separated by crossing over, so they tend to be inherited together.
- **Tags:** `KC::Bio::Genetics` `Prereq::Bio::DNA` `MCAT::Bio_Biochem` `Difficulty::3`

### MCAT-BIO-GEN-003

- **KC:** `Bio::Genetics`
- **Prereqs:** `Prereq::Bio::DNA`
- **Difficulty:** 3
- **Question:** A recessive disease allele is present in a carrier who has no symptoms. What best explains the carrier's normal phenotype?
- **A:** The recessive allele is absent from somatic cells
- **B:** One functional allele provides enough gene product for normal function
- **C:** Recessive alleles are never transcribed
- **D:** The allele is removed during meiosis
- **Correct:** B
- **Explanation:** Many recessive disorders occur only when both alleles lose function; one working copy can be sufficient.
- **Tags:** `KC::Bio::Genetics` `Prereq::Bio::DNA` `MCAT::Bio_Biochem` `Difficulty::3`

### MCAT-BIO-GEN-004

- **KC:** `Bio::Genetics`
- **Prereqs:** `Prereq::Bio::DNA`
- **Difficulty:** 4
- **Question:** A trait appears in every generation and affects males and females equally. An affected heterozygous parent and unaffected parent have children. Which inheritance pattern best fits?
- **A:** Autosomal dominant
- **B:** Autosomal recessive
- **C:** X-linked recessive
- **D:** Mitochondrial
- **Correct:** A
- **Explanation:** Autosomal dominant traits often show vertical transmission and affect both sexes similarly.
- **Tags:** `KC::Bio::Genetics` `Prereq::Bio::DNA` `MCAT::Bio_Biochem` `Difficulty::4`

### MCAT-BIO-GEN-005

- **KC:** `Bio::Genetics`
- **Prereqs:** `Prereq::Bio::DNA`
- **Difficulty:** 5
- **Question:** A population is in Hardy-Weinberg equilibrium. If the frequency of a recessive disease phenotype is 0.01, what is the approximate carrier frequency?
- **A:** 0.01
- **B:** 0.02
- **C:** 0.18
- **D:** 0.99
- **Correct:** C
- **Explanation:** The recessive phenotype frequency is q^2 = 0.01, so q = 0.1 and p is about 0.9; carrier frequency is 2pq, about 0.18.
- **Tags:** `KC::Bio::Genetics` `Prereq::Bio::DNA` `MCAT::Bio_Biochem` `Difficulty::5`

## Bio::Eukaryotic_Cell

### MCAT-BIO-EUK-CELL-001

- **KC:** `Bio::Eukaryotic_Cell`
- **Prereqs:** none
- **Difficulty:** 2
- **Question:** A drug disrupts the formation of transport vesicles from the rough endoplasmic reticulum. Which cellular process would be most directly impaired?
- **A:** ATP synthesis by oxidative phosphorylation
- **B:** Delivery of newly synthesized secretory proteins to the Golgi apparatus
- **C:** Replication of nuclear DNA during S phase
- **D:** Breakdown of fatty acids in peroxisomes
- **Correct:** B
- **Explanation:** Secretory and membrane proteins made on the rough ER are packaged into vesicles that move to the Golgi for modification and sorting.
- **Tags:** `KC::Bio::Eukaryotic_Cell` `MCAT::Bio_Biochem` `Difficulty::2`

### MCAT-BIO-EUK-CELL-002

- **KC:** `Bio::Eukaryotic_Cell`
- **Prereqs:** none
- **Difficulty:** 3
- **Question:** A cell shows swollen mitochondria and reduced oxygen consumption. Which consequence is most likely?
- **A:** Increased lysosomal digestion of extracellular proteins
- **B:** Decreased ATP production from the electron transport chain
- **C:** Increased translation of proteins on free ribosomes
- **D:** Decreased transcription of ribosomal RNA in the nucleolus
- **Correct:** B
- **Explanation:** Mitochondria use oxygen as the final electron acceptor in oxidative phosphorylation, so impaired oxygen consumption reduces ATP production.
- **Tags:** `KC::Bio::Eukaryotic_Cell` `MCAT::Bio_Biochem` `Difficulty::3`

### MCAT-BIO-EUK-CELL-003

- **KC:** `Bio::Eukaryotic_Cell`
- **Prereqs:** none
- **Difficulty:** 3
- **Question:** A mutation prevents microtubule polymerization. Which cellular event would be most affected?
- **A:** Separation of chromosomes during mitosis
- **B:** Formation of peptide bonds during translation
- **C:** Diffusion of small nonpolar molecules across the membrane
- **D:** Hydrolysis of proteins inside lysosomes
- **Correct:** A
- **Explanation:** Microtubules form the mitotic spindle, which attaches to chromosomes and helps separate them during cell division.
- **Tags:** `KC::Bio::Eukaryotic_Cell` `MCAT::Bio_Biochem` `Difficulty::3`

### MCAT-BIO-EUK-CELL-004

- **KC:** `Bio::Eukaryotic_Cell`
- **Prereqs:** none
- **Difficulty:** 2
- **Question:** Why does compartmentalization benefit eukaryotic cells?
- **A:** It prevents all proteins from interacting with membranes
- **B:** It allows incompatible biochemical processes to occur in separate environments
- **C:** It eliminates the need for regulated transport
- **D:** It makes all metabolic reactions occur at the same pH
- **Correct:** B
- **Explanation:** Organelles create specialized environments, allowing processes such as lysosomal digestion, oxidative phosphorylation, and protein modification to occur efficiently.
- **Tags:** `KC::Bio::Eukaryotic_Cell` `MCAT::Bio_Biochem` `Difficulty::2`

### MCAT-BIO-EUK-CELL-005

- **KC:** `Bio::Eukaryotic_Cell`
- **Prereqs:** none
- **Difficulty:** 4
- **Question:** A protein normally found in lysosomes is instead secreted from the cell. Which broad process most likely failed?
- **A:** Nuclear import of transcription factors
- **B:** Sorting of proteins in the endomembrane system
- **C:** Assembly of actin filaments at the cell cortex
- **D:** Initiation of DNA replication at origins
- **Correct:** B
- **Explanation:** Lysosomal proteins pass through the ER and Golgi, where sorting signals help direct them to lysosomes rather than secretion.
- **Tags:** `KC::Bio::Eukaryotic_Cell` `MCAT::Bio_Biochem` `Difficulty::4`

## Biochem::Amino_Acids

### MCAT-BIOCHEM-AA-001

- **KC:** `Biochem::Amino_Acids`
- **Prereqs:** none
- **Difficulty:** 2
- **Question:** At physiological pH, most free amino acids exist primarily as zwitterions. What does this mean?
- **A:** They contain no charged groups
- **B:** They contain both a positively charged amino group and a negatively charged carboxylate group
- **C:** They contain only a negatively charged side chain
- **D:** They cannot participate in hydrogen bonding
- **Correct:** B
- **Explanation:** Near physiological pH, the amino group is usually protonated and the carboxyl group is usually deprotonated, giving both positive and negative charges.
- **Tags:** `KC::Biochem::Amino_Acids` `MCAT::Bio_Biochem` `Difficulty::2`

### MCAT-BIOCHEM-AA-002

- **KC:** `Biochem::Amino_Acids`
- **Prereqs:** none
- **Difficulty:** 3
- **Question:** A protein region embedded in the hydrophobic core of a membrane is most likely enriched in which type of amino acid side chain?
- **A:** Nonpolar side chains
- **B:** Positively charged side chains
- **C:** Negatively charged side chains
- **D:** Strongly acidic side chains
- **Correct:** A
- **Explanation:** The membrane interior is hydrophobic, so amino acids with nonpolar side chains are favored in membrane-spanning regions.
- **Tags:** `KC::Biochem::Amino_Acids` `MCAT::Bio_Biochem` `Difficulty::3`

### MCAT-BIOCHEM-AA-003

- **KC:** `Biochem::Amino_Acids`
- **Prereqs:** none
- **Difficulty:** 4
- **Question:** Replacing an acidic amino acid on a protein surface with a basic amino acid is most likely to affect protein function by changing what property?
- **A:** Peptide bond directionality
- **B:** Local charge interactions with nearby residues or ligands
- **C:** The identity of the genetic code codon table
- **D:** The chirality of every amino acid in the protein
- **Correct:** B
- **Explanation:** Acidic and basic side chains differ in charge at physiological pH, so substitutions can disrupt salt bridges, binding sites, or local electrostatic interactions.
- **Tags:** `KC::Biochem::Amino_Acids` `MCAT::Bio_Biochem` `Difficulty::4`

### MCAT-BIOCHEM-AA-004

- **KC:** `Biochem::Amino_Acids`
- **Prereqs:** none
- **Difficulty:** 3
- **Question:** Why can proline disrupt an alpha helix more than many other amino acids?
- **A:** Its side chain is always negatively charged
- **B:** Its ring structure restricts backbone flexibility and limits normal hydrogen bonding geometry
- **C:** It lacks a carboxyl group in proteins
- **D:** It forms peptide bonds only at extremely low pH
- **Correct:** B
- **Explanation:** Proline's cyclic structure constrains the backbone and can interfere with the regular hydrogen bonding pattern needed for alpha helices.
- **Tags:** `KC::Biochem::Amino_Acids` `MCAT::Bio_Biochem` `Difficulty::3`

### MCAT-BIOCHEM-AA-005

- **KC:** `Biochem::Amino_Acids`
- **Prereqs:** none
- **Difficulty:** 4
- **Question:** Two cysteine residues in a protein become oxidized to form a disulfide bond. What is the main structural consequence?
- **A:** The protein backbone is hydrolyzed
- **B:** A covalent cross-link can stabilize the folded protein structure
- **C:** The amino acids are converted into nucleotides
- **D:** All nonpolar side chains become charged
- **Correct:** B
- **Explanation:** Oxidation of cysteine thiols can form a covalent disulfide bridge, often stabilizing tertiary or quaternary structure.
- **Tags:** `KC::Biochem::Amino_Acids` `MCAT::Bio_Biochem` `Difficulty::4`

## Biochem::Peptides_and_Proteins

### BIOCHEM-PEP-001

- **KC:** `Biochem::Peptides_and_Proteins`
- **Prereqs:** `Prereq::Biochem::Amino_Acids`
- **Difficulty:** 1
- **Question:** Why does peptide bond formation reduce the flexibility of a protein backbone compared with freely rotating single bonds?
- **A:** The peptide bond is ionic and prevents nearby atoms from rotating
- **B:** The peptide bond has partial double-bond character due to resonance
- **C:** The peptide bond forms a disulfide bridge between adjacent residues
- **D:** The peptide bond is always buried in the hydrophobic core
- **Correct:** B
- **Explanation:** Resonance between the carbonyl and amide nitrogen gives the peptide bond partial double-bond character, making it planar and limiting rotation.
- **Tags:** `KC::Biochem::Peptides_and_Proteins` `Prereq::Biochem::Amino_Acids` `MCAT::Bio_Biochem` `Difficulty::1`

### BIOCHEM-PEP-002

- **KC:** `Biochem::Peptides_and_Proteins`
- **Prereqs:** `Prereq::Biochem::Amino_Acids`
- **Difficulty:** 2
- **Question:** A peptide contains several lysine and arginine residues exposed on its surface. Which interaction is most likely to help it bind DNA?
- **A:** Hydrophobic interactions with nucleotide bases
- **B:** Covalent bonding to the sugar-phosphate backbone
- **C:** Electrostatic attraction to the negatively charged phosphate backbone
- **D:** Hydrogen bonding only with deoxyribose sugars
- **Correct:** C
- **Explanation:** Lysine and arginine are positively charged at physiological pH, so they can interact favorably with the negatively charged phosphate backbone of DNA.
- **Tags:** `KC::Biochem::Peptides_and_Proteins` `Prereq::Biochem::Amino_Acids` `MCAT::Bio_Biochem` `Difficulty::2`

### BIOCHEM-PEP-003

- **KC:** `Biochem::Peptides_and_Proteins`
- **Prereqs:** `Prereq::Biochem::Amino_Acids`
- **Difficulty:** 2
- **Question:** A short peptide is placed in a solution with pH well above the pKa of its N-terminus and C-terminus. What general charge change is expected at the termini?
- **A:** The N-terminus becomes more positive and the C-terminus becomes neutral
- **B:** The N-terminus becomes neutral and the C-terminus remains negative
- **C:** Both termini become positively charged
- **D:** Both termini become neutral
- **Correct:** B
- **Explanation:** At high pH, the N-terminus tends to lose a proton and become neutral, while the C-terminus remains deprotonated and negatively charged.
- **Tags:** `KC::Biochem::Peptides_and_Proteins` `Prereq::Biochem::Amino_Acids` `MCAT::Bio_Biochem` `Difficulty::2`

### BIOCHEM-PEP-004

- **KC:** `Biochem::Peptides_and_Proteins`
- **Prereqs:** `Prereq::Biochem::Amino_Acids`
- **Difficulty:** 3
- **Question:** During protein synthesis, why is the amino acid sequence of a peptide considered directional?
- **A:** Peptides are synthesized and conventionally written from N-terminus to C-terminus
- **B:** Peptides are synthesized and conventionally written from C-terminus to N-terminus
- **C:** Every amino acid side chain points in the same direction
- **D:** Peptide bonds can form only between identical amino acids
- **Correct:** A
- **Explanation:** Proteins are synthesized by adding amino acids to the growing C-terminus, and sequences are conventionally written from the N-terminus to the C-terminus.
- **Tags:** `KC::Biochem::Peptides_and_Proteins` `Prereq::Biochem::Amino_Acids` `MCAT::Bio_Biochem` `Difficulty::3`

### BIOCHEM-PEP-005

- **KC:** `Biochem::Peptides_and_Proteins`
- **Prereqs:** `Prereq::Biochem::Amino_Acids`
- **Difficulty:** 3
- **Question:** A peptide becomes less soluble after several polar residues are replaced with nonpolar residues. What is the most likely explanation?
- **A:** Nonpolar side chains form stronger ion-dipole interactions with water
- **B:** Nonpolar side chains reduce favorable interactions with the aqueous environment
- **C:** Nonpolar side chains always break peptide bonds
- **D:** Nonpolar side chains eliminate all secondary structure
- **Correct:** B
- **Explanation:** Polar and charged residues can interact favorably with water, while nonpolar residues are less compatible with an aqueous environment and can reduce solubility.
- **Tags:** `KC::Biochem::Peptides_and_Proteins` `Prereq::Biochem::Amino_Acids` `MCAT::Bio_Biochem` `Difficulty::3`

## Biochem::Protein_Structure_and_Function

### BIOCHEM-PSF-001

- **KC:** `Biochem::Protein_Structure_and_Function`
- **Prereqs:** `Prereq::Biochem::Peptides_and_Proteins`
- **Difficulty:** 2
- **Question:** Which statement best explains why primary structure can strongly influence protein function?
- **A:** The amino acid sequence determines possible interactions that guide folding
- **B:** Primary structure directly measures the protein's final 3D volume
- **C:** Primary structure includes only disulfide bonds and prosthetic groups
- **D:** The amino acid sequence is unrelated to tertiary structure
- **Correct:** A
- **Explanation:** The sequence of amino acids determines side-chain chemistry and possible intramolecular interactions, which influence folding and function.
- **Tags:** `KC::Biochem::Protein_Structure_and_Function` `Prereq::Biochem::Peptides_and_Proteins` `MCAT::Bio_Biochem` `Difficulty::2`

### BIOCHEM-PSF-002

- **KC:** `Biochem::Protein_Structure_and_Function`
- **Prereqs:** `Prereq::Biochem::Peptides_and_Proteins`
- **Difficulty:** 2
- **Question:** What mainly stabilizes alpha helices and beta sheets in protein secondary structure?
- **A:** Hydrogen bonds between backbone atoms
- **B:** Covalent bonds between adjacent side chains
- **C:** Ionic bonds between all neighboring residues
- **D:** Hydrophobic interactions between peptide bonds
- **Correct:** A
- **Explanation:** Secondary structures are stabilized primarily by hydrogen bonding between backbone carbonyl oxygens and amide hydrogens.
- **Tags:** `KC::Biochem::Protein_Structure_and_Function` `Prereq::Biochem::Peptides_and_Proteins` `MCAT::Bio_Biochem` `Difficulty::2`

### BIOCHEM-PSF-003

- **KC:** `Biochem::Protein_Structure_and_Function`
- **Prereqs:** `Prereq::Biochem::Peptides_and_Proteins`
- **Difficulty:** 3
- **Question:** A mutation replaces a buried hydrophobic residue with a charged residue. Why might this destabilize the folded protein?
- **A:** Charged residues cannot participate in any protein interactions
- **B:** A charged residue is usually unfavorable in a nonpolar core without stabilizing interactions
- **C:** The mutation necessarily breaks every backbone hydrogen bond
- **D:** Charged residues always force proteins into beta sheets
- **Correct:** B
- **Explanation:** Protein interiors are often hydrophobic. Introducing an unsatisfied charged group into that environment can be energetically unfavorable and destabilize folding.
- **Tags:** `KC::Biochem::Protein_Structure_and_Function` `Prereq::Biochem::Peptides_and_Proteins` `MCAT::Bio_Biochem` `Difficulty::3`

### BIOCHEM-PSF-004

- **KC:** `Biochem::Protein_Structure_and_Function`
- **Prereqs:** `Prereq::Biochem::Peptides_and_Proteins`
- **Difficulty:** 4
- **Question:** An enzyme loses activity after heating but its amino acid sequence remains unchanged. What best explains the loss of function?
- **A:** Heat changed the primary structure by reversing peptide bond direction
- **B:** Heat disrupted higher-order structure needed to form the active site
- **C:** Heat converted all amino acids into carbohydrates
- **D:** Heat removed the genetic code from the protein
- **Correct:** B
- **Explanation:** Denaturation can disrupt secondary, tertiary, or quaternary structure without changing the primary sequence, altering the active site and reducing function.
- **Tags:** `KC::Biochem::Protein_Structure_and_Function` `Prereq::Biochem::Peptides_and_Proteins` `MCAT::Bio_Biochem` `Difficulty::4`

### BIOCHEM-PSF-005

- **KC:** `Biochem::Protein_Structure_and_Function`
- **Prereqs:** `Prereq::Biochem::Peptides_and_Proteins`
- **Difficulty:** 4
- **Question:** A protein functions only when four separate polypeptide chains assemble together. Which level of structure is most directly required for its activity?
- **A:** Primary structure only
- **B:** Secondary structure only
- **C:** Tertiary structure only
- **D:** Quaternary structure
- **Correct:** D
- **Explanation:** Quaternary structure refers to the arrangement of multiple polypeptide subunits, which can be essential for the function of multimeric proteins.
- **Tags:** `KC::Biochem::Protein_Structure_and_Function` `Prereq::Biochem::Peptides_and_Proteins` `MCAT::Bio_Biochem` `Difficulty::4`

## Biochem::Enzymes

### MCAT-BIOCHEM-ENZ-001

- **KC:** `Biochem::Enzymes`
- **Prereqs:** `Prereq::Biochem::Protein_Structure_and_Function`
- **Difficulty:** 2
- **Question:** An enzyme catalyzes a reaction by stabilizing the transition state. Which effect would this most directly have on the reaction?
- **A:** Increase the overall free energy change of the reaction
- **B:** Lower the activation energy
- **C:** Shift the equilibrium toward products
- **D:** Make an unfavorable reaction favorable
- **Correct:** B
- **Explanation:** Enzymes speed reactions by lowering activation energy, usually by stabilizing the transition state. They do not change delta G or the reaction equilibrium.
- **Tags:** `KC::Biochem::Enzymes` `Prereq::Biochem::Protein_Structure_and_Function` `MCAT::Bio_Biochem` `Difficulty::2`

### MCAT-BIOCHEM-ENZ-002

- **KC:** `Biochem::Enzymes`
- **Prereqs:** `Prereq::Biochem::Protein_Structure_and_Function`
- **Difficulty:** 3
- **Question:** A competitive inhibitor is added to an enzyme-catalyzed reaction. Which change is expected if substrate concentration is increased sufficiently?
- **A:** The inhibitor effect can be overcome
- **B:** Vmax permanently decreases
- **C:** The enzyme becomes denatured
- **D:** The reaction becomes nonspontaneous
- **Correct:** A
- **Explanation:** Competitive inhibitors compete with substrate for the active site, so high substrate concentration can reduce their effect. Vmax is unchanged, while apparent Km increases.
- **Tags:** `KC::Biochem::Enzymes` `Prereq::Biochem::Protein_Structure_and_Function` `MCAT::Bio_Biochem` `Difficulty::3`

### MCAT-BIOCHEM-ENZ-003

- **KC:** `Biochem::Enzymes`
- **Prereqs:** `Prereq::Biochem::Protein_Structure_and_Function`
- **Difficulty:** 3
- **Question:** An allosteric enzyme shows cooperative substrate binding. Which graph shape would most likely describe its reaction velocity as substrate concentration increases?
- **A:** Linear
- **B:** Hyperbolic
- **C:** Sigmoidal
- **D:** Flat at all substrate concentrations
- **Correct:** C
- **Explanation:** Cooperative enzymes often show sigmoidal kinetics because substrate binding changes the enzyme's conformation and affects binding at other sites.
- **Tags:** `KC::Biochem::Enzymes` `Prereq::Biochem::Protein_Structure_and_Function` `MCAT::Bio_Biochem` `Difficulty::3`

### MCAT-BIOCHEM-ENZ-004

- **KC:** `Biochem::Enzymes`
- **Prereqs:** `Prereq::Biochem::Protein_Structure_and_Function`
- **Difficulty:** 4
- **Question:** A mutation weakens an enzyme's ability to bind the transition state but does not affect substrate binding. What is the most likely consequence?
- **A:** Catalytic rate decreases
- **B:** Substrate concentration decreases at equilibrium
- **C:** Product becomes more stable than reactant
- **D:** The enzyme no longer binds any ligand
- **Correct:** A
- **Explanation:** Preferential transition-state stabilization is central to catalysis. If that stabilization is weakened, activation energy rises and the reaction rate decreases.
- **Tags:** `KC::Biochem::Enzymes` `Prereq::Biochem::Protein_Structure_and_Function` `MCAT::Bio_Biochem` `Difficulty::4`

### MCAT-BIOCHEM-ENZ-005

- **KC:** `Biochem::Enzymes`
- **Prereqs:** `Prereq::Biochem::Protein_Structure_and_Function`
- **Difficulty:** 4
- **Question:** A pathway product binds an early enzyme in the pathway and decreases its activity. What is the main biological purpose of this regulation?
- **A:** Prevent unnecessary production of the pathway product
- **B:** Increase entropy by unfolding the enzyme
- **C:** Convert an endergonic reaction into an exergonic one
- **D:** Eliminate the need for cofactors
- **Correct:** A
- **Explanation:** Feedback inhibition helps maintain metabolic balance by reducing pathway flux when enough product has accumulated.
- **Tags:** `KC::Biochem::Enzymes` `Prereq::Biochem::Protein_Structure_and_Function` `MCAT::Bio_Biochem` `Difficulty::4`

## Biochem::Bioenergetics

### MCAT-BIOCHEM-BIOEN-001

- **KC:** `Biochem::Bioenergetics`
- **Prereqs:** `Prereq::Biochem::Enzymes` `Prereq::Bio::Eukaryotic_Cell`
- **Difficulty:** 2
- **Question:** A reaction has a negative delta G under cellular conditions. What does this indicate?
- **A:** The reaction is thermodynamically favorable
- **B:** The reaction must occur rapidly
- **C:** The reaction requires ATP hydrolysis
- **D:** The reaction has no activation energy
- **Correct:** A
- **Explanation:** A negative delta G means a reaction is thermodynamically favorable, but it does not guarantee a fast rate. Kinetics depend on activation energy and catalysis.
- **Tags:** `KC::Biochem::Bioenergetics` `Prereq::Biochem::Enzymes` `Prereq::Bio::Eukaryotic_Cell` `MCAT::Bio_Biochem` `Difficulty::2`

### MCAT-BIOCHEM-BIOEN-002

- **KC:** `Biochem::Bioenergetics`
- **Prereqs:** `Prereq::Biochem::Enzymes` `Prereq::Bio::Eukaryotic_Cell`
- **Difficulty:** 3
- **Question:** Cells often couple ATP hydrolysis to an energetically unfavorable reaction. Why can this make the overall process favorable?
- **A:** The coupled delta G values add together
- **B:** ATP removes all activation energy
- **C:** ATP changes the laws of equilibrium
- **D:** The enzyme is consumed as fuel
- **Correct:** A
- **Explanation:** Coupled reactions are favorable when the total delta G is negative. ATP hydrolysis can provide enough negative delta G to drive an unfavorable step.
- **Tags:** `KC::Biochem::Bioenergetics` `Prereq::Biochem::Enzymes` `Prereq::Bio::Eukaryotic_Cell` `MCAT::Bio_Biochem` `Difficulty::3`

### MCAT-BIOCHEM-BIOEN-003

- **KC:** `Biochem::Bioenergetics`
- **Prereqs:** `Prereq::Biochem::Enzymes` `Prereq::Bio::Eukaryotic_Cell`
- **Difficulty:** 3
- **Question:** During oxidative phosphorylation, what is the immediate energy source used by ATP synthase to produce ATP?
- **A:** Proton movement down an electrochemical gradient
- **B:** Direct oxidation of glucose by ATP synthase
- **C:** Cleavage of oxygen into free radicals
- **D:** Heat released from the electron transport chain
- **Correct:** A
- **Explanation:** ATP synthase uses the proton-motive force generated by electron transport. Proton flow through the enzyme powers ATP formation.
- **Tags:** `KC::Biochem::Bioenergetics` `Prereq::Biochem::Enzymes` `Prereq::Bio::Eukaryotic_Cell` `MCAT::Bio_Biochem` `Difficulty::3`

### MCAT-BIOCHEM-BIOEN-004

- **KC:** `Biochem::Bioenergetics`
- **Prereqs:** `Prereq::Biochem::Enzymes` `Prereq::Bio::Eukaryotic_Cell`
- **Difficulty:** 4
- **Question:** A compound allows protons to cross the inner mitochondrial membrane without passing through ATP synthase. What is the expected effect?
- **A:** Electron transport may continue while ATP production decreases
- **B:** ATP production increases sharply
- **C:** Oxygen consumption must stop immediately
- **D:** NADH can no longer donate electrons
- **Correct:** A
- **Explanation:** An uncoupler dissipates the proton gradient, reducing ATP synthesis even if electron transport and oxygen consumption continue.
- **Tags:** `KC::Biochem::Bioenergetics` `Prereq::Biochem::Enzymes` `Prereq::Bio::Eukaryotic_Cell` `MCAT::Bio_Biochem` `Difficulty::4`

### MCAT-BIOCHEM-BIOEN-005

- **KC:** `Biochem::Bioenergetics`
- **Prereqs:** `Prereq::Biochem::Enzymes` `Prereq::Bio::Eukaryotic_Cell`
- **Difficulty:** 5
- **Question:** A cell has abundant NADH but impaired oxygen availability. Which outcome best explains why ATP production by oxidative phosphorylation falls?
- **A:** Electrons cannot efficiently flow to the terminal electron acceptor
- **B:** NADH no longer contains high-energy electrons
- **C:** ATP synthase converts ATP into oxygen
- **D:** The citric acid cycle directly pumps protons without electron transport
- **Correct:** A
- **Explanation:** Oxygen is the terminal electron acceptor in aerobic respiration. Without sufficient oxygen, electron transport slows, the proton gradient weakens, and oxidative ATP production falls.
- **Tags:** `KC::Biochem::Bioenergetics` `Prereq::Biochem::Enzymes` `Prereq::Bio::Eukaryotic_Cell` `MCAT::Bio_Biochem` `Difficulty::5`

## Biochem::Glycolysis

### MCAT-BB-GLY-001

- **KC:** `Biochem::Glycolysis`
- **Prereqs:** `Prereq::Biochem::Bioenergetics`
- **Difficulty:** 2
- **Question:** Why does glycolysis include an early ATP investment phase before ATP is produced?
- **A:** To make glucose less soluble so it can enter mitochondria
- **B:** To destabilize glucose and prepare it for cleavage into smaller phosphorylated intermediates
- **C:** To directly reduce NAD+ before pyruvate formation
- **D:** To bypass the need for enzyme catalysis later in the pathway
- **Correct:** B
- **Explanation:** Early phosphorylation traps glucose in the cell and makes later cleavage and energy payoff reactions more favorable.
- **Tags:** `KC::Biochem::Glycolysis` `Prereq::Biochem::Bioenergetics` `MCAT::Bio_Biochem` `Difficulty::2`

### MCAT-BB-GLY-002

- **KC:** `Biochem::Glycolysis`
- **Prereqs:** `Prereq::Biochem::Bioenergetics`
- **Difficulty:** 3
- **Question:** A cell with limited oxygen continues glycolysis mainly because glycolysis can still:
- **A:** fully oxidize pyruvate to CO2
- **B:** produce ATP through substrate-level phosphorylation
- **C:** generate most ATP through oxidative phosphorylation
- **D:** convert acetyl-CoA directly into lactate
- **Correct:** B
- **Explanation:** Glycolysis can make a small amount of ATP without oxygen by substrate-level phosphorylation, as long as NAD+ is regenerated.
- **Tags:** `KC::Biochem::Glycolysis` `Prereq::Biochem::Bioenergetics` `MCAT::Bio_Biochem` `Difficulty::3`

### MCAT-BB-GLY-003

- **KC:** `Biochem::Glycolysis`
- **Prereqs:** `Prereq::Biochem::Bioenergetics`
- **Difficulty:** 3
- **Question:** Why is NAD+ regeneration essential for glycolysis to continue under anaerobic conditions?
- **A:** NAD+ directly phosphorylates ADP to ATP
- **B:** NAD+ is required to accept electrons during oxidation of a glycolytic intermediate
- **C:** NAD+ converts pyruvate into acetyl-CoA in the cytosol
- **D:** NAD+ prevents glucose from entering the cell
- **Correct:** B
- **Explanation:** Glycolysis requires NAD+ as an electron acceptor; fermentation regenerates NAD+ so the pathway can continue.
- **Tags:** `KC::Biochem::Glycolysis` `Prereq::Biochem::Bioenergetics` `MCAT::Bio_Biochem` `Difficulty::3`

### MCAT-BB-GLY-004

- **KC:** `Biochem::Glycolysis`
- **Prereqs:** `Prereq::Biochem::Bioenergetics`
- **Difficulty:** 4
- **Question:** High cellular ATP would be expected to slow glycolysis because ATP signals that the cell:
- **A:** needs to increase glucose oxidation immediately
- **B:** has sufficient energy charge and should conserve glucose
- **C:** lacks enough phosphate groups for metabolic reactions
- **D:** must activate all irreversible glycolytic enzymes
- **Correct:** B
- **Explanation:** ATP acts as an energy-status signal; when ATP is abundant, glycolytic flux decreases to avoid unnecessary fuel breakdown.
- **Tags:** `KC::Biochem::Glycolysis` `Prereq::Biochem::Bioenergetics` `MCAT::Bio_Biochem` `Difficulty::4`

### MCAT-BB-GLY-005

- **KC:** `Biochem::Glycolysis`
- **Prereqs:** `Prereq::Biochem::Bioenergetics`
- **Difficulty:** 4
- **Question:** Compared with complete aerobic oxidation of glucose, glycolysis alone captures only a small fraction of glucose's available energy because:
- **A:** pyruvate still contains many high-energy electrons
- **B:** glycolysis consumes all ATP it produces
- **C:** glucose is reduced to carbon dioxide during glycolysis
- **D:** NADH cannot store chemical energy
- **Correct:** A
- **Explanation:** Glycolysis partially oxidizes glucose to pyruvate; much of the remaining energy is harvested later by the citric acid cycle and oxidative phosphorylation.
- **Tags:** `KC::Biochem::Glycolysis` `Prereq::Biochem::Bioenergetics` `MCAT::Bio_Biochem` `Difficulty::4`

## Biochem::Citric_Acid_Cycle

### MCAT-BB-CAC-001

- **KC:** `Biochem::Citric_Acid_Cycle`
- **Prereqs:** `Prereq::Biochem::Bioenergetics` `Prereq::Biochem::Glycolysis`
- **Difficulty:** 2
- **Question:** The citric acid cycle is best described as a pathway that primarily:
- **A:** splits glucose into two three-carbon molecules
- **B:** captures energy from acetyl-CoA as reduced electron carriers
- **C:** produces lactate to regenerate cytosolic NAD+
- **D:** directly uses oxygen to phosphorylate ADP
- **Correct:** B
- **Explanation:** The cycle oxidizes acetyl-CoA and stores much of the released energy in NADH and FADH2.
- **Tags:** `KC::Biochem::Citric_Acid_Cycle` `Prereq::Biochem::Bioenergetics` `Prereq::Biochem::Glycolysis` `MCAT::Bio_Biochem` `Difficulty::2`

### MCAT-BB-CAC-002

- **KC:** `Biochem::Citric_Acid_Cycle`
- **Prereqs:** `Prereq::Biochem::Bioenergetics` `Prereq::Biochem::Glycolysis`
- **Difficulty:** 3
- **Question:** Why does the citric acid cycle slow when the electron transport chain is impaired?
- **A:** Acetyl-CoA can no longer enter mitochondria
- **B:** NADH and FADH2 cannot be efficiently reoxidized to NAD+ and FAD
- **C:** Citrate can no longer be converted into glucose
- **D:** Carbon dioxide begins inhibiting glycolysis directly
- **Correct:** B
- **Explanation:** The cycle depends on oxidized electron carriers; if NADH and FADH2 are not reoxidized, key oxidation steps slow.
- **Tags:** `KC::Biochem::Citric_Acid_Cycle` `Prereq::Biochem::Bioenergetics` `Prereq::Biochem::Glycolysis` `MCAT::Bio_Biochem` `Difficulty::3`

### MCAT-BB-CAC-003

- **KC:** `Biochem::Citric_Acid_Cycle`
- **Prereqs:** `Prereq::Biochem::Bioenergetics` `Prereq::Biochem::Glycolysis`
- **Difficulty:** 3
- **Question:** The release of CO2 during the citric acid cycle reflects which broader process?
- **A:** Reduction of acetyl carbons into storage lipids
- **B:** Oxidation of carbon atoms originally entering metabolism as fuel
- **C:** Direct conversion of oxygen into carbon dioxide
- **D:** Phosphorylation of glucose before cleavage
- **Correct:** B
- **Explanation:** CO2 release indicates oxidative decarboxylation, in which fuel-derived carbons are oxidized while electrons are captured by carriers.
- **Tags:** `KC::Biochem::Citric_Acid_Cycle` `Prereq::Biochem::Bioenergetics` `Prereq::Biochem::Glycolysis` `MCAT::Bio_Biochem` `Difficulty::3`

### MCAT-BB-CAC-004

- **KC:** `Biochem::Citric_Acid_Cycle`
- **Prereqs:** `Prereq::Biochem::Bioenergetics` `Prereq::Biochem::Glycolysis`
- **Difficulty:** 4
- **Question:** A high NADH/NAD+ ratio would tend to inhibit the citric acid cycle because it signals:
- **A:** low electron carrier reduction and urgent need for more NADH
- **B:** abundant reduced electron carriers and limited need for further fuel oxidation
- **C:** complete depletion of acetyl-CoA
- **D:** irreversible conversion of citrate into pyruvate
- **Correct:** B
- **Explanation:** High NADH indicates that reducing power is already abundant, so additional oxidative steps in the cycle are downregulated.
- **Tags:** `KC::Biochem::Citric_Acid_Cycle` `Prereq::Biochem::Bioenergetics` `Prereq::Biochem::Glycolysis` `MCAT::Bio_Biochem` `Difficulty::4`

### MCAT-BB-CAC-005

- **KC:** `Biochem::Citric_Acid_Cycle`
- **Prereqs:** `Prereq::Biochem::Bioenergetics` `Prereq::Biochem::Glycolysis`
- **Difficulty:** 5
- **Question:** Why is the citric acid cycle considered amphibolic?
- **A:** It occurs only when oxygen is absent
- **B:** It both oxidizes fuel for energy and supplies intermediates for biosynthesis
- **C:** It replaces glycolysis in cells with mitochondria
- **D:** It produces ATP only by oxidative phosphorylation
- **Correct:** B
- **Explanation:** The cycle participates in catabolism by harvesting energy and in anabolism by providing carbon skeletons for biosynthetic pathways.
- **Tags:** `KC::Biochem::Citric_Acid_Cycle` `Prereq::Biochem::Bioenergetics` `Prereq::Biochem::Glycolysis` `MCAT::Bio_Biochem` `Difficulty::5`
