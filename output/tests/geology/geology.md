# SciForge Geology Test

44 benchmark entries across 5 formats.

---

## Grid
```
  dating          : AFR   CA   DC  HLP   RD  RDH  RDL
  erosion         : CWR  FER  FWR  VEI  WET
  glaciology      : GBE  GSI  GSR   IV  SIV
  hydrology       :  CV   FN  HET   MV   RN   SP
  petrology       : CQN  CSV   DI   LT   MN  TAS   VA
  seismology      :  GR  GRD  PWF  PWV  RAR   RM  SWV   SE
  tectonics       :  GG   HF   IE   MN   PV   VA
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
| dating_age_from_ratio | dating | AFR | Age from ratio | geology | dating::age_from_ratio | f64 | 1 | 1957684.00 | 1000.00 |
| dating_carbon14_age | dating | CA | Carbon14 age | geology | dating::carbon14_age | f64 | 1 | 1862614.00 | 1000.00 |
| dating_decay_constant | dating | DC | Decay constant | geology | dating::decay_constant | f64 | 1 | 1760943.00 | 1000.00 |
| dating_half_life_positive_for_positive_lambda | dating | HLP | Half life positive for positive lambda | geology | dating::half_life_positive_for_positive_lambda | f64 | 1 | 2194624.00 | 500.00 |
| dating_radioactive_decay | dating | RD | Radioactive decay | geology | dating::radioactive_decay | f64 | 1 | 2253024.00 | 500.00 |
| dating_radioactive_decay_halflife | dating | RDH | Radioactive decay halflife | geology | dating::radioactive_decay_halflife | f64 | 1 | 2446995.00 | 500.00 |
| dating_radioactive_decay_large_time_stability | dating | RDL | Radioactive decay large time stability | geology | dating::radioactive_decay_large_time_stability | f64 | 1 | 2119463.00 | 500.00 |
| erosion_chemical_weathering_rate | erosion | CWR | Chemical weathering rate | geology | erosion::chemical_weathering_rate | f64 | 1 | 1923054.00 | 1000.00 |
| erosion_fluvial_erosion_rate | erosion | FER | Fluvial erosion rate | geology | erosion::fluvial_erosion_rate | f64 | 1 | 1895824.00 | 1000.00 |
| erosion_frost_weathering_rate | erosion | FWR | Frost weathering rate | geology | erosion::frost_weathering_rate | f64 | 1 | 1844403.00 | 1000.00 |
| erosion_volcanic_explosivity_index | erosion | VEI | Volcanic explosivity index | geology | erosion::volcanic_explosivity_index | f64 | 1 | 1776603.00 | 1000.00 |
| erosion_wind_erosion_threshold | erosion | WET | Wind erosion threshold | geology | erosion::wind_erosion_threshold | f64 | 1 | 2155604.00 | 500.00 |
| glaciology_glacial_bed_erosion | glaciology | GBE | Glacial bed erosion | geology | glaciology::glacial_bed_erosion | f64 | 1 | 2140534.00 | 500.00 |
| glaciology_glen_strain_increases_with_stress | glaciology | GSI | Glen strain increases with stress | geology | glaciology::glen_strain_increases_with_stress | f64 | 1 | 2047874.00 | 500.00 |
| glaciology_glen_strain_rate | glaciology | GSR | Glen strain rate | geology | glaciology::glen_strain_rate | f64 | 1 | 2249724.00 | 500.00 |
| glaciology_ice_viscosity | glaciology | IV | Ice viscosity | geology | glaciology::ice_viscosity | f64 | 1 | 2000324.00 | 500.00 |
| glaciology_shallow_ice_velocity | glaciology | SIV | Shallow ice velocity | geology | glaciology::shallow_ice_velocity | f64 | 1 | 1944933.00 | 1000.00 |
| hydrology_chezy_velocity | hydrology | CV | Chezy velocity | geology | hydrology::chezy_velocity | f64 | 1 | 1885934.00 | 1000.00 |
| hydrology_froude_number | hydrology | FN | Froude number | geology | hydrology::froude_number | f64 | 1 | 1862303.00 | 1000.00 |
| hydrology_hjulstrom_erosion_threshold | hydrology | HET | Hjulstrom erosion threshold | geology | hydrology::hjulstrom_erosion_threshold | f64 | 1 | 1990684.00 | 1000.00 |
| hydrology_manning_velocity | hydrology | MV | Manning velocity | geology | hydrology::manning_velocity | f64 | 1 | 2103384.00 | 500.00 |
| hydrology_reynolds_number | hydrology | RN | Reynolds number | geology | hydrology::reynolds_number | f64 | 1 | 1943003.00 | 1000.00 |
| hydrology_stream_power | hydrology | SP | Stream power | geology | hydrology::stream_power | f64 | 1 | 1903824.00 | 1000.00 |
| petrology_cipw_quartz_norm | petrology | CQN | Cipw quartz norm | geology | petrology::cipw_quartz_norm | f64 | 1 | 1851594.00 | 1000.00 |
| petrology_crystal_settling_velocity | petrology | CSV | Crystal settling velocity | geology | petrology::crystal_settling_velocity | f64 | 1 | 2236964.00 | 500.00 |
| petrology_differentiation_index | petrology | DI | Differentiation index | geology | petrology::differentiation_index | f64 | 1 | 1877293.00 | 1000.00 |
| petrology_liquidus_temperature | petrology | LT | Liquidus temperature | geology | petrology::liquidus_temperature | f64 | 1 | 1888234.00 | 1000.00 |
| petrology_mg_number | petrology | MN | Mg number | geology | petrology::mg_number | f64 | 1 | 1899423.00 | 1000.00 |
| petrology_total_alkali_silica | petrology | TAS | Total alkali silica | geology | petrology::total_alkali_silica | f64 | 1 | 2040144.00 | 500.00 |
| petrology_viscosity_arrhenius | petrology | VA | Viscosity arrhenius | geology | petrology::viscosity_arrhenius | f64 | 1 | 2239284.00 | 500.00 |
| seismology_gutenberg_richter | seismology | GR | Gutenberg richter | geology | seismology::gutenberg_richter | f64 | 1 | 1978554.00 | 1000.00 |
| seismology_gutenberg_richter_decreases_with_magnitude | seismology | GRD | Gutenberg richter decreases with magnitude | geology | seismology::gutenberg_richter_decreases_with_magnitude | f64 | 1 | 2242484.00 | 500.00 |
| seismology_p_wave_faster_than_s_wave | seismology | PWF | P wave faster than s wave | geology | seismology::p_wave_faster_than_s_wave | f64 | 1 | 2005804.00 | 500.00 |
| seismology_p_wave_velocity | seismology | PWV | P wave velocity | geology | seismology::p_wave_velocity | f64 | 1 | 1948843.00 | 1000.00 |
| seismology_richter_amplitude_ratio_adds_one_magnitude | seismology | RAR | Richter amplitude ratio adds one magnitude | geology | seismology::richter_amplitude_ratio_adds_one_magnitude | f64 | 1 | 1899544.00 | 1000.00 |
| seismology_richter_magnitude | seismology | RM | Richter magnitude | geology | seismology::richter_magnitude | f64 | 1 | 2141804.00 | 500.00 |
| seismology_s_wave_velocity | seismology | SWV | S wave velocity | geology | seismology::s_wave_velocity | f64 | 1 | 2165694.00 | 500.00 |
| seismology_seismic_energy | seismology | SE | Seismic energy | geology | seismology::seismic_energy | f64 | 1 | 2427544.00 | 500.00 |
| tectonics_geothermal_gradient | tectonics | GG | Geothermal gradient | geology | tectonics::geothermal_gradient | f64 | 1 | 2344304.00 | 500.00 |
| tectonics_heat_flow | tectonics | HF | Heat flow | geology | tectonics::heat_flow | f64 | 1 | 2084524.00 | 500.00 |
| tectonics_isostatic_equilibrium | tectonics | IE | Isostatic equilibrium | geology | tectonics::isostatic_equilibrium | f64 | 1 | 1876944.00 | 1000.00 |
| tectonics_mg_number | tectonics | MN | Mg number | geology | tectonics::mg_number | f64 | 1 | 1780093.00 | 1000.00 |
| tectonics_plate_velocity | tectonics | PV | Plate velocity | geology | tectonics::plate_velocity | f64 | 1 | 1756523.00 | 1000.00 |
| tectonics_viscosity_arrhenius | tectonics | VA | Viscosity arrhenius | geology | tectonics::viscosity_arrhenius | f64 | 1 | 1853304.00 | 1000.00 |
## Analytics
### Category Distribution
| Category | Count | Avg (ns) | Min (ns) | Max (ns) |
|---|---|---|---|---|
| dating | 7 | 2085049.57 | 1760943.00 | 2446995.00 |
| erosion | 5 | 1919097.60 | 1776603.00 | 2155604.00 |
| glaciology | 5 | 2076677.80 | 1944933.00 | 2249724.00 |
| hydrology | 6 | 1948188.67 | 1862303.00 | 2103384.00 |
| petrology | 7 | 2004705.14 | 1851594.00 | 2239284.00 |
| seismology | 8 | 2101283.88 | 1899544.00 | 2427544.00 |
| tectonics | 6 | 1949282.00 | 1756523.00 | 2344304.00 |

### Top 10 Fastest Entries
| # | Entry | Avg (ns) |
|---|---|---|
| 1 | tectonics_plate_velocity | 1756523.00 |
| 2 | dating_decay_constant | 1760943.00 |
| 3 | erosion_volcanic_explosivity_index | 1776603.00 |
| 4 | tectonics_mg_number | 1780093.00 |
| 5 | erosion_frost_weathering_rate | 1844403.00 |
| 6 | petrology_cipw_quartz_norm | 1851594.00 |
| 7 | tectonics_viscosity_arrhenius | 1853304.00 |
| 8 | hydrology_froude_number | 1862303.00 |
| 9 | dating_carbon14_age | 1862614.00 |
| 10 | tectonics_isostatic_equilibrium | 1876944.00 |

### Top 10 Slowest Entries
| # | Entry | Avg (ns) |
|---|---|---|
| 1 | dating_radioactive_decay_halflife | 2446995.00 |
| 2 | seismology_seismic_energy | 2427544.00 |
| 3 | tectonics_geothermal_gradient | 2344304.00 |
| 4 | dating_radioactive_decay | 2253024.00 |
| 5 | glaciology_glen_strain_rate | 2249724.00 |
| 6 | seismology_gutenberg_richter_decreases_with_magnitude | 2242484.00 |
| 7 | petrology_viscosity_arrhenius | 2239284.00 |
| 8 | petrology_crystal_settling_velocity | 2236964.00 |
| 9 | dating_half_life_positive_for_positive_lambda | 2194624.00 |
| 10 | seismology_s_wave_velocity | 2165694.00 |

