# MCAT Generated Cards — Psych/Soc Passages

Importable multiple-choice cards **converted from the reading passages in
`added features/passages-psych-soc.md`** so those (previously orphaned) passages
become scored content in the Psychological, Social, and Biological Foundations of
Behavior (Psych/Soc) section. Every card keeps its original passage **Source**
citation, correct answer, and explanation; no new science is invented.

Block format matches the strict, line-based importer used by the other
`generated-cards-*.md` files (one field per line, `## <KC id>` group headers,
byte-exact `KC::` tags from `added features/kc-map-unified.md` §6). Each Question
line carries a trimmed (~120-word) version of the passage plus the question stem.

- **Source file:** `added features/passages-psych-soc.md` (3 passages, 16 questions).
- **Total cards:** 16 (`MCAT-PASSAGE-PS-001` … `MCAT-PASSAGE-PS-016`).
- **Coverage:** both sub-domains — **psychology** (`PsychSoc::Cognition`,
  `PsychSoc::Motivation`, `PsychSoc::Stereotypes`) and **sociology**
  (`PsychSoc::Stratification`, `PsychSoc::Social_Inequality`).
- **KC ids used (all real, from §6):** `PsychSoc::Cognition` (Psy),
  `PsychSoc::Motivation` (Psy), `PsychSoc::Stereotypes` (Psy),
  `PsychSoc::Stratification` (**Soc**), `PsychSoc::Social_Inequality` (**Soc**).
- Primary section is `MCAT::Psych_Soc` for every card. Passage IRT/reasoning
  metadata (`IRT::Discrimination`, `IRT::Guessing`, `Reasoning::…`) is attached
  per the passage-scoring convention.

---

## PsychSoc::Cognition

### MCAT-PASSAGE-PS-001

- **KC:** `PsychSoc::Cognition`
- **Prereqs:** none
- **Difficulty:** 2
- **Question:** Passage (Source: Radel R, Gruet M, Barzykowski K (2019), "Testing the ego-depletion effect in optimized conditions," PLOS ONE 14(3):e0213026, CC BY, https://doi.org/10.1371/journal.pone.0213026): Ego-depletion theory proposes self-control draws on a limited resource, so exerting it on one task leaves less for the next; early support later looked inflated by publication bias, and a large pre-registered replication found an effect near zero. To test it under optimized conditions, researchers used inhibitory-control conflict tasks: a one-hour Stroop task (high-inhibition = 75% incongruent trials; control = 0%) as the depleting manipulation, and a separate Simon task measuring the interference effect (incongruent minus congruent performance) as the self-control outcome, with a stabilized pre-test baseline. Study 1 (between-subjects, n=82) found a significant depletion effect; Study 2 (within-subjects, n=52) did not, so the effect's robustness remains uncertain. — Question: The "interference effect" on the Simon task—the difference in performance between incongruent and congruent trials—was used as a measure of:
- **A:** long-term memory capacity
- **B:** inhibitory (self-)control
- **C:** visual acuity
- **D:** verbal fluency
- **Correct:** B
- **Explanation:** The passage frames self-control as resting on inhibitory control and uses the conflict-driven interference effect as its index; (A), (C), and (D) name unrelated abilities the tasks were not designed to assess.
- **Tags:** `KC::PsychSoc::Cognition` `MCAT::Psych_Soc` `Difficulty::2` `IRT::Discrimination::0.9` `IRT::Guessing::0.25` `Reasoning::Comprehension`

### MCAT-PASSAGE-PS-002

- **KC:** `PsychSoc::Cognition`
- **Prereqs:** none
- **Difficulty:** 3
- **Question:** Passage (Source: Radel R, Gruet M, Barzykowski K (2019), "Testing the ego-depletion effect in optimized conditions," PLOS ONE 14(3):e0213026, CC BY, https://doi.org/10.1371/journal.pone.0213026): Ego-depletion theory proposes self-control draws on a limited resource, so exerting it on one task leaves less for the next; early support later looked inflated by publication bias, and a large pre-registered replication found an effect near zero. To test it under optimized conditions, researchers used inhibitory-control conflict tasks: a one-hour Stroop task (high-inhibition = 75% incongruent trials; control = 0%) as the depleting manipulation, and a separate Simon task measuring the interference effect (incongruent minus congruent performance) as the self-control outcome, with a stabilized pre-test baseline. Study 1 (between-subjects, n=82) found a significant depletion effect; Study 2 (within-subjects, n=52) did not, so the effect's robustness remains uncertain. — Question: Why did the researchers include a pre-test on the Simon task and train participants until performance stabilized?
- **A:** To deplete participants' self-control before the main task
- **B:** To establish a baseline so post-manipulation changes could be attributed to the Stroop manipulation rather than to pre-existing individual differences
- **C:** To ensure participants would fail the post-test
- **D:** To measure the Stroop effect directly
- **Correct:** B
- **Explanation:** The baseline meant any change after the Stroop manipulation could be attributed to the manipulation rather than to pre-existing differences; (A) misidentifies the Simon pre-test as the depleting task, (C) is not a real design goal, and (D) confuses the Stroop manipulation with the Simon outcome measure.
- **Tags:** `KC::PsychSoc::Cognition` `MCAT::Psych_Soc` `Difficulty::3` `IRT::Discrimination::1.0` `IRT::Guessing::0.25` `Reasoning::Inference`

### MCAT-PASSAGE-PS-003

- **KC:** `PsychSoc::Cognition`
- **Prereqs:** none
- **Difficulty:** 2
- **Question:** Passage (Source: Radel R, Gruet M, Barzykowski K (2019), "Testing the ego-depletion effect in optimized conditions," PLOS ONE 14(3):e0213026, CC BY, https://doi.org/10.1371/journal.pone.0213026): Ego-depletion theory proposes self-control draws on a limited resource, so exerting it on one task leaves less for the next; early support later looked inflated by publication bias, and a large pre-registered replication found an effect near zero. To test it under optimized conditions, researchers used inhibitory-control conflict tasks: a one-hour Stroop task (high-inhibition = 75% incongruent trials; control = 0%) as the depleting manipulation, and a separate Simon task measuring the interference effect (incongruent minus congruent performance) as the self-control outcome, with a stabilized pre-test baseline. Study 1 (between-subjects, n=82) found a significant depletion effect; Study 2 (within-subjects, n=52) did not, so the effect's robustness remains uncertain. — Question: The high-inhibition and control conditions differed primarily in:
- **A:** the total duration of the Stroop task
- **B:** the proportion of incongruent trials (75% vs. 0%), i.e., the amount of inhibition required
- **C:** whether participants completed a Simon task at all
- **D:** the number of participants assigned to each
- **Correct:** B
- **Explanation:** Both conditions ran the Stroop for the same hour and differed in incongruent-trial proportion (75% vs. 0%), which sets the inhibition demand; duration was constant (A), both did the Simon task (C), and sample size (D) was not the manipulated variable.
- **Tags:** `KC::PsychSoc::Cognition` `MCAT::Psych_Soc` `Difficulty::2` `IRT::Discrimination::0.9` `IRT::Guessing::0.25` `Reasoning::Data`

### MCAT-PASSAGE-PS-004

- **KC:** `PsychSoc::Cognition`
- **Prereqs:** none
- **Difficulty:** 3
- **Question:** Passage (Source: Radel R, Gruet M, Barzykowski K (2019), "Testing the ego-depletion effect in optimized conditions," PLOS ONE 14(3):e0213026, CC BY, https://doi.org/10.1371/journal.pone.0213026): Ego-depletion theory proposes self-control draws on a limited resource, so exerting it on one task leaves less for the next; early support later looked inflated by publication bias, and a large pre-registered replication found an effect near zero. To test it under optimized conditions, researchers used inhibitory-control conflict tasks: a one-hour Stroop task (high-inhibition = 75% incongruent trials; control = 0%) as the depleting manipulation, and a separate Simon task measuring the interference effect (incongruent minus congruent performance) as the self-control outcome, with a stabilized pre-test baseline. Study 1 (between-subjects, n=82) found a significant depletion effect; Study 2 (within-subjects, n=52) did not, so the effect's robustness remains uncertain. — Question: Given that Study 1 (between-subjects) found a significant effect but Study 2 (within-subjects) did not, the most scientifically appropriate conclusion is that:
- **A:** the ego-depletion effect is definitively proven to be real
- **B:** the ego-depletion effect is definitively proven not to exist
- **C:** the evidence is mixed, so confidence in a robust effect is not warranted without further replication
- **D:** within-subjects designs are always invalid
- **Correct:** C
- **Explanation:** Conflicting results across designs justify caution rather than a firm verdict; (A) and (B) overclaim in opposite directions from mixed data, and (D) is an unjustified blanket dismissal of a standard design.
- **Tags:** `KC::PsychSoc::Cognition` `MCAT::Psych_Soc` `Difficulty::3` `IRT::Discrimination::1.1` `IRT::Guessing::0.25` `Reasoning::Inference`

### MCAT-PASSAGE-PS-005

- **KC:** `PsychSoc::Cognition`
- **Prereqs:** none
- **Difficulty:** 2
- **Question:** Passage (Source: Radel R, Gruet M, Barzykowski K (2019), "Testing the ego-depletion effect in optimized conditions," PLOS ONE 14(3):e0213026, CC BY, https://doi.org/10.1371/journal.pone.0213026): Ego-depletion theory proposes self-control draws on a limited resource, so exerting it on one task leaves less for the next; early support later looked inflated by publication bias, and a large pre-registered replication found an effect near zero. Meta-analyses suggested earlier support for ego depletion was inflated because positive findings are more likely to be published than null ones. To test it under optimized conditions, researchers used a one-hour Stroop task (high-inhibition = 75% incongruent trials; control = 0%) as the depleting manipulation and a separate Simon task measuring the interference effect as the self-control outcome, with a stabilized pre-test baseline. Study 1 found a significant effect; Study 2 did not, so robustness remains uncertain. — Question: The passage notes that meta-analyses suggested earlier support for ego depletion was inflated by "publication bias." Publication bias refers to the tendency for:
- **A:** researchers to fabricate data
- **B:** journals and authors to publish positive/significant findings more readily than null findings
- **C:** participants to guess a study's hypothesis
- **D:** effects to shrink simply because samples are large
- **Correct:** B
- **Explanation:** Publication bias is the preferential publication of significant over null results, skewing the visible literature; (A) describes fraud, (C) describes demand characteristics, and (D) misstates statistical power.
- **Tags:** `KC::PsychSoc::Cognition` `MCAT::Psych_Soc` `Difficulty::2` `IRT::Discrimination::0.8` `IRT::Guessing::0.25` `Reasoning::Comprehension`

### MCAT-PASSAGE-PS-006

- **KC:** `PsychSoc::Cognition`
- **Prereqs:** none
- **Difficulty:** 2
- **Question:** Passage (Source: Gajdos T, Régner I, Huguet P, Hainguerlot M, Vergnaud J-C, Sackur J, et al. (2019), "Does social context impact metacognition? Evidence from stereotype threat in a visual search task," PLOS ONE 14(4):e0215050, CC BY, https://doi.org/10.1371/journal.pone.0215050): Metacognition is the monitoring of one's own mental processes, such as confidence in a decision. Most stereotype-threat research shows impaired performance, but this study asked whether the same social context changes metacognition. 125 science students did a visual-search task, reporting a target's color, then judging their confidence and how many items they had scanned. Half were placed under stereotype threat by framing the task as visuo-spatial ability with a gender difference; others were told there was no difference. Researchers measured confidence calibration (confident when correct, unsure when wrong) and, using a computational model to estimate items actually scanned, the accuracy of the scan reports. Threat enhanced both measures of metacognitive monitoring. — Question: As used in the passage, "metacognition" is best defined as:
- **A:** the speed at which a person can search a visual display
- **B:** the monitoring and evaluation of one's own cognitive processes, such as confidence in a decision
- **C:** the ability to memorize long lists of items
- **D:** an automatic physiological stress response
- **Correct:** B
- **Explanation:** The passage defines metacognition as the monitoring of one's own mental processes, including confidence judgments; (A) describes task performance, (C) describes memory capacity, and (D) describes a stress response, not the definition.
- **Tags:** `KC::PsychSoc::Cognition` `MCAT::Psych_Soc` `Difficulty::2` `IRT::Discrimination::0.8` `IRT::Guessing::0.25` `Reasoning::Comprehension`

### MCAT-PASSAGE-PS-007

- **KC:** `PsychSoc::Cognition`
- **Prereqs:** none
- **Difficulty:** 4
- **Question:** Passage (Source: Gajdos T, Régner I, Huguet P, Hainguerlot M, Vergnaud J-C, Sackur J, et al. (2019), "Does social context impact metacognition? Evidence from stereotype threat in a visual search task," PLOS ONE 14(4):e0215050, CC BY, https://doi.org/10.1371/journal.pone.0215050): Metacognition is the monitoring of one's own mental processes, such as confidence in a decision. Most stereotype-threat research shows impaired performance, but this study asked whether the same social context changes metacognition. 125 science students did a visual-search task, reporting a target's color, then judging their confidence and how many items they had scanned. Half were placed under stereotype threat by framing the task as visuo-spatial ability with a gender difference; others were told there was no difference. Researchers measured confidence calibration (confident when correct, unsure when wrong) and, using a computational model to estimate items actually scanned, the accuracy of the scan reports. Threat enhanced both measures of metacognitive monitoring. — Question: Why did the researchers use a computational model of visual search?
- **A:** To replace the participants' color judgments
- **B:** To estimate the number of items actually scanned, which cannot be directly observed, for comparison with each participant's self-report
- **C:** To increase the difficulty of the search task
- **D:** To measure participants' confidence directly
- **Correct:** B
- **Explanation:** The model was needed because the actual number of items scanned cannot be observed directly, providing a benchmark for the self-reported number; the model makes no color judgments (A), was not used to raise difficulty (C), and is separate from the confidence/calibration measure (D).
- **Tags:** `KC::PsychSoc::Cognition` `MCAT::Psych_Soc` `Difficulty::4` `IRT::Discrimination::1.1` `IRT::Guessing::0.25` `Reasoning::Inference`

### MCAT-PASSAGE-PS-008

- **KC:** `PsychSoc::Cognition`
- **Prereqs:** none
- **Difficulty:** 3
- **Question:** Passage (Source: Gajdos T, Régner I, Huguet P, Hainguerlot M, Vergnaud J-C, Sackur J, et al. (2019), "Does social context impact metacognition? Evidence from stereotype threat in a visual search task," PLOS ONE 14(4):e0215050, CC BY, https://doi.org/10.1371/journal.pone.0215050): Metacognition is the monitoring of one's own mental processes, such as confidence in a decision. Most stereotype-threat research shows impaired performance, but this study asked whether the same social context changes metacognition. 125 science students did a visual-search task, reporting a target's color, then judging their confidence and how many items they had scanned. Half were placed under stereotype threat by framing the task as visuo-spatial ability with a gender difference; others were told there was no difference. Researchers measured confidence calibration (good metacognition means being confident when correct and unsure when wrong) and, using a computational model to estimate items actually scanned, the accuracy of the scan reports. Threat enhanced both measures of metacognitive monitoring. — Question: The confidence-based calibration measure reflects good metacognition when a participant is:
- **A:** always highly confident regardless of accuracy
- **B:** confident when their answers are correct and unsure when their answers are wrong
- **C:** never confident about any answer
- **D:** faster than the computational model predicts
- **Correct:** B
- **Explanation:** Good metacognition means confidence tracks accuracy—confident when correct and unsure when wrong; (A) and (C) describe uniform confidence that ignores accuracy, and (D) confuses calibration with the separate process/speed measure.
- **Tags:** `KC::PsychSoc::Cognition` `MCAT::Psych_Soc` `Difficulty::3` `IRT::Discrimination::1.0` `IRT::Guessing::0.25` `Reasoning::Comprehension`

## PsychSoc::Motivation

### MCAT-PASSAGE-PS-009

- **KC:** `PsychSoc::Motivation`
- **Prereqs:** none
- **Difficulty:** 3
- **Question:** Passage (Source: Radel R, Gruet M, Barzykowski K (2019), "Testing the ego-depletion effect in optimized conditions," PLOS ONE 14(3):e0213026, CC BY, https://doi.org/10.1371/journal.pone.0213026): Ego-depletion theory proposes self-control draws on a limited resource, so exerting it on one task leaves less for the next. Criticizing earlier tests that used very short "depleting" tasks and no baseline, the researchers set out to test the hypothesis under conditions optimized to reveal it if it exists. The depleting manipulation was a Stroop task performed continuously for a full hour (high-inhibition = 75% incongruent trials; control = 0%), while a separate Simon task measured self-control via the interference effect against a stabilized pre-test baseline. Study 1 (n=82) found a significant depletion effect; Study 2 (n=52) did not, leaving the effect's robustness uncertain. — Question: Why did the researchers deliberately use a *long* (one-hour) continuous depleting task rather than the very short tasks used in many earlier studies?
- **A:** To reduce the total cognitive demand on participants
- **B:** Because they judged that a longer, more demanding task gives a better chance of producing depletion if the effect is real
- **C:** Because the Simon task requires exactly one hour to administer
- **D:** To eliminate the need for a control condition
- **Correct:** B
- **Explanation:** They optimized conditions to reveal the effect if it exists, criticizing very short depleting tasks, so a long demanding task is more likely to induce depletion; (A) is the opposite of the intent, (C) misattributes the duration to the Simon task, and (D) is wrong because a control condition was central.
- **Tags:** `KC::PsychSoc::Motivation` `MCAT::Psych_Soc` `Difficulty::3` `IRT::Discrimination::1.0` `IRT::Guessing::0.25` `Reasoning::Inference`

## PsychSoc::Stereotypes

### MCAT-PASSAGE-PS-010

- **KC:** `PsychSoc::Stereotypes`
- **Prereqs:** none
- **Difficulty:** 3
- **Question:** Passage (Source: Inglis M, O'Hagan S (2022), "Stereotype threat, gender and mathematics attainment: A conceptual replication of Stricker & Ward," PLOS ONE 17(5):e0267699, CC BY, https://doi.org/10.1371/journal.pone.0267699): Stereotype threat is the apprehension of confirming a negative stereotype about one's group; the worry is thought to consume attention and working memory, so reminding women of a gender-math stereotype can lower test performance. An earlier analysis suggested that asking test-takers' gender after rather than before a math test could raise women's scores, potentially giving thousands more women credit each year—a cheap remedy for the gender gap. Researchers ran a pre-registered conceptual replication in a national math competition, randomly assigning students to report gender before or after the problems. The result was null: question placement did not affect performance, weakening the case for adopting the intervention as policy. — Question: According to the passage, the proposed mechanism by which stereotype threat harms performance is that the worry about confirming the stereotype:
- **A:** improves motivation and effort
- **B:** consumes attention and working memory needed for the task
- **C:** permanently lowers intelligence
- **D:** changes the objective difficulty of the test
- **Correct:** B
- **Explanation:** The passage says the worry is thought to consume attention and working memory; (A) is the opposite of the claimed harm, (C) overstates a transient situational effect as permanent, and (D) confuses a psychological state with the test's actual difficulty.
- **Tags:** `KC::PsychSoc::Stereotypes` `MCAT::Psych_Soc` `Difficulty::3` `IRT::Discrimination::1.0` `IRT::Guessing::0.25` `Reasoning::Comprehension`

### MCAT-PASSAGE-PS-011

- **KC:** `PsychSoc::Stereotypes`
- **Prereqs:** none
- **Difficulty:** 4
- **Question:** Passage (Source: Inglis M, O'Hagan S (2022), "Stereotype threat, gender and mathematics attainment: A conceptual replication of Stricker & Ward," PLOS ONE 17(5):e0267699, CC BY, https://doi.org/10.1371/journal.pone.0267699): Stereotype threat is the apprehension of confirming a negative stereotype about one's group; the worry is thought to consume attention and working memory, so reminding women of a gender-math stereotype can lower test performance. An earlier analysis suggested moving the gender question to the end of a math test could raise women's scores. Because claims of large effects from tiny changes deserve scrutiny, researchers ran a pre-registered conceptual replication—publicly committing to hypotheses, methods, and analysis plan in advance to guard against after-the-fact reinterpretation—randomly assigning students to report gender before or after the problems. The result was null: question placement did not affect performance, weakening the case for the intervention. — Question: Why is pre-registration highlighted as a strength of this study?
- **A:** It guarantees the hypothesis will be supported
- **B:** It commits the researchers to hypotheses and analyses in advance, guarding against selectively reporting analyses that happen to be significant
- **C:** It increases the number of participants
- **D:** It replaces the need for a control comparison
- **Correct:** B
- **Explanation:** Pre-registration protects against searching through many analyses until a significant result appears; it does not favor any outcome (A), is unrelated to sample size (C), and does not remove the need for comparison groups (D).
- **Tags:** `KC::PsychSoc::Stereotypes` `MCAT::Psych_Soc` `Difficulty::4` `IRT::Discrimination::1.2` `IRT::Guessing::0.25` `Reasoning::Application`

### MCAT-PASSAGE-PS-012

- **KC:** `PsychSoc::Stereotypes`
- **Prereqs:** none
- **Difficulty:** 4
- **Question:** Passage (Source: Inglis M, O'Hagan S (2022), "Stereotype threat, gender and mathematics attainment: A conceptual replication of Stricker & Ward," PLOS ONE 17(5):e0267699, CC BY, https://doi.org/10.1371/journal.pone.0267699): Stereotype threat is the apprehension of confirming a negative stereotype about one's group, and reminding women of a gender-math stereotype can lower test performance. An earlier analysis suggested moving the gender question to the end of a math test could raise women's scores. Researchers ran a pre-registered conceptual replication—testing the same underlying idea in a new setting rather than copying the original study exactly—in a national math competition, randomly assigning students to report gender before or after the problems. The result was null. Conceptual replication tests whether a claimed effect generalizes beyond the exact conditions of the original study, which is what matters if the effect is to guide real decisions. — Question: A "conceptual" replication (as opposed to a direct/exact replication) is valuable primarily because it tests whether:
- **A:** the original researchers were honest
- **B:** an effect generalizes beyond the exact conditions of the original study
- **C:** the same participants respond the same way twice
- **D:** a larger sample can be recruited
- **Correct:** B
- **Explanation:** Conceptual replication tests whether a claimed effect generalizes beyond the exact conditions of the original study; (A) is not its purpose, (C) describes test–retest reliability, and (D) is incidental.
- **Tags:** `KC::PsychSoc::Stereotypes` `MCAT::Psych_Soc` `Difficulty::4` `IRT::Discrimination::1.1` `IRT::Guessing::0.25` `Reasoning::Application`

### MCAT-PASSAGE-PS-013

- **KC:** `PsychSoc::Stereotypes`
- **Prereqs:** none
- **Difficulty:** 4
- **Question:** Passage (Source: Gajdos T, Régner I, Huguet P, Hainguerlot M, Vergnaud J-C, Sackur J, et al. (2019), "Does social context impact metacognition? Evidence from stereotype threat in a visual search task," PLOS ONE 14(4):e0215050, CC BY, https://doi.org/10.1371/journal.pone.0215050): Stereotype threat is the apprehension of confirming a negative stereotype about one's group, and most research shows it impairs performance by taxing attention and working memory. This study asked whether it also changes metacognition—the monitoring of one's own decisions. 125 students did a visual-search task under threat (task framed as visuo-spatial ability with a gender difference) or no threat. Researchers measured confidence calibration and, via a computational model, the accuracy of participants' reports of how many items they scanned. Contrary to a deficits-only view, threat enhanced metacognitive monitoring of both outcome and process, correcting the assumption that stereotype threat produces only deficits. — Question: The study's central finding most directly challenges which assumption?
- **A:** That stereotype threat can only impair, and never enhance, aspects of cognition
- **B:** That confidence judgments can be measured
- **C:** That visual search depends on the number of distractors
- **D:** That men and women can perform the same task
- **Correct:** A
- **Explanation:** By showing threat enhanced metacognitive monitoring, the study is a corrective to the assumption that stereotype threat produces only deficits; (B), (C), and (D) are background facts the study does not contest.
- **Tags:** `KC::PsychSoc::Stereotypes` `MCAT::Psych_Soc` `Difficulty::4` `IRT::Discrimination::1.2` `IRT::Guessing::0.25` `Reasoning::Inference`

## PsychSoc::Stratification

### MCAT-PASSAGE-PS-014

- **KC:** `PsychSoc::Stratification`
- **Prereqs:** none
- **Difficulty:** 3
- **Question:** Passage (Source: Inglis M, O'Hagan S (2022), "Stereotype threat, gender and mathematics attainment: A conceptual replication of Stricker & Ward," PLOS ONE 17(5):e0267699, CC BY, https://doi.org/10.1371/journal.pone.0267699): Gender gaps in advanced mathematics attainment are a persistent form of group inequality. Stereotype threat—the apprehension of confirming a negative stereotype about one's group—may widen such gaps, and an earlier analysis suggested that asking test-takers' gender after rather than before a math test could raise women's scores, potentially giving thousands more women credit each year. Researchers ran a pre-registered conceptual replication in a national math competition, randomly assigning students to report their gender before or after the mathematics problems and comparing scores. The result was null: question placement did not affect performance, so this cheap structural remedy is unlikely to narrow the gender gap. — Question: What was the independent variable manipulated in the replication study?
- **A:** The difficulty of the mathematics problems
- **B:** Whether demographic (gender) questions were placed before or after the test
- **C:** The gender of the participants
- **D:** Whether participants were told their scores
- **Correct:** B
- **Explanation:** Participants were randomly assigned to answer demographic questions either before or after the mathematics problems, which is the manipulated variable; (A) was not manipulated, (C) gender is a measured characteristic, and (D) is not part of the design.
- **Tags:** `KC::PsychSoc::Stratification` `MCAT::Psych_Soc` `Difficulty::3` `IRT::Discrimination::1.0` `IRT::Guessing::0.25` `Reasoning::Data`

### MCAT-PASSAGE-PS-015

- **KC:** `PsychSoc::Stratification`
- **Prereqs:** none
- **Difficulty:** 3
- **Question:** Passage (Source: Gajdos T, Régner I, Huguet P, Hainguerlot M, Vergnaud J-C, Sackur J, et al. (2019), "Does social context impact metacognition? Evidence from stereotype threat in a visual search task," PLOS ONE 14(4):e0215050, CC BY, https://doi.org/10.1371/journal.pone.0215050): Some skill domains carry culturally shared beliefs that rank social groups by ability—for example, a negative stereotype about women in visuo-spatial or geometric ability. To study how such group-based expectations operate, researchers had 125 science students perform a visual-search task. Roughly half (balanced across men and women) were placed under stereotype threat by framing the task as visuo-spatial ability and telling them previous studies had found a difference between men and women; the others were told previous studies had found no such difference. The researchers then compared confidence calibration and the accuracy of participants' reports about their own search process across conditions. — Question: How did the stereotype-threat and no-threat conditions differ?
- **A:** Only the threat group performed the visual search task
- **B:** The threat group was told previous studies found a gender difference on this type of task, while the no-threat group was told previous studies found no such difference
- **C:** The threat group received easier trials
- **D:** The no-threat group did not report confidence
- **Correct:** B
- **Explanation:** The manipulation framed the task as gender-stereotyped and told the threat group that previous studies had found a difference between men and women, versus no such difference for the others; both groups did the task (A), and task difficulty and measures were held constant (C, D).
- **Tags:** `KC::PsychSoc::Stratification` `MCAT::Psych_Soc` `Difficulty::3` `IRT::Discrimination::0.9` `IRT::Guessing::0.25` `Reasoning::Comprehension`

## PsychSoc::Social_Inequality

### MCAT-PASSAGE-PS-016

- **KC:** `PsychSoc::Social_Inequality`
- **Prereqs:** none
- **Difficulty:** 4
- **Question:** Passage (Source: Inglis M, O'Hagan S (2022), "Stereotype threat, gender and mathematics attainment: A conceptual replication of Stricker & Ward," PLOS ONE 17(5):e0267699, CC BY, https://doi.org/10.1371/journal.pone.0267699): Gender gaps in mathematics attainment are a widely studied form of social inequality. An earlier analysis proposed a cheap remedy: moving a demographic gender question to the end of a math test, estimated to let thousands more women earn credit each year. Researchers ran a pre-registered conceptual replication in a national math competition, randomly assigning students to report gender before or after the problems. The result was null—question placement did not affect performance. A failure to replicate does not prove stereotype threat never operates, but it shows this specific, widely publicized intervention did not deliver the promised benefit, cautioning against building policy on untested interventions. — Question: Which is the most defensible interpretation of the study's null result?
- **A:** Stereotype threat has been proven not to exist
- **B:** This particular intervention (moving the demographic question) did not produce the promised benefit, though it does not disprove stereotype threat in general
- **C:** The researchers must have made an error, since the earlier analysis found an effect
- **D:** Gender gaps in mathematics are entirely explained by stereotype threat
- **Correct:** B
- **Explanation:** Failure to replicate does not by itself prove that stereotype threat never operates, but it does undercut this specific intervention; (A) overgeneralizes, (C) assumes error rather than legitimate replication differences, and (D) is unsupported and contradicts the null finding.
- **Tags:** `KC::PsychSoc::Social_Inequality` `MCAT::Psych_Soc` `Difficulty::4` `IRT::Discrimination::1.2` `IRT::Guessing::0.25` `Reasoning::Inference`
