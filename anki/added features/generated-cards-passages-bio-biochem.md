# MCAT Generated Cards — Bio/Biochem Passages

Importable multiple-choice cards **converted from the reading passages in
`added features/passages-bio-biochem.md`** so those (previously orphaned)
passages become scored content in the Bio/Biochem section. Every card keeps its
original passage **Source** citation, correct answer, and explanation; no new
science is invented.

Block format matches the strict, line-based importer used by the other
`generated-cards-*.md` files (one field per line, `## <KC id>` group headers,
byte-exact `KC::` tags from `added features/kc-map-unified.md` §6). Each Question
line carries a trimmed (~120-word) version of the passage plus the question stem.

- **Source file:** `added features/passages-bio-biochem.md` (3 passages, 16 questions).
- **Total cards:** 16 (`MCAT-PASSAGE-BB-001` … `MCAT-PASSAGE-BB-016`).
- **KC ids used (all real, from §6):** `Bio::Biotechnology`, `Biochem::Nucleotides_and_Nucleic_Acids`, `Bio::Evolution`, `Bio::Translation`, `Bio::Circulatory_System`, `Bio::Muscular_System`.
- Primary section is `MCAT::Bio_Biochem` for every card. Passage IRT/reasoning
  metadata (`IRT::Discrimination`, `IRT::Guessing`, `Reasoning::…`) is attached
  per the passage-scoring convention.

---

## Bio::Biotechnology

### MCAT-PASSAGE-BB-001

- **KC:** `Bio::Biotechnology`
- **Prereqs:** none
- **Difficulty:** 3
- **Question:** Passage (Source: Magnusson JP et al. (2021), "Enhanced Cas12a multi-gene regulation using a CRISPR array separator," eLife 10:e66406, CC BY 4.0, https://doi.org/10.7554/eLife.66406): Cas12a can process a single CRISPR array transcript into multiple guide RNAs (gRNAs), each a repeat (recognized by the enzyme) plus a spacer (targeting DNA). To study multi-gRNA arrays, researchers placed a "dummy" gRNA upstream of a gRNA targeting a GFP reporter, using nuclease-dead Cas12a fused to a transcriptional activator so GFP reported downstream gRNA activity. Activity varied dramatically; a single-nucleotide change in the upstream spacer could nearly abolish downstream activation. High-GC spacers were predicted to form stable RNA secondary structure that impaired processing near the cleavage site. Short AT-rich synthetic separators (AAAT) placed upstream of a gRNA rescued high-GC arrays, whereas a full-length natural separator was not fully excised and abolished activity; the optimized design activated seven genes at once. — Question: Why did the researchers use a nuclease-dead Cas12a fused to a transcriptional activator together with a GFP reporter?
- **A:** To cut the GFP gene out of the genome and measure repair efficiency
- **B:** So that GFP fluorescence would report how effectively the downstream gRNA directed gene activation
- **C:** To prevent the CRISPR array from being transcribed
- **D:** To ensure the dummy spacer could bind DNA directly without a gRNA
- **Correct:** B
- **Explanation:** A catalytically dead Cas12a fused to an activator turns a target gene on instead of cutting it, so GFP output serves as a readout of downstream gRNA activity. (A) is wrong—the enzyme is nuclease-dead, so it does not cut. (C) is wrong; the array must be transcribed and processed for the system to work. (D) misstates the mechanism—spacers guide the protein via the gRNA, not on their own.
- **Tags:** `KC::Bio::Biotechnology` `MCAT::Bio_Biochem` `Difficulty::3` `IRT::Discrimination::0.9` `IRT::Guessing::0.25` `Reasoning::Comprehension`

### MCAT-PASSAGE-BB-002

- **KC:** `Bio::Biotechnology`
- **Prereqs:** none
- **Difficulty:** 3
- **Question:** Passage (Source: Magnusson JP et al. (2021), "Enhanced Cas12a multi-gene regulation using a CRISPR array separator," eLife 10:e66406, CC BY 4.0, https://doi.org/10.7554/eLife.66406): Cas12a can process a single CRISPR array transcript into multiple guide RNAs (gRNAs), each a repeat (recognized by the enzyme) plus a spacer (targeting DNA). To study multi-gRNA arrays, researchers placed a "dummy" gRNA upstream of a gRNA targeting a GFP reporter, using nuclease-dead Cas12a fused to a transcriptional activator so GFP reported downstream gRNA activity. Activity varied dramatically; a single-nucleotide change in the upstream spacer could nearly abolish downstream activation. High-GC spacers were predicted to form stable RNA secondary structure that impaired processing near the cleavage site. Short AT-rich synthetic separators (AAAT) placed upstream of a gRNA rescued high-GC arrays, whereas a full-length natural separator was not fully excised and abolished activity; the optimized design activated seven genes at once. — Question: A single-nucleotide change in the upstream dummy spacer sometimes nearly abolished activation of the downstream reporter. This best supports the conclusion that:
- **A:** the two gRNAs are transcribed from independent promoters
- **B:** the sequence of one spacer can influence the performance of a neighboring gRNA in the same array
- **C:** GFP fluorescence is unrelated to gRNA activity
- **D:** Cas12a cannot process arrays containing more than one gRNA
- **Correct:** B
- **Explanation:** If an upstream change alters downstream output, the spacers are not acting independently—one gRNA's sequence affects its neighbor. (A) contradicts the single-array design (one transcript). (C) contradicts the reporter logic established in the passage. (D) is false; Cas12a does process multi-gRNA arrays, just unevenly.
- **Tags:** `KC::Bio::Biotechnology` `MCAT::Bio_Biochem` `Difficulty::3` `IRT::Discrimination::1.0` `IRT::Guessing::0.25` `Reasoning::Inference`

### MCAT-PASSAGE-BB-003

- **KC:** `Bio::Biotechnology`
- **Prereqs:** none
- **Difficulty:** 4
- **Question:** Passage (Source: Magnusson JP et al. (2021), "Enhanced Cas12a multi-gene regulation using a CRISPR array separator," eLife 10:e66406, CC BY 4.0, https://doi.org/10.7554/eLife.66406): Cas12a can process a single CRISPR array transcript into multiple guide RNAs (gRNAs), each a repeat (recognized by the enzyme) plus a spacer (targeting DNA). To study multi-gRNA arrays, researchers placed a "dummy" gRNA upstream of a gRNA targeting a GFP reporter, using nuclease-dead Cas12a fused to a transcriptional activator so GFP reported downstream gRNA activity. A synthetic separator was tested by placing it either upstream or only downstream of the reporter gRNA. Short AT-rich synthetic separators (AAAT) placed upstream of a gRNA rescued poorly performing high-GC arrays, whereas the same sequence added only downstream gave no benefit; a full-length natural separator was not fully excised and abolished activity. The optimized upstream-separator design activated seven genes at once. — Question: Adding the AAAT separator only downstream of the reporter gRNA produced no improvement, whereas placing it upstream did. This result most directly indicates that the separator improves performance by:
- **A:** extending the 3′ end of the reporter spacer
- **B:** aiding processing/insulation of the array upstream of a gRNA rather than by modifying that gRNA's 3′ end
- **C:** increasing the GC content of the downstream spacer
- **D:** preventing transcription of the upstream dummy gRNA
- **Correct:** B
- **Explanation:** Benefit only when placed upstream shows the separator acts on processing/insulation ahead of a gRNA, not by lengthening its 3′ end. (A) is the hypothesis the control rules out. (C) is wrong—AT-rich separators lower, not raise, local GC. (D) is not supported; the array is still transcribed.
- **Tags:** `KC::Bio::Biotechnology` `MCAT::Bio_Biochem` `Difficulty::4` `IRT::Discrimination::1.1` `IRT::Guessing::0.25` `Reasoning::Inference`

### MCAT-PASSAGE-BB-004

- **KC:** `Bio::Biotechnology`
- **Prereqs:** none
- **Difficulty:** 4
- **Question:** Passage (Source: Magnusson JP et al. (2021), "Enhanced Cas12a multi-gene regulation using a CRISPR array separator," eLife 10:e66406, CC BY 4.0, https://doi.org/10.7554/eLife.66406): Cas12a processes a single CRISPR array transcript into multiple guide RNAs (gRNAs), each a repeat plus a spacer, using nuclease-dead Cas12a fused to a transcriptional activator so a GFP reporter measured downstream gRNA activity. In natural arrays each gRNA is separated by a short AT-rich "separator" that is excised during processing. Reintroducing the full-length natural separator into synthetic arrays almost eliminated activity because the enzyme could not fully remove the long fragment, which remained attached to the upstream spacer. Short synthetic AT-rich separators (AAAT) placed upstream instead rescued high-GC arrays, and the optimized design activated seven genes at once. — Question: Why did reintroducing the full-length natural separator almost eliminate array activity in human cells?
- **A:** The full separator increased the GC content of every spacer
- **B:** The enzyme could not fully excise the long separator, so it remained attached and impaired the gRNA
- **C:** The full separator prevented the activator from being expressed
- **D:** The full separator caused the DNA target to be cut instead of activated
- **Correct:** B
- **Explanation:** The passage states the enzyme could not fully remove the long fragment, which remained attached to the upstream spacer, impairing function. (A) contradicts the AT-rich nature of the separator. (C) is not claimed. (D) is impossible with a nuclease-dead enzyme.
- **Tags:** `KC::Bio::Biotechnology` `MCAT::Bio_Biochem` `Difficulty::4` `IRT::Discrimination::1.0` `IRT::Guessing::0.25` `Reasoning::Comprehension`

### MCAT-PASSAGE-BB-005

- **KC:** `Bio::Biotechnology`
- **Prereqs:** none
- **Difficulty:** 4
- **Question:** Passage (Source: Magnusson JP et al. (2021), "Enhanced Cas12a multi-gene regulation using a CRISPR array separator," eLife 10:e66406, CC BY 4.0, https://doi.org/10.7554/eLife.66406): Cas12a processes a single CRISPR array transcript into multiple guide RNAs (gRNAs), each a repeat plus a spacer, and a nuclease-dead Cas12a fused to a transcriptional activator lets a GFP reporter measure gRNA activity. Multi-gRNA arrays performed unpredictably: high-GC spacers formed stable RNA secondary structure that impaired processing, and a single-nucleotide change upstream could abolish downstream activation. Short AT-rich synthetic separators (AAAT) placed upstream of each gRNA restored performance, whereas a single G did not and a full-length natural separator (poorly excised) abolished activity. Position of the reporter gRNA within a fixed array did not matter. An array with synthetic separators between all gRNAs activated seven endogenous genes simultaneously. — Question: Based on the passage, which array design would be predicted to give the most reliable simultaneous activation of multiple genes?
- **A:** Multiple gRNAs with high-GC spacers and no separators
- **B:** Multiple gRNAs each preceded by a short AT-rich synthetic separator
- **C:** A single long full-length natural separator placed after the last gRNA
- **D:** Multiple gRNAs arranged so the most important one is always last
- **Correct:** B
- **Explanation:** Short AT-rich separators before each gRNA restored performance and enabled activation of seven genes at once. (A) is the failing design (high GC, no insulation). (C) uses the long separator shown to abolish activity. (D) is undercut by the control showing position did not matter.
- **Tags:** `KC::Bio::Biotechnology` `MCAT::Bio_Biochem` `Difficulty::4` `IRT::Discrimination::1.2` `IRT::Guessing::0.25` `Reasoning::Application`

---

## Biochem::Nucleotides_and_Nucleic_Acids

### MCAT-PASSAGE-BB-006

- **KC:** `Biochem::Nucleotides_and_Nucleic_Acids`
- **Prereqs:** none
- **Difficulty:** 3
- **Question:** Passage (Source: Magnusson JP et al. (2021), "Enhanced Cas12a multi-gene regulation using a CRISPR array separator," eLife 10:e66406, CC BY 4.0, https://doi.org/10.7554/eLife.66406): Cas12a processes a CRISPR array into multiple guide RNAs (gRNAs), each a repeat plus a spacer, and a nuclease-dead Cas12a-activator fusion let a GFP reporter measure downstream gRNA activity. Multi-gRNA arrays behaved unpredictably. Using a structure-prediction tool, researchers found that a spacer's guanine-plus-cytosine (GC) content strongly predicted its tendency to form stable RNA secondary structure, and that greater predicted structure was associated with poorer downstream activity. High-GC spacers, whose bases pair more strongly, appeared to fold in ways that interfered with the enzyme's processing of the array; GC content near the enzyme's cleavage site was especially predictive. — Question: The correlation between high GC content and reduced downstream activity is explained in the passage by the fact that GC-rich sequences:
- **A:** are transcribed more slowly by RNA polymerase
- **B:** form stronger base pairing and more stable RNA secondary structure that interferes with array processing
- **C:** cannot be recognized by the transcriptional activator
- **D:** increase the number of gRNAs produced from the array
- **Correct:** B
- **Explanation:** G–C pairs (three hydrogen bonds) base-pair more strongly, promoting stable secondary structure that impedes Cas12a processing—especially near the cleavage site. (A) concerns transcription rate, which the passage does not invoke. (C) misattributes the effect to activator recognition. (D) is the opposite of the observed impairment.
- **Tags:** `KC::Biochem::Nucleotides_and_Nucleic_Acids` `MCAT::Bio_Biochem` `Difficulty::3` `IRT::Discrimination::1.1` `IRT::Guessing::0.25` `Reasoning::Comprehension`

---

## Bio::Evolution

### MCAT-PASSAGE-BB-007

- **KC:** `Bio::Evolution`
- **Prereqs:** none
- **Difficulty:** 3
- **Question:** Passage (Source: Trindade S et al. (2009), "Positive Epistasis Drives the Acquisition of Multidrug Resistance," PLOS Genetics 5(7):e1000578, CC BY, https://doi.org/10.1371/journal.pgen.1000578): Antibiotic-resistance mutations often carry a fitness cost: without the drug, resistant cells compete less well than sensitive ancestors. Investigators isolated E. coli mutants resistant to one of three drugs with distinct targets—nalidixic acid (DNA gyrase, gyrA), rifampicin (RNA polymerase, rpoB), and streptomycin (ribosome, rpsL)—each from a single nucleotide change. Fitness cost was measured by competing each resistant strain against a marked sensitive strain in drug-free medium; single resistance reduced fitness by about 9% on average. Combining two resistance alleles cost less than the sum of the individual costs (positive epistasis); some pairs added no cost, and a few costly mutations became advantageous alongside another. — Question: Why were the competition assays performed in medium that contained no antibiotic?
- **A:** To select for bacteria that had lost their resistance mutations
- **B:** To isolate the intrinsic fitness cost of the resistance mutation from any effect of the drug
- **C:** Because antibiotics would kill the marked sensitive reference strain instantly
- **D:** To force the bacteria to acquire additional resistance mutations
- **Correct:** B
- **Explanation:** Without the drug, any competitive disadvantage of the resistant strain reflects the burden of the mutation itself, not drug pressure—exactly the quantity the study measures. (A) is not the purpose; the assay measures cost, not reversion selection. (C) may be true but is not why the assay is run drug-free. (D) is unrelated to the assay's purpose.
- **Tags:** `KC::Bio::Evolution` `MCAT::Bio_Biochem` `Difficulty::3` `IRT::Discrimination::1.0` `IRT::Guessing::0.25` `Reasoning::Comprehension`

### MCAT-PASSAGE-BB-008

- **KC:** `Bio::Evolution`
- **Prereqs:** none
- **Difficulty:** 3
- **Question:** Passage (Source: Trindade S et al. (2009), "Positive Epistasis Drives the Acquisition of Multidrug Resistance," PLOS Genetics 5(7):e1000578, CC BY, https://doi.org/10.1371/journal.pgen.1000578): Antibiotic-resistance mutations often reduce competitive fitness in the absence of the drug. Investigators measured the fitness cost of single resistance mutations in E. coli by competing each against a marked sensitive strain in drug-free medium, then combined pairs of resistance alleles into one background. If mutations acted independently, the double-resistance cost should approximate the sum of the two single costs. Instead the average cost of double resistance was consistently less than that sum—a pattern the authors call pervasive positive epistasis. In some pairings the second resistance added no measurable cost, and in a few cases a mutation costly on its own became advantageous alongside another. — Question: In this study, "positive epistasis" between two resistance mutations means that the combined fitness cost of carrying both is:
- **A:** greater than the sum of the individual costs
- **B:** less than the sum of the individual costs
- **C:** exactly equal to the sum of the individual costs
- **D:** always zero
- **Correct:** B
- **Explanation:** The passage defines the observed positive epistasis as double-resistance costs being less than the sum expected from independent effects. (A) describes negative epistasis. (C) describes the no-epistasis (additive) baseline. (D) overstates it—some combinations are cost-free, but positive epistasis does not require zero cost.
- **Tags:** `KC::Bio::Evolution` `MCAT::Bio_Biochem` `Difficulty::3` `IRT::Discrimination::0.9` `IRT::Guessing::0.25` `Reasoning::Comprehension`

### MCAT-PASSAGE-BB-009

- **KC:** `Bio::Evolution`
- **Prereqs:** none
- **Difficulty:** 3
- **Question:** Passage (Source: Trindade S et al. (2009), "Positive Epistasis Drives the Acquisition of Multidrug Resistance," PLOS Genetics 5(7):e1000578, CC BY, https://doi.org/10.1371/journal.pgen.1000578): The fitness cost of antibiotic resistance helps determine whether resistance persists once drug use stops. Investigators combined pairs of resistance alleles in E. coli and found the cost of double resistance was consistently less than the sum of the single costs (positive epistasis); in some pairings the second resistance added no measurable cost at all. Because acquiring a second resistance often costs far less than an additive model predicts—and can be nearly free—selection against multidrug-resistant bacteria in drug-free environments may be much weaker than expected, so favorable combinations tend to accumulate and spread. — Question: The finding that some double-resistant strains have essentially no added fitness cost most strongly implies that, in a drug-free setting, such strains will:
- **A:** be rapidly eliminated by natural selection
- **B:** experience little selective disadvantage and therefore tend to persist
- **C:** immediately lose one of their resistance mutations
- **D:** grow faster than any sensitive strain under all conditions
- **Correct:** B
- **Explanation:** Weak or absent cost means weak selection against the strain, so it can persist even without the drug. (A) is the opposite of the passage's conclusion. (C) is not implied; low cost reduces pressure to revert. (D) overreaches—only a few combinations were beneficial, not universally faster.
- **Tags:** `KC::Bio::Evolution` `MCAT::Bio_Biochem` `Difficulty::3` `IRT::Discrimination::1.1` `IRT::Guessing::0.25` `Reasoning::Inference`

### MCAT-PASSAGE-BB-010

- **KC:** `Bio::Evolution`
- **Prereqs:** none
- **Difficulty:** 4
- **Question:** Passage (Source: Trindade S et al. (2009), "Positive Epistasis Drives the Acquisition of Multidrug Resistance," PLOS Genetics 5(7):e1000578, CC BY, https://doi.org/10.1371/journal.pgen.1000578): Investigators measured the fitness cost of single and paired antibiotic-resistance mutations in E. coli using drug-free competition assays. The average cost of double resistance was consistently less than the sum of the single costs (positive epistasis). In a few striking cases a mutation that was costly on its own became advantageous when placed alongside another resistance mutation. These interactions were allele-specific rather than gene-specific: two different mutations in the same gene could interact very differently with a given partner, and one mutation might interact favorably with one partner but unfavorably with another. — Question: A mutation that reduces fitness on its own but increases fitness when combined with a second resistance mutation is best characterized as:
- **A:** a compensatory interaction (sign epistasis) between the two alleles
- **B:** evidence that the mutations are on the same codon
- **C:** proof that resistance carries no cost
- **D:** an example of independent, additive effects
- **Correct:** A
- **Explanation:** When a mutation's fitness effect flips sign depending on the genetic background, the interaction is compensatory (sign epistasis)—as the passage notes for cases where a costly mutation became advantageous alongside another. (B) is unsupported and irrelevant to the fitness logic. (C) overgeneralizes from special cases. (D) contradicts the very presence of an interaction.
- **Tags:** `KC::Bio::Evolution` `MCAT::Bio_Biochem` `Difficulty::4` `IRT::Discrimination::1.2` `IRT::Guessing::0.25` `Reasoning::Inference`

---

## Bio::Translation

### MCAT-PASSAGE-BB-011

- **KC:** `Bio::Translation`
- **Prereqs:** none
- **Difficulty:** 2
- **Question:** Passage (Source: Trindade S et al. (2009), "Positive Epistasis Drives the Acquisition of Multidrug Resistance," PLOS Genetics 5(7):e1000578, CC BY, https://doi.org/10.1371/journal.pgen.1000578): Investigators isolated E. coli mutants resistant to one of three antibiotics with distinct cellular targets. Nalidixic acid acts on DNA gyrase and blocks DNA replication; resistance arose from mutations in gyrA. Rifampicin binds the β-subunit of RNA polymerase and blocks transcription; resistance arose in rpoB. Streptomycin binds the ribosome and blocks the elongation step of translation; resistance arose in rpsL. Each resistant clone carried a single nucleotide change in the relevant gene. — Question: Which resistance mechanism described in the passage would most directly impair a cell's ability to synthesize proteins?
- **A:** A gyrA mutation conferring nalidixic-acid resistance
- **B:** An rpoB mutation conferring rifampicin resistance
- **C:** An rpsL mutation conferring streptomycin resistance
- **D:** None; antibiotic resistance never affects protein synthesis
- **Correct:** C
- **Explanation:** Streptomycin targets the ribosome and blocks translation elongation, so rpsL-mediated changes involve the protein-synthesis machinery. (A) involves DNA gyrase (replication). (B) involves RNA polymerase (transcription). (D) is contradicted by the ribosomal target described.
- **Tags:** `KC::Bio::Translation` `MCAT::Bio_Biochem` `Difficulty::2` `IRT::Discrimination::0.8` `IRT::Guessing::0.25` `Reasoning::Application`

---

## Bio::Circulatory_System

### MCAT-PASSAGE-BB-012

- **KC:** `Bio::Circulatory_System`
- **Prereqs:** none
- **Difficulty:** 2
- **Question:** Passage (Source: Berry MJ et al. (2020), "A randomized controlled trial of nitrate supplementation in well-trained middle and older-aged adults," PLOS ONE 15(6):e0235047, CC BY, https://doi.org/10.1371/journal.pone.0235047): Nitric oxide (NO) widens blood vessels, influences how efficiently mitochondria use oxygen, and affects muscle contraction. Cells make NO from L-arginine via NO synthase, but dietary nitrate can also be reduced first to nitrite and then to NO. Nitrate-rich foods such as beetroot raise circulating nitrate and nitrite, and some studies report this lowers the oxygen cost of exercise and improves endurance, effects seen most clearly in untrained people or patients. — Question: In the alternative (non-enzymatic) pathway described, dietary nitrate is converted to NO in which order?
- **A:** Nitrate → nitric oxide → nitrite
- **B:** Nitrite → nitrate → nitric oxide
- **C:** Nitrate → nitrite → nitric oxide
- **D:** L-arginine → nitrate → nitrite
- **Correct:** C
- **Explanation:** The passage states dietary nitrate is reduced first to nitrite and then to NO. (A) and (B) scramble the order. (D) describes the separate L-arginine/NO-synthase route, not the dietary-nitrate reduction pathway.
- **Tags:** `KC::Bio::Circulatory_System` `MCAT::Bio_Biochem` `Difficulty::2` `IRT::Discrimination::0.8` `IRT::Guessing::0.25` `Reasoning::Comprehension`

### MCAT-PASSAGE-BB-013

- **KC:** `Bio::Circulatory_System`
- **Prereqs:** none
- **Difficulty:** 3
- **Question:** Passage (Source: Berry MJ et al. (2020), "A randomized controlled trial of nitrate supplementation in well-trained middle and older-aged adults," PLOS ONE 15(6):e0235047, CC BY, https://doi.org/10.1371/journal.pone.0235047): To test whether dietary nitrate improves endurance in healthy well-trained adults, researchers ran a double-blind, placebo-controlled, crossover trial with fifteen adults aged 41 to 64. Each participant completed the study twice, once consuming a nitrate-rich beverage daily for seven days and once a nitrate-depleted placebo that was otherwise identical in taste and appearance; the order was randomized, and neither participants nor staff knew which beverage was in use. The primary outcome was time cycling at a constant submaximal work rate, and blood samples confirmed the beverages changed nitrate and nitrite as intended. — Question: Why did the investigators use a crossover design in which each participant received both the nitrate and placebo beverages?
- **A:** To increase the total number of distinct participants needed
- **B:** So each participant serves as their own control, reducing the influence of between-person differences
- **C:** To guarantee a statistically significant result
- **D:** To prevent plasma nitrate levels from changing
- **Correct:** B
- **Explanation:** In a crossover, each person experiences both conditions, so stable individual traits are controlled within-subject. (A) is backwards—crossover designs typically need fewer participants. (C) is false; design cannot guarantee significance, and here the result was non-significant. (D) contradicts the confirmed rise in plasma nitrate.
- **Tags:** `KC::Bio::Circulatory_System` `MCAT::Bio_Biochem` `Difficulty::3` `IRT::Discrimination::0.9` `IRT::Guessing::0.25` `Reasoning::Application`

### MCAT-PASSAGE-BB-014

- **KC:** `Bio::Circulatory_System`
- **Prereqs:** none
- **Difficulty:** 3
- **Question:** Passage (Source: Berry MJ et al. (2020), "A randomized controlled trial of nitrate supplementation in well-trained middle and older-aged adults," PLOS ONE 15(6):e0235047, CC BY, https://doi.org/10.1371/journal.pone.0235047): In a crossover trial of dietary nitrate in well-trained adults, the biochemical manipulation succeeded: after the nitrate-rich beverage, plasma nitrate rose by roughly 260 micromolar and nitrite by about 0.47 micromolar relative to placebo, both highly statistically significant. The performance results did not follow—mean exercise time and oxygen consumption did not differ significantly. The authors noted the confirmed rise was informative because it shows the absence of a clear performance effect cannot be blamed on a failure to deliver the compound. — Question: The researchers measured plasma nitrate and nitrite mainly in order to:
- **A:** confirm that the supplement actually delivered nitrate, so a null performance result could not be attributed to failed delivery
- **B:** use plasma nitrate as the study's primary performance outcome
- **C:** determine each participant's maximum work rate
- **D:** show that the placebo raised nitrite more than the nitrate beverage
- **Correct:** A
- **Explanation:** The passage calls the confirmed rise informative because it shows the absence of a clear performance effect cannot be blamed on a failure to deliver the compound—a manipulation check. (B) is wrong; the primary outcome was exercise time. (C) refers to a separate baseline test. (D) reverses the finding (the nitrate beverage raised levels).
- **Tags:** `KC::Bio::Circulatory_System` `MCAT::Bio_Biochem` `Difficulty::3` `IRT::Discrimination::1.0` `IRT::Guessing::0.25` `Reasoning::Comprehension`

### MCAT-PASSAGE-BB-015

- **KC:** `Bio::Circulatory_System`
- **Prereqs:** none
- **Difficulty:** 3
- **Question:** Passage (Source: Berry MJ et al. (2020), "A randomized controlled trial of nitrate supplementation in well-trained middle and older-aged adults," PLOS ONE 15(6):e0235047, CC BY, https://doi.org/10.1371/journal.pone.0235047): Dietary nitrate raises circulating nitrate and nitrite and, via the NO pathway, can widen blood vessels and influence mitochondrial oxygen use; benefits on the oxygen cost of exercise are seen most clearly in untrained people or patients. In a crossover trial with fifteen well-trained adults, nitrate supplementation produced non-significant, highly variable effects on exercise tolerance. The authors note that effects seen in one population may be weaker or absent in another that is already highly trained. — Question: The passage suggests that dietary nitrate may benefit performance less in well-trained adults than in untrained people. Which explanation is most consistent with the passage's framing?
- **A:** Well-trained individuals cannot absorb nitrate from food
- **B:** Trained individuals may already have well-optimized vascular and mitochondrial function, leaving less room for improvement (a ceiling effect)
- **C:** Training destroys NO synthase enzymes
- **D:** Nitrate is converted to a different molecule in trained people
- **Correct:** B
- **Explanation:** The passage notes benefits are seen most clearly in untrained people or patients and that effects may be weaker or absent in an already highly trained population, consistent with a ceiling effect. (A) contradicts the confirmed plasma rise (absorption succeeded). (C) and (D) invent mechanisms the passage never describes.
- **Tags:** `KC::Bio::Circulatory_System` `MCAT::Bio_Biochem` `Difficulty::3` `IRT::Discrimination::1.1` `IRT::Guessing::0.25` `Reasoning::Inference`

---

## Bio::Muscular_System

### MCAT-PASSAGE-BB-016

- **KC:** `Bio::Muscular_System`
- **Prereqs:** none
- **Difficulty:** 4
- **Question:** Passage (Source: Berry MJ et al. (2020), "A randomized controlled trial of nitrate supplementation in well-trained middle and older-aged adults," PLOS ONE 15(6):e0235047, CC BY, https://doi.org/10.1371/journal.pone.0235047): In a double-blind crossover trial, fifteen well-trained adults took a nitrate-rich or nitrate-depleted beverage for seven days each. Plasma nitrate and nitrite rose significantly after the nitrate beverage, confirming delivery. However, mean cycling time at a submaximal work rate was about 1131 seconds after nitrate versus about 1060 seconds after placebo, a difference that was not statistically significant; oxygen consumption, heart rate, and perceived exertion also did not differ. The authors caution that with only fifteen participants and large variability, a non-significant result does not prove nitrate has no effect. — Question: Which is the most appropriate interpretation of the non-significant difference in exercise time (about 1131 vs. 1060 seconds)?
- **A:** Nitrate supplementation definitively has no physiological effect in any population
- **B:** The study proves nitrate impairs exercise tolerance
- **C:** This study did not detect a benefit, but the small sample and high variability limit the ability to rule out a real effect
- **D:** The manipulation of plasma nitrate must have failed
- **Correct:** C
- **Explanation:** A non-significant result with n = 15 and high variability means did not detect, not proved absent, as the passage explicitly cautions. (A) overgeneralizes beyond this trained population. (B) is unsupported—the nitrate mean was numerically higher, not lower, and non-significant. (D) is contradicted by the confirmed, significant rise in plasma nitrate.
- **Tags:** `KC::Bio::Muscular_System` `MCAT::Bio_Biochem` `Difficulty::4` `IRT::Discrimination::1.0` `IRT::Guessing::0.25` `Reasoning::Data`
