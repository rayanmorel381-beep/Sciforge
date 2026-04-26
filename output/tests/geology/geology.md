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
| dating_age_from_ratio | dating | AFR | Age from ratio | geology | dating::age_from_ratio | f64 | 1 | 2345040.00 | 500.00 |
| dating_carbon14_age | dating | CA | Carbon14 age | geology | dating::carbon14_age | f64 | 1 | 1887980.00 | 1000.00 |
| dating_decay_constant | dating | DC | Decay constant | geology | dating::decay_constant | f64 | 1 | 1800390.00 | 1000.00 |
| dating_half_life_positive_for_positive_lambda | dating | HLP | Half life positive for positive lambda | geology | dating::half_life_positive_for_positive_lambda | f64 | 1 | 1754751.00 | 1000.00 |
| dating_radioactive_decay | dating | RD | Radioactive decay | geology | dating::radioactive_decay | f64 | 1 | 1728440.00 | 1000.00 |
| dating_radioactive_decay_halflife | dating | RDH | Radioactive decay halflife | geology | dating::radioactive_decay_halflife | f64 | 1 | 1733370.00 | 1000.00 |
| dating_radioactive_decay_large_time_stability | dating | RDL | Radioactive decay large time stability | geology | dating::radioactive_decay_large_time_stability | f64 | 1 | 1787941.00 | 1000.00 |
| erosion_chemical_weathering_rate | erosion | CWR | Chemical weathering rate | geology | erosion::chemical_weathering_rate | f64 | 1 | 1882400.00 | 1000.00 |
| erosion_fluvial_erosion_rate | erosion | FER | Fluvial erosion rate | geology | erosion::fluvial_erosion_rate | f64 | 1 | 2511600.00 | 500.00 |
| erosion_frost_weathering_rate | erosion | FWR | Frost weathering rate | geology | erosion::frost_weathering_rate | f64 | 1 | 1810831.00 | 1000.00 |
| erosion_volcanic_explosivity_index | erosion | VEI | Volcanic explosivity index | geology | erosion::volcanic_explosivity_index | f64 | 1 | 1739600.00 | 1000.00 |
| erosion_wind_erosion_threshold | erosion | WET | Wind erosion threshold | geology | erosion::wind_erosion_threshold | f64 | 1 | 1769440.00 | 1000.00 |
| glaciology_glacial_bed_erosion | glaciology | GBE | Glacial bed erosion | geology | glaciology::glacial_bed_erosion | f64 | 1 | 1756320.00 | 1000.00 |
| glaciology_glen_strain_increases_with_stress | glaciology | GSI | Glen strain increases with stress | geology | glaciology::glen_strain_increases_with_stress | f64 | 1 | 1900971.00 | 1000.00 |
| glaciology_glen_strain_rate | glaciology | GSR | Glen strain rate | geology | glaciology::glen_strain_rate | f64 | 1 | 2354760.00 | 500.00 |
| glaciology_ice_viscosity | glaciology | IV | Ice viscosity | geology | glaciology::ice_viscosity | f64 | 1 | 2614761.00 | 500.00 |
| glaciology_shallow_ice_velocity | glaciology | SIV | Shallow ice velocity | geology | glaciology::shallow_ice_velocity | f64 | 1 | 2273110.00 | 500.00 |
| hydrology_chezy_velocity | hydrology | CV | Chezy velocity | geology | hydrology::chezy_velocity | f64 | 1 | 2124550.00 | 500.00 |
| hydrology_froude_number | hydrology | FN | Froude number | geology | hydrology::froude_number | f64 | 1 | 1898791.00 | 1000.00 |
| hydrology_hjulstrom_erosion_threshold | hydrology | HET | Hjulstrom erosion threshold | geology | hydrology::hjulstrom_erosion_threshold | f64 | 1 | 1963620.00 | 1000.00 |
| hydrology_manning_velocity | hydrology | MV | Manning velocity | geology | hydrology::manning_velocity | f64 | 1 | 1906670.00 | 1000.00 |
| hydrology_reynolds_number | hydrology | RN | Reynolds number | geology | hydrology::reynolds_number | f64 | 1 | 2168321.00 | 500.00 |
| hydrology_stream_power | hydrology | SP | Stream power | geology | hydrology::stream_power | f64 | 1 | 2073870.00 | 500.00 |
| petrology_cipw_quartz_norm | petrology | CQN | Cipw quartz norm | geology | petrology::cipw_quartz_norm | f64 | 1 | 2267000.00 | 500.00 |
| petrology_crystal_settling_velocity | petrology | CSV | Crystal settling velocity | geology | petrology::crystal_settling_velocity | f64 | 1 | 1938511.00 | 1000.00 |
| petrology_differentiation_index | petrology | DI | Differentiation index | geology | petrology::differentiation_index | f64 | 1 | 1799500.00 | 1000.00 |
| petrology_liquidus_temperature | petrology | LT | Liquidus temperature | geology | petrology::liquidus_temperature | f64 | 1 | 1705510.00 | 1000.00 |
| petrology_mg_number | petrology | MN | Mg number | geology | petrology::mg_number | f64 | 1 | 1709191.00 | 1000.00 |
| petrology_total_alkali_silica | petrology | TAS | Total alkali silica | geology | petrology::total_alkali_silica | f64 | 1 | 1786690.00 | 1000.00 |
| petrology_viscosity_arrhenius | petrology | VA | Viscosity arrhenius | geology | petrology::viscosity_arrhenius | f64 | 1 | 1731330.00 | 1000.00 |
| seismology_gutenberg_richter | seismology | GR | Gutenberg richter | geology | seismology::gutenberg_richter | f64 | 1 | 1956001.00 | 1000.00 |
| seismology_gutenberg_richter_decreases_with_magnitude | seismology | GRD | Gutenberg richter decreases with magnitude | geology | seismology::gutenberg_richter_decreases_with_magnitude | f64 | 1 | 2529250.00 | 500.00 |
| seismology_p_wave_faster_than_s_wave | seismology | PWF | P wave faster than s wave | geology | seismology::p_wave_faster_than_s_wave | f64 | 1 | 1862510.00 | 1000.00 |
| seismology_p_wave_velocity | seismology | PWV | P wave velocity | geology | seismology::p_wave_velocity | f64 | 1 | 1804261.00 | 1000.00 |
| seismology_richter_amplitude_ratio_adds_one_magnitude | seismology | RAR | Richter amplitude ratio adds one magnitude | geology | seismology::richter_amplitude_ratio_adds_one_magnitude | f64 | 1 | 1762060.00 | 1000.00 |
| seismology_richter_magnitude | seismology | RM | Richter magnitude | geology | seismology::richter_magnitude | f64 | 1 | 1722690.00 | 1000.00 |
| seismology_s_wave_velocity | seismology | SWV | S wave velocity | geology | seismology::s_wave_velocity | f64 | 1 | 1848020.00 | 1000.00 |
| seismology_seismic_energy | seismology | SE | Seismic energy | geology | seismology::seismic_energy | f64 | 1 | 2339891.00 | 500.00 |
| tectonics_geothermal_gradient | tectonics | GG | Geothermal gradient | geology | tectonics::geothermal_gradient | f64 | 1 | 1956800.00 | 1000.00 |
| tectonics_heat_flow | tectonics | HF | Heat flow | geology | tectonics::heat_flow | f64 | 1 | 1980480.00 | 1000.00 |
| tectonics_isostatic_equilibrium | tectonics | IE | Isostatic equilibrium | geology | tectonics::isostatic_equilibrium | f64 | 1 | 1977931.00 | 1000.00 |
| tectonics_mg_number | tectonics | MN | Mg number | geology | tectonics::mg_number | f64 | 1 | 1907140.00 | 1000.00 |
| tectonics_plate_velocity | tectonics | PV | Plate velocity | geology | tectonics::plate_velocity | f64 | 1 | 2192391.00 | 500.00 |
| tectonics_viscosity_arrhenius | tectonics | VA | Viscosity arrhenius | geology | tectonics::viscosity_arrhenius | f64 | 1 | 1932310.00 | 1000.00 |
## Analytics
### Category Distribution
| Category | Count | Avg (ns) | Min (ns) | Max (ns) |
|---|---|---|---|---|
| dating | 7 | 1862558.86 | 1728440.00 | 2345040.00 |
| erosion | 5 | 1942774.20 | 1739600.00 | 2511600.00 |
| glaciology | 5 | 2179984.40 | 1756320.00 | 2614761.00 |
| hydrology | 6 | 2022637.00 | 1898791.00 | 2168321.00 |
| petrology | 7 | 1848247.43 | 1705510.00 | 2267000.00 |
| seismology | 8 | 1978085.38 | 1722690.00 | 2529250.00 |
| tectonics | 6 | 1991175.33 | 1907140.00 | 2192391.00 |

### Top 10 Fastest Entries
| # | Entry | Avg (ns) |
|---|---|---|
| 1 | petrology_liquidus_temperature | 1705510.00 |
| 2 | petrology_mg_number | 1709191.00 |
| 3 | seismology_richter_magnitude | 1722690.00 |
| 4 | dating_radioactive_decay | 1728440.00 |
| 5 | petrology_viscosity_arrhenius | 1731330.00 |
| 6 | dating_radioactive_decay_halflife | 1733370.00 |
| 7 | erosion_volcanic_explosivity_index | 1739600.00 |
| 8 | dating_half_life_positive_for_positive_lambda | 1754751.00 |
| 9 | glaciology_glacial_bed_erosion | 1756320.00 |
| 10 | seismology_richter_amplitude_ratio_adds_one_magnitude | 1762060.00 |

### Top 10 Slowest Entries
| # | Entry | Avg (ns) |
|---|---|---|
| 1 | glaciology_ice_viscosity | 2614761.00 |
| 2 | seismology_gutenberg_richter_decreases_with_magnitude | 2529250.00 |
| 3 | erosion_fluvial_erosion_rate | 2511600.00 |
| 4 | glaciology_glen_strain_rate | 2354760.00 |
| 5 | dating_age_from_ratio | 2345040.00 |
| 6 | seismology_seismic_energy | 2339891.00 |
| 7 | glaciology_shallow_ice_velocity | 2273110.00 |
| 8 | petrology_cipw_quartz_norm | 2267000.00 |
| 9 | tectonics_plate_velocity | 2192391.00 |
| 10 | hydrology_reynolds_number | 2168321.00 |

