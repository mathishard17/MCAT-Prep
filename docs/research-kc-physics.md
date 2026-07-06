# Research KC Map: MCAT Physics (Track A)

Machine-usable Knowledge Component (KC) map for the **Physics** discipline of the
MCAT Concept Scheduler. This is a research draft for Track A ("MCAT Knowledge
Research") in `added features/next-feature-expansion-plan.md`. It is docs-only and
does not modify any code or graph.

- **Area prefix:** `Physics::` (KC ids are the string after `KC::`, e.g. a card
  tags `KC::Physics::Kinematics`; the stored KC id is `Physics::Kinematics`).
- **Primary MCAT section:** `Chem_Phys` for every Physics KC. The current backend
  (`derived_mcat_sections_for_topics` in `qt/aqt/concept_tags.py`) auto-derives
  **only** `Chem_Phys` for `Physics::` topics. Any `Bio_Biochem` / `Psych_Soc`
  overlaps listed below are **passage-level** overlaps (a physics idea showing up
  inside a biology/psych passage), marked with `*`; they are research notes, not
  auto-tagged sections.
- **Content is synthetic/original.** No copyrighted prep material is reproduced.
  Uncertain / lower-confidence items are marked `(verify)`.

## Legend

- **Type:** `foundation` (entry prerequisite, calibration-friendly) ·
  `mechanism` (core model/relationship) · `application` (applied/bio context) ·
  `detail` (narrower, often higher-difficulty or lower-yield).
- **Difficulty ladder:** suggested `Difficulty::` span for a KC's cards, written
  `start → peak` on the 1–5 scale from the tag convention. Author easy
  calibration cards at `start` and reserve `peak` for harder reasoning items.
- **Sections:** `Chem_Phys` = primary; `X*` = passage-level cross-section overlap
  (not auto-derived by current code).

## KC Master Table (26 KCs)

| KC id | Parent cluster | Prerequisites (KC ids) | Overlapping sections | Difficulty ladder | Type |
|---|---|---|---|---|---|
| `Physics::Units_And_Measurement` | Foundations & Math | — (root) | `Chem_Phys` | 1 → 2 | foundation |
| `Physics::Kinematics` | Classical Mechanics | `Physics::Units_And_Measurement` | `Chem_Phys` | 1 → 3 | foundation |
| `Physics::Newtons_Laws` | Classical Mechanics | `Physics::Kinematics` | `Chem_Phys` | 1 → 4 | mechanism |
| `Physics::Force_Equilibrium` | Classical Mechanics | `Physics::Newtons_Laws` | `Chem_Phys` | 2 → 4 | mechanism |
| `Physics::Work_And_Energy` | Classical Mechanics | `Physics::Newtons_Laws` | `Chem_Phys` | 2 → 4 | mechanism |
| `Physics::Momentum_And_Impulse` | Classical Mechanics | `Physics::Newtons_Laws` | `Chem_Phys` | 2 → 4 | mechanism |
| `Physics::Rotational_Motion` | Classical Mechanics | `Physics::Force_Equilibrium`, `Physics::Work_And_Energy` | `Chem_Phys` | 3 → 5 | detail |
| `Physics::Periodic_Motion` | Oscillations, Waves & Sound | `Physics::Force_Equilibrium`, `Physics::Work_And_Energy` | `Chem_Phys` | 3 → 5 | mechanism |
| `Physics::Waves` | Oscillations, Waves & Sound | `Physics::Periodic_Motion` | `Chem_Phys` | 3 → 5 | mechanism |
| `Physics::Sound` | Oscillations, Waves & Sound | `Physics::Waves` | `Chem_Phys`, `Bio_Biochem*`, `Psych_Soc*` | 3 → 5 | application |
| `Physics::Fluid_Statics` | Fluids | `Physics::Force_Equilibrium` | `Chem_Phys`, `Bio_Biochem*` | 2 → 4 | mechanism |
| `Physics::Fluid_Dynamics` | Fluids | `Physics::Fluid_Statics`, `Physics::Work_And_Energy` | `Chem_Phys`, `Bio_Biochem*` | 3 → 5 | mechanism |
| `Physics::Thermodynamics` | Thermodynamics | `Physics::Work_And_Energy`, `GenChem::Thermochemistry` (verify) | `Chem_Phys`, `Bio_Biochem*` | 2 → 4 | mechanism |
| `Physics::Electrostatics` | Electricity & Magnetism | `Physics::Work_And_Energy` | `Chem_Phys` | 2 → 4 | mechanism |
| `Physics::Electrical_Circuits` | Electricity & Magnetism | `Physics::Electrostatics` | `Chem_Phys`, `Bio_Biochem*` | 2 → 4 | mechanism |
| `Physics::Circuit_Elements` | Electricity & Magnetism | `Physics::Electrical_Circuits` | `Chem_Phys`, `Bio_Biochem*` | 3 → 5 | detail |
| `Physics::Magnetism` | Electricity & Magnetism | `Physics::Electrostatics` | `Chem_Phys` | 3 → 5 | detail |
| `Physics::Electromagnetic_Radiation` | Light & Optics | `Physics::Waves`, `Physics::Magnetism` | `Chem_Phys` | 2 → 4 | mechanism |
| `Physics::Geometric_Optics` | Light & Optics | `Physics::Electromagnetic_Radiation` | `Chem_Phys`, `Bio_Biochem*` | 3 → 5 | application |
| `Physics::Physical_Optics` | Light & Optics | `Physics::Geometric_Optics`, `Physics::Waves` | `Chem_Phys` | 4 → 5 | detail |
| `Physics::Atomic_Structure` | Atomic & Nuclear | `Physics::Electromagnetic_Radiation`, `Physics::Electrostatics` | `Chem_Phys` | 3 → 5 | mechanism |
| `Physics::Nuclear_Physics` | Atomic & Nuclear | `Physics::Atomic_Structure` | `Chem_Phys`, `Bio_Biochem*` | 3 → 5 | detail |
| `Physics::Circulatory_Hemodynamics` | Biological Applications | `Physics::Fluid_Dynamics` | `Chem_Phys`, `Bio_Biochem*` | 3 → 5 | application |
| `Physics::Gas_Exchange_And_Respiration_Physics` | Biological Applications | `Physics::Fluid_Statics`, `Physics::Thermodynamics`, `GenChem::Gas_Phase` | `Chem_Phys`, `Bio_Biochem*` | 3 → 5 | application |
| `Physics::Optics_Of_The_Eye` | Biological Applications | `Physics::Geometric_Optics` | `Chem_Phys`, `Bio_Biochem*`, `Psych_Soc*` | 3 → 5 | application |
| `Physics::Bioelectricity` | Biological Applications | `Physics::Circuit_Elements`, `GenChem::Ions_in_Solutions` | `Chem_Phys`, `Bio_Biochem*` | 4 → 5 | application |

## Prerequisite Edges (`prerequisite -> target`)

Authored in the canonical direction from `added features/mcat-graph-audit.md`
(source pairs are `(prerequisite, target)`; the backend stores each prerequisite
on the target via `add_prerequisite(target, prerequisite)`). All edges are
acyclic (see topological check below). `GenChem::*` nodes are external roots
owned by the General Chemistry map.

```text
Physics::Units_And_Measurement -> Physics::Kinematics
Physics::Kinematics -> Physics::Newtons_Laws
Physics::Newtons_Laws -> Physics::Force_Equilibrium
Physics::Newtons_Laws -> Physics::Work_And_Energy
Physics::Newtons_Laws -> Physics::Momentum_And_Impulse
Physics::Force_Equilibrium -> Physics::Rotational_Motion
Physics::Work_And_Energy -> Physics::Rotational_Motion
Physics::Force_Equilibrium -> Physics::Periodic_Motion
Physics::Work_And_Energy -> Physics::Periodic_Motion
Physics::Force_Equilibrium -> Physics::Fluid_Statics
Physics::Fluid_Statics -> Physics::Fluid_Dynamics
Physics::Work_And_Energy -> Physics::Fluid_Dynamics
Physics::Work_And_Energy -> Physics::Thermodynamics
GenChem::Thermochemistry -> Physics::Thermodynamics        (cross-discipline, verify)
Physics::Periodic_Motion -> Physics::Waves
Physics::Waves -> Physics::Sound
Physics::Work_And_Energy -> Physics::Electrostatics
Physics::Electrostatics -> Physics::Electrical_Circuits
Physics::Electrical_Circuits -> Physics::Circuit_Elements
Physics::Electrostatics -> Physics::Magnetism
Physics::Waves -> Physics::Electromagnetic_Radiation
Physics::Magnetism -> Physics::Electromagnetic_Radiation
Physics::Electromagnetic_Radiation -> Physics::Geometric_Optics
Physics::Geometric_Optics -> Physics::Physical_Optics
Physics::Waves -> Physics::Physical_Optics
Physics::Electromagnetic_Radiation -> Physics::Atomic_Structure
Physics::Electrostatics -> Physics::Atomic_Structure
Physics::Atomic_Structure -> Physics::Nuclear_Physics
Physics::Fluid_Dynamics -> Physics::Circulatory_Hemodynamics
Physics::Fluid_Statics -> Physics::Gas_Exchange_And_Respiration_Physics
Physics::Thermodynamics -> Physics::Gas_Exchange_And_Respiration_Physics
GenChem::Gas_Phase -> Physics::Gas_Exchange_And_Respiration_Physics   (cross-discipline)
Physics::Geometric_Optics -> Physics::Optics_Of_The_Eye
Physics::Circuit_Elements -> Physics::Bioelectricity
GenChem::Ions_in_Solutions -> Physics::Bioelectricity                 (cross-discipline)
```

### Acyclicity check (topological order)

The following order lists every KC after all of its prerequisites, so the graph
is a DAG (matches the cycle guard in `KnowledgeGraph::cycle`):

1. `Physics::Units_And_Measurement`
2. `Physics::Kinematics`
3. `Physics::Newtons_Laws`
4. `Physics::Force_Equilibrium`
5. `Physics::Work_And_Energy`
6. `Physics::Momentum_And_Impulse`
7. `Physics::Rotational_Motion`
8. `Physics::Periodic_Motion`
9. `Physics::Fluid_Statics`
10. `Physics::Fluid_Dynamics`
11. `Physics::Thermodynamics`
12. `Physics::Waves`
13. `Physics::Sound`
14. `Physics::Electrostatics`
15. `Physics::Electrical_Circuits`
16. `Physics::Circuit_Elements`
17. `Physics::Magnetism`
18. `Physics::Electromagnetic_Radiation`
19. `Physics::Geometric_Optics`
20. `Physics::Physical_Optics`
21. `Physics::Atomic_Structure`
22. `Physics::Nuclear_Physics`
23. `Physics::Circulatory_Hemodynamics`
24. `Physics::Gas_Exchange_And_Respiration_Physics`
25. `Physics::Optics_Of_The_Eye`
26. `Physics::Bioelectricity`

(External `GenChem::*` roots precede any Physics KC that references them.)

## Per-KC Scope (sub-skills)

Concise scope notes to guide card/problem generation (Track B). Kept synthetic
and high-level.

- **`Physics::Units_And_Measurement`** — SI units, unit conversion, dimensional
  analysis, scientific notation, significant figures, scalar vs vector, vector
  addition/components. Root prerequisite for the whole discipline.
- **`Physics::Kinematics`** — displacement/velocity/acceleration, 1D constant-
  acceleration equations, free fall, 2D projectile motion, relative velocity,
  motion graphs.
- **`Physics::Newtons_Laws`** — 1st/2nd/3rd laws, free-body diagrams, weight vs
  mass, normal/tension/friction (static & kinetic), inclined planes, centripetal
  force for uniform circular motion.
- **`Physics::Force_Equilibrium`** — translational and rotational equilibrium,
  torque, center of mass/gravity, levers and simple machines, mechanical
  advantage (bones/joints as levers).
- **`Physics::Work_And_Energy`** — work, work–energy theorem, kinetic & potential
  (gravitational/elastic) energy, conservation of mechanical energy, power,
  efficiency, conservative vs non-conservative forces.
- **`Physics::Momentum_And_Impulse`** — momentum, impulse–momentum theorem,
  conservation of momentum, elastic vs inelastic collisions, center-of-mass
  motion. (Draft `mcat.md` had no explicit momentum KC — added here.)
- **`Physics::Rotational_Motion`** — angular kinematics, moment of inertia,
  angular momentum & its conservation, rotational kinetic energy. Lower MCAT
  yield beyond torque/equilibrium `(verify)`; keep card count small.
- **`Physics::Periodic_Motion`** — simple harmonic motion, Hooke's law/springs,
  pendulums, amplitude/period/frequency, energy in SHM, damping/resonance.
  Bridges mechanics to waves.
- **`Physics::Waves`** — transverse/longitudinal waves, wavelength/frequency/
  speed, superposition & interference, standing waves, nodes/antinodes,
  resonance.
- **`Physics::Sound`** — sound as a longitudinal wave, speed in media, intensity
  and the decibel scale, Doppler effect, beats, harmonics in pipes/strings,
  ultrasound and hearing (bio/psych passage overlap).
- **`Physics::Fluid_Statics`** — density & specific gravity, pressure, Pascal's
  principle, hydrostatic pressure with depth, Archimedes' principle/buoyancy,
  surface tension, manometers/barometers.
- **`Physics::Fluid_Dynamics`** — continuity equation, Bernoulli's equation,
  laminar vs turbulent flow, viscosity, Poiseuille flow, flow resistance.
- **`Physics::Thermodynamics`** — temperature scales, heat vs work, specific heat
  & calorimetry, phase changes/latent heat, thermal expansion, laws of
  thermodynamics, PV work, entropy. Overlaps `GenChem::Thermochemistry`,
  `GenChem::Gas_Phase`.
- **`Physics::Electrostatics`** — charge, Coulomb's law, electric field, electric
  potential energy & potential (voltage), equipotential lines, dipoles,
  conductors vs insulators.
- **`Physics::Electrical_Circuits`** — current, Ohm's law, resistivity, resistors
  in series/parallel, Kirchhoff's laws, EMF & internal resistance, electrical
  power/energy.
- **`Physics::Circuit_Elements`** — capacitors & capacitance, dielectrics, energy
  stored, RC time constant, ammeters/voltmeters. Bio overlap: cell membrane as a
  capacitor.
- **`Physics::Magnetism`** — magnetic fields, force on moving charges and
  current-carrying wires, right-hand rules, basic electromagnetic induction
  (Faraday/Lenz, qualitative). `(verify)` MCAT depth on induction.
- **`Physics::Electromagnetic_Radiation`** — EM spectrum, speed of light,
  wave–particle duality, photon energy `E = hf`, links to spectroscopy. Overlaps
  `GenChem::Molecular_Structure` and `Orgo::Molecular_Structure_and_Absorption_Spectra`.
- **`Physics::Geometric_Optics`** — reflection, refraction/Snell's law, total
  internal reflection, mirrors and thin lenses, lens/mirror equation,
  magnification, ray diagrams, dispersion.
- **`Physics::Physical_Optics`** — interference (double-slit, thin films),
  diffraction, polarization, wave nature of light. Higher difficulty, moderate
  yield.
- **`Physics::Atomic_Structure`** — Bohr model, quantized energy levels,
  absorption/emission spectra, photoelectric effect, quantum numbers
  (qualitative). Heavy overlap with `GenChem` electronic structure — coordinate
  ownership with the GenChem map `(verify)`.
- **`Physics::Nuclear_Physics`** — nuclear notation, radioactive decay
  (alpha/beta/gamma), half-life, mass–energy equivalence `E = mc^2`, binding
  energy, fission/fusion. Bio/medical overlap: imaging (PET), radiotherapy,
  radiometric dating.
- **`Physics::Circulatory_Hemodynamics`** — blood pressure, flow rate &
  continuity in vessels, resistance/Poiseuille in circulation, pressure drops,
  compliance. Applied bridge to `Bio::Circulatory_System`.
- **`Physics::Gas_Exchange_And_Respiration_Physics`** — partial pressures &
  Dalton's law, gas diffusion, surface tension/surfactant, Laplace's law in
  alveoli, pressure–volume work of breathing. Applied bridge to
  `Bio::Respiratory_System`.
- **`Physics::Optics_Of_The_Eye`** — the eye as a lens system, accommodation,
  refractive errors (myopia/hyperopia), corrective lenses & diopters, resolution.
  Bridges to vision/sensation (`Psych_Soc*`).
- **`Physics::Bioelectricity`** — membrane potential as an RC circuit, resting/
  action potentials (physics view), capacitance & conduction, cable properties.
  Applied bridge to `Bio::Nervous_System`; needs `GenChem::Ions_in_Solutions`
  (electrochemical gradients / Nernst).

## Cross-Discipline Prerequisites & Overlaps

### Cross-discipline prerequisites referenced (Physics depends on other areas)

These are the only outward prerequisite edges into non-Physics KCs. All point
into General Chemistry, which is upstream of Physics here, so they add no cycles.

| Target Physics KC | Cross-discipline prerequisite | Rationale | Confidence |
|---|---|---|---|
| `Physics::Thermodynamics` | `GenChem::Thermochemistry` | Heat, enthalpy, calorimetry grounding | `(verify)` — could be an overlap rather than a hard prereq |
| `Physics::Gas_Exchange_And_Respiration_Physics` | `GenChem::Gas_Phase` | Ideal gas, partial pressures | high |
| `Physics::Bioelectricity` | `GenChem::Ions_in_Solutions` | Ionic gradients / Nernst potential | high |

### Downstream cross-discipline consumers (Physics is a prerequisite for Bio)

Already present in `mcat.md` / `mcat-graph-audit.md`; preserved by this map:

- `Physics::Fluids -> Bio::Circulatory_System` — under this refined map the edge
  should target the split node `Physics::Fluid_Dynamics` (with
  `Physics::Fluid_Statics` upstream). `Physics::Circulatory_Hemodynamics` is the
  dedicated applied bridge.
- `Physics::Electrical_Circuits -> Bio::Nervous_System` — id preserved unchanged;
  `Physics::Bioelectricity` deepens this bridge.

### Passage-level section overlaps (not auto-tagged)

`derived_mcat_sections_for_topics` maps `Physics::` topics to `Chem_Phys` only.
The overlaps below (`*` in the table) describe where physics content commonly
appears inside other sections' passages and are for content-planning only:

- **`Bio_Biochem*`**: `Fluid_Statics`, `Fluid_Dynamics`, `Circulatory_Hemodynamics`,
  `Gas_Exchange_And_Respiration_Physics`, `Bioelectricity`, `Circuit_Elements`,
  `Electrical_Circuits`, `Sound`, `Thermodynamics`, `Nuclear_Physics`,
  `Optics_Of_The_Eye`.
- **`Psych_Soc*`**: `Optics_Of_The_Eye`, `Sound` (sensation/perception of vision
  and hearing).
- **Chem/Phys internal overlaps with other disciplines**: `Thermodynamics` ↔
  `GenChem::Thermochemistry`/`GenChem::Gas_Phase`; `Atomic_Structure` ↔ GenChem
  electronic structure; `Electromagnetic_Radiation` ↔ GenChem/Orgo spectroscopy.

## Difficulty Ladder Rationale

- **Foundations (`1 → 2/3`)**: `Units_And_Measurement`, `Kinematics` — sampled
  early for calibration with some randomness, per `mcat.md` Layer 1 guidance.
- **Core mechanisms (`2 → 4`)**: most `mechanism` KCs; author 2 easy, 2 medium,
  1 hard per the demo card allocation pattern in `mcat.md`.
- **Applied & detail (`3 → 5`)**: multi-step reasoning, `Data`/`Application`
  reasoning items, and biological-application KCs peak at `Difficulty::5`.
- `Difficulty::N` maps to IRT difficulty `b` in `concept.rs`
  (`difficulty_to_irt_b`: 1→−2.0 … 5→+2.0), so the ladder also seeds item
  difficulty for the section IRT model.

## Alignment With Existing Draft Ids (`mcat.md`)

This map refines the flat Physics list in `mcat.md`. Preserve the two ids that
already have cross-discipline edges (`Physics::Fluids`, `Physics::Electrical_Circuits`)
during reconciliation; the rest are consolidations/splits.

| This map (KC id) | Existing draft id(s) in `mcat.md` | Relationship |
|---|---|---|
| `Physics::Units_And_Measurement` | — | new (added foundation root) |
| `Physics::Kinematics` | `Physics::Translational_Motion` | rename |
| `Physics::Newtons_Laws` | `Physics::Force` | split (dynamics) |
| `Physics::Force_Equilibrium` | `Physics::Equilibrium`, `Physics::Force` | consolidate (statics/torque) |
| `Physics::Work_And_Energy` | `Physics::Work`, `Physics::Energy` | merge |
| `Physics::Momentum_And_Impulse` | — | new (draft lacked momentum) |
| `Physics::Rotational_Motion` | — | new (low-yield detail) |
| `Physics::Periodic_Motion` | `Physics::Periodic_Motion` | keep |
| `Physics::Waves` | — | new (explicit wave node) |
| `Physics::Sound` | `Physics::Sound` | keep |
| `Physics::Fluid_Statics` | `Physics::Fluids`, `Physics::Matter` | split |
| `Physics::Fluid_Dynamics` | `Physics::Fluids` | split (**preserve** `Physics::Fluids` edge → maps here) |
| `Physics::Thermodynamics` | `Physics::Thermodynamics`, `Physics::Matter` | keep/absorb |
| `Physics::Electrostatics` | `Physics::Electrostatics` | keep |
| `Physics::Electrical_Circuits` | `Physics::Electrical_Circuits` | keep (**preserve** — Bio edge) |
| `Physics::Circuit_Elements` | `Physics::Circuit_Elements` | keep |
| `Physics::Magnetism` | `Physics::Magnetism` | keep |
| `Physics::Electromagnetic_Radiation` | `Physics::Electromagnetic_Radiation` | keep |
| `Physics::Geometric_Optics` | `Physics::Optics`, `Physics::Light` | split |
| `Physics::Physical_Optics` | `Physics::Optics`, `Physics::Light` | split |
| `Physics::Atomic_Structure` | `Physics::Atoms`, `Physics::Electronic_Structure`, `Physics::Atomic_and_Chemical_Behavior` | merge |
| `Physics::Nuclear_Physics` | `Physics::Nuclear_Decay` | rename |
| `Physics::Circulatory_Hemodynamics` | — | new (bio application) |
| `Physics::Gas_Exchange_And_Respiration_Physics` | — | new (bio application) |
| `Physics::Optics_Of_The_Eye` | — | new (bio application) |
| `Physics::Bioelectricity` | — | new (bio application) |

## Research Notes

- **Method / sources.** Structure follows the AAMC "What's on the MCAT" Chem/Phys
  Foundational Concepts 4 and 5 (physical principles of living systems) and
  standard introductory-physics topic ordering, expressed as original synthetic
  KCs. No copyrighted item text is reproduced. Confirm the taxonomy against a
  current AAMC content outline before freezing `(verify)`.
- **KC count:** 26 Physics KCs (target was ~20–30).
- **Section tagging accuracy.** The backend derives only `Chem_Phys` for
  `Physics::` topics. The `Bio_Biochem*` / `Psych_Soc*` labels are passage-level
  content-planning overlaps, not tags the scheduler will apply automatically. If
  the team wants physics bio-application cards to feed `Bio_Biochem` IRT
  evidence, that requires either dual `KC::` tags on those cards or a change to
  `derived_mcat_sections_for_topics` (out of scope for this docs-only task).
- **Acyclicity.** All 26 KCs plus the 3 external `GenChem::*` roots form a DAG;
  see the topological order above. Every prerequisite precedes its target.
- **Draft gaps found.** The `mcat.md` Physics list had **no momentum KC** and no
  units/measurement foundation; both are added. `Physics::Matter`,
  `Physics::Electronic_Structure`, and `Physics::Atomic_and_Chemical_Behavior`
  are absorbed into fluids/thermo and atomic structure to reduce overlap with the
  GenChem map.
- **Id preservation.** `Physics::Electrical_Circuits` is kept verbatim because
  `Bio::Nervous_System` already depends on it. `Physics::Fluids` is split into
  `Physics::Fluid_Statics` / `Physics::Fluid_Dynamics`; the existing
  `Bio::Circulatory_System` prerequisite should be re-pointed to
  `Physics::Fluid_Dynamics` during reconciliation.
- **Uncertain items (`verify`)**: MCAT yield/depth of `Physics::Rotational_Motion`
  and electromagnetic induction in `Physics::Magnetism`; whether
  `GenChem::Thermochemistry` should be a hard prerequisite vs. an overlap for
  `Physics::Thermodynamics`; and ownership of atomic-structure content shared with
  the GenChem map.
- **Handoffs.** Track B (generation harness) can use `KC id`, `Difficulty ladder`,
  and `Type` to seed `Difficulty::`/`Reasoning::` tags; Track C (lessons) can use
  Per-KC Scope for lesson stubs; engine integration should ingest the edges list
  after graph-edge regression review.
