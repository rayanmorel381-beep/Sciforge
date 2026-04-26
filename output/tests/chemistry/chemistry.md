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
| acid_base_equivalence_point_volume | acid_base | EPV | Equivalence point volume | chemistry | acid_base::equivalence_point_volume | f64 | 1 | 2152229.00 | 500.00 |
| acid_base_henderson_hasselbalch | acid_base | HH | Henderson hasselbalch | chemistry | acid_base::henderson_hasselbalch | f64 | 1 | 1948919.00 | 1000.00 |
| acid_base_henderson_hasselbalch_excess_base | acid_base | HHE | Henderson hasselbalch excess base | chemistry | acid_base::henderson_hasselbalch_excess_base | f64 | 1 | 1829919.00 | 1000.00 |
| acid_base_ph_strong_acid | acid_base | PSA | Ph strong acid | chemistry | acid_base::ph_strong_acid | f64 | 1 | 1872120.00 | 1000.00 |
| acid_base_ph_strong_base | acid_base | PSB | Ph strong base | chemistry | acid_base::ph_strong_base | f64 | 1 | 1764459.00 | 1000.00 |
| acid_base_pka_roundtrip | acid_base | PR | Pka roundtrip | chemistry | acid_base::pka_roundtrip | f64 | 1 | 1756239.00 | 1000.00 |
| analytical_absorbance_transmittance_roundtrip | analytical | ATR | Absorbance transmittance roundtrip | chemistry | analytical::absorbance_transmittance_roundtrip | f64 | 1 | 2003619.00 | 500.00 |
| analytical_beer_lambert | analytical | BL | Beer lambert | chemistry | analytical::beer_lambert | f64 | 1 | 2218109.00 | 500.00 |
| analytical_dilution | analytical | Di | Dilution | chemistry | analytical::dilution | f64 | 1 | 2064179.00 | 500.00 |
| analytical_percent_recovery | analytical | PR | Percent recovery | chemistry | analytical::percent_recovery | f64 | 1 | 2240749.00 | 500.00 |
| analytical_retention_factor | analytical | RF | Retention factor | chemistry | analytical::retention_factor | f64 | 1 | 2247389.00 | 500.00 |
| analytical_signal_to_noise | analytical | STN | Signal to noise | chemistry | analytical::signal_to_noise | f64 | 1 | 2168159.00 | 500.00 |
| colloids_brownian_diffusion_coefficient | colloids | BDC | Brownian diffusion coefficient | chemistry | colloids::brownian_diffusion_coefficient | f64 | 1 | 2029799.00 | 500.00 |
| colloids_stokes_sedimentation | colloids | SS | Stokes sedimentation | chemistry | colloids::stokes_sedimentation | f64 | 1 | 1762510.00 | 1000.00 |
| colloids_zeta_potential_smoluchowski | colloids | ZPS | Zeta potential smoluchowski | chemistry | colloids::zeta_potential_smoluchowski | f64 | 1 | 1731269.00 | 1000.00 |
| computational_dft_hartree_energy | computational | DHE | Dft hartree energy | chemistry | computational::dft_hartree_energy | f64 | 1 | 1938939.00 | 1000.00 |
| computational_gaussian_primitive | computational | GP | Gaussian primitive | chemistry | computational::gaussian_primitive | f64 | 1 | 1809959.00 | 1000.00 |
| computational_thomas_fermi_kinetic_energy | computational | TFK | Thomas fermi kinetic energy | chemistry | computational::thomas_fermi_kinetic_energy | f64 | 1 | 2034499.00 | 500.00 |
| crystallography_bragg_law | crystallography | BL | Bragg law | chemistry | crystallography::bragg_law | f64 | 1 | 1782409.00 | 1000.00 |
| crystallography_interplanar_spacing_cubic | crystallography | ISC | Interplanar spacing cubic | chemistry | crystallography::interplanar_spacing_cubic | f64 | 1 | 1877910.00 | 1000.00 |
| crystallography_scherrer_crystal_size | crystallography | SCS | Scherrer crystal size | chemistry | crystallography::scherrer_crystal_size | f64 | 1 | 1949829.00 | 1000.00 |
| dispatch_arrhenius_temperature_effect | dispatch | ATE | Arrhenius temperature effect | chemistry | dispatch::arrhenius_temperature_effect | f64 | 1 | 2065189.00 | 500.00 |
| dispatch_cell_potential | dispatch | CP | Cell potential | chemistry | dispatch::cell_potential | f64 | 1 | 2221699.00 | 500.00 |
| dispatch_dispatch_missing_parameter_returns_error | dispatch | DMP | Dispatch missing parameter returns error | chemistry | dispatch::dispatch_missing_parameter_returns_error | f64 | 1 | 2209119.00 | 500.00 |
| dispatch_dispatch_unknown_function_returns_error | dispatch | DUF | Dispatch unknown function returns error | chemistry | dispatch::dispatch_unknown_function_returns_error | f64 | 1 | 2177829.00 | 500.00 |
| dispatch_gibbs_keq_roundtrip | dispatch | GKR | Gibbs keq roundtrip | chemistry | dispatch::gibbs_keq_roundtrip | f64 | 1 | 2004119.00 | 500.00 |
| dispatch_half_life_first_order | dispatch | HLF | Half life first order | chemistry | dispatch::half_life_first_order | f64 | 1 | 1896219.00 | 1000.00 |
| dispatch_nernst_potential | dispatch | NP | Nernst potential | chemistry | dispatch::nernst_potential | f64 | 1 | 1826650.00 | 1000.00 |
| dispatch_rate_constant_arrhenius | dispatch | RCA | Rate constant arrhenius | chemistry | dispatch::rate_constant_arrhenius | f64 | 1 | 1781359.00 | 1000.00 |
| electrochemistry_butler_volmer | electrochemistry | BV | Butler volmer | chemistry | electrochemistry::butler_volmer | f64 | 1 | 1795849.00 | 1000.00 |
| electrochemistry_cell_potential | electrochemistry | CP | Cell potential | chemistry | electrochemistry::cell_potential | f64 | 1 | 1731119.00 | 1000.00 |
| electrochemistry_faraday_mass | electrochemistry | FM | Faraday mass | chemistry | electrochemistry::faraday_mass | f64 | 1 | 1732559.00 | 1000.00 |
| electrochemistry_nernst_potential | electrochemistry | NP | Nernst potential | chemistry | electrochemistry::nernst_potential | f64 | 1 | 1873560.00 | 1000.00 |
| environmental_biochemical_oxygen_demand | environmental | BOD | Biochemical oxygen demand | chemistry | environmental::biochemical_oxygen_demand | f64 | 1 | 1902919.00 | 1000.00 |
| environmental_dissolved_oxygen_saturation | environmental | DOS | Dissolved oxygen saturation | chemistry | environmental::dissolved_oxygen_saturation | f64 | 1 | 1753249.00 | 1000.00 |
| environmental_henry_law_solubility | environmental | HLS | Henry law solubility | chemistry | environmental::henry_law_solubility | f64 | 1 | 1720379.00 | 1000.00 |
| equilibrium_equilibrium_constant_from_gibbs | equilibrium | ECF | Equilibrium constant from gibbs | chemistry | equilibrium::equilibrium_constant_from_gibbs | f64 | 1 | 1778409.00 | 1000.00 |
| equilibrium_reaction_quotient | equilibrium | RQ | Reaction quotient | chemistry | equilibrium::reaction_quotient | f64 | 1 | 1804550.00 | 1000.00 |
| equilibrium_vant_hoff | equilibrium | VH | Vant hoff | chemistry | equilibrium::vant_hoff | f64 | 1 | 1697389.00 | 1000.00 |
| gas_laws_boyles_law | gas_laws | BL | Boyles law | chemistry | gas_laws::boyles_law | f64 | 1 | 1708059.00 | 1000.00 |
| gas_laws_boyles_law_pressure_halves_when_volume_doubles | gas_laws | BLP | Boyles law pressure halves when volume doubles | chemistry | gas_laws::boyles_law_pressure_halves_when_volume_doubles | f64 | 1 | 1769869.00 | 1000.00 |
| gas_laws_charles_law | gas_laws | CL | Charles law | chemistry | gas_laws::charles_law | f64 | 1 | 1704450.00 | 1000.00 |
| gas_laws_dalton_partial_pressure | gas_laws | DPP | Dalton partial pressure | chemistry | gas_laws::dalton_partial_pressure | f64 | 1 | 1768959.00 | 1000.00 |
| gas_laws_grahams_law_effusion | gas_laws | GLE | Grahams law effusion | chemistry | gas_laws::grahams_law_effusion | f64 | 1 | 1790609.00 | 1000.00 |
| gas_laws_ideal_gas_inverse_temperature_roundtrip | gas_laws | IGI | Ideal gas inverse temperature roundtrip | chemistry | gas_laws::ideal_gas_inverse_temperature_roundtrip | f64 | 1 | 1730139.00 | 1000.00 |
| gas_laws_ideal_gas_pressure_chem | gas_laws | IGP | Ideal gas pressure chem | chemistry | gas_laws::ideal_gas_pressure_chem | f64 | 1 | 1725939.00 | 1000.00 |
| gas_laws_van_der_waals_pressure | gas_laws | VDW | Van der waals pressure | chemistry | gas_laws::van_der_waals_pressure | f64 | 1 | 1751040.00 | 1000.00 |
| green_chemistry_e_factor | green_chemistry | EF | E factor | chemistry | green_chemistry::e_factor | f64 | 1 | 1677509.00 | 1000.00 |
| green_chemistry_gc_atom_economy | green_chemistry | GAE | Gc atom economy | chemistry | green_chemistry::gc_atom_economy | f64 | 1 | 1731189.00 | 1000.00 |
| green_chemistry_reaction_mass_efficiency | green_chemistry | RME | Reaction mass efficiency | chemistry | green_chemistry::reaction_mass_efficiency | f64 | 1 | 1690569.00 | 1000.00 |
| inorganic_lattice_energy_born_lande | inorganic | LEB | Lattice energy born lande | chemistry | inorganic::lattice_energy_born_lande | f64 | 1 | 1807680.00 | 1000.00 |
| inorganic_magnetic_moment_spin_only | inorganic | MMS | Magnetic moment spin only | chemistry | inorganic::magnetic_moment_spin_only | f64 | 1 | 1691509.00 | 1000.00 |
| inorganic_pauling_electronegativity_difference | inorganic | PED | Pauling electronegativity difference | chemistry | inorganic::pauling_electronegativity_difference | f64 | 1 | 1691599.00 | 1000.00 |
| kinetics_eyring_equation | kinetics | EE | Eyring equation | chemistry | kinetics::eyring_equation | f64 | 1 | 1750489.00 | 1000.00 |
| kinetics_half_life_first_order | kinetics | HLF | Half life first order | chemistry | kinetics::half_life_first_order | f64 | 1 | 1848190.00 | 1000.00 |
| kinetics_michaelis_menten | kinetics | MM | Michaelis menten | chemistry | kinetics::michaelis_menten | f64 | 1 | 3377648.00 | 333.33 |
| kinetics_rate_constant_arrhenius | kinetics | RCA | Rate constant arrhenius | chemistry | kinetics::rate_constant_arrhenius | f64 | 1 | 2093079.00 | 500.00 |
| molecular_bond_order | molecular | BO | Bond order | chemistry | molecular::bond_order | f64 | 1 | 1994989.00 | 1000.00 |
| molecular_dipole_moment | molecular | DM | Dipole moment | chemistry | molecular::dipole_moment | f64 | 1 | 1808279.00 | 1000.00 |
| molecular_formal_charge | molecular | FC | Formal charge | chemistry | molecular::formal_charge | f64 | 1 | 1829680.00 | 1000.00 |
| molecular_molar_mass | molecular | MM | Molar mass | chemistry | molecular::molar_mass | f64 | 1 | 2067879.00 | 500.00 |
| nuclear_binding_energy_per_nucleon | nuclear | BEP | Binding energy per nucleon | chemistry | nuclear::binding_energy_per_nucleon | f64 | 1 | 1726649.00 | 1000.00 |
| nuclear_half_life_to_decay_constant | nuclear | HLT | Half life to decay constant | chemistry | nuclear::half_life_to_decay_constant | f64 | 1 | 1671649.00 | 1000.00 |
| nuclear_q_value | nuclear | QV | Q value | chemistry | nuclear::q_value | f64 | 1 | 1667610.00 | 1000.00 |
| organic_degree_of_unsaturation | organic | DOU | Degree of unsaturation | chemistry | organic::degree_of_unsaturation | f64 | 1 | 1646609.00 | 1000.00 |
| organic_hammett_equation | organic | HE | Hammett equation | chemistry | organic::hammett_equation | f64 | 1 | 1823799.00 | 1000.00 |
| organic_partition_coefficient_log_p | organic | PCL | Partition coefficient log p | chemistry | organic::partition_coefficient_log_p | f64 | 1 | 1731699.00 | 1000.00 |
| photochemistry_fret_efficiency | photochemistry | FE | Fret efficiency | chemistry | photochemistry::fret_efficiency | f64 | 1 | 1840690.00 | 1000.00 |
| photochemistry_photon_energy | photochemistry | PE | Photon energy | chemistry | photochemistry::photon_energy | f64 | 1 | 1723029.00 | 1000.00 |
| photochemistry_quantum_yield | photochemistry | QY | Quantum yield | chemistry | photochemistry::quantum_yield | f64 | 1 | 1793759.00 | 1000.00 |
| photochemistry_stern_volmer | photochemistry | SV | Stern volmer | chemistry | photochemistry::stern_volmer | f64 | 1 | 1739479.00 | 1000.00 |
| polymers_carothers_equation | polymers | CE | Carothers equation | chemistry | polymers::carothers_equation | f64 | 1 | 1703179.00 | 1000.00 |
| polymers_flory_huggins_free_energy | polymers | FHF | Flory huggins free energy | chemistry | polymers::flory_huggins_free_energy | f64 | 1 | 1682250.00 | 1000.00 |
| polymers_polydispersity_index | polymers | PI | Polydispersity index | chemistry | polymers::polydispersity_index | f64 | 1 | 1903169.00 | 1000.00 |
| quantum_chem_huckel_total_pi_energy | quantum_chem | HTP | Huckel total pi energy | chemistry | quantum_chem::huckel_total_pi_energy | f64 | 1 | 1971249.00 | 1000.00 |
| quantum_chem_lcao_bonding_energy | quantum_chem | LBE | Lcao bonding energy | chemistry | quantum_chem::lcao_bonding_energy | f64 | 1 | 1794769.00 | 1000.00 |
| quantum_chem_mulliken_population | quantum_chem | MP | Mulliken population | chemistry | quantum_chem::mulliken_population | f64 | 1 | 1785049.00 | 1000.00 |
| reaction_engineering_cstr_volume | reaction_engineering | CV | Cstr volume | chemistry | reaction_engineering::cstr_volume | f64 | 1 | 2875219.00 | 500.00 |
| reaction_engineering_damkohler_number | reaction_engineering | DN | Damkohler number | chemistry | reaction_engineering::damkohler_number | f64 | 1 | 3150919.00 | 333.33 |
| reaction_engineering_pfr_conversion_first_order | reaction_engineering | PCF | Pfr conversion first order | chemistry | reaction_engineering::pfr_conversion_first_order | f64 | 1 | 2530869.00 | 500.00 |
| solid_state_band_gap_from_absorption | solid_state | BGF | Band gap from absorption | chemistry | solid_state::band_gap_from_absorption | f64 | 1 | 2239929.00 | 500.00 |
| solid_state_conductivity_semiconductor | solid_state | CS | Conductivity semiconductor | chemistry | solid_state::conductivity_semiconductor | f64 | 1 | 2115769.00 | 500.00 |
| solid_state_fermi_dirac | solid_state | FD | Fermi dirac | chemistry | solid_state::fermi_dirac | f64 | 1 | 2119719.00 | 500.00 |
| solutions_boiling_point_elevation | solutions | BPE | Boiling point elevation | chemistry | solutions::boiling_point_elevation | f64 | 1 | 2250509.00 | 500.00 |
| solutions_freezing_point_depression | solutions | FPD | Freezing point depression | chemistry | solutions::freezing_point_depression | f64 | 1 | 2644809.00 | 500.00 |
| solutions_osmotic_pressure | solutions | OP | Osmotic pressure | chemistry | solutions::osmotic_pressure | f64 | 1 | 2594048.00 | 500.00 |
| solutions_raoult_law | solutions | RL | Raoult law | chemistry | solutions::raoult_law | f64 | 1 | 2737569.00 | 500.00 |
| spectroscopy_chemical_shift_ppm | spectroscopy | CSP | Chemical shift ppm | chemistry | spectroscopy::chemical_shift_ppm | f64 | 1 | 2717059.00 | 500.00 |
| spectroscopy_larmor_frequency | spectroscopy | LF | Larmor frequency | chemistry | spectroscopy::larmor_frequency | f64 | 1 | 2713169.00 | 500.00 |
| spectroscopy_reduced_mass | spectroscopy | RM | Reduced mass | chemistry | spectroscopy::reduced_mass | f64 | 1 | 2916349.00 | 500.00 |
| spectroscopy_wavenumber_to_wavelength | spectroscopy | WTW | Wavenumber to wavelength | chemistry | spectroscopy::wavenumber_to_wavelength | f64 | 1 | 3368428.00 | 333.33 |
| stoichiometry_atom_economy | stoichiometry | AE | Atom economy | chemistry | stoichiometry::atom_economy | f64 | 1 | 3047689.00 | 333.33 |
| stoichiometry_limiting_reagent | stoichiometry | LR | Limiting reagent | chemistry | stoichiometry::limiting_reagent | f64 | 1 | 3037618.00 | 333.33 |
| stoichiometry_percent_yield | stoichiometry | PY | Percent yield | chemistry | stoichiometry::percent_yield | f64 | 1 | 3129489.00 | 333.33 |
| stoichiometry_theoretical_yield | stoichiometry | TY | Theoretical yield | chemistry | stoichiometry::theoretical_yield | f64 | 1 | 3511799.00 | 333.33 |
| surface_contact_angle | surface | CA | Contact angle | chemistry | surface::contact_angle | f64 | 1 | 3160308.00 | 333.33 |
| surface_freundlich_isotherm | surface | FI | Freundlich isotherm | chemistry | surface::freundlich_isotherm | f64 | 1 | 3029159.00 | 333.33 |
| surface_langmuir_isotherm | surface | LI | Langmuir isotherm | chemistry | surface::langmuir_isotherm | f64 | 1 | 2892998.00 | 500.00 |
| surface_laplace_pressure | surface | LP | Laplace pressure | chemistry | surface::laplace_pressure | f64 | 1 | 2567289.00 | 500.00 |
| thermochemistry_clausius_clapeyron | thermochemistry | CC | Clausius clapeyron | chemistry | thermochemistry::clausius_clapeyron | f64 | 1 | 2618499.00 | 500.00 |
| thermochemistry_entropy_change | thermochemistry | EC | Entropy change | chemistry | thermochemistry::entropy_change | f64 | 1 | 2235949.00 | 500.00 |
| thermochemistry_gibbs_free_energy | thermochemistry | GFE | Gibbs free energy | chemistry | thermochemistry::gibbs_free_energy | f64 | 1 | 1939139.00 | 1000.00 |
| thermochemistry_hess_law | thermochemistry | HL | Hess law | chemistry | thermochemistry::hess_law | f64 | 1 | 2055189.00 | 500.00 |
| transport_diffusion_coefficient_stokes_einstein | transport | DCS | Diffusion coefficient stokes einstein | chemistry | transport::diffusion_coefficient_stokes_einstein | f64 | 1 | 1845280.00 | 1000.00 |
| transport_fick_first_law | transport | FFL | Fick first law | chemistry | transport::fick_first_law | f64 | 1 | 1939049.00 | 1000.00 |
| transport_sherwood_number | transport | SN | Sherwood number | chemistry | transport::sherwood_number | f64 | 1 | 3494328.00 | 333.33 |
## Analytics
### Category Distribution
| Category | Count | Avg (ns) | Min (ns) | Max (ns) |
|---|---|---|---|---|
| acid_base | 6 | 1887314.12 | 1756239.00 | 2152229.00 |
| analytical | 6 | 2157034.00 | 2003619.00 | 2247389.00 |
| colloids | 3 | 1841192.62 | 1731269.00 | 2029799.00 |
| computational | 3 | 1927799.00 | 1809959.00 | 2034499.00 |
| crystallography | 3 | 1870049.38 | 1782409.00 | 1949829.00 |
| dispatch | 8 | 2022772.88 | 1781359.00 | 2221699.00 |
| electrochemistry | 4 | 1783271.75 | 1731119.00 | 1873560.00 |
| environmental | 3 | 1792182.38 | 1720379.00 | 1902919.00 |
| equilibrium | 3 | 1760116.00 | 1697389.00 | 1804550.00 |
| gas_laws | 8 | 1743633.00 | 1704450.00 | 1790609.00 |
| green_chemistry | 3 | 1699755.62 | 1677509.00 | 1731189.00 |
| inorganic | 3 | 1730262.62 | 1691509.00 | 1807680.00 |
| kinetics | 4 | 2267351.50 | 1750489.00 | 3377648.00 |
| molecular | 4 | 1925206.75 | 1808279.00 | 2067879.00 |
| nuclear | 3 | 1688636.00 | 1667610.00 | 1726649.00 |
| organic | 3 | 1734035.62 | 1646609.00 | 1823799.00 |
| photochemistry | 4 | 1774239.25 | 1723029.00 | 1840690.00 |
| polymers | 3 | 1762866.00 | 1682250.00 | 1903169.00 |
| quantum_chem | 3 | 1850355.62 | 1785049.00 | 1971249.00 |
| reaction_engineering | 3 | 2852335.75 | 2530869.00 | 3150919.00 |
| solid_state | 3 | 2158472.25 | 2115769.00 | 2239929.00 |
| solutions | 4 | 2556733.75 | 2250509.00 | 2737569.00 |
| spectroscopy | 4 | 2928751.25 | 2713169.00 | 3368428.00 |
| stoichiometry | 4 | 3181648.75 | 3037618.00 | 3511799.00 |
| surface | 4 | 2912438.50 | 2567289.00 | 3160308.00 |
| thermochemistry | 4 | 2212194.00 | 1939139.00 | 2618499.00 |
| transport | 3 | 2426219.00 | 1845280.00 | 3494328.00 |

### Top 10 Fastest Entries
| # | Entry | Avg (ns) |
|---|---|---|
| 1 | organic_degree_of_unsaturation | 1646609.00 |
| 2 | nuclear_q_value | 1667610.00 |
| 3 | nuclear_half_life_to_decay_constant | 1671649.00 |
| 4 | green_chemistry_e_factor | 1677509.00 |
| 5 | polymers_flory_huggins_free_energy | 1682250.00 |
| 6 | green_chemistry_reaction_mass_efficiency | 1690569.00 |
| 7 | inorganic_magnetic_moment_spin_only | 1691509.00 |
| 8 | inorganic_pauling_electronegativity_difference | 1691599.00 |
| 9 | equilibrium_vant_hoff | 1697389.00 |
| 10 | polymers_carothers_equation | 1703179.00 |

### Top 10 Slowest Entries
| # | Entry | Avg (ns) |
|---|---|---|
| 1 | stoichiometry_theoretical_yield | 3511799.00 |
| 2 | transport_sherwood_number | 3494328.00 |
| 3 | kinetics_michaelis_menten | 3377648.00 |
| 4 | spectroscopy_wavenumber_to_wavelength | 3368428.00 |
| 5 | surface_contact_angle | 3160308.00 |
| 6 | reaction_engineering_damkohler_number | 3150919.00 |
| 7 | stoichiometry_percent_yield | 3129489.00 |
| 8 | stoichiometry_atom_economy | 3047689.00 |
| 9 | stoichiometry_limiting_reagent | 3037618.00 |
| 10 | surface_freundlich_isotherm | 3029159.00 |

