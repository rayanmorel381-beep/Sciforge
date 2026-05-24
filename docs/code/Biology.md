# Biology Source Code Guide

This page documents the source implementation behind `sciforge::biology`, including file layout, execution flow, and Hub dispatch integration.

## Source Coverage

### What is explained
- File-level implementation layout in `src/biology/`.
- Main computation groups and where they are implemented.
- Runtime call path when biology functions are executed through Hub dispatch.

### Visibility and external access
- Visibility: internal (`pub(crate)` in `src/lib.rs`).
- External consumers should use `sciforge::hub` as the public entry point.
- Paths in this page are for source reading and internal crate development.


## Source Code Explanation

### Relevant file structure
- Main implementation: `src/biology/`
- Module entry point: `src/biology/mod.rs`
- Hub routing (when applicable): `src/hub/engine/dispatch/biology.rs`

### Internal logic
1. The module exposes subdomains through `mod.rs` and targeted `pub use` exports.
2. Each `.rs` file implements a coherent block of equations and functions.
3. The Hub invokes these functions through the domain dispatcher when execution goes through `ExperimentRunner`.

### What to verify in source code
- Exact signature: input/output types and argument order.
- Numerical preconditions: divisions, roots, logarithms, and validity ranges.
- Implicit conventions: units, normalization, bounds, and tolerances.

## Modules

- `aging` — telomere dynamics, senescence models, Gompertz mortality, oxidative stress
- `bioelectricity` — membrane potential, Hodgkin-Huxley, action potential, impedance, stimulation
- `bioenergetics` — ATP yield, metabolic rate, photosynthesis, respiration, thermodynamics
- `biogeography` — species-area, island biogeography, climate zones, connectivity
- `bioinformatics` — sequence alignment, assembly, expression, quality
- `biomechanics` — muscle force, tissue stress, locomotion, fluid mechanics
- `biophysics` — membrane transport, molecular dynamics, polymers, protein folding
- `biostatistics` — survival analysis, clinical trials, regression, meta-analysis
- `cancer_biology` — tumor growth, microenvironment, immunology, therapy
- `cell` — cell cycle, adhesion, organelles, signaling, transport
- `chronobiology` — circadian rhythms, entrainment, oscillators
- `cryobiology` — freezing, ice formation, preservation
- `developmental` — differentiation, gene regulation, morphogens, patterns
- `ecology` — Lotka-Volterra, diversity, ecosystem, food web, similarity
- `endocrinology` — hormone kinetics, axes, receptors
- `enzyme` — Michaelis-Menten, inhibition, regulation, mechanisms
- `epigenetics` — chromatin, histones, methylation, inheritance, noncoding RNA
- `ethology` — behavior, communication, foraging, learning
- `evolution` — adaptation, fitness, molecular clock, neutral theory, speciation
- `genetics` — drift, equilibrium, linkage, quantitative traits, selection
- `genomics` — annotation, motifs, ORF, statistics, variants
- `immunology` — adaptive, innate, antibodies, cytokines, dynamics
- `marine_biology` — coral, fisheries, ocean, plankton
- `microbiology` — biofilm, culture, growth, metabolism, quorum sensing
- `mycology` — decomposition, ecology, growth, symbiosis
- `neuroscience` — neural models, cognition, networks, synapses, analysis
- `nutrition` — absorption, energy balance, metabolism, micronutrients
- `paleobiology` — dating, diversity, extinction, morphology
- `parasitology` — epidemiology, host-parasite, immunity, virulence
- `pharmacology` — PK/PD, absorption, drug interactions, pharmacodynamics
- `phylogenetics` — alignment, distance, molecular clock, sequence, tree
- `physiology` — cardiac, hemodynamics, renal, respiratory, thermoregulation
- `plant_biology` — ecology, growth, photosynthesis, transport
- `population` — age structure, epidemiology, growth, predation, spatial
- `proteomics` — mass spec, networks, properties, quantification
- `radiobiology` — DNA damage, dose-response, fractionation, shielding
- `reproduction` — embryogenesis, fertility, hormonal cycles, IVF
- `stem_cell` — differentiation, dynamics, niche, reprogramming
- `structural` — docking, geometry, secondary structure, superposition
- `synthetic_biology` — circuits, CRISPR, metabolic engineering, stochastic
- `systems_biology` — bistability, flux, networks, oscillations, sensitivity
- `tissue_engineering` — bioreactor, scaffold, tissue, vascularization
- `toxicology` — accumulation, dose-response, ecotoxicology, mechanisms
- `virology` — dynamics, epidemiology, evolution, structure

---

## aging API (70 functions)

| Function | Signature |
|----------|-----------|
| `telomere_shortening` | `(initial_length: f64, loss_per_division: f64, divisions: f64) -> f64` |
| `hayflick_limit` | `(initial_length: f64, critical_length: f64, loss_per_division: f64) -> f64` |
| `telomerase_equilibrium` | `( shortening_rate: f64, elongation_rate: f64, initial: f64, t: f64, ) -> f64` |
| `oxidative_damage_accumulation` | `( production_rate: f64, repair_rate: f64, t: f64, initial_damage: f64, ) -> f64` |
| `mitochondrial_damage` | `(intact_fraction: f64, damage_rate: f64, dt: f64) -> f64` |
| `senescent_cell_fraction` | `( division_rate: f64, senescence_prob: f64, clearance_rate: f64, t: f64, ) -> f64` |
| `caloric_restriction_lifespan` | `( base_lifespan: f64, restriction_fraction: f64, effect_coefficient: f64, ) -> f64` |
| `reliability_theory_survival` | `( n_elements: usize, element_failure_rate: f64, redundancy: usize, t: f64, ) -> f64` |
| `ros_steady_state` | `(production_rate: f64, sod_activity: f64, catalase_activity: f64) -> f64` |
| `protein_aggregation` | `( misfolded: f64, aggregation_rate: f64, chaperone_capacity: f64, dt: f64, ) -> f64` |
| `dna_repair_capacity` | `(age: f64, base_capacity: f64, decline_rate: f64) -> f64` |
| `somatic_mutation_accumulation` | `( mutation_rate: f64, divisions: f64, repair_efficiency: f64, ) -> f64` |
| `epigenetic_clock_horvath` | `(cpg_values: &[f64], coefficients: &[f64], intercept: f64) -> f64` |
| `nad_decline` | `(initial_nad: f64, decline_rate: f64, age: f64) -> f64` |
| `autophagy_flux` | `( substrate: f64, autophagosome_formation: f64, lysosomal_activity: f64, age_factor: f64, ) -> f64` |
| `stem_cell_exhaustion` | `( initial_pool: f64, division_rate: f64, senescence_prob: f64, age: f64, ) -> f64` |
| `inflammaging_cytokine` | `(basal: f64, senescent_cells: f64, amplification: f64) -> f64` |
| `crosslink_accumulation` | `(rate: f64, turnover: f64, t: f64) -> f64` |
| `lipofuscin_accumulation` | `(production_rate: f64, t: f64) -> f64` |
| `immune_senescence` | `( naive_t_cells: f64, thymic_output_rate: f64, age: f64, proliferation_capacity: f64, ) -> f64` |
| `gompertz_mortality_rate` | `(a: f64, b: f64, age: f64) -> f64` |
| `gompertz_makeham` | `(a: f64, b: f64, c: f64, age: f64) -> f64` |
| `weibull_mortality_hazard` | `(lambda: f64, k: f64, age: f64) -> f64` |
| `mortality_doubling_time` | `(b: f64) -> f64` |
| `survival_probability` | `(hazard_rates: &[f64], dt: f64) -> f64` |
| `life_expectancy` | `(survival_curve: &[(f64, f64)]) -> f64` |
| `deceleration_plateau` | `(age: f64, plateau_age: f64, plateau_rate: f64, a: f64, b: f64) -> f64` |
| `frailty_deficit_index` | `(deficits: usize, total_items: usize) -> f64` |
| `phenotypic_age_levine` | `( albumin: f64, creatinine: f64, glucose: f64, crp: f64, lymphocyte_pct: f64, mcv: f64, rdw: f64, alp: f64, wbc: f64, chronological_age: f64, ) -> f64` |
| `horvath_clock` | `(cpg_betas: &[f64], coefficients: &[f64], intercept: f64) -> f64` |
| `cr_lifespan_extension` | `( baseline_lifespan: f64, restriction_fraction: f64, max_extension: f64, ) -> f64` |
| `reliability_theory_failure` | `( initial_elements: usize, redundancy: usize, failure_rate: f64, t: f64, ) -> f64` |
| `ros_production_rate` | `(metabolic_rate: f64, coupling_efficiency: f64) -> f64` |
| `antioxidant_capacity` | `( sod: f64, catalase: f64, glutathione: f64, k_sod: f64, k_cat: f64, k_gsh: f64, ) -> f64` |
| `oxidative_damage_rate` | `(ros_level: f64, antioxidant: f64) -> f64` |
| `lipid_peroxidation` | `( pufa_concentration: f64, ros_level: f64, k_initiation: f64, k_propagation: f64, ) -> f64` |
| `protein_carbonylation` | `(protein_conc: f64, ros_level: f64, rate_constant: f64) -> f64` |
| `dna_8oxog_formation` | `(ros_level: f64, guanine_sites: f64, k_oxidation: f64) -> f64` |
| `mitochondrial_ros_vicious_cycle` | `( damage: f64, ros_base: f64, amplification: f64, repair_rate: f64, dt: f64, ) -> f64` |
| `glutathione_ratio` | `(gsh: f64, gssg: f64) -> f64` |
| `fenton_reaction_rate` | `(fe2: f64, h2o2: f64, k_fenton: f64) -> f64` |
| `nrf2_response` | `(ros_level: f64, keap1: f64, k_activation: f64) -> f64` |
| `carbonyl_stress` | `(methylglyoxal: f64, glyoxalase: f64, km: f64) -> f64` |
| `oxidative_stress_index` | `(total_oxidant: f64, total_antioxidant: f64) -> f64` |
| `gompertz_mortality` | `(a: f64, b: f64, age: f64) -> f64` |
| `gompertz_survival` | `(a: f64, b: f64, age: f64) -> f64` |
| `weibull_mortality` | `(lambda: f64, k: f64, age: f64) -> f64` |
| `weibull_survival` | `(lambda: f64, k: f64, age: f64) -> f64` |
| `gompertz_makeham_mortality` | `(a: f64, b: f64, c: f64, age: f64) -> f64` |
| `mortality_rate_doubling_time` | `(b: f64) -> f64` |
| `life_expectancy_from_survival` | `(survival: impl Fn(f64) -> f64, max_age: f64, dt: f64) -> f64` |
| `siler_mortality` | `(a1: f64, b1: f64, a2: f64, a3: f64, b3: f64, age: f64) -> f64` |
| `logistic_mortality_plateau` | `(a: f64, b: f64, c: f64, age: f64) -> f64` |
| `demographic_entropy` | `(life_table_lx: &[f64]) -> f64` |
| `net_reproduction_rate` | `(survivorship: &[f64], fecundity: &[f64]) -> f64` |
| `generation_time` | `(survivorship: &[f64], fecundity: &[f64]) -> f64` |
| `actuarial_senescence_rate` | `( mortality_young: f64, mortality_old: f64, age_interval: f64, ) -> f64` |
| `proportional_hazards` | `(baseline_hazard: f64, covariates: &[f64], coefficients: &[f64]) -> f64` |
| `biological_age_levine` | `( chronological_age: f64, albumin: f64, creatinine: f64, glucose: f64, crp_ln: f64, lymphocyte_pct: f64, mcv: f64, rdw: f64, alkaline_phosphatase: f64, wbc: f64, ) -> f64` |
| `frailty_index` | `(deficits_present: u32, deficits_measured: u32) -> f64` |
| `disability_free_life_expectancy` | `(survival: &[f64], disability_free: &[f64]) -> f64` |
| `telomere_attrition` | `(initial_length: f64, loss_per_division: f64, divisions: usize) -> f64` |
| `telomerase_elongation` | `(current_length: f64, rate: f64, telomerase_activity: f64) -> f64` |
| `replicative_limit` | `(initial_length: f64, loss_per_division: f64, critical_length: f64) -> f64` |
| `telomere_length_distribution` | `(mean: f64, std_dev: f64, n_chromosomes: usize) -> Vec<f64>` |
| `critical_shortening_probability` | `(mean_length: f64, std_dev: f64, critical: f64) -> f64` |
| `shelterin_protection` | `(telomere_length: f64, shelterin_kd: f64) -> f64` |
| `end_replication_problem` | `(lagging_strand_loss: f64, divisions: usize) -> f64` |
| `alt_pathway_elongation` | `( recombination_rate: f64, donor_length: f64, recipient_length: f64, ) -> f64` |
| `telomere_age_regression` | `(age: f64, birth_length: f64, annual_loss: f64) -> f64` |

---

## bioelectricity API (81 functions)

| Function | Signature |
|----------|-----------|
| `action_potential_shape` | `( t: f64, v_rest: f64, v_peak: f64, tau_rise: f64, tau_fall: f64, ) -> f64` |
| `cable_equation_steady` | `(v0: f64, x: f64, lambda: f64) -> f64` |
| `electrotonic_length` | `(physical_length: f64, space_constant: f64) -> f64` |
| `input_resistance_cylinder` | `(rm: f64, ri: f64, diameter: f64) -> f64` |
| `strength_duration_weiss` | `(rheobase: f64, chronaxie: f64, duration: f64) -> f64` |
| `strength_duration_lapicque` | `(rheobase: f64, chronaxie: f64, duration: f64) -> f64` |
| `local_field_potential` | `(currents: &[f64], distances: &[f64], sigma: f64) -> f64` |
| `extracellular_spike_amplitude` | `(transmembrane_current: f64, distance: f64, sigma: f64) -> f64` |
| `impedance_tissue` | `(resistance: f64, capacitance: f64, frequency: f64) -> f64` |
| `defibrillation_threshold` | `(body_mass: f64, transthoracic_impedance: f64) -> f64` |
| `bioimpedance_body_composition` | `( impedance: f64, height: f64, weight: f64, age: f64, sex_factor: f64, ) -> f64` |
| `coulter_counter_volume` | `( baseline_impedance: f64, pulse_amplitude: f64, orifice_volume: f64, ) -> f64` |
| `dielectrophoresis_force` | `(radius: f64, epsilon_m: f64, cm_factor: f64, grad_e2: f64) -> f64` |
| `clausius_mossotti` | `(epsilon_p: f64, epsilon_m: f64) -> f64` |
| `electroporation_threshold` | `(membrane_thickness: f64, breakdown_voltage: f64) -> f64` |
| `electroporation_pore_density` | `(v_m: f64, v_ep: f64, n0: f64, alpha: f64) -> f64` |
| `joule_heating` | `(current_density: f64, conductivity: f64, duration: f64) -> f64` |
| `electrode_double_layer_capacitance` | `(epsilon: f64, debye_length: f64) -> f64` |
| `iontophoresis_flux` | `(current: f64, z: f64, transport_number: f64) -> f64` |
| `skin_impedance_model` | `(r_stratum: f64, c_stratum: f64, r_deep: f64, frequency: f64) -> f64` |
| `ecg_lead_vector` | `(dipole: (f64, f64, f64), lead_direction: (f64, f64, f64)) -> f64` |
| `eeg_dipole_potential` | `( dipole_moment: f64, cos_angle: f64, distance: f64, conductivity: f64, ) -> f64` |
| `nerve_conduction_velocity_temperature` | `(v_ref: f64, q10: f64, t: f64, t_ref: f64) -> f64` |
| `nernst_potential` | `( z: f64, temperature: f64, concentration_out: f64, concentration_in: f64, ) -> f64` |
| `goldman_hodgkin_katz` | `( pk: f64, k_out: f64, k_in: f64, pna: f64, na_out: f64, na_in: f64, pcl: f64, cl_out: f64, cl_in: f64, temperature: f64, ) -> f64` |
| `cable_equation_steady_state` | `(v0: f64, x: f64, lambda: f64) -> f64` |
| `membrane_time_constant` | `(rm: f64, cm: f64) -> f64` |
| `length_constant` | `(rm: f64, ri: f64) -> f64` |
| `gap_junction_conductance` | `( n_channels: f64, single_channel_conductance: f64, open_probability: f64, ) -> f64` |
| `electrodiffusion_flux` | `( permeability: f64, z: f64, vm: f64, c_out: f64, c_in: f64, temperature: f64, ) -> f64` |
| `hodgkin_huxley_sodium_current` | `(g_na: f64, m: f64, h: f64, v: f64, e_na: f64) -> f64` |
| `hodgkin_huxley_potassium_current` | `(g_k: f64, n: f64, v: f64, e_k: f64) -> f64` |
| `hodgkin_huxley_leak_current` | `(g_l: f64, v: f64, e_l: f64) -> f64` |
| `hodgkin_huxley_dv_dt` | `(cm: f64, i_ext: f64, i_na: f64, i_k: f64, i_l: f64) -> f64` |
| `gating_alpha_n` | `(v: f64) -> f64` |
| `gating_beta_n` | `(v: f64) -> f64` |
| `gating_alpha_m` | `(v: f64) -> f64` |
| `gating_beta_m` | `(v: f64) -> f64` |
| `gating_alpha_h` | `(v: f64) -> f64` |
| `gating_beta_h` | `(v: f64) -> f64` |
| `gating_steady_state` | `(alpha: f64, beta: f64) -> f64` |
| `gating_time_constant` | `(alpha: f64, beta: f64) -> f64` |
| `reversal_potential_two_ion` | `(g1: f64, e1: f64, g2: f64, e2: f64) -> f64` |
| `membrane_capacitance_current` | `(cm: f64, dv_dt: f64) -> f64` |
| `ion_channel_open_probability` | `(v: f64, v_half: f64, slope: f64) -> f64` |
| `synaptic_conductance_alpha` | `(g_max: f64, t: f64, tau: f64) -> f64` |
| `synaptic_current` | `(g_syn: f64, v_post: f64, e_syn: f64) -> f64` |
| `calcium_nernst` | `(temperature: f64, ca_out: f64, ca_in: f64) -> f64` |
| `chloride_equilibrium` | `(temperature: f64, cl_out: f64, cl_in: f64) -> f64` |
| `resting_potential_contribution` | `( conductance: f64, reversal: f64, total_conductance: f64, ) -> f64` |
| `space_clamp_error` | `(distance: f64, lambda: f64) -> f64` |
| `action_potential_threshold_estimate` | `(v_rest: f64, depolarization: f64) -> f64` |
| `conduction_velocity` | `(diameter: f64, myelinated: bool) -> f64` |
| `saltatory_conduction_delay` | `(internode_distance: f64, velocity: f64) -> f64` |
| `membrane_resistance_per_area` | `(resistivity: f64, thickness: f64) -> f64` |
| `specific_membrane_capacitance` | `(epsilon_r: f64, thickness: f64) -> f64` |
| `defibrillation_energy` | `(capacitance: f64, voltage: f64) -> f64` |
| `electrode_impedance` | `(resistance: f64, capacitance: f64, frequency: f64) -> f64` |
| `stimulation_strength_duration` | `(rheobase: f64, chronaxie: f64, pulse_width: f64) -> f64` |
| `bioimpedance_cole_model` | `( r_inf: f64, r_0: f64, tau: f64, alpha: f64, frequency: f64, ) -> (f64, f64)` |
| `transcranial_current_density` | `(current: f64, electrode_area: f64) -> f64` |
| `neural_recruitment_curve` | `( stimulus: f64, threshold: f64, saturation: f64, steepness: f64, ) -> f64` |
| `charge_density` | `(charge: f64, electrode_area: f64) -> f64` |
| `cathodic_charge_balanced` | `( anodic_amplitude: f64, anodic_duration: f64, cathodic_duration: f64, ) -> f64` |
| `pulse_train_energy` | `( amplitude: f64, pulse_width: f64, frequency: f64, duration: f64, impedance: f64, ) -> f64` |
| `tissue_heating` | `( current_density: f64, conductivity: f64, duration: f64, specific_heat: f64, density: f64, ) -> f64` |
| `tms_induced_efield` | `(di_dt: f64, coil_inductance: f64, distance: f64) -> f64` |
| `dbs_volume_tissue_activated` | `(current: f64, impedance: f64, threshold_efield: f64) -> f64` |
| `cochlear_implant_spread` | `(current: f64, distance: f64, sigma: f64) -> f64` |
| `fes_fatigue_index` | `(initial_force: f64, final_force: f64) -> f64` |
| `shannon_safety_limit` | `(charge_per_phase_uc: f64, electrode_area_cm2: f64) -> f64` |
| `biphasic_pulse_charge` | `(amplitude: f64, phase_duration: f64) -> f64` |
| `interphase_gap_effect` | `(threshold_no_gap: f64, gap_duration: f64, time_constant: f64) -> f64` |
| `electrochemical_safety_margin` | `(water_window: f64, electrode_potential: f64) -> f64` |
| `warburg_impedance` | `(sigma: f64, frequency: f64) -> (f64, f64)` |
| `constant_phase_element` | `(q: f64, alpha: f64, frequency: f64) -> (f64, f64)` |
| `chronaxie_from_strength_duration` | `( rheobase: f64, threshold_at_pw: f64, pulse_width: f64, ) -> f64` |
| `galvanic_skin_response` | `( baseline_conductance: f64, peak_conductance: f64, t: f64, tau_rise: f64, tau_decay: f64, ) -> f64` |
| `total_charge_delivered` | `( amplitude: f64, pulse_width: f64, frequency: f64, duration: f64, ) -> f64` |
| `electrode_polarization_voltage` | `(charge: f64, capacitance: f64) -> f64` |
| `anodal_break_excitation_threshold` | `( membrane_tau: f64, pulse_duration: f64, rheobase: f64, ) -> f64` |

---

## bioenergetics API (76 functions)

| Function | Signature |
|----------|-----------|
| `atp_hydrolysis_free_energy` | `(delta_g0: f64, atp: f64, adp: f64, pi: f64, t: f64) -> f64` |
| `p_o_ratio` | `(atp_produced: f64, oxygen_consumed: f64) -> f64` |
| `respiratory_control_index` | `(state3_rate: f64, state4_rate: f64) -> f64` |
| `uncoupling_heat` | `(pmf: f64, proton_leak: f64) -> f64` |
| `chemiosmotic_atp_rate` | `(pmf: f64, atp_synthase_activity: f64, h_per_atp: f64) -> f64` |
| `shuttle_efficiency_malate_aspartate` | `(nadh_cytoplasmic: f64, transfer_rate: f64) -> f64` |
| `shuttle_efficiency_glycerol_3p` | `(nadh_cytoplasmic: f64, transfer_rate: f64) -> f64` |
| `metabolic_water` | `(glucose_oxidized: f64) -> f64` |
| `adenylate_energy_charge` | `(atp: f64, adp: f64, amp: f64) -> f64` |
| `phosphocreatine_equilibrium` | `(creatine: f64, atp: f64, k_eq: f64) -> f64` |
| `myosin_atpase_rate` | `(load_fraction: f64, vmax: f64) -> f64` |
| `ionic_gradient_energy` | `(z: f64, vm: f64, c_out: f64, c_in: f64, t: f64) -> f64` |
| `glycolysis_net_atp` | `(glucose: f64) -> f64` |
| `glycolysis_pyruvate_yield` | `(glucose: f64) -> f64` |
| `gluconeogenesis_cost` | `(glucose: f64) -> f64` |
| `pentose_phosphate_nadph` | `(glucose_6p: f64) -> f64` |
| `fatty_acid_synthesis_cost` | `(acetyl_coa_units: f64) -> f64` |
| `urea_cycle_cost` | `(amino_acids: f64) -> f64` |
| `glycogen_storage_efficiency` | `(glucose_units: f64) -> f64` |
| `warburg_effect` | `(aerobic_glycolysis_rate: f64, oxidative_rate: f64) -> f64` |
| `ketogenesis_yield` | `(acetyl_coa: f64) -> f64` |
| `amino_acid_catabolism_atp` | `( carbon_count: usize, is_glucogenic: bool, is_ketogenic: bool, ) -> f64` |
| `cori_cycle_cost` | `(lactate: f64) -> f64` |
| `respiratory_quotient` | `(co2_produced: f64, o2_consumed: f64) -> f64` |
| `metabolic_flux_control_coefficient` | `(v_enzyme: f64, v_pathway: f64, elasticity: f64) -> f64` |
| `farquhar_model` | `( vcmax: f64, jmax: f64, ci: f64, gamma_star: f64, kc: f64, ko: f64, o: f64, rd: f64, par: f64, ) -> f64` |
| `electron_transport_rate` | `(jmax: f64, par: f64) -> f64` |
| `light_response_curve` | `(amax: f64, phi: f64, par: f64, rd: f64) -> f64` |
| `light_compensation_point` | `(amax: f64, phi: f64, rd: f64) -> f64` |
| `water_use_efficiency` | `(assimilation: f64, transpiration: f64) -> f64` |
| `rubisco_specificity` | `(vcmax: f64, kc: f64, vomax: f64, ko: f64) -> f64` |
| `photorespiration_rate` | `(vomax: f64, o: f64, ko: f64, ci: f64, kc: f64) -> f64` |
| `quantum_yield` | `(assimilation_rate: f64, photon_flux: f64) -> f64` |
| `co2_compensation_point_photo` | `( gamma_star: f64, rd: f64, vcmax: f64, kc: f64, ko: f64, o: f64, ) -> f64` |
| `stomatal_conductance_ball_berry` | `( assimilation: f64, rh: f64, cs: f64, g0: f64, g1: f64, ) -> f64` |
| `mesophyll_conductance_photo` | `(assimilation: f64, ci: f64, cc: f64) -> f64` |
| `triose_phosphate_utilization` | `(tpu: f64, ci: f64, gamma_star: f64) -> f64` |
| `light_inhibition_photoinhibition` | `(fv_fm_initial: f64, light_excess: f64, ki: f64) -> f64` |
| `canopy_photosynthesis_sun_shade` | `(lai: f64, k_ext: f64, a_sun: f64, a_shade: f64) -> f64` |
| `carbon_concentrating_mechanism_benefit` | `(ci_c3: f64, ci_c4: f64, vcmax: f64, kc: f64) -> f64` |
| `atp_free_energy` | `(delta_g0: f64, atp: f64, adp: f64, pi: f64, t: f64) -> f64` |
| `atp_synthase_rate` | `(proton_gradient: f64, n_protons: f64, delta_g_atp: f64, t: f64) -> f64` |
| `proton_motive_force` | `(delta_psi: f64, delta_ph: f64, t: f64) -> f64` |
| `p_to_o_ratio` | `(atp_produced: f64, oxygen_consumed: f64) -> f64` |
| `respiratory_control_ratio` | `(state3: f64, state4: f64) -> f64` |
| `membrane_potential_nernst` | `(z: f64, c_out: f64, c_in: f64, t: f64) -> f64` |
| `uncoupler_effect` | `(pmf: f64, permeability: f64, concentration: f64) -> f64` |
| `citric_acid_cycle_nadh_yield` | `(acetyl_coa_flux: f64) -> f64` |
| `citric_acid_cycle_fadh2_yield` | `(acetyl_coa_flux: f64) -> f64` |
| `electron_transport_efficiency` | `( n_electrons: f64, delta_e: f64, delta_g_atp: f64, n_atp: f64, ) -> f64` |
| `substrate_level_phosphorylation` | `(n_reactions: f64, delta_g_per_reaction: f64) -> f64` |
| `anaerobic_atp_yield` | `(glucose_flux: f64) -> f64` |
| `aerobic_atp_yield` | `(glucose_flux: f64) -> f64` |
| `lactate_production_rate` | `(pyruvate_flux: f64, nad_ratio: f64) -> f64` |
| `beta_oxidation_atp_yield` | `(carbon_chain_length: f64) -> f64` |
| `creatine_phosphate_buffer` | `(atp: f64, adp: f64, cr_p: f64, keq: f64) -> f64` |
| `gibbs_free_energy_reaction` | `(delta_h: f64, t: f64, delta_s: f64) -> f64` |
| `gibbs_free_energy_body_temp` | `(delta_h: f64, delta_s: f64) -> f64` |
| `equilibrium_constant_body_temp` | `(delta_g0: f64) -> f64` |
| `redox_potential_body_temp` | `(e0: f64, n: f64, oxidized: f64, reduced: f64) -> f64` |
| `equilibrium_constant` | `(delta_g0: f64, t: f64) -> f64` |
| `redox_potential` | `(e0: f64, n: f64, oxidized: f64, reduced: f64, t: f64) -> f64` |
| `energy_charge` | `(atp: f64, adp: f64, amp: f64) -> f64` |
| `metabolic_rate_kleiber` | `(mass: f64) -> f64` |
| `oxygen_consumption_rate` | `(metabolic_rate: f64, oxycaloric_equivalent: f64) -> f64` |
| `coupling_efficiency` | `(delta_g_atp: f64, delta_g_substrate: f64, n_atp: f64) -> f64` |
| `heat_dissipation` | `(delta_g_reaction: f64, useful_work: f64) -> f64` |
| `metabolic_rate_q10` | `(rate_ref: f64, t: f64, t_ref: f64, q10: f64) -> f64` |
| `arrhenius_metabolic` | `(rate_ref: f64, ea: f64, t: f64, t_ref: f64) -> f64` |
| `thermogenic_cost` | `(delta_h: f64, efficiency: f64) -> f64` |
| `proton_gradient_energy` | `(n_protons: f64, delta_mu: f64) -> f64` |
| `nad_redox_potential` | `(nad_ox: f64, nad_red: f64, e0: f64, t: f64) -> f64` |
| `entropy_production_rate` | `(heat_flux: f64, temperature: f64) -> f64` |
| `exergy_content` | `(delta_h: f64, t0: f64, delta_s: f64) -> f64` |
| `muscle_mechanical_efficiency` | `(work_output: f64, metabolic_input: f64) -> f64` |
| `basal_metabolic_scaling` | `(m0: f64, mass: f64, exponent: f64) -> f64` |

---

## biogeography API (52 functions)

| Function | Signature |
|----------|-----------|
| `species_range_overlap` | `(range_a: (f64, f64), range_b: (f64, f64)) -> f64` |
| `range_size_latitude` | `(area: f64) -> f64` |
| `elevational_diversity_gradient` | `( elevation: f64, peak_elevation: f64, max_richness: f64, ) -> f64` |
| `biome_niche_model` | `( temperature: f64, precipitation: f64, t_opt: f64, p_opt: f64, t_width: f64, p_width: f64, ) -> f64` |
| `regional_endemism_index` | `(endemic_species: usize, total_species: usize) -> f64` |
| `latitudinal_diversity_gradient` | `(latitude: f64, max_richness: f64, steepness: f64) -> f64` |
| `range_shift_velocity` | `(temp_change_rate: f64, spatial_temp_gradient: f64) -> f64` |
| `climate_envelope_suitability` | `( temp: f64, precip: f64, temp_min: f64, temp_max: f64, precip_min: f64, precip_max: f64, ) -> f64` |
| `refugia_persistence` | `(area: f64, min_viable_area: f64, climate_stability: f64) -> f64` |
| `wallace_line_effect` | `(dispersal_ability: f64, barrier_width: f64) -> f64` |
| `landscape_resistance` | `(cost_surface: &[Vec<f64>], path: &[(usize, usize)]) -> f64` |
| `isolation_by_distance` | `(genetic_dist: &[f64], geographic_dist: &[f64]) -> f64` |
| `connectivity_index` | `(patch_areas: &[f64], distances: &[Vec<f64>], alpha: f64) -> Vec<f64>` |
| `metapopulation_incidence` | `(connectivity: f64, e: f64, colonization_coeff: f64) -> f64` |
| `habitat_fragmentation_index` | `(total_area: f64, n_patches: usize, perimeter_sum: f64) -> f64` |
| `effective_mesh_size` | `(patch_areas: &[f64], total_area: f64) -> f64` |
| `proximity_index` | `(focal_area: f64, neighbor_areas: &[f64], distances: &[f64]) -> f64` |
| `corridor_effectiveness` | `( corridor_length: f64, corridor_width: f64, matrix_resistance: f64, ) -> f64` |
| `graph_connectivity` | `(adjacency: &[Vec<bool>]) -> f64` |
| `stepping_stone_migration` | `(populations: &[f64], migration_rate: f64) -> Vec<f64>` |
| `least_cost_distance` | `( cost_surface: &[Vec<f64>], start: (usize, usize), end: (usize, usize), ) -> f64` |
| `resistance_distance` | `(conductances: &[Vec<f64>], node_a: usize, node_b: usize) -> f64` |
| `patch_cohesion` | `(patch_perimeters: &[f64], patch_areas: &[f64], total_cells: f64) -> f64` |
| `circuitscape_effective_resistance` | `(node_conductances: &[f64]) -> f64` |
| `dispersal_kernel_exponential` | `(distance: f64, mean_dispersal: f64) -> f64` |
| `dispersal_kernel_2dt` | `(distance: f64, a: f64, p: f64) -> f64` |
| `range_shift_rate` | `(warming_rate: f64, lapse_rate: f64) -> f64` |
| `latitudinal_gradient` | `(species_tropical: f64, species_polar: f64, lat_range: f64) -> f64` |
| `altitudinal_gradient` | `(species_low: f64, species_high: f64, alt_range: f64) -> f64` |
| `bioclimatic_envelope` | `( temp: f64, precip: f64, temp_min: f64, temp_max: f64, precip_min: f64, precip_max: f64, ) -> f64` |
| `species_area_relationship` | `(c: f64, z: f64, area: f64) -> f64` |
| `endemism_index` | `(endemic_species: usize, total_species: usize) -> f64` |
| `climate_velocity` | `(temp_change_rate: f64, spatial_gradient: f64) -> f64` |
| `habitat_suitability_index` | `(variables: &[f64], optima: &[f64], tolerances: &[f64]) -> f64` |
| `island_equilibrium_richness` | `( immigration_max: f64, extinction_max: f64, area: f64, distance: f64, ) -> f64` |
| `nestedness_temperature` | `(presence_matrix: &[Vec<bool>]) -> f64` |
| `mid_domain_effect` | `(domain_size: f64, range_size: f64) -> f64` |
| `beta_diversity_whittaker` | `(gamma: f64, alpha_mean: f64) -> f64` |
| `beta_diversity_sorensen` | `(shared: f64, unique_a: f64, unique_b: f64) -> f64` |
| `rapoport_rule` | `(range_sizes: &[f64], latitudes: &[f64]) -> f64` |
| `occupancy_frequency` | `(presences: &[bool]) -> f64` |
| `island_species_area` | `(c: f64, z: f64, area: f64) -> f64` |
| `island_immigration_rate` | `(s: f64, p: f64, i_max: f64) -> f64` |
| `island_extinction_rate` | `(s: f64, e_max: f64) -> f64` |
| `macarthur_wilson_equilibrium` | `(i_max: f64, e_max: f64, p: f64) -> f64` |
| `macarthur_wilson_turnover` | `(i_max: f64, e_max: f64, p: f64) -> f64` |
| `distance_decay` | `(similarity_0: f64, decay_rate: f64, distance: f64) -> f64` |
| `rescue_effect` | `(extinction_base: f64, immigration: f64) -> f64` |
| `target_effect` | `(immigration_base: f64, area: f64, area_ref: f64) -> f64` |
| `species_isolation_index` | `(distances: &[f64]) -> f64` |
| `area_effect_on_extinction` | `(e_base: f64, area: f64, z: f64) -> f64` |
| `habitat_diversity` | `(area: f64, k: f64) -> f64` |

---

## bioinformatics API (51 functions)

| Function | Signature |
|----------|-----------|
| `smith_waterman_score` | `( seq1: &[u8], seq2: &[u8], match_score: i32, mismatch: i32, gap: i32, ) -> i32` |
| `needleman_wunsch_score` | `( seq1: &[u8], seq2: &[u8], match_score: i32, mismatch: i32, gap: i32, ) -> i32` |
| `edit_distance` | `(seq1: &[u8], seq2: &[u8]) -> usize` |
| `hamming_distance` | `(seq1: &[u8], seq2: &[u8]) -> usize` |
| `alignment_gc_content` | `(seq: &[u8]) -> f64` |
| `sequence_identity` | `(seq1: &[u8], seq2: &[u8]) -> f64` |
| `codon_frequency` | `(seq: &[u8]) -> Vec<(u32, usize)>` |
| `reverse_complement` | `(seq: &[u8]) -> Vec<u8>` |
| `melting_temperature_basic` | `( a_count: usize, t_count: usize, g_count: usize, c_count: usize, ) -> f64` |
| `assembly_n50` | `(contig_lengths: &[usize]) -> usize` |
| `n_metric` | `(contig_lengths: &[usize], fraction: f64) -> usize` |
| `l50` | `(contig_lengths: &[usize]) -> usize` |
| `genome_coverage` | `(total_bases_sequenced: usize, genome_size: usize) -> f64` |
| `lander_waterman` | `(coverage: f64) -> f64` |
| `expected_contigs` | `( n_reads: usize, read_length: usize, genome_size: usize, overlap: usize, ) -> f64` |
| `assembly_completeness` | `(aligned_bases: usize, reference_size: usize) -> f64` |
| `gc_content_reads` | `(reads: &[&[u8]]) -> f64` |
| `ng50` | `(contig_lengths: &[usize], genome_size: usize) -> usize` |
| `aunga` | `(contig_lengths: &[usize], genome_size: usize) -> f64` |
| `misassembly_rate` | `(misassemblies: usize, total_contigs: usize) -> f64` |
| `chimeric_contig_fraction` | `(chimeric: usize, total: usize) -> f64` |
| `contig_size_distribution` | `(contig_lengths: &[usize]) -> (f64, f64, usize, usize)` |
| `expected_gap_count` | `(coverage: f64, genome_size: usize, read_length: usize) -> f64` |
| `principal_component_variance` | `(eigenvalues: &[f64], component: usize) -> f64` |
| `manhattan_distance_features` | `(a: &[f64], b: &[f64]) -> f64` |
| `euclidean_distance_features` | `(a: &[f64], b: &[f64]) -> f64` |
| `pearson_correlation` | `(x: &[f64], y: &[f64]) -> f64` |
| `fold_change` | `(treatment: f64, control: f64) -> f64` |
| `log2_fold_change` | `(treatment: f64, control: f64) -> f64` |
| `rpkm` | `(read_count: f64, gene_length_kb: f64, total_reads_millions: f64) -> f64` |
| `tpm` | `(read_count: f64, gene_length_kb: f64, sum_rpk: f64) -> f64` |
| `fpkm` | `(fragment_count: f64, gene_length_kb: f64, total_fragments_millions: f64) -> f64` |
| `deseq2_size_factor` | `(counts: &[f64], geometric_means: &[f64]) -> f64` |
| `benjamini_hochberg` | `(p_values: &mut [(usize, f64)]) -> Vec<(usize, f64)>` |
| `volcano_significant` | `(log2fc: f64, p_value: f64, fc_threshold: f64, p_threshold: f64) -> bool` |
| `phred_to_probability` | `(phred: f64) -> f64` |
| `probability_to_phred` | `(p: f64) -> f64` |
| `average_quality` | `(qualities: &[u8]) -> f64` |
| `quality_filter` | `(qualities: &[u8], min_quality: u8, window: usize, min_fraction: f64) -> bool` |
| `expected_errors` | `(qualities: &[u8]) -> f64` |
| `trim_quality` | `(qualities: &[u8], min_quality: u8) -> usize` |
| `n50` | `(lengths: &[usize]) -> usize` |
| `gc_content` | `(sequence: &[u8]) -> f64` |
| `adapter_match_score` | `(read: &[u8], adapter: &[u8]) -> usize` |
| `complexity_dust` | `(sequence: &[u8], window: usize) -> f64` |
| `kmer_frequency` | `(sequence: &[u8], k: usize) -> Vec<(Vec<u8>, usize)>` |
| `shannon_entropy_sequence` | `(sequence: &[u8]) -> f64` |
| `sliding_window_quality` | `(qualities: &[u8], window: usize) -> Vec<f64>` |
| `per_base_quality_distribution` | `(quality_matrix: &[Vec<u8>]) -> Vec<(f64, f64)>` |
| `duplication_rate` | `(total_reads: usize, unique_reads: usize) -> f64` |
| `chimera_breakpoint_score` | `(alignment_a: usize, alignment_b: usize, read_length: usize) -> f64` |

---

## biomechanics API (67 functions)

| Function | Signature |
|----------|-----------|
| `poiseuille_flow` | `(delta_p: f64, radius: f64, length: f64, viscosity: f64) -> f64` |
| `wall_shear_stress` | `(flow_rate: f64, radius: f64, viscosity: f64) -> f64` |
| `reynolds_number` | `(density: f64, velocity: f64, diameter: f64, viscosity: f64) -> f64` |
| `murrays_law_radius` | `(parent_radius: f64, n_children: usize) -> f64` |
| `windkessel_2element` | `( p0: f64, r: f64, c: f64, flow: impl Fn(f64) -> f64, dt: f64, steps: usize, ) -> Vec<f64>` |
| `pulse_wave_velocity` | `( elastic_modulus: f64, wall_thickness: f64, radius: f64, density: f64, ) -> f64` |
| `casson_viscosity` | `(tau_y: f64, eta_inf: f64, shear_rate: f64) -> f64` |
| `oxygen_dissociation_hill` | `(po2: f64, p50: f64, n: f64) -> f64` |
| `cardiac_output` | `(stroke_volume: f64, heart_rate: f64) -> f64` |
| `mean_arterial_pressure` | `(systolic: f64, diastolic: f64) -> f64` |
| `total_peripheral_resistance` | `(map: f64, cvp: f64, cardiac_output: f64) -> f64` |
| `womersley_number` | `(radius: f64, angular_freq: f64, kinematic_viscosity: f64) -> f64` |
| `fahraeus_lindqvist` | `(viscosity_plasma: f64, hematocrit: f64, diameter_um: f64) -> f64` |
| `compliance` | `(delta_v: f64, delta_p: f64) -> f64` |
| `laplace_law_sphere` | `(pressure: f64, radius: f64, wall_thickness: f64) -> f64` |
| `laplace_law_cylinder` | `(pressure: f64, radius: f64, wall_thickness: f64) -> f64` |
| `bernoulli_velocity` | `(delta_p: f64, density: f64) -> f64` |
| `systemic_vascular_resistance` | `(map: f64, rap: f64, co: f64) -> f64` |
| `ejection_fraction` | `(edv: f64, esv: f64) -> f64` |
| `gait_stride_length` | `(velocity: f64, cadence: f64) -> f64` |
| `ground_reaction_force` | `(mass: f64, acceleration: f64) -> f64` |
| `joint_moment` | `(force: f64, moment_arm: f64) -> f64` |
| `joint_power` | `(moment: f64, angular_velocity: f64) -> f64` |
| `center_of_pressure` | `(forces: &[(f64, f64, f64)], positions: &[(f64, f64)]) -> (f64, f64)` |
| `inverse_dynamics_moment` | `( i_segment: f64, alpha: f64, proximal_force: f64, proximal_arm: f64, distal_force: f64, distal_arm: f64, ) -> f64` |
| `metabolic_cost_of_transport` | `(metabolic_rate: f64, mass: f64, velocity: f64) -> f64` |
| `froude_number` | `(velocity: f64, leg_length: f64) -> f64` |
| `dynamic_stability_margin` | `(base_of_support: &[(f64, f64)], com: (f64, f64)) -> f64` |
| `work_loop_area` | `(force: &[f64], length: &[f64]) -> f64` |
| `pendulum_energy_recovery` | `(ek_change: f64, ep_change: f64) -> f64` |
| `new` | `(f_max: f64, l_opt: f64, v_max: f64) -> Self` |
| `force_length` | `(&self, length: f64) -> f64` |
| `force_velocity` | `(&self, velocity: f64) -> f64` |
| `total_force` | `(&self, length: f64, velocity: f64) -> f64` |
| `cross_bridge_huxley` | `(x: f64, f_rate: f64, g_rate: f64, dt: f64, n: f64) -> f64` |
| `pennation_angle_force` | `(f_tendon: f64, angle_rad: f64) -> f64` |
| `joint_torque` | `(force: f64, moment_arm: f64) -> f64` |
| `angular_impulse` | `(torque: f64, dt: f64) -> f64` |
| `muscle_power` | `(force: f64, velocity: f64) -> f64` |
| `work` | `(force: f64, displacement: f64) -> f64` |
| `tendon_force` | `(stiffness: f64, strain: f64) -> f64` |
| `excitation_contraction_coupling` | `(calcium: f64, k_half: f64, n: f64) -> f64` |
| `fatigue_model` | `(force_max: f64, time: f64, fatigue_rate: f64) -> f64` |
| `muscle_stiffness` | `(force: f64, length: f64, l_opt: f64) -> f64` |
| `isometric_twitch` | `(f_max: f64, t: f64, tp: f64) -> f64` |
| `tetanus_fusion` | `(f_twitch: f64, frequency: f64, fusion_freq: f64) -> f64` |
| `muscle_metabolic_rate` | `(force: f64, velocity: f64, activation: f64, basal: f64) -> f64` |
| `fiber_type_recruitment` | `( excitation: f64, threshold_slow: f64, threshold_fast: f64, ) -> (f64, f64)` |
| `sarcomere_force_length` | `(sl: f64) -> f64` |
| `linear_elastic_stress` | `(modulus: f64, strain: f64) -> f64` |
| `kelvin_voigt` | `(modulus: f64, viscosity: f64, strain: f64, strain_rate: f64) -> f64` |
| `maxwell_stress_relaxation` | `(sigma0: f64, modulus: f64, viscosity: f64, t: f64) -> f64` |
| `standard_linear_solid` | `( e1: f64, e2: f64, eta: f64, strain: f64, strain_rate: f64, stress: f64, ) -> f64` |
| `hyperelastic_neo_hookean` | `(c1: f64, lambda: f64) -> f64` |
| `mooney_rivlin` | `(c1: f64, c2: f64, lambda: f64) -> f64` |
| `poroelastic_consolidation` | `( stress: f64, modulus: f64, permeability: f64, viscosity: f64, thickness: f64, t: f64, ) -> f64` |
| `strain_energy_density_linear` | `(modulus: f64, strain: f64) -> f64` |
| `creep_power_law` | `(a: f64, sigma: f64, n: f64, t: f64) -> f64` |
| `bone_density_wolff` | `( rho0: f64, stimulus: f64, reference_stimulus: f64, rate: f64, dt: f64, ) -> f64` |
| `ogden_model` | `(mu: f64, alpha: f64, lambda: f64) -> f64` |
| `fracture_toughness` | `(force: f64, crack_length: f64, width: f64, thickness: f64) -> f64` |
| `viscoelastic_prony` | `(moduli: &[f64], taus: &[f64], t: f64) -> f64` |
| `tissue_hydration_swelling` | `(phi_0: f64, pi_ext: f64, bulk_modulus: f64) -> f64` |
| `biphasic_permeability` | `(k0: f64, strain: f64, m: f64) -> f64` |
| `stress_fiber_remodeling` | `(sigma_old: f64, reference: f64, rate: f64, dt: f64) -> f64` |
| `damage_accumulation` | `(d: f64, stress: f64, threshold: f64, rate: f64, dt: f64) -> f64` |
| `elastic_modulus_density` | `(rho: f64, c: f64, exponent: f64) -> f64` |

---

## biophysics API (79 functions)

| Function | Signature |
|----------|-----------|
| `membrane_bending_energy` | `(kappa: f64, curvature: f64, spontaneous_curvature: f64) -> f64` |
| `helfrich_energy` | `(kappa: f64, kappa_bar: f64, c1: f64, c2: f64, c0: f64) -> f64` |
| `membrane_tension` | `(area_strain: f64, stretch_modulus: f64) -> f64` |
| `lipid_diffusion_saffman_delbruck` | `( viscosity_membrane: f64, viscosity_water: f64, membrane_thickness: f64, radius: f64, t: f64, ) -> f64` |
| `osmotic_lysis_threshold` | `( internal_osmolarity: f64, membrane_tension_max: f64, radius: f64, ) -> f64` |
| `vesicle_budding_energy` | `(kappa: f64, radius: f64) -> f64` |
| `flip_flop_rate` | `(activation_energy: f64, t: f64) -> f64` |
| `lateral_pressure_profile` | `( depth: f64, head_pressure: f64, tail_pressure: f64, thickness: f64, ) -> f64` |
| `line_tension_domain` | `(length: f64, lambda: f64) -> f64` |
| `lennard_jones` | `(r: f64, epsilon: f64, sigma: f64) -> f64` |
| `lennard_jones_force` | `(r: f64, epsilon: f64, sigma: f64) -> f64` |
| `coulomb_interaction` | `(q1: f64, q2: f64, r: f64, epsilon_r: f64) -> f64` |
| `debye_huckel` | `(q: f64, r: f64, kappa: f64, epsilon_r: f64) -> f64` |
| `verlet_step` | `( positions: &mut [f64], velocities: &mut [f64], forces: &[f64], masses: &[f64], dt: f64, )` |
| `kinetic_energy` | `(velocities: &[f64], masses: &[f64]) -> f64` |
| `temperature_from_ke` | `(ke: f64, n_particles: usize, n_dim: usize) -> f64` |
| `morse_potential` | `(r: f64, d_e: f64, a: f64, r_e: f64) -> f64` |
| `harmonic_bond` | `(r: f64, k: f64, r0: f64) -> f64` |
| `harmonic_angle` | `(theta: f64, k: f64, theta0: f64) -> f64` |
| `dihedral_potential` | `(phi: f64, k: f64, n: f64, delta: f64) -> f64` |
| `velocity_verlet_step` | `( positions: &mut [f64], velocities: &mut [f64], forces_old: &[f64], forces_new: &[f64], masses: &[f64], dt: f64, )` |
| `berendsen_thermostat` | `( velocities: &mut [f64], current_temp: f64, target_temp: f64, tau: f64, dt: f64, )` |
| `nose_hoover_friction` | `(ke: f64, target_ke: f64, q: f64) -> f64` |
| `switching_function` | `(r: f64, r_on: f64, r_off: f64) -> f64` |
| `pair_correlation_bin` | `( distances: &[f64], r_min: f64, r_max: f64, n_particles: usize, volume: f64, ) -> f64` |
| `pressure_virial` | `(n: usize, volume: f64, temperature: f64, virial_sum: f64) -> f64` |
| `mean_free_path` | `(density: f64, cross_section: f64) -> f64` |
| `born_mayer_repulsion` | `(a: f64, b: f64, r: f64) -> f64` |
| `buckingham_potential` | `(a: f64, b: f64, c: f64, r: f64) -> f64` |
| `worm_like_chain` | `(l: f64, lp: f64, lc: f64) -> f64` |
| `freely_jointed_chain` | `(l: f64, n: usize, b: f64) -> f64` |
| `end_to_end_distance_rms` | `(n: usize, b: f64) -> f64` |
| `radius_of_gyration` | `(n: usize, b: f64) -> f64` |
| `persistence_length_from_tangent` | `(cos_theta: f64, segment_length: f64) -> f64` |
| `kratky_porod_energy` | `(kappa: f64, ds: f64, curvature: f64) -> f64` |
| `dna_twist_energy` | `(c_twist: f64, delta_twist: f64, length: f64) -> f64` |
| `stokes_einstein_diffusion` | `(t: f64, viscosity: f64, radius: f64) -> f64` |
| `mean_squared_displacement` | `(d: f64, t: f64, n_dim: usize) -> f64` |
| `sedimentation_coefficient` | `( mass: f64, partial_specific_vol: f64, rho_solvent: f64, friction: f64, ) -> f64` |
| `flory_radius` | `(n: usize, b: f64, nu: f64) -> f64` |
| `kuhn_length` | `(persistence_length: f64) -> f64` |
| `contour_length` | `(n: usize, b: f64) -> f64` |
| `extensible_wlc` | `(force: f64, lp: f64, lc: f64, stretch_modulus: f64, t: f64) -> f64` |
| `odijk_deflection_length` | `(lp: f64, d: f64) -> f64` |
| `blob_size` | `(kbt: f64, force: f64) -> f64` |
| `zimm_relaxation_time` | `(viscosity: f64, rg: f64, kbt: f64) -> f64` |
| `rouse_relaxation_time` | `(friction: f64, n: usize, b: f64, kbt: f64) -> f64` |
| `intrinsic_viscosity` | `(rg: f64, mw: f64) -> f64` |
| `overlap_concentration` | `(mw: f64, rg: f64) -> f64` |
| `debye_scattering` | `(q: f64, rg: f64) -> f64` |
| `ramachandran_energy` | `(phi: f64, psi: f64) -> f64` |
| `hydrophobic_free_energy` | `(sasa_nonpolar: f64, gamma: f64) -> f64` |
| `hydrogen_bond_energy` | `(r: f64, theta: f64, epsilon: f64, r0: f64) -> f64` |
| `electrostatic_solvation` | `(charge: f64, radius: f64, epsilon_solvent: f64) -> f64` |
| `fold_stability` | `(delta_h: f64, delta_s: f64, delta_cp: f64, t: f64, t_ref: f64) -> f64` |
| `fraction_folded` | `(delta_g: f64, t: f64) -> f64` |
| `two_state_folding_rate` | `(k0: f64, delta_g_dagger: f64, t: f64) -> f64` |
| `zimm_bragg_helix_coil` | `(s: f64, sigma: f64, n: usize) -> f64` |
| `contact_order` | `(contacts: &[(usize, usize)], chain_length: usize) -> f64` |
| `phi_value` | `( delta_g_mut_folding: f64, delta_g_wt_folding: f64, delta_g_mut_ts: f64, delta_g_wt_ts: f64, ) -> f64` |
| `kauzmann_hydrophobic` | `(delta_cp: f64, t: f64, t_s: f64, t_h: f64, delta_h_h: f64) -> f64` |
| `go_model_energy` | `( contacts: &[(usize, usize)], distances: &[f64], native_distances: &[f64], epsilon: f64, ) -> f64` |
| `native_contact_fraction` | `( current_distances: &[f64], native_distances: &[f64], cutoff: f64, ) -> f64` |
| `radius_of_gyration_3d` | `(coords: &[(f64, f64, f64)]) -> f64` |
| `denaturation_midpoint` | `(delta_h: f64, delta_s: f64) -> f64` |
| `chevron_plot_folding` | `(k_f_water: f64, m_f: f64, denaturant: f64) -> f64` |
| `chevron_plot_unfolding` | `(k_u_water: f64, m_u: f64, denaturant: f64) -> f64` |
| `optical_trap_force` | `(laser_power: f64, n_medium: f64, trap_efficiency: f64) -> f64` |
| `fret_efficiency` | `(r: f64, r0: f64) -> f64` |
| `fret_distance` | `(efficiency: f64, r0: f64) -> f64` |
| `fluorescence_lifetime` | `(quantum_yield: f64, radiative_rate: f64) -> f64` |
| `photobleaching_rate` | `(intensity: f64, cross_section: f64, quantum_yield_bleach: f64) -> f64` |
| `fluorescence_recovery_half_time` | `(beam_radius: f64, diffusion_coeff: f64) -> f64` |
| `single_molecule_diffusion_msd` | `(d: f64, t: f64, localization_error: f64) -> f64` |
| `afm_cantilever_force` | `(spring_constant: f64, deflection: f64) -> f64` |
| `hertz_contact_indentation` | `( force: f64, radius: f64, youngs_modulus: f64, poisson: f64, ) -> f64` |
| `micropipette_aspiration_tension` | `(pressure: f64, pipette_radius: f64) -> f64` |
| `youngs_modulus_from_hertz` | `( force: f64, indentation: f64, tip_radius: f64, poisson: f64, ) -> f64` |
| `traction_force` | `(displacement: f64, substrate_stiffness: f64) -> f64` |

---

## biostatistics API (67 functions)

| Function | Signature |
|----------|-----------|
| `odds_ratio` | `(a: usize, b: usize, c: usize, d: usize) -> f64` |
| `relative_risk` | `(a: usize, b: usize, c: usize, d: usize) -> f64` |
| `absolute_risk_reduction` | `(risk_control: f64, risk_treatment: f64) -> f64` |
| `number_needed_to_treat` | `(arr: f64) -> f64` |
| `sensitivity` | `(tp: usize, fn_count: usize) -> f64` |
| `specificity` | `(tn: usize, fp: usize) -> f64` |
| `positive_predictive_value` | `(tp: usize, fp: usize) -> f64` |
| `negative_predictive_value` | `(tn: usize, fn_count: usize) -> f64` |
| `f1_score` | `(tp: usize, fp: usize, fn_count: usize) -> f64` |
| `roc_auc` | `(scores: &[(f64, bool)]) -> f64` |
| `cohens_kappa` | `(observed_agreement: f64, expected_agreement: f64) -> f64` |
| `likelihood_ratio_positive` | `(sensitivity: f64, specificity: f64) -> f64` |
| `likelihood_ratio_negative` | `(sensitivity: f64, specificity: f64) -> f64` |
| `diagnostic_odds_ratio` | `(tp: usize, fp: usize, fn_count: usize, tn: usize) -> f64` |
| `youden_index` | `(sensitivity: f64, specificity: f64) -> f64` |
| `matthews_correlation_coefficient` | `(tp: usize, tn: usize, fp: usize, fn_count: usize) -> f64` |
| `prevalence_adjusted_ppv` | `(sensitivity: f64, specificity: f64, prevalence: f64) -> f64` |
| `sample_size_two_proportions` | `(p1: f64, p2: f64, alpha_z: f64, power_z: f64) -> f64` |
| `confidence_interval_proportion` | `(p: f64, n: usize, z: f64) -> (f64, f64)` |
| `attributable_risk` | `(risk_exposed: f64, risk_unexposed: f64) -> f64` |
| `population_attributable_fraction` | `( risk_exposed: f64, risk_unexposed: f64, prevalence_exposure: f64, ) -> f64` |
| `meta_analysis_fixed_effect` | `(effects: &[f64], variances: &[f64]) -> (f64, f64)` |
| `cochran_q` | `(effects: &[f64], variances: &[f64]) -> f64` |
| `i_squared` | `(q: f64, k: usize) -> f64` |
| `tau_squared_dsl` | `(q: f64, k: usize, variances: &[f64]) -> f64` |
| `meta_analysis_random_effects` | `(effects: &[f64], variances: &[f64], tau2: f64) -> (f64, f64)` |
| `funnel_plot_asymmetry` | `(effects: &[f64], se: &[f64]) -> f64` |
| `trim_and_fill` | `(effects: &[f64]) -> (f64, usize)` |
| `fail_safe_n` | `(effects: &[f64], variances: &[f64], alpha_z: f64) -> f64` |
| `prediction_interval` | `(pooled: f64, tau2: f64, se_pooled: f64, k: usize) -> (f64, f64)` |
| `egger_regression` | `(effects: &[f64], se: &[f64]) -> (f64, f64)` |
| `cumulative_meta_analysis` | `(effects: &[f64], variances: &[f64]) -> Vec<(f64, f64)>` |
| `influence_analysis` | `(effects: &[f64], variances: &[f64]) -> Vec<f64>` |
| `h_squared` | `(q: f64, k: usize) -> f64` |
| `meta_regression_slope` | `(effects: &[f64], variances: &[f64], covariate: &[f64]) -> f64` |
| `simple_linear_regression` | `(x: &[f64], y: &[f64]) -> (f64, f64)` |
| `r_squared` | `(y: &[f64], y_pred: &[f64]) -> f64` |
| `logistic_regression_probability` | `(beta: &[f64], x: &[f64]) -> f64` |
| `aic` | `(log_likelihood: f64, k: usize) -> f64` |
| `bic` | `(log_likelihood: f64, k: usize, n: usize) -> f64` |
| `residual_standard_error` | `(residuals: &[f64], p: usize) -> f64` |
| `chi_squared_statistic` | `(observed: &[f64], expected: &[f64]) -> f64` |
| `welch_t_statistic` | `(m1: f64, m2: f64, s1: f64, s2: f64, n1: usize, n2: usize) -> f64` |
| `mann_whitney_u` | `(ranks_group1: &[f64], n1: usize, n2: usize) -> f64` |
| `bonferroni_correction` | `(p_value: f64, n_tests: usize) -> f64` |
| `fishers_exact_test_odds` | `(a: usize, b: usize, c: usize, d: usize) -> f64` |
| `spearman_rank_correlation` | `(rank_x: &[f64], rank_y: &[f64]) -> f64` |
| `power_analysis_two_sample` | `(effect_size: f64, alpha_z: f64, power_z: f64) -> f64` |
| `kaplan_meier` | `(times: &[f64], events: &[bool]) -> Vec<(f64, f64)>` |
| `log_rank_statistic` | `( times1: &[f64], events1: &[bool], times2: &[f64], events2: &[bool], ) -> f64` |
| `hazard_ratio` | `( events_treatment: usize, time_treatment: f64, events_control: usize, time_control: f64, ) -> f64` |
| `median_survival` | `(curve: &[(f64, f64)]) -> f64` |
| `nelson_aalen` | `(times: &[f64], events: &[bool]) -> Vec<(f64, f64)>` |
| `exponential_survival` | `(lambda: f64, t: f64) -> f64` |
| `weibull_survival` | `(lambda: f64, k: f64, t: f64) -> f64` |
| `restricted_mean_survival_time` | `(curve: &[(f64, f64)], t_max: f64) -> f64` |
| `greenwood_variance` | `(curve: &[(f64, f64)], at_risk: &[usize], events: &[usize]) -> Vec<f64>` |
| `cumulative_incidence` | `(times: &[f64], events: &[bool], competing: &[bool]) -> Vec<(f64, f64)>` |
| `life_table` | `( age_groups: &[(f64, f64)], deaths: &[f64], population: &[f64], ) -> Vec<(f64, f64, f64)>` |
| `log_logistic_survival` | `(alpha: f64, beta: f64, t: f64) -> f64` |
| `gompertz_survival` | `(alpha: f64, beta: f64, t: f64) -> f64` |
| `cox_partial_likelihood_contribution` | `(beta_x: f64, risk_set_sum: f64) -> f64` |
| `breslow_cumulative_hazard` | `(event_times: &[f64], risk_set_sums: &[f64]) -> Vec<(f64, f64)>` |
| `survival_from_hazard` | `(cumulative_hazard: f64) -> f64` |
| `conditional_survival` | `(s_t: f64, s_t_plus_x: f64) -> f64` |
| `cure_fraction_model` | `(cure_rate: f64, lambda: f64, t: f64) -> f64` |
| `aalen_johansen` | `(times: &[f64], events: &[u8], n_causes: usize) -> Vec<Vec<(f64, f64)>>` |

---

## cancer_biology API (65 functions)

| Function | Signature |
|----------|-----------|
| `tumor_immune_ode` | `( tumor: f64, immune: f64, growth_rate: f64, carrying_capacity: f64, kill_rate: f64, stimulation: f64, decay_rate: f64, ) -> (f64, f64)` |
| `tumor_immune_simulate` | `( tumor0: f64, immune0: f64, growth_rate: f64, carrying_capacity: f64, kill_rate: f64, stimulation: f64, decay_rate: f64, dt: f64, steps: usize, ) -> Vec<(f64, f64)>` |
| `immunoediting_escape` | `( immunogenic_clones: f64, escape_mutation_rate: f64, immune_pressure: f64, ) -> f64` |
| `checkpoint_blockade_effect` | `( baseline_kill: f64, pd1_inhibition: f64, ctla4_inhibition: f64, ) -> f64` |
| `car_t_cell_expansion` | `( initial_cells: f64, antigen_density: f64, expansion_rate: f64, t: f64, ) -> f64` |
| `cytokine_release_syndrome` | `( activated_cells: f64, cytokine_per_cell: f64, clearance_rate: f64, t: f64, ) -> f64` |
| `tumor_neoantigen_fitness` | `( binding_affinity: f64, expression_level: f64, clonality: f64, ) -> f64` |
| `abscopal_effect` | `( local_dose: f64, immune_activation: f64, distant_tumor: f64, sensitivity: f64, ) -> f64` |
| `tumor_angiogenesis_vegf` | `(vegf: f64, endothelial_proliferation_rate: f64, kd: f64) -> f64` |
| `vessel_density` | `( new_vessels: f64, existing_vessels: f64, regression_rate: f64, dt: f64, ) -> f64` |
| `oxygen_diffusion_krogh` | `( p_vessel: f64, consumption_rate: f64, diffusion_coeff: f64, r: f64, r_vessel: f64, ) -> f64` |
| `hypoxia_fraction` | `(distances: &[f64], diffusion_limit: f64) -> f64` |
| `microenvironment_tmb` | `(mutations: usize, megabases_sequenced: f64) -> f64` |
| `clonal_fitness_advantage` | `(clone_sizes: &[f64], fitness_values: &[f64]) -> f64` |
| `tumor_heterogeneity_shannon` | `(clone_fractions: &[f64]) -> f64` |
| `metastatic_probability` | `( invasion_rate: f64, survival_fraction: f64, colonization_rate: f64, time: f64, ) -> f64` |
| `emt_score` | `(epithelial_markers: &[f64], mesenchymal_markers: &[f64]) -> f64` |
| `immune_escape_probability` | `(mhc_expression: f64, pd_l1: f64, neoantigen_load: f64) -> f64` |
| `csc_fraction` | `( symmetric_division_rate: f64, asymmetric_rate: f64, differentiation_rate: f64, ) -> f64` |
| `pharmacokinetic_tumor_exposure` | `( dose: f64, bioavailability: f64, volume_distribution: f64, tumor_perfusion_fraction: f64, ) -> f64` |
| `cell_kill_log` | `(initial: f64, surviving_fraction: f64, cycles: u32) -> f64` |
| `skipper_schabel_log_kill` | `(n: f64, dose: f64, sensitivity: f64) -> f64` |
| `drug_resistance_goldie_coldman` | `(n: f64, mutation_rate: f64) -> f64` |
| `combination_therapy_survival` | `(sf_a: f64, sf_b: f64, interaction: f64) -> f64` |
| `tumor_immune_interaction` | `( tumor: f64, immune: f64, growth_rate: f64, kill_rate: f64, stimulation: f64, decay: f64, k: f64, ) -> (f64, f64)` |
| `hallmarks_proliferation_index` | `(mitotic_count: f64, area: f64) -> f64` |
| `cancer_stem_cell_fraction` | `( symmetric_division_rate: f64, asymmetric_division_rate: f64, differentiation_rate: f64, ) -> f64` |
| `linear_quadratic_survival` | `(dose: f64, alpha: f64, beta: f64) -> f64` |
| `biologically_effective_dose` | `(dose: f64, fractions: f64, alpha_beta: f64) -> f64` |
| `equivalent_dose_2gy` | `(dose: f64, dose_per_fraction: f64, alpha_beta: f64) -> f64` |
| `tumor_control_probability` | `(n_cells: f64, surviving_fraction: f64) -> f64` |
| `normal_tissue_complication_probability` | `(dose: f64, td50: f64, gamma50: f64) -> f64` |
| `therapeutic_ratio` | `(tcp: f64, ntcp: f64) -> f64` |
| `immunotherapy_checkpoint_response` | `( tumor: f64, t_cells: f64, activation_rate: f64, exhaustion_rate: f64, checkpoint_blockade: f64, ) -> f64` |
| `car_t_expansion` | `( initial_cells: f64, antigen_stimulation: f64, expansion_rate: f64, t: f64, ) -> f64` |
| `antibody_drug_conjugate_kill` | `( antibody_conc: f64, target_density: f64, internalization_rate: f64, drug_potency: f64, kd: f64, ) -> f64` |
| `metronomic_antiangiogenic_effect` | `(dose: f64, frequency: f64, sensitivity: f64) -> f64` |
| `fractionation_schedule_bde` | `( n_fractions: u32, dose_per_fraction: f64, alpha_beta: f64, ) -> f64` |
| `cell_cycle_specific_kill` | `(drug_conc: f64, phase_fraction: f64, sensitivity: f64) -> f64` |
| `combination_index_chou_talalay` | `( fa: f64, dose_a: f64, dose_b: f64, dm_a: f64, dm_b: f64, m_a: f64, m_b: f64, ) -> f64` |
| `radiation_oxygen_enhancement_ratio` | `(dose_hypoxic: f64, dose_aerobic: f64) -> f64` |
| `hyperthermia_enhancement` | `(dose: f64, thermal_enhancement_ratio: f64) -> f64` |
| `tumor_growth_gompertz` | `(n: f64, n_max: f64, alpha: f64, dt: f64) -> f64` |
| `tumor_growth_logistic` | `(n: f64, k: f64, r: f64, dt: f64) -> f64` |
| `tumor_doubling_time` | `(growth_rate: f64) -> f64` |
| `clonal_evolution_fitness` | `( clone_sizes: &[f64], fitness: &[f64], mutation_rate: f64, ) -> Vec<f64>` |
| `metastasis_probability` | `( tumor_size: f64, shedding_rate: f64, survival_fraction: f64, colonization_rate: f64, ) -> f64` |
| `tumor_angiogenesis_rate` | `( tumor_size: f64, vegf_production: f64, inhibitor: f64, threshold: f64, ) -> f64` |
| `norton_simon_regression` | `(n: f64, kill_fraction: f64, gompertz_rate: f64, n_max: f64) -> f64` |
| `tumor_growth_exponential` | `(n0: f64, rate: f64, t: f64) -> f64` |
| `tumor_growth_von_bertalanffy` | `(n: f64, a: f64, b: f64, dt: f64) -> f64` |
| `tumor_volume_spherical` | `(diameter: f64) -> f64` |
| `tumor_volume_ellipsoid` | `(length: f64, width: f64, height: f64) -> f64` |
| `recist_response` | `(baseline_diameter: f64, current_diameter: f64) -> f64` |
| `tumor_mutation_burden` | `(somatic_mutations: f64, exome_size_mb: f64) -> f64` |
| `heterogeneity_shannon` | `(clone_fractions: &[f64]) -> f64` |
| `circulating_tumor_cells` | `(shedding: f64, tumor_size: f64, half_life: f64) -> f64` |
| `warburg_glycolysis_rate` | `( glucose: f64, vmax: f64, km: f64, oxygen_inhibition: f64, oxygen: f64, ) -> f64` |
| `hypoxia_inducible_factor` | `(po2: f64, km_o2: f64, max_expression: f64) -> f64` |
| `necrotic_core_radius` | `(tumor_radius: f64, diffusion_length: f64) -> f64` |
| `viable_rim_fraction` | `(tumor_radius: f64, necrotic_radius: f64) -> f64` |
| `ctc_cluster_survival` | `(single_ctc_survival: f64, cluster_size: u32) -> f64` |
| `invasion_index` | `(invaded_distance: f64, time: f64) -> f64` |
| `epithelial_mesenchymal_transition` | `(tgf_beta: f64, threshold: f64, hill: f64) -> f64` |
| `microsatellite_instability_score` | `(unstable_markers: u32, total_markers: u32) -> f64` |

---

## cell API (78 functions)

| Function | Signature |
|----------|-----------|
| `cell_adhesion_energy` | `(contact_area: f64, cadherin_density: f64, bond_energy: f64) -> f64` |
| `integrin_focal_adhesion_force` | `(integrin_count: f64, force_per_integrin: f64) -> f64` |
| `adhesion_receptor_binding` | `(ligand: f64, receptor: f64, kd: f64) -> f64` |
| `cell_cell_junction_strength` | `( tight_junction: f64, adherens_junction: f64, desmosome: f64, ) -> f64` |
| `chemotaxis_velocity` | `(gradient: f64, sensitivity: f64, max_speed: f64) -> f64` |
| `haptotaxis_velocity` | `(ecm_gradient: f64, adhesion_strength: f64, drag: f64) -> f64` |
| `durotaxis_force` | `(stiffness_gradient: f64, cell_contractility: f64) -> f64` |
| `collective_migration_speed` | `(leader_speed: f64, follower_count: usize, coupling: f64) -> f64` |
| `wound_healing_rate` | `(gap_width: f64, migration_speed: f64, proliferation_rate: f64) -> f64` |
| `ecm_remodeling_rate` | `(mmp_activity: f64, timp_activity: f64, substrate: f64) -> f64` |
| `cell_spreading_area` | `(adhesion_strength: f64, cortical_tension: f64) -> f64` |
| `catch_bond_lifetime` | `(force: f64, k0: f64, x1: f64, x2: f64) -> f64` |
| `cell_cycle_ode` | `( cyclin: f64, cdk: f64, apc: f64, k_syn: f64, k_deg: f64, k_act: f64, k_inact: f64, ) -> (f64, f64, f64)` |
| `cell_cycle_simulate` | `( cyclin0: f64, cdk0: f64, apc0: f64, k_syn: f64, k_deg: f64, k_act: f64, k_inact: f64, dt: f64, steps: usize, ) -> Vec<(f64, f64, f64)>` |
| `mitotic_index` | `(dividing_cells: usize, total_cells: usize) -> f64` |
| `cell_growth_logistic` | `(n: f64, r: f64, k: f64, dt: f64, steps: usize) -> Vec<f64>` |
| `g1_checkpoint` | `(dna_damage: f64, p53_threshold: f64, rb_active: f64) -> bool` |
| `g2_checkpoint` | `(dna_damage: f64, repair_capacity: f64, cdk1_activity: f64) -> bool` |
| `spindle_assembly_checkpoint` | `(unattached_kinetochores: usize) -> bool` |
| `apoptosis_probability` | `(dna_damage: f64, p53: f64, bcl2: f64, bax: f64) -> f64` |
| `cell_doubling_time` | `(growth_rate: f64) -> f64` |
| `contact_inhibition` | `(density: f64, max_density: f64, steepness: f64) -> f64` |
| `phase_duration` | `( total_cycle_time: f64, g1_fraction: f64, s_fraction: f64, g2_fraction: f64, ) -> (f64, f64, f64, f64)` |
| `dna_damage_accumulation` | `( damage: f64, production_rate: f64, repair_rate: f64, dt: f64, ) -> f64` |
| `restriction_point` | `(growth_factor: f64, threshold: f64, rb_phosphorylation: f64) -> bool` |
| `cyclin_oscillator` | `(cyclin: f64, cdk: f64, k_syn: f64, k_deg: f64) -> f64` |
| `cell_senescence_probability` | `( telomere_length: f64, critical_length: f64, dna_damage: f64, ) -> f64` |
| `proliferation_index` | `(s_phase: usize, g2m_phase: usize, total: usize) -> f64` |
| `growth_fraction` | `(proliferating: usize, total: usize) -> f64` |
| `cell_loss_factor` | `(growth_rate: f64, doubling_time: f64) -> f64` |
| `hayflick_limit` | `(initial_telomere: f64, loss_per_division: f64, critical: f64) -> f64` |
| `quiescence_entry` | `( growth_factor: f64, nutrient: f64, gf_threshold: f64, nutrient_threshold: f64, ) -> bool` |
| `autophagy_flux` | `(lc3_ii: f64, p62: f64, bafilomycin_effect: f64) -> f64` |
| `proteasome_degradation_rate` | `(ubiquitin_tags: f64, proteasome_activity: f64, km: f64) -> f64` |
| `lysosome_ph` | `(v_atpase_rate: f64, proton_leak: f64, buffer_capacity: f64, volume: f64) -> f64` |
| `endosome_maturation` | `(rab5: f64, rab7: f64, conversion_rate: f64, dt: f64) -> (f64, f64)` |
| `receptor_recycling` | `(internalized: f64, recycling_rate: f64, degradation_rate: f64) -> f64` |
| `mitochondrial_fission_rate` | `(drp1: f64, fis1: f64, threshold: f64) -> f64` |
| `mitochondrial_fusion_rate` | `(mfn1: f64, mfn2: f64, opa1: f64) -> f64` |
| `er_stress_upr` | `(misfolded: f64, bip: f64, ire1_threshold: f64) -> f64` |
| `golgi_transport_rate` | `(cargo: f64, coat_protein: f64, gtp: f64, km_coat: f64) -> f64` |
| `peroxisome_beta_oxidation` | `(vlcfa: f64, enzyme_activity: f64, km: f64) -> f64` |
| `cytoskeleton_treadmilling` | `(polymerization_rate: f64, depolymerization_rate: f64) -> f64` |
| `nuclear_import_rate` | `(cargo: f64, importin: f64, ran_gtp: f64, kd: f64) -> f64` |
| `cell_volume_regulation` | `( volume: f64, target_volume: f64, permeability: f64, osmotic_difference: f64, ) -> f64` |
| `ligand_receptor_binding` | `(l: f64, r_total: f64, kd: f64) -> f64` |
| `hill_response` | `(signal: f64, k: f64, n: f64) -> f64` |
| `bistable_switch` | `(x: f64, k1: f64, k2: f64, n: f64, alpha: f64, beta: f64) -> f64` |
| `bistable_simulate` | `( x0: f64, k1: f64, k2: f64, n: f64, alpha: f64, beta: f64, dt: f64, steps: usize, ) -> Vec<f64>` |
| `mapk_cascade` | `( raf: f64, mek: f64, erk: f64, signal: f64, k_activate: &[f64; 3], k_deactivate: &[f64; 3], ) -> (f64, f64, f64)` |
| `mapk_simulate` | `( raf0: f64, mek0: f64, erk0: f64, signal: f64, k_activate: &[f64; 3], k_deactivate: &[f64; 3], dt: f64, steps: usize, ) -> Vec<(f64, f64, f64)>` |
| `goldbeter_koshland` | `(v1: f64, v2: f64, j1: f64, j2: f64) -> f64` |
| `negative_feedback` | `(output: f64, k_prod: f64, k_deg: f64, k_inh: f64, n: f64) -> f64` |
| `positive_feedback` | `(x: f64, basal: f64, vmax: f64, k: f64, n: f64, deg: f64) -> f64` |
| `receptor_desensitization` | `( active: f64, ligand: f64, kd: f64, k_intern: f64, k_recycle: f64, total: f64, ) -> f64` |
| `dual_phosphorylation` | `(x: f64, kinase: f64, phosphatase: f64, k1: f64, k2: f64) -> f64` |
| `coherent_feedforward` | `( signal: f64, x: f64, k_sx: f64, k_xy: f64, k_sy: f64, threshold: f64, ) -> f64` |
| `incoherent_feedforward` | `(signal: f64, x: f64, k_activation: f64, k_repression: f64) -> f64` |
| `michaelis_menten_cascade` | `(substrate: f64, enzyme: f64, km: f64, vmax: f64) -> f64` |
| `scaffold_complex_formation` | `(a: f64, b: f64, scaffold: f64, ka: f64, kb: f64) -> f64` |
| `crosstalk_inhibition` | `( pathway_a: f64, pathway_b: f64, k_inh_ab: f64, k_inh_ba: f64, ) -> (f64, f64)` |
| `fick_first_law` | `(d: f64, dc_dx: f64) -> f64` |
| `fick_second_law_1d` | `(conc: &mut [f64], d: f64, dx: f64, dt: f64, steps: usize)` |
| `nernst_potential` | `(z: f64, t: f64, c_out: f64, c_in: f64) -> f64` |
| `goldman_equation` | `( p_na: f64, p_k: f64, p_cl: f64, na_out: f64, na_in: f64, k_out: f64, k_in: f64, cl_out: f64, cl_in: f64, t: f64, ) -> f64` |
| `osmotic_pressure` | `(c: f64, t: f64) -> f64` |
| `donnan_ratio` | `(z_ion: f64, z_macro: f64, c_macro: f64, c_salt: f64) -> f64` |
| `active_transport_rate` | `(vmax: f64, substrate: f64, km: f64, atp: f64, km_atp: f64) -> f64` |
| `membrane_capacitance_current` | `(cm: f64, dv_dt: f64) -> f64` |
| `electrochemical_gradient` | `(z: f64, vm: f64, equilibrium_potential: f64) -> f64` |
| `vesicle_fusion_rate` | `(calcium: f64, kd: f64, n: f64, k_max: f64) -> f64` |
| `endocytosis_rate` | `( receptor_bound: f64, k_intern: f64, coat_protein: f64, kd_coat: f64, ) -> f64` |
| `exocytosis_rate` | `(vesicles: f64, calcium: f64, kd: f64) -> f64` |
| `gap_junction_flux` | `(c1: f64, c2: f64, permeability: f64) -> f64` |
| `facilitated_diffusion` | `(c_out: f64, c_in: f64, vmax: f64, km: f64) -> f64` |
| `cotransport_rate` | `(substrate: f64, ion: f64, vmax: f64, km_s: f64, km_i: f64) -> f64` |
| `pinocytosis_uptake` | `(volume_rate: f64, concentration: f64) -> f64` |
| `ion_channel_conductance` | `(g_max: f64, open_probability: f64, driving_force: f64) -> f64` |

---

## chronobiology API (49 functions)

| Function | Signature |
|----------|-----------|
| `zeitgeber_strength` | `(light_intensity: f64, threshold: f64, saturation: f64) -> f64` |
| `phase_response_curve` | `(phase: f64, light_pulse_phase: f64, sensitivity: f64) -> f64` |
| `jet_lag_recovery` | `(timezone_shift: f64, adaptation_rate: f64, days: f64) -> f64` |
| `shift_work_desynchrony` | `(internal_phase: f64, external_phase: f64) -> f64` |
| `seasonal_photoperiod` | `(day_of_year: usize, latitude: f64) -> f64` |
| `melatonin_suppression` | `(light_intensity: f64, ic50: f64, hill_n: f64) -> f64` |
| `social_zeitgeber_strength` | `(regularity: f64, social_contacts: f64) -> f64` |
| `food_entrainment` | `(feeding_time: f64, clock_phase: f64, coupling: f64) -> f64` |
| `chronotype_score` | `(midpoint_sleep: f64) -> f64` |
| `circadian_amplitude_damping` | `(initial_amplitude: f64, damping_rate: f64, t: f64) -> f64` |
| `goodwin_oscillator` | `( x: f64, y: f64, z: f64, k1: f64, k2: f64, k3: f64, ki: f64, n: f64, ) -> (f64, f64, f64)` |
| `van_der_pol_circadian` | `( x: f64, y: f64, mu: f64, tau: f64, light: f64, alpha: f64, ) -> (f64, f64)` |
| `phase_response` | `(phase: f64, light_intensity: f64, sensitivity: f64, tau: f64) -> f64` |
| `entrainment_range` | `(coupling_strength: f64, intrinsic_period: f64) -> (f64, f64)` |
| `melatonin_profile` | `(t: f64, onset: f64, offset: f64, amplitude: f64) -> f64` |
| `desynchrony_index` | `(observed_period: f64, zeitgeber_period: f64) -> f64` |
| `goodwin_simulate` | `( x0: f64, y0: f64, z0: f64, k1: f64, k2: f64, k3: f64, ki: f64, n: f64, dt: f64, steps: usize, ) -> Vec<(f64, f64, f64)>` |
| `kuramoto_order_parameter` | `(phases: &[f64]) -> (f64, f64)` |
| `kuramoto_step` | `(phases: &mut [f64], frequencies: &[f64], coupling: f64, dt: f64)` |
| `arnolds_tongue_boundary` | `(coupling: f64, detuning: f64) -> bool` |
| `repressilator` | `( a: f64, b: f64, c: f64, alpha: f64, alpha0: f64, n: f64, beta: f64, ) -> (f64, f64, f64)` |
| `amplitude_phase_from_timeseries` | `(values: &[f64], period: f64) -> (f64, f64)` |
| `phase_diffusion_coefficient` | `(phase_variance: f64, time: f64) -> f64` |
| `limit_cycle_stability` | `(floquet_exponent: f64) -> bool` |
| `poincare_section_period` | `(crossing_times: &[f64]) -> f64` |
| `detrend_moving_average` | `(data: &[f64], window: usize) -> Vec<f64>` |
| `instantaneous_frequency` | `(phase_prev: f64, phase_curr: f64, dt: f64) -> f64` |
| `mutual_information_phase` | `(phases1: &[f64], phases2: &[f64], n_bins: usize) -> f64` |
| `stochastic_resonance_snr` | `(signal_power: f64, noise_intensity: f64, threshold: f64) -> f64` |
| `jet_lag_resync_time` | `(time_zones_crossed: f64, resync_rate: f64) -> f64` |
| `sleep_pressure` | `(wake_duration: f64, buildup_rate: f64, max_pressure: f64) -> f64` |
| `two_process_model` | `(sleep_pressure: f64, circadian_amplitude: f64, phase: f64) -> f64` |
| `photoperiod` | `(latitude_rad: f64, declination_rad: f64) -> f64` |
| `ultradian_rhythm` | `(amplitudes: &[f64], periods: &[f64], t: f64) -> f64` |
| `chronotype_shift` | `(mid_sleep_free: f64, sleep_debt_correction: f64) -> f64` |
| `circadian_acrophase` | `(data: &[f64], period: f64) -> f64` |
| `cosinor_amplitude` | `(data: &[f64], period: f64) -> f64` |
| `social_jet_lag` | `(weekday_midsleep: f64, weekend_midsleep: f64) -> f64` |
| `mesor` | `(data: &[f64]) -> f64` |
| `sleep_debt` | `(wake_hours: f64, optimal_sleep: f64, actual_sleep: f64) -> f64` |
| `circadian_phase_estimate` | `(core_body_temp_min_time: f64) -> f64` |
| `light_phase_advance` | `(lux: f64, sensitivity: f64, timing_factor: f64) -> f64` |
| `dim_light_melatonin_onset` | `(melatonin_levels: &[f64], threshold: f64) -> Option<usize>` |
| `infradian_cycle` | `(base_amplitude: f64, period_days: f64, day: f64) -> f64` |
| `temperature_compensation_q10` | `(rate_t1: f64, rate_t2: f64, t1: f64, t2: f64) -> f64` |
| `masking_effect` | `(endogenous: f64, exogenous_signal: f64, masking_gain: f64) -> f64` |
| `relative_amplitude` | `(max_val: f64, min_val: f64) -> f64` |
| `interdaily_stability` | `(data: &[f64], period: usize) -> f64` |
| `intradaily_variability` | `(data: &[f64]) -> f64` |

---

## cryobiology API (46 functions)

| Function | Signature |
|----------|-----------|
| `mazur_two_factor_model` | `(cooling_rate: f64, optimal_rate: f64, width: f64) -> f64` |
| `ice_nucleation_rate` | `(temperature: f64, volume: f64, j0: f64, activation_energy: f64) -> f64` |
| `critical_cooling_rate` | `(cpa_concentration: f64, base_rate: f64, sensitivity: f64) -> f64` |
| `vitrification_probability` | `(cooling_rate: f64, critical_rate: f64) -> f64` |
| `cpa_toxicity` | `(concentration: f64, exposure_time: f64, k_tox: f64) -> f64` |
| `cell_volume_response` | `(v0: f64, osmolarity_ratio: f64, vb: f64) -> f64` |
| `freeze_thaw_survival` | `( initial_viability: f64, ice_damage: f64, osmotic_damage: f64, cpa_damage: f64, ) -> f64` |
| `intracellular_ice_formation_probability` | `( cooling_rate: f64, critical_rate: f64, n: f64, ) -> f64` |
| `osmotic_tolerance_limit` | `(v_min: f64, v_max: f64, initial_volume: f64) -> (f64, f64)` |
| `kedem_katchalsky_water_flux` | `( lp: f64, area: f64, delta_pi: f64, sigma: f64, delta_p: f64, ) -> f64` |
| `kedem_katchalsky_solute_flux` | `( ps: f64, area: f64, delta_c: f64, sigma: f64, jv: f64, c_mean: f64, ) -> f64` |
| `freezing_point_depression` | `(concentration: f64, kf: f64, dissociation_factor: f64) -> f64` |
| `hemolysis_fraction` | `(osmolality: f64, half_lysis_osmolality: f64, steepness: f64) -> f64` |
| `stefan_freezing_front` | `(thermal_diffusivity: f64, t: f64, stefan_number: f64) -> f64` |
| `supercooling_degree` | `(freezing_point: f64, nucleation_temp: f64) -> f64` |
| `ice_crystal_growth_rate` | `( supercooling: f64, diffusivity: f64, activation_energy: f64, temperature: f64, ) -> f64` |
| `cpa_loading_protocol_step` | `( v: f64, lp: f64, area: f64, osm_in: f64, osm_out: f64, ps: f64, c_in: f64, c_out: f64, vb: f64, dt: f64, ) -> (f64, f64)` |
| `rewarming_crystallization_risk` | `(warming_rate: f64, critical_warming: f64) -> f64` |
| `glass_transition_temperature` | `(cpa_fraction: f64, tg_cpa: f64, tg_water: f64) -> f64` |
| `nucleation_temperature` | `(solution_concentration: f64, cooling_rate: f64) -> f64` |
| `ice_growth_rate` | `(supercooling: f64, diffusion_coeff: f64, latent_heat: f64) -> f64` |
| `intracellular_ice_probability` | `(cooling_rate: f64, critical_rate: f64) -> f64` |
| `ostwald_recrystallization_rate` | `(temperature: f64, activation_energy: f64) -> f64` |
| `anti_freeze_protein_thermal_hysteresis` | `(concentration: f64, k_th: f64, n: f64) -> f64` |
| `cryoprotectant_toxicity` | `( concentration: f64, temperature: f64, exposure_time: f64, k_tox: f64, ) -> f64` |
| `dehydration_during_freezing` | `( initial_water: f64, osmotic_coefficient: f64, ice_fraction: f64, ) -> f64` |
| `vitrification_temperature` | `( water_fraction: f64, tg_pure_solute: f64, tg_water: f64, k_gt: f64, ) -> f64` |
| `storage_decay_arrhenius` | `(a: f64, ea: f64, temperature_k: f64) -> f64` |
| `shelf_life` | `(initial_viability: f64, threshold: f64, decay_rate: f64) -> f64` |
| `recrystallization_rate` | `(temperature: f64, activation_energy: f64, pre_factor: f64) -> f64` |
| `warming_rate_survival` | `(warming_rate: f64, optimal_warming: f64, sigma: f64) -> f64` |
| `devitrification_probability` | `(warming_rate: f64, critical_warming_rate: f64) -> f64` |
| `cpa_permeation` | `( permeability: f64, surface_area: f64, concentration_out: f64, concentration_in: f64, ) -> f64` |
| `two_parameter_model_volume` | `( volume0: f64, lp: f64, surface_area: f64, osm_out: f64, osm_in: f64, dt: f64, ) -> f64` |
| `cooling_rate_survival` | `(cooling_rate: f64, optimal: f64, sigma: f64) -> f64` |
| `ice_nucleation_probability` | `(temperature: f64, volume: f64, j0: f64, ea: f64) -> f64` |
| `lyophilization_primary_drying_rate` | `(heat_input: f64, sublimation_enthalpy: f64) -> f64` |
| `lyophilization_collapse_temperature` | `(tg_prime: f64, offset: f64) -> f64` |
| `trehalose_protection` | `(trehalose_conc: f64, k_protect: f64, max_protection: f64) -> f64` |
| `thawing_temperature_profile` | `(t_initial: f64, t_bath: f64, k: f64, time: f64) -> f64` |
| `post_thaw_recovery_kinetics` | `(plateau: f64, recovery_rate: f64, t: f64) -> f64` |
| `controlled_rate_freezer_program` | `(target_rate: f64, current_temp: f64, dt: f64) -> f64` |
| `thermal_seed_temperature` | `(sample_temp: f64, seed_offset: f64) -> f64` |
| `isochoric_preservation_pressure` | `( temperature: f64, reference_temp: f64, bulk_modulus: f64, expansion_coeff: f64, ) -> f64` |
| `q10_temperature_coefficient` | `(rate_t2: f64, rate_t1: f64, t2: f64, t1: f64) -> f64` |
| `wlf_viscosity_shift` | `(c1: f64, c2: f64, temperature: f64, tg: f64) -> f64` |

---

## developmental API (35 functions)

| Function | Signature |
|----------|-----------|
| `waddington_landscape` | `(x: f64, param: f64) -> f64` |
| `differentiation_potential` | `(x: f64, param: f64) -> f64` |
| `differentiation_simulate` | `( x0: f64, param_start: f64, param_end: f64, noise: f64, dt: f64, steps: usize, ) -> Vec<(f64, f64)>` |
| `somitogenesis_clock` | `(phase: f64, omega: f64, coupling: f64, neighbor_phase: f64) -> f64` |
| `somite_clock_simulate` | `( phases: &mut [f64], omega: f64, coupling: f64, dt: f64, steps: usize, ) -> Vec<Vec<f64>>` |
| `lateral_inhibition_step` | `( signal: f64, neighbor_signal: f64, delta: f64, notch: f64, k: f64, ) -> (f64, f64)` |
| `cell_fate_probability` | `(signal: f64, threshold: f64, steepness: f64) -> f64` |
| `gene_regulatory_network_step` | `( expression: &mut [f64], interactions: &[Vec<f64>], basal_rates: &[f64], degradation: &[f64], dt: f64, )` |
| `toggle_switch` | `(a: f64, b: f64, alpha: f64, beta: f64, n: f64) -> (f64, f64)` |
| `apical_basal_polarity` | `(par3: f64, par6: f64, atypical_pkc: f64, par1: f64) -> f64` |
| `planar_cell_polarity` | `( frizzled: f64, vang: f64, coupling: f64, neighbor_fz: f64, neighbor_vang: f64, ) -> (f64, f64)` |
| `notch_delta_lateral_inhibition_ode` | `( notch: f64, delta: f64, neighbor_delta: f64, beta_n: f64, beta_d: f64, k: f64, n: f64, ) -> (f64, f64)` |
| `induction_competence` | `( signal: f64, competence_window: f64, time: f64, window_center: f64, ) -> f64` |
| `reaction_diffusion_activator_inhibitor` | `( a: f64, h: f64, da: f64, rho_a: f64, mu_a: f64, dh: f64, rho_h: f64, mu_h: f64, laplacian_a: f64, laplacian_h: f64, ) -> (f64, f64)` |
| `hox_gene_expression` | `(position: f64, boundaries: &[(f64, f64)]) -> Vec<bool>` |
| `morphogenetic_field_potential` | `( cell_position: (f64, f64), field_center: (f64, f64), field_strength: f64, decay: f64, ) -> f64` |
| `morphogen_gradient_steady` | `(source: f64, decay: f64, diffusion: f64, x: f64) -> f64` |
| `morphogen_diffusion_1d` | `( conc: &mut [f64], d: f64, decay: f64, source_idx: usize, source_rate: f64, dx: f64, dt: f64, steps: usize, )` |
| `french_flag_model` | `(concentration: f64, t1: f64, t2: f64) -> u8` |
| `bicoid_gradient` | `(c0: f64, length: f64, lambda: f64, x: f64) -> f64` |
| `positional_information` | `(thresholds: &[f64], gradient: &[f64]) -> Vec<u8>` |
| `morphogen_gradient_reaction` | `( conc: &mut [f64], production: &[f64], degradation: f64, diffusion: f64, dx: f64, dt: f64, steps: usize, )` |
| `shh_patterning` | `(distance: f64, concentration: f64, thresholds: &[(f64, u8)]) -> u8` |
| `morphogen_noise_filtering` | `(signal: &[f64], window: usize) -> Vec<f64>` |
| `interpretation_delay` | `(concentration_history: &[f64], delay: usize) -> f64` |
| `wnt_gradient` | `(source_strength: f64, decay_rate: f64, positions: &[f64]) -> Vec<f64>` |
| `turing_reaction_diffusion` | `( u: &mut [f64], v: &mut [f64], du: f64, dv: f64, a: f64, b: f64, dx: f64, dt: f64, steps: usize, )` |
| `turing_instability_condition` | `(du: f64, dv: f64, fu: f64, gv: f64, fg_det: f64) -> bool` |
| `gierer_meinhardt` | `( activator: &mut [f64], inhibitor: &mut [f64], da: f64, di: f64, rho_a: f64, rho_i: f64, mu_a: f64, mu_i: f64, dx: f64, dt: f64, steps: usize, )` |
| `french_flag_positional` | `( position: f64, threshold_high: f64, threshold_low: f64, morphogen_source: f64, decay_length: f64, ) -> u8` |
| `lateral_inhibition` | `( cells: &mut [f64], notch: &mut [f64], delta: &mut [f64], beta_n: f64, beta_d: f64, k: f64, n: f64, dt: f64, steps: usize, )` |
| `clock_and_wavefront` | `( position: f64, wavefront_speed: f64, frequency: f64, t: f64, threshold: f64, ) -> bool` |
| `schnakenberg` | `( u: &mut [f64], v: &mut [f64], du: f64, dv: f64, a: f64, b: f64, gamma: f64, dx: f64, dt: f64, steps: usize, )` |
| `voronoi_cell_sorting` | `( positions: &[(f64, f64)], types: &[u8], adhesion_same: f64, adhesion_diff: f64, ) -> Vec<(f64, f64)>` |
| `wave_pinning` | `( u: &mut [f64], v: &mut [f64], d: f64, k_on: f64, k_off: f64, k_fb: f64, hill_n: f64, dx: f64, dt: f64, steps: usize, )` |

---

## ecology API (50 functions)

| Function | Signature |
|----------|-----------|
| `shannon_diversity` | `(abundances: &[f64]) -> f64` |
| `simpson_diversity` | `(abundances: &[f64]) -> f64` |
| `inverse_simpson` | `(abundances: &[f64]) -> f64` |
| `species_richness` | `(abundances: &[f64]) -> usize` |
| `pielou_evenness` | `(abundances: &[f64]) -> f64` |
| `berger_parker` | `(abundances: &[f64]) -> f64` |
| `margalef_richness` | `(species: usize, total_individuals: f64) -> f64` |
| `chao1` | `(observed: usize, singletons: usize, doubletons: usize) -> f64` |
| `hill_number` | `(abundances: &[f64], q: f64) -> f64` |
| `trophic_cascade` | `( levels: &[f64], growth_rates: &[f64], carrying_capacities: &[f64], interaction_strengths: &[f64], dt: f64, steps: usize, ) -> Vec<Vec<f64>>` |
| `reaction_diffusion_1d` | `( u: &mut [f64], v: &mut [f64], du: f64, dv: f64, f_coeff: f64, k: f64, dx: f64, dt: f64, steps: usize, )` |
| `species_area` | `(c: f64, z: f64, area: f64) -> f64` |
| `island_biogeography_equilibrium` | `(immigration_rate: f64, extinction_rate: f64) -> f64` |
| `carrying_capacity_from_resources` | `(resource: f64, consumption_per_capita: f64) -> f64` |
| `succession_model` | `( biomass: &[f64], growth_rates: &[f64], capacities: &[f64], competition: &[Vec<f64>], dt: f64, steps: usize, ) -> Vec<Vec<f64>>` |
| `dispersal_kernel_gaussian` | `(distance: f64, sigma: f64) -> f64` |
| `net_primary_productivity` | `(gpp: f64, autotrophic_respiration: f64) -> f64` |
| `net_ecosystem_productivity` | `(npp: f64, heterotrophic_respiration: f64) -> f64` |
| `carbon_use_efficiency` | `(npp: f64, gpp: f64) -> f64` |
| `nitrogen_mineralization` | `( organic_n: f64, microbial_activity: f64, temperature_factor: f64, ) -> f64` |
| `nutrient_use_efficiency` | `(biomass_produced: f64, nutrient_absorbed: f64) -> f64` |
| `liebig_minimum` | `(nutrients: &[f64], requirements: &[f64]) -> f64` |
| `decomposition_rate` | `(initial_mass: f64, k: f64, t: f64) -> f64` |
| `soil_respiration` | `(temperature: f64, moisture: f64, q10: f64, r_ref: f64) -> f64` |
| `evapotranspiration_penman_monteith` | `( net_radiation: f64, soil_heat_flux: f64, air_temp: f64, vpd: f64, wind_speed: f64, surface_resistance: f64, ) -> f64` |
| `water_use_efficiency` | `(carbon_assimilated: f64, water_transpired: f64) -> f64` |
| `litter_bag_decomposition` | `(initial_mass: f64, remaining_mass: f64, time: f64) -> f64` |
| `lotka_volterra_competition` | `( n1: f64, n2: f64, r1: f64, r2: f64, k1: f64, k2: f64, alpha12: f64, alpha21: f64, ) -> (f64, f64)` |
| `lotka_volterra_predator_prey` | `( prey: f64, predator: f64, r: f64, a: f64, b: f64, m: f64, ) -> (f64, f64)` |
| `rosenzweig_macarthur` | `( prey: f64, predator: f64, r: f64, k: f64, a: f64, h: f64, e: f64, m: f64, ) -> (f64, f64)` |
| `type_ii_functional_response` | `(prey_density: f64, attack_rate: f64, handling_time: f64) -> f64` |
| `type_iii_functional_response` | `( prey_density: f64, attack_rate: f64, handling_time: f64, exponent: f64, ) -> f64` |
| `nutrient_cycling` | `( nutrient: f64, producers: f64, decomposers: f64, uptake_rate: f64, mortality_rate: f64, decomposition_rate: f64, ) -> (f64, f64, f64)` |
| `disturbance_regime` | `( biomass: f64, disturbance_intensity: f64, return_interval: f64, time_since: f64, ) -> f64` |
| `intermediate_disturbance_diversity` | `( disturbance_frequency: f64, max_diversity: f64, optimal_frequency: f64, ) -> f64` |
| `metapopulation_levins` | `(p: f64, colonization: f64, extinction: f64) -> f64` |
| `source_sink_dynamics` | `( source_emigration: f64, sink_mortality: f64, sink_immigration: f64, ) -> f64` |
| `food_web_connectance` | `(links: usize, species: usize) -> f64` |
| `trophic_level` | `(diet_trophic_levels: &[f64], diet_fractions: &[f64]) -> f64` |
| `lindeman_efficiency` | `(energy_n_plus_1: f64, energy_n: f64) -> f64` |
| `bray_curtis` | `(a: &[f64], b: &[f64]) -> f64` |
| `jaccard` | `(a: &[bool], b: &[bool]) -> f64` |
| `sorensen` | `(a: &[bool], b: &[bool]) -> f64` |
| `morisita_horn` | `(a: &[f64], b: &[f64]) -> f64` |
| `euclidean_distance` | `(a: &[f64], b: &[f64]) -> f64` |
| `whittaker_beta` | `(gamma: usize, alpha_mean: f64) -> f64` |
| `horn_overlap` | `(a: &[f64], b: &[f64]) -> f64` |
| `chao_jaccard` | `(shared: usize, a_only: usize, b_only: usize, n_a: usize, n_b: usize) -> f64` |
| `manhattan_distance` | `(a: &[f64], b: &[f64]) -> f64` |
| `canberra_distance` | `(a: &[f64], b: &[f64]) -> f64` |

---

## endocrinology API (60 functions)

| Function | Signature |
|----------|-----------|
| `hpa_axis_cortisol` | `( crh: f64, acth_gain: f64, cortisol_gain: f64, feedback_strength: f64, cortisol_current: f64, ) -> (f64, f64)` |
| `hpg_axis_testosterone` | `( gnrh: f64, lh_gain: f64, testosterone_gain: f64, feedback: f64, testosterone_current: f64, ) -> (f64, f64)` |
| `hpt_axis_t4` | `( trh: f64, tsh_gain: f64, t4_gain: f64, feedback: f64, t4_current: f64, ) -> (f64, f64)` |
| `glucose_insulin_model_step` | `( glucose: f64, insulin: f64, glucose_input: f64, dt: f64, si: f64, sg: f64, n: f64, gamma: f64, g_threshold: f64, ) -> (f64, f64)` |
| `calcium_pth_feedback` | `(calcium: f64, setpoint: f64, pth_max: f64, steepness: f64) -> f64` |
| `raas_angiotensin` | `(renin: f64, angiotensinogen: f64, ace_activity: f64) -> f64` |
| `aldosterone_response` | `( angiotensin_ii: f64, potassium: f64, gain_ang: f64, gain_k: f64, ) -> f64` |
| `growth_hormone_igf1` | `(gh: f64, liver_response: f64, feedback: f64, igf1_current: f64) -> f64` |
| `leptin_energy_feedback` | `( fat_mass: f64, leptin_sensitivity: f64, energy_expenditure_base: f64, ) -> f64` |
| `cortisol_awakening_response` | `( basal_cortisol: f64, car_amplitude: f64, time_after_wake_min: f64, ) -> f64` |
| `hormone_synthesis_rate` | `(enzyme_conc: f64, substrate: f64, km: f64, vmax: f64) -> f64` |
| `hormone_half_life_clearance` | `(concentration: f64, half_life: f64, t: f64) -> f64` |
| `pulsatile_release` | `(amplitude: f64, frequency: f64, t: f64, basal: f64) -> f64` |
| `negative_feedback_loop` | `(setpoint: f64, current: f64, gain: f64) -> f64` |
| `positive_feedback_loop` | `(stimulus: f64, hormone_level: f64, gain: f64, threshold: f64) -> f64` |
| `receptor_saturation` | `(hormone: f64, kd: f64, receptor_total: f64) -> f64` |
| `hormone_free_fraction` | `(total: f64, binding_protein: f64, kd: f64) -> f64` |
| `circadian_hormone_profile` | `(amplitude: f64, phase: f64, t_hours: f64, mesor: f64) -> f64` |
| `steroidogenesis_rate` | `(cholesterol: f64, star_protein: f64, enzyme_activity: f64) -> f64` |
| `thyroid_hormone_conversion` | `(t4: f64, deiodinase_activity: f64, km: f64) -> f64` |
| `insulin_sensitivity_index` | `(glucose: f64, insulin: f64) -> f64` |
| `homa_ir` | `(fasting_glucose_mmol: f64, fasting_insulin_mu_per_ml: f64) -> f64` |
| `homa_beta` | `(fasting_insulin_mu_per_ml: f64, fasting_glucose_mmol: f64) -> f64` |
| `hormone_clearance` | `(c0: f64, half_life: f64, t: f64) -> f64` |
| `hormone_infusion_steady_state` | `(infusion_rate: f64, clearance_rate: f64) -> f64` |
| `hormone_infusion_transient` | `(infusion_rate: f64, clearance_rate: f64, t: f64) -> f64` |
| `pulsatile_secretion` | `( amplitude: f64, frequency: f64, phase: f64, baseline: f64, t: f64, ) -> f64` |
| `negative_feedback` | `(hormone_level: f64, set_point: f64, gain: f64) -> f64` |
| `positive_feedback` | `(hormone_level: f64, threshold: f64, gain: f64, max_rate: f64) -> f64` |
| `hpa_axis_step` | `( crf: f64, acth: f64, cortisol: f64, k1: f64, k2: f64, k3: f64, d1: f64, d2: f64, d3: f64, neg_gain: f64, ) -> (f64, f64, f64)` |
| `thyroid_axis_tsh_t4` | `( tsh: f64, t4: f64, trh: f64, k_stim: f64, k_inh: f64, k_prod: f64, d_tsh: f64, d_t4: f64, ) -> (f64, f64)` |
| `insulin_secretion_glucose` | `(glucose: f64, beta_cell_mass: f64, km: f64, vmax: f64) -> f64` |
| `glucose_insulin_dynamics` | `( glucose: f64, insulin: f64, gin: f64, si: f64, sg: f64, n: f64, ib: f64, gb: f64, ) -> (f64, f64)` |
| `hormone_binding_to_carrier` | `(total_hormone: f64, carrier: f64, kd: f64) -> f64` |
| `free_hormone_fraction` | `(total: f64, binding_proteins: f64, kd: f64) -> f64` |
| `cortisol_diurnal_rhythm` | `(t_hours: f64, peak_amplitude: f64, nadir: f64) -> f64` |
| `growth_hormone_pulse` | `(t: f64, pulse_times: &[f64], amplitude: f64, half_life: f64) -> f64` |
| `renin_angiotensin_aldosterone` | `( renin: f64, angiotensinogen: f64, ace: f64, k_renin: f64, k_ace: f64, k_aldo: f64, ) -> (f64, f64, f64)` |
| `parathyroid_calcium_response` | `( calcium: f64, set_point: f64, max_pth: f64, steepness: f64, ) -> f64` |
| `leptin_secretion` | `(fat_mass: f64, sensitivity: f64) -> f64` |
| `ghrelin_fasting_profile` | `( t_since_meal: f64, peak_time: f64, amplitude: f64, baseline: f64, ) -> f64` |
| `receptor_binding_fraction` | `(ligand: f64, kd: f64) -> f64` |
| `competitive_binding` | `(ligand: f64, competitor: f64, kd: f64, ki: f64) -> f64` |
| `receptor_up_regulation` | `(r0: f64, stimulus: f64, k_up: f64, k_deg: f64, t: f64) -> f64` |
| `receptor_down_regulation` | `(r0: f64, stimulus: f64, k_down: f64, k_synth: f64, t: f64) -> f64` |
| `dose_response_hill` | `(dose: f64, ec50: f64, emax: f64, n: f64) -> f64` |
| `insulin_glucose_minimal_model` | `( g: f64, x: f64, insulin: f64, gb: f64, p1: f64, p2: f64, p3: f64, ib: f64, ) -> (f64, f64)` |
| `receptor_internalization` | `( surface: f64, ligand: f64, k_intern: f64, k_recycle: f64, ) -> (f64, f64)` |
| `receptor_clearance_rate` | `(concentration: f64, half_life: f64) -> f64` |
| `feedback_loop_negative` | `( stimulus: f64, hormone: f64, sensitivity: f64, set_point: f64, ) -> f64` |
| `receptor_pulsatile_response` | `(amplitude: f64, frequency: f64, t: f64, baseline: f64) -> f64` |
| `allosteric_modulation` | `(ligand: f64, modulator: f64, kd: f64, alpha: f64, beta: f64) -> f64` |
| `spare_receptor_response` | `(ligand: f64, kd: f64, receptor_reserve: f64) -> f64` |
| `desensitization_kinetics` | `( r0: f64, agonist: f64, k_desens: f64, k_resens: f64, t: f64, ) -> f64` |
| `second_messenger_camp` | `(receptor_activity: f64, k_synth: f64, k_pde: f64, basal: f64) -> f64` |
| `ip3_calcium_release` | `(ip3: f64, k_release: f64, k_serca: f64, store: f64) -> f64` |
| `receptor_dimerization` | `(monomer: f64, kd_dimer: f64) -> f64` |
| `beta_arrestin_recruitment` | `(agonist: f64, receptor: f64, k_arr: f64) -> f64` |
| `receptor_tyrosine_kinase_activation` | `(ligand: f64, receptor: f64, km: f64, vmax: f64) -> f64` |
| `gpcr_g_protein_cycle` | `( active_receptor: f64, gdp_bound: f64, k_exchange: f64, k_hydrolysis: f64, ) -> (f64, f64)` |

---

## enzyme API (50 functions)

| Function | Signature |
|----------|-----------|
| `competitive_inhibition` | `(substrate: f64, inhibitor: f64, vmax: f64, km: f64, ki: f64) -> f64` |
| `uncompetitive_inhibition` | `( substrate: f64, inhibitor: f64, vmax: f64, km: f64, ki: f64, ) -> f64` |
| `mixed_inhibition` | `( substrate: f64, inhibitor: f64, vmax: f64, km: f64, ki: f64, ki_prime: f64, ) -> f64` |
| `noncompetitive_inhibition` | `( substrate: f64, inhibitor: f64, vmax: f64, km: f64, ki: f64, ) -> f64` |
| `substrate_inhibition_velocity` | `(substrate: f64, vmax: f64, km: f64, ksi: f64) -> f64` |
| `irreversible_inhibition` | `(active_enzyme: f64, inhibitor: f64, k_inact: f64, t: f64) -> f64` |
| `tight_binding_inhibition` | `(enzyme_total: f64, inhibitor_total: f64, ki_app: f64) -> f64` |
| `ic50_to_ki` | `(ic50: f64, substrate: f64, km: f64) -> f64` |
| `ki_to_ic50` | `(ki: f64, substrate: f64, km: f64) -> f64` |
| `cheng_prusoff_uncompetitive` | `(ic50: f64, substrate: f64, km: f64) -> f64` |
| `inhibition_constant_dixon` | `( v_no_inhibitor: f64, v_with_inhibitor: f64, inhibitor: f64, substrate: f64, km: f64, ) -> f64` |
| `michaelis_menten` | `(s: f64, vmax: f64, km: f64) -> f64` |
| `michaelis_menten_competitive` | `(s: f64, vmax: f64, km: f64, inhibitor: f64, ki: f64) -> f64` |
| `michaelis_menten_uncompetitive` | `(s: f64, vmax: f64, km: f64, inhibitor: f64, ki: f64) -> f64` |
| `michaelis_menten_noncompetitive` | `(s: f64, vmax: f64, km: f64, inhibitor: f64, ki: f64) -> f64` |
| `hill_equation` | `(s: f64, vmax: f64, k: f64, n: f64) -> f64` |
| `lineweaver_burk` | `(s: &[f64], v: &[f64]) -> (f64, f64)` |
| `eadie_hofstee` | `(v: &[f64], s: &[f64]) -> (f64, f64)` |
| `kcat` | `(vmax: f64, e_total: f64) -> f64` |
| `catalytic_efficiency` | `(kcat_val: f64, km: f64) -> f64` |
| `enzyme_kinetics_solve` | `( s0: f64, e0: f64, k1: f64, k_1: f64, k2: f64, dt: f64, steps: usize, ) -> Vec<(f64, f64, f64)>` |
| `ping_pong` | `(a: f64, b: f64, vmax: f64, ka: f64, kb: f64) -> f64` |
| `ordered_bi_bi` | `(a: f64, b: f64, vmax: f64, ka: f64, kb: f64, kia: f64) -> f64` |
| `random_bi_bi` | `(a: f64, b: f64, vmax: f64, ka: f64, kb: f64, kia: f64, kib: f64) -> f64` |
| `substrate_inhibition` | `(s: f64, vmax: f64, km: f64, ki: f64) -> f64` |
| `allosteric_enzyme` | `(s: f64, vmax: f64, k05: f64, n_hill: f64) -> f64` |
| `covalent_modification_cycle` | `( substrate: f64, kinase_vmax: f64, kinase_km: f64, phosphatase_vmax: f64, phosphatase_km: f64, ) -> f64` |
| `enzyme_activation_energy` | `(k_cat: f64, temperature: f64) -> f64` |
| `suicide_inhibition` | `(e0: f64, inhibitor: f64, ki: f64, kinact: f64, t: f64) -> f64` |
| `enzyme_cooperativity` | `(substrate: f64, vmax: f64, s05: &[f64], weights: &[f64]) -> f64` |
| `metabolic_network_steady_state` | `( stoich: &[Vec<f64>], rates: impl Fn(&[f64]) -> Vec<f64>, initial: &[f64], dt: f64, steps: usize, tolerance: f64, ) -> Vec<f64>` |
| `arrhenius` | `(a: f64, ea: f64, t: f64) -> f64` |
| `q10_factor` | `(rate1: f64, rate2: f64, t1: f64, t2: f64) -> f64` |
| `metabolic_control_coefficient` | `( flux_perturbed: f64, flux_original: f64, enzyme_perturbed: f64, enzyme_original: f64, ) -> f64` |
| `gibbs_free_energy` | `(delta_g0: f64, t: f64, q: f64) -> f64` |
| `mass_action_ratio` | `( products: &[f64], reactants: &[f64], stoich_p: &[f64], stoich_r: &[f64], ) -> f64` |
| `reaction_quotient_vs_keq` | `(q: f64, keq: f64) -> f64` |
| `flux_control_summation` | `(coefficients: &[f64]) -> f64` |
| `elasticity_coefficient` | `( rate: f64, metabolite: f64, delta_rate: f64, delta_metabolite: f64, ) -> f64` |
| `supply_demand_modular` | `( supply_flux: f64, demand_flux: f64, linking_metabolite: f64, elasticity_supply: f64, elasticity_demand: f64, ) -> f64` |
| `allosteric_monod_wyman_changeux` | `(substrate: f64, n: f64, l0: f64, kr: f64, kt: f64) -> f64` |
| `hill_cooperativity` | `(substrate: f64, k_half: f64, n: f64) -> f64` |
| `phosphorylation_switch` | `( kinase: f64, phosphatase: f64, km_kin: f64, km_phos: f64, vmax_kin: f64, vmax_phos: f64, total_protein: f64, ) -> f64` |
| `zymogen_activation` | `(zymogen: f64, activator: f64, k_act: f64) -> f64` |
| `product_inhibition_ordered` | `( substrate: f64, product: f64, vmax_f: f64, km: f64, kp: f64, ) -> f64` |
| `isozyme_total_activity` | `(activities: &[f64], fractions: &[f64]) -> f64` |
| `temperature_activation` | `(rate_ref: f64, ea: f64, t: f64, t_ref: f64) -> f64` |
| `thermal_denaturation` | `(activity: f64, k_denat: f64, t: f64) -> f64` |
| `feedback_inhibition` | `( product: f64, ki: f64, n: f64, vmax: f64, substrate: f64, km: f64, ) -> f64` |
| `cascade_amplification` | `(initial_signal: f64, amplification_factors: &[f64]) -> f64` |

---

## epigenetics API (51 functions)

| Function | Signature |
|----------|-----------|
| `chromatin_accessibility` | `( open_fraction: f64, remodeler_activity: f64, histone_density: f64, ) -> f64` |
| `nucleosome_occupancy` | `(dna_affinity: f64, histone_conc: f64, competitor_conc: f64) -> f64` |
| `histone_mark_propagation` | `( mark_fraction: f64, write_rate: f64, erase_rate: f64, dt: f64, ) -> f64` |
| `polycomb_silencing` | `(pc_binding: f64, h3k27me3: f64, cooperative_factor: f64) -> f64` |
| `trithorax_activation` | `(trx_binding: f64, h3k4me3: f64, cooperative_factor: f64) -> f64` |
| `chromatin_loop_probability` | `(distance_bp: f64, persistence_length_bp: f64) -> f64` |
| `tad_insulation_score` | `(contacts_within: f64, contacts_across: f64) -> f64` |
| `enhancer_promoter_contact` | `( distance: f64, cohesin_loading: f64, ctcf_binding: f64, decay_length: f64, ) -> f64` |
| `bivalent_resolution` | `(h3k4me3: f64, h3k27me3: f64, differentiation_signal: f64) -> (f64, f64)` |
| `heterochromatin_spread` | `( hp1_conc: f64, h3k9me3: f64, barrier_strength: f64, distance: f64, ) -> f64` |
| `atac_seq_signal` | `(fragment_count: f64, total_reads: f64, region_length_bp: f64) -> f64` |
| `histone_mark_occupancy` | `(k_on: f64, k_off: f64) -> f64` |
| `histone_mark_dynamics` | `( occupancy: f64, k_on: f64, k_off: f64, k_spread: f64, neighbor_occ: f64, ) -> f64` |
| `histone_spread_simulate` | `( occupancies: &mut [f64], k_on: f64, k_off: f64, k_spread: f64, dt: f64, steps: usize, ) -> Vec<Vec<f64>>` |
| `nucleosome_positioning_energy` | `( dna_flexibility: &[f64], position: usize, wrap_length: usize, ) -> f64` |
| `chromatin_compaction_ratio` | `(extended_length: f64, compacted_length: f64) -> f64` |
| `histone_acetylation_equilibrium` | `(hat_activity: f64, hdac_activity: f64) -> f64` |
| `bivalent_domain_resolution` | `( h3k4me3: f64, h3k27me3: f64, signal: f64, threshold: f64, ) -> (f64, f64)` |
| `chip_seq_enrichment` | `( ip_reads: f64, input_reads: f64, ip_total: f64, input_total: f64, ) -> f64` |
| `reader_writer_feedback` | `( mark: f64, reader_affinity: f64, writer_rate: f64, eraser_rate: f64, ) -> f64` |
| `heterochromatin_spreading` | `( marks: &mut [f64], spread_rate: f64, barrier_positions: &[usize], dt: f64, )` |
| `epigenetic_inheritance_probability` | `(maintenance: f64, generations: usize) -> f64` |
| `transgenerational_decay` | `(mark: f64, reset_rate: f64, generations: usize) -> Vec<f64>` |
| `epimutation_rate` | `(changes: usize, sites: usize, generations: usize) -> f64` |
| `epiallele_frequency` | `( fitness_epi: f64, fitness_normal: f64, freq: f64, generations: usize, ) -> Vec<f64>` |
| `chromatin_state_transition` | `(state: &[f64], transition_matrix: &[Vec<f64>]) -> Vec<f64>` |
| `imprinting_expression` | `(maternal: f64, paternal: f64, imprint_maternal: bool) -> f64` |
| `paramutation` | `( strong_allele: f64, weak_allele: f64, conversion_rate: f64, generations: usize, ) -> Vec<(f64, f64)>` |
| `metastable_epiallele` | `( base_expression: f64, methylation: f64, stochastic_variance: f64, ) -> f64` |
| `epigenetic_clock` | `(methylation_sites: &[f64], coefficients: &[f64], intercept: f64) -> f64` |
| `environmental_epigenetic_response` | `( stress: f64, sensitivity: f64, methylation_change_rate: f64, baseline_methylation: f64, ) -> f64` |
| `cpg_methylation_level` | `(methylated: usize, total_cpg: usize) -> f64` |
| `methylation_decay` | `( level: f64, dilution_rate: f64, maintenance_efficiency: f64, generations: usize, ) -> Vec<f64>` |
| `de_novo_methylation` | `(unmethylated: f64, rate: f64, dt: f64) -> f64` |
| `bisulfite_conversion_efficiency` | `(converted_c: usize, total_c: usize) -> f64` |
| `methylation_entropy` | `(profile: &[f64]) -> f64` |
| `hydroxymethylation_rate` | `(methylation: f64, tet_activity: f64) -> f64` |
| `tet_oxidation_cascade` | `(mc: f64, tet: f64, dt: f64) -> (f64, f64, f64)` |
| `dnmt_processivity` | `( initial_methylation: &[f64], processivity: f64, direction_forward: bool, ) -> Vec<f64>` |
| `cpg_island_density` | `(sequence_length: usize, cpg_count: usize, expected_cpg: f64) -> f64` |
| `age_methylation_predictor` | `(cpg_beta_values: &[f64], weights: &[f64], offset: f64) -> f64` |
| `mirna_target_repression` | `( mirna_conc: f64, target_mrna: f64, kd: f64, max_repression: f64, ) -> f64` |
| `mirna_seed_match_score` | `( seed_matches: usize, three_prime_pairing: f64, site_accessibility: f64, ) -> f64` |
| `lncrna_scaffold_activity` | `( lncrna_conc: f64, protein_a: f64, protein_b: f64, kd_a: f64, kd_b: f64, ) -> f64` |
| `xist_silencing_spread` | `( xist_expression: f64, distance_from_xic: f64, spread_rate: f64, ) -> f64` |
| `pirna_transposon_silencing` | `( pirna_conc: f64, transposon_copies: f64, silencing_efficiency: f64, ) -> f64` |
| `circular_rna_mirna_sponge` | `( circrna: f64, binding_sites: f64, mirna_total: f64, kd: f64, ) -> f64` |
| `rnai_knockdown_efficiency` | `( sirna_conc: f64, target_mrna: f64, risc_activity: f64, kd: f64, ) -> f64` |
| `enhancer_rna_activity` | `( erna_level: f64, enhancer_activity_base: f64, amplification: f64, ) -> f64` |
| `antisense_rna_regulation` | `( sense_mrna: f64, antisense_rna: f64, duplex_rate: f64, degradation_rate: f64, ) -> f64` |
| `ncrna_mediated_methylation` | `( ncrna_guide: f64, target_region_accessibility: f64, dnmt_activity: f64, ) -> f64` |

---

## ethology API (60 functions)

| Function | Signature |
|----------|-----------|
| `territory_size` | `(body_mass: f64, scaling_exponent: f64, constant: f64) -> f64` |
| `territory_defense_cost` | `(perimeter: f64, intruder_rate: f64, cost_per_encounter: f64) -> f64` |
| `boldness_shyness_continuum` | `(stimulus: f64, threshold: f64, steepness: f64) -> f64` |
| `dilution_effect` | `(group_size: f64) -> f64` |
| `many_eyes_detection` | `(individual_detection: f64, group_size: f64) -> f64` |
| `dominance_probability` | `(rating_a: f64, rating_b: f64) -> f64` |
| `allee_effect_growth` | `(n: f64, k: f64, r: f64, a: f64) -> f64` |
| `predator_avoidance_flight_distance` | `(body_mass: f64, scaling: f64, risk_factor: f64) -> f64` |
| `hamilton_relatedness_benefit` | `(relatedness: f64, benefit: f64, cost: f64) -> bool` |
| `reciprocal_altruism_threshold` | `(benefit: f64, cost: f64, probability_future: f64) -> bool` |
| `selfish_herd_risk` | `(distance_to_nearest: f64, predator_speed: f64) -> f64` |
| `vigilance_group_tradeoff` | `(group_size: f64, individual_scan_rate: f64) -> f64` |
| `confusion_effect` | `(group_size: f64, predator_success_solo: f64) -> f64` |
| `mobbing_probability` | `(group_size: f64, predator_danger: f64, threshold: f64) -> f64` |
| `learning_curve_operant` | `(trials: f64, asymptote: f64, rate: f64) -> f64` |
| `stimulus_generalization` | `(distance: f64, width: f64) -> f64` |
| `ideal_despotic_distribution` | `(rank: f64, max_rank: f64, total_resource: f64) -> f64` |
| `aggression_cost_benefit` | `( resource_value: f64, fighting_ability: f64, injury_cost: f64, ) -> f64` |
| `migration_threshold` | `(food_current: f64, food_destination: f64, travel_cost: f64) -> bool` |
| `information_center_benefit` | `(colony_size: f64, discovery_prob: f64) -> f64` |
| `social_network_centrality` | `(connections: f64, max_connections: f64) -> f64` |
| `handicap_signal_cost` | `(quality: f64, signal_intensity: f64, cost_coeff: f64) -> f64` |
| `mate_choice_copying` | `(intrinsic_preference: f64, social_info: f64, weight: f64) -> f64` |
| `signal_detection_d_prime` | `(hit_rate: f64, false_alarm_rate: f64) -> f64` |
| `honest_signal_handicap` | `(quality: f64, cost_per_signal: f64, benefit_per_signal: f64) -> f64` |
| `alarm_call_kin_selection` | `( relatedness: f64, benefit_to_kin: f64, cost_to_caller: f64, ) -> bool` |
| `mate_choice_threshold` | `( quality_assessed: f64, search_cost: f64, encounters: usize, threshold: f64, ) -> bool` |
| `ritualized_contest` | `(size_a: f64, size_b: f64, motivation_a: f64, motivation_b: f64) -> f64` |
| `hawk_dove_contest` | `(v: f64, c: f64, p_hawk: f64) -> (f64, f64)` |
| `producer_scrounger_frequency` | `( producer_payoff: f64, scrounger_payoff: f64, p_producer: f64, selection_strength: f64, ) -> f64` |
| `territory_size_optimal` | `(energy_gain_rate: f64, defense_cost_per_area: f64) -> f64` |
| `dominance_index` | `(wins: f64, total_interactions: f64) -> f64` |
| `optimal_diet_value` | `(energy_gain: f64, handling_time: f64, encounter_rate: f64) -> f64` |
| `marginal_value_theorem` | `( travel_time: f64, gain_curve: impl Fn(f64) -> f64, max_t: f64, ) -> f64` |
| `ideal_free_distribution` | `(resource: &[f64], total_individuals: f64) -> Vec<f64>` |
| `hawk_dove_payoff` | `(v: f64, c: f64, hawk_freq: f64) -> (f64, f64)` |
| `ess_hawk_frequency` | `(v: f64, c: f64) -> f64` |
| `tit_for_tat_payoff` | `(r: f64, s: f64, t: f64, p: f64, opponent_cooperates: bool) -> f64` |
| `prey_choice_ranking` | `(prey_types: &[(f64, f64)]) -> Vec<(usize, f64)>` |
| `risk_sensitive_foraging` | `(mean_gain: f64, variance: f64, risk_aversion: f64) -> f64` |
| `central_place_foraging` | `( distance: f64, load: f64, travel_cost_per_unit: f64, gain_per_load: f64, ) -> f64` |
| `producer_scrounger_game` | `(p_freq: f64, finder_advantage: f64, group_size: f64) -> (f64, f64)` |
| `giving_up_density` | `(metabolic_cost: f64, predation_cost: f64, missed_opportunity: f64) -> f64` |
| `patch_residence_time` | `(gain_rate: f64, travel_time: f64, depletion_rate: f64) -> f64` |
| `functional_response_type_ii` | `(prey_density: f64, attack_rate: f64, handling_time: f64) -> f64` |
| `functional_response_type_iii` | `( prey_density: f64, attack_max: f64, half_sat: f64, handling_time: f64, ) -> f64` |
| `starvation_risk` | `(reserves: f64, daily_cost: f64, variance: f64) -> f64` |
| `cache_pilferage_rate` | `(competitors: f64, detection_prob: f64, cache_density: f64) -> f64` |
| `optimal_load_size` | `( distance: f64, max_load: f64, loading_rate: f64, travel_speed: f64, ) -> f64` |
| `diet_breadth_threshold` | `(energy: &[f64], handling: &[f64], encounter: &[f64]) -> usize` |
| `habituation` | `(response: f64, stimulus_count: usize, decay_rate: f64) -> f64` |
| `sensitization` | `(response: f64, noxious_stimulus: f64, gain: f64, saturation: f64) -> f64` |
| `operant_conditioning_rate` | `(reinforcement_rate: f64, response_rate: f64, k: f64) -> f64` |
| `classical_conditioning_rescorla_wagner` | `(v: f64, alpha: f64, beta: f64, lambda: f64) -> f64` |
| `spatial_learning_distance` | `( trial: usize, asymptote: f64, learning_rate: f64, initial_distance: f64, ) -> f64` |
| `imprinting_critical_period` | `(exposure_time: f64, peak_time: f64, width: f64) -> f64` |
| `social_learning_transmission` | `(demonstrators: f64, naive: f64, transmission_rate: f64) -> f64` |
| `memory_retention_ebbinghaus` | `(strength: f64, time: f64, stability: f64) -> f64` |
| `working_memory_capacity` | `(items: &[f64], capacity: usize) -> f64` |
| `win_stay_lose_shift` | `(previous_outcome: f64, threshold: f64) -> bool` |

---

## evolution API (46 functions)

| Function | Signature |
|----------|-----------|
| `adaptation_rate` | `( selection_coefficient: f64, mutation_rate: f64, population_size: f64, ) -> f64` |
| `fisher_geometric_adaptation` | `(phenotype: &[f64], optimum: &[f64]) -> f64` |
| `adaptive_walk_progress` | `( current_fitness: f64, max_fitness: f64, beneficial_rate: f64, step: usize, ) -> f64` |
| `beneficial_mutation_fixation_probability` | `(s: f64, ne: f64) -> f64` |
| `phenotypic_plasticity` | `( genotype_value: f64, environment: f64, reaction_norm_slope: f64, ) -> f64` |
| `baldwin_effect` | `(learning_rate: f64, genetic_assimilation: f64, generations: usize) -> f64` |
| `red_queen_coevolution` | `( host_fitness: f64, parasite_fitness: f64, host_adapt_rate: f64, parasite_adapt_rate: f64, dt: f64, ) -> (f64, f64)` |
| `environmental_gradient_selection` | `( position: f64, optimum_slope: f64, selection_width: f64, phenotype: f64, ) -> f64` |
| `frequency_dependent_selection` | `( frequency: f64, baseline_fitness: f64, fd_coefficient: f64, ) -> f64` |
| `adaptive_radiation_rate` | `( niche_count: usize, occupied: usize, diversification_rate: f64, ) -> f64` |
| `fitness_landscape_nk` | `(genotype: &[u8], k: usize, contributions: &[Vec<f64>]) -> f64` |
| `fitness_landscape_additive` | `(genotype: &[u8], effects: &[f64]) -> f64` |
| `fisher_geometric_model` | `(distance: f64, n_dimensions: usize) -> f64` |
| `mutation_step_probability` | `(distance: f64, step_size: f64, n_dim: usize) -> f64` |
| `adaptive_walk` | `(distance0: f64, step_size: f64, n_dim: usize, max_steps: usize) -> Vec<f64>` |
| `epistasis` | `(w_ab: f64, w_a: f64, w_b: f64, w_ref: f64) -> f64` |
| `frequency_dependent_fitness` | `(freq: f64, advantage_rare: f64) -> f64` |
| `density_dependent_fitness` | `(population: f64, carrying_capacity: f64, r_max: f64) -> f64` |
| `wrightian_fitness` | `(genotype: &[bool], loci_effects: &[f64], dominance: &[f64]) -> f64` |
| `fitness_landscape_rugged` | `(genotype: &[u8], peaks: &[(&[u8], f64)]) -> f64` |
| `substitution_rate` | `(mu: f64, ne: f64, s: f64) -> f64` |
| `dn_ds_ratio` | `(dn: f64, ds: f64) -> f64` |
| `molecular_clock_rate` | `(substitutions: f64, divergence_time: f64) -> f64` |
| `coalescent_time_pair` | `(ne: f64) -> f64` |
| `expected_segregating_sites` | `(theta: f64, n: usize) -> f64` |
| `watterson_estimator` | `(seg_sites: usize, n: usize) -> f64` |
| `mcdonald_kreitman` | `(dn: f64, ds: f64, pn: f64, ps: f64) -> f64` |
| `neutral_substitution_rate` | `(mutation_rate: f64) -> f64` |
| `effective_neutral_mutations` | `(total_mutations: usize, fraction_neutral: f64) -> f64` |
| `nearly_neutral_boundary` | `(ne: f64) -> f64` |
| `tajima_d` | `(pi: f64, theta_w: f64, n: usize) -> f64` |
| `watterson_theta` | `(segregating_sites: usize, n: usize, sequence_length: usize) -> f64` |
| `nucleotide_diversity` | `(differences: &[f64], n_sequences: usize) -> f64` |
| `ewens_sampling_formula` | `(n: usize, theta: f64) -> f64` |
| `coalescent_expected_time` | `(n: usize, ne: f64) -> f64` |
| `mcdonald_kreitman_ratio` | `(dn: f64, ds: f64, pn: f64, ps: f64) -> f64` |
| `neutrality_index` | `(dn: f64, ds: f64, pn: f64, ps: f64) -> f64` |
| `direction_of_selection` | `(ni: f64) -> f64` |
| `speciation_rate_bdi` | `(lambda: f64, mu: f64, t: f64, n0: f64) -> f64` |
| `allopatric_divergence` | `(d0: f64, mu: f64, t: f64) -> f64` |
| `reproductive_isolation` | `(genetic_distance: f64, k: f64, n: f64) -> f64` |
| `reinforcement_strength` | `(sympatry: f64, hybrid_fitness: f64) -> f64` |
| `yule_process_expected_species` | `(lambda: f64, t: f64) -> f64` |
| `birth_death_survival` | `(lambda: f64, mu: f64, t: f64) -> f64` |
| `character_displacement` | `(z1: f64, z2: f64, alpha: f64, sigma: f64) -> (f64, f64)` |
| `ecological_speciation_fitness` | `( trait_val: f64, optimum1: f64, optimum2: f64, sigma: f64, ) -> (f64, f64)` |

---

## genetics API (59 functions)

| Function | Signature |
|----------|-----------|
| `genetic_drift_wright_fisher` | `( p: f64, pop_size: usize, generations: usize, seed: u64, ) -> Vec<f64>` |
| `drift_effective_population_size` | `(n_males: f64, n_females: f64) -> f64` |
| `effective_population_size_varying` | `(sizes: &[f64]) -> f64` |
| `heterozygosity_loss` | `(ne: f64, generations: usize) -> f64` |
| `mutation_drift_equilibrium` | `(ne: f64, mu: f64) -> f64` |
| `fixation_probability_neutral` | `(ne: f64) -> f64` |
| `fixation_probability_selection` | `(ne: f64, s: f64, p: f64) -> f64` |
| `fixation_time_neutral` | `(ne: f64) -> f64` |
| `bottleneck_heterozygosity` | `(h0: f64, n_bottleneck: f64, generations: usize) -> f64` |
| `hardy_weinberg` | `(p: f64) -> (f64, f64, f64)` |
| `hardy_weinberg_multi` | `(freqs: &[f64]) -> Vec<Vec<f64>>` |
| `chi_squared_hw` | `(observed: &[f64], p: f64, n_total: f64) -> f64` |
| `inbreeding_coefficient` | `(h_obs: f64, h_exp: f64) -> f64` |
| `wahlund_effect` | `(subpop_freqs: &[f64]) -> f64` |
| `fst` | `(subpop_freqs: &[f64]) -> f64` |
| `nei_genetic_distance` | `(pop1_freqs: &[f64], pop2_freqs: &[f64]) -> f64` |
| `expected_heterozygosity` | `(allele_freqs: &[f64]) -> f64` |
| `allele_frequency_cline` | `(x: f64, center: f64, width: f64) -> f64` |
| `effective_population_size` | `(n_males: f64, n_females: f64) -> f64` |
| `recombination_frequency` | `(recombinants: f64, total_offspring: f64) -> f64` |
| `map_distance_kosambi` | `(recombination_freq: f64) -> f64` |
| `map_distance_haldane` | `(recombination_freq: f64) -> f64` |
| `haldane_to_recombination` | `(map_distance: f64) -> f64` |
| `lod_score` | `(theta: f64, recombinants: usize, non_recombinants: usize) -> f64` |
| `three_point_cross_distance` | `(class_counts: &[f64; 8]) -> (f64, f64, f64)` |
| `interference` | `(observed_double_co: f64, expected_double_co: f64) -> f64` |
| `chiasma_frequency` | `(recombination_freq: f64) -> f64` |
| `synaptonemal_complex_length` | `(chromosome_length_mb: f64, loop_size_kb: f64) -> f64` |
| `coalescent_expected_time` | `(n: usize) -> f64` |
| `watterson_theta` | `(segregating_sites: usize, n: usize) -> f64` |
| `fst_pairwise` | `(ht: f64, hs: f64) -> f64` |
| `nucleotide_diversity` | `(sequences: &[&[u8]]) -> f64` |
| `tajima_d` | `(pi: f64, theta_w: f64, n: usize, seg_sites: usize) -> f64` |
| `nei_distance` | `(pop1_freqs: &[f64], pop2_freqs: &[f64]) -> f64` |
| `molecular_heterozygosity` | `(freqs: &[f64]) -> f64` |
| `broad_sense_heritability` | `(genetic_variance: f64, phenotypic_variance: f64) -> f64` |
| `narrow_sense_heritability` | `(additive_variance: f64, phenotypic_variance: f64) -> f64` |
| `breeder_equation` | `(heritability: f64, selection_differential: f64) -> f64` |
| `selection_differential` | `(mean_selected: f64, mean_population: f64) -> f64` |
| `additive_genetic_variance` | `(allele_freq: f64, allele_effect: f64) -> f64` |
| `dominance_variance` | `(allele_freq: f64, dominance_deviation: f64) -> f64` |
| `qtl_effect_from_means` | `(mean_aa: f64, mean_ab: f64, mean_bb: f64) -> (f64, f64)` |
| `phenotypic_variance_components` | `(va: f64, vd: f64, ve: f64, vi: f64) -> f64` |
| `realized_heritability` | `(response: f64, selection_differential: f64) -> f64` |
| `mid_parent_regression` | `( parent1: f64, parent2: f64, heritability: f64, population_mean: f64, ) -> f64` |
| `lander_botstein_lod` | `(n: usize, r_squared: f64) -> f64` |
| `polygenic_score` | `(effects: &[f64], genotypes: &[f64]) -> f64` |
| `falconer_liability_threshold` | `(prevalence: f64) -> f64` |
| `snp_heritability` | `( beta_squared_sum: f64, variance_explained: f64, phenotypic_variance: f64, ) -> f64` |
| `allele_frequency_selection` | `(p: f64, w_aa: f64, w_ab: f64, w_bb: f64) -> f64` |
| `selection_iterate` | `(p0: f64, w_aa: f64, w_ab: f64, w_bb: f64, generations: usize) -> Vec<f64>` |
| `selection_coefficient` | `(w_mutant: f64, w_wildtype: f64) -> f64` |
| `mutation_selection_balance` | `(mu: f64, s: f64) -> f64` |
| `mutation_selection_balance_recessive` | `(mu: f64, s: f64) -> f64` |
| `frequency_dependent_fitness` | `(p: f64, a: f64, b: f64) -> f64` |
| `heterozygote_advantage_equilibrium` | `(s: f64, t: f64) -> f64` |
| `disruptive_selection` | `( p: f64, w_aa: f64, w_ab: f64, w_bb: f64, generations: usize, ) -> Vec<f64>` |
| `truncation_selection` | `(mean: f64, variance: f64, threshold: f64) -> f64` |
| `response_to_selection` | `(heritability: f64, selection_differential: f64) -> f64` |

---

## genomics API (46 functions)

| Function | Signature |
|----------|-----------|
| `gene_density` | `(genes: usize, region_length_mb: f64) -> f64` |
| `cpg_enrichment` | `(cpg_count: usize, c_count: usize, g_count: usize, length: usize) -> f64` |
| `codon_adaptation_index` | `(codon_weights: &[f64]) -> f64` |
| `enc_wright` | `(codon_family_homozygosities: &[f64]) -> f64` |
| `repeat_density` | `(repeat_bases: usize, total_bases: usize) -> f64` |
| `synteny_score` | `(conserved_blocks: usize, total_genes: usize) -> f64` |
| `n50` | `(contig_lengths: &mut [f64]) -> f64` |
| `genome_completeness_busco` | `(complete: usize, fragmented: usize, total_buscos: usize) -> f64` |
| `ka_ks_ratio` | `( nonsynonymous_subs: f64, synonymous_subs: f64, nonsynonymous_sites: f64, synonymous_sites: f64, ) -> f64` |
| `gc_isochore` | `(gc_content: f64) -> &'static str` |
| `pwm_score` | `(pwm: &[Vec<f64>], sequence: &[u8]) -> f64` |
| `pwm_scan` | `(pwm: &[Vec<f64>], sequence: &[u8], threshold: f64) -> Vec<(usize, f64)>` |
| `information_content` | `(pwm: &[Vec<f64>]) -> Vec<f64>` |
| `total_information` | `(pwm: &[Vec<f64>]) -> f64` |
| `consensus_sequence` | `(pwm: &[Vec<f64>]) -> String` |
| `frequency_matrix` | `(sequences: &[&[u8]], length: usize) -> Vec<Vec<f64>>` |
| `find_orfs` | `(sequence: &str, min_length: usize) -> Vec<(usize, usize, String)>` |
| `codon_usage` | `(sequence: &str) -> Vec<(String, usize)>` |
| `reading_frame_proteins` | `(sequence: &str, frame: usize) -> Vec<String>` |
| `gc_content` | `(sequence: &str) -> f64` |
| `gc3_content` | `(sequence: &str) -> f64` |
| `effective_number_of_codons` | `(codon_counts: &[(String, usize)]) -> f64` |
| `longest_orf_length` | `(sequence: &str) -> usize` |
| `nucleotide_frequency` | `(sequence: &str) -> [f64; 4]` |
| `translate` | `(sequence: &str) -> String` |
| `reverse_complement` | `(sequence: &str) -> String` |
| `kmer_count` | `(sequence: &[u8], k: usize) -> Vec<(Vec<u8>, usize)>` |
| `gc_skew` | `(sequence: &[u8], window: usize) -> Vec<f64>` |
| `cpg_observed_expected` | `(sequence: &[u8]) -> f64` |
| `linguistic_complexity` | `(sequence: &[u8]) -> f64` |
| `at_content` | `(sequence: &[u8]) -> f64` |
| `dinucleotide_frequency` | `(sequence: &[u8]) -> Vec<(u8, u8, f64)>` |
| `sequence_entropy` | `(sequence: &[u8]) -> f64` |
| `transition_transversion` | `(sequence_a: &[u8], sequence_b: &[u8]) -> f64` |
| `snp_allele_frequency` | `(alt_count: usize, total_alleles: usize) -> f64` |
| `minor_allele_frequency` | `(allele_freq: f64) -> f64` |
| `hardy_weinberg_expected` | `(p: f64) -> (f64, f64, f64)` |
| `hardy_weinberg_chi_squared` | `(observed: &[f64; 3], expected: &[f64; 3]) -> f64` |
| `ti_tv_ratio` | `(transitions: usize, transversions: usize) -> f64` |
| `heterozygosity` | `(allele_freqs: &[f64]) -> f64` |
| `fst_weir_cockerham` | `(het_within: f64, het_total: f64) -> f64` |
| `linkage_disequilibrium` | `(freq_ab: f64, freq_a: f64, freq_b: f64) -> f64` |
| `r_squared_ld` | `(d: f64, freq_a: f64, freq_b: f64) -> f64` |
| `d_prime` | `(d: f64, freq_a: f64, freq_b: f64) -> f64` |
| `indel_frameshift` | `(indel_length: i64) -> bool` |
| `copy_number_variant_dosage` | `(reads_sample: f64, reads_reference: f64, ploidy: f64) -> f64` |

---

## immunology API (49 functions)

| Function | Signature |
|----------|-----------|
| `clonal_expansion` | `(n0: f64, proliferation_rate: f64, death_rate: f64, t: f64) -> f64` |
| `clonal_selection_dynamics` | `( naive: f64, effector: f64, memory: f64, antigen: f64, k_activation: f64, k_prolif: f64, k_death: f64, k_memory: f64, k_clear: f64, ) -> (f64, f64, f64, f64)` |
| `clonal_selection_simulate` | `( naive0: f64, effector0: f64, memory0: f64, antigen0: f64, k_activation: f64, k_prolif: f64, k_death: f64, k_memory: f64, k_clear: f64, dt: f64, steps: usize, ) -> Vec<(f64, f64, f64, f64)>` |
| `tcell_activation_threshold` | `(signal: f64, costimulation: f64, threshold: f64) -> bool` |
| `cytokine_hill_response` | `(cytokine: f64, ec50: f64, n: f64) -> f64` |
| `treg_suppression` | `(effector_rate: f64, treg: f64, k_supp: f64) -> f64` |
| `memory_recall_response` | `( memory: f64, antigen: f64, k_recall: f64, fold_expansion: f64, ) -> f64` |
| `somatic_affinity_maturation` | `( initial_kd: f64, rounds: usize, improvement_per_round: f64, ) -> f64` |
| `antibody_titer` | `(dilution_factor: f64, endpoint_dilution: usize) -> f64` |
| `isotype_switch_probability` | `( cytokine_signal: f64, switch_region_accessibility: f64, aid_activity: f64, ) -> f64` |
| `somatic_hypermutation_rate` | `(aid_expression: f64, base_rate: f64, hotspot_factor: f64) -> f64` |
| `neutralization_potency` | `(ic50: f64, virus_titer: f64) -> f64` |
| `opsonization_index` | `( antibody_bound: f64, fc_receptor_density: f64, complement_coating: f64, ) -> f64` |
| `adcc_killing_rate` | `( antibody_density: f64, nk_cell_count: f64, target_count: f64, k_sat: f64, ) -> f64` |
| `complement_cascade_c3b` | `(c3: f64, convertase_activity: f64, factor_h_inhibition: f64) -> f64` |
| `multivalent_avidity` | `(monovalent_kd: f64, valency: usize, cooperativity: f64) -> f64` |
| `elisa_concentration` | `(od: f64, od_max: f64, ec50: f64, hill: f64) -> f64` |
| `b_cell_germinal_center_selection` | `(affinity: f64, threshold: f64, t_cell_help: f64) -> bool` |
| `cytokine_production_rate` | `(stimulus: f64, cell_count: f64, max_rate: f64, ec50: f64) -> f64` |
| `cytokine_decay` | `(concentration: f64, half_life: f64, dt: f64) -> f64` |
| `th1_th2_balance` | `(ifn_gamma: f64, il4: f64) -> f64` |
| `th17_regulatory_balance` | `(il17: f64, il10: f64, tgf_beta: f64) -> f64` |
| `cytokine_storm_severity` | `( il6: f64, tnf: f64, il1b: f64, threshold_il6: f64, threshold_tnf: f64, threshold_il1b: f64, ) -> f64` |
| `jak_stat_signaling` | `( cytokine: f64, receptor_density: f64, jak_activity: f64, socs_inhibition: f64, ) -> f64` |
| `nfkb_activation` | `(stimulus: f64, ikk_activity: f64, ikb_level: f64) -> f64` |
| `chemokine_gradient` | `( source_conc: f64, distance: f64, diffusion_coeff: f64, decay_rate: f64, ) -> f64` |
| `autocrine_loop` | `(production_rate: f64, receptor_sensitivity: f64, degradation: f64) -> f64` |
| `paracrine_signaling` | `( source_cells: f64, target_distance: f64, diffusion: f64, production: f64, decay: f64, ) -> f64` |
| `nlrp3_inflammasome_activation` | `( damp_signal: f64, nlrp3: f64, asc: f64, threshold: f64, ) -> f64` |
| `sir_immune` | `( s: f64, i: f64, r: f64, immune: f64, beta: f64, gamma: f64, k_immune: f64, k_decay: f64, ) -> (f64, f64, f64, f64)` |
| `sir_immune_simulate` | `( s0: f64, i0: f64, r0: f64, immune0: f64, beta: f64, gamma: f64, k_immune: f64, k_decay: f64, dt: f64, steps: usize, ) -> Vec<(f64, f64, f64, f64)>` |
| `vaccine_efficacy` | `(arr_vacc: f64, arr_placebo: f64) -> f64` |
| `herd_immunity_fraction` | `(r0: f64) -> f64` |
| `antibody_decay` | `(ab0: f64, half_life: f64, t: f64) -> f64` |
| `booster_response` | `(ab_pre: f64, fold_boost: f64, decay_rate: f64, t: f64) -> f64` |
| `seroconversion_probability` | `(dose: f64, ed50: f64, n: f64) -> f64` |
| `waning_immunity` | `(immune_fraction: f64, waning_rate: f64, t: f64) -> f64` |
| `maternal_antibody_decay` | `(ab0: f64, half_life: f64, t_months: f64) -> f64` |
| `seir_step` | `( s: f64, e: f64, i: f64, r: f64, beta: f64, sigma: f64, gamma: f64, dt: f64, ) -> (f64, f64, f64, f64)` |
| `antigen_antibody_binding` | `(ab: f64, ag: f64, ka: f64) -> f64` |
| `affinity_maturation` | `(kd_initial: f64, mutation_rate: f64, generations: usize) -> Vec<f64>` |
| `avidity` | `(kd_monovalent: f64, n_sites: usize) -> f64` |
| `neutralization_curve` | `(dose: f64, ic50: f64, n: f64) -> f64` |
| `complement_cascade` | `(c0: f64, rate: f64, t: f64) -> f64` |
| `toll_like_receptor_activation` | `(pamp: f64, receptor_density: f64, kd: f64) -> f64` |
| `phagocytosis_rate` | `(pathogen: f64, phagocyte: f64, k_phag: f64, saturation: f64) -> f64` |
| `nk_cell_killing` | `( target_mhc: f64, mhc_threshold: f64, activating_ligand: f64, kill_rate: f64, ) -> f64` |
| `cytokine_cascade` | `( initial_cytokines: &[f64], amplification: &[Vec<f64>], steps: usize, ) -> Vec<Vec<f64>>` |
| `inflammasome_activation` | `(damp: f64, signal2: f64, threshold: f64, nlrp3: f64) -> f64` |

---

## marine_biology API (62 functions)

| Function | Signature |
|----------|-----------|
| `bleaching_thermal_threshold` | `(sst: f64, mmm: f64) -> f64` |
| `dhw_accumulation` | `(dhw_accumulated: f64, hotspot: f64, dt_weeks: f64) -> f64` |
| `calcification_rate` | `( omega_aragonite: f64, temperature: f64, light: f64, max_rate: f64, ) -> f64` |
| `reef_rugosity` | `(surface_distance: f64, linear_distance: f64) -> f64` |
| `coral_recruitment_rate` | `( larval_supply: f64, settlement_rate: f64, post_settlement_survival: f64, available_substrate: f64, ) -> f64` |
| `symbiodinium_density` | `( photosynthesis_rate: f64, respiration_rate: f64, expulsion_rate: f64, dt: f64, current_density: f64, ) -> f64` |
| `ocean_acidification_saturation` | `(ca_conc: f64, co3_conc: f64, ksp: f64) -> f64` |
| `mangrove_carbon_sequestration` | `(biomass: f64, carbon_fraction: f64, growth_rate: f64) -> f64` |
| `seagrass_light_attenuation` | `( surface_irradiance: f64, attenuation_coeff: f64, depth: f64, ) -> f64` |
| `marine_protected_area_spillover` | `( biomass_inside: f64, biomass_outside: f64, perimeter_area_ratio: f64, diffusion: f64, ) -> f64` |
| `trophic_transfer_efficiency` | `(production_upper: f64, production_lower: f64) -> f64` |
| `fish_growth_von_bertalanffy` | `(l_inf: f64, k: f64, t: f64, t0: f64) -> f64` |
| `fish_mortality_total` | `(natural: f64, fishing: f64) -> f64` |
| `maximum_sustainable_yield` | `(r: f64, k: f64) -> f64` |
| `stock_recruitment_beverton_holt` | `(alpha: f64, beta: f64, spawners: f64) -> f64` |
| `stock_recruitment_ricker` | `(alpha: f64, beta: f64, spawners: f64) -> f64` |
| `catch_per_unit_effort` | `(catch: f64, effort: f64) -> f64` |
| `decompression_no_stop_time` | `(depth_m: f64, surface_rate: f64) -> f64` |
| `schaefer_biomass` | `(biomass: f64, r: f64, k: f64, catch: f64, dt: f64) -> f64` |
| `fishing_mortality_from_effort` | `(catchability: f64, effort: f64) -> f64` |
| `yield_per_recruit` | `(f: f64, m: f64, w_inf: f64, k: f64, t_c: f64, t_r: f64, t0: f64) -> f64` |
| `spawning_stock_biomass` | `(numbers: &[f64], weights: &[f64], maturity: &[f64]) -> f64` |
| `exploitation_rate` | `(f: f64, z: f64) -> f64` |
| `virtual_population_analysis` | `(catch: f64, m: f64, f_rate: f64) -> f64` |
| `length_weight_relation` | `(length: f64, a: f64, b: f64) -> f64` |
| `condition_factor_fulton` | `(weight: f64, length: f64) -> f64` |
| `surplus_production` | `(biomass_t: f64, biomass_t1: f64, catch: f64) -> f64` |
| `fox_model_equilibrium_yield` | `(f: f64, msy: f64, f_msy: f64) -> f64` |
| `selectivity_logistic` | `(length: f64, l50: f64, slope: f64) -> f64` |
| `discard_mortality` | `(catch: f64, discard_fraction: f64, discard_survival: f64) -> f64` |
| `osmoregulation_flux` | `( internal_osmolarity: f64, external_osmolarity: f64, permeability: f64, surface_area: f64, ) -> f64` |
| `coral_bleaching_threshold` | `(sst: f64, mmm: f64) -> f64` |
| `degree_heating_weeks` | `(weekly_anomalies: &[f64]) -> f64` |
| `ocean_acidification_ph` | `(pco2: f64, alkalinity: f64, temperature: f64) -> f64` |
| `carbonate_saturation_state` | `(ca_concentration: f64, co3_concentration: f64, ksp: f64) -> f64` |
| `bioluminescence_intensity` | `(luciferin: f64, luciferase: f64, oxygen: f64, km: f64) -> f64` |
| `depth_light_attenuation` | `(surface_irradiance: f64, attenuation_coeff: f64, depth: f64) -> f64` |
| `thermohaline_density` | `(temperature: f64, salinity: f64) -> f64` |
| `mixed_layer_depth_temperature` | `( profile_temps: &[f64], profile_depths: &[f64], threshold: f64, ) -> f64` |
| `ekman_depth` | `(coriolis: f64, eddy_viscosity: f64) -> f64` |
| `ekman_transport` | `(wind_stress: f64, coriolis: f64, density: f64) -> f64` |
| `sverdrup_transport` | `(wind_stress_curl: f64, beta: f64) -> f64` |
| `primary_production_eppley` | `(chlorophyll: f64, par: f64, temperature: f64) -> f64` |
| `new_production_f_ratio` | `(nitrate_uptake: f64, total_uptake: f64) -> f64` |
| `nitrogen_fixation_rate` | `(temperature: f64, iron: f64, light: f64) -> f64` |
| `oxygen_minimum_zone_depth` | `( surface_o2: f64, respiration_rate: f64, ventilation_rate: f64, ) -> f64` |
| `seawater_sound_speed` | `(temperature: f64, salinity: f64, depth: f64) -> f64` |
| `coral_calcification_rate` | `(aragonite_saturation: f64, temperature: f64, light: f64) -> f64` |
| `tidal_range` | `(lunar_amplitude: f64, solar_amplitude: f64, phase_angle: f64) -> f64` |
| `wave_energy_density` | `(density: f64, gravity: f64, wave_height: f64) -> f64` |
| `deep_water_wave_speed` | `(gravity: f64, wavelength: f64) -> f64` |
| `upwelling_velocity` | `( wind_stress: f64, coriolis: f64, density: f64, coast_distance: f64, ) -> f64` |
| `phytoplankton_growth` | `(mu_max: f64, nutrient: f64, ks: f64, light: f64, ki: f64) -> f64` |
| `bloom_critical_depth` | `( surface_irradiance: f64, compensation_irradiance: f64, attenuation: f64, ) -> f64` |
| `sverdrup_critical_depth` | `( avg_irradiance: f64, compensation_irradiance: f64, mixed_layer_depth: f64, attenuation: f64, ) -> f64` |
| `nutrient_phytoplankton_zooplankton_step` | `( n: f64, p: f64, z: f64, dt: f64, mu: f64, ks: f64, grazing: f64, kp: f64, mortality_z: f64, recycling: f64, ) -> (f64, f64, f64)` |
| `chlorophyll_a_from_biomass` | `(biomass: f64, carbon_to_chl_ratio: f64) -> f64` |
| `zooplankton_diel_migration_depth` | `( daytime_depth: f64, nighttime_depth: f64, time_hours: f64, ) -> f64` |
| `harmful_algal_bloom_toxin` | `( cell_density: f64, toxin_per_cell: f64, decay_rate: f64, t: f64, ) -> f64` |
| `redfield_ratio_deviation` | `(c: f64, n: f64, p: f64) -> (f64, f64)` |
| `spring_bloom_timing` | `(mixed_layer_depth: f64, critical_depth: f64) -> bool` |
| `export_production` | `(primary_production: f64, f_ratio: f64) -> f64` |

---

## microbiology API (44 functions)

| Function | Signature |
|----------|-----------|
| `biofilm_formation_rate` | `( planktonic: f64, attachment_rate: f64, surface_area: f64, detachment_rate: f64, biofilm: f64, ) -> f64` |
| `biofilm_thickness` | `(growth_rate: f64, nutrient: f64, ks: f64, detachment: f64, t: f64) -> f64` |
| `extracellular_matrix_production` | `( cell_density: f64, signal: f64, max_rate: f64, threshold: f64, ) -> f64` |
| `biofilm_diffusion_limitation` | `( bulk_conc: f64, thickness: f64, diffusion_biofilm: f64, consumption_rate: f64, ) -> f64` |
| `persister_fraction` | `( antibiotic_conc: f64, mic: f64, base_fraction: f64, max_fraction: f64, ) -> f64` |
| `antibiotic_resistance_mutation_rate` | `( population: f64, mutation_rate: f64, selective_advantage: f64, ) -> f64` |
| `minimum_inhibitory_concentration_ratio` | `(mic_resistant: f64, mic_susceptible: f64) -> f64` |
| `horizontal_gene_transfer` | `(donor: f64, recipient: f64, conjugation_rate: f64) -> f64` |
| `competence_transformation` | `( dna_conc: f64, competent_cells: f64, uptake_rate: f64, integration_efficiency: f64, ) -> f64` |
| `phage_therapy_lysis` | `( phage: f64, bacteria: f64, adsorption_rate: f64, burst_size: f64, latent_period: f64, dt: f64, ) -> (f64, f64)` |
| `chemostat_steady_state_biomass` | `(y: f64, s_in: f64, ks: f64, mu_max: f64, d: f64) -> f64` |
| `chemostat_washout_dilution` | `(mu_max: f64, s_in: f64, ks: f64) -> f64` |
| `minimum_inhibitory_concentration` | `( e0: f64, emax: f64, ec50: f64, n: f64, target_kill: f64, ) -> f64` |
| `competitive_exclusion` | `( x1: f64, x2: f64, s: f64, mu1: f64, mu2: f64, ks1: f64, ks2: f64, y1: f64, y2: f64, d: f64, s_in: f64, ) -> (f64, f64, f64)` |
| `serial_dilution` | `( n0: f64, dilution_factor: f64, transfers: usize, growth_per_cycle: f64, ) -> Vec<f64>` |
| `biofilm_formation` | `( planktonic: f64, attachment_rate: f64, detachment_rate: f64, biofilm: f64, carrying_capacity: f64, ) -> (f64, f64)` |
| `quorum_sensing` | `( cell_density: f64, autoinducer_production: f64, threshold: f64, n_hill: f64, ) -> f64` |
| `colony_forming_units` | `(od600: f64, calibration_factor: f64) -> f64` |
| `turbidostat` | `(biomass: f64, target_od: f64, mu: f64, dt: f64) -> f64` |
| `ph_growth_response` | `(ph: f64, ph_opt: f64, ph_min: f64, ph_max: f64) -> f64` |
| `monod_growth` | `(mu_max: f64, s: f64, ks: f64) -> f64` |
| `monod_dynamics` | `( x: f64, s: f64, mu_max: f64, ks: f64, y: f64, d: f64, s_in: f64, ) -> (f64, f64)` |
| `monod_simulate` | `( x0: f64, s0: f64, mu_max: f64, ks: f64, y: f64, d: f64, s_in: f64, dt: f64, steps: usize, ) -> Vec<(f64, f64)>` |
| `bacterial_growth_phases` | `(n0: f64, mu: f64, k: f64, lag: f64, t: f64) -> f64` |
| `generation_time_bacteria` | `(mu: f64) -> f64` |
| `death_phase` | `(n_peak: f64, death_rate: f64, t: f64) -> f64` |
| `diauxic_growth` | `(s1: f64, s2: f64, mu1: f64, mu2: f64, ks1: f64, ks2: f64, ki: f64) -> f64` |
| `fermentation_yield` | `(substrate: f64, yield_coefficient: f64) -> f64` |
| `anaerobic_atp_yield` | `(glucose_moles: f64, pathway_efficiency: f64) -> f64` |
| `chemolithoautotrophy_energy` | `(delta_g: f64, moles_substrate: f64, efficiency: f64) -> f64` |
| `nitrogen_fixation_cost` | `(n2_fixed: f64, atp_per_n2: f64) -> f64` |
| `denitrification_rate` | `( no3: f64, carbon_source: f64, max_rate: f64, ks_no3: f64, ks_c: f64, ) -> f64` |
| `sulfate_reduction_rate` | `( sulfate: f64, electron_donor: f64, max_rate: f64, ks_so4: f64, ks_donor: f64, ) -> f64` |
| `methanogenesis_rate` | `( co2: f64, h2: f64, max_rate: f64, ks_co2: f64, ks_h2: f64, temperature: f64, ) -> f64` |
| `anammox_rate` | `(nh4: f64, no2: f64, max_rate: f64, ks_nh4: f64, ks_no2: f64) -> f64` |
| `iron_oxidation_rate` | `(fe2: f64, o2: f64, ph: f64, max_rate: f64) -> f64` |
| `bioremediation_degradation` | `( contaminant: f64, biomass: f64, max_rate: f64, ks: f64, inhibition_conc: f64, ) -> f64` |
| `quorum_sensing_ahl` | `(n: f64, k_prod: f64, k_deg: f64, diffusion: f64) -> f64` |
| `quorum_activation` | `(ahl: f64, threshold: f64, n: f64) -> f64` |
| `quorum_bistable` | `( ahl0: f64, n_cells: f64, k_prod: f64, k_deg: f64, k_auto: f64, threshold: f64, hill_n: f64, dt: f64, steps: usize, ) -> Vec<f64>` |
| `biofilm_growth` | `( b: f64, mu: f64, k_attach: f64, planktonic: f64, k_detach: f64, k_max: f64, ) -> f64` |
| `biofilm_simulate` | `( b0: f64, planktonic0: f64, mu: f64, k_attach: f64, k_detach: f64, k_max: f64, growth_p: f64, dt: f64, steps: usize, ) -> Vec<(f64, f64)>` |
| `antibiotic_kill` | `(n: f64, mic: f64, conc: f64, k_max: f64, hill: f64) -> f64` |
| `mutation_resistance_probability` | `(mu: f64, n: f64) -> f64` |

---

## mycology API (53 functions)

| Function | Signature |
|----------|-----------|
| `decomposition_rate` | `(k: f64, mass: f64) -> f64` |
| `remaining_mass` | `(initial: f64, k: f64, t: f64) -> f64` |
| `lignocellulose_decay` | `( cellulose: f64, lignin: f64, k_cellulose: f64, k_lignin: f64, dt: f64, ) -> (f64, f64)` |
| `enzyme_mediated_decomposition` | `(substrate: f64, enzyme: f64, vmax: f64, km: f64) -> f64` |
| `mycorrhizal_nutrient_exchange` | `( plant_carbon: f64, fungal_phosphorus: f64, exchange_coeff: f64, ) -> (f64, f64)` |
| `saprotrophic_efficiency` | `(carbon_assimilated: f64, carbon_consumed: f64) -> f64` |
| `humus_formation_rate` | `( recalcitrant_fraction: f64, decomposition_rate: f64, carbon_input: f64, ) -> f64` |
| `nitrogen_mineralization` | `( cn_ratio_substrate: f64, cn_ratio_microbe: f64, carbon_flow: f64, ) -> f64` |
| `white_rot_lignin_degradation` | `( lignin: f64, laccase_activity: f64, peroxidase_activity: f64, km: f64, ) -> f64` |
| `brown_rot_cellulose_attack` | `( cellulose: f64, hydroxyl_radical: f64, rate_constant: f64, ) -> f64` |
| `soft_rot_cavity_formation` | `( cellulose: f64, moisture: f64, enzyme_activity: f64, threshold_moisture: f64, ) -> f64` |
| `mycorrhizal_carbon_cost` | `(plant_npp: f64, allocation_fraction: f64) -> f64` |
| `ectomycorrhizal_hyphal_exploration` | `( biomass: f64, soil_volume: f64, exploration_type_factor: f64, ) -> f64` |
| `arbuscular_mycorrhizal_phosphorus_uptake` | `( hyphal_length: f64, soil_p: f64, uptake_rate: f64, km: f64, ) -> f64` |
| `wood_decay_mass_loss` | `( initial_density: f64, fungal_activity: f64, moisture_factor: f64, temp_factor: f64, t: f64, ) -> f64` |
| `litter_quality_index` | `(nitrogen_content: f64, lignin_content: f64) -> f64` |
| `carbon_use_efficiency` | `(co2_respired: f64, carbon_assimilated: f64) -> f64` |
| `saprotrophic_decomposition` | `( substrate_mass: f64, enzyme_activity: f64, moisture: f64, temperature: f64, optimal_temp: f64, ) -> f64` |
| `wood_decay_rate` | `( lignin_fraction: f64, cellulose_fraction: f64, fungal_type_lignin_pref: f64, ) -> f64` |
| `fungal_succession_priority` | `(colonization_order: usize, competitive_ability: f64) -> f64` |
| `spore_germination_rate` | `( moisture: f64, temperature: f64, temp_min: f64, temp_max: f64, temp_opt: f64, ) -> f64` |
| `fairy_ring_expansion` | `(ring_radius: f64, growth_rate: f64, nutrient_depletion: f64) -> f64` |
| `ergosterol_biomass_estimate` | `(ergosterol_conc: f64, conversion_factor: f64) -> f64` |
| `fungal_carbon_mineralization` | `(biomass: f64, cue: f64, substrate_consumed: f64) -> f64` |
| `mycelial_network_resilience` | `(connections: usize, nodes: usize, redundancy: f64) -> f64` |
| `hyphal_growth_rate` | `(tip_extension: f64, branching_rate: f64, tips: f64) -> f64` |
| `colony_radial_growth` | `(r0: f64, rate: f64, t: f64) -> f64` |
| `spore_germination_probability` | `( water_activity: f64, temperature: f64, aw_min: f64, t_min: f64, t_max: f64, ) -> f64` |
| `mycelial_network_transport` | `( concentration_source: f64, concentration_sink: f64, conductance: f64, ) -> f64` |
| `chitin_content` | `(dry_mass: f64, chitin_fraction: f64) -> f64` |
| `fungal_biomass_from_ergosterol` | `(ergosterol_ug: f64, conversion_factor: f64) -> f64` |
| `substrate_colonization_speed` | `(growth_rate: f64, nutrient_availability: f64, km: f64) -> f64` |
| `fairy_ring_radius` | `(initial_radius: f64, expansion_rate: f64, t: f64) -> f64` |
| `spore_dispersal_distance` | `( wind_speed: f64, release_height: f64, terminal_velocity: f64, ) -> f64` |
| `yeast_budding_rate` | `( nutrient: f64, temperature: f64, optimal_temp: f64, temp_width: f64, ) -> f64` |
| `mycelial_biomass_logistic` | `(biomass: f64, max_biomass: f64, mu_max: f64, dt: f64) -> f64` |
| `branching_frequency` | `(hyphal_length: f64, branch_count: f64) -> f64` |
| `hyphal_tip_vesicle_supply` | `( vesicle_production: f64, distance_to_tip: f64, diffusion: f64, ) -> f64` |
| `conidiation_rate` | `(nutrient_depletion: f64, light_signal: f64, threshold: f64) -> f64` |
| `rhizomorph_transport_rate` | `( pressure_gradient: f64, conductance: f64, cross_section: f64, ) -> f64` |
| `lichenization_benefit` | `( algal_photosynthate: f64, fungal_protection: f64, exchange_rate: f64, ) -> f64` |
| `spore_survival_uv` | `(initial_viability: f64, uv_dose: f64, sensitivity: f64) -> f64` |
| `monod_fungal_growth` | `(mu_max: f64, substrate: f64, ks: f64) -> f64` |
| `biofilm_formation_rate` | `( cell_density: f64, surface_affinity: f64, quorum_signal: f64, threshold: f64, ) -> f64` |
| `mycorrhizal_nutrient_transfer` | `( root_surface_area: f64, hyphal_density: f64, nutrient_conc: f64, transfer_efficiency: f64, ) -> f64` |
| `mycorrhizal_carbon_allocation` | `(carbon_to_fungus: f64, total_photosynthate: f64) -> f64` |
| `mycorrhizal_colonization` | `( inoculum_potential: f64, root_growth_rate: f64, susceptibility: f64, time: f64, ) -> f64` |
| `common_mycorrhizal_network_transfer` | `( donor_surplus: f64, recipient_deficit: f64, network_conductance: f64, ) -> f64` |
| `lichen_photobiont_contribution` | `( photobiont_biomass: f64, photosynthesis_rate: f64, transfer_fraction: f64, ) -> f64` |
| `endophyte_benefit` | `(plant_growth_base: f64, endophyte_effect: f64, stress_level: f64) -> f64` |
| `fungal_network_distance` | `(hyphal_growth_rate: f64, branching_angle: f64, time: f64) -> f64` |
| `truffle_spore_dispersal` | `( spore_count: f64, wind_speed: f64, animal_vectors: f64, decay_distance: f64, distance: f64, ) -> f64` |
| `mycobiome_diversity_shannon` | `(abundances: &[f64]) -> f64` |

---

## neuroscience API (54 functions)

| Function | Signature |
|----------|-----------|
| `firing_rate` | `(spikes: &[usize], dt: f64, total_steps: usize) -> f64` |
| `interspike_intervals` | `(spikes: &[usize], dt: f64) -> Vec<f64>` |
| `coefficient_of_variation` | `(intervals: &[f64]) -> f64` |
| `fano_factor` | `(spike_counts: &[usize]) -> f64` |
| `spike_count_correlation` | `(spikes_a: &[usize], spikes_b: &[usize]) -> f64` |
| `cross_correlogram` | `( spikes_a: &[f64], spikes_b: &[f64], bin_width: f64, max_lag: f64, ) -> Vec<(f64, usize)>` |
| `local_field_potential_power` | `(signal: &[f64], freq: f64, dt: f64) -> f64` |
| `spike_triggered_average` | `(stimulus: &[f64], spike_times: &[usize], window: usize) -> Vec<f64>` |
| `burst_detection` | `(isi: &[f64], threshold: f64) -> Vec<(usize, usize)>` |
| `information_rate` | `(spike_counts: &[usize], stimulus_repeats: usize) -> f64` |
| `drift_diffusion_decision` | `( drift_rate: f64, noise: f64, threshold: f64, bias: f64, dt: f64, steps: usize, ) -> (f64, usize)` |
| `softmax_choice` | `(values: &[f64], temperature: f64) -> Vec<f64>` |
| `rescorla_wagner_update` | `(value: f64, reward: f64, alpha: f64) -> f64` |
| `td_learning_update` | `( value_current: f64, value_next: f64, reward: f64, alpha: f64, gamma: f64, ) -> f64` |
| `reward_prediction_error` | `(reward: f64, expected: f64) -> f64` |
| `weber_fraction` | `(jnd: f64, stimulus_intensity: f64) -> f64` |
| `signal_to_noise_neural` | `(signal_mean: f64, noise_std: f64) -> f64` |
| `attentional_gain` | `(stimulus: f64, attention: f64, baseline: f64, gain: f64) -> f64` |
| `working_memory_decay` | `( strength: f64, decay_rate: f64, interference_count: usize, dt: f64, ) -> f64` |
| `neural_efficiency` | `(performance: f64, metabolic_cost: f64) -> f64` |
| `bayesian_integration` | `( prior_mean: f64, prior_var: f64, likelihood_mean: f64, likelihood_var: f64, ) -> (f64, f64)` |
| `step` | `(&mut self, i_ext: f64, dt: f64)` |
| `simulate` | `(&mut self, i_ext: f64, dt: f64, steps: usize) -> Vec<f64>` |
| `new` | `(a: f64, b: f64, tau: f64) -> Self` |
| `step` | `(&mut self, i_ext: f64, dt: f64)` |
| `simulate` | `(&mut self, i_ext: f64, dt: f64, steps: usize) -> Vec<(f64, f64)>` |
| `new` | `(v_rest: f64, v_thresh: f64, v_reset: f64, tau: f64, r: f64) -> Self` |
| `step` | `(&mut self, i_ext: f64, dt: f64) -> bool` |
| `simulate` | `(&mut self, i_ext: f64, dt: f64, steps: usize) -> (Vec<f64>, Vec<usize>)` |
| `regular_spiking` | `() -> Self` |
| `fast_spiking` | `() -> Self` |
| `bursting` | `() -> Self` |
| `step` | `(&mut self, i_ext: f64, dt: f64) -> bool` |
| `simulate` | `(&mut self, i_ext: f64, dt: f64, steps: usize) -> (Vec<f64>, Vec<usize>)` |
| `step` | `(&mut self, i_ext: f64, dt: f64)` |
| `simulate` | `(&mut self, i_ext: f64, dt: f64, steps: usize) -> Vec<(f64, f64)>` |
| `excitatory` | `(weight: f64) -> Self` |
| `inhibitory` | `(weight: f64) -> Self` |
| `step` | `(&mut self, pre_spike: bool, dt: f64)` |
| `current` | `(&self, v_post: f64) -> f64` |
| `stdp_update` | `(delta_t: f64, a_plus: f64, a_minus: f64, tau_plus: f64, tau_minus: f64) -> f64` |
| `simulate_network` | `( n_neurons: usize, weights: &[Vec<f64>], external_current: f64, dt: f64, steps: usize, threshold: f64, reset: f64, tau: f64, resistance: f64, rest: f64, ) -> Vec<Vec<f64>>` |
| `mean_field_rate` | `(mu: f64, sigma: f64, threshold: f64, reset: f64, tau: f64) -> f64` |
| `ltp_magnitude` | `( stimulus_frequency: f64, calcium_influx: f64, threshold: f64, max_potentiation: f64, ) -> f64` |
| `ltd_magnitude` | `( calcium_level: f64, low_threshold: f64, high_threshold: f64, max_depression: f64, ) -> f64` |
| `stdp_weight_change` | `( delta_t_ms: f64, a_plus: f64, a_minus: f64, tau_plus: f64, tau_minus: f64, ) -> f64` |
| `synaptic_vesicle_release_probability` | `(calcium: f64, n_vesicles: usize, p_single: f64) -> f64` |
| `short_term_facilitation` | `(baseline_p: f64, facilitation: f64, tau: f64, isi: f64) -> f64` |
| `short_term_depression` | `(available: f64, release_p: f64, recovery_tau: f64, isi: f64) -> f64` |
| `miniature_epsp_amplitude` | `( quantal_size: f64, n_receptors: f64, receptor_conductance: f64, ) -> f64` |
| `nmda_voltage_dependence` | `(voltage: f64, mg_conc: f64) -> f64` |
| `dendritic_spine_volume_change` | `( calcium: f64, actin_polymerization: f64, spine_volume: f64, max_growth: f64, ) -> f64` |
| `homeostatic_synaptic_scaling` | `( target_rate: f64, current_rate: f64, scaling_tau: f64, dt: f64, ) -> f64` |
| `glutamate_clearance` | `(released: f64, transporter_density: f64, vmax: f64, km: f64) -> f64` |

---

## nutrition API (64 functions)

| Function | Signature |
|----------|-----------|
| `nutrient_absorption_first_order` | `(dose: f64, ka: f64, t: f64) -> f64` |
| `gastric_emptying` | `(volume: f64, half_life: f64, t: f64) -> f64` |
| `glycemic_index_incremental_auc` | `(glucose_values: &[f64], baseline: f64, dt: f64) -> f64` |
| `protein_digestibility_corrected_amino_acid_score` | `( limiting_aa_mg_g: f64, reference_mg_g: f64, digestibility: f64, ) -> f64` |
| `nitrogen_balance` | `(protein_intake_g: f64, urinary_n: f64, fecal_n: f64, sweat_n: f64) -> f64` |
| `water_requirement_holliday_segar` | `(weight_kg: f64) -> f64` |
| `iron_absorption` | `(non_heme_mg: f64, enhancers: f64, inhibitors: f64, heme_mg: f64) -> f64` |
| `calcium_absorption_fraction` | `(intake_mg: f64, vitamin_d_nmol: f64) -> f64` |
| `intestinal_transit_time` | `(fiber_g: f64, fluid_ml: f64, base_time_h: f64) -> f64` |
| `oral_bioavailability` | `( fraction_absorbed: f64, gut_wall_extraction: f64, hepatic_extraction: f64, ) -> f64` |
| `michaelis_menten_absorption` | `(vmax: f64, concentration: f64, km: f64) -> f64` |
| `fat_soluble_vitamin_absorption` | `(dose: f64, fat_intake_g: f64, bile_salt_conc: f64) -> f64` |
| `zinc_absorption_fraction` | `(intake_mg: f64, phytate_mg: f64) -> f64` |
| `paracellular_absorption` | `(permeability: f64, surface_area: f64, concentration: f64) -> f64` |
| `glucose_transporter_kinetics` | `(glucose: f64, vmax: f64, km: f64, insulin_factor: f64) -> f64` |
| `amino_acid_absorption_rate` | `( concentration: f64, vmax: f64, km: f64, competition_factor: f64, ) -> f64` |
| `basal_metabolic_rate_mifflin` | `( weight_kg: f64, height_cm: f64, age: f64, is_male: bool, ) -> f64` |
| `tdee` | `(bmr: f64, activity_factor: f64, thermic_effect: f64) -> f64` |
| `energy_balance` | `(intake_kcal: f64, expenditure_kcal: f64) -> f64` |
| `weight_change_prediction` | `(energy_balance_kcal_per_day: f64, days: f64) -> f64` |
| `diet_induced_thermogenesis` | `(protein_kcal: f64, carb_kcal: f64, fat_kcal: f64) -> f64` |
| `respiratory_exchange_ratio` | `(co2_produced: f64, o2_consumed: f64) -> f64` |
| `substrate_oxidation_from_rer` | `(rer: f64) -> (f64, f64)` |
| `glycemic_index_load` | `(gi: f64, carb_grams: f64) -> f64` |
| `insulin_index_response` | `(glycemic_load: f64, protein_factor: f64, protein_grams: f64) -> f64` |
| `body_composition_bmi` | `(weight_kg: f64, height_m: f64) -> f64` |
| `body_fat_percentage_navy` | `(waist_cm: f64, neck_cm: f64, height_cm: f64, is_male: bool) -> f64` |
| `harris_benedict_male` | `(weight_kg: f64, height_cm: f64, age: f64) -> f64` |
| `harris_benedict_female` | `(weight_kg: f64, height_cm: f64, age: f64) -> f64` |
| `mifflin_st_jeor_male` | `(weight_kg: f64, height_cm: f64, age: f64) -> f64` |
| `mifflin_st_jeor_female` | `(weight_kg: f64, height_cm: f64, age: f64) -> f64` |
| `total_daily_energy_expenditure` | `(bmr: f64, activity_factor: f64) -> f64` |
| `thermic_effect_of_food` | `(caloric_intake: f64, tef_fraction: f64) -> f64` |
| `body_mass_index` | `(weight_kg: f64, height_m: f64) -> f64` |
| `lean_body_mass_boer_male` | `(weight_kg: f64, height_cm: f64) -> f64` |
| `lean_body_mass_boer_female` | `(weight_kg: f64, height_cm: f64) -> f64` |
| `body_fat_percentage` | `(total_mass: f64, lean_mass: f64) -> f64` |
| `katch_mcardle_bmr` | `(lean_body_mass_kg: f64) -> f64` |
| `cunningham_bmr` | `(lean_body_mass_kg: f64) -> f64` |
| `respiratory_quotient` | `(co2_produced: f64, o2_consumed: f64) -> f64` |
| `energy_from_macros` | `(carb_g: f64, protein_g: f64, fat_g: f64, alcohol_g: f64) -> f64` |
| `waist_to_hip_ratio` | `(waist_cm: f64, hip_cm: f64) -> f64` |
| `body_surface_area_dubois` | `(weight_kg: f64, height_cm: f64) -> f64` |
| `ideal_body_weight_devine_male` | `(height_cm: f64) -> f64` |
| `ideal_body_weight_devine_female` | `(height_cm: f64) -> f64` |
| `adjusted_body_weight` | `(actual_kg: f64, ideal_kg: f64) -> f64` |
| `resting_metabolic_rate_owen_male` | `(weight_kg: f64) -> f64` |
| `resting_metabolic_rate_owen_female` | `(weight_kg: f64) -> f64` |
| `glycemic_load` | `(glycemic_index: f64, available_carbs_g: f64) -> f64` |
| `fat_oxidation_rate` | `(vo2: f64, vco2: f64) -> f64` |
| `carb_oxidation_rate` | `(vco2: f64, vo2: f64) -> f64` |
| `protein_requirement_rda` | `(weight_kg: f64) -> f64` |
| `estimated_average_requirement_calcium` | `(age: f64) -> f64` |
| `recommended_daily_intake_scaling` | `(body_weight_kg: f64, rdi_per_kg: f64) -> f64` |
| `vitamin_d_synthesis` | `( uvb_intensity: f64, skin_area: f64, melanin_factor: f64, exposure_minutes: f64, ) -> f64` |
| `iron_absorption_enhancers` | `(non_heme_iron: f64, vitamin_c_mg: f64, meat_factor: f64) -> f64` |
| `calcium_absorption` | `(intake: f64, vitamin_d: f64, age_factor: f64) -> f64` |
| `zinc_copper_ratio` | `(zinc_intake: f64, copper_intake: f64) -> f64` |
| `bioavailability_factor` | `( intake: f64, absorption_fraction: f64, first_pass_extraction: f64, ) -> f64` |
| `folate_neural_tube_risk` | `(folate_ug: f64, risk_base: f64, protective_threshold: f64) -> f64` |
| `omega3_omega6_ratio` | `(omega3: f64, omega6: f64) -> f64` |
| `antioxidant_capacity_orac` | `(concentration: f64, orac_per_unit: f64) -> f64` |
| `iodine_thyroid_requirement` | `( body_weight_kg: f64, base_requirement_ug_per_kg: f64, pregnancy_factor: f64, ) -> f64` |
| `nutrient_density_score` | `(nutrients: &[f64], daily_values: &[f64], calories: f64) -> f64` |

---

## paleobiology API (50 functions)

| Function | Signature |
|----------|-----------|
| `radiometric_age` | `(parent: f64, daughter: f64, decay_constant: f64) -> f64` |
| `half_life_to_decay_constant` | `(half_life: f64) -> f64` |
| `carbon14_age` | `(ratio_sample: f64, ratio_modern: f64) -> f64` |
| `extinction_rate` | `(extinctions: f64, taxa_at_start: f64, interval_myr: f64) -> f64` |
| `origination_rate` | `(originations: f64, taxa_at_end: f64, interval_myr: f64) -> f64` |
| `net_diversification_rate` | `(origination: f64, extinction: f64) -> f64` |
| `turnover_rate` | `(origination: f64, extinction: f64) -> f64` |
| `survivorship_cohort` | `(initial: f64, extinction_rate: f64, t_myr: f64) -> f64` |
| `standing_diversity` | `(origination_rate: f64, extinction_rate: f64, d0: f64, t: f64) -> f64` |
| `taxonomic_rate_sampling_corrected` | `( observed_crossers: f64, singletons: f64, total: f64, ) -> f64` |
| `stratigraphic_completeness` | `(gaps_duration: f64, total_duration: f64) -> f64` |
| `confidence_interval_range` | `(known_range: f64, n_horizons: f64, confidence: f64) -> f64` |
| `logistic_diversity` | `(d0: f64, r: f64, k: f64, t: f64) -> f64` |
| `recovery_time_after_extinction` | `( pre_extinction: f64, post_extinction: f64, diversification_rate: f64, ) -> f64` |
| `signor_lipps_correction` | `(observed_last: f64, sampling_prob: f64, n_taxa: f64) -> f64` |
| `potassium_argon_age` | `(k40: f64, ar40: f64) -> f64` |
| `faith_phylogenetic_diversity` | `(branch_lengths: &[f64]) -> f64` |
| `mean_pairwise_distance` | `(distances: &[f64], n_taxa: usize) -> f64` |
| `net_relatedness_index` | `(mpd_observed: f64, mpd_null_mean: f64, mpd_null_sd: f64) -> f64` |
| `nearest_taxon_index` | `(mntd_observed: f64, mntd_null_mean: f64, mntd_null_sd: f64) -> f64` |
| `evolutionary_distinctiveness` | `(terminal_branch_length: f64, clade_size: usize) -> f64` |
| `phylogenetic_species_variability` | `(species_correlation_sum: f64, n_species: usize) -> f64` |
| `diversification_rate` | `(n_extant: f64, stem_age: f64) -> f64` |
| `lineage_through_time_expected` | `(n0: f64, birth_rate: f64, death_rate: f64, t: f64) -> f64` |
| `gamma_statistic` | `(branching_times: &[f64]) -> f64` |
| `background_extinction_rate` | `(species_lost: f64, total_species: f64, time_my: f64) -> f64` |
| `mass_extinction_magnitude` | `(species_before: f64, species_after: f64) -> f64` |
| `recovery_time_exponential` | `(species_lost_fraction: f64, origination_rate: f64) -> f64` |
| `kill_curve_severity` | `( environmental_perturbation: f64, vulnerability: f64, threshold: f64, ) -> f64` |
| `selectivity_index` | `(extinction_rate_group: f64, extinction_rate_background: f64) -> f64` |
| `origination_extinction_balance` | `(origination_rate: f64, extinction_rate: f64) -> f64` |
| `survivorship_curve` | `(initial_cohort: f64, extinction_rate: f64, t: f64) -> f64` |
| `lazarus_taxon_probability` | `( true_extinction: f64, sampling_probability: f64, gap_duration: f64, ) -> f64` |
| `signor_lipps_effect` | `(last_appearance: f64, sampling_interval: f64) -> f64` |
| `biodiversity_through_time` | `( origination_rate: f64, extinction_rate: f64, initial_diversity: f64, t: f64, ) -> f64` |
| `waiting_time_to_extinction` | `(population_size: f64, extinction_rate: f64) -> f64` |
| `morphological_disparity` | `(trait_values: &[Vec<f64>]) -> f64` |
| `rarefied_diversity` | `(abundances: &[usize], sample_size: usize) -> f64` |
| `foote_boundary_crosser_rate` | `(n_bt: f64, n_fl: f64) -> f64` |
| `completeness_index` | `(known_intervals: f64, total_range: f64) -> f64` |
| `lazarus_ratio` | `(lazarus_taxa: f64, total_taxa: f64) -> f64` |
| `body_size_cope_trend` | `(sizes: &[f64]) -> f64` |
| `morphospace_range` | `(trait_values: &[f64]) -> f64` |
| `morphospace_volume` | `(ranges: &[f64]) -> f64` |
| `pairwise_morphological_distance` | `(a: &[f64], b: &[f64]) -> f64` |
| `evolutionary_rate_darwin` | `(size_initial: f64, size_final: f64, time_myr: f64) -> f64` |
| `evolutionary_rate_haldane` | `( size_initial: f64, size_final: f64, time_generations: f64, pooled_sd: f64, ) -> f64` |
| `taphonomic_bias` | `(original_richness: f64, preservation_prob: f64) -> f64` |
| `ghost_lineage_duration` | `(first_appearance: f64, inferred_origin: f64) -> f64` |
| `disparity_centroid_distance` | `(taxon: &[f64], centroid: &[f64]) -> f64` |

---

## parasitology API (55 functions)

| Function | Signature |
|----------|-----------|
| `parasite_r0` | `(beta: f64, host_density: f64, recovery_rate: f64, mortality_rate: f64) -> f64` |
| `parasite_transmission_rate` | `( contact_rate: f64, infectivity: f64, density_susceptible: f64, density_infected: f64, ) -> f64` |
| `sir_parasite_step` | `( s: f64, i: f64, r: f64, beta: f64, gamma: f64, mu: f64, dt: f64, ) -> (f64, f64, f64)` |
| `parasite_aggregation_k` | `(mean_burden: f64, variance_burden: f64) -> f64` |
| `parasite_negative_binomial_prevalence` | `(mean_burden: f64, k: f64) -> f64` |
| `superinfection_rate` | `( current_parasites: usize, max_parasites: usize, exposure_rate: f64, ) -> f64` |
| `vector_borne_r0` | `( mosquito_density: f64, biting_rate: f64, prob_m_to_h: f64, prob_h_to_m: f64, mosquito_mortality: f64, extrinsic_incubation: f64, recovery: f64, ) -> f64` |
| `definitive_intermediate_host_cycle` | `( cercariae_production: f64, snail_infection_rate: f64, human_exposure: f64, worm_establishment: f64, ) -> f64` |
| `host_parasite_lotka_volterra` | `( h: f64, p: f64, r: f64, k: f64, a: f64, c: f64, d: f64, ) -> (f64, f64)` |
| `anderson_may` | `( h: f64, p: f64, alpha: f64, beta: f64, b: f64, d_h: f64, d_p: f64, k_aggregation: f64, ) -> (f64, f64)` |
| `negative_binomial_prevalence` | `(mean_burden: f64, k: f64) -> f64` |
| `parasite_aggregation_index` | `(variance: f64, mean: f64) -> f64` |
| `superinfection_probability` | `(exposure_rate: f64, current_burden: f64, max_burden: f64) -> f64` |
| `basic_reproduction_number_parasite` | `( beta: f64, lambda: f64, h: f64, mu_p: f64, mu_h: f64, alpha: f64, ) -> f64` |
| `coevolution_red_queen` | `( host_fitness: f64, parasite_fitness: f64, arms_race_rate: f64, ) -> (f64, f64)` |
| `nicholson_bailey` | `(h: f64, p: f64, r: f64, a: f64) -> (f64, f64)` |
| `negative_binomial_zero_class` | `(mean_burden: f64, k: f64) -> f64` |
| `parasite_induced_mortality` | `(alpha: f64, burden: f64) -> f64` |
| `acquired_immunity_reduction` | `(exposure: f64, max_immunity: f64, half_exposure: f64) -> f64` |
| `intermediate_host_prevalence` | `(beta: f64, h2: f64, mu_l: f64, mu_h2: f64) -> f64` |
| `cercarial_force_of_infection` | `( cercarial_density: f64, contact_rate: f64, penetration_prob: f64, ) -> f64` |
| `predator_prey_parasite_manipulation` | `( h: f64, p: f64, prey: f64, r: f64, a: f64, manipulation_factor: f64, conversion: f64, death: f64, ) -> (f64, f64)` |
| `density_dependent_fecundity` | `(fecundity_max: f64, burden: f64, k_dens: f64) -> f64` |
| `mate_probability_dioecious` | `(burden: f64, k_agg: f64) -> f64` |
| `parasite_free_equilibrium` | `(birth: f64, death: f64, carrying_capacity: f64) -> f64` |
| `antigenic_variation_escape` | `( immune_recognition: f64, switch_rate: f64, variants: usize, ) -> f64` |
| `immune_evasion_molecular_mimicry` | `( host_molecule_similarity: f64, immune_response_base: f64, ) -> f64` |
| `immunosuppression_by_parasite` | `( il10_induction: f64, treg_expansion: f64, effector_response: f64, ) -> f64` |
| `encapsulation_melanization` | `( hemocyte_density: f64, parasite_surface_area: f64, phenoloxidase: f64, ) -> f64` |
| `acquired_immunity_buildup` | `(exposure_events: usize, max_immunity: f64, rate: f64) -> f64` |
| `maternal_antibody_decay` | `(initial_titer: f64, half_life_weeks: f64, age_weeks: f64) -> f64` |
| `concomitant_immunity` | `(adult_worms: f64, larval_killing_rate: f64, new_larvae: f64) -> f64` |
| `eosinophil_response` | `(parasite_burden: f64, il5_level: f64, eosinophil_base: f64) -> f64` |
| `granuloma_formation_rate` | `( antigen_deposition: f64, macrophage_activation: f64, fibrosis_rate: f64, ) -> f64` |
| `hygiene_hypothesis_index` | `( parasite_exposure: f64, allergy_risk_base: f64, protection_factor: f64, ) -> f64` |
| `parasite_virulence_tradeoff` | `(virulence: f64, beta_max: f64, v_half: f64) -> f64` |
| `optimal_virulence` | `(beta_max: f64, v_half: f64, mortality_background: f64) -> f64` |
| `immune_evasion_probability` | `(parasite_diversity: f64, immune_memory: f64) -> f64` |
| `worm_burden_distribution_mean` | `(epg: f64, fecundity: f64) -> f64` |
| `force_of_infection` | `( contact_rate: f64, environmental_contamination: f64, susceptibility: f64, ) -> f64` |
| `age_intensity_profile` | `(age: f64, peak_age: f64, max_intensity: f64, shape: f64) -> f64` |
| `superinfection_threshold` | `(r0_resident: f64, r0_challenger: f64) -> bool` |
| `aggregation_parameter` | `(mean_burden: f64, variance: f64) -> f64` |
| `drug_resistance_spread` | `( sensitive_freq: f64, resistant_fitness: f64, treatment_coverage: f64, ) -> f64` |
| `basic_reproduction_number_macroparasite` | `( beta: f64, lambda: f64, mu_host: f64, mu_parasite: f64, alpha: f64, ) -> f64` |
| `case_fatality_rate` | `(virulence: f64, host_resistance: f64) -> f64` |
| `parasite_clearance_rate` | `( immune_activity: f64, drug_efficacy: f64, natural_death: f64, ) -> f64` |
| `morbidity_intensity` | `(burden: f64, threshold: f64, severity_coeff: f64) -> f64` |
| `transmission_seasonality` | `(baseline_beta: f64, amplitude: f64, t: f64, period: f64) -> f64` |
| `mass_drug_administration_impact` | `(prevalence: f64, coverage: f64, efficacy: f64) -> f64` |
| `reinfection_rate` | `( force_of_infection: f64, waning_immunity: f64, time_since_treatment: f64, ) -> f64` |
| `pathogen_shedding_rate` | `(burden: f64, per_parasite_shed: f64, saturation: f64) -> f64` |
| `environmental_reservoir_decay` | `(contamination: f64, decay_rate: f64, input_rate: f64) -> f64` |
| `host_specificity_index` | `(hosts_used: f64, hosts_available: f64) -> f64` |
| `virulence_evolution_si` | `(beta: f64, alpha: f64, gamma: f64, mu: f64) -> f64` |

---

## pharmacology API (72 functions)

| Function | Signature |
|----------|-----------|
| `oral_bioavailability` | `( fraction_absorbed: f64, gut_wall_extraction: f64, hepatic_extraction: f64, ) -> f64` |
| `intestinal_permeability_papp` | `( amount_receiver: f64, area: f64, time: f64, donor_conc: f64, ) -> f64` |
| `dissolution_noyes_whitney` | `( diffusion_coeff: f64, surface_area: f64, cs: f64, c: f64, thickness: f64, volume: f64, ) -> f64` |
| `biopharmaceutics_classification` | `(solubility_high: bool, permeability_high: bool) -> u8` |
| `hepatic_clearance_well_stirred` | `(liver_blood_flow: f64, fu: f64, cl_int: f64) -> f64` |
| `renal_drug_clearance` | `(gfr: f64, fu: f64, secretion: f64, reabsorption_fraction: f64) -> f64` |
| `protein_binding` | `(ka: f64, protein_conc: f64) -> f64` |
| `apparent_volume_of_distribution` | `(dose: f64, plasma_concentration: f64) -> f64` |
| `compartment_distribution` | `(dose: f64, kel: f64, k12: f64, k21: f64, t: f64) -> f64` |
| `p_glycoprotein_efflux` | `(intracellular_conc: f64, pgp_activity: f64, km: f64) -> f64` |
| `drug_drug_interaction_auc_ratio` | `(inhibitor_conc: f64, ki: f64) -> f64` |
| `cyp_induction_fold` | `(inducer_conc: f64, ec50: f64, emax: f64) -> f64` |
| `competitive_displacement` | `(drug_a_bound: f64, drug_b_conc: f64, kb: f64) -> f64` |
| `synergy_bliss_independence` | `(effect_a: f64, effect_b: f64) -> f64` |
| `loewe_combination_index` | `( dose_a: f64, dose_a_alone: f64, dose_b: f64, dose_b_alone: f64, ) -> f64` |
| `isobologram_point` | `(dose_a: f64, ic50_a: f64, dose_b: f64, ic50_b: f64) -> f64` |
| `prodrug_activation` | `( prodrug_conc: f64, enzyme_activity: f64, km: f64, activation_fraction: f64, ) -> f64` |
| `drug_therapeutic_index` | `(td50: f64, ed50: f64) -> f64` |
| `loading_dose_calculation` | `( target_concentration: f64, volume_of_distribution: f64, bioavailability: f64, ) -> f64` |
| `maintenance_dose_calculation` | `( target_concentration: f64, clearance: f64, bioavailability: f64, dosing_interval: f64, ) -> f64` |
| `steady_state_accumulation` | `(dose: f64, half_life: f64, dosing_interval: f64) -> f64` |
| `emax_model` | `(e0: f64, emax: f64, c: f64, ec50: f64) -> f64` |
| `sigmoid_emax` | `(e0: f64, emax: f64, c: f64, ec50: f64, n: f64) -> f64` |
| `log_logistic` | `(c: f64, ec50: f64, slope: f64) -> f64` |
| `therapeutic_index` | `(td50: f64, ed50: f64) -> f64` |
| `dose_response_hill` | `(dose: f64, dmax: f64, ec50: f64, n: f64) -> f64` |
| `competitive_antagonism` | `(agonist: f64, ec50: f64, antagonist: f64, kb: f64) -> f64` |
| `schild_equation` | `(dose_ratio: f64, antagonist: f64) -> f64` |
| `receptor_occupancy` | `(l: f64, kd: f64) -> f64` |
| `clark_equation` | `(l: f64, kd: f64, emax: f64) -> f64` |
| `operational_model` | `(l: f64, kd: f64, tau: f64, n: f64, emax: f64) -> f64` |
| `imax_model` | `(i0: f64, imax: f64, c: f64, ic50: f64) -> f64` |
| `combination_index` | `(d1: f64, dx1: f64, d2: f64, dx2: f64) -> f64` |
| `non_competitive_antagonism` | `( agonist: f64, ec50: f64, antagonist: f64, kb: f64, emax: f64, ) -> f64` |
| `irreversible_antagonism` | `(agonist: f64, ec50: f64, fraction_remaining: f64, emax: f64) -> f64` |
| `allosteric_modulator` | `( agonist: f64, ec50: f64, modulator: f64, alpha: f64, beta: f64, km: f64, emax: f64, ) -> f64` |
| `patlak_plot_slope` | `(plasma_integral: f64, plasma_conc: f64, tissue_conc: f64) -> f64` |
| `two_state_receptor` | `(l: f64, kd_active: f64, kd_inactive: f64, l0: f64) -> f64` |
| `partial_agonist_effect` | `(l: f64, kd: f64, intrinsic_efficacy: f64, emax: f64) -> f64` |
| `inverse_agonist_effect` | `(e0: f64, l: f64, kd: f64, neg_efficacy: f64) -> f64` |
| `biased_agonism_ratio` | `(e1: f64, ec50_1: f64, e2: f64, ec50_2: f64) -> f64` |
| `pk_pd_effect_compartment` | `(ce: f64, emax: f64, ec50: f64, n: f64) -> f64` |
| `hysteresis_collapse_ke0` | `(plasma: f64, effect_prev: f64, ke0: f64, dt: f64) -> f64` |
| `tolerance_factor` | `(exposure_time: f64, tolerance_rate: f64) -> f64` |
| `one_compartment` | `(dose: f64, vd: f64, ke: f64, t: f64) -> f64` |
| `one_compartment_iv_infusion` | `(r0: f64, ke: f64, vd: f64, t: f64, t_inf: f64) -> f64` |
| `two_compartment` | `(a: f64, alpha: f64, b: f64, beta: f64, t: f64) -> f64` |
| `oral_one_compartment` | `(f_bio: f64, dose: f64, ka: f64, ke: f64, vd: f64, t: f64) -> f64` |
| `clearance` | `(ke: f64, vd: f64) -> f64` |
| `half_life` | `(ke: f64) -> f64` |
| `auc_iv_bolus` | `(dose: f64, cl: f64) -> f64` |
| `auc_trapezoidal` | `(times: &[f64], concentrations: &[f64]) -> f64` |
| `bioavailability` | `(auc_oral: f64, dose_oral: f64, auc_iv: f64, dose_iv: f64) -> f64` |
| `volume_of_distribution` | `(dose: f64, c0: f64) -> f64` |
| `steady_state_concentration` | `(dose: f64, cl: f64, tau: f64, f_bio: f64) -> f64` |
| `loading_dose` | `(css_target: f64, vd: f64, f_bio: f64) -> f64` |
| `maintenance_dose` | `(css_target: f64, cl: f64, tau: f64, f_bio: f64) -> f64` |
| `accumulation_factor` | `(ke: f64, tau: f64) -> f64` |
| `tmax_oral` | `(ka: f64, ke: f64) -> f64` |
| `cmax_oral` | `(f_bio: f64, dose: f64, ka: f64, ke: f64, vd: f64) -> f64` |
| `three_compartment` | `(a: f64, alpha: f64, b: f64, beta: f64, c: f64, gamma: f64, t: f64) -> f64` |
| `multiple_dose_superposition` | `( dose: f64, vd: f64, ke: f64, tau: f64, t: f64, n_doses: usize, ) -> f64` |
| `css_max` | `(dose: f64, vd: f64, ke: f64, tau: f64) -> f64` |
| `css_min` | `(dose: f64, vd: f64, ke: f64, tau: f64) -> f64` |
| `time_above_mic` | `(dose: f64, vd: f64, ke: f64, mic: f64) -> f64` |
| `hepatic_extraction_ratio` | `(cl_hepatic: f64, q_hepatic: f64) -> f64` |
| `well_stirred_model` | `(q_h: f64, fu: f64, cl_int: f64) -> f64` |
| `renal_clearance` | `(fraction_unbound: f64, gfr: f64) -> f64` |
| `auc_log_trapezoidal` | `(times: &[f64], concentrations: &[f64]) -> f64` |
| `mean_residence_time` | `(aumc: f64, auc: f64) -> f64` |
| `aumc_trapezoidal` | `(times: &[f64], concentrations: &[f64]) -> f64` |
| `flip_flop_kinetics` | `(ka: f64, ke: f64) -> bool` |

---

## phylogenetics API (46 functions)

| Function | Signature |
|----------|-----------|
| `aa_to_index` | `(aa: u8) -> Option<usize>` |
| `blosum62_score` | `(a: u8, b: u8) -> i8` |
| `needleman_wunsch` | `(seq_a: &[u8], seq_b: &[u8], gap_penalty: i32) -> (Vec<u8>, Vec<u8>, i32)` |
| `smith_waterman` | `(seq_a: &[u8], seq_b: &[u8], gap_penalty: i32) -> (Vec<u8>, Vec<u8>, i32)` |
| `alignment_identity` | `(align_a: &[u8], align_b: &[u8]) -> f64` |
| `affine_gap_needleman_wunsch` | `( seq_a: &[u8], seq_b: &[u8], gap_open: i32, gap_extend: i32, ) -> (Vec<u8>, Vec<u8>, i32)` |
| `multiple_alignment_score` | `(alignment: &[Vec<u8>]) -> i32` |
| `pairwise_distance` | `(align_a: &[u8], align_b: &[u8]) -> f64` |
| `jukes_cantor_distance` | `(p_distance: f64) -> f64` |
| `gap_fraction` | `(aligned: &[u8]) -> f64` |
| `hamming_distance` | `(a: &[u8], b: &[u8]) -> usize` |
| `p_distance` | `(a: &[u8], b: &[u8]) -> f64` |
| `jukes_cantor` | `(p: f64) -> f64` |
| `kimura_2p` | `(transitions: f64, transversions: f64, length: f64) -> f64` |
| `count_transitions_transversions` | `(a: &[u8], b: &[u8]) -> (usize, usize)` |
| `distance_matrix` | `(sequences: &[&[u8]]) -> Vec<Vec<f64>>` |
| `log_det_distance` | `(freq_matrix: &[[f64; 4]; 4]) -> f64` |
| `molecular_clock_rate` | `(substitutions: f64, divergence_time: f64) -> f64` |
| `strict_clock_branch_length` | `(rate: f64, time: f64) -> f64` |
| `relaxed_clock_lognormal` | `(mean_rate: f64, sigma: f64, branch_deviation: f64) -> f64` |
| `divergence_time_from_distance` | `(genetic_distance: f64, substitution_rate: f64) -> f64` |
| `jc_distance` | `(p_diff: f64) -> f64` |
| `kimura_2p_distance` | `(transitions: f64, transversions: f64, length: f64) -> f64` |
| `bayesian_clock_calibration` | `( prior_age: f64, prior_sd: f64, likelihood_age: f64, likelihood_sd: f64, ) -> (f64, f64)` |
| `rate_heterogeneity_gamma` | `(alpha: f64, n_categories: usize) -> Vec<f64>` |
| `local_clock_assignment` | `(branch_rates: &[f64], threshold: f64) -> Vec<usize>` |
| `gc_content` | `(seq: &[u8]) -> f64` |
| `complement` | `(base: u8) -> u8` |
| `reverse_complement` | `(seq: &[u8]) -> Vec<u8>` |
| `translate_codon` | `(codon: &[u8]) -> u8` |
| `translate` | `(dna: &[u8]) -> Vec<u8>` |
| `transcribe` | `(dna: &[u8]) -> Vec<u8>` |
| `nucleotide_frequencies` | `(seq: &[u8]) -> [f64; 4]` |
| `molecular_weight_dna` | `(seq: &[u8]) -> f64` |
| `melting_temperature_basic` | `(seq: &[u8]) -> f64` |
| `upgma` | `(dist_matrix: &[Vec<f64>]) -> Vec<(usize, usize, f64)>` |
| `neighbor_joining` | `(dist_matrix: &[Vec<f64>]) -> Vec<(usize, usize, f64, f64)>` |
| `wpgma` | `(dist_matrix: &[Vec<f64>]) -> Vec<(usize, usize, f64)>` |
| `molecular_clock_test` | `(branch_lengths: &[f64], expected: &[f64]) -> f64` |
| `robinson_foulds` | `(splits_a: &[Vec<bool>], splits_b: &[Vec<bool>]) -> usize` |
| `sackin_index` | `(branch_depths: &[usize]) -> f64` |
| `colless_index` | `(left_sizes: &[usize], right_sizes: &[usize]) -> f64` |
| `branch_length_total` | `(branch_lengths: &[f64]) -> f64` |
| `patristic_distance` | `(tree_distances: &[Vec<f64>], i: usize, j: usize) -> f64` |
| `parsimony_score` | `(sequences: &[&[u8]], tree_topology: &[(usize, usize)]) -> usize` |
| `gamma_rate_categories` | `(alpha: f64, n_categories: usize) -> Vec<f64>` |

---

## physiology API (47 functions)

| Function | Signature |
|----------|-----------|
| `frank_starling_mechanism` | `( end_diastolic_volume: f64, contractility: f64, max_stroke_volume: f64, ) -> f64` |
| `stroke_volume_cardiac_output` | `(heart_rate: f64, stroke_volume: f64) -> f64` |
| `ejection_fraction` | `(stroke_volume: f64, end_diastolic_volume: f64) -> f64` |
| `map_calculation` | `(systolic: f64, diastolic: f64) -> f64` |
| `systemic_vascular_resistance` | `(map: f64, cvp: f64, cardiac_output: f64) -> f64` |
| `myocardial_oxygen_consumption` | `(heart_rate: f64, systolic_bp: f64) -> f64` |
| `windkessel_pressure` | `( cardiac_output: f64, resistance: f64, compliance: f64, t: f64, heart_rate: f64, ) -> f64` |
| `coronary_flow_reserve` | `(hyperemic_flow: f64, resting_flow: f64) -> f64` |
| `qt_correction_bazett` | `(qt_ms: f64, rr_ms: f64) -> f64` |
| `cardiac_work` | `(stroke_volume_ml: f64, mean_pressure_mmhg: f64) -> f64` |
| `preload_recruitable_stroke_work` | `(stroke_work: f64, edv: f64, v0: f64) -> f64` |
| `poiseuille_flow` | `(radius: f64, length: f64, pressure_drop: f64, viscosity: f64) -> f64` |
| `wall_shear_stress` | `(viscosity: f64, flow_rate: f64, radius: f64) -> f64` |
| `mean_arterial_pressure` | `(systolic: f64, diastolic: f64) -> f64` |
| `cardiac_output` | `(stroke_volume: f64, heart_rate: f64) -> f64` |
| `total_peripheral_resistance` | `(map: f64, cvp: f64, cardiac_output: f64) -> f64` |
| `frank_starling` | `(preload: f64, k: f64, max_force: f64) -> f64` |
| `pulse_wave_velocity` | `(elasticity: f64, wall_thickness: f64, radius: f64, density: f64) -> f64` |
| `windkessel_2` | `(flow: f64, pressure: f64, resistance: f64, compliance: f64) -> f64` |
| `glomerular_filtration_rate` | `(kf: f64, p_gc: f64, p_bs: f64, pi_gc: f64) -> f64` |
| `creatinine_clearance` | `(urine_cr: f64, urine_volume: f64, plasma_cr: f64) -> f64` |
| `fractional_excretion` | `(urine_x: f64, plasma_cr: f64, plasma_x: f64, urine_cr: f64) -> f64` |
| `free_water_clearance` | `(urine_volume: f64, urine_osm: f64, plasma_osm: f64) -> f64` |
| `tubular_reabsorption_rate` | `(filtered_load: f64, excretion_rate: f64) -> f64` |
| `cockcroft_gault` | `(age: f64, weight: f64, serum_cr: f64, is_female: bool) -> f64` |
| `mdrd_gfr` | `(serum_cr: f64, age: f64, is_female: bool, is_black: bool) -> f64` |
| `tubuloglomerular_feedback` | `( nacl_macula: f64, nacl_target: f64, sensitivity: f64, gfr_baseline: f64, ) -> f64` |
| `urine_concentration_ratio` | `(urine_osm: f64, plasma_osm: f64) -> f64` |
| `anion_gap` | `(sodium: f64, chloride: f64, bicarbonate: f64) -> f64` |
| `tidal_volume` | `(respiratory_rate: f64, minute_ventilation: f64) -> f64` |
| `alveolar_ventilation` | `(tidal_volume: f64, dead_space: f64, rate: f64) -> f64` |
| `alveolar_gas_equation` | `(fio2: f64, p_atm: f64, p_h2o: f64, paco2: f64, rq: f64) -> f64` |
| `airway_resistance` | `(pressure_drop: f64, flow: f64) -> f64` |
| `lung_compliance` | `(volume_change: f64, pressure_change: f64) -> f64` |
| `oxygen_content` | `(hb: f64, sao2: f64, pao2: f64) -> f64` |
| `oxygen_delivery` | `(cardiac_output: f64, cao2: f64) -> f64` |
| `fick_oxygen_consumption` | `(cardiac_output: f64, cao2: f64, cvo2: f64) -> f64` |
| `heat_balance` | `( metabolic_rate: f64, work: f64, radiation: f64, convection: f64, evaporation: f64, ) -> f64` |
| `newton_cooling` | `(body_temp: f64, ambient_temp: f64, h: f64, surface_area: f64) -> f64` |
| `evaporative_heat_loss` | `(sweat_rate: f64, latent_heat: f64) -> f64` |
| `core_temperature_regulation` | `( set_point: f64, core_temp: f64, gain_shiver: f64, gain_sweat: f64, ) -> (f64, f64)` |
| `wind_chill_index` | `(air_temp: f64, wind_speed_kmh: f64) -> f64` |
| `heat_index` | `(temperature_f: f64, relative_humidity: f64) -> f64` |
| `body_surface_area_dubois` | `(weight_kg: f64, height_cm: f64) -> f64` |
| `counter_current_heat_exchange` | `( arterial_temp: f64, venous_temp_return: f64, efficiency: f64, ) -> f64` |
| `brown_adipose_tissue_thermogenesis` | `( bat_mass: f64, ucp1_activity: f64, substrate_availability: f64, ) -> f64` |
| `metabolic_rate_q10` | `(rate_ref: f64, temp: f64, temp_ref: f64, q10: f64) -> f64` |

---

## plant_biology API (67 functions)

| Function | Signature |
|----------|-----------|
| `competitive_exclusion_tilman` | `(r_star_a: f64, r_star_b: f64) -> &'static str` |
| `allelopathy_effect` | `(allelochemical_conc: f64, ic50: f64, max_inhibition: f64) -> f64` |
| `light_competition_beer_lambert` | `(light_above: f64, lai: f64, extinction_coeff: f64) -> f64` |
| `canopy_lai` | `(leaf_area: f64, ground_area: f64) -> f64` |
| `sla` | `(leaf_area: f64, leaf_dry_mass: f64) -> f64` |
| `plant_defense_investment` | `(growth_rate: f64, defense_allocation: f64) -> f64` |
| `herbivory_damage` | `( herbivore_density: f64, feeding_rate: f64, plant_biomass: f64, defense_level: f64, ) -> f64` |
| `seed_dispersal_kernel` | `(distance: f64, mean_dispersal: f64) -> f64` |
| `pollination_success` | `(pollinator_visits: f64, pollen_per_visit: f64, ovule_count: f64) -> f64` |
| `nitrogen_fixation_symbiotic` | `( nodule_mass: f64, nitrogenase_activity: f64, oxygen_limitation: f64, ) -> f64` |
| `root_growth_logistic` | `(length: f64, max_length: f64, rate: f64, dt: f64) -> f64` |
| `auxin_gradient` | `(source_concentration: f64, distance: f64, diffusion: f64, decay: f64) -> f64` |
| `phototropism_bending_rate` | `(light_differential: f64, sensitivity: f64) -> f64` |
| `gravitropism_response` | `(angle: f64, sensitivity: f64, dt: f64) -> f64` |
| `leaf_area_index` | `(total_leaf_area: f64, ground_area: f64) -> f64` |
| `beer_lambert_canopy` | `(light_above: f64, k: f64, lai: f64) -> f64` |
| `thermal_time` | `(daily_mean_temp: f64, base_temp: f64) -> f64` |
| `water_potential` | `(osmotic: f64, pressure: f64, gravitational: f64) -> f64` |
| `xylem_flow_rate` | `(pressure_gradient: f64, conductivity: f64, cross_section: f64) -> f64` |
| `phloem_transport_munch` | `(source_pressure: f64, sink_pressure: f64, resistance: f64) -> f64` |
| `allometric_biomass` | `(diameter: f64, a: f64, b: f64) -> f64` |
| `specific_leaf_area` | `(leaf_area: f64, leaf_dry_mass: f64) -> f64` |
| `relative_growth_rate` | `(biomass_initial: f64, biomass_final: f64, time: f64) -> f64` |
| `net_assimilation_rate` | `(biomass_change: f64, leaf_area_avg: f64, time: f64) -> f64` |
| `phytochrome_response` | `(red: f64, far_red: f64) -> f64` |
| `vernalization_progress` | `(temp: f64, optimal_temp: f64, range: f64, dt: f64) -> f64` |
| `photoperiod_response` | `(day_length: f64, critical_length: f64, sensitivity: f64) -> f64` |
| `root_shoot_ratio` | `(root_biomass: f64, shoot_biomass: f64) -> f64` |
| `canopy_gap_fraction` | `(lai: f64, k: f64) -> f64` |
| `stem_taper` | `(diameter_base: f64, height_fraction: f64, taper_exponent: f64) -> f64` |
| `cavitation_vulnerability` | `(pressure: f64, p50: f64, slope: f64) -> f64` |
| `turgor_pressure` | `(osmotic_potential: f64, water_potential: f64) -> f64` |
| `gibberellin_stem_elongation` | `(ga_concentration: f64, max_rate: f64, km: f64) -> f64` |
| `senescence_chlorophyll_loss` | `(chl0: f64, degradation_rate: f64, t: f64) -> f64` |
| `frost_hardiness` | `(temp: f64, lt50: f64, slope: f64) -> f64` |
| `farquhar_c3` | `( vcmax: f64, ci: f64, gamma_star: f64, kc: f64, ko: f64, o: f64, j: f64, rd: f64, ) -> f64` |
| `light_response_curve` | `(phi: f64, ppfd: f64, amax: f64, theta: f64, rd: f64) -> f64` |
| `transpiration_rate` | `(stomatal_conductance: f64, vpd: f64) -> f64` |
| `ball_berry_conductance` | `(a_n: f64, cs: f64, rh: f64, g0: f64, g1: f64) -> f64` |
| `water_use_efficiency` | `(a_n: f64, transpiration: f64) -> f64` |
| `penman_monteith` | `( net_radiation: f64, soil_heat: f64, vpd: f64, ga: f64, gs: f64, delta: f64, gamma: f64, rho: f64, cp: f64, ) -> f64` |
| `rubisco_specificity` | `(vcmax: f64, kc: f64, vomax: f64, ko: f64) -> f64` |
| `photorespiration_rate` | `(vomax: f64, o: f64, ko: f64, ci: f64, kc: f64) -> f64` |
| `electron_transport_rate` | `( ppfd: f64, absorptance: f64, fraction_psii: f64, phi_psii: f64, ) -> f64` |
| `stomatal_optimization` | `(vpd: f64, ca: f64, lambda_wue: f64, g1: f64) -> f64` |
| `c4_photosynthesis` | `( vpmax: f64, ci: f64, kp: f64, vcmax: f64, ko: f64, kc: f64, o: f64, rd: f64, ) -> f64` |
| `cam_malic_acid_storage` | `( co2_fixed_night: f64, vacuole_capacity: f64, current_malate: f64, ) -> f64` |
| `cam_daytime_decarboxylation` | `(malate: f64, decarboxylation_rate: f64) -> f64` |
| `chlorophyll_fluorescence_fv_fm` | `(f0: f64, fm: f64) -> f64` |
| `non_photochemical_quenching` | `(fm: f64, fm_prime: f64) -> f64` |
| `photochemical_quenching` | `(fs: f64, f0_prime: f64, fm_prime: f64) -> f64` |
| `quantum_yield_psii` | `(phi_psii: f64, ppfd: f64) -> f64` |
| `co2_compensation_point` | `( gamma_star: f64, rd: f64, vcmax: f64, kc: f64, ko: f64, o: f64, ) -> f64` |
| `mesophyll_conductance` | `(a_n: f64, ci: f64, cc: f64) -> f64` |
| `light_use_efficiency` | `(gpp: f64, apar: f64) -> f64` |
| `vcmax_temperature_response` | `(vcmax25: f64, ha: f64, temp_k: f64) -> f64` |
| `jmax_temperature_peaked` | `(jmax25: f64, ha: f64, hd: f64, ds: f64, temp_k: f64) -> f64` |
| `xylem_flow_hagen_poiseuille` | `( radius: f64, pressure_gradient: f64, viscosity: f64, length: f64, ) -> f64` |
| `leaf_transpiration_rate` | `(stomatal_conductance: f64, vpd: f64) -> f64` |
| `cohesion_tension_water_potential` | `( osmotic: f64, pressure: f64, gravity: f64, matric: f64, ) -> f64` |
| `phloem_munch_flow` | `(turgor_source: f64, turgor_sink: f64, resistance: f64) -> f64` |
| `root_water_uptake` | `( soil_water_potential: f64, root_water_potential: f64, root_conductance: f64, ) -> f64` |
| `xylem_cavitation_vulnerability` | `(water_potential: f64, p50: f64, slope: f64) -> f64` |
| `stomatal_conductance_ball_berry` | `( assimilation: f64, humidity: f64, co2_surface: f64, g0: f64, g1: f64, ) -> f64` |
| `sugar_loading_rate` | `(sucrose_conc: f64, vmax: f64, km: f64) -> f64` |
| `root_hydraulic_conductivity` | `( flow_rate: f64, root_surface_area: f64, pressure_difference: f64, ) -> f64` |
| `sap_flow_heat_pulse` | `( thermal_diffusivity: f64, heat_pulse_distance: f64, time_to_max: f64, ) -> f64` |

---

## population API (49 functions)

| Function | Signature |
|----------|-----------|
| `leslie_matrix_multiply` | `(matrix: &[Vec<f64>], population: &[f64]) -> Vec<f64>` |
| `stable_age_distribution` | `(fecundities: &[f64], survivals: &[f64]) -> Vec<f64>` |
| `cohort_generation_time` | `(age_fecundity: &[(f64, f64)], lambda: f64) -> f64` |
| `reproductive_value` | `(lx: &[f64], mx: &[f64], lambda: f64) -> Vec<f64>` |
| `euler_lotka_lambda` | `(lx: &[f64], mx: &[f64]) -> f64` |
| `sensitivity_element` | `(v_i: f64, w_j: f64, vw_dot: f64) -> f64` |
| `elasticity_element` | `(sensitivity: f64, a_ij: f64, lambda: f64) -> f64` |
| `sir_model` | `(s: f64, i: f64, r: f64, beta: f64, gamma: f64) -> (f64, f64, f64)` |
| `sir_solve` | `( s0: f64, i0: f64, r0: f64, beta: f64, gamma: f64, dt: f64, steps: usize, ) -> Vec<(f64, f64, f64)>` |
| `seir_model` | `( s: f64, e: f64, i: f64, r: f64, beta: f64, sigma: f64, gamma: f64, ) -> (f64, f64, f64, f64)` |
| `seir_solve` | `( s0: f64, e0: f64, i0: f64, r0: f64, beta: f64, sigma: f64, gamma: f64, dt: f64, steps: usize, ) -> Vec<(f64, f64, f64, f64)>` |
| `sis_model` | `(s: f64, i: f64, beta: f64, gamma: f64) -> (f64, f64)` |
| `sirs_model` | `(s: f64, i: f64, r: f64, beta: f64, gamma: f64, xi: f64) -> (f64, f64, f64)` |
| `basic_reproduction_number` | `(beta: f64, gamma: f64) -> f64` |
| `herd_immunity_threshold` | `(r0: f64) -> f64` |
| `final_size_equation` | `(r0: f64, tolerance: f64, max_iter: usize) -> f64` |
| `generation_time` | `(incubation: f64, infectious_period: f64) -> f64` |
| `logistic_growth` | `(n: f64, r: f64, k: f64) -> f64` |
| `logistic_solve` | `(n0: f64, r: f64, k: f64, dt: f64, steps: usize) -> Vec<f64>` |
| `exponential_growth` | `(n0: f64, r: f64, t: f64) -> f64` |
| `gompertz` | `(n: f64, a: f64, b: f64, k: f64) -> f64` |
| `allee_effect` | `(n: f64, r: f64, k: f64, a: f64) -> f64` |
| `beverton_holt` | `(n: f64, r: f64, k: f64) -> f64` |
| `ricker` | `(n: f64, r: f64, k: f64) -> f64` |
| `doubling_time` | `(r: f64) -> f64` |
| `von_bertalanffy` | `(l_inf: f64, k: f64, t: f64, t0: f64) -> f64` |
| `theta_logistic` | `(n: f64, r: f64, k: f64, theta: f64) -> f64` |
| `moran_process_fixation` | `(n: usize, r: f64) -> f64` |
| `lotka_volterra` | `( prey: f64, pred: f64, alpha: f64, beta: f64, delta: f64, gamma: f64, ) -> (f64, f64)` |
| `lotka_volterra_solve` | `( prey0: f64, pred0: f64, alpha: f64, beta: f64, delta: f64, gamma: f64, dt: f64, steps: usize, ) -> Vec<(f64, f64)>` |
| `competitive_lotka_volterra` | `( n1: f64, n2: f64, r1: f64, r2: f64, k1: f64, k2: f64, a12: f64, a21: f64, ) -> (f64, f64)` |
| `holling_type_ii` | `(prey: f64, attack_rate: f64, handling_time: f64) -> f64` |
| `holling_type_iii` | `(prey: f64, attack_rate: f64, handling_time: f64) -> f64` |
| `rosenzweig_macarthur` | `( prey: f64, pred: f64, r: f64, k: f64, a: f64, h: f64, e: f64, d: f64, ) -> (f64, f64)` |
| `beddington_deangelis` | `(prey: f64, pred: f64, a: f64, b: f64, c: f64) -> f64` |
| `ratio_dependent` | `(prey: f64, pred: f64, a: f64, h: f64) -> f64` |
| `intraguild_predation` | `( prey: f64, meso: f64, top: f64, r: f64, k: f64, a_mp: f64, a_tp: f64, a_tm: f64, e_mp: f64, e_tp: f64, e_tm: f64, d_m: f64, d_t: f64, ) -> (f64, f64, f64)` |
| `apparent_competition` | `( prey1: f64, prey2: f64, pred: f64, a1: f64, a2: f64, r1: f64, r2: f64, k1: f64, k2: f64, e: f64, d: f64, ) -> (f64, f64, f64)` |
| `diffusion_dispersal` | `(density: f64, diffusion_coeff: f64, gradient: f64) -> f64` |
| `reaction_diffusion_fisher` | `(n: f64, r: f64, k: f64, d: f64, laplacian: f64) -> f64` |
| `fisher_wave_speed` | `(r: f64, d: f64) -> f64` |
| `range_expansion_rate` | `(diffusion: f64, growth_rate: f64) -> f64` |
| `stepping_stone_migration` | `( source_density: f64, target_density: f64, migration_rate: f64, ) -> f64` |
| `isolation_by_distance` | `(fst: f64) -> f64` |
| `landscape_resistance` | `(distance: f64, resistance_cost: f64) -> f64` |
| `gravity_model_migration` | `(pop_source: f64, pop_dest: f64, distance: f64, alpha: f64) -> f64` |
| `corridor_effectiveness` | `( width: f64, length: f64, habitat_quality: f64, species_mobility: f64, ) -> f64` |
| `allee_effect_spatial` | `(density: f64, allee_threshold: f64, r: f64, k: f64) -> f64` |
| `kernel_based_dispersal` | `(distance: f64, alpha: f64, shape: f64) -> f64` |

---

## proteomics API (31 functions)

| Function | Signature |
|----------|-----------|
| `b_ion_masses` | `(sequence: &str) -> Vec<f64>` |
| `y_ion_masses` | `(sequence: &str) -> Vec<f64>` |
| `mz_ratio` | `(mass: f64, charge: usize) -> f64` |
| `mass_from_mz` | `(mz: f64, charge: usize) -> f64` |
| `mass_accuracy_ppm` | `(theoretical: f64, observed: f64) -> f64` |
| `isotope_pattern_averagine` | `(mass: f64, n_peaks: usize) -> Vec<f64>` |
| `ppi_degree` | `(adjacency_row: &[bool]) -> usize` |
| `clustering_coefficient` | `(neighbors_connected: usize, degree: usize) -> f64` |
| `betweenness_centrality_approx` | `( shortest_paths_through: f64, total_shortest_paths: f64, ) -> f64` |
| `network_density` | `(edges: usize, nodes: usize) -> f64` |
| `scale_free_exponent` | `(degree_distribution: &[f64]) -> f64` |
| `hub_score` | `(degree: usize, max_degree: usize) -> f64` |
| `edge_betweenness` | `(flow_through_edge: f64, total_flow: f64) -> f64` |
| `protein_complex_stoichiometry` | `(abundances: &[f64]) -> Vec<f64>` |
| `functional_enrichment_odds_ratio` | `( hits_in_set: usize, set_size: usize, hits_total: usize, genome_size: usize, ) -> f64` |
| `guilt_by_association_score` | `(neighbor_annotations: &[bool]) -> f64` |
| `peptide_molecular_weight` | `(sequence: &str) -> f64` |
| `isoelectric_point` | `(sequence: &str) -> f64` |
| `gravy_index` | `(sequence: &str) -> f64` |
| `extinction_coefficient_280` | `(n_trp: usize, n_tyr: usize, n_cystine: usize) -> f64` |
| `spectral_count_nsaf` | `(spectral_count: f64, protein_length: f64, total_nsaf: f64) -> f64` |
| `ibaq` | `(total_intensity: f64, num_observable_peptides: usize) -> f64` |
| `lfq_ratio` | `(intensity_sample: f64, intensity_reference: f64) -> f64` |
| `tmt_reporter_ratio` | `(reporter_intensity: f64, reference_channel: f64) -> f64` |
| `silac_ratio` | `(heavy: f64, light: f64) -> f64` |
| `protein_fdr` | `(decoy_hits: f64, target_hits: f64) -> f64` |
| `mascot_ion_score` | `(observed: f64, expected: f64) -> f64` |
| `em_pai` | `(observed_peptides: usize, observable_peptides: usize) -> f64` |
| `protein_coverage` | `(identified_residues: usize, total_residues: usize) -> f64` |
| `xcorr_normalized` | `(xcorr: f64, peptide_length: usize) -> f64` |
| `missed_cleavage_rate` | `(missed: usize, total_peptides: usize) -> f64` |

---

## radiobiology API (57 functions)

| Function | Signature |
|----------|-----------|
| `dna_strand_break_probability` | `(dose: f64, target_size: f64, repair_efficiency: f64) -> f64` |
| `base_excision_repair_rate` | `(damage_sites: f64, enzyme_concentration: f64, km: f64) -> f64` |
| `misrepair_probability` | `(damage_density: f64, complexity_factor: f64) -> f64` |
| `chromosome_aberration_yield` | `(dose: f64, alpha: f64, beta: f64) -> f64` |
| `lethal_aberration_fraction` | `(aberrations: f64) -> f64` |
| `mutation_frequency` | `(dose: f64, spontaneous_rate: f64, induced_rate_per_gy: f64) -> f64` |
| `double_strand_break_yield` | `(dose: f64, let_factor: f64) -> f64` |
| `nhej_repair_kinetics` | `( breaks: f64, fast_rate: f64, slow_rate: f64, fast_fraction: f64, t: f64, ) -> f64` |
| `homologous_recombination_probability` | `( cell_cycle_s_g2_fraction: f64, sister_chromatid_available: bool, ) -> f64` |
| `clustered_damage_probability` | `(dose: f64, let_val: f64, target_radius: f64) -> f64` |
| `single_strand_break_yield` | `(dose: f64) -> f64` |
| `oxidative_base_damage_yield` | `(dose: f64, oxygen_concentration: f64) -> f64` |
| `dna_damage_complexity_score` | `(ssb: f64, dsb: f64, base_damage: f64) -> f64` |
| `foci_formation_kinetics` | `(dsb: f64, recruitment_rate: f64, t: f64) -> f64` |
| `foci_resolution_kinetics` | `(foci_max: f64, repair_rate: f64, t: f64) -> f64` |
| `micronucleus_formation` | `(dose: f64, alpha_mn: f64, beta_mn: f64) -> f64` |
| `comet_tail_moment` | `(tail_length: f64, tail_dna_fraction: f64) -> f64` |
| `gamma_h2ax_signal` | `(dsb: f64, spreading_factor: f64, background: f64) -> f64` |
| `repair_pathway_choice` | `(dsb: f64, cell_cycle_phase: f64, brca_status: f64) -> (f64, f64)` |
| `linear_quadratic_survival` | `(dose: f64, alpha: f64, beta: f64) -> f64` |
| `biologically_effective_dose` | `(n: f64, d: f64, alpha_beta: f64) -> f64` |
| `equivalent_dose_2gy` | `(bed: f64, alpha_beta: f64) -> f64` |
| `tcp` | `(n_cells: f64, survival_fraction: f64) -> f64` |
| `ntcp_lyman` | `(dose: f64, td50: f64, m: f64) -> f64` |
| `oxygen_enhancement_ratio` | `(dose_hypoxic: f64, dose_oxic: f64) -> f64` |
| `dna_dsb_yield` | `(dose: f64, yield_per_gray: f64) -> f64` |
| `repair_kinetics` | `(dsb0: f64, repair_rate: f64, t: f64) -> f64` |
| `fractionation_survival` | `( n_fractions: usize, dose_per_fraction: f64, alpha: f64, beta: f64, repair_factor: f64, ) -> f64` |
| `relative_biological_effectiveness` | `(dose_ref: f64, dose_test: f64) -> f64` |
| `let_to_rbe` | `(let_val: f64, rbe_max: f64, let_half: f64) -> f64` |
| `protraction_factor` | `(dose_rate: f64, repair_half_time: f64, total_time: f64) -> f64` |
| `bystander_effect` | `(dose: f64, max_effect: f64, dose_half: f64) -> f64` |
| `adaptive_response` | `( priming_dose: f64, challenge_dose: f64, alpha: f64, beta: f64, reduction_factor: f64, ) -> f64` |
| `low_dose_hypersensitivity` | `(dose: f64, alpha_r: f64, alpha_s: f64, dc: f64, beta: f64) -> f64` |
| `tumor_growth_delay` | `(dose: f64, alpha: f64, beta: f64, doubling_time: f64) -> f64` |
| `complication_free_cure` | `(tcp_val: f64, ntcp_val: f64) -> f64` |
| `isoeffect_dose` | `(n1: f64, d1: f64, alpha_beta: f64, n2: f64) -> f64` |
| `bed_biologically_effective_dose` | `(n: f64, d: f64, alpha_beta: f64) -> f64` |
| `eqd2` | `(n: f64, d: f64, alpha_beta: f64) -> f64` |
| `tumor_control_probability` | `(n_clonogens: f64, surviving_fraction: f64) -> f64` |
| `normal_tissue_complication_probability` | `(dose: f64, td50: f64, gamma50: f64) -> f64` |
| `incomplete_repair_factor` | `(delta_t: f64, repair_half_time: f64) -> f64` |
| `repopulation_dose_equivalent` | `( doubling_time: f64, treatment_duration: f64, alpha: f64, ) -> f64` |
| `lq_with_repopulation` | `( alpha: f64, beta: f64, dose: f64, n_fractions: f64, treatment_days: f64, tp: f64, tk: f64, ) -> f64` |
| `therapeutic_ratio` | `(tcp: f64, ntcp: f64) -> f64` |
| `fraction_size_optimization` | `(alpha_beta_tumor: f64, alpha_beta_normal: f64) -> f64` |
| `hyperfractionation_advantage` | `(d_conventional: f64, d_hyper: f64, alpha_beta: f64) -> f64` |
| `radiation_shielding_half_value` | `(initial_intensity: f64, hvl: f64, thickness: f64) -> f64` |
| `shielding_tenth_value` | `(initial_intensity: f64, tvl: f64, thickness: f64) -> f64` |
| `mass_attenuation` | `(intensity: f64, mu_over_rho: f64, density: f64, thickness: f64) -> f64` |
| `buildup_factor` | `(beam_layers: f64, mu: f64, thickness: f64) -> f64` |
| `concrete_shielding_thickness` | `(dose_rate: f64, dose_limit: f64, hvl: f64) -> f64` |
| `lead_equivalent_thickness` | `(mu_material: f64, mu_lead: f64, thickness_material: f64) -> f64` |
| `inverse_square_distance` | `(dose_at_d1: f64, d1: f64, d2: f64) -> f64` |
| `occupancy_factor_dose` | `(dose_unshielded: f64, occupancy: f64, use_factor: f64) -> f64` |
| `neutron_shielding_hydrogen` | `(thickness_cm: f64, cross_section: f64, density_h: f64) -> f64` |
| `annual_dose_limit_check` | `(dose_received: f64, dose_limit: f64) -> f64` |

---

## reproduction API (62 functions)

| Function | Signature |
|----------|-----------|
| `cleavage_timing` | `(stage: u32, base_interval: f64, temperature_factor: f64) -> f64` |
| `blastocyst_cell_count` | `(initial_cells: f64, division_rate: f64, t: f64) -> f64` |
| `morphogen_gradient_embryo` | `(source: f64, diffusion: f64, degradation: f64, x: f64) -> f64` |
| `gastrulation_cell_migration` | `( chemotactic_sensitivity: f64, gradient: f64, random_motility: f64, ) -> f64` |
| `somitogenesis_clock` | `(frequency: f64, wavefront_speed: f64, position: f64, t: f64) -> f64` |
| `fetal_weight_hadlock` | `(gestational_age_weeks: f64) -> f64` |
| `placental_transfer_rate` | `( maternal_conc: f64, fetal_conc: f64, permeability: f64, surface_area: f64, ) -> f64` |
| `crown_rump_length` | `(gestational_age_weeks: f64) -> f64` |
| `biparietal_diameter` | `(gestational_age_weeks: f64) -> f64` |
| `amniotic_fluid_index` | `(quadrants: &[f64; 4]) -> f64` |
| `neural_tube_closure_progress` | `(t: f64, rate: f64, max_closure: f64) -> f64` |
| `organogenesis_differentiation_rate` | `( morphogen_conc: f64, threshold: f64, hill_coefficient: f64, ) -> f64` |
| `turing_activator_inhibitor` | `( activator: f64, inhibitor: f64, rho_a: f64, rho_i: f64, mu_a: f64, mu_i: f64, kappa: f64, ) -> (f64, f64)` |
| `fetal_lung_maturity_ls_ratio` | `(lecithin: f64, sphingomyelin: f64) -> f64` |
| `apgar_component` | `( heart_rate: f64, respiration: f64, muscle_tone: f64, reflex: f64, color: f64, ) -> f64` |
| `fetal_heart_rate_baseline` | `(gestational_age_weeks: f64) -> f64` |
| `umbilical_artery_pi` | `(systolic: f64, diastolic: f64, mean: f64) -> f64` |
| `placental_oxygen_delivery` | `( blood_flow: f64, hemoglobin: f64, saturation: f64, o2_binding_capacity: f64, ) -> f64` |
| `trophoblast_invasion_depth` | `( migration_rate: f64, protease_activity: f64, resistance: f64, t: f64, ) -> f64` |
| `gestational_sac_diameter` | `(gestational_age_days: f64) -> f64` |
| `yolk_sac_regression` | `(initial_size: f64, regression_rate: f64, t: f64) -> f64` |
| `limb_bud_outgrowth` | `(fgf_conc: f64, shh_conc: f64, growth_rate: f64, t: f64) -> f64` |
| `cell_fate_probability` | `(signal_strength: f64, noise: f64, threshold: f64) -> f64` |
| `ovarian_cycle_hormone` | `( t: f64, amplitude: f64, peak_day: f64, width: f64, baseline: f64, ) -> f64` |
| `follicle_growth` | `(diameter: f64, fsh: f64, growth_rate: f64, max_diameter: f64) -> f64` |
| `sperm_motility_fraction` | `(velocity: f64, threshold: f64, concentration: f64) -> f64` |
| `sperm_capacitation_rate` | `(t: f64, half_time: f64) -> f64` |
| `fertilization_probability` | `(sperm_count: f64, half_max: f64) -> f64` |
| `implantation_window` | `(progesterone: f64, threshold: f64, estrogen_ratio: f64) -> bool` |
| `hcg_doubling` | `(initial: f64, doubling_time: f64, t: f64) -> f64` |
| `lh_surge_model` | `(t: f64, t_peak: f64, amplitude: f64, rise_rate: f64, fall_rate: f64) -> f64` |
| `estradiol_follicular` | `(follicle_diameter: f64, num_follicles: f64, sensitivity: f64) -> f64` |
| `progesterone_luteal` | `( t_post_ovulation: f64, peak: f64, rise_rate: f64, fall_rate: f64, ) -> f64` |
| `oocyte_quality_age` | `( base_quality: f64, age: f64, decline_start: f64, decline_rate: f64, ) -> f64` |
| `antral_follicle_count` | `(age: f64, initial_pool: f64, depletion_rate: f64) -> f64` |
| `anti_mullerian_hormone` | `(follicle_count: f64, sensitivity: f64) -> f64` |
| `ivf_success_rate` | `(age: f64, embryo_quality: f64, endometrial_thickness: f64) -> f64` |
| `menstrual_cycle_length` | `(lh_peak_day: f64, luteal_phase_length: f64) -> f64` |
| `sperm_concentration_fertility` | `(concentration: f64, motility: f64, morphology: f64) -> f64` |
| `cumulative_pregnancy_rate` | `(monthly_fecundability: f64, months: u32) -> f64` |
| `zona_pellucida_binding` | `(receptors: f64, sperm_conc: f64, kd: f64) -> f64` |
| `acrosome_reaction_rate` | `(capacitated_fraction: f64, zona_signal: f64, k: f64) -> f64` |
| `endometrial_receptivity` | `(p4: f64, lif: f64, integrin: f64, threshold_p4: f64) -> f64` |
| `twin_probability_dizygotic` | `(age: f64, fsh_level: f64) -> f64` |
| `menstrual_cycle_hormone` | `(day: f64, hormone: &str) -> f64` |
| `ovulation_probability` | `(lh_surge: f64, follicle_maturity: f64, threshold: f64) -> f64` |
| `endometrial_thickness` | `(day: f64, estrogen: f64) -> f64` |
| `fertility_window` | `(cycle_day: f64, cycle_length: f64) -> f64` |
| `hcg_doubling_time` | `(initial_hcg: f64, days: f64, doubling_time: f64) -> f64` |
| `implantation_probability` | `( embryo_quality: f64, endometrial_receptivity: f64, age_factor: f64, ) -> f64` |
| `spermatogenesis_duration_days` | `() -> f64` |
| `sperm_motility_score` | `(progressive: f64, non_progressive: f64, immotile: f64) -> f64` |
| `testosterone_circadian` | `(hour: f64, peak_level: f64, trough_level: f64) -> f64` |
| `ivf_cycle_success_rate` | `(age: f64, embryo_quality: f64, endometrial_thickness: f64) -> f64` |
| `ovarian_reserve_amh` | `(amh_ng_ml: f64) -> &'static str` |
| `antral_follicle_response` | `(fsh_dose: f64, sensitivity: f64, max_follicles: f64) -> f64` |
| `ohss_risk` | `(estradiol: f64, follicle_count: usize, bmi: f64) -> f64` |
| `embryo_grading_score` | `(cell_count: usize, fragmentation_pct: f64, symmetry: f64) -> f64` |
| `blastocyst_expansion_rate` | `(hours_post_fertilization: f64) -> f64` |
| `cryopreservation_survival` | `(cooling_rate: f64, optimal_rate: f64, cpa_conc: f64) -> f64` |
| `cumulative_ivf_pregnancy_rate` | `(cycle_rate: f64, cycles: usize) -> f64` |
| `sperm_dna_fragmentation_impact` | `(dfi: f64, baseline_fertility: f64) -> f64` |

---

## stem_cell API (50 functions)

| Function | Signature |
|----------|-----------|
| `waddington_landscape_potential` | `( state: f64, attractor_a: f64, attractor_b: f64, barrier: f64, ) -> f64` |
| `differentiation_commitment` | `( transcription_factor_a: f64, transcription_factor_b: f64, hill: f64, ) -> f64` |
| `lineage_progression` | `( progenitor: f64, differentiation_rate: f64, proliferation_rate: f64, dt: f64, ) -> (f64, f64)` |
| `multipotency_index` | `(expressed_lineage_genes: &[f64]) -> f64` |
| `cell_fate_probability_stochastic` | `(tf_level: f64, noise: f64, threshold: f64) -> f64` |
| `directed_differentiation_efficiency` | `(target_markers: f64, total_cells: f64) -> f64` |
| `transdifferentiation_barrier` | `( epigenetic_distance: f64, reprogramming_factors: f64, efficiency_base: f64, ) -> f64` |
| `organoid_differentiation_layers` | `(time: f64, layer_rate: f64, max_layers: f64) -> f64` |
| `terminal_differentiation_irreversibility` | `(rb_phosphorylation: f64, cdki_level: f64) -> f64` |
| `self_renewal_probability` | `(symmetric_rate: f64, total_division_rate: f64) -> f64` |
| `stem_cell_pool_dynamics` | `(s: f64, r: f64, d: f64, p: f64, dt: f64) -> f64` |
| `asymmetric_division_output` | `( stem_cells: f64, division_rate: f64, asymmetric_fraction: f64, ) -> f64` |
| `lineage_commitment` | `(signal_strength: f64, threshold: f64, hill_n: f64) -> f64` |
| `niche_occupancy` | `(stem_cells: f64, niche_capacity: f64) -> f64` |
| `niche_competition` | `( resident: f64, challenger: f64, fitness_resident: f64, fitness_challenger: f64, ) -> f64` |
| `dedifferentiation_rate` | `(injury_signal: f64, plasticity: f64, baseline: f64) -> f64` |
| `stem_cell_aging` | `(initial_pool: f64, depletion_rate: f64, age: f64) -> f64` |
| `transit_amplifying_generations` | `( progenitor: f64, divisions: u32, survival_per_div: f64, ) -> f64` |
| `quiescence_exit_rate` | `(growth_factor: f64, threshold: f64, max_rate: f64) -> f64` |
| `clonal_dominance` | `(fitness: &[f64]) -> Vec<f64>` |
| `neutral_drift_clone_survival` | `(initial_clones: f64, time: f64, replacement_rate: f64) -> f64` |
| `hematopoietic_hierarchy_output` | `( hsc: f64, mpp_rate: f64, clp_rate: f64, cmp_rate: f64, ) -> (f64, f64)` |
| `telomere_shortening_per_division` | `( initial_length: f64, loss_per_division: f64, divisions: f64, ) -> f64` |
| `hayflick_limit_remaining` | `( telomere_length: f64, critical_length: f64, loss_per_division: f64, ) -> f64` |
| `symmetric_commitment_probability` | `(niche_signal: f64, k_niche: f64) -> f64` |
| `stem_cell_niche_occupancy` | `( stem_cells: f64, niche_capacity: f64, adhesion_strength: f64, ) -> f64` |
| `niche_signal_gradient` | `(source_strength: f64, distance: f64, decay_length: f64) -> f64` |
| `quiescence_probability` | `(niche_signal: f64, threshold: f64) -> f64` |
| `niche_asymmetric_division` | `(niche_polarization: f64, cell_polarity: f64) -> f64` |
| `hematopoietic_niche_osteoblast` | `( osteoblast_count: f64, hsc_supported: f64, max_ratio: f64, ) -> f64` |
| `perivascular_niche_oxygen` | `( distance_from_vessel: f64, vessel_po2: f64, consumption_rate: f64, diffusion: f64, ) -> f64` |
| `intestinal_crypt_dynamics` | `( stem_cells: f64, division_rate: f64, loss_rate: f64, niche_capacity: f64, dt: f64, ) -> f64` |
| `wnt_gradient_crypt` | `(position: f64, crypt_depth: f64, wnt_max: f64) -> f64` |
| `notch_lateral_inhibition_niche` | `(notch_signal: f64, delta_neighbors: f64, gain: f64) -> f64` |
| `mesenchymal_niche_paracrine` | `( mscs: f64, growth_factor_per_cell: f64, distance: f64, decay: f64, ) -> f64` |
| `reprogramming_efficiency` | `( oct4: f64, sox2: f64, klf4: f64, myc: f64, epigenetic_barrier: f64, ) -> f64` |
| `ipsc_colony_formation` | `( seeded_cells: f64, reprogramming_efficiency: f64, survival_fraction: f64, ) -> f64` |
| `differentiation_cascade` | `(progenitor: f64, rates: &[f64]) -> Vec<f64>` |
| `waddington_potential` | `(state: f64, landscape: impl Fn(f64) -> f64, noise: f64) -> f64` |
| `organoid_growth` | `(cells: f64, growth_rate: f64, carrying_capacity: f64, dt: f64) -> f64` |
| `yamanaka_factor_dynamics` | `( oct4: f64, sox2: f64, klf4: f64, myc: f64, dt: f64, degradation: f64, ) -> (f64, f64, f64, f64)` |
| `stochastic_reprogramming_events` | `(cells: usize, probability_per_cell: f64) -> f64` |
| `partial_reprogramming_state` | `( methylation_age: f64, cycles: usize, reset_per_cycle: f64, ) -> f64` |
| `direct_lineage_conversion` | `( efficiency_base: f64, tf_combination: &[f64], synergy: f64, ) -> f64` |
| `asymmetric_division_ratio` | `( stem_cells: f64, symmetric_prob: f64, differentiation_rate: f64, ) -> (f64, f64)` |
| `epigenetic_barrier_height` | `( methylation_level: f64, histone_marks: f64, chromatin_accessibility: f64, ) -> f64` |
| `crispr_activation_efficiency` | `( guide_specificity: f64, activator_strength: f64, chromatin_state: f64, ) -> f64` |
| `embryoid_body_formation` | `( single_cells: f64, aggregation_rate: f64, min_cells_per_eb: f64, ) -> f64` |
| `directed_differentiation_yield` | `( input_cells: f64, protocol_efficiency: f64, purity: f64, ) -> f64` |
| `maturation_index` | `(marker_expression: &[f64], weights: &[f64]) -> f64` |

---

## structural API (39 functions)

| Function | Signature |
|----------|-----------|
| `lennard_jones_potential` | `(r: f64, epsilon: f64, sigma: f64) -> f64` |
| `coulomb_interaction` | `(q1: f64, q2: f64, r: f64, dielectric: f64) -> f64` |
| `desolvation_energy` | `(buried_area: f64, solvation_parameter: f64) -> f64` |
| `shape_complementarity` | `(interface_area: f64, gap_volume: f64) -> f64` |
| `binding_free_energy` | `( van_der_waals: f64, electrostatic: f64, desolvation: f64, entropy_penalty: f64, ) -> f64` |
| `kd_from_delta_g` | `(delta_g: f64, temperature: f64) -> f64` |
| `buried_surface_area` | `(asa_complex: f64, asa_receptor: f64, asa_ligand: f64) -> f64` |
| `hydrogen_bond_energy` | `(distance: f64, angle_deg: f64) -> f64` |
| `clash_score` | `(distances: &[f64], vdw_threshold: f64) -> f64` |
| `interface_residue_count` | `(distances_to_partner: &[f64], cutoff: f64) -> usize` |
| `druggability_score` | `(pocket_volume: f64, hydrophobicity: f64, enclosure: f64) -> f64` |
| `euclidean_distance_3d` | `(a: &[f64; 3], b: &[f64; 3]) -> f64` |
| `bond_angle` | `(a: &[f64; 3], b: &[f64; 3], c: &[f64; 3]) -> f64` |
| `dihedral_angle` | `(a: &[f64; 3], b: &[f64; 3], c: &[f64; 3], d: &[f64; 3]) -> f64` |
| `center_of_mass` | `(coords: &[[f64; 3]], masses: &[f64]) -> [f64; 3]` |
| `radius_of_gyration` | `(coords: &[[f64; 3]], masses: &[f64]) -> f64` |
| `solvent_accessible_distance` | `(point: &[f64; 3], surface_points: &[[f64; 3]]) -> f64` |
| `inertia_tensor` | `(coords: &[[f64; 3]], masses: &[f64]) -> [[f64; 3]; 3]` |
| `planarity` | `(coords: &[[f64; 3]]) -> f64` |
| `alpha_helix_propensity` | `(residue_propensities: &[f64]) -> f64` |
| `beta_sheet_propensity` | `(residue_propensities: &[f64]) -> f64` |
| `chou_fasman_nucleation` | `(propensities: &[f64], window: usize, threshold: f64) -> Vec<bool>` |
| `gor_information_value` | `(residue_freq_in_structure: f64, residue_freq_overall: f64) -> f64` |
| `coiled_coil_probability` | `(heptad_score: f64, hydrophobic_moment: f64) -> f64` |
| `disorder_prediction` | `(hydrophobicity: f64, charge: f64, complexity: f64) -> f64` |
| `solvent_accessibility` | `(residue_asa: f64, max_asa: f64) -> f64` |
| `ramachandran_energy` | `(phi: f64, psi: f64) -> f64` |
| `relative_contact_order` | `(contacts: &[(usize, usize)], chain_length: usize) -> f64` |
| `hydrophobic_moment` | `(hydrophobicities: &[f64], angle_deg: f64) -> f64` |
| `rmsd` | `(coords_a: &[[f64; 3]], coords_b: &[[f64; 3]]) -> f64` |
| `gdt_ts` | `(coords_a: &[[f64; 3]], coords_b: &[[f64; 3]], cutoffs: &[f64]) -> f64` |
| `tm_score` | `(coords_a: &[[f64; 3]], coords_b: &[[f64; 3]], l_target: usize) -> f64` |
| `contact_map` | `(coords: &[[f64; 3]], cutoff: f64) -> Vec<(usize, usize)>` |
| `rg_from_coords` | `(coords: &[[f64; 3]]) -> f64` |
| `solvent_accessible_surface_approx` | `(radii: &[f64], probe: f64) -> f64` |
| `lrmsd` | `(coords_a: &[[f64; 3]], coords_b: &[[f64; 3]], residue_indices: &[usize]) -> f64` |
| `drmsd` | `(coords_a: &[[f64; 3]], coords_b: &[[f64; 3]]) -> f64` |
| `absolute_contact_order` | `(contacts: &[(usize, usize)], n_residues: usize) -> f64` |
| `b_factor_normalize` | `(b_factors: &[f64]) -> Vec<f64>` |

---


## Hub dispatch mapping

All biology functions are wired through:

- `src/hub/engine/dispatch/biology.rs`
