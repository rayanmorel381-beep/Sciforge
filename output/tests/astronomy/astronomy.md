# SciForge Astronomy Benchmarks

59 benchmark entries across 5 formats.

---

## Grid
```
  celestial       :  GF   HS  JDT   SP
  cosmology       : ADW  AOU  AMR  AUF  CTA  CTS  CDI  CDZ   CD  DDT  EZL  EZL  HAZ  HEI   HV  LTI  LAD  RRV   SF  SFH
  galactic        : GEV  GRV   TR
  impacts         :  CD   FR   IV
  orbits          :  CV   EV  EVC  HDV  KPE  PAC   RL  SOI  TAR  VVC
  planetary       : HZI  HZO   SG
  rotation        : RKE  SVA
  stellar         : AMA   CL   DM  ELS  LFR  MSL  MLM   SR  SBB  SFI  WPW
  tides           : STA  TLT   TP
```
---

## Directory Layout
```
csv/
  all.csv
json/
  {label}.json
  summary.json
yaml/
  {label}.yaml
  summary.yaml
toml/
  {label}.toml
  summary.toml
bmk/
  {label}.bmk
benchmark.html
benchmark.md
```

## Results
| Label | category | symbol | name | domain | test | Precision | Iterations | Avg (ns) | Iters/s |
|---|---|---|---|---|---|---|---|---|---|
| celestial_gravitational_force | celestial | GF | Gravitational force | astronomy | celestial::gravitational_force | f64 | 1 | 5499748.00 | 200.00 |
| celestial_hill_sphere | celestial | HS | Hill sphere | astronomy | celestial::hill_sphere | f64 | 1 | 2836138.00 | 500.00 |
| celestial_julian_date_to_j2000_century | celestial | JDT | Julian date to j2000 century | astronomy | celestial::julian_date_to_j2000_century | f64 | 1 | 2219679.00 | 500.00 |
| celestial_synodic_period | celestial | SP | Synodic period | astronomy | celestial::synodic_period | f64 | 1 | 2186719.00 | 500.00 |
| cosmology_age_decreases_with_redshift | cosmology | ADW | Age decreases with redshift | astronomy | cosmology::age_decreases_with_redshift | f64 | 1 | 2154409.00 | 500.00 |
| cosmology_age_of_universe | cosmology | AOU | Age of universe | astronomy | cosmology::age_of_universe | f64 | 1 | 2286299.00 | 500.00 |
| cosmology_astronomy_missing_required_parameter_returns_error | cosmology | AMR | Astronomy missing required parameter returns error | astronomy | cosmology::astronomy_missing_required_parameter_returns_error | f64 | 1 | 2139599.00 | 500.00 |
| cosmology_astronomy_unknown_function_returns_error | cosmology | AUF | Astronomy unknown function returns error | astronomy | cosmology::astronomy_unknown_function_returns_error | f64 | 1 | 1981020.00 | 1000.00 |
| cosmology_cmb_temperature_at_z | cosmology | CTA | Cmb temperature at z | astronomy | cosmology::cmb_temperature_at_z | f64 | 1 | 1790849.00 | 1000.00 |
| cosmology_cmb_temperature_scales_with_z | cosmology | CTS | Cmb temperature scales with z | astronomy | cosmology::cmb_temperature_scales_with_z | f64 | 1 | 1742679.00 | 1000.00 |
| cosmology_comoving_distance_increases_with_redshift | cosmology | CDI | Comoving distance increases with redshift | astronomy | cosmology::comoving_distance_increases_with_redshift | f64 | 1 | 1832989.00 | 1000.00 |
| cosmology_comoving_distance_zero_at_zero_redshift | cosmology | CDZ | Comoving distance zero at zero redshift | astronomy | cosmology::comoving_distance_zero_at_zero_redshift | f64 | 1 | 1811669.00 | 1000.00 |
| cosmology_critical_density | cosmology | CD | Critical density | astronomy | cosmology::critical_density | f64 | 1 | 1853390.00 | 1000.00 |
| cosmology_distance_duality_table_driven | cosmology | DDT | Distance duality table driven | astronomy | cosmology::distance_duality_table_driven | f64 | 1 | 2259109.00 | 500.00 |
| cosmology_e_z_lcdm_sweep_monotonic_for_positive_z | cosmology | EZL | E z lcdm sweep monotonic for positive z | astronomy | cosmology::e_z_lcdm_sweep_monotonic_for_positive_z | f64 | 1 | 1906639.00 | 1000.00 |
| cosmology_e_z_lcdm_today | cosmology | EZL | E z lcdm today | astronomy | cosmology::e_z_lcdm_today | f64 | 1 | 1790449.00 | 1000.00 |
| cosmology_hubble_at_z_matches_h0_times_ez_lcdm | cosmology | HAZ | Hubble at z matches h0 times ez lcdm | astronomy | cosmology::hubble_at_z_matches_h0_times_ez_lcdm | f64 | 1 | 1780059.00 | 1000.00 |
| cosmology_hubble_ez_identity_table_driven | cosmology | HEI | Hubble ez identity table driven | astronomy | cosmology::hubble_ez_identity_table_driven | f64 | 1 | 1791079.00 | 1000.00 |
| cosmology_hubble_velocity | cosmology | HV | Hubble velocity | astronomy | cosmology::hubble_velocity | f64 | 1 | 1783230.00 | 1000.00 |
| cosmology_lookback_time_increases_with_redshift | cosmology | LTI | Lookback time increases with redshift | astronomy | cosmology::lookback_time_increases_with_redshift | f64 | 1 | 1932119.00 | 1000.00 |
| cosmology_luminosity_angular_distance_duality_from_z | cosmology | LAD | Luminosity angular distance duality from z | astronomy | cosmology::luminosity_angular_distance_duality_from_z | f64 | 1 | 2219909.00 | 500.00 |
| cosmology_relativistic_redshift_velocity_roundtrip | cosmology | RRV | Relativistic redshift velocity roundtrip | astronomy | cosmology::relativistic_redshift_velocity_roundtrip | f64 | 1 | 2215559.00 | 500.00 |
| cosmology_scale_factor | cosmology | SF | Scale factor | astronomy | cosmology::scale_factor | f64 | 1 | 2184019.00 | 500.00 |
| cosmology_scale_factor_high_z | cosmology | SFH | Scale factor high z | astronomy | cosmology::scale_factor_high_z | f64 | 1 | 2186509.00 | 500.00 |
| galactic_galactic_escape_velocity | galactic | GEV | Galactic escape velocity | astronomy | galactic::galactic_escape_velocity | f64 | 1 | 2224499.00 | 500.00 |
| galactic_galactic_rotation_velocity | galactic | GRV | Galactic rotation velocity | astronomy | galactic::galactic_rotation_velocity | f64 | 1 | 2625109.00 | 500.00 |
| galactic_tidal_radius | galactic | TR | Tidal radius | astronomy | galactic::tidal_radius | f64 | 1 | 2416869.00 | 500.00 |
| impacts_crater_diameter | impacts | CD | Crater diameter | astronomy | impacts::crater_diameter | f64 | 1 | 2436449.00 | 500.00 |
| impacts_fireball_radius | impacts | FR | Fireball radius | astronomy | impacts::fireball_radius | f64 | 1 | 2142609.00 | 500.00 |
| impacts_impact_velocity | impacts | IV | Impact velocity | astronomy | impacts::impact_velocity | f64 | 1 | 2264409.00 | 500.00 |
| orbits_circular_velocity | orbits | CV | Circular velocity | astronomy | orbits::circular_velocity | f64 | 1 | 1827569.00 | 1000.00 |
| orbits_escape_velocity | orbits | EV | Escape velocity | astronomy | orbits::escape_velocity | f64 | 1 | 2249449.00 | 500.00 |
| orbits_escape_vs_circular_velocity_ratio | orbits | EVC | Escape vs circular velocity ratio | astronomy | orbits::escape_vs_circular_velocity_ratio | f64 | 1 | 1980699.00 | 1000.00 |
| orbits_hohmann_delta_v_positive_and_zero_identity | orbits | HDV | Hohmann delta v positive and zero identity | astronomy | orbits::hohmann_delta_v_positive_and_zero_identity | f64 | 1 | 2555539.00 | 500.00 |
| orbits_kepler_period_earth | orbits | KPE | Kepler period earth | astronomy | orbits::kepler_period_earth | f64 | 1 | 2288119.00 | 500.00 |
| orbits_periapsis_apoapsis_consistency | orbits | PAC | Periapsis apoapsis consistency | astronomy | orbits::periapsis_apoapsis_consistency | f64 | 1 | 2193379.00 | 500.00 |
| orbits_roche_limit | orbits | RL | Roche limit | astronomy | orbits::roche_limit | f64 | 1 | 2279879.00 | 500.00 |
| orbits_sphere_of_influence_earth | orbits | SOI | Sphere of influence earth | astronomy | orbits::sphere_of_influence_earth | f64 | 1 | 2176399.00 | 500.00 |
| orbits_true_anomaly_radius_matches_extrema | orbits | TAR | True anomaly radius matches extrema | astronomy | orbits::true_anomaly_radius_matches_extrema | f64 | 1 | 2493759.00 | 500.00 |
| orbits_vis_viva_circular | orbits | VVC | Vis viva circular | astronomy | orbits::vis_viva_circular | f64 | 1 | 2235179.00 | 500.00 |
| planetary_habitable_zone_inner | planetary | HZI | Habitable zone inner | astronomy | planetary::habitable_zone_inner | f64 | 1 | 1880889.00 | 1000.00 |
| planetary_habitable_zone_outer | planetary | HZO | Habitable zone outer | astronomy | planetary::habitable_zone_outer | f64 | 1 | 1899979.00 | 1000.00 |
| planetary_surface_gravity | planetary | SG | Surface gravity | astronomy | planetary::surface_gravity | f64 | 1 | 2285609.00 | 500.00 |
| rotation_rotational_kinetic_energy | rotation | RKE | Rotational kinetic energy | astronomy | rotation::rotational_kinetic_energy | f64 | 1 | 2208579.00 | 500.00 |
| rotation_surface_velocity_at_latitude | rotation | SVA | Surface velocity at latitude | astronomy | rotation::surface_velocity_at_latitude | f64 | 1 | 2213009.00 | 500.00 |
| stellar_absolute_magnitude_and_distance_modulus_roundtrip | stellar | AMA | Absolute magnitude and distance modulus roundtrip | astronomy | stellar::absolute_magnitude_and_distance_modulus_roundtrip | f64 | 1 | 2343329.00 | 500.00 |
| stellar_chandrasekhar_limit | stellar | CL | Chandrasekhar limit | astronomy | stellar::chandrasekhar_limit | f64 | 1 | 2510739.00 | 500.00 |
| stellar_distance_modulus | stellar | DM | Distance modulus | astronomy | stellar::distance_modulus | f64 | 1 | 2460459.00 | 500.00 |
| stellar_eddington_luminosity_scales_linearly_with_mass | stellar | ELS | Eddington luminosity scales linearly with mass | astronomy | stellar::eddington_luminosity_scales_linearly_with_mass | f64 | 1 | 2349879.00 | 500.00 |
| stellar_luminosity_from_radius_temp | stellar | LFR | Luminosity from radius temp | astronomy | stellar::luminosity_from_radius_temp | f64 | 1 | 2222319.00 | 500.00 |
| stellar_main_sequence_lifetime | stellar | MSL | Main sequence lifetime | astronomy | stellar::main_sequence_lifetime | f64 | 1 | 2103799.00 | 500.00 |
| stellar_mass_luminosity_monotonic_in_solar_range | stellar | MLM | Mass luminosity monotonic in solar range | astronomy | stellar::mass_luminosity_monotonic_in_solar_range | f64 | 1 | 1933069.00 | 1000.00 |
| stellar_schwarzschild_radius | stellar | SR | Schwarzschild radius | astronomy | stellar::schwarzschild_radius | f64 | 1 | 1887099.00 | 1000.00 |
| stellar_stefan_boltzmann_blackbody | stellar | SBB | Stefan boltzmann blackbody | astronomy | stellar::stefan_boltzmann_blackbody | f64 | 1 | 1837400.00 | 1000.00 |
| stellar_stellar_flux_inverse_square_distance | stellar | SFI | Stellar flux inverse square distance | astronomy | stellar::stellar_flux_inverse_square_distance | f64 | 1 | 2429828.00 | 500.00 |
| stellar_wien_peak_wavelength | stellar | WPW | Wien peak wavelength | astronomy | stellar::wien_peak_wavelength | f64 | 1 | 2136050.00 | 500.00 |
| tides_spring_tide_amplitude | tides | STA | Spring tide amplitude | astronomy | tides::spring_tide_amplitude | f64 | 1 | 2296899.00 | 500.00 |
| tides_tidal_locking_timescale | tides | TLT | Tidal locking timescale | astronomy | tides::tidal_locking_timescale | f64 | 1 | 2618138.00 | 500.00 |
| tides_tidal_potential | tides | TP | Tidal potential | astronomy | tides::tidal_potential | f64 | 1 | 2471599.00 | 500.00 |
## Analytics
### Category Distribution
| Category | Count | Avg (ns) | Min (ns) | Max (ns) |
|---|---|---|---|---|
| celestial | 4 | 3185571.00 | 2186719.00 | 5499748.00 |
| cosmology | 20 | 1982079.25 | 1742679.00 | 2286299.00 |
| galactic | 3 | 2422159.00 | 2224499.00 | 2625109.00 |
| impacts | 3 | 2281155.75 | 2142609.00 | 2436449.00 |
| orbits | 10 | 2227997.25 | 1827569.00 | 2555539.00 |
| planetary | 3 | 2022159.00 | 1880889.00 | 2285609.00 |
| rotation | 2 | 2210794.00 | 2208579.00 | 2213009.00 |
| stellar | 11 | 2201270.00 | 1837400.00 | 2510739.00 |
| tides | 3 | 2462212.00 | 2296899.00 | 2618138.00 |

### Top 10 Fastest Entries
| # | Entry | Avg (ns) |
|---|---|---|
| 1 | cosmology_cmb_temperature_scales_with_z | 1742679.00 |
| 2 | cosmology_hubble_at_z_matches_h0_times_ez_lcdm | 1780059.00 |
| 3 | cosmology_hubble_velocity | 1783230.00 |
| 4 | cosmology_e_z_lcdm_today | 1790449.00 |
| 5 | cosmology_cmb_temperature_at_z | 1790849.00 |
| 6 | cosmology_hubble_ez_identity_table_driven | 1791079.00 |
| 7 | cosmology_comoving_distance_zero_at_zero_redshift | 1811669.00 |
| 8 | orbits_circular_velocity | 1827569.00 |
| 9 | cosmology_comoving_distance_increases_with_redshift | 1832989.00 |
| 10 | stellar_stefan_boltzmann_blackbody | 1837400.00 |

### Top 10 Slowest Entries
| # | Entry | Avg (ns) |
|---|---|---|
| 1 | celestial_gravitational_force | 5499748.00 |
| 2 | celestial_hill_sphere | 2836138.00 |
| 3 | galactic_galactic_rotation_velocity | 2625109.00 |
| 4 | tides_tidal_locking_timescale | 2618138.00 |
| 5 | orbits_hohmann_delta_v_positive_and_zero_identity | 2555539.00 |
| 6 | stellar_chandrasekhar_limit | 2510739.00 |
| 7 | orbits_true_anomaly_radius_matches_extrema | 2493759.00 |
| 8 | tides_tidal_potential | 2471599.00 |
| 9 | stellar_distance_modulus | 2460459.00 |
| 10 | impacts_crater_diameter | 2436449.00 |

