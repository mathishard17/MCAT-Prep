# MCAT Bio/Biochem Practice Passages

**All passages below are ORIGINAL prose** written for MCAT-style "Biological and
Biochemical Foundations of Living Systems" practice. Each science passage is an original
paraphrase of the study design and findings of a **real, open-access (CC BY)** article;
**no text is reproduced verbatim**. Every passage carries a full **Source** citation
(authors, title, journal, year, license, DOI). Numerical results are drawn from the cited
papers; some values are rounded and some experimental detail is simplified for a
passage-length treatment.

Each question lists the **Correct** answer and an **Explanation** covering why the key is
right and the distractors wrong. Questions are answerable from the passage plus
foundational MCAT science.

Contents: **3 passages, 16 questions total.**

- `SCI-BIO-P01` — molecular biology / gene regulation (CRISPR–Cas12a) — 6 Q
- `SCI-BIO-P02` — genetics & evolution (antibiotic-resistance fitness costs) — 5 Q
- `SCI-BIO-P03` — physiology / biochemistry (dietary nitrate and the NO pathway) — 5 Q

---

## SCI-BIO-P01
- **Discipline:** Bio/Biochem
- **Source:** Original synthetic passage based on Magnusson JP, et al. (2021), "Enhanced Cas12a multi-gene regulation using a CRISPR array separator," *eLife* 10:e66406, CC BY 4.0, https://doi.org/10.7554/eLife.66406. No text reproduced verbatim; values simplified.
- **Passage:** CRISPR–Cas systems can be repurposed to control several genes at once. The Cas12a enzyme is attractive for this because, unlike Cas9, it can process a single long transcript—a CRISPR array—into multiple guide RNAs (gRNAs) on its own. Each gRNA consists of a repeat, which folds into a structure the enzyme recognizes, and a spacer, whose sequence directs the enzyme to a matching DNA site. In practice, however, arrays that encode several gRNAs often perform unpredictably, with some gRNAs far less active than others.

  To study this, researchers built arrays in which a first "dummy" gRNA of defined sequence preceded a second gRNA that targeted a reporter gene (GFP). They used a nuclease-dead Cas12a fused to a transcriptional activator, so that the complex switched the reporter on rather than cutting DNA; GFP fluorescence therefore reported the activity of the downstream gRNA. If the dummy spacer had no effect on its neighbor, every construct should yield equal GFP. Instead, activity varied dramatically: in extreme cases, changing a single nucleotide in the upstream dummy spacer nearly abolished activation of the downstream reporter.

  The team suspected RNA folding. Using a structure-prediction tool, they found that the guanine-plus-cytosine (GC) content of a spacer strongly predicted its tendency to form stable secondary structure, and that greater predicted structure was associated with poorer activity of the downstream gRNA. High-GC spacers, whose bases pair more strongly, appeared to fold in ways that interfered with the enzyme's processing of the array; GC content near the enzyme's cleavage site was especially predictive.

  Looking to nature for a solution, the researchers noticed that in natural arrays each gRNA is normally separated from the next by a short, AT-rich fragment—a "separator"—that is excised during processing. They reasoned that this AT-rich segment might insulate each gRNA from the folding of its upstream neighbor. Reintroducing the full natural separator into their synthetic arrays, however, almost eliminated activity, because the enzyme could not fully remove the long fragment, which remained attached to the upstream spacer. They therefore designed very short synthetic separators of one to four nucleotides. An AT-rich separator (AAAT) placed before a gRNA markedly improved activation of poorly performing, high-GC arrays, whereas a single guanine (G) did not.

  Two controls sharpened the interpretation. First, adding the AAAT sequence only downstream of the reporter gRNA gave no benefit, indicating that the separator acts by improving processing of the array upstream of a gRNA rather than by extending that gRNA's 3′ end. Second, when the reporter gRNA was moved to different positions within an array of otherwise fixed composition, its activity did not change, arguing against a simple position effect. Finally, an array carrying the synthetic separator between all of its gRNAs allowed the simultaneous activation of seven different endogenous genes in human cells, with improvements over the separator-free array ranging from modest to roughly eightfold across targets.

### SCI-BIO-P01-Q1
- **Stem:** Why did the researchers use a nuclease-dead Cas12a fused to a transcriptional activator together with a GFP reporter?
- **A:** To cut the GFP gene out of the genome and measure repair efficiency
- **B:** So that GFP fluorescence would report how effectively the downstream gRNA directed gene activation
- **C:** To prevent the CRISPR array from being transcribed
- **D:** To ensure the dummy spacer could bind DNA directly without a gRNA
- **Correct:** B
- **Explanation:** A catalytically dead Cas12a fused to an activator turns a target gene *on* instead of cutting it, so GFP output serves as a readout of downstream gRNA activity. (A) is wrong—the enzyme is nuclease-*dead*, so it does not cut. (C) is wrong; the array must be transcribed and processed for the system to work. (D) misstates the mechanism—spacers guide the protein via the gRNA, not on their own.

### SCI-BIO-P01-Q2
- **Stem:** A single-nucleotide change in the *upstream* dummy spacer sometimes nearly abolished activation of the *downstream* reporter. This best supports the conclusion that:
- **A:** the two gRNAs are transcribed from independent promoters
- **B:** the sequence of one spacer can influence the performance of a neighboring gRNA in the same array
- **C:** GFP fluorescence is unrelated to gRNA activity
- **D:** Cas12a cannot process arrays containing more than one gRNA
- **Correct:** B
- **Explanation:** If an upstream change alters downstream output, the spacers are not acting independently—one gRNA's sequence affects its neighbor. (A) contradicts the single-array design (one transcript). (C) contradicts the reporter logic established in the passage. (D) is false; Cas12a does process multi-gRNA arrays, just unevenly.

### SCI-BIO-P01-Q3
- **Stem:** The correlation between high GC content and reduced downstream activity is explained in the passage by the fact that GC-rich sequences:
- **A:** are transcribed more slowly by RNA polymerase
- **B:** form stronger base pairing and more stable RNA secondary structure that interferes with array processing
- **C:** cannot be recognized by the transcriptional activator
- **D:** increase the number of gRNAs produced from the array
- **Correct:** B
- **Explanation:** G–C pairs (three hydrogen bonds) base-pair more strongly, promoting stable secondary structure that impedes Cas12a processing—especially near the cleavage site. (A) concerns transcription rate, which the passage does not invoke. (C) misattributes the effect to activator recognition. (D) is the opposite of the observed impairment.

### SCI-BIO-P01-Q4
- **Stem:** Adding the AAAT separator *only downstream* of the reporter gRNA produced no improvement, whereas placing it *upstream* did. This result most directly indicates that the separator improves performance by:
- **A:** extending the 3′ end of the reporter spacer
- **B:** aiding processing/insulation of the array upstream of a gRNA rather than by modifying that gRNA's 3′ end
- **C:** increasing the GC content of the downstream spacer
- **D:** preventing transcription of the upstream dummy gRNA
- **Correct:** B
- **Explanation:** Benefit only when placed upstream shows the separator acts on processing/insulation ahead of a gRNA, not by lengthening its 3′ end. (A) is the hypothesis the control rules out. (C) is wrong—AT-rich separators lower, not raise, local GC. (D) is not supported; the array is still transcribed.

### SCI-BIO-P01-Q5
- **Stem:** Why did reintroducing the *full-length* natural separator almost eliminate array activity in human cells?
- **A:** The full separator increased the GC content of every spacer
- **B:** The enzyme could not fully excise the long separator, so it remained attached and impaired the gRNA
- **C:** The full separator prevented the activator from being expressed
- **D:** The full separator caused the DNA target to be cut instead of activated
- **Correct:** B
- **Explanation:** The passage states the enzyme "could not fully remove the long fragment, which remained attached to the upstream spacer," impairing function. (A) contradicts the AT-rich nature of the separator. (C) is not claimed. (D) is impossible with a nuclease-dead enzyme.

### SCI-BIO-P01-Q6
- **Stem:** Based on the passage, which array design would be predicted to give the *most reliable* simultaneous activation of multiple genes?
- **A:** Multiple gRNAs with high-GC spacers and no separators
- **B:** Multiple gRNAs each preceded by a short AT-rich synthetic separator
- **C:** A single long full-length natural separator placed after the last gRNA
- **D:** Multiple gRNAs arranged so the most important one is always last
- **Correct:** B
- **Explanation:** Short AT-rich separators before each gRNA restored performance and enabled activation of seven genes at once. (A) is the failing design (high GC, no insulation). (C) uses the long separator shown to abolish activity. (D) is undercut by the control showing position did not matter.

---

## SCI-BIO-P02
- **Discipline:** Bio/Biochem
- **Source:** Original synthetic passage based on Trindade S, Sousa A, Xavier KB, Dionisio F, Ferreira MG, Gordo I (2009), "Positive Epistasis Drives the Acquisition of Multidrug Resistance," *PLOS Genetics* 5(7):e1000578, CC BY, https://doi.org/10.1371/journal.pgen.1000578. No text reproduced verbatim; values simplified.
- **Passage:** Mutations that make bacteria resistant to antibiotics frequently carry a cost: in the absence of the drug, the resistant cell competes less successfully than its sensitive ancestor. This cost matters clinically, because it helps determine whether resistance will persist once antibiotic use stops. When a strain carries resistance to several drugs at once, a central question is whether the total cost is simply the sum of the individual costs, or whether the mutations interact.

  To address this, investigators isolated spontaneous mutants of *Escherichia coli* resistant to one of three antibiotics with distinct cellular targets. Nalidixic acid acts on DNA gyrase and blocks DNA replication; resistance arose from mutations in *gyrA*. Rifampicin binds the β-subunit of RNA polymerase and blocks transcription; resistance arose in *rpoB*. Streptomycin binds the ribosome and blocks the elongation step of translation; resistance arose in *rpsL*. Each resistant clone carried a single nucleotide change in the relevant gene.

  The fitness cost of each mutation was measured with a competition assay. A resistant strain was grown together with a marked sensitive reference strain in drug-free medium, and the change in their relative numbers over time revealed which competitor grew faster. Because no antibiotic was present, any disadvantage of the resistant strain reflected the intrinsic burden of the resistance mutation rather than any effect of the drug. Averaged across clones, single resistance reduced competitive fitness by about 9 percent, though the burden varied by drug: nalidixic-acid resistance cost roughly 3 percent, rifampicin about 9 percent, and streptomycin about 13 percent.

  The researchers then combined pairs of resistance alleles into a single genetic background and measured the fitness of the resulting double-resistant strains. If the two mutations acted independently, the cost of double resistance should approximate the sum of the two single costs. Instead, the average cost of double resistance was consistently *less* than that sum—a pattern the authors describe as pervasive positive epistasis among resistance alleles. In some pairings the second resistance added no measurable cost at all, and in a few striking cases a mutation that was costly on its own became advantageous when placed alongside another resistance mutation.

  These interactions were allele-specific rather than gene-specific: two different mutations in the same gene could interact very differently with a given partner, and one mutation might interact favorably with one partner but unfavorably with another. The practical implication is sobering. Because acquiring a second resistance often costs far less than a simple additive model predicts—and can even be nearly free—selection against multidrug-resistant bacteria in drug-free environments may be much weaker than expected. Combinations that happen to interact favorably are the ones most likely to accumulate and spread, helping to explain why multidrug resistance, once established, can be so difficult to reverse.

### SCI-BIO-P02-Q1
- **Stem:** Why were the competition assays performed in medium that contained *no* antibiotic?
- **A:** To select for bacteria that had lost their resistance mutations
- **B:** To isolate the intrinsic fitness cost of the resistance mutation from any effect of the drug
- **C:** Because antibiotics would kill the marked sensitive reference strain instantly
- **D:** To force the bacteria to acquire additional resistance mutations
- **Correct:** B
- **Explanation:** Without the drug, any competitive disadvantage of the resistant strain reflects the burden of the mutation itself, not drug pressure—exactly the quantity the study measures. (A) is not the purpose; the assay measures cost, not reversion selection. (C) may be true but is not why the assay is run drug-free (the point is measuring intrinsic cost). (D) is unrelated to the assay's purpose.

### SCI-BIO-P02-Q2
- **Stem:** In this study, "positive epistasis" between two resistance mutations means that the combined fitness cost of carrying both is:
- **A:** greater than the sum of the individual costs
- **B:** less than the sum of the individual costs
- **C:** exactly equal to the sum of the individual costs
- **D:** always zero
- **Correct:** B
- **Explanation:** The passage defines the observed positive epistasis as double-resistance costs being "less than that sum" expected from independent effects. (A) describes negative epistasis. (C) describes the no-epistasis (additive) baseline. (D) overstates it—some combinations are cost-free, but positive epistasis does not require zero cost.

### SCI-BIO-P02-Q3
- **Stem:** Which resistance mechanism described in the passage would most directly impair a cell's ability to synthesize proteins?
- **A:** A *gyrA* mutation conferring nalidixic-acid resistance
- **B:** An *rpoB* mutation conferring rifampicin resistance
- **C:** An *rpsL* mutation conferring streptomycin resistance
- **D:** None; antibiotic resistance never affects protein synthesis
- **Correct:** C
- **Explanation:** Streptomycin targets the ribosome and blocks translation elongation, so *rpsL*-mediated changes involve the translation machinery. (A) involves DNA gyrase (replication). (B) involves RNA polymerase (transcription). (D) is contradicted by the ribosomal target described.

### SCI-BIO-P02-Q4
- **Stem:** The finding that some double-resistant strains have essentially no added fitness cost most strongly implies that, in a drug-free setting, such strains will:
- **A:** be rapidly eliminated by natural selection
- **B:** experience little selective disadvantage and therefore tend to persist
- **C:** immediately lose one of their resistance mutations
- **D:** grow faster than any sensitive strain under all conditions
- **Correct:** B
- **Explanation:** Weak or absent cost means weak selection against the strain, so it can persist even without the drug. (A) is the opposite of the passage's conclusion. (C) is not implied; low cost reduces pressure to revert. (D) overreaches—only a few combinations were beneficial, not universally faster.

### SCI-BIO-P02-Q5
- **Stem:** A mutation that reduces fitness on its own but *increases* fitness when combined with a second resistance mutation is best characterized as:
- **A:** a compensatory interaction (sign epistasis) between the two alleles
- **B:** evidence that the mutations are on the same codon
- **C:** proof that resistance carries no cost
- **D:** an example of independent, additive effects
- **Correct:** A
- **Explanation:** When a mutation's fitness effect flips sign depending on the genetic background, the interaction is compensatory (sign epistasis)—as the passage notes for cases where a costly mutation "became advantageous when placed alongside another." (B) is unsupported and irrelevant to the fitness logic. (C) overgeneralizes from special cases. (D) contradicts the very presence of an interaction.

---

## SCI-BIO-P03
- **Discipline:** Bio/Biochem
- **Source:** Original synthetic passage based on Berry MJ, Miller GD, Kim-Shapiro DB, Fletcher MS, Jones CG, Gauthier ZD, et al. (2020), "A randomized controlled trial of nitrate supplementation in well-trained middle and older-aged adults," *PLOS ONE* 15(6):e0235047, CC BY, https://doi.org/10.1371/journal.pone.0235047. No text reproduced verbatim; values rounded.
- **Passage:** Nitric oxide (NO) is a small signaling molecule that widens blood vessels, influences how efficiently mitochondria use oxygen, and affects muscle contraction. Cells can synthesize NO from the amino acid L-arginine using NO synthase enzymes, but NO can also be produced by a separate route in which dietary nitrate is reduced first to nitrite and then to NO. Foods rich in nitrate, such as beetroot, raise circulating nitrate and nitrite, and some studies report that this lowers the oxygen cost of exercise and improves endurance—effects seen most clearly in untrained people or patients. Whether the same benefit occurs in healthy, well-trained middle-aged and older adults was unclear.

  To test this, researchers ran a double-blind, placebo-controlled, crossover trial with fifteen well-trained adults aged 41 to 64. Each participant completed the study twice, once consuming a nitrate-rich beverage daily for seven days and once consuming a nitrate-depleted placebo beverage that was otherwise identical in taste and appearance; the order of the two conditions was randomized, and neither the participants nor the staff collecting data knew which beverage was in use at a given time. The primary outcome was the length of time a participant could sustain cycling at a constant, submaximal work rate set to 75 percent of a previously measured maximum—a common measure of exercise tolerance. Blood samples were drawn to confirm that the beverages changed nitrate and nitrite levels as intended.

  The biochemical manipulation succeeded: after the nitrate-rich beverage, plasma nitrate rose by roughly 260 micromolar and nitrite by about 0.47 micromolar relative to placebo, both highly statistically significant. The performance results, however, did not follow. Mean exercise time was about 1131 seconds after the nitrate-rich beverage and about 1060 seconds after placebo, a difference that was not statistically significant. Oxygen consumption, heart rate, and ratings of perceived exertion during exercise also did not differ significantly between conditions. The authors noted that individual responses were highly variable, with some participants improving and others not.

  They concluded that nitrate supplementation had non-significant, though highly variable, effects on exercise tolerance in this population. Importantly, a non-significant result of this kind does not prove that nitrate has no effect; with only fifteen participants and large person-to-person variability, the study may have lacked the statistical power to detect a small or inconsistent benefit. The confirmed rise in plasma nitrate and nitrite is nonetheless informative, because it shows that the absence of a clear performance effect cannot be blamed on a failure to deliver the compound. The findings illustrate a recurring theme in physiology: an intervention that reliably changes a biochemical marker does not necessarily produce a measurable change in whole-body performance, and effects seen in one population may be weaker or absent in another that is already highly trained.

### SCI-BIO-P03-Q1
- **Stem:** In the alternative (non-enzymatic) pathway described, dietary nitrate is converted to NO in which order?
- **A:** Nitrate → nitric oxide → nitrite
- **B:** Nitrite → nitrate → nitric oxide
- **C:** Nitrate → nitrite → nitric oxide
- **D:** L-arginine → nitrate → nitrite
- **Correct:** C
- **Explanation:** The passage states dietary nitrate "is reduced first to nitrite and then to NO." (A) and (B) scramble the order. (D) describes the separate L-arginine/NO-synthase route, not the dietary-nitrate reduction pathway.

### SCI-BIO-P03-Q2
- **Stem:** Why did the investigators use a crossover design in which each participant received both the nitrate and placebo beverages?
- **A:** To increase the total number of distinct participants needed
- **B:** So each participant serves as their own control, reducing the influence of between-person differences
- **C:** To guarantee a statistically significant result
- **D:** To prevent plasma nitrate levels from changing
- **Correct:** B
- **Explanation:** In a crossover, each person experiences both conditions, so stable individual traits are controlled within-subject. (A) is backwards—crossover designs typically need *fewer* participants. (C) is false; design cannot guarantee significance, and here the result was non-significant. (D) contradicts the confirmed rise in plasma nitrate.

### SCI-BIO-P03-Q3
- **Stem:** The researchers measured plasma nitrate and nitrite mainly in order to:
- **A:** confirm that the supplement actually delivered nitrate, so a null performance result could not be attributed to failed delivery
- **B:** use plasma nitrate as the study's primary performance outcome
- **C:** determine each participant's maximum work rate
- **D:** show that the placebo raised nitrite more than the nitrate beverage
- **Correct:** A
- **Explanation:** The passage calls the confirmed rise "informative, because it shows that the absence of a clear performance effect cannot be blamed on a failure to deliver the compound"—a manipulation check. (B) is wrong; the primary outcome was exercise time. (C) refers to a separate baseline test. (D) reverses the finding (the nitrate beverage raised levels).

### SCI-BIO-P03-Q4
- **Stem:** Which is the most appropriate interpretation of the non-significant difference in exercise time (about 1131 vs. 1060 seconds)?
- **A:** Nitrate supplementation definitively has no physiological effect in any population
- **B:** The study proves nitrate impairs exercise tolerance
- **C:** This study did not detect a benefit, but the small sample and high variability limit the ability to rule out a real effect
- **D:** The manipulation of plasma nitrate must have failed
- **Correct:** C
- **Explanation:** A non-significant result with n = 15 and high variability means "did not detect," not "proved absent," as the passage explicitly cautions. (A) overgeneralizes beyond this trained population. (B) is unsupported—the nitrate mean was numerically higher, not lower, and non-significant. (D) is contradicted by the confirmed, significant rise in plasma nitrate.

### SCI-BIO-P03-Q5
- **Stem:** The passage suggests that dietary nitrate may benefit performance less in well-trained adults than in untrained people. Which explanation is most consistent with the passage's framing?
- **A:** Well-trained individuals cannot absorb nitrate from food
- **B:** Trained individuals may already have well-optimized vascular and mitochondrial function, leaving less room for improvement (a ceiling effect)
- **C:** Training destroys NO synthase enzymes
- **D:** Nitrate is converted to a different molecule in trained people
- **Correct:** B
- **Explanation:** The passage notes benefits are "seen most clearly in untrained people or patients" and that effects "may be weaker or absent in another [population] that is already highly trained," consistent with a ceiling effect. (A) contradicts the confirmed plasma rise (absorption succeeded). (C) and (D) invent mechanisms the passage never describes.
