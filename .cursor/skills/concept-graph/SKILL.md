---
name: concept-graph
description: >-
  Render knowledge/concept graphs (prerequisite DAGs) as readable layered
  "linear path" layouts with vertical+horizontal scroll and zoom, instead of
  force-directed hairballs. Use when building or editing the MCAT concept graph,
  the reviewer Progress sidebar graph, the deck-options lattice, or any
  prerequisite-DAG / knowledge-graph visualization in this repo.
---

# Concept graph layout (layered "linear paths")

Prerequisite graphs in this repo are DAGs (the KC map is globally acyclic). Draw
them as **layered lanes**, not force-directed blobs: a student should read
"builds on -> this -> unlocks" left to right, and be able to scroll/zoom around
a large map rather than untangle a hairball.

## Principles

1. **Layered, not physics.** Column = topological longest-path depth. Roots
   (no prerequisites) are column 0; each node sits one column right of its
   deepest prerequisite. Edges then flow left -> right ("linear paths").
2. **Lanes by discipline.** Stack disciplines as horizontal bands
   (`Bio, Biochem, GenChem, Orgo, Physics, PsychSoc, CARS`), so each lane is a
   clean chain. Color by MCAT super-section (Bio/Biochem blue, Chem/Phys red,
   Psych/Soc green).
3. **Scroll + zoom, not pan-transform.** Put nodes in a canvas sized in pixels
   (`units * step * zoom`) inside an `overflow: auto` container so real
   scrollbars appear. Zoom = recompute pixel positions (keep dot size + font
   constant so nodes stay clickable/legible); anchor ctrl/cmd+wheel zoom at the
   cursor and offer `-` / `Fit` / `+` buttons.
4. **Default to focus, opt into the galaxy.** During review show only the
   current node + its direct prerequisites and dependents (a handful of
   circles). A "full map" toggle reveals the whole layered graph to explore.
5. **Highlight the frontier.** Mark startable nodes (outer-fringe: prerequisites
   met, not yet mastered) with a green ring + pointer; fade locked nodes; ring
   the current node. The graph itself is the topic picker — click a startable
   node to choose it.
6. **Size by importance.** Dot radius grows with how many concepts list the node
   as a prerequisite (a "lynchpin" count), so foundational nodes read as larger.

## Layout algorithm

```
layeredUnits(ids, edges) -> { pos: {id: {col, row}}, cols, rows }
  1. depth via longest-path over a topological order (Kahn):
     depth[v] = max(depth[u] + 1) over prerequisites u -> v; roots = 0
  2. group ids into discipline lanes (fixed lane order)
  3. within each lane, sort by (depth, id); stack nodes sharing a column into
     successive rows; lane height = tallest column; leave a gap row between lanes
  4. col = depth, row = laneTop + stackIndex
```

Focused view is the same idea at 3 columns: prerequisites (col 0) ->
current KC (col 1) -> unlocks (col 2).

## Rendering conventions

- Convert units to pixels every paint: `x = PAD + col*COLW*zoom`,
  `y = PAD + row*ROWH*zoom`. Size the canvas to fit + a right pad for labels.
- Edges: one SVG sized to the canvas; `<line>` between prerequisite and target
  dot centers; `vector-effect: non-scaling-stroke`.
- Node = an absolutely-positioned point; the dot is centered on `(x, y)` and the
  label floats to its right (so vertical spacing isn't eaten by labels). Hide
  labels when zoomed out (`zoom < ~0.85`).
- Auto-scroll the focus node to center when the panel opens.

## Where this lives in this repo

- **Reviewer sidebar (source of truth):** the injected JS inside
  `revHtml()` in `qt/aqt/reviewer.py` (`_renderConceptGraphSidebar`,
  `layeredUnits`, `neighbourhoodUnits`, `paint`/`setZoom`/`fitZoom`); styles in
  `ts/reviewer/reviewer.scss` (`.concept-sidebar-graph.scroll`,
  `.concept-graph-canvas`, `.concept-sidebar-node.available`).
- **Deck options:** the lattice + per-area graphs in
  `ts/routes/deck-options/ConceptSchedulerOptions.svelte`.
- **Payload contract** (from `get_concept_scheduler_status`):
  `nodes[] = {id, mastery, fringe, answered, positive, negative, memory}`,
  `edges[] = {prerequisiteId, targetId}`, `focusKc`. `fringe` is
  `inner` (mastered) | `outer` (startable frontier) | `locked`.
- Clicking a startable node calls `pycmd("conceptStart:<KC>")`, which persists
  the choice via the `SetConceptSelectedTopic` backend RPC.

## Pitfalls

- Don't reintroduce a force simulation for DAGs — it produces the hairball this
  layout replaces.
- Inline-style the SVG width/height/position in `paint()`; a generic
  `svg { width: 100% }` rule will otherwise clamp it to the viewport, not the
  scrollable canvas.
- Keep the layout deterministic (stable sort by id) so the map doesn't jump
  between cards.
- Reviewer graph JS lives in a Python string and is only validated at runtime in
  the webview — re-open the Progress panel after a full app restart to verify.
