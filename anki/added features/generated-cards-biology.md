# MCAT Generated Cards — Biology

Synthetic, original multiple-choice cards for every Biology (`Bio::`) Knowledge
Component in the unified KC map (`added features/kc-map-unified.md`, §6 Biology
table). Block format is identical to `added features/mcat_demo_cards.md` so the
same importer (`rslib/src/scheduler/concept_demo.rs`) can parse it.

All content is original scope-based reasoning. No copyrighted prep text is
reproduced.

Conventions used here (from the generation harness / content contract):

- Area code for Biology is `BIO`; card IDs are `MCAT-BIO-<TOPIC>-<NNN>`, where
  `<TOPIC>` is the KC's stable `kc_code` topic (unique per KC).
- Each `## <KC id>` group holds two cards: one lower-band calibration item and
  one higher-band application/stretch item, both inside the KC's difficulty band.
- `KC::`/`Prereq::` tags are byte-exact to the unified map; the `Prereq::` set on
  each card equals that KC's canonical prerequisite set (union of intra- and
  cross-discipline edges), so cross-discipline prereqs (`Biochem::`, `GenChem::`,
  `Physics::`) appear where the map lists them.
- Primary section is `MCAT::Bio_Biochem` for every Bio KC (per the map).
- The two reused demo KCs `Bio::DNA` and `Bio::Genetics` are numbered from `006`
  so their IDs do not collide with the demo deck's existing `001`–`005` cards.

---

## Bio::DNA

### MCAT-BIO-DNA-006

- **KC:** `Bio::DNA`
- **Prereqs:** none
- **Difficulty:** 2
- **Question:** Which structural feature of DNA most directly allows each strand to act as a template for an identical copy during replication?
- **A:** Complementary base pairing between adenine-thymine and guanine-cytosine
- **B:** The antiparallel orientation of the two strands
- **C:** The presence of ribose sugar in the backbone
- **D:** A random sequence of nitrogenous bases
- **Correct:** A
- **Explanation:** Because adenine always pairs with thymine and guanine with cytosine, each strand specifies the exact sequence of its partner, enabling faithful copying.
- **Tags:** `KC::Bio::DNA` `MCAT::Bio_Biochem` `Difficulty::2`

### MCAT-BIO-DNA-007

- **KC:** `Bio::DNA`
- **Prereqs:** none
- **Difficulty:** 3
- **Question:** A sample of double-stranded DNA is found to be 20% adenine by base count. What percentage of the bases are guanine?
- **A:** 20%
- **B:** 30%
- **C:** 40%
- **D:** 60%
- **Correct:** B
- **Explanation:** With A equal to 20%, thymine is also 20%, leaving 60% split equally between guanine and cytosine, so guanine is 30%.
- **Tags:** `KC::Bio::DNA` `MCAT::Bio_Biochem` `Difficulty::3`

## Bio::DNA_Replication

### MCAT-BIO-DNAR-001

- **KC:** `Bio::DNA_Replication`
- **Prereqs:** `Prereq::Bio::DNA` `Prereq::Biochem::Enzymes`
- **Difficulty:** 2
- **Question:** DNA replication is described as semiconservative. What does this mean for the two daughter duplexes produced from one parental molecule?
- **A:** Each daughter molecule contains two newly synthesized strands
- **B:** One daughter molecule is entirely old and the other entirely new
- **C:** Each daughter molecule contains one parental strand and one new strand
- **D:** Both daughter molecules are made only of parental strands
- **Correct:** C
- **Explanation:** Semiconservative replication keeps one original template strand paired with one newly made strand in each daughter duplex.
- **Tags:** `KC::Bio::DNA_Replication` `Prereq::Bio::DNA` `Prereq::Biochem::Enzymes` `MCAT::Bio_Biochem` `Difficulty::2`

### MCAT-BIO-DNAR-002

- **KC:** `Bio::DNA_Replication`
- **Prereqs:** `Prereq::Bio::DNA` `Prereq::Biochem::Enzymes`
- **Difficulty:** 4
- **Question:** The lagging strand is built discontinuously as short Okazaki fragments. Which property of DNA polymerase most directly forces this discontinuous synthesis?
- **A:** Polymerase removes RNA primers before it elongates
- **B:** Polymerase unwinds the double helix ahead of the fork
- **C:** Polymerase joins fragments using a separate repair pathway
- **D:** Polymerase can add nucleotides only in the 5-prime to 3-prime direction
- **Correct:** D
- **Explanation:** Because synthesis proceeds only 5-prime to 3-prime, the strand oriented opposite to fork movement must be made in short pieces as template is exposed.
- **Tags:** `KC::Bio::DNA_Replication` `Prereq::Bio::DNA` `Prereq::Biochem::Enzymes` `MCAT::Bio_Biochem` `Difficulty::4`

## Bio::Transcription

### MCAT-BIO-TRA-001

- **KC:** `Bio::Transcription`
- **Prereqs:** `Prereq::Bio::DNA` `Prereq::Biochem::Nucleotides_and_Nucleic_Acids`
- **Difficulty:** 2
- **Question:** During transcription, which molecule is synthesized directly from a DNA template?
- **A:** A messenger RNA strand
- **B:** A double-stranded DNA copy
- **C:** A polypeptide chain
- **D:** A phospholipid molecule
- **Correct:** A
- **Explanation:** RNA polymerase reads a DNA template and builds a complementary single-stranded messenger RNA.
- **Tags:** `KC::Bio::Transcription` `Prereq::Bio::DNA` `Prereq::Biochem::Nucleotides_and_Nucleic_Acids` `MCAT::Bio_Biochem` `Difficulty::2`

### MCAT-BIO-TRA-002

- **KC:** `Bio::Transcription`
- **Prereqs:** `Prereq::Bio::DNA` `Prereq::Biochem::Nucleotides_and_Nucleic_Acids`
- **Difficulty:** 4
- **Question:** In a eukaryotic pre-mRNA, splicing mistakenly removes a coding exon along with the neighboring introns. What is the most likely consequence for the encoded protein?
- **A:** The mRNA will gain a longer poly-A tail
- **B:** The reading frame or amino acid sequence may be altered, changing the product
- **C:** Transcription will restart from a new promoter
- **D:** The gene's underlying DNA sequence will be permanently changed
- **Correct:** B
- **Explanation:** Exons carry coding information, so losing one can shift the reading frame or delete residues, yielding an altered or nonfunctional protein without changing the DNA.
- **Tags:** `KC::Bio::Transcription` `Prereq::Bio::DNA` `Prereq::Biochem::Nucleotides_and_Nucleic_Acids` `MCAT::Bio_Biochem` `Difficulty::4`

## Bio::Translation

### MCAT-BIO-TRA2-001

- **KC:** `Bio::Translation`
- **Prereqs:** `Prereq::Bio::Transcription` `Prereq::Biochem::Peptides_and_Proteins`
- **Difficulty:** 2
- **Question:** During translation, what determines the order in which amino acids are added to the growing polypeptide?
- **A:** The order of amino acids already inside the ribosome
- **B:** The number of introns in the original gene
- **C:** The sequence of codons in the mRNA
- **D:** The concentration of DNA polymerase
- **Correct:** C
- **Explanation:** Each mRNA codon is matched by a complementary tRNA anticodon that carries a specific amino acid, so codon order sets the amino acid sequence.
- **Tags:** `KC::Bio::Translation` `Prereq::Bio::Transcription` `Prereq::Biochem::Peptides_and_Proteins` `MCAT::Bio_Biochem` `Difficulty::2`

### MCAT-BIO-TRA2-002

- **KC:** `Bio::Translation`
- **Prereqs:** `Prereq::Bio::Transcription` `Prereq::Biochem::Peptides_and_Proteins`
- **Difficulty:** 4
- **Question:** A point mutation changes a codon that specified an amino acid into a stop codon partway through the message. What is the most likely effect on the protein?
- **A:** The protein will be longer than normal
- **B:** The protein sequence will be unchanged
- **C:** Translation will switch to the opposite DNA strand
- **D:** The protein will be truncated early during translation
- **Correct:** D
- **Explanation:** A premature stop codon ends translation early, producing a shortened and often nonfunctional protein.
- **Tags:** `KC::Bio::Translation` `Prereq::Bio::Transcription` `Prereq::Biochem::Peptides_and_Proteins` `MCAT::Bio_Biochem` `Difficulty::4`

## Bio::Gene_Expression_Regulation

### MCAT-BIO-GENER-001

- **KC:** `Bio::Gene_Expression_Regulation`
- **Prereqs:** `Prereq::Bio::Transcription` `Prereq::Bio::Translation`
- **Difficulty:** 3
- **Question:** In a bacterial operon, a repressor protein binds the operator when the pathway's end product is abundant. What is the effect on the operon's genes?
- **A:** Transcription is blocked, reducing enzyme production
- **B:** Transcription increases to make even more product
- **C:** The genes are permanently deleted from the chromosome
- **D:** The genes are translated without first being transcribed
- **Correct:** A
- **Explanation:** Repressor binding to the operator blocks RNA polymerase, decreasing transcription when the product is already plentiful.
- **Tags:** `KC::Bio::Gene_Expression_Regulation` `Prereq::Bio::Transcription` `Prereq::Bio::Translation` `MCAT::Bio_Biochem` `Difficulty::3`

### MCAT-BIO-GENER-002

- **KC:** `Bio::Gene_Expression_Regulation`
- **Prereqs:** `Prereq::Bio::Transcription` `Prereq::Bio::Translation`
- **Difficulty:** 5
- **Question:** Two cell types contain identical genomes yet express different proteins and stably maintain those patterns across divisions. Which mechanism best explains this without altering DNA sequence?
- **A:** Independent assortment of chromosomes during meiosis
- **B:** Epigenetic modifications such as DNA methylation and histone changes
- **C:** Random mutation of the two cells' coding sequences
- **D:** Selective loss of unused genes from each genome
- **Correct:** B
- **Explanation:** Heritable epigenetic marks change how accessible genes are for transcription, letting cells with the same DNA keep distinct, stable expression programs.
- **Tags:** `KC::Bio::Gene_Expression_Regulation` `Prereq::Bio::Transcription` `Prereq::Bio::Translation` `MCAT::Bio_Biochem` `Difficulty::5`

## Bio::Biotechnology

### MCAT-BIO-BIO-001

- **KC:** `Bio::Biotechnology`
- **Prereqs:** `Prereq::Bio::DNA` `Prereq::Bio::Genetics`
- **Difficulty:** 3
- **Question:** The polymerase chain reaction (PCR) amplifies a specific DNA region. Which component determines which region is copied?
- **A:** The heat-stable DNA polymerase
- **B:** The concentration of free nucleotides
- **C:** The pair of primers
- **D:** The maximum temperature of the thermal cycler
- **Correct:** C
- **Explanation:** Primers are short sequences complementary to the ends of the target, so their design defines which segment is amplified.
- **Tags:** `KC::Bio::Biotechnology` `Prereq::Bio::DNA` `Prereq::Bio::Genetics` `MCAT::Bio_Biochem` `Difficulty::3`

### MCAT-BIO-BIO-002

- **KC:** `Bio::Biotechnology`
- **Prereqs:** `Prereq::Bio::DNA` `Prereq::Bio::Genetics`
- **Difficulty:** 4
- **Question:** In gel electrophoresis of DNA, why do smaller fragments migrate farther toward the positive electrode than larger fragments?
- **A:** Smaller fragments carry a net positive charge
- **B:** Larger fragments are attracted to the negative electrode
- **C:** Larger fragments have no net charge
- **D:** Smaller fragments move more easily through the gel matrix
- **Correct:** D
- **Explanation:** DNA is uniformly negative and moves toward the positive electrode, and the gel sieves by size, so smaller fragments travel faster and farther.
- **Tags:** `KC::Bio::Biotechnology` `Prereq::Bio::DNA` `Prereq::Bio::Genetics` `MCAT::Bio_Biochem` `Difficulty::4`

## Bio::Genetics

### MCAT-BIO-GEN-006

- **KC:** `Bio::Genetics`
- **Prereqs:** `Prereq::Bio::DNA`
- **Difficulty:** 2
- **Question:** An organism has genotype Aa for a gene showing complete dominance. Which statement best describes its phenotype?
- **A:** The dominant allele determines the phenotype
- **B:** The phenotype reflects only the recessive allele
- **C:** The two alleles blend to give an intermediate phenotype
- **D:** The phenotype changes with each new generation
- **Correct:** A
- **Explanation:** Under complete dominance, the dominant allele masks the recessive allele, so the heterozygote shows the dominant phenotype.
- **Tags:** `KC::Bio::Genetics` `Prereq::Bio::DNA` `MCAT::Bio_Biochem` `Difficulty::2`

### MCAT-BIO-GEN-007

- **KC:** `Bio::Genetics`
- **Prereqs:** `Prereq::Bio::DNA`
- **Difficulty:** 4
- **Question:** A mutation yields a completely nonfunctional enzyme, yet heterozygous carriers are phenotypically normal. Which concept best explains the carriers' normal phenotype?
- **A:** Genetic linkage between two nearby genes
- **B:** One functional allele provides enough product for a normal phenotype
- **C:** Codominant expression of both alleles
- **D:** Nondisjunction during mitosis
- **Correct:** B
- **Explanation:** When a single working allele makes sufficient product, the loss-of-function allele behaves recessively and carriers appear unaffected.
- **Tags:** `KC::Bio::Genetics` `Prereq::Bio::DNA` `MCAT::Bio_Biochem` `Difficulty::4`

## Bio::Meiosis

### MCAT-BIO-MEI-001

- **KC:** `Bio::Meiosis`
- **Prereqs:** `Prereq::Bio::Cell_Cycle_and_Mitosis`
- **Difficulty:** 2
- **Question:** How do the gametes produced by meiosis differ from ordinary body cells?
- **A:** Gametes have twice the chromosome number
- **B:** Gametes contain only mitochondrial DNA
- **C:** Gametes are haploid, with half the chromosome number
- **D:** Gametes are unable to undergo fertilization
- **Correct:** C
- **Explanation:** Meiosis halves the chromosome number to produce haploid gametes, so that fertilization restores the diploid number.
- **Tags:** `KC::Bio::Meiosis` `Prereq::Bio::Cell_Cycle_and_Mitosis` `MCAT::Bio_Biochem` `Difficulty::2`

### MCAT-BIO-MEI-002

- **KC:** `Bio::Meiosis`
- **Prereqs:** `Prereq::Bio::Cell_Cycle_and_Mitosis`
- **Difficulty:** 4
- **Question:** Crossing over during prophase I increases genetic variation. Between which structures does this exchange occur?
- **A:** Between sister chromatids of the same chromosome
- **B:** Between unrelated chromosomes during anaphase II
- **C:** Between the centrioles and the spindle apparatus
- **D:** Between homologous chromosomes at chiasmata
- **Correct:** D
- **Explanation:** Crossing over swaps segments between non-sister chromatids of homologous chromosomes at chiasmata, producing recombinant chromosomes.
- **Tags:** `KC::Bio::Meiosis` `Prereq::Bio::Cell_Cycle_and_Mitosis` `MCAT::Bio_Biochem` `Difficulty::4`

## Bio::Mendelian_Genetics

### MCAT-BIO-MENG-001

- **KC:** `Bio::Mendelian_Genetics`
- **Prereqs:** `Prereq::Bio::Genetics` `Prereq::Bio::Meiosis`
- **Difficulty:** 3
- **Question:** Two organisms heterozygous for a single gene with complete dominance (Aa x Aa) are crossed. What fraction of offspring are expected to show the recessive phenotype?
- **A:** 1/4
- **B:** 1/2
- **C:** 3/4
- **D:** 0
- **Correct:** A
- **Explanation:** An Aa x Aa cross yields a 3-to-1 dominant-to-recessive ratio, so one quarter of offspring are homozygous recessive and show that phenotype.
- **Tags:** `KC::Bio::Mendelian_Genetics` `Prereq::Bio::Genetics` `Prereq::Bio::Meiosis` `MCAT::Bio_Biochem` `Difficulty::3`

### MCAT-BIO-MENG-002

- **KC:** `Bio::Mendelian_Genetics`
- **Prereqs:** `Prereq::Bio::Genetics` `Prereq::Bio::Meiosis`
- **Difficulty:** 5
- **Question:** In a dihybrid cross of two individuals heterozygous for two independently assorting genes (AaBb x AaBb), what fraction of offspring are homozygous recessive for both traits?
- **A:** 1/4
- **B:** 1/16
- **C:** 3/16
- **D:** 9/16
- **Correct:** B
- **Explanation:** Each gene independently gives 1/4 homozygous recessive, so both together occur at 1/4 times 1/4, which is 1/16.
- **Tags:** `KC::Bio::Mendelian_Genetics` `Prereq::Bio::Genetics` `Prereq::Bio::Meiosis` `MCAT::Bio_Biochem` `Difficulty::5`

## Bio::Eukaryotic_Cell

### MCAT-BIO-EUKC-001

- **KC:** `Bio::Eukaryotic_Cell`
- **Prereqs:** none
- **Difficulty:** 2
- **Question:** Which feature most clearly distinguishes a eukaryotic cell from a prokaryotic cell?
- **A:** The presence of a plasma membrane
- **B:** The ability to synthesize proteins
- **C:** The presence of a membrane-bound nucleus
- **D:** The presence of ribosomes
- **Correct:** C
- **Explanation:** Eukaryotic cells enclose their DNA within a membrane-bound nucleus, whereas prokaryotes lack membrane-bound organelles.
- **Tags:** `KC::Bio::Eukaryotic_Cell` `MCAT::Bio_Biochem` `Difficulty::2`

### MCAT-BIO-EUKC-002

- **KC:** `Bio::Eukaryotic_Cell`
- **Prereqs:** none
- **Difficulty:** 3
- **Question:** A cell secretes large amounts of a protein hormone. Which pair of organelles would you expect to be especially well developed?
- **A:** Lysosomes and peroxisomes
- **B:** Smooth endoplasmic reticulum and centrioles
- **C:** The nucleolus and cytoskeleton only
- **D:** Rough endoplasmic reticulum and Golgi apparatus
- **Correct:** D
- **Explanation:** Secreted proteins are made on the rough ER and then processed and packaged by the Golgi, so both are prominent in secretory cells.
- **Tags:** `KC::Bio::Eukaryotic_Cell` `MCAT::Bio_Biochem` `Difficulty::3`

## Bio::Prokaryotes_vs_Eukaryotes

### MCAT-BIO-PROE-001

- **KC:** `Bio::Prokaryotes_vs_Eukaryotes`
- **Prereqs:** `Prereq::Bio::Eukaryotic_Cell`
- **Difficulty:** 1
- **Question:** Where is the genetic material located in a typical prokaryotic cell?
- **A:** In the cytoplasm, in a region called the nucleoid
- **B:** Inside a membrane-bound nucleus
- **C:** Within the mitochondria
- **D:** Inside the endoplasmic reticulum
- **Correct:** A
- **Explanation:** Prokaryotes lack a nucleus, so their DNA is concentrated in a cytoplasmic region called the nucleoid.
- **Tags:** `KC::Bio::Prokaryotes_vs_Eukaryotes` `Prereq::Bio::Eukaryotic_Cell` `MCAT::Bio_Biochem` `Difficulty::1`

### MCAT-BIO-PROE-002

- **KC:** `Bio::Prokaryotes_vs_Eukaryotes`
- **Prereqs:** `Prereq::Bio::Eukaryotic_Cell`
- **Difficulty:** 2
- **Question:** Which structure is commonly present in prokaryotic cells but absent from typical animal cells?
- **A:** A plasma membrane
- **B:** A cell wall containing peptidoglycan
- **C:** Ribosomes
- **D:** Cytoplasm
- **Correct:** B
- **Explanation:** Many prokaryotes have a peptidoglycan cell wall, a feature not found in animal cells.
- **Tags:** `KC::Bio::Prokaryotes_vs_Eukaryotes` `Prereq::Bio::Eukaryotic_Cell` `MCAT::Bio_Biochem` `Difficulty::2`

## Bio::Cell_Membrane_and_Transport

### MCAT-BIO-CELMT-001

- **KC:** `Bio::Cell_Membrane_and_Transport`
- **Prereqs:** `Prereq::Bio::Eukaryotic_Cell` `Prereq::Biochem::Carbohydrates_and_Lipids`
- **Difficulty:** 2
- **Question:** Oxygen moves across the plasma membrane from higher to lower concentration without any energy input. This is an example of what?
- **A:** Active transport
- **B:** Endocytosis
- **C:** Simple diffusion
- **D:** Primary active pumping
- **Correct:** C
- **Explanation:** Small nonpolar molecules such as oxygen cross the lipid bilayer by simple diffusion down their concentration gradient, requiring no ATP.
- **Tags:** `KC::Bio::Cell_Membrane_and_Transport` `Prereq::Bio::Eukaryotic_Cell` `Prereq::Biochem::Carbohydrates_and_Lipids` `MCAT::Bio_Biochem` `Difficulty::2`

### MCAT-BIO-CELMT-002

- **KC:** `Bio::Cell_Membrane_and_Transport`
- **Prereqs:** `Prereq::Bio::Eukaryotic_Cell` `Prereq::Biochem::Carbohydrates_and_Lipids`
- **Difficulty:** 4
- **Question:** A transporter moves glucose into a cell against its gradient by simultaneously letting sodium flow down its gradient. This mechanism is best described as:
- **A:** Simple diffusion
- **B:** Facilitated diffusion of glucose alone
- **C:** Osmosis
- **D:** Secondary active transport
- **Correct:** D
- **Explanation:** Secondary active transport uses the energy stored in one ion's gradient (sodium) to drive the uphill movement of another solute (glucose).
- **Tags:** `KC::Bio::Cell_Membrane_and_Transport` `Prereq::Bio::Eukaryotic_Cell` `Prereq::Biochem::Carbohydrates_and_Lipids` `MCAT::Bio_Biochem` `Difficulty::4`

## Bio::Cell_Signaling

### MCAT-BIO-CELS-001

- **KC:** `Bio::Cell_Signaling`
- **Prereqs:** `Prereq::Bio::Cell_Membrane_and_Transport` `Prereq::Biochem::Protein_Structure_and_Function`
- **Difficulty:** 3
- **Question:** A hydrophilic peptide hormone cannot cross the plasma membrane. How does it typically trigger a response in its target cell?
- **A:** By binding a cell-surface receptor that activates intracellular signaling
- **B:** By diffusing to the nucleus and binding DNA directly
- **C:** By dissolving straight through the lipid bilayer
- **D:** By being pumped in through voltage-gated ion channels
- **Correct:** A
- **Explanation:** Peptide hormones bind membrane receptors, which relay the signal inside the cell through second messengers or signaling cascades.
- **Tags:** `KC::Bio::Cell_Signaling` `Prereq::Bio::Cell_Membrane_and_Transport` `Prereq::Biochem::Protein_Structure_and_Function` `MCAT::Bio_Biochem` `Difficulty::3`

### MCAT-BIO-CELS-002

- **KC:** `Bio::Cell_Signaling`
- **Prereqs:** `Prereq::Bio::Cell_Membrane_and_Transport` `Prereq::Biochem::Protein_Structure_and_Function`
- **Difficulty:** 5
- **Question:** A signaling pathway uses a cascade in which each activated kinase phosphorylates many downstream kinases. What is the main functional advantage of this arrangement?
- **A:** It permanently rewrites the cell's DNA
- **B:** It amplifies the signal so a few molecules produce a large response
- **C:** It prevents the signal from ever being turned off
- **D:** It removes the need for a receptor
- **Correct:** B
- **Explanation:** Because each enzyme activates many targets, a cascade greatly amplifies an initially small signal.
- **Tags:** `KC::Bio::Cell_Signaling` `Prereq::Bio::Cell_Membrane_and_Transport` `Prereq::Biochem::Protein_Structure_and_Function` `MCAT::Bio_Biochem` `Difficulty::5`

## Bio::Cytoskeleton

### MCAT-BIO-CYT-001

- **KC:** `Bio::Cytoskeleton`
- **Prereqs:** `Prereq::Bio::Eukaryotic_Cell`
- **Difficulty:** 2
- **Question:** Which cytoskeletal component forms the tracks along which motor proteins carry vesicles over long distances in the cell?
- **A:** The nuclear envelope
- **B:** The lipid bilayer
- **C:** Microtubules
- **D:** Ribosomes
- **Correct:** C
- **Explanation:** Microtubules serve as tracks along which motor proteins such as kinesin and dynein transport cargo through the cell.
- **Tags:** `KC::Bio::Cytoskeleton` `Prereq::Bio::Eukaryotic_Cell` `MCAT::Bio_Biochem` `Difficulty::2`

### MCAT-BIO-CYT-002

- **KC:** `Bio::Cytoskeleton`
- **Prereqs:** `Prereq::Bio::Eukaryotic_Cell`
- **Difficulty:** 3
- **Question:** A drug locks microtubules in place so they cannot depolymerize. Which cellular process is most directly disrupted?
- **A:** Passive diffusion of gases across the membrane
- **B:** Synthesis of membrane lipids
- **C:** Transcription of ribosomal RNA
- **D:** Chromosome separation during mitosis
- **Correct:** D
- **Explanation:** The mitotic spindle relies on dynamic microtubules that shorten to pull chromosomes apart, so blocking depolymerization halts proper chromosome separation.
- **Tags:** `KC::Bio::Cytoskeleton` `Prereq::Bio::Eukaryotic_Cell` `MCAT::Bio_Biochem` `Difficulty::3`

## Bio::Cell_Cycle_and_Mitosis

### MCAT-BIO-CELCM-001

- **KC:** `Bio::Cell_Cycle_and_Mitosis`
- **Prereqs:** `Prereq::Bio::DNA_Replication` `Prereq::Bio::Eukaryotic_Cell`
- **Difficulty:** 2
- **Question:** During which phase of the cell cycle is nuclear DNA replicated?
- **A:** S phase
- **B:** G1 phase
- **C:** M phase
- **D:** G0 phase
- **Correct:** A
- **Explanation:** DNA synthesis occurs in S (synthesis) phase, producing two identical sister chromatids per chromosome.
- **Tags:** `KC::Bio::Cell_Cycle_and_Mitosis` `Prereq::Bio::DNA_Replication` `Prereq::Bio::Eukaryotic_Cell` `MCAT::Bio_Biochem` `Difficulty::2`

### MCAT-BIO-CELCM-002

- **KC:** `Bio::Cell_Cycle_and_Mitosis`
- **Prereqs:** `Prereq::Bio::DNA_Replication` `Prereq::Bio::Eukaryotic_Cell`
- **Difficulty:** 4
- **Question:** A cell with damaged DNA fails to arrest at the G2/M checkpoint. What is the most likely harmful consequence?
- **A:** The cell will stop making all proteins
- **B:** The cell may divide and pass unrepaired mutations to daughter cells
- **C:** The cell will immediately become haploid
- **D:** The cell will skip DNA replication entirely
- **Correct:** B
- **Explanation:** Checkpoints normally halt division until damage is repaired, so bypassing them lets mutated DNA be transmitted to daughter cells, a step toward cancer.
- **Tags:** `KC::Bio::Cell_Cycle_and_Mitosis` `Prereq::Bio::DNA_Replication` `Prereq::Bio::Eukaryotic_Cell` `MCAT::Bio_Biochem` `Difficulty::4`

## Bio::Viruses

### MCAT-BIO-VIR-001

- **KC:** `Bio::Viruses`
- **Prereqs:** `Prereq::Bio::DNA`
- **Difficulty:** 2
- **Question:** Why are viruses considered obligate intracellular parasites?
- **A:** They contain their own ribosomes and mitochondria
- **B:** They divide independently by binary fission
- **C:** They can reproduce only by using a host cell's machinery
- **D:** They generate ATP through their own metabolism
- **Correct:** C
- **Explanation:** Viruses lack the machinery to reproduce on their own and must commandeer a host cell to make new virus particles.
- **Tags:** `KC::Bio::Viruses` `Prereq::Bio::DNA` `MCAT::Bio_Biochem` `Difficulty::2`

### MCAT-BIO-VIR-002

- **KC:** `Bio::Viruses`
- **Prereqs:** `Prereq::Bio::DNA`
- **Difficulty:** 4
- **Question:** A retrovirus carries RNA and the enzyme reverse transcriptase. What is the direct role of that enzyme in the viral life cycle?
- **A:** It translates the viral RNA into protein
- **B:** It replicates the viral RNA directly into more RNA
- **C:** It degrades the host cell's genome
- **D:** It synthesizes a DNA copy from the viral RNA template
- **Correct:** D
- **Explanation:** Reverse transcriptase makes a DNA copy of the viral RNA, which can then integrate into the host genome.
- **Tags:** `KC::Bio::Viruses` `Prereq::Bio::DNA` `MCAT::Bio_Biochem` `Difficulty::4`

## Bio::Bacteria

### MCAT-BIO-BAC-001

- **KC:** `Bio::Bacteria`
- **Prereqs:** `Prereq::Bio::Genetics` `Prereq::Bio::Prokaryotes_vs_Eukaryotes`
- **Difficulty:** 2
- **Question:** Bacteria typically reproduce asexually by which process?
- **A:** Binary fission
- **B:** Meiosis
- **C:** Mitosis with a spindle apparatus
- **D:** Budding from a nucleus
- **Correct:** A
- **Explanation:** Bacteria divide by binary fission, copying their single chromosome and splitting into two cells.
- **Tags:** `KC::Bio::Bacteria` `Prereq::Bio::Genetics` `Prereq::Bio::Prokaryotes_vs_Eukaryotes` `MCAT::Bio_Biochem` `Difficulty::2`

### MCAT-BIO-BAC-002

- **KC:** `Bio::Bacteria`
- **Prereqs:** `Prereq::Bio::Genetics` `Prereq::Bio::Prokaryotes_vs_Eukaryotes`
- **Difficulty:** 4
- **Question:** Two bacteria come into contact and one transfers a plasmid carrying an antibiotic-resistance gene to the other. This transfer of genetic material is called:
- **A:** Transcription
- **B:** Conjugation
- **C:** Binary fission
- **D:** Translation
- **Correct:** B
- **Explanation:** Conjugation transfers DNA, often a plasmid, from one bacterium to another through direct contact, spreading traits such as resistance.
- **Tags:** `KC::Bio::Bacteria` `Prereq::Bio::Genetics` `Prereq::Bio::Prokaryotes_vs_Eukaryotes` `MCAT::Bio_Biochem` `Difficulty::4`

## Bio::Fungi

### MCAT-BIO-FUN-001

- **KC:** `Bio::Fungi`
- **Prereqs:** `Prereq::Bio::Eukaryotic_Cell`
- **Difficulty:** 2
- **Question:** Fungi obtain nutrients primarily by which mode of nutrition?
- **A:** Photosynthesis using chloroplasts
- **B:** Ingestion of large food particles
- **C:** Absorption of nutrients after external digestion
- **D:** Chemosynthesis from inorganic minerals
- **Correct:** C
- **Explanation:** Fungi secrete enzymes that digest material externally and then absorb the resulting small molecules.
- **Tags:** `KC::Bio::Fungi` `Prereq::Bio::Eukaryotic_Cell` `MCAT::Bio_Biochem` `Difficulty::2`

### MCAT-BIO-FUN-002

- **KC:** `Bio::Fungi`
- **Prereqs:** `Prereq::Bio::Eukaryotic_Cell`
- **Difficulty:** 3
- **Question:** The cell walls of fungi are composed mainly of which structural polysaccharide?
- **A:** Cellulose
- **B:** Peptidoglycan
- **C:** Glycogen
- **D:** Chitin
- **Correct:** D
- **Explanation:** Fungal cell walls are built largely of chitin, distinguishing them from the cellulose walls of plants and the peptidoglycan walls of bacteria.
- **Tags:** `KC::Bio::Fungi` `Prereq::Bio::Eukaryotic_Cell` `MCAT::Bio_Biochem` `Difficulty::3`

## Bio::Nervous_System

### MCAT-BIO-NERS-001

- **KC:** `Bio::Nervous_System`
- **Prereqs:** `Prereq::Bio::Cell_Membrane_and_Transport` `Prereq::GenChem::Ions_in_Solutions` `Prereq::Physics::Electrical_Circuits`
- **Difficulty:** 3
- **Question:** The inside of a resting neuron is negative relative to the outside. Which factor is most responsible for maintaining this gradient?
- **A:** The sodium-potassium pump together with selective ion permeability
- **B:** An equal distribution of all ions across the membrane
- **C:** The complete absence of membrane transport proteins
- **D:** Free diffusion of large proteins out of the cell
- **Correct:** A
- **Explanation:** The sodium-potassium pump plus the membrane's higher resting permeability to potassium establish and maintain the negative resting potential.
- **Tags:** `KC::Bio::Nervous_System` `Prereq::Bio::Cell_Membrane_and_Transport` `Prereq::GenChem::Ions_in_Solutions` `Prereq::Physics::Electrical_Circuits` `MCAT::Bio_Biochem` `Difficulty::3`

### MCAT-BIO-NERS-002

- **KC:** `Bio::Nervous_System`
- **Prereqs:** `Prereq::Bio::Cell_Membrane_and_Transport` `Prereq::GenChem::Ions_in_Solutions` `Prereq::Physics::Electrical_Circuits`
- **Difficulty:** 5
- **Question:** During an action potential, the rapid rising phase (depolarization) is caused primarily by:
- **A:** Potassium ions leaving the cell
- **B:** Sodium ions entering through voltage-gated channels
- **C:** Chloride ions entering the cell
- **D:** Active transport of calcium out of the axon
- **Correct:** B
- **Explanation:** At threshold, voltage-gated sodium channels open and the resulting sodium influx rapidly depolarizes the membrane.
- **Tags:** `KC::Bio::Nervous_System` `Prereq::Bio::Cell_Membrane_and_Transport` `Prereq::GenChem::Ions_in_Solutions` `Prereq::Physics::Electrical_Circuits` `MCAT::Bio_Biochem` `Difficulty::5`

## Bio::Endocrine_System

### MCAT-BIO-ENDS-001

- **KC:** `Bio::Endocrine_System`
- **Prereqs:** `Prereq::Bio::Cell_Signaling`
- **Difficulty:** 3
- **Question:** Compared with peptide hormones, steroid hormones typically:
- **A:** Cannot enter target cells at all
- **B:** Act only through cell-surface receptors
- **C:** Diffuse through the membrane and bind intracellular receptors
- **D:** Are stored in large secretory vesicles before release
- **Correct:** C
- **Explanation:** Being lipid-soluble, steroid hormones cross the plasma membrane and bind intracellular receptors that often regulate gene transcription.
- **Tags:** `KC::Bio::Endocrine_System` `Prereq::Bio::Cell_Signaling` `MCAT::Bio_Biochem` `Difficulty::3`

### MCAT-BIO-ENDS-002

- **KC:** `Bio::Endocrine_System`
- **Prereqs:** `Prereq::Bio::Cell_Signaling`
- **Difficulty:** 5
- **Question:** A rising blood level of a hormone inhibits the glands that stimulated its release. This regulatory arrangement is best described as:
- **A:** Positive feedback
- **B:** Feed-forward amplification
- **C:** Independent assortment
- **D:** Negative feedback
- **Correct:** D
- **Explanation:** Negative feedback occurs when a hormone suppresses its own upstream signals, keeping hormone levels near a set point.
- **Tags:** `KC::Bio::Endocrine_System` `Prereq::Bio::Cell_Signaling` `MCAT::Bio_Biochem` `Difficulty::5`

## Bio::Muscular_System

### MCAT-BIO-MUSS-001

- **KC:** `Bio::Muscular_System`
- **Prereqs:** `Prereq::Bio::Eukaryotic_Cell` `Prereq::Biochem::Bioenergetics`
- **Difficulty:** 2
- **Question:** According to the sliding-filament model, muscle contraction occurs when:
- **A:** Thin filaments slide past thick filaments, shortening the sarcomere
- **B:** Actin and myosin filaments each shorten individually
- **C:** The sarcomere length stays constant throughout contraction
- **D:** Myosin dissolves into the surrounding cytoplasm
- **Correct:** A
- **Explanation:** Contraction results from thin (actin) filaments sliding over thick (myosin) filaments, shortening each sarcomere while the filaments keep their own length.
- **Tags:** `KC::Bio::Muscular_System` `Prereq::Bio::Eukaryotic_Cell` `Prereq::Biochem::Bioenergetics` `MCAT::Bio_Biochem` `Difficulty::2`

### MCAT-BIO-MUSS-002

- **KC:** `Bio::Muscular_System`
- **Prereqs:** `Prereq::Bio::Eukaryotic_Cell` `Prereq::Biochem::Bioenergetics`
- **Difficulty:** 4
- **Question:** After a muscle fully depletes its ATP, cross-bridges cannot detach. Which step of the cross-bridge cycle requires ATP binding?
- **A:** Formation of the initial actin-myosin cross-bridge
- **B:** Release of the myosin head from actin
- **C:** Depolarization of the muscle cell membrane
- **D:** Release of calcium from the sarcoplasmic reticulum
- **Correct:** B
- **Explanation:** ATP binding to myosin is required for the head to release actin, so without ATP the cross-bridges stay attached, as in rigor.
- **Tags:** `KC::Bio::Muscular_System` `Prereq::Bio::Eukaryotic_Cell` `Prereq::Biochem::Bioenergetics` `MCAT::Bio_Biochem` `Difficulty::4`

## Bio::Skeletal_System

### MCAT-BIO-SKES-001

- **KC:** `Bio::Skeletal_System`
- **Prereqs:** `Prereq::Bio::Eukaryotic_Cell` `Prereq::GenChem::Ions_in_Solutions`
- **Difficulty:** 2
- **Question:** Besides providing structural support, bone serves as the body's major reservoir for which mineral ion?
- **A:** Iodine
- **B:** Iron
- **C:** Calcium
- **D:** Zinc
- **Correct:** C
- **Explanation:** Bone stores most of the body's calcium and exchanges it with the blood to help regulate calcium levels.
- **Tags:** `KC::Bio::Skeletal_System` `Prereq::Bio::Eukaryotic_Cell` `Prereq::GenChem::Ions_in_Solutions` `MCAT::Bio_Biochem` `Difficulty::2`

### MCAT-BIO-SKES-002

- **KC:** `Bio::Skeletal_System`
- **Prereqs:** `Prereq::Bio::Eukaryotic_Cell` `Prereq::GenChem::Ions_in_Solutions`
- **Difficulty:** 3
- **Question:** When blood calcium falls, osteoclast activity increases. What is the direct effect of osteoclasts on bone?
- **A:** They deposit new bone matrix
- **B:** They convert cartilage into muscle
- **C:** They lock calcium permanently inside the bone
- **D:** They resorb bone, releasing calcium into the blood
- **Correct:** D
- **Explanation:** Osteoclasts break down bone matrix, releasing calcium into the bloodstream to help restore normal blood calcium.
- **Tags:** `KC::Bio::Skeletal_System` `Prereq::Bio::Eukaryotic_Cell` `Prereq::GenChem::Ions_in_Solutions` `MCAT::Bio_Biochem` `Difficulty::3`

## Bio::Circulatory_System

### MCAT-BIO-CIRS-001

- **KC:** `Bio::Circulatory_System`
- **Prereqs:** `Prereq::Bio::Eukaryotic_Cell` `Prereq::Physics::Fluid_Dynamics`
- **Difficulty:** 2
- **Question:** In the human heart, which vessel carries oxygen-poor blood from the right ventricle toward the lungs?
- **A:** The pulmonary artery
- **B:** The aorta
- **C:** The pulmonary vein
- **D:** The coronary artery
- **Correct:** A
- **Explanation:** The right ventricle pumps deoxygenated blood into the pulmonary artery, which delivers it to the lungs for gas exchange.
- **Tags:** `KC::Bio::Circulatory_System` `Prereq::Bio::Eukaryotic_Cell` `Prereq::Physics::Fluid_Dynamics` `MCAT::Bio_Biochem` `Difficulty::2`

### MCAT-BIO-CIRS-002

- **KC:** `Bio::Circulatory_System`
- **Prereqs:** `Prereq::Bio::Eukaryotic_Cell` `Prereq::Physics::Fluid_Dynamics`
- **Difficulty:** 4
- **Question:** Capillaries are the primary site of exchange between blood and tissues. Which structural feature most directly supports this function?
- **A:** Thick, muscular vessel walls
- **B:** Walls one cell layer thick with a large total surface area
- **C:** Numerous one-way valves
- **D:** Strong high-pressure elastic recoil
- **Correct:** B
- **Explanation:** Capillaries have very thin walls and an enormous combined surface area, allowing efficient diffusion of gases and nutrients.
- **Tags:** `KC::Bio::Circulatory_System` `Prereq::Bio::Eukaryotic_Cell` `Prereq::Physics::Fluid_Dynamics` `MCAT::Bio_Biochem` `Difficulty::4`

## Bio::Respiratory_System

### MCAT-BIO-RESS-001

- **KC:** `Bio::Respiratory_System`
- **Prereqs:** `Prereq::Bio::Eukaryotic_Cell` `Prereq::GenChem::Gas_Phase`
- **Difficulty:** 2
- **Question:** In the alveoli, oxygen moves from the air spaces into the blood. What process drives this movement?
- **A:** Active transport requiring ATP
- **B:** Bulk flow through open protein channels
- **C:** Diffusion down the oxygen partial-pressure gradient
- **D:** Osmosis of water carrying dissolved oxygen
- **Correct:** C
- **Explanation:** Oxygen diffuses from its higher partial pressure in the alveoli to its lower partial pressure in the blood, requiring no energy input.
- **Tags:** `KC::Bio::Respiratory_System` `Prereq::Bio::Eukaryotic_Cell` `Prereq::GenChem::Gas_Phase` `MCAT::Bio_Biochem` `Difficulty::2`

### MCAT-BIO-RESS-002

- **KC:** `Bio::Respiratory_System`
- **Prereqs:** `Prereq::Bio::Eukaryotic_Cell` `Prereq::GenChem::Gas_Phase`
- **Difficulty:** 4
- **Question:** A rise in blood carbon dioxide lowers blood pH and increases the drive to breathe. Which reaction best explains the increased acidity?
- **A:** Carbon dioxide removes hydroxide ions directly from plasma proteins
- **B:** Carbon dioxide binds oxygen to form a strong acid
- **C:** Carbon dioxide is converted into lactic acid within the lungs
- **D:** Carbon dioxide reacts with water to form carbonic acid, releasing hydrogen ions
- **Correct:** D
- **Explanation:** Carbon dioxide combines with water to form carbonic acid, which dissociates into bicarbonate and hydrogen ions, lowering pH and stimulating ventilation.
- **Tags:** `KC::Bio::Respiratory_System` `Prereq::Bio::Eukaryotic_Cell` `Prereq::GenChem::Gas_Phase` `MCAT::Bio_Biochem` `Difficulty::4`

## Bio::Digestive_System

### MCAT-BIO-DIGS-001

- **KC:** `Bio::Digestive_System`
- **Prereqs:** `Prereq::Bio::Eukaryotic_Cell` `Prereq::Biochem::Enzymes`
- **Difficulty:** 2
- **Question:** Which organ is the primary site for absorption of digested nutrients into the bloodstream?
- **A:** The small intestine
- **B:** The stomach
- **C:** The esophagus
- **D:** The large intestine
- **Correct:** A
- **Explanation:** The small intestine, with its villi and microvilli, provides a large surface area that is the main site of nutrient absorption.
- **Tags:** `KC::Bio::Digestive_System` `Prereq::Bio::Eukaryotic_Cell` `Prereq::Biochem::Enzymes` `MCAT::Bio_Biochem` `Difficulty::2`

### MCAT-BIO-DIGS-002

- **KC:** `Bio::Digestive_System`
- **Prereqs:** `Prereq::Bio::Eukaryotic_Cell` `Prereq::Biochem::Enzymes`
- **Difficulty:** 4
- **Question:** Pancreatic proteases are secreted as inactive zymogens and activated only in the intestine. What is the main benefit of this arrangement?
- **A:** It lets the enzymes work at very low temperatures
- **B:** It prevents the enzymes from digesting the pancreas that makes them
- **C:** It raises the pH of the stomach contents
- **D:** It allows the enzymes to function without any substrate
- **Correct:** B
- **Explanation:** Secreting inactive precursors keeps the enzymes from digesting the tissue that produces them; they are switched on only after reaching the intestinal lumen.
- **Tags:** `KC::Bio::Digestive_System` `Prereq::Bio::Eukaryotic_Cell` `Prereq::Biochem::Enzymes` `MCAT::Bio_Biochem` `Difficulty::4`

## Bio::Immune_System

### MCAT-BIO-IMMS-001

- **KC:** `Bio::Immune_System`
- **Prereqs:** `Prereq::Bio::Eukaryotic_Cell` `Prereq::Biochem::Protein_Structure_and_Function`
- **Difficulty:** 3
- **Question:** Which cells differentiate to produce antibodies during the adaptive immune response?
- **A:** Red blood cells
- **B:** Platelets
- **C:** B lymphocytes (plasma cells)
- **D:** Bone-forming osteocytes
- **Correct:** C
- **Explanation:** Activated B lymphocytes differentiate into plasma cells that secrete antibodies specific to an antigen.
- **Tags:** `KC::Bio::Immune_System` `Prereq::Bio::Eukaryotic_Cell` `Prereq::Biochem::Protein_Structure_and_Function` `MCAT::Bio_Biochem` `Difficulty::3`

### MCAT-BIO-IMMS-002

- **KC:** `Bio::Immune_System`
- **Prereqs:** `Prereq::Bio::Eukaryotic_Cell` `Prereq::Biochem::Protein_Structure_and_Function`
- **Difficulty:** 5
- **Question:** On second exposure to a pathogen, the body mounts a faster and stronger antibody response than the first time. This improvement is due primarily to:
- **A:** Innate barriers such as the skin
- **B:** An increased red blood cell count
- **C:** A permanent change to the pathogen's DNA
- **D:** Memory lymphocytes generated during the first exposure
- **Correct:** D
- **Explanation:** The first exposure produces long-lived memory cells that respond rapidly upon re-exposure, giving the faster, stronger secondary response.
- **Tags:** `KC::Bio::Immune_System` `Prereq::Bio::Eukaryotic_Cell` `Prereq::Biochem::Protein_Structure_and_Function` `MCAT::Bio_Biochem` `Difficulty::5`

## Bio::Lymphatic_System

### MCAT-BIO-LYMS-001

- **KC:** `Bio::Lymphatic_System`
- **Prereqs:** `Prereq::Bio::Circulatory_System` `Prereq::Bio::Immune_System`
- **Difficulty:** 2
- **Question:** One major function of the lymphatic system is to:
- **A:** Return excess interstitial fluid to the bloodstream
- **B:** Pump oxygenated blood to the tissues
- **C:** Produce digestive enzymes for the gut
- **D:** Generate action potentials in neurons
- **Correct:** A
- **Explanation:** The lymphatic system collects fluid that leaks into tissues and returns it to the circulation, maintaining fluid balance.
- **Tags:** `KC::Bio::Lymphatic_System` `Prereq::Bio::Circulatory_System` `Prereq::Bio::Immune_System` `MCAT::Bio_Biochem` `Difficulty::2`

### MCAT-BIO-LYMS-002

- **KC:** `Bio::Lymphatic_System`
- **Prereqs:** `Prereq::Bio::Circulatory_System` `Prereq::Bio::Immune_System`
- **Difficulty:** 3
- **Question:** Lymph nodes contribute to immunity mainly by:
- **A:** Storing red blood cells for emergencies
- **B:** Filtering lymph and exposing trapped pathogens to immune cells
- **C:** Producing bile to aid digestion
- **D:** Secreting hormones into the blood
- **Correct:** B
- **Explanation:** Lymph nodes filter lymph and provide a site where immune cells encounter and respond to trapped pathogens.
- **Tags:** `KC::Bio::Lymphatic_System` `Prereq::Bio::Circulatory_System` `Prereq::Bio::Immune_System` `MCAT::Bio_Biochem` `Difficulty::3`

## Bio::Skin_System

### MCAT-BIO-SKIS-001

- **KC:** `Bio::Skin_System`
- **Prereqs:** `Prereq::Bio::Eukaryotic_Cell`
- **Difficulty:** 1
- **Question:** Which is a primary protective function of the skin?
- **A:** Producing insulin for glucose control
- **B:** Filtering waste from the blood
- **C:** Acting as a barrier against pathogens and water loss
- **D:** Generating new red blood cells
- **Correct:** C
- **Explanation:** The skin forms a physical barrier that limits pathogen entry and reduces water loss, among its other roles.
- **Tags:** `KC::Bio::Skin_System` `Prereq::Bio::Eukaryotic_Cell` `MCAT::Bio_Biochem` `Difficulty::1`

### MCAT-BIO-SKIS-002

- **KC:** `Bio::Skin_System`
- **Prereqs:** `Prereq::Bio::Eukaryotic_Cell`
- **Difficulty:** 3
- **Question:** When body temperature rises, blood vessels in the skin dilate. How does this help cool the body?
- **A:** It reduces blood flow to the surface, trapping heat inside
- **B:** It generates additional heat through shivering
- **C:** It completely stops sweat production
- **D:** It brings warm blood near the surface, increasing heat loss to the environment
- **Correct:** D
- **Explanation:** Vasodilation moves more warm blood near the skin surface, increasing heat transfer to the environment and lowering body temperature.
- **Tags:** `KC::Bio::Skin_System` `Prereq::Bio::Eukaryotic_Cell` `MCAT::Bio_Biochem` `Difficulty::3`

## Bio::Excretory_System

### MCAT-BIO-EXCS-001

- **KC:** `Bio::Excretory_System`
- **Prereqs:** `Prereq::Bio::Circulatory_System` `Prereq::GenChem::Acid_Base_Equilibria` `Prereq::GenChem::Ions_in_Solutions`
- **Difficulty:** 3
- **Question:** In the nephron, which step of urine formation is performed by the glomerulus?
- **A:** Filtration of blood plasma into the tubule
- **B:** Reabsorption of glucose into the blood
- **C:** Secretion of hydrogen ions into the filtrate
- **D:** Final concentration of urine in the collecting duct
- **Correct:** A
- **Explanation:** The glomerulus filters plasma under pressure, forming the filtrate that the rest of the nephron then modifies.
- **Tags:** `KC::Bio::Excretory_System` `Prereq::Bio::Circulatory_System` `Prereq::GenChem::Acid_Base_Equilibria` `Prereq::GenChem::Ions_in_Solutions` `MCAT::Bio_Biochem` `Difficulty::3`

### MCAT-BIO-EXCS-002

- **KC:** `Bio::Excretory_System`
- **Prereqs:** `Prereq::Bio::Circulatory_System` `Prereq::GenChem::Acid_Base_Equilibria` `Prereq::GenChem::Ions_in_Solutions`
- **Difficulty:** 5
- **Question:** Antidiuretic hormone (ADH) increases the water permeability of the collecting duct. What is the direct effect on the urine produced?
- **A:** A larger volume of more dilute urine
- **B:** A smaller volume of more concentrated urine
- **C:** Increased glucose excretion in the urine
- **D:** Complete loss of sodium reabsorption
- **Correct:** B
- **Explanation:** ADH promotes water reabsorption from the collecting duct back into the blood, producing a smaller volume of more concentrated urine.
- **Tags:** `KC::Bio::Excretory_System` `Prereq::Bio::Circulatory_System` `Prereq::GenChem::Acid_Base_Equilibria` `Prereq::GenChem::Ions_in_Solutions` `MCAT::Bio_Biochem` `Difficulty::5`

## Bio::Reproductive_System

### MCAT-BIO-REPS-001

- **KC:** `Bio::Reproductive_System`
- **Prereqs:** `Prereq::Bio::Endocrine_System` `Prereq::Bio::Meiosis`
- **Difficulty:** 2
- **Question:** In human males, which process produces haploid sperm cells?
- **A:** Mitosis of somatic cells
- **B:** Binary fission
- **C:** Spermatogenesis by meiosis
- **D:** Budding from precursor cells
- **Correct:** C
- **Explanation:** Spermatogenesis uses meiosis to produce haploid sperm from diploid precursor cells.
- **Tags:** `KC::Bio::Reproductive_System` `Prereq::Bio::Endocrine_System` `Prereq::Bio::Meiosis` `MCAT::Bio_Biochem` `Difficulty::2`

### MCAT-BIO-REPS-002

- **KC:** `Bio::Reproductive_System`
- **Prereqs:** `Prereq::Bio::Endocrine_System` `Prereq::Bio::Meiosis`
- **Difficulty:** 4
- **Question:** A mid-cycle surge in luteinizing hormone (LH) in females most directly triggers which event?
- **A:** Menstruation
- **B:** Fertilization of the egg
- **C:** Implantation in the uterus
- **D:** Ovulation
- **Correct:** D
- **Explanation:** The mid-cycle LH surge triggers ovulation, the release of a mature egg from the ovarian follicle.
- **Tags:** `KC::Bio::Reproductive_System` `Prereq::Bio::Endocrine_System` `Prereq::Bio::Meiosis` `MCAT::Bio_Biochem` `Difficulty::4`

## Bio::Embryology

### MCAT-BIO-EMB-001

- **KC:** `Bio::Embryology`
- **Prereqs:** `Prereq::Bio::Gene_Expression_Regulation` `Prereq::Bio::Reproductive_System`
- **Difficulty:** 3
- **Question:** During early embryonic development, gastrulation is the process that:
- **A:** Forms the three primary germ layers
- **B:** Completes the formation of all organs
- **C:** Produces identical daughter cells with no rearrangement
- **D:** Halts all cell division in the embryo
- **Correct:** A
- **Explanation:** Gastrulation reorganizes the embryo into three germ layers (ectoderm, mesoderm, endoderm) that later give rise to different tissues.
- **Tags:** `KC::Bio::Embryology` `Prereq::Bio::Gene_Expression_Regulation` `Prereq::Bio::Reproductive_System` `MCAT::Bio_Biochem` `Difficulty::3`

### MCAT-BIO-EMB-002

- **KC:** `Bio::Embryology`
- **Prereqs:** `Prereq::Bio::Gene_Expression_Regulation` `Prereq::Bio::Reproductive_System`
- **Difficulty:** 5
- **Question:** Cells in an early embryo carry identical DNA yet develop into different tissues. This differentiation depends most directly on:
- **A:** Loss of unneeded genes from each cell
- **B:** Differential gene expression guided by regulatory signals
- **C:** Random mutations unique to each cell
- **D:** Changes in the genetic code itself
- **Correct:** B
- **Explanation:** Differentiation arises because cells express different subsets of the same genome in response to positional and regulatory signals.
- **Tags:** `KC::Bio::Embryology` `Prereq::Bio::Gene_Expression_Regulation` `Prereq::Bio::Reproductive_System` `MCAT::Bio_Biochem` `Difficulty::5`

## Bio::Evolution

### MCAT-BIO-EVO-001

- **KC:** `Bio::Evolution`
- **Prereqs:** `Prereq::Bio::Genetics`
- **Difficulty:** 2
- **Question:** Natural selection acts most directly on which of the following?
- **A:** The DNA sequence of a single gene in isolation
- **B:** Traits acquired only during an individual's lifetime
- **C:** Heritable phenotypic variation that affects survival and reproduction
- **D:** The total chromosome number characteristic of a species
- **Correct:** C
- **Explanation:** Natural selection favors heritable traits that improve survival and reproduction, shifting allele frequencies over generations.
- **Tags:** `KC::Bio::Evolution` `Prereq::Bio::Genetics` `MCAT::Bio_Biochem` `Difficulty::2`

### MCAT-BIO-EVO-002

- **KC:** `Bio::Evolution`
- **Prereqs:** `Prereq::Bio::Genetics`
- **Difficulty:** 4
- **Question:** After an insect population is treated with an insecticide for several generations, resistance becomes common. Which explanation is consistent with natural selection?
- **A:** The insecticide caused each insect to develop resistance during its life
- **B:** The insects consciously chose to become resistant
- **C:** The population's mutation rate was unaffected by anything at all
- **D:** Resistant variants already present survived and reproduced more
- **Correct:** D
- **Explanation:** Selection acts on pre-existing variation: resistant individuals survive treatment and pass resistance alleles to offspring, so their frequency rises.
- **Tags:** `KC::Bio::Evolution` `Prereq::Bio::Genetics` `MCAT::Bio_Biochem` `Difficulty::4`

## Bio::Population_Genetics

### MCAT-BIO-POPG-001

- **KC:** `Bio::Population_Genetics`
- **Prereqs:** `Prereq::Bio::Evolution` `Prereq::Bio::Mendelian_Genetics`
- **Difficulty:** 3
- **Question:** For a population to stay in Hardy-Weinberg equilibrium, which set of conditions must hold?
- **A:** No mutation, no migration, random mating, a large population, and no selection
- **B:** Frequent, strong natural selection each generation
- **C:** A very small breeding population
- **D:** Strong nonrandom (assortative) mating
- **Correct:** A
- **Explanation:** Hardy-Weinberg equilibrium requires no mutation, no gene flow, random mating, a large population, and no selection.
- **Tags:** `KC::Bio::Population_Genetics` `Prereq::Bio::Evolution` `Prereq::Bio::Mendelian_Genetics` `MCAT::Bio_Biochem` `Difficulty::3`

### MCAT-BIO-POPG-002

- **KC:** `Bio::Population_Genetics`
- **Prereqs:** `Prereq::Bio::Evolution` `Prereq::Bio::Mendelian_Genetics`
- **Difficulty:** 5
- **Question:** In a Hardy-Weinberg population, the recessive phenotype has a frequency of 0.04. What is the frequency of the dominant allele?
- **A:** 0.2
- **B:** 0.8
- **C:** 0.4
- **D:** 0.96
- **Correct:** B
- **Explanation:** The recessive phenotype frequency equals q squared, so q equals 0.2 and the dominant allele frequency p equals 1 minus 0.2, which is 0.8.
- **Tags:** `KC::Bio::Population_Genetics` `Prereq::Bio::Evolution` `Prereq::Bio::Mendelian_Genetics` `MCAT::Bio_Biochem` `Difficulty::5`

## Bio::Biodiversity_and_Phylogeny

### MCAT-BIO-BIOP-001

- **KC:** `Bio::Biodiversity_and_Phylogeny`
- **Prereqs:** `Prereq::Bio::Evolution`
- **Difficulty:** 2
- **Question:** On a phylogenetic tree, two species joined by the most recent common ancestor are best described as:
- **A:** The most distantly related species on the tree
- **B:** Completely unrelated species
- **C:** The most closely related species shown
- **D:** Members of different kingdoms by definition
- **Correct:** C
- **Explanation:** Sharing a more recent common ancestor indicates a closer evolutionary relationship.
- **Tags:** `KC::Bio::Biodiversity_and_Phylogeny` `Prereq::Bio::Evolution` `MCAT::Bio_Biochem` `Difficulty::2`

### MCAT-BIO-BIOP-002

- **KC:** `Bio::Biodiversity_and_Phylogeny`
- **Prereqs:** `Prereq::Bio::Evolution`
- **Difficulty:** 3
- **Question:** A trait shared by two species because both inherited it from a common ancestor is called:
- **A:** An analogous structure
- **B:** A vestigial adaptation
- **C:** A convergent trait
- **D:** A homologous structure
- **Correct:** D
- **Explanation:** Homologous structures derive from a shared ancestor, whereas analogous structures arise independently through convergent evolution.
- **Tags:** `KC::Bio::Biodiversity_and_Phylogeny` `Prereq::Bio::Evolution` `MCAT::Bio_Biochem` `Difficulty::3`
