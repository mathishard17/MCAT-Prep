# MCAT Chem/Phys Practice Passages

**All passages below are ORIGINAL prose** written for MCAT-style "Chemical and Physical
Foundations of Biological Systems" practice. Each science passage is an original paraphrase
of the study design and findings of a **real, open-access (CC BY)** article; **no text is
reproduced verbatim**. Every passage carries a full **Source** citation (authors, title,
journal, year, license, DOI). Numerical results are drawn from the cited papers; some
values are rounded and some experimental detail is simplified for a passage-length
treatment.

Each question lists the **Correct** answer and an **Explanation** covering why the key is
right and the distractors wrong. Questions are answerable from the passage plus
foundational MCAT chemistry/physics.

Contents: **3 passages, 16 questions total.**

- `SCI-CP-P01` — general chemistry: reaction kinetics & photocatalysis — 6 Q
- `SCI-CP-P02` — physics: work, energy, and elastic energy in running — 5 Q
- `SCI-CP-P03` — physics/thermodynamics: thermoelectric energy conversion — 5 Q

---

## SCI-CP-P01
- **Discipline:** Chem/Phys
- **Source:** Original synthetic passage based on Nugroho D, Wannakan K, Nanan S, Benchawattananon R (2024), "Hydrothermal synthesis of Zingiber/ZnO for enhanced photodegradation of ofloxacin antibiotic and reactive red azo dye (RR141)," *PLOS ONE* 19(5):e0300402, CC BY, https://doi.org/10.1371/journal.pone.0300402. No text reproduced verbatim; values rounded.
- **Passage:** Semiconductor photocatalysts can break down organic pollutants in water using light. When a photon of sufficient energy strikes a semiconductor such as zinc oxide (ZnO), it promotes an electron from the valence band to the conduction band, leaving behind a positively charged "hole." If the electron and hole reach the surface before recombining, they drive reactions that generate highly reactive species—hydroxyl radicals and superoxide—that attack and fragment nearby organic molecules. A central limitation is that the electron and hole often simply recombine, wasting the absorbed energy as heat.

  In one study, researchers combined a plant extract with ZnO by a hydrothermal method to build a composite photocatalyst containing a *heterojunction*—an interface between two materials that helps separate the electron and hole, slowing their recombination and leaving more charge carriers available to react. They tested the material against two pollutants: a red azo dye (RR141) and the antibiotic ofloxacin. To follow each reaction, they periodically measured the solution's light absorbance; because absorbance is proportional to the concentration of the absorbing compound, the decline in absorbance over time tracked the disappearance of the pollutant.

  The degradation followed pseudo–first-order kinetics, meaning a plot of ln(C₀/C) against time gave a straight line, where C₀ is the initial concentration and C the concentration at time t. The slope of that line is the rate constant, k. For the dye RR141 the rate constant was about 0.02 per minute, roughly twice that for ofloxacin at about 0.01 per minute, indicating that the dye was broken down more quickly under the same conditions. Under ultraviolet illumination the catalyst removed about 97 percent of the dye and about 99 percent of the antibiotic; under natural sunlight it removed about 97 percent of the dye and about 95 percent of the antibiotic. The catalyst retained its activity across five successive cycles of use—an important practical feature, since a catalyst is by definition not consumed by the reaction it accelerates.

  The authors attributed the composite's strong performance to more efficient charge separation at the heterojunction. When recombination is suppressed, more electrons and holes reach the surface to produce reactive radicals, so more pollutant is degraded per unit time. Consistent with the general behavior of such systems, the researchers also noted that raising the initial concentration of pollutant tends to lower the *fraction* removed, because a limited number of surface sites and reactive radicals must act on a larger quantity of molecules, and because deeply colored solutions can block light from reaching the catalyst.

  For a first-order process, one useful property is that the time required to degrade half of the remaining pollutant does not depend on how much is present: the half-life equals ln 2 divided by k. A larger rate constant therefore corresponds to a shorter half-life and faster cleanup—one reason the rate constant, rather than the percentage removed in a single fixed interval, is often the more transferable measure of catalytic performance.

### SCI-CP-P01-Q1
- **Stem:** A plot of ln(C₀/C) versus time is linear for both pollutants. This observation indicates that the degradation reactions are:
- **A:** zero-order in the pollutant
- **B:** first-order in the pollutant, with the slope equal to the rate constant k
- **C:** second-order in the pollutant
- **D:** independent of time
- **Correct:** B
- **Explanation:** A linear ln(C₀/C)-versus-time plot is the signature of first-order kinetics, and its slope is k. (A) zero-order gives a linear plot of concentration (C) versus time, not the ln form. (C) second-order gives a linear 1/C-versus-time plot. (D) is nonsensical—the concentration clearly changes with time.

### SCI-CP-P01-Q2
- **Stem:** Based on the reported rate constants, how does the degradation of RR141 (k ≈ 0.02 min⁻¹) compare with that of ofloxacin (k ≈ 0.01 min⁻¹)?
- **A:** RR141 degrades about twice as fast, and has about half the half-life
- **B:** RR141 degrades about half as fast, and has twice the half-life
- **C:** They degrade at identical rates because both are first-order
- **D:** Ofloxacin has the shorter half-life
- **Correct:** A
- **Explanation:** A larger k means a faster reaction; since t₁/₂ = ln 2 / k, doubling k halves the half-life. (B) reverses the relationship. (C) confuses reaction *order* with reaction *rate*—same order, different k. (D) is backwards, since ofloxacin has the smaller k and thus the longer half-life.

### SCI-CP-P01-Q3
- **Stem:** The heterojunction improves photocatalytic performance primarily by:
- **A:** increasing the energy of the incoming photons
- **B:** promoting electron–hole recombination
- **C:** improving separation of the electron and hole so more charge carriers reach the surface to form reactive radicals
- **D:** absorbing the reactive radicals before they can react
- **Correct:** C
- **Explanation:** The passage says the heterojunction "helps separate the electron and hole, slowing their recombination," leaving more carriers to generate radicals. (A) is impossible—an interface does not change photon energy. (B) is the opposite of the benefit. (D) would reduce, not enhance, degradation.

### SCI-CP-P01-Q4
- **Stem:** The researchers tracked pollutant concentration by measuring the solution's light absorbance. This approach relies on the principle that:
- **A:** absorbance is proportional to concentration (Beer–Lambert behavior)
- **B:** absorbance is independent of concentration
- **C:** absorbance increases as the pollutant is degraded
- **D:** only the catalyst, not the pollutant, absorbs light
- **Correct:** A
- **Explanation:** The passage states "absorbance is proportional to the concentration of the absorbing compound," the Beer–Lambert relationship. (B) contradicts that proportionality. (C) is reversed—absorbance *decreases* as the pollutant disappears. (D) is wrong; it is the pollutant's absorbance that is monitored.

### SCI-CP-P01-Q5
- **Stem:** For a first-order reaction with rate constant k ≈ 0.02 min⁻¹, the half-life is approximately (use ln 2 ≈ 0.69):
- **A:** about 0.014 minutes
- **B:** about 3.5 minutes
- **C:** about 35 minutes
- **D:** about 100 minutes
- **Correct:** C
- **Explanation:** t₁/₂ = ln 2 / k = 0.69 / 0.02 ≈ 34.5 minutes. (A) multiplies instead of divides. (B) is off by an order of magnitude. (D) does not follow from the formula. This also reinforces that first-order half-life depends only on k, not on starting concentration.

### SCI-CP-P01-Q6
- **Stem:** According to the passage, why does increasing the initial pollutant concentration tend to *decrease* the fraction (percentage) of pollutant removed?
- **A:** Higher concentration raises the number of catalyst surface sites
- **B:** A limited number of active sites and radicals must act on more molecules, and darker solutions block light
- **C:** The reaction switches to zero-order and stops entirely
- **D:** The catalyst is consumed more rapidly at high concentration
- **Correct:** B
- **Explanation:** The passage attributes the drop to "a limited number of surface sites and reactive radicals" spread over more molecules and to light-blocking by deeply colored solutions. (A) is wrong—the number of sites is fixed by the catalyst, not the pollutant. (C) overstates and misapplies reaction order. (D) contradicts the definition of a catalyst as not consumed (and its five-cycle stability).

---

## SCI-CP-P02
- **Discipline:** Chem/Phys
- **Source:** Original synthetic passage based on Wunsch T, Alexander N, Kröll J, Stöggl T, Schwameder H (2017), "Effects of a leaf spring structured midsole on joint mechanics and lower limb muscle forces in running," *PLOS ONE* 12(2):e0172287, CC BY, https://doi.org/10.1371/journal.pone.0172287. No text reproduced verbatim; detail simplified.
- **Passage:** During running, the leg does not behave like a rigid strut. On each step the body's center of mass falls and the leg joints flex, absorbing mechanical energy; then the leg extends and returns energy to propel the body upward and forward. In the language of mechanics, a joint absorbs energy when the muscle–tendon forces around it do negative work (acting opposite to the motion, as in a controlled lowering) and generates energy when they do positive work (acting in the direction of motion). A well-designed shoe might store some of the energy that would otherwise be dissipated and return it later, much as a compressed spring stores elastic potential energy and releases it as it rebounds.

  To test whether a "leaf spring" midsole could do this, researchers compared such a shoe (LEAF) with a conventional foam midsole shoe (FOAM). Nine long-distance runners who strike the ground heel-first ran overground at a controlled speed of about 3.0 meters per second in each shoe. Motion-capture and force-plate measurements were fed into a musculoskeletal model that estimated the positive and negative work done at the hip, knee, and ankle and the forces produced by individual lower-limb muscles. Holding running speed constant across conditions was essential: because kinetic energy depends on speed, comparing the shoes at different speeds would confound the effect of the footwear with the effect of running faster or slower.

  Relative to the foam shoe, the leaf-spring shoe significantly reduced the energy absorbed at the hip joint and the energy generated at the ankle joint. It also significantly lowered the forces in the major plantar-flexor muscles of the calf—the soleus and both heads of the gastrocnemius—which normally act across the ankle during push-off. Several other muscles showed no significant change between the two shoes. The authors interpreted the pattern as evidence that the elastic structure of the midsole took over part of the mechanical work the ankle musculature would otherwise perform: if the shoe stores energy as it deforms under load and returns that energy during push-off, the calf muscles need to generate less force to achieve the same propulsion.

  Reducing muscle force for a given running task is potentially beneficial because muscular force production is metabolically expensive; a shoe that lowers the required force could, in principle, reduce the energy cost of running or delay fatigue. The study did not by itself demonstrate a change in whole-body energy expenditure, however—it measured joint mechanics and modeled muscle forces, not oxygen consumption. The results establish a plausible mechanism (elastic energy return reducing plantar-flexor demand) while leaving the performance payoff to be confirmed by direct metabolic measurement. This distinction—between demonstrating a mechanical mechanism and demonstrating a functional benefit—is an important limitation to keep in mind when interpreting biomechanical studies.

### SCI-CP-P02-Q1
- **Stem:** In the passage's usage, a joint "absorbs energy" when the surrounding muscle–tendon forces do:
- **A:** positive work, acting in the direction of motion
- **B:** negative work, acting opposite to the direction of motion
- **C:** zero net work over the stride
- **D:** work only against gravity
- **Correct:** B
- **Explanation:** The passage defines energy absorption as forces doing "negative work (acting opposite to the motion, as in a controlled lowering)." (A) describes energy generation (positive work). (C) contradicts absorption, which requires nonzero negative work. (D) is too narrow and not how the passage defines the term.

### SCI-CP-P02-Q2
- **Stem:** Why was it important that participants ran at the same controlled speed (~3.0 m/s) in both shoe conditions?
- **A:** Because muscle forces cannot be measured at other speeds
- **B:** To control a variable (speed) that affects kinetic energy, so differences can be attributed to the shoe rather than to pace
- **C:** Because kinetic energy does not depend on speed
- **D:** To ensure the runners reached maximum oxygen consumption
- **Correct:** B
- **Explanation:** Speed is a confounder because kinetic energy depends on it; holding it constant isolates the footwear effect. (A) is false—forces can be measured at various speeds. (C) is physically wrong (KE = ½mv²). (D) is unrelated; the test was submaximal and metabolic rate was not measured.

### SCI-CP-P02-Q3
- **Stem:** The proposed mechanism for how the leaf-spring midsole reduces calf-muscle force is most analogous to:
- **A:** a rigid steel rod that transmits force without deforming
- **B:** a compressed spring that stores elastic potential energy and returns it during push-off
- **C:** a damper that converts all input energy to heat
- **D:** a battery that stores chemical energy
- **Correct:** B
- **Explanation:** The passage explicitly likens the midsole to "a compressed spring [that] stores elastic potential energy and releases it as it rebounds," returning energy at push-off. (A) stores no energy. (C) dissipates energy rather than returning it (the opposite of the intended benefit). (D) involves chemical, not elastic mechanical, storage.

### SCI-CP-P02-Q4
- **Stem:** The finding that the LEAF shoe reduced *ankle* energy generation and *plantar-flexor* (soleus/gastrocnemius) forces most directly supports which conclusion?
- **A:** The runners were propelling themselves entirely with their hip muscles
- **B:** The shoe's elastic return performed part of the propulsive work normally done by the ankle musculature
- **C:** The shoe increased the total mechanical work required to run
- **D:** The gastrocnemius does not act across the ankle
- **Correct:** B
- **Explanation:** Lower ankle energy generation and reduced calf force are consistent with the elastic midsole supplying part of the push-off work, the passage's stated interpretation. (A) overstates and ignores the ankle's continued role. (C) contradicts the energy-saving mechanism. (D) is factually wrong—the plantar flexors act across the ankle.

### SCI-CP-P02-Q5
- **Stem:** Which statement best describes a key limitation the passage identifies?
- **A:** The study directly measured oxygen consumption and found no change
- **B:** The study demonstrated a mechanical mechanism but did not measure whole-body energy expenditure, so a metabolic benefit remains unconfirmed
- **C:** The study proved the leaf-spring shoe reduces running energy cost
- **D:** The study used different running speeds, invalidating all comparisons
- **Correct:** B
- **Explanation:** The passage states it "measured joint mechanics and modeled muscle forces, not oxygen consumption," so the metabolic payoff is unconfirmed. (A) is wrong—O₂ was not measured. (C) overclaims what the data show. (D) is false; speed was deliberately held constant.

---

## SCI-CP-P03
- **Discipline:** Chem/Phys
- **Source:** Original synthetic passage based on Rodríguez-Barreiro R, Abendroth C, Vilanova C, Moya A, Porcar M (2013), "Towards a Microbial Thermoelectric Cell," *PLOS ONE* 8(2):e56358, CC BY, https://doi.org/10.1371/journal.pone.0056358. No text reproduced verbatim; values rounded.
- **Passage:** Growing microbes release heat. In industrial fermentations this metabolic heat is usually treated as waste, but it represents chemical energy that living cells have converted into thermal energy. Researchers asked whether some of that heat could be turned into electricity using a thermoelectric generator (TEG), a solid-state device with no moving parts that produces a voltage when its two faces are held at different temperatures.

  The physical basis is the Seebeck effect: in a suitable material, a temperature difference across the material drives charge carriers from the hot side toward the cold side, establishing a voltage that is, to a first approximation, proportional to the temperature difference (ΔT) between the faces. A TEG therefore behaves like a heat engine, and like any heat engine it does not run on heat alone but on a *flow* of heat from a hot reservoir to a cold one; without a maintained temperature difference, no net electrical output is possible.

  The team built what they called a microbial thermoelectric cell: a thermally insulated reactor containing a culture of baker's yeast, with a small heat-exchange surface on which a TEG was mounted so that one face contacted the warm culture and the other face was exposed to cooler surroundings. Insulating the reactor served two purposes—it limited the escape of metabolic heat to the environment and helped the culture warm above ambient temperature, both of which support a usable temperature difference across the generator. With about 1.7 liters of culture, the internal temperature rose to roughly 41 degrees Celsius, and the device produced a net voltage of approximately 250 to 600 millivolts.

  Because the temperature difference available from a warm microbial culture is small compared with that in a furnace or a combustion engine, the generator had to be one optimized to work at low ΔT. This constraint reflects a fundamental feature of heat engines: the maximum fraction of heat that can be converted to work increases with the temperature difference between the hot and cold reservoirs, so a small ΔT imposes a low ceiling on efficiency. The authors framed the device not as a high-efficiency power source but as a way to recover a portion of energy that would otherwise be lost—potentially as a by-product of processes such as fermentation or aerobic waste digestion.

  The overall energy pathway is a chain of conversions: chemical energy stored in nutrients is released as heat by microbial metabolism; heat flows across the thermoelectric material because of the temperature difference; and a portion of that heat flow is converted into electrical energy. Each step is subject to the limits of thermodynamics, and losses at every stage mean that only a fraction of the original chemical energy emerges as electricity. Even so, the demonstration shows that the exothermic character of microbial growth—normally an engineering nuisance—can in principle be harnessed to generate a measurable electrical output.

### SCI-CP-P03-Q1
- **Stem:** According to the Seebeck effect as described, the voltage produced by the thermoelectric generator is, to a first approximation, proportional to:
- **A:** the absolute temperature of the cold side only
- **B:** the temperature difference (ΔT) between the two faces
- **C:** the total volume of the yeast culture
- **D:** the electrical resistance of the wires
- **Correct:** B
- **Explanation:** The passage states the voltage "is, to a first approximation, proportional to the temperature difference (ΔT) between the faces." (A) is wrong—one face's absolute temperature is insufficient; what matters is the difference. (C) affects total heat available but is not what the Seebeck voltage is proportional to. (D) is unrelated to the Seebeck voltage itself.

### SCI-CP-P03-Q2
- **Stem:** Insulating the reactor contributes to electrical output mainly because it:
- **A:** eliminates the need for a temperature difference
- **B:** retains metabolic heat and lets the culture warm above ambient, helping maintain a temperature difference across the TEG
- **C:** cools the hot side to match the surroundings
- **D:** increases the Seebeck coefficient of the material
- **Correct:** B
- **Explanation:** The passage says insulation "limited the escape of metabolic heat" and "helped the culture warm above ambient temperature," both supporting a usable ΔT. (A) is the opposite—ΔT is required, not eliminated. (C) would reduce ΔT and output. (D) confuses the device's material property with the reactor's thermal design.

### SCI-CP-P03-Q3
- **Stem:** Why did the researchers need a generator specifically optimized for *low* ΔT?
- **A:** Because a warm microbial culture provides only a small temperature difference, which limits achievable efficiency
- **B:** Because low ΔT produces the highest possible efficiency of any heat engine
- **C:** Because the Seebeck effect only works below room temperature
- **D:** Because yeast cannot survive large temperature differences across a device
- **Correct:** A
- **Explanation:** The culture provides a small ΔT, and efficiency rises with ΔT, so a device tuned for small differences is needed. (B) is backwards—low ΔT gives *low* efficiency. (C) is false; the effect is not restricted to sub-room temperatures. (D) misattributes the design constraint to yeast survival rather than to the physics of small ΔT.

### SCI-CP-P03-Q4
- **Stem:** Which sequence correctly represents the energy-conversion chain described in the passage?
- **A:** Electrical energy → chemical energy → thermal energy
- **B:** Chemical energy (nutrients) → thermal energy (metabolism) → electrical energy (TEG)
- **C:** Thermal energy → chemical energy → electrical energy
- **D:** Chemical energy → electrical energy directly, with no heat involved
- **Correct:** B
- **Explanation:** The passage lays out chemical → thermal → electrical: nutrients' chemical energy becomes heat via metabolism, which the TEG partly converts to electricity. (A) and (C) scramble the order. (D) omits the essential thermal intermediate that the TEG depends on.

### SCI-CP-P03-Q5
- **Stem:** A student proposes increasing the device's electrical output. Which change is most consistent with the physics described in the passage?
- **A:** Reducing the temperature difference between the two faces of the TEG
- **B:** Increasing the temperature difference, e.g., by warming the culture side or cooling the exposed side
- **C:** Removing the insulation so heat escapes more freely
- **D:** Using a smaller culture to lower the internal temperature
- **Correct:** B
- **Explanation:** Since Seebeck voltage scales with ΔT and heat-engine efficiency rises with ΔT, increasing the temperature difference should increase output. (A) reduces ΔT and output. (C) lets heat escape, shrinking ΔT. (D) lowers the hot-side temperature, again reducing ΔT.
