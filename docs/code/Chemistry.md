# Chemistry Source Code Guide

This page documents the source implementation behind `sciforge::chemistry`, including file layout and Hub dispatch wiring.

## Source Coverage

### What is explained
- File-level implementation layout in `src/chemistry/`.
- Main computation groups and where they are implemented.
- Runtime call path when chemistry functions are executed through Hub dispatch.

### Visibility and external access
- Visibility: internal (`pub(crate)` in `src/lib.rs`).
- External consumers should use `sciforge::hub` as the public entry point.
- Paths in this page are for source reading and internal crate development.


## Source Code Explanation

### Relevant file structure
- Main implementation: `src/chemistry/`
- Module entry point: `src/chemistry/mod.rs`
- Hub routing: `src/hub/engine/dispatch/chemistry.rs`

### Internal execution flow
1. The module exposes subdomains through `mod.rs` and targeted `pub use` exports.
2. Each `.rs` file implements a coherent block of equations and functions.
3. The Hub invokes these functions through the domain dispatcher when execution goes through `ExperimentRunner`.

### What to check while reading code
- Exact signature: input/output types and argument order.
- Numerical preconditions: divisions, roots, logarithms, and validity ranges.
- Implicit conventions: units, normalization, bounds, and tolerances.

## Modules

- `acid_base`
- `analytical`
- `colloids`
- `computational`
- `crystallography`
- `electrochemistry`
- `environmental`
- `equilibrium`
- `gas_laws`
- `green_chemistry`
- `inorganic`
- `kinetics`
- `molecular`
- `nuclear`
- `organic`
- `photochemistry`
- `polymers`
- `quantum_chem`
- `reaction_engineering`
- `solid_state`
- `solutions`
- `spectroscopy`
- `stoichiometry`
- `surface`
- `thermochemistry`
- `transport`

---

## Acid-Base API (19 functions)

### equilibria (11 functions)

| Function | Signature |
|----------|-----------|
| `henderson_hasselbalch` | `(pka: f64, base: f64, acid: f64) -> f64` |
| `pka_from_ka` | `(ka: f64) -> f64` |
| `ka_from_pka` | `(pka: f64) -> f64` |
| `pkb_from_pka` | `(pka: f64, pkw: f64) -> f64` |
| `ph_strong_acid` | `(concentration: f64) -> f64` |
| `ph_strong_base` | `(concentration: f64) -> f64` |
| `ph_weak_acid` | `(ka: f64, c: f64) -> f64` |
| `ph_weak_base` | `(kb: f64, c: f64) -> f64` |
| `alpha_fraction` | `(h: f64, ka_values: &[f64], species_index: usize) -> f64` |
| `amphiprotic_ph` | `(pka1: f64, pka2: f64) -> f64` |
| `ionic_product_water` | `(t: f64) -> f64` |

### titrations (8 functions)

| Function | Signature |
|----------|-----------|
| `strong_acid_strong_base_ph` | `(c_acid: f64, v_acid: f64, c_base: f64, v_base: f64) -> f64` |
| `weak_acid_strong_base_ph` | `(c_acid: f64, v_acid: f64, ka: f64, c_base: f64, v_base: f64) -> f64` |
| `equivalence_point_volume` | `(c_analyte: f64, v_analyte: f64, c_titrant: f64) -> f64` |
| `half_equivalence_ph` | `(pka: f64) -> f64` |
| `buffer_range` | `(pka: f64) -> (f64, f64)` |
| `buffer_capacity_vanslyke` | `(c_total: f64, ka: f64, h: f64) -> f64` |
| `diprotic_ph_first_equiv` | `(pka1: f64, pka2: f64) -> f64` |
| `back_titration_moles` | `(mol_excess_added: f64, c_back_titrant: f64, v_back_titrant: f64) -> f64` |

---

## Analytical API (20 functions)

### chromatography (7 functions)

| Function | Signature |
|----------|-----------|
| `retention_factor_rf` | `(distance_solute: f64, distance_solvent: f64) -> f64` |
| `hplc_resolution` | `(tr1: f64, tr2: f64, w1: f64, w2: f64) -> f64` |
| `theoretical_plates` | `(tr: f64, w: f64) -> f64` |
| `height_equivalent_theoretical_plate` | `(column_length: f64, n_plates: f64) -> f64` |
| `van_deemter` | `(a: f64, b: f64, c: f64, u: f64) -> f64` |
| `selectivity_factor` | `(k2: f64, k1: f64) -> f64` |
| `capacity_factor` | `(tr: f64, t0: f64) -> f64` |

### quantitative (8 functions)

| Function | Signature |
|----------|-----------|
| `dilution` | `(c1: f64, v1: f64, v2: f64) -> f64` |
| `titration_equivalence_volume` | `(c_analyte: f64, v_analyte: f64, c_titrant: f64, stoich_ratio: f64) -> f64` |
| `limit_of_detection` | `(blank_std: f64, slope: f64) -> f64` |
| `limit_of_quantitation` | `(blank_std: f64, slope: f64) -> f64` |
| `percent_recovery` | `(measured: f64, expected: f64) -> f64` |
| `relative_standard_deviation` | `(std_dev: f64, mean: f64) -> f64` |
| `gravimetric_factor` | `(mw_analyte: f64, mw_precipitate: f64, stoich: f64) -> f64` |
| `karl_fischer_water_content` | `(volume_reagent: f64, reagent_factor: f64, sample_mass: f64) -> f64` |

### spectrophotometry (5 functions)

| Function | Signature |
|----------|-----------|
| `beer_lambert` | `(epsilon: f64, path_length: f64, concentration: f64) -> f64` |
| `absorbance_to_transmittance` | `(absorbance: f64) -> f64` |
| `transmittance_to_absorbance` | `(transmittance: f64) -> f64` |
| `concentration_from_absorbance` | `(absorbance: f64, epsilon: f64, path_length: f64) -> f64` |
| `signal_to_noise` | `(signal: f64, noise: f64) -> f64` |

---

## Colloids API (18 functions)

### properties (10 functions)

| Function | Signature |
|----------|-----------|
| `stokes_sedimentation` | `(r: f64, rho_p: f64, rho_f: f64, viscosity: f64) -> f64` |
| `brownian_diffusion_coefficient` | `(t: f64, viscosity: f64, r: f64) -> f64` |
| `einstein_diffusion_displacement` | `(d: f64, t: f64) -> f64` |
| `peclet_number_colloid` | `(velocity: f64, r: f64, d: f64) -> f64` |
| `osmotic_pressure_colloid` | `(n_particles: f64, volume: f64, t: f64) -> f64` |
| `sedimentation_coefficient` | `(velocity: f64, omega: f64, r_centrifuge: f64) -> f64` |
| `tyndall_scattering_intensity` | `(n: f64, v_particle: f64, wavelength: f64) -> f64` |
| `specific_surface_area` | `(radius: f64, density: f64) -> f64` |
| `flocculation_rate_smoluchowski` | `(n0: f64, k_b: f64, t: f64, viscosity: f64) -> f64` |
| `half_life_coagulation` | `(n0: f64, k_rate: f64) -> f64` |

### stability (8 functions)

| Function | Signature |
|----------|-----------|
| `dlvo_total_energy` | `(van_der_waals: f64, electrostatic: f64) -> f64` |
| `hamaker_sphere_sphere` | `(a_h: f64, r1: f64, r2: f64, d: f64) -> f64` |
| `hamaker_sphere_surface` | `(a_h: f64, r: f64, d: f64) -> f64` |
| `debye_length` | `(epsilon_r: f64, t: f64, ionic_strength: f64) -> f64` |
| `electrostatic_repulsion` | `(epsilon_r: f64, r: f64, psi0: f64, kappa: f64, d: f64) -> f64` |
| `zeta_potential_smoluchowski` | `(mobility: f64, viscosity: f64, epsilon: f64) -> f64` |
| `schulze_hardy_ccc` | `(z: i32) -> f64` |
| `critical_coagulation_concentration` | `(epsilon: f64, t: f64, psi0: f64, a_h: f64, z: f64) -> f64` |

---

## Computational API (18 functions)

### basis_sets (9 functions)

| Function | Signature |
|----------|-----------|
| `gaussian_primitive` | `(alpha: f64, r_sq: f64) -> f64` |
| `normalization_s_orbital` | `(alpha: f64) -> f64` |
| `normalization_p_orbital` | `(alpha: f64) -> f64` |
| `slater_exponent` | `(z_eff: f64, n: f64) -> f64` |
| `boys_function_zero` | `(t: f64) -> f64` |
| `overlap_integral_1s` | `(alpha1: f64, alpha2: f64, r_sq: f64) -> f64` |
| `kinetic_integral_1s` | `(alpha1: f64, alpha2: f64, r_sq: f64) -> f64` |
| `sto_ng_coefficients` | `(n: usize) -> Vec<(f64, f64)>` |
| `contracted_gaussian` | `(coeffs: &[(f64, f64)], r_sq: f64) -> f64` |

### dft (9 functions)

| Function | Signature |
|----------|-----------|
| `thomas_fermi_kinetic_energy` | `(density: &[f64], volume_element: f64) -> f64` |
| `exchange_energy_lda` | `(density: &[f64], volume_element: f64) -> f64` |
| `hartree_energy` | `(density: &[f64], potential: &[f64], volume_element: f64) -> f64` |
| `nuclear_attraction_energy` | `(z: f64, r: f64) -> f64` |
| `electron_nuclear_energy` | `(density: &[f64], distances: &[f64], z: f64, volume_element: f64) -> f64` |
| `xc_potential_lda` | `(density: f64) -> f64` |
| `correlation_energy_vwn` | `(rs: f64) -> f64` |
| `wigner_seitz_radius` | `(density: f64) -> f64` |
| `kohn_sham_total_energy` | `(eigenvalue_sum: f64, hartree: f64, xc_energy: f64, xc_potential_integral: f64) -> f64` |

---

## Crystallography API (16 functions)

### diffraction (6 functions)

| Function | Signature |
|----------|-----------|
| `bragg_law` | `(d: f64, theta: f64, n: u32) -> f64` |
| `bragg_angle` | `(wavelength: f64, d: f64, n: u32) -> f64` |
| `interplanar_spacing_cubic` | `(a: f64, h: i32, k: i32, l: i32) -> f64` |
| `structure_factor_bcc` | `(h: i32, k: i32, l: i32, f_atom: f64) -> f64` |
| `structure_factor_fcc` | `(h: i32, k: i32, l: i32, f_atom: f64) -> f64` |
| `scherrer_crystal_size` | `(k: f64, wavelength: f64, fwhm: f64, theta: f64) -> f64` |

### lattice (10 functions)

| Function | Signature |
|----------|-----------|
| `cubic_volume` | `(a: f64) -> f64` |
| `tetragonal_volume` | `(a: f64, c: f64) -> f64` |
| `orthorhombic_volume` | `(a: f64, b: f64, c: f64) -> f64` |
| `hexagonal_volume` | `(a: f64, c: f64) -> f64` |
| `packing_fraction_simple_cubic` | `() -> f64` |
| `packing_fraction_bcc` | `() -> f64` |
| `packing_fraction_fcc` | `() -> f64` |
| `density_from_unit_cell` | `(z: u32, molar_mass: f64, volume: f64, avogadro: f64) -> f64` |
| `miller_to_direction_cosines` | `(h: i32, k: i32, l: i32) -> (f64, f64, f64)` |
| `reciprocal_lattice_vector` | `(a: f64) -> f64` |

---

## Electrochemistry API (20 functions)

### cells (10 functions)

| Function | Signature |
|----------|-----------|
| `nernst_potential` | `(e0: f64, n: f64, q: f64, t: f64) -> f64` |
| `cell_potential` | `(e_cathode: f64, e_anode: f64) -> f64` |
| `gibbs_from_cell_potential` | `(n: f64, e_cell: f64) -> f64` |
| `faraday_mass` | `(i: f64, t: f64, m: f64, n: f64) -> f64` |
| `overpotential_tafel` | `(a: f64, b: f64, current_density: f64) -> f64` |
| `butler_volmer` | `(i0: f64, alpha_a: f64, alpha_c: f64, eta: f64, t: f64) -> f64` |
| `open_circuit_voltage` | `(e_cathode: f64, e_anode: f64, n_electrons: f64, t: f64, q: f64) -> f64` |
| `faradaic_efficiency` | `(actual_yield: f64, theoretical_yield: f64) -> f64` |
| `energy_density_battery` | `(voltage: f64, capacity_ah: f64, mass_kg: f64) -> f64` |
| `coulombic_efficiency` | `(discharge_capacity: f64, charge_capacity: f64) -> f64` |

### transport (10 functions)

| Function | Signature |
|----------|-----------|
| `conductivity` | `(resistance: f64, cell_constant: f64) -> f64` |
| `molar_conductivity` | `(conductivity: f64, concentration: f64) -> f64` |
| `kohlrausch` | `(lambda_inf: f64, k: f64, c: f64) -> f64` |
| `debye_huckel_activity` | `(z: f64, ionic_strength: f64) -> f64` |
| `ionic_strength` | `(ions: &[(f64, f64)]) -> f64` |
| `transference_number` | `(lambda_ion: f64, lambda_total: f64) -> f64` |
| `debye_huckel_extended` | `(z: f64, ionic_strength: f64, a_ion: f64) -> f64` |
| `onsager_equation` | `(lambda_inf: f64, a: f64, b: f64, c: f64) -> f64` |
| `walden_product` | `(viscosity: f64, molar_conductivity: f64) -> f64` |
| `mobility_from_conductivity` | `(lambda_ion: f64, z: f64) -> f64` |

---

## Environmental API (18 functions)

### atmosphere (8 functions)

| Function | Signature |
|----------|-----------|
| `ozone_formation_rate` | `(k: f64, no2: f64, hv: f64) -> f64` |
| `ozone_destruction_rate` | `(k: f64, o3: f64, no: f64) -> f64` |
| `photochemical_smog_potential` | `(voc: f64, nox: f64) -> f64` |
| `atmospheric_lifetime` | `(concentration: f64, removal_rate: f64) -> f64` |
| `global_warming_potential` | `(rf_gas: f64, lifetime_gas: f64, rf_co2: f64, lifetime_co2: f64) -> f64` |
| `henry_law_solubility` | `(kh: f64, partial_pressure: f64) -> f64` |
| `ozone_depletion_potential` | `(cl_atoms: f64, lifetime: f64, mw: f64) -> f64` |
| `photolysis_rate_constant` | `(quantum_yield: f64, absorption: f64, actinic_flux: f64) -> f64` |

### water (10 functions)

| Function | Signature |
|----------|-----------|
| `biochemical_oxygen_demand` | `(bod_ultimate: f64, k: f64, t: f64) -> f64` |
| `chemical_oxygen_demand` | `(sample_oxygen: f64, blank_oxygen: f64, volume: f64) -> f64` |
| `dissolved_oxygen_saturation` | `(t: f64) -> f64` |
| `streeter_phelps` | `(d0: f64, l0: f64, kd: f64, kr: f64, t: f64) -> f64` |
| `critical_point_time` | `(kd: f64, kr: f64, d0: f64, l0: f64) -> f64` |
| `chlorine_decay` | `(c0: f64, k: f64, t: f64) -> f64` |
| `ct_disinfection` | `(c: f64, t: f64) -> f64` |
| `hardness_total` | `(ca_mg_l: f64, mg_mg_l: f64) -> f64` |
| `langelier_saturation_index` | `(ph: f64, ph_s: f64) -> f64` |
| `total_dissolved_solids_from_conductivity` | `(conductivity_us: f64) -> f64` |

---

## Equilibrium API (18 functions)

### constants (9 functions)

| Function | Signature |
|----------|-----------|
| `equilibrium_constant_from_gibbs` | `(delta_g: f64, t: f64) -> f64` |
| `reaction_quotient` | `(products: &[(f64, f64)], reactants: &[(f64, f64)]) -> f64` |
| `le_chatelier_shift` | `(q: f64, keq: f64) -> i32` |
| `kp_from_kc` | `(kc: f64, t: f64, delta_n: f64) -> f64` |
| `vant_hoff` | `(k1: f64, delta_h: f64, t1: f64, t2: f64) -> f64` |
| `degree_of_dissociation` | `(keq: f64, c0: f64) -> f64` |
| `temperature_dependence_keq` | `(k_ref: f64, delta_h: f64, t_ref: f64, t: f64) -> f64` |
| `gibbs_from_keq` | `(keq: f64, t: f64) -> f64` |
| `pressure_effect_on_keq` | `(keq: f64, delta_v: f64, p1: f64, p2: f64, t: f64) -> f64` |

### ionic (9 functions)

| Function | Signature |
|----------|-----------|
| `common_ion_effect` | `(ksp: f64, common_ion_conc: f64, stoich_coeff: f64) -> f64` |
| `buffer_capacity` | `(ca: f64, cb: f64, ka: f64, h: f64) -> f64` |
| `ph_weak_acid` | `(ka: f64, c: f64) -> f64` |
| `ph_buffer` | `(ka: f64, acid: f64, base: f64) -> f64` |
| `solubility_product` | `(ion_concentrations: &[(f64, f64)]) -> f64` |
| `distribution_coefficient` | `(c_organic: f64, c_aqueous: f64) -> f64` |
| `solubility_from_ksp` | `(ksp: f64, cation_stoich: f64, anion_stoich: f64) -> f64` |
| `formation_constant` | `(product_conc: f64, metal_conc: f64, ligand_conc: f64, n: f64) -> f64` |
| `conditional_formation_constant` | `(kf: f64, alpha_y: f64) -> f64` |

---

## Gas Laws API (27 functions)

### ideal (14 functions)

| Function | Signature |
|----------|-----------|
| `ideal_gas_pressure` | `(n: f64, t: f64, v: f64) -> f64` |
| `ideal_gas_volume` | `(n: f64, t: f64, p: f64) -> f64` |
| `ideal_gas_temperature` | `(p: f64, v: f64, n: f64) -> f64` |
| `boyles_law` | `(p1: f64, v1: f64, v2: f64) -> f64` |
| `charles_law` | `(v1: f64, t1: f64, t2: f64) -> f64` |
| `gay_lussac_law` | `(p1: f64, t1: f64, t2: f64) -> f64` |
| `combined_gas_law` | `(p1: f64, v1: f64, t1: f64, t2: f64, p2: f64) -> f64` |
| `dalton_partial_pressure` | `(mole_fraction: f64, total_pressure: f64) -> f64` |
| `grahams_law_effusion` | `(m1: f64, m2: f64) -> f64` |
| `gas_density` | `(p: f64, mw: f64, t: f64) -> f64` |
| `rms_speed` | `(t: f64, mw: f64) -> f64` |
| `mean_speed` | `(t: f64, mw: f64) -> f64` |
| `most_probable_speed` | `(t: f64, mw: f64) -> f64` |
| `mean_free_path` | `(d: f64, n_over_v: f64) -> f64` |

### real (13 functions)

| Function | Signature |
|----------|-----------|
| `van_der_waals_pressure` | `(n: f64, t: f64, v: f64, a: f64, b: f64) -> f64` |
| `redlich_kwong_pressure` | `(n: f64, t: f64, v: f64, a: f64, b: f64) -> f64` |
| `compressibility_factor` | `(p: f64, v: f64, n: f64, t: f64) -> f64` |
| `virial_equation_b` | `(p: f64, t: f64, b: f64) -> f64` |
| `boyle_temperature` | `(a: f64, b: f64) -> f64` |
| `critical_temperature` | `(a: f64, b: f64) -> f64` |
| `critical_pressure` | `(a: f64, b: f64) -> f64` |
| `critical_volume` | `(b: f64) -> f64` |
| `reduced_temperature` | `(t: f64, tc: f64) -> f64` |
| `reduced_pressure` | `(p: f64, pc: f64) -> f64` |
| `peng_robinson_pressure` | `(t: f64, vm: f64, a: f64, b: f64) -> f64` |
| `fugacity_coefficient` | `(z: f64, b_prime: f64, p: f64) -> f64` |
| `acentric_factor` | `(p_sat: f64, pc: f64) -> f64` |

---

## Green Chemistry API (18 functions)

### metrics (10 functions)

| Function | Signature |
|----------|-----------|
| `atom_economy` | `(mw_product: f64, mw_all_products: f64) -> f64` |
| `e_factor` | `(mass_waste: f64, mass_product: f64) -> f64` |
| `process_mass_intensity` | `(total_mass_input: f64, mass_product: f64) -> f64` |
| `reaction_mass_efficiency` | `(mass_product: f64, total_mass_reactants: f64) -> f64` |
| `carbon_efficiency` | `(mass_carbon_product: f64, mass_carbon_reactants: f64) -> f64` |
| `mass_productivity` | `(mass_product: f64, total_mass_used: f64) -> f64` |
| `solvent_intensity` | `(mass_solvent: f64, mass_product: f64) -> f64` |
| `water_intensity` | `(mass_water: f64, mass_product: f64) -> f64` |
| `effective_mass_yield` | `(mass_product: f64, mass_non_benign: f64) -> f64` |
| `renewable_feedstock_index` | `(mass_renewable: f64, total_mass: f64) -> f64` |

### principles (8 functions)

| Function | Signature |
|----------|-----------|
| `catalyst_turnover_number` | `(moles_product: f64, moles_catalyst: f64) -> f64` |
| `catalyst_turnover_frequency` | `(ton: f64, time: f64) -> f64` |
| `energy_efficiency` | `(useful_energy: f64, total_energy: f64) -> f64` |
| `selectivity` | `(moles_desired: f64, moles_converted: f64) -> f64` |
| `yield_from_selectivity_conversion` | `(selectivity_frac: f64, conversion_frac: f64) -> f64` |
| `stoichiometric_factor` | `(actual_mass_reactant: f64, stoichiometric_mass: f64) -> f64` |
| `environmental_quotient` | `(e_factor: f64, unfriendliness: f64) -> f64` |
| `mass_index` | `(total_mass_input: f64, total_mass_output: f64) -> f64` |

---

## Inorganic API (16 functions)

### bonding (4 functions)

| Function | Signature |
|----------|-----------|
| `lattice_energy_born_lande` | `(madelung: f64, z_plus: f64, z_minus: f64, r0: f64, born_exponent: f64) -> f64` |
| `lattice_energy_kapustinskii` | `(nu: u32, z_plus: f64, z_minus: f64, r_plus: f64, r_minus: f64) -> f64` |
| `pauling_electronegativity_difference` | `(en_a: f64, en_b: f64) -> f64` |
| `percent_ionic_character` | `(en_diff: f64) -> f64` |

### coordination (4 functions)

| Function | Signature |
|----------|-----------|
| `effective_atomic_number` | `(metal_electrons: u32, ligand_electrons: u32) -> u32` |
| `chelate_effect` | `(k_mono: f64, k_chelate: f64) -> f64` |
| `irving_williams_stability` | `(ionization_energy: f64, ionic_radius: f64) -> f64` |
| `coordination_number_radius_ratio` | `(r_cation: f64, r_anion: f64) -> u32` |

### crystal_field (8 functions)

| Function | Signature |
|----------|-----------|
| `crystal_field_splitting_octahedral` | `(dq: f64) -> f64` |
| `crystal_field_splitting_tetrahedral` | `(dq_oct: f64) -> f64` |
| `cfse_octahedral` | `(t2g: u32, eg: u32, dq: f64, pairing_energy: f64) -> f64` |
| `magnetic_moment_spin_only` | `(unpaired: u32) -> f64` |
| `spectrochemical_series_dq` | `(ligand_f: f64, metal_g: f64) -> f64` |
| `jahn_teller_distortion` | `(eg_occupation: u32) -> bool` |
| `nephelauxetic_ratio` | `(b_complex: f64, b_free_ion: f64) -> f64` |
| `racah_parameter_b` | `(transitions: &[f64], dq: f64) -> f64` |

---

## Kinetics API (23 functions)

### mechanisms (11 functions)

| Function | Signature |
|----------|-----------|
| `michaelis_menten` | `(vmax: f64, km: f64, s: f64) -> f64` |
| `lineweaver_burk_slope` | `(km: f64, vmax: f64) -> f64` |
| `lineweaver_burk_intercept` | `(vmax: f64) -> f64` |
| `lindemann_unimolecular` | `(k_inf: f64, k0: f64, m: f64) -> f64` |
| `consecutive_reaction` | `(c0: f64, k1: f64, k2: f64, t: f64) -> (f64, f64, f64)` |
| `reversible_first_order` | `(c0: f64, kf: f64, kr: f64, t: f64) -> (f64, f64)` |
| `parallel_reactions` | `(c0: f64, k_values: &[f64], t: f64) -> Vec<f64>` |
| `steady_state_intermediate` | `(k1: f64, k_minus1: f64, k2: f64, a: f64) -> f64` |
| `pre_equilibrium_rate` | `(k1: f64, k_minus1: f64, k2: f64, a: f64, b: f64) -> f64` |
| `competitive_inhibition` | `(vmax: f64, km: f64, s: f64, i: f64, ki: f64) -> f64` |
| `uncompetitive_inhibition` | `(vmax: f64, km: f64, s: f64, i: f64, ki: f64) -> f64` |

### rates (12 functions)

| Function | Signature |
|----------|-----------|
| `rate_constant_arrhenius` | `(a: f64, ea: f64, t: f64) -> f64` |
| `half_life_first_order` | `(k: f64) -> f64` |
| `concentration_first_order` | `(c0: f64, k: f64, t: f64) -> f64` |
| `concentration_second_order` | `(c0: f64, k: f64, t: f64) -> f64` |
| `concentration_zero_order` | `(c0: f64, k: f64, t: f64) -> f64` |
| `rate_law` | `(k: f64, concentrations: &[f64], orders: &[f64]) -> f64` |
| `activation_energy_two_temps` | `(k1: f64, k2: f64, t1: f64, t2: f64) -> f64` |
| `half_life_second_order` | `(k: f64, c0: f64) -> f64` |
| `half_life_zero_order` | `(k: f64, c0: f64) -> f64` |
| `integrated_rate_law_nth` | `(c0: f64, k: f64, t: f64, n: f64) -> f64` |
| `eyring_equation` | `(kappa: f64, kb: f64, t: f64, h: f64, delta_g_dagger: f64) -> f64` |
| `collision_frequency` | `(na: f64, nb: f64, sigma: f64, t: f64, mu: f64) -> f64` |

---

## Molecular API (20 functions)

### bonding (12 functions)

| Function | Signature |
|----------|-----------|
| `bond_order` | `(bonding_electrons: u32, antibonding_electrons: u32) -> f64` |
| `dipole_moment` | `(charges: &[(f64, [f64; 3])]) -> [f64; 3]` |
| `dipole_magnitude` | `(mu: &[f64; 3]) -> f64` |
| `coulomb_integral` | `(z_eff: f64, n: f64) -> f64` |
| `slater_shielding` | `(electrons_below: &[u32], shielding_constants: &[f64]) -> f64` |
| `electronegativity_mulliken` | `(ie: f64, ea: f64) -> f64` |
| `formal_charge` | `(valence: i32, lone_pair: i32, bonding: i32) -> i32` |
| `molar_mass` | `(atomic_masses: &[f64], counts: &[u32]) -> f64` |
| `percent_composition` | `(element_mass: f64, element_count: u32, molar_mass: f64) -> f64` |
| `electronegativity_allred_rochow` | `(z_eff: f64, r_cov: f64) -> f64` |
| `polarizability_estimate` | `(volume_angstrom3: f64) -> f64` |
| `bond_dissociation_energy_pauling` | `(d_aa: f64, d_bb: f64, en_diff: f64) -> f64` |

### geometry (8 functions)

| Function | Signature |
|----------|-----------|
| `vsepr_angle` | `(bonding_pairs: u32, lone_pairs: u32) -> f64` |
| `hybridization_sp` | `(bonding_regions: u32) -> &'static str` |
| `ideal_gas_moles` | `(p: f64, v: f64, t: f64) -> f64` |
| `molecular_geometry_name` | `(bonding_pairs: u32, lone_pairs: u32) -> &'static str` |
| `bond_length_estimate` | `(r1: f64, r2: f64) -> f64` |
| `bond_energy_badger` | `(r_e: f64, a: f64, b: f64) -> f64` |
| `coordination_geometry_angles` | `(coordination: u32) -> f64` |
| `effective_nuclear_charge` | `(z: u32, s: f64) -> f64` |

---

## Nuclear API (12 functions)

### decay (6 functions)

| Function | Signature |
|----------|-----------|
| `half_life_to_decay_constant` | `(half_life: f64) -> f64` |
| `decay_constant_to_half_life` | `(lambda: f64) -> f64` |
| `remaining_nuclei` | `(n0: f64, lambda: f64, t: f64) -> f64` |
| `activity` | `(lambda: f64, n: f64) -> f64` |
| `decay_chain_intermediate` | `(n0: f64, lambda1: f64, lambda2: f64, t: f64) -> f64` |
| `specific_activity` | `(lambda: f64, avogadro: f64, molar_mass: f64) -> f64` |

### energy (6 functions)

| Function | Signature |
|----------|-----------|
| `mass_defect` | `(z: u32, n: u32, atomic_mass: f64) -> f64` |
| `binding_energy` | `(mass_defect: f64) -> f64` |
| `binding_energy_per_nucleon` | `(binding_energy: f64, a: u32) -> f64` |
| `q_value` | `(reactant_masses: &[f64], product_masses: &[f64]) -> f64` |
| `semi_empirical_mass_formula` | `(a: u32, z: u32, av: f64, as_: f64, ac: f64, aa: f64, ap: f64) -> f64` |
| `threshold_energy` | `(q: f64, mass_projectile: f64, mass_target: f64) -> f64` |

---

## Organic API (20 functions)

### descriptors (6 functions)

| Function | Signature |
|----------|-----------|
| `cahn_ingold_prelog_priority` | `(atomic_numbers: &[u32]) -> Vec<usize>` |
| `topological_index_wiener` | `(distance_matrix: &[Vec<u32>]) -> u64` |
| `randic_index` | `(adjacency: &[Vec<bool>], degrees: &[u32]) -> f64` |
| `partition_coefficient_log_p` | `(fragments: &[f64]) -> f64` |
| `topological_polar_surface_area` | `(contributions: &[f64]) -> f64` |
| `rotatable_bonds` | `(single_bonds: u32, ring_bonds: u32, terminal_bonds: u32) -> u32` |

### reactions (7 functions)

| Function | Signature |
|----------|-----------|
| `sn1_rate` | `(k: f64, substrate: f64) -> f64` |
| `sn2_rate` | `(k: f64, substrate: f64, nucleophile: f64) -> f64` |
| `e1_rate` | `(k: f64, substrate: f64) -> f64` |
| `e2_rate` | `(k: f64, substrate: f64, base: f64) -> f64` |
| `hammett_equation` | `(rho: f64, sigma: f64, log_k0: f64) -> f64` |
| `hammett_acidity` | `(pka_ref: f64, rho: f64, sigma: f64) -> f64` |
| `taft_equation` | `(rho_star: f64, sigma_star: f64, es: f64, delta: f64) -> f64` |

### structure (7 functions)

| Function | Signature |
|----------|-----------|
| `degree_of_unsaturation` | `(c: u32, h: u32, n: u32, halogens: u32) -> f64` |
| `molecular_formula_mass` | `(c: u32, h: u32, o: u32, n: u32, s: u32) -> f64` |
| `alkane_boiling_point_estimate` | `(carbon_count: u32) -> f64` |
| `huckel_energy_linear` | `(n_atoms: usize, alpha: f64, beta: f64) -> Vec<f64>` |
| `huckel_energy_cyclic` | `(n_atoms: usize, alpha: f64, beta: f64) -> Vec<f64>` |
| `delocalization_energy` | `(total_pi_energy: f64, isolated_energy: f64) -> f64` |
| `resonance_stabilization` | `(n_structures: usize) -> f64` |

---

## Photochemistry API (13 functions)

### kinetics (7 functions)

| Function | Signature |
|----------|-----------|
| `photolysis_rate` | `(quantum_yield: f64, absorption_cross_section: f64, flux: f64) -> f64` |
| `stern_volmer` | `(i0: f64, ksv: f64, quencher: f64) -> f64` |
| `stern_volmer_ratio` | `(ksv: f64, quencher: f64) -> f64` |
| `rate_intersystem_crossing` | `(total_rate: f64, rate_fluorescence: f64, rate_internal_conversion: f64) -> f64` |
| `phosphorescence_lifetime` | `(rate_phosphorescence: f64, rate_non_radiative: f64) -> f64` |
| `forster_radius` | `(quantum_yield_donor: f64, kappa_sq: f64, overlap_integral: f64, n_refraction: f64) -> f64` |
| `fret_efficiency` | `(r: f64, r0: f64) -> f64` |

### quantum_yield (6 functions)

| Function | Signature |
|----------|-----------|
| `quantum_yield` | `(molecules_reacted: f64, photons_absorbed: f64) -> f64` |
| `photon_energy` | `(wavelength_nm: f64) -> f64` |
| `photon_energy_ev` | `(wavelength_nm: f64) -> f64` |
| `einstein_energy` | `(wavelength_nm: f64) -> f64` |
| `fluorescence_lifetime` | `(rate_radiative: f64, rate_non_radiative: f64) -> f64` |
| `fluorescence_quantum_yield` | `(rate_radiative: f64, rate_non_radiative: f64) -> f64` |

---

## Polymers API (14 functions)

### distributions (6 functions)

| Function | Signature |
|----------|-----------|
| `number_average_molar_mass` | `(ni: &[f64], mi: &[f64]) -> f64` |
| `weight_average_molar_mass` | `(ni: &[f64], mi: &[f64]) -> f64` |
| `z_average_molar_mass` | `(ni: &[f64], mi: &[f64]) -> f64` |
| `schulz_flory_distribution` | `(p: f64, x: u32) -> f64` |
| `most_probable_chain_length` | `(p: f64) -> f64` |
| `flory_huggins_free_energy` | `(phi: f64, n1: f64, n2: f64, chi: f64, temperature: f64) -> f64` |

### properties (8 functions)

| Function | Signature |
|----------|-----------|
| `degree_of_polymerization_number` | `(mn: f64, m0: f64) -> f64` |
| `degree_of_polymerization_weight` | `(mw: f64, m0: f64) -> f64` |
| `polydispersity_index` | `(mw: f64, mn: f64) -> f64` |
| `intrinsic_viscosity_mark_houwink` | `(k: f64, m: f64, a: f64) -> f64` |
| `end_to_end_distance_freely_jointed` | `(n: f64, l: f64) -> f64` |
| `radius_of_gyration` | `(end_to_end: f64) -> f64` |
| `glass_transition_fox` | `(w1: f64, tg1: f64, w2: f64, tg2: f64) -> f64` |
| `carothers_equation` | `(p: f64, f_avg: f64) -> f64` |

---

## Quantum Chemistry API (12 functions)

### huckel (6 functions)

| Function | Signature |
|----------|-----------|
| `huckel_secular_determinant` | `(n: usize, alpha: f64, beta: f64) -> Vec<Vec<f64>>` |
| `huckel_cyclic_determinant` | `(n: usize, alpha: f64, beta: f64) -> Vec<Vec<f64>>` |
| `huckel_charge_density` | `(coefficients: &[Vec<f64>], occupations: &[f64]) -> Vec<f64>` |
| `huckel_bond_order` | `(coefficients: &[Vec<f64>], occupations: &[f64], atom_i: usize, atom_j: usize) -> f64` |
| `huckel_total_pi_energy` | `(eigenvalues: &[f64], occupations: &[f64]) -> f64` |
| `huckel_free_valence` | `(bond_orders_sum: f64) -> f64` |

### orbitals (6 functions)

| Function | Signature |
|----------|-----------|
| `lcao_bonding_energy` | `(alpha: f64, beta: f64, overlap: f64) -> f64` |
| `lcao_antibonding_energy` | `(alpha: f64, beta: f64, overlap: f64) -> f64` |
| `overlap_integral_1s` | `(r: f64, zeta: f64) -> f64` |
| `hartree_energy` | `(kinetic: f64, nuclear_attraction: f64, electron_repulsion: f64) -> f64` |
| `roothaan_total_energy` | `(one_electron: &[f64], fock_eigenvalues: &[f64], nuclear_repulsion: f64) -> f64` |
| `mulliken_population` | `(density: &[Vec<f64>], overlap: &[Vec<f64>]) -> Vec<f64>` |

---

## Reaction Engineering API (19 functions)

### design (10 functions)

| Function | Signature |
|----------|-----------|
| `damkohler_number` | `(k: f64, tau: f64, c0: f64, order: f64) -> f64` |
| `selectivity` | `(r_desired: f64, r_undesired: f64) -> f64` |
| `yield_reactor` | `(moles_desired: f64, moles_reacted: f64) -> f64` |
| `overall_selectivity` | `(moles_desired: f64, moles_all_products: f64) -> f64` |
| `thiele_modulus` | `(r: f64, k: f64, d_eff: f64) -> f64` |
| `effectiveness_factor_sphere` | `(phi: f64) -> f64` |
| `weisz_prater_criterion` | `(r_obs: f64, r_particle: f64, d_eff: f64, c_s: f64) -> f64` |
| `residence_time_distribution_cstr` | `(t: f64, tau: f64) -> f64` |
| `residence_time_distribution_pfr` | `(t: f64, tau: f64) -> f64` |
| `recycle_ratio_effect` | `(x_single: f64, recycle_ratio: f64) -> f64` |

### reactors (9 functions)

| Function | Signature |
|----------|-----------|
| `cstr_volume` | `(f_a0: f64, x: f64, r_a: f64) -> f64` |
| `pfr_conversion_first_order` | `(k: f64, tau: f64) -> f64` |
| `cstr_conversion_first_order` | `(k: f64, tau: f64) -> f64` |
| `batch_time_first_order` | `(k: f64, x: f64) -> f64` |
| `batch_time_second_order` | `(k: f64, c0: f64, x: f64) -> f64` |
| `space_time` | `(volume: f64, volumetric_flow: f64) -> f64` |
| `space_velocity` | `(volumetric_flow: f64, volume: f64) -> f64` |
| `levenspiel_plot_area` | `(fa0_over_ra: &[f64], dx: f64) -> f64` |
| `cstr_series_conversion` | `(k: f64, tau_each: f64, n: u32) -> f64` |

---

## Solid State API (17 functions)

### band_theory (9 functions)

| Function | Signature |
|----------|-----------|
| `fermi_dirac` | `(energy: f64, fermi_level: f64, t: f64) -> f64` |
| `band_gap_from_absorption` | `(wavelength_edge_nm: f64) -> f64` |
| `intrinsic_carrier_concentration` | `(nc: f64, nv: f64, eg: f64, t: f64) -> f64` |
| `conductivity_semiconductor` | `(n: f64, mu_e: f64, p: f64, mu_h: f64) -> f64` |
| `hall_coefficient` | `(n: f64, p: f64) -> f64` |
| `doping_carrier_concentration` | `(nd: f64, ni: f64) -> (f64, f64)` |
| `resistivity_from_conductivity` | `(sigma: f64) -> f64` |
| `seebeck_coefficient` | `(delta_v: f64, delta_t: f64) -> f64` |
| `ionic_conductivity_arrhenius` | `(sigma0: f64, ea: f64, t: f64) -> f64` |

### defects (8 functions)

| Function | Signature |
|----------|-----------|
| `schottky_defect_concentration` | `(n_sites: f64, e_formation: f64, t: f64) -> f64` |
| `frenkel_defect_concentration` | `(n_sites: f64, n_interstitial: f64, e_formation: f64, t: f64) -> f64` |
| `vacancy_diffusion_coefficient` | `(d0: f64, q: f64, t: f64) -> f64` |
| `color_center_absorption` | `(wavelength_nm: f64) -> f64` |
| `kroger_vink_equilibrium` | `(k: f64, oxygen_pressure: f64, exponent: f64) -> f64` |
| `nonstoichiometry_delta` | `(mass_change: f64, molar_mass_oxygen: f64, sample_mass: f64, molar_mass_sample: f64) -> f64` |
| `defect_formation_volume` | `(lattice_param_defect: f64, lattice_param_perfect: f64) -> f64` |
| `burgers_vector_magnitude` | `(a: f64, h: i32, k: i32, l: i32) -> f64` |

---

## Solutions API (14 functions)

### colligative (6 functions)

| Function | Signature |
|----------|-----------|
| `boiling_point_elevation` | `(kb: f64, molality: f64, vant_hoff_factor: f64) -> f64` |
| `freezing_point_depression` | `(kf: f64, molality: f64, vant_hoff_factor: f64) -> f64` |
| `osmotic_pressure` | `(molarity: f64, temperature: f64, vant_hoff_factor: f64) -> f64` |
| `vapor_pressure_lowering` | `(x_solvent: f64, p0_solvent: f64) -> f64` |
| `molar_mass_from_ebullioscopy` | `(kb: f64, mass_solute: f64, mass_solvent_kg: f64, delta_t: f64) -> f64` |
| `molar_mass_from_cryoscopy` | `(kf: f64, mass_solute: f64, mass_solvent_kg: f64, delta_t: f64) -> f64` |

### mixtures (8 functions)

| Function | Signature |
|----------|-----------|
| `raoult_law` | `(x_solvent: f64, p0_solvent: f64) -> f64` |
| `henrys_law` | `(kh: f64, partial_pressure: f64) -> f64` |
| `mole_fraction` | `(moles_component: f64, total_moles: f64) -> f64` |
| `molality` | `(moles_solute: f64, mass_solvent_kg: f64) -> f64` |
| `molarity` | `(moles_solute: f64, volume_liters: f64) -> f64` |
| `gibbs_duhem_check` | `(x1: f64, d_mu1: f64, x2: f64, d_mu2: f64) -> f64` |
| `activity_from_mole_fraction` | `(gamma: f64, x: f64) -> f64` |
| `margules_one_suffix` | `(a: f64, x1: f64) -> f64` |

---

## Spectroscopy API (20 functions)

### ir (6 functions)

| Function | Signature |
|----------|-----------|
| `wavenumber_to_wavelength` | `(wavenumber_cm: f64) -> f64` |
| `wavelength_to_wavenumber` | `(wavelength_um: f64) -> f64` |
| `wavenumber_to_frequency` | `(wavenumber_cm: f64) -> f64` |
| `force_constant_from_frequency` | `(wavenumber: f64, reduced_mass_amu: f64) -> f64` |
| `reduced_mass` | `(m1: f64, m2: f64) -> f64` |
| `ir_intensity_ratio` | `(absorbance_sample: f64, absorbance_reference: f64) -> f64` |

### mass_spec (6 functions)

| Function | Signature |
|----------|-----------|
| `mass_to_charge` | `(mass: f64, charge: u32) -> f64` |
| `exact_mass_difference` | `(theoretical: f64, experimental: f64) -> f64` |
| `nitrogen_rule` | `(nominal_mass: u32) -> bool` |
| `rings_plus_double_bonds` | `(c: u32, h: u32, n: u32, halogens: u32) -> f64` |
| `isotope_pattern_monoisotopic` | `(abundances: &[f64]) -> f64` |
| `resolving_power` | `(m: f64, delta_m: f64) -> f64` |

### nmr (8 functions)

| Function | Signature |
|----------|-----------|
| `chemical_shift_ppm` | `(frequency_sample: f64, frequency_reference: f64, spectrometer: f64) -> f64` |
| `coupling_constant_first_order` | `(line_separation_hz: f64) -> f64` |
| `multiplicity` | `(n_neighbors: u32) -> u32` |
| `larmor_frequency` | `(gamma: f64, b0: f64) -> f64` |
| `t1_inversion_recovery` | `(m0: f64, t1: f64, tau: f64) -> f64` |
| `t2_spin_echo` | `(m0: f64, t2: f64, tau: f64) -> f64` |
| `linewidth_from_t2` | `(t2: f64) -> f64` |
| `nuclear_overhauser_enhancement` | `(gamma_irradiated: f64, gamma_observed: f64) -> f64` |

---

## Stoichiometry API (17 functions)

### balancing (7 functions)

| Function | Signature |
|----------|-----------|
| `limiting_reagent` | `(moles: &[f64], coefficients: &[f64]) -> usize` |
| `theoretical_yield` | `(moles_limiting: f64, coeff_limiting: f64, coeff_product: f64, mw_product: f64) -> f64` |
| `percent_yield` | `(actual: f64, theoretical: f64) -> f64` |
| `excess_reagent` | `(moles_a: f64, coeff_a: f64, moles_b: f64, coeff_b: f64) -> f64` |
| `moles_from_mass` | `(mass: f64, molar_mass: f64) -> f64` |
| `mass_from_moles` | `(moles: f64, molar_mass: f64) -> f64` |
| `number_of_particles` | `(moles: f64) -> f64` |

### calculations (10 functions)

| Function | Signature |
|----------|-----------|
| `oxidation_number_simple` | `(charge: i32, num_atoms: u32) -> f64` |
| `equivalent_mass` | `(molar_mass: f64, n_equivalents: f64) -> f64` |
| `normality` | `(equivalents: f64, volume_liters: f64) -> f64` |
| `atom_economy` | `(mw_desired_product: f64, mw_all_products: &[f64]) -> f64` |
| `empirical_formula_ratio` | `(masses: &[f64], molar_masses: &[f64]) -> Vec<f64>` |
| `dilution_factor` | `(v_initial: f64, v_final: f64) -> f64` |
| `stoichiometric_ratio` | `(coeff_a: f64, coeff_b: f64) -> f64` |
| `mass_percent` | `(mass_solute: f64, mass_solution: f64) -> f64` |
| `ppm_from_mass` | `(mass_solute: f64, mass_solution: f64) -> f64` |
| `molarity_to_molality` | `(molarity: f64, density: f64, molar_mass_solute: f64) -> f64` |

---

## Surface API (13 functions)

### adsorption (6 functions)

| Function | Signature |
|----------|-----------|
| `langmuir_isotherm` | `(theta_max: f64, k: f64, pressure: f64) -> f64` |
| `freundlich_isotherm` | `(kf: f64, pressure: f64, n: f64) -> f64` |
| `bet_isotherm` | `(v_mono: f64, c: f64, p: f64, p0: f64) -> f64` |
| `temkin_isotherm` | `(rt_over_b: f64, a: f64, pressure: f64) -> f64` |
| `langmuir_dissociative` | `(k: f64, pressure: f64) -> f64` |
| `bet_surface_area` | `(v_mono: f64, cross_section: f64, avogadro: f64, molar_volume: f64) -> f64` |

### tension (7 functions)

| Function | Signature |
|----------|-----------|
| `surface_tension_young` | `(gamma_sv: f64, gamma_sl: f64, cos_theta: f64) -> f64` |
| `contact_angle` | `(gamma_sv: f64, gamma_sl: f64, gamma_lv: f64) -> f64` |
| `capillary_rise` | `(gamma: f64, cos_theta: f64, rho: f64, g: f64, radius: f64) -> f64` |
| `laplace_pressure` | `(gamma: f64, r1: f64, r2: f64) -> f64` |
| `gibbs_adsorption` | `(d_gamma: f64, d_ln_concentration: f64, temperature: f64) -> f64` |
| `spreading_coefficient` | `(gamma_sv: f64, gamma_lv: f64, gamma_sl: f64) -> f64` |
| `work_of_adhesion` | `(gamma_lv: f64, cos_theta: f64) -> f64` |

---

## Thermochemistry API (21 functions)

### enthalpy (11 functions)

| Function | Signature |
|----------|-----------|
| `hess_law` | `(enthalpies: &[f64], coefficients: &[f64]) -> f64` |
| `bond_enthalpy` | `(bonds_broken: &[f64], bonds_formed: &[f64]) -> f64` |
| `born_haber_lattice_energy` | `(sublimation: f64, ionization: f64, dissociation: f64, electron_affinity: f64, formation: f64) -> f64` |
| `calorimetry` | `(mass: f64, specific_heat: f64, delta_t: f64) -> f64` |
| `kirchhoff` | `(delta_h_t1: f64, delta_cp: f64, t1: f64, t2: f64) -> f64` |
| `heat_of_combustion` | `(n_co2: f64, n_h2o: f64, hf_compound: f64) -> f64` |
| `clausius_clapeyron` | `(p1: f64, delta_h_vap: f64, t1: f64, t2: f64) -> f64` |
| `heat_capacity_ratio` | `(cp: f64, cv: f64) -> f64` |
| `adiabatic_flame_temperature` | `(reactant_enthalpy: f64, product_enthalpy_base: f64, cp_products: f64, t_base: f64) -> f64` |
| `standard_enthalpy_of_reaction` | `(products_hf: &[f64], products_coeff: &[f64], reactants_hf: &[f64], reactants_coeff: &[f64]) -> f64` |
| `bomb_calorimeter` | `(delta_t: f64, c_cal: f64, mass_sample: f64) -> f64` |

### entropy (10 functions)

| Function | Signature |
|----------|-----------|
| `entropy_change` | `(q_rev: f64, t: f64) -> f64` |
| `gibbs_free_energy` | `(delta_h: f64, t: f64, delta_s: f64) -> f64` |
| `spontaneity_temperature` | `(delta_h: f64, delta_s: f64) -> f64` |
| `entropy_mixing_ideal` | `(mole_fractions: &[f64]) -> f64` |
| `gibbs_helmholtz` | `(delta_g1: f64, delta_h: f64, t1: f64, t2: f64) -> f64` |
| `trouton_rule_entropy` | `(delta_h_vap: f64, t_boil: f64) -> f64` |
| `standard_entropy_of_reaction` | `(products_s: &[f64], products_coeff: &[f64], reactants_s: &[f64], reactants_coeff: &[f64]) -> f64` |
| `entropy_phase_transition` | `(delta_h: f64, t_transition: f64) -> f64` |
| `debye_entropy` | `(t: f64, theta_d: f64) -> f64` |
| `helmholtz_free_energy` | `(u: f64, t: f64, s: f64) -> f64` |

---

## Transport API (17 functions)

### diffusion (7 functions)

| Function | Signature |
|----------|-----------|
| `fick_first_law` | `(d: f64, dc_dx: f64) -> f64` |
| `fick_second_law_solution` | `(c0: f64, cs: f64, x: f64, d: f64, t: f64) -> f64` |
| `diffusion_coefficient_stokes_einstein` | `(t: f64, viscosity: f64, r: f64) -> f64` |
| `wilke_chang` | `(t: f64, viscosity: f64, mw_solvent: f64, phi: f64, v_solute: f64) -> f64` |
| `knudsen_diffusivity` | `(r_pore: f64, t: f64, mw: f64) -> f64` |
| `effective_diffusivity` | `(d_bulk: f64, porosity: f64, tortuosity: f64) -> f64` |
| `diffusion_time_estimate` | `(length: f64, d: f64) -> f64` |

### mass_transfer (10 functions)

| Function | Signature |
|----------|-----------|
| `mass_transfer_coefficient_film` | `(d: f64, delta: f64) -> f64` |
| `mass_flux` | `(k: f64, c_bulk: f64, c_surface: f64) -> f64` |
| `sherwood_number` | `(k: f64, l: f64, d: f64) -> f64` |
| `schmidt_number` | `(viscosity: f64, density: f64, d: f64) -> f64` |
| `penetration_theory_coefficient` | `(d: f64, t_contact: f64) -> f64` |
| `surface_renewal_coefficient` | `(d: f64, s: f64) -> f64` |
| `two_film_theory_overall` | `(k_g: f64, k_l: f64, henry: f64) -> f64` |
| `mass_transfer_biot` | `(k_ext: f64, r: f64, d_eff: f64) -> f64` |
| `overall_mass_transfer_resistance` | `(resistances: &[f64]) -> f64` |
| `ntu_mass_transfer` | `(k: f64, a: f64, flow: f64) -> f64` |

---

## Hub dispatch mapping

All chemistry functions are wired through `src/hub/engine/dispatch/chemistry.rs`.

```rust
use sciforge::hub::prelude::*;

let exp = Experiment::new(DomainType::Chemistry, "rate_constant_arrhenius")
        .param("a", ParameterValue::Scalar(1e13))
        .param("ea", ParameterValue::Scalar(75000.0))
        .param("t", ParameterValue::Scalar(500.0));

let out = ExperimentRunner::new().run(&exp)?;
```
