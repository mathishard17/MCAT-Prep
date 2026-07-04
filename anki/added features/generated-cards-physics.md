# MCAT Physics Flashcards

Synthetic, original multiple-choice cards for the Physics (`Physics::`) Knowledge
Components of the MCAT Concept Scheduler. At least one card per Physics KC (26
KCs), in the importer block format used by `added features/mcat_demo_cards.md`.
All content is original/synthetic; no copyrighted prep material is reproduced.

## Tags

- Section tag: `MCAT::Chem_Phys`
- KC tag: `KC::Physics::<Component>`
- Prerequisite tag: `Prereq::Physics::<Component>` (or cross-discipline `Prereq::GenChem::<Component>`)
- Difficulty tag: `Difficulty::<1-5>`

---

## Physics::Units_And_Measurement

### MCAT-PHY-UNIM-001

- **KC:** `Physics::Units_And_Measurement`
- **Prereqs:** none
- **Difficulty:** 2
- **Question:** A displacement vector has a magnitude of 10 m directed 30 degrees above the horizontal. Approximately what is the magnitude of its horizontal component?
- **A:** 5.0 m
- **B:** 8.7 m
- **C:** 10 m
- **D:** 0 m
- **Correct:** B
- **Explanation:** The horizontal component equals the magnitude times the cosine of the angle, so 10 m times cos 30 degrees is about 8.7 m.
- **Tags:** `KC::Physics::Units_And_Measurement` `MCAT::Chem_Phys` `Difficulty::2`

## Physics::Kinematics

### MCAT-PHY-KIN-001

- **KC:** `Physics::Kinematics`
- **Prereqs:** `Prereq::Physics::Units_And_Measurement`
- **Difficulty:** 2
- **Question:** A ball is thrown straight up and later returns to its original launch height. Ignoring air resistance, how does its speed just before returning compare with its initial launch speed?
- **A:** Greater, because it accelerates the whole way down
- **B:** Equal, because the motion is symmetric
- **C:** Less, because energy is lost at the highest point
- **D:** Zero, because velocity is zero at the top
- **Correct:** B
- **Explanation:** Without air resistance the upward and downward paths are symmetric, so the ball returns to its launch height with the same speed it started with.
- **Tags:** `KC::Physics::Kinematics` `Prereq::Physics::Units_And_Measurement` `MCAT::Chem_Phys` `Difficulty::2`

## Physics::Newtons_Laws

### MCAT-PHY-NEWL-001

- **KC:** `Physics::Newtons_Laws`
- **Prereqs:** `Prereq::Physics::Kinematics`
- **Difficulty:** 3
- **Question:** A book rests on a table. According to Newton's third law, what is the reaction force paired with the gravitational force the Earth exerts on the book?
- **A:** The normal force the table exerts on the book
- **B:** The weight of the table pressing down
- **C:** The gravitational force the book exerts on the Earth
- **D:** The static friction force acting on the book
- **Correct:** C
- **Explanation:** Third-law pairs act on different objects, so the reaction to Earth pulling the book down is the book pulling the Earth up with equal magnitude.
- **Tags:** `KC::Physics::Newtons_Laws` `Prereq::Physics::Kinematics` `MCAT::Chem_Phys` `Difficulty::3`

## Physics::Force_Equilibrium

### MCAT-PHY-FORE-001

- **KC:** `Physics::Force_Equilibrium`
- **Prereqs:** `Prereq::Physics::Newtons_Laws`
- **Difficulty:** 3
- **Question:** A uniform beam balances on a central pivot. A 4 kg mass is placed 1 m to the left of the pivot. How far to the right of the pivot must a 2 kg mass be placed to keep the beam in rotational equilibrium?
- **A:** 0.5 m
- **B:** 1 m
- **C:** 2 m
- **D:** 4 m
- **Correct:** C
- **Explanation:** Rotational equilibrium requires equal and opposite torques, so (4 kg)(1 m) must equal (2 kg)(d), giving d equal to 2 m.
- **Tags:** `KC::Physics::Force_Equilibrium` `Prereq::Physics::Newtons_Laws` `MCAT::Chem_Phys` `Difficulty::3`

## Physics::Work_And_Energy

### MCAT-PHY-WORE-001

- **KC:** `Physics::Work_And_Energy`
- **Prereqs:** `Prereq::Physics::Newtons_Laws`
- **Difficulty:** 3
- **Question:** A crate is pushed at constant velocity across a rough, level floor for 5 m. What is the net work done on the crate over that displacement?
- **A:** Positive, equal to the applied force times the distance
- **B:** Zero, because the kinetic energy does not change
- **C:** Negative, equal to the friction force times the distance
- **D:** Positive, equal to the crate's weight times the distance
- **Correct:** B
- **Explanation:** By the work-energy theorem, net work equals the change in kinetic energy; at constant velocity kinetic energy is unchanged, so the net work is zero.
- **Tags:** `KC::Physics::Work_And_Energy` `Prereq::Physics::Newtons_Laws` `MCAT::Chem_Phys` `Difficulty::3`

## Physics::Momentum_And_Impulse

### MCAT-PHY-MOMI-001

- **KC:** `Physics::Momentum_And_Impulse`
- **Prereqs:** `Prereq::Physics::Newtons_Laws`
- **Difficulty:** 3
- **Question:** An airbag increases the time over which a passenger's momentum changes during a collision. Compared with a sudden stop, this longer stopping time does what to the average force on the passenger?
- **A:** Increases it
- **B:** Decreases it
- **C:** Leaves it unchanged
- **D:** Reduces the total impulse to zero
- **Correct:** B
- **Explanation:** Impulse equals the change in momentum and also equals average force times time, so for a fixed momentum change a longer time means a smaller average force.
- **Tags:** `KC::Physics::Momentum_And_Impulse` `Prereq::Physics::Newtons_Laws` `MCAT::Chem_Phys` `Difficulty::3`

## Physics::Rotational_Motion

### MCAT-PHY-ROTM-001

- **KC:** `Physics::Rotational_Motion`
- **Prereqs:** `Prereq::Physics::Force_Equilibrium` `Prereq::Physics::Work_And_Energy`
- **Difficulty:** 4
- **Question:** A spinning figure skater pulls both arms inward, reducing their moment of inertia. With negligible external torque, what happens to their angular speed?
- **A:** It decreases
- **B:** It stays the same
- **C:** It increases
- **D:** It drops to zero
- **Correct:** C
- **Explanation:** With no external torque, angular momentum (moment of inertia times angular speed) is conserved, so reducing the moment of inertia increases the angular speed.
- **Tags:** `KC::Physics::Rotational_Motion` `Prereq::Physics::Force_Equilibrium` `Prereq::Physics::Work_And_Energy` `MCAT::Chem_Phys` `Difficulty::4`

## Physics::Periodic_Motion

### MCAT-PHY-PERM-001

- **KC:** `Physics::Periodic_Motion`
- **Prereqs:** `Prereq::Physics::Force_Equilibrium` `Prereq::Physics::Work_And_Energy`
- **Difficulty:** 3
- **Question:** A mass oscillates on an ideal spring. If the attached mass is quadrupled while the spring constant stays the same, how does the period of oscillation change?
- **A:** It is halved
- **B:** It is unchanged
- **C:** It doubles
- **D:** It quadruples
- **Correct:** C
- **Explanation:** The period of a mass-spring system is proportional to the square root of the mass, so multiplying the mass by four doubles the period.
- **Tags:** `KC::Physics::Periodic_Motion` `Prereq::Physics::Force_Equilibrium` `Prereq::Physics::Work_And_Energy` `MCAT::Chem_Phys` `Difficulty::3`

## Physics::Waves

### MCAT-PHY-WAV-001

- **KC:** `Physics::Waves`
- **Prereqs:** `Prereq::Physics::Periodic_Motion`
- **Difficulty:** 3
- **Question:** A wave passes from one medium into another where its speed decreases. If the frequency stays constant, what happens to the wavelength?
- **A:** It increases
- **B:** It decreases
- **C:** It stays the same
- **D:** It becomes zero
- **Correct:** B
- **Explanation:** Wave speed equals frequency times wavelength, so with a fixed frequency a lower speed corresponds to a shorter wavelength.
- **Tags:** `KC::Physics::Waves` `Prereq::Physics::Periodic_Motion` `MCAT::Chem_Phys` `Difficulty::3`

## Physics::Sound

### MCAT-PHY-SOU-001

- **KC:** `Physics::Sound`
- **Prereqs:** `Prereq::Physics::Waves`
- **Difficulty:** 4
- **Question:** Sound intensity level is measured on the decibel scale. If the sound intensity increases by a factor of 100, approximately how much does the sound intensity level increase?
- **A:** 2 dB
- **B:** 20 dB
- **C:** 100 dB
- **D:** 200 dB
- **Correct:** B
- **Explanation:** The decibel scale is logarithmic, so each factor of 10 in intensity adds about 10 dB; a factor of 100 is two such steps, adding about 20 dB.
- **Tags:** `KC::Physics::Sound` `Prereq::Physics::Waves` `MCAT::Chem_Phys` `Difficulty::4`

## Physics::Fluid_Statics

### MCAT-PHY-FLUS-001

- **KC:** `Physics::Fluid_Statics`
- **Prereqs:** `Prereq::Physics::Force_Equilibrium`
- **Difficulty:** 3
- **Question:** A solid object floats at rest, partially submerged in water. What is true about the buoyant force acting on it?
- **A:** It is greater than the object's weight
- **B:** It equals the object's weight
- **C:** It is less than the object's weight
- **D:** It is zero because the object floats
- **Correct:** B
- **Explanation:** A floating object is in equilibrium, so by Archimedes' principle the upward buoyant force exactly balances the object's weight.
- **Tags:** `KC::Physics::Fluid_Statics` `Prereq::Physics::Force_Equilibrium` `MCAT::Chem_Phys` `Difficulty::3`

## Physics::Fluid_Dynamics

### MCAT-PHY-FLUD-001

- **KC:** `Physics::Fluid_Dynamics`
- **Prereqs:** `Prereq::Physics::Fluid_Statics` `Prereq::Physics::Work_And_Energy`
- **Difficulty:** 4
- **Question:** An incompressible fluid flows through a horizontal pipe that narrows to half its cross-sectional area. Compared with the wide section, the fluid in the narrow section has:
- **A:** lower speed and higher pressure
- **B:** higher speed and lower pressure
- **C:** higher speed and higher pressure
- **D:** lower speed and lower pressure
- **Correct:** B
- **Explanation:** The continuity equation requires higher speed where the area is smaller, and Bernoulli's equation then predicts lower pressure where the fluid moves faster.
- **Tags:** `KC::Physics::Fluid_Dynamics` `Prereq::Physics::Fluid_Statics` `Prereq::Physics::Work_And_Energy` `MCAT::Chem_Phys` `Difficulty::4`

## Physics::Thermodynamics

### MCAT-PHY-THE-001

- **KC:** `Physics::Thermodynamics`
- **Prereqs:** `Prereq::GenChem::Thermochemistry` `Prereq::Physics::Work_And_Energy`
- **Difficulty:** 3
- **Question:** A gas is compressed quickly with no heat exchange with its surroundings. According to the first law of thermodynamics, what happens to the internal energy of the gas?
- **A:** It decreases
- **B:** It stays the same
- **C:** It increases
- **D:** It depends only on the final volume
- **Correct:** C
- **Explanation:** With no heat transfer, the work done on the gas during compression goes into internal energy, so the internal energy (and temperature) rises.
- **Tags:** `KC::Physics::Thermodynamics` `Prereq::GenChem::Thermochemistry` `Prereq::Physics::Work_And_Energy` `MCAT::Chem_Phys` `Difficulty::3`

## Physics::Electrostatics

### MCAT-PHY-ELE-001

- **KC:** `Physics::Electrostatics`
- **Prereqs:** `Prereq::Physics::Work_And_Energy`
- **Difficulty:** 3
- **Question:** Two point charges exert an electrostatic force on each other. If the distance between them is doubled, how does the magnitude of the force change?
- **A:** It is halved
- **B:** It is doubled
- **C:** It is reduced to one-fourth
- **D:** It is quadrupled
- **Correct:** C
- **Explanation:** By Coulomb's law the force is inversely proportional to the square of the separation, so doubling the distance reduces the force to one-fourth.
- **Tags:** `KC::Physics::Electrostatics` `Prereq::Physics::Work_And_Energy` `MCAT::Chem_Phys` `Difficulty::3`

## Physics::Electrical_Circuits

### MCAT-PHY-ELEC-001

- **KC:** `Physics::Electrical_Circuits`
- **Prereqs:** `Prereq::Physics::Electrostatics`
- **Difficulty:** 3
- **Question:** Two identical resistors are connected in parallel across a battery. Compared with a single one of those resistors alone, the equivalent resistance of the parallel pair is:
- **A:** twice as large
- **B:** half as large
- **C:** the same
- **D:** four times as large
- **Correct:** B
- **Explanation:** Two equal resistors in parallel have an equivalent resistance equal to half the value of one resistor.
- **Tags:** `KC::Physics::Electrical_Circuits` `Prereq::Physics::Electrostatics` `MCAT::Chem_Phys` `Difficulty::3`

## Physics::Circuit_Elements

### MCAT-PHY-CIRE-001

- **KC:** `Physics::Circuit_Elements`
- **Prereqs:** `Prereq::Physics::Electrical_Circuits`
- **Difficulty:** 4
- **Question:** A parallel-plate capacitor stays connected to a constant-voltage battery while a dielectric is inserted between its plates. How does the charge stored on the capacitor change?
- **A:** It increases
- **B:** It decreases
- **C:** It stays the same
- **D:** It becomes zero
- **Correct:** A
- **Explanation:** A dielectric raises the capacitance, and at constant voltage the stored charge equals capacitance times voltage, so the charge increases.
- **Tags:** `KC::Physics::Circuit_Elements` `Prereq::Physics::Electrical_Circuits` `MCAT::Chem_Phys` `Difficulty::4`

## Physics::Magnetism

### MCAT-PHY-MAG-001

- **KC:** `Physics::Magnetism`
- **Prereqs:** `Prereq::Physics::Electrostatics`
- **Difficulty:** 4
- **Question:** A positive charge moves exactly parallel to a uniform magnetic field. What magnetic force does the charge experience?
- **A:** A force in the direction of its motion
- **B:** A force opposite to its motion
- **C:** Zero force
- **D:** A force that steadily increases its speed
- **Correct:** C
- **Explanation:** The magnetic force depends on the component of velocity perpendicular to the field, so a charge moving parallel to the field feels no magnetic force.
- **Tags:** `KC::Physics::Magnetism` `Prereq::Physics::Electrostatics` `MCAT::Chem_Phys` `Difficulty::4`

## Physics::Electromagnetic_Radiation

### MCAT-PHY-ELER-001

- **KC:** `Physics::Electromagnetic_Radiation`
- **Prereqs:** `Prereq::Physics::Magnetism` `Prereq::Physics::Waves`
- **Difficulty:** 3
- **Question:** Blue light has a shorter wavelength than red light. In a vacuum, how does the energy of a single blue photon compare with that of a single red photon?
- **A:** Lower, because shorter waves carry less energy
- **B:** Higher, because photon energy increases with frequency
- **C:** The same, because both travel at the speed of light
- **D:** Zero, because photons have no rest mass
- **Correct:** B
- **Explanation:** Photon energy equals Planck's constant times frequency, and a shorter wavelength means a higher frequency, so the blue photon has more energy.
- **Tags:** `KC::Physics::Electromagnetic_Radiation` `Prereq::Physics::Magnetism` `Prereq::Physics::Waves` `MCAT::Chem_Phys` `Difficulty::3`

## Physics::Geometric_Optics

### MCAT-PHY-GEOO-001

- **KC:** `Physics::Geometric_Optics`
- **Prereqs:** `Prereq::Physics::Electromagnetic_Radiation`
- **Difficulty:** 4
- **Question:** An object is placed beyond the focal length of a converging (convex) lens. What kind of image does the lens form?
- **A:** Virtual and upright
- **B:** Real and inverted
- **C:** Virtual and inverted
- **D:** No image forms at all
- **Correct:** B
- **Explanation:** When an object is beyond the focal length of a converging lens, the refracted rays actually converge on the far side, forming a real, inverted image.
- **Tags:** `KC::Physics::Geometric_Optics` `Prereq::Physics::Electromagnetic_Radiation` `MCAT::Chem_Phys` `Difficulty::4`

## Physics::Physical_Optics

### MCAT-PHY-PHYO-001

- **KC:** `Physics::Physical_Optics`
- **Prereqs:** `Prereq::Physics::Geometric_Optics` `Prereq::Physics::Waves`
- **Difficulty:** 4
- **Question:** In a double-slit experiment with monochromatic light, a bright fringe appears at a point on the screen where the light waves from the two slits arrive:
- **A:** exactly out of phase
- **B:** in phase, interfering constructively
- **C:** with a randomly varying phase
- **D:** polarized perpendicular to each other
- **Correct:** B
- **Explanation:** Bright fringes come from constructive interference, which occurs where the path difference makes the two waves arrive in phase.
- **Tags:** `KC::Physics::Physical_Optics` `Prereq::Physics::Geometric_Optics` `Prereq::Physics::Waves` `MCAT::Chem_Phys` `Difficulty::4`

## Physics::Atomic_Structure

### MCAT-PHY-ATOS-001

- **KC:** `Physics::Atomic_Structure`
- **Prereqs:** `Prereq::Physics::Electromagnetic_Radiation` `Prereq::Physics::Electrostatics`
- **Difficulty:** 4
- **Question:** An electron in a hydrogen atom drops from a higher energy level to a lower one. What is emitted, and what sets its wavelength?
- **A:** A photon whose energy equals the difference between the two levels
- **B:** An electron with kinetic energy equal to the level difference
- **C:** A proton whose mass corresponds to the energy difference
- **D:** Heat only, unrelated to the spacing of the levels
- **Correct:** A
- **Explanation:** Atomic energy levels are quantized, so the emitted photon carries an energy exactly equal to the gap between levels, which fixes its frequency and wavelength.
- **Tags:** `KC::Physics::Atomic_Structure` `Prereq::Physics::Electromagnetic_Radiation` `Prereq::Physics::Electrostatics` `MCAT::Chem_Phys` `Difficulty::4`

## Physics::Nuclear_Physics

### MCAT-PHY-NUCP-001

- **KC:** `Physics::Nuclear_Physics`
- **Prereqs:** `Prereq::Physics::Atomic_Structure`
- **Difficulty:** 4
- **Question:** A radioactive isotope has a half-life of 8 days. What fraction of an initial sample remains after 24 days?
- **A:** One-half
- **B:** One-quarter
- **C:** One-eighth
- **D:** None of it
- **Correct:** C
- **Explanation:** In 24 days the sample passes through three half-lives, so the remaining fraction is one-half raised to the third power, which is one-eighth.
- **Tags:** `KC::Physics::Nuclear_Physics` `Prereq::Physics::Atomic_Structure` `MCAT::Chem_Phys` `Difficulty::4`

## Physics::Circulatory_Hemodynamics

### MCAT-PHY-CIRH-001

- **KC:** `Physics::Circulatory_Hemodynamics`
- **Prereqs:** `Prereq::Physics::Fluid_Dynamics`
- **Difficulty:** 4
- **Question:** Blood moves much more slowly through the capillary beds than through the aorta, even though each individual capillary is tiny. Which factor best explains this slower flow?
- **A:** Blood suddenly becomes far more viscous in capillaries
- **B:** The combined cross-sectional area of all the capillaries is much larger than that of the aorta
- **C:** Blood pressure reaches its highest value in the capillaries
- **D:** Capillaries actively pump blood backward against flow
- **Correct:** B
- **Explanation:** By the continuity principle, flow speed drops where total cross-sectional area rises, and the summed area of the capillary bed greatly exceeds that of the aorta.
- **Tags:** `KC::Physics::Circulatory_Hemodynamics` `Prereq::Physics::Fluid_Dynamics` `MCAT::Chem_Phys` `Difficulty::4`

## Physics::Gas_Exchange_And_Respiration_Physics

### MCAT-PHY-GASERP-001

- **KC:** `Physics::Gas_Exchange_And_Respiration_Physics`
- **Prereqs:** `Prereq::GenChem::Gas_Phase` `Prereq::Physics::Fluid_Statics` `Prereq::Physics::Thermodynamics`
- **Difficulty:** 4
- **Question:** Pulmonary surfactant lowers the surface tension inside alveoli. According to the law of Laplace, why does this help keep small alveoli from collapsing?
- **A:** It raises the pressure required to inflate the largest alveoli
- **B:** It lowers the inward collapsing pressure that otherwise rises as alveolar radius shrinks
- **C:** It removes oxygen's partial pressure from the alveolar air
- **D:** It makes the collapsing pressure independent of alveolar radius
- **Correct:** B
- **Explanation:** By Laplace's law the collapsing pressure grows as radius shrinks, so by reducing surface tension most in small alveoli, surfactant lowers that pressure and prevents collapse.
- **Tags:** `KC::Physics::Gas_Exchange_And_Respiration_Physics` `Prereq::GenChem::Gas_Phase` `Prereq::Physics::Fluid_Statics` `Prereq::Physics::Thermodynamics` `MCAT::Chem_Phys` `Difficulty::4`

## Physics::Optics_Of_The_Eye

### MCAT-PHY-OPTE-001

- **KC:** `Physics::Optics_Of_The_Eye`
- **Prereqs:** `Prereq::Physics::Geometric_Optics`
- **Difficulty:** 4
- **Question:** In a nearsighted (myopic) eye, images of distant objects are focused in front of the retina. Which type of corrective lens moves the focus back onto the retina?
- **A:** A converging (convex) lens
- **B:** A diverging (concave) lens
- **C:** A flat prism with no focal length
- **D:** A polarizing filter
- **Correct:** B
- **Explanation:** Myopia focuses distant light too soon, so a diverging lens spreads the incoming rays first, letting the eye form the image farther back on the retina.
- **Tags:** `KC::Physics::Optics_Of_The_Eye` `Prereq::Physics::Geometric_Optics` `MCAT::Chem_Phys` `Difficulty::4`

## Physics::Bioelectricity

### MCAT-PHY-BIO-001

- **KC:** `Physics::Bioelectricity`
- **Prereqs:** `Prereq::GenChem::Ions_in_Solutions` `Prereq::Physics::Circuit_Elements`
- **Difficulty:** 4
- **Question:** A neuron's plasma membrane is often modeled as a capacitor in parallel with a resistor. Which membrane property corresponds to the capacitance in this model?
- **A:** The ability of open ion channels to let ions leak across the membrane
- **B:** The lipid bilayer's ability to hold separated charge on its two surfaces
- **C:** The rate at which the sodium-potassium pump consumes ATP
- **D:** The concentration of glucose inside the cell
- **Correct:** B
- **Explanation:** The thin insulating bilayer separates charge between the inside and outside of the cell, storing it like a capacitor, while ion channels form the parallel resistive leak path.
- **Tags:** `KC::Physics::Bioelectricity` `Prereq::GenChem::Ions_in_Solutions` `Prereq::Physics::Circuit_Elements` `MCAT::Chem_Phys` `Difficulty::4`
