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
| celestial_gravitational_force | celestial | GF | Gravitational force | astronomy | celestial::gravitational_force | f64 | 1 | 2418580.00 | 500.00 |
| celestial_hill_sphere | celestial | HS | Hill sphere | astronomy | celestial::hill_sphere | f64 | 1 | 2585930.00 | 500.00 |
| celestial_julian_date_to_j2000_century | celestial | JDT | Julian date to j2000 century | astronomy | celestial::julian_date_to_j2000_century | f64 | 1 | 2344891.00 | 500.00 |
| celestial_synodic_period | celestial | SP | Synodic period | astronomy | celestial::synodic_period | f64 | 1 | 1923450.00 | 1000.00 |
| cosmology_age_decreases_with_redshift | cosmology | ADW | Age decreases with redshift | astronomy | cosmology::age_decreases_with_redshift | f64 | 1 | 1902250.00 | 1000.00 |
| cosmology_age_of_universe | cosmology | AOU | Age of universe | astronomy | cosmology::age_of_universe | f64 | 1 | 1758651.00 | 1000.00 |
| cosmology_astronomy_missing_required_parameter_returns_error | cosmology | AMR | Astronomy missing required parameter returns error | astronomy | cosmology::astronomy_missing_required_parameter_returns_error | f64 | 1 | 1860870.00 | 1000.00 |
| cosmology_astronomy_unknown_function_returns_error | cosmology | AUF | Astronomy unknown function returns error | astronomy | cosmology::astronomy_unknown_function_returns_error | f64 | 1 | 1854240.00 | 1000.00 |
| cosmology_cmb_temperature_at_z | cosmology | CTA | Cmb temperature at z | astronomy | cosmology::cmb_temperature_at_z | f64 | 1 | 1897491.00 | 1000.00 |
| cosmology_cmb_temperature_scales_with_z | cosmology | CTS | Cmb temperature scales with z | astronomy | cosmology::cmb_temperature_scales_with_z | f64 | 1 | 1810390.00 | 1000.00 |
| cosmology_comoving_distance_increases_with_redshift | cosmology | CDI | Comoving distance increases with redshift | astronomy | cosmology::comoving_distance_increases_with_redshift | f64 | 1 | 1996610.00 | 1000.00 |
| cosmology_comoving_distance_zero_at_zero_redshift | cosmology | CDZ | Comoving distance zero at zero redshift | astronomy | cosmology::comoving_distance_zero_at_zero_redshift | f64 | 1 | 1815201.00 | 1000.00 |
| cosmology_critical_density | cosmology | CD | Critical density | astronomy | cosmology::critical_density | f64 | 1 | 1790740.00 | 1000.00 |
| cosmology_distance_duality_table_driven | cosmology | DDT | Distance duality table driven | astronomy | cosmology::distance_duality_table_driven | f64 | 1 | 2098650.00 | 500.00 |
| cosmology_e_z_lcdm_sweep_monotonic_for_positive_z | cosmology | EZL | E z lcdm sweep monotonic for positive z | astronomy | cosmology::e_z_lcdm_sweep_monotonic_for_positive_z | f64 | 1 | 1766911.00 | 1000.00 |
| cosmology_e_z_lcdm_today | cosmology | EZL | E z lcdm today | astronomy | cosmology::e_z_lcdm_today | f64 | 1 | 1723810.00 | 1000.00 |
| cosmology_hubble_at_z_matches_h0_times_ez_lcdm | cosmology | HAZ | Hubble at z matches h0 times ez lcdm | astronomy | cosmology::hubble_at_z_matches_h0_times_ez_lcdm | f64 | 1 | 1824760.00 | 1000.00 |
| cosmology_hubble_ez_identity_table_driven | cosmology | HEI | Hubble ez identity table driven | astronomy | cosmology::hubble_ez_identity_table_driven | f64 | 1 | 2020440.00 | 500.00 |
| cosmology_hubble_velocity | cosmology | HV | Hubble velocity | astronomy | cosmology::hubble_velocity | f64 | 1 | 1970611.00 | 1000.00 |
| cosmology_lookback_time_increases_with_redshift | cosmology | LTI | Lookback time increases with redshift | astronomy | cosmology::lookback_time_increases_with_redshift | f64 | 1 | 1852290.00 | 1000.00 |
| cosmology_luminosity_angular_distance_duality_from_z | cosmology | LAD | Luminosity angular distance duality from z | astronomy | cosmology::luminosity_angular_distance_duality_from_z | f64 | 1 | 1835050.00 | 1000.00 |
| cosmology_relativistic_redshift_velocity_roundtrip | cosmology | RRV | Relativistic redshift velocity roundtrip | astronomy | cosmology::relativistic_redshift_velocity_roundtrip | f64 | 1 | 1756931.00 | 1000.00 |
| cosmology_scale_factor | cosmology | SF | Scale factor | astronomy | cosmology::scale_factor | f64 | 1 | 1996360.00 | 1000.00 |
| cosmology_scale_factor_high_z | cosmology | SFH | Scale factor high z | astronomy | cosmology::scale_factor_high_z | f64 | 1 | 3054211.00 | 333.33 |
| galactic_galactic_escape_velocity | galactic | GEV | Galactic escape velocity | astronomy | galactic::galactic_escape_velocity | f64 | 1 | 2340760.00 | 500.00 |
| galactic_galactic_rotation_velocity | galactic | GRV | Galactic rotation velocity | astronomy | galactic::galactic_rotation_velocity | f64 | 1 | 2293720.00 | 500.00 |
| galactic_tidal_radius | galactic | TR | Tidal radius | astronomy | galactic::tidal_radius | f64 | 1 | 2301521.00 | 500.00 |
| impacts_crater_diameter | impacts | CD | Crater diameter | astronomy | impacts::crater_diameter | f64 | 1 | 2181410.00 | 500.00 |
| impacts_fireball_radius | impacts | FR | Fireball radius | astronomy | impacts::fireball_radius | f64 | 1 | 2344410.00 | 500.00 |
| impacts_impact_velocity | impacts | IV | Impact velocity | astronomy | impacts::impact_velocity | f64 | 1 | 2185011.00 | 500.00 |
| orbits_circular_velocity | orbits | CV | Circular velocity | astronomy | orbits::circular_velocity | f64 | 1 | 1964850.00 | 1000.00 |
| orbits_escape_velocity | orbits | EV | Escape velocity | astronomy | orbits::escape_velocity | f64 | 1 | 1773520.00 | 1000.00 |
| orbits_escape_vs_circular_velocity_ratio | orbits | EVC | Escape vs circular velocity ratio | astronomy | orbits::escape_vs_circular_velocity_ratio | f64 | 1 | 1773721.00 | 1000.00 |
| orbits_hohmann_delta_v_positive_and_zero_identity | orbits | HDV | Hohmann delta v positive and zero identity | astronomy | orbits::hohmann_delta_v_positive_and_zero_identity | f64 | 1 | 1977900.00 | 1000.00 |
| orbits_kepler_period_earth | orbits | KPE | Kepler period earth | astronomy | orbits::kepler_period_earth | f64 | 1 | 1785530.00 | 1000.00 |
| orbits_periapsis_apoapsis_consistency | orbits | PAC | Periapsis apoapsis consistency | astronomy | orbits::periapsis_apoapsis_consistency | f64 | 1 | 1774631.00 | 1000.00 |
| orbits_roche_limit | orbits | RL | Roche limit | astronomy | orbits::roche_limit | f64 | 1 | 1893530.00 | 1000.00 |
| orbits_sphere_of_influence_earth | orbits | SOI | Sphere of influence earth | astronomy | orbits::sphere_of_influence_earth | f64 | 1 | 1948590.00 | 1000.00 |
| orbits_true_anomaly_radius_matches_extrema | orbits | TAR | True anomaly radius matches extrema | astronomy | orbits::true_anomaly_radius_matches_extrema | f64 | 1 | 1913171.00 | 1000.00 |
| orbits_vis_viva_circular | orbits | VVC | Vis viva circular | astronomy | orbits::vis_viva_circular | f64 | 1 | 1839940.00 | 1000.00 |
| planetary_habitable_zone_inner | planetary | HZI | Habitable zone inner | astronomy | planetary::habitable_zone_inner | f64 | 1 | 1823610.00 | 1000.00 |
| planetary_habitable_zone_outer | planetary | HZO | Habitable zone outer | astronomy | planetary::habitable_zone_outer | f64 | 1 | 2405651.00 | 500.00 |
| planetary_surface_gravity | planetary | SG | Surface gravity | astronomy | planetary::surface_gravity | f64 | 1 | 1969520.00 | 1000.00 |
| rotation_rotational_kinetic_energy | rotation | RKE | Rotational kinetic energy | astronomy | rotation::rotational_kinetic_energy | f64 | 1 | 2141540.00 | 500.00 |
| rotation_surface_velocity_at_latitude | rotation | SVA | Surface velocity at latitude | astronomy | rotation::surface_velocity_at_latitude | f64 | 1 | 1926761.00 | 1000.00 |
| stellar_absolute_magnitude_and_distance_modulus_roundtrip | stellar | AMA | Absolute magnitude and distance modulus roundtrip | astronomy | stellar::absolute_magnitude_and_distance_modulus_roundtrip | f64 | 1 | 2031540.00 | 500.00 |
| stellar_chandrasekhar_limit | stellar | CL | Chandrasekhar limit | astronomy | stellar::chandrasekhar_limit | f64 | 1 | 1978980.00 | 1000.00 |
| stellar_distance_modulus | stellar | DM | Distance modulus | astronomy | stellar::distance_modulus | f64 | 1 | 1940471.00 | 1000.00 |
| stellar_eddington_luminosity_scales_linearly_with_mass | stellar | ELS | Eddington luminosity scales linearly with mass | astronomy | stellar::eddington_luminosity_scales_linearly_with_mass | f64 | 1 | 2020000.00 | 500.00 |
| stellar_luminosity_from_radius_temp | stellar | LFR | Luminosity from radius temp | astronomy | stellar::luminosity_from_radius_temp | f64 | 1 | 2040770.00 | 500.00 |
| stellar_main_sequence_lifetime | stellar | MSL | Main sequence lifetime | astronomy | stellar::main_sequence_lifetime | f64 | 1 | 1872621.00 | 1000.00 |
| stellar_mass_luminosity_monotonic_in_solar_range | stellar | MLM | Mass luminosity monotonic in solar range | astronomy | stellar::mass_luminosity_monotonic_in_solar_range | f64 | 1 | 2109080.00 | 500.00 |
| stellar_schwarzschild_radius | stellar | SR | Schwarzschild radius | astronomy | stellar::schwarzschild_radius | f64 | 1 | 1874520.00 | 1000.00 |
| stellar_stefan_boltzmann_blackbody | stellar | SBB | Stefan boltzmann blackbody | astronomy | stellar::stefan_boltzmann_blackbody | f64 | 1 | 1898931.00 | 1000.00 |
| stellar_stellar_flux_inverse_square_distance | stellar | SFI | Stellar flux inverse square distance | astronomy | stellar::stellar_flux_inverse_square_distance | f64 | 1 | 1939220.00 | 1000.00 |
| stellar_wien_peak_wavelength | stellar | WPW | Wien peak wavelength | astronomy | stellar::wien_peak_wavelength | f64 | 1 | 1884390.00 | 1000.00 |
| tides_spring_tide_amplitude | tides | STA | Spring tide amplitude | astronomy | tides::spring_tide_amplitude | f64 | 1 | 1989851.00 | 1000.00 |
| tides_tidal_locking_timescale | tides | TLT | Tidal locking timescale | astronomy | tides::tidal_locking_timescale | f64 | 1 | 1971680.00 | 1000.00 |
| tides_tidal_potential | tides | TP | Tidal potential | astronomy | tides::tidal_potential | f64 | 1 | 2007850.00 | 500.00 |
## Analytics
### Category Distribution
| Category | Count | Avg (ns) | Min (ns) | Max (ns) |
|---|---|---|---|---|
| celestial | 4 | 2318212.75 | 1923450.00 | 2585930.00 |
| cosmology | 20 | 1929323.35 | 1723810.00 | 3054211.00 |
| galactic | 3 | 2312000.33 | 2293720.00 | 2340760.00 |
| impacts | 3 | 2236943.67 | 2181410.00 | 2344410.00 |
| orbits | 10 | 1864538.30 | 1773520.00 | 1977900.00 |
| planetary | 3 | 2066260.33 | 1823610.00 | 2405651.00 |
| rotation | 2 | 2034150.50 | 1926761.00 | 2141540.00 |
| stellar | 11 | 1962774.82 | 1872621.00 | 2109080.00 |
| tides | 3 | 1989793.67 | 1971680.00 | 2007850.00 |

### Top 10 Fastest Entries
| # | Entry | Avg (ns) |
|---|---|---|
| 1 | cosmology_e_z_lcdm_today | 1723810.00 |
| 2 | cosmology_relativistic_redshift_velocity_roundtrip | 1756931.00 |
| 3 | cosmology_age_of_universe | 1758651.00 |
| 4 | cosmology_e_z_lcdm_sweep_monotonic_for_positive_z | 1766911.00 |
| 5 | orbits_escape_velocity | 1773520.00 |
| 6 | orbits_escape_vs_circular_velocity_ratio | 1773721.00 |
| 7 | orbits_periapsis_apoapsis_consistency | 1774631.00 |
| 8 | orbits_kepler_period_earth | 1785530.00 |
| 9 | cosmology_critical_density | 1790740.00 |
| 10 | cosmology_cmb_temperature_scales_with_z | 1810390.00 |

### Top 10 Slowest Entries
| # | Entry | Avg (ns) |
|---|---|---|
| 1 | cosmology_scale_factor_high_z | 3054211.00 |
| 2 | celestial_hill_sphere | 2585930.00 |
| 3 | celestial_gravitational_force | 2418580.00 |
| 4 | planetary_habitable_zone_outer | 2405651.00 |
| 5 | celestial_julian_date_to_j2000_century | 2344891.00 |
| 6 | impacts_fireball_radius | 2344410.00 |
| 7 | galactic_galactic_escape_velocity | 2340760.00 |
| 8 | galactic_tidal_radius | 2301521.00 |
| 9 | galactic_galactic_rotation_velocity | 2293720.00 |
| 10 | impacts_impact_velocity | 2185011.00 |

