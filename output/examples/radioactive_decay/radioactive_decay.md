# Radioactive Decay — Chains, Half-lives & Activities from Isotope Data

12 benchmark entries across 5 formats.

---

## Grid
```
  Alpha           : ²³⁸U  ²³⁵U  ²³²Th  ²³⁹Pu  ²⁴¹Am
  Other           : ⁴⁰K  ¹⁴C  ²²⁶Ra  ⁹⁰Sr  ¹³⁷Cs  ¹³¹I
  Beta            : ⁶⁰Co
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
| Label | category | symbol | name | Z | A | N | half_life | decay_constant | decay_mode | specific_activity | remaining_3hl | binding_energy | Precision | Iterations | Avg (ns) | Iters/s |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| ²³⁸U | Alpha | ²³⁸U | Uranium | 92 | 238 | 146 | 4.468e9 years | 4.9156e-18 s⁻¹ | alpha | 1.244e4 Bq/g | 125000 / 1M after 3 half-lives | 7.571 MeV/nucleon | f64 | 5000 | 2226.82 | 454545.47 |
| ²³⁵U | Alpha | ²³⁵U | Uranium | 92 | 235 | 143 | 704000000.00 years | 3.1197e-17 s⁻¹ | alpha | 7.995e4 Bq/g | 125000 / 1M after 3 half-lives | 7.565 MeV/nucleon | f64 | 5000 | 2250.62 | 454545.47 |
| ²³²Th | Alpha | ²³²Th | Thorium | 90 | 232 | 142 | 1.405e10 years | 1.5632e-18 s⁻¹ | alpha | 4.058e3 Bq/g | 125000 / 1M after 3 half-lives | 7.570 MeV/nucleon | f64 | 5000 | 2421.41 | 416666.66 |
| ⁴⁰K | Other | ⁴⁰K | Potassium | 19 | 40 | 21 | 1.248e9 years | 1.7598e-17 s⁻¹ | beta- (electron emission) | 2.650e5 Bq/g | 125000 / 1M after 3 half-lives | 7.457 MeV/nucleon | f64 | 5000 | 2266.50 | 454545.47 |
| ¹⁴C | Other | ¹⁴C | Carbon | 6 | 14 | 8 | 5730.00 years | 3.8330e-12 s⁻¹ | beta- (electron emission) | 1.649e11 Bq/g | 125000 / 1M after 3 half-lives | 7.517 MeV/nucleon | f64 | 5000 | 2285.18 | 454545.47 |
| ²²⁶Ra | Other | ²²⁶Ra | Radium | 88 | 226 | 138 | 1600.00 years | 1.3727e-11 s⁻¹ | alpha emission | 3.658e10 Bq/g | 125000 / 1M after 3 half-lives | 7.568 MeV/nucleon | f64 | 5000 | 2268.76 | 454545.47 |
| ⁹⁰Sr | Other | ⁹⁰Sr | Strontium | 38 | 90 | 52 | 28.79 years | 7.6286e-10 s⁻¹ | beta- (electron emission) | 5.105e12 Bq/g | 125000 / 1M after 3 half-lives | 7.525 MeV/nucleon | f64 | 5000 | 2214.38 | 454545.47 |
| ¹³⁷Cs | Other | ¹³⁷Cs | Cesium | 55 | 137 | 82 | 30.17 years | 7.2804e-10 s⁻¹ | beta- (electron emission) | 3.200e12 Bq/g | 125000 / 1M after 3 half-lives | 7.552 MeV/nucleon | f64 | 5000 | 2254.30 | 454545.47 |
| ¹³¹I | Other | ¹³¹I | Iodine | 53 | 131 | 78 | 8.03 days | 9.9967e-7 s⁻¹ | beta- (electron emission) | 4.596e15 Bq/g | 125000 / 1M after 3 half-lives | 7.548 MeV/nucleon | f64 | 5000 | 2220.21 | 454545.47 |
| ⁶⁰Co | Beta | ⁶⁰Co | Cobalt | 27 | 60 | 33 | 5.27 years | 4.1664e-9 s⁻¹ | beta- | 4.182e13 Bq/g | 125000 / 1M after 3 half-lives | 7.489 MeV/nucleon | f64 | 5000 | 2333.16 | 454545.47 |
| ²³⁹Pu | Alpha | ²³⁹Pu | Plutonium | 94 | 239 | 145 | 24110.00 years | 9.1094e-13 s⁻¹ | alpha | 2.295e9 Bq/g | 125000 / 1M after 3 half-lives | 7.563 MeV/nucleon | f64 | 5000 | 2407.81 | 416666.66 |
| ²⁴¹Am | Alpha | ²⁴¹Am | Americium | 95 | 241 | 146 | 432.20 years | 5.0816e-11 s⁻¹ | alpha | 1.270e11 Bq/g | 125000 / 1M after 3 half-lives | 7.561 MeV/nucleon | f64 | 5000 | 2720.78 | 384615.38 |
## Analytics
### Category Distribution
| Category | Count | Avg (ns) | Min (ns) | Max (ns) |
|---|---|---|---|---|
| Alpha | 5 | 2405.49 | 2226.82 | 2720.78 |
| Other | 6 | 2251.56 | 2214.38 | 2285.18 |
| Beta | 1 | 2333.16 | 2333.16 | 2333.16 |

### Top 10 Fastest Entries
| # | Entry | Avg (ns) |
|---|---|---|
| 1 | ⁹⁰Sr | 2214.38 |
| 2 | ¹³¹I | 2220.21 |
| 3 | ²³⁸U | 2226.82 |
| 4 | ²³⁵U | 2250.62 |
| 5 | ¹³⁷Cs | 2254.30 |
| 6 | ⁴⁰K | 2266.50 |
| 7 | ²²⁶Ra | 2268.76 |
| 8 | ¹⁴C | 2285.18 |
| 9 | ⁶⁰Co | 2333.16 |
| 10 | ²³⁹Pu | 2407.81 |

### Top 10 Slowest Entries
| # | Entry | Avg (ns) |
|---|---|---|
| 1 | ²⁴¹Am | 2720.78 |
| 2 | ²³²Th | 2421.41 |
| 3 | ²³⁹Pu | 2407.81 |
| 4 | ⁶⁰Co | 2333.16 |
| 5 | ¹⁴C | 2285.18 |
| 6 | ²²⁶Ra | 2268.76 |
| 7 | ⁴⁰K | 2266.50 |
| 8 | ¹³⁷Cs | 2254.30 |
| 9 | ²³⁵U | 2250.62 |
| 10 | ²³⁸U | 2226.82 |

