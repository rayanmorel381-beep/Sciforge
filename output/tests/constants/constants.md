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
| astro_astronomical_unit | astro | AU | Astronomical unit | constants | astro::astronomical_unit | f64 | 1 | 1146710.00 | 1000.00 |
| astro_cmb_temperature | astro | CT | Cmb temperature | constants | astro::cmb_temperature | f64 | 1 | 1246740.00 | 1000.00 |
| astro_earth_mass | astro | EM | Earth mass | constants | astro::earth_mass | f64 | 1 | 1515101.00 | 1000.00 |
| astro_hubble_constant | astro | HC | Hubble constant | constants | astro::hubble_constant | f64 | 1 | 1216360.00 | 1000.00 |
| astro_light_year | astro | LY | Light year | constants | astro::light_year | f64 | 1 | 1353870.00 | 1000.00 |
| astro_parsec | astro | Pa | Parsec | constants | astro::parsec | f64 | 1 | 1209980.00 | 1000.00 |
| astro_parsec_greater_than_ly | astro | PGT | Parsec greater than ly | constants | astro::parsec_greater_than_ly | f64 | 1 | 1107100.00 | 1000.00 |
| astro_solar_mass | astro | SM | Solar mass | constants | astro::solar_mass | f64 | 1 | 1160531.00 | 1000.00 |
| atomic_amu_to_kg | atomic | ATK | Amu to kg | constants | atomic::amu_to_kg | f64 | 1 | 1076210.00 | 1000.00 |
| atomic_bohr_radius | atomic | BR | Bohr radius | constants | atomic::bohr_radius | f64 | 1 | 1159610.00 | 1000.00 |
| atomic_compton_wavelength | atomic | CW | Compton wavelength | constants | atomic::compton_wavelength | f64 | 1 | 1394240.00 | 1000.00 |
| atomic_electron_mass | atomic | EM | Electron mass | constants | atomic::electron_mass | f64 | 1 | 1432570.00 | 1000.00 |
| atomic_neutron_mass | atomic | NM | Neutron mass | constants | atomic::neutron_mass | f64 | 1 | 1603471.00 | 1000.00 |
| atomic_proton_heavier_than_electron | atomic | PHT | Proton heavier than electron | constants | atomic::proton_heavier_than_electron | f64 | 1 | 1438830.00 | 1000.00 |
| atomic_proton_mass | atomic | PM | Proton mass | constants | atomic::proton_mass | f64 | 1 | 1443590.00 | 1000.00 |
| fundamental_avogadro_number | fundamental | AN | Avogadro number | constants | fundamental::avogadro_number | f64 | 1 | 1448910.00 | 1000.00 |
| fundamental_boltzmann_constant | fundamental | BC | Boltzmann constant | constants | fundamental::boltzmann_constant | f64 | 1 | 1434001.00 | 1000.00 |
| fundamental_coulomb_constant | fundamental | CC | Coulomb constant | constants | fundamental::coulomb_constant | f64 | 1 | 1166330.00 | 1000.00 |
| fundamental_elementary_charge | fundamental | EC | Elementary charge | constants | fundamental::elementary_charge | f64 | 1 | 1172310.00 | 1000.00 |
| fundamental_gas_constant | fundamental | GC | Gas constant | constants | fundamental::gas_constant | f64 | 1 | 1214380.00 | 1000.00 |
| fundamental_gravitational_constant | fundamental | GC | Gravitational constant | constants | fundamental::gravitational_constant | f64 | 1 | 1213580.00 | 1000.00 |
| fundamental_planck_constant | fundamental | PC | Planck constant | constants | fundamental::planck_constant | f64 | 1 | 1129171.00 | 1000.00 |
| fundamental_reduced_planck | fundamental | RP | Reduced planck | constants | fundamental::reduced_planck | f64 | 1 | 1144010.00 | 1000.00 |
| fundamental_speed_of_light | fundamental | SOL | Speed of light | constants | fundamental::speed_of_light | f64 | 1 | 1440350.00 | 1000.00 |
| fundamental_stefan_boltzmann | fundamental | SB | Stefan boltzmann | constants | fundamental::stefan_boltzmann | f64 | 1 | 1228190.00 | 1000.00 |
| units_angle_conversion_inverse_identity | units | ACI | Angle conversion inverse identity | constants | units::angle_conversion_inverse_identity | f64 | 1 | 1161750.00 | 1000.00 |
| units_atm_to_pascal | units | ATP | Atm to pascal | constants | units::atm_to_pascal | f64 | 1 | 1198811.00 | 1000.00 |
| units_calorie_to_joule | units | CTJ | Calorie to joule | constants | units::calorie_to_joule | f64 | 1 | 1218290.00 | 1000.00 |
| units_degree_rad_roundtrip | units | DRR | Degree rad roundtrip | constants | units::degree_rad_roundtrip | f64 | 1 | 1149830.00 | 1000.00 |
| units_ev_joule_roundtrip_multiple_values | units | EJR | Ev joule roundtrip multiple values | constants | units::ev_joule_roundtrip_multiple_values | f64 | 1 | 1199350.00 | 1000.00 |
| units_ev_roundtrip | units | ER | Ev roundtrip | constants | units::ev_roundtrip | f64 | 1 | 1147751.00 | 1000.00 |
| units_ev_to_joule | units | ETJ | Ev to joule | constants | units::ev_to_joule | f64 | 1 | 1310130.00 | 1000.00 |
## Analytics
### Category Distribution
| Category | Count | Avg (ns) | Min (ns) | Max (ns) |
|---|---|---|---|---|
| astro | 8 | 1244549.00 | 1107100.00 | 1515101.00 |
| atomic | 7 | 1364074.43 | 1076210.00 | 1603471.00 |
| fundamental | 10 | 1259123.20 | 1129171.00 | 1448910.00 |
| units | 7 | 1197987.43 | 1147751.00 | 1310130.00 |

### Top 10 Fastest Entries
| # | Entry | Avg (ns) |
|---|---|---|
| 1 | atomic_amu_to_kg | 1076210.00 |
| 2 | astro_parsec_greater_than_ly | 1107100.00 |
| 3 | fundamental_planck_constant | 1129171.00 |
| 4 | fundamental_reduced_planck | 1144010.00 |
| 5 | astro_astronomical_unit | 1146710.00 |
| 6 | units_ev_roundtrip | 1147751.00 |
| 7 | units_degree_rad_roundtrip | 1149830.00 |
| 8 | atomic_bohr_radius | 1159610.00 |
| 9 | astro_solar_mass | 1160531.00 |
| 10 | units_angle_conversion_inverse_identity | 1161750.00 |

### Top 10 Slowest Entries
| # | Entry | Avg (ns) |
|---|---|---|
| 1 | atomic_neutron_mass | 1603471.00 |
| 2 | astro_earth_mass | 1515101.00 |
| 3 | fundamental_avogadro_number | 1448910.00 |
| 4 | atomic_proton_mass | 1443590.00 |
| 5 | fundamental_speed_of_light | 1440350.00 |
| 6 | atomic_proton_heavier_than_electron | 1438830.00 |
| 7 | fundamental_boltzmann_constant | 1434001.00 |
| 8 | atomic_electron_mass | 1432570.00 |
| 9 | atomic_compton_wavelength | 1394240.00 |
| 10 | astro_light_year | 1353870.00 |

