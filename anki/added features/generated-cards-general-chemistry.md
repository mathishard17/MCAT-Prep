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

### MCAT-GCH-ATOS-002

- **KC:** `GenChem::Atomic_Structure`
- **Prereqs:** none
- **Difficulty:** 2
- **Question:** An ion has 11 protons, 12 neutrons, and 10 electrons. What is its net charge?
- **A:** +1
- **B:** -1
- **C:** +2
- **D:** 0
- **Correct:** A
- **Explanation:** Net charge equals the number of protons minus the number of electrons, 11 - 10 = +1; neutrons do not affect charge.
- **Tags:** `KC::GenChem::Atomic_Structure` `MCAT::Chem_Phys` `Difficulty::2`

### MCAT-GCH-ATOS-003

- **KC:** `GenChem::Atomic_Structure`
- **Prereqs:** none
- **Difficulty:** 3
- **Question:** Two neutral atoms of the same element always share the same number of protons but may differ in their number of:
- **A:** neutrons
- **B:** protons
- **C:** electrons
- **D:** occupied electron shells
- **Correct:** A
- **Explanation:** Atoms of one element that differ only in neutron count are isotopes; a neutral atom's proton and electron numbers are fixed by the element.
- **Tags:** `KC::GenChem::Atomic_Structure` `MCAT::Chem_Phys` `Difficulty::3`

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

### MCAT-GCH-ELEC-002

- **KC:** `GenChem::Electron_Configuration`
- **Prereqs:** `Prereq::GenChem::Atomic_Structure` `Prereq::Physics::Atomic_Structure`
- **Difficulty:** 2
- **Question:** How many valence electrons does a neutral phosphorus atom have, given its configuration 1s2 2s2 2p6 3s2 3p3?
- **A:** 3
- **B:** 5
- **C:** 7
- **D:** 15
- **Correct:** B
- **Explanation:** Valence electrons occupy the outermost principal shell (n = 3), and 3s2 3p3 contains 2 + 3 = 5 electrons.
- **Tags:** `KC::GenChem::Electron_Configuration` `Prereq::GenChem::Atomic_Structure` `Prereq::Physics::Atomic_Structure` `MCAT::Chem_Phys` `Difficulty::2`

### MCAT-GCH-ELEC-003

- **KC:** `GenChem::Electron_Configuration`
- **Prereqs:** `Prereq::GenChem::Atomic_Structure` `Prereq::Physics::Atomic_Structure`
- **Difficulty:** 4
- **Question:** Which is the correct ground-state electron configuration of a neutral iron atom (Z = 26)?
- **A:** [Ar] 4s2 3d6
- **B:** [Ar] 4s2 3d8
- **C:** [Ar] 3d8
- **D:** [Ar] 4s1 3d7
- **Correct:** A
- **Explanation:** After the 18-electron argon core, the remaining 8 electrons fill 4s before 3d, giving 4s2 3d6.
- **Tags:** `KC::GenChem::Electron_Configuration` `Prereq::GenChem::Atomic_Structure` `Prereq::Physics::Atomic_Structure` `MCAT::Chem_Phys` `Difficulty::4`

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

### MCAT-GCH-ATOSQ-002

- **KC:** `GenChem::Atomic_Spectra_and_Quantum`
- **Prereqs:** `Prereq::GenChem::Electron_Configuration` `Prereq::Physics::Electromagnetic_Radiation`
- **Difficulty:** 3
- **Question:** In the Bohr model of hydrogen, absorbing a photon of exactly the right energy causes an electron to:
- **A:** move to a higher energy level
- **B:** drop to a lower energy level
- **C:** leave the atom regardless of the photon's energy
- **D:** change only the direction of its spin
- **Correct:** A
- **Explanation:** Absorbed energy promotes the electron to a higher allowed level; a drop to a lower level instead accompanies photon emission.
- **Tags:** `KC::GenChem::Atomic_Spectra_and_Quantum` `Prereq::GenChem::Electron_Configuration` `Prereq::Physics::Electromagnetic_Radiation` `MCAT::Chem_Phys` `Difficulty::3`

### MCAT-GCH-ATOSQ-003

- **KC:** `GenChem::Atomic_Spectra_and_Quantum`
- **Prereqs:** `Prereq::GenChem::Electron_Configuration` `Prereq::Physics::Electromagnetic_Radiation`
- **Difficulty:** 5
- **Question:** In hydrogen, the n = 3 to n = 2 transition emits red light near 656 nm. Compared with it, the n = 4 to n = 2 transition emits light with:
- **A:** a longer wavelength
- **B:** a shorter wavelength
- **C:** an identical wavelength
- **D:** no photon at all
- **Correct:** B
- **Explanation:** The n = 4 to n = 2 energy gap is larger than the n = 3 to n = 2 gap, so the emitted photon has more energy and a shorter wavelength.
- **Tags:** `KC::GenChem::Atomic_Spectra_and_Quantum` `Prereq::GenChem::Electron_Configuration` `Prereq::Physics::Electromagnetic_Radiation` `MCAT::Chem_Phys` `Difficulty::5`

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

### MCAT-GCH-PERT-002

- **KC:** `GenChem::Periodic_Trends`
- **Prereqs:** `Prereq::GenChem::Electron_Configuration`
- **Difficulty:** 3
- **Question:** Among Na, Cl, F, and Cs, which element has the highest first ionization energy?
- **A:** Cs
- **B:** Na
- **C:** Cl
- **D:** F
- **Correct:** D
- **Explanation:** First ionization energy increases up and to the right in the periodic table, so fluorine, the highest and farthest right of these, has the largest value.
- **Tags:** `KC::GenChem::Periodic_Trends` `Prereq::GenChem::Electron_Configuration` `MCAT::Chem_Phys` `Difficulty::3`

### MCAT-GCH-PERT-003

- **KC:** `GenChem::Periodic_Trends`
- **Prereqs:** `Prereq::GenChem::Electron_Configuration`
- **Difficulty:** 1
- **Question:** Which property generally increases when moving from left to right across a period?
- **A:** electronegativity
- **B:** atomic radius
- **C:** metallic character
- **D:** the number of occupied electron shells
- **Correct:** A
- **Explanation:** Rising effective nuclear charge across a period increases electronegativity, whereas atomic radius and metallic character decrease and the number of shells stays constant.
- **Tags:** `KC::GenChem::Periodic_Trends` `Prereq::GenChem::Electron_Configuration` `MCAT::Chem_Phys` `Difficulty::1`

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

### MCAT-GCH-CHEB-002

- **KC:** `GenChem::Chemical_Bonding`
- **Prereqs:** `Prereq::GenChem::Periodic_Trends`
- **Difficulty:** 2
- **Question:** Which type of bond forms when two nonmetal atoms share one or more pairs of electrons?
- **A:** ionic
- **B:** covalent
- **C:** metallic
- **D:** hydrogen
- **Correct:** B
- **Explanation:** Sharing of electron pairs between nonmetals defines a covalent bond; ionic bonding instead involves electron transfer between a metal and a nonmetal.
- **Tags:** `KC::GenChem::Chemical_Bonding` `Prereq::GenChem::Periodic_Trends` `MCAT::Chem_Phys` `Difficulty::2`

### MCAT-GCH-CHEB-003

- **KC:** `GenChem::Chemical_Bonding`
- **Prereqs:** `Prereq::GenChem::Periodic_Trends`
- **Difficulty:** 3
- **Question:** A diatomic molecule forms between two atoms whose electronegativity difference is about 0.5. This bond is best described as:
- **A:** nonpolar covalent
- **B:** polar covalent
- **C:** ionic
- **D:** metallic
- **Correct:** B
- **Explanation:** A small but nonzero electronegativity difference (roughly 0.4 to 1.7) produces unequal electron sharing, which is a polar covalent bond.
- **Tags:** `KC::GenChem::Chemical_Bonding` `Prereq::GenChem::Periodic_Trends` `MCAT::Chem_Phys` `Difficulty::3`

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

### MCAT-GCH-MOLG-002

- **KC:** `GenChem::Molecular_Geometry`
- **Prereqs:** `Prereq::GenChem::Chemical_Bonding`
- **Difficulty:** 2
- **Question:** Using VSEPR theory, what is the molecular shape of a central atom with three bonding domains and no lone pairs, such as in BF3?
- **A:** bent
- **B:** trigonal planar
- **C:** tetrahedral
- **D:** linear
- **Correct:** B
- **Explanation:** Three electron domains with no lone pairs spread out into a trigonal planar shape with bond angles near 120 degrees.
- **Tags:** `KC::GenChem::Molecular_Geometry` `Prereq::GenChem::Chemical_Bonding` `MCAT::Chem_Phys` `Difficulty::2`

### MCAT-GCH-MOLG-003

- **KC:** `GenChem::Molecular_Geometry`
- **Prereqs:** `Prereq::GenChem::Chemical_Bonding`
- **Difficulty:** 4
- **Question:** In a water molecule the central oxygen has two bonding pairs and two lone pairs. What is its molecular geometry?
- **A:** linear
- **B:** bent
- **C:** trigonal planar
- **D:** tetrahedral
- **Correct:** B
- **Explanation:** Four electron domains give a tetrahedral electron geometry, but the two lone pairs leave a bent molecular shape with an angle near 104.5 degrees.
- **Tags:** `KC::GenChem::Molecular_Geometry` `Prereq::GenChem::Chemical_Bonding` `MCAT::Chem_Phys` `Difficulty::4`

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

### MCAT-GCH-STO-002

- **KC:** `GenChem::Stoichiometry`
- **Prereqs:** `Prereq::GenChem::Chemical_Bonding`
- **Difficulty:** 1
- **Question:** How many moles are present in 36 g of water, which has a molar mass of about 18 g/mol?
- **A:** 0.5 mol
- **B:** 1 mol
- **C:** 2 mol
- **D:** 18 mol
- **Correct:** C
- **Explanation:** Moles equal mass divided by molar mass, so 36 g / 18 g/mol = 2 mol.
- **Tags:** `KC::GenChem::Stoichiometry` `Prereq::GenChem::Chemical_Bonding` `MCAT::Chem_Phys` `Difficulty::1`

### MCAT-GCH-STO-003

- **KC:** `GenChem::Stoichiometry`
- **Prereqs:** `Prereq::GenChem::Chemical_Bonding`
- **Difficulty:** 3
- **Question:** For the reaction N2 + 3 H2 -> 2 NH3, how many moles of H2 are needed to react completely with 2 mol of N2?
- **A:** 2 mol
- **B:** 3 mol
- **C:** 6 mol
- **D:** 9 mol
- **Correct:** C
- **Explanation:** The mole ratio of N2 to H2 is 1 to 3, so 2 mol of N2 requires 3 times 2 = 6 mol of H2.
- **Tags:** `KC::GenChem::Stoichiometry` `Prereq::GenChem::Chemical_Bonding` `MCAT::Chem_Phys` `Difficulty::3`

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

### MCAT-GCH-REAT-002

- **KC:** `GenChem::Reaction_Types`
- **Prereqs:** `Prereq::GenChem::Stoichiometry`
- **Difficulty:** 2
- **Question:** The reaction Zn + 2 HCl -> ZnCl2 + H2 is best classified as which type?
- **A:** synthesis
- **B:** decomposition
- **C:** single-displacement
- **D:** acid-base neutralization
- **Correct:** C
- **Explanation:** An element (zinc) replaces another element (hydrogen) within a compound, which is the defining feature of a single-displacement reaction.
- **Tags:** `KC::GenChem::Reaction_Types` `Prereq::GenChem::Stoichiometry` `MCAT::Chem_Phys` `Difficulty::2`

### MCAT-GCH-REAT-003

- **KC:** `GenChem::Reaction_Types`
- **Prereqs:** `Prereq::GenChem::Stoichiometry`
- **Difficulty:** 4
- **Question:** Which of the following is a combustion reaction?
- **A:** CH4 + 2 O2 -> CO2 + 2 H2O
- **B:** HCl + NaOH -> NaCl + H2O
- **C:** AgNO3 + NaCl -> AgCl + NaNO3
- **D:** CaCO3 -> CaO + CO2
- **Correct:** A
- **Explanation:** Combustion is the reaction of a fuel with oxygen to form oxides such as CO2 and water; the other choices are neutralization, precipitation, and decomposition.
- **Tags:** `KC::GenChem::Reaction_Types` `Prereq::GenChem::Stoichiometry` `MCAT::Chem_Phys` `Difficulty::4`

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

### MCAT-GCH-REDR-002

- **KC:** `GenChem::Redox_Reactions`
- **Prereqs:** `Prereq::GenChem::Reaction_Types`
- **Difficulty:** 2
- **Question:** What is the oxidation state of sulfur in the sulfate ion, SO4^2-?
- **A:** +2
- **B:** +4
- **C:** +6
- **D:** -2
- **Correct:** C
- **Explanation:** Each oxygen contributes -2 for a total of -8; for the ion's overall charge to be -2, sulfur must be +6.
- **Tags:** `KC::GenChem::Redox_Reactions` `Prereq::GenChem::Reaction_Types` `MCAT::Chem_Phys` `Difficulty::2`

### MCAT-GCH-REDR-003

- **KC:** `GenChem::Redox_Reactions`
- **Prereqs:** `Prereq::GenChem::Reaction_Types`
- **Difficulty:** 4
- **Question:** In the half-reaction Fe -> Fe3+ + 3 e-, the iron is:
- **A:** oxidized and acts as a reducing agent
- **B:** reduced and acts as an oxidizing agent
- **C:** neither oxidized nor reduced
- **D:** reduced and acts as a reducing agent
- **Correct:** A
- **Explanation:** Iron loses three electrons, changing from oxidation state 0 to +3, so it is oxidized and serves as the reducing agent.
- **Tags:** `KC::GenChem::Redox_Reactions` `Prereq::GenChem::Reaction_Types` `MCAT::Chem_Phys` `Difficulty::4`

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

### MCAT-GCH-GASP-002

- **KC:** `GenChem::Gas_Phase`
- **Prereqs:** `Prereq::GenChem::Stoichiometry` `Prereq::Physics::Thermodynamics`
- **Difficulty:** 2
- **Question:** At constant temperature, if the pressure on a fixed quantity of ideal gas is doubled, its volume:
- **A:** doubles
- **B:** is halved
- **C:** is unchanged
- **D:** quadruples
- **Correct:** B
- **Explanation:** Boyle's law states that pressure and volume are inversely proportional at constant temperature, so doubling the pressure halves the volume.
- **Tags:** `KC::GenChem::Gas_Phase` `Prereq::GenChem::Stoichiometry` `Prereq::Physics::Thermodynamics` `MCAT::Chem_Phys` `Difficulty::2`

### MCAT-GCH-GASP-003

- **KC:** `GenChem::Gas_Phase`
- **Prereqs:** `Prereq::GenChem::Stoichiometry` `Prereq::Physics::Thermodynamics`
- **Difficulty:** 4
- **Question:** A 2.0 L sample of ideal gas is at 3.0 atm. If it is compressed to 1.0 L at constant temperature, what is the new pressure?
- **A:** 1.5 atm
- **B:** 3.0 atm
- **C:** 6.0 atm
- **D:** 12 atm
- **Correct:** C
- **Explanation:** By Boyle's law P1 times V1 = P2 times V2, so P2 = (3.0 atm)(2.0 L) / (1.0 L) = 6.0 atm.
- **Tags:** `KC::GenChem::Gas_Phase` `Prereq::GenChem::Stoichiometry` `Prereq::Physics::Thermodynamics` `MCAT::Chem_Phys` `Difficulty::4`

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

### MCAT-GCH-INTF-002

- **KC:** `GenChem::Intermolecular_Forces`
- **Prereqs:** `Prereq::GenChem::Molecular_Geometry`
- **Difficulty:** 1
- **Question:** Which of the following is generally the weakest intermolecular force?
- **A:** hydrogen bonding
- **B:** dipole-dipole attraction
- **C:** London dispersion forces
- **D:** ion-dipole attraction
- **Correct:** C
- **Explanation:** London dispersion forces arise from temporary induced dipoles and are the weakest of these interactions, though they act between all molecules.
- **Tags:** `KC::GenChem::Intermolecular_Forces` `Prereq::GenChem::Molecular_Geometry` `MCAT::Chem_Phys` `Difficulty::1`

### MCAT-GCH-INTF-003

- **KC:** `GenChem::Intermolecular_Forces`
- **Prereqs:** `Prereq::GenChem::Molecular_Geometry`
- **Difficulty:** 3
- **Question:** Which molecule can act as a hydrogen-bond donor?
- **A:** CH4
- **B:** NH3
- **C:** CO2
- **D:** CCl4
- **Correct:** B
- **Explanation:** Hydrogen bonding requires a hydrogen bonded directly to nitrogen, oxygen, or fluorine; only NH3 has N-H bonds.
- **Tags:** `KC::GenChem::Intermolecular_Forces` `Prereq::GenChem::Molecular_Geometry` `MCAT::Chem_Phys` `Difficulty::3`

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

### MCAT-GCH-PHAPC-002

- **KC:** `GenChem::Phases_and_Phase_Changes`
- **Prereqs:** `Prereq::GenChem::Gas_Phase` `Prereq::GenChem::Intermolecular_Forces`
- **Difficulty:** 2
- **Question:** Which of the following phase changes is exothermic?
- **A:** melting
- **B:** boiling
- **C:** freezing
- **D:** sublimation
- **Correct:** C
- **Explanation:** Freezing releases heat as particles settle into an ordered solid, whereas melting, boiling, and sublimation all absorb heat.
- **Tags:** `KC::GenChem::Phases_and_Phase_Changes` `Prereq::GenChem::Gas_Phase` `Prereq::GenChem::Intermolecular_Forces` `MCAT::Chem_Phys` `Difficulty::2`

### MCAT-GCH-PHAPC-003

- **KC:** `GenChem::Phases_and_Phase_Changes`
- **Prereqs:** `Prereq::GenChem::Gas_Phase` `Prereq::GenChem::Intermolecular_Forces`
- **Difficulty:** 3
- **Question:** On a heating curve, the temperature stays constant during a phase change even as heat is added because the energy is used to:
- **A:** raise the average kinetic energy of the particles
- **B:** overcome the intermolecular forces holding the particles together
- **C:** break covalent bonds within the molecules
- **D:** increase the pressure of the system
- **Correct:** B
- **Explanation:** During a phase change the absorbed heat separates particles by overcoming intermolecular forces, so average kinetic energy, and thus temperature, does not change.
- **Tags:** `KC::GenChem::Phases_and_Phase_Changes` `Prereq::GenChem::Gas_Phase` `Prereq::GenChem::Intermolecular_Forces` `MCAT::Chem_Phys` `Difficulty::3`

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

### MCAT-GCH-SOLS-002

- **KC:** `GenChem::Solutions_and_Solubility`
- **Prereqs:** `Prereq::GenChem::Intermolecular_Forces`
- **Difficulty:** 1
- **Question:** What is the molarity of a solution made by dissolving 0.50 mol of NaCl in enough water to make 2.0 L of solution?
- **A:** 0.25 mol/L
- **B:** 0.50 mol/L
- **C:** 1.0 mol/L
- **D:** 2.0 mol/L
- **Correct:** A
- **Explanation:** Molarity equals moles of solute divided by liters of solution, so 0.50 mol / 2.0 L = 0.25 mol/L.
- **Tags:** `KC::GenChem::Solutions_and_Solubility` `Prereq::GenChem::Intermolecular_Forces` `MCAT::Chem_Phys` `Difficulty::1`

### MCAT-GCH-SOLS-003

- **KC:** `GenChem::Solutions_and_Solubility`
- **Prereqs:** `Prereq::GenChem::Intermolecular_Forces`
- **Difficulty:** 3
- **Question:** The solubility of most solid ionic compounds in water tends to increase when:
- **A:** the temperature is raised
- **B:** the temperature is lowered
- **C:** the solution is left completely unstirred
- **D:** the pressure above the solution is decreased
- **Correct:** A
- **Explanation:** Dissolving most solids is favored at higher temperatures, so their solubility rises with temperature; pressure changes mainly affect the solubility of gases.
- **Tags:** `KC::GenChem::Solutions_and_Solubility` `Prereq::GenChem::Intermolecular_Forces` `MCAT::Chem_Phys` `Difficulty::3`

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

### MCAT-GCH-IONS-002

- **KC:** `GenChem::Ions_in_Solutions`
- **Prereqs:** `Prereq::GenChem::Solutions_and_Solubility`
- **Difficulty:** 2
- **Question:** Which of the following behaves as a strong electrolyte when dissolved in water?
- **A:** glucose
- **B:** ethanol
- **C:** NaCl
- **D:** dissolved oxygen gas
- **Correct:** C
- **Explanation:** NaCl dissociates completely into Na+ and Cl- ions, making it a strong electrolyte; glucose and ethanol dissolve as neutral molecules and conduct poorly.
- **Tags:** `KC::GenChem::Ions_in_Solutions` `Prereq::GenChem::Solutions_and_Solubility` `MCAT::Chem_Phys` `Difficulty::2`

### MCAT-GCH-IONS-003

- **KC:** `GenChem::Ions_in_Solutions`
- **Prereqs:** `Prereq::GenChem::Solutions_and_Solubility`
- **Difficulty:** 4
- **Question:** A solution must be electrically neutral. If it contains 0.10 mol/L Na+ and 0.10 mol/L Ca2+, what concentration of Cl- (a -1 ion) is required to balance the charge?
- **A:** 0.10 mol/L
- **B:** 0.20 mol/L
- **C:** 0.30 mol/L
- **D:** 0.40 mol/L
- **Correct:** C
- **Explanation:** Total cation charge is 0.10 + 2 times 0.10 = 0.30 mol/L of positive charge, so 0.30 mol/L of a -1 anion is needed for balance.
- **Tags:** `KC::GenChem::Ions_in_Solutions` `Prereq::GenChem::Solutions_and_Solubility` `MCAT::Chem_Phys` `Difficulty::4`

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

### MCAT-GCH-SPE-002

- **KC:** `GenChem::Spectrophotometry`
- **Prereqs:** `Prereq::GenChem::Solutions_and_Solubility` `Prereq::Physics::Electromagnetic_Radiation`
- **Difficulty:** 3
- **Question:** In the Beer-Lambert law A = e b c, if the concentration c is tripled while the path length stays constant, the absorbance A will:
- **A:** stay the same
- **B:** be reduced to one third
- **C:** triple
- **D:** increase ninefold
- **Correct:** C
- **Explanation:** Absorbance is directly proportional to concentration, so tripling the concentration triples the absorbance.
- **Tags:** `KC::GenChem::Spectrophotometry` `Prereq::GenChem::Solutions_and_Solubility` `Prereq::Physics::Electromagnetic_Radiation` `MCAT::Chem_Phys` `Difficulty::3`

### MCAT-GCH-SPE-003

- **KC:** `GenChem::Spectrophotometry`
- **Prereqs:** `Prereq::GenChem::Solutions_and_Solubility` `Prereq::Physics::Electromagnetic_Radiation`
- **Difficulty:** 5
- **Question:** A spectrophotometric assay is usually run at the analyte's wavelength of maximum absorbance (lambda max) mainly to:
- **A:** minimize the measured absorbance
- **B:** maximize the sensitivity of the measurement
- **C:** ensure that no light is absorbed
- **D:** detect only scattered light
- **Correct:** B
- **Explanation:** At lambda max the absorbance changes most per unit concentration, giving the greatest sensitivity and the most reliable measurements.
- **Tags:** `KC::GenChem::Spectrophotometry` `Prereq::GenChem::Solutions_and_Solubility` `Prereq::Physics::Electromagnetic_Radiation` `MCAT::Chem_Phys` `Difficulty::5`

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

### MCAT-GCH-COLP-002

- **KC:** `GenChem::Colligative_Properties`
- **Prereqs:** `Prereq::GenChem::Phases_and_Phase_Changes` `Prereq::GenChem::Solutions_and_Solubility`
- **Difficulty:** 3
- **Question:** Which of the following is a colligative property?
- **A:** density
- **B:** freezing-point depression
- **C:** color
- **D:** viscosity
- **Correct:** B
- **Explanation:** Colligative properties depend on the number of dissolved particles rather than their identity, and freezing-point depression is a standard example.
- **Tags:** `KC::GenChem::Colligative_Properties` `Prereq::GenChem::Phases_and_Phase_Changes` `Prereq::GenChem::Solutions_and_Solubility` `MCAT::Chem_Phys` `Difficulty::3`

### MCAT-GCH-COLP-003

- **KC:** `GenChem::Colligative_Properties`
- **Prereqs:** `Prereq::GenChem::Phases_and_Phase_Changes` `Prereq::GenChem::Solutions_and_Solubility`
- **Difficulty:** 5
- **Question:** Assuming complete dissociation, what is the van't Hoff factor (i) for K2SO4 dissolved in water?
- **A:** 1
- **B:** 2
- **C:** 3
- **D:** 4
- **Correct:** C
- **Explanation:** K2SO4 dissociates into two K+ ions and one SO4^2- ion, giving three particles per formula unit, so i = 3.
- **Tags:** `KC::GenChem::Colligative_Properties` `Prereq::GenChem::Phases_and_Phase_Changes` `Prereq::GenChem::Solutions_and_Solubility` `MCAT::Chem_Phys` `Difficulty::5`

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

### MCAT-GCH-SOLE-002

- **KC:** `GenChem::Solubility_Equilibria`
- **Prereqs:** `Prereq::GenChem::Equilibrium` `Prereq::GenChem::Ions_in_Solutions`
- **Difficulty:** 3
- **Question:** The solubility product constant (Ksp) describes the equilibrium for:
- **A:** the rate at which a solid dissolves
- **B:** a slightly soluble ionic solid dissolving into its ions
- **C:** the pH of a buffer solution
- **D:** the boiling point of a solution
- **Correct:** B
- **Explanation:** Ksp is the equilibrium constant for the dissolution of a sparingly soluble ionic compound into its constituent ions.
- **Tags:** `KC::GenChem::Solubility_Equilibria` `Prereq::GenChem::Equilibrium` `Prereq::GenChem::Ions_in_Solutions` `MCAT::Chem_Phys` `Difficulty::3`

### MCAT-GCH-SOLE-003

- **KC:** `GenChem::Solubility_Equilibria`
- **Prereqs:** `Prereq::GenChem::Equilibrium` `Prereq::GenChem::Ions_in_Solutions`
- **Difficulty:** 4
- **Question:** Adding solid NaCl to a saturated solution of AgCl, which shares the chloride ion, will:
- **A:** increase the solubility of AgCl
- **B:** decrease the solubility of AgCl
- **C:** have no effect on the solubility of AgCl
- **D:** increase the value of Ksp
- **Correct:** B
- **Explanation:** The common-ion effect shifts the dissolution equilibrium back toward solid AgCl, lowering its solubility, while Ksp stays constant at a fixed temperature.
- **Tags:** `KC::GenChem::Solubility_Equilibria` `Prereq::GenChem::Equilibrium` `Prereq::GenChem::Ions_in_Solutions` `MCAT::Chem_Phys` `Difficulty::4`

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

### MCAT-GCH-THE-002

- **KC:** `GenChem::Thermochemistry`
- **Prereqs:** `Prereq::GenChem::Stoichiometry` `Prereq::Physics::Work_And_Energy`
- **Difficulty:** 2
- **Question:** An exothermic reaction is one that:
- **A:** absorbs heat and has a positive delta H
- **B:** releases heat and has a negative delta H
- **C:** neither absorbs nor releases heat
- **D:** always keeps the temperature constant
- **Correct:** B
- **Explanation:** Exothermic reactions release heat to the surroundings, which corresponds to a negative enthalpy change (delta H less than 0).
- **Tags:** `KC::GenChem::Thermochemistry` `Prereq::GenChem::Stoichiometry` `Prereq::Physics::Work_And_Energy` `MCAT::Chem_Phys` `Difficulty::2`

### MCAT-GCH-THE-003

- **KC:** `GenChem::Thermochemistry`
- **Prereqs:** `Prereq::GenChem::Stoichiometry` `Prereq::Physics::Work_And_Energy`
- **Difficulty:** 4
- **Question:** How much heat is released when 50 g of a substance with a specific heat of 2.0 J per gram per degree C cools by 20 degrees C?
- **A:** 200 J
- **B:** 1000 J
- **C:** 2000 J
- **D:** 4000 J
- **Correct:** C
- **Explanation:** Using q = m times c times delta T, the magnitude is 50 g times 2.0 times 20 degrees C = 2000 J of heat released.
- **Tags:** `KC::GenChem::Thermochemistry` `Prereq::GenChem::Stoichiometry` `Prereq::Physics::Work_And_Energy` `MCAT::Chem_Phys` `Difficulty::4`

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

### MCAT-GCH-THE2-002

- **KC:** `GenChem::Thermodynamics`
- **Prereqs:** `Prereq::GenChem::Thermochemistry` `Prereq::Physics::Thermodynamics`
- **Difficulty:** 3
- **Question:** In the relationship delta G = delta H - T delta S, a process is spontaneous when delta G is:
- **A:** positive
- **B:** negative
- **C:** zero
- **D:** equal to delta H
- **Correct:** B
- **Explanation:** A negative Gibbs free energy change indicates a spontaneous process at constant temperature and pressure.
- **Tags:** `KC::GenChem::Thermodynamics` `Prereq::GenChem::Thermochemistry` `Prereq::Physics::Thermodynamics` `MCAT::Chem_Phys` `Difficulty::3`

### MCAT-GCH-THE2-003

- **KC:** `GenChem::Thermodynamics`
- **Prereqs:** `Prereq::GenChem::Thermochemistry` `Prereq::Physics::Thermodynamics`
- **Difficulty:** 5
- **Question:** A reaction has delta H greater than 0 and delta S greater than 0. At which temperatures is it spontaneous?
- **A:** at all temperatures
- **B:** only at high temperatures
- **C:** only at low temperatures
- **D:** at no temperature
- **Correct:** B
- **Explanation:** With positive delta H and positive delta S, delta G = delta H - T delta S becomes negative only when T is large enough, so the reaction is spontaneous at high temperatures.
- **Tags:** `KC::GenChem::Thermodynamics` `Prereq::GenChem::Thermochemistry` `Prereq::Physics::Thermodynamics` `MCAT::Chem_Phys` `Difficulty::5`

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

### MCAT-GCH-KIN-002

- **KC:** `GenChem::Kinetics`
- **Prereqs:** `Prereq::GenChem::Stoichiometry` `Prereq::GenChem::Thermochemistry`
- **Difficulty:** 2
- **Question:** How does a catalyst speed up a chemical reaction?
- **A:** by increasing the activation energy
- **B:** by lowering the activation energy
- **C:** by shifting the equilibrium toward products
- **D:** by raising the temperature of the system
- **Correct:** B
- **Explanation:** A catalyst provides an alternative pathway with a lower activation energy and is not consumed, speeding both the forward and reverse reactions.
- **Tags:** `KC::GenChem::Kinetics` `Prereq::GenChem::Stoichiometry` `Prereq::GenChem::Thermochemistry` `MCAT::Chem_Phys` `Difficulty::2`

### MCAT-GCH-KIN-003

- **KC:** `GenChem::Kinetics`
- **Prereqs:** `Prereq::GenChem::Stoichiometry` `Prereq::GenChem::Thermochemistry`
- **Difficulty:** 3
- **Question:** For a reaction that is first order in reactant A, doubling the concentration of A changes the reaction rate by a factor of:
- **A:** 1 (no change)
- **B:** 2
- **C:** 4
- **D:** 8
- **Correct:** B
- **Explanation:** A first-order rate is directly proportional to the concentration of A, so doubling that concentration doubles the rate.
- **Tags:** `KC::GenChem::Kinetics` `Prereq::GenChem::Stoichiometry` `Prereq::GenChem::Thermochemistry` `MCAT::Chem_Phys` `Difficulty::3`

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

### MCAT-GCH-EQU-002

- **KC:** `GenChem::Equilibrium`
- **Prereqs:** `Prereq::GenChem::Kinetics`
- **Difficulty:** 2
- **Question:** For the general reaction aA + bB <-> cC + dD, the equilibrium constant K is expressed as:
- **A:** the product concentrations each raised to their coefficients, divided by the reactant concentrations each raised to their coefficients
- **B:** the reactant concentrations divided by the product concentrations
- **C:** the sum of the product and reactant concentrations
- **D:** the reactant concentrations multiplied by the product concentrations
- **Correct:** A
- **Explanation:** K equals the products, each concentration raised to its stoichiometric coefficient, divided by the reactants treated the same way.
- **Tags:** `KC::GenChem::Equilibrium` `Prereq::GenChem::Kinetics` `MCAT::Chem_Phys` `Difficulty::2`

### MCAT-GCH-EQU-003

- **KC:** `GenChem::Equilibrium`
- **Prereqs:** `Prereq::GenChem::Kinetics`
- **Difficulty:** 4
- **Question:** If a reaction's reaction quotient Q is momentarily less than its equilibrium constant K, the reaction will:
- **A:** proceed in the forward direction to form more products
- **B:** proceed in the reverse direction to form more reactants
- **C:** stop completely
- **D:** change the value of K
- **Correct:** A
- **Explanation:** When Q is less than K, the forward reaction is favored, producing more products until Q rises to equal K.
- **Tags:** `KC::GenChem::Equilibrium` `Prereq::GenChem::Kinetics` `MCAT::Chem_Phys` `Difficulty::4`

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

### MCAT-GCH-ACIBE-002

- **KC:** `GenChem::Acid_Base_Equilibria`
- **Prereqs:** `Prereq::GenChem::Equilibrium`
- **Difficulty:** 1
- **Question:** At 25 degrees C, what is the pOH of a solution whose pH is 9?
- **A:** 5
- **B:** 9
- **C:** 14
- **D:** -5
- **Correct:** A
- **Explanation:** At 25 degrees C, pH + pOH = 14, so pOH = 14 - 9 = 5.
- **Tags:** `KC::GenChem::Acid_Base_Equilibria` `Prereq::GenChem::Equilibrium` `MCAT::Chem_Phys` `Difficulty::1`

### MCAT-GCH-ACIBE-003

- **KC:** `GenChem::Acid_Base_Equilibria`
- **Prereqs:** `Prereq::GenChem::Equilibrium`
- **Difficulty:** 3
- **Question:** According to the Bronsted-Lowry definition, an acid is a substance that:
- **A:** donates a proton (H+)
- **B:** accepts a proton (H+)
- **C:** donates a lone pair of electrons
- **D:** accepts a hydroxide ion
- **Correct:** A
- **Explanation:** A Bronsted-Lowry acid is a proton (H+) donor, whereas a Bronsted-Lowry base is a proton acceptor.
- **Tags:** `KC::GenChem::Acid_Base_Equilibria` `Prereq::GenChem::Equilibrium` `MCAT::Chem_Phys` `Difficulty::3`

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

### MCAT-GCH-BUF-002

- **KC:** `GenChem::Buffers`
- **Prereqs:** `Prereq::GenChem::Acid_Base_Equilibria`
- **Difficulty:** 3
- **Question:** A buffer solution is most commonly prepared from:
- **A:** a strong acid and a strong base
- **B:** a weak acid and its conjugate base
- **C:** two different strong acids
- **D:** a soluble salt dissolved in pure water
- **Correct:** B
- **Explanation:** A buffer contains comparable amounts of a weak acid and its conjugate base (or a weak base and its conjugate acid), which lets it resist changes in pH.
- **Tags:** `KC::GenChem::Buffers` `Prereq::GenChem::Acid_Base_Equilibria` `MCAT::Chem_Phys` `Difficulty::3`

### MCAT-GCH-BUF-003

- **KC:** `GenChem::Buffers`
- **Prereqs:** `Prereq::GenChem::Acid_Base_Equilibria`
- **Difficulty:** 5
- **Question:** A buffer contains equal concentrations of a weak acid (pKa = 4.7) and its conjugate base. Using the Henderson-Hasselbalch equation, what is the pH?
- **A:** 4.7
- **B:** 7.0
- **C:** 9.3
- **D:** 14.0
- **Correct:** A
- **Explanation:** When the conjugate base and acid concentrations are equal, the log term is zero, so pH = pKa = 4.7.
- **Tags:** `KC::GenChem::Buffers` `Prereq::GenChem::Acid_Base_Equilibria` `MCAT::Chem_Phys` `Difficulty::5`

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

### MCAT-GCH-TIT-002

- **KC:** `GenChem::Titration`
- **Prereqs:** `Prereq::GenChem::Acid_Base_Equilibria` `Prereq::GenChem::Buffers`
- **Difficulty:** 3
- **Question:** At the equivalence point of a strong acid-strong base titration at 25 degrees C, the pH is:
- **A:** 0
- **B:** 7
- **C:** 14
- **D:** equal to the pKa of the acid
- **Correct:** B
- **Explanation:** A strong acid fully neutralized by a strong base leaves a neutral salt solution, so the pH at the equivalence point is 7.
- **Tags:** `KC::GenChem::Titration` `Prereq::GenChem::Acid_Base_Equilibria` `Prereq::GenChem::Buffers` `MCAT::Chem_Phys` `Difficulty::3`

### MCAT-GCH-TIT-003

- **KC:** `GenChem::Titration`
- **Prereqs:** `Prereq::GenChem::Acid_Base_Equilibria` `Prereq::GenChem::Buffers`
- **Difficulty:** 4
- **Question:** How many milliliters of 0.10 mol/L NaOH are required to neutralize 20 mL of 0.10 mol/L HCl?
- **A:** 10 mL
- **B:** 20 mL
- **C:** 40 mL
- **D:** 200 mL
- **Correct:** B
- **Explanation:** HCl and NaOH react in a 1 to 1 ratio, so with equal concentrations equal volumes are needed, and 20 mL of NaOH neutralizes 20 mL of HCl.
- **Tags:** `KC::GenChem::Titration` `Prereq::GenChem::Acid_Base_Equilibria` `Prereq::GenChem::Buffers` `MCAT::Chem_Phys` `Difficulty::4`

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

### MCAT-GCH-ELE-002

- **KC:** `GenChem::Electrochemistry`
- **Prereqs:** `Prereq::GenChem::Redox_Reactions` `Prereq::GenChem::Thermodynamics` `Prereq::Physics::Electrical_Circuits` `Prereq::Physics::Electrostatics`
- **Difficulty:** 2
- **Question:** In any electrochemical cell, reduction always takes place at the:
- **A:** anode
- **B:** cathode
- **C:** salt bridge
- **D:** wire connecting the electrodes
- **Correct:** B
- **Explanation:** By definition, reduction occurs at the cathode and oxidation at the anode, in both galvanic and electrolytic cells.
- **Tags:** `KC::GenChem::Electrochemistry` `Prereq::GenChem::Redox_Reactions` `Prereq::GenChem::Thermodynamics` `Prereq::Physics::Electrical_Circuits` `Prereq::Physics::Electrostatics` `MCAT::Chem_Phys` `Difficulty::2`

### MCAT-GCH-ELE-003

- **KC:** `GenChem::Electrochemistry`
- **Prereqs:** `Prereq::GenChem::Redox_Reactions` `Prereq::GenChem::Thermodynamics` `Prereq::Physics::Electrical_Circuits` `Prereq::Physics::Electrostatics`
- **Difficulty:** 4
- **Question:** A galvanic cell has a positive standard cell potential (E cell greater than 0). The redox reaction is therefore:
- **A:** nonspontaneous
- **B:** spontaneous
- **C:** at equilibrium
- **D:** impossible
- **Correct:** B
- **Explanation:** A positive cell potential corresponds to a negative delta G, so the reaction proceeds spontaneously as written.
- **Tags:** `KC::GenChem::Electrochemistry` `Prereq::GenChem::Redox_Reactions` `Prereq::GenChem::Thermodynamics` `Prereq::Physics::Electrical_Circuits` `Prereq::Physics::Electrostatics` `MCAT::Chem_Phys` `Difficulty::4`

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

### MCAT-GCH-NUCC-002

- **KC:** `GenChem::Nuclear_Chemistry`
- **Prereqs:** `Prereq::GenChem::Atomic_Structure` `Prereq::Physics::Atomic_Structure` `Prereq::Physics::Nuclear_Physics`
- **Difficulty:** 3
- **Question:** During beta-minus decay, a neutron is converted into a proton. As a result, the atomic number:
- **A:** increases by 1 while the mass number stays the same
- **B:** decreases by 1 while the mass number stays the same
- **C:** increases by 2 and the mass number decreases by 4
- **D:** stays the same while the mass number decreases by 1
- **Correct:** A
- **Explanation:** Beta-minus decay converts a neutron into a proton and emits an electron, so the atomic number rises by 1 while the mass number is unchanged.
- **Tags:** `KC::GenChem::Nuclear_Chemistry` `Prereq::GenChem::Atomic_Structure` `Prereq::Physics::Atomic_Structure` `Prereq::Physics::Nuclear_Physics` `MCAT::Chem_Phys` `Difficulty::3`

### MCAT-GCH-NUCC-003

- **KC:** `GenChem::Nuclear_Chemistry`
- **Prereqs:** `Prereq::GenChem::Atomic_Structure` `Prereq::Physics::Atomic_Structure` `Prereq::Physics::Nuclear_Physics`
- **Difficulty:** 4
- **Question:** A radioactive isotope has a half-life of 8 days. What fraction of the original sample remains after 24 days?
- **A:** 1/2
- **B:** 1/4
- **C:** 1/8
- **D:** 1/16
- **Correct:** C
- **Explanation:** 24 days equals three half-lives, so the remaining fraction is (1/2) to the third power, which is 1/8.
- **Tags:** `KC::GenChem::Nuclear_Chemistry` `Prereq::GenChem::Atomic_Structure` `Prereq::Physics::Atomic_Structure` `Prereq::Physics::Nuclear_Physics` `MCAT::Chem_Phys` `Difficulty::4`
