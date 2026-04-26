# Biology Module

The biology module is the largest in SciForge, spanning **44 submodules** across **176 source files**. It provides computational models for molecular biology, cellular processes, organismal physiology, population dynamics, and ecological systems — from the scale of individual molecules to entire ecosystems.

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


## 1. Aging

Models of biological aging, senescence, and mortality across the lifespan.

**Telomere dynamics.** Telomeres shorten with each cell division. SciForge models linear attrition:

$$L(n) = L_0 - \delta \cdot n$$

where $L_0$ is the initial telomere length, $\delta$ the loss per division, and $n$ the number of divisions. The **Hayflick limit** (maximum replicative capacity) is:

$$n_{\max} = \frac{L_0 - L_{\text{crit}}}{\delta}$$

When telomerase is active, an equilibrium length emerges from the balance of shortening rate $k_s$ and elongation rate $k_e$:

$$L_{\text{eq}} = \frac{k_e}{k_s}, \qquad L(t) = L_{\text{eq}} + (L_0 - L_{\text{eq}}) e^{-k_s t}$$

**Mortality models.** The **Gompertz law** describes the exponential increase of mortality with age:

$$\mu(x) = a \, e^{bx}$$

where $a$ is the baseline mortality and $b$ the rate of aging. The **Gompertz–Makeham** extension adds an age-independent term $c$:

$$\mu(x) = c + a \, e^{bx}$$

The **mortality rate doubling time** (MRDT) is $\ln 2 / b$. Survival probability follows:

$$S(x) = \exp\!\Bigl(-\frac{a}{b}(e^{bx} - 1)\Bigr)$$

The **Weibull** alternative gives $h(x) = \lambda k (\lambda x)^{k-1}$ with survival $S(x) = e^{-(\lambda x)^k}$.

**Oxidative stress.** ROS production depends on metabolic rate and mitochondrial coupling efficiency:

$$\dot{R} = M \cdot (1 - \eta)$$

Antioxidant capacity integrates SOD, catalase, and glutathione: $A = [\text{SOD}] k_1 + [\text{Cat}] k_2 + [\text{GSH}] k_3$. Net oxidative damage rate is $(\dot{R} - A)^+$. Lipid peroxidation follows a two-step initiation-propagation model, and protein carbonylation is modeled as second-order kinetics.

---

## 2. Bioelectricity

Electrical phenomena in biological membranes and tissues.

**Nernst equation.** The equilibrium potential for a single ion species:

$$E_{\text{ion}} = \frac{RT}{zF} \ln \frac{[\text{ion}]_{\text{out}}}{[\text{ion}]_{\text{in}}}$$

**Goldman–Hodgkin–Katz equation.** The resting membrane potential considering multiple permeant ions (Na⁺, K⁺, Cl⁻):

$$V_m = \frac{RT}{F} \ln \frac{P_{\text{K}}[\text{K}^+]_o + P_{\text{Na}}[\text{Na}^+]_o + P_{\text{Cl}}[\text{Cl}^-]_i}{P_{\text{K}}[\text{K}^+]_i + P_{\text{Na}}[\text{Na}^+]_i + P_{\text{Cl}}[\text{Cl}^-]_o}$$

**Action potential.** SciForge implements the Hodgkin–Huxley model with gating variables $m$, $h$, $n$:

$$C_m \frac{dV}{dt} = -g_{\text{Na}} m^3 h (V - E_{\text{Na}}) - g_{\text{K}} n^4 (V - E_{\text{K}}) - g_L (V - E_L) + I_{\text{ext}}$$

Propagation velocity depends on fiber diameter $d$ and membrane properties. Cable equation analysis gives the length constant $\lambda = \sqrt{r_m / r_i}$.

**Bioimpedance.** Tissue impedance is modeled with the Cole equation: $Z(\omega) = R_\infty + \frac{R_0 - R_\infty}{1 + (j\omega\tau)^\alpha}$, used in body composition analysis and tissue characterization.

---

## 3. Bioenergetics

Energy transformation in living systems.

**ATP synthesis.** The rate of mitochondrial ATP production depends on proton motive force and the P/O ratio. Energy charge quantifies the adenylate pool status:

$$\text{EC} = \frac{[\text{ATP}] + \frac{1}{2}[\text{ADP}]}{[\text{ATP}] + [\text{ADP}] + [\text{AMP}]}$$

Cells maintain EC near 0.85–0.90 for normal function.

**Metabolic scaling.** Basal metabolic rate scales allometrically with body mass:

$$B = B_0 M^{3/4}$$

(Kleiber's law). The metabolic theory of ecology extends this to temperature with an Arrhenius-type activation:

$$B = B_0 M^{3/4} e^{-E_a / kT}$$

**Photosynthesis.** Light reactions are modeled with quantum yield $\Phi$: the ratio of CO₂ fixed to photons absorbed. The light response curve follows a rectangular hyperbola:

$$A = \frac{A_{\max} \cdot I}{K_I + I} - R_d$$

where $A_{\max}$ is maximum assimilation rate, $K_I$ the half-saturation light intensity, and $R_d$ the dark respiration. Calvin cycle efficiency depends on RuBisCO kinetics (Michaelis–Menten with CO₂/O₂ competition).

**Respiration.** Aerobic yield: ~30–32 ATP per glucose. ATP yield from substrate-level and oxidative phosphorylation is computed stoichiometrically. Anaerobic glycolysis yields 2 ATP per glucose with lactate as end product.

---

## 4. Biogeography

Geographic distribution of species.

**Island biogeography.** MacArthur and Wilson's equilibrium theory models species richness as the balance of immigration rate $I$ and extinction rate $E$:

$$\frac{dS}{dt} = I_0 \left(1 - \frac{S}{P}\right) - E_0 \frac{S}{P}$$

where $P$ is the mainland species pool and $S$ the island species count. Equilibrium: $\hat{S} = P \cdot I_0 / (I_0 + E_0)$.

**Species–area relationship:** $S = c A^z$, where $z \approx 0.25$ for islands and $z \approx 0.15$ for continental samples.

**Climate envelope models.** Species occurrence probability is computed from temperature, precipitation, and altitude using Gaussian suitability functions, enabling range shift predictions under climate scenarios.

---

## 5. Bioinformatics

Computational analysis of biological sequences.

**Sequence alignment.** The **Needleman–Wunsch** algorithm (global alignment) fills a dynamic programming matrix $F$ with:

$$F_{i,j} = \max \begin{cases} F_{i-1,j-1} + s(a_i, b_j) \\ F_{i-1,j} + d \\ F_{i,j-1} + d \end{cases}$$

where $s(a_i, b_j)$ is the substitution score (from BLOSUM/PAM matrices) and $d$ the gap penalty. Smith–Waterman provides local alignment with a zero floor.

**Assembly metrics.** Genome assembly quality is assessed via **N50** (the contig length at which 50% of the total assembly is contained in contigs of that length or longer) and coverage depth:

$$\text{Coverage} = \frac{N \cdot L}{G}$$

where $N$ is the number of reads, $L$ the read length, and $G$ the genome size.

**Phred quality scores.** Base-calling confidence: $Q = -10 \log_{10} P_e$. A Phred score of 30 means $P_e = 0.001$ (99.9% accuracy). SciForge converts between Phred scores and error probabilities for read filtering pipelines.

**Gene expression.** Fold change is $\text{FC} = \bar{X}_{\text{treated}} / \bar{X}_{\text{control}}$, typically reported as $\log_2 \text{FC}$. FPKM/TPM normalization accounts for gene length and sequencing depth.

---

## 6. Biomechanics

Mechanics of biological structures and movement.

**Blood flow.** The **Hagen–Poiseuille law** governs laminar flow in vessels:

$$Q = \frac{\pi r^4 \Delta P}{8 \mu L}$$

Wall shear stress: $\tau_w = \frac{4 \mu Q}{\pi r^3}$. Non-Newtonian effects (shear-thinning of blood) are captured via power-law or Carreau models.

**Muscle mechanics.** The **Hill muscle model** relates force and velocity:

$$(F + a)(v + b) = (F_0 + a) b$$

where $F_0$ is the isometric force, $a$ and $b$ are Hill constants. The force-length relationship follows a Gaussian around optimal sarcomere length.

**Viscoelasticity.** Biological tissues are modeled as:
- **Kelvin–Voigt**: $\sigma = E\varepsilon + \eta \dot{\varepsilon}$ (creep behavior)
- **Maxwell**: $\dot{\varepsilon} = \dot{\sigma}/E + \sigma/\eta$ (stress relaxation)
- **Standard linear solid**: combines both for realistic tissue response

**Locomotion.** Metabolic cost of transport (COT) is energy per unit distance per unit mass. Stride frequency scales with limb length $l$: $f \propto \sqrt{g/l}$ (pendulum model). Froude number $\text{Fr} = v^2/(g l)$ determines gait transitions.

---

## 7. Biophysics

Physical principles applied to biological systems.

**Membrane mechanics.** The **Helfrich bending energy** of a lipid bilayer:

$$E = \frac{\kappa}{2} \oint (2H - C_0)^2 \, dA + \kappa_G \oint K \, dA$$

where $\kappa$ is the bending modulus, $H$ the mean curvature, $C_0$ the spontaneous curvature, $K$ the Gaussian curvature, and $\kappa_G$ the Gaussian modulus. This governs vesicle shapes, budding, and membrane fusion.

**Molecular forces.** The **Lennard-Jones potential** for van der Waals interactions:

$$V(r) = 4\varepsilon \left[\left(\frac{\sigma}{r}\right)^{12} - \left(\frac{\sigma}{r}\right)^6\right]$$

with well depth $\varepsilon$ and size parameter $\sigma$.

**Polymer models.** The **worm-like chain** (WLC) force-extension relation (Marko–Siggia interpolation):

$$F = \frac{k_B T}{L_p} \left[\frac{1}{4(1 - x/L_c)^2} - \frac{1}{4} + \frac{x}{L_c}\right]$$

where $L_p$ is persistence length, $L_c$ contour length. The **freely jointed chain** (FJC) uses the Langevin function: $\langle x \rangle / L_c = \coth(Fb/k_BT) - k_BT/(Fb)$.

**FRET.** Förster resonance energy transfer efficiency:

$$E = \frac{1}{1 + (r/R_0)^6}$$

where $R_0$ is the Förster radius (typically 2–7 nm). Used to measure intramolecular distances in proteins.

**Protein folding.** Two-state folding free energy: $\Delta G = -RT \ln K_{\text{eq}}$, with Ramachandran-based steric energies for backbone conformations.

---

## 8. Biostatistics

Statistical methods for biological and clinical data.

**Survival analysis.** The **Kaplan–Meier** estimator of the survival function:

$$\hat{S}(t) = \prod_{t_i \le t} \left(1 - \frac{d_i}{n_i}\right)$$

where $d_i$ is the number of events at time $t_i$ and $n_i$ the number at risk. Median survival is $t$ at $\hat{S}(t) = 0.5$.

**Clinical statistics.** Odds ratio: $\text{OR} = (a \cdot d) / (b \cdot c)$ from a 2×2 contingency table. Relative risk: $\text{RR} = [a/(a+b)] / [c/(c+d)]$. Number needed to treat: $\text{NNT} = 1/\text{ARR}$.

**Meta-analysis.** Combined effect size via inverse-variance weighting: $\hat{\theta} = \sum w_i \theta_i / \sum w_i$ with $w_i = 1/\sigma_i^2$. Heterogeneity is assessed with $I^2 = \max(0, (Q - \text{df})/Q \times 100\%)$, where $Q$ is Cochran's Q statistic.

**Logistic regression.** Probability of event: $P = 1 / (1 + e^{-(\beta_0 + \beta_1 x)})$. Log-odds: $\ln(P/(1-P)) = \beta_0 + \beta_1 x$.

---

## 9. Cancer Biology

Tumor growth, immune surveillance, and therapy modeling.

**Tumor–immune dynamics.** SciForge models the interaction between tumor cells $T$ and immune cells $I$:

$$\frac{dT}{dt} = r T \left(1 - \frac{T}{K}\right) - \alpha T I, \qquad \frac{dI}{dt} = s + \frac{\rho T I}{\eta + T} - \delta I$$

where $r$ is the tumor growth rate, $K$ carrying capacity, $\alpha$ the kill rate, $s$ the basal immune influx, $\rho$ the immune stimulation, $\eta$ the half-saturation constant, and $\delta$ immune cell death.

**Immunoediting.** The three phases (elimination, equilibrium, escape) are captured through varying $\alpha$ and $\rho$ parameters. Immunotherapy is modeled by boosting $s$ or $\alpha$.

**Gompertzian tumor growth.** $V(t) = V_0 \exp\!\left[\frac{a}{b}(1 - e^{-bt})\right]$, where growth decelerates as the tumor approaches its limiting size $V_\infty = V_0 e^{a/b}$.

---

## 10. Cell Biology

Cellular structure, division, signaling, and transport.

**Cell cycle.** Doubling time: $t_d = \ln 2 / \mu$, where $\mu$ is the specific growth rate. Checkpoint models use threshold concentrations of cyclins and CDKs.

**Receptor–ligand binding.** At equilibrium: $[RL] = \frac{[R]_T [L]}{K_d + [L]}$, following Michaelis–Menten-type saturation. Hill cooperativity extends this for multivalent receptors.

**Fick's diffusion.** Molecular transport across membranes: $J = -D \frac{dC}{dx}$. For facilitated diffusion through channels, the Goldman constant field assumption applies.

**Cell adhesion.** Adhesion energy is computed from contact area and surface energy density. Focal adhesion density depends on substrate stiffness (mechanotransduction).

---

## 11. Chronobiology

Biological rhythms and timing systems.

**Circadian oscillators.** The **Goodwin oscillator** models mRNA ($m$), protein ($p$), and repressor ($r$):

$$\frac{dm}{dt} = \frac{k_1}{1 + (r/K_i)^n} - d_1 m, \quad \frac{dp}{dt} = k_2 m - d_2 p, \quad \frac{dr}{dt} = k_3 p - d_3 r$$

Sustained oscillations require $n \ge 8$ for the Goodwin model. Coupled oscillators synchronize via diffusive coupling with phase-dependent coupling strengths.

**Entrainment.** Light pulses shift the oscillator phase according to a **phase response curve** (PRC). Entrainment range depends on the zeitgeber strength and the oscillator's natural period.

---

## 12. Cryobiology

Low-temperature biology and preservation.

**Freezing point depression.** For an ideal dilute solution: $\Delta T_f = K_f \cdot m$, where $K_f$ is the cryoscopic constant and $m$ the molality. Cryoprotectant (DMSO, glycerol) concentration modulates both osmotic stress and ice nucleation.

**Ice formation.** Intracellular ice formation probability follows an Arrhenius-type nucleation rate: $J = J_0 \exp(-\Delta G^* / k_B T)$. Rapid cooling increases intracellular ice risk; slow cooling causes osmotic dehydration.

**Viability.** Cell survival after cryopreservation follows: $V = V_0 \exp(-k \cdot t_{\text{exposure}})$, with optimal cooling rates balancing ice damage and solution effects.

---

## 13. Developmental Biology

Embryonic development and pattern formation.

**Waddington landscape.** Cell fate is modeled as a ball rolling on an epigenetic potential surface $U(x)$. Commitment is: $\text{CI} = 1 - \exp(-|U(x) - U(x_{\text{uncommitted}})|)$, increasing as cells descend into deep valleys.

**Morphogen gradients.** Exponential gradient from a localized source with diffusion $D$ and degradation $\mu$:

$$C(x) = C_0 \, e^{-x/\lambda}, \qquad \lambda = \sqrt{D/\mu}$$

The **French flag model** interprets positional information via threshold concentrations defining distinct cell fates.

**Turing patterns.** Two-component reaction–diffusion generating spatial patterns:

$$\frac{\partial u}{\partial t} = D_u \nabla^2 u + f(u,v), \qquad \frac{\partial v}{\partial t} = D_v \nabla^2 v + g(u,v)$$

Instability requires $D_v \gg D_u$ (long-range inhibition, short-range activation).

**Somitogenesis clock.** Periodic segmentation with period $T_{\text{clock}}$ and wavefront velocity $v_{\text{front}}$ gives somite length $L = v_{\text{front}} \times T_{\text{clock}}$.

---

## 14. Ecology

Community and ecosystem dynamics.

**Lotka–Volterra competition.** Two-species competitive dynamics:

$$\frac{dN_1}{dt} = r_1 N_1 \left(\frac{K_1 - N_1 - \alpha_{12} N_2}{K_1}\right), \qquad \frac{dN_2}{dt} = r_2 N_2 \left(\frac{K_2 - N_2 - \alpha_{21} N_1}{K_2}\right)$$

Coexistence requires $\alpha_{12} < K_1/K_2$ and $\alpha_{21} < K_2/K_1$ (each species more limited by intraspecific competition).

**Predator–prey.** The **Rosenzweig–MacArthur** model with saturating functional response:

$$\frac{dN}{dt} = rN\left(1 - \frac{N}{K}\right) - \frac{aNP}{1 + ahN}, \qquad \frac{dP}{dt} = e \frac{aNP}{1 + ahN} - mP$$

**Functional responses.** Type I: $f = aN$; Type II: $f = aN/(1+ahN)$; Type III: $f = aN^2/(1+ahN^2)$.

**Diversity indices.** Shannon entropy: $H' = -\sum p_i \ln p_i$. Simpson: $D = \sum p_i^2$. Evenness: $J = H'/\ln S$.

---

## 15. Endocrinology

Hormonal regulation and signaling.

**Hormone clearance.** First-order elimination: $C(t) = C_0 e^{-k_e t}$, with half-life $t_{1/2} = \ln 2 / k_e$. Metabolic clearance rate: $\text{MCR} = k_e \cdot V_d$.

**Pulsatile secretion.** Hormones like GnRH are released in pulses. Each pulse is modeled as a bolus: $C(t) = \frac{D}{V_d} e^{-k_e t}$, with interpulse interval $T_p$. The cumulative concentration from multiple pulses is the sum of decaying exponentials.

**Negative feedback.** HPA axis: cortisol suppresses ACTH release. Modeled as: $\frac{d[\text{ACTH}]}{dt} = S_0 / (1 + [\text{Cortisol}]/K_i) - k_d [\text{ACTH}]$.

---

## 16. Enzyme Kinetics

Catalytic mechanisms and regulation.

**Michaelis–Menten kinetics.** The fundamental equation of enzyme kinetics:

$$v = \frac{V_{\max} [S]}{K_m + [S]}$$

where $V_{\max} = k_{\text{cat}} [E]_T$ and $K_m = (k_{-1} + k_{\text{cat}})/k_1$. Catalytic efficiency: $k_{\text{cat}}/K_m$.

**Inhibition modes.** All four modes are implemented:
- **Competitive**: $v = V_{\max}[S] / (K_m(1 + [I]/K_i) + [S])$
- **Uncompetitive**: $v = V_{\max}[S] / (K_m + [S](1 + [I]/K_i'))$
- **Noncompetitive**: $v = V_{\max}[S] / ((K_m + [S])(1 + [I]/K_i))$
- **Mixed**: both $K_i$ and $K_i'$ differ

**Lineweaver–Burk linearization.** $\frac{1}{v} = \frac{K_m}{V_{\max}} \cdot \frac{1}{[S]} + \frac{1}{V_{\max}}$.

**Hill equation.** For cooperative enzymes: $v = V_{\max} [S]^n / (K_{0.5}^n + [S]^n)$. Hill coefficient $n > 1$ indicates positive cooperativity.

---

## 17. Epigenetics

Heritable modifications beyond DNA sequence.

**Histone modification kinetics.** Dynamic balance of writers and erasers:

$$\frac{d[\text{mark}]}{dt} = k_{\text{write}} (1 - [\text{mark}]) - k_{\text{erase}} [\text{mark}]$$

Steady-state mark level: $[\text{mark}]_{\text{eq}} = k_{\text{write}} / (k_{\text{write}} + k_{\text{erase}})$.

**DNA methylation.** CpG methylation dynamics involve maintenance methyltransferase (DNMT1) fidelity and de novo methylation rates. Methylation clocks correlate cumulative CpG changes with biological age.

---

## 18. Ethology

Animal behavior and decision-making.

**Optimal foraging theory.** The **marginal value theorem** determines optimal patch residence time: leave when the marginal intake rate in the current patch drops to the average rate for the habitat:

$$\frac{\partial g(t)}{\partial t} = \frac{g^*}{t^* + \bar{T}}$$

where $g(t)$ is cumulative gain in the patch and $\bar{T}$ is average travel time between patches.

**Optimal diet model.** Prey type $i$ should be included if its profitability $e_i / h_i$ exceeds the average return from higher-ranked items. Energy per item: $e_i$; handling time: $h_i$.

**Ideal free distribution.** Animals distribute across habitats proportionally to resource availability: $n_i / N = r_i / R$, equalizing per-capita intake.

---

## 19. Evolution

Evolutionary processes at the molecular and population level.

**Molecular clock.** Substitution rate: $r = k / (2t)$, where $k$ is the observed substitutions and $t$ the divergence time. For protein-coding genes, the **dN/dS ratio** ($\omega$) distinguishes selection regimes:
- $\omega < 1$: purifying selection
- $\omega = 1$: neutral evolution
- $\omega > 1$: positive selection

**Coalescent theory.** Expected coalescence time for two lineages in a population of effective size $N_e$: $E[T_2] = 2 N_e$ generations. Neutral diversity: $\pi = 4 N_e \mu$.

**Watterson's theta.** Estimated from the number of segregating sites $S$:

$$\hat{\theta}_W = \frac{S}{\sum_{i=1}^{n-1} 1/i}$$

**Fixation probability.** For a new mutation with selection coefficient $s$ in a diploid population: $P_{\text{fix}} \approx 2s$ (Haldane's approximation for beneficial mutations).

---

## 20. Genetics

Classical and molecular genetics.

**Hardy–Weinberg equilibrium.** Allele frequencies remain constant in the absence of evolutionary forces. For alleles $p$ and $q = 1-p$: genotype frequencies $p^2 : 2pq : q^2$.

**F-statistics.** Wright's $F_{ST}$ measures population differentiation:

$$F_{ST} = \frac{H_T - H_S}{H_T}$$

where $H_T$ is total expected heterozygosity and $H_S$ the mean within-subpopulation heterozygosity.

**Nucleotide diversity.** $\pi = \frac{n}{n-1} \sum_{i<j} 2 p_i p_j d_{ij}$, the average number of pairwise differences per site.

**Watterson's theta** from segregating sites provides an alternative estimate of $4 N_e \mu$.

---

## 21. Genomics

Genome-scale analysis.

**Gene density.** Genes per megabase: $\rho = n_{\text{genes}} / (L_{\text{genome}} / 10^6)$.

**CpG enrichment.** Observed/expected CpG ratio:

$$\text{CpG}_{o/e} = \frac{N_{\text{CpG}} \cdot L}{N_C \cdot N_G}$$

Ratios > 0.6 indicate CpG islands (associated with promoter regions).

**Codon adaptation index (CAI).** Measures synonymous codon usage bias relative to a reference set of highly expressed genes:

$$\text{CAI} = \exp\!\left(\frac{1}{L} \sum_{i=1}^{L} \ln w_i\right)$$

where $w_i$ is the relative adaptiveness of the $i$-th codon. High CAI correlates with high expression.

---

## 22. Immunology

Immune system modeling.

**Somatic hypermutation and affinity maturation.** B-cell receptor affinity evolves through iterative mutation and selection in germinal centers. Affinity after $n$ rounds: $A_n = A_0 (1 + \delta)^n$, where $\delta$ is the mean fractional improvement per round.

**Antibody titer.** Concentration doubling during acute response: $T(t) = T_0 \cdot 2^{t / t_d}$ during the expansion phase, followed by exponential decay: $T(t) = T_{\text{peak}} e^{-k t}$.

**Neutralization.** The fraction of pathogen neutralized: $f = \frac{[Ab]^n}{K_d^n + [Ab]^n}$ (Hill-type dose–response).

**Isotype switching.** Class-switch recombination probabilities depend on cytokine milieu signals (IL-4 → IgE, IFN-γ → IgG).

---

## 23. Marine Biology

Ocean ecosystems and organisms.

**Phytoplankton growth.** Nutrient-limited growth with Monod kinetics:

$$\mu = \mu_{\max} \frac{N}{K_N + N}$$

with extensions for light limitation (Smith function) and temperature dependence (Eppley curve: $\mu_{\max} = 0.59 e^{0.0633 T}$).

**Sverdrup critical depth.** A spring bloom occurs when the mixed layer depth $Z_m$ is shallower than the critical depth $Z_{cr}$:

$$Z_{cr} = \frac{I_0 (1 - e^{-k Z_{cr}})}{k \cdot I_c}$$

where $I_c$ is the compensation irradiance and $k$ the extinction coefficient.

**NPZ model.** Nutrient–Phytoplankton–Zooplankton coupled ODEs:

$$\frac{dN}{dt} = -\mu P + \gamma Z + m P, \quad \frac{dP}{dt} = \mu P - g P Z - m P, \quad \frac{dZ}{dt} = g P Z - \gamma Z$$

---

## 24. Microbiology

Microbial growth and interactions.

**Quorum sensing.** Autoinducer (AHL) dynamics:

$$\frac{d[\text{AHL}]}{dt} = k_{\text{prod}} \cdot n_{\text{cells}} - k_{\text{deg}} [\text{AHL}]$$

When $[\text{AHL}]$ exceeds a threshold $K_T$, target gene activation is: $f = [\text{AHL}]^h / (K_T^h + [\text{AHL}]^h)$ (Hill function).

**Monod growth kinetics.** $\mu = \mu_{\max} S / (K_s + S)$, where $S$ is the limiting substrate and $K_s$ the half-saturation constant.

**Chemostat steady state.** Dilution rate $D$ sets the wash-out threshold: at steady state, $\mu = D$, giving $S^* = K_s D / (\mu_{\max} - D)$.

---

## 25. Mycology

Fungal biology and ecology.

**Saprotrophic decomposition.** First-order exponential decay of substrate: $M(t) = M_0 e^{-k t}$, with carbon release rate $\dot{C} = k \cdot M(t)$. Decomposition rate constants $k$ vary with temperature (Q₁₀ models) and moisture.

**Spore germination.** Germination percentage as a function of time: $G(t) = G_{\max}(1 - e^{-r t})$. Optimal conditions are moisture > threshold and temperature in a species-specific range.

**Hyphal growth.** Colony radius expansion follows linear kinetics: $r(t) = r_0 + v t$. Branching frequency determines mycelial density.

---

## 26. Neuroscience

Computational neuroscience and neural systems.

**Spike analysis.** Mean firing rate: $\bar{r} = n_{\text{spikes}} / T$. Interspike interval (ISI) distribution characterizes regularity. The **Fano factor** measures spike count variability:

$$F = \frac{\text{Var}(N)}{\langle N \rangle}$$

$F = 1$ for Poisson spiking; $F < 1$ indicates regularity; $F > 1$ clustering.

**Coefficient of variation** of ISI: $\text{CV} = \sigma_{\text{ISI}} / \mu_{\text{ISI}}$.

**Integrate-and-fire.** $\tau_m \frac{dV}{dt} = -(V - V_{\text{rest}}) + R I_{\text{ext}}$. Spike when $V = V_{\text{threshold}}$; reset to $V_{\text{reset}}$.

---

## 27. Nutrition

Nutrient metabolism and dietary analysis.

**First-order absorption.** Plasma concentration after oral intake:

$$C(t) = \frac{D \cdot k_a}{V_d (k_a - k_e)} (e^{-k_e t} - e^{-k_a t})$$

where $k_a$ is the absorption rate, $k_e$ elimination rate, and $D$ the dose.

**Gastric emptying.** Exponential emptying: $V(t) = V_0 e^{-k_g t}$ for liquids; power-exponential for solids.

**Glycemic index.** $\text{GI} = (\text{AUC}_{\text{food}} / \text{AUC}_{\text{reference}}) \times 100$. Glycemic load: $\text{GL} = \text{GI} \times g_{\text{carb}} / 100$.

---

## 28. Paleobiology

Ancient life and deep time.

**Background extinction rate.** Expressed in extinctions per million species-years (E/MSY):

$$\dot{E}_{\text{bg}} = \frac{\text{extinctions}}{N \cdot \Delta t}$$

**Mass extinction magnitude.** Kill curve: percentage of taxa lost. $M = 1 - S_{\text{after}} / S_{\text{before}}$ where $S$ is species richness. The "Big Five" events exceeded 75% genus-level extinction.

**Origination rate.** $\hat{o} = \ln(N_{t+1} / N_t + E) / \Delta t$, accounting for both new appearances and losses.

---

## 29. Parasitology

Host–parasite interactions.

**Anderson–May model.** Macroparasite dynamics with mean burden $P$ in host population $H$:

$$\frac{dH}{dt} = (b - d) H - \alpha P, \qquad \frac{dP}{dt} = \beta H - (\mu + d + \alpha) P - \alpha \frac{k+1}{k} \frac{P^2}{H}$$

where $k$ is the negative binomial aggregation parameter (lower $k$ = more aggregated parasites).

**Negative binomial distribution.** Parasite burdens are overdispersed: $\text{Var} = \mu (1 + \mu/k)$. Proportion of hosts with > $n$ parasites: $P(X > n) = 1 - \text{NB}(n; k, \mu)$.

---

## 30. Pharmacology

Drug action and disposition.

**Oral bioavailability.** $F = f_{\text{abs}} \cdot (1 - E_{\text{gut}}) \cdot (1 - E_{\text{hepatic}})$, where $E$ represents extraction ratios.

**Noyes–Whitney dissolution.** $\frac{dm}{dt} = D \frac{A}{h}(C_s - C_t)$, where $A$ is surface area, $h$ diffusion layer thickness, $C_s$ solubility, $C_t$ bulk concentration.

**Hepatic clearance.** The well-stirred model: $CL_{\text{H}} = \frac{Q_H \cdot f_u \cdot CL_{\text{int}}}{Q_H + f_u \cdot CL_{\text{int}}}$, where $Q_H$ is hepatic blood flow, $f_u$ the unbound fraction, and $CL_{\text{int}}$ the intrinsic clearance.

**One-compartment PK.** IV bolus: $C(t) = (D/V_d) e^{-k_e t}$. Half-life: $t_{1/2} = 0.693 / k_e$. AUC: $D / (V_d \cdot k_e)$.

---

## 31. Phylogenetics

Evolutionary relationships and tree construction.

**Jukes–Cantor distance.** Corrects for multiple substitutions at the same site:

$$d = -\frac{3}{4} \ln\!\left(1 - \frac{4p}{3}\right)$$

where $p$ is the observed fraction of differing sites.

**Kimura 2-parameter.** Separates transitions ($P$) and transversions ($Q$):

$$d = -\frac{1}{2} \ln(1 - 2P - Q) - \frac{1}{4} \ln(1 - 2Q)$$

**Tree construction.** Distance matrices feed into neighbor-joining or UPGMA algorithms. Likelihood methods evaluate tree topologies using substitution model parameters.

---

## 32. Physiology

Organ system function modeling.

**Frank–Starling mechanism.** Stroke volume increases with end-diastolic volume:

$$SV = SV_{\max} \cdot (1 - e^{-k \cdot V_{\text{ED}}})$$

**Cardiac output.** $CO = HR \times SV$. Ejection fraction: $EF = SV / V_{\text{ED}}$.

**Mean arterial pressure.** $\text{MAP} = \text{DBP} + \frac{1}{3}(\text{SBP} - \text{DBP})$. Vascular resistance: $R = \text{MAP} / CO$.

**Renal function.** GFR estimation, tubular reabsorption rates, and clearance calculations.

**Oxygen dissociation.** Hill equation for hemoglobin: $Y = \frac{pO_2^n}{P_{50}^n + pO_2^n}$ with $n \approx 2.8$ and $P_{50} \approx 26.6$ mmHg.

---

## 33. Plant Biology

Plant physiology and ecology.

**Tilman competition.** Two species competing for a single resource: the species with the lower $R^*$ (minimum resource requirement) wins: $R^* = K_s \cdot m / (\mu_{\max} - m)$.

**Allelopathy.** Chemical inhibition between plants: $G = G_0 e^{-\alpha \cdot [\text{toxin}]}$.

**Canopy light extinction.** Beer–Lambert law through leaf layers:

$$I(L) = I_0 \, e^{-k \cdot \text{LAI}}$$

where $\text{LAI}$ is leaf area index and $k$ the extinction coefficient (~0.5 for broadleaf).

---

## 34. Population Biology

Population dynamics and structure.

**Logistic growth.** Density-dependent growth:

$$\frac{dN}{dt} = rN\left(1 - \frac{N}{K}\right)$$

Solution: $N(t) = K / (1 + (K/N_0 - 1)e^{-rt})$.

**SIR epidemiological model.** Compartmental disease dynamics solved with RK4:

$$\frac{dS}{dt} = -\frac{\beta S I}{N}, \quad \frac{dI}{dt} = \frac{\beta S I}{N} - \gamma I, \quad \frac{dR}{dt} = \gamma I$$

Basic reproduction number: $R_0 = \beta / \gamma$. Herd immunity threshold: $1 - 1/R_0$. SciForge also implements **SEIR** (with exposed compartment, latency rate $\sigma$).

**Leslie matrix.** Age-structured population projection: $\mathbf{n}_{t+1} = \mathbf{L} \cdot \mathbf{n}_t$, where $\mathbf{L}$ contains fecundities in the first row and survival probabilities on the subdiagonal. The dominant eigenvalue $\lambda$ gives the asymptotic growth rate.

---

## 35. Proteomics

Protein-level analysis.

**Peptide molecular weight.** Computed from amino acid residue masses:

$$M_{\text{peptide}} = \sum_i n_i \cdot M_{\text{residue},i} + M_{\text{H}_2\text{O}}$$

where the water molecule accounts for the uncondensed termini. SciForge provides a complete table of 20 amino acid average and monoisotopic masses (e.g., Gly = 57.021 Da, Trp = 186.079 Da).

**Isoelectric point.** Estimated by finding the pH at which net charge is zero, summing Henderson–Hasselbalch contributions from all ionizable residues.

---

## 36. Radiobiology

Radiation effects on biological systems.

**DNA damage.** Single-strand break probability per gray: $P_{\text{SSB}} = 1 - e^{-\alpha_{\text{SSB}} D}$. DSB probability: $P_{\text{DSB}} = 1 - e^{-\alpha_{\text{DSB}} D}$. DSBs are the critical lethal lesion.

**Chromosome aberrations.** Dicentric yield: $Y = \alpha D + \beta D^2$ (linear-quadratic model). This underlies biological dosimetry.

**Linear-quadratic cell survival.** $S(D) = e^{-\alpha D - \beta D^2}$, with $\alpha/\beta$ ratio distinguishing early-responding ($\alpha/\beta \approx 10$ Gy) from late-responding ($\alpha/\beta \approx 3$ Gy) tissues.

---

## 37. Reproduction

Reproductive biology and embryogenesis.

**Cleavage timing.** Embryonic cell count at time $t$: $N(t) = 2^{\lfloor t / T_{\text{cycle}} \rfloor}$, where $T_{\text{cycle}}$ is the cleavage cycle duration (~12–24 hours in mammals).

**Morphogen gradients.** Exponential decay from a source (see Developmental Biology). Threshold-dependent cell fate commitment determines tissue patterning.

**Somitogenesis clock.** Periodic gene expression drives segmentation with period $T$. Somite size $= v_{\text{wavefront}} \times T$.

---

## 38. Stem Cell Biology

Stem cell dynamics and engineering.

**Waddington potential.** The multipotency index quantifies a cell's distance from committed states:

$$\text{MI} = 1 - \frac{|U(x) - U_{\text{peak}}|}{U_{\text{peak}} - U_{\text{valley}}}$$

$\text{MI} = 1$ at the hilltop (fully multipotent); $\text{MI} = 0$ at a valley (fully committed).

**Differentiation commitment.** Commitment index: $\text{CI} = 1 - \exp(-|U(x) - U_{\text{uncommitted}}|)$.

**Self-renewal vs. differentiation.** Balance determines stem cell pool maintenance. Symmetric division: two stem cells or two differentiated cells. Asymmetric: one of each.

---

## 39. Structural Biology

Macromolecular structure analysis.

**3D molecular geometry.** Bond angles from three atomic positions using the dot product:

$$\theta = \arccos\!\left(\frac{\vec{BA} \cdot \vec{BC}}{|\vec{BA}||\vec{BC}|}\right)$$

Dihedral (torsion) angles from four atoms using the cross-product method. **Ramachandran plots** map backbone $\phi/\psi$ angles to identify allowed conformational regions.

**RMSD.** Root mean square deviation between two structures: $\text{RMSD} = \sqrt{\frac{1}{N}\sum_{i=1}^{N} d_i^2}$.

---

## 40. Synthetic Biology

Engineered biological systems.

**Toggle switch.** Gardner's bistable genetic circuit with mutual repression:

$$\frac{du}{dt} = \frac{\alpha_1}{1 + v^n} - \beta u, \qquad \frac{dv}{dt} = \frac{\alpha_2}{1 + u^n} - \gamma v$$

Two stable steady states exist when $n > 1$ and $\alpha_1, \alpha_2$ are sufficiently large relative to $\beta, \gamma$.

**Repressilator.** Three-gene ring oscillator (Elowitz & Leibler):

$$\frac{dx_i}{dt} = \frac{\alpha}{1 + x_{(i+2)\bmod 3}^n} + \alpha_0 - \beta x_i$$

Sustained oscillations emerge for appropriate parameter regimes.

**CRISPR scoring.** On-target efficiency depends on GC content, position-specific scores, and PAM strength: $S = S_{\text{pos}} \times f_{\text{GC}} \times \text{PAM}$. Off-target CFD (cutting frequency determination) score: $\text{CFD} = \prod (1 - p_{\text{mismatch},i})$.

**Metabolic engineering.** The **flux balance objective** maximizes a linear function: $Z = \mathbf{c}^T \mathbf{v}$ subject to stoichiometric constraints $\mathbf{S} \mathbf{v} = 0$. Theoretical yield: $Y = (\text{substrate carbons} / \text{product carbons}) \times \eta$.

**Stochastic gene expression.** Mean protein level: $\langle P \rangle = (k_{\text{txn}} k_{\text{tln}}) / (\delta_m \delta_p)$. Fano factor: $F = 1 + b$, where burst size $b = k_{\text{tln}} / (\delta_m + \delta_p)$. Intrinsic noise: $\eta^2_{\text{int}} = (1+b)/\langle P \rangle$.

---

## 41. Systems Biology

Whole-system modeling of biological networks.

**Bistability.** Toggle switch dynamics (same formalism as synthetic biology) produce hysteresis and ultrasensitivity in signaling cascades.

**Flux balance analysis (FBA).** Steady-state metabolic flux through stoichiometric matrix $\mathbf{S}$: $\mathbf{S} \mathbf{v} = 0$. Feasibility requires all metabolic production/consumption rates to balance within tolerance.

**Boolean networks.** Gene regulatory networks with discrete ON/OFF states. State update: $x_i(t+1) = f_i(x_{j_1}(t), x_{j_2}(t), \ldots)$ where $f_i$ is a Boolean rule. Attractor analysis reveals stable cell states.

**Goodwin oscillator.** Negative feedback loop with cooperative repression (Hill coefficient $n$). Simulated with Euler integration. The **repressilator** extends this to three-gene ring topology.

**Sensitivity analysis.** Local sensitivity: normalized partial derivative. **Morris method**: elementary effects screening. **Sobol indices**: first-order $S_i = V_i / V_T$, total-order $S_{Ti} = 1 - V_{-i}/V_T$, and partial rank correlation coefficient (PRCC) for monotonic relationships.

---

## 42. Tissue Engineering

Engineering biological tissues.

**Bioreactor design.** Perfusion rate: $D = Q / V$. Shear stress in parallel-plate bioreactor:

$$\tau = \frac{6 \mu Q}{w h^2}$$

where $\mu$ is viscosity, $Q$ flow rate, $w$ width, $h$ height. Oxygen transfer rate: $\text{OTR} = k_L a \cdot (C^* - C)$.

**Scaffold degradation.** Bulk erosion: $M(t) = M_0 e^{-kt}$. Surface erosion: $M(t) = M_0 - r \cdot A \cdot t$. Porosity: $\phi = V_{\text{void}} / V_{\text{total}}$. Mechanical stiffness degrades as $E(t) = E_0 (1 - f_d)^n$.

**Krogh cylinder.** Oxygen distribution around a capillary:

$$pO_2(r) = pO_{2,\text{cap}} - \frac{Q_0}{4D}(r^2 - r_c^2 - 2r_c^2 \ln(r/r_c))$$

Nutrient penetration depth: $L = \sqrt{2 D C_s / Q_0}$, limiting maximum engineered tissue thickness without vascularization.

**Vascularization.** VEGF concentration follows exponential decay from source: $C(x) = C_0 e^{-x/\lambda}$. Sprouting probability: Hill function of VEGF level. Optimal vessel spacing: $2\sqrt{2 D \cdot pO_{2,a} / Q_0}$.

---

## 43. Toxicology

Effects of toxic substances.

**Bioconcentration factor (BCF).** $\text{BCF} = C_{\text{organism}} / C_{\text{water}}$. **Biomagnification factor**: $\text{BMF} = C_{\text{predator}} / C_{\text{prey}}$. One-compartment toxicokinetics:

$$\frac{dC}{dt} = k_{\text{uptake}} \cdot C_{\text{exposure}} - k_{\text{elim}} \cdot C$$

**LD50 estimation.** Probit analysis: transform dose–response data via probit($p$) vs. $\ln(\text{dose})$, then linear regression to find the dose at probit = 5 (50% mortality).

**Dose–response.** Hill model: $E = E_{\max} D^n / (EC_{50}^n + D^n)$. Benchmark dose (BMD): the dose producing a specified benchmark response above background.

**Oxidative stress mechanisms.** Oxidative stress index: $\text{OSI} = \text{ROS production} / \text{antioxidant capacity}$. DNA adduct formation: $\dot{A} = k [M][DNA]$. Lipid peroxidation and protein carbonylation follow second-order kinetics.

**Ecotoxicology.** Species sensitivity distributions (SSD) rank species NOEC/EC50 values to derive protective concentrations (HC5 = hazardous concentration for 5% of species).

---

## 44. Virology

Virus biology and dynamics.

**Target-cell limited model.** Within-host viral dynamics:

$$\frac{dx}{dt} = -\beta x v, \qquad \frac{dy}{dt} = \beta x v - \delta y, \qquad \frac{dv}{dt} = p y - c v$$

where $x$ = uninfected target cells, $y$ = infected cells, $v$ = free virus, $\beta$ = infection rate, $\delta$ = infected cell death, $p$ = virus production, $c$ = clearance.

**Basic reproduction number.** $R_0 = \frac{\beta p x_0}{\delta c}$. Infection establishes when $R_0 > 1$. Burst size: $N = p / \delta$.

**Viral load decay.** Under therapy: $v(t) = v_0 e^{-ct}$, with half-life $t_{1/2} = \ln 2 / c$ (typically 6 hours for HIV).

**Quasispecies theory.** Fitness of the master sequence with mutation rate $\mu$ per base and genome length $L$:

$$W = w_0 (1 - \mu)^L$$

**Error threshold.** Maximum mutation rate compatible with selection: $\mu_{\max} = 1 - (1/\sigma)^{1/L}$ where $\sigma$ is the superiority of the master sequence.

**Epidemiology.** SIR and SEIR models (see Population Biology). Epidemic doubling time, generation time, and serial interval are computed for emerging pathogens.

**Capsid geometry.** Caspar–Klug theory: T-number triangulation gives $60T$ subunits per icosahedral capsid. Capsid radius: $R = \sqrt{N \cdot A_{\text{sub}} / (4\pi)}$. Genome packaging density: $\rho = L_{\text{genome}} / V_{\text{capsid}}$.

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
