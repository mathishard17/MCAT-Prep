# MCAT Psychology / Sociology Generated Cards

Synthetic, original multiple-choice cards for the Concept Scheduler — one card
per Psychology/Sociology (`PsychSoc::`) Knowledge Component. Block format is
copied exactly from `added features/mcat_demo_cards.md` so these import through
the same parser (`rslib/src/scheduler/concept_demo.rs`).

- **Coverage:** all 34 `PsychSoc::` KCs from `added features/research-kc-psych-soc.md`
  / `added features/kc-map-unified.md` (§6, Psychology/Sociology table).
- **IDs:** `MCAT-PSY-<TOPIC>-NNN`, where `<TOPIC>` is the stable per-KC code from
  the unified map's `kc_code` column (`PSY-<TOPIC>`); unique per KC.
- **Section:** primary `MCAT::Psych_Soc` for every KC (the unified map lists
  `Psych_Soc` as the primary section for all 34).
- **Prereqs:** taken verbatim from the unified map §6 "Prerequisites (canonical
  ids)" column (which includes the soft/`(verify)` cross-discipline `Bio::` edges
  into `Biological_and_Social_Factors` and `Stress`). Edge direction is
  prerequisite -> target.
- **Difficulty:** chosen within each KC's difficulty band from the unified map.
- **Synthetic/original.** No copyrighted prep text is reproduced.

## Tags

- Section tag: `MCAT::Psych_Soc`
- KC tag: `KC::PsychSoc::<Topic>`
- Prerequisite tag: `Prereq::<Area>::<Topic>`
- Difficulty tag: `Difficulty::<1-5>`

---

## PsychSoc::Biological_and_Social_Factors

### MCAT-PSY-BIOSF-001

- **KC:** `PsychSoc::Biological_and_Social_Factors`
- **Prereqs:** `Prereq::Bio::Endocrine_System` `Prereq::Bio::Genetics` `Prereq::Bio::Nervous_System`
- **Difficulty:** 2
- **Question:** Researchers compare identical and fraternal twins to estimate how much genetic variation contributes to a behavioral trait. This approach best reflects which principle about the origins of behavior?
- **A:** Behavior is determined solely by environmental learning
- **B:** Behavior arises from an interaction of genetic and environmental factors
- **C:** Behavior is fixed entirely by genes present at conception
- **D:** Behavior cannot be investigated with scientific methods
- **Correct:** B
- **Explanation:** The biopsychosocial view holds that behavior emerges from interacting biological (genetic, neural, hormonal) and social/environmental influences, and twin designs estimate the relative genetic contribution to that mix.
- **Tags:** `KC::PsychSoc::Biological_and_Social_Factors` `Prereq::Bio::Endocrine_System` `Prereq::Bio::Genetics` `Prereq::Bio::Nervous_System` `MCAT::Psych_Soc` `Difficulty::2`

### MCAT-PSY-BIOSF-002

- **KC:** `PsychSoc::Biological_and_Social_Factors`
- **Prereqs:** `Prereq::Bio::Endocrine_System` `Prereq::Bio::Genetics` `Prereq::Bio::Nervous_System`
- **Difficulty:** 1
- **Question:** Which branch of the nervous system is chiefly responsible for the rapid "fight-or-flight" physiological changes a person experiences when suddenly frightened?
- **A:** The sympathetic nervous system
- **B:** The parasympathetic nervous system
- **C:** The somatic sensory system
- **D:** The enteric nervous system acting alone
- **Correct:** A
- **Explanation:** The sympathetic branch of the autonomic nervous system mobilizes the body for action during arousal, raising heart rate, respiration, and glucose availability.
- **Tags:** `KC::PsychSoc::Biological_and_Social_Factors` `Prereq::Bio::Endocrine_System` `Prereq::Bio::Genetics` `Prereq::Bio::Nervous_System` `MCAT::Psych_Soc` `Difficulty::1`

### MCAT-PSY-BIOSF-003

- **KC:** `PsychSoc::Biological_and_Social_Factors`
- **Prereqs:** `Prereq::Bio::Endocrine_System` `Prereq::Bio::Genetics` `Prereq::Bio::Nervous_System`
- **Difficulty:** 3
- **Question:** Identical twins raised apart sometimes differ in whether a heritable illness actually appears, even though they share the same genome. This observation best illustrates what?
- **A:** That genes play no role in the illness
- **B:** Gene-environment interaction, in which environmental factors modulate genetic expression
- **C:** That the twins must not truly be genetically identical
- **D:** Purely environmental determinism of the illness
- **Correct:** B
- **Explanation:** Differences between genetically identical twins reveal how environmental factors interact with and can alter the expression of shared genetic predispositions.
- **Tags:** `KC::PsychSoc::Biological_and_Social_Factors` `Prereq::Bio::Endocrine_System` `Prereq::Bio::Genetics` `Prereq::Bio::Nervous_System` `MCAT::Psych_Soc` `Difficulty::3`

## PsychSoc::Sensory_Processing

### MCAT-PSY-SENP-001

- **KC:** `PsychSoc::Sensory_Processing`
- **Prereqs:** `Prereq::Bio::Nervous_System`
- **Difficulty:** 2
- **Question:** The process by which sensory receptors convert physical stimulus energy into electrical signals the nervous system can interpret is called what?
- **A:** Transduction
- **B:** Accommodation
- **C:** Habituation
- **D:** Assimilation
- **Correct:** A
- **Explanation:** Transduction is the conversion of external stimulus energy, such as light or sound, into action potentials that neurons transmit toward the brain.
- **Tags:** `KC::PsychSoc::Sensory_Processing` `Prereq::Bio::Nervous_System` `MCAT::Psych_Soc` `Difficulty::2`

### MCAT-PSY-SENP-002

- **KC:** `PsychSoc::Sensory_Processing`
- **Prereqs:** `Prereq::Bio::Nervous_System`
- **Difficulty:** 1
- **Question:** The minimum intensity of a stimulus that an observer can detect on 50 percent of trials is known as what?
- **A:** The difference threshold
- **B:** The absolute threshold
- **C:** The receptive field
- **D:** The refractory period
- **Correct:** B
- **Explanation:** The absolute threshold is the smallest amount of stimulus energy that can be detected half of the time.
- **Tags:** `KC::PsychSoc::Sensory_Processing` `Prereq::Bio::Nervous_System` `MCAT::Psych_Soc` `Difficulty::1`

### MCAT-PSY-SENP-003

- **KC:** `PsychSoc::Sensory_Processing`
- **Prereqs:** `Prereq::Bio::Nervous_System`
- **Difficulty:** 3
- **Question:** A radar operator must decide whether a faint blip is a real target or background noise, and the rate of hits versus false alarms depends on both sensitivity and a decision criterion. This framework is best described by what?
- **A:** Signal detection theory
- **B:** Weber's law
- **C:** Sensory adaptation
- **D:** Place theory
- **Correct:** A
- **Explanation:** Signal detection theory separates true sensory sensitivity from the response criterion (bias) that governs detection decisions under uncertainty.
- **Tags:** `KC::PsychSoc::Sensory_Processing` `Prereq::Bio::Nervous_System` `MCAT::Psych_Soc` `Difficulty::3`

## PsychSoc::The_Senses

### MCAT-PSY-SEN-001

- **KC:** `PsychSoc::The_Senses`
- **Prereqs:** `Prereq::PsychSoc::Sensory_Processing`
- **Difficulty:** 2
- **Question:** A person moves from a brightly lit room into a dark theater and gradually becomes able to make out dim shapes. Which photoreceptors are chiefly responsible for this recovery of low-light vision?
- **A:** Cones, which specialize in color under bright light
- **B:** Rods, which are highly sensitive under low light
- **C:** Cochlear hair cells that respond to vibration
- **D:** Olfactory receptors in the nasal epithelium
- **Correct:** B
- **Explanation:** Rods are far more light-sensitive than cones and mediate vision in dim conditions, and their gradual sensitization underlies dark adaptation.
- **Tags:** `KC::PsychSoc::The_Senses` `Prereq::PsychSoc::Sensory_Processing` `MCAT::Psych_Soc` `Difficulty::2`

### MCAT-PSY-SEN-002

- **KC:** `PsychSoc::The_Senses`
- **Prereqs:** `Prereq::PsychSoc::Sensory_Processing`
- **Difficulty:** 1
- **Question:** Which fluid-filled structures of the inner ear detect rotational movements of the head and contribute to the sense of balance?
- **A:** The semicircular canals
- **B:** The cones of the retina
- **C:** The taste buds of the tongue
- **D:** The olfactory bulb
- **Correct:** A
- **Explanation:** The semicircular canals of the inner ear sense rotational acceleration of the head, a central part of the vestibular sense of balance.
- **Tags:** `KC::PsychSoc::The_Senses` `Prereq::PsychSoc::Sensory_Processing` `MCAT::Psych_Soc` `Difficulty::1`

### MCAT-PSY-SEN-003

- **KC:** `PsychSoc::The_Senses`
- **Prereqs:** `Prereq::PsychSoc::Sensory_Processing`
- **Difficulty:** 3
- **Question:** According to the place theory of hearing, how does the cochlea primarily encode the pitch of a high-frequency tone?
- **A:** By the specific location along the basilar membrane where hair cells are maximally displaced
- **B:** By the total number of hair cells that are destroyed
- **C:** By the wavelength of light entering the ear
- **D:** By the concentration of odorant molecules present
- **Correct:** A
- **Explanation:** Place theory holds that different frequencies maximally stimulate hair cells at different locations along the basilar membrane, so pitch is coded by which location responds most.
- **Tags:** `KC::PsychSoc::The_Senses` `Prereq::PsychSoc::Sensory_Processing` `MCAT::Psych_Soc` `Difficulty::3`

## PsychSoc::Attention

### MCAT-PSY-ATT-001

- **KC:** `PsychSoc::Attention`
- **Prereqs:** `Prereq::PsychSoc::Sensory_Processing`
- **Difficulty:** 3
- **Question:** At a noisy party a student focuses on one friend's voice yet still notices when someone across the room says the student's name. This observation best illustrates what?
- **A:** Selective attention that still monitors unattended channels
- **B:** A complete failure of divided attention
- **C:** The decay of sensory (iconic) memory
- **D:** Total inattentional blindness
- **Correct:** A
- **Explanation:** Selective attention filters input toward one channel, but personally significant stimuli such as one's own name can still break through from unattended channels, the classic cocktail-party effect.
- **Tags:** `KC::PsychSoc::Attention` `Prereq::PsychSoc::Sensory_Processing` `MCAT::Psych_Soc` `Difficulty::3`

### MCAT-PSY-ATT-002

- **KC:** `PsychSoc::Attention`
- **Prereqs:** `Prereq::PsychSoc::Sensory_Processing`
- **Difficulty:** 2
- **Question:** Observers asked to count basketball passes in a well-known demonstration often fail to notice a person in a gorilla suit stroll through the scene. This failure to perceive an unexpected but visible object is called what?
- **A:** Change blindness
- **B:** Inattentional blindness
- **C:** Sensory adaptation
- **D:** The McGurk effect
- **Correct:** B
- **Explanation:** Inattentional blindness is the failure to notice a fully visible, unexpected stimulus when attention is focused on another demanding task.
- **Tags:** `KC::PsychSoc::Attention` `Prereq::PsychSoc::Sensory_Processing` `MCAT::Psych_Soc` `Difficulty::2`

### MCAT-PSY-ATT-003

- **KC:** `PsychSoc::Attention`
- **Prereqs:** `Prereq::PsychSoc::Sensory_Processing`
- **Difficulty:** 4
- **Question:** A driver can comfortably steer while listening to the radio but begins to swerve when composing a text message. This drop in performance when two effortful tasks compete is best explained by what?
- **A:** The limited capacity of divided attention across controlled tasks
- **B:** Complete automaticity of both tasks at once
- **C:** A shift toward pure bottom-up processing
- **D:** Failure of sensory transduction in the eye
- **Correct:** A
- **Explanation:** Divided attention draws on limited controlled-processing resources, so performance suffers when two tasks each demand effortful attention rather than being automatic.
- **Tags:** `KC::PsychSoc::Attention` `Prereq::PsychSoc::Sensory_Processing` `MCAT::Psych_Soc` `Difficulty::4`

## PsychSoc::Perception

### MCAT-PSY-PER-001

- **KC:** `PsychSoc::Perception`
- **Prereqs:** `Prereq::PsychSoc::Attention` `Prereq::PsychSoc::The_Senses`
- **Difficulty:** 3
- **Question:** A reader easily interprets a sloppily written word by using the surrounding sentence. This reliance on prior knowledge and expectation to interpret sensory input is best described as what?
- **A:** Bottom-up processing
- **B:** Sensory transduction
- **C:** Top-down processing
- **D:** Absolute threshold detection
- **Correct:** C
- **Explanation:** Top-down processing uses prior knowledge, context, and expectations to interpret ambiguous input, in contrast to bottom-up processing that builds perception from raw sensory features.
- **Tags:** `KC::PsychSoc::Perception` `Prereq::PsychSoc::Attention` `Prereq::PsychSoc::The_Senses` `MCAT::Psych_Soc` `Difficulty::3`

### MCAT-PSY-PER-002

- **KC:** `PsychSoc::Perception`
- **Prereqs:** `Prereq::PsychSoc::Attention` `Prereq::PsychSoc::The_Senses`
- **Difficulty:** 2
- **Question:** Scattered dots that all drift in the same direction at the same speed are spontaneously seen as one moving group. Which Gestalt principle of organization best explains this?
- **A:** Common fate
- **B:** Closure
- **C:** The phi phenomenon
- **D:** Retinal disparity
- **Correct:** A
- **Explanation:** The Gestalt principle of common fate groups together elements moving in the same direction and speed so that they are perceived as a single unit.
- **Tags:** `KC::PsychSoc::Perception` `Prereq::PsychSoc::Attention` `Prereq::PsychSoc::The_Senses` `MCAT::Psych_Soc` `Difficulty::2`

### MCAT-PSY-PER-003

- **KC:** `PsychSoc::Perception`
- **Prereqs:** `Prereq::PsychSoc::Attention` `Prereq::PsychSoc::The_Senses`
- **Difficulty:** 4
- **Question:** A person continues to perceive a door as rectangular even though its image on the retina becomes a trapezoid as it swings open. This stability of perceived form is best described as what?
- **A:** Shape constancy
- **B:** Retinal disparity
- **C:** The absolute threshold
- **D:** Sensory adaptation
- **Correct:** A
- **Explanation:** Shape constancy is the perceptual constancy by which an object is seen as keeping a stable shape even as its retinal image changes with viewing angle.
- **Tags:** `KC::PsychSoc::Perception` `Prereq::PsychSoc::Attention` `Prereq::PsychSoc::The_Senses` `MCAT::Psych_Soc` `Difficulty::4`

## PsychSoc::Cognition

### MCAT-PSY-COG-001

- **KC:** `PsychSoc::Cognition`
- **Prereqs:** `Prereq::PsychSoc::Attention` `Prereq::PsychSoc::Perception`
- **Difficulty:** 3
- **Question:** A person assumes a quiet, book-loving stranger is more likely a librarian than a salesperson, ignoring that salespeople are far more numerous. This judgment error best reflects which cognitive shortcut?
- **A:** The representativeness heuristic
- **B:** Functional fixedness
- **C:** Careful base-rate calculation
- **D:** Algorithmic problem solving
- **Correct:** A
- **Explanation:** The representativeness heuristic judges likelihood by resemblance to a prototype while neglecting base rates, producing systematic errors in probability estimates.
- **Tags:** `KC::PsychSoc::Cognition` `Prereq::PsychSoc::Attention` `Prereq::PsychSoc::Perception` `MCAT::Psych_Soc` `Difficulty::3`

### MCAT-PSY-COG-002

- **KC:** `PsychSoc::Cognition`
- **Prereqs:** `Prereq::PsychSoc::Attention` `Prereq::PsychSoc::Perception`
- **Difficulty:** 2
- **Question:** After watching several vivid news reports about shark attacks, a swimmer greatly overestimates how likely such an attack is. This misjudgment based on how easily examples come to mind reflects which shortcut?
- **A:** The availability heuristic
- **B:** The representativeness heuristic
- **C:** Formal deductive reasoning
- **D:** Functional fixedness
- **Correct:** A
- **Explanation:** The availability heuristic estimates probability by how readily instances come to mind, so vivid or recent events inflate perceived frequency.
- **Tags:** `KC::PsychSoc::Cognition` `Prereq::PsychSoc::Attention` `Prereq::PsychSoc::Perception` `MCAT::Psych_Soc` `Difficulty::2`

### MCAT-PSY-COG-003

- **KC:** `PsychSoc::Cognition`
- **Prereqs:** `Prereq::PsychSoc::Attention` `Prereq::PsychSoc::Perception`
- **Difficulty:** 4
- **Question:** Needing to drive a nail, a person cannot solve the problem because they think of a heavy wrench only as a tool for bolts rather than as a possible hammer. This obstacle is best labeled what?
- **A:** Functional fixedness
- **B:** Confirmation bias
- **C:** The framing effect
- **D:** Belief perseverance
- **Correct:** A
- **Explanation:** Functional fixedness is the tendency to see objects only in their customary use, which blocks the novel uses sometimes needed to solve a problem.
- **Tags:** `KC::PsychSoc::Cognition` `Prereq::PsychSoc::Attention` `Prereq::PsychSoc::Perception` `MCAT::Psych_Soc` `Difficulty::4`

## PsychSoc::Memory

### MCAT-PSY-MEM-001

- **KC:** `PsychSoc::Memory`
- **Prereqs:** `Prereq::PsychSoc::Cognition`
- **Difficulty:** 3
- **Question:** A student recalls the digit string 1-4-9-2-1-7-7-6 more easily by grouping it into two memorable years. This strategy improves short-term retention primarily through what mechanism?
- **A:** Expanding the absolute capacity of sensory memory
- **B:** Chunking, which combines items into fewer meaningful units
- **C:** Preventing all decay of long-term memory
- **D:** Removing the need for retrieval cues
- **Correct:** B
- **Explanation:** Chunking groups individual items into larger meaningful units, effectively increasing how much information fits within the limited capacity of short-term/working memory.
- **Tags:** `KC::PsychSoc::Memory` `Prereq::PsychSoc::Cognition` `MCAT::Psych_Soc` `Difficulty::3`

### MCAT-PSY-MEM-002

- **KC:** `PsychSoc::Memory`
- **Prereqs:** `Prereq::PsychSoc::Cognition`
- **Difficulty:** 2
- **Question:** Given a long list of words to recall freely, people tend to remember the first and last items better than those in the middle. This pattern is known as what?
- **A:** The serial position effect
- **B:** The spacing effect
- **C:** Source amnesia
- **D:** Proactive interference
- **Correct:** A
- **Explanation:** The serial position effect combines the primacy effect (better recall of early items) and the recency effect (better recall of late items), leaving middle items weakest.
- **Tags:** `KC::PsychSoc::Memory` `Prereq::PsychSoc::Cognition` `MCAT::Psych_Soc` `Difficulty::2`

### MCAT-PSY-MEM-003

- **KC:** `PsychSoc::Memory`
- **Prereqs:** `Prereq::PsychSoc::Cognition`
- **Difficulty:** 4
- **Question:** A student who learned French years ago now struggles to learn Spanish because the older French vocabulary keeps intruding. This disruption of new learning by old memories is called what?
- **A:** Retroactive interference
- **B:** Proactive interference
- **C:** The recency effect
- **D:** Encoding failure
- **Correct:** B
- **Explanation:** Proactive interference occurs when previously learned information interferes with the acquisition or retrieval of newly learned material.
- **Tags:** `KC::PsychSoc::Memory` `Prereq::PsychSoc::Cognition` `MCAT::Psych_Soc` `Difficulty::4`

## PsychSoc::Consciousness

### MCAT-PSY-CON-001

- **KC:** `PsychSoc::Consciousness`
- **Prereqs:** `Prereq::PsychSoc::Attention` `Prereq::PsychSoc::Cognition`
- **Difficulty:** 3
- **Question:** During one sleep stage the EEG resembles wakefulness, the eyes move rapidly, vivid dreaming is common, and skeletal muscles are largely paralyzed. This stage is best identified as what?
- **A:** Stage 1 NREM sleep
- **B:** Slow-wave (deep) NREM sleep
- **C:** REM sleep
- **D:** A hypnagogic pre-sleep state
- **Correct:** C
- **Explanation:** REM sleep features wake-like brain activity, rapid eye movements, vivid dreams, and muscle atonia that prevents physically acting out dreams.
- **Tags:** `KC::PsychSoc::Consciousness` `Prereq::PsychSoc::Attention` `Prereq::PsychSoc::Cognition` `MCAT::Psych_Soc` `Difficulty::3`

### MCAT-PSY-CON-002

- **KC:** `PsychSoc::Consciousness`
- **Prereqs:** `Prereq::PsychSoc::Attention` `Prereq::PsychSoc::Cognition`
- **Difficulty:** 2
- **Question:** The roughly 24-hour internal cycle that regulates the human sleep-wake pattern, influenced by light and the hormone melatonin, is called what?
- **A:** The circadian rhythm
- **B:** The refractory period
- **C:** The general adaptation syndrome
- **D:** The absolute threshold
- **Correct:** A
- **Explanation:** The circadian rhythm is the body's approximately 24-hour biological clock, entrained by light and modulated by melatonin released from the pineal gland.
- **Tags:** `KC::PsychSoc::Consciousness` `Prereq::PsychSoc::Attention` `Prereq::PsychSoc::Cognition` `MCAT::Psych_Soc` `Difficulty::2`

### MCAT-PSY-CON-003

- **KC:** `PsychSoc::Consciousness`
- **Prereqs:** `Prereq::PsychSoc::Attention` `Prereq::PsychSoc::Cognition`
- **Difficulty:** 4
- **Question:** A recreational drug slows central nervous system activity, reduces anxiety, and impairs coordination and judgment. This drug is best classified as which type of psychoactive substance?
- **A:** A depressant
- **B:** A stimulant
- **C:** A hallucinogen
- **D:** A substance with no central nervous system effect
- **Correct:** A
- **Explanation:** Depressants such as alcohol and benzodiazepines reduce central nervous system activity, producing sedation, lowered anxiety, and impaired coordination.
- **Tags:** `KC::PsychSoc::Consciousness` `Prereq::PsychSoc::Attention` `Prereq::PsychSoc::Cognition` `MCAT::Psych_Soc` `Difficulty::4`

## PsychSoc::Language

### MCAT-PSY-LAN-001

- **KC:** `PsychSoc::Language`
- **Prereqs:** `Prereq::PsychSoc::Cognition`
- **Difficulty:** 3
- **Question:** A young child produces the novel error "I goed to the park," a form never modeled by adults. This overregularization is most often cited as evidence for which view of language development?
- **A:** Language is acquired purely by imitating heard speech
- **B:** Children actively infer and apply grammatical rules, not only copy input
- **C:** Language depends only on operant reinforcement of correct words
- **D:** Grammar cannot develop without explicit formal instruction
- **Correct:** B
- **Explanation:** Overregularizing a rule such as adding "-ed" where it does not belong shows children extend inferred grammatical rules rather than merely imitating the speech they hear.
- **Tags:** `KC::PsychSoc::Language` `Prereq::PsychSoc::Cognition` `MCAT::Psych_Soc` `Difficulty::3`

### MCAT-PSY-LAN-002

- **KC:** `PsychSoc::Language`
- **Prereqs:** `Prereq::PsychSoc::Cognition`
- **Difficulty:** 2
- **Question:** A patient understands speech and knows what they want to say but produces slow, effortful, broken sentences after a stroke. Damage to which area is most consistent with this pattern?
- **A:** Broca's area
- **B:** Wernicke's area
- **C:** The occipital lobe
- **D:** The cerebellum
- **Correct:** A
- **Explanation:** Broca's area in the frontal lobe supports speech production; damage causes Broca's (expressive) aphasia, marked by halting, effortful speech with relatively preserved comprehension.
- **Tags:** `KC::PsychSoc::Language` `Prereq::PsychSoc::Cognition` `MCAT::Psych_Soc` `Difficulty::2`

### MCAT-PSY-LAN-003

- **KC:** `PsychSoc::Language`
- **Prereqs:** `Prereq::PsychSoc::Cognition`
- **Difficulty:** 4
- **Question:** The proposal that the particular language a person speaks shapes or constrains the way they think about the world is most associated with which idea?
- **A:** The linguistic relativity (Whorfian) hypothesis
- **B:** The theory of universal grammar
- **C:** Overregularization of grammar
- **D:** The phonological loop
- **Correct:** A
- **Explanation:** The linguistic relativity hypothesis, associated with Sapir and Whorf, proposes that the structure and vocabulary of one's language influence cognition and perception.
- **Tags:** `KC::PsychSoc::Language` `Prereq::PsychSoc::Cognition` `MCAT::Psych_Soc` `Difficulty::4`

## PsychSoc::Intelligence

### MCAT-PSY-INT-001

- **KC:** `PsychSoc::Intelligence`
- **Prereqs:** `Prereq::PsychSoc::Cognition` `Prereq::PsychSoc::Language` `Prereq::PsychSoc::Memory`
- **Difficulty:** 4
- **Question:** As adults age, their accumulated vocabulary and factual knowledge remain stable or grow, while their speed at solving unfamiliar logic puzzles tends to decline. This pattern best reflects the distinction between what?
- **A:** Crystallized intelligence remaining stable and fluid intelligence declining
- **B:** General intelligence versus emotional intelligence
- **C:** Achievement testing versus aptitude testing
- **D:** Test reliability versus test validity
- **Correct:** A
- **Explanation:** Crystallized intelligence (learned knowledge and skills) is well maintained or increases with age, whereas fluid intelligence (reasoning about novel problems) typically declines.
- **Tags:** `KC::PsychSoc::Intelligence` `Prereq::PsychSoc::Cognition` `Prereq::PsychSoc::Language` `Prereq::PsychSoc::Memory` `MCAT::Psych_Soc` `Difficulty::4`

### MCAT-PSY-INT-002

- **KC:** `PsychSoc::Intelligence`
- **Prereqs:** `Prereq::PsychSoc::Cognition` `Prereq::PsychSoc::Language` `Prereq::PsychSoc::Memory`
- **Difficulty:** 3
- **Question:** Charles Spearman used factor analysis to argue that performance across diverse mental tests is correlated because they all tap one underlying ability. He called this factor what?
- **A:** General intelligence (g)
- **B:** Emotional intelligence
- **C:** Crystallized intelligence
- **D:** The Flynn effect
- **Correct:** A
- **Explanation:** Spearman proposed a general intelligence factor, g, to account for the positive correlations observed among performance on many different cognitive tasks.
- **Tags:** `KC::PsychSoc::Intelligence` `Prereq::PsychSoc::Cognition` `Prereq::PsychSoc::Language` `Prereq::PsychSoc::Memory` `MCAT::Psych_Soc` `Difficulty::3`

### MCAT-PSY-INT-003

- **KC:** `PsychSoc::Intelligence`
- **Prereqs:** `Prereq::PsychSoc::Cognition` `Prereq::PsychSoc::Language` `Prereq::PsychSoc::Memory`
- **Difficulty:** 5
- **Question:** Before an intelligence test can be interpreted, it is administered to a large representative sample so that any individual's score can be compared against the population distribution. This process is called what?
- **A:** Standardization
- **B:** Test-retest reliability
- **C:** Content validity
- **D:** Heritability estimation
- **Correct:** A
- **Explanation:** Standardization involves administering a test to a representative sample to establish norms, allowing an individual score to be interpreted relative to the broader population.
- **Tags:** `KC::PsychSoc::Intelligence` `Prereq::PsychSoc::Cognition` `Prereq::PsychSoc::Language` `Prereq::PsychSoc::Memory` `MCAT::Psych_Soc` `Difficulty::5`

## PsychSoc::Learning

### MCAT-PSY-LEA-001

- **KC:** `PsychSoc::Learning`
- **Prereqs:** `Prereq::PsychSoc::Biological_and_Social_Factors`
- **Difficulty:** 2
- **Question:** A dog repeatedly hears a bell just before receiving food and eventually salivates to the bell alone. In classical conditioning terms, the bell has become what?
- **A:** An unconditioned stimulus
- **B:** An unconditioned response
- **C:** A conditioned stimulus
- **D:** A negative reinforcer
- **Correct:** C
- **Explanation:** After pairing with food (the unconditioned stimulus), the initially neutral bell becomes a conditioned stimulus that on its own elicits the conditioned response of salivation.
- **Tags:** `KC::PsychSoc::Learning` `Prereq::PsychSoc::Biological_and_Social_Factors` `MCAT::Psych_Soc` `Difficulty::2`

### MCAT-PSY-LEA-002

- **KC:** `PsychSoc::Learning`
- **Prereqs:** `Prereq::PsychSoc::Biological_and_Social_Factors`
- **Difficulty:** 1
- **Question:** A rat presses a lever and a loud, unpleasant noise is switched off; the rat then presses the lever more often. Removing the aversive noise to increase the behavior is an example of what?
- **A:** Positive punishment
- **B:** Negative reinforcement
- **C:** Positive reinforcement
- **D:** Extinction
- **Correct:** B
- **Explanation:** Negative reinforcement increases a behavior by removing an aversive stimulus, which is distinct from punishment, which decreases behavior.
- **Tags:** `KC::PsychSoc::Learning` `Prereq::PsychSoc::Biological_and_Social_Factors` `MCAT::Psych_Soc` `Difficulty::1`

### MCAT-PSY-LEA-003

- **KC:** `PsychSoc::Learning`
- **Prereqs:** `Prereq::PsychSoc::Biological_and_Social_Factors`
- **Difficulty:** 3
- **Question:** Children who watched an adult behave aggressively toward an inflatable doll later imitated that aggression themselves without any direct reward. This finding is most directly associated with which type of learning?
- **A:** Observational learning
- **B:** Classical conditioning
- **C:** Habituation
- **D:** Taste aversion
- **Correct:** A
- **Explanation:** Bandura's Bobo doll studies demonstrated observational learning, in which behavior is acquired by watching and imitating a model rather than through direct reinforcement.
- **Tags:** `KC::PsychSoc::Learning` `Prereq::PsychSoc::Biological_and_Social_Factors` `MCAT::Psych_Soc` `Difficulty::3`

## PsychSoc::Emotion

### MCAT-PSY-EMO-001

- **KC:** `PsychSoc::Emotion`
- **Prereqs:** `Prereq::PsychSoc::Biological_and_Social_Factors`
- **Difficulty:** 3
- **Question:** One theory of emotion proposes that we feel afraid because we first notice our heart racing and body trembling, rather than trembling because we already feel afraid. This describes which theory?
- **A:** Cannon-Bard theory
- **B:** James-Lange theory
- **C:** Schachter-Singer two-factor theory
- **D:** The Yerkes-Dodson law
- **Correct:** B
- **Explanation:** The James-Lange theory holds that emotions arise from perceiving bodily and physiological changes that a stimulus triggers.
- **Tags:** `KC::PsychSoc::Emotion` `Prereq::PsychSoc::Biological_and_Social_Factors` `MCAT::Psych_Soc` `Difficulty::3`

### MCAT-PSY-EMO-002

- **KC:** `PsychSoc::Emotion`
- **Prereqs:** `Prereq::PsychSoc::Biological_and_Social_Factors`
- **Difficulty:** 2
- **Question:** One theory of emotion states that an emotion-arousing stimulus simultaneously and independently triggers physiological arousal and the conscious feeling of emotion. Which theory is this?
- **A:** The Cannon-Bard theory
- **B:** The James-Lange theory
- **C:** The overjustification effect
- **D:** The Yerkes-Dodson law
- **Correct:** A
- **Explanation:** The Cannon-Bard theory proposes that a stimulus produces physiological arousal and the subjective emotional experience at the same time, rather than one causing the other.
- **Tags:** `KC::PsychSoc::Emotion` `Prereq::PsychSoc::Biological_and_Social_Factors` `MCAT::Psych_Soc` `Difficulty::2`

### MCAT-PSY-EMO-003

- **KC:** `PsychSoc::Emotion`
- **Prereqs:** `Prereq::PsychSoc::Biological_and_Social_Factors`
- **Difficulty:** 4
- **Question:** According to the Schachter-Singer two-factor theory, a person who feels physiological arousal will experience a specific emotion only after doing what?
- **A:** Cognitively labeling that arousal based on situational cues
- **B:** Completely suppressing the arousal
- **C:** Waiting for the arousal to disappear
- **D:** Increasing the arousal to a maximum level
- **Correct:** A
- **Explanation:** The two-factor theory holds that emotion arises from general physiological arousal combined with a cognitive label the person assigns using context.
- **Tags:** `KC::PsychSoc::Emotion` `Prereq::PsychSoc::Biological_and_Social_Factors` `MCAT::Psych_Soc` `Difficulty::4`

## PsychSoc::Motivation

### MCAT-PSY-MOT-001

- **KC:** `PsychSoc::Motivation`
- **Prereqs:** `Prereq::PsychSoc::Biological_and_Social_Factors` `Prereq::PsychSoc::Emotion`
- **Difficulty:** 3
- **Question:** Children who already loved drawing were paid cash for each picture; after payments stopped, they drew less than before rewards began. This decline is best explained by what?
- **A:** Drive reduction of a biological need
- **B:** The overjustification effect undermining intrinsic motivation
- **C:** A rise in intrinsic motivation
- **D:** Negative reinforcement of drawing
- **Correct:** B
- **Explanation:** The overjustification effect occurs when external rewards for an already-enjoyed activity reduce intrinsic motivation, so the behavior drops once the rewards are removed.
- **Tags:** `KC::PsychSoc::Motivation` `Prereq::PsychSoc::Biological_and_Social_Factors` `Prereq::PsychSoc::Emotion` `MCAT::Psych_Soc` `Difficulty::3`

### MCAT-PSY-MOT-002

- **KC:** `PsychSoc::Motivation`
- **Prereqs:** `Prereq::PsychSoc::Biological_and_Social_Factors` `Prereq::PsychSoc::Emotion`
- **Difficulty:** 2
- **Question:** In Maslow's hierarchy of needs, which needs must generally be satisfied before a person is motivated to pursue esteem and self-actualization?
- **A:** Physiological and safety needs
- **B:** Self-actualization needs
- **C:** Transcendence needs
- **D:** Only esteem needs
- **Correct:** A
- **Explanation:** Maslow's hierarchy holds that lower-level physiological and safety needs are typically met before higher-level needs such as esteem and self-actualization become motivating.
- **Tags:** `KC::PsychSoc::Motivation` `Prereq::PsychSoc::Biological_and_Social_Factors` `Prereq::PsychSoc::Emotion` `MCAT::Psych_Soc` `Difficulty::2`

### MCAT-PSY-MOT-003

- **KC:** `PsychSoc::Motivation`
- **Prereqs:** `Prereq::PsychSoc::Biological_and_Social_Factors` `Prereq::PsychSoc::Emotion`
- **Difficulty:** 4
- **Question:** A dehydrated person feels a strong urge to seek water, and that urge subsides once they drink. Drive-reduction theory explains this motivation primarily as an attempt to do what?
- **A:** Restore internal physiological homeostasis
- **B:** Maximize external rewards regardless of need
- **C:** Seek arousal purely for its own sake
- **D:** Imitate the behavior of observed models
- **Correct:** A
- **Explanation:** Drive-reduction theory proposes that a physiological need creates a drive that motivates behavior aimed at reducing the drive and returning the body to homeostasis.
- **Tags:** `KC::PsychSoc::Motivation` `Prereq::PsychSoc::Biological_and_Social_Factors` `Prereq::PsychSoc::Emotion` `MCAT::Psych_Soc` `Difficulty::4`

## PsychSoc::Stress

### MCAT-PSY-STR-001

- **KC:** `PsychSoc::Stress`
- **Prereqs:** `Prereq::Bio::Endocrine_System` `Prereq::PsychSoc::Emotion`
- **Difficulty:** 3
- **Question:** During prolonged stress, the hypothalamic-pituitary-adrenal axis sustains release of which hormone that is associated with the exhaustion phase of the general adaptation syndrome?
- **A:** Insulin
- **B:** Cortisol
- **C:** Melatonin
- **D:** Oxytocin
- **Correct:** B
- **Explanation:** Chronic stress keeps the HPA axis active and elevates cortisol, and prolonged cortisol exposure is linked to the exhaustion phase and adverse health effects.
- **Tags:** `KC::PsychSoc::Stress` `Prereq::Bio::Endocrine_System` `Prereq::PsychSoc::Emotion` `MCAT::Psych_Soc` `Difficulty::3`

### MCAT-PSY-STR-002

- **KC:** `PsychSoc::Stress`
- **Prereqs:** `Prereq::Bio::Endocrine_System` `Prereq::PsychSoc::Emotion`
- **Difficulty:** 2
- **Question:** In Lazarus's cognitive appraisal model of stress, primary appraisal refers to what?
- **A:** Evaluating whether a situation is threatening, harmful, or challenging
- **B:** Deciding which coping resources are available to handle it
- **C:** Measuring the cortisol level circulating in the blood
- **D:** The exhaustion phase of prolonged arousal
- **Correct:** A
- **Explanation:** In Lazarus's model, primary appraisal is the initial evaluation of whether an event is irrelevant, benign, or stressful, that is, threatening, harmful, or challenging.
- **Tags:** `KC::PsychSoc::Stress` `Prereq::Bio::Endocrine_System` `Prereq::PsychSoc::Emotion` `MCAT::Psych_Soc` `Difficulty::2`

### MCAT-PSY-STR-003

- **KC:** `PsychSoc::Stress`
- **Prereqs:** `Prereq::Bio::Endocrine_System` `Prereq::PsychSoc::Emotion`
- **Difficulty:** 4
- **Question:** Facing an exam, a student responds by building a study schedule and seeking tutoring rather than trying to calm their anxious feelings. This response is best classified as which coping strategy?
- **A:** Problem-focused coping
- **B:** Emotion-focused coping
- **C:** Avoidance coping
- **D:** The alarm reaction
- **Correct:** A
- **Explanation:** Problem-focused coping targets the stressor itself through direct action, whereas emotion-focused coping aims to manage the emotional reaction to the stressor.
- **Tags:** `KC::PsychSoc::Stress` `Prereq::Bio::Endocrine_System` `Prereq::PsychSoc::Emotion` `MCAT::Psych_Soc` `Difficulty::4`

## PsychSoc::Personality

### MCAT-PSY-PER2-001

- **KC:** `PsychSoc::Personality`
- **Prereqs:** `Prereq::PsychSoc::Biological_and_Social_Factors`
- **Difficulty:** 3
- **Question:** A questionnaire scores individuals on openness, conscientiousness, extraversion, agreeableness, and neuroticism. This assessment is grounded in which approach to personality?
- **A:** The psychoanalytic approach
- **B:** The trait (five-factor) approach
- **C:** The humanistic approach
- **D:** The strict behaviorist approach
- **Correct:** B
- **Explanation:** The five-factor (Big Five) model is a trait approach that describes personality along five broad, measurable dimensions.
- **Tags:** `KC::PsychSoc::Personality` `Prereq::PsychSoc::Biological_and_Social_Factors` `MCAT::Psych_Soc` `Difficulty::3`

### MCAT-PSY-PER2-002

- **KC:** `PsychSoc::Personality`
- **Prereqs:** `Prereq::PsychSoc::Biological_and_Social_Factors`
- **Difficulty:** 2
- **Question:** In Freud's structural model of personality, which component operates on the "pleasure principle," demanding immediate gratification of basic urges?
- **A:** The id
- **B:** The ego
- **C:** The superego
- **D:** The locus of control
- **Correct:** A
- **Explanation:** Freud described the id as the primitive component that operates on the pleasure principle, seeking immediate satisfaction of drives without regard to reality.
- **Tags:** `KC::PsychSoc::Personality` `Prereq::PsychSoc::Biological_and_Social_Factors` `MCAT::Psych_Soc` `Difficulty::2`

### MCAT-PSY-PER2-003

- **KC:** `PsychSoc::Personality`
- **Prereqs:** `Prereq::PsychSoc::Biological_and_Social_Factors`
- **Difficulty:** 4
- **Question:** A humanistic psychologist argues that a healthy personality develops when a person is accepted without any conditions placed on that acceptance. This concept, central to Carl Rogers's theory, is called what?
- **A:** Unconditional positive regard
- **B:** Reciprocal determinism
- **C:** A defense mechanism
- **D:** An external locus of control
- **Correct:** A
- **Explanation:** Carl Rogers held that unconditional positive regard, being accepted without conditions, fosters congruence and healthy self-development in the humanistic view.
- **Tags:** `KC::PsychSoc::Personality` `Prereq::PsychSoc::Biological_and_Social_Factors` `MCAT::Psych_Soc` `Difficulty::4`

## PsychSoc::Self_and_Identity

### MCAT-PSY-SELI-001

- **KC:** `PsychSoc::Self_and_Identity`
- **Prereqs:** `Prereq::PsychSoc::Personality` `Prereq::PsychSoc::Socialization`
- **Difficulty:** 3
- **Question:** Charles Cooley proposed that a person's self-concept develops largely from imagining how other people perceive and judge them. This idea is known as what?
- **A:** The looking-glass self
- **B:** Self-actualization
- **C:** The fundamental attribution error
- **D:** An external locus of control
- **Correct:** A
- **Explanation:** Cooley's looking-glass self proposes that we build our self-image from our perception of how others see and evaluate us.
- **Tags:** `KC::PsychSoc::Self_and_Identity` `Prereq::PsychSoc::Personality` `Prereq::PsychSoc::Socialization` `MCAT::Psych_Soc` `Difficulty::3`

### MCAT-PSY-SELI-002

- **KC:** `PsychSoc::Self_and_Identity`
- **Prereqs:** `Prereq::PsychSoc::Personality` `Prereq::PsychSoc::Socialization`
- **Difficulty:** 2
- **Question:** Albert Bandura used the term "self-efficacy" to describe what?
- **A:** A person's belief in their capability to carry out the actions needed to reach a specific goal
- **B:** A person's overall sense of self-worth across all domains
- **C:** The internalized attitudes of society toward the self
- **D:** A status a person is assigned involuntarily at birth
- **Correct:** A
- **Explanation:** Bandura defined self-efficacy as one's belief in one's ability to succeed at specific tasks, which shapes effort, persistence, and the goals one attempts.
- **Tags:** `KC::PsychSoc::Self_and_Identity` `Prereq::PsychSoc::Personality` `Prereq::PsychSoc::Socialization` `MCAT::Psych_Soc` `Difficulty::2`

### MCAT-PSY-SELI-003

- **KC:** `PsychSoc::Self_and_Identity`
- **Prereqs:** `Prereq::PsychSoc::Personality` `Prereq::PsychSoc::Socialization`
- **Difficulty:** 4
- **Question:** George Herbert Mead distinguished the spontaneous, acting "I" from the socialized "me." The "me" primarily represents what?
- **A:** The organized attitudes and expectations of others that a person has internalized
- **B:** The purely biological drives of the individual
- **C:** A person's unlearned reflexive responses
- **D:** The physical body as separate from the mind
- **Correct:** A
- **Explanation:** In Mead's theory, the "me" is the socialized self, made up of the internalized expectations of society, while the "I" is the individual's spontaneous, creative response.
- **Tags:** `KC::PsychSoc::Self_and_Identity` `Prereq::PsychSoc::Personality` `Prereq::PsychSoc::Socialization` `MCAT::Psych_Soc` `Difficulty::4`

## PsychSoc::Psychological_Disorders

### MCAT-PSY-PSYD-001

- **KC:** `PsychSoc::Psychological_Disorders`
- **Prereqs:** `Prereq::PsychSoc::Biological_and_Social_Factors` `Prereq::PsychSoc::Emotion` `Prereq::PsychSoc::Stress`
- **Difficulty:** 4
- **Question:** A model proposes that a person with a genetic vulnerability develops a disorder only when they also encounter sufficient environmental stress. This explanation is best described as what?
- **A:** The diathesis-stress model
- **B:** The medical model acting alone
- **C:** Pure behavioral conditioning
- **D:** The just-world hypothesis
- **Correct:** A
- **Explanation:** The diathesis-stress model holds that psychological disorders emerge from the interaction of a predisposition (diathesis) with stressful experiences.
- **Tags:** `KC::PsychSoc::Psychological_Disorders` `Prereq::PsychSoc::Biological_and_Social_Factors` `Prereq::PsychSoc::Emotion` `Prereq::PsychSoc::Stress` `MCAT::Psych_Soc` `Difficulty::4`

### MCAT-PSY-PSYD-002

- **KC:** `PsychSoc::Psychological_Disorders`
- **Prereqs:** `Prereq::PsychSoc::Biological_and_Social_Factors` `Prereq::PsychSoc::Emotion` `Prereq::PsychSoc::Stress`
- **Difficulty:** 3
- **Question:** In schizophrenia, hallucinations and delusions are best classified as which category of symptoms?
- **A:** Positive symptoms
- **B:** Negative symptoms
- **C:** Cognitive symptoms of depression
- **D:** Manic symptoms
- **Correct:** A
- **Explanation:** Positive symptoms of schizophrenia are additions to normal experience, such as hallucinations and delusions, whereas negative symptoms are deficits such as flat affect and avolition.
- **Tags:** `KC::PsychSoc::Psychological_Disorders` `Prereq::PsychSoc::Biological_and_Social_Factors` `Prereq::PsychSoc::Emotion` `Prereq::PsychSoc::Stress` `MCAT::Psych_Soc` `Difficulty::3`

### MCAT-PSY-PSYD-003

- **KC:** `PsychSoc::Psychological_Disorders`
- **Prereqs:** `Prereq::PsychSoc::Biological_and_Social_Factors` `Prereq::PsychSoc::Emotion` `Prereq::PsychSoc::Stress`
- **Difficulty:** 5
- **Question:** A clinician diagnoses major depressive disorder only after low mood and loss of interest have persisted for at least two weeks and impair daily functioning. Requiring duration and impairment criteria most directly serves which purpose?
- **A:** Distinguishing a clinical disorder from normal, transient distress
- **B:** Establishing that all sadness is pathological
- **C:** Proving disorders have no biological basis
- **D:** Showing that diagnostic labels are unnecessary
- **Correct:** A
- **Explanation:** Diagnostic criteria requiring symptoms to persist and cause significant distress or impairment help separate a clinical disorder from ordinary, transient emotional reactions.
- **Tags:** `KC::PsychSoc::Psychological_Disorders` `Prereq::PsychSoc::Biological_and_Social_Factors` `Prereq::PsychSoc::Emotion` `Prereq::PsychSoc::Stress` `MCAT::Psych_Soc` `Difficulty::5`

## PsychSoc::Attitudes_and_Beliefs

### MCAT-PSY-ATTB-001

- **KC:** `PsychSoc::Attitudes_and_Beliefs`
- **Prereqs:** `Prereq::PsychSoc::Cognition` `Prereq::PsychSoc::Learning`
- **Difficulty:** 3
- **Question:** After freely choosing to perform a tedious task for very little pay, participants later rated the task as more enjoyable than participants who were paid a large sum. This shift is best explained by what?
- **A:** The mere exposure effect
- **B:** Reduction of cognitive dissonance
- **C:** Operant extinction
- **D:** The bystander effect
- **Correct:** B
- **Explanation:** With little external justification, participants felt dissonance between "this was boring" and "I chose to do it," and reduced it by changing their attitude to see the task as enjoyable.
- **Tags:** `KC::PsychSoc::Attitudes_and_Beliefs` `Prereq::PsychSoc::Cognition` `Prereq::PsychSoc::Learning` `MCAT::Psych_Soc` `Difficulty::3`

### MCAT-PSY-ATTB-002

- **KC:** `PsychSoc::Attitudes_and_Beliefs`
- **Prereqs:** `Prereq::PsychSoc::Cognition` `Prereq::PsychSoc::Learning`
- **Difficulty:** 2
- **Question:** According to the elaboration likelihood model, persuasion that works through careful, logical evaluation of the strength of an argument follows which route?
- **A:** The central route
- **B:** The peripheral route
- **C:** The route of social loafing
- **D:** The route of deindividuation
- **Correct:** A
- **Explanation:** The central route to persuasion involves thoughtful consideration of argument quality and tends to produce durable attitude change, unlike the cue-based peripheral route.
- **Tags:** `KC::PsychSoc::Attitudes_and_Beliefs` `Prereq::PsychSoc::Cognition` `Prereq::PsychSoc::Learning` `MCAT::Psych_Soc` `Difficulty::2`

### MCAT-PSY-ATTB-003

- **KC:** `PsychSoc::Attitudes_and_Beliefs`
- **Prereqs:** `Prereq::PsychSoc::Cognition` `Prereq::PsychSoc::Learning`
- **Difficulty:** 4
- **Question:** A salesperson first gets a customer to agree to a trivial request, which makes the customer more likely to later agree to a much larger one. This compliance strategy is called what?
- **A:** The foot-in-the-door technique
- **B:** The door-in-the-face technique
- **C:** The mere exposure effect
- **D:** The bystander effect
- **Correct:** A
- **Explanation:** The foot-in-the-door technique raises compliance with a large request by first securing agreement to a smaller, related one, partly through consistency pressures.
- **Tags:** `KC::PsychSoc::Attitudes_and_Beliefs` `Prereq::PsychSoc::Cognition` `Prereq::PsychSoc::Learning` `MCAT::Psych_Soc` `Difficulty::4`

## PsychSoc::Stereotypes

### MCAT-PSY-STE-001

- **KC:** `PsychSoc::Stereotypes`
- **Prereqs:** `Prereq::PsychSoc::Attitudes_and_Beliefs` `Prereq::PsychSoc::Cognition`
- **Difficulty:** 3
- **Question:** Reminding members of a group about a negative stereotype of their group just before a difficult test caused them to score lower than an unreminded control group. This phenomenon is called what?
- **A:** Stereotype threat
- **B:** The contact hypothesis
- **C:** Ingroup favoritism
- **D:** Deindividuation
- **Correct:** A
- **Explanation:** Stereotype threat is the anxiety and performance decrement that arise when people fear confirming a negative stereotype about a group they belong to.
- **Tags:** `KC::PsychSoc::Stereotypes` `Prereq::PsychSoc::Attitudes_and_Beliefs` `Prereq::PsychSoc::Cognition` `MCAT::Psych_Soc` `Difficulty::3`

### MCAT-PSY-STE-002

- **KC:** `PsychSoc::Stereotypes`
- **Prereqs:** `Prereq::PsychSoc::Attitudes_and_Beliefs` `Prereq::PsychSoc::Cognition`
- **Difficulty:** 2
- **Question:** A teacher expects certain students to excel and, by treating them warmly and giving them more attention, actually causes them to perform better. This process is best described as what?
- **A:** A self-fulfilling prophecy
- **B:** The just-world hypothesis
- **C:** Deindividuation
- **D:** The contact hypothesis
- **Correct:** A
- **Explanation:** A self-fulfilling prophecy occurs when an expectation elicits behavior that makes the expectation come true, as in expectancy effects on student performance.
- **Tags:** `KC::PsychSoc::Stereotypes` `Prereq::PsychSoc::Attitudes_and_Beliefs` `Prereq::PsychSoc::Cognition` `MCAT::Psych_Soc` `Difficulty::2`

### MCAT-PSY-STE-003

- **KC:** `PsychSoc::Stereotypes`
- **Prereqs:** `Prereq::PsychSoc::Attitudes_and_Beliefs` `Prereq::PsychSoc::Cognition`
- **Difficulty:** 4
- **Question:** People overestimate how often members of a small minority group perform a rare behavior because both the group and the behavior are distinctive and stand out together in memory. This bias underlying stereotype formation is called what?
- **A:** Illusory correlation
- **B:** The framing effect
- **C:** Regression to the mean
- **D:** The actor-observer bias
- **Correct:** A
- **Explanation:** An illusory correlation is a perceived association between two distinctive but statistically unrelated variables, which can seed and reinforce stereotypes.
- **Tags:** `KC::PsychSoc::Stereotypes` `Prereq::PsychSoc::Attitudes_and_Beliefs` `Prereq::PsychSoc::Cognition` `MCAT::Psych_Soc` `Difficulty::4`

## PsychSoc::Prejudice_and_Bias

### MCAT-PSY-PREB-001

- **KC:** `PsychSoc::Prejudice_and_Bias`
- **Prereqs:** `Prereq::PsychSoc::Attitudes_and_Beliefs` `Prereq::PsychSoc::Stereotypes`
- **Difficulty:** 4
- **Question:** A researcher predicts that having two rival groups cooperate as equals toward a shared goal will reduce mutual prejudice. This prediction is grounded in which idea?
- **A:** The just-world hypothesis
- **B:** The contact hypothesis
- **C:** Social loafing
- **D:** The actor-observer bias
- **Correct:** B
- **Explanation:** The contact hypothesis proposes that intergroup prejudice decreases when groups interact under favorable conditions such as equal status and cooperation toward common goals.
- **Tags:** `KC::PsychSoc::Prejudice_and_Bias` `Prereq::PsychSoc::Attitudes_and_Beliefs` `Prereq::PsychSoc::Stereotypes` `MCAT::Psych_Soc` `Difficulty::4`

### MCAT-PSY-PREB-002

- **KC:** `PsychSoc::Prejudice_and_Bias`
- **Prereqs:** `Prereq::PsychSoc::Attitudes_and_Beliefs` `Prereq::PsychSoc::Stereotypes`
- **Difficulty:** 3
- **Question:** In the classic Robbers Cave study, competition between two groups of boys for limited prizes rapidly produced mutual hostility. This finding best supports which explanation of intergroup prejudice?
- **A:** Realistic conflict theory
- **B:** The looking-glass self
- **C:** The overjustification effect
- **D:** Drive-reduction theory
- **Correct:** A
- **Explanation:** Realistic conflict theory holds that intergroup hostility arises from competition over scarce resources, as demonstrated in the Robbers Cave experiment.
- **Tags:** `KC::PsychSoc::Prejudice_and_Bias` `Prereq::PsychSoc::Attitudes_and_Beliefs` `Prereq::PsychSoc::Stereotypes` `MCAT::Psych_Soc` `Difficulty::3`

### MCAT-PSY-PREB-003

- **KC:** `PsychSoc::Prejudice_and_Bias`
- **Prereqs:** `Prereq::PsychSoc::Attitudes_and_Beliefs` `Prereq::PsychSoc::Stereotypes`
- **Difficulty:** 5
- **Question:** During an economic downturn, a frustrated majority directs blame and hostility onto a relatively powerless minority rather than the true cause of their hardship. This pattern is best explained by which concept?
- **A:** The scapegoat (frustration-aggression) hypothesis
- **B:** The contact hypothesis
- **C:** Social facilitation
- **D:** Meritocracy
- **Correct:** A
- **Explanation:** The scapegoat hypothesis links frustration to displaced aggression, in which a vulnerable group is unfairly blamed for problems it did not cause.
- **Tags:** `KC::PsychSoc::Prejudice_and_Bias` `Prereq::PsychSoc::Attitudes_and_Beliefs` `Prereq::PsychSoc::Stereotypes` `MCAT::Psych_Soc` `Difficulty::5`

## PsychSoc::Social_Interaction

### MCAT-PSY-SOCI-001

- **KC:** `PsychSoc::Social_Interaction`
- **Prereqs:** `Prereq::PsychSoc::Attitudes_and_Beliefs` `Prereq::PsychSoc::Self_and_Identity`
- **Difficulty:** 4
- **Question:** Erving Goffman described social life as theater in which people manage the impressions others form of them, behaving differently in front-stage and back-stage settings. This perspective is called what?
- **A:** The dramaturgical approach
- **B:** Structural functionalism
- **C:** Rational-choice exchange theory
- **D:** The tragedy of the commons
- **Correct:** A
- **Explanation:** Goffman's dramaturgical approach analyzes interaction as a theatrical performance in which individuals engage in impression management across front- and back-stage regions.
- **Tags:** `KC::PsychSoc::Social_Interaction` `Prereq::PsychSoc::Attitudes_and_Beliefs` `Prereq::PsychSoc::Self_and_Identity` `MCAT::Psych_Soc` `Difficulty::4`

### MCAT-PSY-SOCI-002

- **KC:** `PsychSoc::Social_Interaction`
- **Prereqs:** `Prereq::PsychSoc::Attitudes_and_Beliefs` `Prereq::PsychSoc::Self_and_Identity`
- **Difficulty:** 3
- **Question:** Seeing a stranger trip on the sidewalk, an observer concludes the person is clumsy rather than noticing the patch of ice. Overattributing others' behavior to their disposition while underweighting the situation is called what?
- **A:** The fundamental attribution error
- **B:** The self-serving bias
- **C:** The bystander effect
- **D:** Impression management
- **Correct:** A
- **Explanation:** The fundamental attribution error is the tendency to overemphasize dispositional causes and underestimate situational causes when explaining other people's behavior.
- **Tags:** `KC::PsychSoc::Social_Interaction` `Prereq::PsychSoc::Attitudes_and_Beliefs` `Prereq::PsychSoc::Self_and_Identity` `MCAT::Psych_Soc` `Difficulty::3`

### MCAT-PSY-SOCI-003

- **KC:** `PsychSoc::Social_Interaction`
- **Prereqs:** `Prereq::PsychSoc::Attitudes_and_Beliefs` `Prereq::PsychSoc::Self_and_Identity`
- **Difficulty:** 5
- **Question:** A theory analyzes relationships as ongoing exchanges in which people try to maximize rewards and minimize costs, continuing a relationship only while the balance stays favorable. This perspective is known as what?
- **A:** Social exchange theory
- **B:** The dramaturgical approach
- **C:** Structural functionalism
- **D:** The looking-glass self
- **Correct:** A
- **Explanation:** Social exchange theory models social behavior as a cost-benefit analysis in which people seek to maximize rewards and minimize costs across their interactions.
- **Tags:** `KC::PsychSoc::Social_Interaction` `Prereq::PsychSoc::Attitudes_and_Beliefs` `Prereq::PsychSoc::Self_and_Identity` `MCAT::Psych_Soc` `Difficulty::5`

## PsychSoc::Group_Behavior

### MCAT-PSY-GROB-001

- **KC:** `PsychSoc::Group_Behavior`
- **Prereqs:** `Prereq::PsychSoc::Social_Interaction` `Prereq::PsychSoc::Socialization`
- **Difficulty:** 4
- **Question:** In a tug-of-war, each person tends to pull with less force when part of a large team than when pulling alone and measured individually. This reduction in individual effort within a group is called what?
- **A:** Social facilitation
- **B:** Social loafing
- **C:** Group polarization
- **D:** Deindividuation
- **Correct:** B
- **Explanation:** Social loafing is the tendency to exert less individual effort in a group than when acting alone, particularly when individual contributions cannot be identified.
- **Tags:** `KC::PsychSoc::Group_Behavior` `Prereq::PsychSoc::Social_Interaction` `Prereq::PsychSoc::Socialization` `MCAT::Psych_Soc` `Difficulty::4`

### MCAT-PSY-GROB-002

- **KC:** `PsychSoc::Group_Behavior`
- **Prereqs:** `Prereq::PsychSoc::Social_Interaction` `Prereq::PsychSoc::Socialization`
- **Difficulty:** 3
- **Question:** A tight-knit advisory committee suppresses dissent and reaches a poor decision because members value harmony and unanimity over critically weighing alternatives. This phenomenon is called what?
- **A:** Groupthink
- **B:** Social facilitation
- **C:** Group polarization
- **D:** Social loafing
- **Correct:** A
- **Explanation:** Groupthink occurs when a cohesive group's desire for consensus overrides realistic appraisal of alternatives, leading to flawed decisions.
- **Tags:** `KC::PsychSoc::Group_Behavior` `Prereq::PsychSoc::Social_Interaction` `Prereq::PsychSoc::Socialization` `MCAT::Psych_Soc` `Difficulty::3`

### MCAT-PSY-GROB-003

- **KC:** `PsychSoc::Group_Behavior`
- **Prereqs:** `Prereq::PsychSoc::Social_Interaction` `Prereq::PsychSoc::Socialization`
- **Difficulty:** 5
- **Question:** After discussing an issue together, group members who initially leaned slightly toward a risky choice collectively endorse an even more extreme position than any of them held alone. This shift is best described as what?
- **A:** Group polarization
- **B:** Deindividuation
- **C:** The bystander effect
- **D:** Social loafing
- **Correct:** A
- **Explanation:** Group polarization is the tendency for group discussion to strengthen members' initial leanings, producing a more extreme collective position.
- **Tags:** `KC::PsychSoc::Group_Behavior` `Prereq::PsychSoc::Social_Interaction` `Prereq::PsychSoc::Socialization` `MCAT::Psych_Soc` `Difficulty::5`

## PsychSoc::Social_Theory

### MCAT-PSY-SOCT-001

- **KC:** `PsychSoc::Social_Theory`
- **Prereqs:** none
- **Difficulty:** 2
- **Question:** A sociologist argues that higher education, beyond its stated goal of teaching skills, also serves the unstated purpose of forming social networks and romantic partnerships. This unintended, unrecognized consequence is best labeled what?
- **A:** A manifest function
- **B:** A latent function
- **C:** A dysfunction
- **D:** A symbolic interaction
- **Correct:** B
- **Explanation:** In functionalist theory, latent functions are the unintended and often unrecognized consequences of a social structure, distinct from its intended manifest functions.
- **Tags:** `KC::PsychSoc::Social_Theory` `MCAT::Psych_Soc` `Difficulty::2`

### MCAT-PSY-SOCT-002

- **KC:** `PsychSoc::Social_Theory`
- **Prereqs:** none
- **Difficulty:** 1
- **Question:** A theorist argues that society is best understood as an arena of struggle in which powerful groups maintain dominance over scarce resources at the expense of others. This view reflects which sociological paradigm?
- **A:** Conflict theory
- **B:** Structural functionalism
- **C:** The dramaturgical approach
- **D:** Rational-choice theory
- **Correct:** A
- **Explanation:** Conflict theory, rooted in the work of Marx, views society as shaped by ongoing struggles between groups over power and limited resources.
- **Tags:** `KC::PsychSoc::Social_Theory` `MCAT::Psych_Soc` `Difficulty::1`

### MCAT-PSY-SOCT-003

- **KC:** `PsychSoc::Social_Theory`
- **Prereqs:** none
- **Difficulty:** 3
- **Question:** A researcher studies how people build shared meaning through their everyday, face-to-face use of symbols and language. This micro-level focus is characteristic of which theoretical perspective?
- **A:** Symbolic interactionism
- **B:** Structural functionalism
- **C:** Conflict theory
- **D:** The demographic transition model
- **Correct:** A
- **Explanation:** Symbolic interactionism is a micro-level perspective that examines how people create and share meaning through symbols during everyday interaction.
- **Tags:** `KC::PsychSoc::Social_Theory` `MCAT::Psych_Soc` `Difficulty::3`

## PsychSoc::Culture

### MCAT-PSY-CUL-001

- **KC:** `PsychSoc::Culture`
- **Prereqs:** none
- **Difficulty:** 2
- **Question:** A society's shared beliefs, values, and norms, as opposed to its physical objects and technology, are best categorized as what?
- **A:** Material culture
- **B:** Nonmaterial culture
- **C:** Cultural diffusion
- **D:** A counterculture
- **Correct:** B
- **Explanation:** Nonmaterial culture consists of intangible elements such as beliefs, values, norms, and language, whereas material culture is the physical artifacts a society produces.
- **Tags:** `KC::PsychSoc::Culture` `MCAT::Psych_Soc` `Difficulty::2`

### MCAT-PSY-CUL-002

- **KC:** `PsychSoc::Culture`
- **Prereqs:** none
- **Difficulty:** 1
- **Question:** Judging the customs of another society as strange or inferior by using the standards of one's own culture is best described as what?
- **A:** Ethnocentrism
- **B:** Cultural relativism
- **C:** Cultural diffusion
- **D:** Assimilation
- **Correct:** A
- **Explanation:** Ethnocentrism is the tendency to evaluate other cultures by the standards of one's own culture, often assuming one's own is superior.
- **Tags:** `KC::PsychSoc::Culture` `MCAT::Psych_Soc` `Difficulty::1`

### MCAT-PSY-CUL-003

- **KC:** `PsychSoc::Culture`
- **Prereqs:** none
- **Difficulty:** 3
- **Question:** When material culture such as new technology changes faster than the nonmaterial norms and laws meant to regulate it, sociologists call the resulting mismatch what?
- **A:** Cultural lag
- **B:** Cultural relativism
- **C:** A subculture
- **D:** Cultural universality
- **Correct:** A
- **Explanation:** Cultural lag, a concept associated with William Ogburn, is the delay between rapid changes in material culture and the slower adjustment of nonmaterial culture.
- **Tags:** `KC::PsychSoc::Culture` `MCAT::Psych_Soc` `Difficulty::3`

## PsychSoc::Socialization

### MCAT-PSY-SOC-001

- **KC:** `PsychSoc::Socialization`
- **Prereqs:** `Prereq::PsychSoc::Culture`
- **Difficulty:** 2
- **Question:** The family is typically described as the most influential agent during which type of socialization?
- **A:** Primary socialization in early childhood
- **B:** Anticipatory socialization for a future role
- **C:** Resocialization inside a total institution
- **D:** Secondary socialization in the workplace
- **Correct:** A
- **Explanation:** Primary socialization occurs in early childhood, and the family is usually the first and most influential agent shaping a child's basic norms and values.
- **Tags:** `KC::PsychSoc::Socialization` `Prereq::PsychSoc::Culture` `MCAT::Psych_Soc` `Difficulty::2`

### MCAT-PSY-SOC-002

- **KC:** `PsychSoc::Socialization`
- **Prereqs:** `Prereq::PsychSoc::Culture`
- **Difficulty:** 1
- **Question:** A new army recruit enters boot camp, where prior habits are stripped away and replaced with a rigid new identity under constant supervision. Erving Goffman would classify boot camp as an example of what?
- **A:** A total institution
- **B:** A reference group
- **C:** A peer subculture
- **D:** A manifest function
- **Correct:** A
- **Explanation:** A total institution, such as a military boot camp or prison, isolates people and tightly controls their lives in order to resocialize them into new roles and identities.
- **Tags:** `KC::PsychSoc::Socialization` `Prereq::PsychSoc::Culture` `MCAT::Psych_Soc` `Difficulty::1`

### MCAT-PSY-SOC-003

- **KC:** `PsychSoc::Socialization`
- **Prereqs:** `Prereq::PsychSoc::Culture`
- **Difficulty:** 3
- **Question:** A high school student begins dressing, speaking, and behaving like the professionals in a career they hope to enter someday. Rehearsing the norms of a future role is best described as what?
- **A:** Anticipatory socialization
- **B:** Primary socialization
- **C:** Resocialization
- **D:** A degradation ceremony
- **Correct:** A
- **Explanation:** Anticipatory socialization is the process of learning and adopting the norms and values of a role a person expects to occupy in the future.
- **Tags:** `KC::PsychSoc::Socialization` `Prereq::PsychSoc::Culture` `MCAT::Psych_Soc` `Difficulty::3`

## PsychSoc::Social_Institutions

### MCAT-PSY-SOCI2-001

- **KC:** `PsychSoc::Social_Institutions`
- **Prereqs:** `Prereq::PsychSoc::Culture` `Prereq::PsychSoc::Social_Theory`
- **Difficulty:** 3
- **Question:** Sociologists group the education system, the family, religion, and government together as examples of what?
- **A:** Social institutions
- **B:** Reference groups
- **C:** Subcultures
- **D:** Informal norms
- **Correct:** A
- **Explanation:** Social institutions are enduring, organized systems of norms and relationships, such as family, education, religion, and government, that meet basic societal needs.
- **Tags:** `KC::PsychSoc::Social_Institutions` `Prereq::PsychSoc::Culture` `Prereq::PsychSoc::Social_Theory` `MCAT::Psych_Soc` `Difficulty::3`

### MCAT-PSY-SOCI2-002

- **KC:** `PsychSoc::Social_Institutions`
- **Prereqs:** `Prereq::PsychSoc::Culture` `Prereq::PsychSoc::Social_Theory`
- **Difficulty:** 2
- **Question:** Beyond academic subjects, schools also transmit punctuality, obedience to authority, and competitiveness. Sociologists call this implicit teaching of norms and values what?
- **A:** The hidden curriculum
- **B:** A manifest function
- **C:** Cultural relativism
- **D:** The dependency ratio
- **Correct:** A
- **Explanation:** The hidden curriculum refers to the norms, values, and dispositions students learn implicitly through the structure and routines of schooling, beyond the formal lessons.
- **Tags:** `KC::PsychSoc::Social_Institutions` `Prereq::PsychSoc::Culture` `Prereq::PsychSoc::Social_Theory` `MCAT::Psych_Soc` `Difficulty::2`

### MCAT-PSY-SOCI2-003

- **KC:** `PsychSoc::Social_Institutions`
- **Prereqs:** `Prereq::PsychSoc::Culture` `Prereq::PsychSoc::Social_Theory`
- **Difficulty:** 4
- **Question:** As a society modernizes, religion tends to lose its influence over public life and over other institutions such as government and education. Sociologists refer to this long-term process as what?
- **A:** Secularization
- **B:** Medicalization
- **C:** Cultural diffusion
- **D:** Resocialization
- **Correct:** A
- **Explanation:** Secularization is the process by which religion loses social and institutional influence as societies modernize and other institutions assume functions once held by religion.
- **Tags:** `KC::PsychSoc::Social_Institutions` `Prereq::PsychSoc::Culture` `Prereq::PsychSoc::Social_Theory` `MCAT::Psych_Soc` `Difficulty::4`

## PsychSoc::Demographics

### MCAT-PSY-DEM-001

- **KC:** `PsychSoc::Demographics`
- **Prereqs:** `Prereq::PsychSoc::Culture` `Prereq::PsychSoc::Social_Institutions`
- **Difficulty:** 3
- **Question:** As a country industrializes, it shifts from high birth and death rates to low birth and death rates. This population pattern is described by which model?
- **A:** The demographic transition model
- **B:** The Malthusian catastrophe
- **C:** The looking-glass self
- **D:** The contact hypothesis
- **Correct:** A
- **Explanation:** The demographic transition model describes the shift from high fertility and mortality to low fertility and mortality that typically accompanies industrialization and development.
- **Tags:** `KC::PsychSoc::Demographics` `Prereq::PsychSoc::Culture` `Prereq::PsychSoc::Social_Institutions` `MCAT::Psych_Soc` `Difficulty::3`

### MCAT-PSY-DEM-002

- **KC:** `PsychSoc::Demographics`
- **Prereqs:** `Prereq::PsychSoc::Culture` `Prereq::PsychSoc::Social_Institutions`
- **Difficulty:** 2
- **Question:** A demographer wants to compare the age and sex composition of two national populations at a glance. Which graphical tool is best suited to this purpose?
- **A:** A population pyramid
- **B:** A scatterplot of national income
- **C:** A Punnett square
- **D:** A family pedigree chart
- **Correct:** A
- **Explanation:** A population pyramid displays a population's distribution by age group and sex, making structural differences between populations easy to compare visually.
- **Tags:** `KC::PsychSoc::Demographics` `Prereq::PsychSoc::Culture` `Prereq::PsychSoc::Social_Institutions` `MCAT::Psych_Soc` `Difficulty::2`

### MCAT-PSY-DEM-003

- **KC:** `PsychSoc::Demographics`
- **Prereqs:** `Prereq::PsychSoc::Culture` `Prereq::PsychSoc::Social_Institutions`
- **Difficulty:** 4
- **Question:** A demographer analyzes a wave of migration by separating the factors that drive people away from their place of origin from the factors that draw them toward a destination. This analysis reflects which framework?
- **A:** Push and pull factors
- **B:** The demographic transition model
- **C:** The dependency ratio
- **D:** Cultural lag
- **Correct:** A
- **Explanation:** Push-pull theory explains migration in terms of push factors that repel people from an origin, such as unemployment, and pull factors that attract them to a destination, such as jobs.
- **Tags:** `KC::PsychSoc::Demographics` `Prereq::PsychSoc::Culture` `Prereq::PsychSoc::Social_Institutions` `MCAT::Psych_Soc` `Difficulty::4`

## PsychSoc::Stratification

### MCAT-PSY-STR2-001

- **KC:** `PsychSoc::Stratification`
- **Prereqs:** `Prereq::PsychSoc::Social_Theory`
- **Difficulty:** 3
- **Question:** A stratification system in which social position is fixed at birth and movement between strata is essentially impossible is best described as what?
- **A:** An open class system
- **B:** A closed, caste-like system
- **C:** A pure meritocracy
- **D:** A system of unlimited social mobility
- **Correct:** B
- **Explanation:** A closed system, such as a caste system, assigns status by ascription at birth and permits little or no movement between strata, unlike an open class system.
- **Tags:** `KC::PsychSoc::Stratification` `Prereq::PsychSoc::Social_Theory` `MCAT::Psych_Soc` `Difficulty::3`

### MCAT-PSY-STR2-002

- **KC:** `PsychSoc::Stratification`
- **Prereqs:** `Prereq::PsychSoc::Social_Theory`
- **Difficulty:** 2
- **Question:** Max Weber argued that a person's overall position in a stratification system depends on more than economic class alone. Which additional dimensions did he emphasize?
- **A:** Status (prestige) and power
- **B:** Height and physical age
- **C:** Reaction time and memory span
- **D:** Material culture only
- **Correct:** A
- **Explanation:** Weber's multidimensional view of stratification identified class (economic position), status (social prestige), and party (power) as distinct dimensions of inequality.
- **Tags:** `KC::PsychSoc::Stratification` `Prereq::PsychSoc::Social_Theory` `MCAT::Psych_Soc` `Difficulty::2`

### MCAT-PSY-STR2-003

- **KC:** `PsychSoc::Stratification`
- **Prereqs:** `Prereq::PsychSoc::Social_Theory`
- **Difficulty:** 4
- **Question:** The Davis-Moore thesis argues that social stratification is functional for society because it does what?
- **A:** Ensures the most qualified people fill the most important positions by attaching greater rewards to them
- **B:** Eliminates all inequality between social groups
- **C:** Guarantees equal outcomes regardless of individual effort
- **D:** Prevents any movement between social positions
- **Correct:** A
- **Explanation:** The Davis-Moore thesis, a functionalist argument, holds that unequal rewards motivate the most capable individuals to fill the positions society deems most important.
- **Tags:** `KC::PsychSoc::Stratification` `Prereq::PsychSoc::Social_Theory` `MCAT::Psych_Soc` `Difficulty::4`

## PsychSoc::Social_Class

### MCAT-PSY-SOCC-001

- **KC:** `PsychSoc::Social_Class`
- **Prereqs:** `Prereq::PsychSoc::Stratification`
- **Difficulty:** 3
- **Question:** When researchers combine income, educational attainment, and occupational prestige into a single measure of a person's standing, they are assessing what?
- **A:** Socioeconomic status
- **B:** Cultural diffusion
- **C:** Absolute poverty
- **D:** The dependency ratio
- **Correct:** A
- **Explanation:** Socioeconomic status is a composite measure of social class based on income, education, and occupational prestige.
- **Tags:** `KC::PsychSoc::Social_Class` `Prereq::PsychSoc::Stratification` `MCAT::Psych_Soc` `Difficulty::3`

### MCAT-PSY-SOCC-002

- **KC:** `PsychSoc::Social_Class`
- **Prereqs:** `Prereq::PsychSoc::Stratification`
- **Difficulty:** 2
- **Question:** A student raised in an affluent home is at ease with the language, tastes, and etiquette prized by elite institutions, giving them an advantage. Pierre Bourdieu called this resource what?
- **A:** Cultural capital
- **B:** Absolute poverty
- **C:** The dependency ratio
- **D:** Ascribed status
- **Correct:** A
- **Explanation:** Bourdieu's concept of cultural capital refers to non-financial assets such as knowledge, tastes, and skills that confer social advantage and can be transmitted across generations.
- **Tags:** `KC::PsychSoc::Social_Class` `Prereq::PsychSoc::Stratification` `MCAT::Psych_Soc` `Difficulty::2`

### MCAT-PSY-SOCC-003

- **KC:** `PsychSoc::Social_Class`
- **Prereqs:** `Prereq::PsychSoc::Stratification`
- **Difficulty:** 4
- **Question:** According to Karl Marx, when workers come to recognize their shared exploitation and their common interests as a class, they have developed what?
- **A:** Class consciousness
- **B:** False consciousness
- **C:** Anomie
- **D:** A meritocracy
- **Correct:** A
- **Explanation:** Class consciousness, in Marxist theory, is the awareness among members of a class of their shared position and interests, which he saw as a precondition for collective action.
- **Tags:** `KC::PsychSoc::Social_Class` `Prereq::PsychSoc::Stratification` `MCAT::Psych_Soc` `Difficulty::4`

## PsychSoc::Social_Mobility

### MCAT-PSY-SOCM-001

- **KC:** `PsychSoc::Social_Mobility`
- **Prereqs:** `Prereq::PsychSoc::Social_Class` `Prereq::PsychSoc::Stratification`
- **Difficulty:** 3
- **Question:** A child of factory workers earns an advanced degree and becomes a physician. Comparing the child's social position to the parents' position illustrates which type of mobility?
- **A:** Intragenerational mobility
- **B:** Intergenerational mobility
- **C:** Horizontal mobility
- **D:** Structural downward mobility
- **Correct:** B
- **Explanation:** Intergenerational mobility compares social position across generations, and a rise from working class to professional between parents and child is upward intergenerational mobility.
- **Tags:** `KC::PsychSoc::Social_Mobility` `Prereq::PsychSoc::Social_Class` `Prereq::PsychSoc::Stratification` `MCAT::Psych_Soc` `Difficulty::3`

### MCAT-PSY-SOCM-002

- **KC:** `PsychSoc::Social_Mobility`
- **Prereqs:** `Prereq::PsychSoc::Social_Class` `Prereq::PsychSoc::Stratification`
- **Difficulty:** 2
- **Question:** A worker is promoted over the course of their career from an entry-level clerk to a senior manager. A change in social position within a single person's own lifetime is called what?
- **A:** Intragenerational mobility
- **B:** Intergenerational mobility
- **C:** Horizontal mobility
- **D:** Structural mobility
- **Correct:** A
- **Explanation:** Intragenerational mobility refers to changes in an individual's social position during their own lifetime, as opposed to comparisons made across generations.
- **Tags:** `KC::PsychSoc::Social_Mobility` `Prereq::PsychSoc::Social_Class` `Prereq::PsychSoc::Stratification` `MCAT::Psych_Soc` `Difficulty::2`

### MCAT-PSY-SOCM-003

- **KC:** `PsychSoc::Social_Mobility`
- **Prereqs:** `Prereq::PsychSoc::Social_Class` `Prereq::PsychSoc::Stratification`
- **Difficulty:** 4
- **Question:** During rapid industrialization, large numbers of people move from farm labor into higher-paying factory and office jobs because the occupational structure itself has changed. This society-wide movement is best described as what?
- **A:** Structural mobility
- **B:** Horizontal mobility
- **C:** Downward mobility
- **D:** Intergenerational immobility
- **Correct:** A
- **Explanation:** Structural mobility results from changes in the economy or occupational structure that shift many people's positions at once, rather than from individual effort alone.
- **Tags:** `KC::PsychSoc::Social_Mobility` `Prereq::PsychSoc::Social_Class` `Prereq::PsychSoc::Stratification` `MCAT::Psych_Soc` `Difficulty::4`

## PsychSoc::Poverty

### MCAT-PSY-POV-001

- **KC:** `PsychSoc::Poverty`
- **Prereqs:** `Prereq::PsychSoc::Social_Class`
- **Difficulty:** 3
- **Question:** A family can meet basic survival needs but has far less than the typical standard of living in their society, limiting participation in ordinary activities. This situation best illustrates what?
- **A:** Absolute poverty
- **B:** Relative poverty
- **C:** Intergenerational mobility
- **D:** Social reproduction
- **Correct:** B
- **Explanation:** Relative poverty is defined by having substantially fewer resources than the societal norm, whereas absolute poverty is the inability to meet basic subsistence needs.
- **Tags:** `KC::PsychSoc::Poverty` `Prereq::PsychSoc::Social_Class` `MCAT::Psych_Soc` `Difficulty::3`

### MCAT-PSY-POV-002

- **KC:** `PsychSoc::Poverty`
- **Prereqs:** `Prereq::PsychSoc::Social_Class`
- **Difficulty:** 2
- **Question:** An agency defines a fixed income threshold below which people cannot afford the minimum food, shelter, and clothing needed to survive. This measure reflects which concept?
- **A:** Absolute poverty
- **B:** Relative poverty
- **C:** Cultural capital
- **D:** Social mobility
- **Correct:** A
- **Explanation:** Absolute poverty is defined by the inability to secure the basic necessities required for survival, measured against a fixed subsistence standard.
- **Tags:** `KC::PsychSoc::Poverty` `Prereq::PsychSoc::Social_Class` `MCAT::Psych_Soc` `Difficulty::2`

### MCAT-PSY-POV-003

- **KC:** `PsychSoc::Poverty`
- **Prereqs:** `Prereq::PsychSoc::Social_Class`
- **Difficulty:** 4
- **Question:** Children raised in poverty often attend under-resourced schools that limit their future earnings, which then keeps their own children in poverty. This self-perpetuating pattern across generations is best described as what?
- **A:** The cycle of poverty
- **B:** Horizontal mobility
- **C:** The demographic transition
- **D:** A meritocracy
- **Correct:** A
- **Explanation:** The cycle of poverty describes how limited resources and opportunities are passed from one generation to the next, making it difficult for families to escape poverty.
- **Tags:** `KC::PsychSoc::Poverty` `Prereq::PsychSoc::Social_Class` `MCAT::Psych_Soc` `Difficulty::4`

## PsychSoc::Social_Inequality

### MCAT-PSY-SOCI3-001

- **KC:** `PsychSoc::Social_Inequality`
- **Prereqs:** `Prereq::PsychSoc::Demographics` `Prereq::PsychSoc::Stratification`
- **Difficulty:** 4
- **Question:** A scholar argues that a woman's experience of disadvantage cannot be understood by examining gender alone but requires considering how race, class, and gender overlap. This framework is known as what?
- **A:** Meritocracy
- **B:** Intersectionality
- **C:** The contact hypothesis
- **D:** Absolute mobility
- **Correct:** B
- **Explanation:** Intersectionality analyzes how overlapping social identities such as race, class, and gender combine to produce distinct, compounded experiences of privilege or disadvantage.
- **Tags:** `KC::PsychSoc::Social_Inequality` `Prereq::PsychSoc::Demographics` `Prereq::PsychSoc::Stratification` `MCAT::Psych_Soc` `Difficulty::4`

### MCAT-PSY-SOCI3-002

- **KC:** `PsychSoc::Social_Inequality`
- **Prereqs:** `Prereq::PsychSoc::Demographics` `Prereq::PsychSoc::Stratification`
- **Difficulty:** 3
- **Question:** A study finds that low-income neighborhoods are far more likely to sit near polluting facilities and to lack green space than wealthier ones. This uneven distribution of environmental burdens is best described as an issue of what?
- **A:** Environmental justice
- **B:** Cultural diffusion
- **C:** The Flynn effect
- **D:** Anticipatory socialization
- **Correct:** A
- **Explanation:** Environmental justice concerns the disproportionate exposure of disadvantaged communities to environmental hazards, a form of spatial and social inequality.
- **Tags:** `KC::PsychSoc::Social_Inequality` `Prereq::PsychSoc::Demographics` `Prereq::PsychSoc::Stratification` `MCAT::Psych_Soc` `Difficulty::3`

### MCAT-PSY-SOCI3-003

- **KC:** `PsychSoc::Social_Inequality`
- **Prereqs:** `Prereq::PsychSoc::Demographics` `Prereq::PsychSoc::Stratification`
- **Difficulty:** 5
- **Question:** A scholar explains persistent global inequality by describing how wealthy "core" nations extract cheap labor and raw materials from poorer "periphery" nations. This account is grounded in which framework?
- **A:** World-systems theory
- **B:** The contact hypothesis
- **C:** The looking-glass self
- **D:** Drive-reduction theory
- **Correct:** A
- **Explanation:** World-systems theory, associated with Wallerstein, explains global inequality through exploitative economic relationships among core, semi-periphery, and periphery nations.
- **Tags:** `KC::PsychSoc::Social_Inequality` `Prereq::PsychSoc::Demographics` `Prereq::PsychSoc::Stratification` `MCAT::Psych_Soc` `Difficulty::5`

## PsychSoc::Health_Disparities

### MCAT-PSY-HEAD-001

- **KC:** `PsychSoc::Health_Disparities`
- **Prereqs:** `Prereq::PsychSoc::Social_Class` `Prereq::PsychSoc::Social_Inequality`
- **Difficulty:** 4
- **Question:** Population studies repeatedly find that as socioeconomic status rises, average health outcomes improve in a stepwise fashion across the entire status range. This pattern is best described as what?
- **A:** The social gradient in health
- **B:** A purely genetic effect
- **C:** The placebo effect
- **D:** Random measurement error
- **Correct:** A
- **Explanation:** The social gradient in health is the consistent finding that health improves with each step up the socioeconomic ladder, reflecting the social determinants of health.
- **Tags:** `KC::PsychSoc::Health_Disparities` `Prereq::PsychSoc::Social_Class` `Prereq::PsychSoc::Social_Inequality` `MCAT::Psych_Soc` `Difficulty::4`

### MCAT-PSY-HEAD-002

- **KC:** `PsychSoc::Health_Disparities`
- **Prereqs:** `Prereq::PsychSoc::Social_Class` `Prereq::PsychSoc::Social_Inequality`
- **Difficulty:** 3
- **Question:** Conditions such as income, education, neighborhood safety, and access to nutritious food strongly shape a population's health outcomes. Collectively, these conditions are known as what?
- **A:** The social determinants of health
- **B:** The placebo effect
- **C:** The germ theory of disease
- **D:** The sick role
- **Correct:** A
- **Explanation:** The social determinants of health are the social and economic conditions, such as income, education, and environment, that shape health outcomes across populations.
- **Tags:** `KC::PsychSoc::Health_Disparities` `Prereq::PsychSoc::Social_Class` `Prereq::PsychSoc::Social_Inequality` `MCAT::Psych_Soc` `Difficulty::3`

### MCAT-PSY-HEAD-003

- **KC:** `PsychSoc::Health_Disparities`
- **Prereqs:** `Prereq::PsychSoc::Social_Class` `Prereq::PsychSoc::Social_Inequality`
- **Difficulty:** 5
- **Question:** Researchers propose that the chronic stress of coping with discrimination and disadvantage gradually wears down the body, contributing to earlier health decline among marginalized groups. This proposed mechanism is best labeled what?
- **A:** The weathering hypothesis
- **B:** The demographic transition
- **C:** The Flynn effect
- **D:** Cultural lag
- **Correct:** A
- **Explanation:** The weathering hypothesis proposes that cumulative exposure to social and economic adversity causes accelerated biological aging and health deterioration in disadvantaged groups.
- **Tags:** `KC::PsychSoc::Health_Disparities` `Prereq::PsychSoc::Social_Class` `Prereq::PsychSoc::Social_Inequality` `MCAT::Psych_Soc` `Difficulty::5`

## PsychSoc::Healthcare_Disparities

### MCAT-PSY-HEAD2-001

- **KC:** `PsychSoc::Healthcare_Disparities`
- **Prereqs:** `Prereq::PsychSoc::Health_Disparities` `Prereq::PsychSoc::Social_Institutions`
- **Difficulty:** 4
- **Question:** Two groups have the same disease prevalence, but one group receives fewer diagnostic tests and lower-quality treatment because of limited insurance coverage and clinic availability. This difference is best classified as what?
- **A:** A healthcare disparity in access and quality
- **B:** A difference in underlying disease incidence
- **C:** A placebo response
- **D:** A demographic transition
- **Correct:** A
- **Explanation:** Healthcare disparities are differences in access to and quality of healthcare services across groups, distinct from disparities in the underlying health outcomes themselves.
- **Tags:** `KC::PsychSoc::Healthcare_Disparities` `Prereq::PsychSoc::Health_Disparities` `Prereq::PsychSoc::Social_Institutions` `MCAT::Psych_Soc` `Difficulty::4`

### MCAT-PSY-HEAD2-002

- **KC:** `PsychSoc::Healthcare_Disparities`
- **Prereqs:** `Prereq::PsychSoc::Health_Disparities` `Prereq::PsychSoc::Social_Institutions`
- **Difficulty:** 3
- **Question:** Studies show that clinicians' unconscious associations can lead them to offer less aggressive pain treatment to patients of certain racial groups even when symptoms are identical. This contributor to healthcare disparities is best described as what?
- **A:** Implicit bias among providers
- **B:** A true difference in disease incidence
- **C:** The placebo effect
- **D:** The demographic transition
- **Correct:** A
- **Explanation:** Implicit, unconscious bias among healthcare providers can produce unequal treatment decisions even when clinical presentations are identical, driving healthcare disparities.
- **Tags:** `KC::PsychSoc::Healthcare_Disparities` `Prereq::PsychSoc::Health_Disparities` `Prereq::PsychSoc::Social_Institutions` `MCAT::Psych_Soc` `Difficulty::3`

### MCAT-PSY-HEAD2-003

- **KC:** `PsychSoc::Healthcare_Disparities`
- **Prereqs:** `Prereq::PsychSoc::Health_Disparities` `Prereq::PsychSoc::Social_Institutions`
- **Difficulty:** 5
- **Question:** A rural county has no nearby hospital, few specialists, and unreliable public transportation, so residents delay care until conditions worsen. These obstacles are best categorized as what?
- **A:** Structural barriers to healthcare access
- **B:** A difference in genetic susceptibility
- **C:** The social gradient in health
- **D:** Anticipatory socialization
- **Correct:** A
- **Explanation:** Structural barriers such as provider shortages, facility distance, and lack of transportation limit access to care and contribute to healthcare disparities independent of underlying disease rates.
- **Tags:** `KC::PsychSoc::Healthcare_Disparities` `Prereq::PsychSoc::Health_Disparities` `Prereq::PsychSoc::Social_Institutions` `MCAT::Psych_Soc` `Difficulty::5`
