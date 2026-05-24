# SciForge Chemistry Test

106 benchmark entries across 5 formats.

---

## Grid
```
  acid_base       : EPV   HH  HHE  PSA  PSB   PR
  analytical      : ATR   BL   Di   PR   RF  STN
  colloids        : BDC   SS  ZPS
  computational   : DHE   GP  TFK
  crystallography :  BL  ISC  SCS
  dispatch        : ATE   CP  DMP  DUF  GKR  HLF   NP  RCA
  electrochemistry:  BV   CP   FM   NP
  environmental   : BOD  DOS  HLS
  equilibrium     : ECF   RQ   VH
  gas_laws        :  BL  BLP   CL  DPP  GLE  IGI  IGP  VDW
  green_chemistry :  EF  GAE  RME
  inorganic       : LEB  MMS  PED
  kinetics        :  EE  HLF   MM  RCA
  molecular       :  BO   DM   FC   MM
  nuclear         : BEP  HLT   QV
  organic         : DOU   HE  PCL
  photochemistry  :  FE   PE   QY   SV
  polymers        :  CE  FHF   PI
  quantum_chem    : HTP  LBE   MP
  reaction_engineering:  CV   DN  PCF
  solid_state     : BGF   CS   FD
  solutions       : BPE  FPD   OP   RL
  spectroscopy    : CSP   LF   RM  WTW
  stoichiometry   :  AE   LR   PY   TY
  surface         :  CA   FI   LI   LP
  thermochemistry :  CC   EC  GFE   HL
  transport       : DCS  FFL   SN
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
| acid_base_equivalence_point_volume | acid_base | EPV | Equivalence point volume | chemistry | acid_base::equivalence_point_volume | f64 | 1 | 3199816.00 | 333.33 |
| acid_base_henderson_hasselbalch | acid_base | HH | Henderson hasselbalch | chemistry | acid_base::henderson_hasselbalch | f64 | 1 | 3228756.00 | 333.33 |
| acid_base_henderson_hasselbalch_excess_base | acid_base | HHE | Henderson hasselbalch excess base | chemistry | acid_base::henderson_hasselbalch_excess_base | f64 | 1 | 2940876.00 | 500.00 |
| acid_base_ph_strong_acid | acid_base | PSA | Ph strong acid | chemistry | acid_base::ph_strong_acid | f64 | 1 | 2706945.00 | 500.00 |
| acid_base_ph_strong_base | acid_base | PSB | Ph strong base | chemistry | acid_base::ph_strong_base | f64 | 1 | 2761065.00 | 500.00 |
| acid_base_pka_roundtrip | acid_base | PR | Pka roundtrip | chemistry | acid_base::pka_roundtrip | f64 | 1 | 2318704.00 | 500.00 |
| analytical_absorbance_transmittance_roundtrip | analytical | ATR | Absorbance transmittance roundtrip | chemistry | analytical::absorbance_transmittance_roundtrip | f64 | 1 | 2188194.00 | 500.00 |
| analytical_beer_lambert | analytical | BL | Beer lambert | chemistry | analytical::beer_lambert | f64 | 1 | 2875965.00 | 500.00 |
| analytical_dilution | analytical | Di | Dilution | chemistry | analytical::dilution | f64 | 1 | 3111296.00 | 333.33 |
| analytical_percent_recovery | analytical | PR | Percent recovery | chemistry | analytical::percent_recovery | f64 | 1 | 3173406.00 | 333.33 |
| analytical_retention_factor | analytical | RF | Retention factor | chemistry | analytical::retention_factor | f64 | 1 | 3216306.00 | 333.33 |
| analytical_signal_to_noise | analytical | STN | Signal to noise | chemistry | analytical::signal_to_noise | f64 | 1 | 3161556.00 | 333.33 |
| colloids_brownian_diffusion_coefficient | colloids | BDC | Brownian diffusion coefficient | chemistry | colloids::brownian_diffusion_coefficient | f64 | 1 | 3199926.00 | 333.33 |
| colloids_stokes_sedimentation | colloids | SS | Stokes sedimentation | chemistry | colloids::stokes_sedimentation | f64 | 1 | 4499708.00 | 250.00 |
| colloids_zeta_potential_smoluchowski | colloids | ZPS | Zeta potential smoluchowski | chemistry | colloids::zeta_potential_smoluchowski | f64 | 1 | 3260246.00 | 333.33 |
| computational_dft_hartree_energy | computational | DHE | Dft hartree energy | chemistry | computational::dft_hartree_energy | f64 | 1 | 3011836.00 | 333.33 |
| computational_gaussian_primitive | computational | GP | Gaussian primitive | chemistry | computational::gaussian_primitive | f64 | 1 | 3057065.00 | 333.33 |
| computational_thomas_fermi_kinetic_energy | computational | TFK | Thomas fermi kinetic energy | chemistry | computational::thomas_fermi_kinetic_energy | f64 | 1 | 3150966.00 | 333.33 |
| crystallography_bragg_law | crystallography | BL | Bragg law | chemistry | crystallography::bragg_law | f64 | 1 | 2945936.00 | 500.00 |
| crystallography_interplanar_spacing_cubic | crystallography | ISC | Interplanar spacing cubic | chemistry | crystallography::interplanar_spacing_cubic | f64 | 1 | 2473564.00 | 500.00 |
| crystallography_scherrer_crystal_size | crystallography | SCS | Scherrer crystal size | chemistry | crystallography::scherrer_crystal_size | f64 | 1 | 2443085.00 | 500.00 |
| dispatch_arrhenius_temperature_effect | dispatch | ATE | Arrhenius temperature effect | chemistry | dispatch::arrhenius_temperature_effect | f64 | 1 | 2338884.00 | 500.00 |
| dispatch_cell_potential | dispatch | CP | Cell potential | chemistry | dispatch::cell_potential | f64 | 1 | 1996164.00 | 1000.00 |
| dispatch_dispatch_missing_parameter_returns_error | dispatch | DMP | Dispatch missing parameter returns error | chemistry | dispatch::dispatch_missing_parameter_returns_error | f64 | 1 | 2231924.00 | 500.00 |
| dispatch_dispatch_unknown_function_returns_error | dispatch | DUF | Dispatch unknown function returns error | chemistry | dispatch::dispatch_unknown_function_returns_error | f64 | 1 | 2151014.00 | 500.00 |
| dispatch_gibbs_keq_roundtrip | dispatch | GKR | Gibbs keq roundtrip | chemistry | dispatch::gibbs_keq_roundtrip | f64 | 1 | 2355104.00 | 500.00 |
| dispatch_half_life_first_order | dispatch | HLF | Half life first order | chemistry | dispatch::half_life_first_order | f64 | 1 | 2565515.00 | 500.00 |
| dispatch_nernst_potential | dispatch | NP | Nernst potential | chemistry | dispatch::nernst_potential | f64 | 1 | 2413804.00 | 500.00 |
| dispatch_rate_constant_arrhenius | dispatch | RCA | Rate constant arrhenius | chemistry | dispatch::rate_constant_arrhenius | f64 | 1 | 2286715.00 | 500.00 |
| electrochemistry_butler_volmer | electrochemistry | BV | Butler volmer | chemistry | electrochemistry::butler_volmer | f64 | 1 | 2048023.00 | 500.00 |
| electrochemistry_cell_potential | electrochemistry | CP | Cell potential | chemistry | electrochemistry::cell_potential | f64 | 1 | 1912744.00 | 1000.00 |
| electrochemistry_faraday_mass | electrochemistry | FM | Faraday mass | chemistry | electrochemistry::faraday_mass | f64 | 1 | 2073684.00 | 500.00 |
| electrochemistry_nernst_potential | electrochemistry | NP | Nernst potential | chemistry | electrochemistry::nernst_potential | f64 | 1 | 2908495.00 | 500.00 |
| environmental_biochemical_oxygen_demand | environmental | BOD | Biochemical oxygen demand | chemistry | environmental::biochemical_oxygen_demand | f64 | 1 | 3525997.00 | 333.33 |
| environmental_dissolved_oxygen_saturation | environmental | DOS | Dissolved oxygen saturation | chemistry | environmental::dissolved_oxygen_saturation | f64 | 1 | 3071855.00 | 333.33 |
| environmental_henry_law_solubility | environmental | HLS | Henry law solubility | chemistry | environmental::henry_law_solubility | f64 | 1 | 2462535.00 | 500.00 |
| equilibrium_equilibrium_constant_from_gibbs | equilibrium | ECF | Equilibrium constant from gibbs | chemistry | equilibrium::equilibrium_constant_from_gibbs | f64 | 1 | 1926654.00 | 1000.00 |
| equilibrium_reaction_quotient | equilibrium | RQ | Reaction quotient | chemistry | equilibrium::reaction_quotient | f64 | 1 | 1958953.00 | 1000.00 |
| equilibrium_vant_hoff | equilibrium | VH | Vant hoff | chemistry | equilibrium::vant_hoff | f64 | 1 | 1763623.00 | 1000.00 |
| gas_laws_boyles_law | gas_laws | BL | Boyles law | chemistry | gas_laws::boyles_law | f64 | 1 | 1882304.00 | 1000.00 |
| gas_laws_boyles_law_pressure_halves_when_volume_doubles | gas_laws | BLP | Boyles law pressure halves when volume doubles | chemistry | gas_laws::boyles_law_pressure_halves_when_volume_doubles | f64 | 1 | 2047674.00 | 500.00 |
| gas_laws_charles_law | gas_laws | CL | Charles law | chemistry | gas_laws::charles_law | f64 | 1 | 1967493.00 | 1000.00 |
| gas_laws_dalton_partial_pressure | gas_laws | DPP | Dalton partial pressure | chemistry | gas_laws::dalton_partial_pressure | f64 | 1 | 1968674.00 | 1000.00 |
| gas_laws_grahams_law_effusion | gas_laws | GLE | Grahams law effusion | chemistry | gas_laws::grahams_law_effusion | f64 | 1 | 1955754.00 | 1000.00 |
| gas_laws_ideal_gas_inverse_temperature_roundtrip | gas_laws | IGI | Ideal gas inverse temperature roundtrip | chemistry | gas_laws::ideal_gas_inverse_temperature_roundtrip | f64 | 1 | 1925243.00 | 1000.00 |
| gas_laws_ideal_gas_pressure_chem | gas_laws | IGP | Ideal gas pressure chem | chemistry | gas_laws::ideal_gas_pressure_chem | f64 | 1 | 1896114.00 | 1000.00 |
| gas_laws_van_der_waals_pressure | gas_laws | VDW | Van der waals pressure | chemistry | gas_laws::van_der_waals_pressure | f64 | 1 | 2377754.00 | 500.00 |
| green_chemistry_e_factor | green_chemistry | EF | E factor | chemistry | green_chemistry::e_factor | f64 | 1 | 1989264.00 | 1000.00 |
| green_chemistry_gc_atom_economy | green_chemistry | GAE | Gc atom economy | chemistry | green_chemistry::gc_atom_economy | f64 | 1 | 2443184.00 | 500.00 |
| green_chemistry_reaction_mass_efficiency | green_chemistry | RME | Reaction mass efficiency | chemistry | green_chemistry::reaction_mass_efficiency | f64 | 1 | 2343375.00 | 500.00 |
| inorganic_lattice_energy_born_lande | inorganic | LEB | Lattice energy born lande | chemistry | inorganic::lattice_energy_born_lande | f64 | 1 | 2021043.00 | 500.00 |
| inorganic_magnetic_moment_spin_only | inorganic | MMS | Magnetic moment spin only | chemistry | inorganic::magnetic_moment_spin_only | f64 | 1 | 2247195.00 | 500.00 |
| inorganic_pauling_electronegativity_difference | inorganic | PED | Pauling electronegativity difference | chemistry | inorganic::pauling_electronegativity_difference | f64 | 1 | 1965833.00 | 1000.00 |
| kinetics_eyring_equation | kinetics | EE | Eyring equation | chemistry | kinetics::eyring_equation | f64 | 1 | 1946214.00 | 1000.00 |
| kinetics_half_life_first_order | kinetics | HLF | Half life first order | chemistry | kinetics::half_life_first_order | f64 | 1 | 1978513.00 | 1000.00 |
| kinetics_michaelis_menten | kinetics | MM | Michaelis menten | chemistry | kinetics::michaelis_menten | f64 | 1 | 2105744.00 | 500.00 |
| kinetics_rate_constant_arrhenius | kinetics | RCA | Rate constant arrhenius | chemistry | kinetics::rate_constant_arrhenius | f64 | 1 | 2368655.00 | 500.00 |
| molecular_bond_order | molecular | BO | Bond order | chemistry | molecular::bond_order | f64 | 1 | 2512054.00 | 500.00 |
| molecular_dipole_moment | molecular | DM | Dipole moment | chemistry | molecular::dipole_moment | f64 | 1 | 2658065.00 | 500.00 |
| molecular_formal_charge | molecular | FC | Formal charge | chemistry | molecular::formal_charge | f64 | 1 | 2305235.00 | 500.00 |
| molecular_molar_mass | molecular | MM | Molar mass | chemistry | molecular::molar_mass | f64 | 1 | 1948033.00 | 1000.00 |
| nuclear_binding_energy_per_nucleon | nuclear | BEP | Binding energy per nucleon | chemistry | nuclear::binding_energy_per_nucleon | f64 | 1 | 1823253.00 | 1000.00 |
| nuclear_half_life_to_decay_constant | nuclear | HLT | Half life to decay constant | chemistry | nuclear::half_life_to_decay_constant | f64 | 1 | 2440335.00 | 500.00 |
| nuclear_q_value | nuclear | QV | Q value | chemistry | nuclear::q_value | f64 | 1 | 2542325.00 | 500.00 |
| organic_degree_of_unsaturation | organic | DOU | Degree of unsaturation | chemistry | organic::degree_of_unsaturation | f64 | 1 | 3241926.00 | 333.33 |
| organic_hammett_equation | organic | HE | Hammett equation | chemistry | organic::hammett_equation | f64 | 1 | 2462084.00 | 500.00 |
| organic_partition_coefficient_log_p | organic | PCL | Partition coefficient log p | chemistry | organic::partition_coefficient_log_p | f64 | 1 | 2010444.00 | 500.00 |
| photochemistry_fret_efficiency | photochemistry | FE | Fret efficiency | chemistry | photochemistry::fret_efficiency | f64 | 1 | 2310114.00 | 500.00 |
| photochemistry_photon_energy | photochemistry | PE | Photon energy | chemistry | photochemistry::photon_energy | f64 | 1 | 2065284.00 | 500.00 |
| photochemistry_quantum_yield | photochemistry | QY | Quantum yield | chemistry | photochemistry::quantum_yield | f64 | 1 | 2072074.00 | 500.00 |
| photochemistry_stern_volmer | photochemistry | SV | Stern volmer | chemistry | photochemistry::stern_volmer | f64 | 1 | 2370204.00 | 500.00 |
| polymers_carothers_equation | polymers | CE | Carothers equation | chemistry | polymers::carothers_equation | f64 | 1 | 2220684.00 | 500.00 |
| polymers_flory_huggins_free_energy | polymers | FHF | Flory huggins free energy | chemistry | polymers::flory_huggins_free_energy | f64 | 1 | 1911754.00 | 1000.00 |
| polymers_polydispersity_index | polymers | PI | Polydispersity index | chemistry | polymers::polydispersity_index | f64 | 1 | 2354944.00 | 500.00 |
| quantum_chem_huckel_total_pi_energy | quantum_chem | HTP | Huckel total pi energy | chemistry | quantum_chem::huckel_total_pi_energy | f64 | 1 | 2082864.00 | 500.00 |
| quantum_chem_lcao_bonding_energy | quantum_chem | LBE | Lcao bonding energy | chemistry | quantum_chem::lcao_bonding_energy | f64 | 1 | 2029264.00 | 500.00 |
| quantum_chem_mulliken_population | quantum_chem | MP | Mulliken population | chemistry | quantum_chem::mulliken_population | f64 | 1 | 1969594.00 | 1000.00 |
| reaction_engineering_cstr_volume | reaction_engineering | CV | Cstr volume | chemistry | reaction_engineering::cstr_volume | f64 | 1 | 2102004.00 | 500.00 |
| reaction_engineering_damkohler_number | reaction_engineering | DN | Damkohler number | chemistry | reaction_engineering::damkohler_number | f64 | 1 | 2103853.00 | 500.00 |
| reaction_engineering_pfr_conversion_first_order | reaction_engineering | PCF | Pfr conversion first order | chemistry | reaction_engineering::pfr_conversion_first_order | f64 | 1 | 2672285.00 | 500.00 |
| solid_state_band_gap_from_absorption | solid_state | BGF | Band gap from absorption | chemistry | solid_state::band_gap_from_absorption | f64 | 1 | 2371955.00 | 500.00 |
| solid_state_conductivity_semiconductor | solid_state | CS | Conductivity semiconductor | chemistry | solid_state::conductivity_semiconductor | f64 | 1 | 2197994.00 | 500.00 |
| solid_state_fermi_dirac | solid_state | FD | Fermi dirac | chemistry | solid_state::fermi_dirac | f64 | 1 | 2109914.00 | 500.00 |
| solutions_boiling_point_elevation | solutions | BPE | Boiling point elevation | chemistry | solutions::boiling_point_elevation | f64 | 1 | 1986943.00 | 1000.00 |
| solutions_freezing_point_depression | solutions | FPD | Freezing point depression | chemistry | solutions::freezing_point_depression | f64 | 1 | 2054204.00 | 500.00 |
| solutions_osmotic_pressure | solutions | OP | Osmotic pressure | chemistry | solutions::osmotic_pressure | f64 | 1 | 2000434.00 | 500.00 |
| solutions_raoult_law | solutions | RL | Raoult law | chemistry | solutions::raoult_law | f64 | 1 | 2598335.00 | 500.00 |
| spectroscopy_chemical_shift_ppm | spectroscopy | CSP | Chemical shift ppm | chemistry | spectroscopy::chemical_shift_ppm | f64 | 1 | 2165504.00 | 500.00 |
| spectroscopy_larmor_frequency | spectroscopy | LF | Larmor frequency | chemistry | spectroscopy::larmor_frequency | f64 | 1 | 2271304.00 | 500.00 |
| spectroscopy_reduced_mass | spectroscopy | RM | Reduced mass | chemistry | spectroscopy::reduced_mass | f64 | 1 | 1975244.00 | 1000.00 |
| spectroscopy_wavenumber_to_wavelength | spectroscopy | WTW | Wavenumber to wavelength | chemistry | spectroscopy::wavenumber_to_wavelength | f64 | 1 | 2003593.00 | 500.00 |
| stoichiometry_atom_economy | stoichiometry | AE | Atom economy | chemistry | stoichiometry::atom_economy | f64 | 1 | 1834744.00 | 1000.00 |
| stoichiometry_limiting_reagent | stoichiometry | LR | Limiting reagent | chemistry | stoichiometry::limiting_reagent | f64 | 1 | 2023463.00 | 500.00 |
| stoichiometry_percent_yield | stoichiometry | PY | Percent yield | chemistry | stoichiometry::percent_yield | f64 | 1 | 2098064.00 | 500.00 |
| stoichiometry_theoretical_yield | stoichiometry | TY | Theoretical yield | chemistry | stoichiometry::theoretical_yield | f64 | 1 | 2138694.00 | 500.00 |
| surface_contact_angle | surface | CA | Contact angle | chemistry | surface::contact_angle | f64 | 1 | 2284804.00 | 500.00 |
| surface_freundlich_isotherm | surface | FI | Freundlich isotherm | chemistry | surface::freundlich_isotherm | f64 | 1 | 2360315.00 | 500.00 |
| surface_langmuir_isotherm | surface | LI | Langmuir isotherm | chemistry | surface::langmuir_isotherm | f64 | 1 | 2021224.00 | 500.00 |
| surface_laplace_pressure | surface | LP | Laplace pressure | chemistry | surface::laplace_pressure | f64 | 1 | 2015033.00 | 500.00 |
| thermochemistry_clausius_clapeyron | thermochemistry | CC | Clausius clapeyron | chemistry | thermochemistry::clausius_clapeyron | f64 | 1 | 2464255.00 | 500.00 |
| thermochemistry_entropy_change | thermochemistry | EC | Entropy change | chemistry | thermochemistry::entropy_change | f64 | 1 | 2301024.00 | 500.00 |
| thermochemistry_gibbs_free_energy | thermochemistry | GFE | Gibbs free energy | chemistry | thermochemistry::gibbs_free_energy | f64 | 1 | 2151994.00 | 500.00 |
| thermochemistry_hess_law | thermochemistry | HL | Hess law | chemistry | thermochemistry::hess_law | f64 | 1 | 2094874.00 | 500.00 |
| transport_diffusion_coefficient_stokes_einstein | transport | DCS | Diffusion coefficient stokes einstein | chemistry | transport::diffusion_coefficient_stokes_einstein | f64 | 1 | 1859913.00 | 1000.00 |
| transport_fick_first_law | transport | FFL | Fick first law | chemistry | transport::fick_first_law | f64 | 1 | 1975804.00 | 1000.00 |
| transport_sherwood_number | transport | SN | Sherwood number | chemistry | transport::sherwood_number | f64 | 1 | 1843314.00 | 1000.00 |
## Analytics
### Category Distribution
| Category | Count | Avg (ns) | Min (ns) | Max (ns) |
|---|---|---|---|---|
| acid_base | 6 | 2859360.33 | 2318704.00 | 3228756.00 |
| analytical | 6 | 2954453.83 | 2188194.00 | 3216306.00 |
| colloids | 3 | 3653293.33 | 3199926.00 | 4499708.00 |
| computational | 3 | 3073289.00 | 3011836.00 | 3150966.00 |
| crystallography | 3 | 2620861.67 | 2443085.00 | 2945936.00 |
| dispatch | 8 | 2292390.50 | 1996164.00 | 2565515.00 |
| electrochemistry | 4 | 2235736.50 | 1912744.00 | 2908495.00 |
| environmental | 3 | 3020129.00 | 2462535.00 | 3525997.00 |
| equilibrium | 3 | 1883076.67 | 1763623.00 | 1958953.00 |
| gas_laws | 8 | 2002626.25 | 1882304.00 | 2377754.00 |
| green_chemistry | 3 | 2258607.67 | 1989264.00 | 2443184.00 |
| inorganic | 3 | 2078023.67 | 1965833.00 | 2247195.00 |
| kinetics | 4 | 2099781.50 | 1946214.00 | 2368655.00 |
| molecular | 4 | 2355846.75 | 1948033.00 | 2658065.00 |
| nuclear | 3 | 2268637.67 | 1823253.00 | 2542325.00 |
| organic | 3 | 2571484.67 | 2010444.00 | 3241926.00 |
| photochemistry | 4 | 2204419.00 | 2065284.00 | 2370204.00 |
| polymers | 3 | 2162460.67 | 1911754.00 | 2354944.00 |
| quantum_chem | 3 | 2027240.67 | 1969594.00 | 2082864.00 |
| reaction_engineering | 3 | 2292714.00 | 2102004.00 | 2672285.00 |
| solid_state | 3 | 2226621.00 | 2109914.00 | 2371955.00 |
| solutions | 4 | 2159979.00 | 1986943.00 | 2598335.00 |
| spectroscopy | 4 | 2103911.25 | 1975244.00 | 2271304.00 |
| stoichiometry | 4 | 2023741.25 | 1834744.00 | 2138694.00 |
| surface | 4 | 2170344.00 | 2015033.00 | 2360315.00 |
| thermochemistry | 4 | 2253036.75 | 2094874.00 | 2464255.00 |
| transport | 3 | 1893010.33 | 1843314.00 | 1975804.00 |

### Top 10 Fastest Entries
| # | Entry | Avg (ns) |
|---|---|---|
| 1 | equilibrium_vant_hoff | 1763623.00 |
| 2 | nuclear_binding_energy_per_nucleon | 1823253.00 |
| 3 | stoichiometry_atom_economy | 1834744.00 |
| 4 | transport_sherwood_number | 1843314.00 |
| 5 | transport_diffusion_coefficient_stokes_einstein | 1859913.00 |
| 6 | gas_laws_boyles_law | 1882304.00 |
| 7 | gas_laws_ideal_gas_pressure_chem | 1896114.00 |
| 8 | polymers_flory_huggins_free_energy | 1911754.00 |
| 9 | electrochemistry_cell_potential | 1912744.00 |
| 10 | gas_laws_ideal_gas_inverse_temperature_roundtrip | 1925243.00 |

### Top 10 Slowest Entries
| # | Entry | Avg (ns) |
|---|---|---|
| 1 | colloids_stokes_sedimentation | 4499708.00 |
| 2 | environmental_biochemical_oxygen_demand | 3525997.00 |
| 3 | colloids_zeta_potential_smoluchowski | 3260246.00 |
| 4 | organic_degree_of_unsaturation | 3241926.00 |
| 5 | acid_base_henderson_hasselbalch | 3228756.00 |
| 6 | analytical_retention_factor | 3216306.00 |
| 7 | colloids_brownian_diffusion_coefficient | 3199926.00 |
| 8 | acid_base_equivalence_point_volume | 3199816.00 |
| 9 | analytical_percent_recovery | 3173406.00 |
| 10 | analytical_signal_to_noise | 3161556.00 |

