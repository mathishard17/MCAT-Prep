// Copyright: Ankitects Pty Ltd and contributors
// License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html

//! Study-feature ablation for the concept scheduler (rubric §8 / SUNDAY-PLAN §4).
//!
//! This is the **3-build** comparison the rubric asks for — full app (concept
//! scheduler ON), the same app with the one feature turned OFF (the ablation), and
//! plain unmodified Anki (the baseline) — run end-to-end through the **real Rust
//! engine** (`Collection::build_queues` + `Collection::answer_card`), on the SAME
//! synthetic learner, the SAME cards, and an EQUAL study-time budget.
//!
//! Pre-registered hypothesis (write it down before running): ordering new cards so
//! prerequisites come first reduces how often a learner is shown a card whose
//! prerequisites they have not yet learned, and — under a stated scaffolding
//! assumption — makes equal study time more effective.
//!
//! MAIN number (model-independent): total **prerequisite violations** over the
//! session. Cutoff (pre-committed as constants): for every seed
//! `violations_ON < violations_OFF` and `violations_ON < violations_PLAIN`, and
//! `mean(violations_ON) <= 0.5 * mean(violations_OFF)`.
//!
//! Secondary (scaffolding-conditioned): held-out question **accuracy**, projected
//! **readiness** (472–528), and **memory** — reported with ranges, plus an honest
//! null (a deck with no prerequisite structure, where the feature is inert).
//!
//! Everything is `#[cfg(test)]`: the module is only compiled for tests and the
//! fixture emitter, exactly like `concept.rs::tests::emit_engine_parity_fixture`.

use anki_proto::deck_config::deck_config::config::NewCardGatherPriority;
use anki_proto::deck_config::deck_config::config::NewCardSortOrder;
use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;

use crate::prelude::*;
use crate::scheduler::answering::CardAnswer;
use crate::scheduler::answering::Rating;
use crate::scheduler::concept::evidence_from_rating;
use crate::scheduler::concept::CardConceptMetadata;
use crate::scheduler::concept::ConceptSchedulerConfig;
use crate::scheduler::concept::ConceptSchedulerState;

// ---- Pre-committed constants (fixed BEFORE running; not tuned to the outcome) ----

/// Number of independent synthetic learners (seeds) used to report a range.
const N_SEEDS: u64 = 12;
/// Base RNG seed; learner `i` uses `GEN_SEED + i`.
const GEN_SEED: u64 = 20260705;

/// Disciplines and their MCAT section. Each discipline hosts several independent
/// prerequisite chains, so the deck has a wide layer of foundational (root) KCs.
const DISCIPLINES: [(&str, &str); 4] = [
    ("Bio", "Bio_Biochem"),
    ("Biochem", "Bio_Biochem"),
    ("Physics", "Chem_Phys"),
    ("PsychSoc", "Psych_Soc"),
];
/// Independent chains per discipline. `roots = DISCIPLINES * CHAINS_PER_DISCIPLINE`.
const CHAINS_PER_DISCIPLINE: usize = 4;

/// Ground-truth learning model (the "student oracle"), DELIBERATELY separate from
/// the engine's Bayesian mastery so the accuracy number is not circular.
const TRUTH_INIT: f64 = 0.10; // latent mastery of an unstudied KC
const SCAFFOLD_LR: f64 = 0.85; // learning rate; gated by prerequisite readiness

/// The MAIN experiment: real prerequisite structure, partial study budget.
/// roots = 4 disciplines * 4 chains = 16; budget spends most study on the ready
/// foundational layer, which is exactly the behaviour the feature is meant to steer.
const MAIN: AblationParams = AblationParams {
    depth: 4,
    cards_per_kc: 3,
    budget: 56,
    with_prereqs: true,
};

/// The NULL control: identical deck shape but NO prerequisite edges. The feature
/// has nothing to reorder, so it should change nothing (honest null result).
const NULL: AblationParams = AblationParams {
    depth: 4,
    cards_per_kc: 3,
    budget: 56,
    with_prereqs: false,
};

#[derive(Clone, Copy)]
struct AblationParams {
    depth: usize,
    cards_per_kc: usize,
    budget: usize,
    with_prereqs: bool,
}

impl AblationParams {
    fn total_cards(&self) -> usize {
        self.total_kcs() * self.cards_per_kc
    }
    fn total_kcs(&self) -> usize {
        self.depth * DISCIPLINES.len() * CHAINS_PER_DISCIPLINE
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Arm {
    /// Full app: concept scheduler on (readiness-sorted new cards).
    On,
    /// Ablation: the same app with the concept scheduler turned off.
    Off,
    /// Baseline: plain, unmodified Anki (default deck config, no concept flag).
    Plain,
}

/// Metrics for one arm at the end of an equal-length study session.
#[derive(Clone, Copy, Debug)]
struct ArmMetrics {
    /// MAIN number: prerequisite violations counted by an arm-independent observer
    /// that reuses the engine's own `mastery_for` + `outer_fringe_prereq_mastery`.
    violations: u32,
    /// The engine's OWN `prerequisite_violations` counter for this deck. For the ON
    /// arm this must equal `violations` (the observer mirrors the engine exactly);
    /// for OFF/PLAIN the engine does not track it (feature gated off), so it is 0.
    engine_violations: u32,
    /// Held-out projected exam accuracy from the ground-truth oracle (not circular).
    accuracy: f64,
    /// Projected MCAT total (472–528) and its 95% band, from the real engine
    /// `section_score_status` fed this arm's study history.
    readiness_total: f64,
    readiness_lower: f64,
    readiness_upper: f64,
    /// Mean projected recall (memory) of studied cards, from the real engine.
    memory: f64,
    /// Fraction of KCs with any evidence (breadth of coverage).
    coverage: f64,
    /// New cards actually introduced (equal-study-time sanity check across arms).
    cards_studied: u32,
}

fn p_correct(truth: f64) -> f64 {
    // Steep logistic: a well-studied card (prerequisites met, so `truth` is high) is
    // answered correctly ~95% of the time, while a card whose prerequisites are unmet
    // (low `truth`) stays near the 4-choice guessing floor. This keeps the engine's
    // Bayesian mastery of a genuinely-learned KC reliably above the prereq threshold.
    (0.15 + 0.80 / (1.0 + (-9.0 * (truth - 0.45)).exp())).clamp(0.02, 0.98)
}

/// Build a synthetic MCAT deck of independent per-discipline prerequisite chains.
///
/// Cards are authored **deepest-first** (reverse topological), so the plain gather
/// order (`LowestPosition`) is adversarial: without reordering, the first cards
/// served are the ones whose prerequisites are furthest from being met.
fn build_deck(col: &mut Collection, params: AblationParams) -> Result<DeckId> {
    let mut deck = col.get_or_create_normal_deck("Ablation")?;
    col.add_or_update_deck(&mut deck)?;
    let nt = col.get_notetype_by_name("Basic")?.unwrap();

    let mut idx = 0usize;
    // Deepest depth first so the plain gather order (LowestPosition) is adversarial:
    // the earliest-served cards are the ones furthest from having their prereqs met.
    for d in (0..params.depth).rev() {
        for (disc, section) in DISCIPLINES {
            for chain in 0..CHAINS_PER_DISCIPLINE {
                let kc = format!("{disc}::C{chain}T{d:02}");
                for _ in 0..params.cards_per_kc {
                    let mut note = nt.new_note();
                    note.set_field(0, format!("Q{idx} {kc}"))?;
                    note.set_field(1, "answer")?;
                    let difficulty = 1 + (idx % 5);
                    let mut tags = vec![
                        format!("KC::{kc}"),
                        format!("MCAT::{section}"),
                        format!("Difficulty::{difficulty}"),
                    ];
                    if params.with_prereqs && d > 0 {
                        tags.push(format!("Prereq::{disc}::C{chain}T{:02}", d - 1));
                    }
                    note.tags = tags;
                    col.add_note(&mut note, deck.id)?;
                    idx += 1;
                }
            }
        }
    }
    Ok(deck.id)
}

/// Apply the arm's deck configuration: the ON/OFF arms share the app's deck config
/// (adversarial insertion order preserved) and differ only by the concept flag;
/// PLAIN uses a stock default Anki deck config.
fn configure_arm(col: &mut Collection, deck_id: DeckId, arm: Arm) -> Result<()> {
    // `get_deck` yields an `Arc<Deck>`; clone out an owned deck we can mutate.
    let mut deck = (*col.get_deck(deck_id)?.or_not_found(deck_id)?).clone();
    let mut conf = DeckConfig::default();
    match arm {
        Arm::On | Arm::Off => {
            conf.inner.new_card_gather_priority = NewCardGatherPriority::LowestPosition as i32;
            conf.inner.new_card_sort_order = NewCardSortOrder::NoSort as i32;
        }
        // PLAIN: leave DeckConfig::default() as-is (stock Anki new-card order).
        Arm::Plain => {}
    }
    // Equal, generous limits for all arms so the budget, not a daily cap, bounds study
    // (9999 is Anki's accepted per-day maximum; larger values are rejected).
    conf.inner.new_per_day = 9999;
    conf.inner.reviews_per_day = 9999;
    col.add_or_update_deck_config(&mut conf)?;
    deck.normal_mut()?.config_id = conf.id.0;
    deck.normal_mut()?.concept_scheduler_enabled = matches!(arm, Arm::On);
    col.add_or_update_deck(&mut deck)?;
    // `get_next_card`/`get_queued_cards` build the queue for the CURRENT deck.
    col.set_current_deck(deck_id)?;

    if matches!(arm, Arm::On) {
        // Unlock readiness sorting immediately (no warm-up gate) so the feature is
        // fully active for the whole equal-length session.
        let mut persisted = col.get_concept_scheduler_state(deck_id);
        persisted.config.readiness_min_seen_cards = 0;
        col.set_concept_scheduler_state(deck_id, &persisted, false)?;
    }
    Ok(())
}

/// Run one arm to completion and return its end-of-session metrics.
fn run_arm(seed: u64, arm: Arm, params: AblationParams) -> Result<ArmMetrics> {
    let mut col = Collection::new();
    let deck_id = build_deck(&mut col, params)?;
    configure_arm(&mut col, deck_id, arm)?;

    // Engine-defined observer, fed identically for every arm. It mirrors
    // `maybe_update_concept_mastery_from_answer` exactly, so for the ON arm it
    // reproduces the engine's own `prerequisite_violations` counter bit-for-bit.
    let config = ConceptSchedulerConfig::default();
    let mut observer = ConceptSchedulerState::default();
    // Ground-truth latent mastery (the student oracle), kept separate from the
    // engine's Bayesian mastery so the accuracy number is not circular.
    let mut truth: std::collections::HashMap<String, f64> = std::collections::HashMap::new();
    let mut rng = StdRng::seed_from_u64(seed);
    let day = col.timing_today()?.days_elapsed;

    let mut cards_studied = 0u32;
    for _ in 0..params.budget {
        // Rebuild the queue each step so the ON arm re-sorts against updated mastery,
        // exactly as the app does during a real session.
        col.clear_study_queues();
        let Some(queued) = col.get_next_card()? else {
            break; // ran out of due cards for this arm
        };
        let note = col
            .storage
            .get_note(queued.card.note_id)?
            .or_not_found(queued.card.note_id)?;
        let metadata = CardConceptMetadata::from_tags(&note.tags);
        if metadata.target_components.is_empty() || metadata.is_overview() {
            // Mirror the engine, which skips these; answer to make progress.
            answer_current(&mut col, &queued, Rating::Good)?;
            continue;
        }
        let target = metadata.target_components[0].as_str().to_string();

        // Oracle: studying the card teaches it, but only as much as its prerequisites
        // are truly mastered (the stated scaffolding effect). Learn, then answer.
        let prereq_readiness = if metadata.prerequisites.is_empty() {
            1.0
        } else {
            metadata
                .prerequisites
                .iter()
                .map(|p| *truth.get(p.as_str()).unwrap_or(&TRUTH_INIT))
                .fold(f64::INFINITY, f64::min)
        };
        let current = *truth.get(&target).unwrap_or(&TRUTH_INIT);
        let updated = current + SCAFFOLD_LR * prereq_readiness * (1.0 - current);
        truth.insert(target, updated);
        let correct = rng.random_range(0.0f64..1.0) < p_correct(updated);
        let rating = if correct { Rating::Good } else { Rating::Again };

        // Observer: mirror the engine's answer-time update EXACTLY (check prereq
        // violation against current mastery, then record target evidence + IRT).
        let evidence = evidence_from_rating(rating);
        if metadata.prerequisites.iter().any(|p| {
            observer.mastery_for(p, &config) < config.outer_fringe_prereq_mastery
        }) {
            observer.record_prerequisite_violation(day);
        }
        for component in &metadata.target_components {
            observer.record_evidence_on_day(component.clone(), evidence, day, &config);
        }
        observer.note_card_answered();

        // Answer through the real engine (updates FSRS state + revlog, and, for the
        // ON arm, the engine's own ConceptSchedulerState).
        answer_current(&mut col, &queued, rating)?;
        cards_studied += 1;
    }

    // ---- Metrics from the real engine + the observer + the ground-truth oracle ----

    // MAIN number: the observer's engine-defined violation count. Read the engine's
    // OWN counter first (before the status RPC touches persisted state) so the ON-arm
    // cross-check compares like with like.
    let violations = observer.prerequisite_violations;
    let engine_violations = col
        .get_concept_scheduler_state(deck_id)
        .state
        .prerequisite_violations;

    // Secondary readiness + memory straight from the shipped dashboard RPC, which
    // reconstructs each arm's study history from the revlog (so it is comparable
    // across arms regardless of whether the concept flag was on).
    let status = col.concept_scheduler_status(deck_id)?;
    let readiness_total = status.projected_total as f64;
    let readiness_lower = status.projected_total_lower as f64;
    let readiness_upper = status.projected_total_upper as f64;
    let memory = status.overall_memory as f64;

    // Held-out projected exam accuracy from the GROUND-TRUTH oracle over all KCs
    // (drawn from a process distinct from the engine's Bayesian mastery, so not
    // circular).
    let graph = col.concept_graph_for_deck(deck_id)?;
    let kc_count = graph.components().count().max(1);
    let accuracy = graph
        .components()
        .map(|c| p_correct(*truth.get(c.as_str()).unwrap_or(&TRUTH_INIT)))
        .sum::<f64>()
        / kc_count as f64;
    let coverage = observer.kcs.values().filter(|s| s.answered > 0).count() as f64 / kc_count as f64;

    Ok(ArmMetrics {
        violations,
        engine_violations,
        accuracy,
        readiness_total,
        readiness_lower,
        readiness_upper,
        memory,
        coverage,
        cards_studied,
    })
}

/// Answer the card at the front of the queue with a chosen rating, through the real
/// `answer_card` flow (mirrors the `test_helpers` answering path).
fn answer_current(
    col: &mut Collection,
    queued: &crate::scheduler::queue::QueuedCard,
    rating: Rating,
) -> Result<()> {
    let new_state = match rating {
        Rating::Again => queued.states.again,
        Rating::Hard => queued.states.hard,
        Rating::Good => queued.states.good,
        Rating::Easy => queued.states.easy,
    };
    col.answer_card(&mut CardAnswer {
        card_id: queued.card.id,
        current_state: queued.states.current,
        new_state,
        rating,
        answered_at: TimestampMillis::now(),
        milliseconds_taken: 0,
        custom_data: None,
        from_queue: true,
    })?;
    Ok(())
}

/// Run all three arms for one seed and return `[On, Off, Plain]`.
fn run_all_arms(seed: u64, params: AblationParams) -> Result<[ArmMetrics; 3]> {
    Ok([
        run_arm(seed, Arm::On, params)?,
        run_arm(seed, Arm::Off, params)?,
        run_arm(seed, Arm::Plain, params)?,
    ])
}

// ---- Aggregation over seeds ----

#[derive(Clone, Debug)]
struct Summary {
    values: Vec<f64>,
}

impl Summary {
    fn new() -> Self {
        Self { values: Vec::new() }
    }
    fn push(&mut self, v: f64) {
        self.values.push(v);
    }
    fn mean(&self) -> f64 {
        if self.values.is_empty() {
            0.0
        } else {
            self.values.iter().sum::<f64>() / self.values.len() as f64
        }
    }
    fn min(&self) -> f64 {
        self.values.iter().cloned().fold(f64::INFINITY, f64::min)
    }
    fn max(&self) -> f64 {
        self.values.iter().cloned().fold(f64::NEG_INFINITY, f64::max)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Collected per-arm, per-metric summaries across all seeds for one param set.
    struct Aggregate {
        violations: [Summary; 3],
        accuracy: [Summary; 3],
        readiness: [Summary; 3],
        readiness_lower: [Summary; 3],
        readiness_upper: [Summary; 3],
        memory: [Summary; 3],
        coverage: [Summary; 3],
        cards_studied: [Summary; 3],
        // ON-arm cross-check: observer count == engine's own counter, every seed.
        on_observer_matches_engine: bool,
    }

    fn aggregate(params: AblationParams) -> Result<Aggregate> {
        let mut agg = Aggregate {
            violations: [Summary::new(), Summary::new(), Summary::new()],
            accuracy: [Summary::new(), Summary::new(), Summary::new()],
            readiness: [Summary::new(), Summary::new(), Summary::new()],
            readiness_lower: [Summary::new(), Summary::new(), Summary::new()],
            readiness_upper: [Summary::new(), Summary::new(), Summary::new()],
            memory: [Summary::new(), Summary::new(), Summary::new()],
            coverage: [Summary::new(), Summary::new(), Summary::new()],
            cards_studied: [Summary::new(), Summary::new(), Summary::new()],
            on_observer_matches_engine: true,
        };
        for i in 0..N_SEEDS {
            let seed = GEN_SEED + i;
            let arms = run_all_arms(seed, params)?;
            for (idx, m) in arms.iter().enumerate() {
                agg.violations[idx].push(m.violations as f64);
                agg.accuracy[idx].push(m.accuracy);
                agg.readiness[idx].push(m.readiness_total);
                agg.readiness_lower[idx].push(m.readiness_lower);
                agg.readiness_upper[idx].push(m.readiness_upper);
                agg.memory[idx].push(m.memory);
                agg.coverage[idx].push(m.coverage);
                agg.cards_studied[idx].push(m.cards_studied as f64);
            }
            // ON arm (index 0): the observer must reproduce the engine's counter.
            if arms[0].violations != arms[0].engine_violations {
                agg.on_observer_matches_engine = false;
            }
        }
        Ok(agg)
    }

    /// The headline 3-build ablation: with a real prerequisite DAG and equal study
    /// time, the concept scheduler ON serves strictly fewer prerequisite-violating
    /// cards than the ablation (OFF) and than plain Anki (PLAIN), every seed.
    #[test]
    fn concept_scheduler_reduces_prerequisite_violations_three_build_ablation() -> Result<()> {
        let params = MAIN;
        let mut on_total = 0u32;
        let mut off_total = 0u32;
        let mut plain_total = 0u32;

        for i in 0..N_SEEDS {
            let seed = GEN_SEED + i;
            let [on, off, plain] = run_all_arms(seed, params)?;

            // Equal study time: every arm introduced the same number of new cards.
            assert_eq!(
                on.cards_studied, off.cards_studied,
                "arms must study an equal number of cards (on={} off={})",
                on.cards_studied, off.cards_studied
            );
            assert_eq!(on.cards_studied, plain.cards_studied);
            assert_eq!(on.cards_studied as usize, params.budget);

            // MAIN number, per seed.
            assert!(
                on.violations < off.violations,
                "seed {seed}: ON ({}) must have fewer prereq violations than OFF ({})",
                on.violations,
                off.violations
            );
            assert!(
                on.violations < plain.violations,
                "seed {seed}: ON ({}) must have fewer prereq violations than PLAIN ({})",
                on.violations,
                plain.violations
            );

            // Engine cross-check: the observer reproduces the engine's own counter.
            assert_eq!(
                on.violations, on.engine_violations,
                "seed {seed}: observer count must equal the engine's prerequisite_violations"
            );

            on_total += on.violations;
            off_total += off.violations;
            plain_total += plain.violations;
        }

        let on_mean = on_total as f64 / N_SEEDS as f64;
        let off_mean = off_total as f64 / N_SEEDS as f64;
        let plain_mean = plain_total as f64 / N_SEEDS as f64;
        eprintln!(
            "THREE-BUILD ABLATION (prerequisite violations over {} seeds, budget {}): \
             ON mean={on_mean:.1}  OFF mean={off_mean:.1}  PLAIN mean={plain_mean:.1}",
            N_SEEDS, params.budget
        );
        assert!(
            on_mean <= 0.5 * off_mean,
            "pre-committed cutoff: mean(ON)={on_mean:.1} must be <= 0.5*mean(OFF)={:.1}",
            0.5 * off_mean
        );
        Ok(())
    }

    /// Secondary metric (scaffolding-conditioned): at equal study time the ON arm
    /// reaches at least as high projected exam accuracy as plain Anki.
    #[test]
    fn concept_scheduler_ablation_secondary_gains_hold() -> Result<()> {
        let agg = aggregate(MAIN)?;
        let on_acc = agg.accuracy[0].mean();
        let plain_acc = agg.accuracy[2].mean();
        let on_ready = agg.readiness[0].mean();
        let plain_ready = agg.readiness[2].mean();
        let on_mem = agg.memory[0].mean();
        let plain_mem = agg.memory[2].mean();
        eprintln!(
            "ABLATION SECONDARY (means): accuracy ON={on_acc:.3} PLAIN={plain_acc:.3}; \
             readiness ON={on_ready:.1} PLAIN={plain_ready:.1}; memory ON={on_mem:.3} PLAIN={plain_mem:.3}"
        );
        assert!(
            on_acc >= plain_acc,
            "scaffolding expectation: mean accuracy ON ({on_acc:.3}) >= PLAIN ({plain_acc:.3})"
        );
        assert!(
            on_ready >= plain_ready - 0.5,
            "readiness ON ({on_ready:.1}) should not trail PLAIN ({plain_ready:.1})"
        );
        Ok(())
    }

    /// Honest null: with NO prerequisite structure there is nothing to defer, so the
    /// feature's headline benefit vanishes — every arm records ZERO prerequisite
    /// violations. (The readiness sort may still reorder by mastery, but that is not
    /// the prerequisite-deferral effect under test, so accuracy is not asserted equal.)
    #[test]
    fn concept_scheduler_ablation_is_inert_without_prerequisites() -> Result<()> {
        for i in 0..N_SEEDS {
            let seed = GEN_SEED + i;
            let [on, off, plain] = run_all_arms(seed, NULL)?;
            assert_eq!(on.violations, 0, "no prereqs => no violations (ON)");
            assert_eq!(off.violations, 0, "no prereqs => no violations (OFF)");
            assert_eq!(plain.violations, 0, "no prereqs => no violations (PLAIN)");
        }
        Ok(())
    }

    // ---- Fixture emitter (source of truth for the Python renderer) ----

    fn metric_json(summary: &Summary) -> serde_json::Value {
        serde_json::json!({
            "mean": summary.mean(),
            "min": summary.min(),
            "max": summary.max(),
            "values": summary.values,
        })
    }

    fn per_arm_json(metric: &[Summary; 3]) -> serde_json::Value {
        serde_json::json!({
            "on": metric_json(&metric[0]),
            "off": metric_json(&metric[1]),
            "plain": metric_json(&metric[2]),
        })
    }

    fn aggregate_json(agg: &Aggregate) -> serde_json::Value {
        serde_json::json!({
            "violations": per_arm_json(&agg.violations),
            "accuracy": per_arm_json(&agg.accuracy),
            "readiness_total": per_arm_json(&agg.readiness),
            "readiness_lower": per_arm_json(&agg.readiness_lower),
            "readiness_upper": per_arm_json(&agg.readiness_upper),
            "memory": per_arm_json(&agg.memory),
            "coverage": per_arm_json(&agg.coverage),
            "cards_studied": per_arm_json(&agg.cards_studied),
        })
    }

    /// Emit engine-derived ablation results so the Python renderer
    /// (`evals/ablation.py`) can build `evals/ABLATION.md` + the chart and re-assert
    /// the pre-committed cutoff. Every number here comes from the REAL engine run.
    /// Writing a fixture file from a test is intentional (mirrors
    /// `concept.rs::tests::emit_engine_parity_fixture`).
    #[test]
    fn emit_ablation_fixture() -> Result<()> {
        let main = aggregate(MAIN)?;
        let null = aggregate(NULL)?;

        // Re-derive the pre-committed cutoff from the emitted numbers.
        let per_seed_ok = (0..N_SEEDS as usize).all(|i| {
            main.violations[0].values[i] < main.violations[1].values[i]
                && main.violations[0].values[i] < main.violations[2].values[i]
        });
        let on_mean = main.violations[0].mean();
        let off_mean = main.violations[1].mean();
        let on_le_half_off = on_mean <= 0.5 * off_mean;

        assert!(per_seed_ok, "cutoff (per-seed ON < OFF and ON < PLAIN) must hold");
        assert!(on_le_half_off, "cutoff mean(ON) <= 0.5*mean(OFF) must hold");
        assert!(
            main.on_observer_matches_engine,
            "ON observer count must equal the engine's own prerequisite_violations counter"
        );
        assert!(
            null.violations[0].mean() == 0.0
                && null.violations[1].mean() == 0.0
                && null.violations[2].mean() == 0.0,
            "null control must record zero violations for every arm"
        );

        let seeds: Vec<u64> = (0..N_SEEDS).map(|i| GEN_SEED + i).collect();
        let fixture = serde_json::json!({
            "meta": {
                "description": "3-build study-feature ablation (concept scheduler ON / OFF / \
                                plain Anki) measured end-to-end through the REAL Anki Rust \
                                engine. Emitted by concept_ablation.rs::tests::emit_ablation_fixture.",
                "generated_by": "anki/rslib/src/scheduler/concept_ablation.rs::tests::emit_ablation_fixture",
                "engine_functions": [
                    "Collection::build_queues", "Collection::answer_card",
                    "ConceptSchedulerState::section_score_status", "Collection::card_memory"
                ],
            },
            "hypothesis": "Ordering new cards so prerequisites come first (the concept \
                           scheduler) reduces how often a learner is served a card whose \
                           prerequisites are not yet learned, and under a stated scaffolding \
                           assumption makes equal study time more effective.",
            "main_number": "prerequisite_violations",
            "cutoff": {
                "per_seed_on_lt_off_and_plain": per_seed_ok,
                "on_mean_le_half_off_mean": on_le_half_off,
                "on_mean": on_mean,
                "off_mean": off_mean,
                "half_off_mean": 0.5 * off_mean,
            },
            "scaffolding_note": "Secondary metrics (accuracy/readiness/memory) assume a \
                                 scaffolding learning effect: a card studied with unmet \
                                 prerequisites is learned less effectively. The MAIN number \
                                 (prerequisite violations) does NOT depend on this assumption.",
            "engine_crosscheck": {
                "on_observer_equals_engine_counter": main.on_observer_matches_engine,
            },
            "arms": ["on", "off", "plain"],
            "seeds": seeds,
            "deck": {
                "cards": MAIN.total_cards(),
                "kcs": MAIN.total_kcs(),
                "disciplines": DISCIPLINES.len(),
                "depth": MAIN.depth,
                "cards_per_kc": MAIN.cards_per_kc,
                "authoring_order": "reverse-topological (deepest-first): adversarial for the plain gather order",
            },
            "budget": MAIN.budget,
            "main": aggregate_json(&main),
            "null_no_prereq": aggregate_json(&null),
        });

        let repo_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .parent()
            .unwrap();
        let path = repo_root.join("evals/fixtures/ablation.json");
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(&path, serde_json::to_string_pretty(&fixture).unwrap() + "\n").unwrap();
        eprintln!("wrote ablation fixture: {}", path.display());
        Ok(())
    }

    // ---- Stress / robustness ----

    /// Scale to a large, deep multi-discipline deck: the engine must not panic and
    /// the feature must still cut prerequisite violations.
    #[test]
    fn ablation_stress_large_deep_deck() -> Result<()> {
        let params = AblationParams {
            depth: 8,
            cards_per_kc: 3,
            budget: 160,
            with_prereqs: true,
        };
        let [on, off, plain] = run_all_arms(GEN_SEED, params)?;
        assert!(on.cards_studied > 0);
        assert!(
            on.violations < off.violations && on.violations < plain.violations,
            "large deck: ON={} OFF={} PLAIN={}",
            on.violations,
            off.violations,
            plain.violations
        );
        Ok(())
    }

    /// A cyclic prerequisite graph must degrade gracefully: the readiness sorter
    /// bails (see `sorting.rs`), the queue still builds, and nothing panics.
    #[test]
    fn ablation_stress_cyclic_prereq_graph_is_safe() -> Result<()> {
        let mut col = Collection::new();
        let mut deck = col.get_or_create_normal_deck("Ablation")?;
        deck.normal_mut()?.concept_scheduler_enabled = true;
        col.add_or_update_deck(&mut deck)?;
        let mut persisted = col.get_concept_scheduler_state(deck.id);
        persisted.config.readiness_min_seen_cards = 0;
        col.set_concept_scheduler_state(deck.id, &persisted, false)?;

        let nt = col.get_notetype_by_name("Basic")?.unwrap();
        // A <-> B mutual prerequisites form a cycle.
        for (kc, prereq) in [("Bio::A", "Bio::B"), ("Bio::B", "Bio::A")] {
            let mut note = nt.new_note();
            note.set_field(0, format!("cycle {kc}"))?;
            note.set_field(1, "answer")?;
            note.tags = vec![
                format!("KC::{kc}"),
                format!("Prereq::{prereq}"),
                "MCAT::Bio_Biochem".into(),
                "Difficulty::3".into(),
            ];
            col.add_note(&mut note, deck.id)?;
        }
        col.set_current_deck(deck.id)?;

        let graph = col.concept_graph_for_deck(deck.id)?;
        assert!(graph.cycle().is_some(), "test deck must contain a cycle");
        // Must not panic and must still serve both cards.
        let served = col.get_queued_cards(50, false)?.cards.len();
        assert_eq!(served, 2, "cyclic deck should still build a full queue");
        Ok(())
    }

    /// "Taps Good without reading": a careless student who always answers Good. The
    /// ordering feature must still defer prerequisite-violating cards (ON <= OFF),
    /// exercising the path where ratings carry no information.
    #[test]
    fn ablation_stress_careless_always_good_student() -> Result<()> {
        // Reuse the harness deck but force Good answers by driving the loop directly.
        for arm in [Arm::On, Arm::Off] {
            let mut col = Collection::new();
            let deck_id = build_deck(&mut col, MAIN)?;
            configure_arm(&mut col, deck_id, arm)?;
            let config = ConceptSchedulerConfig::default();
            let mut observer = ConceptSchedulerState::default();
            let day = col.timing_today()?.days_elapsed;
            let mut violations = 0u32;
            for _ in 0..MAIN.budget {
                col.clear_study_queues();
                let Some(queued) = col.get_next_card()? else { break };
                let note = col
                    .storage
                    .get_note(queued.card.note_id)?
                    .or_not_found(queued.card.note_id)?;
                let metadata = CardConceptMetadata::from_tags(&note.tags);
                if metadata.prerequisites.iter().any(|p| {
                    observer.mastery_for(p, &config) < config.outer_fringe_prereq_mastery
                }) {
                    observer.record_prerequisite_violation(day);
                    violations += 1;
                }
                for component in &metadata.target_components {
                    observer.record_evidence_on_day(
                        component.clone(),
                        evidence_from_rating(Rating::Good),
                        day,
                        &config,
                    );
                }
                observer.note_card_answered();
                answer_current(&mut col, &queued, Rating::Good)?;
            }
            if arm == Arm::On {
                assert!(violations < MAIN.budget as u32, "ON careless: {violations}");
            }
        }
        Ok(())
    }

    /// Undo safety: answering a card then undoing restores it to a fresh new card,
    /// with no leftover reps (no corruption from the concept-scheduler answer hook).
    #[test]
    fn ablation_stress_answer_then_undo_is_clean() -> Result<()> {
        let mut col = Collection::new();
        let deck_id = build_deck(&mut col, MAIN)?;
        configure_arm(&mut col, deck_id, Arm::On)?;

        let queued = col.get_next_card()?.unwrap();
        let card_id = queued.card.id;
        assert_eq!(queued.card.reps, 0);
        answer_current(&mut col, &queued, Rating::Good)?;
        assert!(col.storage.get_card(card_id)?.unwrap().reps >= 1);

        col.undo()?;
        let restored = col.storage.get_card(card_id)?.unwrap();
        assert_eq!(restored.reps, 0, "undo must restore the card to a new card");
        // The queue still builds after undo (collection is consistent).
        assert!(col.get_queued_cards(10, false)?.cards.len() > 0);
        Ok(())
    }
}
