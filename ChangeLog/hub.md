# Hub — ChangeLog

---

## v0.0.3

### Refactoring — `hub::engine::dispatch` — monolithic → per-domain (~128 files)

| Change | Detail |
|---|---|
| `dispatch/astronomy.rs` → `dispatch/astronomy/` | 8 files (celestial, cosmology, galactic, impacts, orbits, rotation, stellar, tides) |
| `dispatch/biology.rs` → `dispatch/biology/` | 46 files (1 per submodule) |
| `dispatch/chemistry.rs` → `dispatch/chemistry/` | 27 files (1 per submodule) |
| `dispatch/geology.rs` → `dispatch/geology/` | 8 files (dating, erosion, geomorphology, glaciology, hydrology, petrology, seismology, tectonics) |
| `dispatch/maths.rs` → `dispatch/maths/` | 19 files (1 per submodule + helpers + misc) |
| `dispatch/meteorology.rs` → `dispatch/meteorology/` | 7 files (atmosphere, clouds, dynamics, ocean, precipitation, radiation, storms, winds) |
| `dispatch/physics.rs` → `dispatch/physics/` | 13 files (1 per submodule) |
| `query.rs` → `query/` | 20 files (1 per domain + cross-domain queries) |
| Total | ~190 files (vs 61 in v0.0.2) |

### 1️⃣ Astrobiology — `hub::domain::cross_domain::astrobiology` — 11 pub fn

| Function | Signature | Description | Module |
|---|---|---|---|
| Equilibrium temperature | `planet_equilibrium_temperature(stellar_luminosity: f64, distance: f64, albedo: f64) → f64` | $T_{eq} = T_\star \sqrt{R_\star / 2d} \cdot (1-A)^{1/4}$ | `cross_domain::astrobiology` |
| HZ inner | `habitable_zone_inner(luminosity: f64) → f64` | Inner HZ boundary | `cross_domain::astrobiology` |
| HZ outer | `habitable_zone_outer(luminosity: f64) → f64` | Outer HZ boundary | `cross_domain::astrobiology` |
| Escape parameter | `atmospheric_escape_parameter(temperature: f64, planet_mass: f64, planet_radius: f64, molecular_mass: f64) → f64` | Jeans escape parameter | `cross_domain::astrobiology` |
| Tidal locking | `tidal_locking_timescale(planet_mass: f64, semi_major: f64, star_mass: f64, planet_radius: f64, tidal_quality: f64) → f64` | Tidal lock timescale | `cross_domain::astrobiology` |
| Mass loss | `energy_limited_mass_loss(xuv_flux: f64, efficiency: f64, planet_mass: f64, planet_radius: f64) → f64` | XUV-driven atmospheric loss | `cross_domain::astrobiology` |
| Biosignature column | `biosignature_column_density(mixing_ratio: f64, surface_pressure: f64, gravity: f64, mean_molecular_mass: f64) → f64` | Column density | `cross_domain::astrobiology` |
| UV flux | `uv_surface_flux(incident_flux: f64, optical_depth: f64) → f64` | Surface UV attenuation | `cross_domain::astrobiology` |
| Photosynthetic limit | `photosynthetic_flux_limit(photon_flux: f64, quantum_efficiency: f64, photon_energy: f64) → f64` | Photon flux limit | `cross_domain::astrobiology` |
| Drake equation | `drake_equation(rate_star_formation: f64, fraction_planets: f64, habitable_per_system: f64, fraction_life: f64, fraction_intelligence: f64, fraction_communication: f64, civilization_lifetime: f64) → f64` | $N = R_\star \cdot f_p \cdot n_e \cdot f_l \cdot f_i \cdot f_c \cdot L$ | `cross_domain::astrobiology` |
| Surface gravity | `surface_gravity(mass: f64, radius: f64) → f64` | $g = GM/R^2$ | `cross_domain::astrobiology` |

### 2️⃣ Astrochemistry — `hub::domain::cross_domain::astrochemistry` — 11 pub fn

| Function | Signature | Description | Module |
|---|---|---|---|
| Jeans mass | `jeans_mass(temperature: f64, number_density: f64, mean_molecular_weight: f64) → f64` | Gravitational collapse mass | `cross_domain::astrochemistry` |
| Jeans length | `jeans_length(temperature: f64, number_density: f64, mean_molecular_weight: f64) → f64` | Collapse length scale | `cross_domain::astrochemistry` |
| Free-fall time | `freefall_time(number_density: f64, mean_molecular_weight: f64) → f64` | $t_{ff} = \sqrt{3\pi/(32G\rho)}$ | `cross_domain::astrochemistry` |
| Thermal velocity | `cloud_thermal_velocity(temperature: f64, mean_molecular_weight: f64) → f64` | $v_{th} = \sqrt{kT/m}$ | `cross_domain::astrochemistry` |
| Bonnor–Ebert mass | `bonnor_ebert_mass(temperature: f64, external_pressure: f64, mean_molecular_weight: f64) → f64` | Critical isothermal mass | `cross_domain::astrochemistry` |
| Photodissociation | `photodissociation_rate(unshielded_rate: f64, uv_field_habing: f64, shielding_factor: f64, visual_extinction: f64) → f64` | UV photodissociation | `cross_domain::astrochemistry` |
| Thermal desorption | `thermal_desorption_rate(attempt_frequency: f64, binding_energy: f64, dust_temperature: f64) → f64` | Grain surface desorption | `cross_domain::astrochemistry` |
| H₂ formation | `h2_formation_rate_on_dust(sticking_coefficient: f64, grain_cross_section: f64, grain_density: f64, temperature: f64) → f64` | Dust-catalyzed H₂ | `cross_domain::astrochemistry` |
| Saha ionization | `saha_ionization_ratio(temperature: f64, electron_density: f64, ionization_energy: f64, partition_ratio: f64) → f64` | Ionization equilibrium | `cross_domain::astrochemistry` |
| Strömgren radius | `stroemgren_radius(ionizing_photon_rate: f64, hydrogen_density: f64, recombination_coeff: f64) → f64` | HII region size | `cross_domain::astrochemistry` |
| Dust temperature | `dust_equilibrium_temperature(luminosity: f64, distance: f64, absorption_efficiency: f64) → f64` | Radiative equilibrium | `cross_domain::astrochemistry` |

### 3️⃣ Astrophysics — `hub::domain::cross_domain::astrophysics` — 11 pub fn

| Function | Signature | Description | Module |
|---|---|---|---|
| Schwarzschild radius | `schwarzschild_radius(mass: f64) → f64` | $r_s = 2GM/c^2$ | `cross_domain::astrophysics` |
| Eddington luminosity | `eddington_luminosity(mass: f64) → f64` | $L_{Edd} = 4\pi GMm_pc/\sigma_T$ | `cross_domain::astrophysics` |
| Chandrasekhar mass | `chandrasekhar_mass(mu_e: f64) → f64` | White dwarf mass limit | `cross_domain::astrophysics` |
| Virial temperature | `virial_temperature(mass: f64, radius: f64, mean_molecular_weight: f64) → f64` | $T_{vir} = GMm/(3kR)$ | `cross_domain::astrophysics` |
| Bondi accretion | `bondi_accretion_rate(mass: f64, density: f64, sound_speed: f64) → f64` | Spherical accretion rate | `cross_domain::astrophysics` |
| Compton wavelength | `compton_wavelength(mass: f64) → f64` | $\lambda_C = h/(mc)$ | `cross_domain::astrophysics` |
| Gravitational redshift | `gravitational_redshift(mass: f64, radius: f64) → f64` | $z = (1 - 2GM/rc^2)^{-1/2} - 1$ | `cross_domain::astrophysics` |
| Synchrotron frequency | `synchrotron_critical_frequency(gamma_factor: f64, magnetic_field: f64) → f64` | Critical synchrotron ν | `cross_domain::astrophysics` |
| Photon sphere | `photon_sphere_radius(mass: f64) → f64` | $r_{ph} = 3GM/c^2$ | `cross_domain::astrophysics` |
| Hawking temperature | `hawking_temperature(mass: f64) → f64` | $T_H = \hbar c^3/(8\pi GMk_B)$ | `cross_domain::astrophysics` |
| Relativistic Doppler | `relativistic_doppler(frequency: f64, velocity: f64) → f64` | Doppler shift | `cross_domain::astrophysics` |

### 4️⃣ Atmospheric Chemistry — `hub::domain::cross_domain::atmospheric_chemistry` — 11 pub fn

| Function | Signature | Description | Module |
|---|---|---|---|
| Photolysis rate | `photolysis_rate(cross_section: f64, quantum_yield: f64, actinic_flux: f64) → f64` | $J = \sigma \phi F$ | `cross_domain::atmospheric_chemistry` |
| Arrhenius rate | `reaction_rate_arrhenius(prefactor: f64, activation_energy: f64, temperature: f64) → f64` | $k = A e^{-E_a/RT}$ | `cross_domain::atmospheric_chemistry` |
| Henry's law | `henry_law_concentration(henry_constant: f64, partial_pressure: f64) → f64` | Gas solubility | `cross_domain::atmospheric_chemistry` |
| Chemical lifetime | `chemical_lifetime(rate_constant: f64, co_reactant_density: f64) → f64` | $\tau = 1/(k \cdot n)$ | `cross_domain::atmospheric_chemistry` |
| Mixing ratio → density | `mixing_ratio_to_number_density(mixing_ratio: f64, pressure: f64, temperature: f64) → f64` | Number density conversion | `cross_domain::atmospheric_chemistry` |
| Deposition velocity | `deposition_velocity(aerodynamic_resistance: f64, surface_resistance: f64, gravitational_settling: f64) → f64` | Dry deposition | `cross_domain::atmospheric_chemistry` |
| AOD | `aerosol_optical_depth(extinction_coeff: f64, layer_thickness: f64) → f64` | $\tau = \beta \cdot \Delta z$ | `cross_domain::atmospheric_chemistry` |
| K(T) equilibrium | `equilibrium_constant_temperature(k_ref: f64, delta_h: f64, t_ref: f64, temperature: f64) → f64` | van 't Hoff equation | `cross_domain::atmospheric_chemistry` |
| Lindemann rate | `lindemann_rate(k0: f64, kinf: f64, m_density: f64) → f64` | Pressure-dependent rate | `cross_domain::atmospheric_chemistry` |
| Mean free path | `mean_free_path(temperature: f64, pressure: f64, collision_diameter: f64) → f64` | $\lambda = kT/(\sqrt{2}\pi d^2 P)$ | `cross_domain::atmospheric_chemistry` |
| Column density | `column_density(number_density: f64, path_length: f64) → f64` | $N = n \cdot L$ | `cross_domain::atmospheric_chemistry` |

### 5️⃣ Atmospheric Physics — `hub::domain::cross_domain::atmospheric_physics` — 11 pub fn

| Function | Signature | Description | Module |
|---|---|---|---|
| Planck radiance | `planck_radiance(wavelength: f64, temperature: f64) → f64` | $B_\lambda(T)$ spectral radiance | `cross_domain::atmospheric_physics` |
| Stefan–Boltzmann | `stefan_boltzmann_flux(temperature: f64) → f64` | $F = \sigma T^4$ | `cross_domain::atmospheric_physics` |
| Brightness temperature | `brightness_temperature(radiance: f64, wavelength: f64) → f64` | Inverse Planck | `cross_domain::atmospheric_physics` |
| Rayleigh cross-section | `rayleigh_scattering_cross_section(wavelength: f64, refractive_index: f64, number_density: f64) → f64` | $\sigma \propto \lambda^{-4}$ | `cross_domain::atmospheric_physics` |
| Optical depth | `optical_depth(cross_section: f64, number_density: f64, path_length: f64) → f64` | $\tau = \sigma n L$ | `cross_domain::atmospheric_physics` |
| Scale height | `atmospheric_scale_height(temperature: f64, mean_molecular_mass: f64, gravity: f64) → f64` | $H = kT/(mg)$ | `cross_domain::atmospheric_physics` |
| P(z) barometric | `pressure_at_altitude(surface_pressure: f64, scale_height: f64, altitude: f64) → f64` | $P = P_0 e^{-z/H}$ | `cross_domain::atmospheric_physics` |
| Dry adiabatic lapse | `dry_adiabatic_lapse_rate(gravity: f64, specific_heat: f64) → f64` | $\Gamma_d = g/c_p$ | `cross_domain::atmospheric_physics` |
| Wien peak | `wien_peak_wavelength(temperature: f64) → f64` | $\lambda_{max} = b/T$ | `cross_domain::atmospheric_physics` |
| Radiative transfer | `schwarzschild_radiative_transfer(source_function: f64, initial_radiance: f64, optical_depth: f64) → f64` | Schwarzschild equation | `cross_domain::atmospheric_physics` |
| Emission temperature | `effective_emission_temperature(outgoing_flux: f64, emissivity: f64) → f64` | $T_e = (F/\varepsilon\sigma)^{1/4}$ | `cross_domain::atmospheric_physics` |

### 6️⃣ Biochemistry — `hub::domain::cross_domain::biochemistry` — 11 pub fn

| Function | Signature | Description | Module |
|---|---|---|---|
| Michaelis–Menten | `michaelis_menten_rate(vmax: f64, substrate: f64, km: f64) → f64` | $v = V_{max}[S]/(K_m+[S])$ | `cross_domain::biochemistry` |
| Henderson–Hasselbalch | `henderson_hasselbalch(pka: f64, base_conc: f64, acid_conc: f64) → f64` | $\text{pH} = pK_a + \log([A^-]/[HA])$ | `cross_domain::biochemistry` |
| Gibbs free energy | `gibbs_free_energy(delta_h: f64, delta_s: f64, temperature: f64) → f64` | $\Delta G = \Delta H - T\Delta S$ | `cross_domain::biochemistry` |
| Nernst potential | `nernst_potential(z: f64, c_out: f64, c_in: f64, temperature: f64) → f64` | $E = (RT/zF)\ln(C_{out}/C_{in})$ | `cross_domain::biochemistry` |
| Turnover number | `enzyme_turnover_number(vmax: f64, enzyme_conc: f64) → f64` | $k_{cat} = V_{max}/[E]_0$ | `cross_domain::biochemistry` |
| Competitive inhibition | `competitive_inhibition_rate(vmax: f64, substrate: f64, km: f64, inhibitor: f64, ki: f64) → f64` | Inhibited Michaelis–Menten | `cross_domain::biochemistry` |
| Osmotic pressure | `osmotic_pressure(molarity: f64, temperature: f64, i_factor: f64) → f64` | $\Pi = iMRT$ | `cross_domain::biochemistry` |
| Arrhenius rate | `arrhenius_rate(prefactor: f64, activation_energy: f64, temperature: f64) → f64` | $k = A e^{-E_a/RT}$ | `cross_domain::biochemistry` |
| Binding free energy | `binding_free_energy(kd: f64, temperature: f64) → f64` | $\Delta G = RT\ln(K_d)$ | `cross_domain::biochemistry` |
| Hill equation | `cooperativity_hill(substrate: f64, k_half: f64, n_hill: f64) → f64` | $\theta = [S]^n/(K^n+[S]^n)$ | `cross_domain::biochemistry` |
| pH | `ph_from_concentration(h_concentration: f64) → f64` | $\text{pH} = -\log_{10}[H^+]$ | `cross_domain::biochemistry` |

### 7️⃣ Biomathematics — `hub::domain::cross_domain::biomathematics` — 11 pub fn

| Function | Signature | Description | Module |
|---|---|---|---|
| Logistic growth | `logistic_growth_rate(r: f64, carrying_capacity: f64, population: f64) → f64` | $dN/dt = rN(1-N/K)$ | `cross_domain::biomathematics` |
| Prey rate | `lotka_volterra_prey_rate(growth_rate: f64, predation_rate: f64, prey: f64, predator: f64) → f64` | $dx/dt = \alpha x - \beta xy$ | `cross_domain::biomathematics` |
| Predator rate | `lotka_volterra_predator_rate(conversion_rate: f64, death_rate: f64, prey: f64, predator: f64) → f64` | $dy/dt = \delta xy - \gamma y$ | `cross_domain::biomathematics` |
| SIR infection | `sir_infection_rate(beta: f64, susceptible: f64, infected: f64, total: f64) → f64` | $dI/dt = \beta SI/N$ | `cross_domain::biomathematics` |
| SIR recovery | `sir_recovery_rate(gamma: f64, infected: f64) → f64` | $dR/dt = \gamma I$ | `cross_domain::biomathematics` |
| R₀ | `basic_reproduction_number(beta: f64, gamma: f64) → f64` | $R_0 = \beta/\gamma$ | `cross_domain::biomathematics` |
| Shannon index | `shannon_diversity_index(proportions: &[f64]) → f64` | $H' = -\sum p_i \ln p_i$ | `cross_domain::biomathematics` |
| Simpson index | `simpson_diversity_index(proportions: &[f64]) → f64` | $D = 1 - \sum p_i^2$ | `cross_domain::biomathematics` |
| Molecular clock | `molecular_clock_distance(substitution_rate: f64, time: f64) → f64` | $d = \mu t$ | `cross_domain::biomathematics` |
| Coalescent time | `coalescent_expected_time(effective_population: f64, n_lineages: f64) → f64` | Expected TMRCA | `cross_domain::biomathematics` |
| Heterozygosity | `wright_fisher_heterozygosity(h0: f64, effective_population: f64, generations: f64) → f64` | $H_t = H_0(1-1/2N_e)^t$ | `cross_domain::biomathematics` |

### 8️⃣ Biophysics — `hub::domain::cross_domain::biophysics` — 11 pub fn

| Function | Signature | Description | Module |
|---|---|---|---|
| Stokes–Einstein | `diffusion_coefficient_stokes_einstein(temperature: f64, viscosity: f64, radius: f64) → f64` | $D = kT/(6\pi\eta r)$ | `cross_domain::biophysics` |
| Membrane capacitance | `membrane_capacitance(area: f64, thickness: f64, dielectric_constant: f64) → f64` | $C = \varepsilon_0 \varepsilon_r A/d$ | `cross_domain::biophysics` |
| Stokes drag | `stokes_drag_force(viscosity: f64, radius: f64, velocity: f64) → f64` | $F = 6\pi\eta r v$ | `cross_domain::biophysics` |
| Sedimentation | `sedimentation_coefficient(particle_mass: f64, solvent_density: f64, particle_density: f64, friction_coefficient: f64) → f64` | Svedberg coefficient | `cross_domain::biophysics` |
| Thermal fluctuation | `thermal_fluctuation_amplitude(temperature: f64, spring_constant: f64) → f64` | $\langle x^2\rangle = kT/\kappa$ | `cross_domain::biophysics` |
| WLC extension | `worm_like_chain_extension(force: f64, contour_length: f64, persistence_length: f64, temperature: f64) → f64` | Worm-like chain model | `cross_domain::biophysics` |
| Reynolds number | `reynolds_number(density: f64, velocity: f64, length: f64, viscosity: f64) → f64` | $Re = \rho v L/\mu$ | `cross_domain::biophysics` |
| Fick diffusion | `fick_diffusion_flux(diffusion_coeff: f64, concentration_gradient: f64) → f64` | $J = -D \nabla c$ | `cross_domain::biophysics` |
| Debye length | `debye_screening_length(temperature: f64, ionic_strength: f64, dielectric_constant: f64) → f64` | $\lambda_D = \sqrt{\varepsilon kT/(2N_A e^2 I)}$ | `cross_domain::biophysics` |
| Electrophoretic mobility | `electrophoretic_mobility(charge: f64, friction_coefficient: f64) → f64` | $\mu_e = q/f$ | `cross_domain::biophysics` |
| Helfrich bending | `helfrich_bending_energy(bending_modulus: f64, mean_curvature: f64, spontaneous_curvature: f64, area: f64) → f64` | Membrane bending energy | `cross_domain::biophysics` |

### 9️⃣ Geochemistry — `hub::domain::cross_domain::geochemistry` — 11 pub fn

| Function | Signature | Description | Module |
|---|---|---|---|
| Partition coefficient | `partition_coefficient(c_solid: f64, c_liquid: f64) → f64` | $K_d = C_s/C_l$ | `cross_domain::geochemistry` |
| Batch melting | `batch_melting(c0: f64, melt_fraction: f64, bulk_d: f64) → f64` | Equilibrium melting | `cross_domain::geochemistry` |
| Fractional crystallization | `fractional_crystallization(c0: f64, fraction_remaining: f64, bulk_d: f64) → f64` | Rayleigh fractionation | `cross_domain::geochemistry` |
| Delta notation | `delta_notation(r_sample: f64, r_standard: f64) → f64` | $\delta = (R_s/R_{std}-1) \times 1000$ | `cross_domain::geochemistry` |
| Rayleigh fractionation | `rayleigh_fractionation(r0: f64, fraction_remaining: f64, alpha: f64) → f64` | $R = R_0 f^{\alpha-1}$ | `cross_domain::geochemistry` |
| Weathering rate | `weathering_rate(rate_constant: f64, surface_area: f64, saturation_state: f64) → f64` | Chemical weathering | `cross_domain::geochemistry` |
| Debye–Hückel | `activity_coefficient_debye_huckel(z: f64, ionic_strength: f64) → f64` | Activity coefficient | `cross_domain::geochemistry` |
| K_sp(T) | `solubility_product_temperature(ksp0: f64, delta_h: f64, t: f64, t0: f64) → f64` | Solubility vs temperature | `cross_domain::geochemistry` |
| Eh–pH boundary | `eh_ph_boundary(e0: f64, n_electrons: f64, n_protons: f64, ph: f64, temperature: f64) → f64` | Pourbaix diagram | `cross_domain::geochemistry` |
| Distribution Kd | `distribution_coefficient(c_adsorbed: f64, c_solution: f64) → f64` | Sorption coefficient | `cross_domain::geochemistry` |
| Two-component mixing | `mixing_two_component(c1: f64, c2: f64, fraction_1: f64) → f64` | $C = f C_1 + (1-f) C_2$ | `cross_domain::geochemistry` |

### 🔟 Geophysics — `hub::domain::cross_domain::geophysics` — 11 pub fn

| Function | Signature | Description | Module |
|---|---|---|---|
| Radiogenic heat | `radiogenic_heat_production(c_u238: f64, c_th232: f64, c_k40: f64, density: f64) → f64` | Radioactive heat production | `cross_domain::geophysics` |
| Bouguer anomaly | `bouguer_anomaly(observed_gravity: f64, reference_gravity: f64, elevation: f64, slab_density: f64) → f64` | Bouguer gravity correction | `cross_domain::geophysics` |
| Buried sphere | `gravity_anomaly_buried_sphere(delta_rho: f64, radius: f64, center_depth: f64, horizontal_distance: f64) → f64` | Sphere gravity anomaly | `cross_domain::geophysics` |
| Magnetic dipole | `magnetic_anomaly_vertical_dipole(moment: f64, depth: f64, horizontal_distance: f64) → f64` | Vertical dipole anomaly | `cross_domain::geophysics` |
| Seismic attenuation | `seismic_wave_attenuation(amplitude_0: f64, frequency: f64, travel_time: f64, quality_factor: f64) → f64` | $A = A_0 e^{-\pi ft/Q}$ | `cross_domain::geophysics` |
| Impedance reflection | `seismic_impedance_reflection(rho1: f64, v1: f64, rho2: f64, v2: f64) → f64` | Reflection coefficient | `cross_domain::geophysics` |
| Flexure wavelength | `lithospheric_flexure_wavelength(flexural_rigidity: f64, mantle_density: f64, infill_density: f64, g_surface: f64) → f64` | Flexural wavelength | `cross_domain::geophysics` |
| Rebound timescale | `isostatic_rebound_timescale(viscosity: f64, mantle_density: f64, g_surface: f64, wavelength: f64) → f64` | Glacial isostatic rebound | `cross_domain::geophysics` |
| Curie depth | `curie_depth(surface_heat_flow: f64, thermal_conductivity: f64, curie_temperature: f64, surface_temperature: f64) → f64` | Magnetic Curie isotherm | `cross_domain::geophysics` |
| Radiative conductivity | `radiative_thermal_conductivity(temperature: f64, refractive_index: f64, absorption_coefficient: f64) → f64` | Mantle radiative k | `cross_domain::geophysics` |
| EM skin depth | `electromagnetic_skin_depth(resistivity: f64, frequency: f64) → f64` | $\delta = \sqrt{2\rho/\omega\mu}$ | `cross_domain::geophysics` |

### 11. Mathematical Physics — `hub::domain::cross_domain::mathematical_physics` — 11 pub fn

| Function | Signature | Description | Module |
|---|---|---|---|
| de Broglie | `de_broglie_wavelength(momentum: f64) → f64` | $\lambda = h/p$ | `cross_domain::mathematical_physics` |
| ΔxΔp (position) | `heisenberg_uncertainty_position(delta_p: f64) → f64` | $\Delta x \geq \hbar/(2\Delta p)$ | `cross_domain::mathematical_physics` |
| ΔxΔp (momentum) | `heisenberg_uncertainty_momentum(delta_x: f64) → f64` | $\Delta p \geq \hbar/(2\Delta x)$ | `cross_domain::mathematical_physics` |
| WKB tunneling | `wkb_tunneling_probability(energy: f64, potential: f64, barrier_width: f64, particle_mass: f64) → f64` | Tunneling probability | `cross_domain::mathematical_physics` |
| Harmonic Z | `partition_function_harmonic(omega: f64, temperature: f64) → f64` | QHO partition function | `cross_domain::mathematical_physics` |
| Fermi–Dirac | `fermi_dirac_distribution(energy: f64, chemical_potential: f64, temperature: f64) → f64` | $f(E) = 1/(e^{(E-\mu)/kT}+1)$ | `cross_domain::mathematical_physics` |
| Bose–Einstein | `bose_einstein_distribution(energy: f64, chemical_potential: f64, temperature: f64) → f64` | $n(E) = 1/(e^{(E-\mu)/kT}-1)$ | `cross_domain::mathematical_physics` |
| DOS 3D free | `density_of_states_3d_free(energy: f64, volume: f64, mass: f64) → f64` | 3D free-particle DOS | `cross_domain::mathematical_physics` |
| Fourier mode | `fourier_mode_frequency(mode_number: f64, length: f64, wave_speed: f64) → f64` | $f_n = nv/(2L)$ | `cross_domain::mathematical_physics` |
| Relativistic energy | `relativistic_energy(rest_mass: f64, momentum: f64) → f64` | $E = \sqrt{(pc)^2+(mc^2)^2}$ | `cross_domain::mathematical_physics` |
| Thermal wavelength | `thermal_wavelength(mass: f64, temperature: f64) → f64` | $\Lambda = h/\sqrt{2\pi mkT}$ | `cross_domain::mathematical_physics` |

### 12. Planetary Geology — `hub::domain::cross_domain::planetary_geology` — 11 pub fn

| Function | Signature | Description | Module |
|---|---|---|---|
| Impact energy | `impact_energy(projectile_mass: f64, impact_velocity: f64) → f64` | $E = \frac{1}{2}mv^2$ | `cross_domain::planetary_geology` |
| Crater diameter | `crater_scaling_diameter(projectile_diameter: f64, projectile_density: f64, target_density: f64, velocity: f64, gravity: f64) → f64` | Pi-scaling law | `cross_domain::planetary_geology` |
| Tidal heating | `tidal_heating_rate(eccentricity: f64, mean_motion: f64, planet_radius: f64, quality_param: f64, primary_mass: f64, semi_major: f64) → f64` | Tidal dissipation rate | `cross_domain::planetary_geology` |
| Equilibrium Tₛ | `surface_temperature_equilibrium(solar_flux: f64, albedo: f64, emissivity: f64) → f64` | $T = (F(1-A)/\varepsilon\sigma)^{1/4}$ | `cross_domain::planetary_geology` |
| Lava cooling | `lava_flow_cooling_time(thickness: f64, thermal_diffusivity: f64) → f64` | Conductive cooling | `cross_domain::planetary_geology` |
| Regolith depth | `regolith_depth(flux_rate: f64, surface_density: f64, time: f64) → f64` | Impact gardening | `cross_domain::planetary_geology` |
| Lithosphere thickness | `lithospheric_thickness(thermal_conductivity: f64, heat_flow: f64, base_temp: f64, surface_temp: f64) → f64` | Thermal lithosphere | `cross_domain::planetary_geology` |
| Differentiation time | `gravitational_differentiation_time(radius: f64, density_diff: f64, gravity: f64, viscosity: f64) → f64` | Core–mantle separation | `cross_domain::planetary_geology` |
| Crater age | `crater_counting_surface_age(crater_density: f64, production_rate: f64) → f64` | Chronostratigraphy | `cross_domain::planetary_geology` |
| Effusion rate | `volcanic_effusion_rate(thermal_flux: f64, specific_heat: f64, delta_t: f64, latent_heat: f64) → f64` | Volcanic output rate | `cross_domain::planetary_geology` |
| Ejecta thickness | `ejecta_blanket_thickness(crater_radius: f64, distance_from_center: f64, rim_thickness: f64) → f64` | Ejecta decay | `cross_domain::planetary_geology` |

No new functions in api, engine core, tools — see `testing.md` for test details.

---

## v0.0.2

### Changes — `hub` — 48 files modified + 6 new files

| Change | Detail |
|---|---|
| Renamed | `domain/math` → `domain/maths` |
| New | `engine/query.rs`, `tools/arena.rs`, `tools/deterministic.rs`, `tools/profiler.rs`, `tools/validation.rs`, `tools/visualization.rs` |
| Modified | 48 existing files |
| Total | 61 files (vs 54 in v0.0.1) |

### 1️⃣ Query — `hub::engine::query` — 12 pub fn + 2 pub struct

| Item | Declaration | Description | Module |
|---|---|---|---|
| `FunctionInfo` | `pub struct` | `domain: DomainType`, `name: String`, `param_names: Vec<String>`, `description: String` | `engine::query` |
| `Catalog` | `pub struct` | Function registry | `engine::query` |

| Function | Signature | Description | Module |
|---|---|---|---|
| New | `Catalog::new() → Self` | Build full catalog | `engine::query` |
| Length | `Catalog::len() → usize` | Entry count | `engine::query` |
| Is empty | `Catalog::is_empty() → bool` | Empty check | `engine::query` |
| By domain | `Catalog::by_domain(&self, domain: &DomainType) → Vec<&FunctionInfo>` | Filter by domain | `engine::query` |
| Search | `Catalog::search(&self, pattern: &str) → Vec<&FunctionInfo>` | Pattern search | `engine::query` |
| Get | `Catalog::get(&self, name: &str) → Option<&FunctionInfo>` | Lookup by name | `engine::query` |
| Names | `Catalog::names(&self) → Vec<&str>` | All function names | `engine::query` |
| Domain summary | `Catalog::domain_summary(&self) → Vec<(String, usize)>` | Count per domain | `engine::query` |
| By param | `Catalog::by_param(&self, param: &str) → Vec<&FunctionInfo>` | Filter by parameter name | `engine::query` |
| By param count | `Catalog::by_param_count(&self, n: usize) → Vec<&FunctionInfo>` | Filter by param count | `engine::query` |
| To Markdown | `Catalog::to_markdown(&self) → String` | Markdown table export | `engine::query` |
| Register | `reg(entries: &mut Vec<FunctionInfo>, domain: DomainType, name: &str, params: &[&str], desc: &str)` | Register entry | `engine::query` |

### 2️⃣ Arena — `hub::tools::arena` — 18 pub fn + 4 pub struct

| Item | Declaration | Description | Module |
|---|---|---|---|
| `Arena` | `pub struct` | Bump allocator for f64 slices | `tools::arena` |
| `ArenaSlice` | `pub struct` | Handle to arena-allocated region | `tools::arena` |
| `ArenaMatrix` | `pub struct` | 2D matrix on arena (`rows`, `cols`) | `tools::arena` |
| `ScratchPool` | `pub struct` | Pool of fixed-size scratch buffers | `tools::arena` |

| Function | Signature | Description | Module |
|---|---|---|---|
| New arena | `Arena::new(capacity: usize) → Self` | Allocate backing buffer | `tools::arena` |
| Alloc slice | `Arena::alloc_slice(&self, n: usize) → Option<ArenaSlice>` | Reserve n f64s | `tools::arena` |
| Write | `Arena::write(&self, slice: &ArenaSlice, data: &[f64])` | Copy data into slice | `tools::arena` |
| Read | `Arena::read(&self, slice: &ArenaSlice) → Vec<f64>` | Copy data out | `tools::arena` |
| Get | `Arena::get(&self, slice: &ArenaSlice, index: usize) → Option<f64>` | Element access | `tools::arena` |
| Set | `Arena::set(&self, slice: &ArenaSlice, index: usize, value: f64)` | Element write | `tools::arena` |
| Reset | `Arena::reset(&self)` | Free all allocations | `tools::arena` |
| Used | `Arena::used(&self) → usize` | Bytes used | `tools::arena` |
| Remaining | `Arena::remaining(&self) → usize` | Bytes free | `tools::arena` |
| Capacity | `Arena::capacity(&self) → usize` | Total capacity | `tools::arena` |
| Slice len | `ArenaSlice::len(&self) → usize` | Element count | `tools::arena` |
| Slice empty | `ArenaSlice::is_empty(&self) → bool` | Empty check | `tools::arena` |
| New matrix | `ArenaMatrix::new(arena: &Arena, rows: usize, cols: usize) → Option<Self>` | Allocate 2D | `tools::arena` |
| Matrix get | `ArenaMatrix::get(&self, arena: &Arena, r: usize, c: usize) → Option<f64>` | Element access | `tools::arena` |
| Matrix set | `ArenaMatrix::set(&self, arena: &Arena, r: usize, c: usize, value: f64)` | Element write | `tools::arena` |
| Matrix to vec | `ArenaMatrix::to_vec(&self, arena: &Arena) → Vec<Vec<f64>>` | Export 2D | `tools::arena` |
| Pool get | `ScratchPool::get(&self, index: usize) → Option<ArenaSlice>` | Get scratch buffer | `tools::arena` |
| Pool write | `ScratchPool::write(&self, index: usize, data: &[f64])` | Write scratch | `tools::arena` |

### 3️⃣ Deterministic — `hub::tools::deterministic` — 17 pub fn + 2 pub struct

| Item | Declaration | Description | Module |
|---|---|---|---|
| `Rng` | `pub struct` | Xoshiro256** PRNG | `tools::deterministic` |
| `ReproducibleContext` | `pub struct` | `seed: u64`, `rng: Rng`, `audit_trail: Vec<String>` | `tools::deterministic` |

| Function | Signature | Description | Module |
|---|---|---|---|
| New RNG | `Rng::new(seed: u64) → Self` | Seeded PRNG | `tools::deterministic` |
| Next u64 | `Rng::next_u64(&mut self) → u64` | Random u64 | `tools::deterministic` |
| Next f64 | `Rng::next_f64(&mut self) → f64` | Random f64 [0, 1) | `tools::deterministic` |
| Uniform | `Rng::uniform(&mut self, lo: f64, hi: f64) → f64` | Uniform [lo, hi) | `tools::deterministic` |
| Next usize | `Rng::next_usize(&mut self, n: usize) → usize` | Random [0, n) | `tools::deterministic` |
| Normal | `Rng::normal(&mut self) → f64` | Standard normal | `tools::deterministic` |
| Normal params | `Rng::normal_params(&mut self, mu: f64, sigma: f64) → f64` | $\mathcal{N}(\mu, \sigma^2)$ | `tools::deterministic` |
| Fill uniform | `Rng::fill_uniform(&mut self, buf: &mut [f64])` | Fill buffer [0, 1) | `tools::deterministic` |
| Fill normal | `Rng::fill_normal(&mut self, buf: &mut [f64])` | Fill buffer N(0,1) | `tools::deterministic` |
| Shuffle | `Rng::shuffle<T>(&mut self, data: &mut [T])` | Fisher–Yates shuffle | `tools::deterministic` |
| Sample uniform | `Rng::sample_uniform(&mut self, n: usize, lo: f64, hi: f64) → Vec<f64>` | n uniform samples | `tools::deterministic` |
| Sample normal | `Rng::sample_normal(&mut self, n: usize, mu: f64, sigma: f64) → Vec<f64>` | n normal samples | `tools::deterministic` |
| Fingerprint | `fingerprint(data: &[f64]) → u64` | Deterministic hash | `tools::deterministic` |
| Fingerprint scalar | `fingerprint_scalar(v: f64) → u64` | Scalar hash | `tools::deterministic` |
| Match | `fingerprints_match(a: u64, b: u64) → bool` | Compare hashes | `tools::deterministic` |
| Assert reproducible | `assert_reproducible(computed: f64, expected: f64, tol: f64) → bool` | Tolerance check | `tools::deterministic` |
| Kahan sum | `kahan_sum(data: &[f64]) → f64` | Compensated summation | `tools::deterministic` |
| Kahan dot | `kahan_dot(a: &[f64], b: &[f64]) → f64` | Compensated dot product | `tools::deterministic` |
| New context | `ReproducibleContext::new(seed: u64) → Self` | Audited context | `tools::deterministic` |
| Log | `ReproducibleContext::log(&mut self, entry: &str)` | Audit trail entry | `tools::deterministic` |
| Reset | `ReproducibleContext::reset(&mut self)` | Reset state | `tools::deterministic` |
| Fork | `ReproducibleContext::fork(&self, sub_seed: u64) → Self` | Sub-context | `tools::deterministic` |
| Audit summary | `ReproducibleContext::audit_summary(&self) → String` | Trail summary | `tools::deterministic` |

### 4️⃣ Profiler — `hub::tools::profiler` — 9 pub fn + 2 pub struct + 1 pub const

| Item | Declaration | Description | Module |
|---|---|---|---|
| `ProfileEntry` | `pub struct` | `domain`, `function_name`, `iterations`, `total_ns`, `min_ns`, `max_ns`, `mean_ns`, `stddev_ns`, `median_ns` | `tools::profiler` |
| `ProfileReport` | `pub struct` | Batch profiling results | `tools::profiler` |
| `PROFILE_CSV_HEADER` | `pub const PROFILE_CSV_HEADER: &str` | CSV header row | `tools::profiler` |

| Function | Signature | Description | Module |
|---|---|---|---|
| Throughput | `ProfileEntry::throughput_per_sec(&self) → f64` | ops/sec | `tools::profiler` |
| CSV row | `ProfileEntry::to_csv_row(&self) → String` | Entry as CSV | `tools::profiler` |
| Profile experiment | `profile_experiment(experiment: &Experiment, iterations: u64) → HubResult<ProfileEntry>` | Single profile | `tools::profiler` |
| Profile batch | `profile_batch(experiments: &[Experiment], iterations: u64) → ProfileReport` | Batch profile | `tools::profiler` |
| Quick profile | `quick_profile(domain: DomainType, func: &str, params: Vec<(&str, f64)>, iterations: u64) → HubResult<ProfileEntry>` | Convenience profile | `tools::profiler` |
| Total time | `ProfileReport::total_time_ns(&self) → u128` | Sum of all runs | `tools::profiler` |
| Slowest | `ProfileReport::slowest(&self) → Option<&ProfileEntry>` | Highest mean_ns | `tools::profiler` |
| Fastest | `ProfileReport::fastest(&self) → Option<&ProfileEntry>` | Lowest mean_ns | `tools::profiler` |
| Report CSV | `ProfileReport::to_csv(&self) → String` | Full CSV report | `tools::profiler` |
| Report Markdown | `ProfileReport::to_markdown(&self) → String` | Full Markdown report | `tools::profiler` |
| Filter domain | `ProfileReport::filter_domain(&self, domain: &str) → Vec<&ProfileEntry>` | By domain | `tools::profiler` |
| Compare | `compare_entries(a: &ProfileEntry, b: &ProfileEntry) → f64` | Speedup ratio | `tools::profiler` |
| Format ns | `format_ns(ns: f64) → String` | Human-readable time | `tools::profiler` |
| Scalar value | `scalar_value(output: &RunOutput) → Option<f64>` | Extract scalar from RunOutput | `tools::profiler` |

### 5️⃣ Validation — `hub::tools::validation` — 8 pub fn + 5 pub struct

| Item | Declaration | Description | Module |
|---|---|---|---|
| `ValidationCase` | `pub struct` | `name`, `domain`, `function`, `params`, `expected`, `tolerance`, `source` | `tools::validation` |
| `ValidationResult` | `pub struct` | `name`, `passed`, `computed`, `expected`, `relative_error`, `tolerance`, `error_message` | `tools::validation` |
| `ValidationReport` | `pub struct` | Aggregated results | `tools::validation` |
| `ValidationThresholds` | `pub struct` | `max_failures`, `max_relative_error` | `tools::validation` |
| `PipelineOutcome` | `pub struct` | `passed`, `report`, `blocked_by_failures`, `blocked_by_error`, `worst_relative_error` | `tools::validation` |
| `ValidationPipeline` | `pub struct` | Threshold-gated pipeline | `tools::validation` |

| Function | Signature | Description | Module |
|---|---|---|---|
| New case | `ValidationCase::new(name: &str, domain: DomainType, function: &str, params: Vec<(&str, f64)>, expected: f64, tolerance: f64, source: &str) → Self` | Create validation case | `tools::validation` |
| Run validation | `run_validation(cases: &[ValidationCase]) → ValidationReport` | Run all cases | `tools::validation` |
| Passed | `ValidationReport::passed_count(&self) → usize` | Pass count | `tools::validation` |
| Failed | `ValidationReport::failed_count(&self) → usize` | Fail count | `tools::validation` |
| All passed | `ValidationReport::all_passed(&self) → bool` | All-pass check | `tools::validation` |
| Failures | `ValidationReport::failures(&self) → Vec<&ValidationResult>` | Failed entries | `tools::validation` |
| Worst error | `ValidationReport::worst_error(&self) → Option<&ValidationResult>` | Highest relative error | `tools::validation` |
| Report Markdown | `ValidationReport::to_markdown(&self) → String` | Markdown output | `tools::validation` |
| Report CSV | `ValidationReport::to_csv(&self) → String` | CSV output | `tools::validation` |
| NaN safety | `check_nan_safety(domain: DomainType, function: &str, params: Vec<(&str, f64)>) → bool` | NaN input check | `tools::validation` |
| Monotonicity | `check_monotonicity(domain: DomainType, function: &str, base_params: Vec<(&str, f64)>, vary_param: &str, values: &[f64], increasing: bool) → bool` | Monotonicity check | `tools::validation` |
| LaTeX | `report_to_latex(report: &ValidationReport) → String` | LaTeX table | `tools::validation` |
| TSV | `report_to_tsv(report: &ValidationReport) → String` | TSV output | `tools::validation` |
| Pipeline new | `ValidationPipeline::new(thresholds: ValidationThresholds) → Self` | Threshold-gated runner | `tools::validation` |
| Add case | `ValidationPipeline::add_case(self, case: ValidationCase) → Self` | Add single case | `tools::validation` |
| Add cases | `ValidationPipeline::add_cases(self, cases: Vec<ValidationCase>) → Self` | Add batch | `tools::validation` |
| Pipeline run | `ValidationPipeline::run(&self) → PipelineOutcome` | Execute pipeline | `tools::validation` |
| Case count | `ValidationPipeline::case_count(&self) → usize` | Number of cases | `tools::validation` |

### 6️⃣ Visualization — `hub::tools::visualization` — 5 pub fn + 2 pub struct

| Item | Declaration | Description | Module |
|---|---|---|---|
| `ChartConfig` | `pub struct` | `width`, `height`, `margin_top/right/bottom/left`, `title`, `x_label`, `y_label`, `background`, `grid`, `font_size` | `tools::visualization` |
| `Series` | `pub struct` | `name: String`, `x: Vec<f64>`, `y: Vec<f64>` | `tools::visualization` |

| Function | Signature | Description | Module |
|---|---|---|---|
| Line chart | `line_chart(series: &[Series], cfg: &ChartConfig) → String` | SVG line chart | `tools::visualization` |
| Scatter plot | `scatter_plot(x: &[f64], y: &[f64], cfg: &ChartConfig) → String` | SVG scatter plot | `tools::visualization` |
| Bar chart | `bar_chart(labels: &[&str], values: &[f64], cfg: &ChartConfig) → String` | SVG bar chart | `tools::visualization` |
| Histogram | `histogram(data: &[f64], bins: usize, cfg: &ChartConfig) → String` | SVG histogram | `tools::visualization` |
| Heatmap | `heatmap(matrix: &[Vec<f64>], cfg: &ChartConfig) → String` | SVG heatmap | `tools::visualization` |

---

## v0.0.1

### 1️⃣ CLI — `hub::api::cli` — 2 pub fn + 1 pub struct + 1 pub enum

| Item | Declaration | Description | Module |
|---|---|---|---|
| `Command` | `pub enum` | `Compute { domain, function, params }`, `Simulate { model, duration, dt }`, `ListDomains`, `ListFunctions { domain }`, `Serve { port }`, `Version`, `Help` | `api::cli` |
| `CliArgs` | `pub struct` | `command: Command`, `verbose: bool` | `api::cli` |

| Function | Signature | Description | Module |
|---|---|---|---|
| Parse | `parse(args: &[String]) → CliArgs` | Parse CLI arguments | `api::cli` |
| Usage | `usage() → &'static str` | Help text | `api::cli` |

### 2️⃣ DTO Request — `hub::api::dto::request` — 10 pub fn + 1 pub struct + 1 pub enum

| Item | Declaration | Description | Module |
|---|---|---|---|
| `ComputeRequest` | `pub struct` | `domain: String`, `function: String`, `params: HashMap<String, ParamValue>` | `api::dto::request` |
| `ParamValue` | `pub enum` | `Scalar(f64)`, `Integer(i64)`, `Text(String)`, `Boolean(bool)`, `Array(Vec<f64>)` | `api::dto::request` |

| Function | Signature | Description | Module |
|---|---|---|---|
| New | `ComputeRequest::new(domain: &str, function: &str) → Self` | Create request | `api::dto::request` |
| With scalar | `ComputeRequest::with_scalar(self, key: &str, value: f64) → Self` | Add scalar param | `api::dto::request` |
| With integer | `ComputeRequest::with_integer(self, key: &str, value: i64) → Self` | Add integer param | `api::dto::request` |
| With text | `ComputeRequest::with_text(self, key: &str, value: &str) → Self` | Add text param | `api::dto::request` |
| With bool | `ComputeRequest::with_bool(self, key: &str, value: bool) → Self` | Add bool param | `api::dto::request` |
| With array | `ComputeRequest::with_array(self, key: &str, value: Vec<f64>) → Self` | Add array param | `api::dto::request` |
| Get scalar | `ComputeRequest::get_scalar(&self, key: &str) → Option<f64>` | Extract scalar | `api::dto::request` |
| Get integer | `ComputeRequest::get_integer(&self, key: &str) → Option<i64>` | Extract integer | `api::dto::request` |
| Get array | `ComputeRequest::get_array(&self, key: &str) → Option<&[f64]>` | Extract array | `api::dto::request` |
| From JSON | `ComputeRequest::from_json_str(json: &str) → Option<Self>` | Parse from JSON | `api::dto::request` |

### 3️⃣ DTO Response — `hub::api::dto::response` — 6 pub fn + 1 pub struct + 1 pub enum

| Item | Declaration | Description | Module |
|---|---|---|---|
| `ComputeResponse` | `pub struct` | `success: bool`, `result: Option<ResultData>`, `error: Option<String>`, `elapsed_ms: f64` | `api::dto::response` |
| `ResultData` | `pub enum` | `Scalar(f64)`, `Pair(f64, f64)`, `Triple(f64, f64, f64)`, `Vector(Vec<f64>)`, `Matrix(Vec<Vec<f64>>)`, `TimeSeries { times, values }`, `Text(String)`, `Boolean(bool)` | `api::dto::response` |

| Function | Signature | Description | Module |
|---|---|---|---|
| OK scalar | `ComputeResponse::ok_scalar(value: f64, elapsed_ms: f64) → Self` | Success with scalar | `api::dto::response` |
| OK vector | `ComputeResponse::ok_vector(values: Vec<f64>, elapsed_ms: f64) → Self` | Success with vector | `api::dto::response` |
| OK time series | `ComputeResponse::ok_time_series(times: Vec<f64>, values: Vec<Vec<f64>>, elapsed_ms: f64) → Self` | Success with time series | `api::dto::response` |
| OK text | `ComputeResponse::ok_text(text: String, elapsed_ms: f64) → Self` | Success with text | `api::dto::response` |
| Fail | `ComputeResponse::fail(msg: &str, elapsed_ms: f64) → Self` | Error response | `api::dto::response` |
| To JSON | `ComputeResponse::to_json(&self) → String` | Serialize to JSON | `api::dto::response` |

### 4️⃣ HTTP — `hub::api::http` — 6 pub fn + 2 pub struct

| Item | Declaration | Description | Module |
|---|---|---|---|
| `HttpServer` | `pub struct` | TCP HTTP server | `api::http` |
| `HttpResponse` | `pub struct` | `status_code: u16`, `status_text: String`, `content_type: String`, `body: String` | `api::http` |

| Function | Signature | Description | Module |
|---|---|---|---|
| New server | `HttpServer::new(addr: &str) → Self` | Bind address | `api::http` |
| Serve | `HttpServer::bind_and_serve(&self, handler: &dyn Fn(&str, &str, &str) → HttpResponse) → std::io::Result<()>` | Accept connections | `api::http` |
| OK | `HttpResponse::ok(body: &str) → Self` | 200 OK | `api::http` |
| Bad request | `HttpResponse::bad_request(body: &str) → Self` | 400 Bad Request | `api::http` |
| Not found | `HttpResponse::not_found() → Self` | 404 Not Found | `api::http` |
| Internal error | `HttpResponse::internal_error(msg: &str) → Self` | 500 Internal Error | `api::http` |

### 5️⃣ Routes — `hub::api::routes` — 3 pub fn

| Function | Signature | Description | Module |
|---|---|---|---|
| Route | `route(domain: &str, function: &str, raw_params: &[&str]) → String` | Dispatch and format | `api::routes` |
| Format | `format(out: &RunOutput) → String` | Output formatter | `api::routes::formatter` |
| Parse params | `parse(raw: &[&str]) → Vec<(String, ParameterValue)>` | Raw string → params | `api::routes::params` |

### 6️⃣ Errors — `hub::domain::common::errors` — 1 pub enum + 1 pub type

| Item | Declaration | Description | Module |
|---|---|---|---|
| `HubError` | `pub enum` | `InvalidInput(String)`, `ComputationFailed(String)`, `OutOfRange { name, min, max, got }`, `DimensionMismatch { expected, got }`, `NotConverged { iterations }`, `EmptyData`, `Overflow`, `Underflow` | `domain::common::errors` |
| `HubResult<T>` | `pub type HubResult<T> = Result<T, HubError>` | Result alias | `domain::common::errors` |

### 7️⃣ Units — `hub::domain::common::units` — 21 pub fn + 7 pub enum

| Item | Declaration | Description | Module |
|---|---|---|---|
| `LengthUnit` | `pub enum` | `Meter`, `Kilometer`, `Centimeter`, `Millimeter`, `Micrometer`, `Nanometer`, `Angstrom`, `Mile`, `Yard`, `Foot`, `Inch`, `AstronomicalUnit`, `LightYear`, `Parsec` | `domain::common::units` |
| `MassUnit` | `pub enum` | `Kilogram`, `Gram`, `Milligram`, `Microgram`, `Tonne`, `Dalton`, `Pound`, `Ounce`, `SolarMass` | `domain::common::units` |
| `TimeUnit` | `pub enum` | `Second`, `Millisecond`, `Microsecond`, `Nanosecond`, `Minute`, `Hour`, `Day`, `Year` | `domain::common::units` |
| `TemperatureUnit` | `pub enum` | `Kelvin`, `Celsius`, `Fahrenheit`, `Rankine` | `domain::common::units` |
| `EnergyUnit` | `pub enum` | `Joule`, `Kilojoule`, `Calorie`, `Kilocalorie`, `ElectronVolt`, `Erg` | `domain::common::units` |
| `PressureUnit` | `pub enum` | `Pascal`, `Kilopascal`, `Megapascal`, `Bar`, `Atmosphere`, `Torr`, `Psi` | `domain::common::units` |
| `AngleUnit` | `pub enum` | `Radian`, `Degree`, `Arcminute`, `Arcsecond` | `domain::common::units` |

| Function | Signature | Description | Module |
|---|---|---|---|
| Length → SI | `length_to_si(value: f64, unit: LengthUnit) → f64` | Convert to meters | `domain::common::units` |
| SI → Length | `length_from_si(value: f64, unit: LengthUnit) → f64` | Convert from meters | `domain::common::units` |
| Mass → SI | `mass_to_si(value: f64, unit: MassUnit) → f64` | Convert to kg | `domain::common::units` |
| SI → Mass | `mass_from_si(value: f64, unit: MassUnit) → f64` | Convert from kg | `domain::common::units` |
| Time → SI | `time_to_si(value: f64, unit: TimeUnit) → f64` | Convert to seconds | `domain::common::units` |
| SI → Time | `time_from_si(value: f64, unit: TimeUnit) → f64` | Convert from seconds | `domain::common::units` |
| T → Kelvin | `temperature_to_kelvin(value: f64, unit: TemperatureUnit) → f64` | Convert to Kelvin | `domain::common::units` |
| Kelvin → T | `kelvin_to_temperature(value: f64, unit: TemperatureUnit) → f64` | Convert from Kelvin | `domain::common::units` |
| Energy → SI | `energy_to_si(value: f64, unit: EnergyUnit) → f64` | Convert to Joules | `domain::common::units` |
| SI → Energy | `energy_from_si(value: f64, unit: EnergyUnit) → f64` | Convert from Joules | `domain::common::units` |
| Pressure → SI | `pressure_to_si(value: f64, unit: PressureUnit) → f64` | Convert to Pascal | `domain::common::units` |
| SI → Pressure | `pressure_from_si(value: f64, unit: PressureUnit) → f64` | Convert from Pascal | `domain::common::units` |
| Angle → rad | `angle_to_radian(value: f64, unit: AngleUnit) → f64` | Convert to radians | `domain::common::units` |
| rad → Angle | `radian_to_angle(value: f64, unit: AngleUnit) → f64` | Convert from radians | `domain::common::units` |
| Convert length | `convert_length(value: f64, from: LengthUnit, to: LengthUnit) → f64` | Direct conversion | `domain::common::units` |
| Convert mass | `convert_mass(value: f64, from: MassUnit, to: MassUnit) → f64` | Direct conversion | `domain::common::units` |
| Convert time | `convert_time(value: f64, from: TimeUnit, to: TimeUnit) → f64` | Direct conversion | `domain::common::units` |
| Convert temperature | `convert_temperature(value: f64, from: TemperatureUnit, to: TemperatureUnit) → f64` | Direct conversion | `domain::common::units` |
| Convert energy | `convert_energy(value: f64, from: EnergyUnit, to: EnergyUnit) → f64` | Direct conversion | `domain::common::units` |
| Convert pressure | `convert_pressure(value: f64, from: PressureUnit, to: PressureUnit) → f64` | Direct conversion | `domain::common::units` |
| Convert angle | `convert_angle(value: f64, from: AngleUnit, to: AngleUnit) → f64` | Direct conversion | `domain::common::units` |

### 8️⃣ Dispatch — `hub::engine::dispatch` — 17 pub fn + 1 pub type

| Item | Declaration | Description | Module |
|---|---|---|---|
| `Params` | `pub type Params = Vec<(String, ParameterValue)>` | Parameter list alias | `engine::dispatch::params` |

| Function | Signature | Description | Module |
|---|---|---|---|
| Dispatch | `dispatch(experiment: &Experiment) → HubResult<RunOutput>` | Route to domain handler | `engine::dispatch` |
| Get f64 | `get_f(p: &Params, name: &str) → HubResult<f64>` | Extract f64 | `engine::dispatch::params` |
| Get i64 | `get_i(p: &Params, name: &str) → HubResult<i64>` | Extract i64 | `engine::dispatch::params` |
| Get usize | `get_u(p: &Params, name: &str) → HubResult<usize>` | Extract usize | `engine::dispatch::params` |
| Get vec | `get_v<'a>(p: &'a Params, name: &str) → HubResult<&'a [f64]>` | Extract &[f64] | `engine::dispatch::params` |
| Get matrix | `get_m<'a>(p: &'a Params, name: &str) → HubResult<&'a [Vec<f64>]>` | Extract matrix | `engine::dispatch::params` |
| Get bool | `get_b(p: &Params, name: &str) → HubResult<bool>` | Extract bool | `engine::dispatch::params` |
| Get str | `get_str<'a>(p: &'a Params, name: &str) → HubResult<&'a str>` | Extract &str | `engine::dispatch::params` |
| Get complex | `get_c(p: &Params, name: &str) → HubResult<Complex>` | Extract Complex | `engine::dispatch::params` |
| Get complex vec | `get_cv(p: &Params, name: &str) → HubResult<Vec<Complex>>` | Extract complex vector | `engine::dispatch::params` |
| Get polynomial | `get_poly<'a>(p: &'a Params, name: &str) → HubResult<&'a [f64]>` | Extract coefficients | `engine::dispatch::params` |
| Get usize vec | `get_uv<'a>(p: &'a Params, name: &str) → HubResult<&'a [usize]>` | Extract &[usize] | `engine::dispatch::params` |
| Get usize matrix | `get_um<'a>(p: &'a Params, name: &str) → HubResult<&'a [Vec<usize>]>` | Extract usize matrix | `engine::dispatch::params` |
| Get edges | `get_edges<'a>(p: &'a Params, name: &str) → HubResult<&'a [(usize, usize, f64)]>` | Extract edge list | `engine::dispatch::params` |
| Get sparse | `get_sparse<'a>(p: &'a Params, name: &str) → HubResult<(usize, usize, &'a [usize], &'a [usize], &'a [f64])>` | Extract CSR sparse | `engine::dispatch::params` |
| Get tensor | `get_tensor<'a>(p: &'a Params, name: &str) → HubResult<(&'a [f64], &'a [usize])>` | Extract tensor data+shape | `engine::dispatch::params` |

### 9️⃣ Experience — `hub::engine::experience` — 4 pub fn + 2 pub struct + 2 pub enum

| Item | Declaration | Description | Module |
|---|---|---|---|
| `DomainType` | `pub enum` | `Maths`, `Physics`, `Chemistry`, `Biology`, `Astronomy`, `Geology`, `Meteorology`, `Astrochemistry`, `Geophysics`, `Astrophysics`, `Biochemistry`, `Biophysics`, `Geochemistry`, `Astrobiology`, `AtmosphericChemistry`, `AtmosphericPhysics`, `PlanetaryGeology`, `Biomathematics`, `MathematicalPhysics` | `engine::experience::experiment` |
| `ParameterValue` | `pub enum` | `Scalar(f64)`, `Integer(i64)`, `Vector(Vec<f64>)`, `Matrix(Vec<Vec<f64>>)`, `Boolean(bool)`, `Text(String)`, `Complex(f64, f64)`, `ComplexVector`, `Polynomial(Vec<f64>)`, `IntVector`, `IntMatrix`, `EdgeList`, `Sparse { rows, cols, row_ptr, col_idx, values }`, `Tensor { data, shape }` | `engine::experience::experiment` |
| `Experiment` | `pub struct` | `domain: DomainType`, `function_name: String`, `parameters: Vec<(String, ParameterValue)>` | `engine::experience::experiment` |
| `RunOutput` | `pub enum` | `Scalar(f64)`, `Vector`, `Pair`, `Triple`, `Matrix`, `Boolean`, `Text`, `Complex`, `ComplexVector`, `ComplexMatrix`, `PolynomialOut`, `TensorOut`, `SparseOut`, `Integer`, `IntVector`, `PairVec`, `TimeSeries { times, values }` | `engine::experience::runner` |
| `ExperimentRunner` | `pub struct` | Stateless runner | `engine::experience::runner` |

| Function | Signature | Description | Module |
|---|---|---|---|
| New experiment | `Experiment::new(domain: DomainType, function_name: &str) → Self` | Create experiment | `engine::experience::experiment` |
| Add param | `Experiment::param(self, name: &str, value: ParameterValue) → Self` | Chain parameter | `engine::experience::experiment` |
| New runner | `ExperimentRunner::new() → Self` | Create runner | `engine::experience::runner` |
| Run | `ExperimentRunner::run(&self, experiment: &Experiment) → HubResult<RunOutput>` | Execute experiment | `engine::experience::runner` |

### 🔟 Campaign — `hub::engine::campaign` — 7 pub fn + 2 pub struct

| Item | Declaration | Description | Module |
|---|---|---|---|
| `Step` | `pub struct` | `name: String`, `experiment: Experiment` | `engine::campaign` |
| `CampaignResult` | `pub struct` | `step_results: Vec<(String, RunOutput)>` | `engine::campaign` |
| `Campaign` | `pub struct` | `name: String`, steps list | `engine::campaign` |

| Function | Signature | Description | Module |
|---|---|---|---|
| Get result | `CampaignResult::get(&self, name: &str) → Option<&RunOutput>` | Lookup by step name | `engine::campaign` |
| Length | `CampaignResult::len(&self) → usize` | Step count | `engine::campaign` |
| Is empty | `CampaignResult::is_empty(&self) → bool` | Empty check | `engine::campaign` |
| Scalars | `CampaignResult::scalars(&self) → Vec<(&str, f64)>` | All scalar results | `engine::campaign` |
| New campaign | `Campaign::new(name: &str) → Self` | Create campaign | `engine::campaign` |
| Add step | `Campaign::add_step(self, name: &str, experiment: Experiment) → Self` | Chain step | `engine::campaign` |
| Run | `Campaign::run(&self, runner: &ExperimentRunner) → HubResult<CampaignResult>` | Execute all steps | `engine::campaign` |

### 11. Pipeline — `hub::engine::pipeline` — 8 pub fn + 2 pub struct

| Item | Declaration | Description | Module |
|---|---|---|---|
| `Stage` | `pub struct` | `name: String`, `transform: Box<dyn Fn(Vec<f64>) → HubResult<Vec<f64>>>` | `engine::pipeline::flow` |
| `Pipeline` | `pub struct` | Chained transformation stages | `engine::pipeline::flow` |

| Function | Signature | Description | Module |
|---|---|---|---|
| New | `Pipeline::new() → Self` | Empty pipeline | `engine::pipeline::flow` |
| Add stage | `Pipeline::add_stage(self, name: &str, transform: Box<dyn Fn(Vec<f64>) → HubResult<Vec<f64>>>) → Self` | Append stage | `engine::pipeline::flow` |
| Execute | `Pipeline::execute(&self, input: Vec<f64>) → HubResult<Vec<f64>>` | Run all stages | `engine::pipeline::flow` |
| Stage count | `Pipeline::stage_count(&self) → usize` | Number of stages | `engine::pipeline::flow` |
| Stage names | `Pipeline::stage_names(&self) → Vec<&str>` | List stage names | `engine::pipeline::flow` |
| Normalize | `normalize_stage() → Box<dyn Fn(Vec<f64>) → HubResult<Vec<f64>>>` | Normalize [0,1] | `engine::pipeline::flow` |
| Scale | `scale_stage(factor: f64) → Box<dyn Fn(Vec<f64>) → HubResult<Vec<f64>>>` | Multiply by factor | `engine::pipeline::flow` |
| Filter positive | `filter_positive_stage() → Box<dyn Fn(Vec<f64>) → HubResult<Vec<f64>>>` | Keep positive values | `engine::pipeline::flow` |

### 12. Simulation — `hub::engine::simulation` — 5 pub fn + 6 pub struct + 1 pub enum + 1 pub trait + 1 pub type

| Item | Declaration | Description | Module |
|---|---|---|---|
| `IntegrationMethod` | `pub enum` | `Euler`, `Heun`, `RungeKutta4`, `Midpoint` | `engine::simulation::integrator` |
| `IntegratorConfig` | `pub struct` | `method`, `dt`, `max_steps`, `tolerance` | `engine::simulation::integrator` |
| `DynamicalSystem` | `pub trait` | `dimension()`, `derivatives(t, state, out)`, `initial_state()`, `time_span()` | `engine::simulation::model` |
| `SystemFn` | `pub type SystemFn = Box<dyn Fn(f64, &[f64], &mut [f64])>` | ODE function alias | `engine::simulation::model` |
| `SimpleModel` | `pub struct` | Generic ODE wrapper | `engine::simulation::model` |
| `HarmonicOscillator` | `pub struct` | `omega`, `x0`, `v0`, `t_span` | `engine::simulation::model` |
| `LotkaVolterra` | `pub struct` | `alpha`, `beta`, `delta`, `gamma`, `prey0`, `pred0`, `t_span` | `engine::simulation::model` |
| `SimulationResult` | `pub struct` | `times`, `states`, `dimension`, `method`, `steps_taken` | `engine::simulation::result` |
| `Solver` | `pub struct` | ODE solver wrapper | `engine::simulation::solver` |

| Function | Signature | Description | Module |
|---|---|---|---|
| New config | `IntegratorConfig::new(method: IntegrationMethod, dt: f64) → Self` | Create config | `engine::simulation::integrator` |
| Integrate | `integrate(config: &IntegratorConfig, system: &dyn DynamicalSystem, y0: &[f64], t0: f64, tf: f64) → HubResult<(Vec<f64>, Vec<Vec<f64>>)>` | Time integration | `engine::simulation::integrator` |
| New model | `SimpleModel::new(y0: Vec<f64>, t_span: (f64, f64), f: SystemFn) → Self` | Generic ODE system | `engine::simulation::model` |
| Final state | `SimulationResult::final_state(&self) → Option<&[f64]>` | Last state | `engine::simulation::result` |
| State at | `SimulationResult::state_at(&self, index: usize) → Option<&[f64]>` | State by index | `engine::simulation::result` |
| Time at | `SimulationResult::time_at(&self, index: usize) → Option<f64>` | Time by index | `engine::simulation::result` |
| Length | `SimulationResult::len(&self) → usize` | Step count | `engine::simulation::result` |
| Column | `SimulationResult::column(&self, dim_index: usize) → Vec<f64>` | Extract dimension | `engine::simulation::result` |
| Max | `SimulationResult::max_of(&self, dim_index: usize) → f64` | Max of dimension | `engine::simulation::result` |
| Min | `SimulationResult::min_of(&self, dim_index: usize) → f64` | Min of dimension | `engine::simulation::result` |
| New solver | `Solver::new(config: IntegratorConfig) → Self` | Wrap config | `engine::simulation::solver` |
| Solve | `Solver::solve(&self, system: &dyn DynamicalSystem) → HubResult<SimulationResult>` | Full solve | `engine::simulation::solver` |

### 13. Worker — `hub::engine::worker` — 14 pub fn + 6 pub struct + 2 pub enum

| Item | Declaration | Description | Module |
|---|---|---|---|
| `ExecutionContext` | `pub struct` | `config`, `metrics`, `dry_run`, `verbose` | `engine::worker::context` |
| `TaskStatus` | `pub enum` | `Pending`, `Running`, `Completed`, `Failed` | `engine::worker::executor` |
| `TaskResult` | `pub struct` | `task_id`, `status`, `output`, `error`, `elapsed_ms` | `engine::worker::executor` |
| `Executor` | `pub struct` | Task executor with retries | `engine::worker::executor` |
| `Task` | `pub struct` | `id: String`, `payload: String` | `engine::worker::queue` |
| `TaskQueue` | `pub struct` | Bounded FIFO queue | `engine::worker::queue` |
| `Priority` | `pub enum` | `Low`, `Normal`, `High`, `Critical` | `engine::worker::scheduler` |
| `ScheduledTask` | `pub struct` | `id`, `priority`, `dependencies` | `engine::worker::scheduler` |
| `Scheduler` | `pub struct` | Dependency-aware scheduler | `engine::worker::scheduler` |

| Function | Signature | Description | Module |
|---|---|---|---|
| New context | `ExecutionContext::new() → Self` | Default context | `engine::worker::context` |
| With config | `ExecutionContext::with_config(self, config: Config) → Self` | Set config | `engine::worker::context` |
| With dry run | `ExecutionContext::with_dry_run(self, dry_run: bool) → Self` | Enable dry run | `engine::worker::context` |
| With verbose | `ExecutionContext::with_verbose(self, verbose: bool) → Self` | Enable verbose | `engine::worker::context` |
| New executor | `Executor::new() → Self` | Default executor | `engine::worker::executor` |
| With retries | `Executor::with_retries(self, retries: usize) → Self` | Set max retries | `engine::worker::executor` |
| Execute | `Executor::execute<F>(&self, task_id: &str, f: F) → TaskResult` | Run task with retries | `engine::worker::executor` |
| New queue | `TaskQueue::new(capacity: usize) → Self` | Bounded queue | `engine::worker::queue` |
| Enqueue | `TaskQueue::enqueue(&mut self, task: Task) → bool` | Push task | `engine::worker::queue` |
| Dequeue | `TaskQueue::dequeue(&mut self) → Option<Task>` | Pop task | `engine::worker::queue` |
| Queue len | `TaskQueue::len(&self) → usize` | Queue size | `engine::worker::queue` |
| Priority weight | `Priority::weight(&self) → u8` | Priority numeric value | `engine::worker::scheduler` |
| New scheduler | `Scheduler::new() → Self` | Empty scheduler | `engine::worker::scheduler` |
| Schedule | `Scheduler::schedule(&mut self) → Vec<String>` | Topological order | `engine::worker::scheduler` |

### 14. Benchmark — `hub::tools::benchmark` — 1 pub fn

| Function | Signature | Description | Module |
|---|---|---|---|
| Run timed | `run_timed(experiment: &Experiment) → HubResult<(RunOutput, String)>` | Timed experiment | `tools::benchmark` |

### 15. Config — `hub::tools::config` — 11 pub fn + 1 pub struct

| Item | Declaration | Description | Module |
|---|---|---|---|
| `Config` | `pub struct` | Key-value configuration store | `tools::config` |

| Function | Signature | Description | Module |
|---|---|---|---|
| New | `Config::new() → Self` | Empty config | `tools::config` |
| Set | `Config::set(&mut self, key: &str, value: &str)` | Set key | `tools::config` |
| Get | `Config::get(&self, key: &str) → Option<&str>` | Get by key | `tools::config` |
| Get or default | `Config::get_or(&self, key: &str, default: &str) → String` | Get with fallback | `tools::config` |
| Get f64 | `Config::get_f64(&self, key: &str) → Option<f64>` | Parse as f64 | `tools::config` |
| Get usize | `Config::get_usize(&self, key: &str) → Option<usize>` | Parse as usize | `tools::config` |
| Get bool | `Config::get_bool(&self, key: &str) → Option<bool>` | Parse as bool | `tools::config` |
| Keys | `Config::keys(&self) → impl Iterator<Item = &str>` | All keys | `tools::config` |
| Length | `Config::len(&self) → usize` | Entry count | `tools::config` |
| Is empty | `Config::is_empty(&self) → bool` | Empty check | `tools::config` |
| Merge | `Config::merge(&mut self, other: &Config)` | Merge configs | `tools::config` |

### 16. Logger — `hub::tools::logger` — 7 pub fn + 1 pub enum

| Item | Declaration | Description | Module |
|---|---|---|---|
| `Level` | `pub enum` | `Trace = 0`, `Debug = 1`, `Info = 2`, `Warn = 3`, `Error = 4`, `Off = 5` | `tools::logger` |

| Function | Signature | Description | Module |
|---|---|---|---|
| As string | `Level::as_str(self) → &'static str` | Level name | `tools::logger` |
| Set level | `set_level(level: Level)` | Global log level | `tools::logger` |
| Current level | `current_level() → Level` | Active level | `tools::logger` |
| Log | `log(level: Level, module: &str, message: &str)` | Generic log | `tools::logger` |
| Trace | `trace(module: &str, message: &str)` | Trace log | `tools::logger` |
| Debug | `debug(module: &str, message: &str)` | Debug log | `tools::logger` |
| Info | `info(module: &str, message: &str)` | Info log | `tools::logger` |

### 17. Metrics — `hub::tools::metrics` — 7 pub fn + 1 pub struct

| Item | Declaration | Description | Module |
|---|---|---|---|
| `Metrics` | `pub struct` | Thread-safe counters and timings | `tools::metrics` |

| Function | Signature | Description | Module |
|---|---|---|---|
| New | `Metrics::new() → Self` | Empty metrics | `tools::metrics` |
| Increment | `Metrics::increment(&self, name: &str)` | Counter +1 | `tools::metrics` |
| Increment by | `Metrics::increment_by(&self, name: &str, amount: u64)` | Counter +N | `tools::metrics` |
| Counter | `Metrics::counter(&self, name: &str) → u64` | Read counter | `tools::metrics` |
| Record timing | `Metrics::record_timing(&self, name: &str, duration_ms: f64)` | Record duration | `tools::metrics` |
| Timing stats | `Metrics::timing_stats(&self, name: &str) → Option<(f64, f64, f64, usize)>` | (mean, min, max, count) | `tools::metrics` |
| Time operation | `Metrics::time_operation<F, R>(&self, name: &str, op: F) → R` | Auto-timed execution | `tools::metrics` |

### 18. Utils — `hub::tools::utils` — 10 pub fn

| Function | Signature | Description | Module |
|---|---|---|---|
| Scientific | `format_scientific(value: f64, precision: usize) → String` | Scientific notation | `tools::utils` |
| SI prefix | `format_si(value: f64) → String` | SI prefix (k, M, G...) | `tools::utils` |
| Linspace | `linspace(start: f64, end: f64, n: usize) → Vec<f64>` | Linearly spaced points | `tools::utils` |
| Logspace | `logspace(start_exp: f64, end_exp: f64, n: usize) → Vec<f64>` | Logarithmically spaced | `tools::utils` |
| Clamp vec | `clamp_vec(data: &mut [f64], min: f64, max: f64)` | Clamp in-place | `tools::utils` |
| Normalize | `normalize(data: &[f64]) → Vec<f64>` | Normalize [0,1] | `tools::utils` |
| Cumulative sum | `cumulative_sum(data: &[f64]) → Vec<f64>` | Running total | `tools::utils` |
| Moving average | `moving_average(data: &[f64], window: usize) → Vec<f64>` | Windowed mean | `tools::utils` |
| Relative error | `relative_error(computed: f64, exact: f64) → f64` | $\|c - e\|/\|e\|$ | `tools::utils` |
| Approx equal | `approx_equal(a: f64, b: f64, tolerance: f64) → bool` | Tolerance check | `tools::utils` |
