# Concept Scheduler for Anki (MCAT Integration)

Anki is exceptionally proficient at optimizing memory retention (when to review an existing card) using spaced repetition algorithms. However, it lacks systemic awareness of pedagogical dependency (what new material or concepts to introduce next). Currently, new cards are introduced purely linearly by creation order or randomly. 

For massive, highly interconnected subjects like the MCAT (~6,000 cards), this forces students to manually micromanage countless sub-decks to avoid seeing advanced concepts before mastering foundational prerequisites. This project resolves this limitation by integrating an optional Bayesian Knowledge Component (KC) Engine directly into Anki's Rust core (rslib).

---

## User Persona

The primary user is a busy undergraduate preparing for the MCAT on a compressed three-month timeline. They need to review the full scope of high-yield MCAT content efficiently while balancing coursework, labs, clinical volunteering, and other obligations. They are willing to practice every day, but need the app to protect their time and tell them what matters most.

This student is not only trying to memorize facts. They need Anki to help them build the conceptual fluency required for MCAT-style logical reasoning, critical thinking, and reading comprehension. The scheduler should therefore prioritize new material in a way that strengthens prerequisite understanding before introducing more advanced passages, mechanisms, and reasoning-heavy concepts.

---

## 1. MVP Definition

The MVP must prove one thing clearly: Anki can choose better new concepts without breaking its existing review system.

The first build should include:

- A real Rust-side scheduler change that affects new-card selection, not just a Python or UI prototype.
- A Knowledge Component lattice built from structured card metadata, where each component can list prerequisite components.
- Tracking of the learner's **inner fringe** and **outer fringe**:
  - **Inner fringe:** concepts the learner is currently strong enough to build from.
  - **Outer fringe:** next eligible concepts whose prerequisites appear sufficiently ready.
- Configurable mastery thresholds. MVP defaults: a KC enters the inner fringe at 0.85 mastery with at least 3 answered cards; a KC enters the outer fringe when all prerequisites are at least 0.70 mastery.
- A fixed daily new-topic budget by card count. For the MVP, use a 1:4 ratio of new-topic cards to review cards, while keeping the setting configurable for future adaptation.
- A review-first session flow. The learner reviews old cards first; after enough review work is completed, the app may introduce a selected new topic; if time remains afterward, the learner returns to review.
- A simple topic choice flow: when new topics are allowed, show the learner 3-5 recommended outer-fringe topics and let them choose which one to learn next.
- Fallback behavior: if no outer-fringe topic is ready, the app only reviews previous cards. If there is not enough review history yet, the app enters a calibration phase and samples cards to collect evidence about the learner.
- Bayesian mastery updates. Card answers update KC probabilities with Bayes' theorem, using answer quality as evidence instead of applying fixed additive point changes.
- Persistence of concept mastery, topic choices, and progress between sessions.
- A visible **Concept Scheduler Mode** toggle for turning the feature on and off.
- Honest score behavior: if there is not enough evidence for a readiness estimate, the app must say so instead of inventing a number.

Not in the MVP:

- Adaptive new-topic limits. The 20/80 split is fixed for now; adapting it to the learner can come later.
- A full MCAT score predictor unless held-back validation data is available. The MVP should hide readiness scoring until a configurable evidence threshold is met.
- AI-generated explanations, cards, or topic maps unless they have named sources and a test set.
- Mobile implementation and mobile sync. The first build focuses on desktop engine behavior; the Android companion comes later.
- A complex analytics dashboard. The first UI should show only the next topic choices, weak areas, the prerequisite-violation counter, and any score ranges the system can honestly support.

---

## 2. The Rules You Cannot Break

All of these are required. Details come later, but this is the short list:

- Make a real change inside Anki's Rust code, not just the Python screens.
- Ship two apps that share one engine: a desktop app and a phone companion. Reviews and progress sync between them.
- Show three separate scores: memory, performance, and readiness. Each score must include a range, not one blended number.
- Test models on held-back data using a setup someone else can re-run and get the same result.
- Pick one study feature, write down what it is expected to do, and test it by turning it off and on.
- Every AI output must come from a named source, get checked against a test set, and beat a simpler method.
- The app must refuse to give a score when it does not have enough data.
- Ship a desktop installer and a phone build that both run with AI switched off.
- Follow the license: AGPL-3.0-or-later, with credit to Anki. Some parts of Anki use BSD-3-Clause.
- Making up a readiness number, or dressing up a guess as a measurement, is an automatic fail. Honest numbers are more valuable than flattering ones.

---

## 3. System Architecture & Scaling Strategy

To prevent performance degradation across a 6,000-card MCAT dataset, the engine decouples individual flashcards from the underlying mathematical nodes. 

Instead of building a network of 6,000 distinct nodes, the system tracks a fixed array of roughly 100 high-yield Knowledge Components (KCs) mapped to the official AAMC blueprint. The Rust core handles the database queries and heavy probability updates instantly, ensuring zero UI lag or stuttering during card transitions.

---

## 4. MVP Tech Stack

The MVP should use Anki's existing architecture wherever possible. The goal is to add one shared scheduling engine and keep both user interfaces thin.

- **Core engine:** Rust inside Anki's `rslib`. This is where Knowledge Component tracking, inner/outer fringe logic, new-topic budgeting, IRT estimates, and new-card sequencing should live.
- **Desktop app:** Anki desktop, using the existing Python/PyQt shell and Svelte/TypeScript deck-options UI. The desktop app is the primary place to configure the feature and inspect progress.
- **Phone companion:** Android app built with Kotlin and Jetpack Compose in a later phase. Mobile sync is not required for the first desktop-engine build.
- **Storage:** Anki's existing SQLite collection database and config storage. The MVP should store KC mastery, fringe state, IRT estimates, and topic choices in isolated JSON-compatible payloads instead of requiring a broad schema migration.
- **Sync/API layer:** Deferred until the phone companion phase. The MVP data model should still be designed so review answers, topic choices, and progress can be synced later without changing the core state model.
- **Validation tooling:** Re-runnable scripts or tests that evaluate the model on held-back data and compare it against simpler baselines.

The first implementation should focus on desktop engine behavior. When the Android app is added later, it should not reimplement the scheduler independently. It should either call into the shared engine through synced state or consume decisions produced by the desktop/Rust engine.

---

## 5. Product Architecture & Required System Changes

This project changes Anki from a tool that only asks "when should this card be reviewed again?" into a study system that also asks "what concept should this student learn next?" The implementation should remain optional and deck-specific so existing Anki behavior stays unchanged unless the user enables the MCAT sequencing engine.

### A. Add a Pedagogical Layer on Top of New Cards

Anki already handles review timing well, so the first change should focus on new-card selection. Instead of introducing new cards only by creation order, deck order, or randomness, the scheduler should rank new cards by conceptual readiness.

This is needed because MCAT prep is cumulative. A student should usually strengthen prerequisite components before seeing cards that depend on them. The Bayesian Knowledge Component model provides that readiness signal by tracking which foundational concepts appear mastered and which target concepts still need exposure.

### B. Track Knowledge Components and Prerequisites

Cards need a way to declare what they teach and what they depend on. The exact input format requires research before it is finalized, because the system needs to be compatible with real MCAT decks and not just a hand-authored demo. The lightweight MVP should start with structured Anki tags such as `KC::Renal_Clearance` and `Prereq::Hydrodynamics`, then refine the convention after testing with sample decks.

This is needed so the scheduler can reason about content instead of treating every card as an isolated fact. The system should infer a card's target concept, prerequisite concepts, and MCAT topic area from metadata that can be added to existing decks without requiring users to redesign their notes from scratch.

### C. Maintain the Inner and Outer Fringes of the Knowledge Lattice

The Knowledge Component graph should behave like a lattice of learnable concepts. At any point, the system should know which concepts are inside the learner's current working boundary and which concepts are just outside it.

The **inner fringe** contains concepts the learner appears ready to build from. For the MVP default, a KC joins the inner fringe when mastery is at least 0.85 and the learner has answered at least 3 cards for that KC. The **outer fringe** contains concepts that are not yet learned but are now reasonable candidates because all prerequisite KCs have mastery of at least 0.70.

This is needed so new-topic selection is constrained. The app should not search the entire MCAT deck for the next card; it should search the outer fringe and recommend concepts that are adjacent to what the student already understands.

### D. Limit New Topic Introductions Per Day

The MVP should reserve most study work for Anki's existing review obligations. Use a card-count budget instead of a time estimate: for every 4 review cards, the scheduler may allow up to 1 new-topic card. This preserves the intended 80/20 review-to-new balance while making the rule easy to implement and test.

Each session should start with reviews. After enough review work has been completed and the new-topic budget is available, the app should show 3-5 outer-fringe topics and let the learner choose which topic to start. After the learner chooses, the scheduler introduces new cards from that topic while still respecting Anki's normal daily limits, burying behavior, and due review priority. If study time remains afterward, the app returns to review.

This is needed because the user has limited time and should not be flooded with new content. Adaptive new-topic limits are explicitly a later feature; the MVP should use a simple fixed budget that is easy to explain and test.

### E. Calibration and Fallback Behavior

If no outer-fringe topic is ready, the app should not force new material. It should fall back to reviewing previous cards and collecting more evidence.

If the learner is new and has little or no review history, the app enters a calibration phase. Calibration should start with prerequisite KCs interleaved across important foundational areas, then continue to harder cards only when prerequisite evidence is sufficient. Sampling should prioritize important prerequisite KCs while allowing some randomness so the system can discover unexpected strengths or weaknesses. The production default should require a configurable target such as 500 seen cards before showing readiness estimates, while the demo can lower this threshold for a fake 50-card deck.

### F. Update Student Ability After Each Answer

Every card answer should update the student's estimated mastery for the related Knowledge Components using Bayes' theorem. A missed or difficult card becomes negative evidence; a correct or easy card becomes positive evidence. The update should revise the probability that the learner has mastered the KC given the observed answer and the card's estimated diagnostic value.

This is needed because the best next concept depends on the student's current state, not just the static deck order. The model should continuously adapt as the student improves, struggles, or forgets material.

### G. Add Item Response Theory for MCAT Score Estimation

In addition to concept mastery, the system should use Item Response Theory (IRT) to estimate the student's likely MCAT performance. Each card or question-like item can carry IRT parameters:

- **Difficulty:** how advanced or challenging the item is.
- **Discrimination:** how strongly the item distinguishes between lower and higher ability students.
- **Guessing:** the probability of getting the item right by chance.

The student's responses update an estimated ability score, which can then be mapped to an approximate MCAT readiness score or section-level score estimate. This is needed because mastery percentages alone do not tell the student whether they are on track for their target score. IRT gives a more test-like estimate by weighting hard, diagnostic questions more heavily than easy recall cards.

### H. The Honesty Rule for Readiness Scores

The app may not show an MCAT readiness score unless it can also show:

- what evidence produced the number,
- what data is still missing,
- how accurate the system's past guesses turned out to be,
- the likely score range, not just a single number,
- and the single best next thing to study.

This rule prevents the system from presenting false precision. A readiness score should feel like an explainable estimate, not a black-box prediction. If the evidence is too thin, the app should say that directly and focus the student on the next useful diagnostic or study action.

### I. Ship Two Apps That Share One Engine

The full product must ship as both a desktop app and a phone companion. This is not required for the first desktop-engine build, but the core state model should be designed with mobile sync in mind. Both apps should eventually use the same underlying scheduling, Knowledge Component, and IRT logic so the student gets consistent recommendations no matter where they study.

Reviews and progress must sync between the two apps. If a student answers cards on the phone between classes, the desktop app should reflect those answers, update the same mastery estimates, and avoid recommending stale next steps. This is needed because the target user studies in short sessions across multiple contexts, and fragmented progress would make the readiness model untrustworthy.

### J. Store State Without Disrupting Existing Collections

The system needs to persist the Knowledge Component lattice, IRT ability estimates, and any derived MCAT readiness metrics between sessions.

The MVP storage model should be a directed graph. Each node is a Knowledge Component containing its ability probability, prerequisite edges, cards completed, unseen cards, and any per-node evidence counts needed for calibration. The same payload should also include overall daily metrics such as review cards completed, new-topic cards introduced, topic choices, and budget used.

This should use an isolated config payload inside Anki's existing collection storage rather than a broad database migration for the first implementation. That keeps the feature safer to test locally, easier to disable, and less likely to corrupt existing profiles.

### K. Keep the User Experience Simple

The user should see a small number of controls and outputs: enable or disable MCAT sequencing for a deck, view weak Knowledge Components, and view an estimated MCAT readiness score.

This is needed because the target user is a busy undergraduate with only three months to prepare. The system should reduce micromanagement, not add another dashboard the student has to constantly tune.

---

## 6. Functional Requirements

### FR-1: Taxonomic Tag Mapping

The system infers conceptual dependencies using standard Anki note tags formatted with strict prefixes for the MVP. The final input format is a research task and should be validated against real or representative MCAT deck data before it is treated as stable.

- Syntax: KC::[Component_Name] and Prereq::[Component_Name]
- Example: A card teaching advanced clearance would be tagged KC::Renal_Clearance and Prereq::Hydrodynamics.

### FR-2: Knowledge Lattice Fringe Tracking

The system maintains a graph of Knowledge Components and classifies each component into one of three states:

- **Inside the learned region:** concepts with enough evidence of mastery.
- **Inner fringe:** concepts with mastery at or above the configurable threshold. MVP default: 0.85 mastery and at least 3 answered cards.
- **Outer fringe:** not-yet-learned concepts whose prerequisites are all ready enough. MVP default: every prerequisite is at or above 0.70 mastery.

The outer fringe is the only pool used for new-topic recommendations in the MVP.

### FR-3: Review-First Daily New-Topic Budget and Topic Choice

The MVP uses a fixed card-count allocation: for every 4 review cards completed, the scheduler may allow up to 1 new-topic card. The learner starts by reviewing old cards first.

When enough review work has been completed and the new-topic budget is available, the app shows 3-5 recommended outer-fringe topics. The learner chooses one, and the scheduler introduces new cards for that topic while preserving Anki's existing review priority and daily limits. If time remains afterward, the learner returns to review.

### FR-4: Bayesian Mastery Probability Updates

When a card is answered, its grade is converted into evidence for Bayes' theorem:

- Again (1) / Hard (2) results in Negative Evidence (0)
- Good (3) / Easy (4) results in Positive Evidence (1)

The system updates the probability of mastery for the related KC using the answer evidence and the card's diagnostic assumptions. The MVP should start with simple configurable likelihood values, then improve them as better item-level data becomes available.

### FR-5: Pedagogical Sorting Logic

New cards are sorted using a dynamic readiness-to-learn score:
Score = P(Prerequisites Mastered) * (1 - P(Target KC Mastered))

This mathematically prioritizes concepts where the student has strong foundational knowledge but has not yet mastered the target topic.

### FR-6: Safe State Persistence

To avoid database schema migration failures, the Knowledge Component graph is serialized into an isolated JSON-compatible config payload. Each node stores ability probabilities, prerequisite edges, cards done, unseen cards, and evidence counts. The payload also stores overall daily metrics, including review cards completed, new-topic cards introduced, topic choices, and budget usage.

### FR-7: Honest Score Refusal

The app must refuse to show a readiness score when it lacks enough evidence, validation history, or score-range confidence. The production default should require a configurable threshold such as 500 seen cards plus sufficient coverage of prerequisites and important components. The demo may lower this threshold for a fake 50-card deck. When the threshold is not met, the app should explain what data is missing and recommend the next best diagnostic or study action.

### FR-8: Calibration and Fallback Behavior

If no outer-fringe topic is ready, the scheduler falls back to reviewing previous cards. If the learner has too little history for meaningful recommendations, the scheduler enters calibration mode. Calibration starts with prerequisite KCs interleaved across important foundational areas, includes some randomized sampling, and only moves to harder components when prerequisite evidence is sufficient.

### FR-9: Concept Scheduler Mode Toggle

The desktop UI must expose a visible **Concept Scheduler Mode** toggle. When the mode is on, the scheduler uses the KC fringe logic. When the mode is off, Anki uses baseline new-card ordering so prerequisite violations and mastery gain can be compared.

---

## 7. Performance & Safety Benchmarks

- **MCAT Scale Bound:** The sorting engine operates at O(N) complexity where N is confined to the daily gathered card pool (fewer than 1,000 cards), completing execution in under 15ms.
- **Graceful Degradation:** If a card contains no KC tags, or if an asset loop is detected, the engine defaults its score evaluation to 0.5, allowing it to fall back safely to standard creation order sequence.
- **Configurable Evidence Thresholds:** Production defaults can target values such as 500 seen cards for readiness scoring, but tests and demos must be able to lower thresholds so a fake 50-card deck can exercise the full flow.
- **Prerequisite Violation Counter:** The MVP must count how often the scheduler introduces a card whose prerequisites are not ready. This counter should appear in the top right of the relevant desktop view and should be compared with Concept Scheduler Mode turned on and off.
- **Baseline Collection:** The first validation baseline is normal Anki ordering with Concept Scheduler Mode off. The system should collect enough data for each user before presenting strong claims.
- **Feature Toggle Evaluation:** Concept Scheduler Mode must have a visible on/off control so one study feature can be tested against the baseline Anki ordering.
- **Mastery Gain Measurement:** The evaluation should attempt to measure mastery gain by comparing KC mastery before and after study sessions. The first version may use synthetic or hand-authored demo data if real medical-domain labels are unavailable.
- **Demo Dataset:** The first demo should use a fake 50-card deck with a small Knowledge Component graph, prerequisite edges, answer history, and enough variation to show calibration, outer-fringe topic choice, fallback review behavior, and score refusal. The initial KC graph is pending research and will be supplied after the MCAT topic mapping is drafted.

