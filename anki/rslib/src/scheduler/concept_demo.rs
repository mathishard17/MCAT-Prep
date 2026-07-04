// Copyright: Ankitects Pty Ltd and contributors
// License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html

use std::collections::BTreeMap;

use anki_proto::scheduler;
use fsrs::FSRS;
use fsrs::FSRS5_DEFAULT_DECAY;

/// Forward horizon (seconds) at which "memory" recall is evaluated. One day is long
/// enough that a low-stability "Again" reads as low recall while a "Good" stays high,
/// instead of both reading ~1.0 immediately after review.
const MEMORY_HORIZON_SECS: u32 = 86_400;

use super::concept::probability_at_least;
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
pub(crate) const MCAT_FULL_DECK_NAME: &str = "MCAT";

#[cfg(test)]
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
#[cfg(test)]
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

const GENERATED_CARD_MD: [&str; 11] = [
    include_str!("../../../added features/generated-cards-biology.md"),
    include_str!("../../../added features/generated-cards-biochemistry.md"),
    include_str!("../../../added features/generated-cards-general-chemistry.md"),
    include_str!("../../../added features/generated-cards-organic-chemistry.md"),
    include_str!("../../../added features/generated-cards-physics.md"),
    include_str!("../../../added features/generated-cards-psych-soc.md"),
    // CARS reading-skill cards (humanities + social-science passages) — give the CARS
    // section real scored content instead of the ~125 prior.
    include_str!("../../../added features/generated-cards-cars.md"),
    include_str!("../../../added features/generated-cards-cars-social.md"),
    // Orphaned reading passages converted to scored cards.
    include_str!("../../../added features/generated-cards-passages-bio-biochem.md"),
    include_str!("../../../added features/generated-cards-passages-chem-phys.md"),
    include_str!("../../../added features/generated-cards-passages-psych-soc.md"),
];

/// Authored lesson pages (one block per KC). `lessons.md` (schema + demo stubs)
/// is parsed first so the diagram-rich discipline files override it for the demo
/// KCs (they reuse the same approved prose and add the inline diagrams).
const LESSON_MD: [&str; 7] = [
    include_str!("../../../added features/lessons.md"),
    include_str!("../../../added features/lessons-biology.md"),
    include_str!("../../../added features/lessons-biochemistry.md"),
    include_str!("../../../added features/lessons-general-chemistry.md"),
    include_str!("../../../added features/lessons-organic-chemistry.md"),
    include_str!("../../../added features/lessons-physics.md"),
    include_str!("../../../added features/lessons-psych-soc.md"),
];

pub(crate) fn parse_mcat_demo_cards() -> Vec<McatDemoCard> {
    parse_cards_md(MCAT_DEMO_CARDS_MD)
}

fn parse_generated_mcat_cards() -> Vec<McatDemoCard> {
    let mut cards = Vec::new();
    for md in GENERATED_CARD_MD {
        cards.extend(parse_cards_md(md));
    }
    cards
}

fn parse_cards_md(md: &str) -> Vec<McatDemoCard> {
    let mut cards = Vec::new();
    let mut current: Option<McatDemoCardBuilder> = None;

    for line in md.lines() {
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

#[derive(Default, Clone)]
struct ParsedLesson {
    kc: String,
    title: String,
    section: String,
    source: String,
    review_status: String,
    overview: String,
    key_concepts: Vec<String>,
    prerequisite_reminder: String,
    worked_example: String,
    common_misconception: String,
    first_retrieval_prompt: String,
    diagram: String,
    diagram_mermaid: String,
    related_kcs: Vec<String>,
}

/// Like `parse_demo_bullet` but tolerates an empty value (e.g. `- **Key Concepts:**`
/// whose items follow as an indented list), used by the multi-line lesson format.
fn parse_lesson_bullet(line: &str) -> Option<(&str, &str)> {
    let rest = line.strip_prefix("- **")?;
    let (key, after) = rest.split_once(":**")?;
    Some((key.trim(), after.trim()))
}

/// Parse the Markdown lesson format (`### LESSON-...` blocks with `- **Key:** value`
/// bullets, multi-line prose continuations, and a nested `Key Concepts` list).
fn parse_lessons_md(md: &str) -> Vec<ParsedLesson> {
    let mut lessons = Vec::new();
    let mut current: Option<ParsedLesson> = None;
    let mut key: Option<String> = None;
    let mut in_mermaid = false;
    let mut in_diagram_html = false;

    for line in md.lines() {
        // Inside an inline <figure>...</figure> SVG diagram (the diagram file inlined
        // into the lesson): capture verbatim with newlines preserved, so it ships
        // inside the lesson string and inlines via innerHTML in the reviewer panel.
        if in_diagram_html {
            if let Some(lesson) = current.as_mut() {
                lesson.diagram.push('\n');
                lesson.diagram.push_str(line);
            }
            if line.contains("</figure>") {
                in_diagram_html = false;
                key = None;
            }
            continue;
        }

        // Inside a ```mermaid fence: capture the body verbatim (newlines matter for
        // Mermaid) until the closing fence, then stop continuing the Diagram field.
        if in_mermaid {
            if line.trim_start().starts_with("```") {
                in_mermaid = false;
                key = None;
            } else if let Some(lesson) = current.as_mut() {
                if !lesson.diagram_mermaid.is_empty() {
                    lesson.diagram_mermaid.push('\n');
                }
                lesson.diagram_mermaid.push_str(line);
            }
            continue;
        }

        if line.starts_with("### ") {
            if let Some(lesson) = current.take() {
                if !lesson.kc.is_empty() {
                    lessons.push(lesson);
                }
            }
            current = Some(ParsedLesson::default());
            key = None;
            continue;
        }
        let Some(lesson) = current.as_mut() else {
            continue;
        };

        // Start of an inline SVG diagram block: capture the whole <figure>...</figure>
        // as the diagram HTML, overriding any preceding caption text.
        let diagram_open = line.trim_start();
        if diagram_open.starts_with("<figure") || diagram_open.starts_with("<svg") {
            lesson.diagram.clear();
            lesson.diagram.push_str(line);
            if line.contains("</figure>") {
                key = None;
            } else {
                in_diagram_html = true;
            }
            continue;
        }

        if let Some((bullet_key, value)) = parse_lesson_bullet(line) {
            key = Some(bullet_key.to_string());
            match bullet_key {
                "KC" => lesson.kc = strip_single_backticks(value).to_string(),
                "Title" => lesson.title = value.to_string(),
                "Section" => lesson.section = strip_single_backticks(value).to_string(),
                "Source" => lesson.source = value.trim().to_lowercase(),
                "Review Status" => lesson.review_status = value.trim().to_lowercase(),
                "Overview" => lesson.overview = value.to_string(),
                "Key Concepts" => {
                    if !value.is_empty() {
                        lesson.key_concepts.push(value.to_string());
                    }
                }
                "Prerequisite Reminder" => lesson.prerequisite_reminder = value.to_string(),
                "Worked Example" => lesson.worked_example = value.to_string(),
                "Common Misconception" => lesson.common_misconception = value.to_string(),
                "First Retrieval Prompt" => lesson.first_retrieval_prompt = value.to_string(),
                "Diagram" => {
                    // Diagrams are either an inline <figure><svg> block (captured
                    // above) or, for back-compat, an image ref "![alt](path)". A
                    // Mermaid flowchart's caption bullet is ignored here (its body is
                    // captured from the ```mermaid fence). Never clobber an inline SVG.
                    let v = value.trim();
                    if v.starts_with("![") && !lesson.diagram.starts_with('<') {
                        lesson.diagram = v.to_string();
                    }
                }
                "Related KCs" => lesson.related_kcs.extend(backtick_values(value)),
                _ => key = None,
            }
            continue;
        }

        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        // Opening of a Mermaid flowchart fence (its body is captured above).
        if trimmed.starts_with("```mermaid") {
            in_mermaid = true;
            continue;
        }
        match key.as_deref() {
            Some("Key Concepts") => {
                if let Some(item) = trimmed.strip_prefix("- ") {
                    lesson.key_concepts.push(item.trim().to_string());
                }
            }
            Some("Related KCs") => lesson.related_kcs.extend(backtick_values(line)),
            Some(prose_key) => {
                let field = match prose_key {
                    "Title" => Some(&mut lesson.title),
                    "Overview" => Some(&mut lesson.overview),
                    "Prerequisite Reminder" => Some(&mut lesson.prerequisite_reminder),
                    "Worked Example" => Some(&mut lesson.worked_example),
                    "Common Misconception" => Some(&mut lesson.common_misconception),
                    "First Retrieval Prompt" => Some(&mut lesson.first_retrieval_prompt),
                    // A Diagram is captured as an inline SVG block or an image ref;
                    // wrapped caption lines are never appended to it.
                    _ => None,
                };
                if let Some(field) = field {
                    if !field.is_empty() {
                        field.push(' ');
                    }
                    field.push_str(trimmed);
                }
            }
            None => {}
        }
    }
    if let Some(lesson) = current.take() {
        if !lesson.kc.is_empty() {
            lessons.push(lesson);
        }
    }
    lessons
}

fn lessons_by_kc() -> &'static std::collections::HashMap<String, ParsedLesson> {
    static LESSONS: std::sync::OnceLock<std::collections::HashMap<String, ParsedLesson>> =
        std::sync::OnceLock::new();
    LESSONS.get_or_init(|| {
        let mut map = std::collections::HashMap::new();
        for md in LESSON_MD {
            for lesson in parse_lessons_md(md) {
                map.insert(lesson.kc.clone(), lesson);
            }
        }
        map
    })
}

/// Display gate: only human-`authored` lessons render. AI-generated lessons stay
/// hidden until the bring-your-own-key + evaluation work (Track H) exists.
fn lesson_passes_gate(lesson: &ParsedLesson) -> bool {
    lesson.source == "authored"
}

fn lesson_to_proto(lesson: &ParsedLesson) -> scheduler::ConceptLesson {
    scheduler::ConceptLesson {
        kc: lesson.kc.clone(),
        title: lesson.title.clone(),
        section: lesson.section.clone(),
        overview: lesson.overview.clone(),
        key_concepts: lesson.key_concepts.clone(),
        prerequisite_reminder: lesson.prerequisite_reminder.clone(),
        worked_example: lesson.worked_example.clone(),
        common_misconception: lesson.common_misconception.clone(),
        first_retrieval_prompt: lesson.first_retrieval_prompt.clone(),
        related_kcs: lesson.related_kcs.clone(),
        diagram: lesson.diagram.clone(),
        diagram_mermaid: lesson.diagram_mermaid.clone(),
        exists: true,
    }
}

/// Resolve the (ungated) lesson for a KC id. `None` when no lesson exists or it is
/// gated (AI-generated).
pub(crate) fn concept_lesson_for(kc: &str) -> Option<scheduler::ConceptLesson> {
    let lesson = lessons_by_kc().get(kc)?;
    lesson_passes_gate(lesson).then(|| lesson_to_proto(lesson))
}

#[cfg(test)]
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

#[cfg(test)]
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

    /// Import the full generated MCAT library (at least one card per KC across all
    /// disciplines) into a separate "MCAT" deck. Idempotent: only adds cards not
    /// already present (dedup by the question field), and does not change the
    /// current deck.
    pub(crate) fn import_mcat_full_deck(&mut self) -> Result<DeckId> {
        let mut deck = self.get_or_create_normal_deck(MCAT_FULL_DECK_NAME)?;
        deck.normal_mut()?.concept_scheduler_enabled = true;
        self.add_or_update_deck(&mut deck)?;

        let mut persisted = self.get_concept_scheduler_state(deck.id);
        persisted.config.readiness_min_seen_cards = 10;
        self.set_concept_scheduler_state(deck.id, &persisted, false)?;

        let mut existing_fronts = std::collections::HashSet::new();
        for card_id in self.storage.all_cards_in_single_deck(deck.id)? {
            let card = self.storage.get_card(card_id)?.or_not_found(card_id)?;
            if let Some(note) = self.storage.get_note(card.note_id)? {
                if let Some(front) = note.fields().first() {
                    existing_fronts.insert(front.clone());
                }
            }
        }

        let nt = self.get_notetype_by_name("Basic")?.unwrap();
        for card in parse_generated_mcat_cards() {
            let front = card.front_html();
            if existing_fronts.contains(&front) {
                continue;
            }
            let mut note = nt.new_note();
            note.set_field(0, front.clone())?;
            note.set_field(1, card.back_html())?;
            note.tags = card.tags;
            self.add_note(&mut note, deck.id)?;
            existing_fronts.insert(front);
        }

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
        let graph = self.concept_graph_for_deck(deck_id)?;
        // Current recall (for the graph/memory display) plus recall projected to the
        // exam date (for readiness). With no exam set, both use "now".
        let kc_memory = self.concept_kc_memory_for_deck(deck_id)?;
        let kc_memory_exam = match persisted.state.exam_timestamp {
            Some(secs) => self.concept_kc_memory_for_deck_at(deck_id, TimestampSecs(secs))?,
            None => kc_memory.clone(),
        };
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

        let recommendations = recommended_topics(&graph, &persisted);
        let recommended_ids: std::collections::HashSet<&str> =
            recommendations.iter().map(|rec| rec.id.as_str()).collect();

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
                    recommended: recommended_ids.contains(component.as_str()),
                }
            })
            .collect();

        let mut edges: Vec<scheduler::ConceptGraphEdge> = Vec::new();
        for target in graph.components() {
            if let Some(prerequisites) = graph.prerequisites(target) {
                for prerequisite in prerequisites {
                    edges.push(scheduler::ConceptGraphEdge {
                        prerequisite_id: prerequisite.as_str().to_string(),
                        target_id: target.as_str().to_string(),
                    });
                }
            }
        }

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
            // Readiness uses exam-projected retention; the displayed section_memory
            // stays as current recall so the sidebar shows "what you remember now".
            let section_memory_exam = section_memory_from_kc_memory(section, &graph, &kc_memory_exam);
            let mut proto = section_score_to_proto(persisted.state.section_score_status(
                section,
                &graph,
                &persisted.config,
                section_memory_exam,
            ));
            let section_memory = section_memory_from_kc_memory(section, &graph, &kc_memory);
            proto.section_has_memory = section_memory.is_some();
            proto.section_memory = section_memory.unwrap_or(0.0) as f32;
            proto
        })
        .collect::<Vec<_>>();
        // Projected MCAT total = sum of the four section readiness estimates (~472-528).
        let has_projection = section_scores.iter().any(|score| score.answered_items > 0);
        let projected_total: f32 = section_scores.iter().map(|score| score.readiness_center).sum();
        // Combine section variances for the total band (independent-error approx),
        // floored near the AAMC total confidence band (~+/-2 points => SE ~1).
        let total_se: f32 = section_scores
            .iter()
            .map(|score| score.readiness_standard_error * score.readiness_standard_error)
            .sum::<f32>()
            .sqrt()
            .max(1.0);
        let projected_total_lower: f32 = (projected_total - 1.96 * total_se).clamp(472.0, 528.0);
        let projected_total_upper: f32 = (projected_total + 1.96 * total_se).clamp(472.0, 528.0);
        // Probability of reaching the user's target total by the exam date.
        let target = persisted.state.target_total_score;
        let has_target = target.is_some();
        let probability_hit_target = match target {
            Some(t) => {
                probability_at_least(projected_total as f64, total_se as f64, t as f64) as f32
            }
            None => 0.0,
        };
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
            projected_total,
            projected_total_lower,
            projected_total_upper,
            has_projection,
            exam_timestamp: persisted.state.exam_timestamp.unwrap_or(0),
            target_total_score: persisted.state.target_total_score.unwrap_or(0),
            probability_hit_target,
            has_target,
        })
    }

    /// Build the concept knowledge graph for a deck from its cards' `KC::`/`Prereq::`
    /// tags, so the scheduler reflects the deck's actual tagged content instead of a
    /// fixed demo graph. KCs with no cards in the deck are not included.
    pub(crate) fn concept_graph_for_deck(&self, deck_id: DeckId) -> Result<KnowledgeGraph> {
        let mut metadata = Vec::new();
        for card_id in self.storage.all_cards_in_single_deck(deck_id)? {
            let card = self.storage.get_card(card_id)?.or_not_found(card_id)?;
            let note = self
                .storage
                .get_note(card.note_id)?
                .or_not_found(card.note_id)?;
            let card_metadata = CardConceptMetadata::from_tags(&note.tags);
            if card_metadata.target_components.is_empty() {
                continue;
            }
            metadata.push(card_metadata);
        }
        Ok(KnowledgeGraph::from_card_metadata(&metadata))
    }

    /// Average per-KC card memory (recall probability of already-studied cards)
    /// for a deck, keyed by KC id. KCs with no studied cards are omitted.
    pub(crate) fn concept_kc_memory_for_deck(
        &mut self,
        deck_id: DeckId,
    ) -> Result<BTreeMap<KnowledgeComponentId, f64>> {
        let now = self.timing_today()?.now;
        self.concept_kc_memory_for_deck_at(deck_id, now)
    }

    /// Like `concept_kc_memory_for_deck`, but evaluates recall at `at_time` (e.g.
    /// the exam date) instead of now, so readiness can be projected to test day.
    pub(crate) fn concept_kc_memory_for_deck_at(
        &mut self,
        deck_id: DeckId,
        at_time: TimestampSecs,
    ) -> Result<BTreeMap<KnowledgeComponentId, f64>> {
        let mut totals: BTreeMap<KnowledgeComponentId, (f64, u32)> = BTreeMap::new();
        for card_id in self.storage.all_cards_in_single_deck(deck_id)? {
            let card = self.storage.get_card(card_id)?.or_not_found(card_id)?;
            let Some(memory) = self.card_memory(&card, at_time)? else {
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

    /// Record (or clear) the user's chosen next topic for a concept-scheduler
    /// deck. A `Some(topic)` is accepted only when it is currently an outer-fringe
    /// KC (prerequisites satisfied, not yet mastered); otherwise it is ignored.
    /// `None` clears the choice and returns to automatic (readiness-sorted) pick.
    pub(crate) fn set_concept_selected_topic(
        &mut self,
        deck_id: DeckId,
        topic: Option<KnowledgeComponentId>,
    ) -> Result<()> {
        let mut persisted = self.get_concept_scheduler_state(deck_id);
        if let Some(topic) = &topic {
            let graph = self.concept_graph_for_deck(deck_id)?;
            if !persisted
                .state
                .is_outer_fringe(topic, &graph, &persisted.config)
            {
                return Ok(());
            }
        }
        persisted.state.selected_topic = topic;
        self.set_concept_scheduler_state(deck_id, &persisted, false)?;
        // Rebuild the queue so the choice takes effect on the next fetch.
        self.clear_study_queues();
        Ok(())
    }

    /// Store the learner's exam date (epoch seconds) and/or target total score.
    /// `None` clears a field. The exam date drives the retention projection and the
    /// target drives the probability-of-hitting-target; out-of-range targets are
    /// ignored. Does not rebuild the queue (these only affect readiness reporting).
    pub(crate) fn set_concept_exam_settings(
        &mut self,
        deck_id: DeckId,
        exam_timestamp: Option<i64>,
        target_total_score: Option<u32>,
    ) -> Result<()> {
        let mut persisted = self.get_concept_scheduler_state(deck_id);
        persisted.state.exam_timestamp = exam_timestamp;
        persisted.state.target_total_score =
            target_total_score.filter(|score| (472..=528).contains(score));
        self.set_concept_scheduler_state(deck_id, &persisted, false)?;
        Ok(())
    }

    /// Durable recall probability of a single card: FSRS retrievability when a memory
    /// state is available, otherwise a rating-decay fallback. Returns `None` for cards
    /// not yet studied.
    ///
    /// Recall is evaluated at a forward HORIZON (at least one day past the last
    /// review), never at the post-review instant. Right after a review the elapsed
    /// time is ≈0 and FSRS retrievability is ~1.0 regardless of the rating, which made
    /// repeatedly pressing "Again" appear to RAISE memory. Projecting forward instead
    /// reflects the low stability an "Again" leaves behind (low recall in a day) vs the
    /// higher stability of a "Good"/"Easy".
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
            // Evaluate FSRS retrievability at a forward HORIZON (≥ 1 day), not at the
            // post-review instant. At t≈0 retrievability is ~1.0 for ANY rating, so
            // repeated "Again" appeared to raise memory; projecting forward instead
            // exposes the low stability an "Again" leaves behind. Once the real elapsed
            // time exceeds the horizon we use it (that is genuine current recall).
            let horizon_secs = elapsed_secs.max(MEMORY_HORIZON_SECS);
            let decay = card.decay.unwrap_or(FSRS5_DEFAULT_DECAY);
            let retrievability = FSRS::new(None).unwrap().current_retrievability_seconds(
                state.into(),
                horizon_secs,
                decay,
            );
            return Ok(Some(retrievability as f64));
        }

        // Fallback when FSRS memory state is unavailable: decay the last rating's base
        // recall probability (already rating-dependent — Again 0.20 vs Good 0.80, so no
        // t≈0 pathology) relative to the current interval, at the real elapsed time.
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
                state.note_card_answered();
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

/// The three MCAT super-sections the picker balances across.
const RECOMMENDATION_SECTIONS: [&str; 3] = ["Bio_Biochem", "Chem_Phys", "Psych_Soc"];
/// Suggested next topics: 2 in the current section + 1 in each other section.
const RECOMMENDATION_TARGET: usize = 4;

/// Map a KC id to its MCAT super-section by discipline prefix (mirrors the JS
/// `sectionOf` in `reviewer.py`).
fn recommendation_section(id: &str) -> &'static str {
    match id.split("::").next().unwrap_or("") {
        "Bio" | "Biochem" => "Bio_Biochem",
        "GenChem" | "Orgo" | "Physics" => "Chem_Phys",
        "PsychSoc" => "Psych_Soc",
        _ => "Other",
    }
}

/// Which section the learner is "currently in": the selected topic's section,
/// else the section with the most answered KCs, else `None` (cold start → the
/// recommendation spreads one per section for a baseline).
fn current_recommendation_section(
    persisted: &super::concept::PersistedConceptSchedulerState,
) -> Option<&'static str> {
    if let Some(topic) = &persisted.state.selected_topic {
        let section = recommendation_section(topic.as_str());
        if section != "Other" {
            return Some(section);
        }
    }
    let mut answered_by_section: std::collections::HashMap<&'static str, u32> =
        std::collections::HashMap::new();
    for (kc, state) in &persisted.state.kcs {
        let section = recommendation_section(kc.as_str());
        if section != "Other" {
            *answered_by_section.entry(section).or_default() += state.answered;
        }
    }
    answered_by_section
        .into_iter()
        .filter(|(_, answered)| *answered > 0)
        .max_by_key(|(_, answered)| *answered)
        .map(|(section, _)| section)
}

/// Suggested next topics from the outer fringe, balanced across sections:
/// **2 in the current section + 1 in each of the other two** (readiness-sorted
/// within each), with unmet slots backfilled by global readiness. Falls back to
/// one-per-section breadth when there is no current section yet.
fn recommended_topics(
    graph: &KnowledgeGraph,
    persisted: &super::concept::PersistedConceptSchedulerState,
) -> Vec<scheduler::ConceptTopicRecommendation> {
    let mut ranked: Vec<scheduler::ConceptTopicRecommendation> = graph
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
    ranked.sort_by(|a, b| {
        b.readiness_score
            .partial_cmp(&a.readiness_score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    let current = current_recommendation_section(persisted);
    let quota = |section: &str| -> usize {
        if Some(section) == current {
            2
        } else {
            1
        }
    };

    let mut picked: Vec<scheduler::ConceptTopicRecommendation> = Vec::new();
    let mut used: std::collections::HashSet<String> = std::collections::HashSet::new();
    // Fill each section's quota in readiness order.
    for rec in &ranked {
        let section = recommendation_section(&rec.id);
        if !RECOMMENDATION_SECTIONS.contains(&section) {
            continue;
        }
        let taken = picked
            .iter()
            .filter(|r| recommendation_section(&r.id) == section)
            .count();
        if taken < quota(section) && used.insert(rec.id.clone()) {
            picked.push(rec.clone());
        }
    }
    // Backfill any unmet slots (a section had too few ready topics) by readiness.
    for rec in &ranked {
        if picked.len() >= RECOMMENDATION_TARGET {
            break;
        }
        if used.insert(rec.id.clone()) {
            picked.push(rec.clone());
        }
    }
    picked.sort_by(|a, b| {
        b.readiness_score
            .partial_cmp(&a.readiness_score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    picked
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

#[cfg(test)]
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
    fn deck_concept_graph_is_built_from_card_metadata() -> Result<()> {
        let mut col = Collection::new();
        let deck_id = col.import_mcat_demo_deck()?;

        // The demo deck's card tags reproduce the canonical 10-node graph.
        let graph = col.concept_graph_for_deck(deck_id)?;
        assert_eq!(graph.components().count(), 10);
        assert!(graph.cycle().is_none());
        for (prerequisite, target) in canonical_mcat_demo_edges() {
            assert!(
                graph.prerequisites(&target).unwrap().contains(&prerequisite),
                "missing canonical edge {} -> {}",
                prerequisite.as_str(),
                target.as_str()
            );
        }

        // Adding a card with brand-new KCs grows the graph past the 10 demo nodes,
        // and the status read model surfaces them (previously hardcoded to 10).
        let nt = col.get_notetype_by_name("Basic")?.unwrap();
        let mut note = nt.new_note();
        note.set_field(0, "What kind of quantity is velocity?")?;
        note.set_field(1, "A vector")?;
        note.tags = vec![
            "KC::Physics::Kinematics".into(),
            "Prereq::Physics::Vectors".into(),
            "MCAT::Chem_Phys".into(),
            "Difficulty::2".into(),
        ];
        col.add_note(&mut note, deck_id)?;

        let graph = col.concept_graph_for_deck(deck_id)?;
        assert!(graph.contains(&kc_id("Physics::Kinematics")));
        assert!(graph
            .prerequisites(&kc_id("Physics::Kinematics"))
            .unwrap()
            .contains(&kc_id("Physics::Vectors")));

        let status = col.concept_scheduler_status(deck_id)?;
        let nodes = status.graph.unwrap().nodes;
        assert!(nodes.len() > 10);
        assert!(nodes.iter().any(|node| node.id == "Physics::Kinematics"));
        Ok(())
    }

    #[test]
    fn import_mcat_full_deck_covers_all_disciplines_and_is_idempotent() -> Result<()> {
        let mut col = Collection::new();
        let deck_id = col.import_mcat_full_deck()?;

        let graph = col.concept_graph_for_deck(deck_id)?;
        // the generated library spans well beyond the 10 demo KCs
        assert!(graph.components().count() > 100);
        let disciplines: std::collections::BTreeSet<String> = graph
            .components()
            .filter_map(|c| c.as_str().split("::").next().map(ToString::to_string))
            .collect();
        for discipline in ["Bio", "Biochem", "GenChem", "Orgo", "Physics", "PsychSoc", "CARS"] {
            assert!(
                disciplines.contains(discipline),
                "missing discipline {discipline}"
            );
        }

        // re-import adds no duplicate cards
        let count_before = col.storage.all_cards_in_single_deck(deck_id)?.len();
        col.import_mcat_full_deck()?;
        let count_after = col.storage.all_cards_in_single_deck(deck_id)?.len();
        assert_eq!(count_before, count_after);
        Ok(())
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
    fn recommends_across_sections_from_outer_fringe() {
        let mut graph = KnowledgeGraph::default();
        for id in [
            "Bio::A", "Bio::B", "Biochem::C", "GenChem::D", "Physics::E", "PsychSoc::F",
        ] {
            graph.add_component(kc_id(id));
        }
        // Cold start (no selected topic, nothing answered): breadth across sections.
        let persisted = super::super::concept::PersistedConceptSchedulerState::default();
        let recs = recommended_topics(&graph, &persisted);
        assert!(recs.len() <= RECOMMENDATION_TARGET);
        let sections: std::collections::HashSet<&str> =
            recs.iter().map(|r| recommendation_section(&r.id)).collect();
        assert!(sections.contains("Bio_Biochem"));
        assert!(sections.contains("Chem_Phys"));
        assert!(sections.contains("Psych_Soc"));
    }

    #[test]
    fn recommends_two_in_current_section() {
        let mut graph = KnowledgeGraph::default();
        for id in ["Bio::A", "Bio::B", "Bio::C", "GenChem::D", "PsychSoc::E"] {
            graph.add_component(kc_id(id));
        }
        let mut persisted = super::super::concept::PersistedConceptSchedulerState::default();
        persisted.state.selected_topic = Some(kc_id("Bio::A"));
        let recs = recommended_topics(&graph, &persisted);
        let bio = recs
            .iter()
            .filter(|r| recommendation_section(&r.id) == "Bio_Biochem")
            .count();
        assert_eq!(bio, 2);
        assert!(recs
            .iter()
            .any(|r| recommendation_section(&r.id) == "Chem_Phys"));
        assert!(recs
            .iter()
            .any(|r| recommendation_section(&r.id) == "Psych_Soc"));
    }

    #[test]
    fn resolves_authored_lessons_including_needs_review() {
        let dna = concept_lesson_for("Bio::DNA").expect("Bio::DNA lesson");
        assert!(dna.exists);
        assert!(!dna.overview.is_empty());
        assert!(!dna.key_concepts.is_empty());
        // A demo KC (also present in lessons.md) must resolve to the discipline
        // file's version, which carries the inline SVG diagram.
        assert!(
            dna.diagram.contains("<svg"),
            "Bio::DNA lost its inline diagram: {:?}",
            dna.diagram
        );
        // Authored + needs_review still renders (gate is source == authored).
        assert!(concept_lesson_for("Biochem::Peptides_and_Proteins").is_some());
        // Unknown KC resolves to nothing.
        assert!(concept_lesson_for("Bogus::Nope").is_none());
    }

    #[test]
    fn lessons_capture_mermaid_without_clobbering_image_or_leaking() {
        // Mermaid-only lesson: the flowchart source lands in `diagram_mermaid` with
        // its newlines intact, and the fence body must not swallow the following
        // lesson's Markdown headings/fence.
        let gng = concept_lesson_for("Biochem::Gluconeogenesis").expect("gluconeogenesis");
        assert!(gng.diagram_mermaid.contains("flowchart"));
        assert!(gng.diagram_mermaid.contains('\n'), "mermaid must keep newlines");
        assert!(!gng.diagram_mermaid.contains("##"), "leaked a heading");
        assert!(!gng.diagram_mermaid.contains("```"), "leaked a fence marker");
        assert!(!gng.diagram.contains("flowchart"), "mermaid leaked into caption");

        // Image + mermaid lesson: the inline SVG stays in `diagram` (never clobbered
        // by the second Diagram caption bullet), the flowchart is captured separately.
        let ek = concept_lesson_for("Biochem::Enzyme_Kinetics").expect("enzyme kinetics");
        assert!(ek.diagram.contains("<svg"), "SVG diagram lost: {:?}", ek.diagram);
        assert!(ek.diagram.contains("</svg>"), "SVG diagram truncated: {:?}", ek.diagram);
        assert!(ek.diagram_mermaid.contains("flowchart"));
    }

    #[test]
    fn lesson_library_covers_all_disciplines() {
        let map = lessons_by_kc();
        assert!(map.len() >= 150, "expected ~172 lessons, got {}", map.len());
        for prefix in [
            "Bio::", "Biochem::", "GenChem::", "Orgo::", "Physics::", "PsychSoc::",
        ] {
            assert!(map.keys().any(|k| k.starts_with(prefix)), "missing {prefix}");
        }
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
