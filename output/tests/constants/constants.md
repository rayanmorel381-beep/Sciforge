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
| astro_astronomical_unit | astro | AU | Astronomical unit | constants | astro::astronomical_unit | f64 | 1 | 1118100.00 | 1000.00 |
| astro_cmb_temperature | astro | CT | Cmb temperature | constants | astro::cmb_temperature | f64 | 1 | 1145599.00 | 1000.00 |
| astro_earth_mass | astro | EM | Earth mass | constants | astro::earth_mass | f64 | 1 | 1456760.00 | 1000.00 |
| astro_hubble_constant | astro | HC | Hubble constant | constants | astro::hubble_constant | f64 | 1 | 1305599.00 | 1000.00 |
| astro_light_year | astro | LY | Light year | constants | astro::light_year | f64 | 1 | 1170949.00 | 1000.00 |
| astro_parsec | astro | Pa | Parsec | constants | astro::parsec | f64 | 1 | 1158460.00 | 1000.00 |
| astro_parsec_greater_than_ly | astro | PGT | Parsec greater than ly | constants | astro::parsec_greater_than_ly | f64 | 1 | 1131859.00 | 1000.00 |
| astro_solar_mass | astro | SM | Solar mass | constants | astro::solar_mass | f64 | 1 | 1182260.00 | 1000.00 |
| atomic_amu_to_kg | atomic | ATK | Amu to kg | constants | atomic::amu_to_kg | f64 | 1 | 1236939.00 | 1000.00 |
| atomic_bohr_radius | atomic | BR | Bohr radius | constants | atomic::bohr_radius | f64 | 1 | 1182540.00 | 1000.00 |
| atomic_compton_wavelength | atomic | CW | Compton wavelength | constants | atomic::compton_wavelength | f64 | 1 | 1145809.00 | 1000.00 |
| atomic_electron_mass | atomic | EM | Electron mass | constants | atomic::electron_mass | f64 | 1 | 1307390.00 | 1000.00 |
| atomic_neutron_mass | atomic | NM | Neutron mass | constants | atomic::neutron_mass | f64 | 1 | 1267209.00 | 1000.00 |
| atomic_proton_heavier_than_electron | atomic | PHT | Proton heavier than electron | constants | atomic::proton_heavier_than_electron | f64 | 1 | 1152830.00 | 1000.00 |
| atomic_proton_mass | atomic | PM | Proton mass | constants | atomic::proton_mass | f64 | 1 | 1179549.00 | 1000.00 |
| fundamental_avogadro_number | fundamental | AN | Avogadro number | constants | fundamental::avogadro_number | f64 | 1 | 1168180.00 | 1000.00 |
| fundamental_boltzmann_constant | fundamental | BC | Boltzmann constant | constants | fundamental::boltzmann_constant | f64 | 1 | 1465449.00 | 1000.00 |
| fundamental_coulomb_constant | fundamental | CC | Coulomb constant | constants | fundamental::coulomb_constant | f64 | 1 | 1946559.00 | 1000.00 |
| fundamental_elementary_charge | fundamental | EC | Elementary charge | constants | fundamental::elementary_charge | f64 | 1 | 1517480.00 | 1000.00 |
| fundamental_gas_constant | fundamental | GC | Gas constant | constants | fundamental::gas_constant | f64 | 1 | 1279009.00 | 1000.00 |
| fundamental_gravitational_constant | fundamental | GC | Gravitational constant | constants | fundamental::gravitational_constant | f64 | 1 | 1238629.00 | 1000.00 |
| fundamental_planck_constant | fundamental | PC | Planck constant | constants | fundamental::planck_constant | f64 | 1 | 1089840.00 | 1000.00 |
| fundamental_reduced_planck | fundamental | RP | Reduced planck | constants | fundamental::reduced_planck | f64 | 1 | 1104560.00 | 1000.00 |
| fundamental_speed_of_light | fundamental | SOL | Speed of light | constants | fundamental::speed_of_light | f64 | 1 | 1138969.00 | 1000.00 |
| fundamental_stefan_boltzmann | fundamental | SB | Stefan boltzmann | constants | fundamental::stefan_boltzmann | f64 | 1 | 1350439.00 | 1000.00 |
| units_angle_conversion_inverse_identity | units | ACI | Angle conversion inverse identity | constants | units::angle_conversion_inverse_identity | f64 | 1 | 1573000.00 | 1000.00 |
| units_atm_to_pascal | units | ATP | Atm to pascal | constants | units::atm_to_pascal | f64 | 1 | 1264949.00 | 1000.00 |
| units_calorie_to_joule | units | CTJ | Calorie to joule | constants | units::calorie_to_joule | f64 | 1 | 1239630.00 | 1000.00 |
| units_degree_rad_roundtrip | units | DRR | Degree rad roundtrip | constants | units::degree_rad_roundtrip | f64 | 1 | 1187439.00 | 1000.00 |
| units_ev_joule_roundtrip_multiple_values | units | EJR | Ev joule roundtrip multiple values | constants | units::ev_joule_roundtrip_multiple_values | f64 | 1 | 1198329.00 | 1000.00 |
| units_ev_roundtrip | units | ER | Ev roundtrip | constants | units::ev_roundtrip | f64 | 1 | 1301330.00 | 1000.00 |
| units_ev_to_joule | units | ETJ | Ev to joule | constants | units::ev_to_joule | f64 | 1 | 1208949.00 | 1000.00 |
## Analytics
### Category Distribution
| Category | Count | Avg (ns) | Min (ns) | Max (ns) |
|---|---|---|---|---|
| astro | 8 | 1208698.25 | 1118100.00 | 1456760.00 |
| atomic | 7 | 1210323.75 | 1145809.00 | 1307390.00 |
| fundamental | 10 | 1329911.38 | 1089840.00 | 1946559.00 |
| units | 7 | 1281946.62 | 1187439.00 | 1573000.00 |

### Top 10 Fastest Entries
| # | Entry | Avg (ns) |
|---|---|---|
| 1 | fundamental_planck_constant | 1089840.00 |
| 2 | fundamental_reduced_planck | 1104560.00 |
| 3 | astro_astronomical_unit | 1118100.00 |
| 4 | astro_parsec_greater_than_ly | 1131859.00 |
| 5 | fundamental_speed_of_light | 1138969.00 |
| 6 | astro_cmb_temperature | 1145599.00 |
| 7 | atomic_compton_wavelength | 1145809.00 |
| 8 | atomic_proton_heavier_than_electron | 1152830.00 |
| 9 | astro_parsec | 1158460.00 |
| 10 | fundamental_avogadro_number | 1168180.00 |

### Top 10 Slowest Entries
| # | Entry | Avg (ns) |
|---|---|---|
| 1 | fundamental_coulomb_constant | 1946559.00 |
| 2 | units_angle_conversion_inverse_identity | 1573000.00 |
| 3 | fundamental_elementary_charge | 1517480.00 |
| 4 | fundamental_boltzmann_constant | 1465449.00 |
| 5 | astro_earth_mass | 1456760.00 |
| 6 | fundamental_stefan_boltzmann | 1350439.00 |
| 7 | atomic_electron_mass | 1307390.00 |
| 8 | astro_hubble_constant | 1305599.00 |
| 9 | units_ev_roundtrip | 1301330.00 |
| 10 | fundamental_gas_constant | 1279009.00 |

