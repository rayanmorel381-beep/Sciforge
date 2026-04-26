# Geology — ChangeLog

---

## v0.0.3

### 1️⃣ New — Erosion — `geology::erosion` — 5 pub fn

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| Fluvial erosion rate | `fluvial_erosion_rate(k: f64, p: f64, alpha: f64, vc: f64) → f64` | $E = k \cdot P^\alpha - v_c$ | `geology::erosion` |
| Chemical weathering rate | `chemical_weathering_rate(a: f64, ea: f64, t: f64, p: f64) → f64` | Arrhenius: $r = A e^{-E_a/RT} P$ | `geology::erosion` |
| Frost weathering rate | `frost_weathering_rate(n_ft: f64, phi: f64) → f64` | Freeze-thaw cycles × porosity | `geology::erosion` |
| Wind erosion threshold | `wind_erosion_threshold(rho_p: f64, rho_a: f64, g: f64, d: f64) → f64` | Bagnold threshold velocity | `geology::erosion` |
| Volcanic explosivity index | `volcanic_explosivity_index(volume_km3: f64) → u8` | VEI from ejecta volume | `geology::erosion` |

### 2️⃣ New — Geomorphology — `geology::geomorphology` — 20 pub fn

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| Hypsometric curve | `hypsometric_curve(mean_elevation: f64, std_dev: f64, fraction: f64) → f64` | Gaussian-based elevation distribution | `geology::geomorphology` |
| Continental fraction | `continental_fraction(total_area: f64, ocean_area: f64) → f64` | $(A_{total}-A_{ocean})/A_{total}$ | `geology::geomorphology` |
| Ocean basin depth | `ocean_basin_depth(ridge_depth: f64, plate_age_myr: f64, thermal_diffusivity: f64) → f64` | Plate cooling model $d \propto \sqrt{t}$ | `geology::geomorphology` |
| Mid-ocean ridge elevation | `mid_ocean_ridge_elevation(spreading_rate_cm_yr: f64, mantle_temperature: f64, reference_temperature: f64) → f64` | Ridge height from mantle state | `geology::geomorphology` |
| Passive margin subsidence | `passive_margin_subsidence(initial_elevation: f64, time_myr: f64, thermal_time_constant_myr: f64) → f64` | Exponential thermal decay | `geology::geomorphology` |
| Orogenic elevation | `orogenic_elevation(convergence_rate: f64, erosion_rate: f64, crustal_density: f64, mantle_density: f64, time: f64) → f64` | Tectonic uplift vs erosion balance | `geology::geomorphology` |
| Erosion rate (Hack) | `erosion_rate_hack(coefficient: f64, drainage_area: f64, slope: f64, m: f64, n: f64) → f64` | $E = k A^m S^n$ — stream power law | `geology::geomorphology` |
| Sediment transport capacity | `sediment_transport_capacity(flow_velocity: f64, depth: f64, grain_size: f64, density_water: f64, density_sediment: f64, g_surface: f64) → f64` | Hydraulic transport | `geology::geomorphology` |
| Crater diameter | `crater_diameter(impactor_diameter: f64, impactor_velocity: f64, impactor_density: f64, target_density: f64, g_surface: f64) → f64` | Impact cratering scaling | `geology::geomorphology` |
| Crater depth-to-diameter | `crater_depth_to_diameter(simple_to_complex_transition: f64, diameter: f64) → f64` | Morphometric relation | `geology::geomorphology` |
| Shield volcano profile | `shield_volcano_profile(base_radius: f64, max_height: f64, r: f64) → f64` | Low-slope radial profile | `geology::geomorphology` |
| Stratovolcano profile | `stratovolcano_profile(base_radius: f64, max_height: f64, r: f64) → f64` | Steep composite cone profile | `geology::geomorphology` |
| Caldera profile | `caldera_profile(caldera_radius: f64, caldera_depth: f64, rim_height: f64, rim_width: f64, r: f64) → f64` | Collapse caldera topography | `geology::geomorphology` |
| Rift valley width | `rift_valley_width(elastic_thickness: f64, youngs_modulus: f64, poisson: f64, mantle_density: f64, g_surface: f64) → f64` | Flexural half-graben width | `geology::geomorphology` |
| Tectonic stress field | `tectonic_stress_field(plate_velocity: f64, viscosity: f64, thickness: f64) → f64` | Regional stress from plate motion | `geology::geomorphology` |
| Fault slip rate | `fault_slip_rate(tectonic_stress: f64, friction_coefficient: f64, normal_stress: f64) → f64` | Rate-dependent faulting | `geology::geomorphology` |
| Weathering rate | `weathering_rate(rate_constant: f64, activation_energy: f64, temperature: f64, precipitation_rate: f64) → f64` | Chemical weathering (Arrhenius) | `geology::geomorphology` |
| Soil production rate | `soil_production_rate(bare_rate: f64, soil_thickness: f64, characteristic_depth: f64) → f64` | Exponential decay with depth | `geology::geomorphology` |
| Landscape diffusion | `landscape_diffusion(diffusivity: f64, curvature: f64) → f64` | Hillslope transport: $q = -\kappa \nabla^2 z$ | `geology::geomorphology` |
| Flexural wavelength | `flexural_wavelength(elastic_thickness: f64, youngs_modulus: f64, poisson: f64, density_contrast: f64, g_surface: f64) → f64` | Lithospheric flexure scale | `geology::geomorphology` |

### 3️⃣ New — Glaciology — `geology::glaciology` — 4 pub fn

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| Glen strain rate | `glen_strain_rate(a: f64, tau: f64, n: f64) → f64` | $\dot{\varepsilon} = A \tau^n$ — Glen's flow law | `geology::glaciology` |
| Shallow ice velocity | `shallow_ice_velocity(a: f64, n: f64, rho: f64, g: f64, alpha: f64, h: f64) → f64` | SIA surface velocity | `geology::glaciology` |
| Ice viscosity | `ice_viscosity(a: f64, tau: f64, n: f64) → f64` | Effective viscosity from Glen's law | `geology::glaciology` |
| Glacial bed erosion | `glacial_bed_erosion(kg: f64, vb: f64, l: f64) → f64` | Quarrying/abrasion rate | `geology::glaciology` |

### 4️⃣ New — Hydrology — `geology::hydrology` — 6 pub fn

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| Manning velocity | `manning_velocity(n: f64, rh: f64, s: f64) → f64` | $v = \frac{1}{n} R_h^{2/3} S^{1/2}$ | `geology::hydrology` |
| Chézy velocity | `chezy_velocity(c: f64, rh: f64, s: f64) → f64` | $v = C \sqrt{R_h S}$ | `geology::hydrology` |
| Froude number | `froude_number(v: f64, g: f64, d: f64) → f64` | $Fr = v/\sqrt{gd}$ | `geology::hydrology` |
| Reynolds number | `reynolds_number(v: f64, d: f64, nu: f64) → f64` | $Re = vd/\nu$ | `geology::hydrology` |
| Stream power | `stream_power(rho: f64, g: f64, q: f64, s: f64) → f64` | $\Omega = \rho g Q S$ | `geology::hydrology` |
| Hjulström threshold | `hjulstrom_erosion_threshold(d_grain: f64) → f64` | Critical erosion velocity | `geology::hydrology` |

### 5️⃣ New — Mantle Dynamics — `geology::mantle` — 25 pub fn

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| Rayleigh number | `rayleigh_number(alpha: f64, g_surface: f64, delta_t: f64, depth: f64, kappa: f64, nu: f64) → f64` | $Ra = \frac{\alpha g \Delta T d^3}{\kappa \nu}$ | `geology::mantle` |
| Nusselt number | `nusselt_number(rayleigh: f64, beta_exponent: f64) → f64` | $Nu = Ra^\beta$ | `geology::mantle` |
| Convective velocity | `convective_velocity(alpha: f64, g_surface: f64, delta_t: f64, depth: f64, kappa: f64, nu: f64) → f64` | Mantle convection speed | `geology::mantle` |
| Core temperature adiabat | `core_temperature_adiabat(surface_core_temp: f64, pressure: f64, gruneisen_parameter: f64, bulk_modulus: f64) → f64` | Adiabatic T-P profile | `geology::mantle` |
| Inner core radius | `inner_core_radius(total_core_radius: f64, core_mass: f64, melting_gradient: f64, adiabatic_gradient: f64) → f64` | Liquid-solid boundary | `geology::mantle` |
| Core heat flux | `core_heat_flux(thermal_conductivity: f64, delta_t: f64, core_radius: f64) → f64` | $q = k \Delta T / R$ | `geology::mantle` |
| CMB heat flux | `cmb_heat_flux(nusselt: f64, thermal_conductivity: f64, delta_t: f64, mantle_depth: f64) → f64` | Core-mantle boundary flux | `geology::mantle` |
| Mantle overturn time | `mantle_overturn_time(mantle_depth: f64, convective_vel: f64) → f64` | $t = d/v$ | `geology::mantle` |
| Viscosity (T-dependent) | `viscosity_temperature(reference_viscosity: f64, activation_energy: f64, temperature: f64, reference_temperature: f64) → f64` | Arrhenius viscosity | `geology::mantle` |
| Thermal boundary layer | `thermal_boundary_layer_thickness(mantle_depth: f64, rayleigh: f64) → f64` | $\delta \propto d \cdot Ra^{-1/3}$ | `geology::mantle` |
| Plume buoyancy flux | `plume_buoyancy_flux(alpha: f64, density: f64, g_surface: f64, delta_t: f64, volume_flux: f64) → f64` | Mantle plume driver | `geology::mantle` |
| Plume conduit radius | `plume_conduit_radius(buoyancy_flux: f64, kinematic_viscosity: f64, thermal_diffusivity: f64) → f64` | Plume geometry | `geology::mantle` |
| Core cooling rate | `core_cooling_rate(surface_heat_flux: f64, core_radius: f64, core_density: f64, specific_heat: f64) → f64` | $dT/dt$ for the core | `geology::mantle` |
| Magnetic Reynolds number | `magnetic_reynolds_number(velocity: f64, length_scale: f64, magnetic_diffusivity: f64) → f64` | $Rm = v L / \eta$ | `geology::mantle` |
| Dynamo active | `dynamo_active(magnetic_reynolds: f64) → bool` | $Rm > Rm_{crit}$ check | `geology::mantle` |
| Dipole moment | `dipole_moment(core_radius: f64, density: f64, angular_velocity: f64, conductivity: f64, convective_power: f64) → f64` | Geomagnetic dipole | `geology::mantle` |
| Convective power | `convective_power(cmb_flux: f64, core_radius: f64) → f64` | Power available for dynamo | `geology::mantle` |
| Tidal heating rate | `tidal_heating_rate(mass: f64, radius: f64, eccentricity: f64, orbital_freq: f64, tidal_q: f64, rigidity: f64) → f64` | Internal tidal dissipation | `geology::mantle` |
| Radiogenic heating | `radiogenic_heating(mass_mantle: f64, concentration_u238: f64, concentration_th232: f64, concentration_k40: f64, time_ga: f64) → f64` | Heat from U, Th, K decay | `geology::mantle` |
| Secular cooling flux | `secular_cooling_flux(core_heat_capacity: f64, core_mass: f64, cooling_rate_k_per_s: f64, core_radius: f64) → f64` | Cooling contribution | `geology::mantle` |
| Gravitational differentiation | `gravitational_differentiation_power(inner_core_growth_rate: f64, density_contrast: f64, g_cmb: f64, core_radius: f64) → f64` | Inner core crystallization power | `geology::mantle` |
| Parameterized mantle T | `parameterized_mantle_temperature(initial_temp: f64, radiogenic_heat: f64, surface_heat_loss: f64, mantle_heat_capacity: f64, mantle_mass: f64, dt: f64) → f64` | Thermal evolution step | `geology::mantle` |
| Stagnant lid thickness | `stagnant_lid_thickness(mantle_depth: f64, activation_energy: f64, delta_t: f64, interior_temp: f64) → f64` | Rheological lid depth | `geology::mantle` |
| Planet surface heat flux | `planet_surface_heat_flux(interior_temp: f64, surface_temp: f64, thermal_conductivity: f64, lithosphere_thickness: f64) → f64` | Surface heat flow | `geology::mantle` |
| Core liquidus | `core_liquidus(pressure_gpa: f64, fe_fraction: f64) → f64` | Iron alloy melting curve | `geology::mantle` |

### 6️⃣ New — Volcanism — `geology::volcanism` — 18 pub fn

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| Magma viscosity | `magma_viscosity(a: f64, activation_energy: f64, temperature_k: f64) → f64` | Arrhenius: $\eta = A e^{E_a/RT}$ | `geology::volcanism` |
| Magma buoyancy force | `magma_buoyancy_force(rho_magma: f64, rho_crust: f64, g_surface: f64, chamber_volume: f64) → f64` | $F = \Delta\rho \cdot g \cdot V$ | `geology::volcanism` |
| Overpressure threshold | `overpressure_threshold(tensile_strength: f64, lithostatic_pressure: f64) → f64` | Failure criterion | `geology::volcanism` |
| Chamber overpressure | `chamber_overpressure(bulk_modulus: f64, volume_injected: f64, chamber_volume: f64) → f64` | $\Delta P = K \cdot \Delta V / V$ | `geology::volcanism` |
| Effusion rate | `effusion_rate(overpressure: f64, conduit_radius: f64, conduit_length: f64, viscosity: f64) → f64` | Poiseuille flow in conduit | `geology::volcanism` |
| Volcanic explosivity | `volcanic_explosivity_index(ejecta_volume_km3: f64) → f64` | VEI from ejecta volume | `geology::volcanism` |
| Lava flow velocity | `lava_flow_velocity(g_surface: f64, slope_angle: f64, thickness: f64, density: f64, viscosity: f64) → f64` | Jeffreys equation | `geology::volcanism` |
| Pyroclastic column height | `pyroclastic_column_height(thermal_flux: f64) → f64` | $H \propto Q^{1/4}$ — plume rise | `geology::volcanism` |
| Seismic-eruption probability | `seismic_to_eruption_probability(cumulative_seismic_moment: f64, critical_moment: f64, b_value: f64) → f64` | Empirical forecasting | `geology::volcanism` |
| Tidal triggering stress | `tidal_triggering_stress(mass_perturber: f64, distance: f64, body_radius: f64) → f64` | Tidal-volcanic coupling | `geology::volcanism` |
| Coulomb failure stress | `coulomb_failure_stress(shear_stress: f64, normal_stress: f64, pore_pressure: f64, friction_coefficient: f64, cohesion: f64) → f64` | $CFS = \tau - \mu(\sigma_n - P) - c$ | `geology::volcanism` |
| Magma ascent velocity | `magma_ascent_velocity(density_contrast: f64, g_surface: f64, conduit_radius: f64, viscosity: f64) → f64` | Stokes ascent in conduit | `geology::volcanism` |
| Volatile exsolution depth | `volatile_exsolution_depth(saturation_pressure: f64, density: f64, g_surface: f64) → f64` | $d = P_{sat}/(\rho g)$ | `geology::volcanism` |
| Fragmentation threshold | `fragmentation_threshold(porosity: f64, vesicularity_critical: f64) → bool` | Critical vesicularity check | `geology::volcanism` |
| Tephra fallout distance | `tephra_fallout_distance(column_height: f64, wind_speed: f64, particle_settling_velocity: f64) → f64` | Advection-settling model | `geology::volcanism` |
| Thermal erosion rate | `thermal_erosion_rate(lava_temperature: f64, substrate_solidus: f64, lava_velocity: f64, thermal_diffusivity: f64) → f64` | Lava channel incision | `geology::volcanism` |
| Degassing rate | `degassing_rate(volatile_concentration: f64, solubility_coefficient: f64, pressure: f64) → f64` | Volatile release rate | `geology::volcanism` |
| Eruption energy | `eruption_energy(ejecta_mass: f64, ejecta_velocity: f64, thermal_energy: f64) → f64` | Total eruption energy budget | `geology::volcanism` |

### 7️⃣ Changes — Existing Modules

| Module | Function Added | Signature |
|---|---|---|
| `geology::dating` | Radiogenic heat production | `radiogenic_heat_production(u238_ppm: f64, th232_ppm: f64, k40_ppm: f64, density: f64) → f64` |
| `geology::petrology` | Viscosity (Arrhenius) | `viscosity_arrhenius(a: f64, ea: f64, t: f64) → f64` |
| `geology::petrology` | Crystal settling velocity | `crystal_settling_velocity(delta_rho: f64, g: f64, r: f64, mu: f64) → f64` |

### 8️⃣ Testing

| Metric | Value |
|---|---|
| Tests | 22 → 45 (+23) |
| Test files | 4 → 8 |
| File list | `main.rs`, `seismology.rs`, `dating.rs`, `tectonics.rs`, `erosion.rs`, `glaciology.rs`, `hydrology.rs`, `petrology.rs` |
| Total suite | 814 tests, 0 warnings |

---

## v0.0.2

### 1️⃣ Module Changes

| Module | Changes |
|---|---|
| `geology::dating` | Modified |
| `geology::petrology` | Modified |
| `geology::seismology` | Modified |
| `geology::tectonics` | Modified |

### 2️⃣ Testing

| Metric | Value |
|---|---|
| Tests | 0 → 22 |
| Test files | `seismology.rs` (8), `dating.rs` (7), `tectonics.rs` (6), `main.rs` (1) |

---

## v0.0.1

### 1️⃣ Seismology — `geology::seismology` — 12 pub fn

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| P-wave velocity | `p_wave_velocity(k: f64, g: f64, rho: f64) → f64` | $v_p = \sqrt{(K + 4G/3)/\rho}$ | `geology::seismology` |
| S-wave velocity | `s_wave_velocity(g: f64, rho: f64) → f64` | $v_s = \sqrt{G/\rho}$ | `geology::seismology` |
| Richter magnitude | `richter_magnitude(amplitude: f64, distance_km: f64) → f64` | $M_L = \log_{10} A + f(d)$ | `geology::seismology` |
| Moment magnitude | `moment_magnitude(seismic_moment: f64) → f64` | $M_w = \frac{2}{3}\log_{10}(M_0) - 10.7$ | `geology::seismology` |
| Seismic moment | `seismic_moment(mw: f64) → f64` | $M_0 = 10^{1.5(M_w + 10.7)}$ | `geology::seismology` |
| Epicenter distance | `epicenter_distance(vp: f64, vs: f64, ts_tp: f64) → f64` | S-P time method | `geology::seismology` |
| Travel time | `travel_time(distance: f64, velocity: f64) → f64` | $t = d/v$ | `geology::seismology` |
| Snell seismic | `snell_seismic(v1: f64, theta1: f64, v2: f64) → f64` | $\sin\theta_1/v_1 = \sin\theta_2/v_2$ | `geology::seismology` |
| Gutenberg-Richter | `gutenberg_richter(a: f64, b: f64, magnitude: f64) → f64` | $\log_{10} N = a - bM$ | `geology::seismology` |
| Omori aftershock | `omori_aftershock(k: f64, c: f64, p: f64, t: f64) → f64` | $n(t) = K/(c+t)^p$ | `geology::seismology` |
| Seismic energy | `seismic_energy(magnitude: f64) → f64` | $\log_{10} E = 1.5M + 4.8$ | `geology::seismology` |
| Peak ground acceleration | `peak_ground_acceleration(a: f64, b: f64, magnitude: f64, distance: f64) → f64` | GMPE attenuation | `geology::seismology` |

### 2️⃣ Geochronology — `geology::dating` — 17 pub fn

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| Radioactive decay | `radioactive_decay(n0: f64, lambda: f64, t: f64) → f64` | $N(t) = N_0 e^{-\lambda t}$ | `geology::dating` |
| Half-life | `half_life(lambda: f64) → f64` | $t_{1/2} = \ln 2 / \lambda$ | `geology::dating` |
| Decay constant | `decay_constant(half_life: f64) → f64` | $\lambda = \ln 2 / t_{1/2}$ | `geology::dating` |
| Age from ratio | `age_from_ratio(ratio_daughter_parent: f64, lambda: f64) → f64` | $t = \ln(1+D/P)/\lambda$ | `geology::dating` |
| ¹⁴C age | `carbon14_age(ratio: f64) → f64` | Radiocarbon dating | `geology::dating` |
| K-Ar age | `potassium_argon_age(ar40: f64, k40: f64) → f64` | ⁴⁰K-⁴⁰Ar system | `geology::dating` |
| U-Pb age | `uranium_lead_age(pb206: f64, u238: f64) → f64` | ²³⁸U-²⁰⁶Pb system | `geology::dating` |
| Isochron age | `isochron_age(slope: f64, lambda: f64) → f64` | $t = \ln(1+slope)/\lambda$ | `geology::dating` |
| Fission track age | `fission_track_age(rho_s: f64, rho_i: f64, rho_d: f64, lambda: f64) → f64` | FT dating method | `geology::dating` |
| Luminescence dose | `luminescence_dose(natural_signal: f64, dose_rate: f64) → f64` | OSL/TL age estimation | `geology::dating` |
| Cosmogenic exposure | `cosmogenic_exposure_age(concentration: f64, production_rate: f64, lambda: f64) → f64` | Surface exposure dating | `geology::dating` |
| ²³⁵U-²⁰⁷Pb age | `uranium_235_lead_age(pb207: f64, u235: f64) → f64` | ²³⁵U-²⁰⁷Pb system | `geology::dating` |
| Concordia ²³⁸U | `concordia_u238_pb206(t: f64) → f64` | $e^{\lambda_{238} t} - 1$ | `geology::dating` |
| Concordia ²³⁵U | `concordia_u235_pb207(t: f64) → f64` | $e^{\lambda_{235} t} - 1$ | `geology::dating` |
| Concordia age | `concordia_age(pb206_u238: f64, pb207_u235: f64) → f64` | Concordia intercept | `geology::dating` |
| Th-Pb age | `thorium_232_lead_age(pb208: f64, th232: f64) → f64` | ²³²Th-²⁰⁸Pb system | `geology::dating` |
| U-Th-He age | `u_th_he_age(he4: f64, u238: f64, u235: f64, th232: f64) → f64` | (U-Th)/He thermochronology | `geology::dating` |

### 3️⃣ Petrology — `geology::petrology` — 8 pub fn

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| CIPW quartz norm | `cipw_quartz_norm(sio2: f64, feldspars: f64, mafics: f64) → f64` | Normative mineralogy | `geology::petrology` |
| Mg number | `mg_number(mgo: f64, feo: f64) → f64` | $Mg\# = MgO/(MgO+FeO)$ | `geology::petrology` |
| Differentiation index | `differentiation_index(q: f64, or_val: f64, ab: f64, ne: f64) → f64` | Sum of normative felsic minerals | `geology::petrology` |
| Total Alkali Silica | `total_alkali_silica(na2o: f64, k2o: f64) → f64` | TAS classification | `geology::petrology` |
| Alumina saturation | `alumina_saturation_index(al2o3: f64, cao: f64, na2o: f64, k2o: f64) → f64` | ASI = Al₂O₃/(CaO+Na₂O+K₂O) | `geology::petrology` |
| Color index | `color_index(mafic_minerals: &[f64]) → f64` | Modal mafic % | `geology::petrology` |
| Liquidus temperature | `liquidus_temperature(composition: f64, t_melt_a: f64, t_melt_b: f64) → f64` | Binary system liquidus | `geology::petrology` |
| Solidus depression | `solidus_depression(water_content: f64, base_solidus: f64, k: f64) → f64` | $T_s = T_{s,dry} - k \cdot H_2O$ | `geology::petrology` |

### 4️⃣ Tectonics — `geology::tectonics` — 11 pub fn

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| Plate velocity | `plate_velocity(distance: f64, time: f64) → f64` | $v = d/t$ | `geology::tectonics` |
| Euler pole velocity | `euler_pole_velocity(omega: f64, radius: f64, colatitude: f64) → f64` | $v = \omega R \sin\theta$ | `geology::tectonics` |
| Isostatic equilibrium | `isostatic_equilibrium(rho_crust: f64, thickness: f64, rho_mantle: f64) → f64` | Archimedes balance | `geology::tectonics` |
| Pratt isostasy | `pratt_isostasy(rho_ref: f64, d_ref: f64, elevation: f64) → f64` | Variable density model | `geology::tectonics` |
| Airy root | `airy_root(elevation: f64, rho_crust: f64, rho_mantle: f64) → f64` | $r = h \cdot \rho_c/(\rho_m - \rho_c)$ | `geology::tectonics` |
| Thermal subsidence | `thermal_subsidence(e0: f64, t: f64, tau: f64) → f64` | $S(t) = E_0(1 - e^{-t/\tau})$ | `geology::tectonics` |
| McKenzie stretching | `mckenzie_stretching(beta: f64, rho_m: f64, rho_c: f64, alpha: f64, tl: f64, tc: f64) → f64` | Rift basin subsidence | `geology::tectonics` |
| Heat flow | `heat_flow(k: f64, dt_dz: f64) → f64` | $q = -k \cdot dT/dz$ — Fourier's law | `geology::tectonics` |
| Geothermal gradient | `geothermal_gradient(surface_temp: f64, depth: f64, gradient: f64) → f64` | $T(z) = T_s + \nabla T \cdot z$ | `geology::tectonics` |
| Flexural rigidity | `flexural_rigidity(e: f64, te: f64, nu: f64) → f64` | $D = E T_e^3 / [12(1-\nu^2)]$ | `geology::tectonics` |
| Elastic thickness | `elastic_thickness_from_rigidity(d: f64, e: f64, nu: f64) → f64` | Inverse of flexural rigidity | `geology::tectonics` |
