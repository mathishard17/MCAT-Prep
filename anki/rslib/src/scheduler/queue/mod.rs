// Copyright: Ankitects Pty Ltd and contributors
// License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html

mod builder;
mod entry;
mod learning;
mod main;
pub(crate) mod undo;

use std::collections::HashMap;
use std::collections::VecDeque;

use anki_proto::scheduler;
use anki_proto::scheduler::SchedulingContext;
pub(crate) use builder::DueCard;
pub(crate) use builder::DueCardKind;
pub(crate) use builder::NewCard;
pub(crate) use entry::QueueEntry;
pub(crate) use entry::QueueEntryKind;
pub(crate) use learning::LearningQueueEntry;
pub(crate) use main::MainQueueEntry;
pub(crate) use main::MainQueueEntryKind;

use self::undo::QueueUpdate;
use super::states::SchedulingStates;
use super::timing::SchedTimingToday;
use crate::prelude::*;
use crate::scheduler::concept::CardConceptMetadata;
use crate::scheduler::concept::KnowledgeComponentId;
use crate::scheduler::states::load_balancer::LoadBalancer;
use crate::timestamp::TimestampSecs;

const CONCEPT_REVIEWS_PER_NEW_SLOT: u8 = 4;
const CONCEPT_FOCUSED_BLOCK_SIZE: u8 = 3;

#[derive(Debug)]
pub(crate) struct CardQueues {
    counts: Counts,
    main: VecDeque<MainQueueEntry>,
    intraday_learning: VecDeque<LearningQueueEntry>,
    current_day: u32,
    learn_ahead_secs: i64,
    build_time: TimestampMillis,
    /// Updated each time a card is answered, and by get_queued_cards() when the
    /// counts are zero. Ensures we don't show a newly-due learning card after a
    /// user returns from editing a review card.
    current_learning_cutoff: TimestampSecs,
    pub(crate) load_balancer: Option<LoadBalancer>,
    pub(crate) concept_session: Option<ConceptSessionState>,
}

#[derive(Debug, Copy, Clone)]
pub struct Counts {
    pub new: usize,
    pub learning: usize,
    pub review: usize,
}

impl Counts {
    fn all_zero(self) -> bool {
        self.new == 0 && self.learning == 0 && self.review == 0
    }
}

#[derive(Debug, Clone)]
pub struct QueuedCard {
    pub card: Card,
    pub kind: QueueEntryKind,
    pub states: SchedulingStates,
    pub context: SchedulingContext,
}

#[derive(Debug)]
pub struct QueuedCards {
    pub cards: Vec<QueuedCard>,
    pub new_count: usize,
    pub learning_count: usize,
    pub review_count: usize,
}

/// When we encounter a card with new or review burying enabled, all future
/// siblings need to be buried, regardless of their own settings.
#[derive(Default, Debug, Clone, Copy)]
pub(crate) struct BuryMode {
    pub(crate) bury_new: bool,
    pub(crate) bury_reviews: bool,
    pub(crate) bury_interday_learning: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ConceptSessionState {
    metadata_by_card: HashMap<CardId, CardConceptMetadata>,
    reviews_toward_next_slot: u8,
    slots_available: u32,
    block_remaining: u8,
    active_topic: Option<KnowledgeComponentId>,
    selected_topic: Option<KnowledgeComponentId>,
    visible_new_cards: Vec<CardId>,
}

#[derive(Debug, Clone)]
pub(crate) struct ConceptQueueSnapshot {
    main: VecDeque<MainQueueEntry>,
    counts: Counts,
    concept_session: Option<ConceptSessionState>,
}

impl ConceptSessionState {
    pub(crate) fn new(
        metadata_by_card: HashMap<CardId, CardConceptMetadata>,
    ) -> Option<ConceptSessionState> {
        (!metadata_by_card.is_empty()).then(|| ConceptSessionState {
            metadata_by_card,
            reviews_toward_next_slot: 0,
            slots_available: 0,
            block_remaining: 0,
            active_topic: None,
            selected_topic: None,
            visible_new_cards: Vec::new(),
        })
    }

    fn record_review(&mut self) {
        self.reviews_toward_next_slot += 1;
        if self.reviews_toward_next_slot >= CONCEPT_REVIEWS_PER_NEW_SLOT {
            self.reviews_toward_next_slot = 0;
            self.slots_available += 1;
        }
    }

    fn record_new_card(&mut self, card_id: CardId) {
        if !self.visible_new_cards.contains(&card_id) {
            return;
        }

        self.slots_available = self.slots_available.saturating_sub(1);
        self.block_remaining = self.block_remaining.saturating_sub(1);
        self.visible_new_cards.retain(|visible| *visible != card_id);
        if self.block_remaining == 0 {
            self.active_topic = None;
        }
    }

    fn maybe_open_block(&mut self, main: &VecDeque<MainQueueEntry>) {
        self.visible_new_cards.clear();
        if self.block_remaining > 0 || self.slots_available == 0 {
            return;
        }

        let review_remaining = main
            .iter()
            .any(|entry| !matches!(entry.kind, MainQueueEntryKind::New));
        if review_remaining && self.slots_available < CONCEPT_FOCUSED_BLOCK_SIZE as u32 {
            return;
        }

        let Some(topic) = self
            .selected_topic
            .clone()
            .or_else(|| self.first_topic_in(main))
        else {
            return;
        };

        self.active_topic = Some(topic);
        self.block_remaining = self
            .slots_available
            .min(CONCEPT_FOCUSED_BLOCK_SIZE as u32)
            .try_into()
            .unwrap_or(CONCEPT_FOCUSED_BLOCK_SIZE);
    }

    fn first_topic_in(&self, main: &VecDeque<MainQueueEntry>) -> Option<KnowledgeComponentId> {
        main.iter().find_map(|entry| {
            if !matches!(entry.kind, MainQueueEntryKind::New) {
                return None;
            }

            self.metadata_by_card
                .get(&entry.id)
                .and_then(|metadata| metadata.target_components.first())
                .cloned()
        })
    }

    fn matches_active_topic(&self, card_id: CardId) -> bool {
        let Some(active_topic) = &self.active_topic else {
            return false;
        };

        self.metadata_by_card
            .get(&card_id)
            .map(|metadata| metadata.target_components.contains(active_topic))
            .unwrap_or(false)
    }

    fn allows_visible_new_card(&self, card_id: CardId) -> bool {
        self.visible_new_cards.contains(&card_id)
    }

    pub(crate) fn status(&self) -> scheduler::ConceptSessionStatus {
        scheduler::ConceptSessionStatus {
            reviews_toward_next_slot: self.reviews_toward_next_slot as u32,
            reviews_per_slot: CONCEPT_REVIEWS_PER_NEW_SLOT as u32,
            slots_available: self.slots_available,
            block_remaining: self.block_remaining as u32,
            block_size: CONCEPT_FOCUSED_BLOCK_SIZE as u32,
            active_topic: self
                .active_topic
                .as_ref()
                .map(|topic| topic.as_str().to_string()),
            selected_topic: self
                .selected_topic
                .as_ref()
                .map(|topic| topic.as_str().to_string()),
            budget_progress: self.reviews_toward_next_slot as f32
                / CONCEPT_REVIEWS_PER_NEW_SLOT as f32,
        }
    }
}

impl Collection {
    pub fn get_next_card(&mut self) -> Result<Option<QueuedCard>> {
        self.get_queued_cards(1, false)
            .map(|queued| queued.cards.first().cloned())
    }

    pub fn get_queued_cards(
        &mut self,
        fetch_limit: usize,
        intraday_learning_only: bool,
    ) -> Result<QueuedCards> {
        let queues = self.get_queues()?;
        queues.prepare_concept_presentation();
        let counts = queues.counts();
        let entries: Vec<_> = if intraday_learning_only {
            queues
                .intraday_now_iter()
                .chain(queues.intraday_ahead_iter())
                .map(Into::into)
                .collect()
        } else {
            queues.iter().take(fetch_limit).collect()
        };
        let cards: Vec<_> = entries
            .into_iter()
            .map(|entry| {
                let card = self
                    .storage
                    .get_card(entry.card_id())?
                    .or_not_found(entry.card_id())?;
                require!(
                    card.mtime == entry.mtime(),
                    "bug: card modified without updating queue: id:{} card:{} entry:{}",
                    card.id,
                    card.mtime,
                    entry.mtime()
                );

                // fixme: pass in card instead of id
                let next_states = self.get_scheduling_states(card.id)?;

                Ok(QueuedCard {
                    context: new_scheduling_context(self, &card)?,
                    card,
                    states: next_states,
                    kind: entry.kind(),
                })
            })
            .collect::<Result<_>>()?;
        Ok(QueuedCards {
            cards,
            new_count: counts.new,
            learning_count: counts.learning,
            review_count: counts.review,
        })
    }
}

fn new_scheduling_context(col: &mut Collection, card: &Card) -> Result<SchedulingContext> {
    Ok(SchedulingContext {
        deck_name: col
            .get_deck(card.original_or_current_deck_id())?
            .or_not_found(card.deck_id)?
            .human_name(),
        seed: card.review_seed(),
        decay: card.decay,
        desired_retention: card.desired_retention,
    })
}

impl CardQueues {
    /// An iterator over the card queues, in the order the cards will
    /// be presented.
    fn iter(&self) -> impl Iterator<Item = QueueEntry> + '_ {
        self.intraday_now_iter()
            .map(Into::into)
            .chain(
                self.main
                    .iter()
                    .filter(|entry| self.entry_is_visible(**entry))
                    .map(Into::into),
            )
            .chain(self.intraday_ahead_iter().map(Into::into))
    }

    pub(crate) fn concept_queue_snapshot(&self) -> Option<ConceptQueueSnapshot> {
        self.concept_session
            .as_ref()
            .map(|concept_session| ConceptQueueSnapshot {
                main: self.main.clone(),
                counts: self.counts,
                concept_session: Some(concept_session.clone()),
            })
    }

    pub(crate) fn concept_session_status(&self) -> Option<scheduler::ConceptSessionStatus> {
        self.concept_session
            .as_ref()
            .map(ConceptSessionState::status)
    }

    pub(crate) fn restore_concept_queue_snapshot(&mut self, snapshot: ConceptQueueSnapshot) {
        self.main = snapshot.main;
        self.counts = snapshot.counts;
        self.concept_session = snapshot.concept_session;
    }

    pub(crate) fn update_concept_session_from_answer(&mut self, entry: &QueueEntry) {
        let Some(concept_session) = self.concept_session.as_mut() else {
            return;
        };

        match entry {
            QueueEntry::Main(entry) => match entry.kind {
                MainQueueEntryKind::New => concept_session.record_new_card(entry.id),
                MainQueueEntryKind::Review | MainQueueEntryKind::InterdayLearning => {
                    concept_session.record_review();
                }
            },
            QueueEntry::IntradayLearning(_) => (),
        }

        self.prepare_concept_presentation();
    }

    pub(crate) fn prepare_concept_presentation(&mut self) {
        let Some(concept_session) = self.concept_session.as_mut() else {
            return;
        };

        let review_remaining = self
            .main
            .iter()
            .any(|entry| !matches!(entry.kind, MainQueueEntryKind::New));
        if !review_remaining
            && concept_session.slots_available == 0
            && concept_session.block_remaining == 0
        {
            concept_session.visible_new_cards = self
                .main
                .iter()
                .filter(|entry| matches!(entry.kind, MainQueueEntryKind::New))
                .map(|entry| entry.id)
                .collect();
            return;
        }

        concept_session.maybe_open_block(&self.main);
        if concept_session.block_remaining == 0 {
            self.move_new_cards_after_reviews();
            return;
        }

        let mut promoted = VecDeque::new();
        let mut deferred = VecDeque::new();
        while let Some(entry) = self.main.pop_front() {
            if matches!(entry.kind, MainQueueEntryKind::New)
                && promoted.len() < concept_session.block_remaining as usize
                && concept_session.matches_active_topic(entry.id)
            {
                concept_session.visible_new_cards.push(entry.id);
                promoted.push_back(entry);
            } else {
                deferred.push_back(entry);
            }
        }

        if promoted.is_empty() {
            concept_session.active_topic = None;
            concept_session.block_remaining = 0;
            concept_session.visible_new_cards.clear();
            self.main = deferred;
            self.move_new_cards_after_reviews();
            return;
        }

        promoted.extend(deferred);
        self.main = promoted;
    }

    fn entry_is_visible(&self, entry: MainQueueEntry) -> bool {
        if !matches!(entry.kind, MainQueueEntryKind::New) {
            return true;
        }

        self.concept_session
            .as_ref()
            .map(|concept_session| concept_session.allows_visible_new_card(entry.id))
            .unwrap_or(true)
    }

    fn move_new_cards_after_reviews(&mut self) {
        let mut due = VecDeque::new();
        let mut new = VecDeque::new();
        while let Some(entry) = self.main.pop_front() {
            if matches!(entry.kind, MainQueueEntryKind::New) {
                new.push_back(entry);
            } else {
                due.push_back(entry);
            }
        }
        due.extend(new);
        self.main = due;
    }

    /// Remove the provided card from the top of the queues and
    /// adjust the counts. If it was not at the top, return an error.
    fn pop_entry(&mut self, id: CardId) -> Result<QueueEntry> {
        if let Some(pos) = self.intraday_learning.iter().position(|e| e.id == id) {
            let entry = self.intraday_learning.remove(pos).unwrap();
            // FIXME:
            // under normal circumstances this should not go below 0, but currently
            // the Python unit tests answer learning cards before they're due
            self.counts.learning = self.counts.learning.saturating_sub(1);
            Ok(entry.into())
        } else if self.main.front().filter(|e| e.id == id).is_some() {
            Ok(self.pop_main().unwrap().into())
        } else {
            invalid_input!("not at top of queue")
        }
    }

    fn push_undo_entry(&mut self, entry: QueueEntry) {
        match entry {
            QueueEntry::IntradayLearning(entry) => self.push_intraday_learning(entry),
            QueueEntry::Main(entry) => self.push_main(entry),
        }
    }

    /// Return the current due counts. If there are no due cards, the learning
    /// cutoff is updated to the current time first, and any newly-due learning
    /// cards are added to the counts.
    pub(crate) fn counts(&mut self) -> Counts {
        if self.counts.all_zero() {
            // we discard the returned undo information in this case
            self.update_learning_cutoff_and_count();
        }
        self.counts
    }

    fn is_stale(&self, current_day: u32) -> bool {
        self.current_day != current_day
    }
}

impl Collection {
    /// This is automatically done when transact() is called for everything
    /// except card answers, so unless you are modifying state outside of a
    /// transaction, you probably don't need this.
    pub(crate) fn clear_study_queues(&mut self) {
        self.state.card_queues = None;
    }

    pub(crate) fn maybe_clear_study_queues_after_op(&mut self, op: &OpChanges) {
        if op.op != Op::AnswerCard && op.requires_study_queue_rebuild() {
            self.state.card_queues = None;
        }
    }

    pub(crate) fn update_queues_after_answering_card(
        &mut self,
        card: &Card,
        timing: SchedTimingToday,
        is_finished_preview: bool,
    ) -> Result<()> {
        if let Some(queues) = &mut self.state.card_queues {
            let concept_before = queues.concept_queue_snapshot();
            let entry = queues.pop_entry(card.id)?;
            let requeued_learning = if is_finished_preview {
                None
            } else {
                queues.maybe_requeue_learning_card(card, timing)
            };
            let cutoff_snapshot = queues.update_learning_cutoff_and_count();
            let queue_build_time = queues.build_time;
            queues.update_concept_session_from_answer(&entry);
            let concept_after = queues.concept_queue_snapshot();
            self.save_queue_update_undo(Box::new(QueueUpdate {
                entry,
                learning_requeue: requeued_learning,
                queue_build_time,
                cutoff_snapshot,
                concept_before,
                concept_after,
            }));
        } else {
            // we currently allow the queues to be empty for unit tests
        }

        Ok(())
    }

    /// Get the card queues, building if necessary.
    pub(crate) fn get_queues(&mut self) -> Result<&mut CardQueues> {
        let deck = self.get_current_deck()?;
        self.clear_queues_if_day_changed()?;
        if self.state.card_queues.is_none() {
            self.state.card_queues = Some(self.build_queues(deck.id)?);
        }

        Ok(self.state.card_queues.as_mut().unwrap())
    }

    // Returns queues if they are valid and have not been rebuilt. If build time has
    // changed, they are cleared.
    pub(crate) fn get_or_invalidate_queues(
        &mut self,
        build_time: TimestampMillis,
    ) -> Result<Option<&mut CardQueues>> {
        self.clear_queues_if_day_changed()?;
        let same_build = self
            .state
            .card_queues
            .as_ref()
            .map(|q| q.build_time == build_time)
            .unwrap_or_default();
        if same_build {
            Ok(self.state.card_queues.as_mut())
        } else {
            self.clear_study_queues();
            Ok(None)
        }
    }

    fn clear_queues_if_day_changed(&mut self) -> Result<()> {
        let timing = self.timing_today()?;
        let day_rolled_over = self
            .state
            .card_queues
            .as_ref()
            .map(|q| q.is_stale(timing.days_elapsed))
            .unwrap_or(false);
        if day_rolled_over {
            self.discard_undo_and_study_queues();
            self.unbury_on_day_rollover(timing.days_elapsed)?;
        }
        Ok(())
    }
}

// test helpers
#[cfg(test)]
impl Collection {
    pub(crate) fn counts(&mut self) -> [usize; 3] {
        self.get_queued_cards(1, false)
            .map(|q| [q.new_count, q.learning_count, q.review_count])
            .unwrap_or([0; 3])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::deckconfig::ReviewMix;
    use crate::scheduler::concept::ConceptMasteryState;
    use crate::scheduler::concept::KnowledgeComponentId;

    fn kc(id: &str) -> KnowledgeComponentId {
        KnowledgeComponentId::new(id).unwrap()
    }

    fn enable_concept_scheduler(col: &mut Collection, deck_id: DeckId) -> Result<()> {
        let deck = col.get_deck(deck_id)?.unwrap();
        let mut deck = (*deck).clone();
        deck.normal_mut()?.concept_scheduler_enabled = true;
        col.update_deck(&mut deck)?;
        Ok(())
    }

    fn add_basic_note_with_tags(
        col: &mut Collection,
        deck_id: DeckId,
        tags: &[&str],
    ) -> Result<NoteId> {
        let nt = col.get_notetype_by_name("Basic")?.unwrap();
        let mut note = nt.new_note();
        note.set_field(0, "front")?;
        note.set_field(1, "back")?;
        note.tags = tags.iter().map(|tag| tag.to_string()).collect();
        col.add_note(&mut note, deck_id)?;
        Ok(note.id)
    }

    fn add_review_card(col: &mut Collection, deck_id: DeckId) -> Result<CardId> {
        let note_id = add_basic_note_with_tags(col, deck_id, &[])?;
        let card_id = col.storage.card_ids_of_notes(&[note_id])?[0];
        col.set_due_date(&[card_id], "0", None)?;
        Ok(card_id)
    }

    fn add_glycolysis_card(col: &mut Collection, deck_id: DeckId) -> Result<NoteId> {
        add_basic_note_with_tags(
            col,
            deck_id,
            &["KC::Biochem::Glycolysis", "Prereq::Biochem::Bioenergetics"],
        )
    }

    fn add_citric_acid_cycle_card(col: &mut Collection, deck_id: DeckId) -> Result<NoteId> {
        add_basic_note_with_tags(
            col,
            deck_id,
            &[
                "KC::Biochem::Citric_Acid_Cycle",
                "Prereq::Biochem::Glycolysis",
            ],
        )
    }

    fn seed_concept_readiness(col: &mut Collection, deck_id: DeckId) -> Result<()> {
        let mut persisted = col.get_concept_scheduler_state(deck_id);
        persisted.config.readiness_min_seen_cards = 0;
        persisted.state.total_seen_cards = 1;
        persisted.state.kcs.insert(
            kc("Biochem::Bioenergetics"),
            ConceptMasteryState {
                mastery: 0.85,
                answered: 3,
                positive: 3,
                negative: 0,
            },
        );
        persisted.state.kcs.insert(
            kc("Biochem::Glycolysis"),
            ConceptMasteryState {
                mastery: 0.20,
                answered: 1,
                positive: 0,
                negative: 1,
            },
        );
        col.set_concept_scheduler_state(deck_id, &persisted, false)?;
        Ok(())
    }

    fn setup_concept_budget_deck(
        review_count: usize,
        new_count: usize,
    ) -> Result<(Collection, Vec<NoteId>)> {
        let mut col = Collection::new();
        let deck_id = DeckId(1);
        enable_concept_scheduler(&mut col, deck_id)?;
        seed_concept_readiness(&mut col, deck_id)?;
        col.update_default_deck_config(|config| {
            config.new_mix = ReviewMix::MixWithReviews as i32;
            config.new_per_day = 100;
            config.reviews_per_day = 100;
        });

        for _ in 0..review_count {
            add_review_card(&mut col, deck_id)?;
        }
        let note_ids = (0..new_count)
            .map(|_| add_glycolysis_card(&mut col, deck_id))
            .collect::<Result<Vec<_>>>()?;
        col.clear_study_queues();

        Ok((col, note_ids))
    }

    fn queued_kinds(col: &mut Collection, fetch_limit: usize) -> Vec<QueueEntryKind> {
        col.get_queued_cards(fetch_limit, false)
            .unwrap()
            .cards
            .into_iter()
            .map(|queued| queued.kind)
            .collect()
    }

    #[test]
    fn concept_budget_waits_for_focused_block() -> Result<()> {
        let (mut col, _) = setup_concept_budget_deck(12, 3)?;

        assert_eq!(
            queued_kinds(&mut col, 5),
            vec![
                QueueEntryKind::Review,
                QueueEntryKind::Review,
                QueueEntryKind::Review,
                QueueEntryKind::Review,
                QueueEntryKind::Review,
            ]
        );

        for _ in 0..11 {
            assert_eq!(queued_kinds(&mut col, 1), vec![QueueEntryKind::Review]);
            col.answer_good();
        }
        assert_eq!(queued_kinds(&mut col, 1), vec![QueueEntryKind::Review]);

        col.answer_good();
        assert_eq!(
            queued_kinds(&mut col, 5),
            vec![
                QueueEntryKind::New,
                QueueEntryKind::New,
                QueueEntryKind::New,
            ]
        );

        Ok(())
    }

    #[test]
    fn concept_budget_spends_partial_block_when_reviews_are_done() -> Result<()> {
        let (mut col, note_ids) = setup_concept_budget_deck(4, 3)?;

        for _ in 0..4 {
            assert_eq!(queued_kinds(&mut col, 1), vec![QueueEntryKind::Review]);
            col.answer_good();
        }

        let queued = col.get_queued_cards(5, false)?;
        assert_eq!(queued.cards.len(), 1);
        assert_eq!(queued.cards[0].kind, QueueEntryKind::New);
        assert!(note_ids.contains(&queued.cards[0].card.note_id));

        Ok(())
    }

    #[test]
    fn concept_budget_respects_selected_topic() -> Result<()> {
        let mut col = Collection::new();
        let deck_id = DeckId(1);
        enable_concept_scheduler(&mut col, deck_id)?;
        seed_concept_readiness(&mut col, deck_id)?;
        col.update_default_deck_config(|config| {
            config.new_mix = ReviewMix::MixWithReviews as i32;
            config.new_per_day = 100;
            config.reviews_per_day = 100;
        });

        for _ in 0..12 {
            add_review_card(&mut col, deck_id)?;
        }
        for _ in 0..3 {
            add_glycolysis_card(&mut col, deck_id)?;
        }
        let citric_acid_cycle_notes = (0..2)
            .map(|_| add_citric_acid_cycle_card(&mut col, deck_id))
            .collect::<Result<Vec<_>>>()?;
        col.clear_study_queues();

        let queues = col.get_queues()?;
        queues.concept_session.as_mut().unwrap().selected_topic =
            Some(kc("Biochem::Citric_Acid_Cycle"));

        for _ in 0..12 {
            assert_eq!(queued_kinds(&mut col, 1), vec![QueueEntryKind::Review]);
            col.answer_good();
        }

        let queued = col.get_queued_cards(5, false)?;
        assert_eq!(queued.cards.len(), 2);
        assert!(queued.cards.iter().all(|queued| {
            queued.kind == QueueEntryKind::New
                && citric_acid_cycle_notes.contains(&queued.card.note_id)
        }));

        Ok(())
    }
}
