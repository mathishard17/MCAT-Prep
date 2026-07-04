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
