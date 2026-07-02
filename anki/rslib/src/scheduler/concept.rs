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
const CONCEPT_SCHEDULER_STATE_SCHEMA_VERSION: u32 = 1;

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

    pub(crate) fn as_str(self) -> &'static str {
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
            // The draft Psych/Soc lattice does not yet split psychology and
            // sociology KCs, so keep the MVP conservative.
            Some(Self::Psychology)
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
            initial_mastery: 0.25,
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

impl Collection {
    pub(crate) fn get_concept_scheduler_state(
        &self,
        deck_id: DeckId,
    ) -> PersistedConceptSchedulerState {
        let key = DeckConfigKey::ConceptSchedulerState.for_deck(deck_id);
        self.get_config_optional(key.as_str()).unwrap_or_default()
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
        let evidence = evidence_from_rating(rating);
        if metadata.prerequisites.iter().any(|prerequisite| {
            persisted.state.mastery_for(prerequisite, &config) < config.outer_fringe_prereq_mastery
        }) {
            persisted.state.record_prerequisite_violation(day);
        }
        for component in &metadata.target_components {
            persisted
                .state
                .record_evidence_on_day(component.clone(), evidence, day, &config);
        }
        let irt_item = IrtItemMetadata::from_card_metadata(&metadata);
        for section in metadata
            .sections
            .iter()
            .filter_map(|section| McatSection::from_tag(section))
        {
            let disciplines = section_disciplines_for_metadata(&metadata, section);
            persisted
                .state
                .record_irt_response(section, disciplines, irt_item, evidence);
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

    pub(crate) fn record_evidence(&mut self, evidence: Evidence, config: &ConceptSchedulerConfig) {
        let (likelihood_if_mastered, likelihood_if_unmastered) = match evidence {
            Evidence::Positive => {
                self.positive += 1;
                (
                    config.positive_likelihood_if_mastered,
                    config.positive_likelihood_if_unmastered,
                )
            }
            Evidence::Negative => {
                self.negative += 1;
                (
                    config.negative_likelihood_if_mastered,
                    config.negative_likelihood_if_unmastered,
                )
            }
        };

        self.answered += 1;
        self.mastery = bayes_update(
            self.mastery,
            likelihood_if_mastered,
            likelihood_if_unmastered,
        );
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

    fn coverage_for_section(&self, section: McatSection) -> f64 {
        section_disciplines(section)
            .iter()
            .map(|(discipline, weight)| {
                let count = self.discipline_counts.get(discipline).copied().unwrap_or(0);
                let required = required_items_for_weight(*weight);
                weight * (count as f64 / required as f64).min(1.0)
            })
            .sum()
    }
}

fn theta_to_scaled_score(theta: f64) -> f64 {
    (125.0 + (theta * 2.5)).clamp(118.0, 132.0)
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
        self.ensure_component(component, config)
            .record_evidence(evidence, config);
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
    ) -> SectionScoreStatus {
        let irt = self.irt_sections.get(&section).cloned().unwrap_or_default();
        let section_mastery = self.section_mastery(section, graph, config);
        let coverage = self.section_coverage(section, graph).clamp(0.0, 1.0);
        let performance_center = irt.scaled_score();
        let performance_standard_error = irt.performance_standard_error();

        // Readiness projects a whole-section score. The blueprint is scored as a
        // blend: the covered fraction scores at demonstrated performance, while
        // the untested (uncovered) fraction has no evidence and is assumed to
        // score at the guessing baseline. Coverage is therefore a hard ceiling:
        // the most readiness can reach is `guess + coverage * (132 - guess)`, so
        // a section can only approach 132 once it is fully covered.
        let guess_floor = config.guessing_baseline_score;
        let readiness_center =
            (guess_floor + coverage * (performance_center - guess_floor)).clamp(118.0, 132.0);

        // Uncertainty: the performance term only informs the covered fraction, so
        // it is scaled by coverage; thin coverage and weak section mastery widen
        // the band. Variances are summed, then square-rooted.
        let performance_standard_error_component = coverage * performance_standard_error;
        let coverage_standard_error = (1.0 - coverage) * config.max_coverage_standard_error;
        let mastery_standard_error = (1.0 - section_mastery) * config.max_mastery_standard_error;
        let readiness_standard_error = ((performance_standard_error_component
            * performance_standard_error_component)
            + (coverage_standard_error * coverage_standard_error)
            + (mastery_standard_error * mastery_standard_error))
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
                .clamp(118.0, 132.0),
            performance_upper: (performance_center + (1.96 * performance_standard_error))
                .clamp(118.0, 132.0),
            section_mastery,
            coverage,
            readiness_center,
            readiness_standard_error,
            readiness_lower: (readiness_center - (1.96 * readiness_standard_error))
                .clamp(118.0, 132.0),
            readiness_upper: (readiness_center + (1.96 * readiness_standard_error))
                .clamp(118.0, 132.0),
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
                let answered = self.answered_for_discipline(*discipline, graph);
                let required = required_items_for_weight(*weight);
                weight * (answered as f64 / required as f64).min(1.0)
            })
            .sum()
    }

    fn answered_for_discipline(&self, discipline: McatDiscipline, graph: &KnowledgeGraph) -> u32 {
        graph
            .components()
            .filter(|component| McatDiscipline::for_component(component) == Some(discipline))
            .map(|component| {
                self.kcs
                    .get(component)
                    .map(|state| state.answered)
                    .unwrap_or_default()
            })
            .sum()
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

        state.record_evidence(Evidence::Positive, &config);
        assert_approx_eq(state.mastery, 0.6);
        assert_eq!(state.answered, 1);
        assert_eq!(state.positive, 1);

        state.record_evidence(Evidence::Negative, &config);
        assert_approx_eq(state.mastery, 0.157_894_736_842_105_25);
        assert_eq!(state.answered, 2);
        assert_eq!(state.negative, 1);
    }

    #[test]
    fn scheduler_state_records_component_evidence() {
        let config = ConceptSchedulerConfig::default();
        let mut state = ConceptSchedulerState::default();

        state.record_evidence(kc("Bio::DNA"), Evidence::Positive, &config);

        let dna = state.kcs.get(&kc("Bio::DNA")).unwrap();
        assert_approx_eq(dna.mastery, 0.6);
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
        assert_approx_eq(citric_acid_cycle.score, 0.15);
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

        let score = state.section_score_status(McatSection::BioBiochem, &graph, &config);

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

        let score = state.section_score_status(McatSection::PsychSoc, &graph, &config);

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

        let score = state.section_score_status(McatSection::BioBiochem, &graph, &config);
        let guess_floor = config.guessing_baseline_score;

        // Coverage is well under 100%, so readiness must sit below performance.
        assert!(score.coverage < 0.7);
        assert!(score.performance_center > guess_floor);
        assert!(score.readiness_center < score.performance_center);
        // Readiness is exactly the coverage-weighted blend toward the guessing floor.
        assert_approx_eq(
            score.readiness_center,
            guess_floor + score.coverage * (score.performance_center - guess_floor),
        );
        // It can never exceed the coverage ceiling, even if performance is maxed.
        let ceiling = guess_floor + score.coverage * (132.0 - guess_floor);
        assert!(score.readiness_center <= ceiling + 1e-9);
        assert!(ceiling < 132.0);
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

        let score = state.section_score_status(McatSection::Cars, &graph, &config);

        assert_approx_eq(score.coverage, 1.0);
        // At full coverage the blend collapses to performance itself.
        assert_approx_eq(score.readiness_center, score.performance_center);
    }

    #[test]
    fn concept_scheduler_state_round_trips_via_deck_config() -> Result<()> {
        let mut col = Collection::new();
        let mut persisted = PersistedConceptSchedulerState::default();
        persisted
            .state
            .record_evidence(kc("Bio::DNA"), Evidence::Positive, &persisted.config);

        col.set_concept_scheduler_state(DeckId(1), &persisted, false)?;
        let loaded = col.get_concept_scheduler_state(DeckId(1));

        assert_eq!(loaded.schema_version, 1);
        assert_eq!(loaded.state.total_seen_cards, 1);
        assert_approx_eq(
            loaded.state.mastery_for(&kc("Bio::DNA"), &loaded.config),
            0.6,
        );

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
}
