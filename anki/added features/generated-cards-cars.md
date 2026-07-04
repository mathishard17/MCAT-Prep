# MCAT Generated Cards — CARS (Critical Analysis and Reasoning Skills)

Self-contained CARS multiple-choice cards built from short **humanities**
passages (philosophy, literature, ethics, aesthetics/art criticism, history of
ideas). Every passage is drawn from a **public-domain** work and is cited inside
the `Question` line (author, work, approximate year); each item is answerable
**from the passage alone**, with no outside knowledge required. Block format
matches `added features/mcat_demo_cards.md` so the same importer
(`rslib/src/scheduler/concept_demo.rs`) can parse it.

CARS skill KCs (a prerequisite chain):

- `CARS::Foundations_of_Comprehension` — Prereqs: none
- `CARS::Reasoning_Within_the_Text` — Prereq: `Prereq::CARS::Foundations_of_Comprehension`
- `CARS::Reasoning_Beyond_the_Text` — Prereq: `Prereq::CARS::Reasoning_Within_the_Text`

Conventions:

- **Total: 21 cards — 7 per skill.**
- IDs: `MCAT-CARS-<TOPIC>-<NNN>`, unique per card (`<TOPIC>` = a short source tag).
- Section: `MCAT::CARS` for every card.
- The `Reasoning::` tag uses CARS-appropriate values — `Comprehension`,
  `Inference`, `Tone`, `Structure`, `Application` — and is stored as a free
  string by `CardConceptMetadata::from_tags` (`rslib/src/scheduler/concept.rs`).
- `IRT::Guessing::0.25` for these four-choice items; `IRT::Discrimination::` in
  `0.7`–`1.5`.
- Skill mapping: Foundations = main idea / comprehension / vocabulary-in-context;
  Within = inference, tone, author's attitude, argument structure; Beyond =
  application to new situations, extension, evaluation of new evidence.

---

## CARS::Foundations_of_Comprehension

### MCAT-CARS-LIBERTY-001

- **KC:** `CARS::Foundations_of_Comprehension`
- **Prereqs:** none
- **Difficulty:** 2
- **Question:** Passage (Mill, *On Liberty*, ~1859): The object of this Essay is to assert one very simple principle, as entitled to govern absolutely the dealings of society with the individual in the way of compulsion and control. That principle is, that the sole end for which mankind are warranted, individually or collectively, in interfering with the liberty of action of any of their number, is self-protection. That the only purpose for which power can be rightfully exercised over any member of a civilised community, against his will, is to prevent harm to others. His own good, either physical or moral, is not a sufficient warrant. He cannot rightfully be compelled to do or forbear because it will be better for him to do so, because it will make him happier, because, in the opinions of others, to do so would be wise, or even right. — Question: Which of the following best expresses the central principle the author is asserting?
- **A:** Society should compel individuals to act for their own physical and moral good.
- **B:** Society may rightfully coerce an individual only to prevent harm to other people.
- **C:** Individual liberty must always yield to the collective opinion of the wise.
- **D:** Power over individuals is justified whenever exercising it increases their happiness.
- **Correct:** B
- **Explanation:** The passage names self-protection, i.e., preventing harm to others, as the "sole end" warranting interference and explicitly denies that a person's own good, happiness, or others' opinions suffice; A, C, and D restate exactly the justifications Mill rejects.
- **Tags:** `KC::CARS::Foundations_of_Comprehension` `MCAT::CARS` `Difficulty::2` `IRT::Discrimination::0.85` `IRT::Guessing::0.25` `Reasoning::Comprehension`

### MCAT-CARS-STUDIES-001

- **KC:** `CARS::Foundations_of_Comprehension`
- **Prereqs:** none
- **Difficulty:** 3
- **Question:** Passage (Bacon, *Of Studies*, ~1625): Studies serve for delight, for ornament, and for ability. Their chief use for delight is in privateness and retiring; for ornament, is in discourse; and for ability, is in the judgment and disposition of business. For expert men can execute, and perhaps judge of particulars, one by one; but the general counsels, and the plots and marshalling of affairs, come best from those that are learned. To spend too much time in studies is sloth; to use them too much for ornament is affectation; to make judgment wholly by their rules is the humour of a scholar. They perfect nature, and are perfected by experience: for natural abilities are like natural plants, that need pruning by study. — Question: As it is used in the passage, the word "ability" most nearly refers to which of the following?
- **A:** Skill in the judgment and conduct of practical affairs.
- **B:** Physical strength suited to manual labor.
- **C:** Natural talent that requires no cultivation.
- **D:** The capacity to entertain oneself in private.
- **Correct:** A
- **Explanation:** Bacon glosses "ability" as residing "in the judgment and disposition of business," i.e., competence in handling affairs; D describes the "delight" use, C contradicts his claim that natural abilities "need pruning by study," and B is never suggested.
- **Tags:** `KC::CARS::Foundations_of_Comprehension` `MCAT::CARS` `Difficulty::3` `IRT::Discrimination::0.95` `IRT::Guessing::0.25` `Reasoning::Comprehension`

### MCAT-CARS-VIRTUE-001

- **KC:** `CARS::Foundations_of_Comprehension`
- **Prereqs:** none
- **Difficulty:** 3
- **Question:** Passage (Aristotle, *Nicomachean Ethics*, ~350 BCE): Virtue, then, being of two kinds, intellectual and moral, intellectual virtue in the main owes both its birth and its growth to teaching, while moral virtue comes about as a result of habit. From this it is also plain that none of the moral virtues arises in us by nature; for nothing that exists by nature can form a habit contrary to its nature. The stone which by nature moves downwards cannot be habituated to move upwards, not even if one tries to train it by throwing it up ten thousand times. Neither by nature, then, nor contrary to nature do the virtues arise in us; rather we are adapted by nature to receive them, and are made perfect by habit. The virtues we get by first exercising them, as also happens in the arts. — Question: The passage is primarily concerned with establishing which of the following?
- **A:** Intellectual virtue and moral virtue are acquired in exactly the same way.
- **B:** Human beings are born already possessing the moral virtues.
- **C:** Moral virtue is acquired through habitual practice rather than by nature.
- **D:** The virtues, like a falling stone, are fixed by unchangeable natural law.
- **Correct:** C
- **Explanation:** Aristotle contrasts intellectual virtue (from teaching) with moral virtue (from habit) and argues the latter arises neither by nature nor against nature but through exercise; A ignores that contrast, and B and D directly contradict "none of the moral virtues arises in us by nature."
- **Tags:** `KC::CARS::Foundations_of_Comprehension` `MCAT::CARS` `Difficulty::3` `IRT::Discrimination::0.9` `IRT::Guessing::0.25` `Reasoning::Comprehension`

### MCAT-CARS-WALDEN-001

- **KC:** `CARS::Foundations_of_Comprehension`
- **Prereqs:** none
- **Difficulty:** 2
- **Question:** Passage (Thoreau, *Walden*, ~1854): I went to the woods because I wished to live deliberately, to front only the essential facts of life, and see if I could not learn what it had to teach, and not, when I came to die, discover that I had not lived. I did not wish to live what was not life, living is so dear; nor did I wish to practise resignation, unless it was quite necessary. I wanted to live deep and suck out all the marrow of life, to live so sturdily and Spartan-like as to put to rout all that was not life, to cut a broad swath and shave close, to drive life into a corner, and reduce it to its lowest terms. — Question: The author's primary purpose in going to the woods was to do which of the following?
- **A:** Practise resignation and withdraw permanently from all human society.
- **B:** Demonstrate that solitude is superior to community for every person.
- **C:** Accumulate the material comforts that make living so "dear."
- **D:** Confront life's essentials so as to truly live rather than merely exist.
- **Correct:** D
- **Explanation:** Thoreau says he wished "to live deliberately, to front only the essential facts of life" so as not to discover at death "that I had not lived"; he explicitly did not wish "to practise resignation," which defeats A, and B and C are never claimed.
- **Tags:** `KC::CARS::Foundations_of_Comprehension` `MCAT::CARS` `Difficulty::2` `IRT::Discrimination::0.8` `IRT::Guessing::0.25` `Reasoning::Comprehension`

### MCAT-CARS-RIGHTS-001

- **KC:** `CARS::Foundations_of_Comprehension`
- **Prereqs:** none
- **Difficulty:** 3
- **Question:** Passage (Wollstonecraft, *A Vindication of the Rights of Woman*, ~1792): My own sex, I hope, will excuse me, if I treat them like rational creatures, instead of flattering their fascinating graces, and viewing them as if they were in a state of perpetual childhood, unable to stand alone. I earnestly wish to point out in what true dignity and human happiness consists. I wish to persuade women to endeavour to acquire strength, both of mind and body, and to convince them that the soft phrases, susceptibility of heart, delicacy of sentiment, and refinement of taste, are almost synonymous with epithets of weakness. Women are, in fact, so much degraded by mistaken notions of female excellence, that I do not mean to add a paragraph of the sentimental cant that has hitherto been offered to my sex. — Question: Which of the following best states the central claim of the passage?
- **A:** Women should be praised chiefly for their delicacy, sentiment, and refinement of taste.
- **B:** Women should be regarded and educated as rational beings who cultivate strength of mind and body.
- **C:** Women should be protected in a state of dependence resembling perpetual childhood.
- **D:** Women should be encouraged to embrace the sentimental language traditionally offered to them.
- **Correct:** B
- **Explanation:** Wollstonecraft asks to treat women "like rational creatures" and to help them "acquire strength, both of mind and body," while calling delicacy and sentiment "epithets of weakness"; A, C, and D each restate the mistaken notions she explicitly repudiates.
- **Tags:** `KC::CARS::Foundations_of_Comprehension` `MCAT::CARS` `Difficulty::3` `IRT::Discrimination::0.95` `IRT::Guessing::0.25` `Reasoning::Comprehension`

### MCAT-CARS-STOIC-001

- **KC:** `CARS::Foundations_of_Comprehension`
- **Prereqs:** none
- **Difficulty:** 2
- **Question:** Passage (Epictetus, *Enchiridion*, ~125 CE): Some things are in our power, and others are not. In our power are opinion, movement towards a thing, desire, aversion; in a word, whatever are our own acts. Not in our power are the body, property, reputation, offices, and, in a word, whatever are not our own acts. The things in our power are by nature free, not subject to restraint or hindrance; but the things not in our power are weak, slavish, subject to restraint, and in the power of others. Remember, then, that if you think the things which are by nature slavish to be free, and the things which are in the power of others to be your own, you will be hindered, you will lament, you will be disturbed, and you will blame both gods and men. — Question: Which of the following best states the main point of the passage?
- **A:** Peace of mind comes from recognizing what is and is not within our own power.
- **B:** The gods and other people are ultimately responsible for our disturbances.
- **C:** Bodily desires should always be preferred to concerns about reputation.
- **D:** Freedom is a legal status granted to some people and denied to others.
- **Correct:** A
- **Explanation:** Epictetus sorts things into those "in our power" and those "not in our power" and warns that confusing them leaves one "hindered" and "disturbed"; B inverts his point (blaming gods and men is the error he diagnoses), while C and D seize on incidental words rather than the governing claim.
- **Tags:** `KC::CARS::Foundations_of_Comprehension` `MCAT::CARS` `Difficulty::2` `IRT::Discrimination::0.8` `IRT::Guessing::0.25` `Reasoning::Comprehension`

### MCAT-CARS-SELFREL-001

- **KC:** `CARS::Foundations_of_Comprehension`
- **Prereqs:** none
- **Difficulty:** 3
- **Question:** Passage (Emerson, *Self-Reliance*, ~1841): There is a time in every man's education when he arrives at the conviction that envy is ignorance; that imitation is suicide; that he must take himself for better, for worse, as his portion. A foolish consistency is the hobgoblin of little minds, adored by little statesmen and philosophers and divines. With consistency a great soul has simply nothing to do. He may as well concern himself with his shadow on the wall. Speak what you think now in hard words, and to-morrow speak what to-morrow thinks in hard words again, though it contradict every thing you said to-day. To be great is to be misunderstood. — Question: In context, the phrase "a foolish consistency" most nearly refers to which of the following?
- **A:** The ordinary logical agreement required for any coherent thought.
- **B:** Loyalty to one's closest friends and family members.
- **C:** A rigid insistence on never contradicting one's own earlier statements.
- **D:** The physical resemblance between a person and his shadow.
- **Correct:** C
- **Explanation:** Emerson illustrates "foolish consistency" by urging one to "speak what you think now" even if "it contradict every thing you said to-day," so he means slavish fidelity to past statements; A mistakes it for all consistency, and B and D read unrelated words literally.
- **Tags:** `KC::CARS::Foundations_of_Comprehension` `MCAT::CARS` `Difficulty::3` `IRT::Discrimination::1.0` `IRT::Guessing::0.25` `Reasoning::Comprehension`

## CARS::Reasoning_Within_the_Text

### MCAT-CARS-DUTY-001

- **KC:** `CARS::Reasoning_Within_the_Text`
- **Prereqs:** `Prereq::CARS::Foundations_of_Comprehension`
- **Difficulty:** 4
- **Question:** Passage (Kant, *Groundwork of the Metaphysics of Morals*, ~1785): It is impossible to conceive anything at all in the world, or even out of it, which can be taken as good without qualification, except a good will. Intelligence, wit, judgment, and the other talents of the mind, however they may be named, or courage, resolution, and perseverance as qualities of temperament, are undoubtedly good and desirable in many respects; but these gifts of nature may also become extremely bad and mischievous if the will which is to make use of them is not good. It is the same with the gifts of fortune. Power, riches, honour, even health, and that complete well-being and contentment with one's condition which is called happiness, inspire pride, and often presumption, if there is not a good will to correct their influence. — Question: It can be inferred that the author regards intelligence and courage as which of the following?
- **A:** Good without qualification, standing on exactly the same level as a good will.
- **B:** Inherently mischievous traits that a wise person should work to suppress.
- **C:** Irrelevant to morality, because only the outcomes of actions determine what is good.
- **D:** Conditionally valuable, since their worth depends on the goodness of the will that employs them.
- **Correct:** D
- **Explanation:** Kant reserves "good without qualification" for a good will and says talents "may also become extremely bad" without one, implying their value is conditional; A contradicts "without qualification," B overstates ("may become," not inherently), and C imports an outcome-based ethics the passage never asserts.
- **Tags:** `KC::CARS::Reasoning_Within_the_Text` `Prereq::CARS::Foundations_of_Comprehension` `MCAT::CARS` `Difficulty::4` `IRT::Discrimination::1.15` `IRT::Guessing::0.25` `Reasoning::Inference`

### MCAT-CARS-MORALS-001

- **KC:** `CARS::Reasoning_Within_the_Text`
- **Prereqs:** `Prereq::CARS::Foundations_of_Comprehension`
- **Difficulty:** 5
- **Question:** Passage (Nietzsche, *Beyond Good and Evil*, ~1886): It has gradually become clear to me what every great philosophy up till now has consisted of—namely, the confession of its originator, and a species of involuntary and unconscious autobiography; and moreover that the moral, or immoral, purpose in every philosophy has constituted the true vital germ out of which the entire plant has grown. Indeed, to understand how the abstrusest metaphysical assertions of a philosopher have been arrived at, it is always well to ask oneself first: at what morality does all this aim? I do not believe, therefore, that an impulse to knowledge is the father of philosophy; but that another impulse has, here as elsewhere, employed knowledge merely as an instrument. — Question: The author's attitude toward philosophers' claims to pure, disinterested knowledge is best described as which of the following?
- **A:** Skeptical, treating such claims as disguised expressions of personal and moral motives.
- **B:** Reverent, regarding metaphysics as reason's highest and purest achievement.
- **C:** Indifferent, finding no meaningful difference among competing philosophies.
- **D:** Alarmed, warning that philosophy is about to destroy conventional morality.
- **Correct:** A
- **Explanation:** Nietzsche calls each great philosophy a "confession of its originator" and denies that "an impulse to knowledge is the father of philosophy," implying hidden motives — a probing, skeptical stance; B and C contradict that critical scrutiny, and D casts him as morally anxious rather than diagnostic.
- **Tags:** `KC::CARS::Reasoning_Within_the_Text` `Prereq::CARS::Foundations_of_Comprehension` `MCAT::CARS` `Difficulty::5` `IRT::Discrimination::1.25` `IRT::Guessing::0.25` `Reasoning::Tone`

### MCAT-CARS-CUSTOM-001

- **KC:** `CARS::Reasoning_Within_the_Text`
- **Prereqs:** `Prereq::CARS::Foundations_of_Comprehension`
- **Difficulty:** 4
- **Question:** Passage (Hume, *An Enquiry Concerning Human Understanding*, ~1748): Custom, then, is the great guide of human life. It is that principle alone which renders our experience useful to us, and makes us expect, for the future, a similar train of events with those which have appeared in the past. Without the influence of custom, we should be entirely ignorant of every matter of fact beyond what is immediately present to the memory and senses. We should never know how to adjust means to ends, or to employ our natural powers in the production of any effect. There would be an end at once of all action, as well as of the chief part of speculation. All inferences from experience, therefore, are effects of custom, not of reasoning. — Question: The passage most strongly implies that our expectations about the future rest primarily on which of the following?
- **A:** A chain of demonstrative reasoning from self-evident first principles.
- **B:** Habit formed by past experience rather than on logical reasoning.
- **C:** Direct sensory access to future events exactly as they will occur.
- **D:** An innate faculty present in the mind prior to any experience.
- **Correct:** B
- **Explanation:** Hume calls custom "the great guide of human life" and concludes that "all inferences from experience are effects of custom, not of reasoning," so expectation rests on habit; A and D name the rationalist and innatist views he rejects, and C contradicts our being "ignorant of every matter of fact beyond" memory and senses.
- **Tags:** `KC::CARS::Reasoning_Within_the_Text` `Prereq::CARS::Foundations_of_Comprehension` `MCAT::CARS` `Difficulty::4` `IRT::Discrimination::1.15` `IRT::Guessing::0.25` `Reasoning::Inference`

### MCAT-CARS-POETRY-001

- **KC:** `CARS::Reasoning_Within_the_Text`
- **Prereqs:** `Prereq::CARS::Foundations_of_Comprehension`
- **Difficulty:** 3
- **Question:** Passage (Shelley, *A Defence of Poetry*, ~1840): Poetry is indeed something divine. It is at once the centre and circumference of knowledge; it is that which comprehends all science, and that to which all science must be referred. Poetry is the record of the best and happiest moments of the happiest and best minds. A poet participates in the eternal, the infinite, and the one; as far as relates to his conceptions, time and place and number are not. Poets are the hierophants of an unapprehended inspiration; the mirrors of the gigantic shadows which futurity casts upon the present; the words which express what they understand not. Poets are the unacknowledged legislators of the world. — Question: The author's tone toward poetry in this passage is best characterized as which of the following?
- **A:** Measured and analytical, weighing poetry's strengths against its clear limits.
- **B:** Nostalgic and mournful, lamenting the lost prestige that poets once held.
- **C:** Exalted and celebratory, treating poetry as a supreme, almost sacred power.
- **D:** Skeptical and ironic, quietly questioning poetry's claims to importance.
- **Correct:** C
- **Explanation:** Shelley heaps superlatives on poetry — "something divine," "centre and circumference of knowledge," "unacknowledged legislators of the world" — an unreservedly exalted tone; A falsely implies balance, B misreads him as elegiac, and D is the opposite of his praise.
- **Tags:** `KC::CARS::Reasoning_Within_the_Text` `Prereq::CARS::Foundations_of_Comprehension` `MCAT::CARS` `Difficulty::3` `IRT::Discrimination::0.95` `IRT::Guessing::0.25` `Reasoning::Tone`

### MCAT-CARS-ARTCRIT-001

- **KC:** `CARS::Reasoning_Within_the_Text`
- **Prereqs:** `Prereq::CARS::Foundations_of_Comprehension`
- **Difficulty:** 3
- **Question:** Passage (Wilde, *The Picture of Dorian Gray*, Preface, ~1891): The artist is the creator of beautiful things. To reveal art and conceal the artist is art's aim. There is no such thing as a moral or an immoral book. Books are well written, or badly written. That is all. The nineteenth century dislike of realism is the rage of Caliban seeing his own face in a glass. No artist has ethical sympathies. An ethical sympathy in an artist is an unpardonable mannerism of style. No artist is ever morbid. The artist can express everything. Vice and virtue are to the artist materials for an art. All art is quite useless. — Question: The author's attitude toward judging art by moral standards is best described as which of the following?
- **A:** Approving, insisting that great art must teach virtuous conduct to its audience.
- **B:** Cautious, conceding that certain subjects are simply too immoral for any art.
- **C:** Conflicted, torn between the demands of aesthetic beauty and ethical duty.
- **D:** Dismissive, holding that moral categories are irrelevant to a work's artistic value.
- **Correct:** D
- **Explanation:** Wilde declares there is "no such thing as a moral or an immoral book," only "well written, or badly written," and that "no artist has ethical sympathies," dismissing moral judgment of art; A, B, and C each attribute a moral concern to a writer who explicitly denies it.
- **Tags:** `KC::CARS::Reasoning_Within_the_Text` `Prereq::CARS::Foundations_of_Comprehension` `MCAT::CARS` `Difficulty::3` `IRT::Discrimination::0.95` `IRT::Guessing::0.25` `Reasoning::Tone`

### MCAT-CARS-CULTURE-001

- **KC:** `CARS::Reasoning_Within_the_Text`
- **Prereqs:** `Prereq::CARS::Foundations_of_Comprehension`
- **Difficulty:** 4
- **Question:** Passage (Arnold, *Culture and Anarchy*, ~1869): The whole scope of the essay is to recommend culture as the great help out of our present difficulties; culture being a pursuit of our total perfection by means of getting to know, on all the matters which most concern us, the best which has been thought and said in the world. He who works for sweetness and light united, works to make reason and the will of God prevail. The men of culture are the true apostles of equality. They are those who have had a passion for diffusing, for making prevail, for carrying from one end of society to the other, the best knowledge, the best ideas of their time. Culture looks beyond machinery; culture hates hatred; culture has one great passion, the passion for sweetness and light. — Question: The author develops his conception of culture primarily by doing which of the following?
- **A:** Defining culture and then specifying its aims and the values it champions.
- **B:** Narrating the historical origins of the word "culture" over the centuries.
- **C:** Refuting a series of named opponents one argument at a time.
- **D:** Presenting statistical evidence about levels of educational attainment.
- **Correct:** A
- **Explanation:** Arnold first defines culture ("a pursuit of our total perfection... the best which has been thought and said") and then elaborates its aims ("sweetness and light," "diffusing... the best ideas"); he offers no etymology, no point-by-point refutation, and no statistics, so B, C, and D misdescribe his method.
- **Tags:** `KC::CARS::Reasoning_Within_the_Text` `Prereq::CARS::Foundations_of_Comprehension` `MCAT::CARS` `Difficulty::4` `IRT::Discrimination::1.2` `IRT::Guessing::0.25` `Reasoning::Structure`

### MCAT-CARS-FICTION-001

- **KC:** `CARS::Reasoning_Within_the_Text`
- **Prereqs:** `Prereq::CARS::Foundations_of_Comprehension`
- **Difficulty:** 4
- **Question:** Passage (Woolf, *Modern Fiction*, ~1925): Look within and life, it seems, is very far from being like this. Examine for a moment an ordinary mind on an ordinary day. The mind receives a myriad impressions—trivial, fantastic, evanescent, or engraved with the sharpness of steel. From all sides they come, an incessant shower of innumerable atoms; and as they fall, as they shape themselves into the life of Monday or Tuesday, the accent falls differently from of old. Life is not a series of gig-lamps symmetrically arranged; life is a luminous halo, a semi-transparent envelope surrounding us from the beginning of consciousness to the end. Is it not the task of the novelist to convey this varying, this unknown and uncircumscribed spirit? — Question: The passage most strongly implies that, in the author's view, a novel succeeds when it does which of the following?
- **A:** Arranges its events in a symmetrical, tightly plotted sequence.
- **B:** Captures the shifting flow of inner impressions that make up consciousness.
- **C:** Records only the sharp, steel-like impressions and discards the rest.
- **D:** Describes in full the external circumstances of a character's daily routine.
- **Correct:** B
- **Explanation:** Woolf contrasts life as a "luminous halo" of "myriad impressions" with life as "gig-lamps symmetrically arranged" and asks whether the novelist's task is to "convey this varying... spirit" of consciousness; A is the approach she rejects, C ignores the "myriad," and D reduces fiction to externals.
- **Tags:** `KC::CARS::Reasoning_Within_the_Text` `Prereq::CARS::Foundations_of_Comprehension` `MCAT::CARS` `Difficulty::4` `IRT::Discrimination::1.1` `IRT::Guessing::0.25` `Reasoning::Inference`

## CARS::Reasoning_Beyond_the_Text

### MCAT-CARS-POETICS-001

- **KC:** `CARS::Reasoning_Beyond_the_Text`
- **Prereqs:** `Prereq::CARS::Reasoning_Within_the_Text`
- **Difficulty:** 4
- **Question:** Passage (Aristotle, *Poetics*, ~335 BCE): Tragedy, then, is an imitation of an action that is serious, complete, and of a certain magnitude; in language embellished with each kind of artistic ornament, the several kinds being found in separate parts of the play; in the form of action, not of narrative; through pity and fear effecting the proper purgation of these emotions. A whole is that which has a beginning, a middle, and an end. The plot, then, is the first principle, and, as it were, the soul of a tragedy; character holds the second place. Most important of all is the structure of the incidents. For tragedy is an imitation, not of men, but of an action and of life. — Question: Based on the passage, which new work would best satisfy the author's definition of tragedy?
- **A:** A long narrative poem, read aloud, cataloguing a hero's many loosely connected adventures.
- **B:** A light comedy that relies on witty dialogue and a chain of amusing misunderstandings.
- **C:** A serious, self-contained drama, enacted on stage, whose well-structured plot arouses pity and fear.
- **D:** A character sketch that vividly portrays one person but relates almost no action at all.
- **Correct:** C
- **Explanation:** Aristotle requires an action that is "serious, complete," presented "in the form of action, not of narrative," effecting "pity and fear," with plot as the "soul" of tragedy; C meets each criterion, while A is narrative, B is not serious, and D subordinates action to character, which he ranks second.
- **Tags:** `KC::CARS::Reasoning_Beyond_the_Text` `Prereq::CARS::Reasoning_Within_the_Text` `MCAT::CARS` `Difficulty::4` `IRT::Discrimination::1.2` `IRT::Guessing::0.25` `Reasoning::Application`

### MCAT-CARS-SENTIMENT-001

- **KC:** `CARS::Reasoning_Beyond_the_Text`
- **Prereqs:** `Prereq::CARS::Reasoning_Within_the_Text`
- **Difficulty:** 3
- **Question:** Passage (Adam Smith, *The Theory of Moral Sentiments*, ~1759): As we have no immediate experience of what other men feel, we can form no idea of the manner in which they are affected, but by conceiving what we ourselves should feel in the like situation. By the imagination we place ourselves in his situation, we conceive ourselves enduring all the same torments, we enter as it were into his body, and become in some measure the same person with him. Whatever is the passion which arises from any object in the person principally concerned, an analogous emotion springs up, at the thought of his situation, in the breast of every attentive spectator. This is the source of our fellow-feeling for the misery of others. — Question: According to the passage, a person is most likely to feel genuine sympathy for a suffering stranger when he does which of the following?
- **A:** Recalls a moral rule requiring him to assist anyone who happens to be suffering.
- **B:** Directly perceives the stranger's sensations without any effort of imagination.
- **C:** Calculates whether helping the stranger will bring him some benefit in return.
- **D:** Imagines himself in the stranger's situation and conceives what he himself would feel there.
- **Correct:** D
- **Explanation:** Smith holds that we grasp others' feelings only "by conceiving what we ourselves should feel in the like situation," placing ourselves in their situation through imagination; A substitutes rule-following, B denies the very imaginative step he requires, and C replaces fellow-feeling with self-interest.
- **Tags:** `KC::CARS::Reasoning_Beyond_the_Text` `Prereq::CARS::Reasoning_Within_the_Text` `MCAT::CARS` `Difficulty::3` `IRT::Discrimination::1.0` `IRT::Guessing::0.25` `Reasoning::Application`

### MCAT-CARS-RENAISS-001

- **KC:** `CARS::Reasoning_Beyond_the_Text`
- **Prereqs:** `Prereq::CARS::Reasoning_Within_the_Text`
- **Difficulty:** 4
- **Question:** Passage (Pater, *The Renaissance*, ~1873): To burn always with this hard, gem-like flame, to maintain this ecstasy, is success in life. Failure is to form habits: for habit is relative to a stereotyped world; meantime it is only the roughness of the eye that makes any two persons, things, situations, seem alike. While all melts under our feet, we may well catch at any exquisite passion, or any contribution to knowledge that seems, by a lifted horizon, to set the spirit free for a moment, or any stirring of the senses, strange dyes, strange flowers, and curious odours, or work of the artist's hands, or the face of one's friend. Not to discriminate every moment some passionate attitude in those about us is, on this short day of frost and sun, to sleep before evening. — Question: The author would most likely regard which of the following ways of living as the greatest success?
- **A:** Seeking out and fully savoring each vivid, passionate moment of experience.
- **B:** Settling into stable, comfortable habits that make each day reliably predictable.
- **C:** Withdrawing from the senses to contemplate unchanging, eternal truths.
- **D:** Postponing all present enjoyment in order to secure greater future rewards.
- **Correct:** A
- **Explanation:** Pater equates success with burning "with this hard, gem-like flame" and calls forming "habits" failure, urging us to "catch at any exquisite passion" and "discriminate every moment"; B is precisely what he labels failure, and C and D reject the intense present experience he prizes.
- **Tags:** `KC::CARS::Reasoning_Beyond_the_Text` `Prereq::CARS::Reasoning_Within_the_Text` `MCAT::CARS` `Difficulty::4` `IRT::Discrimination::1.15` `IRT::Guessing::0.25` `Reasoning::Application`

### MCAT-CARS-MEDIT-001

- **KC:** `CARS::Reasoning_Beyond_the_Text`
- **Prereqs:** `Prereq::CARS::Reasoning_Within_the_Text`
- **Difficulty:** 3
- **Question:** Passage (Marcus Aurelius, *Meditations*, ~170 CE): If thou art pained by any external thing, it is not this thing that disturbs thee, but thy own judgment about it. And it is in thy power to wipe out this judgment now. If anything in thy own disposition gives thee pain, who hinders thee from correcting thy opinion? Take away the complaint, "I have been harmed," and the harm is taken away. Things themselves touch not the soul, not in the least degree; nor have they admission to the soul, nor can they turn or move the soul; but the soul turns and moves itself alone. That which does not make a man worse than he was, also does not make his life worse, nor does it harm him either from without or from within. — Question: Based on the passage, how would the author most likely advise a person who feels wronged by an insult?
- **A:** Retaliate against the offender in order to restore one's damaged sense of honor.
- **B:** Recognize that the distress arises from one's own judgment, which one has the power to revise.
- **C:** Accept that the insult has genuinely harmed one's soul and worsened one's character.
- **D:** Avoid the offender permanently so that the insult can never possibly recur.
- **Correct:** B
- **Explanation:** Marcus Aurelius holds that "it is not this thing that disturbs thee, but thy own judgment," which "it is in thy power to wipe out," so the remedy is correcting one's opinion; C contradicts "things themselves touch not the soul," while A and D address the external event rather than the judgment.
- **Tags:** `KC::CARS::Reasoning_Beyond_the_Text` `Prereq::CARS::Reasoning_Within_the_Text` `MCAT::CARS` `Difficulty::3` `IRT::Discrimination::1.0` `IRT::Guessing::0.25` `Reasoning::Application`

### MCAT-CARS-TRADITION-001

- **KC:** `CARS::Reasoning_Beyond_the_Text`
- **Prereqs:** `Prereq::CARS::Reasoning_Within_the_Text`
- **Difficulty:** 4
- **Question:** Passage (Burke, *Reflections on the Revolution in France*, ~1790): You will observe, that from Magna Charta to the Declaration of Right, it has been the uniform policy of our constitution to claim and assert our liberties as an entailed inheritance derived to us from our forefathers, and to be transmitted to our posterity. This policy appears to me to be the result of profound reflection. By adhering in this manner to our forefathers, we are guided not by the superstition of antiquarians, but by the spirit of philosophic analogy. The idea of inheritance furnishes a sure principle of conservation, and a sure principle of transmission; without at all excluding a principle of improvement. A spirit of innovation is generally the result of a selfish temper and confined views. — Question: The author would most likely endorse which approach to reforming a nation's institutions?
- **A:** Sweep away existing institutions and rebuild them on wholly new principles.
- **B:** Freeze all institutions exactly as they are and forbid any change whatever.
- **C:** Introduce changes gradually, preserving continuity with inherited traditions.
- **D:** Let each generation discard the past and design society from abstract reason alone.
- **Correct:** C
- **Explanation:** Burke praises inheritance as "a sure principle of conservation... without at all excluding a principle of improvement," i.e., reform continuous with tradition; B is refuted by that "principle of improvement," while A and D describe the radical "spirit of innovation" he condemns as selfish.
- **Tags:** `KC::CARS::Reasoning_Beyond_the_Text` `Prereq::CARS::Reasoning_Within_the_Text` `MCAT::CARS` `Difficulty::4` `IRT::Discrimination::1.2` `IRT::Guessing::0.25` `Reasoning::Application`

### MCAT-CARS-GOTHIC-001

- **KC:** `CARS::Reasoning_Beyond_the_Text`
- **Prereqs:** `Prereq::CARS::Reasoning_Within_the_Text`
- **Difficulty:** 5
- **Question:** Passage (Ruskin, *The Stones of Venice*, ~1853): Accustomed to the perpetual comparison of finished work, we are apt to forget the wholesome nature of imperfection. Understand this clearly: you can teach a man to draw a straight line, and to strike a curve, and to copy any number of given lines or forms with admirable speed and perfect precision; and you find his work perfect of its kind: but if you ask him to think about any of those forms, to consider if he cannot find any better in his own head, he stops; his execution becomes hesitating; he thinks, and ten to one he thinks wrong; ten to one he makes a mistake in the first touch he gives to his work as a thinking being. But you have made a man of him for all that. He was only a machine before, an animated tool. — Question: Based on the passage, the author would most likely praise which of the following?
- **A:** A flawlessly machined part copied to exact specifications by a highly trained laborer.
- **B:** A design in which every worker suppresses his own ideas for the sake of perfect uniformity.
- **C:** A factory system prized chiefly for the speed and mechanical precision of its output.
- **D:** A hand-carved ornament, with visible irregularities, made by a workman free to invent as he goes.
- **Correct:** D
- **Explanation:** Ruskin calls imperfection "wholesome" and values the workman who thinks for himself, even when he "makes a mistake," over the precise copyist who "was only a machine"; D rewards that inventive freedom, while A, B, and C celebrate the mechanical precision and suppression of thought he criticizes.
- **Tags:** `KC::CARS::Reasoning_Beyond_the_Text` `Prereq::CARS::Reasoning_Within_the_Text` `MCAT::CARS` `Difficulty::5` `IRT::Discrimination::1.3` `IRT::Guessing::0.25` `Reasoning::Application`

### MCAT-CARS-CAVE-001

- **KC:** `CARS::Reasoning_Beyond_the_Text`
- **Prereqs:** `Prereq::CARS::Reasoning_Within_the_Text`
- **Difficulty:** 3
- **Question:** Passage (Plato, *Republic*, ~375 BCE): Behold human beings living in an underground den; here they have been from their childhood, and have their legs and necks chained so that they cannot move, and can only see before them, being prevented by the chains from turning round their heads. Above and behind them a fire is blazing at a distance, and between the fire and the prisoners there is a raised way; and along the way a low wall has been built. And do you see men passing along the wall carrying vessels, and statues and figures of animals, which appear over the wall? To the prisoners the truth would be literally nothing but the shadows of the images. And if they were released, at first they would still persist in maintaining the superior truth of the shadows. — Question: Which modern situation best parallels the prisoners' condition as it is described in the passage?
- **A:** People who mistake flickering images on a screen for reality, never seeing what casts them.
- **B:** Students who master a subject through years of rigorous, firsthand investigation.
- **C:** Travelers who leave home specifically to observe unfamiliar customs directly.
- **D:** Scientists who readily revise their theories whenever new evidence appears.
- **Correct:** A
- **Explanation:** The prisoners are chained facing shadows cast by unseen objects and take "the shadows of the images" for the whole truth; A mirrors mistaking projected images for reality while ignoring their source, whereas B, C, and D describe people who actively seek or accept fuller reality — the opposite of the prisoners.
- **Tags:** `KC::CARS::Reasoning_Beyond_the_Text` `Prereq::CARS::Reasoning_Within_the_Text` `MCAT::CARS` `Difficulty::3` `IRT::Discrimination::1.05` `IRT::Guessing::0.25` `Reasoning::Application`
