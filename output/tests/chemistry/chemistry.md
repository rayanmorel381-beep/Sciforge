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
| acid_base_equivalence_point_volume | acid_base | EPV | Equivalence point volume | chemistry | acid_base::equivalence_point_volume | f64 | 1 | 2903591.00 | 500.00 |
| acid_base_henderson_hasselbalch | acid_base | HH | Henderson hasselbalch | chemistry | acid_base::henderson_hasselbalch | f64 | 1 | 2454720.00 | 500.00 |
| acid_base_henderson_hasselbalch_excess_base | acid_base | HHE | Henderson hasselbalch excess base | chemistry | acid_base::henderson_hasselbalch_excess_base | f64 | 1 | 2384241.00 | 500.00 |
| acid_base_ph_strong_acid | acid_base | PSA | Ph strong acid | chemistry | acid_base::ph_strong_acid | f64 | 1 | 2154050.00 | 500.00 |
| acid_base_ph_strong_base | acid_base | PSB | Ph strong base | chemistry | acid_base::ph_strong_base | f64 | 1 | 2168290.00 | 500.00 |
| acid_base_pka_roundtrip | acid_base | PR | Pka roundtrip | chemistry | acid_base::pka_roundtrip | f64 | 1 | 2187291.00 | 500.00 |
| analytical_absorbance_transmittance_roundtrip | analytical | ATR | Absorbance transmittance roundtrip | chemistry | analytical::absorbance_transmittance_roundtrip | f64 | 1 | 2146030.00 | 500.00 |
| analytical_beer_lambert | analytical | BL | Beer lambert | chemistry | analytical::beer_lambert | f64 | 1 | 1863600.00 | 1000.00 |
| analytical_dilution | analytical | Di | Dilution | chemistry | analytical::dilution | f64 | 1 | 1887311.00 | 1000.00 |
| analytical_percent_recovery | analytical | PR | Percent recovery | chemistry | analytical::percent_recovery | f64 | 1 | 1826290.00 | 1000.00 |
| analytical_retention_factor | analytical | RF | Retention factor | chemistry | analytical::retention_factor | f64 | 1 | 1824800.00 | 1000.00 |
| analytical_signal_to_noise | analytical | STN | Signal to noise | chemistry | analytical::signal_to_noise | f64 | 1 | 1888341.00 | 1000.00 |
| colloids_brownian_diffusion_coefficient | colloids | BDC | Brownian diffusion coefficient | chemistry | colloids::brownian_diffusion_coefficient | f64 | 1 | 1941600.00 | 1000.00 |
| colloids_stokes_sedimentation | colloids | SS | Stokes sedimentation | chemistry | colloids::stokes_sedimentation | f64 | 1 | 2076290.00 | 500.00 |
| colloids_zeta_potential_smoluchowski | colloids | ZPS | Zeta potential smoluchowski | chemistry | colloids::zeta_potential_smoluchowski | f64 | 1 | 2469781.00 | 500.00 |
| computational_dft_hartree_energy | computational | DHE | Dft hartree energy | chemistry | computational::dft_hartree_energy | f64 | 1 | 2306270.00 | 500.00 |
| computational_gaussian_primitive | computational | GP | Gaussian primitive | chemistry | computational::gaussian_primitive | f64 | 1 | 2332441.00 | 500.00 |
| computational_thomas_fermi_kinetic_energy | computational | TFK | Thomas fermi kinetic energy | chemistry | computational::thomas_fermi_kinetic_energy | f64 | 1 | 2591490.00 | 500.00 |
| crystallography_bragg_law | crystallography | BL | Bragg law | chemistry | crystallography::bragg_law | f64 | 1 | 2726460.00 | 500.00 |
| crystallography_interplanar_spacing_cubic | crystallography | ISC | Interplanar spacing cubic | chemistry | crystallography::interplanar_spacing_cubic | f64 | 1 | 2526641.00 | 500.00 |
| crystallography_scherrer_crystal_size | crystallography | SCS | Scherrer crystal size | chemistry | crystallography::scherrer_crystal_size | f64 | 1 | 2446600.00 | 500.00 |
| dispatch_arrhenius_temperature_effect | dispatch | ATE | Arrhenius temperature effect | chemistry | dispatch::arrhenius_temperature_effect | f64 | 1 | 2520701.00 | 500.00 |
| dispatch_cell_potential | dispatch | CP | Cell potential | chemistry | dispatch::cell_potential | f64 | 1 | 2551810.00 | 500.00 |
| dispatch_dispatch_missing_parameter_returns_error | dispatch | DMP | Dispatch missing parameter returns error | chemistry | dispatch::dispatch_missing_parameter_returns_error | f64 | 1 | 2411200.00 | 500.00 |
| dispatch_dispatch_unknown_function_returns_error | dispatch | DUF | Dispatch unknown function returns error | chemistry | dispatch::dispatch_unknown_function_returns_error | f64 | 1 | 5294451.00 | 200.00 |
| dispatch_gibbs_keq_roundtrip | dispatch | GKR | Gibbs keq roundtrip | chemistry | dispatch::gibbs_keq_roundtrip | f64 | 1 | 4299801.00 | 250.00 |
| dispatch_half_life_first_order | dispatch | HLF | Half life first order | chemistry | dispatch::half_life_first_order | f64 | 1 | 3280751.00 | 333.33 |
| dispatch_nernst_potential | dispatch | NP | Nernst potential | chemistry | dispatch::nernst_potential | f64 | 1 | 3274520.00 | 333.33 |
| dispatch_rate_constant_arrhenius | dispatch | RCA | Rate constant arrhenius | chemistry | dispatch::rate_constant_arrhenius | f64 | 1 | 3226701.00 | 333.33 |
| electrochemistry_butler_volmer | electrochemistry | BV | Butler volmer | chemistry | electrochemistry::butler_volmer | f64 | 1 | 2446410.00 | 500.00 |
| electrochemistry_cell_potential | electrochemistry | CP | Cell potential | chemistry | electrochemistry::cell_potential | f64 | 1 | 2223470.00 | 500.00 |
| electrochemistry_faraday_mass | electrochemistry | FM | Faraday mass | chemistry | electrochemistry::faraday_mass | f64 | 1 | 2986871.00 | 500.00 |
| electrochemistry_nernst_potential | electrochemistry | NP | Nernst potential | chemistry | electrochemistry::nernst_potential | f64 | 1 | 2688540.00 | 500.00 |
| environmental_biochemical_oxygen_demand | environmental | BOD | Biochemical oxygen demand | chemistry | environmental::biochemical_oxygen_demand | f64 | 1 | 3172921.00 | 333.33 |
| environmental_dissolved_oxygen_saturation | environmental | DOS | Dissolved oxygen saturation | chemistry | environmental::dissolved_oxygen_saturation | f64 | 1 | 3142561.00 | 333.33 |
| environmental_henry_law_solubility | environmental | HLS | Henry law solubility | chemistry | environmental::henry_law_solubility | f64 | 1 | 2536110.00 | 500.00 |
| equilibrium_equilibrium_constant_from_gibbs | equilibrium | ECF | Equilibrium constant from gibbs | chemistry | equilibrium::equilibrium_constant_from_gibbs | f64 | 1 | 2206610.00 | 500.00 |
| equilibrium_reaction_quotient | equilibrium | RQ | Reaction quotient | chemistry | equilibrium::reaction_quotient | f64 | 1 | 2372111.00 | 500.00 |
| equilibrium_vant_hoff | equilibrium | VH | Vant hoff | chemistry | equilibrium::vant_hoff | f64 | 1 | 2507570.00 | 500.00 |
| gas_laws_boyles_law | gas_laws | BL | Boyles law | chemistry | gas_laws::boyles_law | f64 | 1 | 3138031.00 | 333.33 |
| gas_laws_boyles_law_pressure_halves_when_volume_doubles | gas_laws | BLP | Boyles law pressure halves when volume doubles | chemistry | gas_laws::boyles_law_pressure_halves_when_volume_doubles | f64 | 1 | 2872550.00 | 500.00 |
| gas_laws_charles_law | gas_laws | CL | Charles law | chemistry | gas_laws::charles_law | f64 | 1 | 2523761.00 | 500.00 |
| gas_laws_dalton_partial_pressure | gas_laws | DPP | Dalton partial pressure | chemistry | gas_laws::dalton_partial_pressure | f64 | 1 | 2745790.00 | 500.00 |
| gas_laws_grahams_law_effusion | gas_laws | GLE | Grahams law effusion | chemistry | gas_laws::grahams_law_effusion | f64 | 1 | 2497130.00 | 500.00 |
| gas_laws_ideal_gas_inverse_temperature_roundtrip | gas_laws | IGI | Ideal gas inverse temperature roundtrip | chemistry | gas_laws::ideal_gas_inverse_temperature_roundtrip | f64 | 1 | 2423911.00 | 500.00 |
| gas_laws_ideal_gas_pressure_chem | gas_laws | IGP | Ideal gas pressure chem | chemistry | gas_laws::ideal_gas_pressure_chem | f64 | 1 | 2633350.00 | 500.00 |
| gas_laws_van_der_waals_pressure | gas_laws | VDW | Van der waals pressure | chemistry | gas_laws::van_der_waals_pressure | f64 | 1 | 2234081.00 | 500.00 |
| green_chemistry_e_factor | green_chemistry | EF | E factor | chemistry | green_chemistry::e_factor | f64 | 1 | 2142740.00 | 500.00 |
| green_chemistry_gc_atom_economy | green_chemistry | GAE | Gc atom economy | chemistry | green_chemistry::gc_atom_economy | f64 | 1 | 2581030.00 | 500.00 |
| green_chemistry_reaction_mass_efficiency | green_chemistry | RME | Reaction mass efficiency | chemistry | green_chemistry::reaction_mass_efficiency | f64 | 1 | 2532831.00 | 500.00 |
| inorganic_lattice_energy_born_lande | inorganic | LEB | Lattice energy born lande | chemistry | inorganic::lattice_energy_born_lande | f64 | 1 | 2268800.00 | 500.00 |
| inorganic_magnetic_moment_spin_only | inorganic | MMS | Magnetic moment spin only | chemistry | inorganic::magnetic_moment_spin_only | f64 | 1 | 2307771.00 | 500.00 |
| inorganic_pauling_electronegativity_difference | inorganic | PED | Pauling electronegativity difference | chemistry | inorganic::pauling_electronegativity_difference | f64 | 1 | 2720600.00 | 500.00 |
| kinetics_eyring_equation | kinetics | EE | Eyring equation | chemistry | kinetics::eyring_equation | f64 | 1 | 2580660.00 | 500.00 |
| kinetics_half_life_first_order | kinetics | HLF | Half life first order | chemistry | kinetics::half_life_first_order | f64 | 1 | 2142471.00 | 500.00 |
| kinetics_michaelis_menten | kinetics | MM | Michaelis menten | chemistry | kinetics::michaelis_menten | f64 | 1 | 2059340.00 | 500.00 |
| kinetics_rate_constant_arrhenius | kinetics | RCA | Rate constant arrhenius | chemistry | kinetics::rate_constant_arrhenius | f64 | 1 | 2247401.00 | 500.00 |
| molecular_bond_order | molecular | BO | Bond order | chemistry | molecular::bond_order | f64 | 1 | 1900220.00 | 1000.00 |
| molecular_dipole_moment | molecular | DM | Dipole moment | chemistry | molecular::dipole_moment | f64 | 1 | 1818850.00 | 1000.00 |
| molecular_formal_charge | molecular | FC | Formal charge | chemistry | molecular::formal_charge | f64 | 1 | 1750970.00 | 1000.00 |
| molecular_molar_mass | molecular | MM | Molar mass | chemistry | molecular::molar_mass | f64 | 1 | 1869441.00 | 1000.00 |
| nuclear_binding_energy_per_nucleon | nuclear | BEP | Binding energy per nucleon | chemistry | nuclear::binding_energy_per_nucleon | f64 | 1 | 1916020.00 | 1000.00 |
| nuclear_half_life_to_decay_constant | nuclear | HLT | Half life to decay constant | chemistry | nuclear::half_life_to_decay_constant | f64 | 1 | 1816080.00 | 1000.00 |
| nuclear_q_value | nuclear | QV | Q value | chemistry | nuclear::q_value | f64 | 1 | 1793011.00 | 1000.00 |
| organic_degree_of_unsaturation | organic | DOU | Degree of unsaturation | chemistry | organic::degree_of_unsaturation | f64 | 1 | 1844170.00 | 1000.00 |
| organic_hammett_equation | organic | HE | Hammett equation | chemistry | organic::hammett_equation | f64 | 1 | 1764800.00 | 1000.00 |
| organic_partition_coefficient_log_p | organic | PCL | Partition coefficient log p | chemistry | organic::partition_coefficient_log_p | f64 | 1 | 1781280.00 | 1000.00 |
| photochemistry_fret_efficiency | photochemistry | FE | Fret efficiency | chemistry | photochemistry::fret_efficiency | f64 | 1 | 1878291.00 | 1000.00 |
| photochemistry_photon_energy | photochemistry | PE | Photon energy | chemistry | photochemistry::photon_energy | f64 | 1 | 1860660.00 | 1000.00 |
| photochemistry_quantum_yield | photochemistry | QY | Quantum yield | chemistry | photochemistry::quantum_yield | f64 | 1 | 1921251.00 | 1000.00 |
| photochemistry_stern_volmer | photochemistry | SV | Stern volmer | chemistry | photochemistry::stern_volmer | f64 | 1 | 1943410.00 | 1000.00 |
| polymers_carothers_equation | polymers | CE | Carothers equation | chemistry | polymers::carothers_equation | f64 | 1 | 1770110.00 | 1000.00 |
| polymers_flory_huggins_free_energy | polymers | FHF | Flory huggins free energy | chemistry | polymers::flory_huggins_free_energy | f64 | 1 | 1787820.00 | 1000.00 |
| polymers_polydispersity_index | polymers | PI | Polydispersity index | chemistry | polymers::polydispersity_index | f64 | 1 | 2000371.00 | 500.00 |
| quantum_chem_huckel_total_pi_energy | quantum_chem | HTP | Huckel total pi energy | chemistry | quantum_chem::huckel_total_pi_energy | f64 | 1 | 2337830.00 | 500.00 |
| quantum_chem_lcao_bonding_energy | quantum_chem | LBE | Lcao bonding energy | chemistry | quantum_chem::lcao_bonding_energy | f64 | 1 | 2096410.00 | 500.00 |
| quantum_chem_mulliken_population | quantum_chem | MP | Mulliken population | chemistry | quantum_chem::mulliken_population | f64 | 1 | 1988691.00 | 1000.00 |
| reaction_engineering_cstr_volume | reaction_engineering | CV | Cstr volume | chemistry | reaction_engineering::cstr_volume | f64 | 1 | 1816170.00 | 1000.00 |
| reaction_engineering_damkohler_number | reaction_engineering | DN | Damkohler number | chemistry | reaction_engineering::damkohler_number | f64 | 1 | 1938040.00 | 1000.00 |
| reaction_engineering_pfr_conversion_first_order | reaction_engineering | PCF | Pfr conversion first order | chemistry | reaction_engineering::pfr_conversion_first_order | f64 | 1 | 1808481.00 | 1000.00 |
| solid_state_band_gap_from_absorption | solid_state | BGF | Band gap from absorption | chemistry | solid_state::band_gap_from_absorption | f64 | 1 | 1818300.00 | 1000.00 |
| solid_state_conductivity_semiconductor | solid_state | CS | Conductivity semiconductor | chemistry | solid_state::conductivity_semiconductor | f64 | 1 | 1777390.00 | 1000.00 |
| solid_state_fermi_dirac | solid_state | FD | Fermi dirac | chemistry | solid_state::fermi_dirac | f64 | 1 | 1742071.00 | 1000.00 |
| solutions_boiling_point_elevation | solutions | BPE | Boiling point elevation | chemistry | solutions::boiling_point_elevation | f64 | 1 | 1769360.00 | 1000.00 |
| solutions_freezing_point_depression | solutions | FPD | Freezing point depression | chemistry | solutions::freezing_point_depression | f64 | 1 | 1760050.00 | 1000.00 |
| solutions_osmotic_pressure | solutions | OP | Osmotic pressure | chemistry | solutions::osmotic_pressure | f64 | 1 | 1800270.00 | 1000.00 |
| solutions_raoult_law | solutions | RL | Raoult law | chemistry | solutions::raoult_law | f64 | 1 | 1738951.00 | 1000.00 |
| spectroscopy_chemical_shift_ppm | spectroscopy | CSP | Chemical shift ppm | chemistry | spectroscopy::chemical_shift_ppm | f64 | 1 | 2411590.00 | 500.00 |
| spectroscopy_larmor_frequency | spectroscopy | LF | Larmor frequency | chemistry | spectroscopy::larmor_frequency | f64 | 1 | 2036681.00 | 500.00 |
| spectroscopy_reduced_mass | spectroscopy | RM | Reduced mass | chemistry | spectroscopy::reduced_mass | f64 | 1 | 1975520.00 | 1000.00 |
| spectroscopy_wavenumber_to_wavelength | spectroscopy | WTW | Wavenumber to wavelength | chemistry | spectroscopy::wavenumber_to_wavelength | f64 | 1 | 1882960.00 | 1000.00 |
| stoichiometry_atom_economy | stoichiometry | AE | Atom economy | chemistry | stoichiometry::atom_economy | f64 | 1 | 1804140.00 | 1000.00 |
| stoichiometry_limiting_reagent | stoichiometry | LR | Limiting reagent | chemistry | stoichiometry::limiting_reagent | f64 | 1 | 1786401.00 | 1000.00 |
| stoichiometry_percent_yield | stoichiometry | PY | Percent yield | chemistry | stoichiometry::percent_yield | f64 | 1 | 1766090.00 | 1000.00 |
| stoichiometry_theoretical_yield | stoichiometry | TY | Theoretical yield | chemistry | stoichiometry::theoretical_yield | f64 | 1 | 1811590.00 | 1000.00 |
| surface_contact_angle | surface | CA | Contact angle | chemistry | surface::contact_angle | f64 | 1 | 1957831.00 | 1000.00 |
| surface_freundlich_isotherm | surface | FI | Freundlich isotherm | chemistry | surface::freundlich_isotherm | f64 | 1 | 2492390.00 | 500.00 |
| surface_langmuir_isotherm | surface | LI | Langmuir isotherm | chemistry | surface::langmuir_isotherm | f64 | 1 | 1850660.00 | 1000.00 |
| surface_laplace_pressure | surface | LP | Laplace pressure | chemistry | surface::laplace_pressure | f64 | 1 | 1786651.00 | 1000.00 |
| thermochemistry_clausius_clapeyron | thermochemistry | CC | Clausius clapeyron | chemistry | thermochemistry::clausius_clapeyron | f64 | 1 | 1784130.00 | 1000.00 |
| thermochemistry_entropy_change | thermochemistry | EC | Entropy change | chemistry | thermochemistry::entropy_change | f64 | 1 | 1811850.00 | 1000.00 |
| thermochemistry_gibbs_free_energy | thermochemistry | GFE | Gibbs free energy | chemistry | thermochemistry::gibbs_free_energy | f64 | 1 | 2284621.00 | 500.00 |
| thermochemistry_hess_law | thermochemistry | HL | Hess law | chemistry | thermochemistry::hess_law | f64 | 1 | 1977350.00 | 1000.00 |
| transport_diffusion_coefficient_stokes_einstein | transport | DCS | Diffusion coefficient stokes einstein | chemistry | transport::diffusion_coefficient_stokes_einstein | f64 | 1 | 2152840.00 | 500.00 |
| transport_fick_first_law | transport | FFL | Fick first law | chemistry | transport::fick_first_law | f64 | 1 | 2071531.00 | 500.00 |
| transport_sherwood_number | transport | SN | Sherwood number | chemistry | transport::sherwood_number | f64 | 1 | 2086250.00 | 500.00 |
## Analytics
### Category Distribution
| Category | Count | Avg (ns) | Min (ns) | Max (ns) |
|---|---|---|---|---|
| acid_base | 6 | 2375363.83 | 2154050.00 | 2903591.00 |
| analytical | 6 | 1906062.00 | 1824800.00 | 2146030.00 |
| colloids | 3 | 2162557.00 | 1941600.00 | 2469781.00 |
| computational | 3 | 2410067.00 | 2306270.00 | 2591490.00 |
| crystallography | 3 | 2566567.00 | 2446600.00 | 2726460.00 |
| dispatch | 8 | 3357491.88 | 2411200.00 | 5294451.00 |
| electrochemistry | 4 | 2586322.75 | 2223470.00 | 2986871.00 |
| environmental | 3 | 2950530.67 | 2536110.00 | 3172921.00 |
| equilibrium | 3 | 2362097.00 | 2206610.00 | 2507570.00 |
| gas_laws | 8 | 2633575.50 | 2234081.00 | 3138031.00 |
| green_chemistry | 3 | 2418867.00 | 2142740.00 | 2581030.00 |
| inorganic | 3 | 2432390.33 | 2268800.00 | 2720600.00 |
| kinetics | 4 | 2257468.00 | 2059340.00 | 2580660.00 |
| molecular | 4 | 1834870.25 | 1750970.00 | 1900220.00 |
| nuclear | 3 | 1841703.67 | 1793011.00 | 1916020.00 |
| organic | 3 | 1796750.00 | 1764800.00 | 1844170.00 |
| photochemistry | 4 | 1900903.00 | 1860660.00 | 1943410.00 |
| polymers | 3 | 1852767.00 | 1770110.00 | 2000371.00 |
| quantum_chem | 3 | 2140977.00 | 1988691.00 | 2337830.00 |
| reaction_engineering | 3 | 1854230.33 | 1808481.00 | 1938040.00 |
| solid_state | 3 | 1779253.67 | 1742071.00 | 1818300.00 |
| solutions | 4 | 1767157.75 | 1738951.00 | 1800270.00 |
| spectroscopy | 4 | 2076687.75 | 1882960.00 | 2411590.00 |
| stoichiometry | 4 | 1792055.25 | 1766090.00 | 1811590.00 |
| surface | 4 | 2021883.00 | 1786651.00 | 2492390.00 |
| thermochemistry | 4 | 1964487.75 | 1784130.00 | 2284621.00 |
| transport | 3 | 2103540.33 | 2071531.00 | 2152840.00 |

### Top 10 Fastest Entries
| # | Entry | Avg (ns) |
|---|---|---|
| 1 | solutions_raoult_law | 1738951.00 |
| 2 | solid_state_fermi_dirac | 1742071.00 |
| 3 | molecular_formal_charge | 1750970.00 |
| 4 | solutions_freezing_point_depression | 1760050.00 |
| 5 | organic_hammett_equation | 1764800.00 |
| 6 | stoichiometry_percent_yield | 1766090.00 |
| 7 | solutions_boiling_point_elevation | 1769360.00 |
| 8 | polymers_carothers_equation | 1770110.00 |
| 9 | solid_state_conductivity_semiconductor | 1777390.00 |
| 10 | organic_partition_coefficient_log_p | 1781280.00 |

### Top 10 Slowest Entries
| # | Entry | Avg (ns) |
|---|---|---|
| 1 | dispatch_dispatch_unknown_function_returns_error | 5294451.00 |
| 2 | dispatch_gibbs_keq_roundtrip | 4299801.00 |
| 3 | dispatch_half_life_first_order | 3280751.00 |
| 4 | dispatch_nernst_potential | 3274520.00 |
| 5 | dispatch_rate_constant_arrhenius | 3226701.00 |
| 6 | environmental_biochemical_oxygen_demand | 3172921.00 |
| 7 | environmental_dissolved_oxygen_saturation | 3142561.00 |
| 8 | gas_laws_boyles_law | 3138031.00 |
| 9 | electrochemistry_faraday_mass | 2986871.00 |
| 10 | acid_base_equivalence_point_volume | 2903591.00 |

