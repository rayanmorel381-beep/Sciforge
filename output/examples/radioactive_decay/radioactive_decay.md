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
| ²³⁸U | Alpha | ²³⁸U | Uranium | 92 | 238 | 146 | 4.468e9 years | 4.9156e-18 s⁻¹ | alpha | 1.244e4 Bq/g | 125000 / 1M after 3 half-lives | 7.571 MeV/nucleon | f64 | 5000 | 2211.11 | 454545.45 |
| ²³⁵U | Alpha | ²³⁵U | Uranium | 92 | 235 | 143 | 704000000.00 years | 3.1197e-17 s⁻¹ | alpha | 7.995e4 Bq/g | 125000 / 1M after 3 half-lives | 7.565 MeV/nucleon | f64 | 5000 | 2212.51 | 454545.45 |
| ²³²Th | Alpha | ²³²Th | Thorium | 90 | 232 | 142 | 1.405e10 years | 1.5632e-18 s⁻¹ | alpha | 4.058e3 Bq/g | 125000 / 1M after 3 half-lives | 7.570 MeV/nucleon | f64 | 5000 | 2288.26 | 454545.45 |
| ⁴⁰K | Other | ⁴⁰K | Potassium | 19 | 40 | 21 | 1.248e9 years | 1.7598e-17 s⁻¹ | beta- (electron emission) | 2.650e5 Bq/g | 125000 / 1M after 3 half-lives | 7.457 MeV/nucleon | f64 | 5000 | 2253.55 | 454545.45 |
| ¹⁴C | Other | ¹⁴C | Carbon | 6 | 14 | 8 | 5730.00 years | 3.8330e-12 s⁻¹ | beta- (electron emission) | 1.649e11 Bq/g | 125000 / 1M after 3 half-lives | 7.517 MeV/nucleon | f64 | 5000 | 2288.72 | 454545.45 |
| ²²⁶Ra | Other | ²²⁶Ra | Radium | 88 | 226 | 138 | 1600.00 years | 1.3727e-11 s⁻¹ | alpha emission | 3.658e10 Bq/g | 125000 / 1M after 3 half-lives | 7.568 MeV/nucleon | f64 | 5000 | 2248.75 | 454545.45 |
| ⁹⁰Sr | Other | ⁹⁰Sr | Strontium | 38 | 90 | 52 | 28.79 years | 7.6286e-10 s⁻¹ | beta- (electron emission) | 5.105e12 Bq/g | 125000 / 1M after 3 half-lives | 7.525 MeV/nucleon | f64 | 5000 | 2289.75 | 454545.45 |
| ¹³⁷Cs | Other | ¹³⁷Cs | Cesium | 55 | 137 | 82 | 30.17 years | 7.2804e-10 s⁻¹ | beta- (electron emission) | 3.200e12 Bq/g | 125000 / 1M after 3 half-lives | 7.552 MeV/nucleon | f64 | 5000 | 2306.20 | 454545.45 |
| ¹³¹I | Other | ¹³¹I | Iodine | 53 | 131 | 78 | 8.03 days | 9.9967e-7 s⁻¹ | beta- (electron emission) | 4.596e15 Bq/g | 125000 / 1M after 3 half-lives | 7.548 MeV/nucleon | f64 | 5000 | 2275.49 | 454545.45 |
| ⁶⁰Co | Beta | ⁶⁰Co | Cobalt | 27 | 60 | 33 | 5.27 years | 4.1664e-9 s⁻¹ | beta- | 4.182e13 Bq/g | 125000 / 1M after 3 half-lives | 7.489 MeV/nucleon | f64 | 5000 | 2287.79 | 454545.45 |
| ²³⁹Pu | Alpha | ²³⁹Pu | Plutonium | 94 | 239 | 145 | 24110.00 years | 9.1094e-13 s⁻¹ | alpha | 2.295e9 Bq/g | 125000 / 1M after 3 half-lives | 7.563 MeV/nucleon | f64 | 5000 | 2226.71 | 454545.45 |
| ²⁴¹Am | Alpha | ²⁴¹Am | Americium | 95 | 241 | 146 | 432.20 years | 5.0816e-11 s⁻¹ | alpha | 1.270e11 Bq/g | 125000 / 1M after 3 half-lives | 7.561 MeV/nucleon | f64 | 5000 | 2238.90 | 454545.45 |
## Analytics
### Category Distribution
| Category | Count | Avg (ns) | Min (ns) | Max (ns) |
|---|---|---|---|---|
| Alpha | 5 | 2235.50 | 2211.11 | 2288.26 |
| Other | 6 | 2277.08 | 2248.75 | 2306.20 |
| Beta | 1 | 2287.79 | 2287.79 | 2287.79 |

### Top 10 Fastest Entries
| # | Entry | Avg (ns) |
|---|---|---|
| 1 | ²³⁸U | 2211.11 |
| 2 | ²³⁵U | 2212.51 |
| 3 | ²³⁹Pu | 2226.71 |
| 4 | ²⁴¹Am | 2238.90 |
| 5 | ²²⁶Ra | 2248.75 |
| 6 | ⁴⁰K | 2253.55 |
| 7 | ¹³¹I | 2275.49 |
| 8 | ⁶⁰Co | 2287.79 |
| 9 | ²³²Th | 2288.26 |
| 10 | ¹⁴C | 2288.72 |

### Top 10 Slowest Entries
| # | Entry | Avg (ns) |
|---|---|---|
| 1 | ¹³⁷Cs | 2306.20 |
| 2 | ⁹⁰Sr | 2289.75 |
| 3 | ¹⁴C | 2288.72 |
| 4 | ²³²Th | 2288.26 |
| 5 | ⁶⁰Co | 2287.79 |
| 6 | ¹³¹I | 2275.49 |
| 7 | ⁴⁰K | 2253.55 |
| 8 | ²²⁶Ra | 2248.75 |
| 9 | ²⁴¹Am | 2238.90 |
| 10 | ²³⁹Pu | 2226.71 |

