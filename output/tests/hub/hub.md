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
| cross_domain_astrobiology_drake_equation | cross_domain | DE | Drake equation | hub | cross_domain::astrobiology::drake_equation | f64 | 1 | 2333624.00 | 500.00 |
| cross_domain_astrobiology_habitable_zone_inner | cross_domain | HZI | Habitable zone inner | hub | cross_domain::astrobiology::habitable_zone_inner | f64 | 1 | 1834474.00 | 1000.00 |
| cross_domain_astrobiology_habitable_zone_outer | cross_domain | HZO | Habitable zone outer | hub | cross_domain::astrobiology::habitable_zone_outer | f64 | 1 | 1909333.00 | 1000.00 |
| cross_domain_astrobiology_surface_gravity | cross_domain | SG | Surface gravity | hub | cross_domain::astrobiology::surface_gravity | f64 | 1 | 2265814.00 | 500.00 |
| cross_domain_astrobiology_uv_surface_flux | cross_domain | USF | Uv surface flux | hub | cross_domain::astrobiology::uv_surface_flux | f64 | 1 | 2200084.00 | 500.00 |
| cross_domain_astrochemistry_dust_equilibrium_temperature | cross_domain | DET | Dust equilibrium temperature | hub | cross_domain::astrochemistry::dust_equilibrium_temperature | f64 | 1 | 2057504.00 | 500.00 |
| cross_domain_astrochemistry_freefall_time | cross_domain | FT | Freefall time | hub | cross_domain::astrochemistry::freefall_time | f64 | 1 | 2366654.00 | 500.00 |
| cross_domain_astrochemistry_jeans_mass | cross_domain | JM | Jeans mass | hub | cross_domain::astrochemistry::jeans_mass | f64 | 1 | 2245815.00 | 500.00 |
| cross_domain_astrochemistry_stroemgren_radius | cross_domain | SR | Stroemgren radius | hub | cross_domain::astrochemistry::stroemgren_radius | f64 | 1 | 2031683.00 | 500.00 |
| cross_domain_astrophysics_compton_wavelength | cross_domain | CW | Compton wavelength | hub | cross_domain::astrophysics::compton_wavelength | f64 | 1 | 1916544.00 | 1000.00 |
| cross_domain_astrophysics_eddington_luminosity | cross_domain | EL | Eddington luminosity | hub | cross_domain::astrophysics::eddington_luminosity | f64 | 1 | 1821073.00 | 1000.00 |
| cross_domain_astrophysics_hawking_temperature | cross_domain | HT | Hawking temperature | hub | cross_domain::astrophysics::hawking_temperature | f64 | 1 | 1727613.00 | 1000.00 |
| cross_domain_astrophysics_schwarzschild_radius | cross_domain | SR | Schwarzschild radius | hub | cross_domain::astrophysics::schwarzschild_radius | f64 | 1 | 2036084.00 | 500.00 |
| cross_domain_atmospheric_chemistry_aerosol_optical_depth | cross_domain | AOD | Aerosol optical depth | hub | cross_domain::atmospheric_chemistry::aerosol_optical_depth | f64 | 1 | 2205294.00 | 500.00 |
| cross_domain_atmospheric_chemistry_column_density | cross_domain | CD | Column density | hub | cross_domain::atmospheric_chemistry::column_density | f64 | 1 | 2650545.00 | 500.00 |
| cross_domain_atmospheric_chemistry_henry_law_concentration | cross_domain | HLC | Henry law concentration | hub | cross_domain::atmospheric_chemistry::henry_law_concentration | f64 | 1 | 2042054.00 | 500.00 |
| cross_domain_atmospheric_chemistry_photolysis_rate | cross_domain | PR | Photolysis rate | hub | cross_domain::atmospheric_chemistry::photolysis_rate | f64 | 1 | 2244704.00 | 500.00 |
| cross_domain_atmospheric_physics_dry_adiabatic_lapse_rate | cross_domain | DAL | Dry adiabatic lapse rate | hub | cross_domain::atmospheric_physics::dry_adiabatic_lapse_rate | f64 | 1 | 1736443.00 | 1000.00 |
| cross_domain_atmospheric_physics_pressure_at_altitude | cross_domain | PAA | Pressure at altitude | hub | cross_domain::atmospheric_physics::pressure_at_altitude | f64 | 1 | 2231914.00 | 500.00 |
| cross_domain_atmospheric_physics_stefan_boltzmann_flux | cross_domain | SBF | Stefan boltzmann flux | hub | cross_domain::atmospheric_physics::stefan_boltzmann_flux | f64 | 1 | 1830364.00 | 1000.00 |
| cross_domain_atmospheric_physics_wien_peak_wavelength | cross_domain | WPW | Wien peak wavelength | hub | cross_domain::atmospheric_physics::wien_peak_wavelength | f64 | 1 | 2056174.00 | 500.00 |
| cross_domain_biochemistry_enzyme_turnover_number | cross_domain | ETN | Enzyme turnover number | hub | cross_domain::biochemistry::enzyme_turnover_number | f64 | 1 | 1962973.00 | 1000.00 |
| cross_domain_biochemistry_gibbs_free_energy | cross_domain | GFE | Gibbs free energy | hub | cross_domain::biochemistry::gibbs_free_energy | f64 | 1 | 2466285.00 | 500.00 |
| cross_domain_biochemistry_henderson_hasselbalch | cross_domain | HH | Henderson hasselbalch | hub | cross_domain::biochemistry::henderson_hasselbalch | f64 | 1 | 2129484.00 | 500.00 |
| cross_domain_biochemistry_michaelis_menten_rate | cross_domain | MMR | Michaelis menten rate | hub | cross_domain::biochemistry::michaelis_menten_rate | f64 | 1 | 2032394.00 | 500.00 |
| cross_domain_biochemistry_ph_from_concentration | cross_domain | PFC | Ph from concentration | hub | cross_domain::biochemistry::ph_from_concentration | f64 | 1 | 1834353.00 | 1000.00 |
| cross_domain_biomathematics_basic_reproduction_number | cross_domain | BRN | Basic reproduction number | hub | cross_domain::biomathematics::basic_reproduction_number | f64 | 1 | 1978154.00 | 1000.00 |
| cross_domain_biomathematics_logistic_growth_rate | cross_domain | LGR | Logistic growth rate | hub | cross_domain::biomathematics::logistic_growth_rate | f64 | 1 | 1829353.00 | 1000.00 |
| cross_domain_biomathematics_molecular_clock_distance | cross_domain | MCD | Molecular clock distance | hub | cross_domain::biomathematics::molecular_clock_distance | f64 | 1 | 1683843.00 | 1000.00 |
| cross_domain_biomathematics_shannon_diversity_index | cross_domain | SDI | Shannon diversity index | hub | cross_domain::biomathematics::shannon_diversity_index | f64 | 1 | 2150154.00 | 500.00 |
| cross_domain_biophysics_electrophoretic_mobility | cross_domain | EM | Electrophoretic mobility | hub | cross_domain::biophysics::electrophoretic_mobility | f64 | 1 | 2342154.00 | 500.00 |
| cross_domain_biophysics_fick_diffusion_flux | cross_domain | FDF | Fick diffusion flux | hub | cross_domain::biophysics::fick_diffusion_flux | f64 | 1 | 2307485.00 | 500.00 |
| cross_domain_biophysics_reynolds_number | cross_domain | RN | Reynolds number | hub | cross_domain::biophysics::reynolds_number | f64 | 1 | 2161554.00 | 500.00 |
| cross_domain_biophysics_stokes_drag_force | cross_domain | SDF | Stokes drag force | hub | cross_domain::biophysics::stokes_drag_force | f64 | 1 | 2079954.00 | 500.00 |
| cross_domain_campaign_astronomy_and_physics | cross_domain | CAA | Campaign astronomy and physics | hub | cross_domain::campaign_astronomy_and_physics | f64 | 1 | 2543144.00 | 500.00 |
| cross_domain_campaign_empty | cross_domain | CE | Campaign empty | hub | cross_domain::campaign_empty | f64 | 1 | 2466395.00 | 500.00 |
| cross_domain_campaign_physics_and_chemistry | cross_domain | CPA | Campaign physics and chemistry | hub | cross_domain::campaign_physics_and_chemistry | f64 | 1 | 2517505.00 | 500.00 |
| cross_domain_campaign_result_scalars | cross_domain | CRS | Campaign result scalars | hub | cross_domain::campaign_result_scalars | f64 | 1 | 3242896.00 | 333.33 |
| cross_domain_cross_domain_biology_and_chemistry | cross_domain | CDB | Cross domain biology and chemistry | hub | cross_domain::cross_domain_biology_and_chemistry | f64 | 1 | 3230385.00 | 333.33 |
| cross_domain_cross_domain_earth_escape_velocity | cross_domain | CDE | Cross domain earth escape velocity | hub | cross_domain::cross_domain_earth_escape_velocity | f64 | 1 | 3378037.00 | 333.33 |
| cross_domain_geochemistry_delta_notation | cross_domain | DN | Delta notation | hub | cross_domain::geochemistry::delta_notation | f64 | 1 | 2294884.00 | 500.00 |
| cross_domain_geochemistry_distribution_coefficient | cross_domain | DC | Distribution coefficient | hub | cross_domain::geochemistry::distribution_coefficient | f64 | 1 | 1970124.00 | 1000.00 |
| cross_domain_geochemistry_mixing_two_component | cross_domain | MTC | Mixing two component | hub | cross_domain::geochemistry::mixing_two_component | f64 | 1 | 2055333.00 | 500.00 |
| cross_domain_geochemistry_partition_coefficient | cross_domain | PC | Partition coefficient | hub | cross_domain::geochemistry::partition_coefficient | f64 | 1 | 2048584.00 | 500.00 |
| cross_domain_geophysics_bouguer_anomaly | cross_domain | BA | Bouguer anomaly | hub | cross_domain::geophysics::bouguer_anomaly | f64 | 1 | 2388975.00 | 500.00 |
| cross_domain_geophysics_electromagnetic_skin_depth | cross_domain | ESD | Electromagnetic skin depth | hub | cross_domain::geophysics::electromagnetic_skin_depth | f64 | 1 | 1933403.00 | 1000.00 |
| cross_domain_geophysics_seismic_impedance_reflection | cross_domain | SIR | Seismic impedance reflection | hub | cross_domain::geophysics::seismic_impedance_reflection | f64 | 1 | 1967704.00 | 1000.00 |
| cross_domain_geophysics_seismic_wave_attenuation | cross_domain | SWA | Seismic wave attenuation | hub | cross_domain::geophysics::seismic_wave_attenuation | f64 | 1 | 2967995.00 | 500.00 |
| cross_domain_invalid_function_returns_error | cross_domain | IFR | Invalid function returns error | hub | cross_domain::invalid_function_returns_error | f64 | 1 | 2389395.00 | 500.00 |
| cross_domain_mathematical_physics_de_broglie_wavelength | cross_domain | DBW | De broglie wavelength | hub | cross_domain::mathematical_physics::de_broglie_wavelength | f64 | 1 | 2354334.00 | 500.00 |
| cross_domain_mathematical_physics_fermi_dirac_distribution | cross_domain | FDD | Fermi dirac distribution | hub | cross_domain::mathematical_physics::fermi_dirac_distribution | f64 | 1 | 3622117.00 | 333.33 |
| cross_domain_mathematical_physics_heisenberg_uncertainty_position | cross_domain | HUP | Heisenberg uncertainty position | hub | cross_domain::mathematical_physics::heisenberg_uncertainty_position | f64 | 1 | 3096396.00 | 333.33 |
| cross_domain_mathematical_physics_relativistic_energy | cross_domain | RE | Relativistic energy | hub | cross_domain::mathematical_physics::relativistic_energy | f64 | 1 | 3178336.00 | 333.33 |
| cross_domain_planetary_geology_crater_counting_surface_age | cross_domain | CCS | Crater counting surface age | hub | cross_domain::planetary_geology::crater_counting_surface_age | f64 | 1 | 3435196.00 | 333.33 |
| cross_domain_planetary_geology_impact_energy | cross_domain | IE | Impact energy | hub | cross_domain::planetary_geology::impact_energy | f64 | 1 | 5805491.00 | 200.00 |
| cross_domain_planetary_geology_surface_temperature_equilibrium | cross_domain | STE | Surface temperature equilibrium | hub | cross_domain::planetary_geology::surface_temperature_equilibrium | f64 | 1 | 3597376.00 | 333.33 |
| cross_domain_planetary_geology_volcanic_effusion_rate | cross_domain | VER | Volcanic effusion rate | hub | cross_domain::planetary_geology::volcanic_effusion_rate | f64 | 1 | 4343318.00 | 250.00 |
| cross_domain_validation_detects_bad_expected_value | cross_domain | VDB | Validation detects bad expected value | hub | cross_domain::validation_detects_bad_expected_value | f64 | 1 | 3341807.00 | 333.33 |
| cross_domain_validation_markdown_output | cross_domain | VMO | Validation markdown output | hub | cross_domain::validation_markdown_output | f64 | 1 | 3274486.00 | 333.33 |
| cross_domain_validation_single_case | cross_domain | VSC | Validation single case | hub | cross_domain::validation_single_case | f64 | 1 | 3070955.00 | 333.33 |
| pipeline_pipeline_chained_stages | pipeline | PCS | Pipeline chained stages | hub | pipeline::pipeline_chained_stages | f64 | 1 | 3044316.00 | 333.33 |
| pipeline_pipeline_empty_input | pipeline | PEI | Pipeline empty input | hub | pipeline::pipeline_empty_input | f64 | 1 | 2459204.00 | 500.00 |
| pipeline_pipeline_normalize_and_filter | pipeline | PNA | Pipeline normalize and filter | hub | pipeline::pipeline_normalize_and_filter | f64 | 1 | 3397997.00 | 333.33 |
| pipeline_pipeline_scale | pipeline | PS | Pipeline scale | hub | pipeline::pipeline_scale | f64 | 1 | 2476194.00 | 500.00 |
| tools_approx_equal_works | tools | AEW | Approx equal works | hub | tools::approx_equal_works | f64 | 1 | 2784225.00 | 500.00 |
| tools_arena_alloc_and_read | tools | AAA | Arena alloc and read | hub | tools::arena_alloc_and_read | f64 | 1 | 3432797.00 | 333.33 |
| tools_arena_capacity_limit | tools | ACL | Arena capacity limit | hub | tools::arena_capacity_limit | f64 | 1 | 2898835.00 | 500.00 |
| tools_arena_matrix | tools | AM | Arena matrix | hub | tools::arena_matrix | f64 | 1 | 2957966.00 | 500.00 |
| tools_arena_reset | tools | AR | Arena reset | hub | tools::arena_reset | f64 | 1 | 4232458.00 | 250.00 |
| tools_fingerprint_changes_with_data | tools | FCW | Fingerprint changes with data | hub | tools::fingerprint_changes_with_data | f64 | 1 | 2941615.00 | 500.00 |
| tools_fingerprint_deterministic | tools | FD | Fingerprint deterministic | hub | tools::fingerprint_deterministic | f64 | 1 | 2808475.00 | 500.00 |
| tools_format_scientific_output | tools | FSO | Format scientific output | hub | tools::format_scientific_output | f64 | 1 | 3228966.00 | 333.33 |
| tools_kahan_dot_basic | tools | KDB | Kahan dot basic | hub | tools::kahan_dot_basic | f64 | 1 | 2482265.00 | 500.00 |
| tools_kahan_sum_precision | tools | KSP | Kahan sum precision | hub | tools::kahan_sum_precision | f64 | 1 | 4952179.00 | 250.00 |
| tools_linspace_basic | tools | LB | Linspace basic | hub | tools::linspace_basic | f64 | 1 | 1969284.00 | 1000.00 |
| tools_logspace_basic | tools | LB | Logspace basic | hub | tools::logspace_basic | f64 | 1 | 1916273.00 | 1000.00 |
| tools_reproducible_context_fork | tools | RCF | Reproducible context fork | hub | tools::reproducible_context_fork | f64 | 1 | 2663155.00 | 500.00 |
| tools_rng_deterministic | tools | RD | Rng deterministic | hub | tools::rng_deterministic | f64 | 1 | 2283154.00 | 500.00 |
| tools_rng_different_seeds_differ | tools | RDS | Rng different seeds differ | hub | tools::rng_different_seeds_differ | f64 | 1 | 1837204.00 | 1000.00 |
| tools_rng_normal_distribution | tools | RND | Rng normal distribution | hub | tools::rng_normal_distribution | f64 | 1 | 2580564.00 | 500.00 |
| tools_rng_uniform_range | tools | RUR | Rng uniform range | hub | tools::rng_uniform_range | f64 | 1 | 1816684.00 | 1000.00 |
| tools_scratch_pool_independent | tools | SPI | Scratch pool independent | hub | tools::scratch_pool_independent | f64 | 1 | 2156334.00 | 500.00 |
## Analytics
### Category Distribution
| Category | Count | Avg (ns) | Min (ns) | Max (ns) |
|---|---|---|---|---|
| cross_domain | 60 | 2453608.37 | 1683843.00 | 5805491.00 |
| pipeline | 4 | 2844427.75 | 2459204.00 | 3397997.00 |
| tools | 18 | 2774579.61 | 1816684.00 | 4952179.00 |

### Top 10 Fastest Entries
| # | Entry | Avg (ns) |
|---|---|---|
| 1 | cross_domain_biomathematics_molecular_clock_distance | 1683843.00 |
| 2 | cross_domain_astrophysics_hawking_temperature | 1727613.00 |
| 3 | cross_domain_atmospheric_physics_dry_adiabatic_lapse_rate | 1736443.00 |
| 4 | tools_rng_uniform_range | 1816684.00 |
| 5 | cross_domain_astrophysics_eddington_luminosity | 1821073.00 |
| 6 | cross_domain_biomathematics_logistic_growth_rate | 1829353.00 |
| 7 | cross_domain_atmospheric_physics_stefan_boltzmann_flux | 1830364.00 |
| 8 | cross_domain_biochemistry_ph_from_concentration | 1834353.00 |
| 9 | cross_domain_astrobiology_habitable_zone_inner | 1834474.00 |
| 10 | tools_rng_different_seeds_differ | 1837204.00 |

### Top 10 Slowest Entries
| # | Entry | Avg (ns) |
|---|---|---|
| 1 | cross_domain_planetary_geology_impact_energy | 5805491.00 |
| 2 | tools_kahan_sum_precision | 4952179.00 |
| 3 | cross_domain_planetary_geology_volcanic_effusion_rate | 4343318.00 |
| 4 | tools_arena_reset | 4232458.00 |
| 5 | cross_domain_mathematical_physics_fermi_dirac_distribution | 3622117.00 |
| 6 | cross_domain_planetary_geology_surface_temperature_equilibrium | 3597376.00 |
| 7 | cross_domain_planetary_geology_crater_counting_surface_age | 3435196.00 |
| 8 | tools_arena_alloc_and_read | 3432797.00 |
| 9 | pipeline_pipeline_normalize_and_filter | 3397997.00 |
| 10 | cross_domain_cross_domain_earth_escape_velocity | 3378037.00 |

