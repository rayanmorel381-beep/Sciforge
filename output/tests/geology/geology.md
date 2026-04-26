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
| dating_age_from_ratio | dating | AFR | Age from ratio | geology | dating::age_from_ratio | f64 | 1 | 2463039.00 | 500.00 |
| dating_carbon14_age | dating | CA | Carbon14 age | geology | dating::carbon14_age | f64 | 1 | 2086439.00 | 500.00 |
| dating_decay_constant | dating | DC | Decay constant | geology | dating::decay_constant | f64 | 1 | 2111509.00 | 500.00 |
| dating_half_life_positive_for_positive_lambda | dating | HLP | Half life positive for positive lambda | geology | dating::half_life_positive_for_positive_lambda | f64 | 1 | 1828439.00 | 1000.00 |
| dating_radioactive_decay | dating | RD | Radioactive decay | geology | dating::radioactive_decay | f64 | 1 | 2154569.00 | 500.00 |
| dating_radioactive_decay_halflife | dating | RDH | Radioactive decay halflife | geology | dating::radioactive_decay_halflife | f64 | 1 | 1737109.00 | 1000.00 |
| dating_radioactive_decay_large_time_stability | dating | RDL | Radioactive decay large time stability | geology | dating::radioactive_decay_large_time_stability | f64 | 1 | 1812260.00 | 1000.00 |
| erosion_chemical_weathering_rate | erosion | CWR | Chemical weathering rate | geology | erosion::chemical_weathering_rate | f64 | 1 | 1695009.00 | 1000.00 |
| erosion_fluvial_erosion_rate | erosion | FER | Fluvial erosion rate | geology | erosion::fluvial_erosion_rate | f64 | 1 | 1749459.00 | 1000.00 |
| erosion_frost_weathering_rate | erosion | FWR | Frost weathering rate | geology | erosion::frost_weathering_rate | f64 | 1 | 1697289.00 | 1000.00 |
| erosion_volcanic_explosivity_index | erosion | VEI | Volcanic explosivity index | geology | erosion::volcanic_explosivity_index | f64 | 1 | 1902340.00 | 1000.00 |
| erosion_wind_erosion_threshold | erosion | WET | Wind erosion threshold | geology | erosion::wind_erosion_threshold | f64 | 1 | 2304438.00 | 500.00 |
| glaciology_glacial_bed_erosion | glaciology | GBE | Glacial bed erosion | geology | glaciology::glacial_bed_erosion | f64 | 1 | 2156729.00 | 500.00 |
| glaciology_glen_strain_increases_with_stress | glaciology | GSI | Glen strain increases with stress | geology | glaciology::glen_strain_increases_with_stress | f64 | 1 | 2050680.00 | 500.00 |
| glaciology_glen_strain_rate | glaciology | GSR | Glen strain rate | geology | glaciology::glen_strain_rate | f64 | 1 | 1764559.00 | 1000.00 |
| glaciology_ice_viscosity | glaciology | IV | Ice viscosity | geology | glaciology::ice_viscosity | f64 | 1 | 1694559.00 | 1000.00 |
| glaciology_shallow_ice_velocity | glaciology | SIV | Shallow ice velocity | geology | glaciology::shallow_ice_velocity | f64 | 1 | 1719449.00 | 1000.00 |
| hydrology_chezy_velocity | hydrology | CV | Chezy velocity | geology | hydrology::chezy_velocity | f64 | 1 | 1775680.00 | 1000.00 |
| hydrology_froude_number | hydrology | FN | Froude number | geology | hydrology::froude_number | f64 | 1 | 2316238.00 | 500.00 |
| hydrology_hjulstrom_erosion_threshold | hydrology | HET | Hjulstrom erosion threshold | geology | hydrology::hjulstrom_erosion_threshold | f64 | 1 | 1943800.00 | 1000.00 |
| hydrology_manning_velocity | hydrology | MV | Manning velocity | geology | hydrology::manning_velocity | f64 | 1 | 2200399.00 | 500.00 |
| hydrology_reynolds_number | hydrology | RN | Reynolds number | geology | hydrology::reynolds_number | f64 | 1 | 1900639.00 | 1000.00 |
| hydrology_stream_power | hydrology | SP | Stream power | geology | hydrology::stream_power | f64 | 1 | 1702869.00 | 1000.00 |
| petrology_cipw_quartz_norm | petrology | CQN | Cipw quartz norm | geology | petrology::cipw_quartz_norm | f64 | 1 | 1794329.00 | 1000.00 |
| petrology_crystal_settling_velocity | petrology | CSV | Crystal settling velocity | geology | petrology::crystal_settling_velocity | f64 | 1 | 1765559.00 | 1000.00 |
| petrology_differentiation_index | petrology | DI | Differentiation index | geology | petrology::differentiation_index | f64 | 1 | 1679760.00 | 1000.00 |
| petrology_liquidus_temperature | petrology | LT | Liquidus temperature | geology | petrology::liquidus_temperature | f64 | 1 | 1673029.00 | 1000.00 |
| petrology_mg_number | petrology | MN | Mg number | geology | petrology::mg_number | f64 | 1 | 1691019.00 | 1000.00 |
| petrology_total_alkali_silica | petrology | TAS | Total alkali silica | geology | petrology::total_alkali_silica | f64 | 1 | 1693619.00 | 1000.00 |
| petrology_viscosity_arrhenius | petrology | VA | Viscosity arrhenius | geology | petrology::viscosity_arrhenius | f64 | 1 | 1682640.00 | 1000.00 |
| seismology_gutenberg_richter | seismology | GR | Gutenberg richter | geology | seismology::gutenberg_richter | f64 | 1 | 1704649.00 | 1000.00 |
| seismology_gutenberg_richter_decreases_with_magnitude | seismology | GRD | Gutenberg richter decreases with magnitude | geology | seismology::gutenberg_richter_decreases_with_magnitude | f64 | 1 | 1694049.00 | 1000.00 |
| seismology_p_wave_faster_than_s_wave | seismology | PWF | P wave faster than s wave | geology | seismology::p_wave_faster_than_s_wave | f64 | 1 | 1672330.00 | 1000.00 |
| seismology_p_wave_velocity | seismology | PWV | P wave velocity | geology | seismology::p_wave_velocity | f64 | 1 | 2398169.00 | 500.00 |
| seismology_richter_amplitude_ratio_adds_one_magnitude | seismology | RAR | Richter amplitude ratio adds one magnitude | geology | seismology::richter_amplitude_ratio_adds_one_magnitude | f64 | 1 | 1918229.00 | 1000.00 |
| seismology_richter_magnitude | seismology | RM | Richter magnitude | geology | seismology::richter_magnitude | f64 | 1 | 1805979.00 | 1000.00 |
| seismology_s_wave_velocity | seismology | SWV | S wave velocity | geology | seismology::s_wave_velocity | f64 | 1 | 1771049.00 | 1000.00 |
| seismology_seismic_energy | seismology | SE | Seismic energy | geology | seismology::seismic_energy | f64 | 1 | 1721629.00 | 1000.00 |
| tectonics_geothermal_gradient | tectonics | GG | Geothermal gradient | geology | tectonics::geothermal_gradient | f64 | 1 | 1749840.00 | 1000.00 |
| tectonics_heat_flow | tectonics | HF | Heat flow | geology | tectonics::heat_flow | f64 | 1 | 1757359.00 | 1000.00 |
| tectonics_isostatic_equilibrium | tectonics | IE | Isostatic equilibrium | geology | tectonics::isostatic_equilibrium | f64 | 1 | 2578839.00 | 500.00 |
| tectonics_mg_number | tectonics | MN | Mg number | geology | tectonics::mg_number | f64 | 1 | 2736879.00 | 500.00 |
| tectonics_plate_velocity | tectonics | PV | Plate velocity | geology | tectonics::plate_velocity | f64 | 1 | 2190369.00 | 500.00 |
| tectonics_viscosity_arrhenius | tectonics | VA | Viscosity arrhenius | geology | tectonics::viscosity_arrhenius | f64 | 1 | 2167569.00 | 500.00 |
## Analytics
### Category Distribution
| Category | Count | Avg (ns) | Min (ns) | Max (ns) |
|---|---|---|---|---|
| dating | 7 | 2027623.38 | 1737109.00 | 2463039.00 |
| erosion | 5 | 1869707.00 | 1695009.00 | 2304438.00 |
| glaciology | 5 | 1877195.25 | 1694559.00 | 2156729.00 |
| hydrology | 6 | 1973270.88 | 1702869.00 | 2316238.00 |
| petrology | 7 | 1711422.12 | 1673029.00 | 1794329.00 |
| seismology | 8 | 1835760.38 | 1672330.00 | 2398169.00 |
| tectonics | 6 | 2196809.25 | 1749840.00 | 2736879.00 |

### Top 10 Fastest Entries
| # | Entry | Avg (ns) |
|---|---|---|
| 1 | seismology_p_wave_faster_than_s_wave | 1672330.00 |
| 2 | petrology_liquidus_temperature | 1673029.00 |
| 3 | petrology_differentiation_index | 1679760.00 |
| 4 | petrology_viscosity_arrhenius | 1682640.00 |
| 5 | petrology_mg_number | 1691019.00 |
| 6 | petrology_total_alkali_silica | 1693619.00 |
| 7 | seismology_gutenberg_richter_decreases_with_magnitude | 1694049.00 |
| 8 | glaciology_ice_viscosity | 1694559.00 |
| 9 | erosion_chemical_weathering_rate | 1695009.00 |
| 10 | erosion_frost_weathering_rate | 1697289.00 |

### Top 10 Slowest Entries
| # | Entry | Avg (ns) |
|---|---|---|
| 1 | tectonics_mg_number | 2736879.00 |
| 2 | tectonics_isostatic_equilibrium | 2578839.00 |
| 3 | dating_age_from_ratio | 2463039.00 |
| 4 | seismology_p_wave_velocity | 2398169.00 |
| 5 | hydrology_froude_number | 2316238.00 |
| 6 | erosion_wind_erosion_threshold | 2304438.00 |
| 7 | hydrology_manning_velocity | 2200399.00 |
| 8 | tectonics_plate_velocity | 2190369.00 |
| 9 | tectonics_viscosity_arrhenius | 2167569.00 |
| 10 | glaciology_glacial_bed_erosion | 2156729.00 |

