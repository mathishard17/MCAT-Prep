"""Crash-test child: open a collection and hammer the real v3 scheduler.

Spawned by ``evals/crash_test.py`` (never run directly). It opens the collection
COPY it is handed and then continuously answers due cards through the shipped v3
scheduler (``anki/pylib/anki/scheduler/v3.py`` -> ``answer_card``), so the Rust
backend is committing card + revlog writes to SQLite the whole time. The parent
SIGKILLs this process mid-loop; because the writes are real and frequent, the kill
lands mid-write, which is exactly what the durability check exercises.

Protocol: once the collection is open and the first card has been answered (the
write path is warm), we print a single ``READY`` line to stdout and flush it. The
parent blocks on that line before starting its random kill timer, guaranteeing the
kill lands during active reviewing rather than during interpreter/backend startup.

Usage: <app pyenv python> _crash_child.py <collection_path> [seed]
"""

from __future__ import annotations

import os
import random
import sys

# Import the shared fixture (which also makes the built anki backend importable).
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import _bench_fixture as fx

# Answering all daily-limited cards would drain the queue in a fraction of a second,
# after which the process would idle and a kill could no longer land mid-write. Lift
# the deck's per-day new/review caps so the ~2000-card deck stays answerable for the
# whole (sub-second) lifetime of this process.
HIGH_LIMIT = 1_000_000
# Defensive only: if the queue ever does empty, recycle a batch of cards back to new
# (a genuine scheduler write) so we never stop mutating the collection.
RECYCLE_BATCH = 400


def _lift_daily_limits(col, did: int) -> None:
    deck = col.decks.get(did)
    if deck is None:
        return
    deck["newLimit"] = HIGH_LIMIT
    deck["reviewLimit"] = HIGH_LIMIT
    col.decks.save(deck)


def main(argv: list[str]) -> int:
    if len(argv) < 2:
        sys.stderr.write("usage: _crash_child.py <collection_path> [seed]\n")
        return 2
    path = argv[1]
    seed = int(argv[2]) if len(argv) > 2 else 0
    rnd = random.Random(seed)

    col = fx.open_collection(path)
    signalled_ready = False
    try:
        did = col.decks.id(fx.DECK_NAME)
        col.decks.select(did)
        _lift_daily_limits(col, did)

        while True:
            card = col.sched.getCard()
            if card is None:
                cids = col.db.list(
                    "select id from cards where did = ? limit ?", did, RECYCLE_BATCH
                )
                if not cids:
                    break
                col.sched.schedule_cards_as_new(cids, reset_counts=True)
                continue

            # Real v3 answer: builds the answer from the backend scheduling states and
            # commits the card update + revlog row. A random rating exercises the
            # again/hard/good/easy transitions (new -> learning/review).
            col.sched.answerCard(card, rnd.choice([1, 2, 3, 4]))

            if not signalled_ready:
                # The write path is warm; tell the parent it may start its kill timer.
                signalled_ready = True
                try:
                    sys.stdout.write("READY\n")
                    sys.stdout.flush()
                except Exception:
                    pass
    except KeyboardInterrupt:
        pass
    except Exception:
        # Any abrupt failure is fine here -- the parent kills us on purpose, and a
        # clean exit (e.g. queue exhausted) is equally acceptable.
        pass
    finally:
        try:
            col.close()
        except Exception:
            pass
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv))
