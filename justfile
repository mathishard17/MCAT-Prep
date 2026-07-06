set shell := ["bash", "-cu"]

default:
    just --list

# Offline resilience test: AI degrades cleanly + scores stay present with no network
offline-test:
    cd anki && PYTHONPATH=out/pylib out/pyenv/bin/python ../evals/offline_test.py

desktop:
    cd anki && just run

desktop-check:
    cd anki && just check

desktop-installer:
    cd anki && ./tools/build-installer

# Section 7 reliability: kill a mid-review process 20x, prove zero corrupted collections.
crash-test:
    cd anki && PYTHONPATH=out/pylib out/pyenv/bin/python ../evals/crash_test.py

# Section 7 one-command ENGINE benchmark (real Rust backend, 50k-card deck).
# Writes evals/BENCHMARK.md + evals/bench-latency.svg, prints the table, and
# exits non-zero if any latency/memory target is missed.
bench:
    cd anki && PYTHONPATH=out/pylib out/pyenv/bin/python ../evals/bench.py

backend:
    cd Anki-Android-Backend && ./build.sh

android-install:
    cd Anki-Android && ./gradlew :AnkiDroid:installPlayDebug

android-check:
    cd Anki-Android && ./gradlew test

rebuild-local-backend:
    ./scripts/rebuild-ankidroid-local-backend.sh

# Study-feature ablation (rubric §8 / SUNDAY-PLAN §4): emit the 3-build results
# from the REAL Rust engine, then render evals/ABLATION.md + evals/ablation.svg
# and re-assert the pre-committed cutoff.
ablation:
    cd anki && CARGO_TARGET_DIR=out/rust cargo test -p anki --features rustls \
        emit_ablation_fixture -- --nocapture
    python3 evals/ablation.py

# Concept-scheduler stress / robustness suite (large deck, cyclic graph,
# adversarial order, careless student, answer+undo) plus the 3-build ablation tests.
stress:
    cd anki && CARGO_TARGET_DIR=out/rust cargo test -p anki --features rustls \
        scheduler::concept_ablation -- --nocapture

