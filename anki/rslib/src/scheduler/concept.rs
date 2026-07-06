// Copyright: Ankitects Pty Ltd and contributors
// License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html

#![allow(dead_code)]

use std::collections::BTreeMap;
use std::collections::BTreeSet;

use serde::Deserialize;
use serde::Serialize;

use crate::config::DeckConfigKey;
use crate::prelude::*;
use crate::scheduler::answering::Rating;

const KC_TAG_PREFIX: &str = "KC::";
const PREREQ_TAG_PREFIX: &str = "Prereq::";
const MCAT_TAG_PREFIX: &str = "MCAT::";
const DIFFICULTY_TAG_PREFIX: &str = "Difficulty::";
const IRT_DISCRIMINATION_TAG_PREFIX: &str = "IRT::Discrimination::";
const IRT_GUESSING_TAG_PREFIX: &str = "IRT::Guessing::";
const REASONING_TAG_PREFIX: &str = "Reasoning::";
const OVERVIEW_TAG_PREFIX: &str = "Overview::";
const CONCEPT_SCHEDULER_STATE_SCHEMA_VERSION: u32 = 2;
/// AAMC-anchored linear map from IRT ability (theta, ~N(0,1) across test-takers)
/// to the 118-132 section scale: `score = SCALE_B + SCALE_A * theta`.
/// `SCALE_B = 125` is the population-median section score (~50th percentile) and
/// `SCALE_A = 2.5` is the scale points per ability SD (the 118-132 band spans
/// roughly +/-3 SD around the median). See `research-scoring.md` for the
/// derivation from AAMC's percentile/scale references.
const SCALE_A: f64 = 2.5;
const SCALE_B: f64 = 125.0;
/// Section score bounds (also the clamp for the projected total's per-section part).
const SCORE_MIN: f64 = 118.0;
const SCORE_MAX: f64 = 132.0;
/// A mastered/unmastered group must collect MORE than this many observations before
/// its Bayesian conditional likelihood switches from the static default to the
/// empirical (data-driven) rate. Defaults stay static below this.
const LIKELIHOOD_MIN_OBSERVATIONS: u32 = 20;
/// Four-choice random-guessing floor for `P(correct | unmastered)`.
const GUESS_FLOOR: f64 = 0.25;
/// How much `P(correct | unmastered)` rises per unit of current mastery probability.
/// The default is `GUESS_FLOOR + PARTIAL_MASTERY_LIFT * mastery`: a partly-learned KC
/// is expected to score above pure guessing, and the two states become less
/// distinguishable as mastery climbs. Tune this to trade evidence-per-answer (bigger
/// lift ⇒ correct answers are weaker evidence ⇒ mastery grows more slowly).
const PARTIAL_MASTERY_LIFT: f64 = 0.5;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub(crate) struct KnowledgeComponentId(String);

impl KnowledgeComponentId {
    pub(crate) fn new(value: impl Into<String>) -> Option<Self> {
        let value = value.into();
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| Self(trimmed.to_string()))
    }

    pub(crate) fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub(crate) struct CardConceptMetadata {
    pub(crate) target_components: Vec<KnowledgeComponentId>,
    pub(crate) overview_components: Vec<KnowledgeComponentId>,
    pub(crate) prerequisites: Vec<KnowledgeComponentId>,
    pub(crate) sections: Vec<String>,
    pub(crate) difficulty: Option<u8>,
    pub(crate) discrimination: Option<f64>,
    pub(crate) guessing: Option<f64>,
    pub(crate) reasoning: Option<String>,
}

impl CardConceptMetadata {
    pub(crate) fn from_tags(tags: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        let mut metadata = Self::default();

        for tag in tags {
            let tag = tag.as_ref();
            if let Some(id) = tag.strip_prefix(KC_TAG_PREFIX) {
                push_unique_component(&mut metadata.target_components, id);
            } else if let Some(id) = tag.strip_prefix(OVERVIEW_TAG_PREFIX) {
                push_unique_component(&mut metadata.overview_components, id);
            } else if let Some(id) = tag.strip_prefix(PREREQ_TAG_PREFIX) {
                push_unique_component(&mut metadata.prerequisites, id);
            } else if let Some(section) = tag.strip_prefix(MCAT_TAG_PREFIX) {
                push_unique_string(&mut metadata.sections, section);
            } else if let Some(difficulty) = tag.strip_prefix(DIFFICULTY_TAG_PREFIX) {
                metadata.difficulty = difficulty
                    .parse::<u8>()
                    .ok()
                    .filter(|value| (1..=5).contains(value));
            } else if let Some(discrimination) = tag.strip_prefix(IRT_DISCRIMINATION_TAG_PREFIX) {
                metadata.discrimination = discrimination
                    .parse::<f64>()
                    .ok()
                    .filter(|value| *value > 0.0);
            } else if let Some(guessing) = tag.strip_prefix(IRT_GUESSING_TAG_PREFIX) {
                metadata.guessing = guessing.parse::<f64>().ok().map(clamp_probability);
            } else if let Some(reasoning) = tag.strip_prefix(REASONING_TAG_PREFIX) {
                let reasoning = reasoning.trim();
                if !reasoning.is_empty() {
                    metadata.reasoning = Some(reasoning.to_string());
                }
            }
        }

        metadata
    }

    pub(crate) fn is_overview(&self) -> bool {
        !self.overview_components.is_empty()
    }

    /// Whether this card is filed under the given MCAT section, per its `MCAT::`
    /// tag(s). Used by the "study by section" filter to restrict the new-card pool
    /// to one section. A card with no recognized section tag belongs to none.
    pub(crate) fn belongs_to_section(&self, section: McatSection) -> bool {
        self.sections
            .iter()
            .filter_map(|tag| McatSection::from_tag(tag))
            .any(|card_section| card_section == section)
    }
}

fn push_unique_component(components: &mut Vec<KnowledgeComponentId>, value: &str) {
    if let Some(component) = KnowledgeComponentId::new(value) {
        if !components.contains(&component) {
            components.push(component);
        }
    }
}

fn push_unique_string(values: &mut Vec<String>, value: &str) {
    let value = value.trim();
    if !value.is_empty() && !values.iter().any(|existing| existing == value) {
        values.push(value.to_string());
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct KnowledgeGraph {
    nodes: BTreeMap<KnowledgeComponentId, KnowledgeComponent>,
}

impl KnowledgeGraph {
    pub(crate) fn add_component(&mut self, component: KnowledgeComponentId) {
        self.nodes
            .entry(component.clone())
            .or_insert_with(|| KnowledgeComponent {
                id: component,
                prerequisites: BTreeSet::new(),
            });
    }

    pub(crate) fn add_prerequisite(
        &mut self,
        target: KnowledgeComponentId,
        prerequisite: KnowledgeComponentId,
    ) {
        self.add_component(prerequisite.clone());
        self.nodes
            .entry(target.clone())
            .or_insert_with(|| KnowledgeComponent {
                id: target,
                prerequisites: BTreeSet::new(),
            })
            .prerequisites
            .insert(prerequisite);
    }

    pub(crate) fn from_card_metadata<'a>(
        cards: impl IntoIterator<Item = &'a CardConceptMetadata>,
    ) -> Self {
        let mut graph = Self::default();

        for card in cards {
            for target in &card.target_components {
                graph.add_component(target.clone());
                for prerequisite in &card.prerequisites {
                    graph.add_prerequisite(target.clone(), prerequisite.clone());
                }
            }
        }

        graph
    }

    pub(crate) fn contains(&self, component: &KnowledgeComponentId) -> bool {
        self.nodes.contains_key(component)
    }

    pub(crate) fn prerequisites(
        &self,
        component: &KnowledgeComponentId,
    ) -> Option<&BTreeSet<KnowledgeComponentId>> {
        self.nodes.get(component).map(|node| &node.prerequisites)
    }

    pub(crate) fn components(&self) -> impl Iterator<Item = &KnowledgeComponentId> {
        self.nodes.keys()
    }

    pub(crate) fn cycle(&self) -> Option<Vec<KnowledgeComponentId>> {
        let mut marks = BTreeMap::new();
        let mut path = Vec::new();

        for component in self.components() {
            if self.find_cycle(component, &mut marks, &mut path).is_some() {
                let cycle_start = path
                    .iter()
                    .position(|item| item == component)
                    .unwrap_or_default();
                return Some(path[cycle_start..].to_vec());
            }
        }

        None
    }

    fn find_cycle(
        &self,
        component: &KnowledgeComponentId,
        marks: &mut BTreeMap<KnowledgeComponentId, VisitMark>,
        path: &mut Vec<KnowledgeComponentId>,
    ) -> Option<()> {
        match marks.get(component) {
            Some(VisitMark::Temporary) => {
                path.push(component.clone());
                return Some(());
            }
            Some(VisitMark::Permanent) => return None,
            None => (),
        }

        marks.insert(component.clone(), VisitMark::Temporary);
        path.push(component.clone());

        if let Some(prerequisites) = self.prerequisites(component) {
            for prerequisite in prerequisites {
                if self.find_cycle(prerequisite, marks, path).is_some() {
                    return Some(());
                }
            }
        }

        path.pop();
        marks.insert(component.clone(), VisitMark::Permanent);
        None
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct KnowledgeComponent {
    pub(crate) id: KnowledgeComponentId,
    pub(crate) prerequisites: BTreeSet<KnowledgeComponentId>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum VisitMark {
    Temporary,
    Permanent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub(crate) enum McatSection {
    BioBiochem,
    ChemPhys,
    PsychSoc,
    Cars,
}

impl McatSection {
    pub(crate) fn from_tag(section: &str) -> Option<Self> {
        match section {
            "Bio_Biochem" | "BioBiochem" => Some(Self::BioBiochem),
            "Chem_Phys" | "ChemPhys" => Some(Self::ChemPhys),
            "Psych_Soc" | "PsychSoc" => Some(Self::PsychSoc),
            "CARS" | "Cars" => Some(Self::Cars),
            _ => None,
        }
    }

    /// The canonical `MCAT::` tag suffix that authored cards carry for this
    /// section. This is the *primary* section a card is filed under (a biochem
    /// card, for instance, is tagged either `Bio_Biochem` or `Chem_Phys`), so it
    /// is the authoritative "study this section" signal — more precise than the
    /// KC discipline, which can span two sections.
    pub(crate) fn tag_suffix(self) -> &'static str {
        match self {
            Self::BioBiochem => "Bio_Biochem",
            Self::ChemPhys => "Chem_Phys",
            Self::PsychSoc => "Psych_Soc",
            Self::Cars => "CARS",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub(crate) enum McatDiscipline {
    Biology,
    Biochemistry,
    GeneralChemistry,
    OrganicChemistry,
    Physics,
    Psychology,
    Sociology,
    Cars,
}

/// Psych/Soc KCs whose MCAT sub-domain is sociology (per `kc-map-unified.md` §6, the
/// Sub-domain column). Everything else under `PsychSoc::` is psychology (the two
/// `Psy/Soc` dual-coded KCs — Prejudice_and_Bias, Group_Behavior — stay in their
/// Social-Psychology home). Without this split every `PsychSoc::` KC mapped to
/// Psychology, so the 30% sociology slice of the Psych/Soc blueprint could never fill
/// and section coverage/readiness were structurally capped at ~70%.
const SOCIOLOGY_PSYCHSOC_KCS: &[&str] = &[
    "PsychSoc::Social_Theory",
    "PsychSoc::Culture",
    "PsychSoc::Socialization",
    "PsychSoc::Social_Institutions",
    "PsychSoc::Demographics",
    "PsychSoc::Stratification",
    "PsychSoc::Social_Class",
    "PsychSoc::Social_Mobility",
    "PsychSoc::Poverty",
    "PsychSoc::Social_Inequality",
    "PsychSoc::Health_Disparities",
    "PsychSoc::Healthcare_Disparities",
];

impl McatDiscipline {
    fn for_component(component: &KnowledgeComponentId) -> Option<Self> {
        let id = component.as_str();
        if id.starts_with("Bio::") {
            Some(Self::Biology)
        } else if id.starts_with("Biochem::") {
            Some(Self::Biochemistry)
        } else if id.starts_with("GenChem::") {
            Some(Self::GeneralChemistry)
        } else if id.starts_with("Orgo::") {
            Some(Self::OrganicChemistry)
        } else if id.starts_with("Physics::") {
            Some(Self::Physics)
        } else if id.starts_with("PsychSoc::") {
            if SOCIOLOGY_PSYCHSOC_KCS.contains(&id) {
                Some(Self::Sociology)
            } else {
                Some(Self::Psychology)
            }
        } else if id.starts_with("CARS::") {
            Some(Self::Cars)
        } else {
            None
        }
    }
}

pub(crate) fn discipline_weight(section: McatSection, discipline: McatDiscipline) -> f64 {
    match section {
        McatSection::BioBiochem => match discipline {
            McatDiscipline::Biology => 0.65,
            McatDiscipline::Biochemistry => 0.25,
            McatDiscipline::GeneralChemistry => 0.05,
            McatDiscipline::OrganicChemistry => 0.05,
            _ => 0.0,
        },
        McatSection::ChemPhys => match discipline {
            McatDiscipline::GeneralChemistry => 0.30,
            McatDiscipline::Biochemistry => 0.25,
            McatDiscipline::Physics => 0.25,
            McatDiscipline::OrganicChemistry => 0.15,
            McatDiscipline::Biology => 0.05,
            _ => 0.0,
        },
        McatSection::PsychSoc => match discipline {
            McatDiscipline::Psychology => 0.65,
            McatDiscipline::Sociology => 0.30,
            McatDiscipline::Biology => 0.05,
            _ => 0.0,
        },
        McatSection::Cars => match discipline {
            McatDiscipline::Cars => 1.0,
            _ => 0.0,
        },
    }
}

const BIO_BIOCHEM_DISCIPLINES: &[(McatDiscipline, f64)] = &[
    (McatDiscipline::Biology, 0.65),
    (McatDiscipline::Biochemistry, 0.25),
    (McatDiscipline::GeneralChemistry, 0.05),
    (McatDiscipline::OrganicChemistry, 0.05),
];
const CHEM_PHYS_DISCIPLINES: &[(McatDiscipline, f64)] = &[
    (McatDiscipline::GeneralChemistry, 0.30),
    (McatDiscipline::Biochemistry, 0.25),
    (McatDiscipline::Physics, 0.25),
    (McatDiscipline::OrganicChemistry, 0.15),
    (McatDiscipline::Biology, 0.05),
];
const PSYCH_SOC_DISCIPLINES: &[(McatDiscipline, f64)] = &[
    (McatDiscipline::Psychology, 0.65),
    (McatDiscipline::Sociology, 0.30),
    (McatDiscipline::Biology, 0.05),
];
const CARS_DISCIPLINES: &[(McatDiscipline, f64)] = &[(McatDiscipline::Cars, 1.0)];

fn section_disciplines(section: McatSection) -> &'static [(McatDiscipline, f64)] {
    match section {
        McatSection::BioBiochem => BIO_BIOCHEM_DISCIPLINES,
        McatSection::ChemPhys => CHEM_PHYS_DISCIPLINES,
        McatSection::PsychSoc => PSYCH_SOC_DISCIPLINES,
        McatSection::Cars => CARS_DISCIPLINES,
    }
}

fn required_items_for_weight(weight: f64) -> u32 {
    ((weight / 0.05).round() as u32).max(1)
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub(crate) struct ConceptSchedulerConfig {
    pub(crate) initial_mastery: f64,
    pub(crate) inner_fringe_mastery: f64,
    pub(crate) inner_fringe_min_answers: u32,
    pub(crate) outer_fringe_prereq_mastery: f64,
    pub(crate) positive_likelihood_if_mastered: f64,
    pub(crate) positive_likelihood_if_unmastered: f64,
    pub(crate) negative_likelihood_if_mastered: f64,
    pub(crate) negative_likelihood_if_unmastered: f64,
    pub(crate) fallback_readiness_score: f64,
    pub(crate) readiness_min_seen_cards: u32,
    #[serde(default = "default_irt_min_section_items")]
    pub(crate) irt_min_section_items: u32,
    #[serde(default = "default_irt_min_section_coverage")]
    pub(crate) irt_min_section_coverage: f64,
    /// Scaled score the untested (uncovered) fraction of a section is assumed to
    /// contribute to readiness: a four-choice random-guessing baseline.
    #[serde(default = "default_guessing_baseline_score")]
    pub(crate) guessing_baseline_score: f64,
    #[serde(default = "default_max_coverage_standard_error")]
    pub(crate) max_coverage_standard_error: f64,
    #[serde(default = "default_max_mastery_standard_error")]
    pub(crate) max_mastery_standard_error: f64,
}

impl Default for ConceptSchedulerConfig {
    fn default() -> Self {
        Self {
            initial_mastery: 0.20,
            inner_fringe_mastery: 0.85,
            inner_fringe_min_answers: 3,
            outer_fringe_prereq_mastery: 0.70,
            positive_likelihood_if_mastered: 0.90,
            positive_likelihood_if_unmastered: 0.20,
            negative_likelihood_if_mastered: 0.10,
            negative_likelihood_if_unmastered: 0.80,
            fallback_readiness_score: 0.5,
            readiness_min_seen_cards: 500,
            irt_min_section_items: default_irt_min_section_items(),
            irt_min_section_coverage: default_irt_min_section_coverage(),
            guessing_baseline_score: default_guessing_baseline_score(),
            max_coverage_standard_error: default_max_coverage_standard_error(),
            max_mastery_standard_error: default_max_mastery_standard_error(),
        }
    }
}

fn default_irt_min_section_items() -> u32 {
    20
}

fn default_irt_min_section_coverage() -> f64 {
    0.60
}

fn default_guessing_baseline_score() -> f64 {
    120.0
}

fn default_max_coverage_standard_error() -> f64 {
    2.0
}

fn default_max_mastery_standard_error() -> f64 {
    2.0
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct PersistedConceptSchedulerState {
    pub(crate) schema_version: u32,
    pub(crate) config: ConceptSchedulerConfig,
    pub(crate) state: ConceptSchedulerState,
}

impl Default for PersistedConceptSchedulerState {
    fn default() -> Self {
        Self {
            schema_version: CONCEPT_SCHEDULER_STATE_SCHEMA_VERSION,
            config: ConceptSchedulerConfig::default(),
            state: ConceptSchedulerState::default(),
        }
    }
}

impl PersistedConceptSchedulerState {
    /// `config` holds only global tuning constants (there is no per-deck config UI),
    /// so on a schema bump we refresh them to the current defaults while preserving
    /// the learner's `state` (mastery/evidence/IRT). This lets constant changes — e.g.
    /// `initial_mastery` 0.25 -> 0.20 — take effect on decks that already persisted an
    /// older config, instead of silently keeping the stale value.
    fn migrate_to_current_schema(&mut self) {
        if self.schema_version < CONCEPT_SCHEDULER_STATE_SCHEMA_VERSION {
            self.config = ConceptSchedulerConfig::default();
            self.schema_version = CONCEPT_SCHEDULER_STATE_SCHEMA_VERSION;
        }
    }
}

impl Collection {
    pub(crate) fn get_concept_scheduler_state(
        &self,
        deck_id: DeckId,
    ) -> PersistedConceptSchedulerState {
        let key = DeckConfigKey::ConceptSchedulerState.for_deck(deck_id);
        let mut persisted: PersistedConceptSchedulerState =
            self.get_config_optional(key.as_str()).unwrap_or_default();
        persisted.migrate_to_current_schema();
        persisted
    }

    pub(crate) fn set_concept_scheduler_state(
        &mut self,
        deck_id: DeckId,
        state: &PersistedConceptSchedulerState,
        undoable: bool,
    ) -> Result<OpOutput<()>> {
        let key = DeckConfigKey::ConceptSchedulerState.for_deck(deck_id);
        self.set_config_json(key.as_str(), state, undoable)
    }

    pub(crate) fn set_concept_scheduler_state_inner(
        &mut self,
        deck_id: DeckId,
        state: &PersistedConceptSchedulerState,
    ) -> Result<bool> {
        let key = DeckConfigKey::ConceptSchedulerState.for_deck(deck_id);
        self.set_config(key.as_str(), state)
    }

    pub(crate) fn maybe_update_concept_mastery_from_answer(
        &mut self,
        home_deck: &Deck,
        note_id: NoteId,
        rating: Rating,
        mcat_answer_correct: Option<bool>,
        day: u32,
    ) -> Result<()> {
        let enabled = home_deck
            .normal()
            .map(|deck| deck.concept_scheduler_enabled)
            .unwrap_or(false);
        if !enabled {
            return Ok(());
        }

        let note = self.storage.get_note(note_id)?.or_not_found(note_id)?;
        let metadata = CardConceptMetadata::from_tags(&note.tags);
        if metadata.target_components.is_empty() {
            return Ok(());
        }
        if metadata.is_overview() {
            return Ok(());
        }

        let mut persisted = self.get_concept_scheduler_state(home_deck.id);
        let config = persisted.config;
        // A WRONG interactive-MC answer can never grow the score, even if the learner
        // self-rates Good/Easy (see `concept_evidence`): `None` here means "neutral".
        let evidence = concept_evidence(mcat_answer_correct, rating);
        // Prerequisite violations depend only on which card was served, not on whether
        // it was answered correctly, so record them regardless of the evidence.
        if metadata.prerequisites.iter().any(|prerequisite| {
            persisted.state.mastery_for(prerequisite, &config) < config.outer_fringe_prereq_mastery
        }) {
            persisted.state.record_prerequisite_violation(day);
        }
        for component in &metadata.target_components {
            match evidence {
                Some(evidence) => persisted
                    .state
                    .record_evidence_on_day(component.clone(), evidence, day, &config),
                // Neutral: still count the attempt (so the live counts stay equal to
                // the revlog reconstruction and a status refresh can't override this),
                // but leave mastery unchanged so the score cannot grow.
                None => persisted.state.note_component_seen(component.clone(), &config),
            }
        }
        persisted.state.note_card_answered();
        let irt_item = IrtItemMetadata::from_card_metadata(&metadata);
        for section in metadata
            .sections
            .iter()
            .filter_map(|section| McatSection::from_tag(section))
        {
            let disciplines = section_disciplines_for_metadata(&metadata, section);
            match evidence {
                Some(evidence) => {
                    persisted
                        .state
                        .record_irt_response(section, disciplines, irt_item, evidence)
                }
                // Neutral: count toward coverage/answered without moving theta.
                None => persisted.state.note_irt_item_seen(section, disciplines),
            }
        }

        self.set_concept_scheduler_state_inner(home_deck.id, &persisted)?;

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Evidence {
    Positive,
    Negative,
}

pub(crate) fn evidence_from_rating(rating: Rating) -> Evidence {
    match rating {
        Rating::Again | Rating::Hard => Evidence::Negative,
        Rating::Good | Rating::Easy => Evidence::Positive,
    }
}

/// The concept-scheduler evidence for an answer, given the true MC correctness when
/// the reviewer knows it (interactive multiple-choice cards).
///
/// A WRONG answer can never produce positive evidence — so a learner cannot grow
/// their score by tapping "Good"/"Easy" after guessing wrong:
/// - wrong + Again/Hard -> `Some(Negative)` (penalize),
/// - wrong + Good/Easy  -> `None` (neutral: unpenalized, but no growth).
///
/// A CORRECT answer, or any card where correctness is unknown (`None`, i.e. normal
/// non-MC reviews), keeps the rating-based signal (`evidence_from_rating`).
pub(crate) fn concept_evidence(
    mcat_answer_correct: Option<bool>,
    rating: Rating,
) -> Option<Evidence> {
    match mcat_answer_correct {
        Some(false) => match rating {
            Rating::Again | Rating::Hard => Some(Evidence::Negative),
            Rating::Good | Rating::Easy => None,
        },
        _ => Some(evidence_from_rating(rating)),
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct ConceptMasteryState {
    pub(crate) mastery: f64,
    pub(crate) answered: u32,
    pub(crate) positive: u32,
    pub(crate) negative: u32,
}

impl ConceptMasteryState {
    pub(crate) fn new(initial_mastery: f64) -> Self {
        Self {
            mastery: clamp_probability(initial_mastery),
            answered: 0,
            positive: 0,
            negative: 0,
        }
    }

    /// Apply one piece of evidence with the given conditional likelihoods
    /// (`P(evidence | mastered)`, `P(evidence | unmastered)`) and Bayes-update mastery.
    /// The likelihoods are chosen by the caller so they can be static defaults or the
    /// empirical data-driven rates.
    pub(crate) fn apply_evidence(
        &mut self,
        evidence: Evidence,
        likelihood_if_mastered: f64,
        likelihood_if_unmastered: f64,
    ) {
        match evidence {
            Evidence::Positive => self.positive += 1,
            Evidence::Negative => self.negative += 1,
        }
        self.answered += 1;
        self.mastery = bayes_update(self.mastery, likelihood_if_mastered, likelihood_if_unmastered);
    }

    /// Count one attempt without changing mastery (a neutral wrong + Good/Easy MC
    /// answer). Bumps `answered` so the live count stays equal to the revlog
    /// reconstruction; otherwise a status refresh could replace this neutral state
    /// with a rating-derived (positive) one and re-open the exploit.
    pub(crate) fn note_seen(&mut self) {
        self.answered += 1;
    }

    pub(crate) fn is_inner_fringe(&self, config: &ConceptSchedulerConfig) -> bool {
        self.mastery >= config.inner_fringe_mastery
            && self.answered >= config.inner_fringe_min_answers
    }
}

impl Default for ConceptMasteryState {
    fn default() -> Self {
        Self::new(ConceptSchedulerConfig::default().initial_mastery)
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub(crate) struct ConceptSchedulerState {
    pub(crate) kcs: BTreeMap<KnowledgeComponentId, ConceptMasteryState>,
    pub(crate) total_seen_cards: u32,
    #[serde(default)]
    pub(crate) daily: BTreeMap<u32, DailyConceptMetrics>,
    #[serde(default)]
    pub(crate) prerequisite_violations: u32,
    #[serde(default)]
    pub(crate) irt_sections: BTreeMap<McatSection, IrtSectionState>,
    /// The user-chosen next topic (an outer-fringe KC). Survives queue rebuilds;
    /// the live `ConceptSessionState.selected_topic` is seeded from this.
    #[serde(default)]
    pub(crate) selected_topic: Option<KnowledgeComponentId>,
    /// The user-chosen MCAT section to study. When set, the new-card pool is
    /// restricted to cards filed under this section (see the queue builder's
    /// section filter); `None` leaves scheduling across all sections unchanged.
    #[serde(default)]
    pub(crate) selected_section: Option<McatSection>,
    /// Exam date as epoch seconds. Drives the retention projection: recall is
    /// evaluated at this date, so readiness reflects what will survive to test day.
    #[serde(default)]
    pub(crate) exam_timestamp: Option<i64>,
    /// Desired MCAT total score (472-528) for the probability-of-hitting-target.
    #[serde(default)]
    pub(crate) target_total_score: Option<u32>,
    /// Correct/wrong tallies split by whether the KC was mastered at answer time.
    /// Once a group exceeds `LIKELIHOOD_MIN_OBSERVATIONS`, its Bayesian conditional
    /// likelihood becomes the empirical rate instead of the static default.
    #[serde(default)]
    pub(crate) likelihood_observations: LikelihoodObservations,
}

/// Per-outcome answer tallies, split by whether the model believed the KC was
/// mastered at answer time. Drives data-driven conditional likelihoods: with enough
/// evidence, `P(correct | mastered)` and `P(correct | unmastered)` come from the
/// learner's own history rather than fixed constants.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct LikelihoodObservations {
    pub(crate) mastered_positive: u32,
    pub(crate) mastered_negative: u32,
    pub(crate) unmastered_positive: u32,
    pub(crate) unmastered_negative: u32,
}

impl LikelihoodObservations {
    fn record(&mut self, was_mastered: bool, evidence: Evidence) {
        match (was_mastered, evidence) {
            (true, Evidence::Positive) => self.mastered_positive += 1,
            (true, Evidence::Negative) => self.mastered_negative += 1,
            (false, Evidence::Positive) => self.unmastered_positive += 1,
            (false, Evidence::Negative) => self.unmastered_negative += 1,
        }
    }

    /// `P(evidence | group)`. Empirical (Laplace-smoothed) once the group has MORE
    /// than `LIKELIHOOD_MIN_OBSERVATIONS` observations, otherwise `default`.
    fn likelihood(&self, mastered: bool, evidence: Evidence, default: f64) -> f64 {
        let (positive, negative) = if mastered {
            (self.mastered_positive, self.mastered_negative)
        } else {
            (self.unmastered_positive, self.unmastered_negative)
        };
        let total = positive + negative;
        if total <= LIKELIHOOD_MIN_OBSERVATIONS {
            return default;
        }
        let count = match evidence {
            Evidence::Positive => positive,
            Evidence::Negative => negative,
        };
        // Laplace (add-one) smoothing keeps the estimate strictly inside (0, 1), so a
        // run of all-correct or all-wrong can't produce a degenerate 0/1 likelihood.
        (count as f64 + 1.0) / (total as f64 + 2.0)
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct DailyConceptMetrics {
    pub(crate) positive: u32,
    pub(crate) negative: u32,
    pub(crate) prerequisite_violations: u32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct IrtItemMetadata {
    pub(crate) difficulty: f64,
    pub(crate) discrimination: f64,
    pub(crate) guessing: f64,
}

impl IrtItemMetadata {
    pub(crate) fn from_card_metadata(metadata: &CardConceptMetadata) -> Self {
        Self {
            difficulty: difficulty_to_irt_b(metadata.difficulty.unwrap_or(3)),
            discrimination: metadata.discrimination.unwrap_or(1.0).max(0.1),
            guessing: metadata.guessing.unwrap_or(0.25).clamp(0.0, 0.95),
        }
    }

    pub(crate) fn probability_correct(self, theta: f64) -> f64 {
        let logistic = 1.0 / (1.0 + (-self.discrimination * (theta - self.difficulty)).exp());
        self.guessing + ((1.0 - self.guessing) * logistic)
    }

    fn derivative(self, theta: f64) -> f64 {
        let logistic = 1.0 / (1.0 + (-self.discrimination * (theta - self.difficulty)).exp());
        (1.0 - self.guessing) * self.discrimination * logistic * (1.0 - logistic)
    }
}

fn difficulty_to_irt_b(difficulty: u8) -> f64 {
    match difficulty {
        1 => -2.0,
        2 => -1.0,
        3 => 0.0,
        4 => 1.0,
        5 => 2.0,
        _ => 0.0,
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct IrtSectionState {
    pub(crate) theta: f64,
    pub(crate) answered: u32,
    pub(crate) correct: u32,
    pub(crate) information: f64,
    #[serde(default)]
    pub(crate) discipline_counts: BTreeMap<McatDiscipline, u32>,
}

impl Default for IrtSectionState {
    fn default() -> Self {
        Self {
            theta: 0.0,
            answered: 0,
            correct: 0,
            information: 0.0,
            discipline_counts: BTreeMap::new(),
        }
    }
}

impl IrtSectionState {
    pub(crate) fn record_response(
        &mut self,
        item: IrtItemMetadata,
        correct: bool,
        disciplines: impl IntoIterator<Item = McatDiscipline>,
    ) {
        let probability = item.probability_correct(self.theta).clamp(0.001, 0.999);
        let derivative = item.derivative(self.theta);
        let variance = (probability * (1.0 - probability)).max(0.001);
        let fisher_information = (derivative * derivative) / variance;
        let observed = if correct { 1.0 } else { 0.0 };
        let gradient = ((observed - probability) * derivative) / variance;
        let step = gradient / (fisher_information + 1.0);

        self.theta = (self.theta + step).clamp(-4.0, 4.0);
        self.answered += 1;
        if correct {
            self.correct += 1;
        }
        self.information += fisher_information.max(0.01);
        for discipline in BTreeSet::from_iter(disciplines) {
            *self.discipline_counts.entry(discipline).or_default() += 1;
        }
    }

    /// Count one attempt toward coverage/`answered` without moving theta or adding
    /// information (a neutral wrong + Good/Easy MC answer). Keeps the section's
    /// `answered` and `discipline_counts` equal to the revlog reconstruction so a
    /// status refresh cannot replace this neutral state with a theta-raising one.
    pub(crate) fn note_item_seen(
        &mut self,
        disciplines: impl IntoIterator<Item = McatDiscipline>,
    ) {
        self.answered += 1;
        for discipline in BTreeSet::from_iter(disciplines) {
            *self.discipline_counts.entry(discipline).or_default() += 1;
        }
    }

    pub(crate) fn performance_standard_error(&self) -> f64 {
        if self.information <= 0.0 {
            4.0
        } else {
            (1.0 / self.information.sqrt()) * 2.5
        }
    }

    pub(crate) fn scaled_score(&self) -> f64 {
        theta_to_scaled_score(self.theta)
    }
}

fn theta_to_scaled_score(theta: f64) -> f64 {
    (SCALE_B + (theta * SCALE_A)).clamp(SCORE_MIN, SCORE_MAX)
}

/// Standard normal CDF (Abramowitz & Stegun 7.1.26 erf approximation). Used to
/// turn a projected-score band into P(score >= target).
fn normal_cdf(z: f64) -> f64 {
    // erf(x) approximation, max abs error ~1.5e-7.
    let sign = if z < 0.0 { -1.0 } else { 1.0 };
    let x = (z / std::f64::consts::SQRT_2).abs();
    let t = 1.0 / (1.0 + 0.3275911 * x);
    let y = 1.0
        - (((((1.061405429 * t - 1.453152027) * t) + 1.421413741) * t - 0.284496736) * t
            + 0.254829592)
            * t
            * (-x * x).exp();
    0.5 * (1.0 + sign * y)
}

/// Probability that a normal(center, se) projected score reaches at least
/// `target`. With no spread it degenerates to a hard threshold.
pub(crate) fn probability_at_least(center: f64, se: f64, target: f64) -> f64 {
    if se <= 0.0 {
        return if center >= target { 1.0 } else { 0.0 };
    }
    (1.0 - normal_cdf((target - center) / se)).clamp(0.0, 1.0)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct SectionScoreStatus {
    pub(crate) section: McatSection,
    pub(crate) enough_evidence: bool,
    pub(crate) theta: f64,
    pub(crate) performance_center: f64,
    pub(crate) performance_standard_error: f64,
    pub(crate) performance_lower: f64,
    pub(crate) performance_upper: f64,
    pub(crate) section_mastery: f64,
    pub(crate) coverage: f64,
    pub(crate) readiness_center: f64,
    pub(crate) readiness_standard_error: f64,
    pub(crate) readiness_lower: f64,
    pub(crate) readiness_upper: f64,
    pub(crate) answered_items: u32,
    pub(crate) required_items: u32,
}

impl ConceptSchedulerState {
    pub(crate) fn mastery_for(
        &self,
        component: &KnowledgeComponentId,
        config: &ConceptSchedulerConfig,
    ) -> f64 {
        self.kcs
            .get(component)
            .map(|state| state.mastery)
            .unwrap_or(config.initial_mastery)
    }

    pub(crate) fn ensure_component(
        &mut self,
        component: KnowledgeComponentId,
        config: &ConceptSchedulerConfig,
    ) -> &mut ConceptMasteryState {
        self.kcs
            .entry(component)
            .or_insert_with(|| ConceptMasteryState::new(config.initial_mastery))
    }

    pub(crate) fn record_evidence(
        &mut self,
        component: KnowledgeComponentId,
        evidence: Evidence,
        config: &ConceptSchedulerConfig,
    ) {
        // Classify the KC's latent state BEFORE the update, for likelihood data
        // collection: "mastered" = the model already believes it's at/above the
        // mastered bar. This is what splits observations into the two groups.
        let was_mastered = self
            .kcs
            .get(&component)
            .map(|state| state.mastery)
            .unwrap_or(config.initial_mastery)
            >= config.inner_fringe_mastery;
        let current_mastery = self
            .kcs
            .get(&component)
            .map(|state| state.mastery)
            .unwrap_or(config.initial_mastery);
        let (likelihood_if_mastered, likelihood_if_unmastered) =
            self.effective_likelihoods(evidence, current_mastery, config);
        self.ensure_component(component, config).apply_evidence(
            evidence,
            likelihood_if_mastered,
            likelihood_if_unmastered,
        );
        self.likelihood_observations.record(was_mastered, evidence);
    }

    /// Conditional likelihoods for a Bayes update. A group uses its empirical
    /// (Laplace-smoothed) rate once it has collected more than
    /// `LIKELIHOOD_MIN_OBSERVATIONS` data points; until then it uses the default. The
    /// two groups (mastered/unmastered) cross over independently. The unmastered
    /// "correct" default is not flat: it is `GUESS_FLOOR + PARTIAL_MASTERY_LIFT *
    /// mastery`, so a partly-learned KC is expected to score above pure guessing.
    fn effective_likelihoods(
        &self,
        evidence: Evidence,
        mastery: f64,
        config: &ConceptSchedulerConfig,
    ) -> (f64, f64) {
        let (default_mastered, default_unmastered) = match evidence {
            Evidence::Positive => (
                config.positive_likelihood_if_mastered,
                (GUESS_FLOOR + PARTIAL_MASTERY_LIFT * mastery)
                    .clamp(0.0, config.positive_likelihood_if_mastered),
            ),
            Evidence::Negative => (
                config.negative_likelihood_if_mastered,
                config.negative_likelihood_if_unmastered,
            ),
        };
        (
            self.likelihood_observations
                .likelihood(true, evidence, default_mastered),
            self.likelihood_observations
                .likelihood(false, evidence, default_unmastered),
        )
    }

    /// Count one answered card toward the evidence total that gates readiness. Call
    /// once per card answer (NOT once per target KC), so a multi-KC card can't inflate
    /// the count and unlock readiness sorting early.
    pub(crate) fn note_card_answered(&mut self) {
        self.total_seen_cards += 1;
    }

    pub(crate) fn record_evidence_on_day(
        &mut self,
        component: KnowledgeComponentId,
        evidence: Evidence,
        day: u32,
        config: &ConceptSchedulerConfig,
    ) {
        self.record_evidence(component, evidence, config);
        let daily = self.daily.entry(day).or_default();
        match evidence {
            Evidence::Positive => daily.positive += 1,
            Evidence::Negative => daily.negative += 1,
        }
    }

    pub(crate) fn record_irt_response(
        &mut self,
        section: McatSection,
        disciplines: impl IntoIterator<Item = McatDiscipline>,
        item: IrtItemMetadata,
        evidence: Evidence,
    ) {
        self.irt_sections
            .entry(section)
            .or_default()
            .record_response(item, evidence == Evidence::Positive, disciplines);
    }

    /// Count a neutral MC attempt for a KC (a wrong answer rated Good/Easy): create or
    /// keep the entry and bump `answered` without changing mastery, so the score
    /// cannot grow and the revlog reconstruction cannot override it.
    pub(crate) fn note_component_seen(
        &mut self,
        component: KnowledgeComponentId,
        config: &ConceptSchedulerConfig,
    ) {
        self.kcs
            .entry(component)
            .or_insert_with(|| ConceptMasteryState::new(config.initial_mastery))
            .note_seen();
    }

    /// Count a neutral MC attempt toward a section's IRT coverage/`answered` without
    /// moving theta.
    pub(crate) fn note_irt_item_seen(
        &mut self,
        section: McatSection,
        disciplines: impl IntoIterator<Item = McatDiscipline>,
    ) {
        self.irt_sections
            .entry(section)
            .or_default()
            .note_item_seen(disciplines);
    }

    pub(crate) fn record_prerequisite_violation(&mut self, day: u32) {
        self.prerequisite_violations += 1;
        self.daily.entry(day).or_default().prerequisite_violations += 1;
    }

    pub(crate) fn is_inner_fringe(
        &self,
        component: &KnowledgeComponentId,
        config: &ConceptSchedulerConfig,
    ) -> bool {
        self.kcs
            .get(component)
            .map(|state| state.is_inner_fringe(config))
            .unwrap_or(false)
    }

    pub(crate) fn is_outer_fringe(
        &self,
        component: &KnowledgeComponentId,
        graph: &KnowledgeGraph,
        config: &ConceptSchedulerConfig,
    ) -> bool {
        if self.is_inner_fringe(component, config) {
            return false;
        }

        graph
            .prerequisites(component)
            .map(|prerequisites| {
                prerequisites.iter().all(|prerequisite| {
                    self.mastery_for(prerequisite, config) >= config.outer_fringe_prereq_mastery
                })
            })
            .unwrap_or(false)
    }

    pub(crate) fn outer_fringe(
        &self,
        graph: &KnowledgeGraph,
        config: &ConceptSchedulerConfig,
    ) -> Vec<KnowledgeComponentId> {
        graph
            .components()
            .filter(|component| self.is_outer_fringe(component, graph, config))
            .cloned()
            .collect()
    }

    pub(crate) fn readiness_score(
        &self,
        target: &KnowledgeComponentId,
        graph: &KnowledgeGraph,
        config: &ConceptSchedulerConfig,
    ) -> ReadinessScore {
        let Some(prerequisites) = graph.prerequisites(target) else {
            return ReadinessScore {
                score: config.fallback_readiness_score,
                prerequisite_mastery: config.fallback_readiness_score,
                target_mastery: self.mastery_for(target, config),
                reason: ReadinessScoreReason::UnknownTarget,
            };
        };

        let prerequisite_mastery = prerequisites
            .iter()
            .map(|prerequisite| self.mastery_for(prerequisite, config))
            .reduce(f64::min)
            .unwrap_or(1.0);
        let target_mastery = self.mastery_for(target, config);

        ReadinessScore {
            score: prerequisite_mastery * (1.0 - target_mastery),
            prerequisite_mastery,
            target_mastery,
            reason: ReadinessScoreReason::Scored,
        }
    }

    pub(crate) fn card_readiness_score(
        &self,
        metadata: &CardConceptMetadata,
        graph: &KnowledgeGraph,
        config: &ConceptSchedulerConfig,
    ) -> f64 {
        metadata
            .target_components
            .iter()
            .map(|target| self.readiness_score(target, graph, config).score)
            .reduce(f64::max)
            .unwrap_or(config.fallback_readiness_score)
    }

    pub(crate) fn readiness_evidence_status(
        &self,
        config: &ConceptSchedulerConfig,
    ) -> ReadinessEvidenceStatus {
        if self.total_seen_cards < config.readiness_min_seen_cards {
            ReadinessEvidenceStatus::InsufficientEvidence {
                seen_cards: self.total_seen_cards,
                required_seen_cards: config.readiness_min_seen_cards,
            }
        } else {
            ReadinessEvidenceStatus::EnoughEvidence {
                seen_cards: self.total_seen_cards,
            }
        }
    }

    pub(crate) fn section_score_status(
        &self,
        section: McatSection,
        graph: &KnowledgeGraph,
        config: &ConceptSchedulerConfig,
        // Blueprint-weighted recall of the section's studied KCs, projected to the
        // exam date (see `Collection::kc_memory_from_deck_cards`). `None` when
        // nothing in the section has memory yet, so retention is not penalized.
        section_memory: Option<f64>,
    ) -> SectionScoreStatus {
        let irt = self.irt_sections.get(&section).cloned().unwrap_or_default();
        let section_mastery = self.section_mastery(section, graph, config);
        let coverage = self.section_coverage(section, graph).clamp(0.0, 1.0);
        // Performance: the AAMC-scaled score implied by the section's IRT ability
        // (theta -> SCALE_B + SCALE_A*theta). This is the demonstrated *ceiling* the
        // learner would hit on covered material that is fully retained.
        let performance_center = irt.scaled_score();
        let performance_standard_error = irt.performance_standard_error();

        // Readiness combines the three signals a projected section score needs:
        //   performance (the ceiling above), memory (still recalled at the exam),
        //   and coverage (how much of the blueprint has been engaged).
        // Only the covered-AND-retained fraction earns the demonstrated ceiling; the
        // untested-or-forgotten remainder sits at the 4-choice guessing floor
        // (~120), NOT the population median (125) — you can't assume median
        // competence on material you haven't studied or have forgotten.
        let retention = section_memory.unwrap_or(1.0).clamp(0.0, 1.0);
        let floor = config.guessing_baseline_score;
        let retained_fraction = (coverage * retention).clamp(0.0, 1.0);
        let readiness_center = (retained_fraction * performance_center
            + (1.0 - retained_fraction) * floor)
            .clamp(SCORE_MIN, SCORE_MAX);

        // Uncertainty: the performance term only informs the covered fraction, so
        // it is scaled by coverage; thin coverage, weak mastery, and shaky retention
        // each widen the band. Variances are summed, then square-rooted.
        let performance_standard_error_component = coverage * performance_standard_error;
        let coverage_standard_error = (1.0 - coverage) * config.max_coverage_standard_error;
        let mastery_standard_error = (1.0 - section_mastery) * config.max_mastery_standard_error;
        let memory_standard_error = (1.0 - retention) * config.max_mastery_standard_error;
        let readiness_standard_error = ((performance_standard_error_component
            * performance_standard_error_component)
            + (coverage_standard_error * coverage_standard_error)
            + (mastery_standard_error * mastery_standard_error)
            + (memory_standard_error * memory_standard_error))
            .sqrt();
        let enough_evidence = irt.answered >= config.irt_min_section_items
            && coverage >= config.irt_min_section_coverage;

        SectionScoreStatus {
            section,
            enough_evidence,
            theta: irt.theta,
            performance_center,
            performance_standard_error,
            performance_lower: (performance_center - (1.96 * performance_standard_error))
                .clamp(SCORE_MIN, SCORE_MAX),
            performance_upper: (performance_center + (1.96 * performance_standard_error))
                .clamp(SCORE_MIN, SCORE_MAX),
            section_mastery,
            coverage,
            readiness_center,
            readiness_standard_error,
            readiness_lower: (readiness_center - (1.96 * readiness_standard_error))
                .clamp(SCORE_MIN, SCORE_MAX),
            readiness_upper: (readiness_center + (1.96 * readiness_standard_error))
                .clamp(SCORE_MIN, SCORE_MAX),
            answered_items: irt.answered,
            required_items: config.irt_min_section_items,
        }
    }

    fn section_mastery(
        &self,
        section: McatSection,
        graph: &KnowledgeGraph,
        config: &ConceptSchedulerConfig,
    ) -> f64 {
        let mut weighted_mastery = 0.0;
        let mut total_weight = 0.0;

        for (discipline, weight) in section_disciplines(section) {
            let weight = *weight;
            if weight <= 0.0 {
                continue;
            }

            total_weight += weight;
            weighted_mastery +=
                weight * self.average_mastery_for_discipline(*discipline, graph, config);
        }

        if total_weight == 0.0 {
            return config.initial_mastery;
        }

        weighted_mastery / total_weight
    }

    fn section_coverage(&self, section: McatSection, graph: &KnowledgeGraph) -> f64 {
        section_disciplines(section)
            .iter()
            .map(|(discipline, weight)| {
                let kc_total = self.kc_count_for_discipline(*discipline, graph);
                if kc_total == 0 {
                    // No KCs of this discipline exist in the deck's graph, so its slice
                    // of the blueprint cannot be covered.
                    return 0.0;
                }
                // A slice is "fully covered" once you've touched a target number of
                // DISTINCT KCs — capped at how many actually exist, so a discipline
                // with few KCs isn't held to an unreachable bar.
                let answered = self.answered_for_discipline(*discipline, graph);
                let required = required_items_for_weight(*weight).min(kc_total);
                weight * (answered as f64 / required as f64).min(1.0)
            })
            .sum()
    }

    /// Number of KCs of a discipline present in the deck's graph.
    fn kc_count_for_discipline(&self, discipline: McatDiscipline, graph: &KnowledgeGraph) -> u32 {
        graph
            .components()
            .filter(|component| McatDiscipline::for_component(component) == Some(discipline))
            .count() as u32
    }

    /// Number of DISTINCT KCs in a discipline that have any evidence (breadth), not
    /// total answer volume. Coverage should reward touching many concepts, not
    /// grinding one — 13 "Again"s on a single KC used to max its discipline's slice.
    fn answered_for_discipline(&self, discipline: McatDiscipline, graph: &KnowledgeGraph) -> u32 {
        graph
            .components()
            .filter(|component| McatDiscipline::for_component(component) == Some(discipline))
            .filter(|component| {
                self.kcs
                    .get(component)
                    .map(|state| state.answered > 0)
                    .unwrap_or(false)
            })
            .count() as u32
    }

    /// Pooled accuracy (correct / answered) over the section's studied KCs, or
    /// `None` when nothing in the section has been answered yet. Retained as a
    /// cross-check in tests; readiness now derives performance from IRT theta.
    #[cfg(test)]
    fn section_accuracy(&self, section: McatSection, graph: &KnowledgeGraph) -> Option<f64> {
        let mut positive = 0u32;
        let mut answered = 0u32;
        for (discipline, weight) in section_disciplines(section) {
            if *weight <= 0.0 {
                continue;
            }
            for component in graph.components() {
                if McatDiscipline::for_component(component) != Some(*discipline) {
                    continue;
                }
                if let Some(state) = self.kcs.get(component) {
                    positive += state.positive;
                    answered += state.positive + state.negative;
                }
            }
        }
        (answered > 0).then(|| positive as f64 / answered as f64)
    }

    fn average_mastery_for_discipline(
        &self,
        discipline: McatDiscipline,
        graph: &KnowledgeGraph,
        config: &ConceptSchedulerConfig,
    ) -> f64 {
        let mut mastery_sum = 0.0;
        let mut count = 0;

        for component in graph.components() {
            if McatDiscipline::for_component(component) == Some(discipline) {
                mastery_sum += self.mastery_for(component, config);
                count += 1;
            }
        }

        if count == 0 {
            config.initial_mastery
        } else {
            mastery_sum / count as f64
        }
    }
}

/// Blueprint-weighted section memory from per-KC memory values.
///
/// Mirrors the discipline weighting used for section mastery, but averages
/// `CardMemory`-derived per-KC recall instead of mastery. Returns `None` when
/// no KC in the section has any studied-card memory, so callers can gate the
/// display instead of showing a misleading 0%.
pub(crate) fn section_memory_from_kc_memory(
    section: McatSection,
    graph: &KnowledgeGraph,
    kc_memory: &BTreeMap<KnowledgeComponentId, f64>,
) -> Option<f64> {
    let mut weighted_memory = 0.0;
    let mut total_weight = 0.0;

    for (discipline, weight) in section_disciplines(section) {
        let weight = *weight;
        if weight <= 0.0 {
            continue;
        }

        let mut memory_sum = 0.0;
        let mut count = 0u32;
        for component in graph.components() {
            if McatDiscipline::for_component(component) == Some(*discipline) {
                if let Some(memory) = kc_memory.get(component) {
                    memory_sum += *memory;
                    count += 1;
                }
            }
        }

        if count > 0 {
            total_weight += weight;
            weighted_memory += weight * (memory_sum / count as f64);
        }
    }

    if total_weight == 0.0 {
        None
    } else {
        Some(weighted_memory / total_weight)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct ReadinessScore {
    pub(crate) score: f64,
    pub(crate) prerequisite_mastery: f64,
    pub(crate) target_mastery: f64,
    pub(crate) reason: ReadinessScoreReason,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ReadinessScoreReason {
    Scored,
    UnknownTarget,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ReadinessEvidenceStatus {
    InsufficientEvidence {
        seen_cards: u32,
        required_seen_cards: u32,
    },
    EnoughEvidence {
        seen_cards: u32,
    },
}

fn bayes_update(
    prior_mastery: f64,
    likelihood_if_mastered: f64,
    likelihood_if_unmastered: f64,
) -> f64 {
    let prior_mastery = clamp_probability(prior_mastery);
    let likelihood_if_mastered = clamp_probability(likelihood_if_mastered);
    let likelihood_if_unmastered = clamp_probability(likelihood_if_unmastered);
    let prior_unmastered = 1.0 - prior_mastery;
    let numerator = likelihood_if_mastered * prior_mastery;
    let denominator = numerator + (likelihood_if_unmastered * prior_unmastered);

    if denominator == 0.0 {
        prior_mastery
    } else {
        numerator / denominator
    }
}

fn clamp_probability(value: f64) -> f64 {
    value.clamp(0.0, 1.0)
}

pub(crate) fn section_disciplines_for_metadata(
    metadata: &CardConceptMetadata,
    section: McatSection,
) -> Vec<McatDiscipline> {
    let mut disciplines = BTreeSet::new();
    for component in &metadata.target_components {
        if let Some(discipline) = McatDiscipline::for_component(component) {
            if discipline_weight(section, discipline) > 0.0 {
                disciplines.insert(discipline);
            }
        }
    }
    disciplines.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn kc(id: &str) -> KnowledgeComponentId {
        KnowledgeComponentId::new(id).unwrap()
    }

    fn assert_approx_eq(actual: f64, expected: f64) {
        assert!(
            (actual - expected).abs() < 0.000_001,
            "expected {expected}, got {actual}"
        );
    }

    #[test]
    fn parses_concept_tags() {
        let metadata = CardConceptMetadata::from_tags([
            "KC::Bio::Genetics",
            "KC::Bio::Genetics",
            "Overview::Bio::Genetics",
            "Prereq::Bio::DNA",
            "MCAT::Bio_Biochem",
            "Difficulty::4",
            "IRT::Discrimination::1.2",
            "IRT::Guessing::0.25",
            "Reasoning::Conceptual",
            "Ignored::Tag",
        ]);

        assert_eq!(metadata.target_components, vec![kc("Bio::Genetics")]);
        assert_eq!(metadata.overview_components, vec![kc("Bio::Genetics")]);
        assert!(metadata.is_overview());
        assert_eq!(metadata.prerequisites, vec![kc("Bio::DNA")]);
        assert_eq!(metadata.sections, vec!["Bio_Biochem"]);
        assert_eq!(metadata.difficulty, Some(4));
        assert_eq!(metadata.discrimination, Some(1.2));
        assert_eq!(metadata.guessing, Some(0.25));
        assert_eq!(metadata.reasoning.as_deref(), Some("Conceptual"));
    }

    #[test]
    fn builds_graph_from_card_metadata() {
        let genetics = CardConceptMetadata::from_tags(["KC::Bio::Genetics", "Prereq::Bio::DNA"]);
        let enzymes = CardConceptMetadata::from_tags([
            "KC::Biochem::Enzymes",
            "Prereq::Biochem::Protein_Structure_and_Function",
        ]);
        let graph = KnowledgeGraph::from_card_metadata([&genetics, &enzymes]);

        assert!(graph.contains(&kc("Bio::Genetics")));
        assert_eq!(
            graph.prerequisites(&kc("Bio::Genetics")).unwrap(),
            &BTreeSet::from([kc("Bio::DNA")])
        );
        assert_eq!(
            graph.prerequisites(&kc("Biochem::Enzymes")).unwrap(),
            &BTreeSet::from([kc("Biochem::Protein_Structure_and_Function")])
        );
    }

    #[test]
    fn detects_prerequisite_cycles() {
        let mut graph = KnowledgeGraph::default();
        graph.add_prerequisite(kc("Bio::Genetics"), kc("Bio::DNA"));
        graph.add_prerequisite(kc("Bio::DNA"), kc("Bio::Genetics"));

        assert!(graph.cycle().is_some());
    }

    #[test]
    fn bayesian_evidence_updates_mastery() {
        let config = ConceptSchedulerConfig::default();
        let mut state = ConceptMasteryState::new(config.initial_mastery);

        state.apply_evidence(
            Evidence::Positive,
            config.positive_likelihood_if_mastered,
            config.positive_likelihood_if_unmastered,
        );
        assert_approx_eq(state.mastery, 0.529_411_764_705_882_4);
        assert_eq!(state.answered, 1);
        assert_eq!(state.positive, 1);

        state.apply_evidence(
            Evidence::Negative,
            config.negative_likelihood_if_mastered,
            config.negative_likelihood_if_unmastered,
        );
        assert_approx_eq(state.mastery, 0.123_287_671_232_876_7);
        assert_eq!(state.answered, 2);
        assert_eq!(state.negative, 1);
    }

    #[test]
    fn scheduler_state_records_component_evidence() {
        let config = ConceptSchedulerConfig::default();
        let mut state = ConceptSchedulerState::default();

        state.record_evidence(kc("Bio::DNA"), Evidence::Positive, &config);
        state.note_card_answered();

        let dna = state.kcs.get(&kc("Bio::DNA")).unwrap();
        // Fresh KC (mastery 0.20) => P(correct|unmastered) default = 0.25 + 0.5*0.20 =
        // 0.35, so posterior = 0.18 / (0.18 + 0.35*0.80) = 0.18/0.46.
        assert_approx_eq(dna.mastery, 0.391_304_347_826_087);
        assert_eq!(dna.answered, 1);
        assert_eq!(state.total_seen_cards, 1);
    }

    #[test]
    fn classifies_inner_and_outer_fringe() {
        let config = ConceptSchedulerConfig::default();
        let mut graph = KnowledgeGraph::default();
        graph.add_prerequisite(kc("Bio::Genetics"), kc("Bio::DNA"));
        graph.add_prerequisite(kc("Bio::Evolution"), kc("Bio::Genetics"));

        let mut state = ConceptSchedulerState::default();
        state.kcs.insert(
            kc("Bio::DNA"),
            ConceptMasteryState {
                mastery: 0.90,
                answered: 3,
                positive: 3,
                negative: 0,
            },
        );

        assert!(state.is_inner_fringe(&kc("Bio::DNA"), &config));
        assert!(state.is_outer_fringe(&kc("Bio::Genetics"), &graph, &config));
        assert!(!state.is_outer_fringe(&kc("Bio::Evolution"), &graph, &config));
        assert_eq!(
            state.outer_fringe(&graph, &config),
            vec![kc("Bio::Genetics")]
        );
    }

    #[test]
    fn readiness_score_prefers_ready_unmastered_targets() {
        let config = ConceptSchedulerConfig::default();
        let mut graph = KnowledgeGraph::default();
        graph.add_prerequisite(kc("Biochem::Glycolysis"), kc("Biochem::Bioenergetics"));
        graph.add_prerequisite(kc("Biochem::Citric_Acid_Cycle"), kc("Biochem::Glycolysis"));

        let mut state = ConceptSchedulerState::default();
        state.kcs.insert(
            kc("Biochem::Bioenergetics"),
            ConceptMasteryState {
                mastery: 0.85,
                answered: 3,
                positive: 3,
                negative: 0,
            },
        );
        state.kcs.insert(
            kc("Biochem::Glycolysis"),
            ConceptMasteryState {
                mastery: 0.20,
                answered: 1,
                positive: 0,
                negative: 1,
            },
        );

        let glycolysis = state.readiness_score(&kc("Biochem::Glycolysis"), &graph, &config);
        let citric_acid_cycle =
            state.readiness_score(&kc("Biochem::Citric_Acid_Cycle"), &graph, &config);

        assert_approx_eq(glycolysis.score, 0.68);
        assert_approx_eq(citric_acid_cycle.score, 0.16);
        assert!(glycolysis.score > citric_acid_cycle.score);
    }

    #[test]
    fn readiness_status_refuses_when_evidence_is_thin() {
        let config = ConceptSchedulerConfig {
            readiness_min_seen_cards: 50,
            ..ConceptSchedulerConfig::default()
        };
        let mut state = ConceptSchedulerState {
            total_seen_cards: 49,
            ..ConceptSchedulerState::default()
        };

        assert_eq!(
            state.readiness_evidence_status(&config),
            ReadinessEvidenceStatus::InsufficientEvidence {
                seen_cards: 49,
                required_seen_cards: 50
            }
        );

        state.total_seen_cards = 50;
        assert_eq!(
            state.readiness_evidence_status(&config),
            ReadinessEvidenceStatus::EnoughEvidence { seen_cards: 50 }
        );
    }

    #[test]
    fn irt_item_metadata_uses_seeded_defaults() {
        let metadata = CardConceptMetadata::from_tags([
            "KC::Biochem::Enzymes",
            "MCAT::Bio_Biochem",
            "Difficulty::5",
        ]);
        let item = IrtItemMetadata::from_card_metadata(&metadata);

        assert_approx_eq(item.difficulty, 2.0);
        assert_approx_eq(item.discrimination, 1.0);
        assert_approx_eq(item.guessing, 0.25);
        assert!(item.probability_correct(2.0) > item.probability_correct(-2.0));
    }

    #[test]
    fn irt_section_state_updates_theta_and_error() {
        let mut section = IrtSectionState::default();
        let item = IrtItemMetadata {
            difficulty: 0.0,
            discrimination: 1.0,
            guessing: 0.25,
        };

        section.record_response(item, true, [McatDiscipline::Biochemistry]);
        let after_correct = section.theta;
        section.record_response(item, false, [McatDiscipline::Biochemistry]);

        assert!(after_correct > 0.0);
        assert_eq!(section.answered, 2);
        assert_eq!(section.correct, 1);
        assert!(section.information > 0.0);
        assert!(section.performance_standard_error().is_finite());
    }

    #[test]
    fn section_score_combines_performance_mastery_and_coverage() {
        let config = ConceptSchedulerConfig {
            irt_min_section_items: 2,
            irt_min_section_coverage: 0.5,
            ..ConceptSchedulerConfig::default()
        };
        let mut graph = KnowledgeGraph::default();
        graph.add_component(kc("Bio::DNA"));
        graph.add_component(kc("Biochem::Enzymes"));
        let mut state = ConceptSchedulerState::default();
        state.kcs.insert(
            kc("Bio::DNA"),
            ConceptMasteryState {
                mastery: 0.9,
                answered: 13,
                positive: 13,
                negative: 0,
            },
        );
        let item = IrtItemMetadata {
            difficulty: 0.0,
            discrimination: 1.0,
            guessing: 0.25,
        };
        for _ in 0..13 {
            state.record_irt_response(
                McatSection::BioBiochem,
                [McatDiscipline::Biology],
                item,
                Evidence::Positive,
            );
        }

        let score = state.section_score_status(McatSection::BioBiochem, &graph, &config, None);

        assert!(score.enough_evidence);
        assert_eq!(score.answered_items, 13);
        assert!(score.coverage >= 0.5);
        assert!(score.readiness_lower <= score.readiness_center);
        assert!(score.readiness_upper >= score.readiness_center);
    }

    #[test]
    fn section_coverage_is_capped_by_discipline_weight() {
        let config = ConceptSchedulerConfig {
            irt_min_section_items: 1,
            irt_min_section_coverage: 0.60,
            ..ConceptSchedulerConfig::default()
        };
        let mut graph = KnowledgeGraph::default();
        graph.add_component(kc("Bio::DNA"));
        graph.add_component(kc("PsychSoc::Memory"));
        let mut state = ConceptSchedulerState::default();

        for _ in 0..100 {
            state.record_evidence(
                kc("Bio::DNA"),
                Evidence::Positive,
                &ConceptSchedulerConfig::default(),
            );
        }

        let score = state.section_score_status(McatSection::PsychSoc, &graph, &config, None);

        assert_approx_eq(score.coverage, 0.05);
        assert!(!score.enough_evidence);
    }

    #[test]
    fn readiness_is_capped_by_partial_coverage() {
        let config = ConceptSchedulerConfig {
            irt_min_section_items: 1,
            irt_min_section_coverage: 0.5,
            ..ConceptSchedulerConfig::default()
        };
        let mut graph = KnowledgeGraph::default();
        graph.add_component(kc("Bio::DNA"));
        let mut state = ConceptSchedulerState::default();

        // Strong biology evidence (high mastery + many correct, easy items) drives
        // performance toward the top of the scale, but the Bio/Biochem blueprint is
        // only ~65% covered because no biochem/chem items were answered.
        let item = IrtItemMetadata {
            difficulty: -1.0,
            discrimination: 1.5,
            guessing: 0.2,
        };
        for _ in 0..40 {
            state.record_evidence(kc("Bio::DNA"), Evidence::Positive, &config);
            state.record_irt_response(
                McatSection::BioBiochem,
                [McatDiscipline::Biology],
                item,
                Evidence::Positive,
            );
        }

        // No memory signal in this unit test => retention defaults to 1.0, so
        // readiness is the coverage-weighted blend of performance and the floor.
        let score = state.section_score_status(McatSection::BioBiochem, &graph, &config, None);
        let floor = config.guessing_baseline_score;
        let performance = score.performance_center;

        // Strong correct evidence lifts performance (theta -> scaled score) above the
        // guessing floor, but partial coverage keeps readiness strictly between them.
        assert!(score.coverage < 0.7);
        assert!(performance > floor);
        assert!(score.readiness_center > floor);
        assert!(score.readiness_center < performance);
        // Readiness is exactly the coverage-weighted blend from the guessing floor
        // (retention = 1 here): floor + coverage * (performance - floor).
        assert_approx_eq(
            score.readiness_center,
            floor + score.coverage * (performance - floor),
        );
        assert!(score.readiness_center >= 118.0);
    }

    #[test]
    fn full_coverage_lets_readiness_reach_performance() {
        let config = ConceptSchedulerConfig {
            irt_min_section_items: 1,
            irt_min_section_coverage: 0.5,
            ..ConceptSchedulerConfig::default()
        };
        // CARS is a single-discipline section (100% weight), so one discipline can
        // reach full coverage on its own.
        let mut graph = KnowledgeGraph::default();
        graph.add_component(kc("CARS::Reasoning"));
        let mut state = ConceptSchedulerState::default();

        let item = IrtItemMetadata {
            difficulty: 0.0,
            discrimination: 1.0,
            guessing: 0.25,
        };
        for _ in 0..40 {
            state.record_evidence(kc("CARS::Reasoning"), Evidence::Positive, &config);
            state.record_irt_response(McatSection::Cars, [McatDiscipline::Cars], item, Evidence::Positive);
        }

        let score = state.section_score_status(McatSection::Cars, &graph, &config, None);

        assert_approx_eq(score.coverage, 1.0);
        // At full coverage (and full retention) the blend collapses to the
        // performance center itself (theta -> AAMC-scaled score).
        assert_approx_eq(score.readiness_center, score.performance_center);
    }

    #[test]
    fn concept_scheduler_state_round_trips_via_deck_config() -> Result<()> {
        let mut col = Collection::new();
        let mut persisted = PersistedConceptSchedulerState::default();
        persisted
            .state
            .record_evidence(kc("Bio::DNA"), Evidence::Positive, &persisted.config);
        persisted.state.note_card_answered();

        col.set_concept_scheduler_state(DeckId(1), &persisted, false)?;
        let loaded = col.get_concept_scheduler_state(DeckId(1));

        assert_eq!(loaded.schema_version, CONCEPT_SCHEDULER_STATE_SCHEMA_VERSION);
        assert_eq!(loaded.state.total_seen_cards, 1);
        assert_approx_eq(
            loaded.state.mastery_for(&kc("Bio::DNA"), &loaded.config),
            0.391_304_347_826_087,
        );

        Ok(())
    }

    #[test]
    fn old_persisted_config_migrates_to_current_defaults() -> Result<()> {
        let mut col = Collection::new();
        // Simulate a deck saved under an older schema with the previous prior (0.25).
        let mut old = PersistedConceptSchedulerState::default();
        old.schema_version = 1;
        old.config.initial_mastery = 0.25;
        old.state
            .record_evidence(kc("Bio::DNA"), Evidence::Positive, &old.config);
        old.state.note_card_answered();
        col.set_concept_scheduler_state(DeckId(1), &old, false)?;

        let loaded = col.get_concept_scheduler_state(DeckId(1));

        // Tuning constants refresh to current defaults; learner state is preserved.
        assert_eq!(loaded.schema_version, CONCEPT_SCHEDULER_STATE_SCHEMA_VERSION);
        assert_approx_eq(loaded.config.initial_mastery, 0.20);
        assert_eq!(loaded.state.total_seen_cards, 1);

        Ok(())
    }

    #[test]
    fn unknown_target_uses_fallback_score() {
        let config = ConceptSchedulerConfig::default();
        let graph = KnowledgeGraph::default();
        let state = ConceptSchedulerState::default();

        let score = state.readiness_score(&kc("Bio::Unknown"), &graph, &config);

        assert_eq!(score.reason, ReadinessScoreReason::UnknownTarget);
        assert_approx_eq(score.score, config.fallback_readiness_score);
    }

    #[test]
    fn readiness_defaults_to_the_prior_without_evidence() {
        let config = ConceptSchedulerConfig::default();
        let mut graph = KnowledgeGraph::default();
        graph.add_component(kc("Bio::DNA"));
        let state = ConceptSchedulerState::default();

        let score = state.section_score_status(McatSection::BioBiochem, &graph, &config, None);

        // No coverage => nothing demonstrated/retained => readiness sits at the
        // guessing floor (~120), NOT the population median (125): you can't assume
        // median competence on unstudied material. It never reads 0.
        assert_eq!(state.section_accuracy(McatSection::BioBiochem, &graph), None);
        assert_approx_eq(score.readiness_center, config.guessing_baseline_score);
        assert!(score.readiness_center >= 118.0);
    }

    #[test]
    fn aamc_scale_maps_theta_to_median_and_clamps() {
        // theta 0 => population median 125; +/-1 SD => +/- SCALE_A; extremes clamp.
        assert_approx_eq(theta_to_scaled_score(0.0), SCALE_B);
        assert_approx_eq(theta_to_scaled_score(1.0), SCALE_B + SCALE_A);
        assert_approx_eq(theta_to_scaled_score(-1.0), SCALE_B - SCALE_A);
        assert_approx_eq(theta_to_scaled_score(10.0), SCORE_MAX);
        assert_approx_eq(theta_to_scaled_score(-10.0), SCORE_MIN);
    }

    #[test]
    fn probability_at_least_matches_normal_tails() {
        // At the center, ~50% chance of reaching the target.
        assert!((probability_at_least(510.0, 3.0, 510.0) - 0.5).abs() < 1e-6);
        // Target well below center => near-certain; well above => near-impossible.
        assert!(probability_at_least(515.0, 2.0, 505.0) > 0.99);
        assert!(probability_at_least(500.0, 2.0, 520.0) < 0.01);
        // ~1 SD above center => ~16%.
        assert!((probability_at_least(510.0, 5.0, 515.0) - 0.1587).abs() < 0.01);
        // Zero spread degenerates to a hard threshold.
        assert_eq!(probability_at_least(510.0, 0.0, 505.0), 1.0);
        assert_eq!(probability_at_least(500.0, 0.0, 505.0), 0.0);
    }

    #[test]
    fn memory_discounts_readiness_toward_the_floor() {
        let config = ConceptSchedulerConfig {
            irt_min_section_items: 1,
            irt_min_section_coverage: 0.5,
            ..ConceptSchedulerConfig::default()
        };
        let mut graph = KnowledgeGraph::default();
        graph.add_component(kc("CARS::Reasoning"));
        let mut state = ConceptSchedulerState::default();
        let item = IrtItemMetadata {
            difficulty: 0.0,
            discrimination: 1.0,
            guessing: 0.25,
        };
        for _ in 0..40 {
            state.record_evidence(kc("CARS::Reasoning"), Evidence::Positive, &config);
            state.record_irt_response(
                McatSection::Cars,
                [McatDiscipline::Cars],
                item,
                Evidence::Positive,
            );
        }

        // Same performance and coverage, different projected retention.
        let full = state.section_score_status(McatSection::Cars, &graph, &config, Some(1.0));
        let half = state.section_score_status(McatSection::Cars, &graph, &config, Some(0.5));
        let floor = config.guessing_baseline_score;

        assert!(full.performance_center > floor);
        // Forgetting pulls readiness back toward the guessing floor.
        assert!(half.readiness_center < full.readiness_center);
        // Halving retention halves the gain above the floor (coverage cancels out).
        assert_approx_eq(
            half.readiness_center - floor,
            0.5 * (full.readiness_center - floor),
        );
    }

    #[test]
    fn psychsoc_kcs_split_into_psychology_and_sociology() {
        // Sub-domain split from kc-map-unified.md §6, so the 30% sociology slice can fill.
        assert_eq!(
            McatDiscipline::for_component(&kc("PsychSoc::Culture")),
            Some(McatDiscipline::Sociology)
        );
        assert_eq!(
            McatDiscipline::for_component(&kc("PsychSoc::Social_Class")),
            Some(McatDiscipline::Sociology)
        );
        // Psychology KCs (and the dual-coded Social-Psychology ones) stay psychology.
        assert_eq!(
            McatDiscipline::for_component(&kc("PsychSoc::Memory")),
            Some(McatDiscipline::Psychology)
        );
        assert_eq!(
            McatDiscipline::for_component(&kc("PsychSoc::Group_Behavior")),
            Some(McatDiscipline::Psychology)
        );
    }

    #[test]
    fn coverage_rewards_breadth_not_grinding_one_kc() {
        let config = ConceptSchedulerConfig::default();
        let mut graph = KnowledgeGraph::default();
        for id in ["Bio::A", "Bio::B", "Bio::C", "Bio::D", "Bio::E"] {
            graph.add_component(kc(id));
        }

        // Grind a SINGLE KC many times.
        let mut grind = ConceptSchedulerState::default();
        for _ in 0..20 {
            grind.record_evidence(kc("Bio::A"), Evidence::Positive, &config);
        }
        // Spread FEWER answers across several distinct KCs.
        let mut spread = ConceptSchedulerState::default();
        for id in ["Bio::A", "Bio::B", "Bio::C", "Bio::D"] {
            spread.record_evidence(kc(id), Evidence::Positive, &config);
        }

        let grind_coverage = grind.section_coverage(McatSection::BioBiochem, &graph);
        let spread_coverage = spread.section_coverage(McatSection::BioBiochem, &graph);
        assert!(
            spread_coverage > grind_coverage,
            "breadth ({spread_coverage}) should beat volume ({grind_coverage})"
        );
    }

    #[test]
    fn total_seen_counts_cards_not_kcs() {
        let config = ConceptSchedulerConfig::default();
        let mut state = ConceptSchedulerState::default();
        // One multi-KC card answer: two component evidences, but one card seen.
        state.record_evidence(kc("Bio::DNA"), Evidence::Positive, &config);
        state.record_evidence(kc("Biochem::Glycolysis"), Evidence::Positive, &config);
        state.note_card_answered();
        assert_eq!(state.total_seen_cards, 1);
    }

    #[test]
    fn conditional_likelihood_goes_empirical_after_enough_observations() {
        let mut obs = LikelihoodObservations::default();
        // At the threshold the group is still on the static default.
        for _ in 0..LIKELIHOOD_MIN_OBSERVATIONS {
            obs.record(false, Evidence::Positive);
        }
        assert_approx_eq(obs.likelihood(false, Evidence::Positive, 0.20), 0.20);

        // One MORE observation (now over the threshold) flips the unmastered group to
        // its empirical (Laplace-smoothed) rate; here every unmastered answer was right.
        obs.record(false, Evidence::Positive);
        let total = (LIKELIHOOD_MIN_OBSERVATIONS + 1) as f64;
        let expected = (total + 1.0) / (total + 2.0);
        assert_approx_eq(obs.likelihood(false, Evidence::Positive, 0.20), expected);
        // The complementary outcome derives from the same counts and sums to 1.
        assert_approx_eq(obs.likelihood(false, Evidence::Negative, 0.80), 1.0 - expected);
        // The mastered group has no data, so it stays on its static default.
        assert_approx_eq(obs.likelihood(true, Evidence::Positive, 0.90), 0.90);
    }

    #[test]
    fn record_evidence_collects_conditional_observations() {
        let config = ConceptSchedulerConfig::default();
        let mut state = ConceptSchedulerState::default();
        // A fresh KC starts unmastered (initial 0.20 < the mastered bar), so a correct
        // answer is recorded as an unmastered-positive observation.
        state.record_evidence(kc("Bio::DNA"), Evidence::Positive, &config);
        assert_eq!(state.likelihood_observations.unmastered_positive, 1);
        assert_eq!(state.likelihood_observations.mastered_positive, 0);
    }

    #[test]
    fn unmastered_correct_likelihood_rises_with_mastery() {
        let config = ConceptSchedulerConfig::default();
        let state = ConceptSchedulerState::default();
        // No observations yet, so the unmastered "correct" default is the mastery-
        // scaled guess model 0.25 + 0.5*mastery (not a flat constant).
        let (_lm_lo, lu_lo) = state.effective_likelihoods(Evidence::Positive, 0.20, &config);
        let (_lm_hi, lu_hi) = state.effective_likelihoods(Evidence::Positive, 0.80, &config);
        assert_approx_eq(lu_lo, GUESS_FLOOR + PARTIAL_MASTERY_LIFT * 0.20);
        assert_approx_eq(lu_hi, GUESS_FLOOR + PARTIAL_MASTERY_LIFT * 0.80);
        assert!(lu_hi > lu_lo);
    }

    /// Deliverable B: emit engine-derived reference values so the held-out Python
    /// model evals can be locked to the REAL Rust engine. Every value below is
    /// produced by the SHIPPED engine functions (`difficulty_to_irt_b`,
    /// `IrtItemMetadata::probability_correct`, `concept_demo::base_recall_for_button`,
    /// the `fsrs` crate) or constants; `evals/test_parity.py` reproduces each from the
    /// Python eval modules and asserts they match to 1e-9. Writing a fixture file from
    /// a test is intentional here.
    #[test]
    fn emit_engine_parity_fixture() {
        use fsrs::MemoryState;
        use fsrs::FSRS;
        use fsrs::FSRS5_DEFAULT_DECAY;
        use serde_json::json;

        // FSRS-5 default power forgetting-curve constants. The Python eval builds its
        // ANALYTIC curve from exactly these (see the honest caveat in
        // ENGINE-FIDELITY.md); it is NOT a direct fsrs-crate call.
        let fsrs_decay = FSRS5_DEFAULT_DECAY as f64;
        let fsrs_factor = 0.9_f64.powf(1.0 / -fsrs_decay) - 1.0;
        // concept_demo::MEMORY_HORIZON_SECS is 86_400 (one day); recall is projected to
        // at least this forward horizon.
        let memory_horizon_days = 86_400.0_f64 / 86_400.0;

        // difficulty tag (1..=5) -> IRT b, via the engine's difficulty_to_irt_b table.
        let difficulty_to_b: Vec<_> = (1u8..=5)
            .map(|d| json!({ "difficulty": d, "b": difficulty_to_irt_b(d) }))
            .collect();

        // IRT 3PL probability_correct over a grid, via the REAL IrtItemMetadata.
        let thetas = [-2.0_f64, -1.0, 0.0, 1.0, 2.0];
        let discriminations = [0.8_f64, 1.0, 1.4];
        let guessings = [0.2_f64, 0.25];
        let mut irt = Vec::new();
        for d in 1u8..=5 {
            let b = difficulty_to_irt_b(d);
            for &theta in &thetas {
                for &discrimination in &discriminations {
                    for &guessing in &guessings {
                        let item = IrtItemMetadata {
                            difficulty: b,
                            discrimination,
                            guessing,
                        };
                        irt.push(json!({
                            "difficulty": d,
                            "b": b,
                            "theta": theta,
                            "discrimination": discrimination,
                            "guessing": guessing,
                            "p": item.probability_correct(theta),
                        }));
                    }
                }
            }
        }

        // Memory fallback (no FSRS state): base_recall_for_button + rating decay.
        let base_recall: Vec<_> = (1u8..=4)
            .map(|button| {
                json!({
                    "button": button,
                    "base": crate::scheduler::concept_demo::base_recall_for_button(button).unwrap(),
                })
            })
            .collect();

        let fallback_elapsed = [0.5_f64, 1.0, 5.0, 20.0];
        let fallback_interval = [1.0_f64, 3.0, 14.0];
        let mut memory_fallback = Vec::new();
        for button in 1u8..=4 {
            let base = crate::scheduler::concept_demo::base_recall_for_button(button).unwrap();
            for &elapsed_days in &fallback_elapsed {
                for &interval_days in &fallback_interval {
                    // Mirrors concept_demo::card_memory's fallback branch exactly.
                    let decay_factor = (-elapsed_days / interval_days.max(1.0)).exp();
                    let recall = (base * decay_factor).clamp(0.0, 1.0);
                    memory_fallback.push(json!({
                        "button": button,
                        "base": base,
                        "elapsed_days": elapsed_days,
                        "interval_days": interval_days,
                        "recall": recall,
                    }));
                }
            }
        }

        // FSRS-5 retrievability. `recall_analytic` is the power curve the Python eval
        // reproduces (the parity target). `recall_fsrs_crate` is the actual fsrs-crate
        // value (f32) at the same point, recorded so the writeup can honestly show the
        // analytic curve and the crate coincide to f32 precision at the default decay.
        let fsrs = FSRS::new(None).unwrap();
        let fsrs_elapsed = [0.5_f64, 1.0, 3.0, 10.0, 30.0];
        let fsrs_stability = [1.0_f64, 5.0, 20.0, 60.0];
        let mut fsrs_curve = Vec::new();
        for &elapsed_days in &fsrs_elapsed {
            let t = elapsed_days.max(memory_horizon_days);
            for &stability in &fsrs_stability {
                let s = stability.max(1e-6);
                let recall_analytic = (1.0 + fsrs_factor * t / s).powf(-fsrs_decay).clamp(0.0, 1.0);
                let seconds = (t * 86_400.0) as u32;
                let recall_crate = fsrs.current_retrievability_seconds(
                    MemoryState {
                        stability: stability as f32,
                        difficulty: 5.0,
                    },
                    seconds,
                    FSRS5_DEFAULT_DECAY,
                ) as f64;
                fsrs_curve.push(json!({
                    "elapsed_days": elapsed_days,
                    "stability": stability,
                    "t_eval_days": t,
                    "recall_analytic": recall_analytic,
                    "recall_fsrs_crate": recall_crate,
                }));
            }
        }

        let fixture = json!({
            "meta": {
                "description": "Reference values emitted by the REAL Anki Rust engine for \
                                Python<->Rust parity of the MCAT model evals.",
                "generated_by": "anki/rslib/src/scheduler/concept.rs::tests::emit_engine_parity_fixture",
                "tolerance": 1e-9,
            },
            "fsrs_constants": {
                "fsrs5_default_decay": fsrs_decay,
                "fsrs_factor": fsrs_factor,
                "memory_horizon_days": memory_horizon_days,
            },
            "difficulty_to_irt_b": difficulty_to_b,
            "irt_probability_correct": irt,
            "base_recall_for_button": base_recall,
            "memory_fallback": memory_fallback,
            "fsrs_curve": fsrs_curve,
        });

        // Package `anki` lives at anki/rslib, so the repo root is two levels up.
        let repo_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .parent()
            .unwrap();
        let path = repo_root.join("evals/fixtures/engine_parity.json");
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(&path, serde_json::to_string_pretty(&fixture).unwrap() + "\n").unwrap();
        eprintln!("wrote engine parity fixture: {}", path.display());
    }
}
