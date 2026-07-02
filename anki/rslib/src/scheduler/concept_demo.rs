// Copyright: Ankitects Pty Ltd and contributors
// License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html

use std::collections::BTreeMap;

use anki_proto::scheduler;
use fsrs::FSRS;
use fsrs::FSRS5_DEFAULT_DECAY;

use super::concept::section_memory_from_kc_memory;
use super::concept::CardConceptMetadata;
use super::concept::ConceptSchedulerState;
use super::concept::Evidence;
use super::concept::KnowledgeComponentId;
use super::concept::KnowledgeGraph;
use super::concept::McatSection;
use super::concept::ReadinessEvidenceStatus;
use super::concept::SectionScoreStatus;
use crate::card::Card;
use crate::prelude::*;

const MCAT_DEMO_CARDS_MD: &str = include_str!("../../../added features/mcat_demo_cards.md");

pub(crate) const MCAT_DEMO_DECK_NAME: &str = "MCAT Demo";

const DEMO_KCS: [&str; 10] = [
    "Bio::DNA",
    "Bio::Genetics",
    "Bio::Eukaryotic_Cell",
    "Biochem::Amino_Acids",
    "Biochem::Peptides_and_Proteins",
    "Biochem::Protein_Structure_and_Function",
    "Biochem::Enzymes",
    "Biochem::Bioenergetics",
    "Biochem::Glycolysis",
    "Biochem::Citric_Acid_Cycle",
];

// Edges are authored as (prerequisite, target). KnowledgeGraph stores the same
// relationship on the target node, via add_prerequisite(target, prerequisite).
const DEMO_EDGES: [(&str, &str); 9] = [
    ("Bio::DNA", "Bio::Genetics"),
    ("Biochem::Amino_Acids", "Biochem::Peptides_and_Proteins"),
    (
        "Biochem::Peptides_and_Proteins",
        "Biochem::Protein_Structure_and_Function",
    ),
    (
        "Biochem::Protein_Structure_and_Function",
        "Biochem::Enzymes",
    ),
    ("Biochem::Enzymes", "Biochem::Bioenergetics"),
    ("Biochem::Bioenergetics", "Biochem::Glycolysis"),
    ("Biochem::Bioenergetics", "Biochem::Citric_Acid_Cycle"),
    ("Biochem::Glycolysis", "Biochem::Citric_Acid_Cycle"),
    ("Bio::Eukaryotic_Cell", "Biochem::Bioenergetics"),
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct McatDemoCard {
    pub(crate) id: String,
    pub(crate) kc: String,
    pub(crate) prerequisites: Vec<String>,
    pub(crate) difficulty: u8,
    pub(crate) question: String,
    pub(crate) answers: [String; 4],
    pub(crate) correct: String,
    pub(crate) explanation: String,
    pub(crate) tags: Vec<String>,
}

pub(crate) fn parse_mcat_demo_cards() -> Vec<McatDemoCard> {
    let mut cards = Vec::new();
    let mut current: Option<McatDemoCardBuilder> = None;

    for line in MCAT_DEMO_CARDS_MD.lines() {
        if let Some(id) = line.strip_prefix("### ") {
            if let Some(builder) = current.take() {
                cards.push(builder.build());
            }
            current = Some(McatDemoCardBuilder::new(id));
            continue;
        }

        let Some(builder) = current.as_mut() else {
            continue;
        };

        if let Some((key, value)) = parse_demo_bullet(line) {
            match key {
                "KC" => builder.kc = strip_single_backticks(value).to_string(),
                "Prereqs" => {
                    builder.prerequisites = if value == "none" {
                        Vec::new()
                    } else {
                        backtick_values(value)
                            .into_iter()
                            .filter_map(|tag| tag.strip_prefix("Prereq::").map(ToString::to_string))
                            .collect()
                    };
                }
                "Difficulty" => builder.difficulty = value.parse().ok(),
                "Question" => builder.question = Some(value.to_string()),
                "A" => builder.answers[0] = Some(value.to_string()),
                "B" => builder.answers[1] = Some(value.to_string()),
                "C" => builder.answers[2] = Some(value.to_string()),
                "D" => builder.answers[3] = Some(value.to_string()),
                "Correct" => builder.correct = Some(value.to_string()),
                "Explanation" => builder.explanation = Some(value.to_string()),
                "Tags" => builder.tags = backtick_values(value),
                _ => (),
            }
        }
    }

    if let Some(builder) = current.take() {
        cards.push(builder.build());
    }

    cards
}

pub(crate) fn canonical_mcat_demo_graph() -> KnowledgeGraph {
    let mut graph = KnowledgeGraph::default();
    for kc in DEMO_KCS {
        graph.add_component(kc_id(kc));
    }
    for (prerequisite, target) in DEMO_EDGES {
        graph.add_prerequisite(kc_id(target), kc_id(prerequisite));
    }
    graph
}

pub(crate) fn canonical_mcat_demo_edges() -> Vec<(KnowledgeComponentId, KnowledgeComponentId)> {
    DEMO_EDGES
        .into_iter()
        .map(|(prerequisite, target)| (kc_id(prerequisite), kc_id(target)))
        .collect()
}

impl Collection {
    pub(crate) fn import_mcat_demo_deck(&mut self) -> Result<DeckId> {
        let mut deck = self.get_or_create_normal_deck(MCAT_DEMO_DECK_NAME)?;
        deck.normal_mut()?.concept_scheduler_enabled = true;
        self.add_or_update_deck(&mut deck)?;

        let mut persisted = self.get_concept_scheduler_state(deck.id);
        persisted.config.readiness_min_seen_cards = 10;
        self.set_concept_scheduler_state(deck.id, &persisted, false)?;

        if !self.storage.all_cards_in_single_deck(deck.id)?.is_empty() {
            self.set_current_deck(deck.id)?;
            return Ok(deck.id);
        }

        let nt = self.get_notetype_by_name("Basic")?.unwrap();
        for card in parse_mcat_demo_cards() {
            let mut note = nt.new_note();
            note.set_field(0, card.front_html())?;
            note.set_field(1, card.back_html())?;
            note.tags = card.tags;
            self.add_note(&mut note, deck.id)?;
        }

        self.set_current_deck(deck.id)?;

        Ok(deck.id)
    }

    pub(crate) fn concept_scheduler_status(
        &mut self,
        deck_id: DeckId,
    ) -> Result<scheduler::ConceptSchedulerStatusResponse> {
        let deck = self.storage.get_deck(deck_id)?.or_not_found(deck_id)?;
        let enabled = deck
            .normal()
            .map(|normal| normal.concept_scheduler_enabled)
            .unwrap_or(false);
        let mut persisted = self.get_concept_scheduler_state(deck_id);
        let reconstructed =
            self.reconstruct_mcat_demo_state_from_revlogs(deck_id, &persisted.config)?;
        if merge_reconstructed_state(&mut persisted.state, reconstructed) {
            self.set_concept_scheduler_state_inner(deck_id, &persisted)?;
        }
        let today = self.timing_today()?.days_elapsed;
        let daily = persisted
            .state
            .daily
            .get(&today)
            .copied()
            .unwrap_or_default();
        let graph = canonical_mcat_demo_graph();
        let kc_memory = self.concept_kc_memory_for_deck(deck_id)?;
        let evidence = persisted.state.readiness_evidence_status(&persisted.config);
        let (evidence_kind, seen_cards, required_seen_cards) = match evidence {
            ReadinessEvidenceStatus::InsufficientEvidence {
                seen_cards,
                required_seen_cards,
            } => (
                scheduler::concept_evidence_status::Kind::Insufficient as i32,
                seen_cards,
                required_seen_cards,
            ),
            ReadinessEvidenceStatus::EnoughEvidence { seen_cards } => (
                scheduler::concept_evidence_status::Kind::Enough as i32,
                seen_cards,
                persisted.config.readiness_min_seen_cards,
            ),
        };

        let nodes = graph
            .components()
            .map(|component| {
                let mastery_state = persisted.state.kcs.get(component);
                let readiness =
                    persisted
                        .state
                        .readiness_score(component, &graph, &persisted.config);
                let fringe = concept_fringe(component, &graph, &persisted);
                scheduler::ConceptGraphNode {
                    id: component.as_str().to_string(),
                    mastery: persisted.state.mastery_for(component, &persisted.config) as f32,
                    fringe: fringe as i32,
                    readiness_score: readiness.score as f32,
                    prerequisite_mastery: readiness.prerequisite_mastery as f32,
                    answered: mastery_state
                        .map(|state| state.answered)
                        .unwrap_or_default(),
                    positive: mastery_state
                        .map(|state| state.positive)
                        .unwrap_or_default(),
                    negative: mastery_state
                        .map(|state| state.negative)
                        .unwrap_or_default(),
                    memory: kc_memory.get(component).copied().unwrap_or(0.0) as f32,
                }
            })
            .collect();

        let edges = canonical_mcat_demo_edges()
            .into_iter()
            .map(|(prerequisite, target)| scheduler::ConceptGraphEdge {
                prerequisite_id: prerequisite.as_str().to_string(),
                target_id: target.as_str().to_string(),
            })
            .collect();

        let recommendations = recommended_topics(&graph, &persisted);
        let session = self
            .state
            .card_queues
            .as_ref()
            .and_then(|queues| queues.concept_session_status());
        let section_scores = [
            McatSection::BioBiochem,
            McatSection::ChemPhys,
            McatSection::PsychSoc,
            McatSection::Cars,
        ]
        .into_iter()
        .map(|section| {
            let mut proto = section_score_to_proto(persisted.state.section_score_status(
                section,
                &graph,
                &persisted.config,
            ));
            let section_memory = section_memory_from_kc_memory(section, &graph, &kc_memory);
            proto.section_has_memory = section_memory.is_some();
            proto.section_memory = section_memory.unwrap_or(0.0) as f32;
            proto
        })
        .collect();
        let has_memory = !kc_memory.is_empty();
        let overall_memory = if has_memory {
            (kc_memory.values().sum::<f64>() / kc_memory.len() as f64) as f32
        } else {
            0.0
        };

        Ok(scheduler::ConceptSchedulerStatusResponse {
            enabled,
            active: enabled,
            evidence: Some(scheduler::ConceptEvidenceStatus {
                kind: evidence_kind,
                seen_cards,
                required_seen_cards,
            }),
            counters: Some(scheduler::ConceptCounters {
                prerequisite_violations_total: persisted.state.prerequisite_violations,
                prerequisite_violations_today: daily.prerequisite_violations,
                daily_positive: daily.positive,
                daily_negative: daily.negative,
                total_seen_cards: persisted.state.total_seen_cards,
            }),
            session,
            graph: Some(scheduler::ConceptGraph {
                nodes,
                edges,
                has_cycle: graph.cycle().is_some(),
            }),
            recommendations,
            section_scores,
            overall_memory,
            has_memory,
        })
    }

    /// Average per-KC card memory (recall probability of already-studied cards)
    /// for a deck, keyed by KC id. KCs with no studied cards are omitted.
    pub(crate) fn concept_kc_memory_for_deck(
        &mut self,
        deck_id: DeckId,
    ) -> Result<BTreeMap<KnowledgeComponentId, f64>> {
        let now = self.timing_today()?.now;
        let mut totals: BTreeMap<KnowledgeComponentId, (f64, u32)> = BTreeMap::new();
        for card_id in self.storage.all_cards_in_single_deck(deck_id)? {
            let card = self.storage.get_card(card_id)?.or_not_found(card_id)?;
            let Some(memory) = self.card_memory(&card, now)? else {
                continue;
            };
            let note = self
                .storage
                .get_note(card.note_id)?
                .or_not_found(card.note_id)?;
            let metadata = CardConceptMetadata::from_tags(&note.tags);
            for component in metadata.target_components {
                let entry = totals.entry(component).or_insert((0.0, 0));
                entry.0 += memory;
                entry.1 += 1;
            }
        }
        Ok(totals
            .into_iter()
            .map(|(component, (sum, count))| {
                (component, if count > 0 { sum / count as f64 } else { 0.0 })
            })
            .collect())
    }

    /// Recall probability of a single card today: FSRS retrievability when a
    /// memory state is available, otherwise a rating-decay fallback. Returns
    /// `None` for cards that have not been studied yet.
    fn card_memory(&self, card: &Card, now: TimestampSecs) -> Result<Option<f64>> {
        if card.reps == 0 {
            return Ok(None);
        }
        let last_review_time = match card.last_review_time {
            Some(time) => time,
            None => match self.storage.time_of_last_review(card.id)? {
                Some(time) => time,
                None => return Ok(None),
            },
        };
        let elapsed_secs = now.elapsed_secs_since(last_review_time).max(0) as u32;

        if let Some(state) = card.memory_state {
            let decay = card.decay.unwrap_or(FSRS5_DEFAULT_DECAY);
            let retrievability = FSRS::new(None).unwrap().current_retrievability_seconds(
                state.into(),
                elapsed_secs,
                decay,
            );
            return Ok(Some(retrievability as f64));
        }

        // Fallback when FSRS memory state is unavailable: decay the last rating's
        // base recall probability relative to the current interval.
        let base = self
            .storage
            .get_revlog_entries_for_card(card.id)?
            .into_iter()
            .rev()
            .find(|entry| entry.has_rating_and_affects_scheduling())
            .and_then(|entry| base_recall_for_button(entry.button_chosen));
        let Some(base) = base else {
            return Ok(None);
        };
        let elapsed_days = elapsed_secs as f64 / 86_400.0;
        let interval_days = card.interval.max(1) as f64;
        let decay_factor = (-elapsed_days / interval_days).exp();
        Ok(Some((base * decay_factor).clamp(0.0, 1.0)))
    }

    fn reconstruct_mcat_demo_state_from_revlogs(
        &self,
        deck_id: DeckId,
        config: &super::concept::ConceptSchedulerConfig,
    ) -> Result<ConceptSchedulerState> {
        let mut state = ConceptSchedulerState::default();
        for card_id in self.storage.all_cards_in_single_deck(deck_id)? {
            let card = self.storage.get_card(card_id)?.or_not_found(card_id)?;
            let note = self
                .storage
                .get_note(card.note_id)?
                .or_not_found(card.note_id)?;
            let metadata = CardConceptMetadata::from_tags(&note.tags);
            if metadata.target_components.is_empty() {
                continue;
            }

            for revlog in self.storage.get_revlog_entries_for_card(card_id)? {
                let Some(evidence) = evidence_from_revlog_button(revlog.button_chosen) else {
                    continue;
                };

                for component in &metadata.target_components {
                    state.record_evidence(component.clone(), evidence, config);
                }
                let irt_item = super::concept::IrtItemMetadata::from_card_metadata(&metadata);
                for section in metadata
                    .sections
                    .iter()
                    .filter_map(|section| super::concept::McatSection::from_tag(section))
                {
                    let disciplines =
                        super::concept::section_disciplines_for_metadata(&metadata, section);
                    state.record_irt_response(section, disciplines, irt_item, evidence);
                }
            }
        }

        Ok(state)
    }
}

fn section_score_to_proto(score: SectionScoreStatus) -> scheduler::ConceptSectionScore {
    scheduler::ConceptSectionScore {
        section: mcat_section_to_proto(score.section) as i32,
        enough_evidence: score.enough_evidence,
        theta: score.theta as f32,
        performance_center: score.performance_center as f32,
        performance_standard_error: score.performance_standard_error as f32,
        performance_lower: score.performance_lower as f32,
        performance_upper: score.performance_upper as f32,
        section_mastery: score.section_mastery as f32,
        coverage: score.coverage as f32,
        readiness_center: score.readiness_center as f32,
        readiness_standard_error: score.readiness_standard_error as f32,
        readiness_lower: score.readiness_lower as f32,
        readiness_upper: score.readiness_upper as f32,
        answered_items: score.answered_items,
        required_items: score.required_items,
        // Populated by the status builder, which has access to card memory.
        section_memory: 0.0,
        section_has_memory: false,
    }
}

fn mcat_section_to_proto(section: McatSection) -> scheduler::McatSection {
    match section {
        McatSection::BioBiochem => scheduler::McatSection::BioBiochem,
        McatSection::ChemPhys => scheduler::McatSection::ChemPhys,
        McatSection::PsychSoc => scheduler::McatSection::PsychSoc,
        McatSection::Cars => scheduler::McatSection::Cars,
    }
}

fn base_recall_for_button(button: u8) -> Option<f64> {
    // Track E fallback: base recall probability implied by the last rating.
    match button {
        1 => Some(0.20),
        2 => Some(0.55),
        3 => Some(0.80),
        4 => Some(0.90),
        _ => None,
    }
}

fn evidence_from_revlog_button(button: u8) -> Option<Evidence> {
    match button {
        1 | 2 => Some(Evidence::Negative),
        3 | 4 => Some(Evidence::Positive),
        _ => None,
    }
}

fn merge_reconstructed_state(
    state: &mut ConceptSchedulerState,
    reconstructed: ConceptSchedulerState,
) -> bool {
    let mut changed = false;

    if reconstructed.total_seen_cards > state.total_seen_cards {
        state.total_seen_cards = reconstructed.total_seen_cards;
        changed = true;
    }

    for (component, reconstructed_mastery) in reconstructed.kcs {
        let should_replace = state
            .kcs
            .get(&component)
            .map(|existing| reconstructed_mastery.answered > existing.answered)
            .unwrap_or(true);
        if should_replace {
            state.kcs.insert(component, reconstructed_mastery);
            changed = true;
        }
    }
    for (section, reconstructed_section) in reconstructed.irt_sections {
        let should_replace = state
            .irt_sections
            .get(&section)
            .map(|existing| {
                reconstructed_section.answered > existing.answered
                    || (reconstructed_section.answered == existing.answered
                        && existing.discipline_counts.is_empty()
                        && !reconstructed_section.discipline_counts.is_empty())
            })
            .unwrap_or(true);
        if should_replace {
            state.irt_sections.insert(section, reconstructed_section);
            changed = true;
        }
    }

    changed
}

fn recommended_topics(
    graph: &KnowledgeGraph,
    persisted: &super::concept::PersistedConceptSchedulerState,
) -> Vec<scheduler::ConceptTopicRecommendation> {
    let mut recommendations: Vec<_> = graph
        .components()
        .filter(|component| {
            persisted
                .state
                .is_outer_fringe(component, graph, &persisted.config)
        })
        .map(|component| {
            let score = persisted
                .state
                .readiness_score(component, graph, &persisted.config);
            scheduler::ConceptTopicRecommendation {
                id: component.as_str().to_string(),
                readiness_score: score.score as f32,
                mastery: score.target_mastery as f32,
                prerequisite_mastery: score.prerequisite_mastery as f32,
                selectable: true,
            }
        })
        .collect();
    recommendations.sort_by(|a, b| {
        b.readiness_score
            .partial_cmp(&a.readiness_score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    recommendations.truncate(5);
    recommendations
}

fn concept_fringe(
    component: &KnowledgeComponentId,
    graph: &KnowledgeGraph,
    persisted: &super::concept::PersistedConceptSchedulerState,
) -> scheduler::ConceptFringe {
    if persisted
        .state
        .is_inner_fringe(component, &persisted.config)
    {
        scheduler::ConceptFringe::Inner
    } else if persisted
        .state
        .is_outer_fringe(component, graph, &persisted.config)
    {
        scheduler::ConceptFringe::Outer
    } else {
        scheduler::ConceptFringe::Locked
    }
}

fn parse_demo_bullet(line: &str) -> Option<(&str, &str)> {
    let line = line.strip_prefix("- **")?;
    let (key, rest) = line.split_once(":** ")?;
    Some((key, rest))
}

fn strip_single_backticks(value: &str) -> &str {
    value
        .strip_prefix('`')
        .and_then(|value| value.strip_suffix('`'))
        .unwrap_or(value)
}

fn backtick_values(value: &str) -> Vec<String> {
    value
        .split('`')
        .enumerate()
        .filter(|(idx, _)| idx % 2 == 1)
        .map(|(_, part)| part.to_string())
        .collect()
}

fn kc_id(value: &str) -> KnowledgeComponentId {
    KnowledgeComponentId::new(value).unwrap()
}

#[derive(Debug, Default)]
struct McatDemoCardBuilder {
    id: String,
    kc: String,
    prerequisites: Vec<String>,
    difficulty: Option<u8>,
    question: Option<String>,
    answers: [Option<String>; 4],
    correct: Option<String>,
    explanation: Option<String>,
    tags: Vec<String>,
}

impl McatDemoCardBuilder {
    fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            ..Default::default()
        }
    }

    fn build(self) -> McatDemoCard {
        McatDemoCard {
            id: self.id,
            kc: self.kc,
            prerequisites: self.prerequisites,
            difficulty: self.difficulty.unwrap(),
            question: self.question.unwrap(),
            answers: self.answers.map(Option::unwrap),
            correct: self.correct.unwrap(),
            explanation: self.explanation.unwrap(),
            tags: self.tags,
        }
    }
}

impl McatDemoCard {
    fn front_html(&self) -> String {
        format!(
            "{}<br><br>A. {}<br>B. {}<br>C. {}<br>D. {}",
            self.question, self.answers[0], self.answers[1], self.answers[2], self.answers[3]
        )
    }

    fn back_html(&self) -> String {
        format!(
            "<b>Correct:</b> {}<br><br>{}",
            self.correct, self.explanation
        )
    }

    #[cfg(test)]
    fn metadata(&self) -> super::concept::CardConceptMetadata {
        super::concept::CardConceptMetadata::from_tags(&self.tags)
    }
}

#[cfg(test)]
mod tests {
    use super::super::concept::CardConceptMetadata;
    use super::super::concept::ConceptMasteryState;
    use super::*;

    #[test]
    fn parses_all_demo_cards_and_tags() {
        let cards = parse_mcat_demo_cards();
        assert_eq!(cards.len(), 50);
        for card in cards {
            assert!(!card.id.is_empty());
            assert!(!card.question.is_empty());
            assert!(!card.explanation.is_empty());
            assert!(matches!(card.correct.as_str(), "A" | "B" | "C" | "D"));
            assert!((1..=5).contains(&card.difficulty));
            let metadata = card.metadata();
            assert_eq!(metadata.target_components.len(), 1);
            assert!(metadata
                .sections
                .iter()
                .any(|section| section == "Bio_Biochem"));
            assert_eq!(metadata.difficulty, Some(card.difficulty));
        }
    }

    #[test]
    fn canonical_demo_graph_has_ten_nodes_and_expected_edges() {
        let graph = canonical_mcat_demo_graph();
        assert_eq!(graph.components().count(), 10);
        assert!(graph.cycle().is_none());
        for (prerequisite, target) in canonical_mcat_demo_edges() {
            assert!(graph
                .prerequisites(&target)
                .unwrap()
                .contains(&prerequisite));
        }
    }

    #[test]
    fn demo_card_prereq_tags_match_canonical_graph_edges() {
        let graph = canonical_mcat_demo_graph();
        for card in parse_mcat_demo_cards() {
            let target = kc_id(&card.kc);
            assert!(graph.contains(&target), "missing target KC {}", card.kc);
            let prerequisites = graph.prerequisites(&target).unwrap();
            let card_prerequisites = card
                .prerequisites
                .iter()
                .map(|prerequisite| kc_id(prerequisite))
                .collect::<std::collections::BTreeSet<_>>();
            assert_eq!(
                &card_prerequisites, prerequisites,
                "{} prereqs do not match canonical graph",
                card.id
            );
            for prerequisite in &card.prerequisites {
                let prerequisite = kc_id(prerequisite);
                assert!(
                    prerequisites.contains(&prerequisite),
                    "{} declares prereq {} that is not a canonical edge",
                    card.id,
                    prerequisite.as_str()
                );
            }
        }
    }

    #[test]
    fn imports_demo_deck_with_fifty_tagged_cards() -> Result<()> {
        let mut col = Collection::new();
        let deck_id = col.import_mcat_demo_deck()?;
        assert_eq!(col.import_mcat_demo_deck()?, deck_id);
        let deck = col.get_deck(deck_id)?.unwrap();
        assert_eq!(deck.human_name(), "MCAT Demo");
        assert!(deck.normal()?.concept_scheduler_enabled);

        let card_ids = col.storage.all_cards_in_single_deck(deck_id)?;
        assert_eq!(card_ids.len(), 50);
        for card_id in card_ids {
            let card = col.storage.get_card(card_id)?.unwrap();
            let note = col.storage.get_note(card.note_id)?.unwrap();
            let metadata = CardConceptMetadata::from_tags(&note.tags);
            assert_eq!(metadata.target_components.len(), 1);
        }

        Ok(())
    }

    #[test]
    fn demo_algorithm_updates_percentages_and_status() -> Result<()> {
        let mut col = Collection::new();
        let deck_id = col.import_mcat_demo_deck()?;
        for _ in 0..5 {
            col.answer_good();
        }

        let persisted = col.get_concept_scheduler_state(deck_id);
        assert_eq!(persisted.state.total_seen_cards, 5);
        assert!(
            persisted
                .state
                .mastery_for(&kc_id("Bio::DNA"), &persisted.config)
                > 0.5
        );

        let status = col.concept_scheduler_status(deck_id)?;
        assert!(status.enabled);
        let graph = status.graph.unwrap();
        assert_eq!(graph.nodes.len(), 10);
        let dna = graph
            .nodes
            .iter()
            .find(|node| node.id == "Bio::DNA")
            .unwrap();
        assert_eq!(dna.answered, 5);
        assert!(dna.mastery > 0.5);
        assert_eq!(status.counters.unwrap().daily_positive, 5);

        Ok(())
    }

    #[test]
    fn demo_status_reports_memory_after_reviews() -> Result<()> {
        let mut col = Collection::new();
        let deck_id = col.import_mcat_demo_deck()?;
        for _ in 0..5 {
            col.answer_good();
        }

        let status = col.concept_scheduler_status(deck_id)?;
        assert!(status.has_memory);
        // Cards just answered Good => high recall probability, but still a valid
        // probability.
        assert!(status.overall_memory > 0.5);
        assert!(status.overall_memory <= 1.0);

        let graph = status.graph.unwrap();
        let dna = graph
            .nodes
            .iter()
            .find(|node| node.id == "Bio::DNA")
            .unwrap();
        // The studied KC has a recall estimate from its studied cards.
        assert!(dna.answered > 0);
        assert!(dna.memory > 0.0);

        // A KC with no studied cards stays at 0 memory so the UI can hide it.
        let untouched = graph
            .nodes
            .iter()
            .find(|node| node.answered == 0)
            .unwrap();
        assert_eq!(untouched.memory, 0.0);

        Ok(())
    }

    #[test]
    fn demo_status_marks_outer_fringe_after_foundation_mastery() -> Result<()> {
        let mut col = Collection::new();
        let deck_id = col.import_mcat_demo_deck()?;
        let mut persisted = col.get_concept_scheduler_state(deck_id);
        persisted.state.kcs.insert(
            kc_id("Bio::DNA"),
            ConceptMasteryState {
                mastery: 0.9,
                answered: 3,
                positive: 3,
                negative: 0,
            },
        );
        persisted.state.total_seen_cards = 10;
        col.set_concept_scheduler_state(deck_id, &persisted, false)?;

        let status = col.concept_scheduler_status(deck_id)?;
        let graph = status.graph.unwrap();
        let genetics = graph
            .nodes
            .iter()
            .find(|node| node.id == "Bio::Genetics")
            .unwrap();
        assert_eq!(genetics.fringe, scheduler::ConceptFringe::Outer as i32);

        Ok(())
    }

    #[test]
    fn demo_status_reconstructs_mastery_from_revlog_history() -> Result<()> {
        let mut col = Collection::new();
        let deck_id = col.import_mcat_demo_deck()?;
        for _ in 0..3 {
            col.answer_good();
        }

        let mut persisted = col.get_concept_scheduler_state(deck_id);
        persisted.state = Default::default();
        col.set_concept_scheduler_state(deck_id, &persisted, false)?;

        let status = col.concept_scheduler_status(deck_id)?;
        let graph = status.graph.unwrap();
        let dna = graph
            .nodes
            .iter()
            .find(|node| node.id == "Bio::DNA")
            .unwrap();
        assert_eq!(dna.answered, 3);
        assert!(dna.mastery > 0.5);

        Ok(())
    }

    #[test]
    fn demo_status_reconstructs_nodes_even_when_total_counter_is_stale() -> Result<()> {
        let mut col = Collection::new();
        let deck_id = col.import_mcat_demo_deck()?;
        for _ in 0..2 {
            col.answer_good();
        }

        let mut persisted = col.get_concept_scheduler_state(deck_id);
        persisted.state.kcs.clear();
        persisted.state.total_seen_cards = 99;
        col.set_concept_scheduler_state(deck_id, &persisted, false)?;

        let status = col.concept_scheduler_status(deck_id)?;
        let graph = status.graph.unwrap();
        let dna = graph
            .nodes
            .iter()
            .find(|node| node.id == "Bio::DNA")
            .unwrap();
        assert_eq!(dna.answered, 2);

        Ok(())
    }

    #[test]
    fn demo_status_backfills_stale_irt_coverage_buckets() -> Result<()> {
        let mut col = Collection::new();
        let deck_id = col.import_mcat_demo_deck()?;
        for _ in 0..5 {
            col.answer_good();
        }

        let mut persisted = col.get_concept_scheduler_state(deck_id);
        let section = super::super::concept::McatSection::BioBiochem;
        let answered = persisted
            .state
            .irt_sections
            .get(&section)
            .map(|state| state.answered)
            .unwrap_or_default();
        persisted
            .state
            .irt_sections
            .get_mut(&section)
            .unwrap()
            .discipline_counts
            .clear();
        col.set_concept_scheduler_state(deck_id, &persisted, false)?;

        let status = col.concept_scheduler_status(deck_id)?;
        let bio_biochem = status
            .section_scores
            .iter()
            .find(|score| score.section == scheduler::McatSection::BioBiochem as i32)
            .unwrap();

        assert_eq!(bio_biochem.answered_items, answered);
        assert!(bio_biochem.coverage > 0.0);

        Ok(())
    }
}
