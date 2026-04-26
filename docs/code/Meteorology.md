# Meteorology Source Code Guide

This page documents the source implementation behind `sciforge::meteorology`, including file layout and Hub dispatch wiring.

## Source Coverage

### What is explained
- File-level implementation layout in `src/meteorology/`.
- Main computation groups and where they are implemented.
- Runtime call path when meteorology functions are executed through Hub dispatch.

### Visibility and external access
- This domain module is `pub(crate)` in `src/lib.rs` and is not part of the external crate API.
- External consumers should use the public `sciforge::hub` API for these computations.
- Direct module paths shown here are for internal development and source-level understanding.


## Source Code Explanation

### Relevant file structure
- Main implementation: `src/meteorology/`
- Module entry point: `src/meteorology/mod.rs`
- Hub routing (when applicable): `src/hub/engine/dispatch/meteorology.rs`

### Internal execution flow
1. The module exposes subdomains through `mod.rs` and targeted `pub use` exports.
2. Each `.rs` file implements a coherent block of equations and functions.
3. The Hub invokes these functions through the domain dispatcher when execution goes through `ExperimentRunner`.

### What to check while reading code
- Exact signature: input/output types and argument order.
- Numerical preconditions: divisions, roots, logarithms, and validity ranges.
- Implicit conventions: units, normalization, bounds, and tolerances.

## Modules

- `atmosphere` — atmospheric structure, thermodynamic properties
- `clouds` — cloud classification, formation, microphysics
- `dynamics` — mesoscale and synoptic dynamics, Coriolis effects
- `ocean` — ocean-atmosphere coupling, SST, thermohaline circulation
- `precipitation` — rainfall statistics, evapotranspiration, runoff
- `radiation` — solar and terrestrial radiation, climate forcing
- `storms` — cyclone intensity, storm surge, convective dynamics
- `winds` — wind profiles, Beaufort scale, turbulence

---

### atmosphere (17 functions)

| Function | Signature |
|----------|-----------|
| `barometric_formula` | `(p0: f64, m: f64, g: f64, h: f64, t: f64) -> f64` |
| `scale_height` | `(t: f64, m: f64, g: f64) -> f64` |
| `lapse_rate_dry` | `() -> f64` |
| `lapse_rate_moist` | `(t: f64, l_v: f64, r_s: f64) -> f64` |
| `potential_temperature` | `(t: f64, p: f64, p0: f64) -> f64` |
| `virtual_temperature` | `(t: f64, r: f64) -> f64` |
| `mixing_ratio` | `(e: f64, p: f64) -> f64` |
| `saturation_vapor_pressure` | `(t_celsius: f64) -> f64` |
| `relative_humidity` | `(e: f64, es: f64) -> f64` |
| `dew_point` | `(t: f64, rh: f64) -> f64` |
| `density_altitude` | `(pressure_altitude: f64, temperature_c: f64) -> f64` |
| `brunt_vaisala_frequency` | `(g: f64, theta: f64, dtheta_dz: f64) -> f64` |
| `mie_phase` | `(cos_theta: f64, g: f64) -> f64` |
| `lifted_condensation_level` | `(t_surface: f64, dew_point: f64) -> f64` |
| `dry_adiabatic_temperature` | `(t_surface: f64, altitude: f64) -> f64` |
| `convective_available_potential_energy` | `( t_parcel: &[f64], t_env: &[f64], dz: f64, ) -> f64` |
| `convective_inhibition` | `( t_parcel: &[f64], t_env: &[f64], dz: f64, ) -> f64` |

---

### clouds (21 functions)

| Function | Signature |
|----------|-----------|
| `saturation_vapor_pressure` | `(temperature_k: f64) -> f64` |
| `cloud_base_altitude` | `( surface_temperature: f64, dew_point: f64, lapse_rate: f64, ) -> f64` |
| `adiabatic_liquid_water_content` | `( altitude_above_base: f64, lapse_rate: f64, temperature_at_base: f64, ) -> f64` |
| `cloud_optical_depth` | `( liquid_water_path: f64, effective_radius: f64, ) -> f64` |
| `cloud_albedo` | `(optical_depth: f64) -> f64` |
| `droplet_growth_condensation` | `( supersaturation: f64, radius: f64, temperature: f64, pressure: f64, ) -> f64` |
| `droplet_growth_collision` | `( collector_radius: f64, collected_radius: f64, liquid_water_content: f64, collection_efficiency: f64, density_water: f64, terminal_velocity_diff: f64, ) -> f64` |
| `autoconversion_rate` | `( lwc: f64, droplet_concentration: f64, threshold_lwc: f64, ) -> f64` |
| `ice_crystal_growth_rate` | `( temperature_k: f64, supersaturation_ice: f64, ) -> f64` |
| `bergeron_process_rate` | `( ice_crystal_density: f64, lwc: f64, temperature_k: f64, ) -> f64` |
| `cloud_layer_temperature` | `( base_temperature: f64, lapse_rate: f64, altitude_above_base: f64, ) -> f64` |
| `mie_size_parameter` | `( particle_radius: f64, wavelength: f64, ) -> f64` |
| `extinction_efficiency` | `(size_parameter: f64) -> f64` |
| `cloud_emissivity` | `(optical_depth: f64) -> f64` |
| `convective_available_potential_energy` | `( parcel_temperatures: &[f64], env_temperatures: &[f64], altitudes: &[f64], g_local: f64, ) -> f64` |
| `convective_inhibition` | `( parcel_temperatures: &[f64], env_temperatures: &[f64], altitudes: &[f64], g_local: f64, ) -> f64` |
| `henyey_greenstein_phase` | `(cos_theta: f64, asymmetry: f64) -> f64` |
| `cloud_radiative_forcing` | `( cloud_albedo: f64, surface_albedo: f64, solar_flux: f64, cloud_fraction: f64, longwave_trapping: f64, ) -> f64` |
| `nucleation_rate_ccn` | `( supersaturation: f64, ccn_concentration: f64, k_exponent: f64, ) -> f64` |
| `rain_evaporation_rate` | `( rain_rate: f64, relative_humidity: f64, temperature: f64, ) -> f64` |
| `terminal_velocity_droplet` | `(radius_m: f64) -> f64` |

---

### dynamics (10 functions)

| Function | Signature |
|----------|-----------|
| `coriolis_parameter` | `(latitude: f64, omega: f64) -> f64` |
| `geostrophic_wind` | `(dp_dx: f64, dp_dy: f64, rho: f64, f: f64) -> (f64, f64)` |
| `rossby_number` | `(u: f64, l: f64, f: f64) -> f64` |
| `rossby_wave_speed` | `(beta: f64, k: f64) -> f64` |
| `thermal_wind` | `(f: f64, delta_t: f64, delta_x: f64, t_mean: f64) -> f64` |
| `potential_vorticity` | `(f: f64, dtheta_dp: f64) -> f64` |
| `ekman_depth` | `(nu: f64, f: f64) -> f64` |
| `richardson_number` | `(n2: f64, du_dz: f64) -> f64` |
| `rossby_deformation_radius` | `(n: f64, h: f64, f: f64) -> f64` |
| `cyclone_gradient_wind` | `(r: f64, f: f64, dp_dr: f64, rho: f64) -> f64` |

---

### ocean (26 functions)

| Function | Signature |
|----------|-----------|
| `deep_water_phase_speed` | `(g_local: f64, wavelength: f64) -> f64` |
| `shallow_water_phase_speed` | `(g_local: f64, depth: f64) -> f64` |
| `wave_dispersion` | `(g_local: f64, k: f64, depth: f64) -> f64` |
| `phillips_spectrum` | `( k: f64, wind_speed: f64, g_local: f64, wind_direction_x: f64, wind_direction_y: f64, kx: f64, ky: f64, ) -> f64` |
| `jonswap_spectrum` | `( omega: f64, wind_speed: f64, fetch: f64, g_local: f64, ) -> f64` |
| `significant_wave_height` | `( wind_speed: f64, fetch: f64, g_local: f64, ) -> f64` |
| `wave_period` | `( wind_speed: f64, fetch: f64, g_local: f64, ) -> f64` |
| `stokes_drift` | `( amplitude: f64, omega: f64, k: f64, depth: f64, z: f64, ) -> f64` |
| `ekman_transport` | `( wind_stress: f64, coriolis_parameter: f64, density: f64, ) -> f64` |
| `ekman_spiral_velocity` | `( surface_current: f64, ekman_depth: f64, z: f64, ) -> (f64, f64)` |
| `thermohaline_density` | `( rho_0: f64, alpha_t: f64, beta_s: f64, temperature: f64, t_ref: f64, salinity: f64, s_ref: f64, ) -> f64` |
| `overturning_stream_function` | `( thermal_forcing: f64, salinity_forcing: f64, alpha_t: f64, beta_s: f64, kappa: f64, depth: f64, ) -> f64` |
| `mixed_layer_depth` | `( wind_stress: f64, buoyancy_flux: f64, coriolis: f64, ) -> f64` |
| `tidal_amplitude` | `( mass_perturber: f64, distance: f64, body_radius: f64, ocean_depth: f64, g_surface: f64, ) -> f64` |
| `geostrophic_current` | `( g_local: f64, sea_surface_slope: f64, coriolis: f64, ) -> f64` |
| `upwelling_velocity` | `( wind_stress_curl: f64, coriolis: f64, density: f64, ) -> f64` |
| `ocean_heat_content` | `( density: f64, specific_heat: f64, delta_t: f64, depth: f64, area: f64, ) -> f64` |
| `sea_ice_growth_rate` | `( thermal_conductivity_ice: f64, ice_thickness: f64, surface_temp: f64, freezing_point: f64, latent_heat: f64, ice_density: f64, ) -> f64` |
| `internal_wave_speed` | `( g_local: f64, density_upper: f64, density_lower: f64, depth_upper: f64, depth_lower: f64, ) -> f64` |
| `langmuir_circulation_depth` | `( wind_speed: f64, surface_current: f64, ) -> f64` |
| `seawater_density` | `(t: f64, s: f64) -> f64` |
| `sound_speed` | `(t: f64, s: f64, d: f64) -> f64` |
| `sea_level_rise_thermal` | `(alpha: f64, delta_t: f64, d: f64) -> f64` |
| `henry_solubility` | `(kh: f64, t: f64, delta_h: f64) -> f64` |
| `haline_contraction_coefficient` | `(salinity_psu: f64) -> f64` |
| `thermal_expansion_coefficient_sw` | `(temperature_c: f64) -> f64` |

---

### precipitation (10 functions)

| Function | Signature |
|----------|-----------|
| `rain_rate_marshall_palmer` | `(z: f64) -> f64` |
| `radar_reflectivity` | `(rain_rate: f64) -> f64` |
| `terminal_velocity_raindrop` | `(diameter_mm: f64) -> f64` |
| `thornthwaite_pet` | `(t_mean: f64, heat_index: f64, day_length_hours: f64) -> f64` |
| `penman_evaporation` | `(delta: f64, rn: f64, gamma: f64, ea: f64, u: f64) -> f64` |
| `intensity_duration_frequency` | `(a: f64, b: f64, duration: f64, return_period: f64) -> f64` |
| `scs_curve_number_runoff` | `(p: f64, cn: f64) -> f64` |
| `rational_method_runoff` | `(c: f64, i: f64, a: f64) -> f64` |
| `unit_hydrograph_peak` | `(a: f64, tp: f64) -> f64` |
| `antecedent_precipitation_index` | `(prev_api: f64, rainfall: f64, k: f64) -> f64` |

---

### radiation (12 functions)

| Function | Signature |
|----------|-----------|
| `stefan_boltzmann_flux` | `(t: f64) -> f64` |
| `solar_constant` | `() -> f64` |
| `albedo_reflected` | `(solar_flux: f64, albedo: f64) -> f64` |
| `effective_temperature` | `(solar_flux: f64, albedo: f64) -> f64` |
| `greenhouse_effect` | `(t_surface: f64, t_effective: f64) -> f64` |
| `optical_depth` | `(absorption_coeff: f64, path_length: f64) -> f64` |
| `beer_lambert` | `(i0: f64, tau: f64) -> f64` |
| `planck_function` | `(wavelength: f64, t: f64) -> f64` |
| `solar_zenith_angle` | `(lat: f64, declination: f64, hour_angle: f64) -> f64` |
| `radiative_forcing_co2` | `(c: f64, c0: f64) -> f64` |
| `climate_sensitivity` | `(delta_t: f64, delta_f: f64) -> f64` |
| `outgoing_longwave_radiation` | `(t: f64, emissivity: f64) -> f64` |

---

### storms (5 functions)

| Function | Signature |
|----------|-----------|
| `potential_intensity` | `(ck: f64, cd: f64, eta: f64, delta_k: f64) -> f64` |
| `accumulated_cyclone_energy` | `(v_kt_series: &[f64]) -> f64` |
| `cape` | `(t_parcel: f64, t_env: f64, g: f64, dz: f64) -> f64` |
| `rossby_deformation_radius` | `(n: f64, h: f64, f: f64) -> f64` |
| `fujita_scale` | `(v: f64) -> u8` |

---

### winds (21 functions)

| Function | Signature |
|----------|-----------|
| `hadley_cell_extent` | `(planet_radius: f64, rotation_rate: f64, delta_t: f64) -> f64` |
| `thermal_wind_shear` | `( g_local: f64, coriolis: f64, temperature: f64, dt_dy: f64, ) -> f64` |
| `jet_stream_velocity` | `( g_local: f64, delta_t: f64, meridional_distance: f64, coriolis: f64, scale_height: f64, ) -> f64` |
| `surface_wind_speed` | `( pressure_gradient: f64, density: f64, drag_coefficient: f64, ) -> f64` |
| `wind_stress` | `( air_density: f64, drag_coefficient: f64, wind_speed: f64, ) -> f64` |
| `planetary_vorticity` | `(rotation_rate: f64, latitude: f64) -> f64` |
| `rossby_wave_phase_speed` | `( beta: f64, zonal_wavenumber: f64, deformation_radius: f64, ) -> f64` |
| `beta_parameter` | `(rotation_rate: f64, planet_radius: f64, latitude: f64) -> f64` |
| `baroclinic_instability_wavelength` | `( deformation_radius: f64, ) -> f64` |
| `sea_breeze_speed` | `( g_local: f64, delta_t: f64, boundary_layer_height: f64, mean_temperature: f64, ) -> f64` |
| `katabatic_wind_speed` | `( g_local: f64, delta_t: f64, mean_temperature: f64, slope_angle: f64, slope_length: f64, drag_coefficient: f64, ) -> f64` |
| `mountain_wave_vertical_velocity` | `( wind_speed: f64, mountain_height: f64, brunt_vaisala: f64, horizontal_wavelength: f64, ) -> f64` |
| `ekman_pumping_velocity` | `( wind_stress_curl: f64, density: f64, coriolis: f64, ) -> f64` |
| `monin_obukhov_length` | `( friction_velocity: f64, surface_temp: f64, sensible_heat_flux: f64, air_density: f64, specific_heat: f64, ) -> f64` |
| `log_wind_profile` | `( friction_velocity: f64, z: f64, roughness_length: f64, ) -> f64` |
| `cyclostrophic_wind` | `( pressure_gradient: f64, density: f64, radius: f64, ) -> f64` |
| `gradient_wind` | `( coriolis: f64, pressure_gradient: f64, density: f64, radius: f64, ) -> f64` |
| `superrotation_index` | `( zonal_wind: f64, planet_radius: f64, rotation_rate: f64, latitude: f64, ) -> f64` |
| `foehn_warming` | `( lapse_dry: f64, lapse_moist: f64, mountain_height: f64, condensation_level: f64, ) -> f64` |
| `beaufort_to_m_s` | `(b: f64) -> f64` |
| `wind_chill` | `(t: f64, v: f64) -> f64` |

---


## Hub dispatch mapping

All meteorology functions are wired through:

- `src/hub/engine/dispatch/meteorology.rs`
