# Manual Lesson Pages (Track C)

Design + stubs only. This document defines a per-KC lesson-page schema, a few
concrete demo lesson stubs, the two reviewer entry points, and the contract
inputs lesson pages need from Track A (KC map) and Track D (topic picker).

No app code is implemented here. This is a docs/design artifact so later
implementation has a frozen target. All lesson content in this file is
synthetic and originally written for the demo; it is not copied from any
copyrighted prep material.

## Goal

Prevent retrieval before initial encoding. A brand-new Knowledge Component
(KC) should be *taught* briefly before the scheduler starts quizzing it, and a
learner should be able to jump from any card back to its concept lesson after
they see the answer. Lessons stay lightweight and local-first.

Ties into the wider plan:

- Track C in `added features/next-feature-expansion-plan.md`.
- KC ids and prerequisite edges come from `added features/mcat.md` and the
  canonical demo graph in `rslib/src/scheduler/concept_demo.rs`.
- AI-generated lesson text is explicitly out of scope for going live until the
  bring-your-own-key + source/eval rules exist (Priority 2 / Track H). See
  "Local-first and AI gating".

## Status and Scope

- In scope now: schema, demo stubs, entry-point design, contract inputs.
- Out of scope now: parser, storage wiring, reviewer buttons, any AI text.
- Local-first: lessons render from content shipped in the repo (or authored
  per profile), with no network call required to study.

## Lesson-Page Schema

Every KC gets exactly one lesson page. The seven required sections are:

1. `overview` - 2-4 sentences framing what the KC is and why it matters.
2. `key_concepts` - a short bullet list of the load-bearing facts/terms.
3. `prerequisite_reminder` - one line reactivating the prior KC(s) this builds
   on. Derived from the KC graph, not hand-maintained (see Lesson Contract).
4. `worked_example` - one small, fully-explained example so the first retrieval
   is not cold.
5. `common_misconception` - the single most common wrong mental model, named
   and corrected.
6. `first_retrieval_prompt` - one low-stakes recall prompt that bridges the
   lesson into quizzing. Must NOT duplicate a real card's answer verbatim.
7. `related_kcs` - neighboring KCs (prerequisites + downstream targets) for
   navigation. Derived from the KC graph.

### Data format

The **human-authored source of truth** is Markdown, mirroring the parser-friendly
convention already used by `added features/mcat_demo_cards.md`
(`### <id>` blocks with `- **Key:** value` bullets, backtick-wrapped KC ids,
`Prereqs: none` for foundations). A future parser can mirror
`parse_mcat_demo_cards()` in `rslib/src/scheduler/concept_demo.rs` and load this
file via `include_str!`.

Per-lesson Markdown block:

```markdown
## <KC id>

### LESSON-<KC-SLUG>

- **KC:** `Bio::DNA`
- **Title:** <human-readable title>
- **Section:** `MCAT::Bio_Biochem`
- **Source:** authored            <!-- authored | ai_generated -->
- **Review Status:** approved     <!-- draft | needs_review | approved -->
- **Overview:** <2-4 sentences>
- **Key Concepts:**
  - <point 1>
  - <point 2>
- **Prerequisite Reminder:** <one line; references `KC::...` or "foundation">
- **Worked Example:** <one small worked example>
- **Common Misconception:** <named misconception + correction>
- **First Retrieval Prompt:** <one recall prompt, not a verbatim card answer>
- **Related KCs:** `Bio::Genetics`, `Biochem::Nucleotides_and_Nucleic_Acids`
```

The **normalized machine shape** (what a renderer or RPC would consume) is a
small JSON/YAML object. This is the form Track D / the reviewer would receive:

```json
{
  "id": "LESSON-BIO-DNA",
  "kc": "Bio::DNA",
  "title": "DNA: Structure and Base Pairing",
  "section": "MCAT::Bio_Biochem",
  "source": "authored",
  "review_status": "approved",
  "overview": "…",
  "key_concepts": ["…", "…"],
  "prerequisite_reminder": {
    "text": "…",
    "prerequisite_kcs": []
  },
  "worked_example": "…",
  "common_misconception": "…",
  "first_retrieval_prompt": "…",
  "related_kcs": ["Bio::Genetics", "Biochem::Nucleotides_and_Nucleic_Acids"]
}
```

### Field reference

| Field | Type | Required | Source | Notes |
| --- | --- | --- | --- | --- |
| `id` | string | yes | authored | Stable `LESSON-<slug>`; slug from KC id. |
| `kc` | KC id | yes | authored | Must be a real KC in the Track A / demo graph. |
| `title` | string | yes | authored | Short display title. |
| `section` | `MCAT::…` tag | yes | derived (Track A) | Blueprint section(s) the KC serves. |
| `source` | enum | yes | system | `authored` renders live; `ai_generated` is gated off. |
| `review_status` | enum | yes | system | Only `approved` renders live. |
| `overview` | string | yes | authored | 2-4 sentences. |
| `key_concepts` | string[] | yes | authored | 3-6 bullets. |
| `prerequisite_reminder` | object | yes | derived + authored | `prerequisite_kcs` from graph; `text` authored. |
| `worked_example` | string | yes | authored | One fully-explained example. |
| `common_misconception` | string | yes | authored | Named + corrected. |
| `first_retrieval_prompt` | string | yes | authored | Bridge to quizzing; not a card answer. |
| `related_kcs` | KC id[] | yes | derived (Track A) | Prereqs + downstream targets. |

### Optional: `diagram` field (static, local-first)

Lessons may carry one **optional** `diagram` field (a `- **Diagram:**` bullet).
It is additive: lessons without it render exactly as before, so it does not
change the seven required sections or the display gate. Two authorable forms,
both text-based, original, and shipped in-repo (no copyrighted images, no
network calls):

- **Mermaid** — preferred for *orderings of stages / processes / cycles* (e.g.
  glycolysis, the cell cycle, the central dogma, signaling cascades): a fenced
  `mermaid` block. Renders in the lesson panel via mermaid.js and degrades to
  readable text if unrendered.
- **SVG** — preferred for *structural / spatial* diagrams (e.g. DNA base
  pairing, the phospholipid bilayer, a labeled eukaryotic cell): hand-authored
  inline `<svg>` or a reference to an `.svg` under `lesson-diagrams/`. Text,
  scalable, and accurate because it is authored, not generated.

Rule: diagrams are **authored** (never AI-raster-generated) so every label is
verifiable — the same accuracy bar as the rest of the lesson. `alt` / `<title>`
text is required for accessibility. Live examples: `lessons-biology.md`
(`Bio::DNA`, SVG) and `lessons-biochemistry.md` (`Biochem::Glycolysis`, Mermaid).

### Lesson validation rules (design intent)

- Every demo KC has exactly one lesson stub (acceptance criterion for Track C).
- `kc` and every `related_kcs` / `prerequisite_kcs` entry must exist in the KC
  graph.
- `prerequisite_kcs` must equal the KC's prerequisites in the canonical graph
  (no hand-drift). Foundations have an empty list and a "foundation" reminder.
- `first_retrieval_prompt` must not be a verbatim copy of any card's `Correct`
  answer or `Explanation` for the same KC.
- Only `source: authored` + `review_status: approved` lessons render in the
  live app; anything else is hidden behind the AI gate.

## Demo Lesson Stubs

Five synthetic stubs covering a spread of the canonical demo chain. Prerequisite
reminders and related KCs match the edges in
`rslib/src/scheduler/concept_demo.rs`
(`DEMO_EDGES`) so the schema stays consistent with the graph.

## Bio::DNA

### LESSON-BIO-DNA

- **KC:** `Bio::DNA`
- **Title:** DNA: Structure and Base Pairing
- **Section:** `MCAT::Bio_Biochem`
- **Source:** authored
- **Review Status:** approved
- **Overview:** DNA is the molecule that stores heritable information as a
  linear sequence of four bases. Its double-helix shape and predictable base
  pairing are what let the sequence be copied and read reliably. Almost every
  later molecular-biology topic assumes you are fluent with this structure.
- **Key Concepts:**
  - A nucleotide has three parts: a deoxyribose sugar, a phosphate, and one of
    four bases (A, T, G, C).
  - Strands run antiparallel (5' to 3' opposite 3' to 5') and are joined by a
    sugar-phosphate backbone.
  - Base pairing is specific: A pairs with T (two hydrogen bonds), G pairs with
    C (three hydrogen bonds).
  - More G-C content means more hydrogen bonds, so the helix is harder to melt
    apart.
- **Prerequisite Reminder:** Foundation KC - no prerequisites assumed beyond
  basic biology vocabulary.
- **Worked Example:** One strand reads 5'-GCTA-3'. Its complement must pair
  base-by-base and run antiparallel, so the partner strand is 3'-CGAT-5' (write
  it 5'-TAGC-3' when read in its own 5' to 3' direction). Notice each G lines up
  with a C and each A with a T.
- **Common Misconception:** "A always pairs with G because they are both
  purines." Pairing is not about purine-vs-purine; it is complementary: a purine
  always pairs with a specific pyrimidine (A-T, G-C) so the helix stays a uniform
  width.
- **First Retrieval Prompt:** Without looking back, state which base pairs with
  guanine and how many hydrogen bonds hold that pair together.
- **Related KCs:** `Bio::Genetics`, `Biochem::Nucleotides_and_Nucleic_Acids`

## Bio::Genetics

### LESSON-BIO-GENETICS

- **KC:** `Bio::Genetics`
- **Title:** Classical Genetics: Alleles and Inheritance
- **Section:** `MCAT::Bio_Biochem`
- **Source:** authored
- **Review Status:** approved
- **Overview:** Genetics is how the information stored in DNA is passed from one
  generation to the next. It connects an organism's underlying alleles
  (genotype) to observable traits (phenotype) using simple probability rules.
- **Key Concepts:**
  - A gene can exist as different versions called alleles.
  - Genotype is the allele pair an organism carries; phenotype is the trait you
    observe.
  - A dominant allele masks a recessive allele in a heterozygote.
  - A Punnett square predicts offspring genotype ratios from parent genotypes.
- **Prerequisite Reminder:** You should already be comfortable with `Bio::DNA`:
  genes are stretches of DNA sequence, and alleles are sequence variants of the
  same gene.
- **Worked Example:** Cross two heterozygotes, Aa x Aa. The Punnett square gives
  offspring 1 AA : 2 Aa : 1 aa. Because A is dominant, three of four show the
  dominant phenotype and one of four shows the recessive phenotype (a 3:1 ratio).
- **Common Misconception:** "Dominant means more common in the population."
  Dominance only describes which allele is expressed in a heterozygote; a
  dominant allele can be rare, and a recessive one can be common.
- **First Retrieval Prompt:** From memory, predict the phenotype ratio of an
  Aa x aa cross and explain why it differs from Aa x Aa.
- **Related KCs:** `Bio::DNA`, `Bio::Evolution`

## Biochem::Amino_Acids

### LESSON-BIOCHEM-AMINO-ACIDS

- **KC:** `Biochem::Amino_Acids`
- **Title:** Amino Acids: The Building Blocks
- **Section:** `MCAT::Bio_Biochem`
- **Source:** authored
- **Review Status:** approved
- **Overview:** Amino acids are the 20 monomers that proteins are built from.
  They share a common core and differ only in a side chain, and that side chain
  is what gives each amino acid its chemical personality.
- **Key Concepts:**
  - Every amino acid has a central (alpha) carbon bonded to an amino group, a
    carboxyl group, a hydrogen, and a variable R side chain.
  - Side chains are grouped as nonpolar, polar uncharged, acidic (negative), or
    basic (positive).
  - Near physiological pH an amino acid is a zwitterion: the amino group is
    protonated (+) and the carboxyl group is deprotonated (-).
  - Side-chain chemistry drives how the eventual protein folds and functions.
- **Prerequisite Reminder:** Foundation KC - no prerequisites, but a working
  sense of acids/bases and polarity helps.
- **Worked Example:** Compare glycine (R = H) with aspartate (R = CH2-COO-).
  Glycine's tiny nonpolar side chain is flexible and fits tight turns; aspartate
  carries a negative charge at physiological pH, so it prefers the water-facing
  surface of a protein and can form salt bridges. Same backbone, very different
  behavior - the side chain decides.
- **Common Misconception:** "All amino acids are acidic because of the carboxyl
  group." Every amino acid has both an acidic carboxyl and a basic amino group;
  whether we call the *side chain* acidic or basic depends only on the R group.
- **First Retrieval Prompt:** From memory, name the four parts bonded to the
  alpha carbon and state which one distinguishes one amino acid from another.
- **Related KCs:** `Biochem::Peptides_and_Proteins`

## Biochem::Enzymes

### LESSON-BIOCHEM-ENZYMES

- **KC:** `Biochem::Enzymes`
- **Title:** Enzymes: Catalysts and Kinetics
- **Section:** `MCAT::Bio_Biochem`
- **Source:** authored
- **Review Status:** approved
- **Overview:** Enzymes are protein catalysts that speed up reactions by
  lowering activation energy without being consumed. Their shape gives them
  specificity, and simple kinetics let us describe how fast they work.
- **Key Concepts:**
  - Enzymes lower activation energy; they do not change the reaction's overall
    energy difference or equilibrium position.
  - The substrate binds a specific active site; shape and chemistry make the fit
    selective.
  - Michaelis-Menten kinetics: Vmax is the maximum rate; Km is the substrate
    concentration at half of Vmax and reflects apparent affinity.
  - Inhibitors change apparent kinetics: competitive inhibitors raise apparent
    Km (same Vmax); noncompetitive inhibitors lower Vmax.
- **Prerequisite Reminder:** Build on `Biochem::Protein_Structure_and_Function`:
  an enzyme's catalytic power comes from its folded 3-D active site, so the same
  forces that fold proteins determine how substrates bind.
- **Worked Example:** Add a competitive inhibitor that resembles the substrate.
  Adding more substrate out-competes it, so the reaction can still reach the same
  Vmax, but it takes more substrate to get to half-max - so measured Km goes up
  while Vmax is unchanged.
- **Common Misconception:** "Enzymes make reactions more thermodynamically
  favorable." Enzymes only change the *rate* (the kinetic barrier). They cannot
  make a non-spontaneous reaction spontaneous or shift the equilibrium constant.
- **First Retrieval Prompt:** From memory, describe what happens to Km and Vmax
  under competitive inhibition, and explain why adding substrate helps.
- **Related KCs:** `Biochem::Protein_Structure_and_Function`,
  `Biochem::Bioenergetics`

## Biochem::Glycolysis

### LESSON-BIOCHEM-GLYCOLYSIS

- **KC:** `Biochem::Glycolysis`
- **Title:** Glycolysis: Glucose to Pyruvate
- **Section:** `MCAT::Bio_Biochem`
- **Source:** authored
- **Review Status:** approved
- **Overview:** Glycolysis is the pathway that splits one glucose into two
  pyruvate molecules in the cytoplasm. It is the shared entry point for both
  aerobic and anaerobic energy metabolism, so it shows up everywhere downstream.
- **Key Concepts:**
  - It runs in the cytoplasm and needs no oxygen.
  - An early investment phase spends 2 ATP; a later payoff phase produces 4 ATP
    and 2 NADH.
  - Net yield per glucose: 2 ATP, 2 NADH, and 2 pyruvate.
  - Pyruvate's fate depends on oxygen: it feeds the citric acid cycle when
    oxygen is present, or fermentation when it is not.
- **Prerequisite Reminder:** Build on `Biochem::Bioenergetics`: glycolysis is a
  worked case of coupling an unfavorable step to ATP hydrolysis and tracking
  energy carriers, so keep ATP/NADH bookkeeping in mind.
- **Worked Example:** Track the ATP ledger for one glucose. Investment phase:
  -2 ATP (spent to phosphorylate and prime the sugar). Payoff phase: +4 ATP
  (made as high-energy intermediates are cashed in). Net = 4 - 2 = 2 ATP, plus
  2 NADH carried forward.
- **Common Misconception:** "Glycolysis makes a lot of ATP by itself." Its net
  direct yield is only 2 ATP per glucose; most cellular ATP comes later from the
  citric acid cycle and oxidative phosphorylation using the NADH glycolysis
  hands off.
- **First Retrieval Prompt:** From memory, give the net ATP, NADH, and pyruvate
  produced per glucose, and say where in the cell this happens.
- **Related KCs:** `Biochem::Bioenergetics`, `Biochem::Citric_Acid_Cycle`

## Entry Points (Design Only)

Two entry points, both wiring into `qt/aqt/reviewer.py`. Nothing below is
implemented; these are the concrete attachment spots so implementation is
unambiguous. A lesson panel would be a local overlay rendered from the stored
lesson content, modeled on the existing Concept Graph sidebar.

### Shared plumbing (mirrors the existing concept sidebar)

- **Panel markup:** `revHtml()` injects `#_concept_kc_badge` and
  `#_concept_graph_sidebar` at `reviewer.py:336-339`. A sibling
  `<div id="_concept_lesson_panel" hidden></div>` would live here, plus
  `window._renderLessonPanel(payload)` / `window._hideLessonPanel()` JS helpers
  modeled on `window._renderConceptGraphSidebar` / `window._hideConceptGraphSidebar`
  (`reviewer.py:381-489`).
- **Resolve current KC:** `_concept_labels(card)` (`reviewer.py:526-534`) already
  extracts `KC::` labels for the current card - reuse it to know which lesson to
  open.
- **Toggle/open method:** a new `_open_lesson_for_current_card()` /
  `_toggle_lesson_panel()` modeled on `_toggle_concept_graph_sidebar`
  (`reviewer.py:619-624`).
- **Command routing:** add an `elif url == "lesson":` branch in `_linkHandler`
  right after the existing `elif url == "conceptGraph":` branch
  (`reviewer.py:943-944`).

### (a) Lesson at the START of a new KC/topic (before quizzing)

Purpose: when a KC is first introduced, show its lesson before its first card.

- **Trigger from Track D topic picker (primary):** the plan says after a topic
  is chosen, "open the lesson page first, then introduce cards from that topic."
  When Track D sets the selected topic, it opens that KC's lesson panel, and only
  serves cards for that KC after the learner dismisses the lesson with a "Start
  retrieval" action. Natural hook: `nextCard()` (`reviewer.py:248-264`), which
  fetches the next card and calls `_showQuestion()`.
- **Trigger on first encounter (fallback):** inside `_showQuestion`
  (`reviewer.py:626-663`), immediately after `_update_concept_graph_sidebar(c)`
  (`reviewer.py:658`) and before `_showAnswerButton()` (`reviewer.py:659`), a
  gate could check "is this card's KC newly introduced / not yet encoded for this
  learner?" If so, render the lesson panel first; the learner reads it, then
  reveals/answers as usual. The "already encoded" flag is a per-KC,
  per-profile local bit (see contract inputs), not synced state.
- **Non-blocking rule:** the start-of-topic lesson is shown once per new KC and
  never blocks review cards for already-encoded KCs, to avoid interrupting normal
  study flow.

### (b) "Lesson" button after the answer is revealed

Purpose: from any card, after seeing the answer, jump to that card's KC lesson.

- **Where it appears:** the button should be visible on the answer side. Two
  options:
  - Add a `Lesson` button in `_bottomHTML` next to the existing `Progress`
    button (`reviewer.py:1082`), enabled once the answer is shown. This makes it a
    persistent, discoverable control.
  - Or inject it into the answer-side controls built by `_answerButtons()` /
    shown by `_showEaseButtons()` (`reviewer.py:1122-1200`) so it only appears
    after reveal, next to the ease buttons.
- **Answer-reveal hook:** `_showAnswer` (`reviewer.py:718-744`) already refreshes
  concept UI after reveal (`_update_concept_badge(c)` /
  `_update_concept_graph_sidebar(c)` at `reviewer.py:738-739`); enabling/showing
  the Lesson control belongs in the same place.
- **Click flow:** button calls `pycmd("lesson")` -> `_linkHandler`
  (`reviewer.py:943`) -> `_open_lesson_for_current_card()` -> resolves KC via
  `_concept_labels(self.card)` -> renders `#_concept_lesson_panel` with the stored
  lesson. If the card has multiple `KC::` tags, offer the primary KC first.
- **Empty state:** if no lesson exists for the KC yet, show a short "No lesson
  for this concept yet" message rather than failing.

## Local-first and AI Gating

- Lessons are local-first: content ships in the repo (or per-profile authored
  files) and renders with no network call. Studying never requires connectivity.
- The `source` and `review_status` fields are the gate. Only
  `source: authored` + `review_status: approved` lessons render in the live app.
- AI-generated lesson text must NOT go live until the bring-your-own-OpenAI-key
  work and source/evaluation rules exist (Priority 2 / Track H in
  `next-feature-expansion-plan.md`). Until then, any `ai_generated` lesson is
  stored but hidden behind the gate, so no unreviewed AI text reaches a learner.
- When AI generation is later allowed, it must run on the user's own key, and
  generated lessons enter as `review_status: needs_review` (never auto-approved).

## Lesson Contract Inputs

What lesson pages need from the two upstream tracks.

### From Track A (KC map)

- **Canonical KC ids** so `kc`, `related_kcs`, and `prerequisite_kcs` reference
  real nodes and can be validated. (Today: the 10 demo KCs in
  `concept_demo.rs`; later: the expanded map in `mcat.md`.)
- **Prerequisite edges** to auto-fill `prerequisite_reminder.prerequisite_kcs`
  and part of `related_kcs`, so lessons never drift from the graph. Lessons
  consume edges; they do not define them.
- **Parent area / discipline + section overlap** to set `section`
  (`MCAT::…`) and to know which MCAT section(s) a lesson supports.
- **Difficulty ladder / foundation-vs-mechanism-vs-application classification**
  to tune worked-example depth (foundations get a concrete first example;
  application KCs get a reasoning-style example).

### From Track D (topic picker)

- **Selected KC id** so the reviewer knows which lesson to open at
  start-of-topic.
- **A "topic selected -> open lesson before cards" signal**, so the picker can
  request the start-of-topic lesson and hand control back to the queue only after
  the learner presses "Start retrieval".
- **A "new KC first-encounter" signal / per-profile encoded flag** so the
  reviewer shows the start-of-topic lesson once per new KC and not on every
  review. This flag is local-first and not synced.
- **The "why recommended" context** (prereqs ready, target not yet mastered,
  new-topic budget, next action) so the lesson's opening can optionally echo why
  this topic is being introduced now.
- **A return/callback path** so that dismissing the lesson resumes the normal
  queue for that KC (the reviewer already flows lesson -> `_showQuestion` ->
  `_showAnswerButton`).

## Acceptance Mapping (Track C)

- "Every demo KC has a lesson stub" - schema + validation require one lesson per
  KC; five representative stubs are authored here and the rest follow the same
  template.
- "New outer-fringe topic selection can open the lesson before quizzing" -
  entry point (a), triggered by Track D topic selection via `nextCard()` /
  `_showQuestion`.
- "Current flashcard can open its related lesson after answer reveal" - entry
  point (b), a `Lesson` control shown after `_showAnswer`, routed through
  `_linkHandler` and resolved with `_concept_labels`.
