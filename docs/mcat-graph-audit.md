# MCAT Graph Audit

## Edge Convention

Canonical graph edges are authored as:

```text
Prerequisite KC -> Target KC
```

In Rust, `KnowledgeGraph::add_prerequisite(target, prerequisite)` stores each prerequisite on the target node. The demo graph keeps source data as `(prerequisite, target)` pairs and converts them into the internal representation.

This means:

```text
Bio::DNA -> Bio::Genetics
```

is stored by calling:

```text
add_prerequisite(Bio::Genetics, Bio::DNA)
```

## Current Demo Graph

The current demo graph contains 10 KCs and 9 prerequisite edges.

Current KCs:

- `Bio::DNA`
- `Bio::Genetics`
- `Bio::Eukaryotic_Cell`
- `Biochem::Amino_Acids`
- `Biochem::Peptides_and_Proteins`
- `Biochem::Protein_Structure_and_Function`
- `Biochem::Enzymes`
- `Biochem::Bioenergetics`
- `Biochem::Glycolysis`
- `Biochem::Citric_Acid_Cycle`

Current edges:

- `Bio::DNA -> Bio::Genetics`
- `Biochem::Amino_Acids -> Biochem::Peptides_and_Proteins`
- `Biochem::Peptides_and_Proteins -> Biochem::Protein_Structure_and_Function`
- `Biochem::Protein_Structure_and_Function -> Biochem::Enzymes`
- `Biochem::Enzymes -> Biochem::Bioenergetics`
- `Biochem::Bioenergetics -> Biochem::Glycolysis`
- `Biochem::Bioenergetics -> Biochem::Citric_Acid_Cycle`
- `Biochem::Glycolysis -> Biochem::Citric_Acid_Cycle`
- `Bio::Eukaryotic_Cell -> Biochem::Bioenergetics`

## Guardrails Added

The backend now has a regression test that verifies every demo card's direct `Prereq::...` tags exactly match the canonical graph edges for that card's target KC.

This prevents Add Cards or demo content from silently reversing edge direction.

## Add Cards Rule

Add Cards no longer asks users to choose prerequisite tags from a checklist.

User-created cards should normally provide:

- `KC::...`
- `Difficulty::...`
- optional `MCAT::...` override
- optional IRT metadata

Prerequisite direction belongs in the canonical graph authoring process, not ad hoc card entry.

## Expansion Toward 200 Cards

Expand in audited batches, not all at once:

1. Finish Bio/Biochem molecular foundations.
2. Add general chemistry KCs and edges.
3. Add physics KCs and edges.
4. Add organic chemistry KCs and edges.
5. Add psych/soc KCs and edges.
6. Add CARS only if there is a meaningful KC structure beyond passage practice.

For each batch:

- Add KCs to the canonical graph.
- Add edges as `prerequisite -> target`.
- Add demo cards only after the graph edges are reviewed.
- Run the graph-edge regression test.
- Check the deck-options graph remains readable.

## Overview Card Flow

Overview cards are represented with:

```text
Overview::<KC>
KC::<KC>
```

Example:

```text
Overview::Biochem::Glycolysis
KC::Biochem::Glycolysis
```

Behavior:

- Overview cards sort before normal practice cards for the same KC when readiness scores tie.
- Answering an overview card does not update mastery or IRT evidence.
- Normal practice cards continue to update mastery and section evidence.

