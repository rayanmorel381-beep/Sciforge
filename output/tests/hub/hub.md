# SciForge Hub Test

82 benchmark entries across 5 formats.

---

## Grid
```
  cross_domain    :  DE  HZI  HZO   SG  USF  DET   FT   JM   SR   CW   EL   HT   SR  AOD   CD  HLC   PR  DAL  PAA  SBF  WPW  ETN  GFE   HH  MMR  PFC  BRN  LGR  MCD  SDI   EM  FDF   RN  SDF  CAA   CE  CPA  CRS  CDB  CDE   DN   DC  MTC   PC   BA  ESD  SIR  SWA  IFR  DBW  FDD  HUP   RE  CCS   IE  STE  VER  VDB  VMO  VSC
  pipeline        : PCS  PEI  PNA   PS
  tools           : AEW  AAA  ACL   AM   AR  FCW   FD  FSO  KDB  KSP   LB   LB  RCF   RD  RDS  RND  RUR  SPI
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
| cross_domain_astrobiology_drake_equation | cross_domain | DE | Drake equation | hub | cross_domain::astrobiology::drake_equation | f64 | 1 | 1983080.00 | 1000.00 |
| cross_domain_astrobiology_habitable_zone_inner | cross_domain | HZI | Habitable zone inner | hub | cross_domain::astrobiology::habitable_zone_inner | f64 | 1 | 1997880.00 | 1000.00 |
| cross_domain_astrobiology_habitable_zone_outer | cross_domain | HZO | Habitable zone outer | hub | cross_domain::astrobiology::habitable_zone_outer | f64 | 1 | 1852981.00 | 1000.00 |
| cross_domain_astrobiology_surface_gravity | cross_domain | SG | Surface gravity | hub | cross_domain::astrobiology::surface_gravity | f64 | 1 | 1727810.00 | 1000.00 |
| cross_domain_astrobiology_uv_surface_flux | cross_domain | USF | Uv surface flux | hub | cross_domain::astrobiology::uv_surface_flux | f64 | 1 | 1783160.00 | 1000.00 |
| cross_domain_astrochemistry_dust_equilibrium_temperature | cross_domain | DET | Dust equilibrium temperature | hub | cross_domain::astrochemistry::dust_equilibrium_temperature | f64 | 1 | 1769671.00 | 1000.00 |
| cross_domain_astrochemistry_freefall_time | cross_domain | FT | Freefall time | hub | cross_domain::astrochemistry::freefall_time | f64 | 1 | 1758850.00 | 1000.00 |
| cross_domain_astrochemistry_jeans_mass | cross_domain | JM | Jeans mass | hub | cross_domain::astrochemistry::jeans_mass | f64 | 1 | 2513420.00 | 500.00 |
| cross_domain_astrochemistry_stroemgren_radius | cross_domain | SR | Stroemgren radius | hub | cross_domain::astrochemistry::stroemgren_radius | f64 | 1 | 2502561.00 | 500.00 |
| cross_domain_astrophysics_compton_wavelength | cross_domain | CW | Compton wavelength | hub | cross_domain::astrophysics::compton_wavelength | f64 | 1 | 2242390.00 | 500.00 |
| cross_domain_astrophysics_eddington_luminosity | cross_domain | EL | Eddington luminosity | hub | cross_domain::astrophysics::eddington_luminosity | f64 | 1 | 2078380.00 | 500.00 |
| cross_domain_astrophysics_hawking_temperature | cross_domain | HT | Hawking temperature | hub | cross_domain::astrophysics::hawking_temperature | f64 | 1 | 2040441.00 | 500.00 |
| cross_domain_astrophysics_schwarzschild_radius | cross_domain | SR | Schwarzschild radius | hub | cross_domain::astrophysics::schwarzschild_radius | f64 | 1 | 2242080.00 | 500.00 |
| cross_domain_atmospheric_chemistry_aerosol_optical_depth | cross_domain | AOD | Aerosol optical depth | hub | cross_domain::atmospheric_chemistry::aerosol_optical_depth | f64 | 1 | 2178070.00 | 500.00 |
| cross_domain_atmospheric_chemistry_column_density | cross_domain | CD | Column density | hub | cross_domain::atmospheric_chemistry::column_density | f64 | 1 | 2266931.00 | 500.00 |
| cross_domain_atmospheric_chemistry_henry_law_concentration | cross_domain | HLC | Henry law concentration | hub | cross_domain::atmospheric_chemistry::henry_law_concentration | f64 | 1 | 2206710.00 | 500.00 |
| cross_domain_atmospheric_chemistry_photolysis_rate | cross_domain | PR | Photolysis rate | hub | cross_domain::atmospheric_chemistry::photolysis_rate | f64 | 1 | 1817451.00 | 1000.00 |
| cross_domain_atmospheric_physics_dry_adiabatic_lapse_rate | cross_domain | DAL | Dry adiabatic lapse rate | hub | cross_domain::atmospheric_physics::dry_adiabatic_lapse_rate | f64 | 1 | 1783020.00 | 1000.00 |
| cross_domain_atmospheric_physics_pressure_at_altitude | cross_domain | PAA | Pressure at altitude | hub | cross_domain::atmospheric_physics::pressure_at_altitude | f64 | 1 | 2480350.00 | 500.00 |
| cross_domain_atmospheric_physics_stefan_boltzmann_flux | cross_domain | SBF | Stefan boltzmann flux | hub | cross_domain::atmospheric_physics::stefan_boltzmann_flux | f64 | 1 | 1982791.00 | 1000.00 |
| cross_domain_atmospheric_physics_wien_peak_wavelength | cross_domain | WPW | Wien peak wavelength | hub | cross_domain::atmospheric_physics::wien_peak_wavelength | f64 | 1 | 1787630.00 | 1000.00 |
| cross_domain_biochemistry_enzyme_turnover_number | cross_domain | ETN | Enzyme turnover number | hub | cross_domain::biochemistry::enzyme_turnover_number | f64 | 1 | 1777500.00 | 1000.00 |
| cross_domain_biochemistry_gibbs_free_energy | cross_domain | GFE | Gibbs free energy | hub | cross_domain::biochemistry::gibbs_free_energy | f64 | 1 | 1706670.00 | 1000.00 |
| cross_domain_biochemistry_henderson_hasselbalch | cross_domain | HH | Henderson hasselbalch | hub | cross_domain::biochemistry::henderson_hasselbalch | f64 | 1 | 1861711.00 | 1000.00 |
| cross_domain_biochemistry_michaelis_menten_rate | cross_domain | MMR | Michaelis menten rate | hub | cross_domain::biochemistry::michaelis_menten_rate | f64 | 1 | 1744330.00 | 1000.00 |
| cross_domain_biochemistry_ph_from_concentration | cross_domain | PFC | Ph from concentration | hub | cross_domain::biochemistry::ph_from_concentration | f64 | 1 | 1755960.00 | 1000.00 |
| cross_domain_biomathematics_basic_reproduction_number | cross_domain | BRN | Basic reproduction number | hub | cross_domain::biomathematics::basic_reproduction_number | f64 | 1 | 2054001.00 | 500.00 |
| cross_domain_biomathematics_logistic_growth_rate | cross_domain | LGR | Logistic growth rate | hub | cross_domain::biomathematics::logistic_growth_rate | f64 | 1 | 1972870.00 | 1000.00 |
| cross_domain_biomathematics_molecular_clock_distance | cross_domain | MCD | Molecular clock distance | hub | cross_domain::biomathematics::molecular_clock_distance | f64 | 1 | 2289740.00 | 500.00 |
| cross_domain_biomathematics_shannon_diversity_index | cross_domain | SDI | Shannon diversity index | hub | cross_domain::biomathematics::shannon_diversity_index | f64 | 1 | 1976551.00 | 1000.00 |
| cross_domain_biophysics_electrophoretic_mobility | cross_domain | EM | Electrophoretic mobility | hub | cross_domain::biophysics::electrophoretic_mobility | f64 | 1 | 1760720.00 | 1000.00 |
| cross_domain_biophysics_fick_diffusion_flux | cross_domain | FDF | Fick diffusion flux | hub | cross_domain::biophysics::fick_diffusion_flux | f64 | 1 | 1834770.00 | 1000.00 |
| cross_domain_biophysics_reynolds_number | cross_domain | RN | Reynolds number | hub | cross_domain::biophysics::reynolds_number | f64 | 1 | 1938151.00 | 1000.00 |
| cross_domain_biophysics_stokes_drag_force | cross_domain | SDF | Stokes drag force | hub | cross_domain::biophysics::stokes_drag_force | f64 | 1 | 2088430.00 | 500.00 |
| cross_domain_campaign_astronomy_and_physics | cross_domain | CAA | Campaign astronomy and physics | hub | cross_domain::campaign_astronomy_and_physics | f64 | 1 | 2483761.00 | 500.00 |
| cross_domain_campaign_empty | cross_domain | CE | Campaign empty | hub | cross_domain::campaign_empty | f64 | 1 | 2513650.00 | 500.00 |
| cross_domain_campaign_physics_and_chemistry | cross_domain | CPA | Campaign physics and chemistry | hub | cross_domain::campaign_physics_and_chemistry | f64 | 1 | 2375040.00 | 500.00 |
| cross_domain_campaign_result_scalars | cross_domain | CRS | Campaign result scalars | hub | cross_domain::campaign_result_scalars | f64 | 1 | 2270331.00 | 500.00 |
| cross_domain_cross_domain_biology_and_chemistry | cross_domain | CDB | Cross domain biology and chemistry | hub | cross_domain::cross_domain_biology_and_chemistry | f64 | 1 | 2342460.00 | 500.00 |
| cross_domain_cross_domain_earth_escape_velocity | cross_domain | CDE | Cross domain earth escape velocity | hub | cross_domain::cross_domain_earth_escape_velocity | f64 | 1 | 2357230.00 | 500.00 |
| cross_domain_geochemistry_delta_notation | cross_domain | DN | Delta notation | hub | cross_domain::geochemistry::delta_notation | f64 | 1 | 2962231.00 | 500.00 |
| cross_domain_geochemistry_distribution_coefficient | cross_domain | DC | Distribution coefficient | hub | cross_domain::geochemistry::distribution_coefficient | f64 | 1 | 3236970.00 | 333.33 |
| cross_domain_geochemistry_mixing_two_component | cross_domain | MTC | Mixing two component | hub | cross_domain::geochemistry::mixing_two_component | f64 | 1 | 2343681.00 | 500.00 |
| cross_domain_geochemistry_partition_coefficient | cross_domain | PC | Partition coefficient | hub | cross_domain::geochemistry::partition_coefficient | f64 | 1 | 2129410.00 | 500.00 |
| cross_domain_geophysics_bouguer_anomaly | cross_domain | BA | Bouguer anomaly | hub | cross_domain::geophysics::bouguer_anomaly | f64 | 1 | 2051471.00 | 500.00 |
| cross_domain_geophysics_electromagnetic_skin_depth | cross_domain | ESD | Electromagnetic skin depth | hub | cross_domain::geophysics::electromagnetic_skin_depth | f64 | 1 | 2150880.00 | 500.00 |
| cross_domain_geophysics_seismic_impedance_reflection | cross_domain | SIR | Seismic impedance reflection | hub | cross_domain::geophysics::seismic_impedance_reflection | f64 | 1 | 2201050.00 | 500.00 |
| cross_domain_geophysics_seismic_wave_attenuation | cross_domain | SWA | Seismic wave attenuation | hub | cross_domain::geophysics::seismic_wave_attenuation | f64 | 1 | 2173761.00 | 500.00 |
| cross_domain_invalid_function_returns_error | cross_domain | IFR | Invalid function returns error | hub | cross_domain::invalid_function_returns_error | f64 | 1 | 3150490.00 | 333.33 |
| cross_domain_mathematical_physics_de_broglie_wavelength | cross_domain | DBW | De broglie wavelength | hub | cross_domain::mathematical_physics::de_broglie_wavelength | f64 | 1 | 3115501.00 | 333.33 |
| cross_domain_mathematical_physics_fermi_dirac_distribution | cross_domain | FDD | Fermi dirac distribution | hub | cross_domain::mathematical_physics::fermi_dirac_distribution | f64 | 1 | 3057090.00 | 333.33 |
| cross_domain_mathematical_physics_heisenberg_uncertainty_position | cross_domain | HUP | Heisenberg uncertainty position | hub | cross_domain::mathematical_physics::heisenberg_uncertainty_position | f64 | 1 | 2898791.00 | 500.00 |
| cross_domain_mathematical_physics_relativistic_energy | cross_domain | RE | Relativistic energy | hub | cross_domain::mathematical_physics::relativistic_energy | f64 | 1 | 2930200.00 | 500.00 |
| cross_domain_planetary_geology_crater_counting_surface_age | cross_domain | CCS | Crater counting surface age | hub | cross_domain::planetary_geology::crater_counting_surface_age | f64 | 1 | 2955731.00 | 500.00 |
| cross_domain_planetary_geology_impact_energy | cross_domain | IE | Impact energy | hub | cross_domain::planetary_geology::impact_energy | f64 | 1 | 3158110.00 | 333.33 |
| cross_domain_planetary_geology_surface_temperature_equilibrium | cross_domain | STE | Surface temperature equilibrium | hub | cross_domain::planetary_geology::surface_temperature_equilibrium | f64 | 1 | 2974211.00 | 500.00 |
| cross_domain_planetary_geology_volcanic_effusion_rate | cross_domain | VER | Volcanic effusion rate | hub | cross_domain::planetary_geology::volcanic_effusion_rate | f64 | 1 | 2962660.00 | 500.00 |
| cross_domain_validation_detects_bad_expected_value | cross_domain | VDB | Validation detects bad expected value | hub | cross_domain::validation_detects_bad_expected_value | f64 | 1 | 3336461.00 | 333.33 |
| cross_domain_validation_markdown_output | cross_domain | VMO | Validation markdown output | hub | cross_domain::validation_markdown_output | f64 | 1 | 4061670.00 | 250.00 |
| cross_domain_validation_single_case | cross_domain | VSC | Validation single case | hub | cross_domain::validation_single_case | f64 | 1 | 3049381.00 | 333.33 |
| pipeline_pipeline_chained_stages | pipeline | PCS | Pipeline chained stages | hub | pipeline::pipeline_chained_stages | f64 | 1 | 2420330.00 | 500.00 |
| pipeline_pipeline_empty_input | pipeline | PEI | Pipeline empty input | hub | pipeline::pipeline_empty_input | f64 | 1 | 2013711.00 | 500.00 |
| pipeline_pipeline_normalize_and_filter | pipeline | PNA | Pipeline normalize and filter | hub | pipeline::pipeline_normalize_and_filter | f64 | 1 | 2137040.00 | 500.00 |
| pipeline_pipeline_scale | pipeline | PS | Pipeline scale | hub | pipeline::pipeline_scale | f64 | 1 | 1967560.00 | 1000.00 |
| tools_approx_equal_works | tools | AEW | Approx equal works | hub | tools::approx_equal_works | f64 | 1 | 1794651.00 | 1000.00 |
| tools_arena_alloc_and_read | tools | AAA | Arena alloc and read | hub | tools::arena_alloc_and_read | f64 | 1 | 2863560.00 | 500.00 |
| tools_arena_capacity_limit | tools | ACL | Arena capacity limit | hub | tools::arena_capacity_limit | f64 | 1 | 2900800.00 | 500.00 |
| tools_arena_matrix | tools | AM | Arena matrix | hub | tools::arena_matrix | f64 | 1 | 2706001.00 | 500.00 |
| tools_arena_reset | tools | AR | Arena reset | hub | tools::arena_reset | f64 | 1 | 2882440.00 | 500.00 |
| tools_fingerprint_changes_with_data | tools | FCW | Fingerprint changes with data | hub | tools::fingerprint_changes_with_data | f64 | 1 | 2274181.00 | 500.00 |
| tools_fingerprint_deterministic | tools | FD | Fingerprint deterministic | hub | tools::fingerprint_deterministic | f64 | 1 | 2165780.00 | 500.00 |
| tools_format_scientific_output | tools | FSO | Format scientific output | hub | tools::format_scientific_output | f64 | 1 | 2170511.00 | 500.00 |
| tools_kahan_dot_basic | tools | KDB | Kahan dot basic | hub | tools::kahan_dot_basic | f64 | 1 | 1881450.00 | 1000.00 |
| tools_kahan_sum_precision | tools | KSP | Kahan sum precision | hub | tools::kahan_sum_precision | f64 | 1 | 4831811.00 | 250.00 |
| tools_linspace_basic | tools | LB | Linspace basic | hub | tools::linspace_basic | f64 | 1 | 2067170.00 | 500.00 |
| tools_logspace_basic | tools | LB | Logspace basic | hub | tools::logspace_basic | f64 | 1 | 2098010.00 | 500.00 |
| tools_reproducible_context_fork | tools | RCF | Reproducible context fork | hub | tools::reproducible_context_fork | f64 | 1 | 1843431.00 | 1000.00 |
| tools_rng_deterministic | tools | RD | Rng deterministic | hub | tools::rng_deterministic | f64 | 1 | 1822530.00 | 1000.00 |
| tools_rng_different_seeds_differ | tools | RDS | Rng different seeds differ | hub | tools::rng_different_seeds_differ | f64 | 1 | 1891730.00 | 1000.00 |
| tools_rng_normal_distribution | tools | RND | Rng normal distribution | hub | tools::rng_normal_distribution | f64 | 1 | 2766501.00 | 500.00 |
| tools_rng_uniform_range | tools | RUR | Rng uniform range | hub | tools::rng_uniform_range | f64 | 1 | 1912770.00 | 1000.00 |
| tools_scratch_pool_independent | tools | SPI | Scratch pool independent | hub | tools::scratch_pool_independent | f64 | 1 | 1877680.00 | 1000.00 |
## Analytics
### Category Distribution
| Category | Count | Avg (ns) | Min (ns) | Max (ns) |
|---|---|---|---|---|
| cross_domain | 60 | 2316654.72 | 1706670.00 | 4061670.00 |
| pipeline | 4 | 2134660.25 | 1967560.00 | 2420330.00 |
| tools | 18 | 2375055.94 | 1794651.00 | 4831811.00 |

### Top 10 Fastest Entries
| # | Entry | Avg (ns) |
|---|---|---|
| 1 | cross_domain_biochemistry_gibbs_free_energy | 1706670.00 |
| 2 | cross_domain_astrobiology_surface_gravity | 1727810.00 |
| 3 | cross_domain_biochemistry_michaelis_menten_rate | 1744330.00 |
| 4 | cross_domain_biochemistry_ph_from_concentration | 1755960.00 |
| 5 | cross_domain_astrochemistry_freefall_time | 1758850.00 |
| 6 | cross_domain_biophysics_electrophoretic_mobility | 1760720.00 |
| 7 | cross_domain_astrochemistry_dust_equilibrium_temperature | 1769671.00 |
| 8 | cross_domain_biochemistry_enzyme_turnover_number | 1777500.00 |
| 9 | cross_domain_atmospheric_physics_dry_adiabatic_lapse_rate | 1783020.00 |
| 10 | cross_domain_astrobiology_uv_surface_flux | 1783160.00 |

### Top 10 Slowest Entries
| # | Entry | Avg (ns) |
|---|---|---|
| 1 | tools_kahan_sum_precision | 4831811.00 |
| 2 | cross_domain_validation_markdown_output | 4061670.00 |
| 3 | cross_domain_validation_detects_bad_expected_value | 3336461.00 |
| 4 | cross_domain_geochemistry_distribution_coefficient | 3236970.00 |
| 5 | cross_domain_planetary_geology_impact_energy | 3158110.00 |
| 6 | cross_domain_invalid_function_returns_error | 3150490.00 |
| 7 | cross_domain_mathematical_physics_de_broglie_wavelength | 3115501.00 |
| 8 | cross_domain_mathematical_physics_fermi_dirac_distribution | 3057090.00 |
| 9 | cross_domain_validation_single_case | 3049381.00 |
| 10 | cross_domain_planetary_geology_surface_temperature_equilibrium | 2974211.00 |

