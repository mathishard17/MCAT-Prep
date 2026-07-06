"""Shared fixture for the Section 7 reliability evals (benchmark / crash / offline).

Builds (and caches) a large MCAT deck in a real Anki collection so the other
scripts can time / stress the actual Rust backend, not a reimplementation.

Run with the built anki backend (the app's pyenv). The canonical entry points are
the `just` recipes (`just bench`, `just crash-test`, `just offline-test`); to run a
script directly:

    cd anki && PYTHONPATH=out/pylib out/pyenv/bin/python ../evals/<script>.py

Smoke test:

    cd anki && PYTHONPATH=out/pylib out/pyenv/bin/python ../evals/_bench_fixture.py --smoke
"""

from __future__ import annotations

import argparse
import os
import sys

REPO = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
ANKI = os.path.join(REPO, "anki")

# Make the built anki backend importable: the generated protobufs + the compiled
# `_rsbridge.so` live under anki/out/pylib, while the package source is anki/pylib.
for _p in (os.path.join(ANKI, "out", "pylib"), os.path.join(ANKI, "pylib")):
    if os.path.isdir(_p) and _p not in sys.path:
        sys.path.insert(0, _p)

try:
    from anki.collection import Collection
except Exception as exc:  # pragma: no cover - environment guard
    sys.stderr.write(
        "\nCould not import the built anki backend.\n"
        "Run through the app's pyenv, e.g.:\n"
        "  cd anki && PYTHONPATH=out/pylib out/pyenv/bin/python ../evals/"
        + os.path.basename(sys.argv[0])
        + "\n(or use the `just` recipe). Build the desktop app once first if needed.\n"
        f"import error: {exc}\n"
    )
    raise

SEED = 20260705
DECK_NAME = "MCAT Bench"
CACHE_DIR = os.path.join(REPO, "evals", ".bench_cache")

_SECTIONS = [
    ("Bio", "Bio_Biochem"),
    ("Biochem", "Bio_Biochem"),
    ("GenChem", "Chem_Phys"),
    ("Orgo", "Chem_Phys"),
    ("Physics", "Chem_Phys"),
    ("PsychSoc", "Psych_Soc"),
]
# ~20 topics per discipline -> ~120 KCs, each a short prerequisite chain, so the
# knowledge graph the dashboard builds has a realistic node+edge count.
_TOPICS_PER_DISCIPLINE = 20


def kc_ids() -> list[str]:
    ids: list[str] = []
    for disc, _section in _SECTIONS:
        for t in range(_TOPICS_PER_DISCIPLINE):
            ids.append(f"{disc}::T{t:02d}")
    return ids


def _section_for(kc: str) -> str:
    disc = kc.split("::", 1)[0]
    for d, section in _SECTIONS:
        if d == disc:
            return section
    return "Bio_Biochem"


def _prereq_for(kc: str) -> str | None:
    # Chain each topic to the previous one in its discipline (T00 is a root).
    disc, t = kc.split("::T")
    idx = int(t)
    if idx == 0:
        return None
    return f"{disc}::T{idx - 1:02d}"


def _tags(kc: str, difficulty: int) -> list[str]:
    tags = [f"KC::{kc}", f"MCAT::{_section_for(kc)}", f"Difficulty::{difficulty}"]
    prereq = _prereq_for(kc)
    if prereq:
        tags.append(f"Prereq::{prereq}")
    return tags


def _front(i: int, kc: str) -> str:
    topic = kc.replace("::", " ")
    return (
        f"Bench question {i} about {topic}. Which option is correct?"
        "<br><br>A. First option"
        "<br>B. Second option"
        "<br>C. Third option"
        "<br>D. Fourth option"
    )


def _back(i: int) -> str:
    letter = "ABCD"[i % 4]
    return f"<b>Correct:</b> {letter}<br><br>Because option {letter} is right for bench card {i}."


def build(n: int = 50_000, rebuild: bool = False) -> str:
    """Return the path to a cached collection with `n` MCAT bench cards, building it
    once if needed. Reruns reuse the cached file so they open instantly."""
    os.makedirs(CACHE_DIR, exist_ok=True)
    path = os.path.join(CACHE_DIR, f"bench_{n}.anki2")
    marker = path + ".done"
    if os.path.exists(path) and os.path.exists(marker) and not rebuild:
        return path
    for p in (path, marker):
        if os.path.exists(p):
            os.remove(p)

    import random

    rnd = random.Random(SEED)
    ids = kc_ids()
    col = Collection(path)
    try:
        did = col.decks.id(DECK_NAME)
        _enable_concept_scheduler(col, did)
        notetype = col.models.by_name("Basic")
        assert notetype is not None, "Basic notetype missing"
        for i in range(n):
            kc = ids[i % len(ids)]
            note = col.new_note(notetype)
            note["Front"] = _front(i, kc)
            note["Back"] = _back(i)
            note.tags = _tags(kc, rnd.randint(1, 5))
            col.add_note(note, did)
    finally:
        col.close()
    open(marker, "w").close()
    return path


def _enable_concept_scheduler(col: Collection, did: int) -> None:
    """Best-effort: flip the deck's concept-scheduler flag if the dict exposes it.
    The dashboard graph builds regardless, so failing here is non-fatal."""
    try:
        deck = col.decks.get(did)
        if deck is not None:
            deck["conceptSchedulerEnabled"] = True
            col.decks.save(deck)
    except Exception:
        pass


def open_collection(path: str) -> Collection:
    return Collection(path)


def status_for(col: Collection, deck_name: str = DECK_NAME):
    """Call the real backend dashboard RPC for the bench deck."""
    did = col.decks.id(deck_name)
    return col._backend.get_concept_scheduler_status(did)


def _main() -> int:
    ap = argparse.ArgumentParser(description="Build/verify the MCAT bench fixture.")
    ap.add_argument("--smoke", action="store_true", help="build a tiny deck and verify")
    ap.add_argument("--rebuild", action="store_true", help="force a rebuild")
    ap.add_argument("-n", type=int, default=50_000, help="number of cards")
    args = ap.parse_args()

    n = 200 if args.smoke else args.n
    path = build(n, rebuild=args.rebuild or args.smoke)
    col = open_collection(path)
    try:
        cards = col.card_count()
        status = status_for(col)
        nodes = len(status.graph.nodes)
        edges = len(status.graph.edges)
        print(f"OK: cards={cards} graph_nodes={nodes} graph_edges={edges} path={path}")
        assert cards >= n, f"expected >= {n} cards, got {cards}"
        assert nodes > 0, "expected a non-empty concept graph"
    finally:
        col.close()
    return 0


if __name__ == "__main__":
    raise SystemExit(_main())
