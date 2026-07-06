#!/usr/bin/env python3
"""Python<->Rust PARITY check: lock the model evals to the REAL shipped engine.

What this proves
----------------
`performance_eval.py` (IRT performance model) and `calibration.py` (memory model)
reimplement, in pure Python, formulas that ship inside the Rust engine
(`anki/rslib/src/scheduler/concept.rs` and `concept_demo.rs`). This test loads a
fixture of reference values emitted BY THAT ENGINE
(`fixtures/engine_parity.json`, written by the Rust test
`concept.rs::tests::emit_engine_parity_fixture`) and asserts the Python eval
functions reproduce every value to within 1e-9. If they diverge, the evals are
no longer measuring the shipped engine, and this test fails (non-zero exit).

What is checked (all against engine-emitted references, tol = 1e-9):
  * difficulty_to_irt_b(d) for d = 1..5                     (performance_eval)
  * IRT 3PL probability_correct over a difficulty x theta x
    discrimination x guessing grid                          (performance_eval)
  * FSRS-5 constants: default decay + derived factor        (calibration)
  * base_recall_for_button(button) for button 1..4          (calibration)
  * memory rating-decay fallback over a grid                (calibration.predict_recall)
  * FSRS-5 forgetting-curve retrievability over a grid      (calibration.predict_recall)

HONEST caveat on the FSRS path
------------------------------
The engine's FSRS path calls the `fsrs` crate (`current_retrievability_seconds`,
computed in f32). The Python eval instead evaluates the ANALYTIC FSRS-5 default
power forgetting curve  R(t) = (1 + FACTOR * t / S) ** (-DECAY)  in f64. These are
the SAME curve only for the default decay (0.5), and the Python side is NOT a
bit-identical crate call. So parity here is asserted against the engine's
`recall_analytic` reference (same analytic formula, same constants) to 1e-9; the
fixture also carries the actual crate value (`recall_fsrs_crate`) and this script
REPORTS -- but does not assert -- the analytic-vs-crate gap, which sits at f32
precision (~1e-7). This is disclosed, not hidden.

Run:  python3 evals/test_parity.py
Prerequisite: run the Rust fixture emitter first, e.g.
  (cd anki && CARGO_TARGET_DIR=out/rust cargo test -p anki --features rustls \
      emit_engine_parity_fixture -- --nocapture)
"""
from __future__ import annotations

import json
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
FIXTURE = HERE / "fixtures" / "engine_parity.json"
ABLATION_FIXTURE = HERE / "fixtures" / "ablation.json"

# Absolute tolerance for "reproduces the engine value". Tight enough that only
# genuine f64 round-off is allowed through, loose enough to survive libm/exp/pow
# implementation differences between Rust and CPython.
TOL = 1e-9

# Import the pure functions under test straight from the eval modules, so we are
# checking the SHIPPED eval code (not a copy).
sys.path.insert(0, str(HERE))
from performance_eval import difficulty_to_irt_b, irt_probability_correct  # noqa: E402
from calibration import (  # noqa: E402
    BASE_RECALL,
    FSRS_DECAY,
    FSRS_FACTOR,
    MEMORY_HORIZON_DAYS,
    predict_recall,
)


class Checker:
    """Accumulates pass/fail counts and the worst mismatch per category."""

    def __init__(self) -> None:
        self.categories: dict[str, dict] = {}

    def check(self, category: str, got: float, expected: float, ctx: str) -> None:
        cat = self.categories.setdefault(
            category, {"passed": 0, "failed": 0, "max_abs_err": 0.0, "failures": []}
        )
        err = abs(float(got) - float(expected))
        cat["max_abs_err"] = max(cat["max_abs_err"], err)
        if err <= TOL:
            cat["passed"] += 1
        else:
            cat["failed"] += 1
            if len(cat["failures"]) < 5:
                cat["failures"].append(
                    f"{ctx}: got {got!r} expected {expected!r} (|err|={err:.3e})"
                )

    def report(self) -> bool:
        total_pass = sum(c["passed"] for c in self.categories.values())
        total_fail = sum(c["failed"] for c in self.categories.values())
        print("\n=== Python<->Rust parity (engine-emitted references, tol = "
              f"{TOL:g}) ===")
        print(f"fixture: {FIXTURE}")
        width = max(len(name) for name in self.categories)
        for name, cat in self.categories.items():
            status = "PASS" if cat["failed"] == 0 else "FAIL"
            print(
                f"  {name.ljust(width)}  {status}  "
                f"pass={cat['passed']:4d}  fail={cat['failed']:4d}  "
                f"max|err|={cat['max_abs_err']:.2e}"
            )
            for line in cat["failures"]:
                print(f"      - {line}")
        print(f"\nTOTAL: pass={total_pass}  fail={total_fail}  "
              f"-> {'ALL PARITY CHECKS PASS' if total_fail == 0 else 'PARITY MISMATCH'}")
        return total_fail == 0


def main() -> int:
    if not FIXTURE.exists():
        print(
            f"ERROR: fixture not found: {FIXTURE}\n"
            "Run the Rust emitter first:\n"
            "  (cd anki && CARGO_TARGET_DIR=out/rust cargo test -p anki "
            "--features rustls emit_engine_parity_fixture -- --nocapture)",
            file=sys.stderr,
        )
        return 2

    fx = json.loads(FIXTURE.read_text(encoding="utf-8"))
    chk = Checker()

    # 1) difficulty_to_irt_b (performance_eval).
    for row in fx["difficulty_to_irt_b"]:
        chk.check(
            "difficulty_to_irt_b",
            difficulty_to_irt_b(row["difficulty"]),
            row["b"],
            f"d={row['difficulty']}",
        )

    # 2) IRT 3PL probability_correct (performance_eval).
    for row in fx["irt_probability_correct"]:
        got = irt_probability_correct(
            theta=row["theta"],
            difficulty=row["difficulty"],
            discrimination=row["discrimination"],
            guessing=row["guessing"],
        )
        chk.check(
            "irt_probability_correct",
            got,
            row["p"],
            f"d={row['difficulty']} theta={row['theta']} a={row['discrimination']} "
            f"c={row['guessing']}",
        )

    # 3) FSRS-5 constants (calibration): the Python analytic curve must be built
    #    from exactly the engine's constants.
    fc = fx["fsrs_constants"]
    chk.check("fsrs_constants", FSRS_DECAY, fc["fsrs5_default_decay"], "FSRS_DECAY")
    chk.check("fsrs_constants", FSRS_FACTOR, fc["fsrs_factor"], "FSRS_FACTOR")
    chk.check(
        "fsrs_constants",
        MEMORY_HORIZON_DAYS,
        fc["memory_horizon_days"],
        "MEMORY_HORIZON_DAYS",
    )

    # 4) base_recall_for_button (calibration).
    for row in fx["base_recall_for_button"]:
        chk.check(
            "base_recall_for_button",
            BASE_RECALL[int(row["button"])],
            row["base"],
            f"button={row['button']}",
        )

    # 5) Memory rating-decay fallback (calibration.predict_recall, no FSRS state).
    for row in fx["memory_fallback"]:
        got = predict_recall(
            {
                "has_fsrs": False,
                "last_rating": row["button"],
                "elapsed_days": row["elapsed_days"],
                "interval_days": row["interval_days"],
            }
        )
        chk.check(
            "memory_fallback",
            got,
            row["recall"],
            f"button={row['button']} elapsed={row['elapsed_days']} "
            f"interval={row['interval_days']}",
        )

    # 6) FSRS-5 forgetting curve (calibration.predict_recall, FSRS path) vs the
    #    engine's ANALYTIC reference. Separately REPORT the analytic-vs-crate gap.
    crate_gap = 0.0
    for row in fx["fsrs_curve"]:
        got = predict_recall(
            {
                "has_fsrs": True,
                "stability": row["stability"],
                "elapsed_days": row["elapsed_days"],
            }
        )
        chk.check(
            "fsrs_curve(analytic)",
            got,
            row["recall_analytic"],
            f"elapsed={row['elapsed_days']} stability={row['stability']}",
        )
        crate_gap = max(crate_gap, abs(row["recall_analytic"] - row["recall_fsrs_crate"]))

    ok = chk.report()
    print(
        "\nFSRS honesty note: the Python eval reproduces the engine's ANALYTIC FSRS-5 "
        "curve to <= 1e-9 (above). The engine itself calls the `fsrs` crate (f32); the "
        f"analytic-vs-crate gap over this grid is max {crate_gap:.2e} (f32 precision) -- "
        "the eval is NOT a bit-identical crate call, only the same curve at the default "
        "decay."
    )

    ablation_ok = check_ablation_lock()
    return 0 if (ok and ablation_ok) else 1


def check_ablation_lock() -> bool:
    """Lock the study-feature ablation (`ABLATION.md`) to the real engine: the ON-arm
    violation count the eval reports must equal the engine's OWN
    `prerequisite_violations` counter, and the pre-committed directional cutoff must
    hold on every seed. Soft-skips if the ablation fixture has not been emitted yet."""
    print("\n=== Ablation engine-lock (fixtures/ablation.json) ===")
    if not ABLATION_FIXTURE.exists():
        print(
            "  SKIP: ablation fixture not found. Emit it with:\n"
            "    (cd anki && CARGO_TARGET_DIR=out/rust cargo test -p anki --features "
            "rustls emit_ablation_fixture -- --nocapture)"
        )
        return True

    fx = json.loads(ABLATION_FIXTURE.read_text(encoding="utf-8"))
    crosscheck = bool(fx["engine_crosscheck"]["on_observer_equals_engine_counter"])
    v = fx["main"]["violations"]
    on, off, plain = v["on"]["values"], v["off"]["values"], v["plain"]["values"]
    per_seed = all(on[i] < off[i] and on[i] < plain[i] for i in range(len(on)))
    print(f"  ON observer == engine prerequisite_violations counter: "
          f"{'PASS' if crosscheck else 'FAIL'}")
    print(f"  per-seed ON < OFF and ON < PLAIN ({len(on)} seeds): "
          f"{'PASS' if per_seed else 'FAIL'}")
    return crosscheck and per_seed


if __name__ == "__main__":
    sys.exit(main())
