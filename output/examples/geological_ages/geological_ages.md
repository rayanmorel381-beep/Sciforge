# Geological Ages — Radiometric Dating with Isotope Half-lives

10 benchmark entries across 5 formats.

---

## Grid
```
  Hadean          :   U    U
  Archean         :   U    K    U
  Proterozoic     :   U    U
  Mesozoic        :   K
  Cenozoic        :   C    C
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
| Label | category | symbol | name | method | parent | daughter | ratio | half_life | decay_constant | computed_age | expected_age | Precision | Iterations | Avg (ns) | Iters/s |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Jack Hills zircon | Hadean | U | Jack Hills zircon | U-Pb (238) | ²³⁸U | ²⁰⁶Pb | 1.0200 | 4.4680e9 years | 4.9156e-18 s⁻¹ | 4532.5 Myr | 4400 Myr | f64 | 5000 | 3817.20 | 263157.91 |
| Acasta Gneiss | Hadean | U | Acasta Gneiss | U-Pb (238) | ²³⁸U | ²⁰⁶Pb | 0.8500 | 4.4680e9 years | 4.9156e-18 s⁻¹ | 3965.7 Myr | 4030 Myr | f64 | 5000 | 3712.41 | 277777.78 |
| Isua Greenstone | Archean | U | Isua Greenstone | U-Pb (235) | ²³⁵U | ²⁰⁷Pb | 0.5800 | 7.0400e8 years | 3.1197e-17 s⁻¹ | 464.5 Myr | 3800 Myr | f64 | 5000 | 4527.30 | 227272.73 |
| Pilbara Craton | Archean | K | Pilbara Craton | K-Ar | ⁴⁰K | ⁴⁰Ar | 0.1100 | 1.2480e9 years | 1.7598e-17 s⁻¹ | 1294.7 Myr | 3500 Myr | f64 | 5000 | 4734.31 | 217391.30 |
| Kaapvaal Craton | Archean | U | Kaapvaal Craton | U-Pb (238) | ²³⁸U | ²⁰⁶Pb | 0.6500 | 4.4680e9 years | 4.9156e-18 s⁻¹ | 3228.2 Myr | 3100 Myr | f64 | 5000 | 3742.12 | 277777.78 |
| Bushveld Complex | Proterozoic | U | Bushveld Complex | U-Pb (238) | ²³⁸U | ²⁰⁶Pb | 0.3200 | 4.4680e9 years | 4.9156e-18 s⁻¹ | 1789.7 Myr | 2055 Myr | f64 | 5000 | 3766.45 | 277777.78 |
| Sudbury Impact | Proterozoic | U | Sudbury Impact | U-Pb (238) | ²³⁸U | ²⁰⁶Pb | 0.2800 | 4.4680e9 years | 4.9156e-18 s⁻¹ | 1591.4 Myr | 1849 Myr | f64 | 5000 | 3745.87 | 277777.78 |
| Deccan Traps | Mesozoic | K | Deccan Traps | K-Ar | ⁴⁰K | ⁴⁰Ar | 0.0050 | 1.2480e9 years | 1.7598e-17 s⁻¹ | 84.1 Myr | 66 Myr | f64 | 5000 | 4590.98 | 227272.73 |
| KT Boundary clay | Cenozoic | C | KT Boundary clay | C-14 | ¹⁴C | ¹⁴N | 0.0000 | 5.7300e3 years | 3.8330e-12 s⁻¹ | beyond range | 65.0 kyr | f64 | 5000 | 4595.33 | 227272.73 |
| Lascaux paintings | Cenozoic | C | Lascaux paintings | C-14 | ¹⁴C | ¹⁴N | 0.8200 | 5.7300e3 years | 3.8330e-12 s⁻¹ | 1641 yr | 17.0 kyr | f64 | 5000 | 4756.22 | 217391.30 |
## Analytics
### Category Distribution
| Category | Count | Avg (ns) | Min (ns) | Max (ns) |
|---|---|---|---|---|
| Hadean | 2 | 3764.81 | 3712.41 | 3817.20 |
| Archean | 3 | 4334.58 | 3742.12 | 4734.31 |
| Proterozoic | 2 | 3756.16 | 3745.87 | 3766.45 |
| Mesozoic | 1 | 4590.98 | 4590.98 | 4590.98 |
| Cenozoic | 2 | 4675.77 | 4595.33 | 4756.22 |

### Top 10 Fastest Entries
| # | Entry | Avg (ns) |
|---|---|---|
| 1 | Acasta Gneiss | 3712.41 |
| 2 | Kaapvaal Craton | 3742.12 |
| 3 | Sudbury Impact | 3745.87 |
| 4 | Bushveld Complex | 3766.45 |
| 5 | Jack Hills zircon | 3817.20 |
| 6 | Isua Greenstone | 4527.30 |
| 7 | Deccan Traps | 4590.98 |
| 8 | KT Boundary clay | 4595.33 |
| 9 | Pilbara Craton | 4734.31 |
| 10 | Lascaux paintings | 4756.22 |

### Top 10 Slowest Entries
| # | Entry | Avg (ns) |
|---|---|---|
| 1 | Lascaux paintings | 4756.22 |
| 2 | Pilbara Craton | 4734.31 |
| 3 | KT Boundary clay | 4595.33 |
| 4 | Deccan Traps | 4590.98 |
| 5 | Isua Greenstone | 4527.30 |
| 6 | Jack Hills zircon | 3817.20 |
| 7 | Bushveld Complex | 3766.45 |
| 8 | Sudbury Impact | 3745.87 |
| 9 | Kaapvaal Craton | 3742.12 |
| 10 | Acasta Gneiss | 3712.41 |

