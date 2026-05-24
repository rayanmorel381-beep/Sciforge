# Astronomy Source Code Guide

This page documents the source implementation behind `sciforge::astronomy`, including file layout and Hub dispatch wiring.

## Source Coverage

### What is explained
- File-level implementation layout in `src/astronomy/`.
- Main computation groups and where they are implemented.
- Runtime call path when astronomy functions are executed through Hub dispatch.

### Visibility and external access
- This domain module is `pub(crate)` in `src/lib.rs` and is not part of the external crate API.
- External consumers should use the public `sciforge::hub` API for these computations.
- Direct module paths shown here are for internal development and source-level understanding.


## Source Code Explanation

### Relevant file structure
- Main implementation: `src/astronomy/`
- Module entry point: `src/astronomy/mod.rs`
- Hub routing: `src/hub/engine/dispatch/astronomy.rs`

### Internal execution flow
1. The module exposes subdomains through `mod.rs` and targeted `pub use` exports.
2. Each `.rs` file implements a coherent block of equations and functions.
3. The Hub invokes these functions through the domain dispatcher when execution goes through `ExperimentRunner`.

### What to check while reading code
- Exact signature: input/output types and argument order.
- Unit assumptions (especially distances, velocities, and Hubble constants).
- Domain constraints (e.g. non-negative redshift, valid density parameters).

## Modules

- `celestial`
- `cosmology`
- `galactic`
- `impacts`
- `orbits`
- `planetary`
- `rotation`
- `stellar`
- `tides`

---

## Celestial API (43 functions, 1 struct)

### `MoonData` struct

| Field | Type |
|-------|------|
| `mass` | `f64` |
| `radius` | `f64` |

### Functions

| Function | Signature |
|----------|-----------|
| `gravitational_force` | `(m1: f64, m2: f64, r: f64) -> f64` |
| `tidal_force` | `(m: f64, r: f64, delta_r: f64) -> f64` |
| `synodic_period` | `(p1: f64, p2: f64) -> f64` |
| `apparent_angular_size` | `(diameter: f64, distance: f64) -> f64` |
| `parallax_distance` | `(parallax_arcsec: f64) -> f64` |
| `barycenter` | `(m1: f64, m2: f64, d: f64) -> f64` |
| `lagrange_l1` | `(d: f64, m1: f64, m2: f64) -> f64` |
| `hill_sphere` | `(a: f64, m: f64, m_star: f64, e: f64) -> f64` |
| `surface_gravity` | `(m: f64, r: f64) -> f64` |
| `sidereal_to_solar` | `(sidereal_period: f64, orbital_period: f64) -> f64` |
| `habitable_zone_inner` | `(luminosity_solar: f64) -> f64` |
| `habitable_zone_outer` | `(luminosity_solar: f64) -> f64` |
| `julian_date_to_j2000_century` | `(jd: f64) -> f64` |
| `j2000_century_to_julian_date` | `(t: f64) -> f64` |
| `mean_obliquity_ecliptic` | `(t_century: f64) -> f64` |
| `nutation_longitude` | `(omega: f64) -> f64` |
| `precession_rate_arcsec_per_year` | `(t_century: f64) -> f64` |
| `equation_of_time` | `(day_of_year: f64) -> f64` |
| `sidereal_year_seconds` | `() -> f64` |
| `tropical_year_seconds` | `() -> f64` |
| `precession_period` | `() -> f64` |
| `day_length_seconds` | `() -> f64` |
| `solar_day_to_sidereal_day` | `(solar_day_s: f64, orbital_period_s: f64) -> f64` |
| `degrees_to_radians` | `(deg: f64) -> f64` |
| `radians_to_degrees` | `(rad: f64) -> f64` |
| `tidal_dissipation_factor` | `() -> f64` |
| `tidal_quality_factor` | `(specific_dissipation: f64) -> f64` |
| `tidal_heating` | `(mass_primary: f64, radius: f64, eccentricity: f64, a: f64, n: f64) -> f64` |
| `au_to_meters` | `(au: f64) -> f64` |
| `meters_to_au` | `(m: f64) -> f64` |
| `light_years_to_meters` | `(ly: f64) -> f64` |
| `meters_to_light_years` | `(m: f64) -> f64` |
| `au_to_light_years` | `(au: f64) -> f64` |
| `light_years_to_au` | `(ly: f64) -> f64` |
| `earth_moon_distance` | `() -> f64` |
| `lunar_orbital_period` | `() -> f64` |
| `moon_data` | `(name: &str) -> Option<MoonData>` |
| `moon_mass` | `(name: &str) -> Option<f64>` |
| `moon_radius` | `(name: &str) -> Option<f64>` |
| `moon_surface_gravity` | `(name: &str) -> Option<f64>` |
| `moon_escape_velocity` | `(name: &str) -> Option<f64>` |
| `moon_volume` | `(name: &str) -> Option<f64>` |
| `moon_mean_density` | `(name: &str) -> Option<f64>` |

---

## Cosmology API (70 functions)

| Function | Signature |
|----------|-----------|
| `hubble_velocity` | `(h0: f64, distance: f64) -> f64` |
| `hubble_distance` | `(h0: f64, velocity: f64) -> f64` |
| `redshift_velocity` | `(v: f64, c: f64) -> f64` |
| `relativistic_redshift` | `(v: f64) -> f64` |
| `velocity_from_redshift` | `(z: f64) -> f64` |
| `cosmological_redshift` | `(a_emit: f64, a_obs: f64) -> f64` |
| `scale_factor` | `(redshift: f64) -> f64` |
| `critical_density` | `(h: f64) -> f64` |
| `critical_density_from_h0` | `(h0: f64) -> f64` |
| `friedmann_expansion` | `(h0: f64, omega_m: f64, omega_r: f64, omega_lambda: f64, a: f64) -> f64` |
| `lookback_time` | `(z: f64, h0: f64) -> f64` |
| `luminosity_distance` | `(comoving_distance: f64, z: f64) -> f64` |
| `angular_diameter_distance` | `(comoving_distance: f64, z: f64) -> f64` |
| `cmb_temperature` | `(t0: f64, z: f64) -> f64` |
| `age_of_universe` | `(h0: f64) -> f64` |
| `e_z` | `(omega_m: f64, omega_r: f64, omega_k: f64, omega_de: f64, z: f64) -> f64` |
| `e_z_lcdm` | `(omega_m: f64, z: f64) -> f64` |
| `e_z_lcdm_rad` | `(omega_m: f64, omega_r: f64, z: f64) -> f64` |
| `e_z_wcdm` | `(omega_m: f64, omega_de: f64, w: f64, z: f64) -> f64` |
| `e_z_w0wa` | `(omega_m: f64, omega_de: f64, w0: f64, wa: f64, z: f64) -> f64` |
| `hubble_at_z` | `(h0: f64, omega_m: f64, omega_r: f64, omega_k: f64, omega_de: f64, z: f64) -> f64` |
| `hubble_at_z_lcdm` | `(h0: f64, omega_m: f64, z: f64) -> f64` |
| `deceleration_parameter` | `(omega_m: f64, z: f64) -> f64` |
| `deceleration_parameter_today` | `(omega_m: f64) -> f64` |
| `hubble_distance_mpc` | `(h0: f64) -> f64` |
| `hubble_time_gyr` | `(h0: f64) -> f64` |
| `omega_k_from` | `(omega_m: f64, omega_r: f64, omega_de: f64) -> f64` |
| `little_h` | `(h0: f64) -> f64` |
| `comoving_distance_from_z` | `(h0: f64, omega_m: f64, z: f64) -> f64` |
| `luminosity_distance_from_z` | `(h0: f64, omega_m: f64, z: f64) -> f64` |
| `angular_diameter_distance_from_z` | `(h0: f64, omega_m: f64, z: f64) -> f64` |
| `distance_modulus_from_z` | `(h0: f64, omega_m: f64, z: f64) -> f64` |
| `comoving_distance_general` | `(h0: f64, omega_m: f64, omega_r: f64, omega_k: f64, omega_de: f64, z: f64) -> f64` |
| `transverse_comoving_distance` | `(h0: f64, omega_m: f64, omega_r: f64, omega_k: f64, omega_de: f64, z: f64) -> f64` |
| `luminosity_distance_general` | `(h0: f64, omega_m: f64, omega_r: f64, omega_k: f64, omega_de: f64, z: f64) -> f64` |
| `angular_diameter_distance_general` | `(h0: f64, omega_m: f64, omega_r: f64, omega_k: f64, omega_de: f64, z: f64) -> f64` |
| `distance_modulus_general` | `(h0: f64, omega_m: f64, omega_r: f64, omega_k: f64, omega_de: f64, z: f64) -> f64` |
| `comoving_distance_wcdm` | `(h0: f64, omega_m: f64, omega_de: f64, w: f64, z: f64) -> f64` |
| `luminosity_distance_wcdm` | `(h0: f64, omega_m: f64, omega_de: f64, w: f64, z: f64) -> f64` |
| `comoving_distance_w0wa` | `(h0: f64, omega_m: f64, omega_de: f64, w0: f64, wa: f64, z: f64) -> f64` |
| `luminosity_distance_w0wa` | `(h0: f64, omega_m: f64, omega_de: f64, w0: f64, wa: f64, z: f64) -> f64` |
| `luminosity_from_angular_diameter` | `(d_a: f64, z: f64) -> f64` |
| `angular_diameter_from_luminosity` | `(d_l: f64, z: f64) -> f64` |
| `proper_distance` | `(comoving_d: f64, z: f64) -> f64` |
| `lookback_time_from_z` | `(h0: f64, omega_m: f64, z: f64) -> f64` |
| `lookback_time_general` | `(h0: f64, omega_m: f64, omega_r: f64, omega_k: f64, omega_de: f64, z: f64) -> f64` |
| `age_at_z` | `(h0: f64, omega_m: f64, z: f64) -> f64` |
| `age_lcdm` | `(h0: f64, omega_m: f64) -> f64` |
| `age_general` | `(h0: f64, omega_m: f64, omega_r: f64, omega_k: f64, omega_de: f64) -> f64` |
| `acceleration_redshift` | `(omega_m: f64) -> f64` |
| `matter_radiation_equality_z` | `(omega_m: f64, omega_r: f64) -> f64` |
| `particle_horizon` | `(h0: f64, omega_m: f64) -> f64` |
| `particle_horizon_general` | `(h0: f64, omega_m: f64, omega_r: f64, omega_k: f64, omega_de: f64) -> f64` |
| `sound_horizon_eh98` | `(omega_m: f64, omega_b: f64, h0: f64) -> f64` |
| `event_horizon` | `(h0: f64, omega_m: f64) -> f64` |
| `comoving_volume_element` | `(h0: f64, omega_m: f64, z: f64) -> f64` |
| `comoving_volume_total` | `(h0: f64, omega_m: f64, z: f64) -> f64` |
| `comoving_volume_shell` | `(h0: f64, omega_m: f64, z1: f64, z2: f64) -> f64` |
| `matter_density_at_z` | `(h0: f64, omega_m: f64, z: f64) -> f64` |
| `radiation_density_at_z` | `(h0: f64, omega_r: f64, z: f64) -> f64` |
| `dark_energy_density_at_z` | `(h0: f64, omega_de: f64, w: f64, z: f64) -> f64` |
| `omega_m_at_z` | `(omega_m: f64, z: f64) -> f64` |
| `omega_de_at_z` | `(omega_m: f64, z: f64) -> f64` |
| `growth_factor_approx` | `(omega_m: f64, z: f64) -> f64` |
| `growth_rate` | `(omega_m: f64, z: f64) -> f64` |
| `sigma8_at_z` | `(sigma8: f64, omega_m: f64, z: f64) -> f64` |
| `cmb_temperature_at_z` | `(z: f64) -> f64` |
| `photon_number_density` | `(z: f64) -> f64` |
| `cmb_energy_density` | `(z: f64) -> f64` |
| `omega_r_from_tcmb` | `(t_cmb: f64, h0: f64) -> f64` |

---

## Galactic API (21 functions)

| Function | Signature |
|----------|-----------|
| `galactic_rotation_velocity` | `(r: f64) -> f64` |
| `sun_orbital_period` | `() -> f64` |
| `sun_orbital_velocity` | `() -> f64` |
| `sun_galactocentric_distance` | `() -> f64` |
| `galactic_mass_within_radius` | `(r: f64) -> f64` |
| `sgr_a_schwarzschild_radius` | `() -> f64` |
| `sgr_a_sphere_of_influence` | `() -> f64` |
| `sgr_a_distance` | `() -> f64` |
| `m31_approach_time` | `() -> f64` |
| `m31_reduced_mass` | `() -> f64` |
| `hubble_recession_velocity` | `(distance_mpc: f64) -> f64` |
| `galactic_disk_density` | `(r: f64, z: f64) -> f64` |
| `galactic_escape_velocity` | `(r: f64) -> f64` |
| `galactic_circular_velocity` | `(r: f64) -> f64` |
| `galactic_dynamical_time` | `(r: f64) -> f64` |
| `stellar_number_density` | `(r: f64, z: f64) -> f64` |
| `oort_a_constant` | `(r: f64, dr: f64) -> f64` |
| `oort_b_constant` | `(r: f64, dr: f64) -> f64` |
| `epicyclic_frequency` | `(r: f64, dr: f64) -> f64` |
| `bulge_mass_within` | `(r: f64) -> f64` |
| `tidal_radius` | `(m_cluster: f64, r_galactic: f64) -> f64` |

---

## Impacts API (5 functions)

| Function | Signature |
|----------|-----------|
| `crater_diameter` | `(rho_i: f64, d_p: f64, v: f64, g: f64, rho_t: f64) -> f64` |
| `fireball_radius` | `(e_kt: f64) -> f64` |
| `ejecta_volume` | `(d: f64, depth: f64) -> f64` |
| `impact_velocity` | `(v_inf: f64, v_esc: f64) -> f64` |
| `ejecta_escape_fraction` | `(v_esc: f64, v_ejecta: f64) -> f64` |

---

## Orbits API (20 functions)

| Function | Signature |
|----------|-----------|
| `kepler_period` | `(a: f64, mu: f64) -> f64` |
| `kepler_velocity` | `(mu: f64, r: f64, a: f64) -> f64` |
| `circular_velocity` | `(mu: f64, r: f64) -> f64` |
| `escape_velocity` | `(mu: f64, r: f64) -> f64` |
| `vis_viva` | `(mu: f64, r: f64, a: f64) -> f64` |
| `orbital_energy` | `(mu: f64, a: f64) -> f64` |
| `angular_momentum` | `(mu: f64, a: f64, e: f64) -> f64` |
| `periapsis` | `(a: f64, e: f64) -> f64` |
| `apoapsis` | `(a: f64, e: f64) -> f64` |
| `true_anomaly_to_radius` | `(a: f64, e: f64, theta: f64) -> f64` |
| `hohmann_delta_v` | `(mu: f64, r1: f64, r2: f64) -> f64` |
| `sphere_of_influence` | `(a: f64, m_planet: f64, m_star: f64) -> f64` |
| `roche_limit` | `(r_primary: f64, rho_primary: f64, rho_secondary: f64) -> f64` |
| `solve_kepler` | `(m: f64, e: f64, tol: f64) -> f64` |
| `gr_perihelion_precession` | `(mass: f64, a: f64, e: f64) -> f64` |
| `j2_value` | `(body: &str) -> Option<f64>` |
| `j2_nodal_regression` | `(j2: f64, r_body: f64, a: f64, e: f64, i: f64, n: f64) -> f64` |
| `j2_apsidal_precession` | `(j2: f64, r_body: f64, a: f64, e: f64, i: f64, n: f64) -> f64` |
| `yoshida4_step` | `(q: &mut [f64], p: &mut [f64], dt: f64, force: &dyn Fn(&[f64]) -> Vec<f64>)` |
| `yoshida4_weights` | `() -> (f64, f64)` |

---

## Planetary API (48 functions, 1 struct)

### `PlanetData` struct

| Field | Type |
|-------|------|
| `mass` | `f64` |
| `radius` | `f64` |
| `flattening` | `f64` |
| `orbital_period` | `f64` |
| `semi_major_axis` | `f64` |
| `eccentricity` | `f64` |
| `inclination` | `f64` |
| `axial_tilt` | `f64` |
| `sidereal_day` | `f64` |
| `surface_gravity` | `f64` |
| `escape_velocity` | `f64` |
| `mean_density` | `f64` |
| `bond_albedo` | `f64` |
| `orbital_velocity` | `f64` |

### Functions

| Function | Signature |
|----------|-----------|
| `hydrostatic_pressure` | `(density: f64, g_surface: f64, depth: f64) -> f64` |
| `central_pressure` | `(density: f64, g_surface: f64, radius: f64) -> f64` |
| `adiabatic_temperature_gradient` | `(g_local: f64, specific_heat: f64) -> f64` |
| `planetary_moment_of_inertia_factor` | `(core_density: f64, mantle_density: f64, core_radius: f64, total_radius: f64) -> f64` |
| `love_number_k2` | `(rigidity: f64, density: f64, radius: f64) -> f64` |
| `tidal_heating` | `(radius: f64, eccentricity: f64, mean_motion: f64, k2: f64, tidal_q: f64, perturber_mass: f64, semi_major_axis: f64) -> f64` |
| `tidal_locking_timescale` | `(mass: f64, radius: f64, semi_major_axis: f64, perturber_mass: f64, tidal_q: f64, rigidity: f64, initial_spin: f64) -> f64` |
| `roche_limit_fluid` | `(primary_radius: f64, primary_density: f64, secondary_density: f64) -> f64` |
| `roche_limit_rigid` | `(primary_radius: f64, primary_density: f64, secondary_density: f64) -> f64` |
| `equilibrium_temperature` | `(stellar_luminosity: f64, semi_major_axis: f64, albedo: f64) -> f64` |
| `greenhouse_surface_temperature` | `(equilibrium_temp: f64, optical_depth_ir: f64) -> f64` |
| `scale_height` | `(temperature: f64, mean_molecular_mass: f64, g_surface: f64) -> f64` |
| `atmospheric_escape_jeans` | `(temperature: f64, molecular_mass: f64, escape_velocity: f64) -> f64` |
| `magnetopause_standoff` | `(dipole_moment: f64, solar_wind_pressure: f64) -> f64` |
| `ring_roche_inner` | `(planet_mass: f64, planet_radius: f64, particle_density: f64) -> f64` |
| `ring_optical_depth` | `(surface_density: f64, particle_radius: f64, particle_density: f64) -> f64` |
| `synchronous_orbit_radius` | `(mass: f64, rotation_period: f64) -> f64` |
| `oblateness` | `(rotation_rate: f64, equatorial_radius: f64, mass: f64) -> f64` |
| `precession_rate` | `(oblateness_j2: f64, planet_radius: f64, semi_major_axis: f64, mean_motion: f64) -> f64` |
| `bond_albedo_from_geometric` | `(geometric_albedo: f64, phase_integral: f64) -> f64` |
| `thermal_inertia` | `(thermal_conductivity: f64, density: f64, specific_heat: f64) -> f64` |
| `diurnal_skin_depth` | `(thermal_diffusivity: f64, rotation_period: f64) -> f64` |
| `subsolar_temperature` | `(stellar_flux: f64, albedo: f64, emissivity: f64) -> f64` |
| `nightside_temperature` | `(thermal_inertia_val: f64, subsolar_temp: f64, rotation_period: f64) -> f64` |
| `sputtering_loss_rate` | `(stellar_wind_flux: f64, sputtering_yield: f64, cross_section: f64) -> f64` |
| `hill_sphere_atmospheric` | `(planet_mass: f64, stellar_mass: f64, semi_major_axis: f64) -> f64` |
| `planet_data` | `(name: &str) -> Option<PlanetData>` |
| `planet_mass` | `(name: &str) -> Option<f64>` |
| `planet_radius` | `(name: &str) -> Option<f64>` |
| `planet_flattening` | `(name: &str) -> Option<f64>` |
| `planet_orbital_period` | `(name: &str) -> Option<f64>` |
| `planet_semi_major_axis` | `(name: &str) -> Option<f64>` |
| `planet_eccentricity` | `(name: &str) -> Option<f64>` |
| `planet_inclination` | `(name: &str) -> Option<f64>` |
| `planet_axial_tilt` | `(name: &str) -> Option<f64>` |
| `planet_sidereal_day` | `(name: &str) -> Option<f64>` |
| `planet_surface_gravity` | `(name: &str) -> Option<f64>` |
| `planet_escape_velocity` | `(name: &str) -> Option<f64>` |
| `planet_mean_density` | `(name: &str) -> Option<f64>` |
| `planet_bond_albedo` | `(name: &str) -> Option<f64>` |
| `planet_orbital_velocity` | `(name: &str) -> Option<f64>` |
| `planet_volume` | `(name: &str) -> Option<f64>` |
| `planet_circumference` | `(name: &str) -> Option<f64>` |
| `planet_surface_area` | `(name: &str) -> Option<f64>` |
| `planet_gravitational_parameter` | `(name: &str) -> Option<f64>` |
| `planet_hill_sphere` | `(name: &str, stellar_mass: f64) -> Option<f64>` |
| `planet_roche_limit` | `(name: &str, secondary_density: f64) -> Option<f64>` |
| `planet_synchronous_orbit` | `(name: &str) -> Option<f64>` |

---

## Rotation API (7 functions)

| Function | Signature |
|----------|-----------|
| `surface_velocity_at_latitude` | `(omega: f64, r: f64, phi: f64) -> f64` |
| `centripetal_acceleration` | `(omega: f64, r: f64, phi: f64) -> f64` |
| `coriolis_parameter` | `(omega: f64, phi: f64) -> f64` |
| `moment_of_inertia` | `(c_factor: f64, m: f64, r: f64) -> f64` |
| `rotational_kinetic_energy` | `(inertia: f64, omega: f64) -> f64` |
| `nutation_obliquity_rad` | `(omega_node: f64) -> f64` |
| `day_length_variation` | `(doy: f64, latitude: f64, tilt: f64) -> f64` |

---

## Stellar API (49 functions)

| Function | Signature |
|----------|-----------|
| `luminosity_from_radius_temp` | `(r: f64, t: f64) -> f64` |
| `absolute_magnitude` | `(apparent_mag: f64, distance_pc: f64) -> f64` |
| `distance_modulus` | `(apparent_mag: f64, absolute_mag: f64) -> f64` |
| `stellar_flux` | `(luminosity: f64, distance: f64) -> f64` |
| `wien_peak_wavelength` | `(temperature: f64) -> f64` |
| `main_sequence_lifetime` | `(mass_solar: f64, luminosity_solar: f64) -> f64` |
| `mass_luminosity_relation` | `(mass_solar: f64) -> f64` |
| `schwarzschild_radius` | `(mass: f64) -> f64` |
| `chandrasekhar_limit` | `() -> f64` |
| `chandrasekhar_limit_kg` | `() -> f64` |
| `is_above_chandrasekhar` | `(mass_kg: f64) -> bool` |
| `eddington_luminosity` | `(mass: f64) -> f64` |
| `spectral_type_temperature` | `(spectral_index: f64) -> f64` |
| `bolometric_correction` | `(t_eff: f64) -> f64` |
| `tov_limit` | `() -> f64` |
| `neutron_star_surface_gravity` | `() -> f64` |
| `neutron_star_mean_density` | `() -> f64` |
| `neutron_star_escape_velocity` | `() -> f64` |
| `pulsar_spindown_luminosity` | `(period: f64, period_dot: f64) -> f64` |
| `pulsar_magnetic_field` | `(period: f64, period_dot: f64) -> f64` |
| `pulsar_characteristic_age` | `(period: f64, period_dot: f64) -> f64` |
| `pulsar_death_line_period` | `(b_field: f64) -> f64` |
| `magnetar_energy_reservoir` | `(b_field: f64) -> f64` |
| `magnetar_typical_energy` | `() -> f64` |
| `radiation_pressure` | `(temperature: f64) -> f64` |
| `gas_pressure` | `(rho: f64, temperature: f64, mu: f64) -> f64` |
| `adiabatic_sound_speed` | `(temperature: f64, mu: f64) -> f64` |
| `pp_chain_luminosity` | `(mass_kg: f64, x_hydrogen: f64) -> f64` |
| `cno_cycle_luminosity` | `(mass_kg: f64, x_hydrogen: f64, z_metals: f64) -> f64` |
| `kelvin_helmholtz_timescale` | `(mass: f64, radius: f64, luminosity: f64) -> f64` |
| `nuclear_timescale` | `(mass: f64, luminosity: f64) -> f64` |
| `white_dwarf_radius_from_mass` | `(mass_kg: f64) -> f64` |
| `eddington_luminosity_numerical` | `(mass_solar: f64) -> f64` |
| `helium_mass` | `() -> f64` |
| `solar_composition` | `() -> (f64, f64, f64)` |
| `solar_effective_temperature` | `() -> f64` |
| `solar_absolute_magnitude` | `() -> f64` |
| `solar_metallicity` | `() -> f64` |
| `solar_main_sequence_lifetime` | `() -> f64` |
| `hydrogen_burning_energy_per_kg` | `() -> f64` |
| `eddington_ratio` | `(luminosity: f64, mass: f64) -> f64` |
| `sun_core_temperature` | `() -> f64` |
| `sun_surface_temperature` | `() -> f64` |
| `sun_core_density` | `() -> f64` |
| `sun_age` | `() -> f64` |
| `sun_rotation_period` | `() -> f64` |
| `solar_density` | `() -> f64` |
| `solar_surface_gravity` | `() -> f64` |
| `sun_luminosity_to_mass_ratio` | `() -> f64` |

---

## Tides API (6 functions)

| Function | Signature |
|----------|-----------|
| `tidal_potential` | `(m: f64, r: f64, d: f64, k2: f64, theta: f64) -> f64` |
| `tidal_bulge_height` | `(a_tidal: f64, r: f64, g: f64, h2: f64) -> f64` |
| `spring_tide_amplitude` | `(h_moon: f64, h_sun: f64) -> f64` |
| `neap_tide_amplitude` | `(h_moon: f64, h_sun: f64) -> f64` |
| `tidal_dissipation_rate` | `(k2: f64, n: f64, m: f64, r: f64, q: f64, d: f64) -> f64` |
| `tidal_locking_timescale` | `(omega: f64, a: f64, mu: f64, q: f64, m: f64, r: f64) -> f64` |

---

## Hub dispatch mapping

All astronomy functions are wired through the astronomy dispatcher in `src/hub/engine/dispatch/astronomy.rs`.

```rust
use sciforge::hub::prelude::*;

let exp = Experiment::new(DomainType::Astronomy, "hubble_at_z_lcdm")
	.param("h0", ParameterValue::Scalar(67.4))
	.param("omega_m", ParameterValue::Scalar(0.315))
	.param("z", ParameterValue::Scalar(1.0));

let out = ExperimentRunner::new().run(&exp)?;
```
