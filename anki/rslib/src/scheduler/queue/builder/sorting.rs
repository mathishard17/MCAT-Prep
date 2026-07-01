// Copyright: Ankitects Pty Ltd and contributors
// License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html

use std::cmp::Ordering;
use std::collections::HashMap;
use std::hash::Hasher;

use fnv::FnvHasher;

use super::ConceptNewCardSort;
use super::NewCard;
use super::NewCardSortOrder;
use super::QueueBuilder;
use crate::prelude::*;
use crate::scheduler::concept::CardConceptMetadata;
use crate::scheduler::concept::KnowledgeGraph;
use crate::scheduler::concept::ReadinessEvidenceStatus;
use crate::tags::split_tags;

impl QueueBuilder {
    pub(super) fn prepare_concept_new_card_sort(&mut self, col: &Collection) -> Result<()> {
        if self.new.is_empty()
            || !self
                .context
                .root_deck
                .normal()
                .map(|deck| deck.concept_scheduler_enabled)
                .unwrap_or(false)
        {
            return Ok(());
        }

        let persisted = col.get_concept_scheduler_state(self.context.root_deck.id);
        if matches!(
            persisted.state.readiness_evidence_status(&persisted.config),
            ReadinessEvidenceStatus::InsufficientEvidence { .. }
        ) {
            return Ok(());
        }

        let mut note_ids: Vec<_> = self.new.iter().map(|card| card.note_id).collect();
        note_ids.sort_unstable();
        note_ids.dedup();

        let metadata_by_note: HashMap<_, _> = col
            .storage
            .get_note_tags_by_id_list(&note_ids)?
            .into_iter()
            .map(|note_tags| {
                (
                    note_tags.id,
                    CardConceptMetadata::from_tags(split_tags(&note_tags.tags)),
                )
            })
            .collect();

        if !metadata_by_note
            .values()
            .any(|metadata| !metadata.target_components.is_empty())
        {
            return Ok(());
        }

        let graph = KnowledgeGraph::from_card_metadata(metadata_by_note.values());
        if graph.cycle().is_some() {
            return Ok(());
        }

        self.context.concept_sort = Some(ConceptNewCardSort {
            graph,
            metadata_by_note,
            persisted,
        });

        Ok(())
    }

    pub(super) fn sort_new(&mut self) {
        if self.context.concept_sort.is_some() {
            self.sort_new_by_concept_readiness();
            return;
        }

        match self.context.sort_options.new_order {
            // preserve gather order
            NewCardSortOrder::NoSort => (),
            NewCardSortOrder::Template => {
                // stable sort to preserve gather order
                self.new
                    .sort_by(|a, b| a.template_index.cmp(&b.template_index))
            }
            NewCardSortOrder::TemplateThenRandom => {
                self.hash_new_cards_by_id();
                self.new.sort_unstable_by(cmp_template_then_hash);
            }
            NewCardSortOrder::RandomNoteThenTemplate => {
                self.hash_new_cards_by_note_id();
                self.new.sort_unstable_by(cmp_hash_then_template);
            }
            NewCardSortOrder::RandomCard => {
                self.hash_new_cards_by_id();
                self.new.sort_unstable_by(cmp_hash)
            }
        }
    }

    fn sort_new_by_concept_readiness(&mut self) {
        let concept_sort = self.context.concept_sort.as_ref().unwrap();
        self.new.sort_by(|a, b| {
            let a_score = concept_sort.score(a);
            let b_score = concept_sort.score(b);
            b_score
                .partial_cmp(&a_score)
                .unwrap_or(Ordering::Equal)
                .then_with(|| concept_sort.is_overview(b).cmp(&concept_sort.is_overview(a)))
                .then_with(|| a.template_index.cmp(&b.template_index))
        });
    }

    fn hash_new_cards_by_id(&mut self) {
        self.new
            .iter_mut()
            .for_each(|card| card.hash_id_with_salt(self.context.timing.days_elapsed as i64));
    }

    fn hash_new_cards_by_note_id(&mut self) {
        self.new
            .iter_mut()
            .for_each(|card| card.hash_note_id_with_salt(self.context.timing.days_elapsed as i64));
    }
}

impl ConceptNewCardSort {
    fn score(&self, card: &NewCard) -> f64 {
        self.metadata_by_note
            .get(&card.note_id)
            .map(|metadata| {
                self.persisted.state.card_readiness_score(
                    metadata,
                    &self.graph,
                    &self.persisted.config,
                )
            })
            .unwrap_or(self.persisted.config.fallback_readiness_score)
    }

    fn is_overview(&self, card: &NewCard) -> bool {
        self.metadata_by_note
            .get(&card.note_id)
            .map(CardConceptMetadata::is_overview)
            .unwrap_or(false)
    }
}

fn cmp_hash(a: &NewCard, b: &NewCard) -> Ordering {
    a.hash.cmp(&b.hash)
}

fn cmp_template_then_hash(a: &NewCard, b: &NewCard) -> Ordering {
    (a.template_index, a.hash).cmp(&(b.template_index, b.hash))
}

fn cmp_hash_then_template(a: &NewCard, b: &NewCard) -> Ordering {
    (a.hash, a.template_index).cmp(&(b.hash, b.template_index))
}

// We sort based on a hash so that if the queue is rebuilt, remaining
// cards come back in the same approximate order (mixing + due learning cards
// may still result in a different card)

impl NewCard {
    fn hash_id_with_salt(&mut self, salt: i64) {
        let mut hasher = FnvHasher::default();
        hasher.write_i64(self.id.0);
        hasher.write_i64(salt);
        self.hash = hasher.finish();
    }

    fn hash_note_id_with_salt(&mut self, salt: i64) {
        let mut hasher = FnvHasher::default();
        hasher.write_i64(self.note_id.0);
        hasher.write_i64(salt);
        self.hash = hasher.finish();
    }
}
