# Geology Source Code Guide

This page documents the source implementation behind `sciforge::geology`, including file layout and Hub dispatch wiring.

## Source Coverage

### What is explained
- File-level implementation layout in `src/geology/`.
- Main computation groups and where they are implemented.
- Runtime call path when geology functions are executed through Hub dispatch.

### Visibility and external access
- This domain module is `pub(crate)` in `src/lib.rs` and is not part of the external crate API.
- External consumers should use the public `sciforge::hub` API for these computations.
- Direct module paths shown here are for internal development and source-level understanding.


## Source Code Explanation

### Relevant file structure
- Main implementation: `src/geology/`
- Module entry point: `src/geology/mod.rs`
- Hub routing (when applicable): `src/hub/engine/dispatch/geology.rs`

### Internal execution flow
1. The module exposes subdomains through `mod.rs` and targeted `pub use` exports.
2. Each `.rs` file implements a coherent block of equations and functions.
3. The Hub invokes these functions through the domain dispatcher when execution goes through `ExperimentRunner`.

### What to check while reading code
- Exact signature: input/output types and argument order.
- Numerical preconditions: divisions, roots, logarithms, and validity ranges.
- Implicit conventions: units, normalization, bounds, and tolerances.

## Modules

- `dating` — geochronology and radiometric dating methods
- `erosion` — fluvial, chemical, frost, and wind erosion models
- `geomorphology` — landscape evolution, drainage, slope stability
- `glaciology` — ice flow, glacial mass balance, isostatic rebound
- `hydrology` — groundwater flow, aquifer properties, infiltration
- `mantle` — mantle convection, viscosity, heat flow, plume dynamics
- `petrology` — rock classification, magma composition, igneous petrology
- `seismology` — seismic waves, earthquake magnitude, aftershock statistics
- `tectonics` — plate tectonics, isostasy, heat flow, flexure
- `volcanism` — eruption dynamics, lava flow, tephra dispersal

---

### dating (18 functions)

| Function | Signature |
|----------|-----------|
| `radioactive_decay` | `(n0: f64, lambda: f64, t: f64) -> f64` |
| `half_life` | `(lambda: f64) -> f64` |
| `decay_constant` | `(half_life: f64) -> f64` |
| `age_from_ratio` | `(ratio_daughter_parent: f64, lambda: f64) -> f64` |
| `carbon14_age` | `(ratio: f64) -> f64` |
| `potassium_argon_age` | `(ar40: f64, k40: f64) -> f64` |
| `uranium_lead_age` | `(pb206: f64, u238: f64) -> f64` |
| `isochron_age` | `(slope: f64, lambda: f64) -> f64` |
| `fission_track_age` | `(rho_s: f64, rho_i: f64, rho_d: f64, lambda: f64) -> f64` |
| `luminescence_dose` | `(natural_signal: f64, dose_rate: f64) -> f64` |
| `cosmogenic_exposure_age` | `(concentration: f64, production_rate: f64, lambda: f64) -> f64` |
| `uranium_235_lead_age` | `(pb207: f64, u235: f64) -> f64` |
| `concordia_u238_pb206` | `(t: f64) -> f64` |
| `concordia_u235_pb207` | `(t: f64) -> f64` |
| `concordia_age` | `(pb206_u238: f64, pb207_u235: f64) -> f64` |
| `thorium_232_lead_age` | `(pb208: f64, th232: f64) -> f64` |
| `u_th_he_age` | `(he4: f64, u238: f64, u235: f64, th232: f64) -> f64` |
| `radiogenic_heat_production` | `(u238_ppm: f64, th232_ppm: f64, k40_ppm: f64, density: f64) -> f64` |

---

### erosion (5 functions)

| Function | Signature |
|----------|-----------|
| `fluvial_erosion_rate` | `(k: f64, p: f64, alpha: f64, vc: f64) -> f64` |
| `chemical_weathering_rate` | `(a: f64, ea: f64, t: f64, p: f64) -> f64` |
| `frost_weathering_rate` | `(n_ft: f64, phi: f64) -> f64` |
| `wind_erosion_threshold` | `(rho_p: f64, rho_a: f64, g: f64, d: f64) -> f64` |
| `volcanic_explosivity_index` | `(volume_km3: f64) -> u8` |

---

### geomorphology (20 functions)

| Function | Signature |
|----------|-----------|
| `hypsometric_curve` | `(mean_elevation: f64, std_dev: f64, fraction: f64) -> f64` |
| `continental_fraction` | `(total_area: f64, ocean_area: f64) -> f64` |
| `ocean_basin_depth` | `( ridge_depth: f64, plate_age_myr: f64, thermal_diffusivity: f64, ) -> f64` |
| `mid_ocean_ridge_elevation` | `( spreading_rate_cm_yr: f64, mantle_temperature: f64, reference_temperature: f64, ) -> f64` |
| `passive_margin_subsidence` | `( initial_elevation: f64, time_myr: f64, thermal_time_constant_myr: f64, ) -> f64` |
| `orogenic_elevation` | `( convergence_rate: f64, erosion_rate: f64, crustal_density: f64, mantle_density: f64, time: f64, ) -> f64` |
| `erosion_rate_hack` | `( coefficient: f64, drainage_area: f64, slope: f64, m: f64, n: f64, ) -> f64` |
| `sediment_transport_capacity` | `( flow_velocity: f64, depth: f64, grain_size: f64, density_water: f64, density_sediment: f64, g_surface: f64, ) -> f64` |
| `crater_diameter` | `( impactor_diameter: f64, impactor_velocity: f64, impactor_density: f64, target_density: f64, g_surface: f64, ) -> f64` |
| `crater_depth_to_diameter` | `(simple_to_complex_transition: f64, diameter: f64) -> f64` |
| `shield_volcano_profile` | `( base_radius: f64, max_height: f64, r: f64, ) -> f64` |
| `stratovolcano_profile` | `( base_radius: f64, max_height: f64, r: f64, ) -> f64` |
| `caldera_profile` | `( caldera_radius: f64, caldera_depth: f64, rim_height: f64, rim_width: f64, r: f64, ) -> f64` |
| `rift_valley_width` | `( elastic_thickness: f64, youngs_modulus: f64, poisson: f64, mantle_density: f64, g_surface: f64, ) -> f64` |
| `tectonic_stress_field` | `( plate_velocity: f64, viscosity: f64, thickness: f64, ) -> f64` |
| `fault_slip_rate` | `( tectonic_stress: f64, friction_coefficient: f64, normal_stress: f64, ) -> f64` |
| `weathering_rate` | `( rate_constant: f64, activation_energy: f64, temperature: f64, precipitation_rate: f64, ) -> f64` |
| `soil_production_rate` | `( bare_rate: f64, soil_thickness: f64, characteristic_depth: f64, ) -> f64` |
| `landscape_diffusion` | `( diffusivity: f64, curvature: f64, ) -> f64` |
| `flexural_wavelength` | `( elastic_thickness: f64, youngs_modulus: f64, poisson: f64, density_contrast: f64, g_surface: f64, ) -> f64` |

---

### glaciology (4 functions)

| Function | Signature |
|----------|-----------|
| `glen_strain_rate` | `(a: f64, tau: f64, n: f64) -> f64` |
| `shallow_ice_velocity` | `(a: f64, n: f64, rho: f64, g: f64, alpha: f64, h: f64) -> f64` |
| `ice_viscosity` | `(a: f64, tau: f64, n: f64) -> f64` |
| `glacial_bed_erosion` | `(kg: f64, vb: f64, l: f64) -> f64` |

---

### hydrology (6 functions)

| Function | Signature |
|----------|-----------|
| `manning_velocity` | `(n: f64, rh: f64, s: f64) -> f64` |
| `chezy_velocity` | `(c: f64, rh: f64, s: f64) -> f64` |
| `froude_number` | `(v: f64, g: f64, d: f64) -> f64` |
| `reynolds_number` | `(v: f64, d: f64, nu: f64) -> f64` |
| `stream_power` | `(rho: f64, g: f64, q: f64, s: f64) -> f64` |
| `hjulstrom_erosion_threshold` | `(d_grain: f64) -> f64` |

---

### mantle (25 functions)

| Function | Signature |
|----------|-----------|
| `rayleigh_number` | `( alpha: f64, g_surface: f64, delta_t: f64, depth: f64, kappa: f64, nu: f64, ) -> f64` |
| `nusselt_number` | `(rayleigh: f64, beta_exponent: f64) -> f64` |
| `convective_velocity` | `( alpha: f64, g_surface: f64, delta_t: f64, depth: f64, kappa: f64, nu: f64, ) -> f64` |
| `core_temperature_adiabat` | `( surface_core_temp: f64, pressure: f64, gruneisen_parameter: f64, bulk_modulus: f64, ) -> f64` |
| `inner_core_radius` | `( total_core_radius: f64, core_mass: f64, melting_gradient: f64, adiabatic_gradient: f64, ) -> f64` |
| `core_heat_flux` | `( thermal_conductivity: f64, delta_t: f64, core_radius: f64, ) -> f64` |
| `cmb_heat_flux` | `( nusselt: f64, thermal_conductivity: f64, delta_t: f64, mantle_depth: f64, ) -> f64` |
| `mantle_overturn_time` | `(mantle_depth: f64, convective_vel: f64) -> f64` |
| `viscosity_temperature` | `( reference_viscosity: f64, activation_energy: f64, temperature: f64, reference_temperature: f64, ) -> f64` |
| `thermal_boundary_layer_thickness` | `( mantle_depth: f64, rayleigh: f64, ) -> f64` |
| `plume_buoyancy_flux` | `( alpha: f64, density: f64, g_surface: f64, delta_t: f64, volume_flux: f64, ) -> f64` |
| `plume_conduit_radius` | `( buoyancy_flux: f64, kinematic_viscosity: f64, thermal_diffusivity: f64, ) -> f64` |
| `core_cooling_rate` | `( surface_heat_flux: f64, core_radius: f64, core_density: f64, specific_heat: f64, ) -> f64` |
| `magnetic_reynolds_number` | `( velocity: f64, length_scale: f64, magnetic_diffusivity: f64, ) -> f64` |
| `dynamo_active` | `(magnetic_reynolds: f64) -> bool` |
| `dipole_moment` | `( core_radius: f64, density: f64, angular_velocity: f64, conductivity: f64, convective_power: f64, ) -> f64` |
| `convective_power` | `( cmb_flux: f64, core_radius: f64, ) -> f64` |
| `tidal_heating_rate` | `( mass: f64, radius: f64, eccentricity: f64, orbital_freq: f64, tidal_q: f64, rigidity: f64, ) -> f64` |
| `radiogenic_heating` | `( mass_mantle: f64, concentration_u238: f64, concentration_th232: f64, concentration_k40: f64, time_ga: f64, ) -> f64` |
| `secular_cooling_flux` | `( core_heat_capacity: f64, core_mass: f64, cooling_rate_k_per_s: f64, core_radius: f64, ) -> f64` |
| `gravitational_differentiation_power` | `( inner_core_growth_rate: f64, density_contrast: f64, g_cmb: f64, core_radius: f64, ) -> f64` |
| `parameterized_mantle_temperature` | `( initial_temp: f64, radiogenic_heat: f64, surface_heat_loss: f64, mantle_heat_capacity: f64, mantle_mass: f64, dt: f64, ) -> f64` |
| `stagnant_lid_thickness` | `( mantle_depth: f64, activation_energy: f64, delta_t: f64, interior_temp: f64, ) -> f64` |
| `planet_surface_heat_flux` | `( interior_temp: f64, surface_temp: f64, thermal_conductivity: f64, lithosphere_thickness: f64, ) -> f64` |
| `core_liquidus` | `(pressure_gpa: f64, fe_fraction: f64) -> f64` |

---

### petrology (10 functions)

| Function | Signature |
|----------|-----------|
| `cipw_quartz_norm` | `(sio2: f64, feldspars: f64, mafics: f64) -> f64` |
| `mg_number` | `(mgo: f64, feo: f64) -> f64` |
| `differentiation_index` | `(q: f64, or_val: f64, ab: f64, ne: f64) -> f64` |
| `total_alkali_silica` | `(na2o: f64, k2o: f64) -> f64` |
| `alumina_saturation_index` | `(al2o3: f64, cao: f64, na2o: f64, k2o: f64) -> f64` |
| `color_index` | `(mafic_minerals: &[f64]) -> f64` |
| `liquidus_temperature` | `(composition: f64, t_melt_a: f64, t_melt_b: f64) -> f64` |
| `solidus_depression` | `(water_content: f64, base_solidus: f64, k: f64) -> f64` |
| `crystal_settling_velocity` | `(delta_rho: f64, g: f64, r: f64, mu: f64) -> f64` |
| `viscosity_arrhenius` | `(a: f64, ea: f64, t: f64) -> f64` |

---

### seismology (12 functions)

| Function | Signature |
|----------|-----------|
| `p_wave_velocity` | `(k: f64, g: f64, rho: f64) -> f64` |
| `s_wave_velocity` | `(g: f64, rho: f64) -> f64` |
| `richter_magnitude` | `(amplitude: f64, distance_km: f64) -> f64` |
| `moment_magnitude` | `(seismic_moment: f64) -> f64` |
| `seismic_moment` | `(mw: f64) -> f64` |
| `epicenter_distance` | `(vp: f64, vs: f64, ts_tp: f64) -> f64` |
| `travel_time` | `(distance: f64, velocity: f64) -> f64` |
| `snell_seismic` | `(v1: f64, theta1: f64, v2: f64) -> f64` |
| `gutenberg_richter` | `(a: f64, b: f64, magnitude: f64) -> f64` |
| `omori_aftershock` | `(k: f64, c: f64, p: f64, t: f64) -> f64` |
| `seismic_energy` | `(magnitude: f64) -> f64` |
| `peak_ground_acceleration` | `(a: f64, b: f64, magnitude: f64, distance: f64) -> f64` |

---

### tectonics (11 functions)

| Function | Signature |
|----------|-----------|
| `plate_velocity` | `(distance: f64, time: f64) -> f64` |
| `euler_pole_velocity` | `(omega: f64, radius: f64, colatitude: f64) -> f64` |
| `isostatic_equilibrium` | `(rho_crust: f64, thickness: f64, rho_mantle: f64) -> f64` |
| `pratt_isostasy` | `(rho_ref: f64, d_ref: f64, elevation: f64) -> f64` |
| `airy_root` | `(elevation: f64, rho_crust: f64, rho_mantle: f64) -> f64` |
| `thermal_subsidence` | `(e0: f64, t: f64, tau: f64) -> f64` |
| `mckenzie_stretching` | `(beta: f64, rho_m: f64, rho_c: f64, alpha: f64, tl: f64, tc: f64) -> f64` |
| `heat_flow` | `(k: f64, dt_dz: f64) -> f64` |
| `geothermal_gradient` | `(surface_temp: f64, depth: f64, gradient: f64) -> f64` |
| `flexural_rigidity` | `(e: f64, te: f64, nu: f64) -> f64` |
| `elastic_thickness_from_rigidity` | `(d: f64, e: f64, nu: f64) -> f64` |

---

### volcanism (18 functions)

| Function | Signature |
|----------|-----------|
| `magma_viscosity` | `(a: f64, activation_energy: f64, temperature_k: f64) -> f64` |
| `magma_buoyancy_force` | `( rho_magma: f64, rho_crust: f64, g_surface: f64, chamber_volume: f64, ) -> f64` |
| `overpressure_threshold` | `( tensile_strength: f64, lithostatic_pressure: f64, ) -> f64` |
| `chamber_overpressure` | `( bulk_modulus: f64, volume_injected: f64, chamber_volume: f64, ) -> f64` |
| `effusion_rate` | `( overpressure: f64, conduit_radius: f64, conduit_length: f64, viscosity: f64, ) -> f64` |
| `volcanic_explosivity_index` | `(ejecta_volume_km3: f64) -> f64` |
| `lava_flow_velocity` | `( g_surface: f64, slope_angle: f64, thickness: f64, density: f64, viscosity: f64, ) -> f64` |
| `pyroclastic_column_height` | `(thermal_flux: f64) -> f64` |
| `seismic_to_eruption_probability` | `( cumulative_seismic_moment: f64, critical_moment: f64, b_value: f64, ) -> f64` |
| `tidal_triggering_stress` | `( mass_perturber: f64, distance: f64, body_radius: f64, ) -> f64` |
| `coulomb_failure_stress` | `( shear_stress: f64, normal_stress: f64, pore_pressure: f64, friction_coefficient: f64, cohesion: f64, ) -> f64` |
| `magma_ascent_velocity` | `( density_contrast: f64, g_surface: f64, conduit_radius: f64, viscosity: f64, ) -> f64` |
| `volatile_exsolution_depth` | `( saturation_pressure: f64, density: f64, g_surface: f64, ) -> f64` |
| `fragmentation_threshold` | `( porosity: f64, vesicularity_critical: f64, ) -> bool` |
| `tephra_fallout_distance` | `( column_height: f64, wind_speed: f64, particle_settling_velocity: f64, ) -> f64` |
| `thermal_erosion_rate` | `( lava_temperature: f64, substrate_solidus: f64, lava_velocity: f64, thermal_diffusivity: f64, ) -> f64` |
| `degassing_rate` | `( volatile_concentration: f64, solubility_coefficient: f64, pressure: f64, ) -> f64` |
| `eruption_energy` | `(ejecta_mass: f64, ejecta_velocity: f64, thermal_energy: f64) -> f64` |

---


## Hub dispatch mapping

All geology functions are wired through:

- `src/hub/engine/dispatch/geology.rs`
