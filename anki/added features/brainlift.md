# Brain Lift: Brainstorming for MCAT Prep App

## Purpose

Research on how to accurately measure and efficiently increase ability on the MCAT. Focuses on the underlying graph like structure of learning combined with the best proven study methods of flashcards and practice problems.

## Scope

### In Scope

• MCAT
• Reading comprehension/critical thinking
• Memory
• Flashcards, ANKI
• Practice Problems
• NN based Models
• Timing

### Out of Scope

• AI chatbots
• Gamification
• Providing Motivation
• Specific science concepts in MCAT
• Lectures, learning everything from scratch

## DOK 4: Spiky POVs

### SPOV 1: Memory Structure Determines Long-term Ability

• So, build the structures well now, or else nothing you do right now will matter in a few months.
• **Elaboration:** The way we learn directly impacts how our brains build up the structures in our memory. With overly detailed flashcards, in order to remember them efficiently, our brains will create a bunch of disconnected nodes of extreme precise information. Yet by working through multi-step complex practice problems, our brains have to retain the information through a systematic graph of concepts, problems, approaches.
• **Opposite POV:** You can cram and can perform well on a test.
• **Rebuttal:** After the test, it’s all gone. The brain forgets and you don’t continue to retrieve.
• **Opposite POV:** You can put a ton of effort in learning and still do poorly on tests.
• **Rebuttal:** Because you are learning the wrong way. You are not building long-lasting and stable mental structures. When you just reread, summarize even, you are not (re)building connections and (re)activating nodes you initially created.

### SPOV 2: An EduLLM Must Be Graph-Based

• **Elaboration:** Firstly, the best way to learn things is not linearly. Prerequisites should not be strict. And our limits are infinite in all directions. In order for an EduLLM to be the most efficient to help humans learn, it must truly understand schemas, the both forward and backward propagation of information through all the nodes, and the flexibility to continue to grow as a network of knowledge components. Neural nets are not bad, but far from enough.
• **Opposite POV:** Transformers work very well.
• **Rebuttal:** When you prompt an LLM to generate a full concept map, it can only generate some topics, and subtopics, and the connections as it outputs. For a large map, after some time when it’s writing a new topic, there’s no way it can attend back to and connect it with the first topic. This attention structure will always be bias towards the most recent tokens. But our brains don’t work like that.

## DOK 3: Insights

• **Insight 1:** The way we learn directly affects the way memory structures are built in our minds
• **Insight 2:** The metric the underlying learning system should optimize for is the same as what standardized tests try to estimate, which is the ability, θ, across all topics.
• **Insight 3:** Learning done in one topic not only affects the ability within that topic, but also in all related topics, from those that were already learned to those that need the topic has a prerequisite.
• **Insight 4:** For modeling learning, it’s essential to have a network/schema structure directly ingrained into the system.

## DOK 2: Summaries

### Memory -> Performance

• For learning knowledge, flashcards are an effective way to learn
• forces active recall, retrieval
• allows interleaving & spaced repetition
• yet, solely memorizing discrete facts prohibits full picture schema structures from being built
• general concept level flashcards improves structure building and performance
• for people capable of building good structures, the difference is negligible between concept level and detailed cards
• MCAT takers introduce a variety of strategies to pair with ANKI flashcards of reviewing chapter by chapter (with reading), only pressing good/again buttons, creating flashcards for missed practice problems, review every single day. However, these shouldn’t be taken as seriously as the real learning science experiments since may be bias. But some people complain it’s slow and feels ineffective (and give up after 1 week)
• Transfer of learning bottleneck: remembered facts don’t directly translate to good problem solving skills
• For learning, practice problems are also of the most effective strategies
• also forces active recall through complex scenarios
• develops flexible schemas, allows transferring of knowledge
• mimics the real test so improvement feels more immediate
• allows for better understanding of the AAMC problem styles
• but too goal focused can cause means-end analysis and overload cognitive working capacity
• more exploratory type guidance can improve topic understanding
• worked examples, solutions vs blindly grinding through problems (to reach goal)
• It is essential to also have distributed practice (3-4 months to prepare)

### Performance -> Readiness

Accurate metrics to measure the user’s abilities
• real MCAT score is based on Item Response Theory + normalizing
• through leveraging difficulty, discrimination, and guessing, for getting estimate of ability
• maximum likelihood across multiple practice problems for better accuracy
• can also be found by “simulating” the learner walking through knowledge lattice, but these approaches are better for…

### Choosing What to Learn Next

• This should be based on current ability on a certain topic
• Different approaches differ in the structure of knowledge, strictness of prereqs, and computational load needed
• But roughly, similar to schemas (from Schema theory), the knowledge is put into the network containing nodes and connections
• nodes are smaller knowledge components
• connections are roughly about prerequisites
• These structures can also be flexible and have updates propagate through the entire network (Bayesian network models), or more rigid with binary mastered/unmastered (Knowledge Space Theory)
• there is a distinction of material mastered, and new topics ready to be learned, inner and outer fringes, to allow retrieval of old concepts
• as well as distinctions in underlying causes of mistakes to be guessing, slip up, true mistake
• human vs data-trained models curation of learning
• need to maintain balance between effectiveness of learning + knowing our own capabilities and having control
• human/ai manual labeling vs having sufficient data for accurate model
• is the model architecture flexible for edge cases? graph neural nets allow higher accuracy for local progress/sparse inputs vs regular neural net

## DOK 1: New Facts 7/1 (gpt subagent swarm)

### Joe Liemandt / Alpha Mastery Lens

• **Finite daily finish line:** Alpha’s product mechanic is not just AI tutoring; students complete a focused academic block and then get “time back” for workshops. **Why it matters:** The MCAT app should define a daily “green state” instead of creating an infinite Anki-style backlog. **Source:** [Alpha School, AI-Powered K-12 Learning in 2 Hours](https://alpha.school/the-program/)
• **High mastery gate:** Alpha publicly frames advancement around high mastery thresholds, roughly 90%+. **Why it matters:** MCAT sequencing should gate new material by demonstrated mastery, but with probabilistic confidence instead of raw percent correct. **Source:** [Joe Liemandt and the Future of Education](https://alpha.school/joe-liemandt-and-the-future-of-education/)
• **Bloom’s 2-sigma effect required correction loops:** The famous tutoring result depended on formative tests, corrective procedures, and parallel retests, not tutoring alone. **Why it matters:** A missed MCAT problem should trigger correction plus a follow-up retest on the same underlying skill. **Source:** [Bloom, “The 2 Sigma Problem”](https://journals.sagepub.com/doi/10.3102/0013189X013006004)
• **Mastery learning can cost too much time:** Meta-analysis found mastery learning helps, especially weaker students, but self-paced mastery can increase time cost and lower completion. **Why it matters:** The app needs deadline-aware mastery so users do not get trapped remediating forever before the exam. **Source:** [Kulik, Kulik & Bangert-Drowns, “Effectiveness of Mastery Learning Programs”](https://journals.sagepub.com/doi/10.3102/00346543060002265)
• **Readiness needs uncertainty bands:** AAMC reports MCAT section scores with confidence bands because scores are imperfect estimates. **Why it matters:** The app should show score range/probability of hitting target, not fake precision. **Source:** [AAMC, Understanding Your MCAT Score Report](https://students-residents.aamc.org/mcat-scores/understanding-your-mcat-score-report)

### Elon Musk / First-Principles Systems Lens

• **Knowledge as a semantic tree:** Musk’s learning advice is to understand the trunk and big branches before the leaves/details. **Why it matters:** The MCAT graph should protect foundational nodes before detail-heavy flashcards, because details need a structure to hang on. **Source:** [Elon Musk Reddit AMA Transcript](https://amatranscripts.com/ama/elon_musk_2015-01-05.html)
• **Reason from fundamentals, not analogy:** Musk describes first-principles reasoning as boiling a problem down to fundamental truths and reasoning up from there. **Why it matters:** Product design should not copy “Anki + question bank”; it should decompose the MCAT into ability, concept graph, reasoning operations, feedback, and retention. **Source:** [TED 2013 transcript summary](https://muskwiki.com/wiki/ted2013-tesla-spacex-solarcity/)
• **Questions need content × reasoning tags:** AAMC says science sections test content knowledge plus scientific reasoning, research design, and data/statistical reasoning. **Why it matters:** Every item should update both content nodes and reasoning-skill nodes. **Source:** [AAMC, Scientific Inquiry and Reasoning Skills](https://students-residents.aamc.org/whats-mcat-exam/scientific-inquiry-reasoning-skills-overview)
• **Measurement precision varies by ability level:** IRT standard errors change depending on θ and item information. **Why it matters:** Adaptive tests should choose the next question for maximum information near the current ability estimate. **Source:** [Efficient Standard Errors in Item Response Theory Models for Short Tests](https://pmc.ncbi.nlm.nih.gov/articles/PMC7221492/)
• **Graph-based knowledge tracing gives structure to prediction:** GKT models concepts as nodes and relationships as edges instead of treating history as a flat sequence. **Why it matters:** The learning system should combine prediction with an explicit concept graph users can inspect. **Source:** [Nakagawa et al., Graph-based Knowledge Tracing](https://rlgm.github.io/papers/70.pdf)

### Carl Hendrick / Evidence-Informed Education Lens

• **Retrieval is not initial comprehension:** Hendrick warns against using retrieval practice before students have encoded anything worth retrieving. **Why it matters:** For brand-new MCAT nodes, the app should start with explanation, modelling, or worked examples before quizzing. **Source:** [Carl Hendrick, “The Lethal Mutation of Retrieval Practice”](https://carlhendrick.substack.com/p/the-lethal-mutation-of-retrieval)
• **Cognitive science is not a magic spell:** EEF says retrieval, spacing, and interleaving have strong cognitive evidence but thinner classroom implementation evidence. **Why it matters:** The app should instrument learning gains and transfer instead of assuming the strategy works because the lab result is good. **Source:** [EEF, Cognitive Science Approaches in the Classroom](https://educationendowmentfoundation.org.uk/education-evidence/evidence-reviews/cognitive-science-approaches-in-the-classroom)
• **Novices need guidance:** Minimal guidance performs poorly for novices; worked examples reduce cognitive load and support schema construction. **Why it matters:** The app should adapt from worked examples to independent timed passages as mastery rises. **Source:** [Kirschner, Sweller & Clark, “Why Minimal Guidance During Instruction Does Not Work”](https://doi.org/10.1207/s15326985ep4102_1)
• **Multiple-choice distractors can teach wrong knowledge:** Feedback reduces the negative effects of multiple-choice testing. **Why it matters:** Wrong MCAT answers should be logged as misconception evidence, and feedback should explicitly neutralize the lure. **Source:** [Butler & Roediger, “Feedback enhances the positive effects and reduces the negative effects of multiple-choice testing”](http://psychnet.wustl.edu/memory/wp-content/uploads/2018/04/Butler-Roediger-2008_MemCog.pdf)
• **Growth mindset is weak without strategy:** Growth-mindset effects are modest and context-dependent; Hendrick argues effort must be tied to knowing how to succeed. **Why it matters:** Avoid motivation slogans; show specific next actions: retrieval, feedback review, error diagnosis, spacing, and reattempts. **Source:** [Sisk et al., Growth Mind-Sets Meta-Analysis](https://doi.org/10.1177/0956797617739704)

### Jeffrey D. Karpicke / Retrieval Lens

• **One correct recall is not enough:** Repeated retrieval after successful recall produces large long-term gains, while repeated studying adds little. **Why it matters:** A concept should not retire after one correct answer. **Source:** [Karpicke & Roediger, “The Critical Importance of Retrieval for Learning”](https://learninglab.psych.purdue.edu/downloads/2008/2008_Karpicke_Roediger_Science.pdf)
• **Students misjudge their future performance:** Learners’ predictions often fail to match delayed performance after retrieval/restudy conditions. **Why it matters:** Readiness should trust performance data more than confidence. **Source:** [Karpicke & Roediger, 2008](https://www.science.org/doi/10.1126/science.1152408)
• **Retrieval can beat concept mapping:** Retrieval practice outperformed elaborative studying with concept mapping on science learning, inference questions, and a final concept-map test. **Why it matters:** A concept graph should not just be viewed; users should reconstruct it from memory. **Source:** [Karpicke & Blunt, “Retrieval Practice Produces More Learning than Elaborative Studying with Concept Mapping”](https://learninglab.psych.purdue.edu/downloads/2011/2011_Karpicke_Blunt_Science.pdf)
• **Concept maps can become retrieval practice:** Retrieval-based concept mapping works when learners create maps from memory instead of copying from visible notes. **Why it matters:** MCAT graph activities should hide source material and ask students to reconstruct pathways, prerequisites, and mechanisms. **Source:** [Blunt & Karpicke, “Learning With Retrieval-Based Concept Mapping”](https://learninglab.psych.purdue.edu/downloads/2014/2014_Blunt_Karpicke_JEDP.pdf)
• **Guided automated scoring can improve retrieval:** Students can over-credit wrong free responses, so automated scoring helps guide retrieval practice. **Why it matters:** The app should score explanations/free responses against criteria, not rely only on self-rated Good/Easy. **Source:** [Grimaldi & Karpicke, “Guided retrieval practice of educational materials using automated scoring”](https://learninglab.psych.purdue.edu/downloads/2014/2014_Grimaldi_Karpicke_JEDP.pdf)

### Malala / Equity and Learner Agency Lens

• **Digital learning does not equal access:** Malala Fund argues digital learning can support independent learning, but poverty, gender norms, disability, rural location, and household pressures restrict access. **Why it matters:** The app should not assume private devices, constant bandwidth, or unlimited study time. **Source:** [Malala Fund, Closing the Gender Digital Learning Divide](https://malala.org/news-and-voices/closing-the-gender-digital-learning-divide)
• **Intermittent data changes product design:** In Malala Fund reporting, smartphone access did not translate to consistent edtech use because data was intermittent and access was unequal. **Why it matters:** Support offline review, small sync payloads, printable/exportable schedules, and no heavy default media. **Source:** [Malala Fund, Girls’ Education and COVID-19 in Pakistan](https://malala.org/news-and-voices/girls-education-and-covid-19-in-pakistan)
• **Access interruptions are not low ability:** UNESCO estimates pandemic distance learning failed to reach at least 500 million students, including 72% of the poorest. **Why it matters:** Adaptive models should separate inconsistent usage from weak knowledge. **Source:** [UNESCO GEM Report 2023](https://gem-report-2023.unesco.org/)
• **Agency needs scaffolding:** OECD found students were more confident using digital tools than managing autonomous learning without reminders. **Why it matters:** A black-box queue is not enough; users need transparent plans, small next steps, and adjustable autonomy. **Source:** [OECD, PISA 2022 Results Volume I](https://www.oecd.org/en/publications/2023/12/pisa-2022-results-volume-i_76772a36/full-report/from-data-to-insights_f1c46d26.html)
• **First-gen medical students face financial constraints:** AAMC reports first-generation MD matriculants remain underrepresented and often come from lower-income backgrounds. **Why it matters:** Core score-improving features should not be premium-only. **Source:** [AAMC, First-Generation U.S. Medical School Matriculants](https://www.aamc.org/media/78371/download)

## DOK 1: Research & Facts

### The Learning Science Experts

#### Carl Hendrick

• Growth Mindset should not be about effort; it should be about exactly **how to succeed**.
• Rereading/highlighting not helpful; Retrieval, Interleaving, Spaced Repetition… all the hard unfamiliar study methods are best
• Link: [https://carlhendrick.substack.com/p/why-you-cannot-believe-your-way-into](https://carlhendrick.substack.com/p/why-you-cannot-believe-your-way-into)

#### John Dunlosky

• Specifically, practice testing and distributed practice are most helpful.
• Practice testing includes flashcards, **practice problems**, and **practice tests**.
• Distributed practice is about spacing out to have consistent practice, instead of cramming
• Source: Dunlosky, J., Rawson, K. A., Marsh, E. J., Nathan, M. J., & Willingham, D. T. (2013). Improving Students’ Learning With Effective Learning Techniques: Promising Directions From Cognitive and Educational Psychology. Psychological Science in the Public Interest, 14(1), 4-58. Full Publication via ERIC PDF.

### Practice Problems vs Flashcards

• Flashcard Group: Practiced active recall via isolated, discrete facts e.g., matching a process name to its precise definition or single variable function
• built a highly efficient but rigid memory architecture. But when a concept was framed inside an unexpected context, their brains struggled to unlock the information because it wasn’t attached to sufficient structural context
• Practice Question Group: Practiced active recall using complex, paragraph-based scenario stems requiring multi-step deductive logic
• developed flexible cognitive schemas. Because they had practiced navigating distracting information and scenario “noise,” they could easily transfer their knowledge to solve completely new multi-step problems
• Groups performed roughly the same on direct retention test (basic recall of the facts), though flashcard group finished faster
• On Transfer/Application Test, Practice Question group outperformed the Flashcards group
• Source: Jacqueline P. Leighton, et al. (2021). “The Differential Effects of Factual vs. Contextualized Retrieval Practice on Learning and Transfer.” While learning scientists universally praise “retrieval practice,” this study explicitly isolates what happens to the brain when you change the delivery mechanism of that retrieval—comparing atomic, isolated prompts (flashcards) with contextual, scenario-based applications (practice problems)

### Anki Flashcard Spaced-Repetition Algorithm

• schedules when the student should see that concept again to ensure it moves from short-term memory into permanent, long-term memory
• forces active recall: have to physically search its neural pathways to construct the answer from scratch
• lacking what concepts and their order to teach; but allows each concept to be ingrained well
• Core Science: The Forgetting Curve. He proved that whenever you learn a new fact, your memory of it drops significantly over the next 48 hours. However, if you review that fact right before it slips out of your brain, the memory gets locked in deeper, and the curve flattens out. Each time you successfully recall the information, it takes longer for you to forget it again.
• 4 modes for each flashcard: Again, Hard, Good, and Easy
• updated Free Spaced Repetition Scheduler FSRS: reduces penalty of choosing hard
• Source: Ebbinghaus, H. (1885). Memory: A Contribution to Experimental Psychology.
• Source: Open Spaced Repetition Organization. The ABC of FSRS. Available on the open-source GitHub Repository Wiki.

### Current Products

• Conversational AI Teaching & Testing: opposed to exhausting 59 question full test; when wrong gets generated sub-hints
• Advanced error logging: Content Gap, Data/Figure Misinterpretation, Pacing/Timing Panic, or AAMC Trap Logic
• AI Tutors: trained on verified medical data to prevent hallucination
• Screenshot analysis (database, text, metabolic pathways)

### Flashcard Theories: Memory -> Performance

#### Benefit

• Force active recall opposed to passive restudying
• Allow for spaced repetition motivated by Forgetting Curve
• Shuffling allows for interleaving, which forces the brain to constantly reassess
• By reducing to simple question/answer problems, optimizes cognitive load to take advantage without overloading working capacity
• Source: Roediger, H. L., & Butler, A. C. (2011). The critical role of retrieval practice in long-term retention. Trends in Cognitive Sciences, 15(1), 20-27. PDF Source via Washington University.
• Source: Demme Learning Research Insights (2026). Beyond the Flashcard: Why Conceptual Understanding Is Key to Fluency. Explores the transition from concrete visual structures to symbolic application Demme Blog Archive.

#### Downside

• Great for memorizing facts, students often find difficulty applying that knowledge to complex, multi-step problem solving like MCAT passages
• This is known as the Transfer of Learning bottleneck. Memorizing a definition gives you a Detail-Level Representation, whereas problem solving requires a Situational/Structural Model.
• The Trap: Flashcards create isolated nodes of information in your brain. Problem solving requires you to build a network of nodes and navigate between them under unique, novel constraints.
• Source: The Learning Scientists (2023). Learning With Flashcards: Detail vs. Conceptual Structures. Structural mapping study analysis on Learning Scientists Blog.

#### Bridging The Gap

• Shift from “Detail-Level” to “Conceptual-Level” Cards:
• Experiment showed that making own cards vs using generated cards had little effect on performance; though making own cards took more study time.
• Also showed positive relationship between using concept-level cards on performance compared to detail-level (but relationship only holds in students who were lacking structure building skills)
• Source: The Learning Scientists (2023). Learning With Flashcards: Detail vs. Conceptual Structures. Structural mapping study analysis on Learning Scientists Blog.
• CLT demonstrates that achieving automated, effortless recall through flashcards is essential for problem solving. By offloading the mental strain of raw fact retrieval, the cognitive workspace is freed up to focus entirely on the complex critical thinking needed to solve the problem itself
• Source: Sweller, J. (1988). Cognitive load during problem solving: Effects on learning. Cognitive Science, 12(2), 257-285.
• If you study all your cards in a neat row, your brain recognizes the pattern. Have to interleave your conceptual cards across slightly different topics, it forces your brain to constantly reassess.
• Source: Bjork, E. L., & Bjork, R. A. (2011). Making things hard on yourself, but in a good way: Creating desirable difficulties to enhance learning. Psychology and the Real World: Essays Illustrating Fundamental Contributions to Society, 56-64.

### MCAT Specifics

• 118-132 points per exam (total avg is 500)
• roughly 2.5 pts / standard deviation (in each test)
• Passage-Based Questions: These present a multi-paragraph technical text (such as a research study, an experimental protocol, or an essay) paired with an accompanying set of 4 to 7 multiple-choice questions. To solve these, users must synthesize data from tables or charts with their own outside scientific knowledge.
• Discrete Questions: These are completely independent, standalone multiple-choice questions that are interspersed between the passages. They do not rely on any reading text and are designed to quickly test pure conceptual recall, factual knowledge, or straightforward calculations.


| Section                                            | Total Time | Question Breakdown                                                | Core Subject Weights                                                                         |
| -------------------------------------------------- | ---------- | ----------------------------------------------------------------- | -------------------------------------------------------------------------------------------- |
| Chemical & Physical Foundations (Chem/Phys)        | 95 mins    | 59 Total Questions: 10 Passages (44 Qs) + 15 Standalone Discretes | General Chemistry: 30%, Biochemistry: 25%, Physics: 25%, Organic Chemistry: 15%, Biology: 5% |
| Critical Analysis & Reasoning (CARS)               | 90 mins    | 53 Total Questions: 9 Passages (53 Qs) + 0 Standalone Discretes   | Humanities: 50%, Social Sciences: 50% (Zero outside science knowledge)                       |
| Biological & Biochemical Foundations (Bio/Biochem) | 95 mins    | 59 Total Questions: 10 Passages (44 Qs) + 15 Standalone Discretes | Biology: 65%, Biochemistry: 25%, General Chemistry: 5%, Organic Chemistry: 5%                |
| Psychological & Social Foundations (Psych/Soc)     | 95 mins    | 59 Total Questions: 10 Passages (44 Qs) + 15 Standalone Discretes | Psychology: 65%, Sociology: 30%, Biology: 5%                                                 |


### MCAT Approaches (Reddit/User Reports)

#### Pro Anki

• You read exactly one chapter of a content book (like Kaplan), open the Anki browser, find the specific hierarchical tag for that exact chapter (e.g., Kaplan Ch1 Amino Acids), and unsuspend only those specific 40–50 cards. Anki is fundamentally a reinforcement tool, not a pure learning tool. You must learn the macro-concept from your book or video first, then instantly use the corresponding cards to lock down the forgetting curve before the information slips away.
• Only press the Again and Good buttons. Completely ignore Hard and Easy. The standard algorithm penalizes you heavily if you press “Hard” by trapping your cards in a loop (Ease Hell), forcing you to see them far too frequently. Trust the spacing intervals. If you got the card wrong or hesitated for more than 5 seconds, hit Again. If you successfully recalled it, hit Good and move on. This keeps your daily review counts streamlined and predictable.
• While studying UWorld / AAMC passage questions, if you miss one, diagnose the root issue and find/create flashcard for missed questions subdeck.
• The algorithm only functions if you do your cards every single day, without exception. Missing three days causes your reviews to snowball, creating a backlog that destroys motivation.
• Sources:
• r/Mcat High-Scorer Retrospectives. The road to 520+ with Anki. Long-term streak tracking and card volume parameters. Reddit Source.
• r/Mcat Community Strategy Guides. How to use Anki? Peer parameters and deck optimization. Reddit Source.
• r/Mcat Moderation Support. How to decide Anki settings? Handling new card caps and study limits. Reddit Source.
• r/Mcat Daily Accountability Threads. MilesDown Anki tips: Staying on top of volume without burning out. Reddit Source.
• r/Mcat Comprehensive FAQs. Genuinely confused on how people use Anki in the beginning stages of studying? Complete breakdown of chapter-by-chapter unsuspending workflows. Reddit Source.
• AnKing Med / Pre-Med Official Video Guide Manuals. Settings, intervals, and why the “Hard” button breaks your schedule. Workflow parameters tracked via AnKiHub Community Support Guides.

#### Anti Anki

• Many high scorers abandon pre-made decks after a week because grinding flashcards leaves you too exhausted. But having deep passage practice is essential for a good score. So instead of wasting months memorizing a massive 6,000-card deck, you should completely skip pre-made decks and make a tiny deck of only 300–500 cards of consistently missed concepts (or no flashcards even) and focus on passage practice problems.
• You do not learn science via such small knowledge fragments, but through context. A superior alternative to flashcards is active review sheets or matrix tables. For example, writing out a one-page comparison map of all four enzyme inhibitors or drawing out the pathways by hand forces your brain to see the big picture.
• The idea that you need a distinct phase for content review before you do practice questions is a fundamental mistake. It stems from fear since students stay in their flashcard bubble because getting questions wrong hurts their confidence. You should start spamming practice questions on day 1, completely open-book if necessary. Getting a question wrong, reading the detailed explanation block, and physically tracing why you fell for the trap teaches you both the core science fact and the AAMC application logic simultaneously.
• Outside of Amino Acids and Psych/Soc terms, a massive portion of the science answers are explicitly hidden right inside the passage text, graphs, and figures if you know how to look for them. The MCAT rewards reading comprehension, logical endurance, data extraction, and the ability to handle panic under intense time constraints: so practice that instead of flashcards.
• Sources:
• Reddit Pre-Med Forum (2026). How often did you guys do Anki? - Counter-Perspectives. Reddit Thread Source.
• r/Mcat Community Archives (2026). Did this with NO Anki (515+ Score Writeups). Reddit Thread Source.
• r/medicalschool Board Prep Overviews. AnKing not working for me? Finding alternate pathways. Reddit Thread Source.
• r/Mcat User Case Studies. Should I stop doing Anki completely? Working full time. Reddit Thread Source.
• r/MedSchoolCanada Open Forums (2025). What happens if you absolutely hate Anki cards? Reddit Thread Source.

### Scoring Theories: Performance -> Readiness

#### Classical Test Theory

1 question 1 point, then scaled for normalization. Can be unfair.

#### Item Response Theory

Goal is to find/calculate the hidden true intelligence value of θ (-3.0 to 3.0)
• accurate to real scoring & best for admissions ranking: real MCAT scoring (+scaling ability θ)
• 3-Parameter logistic model: Difficulty (b), Discrimination (a), Guessing (c)
• b: how much ability (θ) needs to have a 50% chance of getting correct. also -3.0 to 3.0
• a: how good to distinguish good from average. represents steepness/slope of probability curve. always positive
• c: probability of guessing correctly; used to factor out noise from guessing correctly
• Instead of a static quiz, a possible code would do this: Serve the user a question of medium difficulty (b=0.0). If they get it right, code executes a maximum likelihood estimation algorithm to bump their estimated ability θ up, and serves a harder question next. If they get it wrong, code serves an easier question.

• 3PL formula: $P(\text{correct}|\theta) = c + \frac{1-c}{1 + e^{-a(\theta-b)}}$
• as θ<<b, P approaches c; as θ>>b, P approaches 1
• Final Score: use Maximum Likelihood to find optimal θ, then scale θ to the MCAT score range.
• Source: National Academies of Sciences, Engineering, and Medicine. Overview of Psychological Testing. National Academies Press.
• Source: Psychometric Comparison Guides, Cogn-IQ Psychometrics.

#### G-Theory

Error comes from specific sources (facets) such as difficulty, subject mix, fatigue
• an advanced ANOVA (Analysis of Variance). Add up the variances from different measurable factors.
• can provide Generalizability Coefficient (G) on how reliable app’s prediction is

#### Knowledge Space Theory

• maps out the exact combination of concepts a student actually understands, allows for structured learning path through a knowledge lattice
• Knowledge State and Knowledge Space (all the feasible states)
• surmise relation $A \preceq B$ if mastering B requires mastering A first
• tracks inner fringe (mastered concepts) and outer fringe (ready to learn) of current knowledge state. This makes learning an upwards walk between different nodes of the graph
• superior for teaching students, because can recommend next lesson based on all of current knowledge
• ALEKS (McGraw Hill) uses pure KST
• Source: International Educational Data Mining Society (EDM). An Evaluation of a Placement Assessment for an Adaptive Learning System. EDM Proceedings.
• Source: Doignon, J.-P., & Falmagne, J.-C. (1999). Knowledge Spaces. Springer-Verlag. (See summary profile on Wikipedia: ALEKS).

#### Bayesian Network Models

More fluid extension of KST.
• Has different degrees of progress (continuous probability score) than binary KST
• models the student’s mind as a Directed Acyclic Graph (DAG), can crucially never loop back
• System treats a student’s mastery of topics as a web of conditional probabilities (with a Conditional Probability Table CPT per node) instead of strict prerequisites —> more flexibility for occasional silly mistakes
• Leverages Bayes Theorem to update the conditional probabilities
• forward propagation e.g. improved performance on basics bump up probabilities for child nodes/downstream topics
• backward propagation e.g. bad performance on a new topic decrease the probability of mastery of previous related upstream topics
• sideways balancing: balances the blame between two necessary subjects accurately if performs badly on a topic that needed both as prerequisites (may just be due to one of the two topics)
• Personalized, efficient with time/energy, flexible to noise
• Source: Mislevy, R. J. (CRESST / University of Maryland). Modeling Conditional Probabilities in Complex Educational Assessments. CRESST Technical Report Repository.


| Feature           | Knowledge Space Theory (KST)                                                          | Bayesian Network Models                                                                            |
| ----------------- | ------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------- |
| Core Output       | A discrete list of mastered topics.                                                   | A percentage probability of mastery for each topic.                                                |
| Mathematical Base | Set Theory and Combinatorics.                                                         | Conditional Probability (Bayes’ Theorem).                                                          |
| Best Used For     | Curating a strict, step-by-step learning path (e.g., “You must learn Topic X today”). | Diagnosing complex, overlapping student profiles where skills are fluid.                           |
| Coding Complexity | Lower. Relies on simple graph traversals and set comparisons.                         | Higher. Requires matrix multiplication or a probabilistic graphical library to update the network. |


#### Bayesian Knowledge Tracing

• Markov chain network to track mastery or unmastery (2 states) of each skill
• Uses 4 states: The Prior probability that the student knew the skill before starting; the Transition probability that the student actually learned the skill by reading the hint or explanation on this question, the Guess probability that they got it right by pure luck, the Slip probability that they actually master the topic but made a careless error.
• famous from Carnegie Learning and early Khan Academy
• Source: Pelánek, J. (2017). Bayesian Knowledge Tracing, Logistic Models, and Beyond: An Overview of Learner Modeling Techniques. User Modeling and User-Adapted Interaction, 27(3), 313-350.
• Source: Nakagawa, H., Iwasawa, Y., & Matsuo, Y. (2019). Graph-based Knowledge Tracing: Modeling student proficiency using graph neural networks. Proceedings of the IEEE/WIC/ACM International Conference on Web Intelligence (WI), 156-163.

#### Graph Neural Networks for Knowledge Tracing (GKT)

• feed raw student data into a graph neural net to update feature vectors of the nodes (concepts and their embeddings)
• Why use a GNN opposed to just a NN or other structure? Most fail to take into consideration the localness of the structure since it’s hard to deal with sparseness of having too many zeros. Also most architectures are trained for grid-like (Euclidean) data [images are grids, text/audio are chronological next token]
• GNNs have message passing: mimics human learning rippling through network
• trained on data instead of manual coding of prerequisites
• Source: Piech, C., Bassen, J., Huang, J., Ganguli, S., Sahami, M., Guibas, L. J., & Sohl-Dickstein, J. (2015). Deep Knowledge Tracing. Advances in Neural Information Processing Systems (NeurIPS), 28.

#### Educational Knowledge Graphs (KG-Aware RAG)

• Knowledge Graph forces an AI tutor to follow structural logic.
• Backend: uses a graph database (like Neo4j) mapping out precise relationships e.g. Glycolysis → PREREQUISITE_FOR → Krebs Cycle
• Retrieval Implementation: When a student chats with the AI tutor, the system doesn’t just do a semantic search; it physically traverses the graph nodes to pull highly structured, contextually precise data, preventing the AI from hallucinating or giving unstructured lessons.
• Source: ScholarSpace (University of Hawaii). Using Knowledge Graphs to Test for the Presence of Hallucinations in Closed RAG-based and CustomGPT-based LLM. Hawaii International Conference on System Sciences (HICSS).

#### Transformer-Bayesian Hybrid Network*

• In this setup, a GNN or a deep sequence encoder processes raw student click history to capture timing, stress, and fast-paced patterns, while a differentiable Bayesian network sits right on top of it to enforce hard causal rules.
• Source: MDPI Applied Sciences (2025). Interpretable Knowledge Tracing via Transformer-Bayesian Hybrid Networks: Learning Temporal Dependencies and Causal Structures in Educational Data.

*Do more research later.*

### Conflicts

• careful human curation vs blackbox models for learning network/prerequisites/etc
• how rigid should learning be? strict prereqs vs flexible skipping around
• learning new concepts vs retrieval of old concepts — how to maintain balance?
• self-regulated learning of being able to learn systematically on our owns vs letting AI personalize entire learning trajectory
• perhaps for college students, should give more power to student
• Source: Azevedo, R., & Hadwin, A. F. (2005). Scaffolding Self-Regulated Learning and Metacognition in Implications for the Design of Advanced Learning Technologies. Educational Psychologist, 40(4), 193-197.

### Consensus

• learning must be broken down to Knowledge Components
• must master sufficiently (though to what extent is debatable) to continue to next topics
• real-time adaption and intervention based on performance: assessments, content, sequencing
• know the root causes such as guessing, slipping up, real mistakes in algorithm
• Source: Koedinger, K. R., Corbett, A. T., & Perfetti, C. (2012). The Knowledge-Component Framework: Toward an Analytical Science of Learning and Instruction. ACM Transactions on Computer-Human Interaction, 19(2), 1-29.