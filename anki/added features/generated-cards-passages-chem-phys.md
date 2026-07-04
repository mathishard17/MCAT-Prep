# MCAT Generated Cards — Chem/Phys Passages

Importable multiple-choice cards **converted from the reading passages in
`added features/passages-chem-phys.md`** so those (previously orphaned) passages
become scored content in the Chemical/Physical Foundations (Chem/Phys) section.
Every card keeps its original passage **Source** citation, correct answer, and
explanation; no new science is invented.

Block format matches the strict, line-based importer used by the other
`generated-cards-*.md` files (one field per line, `## <KC id>` group headers,
byte-exact `KC::` tags from `added features/kc-map-unified.md` §6). Each Question
line carries a trimmed (~120-word) version of the passage plus the question stem.

- **Source file:** `added features/passages-chem-phys.md` (3 passages, 16 questions).
- **Total cards:** 16 (`MCAT-PASSAGE-CP-001` … `MCAT-PASSAGE-CP-016`).
- **KC ids used (all real, from §6):** `GenChem::Kinetics`, `GenChem::Spectrophotometry`, `Physics::Work_And_Energy`, `Physics::Thermodynamics`.
- Primary section is `MCAT::Chem_Phys` for every card. Passage IRT/reasoning
  metadata (`IRT::Discrimination`, `IRT::Guessing`, `Reasoning::…`) is attached
  per the passage-scoring convention.

---

## GenChem::Kinetics

### MCAT-PASSAGE-CP-001

- **KC:** `GenChem::Kinetics`
- **Prereqs:** none
- **Difficulty:** 3
- **Question:** Passage (Source: Nugroho D, Wannakan K, Nanan S, Benchawattananon R (2024), "Hydrothermal synthesis of Zingiber/ZnO for enhanced photodegradation of ofloxacin antibiotic and reactive red azo dye (RR141)," PLOS ONE 19(5):e0300402, CC BY, https://doi.org/10.1371/journal.pone.0300402): Semiconductor photocatalysts such as zinc oxide (ZnO) use light to break down organic pollutants in water. Researchers built a plant-extract/ZnO composite photocatalyst and tested it against a red azo dye (RR141) and the antibiotic ofloxacin, following each reaction by periodically measuring the solution's light absorbance over time. The degradation followed pseudo–first-order kinetics, meaning a plot of ln(C₀/C) against time gave a straight line, where C₀ is the initial concentration and C the concentration at time t; the slope of that line is the rate constant, k. A larger rate constant corresponds to a shorter half-life and faster cleanup of the pollutant. — Question: A plot of ln(C₀/C) versus time is linear for both pollutants. This observation indicates that the degradation reactions are:
- **A:** zero-order in the pollutant
- **B:** first-order in the pollutant, with the slope equal to the rate constant k
- **C:** second-order in the pollutant
- **D:** independent of time
- **Correct:** B
- **Explanation:** A linear ln(C₀/C)-versus-time plot is the signature of first-order kinetics, and its slope is k. (A) zero-order gives a linear plot of concentration (C) versus time, not the ln form. (C) second-order gives a linear 1/C-versus-time plot. (D) is nonsensical—the concentration clearly changes with time.
- **Tags:** `KC::GenChem::Kinetics` `MCAT::Chem_Phys` `Difficulty::3` `IRT::Discrimination::0.9` `IRT::Guessing::0.25` `Reasoning::Comprehension`

### MCAT-PASSAGE-CP-002

- **KC:** `GenChem::Kinetics`
- **Prereqs:** none
- **Difficulty:** 3
- **Question:** Passage (Source: Nugroho D, Wannakan K, Nanan S, Benchawattananon R (2024), "Hydrothermal synthesis of Zingiber/ZnO for enhanced photodegradation of ofloxacin antibiotic and reactive red azo dye (RR141)," PLOS ONE 19(5):e0300402, CC BY, https://doi.org/10.1371/journal.pone.0300402): In this photocatalysis study, degradation of both pollutants followed pseudo–first-order kinetics, so a plot of ln(C₀/C) versus time was linear with slope equal to the rate constant k. For the red azo dye RR141 the rate constant was about 0.02 per minute, roughly twice that for the antibiotic ofloxacin at about 0.01 per minute, indicating the dye was broken down more quickly under the same conditions. For a first-order process, the time required to degrade half of the remaining pollutant does not depend on how much is present: the half-life equals ln 2 divided by k, so a larger rate constant corresponds to a shorter half-life. — Question: Based on the reported rate constants, how does the degradation of RR141 (k ≈ 0.02 min⁻¹) compare with that of ofloxacin (k ≈ 0.01 min⁻¹)?
- **A:** RR141 degrades about twice as fast, and has about half the half-life
- **B:** RR141 degrades about half as fast, and has twice the half-life
- **C:** They degrade at identical rates because both are first-order
- **D:** Ofloxacin has the shorter half-life
- **Correct:** A
- **Explanation:** A larger k means a faster reaction; since t₁/₂ = ln 2 / k, doubling k halves the half-life. (B) reverses the relationship. (C) confuses reaction order with reaction rate—same order, different k. (D) is backwards, since ofloxacin has the smaller k and thus the longer half-life.
- **Tags:** `KC::GenChem::Kinetics` `MCAT::Chem_Phys` `Difficulty::3` `IRT::Discrimination::1.0` `IRT::Guessing::0.25` `Reasoning::Application`

### MCAT-PASSAGE-CP-003

- **KC:** `GenChem::Kinetics`
- **Prereqs:** none
- **Difficulty:** 3
- **Question:** Passage (Source: Nugroho D, Wannakan K, Nanan S, Benchawattananon R (2024), "Hydrothermal synthesis of Zingiber/ZnO for enhanced photodegradation of ofloxacin antibiotic and reactive red azo dye (RR141)," PLOS ONE 19(5):e0300402, CC BY, https://doi.org/10.1371/journal.pone.0300402): When a photon of sufficient energy strikes a semiconductor such as ZnO, it promotes an electron from the valence band to the conduction band, leaving behind a positively charged hole. If the electron and hole reach the surface before recombining, they drive reactions that generate highly reactive species—hydroxyl radicals and superoxide—that fragment nearby organic molecules; a central limitation is that the electron and hole often simply recombine, wasting the energy. To combat this, researchers built a composite containing a heterojunction—an interface between two materials that helps separate the electron and hole, slowing recombination and leaving more charge carriers available to react, so more pollutant is degraded per unit time. — Question: The heterojunction improves photocatalytic performance primarily by:
- **A:** increasing the energy of the incoming photons
- **B:** promoting electron–hole recombination
- **C:** improving separation of the electron and hole so more charge carriers reach the surface to form reactive radicals
- **D:** absorbing the reactive radicals before they can react
- **Correct:** C
- **Explanation:** The passage says the heterojunction "helps separate the electron and hole, slowing their recombination," leaving more carriers to generate radicals. (A) is impossible—an interface does not change photon energy. (B) is the opposite of the benefit. (D) would reduce, not enhance, degradation.
- **Tags:** `KC::GenChem::Kinetics` `MCAT::Chem_Phys` `Difficulty::3` `IRT::Discrimination::0.9` `IRT::Guessing::0.25` `Reasoning::Comprehension`

### MCAT-PASSAGE-CP-004

- **KC:** `GenChem::Kinetics`
- **Prereqs:** none
- **Difficulty:** 3
- **Question:** Passage (Source: Nugroho D, Wannakan K, Nanan S, Benchawattananon R (2024), "Hydrothermal synthesis of Zingiber/ZnO for enhanced photodegradation of ofloxacin antibiotic and reactive red azo dye (RR141)," PLOS ONE 19(5):e0300402, CC BY, https://doi.org/10.1371/journal.pone.0300402): The photocatalytic degradation of the pollutants followed pseudo–first-order kinetics, with the rate constant k obtained as the slope of a linear plot of ln(C₀/C) versus time. For the red azo dye RR141, k was about 0.02 per minute. A useful property of a first-order process is that the time required to degrade half of the remaining pollutant does not depend on how much is present: the half-life equals ln 2 divided by k (with ln 2 ≈ 0.69). A larger rate constant therefore corresponds to a shorter half-life and faster cleanup, which is why the rate constant is often a more transferable measure of catalytic performance than percentage removed. — Question: For a first-order reaction with rate constant k ≈ 0.02 min⁻¹, the half-life is approximately (use ln 2 ≈ 0.69):
- **A:** about 0.014 minutes
- **B:** about 3.5 minutes
- **C:** about 35 minutes
- **D:** about 100 minutes
- **Correct:** C
- **Explanation:** t₁/₂ = ln 2 / k = 0.69 / 0.02 ≈ 34.5 minutes. (A) multiplies instead of divides. (B) is off by an order of magnitude. (D) does not follow from the formula. This also reinforces that first-order half-life depends only on k, not on starting concentration.
- **Tags:** `KC::GenChem::Kinetics` `MCAT::Chem_Phys` `Difficulty::3` `IRT::Discrimination::1.0` `IRT::Guessing::0.25` `Reasoning::Application`

### MCAT-PASSAGE-CP-005

- **KC:** `GenChem::Kinetics`
- **Prereqs:** none
- **Difficulty:** 3
- **Question:** Passage (Source: Nugroho D, Wannakan K, Nanan S, Benchawattananon R (2024), "Hydrothermal synthesis of Zingiber/ZnO for enhanced photodegradation of ofloxacin antibiotic and reactive red azo dye (RR141)," PLOS ONE 19(5):e0300402, CC BY, https://doi.org/10.1371/journal.pone.0300402): The plant-extract/ZnO composite degraded a red azo dye and an antibiotic via reactive radicals produced at the catalyst surface, and it retained its activity across five successive cycles—consistent with a catalyst not being consumed by the reaction it accelerates. The authors noted that raising the initial concentration of pollutant tends to lower the fraction removed, because a limited number of surface sites and reactive radicals must act on a larger quantity of molecules, and because deeply colored solutions can block light from reaching the catalyst. Under ultraviolet light the catalyst removed about 97 percent of the dye and 99 percent of the antibiotic. — Question: According to the passage, why does increasing the initial pollutant concentration tend to decrease the fraction (percentage) of pollutant removed?
- **A:** Higher concentration raises the number of catalyst surface sites
- **B:** A limited number of active sites and radicals must act on more molecules, and darker solutions block light
- **C:** The reaction switches to zero-order and stops entirely
- **D:** The catalyst is consumed more rapidly at high concentration
- **Correct:** B
- **Explanation:** The passage attributes the drop to "a limited number of surface sites and reactive radicals" spread over more molecules and to light-blocking by deeply colored solutions. (A) is wrong—the number of sites is fixed by the catalyst, not the pollutant. (C) overstates and misapplies reaction order. (D) contradicts the definition of a catalyst as not consumed (and its five-cycle stability).
- **Tags:** `KC::GenChem::Kinetics` `MCAT::Chem_Phys` `Difficulty::3` `IRT::Discrimination::1.0` `IRT::Guessing::0.25` `Reasoning::Comprehension`

---

## GenChem::Spectrophotometry

### MCAT-PASSAGE-CP-006

- **KC:** `GenChem::Spectrophotometry`
- **Prereqs:** none
- **Difficulty:** 2
- **Question:** Passage (Source: Nugroho D, Wannakan K, Nanan S, Benchawattananon R (2024), "Hydrothermal synthesis of Zingiber/ZnO for enhanced photodegradation of ofloxacin antibiotic and reactive red azo dye (RR141)," PLOS ONE 19(5):e0300402, CC BY, https://doi.org/10.1371/journal.pone.0300402): To follow each photocatalytic reaction, researchers periodically measured the solution's light absorbance; because absorbance is proportional to the concentration of the absorbing compound, the decline in absorbance over time tracked the disappearance of the pollutant. They tested the ZnO composite against a red azo dye (RR141) and the antibiotic ofloxacin, and the degradation followed pseudo–first-order kinetics. As the pollutant was consumed, its concentration fell and the measured absorbance decreased accordingly, providing the concentration-versus-time data used to extract the rate constant. — Question: The researchers tracked pollutant concentration by measuring the solution's light absorbance. This approach relies on the principle that:
- **A:** absorbance is proportional to concentration (Beer–Lambert behavior)
- **B:** absorbance is independent of concentration
- **C:** absorbance increases as the pollutant is degraded
- **D:** only the catalyst, not the pollutant, absorbs light
- **Correct:** A
- **Explanation:** The passage states "absorbance is proportional to the concentration of the absorbing compound," the Beer–Lambert relationship. (B) contradicts that proportionality. (C) is reversed—absorbance decreases as the pollutant disappears. (D) is wrong; it is the pollutant's absorbance that is monitored.
- **Tags:** `KC::GenChem::Spectrophotometry` `MCAT::Chem_Phys` `Difficulty::2` `IRT::Discrimination::0.8` `IRT::Guessing::0.25` `Reasoning::Comprehension`

---

## Physics::Work_And_Energy

### MCAT-PASSAGE-CP-007

- **KC:** `Physics::Work_And_Energy`
- **Prereqs:** none
- **Difficulty:** 2
- **Question:** Passage (Source: Wunsch T, Alexander N, Kröll J, Stöggl T, Schwameder H (2017), "Effects of a leaf spring structured midsole on joint mechanics and lower limb muscle forces in running," PLOS ONE 12(2):e0172287, CC BY, https://doi.org/10.1371/journal.pone.0172287): During running, the leg does not behave like a rigid strut. On each step the body's center of mass falls and the leg joints flex, absorbing mechanical energy; then the leg extends and returns energy to propel the body upward and forward. In the language of mechanics, a joint absorbs energy when the muscle–tendon forces around it do negative work (acting opposite to the motion, as in a controlled lowering) and generates energy when they do positive work (acting in the direction of motion). A well-designed shoe might store energy that would otherwise be dissipated and return it later, like a compressed spring. — Question: In the passage's usage, a joint "absorbs energy" when the surrounding muscle–tendon forces do:
- **A:** positive work, acting in the direction of motion
- **B:** negative work, acting opposite to the direction of motion
- **C:** zero net work over the stride
- **D:** work only against gravity
- **Correct:** B
- **Explanation:** The passage defines energy absorption as forces doing "negative work (acting opposite to the motion, as in a controlled lowering)." (A) describes energy generation (positive work). (C) contradicts absorption, which requires nonzero negative work. (D) is too narrow and not how the passage defines the term.
- **Tags:** `KC::Physics::Work_And_Energy` `MCAT::Chem_Phys` `Difficulty::2` `IRT::Discrimination::0.8` `IRT::Guessing::0.25` `Reasoning::Comprehension`

### MCAT-PASSAGE-CP-008

- **KC:** `Physics::Work_And_Energy`
- **Prereqs:** none
- **Difficulty:** 3
- **Question:** Passage (Source: Wunsch T, Alexander N, Kröll J, Stöggl T, Schwameder H (2017), "Effects of a leaf spring structured midsole on joint mechanics and lower limb muscle forces in running," PLOS ONE 12(2):e0172287, CC BY, https://doi.org/10.1371/journal.pone.0172287): To test whether a leaf-spring midsole stores and returns energy, researchers compared such a shoe (LEAF) with a conventional foam midsole shoe (FOAM). Nine heel-striking long-distance runners ran overground at a controlled speed of about 3.0 meters per second in each shoe, while motion-capture and force-plate data fed a musculoskeletal model that estimated joint work and individual muscle forces. Holding running speed constant across conditions was essential: because kinetic energy depends on speed (KE = ½mv²), comparing the shoes at different speeds would confound the effect of the footwear with the effect of running faster or slower. — Question: Why was it important that participants ran at the same controlled speed (~3.0 m/s) in both shoe conditions?
- **A:** Because muscle forces cannot be measured at other speeds
- **B:** To control a variable (speed) that affects kinetic energy, so differences can be attributed to the shoe rather than to pace
- **C:** Because kinetic energy does not depend on speed
- **D:** To ensure the runners reached maximum oxygen consumption
- **Correct:** B
- **Explanation:** Speed is a confounder because kinetic energy depends on it; holding it constant isolates the footwear effect. (A) is false—forces can be measured at various speeds. (C) is physically wrong (KE = ½mv²). (D) is unrelated; the test was submaximal and metabolic rate was not measured.
- **Tags:** `KC::Physics::Work_And_Energy` `MCAT::Chem_Phys` `Difficulty::3` `IRT::Discrimination::1.0` `IRT::Guessing::0.25` `Reasoning::Application`

### MCAT-PASSAGE-CP-009

- **KC:** `Physics::Work_And_Energy`
- **Prereqs:** none
- **Difficulty:** 2
- **Question:** Passage (Source: Wunsch T, Alexander N, Kröll J, Stöggl T, Schwameder H (2017), "Effects of a leaf spring structured midsole on joint mechanics and lower limb muscle forces in running," PLOS ONE 12(2):e0172287, CC BY, https://doi.org/10.1371/journal.pone.0172287): During running the leg joints alternately absorb and generate mechanical energy. A well-designed shoe might store some of the energy that would otherwise be dissipated and return it later, much as a compressed spring stores elastic potential energy and releases it as it rebounds. Researchers tested a leaf-spring midsole shoe against a foam shoe; they proposed that if the shoe stores energy as it deforms under load and returns that energy during push-off, the calf muscles need to generate less force to achieve the same propulsion. A rigid strut, by contrast, stores no energy, and a damper would dissipate energy as heat rather than returning it. — Question: The proposed mechanism for how the leaf-spring midsole reduces calf-muscle force is most analogous to:
- **A:** a rigid steel rod that transmits force without deforming
- **B:** a compressed spring that stores elastic potential energy and returns it during push-off
- **C:** a damper that converts all input energy to heat
- **D:** a battery that stores chemical energy
- **Correct:** B
- **Explanation:** The passage explicitly likens the midsole to "a compressed spring [that] stores elastic potential energy and releases it as it rebounds," returning energy at push-off. (A) stores no energy. (C) dissipates energy rather than returning it (the opposite of the intended benefit). (D) involves chemical, not elastic mechanical, storage.
- **Tags:** `KC::Physics::Work_And_Energy` `MCAT::Chem_Phys` `Difficulty::2` `IRT::Discrimination::0.8` `IRT::Guessing::0.25` `Reasoning::Comprehension`

### MCAT-PASSAGE-CP-010

- **KC:** `Physics::Work_And_Energy`
- **Prereqs:** none
- **Difficulty:** 3
- **Question:** Passage (Source: Wunsch T, Alexander N, Kröll J, Stöggl T, Schwameder H (2017), "Effects of a leaf spring structured midsole on joint mechanics and lower limb muscle forces in running," PLOS ONE 12(2):e0172287, CC BY, https://doi.org/10.1371/journal.pone.0172287): Relative to the foam shoe, the leaf-spring shoe significantly reduced the energy absorbed at the hip joint and the energy generated at the ankle joint, and it lowered the forces in the major plantar-flexor muscles of the calf—the soleus and both heads of the gastrocnemius—which normally act across the ankle during push-off. The authors interpreted this as evidence that the elastic structure of the midsole took over part of the mechanical work the ankle musculature would otherwise perform: if the shoe stores energy as it deforms and returns it during push-off, the calf muscles need to generate less force to achieve the same propulsion. — Question: The finding that the LEAF shoe reduced ankle energy generation and plantar-flexor (soleus/gastrocnemius) forces most directly supports which conclusion?
- **A:** The runners were propelling themselves entirely with their hip muscles
- **B:** The shoe's elastic return performed part of the propulsive work normally done by the ankle musculature
- **C:** The shoe increased the total mechanical work required to run
- **D:** The gastrocnemius does not act across the ankle
- **Correct:** B
- **Explanation:** Lower ankle energy generation and reduced calf force are consistent with the elastic midsole supplying part of the push-off work, the passage's stated interpretation. (A) overstates and ignores the ankle's continued role. (C) contradicts the energy-saving mechanism. (D) is factually wrong—the plantar flexors act across the ankle.
- **Tags:** `KC::Physics::Work_And_Energy` `MCAT::Chem_Phys` `Difficulty::3` `IRT::Discrimination::1.1` `IRT::Guessing::0.25` `Reasoning::Inference`

### MCAT-PASSAGE-CP-011

- **KC:** `Physics::Work_And_Energy`
- **Prereqs:** none
- **Difficulty:** 3
- **Question:** Passage (Source: Wunsch T, Alexander N, Kröll J, Stöggl T, Schwameder H (2017), "Effects of a leaf spring structured midsole on joint mechanics and lower limb muscle forces in running," PLOS ONE 12(2):e0172287, CC BY, https://doi.org/10.1371/journal.pone.0172287): The leaf-spring shoe reduced modeled plantar-flexor muscle forces, which is potentially beneficial because muscular force production is metabolically expensive; a shoe that lowers the required force could in principle reduce the energy cost of running or delay fatigue. The study did not by itself demonstrate a change in whole-body energy expenditure, however—it measured joint mechanics and modeled muscle forces, not oxygen consumption. The results establish a plausible mechanism (elastic energy return reducing plantar-flexor demand) while leaving the performance payoff to be confirmed by direct metabolic measurement, a key distinction between demonstrating a mechanism and demonstrating a functional benefit. — Question: Which statement best describes a key limitation the passage identifies?
- **A:** The study directly measured oxygen consumption and found no change
- **B:** The study demonstrated a mechanical mechanism but did not measure whole-body energy expenditure, so a metabolic benefit remains unconfirmed
- **C:** The study proved the leaf-spring shoe reduces running energy cost
- **D:** The study used different running speeds, invalidating all comparisons
- **Correct:** B
- **Explanation:** The passage states it "measured joint mechanics and modeled muscle forces, not oxygen consumption," so the metabolic payoff is unconfirmed. (A) is wrong—O₂ was not measured. (C) overclaims what the data show. (D) is false; speed was deliberately held constant.
- **Tags:** `KC::Physics::Work_And_Energy` `MCAT::Chem_Phys` `Difficulty::3` `IRT::Discrimination::1.1` `IRT::Guessing::0.25` `Reasoning::Inference`

---

## Physics::Thermodynamics

### MCAT-PASSAGE-CP-012

- **KC:** `Physics::Thermodynamics`
- **Prereqs:** none
- **Difficulty:** 2
- **Question:** Passage (Source: Rodríguez-Barreiro R, Abendroth C, Vilanova C, Moya A, Porcar M (2013), "Towards a Microbial Thermoelectric Cell," PLOS ONE 8(2):e56358, CC BY, https://doi.org/10.1371/journal.pone.0056358): Growing microbes release metabolic heat, usually treated as waste. Researchers asked whether some of that heat could be turned into electricity using a thermoelectric generator (TEG), a solid-state device that produces a voltage when its two faces are held at different temperatures. The physical basis is the Seebeck effect: in a suitable material, a temperature difference across the material drives charge carriers from the hot side toward the cold side, establishing a voltage that is, to a first approximation, proportional to the temperature difference (ΔT) between the faces. Like any heat engine, a TEG runs not on heat alone but on a flow of heat from a hot reservoir to a cold one. — Question: According to the Seebeck effect as described, the voltage produced by the thermoelectric generator is, to a first approximation, proportional to:
- **A:** the absolute temperature of the cold side only
- **B:** the temperature difference (ΔT) between the two faces
- **C:** the total volume of the yeast culture
- **D:** the electrical resistance of the wires
- **Correct:** B
- **Explanation:** The passage states the voltage "is, to a first approximation, proportional to the temperature difference (ΔT) between the faces." (A) is wrong—one face's absolute temperature is insufficient; what matters is the difference. (C) affects total heat available but is not what the Seebeck voltage is proportional to. (D) is unrelated to the Seebeck voltage itself.
- **Tags:** `KC::Physics::Thermodynamics` `MCAT::Chem_Phys` `Difficulty::2` `IRT::Discrimination::0.8` `IRT::Guessing::0.25` `Reasoning::Comprehension`

### MCAT-PASSAGE-CP-013

- **KC:** `Physics::Thermodynamics`
- **Prereqs:** none
- **Difficulty:** 3
- **Question:** Passage (Source: Rodríguez-Barreiro R, Abendroth C, Vilanova C, Moya A, Porcar M (2013), "Towards a Microbial Thermoelectric Cell," PLOS ONE 8(2):e56358, CC BY, https://doi.org/10.1371/journal.pone.0056358): The team built a microbial thermoelectric cell: a thermally insulated reactor containing baker's yeast, with a TEG mounted so one face contacted the warm culture and the other face was exposed to cooler surroundings. Insulating the reactor served two purposes—it limited the escape of metabolic heat to the environment and helped the culture warm above ambient temperature, both of which support a usable temperature difference across the generator. With about 1.7 liters of culture, the internal temperature rose to roughly 41 degrees Celsius, and the device produced a net voltage of approximately 250 to 600 millivolts. Without a maintained temperature difference, no net electrical output is possible. — Question: Insulating the reactor contributes to electrical output mainly because it:
- **A:** eliminates the need for a temperature difference
- **B:** retains metabolic heat and lets the culture warm above ambient, helping maintain a temperature difference across the TEG
- **C:** cools the hot side to match the surroundings
- **D:** increases the Seebeck coefficient of the material
- **Correct:** B
- **Explanation:** The passage says insulation "limited the escape of metabolic heat" and "helped the culture warm above ambient temperature," both supporting a usable ΔT. (A) is the opposite—ΔT is required, not eliminated. (C) would reduce ΔT and output. (D) confuses the device's material property with the reactor's thermal design.
- **Tags:** `KC::Physics::Thermodynamics` `MCAT::Chem_Phys` `Difficulty::3` `IRT::Discrimination::0.9` `IRT::Guessing::0.25` `Reasoning::Comprehension`

### MCAT-PASSAGE-CP-014

- **KC:** `Physics::Thermodynamics`
- **Prereqs:** none
- **Difficulty:** 4
- **Question:** Passage (Source: Rodríguez-Barreiro R, Abendroth C, Vilanova C, Moya A, Porcar M (2013), "Towards a Microbial Thermoelectric Cell," PLOS ONE 8(2):e56358, CC BY, https://doi.org/10.1371/journal.pone.0056358): Because the temperature difference available from a warm microbial culture is small compared with that in a furnace or combustion engine, the generator had to be one optimized to work at low ΔT. This constraint reflects a fundamental feature of heat engines: the maximum fraction of heat that can be converted to work increases with the temperature difference between the hot and cold reservoirs, so a small ΔT imposes a low ceiling on efficiency. The authors framed the device not as a high-efficiency power source but as a way to recover a portion of energy that would otherwise be lost, for example from fermentation or aerobic waste digestion. — Question: Why did the researchers need a generator specifically optimized for low ΔT?
- **A:** Because a warm microbial culture provides only a small temperature difference, which limits achievable efficiency
- **B:** Because low ΔT produces the highest possible efficiency of any heat engine
- **C:** Because the Seebeck effect only works below room temperature
- **D:** Because yeast cannot survive large temperature differences across a device
- **Correct:** A
- **Explanation:** The culture provides a small ΔT, and efficiency rises with ΔT, so a device tuned for small differences is needed. (B) is backwards—low ΔT gives low efficiency. (C) is false; the effect is not restricted to sub-room temperatures. (D) misattributes the design constraint to yeast survival rather than to the physics of small ΔT.
- **Tags:** `KC::Physics::Thermodynamics` `MCAT::Chem_Phys` `Difficulty::4` `IRT::Discrimination::1.2` `IRT::Guessing::0.25` `Reasoning::Inference`

### MCAT-PASSAGE-CP-015

- **KC:** `Physics::Thermodynamics`
- **Prereqs:** none
- **Difficulty:** 3
- **Question:** Passage (Source: Rodríguez-Barreiro R, Abendroth C, Vilanova C, Moya A, Porcar M (2013), "Towards a Microbial Thermoelectric Cell," PLOS ONE 8(2):e56358, CC BY, https://doi.org/10.1371/journal.pone.0056358): The overall energy pathway in the microbial thermoelectric cell is a chain of conversions: chemical energy stored in nutrients is released as heat by microbial metabolism; heat flows across the thermoelectric material because of the temperature difference between its faces; and a portion of that heat flow is converted into electrical energy by the Seebeck effect. Each step is subject to the limits of thermodynamics, and losses at every stage mean that only a fraction of the original chemical energy emerges as electricity. Even so, the exothermic character of microbial growth can in principle be harnessed to generate a measurable electrical output. — Question: Which sequence correctly represents the energy-conversion chain described in the passage?
- **A:** Electrical energy → chemical energy → thermal energy
- **B:** Chemical energy (nutrients) → thermal energy (metabolism) → electrical energy (TEG)
- **C:** Thermal energy → chemical energy → electrical energy
- **D:** Chemical energy → electrical energy directly, with no heat involved
- **Correct:** B
- **Explanation:** The passage lays out chemical → thermal → electrical: nutrients' chemical energy becomes heat via metabolism, which the TEG partly converts to electricity. (A) and (C) scramble the order. (D) omits the essential thermal intermediate that the TEG depends on.
- **Tags:** `KC::Physics::Thermodynamics` `MCAT::Chem_Phys` `Difficulty::3` `IRT::Discrimination::0.9` `IRT::Guessing::0.25` `Reasoning::Comprehension`

### MCAT-PASSAGE-CP-016

- **KC:** `Physics::Thermodynamics`
- **Prereqs:** none
- **Difficulty:** 3
- **Question:** Passage (Source: Rodríguez-Barreiro R, Abendroth C, Vilanova C, Moya A, Porcar M (2013), "Towards a Microbial Thermoelectric Cell," PLOS ONE 8(2):e56358, CC BY, https://doi.org/10.1371/journal.pone.0056358): In the microbial thermoelectric cell, the Seebeck voltage is proportional to the temperature difference (ΔT) across the generator's faces, and the maximum efficiency of any heat engine increases with the temperature difference between its hot and cold reservoirs. The device used an insulated reactor of baker's yeast to keep the culture warm (about 41 degrees Celsius) while the opposite face stayed cooler, producing roughly 250 to 600 millivolts. Because both the voltage and the attainable efficiency grow with ΔT, choices that shrink the temperature difference—removing insulation, cooling the hot side, or lowering the culture temperature—reduce the output. — Question: A student proposes increasing the device's electrical output. Which change is most consistent with the physics described in the passage?
- **A:** Reducing the temperature difference between the two faces of the TEG
- **B:** Increasing the temperature difference, e.g., by warming the culture side or cooling the exposed side
- **C:** Removing the insulation so heat escapes more freely
- **D:** Using a smaller culture to lower the internal temperature
- **Correct:** B
- **Explanation:** Since Seebeck voltage scales with ΔT and heat-engine efficiency rises with ΔT, increasing the temperature difference should increase output. (A) reduces ΔT and output. (C) lets heat escape, shrinking ΔT. (D) lowers the hot-side temperature, again reducing ΔT.
- **Tags:** `KC::Physics::Thermodynamics` `MCAT::Chem_Phys` `Difficulty::3` `IRT::Discrimination::1.0` `IRT::Guessing::0.25` `Reasoning::Application`
