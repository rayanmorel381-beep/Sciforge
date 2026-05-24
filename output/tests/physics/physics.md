# SciForge Physics Test

73 benchmark entries across 5 formats.

---

## Grid
```
  acoustics       :  DA  ILD  RTS  SOS
  dispatch        : DGM  DMP  DUF   DV   HC  MSD
  electrodynamics : CPP  EFP   SD
  electromagnetism: CPP  EFP   IS  MFW   ML  MLQ   OC  OLR   OR   OV
  electronics     :  DS   OC  RRF   VD
  fluid_mechanics :  BP   DF   RN
  materials       :  DC  FFL   VC
  mechanics       :  BF   BP  DSW   DF   HS   HS   MN
  nucleosynthesis : BEP   CB  HLF  NRF
  optics          :  BA   RC   Sn  TLE
  particle        :  CT  FSC   PE   PE
  quantum         : HOE   HE  IWE   TC
  relativity      : CWS  EMR  GFA  GFI   LC  PRP   TD  TDH
  solid_mechanics :  HS   TS   VM
  thermodynamics  :  BD   CE  IGP  IGT  SBP   WD
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
| acoustics_doppler_approaching | acoustics | DA | Doppler approaching | physics | acoustics::doppler_approaching | f64 | 1 | 2238924.00 | 500.00 |
| acoustics_intensity_level_db | acoustics | ILD | Intensity level db | physics | acoustics::intensity_level_db | f64 | 1 | 1823883.00 | 1000.00 |
| acoustics_reverberation_time_sabine | acoustics | RTS | Reverberation time sabine | physics | acoustics::reverberation_time_sabine | f64 | 1 | 1762503.00 | 1000.00 |
| acoustics_speed_of_sound_gas | acoustics | SOS | Speed of sound gas | physics | acoustics::speed_of_sound_gas | f64 | 1 | 1791574.00 | 1000.00 |
| dispatch_diffraction_grating_maxima | dispatch | DGM | Diffraction grating maxima | physics | dispatch::diffraction_grating_maxima | f64 | 1 | 2316134.00 | 500.00 |
| dispatch_dispatch_missing_parameter_returns_error | dispatch | DMP | Dispatch missing parameter returns error | physics | dispatch::dispatch_missing_parameter_returns_error | f64 | 1 | 2695035.00 | 500.00 |
| dispatch_dispatch_unknown_function_returns_error | dispatch | DUF | Dispatch unknown function returns error | physics | dispatch::dispatch_unknown_function_returns_error | f64 | 1 | 2094074.00 | 500.00 |
| dispatch_drift_velocity | dispatch | DV | Drift velocity | physics | dispatch::drift_velocity | f64 | 1 | 1928504.00 | 1000.00 |
| dispatch_hall_coefficient | dispatch | HC | Hall coefficient | physics | dispatch::hall_coefficient | f64 | 1 | 1840533.00 | 1000.00 |
| dispatch_maxwell_speed_distribution_peak | dispatch | MSD | Maxwell speed distribution peak | physics | dispatch::maxwell_speed_distribution_peak | f64 | 1 | 1859823.00 | 1000.00 |
| electrodynamics_capacitance_parallel_plate | electrodynamics | CPP | Capacitance parallel plate | physics | electrodynamics::capacitance_parallel_plate | f64 | 1 | 1757714.00 | 1000.00 |
| electrodynamics_electric_field_point_charge | electrodynamics | EFP | Electric field point charge | physics | electrodynamics::electric_field_point_charge | f64 | 1 | 1873473.00 | 1000.00 |
| electrodynamics_skin_depth | electrodynamics | SD | Skin depth | physics | electrodynamics::skin_depth | f64 | 1 | 2021624.00 | 500.00 |
| electromagnetism_capacitance_parallel_plate | electromagnetism | CPP | Capacitance parallel plate | physics | electromagnetism::capacitance_parallel_plate | f64 | 1 | 2172614.00 | 500.00 |
| electromagnetism_electric_field_point_charge | electromagnetism | EFP | Electric field point charge | physics | electromagnetism::electric_field_point_charge | f64 | 1 | 1898793.00 | 1000.00 |
| electromagnetism_inductance_solenoid | electromagnetism | IS | Inductance solenoid | physics | electromagnetism::inductance_solenoid | f64 | 1 | 1768804.00 | 1000.00 |
| electromagnetism_magnetic_field_wire | electromagnetism | MFW | Magnetic field wire | physics | electromagnetism::magnetic_field_wire | f64 | 1 | 1791073.00 | 1000.00 |
| electromagnetism_malus_law | electromagnetism | ML | Malus law | physics | electromagnetism::malus_law | f64 | 1 | 2191744.00 | 500.00 |
| electromagnetism_malus_law_quarter_turn_extinction | electromagnetism | MLQ | Malus law quarter turn extinction | physics | electromagnetism::malus_law_quarter_turn_extinction | f64 | 1 | 2958816.00 | 500.00 |
| electromagnetism_ohm_current | electromagnetism | OC | Ohm current | physics | electromagnetism::ohm_current | f64 | 1 | 2743115.00 | 500.00 |
| electromagnetism_ohm_law_roundtrip_consistency | electromagnetism | OLR | Ohm law roundtrip consistency | physics | electromagnetism::ohm_law_roundtrip_consistency | f64 | 1 | 2972305.00 | 500.00 |
| electromagnetism_ohm_resistance | electromagnetism | OR | Ohm resistance | physics | electromagnetism::ohm_resistance | f64 | 1 | 2644235.00 | 500.00 |
| electromagnetism_ohm_voltage | electromagnetism | OV | Ohm voltage | physics | electromagnetism::ohm_voltage | f64 | 1 | 2411474.00 | 500.00 |
| electronics_diode_shockley | electronics | DS | Diode shockley | physics | electronics::diode_shockley | f64 | 1 | 2472625.00 | 500.00 |
| electronics_ohm_current | electronics | OC | Ohm current | physics | electronics::ohm_current | f64 | 1 | 2131514.00 | 500.00 |
| electronics_rlc_resonant_frequency | electronics | RRF | Rlc resonant frequency | physics | electronics::rlc_resonant_frequency | f64 | 1 | 2358184.00 | 500.00 |
| electronics_voltage_divider | electronics | VD | Voltage divider | physics | electronics::voltage_divider | f64 | 1 | 2147694.00 | 500.00 |
| fluid_mechanics_bernoulli_pressure | fluid_mechanics | BP | Bernoulli pressure | physics | fluid_mechanics::bernoulli_pressure | f64 | 1 | 2116874.00 | 500.00 |
| fluid_mechanics_drag_force | fluid_mechanics | DF | Drag force | physics | fluid_mechanics::drag_force | f64 | 1 | 2476255.00 | 500.00 |
| fluid_mechanics_reynolds_number | fluid_mechanics | RN | Reynolds number | physics | fluid_mechanics::reynolds_number | f64 | 1 | 2254974.00 | 500.00 |
| materials_diffusion_coefficient | materials | DC | Diffusion coefficient | physics | materials::diffusion_coefficient | f64 | 1 | 1933984.00 | 1000.00 |
| materials_fick_first_law | materials | FFL | Fick first law | physics | materials::fick_first_law | f64 | 1 | 1746043.00 | 1000.00 |
| materials_vacancy_concentration | materials | VC | Vacancy concentration | physics | materials::vacancy_concentration | f64 | 1 | 1872913.00 | 1000.00 |
| mechanics_beat_frequency | mechanics | BF | Beat frequency | physics | mechanics::beat_frequency | f64 | 1 | 2258654.00 | 500.00 |
| mechanics_bernoulli_pressure | mechanics | BP | Bernoulli pressure | physics | mechanics::bernoulli_pressure | f64 | 1 | 1993723.00 | 1000.00 |
| mechanics_doppler_shift_wavelength | mechanics | DSW | Doppler shift wavelength | physics | mechanics::doppler_shift_wavelength | f64 | 1 | 2208294.00 | 500.00 |
| mechanics_drag_force | mechanics | DF | Drag force | physics | mechanics::drag_force | f64 | 1 | 2011824.00 | 500.00 |
| mechanics_hooke_strain | mechanics | HS | Hooke strain | physics | mechanics::hooke_strain | f64 | 1 | 1852213.00 | 1000.00 |
| mechanics_hooke_stress | mechanics | HS | Hooke stress | physics | mechanics::hooke_stress | f64 | 1 | 1842514.00 | 1000.00 |
| mechanics_mach_number | mechanics | MN | Mach number | physics | mechanics::mach_number | f64 | 1 | 1975193.00 | 1000.00 |
| nucleosynthesis_binding_energy_per_nucleon_semf | nucleosynthesis | BEP | Binding energy per nucleon semf | physics | nucleosynthesis::binding_energy_per_nucleon_semf | f64 | 1 | 2445165.00 | 500.00 |
| nucleosynthesis_coulomb_barrier | nucleosynthesis | CB | Coulomb barrier | physics | nucleosynthesis::coulomb_barrier | f64 | 1 | 2023114.00 | 500.00 |
| nucleosynthesis_half_life_from_constant | nucleosynthesis | HLF | Half life from constant | physics | nucleosynthesis::half_life_from_constant | f64 | 1 | 2538144.00 | 500.00 |
| nucleosynthesis_nuclear_radius_fm | nucleosynthesis | NRF | Nuclear radius fm | physics | nucleosynthesis::nuclear_radius_fm | f64 | 1 | 2240855.00 | 500.00 |
| optics_brewster_angle | optics | BA | Brewster angle | physics | optics::brewster_angle | f64 | 1 | 1860873.00 | 1000.00 |
| optics_rayleigh_criterion | optics | RC | Rayleigh criterion | physics | optics::rayleigh_criterion | f64 | 1 | 2033634.00 | 500.00 |
| optics_snell | optics | Sn | Snell | physics | optics::snell | f64 | 1 | 2312144.00 | 500.00 |
| optics_thin_lens_equation | optics | TLE | Thin lens equation | physics | optics::thin_lens_equation | f64 | 1 | 1942894.00 | 1000.00 |
| particle_compton_time | particle | CT | Compton time | physics | particle::compton_time | f64 | 1 | 1830173.00 | 1000.00 |
| particle_fine_structure_constant | particle | FSC | Fine structure constant | physics | particle::fine_structure_constant | f64 | 1 | 2239534.00 | 500.00 |
| particle_photon_energy | particle | PE | Photon energy | physics | particle::photon_energy | f64 | 1 | 2331164.00 | 500.00 |
| particle_planck_energy | particle | PE | Planck energy | physics | particle::planck_energy | f64 | 1 | 2100374.00 | 500.00 |
| quantum_harmonic_oscillator_energy | quantum | HOE | Harmonic oscillator energy | physics | quantum::harmonic_oscillator_energy | f64 | 1 | 1980854.00 | 1000.00 |
| quantum_hydrogen_energy | quantum | HE | Hydrogen energy | physics | quantum::hydrogen_energy | f64 | 1 | 1872043.00 | 1000.00 |
| quantum_infinite_well_energy | quantum | IWE | Infinite well energy | physics | quantum::infinite_well_energy | f64 | 1 | 1836414.00 | 1000.00 |
| quantum_tunneling_coefficient | quantum | TC | Tunneling coefficient | physics | quantum::tunneling_coefficient | f64 | 1 | 2500275.00 | 500.00 |
| relativity_compton_wavelength_shift | relativity | CWS | Compton wavelength shift | physics | relativity::compton_wavelength_shift | f64 | 1 | 2293164.00 | 500.00 |
| relativity_energy_momentum_relation | relativity | EMR | Energy momentum relation | physics | relativity::energy_momentum_relation | f64 | 1 | 2073874.00 | 500.00 |
| relativity_gamma_factor_at_rest | relativity | GFA | Gamma factor at rest | physics | relativity::gamma_factor_at_rest | f64 | 1 | 2035943.00 | 500.00 |
| relativity_gamma_factor_increases_with_velocity | relativity | GFI | Gamma factor increases with velocity | physics | relativity::gamma_factor_increases_with_velocity | f64 | 1 | 2117754.00 | 500.00 |
| relativity_length_contraction | relativity | LC | Length contraction | physics | relativity::length_contraction | f64 | 1 | 2524345.00 | 500.00 |
| relativity_planck_radiation_positive | relativity | PRP | Planck radiation positive | physics | relativity::planck_radiation_positive | f64 | 1 | 2137304.00 | 500.00 |
| relativity_time_dilation | relativity | TD | Time dilation | physics | relativity::time_dilation | f64 | 1 | 2135114.00 | 500.00 |
| relativity_time_dilation_high_v | relativity | TDH | Time dilation high v | physics | relativity::time_dilation_high_v | f64 | 1 | 2090724.00 | 500.00 |
| solid_mechanics_hooke_stress | solid_mechanics | HS | Hooke stress | physics | solid_mechanics::hooke_stress | f64 | 1 | 1870773.00 | 1000.00 |
| solid_mechanics_thermal_stress | solid_mechanics | TS | Thermal stress | physics | solid_mechanics::thermal_stress | f64 | 1 | 1903164.00 | 1000.00 |
| solid_mechanics_von_mises | solid_mechanics | VM | Von mises | physics | solid_mechanics::von_mises | f64 | 1 | 2029323.00 | 500.00 |
| thermodynamics_boltzmann_distribution | thermodynamics | BD | Boltzmann distribution | physics | thermodynamics::boltzmann_distribution | f64 | 1 | 2297755.00 | 500.00 |
| thermodynamics_carnot_efficiency | thermodynamics | CE | Carnot efficiency | physics | thermodynamics::carnot_efficiency | f64 | 1 | 2522254.00 | 500.00 |
| thermodynamics_ideal_gas_pressure | thermodynamics | IGP | Ideal gas pressure | physics | thermodynamics::ideal_gas_pressure | f64 | 1 | 2131304.00 | 500.00 |
| thermodynamics_ideal_gas_temperature | thermodynamics | IGT | Ideal gas temperature | physics | thermodynamics::ideal_gas_temperature | f64 | 1 | 2096884.00 | 500.00 |
| thermodynamics_stefan_boltzmann_power | thermodynamics | SBP | Stefan boltzmann power | physics | thermodynamics::stefan_boltzmann_power | f64 | 1 | 2123454.00 | 500.00 |
| thermodynamics_wien_displacement | thermodynamics | WD | Wien displacement | physics | thermodynamics::wien_displacement | f64 | 1 | 1964834.00 | 1000.00 |
## Analytics
### Category Distribution
| Category | Count | Avg (ns) | Min (ns) | Max (ns) |
|---|---|---|---|---|
| acoustics | 4 | 1904221.00 | 1762503.00 | 2238924.00 |
| dispatch | 6 | 2122350.50 | 1840533.00 | 2695035.00 |
| electrodynamics | 3 | 1884270.33 | 1757714.00 | 2021624.00 |
| electromagnetism | 10 | 2355297.30 | 1768804.00 | 2972305.00 |
| electronics | 4 | 2277504.25 | 2131514.00 | 2472625.00 |
| fluid_mechanics | 3 | 2282701.00 | 2116874.00 | 2476255.00 |
| materials | 3 | 1850980.00 | 1746043.00 | 1933984.00 |
| mechanics | 7 | 2020345.00 | 1842514.00 | 2258654.00 |
| nucleosynthesis | 4 | 2311819.50 | 2023114.00 | 2538144.00 |
| optics | 4 | 2037386.25 | 1860873.00 | 2312144.00 |
| particle | 4 | 2125311.25 | 1830173.00 | 2331164.00 |
| quantum | 4 | 2047396.50 | 1836414.00 | 2500275.00 |
| relativity | 8 | 2176027.75 | 2035943.00 | 2524345.00 |
| solid_mechanics | 3 | 1934420.00 | 1870773.00 | 2029323.00 |
| thermodynamics | 6 | 2189414.17 | 1964834.00 | 2522254.00 |

### Top 10 Fastest Entries
| # | Entry | Avg (ns) |
|---|---|---|
| 1 | materials_fick_first_law | 1746043.00 |
| 2 | electrodynamics_capacitance_parallel_plate | 1757714.00 |
| 3 | acoustics_reverberation_time_sabine | 1762503.00 |
| 4 | electromagnetism_inductance_solenoid | 1768804.00 |
| 5 | electromagnetism_magnetic_field_wire | 1791073.00 |
| 6 | acoustics_speed_of_sound_gas | 1791574.00 |
| 7 | acoustics_intensity_level_db | 1823883.00 |
| 8 | particle_compton_time | 1830173.00 |
| 9 | quantum_infinite_well_energy | 1836414.00 |
| 10 | dispatch_hall_coefficient | 1840533.00 |

### Top 10 Slowest Entries
| # | Entry | Avg (ns) |
|---|---|---|
| 1 | electromagnetism_ohm_law_roundtrip_consistency | 2972305.00 |
| 2 | electromagnetism_malus_law_quarter_turn_extinction | 2958816.00 |
| 3 | electromagnetism_ohm_current | 2743115.00 |
| 4 | dispatch_dispatch_missing_parameter_returns_error | 2695035.00 |
| 5 | electromagnetism_ohm_resistance | 2644235.00 |
| 6 | nucleosynthesis_half_life_from_constant | 2538144.00 |
| 7 | relativity_length_contraction | 2524345.00 |
| 8 | thermodynamics_carnot_efficiency | 2522254.00 |
| 9 | quantum_tunneling_coefficient | 2500275.00 |
| 10 | fluid_mechanics_drag_force | 2476255.00 |

