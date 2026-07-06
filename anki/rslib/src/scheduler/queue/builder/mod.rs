// Copyright: Ankitects Pty Ltd and contributors
// License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html

mod burying;
mod gathering;
pub(crate) mod intersperser;
pub(crate) mod sized_chain;
mod sorting;

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

use intersperser::Intersperser;
use sized_chain::SizedChain;

use super::BuryMode;
use super::CardQueues;
use super::ConceptSessionState;
use super::Counts;
use super::LearningQueueEntry;
use super::MainQueueEntry;
use super::MainQueueEntryKind;
use crate::deckconfig::NewCardGatherPriority;
use crate::deckconfig::NewCardSortOrder;
use crate::deckconfig::ReviewCardOrder;
use crate::deckconfig::ReviewMix;
use crate::decks::limits::LimitTreeMap;
use crate::prelude::*;
use crate::scheduler::concept::CardConceptMetadata;
use crate::scheduler::concept::KnowledgeGraph;
use crate::scheduler::concept::PersistedConceptSchedulerState;
use crate::scheduler::states::load_balancer::LoadBalancer;
use crate::scheduler::timing::SchedTimingToday;

/// Temporary holder for review cards that will be built into a queue.
#[derive(Debug, Clone, Copy)]
pub(crate) struct DueCard {
    pub id: CardId,
    pub note_id: NoteId,
    pub mtime: TimestampSecs,
    pub due: i32,
    pub current_deck_id: DeckId,
    pub original_deck_id: DeckId,
    pub kind: DueCardKind,
    pub reps: u32,
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum DueCardKind {
    Review,
    Learning,
}

/// Temporary holder for new cards that will be built into a queue.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct NewCard {
    pub id: CardId,
    pub note_id: NoteId,
    pub mtime: TimestampSecs,
    pub current_deck_id: DeckId,
    pub original_deck_id: DeckId,
    pub template_index: u32,
    pub hash: u64,
}

impl From<DueCard> for MainQueueEntry {
    fn from(c: DueCard) -> Self {
        MainQueueEntry {
            id: c.id,
            mtime: c.mtime,
            kind: match c.kind {
                DueCardKind::Review => MainQueueEntryKind::Review,
                DueCardKind::Learning => MainQueueEntryKind::InterdayLearning,
            },
        }
    }
}

impl From<NewCard> for MainQueueEntry {
    fn from(c: NewCard) -> Self {
        MainQueueEntry {
            id: c.id,
            mtime: c.mtime,
            kind: MainQueueEntryKind::New,
        }
    }
}

impl From<DueCard> for LearningQueueEntry {
    fn from(c: DueCard) -> Self {
        LearningQueueEntry {
            due: TimestampSecs(c.due as i64),
            id: c.id,
            mtime: c.mtime,
            reps: c.reps,
        }
    }
}

#[derive(Default, Clone, Debug)]
pub(super) struct QueueSortOptions {
    pub(super) new_order: NewCardSortOrder,
    pub(super) new_gather_priority: NewCardGatherPriority,
    pub(super) review_order: ReviewCardOrder,
    pub(super) day_learn_mix: ReviewMix,
    pub(super) new_review_mix: ReviewMix,
}

#[derive(Debug)]
pub(super) struct QueueBuilder {
    pub(super) new: Vec<NewCard>,
    pub(super) review: Vec<DueCard>,
    pub(super) learning: Vec<DueCard>,
    pub(super) day_learning: Vec<DueCard>,
    limits: LimitTreeMap,
    load_balancer: Option<LoadBalancer>,
    context: Context,
    /// When the concept scheduler has a selected MCAT section, the note ids of
    /// cards filed under it. New cards whose note is absent are skipped during
    /// gathering so the section fills the daily budget. `None` = no restriction.
    concept_section_notes: Option<HashSet<NoteId>>,
}

/// Data container and helper for building queues.
#[derive(Debug, Clone)]
struct Context {
    timing: SchedTimingToday,
    config_map: HashMap<DeckConfigId, DeckConfig>,
    root_deck: Deck,
    sort_options: QueueSortOptions,
    concept_sort: Option<ConceptNewCardSort>,
    seen_note_ids: HashMap<NoteId, BuryMode>,
    deck_map: HashMap<DeckId, Deck>,
    fsrs: bool,
}

#[derive(Debug, Clone)]
struct ConceptNewCardSort {
    graph: KnowledgeGraph,
    metadata_by_note: HashMap<NoteId, CardConceptMetadata>,
    persisted: PersistedConceptSchedulerState,
}

impl QueueBuilder {
    pub(super) fn new(col: &mut Collection, deck_id: DeckId) -> Result<Self> {
        let timing = col.timing_for_timestamp(TimestampSecs::now())?;
        let new_cards_ignore_review_limit = col.get_config_bool(BoolKey::NewCardsIgnoreReviewLimit);
        let apply_all_parent_limits = col.get_config_bool(BoolKey::ApplyAllParentLimits);
        let config_map = col.storage.get_deck_config_map()?;
        let root_deck = col.storage.get_deck(deck_id)?.or_not_found(deck_id)?;
        let mut decks = col.storage.child_decks(&root_deck)?;
        decks.insert(0, root_deck.clone());
        if apply_all_parent_limits {
            for parent in col.storage.parent_decks(&root_deck)? {
                decks.insert(0, parent);
            }
        }
        let limits = LimitTreeMap::build(
            &decks,
            &config_map,
            timing.days_elapsed,
            new_cards_ignore_review_limit,
        );
        let sort_options = sort_options(&root_deck, &config_map);
        let deck_map = col.storage.get_decks_map()?;

        let load_balancer = col
            .get_config_bool(BoolKey::LoadBalancerEnabled)
            .then(|| {
                let did_to_dcid = deck_map
                    .values()
                    .filter_map(|deck| Some((deck.id, deck.config_id()?)))
                    .collect::<HashMap<_, _>>();
                LoadBalancer::new(
                    timing.days_elapsed,
                    did_to_dcid,
                    col.timing_today()?.next_day_at,
                    &col.storage,
                )
            })
            .transpose()?;

        Ok(QueueBuilder {
            new: Vec::new(),
            review: Vec::new(),
            learning: Vec::new(),
            day_learning: Vec::new(),
            limits,
            load_balancer,
            concept_section_notes: None,
            context: Context {
                timing,
                config_map,
                root_deck,
                sort_options,
                concept_sort: None,
                seen_note_ids: HashMap::new(),
                deck_map,
                fsrs: col.get_config_bool(BoolKey::Fsrs),
            },
        })
    }

    pub(super) fn build(mut self, learn_ahead_secs: i64) -> CardQueues {
        self.sort_new();
        let concept_session = self.context.concept_sort.as_ref().and_then(|concept_sort| {
            let mut session = ConceptSessionState::new(
                self.new
                    .iter()
                    .filter_map(|card| {
                        concept_sort
                            .metadata_by_note
                            .get(&card.note_id)
                            .cloned()
                            .map(|metadata| (card.id, metadata))
                    })
                    .collect(),
            )?;
            // Seed the user's chosen topic so it survives frequent queue rebuilds.
            session.selected_topic = concept_sort.persisted.state.selected_topic.clone();
            Some(session)
        });

        // intraday learning and total learn count
        let intraday_learning = sort_learning(self.learning);
        let now = TimestampSecs::now();
        let cutoff = now.adding_secs(learn_ahead_secs);
        let learn_count =
            intraday_learning.iter().filter(|e| e.due <= cutoff).count() + self.day_learning.len();
        let review_count = self.review.len();
        let new_count = self.new.len();

        // merge interday and new cards into main
        let with_interday_learn = merge_day_learning(
            self.review,
            self.day_learning,
            self.context.sort_options.day_learn_mix,
        );
        let main_iter = merge_new(
            with_interday_learn,
            self.new,
            self.context.sort_options.new_review_mix,
        );

        let mut queues = CardQueues {
            counts: Counts {
                new: new_count,
                review: review_count,
                learning: learn_count,
            },
            main: main_iter.collect(),
            intraday_learning,
            learn_ahead_secs,
            current_day: self.context.timing.days_elapsed,
            build_time: TimestampMillis::now(),
            load_balancer: self.load_balancer,
            concept_session,
            current_learning_cutoff: now,
        };
        queues.prepare_concept_presentation();
        queues
    }
}

fn sort_options(deck: &Deck, config_map: &HashMap<DeckConfigId, DeckConfig>) -> QueueSortOptions {
    deck.config_id()
        .and_then(|config_id| config_map.get(&config_id))
        .map(|config| QueueSortOptions {
            new_order: config.inner.new_card_sort_order(),
            new_gather_priority: config.inner.new_card_gather_priority(),
            review_order: config.inner.review_order(),
            day_learn_mix: config.inner.interday_learning_mix(),
            new_review_mix: config.inner.new_mix(),
        })
        .unwrap_or_else(|| {
            // filtered decks do not space siblings
            QueueSortOptions {
                new_order: NewCardSortOrder::NoSort,
                ..Default::default()
            }
        })
}

fn merge_day_learning(
    reviews: Vec<DueCard>,
    day_learning: Vec<DueCard>,
    mode: ReviewMix,
) -> Box<dyn ExactSizeIterator<Item = MainQueueEntry>> {
    let day_learning_iter = day_learning.into_iter().map(Into::into);
    let reviews_iter = reviews.into_iter().map(Into::into);

    match mode {
        ReviewMix::AfterReviews => Box::new(SizedChain::new(reviews_iter, day_learning_iter)),
        ReviewMix::BeforeReviews => Box::new(SizedChain::new(day_learning_iter, reviews_iter)),
        ReviewMix::MixWithReviews => Box::new(Intersperser::new(reviews_iter, day_learning_iter)),
    }
}

fn merge_new(
    review_iter: impl ExactSizeIterator<Item = MainQueueEntry> + 'static,
    new: Vec<NewCard>,
    mode: ReviewMix,
) -> Box<dyn ExactSizeIterator<Item = MainQueueEntry>> {
    let new_iter = new.into_iter().map(Into::into);

    match mode {
        ReviewMix::BeforeReviews => Box::new(SizedChain::new(new_iter, review_iter)),
        ReviewMix::AfterReviews => Box::new(SizedChain::new(review_iter, new_iter)),
        ReviewMix::MixWithReviews => Box::new(Intersperser::new(review_iter, new_iter)),
    }
}

fn sort_learning(learning: Vec<DueCard>) -> VecDeque<LearningQueueEntry> {
    let mut entries: Vec<LearningQueueEntry> =
        learning.into_iter().map(LearningQueueEntry::from).collect();
    entries.sort_by(|a, b| a.cmp_by_reps_then_due(b));
    entries.into_iter().collect()
}

impl Collection {
    pub(crate) fn build_queues(&mut self, deck_id: DeckId) -> Result<CardQueues> {
        let mut queues = QueueBuilder::new(self, deck_id)?;
        self.storage
            .update_active_decks(&queues.context.root_deck)?;

        queues.gather_cards(self)?;

        let queues = queues.build(self.learn_ahead_secs() as i64);

        Ok(queues)
    }
}

#[cfg(test)]
mod test {
    use anki_proto::deck_config::deck_config::config::NewCardGatherPriority;
    use anki_proto::deck_config::deck_config::config::NewCardSortOrder;

    use super::*;
    use crate::card::CardQueue;
    use crate::card::CardType;
    use crate::scheduler::concept::ConceptMasteryState;
    use crate::scheduler::concept::KnowledgeComponentId;
    use crate::scheduler::concept::McatSection;

    impl Collection {
        fn set_deck_gather_order(&mut self, deck: &mut Deck, order: NewCardGatherPriority) {
            let mut conf = DeckConfig::default();
            conf.inner.new_card_gather_priority = order as i32;
            conf.inner.new_card_sort_order = NewCardSortOrder::NoSort as i32;
            self.add_or_update_deck_config(&mut conf).unwrap();
            deck.normal_mut().unwrap().config_id = conf.id.0;
            self.add_or_update_deck(deck).unwrap();
        }

        fn set_deck_new_limit(&mut self, deck: &mut Deck, new_limit: u32) {
            let mut conf = DeckConfig::default();
            conf.inner.new_per_day = new_limit;
            self.add_or_update_deck_config(&mut conf).unwrap();
            deck.normal_mut().unwrap().config_id = conf.id.0;
            self.add_or_update_deck(deck).unwrap();
        }

        fn set_deck_review_limit(&mut self, deck: DeckId, limit: u32) {
            let dcid = self.get_deck(deck).unwrap().unwrap().config_id().unwrap();
            let mut conf = self.get_deck_config(dcid, false).unwrap().unwrap();
            conf.inner.reviews_per_day = limit;
            self.add_or_update_deck_config(&mut conf).unwrap();
        }

        fn queue_as_deck_and_template(&mut self, deck_id: DeckId) -> Vec<(DeckId, u16)> {
            self.build_queues(deck_id)
                .unwrap()
                .iter()
                .map(|entry| {
                    let card = self.storage.get_card(entry.card_id()).unwrap().unwrap();
                    (card.deck_id, card.template_idx)
                })
                .collect()
        }

        fn queue_note_ids(&mut self, deck_id: DeckId) -> Vec<NoteId> {
            self.build_queues(deck_id)
                .unwrap()
                .iter()
                .map(|entry| {
                    self.storage
                        .get_card(entry.card_id())
                        .unwrap()
                        .unwrap()
                        .note_id
                })
                .collect()
        }

        fn set_deck_review_order(&mut self, deck: &mut Deck, order: ReviewCardOrder) {
            let mut conf = DeckConfig::default();
            conf.inner.review_order = order as i32;
            self.add_or_update_deck_config(&mut conf).unwrap();
            deck.normal_mut().unwrap().config_id = conf.id.0;
            self.add_or_update_deck(deck).unwrap();
        }

        fn queue_as_due_and_ivl(&mut self, deck_id: DeckId) -> Vec<(i32, u32)> {
            self.build_queues(deck_id)
                .unwrap()
                .iter()
                .map(|entry| {
                    let card = self.storage.get_card(entry.card_id()).unwrap().unwrap();
                    (card.due, card.interval)
                })
                .collect()
        }
    }

    fn kc(id: &str) -> KnowledgeComponentId {
        KnowledgeComponentId::new(id).unwrap()
    }

    fn add_tagged_note(col: &mut Collection, deck_id: DeckId, tags: &[&str]) -> Result<NoteId> {
        let nt = col.get_notetype_by_name("Basic")?.unwrap();
        let mut note = nt.new_note();
        note.tags = tags.iter().map(|tag| tag.to_string()).collect();
        col.add_note(&mut note, deck_id)?;
        Ok(note.id)
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

    #[test]
    fn should_build_empty_queue_if_limit_is_reached() {
        let mut col = Collection::new();
        CardAdder::new().due_dates(["0"]).add(&mut col);
        col.set_deck_review_limit(DeckId(1), 0);
        assert_eq!(col.queue_as_deck_and_template(DeckId(1)), vec![]);
    }

    #[test]
    fn new_queue_building() -> Result<()> {
        let mut col = Collection::new();

        // parent
        // ┣━━child━━grandchild
        // ┗━━child_2
        let mut parent = DeckAdder::new("parent").add(&mut col);
        let mut child = DeckAdder::new("parent::child").add(&mut col);
        let child_2 = DeckAdder::new("parent::child_2").add(&mut col);
        let grandchild = DeckAdder::new("parent::child::grandchild").add(&mut col);

        // add 2 new cards to each deck
        for deck in [&parent, &child, &child_2, &grandchild] {
            CardAdder::new().siblings(2).deck(deck.id).add(&mut col);
        }

        // set child's new limit to 3, which should affect grandchild
        col.set_deck_new_limit(&mut child, 3);

        // depth-first tree order
        col.set_deck_gather_order(&mut parent, NewCardGatherPriority::Deck);
        let cards = vec![
            (parent.id, 0),
            (parent.id, 1),
            (child.id, 0),
            (child.id, 1),
            (grandchild.id, 0),
            (child_2.id, 0),
            (child_2.id, 1),
        ];
        assert_eq!(col.queue_as_deck_and_template(parent.id), cards);

        // insertion order
        col.set_deck_gather_order(&mut parent, NewCardGatherPriority::LowestPosition);
        let cards = vec![
            (parent.id, 0),
            (parent.id, 1),
            (child.id, 0),
            (child.id, 1),
            (child_2.id, 0),
            (child_2.id, 1),
            (grandchild.id, 0),
        ];
        assert_eq!(col.queue_as_deck_and_template(parent.id), cards);

        // inverted insertion order, but sibling order is preserved
        col.set_deck_gather_order(&mut parent, NewCardGatherPriority::HighestPosition);
        let cards = vec![
            (grandchild.id, 0),
            (grandchild.id, 1),
            (child_2.id, 0),
            (child_2.id, 1),
            (child.id, 0),
            (parent.id, 0),
            (parent.id, 1),
        ];
        assert_eq!(col.queue_as_deck_and_template(parent.id), cards);

        Ok(())
    }

    #[test]
    fn concept_enabled_sorts_new_cards_by_readiness() -> Result<()> {
        let mut col = Collection::new();
        let mut deck = col.get_or_create_normal_deck("Default")?;
        deck.normal_mut()?.concept_scheduler_enabled = true;
        col.add_or_update_deck(&mut deck)?;
        col.set_deck_gather_order(&mut deck, NewCardGatherPriority::LowestPosition);
        seed_concept_readiness(&mut col, deck.id)?;

        let citric_acid_cycle = add_tagged_note(
            &mut col,
            deck.id,
            &[
                "KC::Biochem::Citric_Acid_Cycle",
                "Prereq::Biochem::Glycolysis",
            ],
        )?;
        let glycolysis = add_tagged_note(
            &mut col,
            deck.id,
            &["KC::Biochem::Glycolysis", "Prereq::Biochem::Bioenergetics"],
        )?;

        assert_eq!(
            col.queue_note_ids(deck.id),
            vec![glycolysis, citric_acid_cycle]
        );

        Ok(())
    }

    #[test]
    fn concept_enabled_sorts_overview_before_matching_practice_card() -> Result<()> {
        let mut col = Collection::new();
        let mut deck = col.get_or_create_normal_deck("Default")?;
        deck.normal_mut()?.concept_scheduler_enabled = true;
        col.add_or_update_deck(&mut deck)?;
        col.set_deck_gather_order(&mut deck, NewCardGatherPriority::LowestPosition);
        seed_concept_readiness(&mut col, deck.id)?;

        let practice = add_tagged_note(
            &mut col,
            deck.id,
            &["KC::Biochem::Glycolysis", "Prereq::Biochem::Bioenergetics"],
        )?;
        let overview = add_tagged_note(
            &mut col,
            deck.id,
            &[
                "KC::Biochem::Glycolysis",
                "Overview::Biochem::Glycolysis",
                "Prereq::Biochem::Bioenergetics",
            ],
        )?;

        assert_eq!(col.queue_note_ids(deck.id), vec![overview, practice]);

        Ok(())
    }

    #[test]
    fn concept_disabled_preserves_new_card_gather_order() -> Result<()> {
        let mut col = Collection::new();
        let mut deck = col.get_or_create_normal_deck("Default")?;
        col.set_deck_gather_order(&mut deck, NewCardGatherPriority::LowestPosition);
        seed_concept_readiness(&mut col, deck.id)?;

        let citric_acid_cycle = add_tagged_note(
            &mut col,
            deck.id,
            &[
                "KC::Biochem::Citric_Acid_Cycle",
                "Prereq::Biochem::Glycolysis",
            ],
        )?;
        let glycolysis = add_tagged_note(
            &mut col,
            deck.id,
            &["KC::Biochem::Glycolysis", "Prereq::Biochem::Bioenergetics"],
        )?;

        assert_eq!(
            col.queue_note_ids(deck.id),
            vec![citric_acid_cycle, glycolysis]
        );

        Ok(())
    }

    /// Study-by-section: selecting an MCAT section restricts the served new-card
    /// pool to that section's cards even though other sections were gathered first
    /// (mirrors the full deck, where Biology cards are inserted — and gathered —
    /// ahead of the other sections). Clearing the selection restores the normal
    /// all-section order.
    #[test]
    fn selected_section_restricts_new_cards_and_clearing_restores() -> Result<()> {
        let mut col = Collection::new();
        let mut deck = col.get_or_create_normal_deck("Default")?;
        deck.normal_mut()?.concept_scheduler_enabled = true;
        col.add_or_update_deck(&mut deck)?;
        // Insertion order == gather order, and Bio is inserted first (as in the
        // real MCAT deck), so without a section it would dominate the new queue.
        col.set_deck_gather_order(&mut deck, NewCardGatherPriority::LowestPosition);

        let bio_a = add_tagged_note(&mut col, deck.id, &["KC::Bio::DNA", "MCAT::Bio_Biochem"])?;
        let bio_b =
            add_tagged_note(&mut col, deck.id, &["KC::Bio::Genetics", "MCAT::Bio_Biochem"])?;
        let chem_a = add_tagged_note(
            &mut col,
            deck.id,
            &["KC::Physics::Kinematics", "MCAT::Chem_Phys"],
        )?;
        let chem_b =
            add_tagged_note(&mut col, deck.id, &["KC::GenChem::Kinetics", "MCAT::Chem_Phys"])?;

        // No section selected: all sections served in gather order (Bio first).
        assert_eq!(
            col.queue_note_ids(deck.id),
            vec![bio_a, bio_b, chem_a, chem_b]
        );

        // Select Chem/Phys: only its cards are served, though Bio gathered first.
        col.set_concept_selected_section(deck.id, Some(McatSection::ChemPhys))?;
        assert_eq!(col.queue_note_ids(deck.id), vec![chem_a, chem_b]);

        // Select Bio/Biochem: the pool flips to the Biology cards.
        col.set_concept_selected_section(deck.id, Some(McatSection::BioBiochem))?;
        assert_eq!(col.queue_note_ids(deck.id), vec![bio_a, bio_b]);

        // Clearing restores the normal all-section order.
        col.set_concept_selected_section(deck.id, None)?;
        assert_eq!(
            col.queue_note_ids(deck.id),
            vec![bio_a, bio_b, chem_a, chem_b]
        );

        Ok(())
    }

    /// A section with no matching cards must fall back to no restriction, so the
    /// new-card queue is never emptied by selecting an absent section.
    #[test]
    fn selected_section_with_no_cards_falls_back_to_all() -> Result<()> {
        let mut col = Collection::new();
        let mut deck = col.get_or_create_normal_deck("Default")?;
        deck.normal_mut()?.concept_scheduler_enabled = true;
        col.add_or_update_deck(&mut deck)?;
        col.set_deck_gather_order(&mut deck, NewCardGatherPriority::LowestPosition);

        let bio_a = add_tagged_note(&mut col, deck.id, &["KC::Bio::DNA", "MCAT::Bio_Biochem"])?;
        let bio_b =
            add_tagged_note(&mut col, deck.id, &["KC::Bio::Genetics", "MCAT::Bio_Biochem"])?;

        // Psych/Soc has no cards here: the filter must not empty the queue.
        col.set_concept_selected_section(deck.id, Some(McatSection::PsychSoc))?;
        assert_eq!(col.queue_note_ids(deck.id), vec![bio_a, bio_b]);

        Ok(())
    }

    /// Undo safety: with a section selected, answering a card then undoing restores
    /// it to a fresh new card and the queue still builds — no corruption from the
    /// section-aware gather path interacting with the answer/undo hooks.
    #[test]
    fn selected_section_review_then_undo_is_clean() -> Result<()> {
        let mut col = Collection::new();
        let mut deck = col.get_or_create_normal_deck("Default")?;
        deck.normal_mut()?.concept_scheduler_enabled = true;
        col.add_or_update_deck(&mut deck)?;
        col.set_deck_gather_order(&mut deck, NewCardGatherPriority::LowestPosition);

        add_tagged_note(&mut col, deck.id, &["KC::Physics::Kinematics", "MCAT::Chem_Phys"])?;
        add_tagged_note(&mut col, deck.id, &["KC::GenChem::Kinetics", "MCAT::Chem_Phys"])?;
        col.set_current_deck(deck.id)?;
        col.set_concept_selected_section(deck.id, Some(McatSection::ChemPhys))?;

        let queued = col.get_next_card()?.unwrap();
        let card_id = queued.card.id;
        assert_eq!(queued.card.reps, 0);

        let answered = col.answer_good();
        assert_eq!(answered.card_id, card_id);
        assert!(col.storage.get_card(card_id)?.unwrap().reps >= 1);

        col.undo()?;
        let restored = col.storage.get_card(card_id)?.unwrap();
        assert_eq!(restored.reps, 0, "undo must restore the card to a new card");
        // The collection is still consistent: the queue rebuilds and the selection
        // persisted (a config write, intentionally not part of the review's undo).
        assert!(!col.queue_note_ids(deck.id).is_empty());
        assert_eq!(
            col.get_concept_scheduler_state(deck.id)
                .state
                .selected_section,
            Some(McatSection::ChemPhys)
        );

        Ok(())
    }

    #[test]
    fn review_queue_building() -> Result<()> {
        let mut col = Collection::new();

        let mut deck = col.get_or_create_normal_deck("Default").unwrap();
        let nt = col.get_notetype_by_name("Basic")?.unwrap();
        let mut cards = vec![];

        // relative overdueness
        let expected_queue = vec![
            (-150, 1),
            (-100, 1),
            (-50, 1),
            (-150, 5),
            (-100, 5),
            (-50, 5),
            (-150, 20),
            (-150, 20),
            (-100, 20),
            (-50, 20),
            (-150, 100),
            (-100, 100),
            (-50, 100),
            (0, 1),
            (0, 5),
            (0, 20),
            (0, 100),
        ];
        for t in expected_queue.iter() {
            let mut note = nt.new_note();
            note.set_field(0, "foo")?;
            note.id.0 = 0;
            col.add_note(&mut note, deck.id)?;
            let mut card = col.storage.get_card_by_ordinal(note.id, 0)?.unwrap();
            card.interval = t.1;
            card.due = t.0;
            card.ctype = CardType::Review;
            card.queue = CardQueue::Review;
            cards.push(card);
        }
        col.update_cards_maybe_undoable(cards, false)?;
        col.set_deck_review_order(&mut deck, ReviewCardOrder::RelativeOverdueness);
        assert_eq!(col.queue_as_due_and_ivl(deck.id), expected_queue);

        Ok(())
    }

    impl Collection {
        fn card_queue_len(&mut self) -> usize {
            self.get_queued_cards(5, false).unwrap().cards.len()
        }
    }

    #[test]
    fn new_card_potentially_burying_review_card() {
        let mut col = Collection::new();
        // add one new and one review card
        CardAdder::new().siblings(2).due_dates(["0"]).add(&mut col);
        // Potentially problematic config: New cards are shown first and would bury
        // review siblings. This poses a problem because we gather review cards first.
        col.update_default_deck_config(|config| {
            config.new_mix = ReviewMix::BeforeReviews as i32;
            config.bury_new = false;
            config.bury_reviews = true;
        });

        let old_queue_len = col.card_queue_len();
        col.answer_easy();
        col.clear_study_queues();

        // The number of cards in the queue must decrease by exactly 1, either because
        // no burying was performed, or the first built queue anticipated it and didn't
        // include the buried card.
        assert_eq!(col.card_queue_len(), old_queue_len - 1);
    }

    #[test]
    fn new_cards_may_ignore_review_limit() {
        let mut col = Collection::new();
        col.set_config_bool(BoolKey::NewCardsIgnoreReviewLimit, true, false)
            .unwrap();
        col.update_default_deck_config(|config| {
            config.reviews_per_day = 0;
        });
        CardAdder::new().add(&mut col);

        // review limit doesn't apply to new card
        assert_eq!(col.card_queue_len(), 1);
    }

    #[test]
    fn reviews_dont_affect_new_limit_before_review_limit_is_reached() {
        let mut col = Collection::new();
        col.update_default_deck_config(|config| {
            config.new_per_day = 1;
        });
        CardAdder::new().siblings(2).due_dates(["0"]).add(&mut col);
        assert_eq!(col.card_queue_len(), 2);
    }

    #[test]
    fn may_apply_parent_limits() {
        let mut col = Collection::new();
        col.set_config_bool(BoolKey::ApplyAllParentLimits, true, false)
            .unwrap();
        col.update_default_deck_config(|config| {
            config.new_per_day = 0;
        });
        let child = DeckAdder::new("Default::child")
            .with_config(|_| ())
            .add(&mut col);
        CardAdder::new().deck(child.id).add(&mut col);
        col.set_current_deck(child.id).unwrap();
        assert_eq!(col.card_queue_len(), 0);
    }

    /// Among the first `k` served new cards, count how many target a KC whose
    /// prerequisites are UNMET — i.e. some prerequisite's persisted mastery is below
    /// the engine's `outer_fringe_prereq_mastery` threshold. Purely in-engine: the
    /// served order comes from the real queue builder and the "unmet" test reuses the
    /// same `mastery_for` / threshold the scheduler itself uses.
    fn count_unmet_prereq_new_cards(col: &mut Collection, deck_id: DeckId, k: usize) -> usize {
        let persisted = col.get_concept_scheduler_state(deck_id);
        let served_card_ids: Vec<CardId> = col
            .build_queues(deck_id)
            .unwrap()
            .iter()
            .take(k)
            .map(|entry| entry.card_id())
            .collect();
        served_card_ids
            .into_iter()
            .filter(|card_id| {
                let card = col.storage.get_card(*card_id).unwrap().unwrap();
                let note = col.storage.get_note(card.note_id).unwrap().unwrap();
                let metadata = CardConceptMetadata::from_tags(&note.tags);
                metadata.prerequisites.iter().any(|prerequisite| {
                    persisted.state.mastery_for(prerequisite, &persisted.config)
                        < persisted.config.outer_fringe_prereq_mastery
                })
            })
            .count()
    }

    /// Study-feature ablation (rubric §8), measured in the REAL engine: with the
    /// concept scheduler ON the new-card queue is sorted by readiness so cards whose
    /// prerequisites are still UNMET are deferred; with it OFF the gather order is
    /// preserved. Mirrors `concept_enabled_sorts_new_cards_by_readiness` but scales to
    /// several ready/unready targets and compares the ON vs OFF counts.
    #[test]
    fn concept_scheduler_defers_unmet_prerequisite_new_cards_ablation() -> Result<()> {
        let mut col = Collection::new();
        let mut deck = col.get_or_create_normal_deck("Default")?;
        deck.normal_mut()?.concept_scheduler_enabled = true;
        col.add_or_update_deck(&mut deck)?;
        col.set_deck_gather_order(&mut deck, NewCardGatherPriority::LowestPosition);
        // Seeds Bioenergetics @0.85 (a MET prereq: >= 0.70) and Glycolysis @0.20
        // (an UNMET prereq: < 0.70), and unlocks readiness sorting.
        seed_concept_readiness(&mut col, deck.id)?;

        // Gather order (LowestPosition == insertion order) places the UNMET-prereq
        // targets first: their only prerequisite is Glycolysis (mastery 0.20 < 0.70).
        for target in [
            "KC::Biochem::Citric_Acid_Cycle",
            "KC::Biochem::Electron_Transport_Chain",
            "KC::Biochem::Oxidative_Phosphorylation",
            "KC::Biochem::Gluconeogenesis",
        ] {
            add_tagged_note(&mut col, deck.id, &[target, "Prereq::Biochem::Glycolysis"])?;
        }
        // ...then the MET-prereq targets, whose prerequisite is Bioenergetics
        // (mastery 0.85 >= 0.70), so the scheduler considers them ready to study.
        for target in [
            "KC::Biochem::Carbohydrate_Metabolism",
            "KC::Biochem::Fatty_Acid_Oxidation",
            "KC::Biochem::Amino_Acid_Metabolism",
            "KC::Biochem::Pentose_Phosphate_Pathway",
        ] {
            add_tagged_note(
                &mut col,
                deck.id,
                &[target, "Prereq::Biochem::Bioenergetics"],
            )?;
        }

        let k = 4;

        // Scheduler ON: readiness sort floats the ready (met-prereq) cards to the front.
        deck.normal_mut()?.concept_scheduler_enabled = true;
        col.add_or_update_deck(&mut deck)?;
        let on_unmet = count_unmet_prereq_new_cards(&mut col, deck.id, k);

        // Scheduler OFF: gather order is preserved, so the unmet-prereq cards stay first.
        deck.normal_mut()?.concept_scheduler_enabled = false;
        col.add_or_update_deck(&mut deck)?;
        let off_unmet = count_unmet_prereq_new_cards(&mut col, deck.id, k);

        eprintln!(
            "PREREQ-VIOLATION ABLATION (first {k} served new cards): \
             scheduler ON unmet-prereq cards = {on_unmet}; OFF unmet-prereq cards = {off_unmet}"
        );

        assert!(
            on_unmet < off_unmet,
            "concept scheduler ON should serve fewer unmet-prerequisite new cards than OFF \
             (on={on_unmet}, off={off_unmet})"
        );

        Ok(())
    }
}
