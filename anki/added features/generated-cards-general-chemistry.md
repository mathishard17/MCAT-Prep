# MCAT General Chemistry Flashcards

Synthetic, original multiple-choice cards for the General Chemistry (`GenChem::`)
Knowledge Components of the MCAT Concept Scheduler. One card per General Chemistry
KC (26 KCs), in the importer block format used by `added features/mcat_demo_cards.md`.
KC ids, prerequisites, sections, and difficulty bands follow the frozen
`added features/kc-map-unified.md`. All content is original/synthetic; no
copyrighted prep material is reproduced.

## Tags

- Section tag: `MCAT::Chem_Phys`
- KC tag: `KC::GenChem::<Component>`
- Prerequisite tag: `Prereq::GenChem::<Component>` (or cross-discipline `Prereq::Physics::<Component>`)
- Difficulty tag: `Difficulty::<1-5>`

---

## GenChem::Atomic_Structure

### MCAT-GCH-ATOS-001

- **KC:** `GenChem::Atomic_Structure`
- **Prereqs:** none
- **Difficulty:** 1
- **Question:** An atom has 17 protons, 18 neutrons, and 17 electrons. What is its mass number?
- **A:** 17
- **B:** 18
- **C:** 35
- **D:** 52
- **Correct:** C
- **Explanation:** Mass number is the sum of protons and neutrons, 17 + 18 = 35; electrons contribute negligibly to the mass number.
- **Tags:** `KC::GenChem::Atomic_Structure` `MCAT::Chem_Phys` `Difficulty::1`

## GenChem::Electron_Configuration

### MCAT-GCH-ELEC-001

- **KC:** `GenChem::Electron_Configuration`
- **Prereqs:** `Prereq::GenChem::Atomic_Structure` `Prereq::Physics::Atomic_Structure`
- **Difficulty:** 3
- **Question:** A neutral atom has the ground-state electron configuration 1s2 2s2 2p6 3s2 3p4. Which element is it?
- **A:** Silicon
- **B:** Oxygen
- **C:** Chlorine
- **D:** Sulfur
- **Correct:** D
- **Explanation:** The electrons sum to 16, so the atomic number is 16, which corresponds to sulfur.
- **Tags:** `KC::GenChem::Electron_Configuration` `Prereq::GenChem::Atomic_Structure` `Prereq::Physics::Atomic_Structure` `MCAT::Chem_Phys` `Difficulty::3`

## GenChem::Atomic_Spectra_and_Quantum

### MCAT-GCH-ATOSQ-001

- **KC:** `GenChem::Atomic_Spectra_and_Quantum`
- **Prereqs:** `Prereq::GenChem::Electron_Configuration` `Prereq::Physics::Electromagnetic_Radiation`
- **Difficulty:** 4
- **Question:** When an excited electron falls between energy levels and emits a photon, a transition with a larger energy gap produces a photon with:
- **A:** a longer wavelength
- **B:** a lower frequency
- **C:** a shorter wavelength
- **D:** the same wavelength regardless of the gap
- **Correct:** C
- **Explanation:** Photon energy equals hc divided by wavelength, so a larger energy gap means a higher frequency and a shorter wavelength.
- **Tags:** `KC::GenChem::Atomic_Spectra_and_Quantum` `Prereq::GenChem::Electron_Configuration` `Prereq::Physics::Electromagnetic_Radiation` `MCAT::Chem_Phys` `Difficulty::4`

## GenChem::Periodic_Trends

### MCAT-GCH-PERT-001

- **KC:** `GenChem::Periodic_Trends`
- **Prereqs:** `Prereq::GenChem::Electron_Configuration`
- **Difficulty:** 2
- **Question:** Atomic radius generally decreases from left to right across a period. What best explains this trend?
- **A:** Electrons are added to shells farther from the nucleus
- **B:** Increasing effective nuclear charge pulls the same-shell electrons inward
- **C:** The number of neutrons decreases across a period
- **D:** Electron-electron repulsion grows faster than nuclear charge
- **Correct:** B
- **Explanation:** Across a period electrons fill the same principal shell while nuclear charge rises, so the higher effective nuclear charge contracts the atom.
- **Tags:** `KC::GenChem::Periodic_Trends` `Prereq::GenChem::Electron_Configuration` `MCAT::Chem_Phys` `Difficulty::2`

## GenChem::Chemical_Bonding

### MCAT-GCH-CHEB-001

- **KC:** `GenChem::Chemical_Bonding`
- **Prereqs:** `Prereq::GenChem::Periodic_Trends`
- **Difficulty:** 1
- **Question:** Sodium and chlorine differ greatly in electronegativity. The bond they form is best classified as:
- **A:** Nonpolar covalent
- **B:** Polar covalent
- **C:** Metallic
- **D:** Ionic
- **Correct:** D
- **Explanation:** A large electronegativity difference drives electron transfer, producing oppositely charged ions held together by electrostatic attraction, which is an ionic bond.
- **Tags:** `KC::GenChem::Chemical_Bonding` `Prereq::GenChem::Periodic_Trends` `MCAT::Chem_Phys` `Difficulty::1`

## GenChem::Molecular_Geometry

### MCAT-GCH-MOLG-001

- **KC:** `GenChem::Molecular_Geometry`
- **Prereqs:** `Prereq::GenChem::Chemical_Bonding`
- **Difficulty:** 3
- **Question:** A central carbon forms four single bonds and has no lone pairs. What is its hybridization and approximate bond angle?
- **A:** sp3 with about 109.5-degree bond angles
- **B:** sp with 180-degree bond angles
- **C:** sp2 with 120-degree bond angles
- **D:** sp3d with 90-degree bond angles
- **Correct:** A
- **Explanation:** Four bonding electron domains adopt a tetrahedral arrangement, corresponding to sp3 hybridization and bond angles near 109.5 degrees.
- **Tags:** `KC::GenChem::Molecular_Geometry` `Prereq::GenChem::Chemical_Bonding` `MCAT::Chem_Phys` `Difficulty::3`

## GenChem::Stoichiometry

### MCAT-GCH-STO-001

- **KC:** `GenChem::Stoichiometry`
- **Prereqs:** `Prereq::GenChem::Chemical_Bonding`
- **Difficulty:** 2
- **Question:** For the reaction 2 H2 + O2 -> 2 H2O, how many moles of water form from 4 mol of H2 with oxygen in excess?
- **A:** 2 mol
- **B:** 4 mol
- **C:** 6 mol
- **D:** 8 mol
- **Correct:** B
- **Explanation:** The mole ratio of H2 to H2O is 2 to 2, or 1 to 1, so 4 mol of H2 yields 4 mol of water when oxygen is in excess.
- **Tags:** `KC::GenChem::Stoichiometry` `Prereq::GenChem::Chemical_Bonding` `MCAT::Chem_Phys` `Difficulty::2`

## GenChem::Reaction_Types

### MCAT-GCH-REAT-001

- **KC:** `GenChem::Reaction_Types`
- **Prereqs:** `Prereq::GenChem::Stoichiometry`
- **Difficulty:** 3
- **Question:** Which of the following is best classified as a decomposition reaction?
- **A:** 2 H2O -> 2 H2 + O2
- **B:** N2 + 3 H2 -> 2 NH3
- **C:** Zn + 2 HCl -> ZnCl2 + H2
- **D:** HCl + NaOH -> NaCl + H2O
- **Correct:** A
- **Explanation:** A decomposition reaction breaks a single compound into simpler substances, as when water splits into hydrogen and oxygen.
- **Tags:** `KC::GenChem::Reaction_Types` `Prereq::GenChem::Stoichiometry` `MCAT::Chem_Phys` `Difficulty::3`

## GenChem::Redox_Reactions

### MCAT-GCH-REDR-001

- **KC:** `GenChem::Redox_Reactions`
- **Prereqs:** `Prereq::GenChem::Reaction_Types`
- **Difficulty:** 3
- **Question:** In the reaction Zn + Cu2+ -> Zn2+ + Cu, which species acts as the oxidizing agent?
- **A:** Zn
- **B:** Zn2+
- **C:** Cu
- **D:** Cu2+
- **Correct:** D
- **Explanation:** Cu2+ gains electrons and is reduced, so it is the oxidizing agent; zinc metal is oxidized and serves as the reducing agent.
- **Tags:** `KC::GenChem::Redox_Reactions` `Prereq::GenChem::Reaction_Types` `MCAT::Chem_Phys` `Difficulty::3`

## GenChem::Gas_Phase

### MCAT-GCH-GASP-001

- **KC:** `GenChem::Gas_Phase`
- **Prereqs:** `Prereq::GenChem::Stoichiometry` `Prereq::Physics::Thermodynamics`
- **Difficulty:** 3
- **Question:** A fixed amount of ideal gas is heated from 200 K to 400 K at constant pressure. What happens to its volume?
- **A:** It is halved
- **B:** It is unchanged
- **C:** It doubles
- **D:** It quadruples
- **Correct:** C
- **Explanation:** At constant pressure the volume of an ideal gas is proportional to absolute temperature, so doubling the temperature from 200 K to 400 K doubles the volume.
- **Tags:** `KC::GenChem::Gas_Phase` `Prereq::GenChem::Stoichiometry` `Prereq::Physics::Thermodynamics` `MCAT::Chem_Phys` `Difficulty::3`

## GenChem::Intermolecular_Forces

### MCAT-GCH-INTF-001

- **KC:** `GenChem::Intermolecular_Forces`
- **Prereqs:** `Prereq::GenChem::Molecular_Geometry`
- **Difficulty:** 2
- **Question:** Which intermolecular force is chiefly responsible for the unusually high boiling point of water?
- **A:** Hydrogen bonding
- **B:** London dispersion forces
- **C:** Ion-dipole forces
- **D:** Metallic bonding
- **Correct:** A
- **Explanation:** Water molecules form extensive hydrogen bonds between O-H hydrogens and oxygen lone pairs, which raises the boiling point well above that of similarly sized molecules.
- **Tags:** `KC::GenChem::Intermolecular_Forces` `Prereq::GenChem::Molecular_Geometry` `MCAT::Chem_Phys` `Difficulty::2`

## GenChem::Phases_and_Phase_Changes

### MCAT-GCH-PHAPC-001

- **KC:** `GenChem::Phases_and_Phase_Changes`
- **Prereqs:** `Prereq::GenChem::Gas_Phase` `Prereq::GenChem::Intermolecular_Forces`
- **Difficulty:** 4
- **Question:** A liquid boils when its vapor pressure equals the surrounding pressure. At high altitude, where atmospheric pressure is lower, the boiling point of water is:
- **A:** higher than at sea level
- **B:** unchanged from sea level
- **C:** lower than at sea level
- **D:** exactly 100 degrees C at any altitude
- **Correct:** C
- **Explanation:** With lower external pressure, a lower vapor pressure (reached at a lower temperature) is enough for boiling, so water boils below 100 degrees C at altitude.
- **Tags:** `KC::GenChem::Phases_and_Phase_Changes` `Prereq::GenChem::Gas_Phase` `Prereq::GenChem::Intermolecular_Forces` `MCAT::Chem_Phys` `Difficulty::4`

## GenChem::Solutions_and_Solubility

### MCAT-GCH-SOLS-001

- **KC:** `GenChem::Solutions_and_Solubility`
- **Prereqs:** `Prereq::GenChem::Intermolecular_Forces`
- **Difficulty:** 2
- **Question:** The like dissolves like guideline predicts that a polar solute will dissolve best in:
- **A:** a nonpolar solvent
- **B:** a polar solvent
- **C:** any gas
- **D:** a metallic solid
- **Correct:** B
- **Explanation:** Polar solutes interact favorably with polar solvents through dipole and hydrogen-bonding interactions, so they dissolve best in polar solvents.
- **Tags:** `KC::GenChem::Solutions_and_Solubility` `Prereq::GenChem::Intermolecular_Forces` `MCAT::Chem_Phys` `Difficulty::2`

## GenChem::Ions_in_Solutions

### MCAT-GCH-IONS-001

- **KC:** `GenChem::Ions_in_Solutions`
- **Prereqs:** `Prereq::GenChem::Solutions_and_Solubility`
- **Difficulty:** 3
- **Question:** Equal concentrations of a strong electrolyte and a weak electrolyte are dissolved in separate water samples. Which conducts electricity better, and why?
- **A:** The strong electrolyte, because it produces more dissolved ions
- **B:** The weak electrolyte, because its molecules stay intact
- **C:** They conduct equally because the concentrations are equal
- **D:** Neither solution conducts electricity
- **Correct:** A
- **Explanation:** A strong electrolyte dissociates completely into mobile ions, so it provides more charge carriers and conducts better than a partially dissociated weak electrolyte.
- **Tags:** `KC::GenChem::Ions_in_Solutions` `Prereq::GenChem::Solutions_and_Solubility` `MCAT::Chem_Phys` `Difficulty::3`

## GenChem::Spectrophotometry

### MCAT-GCH-SPE-001

- **KC:** `GenChem::Spectrophotometry`
- **Prereqs:** `Prereq::GenChem::Solutions_and_Solubility` `Prereq::Physics::Electromagnetic_Radiation`
- **Difficulty:** 4
- **Question:** A colored solution's absorbance is measured in a spectrophotometer. If the cuvette path length is doubled while concentration stays constant, the absorbance will:
- **A:** be halved
- **B:** be unchanged
- **C:** drop to zero
- **D:** double
- **Correct:** D
- **Explanation:** By the Beer-Lambert law (A = e b c) absorbance is proportional to path length, so doubling the path length doubles the absorbance.
- **Tags:** `KC::GenChem::Spectrophotometry` `Prereq::GenChem::Solutions_and_Solubility` `Prereq::Physics::Electromagnetic_Radiation` `MCAT::Chem_Phys` `Difficulty::4`

## GenChem::Colligative_Properties

### MCAT-GCH-COLP-001

- **KC:** `GenChem::Colligative_Properties`
- **Prereqs:** `Prereq::GenChem::Phases_and_Phase_Changes` `Prereq::GenChem::Solutions_and_Solubility`
- **Difficulty:** 4
- **Question:** Assuming equal molal concentrations, which aqueous solution has the highest boiling point?
- **A:** glucose, a nonelectrolyte
- **B:** NaCl
- **C:** pure water
- **D:** CaCl2
- **Correct:** D
- **Explanation:** Boiling-point elevation depends on the number of dissolved particles, and CaCl2 releases about three ions per formula unit, more than NaCl (two) or glucose (one).
- **Tags:** `KC::GenChem::Colligative_Properties` `Prereq::GenChem::Phases_and_Phase_Changes` `Prereq::GenChem::Solutions_and_Solubility` `MCAT::Chem_Phys` `Difficulty::4`

## GenChem::Solubility_Equilibria

### MCAT-GCH-SOLE-001

- **KC:** `GenChem::Solubility_Equilibria`
- **Prereqs:** `Prereq::GenChem::Equilibrium` `Prereq::GenChem::Ions_in_Solutions`
- **Difficulty:** 5
- **Question:** For a salt with the formula MX2 and molar solubility s, which expression gives its solubility product Ksp?
- **A:** Ksp = s squared
- **B:** Ksp = 2 times s squared
- **C:** Ksp = 4 times s cubed
- **D:** Ksp = s cubed
- **Correct:** C
- **Explanation:** Dissolving gives a metal-ion concentration of s and an X-ion concentration of 2s, so Ksp = (s)(2s) squared = 4 times s cubed.
- **Tags:** `KC::GenChem::Solubility_Equilibria` `Prereq::GenChem::Equilibrium` `Prereq::GenChem::Ions_in_Solutions` `MCAT::Chem_Phys` `Difficulty::5`

## GenChem::Thermochemistry

### MCAT-GCH-THE-001

- **KC:** `GenChem::Thermochemistry`
- **Prereqs:** `Prereq::GenChem::Stoichiometry` `Prereq::Physics::Work_And_Energy`
- **Difficulty:** 3
- **Question:** About how much heat is needed to raise the temperature of 100 g of water by 10 degrees C, given that the specific heat of water is about 4.18 J per gram per degree C?
- **A:** about 42 J
- **B:** about 418 J
- **C:** about 4180 J
- **D:** about 41800 J
- **Correct:** C
- **Explanation:** Heat equals mass times specific heat times temperature change, so q = 100 g times 4.18 times 10 degrees C, which is about 4180 J.
- **Tags:** `KC::GenChem::Thermochemistry` `Prereq::GenChem::Stoichiometry` `Prereq::Physics::Work_And_Energy` `MCAT::Chem_Phys` `Difficulty::3`

## GenChem::Thermodynamics

### MCAT-GCH-THE2-001

- **KC:** `GenChem::Thermodynamics`
- **Prereqs:** `Prereq::GenChem::Thermochemistry` `Prereq::Physics::Thermodynamics`
- **Difficulty:** 4
- **Question:** A reaction is exothermic (delta H less than 0) and decreases entropy (delta S less than 0). At which temperatures is it spontaneous?
- **A:** at all temperatures
- **B:** only at sufficiently low temperatures
- **C:** at no temperature
- **D:** only at high temperatures
- **Correct:** B
- **Explanation:** With negative delta H and negative delta S, delta G = delta H minus T times delta S is negative only when T is small enough, so it is spontaneous at low temperatures.
- **Tags:** `KC::GenChem::Thermodynamics` `Prereq::GenChem::Thermochemistry` `Prereq::Physics::Thermodynamics` `MCAT::Chem_Phys` `Difficulty::4`

## GenChem::Kinetics

### MCAT-GCH-KIN-001

- **KC:** `GenChem::Kinetics`
- **Prereqs:** `Prereq::GenChem::Stoichiometry` `Prereq::GenChem::Thermochemistry`
- **Difficulty:** 4
- **Question:** A reaction is second order in reactant A. If the concentration of A is doubled, the reaction rate changes by a factor of:
- **A:** 2
- **B:** 4
- **C:** 8
- **D:** no change
- **Correct:** B
- **Explanation:** For a second-order dependence the rate is proportional to concentration squared, so doubling A multiplies the rate by 2 squared, which is 4.
- **Tags:** `KC::GenChem::Kinetics` `Prereq::GenChem::Stoichiometry` `Prereq::GenChem::Thermochemistry` `MCAT::Chem_Phys` `Difficulty::4`

## GenChem::Equilibrium

### MCAT-GCH-EQU-001

- **KC:** `GenChem::Equilibrium`
- **Prereqs:** `Prereq::GenChem::Kinetics`
- **Difficulty:** 3
- **Question:** A reaction is at equilibrium. According to Le Chatelier's principle, adding more reactant will:
- **A:** shift the equilibrium toward products
- **B:** shift the equilibrium toward reactants
- **C:** have no effect on the equilibrium position
- **D:** change the value of the equilibrium constant
- **Correct:** A
- **Explanation:** Adding reactant is a stress the system relieves by shifting toward products; the equilibrium constant itself is unchanged at constant temperature.
- **Tags:** `KC::GenChem::Equilibrium` `Prereq::GenChem::Kinetics` `MCAT::Chem_Phys` `Difficulty::3`

## GenChem::Acid_Base_Equilibria

### MCAT-GCH-ACIBE-001

- **KC:** `GenChem::Acid_Base_Equilibria`
- **Prereqs:** `Prereq::GenChem::Equilibrium`
- **Difficulty:** 2
- **Question:** What is the pH of a 0.010 M solution of the strong acid HCl at 25 degrees C?
- **A:** 1
- **B:** 2
- **C:** 10
- **D:** 12
- **Correct:** B
- **Explanation:** HCl dissociates completely, so the hydrogen-ion concentration is 0.010 M and pH = negative log of 0.010 = 2.
- **Tags:** `KC::GenChem::Acid_Base_Equilibria` `Prereq::GenChem::Equilibrium` `MCAT::Chem_Phys` `Difficulty::2`

## GenChem::Buffers

### MCAT-GCH-BUF-001

- **KC:** `GenChem::Buffers`
- **Prereqs:** `Prereq::GenChem::Acid_Base_Equilibria`
- **Difficulty:** 4
- **Question:** Adding a small amount of strong acid to an effective buffer produces only a small change in pH. Why?
- **A:** The added strong acid does not dissociate in the buffer
- **B:** The buffer raises the acid's Ka to compensate
- **C:** The added acid reacts with water to raise the pH sharply
- **D:** The conjugate base component neutralizes most of the added acid
- **Correct:** D
- **Explanation:** A buffer contains a weak acid and its conjugate base; the conjugate base consumes added hydrogen ions, so the pH changes only slightly.
- **Tags:** `KC::GenChem::Buffers` `Prereq::GenChem::Acid_Base_Equilibria` `MCAT::Chem_Phys` `Difficulty::4`

## GenChem::Titration

### MCAT-GCH-TIT-001

- **KC:** `GenChem::Titration`
- **Prereqs:** `Prereq::GenChem::Acid_Base_Equilibria` `Prereq::GenChem::Buffers`
- **Difficulty:** 5
- **Question:** During the titration of a weak acid with a strong base, the pH at the half-equivalence point is equal to:
- **A:** 7.0 for every weak acid
- **B:** the pKa of the weak acid
- **C:** 0
- **D:** the pKb of the conjugate base
- **Correct:** B
- **Explanation:** At half-equivalence, half the weak acid has become its conjugate base, so their concentrations are equal and the pH equals the acid's pKa.
- **Tags:** `KC::GenChem::Titration` `Prereq::GenChem::Acid_Base_Equilibria` `Prereq::GenChem::Buffers` `MCAT::Chem_Phys` `Difficulty::5`

## GenChem::Electrochemistry

### MCAT-GCH-ELE-001

- **KC:** `GenChem::Electrochemistry`
- **Prereqs:** `Prereq::GenChem::Redox_Reactions` `Prereq::GenChem::Thermodynamics` `Prereq::Physics::Electrical_Circuits` `Prereq::Physics::Electrostatics`
- **Difficulty:** 3
- **Question:** In a galvanic (voltaic) cell, where does oxidation occur, and what is that electrode's conventional sign?
- **A:** at the anode, which is the negative electrode
- **B:** at the cathode, which is the positive electrode
- **C:** at the cathode, which is the negative electrode
- **D:** at the anode, which is the positive electrode
- **Correct:** A
- **Explanation:** Oxidation always occurs at the anode, and in a galvanic cell the anode is the negative electrode.
- **Tags:** `KC::GenChem::Electrochemistry` `Prereq::GenChem::Redox_Reactions` `Prereq::GenChem::Thermodynamics` `Prereq::Physics::Electrical_Circuits` `Prereq::Physics::Electrostatics` `MCAT::Chem_Phys` `Difficulty::3`

## GenChem::Nuclear_Chemistry

### MCAT-GCH-NUCC-001

- **KC:** `GenChem::Nuclear_Chemistry`
- **Prereqs:** `Prereq::GenChem::Atomic_Structure` `Prereq::Physics::Atomic_Structure` `Prereq::Physics::Nuclear_Physics`
- **Difficulty:** 2
- **Question:** During alpha decay, how do the atomic number and mass number of the nucleus change?
- **A:** The atomic number decreases by 2 and the mass number decreases by 4
- **B:** The atomic number increases by 1 with no change in mass number
- **C:** The atomic number decreases by 1 and the mass number decreases by 1
- **D:** Both the atomic number and mass number are unchanged
- **Correct:** A
- **Explanation:** An alpha particle is a helium-4 nucleus, so emitting it lowers the atomic number by 2 and the mass number by 4.
- **Tags:** `KC::GenChem::Nuclear_Chemistry` `Prereq::GenChem::Atomic_Structure` `Prereq::Physics::Atomic_Structure` `Prereq::Physics::Nuclear_Physics` `MCAT::Chem_Phys` `Difficulty::2`
