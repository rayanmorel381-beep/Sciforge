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
| cross_domain_astrobiology_drake_equation | cross_domain | DE | Drake equation | hub | cross_domain::astrobiology::drake_equation | f64 | 1 | 2206008.00 | 500.00 |
| cross_domain_astrobiology_habitable_zone_inner | cross_domain | HZI | Habitable zone inner | hub | cross_domain::astrobiology::habitable_zone_inner | f64 | 1 | 1884250.00 | 1000.00 |
| cross_domain_astrobiology_habitable_zone_outer | cross_domain | HZO | Habitable zone outer | hub | cross_domain::astrobiology::habitable_zone_outer | f64 | 1 | 1817039.00 | 1000.00 |
| cross_domain_astrobiology_surface_gravity | cross_domain | SG | Surface gravity | hub | cross_domain::astrobiology::surface_gravity | f64 | 1 | 1794769.00 | 1000.00 |
| cross_domain_astrobiology_uv_surface_flux | cross_domain | USF | Uv surface flux | hub | cross_domain::astrobiology::uv_surface_flux | f64 | 1 | 1759109.00 | 1000.00 |
| cross_domain_astrochemistry_dust_equilibrium_temperature | cross_domain | DET | Dust equilibrium temperature | hub | cross_domain::astrochemistry::dust_equilibrium_temperature | f64 | 1 | 1678590.00 | 1000.00 |
| cross_domain_astrochemistry_freefall_time | cross_domain | FT | Freefall time | hub | cross_domain::astrochemistry::freefall_time | f64 | 1 | 1648329.00 | 1000.00 |
| cross_domain_astrochemistry_jeans_mass | cross_domain | JM | Jeans mass | hub | cross_domain::astrochemistry::jeans_mass | f64 | 1 | 1676549.00 | 1000.00 |
| cross_domain_astrochemistry_stroemgren_radius | cross_domain | SR | Stroemgren radius | hub | cross_domain::astrochemistry::stroemgren_radius | f64 | 1 | 1644349.00 | 1000.00 |
| cross_domain_astrophysics_compton_wavelength | cross_domain | CW | Compton wavelength | hub | cross_domain::astrophysics::compton_wavelength | f64 | 1 | 1714900.00 | 1000.00 |
| cross_domain_astrophysics_eddington_luminosity | cross_domain | EL | Eddington luminosity | hub | cross_domain::astrophysics::eddington_luminosity | f64 | 1 | 1675299.00 | 1000.00 |
| cross_domain_astrophysics_hawking_temperature | cross_domain | HT | Hawking temperature | hub | cross_domain::astrophysics::hawking_temperature | f64 | 1 | 1659639.00 | 1000.00 |
| cross_domain_astrophysics_schwarzschild_radius | cross_domain | SR | Schwarzschild radius | hub | cross_domain::astrophysics::schwarzschild_radius | f64 | 1 | 1674220.00 | 1000.00 |
| cross_domain_atmospheric_chemistry_aerosol_optical_depth | cross_domain | AOD | Aerosol optical depth | hub | cross_domain::atmospheric_chemistry::aerosol_optical_depth | f64 | 1 | 1802549.00 | 1000.00 |
| cross_domain_atmospheric_chemistry_column_density | cross_domain | CD | Column density | hub | cross_domain::atmospheric_chemistry::column_density | f64 | 1 | 1727619.00 | 1000.00 |
| cross_domain_atmospheric_chemistry_henry_law_concentration | cross_domain | HLC | Henry law concentration | hub | cross_domain::atmospheric_chemistry::henry_law_concentration | f64 | 1 | 1829959.00 | 1000.00 |
| cross_domain_atmospheric_chemistry_photolysis_rate | cross_domain | PR | Photolysis rate | hub | cross_domain::atmospheric_chemistry::photolysis_rate | f64 | 1 | 1800089.00 | 1000.00 |
| cross_domain_atmospheric_physics_dry_adiabatic_lapse_rate | cross_domain | DAL | Dry adiabatic lapse rate | hub | cross_domain::atmospheric_physics::dry_adiabatic_lapse_rate | f64 | 1 | 1647640.00 | 1000.00 |
| cross_domain_atmospheric_physics_pressure_at_altitude | cross_domain | PAA | Pressure at altitude | hub | cross_domain::atmospheric_physics::pressure_at_altitude | f64 | 1 | 1644369.00 | 1000.00 |
| cross_domain_atmospheric_physics_stefan_boltzmann_flux | cross_domain | SBF | Stefan boltzmann flux | hub | cross_domain::atmospheric_physics::stefan_boltzmann_flux | f64 | 1 | 1647989.00 | 1000.00 |
| cross_domain_atmospheric_physics_wien_peak_wavelength | cross_domain | WPW | Wien peak wavelength | hub | cross_domain::atmospheric_physics::wien_peak_wavelength | f64 | 1 | 1808119.00 | 1000.00 |
| cross_domain_biochemistry_enzyme_turnover_number | cross_domain | ETN | Enzyme turnover number | hub | cross_domain::biochemistry::enzyme_turnover_number | f64 | 1 | 1842520.00 | 1000.00 |
| cross_domain_biochemistry_gibbs_free_energy | cross_domain | GFE | Gibbs free energy | hub | cross_domain::biochemistry::gibbs_free_energy | f64 | 1 | 2899588.00 | 500.00 |
| cross_domain_biochemistry_henderson_hasselbalch | cross_domain | HH | Henderson hasselbalch | hub | cross_domain::biochemistry::henderson_hasselbalch | f64 | 1 | 2427579.00 | 500.00 |
| cross_domain_biochemistry_michaelis_menten_rate | cross_domain | MMR | Michaelis menten rate | hub | cross_domain::biochemistry::michaelis_menten_rate | f64 | 1 | 2194549.00 | 500.00 |
| cross_domain_biochemistry_ph_from_concentration | cross_domain | PFC | Ph from concentration | hub | cross_domain::biochemistry::ph_from_concentration | f64 | 1 | 2377109.00 | 500.00 |
| cross_domain_biomathematics_basic_reproduction_number | cross_domain | BRN | Basic reproduction number | hub | cross_domain::biomathematics::basic_reproduction_number | f64 | 1 | 2343159.00 | 500.00 |
| cross_domain_biomathematics_logistic_growth_rate | cross_domain | LGR | Logistic growth rate | hub | cross_domain::biomathematics::logistic_growth_rate | f64 | 1 | 2084899.00 | 500.00 |
| cross_domain_biomathematics_molecular_clock_distance | cross_domain | MCD | Molecular clock distance | hub | cross_domain::biomathematics::molecular_clock_distance | f64 | 1 | 1843679.00 | 1000.00 |
| cross_domain_biomathematics_shannon_diversity_index | cross_domain | SDI | Shannon diversity index | hub | cross_domain::biomathematics::shannon_diversity_index | f64 | 1 | 1906340.00 | 1000.00 |
| cross_domain_biophysics_electrophoretic_mobility | cross_domain | EM | Electrophoretic mobility | hub | cross_domain::biophysics::electrophoretic_mobility | f64 | 1 | 1652439.00 | 1000.00 |
| cross_domain_biophysics_fick_diffusion_flux | cross_domain | FDF | Fick diffusion flux | hub | cross_domain::biophysics::fick_diffusion_flux | f64 | 1 | 1641399.00 | 1000.00 |
| cross_domain_biophysics_reynolds_number | cross_domain | RN | Reynolds number | hub | cross_domain::biophysics::reynolds_number | f64 | 1 | 1641519.00 | 1000.00 |
| cross_domain_biophysics_stokes_drag_force | cross_domain | SDF | Stokes drag force | hub | cross_domain::biophysics::stokes_drag_force | f64 | 1 | 1713970.00 | 1000.00 |
| cross_domain_campaign_astronomy_and_physics | cross_domain | CAA | Campaign astronomy and physics | hub | cross_domain::campaign_astronomy_and_physics | f64 | 1 | 1799749.00 | 1000.00 |
| cross_domain_campaign_empty | cross_domain | CE | Campaign empty | hub | cross_domain::campaign_empty | f64 | 1 | 1667329.00 | 1000.00 |
| cross_domain_campaign_physics_and_chemistry | cross_domain | CPA | Campaign physics and chemistry | hub | cross_domain::campaign_physics_and_chemistry | f64 | 1 | 1794899.00 | 1000.00 |
| cross_domain_campaign_result_scalars | cross_domain | CRS | Campaign result scalars | hub | cross_domain::campaign_result_scalars | f64 | 1 | 1794080.00 | 1000.00 |
| cross_domain_cross_domain_biology_and_chemistry | cross_domain | CDB | Cross domain biology and chemistry | hub | cross_domain::cross_domain_biology_and_chemistry | f64 | 1 | 1794179.00 | 1000.00 |
| cross_domain_cross_domain_earth_escape_velocity | cross_domain | CDE | Cross domain earth escape velocity | hub | cross_domain::cross_domain_earth_escape_velocity | f64 | 1 | 1799949.00 | 1000.00 |
| cross_domain_geochemistry_delta_notation | cross_domain | DN | Delta notation | hub | cross_domain::geochemistry::delta_notation | f64 | 1 | 1811949.00 | 1000.00 |
| cross_domain_geochemistry_distribution_coefficient | cross_domain | DC | Distribution coefficient | hub | cross_domain::geochemistry::distribution_coefficient | f64 | 1 | 1830239.00 | 1000.00 |
| cross_domain_geochemistry_mixing_two_component | cross_domain | MTC | Mixing two component | hub | cross_domain::geochemistry::mixing_two_component | f64 | 1 | 1656700.00 | 1000.00 |
| cross_domain_geochemistry_partition_coefficient | cross_domain | PC | Partition coefficient | hub | cross_domain::geochemistry::partition_coefficient | f64 | 1 | 1704049.00 | 1000.00 |
| cross_domain_geophysics_bouguer_anomaly | cross_domain | BA | Bouguer anomaly | hub | cross_domain::geophysics::bouguer_anomaly | f64 | 1 | 1657019.00 | 1000.00 |
| cross_domain_geophysics_electromagnetic_skin_depth | cross_domain | ESD | Electromagnetic skin depth | hub | cross_domain::geophysics::electromagnetic_skin_depth | f64 | 1 | 1741239.00 | 1000.00 |
| cross_domain_geophysics_seismic_impedance_reflection | cross_domain | SIR | Seismic impedance reflection | hub | cross_domain::geophysics::seismic_impedance_reflection | f64 | 1 | 1794130.00 | 1000.00 |
| cross_domain_geophysics_seismic_wave_attenuation | cross_domain | SWA | Seismic wave attenuation | hub | cross_domain::geophysics::seismic_wave_attenuation | f64 | 1 | 1764319.00 | 1000.00 |
| cross_domain_invalid_function_returns_error | cross_domain | IFR | Invalid function returns error | hub | cross_domain::invalid_function_returns_error | f64 | 1 | 1833349.00 | 1000.00 |
| cross_domain_mathematical_physics_de_broglie_wavelength | cross_domain | DBW | De broglie wavelength | hub | cross_domain::mathematical_physics::de_broglie_wavelength | f64 | 1 | 1773679.00 | 1000.00 |
| cross_domain_mathematical_physics_fermi_dirac_distribution | cross_domain | FDD | Fermi dirac distribution | hub | cross_domain::mathematical_physics::fermi_dirac_distribution | f64 | 1 | 1696440.00 | 1000.00 |
| cross_domain_mathematical_physics_heisenberg_uncertainty_position | cross_domain | HUP | Heisenberg uncertainty position | hub | cross_domain::mathematical_physics::heisenberg_uncertainty_position | f64 | 1 | 1690269.00 | 1000.00 |
| cross_domain_mathematical_physics_relativistic_energy | cross_domain | RE | Relativistic energy | hub | cross_domain::mathematical_physics::relativistic_energy | f64 | 1 | 2060959.00 | 500.00 |
| cross_domain_planetary_geology_crater_counting_surface_age | cross_domain | CCS | Crater counting surface age | hub | cross_domain::planetary_geology::crater_counting_surface_age | f64 | 1 | 3634938.00 | 333.33 |
| cross_domain_planetary_geology_impact_energy | cross_domain | IE | Impact energy | hub | cross_domain::planetary_geology::impact_energy | f64 | 1 | 2266969.00 | 500.00 |
| cross_domain_planetary_geology_surface_temperature_equilibrium | cross_domain | STE | Surface temperature equilibrium | hub | cross_domain::planetary_geology::surface_temperature_equilibrium | f64 | 1 | 2158419.00 | 500.00 |
| cross_domain_planetary_geology_volcanic_effusion_rate | cross_domain | VER | Volcanic effusion rate | hub | cross_domain::planetary_geology::volcanic_effusion_rate | f64 | 1 | 2355909.00 | 500.00 |
| cross_domain_validation_detects_bad_expected_value | cross_domain | VDB | Validation detects bad expected value | hub | cross_domain::validation_detects_bad_expected_value | f64 | 1 | 2362449.00 | 500.00 |
| cross_domain_validation_markdown_output | cross_domain | VMO | Validation markdown output | hub | cross_domain::validation_markdown_output | f64 | 1 | 2283819.00 | 500.00 |
| cross_domain_validation_single_case | cross_domain | VSC | Validation single case | hub | cross_domain::validation_single_case | f64 | 1 | 2575979.00 | 500.00 |
| pipeline_pipeline_chained_stages | pipeline | PCS | Pipeline chained stages | hub | pipeline::pipeline_chained_stages | f64 | 1 | 2302369.00 | 500.00 |
| pipeline_pipeline_empty_input | pipeline | PEI | Pipeline empty input | hub | pipeline::pipeline_empty_input | f64 | 1 | 1826319.00 | 1000.00 |
| pipeline_pipeline_normalize_and_filter | pipeline | PNA | Pipeline normalize and filter | hub | pipeline::pipeline_normalize_and_filter | f64 | 1 | 2458799.00 | 500.00 |
| pipeline_pipeline_scale | pipeline | PS | Pipeline scale | hub | pipeline::pipeline_scale | f64 | 1 | 2815099.00 | 500.00 |
| tools_approx_equal_works | tools | AEW | Approx equal works | hub | tools::approx_equal_works | f64 | 1 | 2334129.00 | 500.00 |
| tools_arena_alloc_and_read | tools | AAA | Arena alloc and read | hub | tools::arena_alloc_and_read | f64 | 1 | 2225549.00 | 500.00 |
| tools_arena_capacity_limit | tools | ACL | Arena capacity limit | hub | tools::arena_capacity_limit | f64 | 1 | 2216839.00 | 500.00 |
| tools_arena_matrix | tools | AM | Arena matrix | hub | tools::arena_matrix | f64 | 1 | 2026409.00 | 500.00 |
| tools_arena_reset | tools | AR | Arena reset | hub | tools::arena_reset | f64 | 1 | 1759449.00 | 1000.00 |
| tools_fingerprint_changes_with_data | tools | FCW | Fingerprint changes with data | hub | tools::fingerprint_changes_with_data | f64 | 1 | 1674710.00 | 1000.00 |
| tools_fingerprint_deterministic | tools | FD | Fingerprint deterministic | hub | tools::fingerprint_deterministic | f64 | 1 | 1707789.00 | 1000.00 |
| tools_format_scientific_output | tools | FSO | Format scientific output | hub | tools::format_scientific_output | f64 | 1 | 1845669.00 | 1000.00 |
| tools_kahan_dot_basic | tools | KDB | Kahan dot basic | hub | tools::kahan_dot_basic | f64 | 1 | 1730159.00 | 1000.00 |
| tools_kahan_sum_precision | tools | KSP | Kahan sum precision | hub | tools::kahan_sum_precision | f64 | 1 | 4199748.00 | 250.00 |
| tools_linspace_basic | tools | LB | Linspace basic | hub | tools::linspace_basic | f64 | 1 | 2410639.00 | 500.00 |
| tools_logspace_basic | tools | LB | Logspace basic | hub | tools::logspace_basic | f64 | 1 | 2218689.00 | 500.00 |
| tools_reproducible_context_fork | tools | RCF | Reproducible context fork | hub | tools::reproducible_context_fork | f64 | 1 | 2645379.00 | 500.00 |
| tools_rng_deterministic | tools | RD | Rng deterministic | hub | tools::rng_deterministic | f64 | 1 | 2132529.00 | 500.00 |
| tools_rng_different_seeds_differ | tools | RDS | Rng different seeds differ | hub | tools::rng_different_seeds_differ | f64 | 1 | 2492989.00 | 500.00 |
| tools_rng_normal_distribution | tools | RND | Rng normal distribution | hub | tools::rng_normal_distribution | f64 | 1 | 2842519.00 | 500.00 |
| tools_rng_uniform_range | tools | RUR | Rng uniform range | hub | tools::rng_uniform_range | f64 | 1 | 1818399.00 | 1000.00 |
| tools_scratch_pool_independent | tools | SPI | Scratch pool independent | hub | tools::scratch_pool_independent | f64 | 1 | 1744309.00 | 1000.00 |
## Analytics
### Category Distribution
| Category | Count | Avg (ns) | Min (ns) | Max (ns) |
|---|---|---|---|---|
| cross_domain | 60 | 1910219.12 | 1641399.00 | 3634938.00 |
| pipeline | 4 | 2350646.50 | 1826319.00 | 2815099.00 |
| tools | 18 | 2223661.00 | 1674710.00 | 4199748.00 |

### Top 10 Fastest Entries
| # | Entry | Avg (ns) |
|---|---|---|
| 1 | cross_domain_biophysics_fick_diffusion_flux | 1641399.00 |
| 2 | cross_domain_biophysics_reynolds_number | 1641519.00 |
| 3 | cross_domain_astrochemistry_stroemgren_radius | 1644349.00 |
| 4 | cross_domain_atmospheric_physics_pressure_at_altitude | 1644369.00 |
| 5 | cross_domain_atmospheric_physics_dry_adiabatic_lapse_rate | 1647640.00 |
| 6 | cross_domain_atmospheric_physics_stefan_boltzmann_flux | 1647989.00 |
| 7 | cross_domain_astrochemistry_freefall_time | 1648329.00 |
| 8 | cross_domain_biophysics_electrophoretic_mobility | 1652439.00 |
| 9 | cross_domain_geochemistry_mixing_two_component | 1656700.00 |
| 10 | cross_domain_geophysics_bouguer_anomaly | 1657019.00 |

### Top 10 Slowest Entries
| # | Entry | Avg (ns) |
|---|---|---|
| 1 | tools_kahan_sum_precision | 4199748.00 |
| 2 | cross_domain_planetary_geology_crater_counting_surface_age | 3634938.00 |
| 3 | cross_domain_biochemistry_gibbs_free_energy | 2899588.00 |
| 4 | tools_rng_normal_distribution | 2842519.00 |
| 5 | pipeline_pipeline_scale | 2815099.00 |
| 6 | tools_reproducible_context_fork | 2645379.00 |
| 7 | cross_domain_validation_single_case | 2575979.00 |
| 8 | tools_rng_different_seeds_differ | 2492989.00 |
| 9 | pipeline_pipeline_normalize_and_filter | 2458799.00 |
| 10 | cross_domain_biochemistry_henderson_hasselbalch | 2427579.00 |

