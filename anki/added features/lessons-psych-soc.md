# Psychology & Sociology Lessons (Track C — scaled draft)

Auto-drafted lesson pages for the 34 `PsychSoc::` KCs in the frozen unified map (`added features/kc-map-unified.md` §6/§7). Schema + style follow `added features/lessons.md`. All content is synthetic and original (not copied from any copyrighted prep material). New lessons are `Source: authored`, `Review Status: needs_review` (hidden behind the display gate until a human approves them, per `lesson-contract.md` §4).

## PsychSoc::Biological_and_Social_Factors

### LESSON-PSYCHSOC-BIOLOGICAL-AND-SOCIAL-FACTORS

- **KC:** `PsychSoc::Biological_and_Social_Factors`
- **Title:** Biological and Social Factors: Nature, Nurture, and Behavior
- **Section:** `MCAT::Psych_Soc`
- **Source:** authored
- **Review Status:** needs_review
- **Overview:** Behavior emerges from the interaction of biological systems
  (genes, hormones, the nervous system) and social context (family, culture,
  experience). This KC frames the biopsychosocial lens the rest of Psych/Soc
  relies on, so it sits upstream of emotion, motivation, personality, and
  disorders. The point is not nature versus nurture but how the two
  continuously shape each other.
- **Key Concepts:**
  - The nervous system, especially the brain, is the proximate substrate of
    behavior; damage or neurotransmitter changes shift behavior directly.
  - Endocrine signals (e.g., cortisol, testosterone) modulate mood, stress, and
    motivation on a slower timescale than neural firing.
  - Genes set predispositions, but heritability describes population variance,
    not an individual's fixed destiny.
  - Environment and experience regulate whether and how genetic and hormonal
    potentials are expressed (gene-environment interaction).
- **Prerequisite Reminder:** Build on `Bio::Nervous_System`,
  `Bio::Endocrine_System`, and `Bio::Genetics` - this KC reuses their neural,
  hormonal, and heritability machinery as the biological half of behavior.
- **Worked Example:** Two siblings share a similar genetic risk for anxiety. One
  grows up in a calm, predictable home and stays subclinical; the other faces
  chronic instability and develops an anxiety disorder. Same predisposition,
  different environments, different outcomes - a clean illustration that biology
  loads the odds while environment often decides the outcome.
- **Common Misconception:** "A heritability of 0.5 means half of my trait is
  caused by my genes." This is the individual-vs-population confusion:
  heritability is a population statistic about the share of variation across
  people, not a within-person split, and it says nothing about how much of one
  person's trait is genetic.
- **First Retrieval Prompt:** From memory, explain in one or two sentences why
  "heritable" does not mean "unchangeable," using a behavior of your choice.
- **Related KCs:** `Bio::Endocrine_System`, `Bio::Genetics`,
  `Bio::Nervous_System`, `PsychSoc::Learning`, `PsychSoc::Emotion`,
  `PsychSoc::Motivation`, `PsychSoc::Personality`,
  `PsychSoc::Psychological_Disorders`

## PsychSoc::Sensory_Processing

### LESSON-PSYCHSOC-SENSORY-PROCESSING

- **KC:** `PsychSoc::Sensory_Processing`
- **Title:** Sensory Processing: From Stimulus to Neural Signal
- **Section:** `MCAT::Psych_Soc`
- **Source:** authored
- **Review Status:** needs_review
- **Overview:** Sensory processing is how physical energy - light, sound,
  pressure, chemicals - is detected by receptors and converted into neural
  signals the brain can use. It is the ground floor of the sensation-perception
  pipeline, so every later KC in this cluster assumes it. The key idea is
  transduction: converting one kind of energy into action potentials.
- **Key Concepts:**
  - Transduction converts environmental stimuli into electrochemical signals at
    specialized receptors.
  - An absolute threshold is the minimum stimulus intensity detectable 50% of
    the time; a difference threshold (JND) is the smallest detectable change.
  - Weber's law: the JND is a constant proportion of the original stimulus, not
    a fixed amount.
  - Sensory adaptation reduces responsiveness to a constant, unchanging
    stimulus.
- **Prerequisite Reminder:** Build on `Bio::Nervous_System`: receptors feed
  sensory neurons and afferent pathways, so recall how a stimulus becomes an
  action potential that travels to the brain.
- **Worked Example:** Add one candle to a room already lit by one candle and the
  change is obvious; add one candle to a room lit by sixty and you notice
  nothing. Weber's law explains this - detectability depends on the proportional
  change, so a fixed one-candle increment is easy against a dim background and
  invisible against a bright one.
- **Common Misconception:** "Sensation and perception are the same thing." This
  is the sensation-perception conflation: sensation is the bottom-up detection
  and transduction of stimuli, whereas perception is the brain's later
  organization and interpretation. This KC is only the detection step.
- **First Retrieval Prompt:** Without looking back, define transduction and give
  one everyday example of sensory adaptation.
- **Related KCs:** `Bio::Nervous_System`, `PsychSoc::The_Senses`,
  `PsychSoc::Attention`

## PsychSoc::The_Senses

### LESSON-PSYCHSOC-THE-SENSES

- **KC:** `PsychSoc::The_Senses`
- **Title:** The Senses: Modalities and Their Pathways
- **Section:** `MCAT::Psych_Soc`
- **Source:** authored
- **Review Status:** needs_review
- **Overview:** This KC covers the specific sensory systems - vision, hearing,
  taste, smell, touch, and the body senses - and how each transduces its
  stimulus. It builds directly on general sensory processing by adding
  modality-specific receptors and pathways. Knowing which receptor and pathway
  serves which stimulus is the load-bearing content.
- **Key Concepts:**
  - Vision: photoreceptors (rods for dim light and motion, cones for color and
    acuity) transduce light; signals travel via the optic nerve to visual
    cortex.
  - Hearing: hair cells in the cochlea transduce sound-wave vibrations, and
    place along the basilar membrane codes pitch.
  - Chemical senses: gustation and olfaction transduce dissolved and airborne
    molecules; smell projects to areas tied to memory and emotion.
  - Somatosensation includes touch, temperature, pain, and proprioception
    (body position).
- **Prerequisite Reminder:** Build on `PsychSoc::Sensory_Processing`: each
  modality here is a specific case of transduction and thresholds, so keep the
  receptor-to-signal idea in mind.
- **Worked Example:** In dim light you see a moving object at the edge of your
  vision but cannot make out its color. Peripheral, low-light vision relies on
  rods, which are sensitive but do not code color; color needs cones, which
  cluster in the fovea and require more light. Same scene, different receptors
  doing the work.
- **Common Misconception:** "We taste the five basic tastes on separate, fixed
  zones of the tongue." This tongue-map myth misreads older data: receptors for
  sweet, salty, sour, bitter, and umami are distributed across the tongue rather
  than confined to zones.
- **First Retrieval Prompt:** From memory, state which photoreceptor dominates in
  dim light and why that predicts poor color vision at night.
- **Related KCs:** `PsychSoc::Sensory_Processing`, `PsychSoc::Perception`

## PsychSoc::Attention

### LESSON-PSYCHSOC-ATTENTION

- **KC:** `PsychSoc::Attention`
- **Title:** Attention: Selecting What Reaches Awareness
- **Section:** `MCAT::Psych_Soc`
- **Source:** authored
- **Review Status:** needs_review
- **Overview:** Attention is the mechanism that selects some information for
  deeper processing while filtering the rest. It gates what reaches perception
  and cognition, which is why it feeds both. The central tension is between
  selective focus and our limited capacity to divide it.
- **Key Concepts:**
  - Selective attention focuses on one stream while filtering others (the
    "cocktail party" effect).
  - Divided attention splits limited resources across tasks and usually degrades
    performance.
  - Inattentional blindness: unattended but fully visible events can go
    unnoticed.
  - Filter-theory debates concern whether unattended input is blocked early or
    late in processing.
- **Prerequisite Reminder:** Build on `PsychSoc::Sensory_Processing`: attention
  operates on already-transduced sensory input, choosing which signals get
  further processing.
- **Worked Example:** At a loud party you track one conversation and tune out the
  rest - until someone across the room says your name and it grabs you. Selective
  attention filters by default, yet a highly salient, self-relevant signal still
  breaks through, which is evidence that unattended input is monitored at least
  somewhat rather than fully blocked.
- **Common Misconception:** "Multitasking means doing two demanding tasks at once,
  efficiently." The multitasking myth ignores that, for attention-demanding
  tasks, we mostly task-switch and pay a switching cost each time; true parallel
  processing is limited to well-automated tasks.
- **First Retrieval Prompt:** Without looking back, explain why texting while
  driving is dangerous in terms of divided attention and switching costs.
- **Related KCs:** `PsychSoc::Sensory_Processing`, `PsychSoc::Perception`,
  `PsychSoc::Cognition`, `PsychSoc::Consciousness`

## PsychSoc::Perception

### LESSON-PSYCHSOC-PERCEPTION

- **KC:** `PsychSoc::Perception`
- **Title:** Perception: Organizing and Interpreting Sensory Input
- **Section:** `MCAT::Psych_Soc`
- **Source:** authored
- **Review Status:** needs_review
- **Overview:** Perception is the brain's construction of a meaningful world from
  raw sensory signals, combining bottom-up data with top-down expectations. It
  sits just above the senses and attention and feeds cognition. The core insight
  is that perception is an active interpretation, not a passive recording.
- **Key Concepts:**
  - Bottom-up processing builds percepts from sensory features; top-down
    processing uses prior knowledge and context to interpret them.
  - Gestalt principles (proximity, similarity, closure, continuity) describe how
    we group elements into wholes.
  - Perceptual constancies keep size, shape, and color stable despite changing
    retinal images.
  - Signal detection theory separates true sensitivity from response bias
    (hits, misses, false alarms).
- **Prerequisite Reminder:** Build on `PsychSoc::The_Senses` and
  `PsychSoc::Attention`: perception organizes the specific sensory signals that
  attention has selected for processing.
- **Worked Example:** You read a smudged "th_" in "the cat sat" and instantly
  perceive "the." The bottom-up input is ambiguous, but top-down context supplies
  the missing letter. A tired radiologist expecting a normal scan might likewise
  miss a faint tumor - top-down expectation shaping what is perceived, for better
  or worse.
- **Common Misconception:** "Perception is just a camera-like copy of what's out
  there." This naive-realism error overlooks that the brain constructs perception
  from expectations, context, and prior learning, which is why illusions reliably
  fool everyone.
- **First Retrieval Prompt:** From memory, contrast bottom-up and top-down
  processing using a single example where context changes what you perceive.
- **Related KCs:** `PsychSoc::Attention`, `PsychSoc::The_Senses`,
  `PsychSoc::Cognition`

## PsychSoc::Cognition

### LESSON-PSYCHSOC-COGNITION

- **KC:** `PsychSoc::Cognition`
- **Title:** Cognition: Thinking, Problem Solving, and Decision Making
- **Section:** `MCAT::Psych_Soc`
- **Source:** authored
- **Review Status:** needs_review
- **Overview:** Cognition covers how we mentally represent information and use it
  to reason, solve problems, and make decisions. It is the hub of this cluster,
  drawing on perception and attention and feeding memory, language, and social
  judgment. A central theme is that efficient shortcuts (heuristics) trade
  accuracy for speed.
- **Key Concepts:**
  - Concepts and prototypes organize knowledge; problem solving uses algorithms
    (guaranteed) or heuristics (fast but fallible).
  - Common heuristics: availability (judging by ease of recall) and
    representativeness (judging by resemblance to a prototype).
  - Biases include confirmation bias, anchoring, and framing effects.
  - Dual-process accounts contrast fast, automatic thinking with slow, effortful
    reasoning.
- **Prerequisite Reminder:** Build on `PsychSoc::Attention` and
  `PsychSoc::Perception`: reasoning operates on the organized, attended
  information those steps deliver.
- **Worked Example:** Asked whether more English words start with "k" or have "k"
  as the third letter, most people say the first - because words starting with
  "k" come to mind faster. That is the availability heuristic (ease of recall
  standing in for actual frequency), not the representativeness heuristic
  (judging by similarity to a category). Telling the two apart is a common trap.
- **Common Misconception:** "Heuristics are just errors or lazy thinking." This
  heuristics-are-bugs error misses that heuristics are adaptive shortcuts that
  are usually efficient and often correct; biases are the predictable failure
  modes that appear when a shortcut misfires.
- **First Retrieval Prompt:** Without looking back, give one scenario each for the
  availability and representativeness heuristics and say how they differ.
- **Related KCs:** `PsychSoc::Attention`, `PsychSoc::Perception`,
  `PsychSoc::Memory`, `PsychSoc::Consciousness`, `PsychSoc::Language`,
  `PsychSoc::Intelligence`, `PsychSoc::Attitudes_and_Beliefs`,
  `PsychSoc::Stereotypes`

## PsychSoc::Memory

### LESSON-PSYCHSOC-MEMORY

- **KC:** `PsychSoc::Memory`
- **Title:** Memory: Encoding, Storage, and Retrieval
- **Section:** `MCAT::Psych_Soc`
- **Source:** authored
- **Review Status:** needs_review
- **Overview:** Memory is the set of processes that encode, store, and retrieve
  information over time. It builds on cognition and reaches the high end of the
  difficulty ladder because it spans multiple models and biological detail. The
  organizing frame is the three stages (encoding, storage, retrieval) and the
  distinct memory stores.
- **Key Concepts:**
  - Stage model: sensory memory to short-term/working memory (limited, seconds)
    to long-term memory (vast, durable).
  - Long-term memory splits into explicit/declarative (facts, events) and
    implicit/nondeclarative (skills, conditioning).
  - Encoding strategies (elaboration, chunking, spacing) and retrieval cues
    strongly affect later recall.
  - Forgetting arises from decay, interference (proactive and retroactive), and
    retrieval failure; memory is reconstructive and can be distorted.
- **Prerequisite Reminder:** Build on `PsychSoc::Cognition`: memory stores and
  manipulates the representations that thinking creates, and working memory is
  where active cognition happens.
- **Worked Example:** You study for two exams back to back, and the first
  subject's terms keep intruding when you try to recall the second. That is
  proactive interference (old learning disrupting new). If instead studying the
  second subject made you lose access to the first, that would be retroactive
  interference. Naming the direction correctly is the usual test point.
- **Common Misconception:** "Memory works like a video recording you can replay."
  The memory-as-recording myth ignores that retrieval is reconstructive -
  memories are rebuilt from fragments and can be reshaped by suggestion, which is
  why eyewitness reports are error-prone.
- **First Retrieval Prompt:** From memory, distinguish proactive from retroactive
  interference with one example of each.
- **Related KCs:** `PsychSoc::Cognition`, `PsychSoc::Intelligence`

## PsychSoc::Consciousness

### LESSON-PSYCHSOC-CONSCIOUSNESS

- **KC:** `PsychSoc::Consciousness`
- **Title:** Consciousness: Awareness, Sleep, and Altered States
- **Section:** `MCAT::Psych_Soc`
- **Source:** authored
- **Review Status:** needs_review
- **Overview:** Consciousness is our awareness of ourselves and our environment,
  including how it varies across sleep, drugs, and other altered states. It draws
  on attention and cognition, since what we are aware of depends on what we
  process. Much of the testable content is the architecture of sleep and the
  effects of psychoactive drugs.
- **Key Concepts:**
  - Sleep cycles through NREM stages and REM (vivid dreaming, muscle atonia)
    roughly every 90 minutes.
  - Circadian rhythms are regulated by light and melatonin; disruption impairs
    mood and cognition.
  - Psychoactive drugs fall into depressants, stimulants, and hallucinogens,
    each shifting neural activity differently.
  - Tolerance, dependence, and withdrawal describe the body's adaptation to
    repeated drug use.
- **Prerequisite Reminder:** Build on `PsychSoc::Attention` and
  `PsychSoc::Cognition`: levels of awareness reflect how attention and higher
  processing are engaged or suppressed.
- **Worked Example:** A person wakes easily during the night, yet a partner
  reports they seemed to be dreaming vividly. The dreaming points to REM sleep,
  and the ease of arousal fits REM's paradoxically active, wake-like brain
  state - unlike deep NREM (slow-wave) sleep, from which waking is hard and
  grogginess is common. Matching the symptom to the stage is the skill.
- **Common Misconception:** "The brain shuts down and rests during sleep." The
  sleep-as-shutdown myth is wrong: the sleeping brain is highly active,
  especially in REM, and sleep is an organized process that consolidates memory
  rather than a simple power-off.
- **First Retrieval Prompt:** Without looking back, name the sleep stage most
  associated with vivid dreaming and one feature that distinguishes it from deep
  sleep.
- **Related KCs:** `PsychSoc::Attention`, `PsychSoc::Cognition`

## PsychSoc::Language

### LESSON-PSYCHSOC-LANGUAGE

- **KC:** `PsychSoc::Language`
- **Title:** Language: Structure and Acquisition
- **Section:** `MCAT::Psych_Soc`
- **Source:** authored
- **Review Status:** needs_review
- **Overview:** Language is the rule-governed system for combining symbols into
  meaning, together with how humans acquire it. It builds on cognition because
  language both expresses and shapes thought. The testable core is the hierarchy
  of language structure and the major theories of how children learn it.
- **Key Concepts:**
  - Language is organized hierarchically: phonemes, morphemes, semantics,
    syntax, and pragmatics.
  - Acquisition follows a universal sequence (babbling, one-word, telegraphic
    speech) on a rough timeline.
  - Nativist (Chomsky, universal grammar) and learning/interactionist accounts
    explain acquisition differently.
  - The Whorfian (linguistic relativity) hypothesis proposes language influences
    thought, in a weaker modern form.
- **Prerequisite Reminder:** Build on `PsychSoc::Cognition`: language uses and
  structures the concepts and representations that cognition provides.
- **Worked Example:** A three-year-old says "I goed to the park." The child has
  never heard "goed," so this cannot be pure imitation; it is overregularization
  - applying the "-ed" past-tense rule too broadly. That supports rule-learning
  accounts of acquisition over a strict imitation-only view, a classic piece of
  evidence to recognize.
- **Common Misconception:** "Children learn language mainly by imitating and being
  rewarded for correct speech." This imitation-only account cannot explain errors
  like "goed" or "foots," which show children extract and apply grammatical rules
  they were never explicitly taught.
- **First Retrieval Prompt:** From memory, explain what a child's error like
  "goed" reveals about how language is acquired.
- **Related KCs:** `PsychSoc::Cognition`, `PsychSoc::Intelligence`

## PsychSoc::Intelligence

### LESSON-PSYCHSOC-INTELLIGENCE

- **KC:** `PsychSoc::Intelligence`
- **Title:** Intelligence: Theories, Testing, and Heritability
- **Section:** `MCAT::Psych_Soc`
- **Source:** authored
- **Review Status:** needs_review
- **Overview:** Intelligence covers how we define, measure, and explain
  individual differences in cognitive ability. As an integrative KC it pulls
  together cognition, memory, and language, and it runs to the top of the
  difficulty ladder through debates about testing and heritability. The
  high-yield content is the competing theories and the properties of good tests.
- **Key Concepts:**
  - Theories range from a general factor (g) to multiple intelligences and
    Sternberg's triarchic model.
  - Good tests require reliability (consistency) and validity (measuring what is
    intended), with standardized norms.
  - IQ is defined relative to a normed population, and the distribution is
    roughly normal.
  - Both heredity and environment contribute; group score gaps are heavily
    shaped by environment and testing conditions, not fixed biology.
- **Prerequisite Reminder:** Build on `PsychSoc::Cognition`, `PsychSoc::Memory`,
  and `PsychSoc::Language`: intelligence measures and integrates these
  underlying capacities.
- **Worked Example:** A new "genius test" gives you a different score each time
  you take it in one week. However impressive it looks, it fails reliability
  (consistency), so its scores cannot be valid. A test can be reliable without
  being valid (consistently measuring the wrong thing), but it cannot be valid
  without first being reliable - a distinction worth reasoning through on
  test-critique questions.
- **Common Misconception:** "IQ is a fixed, purely genetic number that captures
  how smart someone is." The fixed-IQ fallacy ignores that IQ is a relative,
  environmentally sensitive estimate of certain abilities; it can shift with
  schooling, health, and test conditions and does not capture all forms of
  ability.
- **First Retrieval Prompt:** Without looking back, explain why a test must be
  reliable to be valid but reliability alone is not enough.
- **Related KCs:** `PsychSoc::Cognition`, `PsychSoc::Language`,
  `PsychSoc::Memory`

## PsychSoc::Learning

### LESSON-PSYCHSOC-LEARNING

- **KC:** `PsychSoc::Learning`
- **Title:** Learning: Conditioning and Observational Learning
- **Section:** `MCAT::Psych_Soc`
- **Source:** authored
- **Review Status:** needs_review
- **Overview:** Learning is a relatively lasting change in behavior or knowledge
  that results from experience. It builds on the biological and social
  foundations and starts at the easy end with conditioning basics before climbing
  to reinforcement schedules. The core distinctions are classical vs operant
  conditioning and observational learning.
- **Key Concepts:**
  - Classical conditioning: a neutral stimulus paired with an unconditioned
    stimulus comes to trigger a conditioned response (Pavlov).
  - Operant conditioning: consequences shape behavior via reinforcement
    (increases) and punishment (decreases), positive (adding) or negative
    (removing).
  - Reinforcement schedules (fixed/variable, ratio/interval) produce
    characteristic response patterns; variable-ratio resists extinction most.
  - Observational learning (Bandura) acquires behavior by watching models,
    without direct reinforcement.
- **Prerequisite Reminder:** Build on
  `PsychSoc::Biological_and_Social_Factors`: learning depends on neural
  plasticity and is shaped by social models and context.
- **Worked Example:** A slot machine pays out after an unpredictable, varying
  number of pulls, and players keep going even after losses. That is a
  variable-ratio schedule, which yields high, steady, extinction-resistant
  responding. Contrast a paycheck every two weeks (fixed interval), which
  produces a very different, deadline-shaped pattern. Matching schedule to
  behavior pattern is the frequent test task.
- **Common Misconception:** "Negative reinforcement is the same as punishment."
  This negative-reinforcement mix-up forgets that negative reinforcement removes
  an aversive stimulus to increase a behavior, while punishment decreases a
  behavior; "negative" means something is taken away, not that the outcome is
  bad.
- **First Retrieval Prompt:** From memory, define negative reinforcement and give
  one example that is clearly not punishment.
- **Related KCs:** `PsychSoc::Biological_and_Social_Factors`,
  `PsychSoc::Attitudes_and_Beliefs`

## PsychSoc::Emotion

### LESSON-PSYCHSOC-EMOTION

- **KC:** `PsychSoc::Emotion`
- **Title:** Emotion: Components and Theories
- **Section:** `MCAT::Psych_Soc`
- **Source:** authored
- **Review Status:** needs_review
- **Overview:** Emotion is a coordinated response with physiological, behavioral,
  and cognitive components. It builds on the biological and social foundations
  and feeds motivation, stress, and disorders. The high-yield material is the
  competing theories of how bodily arousal and conscious feeling relate.
- **Key Concepts:**
  - Emotion has three components: physiological arousal, expressive behavior,
    and subjective/cognitive experience.
  - James-Lange: the bodily response comes first, and the feeling is the
    interpretation of it.
  - Cannon-Bard: arousal and the felt emotion occur simultaneously and
    independently.
  - Schachter-Singer two-factor: emotion requires arousal plus a cognitive label
    of that arousal.
- **Prerequisite Reminder:** Build on
  `PsychSoc::Biological_and_Social_Factors`: emotions arise from autonomic and
  limbic activity shaped by context and appraisal.
- **Worked Example:** Your heart pounds after a near-miss in traffic, and you
  label the feeling "fear." James-Lange says the pounding heart came first and
  fear is your read of it; Schachter-Singer adds that the same arousal could be
  labeled "excitement" in a different context, such as a roller coaster. The
  theories differ in whether a cognitive label is required, which is the usual
  discrimination asked.
- **Common Misconception:** "You feel the emotion first, and that makes your body
  react." This feeling-first assumption is reversed or paralleled by major
  theories: bodily arousal can precede or accompany the felt emotion, and
  interpretation of arousal helps determine which emotion you experience.
- **First Retrieval Prompt:** Without looking back, contrast the James-Lange and
  Schachter-Singer theories on the role of cognitive labeling.
- **Related KCs:** `PsychSoc::Biological_and_Social_Factors`,
  `PsychSoc::Motivation`, `PsychSoc::Stress`,
  `PsychSoc::Psychological_Disorders`

## PsychSoc::Motivation

### LESSON-PSYCHSOC-MOTIVATION

- **KC:** `PsychSoc::Motivation`
- **Title:** Motivation: Drives, Needs, and Incentives
- **Section:** `MCAT::Psych_Soc`
- **Source:** authored
- **Review Status:** needs_review
- **Overview:** Motivation is what initiates, directs, and sustains goal-directed
  behavior. It builds on the biological and social foundations and on emotion,
  since feelings often energize action. The testable core is the major theories
  and the intrinsic-extrinsic distinction.
- **Key Concepts:**
  - Drive-reduction theory: physiological needs create drives that behavior
    reduces to restore homeostasis.
  - Arousal theory (Yerkes-Dodson): performance peaks at an intermediate arousal
    level, and that optimum is lower for hard tasks.
  - Maslow's hierarchy orders needs from physiological to self-actualization.
  - Intrinsic motivation (internal satisfaction) differs from extrinsic
    (external rewards); over-rewarding can undermine intrinsic interest.
- **Prerequisite Reminder:** Build on
  `PsychSoc::Biological_and_Social_Factors` and `PsychSoc::Emotion`: motivation
  reflects homeostatic and hormonal drives plus the pushes and pulls of emotion.
- **Worked Example:** A student who loves painting is paid per finished canvas and
  later paints less once payments stop. This is the overjustification effect: an
  external reward crowded out intrinsic motivation. Drive-reduction theory
  struggles here because no physiological need is at stake, which is why the
  intrinsic-extrinsic framework is the better lens - a useful theory-selection
  move.
- **Common Misconception:** "More reward always produces more motivation." This
  more-reward-is-better assumption ignores that extrinsic rewards can reduce
  intrinsic motivation (overjustification), and that beyond an optimal point
  higher arousal or pressure can worsen performance.
- **First Retrieval Prompt:** From memory, explain the overjustification effect
  and predict what happens to a hobby once you start being paid for it.
- **Related KCs:** `PsychSoc::Biological_and_Social_Factors`,
  `PsychSoc::Emotion`

## PsychSoc::Stress

### LESSON-PSYCHSOC-STRESS

- **KC:** `PsychSoc::Stress`
- **Title:** Stress: Appraisal, Physiology, and Coping
- **Section:** `MCAT::Psych_Soc`
- **Source:** authored
- **Review Status:** needs_review
- **Overview:** Stress is the response to demands appraised as taxing or
  exceeding one's resources. As an application KC it integrates emotion with
  endocrine physiology and feeds psychological disorders. The key content is
  appraisal, the stress-response systems, and coping strategies.
- **Key Concepts:**
  - Primary appraisal judges whether a stressor is threatening; secondary
    appraisal judges whether one can cope.
  - The acute response is sympathetic "fight or flight"; the sustained response
    runs through the HPA axis and cortisol.
  - Selye's general adaptation syndrome: alarm, resistance, exhaustion.
  - Chronic stress harms immune, cardiovascular, and mental health;
    problem-focused and emotion-focused coping differ in aim.
- **Prerequisite Reminder:** Build on `PsychSoc::Emotion` and
  `Bio::Endocrine_System`: stress couples emotional appraisal to the HPA axis and
  cortisol release.
- **Worked Example:** Two students face the same hard exam. One appraises it as
  "a challenge I prepared for" and feels energized; the other appraises it as "a
  threat I can't handle" and feels overwhelmed. Identical stressor, opposite
  responses - the difference is cognitive appraisal, not the event itself. This
  is why stress is treated as an interaction between person and situation.
- **Common Misconception:** "Stress is caused directly by stressful events." This
  stimulus-only view forgets that the same event can be energizing or
  debilitating depending on appraisal and coping resources; stress lives in the
  person-situation interaction, not the event alone.
- **First Retrieval Prompt:** Without looking back, explain how primary and
  secondary appraisal can make the same event stressful for one person and not
  another.
- **Related KCs:** `Bio::Endocrine_System`, `PsychSoc::Emotion`,
  `PsychSoc::Psychological_Disorders`

## PsychSoc::Personality

### LESSON-PSYCHSOC-PERSONALITY

- **KC:** `PsychSoc::Personality`
- **Title:** Personality: Traits and Theoretical Perspectives
- **Section:** `MCAT::Psych_Soc`
- **Source:** authored
- **Review Status:** needs_review
- **Overview:** Personality is the characteristic, relatively stable pattern of
  thinking, feeling, and behaving that distinguishes individuals. It builds on
  the biological and social foundations and feeds self and identity. The
  high-yield content is the major theoretical perspectives and the trait
  approach.
- **Key Concepts:**
  - Trait theories describe stable dimensions; the Big Five (OCEAN) is the
    dominant empirical model.
  - Psychoanalytic theory emphasizes unconscious conflict (id, ego, superego).
  - Humanistic theory emphasizes growth, self-concept, and self-actualization.
  - Social-cognitive theory stresses reciprocal interaction of person, behavior,
    and environment.
- **Prerequisite Reminder:** Build on
  `PsychSoc::Biological_and_Social_Factors`: temperament has biological roots
  that interact with experience to shape personality.
- **Worked Example:** Someone consistently plans ahead, meets deadlines, and
  keeps an organized space. In Big Five terms this is high conscientiousness, an
  empirically measured trait. A psychoanalytic account would instead invoke
  unconscious drives, and a humanistic one would focus on the person's
  self-concept - choosing which framework a question is testing is the core skill
  here.
- **Common Misconception:** "Personality 'types' put people in fixed boxes that
  predict behavior well." This type-over-trait error ignores that evidence favors
  continuous traits over discrete types, and that behavior also depends heavily
  on the situation, so single "type" labels predict behavior poorly.
- **First Retrieval Prompt:** From memory, list the Big Five dimensions and say
  why traits are considered continuous rather than all-or-none types.
- **Related KCs:** `PsychSoc::Biological_and_Social_Factors`,
  `PsychSoc::Self_and_Identity`

## PsychSoc::Self_and_Identity

### LESSON-PSYCHSOC-SELF-AND-IDENTITY

- **KC:** `PsychSoc::Self_and_Identity`
- **Title:** Self and Identity: Self-Concept and Social Identity
- **Section:** `MCAT::Psych_Soc`
- **Source:** authored
- **Review Status:** needs_review
- **Overview:** This KC covers how people form a sense of self and identity
  through internal beliefs and social context. It integrates personality with
  socialization, since identity is built with others. The testable content spans
  self-concept, self-esteem, self-efficacy, and identity development.
- **Key Concepts:**
  - Self-concept is the set of beliefs about oneself; self-esteem is the
    evaluative feeling about it.
  - Self-efficacy (Bandura) is the belief in one's ability to succeed at a
    specific task.
  - Identity forms through stages/statuses (e.g., Erikson, Marcia) and through
    group membership (social identity).
  - The looking-glass self and reference groups show identity is shaped by how we
    think others see us.
- **Prerequisite Reminder:** Build on `PsychSoc::Personality` and
  `PsychSoc::Socialization`: identity emerges as stable traits meet the social
  feedback and roles that socialization provides.
- **Worked Example:** A student believes "I'm generally a capable person"
  (self-concept and self-esteem) yet says "I doubt I can pass organic chemistry"
  (low self-efficacy for that task). The two can diverge: self-efficacy is
  task-specific while self-esteem is global. Spotting that a question is about a
  specific-task belief, not global self-worth, is the discrimination being
  tested.
- **Common Misconception:** "Self-esteem and self-efficacy are the same thing."
  This esteem-efficacy conflation misses that self-esteem is a global evaluation
  of self-worth while self-efficacy is task-specific confidence, and a person can
  be high on one and low on the other.
- **First Retrieval Prompt:** Without looking back, distinguish self-esteem from
  self-efficacy with one example where they diverge.
- **Related KCs:** `PsychSoc::Personality`, `PsychSoc::Socialization`,
  `PsychSoc::Social_Interaction`

## PsychSoc::Psychological_Disorders

### LESSON-PSYCHSOC-PSYCHOLOGICAL-DISORDERS

- **KC:** `PsychSoc::Psychological_Disorders`
- **Title:** Psychological Disorders: Classification and Models
- **Section:** `MCAT::Psych_Soc`
- **Source:** authored
- **Review Status:** needs_review
- **Overview:** This KC covers how mental disorders are defined, classified, and
  explained. It integrates the biological and social foundations with emotion and
  stress, and runs to the top of the difficulty ladder through diagnostic
  reasoning. The core content is the major categories and the biopsychosocial and
  diathesis-stress models.
- **Key Concepts:**
  - Disorders are patterns causing distress or dysfunction, classified (e.g., in
    the DSM) by symptom clusters.
  - Major categories include anxiety, mood (depression, bipolar), schizophrenia,
    and personality disorders.
  - The biopsychosocial model integrates biological, psychological, and social
    causes.
  - The diathesis-stress model: a predisposition (diathesis) plus stress
    triggers onset.
- **Prerequisite Reminder:** Build on
  `PsychSoc::Biological_and_Social_Factors`, `PsychSoc::Emotion`, and
  `PsychSoc::Stress`: disorders reflect biological vulnerability interacting with
  emotional processes and stressors.
- **Worked Example:** Two people carry a similar genetic vulnerability to
  depression. One encounters a major loss and develops the disorder; the other,
  with strong support and low stress, does not. That is the diathesis-stress
  model: predisposition alone may be insufficient, and stress alone may be
  insufficient, but their combination crosses the threshold. Reasoning about the
  interaction, not either factor alone, is what disorder questions reward.
- **Common Misconception:** "Mental disorders have a single cause - usually a
  brain chemical imbalance." This single-cause oversimplification ignores that
  most disorders are multifactorial, best captured by biopsychosocial and
  diathesis-stress models rather than one neurotransmitter story.
- **First Retrieval Prompt:** From memory, use the diathesis-stress model to
  explain why two people with the same predisposition can have different
  outcomes.
- **Related KCs:** `PsychSoc::Biological_and_Social_Factors`,
  `PsychSoc::Emotion`, `PsychSoc::Stress`

## PsychSoc::Attitudes_and_Beliefs

### LESSON-PSYCHSOC-ATTITUDES-AND-BELIEFS

- **KC:** `PsychSoc::Attitudes_and_Beliefs`
- **Title:** Attitudes and Beliefs: Formation and Change
- **Section:** `MCAT::Psych_Soc`
- **Source:** authored
- **Review Status:** needs_review
- **Overview:** Attitudes are evaluations of people, objects, or ideas, and this
  KC covers how they form and change. It builds on cognition and learning and
  feeds stereotypes, prejudice, and social interaction. The high-yield content is
  the components of attitudes and the routes to persuasion.
- **Key Concepts:**
  - Attitudes have affective, behavioral, and cognitive components (the ABC
    model).
  - Attitudes predict behavior best when specific, strong, and free of
    situational pressure.
  - Cognitive dissonance: conflict between attitude and behavior motivates
    attitude change to reduce discomfort.
  - Elaboration likelihood model: central (argument-based) vs peripheral
    (cue-based) routes to persuasion.
- **Prerequisite Reminder:** Build on `PsychSoc::Cognition` and
  `PsychSoc::Learning`: attitudes are learned evaluations built from, and
  reasoned about with, cognitive processes.
- **Worked Example:** Someone who dislikes a chore agrees to do it for very little
  pay, then reports actually enjoying it. With no large external reward to
  justify the behavior, they reduce the attitude-behavior conflict by changing
  the attitude - classic cognitive dissonance. Note this works only when external
  justification is weak; a big payment would let them keep the original attitude
  ("I did it for the money").
- **Common Misconception:** "Attitudes reliably predict behavior." This
  attitude-behavior consistency assumption is often weak; the link strengthens
  only when attitudes are specific, strong, and situational pressures are low,
  and behavior can also change attitudes (dissonance).
- **First Retrieval Prompt:** Without looking back, explain how cognitive
  dissonance predicts attitude change after acting against one's attitude for
  little reward.
- **Related KCs:** `PsychSoc::Cognition`, `PsychSoc::Learning`,
  `PsychSoc::Stereotypes`, `PsychSoc::Prejudice_and_Bias`,
  `PsychSoc::Social_Interaction`

## PsychSoc::Stereotypes

### LESSON-PSYCHSOC-STEREOTYPES

- **KC:** `PsychSoc::Stereotypes`
- **Title:** Stereotypes: Cognitive Bases of Social Categorization
- **Section:** `MCAT::Psych_Soc`
- **Source:** authored
- **Review Status:** needs_review
- **Overview:** Stereotypes are generalized beliefs about the attributes of a
  social group. This KC builds on cognition and attitudes and feeds prejudice and
  bias. The key idea is that stereotyping is a cognitive categorization process
  that can operate automatically and shape behavior.
- **Key Concepts:**
  - Stereotypes are the cognitive component of intergroup bias; prejudice is the
    affective and discrimination the behavioral component.
  - Categorization is efficient but overgeneralizes and ignores within-group
    variation.
  - Stereotype threat: awareness of a negative stereotype can impair performance
    in that domain.
  - Self-fulfilling prophecy: expectations can elicit the very behavior they
    predict.
- **Prerequisite Reminder:** Build on `PsychSoc::Attitudes_and_Beliefs` and
  `PsychSoc::Cognition`: stereotypes are category-based cognitions layered onto
  group attitudes.
- **Worked Example:** A capable student reminded of a negative stereotype about
  their group right before a test underperforms relative to a control group. This
  is stereotype threat: the anxiety and self-monitoring triggered by the
  stereotype, not any real deficit, drives the drop. Distinguishing this from a
  genuine ability difference is exactly the interpretive trap in study-based
  questions.
- **Common Misconception:** "Stereotype, prejudice, and discrimination are
  interchangeable words." This conflation ignores that a stereotype is the belief
  (cognitive), prejudice is the feeling (affective), and discrimination is the
  action (behavioral); a person can hold one without expressing the others.
- **First Retrieval Prompt:** From memory, place stereotype, prejudice, and
  discrimination in the cognitive-affective-behavioral framework.
- **Related KCs:** `PsychSoc::Attitudes_and_Beliefs`, `PsychSoc::Cognition`,
  `PsychSoc::Prejudice_and_Bias`

## PsychSoc::Prejudice_and_Bias

### LESSON-PSYCHSOC-PREJUDICE-AND-BIAS

- **KC:** `PsychSoc::Prejudice_and_Bias`
- **Title:** Prejudice and Bias: Intergroup Attitudes in Context
- **Section:** `MCAT::Psych_Soc`
- **Source:** authored
- **Review Status:** needs_review
- **Overview:** Prejudice is a negative attitude toward a group, and bias
  includes the systematic distortions that sustain it. As an application KC
  bridging psychology and sociology, it integrates attitudes and stereotypes with
  group and structural context. The high-yield content is the sources of
  prejudice and how it is reduced.
- **Key Concepts:**
  - Prejudice is the affective component of intergroup bias; discrimination is
    its behavioral expression.
  - Sources include in-group/out-group dynamics, ethnocentrism, and competition
    for resources (realistic conflict).
  - Individual bias (implicit and explicit) interacts with
    institutional/structural discrimination.
  - The contact hypothesis: cooperative, equal-status contact toward shared goals
    can reduce prejudice.
- **Prerequisite Reminder:** Build on `PsychSoc::Attitudes_and_Beliefs` and
  `PsychSoc::Stereotypes`: prejudice is the negative-attitude side of group bias
  built on stereotyped beliefs.
- **Worked Example:** A hiring manager sincerely reports no prejudice yet
  consistently rates identical resumes lower when they carry a name associated
  with an out-group. This is implicit bias producing discrimination without
  conscious animus, and it can be amplified by institutional practices. It shows
  that reducing prejudice takes more than good intentions - a common reasoning
  point on structural-versus-individual questions.
- **Common Misconception:** "Prejudice is always a conscious, deliberate
  dislike." This explicit-only view ignores that much bias is implicit and
  automatic, and that discrimination can be produced by institutions and norms
  even when individuals report no prejudice.
- **First Retrieval Prompt:** Without looking back, state the contact hypothesis
  and the conditions contact must meet to reduce prejudice.
- **Related KCs:** `PsychSoc::Attitudes_and_Beliefs`, `PsychSoc::Stereotypes`

## PsychSoc::Social_Interaction

### LESSON-PSYCHSOC-SOCIAL-INTERACTION

- **KC:** `PsychSoc::Social_Interaction`
- **Title:** Social Interaction: Self-Presentation and Influence
- **Section:** `MCAT::Psych_Soc`
- **Source:** authored
- **Review Status:** needs_review
- **Overview:** Social interaction covers how people act on and influence one
  another, from first impressions to helping and aggression. It integrates self
  and identity with attitudes and feeds group behavior. The high-yield content is
  attribution, self-presentation, and the major social-influence phenomena.
- **Key Concepts:**
  - Attribution theory: we explain behavior via internal (dispositional) or
    external (situational) causes.
  - The fundamental attribution error overweights disposition for others'
    behavior; actor-observer and self-serving biases shift this.
  - Self-presentation and impression management (dramaturgy) shape how we appear
    to others.
  - Social influence includes conformity, compliance, and obedience.
- **Prerequisite Reminder:** Build on `PsychSoc::Attitudes_and_Beliefs` and
  `PsychSoc::Self_and_Identity`: interaction is where attitudes and identity are
  enacted and negotiated with others.
- **Worked Example:** A driver cuts you off and you think "what a jerk"
  (disposition), but when you cut someone off you think "I had to, I'm late"
  (situation). That asymmetry is the actor-observer bias, a relative of the
  fundamental attribution error. Correctly labeling which attribution bias a
  scenario shows - and from whose perspective - is the recurring test task.
- **Common Misconception:** "People's behavior mostly reflects their personality,
  so the situation is secondary." This is the fundamental attribution error
  itself: observers systematically underestimate situational forces and
  overattribute others' behavior to character.
- **First Retrieval Prompt:** From memory, define the fundamental attribution
  error and give an everyday example of overattributing behavior to disposition.
- **Related KCs:** `PsychSoc::Attitudes_and_Beliefs`,
  `PsychSoc::Self_and_Identity`, `PsychSoc::Group_Behavior`

## PsychSoc::Group_Behavior

### LESSON-PSYCHSOC-GROUP-BEHAVIOR

- **KC:** `PsychSoc::Group_Behavior`
- **Title:** Group Behavior: Processes in and Between Groups
- **Section:** `MCAT::Psych_Soc`
- **Source:** authored
- **Review Status:** needs_review
- **Overview:** Group behavior covers how being in a group changes individual
  behavior and how groups make decisions. Bridging psychology and sociology, it
  integrates social interaction with socialization. The high-yield content is the
  classic group-process phenomena and how to distinguish them.
- **Key Concepts:**
  - Social facilitation vs social loafing: others' presence can boost performance
    on easy tasks but reduce individual effort in groups.
  - Deindividuation: reduced self-awareness in groups can loosen normal
    restraints.
  - Groupthink and group polarization distort group decision-making.
  - Conformity (Asch), obedience (Milgram), and the bystander effect show
    situational power over behavior.
- **Prerequisite Reminder:** Build on `PsychSoc::Social_Interaction` and
  `PsychSoc::Socialization`: group behavior scales up interpersonal influence
  within the norms socialization instills.
- **Worked Example:** A person who would help someone in distress when alone does
  nothing as part of a large, passive crowd. This is the bystander effect via
  diffusion of responsibility - the more witnesses, the less any one person feels
  obligated. It is distinct from conformity (matching others to fit in); here the
  key mechanism is spread-out responsibility, which is the distinction questions
  probe.
- **Common Misconception:** "There is safety in numbers, so more bystanders means
  help is more likely." This safety-in-numbers assumption is reversed by the
  bystander effect: larger groups can reduce the likelihood any individual helps,
  because responsibility diffuses and people look to others to act.
- **First Retrieval Prompt:** Without looking back, explain the bystander effect
  and why it is driven by diffusion of responsibility rather than simple
  conformity.
- **Related KCs:** `PsychSoc::Social_Interaction`, `PsychSoc::Socialization`

## PsychSoc::Social_Theory

### LESSON-PSYCHSOC-SOCIAL-THEORY

- **KC:** `PsychSoc::Social_Theory`
- **Title:** Social Theory: Sociological Paradigms
- **Section:** `MCAT::Psych_Soc`
- **Source:** authored
- **Review Status:** needs_review
- **Overview:** Social theory provides the major paradigms sociologists use to
  explain how society works. As a root KC of the sociology cluster, it underlies
  institutions, stratification, and inequality. The load-bearing content is the
  handful of theoretical lenses and the level of analysis each takes.
- **Key Concepts:**
  - Functionalism: society is a system of interdependent parts that maintain
    stability (macro level).
  - Conflict theory: society is shaped by competition over scarce resources and
    power (macro level).
  - Symbolic interactionism: society is built from everyday meanings and
    interactions (micro level).
  - Social constructionism and other lenses (exchange, feminist theory) add
    further perspectives.
- **Prerequisite Reminder:** Foundation KC - no prerequisites assumed beyond a
  general sense of what "society" and "social structure" mean.
- **Worked Example:** Consider public education. A functionalist asks how schools
  maintain society (teaching skills and shared values); a conflict theorist asks
  how schools reproduce inequality (tracking, unequal funding); a symbolic
  interactionist asks how teacher-student labels shape identities in the
  classroom. Same institution, three levels of analysis - matching a question's
  framing to the right paradigm is the essential skill.
- **Common Misconception:** "One sociological theory is correct and the others are
  wrong." This one-true-paradigm error misses that the paradigms are
  complementary lenses at different levels of analysis; the exam tests choosing
  the lens that fits a scenario, not ranking them as true or false.
- **First Retrieval Prompt:** From memory, contrast how functionalism and conflict
  theory would each explain the existence of social stratification.
- **Related KCs:** `PsychSoc::Social_Institutions`, `PsychSoc::Stratification`

## PsychSoc::Culture

### LESSON-PSYCHSOC-CULTURE

- **KC:** `PsychSoc::Culture`
- **Title:** Culture: Shared Meanings and Practices
- **Section:** `MCAT::Psych_Soc`
- **Source:** authored
- **Review Status:** needs_review
- **Overview:** Culture is the shared beliefs, values, norms, symbols, and
  material objects of a group. As a foundational sociology KC it underlies
  socialization, institutions, and demographics. The core content is the
  components of culture and how it is transmitted and varies.
- **Key Concepts:**
  - Material culture (objects) vs nonmaterial culture (values, norms, beliefs,
    symbols, language).
  - Norms range from folkways (mild) to mores and taboos (strong); sanctions
    enforce them.
  - Ethnocentrism (judging others by one's own culture) vs cultural relativism
    (understanding on their terms).
  - Culture is transmitted through socialization and can change via diffusion and
    innovation.
- **Prerequisite Reminder:** Foundation KC - no prerequisites assumed beyond
  everyday familiarity with customs and norms.
- **Worked Example:** Eating with your hands is normal and polite in some cultures
  and frowned upon in others. Judging the practice as "bad manners" by your own
  standards is ethnocentrism; understanding it within its own cultural context is
  cultural relativism. Recognizing which stance a passage takes is a frequent,
  low-difficulty test point.
- **Common Misconception:** "Culture just means art, music, and food." This
  culture-as-the-arts narrowing is too small: sociologically, culture is the
  whole system of shared norms, values, symbols, and material objects that guides
  behavior.
- **First Retrieval Prompt:** Without looking back, distinguish material from
  nonmaterial culture and give one example of each.
- **Related KCs:** `PsychSoc::Socialization`, `PsychSoc::Social_Institutions`,
  `PsychSoc::Demographics`

## PsychSoc::Socialization

### LESSON-PSYCHSOC-SOCIALIZATION

- **KC:** `PsychSoc::Socialization`
- **Title:** Socialization: Learning Society's Norms
- **Section:** `MCAT::Psych_Soc`
- **Source:** authored
- **Review Status:** needs_review
- **Overview:** Socialization is the lifelong process by which people learn and
  internalize their culture's norms, values, and roles. It builds on culture and
  feeds self and identity and group behavior. The key content is the agents of
  socialization and the primary-vs-secondary distinction.
- **Key Concepts:**
  - Primary socialization (childhood, mainly family) vs secondary socialization
    (later, via school, peers, media, work).
  - Agents of socialization include family, peers, schools, media, and religion.
  - Resocialization replaces old norms with new ones, sometimes in total
    institutions.
  - Norms and roles are internalized so that much social behavior becomes taken
    for granted.
- **Prerequisite Reminder:** Build on `PsychSoc::Culture`: socialization is the
  mechanism by which a culture's norms and values are transmitted to individuals.
- **Worked Example:** A new military recruit gives up civilian habits and adopts
  strict routines, dress, and hierarchy in a controlled environment. This is
  resocialization within a total institution: old norms are stripped and new ones
  installed. Contrast a child learning table manners at home (primary
  socialization) - the same broad process at a very different intensity and life
  stage, which is what questions test.
- **Common Misconception:** "Socialization ends when you grow up." This
  childhood-only view is wrong: socialization is lifelong, and adults are
  resocialized by new jobs, roles, and institutions well beyond childhood.
- **First Retrieval Prompt:** From memory, distinguish primary from secondary
  socialization and name two agents of each.
- **Related KCs:** `PsychSoc::Culture`, `PsychSoc::Self_and_Identity`,
  `PsychSoc::Group_Behavior`

## PsychSoc::Social_Institutions

### LESSON-PSYCHSOC-SOCIAL-INSTITUTIONS

- **KC:** `PsychSoc::Social_Institutions`
- **Title:** Social Institutions: Organized Patterns Meeting Social Needs
- **Section:** `MCAT::Psych_Soc`
- **Source:** authored
- **Review Status:** needs_review
- **Overview:** Social institutions are the enduring structures - family,
  education, religion, government, economy, medicine - that organize how societies
  meet basic needs. This KC integrates social theory and culture and feeds
  demographics and healthcare disparities. The core content is what each
  institution does and how theories interpret them.
- **Key Concepts:**
  - Institutions are stable, patterned sets of norms and roles serving
    recognized social functions.
  - Major institutions: family, education, religion, government/economy, and
    medicine.
  - Functionalist, conflict, and interactionist lenses interpret each
    institution differently.
  - Institutions can produce both social order and the reproduction of
    inequality.
- **Prerequisite Reminder:** Build on `PsychSoc::Culture` and
  `PsychSoc::Social_Theory`: institutions are culture organized into durable
  structures, read through sociological paradigms.
- **Worked Example:** Take medicine as an institution. A functionalist highlights
  the "sick role" that legitimizes stepping back from duties while one recovers;
  a conflict theorist highlights unequal access and the power of medical
  gatekeepers. Same institution, two lenses - and choosing the lens that matches
  the question's angle is the recurring application skill.
- **Common Misconception:** "An institution is a specific building or
  organization, like one particular hospital." This institution-as-building error
  confuses a single organization with the abstract, enduring pattern of norms and
  roles (medicine, education) that the term actually names.
- **First Retrieval Prompt:** Without looking back, define a social institution
  and explain how it differs from a single organization.
- **Related KCs:** `PsychSoc::Culture`, `PsychSoc::Social_Theory`,
  `PsychSoc::Demographics`, `PsychSoc::Healthcare_Disparities`

## PsychSoc::Demographics

### LESSON-PSYCHSOC-DEMOGRAPHICS

- **KC:** `PsychSoc::Demographics`
- **Title:** Demographics: Population Structure and Change
- **Section:** `MCAT::Psych_Soc`
- **Source:** authored
- **Review Status:** needs_review
- **Overview:** Demographics is the study of population characteristics and how
  they change over time. It integrates culture and institutions and feeds social
  inequality. The high-yield content is the demographic variables, the tools that
  summarize them, and the theories of population change.
- **Key Concepts:**
  - Core demographic variables: age, sex/gender, race/ethnicity, immigration
    status, and sexual orientation.
  - Population change is driven by fertility, mortality, and migration.
  - Demographic transition theory links industrialization to falling death then
    birth rates.
  - Tools include population pyramids and rates (birth, death, growth).
- **Prerequisite Reminder:** Build on `PsychSoc::Culture` and
  `PsychSoc::Social_Institutions`: demographic patterns reflect cultural values
  (for example, around family) enacted through institutions.
- **Worked Example:** A wealthy country shows a population pyramid that is narrow
  at the base and wide in the middle and top. That shape signals low fertility
  and an aging population, consistent with a late stage of the demographic
  transition. Reading the shape to infer fertility, aging, and likely future
  dependency ratios is exactly what data-interpretation questions ask.
- **Common Misconception:** "'Demographic transition' just means a population is
  growing." This growth-only reading misses that the transition describes a
  staged shift in birth and death rates with development; late stages can mean
  slowing or shrinking, not simple growth.
- **First Retrieval Prompt:** From memory, name the three drivers of population
  change and say what a bottom-heavy population pyramid implies about fertility.
- **Related KCs:** `PsychSoc::Culture`, `PsychSoc::Social_Institutions`,
  `PsychSoc::Social_Inequality`

## PsychSoc::Stratification

### LESSON-PSYCHSOC-STRATIFICATION

- **KC:** `PsychSoc::Stratification`
- **Title:** Stratification: Systems of Social Ranking
- **Section:** `MCAT::Psych_Soc`
- **Source:** authored
- **Review Status:** needs_review
- **Overview:** Stratification is the structured ranking of groups in a society by
  access to resources, power, and prestige. It builds on social theory and feeds
  class, mobility, poverty, and inequality. The core content is the dimensions of
  stratification and the open-vs-closed distinction.
- **Key Concepts:**
  - Weber's dimensions: economic class, social status (prestige), and political
    power.
  - Systems range from closed (caste, ascribed status) to open (class, achieved
    status).
  - Stratification tends to be reproduced across generations (e.g., inherited
    advantage).
  - Functionalist (Davis-Moore) and conflict explanations disagree on whether it
    is necessary or exploitative.
- **Prerequisite Reminder:** Build on `PsychSoc::Social_Theory`: stratification is
  analyzed through the functionalist and conflict paradigms that theory supplies.
- **Worked Example:** In a caste system, position is assigned at birth and
  movement is essentially barred (ascribed status, closed system); in a class
  system, education or work can shift position (achieved status, open system).
  Placing a described society on the open-closed continuum, and naming the status
  type, is the standard mechanism question here.
- **Common Misconception:** "Stratification is only about money and income." This
  income-only view forgets that Weber showed stratification is multidimensional -
  prestige and power can diverge from wealth (a revered but poorly paid clergy
  member, or wealth without social status).
- **First Retrieval Prompt:** Without looking back, list Weber's three dimensions
  of stratification and contrast an open with a closed system.
- **Related KCs:** `PsychSoc::Social_Theory`, `PsychSoc::Social_Class`,
  `PsychSoc::Social_Mobility`, `PsychSoc::Social_Inequality`

## PsychSoc::Social_Class

### LESSON-PSYCHSOC-SOCIAL-CLASS

- **KC:** `PsychSoc::Social_Class`
- **Title:** Social Class: Position and Its Consequences
- **Section:** `MCAT::Psych_Soc`
- **Source:** authored
- **Review Status:** needs_review
- **Overview:** Social class is a person's position in the economic stratification
  system and the life outcomes tied to it. It builds on stratification and feeds
  mobility, poverty, and health disparities. The high-yield content is how class
  is measured and how it shapes life chances.
- **Key Concepts:**
  - Socioeconomic status (SES) combines income, education, and occupation.
  - Class shapes "life chances": health, education, and opportunity.
  - Cultural and social capital (Bourdieu) transmit class advantage beyond money.
  - Class boundaries can be reproduced through schools, networks, and
    neighborhoods.
- **Prerequisite Reminder:** Build on `PsychSoc::Stratification`: class is the
  position an individual occupies within the broader stratification system.
- **Worked Example:** Two equally talented students apply to college; one has
  parents who know the process, edit essays, and fund test prep (cultural and
  social capital), the other does not. Even with equal ability, the first is
  advantaged - showing class works through capital, not just cash. Recognizing
  capital as the mechanism, rather than income alone, is the applied point.
- **Common Misconception:** "Class is purely about how much money you make." This
  income-equals-class error ignores education, occupation, and cultural/social
  capital, which is why two people with similar incomes can occupy different class
  positions.
- **First Retrieval Prompt:** From memory, explain how cultural or social capital
  can reproduce class advantage even when incomes are similar.
- **Related KCs:** `PsychSoc::Stratification`, `PsychSoc::Social_Mobility`,
  `PsychSoc::Poverty`, `PsychSoc::Health_Disparities`

## PsychSoc::Social_Mobility

### LESSON-PSYCHSOC-SOCIAL-MOBILITY

- **KC:** `PsychSoc::Social_Mobility`
- **Title:** Social Mobility: Movement Between Positions
- **Section:** `MCAT::Psych_Soc`
- **Source:** authored
- **Review Status:** needs_review
- **Overview:** Social mobility is the movement of individuals or groups between
  positions in the stratification system. It builds on class and stratification.
  The core content is the types of mobility and what enables or blocks them.
- **Key Concepts:**
  - Vertical (up or down) vs horizontal mobility; intergenerational vs
    intragenerational.
  - Structural mobility results from society-wide changes (e.g., new industries),
    not individual effort alone.
  - Meritocracy is the ideal that position reflects ability and effort; reality
    is constrained by starting position.
  - Mobility is limited by reproduction of advantage across generations.
- **Prerequisite Reminder:** Build on `PsychSoc::Social_Class` and
  `PsychSoc::Stratification`: mobility is change in one's class position within
  the stratification structure.
- **Worked Example:** During rapid industrialization, millions move from farm
  labor into factory and office jobs and rise in status. Because a shift in the
  economy - not just personal effort - lifted them, this is structural mobility.
  Distinguishing it from individual upward mobility due to personal achievement is
  the classic type-identification task.
- **Common Misconception:** "Upward mobility is simply a matter of individual hard
  work." This pure-meritocracy assumption overlooks that much mobility is
  structural (driven by economic change) and is constrained by inherited
  advantage, so effort alone does not determine outcomes.
- **First Retrieval Prompt:** Without looking back, define structural mobility and
  give an example that is not explained by individual effort.
- **Related KCs:** `PsychSoc::Social_Class`, `PsychSoc::Stratification`

## PsychSoc::Poverty

### LESSON-PSYCHSOC-POVERTY

- **KC:** `PsychSoc::Poverty`
- **Title:** Poverty: Measurement and Persistence
- **Section:** `MCAT::Psych_Soc`
- **Source:** authored
- **Review Status:** needs_review
- **Overview:** Poverty is the condition of lacking the resources for a minimum
  standard of living, and this KC covers how it is measured and why it persists.
  It builds on social class. The high-yield content is absolute vs relative
  poverty and the structural drivers of persistence.
- **Key Concepts:**
  - Absolute poverty (below a fixed subsistence line) vs relative poverty (below
    the society's typical standard).
  - The poverty line and poverty rates are measurement conventions with
    limitations.
  - Social reproduction and the "cycle of poverty" transmit disadvantage across
    generations.
  - Structural causes (labor markets, discrimination, policy) complement
    individual explanations.
- **Prerequisite Reminder:** Build on `PsychSoc::Social_Class`: poverty is the low
  end of the class structure and shares its capital and life-chances mechanisms.
- **Worked Example:** A family has enough to eat but cannot afford internet,
  transportation, or activities peers take for granted, limiting opportunities.
  They may sit above an absolute subsistence line yet be in relative poverty for
  their society. Deciding which definition a scenario invokes - and why relative
  poverty still restricts life chances - is the applied distinction.
- **Common Misconception:** "Poverty is mainly the result of individual choices or
  effort." This individual-blame (just-world) view underweights the structural
  drivers - labor markets, discrimination, and social reproduction - that sustain
  poverty independent of personal effort.
- **First Retrieval Prompt:** From memory, contrast absolute and relative poverty
  and give a case that counts as relative but not absolute poverty.
- **Related KCs:** `PsychSoc::Social_Class`

## PsychSoc::Social_Inequality

### LESSON-PSYCHSOC-SOCIAL-INEQUALITY

- **KC:** `PsychSoc::Social_Inequality`
- **Title:** Social Inequality: Patterns Across Groups and Space
- **Section:** `MCAT::Psych_Soc`
- **Source:** authored
- **Review Status:** needs_review
- **Overview:** Social inequality is the uneven distribution of resources and
  opportunities produced by stratification, viewed across groups, places, and
  their intersections. As an integrative KC it builds on stratification and
  demographics and feeds health disparities, running to the top of the difficulty
  ladder. The high-yield content is spatial and intersectional patterns and how
  inequality is maintained.
- **Key Concepts:**
  - Inequality is patterned by class, race, gender, and their intersections
    (intersectionality).
  - Spatial inequality includes residential segregation and the
    urban/suburban/rural divide (e.g., environmental justice).
  - Global inequality compares life chances across nations, not just within
    them.
  - Inequality is maintained by institutions, policy, and social reproduction,
    not only individual acts.
- **Prerequisite Reminder:** Build on `PsychSoc::Demographics` and
  `PsychSoc::Stratification`: inequality is the demographic pattern of outcomes
  that the stratification mechanism produces.
- **Worked Example:** A low-income neighborhood has more pollution, fewer grocery
  stores, and worse-funded schools, and its residents are disproportionately from
  one racial group. The overlap of class, race, and place is intersectional and
  spatial inequality - no single factor tells the story. Reasoning about how
  multiple axes compound, rather than treating them separately, is what
  high-difficulty questions reward.
- **Common Misconception:** "Inequality is just individual differences in income
  adding up." This individual-differences reduction misses that inequality is
  structured and intersectional - patterned by group and place and reproduced by
  institutions - so it is not merely a sum of independent personal outcomes.
- **First Retrieval Prompt:** Without looking back, explain what intersectionality
  adds beyond looking at class, race, or gender one at a time.
- **Related KCs:** `PsychSoc::Demographics`, `PsychSoc::Stratification`,
  `PsychSoc::Health_Disparities`

## PsychSoc::Health_Disparities

### LESSON-PSYCHSOC-HEALTH-DISPARITIES

- **KC:** `PsychSoc::Health_Disparities`
- **Title:** Health Disparities: Unequal Health Outcomes
- **Section:** `MCAT::Psych_Soc`
- **Source:** authored
- **Review Status:** needs_review
- **Overview:** Health disparities are differences in health outcomes tied to
  social position - class, race, gender, and place. As an integrative KC it builds
  on social class and inequality and feeds healthcare disparities, at the top of
  the difficulty ladder. The core content is the social determinants of health
  and the outcome-vs-access distinction.
- **Key Concepts:**
  - Social determinants of health: income, education, environment, and social
    support shape outcomes.
  - The social gradient: health improves stepwise with rising socioeconomic
    status.
  - Disparities appear as differences in morbidity and mortality across groups.
  - Distinguish health outcomes (disparities in disease/death) from
    access/quality (healthcare disparities).
- **Prerequisite Reminder:** Build on `PsychSoc::Social_Class` and
  `PsychSoc::Social_Inequality`: health disparities are inequality expressed in
  bodies and outcomes across class positions.
- **Worked Example:** Across income brackets, each step up is associated with
  better average health - not just the poorest being sick. This social gradient
  shows health tracks social position continuously, implicating determinants like
  stress, environment, and resources rather than a single cause. Interpreting the
  gradient, rather than a simple poor-vs-rich split, is the reasoning the top of
  the ladder asks for.
- **Common Misconception:** "Health disparities come down to individual behaviors
  and genetics." This behavior-and-biology-only view underweights social
  determinants (income, education, environment, discrimination), as the
  continuous social gradient across all income levels shows.
- **First Retrieval Prompt:** From memory, explain the social gradient in health
  and why it argues against a purely individual explanation.
- **Related KCs:** `PsychSoc::Social_Class`, `PsychSoc::Social_Inequality`,
  `PsychSoc::Healthcare_Disparities`

## PsychSoc::Healthcare_Disparities

### LESSON-PSYCHSOC-HEALTHCARE-DISPARITIES

- **KC:** `PsychSoc::Healthcare_Disparities`
- **Title:** Healthcare Disparities: Access, Quality, and the System
- **Section:** `MCAT::Psych_Soc`
- **Source:** authored
- **Review Status:** needs_review
- **Overview:** Healthcare disparities are inequalities in access to and quality
  of medical care, distinct from disparities in health outcomes themselves. It
  builds on health disparities and social institutions, at the top of the
  difficulty ladder. The high-yield content is the access/quality lens and the
  barriers built into the healthcare system.
- **Key Concepts:**
  - The focus is the healthcare system: access, insurance coverage, quality, and
    treatment differences.
  - Barriers include cost, insurance status, geography, and provider bias.
  - Medicine as an institution can reproduce inequality through unequal access
    and treatment.
  - Healthcare disparities contribute to, but are distinct from, population
    health disparities.
- **Prerequisite Reminder:** Build on `PsychSoc::Health_Disparities` and
  `PsychSoc::Social_Institutions`: this KC narrows to the care system, the
  institutional channel through which health outcomes are shaped.
- **Worked Example:** Two patients have the same chest-pain symptoms; one is
  insured and lives near a hospital, the other is uninsured in a rural area and
  delays care. The gap here is in access to and quality of care (a healthcare
  disparity), which can then produce a worse outcome (a health disparity).
  Keeping the cause (system access) separate from the effect (outcome) is exactly
  the distinction these questions test.
- **Common Misconception:** "Health disparities and healthcare disparities are the
  same thing." This outcome-access conflation misses that health disparities are
  differences in outcomes (who gets sick or dies), while healthcare disparities
  are differences in access to and quality of care - a system-level cause that
  must be separated from the outcomes it influences.
- **First Retrieval Prompt:** Without looking back, distinguish healthcare
  disparities from health disparities and give one system-level barrier to care.
- **Related KCs:** `PsychSoc::Health_Disparities`,
  `PsychSoc::Social_Institutions`
