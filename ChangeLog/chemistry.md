# Chemistry — ChangeLog

---

## v0.0.3

No new functions — see `testing.md` for test details.

---

## v0.0.2

### Changes

| Metric | Value |
|---|---|
| Submodules rewritten | 26 / 26 |

---

## v0.0.1

### 1️⃣ Kinetics — `chemistry::kinetics` — 24 pub fn

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| Arrhenius rate constant | `rate_constant_arrhenius(a: f64, ea: f64, t: f64) → f64` | $k = A e^{-E_a / RT}$ | `kinetics::rates` |
| First-order half-life | `half_life_first_order(k: f64) → f64` | $t_{1/2} = \ln 2 / k$ | `kinetics::rates` |
| First-order concentration | `concentration_first_order(c0: f64, k: f64, t: f64) → f64` | $[A] = [A]_0 e^{-kt}$ | `kinetics::rates` |
| Second-order concentration | `concentration_second_order(c0: f64, k: f64, t: f64) → f64` | $1/[A] = 1/[A]_0 + kt$ | `kinetics::rates` |
| Zero-order concentration | `concentration_zero_order(c0: f64, k: f64, t: f64) → f64` | $[A] = [A]_0 - kt$ | `kinetics::rates` |
| Rate law | `rate_law(k: f64, concentrations: &[f64], orders: &[f64]) → f64` | $r = k \prod [A_i]^{n_i}$ | `kinetics::rates` |
| Activation energy (two temps) | `activation_energy_two_temps(k1: f64, k2: f64, t1: f64, t2: f64) → f64` | $E_a = R \ln(k_2/k_1) / (1/T_1 - 1/T_2)$ | `kinetics::rates` |
| Second-order half-life | `half_life_second_order(k: f64, c0: f64) → f64` | $t_{1/2} = 1/(k[A]_0)$ | `kinetics::rates` |
| Zero-order half-life | `half_life_zero_order(k: f64, c0: f64) → f64` | $t_{1/2} = [A]_0 / 2k$ | `kinetics::rates` |
| Nth-order integrated rate | `integrated_rate_law_nth(c0: f64, k: f64, t: f64, n: f64) → f64` | $[A]^{1-n} = [A]_0^{1-n} + (n-1)kt$ | `kinetics::rates` |
| Eyring equation | `eyring_equation(kappa: f64, kb: f64, t: f64, h: f64, delta_g_dagger: f64) → f64` | $k = \kappa \frac{k_B T}{h} e^{-\Delta G^\ddagger / RT}$ | `kinetics::rates` |
| Collision frequency | `collision_frequency(na: f64, nb: f64, sigma: f64, t: f64, mu: f64) → f64` | $Z_{AB} = N_A N_B \sigma \sqrt{8k_BT/\pi\mu}$ | `kinetics::rates` |
| Michaelis-Menten | `michaelis_menten(vmax: f64, km: f64, s: f64) → f64` | $v = V_{max}[S]/(K_m + [S])$ | `kinetics::mechanisms` |
| Lineweaver-Burk slope | `lineweaver_burk_slope(km: f64, vmax: f64) → f64` | $K_m / V_{max}$ | `kinetics::mechanisms` |
| Lineweaver-Burk intercept | `lineweaver_burk_intercept(vmax: f64) → f64` | $1 / V_{max}$ | `kinetics::mechanisms` |
| Lindemann unimolecular | `lindemann_unimolecular(k_inf: f64, k0: f64, m: f64) → f64` | $k_{uni} = k_\infty k_0[M] / (k_\infty + k_0[M])$ | `kinetics::mechanisms` |
| Consecutive reaction | `consecutive_reaction(c0: f64, k1: f64, k2: f64, t: f64) → (f64, f64, f64)` | $A \xrightarrow{k_1} B \xrightarrow{k_2} C$ | `kinetics::mechanisms` |
| Reversible first-order | `reversible_first_order(c0: f64, kf: f64, kr: f64, t: f64) → (f64, f64)` | $A \rightleftharpoons B$ | `kinetics::mechanisms` |
| Parallel reactions | `parallel_reactions(c0: f64, k_values: &[f64], t: f64) → Vec<f64>` | $A \xrightarrow{k_i} P_i$ | `kinetics::mechanisms` |
| Steady-state intermediate | `steady_state_intermediate(k1: f64, k_minus1: f64, k2: f64, a: f64) → f64` | $[I]_{ss} = k_1[A]/(k_{-1}+k_2)$ | `kinetics::mechanisms` |
| Pre-equilibrium rate | `pre_equilibrium_rate(k1: f64, k_minus1: f64, k2: f64, a: f64, b: f64) → f64` | $r = k_2 K_{eq}[A][B]$ | `kinetics::mechanisms` |
| Catalytic efficiency | `catalytic_efficiency(kcat: f64, km: f64) → f64` | $k_{cat}/K_m$ | `kinetics::mechanisms` |
| Competitive inhibition | `competitive_inhibition(vmax: f64, km: f64, s: f64, i: f64, ki: f64) → f64` | $v = V_{max}[S]/(K_m(1+[I]/K_i)+[S])$ | `kinetics::mechanisms` |
| Uncompetitive inhibition | `uncompetitive_inhibition(vmax: f64, km: f64, s: f64, i: f64, ki: f64) → f64` | $v = V_{max}[S]/(K_m+[S](1+[I]/K_i))$ | `kinetics::mechanisms` |

### 2️⃣ Equilibrium — `chemistry::equilibrium` — 18 pub fn

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| Keq from Gibbs | `equilibrium_constant_from_gibbs(delta_g: f64, t: f64) → f64` | $K = e^{-\Delta G / RT}$ | `equilibrium::constants` |
| Reaction quotient | `reaction_quotient(products: &[(f64, f64)], reactants: &[(f64, f64)]) → f64` | $Q = \prod [P_i]^{p_i} / \prod [R_j]^{r_j}$ | `equilibrium::constants` |
| Le Chatelier shift | `le_chatelier_shift(q: f64, keq: f64) → i32` | $Q < K$ → forward, $Q > K$ → reverse | `equilibrium::constants` |
| Kp from Kc | `kp_from_kc(kc: f64, t: f64, delta_n: f64) → f64` | $K_p = K_c (RT)^{\Delta n}$ | `equilibrium::constants` |
| Van 't Hoff | `vant_hoff(k1: f64, delta_h: f64, t1: f64, t2: f64) → f64` | $\ln(K_2/K_1) = -\Delta H/R (1/T_2 - 1/T_1)$ | `equilibrium::constants` |
| Degree of dissociation | `degree_of_dissociation(keq: f64, c0: f64) → f64` | $\alpha = \sqrt{K_{eq}/c_0}$ | `equilibrium::constants` |
| Keq temperature dependence | `temperature_dependence_keq(k_ref: f64, delta_h: f64, t_ref: f64, t: f64) → f64` | Van 't Hoff temperature extrapolation | `equilibrium::constants` |
| Gibbs from Keq | `gibbs_from_keq(keq: f64, t: f64) → f64` | $\Delta G = -RT \ln K$ | `equilibrium::constants` |
| Pressure effect on Keq | `pressure_effect_on_keq(keq: f64, delta_v: f64, p1: f64, p2: f64, t: f64) → f64` | $K(P_2) = K(P_1) e^{-\Delta V(P_2-P_1)/RT}$ | `equilibrium::constants` |
| Common ion effect | `common_ion_effect(ksp: f64, common_ion_conc: f64, stoich_coeff: f64) → f64` | Solubility shift by common ion | `equilibrium::ionic` |
| Buffer capacity | `buffer_capacity(ca: f64, cb: f64, ka: f64, h: f64) → f64` | $\beta = 2.303(C_a K_a [H^+]/(K_a+[H^+])^2)$ | `equilibrium::ionic` |
| Weak acid pH | `ph_weak_acid(ka: f64, c: f64) → f64` | $\text{pH} = \frac{1}{2}(pK_a - \log c)$ | `equilibrium::ionic` |
| Buffer pH | `ph_buffer(ka: f64, acid: f64, base: f64) → f64` | $\text{pH} = pK_a + \log([A^-]/[HA])$ | `equilibrium::ionic` |
| Solubility product | `solubility_product(ion_concentrations: &[(f64, f64)]) → f64` | $K_{sp} = \prod [ion_i]^{n_i}$ | `equilibrium::ionic` |
| Distribution coefficient | `distribution_coefficient(c_organic: f64, c_aqueous: f64) → f64` | $K_D = C_{org}/C_{aq}$ | `equilibrium::ionic` |
| Solubility from Ksp | `solubility_from_ksp(ksp: f64, cation_stoich: f64, anion_stoich: f64) → f64` | $s = (K_{sp}/(m^m n^n))^{1/(m+n)}$ | `equilibrium::ionic` |
| Formation constant | `formation_constant(product_conc: f64, metal_conc: f64, ligand_conc: f64, n: f64) → f64` | $K_f = [ML_n]/([M][L]^n)$ | `equilibrium::ionic` |
| Conditional formation constant | `conditional_formation_constant(kf: f64, alpha_y: f64) → f64` | $K_f' = K_f / \alpha_Y$ | `equilibrium::ionic` |

### 3️⃣ Electrochemistry — `chemistry::electrochemistry` — 20 pub fn

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| Nernst potential | `nernst_potential(e0: f64, n: f64, q: f64, t: f64) → f64` | $E = E^0 - \frac{RT}{nF}\ln Q$ | `electrochemistry::cells` |
| Cell potential | `cell_potential(e_cathode: f64, e_anode: f64) → f64` | $E_{cell} = E_{cathode} - E_{anode}$ | `electrochemistry::cells` |
| Gibbs from cell potential | `gibbs_from_cell_potential(n: f64, e_cell: f64) → f64` | $\Delta G = -nFE$ | `electrochemistry::cells` |
| Faraday mass | `faraday_mass(i: f64, t: f64, m: f64, n: f64) → f64` | $m = MIt/nF$ | `electrochemistry::cells` |
| Tafel overpotential | `overpotential_tafel(a: f64, b: f64, current_density: f64) → f64` | $\eta = a + b \log j$ | `electrochemistry::cells` |
| Butler-Volmer | `butler_volmer(i0: f64, alpha_a: f64, alpha_c: f64, eta: f64, t: f64) → f64` | $j = j_0[e^{\alpha_a F\eta/RT} - e^{-\alpha_c F\eta/RT}]$ | `electrochemistry::cells` |
| Open circuit voltage | `open_circuit_voltage(e_cathode: f64, e_anode: f64, n_electrons: f64, t: f64, q: f64) → f64` | OCV with Nernst correction | `electrochemistry::cells` |
| Faradaic efficiency | `faradaic_efficiency(actual_yield: f64, theoretical_yield: f64) → f64` | $\eta_F = m_{actual}/m_{theoretical}$ | `electrochemistry::cells` |
| Battery energy density | `energy_density_battery(voltage: f64, capacity_ah: f64, mass_kg: f64) → f64` | $E_d = V \cdot C / m$ (Wh/kg) | `electrochemistry::cells` |
| Coulombic efficiency | `coulombic_efficiency(discharge_capacity: f64, charge_capacity: f64) → f64` | $\eta_C = Q_{discharge}/Q_{charge}$ | `electrochemistry::cells` |
| Conductivity | `conductivity(resistance: f64, cell_constant: f64) → f64` | $\kappa = K_{cell}/R$ | `electrochemistry::transport` |
| Molar conductivity | `molar_conductivity(conductivity: f64, concentration: f64) → f64` | $\Lambda_m = \kappa / c$ | `electrochemistry::transport` |
| Kohlrausch law | `kohlrausch(lambda_inf: f64, k: f64, c: f64) → f64` | $\Lambda_m = \Lambda_m^\infty - K\sqrt{c}$ | `electrochemistry::transport` |
| Debye-Hückel activity | `debye_huckel_activity(z: f64, ionic_strength: f64) → f64` | $\log \gamma = -Az^2\sqrt{I}$ | `electrochemistry::transport` |
| Ionic strength | `ionic_strength(ions: &[(f64, f64)]) → f64` | $I = \frac{1}{2}\sum c_i z_i^2$ | `electrochemistry::transport` |
| Transference number | `transference_number(lambda_ion: f64, lambda_total: f64) → f64` | $t_i = \lambda_i / \Lambda$ | `electrochemistry::transport` |
| Extended Debye-Hückel | `debye_huckel_extended(z: f64, ionic_strength: f64, a_ion: f64) → f64` | $\log \gamma = -Az^2\sqrt{I}/(1+Ba\sqrt{I})$ | `electrochemistry::transport` |
| Onsager equation | `onsager_equation(lambda_inf: f64, a: f64, b: f64, c: f64) → f64` | $\Lambda = \Lambda^\infty - (A+B\Lambda^\infty)\sqrt{c}$ | `electrochemistry::transport` |
| Walden product | `walden_product(viscosity: f64, molar_conductivity: f64) → f64` | $\eta \Lambda_m = \text{const}$ | `electrochemistry::transport` |
| Mobility from conductivity | `mobility_from_conductivity(lambda_ion: f64, z: f64) → f64` | $u = \lambda / (zF)$ | `electrochemistry::transport` |

### 4️⃣ Thermochemistry — `chemistry::thermochemistry` — 21 pub fn

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| Hess's law | `hess_law(enthalpies: &[f64], coefficients: &[f64]) → f64` | $\Delta H = \sum \nu_i \Delta H_i$ | `thermochemistry::enthalpy` |
| Bond enthalpy | `bond_enthalpy(bonds_broken: &[f64], bonds_formed: &[f64]) → f64` | $\Delta H = \sum D_{broken} - \sum D_{formed}$ | `thermochemistry::enthalpy` |
| Born-Haber lattice energy | `born_haber_lattice_energy(sublimation: f64, ionization: f64, dissociation: f64, electron_affinity: f64, formation: f64) → f64` | Born-Haber cycle calculation | `thermochemistry::enthalpy` |
| Calorimetry | `calorimetry(mass: f64, specific_heat: f64, delta_t: f64) → f64` | $q = mc\Delta T$ | `thermochemistry::enthalpy` |
| Kirchhoff equation | `kirchhoff(delta_h_t1: f64, delta_cp: f64, t1: f64, t2: f64) → f64` | $\Delta H(T_2) = \Delta H(T_1) + \Delta C_p(T_2-T_1)$ | `thermochemistry::enthalpy` |
| Heat of combustion | `heat_of_combustion(n_co2: f64, n_h2o: f64, hf_compound: f64) → f64` | $\Delta H_c$ from formation enthalpies | `thermochemistry::enthalpy` |
| Clausius-Clapeyron | `clausius_clapeyron(p1: f64, delta_h_vap: f64, t1: f64, t2: f64) → f64` | $\ln(P_2/P_1) = -\Delta H_{vap}/R(1/T_2-1/T_1)$ | `thermochemistry::enthalpy` |
| Heat capacity ratio | `heat_capacity_ratio(cp: f64, cv: f64) → f64` | $\gamma = C_p/C_v$ | `thermochemistry::enthalpy` |
| Adiabatic flame temperature | `adiabatic_flame_temperature(reactant_enthalpy: f64, product_enthalpy_base: f64, cp_products: f64, t_base: f64) → f64` | $T_{ad} = T_{base} + \Delta H/C_p$ | `thermochemistry::enthalpy` |
| Standard enthalpy of reaction | `standard_enthalpy_of_reaction(products_hf: &[f64], products_coeff: &[f64], reactants_hf: &[f64], reactants_coeff: &[f64]) → f64` | $\Delta H^\circ = \sum \nu \Delta H_f^\circ(prod) - \sum \nu \Delta H_f^\circ(react)$ | `thermochemistry::enthalpy` |
| Bomb calorimeter | `bomb_calorimeter(delta_t: f64, c_cal: f64, mass_sample: f64) → f64` | $q = C_{cal}\Delta T / m$ | `thermochemistry::enthalpy` |
| Entropy change | `entropy_change(q_rev: f64, t: f64) → f64` | $\Delta S = q_{rev}/T$ | `thermochemistry::entropy` |
| Gibbs free energy | `gibbs_free_energy(delta_h: f64, t: f64, delta_s: f64) → f64` | $\Delta G = \Delta H - T\Delta S$ | `thermochemistry::entropy` |
| Spontaneity temperature | `spontaneity_temperature(delta_h: f64, delta_s: f64) → f64` | $T = \Delta H / \Delta S$ | `thermochemistry::entropy` |
| Entropy of mixing (ideal) | `entropy_mixing_ideal(mole_fractions: &[f64]) → f64` | $\Delta S_{mix} = -R \sum x_i \ln x_i$ | `thermochemistry::entropy` |
| Gibbs-Helmholtz | `gibbs_helmholtz(delta_g1: f64, delta_h: f64, t1: f64, t2: f64) → f64` | $\Delta G_2/T_2 = \Delta G_1/T_1 + \Delta H(1/T_2-1/T_1)$ | `thermochemistry::entropy` |
| Trouton's rule | `trouton_rule_entropy(delta_h_vap: f64, t_boil: f64) → f64` | $\Delta S_{vap} = \Delta H_{vap}/T_b \approx 85$ J/(mol·K) | `thermochemistry::entropy` |
| Standard entropy of reaction | `standard_entropy_of_reaction(products_s: &[f64], products_coeff: &[f64], reactants_s: &[f64], reactants_coeff: &[f64]) → f64` | $\Delta S^\circ = \sum \nu S^\circ(prod) - \sum \nu S^\circ(react)$ | `thermochemistry::entropy` |
| Entropy phase transition | `entropy_phase_transition(delta_h: f64, t_transition: f64) → f64` | $\Delta S = \Delta H / T$ | `thermochemistry::entropy` |
| Debye entropy | `debye_entropy(t: f64, theta_d: f64) → f64` | Debye model: $S(T, \Theta_D)$ | `thermochemistry::entropy` |
| Helmholtz free energy | `helmholtz_free_energy(u: f64, t: f64, s: f64) → f64` | $A = U - TS$ | `thermochemistry::entropy` |

### 5️⃣ Molecular — `chemistry::molecular` — 20 pub fn

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| Bond order | `bond_order(bonding_electrons: u32, antibonding_electrons: u32) → f64` | $BO = (n_b - n_a)/2$ | `molecular::bonding` |
| Dipole moment vector | `dipole_moment(charges: &[(f64, [f64; 3])]) → [f64; 3]` | $\vec{\mu} = \sum q_i \vec{r}_i$ | `molecular::bonding` |
| Dipole magnitude | `dipole_magnitude(mu: &[f64; 3]) → f64` | $|\mu| = \sqrt{\mu_x^2+\mu_y^2+\mu_z^2}$ | `molecular::bonding` |
| Coulomb integral | `coulomb_integral(z_eff: f64, n: f64) → f64` | $\alpha = -Z_{eff}^2/(2n^2)$ (Hartree) | `molecular::bonding` |
| Slater shielding | `slater_shielding(electrons_below: &[u32], shielding_constants: &[f64]) → f64` | $\sigma = \sum s_i$ | `molecular::bonding` |
| Mulliken electronegativity | `electronegativity_mulliken(ie: f64, ea: f64) → f64` | $\chi_M = (IE + EA)/2$ | `molecular::bonding` |
| Formal charge | `formal_charge(valence: i32, lone_pair: i32, bonding: i32) → i32` | $FC = V - LP - B/2$ | `molecular::bonding` |
| Molar mass | `molar_mass(atomic_masses: &[f64], counts: &[u32]) → f64` | $M = \sum n_i M_i$ | `molecular::bonding` |
| Percent composition | `percent_composition(element_mass: f64, element_count: u32, molar_mass: f64) → f64` | $\%_i = n_i M_i / M \times 100$ | `molecular::bonding` |
| Allred-Rochow electronegativity | `electronegativity_allred_rochow(z_eff: f64, r_cov: f64) → f64` | $\chi_{AR} = 0.359 Z_{eff}/r_{cov}^2 + 0.744$ | `molecular::bonding` |
| Polarizability estimate | `polarizability_estimate(volume_angstrom3: f64) → f64` | $\alpha \approx 4\pi\varepsilon_0 V$ | `molecular::bonding` |
| Pauling bond dissociation | `bond_dissociation_energy_pauling(d_aa: f64, d_bb: f64, en_diff: f64) → f64` | $D(AB) = \sqrt{D(AA) \cdot D(BB)} + 96.5(\Delta\chi)^2$ | `molecular::bonding` |
| VSEPR angle | `vsepr_angle(bonding_pairs: u32, lone_pairs: u32) → f64` | Ideal angle from electron pair geometry | `molecular::geometry` |
| Hybridization | `hybridization_sp(bonding_regions: u32) → &'static str` | sp / sp2 / sp3 / sp3d / sp3d2 | `molecular::geometry` |
| Ideal gas moles | `ideal_gas_moles(p: f64, v: f64, t: f64) → f64` | $n = PV/RT$ | `molecular::geometry` |
| Molecular geometry name | `molecular_geometry_name(bonding_pairs: u32, lone_pairs: u32) → &'static str` | VSEPR geometry label | `molecular::geometry` |
| Bond length estimate | `bond_length_estimate(r1: f64, r2: f64) → f64` | $d_{AB} = r_A + r_B$ | `molecular::geometry` |
| Badger's rule | `bond_energy_badger(r_e: f64, a: f64, b: f64) → f64` | $k = a(r_e - b)^{-3}$ | `molecular::geometry` |
| Coordination geometry angles | `coordination_geometry_angles(coordination: u32) → f64` | Ideal angle for coordination number | `molecular::geometry` |
| Effective nuclear charge | `effective_nuclear_charge(z: u32, s: f64) → f64` | $Z_{eff} = Z - \sigma$ | `molecular::geometry` |

### 6️⃣ Organic — `chemistry::organic` — 20 pub fn

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| CIP priority | `cahn_ingold_prelog_priority(atomic_numbers: &[u32]) → Vec<usize>` | Cahn-Ingold-Prelog priority ordering | `organic::descriptors` |
| Wiener index | `topological_index_wiener(distance_matrix: &[Vec<u32>]) → u64` | $W = \frac{1}{2}\sum_{i,j} d_{ij}$ | `organic::descriptors` |
| Randić index | `randic_index(adjacency: &[Vec<bool>], degrees: &[u32]) → f64` | $\chi = \sum_{(i,j)} (d_i d_j)^{-1/2}$ | `organic::descriptors` |
| LogP (partition coefficient) | `partition_coefficient_log_p(fragments: &[f64]) → f64` | $\log P = \sum f_i$ (fragment method) | `organic::descriptors` |
| TPSA | `topological_polar_surface_area(contributions: &[f64]) → f64` | $TPSA = \sum A_i$ | `organic::descriptors` |
| Rotatable bonds | `rotatable_bonds(single_bonds: u32, ring_bonds: u32, terminal_bonds: u32) → u32` | Non-ring, non-terminal single bonds | `organic::descriptors` |
| SN1 rate | `sn1_rate(k: f64, substrate: f64) → f64` | $r = k[R\text{-}X]$ | `organic::reactions` |
| SN2 rate | `sn2_rate(k: f64, substrate: f64, nucleophile: f64) → f64` | $r = k[R\text{-}X][Nu^-]$ | `organic::reactions` |
| E1 rate | `e1_rate(k: f64, substrate: f64) → f64` | $r = k[R\text{-}X]$ | `organic::reactions` |
| E2 rate | `e2_rate(k: f64, substrate: f64, base: f64) → f64` | $r = k[R\text{-}X][B^-]$ | `organic::reactions` |
| Hammett equation | `hammett_equation(rho: f64, sigma: f64, log_k0: f64) → f64` | $\log k = \log k_0 + \rho\sigma$ | `organic::reactions` |
| Hammett acidity | `hammett_acidity(pka_ref: f64, rho: f64, sigma: f64) → f64` | $pK_a = pK_a^{ref} - \rho\sigma$ | `organic::reactions` |
| Taft equation | `taft_equation(rho_star: f64, sigma_star: f64, es: f64, delta: f64) → f64` | $\log(k/k_0) = \rho^*\sigma^* + \delta E_s$ | `organic::reactions` |
| Degree of unsaturation | `degree_of_unsaturation(c: u32, h: u32, n: u32, halogens: u32) → f64` | $DoU = (2C+2+N-H-X)/2$ | `organic::structure` |
| Molecular formula mass | `molecular_formula_mass(c: u32, h: u32, o: u32, n: u32, s: u32) → f64` | $M = 12C + H + 16O + 14N + 32S$ | `organic::structure` |
| Alkane boiling point | `alkane_boiling_point_estimate(carbon_count: u32) → f64` | Empirical $T_b(n)$ correlation | `organic::structure` |
| Hückel energy (linear) | `huckel_energy_linear(n_atoms: usize, alpha: f64, beta: f64) → Vec<f64>` | $E_k = \alpha + 2\beta\cos\frac{k\pi}{n+1}$ | `organic::structure` |
| Hückel energy (cyclic) | `huckel_energy_cyclic(n_atoms: usize, alpha: f64, beta: f64) → Vec<f64>` | $E_k = \alpha + 2\beta\cos\frac{2k\pi}{n}$ | `organic::structure` |
| Delocalization energy | `delocalization_energy(total_pi_energy: f64, isolated_energy: f64) → f64` | $E_{deloc} = E_\pi - E_{isolated}$ | `organic::structure` |
| Resonance stabilization | `resonance_stabilization(n_structures: usize) → f64` | Stabilization from contributing structures | `organic::structure` |

### 7️⃣ Inorganic — `chemistry::inorganic` — 16 pub fn

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| Born-Landé lattice energy | `lattice_energy_born_lande(madelung: f64, z_plus: f64, z_minus: f64, r0: f64, born_exponent: f64) → f64` | $U = -\frac{N_A M z^+ z^- e^2}{4\pi\varepsilon_0 r_0}(1-1/n)$ | `inorganic::bonding` |
| Kapustinskii equation | `lattice_energy_kapustinskii(nu: u32, z_plus: f64, z_minus: f64, r_plus: f64, r_minus: f64) → f64` | $U = K \nu |z^+||z^-|/(r_++r_-)$ | `inorganic::bonding` |
| Pauling EN difference | `pauling_electronegativity_difference(en_a: f64, en_b: f64) → f64` | $\Delta\chi = |\chi_A - \chi_B|$ | `inorganic::bonding` |
| Percent ionic character | `percent_ionic_character(en_diff: f64) → f64` | $\% = 100(1-e^{-0.25\Delta\chi^2})$ | `inorganic::bonding` |
| Octahedral CFSE | `crystal_field_splitting_octahedral(dq: f64) → f64` | $\Delta_o = 10Dq$ | `inorganic::crystal_field` |
| Tetrahedral CFSE | `crystal_field_splitting_tetrahedral(dq_oct: f64) → f64` | $\Delta_t = \frac{4}{9}\Delta_o$ | `inorganic::crystal_field` |
| CFSE octahedral | `cfse_octahedral(t2g: u32, eg: u32, dq: f64, pairing_energy: f64) → f64` | $CFSE = -0.4 n_{t_{2g}} + 0.6 n_{e_g}$ (in $\Delta_o$) | `inorganic::crystal_field` |
| Spin-only magnetic moment | `magnetic_moment_spin_only(unpaired: u32) → f64` | $\mu = \sqrt{n(n+2)}$ BM | `inorganic::crystal_field` |
| Spectrochemical Dq | `spectrochemical_series_dq(ligand_f: f64, metal_g: f64) → f64` | $\Delta_o = f_{ligand} \times g_{metal}$ | `inorganic::crystal_field` |
| Jahn-Teller distortion | `jahn_teller_distortion(eg_occupation: u32) → bool` | Degenerate $e_g$ → JT active | `inorganic::crystal_field` |
| Nephelauxetic ratio | `nephelauxetic_ratio(b_complex: f64, b_free_ion: f64) → f64` | $\beta = B_{complex}/B_{free}$ | `inorganic::crystal_field` |
| Racah B parameter | `racah_parameter_b(transitions: &[f64], dq: f64) → f64` | $B$ from Tanabe-Sugano diagram | `inorganic::crystal_field` |
| Effective atomic number | `effective_atomic_number(metal_electrons: u32, ligand_electrons: u32) → u32` | $EAN = Z_{metal} + e_{ligand}$ | `inorganic::coordination` |
| Chelate effect | `chelate_effect(k_mono: f64, k_chelate: f64) → f64` | $\Delta = \log(K_{chelate}/K_{mono})$ | `inorganic::coordination` |
| Irving-Williams stability | `irving_williams_stability(ionization_energy: f64, ionic_radius: f64) → f64` | Stability ∝ IE / r | `inorganic::coordination` |
| Radius ratio rule | `coordination_number_radius_ratio(r_cation: f64, r_anion: f64) → u32` | $r_+/r_-$ → CN (4, 6, 8) | `inorganic::coordination` |

### 8️⃣ Analytical — `chemistry::analytical` — 20 pub fn

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| Retention factor | `retention_factor_rf(distance_solute: f64, distance_solvent: f64) → f64` | $R_f = d_{solute}/d_{solvent}$ | `analytical::chromatography` |
| HPLC resolution | `hplc_resolution(tr1: f64, tr2: f64, w1: f64, w2: f64) → f64` | $R_s = 2(t_{R2}-t_{R1})/(w_1+w_2)$ | `analytical::chromatography` |
| Theoretical plates | `theoretical_plates(tr: f64, w: f64) → f64` | $N = 16(t_R/w)^2$ | `analytical::chromatography` |
| HETP | `height_equivalent_theoretical_plate(column_length: f64, n_plates: f64) → f64` | $H = L/N$ | `analytical::chromatography` |
| Van Deemter | `van_deemter(a: f64, b: f64, c: f64, u: f64) → f64` | $H = A + B/u + Cu$ | `analytical::chromatography` |
| Selectivity factor | `selectivity_factor(k2: f64, k1: f64) → f64` | $\alpha = k_2'/k_1'$ | `analytical::chromatography` |
| Capacity factor | `capacity_factor(tr: f64, t0: f64) → f64` | $k' = (t_R - t_0)/t_0$ | `analytical::chromatography` |
| Dilution | `dilution(c1: f64, v1: f64, v2: f64) → f64` | $C_2 = C_1 V_1 / V_2$ | `analytical::quantitative` |
| Titration equivalence volume | `titration_equivalence_volume(c_analyte: f64, v_analyte: f64, c_titrant: f64, stoich_ratio: f64) → f64` | $V_{eq} = C_a V_a / (n \cdot C_t)$ | `analytical::quantitative` |
| Limit of detection | `limit_of_detection(blank_std: f64, slope: f64) → f64` | $LOD = 3\sigma_{blank}/S$ | `analytical::quantitative` |
| Limit of quantitation | `limit_of_quantitation(blank_std: f64, slope: f64) → f64` | $LOQ = 10\sigma_{blank}/S$ | `analytical::quantitative` |
| Percent recovery | `percent_recovery(measured: f64, expected: f64) → f64` | $\%R = (measured/expected) \times 100$ | `analytical::quantitative` |
| Relative standard deviation | `relative_standard_deviation(std_dev: f64, mean: f64) → f64` | $RSD = \sigma/\bar{x} \times 100$ | `analytical::quantitative` |
| Gravimetric factor | `gravimetric_factor(mw_analyte: f64, mw_precipitate: f64, stoich: f64) → f64` | $GF = n \cdot M_A / M_P$ | `analytical::quantitative` |
| Karl Fischer water | `karl_fischer_water_content(volume_reagent: f64, reagent_factor: f64, sample_mass: f64) → f64` | $\%H_2O = V \cdot f / m$ | `analytical::quantitative` |
| Beer-Lambert | `beer_lambert(epsilon: f64, path_length: f64, concentration: f64) → f64` | $A = \varepsilon l c$ | `analytical::spectrophotometry` |
| Absorbance → transmittance | `absorbance_to_transmittance(absorbance: f64) → f64` | $T = 10^{-A}$ | `analytical::spectrophotometry` |
| Transmittance → absorbance | `transmittance_to_absorbance(transmittance: f64) → f64` | $A = -\log T$ | `analytical::spectrophotometry` |
| Concentration from absorbance | `concentration_from_absorbance(absorbance: f64, epsilon: f64, path_length: f64) → f64` | $c = A/(\varepsilon l)$ | `analytical::spectrophotometry` |
| Signal-to-noise | `signal_to_noise(signal: f64, noise: f64) → f64` | $S/N = S/N$ | `analytical::spectrophotometry` |

### 9️⃣ Quantum Chemistry — `chemistry::quantum_chem` — 11 pub fn

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| LCAO bonding energy | `lcao_bonding_energy(alpha: f64, beta: f64, overlap: f64) → f64` | $E_+ = (\alpha+\beta)/(1+S)$ | `quantum_chem::orbitals` |
| LCAO antibonding energy | `lcao_antibonding_energy(alpha: f64, beta: f64, overlap: f64) → f64` | $E_- = (\alpha-\beta)/(1-S)$ | `quantum_chem::orbitals` |
| Overlap integral 1s | `overlap_integral_1s(r: f64, zeta: f64) → f64` | $S = e^{-\zeta r}(1+\zeta r + \zeta^2 r^2/3)$ | `quantum_chem::orbitals` |
| Hartree energy | `hartree_energy(kinetic: f64, nuclear_attraction: f64, electron_repulsion: f64) → f64` | $E = T + V_{ne} + V_{ee}$ | `quantum_chem::orbitals` |
| Roothaan total energy | `roothaan_total_energy(one_electron: &[f64], fock_eigenvalues: &[f64], nuclear_repulsion: f64) → f64` | $E = \frac{1}{2}\sum(h_i + \varepsilon_i) + V_{nn}$ | `quantum_chem::orbitals` |
| Mulliken population | `mulliken_population(density: &[Vec<f64>], overlap: &[Vec<f64>]) → Vec<f64>` | $q_A = Z_A - \sum_{\mu \in A}(PS)_{\mu\mu}$ | `quantum_chem::orbitals` |
| Nuclear repulsion energy | `nuclear_repulsion_energy(z1: f64, z2: f64, r: f64) → f64` | $V_{nn} = Z_1 Z_2 e^2 / r$ | `quantum_chem::orbitals` |
| Hückel secular determinant | `huckel_secular_determinant(n: usize, alpha: f64, beta: f64) → Vec<Vec<f64>>` | $\det(H - ES) = 0$ (linear) | `quantum_chem::huckel` |
| Hückel cyclic determinant | `huckel_cyclic_determinant(n: usize, alpha: f64, beta: f64) → Vec<Vec<f64>>` | $\det(H - ES) = 0$ (cyclic) | `quantum_chem::huckel` |
| Hückel charge density | `huckel_charge_density(coefficients: &[Vec<f64>], occupations: &[f64]) → Vec<f64>` | $q_r = \sum_i n_i c_{ir}^2$ | `quantum_chem::huckel` |
| Hückel bond order | `huckel_bond_order(coefficients: &[Vec<f64>], occupations: &[f64], atom_i: usize, atom_j: usize) → f64` | $p_{rs} = \sum_i n_i c_{ir} c_{is}$ | `quantum_chem::huckel` |

### 🔟 Solutions — `chemistry::solutions` — 14 pub fn

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| Boiling point elevation | `boiling_point_elevation(kb: f64, molality: f64, vant_hoff_factor: f64) → f64` | $\Delta T_b = i K_b m$ | `solutions::colligative` |
| Freezing point depression | `freezing_point_depression(kf: f64, molality: f64, vant_hoff_factor: f64) → f64` | $\Delta T_f = i K_f m$ | `solutions::colligative` |
| Osmotic pressure | `osmotic_pressure(molarity: f64, temperature: f64, vant_hoff_factor: f64) → f64` | $\Pi = iMRT$ | `solutions::colligative` |
| Vapor pressure lowering | `vapor_pressure_lowering(x_solvent: f64, p0_solvent: f64) → f64` | $\Delta P = x_{solute} P_0$ | `solutions::colligative` |
| Molar mass (ebullioscopy) | `molar_mass_from_ebullioscopy(kb: f64, mass_solute: f64, mass_solvent_kg: f64, delta_t: f64) → f64` | $M = K_b m_s / (m_{solv} \Delta T_b)$ | `solutions::colligative` |
| Molar mass (cryoscopy) | `molar_mass_from_cryoscopy(kf: f64, mass_solute: f64, mass_solvent_kg: f64, delta_t: f64) → f64` | $M = K_f m_s / (m_{solv} \Delta T_f)$ | `solutions::colligative` |
| Raoult's law | `raoult_law(x_solvent: f64, p0_solvent: f64) → f64` | $P = x P_0$ | `solutions::mixtures` |
| Henry's law | `henrys_law(kh: f64, partial_pressure: f64) → f64` | $C = K_H P$ | `solutions::mixtures` |
| Mole fraction | `mole_fraction(moles_component: f64, total_moles: f64) → f64` | $x_i = n_i / n_{total}$ | `solutions::mixtures` |
| Molality | `molality(moles_solute: f64, mass_solvent_kg: f64) → f64` | $m = n / m_{solv}$ | `solutions::mixtures` |
| Molarity | `molarity(moles_solute: f64, volume_liters: f64) → f64` | $M = n / V$ | `solutions::mixtures` |
| Gibbs-Duhem check | `gibbs_duhem_check(x1: f64, d_mu1: f64, x2: f64, d_mu2: f64) → f64` | $\sum x_i d\mu_i = 0$ | `solutions::mixtures` |
| Activity from mole fraction | `activity_from_mole_fraction(gamma: f64, x: f64) → f64` | $a = \gamma x$ | `solutions::mixtures` |
| Margules one-suffix | `margules_one_suffix(a: f64, x1: f64) → f64` | $\ln\gamma_1 = A x_2^2$ | `solutions::mixtures` |

### 11. Gas Laws — `chemistry::gas_laws` — 27 pub fn

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| Ideal gas pressure | `ideal_gas_pressure(n: f64, t: f64, v: f64) → f64` | $P = nRT/V$ | `gas_laws::ideal` |
| Ideal gas volume | `ideal_gas_volume(n: f64, t: f64, p: f64) → f64` | $V = nRT/P$ | `gas_laws::ideal` |
| Ideal gas temperature | `ideal_gas_temperature(p: f64, v: f64, n: f64) → f64` | $T = PV/nR$ | `gas_laws::ideal` |
| Boyle's law | `boyles_law(p1: f64, v1: f64, v2: f64) → f64` | $P_2 = P_1 V_1 / V_2$ | `gas_laws::ideal` |
| Charles's law | `charles_law(v1: f64, t1: f64, t2: f64) → f64` | $V_2 = V_1 T_2 / T_1$ | `gas_laws::ideal` |
| Gay-Lussac's law | `gay_lussac_law(p1: f64, t1: f64, t2: f64) → f64` | $P_2 = P_1 T_2 / T_1$ | `gas_laws::ideal` |
| Combined gas law | `combined_gas_law(p1: f64, v1: f64, t1: f64, t2: f64, p2: f64) → f64` | $V_2 = P_1 V_1 T_2 / (T_1 P_2)$ | `gas_laws::ideal` |
| Dalton partial pressure | `dalton_partial_pressure(mole_fraction: f64, total_pressure: f64) → f64` | $P_i = x_i P_{total}$ | `gas_laws::ideal` |
| Graham's law | `grahams_law_effusion(m1: f64, m2: f64) → f64` | $r_1/r_2 = \sqrt{M_2/M_1}$ | `gas_laws::ideal` |
| Gas density | `gas_density(p: f64, mw: f64, t: f64) → f64` | $\rho = PM/RT$ | `gas_laws::ideal` |
| RMS speed | `rms_speed(t: f64, mw: f64) → f64` | $v_{rms} = \sqrt{3RT/M}$ | `gas_laws::ideal` |
| Mean speed | `mean_speed(t: f64, mw: f64) → f64` | $\bar{v} = \sqrt{8RT/\pi M}$ | `gas_laws::ideal` |
| Most probable speed | `most_probable_speed(t: f64, mw: f64) → f64` | $v_{mp} = \sqrt{2RT/M}$ | `gas_laws::ideal` |
| Mean free path | `mean_free_path(d: f64, n_over_v: f64) → f64` | $\lambda = 1/(\sqrt{2}\pi d^2 n/V)$ | `gas_laws::ideal` |
| Van der Waals pressure | `van_der_waals_pressure(n: f64, t: f64, v: f64, a: f64, b: f64) → f64` | $P = nRT/(V-nb) - a(n/V)^2$ | `gas_laws::real` |
| Redlich-Kwong pressure | `redlich_kwong_pressure(n: f64, t: f64, v: f64, a: f64, b: f64) → f64` | $P = nRT/(V-nb) - an^2/(T^{1/2}V(V+nb))$ | `gas_laws::real` |
| Compressibility factor | `compressibility_factor(p: f64, v: f64, n: f64, t: f64) → f64` | $Z = PV/nRT$ | `gas_laws::real` |
| Virial equation (B) | `virial_equation_b(p: f64, t: f64, b: f64) → f64` | $PV_m = RT(1 + B/V_m)$ | `gas_laws::real` |
| Boyle temperature | `boyle_temperature(a: f64, b: f64) → f64` | $T_B = a/(bR)$ | `gas_laws::real` |
| Critical temperature | `critical_temperature(a: f64, b: f64) → f64` | $T_c = 8a/(27bR)$ | `gas_laws::real` |
| Critical pressure | `critical_pressure(a: f64, b: f64) → f64` | $P_c = a/(27b^2)$ | `gas_laws::real` |
| Critical volume | `critical_volume(b: f64) → f64` | $V_c = 3b$ | `gas_laws::real` |
| Reduced temperature | `reduced_temperature(t: f64, tc: f64) → f64` | $T_r = T/T_c$ | `gas_laws::real` |
| Reduced pressure | `reduced_pressure(p: f64, pc: f64) → f64` | $P_r = P/P_c$ | `gas_laws::real` |
| Peng-Robinson pressure | `peng_robinson_pressure(t: f64, vm: f64, a: f64, b: f64) → f64` | $P = RT/(V_m-b) - a/(V_m^2+2bV_m-b^2)$ | `gas_laws::real` |
| Fugacity coefficient | `fugacity_coefficient(z: f64, b_prime: f64, p: f64) → f64` | $\ln\phi = Z-1-\ln(Z-B')$ | `gas_laws::real` |
| Acentric factor | `acentric_factor(p_sat: f64, pc: f64) → f64` | $\omega = -\log(P_{sat}/P_c) - 1$ at $T_r=0.7$ | `gas_laws::real` |

### 12. Acid-Base — `chemistry::acid_base` — 19 pub fn

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| Henderson-Hasselbalch | `henderson_hasselbalch(pka: f64, base: f64, acid: f64) → f64` | $\text{pH} = pK_a + \log([A^-]/[HA])$ | `acid_base::equilibria` |
| pKa from Ka | `pka_from_ka(ka: f64) → f64` | $pK_a = -\log K_a$ | `acid_base::equilibria` |
| Ka from pKa | `ka_from_pka(pka: f64) → f64` | $K_a = 10^{-pK_a}$ | `acid_base::equilibria` |
| pKb from pKa | `pkb_from_pka(pka: f64, pkw: f64) → f64` | $pK_b = pK_w - pK_a$ | `acid_base::equilibria` |
| Strong acid pH | `ph_strong_acid(concentration: f64) → f64` | $\text{pH} = -\log[H^+]$ | `acid_base::equilibria` |
| Strong base pH | `ph_strong_base(concentration: f64) → f64` | $\text{pH} = 14 + \log[OH^-]$ | `acid_base::equilibria` |
| Weak acid pH | `ph_weak_acid(ka: f64, c: f64) → f64` | $\text{pH} = \frac{1}{2}(pK_a - \log c)$ | `acid_base::equilibria` |
| Weak base pH | `ph_weak_base(kb: f64, c: f64) → f64` | $\text{pOH} = \frac{1}{2}(pK_b - \log c)$ | `acid_base::equilibria` |
| Alpha fraction | `alpha_fraction(h: f64, ka_values: &[f64], species_index: usize) → f64` | $\alpha_i = f([H^+], K_a)$ | `acid_base::equilibria` |
| Amphiprotic pH | `amphiprotic_ph(pka1: f64, pka2: f64) → f64` | $\text{pH} = (pK_{a1}+pK_{a2})/2$ | `acid_base::equilibria` |
| Ionic product of water | `ionic_product_water(t: f64) → f64` | $K_w(T) = [H^+][OH^-]$ | `acid_base::equilibria` |
| Strong/strong titration pH | `strong_acid_strong_base_ph(c_acid: f64, v_acid: f64, c_base: f64, v_base: f64) → f64` | pH of strong acid–strong base mixture | `acid_base::titrations` |
| Weak acid/strong base pH | `weak_acid_strong_base_ph(c_acid: f64, v_acid: f64, ka: f64, c_base: f64, v_base: f64) → f64` | pH of weak acid–strong base mixture | `acid_base::titrations` |
| Equivalence point volume | `equivalence_point_volume(c_analyte: f64, v_analyte: f64, c_titrant: f64) → f64` | $V_{eq} = C_a V_a / C_t$ | `acid_base::titrations` |
| Half-equivalence pH | `half_equivalence_ph(pka: f64) → f64` | $\text{pH} = pK_a$ | `acid_base::titrations` |
| Buffer range | `buffer_range(pka: f64) → (f64, f64)` | $pK_a \pm 1$ | `acid_base::titrations` |
| Van Slyke buffer capacity | `buffer_capacity_vanslyke(c_total: f64, ka: f64, h: f64) → f64` | $\beta = 2.303 C K_a[H^+]/(K_a+[H^+])^2$ | `acid_base::titrations` |
| Diprotic first equiv pH | `diprotic_ph_first_equiv(pka1: f64, pka2: f64) → f64` | $\text{pH} = (pK_{a1}+pK_{a2})/2$ | `acid_base::titrations` |
| Back titration moles | `back_titration_moles(mol_excess_added: f64, c_back_titrant: f64, v_back_titrant: f64) → f64` | $n_{analyte} = n_{excess} - C_b V_b$ | `acid_base::titrations` |

### 13. Nuclear — `chemistry::nuclear` — 12 pub fn

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| Mass defect | `mass_defect(z: u32, n: u32, atomic_mass: f64) → f64` | $\Delta m = Z m_p + N m_n - M$ | `nuclear::energy` |
| Binding energy | `binding_energy(mass_defect: f64) → f64` | $E_B = \Delta m \cdot c^2$ | `nuclear::energy` |
| Binding energy per nucleon | `binding_energy_per_nucleon(binding_energy: f64, a: u32) → f64` | $E_B/A$ | `nuclear::energy` |
| Q-value | `q_value(reactant_masses: &[f64], product_masses: &[f64]) → f64` | $Q = (\sum m_r - \sum m_p)c^2$ | `nuclear::energy` |
| Semi-empirical mass formula | `semi_empirical_mass_formula(a: u32, z: u32, av: f64, as_: f64, ac: f64, aa: f64, ap: f64) → f64` | $E_B = a_V A - a_S A^{2/3} - a_C Z^2/A^{1/3} - a_A(N-Z)^2/A + \delta$ | `nuclear::energy` |
| Threshold energy | `threshold_energy(q: f64, mass_projectile: f64, mass_target: f64) → f64` | $E_{th} = -Q(1+m_p/m_T)/2$ | `nuclear::energy` |
| Decay constant from half-life | `half_life_to_decay_constant(half_life: f64) → f64` | $\lambda = \ln 2 / t_{1/2}$ | `nuclear::decay` |
| Half-life from decay constant | `decay_constant_to_half_life(lambda: f64) → f64` | $t_{1/2} = \ln 2 / \lambda$ | `nuclear::decay` |
| Remaining nuclei | `remaining_nuclei(n0: f64, lambda: f64, t: f64) → f64` | $N = N_0 e^{-\lambda t}$ | `nuclear::decay` |
| Activity | `activity(lambda: f64, n: f64) → f64` | $A = \lambda N$ | `nuclear::decay` |
| Decay chain intermediate | `decay_chain_intermediate(n0: f64, lambda1: f64, lambda2: f64, t: f64) → f64` | Bateman equation: $N_2(t)$ | `nuclear::decay` |
| Specific activity | `specific_activity(lambda: f64, avogadro: f64, molar_mass: f64) → f64` | $a = \lambda N_A / M$ | `nuclear::decay` |

### 14. Photochemistry — `chemistry::photochemistry` — 13 pub fn

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| Quantum yield | `quantum_yield(molecules_reacted: f64, photons_absorbed: f64) → f64` | $\Phi = N_{reacted}/N_{photons}$ | `photochemistry::quantum_yield` |
| Photon energy (J) | `photon_energy(wavelength_nm: f64) → f64` | $E = hc/\lambda$ | `photochemistry::quantum_yield` |
| Photon energy (eV) | `photon_energy_ev(wavelength_nm: f64) → f64` | $E = 1240/\lambda$ (eV) | `photochemistry::quantum_yield` |
| Einstein energy | `einstein_energy(wavelength_nm: f64) → f64` | $E = N_A hc/\lambda$ | `photochemistry::quantum_yield` |
| Fluorescence lifetime | `fluorescence_lifetime(rate_radiative: f64, rate_non_radiative: f64) → f64` | $\tau = 1/(k_r + k_{nr})$ | `photochemistry::quantum_yield` |
| Fluorescence quantum yield | `fluorescence_quantum_yield(rate_radiative: f64, rate_non_radiative: f64) → f64` | $\Phi_f = k_r/(k_r + k_{nr})$ | `photochemistry::quantum_yield` |
| Photolysis rate | `photolysis_rate(quantum_yield: f64, absorption_cross_section: f64, flux: f64) → f64` | $J = \Phi \sigma F$ | `photochemistry::kinetics` |
| Stern-Volmer | `stern_volmer(i0: f64, ksv: f64, quencher: f64) → f64` | $I = I_0/(1 + K_{SV}[Q])$ | `photochemistry::kinetics` |
| Stern-Volmer ratio | `stern_volmer_ratio(ksv: f64, quencher: f64) → f64` | $I_0/I = 1 + K_{SV}[Q]$ | `photochemistry::kinetics` |
| ISC rate | `rate_intersystem_crossing(total_rate: f64, rate_fluorescence: f64, rate_internal_conversion: f64) → f64` | $k_{ISC} = k_{total} - k_f - k_{IC}$ | `photochemistry::kinetics` |
| Phosphorescence lifetime | `phosphorescence_lifetime(rate_phosphorescence: f64, rate_non_radiative: f64) → f64` | $\tau_p = 1/(k_p + k_{nr})$ | `photochemistry::kinetics` |
| Förster radius | `forster_radius(quantum_yield_donor: f64, kappa_sq: f64, overlap_integral: f64, n_refraction: f64) → f64` | $R_0^6 = \frac{9000\ln 10\, \kappa^2 \Phi_D J}{128\pi^5 n^4 N_A}$ | `photochemistry::kinetics` |
| FRET efficiency | `fret_efficiency(r: f64, r0: f64) → f64` | $E = R_0^6/(R_0^6 + r^6)$ | `photochemistry::kinetics` |

### 15. Polymers — `chemistry::polymers` — 14 pub fn

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| Number-average molar mass | `number_average_molar_mass(ni: &[f64], mi: &[f64]) → f64` | $\bar{M}_n = \sum N_i M_i / \sum N_i$ | `polymers::distributions` |
| Weight-average molar mass | `weight_average_molar_mass(ni: &[f64], mi: &[f64]) → f64` | $\bar{M}_w = \sum N_i M_i^2 / \sum N_i M_i$ | `polymers::distributions` |
| Z-average molar mass | `z_average_molar_mass(ni: &[f64], mi: &[f64]) → f64` | $\bar{M}_z = \sum N_i M_i^3 / \sum N_i M_i^2$ | `polymers::distributions` |
| Schulz-Flory distribution | `schulz_flory_distribution(p: f64, x: u32) → f64` | $w(x) = x(1-p)^2 p^{x-1}$ | `polymers::distributions` |
| Most probable chain length | `most_probable_chain_length(p: f64) → f64` | $x_n = 1/(1-p)$ | `polymers::distributions` |
| Flory-Huggins free energy | `flory_huggins_free_energy(phi: f64, n1: f64, n2: f64, chi: f64, temperature: f64) → f64` | $\Delta G_m = RT[\phi\ln\phi/N_1 + (1-\phi)\ln(1-\phi)/N_2 + \chi\phi(1-\phi)]$ | `polymers::distributions` |
| DPn | `degree_of_polymerization_number(mn: f64, m0: f64) → f64` | $DP_n = \bar{M}_n / M_0$ | `polymers::properties` |
| DPw | `degree_of_polymerization_weight(mw: f64, m0: f64) → f64` | $DP_w = \bar{M}_w / M_0$ | `polymers::properties` |
| Polydispersity index | `polydispersity_index(mw: f64, mn: f64) → f64` | $PDI = \bar{M}_w / \bar{M}_n$ | `polymers::properties` |
| Mark-Houwink viscosity | `intrinsic_viscosity_mark_houwink(k: f64, m: f64, a: f64) → f64` | $[\eta] = K M^a$ | `polymers::properties` |
| End-to-end distance | `end_to_end_distance_freely_jointed(n: f64, l: f64) → f64` | $\langle r^2 \rangle^{1/2} = l\sqrt{n}$ | `polymers::properties` |
| Radius of gyration | `radius_of_gyration(end_to_end: f64) → f64` | $R_g = r/\sqrt{6}$ | `polymers::properties` |
| Fox equation (Tg blend) | `glass_transition_fox(w1: f64, tg1: f64, w2: f64, tg2: f64) → f64` | $1/T_g = w_1/T_{g1} + w_2/T_{g2}$ | `polymers::properties` |
| Carothers equation | `carothers_equation(p: f64, f_avg: f64) → f64` | $DP_n = 2/(2-pf_{avg})$ | `polymers::properties` |

### 16. Spectroscopy — `chemistry::spectroscopy` — 20 pub fn

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| Mass-to-charge ratio | `mass_to_charge(mass: f64, charge: u32) → f64` | $m/z = M/q$ | `spectroscopy::mass_spec` |
| Exact mass difference | `exact_mass_difference(theoretical: f64, experimental: f64) → f64` | $\Delta m = m_{theo} - m_{exp}$ | `spectroscopy::mass_spec` |
| Nitrogen rule | `nitrogen_rule(nominal_mass: u32) → bool` | Odd $N$ count → odd nominal mass | `spectroscopy::mass_spec` |
| Rings + double bonds | `rings_plus_double_bonds(c: u32, h: u32, n: u32, halogens: u32) → f64` | $RDB = (2C+2+N-H-X)/2$ | `spectroscopy::mass_spec` |
| Monoisotopic abundance | `isotope_pattern_monoisotopic(abundances: &[f64]) → f64` | $P_{mono} = \prod a_i$ | `spectroscopy::mass_spec` |
| Resolving power | `resolving_power(m: f64, delta_m: f64) → f64` | $R = m/\Delta m$ | `spectroscopy::mass_spec` |
| Chemical shift (ppm) | `chemical_shift_ppm(frequency_sample: f64, frequency_reference: f64, spectrometer: f64) → f64` | $\delta = 10^6(\nu - \nu_{ref})/\nu_0$ | `spectroscopy::nmr` |
| Coupling constant | `coupling_constant_first_order(line_separation_hz: f64) → f64` | $J = \Delta\nu$ (Hz) | `spectroscopy::nmr` |
| Multiplicity | `multiplicity(n_neighbors: u32) → u32` | $m = n + 1$ | `spectroscopy::nmr` |
| Larmor frequency | `larmor_frequency(gamma: f64, b0: f64) → f64` | $\nu_L = \gamma B_0 / 2\pi$ | `spectroscopy::nmr` |
| T1 inversion recovery | `t1_inversion_recovery(m0: f64, t1: f64, tau: f64) → f64` | $M(\tau) = M_0(1-2e^{-\tau/T_1})$ | `spectroscopy::nmr` |
| T2 spin echo | `t2_spin_echo(m0: f64, t2: f64, tau: f64) → f64` | $M(\tau) = M_0 e^{-\tau/T_2}$ | `spectroscopy::nmr` |
| Linewidth from T2 | `linewidth_from_t2(t2: f64) → f64` | $\Delta\nu = 1/(\pi T_2)$ | `spectroscopy::nmr` |
| NOE enhancement | `nuclear_overhauser_enhancement(gamma_irradiated: f64, gamma_observed: f64) → f64` | $\eta = \gamma_I / (2\gamma_S)$ | `spectroscopy::nmr` |
| Wavenumber → wavelength | `wavenumber_to_wavelength(wavenumber_cm: f64) → f64` | $\lambda = 10^4 / \tilde{\nu}$ (μm) | `spectroscopy::ir` |
| Wavelength → wavenumber | `wavelength_to_wavenumber(wavelength_um: f64) → f64` | $\tilde{\nu} = 10^4 / \lambda$ (cm⁻¹) | `spectroscopy::ir` |
| Wavenumber → frequency | `wavenumber_to_frequency(wavenumber_cm: f64) → f64` | $\nu = c \tilde{\nu}$ | `spectroscopy::ir` |
| Force constant | `force_constant_from_frequency(wavenumber: f64, reduced_mass_amu: f64) → f64` | $k = 4\pi^2 c^2 \tilde{\nu}^2 \mu$ | `spectroscopy::ir` |
| Reduced mass | `reduced_mass(m1: f64, m2: f64) → f64` | $\mu = m_1 m_2/(m_1+m_2)$ | `spectroscopy::ir` |
| IR intensity ratio | `ir_intensity_ratio(absorbance_sample: f64, absorbance_reference: f64) → f64` | $I_r = A_s/A_{ref}$ | `spectroscopy::ir` |

### 17. Surface — `chemistry::surface` — 13 pub fn

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| Langmuir isotherm | `langmuir_isotherm(theta_max: f64, k: f64, pressure: f64) → f64` | $\theta = \theta_{max} KP/(1+KP)$ | `surface::adsorption` |
| Freundlich isotherm | `freundlich_isotherm(kf: f64, pressure: f64, n: f64) → f64` | $\theta = K_f P^{1/n}$ | `surface::adsorption` |
| BET isotherm | `bet_isotherm(v_mono: f64, c: f64, p: f64, p0: f64) → f64` | $V = \frac{V_m c x}{(1-x)(1+(c-1)x)}$ | `surface::adsorption` |
| Temkin isotherm | `temkin_isotherm(rt_over_b: f64, a: f64, pressure: f64) → f64` | $\theta = (RT/b)\ln(aP)$ | `surface::adsorption` |
| Dissociative Langmuir | `langmuir_dissociative(k: f64, pressure: f64) → f64` | $\theta = \sqrt{KP}/(1+\sqrt{KP})$ | `surface::adsorption` |
| BET surface area | `bet_surface_area(v_mono: f64, cross_section: f64, avogadro: f64, molar_volume: f64) → f64` | $S = V_m N_A \sigma / V_{mol}$ | `surface::adsorption` |
| Young equation | `surface_tension_young(gamma_sv: f64, gamma_sl: f64, cos_theta: f64) → f64` | $\gamma_{LV}\cos\theta = \gamma_{SV} - \gamma_{SL}$ | `surface::tension` |
| Contact angle | `contact_angle(gamma_sv: f64, gamma_sl: f64, gamma_lv: f64) → f64` | $\cos\theta = (\gamma_{SV}-\gamma_{SL})/\gamma_{LV}$ | `surface::tension` |
| Capillary rise | `capillary_rise(gamma: f64, cos_theta: f64, rho: f64, g: f64, radius: f64) → f64` | $h = 2\gamma\cos\theta/(\rho g r)$ | `surface::tension` |
| Laplace pressure | `laplace_pressure(gamma: f64, r1: f64, r2: f64) → f64` | $\Delta P = \gamma(1/R_1+1/R_2)$ | `surface::tension` |
| Gibbs adsorption | `gibbs_adsorption(d_gamma: f64, d_ln_concentration: f64, temperature: f64) → f64` | $\Gamma = -\frac{1}{RT}\frac{d\gamma}{d\ln c}$ | `surface::tension` |
| Spreading coefficient | `spreading_coefficient(gamma_sv: f64, gamma_lv: f64, gamma_sl: f64) → f64` | $S = \gamma_{SV} - \gamma_{SL} - \gamma_{LV}$ | `surface::tension` |
| Work of adhesion | `work_of_adhesion(gamma_lv: f64, cos_theta: f64) → f64` | $W_a = \gamma_{LV}(1+\cos\theta)$ | `surface::tension` |

### 18. Crystallography — `chemistry::crystallography` — 16 pub fn

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| Cubic volume | `cubic_volume(a: f64) → f64` | $V = a^3$ | `crystallography::lattice` |
| Tetragonal volume | `tetragonal_volume(a: f64, c: f64) → f64` | $V = a^2 c$ | `crystallography::lattice` |
| Orthorhombic volume | `orthorhombic_volume(a: f64, b: f64, c: f64) → f64` | $V = abc$ | `crystallography::lattice` |
| Hexagonal volume | `hexagonal_volume(a: f64, c: f64) → f64` | $V = \frac{\sqrt{3}}{2}a^2 c$ | `crystallography::lattice` |
| SC packing fraction | `packing_fraction_simple_cubic() → f64` | $\pi/6 \approx 0.524$ | `crystallography::lattice` |
| BCC packing fraction | `packing_fraction_bcc() → f64` | $\pi\sqrt{3}/8 \approx 0.680$ | `crystallography::lattice` |
| FCC packing fraction | `packing_fraction_fcc() → f64` | $\pi/(3\sqrt{2}) \approx 0.740$ | `crystallography::lattice` |
| Unit cell density | `density_from_unit_cell(z: u32, molar_mass: f64, volume: f64, avogadro: f64) → f64` | $\rho = ZM/(N_A V)$ | `crystallography::lattice` |
| Miller direction cosines | `miller_to_direction_cosines(h: i32, k: i32, l: i32) → (f64, f64, f64)` | $(h,k,l)/\sqrt{h^2+k^2+l^2}$ | `crystallography::lattice` |
| Reciprocal lattice vector | `reciprocal_lattice_vector(a: f64) → f64` | $a^* = 2\pi/a$ | `crystallography::lattice` |
| Bragg's law | `bragg_law(d: f64, theta: f64, n: u32) → f64` | $n\lambda = 2d\sin\theta$ | `crystallography::diffraction` |
| Bragg angle | `bragg_angle(wavelength: f64, d: f64, n: u32) → f64` | $\theta = \arcsin(n\lambda/2d)$ | `crystallography::diffraction` |
| Interplanar spacing (cubic) | `interplanar_spacing_cubic(a: f64, h: i32, k: i32, l: i32) → f64` | $d = a/\sqrt{h^2+k^2+l^2}$ | `crystallography::diffraction` |
| Structure factor BCC | `structure_factor_bcc(h: i32, k: i32, l: i32, f_atom: f64) → f64` | $F = f[1+e^{i\pi(h+k+l)}]$ | `crystallography::diffraction` |
| Structure factor FCC | `structure_factor_fcc(h: i32, k: i32, l: i32, f_atom: f64) → f64` | $F = f[1+e^{i\pi(h+k)}+e^{i\pi(h+l)}+e^{i\pi(k+l)}]$ | `crystallography::diffraction` |
| Scherrer crystal size | `scherrer_crystal_size(k: f64, wavelength: f64, fwhm: f64, theta: f64) → f64` | $\tau = K\lambda/(\beta\cos\theta)$ | `crystallography::diffraction` |

### 19. Stoichiometry — `chemistry::stoichiometry` — 17 pub fn

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| Limiting reagent | `limiting_reagent(moles: &[f64], coefficients: &[f64]) → usize` | $\min(n_i/\nu_i)$ | `stoichiometry::balancing` |
| Theoretical yield | `theoretical_yield(moles_limiting: f64, coeff_limiting: f64, coeff_product: f64, mw_product: f64) → f64` | $m = n_{lim} \cdot \nu_P/\nu_{lim} \cdot M_P$ | `stoichiometry::balancing` |
| Percent yield | `percent_yield(actual: f64, theoretical: f64) → f64` | $\%Y = (m_{actual}/m_{theo}) \times 100$ | `stoichiometry::balancing` |
| Excess reagent | `excess_reagent(moles_a: f64, coeff_a: f64, moles_b: f64, coeff_b: f64) → f64` | Moles of reagent in excess | `stoichiometry::balancing` |
| Moles from mass | `moles_from_mass(mass: f64, molar_mass: f64) → f64` | $n = m/M$ | `stoichiometry::balancing` |
| Mass from moles | `mass_from_moles(moles: f64, molar_mass: f64) → f64` | $m = nM$ | `stoichiometry::balancing` |
| Number of particles | `number_of_particles(moles: f64) → f64` | $N = n N_A$ | `stoichiometry::balancing` |
| Oxidation number | `oxidation_number_simple(charge: i32, num_atoms: u32) → f64` | $ON = q/n$ | `stoichiometry::calculations` |
| Equivalent mass | `equivalent_mass(molar_mass: f64, n_equivalents: f64) → f64` | $E = M/n_{eq}$ | `stoichiometry::calculations` |
| Normality | `normality(equivalents: f64, volume_liters: f64) → f64` | $N = eq/V$ | `stoichiometry::calculations` |
| Atom economy | `atom_economy(mw_desired_product: f64, mw_all_products: &[f64]) → f64` | $AE = M_P/\sum M_i \times 100$ | `stoichiometry::calculations` |
| Empirical formula ratio | `empirical_formula_ratio(masses: &[f64], molar_masses: &[f64]) → Vec<f64>` | $n_i = m_i/M_i$ → ratio | `stoichiometry::calculations` |
| Dilution factor | `dilution_factor(v_initial: f64, v_final: f64) → f64` | $DF = V_f/V_i$ | `stoichiometry::calculations` |
| Stoichiometric ratio | `stoichiometric_ratio(coeff_a: f64, coeff_b: f64) → f64` | $\nu_A/\nu_B$ | `stoichiometry::calculations` |
| Mass percent | `mass_percent(mass_solute: f64, mass_solution: f64) → f64` | $\%w = m_s/m_{sol} \times 100$ | `stoichiometry::calculations` |
| PPM from mass | `ppm_from_mass(mass_solute: f64, mass_solution: f64) → f64` | $ppm = (m_s/m_{sol}) \times 10^6$ | `stoichiometry::calculations` |
| Molarity → molality | `molarity_to_molality(molarity: f64, density: f64, molar_mass_solute: f64) → f64` | $m = M_{arity}/(d - M_{arity} \cdot M_s)$ | `stoichiometry::calculations` |

### 20. Colloids — `chemistry::colloids` — 18 pub fn

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| Stokes sedimentation | `stokes_sedimentation(r: f64, rho_p: f64, rho_f: f64, viscosity: f64) → f64` | $v = 2r^2(\rho_p-\rho_f)g/(9\eta)$ | `colloids::properties` |
| Brownian diffusion | `brownian_diffusion_coefficient(t: f64, viscosity: f64, r: f64) → f64` | $D = k_BT/(6\pi\eta r)$ | `colloids::properties` |
| Einstein displacement | `einstein_diffusion_displacement(d: f64, t: f64) → f64` | $\langle x^2 \rangle^{1/2} = \sqrt{2Dt}$ | `colloids::properties` |
| Péclet number | `peclet_number_colloid(velocity: f64, r: f64, d: f64) → f64` | $Pe = vr/D$ | `colloids::properties` |
| Osmotic pressure (colloid) | `osmotic_pressure_colloid(n_particles: f64, volume: f64, t: f64) → f64` | $\Pi = (N/V)k_BT$ | `colloids::properties` |
| Sedimentation coefficient | `sedimentation_coefficient(velocity: f64, omega: f64, r_centrifuge: f64) → f64` | $s = v/(\omega^2 r)$ | `colloids::properties` |
| Tyndall scattering | `tyndall_scattering_intensity(n: f64, v_particle: f64, wavelength: f64) → f64` | $I \propto NV^2/\lambda^4$ | `colloids::properties` |
| Specific surface area | `specific_surface_area(radius: f64, density: f64) → f64` | $S_s = 3/(\rho r)$ | `colloids::properties` |
| Smoluchowski flocculation | `flocculation_rate_smoluchowski(n0: f64, k_b: f64, t: f64, viscosity: f64) → f64` | $J = 4k_BTN^2/(3\eta)$ | `colloids::properties` |
| Coagulation half-life | `half_life_coagulation(n0: f64, k_rate: f64) → f64` | $t_{1/2} = 1/(kN_0)$ | `colloids::properties` |
| DLVO total energy | `dlvo_total_energy(van_der_waals: f64, electrostatic: f64) → f64` | $V_T = V_{vdW} + V_{elec}$ | `colloids::stability` |
| Hamaker sphere-sphere | `hamaker_sphere_sphere(a_h: f64, r1: f64, r2: f64, d: f64) → f64` | $V = -A_{H}R_1R_2/[6d(R_1+R_2)]$ | `colloids::stability` |
| Hamaker sphere-surface | `hamaker_sphere_surface(a_h: f64, r: f64, d: f64) → f64` | $V = -A_{H}R/(6d)$ | `colloids::stability` |
| Debye length | `debye_length(epsilon_r: f64, t: f64, ionic_strength: f64) → f64` | $\kappa^{-1} = \sqrt{\varepsilon_r \varepsilon_0 k_BT/(2N_Ae^2I)}$ | `colloids::stability` |
| Electrostatic repulsion | `electrostatic_repulsion(epsilon_r: f64, r: f64, psi0: f64, kappa: f64, d: f64) → f64` | $V = 2\pi\varepsilon R\psi_0^2 e^{-\kappa d}$ | `colloids::stability` |
| Zeta potential (Smoluchowski) | `zeta_potential_smoluchowski(mobility: f64, viscosity: f64, epsilon: f64) → f64` | $\zeta = \mu\eta/\varepsilon$ | `colloids::stability` |
| Schulze-Hardy CCC | `schulze_hardy_ccc(z: i32) → f64` | $CCC \propto 1/z^6$ | `colloids::stability` |
| Critical coagulation conc. | `critical_coagulation_concentration(epsilon: f64, t: f64, psi0: f64, a_h: f64, z: f64) → f64` | CCC from DLVO parameters | `colloids::stability` |

### 21. Solid State — `chemistry::solid_state` — 17 pub fn

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| Fermi-Dirac distribution | `fermi_dirac(energy: f64, fermi_level: f64, t: f64) → f64` | $f(E) = 1/(1+e^{(E-E_F)/k_BT})$ | `solid_state::band_theory` |
| Band gap from absorption | `band_gap_from_absorption(wavelength_edge_nm: f64) → f64` | $E_g = hc/\lambda_{edge}$ | `solid_state::band_theory` |
| Intrinsic carrier conc. | `intrinsic_carrier_concentration(nc: f64, nv: f64, eg: f64, t: f64) → f64` | $n_i = \sqrt{N_C N_V} e^{-E_g/2k_BT}$ | `solid_state::band_theory` |
| Semiconductor conductivity | `conductivity_semiconductor(n: f64, mu_e: f64, p: f64, mu_h: f64) → f64` | $\sigma = e(n\mu_e + p\mu_h)$ | `solid_state::band_theory` |
| Hall coefficient | `hall_coefficient(n: f64, p: f64) → f64` | $R_H = (p-n)/[e(p+n)^2]$ | `solid_state::band_theory` |
| Doping carrier conc. | `doping_carrier_concentration(nd: f64, ni: f64) → (f64, f64)` | $n \approx N_D$, $p = n_i^2/n$ | `solid_state::band_theory` |
| Resistivity | `resistivity_from_conductivity(sigma: f64) → f64` | $\rho = 1/\sigma$ | `solid_state::band_theory` |
| Seebeck coefficient | `seebeck_coefficient(delta_v: f64, delta_t: f64) → f64` | $S = \Delta V / \Delta T$ | `solid_state::band_theory` |
| Ionic conductivity (Arrhenius) | `ionic_conductivity_arrhenius(sigma0: f64, ea: f64, t: f64) → f64` | $\sigma = \sigma_0 e^{-E_a/k_BT}$ | `solid_state::band_theory` |
| Schottky defect conc. | `schottky_defect_concentration(n_sites: f64, e_formation: f64, t: f64) → f64` | $n_s = N e^{-E_f/2k_BT}$ | `solid_state::defects` |
| Frenkel defect conc. | `frenkel_defect_concentration(n_sites: f64, n_interstitial: f64, e_formation: f64, t: f64) → f64` | $n_F = \sqrt{NN'} e^{-E_f/2k_BT}$ | `solid_state::defects` |
| Vacancy diffusion coeff. | `vacancy_diffusion_coefficient(d0: f64, q: f64, t: f64) → f64` | $D = D_0 e^{-Q/k_BT}$ | `solid_state::defects` |
| Color center absorption | `color_center_absorption(wavelength_nm: f64) → f64` | $E = hc/\lambda$ | `solid_state::defects` |
| Kröger-Vink equilibrium | `kroger_vink_equilibrium(k: f64, oxygen_pressure: f64, exponent: f64) → f64` | $[V_O] = K \cdot P_{O_2}^{n}$ | `solid_state::defects` |
| Nonstoichiometry delta | `nonstoichiometry_delta(mass_change: f64, molar_mass_oxygen: f64, sample_mass: f64, molar_mass_sample: f64) → f64` | $\delta = \Delta m M_{sample}/(M_O m_{sample})$ | `solid_state::defects` |
| Defect formation volume | `defect_formation_volume(lattice_param_defect: f64, lattice_param_perfect: f64) → f64` | $\Delta V = a_{def}^3 - a_0^3$ | `solid_state::defects` |
| Burgers vector magnitude | `burgers_vector_magnitude(a: f64, h: i32, k: i32, l: i32) → f64` | $b = a\sqrt{h^2+k^2+l^2}/2$ | `solid_state::defects` |

### 22. Computational — `chemistry::computational` — 17 pub fn

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| Thomas-Fermi kinetic energy | `thomas_fermi_kinetic_energy(density: &[f64], volume_element: f64) → f64` | $T_{TF} = C_F \int \rho^{5/3} d\mathbf{r}$ | `computational::dft` |
| LDA exchange energy | `exchange_energy_lda(density: &[f64], volume_element: f64) → f64` | $E_x = C_x \int \rho^{4/3} d\mathbf{r}$ | `computational::dft` |
| Hartree energy (DFT) | `hartree_energy(density: &[f64], potential: &[f64], volume_element: f64) → f64` | $E_H = \frac{1}{2}\int \rho(\mathbf{r})V_H(\mathbf{r})d\mathbf{r}$ | `computational::dft` |
| Nuclear attraction energy | `nuclear_attraction_energy(z: f64, r: f64) → f64` | $V_{ne} = -Z/r$ | `computational::dft` |
| Electron-nuclear energy | `electron_nuclear_energy(density: &[f64], distances: &[f64], z: f64, volume_element: f64) → f64` | $E_{ne} = -Z\int \rho/r \, d\mathbf{r}$ | `computational::dft` |
| XC potential (LDA) | `xc_potential_lda(density: f64) → f64` | $V_{xc} = dE_{xc}/d\rho$ | `computational::dft` |
| VWN correlation energy | `correlation_energy_vwn(rs: f64) → f64` | Vosko-Wilk-Nusair parametrization | `computational::dft` |
| Wigner-Seitz radius | `wigner_seitz_radius(density: f64) → f64` | $r_s = (3/4\pi\rho)^{1/3}$ | `computational::dft` |
| Kohn-Sham total energy | `kohn_sham_total_energy(eigenvalue_sum: f64, hartree: f64, xc_energy: f64, xc_potential_integral: f64) → f64` | $E_{KS} = \sum\varepsilon_i - E_H + E_{xc} - \int V_{xc}\rho\,d\mathbf{r}$ | `computational::dft` |
| Gaussian primitive | `gaussian_primitive(alpha: f64, r_sq: f64) → f64` | $g(\alpha, r) = e^{-\alpha r^2}$ | `computational::basis_sets` |
| s-orbital normalization | `normalization_s_orbital(alpha: f64) → f64` | $N = (2\alpha/\pi)^{3/4}$ | `computational::basis_sets` |
| p-orbital normalization | `normalization_p_orbital(alpha: f64) → f64` | $N = (128\alpha^5/\pi^3)^{1/4}$ | `computational::basis_sets` |
| Slater exponent | `slater_exponent(z_eff: f64, n: f64) → f64` | $\zeta = Z_{eff}/n$ | `computational::basis_sets` |
| Boys function F₀ | `boys_function_zero(t: f64) → f64` | $F_0(t) = \text{erf}(\sqrt{t})\sqrt{\pi}/(2\sqrt{t})$ | `computational::basis_sets` |
| Overlap integral 1s (Gauss) | `overlap_integral_1s(alpha1: f64, alpha2: f64, r_sq: f64) → f64` | $S = (\pi/(\alpha_1+\alpha_2))^{3/2} e^{-\alpha_1\alpha_2 r^2/(\alpha_1+\alpha_2)}$ | `computational::basis_sets` |
| Kinetic integral 1s | `kinetic_integral_1s(alpha1: f64, alpha2: f64, r_sq: f64) → f64` | $T = \alpha_1\alpha_2/(\alpha_1+\alpha_2)(3-2\alpha_1\alpha_2 r^2/(\alpha_1+\alpha_2))S$ | `computational::basis_sets` |
| STO-nG coefficients | `sto_ng_coefficients(n: usize) → Vec<(f64, f64)>` | Fitted (exponent, coefficient) pairs | `computational::basis_sets` |

### 23. Reaction Engineering — `chemistry::reaction_engineering` — 19 pub fn

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| CSTR volume | `cstr_volume(f_a0: f64, x: f64, r_a: f64) → f64` | $V = F_{A0} X / (-r_A)$ | `reaction_engineering::reactors` |
| PFR first-order conversion | `pfr_conversion_first_order(k: f64, tau: f64) → f64` | $X = 1 - e^{-k\tau}$ | `reaction_engineering::reactors` |
| CSTR first-order conversion | `cstr_conversion_first_order(k: f64, tau: f64) → f64` | $X = k\tau/(1+k\tau)$ | `reaction_engineering::reactors` |
| Batch time (1st order) | `batch_time_first_order(k: f64, x: f64) → f64` | $t = -\ln(1-X)/k$ | `reaction_engineering::reactors` |
| Batch time (2nd order) | `batch_time_second_order(k: f64, c0: f64, x: f64) → f64` | $t = X/[kC_0(1-X)]$ | `reaction_engineering::reactors` |
| Space time | `space_time(volume: f64, volumetric_flow: f64) → f64` | $\tau = V/v_0$ | `reaction_engineering::reactors` |
| Space velocity | `space_velocity(volumetric_flow: f64, volume: f64) → f64` | $SV = v_0/V$ | `reaction_engineering::reactors` |
| Levenspiel plot area | `levenspiel_plot_area(fa0_over_ra: &[f64], dx: f64) → f64` | $V/F_{A0} = \int (F_{A0}/(-r_A)) dX$ | `reaction_engineering::reactors` |
| CSTR in series conversion | `cstr_series_conversion(k: f64, tau_each: f64, n: u32) → f64` | $X = 1 - 1/(1+k\tau)^n$ | `reaction_engineering::reactors` |
| Damköhler number | `damkohler_number(k: f64, tau: f64, c0: f64, order: f64) → f64` | $Da = k\tau C_0^{n-1}$ | `reaction_engineering::design` |
| Selectivity | `selectivity(r_desired: f64, r_undesired: f64) → f64` | $S = r_D/r_U$ | `reaction_engineering::design` |
| Reactor yield | `yield_reactor(moles_desired: f64, moles_reacted: f64) → f64` | $Y = n_D/n_{reacted}$ | `reaction_engineering::design` |
| Overall selectivity | `overall_selectivity(moles_desired: f64, moles_all_products: f64) → f64` | $S_{ov} = n_D/n_{total}$ | `reaction_engineering::design` |
| Thiele modulus | `thiele_modulus(r: f64, k: f64, d_eff: f64) → f64` | $\phi = R\sqrt{k/D_{eff}}$ | `reaction_engineering::design` |
| Effectiveness factor (sphere) | `effectiveness_factor_sphere(phi: f64) → f64` | $\eta = 3(\phi\coth\phi-1)/\phi^2$ | `reaction_engineering::design` |
| Weisz-Prater criterion | `weisz_prater_criterion(r_obs: f64, r_particle: f64, d_eff: f64, c_s: f64) → f64` | $C_{WP} = (-r_{obs})R^2/(D_{eff}C_s)$ | `reaction_engineering::design` |
| RTD for CSTR | `residence_time_distribution_cstr(t: f64, tau: f64) → f64` | $E(t) = e^{-t/\tau}/\tau$ | `reaction_engineering::design` |
| RTD for PFR | `residence_time_distribution_pfr(t: f64, tau: f64) → f64` | $E(t) = \delta(t-\tau)$ | `reaction_engineering::design` |
| Recycle ratio effect | `recycle_ratio_effect(x_single: f64, recycle_ratio: f64) → f64` | Conversion with recycle stream | `reaction_engineering::design` |

### 24. Transport — `chemistry::transport` — 17 pub fn

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| Fick's first law | `fick_first_law(d: f64, dc_dx: f64) → f64` | $J = -D \, dc/dx$ | `transport::diffusion` |
| Fick's second law solution | `fick_second_law_solution(c0: f64, cs: f64, x: f64, d: f64, t: f64) → f64` | $C = C_s - (C_s-C_0)\text{erf}(x/2\sqrt{Dt})$ | `transport::diffusion` |
| Stokes-Einstein diffusion | `diffusion_coefficient_stokes_einstein(t: f64, viscosity: f64, r: f64) → f64` | $D = k_BT/(6\pi\eta r)$ | `transport::diffusion` |
| Wilke-Chang | `wilke_chang(t: f64, viscosity: f64, mw_solvent: f64, phi: f64, v_solute: f64) → f64` | $D = 7.4\times10^{-8}(\phi M_B)^{1/2}T/(\eta V_A^{0.6})$ | `transport::diffusion` |
| Knudsen diffusivity | `knudsen_diffusivity(r_pore: f64, t: f64, mw: f64) → f64` | $D_K = (2r/3)\sqrt{8RT/\pi M}$ | `transport::diffusion` |
| Effective diffusivity | `effective_diffusivity(d_bulk: f64, porosity: f64, tortuosity: f64) → f64` | $D_{eff} = D_b \varepsilon / \tau$ | `transport::diffusion` |
| Diffusion time estimate | `diffusion_time_estimate(length: f64, d: f64) → f64` | $t \sim L^2/D$ | `transport::diffusion` |
| Film mass transfer coeff. | `mass_transfer_coefficient_film(d: f64, delta: f64) → f64` | $k = D/\delta$ | `transport::mass_transfer` |
| Mass flux | `mass_flux(k: f64, c_bulk: f64, c_surface: f64) → f64` | $N = k(C_b - C_s)$ | `transport::mass_transfer` |
| Sherwood number | `sherwood_number(k: f64, l: f64, d: f64) → f64` | $Sh = kL/D$ | `transport::mass_transfer` |
| Schmidt number | `schmidt_number(viscosity: f64, density: f64, d: f64) → f64` | $Sc = \nu/D$ | `transport::mass_transfer` |
| Penetration theory coeff. | `penetration_theory_coefficient(d: f64, t_contact: f64) → f64` | $k = 2\sqrt{D/\pi t_c}$ | `transport::mass_transfer` |
| Surface renewal coeff. | `surface_renewal_coefficient(d: f64, s: f64) → f64` | $k = \sqrt{Ds}$ | `transport::mass_transfer` |
| Two-film overall coeff. | `two_film_theory_overall(k_g: f64, k_l: f64, henry: f64) → f64` | $1/K = 1/k_g + H/k_l$ | `transport::mass_transfer` |
| Mass transfer Biot | `mass_transfer_biot(k_ext: f64, r: f64, d_eff: f64) → f64` | $Bi_m = k_{ext}R/D_{eff}$ | `transport::mass_transfer` |
| Overall MT resistance | `overall_mass_transfer_resistance(resistances: &[f64]) → f64` | $1/K = \sum 1/k_i$ | `transport::mass_transfer` |
| NTU mass transfer | `ntu_mass_transfer(k: f64, a: f64, flow: f64) → f64` | $NTU = ka/F$ | `transport::mass_transfer` |

### 25. Environmental — `chemistry::environmental` — 18 pub fn

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| BOD | `biochemical_oxygen_demand(bod_ultimate: f64, k: f64, t: f64) → f64` | $BOD = L_0(1-e^{-kt})$ | `environmental::water` |
| COD | `chemical_oxygen_demand(sample_oxygen: f64, blank_oxygen: f64, volume: f64) → f64` | $COD = (O_{blank}-O_{sample})/V$ | `environmental::water` |
| DO saturation | `dissolved_oxygen_saturation(t: f64) → f64` | $DO_{sat}(T)$ empirical correlation | `environmental::water` |
| Streeter-Phelps | `streeter_phelps(d0: f64, l0: f64, kd: f64, kr: f64, t: f64) → f64` | $D = \frac{k_d L_0}{k_r-k_d}(e^{-k_dt}-e^{-k_rt})+D_0e^{-k_rt}$ | `environmental::water` |
| Critical point time | `critical_point_time(kd: f64, kr: f64, d0: f64, l0: f64) → f64` | $t_c = \frac{1}{k_r-k_d}\ln\frac{k_r}{k_d}[1-D_0(k_r-k_d)/(k_dL_0)]$ | `environmental::water` |
| Chlorine decay | `chlorine_decay(c0: f64, k: f64, t: f64) → f64` | $C = C_0 e^{-kt}$ | `environmental::water` |
| CT disinfection | `ct_disinfection(c: f64, t: f64) → f64` | $CT = C \times t$ | `environmental::water` |
| Total hardness | `hardness_total(ca_mg_l: f64, mg_mg_l: f64) → f64` | $H = 2.5[Ca^{2+}] + 4.1[Mg^{2+}]$ (as CaCO₃) | `environmental::water` |
| Langelier saturation index | `langelier_saturation_index(ph: f64, ph_s: f64) → f64` | $LSI = pH - pH_s$ | `environmental::water` |
| TDS from conductivity | `total_dissolved_solids_from_conductivity(conductivity_us: f64) → f64` | $TDS \approx 0.65 \times \kappa$ | `environmental::water` |
| Ozone formation rate | `ozone_formation_rate(k: f64, no2: f64, hv: f64) → f64` | $r = k[NO_2][h\nu]$ | `environmental::atmosphere` |
| Ozone destruction rate | `ozone_destruction_rate(k: f64, o3: f64, no: f64) → f64` | $r = k[O_3][NO]$ | `environmental::atmosphere` |
| Smog potential | `photochemical_smog_potential(voc: f64, nox: f64) → f64` | $P = [VOC]/[NO_x]$ ratio | `environmental::atmosphere` |
| Atmospheric lifetime | `atmospheric_lifetime(concentration: f64, removal_rate: f64) → f64` | $\tau = C/R$ | `environmental::atmosphere` |
| Global warming potential | `global_warming_potential(rf_gas: f64, lifetime_gas: f64, rf_co2: f64, lifetime_co2: f64) → f64` | $GWP = \int RF_{gas}/\int RF_{CO_2}$ | `environmental::atmosphere` |
| Henry law solubility | `henry_law_solubility(kh: f64, partial_pressure: f64) → f64` | $C = K_H P$ | `environmental::atmosphere` |
| ODP | `ozone_depletion_potential(cl_atoms: f64, lifetime: f64, mw: f64) → f64` | $ODP = n_{Cl} \tau / (M \cdot ref)$ | `environmental::atmosphere` |
| Photolysis rate constant | `photolysis_rate_constant(quantum_yield: f64, absorption: f64, actinic_flux: f64) → f64` | $J = \Phi \sigma F$ | `environmental::atmosphere` |

### 26. Green Chemistry — `chemistry::green_chemistry` — 18 pub fn

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| Atom economy | `atom_economy(mw_product: f64, mw_all_products: f64) → f64` | $AE = M_P / \sum M_i \times 100$ | `green_chemistry::metrics` |
| E-factor | `e_factor(mass_waste: f64, mass_product: f64) → f64` | $E = m_{waste}/m_{product}$ | `green_chemistry::metrics` |
| Process mass intensity | `process_mass_intensity(total_mass_input: f64, mass_product: f64) → f64` | $PMI = m_{total}/m_{product}$ | `green_chemistry::metrics` |
| Reaction mass efficiency | `reaction_mass_efficiency(mass_product: f64, total_mass_reactants: f64) → f64` | $RME = m_P/m_R \times 100$ | `green_chemistry::metrics` |
| Carbon efficiency | `carbon_efficiency(mass_carbon_product: f64, mass_carbon_reactants: f64) → f64` | $CE = C_{product}/C_{reactants} \times 100$ | `green_chemistry::metrics` |
| Mass productivity | `mass_productivity(mass_product: f64, total_mass_used: f64) → f64` | $MP = m_P/m_{total} \times 100$ | `green_chemistry::metrics` |
| Solvent intensity | `solvent_intensity(mass_solvent: f64, mass_product: f64) → f64` | $SI = m_{solvent}/m_P$ | `green_chemistry::metrics` |
| Water intensity | `water_intensity(mass_water: f64, mass_product: f64) → f64` | $WI = m_{water}/m_P$ | `green_chemistry::metrics` |
| Effective mass yield | `effective_mass_yield(mass_product: f64, mass_non_benign: f64) → f64` | $EMY = m_P/m_{non\text{-}benign} \times 100$ | `green_chemistry::metrics` |
| Renewable feedstock index | `renewable_feedstock_index(mass_renewable: f64, total_mass: f64) → f64` | $RFI = m_{renew}/m_{total} \times 100$ | `green_chemistry::metrics` |
| Turnover number | `catalyst_turnover_number(moles_product: f64, moles_catalyst: f64) → f64` | $TON = n_P/n_{cat}$ | `green_chemistry::principles` |
| Turnover frequency | `catalyst_turnover_frequency(ton: f64, time: f64) → f64` | $TOF = TON/t$ | `green_chemistry::principles` |
| Energy efficiency | `energy_efficiency(useful_energy: f64, total_energy: f64) → f64` | $\eta = E_{useful}/E_{total} \times 100$ | `green_chemistry::principles` |
| Selectivity | `selectivity(moles_desired: f64, moles_converted: f64) → f64` | $S = n_D/n_{conv}$ | `green_chemistry::principles` |
| Yield from S×X | `yield_from_selectivity_conversion(selectivity_frac: f64, conversion_frac: f64) → f64` | $Y = S \times X$ | `green_chemistry::principles` |
| Stoichiometric factor | `stoichiometric_factor(actual_mass_reactant: f64, stoichiometric_mass: f64) → f64` | $SF = m_{actual}/m_{stoich}$ | `green_chemistry::principles` |
| Environmental quotient | `environmental_quotient(e_factor: f64, unfriendliness: f64) → f64` | $EQ = E \times Q$ | `green_chemistry::principles` |
| Mass index | `mass_index(total_mass_input: f64, total_mass_output: f64) → f64` | $MI = m_{in}/m_{out}$ | `green_chemistry::principles` |
