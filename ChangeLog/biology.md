# Biology — ChangeLog

---

## v0.0.3

No new functions — see `testing.md` for test details.

---

## v0.0.2

### Changes

| Metric | Value |
|---|---|
| Submodules rewritten | 44 / 44 |

---

## v0.0.1

Module: `src/biology/` — 44 submodules, 185+ files, 2422 pub fn

### 1️⃣ population/ — Population Dynamics — 49 pub fn

#### age_structure.rs (7 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `leslie_matrix_multiply` | `fn leslie_matrix_multiply(matrix: &[Vec<f64>], population: &[f64]) → Vec<f64>` | $\mathbf{n}_{t+1} = \mathbf{L} \cdot \mathbf{n}_t$ — Leslie matrix projection | `biology::population::age_structure` |
| `stable_age_distribution` | `fn stable_age_distribution(fecundities: &[f64], survivals: &[f64]) → Vec<f64>` | Stable age distribution eigenvector | `biology::population::age_structure` |
| `cohort_generation_time` | `fn cohort_generation_time(age_fecundity: &[(f64, f64)], lambda: f64) → f64` | $T = \sum x l_x m_x / R_0$ | `biology::population::age_structure` |
| `reproductive_value` | `fn reproductive_value(lx: &[f64], mx: &[f64], lambda: f64) → Vec<f64>` | Fisher's reproductive value | `biology::population::age_structure` |
| `euler_lotka_lambda` | `fn euler_lotka_lambda(lx: &[f64], mx: &[f64]) → f64` | $\sum l_x m_x e^{-rx} = 1$ | `biology::population::age_structure` |
| `sensitivity_element` | `fn sensitivity_element(v_i: f64, w_j: f64, vw_dot: f64) → f64` | $s_{ij} = v_i w_j / \langle \mathbf{v}, \mathbf{w} \rangle$ | `biology::population::age_structure` |
| `elasticity_element` | `fn elasticity_element(sensitivity: f64, a_ij: f64, lambda: f64) → f64` | $e_{ij} = (a_{ij}/\lambda) s_{ij}$ | `biology::population::age_structure` |

#### epidemiology.rs (10 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `sir_model` | `fn sir_model(s: f64, i: f64, r: f64, beta: f64, gamma: f64) → (f64, f64, f64)` | $dS=-\beta SI,\ dI=\beta SI-\gamma I$ | `biology::population::epidemiology` |
| `sir_solve` | `fn sir_solve(s0: f64, i0: f64, r0: f64, beta: f64, gamma: f64, dt: f64, steps: usize) → Vec<(f64, f64, f64)>` | SIR numerical integration | `biology::population::epidemiology` |
| `seir_model` | `fn seir_model(s: f64, e: f64, i: f64, r: f64, beta: f64, sigma: f64, gamma: f64) → (f64, f64, f64, f64)` | SEIR compartmental model | `biology::population::epidemiology` |
| `seir_solve` | `fn seir_solve(s0: f64, e0: f64, i0: f64, r0: f64, beta: f64, sigma: f64, gamma: f64, dt: f64, steps: usize) → Vec<(f64, f64, f64, f64)>` | SEIR numerical integration | `biology::population::epidemiology` |
| `sis_model` | `fn sis_model(s: f64, i: f64, beta: f64, gamma: f64) → (f64, f64)` | SIS endemic model | `biology::population::epidemiology` |
| `sirs_model` | `fn sirs_model(s: f64, i: f64, r: f64, beta: f64, gamma: f64, xi: f64) → (f64, f64, f64)` | SIRS with waning immunity | `biology::population::epidemiology` |
| `basic_reproduction_number` | `fn basic_reproduction_number(beta: f64, gamma: f64) → f64` | $R_0 = \beta / \gamma$ | `biology::population::epidemiology` |
| `herd_immunity_threshold` | `fn herd_immunity_threshold(r0: f64) → f64` | $p_c = 1 - 1/R_0$ | `biology::population::epidemiology` |
| `final_size_equation` | `fn final_size_equation(r0: f64, tolerance: f64, max_iter: usize) → f64` | Epidemic final size equation | `biology::population::epidemiology` |
| `generation_time` | `fn generation_time(incubation: f64, infectious_period: f64) → f64` | Mean generation time | `biology::population::epidemiology` |

#### growth.rs (11 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `logistic_growth` | `fn logistic_growth(n: f64, r: f64, k: f64) → f64` | $dN/dt = rN(1-N/K)$ | `biology::population::growth` |
| `logistic_solve` | `fn logistic_solve(n0: f64, r: f64, k: f64, dt: f64, steps: usize) → Vec<f64>` | Logistic numerical integration | `biology::population::growth` |
| `exponential_growth` | `fn exponential_growth(n0: f64, r: f64, t: f64) → f64` | $N(t) = N_0 e^{rt}$ | `biology::population::growth` |
| `gompertz` | `fn gompertz(n: f64, a: f64, b: f64, k: f64) → f64` | Gompertz growth model | `biology::population::growth` |
| `allee_effect` | `fn allee_effect(n: f64, r: f64, k: f64, a: f64) → f64` | Growth with Allee threshold | `biology::population::growth` |
| `beverton_holt` | `fn beverton_holt(n: f64, r: f64, k: f64) → f64` | Beverton-Holt recruitment | `biology::population::growth` |
| `ricker` | `fn ricker(n: f64, r: f64, k: f64) → f64` | $N_{t+1}=N_t e^{r(1-N_t/K)}$ | `biology::population::growth` |
| `doubling_time` | `fn doubling_time(r: f64) → f64` | $t_d = \ln 2 / r$ | `biology::population::growth` |
| `von_bertalanffy` | `fn von_bertalanffy(l_inf: f64, k: f64, t: f64, t0: f64) → f64` | $L(t) = L_\infty(1-e^{-k(t-t_0)})$ | `biology::population::growth` |
| `theta_logistic` | `fn theta_logistic(n: f64, r: f64, k: f64, theta: f64) → f64` | $dN/dt = rN(1-(N/K)^\theta)$ | `biology::population::growth` |
| `moran_process_fixation` | `fn moran_process_fixation(n: usize, r: f64) → f64` | $\rho = (1-1/r)/(1-1/r^N)$ | `biology::population::growth` |

#### predation.rs (10 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `lotka_volterra` | `fn lotka_volterra(prey: f64, pred: f64, alpha: f64, beta: f64, delta: f64, gamma: f64) → (f64, f64)` | Lotka-Volterra predator-prey | `biology::population::predation` |
| `lotka_volterra_solve` | `fn lotka_volterra_solve(prey0: f64, pred0: f64, alpha: f64, beta: f64, delta: f64, gamma: f64, dt: f64, steps: usize) → Vec<(f64, f64)>` | Lotka-Volterra integration | `biology::population::predation` |
| `competitive_lotka_volterra` | `fn competitive_lotka_volterra(n1: f64, n2: f64, r1: f64, r2: f64, k1: f64, k2: f64, a12: f64, a21: f64) → (f64, f64)` | Two-species competition | `biology::population::predation` |
| `holling_type_ii` | `fn holling_type_ii(prey: f64, attack_rate: f64, handling_time: f64) → f64` | $f = aN/(1+ahN)$ — Type II | `biology::population::predation` |
| `holling_type_iii` | `fn holling_type_iii(prey: f64, attack_rate: f64, handling_time: f64) → f64` | $f = aN^2/(1+ahN^2)$ — Type III | `biology::population::predation` |
| `rosenzweig_macarthur` | `fn rosenzweig_macarthur(prey: f64, pred: f64, r: f64, k: f64, a: f64, h: f64, e: f64, d: f64) → (f64, f64)` | Rosenzweig-MacArthur model | `biology::population::predation` |
| `beddington_deangelis` | `fn beddington_deangelis(prey: f64, pred: f64, a: f64, b: f64, c: f64) → f64` | Beddington-DeAngelis response | `biology::population::predation` |
| `ratio_dependent` | `fn ratio_dependent(prey: f64, pred: f64, a: f64, h: f64) → f64` | Ratio-dependent functional response | `biology::population::predation` |
| `intraguild_predation` | `fn intraguild_predation(prey: f64, meso: f64, top: f64, r: f64, k: f64, a_mp: f64, a_tp: f64, a_tm: f64, e_mp: f64, e_tp: f64, e_tm: f64, d_m: f64, d_t: f64) → (f64, f64, f64)` | Three-species IGP model | `biology::population::predation` |
| `apparent_competition` | `fn apparent_competition(prey1: f64, prey2: f64, pred: f64, a1: f64, a2: f64, r1: f64, r2: f64, k1: f64, k2: f64, e: f64, d: f64) → (f64, f64, f64)` | Apparent competition model | `biology::population::predation` |

#### spatial.rs (11 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `diffusion_dispersal` | `fn diffusion_dispersal(density: f64, diffusion_coeff: f64, gradient: f64) → f64` | $J = -D \nabla n$ — Fick's law | `biology::population::spatial` |
| `reaction_diffusion_fisher` | `fn reaction_diffusion_fisher(n: f64, r: f64, k: f64, d: f64, laplacian: f64) → f64` | Fisher-KPP equation | `biology::population::spatial` |
| `fisher_wave_speed` | `fn fisher_wave_speed(r: f64, d: f64) → f64` | $c = 2\sqrt{rD}$ | `biology::population::spatial` |
| `range_expansion_rate` | `fn range_expansion_rate(diffusion: f64, growth_rate: f64) → f64` | Range expansion velocity | `biology::population::spatial` |
| `stepping_stone_migration` | `fn stepping_stone_migration(source_density: f64, target_density: f64, migration_rate: f64) → f64` | Stepping-stone migration | `biology::population::spatial` |
| `isolation_by_distance` | `fn isolation_by_distance(fst: f64) → f64` | $M = (1-F_{ST})/(4F_{ST})$ | `biology::population::spatial` |
| `landscape_resistance` | `fn landscape_resistance(distance: f64, resistance_cost: f64) → f64` | Landscape resistance distance | `biology::population::spatial` |
| `gravity_model_migration` | `fn gravity_model_migration(pop_source: f64, pop_dest: f64, distance: f64, alpha: f64) → f64` | Gravity model of migration | `biology::population::spatial` |
| `corridor_effectiveness` | `fn corridor_effectiveness(width: f64, length: f64, habitat_quality: f64, species_mobility: f64) → f64` | Corridor effectiveness index | `biology::population::spatial` |
| `allee_effect_spatial` | `fn allee_effect_spatial(density: f64, allee_threshold: f64, r: f64, k: f64) → f64` | Spatial Allee effect | `biology::population::spatial` |
| `kernel_based_dispersal` | `fn kernel_based_dispersal(distance: f64, alpha: f64, shape: f64) → f64` | Dispersal kernel | `biology::population::spatial` |

### 2️⃣ ecology/ — Ecology — 50 pub fn

#### diversity.rs (9 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `shannon_diversity` | `fn shannon_diversity(abundances: &[f64]) → f64` | $H = -\sum p_i \ln p_i$ | `biology::ecology::diversity` |
| `simpson_diversity` | `fn simpson_diversity(abundances: &[f64]) → f64` | $D = 1 - \sum p_i^2$ | `biology::ecology::diversity` |
| `inverse_simpson` | `fn inverse_simpson(abundances: &[f64]) → f64` | $1 / \sum p_i^2$ | `biology::ecology::diversity` |
| `species_richness` | `fn species_richness(abundances: &[f64]) → usize` | Number of species | `biology::ecology::diversity` |
| `pielou_evenness` | `fn pielou_evenness(abundances: &[f64]) → f64` | $J = H / \ln S$ | `biology::ecology::diversity` |
| `berger_parker` | `fn berger_parker(abundances: &[f64]) → f64` | Berger-Parker dominance | `biology::ecology::diversity` |
| `margalef_richness` | `fn margalef_richness(species: usize, total_individuals: f64) → f64` | $D_M = (S-1)/\ln N$ | `biology::ecology::diversity` |
| `chao1` | `fn chao1(observed: usize, singletons: usize, doubletons: usize) → f64` | Chao1 richness estimator | `biology::ecology::diversity` |
| `hill_number` | `fn hill_number(abundances: &[f64], q: f64) → f64` | ${}^qD = (\sum p_i^q)^{1/(1-q)}$ | `biology::ecology::diversity` |

#### dynamics.rs (7 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `trophic_cascade` | `fn trophic_cascade(levels: &[f64], growth_rates: &[f64], carrying_capacities: &[f64], interaction_strengths: &[f64], dt: f64, steps: usize) → Vec<Vec<f64>>` | Multi-level trophic cascade | `biology::ecology::dynamics` |
| `reaction_diffusion_1d` | `fn reaction_diffusion_1d(u: &mut [f64], v: &mut [f64], du: f64, dv: f64, f_coeff: f64, k: f64, dx: f64, dt: f64, steps: usize)` | 1D reaction-diffusion | `biology::ecology::dynamics` |
| `species_area` | `fn species_area(c: f64, z: f64, area: f64) → f64` | $S = cA^z$ | `biology::ecology::dynamics` |
| `island_biogeography_equilibrium` | `fn island_biogeography_equilibrium(immigration_rate: f64, extinction_rate: f64) → f64` | MacArthur-Wilson equilibrium | `biology::ecology::dynamics` |
| `carrying_capacity_from_resources` | `fn carrying_capacity_from_resources(resource: f64, consumption_per_capita: f64) → f64` | $K = R / c$ | `biology::ecology::dynamics` |
| `succession_model` | `fn succession_model(biomass: &[f64], growth_rates: &[f64], capacities: &[f64], competition: &[Vec<f64>], dt: f64, steps: usize) → Vec<Vec<f64>>` | Ecological succession | `biology::ecology::dynamics` |
| `dispersal_kernel_gaussian` | `fn dispersal_kernel_gaussian(distance: f64, sigma: f64) → f64` | Gaussian dispersal kernel | `biology::ecology::dynamics` |

#### ecosystem.rs (11 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `net_primary_productivity` | `fn net_primary_productivity(gpp: f64, autotrophic_respiration: f64) → f64` | $NPP = GPP - R_a$ | `biology::ecology::ecosystem` |
| `net_ecosystem_productivity` | `fn net_ecosystem_productivity(npp: f64, heterotrophic_respiration: f64) → f64` | $NEP = NPP - R_h$ | `biology::ecology::ecosystem` |
| `carbon_use_efficiency` | `fn carbon_use_efficiency(npp: f64, gpp: f64) → f64` | $CUE = NPP / GPP$ | `biology::ecology::ecosystem` |
| `nitrogen_mineralization` | `fn nitrogen_mineralization(organic_n: f64, microbial_activity: f64, temperature_factor: f64) → f64` | N mineralization rate | `biology::ecology::ecosystem` |
| `nutrient_use_efficiency` | `fn nutrient_use_efficiency(biomass_produced: f64, nutrient_absorbed: f64) → f64` | NUE ratio | `biology::ecology::ecosystem` |
| `liebig_minimum` | `fn liebig_minimum(nutrients: &[f64], requirements: &[f64]) → f64` | Liebig's law of the minimum | `biology::ecology::ecosystem` |
| `decomposition_rate` | `fn decomposition_rate(initial_mass: f64, k: f64, t: f64) → f64` | $M(t) = M_0 e^{-kt}$ | `biology::ecology::ecosystem` |
| `soil_respiration` | `fn soil_respiration(temperature: f64, moisture: f64, q10: f64, r_ref: f64) → f64` | $R = R_{ref} Q_{10}^{(T-T_{ref})/10}$ | `biology::ecology::ecosystem` |
| `evapotranspiration_penman_monteith` | `fn evapotranspiration_penman_monteith(net_radiation: f64, soil_heat_flux: f64, air_temp: f64, vpd: f64, wind_speed: f64, surface_resistance: f64) → f64` | Penman-Monteith ET | `biology::ecology::ecosystem` |
| `water_use_efficiency` | `fn water_use_efficiency(carbon_assimilated: f64, water_transpired: f64) → f64` | $WUE = C / E$ | `biology::ecology::ecosystem` |
| `litter_bag_decomposition` | `fn litter_bag_decomposition(initial_mass: f64, remaining_mass: f64, time: f64) → f64` | Litter decomposition constant | `biology::ecology::ecosystem` |

#### food_web.rs (13 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `lotka_volterra_competition` | `fn lotka_volterra_competition(n1: f64, n2: f64, r1: f64, r2: f64, k1: f64, k2: f64, alpha12: f64, alpha21: f64) → (f64, f64)` | Two-species competition | `biology::ecology::food_web` |
| `lotka_volterra_predator_prey` | `fn lotka_volterra_predator_prey(prey: f64, predator: f64, r: f64, a: f64, b: f64, m: f64) → (f64, f64)` | Predator-prey dynamics | `biology::ecology::food_web` |
| `rosenzweig_macarthur` | `fn rosenzweig_macarthur(prey: f64, predator: f64, r: f64, k: f64, a: f64, h: f64, e: f64, m: f64) → (f64, f64)` | Rosenzweig-MacArthur model | `biology::ecology::food_web` |
| `type_ii_functional_response` | `fn type_ii_functional_response(prey_density: f64, attack_rate: f64, handling_time: f64) → f64` | $f = aN/(1+ahN)$ | `biology::ecology::food_web` |
| `type_iii_functional_response` | `fn type_iii_functional_response(prey_density: f64, attack_rate: f64, handling_time: f64, exponent: f64) → f64` | Type III functional response | `biology::ecology::food_web` |
| `nutrient_cycling` | `fn nutrient_cycling(nutrient: f64, producers: f64, decomposers: f64, uptake_rate: f64, mortality_rate: f64, decomposition_rate: f64) → (f64, f64, f64)` | N-P-D nutrient cycling | `biology::ecology::food_web` |
| `disturbance_regime` | `fn disturbance_regime(biomass: f64, disturbance_intensity: f64, return_interval: f64, time_since: f64) → f64` | Disturbance regime dynamics | `biology::ecology::food_web` |
| `intermediate_disturbance_diversity` | `fn intermediate_disturbance_diversity(disturbance_frequency: f64, max_diversity: f64, optimal_frequency: f64) → f64` | IDH diversity model | `biology::ecology::food_web` |
| `metapopulation_levins` | `fn metapopulation_levins(p: f64, colonization: f64, extinction: f64) → f64` | $dp/dt = cp(1-p) - ep$ | `biology::ecology::food_web` |
| `source_sink_dynamics` | `fn source_sink_dynamics(source_emigration: f64, sink_mortality: f64, sink_immigration: f64) → f64` | Source-sink dynamics | `biology::ecology::food_web` |
| `food_web_connectance` | `fn food_web_connectance(links: usize, species: usize) → f64` | $C = L / S^2$ | `biology::ecology::food_web` |
| `trophic_level` | `fn trophic_level(diet_trophic_levels: &[f64], diet_fractions: &[f64]) → f64` | Mean trophic level | `biology::ecology::food_web` |
| `lindeman_efficiency` | `fn lindeman_efficiency(energy_n_plus_1: f64, energy_n: f64) → f64` | Lindeman trophic efficiency | `biology::ecology::food_web` |

#### similarity.rs (10 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `bray_curtis` | `fn bray_curtis(a: &[f64], b: &[f64]) → f64` | Bray-Curtis dissimilarity | `biology::ecology::similarity` |
| `jaccard` | `fn jaccard(a: &[bool], b: &[bool]) → f64` | $J = |A \cap B| / |A \cup B|$ | `biology::ecology::similarity` |
| `sorensen` | `fn sorensen(a: &[bool], b: &[bool]) → f64` | Sørensen similarity | `biology::ecology::similarity` |
| `morisita_horn` | `fn morisita_horn(a: &[f64], b: &[f64]) → f64` | Morisita-Horn overlap | `biology::ecology::similarity` |
| `euclidean_distance` | `fn euclidean_distance(a: &[f64], b: &[f64]) → f64` | $d = \sqrt{\sum(a_i - b_i)^2}$ | `biology::ecology::similarity` |
| `whittaker_beta` | `fn whittaker_beta(gamma: usize, alpha_mean: f64) → f64` | $\beta_W = \gamma / \bar{\alpha} - 1$ | `biology::ecology::similarity` |
| `horn_overlap` | `fn horn_overlap(a: &[f64], b: &[f64]) → f64` | Horn's overlap index | `biology::ecology::similarity` |
| `chao_jaccard` | `fn chao_jaccard(shared: usize, a_only: usize, b_only: usize, n_a: usize, n_b: usize) → f64` | Chao-Jaccard estimator | `biology::ecology::similarity` |
| `manhattan_distance` | `fn manhattan_distance(a: &[f64], b: &[f64]) → f64` | $d = \sum |a_i - b_i|$ | `biology::ecology::similarity` |
| `canberra_distance` | `fn canberra_distance(a: &[f64], b: &[f64]) → f64` | Canberra distance | `biology::ecology::similarity` |

### 3️⃣ biogeography/ — Biogeography — 52 pub fn

#### climate.rs (10 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `species_range_overlap` | `fn species_range_overlap(range_a: (f64, f64), range_b: (f64, f64)) → f64` | Range overlap fraction | `biology::biogeography::climate` |
| `range_size_latitude` | `fn range_size_latitude(area: f64) → f64` | Range size from latitude | `biology::biogeography::climate` |
| `elevational_diversity_gradient` | `fn elevational_diversity_gradient(elevation: f64, peak_elevation: f64, max_richness: f64) → f64` | Elevation-diversity hump | `biology::biogeography::climate` |
| `biome_niche_model` | `fn biome_niche_model(temperature: f64, precipitation: f64, t_opt: f64, p_opt: f64, t_width: f64, p_width: f64) → f64` | Bioclimatic niche model | `biology::biogeography::climate` |
| `regional_endemism_index` | `fn regional_endemism_index(endemic_species: usize, total_species: usize) → f64` | Endemic/total ratio | `biology::biogeography::climate` |
| `latitudinal_diversity_gradient` | `fn latitudinal_diversity_gradient(latitude: f64, max_richness: f64, steepness: f64) → f64` | Latitudinal diversity gradient | `biology::biogeography::climate` |
| `range_shift_velocity` | `fn range_shift_velocity(temp_change_rate: f64, spatial_temp_gradient: f64) → f64` | Climate velocity $v = dT/dt / (dT/dx)$ | `biology::biogeography::climate` |
| `climate_envelope_suitability` | `fn climate_envelope_suitability(temp: f64, precip: f64, temp_min: f64, temp_max: f64, precip_min: f64, precip_max: f64) → f64` | Climate envelope suitability | `biology::biogeography::climate` |
| `refugia_persistence` | `fn refugia_persistence(area: f64, min_viable_area: f64, climate_stability: f64) → f64` | Refugia persistence probability | `biology::biogeography::climate` |
| `wallace_line_effect` | `fn wallace_line_effect(dispersal_ability: f64, barrier_width: f64) → f64` | Biogeographic barrier effect | `biology::biogeography::climate` |

#### connectivity.rs (14 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `landscape_resistance` | `fn landscape_resistance(cost_surface: &[Vec<f64>], path: &[(usize, usize)]) → f64` | Landscape resistance cost | `biology::biogeography::connectivity` |
| `isolation_by_distance` | `fn isolation_by_distance(genetic_dist: &[f64], geographic_dist: &[f64]) → f64` | IBD regression | `biology::biogeography::connectivity` |
| `connectivity_index` | `fn connectivity_index(patch_areas: &[f64], distances: &[Vec<f64>], alpha: f64) → Vec<f64>` | Hanski connectivity index | `biology::biogeography::connectivity` |
| `metapopulation_incidence` | `fn metapopulation_incidence(connectivity: f64, e: f64, colonization_coeff: f64) → f64` | Incidence function model | `biology::biogeography::connectivity` |
| `habitat_fragmentation_index` | `fn habitat_fragmentation_index(total_area: f64, n_patches: usize, perimeter_sum: f64) → f64` | Fragmentation index | `biology::biogeography::connectivity` |
| `effective_mesh_size` | `fn effective_mesh_size(patch_areas: &[f64], total_area: f64) → f64` | $m_{eff} = \sum A_i^2 / A_{total}$ | `biology::biogeography::connectivity` |
| `proximity_index` | `fn proximity_index(focal_area: f64, neighbor_areas: &[f64], distances: &[f64]) → f64` | Proximity index | `biology::biogeography::connectivity` |
| `corridor_effectiveness` | `fn corridor_effectiveness(corridor_length: f64, corridor_width: f64, matrix_resistance: f64) → f64` | Corridor effectiveness | `biology::biogeography::connectivity` |
| `graph_connectivity` | `fn graph_connectivity(adjacency: &[Vec<bool>]) → f64` | Graph connectivity metric | `biology::biogeography::connectivity` |
| `stepping_stone_migration` | `fn stepping_stone_migration(populations: &[f64], migration_rate: f64) → Vec<f64>` | Stepping-stone migration | `biology::biogeography::connectivity` |
| `least_cost_distance` | `fn least_cost_distance(cost_surface: &[Vec<f64>], start: (usize, usize), end: (usize, usize)) → f64` | Least-cost path distance | `biology::biogeography::connectivity` |
| `resistance_distance` | `fn resistance_distance(conductances: &[Vec<f64>], node_a: usize, node_b: usize) → f64` | Circuit-theory resistance | `biology::biogeography::connectivity` |
| `patch_cohesion` | `fn patch_cohesion(patch_perimeters: &[f64], patch_areas: &[f64], total_cells: f64) → f64` | Patch cohesion index | `biology::biogeography::connectivity` |
| `circuitscape_effective_resistance` | `fn circuitscape_effective_resistance(node_conductances: &[f64]) → f64` | Circuitscape effective resistance | `biology::biogeography::connectivity` |

#### distribution.rs (17 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `dispersal_kernel_exponential` | `fn dispersal_kernel_exponential(distance: f64, mean_dispersal: f64) → f64` | $k(d) = e^{-d/\bar{d}}$ | `biology::biogeography::distribution` |
| `dispersal_kernel_2dt` | `fn dispersal_kernel_2dt(distance: f64, a: f64, p: f64) → f64` | Clark 2Dt kernel | `biology::biogeography::distribution` |
| `range_shift_rate` | `fn range_shift_rate(warming_rate: f64, lapse_rate: f64) → f64` | Range shift velocity | `biology::biogeography::distribution` |
| `latitudinal_gradient` | `fn latitudinal_gradient(species_tropical: f64, species_polar: f64, lat_range: f64) → f64` | Latitudinal species gradient | `biology::biogeography::distribution` |
| `altitudinal_gradient` | `fn altitudinal_gradient(species_low: f64, species_high: f64, alt_range: f64) → f64` | Altitudinal species gradient | `biology::biogeography::distribution` |
| `bioclimatic_envelope` | `fn bioclimatic_envelope(temp: f64, precip: f64, temp_min: f64, temp_max: f64, precip_min: f64, precip_max: f64) → f64` | Bioclimatic envelope model | `biology::biogeography::distribution` |
| `species_area_relationship` | `fn species_area_relationship(c: f64, z: f64, area: f64) → f64` | $S = cA^z$ | `biology::biogeography::distribution` |
| `endemism_index` | `fn endemism_index(endemic_species: usize, total_species: usize) → f64` | Endemic species fraction | `biology::biogeography::distribution` |
| `climate_velocity` | `fn climate_velocity(temp_change_rate: f64, spatial_gradient: f64) → f64` | $v = dT/dt / (dT/dx)$ | `biology::biogeography::distribution` |
| `habitat_suitability_index` | `fn habitat_suitability_index(variables: &[f64], optima: &[f64], tolerances: &[f64]) → f64` | HSI composite | `biology::biogeography::distribution` |
| `island_equilibrium_richness` | `fn island_equilibrium_richness(immigration_max: f64, extinction_max: f64, area: f64, distance: f64) → f64` | Island equilibrium richness | `biology::biogeography::distribution` |
| `nestedness_temperature` | `fn nestedness_temperature(presence_matrix: &[Vec<bool>]) → f64` | Nestedness temperature | `biology::biogeography::distribution` |
| `mid_domain_effect` | `fn mid_domain_effect(domain_size: f64, range_size: f64) → f64` | Mid-domain effect | `biology::biogeography::distribution` |
| `beta_diversity_whittaker` | `fn beta_diversity_whittaker(gamma: f64, alpha_mean: f64) → f64` | $\beta_W = \gamma / \bar{\alpha} - 1$ | `biology::biogeography::distribution` |
| `beta_diversity_sorensen` | `fn beta_diversity_sorensen(shared: f64, unique_a: f64, unique_b: f64) → f64` | Sørensen beta diversity | `biology::biogeography::distribution` |
| `rapoport_rule` | `fn rapoport_rule(range_sizes: &[f64], latitudes: &[f64]) → f64` | Rapoport's rule correlation | `biology::biogeography::distribution` |
| `occupancy_frequency` | `fn occupancy_frequency(presences: &[bool]) → f64` | Occupancy frequency | `biology::biogeography::distribution` |

#### island.rs (11 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `island_species_area` | `fn island_species_area(c: f64, z: f64, area: f64) → f64` | $S = cA^z$ | `biology::biogeography::island` |
| `island_immigration_rate` | `fn island_immigration_rate(s: f64, p: f64, i_max: f64) → f64` | Immigration rate | `biology::biogeography::island` |
| `island_extinction_rate` | `fn island_extinction_rate(s: f64, e_max: f64) → f64` | Extinction rate | `biology::biogeography::island` |
| `macarthur_wilson_equilibrium` | `fn macarthur_wilson_equilibrium(i_max: f64, e_max: f64, p: f64) → f64` | MacArthur-Wilson equilibrium | `biology::biogeography::island` |
| `macarthur_wilson_turnover` | `fn macarthur_wilson_turnover(i_max: f64, e_max: f64, p: f64) → f64` | Turnover rate | `biology::biogeography::island` |
| `distance_decay` | `fn distance_decay(similarity_0: f64, decay_rate: f64, distance: f64) → f64` | Distance-decay similarity | `biology::biogeography::island` |
| `rescue_effect` | `fn rescue_effect(extinction_base: f64, immigration: f64) → f64` | Rescue effect | `biology::biogeography::island` |
| `target_effect` | `fn target_effect(immigration_base: f64, area: f64, area_ref: f64) → f64` | Target area effect | `biology::biogeography::island` |
| `species_isolation_index` | `fn species_isolation_index(distances: &[f64]) → f64` | Isolation index | `biology::biogeography::island` |
| `area_effect_on_extinction` | `fn area_effect_on_extinction(e_base: f64, area: f64, z: f64) → f64` | Area-extinction relationship | `biology::biogeography::island` |
| `habitat_diversity` | `fn habitat_diversity(area: f64, k: f64) → f64` | Habitat diversity from area | `biology::biogeography::island` |

### 4️⃣ marine_biology/ — Marine Biology — 62 pub fn

#### coral.rs (10 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `bleaching_thermal_threshold` | `fn bleaching_thermal_threshold(sst: f64, mmm: f64) → f64` | Coral bleaching thermal stress threshold | `biology::marine_biology::coral` |
| `dhw_accumulation` | `fn dhw_accumulation(dhw_accumulated: f64, hotspot: f64, dt_weeks: f64) → f64` | Degree Heating Weeks accumulation | `biology::marine_biology::coral` |
| `calcification_rate` | `fn calcification_rate(omega_aragonite: f64, temperature: f64, light: f64, max_rate: f64) → f64` | Coral calcification rate | `biology::marine_biology::coral` |
| `reef_rugosity` | `fn reef_rugosity(surface_distance: f64, linear_distance: f64) → f64` | Reef structural complexity index | `biology::marine_biology::coral` |
| `coral_recruitment_rate` | `fn coral_recruitment_rate(larval_supply: f64, settlement_rate: f64, post_settlement_survival: f64, available_substrate: f64) → f64` | Coral recruitment rate | `biology::marine_biology::coral` |
| `symbiodinium_density` | `fn symbiodinium_density(photosynthesis_rate: f64, respiration_rate: f64, expulsion_rate: f64, dt: f64, current_density: f64) → f64` | Symbiodinium population dynamics | `biology::marine_biology::coral` |
| `ocean_acidification_saturation` | `fn ocean_acidification_saturation(ca_conc: f64, co3_conc: f64, ksp: f64) → f64` | $\Omega = [\text{Ca}^{2+}][\text{CO}_3^{2-}] / K_{sp}$ | `biology::marine_biology::coral` |
| `mangrove_carbon_sequestration` | `fn mangrove_carbon_sequestration(biomass: f64, carbon_fraction: f64, growth_rate: f64) → f64` | Mangrove carbon sequestration | `biology::marine_biology::coral` |
| `seagrass_light_attenuation` | `fn seagrass_light_attenuation(surface_irradiance: f64, attenuation_coeff: f64, depth: f64) → f64` | $I(z) = I_0 e^{-kz}$ | `biology::marine_biology::coral` |
| `marine_protected_area_spillover` | `fn marine_protected_area_spillover(biomass_inside: f64, biomass_outside: f64, perimeter_area_ratio: f64, diffusion: f64) → f64` | MPA spillover effect | `biology::marine_biology::coral` |

#### fisheries.rs (20 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `trophic_transfer_efficiency` | `fn trophic_transfer_efficiency(production_upper: f64, production_lower: f64) → f64` | Trophic transfer efficiency | `biology::marine_biology::fisheries` |
| `fish_growth_von_bertalanffy` | `fn fish_growth_von_bertalanffy(l_inf: f64, k: f64, t: f64, t0: f64) → f64` | $L(t) = L_\infty(1 - e^{-k(t-t_0)})$ | `biology::marine_biology::fisheries` |
| `fish_mortality_total` | `fn fish_mortality_total(natural: f64, fishing: f64) → f64` | $Z = M + F$ | `biology::marine_biology::fisheries` |
| `maximum_sustainable_yield` | `fn maximum_sustainable_yield(r: f64, k: f64) → f64` | $MSY = rK/4$ | `biology::marine_biology::fisheries` |
| `stock_recruitment_beverton_holt` | `fn stock_recruitment_beverton_holt(alpha: f64, beta: f64, spawners: f64) → f64` | Beverton-Holt stock-recruitment | `biology::marine_biology::fisheries` |
| `stock_recruitment_ricker` | `fn stock_recruitment_ricker(alpha: f64, beta: f64, spawners: f64) → f64` | Ricker stock-recruitment | `biology::marine_biology::fisheries` |
| `catch_per_unit_effort` | `fn catch_per_unit_effort(catch: f64, effort: f64) → f64` | $CPUE = C / E$ | `biology::marine_biology::fisheries` |
| `decompression_no_stop_time` | `fn decompression_no_stop_time(depth_m: f64, surface_rate: f64) → f64` | No-decompression stop time | `biology::marine_biology::fisheries` |
| `schaefer_biomass` | `fn schaefer_biomass(biomass: f64, r: f64, k: f64, catch: f64, dt: f64) → f64` | Schaefer biomass dynamics | `biology::marine_biology::fisheries` |
| `fishing_mortality_from_effort` | `fn fishing_mortality_from_effort(catchability: f64, effort: f64) → f64` | $F = qE$ | `biology::marine_biology::fisheries` |
| `yield_per_recruit` | `fn yield_per_recruit(f: f64, m: f64, w_inf: f64, k: f64, t_c: f64, t_r: f64, t0: f64) → f64` | Beverton-Holt yield-per-recruit | `biology::marine_biology::fisheries` |
| `spawning_stock_biomass` | `fn spawning_stock_biomass(numbers: &[f64], weights: &[f64], maturity: &[f64]) → f64` | $SSB = \sum N_i W_i M_i$ | `biology::marine_biology::fisheries` |
| `exploitation_rate` | `fn exploitation_rate(f: f64, z: f64) → f64` | $E = F / Z$ | `biology::marine_biology::fisheries` |
| `virtual_population_analysis` | `fn virtual_population_analysis(catch: f64, m: f64, f_rate: f64) → f64` | VPA cohort analysis | `biology::marine_biology::fisheries` |
| `length_weight_relation` | `fn length_weight_relation(length: f64, a: f64, b: f64) → f64` | $W = aL^b$ | `biology::marine_biology::fisheries` |
| `condition_factor_fulton` | `fn condition_factor_fulton(weight: f64, length: f64) → f64` | $K = 100W/L^3$ — Fulton's condition factor | `biology::marine_biology::fisheries` |
| `surplus_production` | `fn surplus_production(biomass_t: f64, biomass_t1: f64, catch: f64) → f64` | $SP = B_{t+1} - B_t + C$ | `biology::marine_biology::fisheries` |
| `fox_model_equilibrium_yield` | `fn fox_model_equilibrium_yield(f: f64, msy: f64, f_msy: f64) → f64` | Fox surplus production model | `biology::marine_biology::fisheries` |
| `selectivity_logistic` | `fn selectivity_logistic(length: f64, l50: f64, slope: f64) → f64` | Logistic gear selectivity | `biology::marine_biology::fisheries` |
| `discard_mortality` | `fn discard_mortality(catch: f64, discard_fraction: f64, discard_survival: f64) → f64` | Discard mortality estimation | `biology::marine_biology::fisheries` |

#### ocean.rs (22 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `osmoregulation_flux` | `fn osmoregulation_flux(internal_osmolarity: f64, external_osmolarity: f64, permeability: f64, surface_area: f64) → f64` | Osmoregulation water flux | `biology::marine_biology::ocean` |
| `coral_bleaching_threshold` | `fn coral_bleaching_threshold(sst: f64, mmm: f64) → f64` | Bleaching thermal threshold | `biology::marine_biology::ocean` |
| `degree_heating_weeks` | `fn degree_heating_weeks(weekly_anomalies: &[f64]) → f64` | DHW cumulative thermal stress | `biology::marine_biology::ocean` |
| `ocean_acidification_ph` | `fn ocean_acidification_ph(pco2: f64, alkalinity: f64, temperature: f64) → f64` | Ocean pH from $pCO_2$ | `biology::marine_biology::ocean` |
| `carbonate_saturation_state` | `fn carbonate_saturation_state(ca_concentration: f64, co3_concentration: f64, ksp: f64) → f64` | $\Omega = [\text{Ca}^{2+}][\text{CO}_3^{2-}] / K_{sp}$ | `biology::marine_biology::ocean` |
| `bioluminescence_intensity` | `fn bioluminescence_intensity(luciferin: f64, luciferase: f64, oxygen: f64, km: f64) → f64` | Bioluminescence intensity | `biology::marine_biology::ocean` |
| `depth_light_attenuation` | `fn depth_light_attenuation(surface_irradiance: f64, attenuation_coeff: f64, depth: f64) → f64` | $I(z) = I_0 e^{-kz}$ | `biology::marine_biology::ocean` |
| `thermohaline_density` | `fn thermohaline_density(temperature: f64, salinity: f64) → f64` | Seawater density $\rho(T, S)$ | `biology::marine_biology::ocean` |
| `mixed_layer_depth_temperature` | `fn mixed_layer_depth_temperature(profile_temps: &[f64], profile_depths: &[f64], threshold: f64) → f64` | Mixed layer depth from temperature | `biology::marine_biology::ocean` |
| `ekman_depth` | `fn ekman_depth(coriolis: f64, eddy_viscosity: f64) → f64` | $D_E = \pi\sqrt{2A_z / f}$ | `biology::marine_biology::ocean` |
| `ekman_transport` | `fn ekman_transport(wind_stress: f64, coriolis: f64, density: f64) → f64` | $M = \tau / (\rho f)$ | `biology::marine_biology::ocean` |
| `sverdrup_transport` | `fn sverdrup_transport(wind_stress_curl: f64, beta: f64) → f64` | Sverdrup transport | `biology::marine_biology::ocean` |
| `primary_production_eppley` | `fn primary_production_eppley(chlorophyll: f64, par: f64, temperature: f64) → f64` | Eppley primary production model | `biology::marine_biology::ocean` |
| `new_production_f_ratio` | `fn new_production_f_ratio(nitrate_uptake: f64, total_uptake: f64) → f64` | $f$-ratio new production | `biology::marine_biology::ocean` |
| `nitrogen_fixation_rate` | `fn nitrogen_fixation_rate(temperature: f64, iron: f64, light: f64) → f64` | $N_2$ fixation rate | `biology::marine_biology::ocean` |
| `oxygen_minimum_zone_depth` | `fn oxygen_minimum_zone_depth(surface_o2: f64, respiration_rate: f64, ventilation_rate: f64) → f64` | OMZ depth estimation | `biology::marine_biology::ocean` |
| `seawater_sound_speed` | `fn seawater_sound_speed(temperature: f64, salinity: f64, depth: f64) → f64` | Sound speed in seawater | `biology::marine_biology::ocean` |
| `coral_calcification_rate` | `fn coral_calcification_rate(aragonite_saturation: f64, temperature: f64, light: f64) → f64` | Coral calcification rate | `biology::marine_biology::ocean` |
| `tidal_range` | `fn tidal_range(lunar_amplitude: f64, solar_amplitude: f64, phase_angle: f64) → f64` | Tidal range from lunar/solar | `biology::marine_biology::ocean` |
| `wave_energy_density` | `fn wave_energy_density(density: f64, gravity: f64, wave_height: f64) → f64` | $E = \rho g H^2 / 8$ | `biology::marine_biology::ocean` |
| `deep_water_wave_speed` | `fn deep_water_wave_speed(gravity: f64, wavelength: f64) → f64` | $c = \sqrt{g\lambda / 2\pi}$ | `biology::marine_biology::ocean` |
| `upwelling_velocity` | `fn upwelling_velocity(wind_stress: f64, coriolis: f64, density: f64, coast_distance: f64) → f64` | Coastal upwelling velocity | `biology::marine_biology::ocean` |

#### plankton.rs (10 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `phytoplankton_growth` | `fn phytoplankton_growth(mu_max: f64, nutrient: f64, ks: f64, light: f64, ki: f64) → f64` | Phytoplankton growth rate | `biology::marine_biology::plankton` |
| `bloom_critical_depth` | `fn bloom_critical_depth(surface_irradiance: f64, compensation_irradiance: f64, attenuation: f64) → f64` | Critical depth for bloom initiation | `biology::marine_biology::plankton` |
| `sverdrup_critical_depth` | `fn sverdrup_critical_depth(avg_irradiance: f64, compensation_irradiance: f64, mixed_layer_depth: f64, attenuation: f64) → f64` | Sverdrup critical depth hypothesis | `biology::marine_biology::plankton` |
| `nutrient_phytoplankton_zooplankton_step` | `fn nutrient_phytoplankton_zooplankton_step(n: f64, p: f64, z: f64, dt: f64, mu: f64, ks: f64, grazing: f64, kp: f64, mortality_z: f64, recycling: f64) → (f64, f64, f64)` | NPZ model time step | `biology::marine_biology::plankton` |
| `chlorophyll_a_from_biomass` | `fn chlorophyll_a_from_biomass(biomass: f64, carbon_to_chl_ratio: f64) → f64` | Chlorophyll-a from biomass | `biology::marine_biology::plankton` |
| `zooplankton_diel_migration_depth` | `fn zooplankton_diel_migration_depth(daytime_depth: f64, nighttime_depth: f64, time_hours: f64) → f64` | Diel vertical migration depth | `biology::marine_biology::plankton` |
| `harmful_algal_bloom_toxin` | `fn harmful_algal_bloom_toxin(cell_density: f64, toxin_per_cell: f64, decay_rate: f64, t: f64) → f64` | HAB toxin concentration | `biology::marine_biology::plankton` |
| `redfield_ratio_deviation` | `fn redfield_ratio_deviation(c: f64, n: f64, p: f64) → (f64, f64)` | Redfield ratio C:N:P deviation | `biology::marine_biology::plankton` |
| `spring_bloom_timing` | `fn spring_bloom_timing(mixed_layer_depth: f64, critical_depth: f64) → bool` | Spring bloom timing condition | `biology::marine_biology::plankton` |
| `export_production` | `fn export_production(primary_production: f64, f_ratio: f64) → f64` | Export production from $f$-ratio | `biology::marine_biology::plankton` |

### 5️⃣ ethology/ — Ethology — 60 pub fn

#### behavior.rs (23 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `territory_size` | `fn territory_size(body_mass: f64, scaling_exponent: f64, constant: f64) → f64` | Territory size allometry | `biology::ethology::behavior` |
| `territory_defense_cost` | `fn territory_defense_cost(perimeter: f64, intruder_rate: f64, cost_per_encounter: f64) → f64` | Territory defense cost | `biology::ethology::behavior` |
| `boldness_shyness_continuum` | `fn boldness_shyness_continuum(stimulus: f64, threshold: f64, steepness: f64) → f64` | Boldness-shyness personality axis | `biology::ethology::behavior` |
| `dilution_effect` | `fn dilution_effect(group_size: f64) → f64` | $P = 1/N$ — Dilution effect | `biology::ethology::behavior` |
| `many_eyes_detection` | `fn many_eyes_detection(individual_detection: f64, group_size: f64) → f64` | Many-eyes predator detection | `biology::ethology::behavior` |
| `dominance_probability` | `fn dominance_probability(rating_a: f64, rating_b: f64) → f64` | Dominance contest probability | `biology::ethology::behavior` |
| `allee_effect_growth` | `fn allee_effect_growth(n: f64, k: f64, r: f64, a: f64) → f64` | Growth with Allee threshold | `biology::ethology::behavior` |
| `predator_avoidance_flight_distance` | `fn predator_avoidance_flight_distance(body_mass: f64, scaling: f64, risk_factor: f64) → f64` | Flight initiation distance | `biology::ethology::behavior` |
| `hamilton_relatedness_benefit` | `fn hamilton_relatedness_benefit(relatedness: f64, benefit: f64, cost: f64) → bool` | $rB > C$ — Hamilton's rule | `biology::ethology::behavior` |
| `reciprocal_altruism_threshold` | `fn reciprocal_altruism_threshold(benefit: f64, cost: f64, probability_future: f64) → bool` | Reciprocal altruism threshold | `biology::ethology::behavior` |
| `selfish_herd_risk` | `fn selfish_herd_risk(distance_to_nearest: f64, predator_speed: f64) → f64` | Selfish herd predation risk | `biology::ethology::behavior` |
| `vigilance_group_tradeoff` | `fn vigilance_group_tradeoff(group_size: f64, individual_scan_rate: f64) → f64` | Group size-vigilance tradeoff | `biology::ethology::behavior` |
| `confusion_effect` | `fn confusion_effect(group_size: f64, predator_success_solo: f64) → f64` | Predator confusion effect | `biology::ethology::behavior` |
| `mobbing_probability` | `fn mobbing_probability(group_size: f64, predator_danger: f64, threshold: f64) → f64` | Mobbing behavior probability | `biology::ethology::behavior` |
| `learning_curve_operant` | `fn learning_curve_operant(trials: f64, asymptote: f64, rate: f64) → f64` | Operant learning curve | `biology::ethology::behavior` |
| `stimulus_generalization` | `fn stimulus_generalization(distance: f64, width: f64) → f64` | Stimulus generalization gradient | `biology::ethology::behavior` |
| `ideal_despotic_distribution` | `fn ideal_despotic_distribution(rank: f64, max_rank: f64, total_resource: f64) → f64` | Ideal despotic distribution | `biology::ethology::behavior` |
| `aggression_cost_benefit` | `fn aggression_cost_benefit(resource_value: f64, fighting_ability: f64, injury_cost: f64) → f64` | Aggression cost-benefit | `biology::ethology::behavior` |
| `migration_threshold` | `fn migration_threshold(food_current: f64, food_destination: f64, travel_cost: f64) → bool` | Migration decision threshold | `biology::ethology::behavior` |
| `information_center_benefit` | `fn information_center_benefit(colony_size: f64, discovery_prob: f64) → f64` | Information center hypothesis | `biology::ethology::behavior` |
| `social_network_centrality` | `fn social_network_centrality(connections: f64, max_connections: f64) → f64` | Social network centrality | `biology::ethology::behavior` |
| `handicap_signal_cost` | `fn handicap_signal_cost(quality: f64, signal_intensity: f64, cost_coeff: f64) → f64` | Zahavi handicap principle | `biology::ethology::behavior` |
| `mate_choice_copying` | `fn mate_choice_copying(intrinsic_preference: f64, social_info: f64, weight: f64) → f64` | Mate choice copying | `biology::ethology::behavior` |

#### communication.rs (9 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `signal_detection_d_prime` | `fn signal_detection_d_prime(hit_rate: f64, false_alarm_rate: f64) → f64` | Signal detection $d'$ | `biology::ethology::communication` |
| `honest_signal_handicap` | `fn honest_signal_handicap(quality: f64, cost_per_signal: f64, benefit_per_signal: f64) → f64` | Honest signaling handicap | `biology::ethology::communication` |
| `alarm_call_kin_selection` | `fn alarm_call_kin_selection(relatedness: f64, benefit_to_kin: f64, cost_to_caller: f64) → bool` | Alarm call kin selection | `biology::ethology::communication` |
| `mate_choice_threshold` | `fn mate_choice_threshold(quality_assessed: f64, search_cost: f64, encounters: usize, threshold: f64) → bool` | Sequential mate choice threshold | `biology::ethology::communication` |
| `ritualized_contest` | `fn ritualized_contest(size_a: f64, size_b: f64, motivation_a: f64, motivation_b: f64) → f64` | Ritualized contest outcome | `biology::ethology::communication` |
| `hawk_dove_contest` | `fn hawk_dove_contest(v: f64, c: f64, p_hawk: f64) → (f64, f64)` | Hawk-Dove game payoffs | `biology::ethology::communication` |
| `producer_scrounger_frequency` | `fn producer_scrounger_frequency(producer_payoff: f64, scrounger_payoff: f64, p_producer: f64, selection_strength: f64) → f64` | Producer-scrounger frequency | `biology::ethology::communication` |
| `territory_size_optimal` | `fn territory_size_optimal(energy_gain_rate: f64, defense_cost_per_area: f64) → f64` | Optimal territory size | `biology::ethology::communication` |
| `dominance_index` | `fn dominance_index(wins: f64, total_interactions: f64) → f64` | Dominance index | `biology::ethology::communication` |

#### foraging.rs (18 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `optimal_diet_value` | `fn optimal_diet_value(energy_gain: f64, handling_time: f64, encounter_rate: f64) → f64` | Optimal diet profitability | `biology::ethology::foraging` |
| `marginal_value_theorem` | `fn marginal_value_theorem(travel_time: f64, gain_curve: impl Fn(f64) -> f64, max_t: f64) → f64` | Charnov marginal value theorem | `biology::ethology::foraging` |
| `ideal_free_distribution` | `fn ideal_free_distribution(resource: &[f64], total_individuals: f64) → Vec<f64>` | Ideal free distribution | `biology::ethology::foraging` |
| `hawk_dove_payoff` | `fn hawk_dove_payoff(v: f64, c: f64, hawk_freq: f64) → (f64, f64)` | Hawk-Dove ESS payoffs | `biology::ethology::foraging` |
| `ess_hawk_frequency` | `fn ess_hawk_frequency(v: f64, c: f64) → f64` | $p^* = V/C$ — ESS hawk frequency | `biology::ethology::foraging` |
| `tit_for_tat_payoff` | `fn tit_for_tat_payoff(r: f64, s: f64, t: f64, p: f64, opponent_cooperates: bool) → f64` | Tit-for-tat strategy payoff | `biology::ethology::foraging` |
| `prey_choice_ranking` | `fn prey_choice_ranking(prey_types: &[(f64, f64)]) → Vec<(usize, f64)>` | Prey profitability ranking | `biology::ethology::foraging` |
| `risk_sensitive_foraging` | `fn risk_sensitive_foraging(mean_gain: f64, variance: f64, risk_aversion: f64) → f64` | Risk-sensitive foraging value | `biology::ethology::foraging` |
| `central_place_foraging` | `fn central_place_foraging(distance: f64, load: f64, travel_cost_per_unit: f64, gain_per_load: f64) → f64` | Central place foraging | `biology::ethology::foraging` |
| `producer_scrounger_game` | `fn producer_scrounger_game(p_freq: f64, finder_advantage: f64, group_size: f64) → (f64, f64)` | Producer-scrounger game | `biology::ethology::foraging` |
| `giving_up_density` | `fn giving_up_density(metabolic_cost: f64, predation_cost: f64, missed_opportunity: f64) → f64` | Giving-up density | `biology::ethology::foraging` |
| `patch_residence_time` | `fn patch_residence_time(gain_rate: f64, travel_time: f64, depletion_rate: f64) → f64` | Optimal patch residence time | `biology::ethology::foraging` |
| `functional_response_type_ii` | `fn functional_response_type_ii(prey_density: f64, attack_rate: f64, handling_time: f64) → f64` | $f = aN/(1+ahN)$ — Type II | `biology::ethology::foraging` |
| `functional_response_type_iii` | `fn functional_response_type_iii(prey_density: f64, attack_max: f64, half_sat: f64, handling_time: f64) → f64` | Type III functional response | `biology::ethology::foraging` |
| `starvation_risk` | `fn starvation_risk(reserves: f64, daily_cost: f64, variance: f64) → f64` | Starvation risk assessment | `biology::ethology::foraging` |
| `cache_pilferage_rate` | `fn cache_pilferage_rate(competitors: f64, detection_prob: f64, cache_density: f64) → f64` | Cache pilferage rate | `biology::ethology::foraging` |
| `optimal_load_size` | `fn optimal_load_size(distance: f64, max_load: f64, loading_rate: f64, travel_speed: f64) → f64` | Optimal load size | `biology::ethology::foraging` |
| `diet_breadth_threshold` | `fn diet_breadth_threshold(energy: &[f64], handling: &[f64], encounter: &[f64]) → usize` | Diet breadth inclusion threshold | `biology::ethology::foraging` |

#### learning.rs (10 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `habituation` | `fn habituation(response: f64, stimulus_count: usize, decay_rate: f64) → f64` | Habituation response decay | `biology::ethology::learning` |
| `sensitization` | `fn sensitization(response: f64, noxious_stimulus: f64, gain: f64, saturation: f64) → f64` | Sensitization response | `biology::ethology::learning` |
| `operant_conditioning_rate` | `fn operant_conditioning_rate(reinforcement_rate: f64, response_rate: f64, k: f64) → f64` | Operant conditioning rate | `biology::ethology::learning` |
| `classical_conditioning_rescorla_wagner` | `fn classical_conditioning_rescorla_wagner(v: f64, alpha: f64, beta: f64, lambda: f64) → f64` | $\Delta V = \alpha\beta(\lambda - V)$ — Rescorla-Wagner | `biology::ethology::learning` |
| `spatial_learning_distance` | `fn spatial_learning_distance(trial: usize, asymptote: f64, learning_rate: f64, initial_distance: f64) → f64` | Spatial learning curve | `biology::ethology::learning` |
| `imprinting_critical_period` | `fn imprinting_critical_period(exposure_time: f64, peak_time: f64, width: f64) → f64` | Imprinting critical period | `biology::ethology::learning` |
| `social_learning_transmission` | `fn social_learning_transmission(demonstrators: f64, naive: f64, transmission_rate: f64) → f64` | Social learning transmission | `biology::ethology::learning` |
| `memory_retention_ebbinghaus` | `fn memory_retention_ebbinghaus(strength: f64, time: f64, stability: f64) → f64` | Ebbinghaus forgetting curve | `biology::ethology::learning` |
| `working_memory_capacity` | `fn working_memory_capacity(items: &[f64], capacity: usize) → f64` | Working memory capacity | `biology::ethology::learning` |
| `win_stay_lose_shift` | `fn win_stay_lose_shift(previous_outcome: f64, threshold: f64) → bool` | Win-stay lose-shift strategy | `biology::ethology::learning` |

### 6️⃣ genetics/ — Genetics — 59 pub fn

#### drift.rs (9 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `genetic_drift_wright_fisher` | `fn genetic_drift_wright_fisher(p: f64, pop_size: usize, generations: usize, seed: u64) → Vec<f64>` | Wright-Fisher drift simulation | `biology::genetics::drift` |
| `drift_effective_population_size` | `fn drift_effective_population_size(n_males: f64, n_females: f64) → f64` | $N_e = 4N_mN_f / (N_m + N_f)$ | `biology::genetics::drift` |
| `effective_population_size_varying` | `fn effective_population_size_varying(sizes: &[f64]) → f64` | Harmonic mean $N_e$ | `biology::genetics::drift` |
| `heterozygosity_loss` | `fn heterozygosity_loss(ne: f64, generations: usize) → f64` | $H_t = H_0(1 - 1/(2N_e))^t$ | `biology::genetics::drift` |
| `mutation_drift_equilibrium` | `fn mutation_drift_equilibrium(ne: f64, mu: f64) → f64` | $\theta = 4N_e\mu$ | `biology::genetics::drift` |
| `fixation_probability_neutral` | `fn fixation_probability_neutral(ne: f64) → f64` | $P = 1/(2N_e)$ | `biology::genetics::drift` |
| `fixation_probability_selection` | `fn fixation_probability_selection(ne: f64, s: f64, p: f64) → f64` | Fixation probability with selection | `biology::genetics::drift` |
| `fixation_time_neutral` | `fn fixation_time_neutral(ne: f64) → f64` | $\bar{t} = 4N_e$ generations | `biology::genetics::drift` |
| `bottleneck_heterozygosity` | `fn bottleneck_heterozygosity(h0: f64, n_bottleneck: f64, generations: usize) → f64` | Bottleneck heterozygosity loss | `biology::genetics::drift` |

#### equilibrium.rs (10 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `hardy_weinberg` | `fn hardy_weinberg(p: f64) → (f64, f64, f64)` | $p^2, 2pq, q^2$ — Hardy-Weinberg | `biology::genetics::equilibrium` |
| `hardy_weinberg_multi` | `fn hardy_weinberg_multi(freqs: &[f64]) → Vec<Vec<f64>>` | Multi-allele Hardy-Weinberg | `biology::genetics::equilibrium` |
| `chi_squared_hw` | `fn chi_squared_hw(observed: &[f64], p: f64, n_total: f64) → f64` | $\chi^2$ test for HW equilibrium | `biology::genetics::equilibrium` |
| `inbreeding_coefficient` | `fn inbreeding_coefficient(h_obs: f64, h_exp: f64) → f64` | $F = 1 - H_o/H_e$ | `biology::genetics::equilibrium` |
| `wahlund_effect` | `fn wahlund_effect(subpop_freqs: &[f64]) → f64` | Wahlund effect heterozygosity deficit | `biology::genetics::equilibrium` |
| `fst` | `fn fst(subpop_freqs: &[f64]) → f64` | $F_{ST}$ population differentiation | `biology::genetics::equilibrium` |
| `nei_genetic_distance` | `fn nei_genetic_distance(pop1_freqs: &[f64], pop2_freqs: &[f64]) → f64` | Nei's genetic distance | `biology::genetics::equilibrium` |
| `expected_heterozygosity` | `fn expected_heterozygosity(allele_freqs: &[f64]) → f64` | $H_e = 1 - \sum p_i^2$ | `biology::genetics::equilibrium` |
| `allele_frequency_cline` | `fn allele_frequency_cline(x: f64, center: f64, width: f64) → f64` | Allele frequency cline | `biology::genetics::equilibrium` |
| `effective_population_size` | `fn effective_population_size(n_males: f64, n_females: f64) → f64` | $N_e = 4N_mN_f/(N_m+N_f)$ | `biology::genetics::equilibrium` |

#### linkage.rs (9 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `recombination_frequency` | `fn recombination_frequency(recombinants: f64, total_offspring: f64) → f64` | Recombination frequency | `biology::genetics::linkage` |
| `map_distance_kosambi` | `fn map_distance_kosambi(recombination_freq: f64) → f64` | Kosambi map function | `biology::genetics::linkage` |
| `map_distance_haldane` | `fn map_distance_haldane(recombination_freq: f64) → f64` | $d = -\frac{1}{2}\ln(1-2r)$ — Haldane | `biology::genetics::linkage` |
| `haldane_to_recombination` | `fn haldane_to_recombination(map_distance: f64) → f64` | Inverse Haldane map function | `biology::genetics::linkage` |
| `lod_score` | `fn lod_score(theta: f64, recombinants: usize, non_recombinants: usize) → f64` | LOD score for linkage | `biology::genetics::linkage` |
| `three_point_cross_distance` | `fn three_point_cross_distance(class_counts: &[f64; 8]) → (f64, f64, f64)` | Three-point cross distances | `biology::genetics::linkage` |
| `interference` | `fn interference(observed_double_co: f64, expected_double_co: f64) → f64` | $I = 1 - \text{CoC}$ — Interference | `biology::genetics::linkage` |
| `chiasma_frequency` | `fn chiasma_frequency(recombination_freq: f64) → f64` | Chiasma frequency | `biology::genetics::linkage` |
| `synaptonemal_complex_length` | `fn synaptonemal_complex_length(chromosome_length_mb: f64, loop_size_kb: f64) → f64` | Synaptonemal complex length | `biology::genetics::linkage` |

#### molecular.rs (7 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `coalescent_expected_time` | `fn coalescent_expected_time(n: usize) → f64` | $E[T_k] = 4N_e/(k(k-1))$ | `biology::genetics::molecular` |
| `watterson_theta` | `fn watterson_theta(segregating_sites: usize, n: usize) → f64` | $\hat{\theta}_W = S / a_n$ — Watterson estimator | `biology::genetics::molecular` |
| `fst_pairwise` | `fn fst_pairwise(ht: f64, hs: f64) → f64` | $F_{ST} = (H_T - H_S)/H_T$ | `biology::genetics::molecular` |
| `nucleotide_diversity` | `fn nucleotide_diversity(sequences: &[&[u8]]) → f64` | $\pi$ — Nucleotide diversity | `biology::genetics::molecular` |
| `tajima_d` | `fn tajima_d(pi: f64, theta_w: f64, n: usize, seg_sites: usize) → f64` | Tajima's $D$ statistic | `biology::genetics::molecular` |
| `nei_distance` | `fn nei_distance(pop1_freqs: &[f64], pop2_freqs: &[f64]) → f64` | Nei's genetic distance | `biology::genetics::molecular` |
| `molecular_heterozygosity` | `fn molecular_heterozygosity(freqs: &[f64]) → f64` | $H = 1 - \sum p_i^2$ | `biology::genetics::molecular` |

#### quantitative.rs (15 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `broad_sense_heritability` | `fn broad_sense_heritability(genetic_variance: f64, phenotypic_variance: f64) → f64` | $H^2 = V_G/V_P$ | `biology::genetics::quantitative` |
| `narrow_sense_heritability` | `fn narrow_sense_heritability(additive_variance: f64, phenotypic_variance: f64) → f64` | $h^2 = V_A/V_P$ | `biology::genetics::quantitative` |
| `breeder_equation` | `fn breeder_equation(heritability: f64, selection_differential: f64) → f64` | $R = h^2 S$ — Breeder's equation | `biology::genetics::quantitative` |
| `selection_differential` | `fn selection_differential(mean_selected: f64, mean_population: f64) → f64` | $S = \bar{z}_s - \bar{z}$ | `biology::genetics::quantitative` |
| `additive_genetic_variance` | `fn additive_genetic_variance(allele_freq: f64, allele_effect: f64) → f64` | $V_A = 2pq\alpha^2$ | `biology::genetics::quantitative` |
| `dominance_variance` | `fn dominance_variance(allele_freq: f64, dominance_deviation: f64) → f64` | $V_D = (2pqd)^2$ | `biology::genetics::quantitative` |
| `qtl_effect_from_means` | `fn qtl_effect_from_means(mean_aa: f64, mean_ab: f64, mean_bb: f64) → (f64, f64)` | QTL additive & dominance effects | `biology::genetics::quantitative` |
| `phenotypic_variance_components` | `fn phenotypic_variance_components(va: f64, vd: f64, ve: f64, vi: f64) → f64` | $V_P = V_A + V_D + V_I + V_E$ | `biology::genetics::quantitative` |
| `realized_heritability` | `fn realized_heritability(response: f64, selection_differential: f64) → f64` | Realized heritability | `biology::genetics::quantitative` |
| `mid_parent_regression` | `fn mid_parent_regression(parent1: f64, parent2: f64, heritability: f64, population_mean: f64) → f64` | Mid-parent offspring regression | `biology::genetics::quantitative` |
| `lander_botstein_lod` | `fn lander_botstein_lod(n: usize, r_squared: f64) → f64` | Lander-Botstein LOD score | `biology::genetics::quantitative` |
| `polygenic_score` | `fn polygenic_score(effects: &[f64], genotypes: &[f64]) → f64` | Polygenic risk score | `biology::genetics::quantitative` |
| `falconer_liability_threshold` | `fn falconer_liability_threshold(prevalence: f64) → f64` | Falconer liability threshold | `biology::genetics::quantitative` |
| `snp_heritability` | `fn snp_heritability(beta_squared_sum: f64, variance_explained: f64, phenotypic_variance: f64) → f64` | SNP-based heritability | `biology::genetics::quantitative` |

#### selection.rs (10 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `allele_frequency_selection` | `fn allele_frequency_selection(p: f64, w_aa: f64, w_ab: f64, w_bb: f64) → f64` | Allele frequency under selection | `biology::genetics::selection` |
| `selection_iterate` | `fn selection_iterate(p0: f64, w_aa: f64, w_ab: f64, w_bb: f64, generations: usize) → Vec<f64>` | Multi-generation selection | `biology::genetics::selection` |
| `selection_coefficient` | `fn selection_coefficient(w_mutant: f64, w_wildtype: f64) → f64` | $s = 1 - w_m/w_w$ | `biology::genetics::selection` |
| `mutation_selection_balance` | `fn mutation_selection_balance(mu: f64, s: f64) → f64` | $\hat{q} = \mu/s$ — Dominant | `biology::genetics::selection` |
| `mutation_selection_balance_recessive` | `fn mutation_selection_balance_recessive(mu: f64, s: f64) → f64` | $\hat{q} = \sqrt{\mu/s}$ — Recessive | `biology::genetics::selection` |
| `frequency_dependent_fitness` | `fn frequency_dependent_fitness(p: f64, a: f64, b: f64) → f64` | Frequency-dependent fitness | `biology::genetics::selection` |
| `heterozygote_advantage_equilibrium` | `fn heterozygote_advantage_equilibrium(s: f64, t: f64) → f64` | $\hat{p} = t/(s+t)$ — Overdominance | `biology::genetics::selection` |
| `disruptive_selection` | `fn disruptive_selection(p: f64, w_aa: f64, w_ab: f64, w_bb: f64, generations: usize) → Vec<f64>` | Disruptive selection dynamics | `biology::genetics::selection` |
| `truncation_selection` | `fn truncation_selection(mean: f64, variance: f64, threshold: f64) → f64` | Truncation selection intensity | `biology::genetics::selection` |
| `response_to_selection` | `fn response_to_selection(heritability: f64, selection_differential: f64) → f64` | $R = h^2 S$ | `biology::genetics::selection` |

### 7️⃣ genomics/ — Genomics — 46 pub fn

#### annotation.rs (10 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `gene_density` | `fn gene_density(genes: usize, region_length_mb: f64) → f64` | Gene density per Mb | `biology::genomics::annotation` |
| `cpg_enrichment` | `fn cpg_enrichment(cpg_count: usize, c_count: usize, g_count: usize, length: usize) → f64` | CpG observed/expected ratio | `biology::genomics::annotation` |
| `codon_adaptation_index` | `fn codon_adaptation_index(codon_weights: &[f64]) → f64` | CAI codon adaptation index | `biology::genomics::annotation` |
| `enc_wright` | `fn enc_wright(codon_family_homozygosities: &[f64]) → f64` | Wright effective number of codons | `biology::genomics::annotation` |
| `repeat_density` | `fn repeat_density(repeat_bases: usize, total_bases: usize) → f64` | Repeat element density | `biology::genomics::annotation` |
| `synteny_score` | `fn synteny_score(conserved_blocks: usize, total_genes: usize) → f64` | Synteny conservation score | `biology::genomics::annotation` |
| `n50` | `fn n50(contig_lengths: &mut [f64]) → f64` | N50 assembly metric | `biology::genomics::annotation` |
| `genome_completeness_busco` | `fn genome_completeness_busco(complete: usize, fragmented: usize, total_buscos: usize) → f64` | BUSCO completeness score | `biology::genomics::annotation` |
| `ka_ks_ratio` | `fn ka_ks_ratio(nonsynonymous_subs: f64, synonymous_subs: f64, nonsynonymous_sites: f64, synonymous_sites: f64) → f64` | $K_a/K_s$ ratio | `biology::genomics::annotation` |
| `gc_isochore` | `fn gc_isochore(gc_content: f64) → &'static str` | GC isochore classification | `biology::genomics::annotation` |

#### motifs.rs (6 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `pwm_score` | `fn pwm_score(pwm: &[Vec<f64>], sequence: &[u8]) → f64` | Position weight matrix score | `biology::genomics::motifs` |
| `pwm_scan` | `fn pwm_scan(pwm: &[Vec<f64>], sequence: &[u8], threshold: f64) → Vec<(usize, f64)>` | PWM motif scanning | `biology::genomics::motifs` |
| `information_content` | `fn information_content(pwm: &[Vec<f64>]) → Vec<f64>` | Per-position information content | `biology::genomics::motifs` |
| `total_information` | `fn total_information(pwm: &[Vec<f64>]) → f64` | Total information content (bits) | `biology::genomics::motifs` |
| `consensus_sequence` | `fn consensus_sequence(pwm: &[Vec<f64>]) → String` | Consensus sequence from PWM | `biology::genomics::motifs` |
| `frequency_matrix` | `fn frequency_matrix(sequences: &[&[u8]], length: usize) → Vec<Vec<f64>>` | Frequency matrix from sequences | `biology::genomics::motifs` |

#### orf.rs (10 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `find_orfs` | `fn find_orfs(sequence: &str, min_length: usize) → Vec<(usize, usize, String)>` | Open reading frame detection | `biology::genomics::orf` |
| `codon_usage` | `fn codon_usage(sequence: &str) → Vec<(String, usize)>` | Codon usage frequency | `biology::genomics::orf` |
| `reading_frame_proteins` | `fn reading_frame_proteins(sequence: &str, frame: usize) → Vec<String>` | Reading frame protein extraction | `biology::genomics::orf` |
| `gc_content` | `fn gc_content(sequence: &str) → f64` | GC content fraction | `biology::genomics::orf` |
| `gc3_content` | `fn gc3_content(sequence: &str) → f64` | GC content at 3rd codon position | `biology::genomics::orf` |
| `effective_number_of_codons` | `fn effective_number_of_codons(codon_counts: &[(String, usize)]) → f64` | Effective number of codons (ENC) | `biology::genomics::orf` |
| `longest_orf_length` | `fn longest_orf_length(sequence: &str) → usize` | Longest ORF length | `biology::genomics::orf` |
| `nucleotide_frequency` | `fn nucleotide_frequency(sequence: &str) → [f64; 4]` | Nucleotide frequency A/T/G/C | `biology::genomics::orf` |
| `translate` | `fn translate(sequence: &str) → String` | DNA to protein translation | `biology::genomics::orf` |
| `reverse_complement` | `fn reverse_complement(sequence: &str) → String` | Reverse complement | `biology::genomics::orf` |

#### statistics.rs (8 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `kmer_count` | `fn kmer_count(sequence: &[u8], k: usize) → Vec<(Vec<u8>, usize)>` | $k$-mer frequency count | `biology::genomics::statistics` |
| `gc_skew` | `fn gc_skew(sequence: &[u8], window: usize) → Vec<f64>` | $(G-C)/(G+C)$ skew | `biology::genomics::statistics` |
| `cpg_observed_expected` | `fn cpg_observed_expected(sequence: &[u8]) → f64` | CpG observed/expected ratio | `biology::genomics::statistics` |
| `linguistic_complexity` | `fn linguistic_complexity(sequence: &[u8]) → f64` | Linguistic sequence complexity | `biology::genomics::statistics` |
| `at_content` | `fn at_content(sequence: &[u8]) → f64` | AT content fraction | `biology::genomics::statistics` |
| `dinucleotide_frequency` | `fn dinucleotide_frequency(sequence: &[u8]) → Vec<(u8, u8, f64)>` | Dinucleotide frequency | `biology::genomics::statistics` |
| `sequence_entropy` | `fn sequence_entropy(sequence: &[u8]) → f64` | Shannon entropy of sequence | `biology::genomics::statistics` |
| `transition_transversion` | `fn transition_transversion(sequence_a: &[u8], sequence_b: &[u8]) → f64` | Ti/Tv ratio | `biology::genomics::statistics` |

#### variants.rs (12 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `snp_allele_frequency` | `fn snp_allele_frequency(alt_count: usize, total_alleles: usize) → f64` | SNP allele frequency | `biology::genomics::variants` |
| `minor_allele_frequency` | `fn minor_allele_frequency(allele_freq: f64) → f64` | Minor allele frequency (MAF) | `biology::genomics::variants` |
| `hardy_weinberg_expected` | `fn hardy_weinberg_expected(p: f64) → (f64, f64, f64)` | $p^2, 2pq, q^2$ expected genotypes | `biology::genomics::variants` |
| `hardy_weinberg_chi_squared` | `fn hardy_weinberg_chi_squared(observed: &[f64; 3], expected: &[f64; 3]) → f64` | $\chi^2$ HW test | `biology::genomics::variants` |
| `ti_tv_ratio` | `fn ti_tv_ratio(transitions: usize, transversions: usize) → f64` | Transition/transversion ratio | `biology::genomics::variants` |
| `heterozygosity` | `fn heterozygosity(allele_freqs: &[f64]) → f64` | $H = 1 - \sum p_i^2$ | `biology::genomics::variants` |
| `fst_weir_cockerham` | `fn fst_weir_cockerham(het_within: f64, het_total: f64) → f64` | Weir-Cockerham $F_{ST}$ | `biology::genomics::variants` |
| `linkage_disequilibrium` | `fn linkage_disequilibrium(freq_ab: f64, freq_a: f64, freq_b: f64) → f64` | $D = f_{AB} - f_A f_B$ | `biology::genomics::variants` |
| `r_squared_ld` | `fn r_squared_ld(d: f64, freq_a: f64, freq_b: f64) → f64` | $r^2$ linkage disequilibrium | `biology::genomics::variants` |
| `d_prime` | `fn d_prime(d: f64, freq_a: f64, freq_b: f64) → f64` | $D'$ normalized LD | `biology::genomics::variants` |
| `indel_frameshift` | `fn indel_frameshift(indel_length: i64) → bool` | Frameshift indel detection | `biology::genomics::variants` |
| `copy_number_variant_dosage` | `fn copy_number_variant_dosage(reads_sample: f64, reads_reference: f64, ploidy: f64) → f64` | CNV dosage estimation | `biology::genomics::variants` |

### 8️⃣ proteomics/ — Proteomics — 31 pub fn

#### mass_spec.rs (6 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `b_ion_masses` | `fn b_ion_masses(sequence: &str) → Vec<f64>` | b-ion fragment masses | `biology::proteomics::mass_spec` |
| `y_ion_masses` | `fn y_ion_masses(sequence: &str) → Vec<f64>` | y-ion fragment masses | `biology::proteomics::mass_spec` |
| `mz_ratio` | `fn mz_ratio(mass: f64, charge: usize) → f64` | $m/z$ ratio | `biology::proteomics::mass_spec` |
| `mass_from_mz` | `fn mass_from_mz(mz: f64, charge: usize) → f64` | Mass from $m/z$ | `biology::proteomics::mass_spec` |
| `mass_accuracy_ppm` | `fn mass_accuracy_ppm(theoretical: f64, observed: f64) → f64` | Mass accuracy (ppm) | `biology::proteomics::mass_spec` |
| `isotope_pattern_averagine` | `fn isotope_pattern_averagine(mass: f64, n_peaks: usize) → Vec<f64>` | Averagine isotope pattern | `biology::proteomics::mass_spec` |

#### networks.rs (10 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `ppi_degree` | `fn ppi_degree(adjacency_row: &[bool]) → usize` | PPI network node degree | `biology::proteomics::networks` |
| `clustering_coefficient` | `fn clustering_coefficient(neighbors_connected: usize, degree: usize) → f64` | Clustering coefficient | `biology::proteomics::networks` |
| `betweenness_centrality_approx` | `fn betweenness_centrality_approx(shortest_paths_through: f64, total_shortest_paths: f64) → f64` | Betweenness centrality | `biology::proteomics::networks` |
| `network_density` | `fn network_density(edges: usize, nodes: usize) → f64` | Network density | `biology::proteomics::networks` |
| `scale_free_exponent` | `fn scale_free_exponent(degree_distribution: &[f64]) → f64` | Scale-free network exponent | `biology::proteomics::networks` |
| `hub_score` | `fn hub_score(degree: usize, max_degree: usize) → f64` | Hub score | `biology::proteomics::networks` |
| `edge_betweenness` | `fn edge_betweenness(flow_through_edge: f64, total_flow: f64) → f64` | Edge betweenness | `biology::proteomics::networks` |
| `protein_complex_stoichiometry` | `fn protein_complex_stoichiometry(abundances: &[f64]) → Vec<f64>` | Complex stoichiometry | `biology::proteomics::networks` |
| `functional_enrichment_odds_ratio` | `fn functional_enrichment_odds_ratio(hits_in_set: usize, set_size: usize, hits_total: usize, genome_size: usize) → f64` | Functional enrichment odds ratio | `biology::proteomics::networks` |
| `guilt_by_association_score` | `fn guilt_by_association_score(neighbor_annotations: &[bool]) → f64` | Guilt-by-association score | `biology::proteomics::networks` |

#### properties.rs (4 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `peptide_molecular_weight` | `fn peptide_molecular_weight(sequence: &str) → f64` | Peptide molecular weight | `biology::proteomics::properties` |
| `isoelectric_point` | `fn isoelectric_point(sequence: &str) → f64` | Isoelectric point (pI) | `biology::proteomics::properties` |
| `gravy_index` | `fn gravy_index(sequence: &str) → f64` | GRAVY hydropathicity index | `biology::proteomics::properties` |
| `extinction_coefficient_280` | `fn extinction_coefficient_280(n_trp: usize, n_tyr: usize, n_cystine: usize) → f64` | $\varepsilon_{280}$ extinction coefficient | `biology::proteomics::properties` |

#### quantification.rs (11 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `spectral_count_nsaf` | `fn spectral_count_nsaf(spectral_count: f64, protein_length: f64, total_nsaf: f64) → f64` | NSAF spectral counting | `biology::proteomics::quantification` |
| `ibaq` | `fn ibaq(total_intensity: f64, num_observable_peptides: usize) → f64` | iBAQ intensity | `biology::proteomics::quantification` |
| `lfq_ratio` | `fn lfq_ratio(intensity_sample: f64, intensity_reference: f64) → f64` | Label-free quantification ratio | `biology::proteomics::quantification` |
| `tmt_reporter_ratio` | `fn tmt_reporter_ratio(reporter_intensity: f64, reference_channel: f64) → f64` | TMT reporter ion ratio | `biology::proteomics::quantification` |
| `silac_ratio` | `fn silac_ratio(heavy: f64, light: f64) → f64` | SILAC heavy/light ratio | `biology::proteomics::quantification` |
| `protein_fdr` | `fn protein_fdr(decoy_hits: f64, target_hits: f64) → f64` | Protein FDR estimation | `biology::proteomics::quantification` |
| `mascot_ion_score` | `fn mascot_ion_score(observed: f64, expected: f64) → f64` | Mascot ion score | `biology::proteomics::quantification` |
| `em_pai` | `fn em_pai(observed_peptides: usize, observable_peptides: usize) → f64` | emPAI abundance index | `biology::proteomics::quantification` |
| `protein_coverage` | `fn protein_coverage(identified_residues: usize, total_residues: usize) → f64` | Protein sequence coverage | `biology::proteomics::quantification` |
| `xcorr_normalized` | `fn xcorr_normalized(xcorr: f64, peptide_length: usize) → f64` | Normalized cross-correlation | `biology::proteomics::quantification` |
| `missed_cleavage_rate` | `fn missed_cleavage_rate(missed: usize, total_peptides: usize) → f64` | Missed cleavage rate | `biology::proteomics::quantification` |

### 9️⃣ epigenetics/ — Epigenetics — 51 pub fn

#### chromatin.rs (11 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `chromatin_accessibility` | `fn chromatin_accessibility(open_fraction: f64, remodeler_activity: f64, histone_density: f64) → f64` | Chromatin accessibility | `biology::epigenetics::chromatin` |
| `nucleosome_occupancy` | `fn nucleosome_occupancy(dna_affinity: f64, histone_conc: f64, competitor_conc: f64) → f64` | Nucleosome occupancy | `biology::epigenetics::chromatin` |
| `histone_mark_propagation` | `fn histone_mark_propagation(mark_fraction: f64, write_rate: f64, erase_rate: f64, dt: f64) → f64` | Histone mark propagation | `biology::epigenetics::chromatin` |
| `polycomb_silencing` | `fn polycomb_silencing(pc_binding: f64, h3k27me3: f64, cooperative_factor: f64) → f64` | Polycomb-mediated silencing | `biology::epigenetics::chromatin` |
| `trithorax_activation` | `fn trithorax_activation(trx_binding: f64, h3k4me3: f64, cooperative_factor: f64) → f64` | Trithorax activation | `biology::epigenetics::chromatin` |
| `chromatin_loop_probability` | `fn chromatin_loop_probability(distance_bp: f64, persistence_length_bp: f64) → f64` | Chromatin loop probability | `biology::epigenetics::chromatin` |
| `tad_insulation_score` | `fn tad_insulation_score(contacts_within: f64, contacts_across: f64) → f64` | TAD insulation score | `biology::epigenetics::chromatin` |
| `enhancer_promoter_contact` | `fn enhancer_promoter_contact(distance: f64, cohesin_loading: f64, ctcf_binding: f64, decay_length: f64) → f64` | Enhancer-promoter contact | `biology::epigenetics::chromatin` |
| `bivalent_resolution` | `fn bivalent_resolution(h3k4me3: f64, h3k27me3: f64, differentiation_signal: f64) → (f64, f64)` | Bivalent domain resolution | `biology::epigenetics::chromatin` |
| `heterochromatin_spread` | `fn heterochromatin_spread(hp1_conc: f64, h3k9me3: f64, barrier_strength: f64, distance: f64) → f64` | Heterochromatin spreading | `biology::epigenetics::chromatin` |
| `atac_seq_signal` | `fn atac_seq_signal(fragment_count: f64, total_reads: f64, region_length_bp: f64) → f64` | ATAC-seq signal | `biology::epigenetics::chromatin` |

#### histones.rs (10 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `histone_mark_occupancy` | `fn histone_mark_occupancy(k_on: f64, k_off: f64) → f64` | $\theta = k_{on}/(k_{on}+k_{off})$ | `biology::epigenetics::histones` |
| `histone_mark_dynamics` | `fn histone_mark_dynamics(occupancy: f64, k_on: f64, k_off: f64, k_spread: f64, neighbor_occ: f64) → f64` | Histone mark dynamics | `biology::epigenetics::histones` |
| `histone_spread_simulate` | `fn histone_spread_simulate(occupancies: &mut [f64], k_on: f64, k_off: f64, k_spread: f64, dt: f64, steps: usize) → Vec<Vec<f64>>` | Histone spreading simulation | `biology::epigenetics::histones` |
| `nucleosome_positioning_energy` | `fn nucleosome_positioning_energy(dna_flexibility: &[f64], position: usize, wrap_length: usize) → f64` | Nucleosome positioning energy | `biology::epigenetics::histones` |
| `chromatin_compaction_ratio` | `fn chromatin_compaction_ratio(extended_length: f64, compacted_length: f64) → f64` | Chromatin compaction ratio | `biology::epigenetics::histones` |
| `histone_acetylation_equilibrium` | `fn histone_acetylation_equilibrium(hat_activity: f64, hdac_activity: f64) → f64` | HAT/HDAC equilibrium | `biology::epigenetics::histones` |
| `bivalent_domain_resolution` | `fn bivalent_domain_resolution(h3k4me3: f64, h3k27me3: f64, signal: f64, threshold: f64) → (f64, f64)` | Bivalent domain resolution | `biology::epigenetics::histones` |
| `chip_seq_enrichment` | `fn chip_seq_enrichment(ip_reads: f64, input_reads: f64, ip_total: f64, input_total: f64) → f64` | ChIP-seq enrichment | `biology::epigenetics::histones` |
| `reader_writer_feedback` | `fn reader_writer_feedback(mark: f64, reader_affinity: f64, writer_rate: f64, eraser_rate: f64) → f64` | Reader-writer feedback loop | `biology::epigenetics::histones` |
| `heterochromatin_spreading` | `fn heterochromatin_spreading(marks: &mut [f64], spread_rate: f64, barrier_positions: &[usize], dt: f64)` | Heterochromatin spreading simulation | `biology::epigenetics::histones` |

#### inheritance.rs (10 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `epigenetic_inheritance_probability` | `fn epigenetic_inheritance_probability(maintenance: f64, generations: usize) → f64` | $P = m^g$ epigenetic inheritance | `biology::epigenetics::inheritance` |
| `transgenerational_decay` | `fn transgenerational_decay(mark: f64, reset_rate: f64, generations: usize) → Vec<f64>` | Transgenerational epigenetic decay | `biology::epigenetics::inheritance` |
| `epimutation_rate` | `fn epimutation_rate(changes: usize, sites: usize, generations: usize) → f64` | Epimutation rate | `biology::epigenetics::inheritance` |
| `epiallele_frequency` | `fn epiallele_frequency(fitness_epi: f64, fitness_normal: f64, freq: f64, generations: usize) → Vec<f64>` | Epiallele frequency dynamics | `biology::epigenetics::inheritance` |
| `chromatin_state_transition` | `fn chromatin_state_transition(state: &[f64], transition_matrix: &[Vec<f64>]) → Vec<f64>` | Chromatin state transition | `biology::epigenetics::inheritance` |
| `imprinting_expression` | `fn imprinting_expression(maternal: f64, paternal: f64, imprint_maternal: bool) → f64` | Genomic imprinting expression | `biology::epigenetics::inheritance` |
| `paramutation` | `fn paramutation(strong_allele: f64, weak_allele: f64, conversion_rate: f64, generations: usize) → Vec<(f64, f64)>` | Paramutation dynamics | `biology::epigenetics::inheritance` |
| `metastable_epiallele` | `fn metastable_epiallele(base_expression: f64, methylation: f64, stochastic_variance: f64) → f64` | Metastable epiallele expression | `biology::epigenetics::inheritance` |
| `epigenetic_clock` | `fn epigenetic_clock(methylation_sites: &[f64], coefficients: &[f64], intercept: f64) → f64` | Epigenetic clock age prediction | `biology::epigenetics::inheritance` |
| `environmental_epigenetic_response` | `fn environmental_epigenetic_response(stress: f64, sensitivity: f64, methylation_change_rate: f64, baseline_methylation: f64) → f64` | Environmental epigenetic response | `biology::epigenetics::inheritance` |

#### methylation.rs (10 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `cpg_methylation_level` | `fn cpg_methylation_level(methylated: usize, total_cpg: usize) → f64` | CpG methylation level | `biology::epigenetics::methylation` |
| `methylation_decay` | `fn methylation_decay(level: f64, dilution_rate: f64, maintenance_efficiency: f64, generations: usize) → Vec<f64>` | Methylation decay over generations | `biology::epigenetics::methylation` |
| `de_novo_methylation` | `fn de_novo_methylation(unmethylated: f64, rate: f64, dt: f64) → f64` | De novo methylation rate | `biology::epigenetics::methylation` |
| `bisulfite_conversion_efficiency` | `fn bisulfite_conversion_efficiency(converted_c: usize, total_c: usize) → f64` | Bisulfite conversion efficiency | `biology::epigenetics::methylation` |
| `methylation_entropy` | `fn methylation_entropy(profile: &[f64]) → f64` | Methylation entropy | `biology::epigenetics::methylation` |
| `hydroxymethylation_rate` | `fn hydroxymethylation_rate(methylation: f64, tet_activity: f64) → f64` | TET hydroxymethylation rate | `biology::epigenetics::methylation` |
| `tet_oxidation_cascade` | `fn tet_oxidation_cascade(mc: f64, tet: f64, dt: f64) → (f64, f64, f64)` | TET oxidation cascade 5mC→5hmC→5fC | `biology::epigenetics::methylation` |
| `dnmt_processivity` | `fn dnmt_processivity(initial_methylation: &[f64], processivity: f64, direction_forward: bool) → Vec<f64>` | DNMT processive methylation | `biology::epigenetics::methylation` |
| `cpg_island_density` | `fn cpg_island_density(sequence_length: usize, cpg_count: usize, expected_cpg: f64) → f64` | CpG island density | `biology::epigenetics::methylation` |
| `age_methylation_predictor` | `fn age_methylation_predictor(cpg_beta_values: &[f64], weights: &[f64], offset: f64) → f64` | Age prediction from methylation | `biology::epigenetics::methylation` |

#### noncoding_rna.rs (10 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `mirna_target_repression` | `fn mirna_target_repression(mirna_conc: f64, target_mrna: f64, kd: f64, max_repression: f64) → f64` | miRNA target repression | `biology::epigenetics::noncoding_rna` |
| `mirna_seed_match_score` | `fn mirna_seed_match_score(seed_matches: usize, three_prime_pairing: f64, site_accessibility: f64) → f64` | miRNA seed match score | `biology::epigenetics::noncoding_rna` |
| `lncrna_scaffold_activity` | `fn lncrna_scaffold_activity(lncrna_conc: f64, protein_a: f64, protein_b: f64, kd_a: f64, kd_b: f64) → f64` | lncRNA scaffold activity | `biology::epigenetics::noncoding_rna` |
| `xist_silencing_spread` | `fn xist_silencing_spread(xist_expression: f64, distance_from_xic: f64, spread_rate: f64) → f64` | Xist silencing spread | `biology::epigenetics::noncoding_rna` |
| `pirna_transposon_silencing` | `fn pirna_transposon_silencing(pirna_conc: f64, transposon_copies: f64, silencing_efficiency: f64) → f64` | piRNA transposon silencing | `biology::epigenetics::noncoding_rna` |
| `circular_rna_mirna_sponge` | `fn circular_rna_mirna_sponge(circrna: f64, binding_sites: f64, mirna_total: f64, kd: f64) → f64` | circRNA miRNA sponge | `biology::epigenetics::noncoding_rna` |
| `rnai_knockdown_efficiency` | `fn rnai_knockdown_efficiency(sirna_conc: f64, target_mrna: f64, risc_activity: f64, kd: f64) → f64` | RNAi knockdown efficiency | `biology::epigenetics::noncoding_rna` |
| `enhancer_rna_activity` | `fn enhancer_rna_activity(erna_level: f64, enhancer_activity_base: f64, amplification: f64) → f64` | eRNA enhancer activity | `biology::epigenetics::noncoding_rna` |
| `antisense_rna_regulation` | `fn antisense_rna_regulation(sense_mrna: f64, antisense_rna: f64, duplex_rate: f64, degradation_rate: f64) → f64` | Antisense RNA regulation | `biology::epigenetics::noncoding_rna` |
| `ncrna_mediated_methylation` | `fn ncrna_mediated_methylation(ncrna_guide: f64, target_region_accessibility: f64, dnmt_activity: f64) → f64` | ncRNA-directed methylation | `biology::epigenetics::noncoding_rna` |

### 🔟 bioinformatics/ — Bioinformatics — 51 pub fn

#### alignment.rs (9 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `smith_waterman_score` | `fn smith_waterman_score(seq1: &[u8], seq2: &[u8], match_score: i32, mismatch: i32, gap: i32) → i32` | Smith-Waterman local alignment | `biology::bioinformatics::alignment` |
| `needleman_wunsch_score` | `fn needleman_wunsch_score(seq1: &[u8], seq2: &[u8], match_score: i32, mismatch: i32, gap: i32) → i32` | Needleman-Wunsch global alignment | `biology::bioinformatics::alignment` |
| `edit_distance` | `fn edit_distance(seq1: &[u8], seq2: &[u8]) → usize` | Levenshtein edit distance | `biology::bioinformatics::alignment` |
| `hamming_distance` | `fn hamming_distance(seq1: &[u8], seq2: &[u8]) → usize` | Hamming distance | `biology::bioinformatics::alignment` |
| `alignment_gc_content` | `fn alignment_gc_content(seq: &[u8]) → f64` | GC content of alignment | `biology::bioinformatics::alignment` |
| `sequence_identity` | `fn sequence_identity(seq1: &[u8], seq2: &[u8]) → f64` | Sequence identity fraction | `biology::bioinformatics::alignment` |
| `codon_frequency` | `fn codon_frequency(seq: &[u8]) → Vec<(u32, usize)>` | Codon frequency table | `biology::bioinformatics::alignment` |
| `reverse_complement` | `fn reverse_complement(seq: &[u8]) → Vec<u8>` | Reverse complement | `biology::bioinformatics::alignment` |
| `melting_temperature_basic` | `fn melting_temperature_basic(a_count: usize, t_count: usize, g_count: usize, c_count: usize) → f64` | $T_m = 2(A+T) + 4(G+C)$ | `biology::bioinformatics::alignment` |

#### assembly.rs (14 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `assembly_n50` | `fn assembly_n50(contig_lengths: &[usize]) → usize` | Assembly N50 metric | `biology::bioinformatics::assembly` |
| `n_metric` | `fn n_metric(contig_lengths: &[usize], fraction: f64) → usize` | Generalized N-metric | `biology::bioinformatics::assembly` |
| `l50` | `fn l50(contig_lengths: &[usize]) → usize` | L50 contig count | `biology::bioinformatics::assembly` |
| `genome_coverage` | `fn genome_coverage(total_bases_sequenced: usize, genome_size: usize) → f64` | Genome coverage depth | `biology::bioinformatics::assembly` |
| `lander_waterman` | `fn lander_waterman(coverage: f64) → f64` | $P = 1 - e^{-c}$ — Lander-Waterman | `biology::bioinformatics::assembly` |
| `expected_contigs` | `fn expected_contigs(n_reads: usize, read_length: usize, genome_size: usize, overlap: usize) → f64` | Expected contig count | `biology::bioinformatics::assembly` |
| `assembly_completeness` | `fn assembly_completeness(aligned_bases: usize, reference_size: usize) → f64` | Assembly completeness | `biology::bioinformatics::assembly` |
| `gc_content_reads` | `fn gc_content_reads(reads: &[&[u8]]) → f64` | GC content across reads | `biology::bioinformatics::assembly` |
| `ng50` | `fn ng50(contig_lengths: &[usize], genome_size: usize) → usize` | NG50 metric | `biology::bioinformatics::assembly` |
| `aunga` | `fn aunga(contig_lengths: &[usize], genome_size: usize) → f64` | auNGA metric | `biology::bioinformatics::assembly` |
| `misassembly_rate` | `fn misassembly_rate(misassemblies: usize, total_contigs: usize) → f64` | Misassembly rate | `biology::bioinformatics::assembly` |
| `chimeric_contig_fraction` | `fn chimeric_contig_fraction(chimeric: usize, total: usize) → f64` | Chimeric contig fraction | `biology::bioinformatics::assembly` |
| `contig_size_distribution` | `fn contig_size_distribution(contig_lengths: &[usize]) → (f64, f64, usize, usize)` | Contig size distribution stats | `biology::bioinformatics::assembly` |
| `expected_gap_count` | `fn expected_gap_count(coverage: f64, genome_size: usize, read_length: usize) → f64` | Expected gap count | `biology::bioinformatics::assembly` |

#### expression.rs (12 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `principal_component_variance` | `fn principal_component_variance(eigenvalues: &[f64], component: usize) → f64` | PCA variance explained | `biology::bioinformatics::expression` |
| `manhattan_distance_features` | `fn manhattan_distance_features(a: &[f64], b: &[f64]) → f64` | Manhattan distance | `biology::bioinformatics::expression` |
| `euclidean_distance_features` | `fn euclidean_distance_features(a: &[f64], b: &[f64]) → f64` | Euclidean distance | `biology::bioinformatics::expression` |
| `pearson_correlation` | `fn pearson_correlation(x: &[f64], y: &[f64]) → f64` | Pearson correlation coefficient | `biology::bioinformatics::expression` |
| `fold_change` | `fn fold_change(treatment: f64, control: f64) → f64` | Fold change | `biology::bioinformatics::expression` |
| `log2_fold_change` | `fn log2_fold_change(treatment: f64, control: f64) → f64` | $\log_2$ fold change | `biology::bioinformatics::expression` |
| `rpkm` | `fn rpkm(read_count: f64, gene_length_kb: f64, total_reads_millions: f64) → f64` | RPKM normalization | `biology::bioinformatics::expression` |
| `tpm` | `fn tpm(read_count: f64, gene_length_kb: f64, sum_rpk: f64) → f64` | TPM normalization | `biology::bioinformatics::expression` |
| `fpkm` | `fn fpkm(fragment_count: f64, gene_length_kb: f64, total_fragments_millions: f64) → f64` | FPKM normalization | `biology::bioinformatics::expression` |
| `deseq2_size_factor` | `fn deseq2_size_factor(counts: &[f64], geometric_means: &[f64]) → f64` | DESeq2 size factor | `biology::bioinformatics::expression` |
| `benjamini_hochberg` | `fn benjamini_hochberg(p_values: &mut [(usize, f64)]) → Vec<(usize, f64)>` | Benjamini-Hochberg FDR correction | `biology::bioinformatics::expression` |
| `volcano_significant` | `fn volcano_significant(log2fc: f64, p_value: f64, fc_threshold: f64, p_threshold: f64) → bool` | Volcano plot significance | `biology::bioinformatics::expression` |

#### quality.rs (16 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `phred_to_probability` | `fn phred_to_probability(phred: f64) → f64` | $P = 10^{-Q/10}$ | `biology::bioinformatics::quality` |
| `probability_to_phred` | `fn probability_to_phred(p: f64) → f64` | $Q = -10\log_{10}(P)$ | `biology::bioinformatics::quality` |
| `average_quality` | `fn average_quality(qualities: &[u8]) → f64` | Average Phred quality | `biology::bioinformatics::quality` |
| `quality_filter` | `fn quality_filter(qualities: &[u8], min_quality: u8, window: usize, min_fraction: f64) → bool` | Sliding window quality filter | `biology::bioinformatics::quality` |
| `expected_errors` | `fn expected_errors(qualities: &[u8]) → f64` | Expected sequencing errors | `biology::bioinformatics::quality` |
| `trim_quality` | `fn trim_quality(qualities: &[u8], min_quality: u8) → usize` | Quality trimming position | `biology::bioinformatics::quality` |
| `n50` | `fn n50(lengths: &[usize]) → usize` | N50 read length metric | `biology::bioinformatics::quality` |
| `gc_content` | `fn gc_content(sequence: &[u8]) → f64` | GC content | `biology::bioinformatics::quality` |
| `adapter_match_score` | `fn adapter_match_score(read: &[u8], adapter: &[u8]) → usize` | Adapter match score | `biology::bioinformatics::quality` |
| `complexity_dust` | `fn complexity_dust(sequence: &[u8], window: usize) → f64` | DUST low-complexity score | `biology::bioinformatics::quality` |
| `kmer_frequency` | `fn kmer_frequency(sequence: &[u8], k: usize) → Vec<(Vec<u8>, usize)>` | $k$-mer frequency | `biology::bioinformatics::quality` |
| `shannon_entropy_sequence` | `fn shannon_entropy_sequence(sequence: &[u8]) → f64` | Shannon entropy | `biology::bioinformatics::quality` |
| `sliding_window_quality` | `fn sliding_window_quality(qualities: &[u8], window: usize) → Vec<f64>` | Sliding window quality scores | `biology::bioinformatics::quality` |
| `per_base_quality_distribution` | `fn per_base_quality_distribution(quality_matrix: &[Vec<u8>]) → Vec<(f64, f64)>` | Per-base quality distribution | `biology::bioinformatics::quality` |
| `duplication_rate` | `fn duplication_rate(total_reads: usize, unique_reads: usize) → f64` | Read duplication rate | `biology::bioinformatics::quality` |
| `chimera_breakpoint_score` | `fn chimera_breakpoint_score(alignment_a: usize, alignment_b: usize, read_length: usize) → f64` | Chimera breakpoint score | `biology::bioinformatics::quality` |

### 1️⃣1️⃣ cell/ — Cell Biology — 78 pub fn

#### adhesion.rs (12 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `cell_adhesion_energy` | `fn cell_adhesion_energy(contact_area: f64, cadherin_density: f64, bond_energy: f64) → f64` | $E = A \cdot \rho \cdot E_b$ | `biology::cell::adhesion` |
| `integrin_focal_adhesion_force` | `fn integrin_focal_adhesion_force(integrin_count: f64, force_per_integrin: f64) → f64` | $F = n \cdot f$ | `biology::cell::adhesion` |
| `adhesion_receptor_binding` | `fn adhesion_receptor_binding(ligand: f64, receptor: f64, kd: f64) → f64` | $B = \frac{L \cdot R}{L + K_d}$ | `biology::cell::adhesion` |
| `cell_cell_junction_strength` | `fn cell_cell_junction_strength(tight_junction: f64, adherens_junction: f64, desmosome: f64) → f64` | Total junction strength | `biology::cell::adhesion` |
| `chemotaxis_velocity` | `fn chemotaxis_velocity(gradient: f64, sensitivity: f64, max_speed: f64) → f64` | Chemotaxis velocity | `biology::cell::adhesion` |
| `haptotaxis_velocity` | `fn haptotaxis_velocity(ecm_gradient: f64, adhesion_strength: f64, drag: f64) → f64` | Haptotaxis velocity | `biology::cell::adhesion` |
| `durotaxis_force` | `fn durotaxis_force(stiffness_gradient: f64, cell_contractility: f64) → f64` | Durotaxis force | `biology::cell::adhesion` |
| `collective_migration_speed` | `fn collective_migration_speed(leader_speed: f64, follower_count: usize, coupling: f64) → f64` | Collective cell migration | `biology::cell::adhesion` |
| `wound_healing_rate` | `fn wound_healing_rate(gap_width: f64, migration_speed: f64, proliferation_rate: f64) → f64` | Wound healing rate | `biology::cell::adhesion` |
| `ecm_remodeling_rate` | `fn ecm_remodeling_rate(mmp_activity: f64, timp_activity: f64, substrate: f64) → f64` | ECM remodeling (MMP/TIMP) | `biology::cell::adhesion` |
| `cell_spreading_area` | `fn cell_spreading_area(adhesion_strength: f64, cortical_tension: f64) → f64` | Cell spreading area | `biology::cell::adhesion` |
| `catch_bond_lifetime` | `fn catch_bond_lifetime(force: f64, k0: f64, x1: f64, x2: f64) → f64` | Catch bond lifetime | `biology::cell::adhesion` |

#### cycle.rs (20 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `cell_cycle_ode` | `fn cell_cycle_ode(cyclin: f64, cdk: f64, apc: f64, k_syn: f64, k_deg: f64, k_act: f64, k_inact: f64) → (f64, f64, f64)` | Cell cycle ODE (cyclin/CDK/APC) | `biology::cell::cycle` |
| `cell_cycle_simulate` | `fn cell_cycle_simulate(cyclin0: f64, cdk0: f64, apc0: f64, k_syn: f64, k_deg: f64, k_act: f64, k_inact: f64, dt: f64, steps: usize) → Vec<(f64, f64, f64)>` | Cell cycle simulation | `biology::cell::cycle` |
| `mitotic_index` | `fn mitotic_index(dividing_cells: usize, total_cells: usize) → f64` | $MI = \frac{n_{div}}{n_{total}}$ | `biology::cell::cycle` |
| `cell_growth_logistic` | `fn cell_growth_logistic(n: f64, r: f64, k: f64, dt: f64, steps: usize) → Vec<f64>` | $\frac{dN}{dt} = rN\left(1 - \frac{N}{K}\right)$ | `biology::cell::cycle` |
| `g1_checkpoint` | `fn g1_checkpoint(dna_damage: f64, p53_threshold: f64, rb_active: f64) → bool` | G1 checkpoint (p53/Rb) | `biology::cell::cycle` |
| `g2_checkpoint` | `fn g2_checkpoint(dna_damage: f64, repair_capacity: f64, cdk1_activity: f64) → bool` | G2 checkpoint (DNA repair) | `biology::cell::cycle` |
| `spindle_assembly_checkpoint` | `fn spindle_assembly_checkpoint(unattached_kinetochores: usize) → bool` | Spindle assembly checkpoint | `biology::cell::cycle` |
| `apoptosis_probability` | `fn apoptosis_probability(dna_damage: f64, p53: f64, bcl2: f64, bax: f64) → f64` | Apoptosis probability (p53/Bcl2/Bax) | `biology::cell::cycle` |
| `cell_doubling_time` | `fn cell_doubling_time(growth_rate: f64) → f64` | $t_d = \frac{\ln 2}{r}$ | `biology::cell::cycle` |
| `contact_inhibition` | `fn contact_inhibition(density: f64, max_density: f64, steepness: f64) → f64` | Contact inhibition | `biology::cell::cycle` |
| `phase_duration` | `fn phase_duration(total_cycle_time: f64, g1_fraction: f64, s_fraction: f64, g2_fraction: f64) → (f64, f64, f64, f64)` | Cell cycle phase durations (G1/S/G2/M) | `biology::cell::cycle` |
| `dna_damage_accumulation` | `fn dna_damage_accumulation(damage: f64, production_rate: f64, repair_rate: f64, dt: f64) → f64` | DNA damage accumulation | `biology::cell::cycle` |
| `restriction_point` | `fn restriction_point(growth_factor: f64, threshold: f64, rb_phosphorylation: f64) → bool` | Restriction point (Rb phosphorylation) | `biology::cell::cycle` |
| `cyclin_oscillator` | `fn cyclin_oscillator(cyclin: f64, cdk: f64, k_syn: f64, k_deg: f64) → f64` | Cyclin oscillator | `biology::cell::cycle` |
| `cell_senescence_probability` | `fn cell_senescence_probability(telomere_length: f64, critical_length: f64, dna_damage: f64) → f64` | Cell senescence probability | `biology::cell::cycle` |
| `proliferation_index` | `fn proliferation_index(s_phase: usize, g2m_phase: usize, total: usize) → f64` | Proliferation index | `biology::cell::cycle` |
| `growth_fraction` | `fn growth_fraction(proliferating: usize, total: usize) → f64` | Growth fraction $GF = \frac{n_p}{n_{total}}$ | `biology::cell::cycle` |
| `cell_loss_factor` | `fn cell_loss_factor(growth_rate: f64, doubling_time: f64) → f64` | Cell loss factor | `biology::cell::cycle` |
| `hayflick_limit` | `fn hayflick_limit(initial_telomere: f64, loss_per_division: f64, critical: f64) → f64` | Hayflick limit (telomere divisions) | `biology::cell::cycle` |
| `quiescence_entry` | `fn quiescence_entry(growth_factor: f64, nutrient: f64, gf_threshold: f64, nutrient_threshold: f64) → bool` | Quiescence entry (G0) | `biology::cell::cycle` |

#### organelles.rs (13 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `autophagy_flux` | `fn autophagy_flux(lc3_ii: f64, p62: f64, bafilomycin_effect: f64) → f64` | Autophagy flux (LC3-II/p62) | `biology::cell::organelles` |
| `proteasome_degradation_rate` | `fn proteasome_degradation_rate(ubiquitin_tags: f64, proteasome_activity: f64, km: f64) → f64` | Proteasome degradation rate | `biology::cell::organelles` |
| `lysosome_ph` | `fn lysosome_ph(v_atpase_rate: f64, proton_leak: f64, buffer_capacity: f64, volume: f64) → f64` | Lysosome pH regulation | `biology::cell::organelles` |
| `endosome_maturation` | `fn endosome_maturation(rab5: f64, rab7: f64, conversion_rate: f64, dt: f64) → (f64, f64)` | Endosome maturation (Rab5→Rab7) | `biology::cell::organelles` |
| `receptor_recycling` | `fn receptor_recycling(internalized: f64, recycling_rate: f64, degradation_rate: f64) → f64` | Receptor recycling | `biology::cell::organelles` |
| `mitochondrial_fission_rate` | `fn mitochondrial_fission_rate(drp1: f64, fis1: f64, threshold: f64) → f64` | Mitochondrial fission (Drp1/Fis1) | `biology::cell::organelles` |
| `mitochondrial_fusion_rate` | `fn mitochondrial_fusion_rate(mfn1: f64, mfn2: f64, opa1: f64) → f64` | Mitochondrial fusion (Mfn/OPA1) | `biology::cell::organelles` |
| `er_stress_upr` | `fn er_stress_upr(misfolded: f64, bip: f64, ire1_threshold: f64) → f64` | ER stress / UPR response | `biology::cell::organelles` |
| `golgi_transport_rate` | `fn golgi_transport_rate(cargo: f64, coat_protein: f64, gtp: f64, km_coat: f64) → f64` | Golgi transport rate | `biology::cell::organelles` |
| `peroxisome_beta_oxidation` | `fn peroxisome_beta_oxidation(vlcfa: f64, enzyme_activity: f64, km: f64) → f64` | Peroxisome β-oxidation | `biology::cell::organelles` |
| `cytoskeleton_treadmilling` | `fn cytoskeleton_treadmilling(polymerization_rate: f64, depolymerization_rate: f64) → f64` | Cytoskeleton treadmilling | `biology::cell::organelles` |
| `nuclear_import_rate` | `fn nuclear_import_rate(cargo: f64, importin: f64, ran_gtp: f64, kd: f64) → f64` | Nuclear import rate (importin/Ran-GTP) | `biology::cell::organelles` |
| `cell_volume_regulation` | `fn cell_volume_regulation(volume: f64, target_volume: f64, permeability: f64, osmotic_difference: f64) → f64` | Cell volume regulation | `biology::cell::organelles` |

#### signaling.rs (16 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `ligand_receptor_binding` | `fn ligand_receptor_binding(l: f64, r_total: f64, kd: f64) → f64` | $B = \frac{R_T \cdot L}{L + K_d}$ | `biology::cell::signaling` |
| `hill_response` | `fn hill_response(signal: f64, k: f64, n: f64) → f64` | $R = \frac{S^n}{K^n + S^n}$ | `biology::cell::signaling` |
| `bistable_switch` | `fn bistable_switch(x: f64, k1: f64, k2: f64, n: f64, alpha: f64, beta: f64) → f64` | Bistable switch dynamics | `biology::cell::signaling` |
| `bistable_simulate` | `fn bistable_simulate(x0: f64, k1: f64, k2: f64, n: f64, alpha: f64, beta: f64, dt: f64, steps: usize) → Vec<f64>` | Bistable switch simulation | `biology::cell::signaling` |
| `mapk_cascade` | `fn mapk_cascade(raf: f64, mek: f64, erk: f64, signal: f64, k_activate: &[f64; 3], k_deactivate: &[f64; 3]) → (f64, f64, f64)` | MAPK cascade (Raf/MEK/ERK) | `biology::cell::signaling` |
| `mapk_simulate` | `fn mapk_simulate(raf0: f64, mek0: f64, erk0: f64, signal: f64, k_activate: &[f64; 3], k_deactivate: &[f64; 3], dt: f64, steps: usize) → Vec<(f64, f64, f64)>` | MAPK cascade simulation | `biology::cell::signaling` |
| `goldbeter_koshland` | `fn goldbeter_koshland(v1: f64, v2: f64, j1: f64, j2: f64) → f64` | Goldbeter-Koshland ultrasensitivity | `biology::cell::signaling` |
| `negative_feedback` | `fn negative_feedback(output: f64, k_prod: f64, k_deg: f64, k_inh: f64, n: f64) → f64` | Negative feedback loop | `biology::cell::signaling` |
| `positive_feedback` | `fn positive_feedback(x: f64, basal: f64, vmax: f64, k: f64, n: f64, deg: f64) → f64` | Positive feedback loop | `biology::cell::signaling` |
| `receptor_desensitization` | `fn receptor_desensitization(active: f64, ligand: f64, kd: f64, k_intern: f64, k_recycle: f64, total: f64) → f64` | Receptor desensitization | `biology::cell::signaling` |
| `dual_phosphorylation` | `fn dual_phosphorylation(x: f64, kinase: f64, phosphatase: f64, k1: f64, k2: f64) → f64` | Dual phosphorylation | `biology::cell::signaling` |
| `coherent_feedforward` | `fn coherent_feedforward(signal: f64, x: f64, k_sx: f64, k_xy: f64, k_sy: f64, threshold: f64) → f64` | Coherent feed-forward loop | `biology::cell::signaling` |
| `incoherent_feedforward` | `fn incoherent_feedforward(signal: f64, x: f64, k_activation: f64, k_repression: f64) → f64` | Incoherent feed-forward loop | `biology::cell::signaling` |
| `michaelis_menten_cascade` | `fn michaelis_menten_cascade(substrate: f64, enzyme: f64, km: f64, vmax: f64) → f64` | Michaelis-Menten cascade | `biology::cell::signaling` |
| `scaffold_complex_formation` | `fn scaffold_complex_formation(a: f64, b: f64, scaffold: f64, ka: f64, kb: f64) → f64` | Scaffold complex formation | `biology::cell::signaling` |
| `crosstalk_inhibition` | `fn crosstalk_inhibition(pathway_a: f64, pathway_b: f64, k_inh_ab: f64, k_inh_ba: f64) → (f64, f64)` | Pathway crosstalk inhibition | `biology::cell::signaling` |

#### transport.rs (17 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `fick_first_law` | `fn fick_first_law(d: f64, dc_dx: f64) → f64` | $J = -D \frac{dC}{dx}$ | `biology::cell::transport` |
| `fick_second_law_1d` | `fn fick_second_law_1d(conc: &mut [f64], d: f64, dx: f64, dt: f64, steps: usize)` | $\frac{\partial C}{\partial t} = D \frac{\partial^2 C}{\partial x^2}$ | `biology::cell::transport` |
| `nernst_potential` | `fn nernst_potential(z: f64, t: f64, c_out: f64, c_in: f64) → f64` | $E = \frac{RT}{zF} \ln\frac{[C]_o}{[C]_i}$ | `biology::cell::transport` |
| `goldman_equation` | `fn goldman_equation(p_na: f64, p_k: f64, p_cl: f64, na_out: f64, na_in: f64, k_out: f64, k_in: f64, cl_out: f64, cl_in: f64, t: f64) → f64` | Goldman-Hodgkin-Katz equation | `biology::cell::transport` |
| `osmotic_pressure` | `fn osmotic_pressure(c: f64, t: f64) → f64` | $\Pi = cRT$ (van 't Hoff) | `biology::cell::transport` |
| `donnan_ratio` | `fn donnan_ratio(z_ion: f64, z_macro: f64, c_macro: f64, c_salt: f64) → f64` | Donnan equilibrium ratio | `biology::cell::transport` |
| `active_transport_rate` | `fn active_transport_rate(vmax: f64, substrate: f64, km: f64, atp: f64, km_atp: f64) → f64` | Active transport rate (ATP-dependent) | `biology::cell::transport` |
| `membrane_capacitance_current` | `fn membrane_capacitance_current(cm: f64, dv_dt: f64) → f64` | $I_C = C_m \frac{dV}{dt}$ | `biology::cell::transport` |
| `electrochemical_gradient` | `fn electrochemical_gradient(z: f64, vm: f64, equilibrium_potential: f64) → f64` | Electrochemical gradient | `biology::cell::transport` |
| `vesicle_fusion_rate` | `fn vesicle_fusion_rate(calcium: f64, kd: f64, n: f64, k_max: f64) → f64` | Vesicle fusion rate (Ca²⁺-dependent) | `biology::cell::transport` |
| `endocytosis_rate` | `fn endocytosis_rate(receptor_bound: f64, k_intern: f64, coat_protein: f64, kd_coat: f64) → f64` | Endocytosis rate | `biology::cell::transport` |
| `exocytosis_rate` | `fn exocytosis_rate(vesicles: f64, calcium: f64, kd: f64) → f64` | Exocytosis rate | `biology::cell::transport` |
| `gap_junction_flux` | `fn gap_junction_flux(c1: f64, c2: f64, permeability: f64) → f64` | Gap junction flux | `biology::cell::transport` |
| `facilitated_diffusion` | `fn facilitated_diffusion(c_out: f64, c_in: f64, vmax: f64, km: f64) → f64` | Facilitated diffusion | `biology::cell::transport` |
| `cotransport_rate` | `fn cotransport_rate(substrate: f64, ion: f64, vmax: f64, km_s: f64, km_i: f64) → f64` | Cotransport rate | `biology::cell::transport` |
| `pinocytosis_uptake` | `fn pinocytosis_uptake(volume_rate: f64, concentration: f64) → f64` | Pinocytosis uptake | `biology::cell::transport` |
| `ion_channel_conductance` | `fn ion_channel_conductance(g_max: f64, open_probability: f64, driving_force: f64) → f64` | $I = g_{max} \cdot P_o \cdot (V - E)$ | `biology::cell::transport` |

---

### 1️⃣2️⃣ enzyme/ — Enzymology — 50 pub fn

#### kinetics.rs (9 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `michaelis_menten` | `fn michaelis_menten(s: f64, vmax: f64, km: f64) → f64` | $v = \frac{V_{max} [S]}{K_m + [S]}$ | `biology::enzyme::kinetics` |
| `michaelis_menten_competitive` | `fn michaelis_menten_competitive(s: f64, vmax: f64, km: f64, inhibitor: f64, ki: f64) → f64` | Competitive inhibition kinetics | `biology::enzyme::kinetics` |
| `michaelis_menten_uncompetitive` | `fn michaelis_menten_uncompetitive(s: f64, vmax: f64, km: f64, inhibitor: f64, ki: f64) → f64` | Uncompetitive inhibition kinetics | `biology::enzyme::kinetics` |
| `michaelis_menten_noncompetitive` | `fn michaelis_menten_noncompetitive(s: f64, vmax: f64, km: f64, inhibitor: f64, ki: f64) → f64` | Non-competitive inhibition kinetics | `biology::enzyme::kinetics` |
| `hill_equation` | `fn hill_equation(s: f64, vmax: f64, k: f64, n: f64) → f64` | $v = \frac{V_{max} [S]^n}{K^n + [S]^n}$ | `biology::enzyme::kinetics` |
| `lineweaver_burk` | `fn lineweaver_burk(s: &[f64], v: &[f64]) → (f64, f64)` | Lineweaver-Burk double reciprocal | `biology::enzyme::kinetics` |
| `eadie_hofstee` | `fn eadie_hofstee(v: &[f64], s: &[f64]) → (f64, f64)` | Eadie-Hofstee plot | `biology::enzyme::kinetics` |
| `kcat` | `fn kcat(vmax: f64, e_total: f64) → f64` | $k_{cat} = \frac{V_{max}}{[E]_T}$ | `biology::enzyme::kinetics` |
| `catalytic_efficiency` | `fn catalytic_efficiency(kcat_val: f64, km: f64) → f64` | $\eta = \frac{k_{cat}}{K_m}$ | `biology::enzyme::kinetics` |

#### inhibition.rs (11 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `competitive_inhibition` | `fn competitive_inhibition(substrate: f64, inhibitor: f64, vmax: f64, km: f64, ki: f64) → f64` | $v = \frac{V_{max}[S]}{K_m(1+\frac{[I]}{K_i})+[S]}$ | `biology::enzyme::inhibition` |
| `uncompetitive_inhibition` | `fn uncompetitive_inhibition(substrate: f64, inhibitor: f64, vmax: f64, km: f64, ki: f64) → f64` | Uncompetitive inhibition | `biology::enzyme::inhibition` |
| `mixed_inhibition` | `fn mixed_inhibition(substrate: f64, inhibitor: f64, vmax: f64, km: f64, ki: f64, ki_prime: f64) → f64` | Mixed inhibition ($K_i$ and $K_i'$) | `biology::enzyme::inhibition` |
| `noncompetitive_inhibition` | `fn noncompetitive_inhibition(substrate: f64, inhibitor: f64, vmax: f64, km: f64, ki: f64) → f64` | Non-competitive inhibition | `biology::enzyme::inhibition` |
| `substrate_inhibition_velocity` | `fn substrate_inhibition_velocity(substrate: f64, vmax: f64, km: f64, ksi: f64) → f64` | Substrate inhibition | `biology::enzyme::inhibition` |
| `irreversible_inhibition` | `fn irreversible_inhibition(active_enzyme: f64, inhibitor: f64, k_inact: f64, t: f64) → f64` | Irreversible inhibition | `biology::enzyme::inhibition` |
| `tight_binding_inhibition` | `fn tight_binding_inhibition(enzyme_total: f64, inhibitor_total: f64, ki_app: f64) → f64` | Tight-binding inhibition | `biology::enzyme::inhibition` |
| `ic50_to_ki` | `fn ic50_to_ki(ic50: f64, substrate: f64, km: f64) → f64` | $K_i = \frac{IC_{50}}{1 + \frac{[S]}{K_m}}$ (Cheng-Prusoff) | `biology::enzyme::inhibition` |
| `ki_to_ic50` | `fn ki_to_ic50(ki: f64, substrate: f64, km: f64) → f64` | $IC_{50} = K_i(1 + \frac{[S]}{K_m})$ | `biology::enzyme::inhibition` |
| `cheng_prusoff_uncompetitive` | `fn cheng_prusoff_uncompetitive(ic50: f64, substrate: f64, km: f64) → f64` | Cheng-Prusoff (uncompetitive) | `biology::enzyme::inhibition` |
| `inhibition_constant_dixon` | `fn inhibition_constant_dixon(v_no_inhibitor: f64, v_with_inhibitor: f64, inhibitor: f64, substrate: f64, km: f64) → f64` | Dixon plot $K_i$ determination | `biology::enzyme::inhibition` |

#### metabolism.rs (10 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `metabolic_network_steady_state` | `fn metabolic_network_steady_state(stoich: &[Vec<f64>], rates: impl Fn(&[f64]) → Vec<f64>, initial: &[f64], dt: f64, steps: usize, tolerance: f64) → Vec<f64>` | Metabolic network steady state | `biology::enzyme::metabolism` |
| `arrhenius` | `fn arrhenius(a: f64, ea: f64, t: f64) → f64` | $k = A e^{-E_a/RT}$ | `biology::enzyme::metabolism` |
| `q10_factor` | `fn q10_factor(rate1: f64, rate2: f64, t1: f64, t2: f64) → f64` | $Q_{10} = \left(\frac{R_2}{R_1}\right)^{10/(T_2-T_1)}$ | `biology::enzyme::metabolism` |
| `metabolic_control_coefficient` | `fn metabolic_control_coefficient(flux_perturbed: f64, flux_original: f64, enzyme_perturbed: f64, enzyme_original: f64) → f64` | Metabolic control coefficient | `biology::enzyme::metabolism` |
| `gibbs_free_energy` | `fn gibbs_free_energy(delta_g0: f64, t: f64, q: f64) → f64` | $\Delta G = \Delta G^0 + RT \ln Q$ | `biology::enzyme::metabolism` |
| `mass_action_ratio` | `fn mass_action_ratio(products: &[f64], reactants: &[f64], stoich_p: &[f64], stoich_r: &[f64]) → f64` | Mass action ratio $\Gamma$ | `biology::enzyme::metabolism` |
| `reaction_quotient_vs_keq` | `fn reaction_quotient_vs_keq(q: f64, keq: f64) → f64` | $\frac{Q}{K_{eq}}$ displacement | `biology::enzyme::metabolism` |
| `flux_control_summation` | `fn flux_control_summation(coefficients: &[f64]) → f64` | Flux control summation theorem $\sum C_i^J = 1$ | `biology::enzyme::metabolism` |
| `elasticity_coefficient` | `fn elasticity_coefficient(rate: f64, metabolite: f64, delta_rate: f64, delta_metabolite: f64) → f64` | Elasticity coefficient $\varepsilon$ | `biology::enzyme::metabolism` |
| `supply_demand_modular` | `fn supply_demand_modular(supply_flux: f64, demand_flux: f64, linking_metabolite: f64, elasticity_supply: f64, elasticity_demand: f64) → f64` | Supply-demand modular rate analysis | `biology::enzyme::metabolism` |

#### mechanisms.rs (10 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `enzyme_kinetics_solve` | `fn enzyme_kinetics_solve(s0: f64, e0: f64, k1: f64, k_1: f64, k2: f64, dt: f64, steps: usize) → Vec<(f64, f64, f64)>` | Full enzyme kinetics ODE solver | `biology::enzyme::mechanisms` |
| `ping_pong` | `fn ping_pong(a: f64, b: f64, vmax: f64, ka: f64, kb: f64) → f64` | Ping-pong (double displacement) | `biology::enzyme::mechanisms` |
| `ordered_bi_bi` | `fn ordered_bi_bi(a: f64, b: f64, vmax: f64, ka: f64, kb: f64, kia: f64) → f64` | Ordered Bi-Bi mechanism | `biology::enzyme::mechanisms` |
| `random_bi_bi` | `fn random_bi_bi(a: f64, b: f64, vmax: f64, ka: f64, kb: f64, kia: f64, kib: f64) → f64` | Random Bi-Bi mechanism | `biology::enzyme::mechanisms` |
| `substrate_inhibition` | `fn substrate_inhibition(s: f64, vmax: f64, km: f64, ki: f64) → f64` | Substrate inhibition | `biology::enzyme::mechanisms` |
| `allosteric_enzyme` | `fn allosteric_enzyme(s: f64, vmax: f64, k05: f64, n_hill: f64) → f64` | Allosteric enzyme (Hill) | `biology::enzyme::mechanisms` |
| `covalent_modification_cycle` | `fn covalent_modification_cycle(substrate: f64, kinase_vmax: f64, kinase_km: f64, phosphatase_vmax: f64, phosphatase_km: f64) → f64` | Covalent modification cycle | `biology::enzyme::mechanisms` |
| `enzyme_activation_energy` | `fn enzyme_activation_energy(k_cat: f64, temperature: f64) → f64` | Activation energy from $k_{cat}$ | `biology::enzyme::mechanisms` |
| `suicide_inhibition` | `fn suicide_inhibition(e0: f64, inhibitor: f64, ki: f64, kinact: f64, t: f64) → f64` | Suicide (mechanism-based) inhibition | `biology::enzyme::mechanisms` |
| `enzyme_cooperativity` | `fn enzyme_cooperativity(substrate: f64, vmax: f64, s05: &[f64], weights: &[f64]) → f64` | Enzyme cooperativity (weighted) | `biology::enzyme::mechanisms` |

#### regulation.rs (10 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `allosteric_monod_wyman_changeux` | `fn allosteric_monod_wyman_changeux(substrate: f64, n: f64, l0: f64, kr: f64, kt: f64) → f64` | MWC allosteric model | `biology::enzyme::regulation` |
| `hill_cooperativity` | `fn hill_cooperativity(substrate: f64, k_half: f64, n: f64) → f64` | Hill cooperativity | `biology::enzyme::regulation` |
| `phosphorylation_switch` | `fn phosphorylation_switch(kinase: f64, phosphatase: f64, km_kin: f64, km_phos: f64, vmax_kin: f64, vmax_phos: f64, total_protein: f64) → f64` | Phosphorylation switch (Goldbeter-Koshland) | `biology::enzyme::regulation` |
| `zymogen_activation` | `fn zymogen_activation(zymogen: f64, activator: f64, k_act: f64) → f64` | Zymogen activation | `biology::enzyme::regulation` |
| `product_inhibition_ordered` | `fn product_inhibition_ordered(substrate: f64, product: f64, vmax_f: f64, km: f64, kp: f64) → f64` | Product inhibition (ordered) | `biology::enzyme::regulation` |
| `isozyme_total_activity` | `fn isozyme_total_activity(activities: &[f64], fractions: &[f64]) → f64` | Isozyme total activity | `biology::enzyme::regulation` |
| `temperature_activation` | `fn temperature_activation(rate_ref: f64, ea: f64, t: f64, t_ref: f64) → f64` | Temperature-dependent activation | `biology::enzyme::regulation` |
| `thermal_denaturation` | `fn thermal_denaturation(activity: f64, k_denat: f64, t: f64) → f64` | Thermal denaturation | `biology::enzyme::regulation` |
| `feedback_inhibition` | `fn feedback_inhibition(product: f64, ki: f64, n: f64, vmax: f64, substrate: f64, km: f64) → f64` | Feedback inhibition | `biology::enzyme::regulation` |
| `cascade_amplification` | `fn cascade_amplification(initial_signal: f64, amplification_factors: &[f64]) → f64` | Cascade amplification | `biology::enzyme::regulation` |

---

### 1️⃣3️⃣ structural/ — Structural Biology — 39 pub fn

#### docking.rs (11 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `lennard_jones_potential` | `fn lennard_jones_potential(r: f64, epsilon: f64, sigma: f64) → f64` | $V_{LJ} = 4\varepsilon\left[\left(\frac{\sigma}{r}\right)^{12} - \left(\frac{\sigma}{r}\right)^6\right]$ | `biology::structural::docking` |
| `coulomb_interaction` | `fn coulomb_interaction(q1: f64, q2: f64, r: f64, dielectric: f64) → f64` | $E = \frac{q_1 q_2}{4\pi\varepsilon r}$ | `biology::structural::docking` |
| `desolvation_energy` | `fn desolvation_energy(buried_area: f64, solvation_parameter: f64) → f64` | Desolvation energy | `biology::structural::docking` |
| `shape_complementarity` | `fn shape_complementarity(interface_area: f64, gap_volume: f64) → f64` | Shape complementarity score | `biology::structural::docking` |
| `binding_free_energy` | `fn binding_free_energy(van_der_waals: f64, electrostatic: f64, desolvation: f64, entropy_penalty: f64) → f64` | $\Delta G_{bind} = \Delta G_{vdW} + \Delta G_{elec} + \Delta G_{desolv} - T\Delta S$ | `biology::structural::docking` |
| `kd_from_delta_g` | `fn kd_from_delta_g(delta_g: f64, temperature: f64) → f64` | $K_d = e^{\Delta G / RT}$ | `biology::structural::docking` |
| `buried_surface_area` | `fn buried_surface_area(asa_complex: f64, asa_receptor: f64, asa_ligand: f64) → f64` | $BSA = ASA_R + ASA_L - ASA_{RL}$ | `biology::structural::docking` |
| `hydrogen_bond_energy` | `fn hydrogen_bond_energy(distance: f64, angle_deg: f64) → f64` | Hydrogen bond energy | `biology::structural::docking` |
| `clash_score` | `fn clash_score(distances: &[f64], vdw_threshold: f64) → f64` | Steric clash score | `biology::structural::docking` |
| `interface_residue_count` | `fn interface_residue_count(distances_to_partner: &[f64], cutoff: f64) → usize` | Interface residue count | `biology::structural::docking` |
| `druggability_score` | `fn druggability_score(pocket_volume: f64, hydrophobicity: f64, enclosure: f64) → f64` | Druggability score | `biology::structural::docking` |

#### geometry.rs (8 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `euclidean_distance_3d` | `fn euclidean_distance_3d(a: &[f64; 3], b: &[f64; 3]) → f64` | $d = \sqrt{\sum_i (a_i - b_i)^2}$ | `biology::structural::geometry` |
| `bond_angle` | `fn bond_angle(a: &[f64; 3], b: &[f64; 3], c: &[f64; 3]) → f64` | Bond angle (3 atoms) | `biology::structural::geometry` |
| `dihedral_angle` | `fn dihedral_angle(a: &[f64; 3], b: &[f64; 3], c: &[f64; 3], d: &[f64; 3]) → f64` | Dihedral angle (4 atoms) | `biology::structural::geometry` |
| `center_of_mass` | `fn center_of_mass(coords: &[[f64; 3]], masses: &[f64]) → [f64; 3]` | $\vec{r}_{cm} = \frac{\sum m_i \vec{r}_i}{\sum m_i}$ | `biology::structural::geometry` |
| `radius_of_gyration` | `fn radius_of_gyration(coords: &[[f64; 3]], masses: &[f64]) → f64` | $R_g = \sqrt{\frac{\sum m_i |\vec{r}_i - \vec{r}_{cm}|^2}{\sum m_i}}$ | `biology::structural::geometry` |
| `solvent_accessible_distance` | `fn solvent_accessible_distance(point: &[f64; 3], surface_points: &[[f64; 3]]) → f64` | Solvent accessible distance | `biology::structural::geometry` |
| `inertia_tensor` | `fn inertia_tensor(coords: &[[f64; 3]], masses: &[f64]) → [[f64; 3]; 3]` | Moment of inertia tensor | `biology::structural::geometry` |
| `planarity` | `fn planarity(coords: &[[f64; 3]]) → f64` | Planarity measure | `biology::structural::geometry` |

#### secondary_structure.rs (10 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `alpha_helix_propensity` | `fn alpha_helix_propensity(residue_propensities: &[f64]) → f64` | α-helix propensity | `biology::structural::secondary_structure` |
| `beta_sheet_propensity` | `fn beta_sheet_propensity(residue_propensities: &[f64]) → f64` | β-sheet propensity | `biology::structural::secondary_structure` |
| `chou_fasman_nucleation` | `fn chou_fasman_nucleation(propensities: &[f64], window: usize, threshold: f64) → Vec<bool>` | Chou-Fasman nucleation prediction | `biology::structural::secondary_structure` |
| `gor_information_value` | `fn gor_information_value(residue_freq_in_structure: f64, residue_freq_overall: f64) → f64` | GOR information value | `biology::structural::secondary_structure` |
| `coiled_coil_probability` | `fn coiled_coil_probability(heptad_score: f64, hydrophobic_moment: f64) → f64` | Coiled-coil probability | `biology::structural::secondary_structure` |
| `disorder_prediction` | `fn disorder_prediction(hydrophobicity: f64, charge: f64, complexity: f64) → f64` | Intrinsic disorder prediction | `biology::structural::secondary_structure` |
| `solvent_accessibility` | `fn solvent_accessibility(residue_asa: f64, max_asa: f64) → f64` | Relative solvent accessibility | `biology::structural::secondary_structure` |
| `ramachandran_energy` | `fn ramachandran_energy(phi: f64, psi: f64) → f64` | Ramachandran energy ($\phi$/$\psi$) | `biology::structural::secondary_structure` |
| `relative_contact_order` | `fn relative_contact_order(contacts: &[(usize, usize)], chain_length: usize) → f64` | Relative contact order | `biology::structural::secondary_structure` |
| `hydrophobic_moment` | `fn hydrophobic_moment(hydrophobicities: &[f64], angle_deg: f64) → f64` | Hydrophobic moment | `biology::structural::secondary_structure` |

#### superposition.rs (10 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `rmsd` | `fn rmsd(coords_a: &[[f64; 3]], coords_b: &[[f64; 3]]) → f64` | $RMSD = \sqrt{\frac{1}{N}\sum_i |\vec{r}_i^A - \vec{r}_i^B|^2}$ | `biology::structural::superposition` |
| `gdt_ts` | `fn gdt_ts(coords_a: &[[f64; 3]], coords_b: &[[f64; 3]], cutoffs: &[f64]) → f64` | Global Distance Test - Total Score | `biology::structural::superposition` |
| `tm_score` | `fn tm_score(coords_a: &[[f64; 3]], coords_b: &[[f64; 3]], l_target: usize) → f64` | TM-score (template modeling) | `biology::structural::superposition` |
| `contact_map` | `fn contact_map(coords: &[[f64; 3]], cutoff: f64) → Vec<(usize, usize)>` | Contact map | `biology::structural::superposition` |
| `rg_from_coords` | `fn rg_from_coords(coords: &[[f64; 3]]) → f64` | Radius of gyration from coordinates | `biology::structural::superposition` |
| `solvent_accessible_surface_approx` | `fn solvent_accessible_surface_approx(radii: &[f64], probe: f64) → f64` | Solvent accessible surface approximation | `biology::structural::superposition` |
| `lrmsd` | `fn lrmsd(coords_a: &[[f64; 3]], coords_b: &[[f64; 3]], residue_indices: &[usize]) → f64` | Local RMSD (ligand) | `biology::structural::superposition` |
| `drmsd` | `fn drmsd(coords_a: &[[f64; 3]], coords_b: &[[f64; 3]]) → f64` | Distance RMSD | `biology::structural::superposition` |
| `absolute_contact_order` | `fn absolute_contact_order(contacts: &[(usize, usize)], n_residues: usize) → f64` | Absolute contact order | `biology::structural::superposition` |
| `b_factor_normalize` | `fn b_factor_normalize(b_factors: &[f64]) → Vec<f64>` | B-factor normalization | `biology::structural::superposition` |

---

### 1️⃣4️⃣ systems_biology/ — Systems Biology — 47 pub fn

#### bistability.rs (10 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `toggle_switch` | `fn toggle_switch(u: f64, v: f64, alpha1: f64, alpha2: f64, beta: f64, gamma: f64, n: f64) → (f64, f64)` | Genetic toggle switch | `biology::systems_biology::bistability` |
| `bistable_toggle_switch` | `fn bistable_toggle_switch(u0: f64, v0: f64, alpha1: f64, alpha2: f64, beta: f64, gamma: f64, n: f64, dt: f64, steps: usize) → Vec<(f64, f64)>` | Toggle switch simulation | `biology::systems_biology::bistability` |
| `ultrasensitivity_index` | `fn ultrasensitivity_index(ec10: f64, ec90: f64) → f64` | $n_H = \frac{\log 81}{\log(EC_{90}/EC_{10})}$ | `biology::systems_biology::bistability` |
| `network_robustness` | `fn network_robustness(output_nominal: f64, output_perturbed: f64, perturbation_fraction: f64) → f64` | Network robustness coefficient | `biology::systems_biology::bistability` |
| `adaptation_precision` | `fn adaptation_precision(response_peak: f64, response_steady: f64, response_basal: f64) → f64` | Adaptation precision | `biology::systems_biology::bistability` |
| `bifurcation_parameter_scan` | `fn bifurcation_parameter_scan(f: fn(f64, f64) -> f64, x0: f64, param_range: &[f64], dt: f64, settle: usize) → Vec<(f64, f64)>` | Bifurcation parameter scan | `biology::systems_biology::bistability` |
| `hysteresis_width` | `fn hysteresis_width(forward_thresholds: &[f64], backward_thresholds: &[f64]) → f64` | Hysteresis width | `biology::systems_biology::bistability` |
| `nullcline_intersection` | `fn nullcline_intersection(f: fn(f64, f64) -> f64, g: fn(f64, f64) -> f64, x_range: (f64, f64), y_range: (f64, f64), resolution: usize) → Vec<(f64, f64)>` | Nullcline intersection | `biology::systems_biology::bistability` |
| `saddle_node_condition` | `fn saddle_node_condition(jacobian: &[[f64; 2]; 2]) → f64` | Saddle-node bifurcation condition | `biology::systems_biology::bistability` |
| `potential_landscape_1d` | `fn potential_landscape_1d(f: fn(f64) -> f64, x_range: (f64, f64), n_points: usize) → Vec<(f64, f64)>` | 1D potential landscape | `biology::systems_biology::bistability` |

#### flux.rs (10 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `stoichiometry_flux` | `fn stoichiometry_flux(stoichiometry: &[Vec<f64>], fluxes: &[f64]) → Vec<f64>` | $\frac{d\mathbf{x}}{dt} = S \cdot \mathbf{v}$ | `biology::systems_biology::flux` |
| `fba_objective` | `fn fba_objective(c: &[f64], fluxes: &[f64]) → f64` | FBA objective $Z = \mathbf{c}^T \mathbf{v}$ | `biology::systems_biology::flux` |
| `flux_balance_feasibility` | `fn flux_balance_feasibility(stoichiometry: &[Vec<f64>], fluxes: &[f64], tolerance: f64) → bool` | Flux balance feasibility check | `biology::systems_biology::flux` |
| `flux_variability` | `fn flux_variability(flux_nominal: f64, objective_fraction: f64, objective_optimal: f64, c_i: f64) → (f64, f64)` | Flux variability analysis | `biology::systems_biology::flux` |
| `metabolic_sensitivity` | `fn metabolic_sensitivity(flux: f64, parameter: f64, delta_flux: f64, delta_param: f64) → f64` | Metabolic sensitivity | `biology::systems_biology::flux` |
| `dead_end_metabolites` | `fn dead_end_metabolites(stoichiometry: &[Vec<f64>]) → Vec<usize>` | Dead-end metabolite detection | `biology::systems_biology::flux` |
| `elementary_flux_mode_check` | `fn elementary_flux_mode_check(stoichiometry: &[Vec<f64>], mode: &[f64], tolerance: f64) → bool` | Elementary flux mode check | `biology::systems_biology::flux` |
| `yield_coefficient` | `fn yield_coefficient(product_flux: f64, substrate_flux: f64) → f64` | $Y = \frac{v_P}{v_S}$ | `biology::systems_biology::flux` |
| `atp_yield` | `fn atp_yield(atp_production_flux: f64, glucose_uptake_flux: f64) → f64` | ATP yield per glucose | `biology::systems_biology::flux` |
| `shadow_price` | `fn shadow_price(objective_change: f64, constraint_change: f64) → f64` | Shadow price (dual variable) | `biology::systems_biology::flux` |

#### networks.rs (10 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `boolean_network_step` | `fn boolean_network_step(state: &[bool], rules: &[Vec<(usize, bool)>]) → Vec<bool>` | Boolean network step | `biology::systems_biology::networks` |
| `boolean_network_simulate` | `fn boolean_network_simulate(initial: &[bool], rules: &[Vec<(usize, bool)>], steps: usize) → Vec<Vec<bool>>` | Boolean network simulation | `biology::systems_biology::networks` |
| `gene_regulatory_hill` | `fn gene_regulatory_hill(activators: &[(f64, f64, f64)], repressors: &[(f64, f64, f64)], basal: f64) → f64` | Gene regulatory Hill function | `biology::systems_biology::networks` |
| `repressilator` | `fn repressilator(m: &[f64; 3], p: &[f64; 3], alpha: f64, alpha0: f64, beta: f64, n: f64) → ([f64; 3], [f64; 3])` | Repressilator ODE | `biology::systems_biology::networks` |
| `repressilator_simulate` | `fn repressilator_simulate(m0: [f64; 3], p0: [f64; 3], alpha: f64, alpha0: f64, beta: f64, n: f64, dt: f64, steps: usize) → Vec<([f64; 3], [f64; 3])>` | Repressilator simulation | `biology::systems_biology::networks` |
| `genetic_toggle_switch` | `fn genetic_toggle_switch(u: f64, v: f64, alpha1: f64, alpha2: f64, beta: f64, gamma: f64) → (f64, f64)` | Genetic toggle switch ODE | `biology::systems_biology::networks` |
| `toggle_switch_simulate` | `fn toggle_switch_simulate(u0: f64, v0: f64, alpha1: f64, alpha2: f64, beta: f64, gamma: f64, dt: f64, steps: usize) → Vec<(f64, f64)>` | Toggle switch simulation | `biology::systems_biology::networks` |
| `feed_forward_loop` | `fn feed_forward_loop(input: f64, x: f64, k_xy: f64, k_iy: f64, n: f64, coherent: bool) → f64` | Feed-forward loop | `biology::systems_biology::networks` |
| `sensitivity_analysis` | `fn sensitivity_analysis(f: impl Fn(f64) -> f64, param: f64, delta: f64) → f64` | Sensitivity analysis | `biology::systems_biology::networks` |
| `robustness_coefficient` | `fn robustness_coefficient(nominal_output: f64, perturbed_outputs: &[f64]) → f64` | Robustness coefficient | `biology::systems_biology::networks` |

#### oscillations.rs (7 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `goodwin_oscillator_step` | `fn goodwin_oscillator_step(mrna: f64, protein: f64, repressor: f64, dt: f64, k1: f64, k2: f64, k3: f64, d1: f64, d2: f64, d3: f64, n: f64, ki: f64) → (f64, f64, f64)` | Goodwin oscillator step | `biology::systems_biology::oscillations` |
| `repressilator_step` | `fn repressilator_step(x: &[f64; 3], dt: f64, alpha: f64, alpha0: f64, beta: f64, n: f64) → [f64; 3]` | Repressilator step | `biology::systems_biology::oscillations` |
| `oscillation_period` | `fn oscillation_period(time_series: &[f64], dt: f64) → f64` | Oscillation period detection | `biology::systems_biology::oscillations` |
| `oscillation_amplitude` | `fn oscillation_amplitude(time_series: &[f64]) → f64` | Oscillation amplitude | `biology::systems_biology::oscillations` |
| `phase_plane_nullcline_x` | `fn phase_plane_nullcline_x(y: f64, params: (f64, f64, f64)) → f64` | Phase plane nullcline (x) | `biology::systems_biology::oscillations` |
| `fitzhugh_nagumo_step` | `fn fitzhugh_nagumo_step(v: f64, w: f64, dt: f64, i_ext: f64, a: f64, b: f64, tau: f64) → (f64, f64)` | FitzHugh-Nagumo step | `biology::systems_biology::oscillations` |
| `entrainment_arnold_tongue` | `fn entrainment_arnold_tongue(coupling_strength: f64, frequency_mismatch: f64) → bool` | Arnold tongue entrainment | `biology::systems_biology::oscillations` |

#### sensitivity.rs (10 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `local_sensitivity` | `fn local_sensitivity(output_perturbed: f64, output_base: f64, param_perturbed: f64, param_base: f64) → f64` | Local sensitivity $S = \frac{\partial y / y}{\partial p / p}$ | `biology::systems_biology::sensitivity` |
| `morris_elementary_effect` | `fn morris_elementary_effect(output_high: f64, output_base: f64, delta: f64) → f64` | Morris elementary effect | `biology::systems_biology::sensitivity` |
| `sobol_first_order` | `fn sobol_first_order(variance_conditional: f64, variance_total: f64) → f64` | Sobol first-order index $S_i = \frac{V_i}{V}$ | `biology::systems_biology::sensitivity` |
| `sobol_total_order` | `fn sobol_total_order(variance_remaining: f64, variance_total: f64) → f64` | Sobol total-order index | `biology::systems_biology::sensitivity` |
| `prcc_partial_rank_correlation` | `fn prcc_partial_rank_correlation(r_xy_given_z: f64) → f64` | Partial rank correlation coefficient | `biology::systems_biology::sensitivity` |
| `parameter_identifiability` | `fn parameter_identifiability(fisher_information_diagonal: f64) → f64` | Parameter identifiability | `biology::systems_biology::sensitivity` |
| `robustness_index` | `fn robustness_index(outputs_perturbed: &[f64], output_nominal: f64, perturbation_range: f64) → f64` | Robustness index | `biology::systems_biology::sensitivity` |
| `bifurcation_distance` | `fn bifurcation_distance(parameter: f64, critical_value: f64) → f64` | Distance to bifurcation | `biology::systems_biology::sensitivity` |
| `latin_hypercube_sample` | `fn latin_hypercube_sample(n_samples: usize, n_params: usize) → Vec<Vec<f64>>` | Latin hypercube sampling | `biology::systems_biology::sensitivity` |
| `metabolic_control_coefficient` | `fn metabolic_control_coefficient(flux_change: f64, flux_base: f64, enzyme_change: f64, enzyme_base: f64) → f64` | Metabolic control coefficient | `biology::systems_biology::sensitivity` |

---

### 1️⃣5️⃣ synthetic_biology/ — Synthetic Biology — 58 pub fn

#### circuits.rs (21 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `toggle_switch` | `fn toggle_switch(u: f64, v: f64, alpha1: f64, alpha2: f64, beta: f64, gamma: f64, n: f64) → (f64, f64)` | Genetic toggle switch | `biology::synthetic_biology::circuits` |
| `repressilator` | `fn repressilator(x: &[f64; 3], alpha: f64, alpha0: f64, beta: f64, n: f64) → [f64; 3]` | Repressilator circuit | `biology::synthetic_biology::circuits` |
| `hill_activation` | `fn hill_activation(x: f64, k: f64, n: f64) → f64` | Hill activation $\frac{x^n}{K^n + x^n}$ | `biology::synthetic_biology::circuits` |
| `hill_repression` | `fn hill_repression(x: f64, k: f64, n: f64) → f64` | Hill repression $\frac{K^n}{K^n + x^n}$ | `biology::synthetic_biology::circuits` |
| `inducible_promoter` | `fn inducible_promoter(inducer: f64, kd: f64, n: f64, basal: f64, max_rate: f64) → f64` | Inducible promoter | `biology::synthetic_biology::circuits` |
| `ribosome_binding_strength` | `fn ribosome_binding_strength(rbs_score: f64, max_translation: f64) → f64` | Ribosome binding site strength | `biology::synthetic_biology::circuits` |
| `and_gate` | `fn and_gate(input_a: f64, input_b: f64, k_a: f64, k_b: f64, n: f64, max_output: f64) → f64` | AND logic gate | `biology::synthetic_biology::circuits` |
| `or_gate` | `fn or_gate(input_a: f64, input_b: f64, k: f64, n: f64, max_output: f64) → f64` | OR logic gate | `biology::synthetic_biology::circuits` |
| `not_gate` | `fn not_gate(input: f64, k: f64, n: f64, max_output: f64) → f64` | NOT logic gate (inverter) | `biology::synthetic_biology::circuits` |
| `oscillator_amplitude` | `fn oscillator_amplitude(protein_levels: &[f64]) → f64` | Oscillator amplitude | `biology::synthetic_biology::circuits` |
| `nand_gate` | `fn nand_gate(input_a: f64, input_b: f64, k: f64, n: f64, max_output: f64) → f64` | NAND logic gate | `biology::synthetic_biology::circuits` |
| `xor_gate` | `fn xor_gate(input_a: f64, input_b: f64, k: f64, n: f64, max_output: f64) → f64` | XOR logic gate | `biology::synthetic_biology::circuits` |
| `feed_forward_loop_c1` | `fn feed_forward_loop_c1(input: f64, x: f64, k_input: f64, k_x: f64, n: f64, max_rate: f64) → f64` | Coherent feed-forward loop (C1) | `biology::synthetic_biology::circuits` |
| `negative_autoregulation` | `fn negative_autoregulation(protein: f64, k: f64, n: f64, basal: f64, max_rate: f64) → f64` | Negative autoregulation | `biology::synthetic_biology::circuits` |
| `positive_autoregulation` | `fn positive_autoregulation(protein: f64, k: f64, n: f64, basal: f64, max_rate: f64) → f64` | Positive autoregulation | `biology::synthetic_biology::circuits` |
| `bandpass_filter` | `fn bandpass_filter(input: f64, k_low: f64, k_high: f64, n: f64, max_output: f64) → f64` | Bandpass filter circuit | `biology::synthetic_biology::circuits` |
| `quorum_sensing_autoinducer` | `fn quorum_sensing_autoinducer(cell_density: f64, production_rate: f64, degradation_rate: f64) → f64` | Quorum sensing autoinducer | `biology::synthetic_biology::circuits` |
| `quorum_sensing_response` | `fn quorum_sensing_response(autoinducer: f64, threshold: f64, n: f64, max_response: f64) → f64` | Quorum sensing response | `biology::synthetic_biology::circuits` |
| `kill_switch_toxin_antitoxin` | `fn kill_switch_toxin_antitoxin(inducer: f64, k_toxin: f64, k_antitoxin: f64, n: f64) → f64` | Kill switch (toxin-antitoxin) | `biology::synthetic_biology::circuits` |
| `metabolic_burden` | `fn metabolic_burden(circuit_protein: f64, growth_rate_max: f64, burden_coeff: f64) → f64` | Metabolic burden | `biology::synthetic_biology::circuits` |
| `biosensor_dose_response` | `fn biosensor_dose_response(analyte: f64, k_half: f64, n: f64, output_min: f64, output_max: f64) → f64` | Biosensor dose-response | `biology::synthetic_biology::circuits` |

#### crispr.rs (10 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `crispr_on_target_score` | `fn crispr_on_target_score(gc_content: f64, position_scores: &[f64]) → f64` | CRISPR on-target score | `biology::synthetic_biology::crispr` |
| `crispr_off_target_cfd` | `fn crispr_off_target_cfd(mismatches: &[(usize, f64)]) → f64` | CRISPR off-target CFD score | `biology::synthetic_biology::crispr` |
| `cas9_cleavage_efficiency` | `fn cas9_cleavage_efficiency(grna_binding: f64, pam_strength: f64, chromatin_accessibility: f64) → f64` | Cas9 cleavage efficiency | `biology::synthetic_biology::crispr` |
| `hdr_efficiency` | `fn hdr_efficiency(template_conc: f64, homology_arm_length: f64, cell_cycle_s_fraction: f64) → f64` | HDR efficiency | `biology::synthetic_biology::crispr` |
| `nhej_indel_spectrum` | `fn nhej_indel_spectrum(cut_position: f64, microhomology_scores: &[f64]) → Vec<f64>` | NHEJ indel spectrum | `biology::synthetic_biology::crispr` |
| `base_editing_efficiency` | `fn base_editing_efficiency(c_to_t_window: bool, accessibility: f64, deaminase_activity: f64) → f64` | Base editing efficiency | `biology::synthetic_biology::crispr` |
| `prime_editing_efficiency` | `fn prime_editing_efficiency(pbs_length: usize, rt_template_length: usize, nick_distance: usize) → f64` | Prime editing efficiency | `biology::synthetic_biology::crispr` |
| `guide_rna_folding_penalty` | `fn guide_rna_folding_penalty(self_complementarity_score: f64) → f64` | Guide RNA folding penalty | `biology::synthetic_biology::crispr` |
| `multiplex_editing_probability` | `fn multiplex_editing_probability(efficiencies: &[f64]) → f64` | Multiplex editing probability | `biology::synthetic_biology::crispr` |
| `gene_drive_frequency` | `fn gene_drive_frequency(drive_efficiency: f64, fitness_cost: f64, generations: usize, initial_freq: f64) → f64` | Gene drive frequency evolution | `biology::synthetic_biology::crispr` |

#### metabolic_engineering.rs (9 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `flux_balance_objective` | `fn flux_balance_objective(fluxes: &[f64], objective_coefficients: &[f64]) → f64` | FBA objective function | `biology::synthetic_biology::metabolic_engineering` |
| `theoretical_yield` | `fn theoretical_yield(substrate_carbons: usize, product_carbons: usize, pathway_efficiency: f64) → f64` | Theoretical yield | `biology::synthetic_biology::metabolic_engineering` |
| `heterologous_metabolic_burden` | `fn heterologous_metabolic_burden(heterologous_protein_fraction: f64, growth_rate_wt: f64) → f64` | Heterologous metabolic burden | `biology::synthetic_biology::metabolic_engineering` |
| `pathway_thermodynamic_driving_force` | `fn pathway_thermodynamic_driving_force(delta_g_steps: &[f64]) → f64` | Max-min driving force (MDF) | `biology::synthetic_biology::metabolic_engineering` |
| `enzyme_cost_minimization` | `fn enzyme_cost_minimization(fluxes: &[f64], kcats: &[f64]) → f64` | Enzyme cost minimization | `biology::synthetic_biology::metabolic_engineering` |
| `cofactor_balance` | `fn cofactor_balance(nadh_produced: f64, nadh_consumed: f64) → f64` | Cofactor (NADH) balance | `biology::synthetic_biology::metabolic_engineering` |
| `titer_rate_yield` | `fn titer_rate_yield(titer: f64, time: f64, substrate_consumed: f64) → (f64, f64)` | Titer-rate-yield (TRY) | `biology::synthetic_biology::metabolic_engineering` |
| `gene_expression_burden` | `fn gene_expression_burden(promoter_strength: f64, copy_number: f64, protein_size: f64) → f64` | Gene expression burden | `biology::synthetic_biology::metabolic_engineering` |
| `heterologous_pathway_flux` | `fn heterologous_pathway_flux(enzyme_levels: &[f64], kms: &[f64], substrates: &[f64]) → f64` | Heterologous pathway flux | `biology::synthetic_biology::metabolic_engineering` |

#### stochastic.rs (18 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `stochastic_gene_expression_mean` | `fn stochastic_gene_expression_mean(transcription_rate: f64, translation_rate: f64, mrna_decay: f64, protein_decay: f64) → f64` | $\langle P \rangle = \frac{k_{tx} \cdot k_{tl}}{\gamma_m \cdot \gamma_p}$ | `biology::synthetic_biology::stochastic` |
| `stochastic_gene_expression_variance` | `fn stochastic_gene_expression_variance(transcription_rate: f64, translation_rate: f64, mrna_decay: f64, protein_decay: f64) → f64` | Gene expression variance | `biology::synthetic_biology::stochastic` |
| `fano_factor` | `fn fano_factor(variance: f64, mean: f64) → f64` | $F = \frac{\sigma^2}{\mu}$ | `biology::synthetic_biology::stochastic` |
| `noise_intrinsic` | `fn noise_intrinsic(burst_size: f64, mean_protein: f64) → f64` | Intrinsic noise | `biology::synthetic_biology::stochastic` |
| `noise_extrinsic` | `fn noise_extrinsic(cv_parameter: f64) → f64` | Extrinsic noise | `biology::synthetic_biology::stochastic` |
| `total_noise` | `fn total_noise(intrinsic: f64, extrinsic: f64) → f64` | $\eta^2_{total} = \eta^2_{int} + \eta^2_{ext}$ | `biology::synthetic_biology::stochastic` |
| `gillespie_propensity_birth` | `fn gillespie_propensity_birth(rate: f64) → f64` | Gillespie birth propensity | `biology::synthetic_biology::stochastic` |
| `gillespie_propensity_death` | `fn gillespie_propensity_death(rate: f64, population: f64) → f64` | Gillespie death propensity | `biology::synthetic_biology::stochastic` |
| `gillespie_tau` | `fn gillespie_tau(total_propensity: f64, random_uniform: f64) → f64` | $\tau = \frac{-\ln(r)}{a_0}$ | `biology::synthetic_biology::stochastic` |
| `burst_frequency` | `fn burst_frequency(transcription_rate: f64, promoter_off_rate: f64) → f64` | Transcriptional burst frequency | `biology::synthetic_biology::stochastic` |
| `burst_size_mean` | `fn burst_size_mean(translation_rate: f64, mrna_decay: f64) → f64` | $b = \frac{k_{tl}}{\gamma_m}$ | `biology::synthetic_biology::stochastic` |
| `coefficient_of_variation_squared` | `fn coefficient_of_variation_squared(mean: f64, variance: f64) → f64` | $CV^2 = \frac{\sigma^2}{\mu^2}$ | `biology::synthetic_biology::stochastic` |
| `two_state_promoter_mean` | `fn two_state_promoter_mean(k_on: f64, k_off: f64, transcription: f64, mrna_decay: f64) → f64` | Two-state promoter mean | `biology::synthetic_biology::stochastic` |
| `two_state_promoter_fano` | `fn two_state_promoter_fano(k_on: f64, k_off: f64, transcription: f64, mrna_decay: f64) → f64` | Two-state promoter Fano factor | `biology::synthetic_biology::stochastic` |
| `langevin_approximation_step` | `fn langevin_approximation_step(x: f64, drift: f64, diffusion: f64, dt: f64, noise: f64) → f64` | Langevin approximation step | `biology::synthetic_biology::stochastic` |
| `chemical_master_equation_steady_state_poisson` | `fn chemical_master_equation_steady_state_poisson(production: f64, degradation: f64) → f64` | CME Poisson steady state | `biology::synthetic_biology::stochastic` |
| `binomial_partitioning_noise` | `fn binomial_partitioning_noise(n: f64, p: f64) → f64` | Binomial partitioning noise | `biology::synthetic_biology::stochastic` |
| `gene_expression_delay_gamma` | `fn gene_expression_delay_gamma(mean_delay: f64, shape: u32, t: f64) → f64` | Gamma-distributed expression delay | `biology::synthetic_biology::stochastic` |

---

### 1️⃣6️⃣ developmental/ — Developmental Biology — 35 pub fn

#### differentiation.rs (7 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `waddington_landscape` | `fn waddington_landscape(x: f64, param: f64) → f64` | Waddington epigenetic landscape | `biology::developmental::differentiation` |
| `differentiation_potential` | `fn differentiation_potential(x: f64, param: f64) → f64` | Differentiation potential | `biology::developmental::differentiation` |
| `differentiation_simulate` | `fn differentiation_simulate(x0: f64, param_start: f64, param_end: f64, noise: f64, dt: f64, steps: usize) → Vec<(f64, f64)>` | Differentiation simulation | `biology::developmental::differentiation` |
| `somitogenesis_clock` | `fn somitogenesis_clock(phase: f64, omega: f64, coupling: f64, neighbor_phase: f64) → f64` | Somitogenesis clock oscillator | `biology::developmental::differentiation` |
| `somite_clock_simulate` | `fn somite_clock_simulate(phases: &mut [f64], omega: f64, coupling: f64, dt: f64, steps: usize) → Vec<Vec<f64>>` | Somite clock simulation | `biology::developmental::differentiation` |
| `lateral_inhibition_step` | `fn lateral_inhibition_step(signal: f64, neighbor_signal: f64, delta: f64, notch: f64, k: f64) → (f64, f64)` | Lateral inhibition (Delta-Notch) | `biology::developmental::differentiation` |
| `cell_fate_probability` | `fn cell_fate_probability(signal: f64, threshold: f64, steepness: f64) → f64` | Cell fate probability | `biology::developmental::differentiation` |

#### gene_regulation.rs (9 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `gene_regulatory_network_step` | `fn gene_regulatory_network_step(expression: &mut [f64], interactions: &[Vec<f64>], basal_rates: &[f64], degradation: &[f64], dt: f64)` | Gene regulatory network ODE step | `biology::developmental::gene_regulation` |
| `toggle_switch` | `fn toggle_switch(a: f64, b: f64, alpha: f64, beta: f64, n: f64) → (f64, f64)` | Toggle switch (bistable) | `biology::developmental::gene_regulation` |
| `apical_basal_polarity` | `fn apical_basal_polarity(par3: f64, par6: f64, atypical_pkc: f64, par1: f64) → f64` | Apical-basal polarity (PAR) | `biology::developmental::gene_regulation` |
| `planar_cell_polarity` | `fn planar_cell_polarity(frizzled: f64, vang: f64, coupling: f64, neighbor_fz: f64, neighbor_vang: f64) → (f64, f64)` | Planar cell polarity (Fz/Vang) | `biology::developmental::gene_regulation` |
| `notch_delta_lateral_inhibition_ode` | `fn notch_delta_lateral_inhibition_ode(notch: f64, delta: f64, neighbor_delta: f64, beta_n: f64, beta_d: f64, k: f64, n: f64) → (f64, f64)` | Notch-Delta lateral inhibition ODE | `biology::developmental::gene_regulation` |
| `induction_competence` | `fn induction_competence(signal: f64, competence_window: f64, time: f64, window_center: f64) → f64` | Induction competence window | `biology::developmental::gene_regulation` |
| `reaction_diffusion_activator_inhibitor` | `fn reaction_diffusion_activator_inhibitor(a: f64, h: f64, da: f64, rho_a: f64, mu_a: f64, dh: f64, rho_h: f64, mu_h: f64, laplacian_a: f64, laplacian_h: f64) → (f64, f64)` | Activator-inhibitor reaction-diffusion | `biology::developmental::gene_regulation` |
| `hox_gene_expression` | `fn hox_gene_expression(position: f64, boundaries: &[(f64, f64)]) → Vec<bool>` | Hox gene expression pattern | `biology::developmental::gene_regulation` |
| `morphogenetic_field_potential` | `fn morphogenetic_field_potential(cell_position: (f64, f64), field_center: (f64, f64), field_strength: f64, decay: f64) → f64` | Morphogenetic field potential | `biology::developmental::gene_regulation` |

#### morphogens.rs (10 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `morphogen_gradient_steady` | `fn morphogen_gradient_steady(source: f64, decay: f64, diffusion: f64, x: f64) → f64` | $C(x) = C_0 e^{-x/\lambda}$ ($\lambda = \sqrt{D/k}$) | `biology::developmental::morphogens` |
| `morphogen_diffusion_1d` | `fn morphogen_diffusion_1d(conc: &mut [f64], d: f64, decay: f64, source_idx: usize, source_rate: f64, dx: f64, dt: f64, steps: usize)` | 1D morphogen diffusion simulation | `biology::developmental::morphogens` |
| `french_flag_model` | `fn french_flag_model(concentration: f64, t1: f64, t2: f64) → u8` | French flag model (Wolpert) | `biology::developmental::morphogens` |
| `bicoid_gradient` | `fn bicoid_gradient(c0: f64, length: f64, lambda: f64, x: f64) → f64` | Bicoid gradient (Drosophila) | `biology::developmental::morphogens` |
| `positional_information` | `fn positional_information(thresholds: &[f64], gradient: &[f64]) → Vec<u8>` | Positional information | `biology::developmental::morphogens` |
| `morphogen_gradient_reaction` | `fn morphogen_gradient_reaction(conc: &mut [f64], production: &[f64], degradation: f64, diffusion: f64, dx: f64, dt: f64, steps: usize)` | Morphogen gradient with reaction | `biology::developmental::morphogens` |
| `shh_patterning` | `fn shh_patterning(distance: f64, concentration: f64, thresholds: &[(f64, u8)]) → u8` | Sonic hedgehog patterning | `biology::developmental::morphogens` |
| `morphogen_noise_filtering` | `fn morphogen_noise_filtering(signal: &[f64], window: usize) → Vec<f64>` | Morphogen noise filtering | `biology::developmental::morphogens` |
| `interpretation_delay` | `fn interpretation_delay(concentration_history: &[f64], delay: usize) → f64` | Morphogen interpretation delay | `biology::developmental::morphogens` |
| `wnt_gradient` | `fn wnt_gradient(source_strength: f64, decay_rate: f64, positions: &[f64]) → Vec<f64>` | Wnt gradient | `biology::developmental::morphogens` |

#### patterns.rs (9 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `turing_reaction_diffusion` | `fn turing_reaction_diffusion(u: &mut [f64], v: &mut [f64], du: f64, dv: f64, a: f64, b: f64, dx: f64, dt: f64, steps: usize)` | Turing reaction-diffusion simulation | `biology::developmental::patterns` |
| `turing_instability_condition` | `fn turing_instability_condition(du: f64, dv: f64, fu: f64, gv: f64, fg_det: f64) → bool` | Turing instability condition | `biology::developmental::patterns` |
| `gierer_meinhardt` | `fn gierer_meinhardt(activator: &mut [f64], inhibitor: &mut [f64], da: f64, di: f64, rho_a: f64, rho_i: f64, mu_a: f64, mu_i: f64, dx: f64, dt: f64, steps: usize)` | Gierer-Meinhardt model | `biology::developmental::patterns` |
| `french_flag_positional` | `fn french_flag_positional(position: f64, threshold_high: f64, threshold_low: f64, morphogen_source: f64, decay_length: f64) → u8` | French flag positional information | `biology::developmental::patterns` |
| `lateral_inhibition` | `fn lateral_inhibition(cells: &mut [f64], notch: &mut [f64], delta: &mut [f64], beta_n: f64, beta_d: f64, k: f64, n: f64, dt: f64, steps: usize)` | Lateral inhibition simulation | `biology::developmental::patterns` |
| `clock_and_wavefront` | `fn clock_and_wavefront(position: f64, wavefront_speed: f64, frequency: f64, t: f64, threshold: f64) → bool` | Clock-and-wavefront (somitogenesis) | `biology::developmental::patterns` |
| `schnakenberg` | `fn schnakenberg(u: &mut [f64], v: &mut [f64], du: f64, dv: f64, a: f64, b: f64, gamma: f64, dx: f64, dt: f64, steps: usize)` | Schnakenberg pattern formation | `biology::developmental::patterns` |
| `voronoi_cell_sorting` | `fn voronoi_cell_sorting(positions: &[(f64, f64)], types: &[u8], adhesion_same: f64, adhesion_diff: f64) → Vec<(f64, f64)>` | Voronoi cell sorting | `biology::developmental::patterns` |
| `wave_pinning` | `fn wave_pinning(u: &mut [f64], v: &mut [f64], d: f64, k_on: f64, k_off: f64, k_fb: f64, hill_n: f64, dx: f64, dt: f64, steps: usize)` | Wave pinning model | `biology::developmental::patterns` |

---

### 1️⃣7️⃣ physiology/ — Physiology — 47 pub fn

#### cardiac.rs (11 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `frank_starling_mechanism` | `fn frank_starling_mechanism(end_diastolic_volume: f64, contractility: f64, max_stroke_volume: f64) → f64` | Frank-Starling mechanism | `biology::physiology::cardiac` |
| `stroke_volume_cardiac_output` | `fn stroke_volume_cardiac_output(heart_rate: f64, stroke_volume: f64) → f64` | $CO = HR \times SV$ | `biology::physiology::cardiac` |
| `ejection_fraction` | `fn ejection_fraction(stroke_volume: f64, end_diastolic_volume: f64) → f64` | $EF = \frac{SV}{EDV}$ | `biology::physiology::cardiac` |
| `map_calculation` | `fn map_calculation(systolic: f64, diastolic: f64) → f64` | $MAP = DBP + \frac{1}{3}(SBP - DBP)$ | `biology::physiology::cardiac` |
| `systemic_vascular_resistance` | `fn systemic_vascular_resistance(map: f64, cvp: f64, cardiac_output: f64) → f64` | $SVR = \frac{MAP - CVP}{CO}$ | `biology::physiology::cardiac` |
| `myocardial_oxygen_consumption` | `fn myocardial_oxygen_consumption(heart_rate: f64, systolic_bp: f64) → f64` | Myocardial O₂ consumption (RPP) | `biology::physiology::cardiac` |
| `windkessel_pressure` | `fn windkessel_pressure(cardiac_output: f64, resistance: f64, compliance: f64, t: f64, heart_rate: f64) → f64` | Windkessel pressure model | `biology::physiology::cardiac` |
| `coronary_flow_reserve` | `fn coronary_flow_reserve(hyperemic_flow: f64, resting_flow: f64) → f64` | $CFR = \frac{Q_{hyp}}{Q_{rest}}$ | `biology::physiology::cardiac` |
| `qt_correction_bazett` | `fn qt_correction_bazett(qt_ms: f64, rr_ms: f64) → f64` | $QTc = \frac{QT}{\sqrt{RR}}$ (Bazett) | `biology::physiology::cardiac` |
| `cardiac_work` | `fn cardiac_work(stroke_volume_ml: f64, mean_pressure_mmhg: f64) → f64` | Cardiac stroke work | `biology::physiology::cardiac` |
| `preload_recruitable_stroke_work` | `fn preload_recruitable_stroke_work(stroke_work: f64, edv: f64, v0: f64) → f64` | Preload recruitable stroke work | `biology::physiology::cardiac` |

#### hemodynamics.rs (8 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `poiseuille_flow` | `fn poiseuille_flow(radius: f64, length: f64, pressure_drop: f64, viscosity: f64) → f64` | $Q = \frac{\pi r^4 \Delta P}{8 \mu L}$ | `biology::physiology::hemodynamics` |
| `wall_shear_stress` | `fn wall_shear_stress(viscosity: f64, flow_rate: f64, radius: f64) → f64` | $\tau = \frac{4 \mu Q}{\pi r^3}$ | `biology::physiology::hemodynamics` |
| `mean_arterial_pressure` | `fn mean_arterial_pressure(systolic: f64, diastolic: f64) → f64` | $MAP = DBP + \frac{1}{3}PP$ | `biology::physiology::hemodynamics` |
| `cardiac_output` | `fn cardiac_output(stroke_volume: f64, heart_rate: f64) → f64` | $CO = SV \times HR$ | `biology::physiology::hemodynamics` |
| `total_peripheral_resistance` | `fn total_peripheral_resistance(map: f64, cvp: f64, cardiac_output: f64) → f64` | $TPR = \frac{MAP - CVP}{CO}$ | `biology::physiology::hemodynamics` |
| `frank_starling` | `fn frank_starling(preload: f64, k: f64, max_force: f64) → f64` | Frank-Starling law | `biology::physiology::hemodynamics` |
| `pulse_wave_velocity` | `fn pulse_wave_velocity(elasticity: f64, wall_thickness: f64, radius: f64, density: f64) → f64` | Moens-Korteweg equation | `biology::physiology::hemodynamics` |
| `windkessel_2` | `fn windkessel_2(flow: f64, pressure: f64, resistance: f64, compliance: f64) → f64` | 2-element Windkessel model | `biology::physiology::hemodynamics` |

#### renal.rs (10 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `glomerular_filtration_rate` | `fn glomerular_filtration_rate(kf: f64, p_gc: f64, p_bs: f64, pi_gc: f64) → f64` | $GFR = K_f (P_{GC} - P_{BS} - \pi_{GC})$ | `biology::physiology::renal` |
| `creatinine_clearance` | `fn creatinine_clearance(urine_cr: f64, urine_volume: f64, plasma_cr: f64) → f64` | $C_{Cr} = \frac{U_{Cr} \times V}{P_{Cr}}$ | `biology::physiology::renal` |
| `fractional_excretion` | `fn fractional_excretion(urine_x: f64, plasma_cr: f64, plasma_x: f64, urine_cr: f64) → f64` | $FE = \frac{U_x P_{Cr}}{P_x U_{Cr}}$ | `biology::physiology::renal` |
| `free_water_clearance` | `fn free_water_clearance(urine_volume: f64, urine_osm: f64, plasma_osm: f64) → f64` | Free water clearance | `biology::physiology::renal` |
| `tubular_reabsorption_rate` | `fn tubular_reabsorption_rate(filtered_load: f64, excretion_rate: f64) → f64` | Tubular reabsorption rate | `biology::physiology::renal` |
| `cockcroft_gault` | `fn cockcroft_gault(age: f64, weight: f64, serum_cr: f64, is_female: bool) → f64` | Cockcroft-Gault equation | `biology::physiology::renal` |
| `mdrd_gfr` | `fn mdrd_gfr(serum_cr: f64, age: f64, is_female: bool, is_black: bool) → f64` | MDRD GFR estimation | `biology::physiology::renal` |
| `tubuloglomerular_feedback` | `fn tubuloglomerular_feedback(nacl_macula: f64, nacl_target: f64, sensitivity: f64, gfr_baseline: f64) → f64` | Tubuloglomerular feedback | `biology::physiology::renal` |
| `urine_concentration_ratio` | `fn urine_concentration_ratio(urine_osm: f64, plasma_osm: f64) → f64` | Urine concentration ratio | `biology::physiology::renal` |
| `anion_gap` | `fn anion_gap(sodium: f64, chloride: f64, bicarbonate: f64) → f64` | $AG = Na^+ - (Cl^- + HCO_3^-)$ | `biology::physiology::renal` |

#### respiratory.rs (8 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `tidal_volume` | `fn tidal_volume(respiratory_rate: f64, minute_ventilation: f64) → f64` | $V_T = \frac{\dot{V}_E}{RR}$ | `biology::physiology::respiratory` |
| `alveolar_ventilation` | `fn alveolar_ventilation(tidal_volume: f64, dead_space: f64, rate: f64) → f64` | $\dot{V}_A = (V_T - V_D) \times RR$ | `biology::physiology::respiratory` |
| `alveolar_gas_equation` | `fn alveolar_gas_equation(fio2: f64, p_atm: f64, p_h2o: f64, paco2: f64, rq: f64) → f64` | $P_{AO_2} = FiO_2(P_{atm}-P_{H_2O}) - \frac{P_{aCO_2}}{RQ}$ | `biology::physiology::respiratory` |
| `airway_resistance` | `fn airway_resistance(pressure_drop: f64, flow: f64) → f64` | $R_{aw} = \frac{\Delta P}{\dot{V}}$ | `biology::physiology::respiratory` |
| `lung_compliance` | `fn lung_compliance(volume_change: f64, pressure_change: f64) → f64` | $C = \frac{\Delta V}{\Delta P}$ | `biology::physiology::respiratory` |
| `oxygen_content` | `fn oxygen_content(hb: f64, sao2: f64, pao2: f64) → f64` | $CaO_2 = 1.34 \cdot Hb \cdot SaO_2 + 0.003 \cdot PaO_2$ | `biology::physiology::respiratory` |
| `oxygen_delivery` | `fn oxygen_delivery(cardiac_output: f64, cao2: f64) → f64` | $DO_2 = CO \times CaO_2$ | `biology::physiology::respiratory` |
| `fick_oxygen_consumption` | `fn fick_oxygen_consumption(cardiac_output: f64, cao2: f64, cvo2: f64) → f64` | $\dot{V}O_2 = CO \times (CaO_2 - CvO_2)$ (Fick) | `biology::physiology::respiratory` |

#### thermoregulation.rs (10 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `heat_balance` | `fn heat_balance(metabolic_rate: f64, work: f64, radiation: f64, convection: f64, evaporation: f64) → f64` | $S = M - W \pm R \pm C - E$ | `biology::physiology::thermoregulation` |
| `newton_cooling` | `fn newton_cooling(body_temp: f64, ambient_temp: f64, h: f64, surface_area: f64) → f64` | Newton cooling $Q = hA(T_b - T_a)$ | `biology::physiology::thermoregulation` |
| `evaporative_heat_loss` | `fn evaporative_heat_loss(sweat_rate: f64, latent_heat: f64) → f64` | Evaporative heat loss | `biology::physiology::thermoregulation` |
| `core_temperature_regulation` | `fn core_temperature_regulation(set_point: f64, core_temp: f64, gain_shiver: f64, gain_sweat: f64) → (f64, f64)` | Core temperature regulation | `biology::physiology::thermoregulation` |
| `wind_chill_index` | `fn wind_chill_index(air_temp: f64, wind_speed_kmh: f64) → f64` | Wind chill index | `biology::physiology::thermoregulation` |
| `heat_index` | `fn heat_index(temperature_f: f64, relative_humidity: f64) → f64` | Heat index | `biology::physiology::thermoregulation` |
| `body_surface_area_dubois` | `fn body_surface_area_dubois(weight_kg: f64, height_cm: f64) → f64` | Du Bois BSA formula | `biology::physiology::thermoregulation` |
| `counter_current_heat_exchange` | `fn counter_current_heat_exchange(arterial_temp: f64, venous_temp_return: f64, efficiency: f64) → f64` | Counter-current heat exchange | `biology::physiology::thermoregulation` |
| `brown_adipose_tissue_thermogenesis` | `fn brown_adipose_tissue_thermogenesis(bat_mass: f64, ucp1_activity: f64, substrate_availability: f64) → f64` | Brown adipose tissue thermogenesis (UCP1) | `biology::physiology::thermoregulation` |
| `metabolic_rate_q10` | `fn metabolic_rate_q10(rate_ref: f64, temp: f64, temp_ref: f64, q10: f64) → f64` | $R = R_{ref} \cdot Q_{10}^{(T-T_{ref})/10}$ | `biology::physiology::thermoregulation` |

---

### 1️⃣8️⃣ endocrinology/ — Endocrinology — 60 pub fn

#### axes.rs (10 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `hpa_axis_cortisol` | `fn hpa_axis_cortisol(crh: f64, acth_gain: f64, cortisol_gain: f64, feedback_strength: f64, cortisol_current: f64) → (f64, f64)` | HPA axis (CRH→ACTH→cortisol) | `biology::endocrinology::axes` |
| `hpg_axis_testosterone` | `fn hpg_axis_testosterone(gnrh: f64, lh_gain: f64, testosterone_gain: f64, feedback: f64, testosterone_current: f64) → (f64, f64)` | HPG axis (GnRH→LH→testosterone) | `biology::endocrinology::axes` |
| `hpt_axis_t4` | `fn hpt_axis_t4(trh: f64, tsh_gain: f64, t4_gain: f64, feedback: f64, t4_current: f64) → (f64, f64)` | HPT axis (TRH→TSH→T4) | `biology::endocrinology::axes` |
| `glucose_insulin_model_step` | `fn glucose_insulin_model_step(glucose: f64, insulin: f64, glucose_input: f64, dt: f64, si: f64, sg: f64, n: f64, gamma: f64, g_threshold: f64) → (f64, f64)` | Glucose-insulin minimal model | `biology::endocrinology::axes` |
| `calcium_pth_feedback` | `fn calcium_pth_feedback(calcium: f64, setpoint: f64, pth_max: f64, steepness: f64) → f64` | Calcium-PTH feedback | `biology::endocrinology::axes` |
| `raas_angiotensin` | `fn raas_angiotensin(renin: f64, angiotensinogen: f64, ace_activity: f64) → f64` | RAAS angiotensin II production | `biology::endocrinology::axes` |
| `aldosterone_response` | `fn aldosterone_response(angiotensin_ii: f64, potassium: f64, gain_ang: f64, gain_k: f64) → f64` | Aldosterone response | `biology::endocrinology::axes` |
| `growth_hormone_igf1` | `fn growth_hormone_igf1(gh: f64, liver_response: f64, feedback: f64, igf1_current: f64) → f64` | GH → IGF-1 axis | `biology::endocrinology::axes` |
| `leptin_energy_feedback` | `fn leptin_energy_feedback(fat_mass: f64, leptin_sensitivity: f64, energy_expenditure_base: f64) → f64` | Leptin energy feedback | `biology::endocrinology::axes` |
| `cortisol_awakening_response` | `fn cortisol_awakening_response(basal_cortisol: f64, car_amplitude: f64, time_after_wake_min: f64) → f64` | Cortisol awakening response (CAR) | `biology::endocrinology::axes` |

#### hormones.rs (13 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `hormone_synthesis_rate` | `fn hormone_synthesis_rate(enzyme_conc: f64, substrate: f64, km: f64, vmax: f64) → f64` | Hormone synthesis rate (Michaelis-Menten) | `biology::endocrinology::hormones` |
| `hormone_half_life_clearance` | `fn hormone_half_life_clearance(concentration: f64, half_life: f64, t: f64) → f64` | $C(t) = C_0 e^{-\frac{\ln 2}{t_{1/2}} t}$ | `biology::endocrinology::hormones` |
| `pulsatile_release` | `fn pulsatile_release(amplitude: f64, frequency: f64, t: f64, basal: f64) → f64` | Pulsatile hormone release | `biology::endocrinology::hormones` |
| `negative_feedback_loop` | `fn negative_feedback_loop(setpoint: f64, current: f64, gain: f64) → f64` | Negative feedback loop | `biology::endocrinology::hormones` |
| `positive_feedback_loop` | `fn positive_feedback_loop(stimulus: f64, hormone_level: f64, gain: f64, threshold: f64) → f64` | Positive feedback loop | `biology::endocrinology::hormones` |
| `receptor_saturation` | `fn receptor_saturation(hormone: f64, kd: f64, receptor_total: f64) → f64` | $B = \frac{R_T [H]}{K_d + [H]}$ | `biology::endocrinology::hormones` |
| `hormone_free_fraction` | `fn hormone_free_fraction(total: f64, binding_protein: f64, kd: f64) → f64` | Free hormone fraction | `biology::endocrinology::hormones` |
| `circadian_hormone_profile` | `fn circadian_hormone_profile(amplitude: f64, phase: f64, t_hours: f64, mesor: f64) → f64` | $H(t) = M + A\cos(2\pi t/24 - \phi)$ | `biology::endocrinology::hormones` |
| `steroidogenesis_rate` | `fn steroidogenesis_rate(cholesterol: f64, star_protein: f64, enzyme_activity: f64) → f64` | Steroidogenesis rate (StAR) | `biology::endocrinology::hormones` |
| `thyroid_hormone_conversion` | `fn thyroid_hormone_conversion(t4: f64, deiodinase_activity: f64, km: f64) → f64` | T4 → T3 conversion (deiodinase) | `biology::endocrinology::hormones` |
| `insulin_sensitivity_index` | `fn insulin_sensitivity_index(glucose: f64, insulin: f64) → f64` | Insulin sensitivity index | `biology::endocrinology::hormones` |
| `homa_ir` | `fn homa_ir(fasting_glucose_mmol: f64, fasting_insulin_mu_per_ml: f64) → f64` | $HOMA\text{-}IR = \frac{G_0 \times I_0}{22.5}$ | `biology::endocrinology::hormones` |
| `homa_beta` | `fn homa_beta(fasting_insulin_mu_per_ml: f64, fasting_glucose_mmol: f64) → f64` | HOMA-β (β-cell function) | `biology::endocrinology::hormones` |

#### kinetics.rs (18 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `hormone_clearance` | `fn hormone_clearance(c0: f64, half_life: f64, t: f64) → f64` | Hormone clearance (first-order) | `biology::endocrinology::kinetics` |
| `hormone_infusion_steady_state` | `fn hormone_infusion_steady_state(infusion_rate: f64, clearance_rate: f64) → f64` | $C_{ss} = \frac{R_{inf}}{CL}$ | `biology::endocrinology::kinetics` |
| `hormone_infusion_transient` | `fn hormone_infusion_transient(infusion_rate: f64, clearance_rate: f64, t: f64) → f64` | Transient infusion concentration | `biology::endocrinology::kinetics` |
| `pulsatile_secretion` | `fn pulsatile_secretion(amplitude: f64, frequency: f64, phase: f64, baseline: f64, t: f64) → f64` | Pulsatile secretion model | `biology::endocrinology::kinetics` |
| `negative_feedback` | `fn negative_feedback(hormone_level: f64, set_point: f64, gain: f64) → f64` | Negative feedback | `biology::endocrinology::kinetics` |
| `positive_feedback` | `fn positive_feedback(hormone_level: f64, threshold: f64, gain: f64, max_rate: f64) → f64` | Positive feedback | `biology::endocrinology::kinetics` |
| `hpa_axis_step` | `fn hpa_axis_step(crf: f64, acth: f64, cortisol: f64, k1: f64, k2: f64, k3: f64, d1: f64, d2: f64, d3: f64, neg_gain: f64) → (f64, f64, f64)` | HPA axis ODE step | `biology::endocrinology::kinetics` |
| `thyroid_axis_tsh_t4` | `fn thyroid_axis_tsh_t4(tsh: f64, t4: f64, trh: f64, k_stim: f64, k_inh: f64, k_prod: f64, d_tsh: f64, d_t4: f64) → (f64, f64)` | Thyroid axis (TSH/T4) ODE | `biology::endocrinology::kinetics` |
| `insulin_secretion_glucose` | `fn insulin_secretion_glucose(glucose: f64, beta_cell_mass: f64, km: f64, vmax: f64) → f64` | Glucose-stimulated insulin secretion | `biology::endocrinology::kinetics` |
| `glucose_insulin_dynamics` | `fn glucose_insulin_dynamics(glucose: f64, insulin: f64, gin: f64, si: f64, sg: f64, n: f64, ib: f64, gb: f64) → (f64, f64)` | Glucose-insulin dynamics ODE | `biology::endocrinology::kinetics` |
| `hormone_binding_to_carrier` | `fn hormone_binding_to_carrier(total_hormone: f64, carrier: f64, kd: f64) → f64` | Hormone-carrier binding | `biology::endocrinology::kinetics` |
| `free_hormone_fraction` | `fn free_hormone_fraction(total: f64, binding_proteins: f64, kd: f64) → f64` | Free hormone fraction | `biology::endocrinology::kinetics` |
| `cortisol_diurnal_rhythm` | `fn cortisol_diurnal_rhythm(t_hours: f64, peak_amplitude: f64, nadir: f64) → f64` | Cortisol diurnal rhythm | `biology::endocrinology::kinetics` |
| `growth_hormone_pulse` | `fn growth_hormone_pulse(t: f64, pulse_times: &[f64], amplitude: f64, half_life: f64) → f64` | GH pulsatile secretion | `biology::endocrinology::kinetics` |
| `renin_angiotensin_aldosterone` | `fn renin_angiotensin_aldosterone(renin: f64, angiotensinogen: f64, ace: f64, k_renin: f64, k_ace: f64, k_aldo: f64) → (f64, f64, f64)` | RAAS cascade kinetics | `biology::endocrinology::kinetics` |
| `parathyroid_calcium_response` | `fn parathyroid_calcium_response(calcium: f64, set_point: f64, max_pth: f64, steepness: f64) → f64` | PTH-calcium response | `biology::endocrinology::kinetics` |
| `leptin_secretion` | `fn leptin_secretion(fat_mass: f64, sensitivity: f64) → f64` | Leptin secretion | `biology::endocrinology::kinetics` |
| `ghrelin_fasting_profile` | `fn ghrelin_fasting_profile(t_since_meal: f64, peak_time: f64, amplitude: f64, baseline: f64) → f64` | Ghrelin fasting profile | `biology::endocrinology::kinetics` |

#### receptors.rs (19 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `receptor_binding_fraction` | `fn receptor_binding_fraction(ligand: f64, kd: f64) → f64` | $f = \frac{[L]}{K_d + [L]}$ | `biology::endocrinology::receptors` |
| `competitive_binding` | `fn competitive_binding(ligand: f64, competitor: f64, kd: f64, ki: f64) → f64` | Competitive binding | `biology::endocrinology::receptors` |
| `receptor_up_regulation` | `fn receptor_up_regulation(r0: f64, stimulus: f64, k_up: f64, k_deg: f64, t: f64) → f64` | Receptor up-regulation | `biology::endocrinology::receptors` |
| `receptor_down_regulation` | `fn receptor_down_regulation(r0: f64, stimulus: f64, k_down: f64, k_synth: f64, t: f64) → f64` | Receptor down-regulation | `biology::endocrinology::receptors` |
| `dose_response_hill` | `fn dose_response_hill(dose: f64, ec50: f64, emax: f64, n: f64) → f64` | $E = \frac{E_{max} D^n}{EC_{50}^n + D^n}$ | `biology::endocrinology::receptors` |
| `insulin_glucose_minimal_model` | `fn insulin_glucose_minimal_model(g: f64, x: f64, insulin: f64, gb: f64, p1: f64, p2: f64, p3: f64, ib: f64) → (f64, f64)` | Bergman minimal model | `biology::endocrinology::receptors` |
| `receptor_internalization` | `fn receptor_internalization(surface: f64, ligand: f64, k_intern: f64, k_recycle: f64) → (f64, f64)` | Receptor internalization/recycling | `biology::endocrinology::receptors` |
| `receptor_clearance_rate` | `fn receptor_clearance_rate(concentration: f64, half_life: f64) → f64` | Receptor clearance rate | `biology::endocrinology::receptors` |
| `feedback_loop_negative` | `fn feedback_loop_negative(stimulus: f64, hormone: f64, sensitivity: f64, set_point: f64) → f64` | Negative feedback loop | `biology::endocrinology::receptors` |
| `receptor_pulsatile_response` | `fn receptor_pulsatile_response(amplitude: f64, frequency: f64, t: f64, baseline: f64) → f64` | Receptor pulsatile response | `biology::endocrinology::receptors` |
| `allosteric_modulation` | `fn allosteric_modulation(ligand: f64, modulator: f64, kd: f64, alpha: f64, beta: f64) → f64` | Allosteric modulation | `biology::endocrinology::receptors` |
| `spare_receptor_response` | `fn spare_receptor_response(ligand: f64, kd: f64, receptor_reserve: f64) → f64` | Spare receptor response | `biology::endocrinology::receptors` |
| `desensitization_kinetics` | `fn desensitization_kinetics(r0: f64, agonist: f64, k_desens: f64, k_resens: f64, t: f64) → f64` | Desensitization kinetics | `biology::endocrinology::receptors` |
| `second_messenger_camp` | `fn second_messenger_camp(receptor_activity: f64, k_synth: f64, k_pde: f64, basal: f64) → f64` | cAMP second messenger | `biology::endocrinology::receptors` |
| `ip3_calcium_release` | `fn ip3_calcium_release(ip3: f64, k_release: f64, k_serca: f64, store: f64) → f64` | IP3-mediated Ca²⁺ release | `biology::endocrinology::receptors` |
| `receptor_dimerization` | `fn receptor_dimerization(monomer: f64, kd_dimer: f64) → f64` | Receptor dimerization | `biology::endocrinology::receptors` |
| `beta_arrestin_recruitment` | `fn beta_arrestin_recruitment(agonist: f64, receptor: f64, k_arr: f64) → f64` | β-arrestin recruitment | `biology::endocrinology::receptors` |
| `receptor_tyrosine_kinase_activation` | `fn receptor_tyrosine_kinase_activation(ligand: f64, receptor: f64, km: f64, vmax: f64) → f64` | Receptor tyrosine kinase activation | `biology::endocrinology::receptors` |
| `gpcr_g_protein_cycle` | `fn gpcr_g_protein_cycle(active_receptor: f64, gdp_bound: f64, k_exchange: f64, k_hydrolysis: f64) → (f64, f64)` | GPCR G-protein cycle | `biology::endocrinology::receptors` |

---

### 1️⃣9️⃣ reproduction/ — Reproduction — 62 pub fn

#### embryogenesis.rs (23 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `cleavage_timing` | `fn cleavage_timing(stage: u32, base_interval: f64, temperature_factor: f64) → f64` | Cleavage timing between stages | `biology::reproduction::embryogenesis` |
| `blastocyst_cell_count` | `fn blastocyst_cell_count(initial_cells: f64, division_rate: f64, t: f64) → f64` | $N(t) = N_0 e^{rt}$ | `biology::reproduction::embryogenesis` |
| `morphogen_gradient_embryo` | `fn morphogen_gradient_embryo(source: f64, diffusion: f64, degradation: f64, x: f64) → f64` | $C(x) = C_0 e^{-x\sqrt{k/D}}$ | `biology::reproduction::embryogenesis` |
| `gastrulation_cell_migration` | `fn gastrulation_cell_migration(chemotactic_sensitivity: f64, gradient: f64, random_motility: f64) → f64` | Chemotactic cell migration rate | `biology::reproduction::embryogenesis` |
| `somitogenesis_clock` | `fn somitogenesis_clock(frequency: f64, wavefront_speed: f64, position: f64, t: f64) → f64` | Clock-wavefront somitogenesis model | `biology::reproduction::embryogenesis` |
| `fetal_weight_hadlock` | `fn fetal_weight_hadlock(gestational_age_weeks: f64) → f64` | Hadlock fetal weight estimation | `biology::reproduction::embryogenesis` |
| `placental_transfer_rate` | `fn placental_transfer_rate(maternal_conc: f64, fetal_conc: f64, permeability: f64, surface_area: f64) → f64` | $J = P \cdot A \cdot (C_m - C_f)$ | `biology::reproduction::embryogenesis` |
| `crown_rump_length` | `fn crown_rump_length(gestational_age_weeks: f64) → f64` | Crown-rump length from GA | `biology::reproduction::embryogenesis` |
| `biparietal_diameter` | `fn biparietal_diameter(gestational_age_weeks: f64) → f64` | Biparietal diameter from GA | `biology::reproduction::embryogenesis` |
| `amniotic_fluid_index` | `fn amniotic_fluid_index(quadrants: &[f64; 4]) → f64` | $\text{AFI} = \sum_{i=1}^{4} q_i$ | `biology::reproduction::embryogenesis` |
| `neural_tube_closure_progress` | `fn neural_tube_closure_progress(t: f64, rate: f64, max_closure: f64) → f64` | Neural tube closure progress | `biology::reproduction::embryogenesis` |
| `organogenesis_differentiation_rate` | `fn organogenesis_differentiation_rate(morphogen_conc: f64, threshold: f64, hill_coefficient: f64) → f64` | Hill-type differentiation rate | `biology::reproduction::embryogenesis` |
| `turing_activator_inhibitor` | `fn turing_activator_inhibitor(activator: f64, inhibitor: f64, rho_a: f64, rho_i: f64, mu_a: f64, mu_i: f64, kappa: f64) → (f64, f64)` | Turing reaction-diffusion pattern | `biology::reproduction::embryogenesis` |
| `fetal_lung_maturity_ls_ratio` | `fn fetal_lung_maturity_ls_ratio(lecithin: f64, sphingomyelin: f64) → f64` | L/S ratio for lung maturity | `biology::reproduction::embryogenesis` |
| `apgar_component` | `fn apgar_component(heart_rate: f64, respiration: f64, muscle_tone: f64, reflex: f64, color: f64) → f64` | APGAR score computation | `biology::reproduction::embryogenesis` |
| `fetal_heart_rate_baseline` | `fn fetal_heart_rate_baseline(gestational_age_weeks: f64) → f64` | Baseline fetal heart rate | `biology::reproduction::embryogenesis` |
| `umbilical_artery_pi` | `fn umbilical_artery_pi(systolic: f64, diastolic: f64, mean: f64) → f64` | $\text{PI} = (S - D) / \bar{V}$ | `biology::reproduction::embryogenesis` |
| `placental_oxygen_delivery` | `fn placental_oxygen_delivery(blood_flow: f64, hemoglobin: f64, saturation: f64, o2_binding_capacity: f64) → f64` | Placental O₂ delivery | `biology::reproduction::embryogenesis` |
| `trophoblast_invasion_depth` | `fn trophoblast_invasion_depth(migration_rate: f64, protease_activity: f64, resistance: f64, t: f64) → f64` | Trophoblast invasion depth | `biology::reproduction::embryogenesis` |
| `gestational_sac_diameter` | `fn gestational_sac_diameter(gestational_age_days: f64) → f64` | Gestational sac diameter | `biology::reproduction::embryogenesis` |
| `yolk_sac_regression` | `fn yolk_sac_regression(initial_size: f64, regression_rate: f64, t: f64) → f64` | $S(t) = S_0 e^{-kt}$ | `biology::reproduction::embryogenesis` |
| `limb_bud_outgrowth` | `fn limb_bud_outgrowth(fgf_conc: f64, shh_conc: f64, growth_rate: f64, t: f64) → f64` | FGF/SHH-mediated limb outgrowth | `biology::reproduction::embryogenesis` |
| `cell_fate_probability` | `fn cell_fate_probability(signal_strength: f64, noise: f64, threshold: f64) → f64` | Cell fate decision probability | `biology::reproduction::embryogenesis` |

#### fertility.rs (21 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `ovarian_cycle_hormone` | `fn ovarian_cycle_hormone(t: f64, amplitude: f64, peak_day: f64, width: f64, baseline: f64) → f64` | Gaussian hormone pulse model | `biology::reproduction::fertility` |
| `follicle_growth` | `fn follicle_growth(diameter: f64, fsh: f64, growth_rate: f64, max_diameter: f64) → f64` | FSH-dependent follicular growth | `biology::reproduction::fertility` |
| `sperm_motility_fraction` | `fn sperm_motility_fraction(velocity: f64, threshold: f64, concentration: f64) → f64` | Motile sperm fraction | `biology::reproduction::fertility` |
| `sperm_capacitation_rate` | `fn sperm_capacitation_rate(t: f64, half_time: f64) → f64` | $C(t) = 1 - e^{-t \ln 2 / t_{1/2}}$ | `biology::reproduction::fertility` |
| `fertilization_probability` | `fn fertilization_probability(sperm_count: f64, half_max: f64) → f64` | $P = N / (N + K_{1/2})$ | `biology::reproduction::fertility` |
| `implantation_window` | `fn implantation_window(progesterone: f64, threshold: f64, estrogen_ratio: f64) → bool` | Implantation window check | `biology::reproduction::fertility` |
| `hcg_doubling` | `fn hcg_doubling(initial: f64, doubling_time: f64, t: f64) → f64` | $\text{hCG}(t) = \text{hCG}_0 \cdot 2^{t/t_d}$ | `biology::reproduction::fertility` |
| `lh_surge_model` | `fn lh_surge_model(t: f64, t_peak: f64, amplitude: f64, rise_rate: f64, fall_rate: f64) → f64` | Asymmetric LH surge model | `biology::reproduction::fertility` |
| `estradiol_follicular` | `fn estradiol_follicular(follicle_diameter: f64, num_follicles: f64, sensitivity: f64) → f64` | Estradiol from follicular development | `biology::reproduction::fertility` |
| `progesterone_luteal` | `fn progesterone_luteal(t_post_ovulation: f64, peak: f64, rise_rate: f64, fall_rate: f64) → f64` | Luteal progesterone profile | `biology::reproduction::fertility` |
| `oocyte_quality_age` | `fn oocyte_quality_age(base_quality: f64, age: f64, decline_start: f64, decline_rate: f64) → f64` | Age-dependent oocyte quality decline | `biology::reproduction::fertility` |
| `antral_follicle_count` | `fn antral_follicle_count(age: f64, initial_pool: f64, depletion_rate: f64) → f64` | AFC from age and depletion | `biology::reproduction::fertility` |
| `anti_mullerian_hormone` | `fn anti_mullerian_hormone(follicle_count: f64, sensitivity: f64) → f64` | AMH from follicle count | `biology::reproduction::fertility` |
| `ivf_success_rate` | `fn ivf_success_rate(age: f64, embryo_quality: f64, endometrial_thickness: f64) → f64` | IVF success rate estimation | `biology::reproduction::fertility` |
| `menstrual_cycle_length` | `fn menstrual_cycle_length(lh_peak_day: f64, luteal_phase_length: f64) → f64` | Cycle length calculation | `biology::reproduction::fertility` |
| `sperm_concentration_fertility` | `fn sperm_concentration_fertility(concentration: f64, motility: f64, morphology: f64) → f64` | Sperm quality composite index | `biology::reproduction::fertility` |
| `cumulative_pregnancy_rate` | `fn cumulative_pregnancy_rate(monthly_fecundability: f64, months: u32) → f64` | $P = 1 - (1 - p)^n$ | `biology::reproduction::fertility` |
| `zona_pellucida_binding` | `fn zona_pellucida_binding(receptors: f64, sperm_conc: f64, kd: f64) → f64` | Zona pellucida binding kinetics | `biology::reproduction::fertility` |
| `acrosome_reaction_rate` | `fn acrosome_reaction_rate(capacitated_fraction: f64, zona_signal: f64, k: f64) → f64` | Acrosome reaction rate | `biology::reproduction::fertility` |
| `endometrial_receptivity` | `fn endometrial_receptivity(p4: f64, lif: f64, integrin: f64, threshold_p4: f64) → f64` | Endometrial receptivity score | `biology::reproduction::fertility` |
| `twin_probability_dizygotic` | `fn twin_probability_dizygotic(age: f64, fsh_level: f64) → f64` | Dizygotic twin probability | `biology::reproduction::fertility` |

#### hormonal_cycles.rs (9 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `menstrual_cycle_hormone` | `fn menstrual_cycle_hormone(day: f64, hormone: &str) → f64` | Hormone level by cycle day | `biology::reproduction::hormonal_cycles` |
| `ovulation_probability` | `fn ovulation_probability(lh_surge: f64, follicle_maturity: f64, threshold: f64) → f64` | Ovulation probability model | `biology::reproduction::hormonal_cycles` |
| `endometrial_thickness` | `fn endometrial_thickness(day: f64, estrogen: f64) → f64` | Endometrial thickness model | `biology::reproduction::hormonal_cycles` |
| `fertility_window` | `fn fertility_window(cycle_day: f64, cycle_length: f64) → f64` | Fertility window score | `biology::reproduction::hormonal_cycles` |
| `hcg_doubling_time` | `fn hcg_doubling_time(initial_hcg: f64, days: f64, doubling_time: f64) → f64` | hCG doubling time model | `biology::reproduction::hormonal_cycles` |
| `implantation_probability` | `fn implantation_probability(embryo_quality: f64, endometrial_receptivity: f64, age_factor: f64) → f64` | Implantation probability | `biology::reproduction::hormonal_cycles` |
| `spermatogenesis_duration_days` | `fn spermatogenesis_duration_days() → f64` | Spermatogenesis duration (~74 days) | `biology::reproduction::hormonal_cycles` |
| `sperm_motility_score` | `fn sperm_motility_score(progressive: f64, non_progressive: f64, immotile: f64) → f64` | WHO motility score | `biology::reproduction::hormonal_cycles` |
| `testosterone_circadian` | `fn testosterone_circadian(hour: f64, peak_level: f64, trough_level: f64) → f64` | Circadian testosterone rhythm | `biology::reproduction::hormonal_cycles` |

#### ivf.rs (9 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `ivf_cycle_success_rate` | `fn ivf_cycle_success_rate(age: f64, embryo_quality: f64, endometrial_thickness: f64) → f64` | IVF cycle success rate | `biology::reproduction::ivf` |
| `ovarian_reserve_amh` | `fn ovarian_reserve_amh(amh_ng_ml: f64) → &'static str` | Ovarian reserve classification by AMH | `biology::reproduction::ivf` |
| `antral_follicle_response` | `fn antral_follicle_response(fsh_dose: f64, sensitivity: f64, max_follicles: f64) → f64` | Follicular response to FSH dose | `biology::reproduction::ivf` |
| `ohss_risk` | `fn ohss_risk(estradiol: f64, follicle_count: usize, bmi: f64) → f64` | OHSS risk score | `biology::reproduction::ivf` |
| `embryo_grading_score` | `fn embryo_grading_score(cell_count: usize, fragmentation_pct: f64, symmetry: f64) → f64` | Embryo grading score | `biology::reproduction::ivf` |
| `blastocyst_expansion_rate` | `fn blastocyst_expansion_rate(hours_post_fertilization: f64) → f64` | Blastocyst expansion rate | `biology::reproduction::ivf` |
| `cryopreservation_survival` | `fn cryopreservation_survival(cooling_rate: f64, optimal_rate: f64, cpa_conc: f64) → f64` | Cryopreservation survival rate | `biology::reproduction::ivf` |
| `cumulative_ivf_pregnancy_rate` | `fn cumulative_ivf_pregnancy_rate(cycle_rate: f64, cycles: usize) → f64` | Cumulative IVF pregnancy rate | `biology::reproduction::ivf` |
| `sperm_dna_fragmentation_impact` | `fn sperm_dna_fragmentation_impact(dfi: f64, baseline_fertility: f64) → f64` | DNA fragmentation impact on fertility | `biology::reproduction::ivf` |

---

### 2️⃣0️⃣ plant_biology/ — Plant Biology — 67 pub fn

#### ecology.rs (10 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `competitive_exclusion_tilman` | `fn competitive_exclusion_tilman(r_star_a: f64, r_star_b: f64) → &'static str` | Tilman's $R^*$ competitive exclusion | `biology::plant_biology::ecology` |
| `allelopathy_effect` | `fn allelopathy_effect(allelochemical_conc: f64, ic50: f64, max_inhibition: f64) → f64` | Allelopathic inhibition | `biology::plant_biology::ecology` |
| `light_competition_beer_lambert` | `fn light_competition_beer_lambert(light_above: f64, lai: f64, extinction_coeff: f64) → f64` | $I = I_0 e^{-kL}$ light competition | `biology::plant_biology::ecology` |
| `canopy_lai` | `fn canopy_lai(leaf_area: f64, ground_area: f64) → f64` | $\text{LAI} = A_{\text{leaf}} / A_{\text{ground}}$ | `biology::plant_biology::ecology` |
| `sla` | `fn sla(leaf_area: f64, leaf_dry_mass: f64) → f64` | $\text{SLA} = A / M$ | `biology::plant_biology::ecology` |
| `plant_defense_investment` | `fn plant_defense_investment(growth_rate: f64, defense_allocation: f64) → f64` | Growth-defense tradeoff | `biology::plant_biology::ecology` |
| `herbivory_damage` | `fn herbivory_damage(herbivore_density: f64, feeding_rate: f64, plant_biomass: f64, defense_level: f64) → f64` | Herbivory damage model | `biology::plant_biology::ecology` |
| `seed_dispersal_kernel` | `fn seed_dispersal_kernel(distance: f64, mean_dispersal: f64) → f64` | Exponential seed dispersal kernel | `biology::plant_biology::ecology` |
| `pollination_success` | `fn pollination_success(pollinator_visits: f64, pollen_per_visit: f64, ovule_count: f64) → f64` | Pollination success rate | `biology::plant_biology::ecology` |
| `nitrogen_fixation_symbiotic` | `fn nitrogen_fixation_symbiotic(nodule_mass: f64, nitrogenase_activity: f64, oxygen_limitation: f64) → f64` | Symbiotic N₂ fixation rate | `biology::plant_biology::ecology` |

#### growth.rs (25 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `root_growth_logistic` | `fn root_growth_logistic(length: f64, max_length: f64, rate: f64, dt: f64) → f64` | Logistic root growth | `biology::plant_biology::growth` |
| `auxin_gradient` | `fn auxin_gradient(source_concentration: f64, distance: f64, diffusion: f64, decay: f64) → f64` | $C(x) = C_0 e^{-x\sqrt{k/D}}$ auxin gradient | `biology::plant_biology::growth` |
| `phototropism_bending_rate` | `fn phototropism_bending_rate(light_differential: f64, sensitivity: f64) → f64` | Phototropism bending rate | `biology::plant_biology::growth` |
| `gravitropism_response` | `fn gravitropism_response(angle: f64, sensitivity: f64, dt: f64) → f64` | Gravitropism response | `biology::plant_biology::growth` |
| `leaf_area_index` | `fn leaf_area_index(total_leaf_area: f64, ground_area: f64) → f64` | $\text{LAI} = A_L / A_G$ | `biology::plant_biology::growth` |
| `beer_lambert_canopy` | `fn beer_lambert_canopy(light_above: f64, k: f64, lai: f64) → f64` | $I = I_0 e^{-kL}$ | `biology::plant_biology::growth` |
| `thermal_time` | `fn thermal_time(daily_mean_temp: f64, base_temp: f64) → f64` | Growing degree-days | `biology::plant_biology::growth` |
| `water_potential` | `fn water_potential(osmotic: f64, pressure: f64, gravitational: f64) → f64` | $\Psi = \Psi_s + \Psi_p + \Psi_g$ | `biology::plant_biology::growth` |
| `xylem_flow_rate` | `fn xylem_flow_rate(pressure_gradient: f64, conductivity: f64, cross_section: f64) → f64` | Xylem flow rate | `biology::plant_biology::growth` |
| `phloem_transport_munch` | `fn phloem_transport_munch(source_pressure: f64, sink_pressure: f64, resistance: f64) → f64` | Münch pressure-flow model | `biology::plant_biology::growth` |
| `allometric_biomass` | `fn allometric_biomass(diameter: f64, a: f64, b: f64) → f64` | $B = a \cdot D^b$ allometric scaling | `biology::plant_biology::growth` |
| `specific_leaf_area` | `fn specific_leaf_area(leaf_area: f64, leaf_dry_mass: f64) → f64` | $\text{SLA} = A / M$ | `biology::plant_biology::growth` |
| `relative_growth_rate` | `fn relative_growth_rate(biomass_initial: f64, biomass_final: f64, time: f64) → f64` | $\text{RGR} = (\ln B_f - \ln B_i) / t$ | `biology::plant_biology::growth` |
| `net_assimilation_rate` | `fn net_assimilation_rate(biomass_change: f64, leaf_area_avg: f64, time: f64) → f64` | $\text{NAR} = \Delta B / (\bar{A} \cdot t)$ | `biology::plant_biology::growth` |
| `phytochrome_response` | `fn phytochrome_response(red: f64, far_red: f64) → f64` | R:FR phytochrome photoequilibrium | `biology::plant_biology::growth` |
| `vernalization_progress` | `fn vernalization_progress(temp: f64, optimal_temp: f64, range: f64, dt: f64) → f64` | Vernalization accumulation | `biology::plant_biology::growth` |
| `photoperiod_response` | `fn photoperiod_response(day_length: f64, critical_length: f64, sensitivity: f64) → f64` | Photoperiod response model | `biology::plant_biology::growth` |
| `root_shoot_ratio` | `fn root_shoot_ratio(root_biomass: f64, shoot_biomass: f64) → f64` | Root:shoot biomass ratio | `biology::plant_biology::growth` |
| `canopy_gap_fraction` | `fn canopy_gap_fraction(lai: f64, k: f64) → f64` | $f = e^{-kL}$ canopy gap fraction | `biology::plant_biology::growth` |
| `stem_taper` | `fn stem_taper(diameter_base: f64, height_fraction: f64, taper_exponent: f64) → f64` | Stem taper model | `biology::plant_biology::growth` |
| `cavitation_vulnerability` | `fn cavitation_vulnerability(pressure: f64, p50: f64, slope: f64) → f64` | Xylem cavitation vulnerability curve | `biology::plant_biology::growth` |
| `turgor_pressure` | `fn turgor_pressure(osmotic_potential: f64, water_potential: f64) → f64` | $\Psi_p = \Psi - \Psi_s$ | `biology::plant_biology::growth` |
| `gibberellin_stem_elongation` | `fn gibberellin_stem_elongation(ga_concentration: f64, max_rate: f64, km: f64) → f64` | Gibberellin-mediated elongation | `biology::plant_biology::growth` |
| `senescence_chlorophyll_loss` | `fn senescence_chlorophyll_loss(chl0: f64, degradation_rate: f64, t: f64) → f64` | $\text{Chl}(t) = \text{Chl}_0 e^{-kt}$ | `biology::plant_biology::growth` |
| `frost_hardiness` | `fn frost_hardiness(temp: f64, lt50: f64, slope: f64) → f64` | Frost hardiness curve (LT₅₀) | `biology::plant_biology::growth` |

#### photosynthesis.rs (22 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `farquhar_c3` | `fn farquhar_c3(vcmax: f64, ci: f64, gamma_star: f64, kc: f64, ko: f64, o: f64, j: f64, rd: f64) → f64` | Farquhar C3 photosynthesis model | `biology::plant_biology::photosynthesis` |
| `light_response_curve` | `fn light_response_curve(phi: f64, ppfd: f64, amax: f64, theta: f64, rd: f64) → f64` | Non-rectangular hyperbola light response | `biology::plant_biology::photosynthesis` |
| `transpiration_rate` | `fn transpiration_rate(stomatal_conductance: f64, vpd: f64) → f64` | $E = g_s \cdot \text{VPD}$ | `biology::plant_biology::photosynthesis` |
| `ball_berry_conductance` | `fn ball_berry_conductance(a_n: f64, cs: f64, rh: f64, g0: f64, g1: f64) → f64` | $g_s = g_0 + g_1 \cdot A_n \cdot h / c_s$ | `biology::plant_biology::photosynthesis` |
| `water_use_efficiency` | `fn water_use_efficiency(a_n: f64, transpiration: f64) → f64` | $\text{WUE} = A_n / E$ | `biology::plant_biology::photosynthesis` |
| `penman_monteith` | `fn penman_monteith(net_radiation: f64, soil_heat: f64, vpd: f64, ga: f64, gs: f64, delta: f64, gamma: f64, rho: f64, cp: f64) → f64` | Penman-Monteith evapotranspiration | `biology::plant_biology::photosynthesis` |
| `rubisco_specificity` | `fn rubisco_specificity(vcmax: f64, kc: f64, vomax: f64, ko: f64) → f64` | Rubisco CO₂/O₂ specificity factor | `biology::plant_biology::photosynthesis` |
| `photorespiration_rate` | `fn photorespiration_rate(vomax: f64, o: f64, ko: f64, ci: f64, kc: f64) → f64` | Photorespiration rate | `biology::plant_biology::photosynthesis` |
| `electron_transport_rate` | `fn electron_transport_rate(ppfd: f64, absorptance: f64, fraction_psii: f64, phi_psii: f64) → f64` | $J = \text{PPFD} \cdot \alpha \cdot f_{\text{II}} \cdot \Phi_{\text{II}}$ | `biology::plant_biology::photosynthesis` |
| `stomatal_optimization` | `fn stomatal_optimization(vpd: f64, ca: f64, lambda_wue: f64, g1: f64) → f64` | Optimal stomatal conductance | `biology::plant_biology::photosynthesis` |
| `c4_photosynthesis` | `fn c4_photosynthesis(vpmax: f64, ci: f64, kp: f64, vcmax: f64, ko: f64, kc: f64, o: f64, rd: f64) → f64` | C4 photosynthesis model | `biology::plant_biology::photosynthesis` |
| `cam_malic_acid_storage` | `fn cam_malic_acid_storage(co2_fixed_night: f64, vacuole_capacity: f64, current_malate: f64) → f64` | CAM nocturnal malate storage | `biology::plant_biology::photosynthesis` |
| `cam_daytime_decarboxylation` | `fn cam_daytime_decarboxylation(malate: f64, decarboxylation_rate: f64) → f64` | CAM daytime decarboxylation | `biology::plant_biology::photosynthesis` |
| `chlorophyll_fluorescence_fv_fm` | `fn chlorophyll_fluorescence_fv_fm(f0: f64, fm: f64) → f64` | $F_v/F_m = (F_m - F_0)/F_m$ | `biology::plant_biology::photosynthesis` |
| `non_photochemical_quenching` | `fn non_photochemical_quenching(fm: f64, fm_prime: f64) → f64` | $\text{NPQ} = (F_m - F_m')/F_m'$ | `biology::plant_biology::photosynthesis` |
| `photochemical_quenching` | `fn photochemical_quenching(fs: f64, f0_prime: f64, fm_prime: f64) → f64` | $q_P = (F_m' - F_s)/(F_m' - F_0')$ | `biology::plant_biology::photosynthesis` |
| `quantum_yield_psii` | `fn quantum_yield_psii(phi_psii: f64, ppfd: f64) → f64` | PSII quantum yield × PPFD | `biology::plant_biology::photosynthesis` |
| `co2_compensation_point` | `fn co2_compensation_point(gamma_star: f64, rd: f64, vcmax: f64, kc: f64, ko: f64, o: f64) → f64` | CO₂ compensation point $\Gamma$ | `biology::plant_biology::photosynthesis` |
| `mesophyll_conductance` | `fn mesophyll_conductance(a_n: f64, ci: f64, cc: f64) → f64` | $g_m = A_n / (C_i - C_c)$ | `biology::plant_biology::photosynthesis` |
| `light_use_efficiency` | `fn light_use_efficiency(gpp: f64, apar: f64) → f64` | $\text{LUE} = \text{GPP} / \text{APAR}$ | `biology::plant_biology::photosynthesis` |
| `vcmax_temperature_response` | `fn vcmax_temperature_response(vcmax25: f64, ha: f64, temp_k: f64) → f64` | Arrhenius $V_{c,\max}$ temperature response | `biology::plant_biology::photosynthesis` |
| `jmax_temperature_peaked` | `fn jmax_temperature_peaked(jmax25: f64, ha: f64, hd: f64, ds: f64, temp_k: f64) → f64` | Peaked $J_{\max}$ temperature response | `biology::plant_biology::photosynthesis` |

#### transport.rs (10 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `xylem_flow_hagen_poiseuille` | `fn xylem_flow_hagen_poiseuille(radius: f64, pressure_gradient: f64, viscosity: f64, length: f64) → f64` | $Q = \pi r^4 \Delta P / (8\mu L)$ | `biology::plant_biology::transport` |
| `leaf_transpiration_rate` | `fn leaf_transpiration_rate(stomatal_conductance: f64, vpd: f64) → f64` | $E = g_s \cdot \text{VPD}$ | `biology::plant_biology::transport` |
| `cohesion_tension_water_potential` | `fn cohesion_tension_water_potential(osmotic: f64, pressure: f64, gravity: f64, matric: f64) → f64` | $\Psi = \Psi_s + \Psi_p + \Psi_g + \Psi_m$ | `biology::plant_biology::transport` |
| `phloem_munch_flow` | `fn phloem_munch_flow(turgor_source: f64, turgor_sink: f64, resistance: f64) → f64` | Münch pressure-flow model | `biology::plant_biology::transport` |
| `root_water_uptake` | `fn root_water_uptake(soil_water_potential: f64, root_water_potential: f64, root_conductance: f64) → f64` | $J = L_p(\Psi_{\text{soil}} - \Psi_{\text{root}})$ | `biology::plant_biology::transport` |
| `xylem_cavitation_vulnerability` | `fn xylem_cavitation_vulnerability(water_potential: f64, p50: f64, slope: f64) → f64` | Cavitation vulnerability curve (P₅₀) | `biology::plant_biology::transport` |
| `stomatal_conductance_ball_berry` | `fn stomatal_conductance_ball_berry(assimilation: f64, humidity: f64, co2_surface: f64, g0: f64, g1: f64) → f64` | Ball-Berry stomatal conductance | `biology::plant_biology::transport` |
| `sugar_loading_rate` | `fn sugar_loading_rate(sucrose_conc: f64, vmax: f64, km: f64) → f64` | Michaelis-Menten sugar loading | `biology::plant_biology::transport` |
| `root_hydraulic_conductivity` | `fn root_hydraulic_conductivity(flow_rate: f64, root_surface_area: f64, pressure_difference: f64) → f64` | Root hydraulic conductivity | `biology::plant_biology::transport` |
| `sap_flow_heat_pulse` | `fn sap_flow_heat_pulse(thermal_diffusivity: f64, heat_pulse_distance: f64, time_to_max: f64) → f64` | Heat pulse sap flow method | `biology::plant_biology::transport` |

---

### 2️⃣1️⃣ chronobiology/ — Chronobiology — 49 pub fn

#### entrainment.rs (10 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `zeitgeber_strength` | `fn zeitgeber_strength(light_intensity: f64, threshold: f64, saturation: f64) → f64` | Zeitgeber strength from light | `biology::chronobiology::entrainment` |
| `phase_response_curve` | `fn phase_response_curve(phase: f64, light_pulse_phase: f64, sensitivity: f64) → f64` | Phase response curve (PRC) | `biology::chronobiology::entrainment` |
| `jet_lag_recovery` | `fn jet_lag_recovery(timezone_shift: f64, adaptation_rate: f64, days: f64) → f64` | Jet lag recovery model | `biology::chronobiology::entrainment` |
| `shift_work_desynchrony` | `fn shift_work_desynchrony(internal_phase: f64, external_phase: f64) → f64` | Shift work circadian desynchrony | `biology::chronobiology::entrainment` |
| `seasonal_photoperiod` | `fn seasonal_photoperiod(day_of_year: usize, latitude: f64) → f64` | Seasonal photoperiod from latitude | `biology::chronobiology::entrainment` |
| `melatonin_suppression` | `fn melatonin_suppression(light_intensity: f64, ic50: f64, hill_n: f64) → f64` | Melatonin suppression by light | `biology::chronobiology::entrainment` |
| `social_zeitgeber_strength` | `fn social_zeitgeber_strength(regularity: f64, social_contacts: f64) → f64` | Social zeitgeber strength | `biology::chronobiology::entrainment` |
| `food_entrainment` | `fn food_entrainment(feeding_time: f64, clock_phase: f64, coupling: f64) → f64` | Food-entrained oscillator | `biology::chronobiology::entrainment` |
| `chronotype_score` | `fn chronotype_score(midpoint_sleep: f64) → f64` | Chronotype from mid-sleep | `biology::chronobiology::entrainment` |
| `circadian_amplitude_damping` | `fn circadian_amplitude_damping(initial_amplitude: f64, damping_rate: f64, t: f64) → f64` | $A(t) = A_0 e^{-\lambda t}$ | `biology::chronobiology::entrainment` |

#### oscillators.rs (19 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `goodwin_oscillator` | `fn goodwin_oscillator(x: f64, y: f64, z: f64, k1: f64, k2: f64, k3: f64, ki: f64, n: f64) → (f64, f64, f64)` | Goodwin oscillator derivatives | `biology::chronobiology::oscillators` |
| `van_der_pol_circadian` | `fn van_der_pol_circadian(x: f64, y: f64, mu: f64, tau: f64, light: f64, alpha: f64) → (f64, f64)` | Van der Pol circadian oscillator | `biology::chronobiology::oscillators` |
| `phase_response` | `fn phase_response(phase: f64, light_intensity: f64, sensitivity: f64, tau: f64) → f64` | Phase response to light | `biology::chronobiology::oscillators` |
| `entrainment_range` | `fn entrainment_range(coupling_strength: f64, intrinsic_period: f64) → (f64, f64)` | Arnold tongue entrainment range | `biology::chronobiology::oscillators` |
| `melatonin_profile` | `fn melatonin_profile(t: f64, onset: f64, offset: f64, amplitude: f64) → f64` | Melatonin secretion profile | `biology::chronobiology::oscillators` |
| `desynchrony_index` | `fn desynchrony_index(observed_period: f64, zeitgeber_period: f64) → f64` | Desynchrony index | `biology::chronobiology::oscillators` |
| `goodwin_simulate` | `fn goodwin_simulate(x0: f64, y0: f64, z0: f64, k1: f64, k2: f64, k3: f64, ki: f64, n: f64, dt: f64, steps: usize) → Vec<(f64, f64, f64)>` | Goodwin oscillator simulation | `biology::chronobiology::oscillators` |
| `kuramoto_order_parameter` | `fn kuramoto_order_parameter(phases: &[f64]) → (f64, f64)` | $r e^{i\psi} = \frac{1}{N}\sum e^{i\theta_k}$ | `biology::chronobiology::oscillators` |
| `kuramoto_step` | `fn kuramoto_step(phases: &mut [f64], frequencies: &[f64], coupling: f64, dt: f64)` | Kuramoto model step | `biology::chronobiology::oscillators` |
| `arnolds_tongue_boundary` | `fn arnolds_tongue_boundary(coupling: f64, detuning: f64) → bool` | Arnold tongue boundary check | `biology::chronobiology::oscillators` |
| `repressilator` | `fn repressilator(a: f64, b: f64, c: f64, alpha: f64, alpha0: f64, n: f64, beta: f64) → (f64, f64, f64)` | Repressilator genetic oscillator | `biology::chronobiology::oscillators` |
| `amplitude_phase_from_timeseries` | `fn amplitude_phase_from_timeseries(values: &[f64], period: f64) → (f64, f64)` | Amplitude and phase extraction | `biology::chronobiology::oscillators` |
| `phase_diffusion_coefficient` | `fn phase_diffusion_coefficient(phase_variance: f64, time: f64) → f64` | $D_\phi = \text{Var}(\phi) / (2t)$ | `biology::chronobiology::oscillators` |
| `limit_cycle_stability` | `fn limit_cycle_stability(floquet_exponent: f64) → bool` | Limit cycle stability (Floquet) | `biology::chronobiology::oscillators` |
| `poincare_section_period` | `fn poincare_section_period(crossing_times: &[f64]) → f64` | Poincaré section period estimation | `biology::chronobiology::oscillators` |
| `detrend_moving_average` | `fn detrend_moving_average(data: &[f64], window: usize) → Vec<f64>` | Detrending by moving average | `biology::chronobiology::oscillators` |
| `instantaneous_frequency` | `fn instantaneous_frequency(phase_prev: f64, phase_curr: f64, dt: f64) → f64` | $f = \Delta\phi / (2\pi \, dt)$ | `biology::chronobiology::oscillators` |
| `mutual_information_phase` | `fn mutual_information_phase(phases1: &[f64], phases2: &[f64], n_bins: usize) → f64` | Phase mutual information | `biology::chronobiology::oscillators` |
| `stochastic_resonance_snr` | `fn stochastic_resonance_snr(signal_power: f64, noise_intensity: f64, threshold: f64) → f64` | Stochastic resonance SNR | `biology::chronobiology::oscillators` |

#### rhythms.rs (20 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `jet_lag_resync_time` | `fn jet_lag_resync_time(time_zones_crossed: f64, resync_rate: f64) → f64` | Jet lag resynchronization time | `biology::chronobiology::rhythms` |
| `sleep_pressure` | `fn sleep_pressure(wake_duration: f64, buildup_rate: f64, max_pressure: f64) → f64` | Process S sleep pressure | `biology::chronobiology::rhythms` |
| `two_process_model` | `fn two_process_model(sleep_pressure: f64, circadian_amplitude: f64, phase: f64) → f64` | Borbély two-process model | `biology::chronobiology::rhythms` |
| `photoperiod` | `fn photoperiod(latitude_rad: f64, declination_rad: f64) → f64` | Photoperiod from latitude/declination | `biology::chronobiology::rhythms` |
| `ultradian_rhythm` | `fn ultradian_rhythm(amplitudes: &[f64], periods: &[f64], t: f64) → f64` | Ultradian rhythm superposition | `biology::chronobiology::rhythms` |
| `chronotype_shift` | `fn chronotype_shift(mid_sleep_free: f64, sleep_debt_correction: f64) → f64` | Chronotype shift (MSFsc) | `biology::chronobiology::rhythms` |
| `circadian_acrophase` | `fn circadian_acrophase(data: &[f64], period: f64) → f64` | Cosinor acrophase estimation | `biology::chronobiology::rhythms` |
| `cosinor_amplitude` | `fn cosinor_amplitude(data: &[f64], period: f64) → f64` | Cosinor amplitude estimation | `biology::chronobiology::rhythms` |
| `social_jet_lag` | `fn social_jet_lag(weekday_midsleep: f64, weekend_midsleep: f64) → f64` | Social jet lag | `biology::chronobiology::rhythms` |
| `mesor` | `fn mesor(data: &[f64]) → f64` | MESOR (midline estimating statistic) | `biology::chronobiology::rhythms` |
| `sleep_debt` | `fn sleep_debt(wake_hours: f64, optimal_sleep: f64, actual_sleep: f64) → f64` | Sleep debt accumulation | `biology::chronobiology::rhythms` |
| `circadian_phase_estimate` | `fn circadian_phase_estimate(core_body_temp_min_time: f64) → f64` | Circadian phase from CBT minimum | `biology::chronobiology::rhythms` |
| `light_phase_advance` | `fn light_phase_advance(lux: f64, sensitivity: f64, timing_factor: f64) → f64` | Light-induced phase advance | `biology::chronobiology::rhythms` |
| `dim_light_melatonin_onset` | `fn dim_light_melatonin_onset(melatonin_levels: &[f64], threshold: f64) → Option<usize>` | DLMO threshold crossing | `biology::chronobiology::rhythms` |
| `infradian_cycle` | `fn infradian_cycle(base_amplitude: f64, period_days: f64, day: f64) → f64` | Infradian cycle model | `biology::chronobiology::rhythms` |
| `temperature_compensation_q10` | `fn temperature_compensation_q10(rate_t1: f64, rate_t2: f64, t1: f64, t2: f64) → f64` | $Q_{10} = (R_2/R_1)^{10/(T_2-T_1)}$ | `biology::chronobiology::rhythms` |
| `masking_effect` | `fn masking_effect(endogenous: f64, exogenous_signal: f64, masking_gain: f64) → f64` | Masking effect on rhythm | `biology::chronobiology::rhythms` |
| `relative_amplitude` | `fn relative_amplitude(max_val: f64, min_val: f64) → f64` | Relative amplitude (RA) | `biology::chronobiology::rhythms` |
| `interdaily_stability` | `fn interdaily_stability(data: &[f64], period: usize) → f64` | Interdaily stability (IS) | `biology::chronobiology::rhythms` |
| `intradaily_variability` | `fn intradaily_variability(data: &[f64]) → f64` | Intradaily variability (IV) | `biology::chronobiology::rhythms` |

---

### 2️⃣2️⃣ neuroscience/ — Neuroscience — 54 pub fn

#### analysis.rs (10 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `firing_rate` | `fn firing_rate(spikes: &[usize], dt: f64, total_steps: usize) → f64` | Mean firing rate | `biology::neuroscience::analysis` |
| `interspike_intervals` | `fn interspike_intervals(spikes: &[usize], dt: f64) → Vec<f64>` | ISI distribution | `biology::neuroscience::analysis` |
| `coefficient_of_variation` | `fn coefficient_of_variation(intervals: &[f64]) → f64` | $\text{CV} = \sigma / \mu$ | `biology::neuroscience::analysis` |
| `fano_factor` | `fn fano_factor(spike_counts: &[usize]) → f64` | $F = \sigma^2 / \mu$ | `biology::neuroscience::analysis` |
| `spike_count_correlation` | `fn spike_count_correlation(spikes_a: &[usize], spikes_b: &[usize]) → f64` | Spike count correlation | `biology::neuroscience::analysis` |
| `cross_correlogram` | `fn cross_correlogram(spikes_a: &[f64], spikes_b: &[f64], bin_width: f64, max_lag: f64) → Vec<(f64, usize)>` | Cross-correlogram | `biology::neuroscience::analysis` |
| `local_field_potential_power` | `fn local_field_potential_power(signal: &[f64], freq: f64, dt: f64) → f64` | LFP spectral power | `biology::neuroscience::analysis` |
| `spike_triggered_average` | `fn spike_triggered_average(stimulus: &[f64], spike_times: &[usize], window: usize) → Vec<f64>` | Spike-triggered average (STA) | `biology::neuroscience::analysis` |
| `burst_detection` | `fn burst_detection(isi: &[f64], threshold: f64) → Vec<(usize, usize)>` | Burst detection from ISI | `biology::neuroscience::analysis` |
| `information_rate` | `fn information_rate(spike_counts: &[usize], stimulus_repeats: usize) → f64` | Neural information rate | `biology::neuroscience::analysis` |

#### cognition.rs (11 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `drift_diffusion_decision` | `fn drift_diffusion_decision(drift_rate: f64, noise: f64, threshold: f64, bias: f64, dt: f64, steps: usize) → (f64, usize)` | Drift-diffusion decision model | `biology::neuroscience::cognition` |
| `softmax_choice` | `fn softmax_choice(values: &[f64], temperature: f64) → Vec<f64>` | $P_i = e^{v_i/\tau} / \sum e^{v_j/\tau}$ | `biology::neuroscience::cognition` |
| `rescorla_wagner_update` | `fn rescorla_wagner_update(value: f64, reward: f64, alpha: f64) → f64` | $V \leftarrow V + \alpha(R - V)$ | `biology::neuroscience::cognition` |
| `td_learning_update` | `fn td_learning_update(value_current: f64, value_next: f64, reward: f64, alpha: f64, gamma: f64) → f64` | $V(s) \leftarrow V(s) + \alpha[R + \gamma V(s') - V(s)]$ | `biology::neuroscience::cognition` |
| `reward_prediction_error` | `fn reward_prediction_error(reward: f64, expected: f64) → f64` | $\delta = R - V$ | `biology::neuroscience::cognition` |
| `weber_fraction` | `fn weber_fraction(jnd: f64, stimulus_intensity: f64) → f64` | $k = \Delta I / I$ | `biology::neuroscience::cognition` |
| `signal_to_noise_neural` | `fn signal_to_noise_neural(signal_mean: f64, noise_std: f64) → f64` | Neural SNR | `biology::neuroscience::cognition` |
| `attentional_gain` | `fn attentional_gain(stimulus: f64, attention: f64, baseline: f64, gain: f64) → f64` | Attentional gain modulation | `biology::neuroscience::cognition` |
| `working_memory_decay` | `fn working_memory_decay(strength: f64, decay_rate: f64, interference_count: usize, dt: f64) → f64` | Working memory decay model | `biology::neuroscience::cognition` |
| `neural_efficiency` | `fn neural_efficiency(performance: f64, metabolic_cost: f64) → f64` | Neural efficiency index | `biology::neuroscience::cognition` |
| `bayesian_integration` | `fn bayesian_integration(prior_mean: f64, prior_var: f64, likelihood_mean: f64, likelihood_var: f64) → (f64, f64)` | Bayesian cue integration | `biology::neuroscience::cognition` |

#### models.rs (15 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `HodgkinHuxley::step` | `fn step(&mut self, i_ext: f64, dt: f64)` | Hodgkin-Huxley single step | `biology::neuroscience::models` |
| `HodgkinHuxley::simulate` | `fn simulate(&mut self, i_ext: f64, dt: f64, steps: usize) → Vec<f64>` | Hodgkin-Huxley simulation | `biology::neuroscience::models` |
| `FitzHughNagumo::new` | `fn new(a: f64, b: f64, tau: f64) → Self` | FitzHugh-Nagumo constructor | `biology::neuroscience::models` |
| `FitzHughNagumo::step` | `fn step(&mut self, i_ext: f64, dt: f64)` | FitzHugh-Nagumo single step | `biology::neuroscience::models` |
| `FitzHughNagumo::simulate` | `fn simulate(&mut self, i_ext: f64, dt: f64, steps: usize) → Vec<(f64, f64)>` | FitzHugh-Nagumo simulation | `biology::neuroscience::models` |
| `LeakyIntegrateFire::new` | `fn new(v_rest: f64, v_thresh: f64, v_reset: f64, tau: f64, r: f64) → Self` | LIF neuron constructor | `biology::neuroscience::models` |
| `LeakyIntegrateFire::step` | `fn step(&mut self, i_ext: f64, dt: f64) → bool` | LIF single step (spike = true) | `biology::neuroscience::models` |
| `LeakyIntegrateFire::simulate` | `fn simulate(&mut self, i_ext: f64, dt: f64, steps: usize) → (Vec<f64>, Vec<usize>)` | LIF simulation | `biology::neuroscience::models` |
| `IzhikevichNeuron::regular_spiking` | `fn regular_spiking() → Self` | Izhikevich regular spiking preset | `biology::neuroscience::models` |
| `IzhikevichNeuron::fast_spiking` | `fn fast_spiking() → Self` | Izhikevich fast spiking preset | `biology::neuroscience::models` |
| `IzhikevichNeuron::bursting` | `fn bursting() → Self` | Izhikevich bursting preset | `biology::neuroscience::models` |
| `IzhikevichNeuron::step` | `fn step(&mut self, i_ext: f64, dt: f64) → bool` | Izhikevich single step | `biology::neuroscience::models` |
| `IzhikevichNeuron::simulate` | `fn simulate(&mut self, i_ext: f64, dt: f64, steps: usize) → (Vec<f64>, Vec<usize>)` | Izhikevich simulation | `biology::neuroscience::models` |
| `MorrisLecar::step` | `fn step(&mut self, i_ext: f64, dt: f64)` | Morris-Lecar single step | `biology::neuroscience::models` |
| `MorrisLecar::simulate` | `fn simulate(&mut self, i_ext: f64, dt: f64, steps: usize) → Vec<(f64, f64)>` | Morris-Lecar simulation | `biology::neuroscience::models` |

#### networks.rs (7 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `Synapse::excitatory` | `fn excitatory(weight: f64) → Self` | Excitatory synapse constructor | `biology::neuroscience::networks` |
| `Synapse::inhibitory` | `fn inhibitory(weight: f64) → Self` | Inhibitory synapse constructor | `biology::neuroscience::networks` |
| `Synapse::step` | `fn step(&mut self, pre_spike: bool, dt: f64)` | Synapse state update | `biology::neuroscience::networks` |
| `Synapse::current` | `fn current(&self, v_post: f64) → f64` | Synaptic current | `biology::neuroscience::networks` |
| `stdp_update` | `fn stdp_update(delta_t: f64, a_plus: f64, a_minus: f64, tau_plus: f64, tau_minus: f64) → f64` | $\Delta w = A_+ e^{-\Delta t/\tau_+}$ (STDP) | `biology::neuroscience::networks` |
| `simulate_network` | `fn simulate_network(n_neurons: usize, weights: &[Vec<f64>], external_current: f64, dt: f64, steps: usize, threshold: f64, reset: f64, tau: f64, resistance: f64, rest: f64) → Vec<Vec<f64>>` | Network simulation | `biology::neuroscience::networks` |
| `mean_field_rate` | `fn mean_field_rate(mu: f64, sigma: f64, threshold: f64, reset: f64, tau: f64) → f64` | Mean-field firing rate | `biology::neuroscience::networks` |

#### synapses.rs (11 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `ltp_magnitude` | `fn ltp_magnitude(stimulus_frequency: f64, calcium_influx: f64, threshold: f64, max_potentiation: f64) → f64` | Long-term potentiation magnitude | `biology::neuroscience::synapses` |
| `ltd_magnitude` | `fn ltd_magnitude(calcium_level: f64, low_threshold: f64, high_threshold: f64, max_depression: f64) → f64` | Long-term depression magnitude | `biology::neuroscience::synapses` |
| `stdp_weight_change` | `fn stdp_weight_change(delta_t_ms: f64, a_plus: f64, a_minus: f64, tau_plus: f64, tau_minus: f64) → f64` | STDP weight change | `biology::neuroscience::synapses` |
| `synaptic_vesicle_release_probability` | `fn synaptic_vesicle_release_probability(calcium: f64, n_vesicles: usize, p_single: f64) → f64` | Vesicle release probability | `biology::neuroscience::synapses` |
| `short_term_facilitation` | `fn short_term_facilitation(baseline_p: f64, facilitation: f64, tau: f64, isi: f64) → f64` | Short-term facilitation | `biology::neuroscience::synapses` |
| `short_term_depression` | `fn short_term_depression(available: f64, release_p: f64, recovery_tau: f64, isi: f64) → f64` | Short-term depression | `biology::neuroscience::synapses` |
| `miniature_epsp_amplitude` | `fn miniature_epsp_amplitude(quantal_size: f64, n_receptors: f64, receptor_conductance: f64) → f64` | Miniature EPSP amplitude | `biology::neuroscience::synapses` |
| `nmda_voltage_dependence` | `fn nmda_voltage_dependence(voltage: f64, mg_conc: f64) → f64` | NMDA Mg²⁺ voltage block | `biology::neuroscience::synapses` |
| `dendritic_spine_volume_change` | `fn dendritic_spine_volume_change(calcium: f64, actin_polymerization: f64, spine_volume: f64, max_growth: f64) → f64` | Dendritic spine volume change | `biology::neuroscience::synapses` |
| `homeostatic_synaptic_scaling` | `fn homeostatic_synaptic_scaling(target_rate: f64, current_rate: f64, scaling_tau: f64, dt: f64) → f64` | Homeostatic synaptic scaling | `biology::neuroscience::synapses` |
| `glutamate_clearance` | `fn glutamate_clearance(released: f64, transporter_density: f64, vmax: f64, km: f64) → f64` | Glutamate transporter clearance | `biology::neuroscience::synapses` |

---

### 2️⃣3️⃣ pharmacology/ — Pharmacology — 72 pub fn

#### absorption.rs (10 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `oral_bioavailability` | `fn oral_bioavailability(fraction_absorbed: f64, gut_wall_extraction: f64, hepatic_extraction: f64) → f64` | $F = f_a(1-E_g)(1-E_h)$ | `biology::pharmacology::absorption` |
| `intestinal_permeability_papp` | `fn intestinal_permeability_papp(amount_receiver: f64, area: f64, time: f64, donor_conc: f64) → f64` | $P_{\text{app}} = dQ/(dt \cdot A \cdot C_0)$ | `biology::pharmacology::absorption` |
| `dissolution_noyes_whitney` | `fn dissolution_noyes_whitney(diffusion_coeff: f64, surface_area: f64, cs: f64, c: f64, thickness: f64, volume: f64) → f64` | Noyes-Whitney dissolution | `biology::pharmacology::absorption` |
| `biopharmaceutics_classification` | `fn biopharmaceutics_classification(solubility_high: bool, permeability_high: bool) → u8` | BCS classification (I–IV) | `biology::pharmacology::absorption` |
| `hepatic_clearance_well_stirred` | `fn hepatic_clearance_well_stirred(liver_blood_flow: f64, fu: f64, cl_int: f64) → f64` | $CL_h = Q_h f_u CL_{\text{int}} / (Q_h + f_u CL_{\text{int}})$ | `biology::pharmacology::absorption` |
| `renal_drug_clearance` | `fn renal_drug_clearance(gfr: f64, fu: f64, secretion: f64, reabsorption_fraction: f64) → f64` | Renal drug clearance | `biology::pharmacology::absorption` |
| `protein_binding` | `fn protein_binding(ka: f64, protein_conc: f64) → f64` | Protein binding fraction | `biology::pharmacology::absorption` |
| `apparent_volume_of_distribution` | `fn apparent_volume_of_distribution(dose: f64, plasma_concentration: f64) → f64` | $V_d = D / C_0$ | `biology::pharmacology::absorption` |
| `compartment_distribution` | `fn compartment_distribution(dose: f64, kel: f64, k12: f64, k21: f64, t: f64) → f64` | Two-compartment distribution | `biology::pharmacology::absorption` |
| `p_glycoprotein_efflux` | `fn p_glycoprotein_efflux(intracellular_conc: f64, pgp_activity: f64, km: f64) → f64` | P-gp efflux rate | `biology::pharmacology::absorption` |

#### drug_interactions.rs (11 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `drug_drug_interaction_auc_ratio` | `fn drug_drug_interaction_auc_ratio(inhibitor_conc: f64, ki: f64) → f64` | $\text{AUC ratio} = 1 + [I]/K_i$ | `biology::pharmacology::drug_interactions` |
| `cyp_induction_fold` | `fn cyp_induction_fold(inducer_conc: f64, ec50: f64, emax: f64) → f64` | CYP induction fold change | `biology::pharmacology::drug_interactions` |
| `competitive_displacement` | `fn competitive_displacement(drug_a_bound: f64, drug_b_conc: f64, kb: f64) → f64` | Competitive protein displacement | `biology::pharmacology::drug_interactions` |
| `synergy_bliss_independence` | `fn synergy_bliss_independence(effect_a: f64, effect_b: f64) → f64` | $E_{AB} = E_A + E_B - E_A E_B$ | `biology::pharmacology::drug_interactions` |
| `loewe_combination_index` | `fn loewe_combination_index(dose_a: f64, dose_a_alone: f64, dose_b: f64, dose_b_alone: f64) → f64` | Loewe CI = $d_A/D_A + d_B/D_B$ | `biology::pharmacology::drug_interactions` |
| `isobologram_point` | `fn isobologram_point(dose_a: f64, ic50_a: f64, dose_b: f64, ic50_b: f64) → f64` | Isobologram interaction index | `biology::pharmacology::drug_interactions` |
| `prodrug_activation` | `fn prodrug_activation(prodrug_conc: f64, enzyme_activity: f64, km: f64, activation_fraction: f64) → f64` | Prodrug activation rate | `biology::pharmacology::drug_interactions` |
| `drug_therapeutic_index` | `fn drug_therapeutic_index(td50: f64, ed50: f64) → f64` | $\text{TI} = TD_{50}/ED_{50}$ | `biology::pharmacology::drug_interactions` |
| `loading_dose_calculation` | `fn loading_dose_calculation(target_concentration: f64, volume_of_distribution: f64, bioavailability: f64) → f64` | $D_L = C_{ss} V_d / F$ | `biology::pharmacology::drug_interactions` |
| `maintenance_dose_calculation` | `fn maintenance_dose_calculation(target_concentration: f64, clearance: f64, bioavailability: f64, dosing_interval: f64) → f64` | $D_M = C_{ss} CL \tau / F$ | `biology::pharmacology::drug_interactions` |
| `steady_state_accumulation` | `fn steady_state_accumulation(dose: f64, half_life: f64, dosing_interval: f64) → f64` | Steady-state accumulation factor | `biology::pharmacology::drug_interactions` |

#### pharmacodynamics.rs (23 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `emax_model` | `fn emax_model(e0: f64, emax: f64, c: f64, ec50: f64) → f64` | $E = E_0 + E_{\max} C / (EC_{50} + C)$ | `biology::pharmacology::pharmacodynamics` |
| `sigmoid_emax` | `fn sigmoid_emax(e0: f64, emax: f64, c: f64, ec50: f64, n: f64) → f64` | $E = E_0 + E_{\max} C^n / (EC_{50}^n + C^n)$ | `biology::pharmacology::pharmacodynamics` |
| `log_logistic` | `fn log_logistic(c: f64, ec50: f64, slope: f64) → f64` | Log-logistic dose-response | `biology::pharmacology::pharmacodynamics` |
| `therapeutic_index` | `fn therapeutic_index(td50: f64, ed50: f64) → f64` | $\text{TI} = TD_{50} / ED_{50}$ | `biology::pharmacology::pharmacodynamics` |
| `dose_response_hill` | `fn dose_response_hill(dose: f64, dmax: f64, ec50: f64, n: f64) → f64` | Hill dose-response | `biology::pharmacology::pharmacodynamics` |
| `competitive_antagonism` | `fn competitive_antagonism(agonist: f64, ec50: f64, antagonist: f64, kb: f64) → f64` | Competitive antagonism shift | `biology::pharmacology::pharmacodynamics` |
| `schild_equation` | `fn schild_equation(dose_ratio: f64, antagonist: f64) → f64` | Schild equation $pA_2$ | `biology::pharmacology::pharmacodynamics` |
| `receptor_occupancy` | `fn receptor_occupancy(l: f64, kd: f64) → f64` | $\rho = [L] / ([L] + K_d)$ | `biology::pharmacology::pharmacodynamics` |
| `clark_equation` | `fn clark_equation(l: f64, kd: f64, emax: f64) → f64` | Clark occupancy-response | `biology::pharmacology::pharmacodynamics` |
| `operational_model` | `fn operational_model(l: f64, kd: f64, tau: f64, n: f64, emax: f64) → f64` | Black-Leff operational model | `biology::pharmacology::pharmacodynamics` |
| `imax_model` | `fn imax_model(i0: f64, imax: f64, c: f64, ic50: f64) → f64` | $I_{\max}$ inhibition model | `biology::pharmacology::pharmacodynamics` |
| `combination_index` | `fn combination_index(d1: f64, dx1: f64, d2: f64, dx2: f64) → f64` | Chou-Talalay combination index | `biology::pharmacology::pharmacodynamics` |
| `non_competitive_antagonism` | `fn non_competitive_antagonism(agonist: f64, ec50: f64, antagonist: f64, kb: f64, emax: f64) → f64` | Non-competitive antagonism | `biology::pharmacology::pharmacodynamics` |
| `irreversible_antagonism` | `fn irreversible_antagonism(agonist: f64, ec50: f64, fraction_remaining: f64, emax: f64) → f64` | Irreversible antagonism | `biology::pharmacology::pharmacodynamics` |
| `allosteric_modulator` | `fn allosteric_modulator(agonist: f64, ec50: f64, modulator: f64, alpha: f64, beta: f64, km: f64, emax: f64) → f64` | Allosteric modulation | `biology::pharmacology::pharmacodynamics` |
| `patlak_plot_slope` | `fn patlak_plot_slope(plasma_integral: f64, plasma_conc: f64, tissue_conc: f64) → f64` | Patlak plot influx constant | `biology::pharmacology::pharmacodynamics` |
| `two_state_receptor` | `fn two_state_receptor(l: f64, kd_active: f64, kd_inactive: f64, l0: f64) → f64` | Two-state receptor model | `biology::pharmacology::pharmacodynamics` |
| `partial_agonist_effect` | `fn partial_agonist_effect(l: f64, kd: f64, intrinsic_efficacy: f64, emax: f64) → f64` | Partial agonist effect | `biology::pharmacology::pharmacodynamics` |
| `inverse_agonist_effect` | `fn inverse_agonist_effect(e0: f64, l: f64, kd: f64, neg_efficacy: f64) → f64` | Inverse agonist effect | `biology::pharmacology::pharmacodynamics` |
| `biased_agonism_ratio` | `fn biased_agonism_ratio(e1: f64, ec50_1: f64, e2: f64, ec50_2: f64) → f64` | Biased agonism transduction ratio | `biology::pharmacology::pharmacodynamics` |
| `pk_pd_effect_compartment` | `fn pk_pd_effect_compartment(ce: f64, emax: f64, ec50: f64, n: f64) → f64` | PK/PD effect compartment | `biology::pharmacology::pharmacodynamics` |
| `hysteresis_collapse_ke0` | `fn hysteresis_collapse_ke0(plasma: f64, effect_prev: f64, ke0: f64, dt: f64) → f64` | Hysteresis collapse ($k_{e0}$) | `biology::pharmacology::pharmacodynamics` |
| `tolerance_factor` | `fn tolerance_factor(exposure_time: f64, tolerance_rate: f64) → f64` | $T(t) = e^{-k_{\text{tol}} t}$ | `biology::pharmacology::pharmacodynamics` |

#### pharmacokinetics.rs (28 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `one_compartment` | `fn one_compartment(dose: f64, vd: f64, ke: f64, t: f64) → f64` | $C(t) = (D/V_d) e^{-k_e t}$ | `biology::pharmacology::pharmacokinetics` |
| `one_compartment_iv_infusion` | `fn one_compartment_iv_infusion(r0: f64, ke: f64, vd: f64, t: f64, t_inf: f64) → f64` | IV infusion one-compartment | `biology::pharmacology::pharmacokinetics` |
| `two_compartment` | `fn two_compartment(a: f64, alpha: f64, b: f64, beta: f64, t: f64) → f64` | $C(t) = A e^{-\alpha t} + B e^{-\beta t}$ | `biology::pharmacology::pharmacokinetics` |
| `oral_one_compartment` | `fn oral_one_compartment(f_bio: f64, dose: f64, ka: f64, ke: f64, vd: f64, t: f64) → f64` | Oral one-compartment Bateman | `biology::pharmacology::pharmacokinetics` |
| `clearance` | `fn clearance(ke: f64, vd: f64) → f64` | $CL = k_e \cdot V_d$ | `biology::pharmacology::pharmacokinetics` |
| `half_life` | `fn half_life(ke: f64) → f64` | $t_{1/2} = \ln 2 / k_e$ | `biology::pharmacology::pharmacokinetics` |
| `auc_iv_bolus` | `fn auc_iv_bolus(dose: f64, cl: f64) → f64` | $\text{AUC} = D / CL$ | `biology::pharmacology::pharmacokinetics` |
| `auc_trapezoidal` | `fn auc_trapezoidal(times: &[f64], concentrations: &[f64]) → f64` | Linear trapezoidal AUC | `biology::pharmacology::pharmacokinetics` |
| `bioavailability` | `fn bioavailability(auc_oral: f64, dose_oral: f64, auc_iv: f64, dose_iv: f64) → f64` | $F = (\text{AUC}_{po} D_{iv}) / (\text{AUC}_{iv} D_{po})$ | `biology::pharmacology::pharmacokinetics` |
| `volume_of_distribution` | `fn volume_of_distribution(dose: f64, c0: f64) → f64` | $V_d = D / C_0$ | `biology::pharmacology::pharmacokinetics` |
| `steady_state_concentration` | `fn steady_state_concentration(dose: f64, cl: f64, tau: f64, f_bio: f64) → f64` | $C_{ss} = F \cdot D / (CL \cdot \tau)$ | `biology::pharmacology::pharmacokinetics` |
| `loading_dose` | `fn loading_dose(css_target: f64, vd: f64, f_bio: f64) → f64` | $D_L = C_{ss} V_d / F$ | `biology::pharmacology::pharmacokinetics` |
| `maintenance_dose` | `fn maintenance_dose(css_target: f64, cl: f64, tau: f64, f_bio: f64) → f64` | $D_M = C_{ss} CL \tau / F$ | `biology::pharmacology::pharmacokinetics` |
| `accumulation_factor` | `fn accumulation_factor(ke: f64, tau: f64) → f64` | $R = 1 / (1 - e^{-k_e \tau})$ | `biology::pharmacology::pharmacokinetics` |
| `tmax_oral` | `fn tmax_oral(ka: f64, ke: f64) → f64` | $t_{\max} = \ln(k_a/k_e) / (k_a - k_e)$ | `biology::pharmacology::pharmacokinetics` |
| `cmax_oral` | `fn cmax_oral(f_bio: f64, dose: f64, ka: f64, ke: f64, vd: f64) → f64` | Oral $C_{\max}$ | `biology::pharmacology::pharmacokinetics` |
| `three_compartment` | `fn three_compartment(a: f64, alpha: f64, b: f64, beta: f64, c: f64, gamma: f64, t: f64) → f64` | Three-compartment model | `biology::pharmacology::pharmacokinetics` |
| `multiple_dose_superposition` | `fn multiple_dose_superposition(dose: f64, vd: f64, ke: f64, tau: f64, t: f64, n_doses: usize) → f64` | Multiple dose superposition | `biology::pharmacology::pharmacokinetics` |
| `css_max` | `fn css_max(dose: f64, vd: f64, ke: f64, tau: f64) → f64` | $C_{ss,\max}$ peak at steady state | `biology::pharmacology::pharmacokinetics` |
| `css_min` | `fn css_min(dose: f64, vd: f64, ke: f64, tau: f64) → f64` | $C_{ss,\min}$ trough at steady state | `biology::pharmacology::pharmacokinetics` |
| `time_above_mic` | `fn time_above_mic(dose: f64, vd: f64, ke: f64, mic: f64) → f64` | Time above MIC | `biology::pharmacology::pharmacokinetics` |
| `hepatic_extraction_ratio` | `fn hepatic_extraction_ratio(cl_hepatic: f64, q_hepatic: f64) → f64` | $E_h = CL_h / Q_h$ | `biology::pharmacology::pharmacokinetics` |
| `well_stirred_model` | `fn well_stirred_model(q_h: f64, fu: f64, cl_int: f64) → f64` | Well-stirred hepatic model | `biology::pharmacology::pharmacokinetics` |
| `renal_clearance` | `fn renal_clearance(fraction_unbound: f64, gfr: f64) → f64` | $CL_r = f_u \cdot \text{GFR}$ | `biology::pharmacology::pharmacokinetics` |
| `auc_log_trapezoidal` | `fn auc_log_trapezoidal(times: &[f64], concentrations: &[f64]) → f64` | Log-linear trapezoidal AUC | `biology::pharmacology::pharmacokinetics` |
| `mean_residence_time` | `fn mean_residence_time(aumc: f64, auc: f64) → f64` | $\text{MRT} = \text{AUMC} / \text{AUC}$ | `biology::pharmacology::pharmacokinetics` |
| `aumc_trapezoidal` | `fn aumc_trapezoidal(times: &[f64], concentrations: &[f64]) → f64` | AUMC trapezoidal | `biology::pharmacology::pharmacokinetics` |
| `flip_flop_kinetics` | `fn flip_flop_kinetics(ka: f64, ke: f64) → bool` | Flip-flop kinetics check ($k_a < k_e$) | `biology::pharmacology::pharmacokinetics` |

---

### 2️⃣4️⃣ toxicology/ — Toxicology — 56 pub fn

#### accumulation.rs (18 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `bcf_ratio` | `fn bcf_ratio(c_organism: f64, c_water: f64) → f64` | $\text{BCF} = C_{\text{org}} / C_{\text{water}}$ | `biology::toxicology::accumulation` |
| `bioaccumulation_factor` | `fn bioaccumulation_factor(c_organism: f64, c_environment: f64) → f64` | $\text{BAF} = C_{\text{org}} / C_{\text{env}}$ | `biology::toxicology::accumulation` |
| `biomagnification_factor` | `fn biomagnification_factor(c_predator: f64, c_prey: f64) → f64` | $\text{BMF} = C_{\text{pred}} / C_{\text{prey}}$ | `biology::toxicology::accumulation` |
| `one_compartment_toxicokinetics` | `fn one_compartment_toxicokinetics(c0: f64, k_uptake: f64, k_elim: f64, c_exposure: f64, dt: f64, steps: usize) → Vec<f64>` | One-compartment toxicokinetic model | `biology::toxicology::accumulation` |
| `depuration_half_life` | `fn depuration_half_life(k_elim: f64) → f64` | $t_{1/2} = \ln 2 / k_e$ | `biology::toxicology::accumulation` |
| `toxic_units` | `fn toxic_units(concentration: f64, ec50: f64) → f64` | $\text{TU} = C / EC_{50}$ | `biology::toxicology::accumulation` |
| `mixture_toxicity_concentration_addition` | `fn mixture_toxicity_concentration_addition(concentrations: &[f64], ec50s: &[f64]) → f64` | $\sum C_i / EC_{50,i}$ concentration addition | `biology::toxicology::accumulation` |
| `haber_ct_product` | `fn haber_ct_product(concentration: f64, time: f64) → f64` | $\text{Ct} = C \times t$ Haber's rule | `biology::toxicology::accumulation` |
| `risk_quotient` | `fn risk_quotient(predicted_environmental_concentration: f64, predicted_no_effect_concentration: f64) → f64` | $\text{RQ} = \text{PEC} / \text{PNEC}$ | `biology::toxicology::accumulation` |
| `trophic_magnification_factor` | `fn trophic_magnification_factor(concentrations: &[f64], trophic_levels: &[f64]) → f64` | Trophic magnification factor (slope) | `biology::toxicology::accumulation` |
| `two_compartment_toxicokinetics` | `fn two_compartment_toxicokinetics(c_fast: f64, c_slow: f64, k12: f64, k21: f64, k_elim: f64, k_uptake: f64, c_exposure: f64, dt: f64) → (f64, f64)` | Two-compartment toxicokinetics | `biology::toxicology::accumulation` |
| `steady_state_body_burden` | `fn steady_state_body_burden(k_uptake: f64, c_exposure: f64, k_elim: f64) → f64` | $C_{ss} = k_u C_e / k_e$ | `biology::toxicology::accumulation` |
| `biota_sediment_accumulation_factor` | `fn biota_sediment_accumulation_factor(c_organism: f64, c_sediment: f64) → f64` | BSAF = $C_{\text{org}} / C_{\text{sed}}$ | `biology::toxicology::accumulation` |
| `lipid_normalized_concentration` | `fn lipid_normalized_concentration(c_tissue: f64, lipid_fraction: f64) → f64` | Lipid-normalized concentration | `biology::toxicology::accumulation` |
| `fugacity_ratio` | `fn fugacity_ratio(c_organism: f64, c_environment: f64, k_ow: f64) → f64` | Fugacity ratio | `biology::toxicology::accumulation` |
| `elimination_rate_from_depuration` | `fn elimination_rate_from_depuration(c_start: f64, c_end: f64, time: f64) → f64` | $k_e = \ln(C_0/C_t) / t$ | `biology::toxicology::accumulation` |
| `critical_body_residue` | `fn critical_body_residue(lc50: f64, bcf: f64) → f64` | $\text{CBR} = LC_{50} \times \text{BCF}$ | `biology::toxicology::accumulation` |
| `dietary_uptake_efficiency` | `fn dietary_uptake_efficiency(assimilation: f64, feeding_rate: f64, body_weight: f64) → f64` | Dietary uptake efficiency | `biology::toxicology::accumulation` |

#### dose_response.rs (17 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `ld50_probit` | `fn ld50_probit(doses: &[f64], responses: &[f64], totals: &[f64]) → f64` | LD₅₀ by probit analysis | `biology::toxicology::dose_response` |
| `dose_response_loglogistic` | `fn dose_response_loglogistic(dose: f64, ec50: f64, slope: f64, bottom: f64, top: f64) → f64` | Log-logistic dose-response curve | `biology::toxicology::dose_response` |
| `therapeutic_window` | `fn therapeutic_window(ec50: f64, td50: f64) → f64` | Therapeutic window | `biology::toxicology::dose_response` |
| `margin_of_safety` | `fn margin_of_safety(td01: f64, ed99: f64) → f64` | $\text{MoS} = TD_{01} / ED_{99}$ | `biology::toxicology::dose_response` |
| `benchmark_dose` | `fn benchmark_dose(ec_target: f64, ec50: f64, slope: f64) → f64` | Benchmark dose (BMD) | `biology::toxicology::dose_response` |
| `haber_rule` | `fn haber_rule(concentration: f64, time: f64, n: f64) → f64` | $C^n \times t = k$ generalized Haber | `biology::toxicology::dose_response` |
| `bioconcentration_factor` | `fn bioconcentration_factor(concentration_organism: f64, concentration_water: f64) → f64` | BCF from organism/water concentrations | `biology::toxicology::dose_response` |
| `reference_dose` | `fn reference_dose(noael: f64, uncertainty_factors: &[f64]) → f64` | $\text{RfD} = \text{NOAEL} / \prod UF_i$ | `biology::toxicology::dose_response` |
| `hormesis_model` | `fn hormesis_model(dose: f64, max_stimulation: f64, ec50_stimulation: f64, ec50_inhibition: f64, slope: f64) → f64` | Hormesis biphasic dose-response | `biology::toxicology::dose_response` |
| `weibull_dose_response` | `fn weibull_dose_response(dose: f64, alpha: f64, beta: f64) → f64` | $P = 1 - e^{-\alpha d^\beta}$ | `biology::toxicology::dose_response` |
| `multistage_cancer_model` | `fn multistage_cancer_model(dose: f64, coefficients: &[f64]) → f64` | Multistage cancer risk model | `biology::toxicology::dose_response` |
| `safety_factor_method` | `fn safety_factor_method(noael: f64, interspecies: f64, intraspecies: f64) → f64` | Safety factor method | `biology::toxicology::dose_response` |
| `acute_toxicity_ratio` | `fn acute_toxicity_ratio(lc50_48h: f64, environmental_conc: f64) → f64` | Acute toxicity ratio | `biology::toxicology::dose_response` |
| `species_sensitivity_distribution` | `fn species_sensitivity_distribution(log_hc5: f64, sigma: f64, z_05: f64) → f64` | SSD HC₅ derivation | `biology::toxicology::dose_response` |
| `no_observed_adverse_effect_concentration` | `fn no_observed_adverse_effect_concentration(responses: &[f64], controls: &[f64]) → Option<usize>` | NOAEC determination | `biology::toxicology::dose_response` |
| `dose_addition_mixture` | `fn dose_addition_mixture(concentrations: &[f64], ec50s: &[f64]) → f64` | Dose addition mixture toxicity | `biology::toxicology::dose_response` |
| `independent_action_mixture` | `fn independent_action_mixture(concentrations: &[f64], ec50s: &[f64], slopes: &[f64]) → f64` | Independent action mixture model | `biology::toxicology::dose_response` |

#### ecotoxicology.rs (10 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `ssd_rank` | `fn ssd_rank(concentrations: &[f64], test_concentration: f64) → f64` | SSD rank position | `biology::toxicology::ecotoxicology` |
| `bcf` | `fn bcf(concentration_organism: f64, concentration_water: f64) → f64` | Bioconcentration factor | `biology::toxicology::ecotoxicology` |
| `baf` | `fn baf(concentration_organism: f64, concentration_environment: f64) → f64` | Bioaccumulation factor | `biology::toxicology::ecotoxicology` |
| `bmf` | `fn bmf(concentration_predator: f64, concentration_prey: f64) → f64` | Biomagnification factor | `biology::toxicology::ecotoxicology` |
| `lc50_probit` | `fn lc50_probit(log_concentration: f64, slope: f64, intercept: f64) → f64` | LC₅₀ probit estimation | `biology::toxicology::ecotoxicology` |
| `environmental_half_life` | `fn environmental_half_life(k_deg: f64) → f64` | $t_{1/2} = \ln 2 / k$ | `biology::toxicology::ecotoxicology` |
| `fugacity_level_one` | `fn fugacity_level_one(total_mass: f64, z_values: &[f64], volumes: &[f64]) → f64` | Level I fugacity model | `biology::toxicology::ecotoxicology` |
| `predicted_no_effect_concentration` | `fn predicted_no_effect_concentration(ec50: f64, assessment_factor: f64) → f64` | $\text{PNEC} = EC_{50} / AF$ | `biology::toxicology::ecotoxicology` |
| `trophic_transfer_efficiency` | `fn trophic_transfer_efficiency(assimilated: f64, ingested: f64) → f64` | Trophic transfer efficiency | `biology::toxicology::ecotoxicology` |
| `acute_toxic_unit` | `fn acute_toxic_unit(concentration: f64, lc50: f64) → f64` | $\text{TU} = C / LC_{50}$ | `biology::toxicology::ecotoxicology` |

#### mechanisms.rs (11 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `oxidative_stress_index` | `fn oxidative_stress_index(ros_production: f64, antioxidant_capacity: f64) → f64` | Oxidative stress index | `biology::toxicology::mechanisms` |
| `dna_adduct_formation_rate` | `fn dna_adduct_formation_rate(reactive_metabolite_conc: f64, dna_conc: f64, k_adduct: f64) → f64` | DNA adduct formation rate | `biology::toxicology::mechanisms` |
| `dose_response_hill` | `fn dose_response_hill(dose: f64, emax: f64, ec50: f64, n: f64) → f64` | Hill dose-response | `biology::toxicology::mechanisms` |
| `bmd_estimate` | `fn bmd_estimate(background: f64, bmr: f64, slope: f64) → f64` | Benchmark dose estimate | `biology::toxicology::mechanisms` |
| `hepatotoxicity_clearance_ratio` | `fn hepatotoxicity_clearance_ratio(metabolite_formation: f64, metabolite_detox: f64) → f64` | Hepatotoxicity clearance ratio | `biology::toxicology::mechanisms` |
| `receptor_mediated_toxicity` | `fn receptor_mediated_toxicity(ligand: f64, receptor_total: f64, kd: f64) → f64` | Receptor-mediated toxicity | `biology::toxicology::mechanisms` |
| `safety_margin` | `fn safety_margin(noael: f64, human_exposure: f64) → f64` | $\text{MoS} = \text{NOAEL} / \text{Exposure}$ | `biology::toxicology::mechanisms` |
| `allometric_dose_scaling` | `fn allometric_dose_scaling(animal_dose: f64, animal_weight: f64, human_weight: f64) → f64` | $D_h = D_a (W_h/W_a)^{0.75}$ | `biology::toxicology::mechanisms` |
| `cytotoxicity_viability` | `fn cytotoxicity_viability(live_cells: f64, total_cells: f64) → f64` | Cell viability fraction | `biology::toxicology::mechanisms` |
| `genotoxicity_micronucleus_rate` | `fn genotoxicity_micronucleus_rate(micronuclei: f64, cells_scored: f64) → f64` | Micronucleus rate | `biology::toxicology::mechanisms` |
| `ames_mutagenicity_ratio` | `fn ames_mutagenicity_ratio(revertants_treated: f64, revertants_control: f64) → f64` | Ames test mutagenicity ratio | `biology::toxicology::mechanisms` |

---

### 2️⃣5️⃣ immunology/ — Immunology — 49 pub fn

#### adaptive.rs (7 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `clonal_expansion` | `fn clonal_expansion(n0: f64, proliferation_rate: f64, death_rate: f64, t: f64) → f64` | $N(t) = N_0 e^{(p-d)t}$ clonal expansion | `biology::immunology::adaptive` |
| `clonal_selection_dynamics` | `fn clonal_selection_dynamics(naive: f64, effector: f64, memory: f64, antigen: f64, k_activation: f64, k_prolif: f64, k_death: f64, k_memory: f64, k_clear: f64) → (f64, f64, f64, f64)` | Clonal selection ODE system | `biology::immunology::adaptive` |
| `clonal_selection_simulate` | `fn clonal_selection_simulate(naive0: f64, effector0: f64, memory0: f64, antigen0: f64, k_activation: f64, k_prolif: f64, k_death: f64, k_memory: f64, k_clear: f64, dt: f64, steps: usize) → Vec<(f64, f64, f64, f64)>` | Clonal selection simulation | `biology::immunology::adaptive` |
| `tcell_activation_threshold` | `fn tcell_activation_threshold(signal: f64, costimulation: f64, threshold: f64) → bool` | T-cell activation threshold | `biology::immunology::adaptive` |
| `cytokine_hill_response` | `fn cytokine_hill_response(cytokine: f64, ec50: f64, n: f64) → f64` | Hill cytokine response | `biology::immunology::adaptive` |
| `treg_suppression` | `fn treg_suppression(effector_rate: f64, treg: f64, k_supp: f64) → f64` | Treg-mediated suppression | `biology::immunology::adaptive` |
| `memory_recall_response` | `fn memory_recall_response(memory: f64, antigen: f64, k_recall: f64, fold_expansion: f64) → f64` | Memory recall expansion | `biology::immunology::adaptive` |

#### antibodies.rs (11 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `somatic_affinity_maturation` | `fn somatic_affinity_maturation(initial_kd: f64, rounds: usize, improvement_per_round: f64) → f64` | Somatic hypermutation affinity gain | `biology::immunology::antibodies` |
| `antibody_titer` | `fn antibody_titer(dilution_factor: f64, endpoint_dilution: usize) → f64` | Antibody titer from dilution | `biology::immunology::antibodies` |
| `isotype_switch_probability` | `fn isotype_switch_probability(cytokine_signal: f64, switch_region_accessibility: f64, aid_activity: f64) → f64` | Isotype switch probability | `biology::immunology::antibodies` |
| `somatic_hypermutation_rate` | `fn somatic_hypermutation_rate(aid_expression: f64, base_rate: f64, hotspot_factor: f64) → f64` | SHM rate | `biology::immunology::antibodies` |
| `neutralization_potency` | `fn neutralization_potency(ic50: f64, virus_titer: f64) → f64` | Neutralization potency | `biology::immunology::antibodies` |
| `opsonization_index` | `fn opsonization_index(antibody_bound: f64, fc_receptor_density: f64, complement_coating: f64) → f64` | Opsonization index | `biology::immunology::antibodies` |
| `adcc_killing_rate` | `fn adcc_killing_rate(antibody_density: f64, nk_cell_count: f64, target_count: f64, k_sat: f64) → f64` | ADCC killing rate | `biology::immunology::antibodies` |
| `complement_cascade_c3b` | `fn complement_cascade_c3b(c3: f64, convertase_activity: f64, factor_h_inhibition: f64) → f64` | C3b complement deposition | `biology::immunology::antibodies` |
| `multivalent_avidity` | `fn multivalent_avidity(monovalent_kd: f64, valency: usize, cooperativity: f64) → f64` | Multivalent avidity | `biology::immunology::antibodies` |
| `elisa_concentration` | `fn elisa_concentration(od: f64, od_max: f64, ec50: f64, hill: f64) → f64` | ELISA 4-parameter logistic | `biology::immunology::antibodies` |
| `b_cell_germinal_center_selection` | `fn b_cell_germinal_center_selection(affinity: f64, threshold: f64, t_cell_help: f64) → bool` | GC B-cell selection | `biology::immunology::antibodies` |

#### cytokines.rs (11 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `cytokine_production_rate` | `fn cytokine_production_rate(stimulus: f64, cell_count: f64, max_rate: f64, ec50: f64) → f64` | Cytokine production rate | `biology::immunology::cytokines` |
| `cytokine_decay` | `fn cytokine_decay(concentration: f64, half_life: f64, dt: f64) → f64` | Cytokine exponential decay | `biology::immunology::cytokines` |
| `th1_th2_balance` | `fn th1_th2_balance(ifn_gamma: f64, il4: f64) → f64` | Th1/Th2 balance ratio | `biology::immunology::cytokines` |
| `th17_regulatory_balance` | `fn th17_regulatory_balance(il17: f64, il10: f64, tgf_beta: f64) → f64` | Th17/Treg balance | `biology::immunology::cytokines` |
| `cytokine_storm_severity` | `fn cytokine_storm_severity(il6: f64, tnf: f64, il1b: f64, threshold_il6: f64, threshold_tnf: f64, threshold_il1b: f64) → f64` | Cytokine storm severity index | `biology::immunology::cytokines` |
| `jak_stat_signaling` | `fn jak_stat_signaling(cytokine: f64, receptor_density: f64, jak_activity: f64, socs_inhibition: f64) → f64` | JAK-STAT signaling output | `biology::immunology::cytokines` |
| `nfkb_activation` | `fn nfkb_activation(stimulus: f64, ikk_activity: f64, ikb_level: f64) → f64` | NF-κB activation | `biology::immunology::cytokines` |
| `chemokine_gradient` | `fn chemokine_gradient(source_conc: f64, distance: f64, diffusion_coeff: f64, decay_rate: f64) → f64` | $C(x) = C_0 e^{-x\sqrt{k/D}}$ chemokine gradient | `biology::immunology::cytokines` |
| `autocrine_loop` | `fn autocrine_loop(production_rate: f64, receptor_sensitivity: f64, degradation: f64) → f64` | Autocrine feedback loop | `biology::immunology::cytokines` |
| `paracrine_signaling` | `fn paracrine_signaling(source_cells: f64, target_distance: f64, diffusion: f64, production: f64, decay: f64) → f64` | Paracrine signaling model | `biology::immunology::cytokines` |
| `nlrp3_inflammasome_activation` | `fn nlrp3_inflammasome_activation(damp_signal: f64, nlrp3: f64, asc: f64, threshold: f64) → f64` | NLRP3 inflammasome activation | `biology::immunology::cytokines` |

#### dynamics.rs (10 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `sir_immune` | `fn sir_immune(s: f64, i: f64, r: f64, immune: f64, beta: f64, gamma: f64, k_immune: f64, k_decay: f64) → (f64, f64, f64, f64)` | SIR + immune dynamics | `biology::immunology::dynamics` |
| `sir_immune_simulate` | `fn sir_immune_simulate(s0: f64, i0: f64, r0: f64, immune0: f64, beta: f64, gamma: f64, k_immune: f64, k_decay: f64, dt: f64, steps: usize) → Vec<(f64, f64, f64, f64)>` | SIR-immune simulation | `biology::immunology::dynamics` |
| `vaccine_efficacy` | `fn vaccine_efficacy(arr_vacc: f64, arr_placebo: f64) → f64` | $VE = 1 - ARR_v / ARR_p$ | `biology::immunology::dynamics` |
| `herd_immunity_fraction` | `fn herd_immunity_fraction(r0: f64) → f64` | $p_c = 1 - 1/R_0$ | `biology::immunology::dynamics` |
| `antibody_decay` | `fn antibody_decay(ab0: f64, half_life: f64, t: f64) → f64` | $Ab(t) = Ab_0 \cdot 2^{-t/t_{1/2}}$ | `biology::immunology::dynamics` |
| `booster_response` | `fn booster_response(ab_pre: f64, fold_boost: f64, decay_rate: f64, t: f64) → f64` | Booster antibody response | `biology::immunology::dynamics` |
| `seroconversion_probability` | `fn seroconversion_probability(dose: f64, ed50: f64, n: f64) → f64` | Seroconversion probability | `biology::immunology::dynamics` |
| `waning_immunity` | `fn waning_immunity(immune_fraction: f64, waning_rate: f64, t: f64) → f64` | Waning immunity model | `biology::immunology::dynamics` |
| `maternal_antibody_decay` | `fn maternal_antibody_decay(ab0: f64, half_life: f64, t_months: f64) → f64` | Maternal antibody decay | `biology::immunology::dynamics` |
| `seir_step` | `fn seir_step(s: f64, e: f64, i: f64, r: f64, beta: f64, sigma: f64, gamma: f64, dt: f64) → (f64, f64, f64, f64)` | SEIR model step | `biology::immunology::dynamics` |

#### innate.rs (10 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `antigen_antibody_binding` | `fn antigen_antibody_binding(ab: f64, ag: f64, ka: f64) → f64` | $[AbAg] = [Ab][Ag] K_a$ | `biology::immunology::innate` |
| `affinity_maturation` | `fn affinity_maturation(kd_initial: f64, mutation_rate: f64, generations: usize) → Vec<f64>` | Affinity maturation over generations | `biology::immunology::innate` |
| `avidity` | `fn avidity(kd_monovalent: f64, n_sites: usize) → f64` | Multivalent avidity | `biology::immunology::innate` |
| `neutralization_curve` | `fn neutralization_curve(dose: f64, ic50: f64, n: f64) → f64` | Neutralization dose-response | `biology::immunology::innate` |
| `complement_cascade` | `fn complement_cascade(c0: f64, rate: f64, t: f64) → f64` | Complement cascade activation | `biology::immunology::innate` |
| `toll_like_receptor_activation` | `fn toll_like_receptor_activation(pamp: f64, receptor_density: f64, kd: f64) → f64` | TLR activation by PAMPs | `biology::immunology::innate` |
| `phagocytosis_rate` | `fn phagocytosis_rate(pathogen: f64, phagocyte: f64, k_phag: f64, saturation: f64) → f64` | Phagocytosis rate (saturating) | `biology::immunology::innate` |
| `nk_cell_killing` | `fn nk_cell_killing(target_mhc: f64, mhc_threshold: f64, activating_ligand: f64, kill_rate: f64) → f64` | NK cell killing (missing-self) | `biology::immunology::innate` |
| `cytokine_cascade` | `fn cytokine_cascade(initial_cytokines: &[f64], amplification: &[Vec<f64>], steps: usize) → Vec<Vec<f64>>` | Cytokine cascade simulation | `biology::immunology::innate` |
| `inflammasome_activation` | `fn inflammasome_activation(damp: f64, signal2: f64, threshold: f64, nlrp3: f64) → f64` | Inflammasome activation | `biology::immunology::innate` |

---

### 2️⃣6️⃣ microbiology/ — Microbiology — 44 pub fn

#### biofilm.rs (10 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `biofilm_formation_rate` | `fn biofilm_formation_rate(attachment_rate: f64, detachment_rate: f64, growth_rate: f64, biomass: f64) → f64` | Biofilm formation rate | `biology::microbiology::biofilm` |
| `biofilm_thickness` | `fn biofilm_thickness(biomass: f64, density: f64, area: f64) → f64` | Biofilm thickness from biomass | `biology::microbiology::biofilm` |
| `extracellular_matrix_production` | `fn extracellular_matrix_production(cell_density: f64, nutrient: f64, k_eps: f64, k_n: f64) → f64` | EPS production rate | `biology::microbiology::biofilm` |
| `biofilm_diffusion_limitation` | `fn biofilm_diffusion_limitation(bulk_concentration: f64, thickness: f64, diffusion_coeff: f64, consumption_rate: f64) → f64` | Diffusion-limited substrate in biofilm | `biology::microbiology::biofilm` |
| `persister_fraction` | `fn persister_fraction(switching_rate_to: f64, switching_rate_from: f64) → f64` | Persister cell fraction at equilibrium | `biology::microbiology::biofilm` |
| `antibiotic_resistance_mutation_rate` | `fn antibiotic_resistance_mutation_rate(mutation_rate: f64, population_size: f64, generations: f64) → f64` | Resistance emergence probability | `biology::microbiology::biofilm` |
| `minimum_inhibitory_concentration_ratio` | `fn minimum_inhibitory_concentration_ratio(mic_biofilm: f64, mic_planktonic: f64) → f64` | MIC biofilm/planktonic ratio | `biology::microbiology::biofilm` |
| `horizontal_gene_transfer` | `fn horizontal_gene_transfer(donor_freq: f64, recipient_freq: f64, transfer_rate: f64, dt: f64) → f64` | HGT conjugation frequency | `biology::microbiology::biofilm` |
| `quorum_sensing_gene_regulation` | `fn quorum_sensing_gene_regulation(autoinducer: f64, threshold: f64, max_expression: f64) → f64` | QS-regulated gene expression | `biology::microbiology::biofilm` |
| `biofilm_detachment_rate` | `fn biofilm_detachment_rate(shear_stress: f64, biofilm_strength: f64, biomass: f64) → f64` | Biofilm detachment rate | `biology::microbiology::biofilm` |

#### culture.rs (10 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `batch_culture_growth` | `fn batch_culture_growth(n0: f64, mu_max: f64, lag: f64, t: f64) → f64` | Batch culture growth with lag | `biology::microbiology::culture` |
| `chemostat_steady_state` | `fn chemostat_steady_state(s_in: f64, d: f64, mu_max: f64, ks: f64, yield_coeff: f64) → (f64, f64)` | Chemostat steady-state (S*, X*) | `biology::microbiology::culture` |
| `turbidostat_cell_density` | `fn turbidostat_cell_density(target_od: f64, current_od: f64, dilution_rate: f64, dt: f64) → f64` | Turbidostat cell density control | `biology::microbiology::culture` |
| `serial_dilution_count` | `fn serial_dilution_count(colonies: f64, dilution_factor: f64, volume_plated: f64) → f64` | CFU from serial dilution | `biology::microbiology::culture` |
| `optical_density_to_cells` | `fn optical_density_to_cells(od600: f64, calibration_factor: f64) → f64` | OD₆₀₀ to cell count | `biology::microbiology::culture` |
| `colony_forming_units` | `fn colony_forming_units(colonies: f64, dilution: f64, volume: f64) → f64` | CFU/mL calculation | `biology::microbiology::culture` |
| `doubling_time` | `fn doubling_time(growth_rate: f64) → f64` | $t_d = \ln 2 / \mu$ | `biology::microbiology::culture` |
| `continuous_culture_washout` | `fn continuous_culture_washout(mu_max: f64, ks: f64, s_in: f64) → f64` | Critical dilution rate for washout | `biology::microbiology::culture` |
| `growth_yield_coefficient` | `fn growth_yield_coefficient(biomass_produced: f64, substrate_consumed: f64) → f64` | $Y = \Delta X / \Delta S$ | `biology::microbiology::culture` |
| `dilution_rate` | `fn dilution_rate(flow_rate: f64, volume: f64) → f64` | $D = F / V$ | `biology::microbiology::culture` |

#### growth.rs (7 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `monod_growth` | `fn monod_growth(mu_max: f64, s: f64, ks: f64) → f64` | $\mu = \mu_{\max} S / (K_s + S)$ | `biology::microbiology::growth` |
| `logistic_bacterial_growth` | `fn logistic_bacterial_growth(n: f64, r: f64, k: f64) → f64` | $dN/dt = rN(1-N/K)$ | `biology::microbiology::growth` |
| `baranyi_model` | `fn baranyi_model(t: f64, y0: f64, ymax: f64, mumax: f64, lag: f64) → f64` | Baranyi bacterial growth model | `biology::microbiology::growth` |
| `lag_phase_duration` | `fn lag_phase_duration(physiological_state: f64, optimal_state: f64, adaptation_rate: f64) → f64` | Lag phase duration estimate | `biology::microbiology::growth` |
| `generation_time` | `fn generation_time(n0: f64, nt: f64, t: f64) → f64` | Generation time from growth data | `biology::microbiology::growth` |
| `diauxic_growth` | `fn diauxic_growth(s1: f64, s2: f64, mu1: f64, mu2: f64, ks1: f64, ks2: f64, ki: f64) → f64` | Diauxic growth (catabolite repression) | `biology::microbiology::growth` |
| `growth_rate_temperature_arrhenius` | `fn growth_rate_temperature_arrhenius(a: f64, ea: f64, t_kelvin: f64) → f64` | $\mu = A e^{-E_a/RT}$ Arrhenius growth | `biology::microbiology::growth` |

#### metabolism.rs (10 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `fermentation_yield` | `fn fermentation_yield(product: f64, substrate: f64) → f64` | $Y_{P/S} = P / S$ | `biology::microbiology::metabolism` |
| `anaerobic_atp_yield` | `fn anaerobic_atp_yield(substrate_carbons: usize, pathway_efficiency: f64) → f64` | Anaerobic ATP yield | `biology::microbiology::metabolism` |
| `chemolithoautotrophy_energy` | `fn chemolithoautotrophy_energy(delta_g: f64, efficiency: f64) → f64` | Chemolithoautotrophic energy yield | `biology::microbiology::metabolism` |
| `nitrogen_fixation_cost` | `fn nitrogen_fixation_cost(moles_n2: f64, atp_per_n2: f64) → f64` | ATP cost of N₂ fixation | `biology::microbiology::metabolism` |
| `denitrification_rate` | `fn denitrification_rate(no3: f64, carbon: f64, mu_max: f64, k_no3: f64, k_c: f64) → f64` | Denitrification rate (dual Monod) | `biology::microbiology::metabolism` |
| `sulfate_reduction_rate` | `fn sulfate_reduction_rate(so4: f64, donor: f64, v_max: f64, k_so4: f64, k_donor: f64) → f64` | Sulfate reduction rate | `biology::microbiology::metabolism` |
| `methanogenesis_rate` | `fn methanogenesis_rate(h2: f64, co2: f64, k_h2: f64, k_co2: f64, v_max: f64) → f64` | Methanogenesis rate | `biology::microbiology::metabolism` |
| `anammox_rate` | `fn anammox_rate(nh4: f64, no2: f64, k_nh4: f64, k_no2: f64, v_max: f64) → f64` | Anammox reaction rate | `biology::microbiology::metabolism` |
| `iron_oxidation_rate` | `fn iron_oxidation_rate(fe2: f64, o2: f64, k_fe: f64, k_o2: f64, v_max: f64) → f64` | Iron oxidation rate | `biology::microbiology::metabolism` |
| `bioremediation_degradation` | `fn bioremediation_degradation(contaminant: f64, biomass: f64, k_max: f64, k_s: f64) → f64` | Bioremediation degradation rate | `biology::microbiology::metabolism` |

#### quorum.rs (7 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `quorum_threshold` | `fn quorum_threshold(cell_density: f64, production_rate: f64, decay_rate: f64, diffusion_loss: f64) → f64` | Quorum sensing threshold | `biology::microbiology::quorum` |
| `autoinducer_accumulation` | `fn autoinducer_accumulation(production: f64, decay: f64, cell_density: f64, volume: f64) → f64` | Autoinducer steady-state | `biology::microbiology::quorum` |
| `quorum_response_hill` | `fn quorum_response_hill(ai_conc: f64, threshold: f64, n: f64, max_response: f64) → f64` | QS Hill response | `biology::microbiology::quorum` |
| `las_system_activation` | `fn las_system_activation(lasl_expression: f64, lasr_conc: f64, c12_hsl: f64, kd: f64) → f64` | LasI/LasR QS activation | `biology::microbiology::quorum` |
| `rhl_system_activation` | `fn rhl_system_activation(rhli_expression: f64, rhlr_conc: f64, c4_hsl: f64, kd: f64) → f64` | RhlI/RhlR QS activation | `biology::microbiology::quorum` |
| `competence_induction` | `fn competence_induction(csp_concentration: f64, threshold: f64, cell_density: f64) → f64` | Competence induction by CSP | `biology::microbiology::quorum` |
| `biofilm_quorum_regulation` | `fn biofilm_quorum_regulation(ai_concentration: f64, threshold_up: f64, threshold_down: f64, biofilm_state: f64) → f64` | QS biofilm regulation | `biology::microbiology::quorum` |

---

### 2️⃣7️⃣ virology/ — Virology — 59 pub fn

#### dynamics.rs (18 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `basic_reproductive_number` | `fn basic_reproductive_number(beta: f64, target_cells: f64, viral_clearance: f64, infected_death: f64) → f64` | $R_0 = \beta T_0 / (c \cdot \delta)$ within-host | `biology::virology::dynamics` |
| `effective_reproductive_number` | `fn effective_reproductive_number(r0: f64, target_fraction: f64) → f64` | $R_e = R_0 \cdot T/T_0$ | `biology::virology::dynamics` |
| `viral_clearance_rate` | `fn viral_clearance_rate(production: f64, clearance: f64) → f64` | Viral clearance rate | `biology::virology::dynamics` |
| `eclipse_phase_duration` | `fn eclipse_phase_duration(k: f64) → f64` | Eclipse phase mean duration | `biology::virology::dynamics` |
| `viral_burst_size` | `fn viral_burst_size(production_rate: f64, infected_lifespan: f64) → f64` | Viral burst size | `biology::virology::dynamics` |
| `target_cell_limited_model` | `fn target_cell_limited_model(t: f64, i: f64, v: f64, beta: f64, delta: f64, p: f64, c: f64) → (f64, f64, f64)` | Target cell limited model (dT, dI, dV) | `biology::virology::dynamics` |
| `viral_load_peak` | `fn viral_load_peak(beta: f64, p: f64, t0: f64, delta: f64, c: f64) → f64` | Peak viral load estimate | `biology::virology::dynamics` |
| `time_to_peak_viremia` | `fn time_to_peak_viremia(r0: f64, delta: f64) → f64` | Time to peak viremia | `biology::virology::dynamics` |
| `set_point_viral_load` | `fn set_point_viral_load(p: f64, lambda: f64, d: f64, beta: f64, delta: f64, c: f64) → f64` | Set-point viral load | `biology::virology::dynamics` |
| `cytopathic_effect_rate` | `fn cytopathic_effect_rate(infected_cells: f64, cpe_rate: f64) → f64` | Cytopathic effect rate | `biology::virology::dynamics` |
| `defective_interfering_particle_ratio` | `fn defective_interfering_particle_ratio(dip: f64, total: f64) → f64` | DIP ratio | `biology::virology::dynamics` |
| `viral_fitness` | `fn viral_fitness(replication_rate: f64, clearance_rate: f64) → f64` | Viral fitness measure | `biology::virology::dynamics` |
| `superinfection_exclusion` | `fn superinfection_exclusion(primary_infection: f64, secondary_moi: f64, exclusion_factor: f64) → f64` | Superinfection exclusion | `biology::virology::dynamics` |
| `viral_tropism_index` | `fn viral_tropism_index(receptor_affinity: f64, coreceptor_affinity: f64, entry_efficiency: f64) → f64` | Viral tropism index | `biology::virology::dynamics` |
| `within_host_r0` | `fn within_host_r0(beta: f64, p: f64, target: f64, delta: f64, c: f64) → f64` | Within-host $R_0$ | `biology::virology::dynamics` |
| `viral_decay_rate` | `fn viral_decay_rate(v0: f64, vt: f64, t: f64) → f64` | $c = \ln(V_0/V_t)/t$ | `biology::virology::dynamics` |
| `dual_infection_competition` | `fn dual_infection_competition(v1: f64, v2: f64, f1: f64, f2: f64, target: f64) → (f64, f64)` | Dual infection competition | `biology::virology::dynamics` |
| `viral_rebound_time` | `fn viral_rebound_time(latent_reservoir: f64, activation_rate: f64, clearance: f64) → f64` | Viral rebound time after treatment | `biology::virology::dynamics` |

#### epidemiology.rs (11 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `sir_basic_reproduction_number` | `fn sir_basic_reproduction_number(beta: f64, gamma: f64) → f64` | $R_0 = \beta / \gamma$ | `biology::virology::epidemiology` |
| `seir_latent_period_effect` | `fn seir_latent_period_effect(sigma: f64, beta: f64, gamma: f64) → f64` | SEIR effective $R_0$ with latency | `biology::virology::epidemiology` |
| `herd_immunity_threshold` | `fn herd_immunity_threshold(r0: f64) → f64` | $p_c = 1 - 1/R_0$ | `biology::virology::epidemiology` |
| `generation_time_viral` | `fn generation_time_viral(latent: f64, infectious: f64) → f64` | Mean generation time | `biology::virology::epidemiology` |
| `serial_interval` | `fn serial_interval(mean_latent: f64, mean_infectious: f64) → f64` | Serial interval estimate | `biology::virology::epidemiology` |
| `attack_rate` | `fn attack_rate(cases: f64, population: f64) → f64` | Attack rate | `biology::virology::epidemiology` |
| `case_fatality_rate` | `fn case_fatality_rate(deaths: f64, cases: f64) → f64` | CFR = deaths / cases | `biology::virology::epidemiology` |
| `secondary_attack_rate` | `fn secondary_attack_rate(secondary_cases: f64, contacts: f64) → f64` | Secondary attack rate | `biology::virology::epidemiology` |
| `epidemic_doubling_time` | `fn epidemic_doubling_time(growth_rate: f64) → f64` | $t_d = \ln 2 / r$ | `biology::virology::epidemiology` |
| `epidemic_final_size` | `fn epidemic_final_size(r0: f64) → f64` | Epidemic final size (transcendental eq.) | `biology::virology::epidemiology` |
| `superspreader_index` | `fn superspreader_index(variance: f64, mean: f64) → f64` | Dispersion parameter $k$ | `biology::virology::epidemiology` |

#### evolution.rs (20 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `quasispecies_fitness` | `fn quasispecies_fitness(master_fitness: f64, mutation_rate: f64, genome_length: f64) → f64` | Quasispecies master fitness | `biology::virology::evolution` |
| `error_threshold` | `fn error_threshold(genome_length: f64, superiority: f64) → f64` | $q_c = \sigma^{-1/L}$ error threshold | `biology::virology::evolution` |
| `muller_ratchet_step` | `fn muller_ratchet_step(population_size: f64, mutation_rate: f64, fitness_cost: f64) → f64` | Muller's ratchet click rate | `biology::virology::evolution` |
| `antigenic_drift_rate` | `fn antigenic_drift_rate(mutation_rate: f64, immune_selection: f64, population_immunity: f64) → f64` | Antigenic drift rate | `biology::virology::evolution` |
| `antigenic_shift_probability` | `fn antigenic_shift_probability(coinfection_rate: f64, segment_count: usize, compatibility: f64) → f64` | Antigenic shift probability | `biology::virology::evolution` |
| `reassortment_probability` | `fn reassortment_probability(coinfection_freq: f64, n_segments: usize) → f64` | Segment reassortment probability | `biology::virology::evolution` |
| `recombination_rate` | `fn recombination_rate(coinfection_freq: f64, template_switch_rate: f64, genome_length: f64) → f64` | Recombination rate | `biology::virology::evolution` |
| `bottleneck_diversity_loss` | `fn bottleneck_diversity_loss(initial_diversity: f64, bottleneck_size: f64) → f64` | Diversity loss through bottleneck | `biology::virology::evolution` |
| `founder_effect_diversity` | `fn founder_effect_diversity(source_diversity: f64, founder_size: f64, source_size: f64) → f64` | Founder effect diversity | `biology::virology::evolution` |
| `within_host_evolution_rate` | `fn within_host_evolution_rate(mutation_rate: f64, generation_time: f64, selection_coeff: f64, ne: f64) → f64` | Within-host evolution rate | `biology::virology::evolution` |
| `immune_escape_probability` | `fn immune_escape_probability(mutation_rate: f64, epitope_sites: f64, fitness_cost: f64) → f64` | Immune escape probability | `biology::virology::evolution` |
| `drug_resistance_probability` | `fn drug_resistance_probability(mutation_rate: f64, target_sites: f64, population_size: f64, fitness_cost: f64) → f64` | Drug resistance emergence | `biology::virology::evolution` |
| `fitness_cost_resistance` | `fn fitness_cost_resistance(wt_fitness: f64, resistant_fitness: f64) → f64` | Fitness cost of resistance | `biology::virology::evolution` |
| `compensatory_mutation_probability` | `fn compensatory_mutation_probability(fitness_deficit: f64, mutation_rate: f64, target_sites: f64) → f64` | Compensatory mutation probability | `biology::virology::evolution` |
| `evolutionary_rescue_probability` | `fn evolutionary_rescue_probability(n: f64, mutation_rate: f64, beneficial_fraction: f64, mean_benefit: f64) → f64` | Evolutionary rescue probability | `biology::virology::evolution` |
| `antigenic_distance` | `fn antigenic_distance(titers_1: &[f64], titers_2: &[f64]) → f64` | Antigenic cartography distance | `biology::virology::evolution` |
| `cross_reactivity` | `fn cross_reactivity(distance: f64, sigma: f64) → f64` | $CR = e^{-d^2/2\sigma^2}$ | `biology::virology::evolution` |
| `original_antigenic_sin` | `fn original_antigenic_sin(primary_response: f64, secondary_response: f64, cross_react: f64) → f64` | Original antigenic sin effect | `biology::virology::evolution` |
| `phylogenetic_diversity_index` | `fn phylogenetic_diversity_index(branch_lengths: &[f64]) → f64` | Phylogenetic diversity index | `biology::virology::evolution` |
| `molecular_clock_rate` | `fn molecular_clock_rate(substitutions: f64, time: f64, genome_length: f64) → f64` | Molecular clock rate | `biology::virology::evolution` |

#### structure.rs (10 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `genome_packaging_density` | `fn genome_packaging_density(genome_length: f64, capsid_volume: f64) → f64` | Genome packaging density | `biology::virology::structure` |
| `capsid_triangulation_number` | `fn capsid_triangulation_number(h: f64, k: f64) → f64` | $T = h^2 + hk + k^2$ | `biology::virology::structure` |
| `capsid_surface_area` | `fn capsid_surface_area(radius: f64) → f64` | Capsid surface area | `biology::virology::structure` |
| `genome_length_constraint` | `fn genome_length_constraint(capsid_radius: f64, persistence_length: f64) → f64` | Genome length packaging constraint | `biology::virology::structure` |
| `virion_diffusion_coefficient` | `fn virion_diffusion_coefficient(radius: f64, viscosity: f64, temperature: f64) → f64` | Stokes-Einstein diffusion | `biology::virology::structure` |
| `receptor_binding_affinity` | `fn receptor_binding_affinity(kon: f64, koff: f64) → f64` | $K_d = k_{\text{off}} / k_{\text{on}}$ | `biology::virology::structure` |
| `envelope_fusion_rate` | `fn envelope_fusion_rate(ph: f64, optimal_ph: f64, max_rate: f64, sensitivity: f64) → f64` | pH-dependent envelope fusion | `biology::virology::structure` |
| `genome_replication_fidelity` | `fn genome_replication_fidelity(error_rate: f64, genome_length: f64) → f64` | $(1 - \mu)^L$ replication fidelity | `biology::virology::structure` |
| `defective_particle_fraction` | `fn defective_particle_fraction(error_rate: f64, genome_length: f64, packaging_error: f64) → f64` | Defective particle fraction | `biology::virology::structure` |
| `virion_stability` | `fn virion_stability(temperature: f64, ph: f64, t_opt: f64, ph_opt: f64, kt: f64, kph: f64) → f64` | Virion environmental stability | `biology::virology::structure` |

---

### 2️⃣8️⃣ parasitology/ — Parasitology — 55 pub fn

#### epidemiology.rs (8 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `prevalence_rate` | `fn prevalence_rate(infected: f64, total: f64) → f64` | Prevalence rate | `biology::parasitology::epidemiology` |
| `incidence_rate` | `fn incidence_rate(new_cases: f64, person_time: f64) → f64` | Incidence rate | `biology::parasitology::epidemiology` |
| `force_of_infection` | `fn force_of_infection(beta: f64, prevalence: f64) → f64` | Force of infection $\lambda = \beta p$ | `biology::parasitology::epidemiology` |
| `parasite_aggregation_index` | `fn parasite_aggregation_index(variance: f64, mean: f64) → f64` | Aggregation index $k$ (negative binomial) | `biology::parasitology::epidemiology` |
| `negative_binomial_prevalence` | `fn negative_binomial_prevalence(mean_burden: f64, k: f64) → f64` | $1 - (1 + m/k)^{-k}$ prevalence | `biology::parasitology::epidemiology` |
| `mean_worm_burden` | `fn mean_worm_burden(total_worms: f64, hosts_examined: f64) → f64` | Mean worm burden | `biology::parasitology::epidemiology` |
| `community_infection_rate` | `fn community_infection_rate(prevalence: f64, intensity: f64, density: f64) → f64` | Community infection rate | `biology::parasitology::epidemiology` |
| `transmission_potential` | `fn transmission_potential(r0: f64, recovery_rate: f64, birth_rate: f64) → f64` | Transmission potential index | `biology::parasitology::epidemiology` |

#### host_parasite.rs (17 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `anderson_may_model` | `fn anderson_may_model(h: f64, p: f64, alpha: f64, beta: f64, b: f64, d: f64, lambda: f64, mu: f64) → (f64, f64)` | Anderson-May host-parasite model | `biology::parasitology::host_parasite` |
| `parasite_induced_mortality` | `fn parasite_induced_mortality(burden: f64, alpha: f64) → f64` | Parasite-induced host mortality | `biology::parasitology::host_parasite` |
| `castration_effect` | `fn castration_effect(fecundity: f64, burden: f64, castration_rate: f64) → f64` | Parasitic castration effect | `biology::parasitology::host_parasite` |
| `parasite_virulence_evolution` | `fn parasite_virulence_evolution(transmission: f64, virulence: f64, recovery: f64) → f64` | Virulence-transmission tradeoff R₀ | `biology::parasitology::host_parasite` |
| `host_switching_probability` | `fn host_switching_probability(phylo_distance: f64, encounter_rate: f64, compatibility: f64) → f64` | Host switching probability | `biology::parasitology::host_parasite` |
| `parasite_manipulation_index` | `fn parasite_manipulation_index(behavior_change: f64, transmission_benefit: f64, host_cost: f64) → f64` | Parasite manipulation index | `biology::parasitology::host_parasite` |
| `host_defense_cost` | `fn host_defense_cost(immune_investment: f64, baseline_fitness: f64, efficiency: f64) → f64` | Host defense cost | `biology::parasitology::host_parasite` |
| `tolerance_response` | `fn tolerance_response(damage: f64, burden: f64, tolerance_slope: f64) → f64` | Host tolerance response | `biology::parasitology::host_parasite` |
| `immune_evasion_rate` | `fn immune_evasion_rate(antigenic_variation: f64, immune_recognition: f64) → f64` | Immune evasion rate | `biology::parasitology::host_parasite` |
| `parasite_competition_within_host` | `fn parasite_competition_within_host(n1: f64, n2: f64, alpha12: f64, alpha21: f64, k1: f64, k2: f64) → (f64, f64)` | Within-host parasite competition | `biology::parasitology::host_parasite` |
| `density_dependent_fecundity` | `fn density_dependent_fecundity(max_fecundity: f64, burden: f64, crowding_coeff: f64) → f64` | Density-dependent fecundity | `biology::parasitology::host_parasite` |
| `parasite_age_intensity` | `fn parasite_age_intensity(age: f64, peak_age: f64, max_intensity: f64, exposure_rate: f64, immunity_rate: f64) → f64` | Age-intensity profile | `biology::parasitology::host_parasite` |
| `acquired_immunity_rate` | `fn acquired_immunity_rate(exposure_history: f64, immune_decay: f64) → f64` | Acquired immunity rate | `biology::parasitology::host_parasite` |
| `parasite_establishment_probability` | `fn parasite_establishment_probability(dose: f64, host_immunity: f64, parasite_fitness: f64) → f64` | Establishment probability | `biology::parasitology::host_parasite` |
| `parasite_fitness_tradeoff` | `fn parasite_fitness_tradeoff(virulence: f64, transmission: f64, host_mortality: f64) → f64` | Parasite fitness tradeoff | `biology::parasitology::host_parasite` |
| `intermediate_host_survival` | `fn intermediate_host_survival(natural_mortality: f64, parasite_mortality: f64, predation: f64) → f64` | Intermediate host survival | `biology::parasitology::host_parasite` |
| `definitive_host_parasite_load` | `fn definitive_host_parasite_load(intermediate_burden: f64, predation_rate: f64, establishment: f64) → f64` | Definitive host parasite load | `biology::parasitology::host_parasite` |

#### immunity.rs (10 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `acquired_immunity_level` | `fn acquired_immunity_level(exposure_cumulative: f64, max_immunity: f64, half_saturation: f64) → f64` | Acquired immunity level | `biology::parasitology::immunity` |
| `immunological_memory_decay` | `fn immunological_memory_decay(memory0: f64, decay_rate: f64, t: f64) → f64` | $M(t) = M_0 e^{-\delta t}$ | `biology::parasitology::immunity` |
| `cross_immunity` | `fn cross_immunity(strain_distance: f64, sigma: f64) → f64` | $CI = e^{-d^2/2\sigma^2}$ | `biology::parasitology::immunity` |
| `vaccine_induced_immunity` | `fn vaccine_induced_immunity(efficacy: f64, doses: f64, waning_rate: f64, time_since: f64) → f64` | Vaccine-induced immunity | `biology::parasitology::immunity` |
| `maternally_derived_immunity` | `fn maternally_derived_immunity(maternal_level: f64, half_life: f64, age_months: f64) → f64` | Maternally-derived immunity decay | `biology::parasitology::immunity` |
| `immune_boosting` | `fn immune_boosting(current_immunity: f64, boost_amount: f64, max_immunity: f64) → f64` | Immune boosting from re-exposure | `biology::parasitology::immunity` |
| `threshold_immunity` | `fn threshold_immunity(immunity_level: f64, threshold: f64) → f64` | Threshold immunity (binary) | `biology::parasitology::immunity` |
| `age_dependent_immunity` | `fn age_dependent_immunity(age: f64, exposure_rate: f64, max_immunity: f64, maturation_age: f64) → f64` | Age-dependent immunity | `biology::parasitology::immunity` |
| `strain_specific_immunity` | `fn strain_specific_immunity(immunity_strain_a: f64, cross_react: f64) → f64` | Strain-specific immunity | `biology::parasitology::immunity` |
| `immunity_waning_rate` | `fn immunity_waning_rate(peak_immunity: f64, current_immunity: f64, time: f64) → f64` | Immunity waning rate | `biology::parasitology::immunity` |

#### virulence.rs (20 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `virulence_transmission_tradeoff` | `fn virulence_transmission_tradeoff(alpha: f64, beta_max: f64, c: f64) → f64` | $\beta(\alpha) = \beta_{\max} \alpha / (\alpha + c)$ | `biology::parasitology::virulence` |
| `optimal_virulence` | `fn optimal_virulence(beta_max: f64, c: f64, mu: f64, gamma: f64) → f64` | Optimal virulence level | `biology::parasitology::virulence` |
| `myxoma_virulence_evolution` | `fn myxoma_virulence_evolution(initial_virulence: f64, generations: usize, selection_coeff: f64, step: f64) → f64` | Myxoma virus virulence evolution | `biology::parasitology::virulence` |
| `virulence_evolution_superinfection` | `fn virulence_evolution_superinfection(alpha: f64, sigma: f64, beta: f64, mu: f64, gamma: f64) → f64` | Virulence evolution with superinfection | `biology::parasitology::virulence` |
| `host_mortality_virulence` | `fn host_mortality_virulence(baseline_mortality: f64, virulence: f64) → f64` | Host mortality from virulence | `biology::parasitology::virulence` |
| `prudent_parasite_virulence` | `fn prudent_parasite_virulence(transmission: f64, virulence: f64, host_lifespan: f64) → f64` | Prudent parasite R₀ | `biology::parasitology::virulence` |
| `short_sighted_virulence` | `fn short_sighted_virulence(within_host_advantage: f64, between_host_fitness: f64, relatedness: f64) → f64` | Short-sighted virulence | `biology::parasitology::virulence` |
| `coinfection_virulence` | `fn coinfection_virulence(alpha1: f64, alpha2: f64, competition: f64) → f64` | Coinfection virulence outcome | `biology::parasitology::virulence` |
| `kin_selection_virulence` | `fn kin_selection_virulence(relatedness: f64, virulence_solo: f64, virulence_mixed: f64) → f64` | Kin selection virulence | `biology::parasitology::virulence` |
| `within_host_virulence_dynamics` | `fn within_host_virulence_dynamics(parasite_load: f64, immune_response: f64, replication_rate: f64, clearance: f64) → f64` | Within-host virulence dynamics | `biology::parasitology::virulence` |
| `spatial_structure_virulence` | `fn spatial_structure_virulence(local_transmission: f64, global_transmission: f64, virulence: f64) → f64` | Spatial structure virulence | `biology::parasitology::virulence` |
| `vector_borne_virulence` | `fn vector_borne_virulence(bite_rate: f64, vector_competence: f64, virulence: f64, recovery: f64) → f64` | Vector-borne virulence R₀ | `biology::parasitology::virulence` |
| `sterility_virulence` | `fn sterility_virulence(fecundity_reduction: f64, burden: f64, host_tolerance: f64) → f64` | Sterility virulence | `biology::parasitology::virulence` |
| `chronic_infection_virulence` | `fn chronic_infection_virulence(damage_rate: f64, repair_rate: f64, duration: f64) → f64` | Chronic infection cumulative damage | `biology::parasitology::virulence` |
| `acute_infection_virulence` | `fn acute_infection_virulence(peak_load: f64, immune_threshold: f64, replication: f64) → f64` | Acute infection virulence | `biology::parasitology::virulence` |
| `dose_dependent_virulence` | `fn dose_dependent_virulence(dose: f64, id50: f64, slope: f64) → f64` | Dose-dependent virulence | `biology::parasitology::virulence` |
| `environmental_transmission_virulence` | `fn environmental_transmission_virulence(shedding_rate: f64, survival_outside: f64, virulence: f64) → f64` | Environmental transmission virulence | `biology::parasitology::virulence` |
| `multiple_infection_virulence` | `fn multiple_infection_virulence(n_strains: f64, mean_virulence: f64, competition_intensity: f64) → f64` | Multiple infection virulence | `biology::parasitology::virulence` |
| `virulence_resistance_coevolution` | `fn virulence_resistance_coevolution(virulence: f64, resistance: f64, cost_v: f64, cost_r: f64) → (f64, f64)` | Virulence-resistance coevolution | `biology::parasitology::virulence` |
| `host_range_virulence` | `fn host_range_virulence(generalism: f64, specialist_virulence: f64, host_similarity: f64) → f64` | Host range virulence tradeoff | `biology::parasitology::virulence` |

---

### 2️⃣9️⃣ mycology/ — Mycology — 53 pub fn

#### decomposition.rs (17 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `cellulose_decomposition_rate` | `fn cellulose_decomposition_rate(enzyme_activity: f64, substrate: f64, km: f64, temperature_factor: f64) → f64` | Cellulose decomposition rate | `biology::mycology::decomposition` |
| `lignin_degradation_rate` | `fn lignin_degradation_rate(peroxidase: f64, h2o2: f64, lignin: f64, km: f64) → f64` | Lignin degradation rate | `biology::mycology::decomposition` |
| `white_rot_efficiency` | `fn white_rot_efficiency(lignin_removed: f64, cellulose_removed: f64) → f64` | White rot selectivity | `biology::mycology::decomposition` |
| `brown_rot_selectivity` | `fn brown_rot_selectivity(cellulose_removed: f64, lignin_removed: f64) → f64` | Brown rot selectivity | `biology::mycology::decomposition` |
| `soft_rot_rate` | `fn soft_rot_rate(moisture: f64, temperature: f64, substrate: f64, optimal_moisture: f64, optimal_temp: f64) → f64` | Soft rot rate | `biology::mycology::decomposition` |
| `enzymatic_hydrolysis_rate` | `fn enzymatic_hydrolysis_rate(enzyme: f64, substrate: f64, km: f64, vmax: f64) → f64` | Enzymatic hydrolysis (Michaelis-Menten) | `biology::mycology::decomposition` |
| `lignocellulose_mass_loss` | `fn lignocellulose_mass_loss(initial_mass: f64, k: f64, t: f64) → f64` | $m(t) = m_0 e^{-kt}$ mass loss | `biology::mycology::decomposition` |
| `fungal_respiration_rate` | `fn fungal_respiration_rate(biomass: f64, specific_rate: f64, temperature: f64, q10: f64, t_ref: f64) → f64` | Fungal respiration rate | `biology::mycology::decomposition` |
| `carbon_mineralization` | `fn carbon_mineralization(organic_c: f64, decomposition_rate: f64, efficiency: f64) → f64` | Carbon mineralization rate | `biology::mycology::decomposition` |
| `nutrient_release_rate` | `fn nutrient_release_rate(initial_nutrient: f64, cn_ratio_substrate: f64, cn_ratio_fungal: f64, decomp_rate: f64) → f64` | Nutrient release/immobilization | `biology::mycology::decomposition` |
| `decomposition_temperature_response` | `fn decomposition_temperature_response(rate_ref: f64, temperature: f64, t_ref: f64, q10: f64) → f64` | $Q_{10}$ temperature response | `biology::mycology::decomposition` |
| `moisture_decomposition_response` | `fn moisture_decomposition_response(moisture: f64, optimal: f64, width: f64) → f64` | Moisture response function | `biology::mycology::decomposition` |
| `substrate_quality_index` | `fn substrate_quality_index(nitrogen: f64, lignin: f64, cellulose: f64) → f64` | Substrate quality index | `biology::mycology::decomposition` |
| `decomposition_constant` | `fn decomposition_constant(mass_remaining_fraction: f64, time: f64) → f64` | $k = -\ln(m_t/m_0)/t$ | `biology::mycology::decomposition` |
| `fungal_biomass_carbon_ratio` | `fn fungal_biomass_carbon_ratio(ergosterol: f64, conversion_factor: f64) → f64` | Fungal biomass-C from ergosterol | `biology::mycology::decomposition` |
| `extracellular_enzyme_kinetics` | `fn extracellular_enzyme_kinetics(vmax: f64, substrate: f64, km: f64, inhibitor: f64, ki: f64) → f64` | Competitive inhibition kinetics | `biology::mycology::decomposition` |
| `humus_formation_rate` | `fn humus_formation_rate(fungal_biomass: f64, humification_coeff: f64, turnover_rate: f64) → f64` | Humus formation rate | `biology::mycology::decomposition` |

#### ecology.rs (8 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `species_richness_estimator` | `fn species_richness_estimator(observed: f64, singletons: f64, doubletons: f64) → f64` | Chao1 richness estimator | `biology::mycology::ecology` |
| `fungal_diversity_index` | `fn fungal_diversity_index(abundances: &[f64]) → f64` | Shannon diversity index | `biology::mycology::ecology` |
| `community_similarity` | `fn community_similarity(shared: f64, unique_a: f64, unique_b: f64) → f64` | Jaccard similarity | `biology::mycology::ecology` |
| `dominance_index` | `fn dominance_index(abundances: &[f64]) → f64` | Simpson dominance index | `biology::mycology::ecology` |
| `species_accumulation` | `fn species_accumulation(total_species: f64, samples: f64, rate: f64) → f64` | Species accumulation curve | `biology::mycology::ecology` |
| `beta_diversity` | `fn beta_diversity(gamma: f64, alpha_mean: f64) → f64` | $\beta = \gamma / \bar{\alpha}$ | `biology::mycology::ecology` |
| `fungal_biomass_to_bacterial_ratio` | `fn fungal_biomass_to_bacterial_ratio(fungal_biomass: f64, bacterial_biomass: f64) → f64` | F:B ratio | `biology::mycology::ecology` |
| `mycelial_network_connectivity` | `fn mycelial_network_connectivity(nodes: f64, edges: f64) → f64` | Network connectivity | `biology::mycology::ecology` |

#### growth.rs (19 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `hyphal_extension_rate` | `fn hyphal_extension_rate(vesicle_supply: f64, wall_extensibility: f64, turgor: f64, yield_threshold: f64) → f64` | Hyphal tip extension rate | `biology::mycology::growth` |
| `colony_radial_growth` | `fn colony_radial_growth(kr: f64, t: f64, lag: f64) → f64` | Colony radial growth rate | `biology::mycology::growth` |
| `branching_frequency` | `fn branching_frequency(tip_density: f64, branch_rate: f64, inhibition_zone: f64) → f64` | Hyphal branching frequency | `biology::mycology::growth` |
| `spore_germination_rate` | `fn spore_germination_rate(temperature: f64, water_activity: f64, t_opt: f64, aw_min: f64) → f64` | Spore germination rate | `biology::mycology::growth` |
| `biomass_yield` | `fn biomass_yield(substrate_consumed: f64, yield_coeff: f64, maintenance: f64, time: f64) → f64` | Biomass yield from substrate | `biology::mycology::growth` |
| `pellet_growth_model` | `fn pellet_growth_model(radius: f64, mu: f64, rho: f64, diffusion_limit: f64) → f64` | Pellet growth model | `biology::mycology::growth` |
| `tip_growth_vesicle_supply` | `fn tip_growth_vesicle_supply(vesicle_flux: f64, tip_area: f64, incorporation_rate: f64) → f64` | Vesicle supply center (VSC) model | `biology::mycology::growth` |
| `mycelial_density` | `fn mycelial_density(total_length: f64, area: f64) → f64` | Mycelial density | `biology::mycology::growth` |
| `hyphal_fusion_frequency` | `fn hyphal_fusion_frequency(density: f64, compatibility: f64, encounter_rate: f64) → f64` | Hyphal fusion (anastomosis) frequency | `biology::mycology::growth` |
| `conidiation_rate` | `fn conidiation_rate(light: f64, nutrient_stress: f64, maturity: f64) → f64` | Conidiation (spore formation) rate | `biology::mycology::growth` |
| `sclerotia_formation` | `fn sclerotia_formation(nutrient_depletion: f64, stress_signal: f64, threshold: f64) → f64` | Sclerotia formation probability | `biology::mycology::growth` |
| `dimorphic_switch_probability` | `fn dimorphic_switch_probability(temperature: f64, co2: f64, ph: f64, switch_threshold: f64) → f64` | Yeast-hyphal dimorphic switch | `biology::mycology::growth` |
| `growth_temperature_cardinal` | `fn growth_temperature_cardinal(t: f64, t_min: f64, t_opt: f64, t_max: f64) → f64` | Cardinal temperature model | `biology::mycology::growth` |
| `water_activity_growth` | `fn water_activity_growth(aw: f64, aw_min: f64, aw_opt: f64) → f64` | Water activity growth response | `biology::mycology::growth` |
| `substrate_inhibition_growth` | `fn substrate_inhibition_growth(s: f64, mu_max: f64, ks: f64, ki: f64) → f64` | $\mu = \mu_{\max} S / (K_s + S + S^2/K_i)$ | `biology::mycology::growth` |
| `oxygen_uptake_rate` | `fn oxygen_uptake_rate(biomass: f64, o2: f64, ko2: f64, qo2_max: f64) → f64` | Oxygen uptake rate | `biology::mycology::growth` |
| `co2_production_rate` | `fn co2_production_rate(o2_uptake: f64, rq: f64) → f64` | $CO_2 = O_2 \times RQ$ | `biology::mycology::growth` |
| `growth_ph_response` | `fn growth_ph_response(ph: f64, ph_min: f64, ph_opt: f64, ph_max: f64) → f64` | pH growth response | `biology::mycology::growth` |
| `antibiotic_production_rate` | `fn antibiotic_production_rate(biomass: f64, specific_production: f64, induction_signal: f64, threshold: f64) → f64` | Secondary metabolite production | `biology::mycology::growth` |

#### symbiosis.rs (9 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `mycorrhizal_colonization_rate` | `fn mycorrhizal_colonization_rate(inoculum: f64, root_density: f64, compatibility: f64, soil_p: f64) → f64` | Mycorrhizal colonization rate | `biology::mycology::symbiosis` |
| `nutrient_exchange_efficiency` | `fn nutrient_exchange_efficiency(delivered: f64, received: f64, transport_cost: f64) → f64` | Nutrient exchange efficiency | `biology::mycology::symbiosis` |
| `carbon_cost_mycorrhiza` | `fn carbon_cost_mycorrhiza(c_transferred: f64, photosynthate_total: f64) → f64` | Carbon cost to plant | `biology::mycology::symbiosis` |
| `phosphorus_uptake_enhancement` | `fn phosphorus_uptake_enhancement(uptake_mycorrhizal: f64, uptake_nonmycorrhizal: f64) → f64` | P uptake enhancement ratio | `biology::mycology::symbiosis` |
| `ectomycorrhizal_network_transport` | `fn ectomycorrhizal_network_transport(source_conc: f64, sink_conc: f64, conductance: f64, distance: f64) → f64` | CMN nutrient transport | `biology::mycology::symbiosis` |
| `arbuscular_mycorrhiza_benefit` | `fn arbuscular_mycorrhiza_benefit(p_uptake: f64, water_uptake: f64, c_cost: f64) → f64` | AM net benefit | `biology::mycology::symbiosis` |
| `endophyte_fitness_effect` | `fn endophyte_fitness_effect(growth_rate: f64, herbivory_reduction: f64, metabolic_cost: f64) → f64` | Endophyte fitness effect | `biology::mycology::symbiosis` |
| `lichen_growth_rate` | `fn lichen_growth_rate(light: f64, moisture: f64, temperature: f64, optimal_moisture: f64, optimal_temp: f64) → f64` | Lichen growth rate | `biology::mycology::symbiosis` |
| `orchid_mycorrhiza_dependency` | `fn orchid_mycorrhiza_dependency(seed_germination_myc: f64, seed_germination_control: f64) → f64` | Orchid mycorrhizal dependency | `biology::mycology::symbiosis` |

---

### 3️⃣0️⃣ evolution/ — Evolution — 46 pub fn

#### adaptation.rs (10 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `adaptation_rate` | `fn adaptation_rate(selection_coefficient: f64, mutation_rate: f64, population_size: f64) → f64` | Adaptation rate | `biology::evolution::adaptation` |
| `fisher_geometric_adaptation` | `fn fisher_geometric_adaptation(phenotype: &[f64], optimum: &[f64]) → f64` | Fisher geometric adaptation distance | `biology::evolution::adaptation` |
| `adaptive_walk_progress` | `fn adaptive_walk_progress(current_fitness: f64, max_fitness: f64, beneficial_rate: f64, step: usize) → f64` | Adaptive walk progress | `biology::evolution::adaptation` |
| `beneficial_mutation_fixation_probability` | `fn beneficial_mutation_fixation_probability(s: f64, ne: f64) → f64` | $P_{\text{fix}} \approx 2s$ (Haldane) | `biology::evolution::adaptation` |
| `phenotypic_plasticity` | `fn phenotypic_plasticity(genotype_value: f64, environment: f64, reaction_norm_slope: f64) → f64` | Reaction norm phenotypic plasticity | `biology::evolution::adaptation` |
| `baldwin_effect` | `fn baldwin_effect(learning_rate: f64, genetic_assimilation: f64, generations: usize) → f64` | Baldwin effect | `biology::evolution::adaptation` |
| `red_queen_coevolution` | `fn red_queen_coevolution(host_fitness: f64, parasite_fitness: f64, host_adapt_rate: f64, parasite_adapt_rate: f64, dt: f64) → (f64, f64)` | Red Queen coevolution dynamics | `biology::evolution::adaptation` |
| `environmental_gradient_selection` | `fn environmental_gradient_selection(position: f64, optimum_slope: f64, selection_width: f64, phenotype: f64) → f64` | Selection along environmental gradient | `biology::evolution::adaptation` |
| `frequency_dependent_selection` | `fn frequency_dependent_selection(frequency: f64, baseline_fitness: f64, fd_coefficient: f64) → f64` | Frequency-dependent selection | `biology::evolution::adaptation` |
| `adaptive_radiation_rate` | `fn adaptive_radiation_rate(niche_count: usize, occupied: usize, diversification_rate: f64) → f64` | Adaptive radiation rate | `biology::evolution::adaptation` |

#### fitness.rs (10 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `fitness_landscape_nk` | `fn fitness_landscape_nk(genotype: &[u8], k: usize, contributions: &[Vec<f64>]) → f64` | NK fitness landscape | `biology::evolution::fitness` |
| `fitness_landscape_additive` | `fn fitness_landscape_additive(genotype: &[u8], effects: &[f64]) → f64` | Additive fitness landscape | `biology::evolution::fitness` |
| `fisher_geometric_model` | `fn fisher_geometric_model(distance: f64, n_dimensions: usize) → f64` | Fisher's geometric model | `biology::evolution::fitness` |
| `mutation_step_probability` | `fn mutation_step_probability(distance: f64, step_size: f64, n_dim: usize) → f64` | Mutation step beneficial probability | `biology::evolution::fitness` |
| `adaptive_walk` | `fn adaptive_walk(distance0: f64, step_size: f64, n_dim: usize, max_steps: usize) → Vec<f64>` | Adaptive walk simulation | `biology::evolution::fitness` |
| `epistasis` | `fn epistasis(w_ab: f64, w_a: f64, w_b: f64, w_ref: f64) → f64` | Epistasis coefficient | `biology::evolution::fitness` |
| `frequency_dependent_fitness` | `fn frequency_dependent_fitness(freq: f64, advantage_rare: f64) → f64` | Frequency-dependent fitness | `biology::evolution::fitness` |
| `density_dependent_fitness` | `fn density_dependent_fitness(population: f64, carrying_capacity: f64, r_max: f64) → f64` | Density-dependent fitness | `biology::evolution::fitness` |
| `wrightian_fitness` | `fn wrightian_fitness(genotype: &[bool], loci_effects: &[f64], dominance: &[f64]) → f64` | Wrightian fitness (multiplicative) | `biology::evolution::fitness` |
| `fitness_landscape_rugged` | `fn fitness_landscape_rugged(genotype: &[u8], peaks: &[(&[u8], f64)]) → f64` | Rugged fitness landscape | `biology::evolution::fitness` |

#### molecular.rs (7 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `substitution_rate` | `fn substitution_rate(mu: f64, ne: f64, s: f64) → f64` | Substitution rate | `biology::evolution::molecular` |
| `dn_ds_ratio` | `fn dn_ds_ratio(dn: f64, ds: f64) → f64` | $dN/dS$ ratio | `biology::evolution::molecular` |
| `molecular_clock_rate` | `fn molecular_clock_rate(substitutions: f64, divergence_time: f64) → f64` | Molecular clock rate | `biology::evolution::molecular` |
| `coalescent_time_pair` | `fn coalescent_time_pair(ne: f64) → f64` | $E[T_2] = N_e$ coalescent time | `biology::evolution::molecular` |
| `expected_segregating_sites` | `fn expected_segregating_sites(theta: f64, n: usize) → f64` | Expected segregating sites | `biology::evolution::molecular` |
| `watterson_estimator` | `fn watterson_estimator(seg_sites: usize, n: usize) → f64` | Watterson's $\hat{\theta}$ | `biology::evolution::molecular` |
| `mcdonald_kreitman` | `fn mcdonald_kreitman(dn: f64, ds: f64, pn: f64, ps: f64) → f64` | McDonald-Kreitman test | `biology::evolution::molecular` |

#### neutral_theory.rs (11 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `neutral_substitution_rate` | `fn neutral_substitution_rate(mutation_rate: f64) → f64` | $k = \mu$ neutral substitution rate | `biology::evolution::neutral_theory` |
| `effective_neutral_mutations` | `fn effective_neutral_mutations(total_mutations: usize, fraction_neutral: f64) → f64` | Effective neutral mutations | `biology::evolution::neutral_theory` |
| `nearly_neutral_boundary` | `fn nearly_neutral_boundary(ne: f64) → f64` | $|s| < 1/N_e$ boundary | `biology::evolution::neutral_theory` |
| `tajima_d` | `fn tajima_d(pi: f64, theta_w: f64, n: usize) → f64` | Tajima's D statistic | `biology::evolution::neutral_theory` |
| `watterson_theta` | `fn watterson_theta(segregating_sites: usize, n: usize, sequence_length: usize) → f64` | Watterson's $\theta$ per site | `biology::evolution::neutral_theory` |
| `nucleotide_diversity` | `fn nucleotide_diversity(differences: &[f64], n_sequences: usize) → f64` | $\pi$ nucleotide diversity | `biology::evolution::neutral_theory` |
| `ewens_sampling_formula` | `fn ewens_sampling_formula(n: usize, theta: f64) → f64` | Ewens sampling formula | `biology::evolution::neutral_theory` |
| `coalescent_expected_time` | `fn coalescent_expected_time(n: usize, ne: f64) → f64` | Coalescent expected time | `biology::evolution::neutral_theory` |
| `mcdonald_kreitman_ratio` | `fn mcdonald_kreitman_ratio(dn: f64, ds: f64, pn: f64, ps: f64) → f64` | MK ratio | `biology::evolution::neutral_theory` |
| `neutrality_index` | `fn neutrality_index(dn: f64, ds: f64, pn: f64, ps: f64) → f64` | Neutrality index (NI) | `biology::evolution::neutral_theory` |
| `direction_of_selection` | `fn direction_of_selection(ni: f64) → f64` | Direction of selection from NI | `biology::evolution::neutral_theory` |

#### speciation.rs (8 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `speciation_rate_bdi` | `fn speciation_rate_bdi(lambda: f64, mu: f64, t: f64, n0: f64) → f64` | Birth-death-immigration speciation | `biology::evolution::speciation` |
| `allopatric_divergence` | `fn allopatric_divergence(d0: f64, mu: f64, t: f64) → f64` | Allopatric divergence | `biology::evolution::speciation` |
| `reproductive_isolation` | `fn reproductive_isolation(genetic_distance: f64, k: f64, n: f64) → f64` | Reproductive isolation (sigmoidal) | `biology::evolution::speciation` |
| `reinforcement_strength` | `fn reinforcement_strength(sympatry: f64, hybrid_fitness: f64) → f64` | Reinforcement strength | `biology::evolution::speciation` |
| `yule_process_expected_species` | `fn yule_process_expected_species(lambda: f64, t: f64) → f64` | Yule process expected species | `biology::evolution::speciation` |
| `birth_death_survival` | `fn birth_death_survival(lambda: f64, mu: f64, t: f64) → f64` | Birth-death survival probability | `biology::evolution::speciation` |
| `character_displacement` | `fn character_displacement(z1: f64, z2: f64, alpha: f64, sigma: f64) → (f64, f64)` | Character displacement | `biology::evolution::speciation` |
| `ecological_speciation_fitness` | `fn ecological_speciation_fitness(trait_val: f64, optimum1: f64, optimum2: f64, sigma: f64) → (f64, f64)` | Ecological speciation fitness | `biology::evolution::speciation` |

---

### 3️⃣1️⃣ biophysics/ — Biophysics — 79 pub fn

#### membrane.rs (9 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `membrane_bending_energy` | `fn membrane_bending_energy(kappa: f64, curvature: f64, spontaneous_curvature: f64) → f64` | $E = \frac{\kappa}{2}(c - c_0)^2$ bending energy | `biology::biophysics::membrane` |
| `helfrich_energy` | `fn helfrich_energy(kappa: f64, kappa_bar: f64, c1: f64, c2: f64, c0: f64) → f64` | Helfrich membrane energy | `biology::biophysics::membrane` |
| `membrane_tension` | `fn membrane_tension(area_strain: f64, stretch_modulus: f64) → f64` | Membrane tension | `biology::biophysics::membrane` |
| `lipid_diffusion_saffman_delbruck` | `fn lipid_diffusion_saffman_delbruck(viscosity_membrane: f64, viscosity_water: f64, membrane_thickness: f64, radius: f64, t: f64) → f64` | Saffman-Delbrück diffusion | `biology::biophysics::membrane` |
| `osmotic_lysis_threshold` | `fn osmotic_lysis_threshold(internal_osmolarity: f64, membrane_tension_max: f64, radius: f64) → f64` | Osmotic lysis threshold | `biology::biophysics::membrane` |
| `vesicle_budding_energy` | `fn vesicle_budding_energy(kappa: f64, radius: f64) → f64` | Vesicle budding energy $8\pi\kappa$ | `biology::biophysics::membrane` |
| `flip_flop_rate` | `fn flip_flop_rate(activation_energy: f64, t: f64) → f64` | Lipid flip-flop rate | `biology::biophysics::membrane` |
| `lateral_pressure_profile` | `fn lateral_pressure_profile(depth: f64, head_pressure: f64, tail_pressure: f64, thickness: f64) → f64` | Lateral pressure profile | `biology::biophysics::membrane` |
| `line_tension_domain` | `fn line_tension_domain(length: f64, lambda: f64) → f64` | Line tension energy | `biology::biophysics::membrane` |

#### molecular_dynamics.rs (20 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `lennard_jones` | `fn lennard_jones(r: f64, epsilon: f64, sigma: f64) → f64` | $V = 4\epsilon[(\sigma/r)^{12} - (\sigma/r)^6]$ | `biology::biophysics::molecular_dynamics` |
| `lennard_jones_force` | `fn lennard_jones_force(r: f64, epsilon: f64, sigma: f64) → f64` | LJ force | `biology::biophysics::molecular_dynamics` |
| `coulomb_interaction` | `fn coulomb_interaction(q1: f64, q2: f64, r: f64, epsilon_r: f64) → f64` | Coulomb interaction | `biology::biophysics::molecular_dynamics` |
| `debye_huckel` | `fn debye_huckel(q: f64, r: f64, kappa: f64, epsilon_r: f64) → f64` | Debye-Hückel screened potential | `biology::biophysics::molecular_dynamics` |
| `verlet_step` | `fn verlet_step(positions: &mut [f64], velocities: &mut [f64], forces: &[f64], masses: &[f64], dt: f64)` | Verlet integration step | `biology::biophysics::molecular_dynamics` |
| `kinetic_energy` | `fn kinetic_energy(velocities: &[f64], masses: &[f64]) → f64` | $KE = \frac{1}{2}\sum m_i v_i^2$ | `biology::biophysics::molecular_dynamics` |
| `temperature_from_ke` | `fn temperature_from_ke(ke: f64, n_particles: usize, n_dim: usize) → f64` | Temperature from kinetic energy | `biology::biophysics::molecular_dynamics` |
| `morse_potential` | `fn morse_potential(r: f64, d_e: f64, a: f64, r_e: f64) → f64` | $V = D_e(1 - e^{-a(r-r_e)})^2$ | `biology::biophysics::molecular_dynamics` |
| `harmonic_bond` | `fn harmonic_bond(r: f64, k: f64, r0: f64) → f64` | $V = \frac{k}{2}(r - r_0)^2$ | `biology::biophysics::molecular_dynamics` |
| `harmonic_angle` | `fn harmonic_angle(theta: f64, k: f64, theta0: f64) → f64` | Harmonic angle potential | `biology::biophysics::molecular_dynamics` |
| `dihedral_potential` | `fn dihedral_potential(phi: f64, k: f64, n: f64, delta: f64) → f64` | Dihedral angle potential | `biology::biophysics::molecular_dynamics` |
| `velocity_verlet_step` | `fn velocity_verlet_step(positions: &mut [f64], velocities: &mut [f64], forces_old: &[f64], forces_new: &[f64], masses: &[f64], dt: f64)` | Velocity Verlet step | `biology::biophysics::molecular_dynamics` |
| `berendsen_thermostat` | `fn berendsen_thermostat(velocities: &mut [f64], current_temp: f64, target_temp: f64, tau: f64, dt: f64)` | Berendsen thermostat | `biology::biophysics::molecular_dynamics` |
| `nose_hoover_friction` | `fn nose_hoover_friction(ke: f64, target_ke: f64, q: f64) → f64` | Nosé-Hoover friction | `biology::biophysics::molecular_dynamics` |
| `switching_function` | `fn switching_function(r: f64, r_on: f64, r_off: f64) → f64` | Switching function | `biology::biophysics::molecular_dynamics` |
| `pair_correlation_bin` | `fn pair_correlation_bin(distances: &[f64], r_min: f64, r_max: f64, n_particles: usize, volume: f64) → f64` | Pair correlation function bin | `biology::biophysics::molecular_dynamics` |
| `pressure_virial` | `fn pressure_virial(n: usize, volume: f64, temperature: f64, virial_sum: f64) → f64` | Virial pressure | `biology::biophysics::molecular_dynamics` |
| `mean_free_path` | `fn mean_free_path(density: f64, cross_section: f64) → f64` | $\lambda = 1/(n\sigma)$ | `biology::biophysics::molecular_dynamics` |
| `born_mayer_repulsion` | `fn born_mayer_repulsion(a: f64, b: f64, r: f64) → f64` | Born-Mayer repulsion | `biology::biophysics::molecular_dynamics` |
| `buckingham_potential` | `fn buckingham_potential(a: f64, b: f64, c: f64, r: f64) → f64` | Buckingham potential | `biology::biophysics::molecular_dynamics` |

#### polymers.rs (21 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `worm_like_chain` | `fn worm_like_chain(l: f64, lp: f64, lc: f64) → f64` | WLC force-extension | `biology::biophysics::polymers` |
| `freely_jointed_chain` | `fn freely_jointed_chain(l: f64, n: usize, b: f64) → f64` | FJC force-extension | `biology::biophysics::polymers` |
| `end_to_end_distance_rms` | `fn end_to_end_distance_rms(n: usize, b: f64) → f64` | $\langle R^2 \rangle^{1/2} = b\sqrt{N}$ | `biology::biophysics::polymers` |
| `radius_of_gyration` | `fn radius_of_gyration(n: usize, b: f64) → f64` | $R_g = b\sqrt{N/6}$ | `biology::biophysics::polymers` |
| `persistence_length_from_tangent` | `fn persistence_length_from_tangent(cos_theta: f64, segment_length: f64) → f64` | Persistence length from tangent correlation | `biology::biophysics::polymers` |
| `kratky_porod_energy` | `fn kratky_porod_energy(kappa: f64, ds: f64, curvature: f64) → f64` | Kratky-Porod bending energy | `biology::biophysics::polymers` |
| `dna_twist_energy` | `fn dna_twist_energy(c_twist: f64, delta_twist: f64, length: f64) → f64` | DNA twist energy | `biology::biophysics::polymers` |
| `stokes_einstein_diffusion` | `fn stokes_einstein_diffusion(t: f64, viscosity: f64, radius: f64) → f64` | $D = k_BT / (6\pi\eta r)$ | `biology::biophysics::polymers` |
| `mean_squared_displacement` | `fn mean_squared_displacement(d: f64, t: f64, n_dim: usize) → f64` | $\langle r^2\rangle = 2nDt$ | `biology::biophysics::polymers` |
| `sedimentation_coefficient` | `fn sedimentation_coefficient(mass: f64, partial_specific_vol: f64, rho_solvent: f64, friction: f64) → f64` | Sedimentation coefficient | `biology::biophysics::polymers` |
| `flory_radius` | `fn flory_radius(n: usize, b: f64, nu: f64) → f64` | $R_F = bN^\nu$ Flory radius | `biology::biophysics::polymers` |
| `kuhn_length` | `fn kuhn_length(persistence_length: f64) → f64` | $b = 2l_p$ Kuhn length | `biology::biophysics::polymers` |
| `contour_length` | `fn contour_length(n: usize, b: f64) → f64` | $L_c = Nb$ contour length | `biology::biophysics::polymers` |
| `extensible_wlc` | `fn extensible_wlc(force: f64, lp: f64, lc: f64, stretch_modulus: f64, t: f64) → f64` | Extensible WLC | `biology::biophysics::polymers` |
| `odijk_deflection_length` | `fn odijk_deflection_length(lp: f64, d: f64) → f64` | Odijk deflection length | `biology::biophysics::polymers` |
| `blob_size` | `fn blob_size(kbt: f64, force: f64) → f64` | Pincus blob size $\xi = k_BT/f$ | `biology::biophysics::polymers` |
| `zimm_relaxation_time` | `fn zimm_relaxation_time(viscosity: f64, rg: f64, kbt: f64) → f64` | Zimm relaxation time | `biology::biophysics::polymers` |
| `rouse_relaxation_time` | `fn rouse_relaxation_time(friction: f64, n: usize, b: f64, kbt: f64) → f64` | Rouse relaxation time | `biology::biophysics::polymers` |
| `intrinsic_viscosity` | `fn intrinsic_viscosity(rg: f64, mw: f64) → f64` | Intrinsic viscosity | `biology::biophysics::polymers` |
| `overlap_concentration` | `fn overlap_concentration(mw: f64, rg: f64) → f64` | Overlap concentration $c^*$ | `biology::biophysics::polymers` |
| `debye_scattering` | `fn debye_scattering(q: f64, rg: f64) → f64` | Debye scattering function | `biology::biophysics::polymers` |

#### protein.rs (17 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `ramachandran_energy` | `fn ramachandran_energy(phi: f64, psi: f64) → f64` | Ramachandran energy | `biology::biophysics::protein` |
| `hydrophobic_free_energy` | `fn hydrophobic_free_energy(sasa_nonpolar: f64, gamma: f64) → f64` | Hydrophobic free energy | `biology::biophysics::protein` |
| `hydrogen_bond_energy` | `fn hydrogen_bond_energy(r: f64, theta: f64, epsilon: f64, r0: f64) → f64` | H-bond energy | `biology::biophysics::protein` |
| `electrostatic_solvation` | `fn electrostatic_solvation(charge: f64, radius: f64, epsilon_solvent: f64) → f64` | Born solvation energy | `biology::biophysics::protein` |
| `fold_stability` | `fn fold_stability(delta_h: f64, delta_s: f64, delta_cp: f64, t: f64, t_ref: f64) → f64` | Protein fold stability $\Delta G(T)$ | `biology::biophysics::protein` |
| `fraction_folded` | `fn fraction_folded(delta_g: f64, t: f64) → f64` | $f = 1/(1 + e^{\Delta G/RT})$ | `biology::biophysics::protein` |
| `two_state_folding_rate` | `fn two_state_folding_rate(k0: f64, delta_g_dagger: f64, t: f64) → f64` | Two-state folding rate | `biology::biophysics::protein` |
| `zimm_bragg_helix_coil` | `fn zimm_bragg_helix_coil(s: f64, sigma: f64, n: usize) → f64` | Zimm-Bragg helix-coil transition | `biology::biophysics::protein` |
| `contact_order` | `fn contact_order(contacts: &[(usize, usize)], chain_length: usize) → f64` | Relative contact order | `biology::biophysics::protein` |
| `phi_value` | `fn phi_value(delta_g_mut_folding: f64, delta_g_wt_folding: f64, delta_g_mut_ts: f64, delta_g_wt_ts: f64) → f64` | Phi value analysis | `biology::biophysics::protein` |
| `kauzmann_hydrophobic` | `fn kauzmann_hydrophobic(delta_cp: f64, t: f64, t_s: f64, t_h: f64, delta_h_h: f64) → f64` | Kauzmann hydrophobic model | `biology::biophysics::protein` |
| `go_model_energy` | `fn go_model_energy(contacts: &[(usize, usize)], distances: &[f64], native_distances: &[f64], epsilon: f64) → f64` | Gō model energy | `biology::biophysics::protein` |
| `native_contact_fraction` | `fn native_contact_fraction(current_distances: &[f64], native_distances: &[f64], cutoff: f64) → f64` | $Q$ native contact fraction | `biology::biophysics::protein` |
| `radius_of_gyration_3d` | `fn radius_of_gyration_3d(coords: &[(f64, f64, f64)]) → f64` | 3D radius of gyration | `biology::biophysics::protein` |
| `denaturation_midpoint` | `fn denaturation_midpoint(delta_h: f64, delta_s: f64) → f64` | $T_m = \Delta H / \Delta S$ | `biology::biophysics::protein` |
| `chevron_plot_folding` | `fn chevron_plot_folding(k_f_water: f64, m_f: f64, denaturant: f64) → f64` | Chevron plot folding arm | `biology::biophysics::protein` |
| `chevron_plot_unfolding` | `fn chevron_plot_unfolding(k_u_water: f64, m_u: f64, denaturant: f64) → f64` | Chevron plot unfolding arm | `biology::biophysics::protein` |

#### single_molecule.rs (12 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `optical_trap_force` | `fn optical_trap_force(laser_power: f64, n_medium: f64, trap_efficiency: f64) → f64` | Optical trap force | `biology::biophysics::single_molecule` |
| `fret_efficiency` | `fn fret_efficiency(r: f64, r0: f64) → f64` | $E = 1/(1 + (r/R_0)^6)$ FRET | `biology::biophysics::single_molecule` |
| `fret_distance` | `fn fret_distance(efficiency: f64, r0: f64) → f64` | FRET distance from efficiency | `biology::biophysics::single_molecule` |
| `fluorescence_lifetime` | `fn fluorescence_lifetime(quantum_yield: f64, radiative_rate: f64) → f64` | Fluorescence lifetime | `biology::biophysics::single_molecule` |
| `photobleaching_rate` | `fn photobleaching_rate(intensity: f64, cross_section: f64, quantum_yield_bleach: f64) → f64` | Photobleaching rate | `biology::biophysics::single_molecule` |
| `fluorescence_recovery_half_time` | `fn fluorescence_recovery_half_time(beam_radius: f64, diffusion_coeff: f64) → f64` | FRAP half-time | `biology::biophysics::single_molecule` |
| `single_molecule_diffusion_msd` | `fn single_molecule_diffusion_msd(d: f64, t: f64, localization_error: f64) → f64` | Single-molecule MSD | `biology::biophysics::single_molecule` |
| `afm_cantilever_force` | `fn afm_cantilever_force(spring_constant: f64, deflection: f64) → f64` | $F = kd$ AFM cantilever | `biology::biophysics::single_molecule` |
| `hertz_contact_indentation` | `fn hertz_contact_indentation(force: f64, radius: f64, youngs_modulus: f64, poisson: f64) → f64` | Hertz contact model | `biology::biophysics::single_molecule` |
| `micropipette_aspiration_tension` | `fn micropipette_aspiration_tension(pressure: f64, pipette_radius: f64) → f64` | Micropipette aspiration tension | `biology::biophysics::single_molecule` |
| `youngs_modulus_from_hertz` | `fn youngs_modulus_from_hertz(force: f64, indentation: f64, tip_radius: f64, poisson: f64) → f64` | Young's modulus from Hertz | `biology::biophysics::single_molecule` |
| `traction_force` | `fn traction_force(displacement: f64, substrate_stiffness: f64) → f64` | Traction force microscopy | `biology::biophysics::single_molecule` |

---

### 3️⃣2️⃣ biomechanics/ — Biomechanics — 67 pub fn

#### fluids.rs (19 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `poiseuille_flow` | `fn poiseuille_flow(delta_p: f64, radius: f64, length: f64, viscosity: f64) → f64` | $Q = \pi r^4 \Delta P / (8\mu L)$ | `biology::biomechanics::fluids` |
| `wall_shear_stress` | `fn wall_shear_stress(flow_rate: f64, radius: f64, viscosity: f64) → f64` | Wall shear stress | `biology::biomechanics::fluids` |
| `reynolds_number` | `fn reynolds_number(density: f64, velocity: f64, diameter: f64, viscosity: f64) → f64` | $Re = \rho v D / \mu$ | `biology::biomechanics::fluids` |
| `murrays_law_radius` | `fn murrays_law_radius(parent_radius: f64, n_children: usize) → f64` | Murray's law branching | `biology::biomechanics::fluids` |
| `windkessel_2element` | `fn windkessel_2element(p0: f64, r: f64, c: f64, flow: impl Fn(f64) -> f64, dt: f64, steps: usize) → Vec<f64>` | 2-element Windkessel model | `biology::biomechanics::fluids` |
| `pulse_wave_velocity` | `fn pulse_wave_velocity(elastic_modulus: f64, wall_thickness: f64, radius: f64, density: f64) → f64` | Moens-Korteweg PWV | `biology::biomechanics::fluids` |
| `casson_viscosity` | `fn casson_viscosity(tau_y: f64, eta_inf: f64, shear_rate: f64) → f64` | Casson model viscosity | `biology::biomechanics::fluids` |
| `oxygen_dissociation_hill` | `fn oxygen_dissociation_hill(po2: f64, p50: f64, n: f64) → f64` | Hill oxygen dissociation | `biology::biomechanics::fluids` |
| `cardiac_output` | `fn cardiac_output(stroke_volume: f64, heart_rate: f64) → f64` | $CO = SV \times HR$ | `biology::biomechanics::fluids` |
| `mean_arterial_pressure` | `fn mean_arterial_pressure(systolic: f64, diastolic: f64) → f64` | $MAP = DBP + (SBP-DBP)/3$ | `biology::biomechanics::fluids` |
| `total_peripheral_resistance` | `fn total_peripheral_resistance(map: f64, cvp: f64, cardiac_output: f64) → f64` | TPR = (MAP-CVP)/CO | `biology::biomechanics::fluids` |
| `womersley_number` | `fn womersley_number(radius: f64, angular_freq: f64, kinematic_viscosity: f64) → f64` | Womersley number $\alpha$ | `biology::biomechanics::fluids` |
| `fahraeus_lindqvist` | `fn fahraeus_lindqvist(viscosity_plasma: f64, hematocrit: f64, diameter_um: f64) → f64` | Fåhraeus-Lindqvist effect | `biology::biomechanics::fluids` |
| `compliance` | `fn compliance(delta_v: f64, delta_p: f64) → f64` | $C = \Delta V / \Delta P$ | `biology::biomechanics::fluids` |
| `laplace_law_sphere` | `fn laplace_law_sphere(pressure: f64, radius: f64, wall_thickness: f64) → f64` | Laplace law (sphere) | `biology::biomechanics::fluids` |
| `laplace_law_cylinder` | `fn laplace_law_cylinder(pressure: f64, radius: f64, wall_thickness: f64) → f64` | Laplace law (cylinder) | `biology::biomechanics::fluids` |
| `bernoulli_velocity` | `fn bernoulli_velocity(delta_p: f64, density: f64) → f64` | Bernoulli velocity | `biology::biomechanics::fluids` |
| `systemic_vascular_resistance` | `fn systemic_vascular_resistance(map: f64, rap: f64, co: f64) → f64` | SVR | `biology::biomechanics::fluids` |
| `ejection_fraction` | `fn ejection_fraction(edv: f64, esv: f64) → f64` | $EF = (EDV - ESV)/EDV$ | `biology::biomechanics::fluids` |

#### locomotion.rs (11 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `gait_stride_length` | `fn gait_stride_length(velocity: f64, cadence: f64) → f64` | Stride length | `biology::biomechanics::locomotion` |
| `ground_reaction_force` | `fn ground_reaction_force(mass: f64, acceleration: f64) → f64` | GRF | `biology::biomechanics::locomotion` |
| `joint_moment` | `fn joint_moment(force: f64, moment_arm: f64) → f64` | Joint moment | `biology::biomechanics::locomotion` |
| `joint_power` | `fn joint_power(moment: f64, angular_velocity: f64) → f64` | $P = M\omega$ | `biology::biomechanics::locomotion` |
| `center_of_pressure` | `fn center_of_pressure(forces: &[(f64, f64, f64)], positions: &[(f64, f64)]) → (f64, f64)` | Center of pressure | `biology::biomechanics::locomotion` |
| `inverse_dynamics_moment` | `fn inverse_dynamics_moment(i_segment: f64, alpha: f64, proximal_force: f64, proximal_arm: f64, distal_force: f64, distal_arm: f64) → f64` | Inverse dynamics moment | `biology::biomechanics::locomotion` |
| `metabolic_cost_of_transport` | `fn metabolic_cost_of_transport(metabolic_rate: f64, mass: f64, velocity: f64) → f64` | Cost of transport | `biology::biomechanics::locomotion` |
| `froude_number` | `fn froude_number(velocity: f64, leg_length: f64) → f64` | $Fr = v^2/(gL)$ | `biology::biomechanics::locomotion` |
| `dynamic_stability_margin` | `fn dynamic_stability_margin(base_of_support: &[(f64, f64)], com: (f64, f64)) → f64` | Dynamic stability margin | `biology::biomechanics::locomotion` |
| `work_loop_area` | `fn work_loop_area(force: &[f64], length: &[f64]) → f64` | Muscle work loop area | `biology::biomechanics::locomotion` |
| `pendulum_energy_recovery` | `fn pendulum_energy_recovery(ek_change: f64, ep_change: f64) → f64` | Pendulum energy recovery | `biology::biomechanics::locomotion` |

#### muscle.rs (19 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `HillMuscle::new` | `fn new(f_max: f64, l_opt: f64, v_max: f64) → Self` | Hill muscle model constructor | `biology::biomechanics::muscle` |
| `HillMuscle::force_length` | `fn force_length(&self, length: f64) → f64` | Force-length relationship | `biology::biomechanics::muscle` |
| `HillMuscle::force_velocity` | `fn force_velocity(&self, velocity: f64) → f64` | Force-velocity relationship | `biology::biomechanics::muscle` |
| `HillMuscle::total_force` | `fn total_force(&self, length: f64, velocity: f64) → f64` | Total muscle force | `biology::biomechanics::muscle` |
| `cross_bridge_huxley` | `fn cross_bridge_huxley(x: f64, f_rate: f64, g_rate: f64, dt: f64, n: f64) → f64` | Huxley cross-bridge model | `biology::biomechanics::muscle` |
| `pennation_angle_force` | `fn pennation_angle_force(f_tendon: f64, angle_rad: f64) → f64` | Pennation angle force | `biology::biomechanics::muscle` |
| `joint_torque` | `fn joint_torque(force: f64, moment_arm: f64) → f64` | Joint torque | `biology::biomechanics::muscle` |
| `angular_impulse` | `fn angular_impulse(torque: f64, dt: f64) → f64` | Angular impulse | `biology::biomechanics::muscle` |
| `muscle_power` | `fn muscle_power(force: f64, velocity: f64) → f64` | $P = Fv$ | `biology::biomechanics::muscle` |
| `work` | `fn work(force: f64, displacement: f64) → f64` | $W = Fd$ | `biology::biomechanics::muscle` |
| `tendon_force` | `fn tendon_force(stiffness: f64, strain: f64) → f64` | Tendon force | `biology::biomechanics::muscle` |
| `excitation_contraction_coupling` | `fn excitation_contraction_coupling(calcium: f64, k_half: f64, n: f64) → f64` | E-C coupling (Hill) | `biology::biomechanics::muscle` |
| `fatigue_model` | `fn fatigue_model(force_max: f64, time: f64, fatigue_rate: f64) → f64` | Fatigue model | `biology::biomechanics::muscle` |
| `muscle_stiffness` | `fn muscle_stiffness(force: f64, length: f64, l_opt: f64) → f64` | Muscle stiffness | `biology::biomechanics::muscle` |
| `isometric_twitch` | `fn isometric_twitch(f_max: f64, t: f64, tp: f64) → f64` | Isometric twitch | `biology::biomechanics::muscle` |
| `tetanus_fusion` | `fn tetanus_fusion(f_twitch: f64, frequency: f64, fusion_freq: f64) → f64` | Tetanus fusion | `biology::biomechanics::muscle` |
| `muscle_metabolic_rate` | `fn muscle_metabolic_rate(force: f64, velocity: f64, activation: f64, basal: f64) → f64` | Muscle metabolic rate | `biology::biomechanics::muscle` |
| `fiber_type_recruitment` | `fn fiber_type_recruitment(excitation: f64, threshold_slow: f64, threshold_fast: f64) → (f64, f64)` | Fiber type recruitment | `biology::biomechanics::muscle` |
| `sarcomere_force_length` | `fn sarcomere_force_length(sl: f64) → f64` | Sarcomere force-length | `biology::biomechanics::muscle` |

#### tissue.rs (18 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `linear_elastic_stress` | `fn linear_elastic_stress(modulus: f64, strain: f64) → f64` | $\sigma = E\varepsilon$ | `biology::biomechanics::tissue` |
| `kelvin_voigt` | `fn kelvin_voigt(modulus: f64, viscosity: f64, strain: f64, strain_rate: f64) → f64` | Kelvin-Voigt model | `biology::biomechanics::tissue` |
| `maxwell_stress_relaxation` | `fn maxwell_stress_relaxation(sigma0: f64, modulus: f64, viscosity: f64, t: f64) → f64` | Maxwell stress relaxation | `biology::biomechanics::tissue` |
| `standard_linear_solid` | `fn standard_linear_solid(e1: f64, e2: f64, eta: f64, strain: f64, strain_rate: f64, stress: f64) → f64` | SLS model | `biology::biomechanics::tissue` |
| `hyperelastic_neo_hookean` | `fn hyperelastic_neo_hookean(c1: f64, lambda: f64) → f64` | Neo-Hookean hyperelastic | `biology::biomechanics::tissue` |
| `mooney_rivlin` | `fn mooney_rivlin(c1: f64, c2: f64, lambda: f64) → f64` | Mooney-Rivlin model | `biology::biomechanics::tissue` |
| `poroelastic_consolidation` | `fn poroelastic_consolidation(stress: f64, modulus: f64, permeability: f64, viscosity: f64, thickness: f64, t: f64) → f64` | Poroelastic consolidation | `biology::biomechanics::tissue` |
| `strain_energy_density_linear` | `fn strain_energy_density_linear(modulus: f64, strain: f64) → f64` | $W = E\varepsilon^2/2$ | `biology::biomechanics::tissue` |
| `creep_power_law` | `fn creep_power_law(a: f64, sigma: f64, n: f64, t: f64) → f64` | Power-law creep | `biology::biomechanics::tissue` |
| `bone_density_wolff` | `fn bone_density_wolff(rho0: f64, stimulus: f64, reference_stimulus: f64, rate: f64, dt: f64) → f64` | Wolff's law bone remodeling | `biology::biomechanics::tissue` |
| `ogden_model` | `fn ogden_model(mu: f64, alpha: f64, lambda: f64) → f64` | Ogden hyperelastic model | `biology::biomechanics::tissue` |
| `fracture_toughness` | `fn fracture_toughness(force: f64, crack_length: f64, width: f64, thickness: f64) → f64` | Fracture toughness | `biology::biomechanics::tissue` |
| `viscoelastic_prony` | `fn viscoelastic_prony(moduli: &[f64], taus: &[f64], t: f64) → f64` | Prony series viscoelastic | `biology::biomechanics::tissue` |
| `tissue_hydration_swelling` | `fn tissue_hydration_swelling(phi_0: f64, pi_ext: f64, bulk_modulus: f64) → f64` | Tissue swelling | `biology::biomechanics::tissue` |
| `biphasic_permeability` | `fn biphasic_permeability(k0: f64, strain: f64, m: f64) → f64` | Biphasic permeability | `biology::biomechanics::tissue` |
| `stress_fiber_remodeling` | `fn stress_fiber_remodeling(sigma_old: f64, reference: f64, rate: f64, dt: f64) → f64` | Stress fiber remodeling | `biology::biomechanics::tissue` |
| `damage_accumulation` | `fn damage_accumulation(d: f64, stress: f64, threshold: f64, rate: f64, dt: f64) → f64` | Damage accumulation | `biology::biomechanics::tissue` |
| `elastic_modulus_density` | `fn elastic_modulus_density(rho: f64, c: f64, exponent: f64) → f64` | $E = c\rho^n$ density-modulus | `biology::biomechanics::tissue` |

---

### 3️⃣3️⃣ bioelectricity/ — Bioelectricity — 81 pub fn

#### action_potential.rs (11 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `action_potential_shape` | `fn action_potential_shape(t: f64, v_rest: f64, v_peak: f64, tau_rise: f64, tau_fall: f64) → f64` | AP shape model | `biology::bioelectricity::action_potential` |
| `cable_equation_steady` | `fn cable_equation_steady(v0: f64, x: f64, lambda: f64) → f64` | $V(x) = V_0 e^{-x/\lambda}$ | `biology::bioelectricity::action_potential` |
| `electrotonic_length` | `fn electrotonic_length(physical_length: f64, space_constant: f64) → f64` | $L = l / \lambda$ | `biology::bioelectricity::action_potential` |
| `input_resistance_cylinder` | `fn input_resistance_cylinder(rm: f64, ri: f64, diameter: f64) → f64` | Input resistance (cylinder) | `biology::bioelectricity::action_potential` |
| `strength_duration_weiss` | `fn strength_duration_weiss(rheobase: f64, chronaxie: f64, duration: f64) → f64` | Weiss strength-duration | `biology::bioelectricity::action_potential` |
| `strength_duration_lapicque` | `fn strength_duration_lapicque(rheobase: f64, chronaxie: f64, duration: f64) → f64` | Lapicque strength-duration | `biology::bioelectricity::action_potential` |
| `local_field_potential` | `fn local_field_potential(currents: &[f64], distances: &[f64], sigma: f64) → f64` | Local field potential | `biology::bioelectricity::action_potential` |
| `extracellular_spike_amplitude` | `fn extracellular_spike_amplitude(transmembrane_current: f64, distance: f64, sigma: f64) → f64` | Extracellular spike amplitude | `biology::bioelectricity::action_potential` |
| `impedance_tissue` | `fn impedance_tissue(resistance: f64, capacitance: f64, frequency: f64) → f64` | Tissue impedance | `biology::bioelectricity::action_potential` |
| `defibrillation_threshold` | `fn defibrillation_threshold(body_mass: f64, transthoracic_impedance: f64) → f64` | Defibrillation threshold | `biology::bioelectricity::action_potential` |
| `bioimpedance_body_composition` | `fn bioimpedance_body_composition(impedance: f64, height: f64, weight: f64, age: f64, sex_factor: f64) → f64` | BIA body composition | `biology::bioelectricity::action_potential` |

#### impedance.rs (12 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `coulter_counter_volume` | `fn coulter_counter_volume(baseline_impedance: f64, pulse_amplitude: f64, orifice_volume: f64) → f64` | Coulter counter volume | `biology::bioelectricity::impedance` |
| `dielectrophoresis_force` | `fn dielectrophoresis_force(radius: f64, epsilon_m: f64, cm_factor: f64, grad_e2: f64) → f64` | DEP force | `biology::bioelectricity::impedance` |
| `clausius_mossotti` | `fn clausius_mossotti(epsilon_p: f64, epsilon_m: f64) → f64` | Clausius-Mossotti factor | `biology::bioelectricity::impedance` |
| `electroporation_threshold` | `fn electroporation_threshold(membrane_thickness: f64, breakdown_voltage: f64) → f64` | Electroporation threshold | `biology::bioelectricity::impedance` |
| `electroporation_pore_density` | `fn electroporation_pore_density(v_m: f64, v_ep: f64, n0: f64, alpha: f64) → f64` | Pore density | `biology::bioelectricity::impedance` |
| `joule_heating` | `fn joule_heating(current_density: f64, conductivity: f64, duration: f64) → f64` | Joule heating | `biology::bioelectricity::impedance` |
| `electrode_double_layer_capacitance` | `fn electrode_double_layer_capacitance(epsilon: f64, debye_length: f64) → f64` | Double-layer capacitance | `biology::bioelectricity::impedance` |
| `iontophoresis_flux` | `fn iontophoresis_flux(current: f64, z: f64, transport_number: f64) → f64` | Iontophoresis flux | `biology::bioelectricity::impedance` |
| `skin_impedance_model` | `fn skin_impedance_model(r_stratum: f64, c_stratum: f64, r_deep: f64, frequency: f64) → f64` | Skin impedance model | `biology::bioelectricity::impedance` |
| `ecg_lead_vector` | `fn ecg_lead_vector(dipole: (f64, f64, f64), lead_direction: (f64, f64, f64)) → f64` | ECG lead vector projection | `biology::bioelectricity::impedance` |
| `eeg_dipole_potential` | `fn eeg_dipole_potential(dipole_moment: f64, cos_angle: f64, distance: f64, conductivity: f64) → f64` | EEG dipole potential | `biology::bioelectricity::impedance` |
| `nerve_conduction_velocity_temperature` | `fn nerve_conduction_velocity_temperature(v_ref: f64, q10: f64, t: f64, t_ref: f64) → f64` | NCV temperature dependence | `biology::bioelectricity::impedance` |

#### membrane.rs (33 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `nernst_potential` | `fn nernst_potential(z: f64, temperature: f64, concentration_out: f64, concentration_in: f64) → f64` | $E = \frac{RT}{zF}\ln\frac{[X]_o}{[X]_i}$ | `biology::bioelectricity::membrane` |
| `goldman_hodgkin_katz` | `fn goldman_hodgkin_katz(pk: f64, k_out: f64, k_in: f64, pna: f64, na_out: f64, na_in: f64, pcl: f64, cl_out: f64, cl_in: f64, temperature: f64) → f64` | GHK voltage equation | `biology::bioelectricity::membrane` |
| `cable_equation_steady_state` | `fn cable_equation_steady_state(v0: f64, x: f64, lambda: f64) → f64` | Cable equation steady state | `biology::bioelectricity::membrane` |
| `membrane_time_constant` | `fn membrane_time_constant(rm: f64, cm: f64) → f64` | $\tau_m = R_m C_m$ | `biology::bioelectricity::membrane` |
| `length_constant` | `fn length_constant(rm: f64, ri: f64) → f64` | $\lambda = \sqrt{R_m/R_i}$ | `biology::bioelectricity::membrane` |
| `gap_junction_conductance` | `fn gap_junction_conductance(n_channels: f64, single_channel_conductance: f64, open_probability: f64) → f64` | Gap junction conductance | `biology::bioelectricity::membrane` |
| `electrodiffusion_flux` | `fn electrodiffusion_flux(permeability: f64, z: f64, vm: f64, c_out: f64, c_in: f64, temperature: f64) → f64` | GHK flux equation | `biology::bioelectricity::membrane` |
| `hodgkin_huxley_sodium_current` | `fn hodgkin_huxley_sodium_current(g_na: f64, m: f64, h: f64, v: f64, e_na: f64) → f64` | $I_{Na} = g_{Na}m^3h(V - E_{Na})$ | `biology::bioelectricity::membrane` |
| `hodgkin_huxley_potassium_current` | `fn hodgkin_huxley_potassium_current(g_k: f64, n: f64, v: f64, e_k: f64) → f64` | $I_K = g_K n^4(V - E_K)$ | `biology::bioelectricity::membrane` |
| `hodgkin_huxley_leak_current` | `fn hodgkin_huxley_leak_current(g_l: f64, v: f64, e_l: f64) → f64` | $I_L = g_L(V - E_L)$ | `biology::bioelectricity::membrane` |
| `hodgkin_huxley_dv_dt` | `fn hodgkin_huxley_dv_dt(cm: f64, i_ext: f64, i_na: f64, i_k: f64, i_l: f64) → f64` | HH $dV/dt$ | `biology::bioelectricity::membrane` |
| `gating_alpha_n` | `fn gating_alpha_n(v: f64) → f64` | $\alpha_n$ gating | `biology::bioelectricity::membrane` |
| `gating_beta_n` | `fn gating_beta_n(v: f64) → f64` | $\beta_n$ gating | `biology::bioelectricity::membrane` |
| `gating_alpha_m` | `fn gating_alpha_m(v: f64) → f64` | $\alpha_m$ gating | `biology::bioelectricity::membrane` |
| `gating_beta_m` | `fn gating_beta_m(v: f64) → f64` | $\beta_m$ gating | `biology::bioelectricity::membrane` |
| `gating_alpha_h` | `fn gating_alpha_h(v: f64) → f64` | $\alpha_h$ gating | `biology::bioelectricity::membrane` |
| `gating_beta_h` | `fn gating_beta_h(v: f64) → f64` | $\beta_h$ gating | `biology::bioelectricity::membrane` |
| `gating_steady_state` | `fn gating_steady_state(alpha: f64, beta: f64) → f64` | $x_\infty = \alpha/(\alpha+\beta)$ | `biology::bioelectricity::membrane` |
| `gating_time_constant` | `fn gating_time_constant(alpha: f64, beta: f64) → f64` | $\tau = 1/(\alpha+\beta)$ | `biology::bioelectricity::membrane` |
| `reversal_potential_two_ion` | `fn reversal_potential_two_ion(g1: f64, e1: f64, g2: f64, e2: f64) → f64` | Two-ion reversal potential | `biology::bioelectricity::membrane` |
| `membrane_capacitance_current` | `fn membrane_capacitance_current(cm: f64, dv_dt: f64) → f64` | $I_C = C_m dV/dt$ | `biology::bioelectricity::membrane` |
| `ion_channel_open_probability` | `fn ion_channel_open_probability(v: f64, v_half: f64, slope: f64) → f64` | Boltzmann open probability | `biology::bioelectricity::membrane` |
| `synaptic_conductance_alpha` | `fn synaptic_conductance_alpha(g_max: f64, t: f64, tau: f64) → f64` | Alpha synaptic conductance | `biology::bioelectricity::membrane` |
| `synaptic_current` | `fn synaptic_current(g_syn: f64, v_post: f64, e_syn: f64) → f64` | $I_{syn} = g_{syn}(V - E_{syn})$ | `biology::bioelectricity::membrane` |
| `calcium_nernst` | `fn calcium_nernst(temperature: f64, ca_out: f64, ca_in: f64) → f64` | Ca²⁺ Nernst potential | `biology::bioelectricity::membrane` |
| `chloride_equilibrium` | `fn chloride_equilibrium(temperature: f64, cl_out: f64, cl_in: f64) → f64` | Cl⁻ equilibrium potential | `biology::bioelectricity::membrane` |
| `resting_potential_contribution` | `fn resting_potential_contribution(conductance: f64, reversal: f64, total_conductance: f64) → f64` | Ion contribution to $V_{\text{rest}}$ | `biology::bioelectricity::membrane` |
| `space_clamp_error` | `fn space_clamp_error(distance: f64, lambda: f64) → f64` | Space clamp error | `biology::bioelectricity::membrane` |
| `action_potential_threshold_estimate` | `fn action_potential_threshold_estimate(v_rest: f64, depolarization: f64) → f64` | AP threshold estimate | `biology::bioelectricity::membrane` |
| `conduction_velocity` | `fn conduction_velocity(diameter: f64, myelinated: bool) → f64` | Conduction velocity | `biology::bioelectricity::membrane` |
| `saltatory_conduction_delay` | `fn saltatory_conduction_delay(internode_distance: f64, velocity: f64) → f64` | Saltatory conduction delay | `biology::bioelectricity::membrane` |
| `membrane_resistance_per_area` | `fn membrane_resistance_per_area(resistivity: f64, thickness: f64) → f64` | Membrane resistance per area | `biology::bioelectricity::membrane` |
| `specific_membrane_capacitance` | `fn specific_membrane_capacitance(epsilon_r: f64, thickness: f64) → f64` | $C_m = \epsilon_r \epsilon_0 / d$ | `biology::bioelectricity::membrane` |

#### stimulation.rs (25 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `defibrillation_energy` | `fn defibrillation_energy(capacitance: f64, voltage: f64) → f64` | $E = \frac{1}{2}CV^2$ | `biology::bioelectricity::stimulation` |
| `electrode_impedance` | `fn electrode_impedance(resistance: f64, capacitance: f64, frequency: f64) → f64` | Electrode impedance | `biology::bioelectricity::stimulation` |
| `stimulation_strength_duration` | `fn stimulation_strength_duration(rheobase: f64, chronaxie: f64, pulse_width: f64) → f64` | Strength-duration curve | `biology::bioelectricity::stimulation` |
| `bioimpedance_cole_model` | `fn bioimpedance_cole_model(r_inf: f64, r_0: f64, tau: f64, alpha: f64, frequency: f64) → (f64, f64)` | Cole model impedance | `biology::bioelectricity::stimulation` |
| `transcranial_current_density` | `fn transcranial_current_density(current: f64, electrode_area: f64) → f64` | tDCS current density | `biology::bioelectricity::stimulation` |
| `neural_recruitment_curve` | `fn neural_recruitment_curve(stimulus: f64, threshold: f64, saturation: f64, steepness: f64) → f64` | Neural recruitment curve | `biology::bioelectricity::stimulation` |
| `charge_density` | `fn charge_density(charge: f64, electrode_area: f64) → f64` | Charge density | `biology::bioelectricity::stimulation` |
| `cathodic_charge_balanced` | `fn cathodic_charge_balanced(anodic_amplitude: f64, anodic_duration: f64, cathodic_duration: f64) → f64` | Charge-balanced pulse | `biology::bioelectricity::stimulation` |
| `pulse_train_energy` | `fn pulse_train_energy(amplitude: f64, pulse_width: f64, frequency: f64, duration: f64, impedance: f64) → f64` | Pulse train energy | `biology::bioelectricity::stimulation` |
| `tissue_heating` | `fn tissue_heating(current_density: f64, conductivity: f64, duration: f64, specific_heat: f64, density: f64) → f64` | Tissue heating from stimulation | `biology::bioelectricity::stimulation` |
| `tms_induced_efield` | `fn tms_induced_efield(di_dt: f64, coil_inductance: f64, distance: f64) → f64` | TMS induced E-field | `biology::bioelectricity::stimulation` |
| `dbs_volume_tissue_activated` | `fn dbs_volume_tissue_activated(current: f64, impedance: f64, threshold_efield: f64) → f64` | DBS VTA | `biology::bioelectricity::stimulation` |
| `cochlear_implant_spread` | `fn cochlear_implant_spread(current: f64, distance: f64, sigma: f64) → f64` | CI current spread | `biology::bioelectricity::stimulation` |
| `fes_fatigue_index` | `fn fes_fatigue_index(initial_force: f64, final_force: f64) → f64` | FES fatigue index | `biology::bioelectricity::stimulation` |
| `shannon_safety_limit` | `fn shannon_safety_limit(charge_per_phase_uc: f64, electrode_area_cm2: f64) → f64` | Shannon safety limit | `biology::bioelectricity::stimulation` |
| `biphasic_pulse_charge` | `fn biphasic_pulse_charge(amplitude: f64, phase_duration: f64) → f64` | Biphasic pulse charge | `biology::bioelectricity::stimulation` |
| `interphase_gap_effect` | `fn interphase_gap_effect(threshold_no_gap: f64, gap_duration: f64, time_constant: f64) → f64` | Interphase gap effect | `biology::bioelectricity::stimulation` |
| `electrochemical_safety_margin` | `fn electrochemical_safety_margin(water_window: f64, electrode_potential: f64) → f64` | Electrochemical safety margin | `biology::bioelectricity::stimulation` |
| `warburg_impedance` | `fn warburg_impedance(sigma: f64, frequency: f64) → (f64, f64)` | Warburg impedance | `biology::bioelectricity::stimulation` |
| `constant_phase_element` | `fn constant_phase_element(q: f64, alpha: f64, frequency: f64) → (f64, f64)` | CPE impedance | `biology::bioelectricity::stimulation` |
| `chronaxie_from_strength_duration` | `fn chronaxie_from_strength_duration(rheobase: f64, threshold_at_pw: f64, pulse_width: f64) → f64` | Chronaxie estimation | `biology::bioelectricity::stimulation` |
| `galvanic_skin_response` | `fn galvanic_skin_response(baseline_conductance: f64, peak_conductance: f64, t: f64, tau_rise: f64, tau_decay: f64) → f64` | GSR model | `biology::bioelectricity::stimulation` |
| `total_charge_delivered` | `fn total_charge_delivered(amplitude: f64, pulse_width: f64, frequency: f64, duration: f64) → f64` | Total charge delivered | `biology::bioelectricity::stimulation` |
| `electrode_polarization_voltage` | `fn electrode_polarization_voltage(charge: f64, capacitance: f64) → f64` | Electrode polarization | `biology::bioelectricity::stimulation` |
| `anodal_break_excitation_threshold` | `fn anodal_break_excitation_threshold(membrane_tau: f64, pulse_duration: f64, rheobase: f64) → f64` | Anodal break excitation | `biology::bioelectricity::stimulation` |

---

### 3️⃣4️⃣ bioenergetics/ — Bioenergetics — 76 pub fn

#### atp.rs (12 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `atp_hydrolysis_free_energy` | `fn atp_hydrolysis_free_energy(delta_g0: f64, atp: f64, adp: f64, pi: f64, t: f64) → f64` | $\Delta G = \Delta G^0 + RT\ln\frac{[ADP][P_i]}{[ATP]}$ | `biology::bioenergetics::atp` |
| `p_o_ratio` | `fn p_o_ratio(atp_produced: f64, oxygen_consumed: f64) → f64` | P/O ratio | `biology::bioenergetics::atp` |
| `respiratory_control_index` | `fn respiratory_control_index(state3_rate: f64, state4_rate: f64) → f64` | RCI = state3/state4 | `biology::bioenergetics::atp` |
| `uncoupling_heat` | `fn uncoupling_heat(pmf: f64, proton_leak: f64) → f64` | Uncoupling heat | `biology::bioenergetics::atp` |
| `chemiosmotic_atp_rate` | `fn chemiosmotic_atp_rate(pmf: f64, atp_synthase_activity: f64, h_per_atp: f64) → f64` | Chemiosmotic ATP synthesis rate | `biology::bioenergetics::atp` |
| `shuttle_efficiency_malate_aspartate` | `fn shuttle_efficiency_malate_aspartate(nadh_cytoplasmic: f64, transfer_rate: f64) → f64` | Malate-aspartate shuttle | `biology::bioenergetics::atp` |
| `shuttle_efficiency_glycerol_3p` | `fn shuttle_efficiency_glycerol_3p(nadh_cytoplasmic: f64, transfer_rate: f64) → f64` | Glycerol-3-phosphate shuttle | `biology::bioenergetics::atp` |
| `metabolic_water` | `fn metabolic_water(glucose_oxidized: f64) → f64` | Metabolic water production | `biology::bioenergetics::atp` |
| `adenylate_energy_charge` | `fn adenylate_energy_charge(atp: f64, adp: f64, amp: f64) → f64` | $EC = \frac{[ATP] + 0.5[ADP]}{[ATP]+[ADP]+[AMP]}$ | `biology::bioenergetics::atp` |
| `phosphocreatine_equilibrium` | `fn phosphocreatine_equilibrium(creatine: f64, atp: f64, k_eq: f64) → f64` | PCr equilibrium | `biology::bioenergetics::atp` |
| `myosin_atpase_rate` | `fn myosin_atpase_rate(load_fraction: f64, vmax: f64) → f64` | Myosin ATPase rate | `biology::bioenergetics::atp` |
| `ionic_gradient_energy` | `fn ionic_gradient_energy(z: f64, vm: f64, c_out: f64, c_in: f64, t: f64) → f64` | Ionic gradient free energy | `biology::bioenergetics::atp` |

#### metabolism.rs (13 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `glycolysis_net_atp` | `fn glycolysis_net_atp(glucose: f64) → f64` | Net ATP from glycolysis | `biology::bioenergetics::metabolism` |
| `glycolysis_pyruvate_yield` | `fn glycolysis_pyruvate_yield(glucose: f64) → f64` | Pyruvate yield | `biology::bioenergetics::metabolism` |
| `gluconeogenesis_cost` | `fn gluconeogenesis_cost(glucose: f64) → f64` | Gluconeogenesis ATP cost | `biology::bioenergetics::metabolism` |
| `pentose_phosphate_nadph` | `fn pentose_phosphate_nadph(glucose_6p: f64) → f64` | PPP NADPH yield | `biology::bioenergetics::metabolism` |
| `fatty_acid_synthesis_cost` | `fn fatty_acid_synthesis_cost(acetyl_coa_units: f64) → f64` | FA synthesis ATP cost | `biology::bioenergetics::metabolism` |
| `urea_cycle_cost` | `fn urea_cycle_cost(amino_acids: f64) → f64` | Urea cycle ATP cost | `biology::bioenergetics::metabolism` |
| `glycogen_storage_efficiency` | `fn glycogen_storage_efficiency(glucose_units: f64) → f64` | Glycogen storage efficiency | `biology::bioenergetics::metabolism` |
| `warburg_effect` | `fn warburg_effect(aerobic_glycolysis_rate: f64, oxidative_rate: f64) → f64` | Warburg effect ratio | `biology::bioenergetics::metabolism` |
| `ketogenesis_yield` | `fn ketogenesis_yield(acetyl_coa: f64) → f64` | Ketogenesis yield | `biology::bioenergetics::metabolism` |
| `amino_acid_catabolism_atp` | `fn amino_acid_catabolism_atp(carbon_count: usize, is_glucogenic: bool, is_ketogenic: bool) → f64` | AA catabolism ATP | `biology::bioenergetics::metabolism` |
| `cori_cycle_cost` | `fn cori_cycle_cost(lactate: f64) → f64` | Cori cycle ATP cost | `biology::bioenergetics::metabolism` |
| `respiratory_quotient` | `fn respiratory_quotient(co2_produced: f64, o2_consumed: f64) → f64` | $RQ = CO_2 / O_2$ | `biology::bioenergetics::metabolism` |
| `metabolic_flux_control_coefficient` | `fn metabolic_flux_control_coefficient(v_enzyme: f64, v_pathway: f64, elasticity: f64) → f64` | MCA control coefficient | `biology::bioenergetics::metabolism` |

#### photosynthesis.rs (15 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `farquhar_model` | `fn farquhar_model(vcmax: f64, jmax: f64, ci: f64, gamma_star: f64, kc: f64, ko: f64, o: f64, rd: f64, par: f64) → f64` | Farquhar-von Caemmerer-Berry model | `biology::bioenergetics::photosynthesis` |
| `electron_transport_rate` | `fn electron_transport_rate(jmax: f64, par: f64) → f64` | Electron transport rate (J) | `biology::bioenergetics::photosynthesis` |
| `light_response_curve` | `fn light_response_curve(amax: f64, phi: f64, par: f64, rd: f64) → f64` | Light response curve | `biology::bioenergetics::photosynthesis` |
| `light_compensation_point` | `fn light_compensation_point(amax: f64, phi: f64, rd: f64) → f64` | Light compensation point | `biology::bioenergetics::photosynthesis` |
| `water_use_efficiency` | `fn water_use_efficiency(assimilation: f64, transpiration: f64) → f64` | WUE = A/E | `biology::bioenergetics::photosynthesis` |
| `rubisco_specificity` | `fn rubisco_specificity(vcmax: f64, kc: f64, vomax: f64, ko: f64) → f64` | Rubisco specificity factor | `biology::bioenergetics::photosynthesis` |
| `photorespiration_rate` | `fn photorespiration_rate(vomax: f64, o: f64, ko: f64, ci: f64, kc: f64) → f64` | Photorespiration rate | `biology::bioenergetics::photosynthesis` |
| `quantum_yield` | `fn quantum_yield(assimilation_rate: f64, photon_flux: f64) → f64` | Quantum yield | `biology::bioenergetics::photosynthesis` |
| `co2_compensation_point_photo` | `fn co2_compensation_point_photo(gamma_star: f64, rd: f64, vcmax: f64, kc: f64, ko: f64, o: f64) → f64` | CO₂ compensation point | `biology::bioenergetics::photosynthesis` |
| `stomatal_conductance_ball_berry` | `fn stomatal_conductance_ball_berry(assimilation: f64, rh: f64, cs: f64, g0: f64, g1: f64) → f64` | Ball-Berry stomatal model | `biology::bioenergetics::photosynthesis` |
| `mesophyll_conductance_photo` | `fn mesophyll_conductance_photo(assimilation: f64, ci: f64, cc: f64) → f64` | Mesophyll conductance | `biology::bioenergetics::photosynthesis` |
| `triose_phosphate_utilization` | `fn triose_phosphate_utilization(tpu: f64, ci: f64, gamma_star: f64) → f64` | TPU limitation | `biology::bioenergetics::photosynthesis` |
| `light_inhibition_photoinhibition` | `fn light_inhibition_photoinhibition(fv_fm_initial: f64, light_excess: f64, ki: f64) → f64` | Photoinhibition | `biology::bioenergetics::photosynthesis` |
| `canopy_photosynthesis_sun_shade` | `fn canopy_photosynthesis_sun_shade(lai: f64, k_ext: f64, a_sun: f64, a_shade: f64) → f64` | Canopy sun/shade model | `biology::bioenergetics::photosynthesis` |
| `carbon_concentrating_mechanism_benefit` | `fn carbon_concentrating_mechanism_benefit(ci_c3: f64, ci_c4: f64, vcmax: f64, kc: f64) → f64` | CCM benefit (C₃ vs C₄) | `biology::bioenergetics::photosynthesis` |

#### respiration.rs (16 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `atp_free_energy` | `fn atp_free_energy(delta_g0: f64, atp: f64, adp: f64, pi: f64, t: f64) → f64` | ATP free energy | `biology::bioenergetics::respiration` |
| `atp_synthase_rate` | `fn atp_synthase_rate(proton_gradient: f64, n_protons: f64, delta_g_atp: f64, t: f64) → f64` | ATP synthase rate | `biology::bioenergetics::respiration` |
| `proton_motive_force` | `fn proton_motive_force(delta_psi: f64, delta_ph: f64, t: f64) → f64` | $\Delta p = \Delta\psi - 2.3RT\Delta pH/F$ | `biology::bioenergetics::respiration` |
| `p_to_o_ratio` | `fn p_to_o_ratio(atp_produced: f64, oxygen_consumed: f64) → f64` | P/O ratio | `biology::bioenergetics::respiration` |
| `respiratory_control_ratio` | `fn respiratory_control_ratio(state3: f64, state4: f64) → f64` | RCR | `biology::bioenergetics::respiration` |
| `membrane_potential_nernst` | `fn membrane_potential_nernst(z: f64, c_out: f64, c_in: f64, t: f64) → f64` | Nernst potential | `biology::bioenergetics::respiration` |
| `uncoupler_effect` | `fn uncoupler_effect(pmf: f64, permeability: f64, concentration: f64) → f64` | Uncoupler effect on PMF | `biology::bioenergetics::respiration` |
| `citric_acid_cycle_nadh_yield` | `fn citric_acid_cycle_nadh_yield(acetyl_coa_flux: f64) → f64` | TCA NADH yield | `biology::bioenergetics::respiration` |
| `citric_acid_cycle_fadh2_yield` | `fn citric_acid_cycle_fadh2_yield(acetyl_coa_flux: f64) → f64` | TCA FADH₂ yield | `biology::bioenergetics::respiration` |
| `electron_transport_efficiency` | `fn electron_transport_efficiency(n_electrons: f64, delta_e: f64, delta_g_atp: f64, n_atp: f64) → f64` | ETC efficiency | `biology::bioenergetics::respiration` |
| `substrate_level_phosphorylation` | `fn substrate_level_phosphorylation(n_reactions: f64, delta_g_per_reaction: f64) → f64` | SLP ATP | `biology::bioenergetics::respiration` |
| `anaerobic_atp_yield` | `fn anaerobic_atp_yield(glucose_flux: f64) → f64` | Anaerobic ATP yield | `biology::bioenergetics::respiration` |
| `aerobic_atp_yield` | `fn aerobic_atp_yield(glucose_flux: f64) → f64` | Aerobic ATP yield | `biology::bioenergetics::respiration` |
| `lactate_production_rate` | `fn lactate_production_rate(pyruvate_flux: f64, nad_ratio: f64) → f64` | Lactate production rate | `biology::bioenergetics::respiration` |
| `beta_oxidation_atp_yield` | `fn beta_oxidation_atp_yield(carbon_chain_length: f64) → f64` | β-oxidation ATP yield | `biology::bioenergetics::respiration` |
| `creatine_phosphate_buffer` | `fn creatine_phosphate_buffer(atp: f64, adp: f64, cr_p: f64, keq: f64) → f64` | PCr buffer | `biology::bioenergetics::respiration` |

#### thermodynamics.rs (20 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `gibbs_free_energy_reaction` | `fn gibbs_free_energy_reaction(delta_h: f64, t: f64, delta_s: f64) → f64` | $\Delta G = \Delta H - T\Delta S$ | `biology::bioenergetics::thermodynamics` |
| `gibbs_free_energy_body_temp` | `fn gibbs_free_energy_body_temp(delta_h: f64, delta_s: f64) → f64` | $\Delta G$ at 310 K | `biology::bioenergetics::thermodynamics` |
| `equilibrium_constant_body_temp` | `fn equilibrium_constant_body_temp(delta_g0: f64) → f64` | $K_{eq}$ at body temperature | `biology::bioenergetics::thermodynamics` |
| `redox_potential_body_temp` | `fn redox_potential_body_temp(e0: f64, n: f64, oxidized: f64, reduced: f64) → f64` | Nernst at 310 K | `biology::bioenergetics::thermodynamics` |
| `equilibrium_constant` | `fn equilibrium_constant(delta_g0: f64, t: f64) → f64` | $K = e^{-\Delta G^0/RT}$ | `biology::bioenergetics::thermodynamics` |
| `redox_potential` | `fn redox_potential(e0: f64, n: f64, oxidized: f64, reduced: f64, t: f64) → f64` | Nernst equation | `biology::bioenergetics::thermodynamics` |
| `energy_charge` | `fn energy_charge(atp: f64, adp: f64, amp: f64) → f64` | Energy charge | `biology::bioenergetics::thermodynamics` |
| `metabolic_rate_kleiber` | `fn metabolic_rate_kleiber(mass: f64) → f64` | $P = 70 M^{0.75}$ Kleiber's law | `biology::bioenergetics::thermodynamics` |
| `oxygen_consumption_rate` | `fn oxygen_consumption_rate(metabolic_rate: f64, oxycaloric_equivalent: f64) → f64` | O₂ consumption rate | `biology::bioenergetics::thermodynamics` |
| `coupling_efficiency` | `fn coupling_efficiency(delta_g_atp: f64, delta_g_substrate: f64, n_atp: f64) → f64` | Coupling efficiency | `biology::bioenergetics::thermodynamics` |
| `heat_dissipation` | `fn heat_dissipation(delta_g_reaction: f64, useful_work: f64) → f64` | Heat dissipation | `biology::bioenergetics::thermodynamics` |
| `metabolic_rate_q10` | `fn metabolic_rate_q10(rate_ref: f64, t: f64, t_ref: f64, q10: f64) → f64` | $Q_{10}$ metabolic rate | `biology::bioenergetics::thermodynamics` |
| `arrhenius_metabolic` | `fn arrhenius_metabolic(rate_ref: f64, ea: f64, t: f64, t_ref: f64) → f64` | Arrhenius metabolic rate | `biology::bioenergetics::thermodynamics` |
| `thermogenic_cost` | `fn thermogenic_cost(delta_h: f64, efficiency: f64) → f64` | Thermogenesis cost | `biology::bioenergetics::thermodynamics` |
| `proton_gradient_energy` | `fn proton_gradient_energy(n_protons: f64, delta_mu: f64) → f64` | Proton gradient energy | `biology::bioenergetics::thermodynamics` |
| `nad_redox_potential` | `fn nad_redox_potential(nad_ox: f64, nad_red: f64, e0: f64, t: f64) → f64` | NAD⁺/NADH redox potential | `biology::bioenergetics::thermodynamics` |
| `entropy_production_rate` | `fn entropy_production_rate(heat_flux: f64, temperature: f64) → f64` | $\dot{S} = \dot{Q}/T$ | `biology::bioenergetics::thermodynamics` |
| `exergy_content` | `fn exergy_content(delta_h: f64, t0: f64, delta_s: f64) → f64` | Exergy content | `biology::bioenergetics::thermodynamics` |
| `muscle_mechanical_efficiency` | `fn muscle_mechanical_efficiency(work_output: f64, metabolic_input: f64) → f64` | Mechanical efficiency | `biology::bioenergetics::thermodynamics` |
| `basal_metabolic_scaling` | `fn basal_metabolic_scaling(m0: f64, mass: f64, exponent: f64) → f64` | $BMR = m_0 M^b$ allometric scaling | `biology::bioenergetics::thermodynamics` |

---

### 3️⃣5️⃣ biostatistics/ — Biostatistics — 67 pub fn

#### clinical.rs (21 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `odds_ratio` | `fn odds_ratio(a: usize, b: usize, c: usize, d: usize) → f64` | $OR = ad/bc$ | `biology::biostatistics::clinical` |
| `relative_risk` | `fn relative_risk(a: usize, b: usize, c: usize, d: usize) → f64` | Relative risk | `biology::biostatistics::clinical` |
| `absolute_risk_reduction` | `fn absolute_risk_reduction(risk_control: f64, risk_treatment: f64) → f64` | ARR | `biology::biostatistics::clinical` |
| `number_needed_to_treat` | `fn number_needed_to_treat(arr: f64) → f64` | $NNT = 1/ARR$ | `biology::biostatistics::clinical` |
| `sensitivity` | `fn sensitivity(tp: usize, fn_count: usize) → f64` | $Se = TP/(TP+FN)$ | `biology::biostatistics::clinical` |
| `specificity` | `fn specificity(tn: usize, fp: usize) → f64` | $Sp = TN/(TN+FP)$ | `biology::biostatistics::clinical` |
| `positive_predictive_value` | `fn positive_predictive_value(tp: usize, fp: usize) → f64` | PPV | `biology::biostatistics::clinical` |
| `negative_predictive_value` | `fn negative_predictive_value(tn: usize, fn_count: usize) → f64` | NPV | `biology::biostatistics::clinical` |
| `f1_score` | `fn f1_score(tp: usize, fp: usize, fn_count: usize) → f64` | F1 score | `biology::biostatistics::clinical` |
| `roc_auc` | `fn roc_auc(scores: &[(f64, bool)]) → f64` | ROC AUC | `biology::biostatistics::clinical` |
| `cohens_kappa` | `fn cohens_kappa(observed_agreement: f64, expected_agreement: f64) → f64` | Cohen's κ | `biology::biostatistics::clinical` |
| `likelihood_ratio_positive` | `fn likelihood_ratio_positive(sensitivity: f64, specificity: f64) → f64` | LR+ | `biology::biostatistics::clinical` |
| `likelihood_ratio_negative` | `fn likelihood_ratio_negative(sensitivity: f64, specificity: f64) → f64` | LR− | `biology::biostatistics::clinical` |
| `diagnostic_odds_ratio` | `fn diagnostic_odds_ratio(tp: usize, fp: usize, fn_count: usize, tn: usize) → f64` | DOR | `biology::biostatistics::clinical` |
| `youden_index` | `fn youden_index(sensitivity: f64, specificity: f64) → f64` | $J = Se + Sp - 1$ | `biology::biostatistics::clinical` |
| `matthews_correlation_coefficient` | `fn matthews_correlation_coefficient(tp: usize, tn: usize, fp: usize, fn_count: usize) → f64` | MCC | `biology::biostatistics::clinical` |
| `prevalence_adjusted_ppv` | `fn prevalence_adjusted_ppv(sensitivity: f64, specificity: f64, prevalence: f64) → f64` | Prevalence-adjusted PPV | `biology::biostatistics::clinical` |
| `sample_size_two_proportions` | `fn sample_size_two_proportions(p1: f64, p2: f64, alpha_z: f64, power_z: f64) → f64` | Sample size (two proportions) | `biology::biostatistics::clinical` |
| `confidence_interval_proportion` | `fn confidence_interval_proportion(p: f64, n: usize, z: f64) → (f64, f64)` | CI for proportion | `biology::biostatistics::clinical` |
| `attributable_risk` | `fn attributable_risk(risk_exposed: f64, risk_unexposed: f64) → f64` | Attributable risk | `biology::biostatistics::clinical` |
| `population_attributable_fraction` | `fn population_attributable_fraction(risk_exposed: f64, risk_unexposed: f64, prevalence_exposure: f64) → f64` | PAF | `biology::biostatistics::clinical` |

#### meta_analysis.rs (14 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `meta_analysis_fixed_effect` | `fn meta_analysis_fixed_effect(effects: &[f64], variances: &[f64]) → (f64, f64)` | Fixed-effect meta-analysis | `biology::biostatistics::meta_analysis` |
| `cochran_q` | `fn cochran_q(effects: &[f64], variances: &[f64]) → f64` | Cochran's Q | `biology::biostatistics::meta_analysis` |
| `i_squared` | `fn i_squared(q: f64, k: usize) → f64` | $I^2 = (Q - k + 1)/Q$ | `biology::biostatistics::meta_analysis` |
| `tau_squared_dsl` | `fn tau_squared_dsl(q: f64, k: usize, variances: &[f64]) → f64` | DerSimonian-Laird $\tau^2$ | `biology::biostatistics::meta_analysis` |
| `meta_analysis_random_effects` | `fn meta_analysis_random_effects(effects: &[f64], variances: &[f64], tau2: f64) → (f64, f64)` | Random-effects meta-analysis | `biology::biostatistics::meta_analysis` |
| `funnel_plot_asymmetry` | `fn funnel_plot_asymmetry(effects: &[f64], se: &[f64]) → f64` | Funnel plot asymmetry | `biology::biostatistics::meta_analysis` |
| `trim_and_fill` | `fn trim_and_fill(effects: &[f64]) → (f64, usize)` | Trim-and-fill | `biology::biostatistics::meta_analysis` |
| `fail_safe_n` | `fn fail_safe_n(effects: &[f64], variances: &[f64], alpha_z: f64) → f64` | Fail-safe N | `biology::biostatistics::meta_analysis` |
| `prediction_interval` | `fn prediction_interval(pooled: f64, tau2: f64, se_pooled: f64, k: usize) → (f64, f64)` | Prediction interval | `biology::biostatistics::meta_analysis` |
| `egger_regression` | `fn egger_regression(effects: &[f64], se: &[f64]) → (f64, f64)` | Egger's regression | `biology::biostatistics::meta_analysis` |
| `cumulative_meta_analysis` | `fn cumulative_meta_analysis(effects: &[f64], variances: &[f64]) → Vec<(f64, f64)>` | Cumulative meta-analysis | `biology::biostatistics::meta_analysis` |
| `influence_analysis` | `fn influence_analysis(effects: &[f64], variances: &[f64]) → Vec<f64>` | Leave-one-out influence | `biology::biostatistics::meta_analysis` |
| `h_squared` | `fn h_squared(q: f64, k: usize) → f64` | $H^2 = Q/(k-1)$ | `biology::biostatistics::meta_analysis` |
| `meta_regression_slope` | `fn meta_regression_slope(effects: &[f64], variances: &[f64], covariate: &[f64]) → f64` | Meta-regression slope | `biology::biostatistics::meta_analysis` |

#### regression.rs (13 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `simple_linear_regression` | `fn simple_linear_regression(x: &[f64], y: &[f64]) → (f64, f64)` | Simple linear regression | `biology::biostatistics::regression` |
| `r_squared` | `fn r_squared(y: &[f64], y_pred: &[f64]) → f64` | $R^2$ coefficient | `biology::biostatistics::regression` |
| `logistic_regression_probability` | `fn logistic_regression_probability(beta: &[f64], x: &[f64]) → f64` | Logistic regression $p$ | `biology::biostatistics::regression` |
| `aic` | `fn aic(log_likelihood: f64, k: usize) → f64` | $AIC = 2k - 2\ln L$ | `biology::biostatistics::regression` |
| `bic` | `fn bic(log_likelihood: f64, k: usize, n: usize) → f64` | $BIC = k\ln n - 2\ln L$ | `biology::biostatistics::regression` |
| `residual_standard_error` | `fn residual_standard_error(residuals: &[f64], p: usize) → f64` | RSE | `biology::biostatistics::regression` |
| `chi_squared_statistic` | `fn chi_squared_statistic(observed: &[f64], expected: &[f64]) → f64` | $\chi^2 = \sum(O-E)^2/E$ | `biology::biostatistics::regression` |
| `welch_t_statistic` | `fn welch_t_statistic(m1: f64, m2: f64, s1: f64, s2: f64, n1: usize, n2: usize) → f64` | Welch's t-test | `biology::biostatistics::regression` |
| `mann_whitney_u` | `fn mann_whitney_u(ranks_group1: &[f64], n1: usize, n2: usize) → f64` | Mann-Whitney U | `biology::biostatistics::regression` |
| `bonferroni_correction` | `fn bonferroni_correction(p_value: f64, n_tests: usize) → f64` | Bonferroni correction | `biology::biostatistics::regression` |
| `fishers_exact_test_odds` | `fn fishers_exact_test_odds(a: usize, b: usize, c: usize, d: usize) → f64` | Fisher's exact test | `biology::biostatistics::regression` |
| `spearman_rank_correlation` | `fn spearman_rank_correlation(rank_x: &[f64], rank_y: &[f64]) → f64` | Spearman's $\rho$ | `biology::biostatistics::regression` |
| `power_analysis_two_sample` | `fn power_analysis_two_sample(effect_size: f64, alpha_z: f64, power_z: f64) → f64` | Power analysis (two-sample) | `biology::biostatistics::regression` |

#### survival.rs (19 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `kaplan_meier` | `fn kaplan_meier(times: &[f64], events: &[bool]) → Vec<(f64, f64)>` | Kaplan-Meier estimator | `biology::biostatistics::survival` |
| `log_rank_statistic` | `fn log_rank_statistic(times1: &[f64], events1: &[bool], times2: &[f64], events2: &[bool]) → f64` | Log-rank test | `biology::biostatistics::survival` |
| `hazard_ratio` | `fn hazard_ratio(events_treatment: usize, time_treatment: f64, events_control: usize, time_control: f64) → f64` | Hazard ratio | `biology::biostatistics::survival` |
| `median_survival` | `fn median_survival(curve: &[(f64, f64)]) → f64` | Median survival time | `biology::biostatistics::survival` |
| `nelson_aalen` | `fn nelson_aalen(times: &[f64], events: &[bool]) → Vec<(f64, f64)>` | Nelson-Aalen estimator | `biology::biostatistics::survival` |
| `exponential_survival` | `fn exponential_survival(lambda: f64, t: f64) → f64` | $S(t) = e^{-\lambda t}$ | `biology::biostatistics::survival` |
| `weibull_survival` | `fn weibull_survival(lambda: f64, k: f64, t: f64) → f64` | $S(t) = e^{-(\lambda t)^k}$ | `biology::biostatistics::survival` |
| `restricted_mean_survival_time` | `fn restricted_mean_survival_time(curve: &[(f64, f64)], t_max: f64) → f64` | RMST | `biology::biostatistics::survival` |
| `greenwood_variance` | `fn greenwood_variance(curve: &[(f64, f64)], at_risk: &[usize], events: &[usize]) → Vec<f64>` | Greenwood variance | `biology::biostatistics::survival` |
| `cumulative_incidence` | `fn cumulative_incidence(times: &[f64], events: &[bool], competing: &[bool]) → Vec<(f64, f64)>` | Cumulative incidence (competing risks) | `biology::biostatistics::survival` |
| `life_table` | `fn life_table(age_groups: &[(f64, f64)], deaths: &[f64], population: &[f64]) → Vec<(f64, f64, f64)>` | Life table | `biology::biostatistics::survival` |
| `log_logistic_survival` | `fn log_logistic_survival(alpha: f64, beta: f64, t: f64) → f64` | Log-logistic survival | `biology::biostatistics::survival` |
| `gompertz_survival` | `fn gompertz_survival(alpha: f64, beta: f64, t: f64) → f64` | Gompertz survival | `biology::biostatistics::survival` |
| `cox_partial_likelihood_contribution` | `fn cox_partial_likelihood_contribution(beta_x: f64, risk_set_sum: f64) → f64` | Cox PH partial likelihood | `biology::biostatistics::survival` |
| `breslow_cumulative_hazard` | `fn breslow_cumulative_hazard(event_times: &[f64], risk_set_sums: &[f64]) → Vec<(f64, f64)>` | Breslow cumulative hazard | `biology::biostatistics::survival` |
| `survival_from_hazard` | `fn survival_from_hazard(cumulative_hazard: f64) → f64` | $S = e^{-H}$ | `biology::biostatistics::survival` |
| `conditional_survival` | `fn conditional_survival(s_t: f64, s_t_plus_x: f64) → f64` | Conditional survival | `biology::biostatistics::survival` |
| `cure_fraction_model` | `fn cure_fraction_model(cure_rate: f64, lambda: f64, t: f64) → f64` | Cure fraction model | `biology::biostatistics::survival` |
| `aalen_johansen` | `fn aalen_johansen(times: &[f64], events: &[u8], n_causes: usize) → Vec<Vec<(f64, f64)>>` | Aalen-Johansen estimator | `biology::biostatistics::survival` |

---

### 3️⃣6️⃣ cancer_biology/ — Cancer Biology — 65 pub fn

#### immunology.rs (8 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `tumor_immune_ode` | `fn tumor_immune_ode(tumor: f64, immune: f64, growth_rate: f64, carrying_capacity: f64, kill_rate: f64, stimulation: f64, decay_rate: f64) → (f64, f64)` | Tumor-immune ODE | `biology::cancer_biology::immunology` |
| `tumor_immune_simulate` | `fn tumor_immune_simulate(tumor0: f64, immune0: f64, growth_rate: f64, carrying_capacity: f64, kill_rate: f64, stimulation: f64, decay_rate: f64, dt: f64, steps: usize) → Vec<(f64, f64)>` | Tumor-immune simulation | `biology::cancer_biology::immunology` |
| `immunoediting_escape` | `fn immunoediting_escape(immunogenic_clones: f64, escape_mutation_rate: f64, immune_pressure: f64) → f64` | Immunoediting escape | `biology::cancer_biology::immunology` |
| `checkpoint_blockade_effect` | `fn checkpoint_blockade_effect(baseline_kill: f64, pd1_inhibition: f64, ctla4_inhibition: f64) → f64` | Checkpoint blockade | `biology::cancer_biology::immunology` |
| `car_t_cell_expansion` | `fn car_t_cell_expansion(initial_cells: f64, antigen_density: f64, expansion_rate: f64, t: f64) → f64` | CAR-T expansion | `biology::cancer_biology::immunology` |
| `cytokine_release_syndrome` | `fn cytokine_release_syndrome(activated_cells: f64, cytokine_per_cell: f64, clearance_rate: f64, t: f64) → f64` | Cytokine release syndrome | `biology::cancer_biology::immunology` |
| `tumor_neoantigen_fitness` | `fn tumor_neoantigen_fitness(binding_affinity: f64, expression_level: f64, clonality: f64) → f64` | Neoantigen fitness | `biology::cancer_biology::immunology` |
| `abscopal_effect` | `fn abscopal_effect(local_dose: f64, immune_activation: f64, distant_tumor: f64, sensitivity: f64) → f64` | Abscopal effect | `biology::cancer_biology::immunology` |

#### microenvironment.rs (12 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `tumor_angiogenesis_vegf` | `fn tumor_angiogenesis_vegf(vegf: f64, endothelial_proliferation_rate: f64, kd: f64) → f64` | VEGF-driven angiogenesis | `biology::cancer_biology::microenvironment` |
| `vessel_density` | `fn vessel_density(new_vessels: f64, existing_vessels: f64, regression_rate: f64, dt: f64) → f64` | Vessel density | `biology::cancer_biology::microenvironment` |
| `oxygen_diffusion_krogh` | `fn oxygen_diffusion_krogh(p_vessel: f64, consumption_rate: f64, diffusion_coeff: f64, r: f64, r_vessel: f64) → f64` | Krogh oxygen diffusion | `biology::cancer_biology::microenvironment` |
| `hypoxia_fraction` | `fn hypoxia_fraction(distances: &[f64], diffusion_limit: f64) → f64` | Hypoxia fraction | `biology::cancer_biology::microenvironment` |
| `microenvironment_tmb` | `fn microenvironment_tmb(mutations: usize, megabases_sequenced: f64) → f64` | Tumor mutational burden | `biology::cancer_biology::microenvironment` |
| `clonal_fitness_advantage` | `fn clonal_fitness_advantage(clone_sizes: &[f64], fitness_values: &[f64]) → f64` | Clonal fitness advantage | `biology::cancer_biology::microenvironment` |
| `tumor_heterogeneity_shannon` | `fn tumor_heterogeneity_shannon(clone_fractions: &[f64]) → f64` | Shannon heterogeneity | `biology::cancer_biology::microenvironment` |
| `metastatic_probability` | `fn metastatic_probability(invasion_rate: f64, survival_fraction: f64, colonization_rate: f64, time: f64) → f64` | Metastasis probability | `biology::cancer_biology::microenvironment` |
| `emt_score` | `fn emt_score(epithelial_markers: &[f64], mesenchymal_markers: &[f64]) → f64` | EMT score | `biology::cancer_biology::microenvironment` |
| `immune_escape_probability` | `fn immune_escape_probability(mhc_expression: f64, pd_l1: f64, neoantigen_load: f64) → f64` | Immune escape probability | `biology::cancer_biology::microenvironment` |
| `csc_fraction` | `fn csc_fraction(symmetric_division_rate: f64, asymmetric_rate: f64, differentiation_rate: f64) → f64` | Cancer stem cell fraction | `biology::cancer_biology::microenvironment` |
| `pharmacokinetic_tumor_exposure` | `fn pharmacokinetic_tumor_exposure(dose: f64, bioavailability: f64, volume_distribution: f64, tumor_perfusion_fraction: f64) → f64` | PK tumor exposure | `biology::cancer_biology::microenvironment` |

#### therapy.rs (22 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `cell_kill_log` | `fn cell_kill_log(initial: f64, surviving_fraction: f64, cycles: u32) → f64` | Log cell kill | `biology::cancer_biology::therapy` |
| `skipper_schabel_log_kill` | `fn skipper_schabel_log_kill(n: f64, dose: f64, sensitivity: f64) → f64` | Skipper-Schabel log kill | `biology::cancer_biology::therapy` |
| `drug_resistance_goldie_coldman` | `fn drug_resistance_goldie_coldman(n: f64, mutation_rate: f64) → f64` | Goldie-Coldman resistance | `biology::cancer_biology::therapy` |
| `combination_therapy_survival` | `fn combination_therapy_survival(sf_a: f64, sf_b: f64, interaction: f64) → f64` | Combination therapy survival | `biology::cancer_biology::therapy` |
| `tumor_immune_interaction` | `fn tumor_immune_interaction(tumor: f64, immune: f64, growth_rate: f64, kill_rate: f64, stimulation: f64, decay: f64, k: f64) → (f64, f64)` | Tumor-immune interaction | `biology::cancer_biology::therapy` |
| `hallmarks_proliferation_index` | `fn hallmarks_proliferation_index(mitotic_count: f64, area: f64) → f64` | Proliferation index | `biology::cancer_biology::therapy` |
| `cancer_stem_cell_fraction` | `fn cancer_stem_cell_fraction(symmetric_division_rate: f64, asymmetric_division_rate: f64, differentiation_rate: f64) → f64` | CSC fraction | `biology::cancer_biology::therapy` |
| `linear_quadratic_survival` | `fn linear_quadratic_survival(dose: f64, alpha: f64, beta: f64) → f64` | $SF = e^{-\alpha D - \beta D^2}$ | `biology::cancer_biology::therapy` |
| `biologically_effective_dose` | `fn biologically_effective_dose(dose: f64, fractions: f64, alpha_beta: f64) → f64` | BED | `biology::cancer_biology::therapy` |
| `equivalent_dose_2gy` | `fn equivalent_dose_2gy(dose: f64, dose_per_fraction: f64, alpha_beta: f64) → f64` | EQD2 | `biology::cancer_biology::therapy` |
| `tumor_control_probability` | `fn tumor_control_probability(n_cells: f64, surviving_fraction: f64) → f64` | TCP | `biology::cancer_biology::therapy` |
| `normal_tissue_complication_probability` | `fn normal_tissue_complication_probability(dose: f64, td50: f64, gamma50: f64) → f64` | NTCP | `biology::cancer_biology::therapy` |
| `therapeutic_ratio` | `fn therapeutic_ratio(tcp: f64, ntcp: f64) → f64` | Therapeutic ratio | `biology::cancer_biology::therapy` |
| `immunotherapy_checkpoint_response` | `fn immunotherapy_checkpoint_response(tumor: f64, t_cells: f64, activation_rate: f64, exhaustion_rate: f64, checkpoint_blockade: f64) → f64` | Checkpoint immunotherapy | `biology::cancer_biology::therapy` |
| `car_t_expansion` | `fn car_t_expansion(initial_cells: f64, antigen_stimulation: f64, expansion_rate: f64, t: f64) → f64` | CAR-T expansion | `biology::cancer_biology::therapy` |
| `antibody_drug_conjugate_kill` | `fn antibody_drug_conjugate_kill(antibody_conc: f64, target_density: f64, internalization_rate: f64, drug_potency: f64, kd: f64) → f64` | ADC kill | `biology::cancer_biology::therapy` |
| `metronomic_antiangiogenic_effect` | `fn metronomic_antiangiogenic_effect(dose: f64, frequency: f64, sensitivity: f64) → f64` | Metronomic antiangiogenic | `biology::cancer_biology::therapy` |
| `fractionation_schedule_bde` | `fn fractionation_schedule_bde(n_fractions: u32, dose_per_fraction: f64, alpha_beta: f64) → f64` | Fractionation BDE | `biology::cancer_biology::therapy` |
| `cell_cycle_specific_kill` | `fn cell_cycle_specific_kill(drug_conc: f64, phase_fraction: f64, sensitivity: f64) → f64` | Cell cycle specific kill | `biology::cancer_biology::therapy` |
| `combination_index_chou_talalay` | `fn combination_index_chou_talalay(fa: f64, dose_a: f64, dose_b: f64, dm_a: f64, dm_b: f64, m_a: f64, m_b: f64) → f64` | Chou-Talalay CI | `biology::cancer_biology::therapy` |
| `radiation_oxygen_enhancement_ratio` | `fn radiation_oxygen_enhancement_ratio(dose_hypoxic: f64, dose_aerobic: f64) → f64` | OER | `biology::cancer_biology::therapy` |
| `hyperthermia_enhancement` | `fn hyperthermia_enhancement(dose: f64, thermal_enhancement_ratio: f64) → f64` | Hyperthermia enhancement | `biology::cancer_biology::therapy` |

#### tumor.rs (23 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `tumor_growth_gompertz` | `fn tumor_growth_gompertz(n: f64, n_max: f64, alpha: f64, dt: f64) → f64` | Gompertz tumor growth | `biology::cancer_biology::tumor` |
| `tumor_growth_logistic` | `fn tumor_growth_logistic(n: f64, k: f64, r: f64, dt: f64) → f64` | Logistic tumor growth | `biology::cancer_biology::tumor` |
| `tumor_doubling_time` | `fn tumor_doubling_time(growth_rate: f64) → f64` | $T_d = \ln 2 / r$ | `biology::cancer_biology::tumor` |
| `clonal_evolution_fitness` | `fn clonal_evolution_fitness(clone_sizes: &[f64], fitness: &[f64], mutation_rate: f64) → Vec<f64>` | Clonal evolution | `biology::cancer_biology::tumor` |
| `metastasis_probability` | `fn metastasis_probability(tumor_size: f64, shedding_rate: f64, survival_fraction: f64, colonization_rate: f64) → f64` | Metastasis probability | `biology::cancer_biology::tumor` |
| `tumor_angiogenesis_rate` | `fn tumor_angiogenesis_rate(tumor_size: f64, vegf_production: f64, inhibitor: f64, threshold: f64) → f64` | Angiogenesis rate | `biology::cancer_biology::tumor` |
| `norton_simon_regression` | `fn norton_simon_regression(n: f64, kill_fraction: f64, gompertz_rate: f64, n_max: f64) → f64` | Norton-Simon regression | `biology::cancer_biology::tumor` |
| `tumor_growth_exponential` | `fn tumor_growth_exponential(n0: f64, rate: f64, t: f64) → f64` | Exponential growth | `biology::cancer_biology::tumor` |
| `tumor_growth_von_bertalanffy` | `fn tumor_growth_von_bertalanffy(n: f64, a: f64, b: f64, dt: f64) → f64` | Von Bertalanffy growth | `biology::cancer_biology::tumor` |
| `tumor_volume_spherical` | `fn tumor_volume_spherical(diameter: f64) → f64` | $V = \pi d^3/6$ | `biology::cancer_biology::tumor` |
| `tumor_volume_ellipsoid` | `fn tumor_volume_ellipsoid(length: f64, width: f64, height: f64) → f64` | Ellipsoid volume | `biology::cancer_biology::tumor` |
| `recist_response` | `fn recist_response(baseline_diameter: f64, current_diameter: f64) → f64` | RECIST response | `biology::cancer_biology::tumor` |
| `tumor_mutation_burden` | `fn tumor_mutation_burden(somatic_mutations: f64, exome_size_mb: f64) → f64` | TMB | `biology::cancer_biology::tumor` |
| `heterogeneity_shannon` | `fn heterogeneity_shannon(clone_fractions: &[f64]) → f64` | Shannon heterogeneity | `biology::cancer_biology::tumor` |
| `circulating_tumor_cells` | `fn circulating_tumor_cells(shedding: f64, tumor_size: f64, half_life: f64) → f64` | CTC dynamics | `biology::cancer_biology::tumor` |
| `warburg_glycolysis_rate` | `fn warburg_glycolysis_rate(glucose: f64, vmax: f64, km: f64, oxygen_inhibition: f64, oxygen: f64) → f64` | Warburg effect | `biology::cancer_biology::tumor` |
| `hypoxia_inducible_factor` | `fn hypoxia_inducible_factor(po2: f64, km_o2: f64, max_expression: f64) → f64` | HIF expression | `biology::cancer_biology::tumor` |
| `necrotic_core_radius` | `fn necrotic_core_radius(tumor_radius: f64, diffusion_length: f64) → f64` | Necrotic core radius | `biology::cancer_biology::tumor` |
| `viable_rim_fraction` | `fn viable_rim_fraction(tumor_radius: f64, necrotic_radius: f64) → f64` | Viable rim fraction | `biology::cancer_biology::tumor` |
| `ctc_cluster_survival` | `fn ctc_cluster_survival(single_ctc_survival: f64, cluster_size: u32) → f64` | CTC cluster survival | `biology::cancer_biology::tumor` |
| `invasion_index` | `fn invasion_index(invaded_distance: f64, time: f64) → f64` | Invasion index | `biology::cancer_biology::tumor` |
| `epithelial_mesenchymal_transition` | `fn epithelial_mesenchymal_transition(tgf_beta: f64, threshold: f64, hill: f64) → f64` | EMT | `biology::cancer_biology::tumor` |
| `microsatellite_instability_score` | `fn microsatellite_instability_score(unstable_markers: u32, total_markers: u32) → f64` | MSI score | `biology::cancer_biology::tumor` |

---

### 3️⃣7️⃣ aging/ — Aging — 70 pub fn

#### damage.rs (20 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `telomere_shortening` | `fn telomere_shortening(initial_length: f64, loss_per_division: f64, divisions: f64) → f64` | Telomere attrition | `biology::aging::damage` |
| `hayflick_limit` | `fn hayflick_limit(initial_length: f64, critical_length: f64, loss_per_division: f64) → f64` | Hayflick limit | `biology::aging::damage` |
| `telomerase_equilibrium` | `fn telomerase_equilibrium(shortening_rate: f64, elongation_rate: f64, initial: f64, t: f64) → f64` | Telomerase equilibrium | `biology::aging::damage` |
| `oxidative_damage_accumulation` | `fn oxidative_damage_accumulation(production_rate: f64, repair_rate: f64, t: f64, initial_damage: f64) → f64` | Oxidative damage | `biology::aging::damage` |
| `mitochondrial_damage` | `fn mitochondrial_damage(intact_fraction: f64, damage_rate: f64, dt: f64) → f64` | Mitochondrial damage | `biology::aging::damage` |
| `senescent_cell_fraction` | `fn senescent_cell_fraction(division_rate: f64, senescence_prob: f64, clearance_rate: f64, t: f64) → f64` | Senescent cell fraction | `biology::aging::damage` |
| `caloric_restriction_lifespan` | `fn caloric_restriction_lifespan(base_lifespan: f64, restriction_fraction: f64, effect_coefficient: f64) → f64` | CR lifespan extension | `biology::aging::damage` |
| `reliability_theory_survival` | `fn reliability_theory_survival(n_elements: usize, element_failure_rate: f64, redundancy: usize, t: f64) → f64` | Reliability theory | `biology::aging::damage` |
| `ros_steady_state` | `fn ros_steady_state(production_rate: f64, sod_activity: f64, catalase_activity: f64) → f64` | ROS steady state | `biology::aging::damage` |
| `protein_aggregation` | `fn protein_aggregation(misfolded: f64, aggregation_rate: f64, chaperone_capacity: f64, dt: f64) → f64` | Protein aggregation | `biology::aging::damage` |
| `dna_repair_capacity` | `fn dna_repair_capacity(age: f64, base_capacity: f64, decline_rate: f64) → f64` | DNA repair capacity | `biology::aging::damage` |
| `somatic_mutation_accumulation` | `fn somatic_mutation_accumulation(mutation_rate: f64, divisions: f64, repair_efficiency: f64) → f64` | Somatic mutations | `biology::aging::damage` |
| `epigenetic_clock_horvath` | `fn epigenetic_clock_horvath(cpg_values: &[f64], coefficients: &[f64], intercept: f64) → f64` | Horvath clock | `biology::aging::damage` |
| `nad_decline` | `fn nad_decline(initial_nad: f64, decline_rate: f64, age: f64) → f64` | NAD⁺ decline | `biology::aging::damage` |
| `autophagy_flux` | `fn autophagy_flux(substrate: f64, autophagosome_formation: f64, lysosomal_activity: f64, age_factor: f64) → f64` | Autophagy flux | `biology::aging::damage` |
| `stem_cell_exhaustion` | `fn stem_cell_exhaustion(initial_pool: f64, division_rate: f64, senescence_prob: f64, age: f64) → f64` | Stem cell exhaustion | `biology::aging::damage` |
| `inflammaging_cytokine` | `fn inflammaging_cytokine(basal: f64, senescent_cells: f64, amplification: f64) → f64` | Inflammaging | `biology::aging::damage` |
| `crosslink_accumulation` | `fn crosslink_accumulation(rate: f64, turnover: f64, t: f64) → f64` | Crosslink accumulation | `biology::aging::damage` |
| `lipofuscin_accumulation` | `fn lipofuscin_accumulation(production_rate: f64, t: f64) → f64` | Lipofuscin accumulation | `biology::aging::damage` |
| `immune_senescence` | `fn immune_senescence(naive_t_cells: f64, thymic_output_rate: f64, age: f64, proliferation_capacity: f64) → f64` | Immune senescence | `biology::aging::damage` |

#### mortality.rs (12 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `gompertz_mortality_rate` | `fn gompertz_mortality_rate(a: f64, b: f64, age: f64) → f64` | $\mu(x) = ae^{bx}$ Gompertz | `biology::aging::mortality` |
| `gompertz_makeham` | `fn gompertz_makeham(a: f64, b: f64, c: f64, age: f64) → f64` | Gompertz-Makeham | `biology::aging::mortality` |
| `weibull_mortality_hazard` | `fn weibull_mortality_hazard(lambda: f64, k: f64, age: f64) → f64` | Weibull hazard | `biology::aging::mortality` |
| `mortality_doubling_time` | `fn mortality_doubling_time(b: f64) → f64` | MRDT = $\ln 2/b$ | `biology::aging::mortality` |
| `survival_probability` | `fn survival_probability(hazard_rates: &[f64], dt: f64) → f64` | Survival probability | `biology::aging::mortality` |
| `life_expectancy` | `fn life_expectancy(survival_curve: &[(f64, f64)]) → f64` | Life expectancy | `biology::aging::mortality` |
| `deceleration_plateau` | `fn deceleration_plateau(age: f64, plateau_age: f64, plateau_rate: f64, a: f64, b: f64) → f64` | Late-life mortality plateau | `biology::aging::mortality` |
| `frailty_deficit_index` | `fn frailty_deficit_index(deficits: usize, total_items: usize) → f64` | Frailty deficit index | `biology::aging::mortality` |
| `phenotypic_age_levine` | `fn phenotypic_age_levine(albumin: f64, creatinine: f64, glucose: f64, crp: f64, lymphocyte_pct: f64, mcv: f64, rdw: f64, alp: f64, wbc: f64, chronological_age: f64) → f64` | Levine phenotypic age | `biology::aging::mortality` |
| `horvath_clock` | `fn horvath_clock(cpg_betas: &[f64], coefficients: &[f64], intercept: f64) → f64` | Horvath epigenetic clock | `biology::aging::mortality` |
| `cr_lifespan_extension` | `fn cr_lifespan_extension(baseline_lifespan: f64, restriction_fraction: f64, max_extension: f64) → f64` | CR lifespan extension | `biology::aging::mortality` |
| `reliability_theory_failure` | `fn reliability_theory_failure(initial_elements: usize, redundancy: usize, failure_rate: f64, t: f64) → f64` | Reliability theory failure | `biology::aging::mortality` |

#### oxidative_stress.rs (12 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `ros_production_rate` | `fn ros_production_rate(metabolic_rate: f64, coupling_efficiency: f64) → f64` | ROS production rate | `biology::aging::oxidative_stress` |
| `antioxidant_capacity` | `fn antioxidant_capacity(sod: f64, catalase: f64, glutathione: f64, k_sod: f64, k_cat: f64, k_gsh: f64) → f64` | Antioxidant capacity | `biology::aging::oxidative_stress` |
| `oxidative_damage_rate` | `fn oxidative_damage_rate(ros_level: f64, antioxidant: f64) → f64` | Oxidative damage rate | `biology::aging::oxidative_stress` |
| `lipid_peroxidation` | `fn lipid_peroxidation(pufa_concentration: f64, ros_level: f64, k_initiation: f64, k_propagation: f64) → f64` | Lipid peroxidation | `biology::aging::oxidative_stress` |
| `protein_carbonylation` | `fn protein_carbonylation(protein_conc: f64, ros_level: f64, rate_constant: f64) → f64` | Protein carbonylation | `biology::aging::oxidative_stress` |
| `dna_8oxog_formation` | `fn dna_8oxog_formation(ros_level: f64, guanine_sites: f64, k_oxidation: f64) → f64` | 8-oxoG formation | `biology::aging::oxidative_stress` |
| `mitochondrial_ros_vicious_cycle` | `fn mitochondrial_ros_vicious_cycle(damage: f64, ros_base: f64, amplification: f64, repair_rate: f64, dt: f64) → f64` | Mitochondrial ROS cycle | `biology::aging::oxidative_stress` |
| `glutathione_ratio` | `fn glutathione_ratio(gsh: f64, gssg: f64) → f64` | GSH/GSSG ratio | `biology::aging::oxidative_stress` |
| `fenton_reaction_rate` | `fn fenton_reaction_rate(fe2: f64, h2o2: f64, k_fenton: f64) → f64` | Fenton reaction | `biology::aging::oxidative_stress` |
| `nrf2_response` | `fn nrf2_response(ros_level: f64, keap1: f64, k_activation: f64) → f64` | Nrf2 response | `biology::aging::oxidative_stress` |
| `carbonyl_stress` | `fn carbonyl_stress(methylglyoxal: f64, glyoxalase: f64, km: f64) → f64` | Carbonyl stress | `biology::aging::oxidative_stress` |
| `oxidative_stress_index` | `fn oxidative_stress_index(total_oxidant: f64, total_antioxidant: f64) → f64` | OSI | `biology::aging::oxidative_stress` |

#### senescence.rs (17 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `gompertz_mortality` | `fn gompertz_mortality(a: f64, b: f64, age: f64) → f64` | Gompertz mortality | `biology::aging::senescence` |
| `gompertz_survival` | `fn gompertz_survival(a: f64, b: f64, age: f64) → f64` | Gompertz survival | `biology::aging::senescence` |
| `weibull_mortality` | `fn weibull_mortality(lambda: f64, k: f64, age: f64) → f64` | Weibull mortality | `biology::aging::senescence` |
| `weibull_survival` | `fn weibull_survival(lambda: f64, k: f64, age: f64) → f64` | Weibull survival | `biology::aging::senescence` |
| `gompertz_makeham_mortality` | `fn gompertz_makeham_mortality(a: f64, b: f64, c: f64, age: f64) → f64` | Gompertz-Makeham | `biology::aging::senescence` |
| `mortality_rate_doubling_time` | `fn mortality_rate_doubling_time(b: f64) → f64` | MRDT | `biology::aging::senescence` |
| `life_expectancy_from_survival` | `fn life_expectancy_from_survival(survival: impl Fn(f64) → f64, max_age: f64, dt: f64) → f64` | Life expectancy | `biology::aging::senescence` |
| `siler_mortality` | `fn siler_mortality(a1: f64, b1: f64, a2: f64, a3: f64, b3: f64, age: f64) → f64` | Siler mortality | `biology::aging::senescence` |
| `logistic_mortality_plateau` | `fn logistic_mortality_plateau(a: f64, b: f64, c: f64, age: f64) → f64` | Logistic mortality plateau | `biology::aging::senescence` |
| `demographic_entropy` | `fn demographic_entropy(life_table_lx: &[f64]) → f64` | Demographic entropy | `biology::aging::senescence` |
| `net_reproduction_rate` | `fn net_reproduction_rate(survivorship: &[f64], fecundity: &[f64]) → f64` | $R_0$ | `biology::aging::senescence` |
| `generation_time` | `fn generation_time(survivorship: &[f64], fecundity: &[f64]) → f64` | Generation time | `biology::aging::senescence` |
| `actuarial_senescence_rate` | `fn actuarial_senescence_rate(mortality_young: f64, mortality_old: f64, age_interval: f64) → f64` | Actuarial senescence rate | `biology::aging::senescence` |
| `proportional_hazards` | `fn proportional_hazards(baseline_hazard: f64, covariates: &[f64], coefficients: &[f64]) → f64` | Cox proportional hazards | `biology::aging::senescence` |
| `biological_age_levine` | `fn biological_age_levine(chronological_age: f64, albumin: f64, creatinine: f64, glucose: f64, crp_ln: f64, lymphocyte_pct: f64, mcv: f64, rdw: f64, alkaline_phosphatase: f64, wbc: f64) → f64` | Levine biological age | `biology::aging::senescence` |
| `frailty_index` | `fn frailty_index(deficits_present: u32, deficits_measured: u32) → f64` | Frailty index | `biology::aging::senescence` |
| `disability_free_life_expectancy` | `fn disability_free_life_expectancy(survival: &[f64], disability_free: &[f64]) → f64` | DFLE | `biology::aging::senescence` |

#### telomeres.rs (9 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `telomere_attrition` | `fn telomere_attrition(initial_length: f64, loss_per_division: f64, divisions: usize) → f64` | Telomere attrition | `biology::aging::telomeres` |
| `telomerase_elongation` | `fn telomerase_elongation(current_length: f64, rate: f64, telomerase_activity: f64) → f64` | Telomerase elongation | `biology::aging::telomeres` |
| `replicative_limit` | `fn replicative_limit(initial_length: f64, loss_per_division: f64, critical_length: f64) → f64` | Replicative limit | `biology::aging::telomeres` |
| `telomere_length_distribution` | `fn telomere_length_distribution(mean: f64, std_dev: f64, n_chromosomes: usize) → Vec<f64>` | Length distribution | `biology::aging::telomeres` |
| `critical_shortening_probability` | `fn critical_shortening_probability(mean_length: f64, std_dev: f64, critical: f64) → f64` | Critical shortening prob | `biology::aging::telomeres` |
| `shelterin_protection` | `fn shelterin_protection(telomere_length: f64, shelterin_kd: f64) → f64` | Shelterin protection | `biology::aging::telomeres` |
| `end_replication_problem` | `fn end_replication_problem(lagging_strand_loss: f64, divisions: usize) → f64` | End replication problem | `biology::aging::telomeres` |
| `alt_pathway_elongation` | `fn alt_pathway_elongation(recombination_rate: f64, donor_length: f64, recipient_length: f64) → f64` | ALT elongation | `biology::aging::telomeres` |
| `telomere_age_regression` | `fn telomere_age_regression(age: f64, birth_length: f64, annual_loss: f64) → f64` | Telomere-age regression | `biology::aging::telomeres` |

---

### 3️⃣8️⃣ stem_cell/ — Stem Cells — 50 pub fn

#### differentiation.rs (9 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `waddington_landscape_potential` | `fn waddington_landscape_potential(state: f64, attractor_a: f64, attractor_b: f64, barrier: f64) → f64` | Waddington landscape | `biology::stem_cell::differentiation` |
| `differentiation_commitment` | `fn differentiation_commitment(transcription_factor_a: f64, transcription_factor_b: f64, hill: f64) → f64` | Differentiation commitment | `biology::stem_cell::differentiation` |
| `lineage_progression` | `fn lineage_progression(progenitor: f64, differentiation_rate: f64, proliferation_rate: f64, dt: f64) → (f64, f64)` | Lineage progression | `biology::stem_cell::differentiation` |
| `multipotency_index` | `fn multipotency_index(expressed_lineage_genes: &[f64]) → f64` | Multipotency index | `biology::stem_cell::differentiation` |
| `cell_fate_probability_stochastic` | `fn cell_fate_probability_stochastic(tf_level: f64, noise: f64, threshold: f64) → f64` | Stochastic cell fate | `biology::stem_cell::differentiation` |
| `directed_differentiation_efficiency` | `fn directed_differentiation_efficiency(target_markers: f64, total_cells: f64) → f64` | Directed differentiation | `biology::stem_cell::differentiation` |
| `transdifferentiation_barrier` | `fn transdifferentiation_barrier(epigenetic_distance: f64, reprogramming_factors: f64, efficiency_base: f64) → f64` | Transdifferentiation barrier | `biology::stem_cell::differentiation` |
| `organoid_differentiation_layers` | `fn organoid_differentiation_layers(time: f64, layer_rate: f64, max_layers: f64) → f64` | Organoid layers | `biology::stem_cell::differentiation` |
| `terminal_differentiation_irreversibility` | `fn terminal_differentiation_irreversibility(rb_phosphorylation: f64, cdki_level: f64) → f64` | Terminal differentiation | `biology::stem_cell::differentiation` |

#### dynamics.rs (16 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `self_renewal_probability` | `fn self_renewal_probability(symmetric_rate: f64, total_division_rate: f64) → f64` | Self-renewal probability | `biology::stem_cell::dynamics` |
| `stem_cell_pool_dynamics` | `fn stem_cell_pool_dynamics(s: f64, r: f64, d: f64, p: f64, dt: f64) → f64` | Pool dynamics | `biology::stem_cell::dynamics` |
| `asymmetric_division_output` | `fn asymmetric_division_output(stem_cells: f64, division_rate: f64, asymmetric_fraction: f64) → f64` | Asymmetric division | `biology::stem_cell::dynamics` |
| `lineage_commitment` | `fn lineage_commitment(signal_strength: f64, threshold: f64, hill_n: f64) → f64` | Lineage commitment | `biology::stem_cell::dynamics` |
| `niche_occupancy` | `fn niche_occupancy(stem_cells: f64, niche_capacity: f64) → f64` | Niche occupancy | `biology::stem_cell::dynamics` |
| `niche_competition` | `fn niche_competition(resident: f64, challenger: f64, fitness_resident: f64, fitness_challenger: f64) → f64` | Niche competition | `biology::stem_cell::dynamics` |
| `dedifferentiation_rate` | `fn dedifferentiation_rate(injury_signal: f64, plasticity: f64, baseline: f64) → f64` | Dedifferentiation rate | `biology::stem_cell::dynamics` |
| `stem_cell_aging` | `fn stem_cell_aging(initial_pool: f64, depletion_rate: f64, age: f64) → f64` | Stem cell aging | `biology::stem_cell::dynamics` |
| `transit_amplifying_generations` | `fn transit_amplifying_generations(progenitor: f64, divisions: u32, survival_per_div: f64) → f64` | Transit amplifying | `biology::stem_cell::dynamics` |
| `quiescence_exit_rate` | `fn quiescence_exit_rate(growth_factor: f64, threshold: f64, max_rate: f64) → f64` | Quiescence exit | `biology::stem_cell::dynamics` |
| `clonal_dominance` | `fn clonal_dominance(fitness: &[f64]) → Vec<f64>` | Clonal dominance | `biology::stem_cell::dynamics` |
| `neutral_drift_clone_survival` | `fn neutral_drift_clone_survival(initial_clones: f64, time: f64, replacement_rate: f64) → f64` | Neutral drift | `biology::stem_cell::dynamics` |
| `hematopoietic_hierarchy_output` | `fn hematopoietic_hierarchy_output(hsc: f64, mpp_rate: f64, clp_rate: f64, cmp_rate: f64) → (f64, f64)` | HSC hierarchy | `biology::stem_cell::dynamics` |
| `telomere_shortening_per_division` | `fn telomere_shortening_per_division(initial_length: f64, loss_per_division: f64, divisions: f64) → f64` | Telomere shortening | `biology::stem_cell::dynamics` |
| `hayflick_limit_remaining` | `fn hayflick_limit_remaining(telomere_length: f64, critical_length: f64, loss_per_division: f64) → f64` | Hayflick limit | `biology::stem_cell::dynamics` |
| `symmetric_commitment_probability` | `fn symmetric_commitment_probability(niche_signal: f64, k_niche: f64) → f64` | Symmetric commitment | `biology::stem_cell::dynamics` |

#### niche.rs (10 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `stem_cell_niche_occupancy` | `fn stem_cell_niche_occupancy(stem_cells: f64, niche_capacity: f64, adhesion_strength: f64) → f64` | Niche occupancy | `biology::stem_cell::niche` |
| `niche_signal_gradient` | `fn niche_signal_gradient(source_strength: f64, distance: f64, decay_length: f64) → f64` | Signal gradient | `biology::stem_cell::niche` |
| `quiescence_probability` | `fn quiescence_probability(niche_signal: f64, threshold: f64) → f64` | Quiescence probability | `biology::stem_cell::niche` |
| `niche_asymmetric_division` | `fn niche_asymmetric_division(niche_polarization: f64, cell_polarity: f64) → f64` | Asymmetric division | `biology::stem_cell::niche` |
| `hematopoietic_niche_osteoblast` | `fn hematopoietic_niche_osteoblast(osteoblast_count: f64, hsc_supported: f64, max_ratio: f64) → f64` | Osteoblastic niche | `biology::stem_cell::niche` |
| `perivascular_niche_oxygen` | `fn perivascular_niche_oxygen(distance_from_vessel: f64, vessel_po2: f64, consumption_rate: f64, diffusion: f64) → f64` | Perivascular O₂ | `biology::stem_cell::niche` |
| `intestinal_crypt_dynamics` | `fn intestinal_crypt_dynamics(stem_cells: f64, division_rate: f64, loss_rate: f64, niche_capacity: f64, dt: f64) → f64` | Intestinal crypt | `biology::stem_cell::niche` |
| `wnt_gradient_crypt` | `fn wnt_gradient_crypt(position: f64, crypt_depth: f64, wnt_max: f64) → f64` | Wnt gradient | `biology::stem_cell::niche` |
| `notch_lateral_inhibition_niche` | `fn notch_lateral_inhibition_niche(notch_signal: f64, delta_neighbors: f64, gain: f64) → f64` | Notch lateral inhibition | `biology::stem_cell::niche` |
| `mesenchymal_niche_paracrine` | `fn mesenchymal_niche_paracrine(mscs: f64, growth_factor_per_cell: f64, distance: f64, decay: f64) → f64` | Paracrine signaling | `biology::stem_cell::niche` |

#### reprogramming.rs (15 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `reprogramming_efficiency` | `fn reprogramming_efficiency(oct4: f64, sox2: f64, klf4: f64, myc: f64, epigenetic_barrier: f64) → f64` | Reprogramming efficiency | `biology::stem_cell::reprogramming` |
| `ipsc_colony_formation` | `fn ipsc_colony_formation(seeded_cells: f64, reprogramming_efficiency: f64, survival_fraction: f64) → f64` | iPSC colony formation | `biology::stem_cell::reprogramming` |
| `differentiation_cascade` | `fn differentiation_cascade(progenitor: f64, rates: &[f64]) → Vec<f64>` | Differentiation cascade | `biology::stem_cell::reprogramming` |
| `waddington_potential` | `fn waddington_potential(state: f64, landscape: impl Fn(f64) → f64, noise: f64) → f64` | Waddington potential | `biology::stem_cell::reprogramming` |
| `organoid_growth` | `fn organoid_growth(cells: f64, growth_rate: f64, carrying_capacity: f64, dt: f64) → f64` | Organoid growth | `biology::stem_cell::reprogramming` |
| `yamanaka_factor_dynamics` | `fn yamanaka_factor_dynamics(oct4: f64, sox2: f64, klf4: f64, myc: f64, dt: f64, degradation: f64) → (f64, f64, f64, f64)` | Yamanaka factors | `biology::stem_cell::reprogramming` |
| `stochastic_reprogramming_events` | `fn stochastic_reprogramming_events(cells: usize, probability_per_cell: f64) → f64` | Stochastic reprogramming | `biology::stem_cell::reprogramming` |
| `partial_reprogramming_state` | `fn partial_reprogramming_state(methylation_age: f64, cycles: usize, reset_per_cycle: f64) → f64` | Partial reprogramming | `biology::stem_cell::reprogramming` |
| `direct_lineage_conversion` | `fn direct_lineage_conversion(efficiency_base: f64, tf_combination: &[f64], synergy: f64) → f64` | Direct lineage conversion | `biology::stem_cell::reprogramming` |
| `asymmetric_division_ratio` | `fn asymmetric_division_ratio(stem_cells: f64, symmetric_prob: f64, differentiation_rate: f64) → (f64, f64)` | Asymmetric division ratio | `biology::stem_cell::reprogramming` |
| `epigenetic_barrier_height` | `fn epigenetic_barrier_height(methylation_level: f64, histone_marks: f64, chromatin_accessibility: f64) → f64` | Epigenetic barrier | `biology::stem_cell::reprogramming` |
| `crispr_activation_efficiency` | `fn crispr_activation_efficiency(guide_specificity: f64, activator_strength: f64, chromatin_state: f64) → f64` | CRISPRa efficiency | `biology::stem_cell::reprogramming` |
| `embryoid_body_formation` | `fn embryoid_body_formation(single_cells: f64, aggregation_rate: f64, min_cells_per_eb: f64) → f64` | Embryoid body formation | `biology::stem_cell::reprogramming` |
| `directed_differentiation_yield` | `fn directed_differentiation_yield(input_cells: f64, protocol_efficiency: f64, purity: f64) → f64` | Differentiation yield | `biology::stem_cell::reprogramming` |
| `maturation_index` | `fn maturation_index(marker_expression: &[f64], weights: &[f64]) → f64` | Maturation index | `biology::stem_cell::reprogramming` |

---

### 3️⃣9️⃣ tissue_engineering/ — Tissue Engineering — 60 pub fn

#### bioreactor.rs (11 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `perfusion_rate` | `fn perfusion_rate(flow_ml_min: f64, volume_ml: f64) → f64` | Perfusion rate | `biology::tissue_engineering::bioreactor` |
| `shear_stress_parallel_plate` | `fn shear_stress_parallel_plate(viscosity: f64, flow_rate: f64, width: f64, height: f64) → f64` | Parallel plate shear | `biology::tissue_engineering::bioreactor` |
| `oxygen_transfer_rate` | `fn oxygen_transfer_rate(kla: f64, c_star: f64, c_bulk: f64) → f64` | OTR | `biology::tissue_engineering::bioreactor` |
| `residence_time` | `fn residence_time(volume_ml: f64, flow_rate_ml_min: f64) → f64` | Residence time | `biology::tissue_engineering::bioreactor` |
| `spinner_flask_shear` | `fn spinner_flask_shear(viscosity: f64, rpm: f64, radius: f64) → f64` | Spinner flask shear | `biology::tissue_engineering::bioreactor` |
| `hollow_fiber_mass_transfer` | `fn hollow_fiber_mass_transfer(permeability: f64, surface_area: f64, delta_c: f64) → f64` | Hollow fiber transfer | `biology::tissue_engineering::bioreactor` |
| `bioreactor_seeding_efficiency` | `fn bioreactor_seeding_efficiency(attached: f64, seeded: f64) → f64` | Seeding efficiency | `biology::tissue_engineering::bioreactor` |
| `scaffold_porosity` | `fn scaffold_porosity(void_volume: f64, total_volume: f64) → f64` | Scaffold porosity | `biology::tissue_engineering::bioreactor` |
| `degradation_rate_first_order` | `fn degradation_rate_first_order(mass: f64, k_deg: f64, time: f64) → f64` | First-order degradation | `biology::tissue_engineering::bioreactor` |
| `mechanical_modulus_scaffold` | `fn mechanical_modulus_scaffold(stress: f64, strain: f64) → f64` | Mechanical modulus | `biology::tissue_engineering::bioreactor` |
| `cell_proliferation_on_scaffold` | `fn cell_proliferation_on_scaffold(n0: f64, doubling_time: f64, elapsed: f64) → f64` | Scaffold proliferation | `biology::tissue_engineering::bioreactor` |

#### scaffold.rs (22 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `scaffold_degradation_bulk` | `fn scaffold_degradation_bulk(m0: f64, k: f64, t: f64) → f64` | Bulk degradation | `biology::tissue_engineering::scaffold` |
| `scaffold_degradation_surface` | `fn scaffold_degradation_surface(m0: f64, rate: f64, surface_area: f64, t: f64) → f64` | Surface degradation | `biology::tissue_engineering::scaffold` |
| `porosity` | `fn porosity(void_volume: f64, total_volume: f64) → f64` | Porosity | `biology::tissue_engineering::scaffold` |
| `pore_interconnectivity` | `fn pore_interconnectivity(connected_pores: f64, total_pores: f64) → f64` | Pore interconnectivity | `biology::tissue_engineering::scaffold` |
| `mechanical_stiffness_degrading` | `fn mechanical_stiffness_degrading(e0: f64, degradation_fraction: f64, n: f64) → f64` | Degrading stiffness | `biology::tissue_engineering::scaffold` |
| `scaffold_swelling_ratio` | `fn scaffold_swelling_ratio(wet_mass: f64, dry_mass: f64) → f64` | Swelling ratio | `biology::tissue_engineering::scaffold` |
| `cell_seeding_efficiency` | `fn cell_seeding_efficiency(attached: f64, seeded: f64) → f64` | Seeding efficiency | `biology::tissue_engineering::scaffold` |
| `nutrient_diffusion_depth` | `fn nutrient_diffusion_depth(diffusivity: f64, consumption_rate: f64, surface_concentration: f64) → f64` | Diffusion depth | `biology::tissue_engineering::scaffold` |
| `construct_viability` | `fn construct_viability(viable_cells: f64, total_cells: f64) → f64` | Construct viability | `biology::tissue_engineering::scaffold` |
| `bioreactor_shear_stress` | `fn bioreactor_shear_stress(flow_rate: f64, viscosity: f64, channel_height: f64, channel_width: f64) → f64` | Bioreactor shear | `biology::tissue_engineering::scaffold` |
| `fiber_diameter_electrospinning` | `fn fiber_diameter_electrospinning(voltage: f64, flow_rate: f64, concentration: f64, distance: f64) → f64` | Electrospinning fiber | `biology::tissue_engineering::scaffold` |
| `scaffold_compressive_modulus` | `fn scaffold_compressive_modulus(stress: f64, strain: f64) → f64` | Compressive modulus | `biology::tissue_engineering::scaffold` |
| `scaffold_tortuosity` | `fn scaffold_tortuosity(path_length: f64, straight_distance: f64) → f64` | Tortuosity | `biology::tissue_engineering::scaffold` |
| `specific_surface_area` | `fn specific_surface_area(surface_area: f64, volume: f64) → f64` | Specific surface area | `biology::tissue_engineering::scaffold` |
| `kozeny_carman_permeability` | `fn kozeny_carman_permeability(porosity_frac: f64, pore_diameter: f64) → f64` | Kozeny-Carman | `biology::tissue_engineering::scaffold` |
| `hydrogel_crosslink_density` | `fn hydrogel_crosslink_density(shear_modulus: f64, temperature: f64) → f64` | Crosslink density | `biology::tissue_engineering::scaffold` |
| `hydrogel_mesh_size` | `fn hydrogel_mesh_size(crosslink_density: f64, cn: f64, bond_length: f64, mr: f64, polymer_density: f64) → f64` | Mesh size | `biology::tissue_engineering::scaffold` |
| `scaffold_surface_energy` | `fn scaffold_surface_energy(contact_angle_deg: f64, liquid_tension: f64) → f64` | Surface energy | `biology::tissue_engineering::scaffold` |
| `drug_release_higuchi` | `fn drug_release_higuchi(k_h: f64, t: f64) → f64` | Higuchi release $Q = k_H\sqrt{t}$ | `biology::tissue_engineering::scaffold` |
| `drug_release_korsmeyer_peppas` | `fn drug_release_korsmeyer_peppas(k: f64, t: f64, n: f64) → f64` | Korsmeyer-Peppas | `biology::tissue_engineering::scaffold` |
| `degradation_molecular_weight` | `fn degradation_molecular_weight(mw0: f64, k: f64, t: f64) → f64` | MW degradation | `biology::tissue_engineering::scaffold` |
| `autocatalytic_degradation` | `fn autocatalytic_degradation(mw0: f64, k: f64, acid_conc: f64, t: f64) → f64` | Autocatalytic degradation | `biology::tissue_engineering::scaffold` |

#### tissue.rs (18 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `cell_proliferation_3d` | `fn cell_proliferation_3d(cells: f64, growth_rate: f64, nutrient_factor: f64, carrying_capacity: f64, dt: f64) → f64` | 3D proliferation | `biology::tissue_engineering::tissue` |
| `oxygen_diffusion_krogh` | `fn oxygen_diffusion_krogh(r_tissue: f64, r_capillary: f64, consumption_rate: f64, diffusivity: f64, c_surface: f64) → f64` | Krogh cylinder | `biology::tissue_engineering::tissue` |
| `nutrient_penetration_depth` | `fn nutrient_penetration_depth(diffusivity: f64, surface_concentration: f64, consumption_rate: f64) → f64` | Penetration depth | `biology::tissue_engineering::tissue` |
| `perfusion_bioreactor_shear` | `fn perfusion_bioreactor_shear(viscosity: f64, flow_rate: f64, channel_height: f64, channel_width: f64) → f64` | Perfusion shear | `biology::tissue_engineering::tissue` |
| `cell_migration_speed` | `fn cell_migration_speed(traction_force: f64, drag_coefficient: f64) → f64` | Migration speed | `biology::tissue_engineering::tissue` |
| `tissue_maturation_index` | `fn tissue_maturation_index(collagen_content: f64, target_collagen: f64, mechanical_strength: f64, target_strength: f64) → f64` | Maturation index | `biology::tissue_engineering::tissue` |
| `vascularization_density` | `fn vascularization_density(vessel_length: f64, tissue_volume: f64) → f64` | Vessel density | `biology::tissue_engineering::tissue` |
| `extracellular_matrix_production` | `fn extracellular_matrix_production(cell_density: f64, production_rate: f64, stimulus: f64, degradation_rate: f64, ecm: f64, dt: f64) → f64` | ECM production | `biology::tissue_engineering::tissue` |
| `cell_sheet_contraction` | `fn cell_sheet_contraction(initial_area: f64, contractility: f64, cell_density: f64, t: f64) → f64` | Cell sheet contraction | `biology::tissue_engineering::tissue` |
| `mechanotransduction_response` | `fn mechanotransduction_response(strain: f64, threshold: f64, sensitivity: f64) → f64` | Mechanotransduction | `biology::tissue_engineering::tissue` |
| `angiogenic_sprouting_rate` | `fn angiogenic_sprouting_rate(vegf: f64, k_vegf: f64, tip_cell_density: f64) → f64` | Angiogenic sprouting | `biology::tissue_engineering::tissue` |
| `tissue_compaction` | `fn tissue_compaction(initial_volume: f64, cell_traction: f64, matrix_stiffness: f64, t: f64) → f64` | Tissue compaction | `biology::tissue_engineering::tissue` |
| `cell_differentiation_rate` | `fn cell_differentiation_rate(transcription_factor: f64, threshold: f64, hill: f64) → f64` | Differentiation rate | `biology::tissue_engineering::tissue` |
| `wound_healing_closure` | `fn wound_healing_closure(gap_width: f64, migration_speed: f64, proliferation_rate: f64, t: f64) → f64` | Wound healing | `biology::tissue_engineering::tissue` |
| `nutrient_consumption_michaelis` | `fn nutrient_consumption_michaelis(concentration: f64, vmax: f64, km: f64) → f64` | Michaelis-Menten | `biology::tissue_engineering::tissue` |
| `cell_viability_hypoxia` | `fn cell_viability_hypoxia(po2: f64, critical_po2: f64, sensitivity: f64) → f64` | Hypoxia viability | `biology::tissue_engineering::tissue` |
| `collagen_fiber_alignment` | `fn collagen_fiber_alignment(strain_direction: f64, fiber_angle: f64) → f64` | Collagen alignment | `biology::tissue_engineering::tissue` |
| `gag_content_cartilage` | `fn gag_content_cartilage(cell_density: f64, tgf_beta: f64, production_rate: f64, degradation_rate: f64, current: f64, dt: f64) → f64` | GAG content | `biology::tissue_engineering::tissue` |

#### vascularization.rs (9 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `vegf_diffusion_concentration` | `fn vegf_diffusion_concentration(source_concentration: f64, distance: f64, diffusion_length: f64) → f64` | VEGF diffusion | `biology::tissue_engineering::vascularization` |
| `sprouting_probability` | `fn sprouting_probability(vegf_concentration: f64, threshold: f64, hill_n: f64) → f64` | Sprouting probability | `biology::tissue_engineering::vascularization` |
| `vessel_spacing_optimal` | `fn vessel_spacing_optimal(oxygen_diffusion_coeff: f64, consumption_rate: f64, po2_arterial: f64) → f64` | Optimal vessel spacing | `biology::tissue_engineering::vascularization` |
| `krogh_cylinder_po2` | `fn krogh_cylinder_po2(po2_arterial: f64, consumption_rate: f64, diff_coeff: f64, r: f64, r_capillary: f64) → f64` | Krogh cylinder pO₂ | `biology::tissue_engineering::vascularization` |
| `angiogenic_switch_balance` | `fn angiogenic_switch_balance(pro_angiogenic: f64, anti_angiogenic: f64) → f64` | Angiogenic switch | `biology::tissue_engineering::vascularization` |
| `capillary_density` | `fn capillary_density(num_capillaries: f64, tissue_area_mm2: f64) → f64` | Capillary density | `biology::tissue_engineering::vascularization` |
| `endothelial_migration_speed` | `fn endothelial_migration_speed(chemotactic_gradient: f64, sensitivity: f64, max_speed: f64) → f64` | Endothelial migration | `biology::tissue_engineering::vascularization` |
| `prevascularization_survival` | `fn prevascularization_survival(distance_to_vessel_um: f64, max_diffusion_distance: f64) → f64` | Prevascularization survival | `biology::tissue_engineering::vascularization` |
| `microvessel_wall_shear_stress` | `fn microvessel_wall_shear_stress(viscosity: f64, flow_rate: f64, radius: f64) → f64` | Microvessel shear | `biology::tissue_engineering::vascularization` |

---

### 4️⃣0️⃣ radiobiology/ — Radiobiology — 57 pub fn

#### dna_damage.rs (19 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `dna_strand_break_probability` | `fn dna_strand_break_probability(dose: f64, target_size: f64, repair_efficiency: f64) → f64` | Strand break probability | `biology::radiobiology::dna_damage` |
| `base_excision_repair_rate` | `fn base_excision_repair_rate(damage_sites: f64, enzyme_concentration: f64, km: f64) → f64` | BER rate | `biology::radiobiology::dna_damage` |
| `misrepair_probability` | `fn misrepair_probability(damage_density: f64, complexity_factor: f64) → f64` | Misrepair probability | `biology::radiobiology::dna_damage` |
| `chromosome_aberration_yield` | `fn chromosome_aberration_yield(dose: f64, alpha: f64, beta: f64) → f64` | Aberration yield | `biology::radiobiology::dna_damage` |
| `lethal_aberration_fraction` | `fn lethal_aberration_fraction(aberrations: f64) → f64` | Lethal aberrations | `biology::radiobiology::dna_damage` |
| `mutation_frequency` | `fn mutation_frequency(dose: f64, spontaneous_rate: f64, induced_rate_per_gy: f64) → f64` | Mutation frequency | `biology::radiobiology::dna_damage` |
| `double_strand_break_yield` | `fn double_strand_break_yield(dose: f64, let_factor: f64) → f64` | DSB yield | `biology::radiobiology::dna_damage` |
| `nhej_repair_kinetics` | `fn nhej_repair_kinetics(breaks: f64, fast_rate: f64, slow_rate: f64, fast_fraction: f64, t: f64) → f64` | NHEJ kinetics | `biology::radiobiology::dna_damage` |
| `homologous_recombination_probability` | `fn homologous_recombination_probability(cell_cycle_s_g2_fraction: f64, sister_chromatid_available: bool) → f64` | HR probability | `biology::radiobiology::dna_damage` |
| `clustered_damage_probability` | `fn clustered_damage_probability(dose: f64, let_val: f64, target_radius: f64) → f64` | Clustered damage | `biology::radiobiology::dna_damage` |
| `single_strand_break_yield` | `fn single_strand_break_yield(dose: f64) → f64` | SSB yield | `biology::radiobiology::dna_damage` |
| `oxidative_base_damage_yield` | `fn oxidative_base_damage_yield(dose: f64, oxygen_concentration: f64) → f64` | Oxidative base damage | `biology::radiobiology::dna_damage` |
| `dna_damage_complexity_score` | `fn dna_damage_complexity_score(ssb: f64, dsb: f64, base_damage: f64) → f64` | Damage complexity | `biology::radiobiology::dna_damage` |
| `foci_formation_kinetics` | `fn foci_formation_kinetics(dsb: f64, recruitment_rate: f64, t: f64) → f64` | Foci formation | `biology::radiobiology::dna_damage` |
| `foci_resolution_kinetics` | `fn foci_resolution_kinetics(foci_max: f64, repair_rate: f64, t: f64) → f64` | Foci resolution | `biology::radiobiology::dna_damage` |
| `micronucleus_formation` | `fn micronucleus_formation(dose: f64, alpha_mn: f64, beta_mn: f64) → f64` | Micronucleus formation | `biology::radiobiology::dna_damage` |
| `comet_tail_moment` | `fn comet_tail_moment(tail_length: f64, tail_dna_fraction: f64) → f64` | Comet assay | `biology::radiobiology::dna_damage` |
| `gamma_h2ax_signal` | `fn gamma_h2ax_signal(dsb: f64, spreading_factor: f64, background: f64) → f64` | γ-H2AX signal | `biology::radiobiology::dna_damage` |
| `repair_pathway_choice` | `fn repair_pathway_choice(dsb: f64, cell_cycle_phase: f64, brca_status: f64) → (f64, f64)` | Repair pathway choice | `biology::radiobiology::dna_damage` |

#### dose_response.rs (18 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `linear_quadratic_survival` | `fn linear_quadratic_survival(dose: f64, alpha: f64, beta: f64) → f64` | $SF = e^{-\alpha D - \beta D^2}$ | `biology::radiobiology::dose_response` |
| `biologically_effective_dose` | `fn biologically_effective_dose(n: f64, d: f64, alpha_beta: f64) → f64` | BED | `biology::radiobiology::dose_response` |
| `equivalent_dose_2gy` | `fn equivalent_dose_2gy(bed: f64, alpha_beta: f64) → f64` | EQD2 | `biology::radiobiology::dose_response` |
| `tcp` | `fn tcp(n_cells: f64, survival_fraction: f64) → f64` | TCP | `biology::radiobiology::dose_response` |
| `ntcp_lyman` | `fn ntcp_lyman(dose: f64, td50: f64, m: f64) → f64` | Lyman NTCP | `biology::radiobiology::dose_response` |
| `oxygen_enhancement_ratio` | `fn oxygen_enhancement_ratio(dose_hypoxic: f64, dose_oxic: f64) → f64` | OER | `biology::radiobiology::dose_response` |
| `dna_dsb_yield` | `fn dna_dsb_yield(dose: f64, yield_per_gray: f64) → f64` | DSB yield | `biology::radiobiology::dose_response` |
| `repair_kinetics` | `fn repair_kinetics(dsb0: f64, repair_rate: f64, t: f64) → f64` | Repair kinetics | `biology::radiobiology::dose_response` |
| `fractionation_survival` | `fn fractionation_survival(n_fractions: usize, dose_per_fraction: f64, alpha: f64, beta: f64, repair_factor: f64) → f64` | Fractionation survival | `biology::radiobiology::dose_response` |
| `relative_biological_effectiveness` | `fn relative_biological_effectiveness(dose_ref: f64, dose_test: f64) → f64` | RBE | `biology::radiobiology::dose_response` |
| `let_to_rbe` | `fn let_to_rbe(let_val: f64, rbe_max: f64, let_half: f64) → f64` | LET-to-RBE | `biology::radiobiology::dose_response` |
| `protraction_factor` | `fn protraction_factor(dose_rate: f64, repair_half_time: f64, total_time: f64) → f64` | Protraction factor | `biology::radiobiology::dose_response` |
| `bystander_effect` | `fn bystander_effect(dose: f64, max_effect: f64, dose_half: f64) → f64` | Bystander effect | `biology::radiobiology::dose_response` |
| `adaptive_response` | `fn adaptive_response(priming_dose: f64, challenge_dose: f64, alpha: f64, beta: f64, reduction_factor: f64) → f64` | Adaptive response | `biology::radiobiology::dose_response` |
| `low_dose_hypersensitivity` | `fn low_dose_hypersensitivity(dose: f64, alpha_r: f64, alpha_s: f64, dc: f64, beta: f64) → f64` | HRS/IRR | `biology::radiobiology::dose_response` |
| `tumor_growth_delay` | `fn tumor_growth_delay(dose: f64, alpha: f64, beta: f64, doubling_time: f64) → f64` | Tumor growth delay | `biology::radiobiology::dose_response` |
| `complication_free_cure` | `fn complication_free_cure(tcp_val: f64, ntcp_val: f64) → f64` | P+ = TCP(1−NTCP) | `biology::radiobiology::dose_response` |
| `isoeffect_dose` | `fn isoeffect_dose(n1: f64, d1: f64, alpha_beta: f64, n2: f64) → f64` | Isoeffect dose | `biology::radiobiology::dose_response` |

#### fractionation.rs (10 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `bed_biologically_effective_dose` | `fn bed_biologically_effective_dose(n: f64, d: f64, alpha_beta: f64) → f64` | BED | `biology::radiobiology::fractionation` |
| `eqd2` | `fn eqd2(n: f64, d: f64, alpha_beta: f64) → f64` | EQD2 | `biology::radiobiology::fractionation` |
| `tumor_control_probability` | `fn tumor_control_probability(n_clonogens: f64, surviving_fraction: f64) → f64` | TCP | `biology::radiobiology::fractionation` |
| `normal_tissue_complication_probability` | `fn normal_tissue_complication_probability(dose: f64, td50: f64, gamma50: f64) → f64` | NTCP | `biology::radiobiology::fractionation` |
| `incomplete_repair_factor` | `fn incomplete_repair_factor(delta_t: f64, repair_half_time: f64) → f64` | Incomplete repair | `biology::radiobiology::fractionation` |
| `repopulation_dose_equivalent` | `fn repopulation_dose_equivalent(doubling_time: f64, treatment_duration: f64, alpha: f64) → f64` | Repopulation dose | `biology::radiobiology::fractionation` |
| `lq_with_repopulation` | `fn lq_with_repopulation(alpha: f64, beta: f64, dose: f64, n_fractions: f64, treatment_days: f64, tp: f64, tk: f64) → f64` | LQ + repopulation | `biology::radiobiology::fractionation` |
| `therapeutic_ratio` | `fn therapeutic_ratio(tcp: f64, ntcp: f64) → f64` | Therapeutic ratio | `biology::radiobiology::fractionation` |
| `fraction_size_optimization` | `fn fraction_size_optimization(alpha_beta_tumor: f64, alpha_beta_normal: f64) → f64` | Fraction optimization | `biology::radiobiology::fractionation` |
| `hyperfractionation_advantage` | `fn hyperfractionation_advantage(d_conventional: f64, d_hyper: f64, alpha_beta: f64) → f64` | Hyperfractionation | `biology::radiobiology::fractionation` |

#### shielding.rs (10 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `radiation_shielding_half_value` | `fn radiation_shielding_half_value(initial_intensity: f64, hvl: f64, thickness: f64) → f64` | HVL shielding | `biology::radiobiology::shielding` |
| `shielding_tenth_value` | `fn shielding_tenth_value(initial_intensity: f64, tvl: f64, thickness: f64) → f64` | TVL shielding | `biology::radiobiology::shielding` |
| `mass_attenuation` | `fn mass_attenuation(intensity: f64, mu_over_rho: f64, density: f64, thickness: f64) → f64` | Mass attenuation | `biology::radiobiology::shielding` |
| `buildup_factor` | `fn buildup_factor(beam_layers: f64, mu: f64, thickness: f64) → f64` | Buildup factor | `biology::radiobiology::shielding` |
| `concrete_shielding_thickness` | `fn concrete_shielding_thickness(dose_rate: f64, dose_limit: f64, hvl: f64) → f64` | Concrete shielding | `biology::radiobiology::shielding` |
| `lead_equivalent_thickness` | `fn lead_equivalent_thickness(mu_material: f64, mu_lead: f64, thickness_material: f64) → f64` | Lead equivalent | `biology::radiobiology::shielding` |
| `inverse_square_distance` | `fn inverse_square_distance(dose_at_d1: f64, d1: f64, d2: f64) → f64` | Inverse square law | `biology::radiobiology::shielding` |
| `occupancy_factor_dose` | `fn occupancy_factor_dose(dose_unshielded: f64, occupancy: f64, use_factor: f64) → f64` | Occupancy factor | `biology::radiobiology::shielding` |
| `neutron_shielding_hydrogen` | `fn neutron_shielding_hydrogen(thickness_cm: f64, cross_section: f64, density_h: f64) → f64` | Neutron shielding | `biology::radiobiology::shielding` |
| `annual_dose_limit_check` | `fn annual_dose_limit_check(dose_received: f64, dose_limit: f64) → f64` | Dose limit check | `biology::radiobiology::shielding` |

---

### 4️⃣1️⃣ cryobiology/ — Cryobiology — 46 pub fn

#### freezing.rs (19 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `mazur_two_factor_model` | `fn mazur_two_factor_model(cooling_rate: f64, optimal_rate: f64, width: f64) → f64` | Mazur two-factor model | `biology::cryobiology::freezing` |
| `ice_nucleation_rate` | `fn ice_nucleation_rate(temperature: f64, volume: f64, j0: f64, activation_energy: f64) → f64` | Ice nucleation rate | `biology::cryobiology::freezing` |
| `critical_cooling_rate` | `fn critical_cooling_rate(cpa_concentration: f64, base_rate: f64, sensitivity: f64) → f64` | Critical cooling rate | `biology::cryobiology::freezing` |
| `vitrification_probability` | `fn vitrification_probability(cooling_rate: f64, critical_rate: f64) → f64` | Vitrification probability | `biology::cryobiology::freezing` |
| `cpa_toxicity` | `fn cpa_toxicity(concentration: f64, exposure_time: f64, k_tox: f64) → f64` | CPA toxicity | `biology::cryobiology::freezing` |
| `cell_volume_response` | `fn cell_volume_response(v0: f64, osmolarity_ratio: f64, vb: f64) → f64` | Cell volume response | `biology::cryobiology::freezing` |
| `freeze_thaw_survival` | `fn freeze_thaw_survival(initial_viability: f64, ice_damage: f64, osmotic_damage: f64, cpa_damage: f64) → f64` | Freeze-thaw survival | `biology::cryobiology::freezing` |
| `intracellular_ice_formation_probability` | `fn intracellular_ice_formation_probability(cooling_rate: f64, critical_rate: f64, n: f64) → f64` | IIF probability | `biology::cryobiology::freezing` |
| `osmotic_tolerance_limit` | `fn osmotic_tolerance_limit(v_min: f64, v_max: f64, initial_volume: f64) → (f64, f64)` | Osmotic tolerance | `biology::cryobiology::freezing` |
| `kedem_katchalsky_water_flux` | `fn kedem_katchalsky_water_flux(lp: f64, area: f64, delta_pi: f64, sigma: f64, delta_p: f64) → f64` | K-K water flux | `biology::cryobiology::freezing` |
| `kedem_katchalsky_solute_flux` | `fn kedem_katchalsky_solute_flux(ps: f64, area: f64, delta_c: f64, sigma: f64, jv: f64, c_mean: f64) → f64` | K-K solute flux | `biology::cryobiology::freezing` |
| `freezing_point_depression` | `fn freezing_point_depression(concentration: f64, kf: f64, dissociation_factor: f64) → f64` | $\Delta T_f = K_f m i$ | `biology::cryobiology::freezing` |
| `hemolysis_fraction` | `fn hemolysis_fraction(osmolality: f64, half_lysis_osmolality: f64, steepness: f64) → f64` | Hemolysis fraction | `biology::cryobiology::freezing` |
| `stefan_freezing_front` | `fn stefan_freezing_front(thermal_diffusivity: f64, t: f64, stefan_number: f64) → f64` | Stefan problem | `biology::cryobiology::freezing` |
| `supercooling_degree` | `fn supercooling_degree(freezing_point: f64, nucleation_temp: f64) → f64` | Supercooling degree | `biology::cryobiology::freezing` |
| `ice_crystal_growth_rate` | `fn ice_crystal_growth_rate(supercooling: f64, diffusivity: f64, activation_energy: f64, temperature: f64) → f64` | Ice crystal growth | `biology::cryobiology::freezing` |
| `cpa_loading_protocol_step` | `fn cpa_loading_protocol_step(v: f64, lp: f64, area: f64, osm_in: f64, osm_out: f64, ps: f64, c_in: f64, c_out: f64, vb: f64, dt: f64) → (f64, f64)` | CPA loading step | `biology::cryobiology::freezing` |
| `rewarming_crystallization_risk` | `fn rewarming_crystallization_risk(warming_rate: f64, critical_warming: f64) → f64` | Rewarming risk | `biology::cryobiology::freezing` |
| `glass_transition_temperature` | `fn glass_transition_temperature(cpa_fraction: f64, tg_cpa: f64, tg_water: f64) → f64` | $T_g$ (Gordon-Taylor) | `biology::cryobiology::freezing` |

#### ice_formation.rs (8 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `nucleation_temperature` | `fn nucleation_temperature(solution_concentration: f64, cooling_rate: f64) → f64` | Nucleation temperature | `biology::cryobiology::ice_formation` |
| `ice_growth_rate` | `fn ice_growth_rate(supercooling: f64, diffusion_coeff: f64, latent_heat: f64) → f64` | Ice growth rate | `biology::cryobiology::ice_formation` |
| `intracellular_ice_probability` | `fn intracellular_ice_probability(cooling_rate: f64, critical_rate: f64) → f64` | IIF probability | `biology::cryobiology::ice_formation` |
| `ostwald_recrystallization_rate` | `fn ostwald_recrystallization_rate(temperature: f64, activation_energy: f64) → f64` | Ostwald ripening | `biology::cryobiology::ice_formation` |
| `anti_freeze_protein_thermal_hysteresis` | `fn anti_freeze_protein_thermal_hysteresis(concentration: f64, k_th: f64, n: f64) → f64` | AFP thermal hysteresis | `biology::cryobiology::ice_formation` |
| `cryoprotectant_toxicity` | `fn cryoprotectant_toxicity(concentration: f64, temperature: f64, exposure_time: f64, k_tox: f64) → f64` | CPA toxicity | `biology::cryobiology::ice_formation` |
| `dehydration_during_freezing` | `fn dehydration_during_freezing(initial_water: f64, osmotic_coefficient: f64, ice_fraction: f64) → f64` | Dehydration | `biology::cryobiology::ice_formation` |
| `vitrification_temperature` | `fn vitrification_temperature(water_fraction: f64, tg_pure_solute: f64, tg_water: f64, k_gt: f64) → f64` | Vitrification $T_g$ | `biology::cryobiology::ice_formation` |

#### preservation.rs (19 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `storage_decay_arrhenius` | `fn storage_decay_arrhenius(a: f64, ea: f64, temperature_k: f64) → f64` | Arrhenius decay | `biology::cryobiology::preservation` |
| `shelf_life` | `fn shelf_life(initial_viability: f64, threshold: f64, decay_rate: f64) → f64` | Shelf life | `biology::cryobiology::preservation` |
| `recrystallization_rate` | `fn recrystallization_rate(temperature: f64, activation_energy: f64, pre_factor: f64) → f64` | Recrystallization rate | `biology::cryobiology::preservation` |
| `warming_rate_survival` | `fn warming_rate_survival(warming_rate: f64, optimal_warming: f64, sigma: f64) → f64` | Warming rate survival | `biology::cryobiology::preservation` |
| `devitrification_probability` | `fn devitrification_probability(warming_rate: f64, critical_warming_rate: f64) → f64` | Devitrification | `biology::cryobiology::preservation` |
| `cpa_permeation` | `fn cpa_permeation(permeability: f64, surface_area: f64, concentration_out: f64, concentration_in: f64) → f64` | CPA permeation | `biology::cryobiology::preservation` |
| `two_parameter_model_volume` | `fn two_parameter_model_volume(volume0: f64, lp: f64, surface_area: f64, osm_out: f64, osm_in: f64, dt: f64) → f64` | 2P volume model | `biology::cryobiology::preservation` |
| `cooling_rate_survival` | `fn cooling_rate_survival(cooling_rate: f64, optimal: f64, sigma: f64) → f64` | Cooling rate survival | `biology::cryobiology::preservation` |
| `ice_nucleation_probability` | `fn ice_nucleation_probability(temperature: f64, volume: f64, j0: f64, ea: f64) → f64` | Ice nucleation probability | `biology::cryobiology::preservation` |
| `lyophilization_primary_drying_rate` | `fn lyophilization_primary_drying_rate(heat_input: f64, sublimation_enthalpy: f64) → f64` | Lyophilization drying | `biology::cryobiology::preservation` |
| `lyophilization_collapse_temperature` | `fn lyophilization_collapse_temperature(tg_prime: f64, offset: f64) → f64` | Collapse temperature | `biology::cryobiology::preservation` |
| `trehalose_protection` | `fn trehalose_protection(trehalose_conc: f64, k_protect: f64, max_protection: f64) → f64` | Trehalose protection | `biology::cryobiology::preservation` |
| `thawing_temperature_profile` | `fn thawing_temperature_profile(t_initial: f64, t_bath: f64, k: f64, time: f64) → f64` | Thawing profile | `biology::cryobiology::preservation` |
| `post_thaw_recovery_kinetics` | `fn post_thaw_recovery_kinetics(plateau: f64, recovery_rate: f64, t: f64) → f64` | Post-thaw recovery | `biology::cryobiology::preservation` |
| `controlled_rate_freezer_program` | `fn controlled_rate_freezer_program(target_rate: f64, current_temp: f64, dt: f64) → f64` | CRF program | `biology::cryobiology::preservation` |
| `thermal_seed_temperature` | `fn thermal_seed_temperature(sample_temp: f64, seed_offset: f64) → f64` | Thermal seeding | `biology::cryobiology::preservation` |
| `isochoric_preservation_pressure` | `fn isochoric_preservation_pressure(temperature: f64, reference_temp: f64, bulk_modulus: f64, expansion_coeff: f64) → f64` | Isochoric pressure | `biology::cryobiology::preservation` |
| `q10_temperature_coefficient` | `fn q10_temperature_coefficient(rate_t2: f64, rate_t1: f64, t2: f64, t1: f64) → f64` | $Q_{10}$ coefficient | `biology::cryobiology::preservation` |
| `wlf_viscosity_shift` | `fn wlf_viscosity_shift(c1: f64, c2: f64, temperature: f64, tg: f64) → f64` | WLF viscosity | `biology::cryobiology::preservation` |

---

### 4️⃣2️⃣ nutrition/ — Nutrition — 64 pub fn

#### absorption.rs (16 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `nutrient_absorption_first_order` | `fn nutrient_absorption_first_order(dose: f64, ka: f64, t: f64) → f64` | First-order absorption | `biology::nutrition::absorption` |
| `gastric_emptying` | `fn gastric_emptying(volume: f64, half_life: f64, t: f64) → f64` | Gastric emptying | `biology::nutrition::absorption` |
| `glycemic_index_incremental_auc` | `fn glycemic_index_incremental_auc(glucose_values: &[f64], baseline: f64, dt: f64) → f64` | Incremental AUC | `biology::nutrition::absorption` |
| `protein_digestibility_corrected_amino_acid_score` | `fn protein_digestibility_corrected_amino_acid_score(limiting_aa_mg_g: f64, reference_mg_g: f64, digestibility: f64) → f64` | PDCAAS | `biology::nutrition::absorption` |
| `nitrogen_balance` | `fn nitrogen_balance(protein_intake_g: f64, urinary_n: f64, fecal_n: f64, sweat_n: f64) → f64` | Nitrogen balance | `biology::nutrition::absorption` |
| `water_requirement_holliday_segar` | `fn water_requirement_holliday_segar(weight_kg: f64) → f64` | Holliday-Segar | `biology::nutrition::absorption` |
| `iron_absorption` | `fn iron_absorption(non_heme_mg: f64, enhancers: f64, inhibitors: f64, heme_mg: f64) → f64` | Iron absorption | `biology::nutrition::absorption` |
| `calcium_absorption_fraction` | `fn calcium_absorption_fraction(intake_mg: f64, vitamin_d_nmol: f64) → f64` | Calcium absorption | `biology::nutrition::absorption` |
| `intestinal_transit_time` | `fn intestinal_transit_time(fiber_g: f64, fluid_ml: f64, base_time_h: f64) → f64` | Transit time | `biology::nutrition::absorption` |
| `oral_bioavailability` | `fn oral_bioavailability(fraction_absorbed: f64, gut_wall_extraction: f64, hepatic_extraction: f64) → f64` | Oral bioavailability | `biology::nutrition::absorption` |
| `michaelis_menten_absorption` | `fn michaelis_menten_absorption(vmax: f64, concentration: f64, km: f64) → f64` | Michaelis-Menten | `biology::nutrition::absorption` |
| `fat_soluble_vitamin_absorption` | `fn fat_soluble_vitamin_absorption(dose: f64, fat_intake_g: f64, bile_salt_conc: f64) → f64` | Fat-soluble vitamin | `biology::nutrition::absorption` |
| `zinc_absorption_fraction` | `fn zinc_absorption_fraction(intake_mg: f64, phytate_mg: f64) → f64` | Zinc absorption | `biology::nutrition::absorption` |
| `paracellular_absorption` | `fn paracellular_absorption(permeability: f64, surface_area: f64, concentration: f64) → f64` | Paracellular | `biology::nutrition::absorption` |
| `glucose_transporter_kinetics` | `fn glucose_transporter_kinetics(glucose: f64, vmax: f64, km: f64, insulin_factor: f64) → f64` | GLUT kinetics | `biology::nutrition::absorption` |
| `amino_acid_absorption_rate` | `fn amino_acid_absorption_rate(concentration: f64, vmax: f64, km: f64, competition_factor: f64) → f64` | AA absorption | `biology::nutrition::absorption` |

#### energy_balance.rs (11 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `basal_metabolic_rate_mifflin` | `fn basal_metabolic_rate_mifflin(weight_kg: f64, height_cm: f64, age: f64, is_male: bool) → f64` | Mifflin-St Jeor BMR | `biology::nutrition::energy_balance` |
| `tdee` | `fn tdee(bmr: f64, activity_factor: f64, thermic_effect: f64) → f64` | TDEE | `biology::nutrition::energy_balance` |
| `energy_balance` | `fn energy_balance(intake_kcal: f64, expenditure_kcal: f64) → f64` | Energy balance | `biology::nutrition::energy_balance` |
| `weight_change_prediction` | `fn weight_change_prediction(energy_balance_kcal_per_day: f64, days: f64) → f64` | Weight prediction | `biology::nutrition::energy_balance` |
| `diet_induced_thermogenesis` | `fn diet_induced_thermogenesis(protein_kcal: f64, carb_kcal: f64, fat_kcal: f64) → f64` | DIT | `biology::nutrition::energy_balance` |
| `respiratory_exchange_ratio` | `fn respiratory_exchange_ratio(co2_produced: f64, o2_consumed: f64) → f64` | RER | `biology::nutrition::energy_balance` |
| `substrate_oxidation_from_rer` | `fn substrate_oxidation_from_rer(rer: f64) → (f64, f64)` | Substrate oxidation | `biology::nutrition::energy_balance` |
| `glycemic_index_load` | `fn glycemic_index_load(gi: f64, carb_grams: f64) → f64` | Glycemic load | `biology::nutrition::energy_balance` |
| `insulin_index_response` | `fn insulin_index_response(glycemic_load: f64, protein_factor: f64, protein_grams: f64) → f64` | Insulin index | `biology::nutrition::energy_balance` |
| `body_composition_bmi` | `fn body_composition_bmi(weight_kg: f64, height_m: f64) → f64` | BMI | `biology::nutrition::energy_balance` |
| `body_fat_percentage_navy` | `fn body_fat_percentage_navy(waist_cm: f64, neck_cm: f64, height_cm: f64, is_male: bool) → f64` | Navy body fat | `biology::nutrition::energy_balance` |

#### metabolism.rs (26 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `harris_benedict_male` | `fn harris_benedict_male(weight_kg: f64, height_cm: f64, age: f64) → f64` | Harris-Benedict (M) | `biology::nutrition::metabolism` |
| `harris_benedict_female` | `fn harris_benedict_female(weight_kg: f64, height_cm: f64, age: f64) → f64` | Harris-Benedict (F) | `biology::nutrition::metabolism` |
| `mifflin_st_jeor_male` | `fn mifflin_st_jeor_male(weight_kg: f64, height_cm: f64, age: f64) → f64` | Mifflin-St Jeor (M) | `biology::nutrition::metabolism` |
| `mifflin_st_jeor_female` | `fn mifflin_st_jeor_female(weight_kg: f64, height_cm: f64, age: f64) → f64` | Mifflin-St Jeor (F) | `biology::nutrition::metabolism` |
| `total_daily_energy_expenditure` | `fn total_daily_energy_expenditure(bmr: f64, activity_factor: f64) → f64` | TDEE | `biology::nutrition::metabolism` |
| `thermic_effect_of_food` | `fn thermic_effect_of_food(caloric_intake: f64, tef_fraction: f64) → f64` | TEF | `biology::nutrition::metabolism` |
| `body_mass_index` | `fn body_mass_index(weight_kg: f64, height_m: f64) → f64` | BMI | `biology::nutrition::metabolism` |
| `lean_body_mass_boer_male` | `fn lean_body_mass_boer_male(weight_kg: f64, height_cm: f64) → f64` | LBM Boer (M) | `biology::nutrition::metabolism` |
| `lean_body_mass_boer_female` | `fn lean_body_mass_boer_female(weight_kg: f64, height_cm: f64) → f64` | LBM Boer (F) | `biology::nutrition::metabolism` |
| `body_fat_percentage` | `fn body_fat_percentage(total_mass: f64, lean_mass: f64) → f64` | Body fat % | `biology::nutrition::metabolism` |
| `katch_mcardle_bmr` | `fn katch_mcardle_bmr(lean_body_mass_kg: f64) → f64` | Katch-McArdle | `biology::nutrition::metabolism` |
| `cunningham_bmr` | `fn cunningham_bmr(lean_body_mass_kg: f64) → f64` | Cunningham BMR | `biology::nutrition::metabolism` |
| `respiratory_quotient` | `fn respiratory_quotient(co2_produced: f64, o2_consumed: f64) → f64` | RQ = VCO₂/VO₂ | `biology::nutrition::metabolism` |
| `energy_from_macros` | `fn energy_from_macros(carb_g: f64, protein_g: f64, fat_g: f64, alcohol_g: f64) → f64` | Macro energy | `biology::nutrition::metabolism` |
| `waist_to_hip_ratio` | `fn waist_to_hip_ratio(waist_cm: f64, hip_cm: f64) → f64` | WHR | `biology::nutrition::metabolism` |
| `body_surface_area_dubois` | `fn body_surface_area_dubois(weight_kg: f64, height_cm: f64) → f64` | BSA Du Bois | `biology::nutrition::metabolism` |
| `ideal_body_weight_devine_male` | `fn ideal_body_weight_devine_male(height_cm: f64) → f64` | IBW Devine (M) | `biology::nutrition::metabolism` |
| `ideal_body_weight_devine_female` | `fn ideal_body_weight_devine_female(height_cm: f64) → f64` | IBW Devine (F) | `biology::nutrition::metabolism` |
| `adjusted_body_weight` | `fn adjusted_body_weight(actual_kg: f64, ideal_kg: f64) → f64` | Adjusted BW | `biology::nutrition::metabolism` |
| `resting_metabolic_rate_owen_male` | `fn resting_metabolic_rate_owen_male(weight_kg: f64) → f64` | RMR Owen (M) | `biology::nutrition::metabolism` |
| `resting_metabolic_rate_owen_female` | `fn resting_metabolic_rate_owen_female(weight_kg: f64) → f64` | RMR Owen (F) | `biology::nutrition::metabolism` |
| `glycemic_load` | `fn glycemic_load(glycemic_index: f64, available_carbs_g: f64) → f64` | Glycemic load | `biology::nutrition::metabolism` |
| `fat_oxidation_rate` | `fn fat_oxidation_rate(vo2: f64, vco2: f64) → f64` | Fat oxidation | `biology::nutrition::metabolism` |
| `carb_oxidation_rate` | `fn carb_oxidation_rate(vco2: f64, vo2: f64) → f64` | Carb oxidation | `biology::nutrition::metabolism` |
| `protein_requirement_rda` | `fn protein_requirement_rda(weight_kg: f64) → f64` | Protein RDA | `biology::nutrition::metabolism` |
| `estimated_average_requirement_calcium` | `fn estimated_average_requirement_calcium(age: f64) → f64` | EAR calcium | `biology::nutrition::metabolism` |

#### micronutrients.rs (11 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `recommended_daily_intake_scaling` | `fn recommended_daily_intake_scaling(body_weight_kg: f64, rdi_per_kg: f64) → f64` | RDI scaling | `biology::nutrition::micronutrients` |
| `vitamin_d_synthesis` | `fn vitamin_d_synthesis(uvb_intensity: f64, skin_area: f64, melanin_factor: f64, exposure_minutes: f64) → f64` | Vitamin D synthesis | `biology::nutrition::micronutrients` |
| `iron_absorption_enhancers` | `fn iron_absorption_enhancers(non_heme_iron: f64, vitamin_c_mg: f64, meat_factor: f64) → f64` | Iron enhancers | `biology::nutrition::micronutrients` |
| `calcium_absorption` | `fn calcium_absorption(intake: f64, vitamin_d: f64, age_factor: f64) → f64` | Calcium absorption | `biology::nutrition::micronutrients` |
| `zinc_copper_ratio` | `fn zinc_copper_ratio(zinc_intake: f64, copper_intake: f64) → f64` | Zn/Cu ratio | `biology::nutrition::micronutrients` |
| `bioavailability_factor` | `fn bioavailability_factor(intake: f64, absorption_fraction: f64, first_pass_extraction: f64) → f64` | Bioavailability | `biology::nutrition::micronutrients` |
| `folate_neural_tube_risk` | `fn folate_neural_tube_risk(folate_ug: f64, risk_base: f64, protective_threshold: f64) → f64` | Folate NTD risk | `biology::nutrition::micronutrients` |
| `omega3_omega6_ratio` | `fn omega3_omega6_ratio(omega3: f64, omega6: f64) → f64` | ω-3/ω-6 ratio | `biology::nutrition::micronutrients` |
| `antioxidant_capacity_orac` | `fn antioxidant_capacity_orac(concentration: f64, orac_per_unit: f64) → f64` | ORAC | `biology::nutrition::micronutrients` |
| `iodine_thyroid_requirement` | `fn iodine_thyroid_requirement(body_weight_kg: f64, base_requirement_ug_per_kg: f64, pregnancy_factor: f64) → f64` | Iodine requirement | `biology::nutrition::micronutrients` |
| `nutrient_density_score` | `fn nutrient_density_score(nutrients: &[f64], daily_values: &[f64], calories: f64) → f64` | Nutrient density | `biology::nutrition::micronutrients` |

---

### 4️⃣3️⃣ paleobiology/ — Paleobiology — 50 pub fn

#### dating.rs (16 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `radiometric_age` | `fn radiometric_age(parent: f64, daughter: f64, decay_constant: f64) → f64` | Radiometric age | `biology::paleobiology::dating` |
| `half_life_to_decay_constant` | `fn half_life_to_decay_constant(half_life: f64) → f64` | $\lambda = \ln 2 / t_{1/2}$ | `biology::paleobiology::dating` |
| `carbon14_age` | `fn carbon14_age(ratio_sample: f64, ratio_modern: f64) → f64` | ¹⁴C age | `biology::paleobiology::dating` |
| `extinction_rate` | `fn extinction_rate(extinctions: f64, taxa_at_start: f64, interval_myr: f64) → f64` | Extinction rate | `biology::paleobiology::dating` |
| `origination_rate` | `fn origination_rate(originations: f64, taxa_at_end: f64, interval_myr: f64) → f64` | Origination rate | `biology::paleobiology::dating` |
| `net_diversification_rate` | `fn net_diversification_rate(origination: f64, extinction: f64) → f64` | Net diversification | `biology::paleobiology::dating` |
| `turnover_rate` | `fn turnover_rate(origination: f64, extinction: f64) → f64` | Turnover rate | `biology::paleobiology::dating` |
| `survivorship_cohort` | `fn survivorship_cohort(initial: f64, extinction_rate: f64, t_myr: f64) → f64` | Cohort survivorship | `biology::paleobiology::dating` |
| `standing_diversity` | `fn standing_diversity(origination_rate: f64, extinction_rate: f64, d0: f64, t: f64) → f64` | Standing diversity | `biology::paleobiology::dating` |
| `taxonomic_rate_sampling_corrected` | `fn taxonomic_rate_sampling_corrected(observed_crossers: f64, singletons: f64, total: f64) → f64` | Sampling correction | `biology::paleobiology::dating` |
| `stratigraphic_completeness` | `fn stratigraphic_completeness(gaps_duration: f64, total_duration: f64) → f64` | Stratigraphic completeness | `biology::paleobiology::dating` |
| `confidence_interval_range` | `fn confidence_interval_range(known_range: f64, n_horizons: f64, confidence: f64) → f64` | CI range | `biology::paleobiology::dating` |
| `logistic_diversity` | `fn logistic_diversity(d0: f64, r: f64, k: f64, t: f64) → f64` | Logistic diversity | `biology::paleobiology::dating` |
| `recovery_time_after_extinction` | `fn recovery_time_after_extinction(pre_extinction: f64, post_extinction: f64, diversification_rate: f64) → f64` | Recovery time | `biology::paleobiology::dating` |
| `signor_lipps_correction` | `fn signor_lipps_correction(observed_last: f64, sampling_prob: f64, n_taxa: f64) → f64` | Signor-Lipps | `biology::paleobiology::dating` |
| `potassium_argon_age` | `fn potassium_argon_age(k40: f64, ar40: f64) → f64` | K-Ar age | `biology::paleobiology::dating` |

#### diversity.rs (9 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `faith_phylogenetic_diversity` | `fn faith_phylogenetic_diversity(branch_lengths: &[f64]) → f64` | Faith's PD | `biology::paleobiology::diversity` |
| `mean_pairwise_distance` | `fn mean_pairwise_distance(distances: &[f64], n_taxa: usize) → f64` | MPD | `biology::paleobiology::diversity` |
| `net_relatedness_index` | `fn net_relatedness_index(mpd_observed: f64, mpd_null_mean: f64, mpd_null_sd: f64) → f64` | NRI | `biology::paleobiology::diversity` |
| `nearest_taxon_index` | `fn nearest_taxon_index(mntd_observed: f64, mntd_null_mean: f64, mntd_null_sd: f64) → f64` | NTI | `biology::paleobiology::diversity` |
| `evolutionary_distinctiveness` | `fn evolutionary_distinctiveness(terminal_branch_length: f64, clade_size: usize) → f64` | Evolutionary distinctiveness | `biology::paleobiology::diversity` |
| `phylogenetic_species_variability` | `fn phylogenetic_species_variability(species_correlation_sum: f64, n_species: usize) → f64` | PSV | `biology::paleobiology::diversity` |
| `diversification_rate` | `fn diversification_rate(n_extant: f64, stem_age: f64) → f64` | Diversification rate | `biology::paleobiology::diversity` |
| `lineage_through_time_expected` | `fn lineage_through_time_expected(n0: f64, birth_rate: f64, death_rate: f64, t: f64) → f64` | LTT expected | `biology::paleobiology::diversity` |
| `gamma_statistic` | `fn gamma_statistic(branching_times: &[f64]) → f64` | Gamma statistic | `biology::paleobiology::diversity` |

#### extinction.rs (11 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `background_extinction_rate` | `fn background_extinction_rate(species_lost: f64, total_species: f64, time_my: f64) → f64` | Background extinction | `biology::paleobiology::extinction` |
| `mass_extinction_magnitude` | `fn mass_extinction_magnitude(species_before: f64, species_after: f64) → f64` | Mass extinction | `biology::paleobiology::extinction` |
| `recovery_time_exponential` | `fn recovery_time_exponential(species_lost_fraction: f64, origination_rate: f64) → f64` | Recovery time | `biology::paleobiology::extinction` |
| `kill_curve_severity` | `fn kill_curve_severity(environmental_perturbation: f64, vulnerability: f64, threshold: f64) → f64` | Kill curve | `biology::paleobiology::extinction` |
| `selectivity_index` | `fn selectivity_index(extinction_rate_group: f64, extinction_rate_background: f64) → f64` | Selectivity index | `biology::paleobiology::extinction` |
| `origination_extinction_balance` | `fn origination_extinction_balance(origination_rate: f64, extinction_rate: f64) → f64` | O-E balance | `biology::paleobiology::extinction` |
| `survivorship_curve` | `fn survivorship_curve(initial_cohort: f64, extinction_rate: f64, t: f64) → f64` | Survivorship curve | `biology::paleobiology::extinction` |
| `lazarus_taxon_probability` | `fn lazarus_taxon_probability(true_extinction: f64, sampling_probability: f64, gap_duration: f64) → f64` | Lazarus probability | `biology::paleobiology::extinction` |
| `signor_lipps_effect` | `fn signor_lipps_effect(last_appearance: f64, sampling_interval: f64) → f64` | Signor-Lipps effect | `biology::paleobiology::extinction` |
| `biodiversity_through_time` | `fn biodiversity_through_time(origination_rate: f64, extinction_rate: f64, initial_diversity: f64, t: f64) → f64` | Biodiversity dynamics | `biology::paleobiology::extinction` |
| `waiting_time_to_extinction` | `fn waiting_time_to_extinction(population_size: f64, extinction_rate: f64) → f64` | Waiting time | `biology::paleobiology::extinction` |

#### morphology.rs (14 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `morphological_disparity` | `fn morphological_disparity(trait_values: &[Vec<f64>]) → f64` | Morphological disparity | `biology::paleobiology::morphology` |
| `rarefied_diversity` | `fn rarefied_diversity(abundances: &[usize], sample_size: usize) → f64` | Rarefied diversity | `biology::paleobiology::morphology` |
| `foote_boundary_crosser_rate` | `fn foote_boundary_crosser_rate(n_bt: f64, n_fl: f64) → f64` | Foote boundary crosser | `biology::paleobiology::morphology` |
| `completeness_index` | `fn completeness_index(known_intervals: f64, total_range: f64) → f64` | Completeness index | `biology::paleobiology::morphology` |
| `lazarus_ratio` | `fn lazarus_ratio(lazarus_taxa: f64, total_taxa: f64) → f64` | Lazarus ratio | `biology::paleobiology::morphology` |
| `body_size_cope_trend` | `fn body_size_cope_trend(sizes: &[f64]) → f64` | Cope's rule | `biology::paleobiology::morphology` |
| `morphospace_range` | `fn morphospace_range(trait_values: &[f64]) → f64` | Morphospace range | `biology::paleobiology::morphology` |
| `morphospace_volume` | `fn morphospace_volume(ranges: &[f64]) → f64` | Morphospace volume | `biology::paleobiology::morphology` |
| `pairwise_morphological_distance` | `fn pairwise_morphological_distance(a: &[f64], b: &[f64]) → f64` | Pairwise distance | `biology::paleobiology::morphology` |
| `evolutionary_rate_darwin` | `fn evolutionary_rate_darwin(size_initial: f64, size_final: f64, time_myr: f64) → f64` | Darwinian rate | `biology::paleobiology::morphology` |
| `evolutionary_rate_haldane` | `fn evolutionary_rate_haldane(size_initial: f64, size_final: f64, time_generations: f64, pooled_sd: f64) → f64` | Haldane rate | `biology::paleobiology::morphology` |
| `taphonomic_bias` | `fn taphonomic_bias(original_richness: f64, preservation_prob: f64) → f64` | Taphonomic bias | `biology::paleobiology::morphology` |
| `ghost_lineage_duration` | `fn ghost_lineage_duration(first_appearance: f64, inferred_origin: f64) → f64` | Ghost lineage | `biology::paleobiology::morphology` |
| `disparity_centroid_distance` | `fn disparity_centroid_distance(taxon: &[f64], centroid: &[f64]) → f64` | Centroid distance | `biology::paleobiology::morphology` |

---

### 4️⃣4️⃣ phylogenetics/ — Phylogenetics — 46 pub fn

#### alignment.rs (10 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `aa_to_index` | `fn aa_to_index(aa: u8) → Option<usize>` | Amino acid index | `biology::phylogenetics::alignment` |
| `blosum62_score` | `fn blosum62_score(a: u8, b: u8) → i8` | BLOSUM62 score | `biology::phylogenetics::alignment` |
| `needleman_wunsch` | `fn needleman_wunsch(seq_a: &[u8], seq_b: &[u8], gap_penalty: i32) → (Vec<u8>, Vec<u8>, i32)` | Needleman-Wunsch | `biology::phylogenetics::alignment` |
| `smith_waterman` | `fn smith_waterman(seq_a: &[u8], seq_b: &[u8], gap_penalty: i32) → (Vec<u8>, Vec<u8>, i32)` | Smith-Waterman | `biology::phylogenetics::alignment` |
| `alignment_identity` | `fn alignment_identity(align_a: &[u8], align_b: &[u8]) → f64` | Alignment identity | `biology::phylogenetics::alignment` |
| `affine_gap_needleman_wunsch` | `fn affine_gap_needleman_wunsch(seq_a: &[u8], seq_b: &[u8], gap_open: i32, gap_extend: i32) → (Vec<u8>, Vec<u8>, i32)` | Affine gap NW | `biology::phylogenetics::alignment` |
| `multiple_alignment_score` | `fn multiple_alignment_score(alignment: &[Vec<u8>]) → i32` | MSA score | `biology::phylogenetics::alignment` |
| `pairwise_distance` | `fn pairwise_distance(align_a: &[u8], align_b: &[u8]) → f64` | Pairwise distance | `biology::phylogenetics::alignment` |
| `jukes_cantor_distance` | `fn jukes_cantor_distance(p_distance: f64) → f64` | Jukes-Cantor | `biology::phylogenetics::alignment` |
| `gap_fraction` | `fn gap_fraction(aligned: &[u8]) → f64` | Gap fraction | `biology::phylogenetics::alignment` |

#### distance.rs (7 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `hamming_distance` | `fn hamming_distance(a: &[u8], b: &[u8]) → usize` | Hamming distance | `biology::phylogenetics::distance` |
| `p_distance` | `fn p_distance(a: &[u8], b: &[u8]) → f64` | p-distance | `biology::phylogenetics::distance` |
| `jukes_cantor` | `fn jukes_cantor(p: f64) → f64` | Jukes-Cantor | `biology::phylogenetics::distance` |
| `kimura_2p` | `fn kimura_2p(transitions: f64, transversions: f64, length: f64) → f64` | Kimura 2P | `biology::phylogenetics::distance` |
| `count_transitions_transversions` | `fn count_transitions_transversions(a: &[u8], b: &[u8]) → (usize, usize)` | Ts/Tv count | `biology::phylogenetics::distance` |
| `distance_matrix` | `fn distance_matrix(sequences: &[&[u8]]) → Vec<Vec<f64>>` | Distance matrix | `biology::phylogenetics::distance` |
| `log_det_distance` | `fn log_det_distance(freq_matrix: &[[f64; 4]; 4]) → f64` | LogDet distance | `biology::phylogenetics::distance` |

#### molecular_clock.rs (9 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `molecular_clock_rate` | `fn molecular_clock_rate(substitutions: f64, divergence_time: f64) → f64` | Clock rate | `biology::phylogenetics::molecular_clock` |
| `strict_clock_branch_length` | `fn strict_clock_branch_length(rate: f64, time: f64) → f64` | Strict clock | `biology::phylogenetics::molecular_clock` |
| `relaxed_clock_lognormal` | `fn relaxed_clock_lognormal(mean_rate: f64, sigma: f64, branch_deviation: f64) → f64` | Relaxed clock | `biology::phylogenetics::molecular_clock` |
| `divergence_time_from_distance` | `fn divergence_time_from_distance(genetic_distance: f64, substitution_rate: f64) → f64` | Divergence time | `biology::phylogenetics::molecular_clock` |
| `jc_distance` | `fn jc_distance(p_diff: f64) → f64` | JC distance | `biology::phylogenetics::molecular_clock` |
| `kimura_2p_distance` | `fn kimura_2p_distance(transitions: f64, transversions: f64, length: f64) → f64` | K2P distance | `biology::phylogenetics::molecular_clock` |
| `bayesian_clock_calibration` | `fn bayesian_clock_calibration(prior_age: f64, prior_sd: f64, likelihood_age: f64, likelihood_sd: f64) → (f64, f64)` | Bayesian calibration | `biology::phylogenetics::molecular_clock` |
| `rate_heterogeneity_gamma` | `fn rate_heterogeneity_gamma(alpha: f64, n_categories: usize) → Vec<f64>` | Gamma heterogeneity | `biology::phylogenetics::molecular_clock` |
| `local_clock_assignment` | `fn local_clock_assignment(branch_rates: &[f64], threshold: f64) → Vec<usize>` | Local clock | `biology::phylogenetics::molecular_clock` |

#### sequence.rs (9 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `gc_content` | `fn gc_content(seq: &[u8]) → f64` | GC content | `biology::phylogenetics::sequence` |
| `complement` | `fn complement(base: u8) → u8` | Complement base | `biology::phylogenetics::sequence` |
| `reverse_complement` | `fn reverse_complement(seq: &[u8]) → Vec<u8>` | Reverse complement | `biology::phylogenetics::sequence` |
| `translate_codon` | `fn translate_codon(codon: &[u8]) → u8` | Codon translation | `biology::phylogenetics::sequence` |
| `translate` | `fn translate(dna: &[u8]) → Vec<u8>` | DNA translation | `biology::phylogenetics::sequence` |
| `transcribe` | `fn transcribe(dna: &[u8]) → Vec<u8>` | Transcription | `biology::phylogenetics::sequence` |
| `nucleotide_frequencies` | `fn nucleotide_frequencies(seq: &[u8]) → [f64; 4]` | Nucleotide frequencies | `biology::phylogenetics::sequence` |
| `molecular_weight_dna` | `fn molecular_weight_dna(seq: &[u8]) → f64` | DNA molecular weight | `biology::phylogenetics::sequence` |
| `melting_temperature_basic` | `fn melting_temperature_basic(seq: &[u8]) → f64` | Melting temperature | `biology::phylogenetics::sequence` |

#### tree.rs (11 pub fn)

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| `upgma` | `fn upgma(dist_matrix: &[Vec<f64>]) → Vec<(usize, usize, f64)>` | UPGMA | `biology::phylogenetics::tree` |
| `neighbor_joining` | `fn neighbor_joining(dist_matrix: &[Vec<f64>]) → Vec<(usize, usize, f64, f64)>` | Neighbor-Joining | `biology::phylogenetics::tree` |
| `wpgma` | `fn wpgma(dist_matrix: &[Vec<f64>]) → Vec<(usize, usize, f64)>` | WPGMA | `biology::phylogenetics::tree` |
| `molecular_clock_test` | `fn molecular_clock_test(branch_lengths: &[f64], expected: &[f64]) → f64` | Clock test | `biology::phylogenetics::tree` |
| `robinson_foulds` | `fn robinson_foulds(splits_a: &[Vec<bool>], splits_b: &[Vec<bool>]) → usize` | Robinson-Foulds | `biology::phylogenetics::tree` |
| `sackin_index` | `fn sackin_index(branch_depths: &[usize]) → f64` | Sackin index | `biology::phylogenetics::tree` |
| `colless_index` | `fn colless_index(left_sizes: &[usize], right_sizes: &[usize]) → f64` | Colless index | `biology::phylogenetics::tree` |
| `branch_length_total` | `fn branch_length_total(branch_lengths: &[f64]) → f64` | Total branch length | `biology::phylogenetics::tree` |
| `patristic_distance` | `fn patristic_distance(tree_distances: &[Vec<f64>], i: usize, j: usize) → f64` | Patristic distance | `biology::phylogenetics::tree` |
| `parsimony_score` | `fn parsimony_score(sequences: &[&[u8]], tree_topology: &[(usize, usize)]) → usize` | Parsimony score | `biology::phylogenetics::tree` |
| `gamma_rate_categories` | `fn gamma_rate_categories(alpha: f64, n_categories: usize) → Vec<f64>` | Gamma categories | `biology::phylogenetics::tree` |
