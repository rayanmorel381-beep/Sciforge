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
| atmosphere_barometric_decreases_with_altitude | atmosphere | BDW | Barometric decreases with altitude | meteorology | atmosphere::barometric_decreases_with_altitude | f64 | 1 | 1964004.00 | 1000.00 |
| atmosphere_barometric_formula | atmosphere | BF | Barometric formula | meteorology | atmosphere::barometric_formula | f64 | 1 | 1906183.00 | 1000.00 |
| atmosphere_lapse_rate_dry | atmosphere | LRD | Lapse rate dry | meteorology | atmosphere::lapse_rate_dry | f64 | 1 | 2223894.00 | 500.00 |
| atmosphere_potential_temperature | atmosphere | PT | Potential temperature | meteorology | atmosphere::potential_temperature | f64 | 1 | 1976354.00 | 1000.00 |
| atmosphere_saturation_vapor_increases_with_temp | atmosphere | SVI | Saturation vapor increases with temp | meteorology | atmosphere::saturation_vapor_increases_with_temp | f64 | 1 | 1836393.00 | 1000.00 |
| atmosphere_saturation_vapor_pressure | atmosphere | SVP | Saturation vapor pressure | meteorology | atmosphere::saturation_vapor_pressure | f64 | 1 | 2263685.00 | 500.00 |
| atmosphere_scale_height | atmosphere | SH | Scale height | meteorology | atmosphere::scale_height | f64 | 1 | 2007663.00 | 500.00 |
| dynamics_coriolis_changes_sign_between_hemispheres | dynamics | CCS | Coriolis changes sign between hemispheres | meteorology | dynamics::coriolis_changes_sign_between_hemispheres | f64 | 1 | 1818384.00 | 1000.00 |
| dynamics_coriolis_parameter | dynamics | CP | Coriolis parameter | meteorology | dynamics::coriolis_parameter | f64 | 1 | 1922843.00 | 1000.00 |
| dynamics_coriolis_zero_at_equator | dynamics | CZA | Coriolis zero at equator | meteorology | dynamics::coriolis_zero_at_equator | f64 | 1 | 1901754.00 | 1000.00 |
| dynamics_ekman_depth | dynamics | ED | Ekman depth | meteorology | dynamics::ekman_depth | f64 | 1 | 2286924.00 | 500.00 |
| dynamics_geostrophic_wind | dynamics | GW | Geostrophic wind | meteorology | dynamics::geostrophic_wind | f64 | 1 | 1934454.00 | 1000.00 |
| dynamics_rossby_number | dynamics | RN | Rossby number | meteorology | dynamics::rossby_number | f64 | 1 | 1803073.00 | 1000.00 |
| dynamics_rossby_number_decreases_with_scale | dynamics | RND | Rossby number decreases with scale | meteorology | dynamics::rossby_number_decreases_with_scale | f64 | 1 | 2393274.00 | 500.00 |
| dynamics_rossby_wave_speed | dynamics | RWS | Rossby wave speed | meteorology | dynamics::rossby_wave_speed | f64 | 1 | 2111584.00 | 500.00 |
| ocean_henry_solubility | ocean | HS | Henry solubility | meteorology | ocean::henry_solubility | f64 | 1 | 2025884.00 | 500.00 |
| ocean_sea_level_rise_thermal | ocean | SLR | Sea level rise thermal | meteorology | ocean::sea_level_rise_thermal | f64 | 1 | 2005704.00 | 500.00 |
| ocean_seawater_density | ocean | SD | Seawater density | meteorology | ocean::seawater_density | f64 | 1 | 2409994.00 | 500.00 |
| ocean_seawater_density_increases_with_salinity | ocean | SDI | Seawater density increases with salinity | meteorology | ocean::seawater_density_increases_with_salinity | f64 | 1 | 2207454.00 | 500.00 |
| ocean_sound_speed | ocean | SS | Sound speed | meteorology | ocean::sound_speed | f64 | 1 | 2152194.00 | 500.00 |
| precipitation_antecedent_precipitation_index | precipitation | API | Antecedent precipitation index | meteorology | precipitation::antecedent_precipitation_index | f64 | 1 | 1949214.00 | 1000.00 |
| precipitation_intensity_duration_frequency | precipitation | IDF | Intensity duration frequency | meteorology | precipitation::intensity_duration_frequency | f64 | 1 | 2425134.00 | 500.00 |
| precipitation_penman_evaporation | precipitation | PE | Penman evaporation | meteorology | precipitation::penman_evaporation | f64 | 1 | 1972724.00 | 1000.00 |
| precipitation_radar_reflectivity | precipitation | RR | Radar reflectivity | meteorology | precipitation::radar_reflectivity | f64 | 1 | 1723033.00 | 1000.00 |
| precipitation_rain_rate_marshall_palmer | precipitation | RRM | Rain rate marshall palmer | meteorology | precipitation::rain_rate_marshall_palmer | f64 | 1 | 1713723.00 | 1000.00 |
| precipitation_scs_curve_number_runoff | precipitation | SCN | Scs curve number runoff | meteorology | precipitation::scs_curve_number_runoff | f64 | 1 | 1861764.00 | 1000.00 |
| precipitation_terminal_velocity_raindrop | precipitation | TVR | Terminal velocity raindrop | meteorology | precipitation::terminal_velocity_raindrop | f64 | 1 | 2264804.00 | 500.00 |
| precipitation_thornthwaite_pet | precipitation | TP | Thornthwaite pet | meteorology | precipitation::thornthwaite_pet | f64 | 1 | 1898764.00 | 1000.00 |
| radiation_albedo_reflected | radiation | AR | Albedo reflected | meteorology | radiation::albedo_reflected | f64 | 1 | 1981263.00 | 1000.00 |
| radiation_effective_temperature | radiation | ET | Effective temperature | meteorology | radiation::effective_temperature | f64 | 1 | 1908454.00 | 1000.00 |
| radiation_planck_function | radiation | PF | Planck function | meteorology | radiation::planck_function | f64 | 1 | 2727955.00 | 500.00 |
| radiation_radiative_forcing_co2 | radiation | RFC | Radiative forcing co2 | meteorology | radiation::radiative_forcing_co2 | f64 | 1 | 2339504.00 | 500.00 |
| radiation_radiative_forcing_co2_zero_when_equal_concentrations | radiation | RFC | Radiative forcing co2 zero when equal concentrations | meteorology | radiation::radiative_forcing_co2_zero_when_equal_concentrations | f64 | 1 | 1817313.00 | 1000.00 |
| radiation_stefan_boltzmann_flux | radiation | SBF | Stefan boltzmann flux | meteorology | radiation::stefan_boltzmann_flux | f64 | 1 | 2082284.00 | 500.00 |
| radiation_stefan_boltzmann_flux_monotonic_with_temperature_sweep | radiation | SBF | Stefan boltzmann flux monotonic with temperature sweep | meteorology | radiation::stefan_boltzmann_flux_monotonic_with_temperature_sweep | f64 | 1 | 2349015.00 | 500.00 |
| storms_cape_positive_buoyancy | storms | CPB | Cape positive buoyancy | meteorology | storms::cape_positive_buoyancy | f64 | 1 | 1932573.00 | 1000.00 |
| storms_cape_zero_for_equal_temps | storms | CZF | Cape zero for equal temps | meteorology | storms::cape_zero_for_equal_temps | f64 | 1 | 2010034.00 | 500.00 |
| storms_fujita_scale | storms | FS | Fujita scale | meteorology | storms::fujita_scale | f64 | 1 | 1990644.00 | 1000.00 |
| storms_fujita_scale_increases_with_wind | storms | FSI | Fujita scale increases with wind | meteorology | storms::fujita_scale_increases_with_wind | f64 | 1 | 2630515.00 | 500.00 |
| storms_potential_intensity | storms | PI | Potential intensity | meteorology | storms::potential_intensity | f64 | 1 | 2195864.00 | 500.00 |
| winds_beaufort_monotonic | winds | BM | Beaufort monotonic | meteorology | winds::beaufort_monotonic | f64 | 1 | 1918213.00 | 1000.00 |
| winds_beaufort_to_m_s | winds | BTM | Beaufort to m s | meteorology | winds::beaufort_to_m_s | f64 | 1 | 2248564.00 | 500.00 |
| winds_beaufort_zero_is_calm | winds | BZI | Beaufort zero is calm | meteorology | winds::beaufort_zero_is_calm | f64 | 1 | 1974514.00 | 1000.00 |
| winds_wind_chill | winds | WC | Wind chill | meteorology | winds::wind_chill | f64 | 1 | 1870764.00 | 1000.00 |
| winds_wind_chill_decreases_with_wind | winds | WCD | Wind chill decreases with wind | meteorology | winds::wind_chill_decreases_with_wind | f64 | 1 | 2052073.00 | 500.00 |
## Analytics
### Category Distribution
| Category | Count | Avg (ns) | Min (ns) | Max (ns) |
|---|---|---|---|---|
| atmosphere | 7 | 2025453.71 | 1836393.00 | 2263685.00 |
| dynamics | 8 | 2021536.25 | 1803073.00 | 2393274.00 |
| ocean | 5 | 2160246.00 | 2005704.00 | 2409994.00 |
| precipitation | 8 | 1976145.00 | 1713723.00 | 2425134.00 |
| radiation | 7 | 2172255.43 | 1817313.00 | 2727955.00 |
| storms | 5 | 2151926.00 | 1932573.00 | 2630515.00 |
| winds | 5 | 2012825.60 | 1870764.00 | 2248564.00 |

### Top 10 Fastest Entries
| # | Entry | Avg (ns) |
|---|---|---|
| 1 | precipitation_rain_rate_marshall_palmer | 1713723.00 |
| 2 | precipitation_radar_reflectivity | 1723033.00 |
| 3 | dynamics_rossby_number | 1803073.00 |
| 4 | radiation_radiative_forcing_co2_zero_when_equal_concentrations | 1817313.00 |
| 5 | dynamics_coriolis_changes_sign_between_hemispheres | 1818384.00 |
| 6 | atmosphere_saturation_vapor_increases_with_temp | 1836393.00 |
| 7 | precipitation_scs_curve_number_runoff | 1861764.00 |
| 8 | winds_wind_chill | 1870764.00 |
| 9 | precipitation_thornthwaite_pet | 1898764.00 |
| 10 | dynamics_coriolis_zero_at_equator | 1901754.00 |

### Top 10 Slowest Entries
| # | Entry | Avg (ns) |
|---|---|---|
| 1 | radiation_planck_function | 2727955.00 |
| 2 | storms_fujita_scale_increases_with_wind | 2630515.00 |
| 3 | precipitation_intensity_duration_frequency | 2425134.00 |
| 4 | ocean_seawater_density | 2409994.00 |
| 5 | dynamics_rossby_number_decreases_with_scale | 2393274.00 |
| 6 | radiation_stefan_boltzmann_flux_monotonic_with_temperature_sweep | 2349015.00 |
| 7 | radiation_radiative_forcing_co2 | 2339504.00 |
| 8 | dynamics_ekman_depth | 2286924.00 |
| 9 | precipitation_terminal_velocity_raindrop | 2264804.00 |
| 10 | atmosphere_saturation_vapor_pressure | 2263685.00 |

