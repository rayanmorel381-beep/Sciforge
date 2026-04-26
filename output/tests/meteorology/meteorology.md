# SciForge Meteorology Test

45 benchmark entries across 5 formats.

---

## Grid
```
  atmosphere      : BDW   BF  LRD   PT  SVI  SVP   SH
  dynamics        : CCS   CP  CZA   ED   GW   RN  RND  RWS
  ocean           :  HS  SLR   SD  SDI   SS
  precipitation   : API  IDF   PE   RR  RRM  SCN  TVR   TP
  radiation       :  AR   ET   PF  RFC  RFC  SBF  SBF
  storms          : CPB  CZF   FS  FSI   PI
  winds           :  BM  BTM  BZI   WC  WCD
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
| atmosphere_barometric_decreases_with_altitude | atmosphere | BDW | Barometric decreases with altitude | meteorology | atmosphere::barometric_decreases_with_altitude | f64 | 1 | 2345790.00 | 500.00 |
| atmosphere_barometric_formula | atmosphere | BF | Barometric formula | meteorology | atmosphere::barometric_formula | f64 | 1 | 2493070.00 | 500.00 |
| atmosphere_lapse_rate_dry | atmosphere | LRD | Lapse rate dry | meteorology | atmosphere::lapse_rate_dry | f64 | 1 | 2137751.00 | 500.00 |
| atmosphere_potential_temperature | atmosphere | PT | Potential temperature | meteorology | atmosphere::potential_temperature | f64 | 1 | 2008090.00 | 500.00 |
| atmosphere_saturation_vapor_increases_with_temp | atmosphere | SVI | Saturation vapor increases with temp | meteorology | atmosphere::saturation_vapor_increases_with_temp | f64 | 1 | 1877660.00 | 1000.00 |
| atmosphere_saturation_vapor_pressure | atmosphere | SVP | Saturation vapor pressure | meteorology | atmosphere::saturation_vapor_pressure | f64 | 1 | 1948561.00 | 1000.00 |
| atmosphere_scale_height | atmosphere | SH | Scale height | meteorology | atmosphere::scale_height | f64 | 1 | 1822450.00 | 1000.00 |
| dynamics_coriolis_changes_sign_between_hemispheres | dynamics | CCS | Coriolis changes sign between hemispheres | meteorology | dynamics::coriolis_changes_sign_between_hemispheres | f64 | 1 | 1820500.00 | 1000.00 |
| dynamics_coriolis_parameter | dynamics | CP | Coriolis parameter | meteorology | dynamics::coriolis_parameter | f64 | 1 | 1756301.00 | 1000.00 |
| dynamics_coriolis_zero_at_equator | dynamics | CZA | Coriolis zero at equator | meteorology | dynamics::coriolis_zero_at_equator | f64 | 1 | 1818780.00 | 1000.00 |
| dynamics_ekman_depth | dynamics | ED | Ekman depth | meteorology | dynamics::ekman_depth | f64 | 1 | 1926370.00 | 1000.00 |
| dynamics_geostrophic_wind | dynamics | GW | Geostrophic wind | meteorology | dynamics::geostrophic_wind | f64 | 1 | 1964351.00 | 1000.00 |
| dynamics_rossby_number | dynamics | RN | Rossby number | meteorology | dynamics::rossby_number | f64 | 1 | 1967600.00 | 1000.00 |
| dynamics_rossby_number_decreases_with_scale | dynamics | RND | Rossby number decreases with scale | meteorology | dynamics::rossby_number_decreases_with_scale | f64 | 1 | 1908620.00 | 1000.00 |
| dynamics_rossby_wave_speed | dynamics | RWS | Rossby wave speed | meteorology | dynamics::rossby_wave_speed | f64 | 1 | 2358321.00 | 500.00 |
| ocean_henry_solubility | ocean | HS | Henry solubility | meteorology | ocean::henry_solubility | f64 | 1 | 2061620.00 | 500.00 |
| ocean_sea_level_rise_thermal | ocean | SLR | Sea level rise thermal | meteorology | ocean::sea_level_rise_thermal | f64 | 1 | 1772630.00 | 1000.00 |
| ocean_seawater_density | ocean | SD | Seawater density | meteorology | ocean::seawater_density | f64 | 1 | 1886241.00 | 1000.00 |
| ocean_seawater_density_increases_with_salinity | ocean | SDI | Seawater density increases with salinity | meteorology | ocean::seawater_density_increases_with_salinity | f64 | 1 | 1752250.00 | 1000.00 |
| ocean_sound_speed | ocean | SS | Sound speed | meteorology | ocean::sound_speed | f64 | 1 | 1720930.00 | 1000.00 |
| precipitation_antecedent_precipitation_index | precipitation | API | Antecedent precipitation index | meteorology | precipitation::antecedent_precipitation_index | f64 | 1 | 1688380.00 | 1000.00 |
| precipitation_intensity_duration_frequency | precipitation | IDF | Intensity duration frequency | meteorology | precipitation::intensity_duration_frequency | f64 | 1 | 2419871.00 | 500.00 |
| precipitation_penman_evaporation | precipitation | PE | Penman evaporation | meteorology | precipitation::penman_evaporation | f64 | 1 | 2031560.00 | 500.00 |
| precipitation_radar_reflectivity | precipitation | RR | Radar reflectivity | meteorology | precipitation::radar_reflectivity | f64 | 1 | 1779490.00 | 1000.00 |
| precipitation_rain_rate_marshall_palmer | precipitation | RRM | Rain rate marshall palmer | meteorology | precipitation::rain_rate_marshall_palmer | f64 | 1 | 1680801.00 | 1000.00 |
| precipitation_scs_curve_number_runoff | precipitation | SCN | Scs curve number runoff | meteorology | precipitation::scs_curve_number_runoff | f64 | 1 | 1663020.00 | 1000.00 |
| precipitation_terminal_velocity_raindrop | precipitation | TVR | Terminal velocity raindrop | meteorology | precipitation::terminal_velocity_raindrop | f64 | 1 | 1748340.00 | 1000.00 |
| precipitation_thornthwaite_pet | precipitation | TP | Thornthwaite pet | meteorology | precipitation::thornthwaite_pet | f64 | 1 | 1703940.00 | 1000.00 |
| radiation_albedo_reflected | radiation | AR | Albedo reflected | meteorology | radiation::albedo_reflected | f64 | 1 | 1836231.00 | 1000.00 |
| radiation_effective_temperature | radiation | ET | Effective temperature | meteorology | radiation::effective_temperature | f64 | 1 | 1684530.00 | 1000.00 |
| radiation_planck_function | radiation | PF | Planck function | meteorology | radiation::planck_function | f64 | 1 | 1685180.00 | 1000.00 |
| radiation_radiative_forcing_co2 | radiation | RFC | Radiative forcing co2 | meteorology | radiation::radiative_forcing_co2 | f64 | 1 | 1758401.00 | 1000.00 |
| radiation_radiative_forcing_co2_zero_when_equal_concentrations | radiation | RFC | Radiative forcing co2 zero when equal concentrations | meteorology | radiation::radiative_forcing_co2_zero_when_equal_concentrations | f64 | 1 | 2209720.00 | 500.00 |
| radiation_stefan_boltzmann_flux | radiation | SBF | Stefan boltzmann flux | meteorology | radiation::stefan_boltzmann_flux | f64 | 1 | 1910490.00 | 1000.00 |
| radiation_stefan_boltzmann_flux_monotonic_with_temperature_sweep | radiation | SBF | Stefan boltzmann flux monotonic with temperature sweep | meteorology | radiation::stefan_boltzmann_flux_monotonic_with_temperature_sweep | f64 | 1 | 1760091.00 | 1000.00 |
| storms_cape_positive_buoyancy | storms | CPB | Cape positive buoyancy | meteorology | storms::cape_positive_buoyancy | f64 | 1 | 1745880.00 | 1000.00 |
| storms_cape_zero_for_equal_temps | storms | CZF | Cape zero for equal temps | meteorology | storms::cape_zero_for_equal_temps | f64 | 1 | 1704330.00 | 1000.00 |
| storms_fujita_scale | storms | FS | Fujita scale | meteorology | storms::fujita_scale | f64 | 1 | 1787041.00 | 1000.00 |
| storms_fujita_scale_increases_with_wind | storms | FSI | Fujita scale increases with wind | meteorology | storms::fujita_scale_increases_with_wind | f64 | 1 | 1704940.00 | 1000.00 |
| storms_potential_intensity | storms | PI | Potential intensity | meteorology | storms::potential_intensity | f64 | 1 | 1789840.00 | 1000.00 |
| winds_beaufort_monotonic | winds | BM | Beaufort monotonic | meteorology | winds::beaufort_monotonic | f64 | 1 | 1908610.00 | 1000.00 |
| winds_beaufort_to_m_s | winds | BTM | Beaufort to m s | meteorology | winds::beaufort_to_m_s | f64 | 1 | 1761491.00 | 1000.00 |
| winds_beaufort_zero_is_calm | winds | BZI | Beaufort zero is calm | meteorology | winds::beaufort_zero_is_calm | f64 | 1 | 1708480.00 | 1000.00 |
| winds_wind_chill | winds | WC | Wind chill | meteorology | winds::wind_chill | f64 | 1 | 1722210.00 | 1000.00 |
| winds_wind_chill_decreases_with_wind | winds | WCD | Wind chill decreases with wind | meteorology | winds::wind_chill_decreases_with_wind | f64 | 1 | 1703271.00 | 1000.00 |
## Analytics
### Category Distribution
| Category | Count | Avg (ns) | Min (ns) | Max (ns) |
|---|---|---|---|---|
| atmosphere | 7 | 2090481.71 | 1822450.00 | 2493070.00 |
| dynamics | 8 | 1940105.38 | 1756301.00 | 2358321.00 |
| ocean | 5 | 1838734.20 | 1720930.00 | 2061620.00 |
| precipitation | 8 | 1839425.25 | 1663020.00 | 2419871.00 |
| radiation | 7 | 1834949.00 | 1684530.00 | 2209720.00 |
| storms | 5 | 1746406.20 | 1704330.00 | 1789840.00 |
| winds | 5 | 1760812.40 | 1703271.00 | 1908610.00 |

### Top 10 Fastest Entries
| # | Entry | Avg (ns) |
|---|---|---|
| 1 | precipitation_scs_curve_number_runoff | 1663020.00 |
| 2 | precipitation_rain_rate_marshall_palmer | 1680801.00 |
| 3 | radiation_effective_temperature | 1684530.00 |
| 4 | radiation_planck_function | 1685180.00 |
| 5 | precipitation_antecedent_precipitation_index | 1688380.00 |
| 6 | winds_wind_chill_decreases_with_wind | 1703271.00 |
| 7 | precipitation_thornthwaite_pet | 1703940.00 |
| 8 | storms_cape_zero_for_equal_temps | 1704330.00 |
| 9 | storms_fujita_scale_increases_with_wind | 1704940.00 |
| 10 | winds_beaufort_zero_is_calm | 1708480.00 |

### Top 10 Slowest Entries
| # | Entry | Avg (ns) |
|---|---|---|
| 1 | atmosphere_barometric_formula | 2493070.00 |
| 2 | precipitation_intensity_duration_frequency | 2419871.00 |
| 3 | dynamics_rossby_wave_speed | 2358321.00 |
| 4 | atmosphere_barometric_decreases_with_altitude | 2345790.00 |
| 5 | radiation_radiative_forcing_co2_zero_when_equal_concentrations | 2209720.00 |
| 6 | atmosphere_lapse_rate_dry | 2137751.00 |
| 7 | ocean_henry_solubility | 2061620.00 |
| 8 | precipitation_penman_evaporation | 2031560.00 |
| 9 | atmosphere_potential_temperature | 2008090.00 |
| 10 | dynamics_rossby_number | 1967600.00 |

