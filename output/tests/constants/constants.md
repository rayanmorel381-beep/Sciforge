# SciForge Constants Test

32 benchmark entries across 5 formats.

---

## Grid
```
  astro           :  AU   CT   EM   HC   LY   Pa  PGT   SM
  atomic          : ATK   BR   CW   EM   NM  PHT   PM
  fundamental     :  AN   BC   CC   EC   GC   GC   PC   RP  SOL   SB
  units           : ACI  ATP  CTJ  DRR  EJR   ER  ETJ
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
| astro_astronomical_unit | astro | AU | Astronomical unit | constants | astro::astronomical_unit | f64 | 1 | 1388132.00 | 1000.00 |
| astro_cmb_temperature | astro | CT | Cmb temperature | constants | astro::cmb_temperature | f64 | 1 | 1554393.00 | 1000.00 |
| astro_earth_mass | astro | EM | Earth mass | constants | astro::earth_mass | f64 | 1 | 1175502.00 | 1000.00 |
| astro_hubble_constant | astro | HC | Hubble constant | constants | astro::hubble_constant | f64 | 1 | 1208092.00 | 1000.00 |
| astro_light_year | astro | LY | Light year | constants | astro::light_year | f64 | 1 | 1239503.00 | 1000.00 |
| astro_parsec | astro | Pa | Parsec | constants | astro::parsec | f64 | 1 | 1216782.00 | 1000.00 |
| astro_parsec_greater_than_ly | astro | PGT | Parsec greater than ly | constants | astro::parsec_greater_than_ly | f64 | 1 | 1208722.00 | 1000.00 |
| astro_solar_mass | astro | SM | Solar mass | constants | astro::solar_mass | f64 | 1 | 1211393.00 | 1000.00 |
| atomic_amu_to_kg | atomic | ATK | Amu to kg | constants | atomic::amu_to_kg | f64 | 1 | 1113482.00 | 1000.00 |
| atomic_bohr_radius | atomic | BR | Bohr radius | constants | atomic::bohr_radius | f64 | 1 | 1118752.00 | 1000.00 |
| atomic_compton_wavelength | atomic | CW | Compton wavelength | constants | atomic::compton_wavelength | f64 | 1 | 1396462.00 | 1000.00 |
| atomic_electron_mass | atomic | EM | Electron mass | constants | atomic::electron_mass | f64 | 1 | 1224282.00 | 1000.00 |
| atomic_neutron_mass | atomic | NM | Neutron mass | constants | atomic::neutron_mass | f64 | 1 | 1473203.00 | 1000.00 |
| atomic_proton_heavier_than_electron | atomic | PHT | Proton heavier than electron | constants | atomic::proton_heavier_than_electron | f64 | 1 | 1424733.00 | 1000.00 |
| atomic_proton_mass | atomic | PM | Proton mass | constants | atomic::proton_mass | f64 | 1 | 1196832.00 | 1000.00 |
| fundamental_avogadro_number | fundamental | AN | Avogadro number | constants | fundamental::avogadro_number | f64 | 1 | 1094902.00 | 1000.00 |
| fundamental_boltzmann_constant | fundamental | BC | Boltzmann constant | constants | fundamental::boltzmann_constant | f64 | 1 | 1192362.00 | 1000.00 |
| fundamental_coulomb_constant | fundamental | CC | Coulomb constant | constants | fundamental::coulomb_constant | f64 | 1 | 1514273.00 | 1000.00 |
| fundamental_elementary_charge | fundamental | EC | Elementary charge | constants | fundamental::elementary_charge | f64 | 1 | 1190132.00 | 1000.00 |
| fundamental_gas_constant | fundamental | GC | Gas constant | constants | fundamental::gas_constant | f64 | 1 | 1230993.00 | 1000.00 |
| fundamental_gravitational_constant | fundamental | GC | Gravitational constant | constants | fundamental::gravitational_constant | f64 | 1 | 1229832.00 | 1000.00 |
| fundamental_planck_constant | fundamental | PC | Planck constant | constants | fundamental::planck_constant | f64 | 1 | 1332032.00 | 1000.00 |
| fundamental_reduced_planck | fundamental | RP | Reduced planck | constants | fundamental::reduced_planck | f64 | 1 | 1190632.00 | 1000.00 |
| fundamental_speed_of_light | fundamental | SOL | Speed of light | constants | fundamental::speed_of_light | f64 | 1 | 1144123.00 | 1000.00 |
| fundamental_stefan_boltzmann | fundamental | SB | Stefan boltzmann | constants | fundamental::stefan_boltzmann | f64 | 1 | 1280612.00 | 1000.00 |
| units_angle_conversion_inverse_identity | units | ACI | Angle conversion inverse identity | constants | units::angle_conversion_inverse_identity | f64 | 1 | 1356603.00 | 1000.00 |
| units_atm_to_pascal | units | ATP | Atm to pascal | constants | units::atm_to_pascal | f64 | 1 | 1291612.00 | 1000.00 |
| units_calorie_to_joule | units | CTJ | Calorie to joule | constants | units::calorie_to_joule | f64 | 1 | 1357812.00 | 1000.00 |
| units_degree_rad_roundtrip | units | DRR | Degree rad roundtrip | constants | units::degree_rad_roundtrip | f64 | 1 | 1487873.00 | 1000.00 |
| units_ev_joule_roundtrip_multiple_values | units | EJR | Ev joule roundtrip multiple values | constants | units::ev_joule_roundtrip_multiple_values | f64 | 1 | 1508983.00 | 1000.00 |
| units_ev_roundtrip | units | ER | Ev roundtrip | constants | units::ev_roundtrip | f64 | 1 | 1256382.00 | 1000.00 |
| units_ev_to_joule | units | ETJ | Ev to joule | constants | units::ev_to_joule | f64 | 1 | 1195943.00 | 1000.00 |
## Analytics
### Category Distribution
| Category | Count | Avg (ns) | Min (ns) | Max (ns) |
|---|---|---|---|---|
| astro | 8 | 1275314.88 | 1175502.00 | 1554393.00 |
| atomic | 7 | 1278249.43 | 1113482.00 | 1473203.00 |
| fundamental | 10 | 1239989.30 | 1094902.00 | 1514273.00 |
| units | 7 | 1350744.00 | 1195943.00 | 1508983.00 |

### Top 10 Fastest Entries
| # | Entry | Avg (ns) |
|---|---|---|
| 1 | fundamental_avogadro_number | 1094902.00 |
| 2 | atomic_amu_to_kg | 1113482.00 |
| 3 | atomic_bohr_radius | 1118752.00 |
| 4 | fundamental_speed_of_light | 1144123.00 |
| 5 | astro_earth_mass | 1175502.00 |
| 6 | fundamental_elementary_charge | 1190132.00 |
| 7 | fundamental_reduced_planck | 1190632.00 |
| 8 | fundamental_boltzmann_constant | 1192362.00 |
| 9 | units_ev_to_joule | 1195943.00 |
| 10 | atomic_proton_mass | 1196832.00 |

### Top 10 Slowest Entries
| # | Entry | Avg (ns) |
|---|---|---|
| 1 | astro_cmb_temperature | 1554393.00 |
| 2 | fundamental_coulomb_constant | 1514273.00 |
| 3 | units_ev_joule_roundtrip_multiple_values | 1508983.00 |
| 4 | units_degree_rad_roundtrip | 1487873.00 |
| 5 | atomic_neutron_mass | 1473203.00 |
| 6 | atomic_proton_heavier_than_electron | 1424733.00 |
| 7 | atomic_compton_wavelength | 1396462.00 |
| 8 | astro_astronomical_unit | 1388132.00 |
| 9 | units_calorie_to_joule | 1357812.00 |
| 10 | units_angle_conversion_inverse_identity | 1356603.00 |

