# Chemistry Module

The **Chemistry** module spans **26 submodules** across **56 source files**, covering analytical, physical, organic, inorganic, and computational chemistry. It provides tools for reaction modeling, equilibrium calculations, spectroscopic analysis, and materials characterization.

---

## Chapter 1 - Module Scope

### Scientific Purpose
This section defines the scope: which problem this module solves, which abstraction level it targets, and which outputs are expected.

### Modeling Assumptions
- The equations are numerical models, not analytical proofs.
- Inputs must respect the documented units.
- Validity domains (linear regime, local approximation, and so on) should be verified by the reader.

### Reading Strategy
1. Read the module structure.
2. Identify equations and their conditions.
3. Map these equations to Rust functions in `docs/code/`.


## Reading Guide

### Goal
This page explains the module from a scientific perspective: assumptions, models, equations, and usage limits.

### How To Read This Document
- Start with the domain section relevant to your case.
- Identify key equations and their usage conditions.
- Then verify the corresponding Rust signatures in `docs/code/`.

### Conventions
- Units: SI unless explicitly stated.
- Variables: standard mathematical notation for the domain.
- Equations are presented as reference models, not formal proofs.


## Chapter 2 - Submodules and Models

### 1. Acid-Base Chemistry

Proton transfer equilibria govern pH, buffer behavior, and titration analysis — central to biology, environmental chemistry, and industrial processes.

**Equilibria** — The Henderson-Hasselbalch equation $\text{pH} = \text{p}K_a + \log_{10}([A^-]/[HA])$ predicts pH of buffer solutions. The module computes pH of strong acids ($\text{pH} = -\log c$), strong bases ($\text{pH} = 14 + \log c$), weak acids ($\text{pH} = \frac{1}{2}(\text{p}K_a - \log c)$), and weak bases. Interconversion between $K_a$, $\text{p}K_a$, and $\text{p}K_b$ is provided ($\text{p}K_a + \text{p}K_b = \text{p}K_w$). For polyprotic systems, the alpha fraction $\alpha_i$ gives the proportion of each protonation state as a function of [H⁺] and the successive $K_a$ values. The amphiprotic pH formula $\text{pH} \approx (\text{p}K_{a1} + \text{p}K_{a2})/2$ handles zwitterionic species.

**Titrations** — Equivalence point determination, half-equivalence pH (equals $\text{p}K_a$), and complete titration curve modeling for mono- and polyprotic acids.

---

### 2. Analytical Chemistry

Quantitative and qualitative techniques for chemical characterization.

**Chromatography** — The van Deemter equation $H = A + B/u + Cu$ models plate height as a function of mobile-phase velocity, combining eddy diffusion ($A$), longitudinal diffusion ($B/u$), and mass transfer resistance ($Cu$). Resolution $R_s$, capacity factor, retention factor, selectivity, and number of theoretical plates are all computed.

**Quantitative Analysis** — Gravimetric yield, volumetric titration calculations, standard addition method, and calibration curve regression.

**Spectrophotometry** — Beer-Lambert law $A = \varepsilon \ell c$ relates absorbance to molar absorptivity, path length, and concentration. Transmittance $T = 10^{-A}$, multi-component analysis via linear superposition of absorptivities.

---

### 3. Colloids

Chemistry of dispersed systems (nanoparticles, emulsions, foams) where interface effects dominate bulk properties.

**Properties** — Particle size distribution statistics, specific surface area, sedimentation rate (Stokes' law $v = 2r^2\Delta\rho g/(9\mu)$), and Brownian motion.

**Stability** — DLVO theory combines van der Waals attraction and electric double-layer repulsion to predict colloidal stability. The module computes zeta potential, coagulation kinetics, Schulze-Hardy rule (critical coagulation concentration scales as $z^{-6}$ for the ion valence), and flocculation rates.

---

### 4. Computational Chemistry

Numerical approaches to molecular electronic structure.

**Basis Sets** — Construction of Gaussian-type orbital (GTO) and Slater-type orbital (STO) basis sets for molecular calculations. Primitive and contracted Gaussian parameters.

**Density Functional Theory** — DFT total energy calculations using exchange-correlation functionals. Energy decomposition into kinetic, Coulomb, exchange, and correlation contributions.

---

### 5. Crystallography

Solid-state structural analysis using diffraction and lattice geometry.

**Diffraction** — Bragg's law $n\lambda = 2d\sin\theta$ for constructive interference of X-rays. Structure factor $F_{hkl}$ calculation, powder diffraction peak positions, reciprocal lattice vectors.

**Lattice** — Unit cell parameters (a, b, c, α, β, γ), the seven crystal systems, packing fractions (FCC 0.74, BCC 0.68, SC 0.52), Miller index determination, and interplanar distances $d_{hkl} = a/\sqrt{h^2+k^2+l^2}$ for cubic systems.

---

### 6. Electrochemistry

Electron transfer processes, from batteries to corrosion to electroplating.

**Cells** — The Nernst equation $E = E^0 - \frac{RT}{nF}\ln Q$ gives the cell potential under non-standard conditions. Standard electrode potentials combine as $E_{\text{cell}} = E_{\text{cathode}} - E_{\text{anode}}$. Gibbs free energy $\Delta G = -nFE$. Faraday's law of electrolysis $m = ItM/(nF)$ gives the mass deposited. The Tafel equation $\eta = a + b\log|j|$ models overpotential. The Butler-Volmer equation $j = j_0[\exp(\alpha_a F\eta/RT) - \exp(-\alpha_c F\eta/RT)]$ describes full current-overpotential behavior. Battery metrics: open-circuit voltage, energy density (Wh/kg), Faradaic efficiency, and coulombic efficiency.

**Transport** — Ionic conductivity, transference numbers, Debye-Hückel limiting law $\log\gamma_\pm = -A|z_+z_-|\sqrt{I}$ for activity coefficients in dilute electrolytes.

---

### 7. Environmental Chemistry

Chemistry of natural systems and pollutant fate.

**Atmosphere** — Ozone formation and destruction cycles, photochemical smog kinetics, greenhouse gas atmospheric lifetimes, and radiative forcing.

**Water** — Dissolved oxygen modeling, biological and chemical oxygen demand (BOD/COD), water hardness (Ca²⁺ + Mg²⁺ equivalents), and composite water quality indices.

---

### 8. Equilibrium

Chemical equilibrium thermodynamics.

**Constants** — Equilibrium constant expressions $K = \prod [P]^{n_p}/\prod [R]^{n_r}$, reaction quotient $Q$, Le Chatelier's principle (qualitative), and $K_p/K_c$ interconversion via $K_p = K_c(RT)^{\Delta n}$.

**Ionic Equilibria** — Solubility product $K_{sp}$, common ion effect on solubility, complex ion formation constants, and precipitation criteria ($Q > K_{sp}$).

---

### 9. Gas Laws

Behavior of ideal and real gases under varying conditions.

**Ideal** — $PV = nRT$, combined gas law $P_1V_1/T_1 = P_2V_2/T_2$, Dalton's law of partial pressures $P_{\text{total}} = \sum P_i$, Graham's law of effusion $\text{rate} \propto 1/\sqrt{M}$, and molar volume at STP.

**Real** — Van der Waals equation $(P + a/V_m^2)(V_m - b) = RT$ corrects for molecular attractions ($a$) and finite molecular volume ($b$). Compressibility factor $Z = PV/(nRT)$ quantifies departure from ideality. Virial coefficients for systematic correction.

---

### 10. Green Chemistry

Sustainability metrics for chemical process evaluation.

**Metrics** — Atom economy $= M_{\text{product}}/M_{\text{reactants}} \times 100\%$, E-factor (mass of waste per mass of product), and process mass intensity (total mass input per mass of product).

**Principles** — The twelve principles of green chemistry (prevention, atom economy, less hazardous synthesis, etc.) encoded as a decision framework, with solvent selection guidelines.

---

### 11. Inorganic Chemistry

Non-carbon-based chemical systems, coordination compounds, and crystal field theory.

**Bonding** — Ionic bond energy, Born-Landé lattice energy $U = \frac{N_A M z^+ z^- e^2}{4\pi\varepsilon_0 r_0}(1-1/n)$, and electronegativity difference classification (ionic vs. covalent).

**Coordination** — Complex geometry (octahedral, tetrahedral, square planar), chelate effect thermodynamics, stability constants, and isomer enumeration.

**Crystal Field Theory** — Octahedral splitting $\Delta_{\text{oct}} = 10Dq$, tetrahedral splitting $\Delta_{\text{tet}} = \frac{4}{9}\Delta_{\text{oct}}$. Crystal field stabilization energy (CFSE) accounting for electron occupation and pairing energy. Spin-only magnetic moment $\mu = \sqrt{n(n+2)}\,\mu_B$. Spectrochemical series (ligand $f$ × metal $g$ product). Jahn-Teller distortion prediction. Nephelauxetic ratio $\beta = B_{\text{complex}}/B_{\text{free ion}}$ measures covalency. Racah parameter extraction from electronic transitions.

---

### 12. Kinetics

Rates and mechanisms of chemical reactions — essential for reactor design, pharmacology, and environmental fate modeling.

**Rate Laws** — The Arrhenius equation $k = A e^{-E_a/RT}$ connects rate constant to activation energy. Integrated rate laws: zero-order $[A] = [A]_0 - kt$, first-order $[A] = [A]_0 e^{-kt}$, second-order $1/[A] - 1/[A]_0 = kt$, and the general $n$th-order form. Half-lives: $t_{1/2} = \ln 2/k$ (first-order), $t_{1/2} = 1/(k[A]_0)$ (second-order), $t_{1/2} = [A]_0/(2k)$ (zero-order). Activation energy extraction from two temperatures: $E_a = R\ln(k_2/k_1)/(1/T_1 - 1/T_2)$.

**Advanced Kinetics** — Eyring equation $k = \kappa(k_BT/h)\exp(-\Delta G^\ddagger/RT)$ from transition-state theory. Collision frequency from kinetic molecular theory. General rate law $r = k\prod [A_i]^{n_i}$.

**Mechanisms** — Elementary step analysis, rate-determining step identification, steady-state approximation for reactive intermediates.

---

### 13. Molecular Chemistry

Molecular structure, bonding, and geometry prediction.

**Bonding** — VSEPR theory, hybridization assignment (sp, sp², sp³, etc.), bond order, and electronegativity (Pauling, Mulliken).

**Geometry** — Bond angle prediction, molecular shape from electron-pair geometry, and dipole moment vector summation.

---

### 14. Nuclear Chemistry

Radioactive processes and nuclear energy.

**Decay** — Alpha, beta, gamma decay rate laws, decay series modeling, and nuclear transmutation calculations.

**Energy** — Binding energy per nucleon from mass defect, fission and fusion energy release, and mass-energy equivalence $E = \Delta m \cdot c^2$.

---

### 15. Organic Chemistry

Carbon-based molecular systems — the foundation of biochemistry and materials science.

**Descriptors** — Molecular weight, degree of unsaturation $\text{DoU} = (2C+2+N-H)/2$, and functional group classification.

**Reactions** — SN1 (rate $= k[\text{substrate}]$), SN2 (rate $= k[\text{substrate}][\text{nucleophile}]$), E1, and E2 kinetics. The Hammett equation $\log(k/k_0) = \rho\sigma$ quantifies substituent electronic effects via $\sigma$ constants and reaction sensitivity $\rho$. The Taft equation separates steric from electronic contributions: $\log(k/k_0) = \rho^*\sigma^* + \delta E_s$.

**Structure** — Constitutional and stereoisomerism, chirality, conformational analysis (Newman projections, ring strain).

---

### 16. Photochemistry

Light-driven chemical processes, fundamental to photosynthesis, solar cells, and photocatalysis.

**Kinetics** — Photochemical reaction rates, photosensitization, and dynamic quenching.

**Quantum Yield** — Fluorescence ($\Phi_f$) and phosphorescence yields, Stern-Volmer analysis $I_0/I = 1 + K_{SV}[Q]$ for quencher concentration dependence.

---

### 17. Polymers

Macromolecular chemistry governing plastics, fibers, and biological polymers.

**Distributions** — Number-average ($M_n$) and weight-average ($M_w$) molecular weight, polydispersity index $PDI = M_w/M_n$, and degree of polymerization.

**Properties** — Mark-Houwink equation $[\eta] = KM^a$ for intrinsic viscosity. End-to-end distance of a freely-jointed chain $R = l\sqrt{N}$. Radius of gyration $R_g = R/\sqrt{6}$. Fox equation for glass transition of blends. Carothers equation $\bar{X}_n = 2/(2 - p\bar{f})$ for step-growth polymerization degree.

---

### 18. Quantum Chemistry

Quantum mechanical treatment of molecular electronic structure.

**Hückel MO Theory** — Secular determinant construction for linear and cyclic conjugated systems. The $\alpha + x\beta$ Hamiltonian matrix encodes nearest-neighbor interactions. $\pi$-electron energies, delocalization energy, and charge density distributions from MO coefficients and occupation numbers.

**Orbitals** — Atomic orbital energies, electron configurations, molecular orbital construction, and orbital symmetry.

---

### 19. Reaction Engineering

Chemical reactor analysis and design for industrial-scale processes.

**Design** — CSTR (continuous stirred-tank reactor) design equation $V = F_{A0}X/(-r_A)$ and PFR (plug flow reactor) equation $V = F_{A0}\int_0^X dX'/(-r_A)$. Residence time, space velocity, and conversion calculations.

**Reactors** — Batch, continuous, and semi-batch reactor models, selectivity, and yield optimization for parallel and series reactions.

---

### 20. Solid-State Chemistry

Electronic and defect structure of crystalline materials.

**Band Theory** — Band gap determination, classification of conductors, insulators, and semiconductors, and Fermi level positioning.

**Defects** — Point defects (vacancies, interstitials), Schottky pairs ($n_s = N\exp(-E_s/2k_BT)$), Frenkel defects, and defect equilibrium thermodynamics.

---

### 21. Solutions

Dissolved species behavior and mixture thermodynamics.

**Colligative Properties** — Boiling point elevation $\Delta T_b = iK_bm$, freezing point depression $\Delta T_f = iK_fm$, osmotic pressure $\Pi = iMRT$, and vapor pressure lowering (Raoult's law). Molar mass determination from ebullioscopy and cryoscopy.

**Mixtures** — Raoult's law $P_i = x_i P_i^0$ for ideal solutions, Henry's law $P = K_H x$ for dilute solutes, activity coefficients for real mixtures, and excess Gibbs energy.

---

### 22. Spectroscopy

Interaction of electromagnetic radiation with matter for structural determination.

**IR Spectroscopy** — Infrared absorption frequencies mapped to functional groups. Selection rules (dipole moment change required). Characteristic group frequencies for O-H, N-H, C=O, C-H, etc.

**Mass Spectrometry** — Mass-to-charge ratio analysis, isotope pattern computation, fragmentation pattern prediction, and molecular ion identification.

**NMR Spectroscopy** — Chemical shift $\delta = (f - f_{\text{ref}})/f_{\text{spec}} \times 10^6$ ppm. Spin-spin coupling constants. Multiplicity from $n+1$ rule. Larmor frequency $\nu = \gamma B_0/(2\pi)$. Relaxation: $T_1$ (inversion recovery $M = M_0(1 - 2e^{-t/T_1})$) and $T_2$ (spin echo $M = M_0 e^{-t/T_2}$). Linewidth $\Delta\nu = 1/(\pi T_2)$. Nuclear Overhauser enhancement $\eta = 1 + \gamma_I/(2\gamma_S)$.

---

### 23. Stoichiometry

Quantitative relationships in chemical reactions.

**Balancing** — Chemical equation balancing using oxidation number methods and half-reaction approaches.

**Calculations** — Mole ratios from balanced equations, limiting reagent identification, theoretical yield, and percent yield.

---

### 24. Surface Chemistry

Interfacial phenomena controlling catalysis, adsorption, and wetting.

**Adsorption** — Langmuir isotherm $\theta = K P/(1+KP)$ (monolayer, uniform surface), Freundlich isotherm $q = K_f P^{1/n}$ (heterogeneous surface), BET isotherm $V = V_m c x/[(1-x)(1-x+cx)]$ for multilayer adsorption ($x = P/P_0$). Temkin isotherm (heat of adsorption decreases linearly with coverage). Dissociative Langmuir adsorption. BET surface area determination from monolayer volume.

**Surface Tension** — Surface tension, capillarity, Young-Laplace equation $\Delta P = 2\gamma/r$, and contact angle (Young's equation).

---

### 25. Thermochemistry

Energy changes in chemical processes.

**Enthalpy** — Hess's law (additivity of enthalpies), standard enthalpies of formation, bond enthalpies (average bond dissociation energies), and calorimetry calculations.

**Entropy** — Standard molar entropy, Gibbs free energy $\Delta G = \Delta H - T\Delta S$, and spontaneity criteria ($\Delta G < 0$).

---

### 26. Transport Phenomena

Mass and momentum transfer in chemical systems.

**Diffusion** — Fick's first law $J = -D\,dc/dx$ (steady-state flux) and second law $\partial c/\partial t = D\,\partial^2 c/\partial x^2$ (transient). Diffusion coefficients in gases and liquids.

**Mass Transfer** — Convective mass transfer coefficients, film theory (stagnant film model), and penetration theory (surface renewal model).

---

## Chapter 3 - Limits, Precision, and Validation

### Numerical Limits
- `f64` rounding errors can accumulate in long simulations.
- Extreme regimes (very large or very small scales) require explicit numerical stability checks.

### Recommended Verification
- Compare against a simple analytical case when available.
- Check the order of magnitude of results.
- Run sensitivity analysis on dominant parameters.

### Transition to Implementation
For concrete function calls, Rust signatures and module paths are documented in `docs/code/`.
