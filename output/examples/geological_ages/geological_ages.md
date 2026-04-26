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
| Jack Hills zircon | Hadean | U | Jack Hills zircon | U-Pb (238) | ²³⁸U | ²⁰⁶Pb | 1.0200 | 4.4680e9 years | 4.9156e-18 s⁻¹ | 4532.5 Myr | 4400 Myr | f64 | 5000 | 3713.78 | 277777.78 |
| Acasta Gneiss | Hadean | U | Acasta Gneiss | U-Pb (238) | ²³⁸U | ²⁰⁶Pb | 0.8500 | 4.4680e9 years | 4.9156e-18 s⁻¹ | 3965.7 Myr | 4030 Myr | f64 | 5000 | 3755.43 | 277777.78 |
| Isua Greenstone | Archean | U | Isua Greenstone | U-Pb (235) | ²³⁵U | ²⁰⁷Pb | 0.5800 | 7.0400e8 years | 3.1197e-17 s⁻¹ | 464.5 Myr | 3800 Myr | f64 | 5000 | 4803.45 | 208333.33 |
| Pilbara Craton | Archean | K | Pilbara Craton | K-Ar | ⁴⁰K | ⁴⁰Ar | 0.1100 | 1.2480e9 years | 1.7598e-17 s⁻¹ | 1294.7 Myr | 3500 Myr | f64 | 5000 | 5376.54 | 192307.69 |
| Kaapvaal Craton | Archean | U | Kaapvaal Craton | U-Pb (238) | ²³⁸U | ²⁰⁶Pb | 0.6500 | 4.4680e9 years | 4.9156e-18 s⁻¹ | 3228.2 Myr | 3100 Myr | f64 | 5000 | 3999.39 | 263157.89 |
| Bushveld Complex | Proterozoic | U | Bushveld Complex | U-Pb (238) | ²³⁸U | ²⁰⁶Pb | 0.3200 | 4.4680e9 years | 4.9156e-18 s⁻¹ | 1789.7 Myr | 2055 Myr | f64 | 5000 | 3987.71 | 263157.89 |
| Sudbury Impact | Proterozoic | U | Sudbury Impact | U-Pb (238) | ²³⁸U | ²⁰⁶Pb | 0.2800 | 4.4680e9 years | 4.9156e-18 s⁻¹ | 1591.4 Myr | 1849 Myr | f64 | 5000 | 3884.42 | 263157.89 |
| Deccan Traps | Mesozoic | K | Deccan Traps | K-Ar | ⁴⁰K | ⁴⁰Ar | 0.0050 | 1.2480e9 years | 1.7598e-17 s⁻¹ | 84.1 Myr | 66 Myr | f64 | 5000 | 4827.58 | 208333.33 |
| KT Boundary clay | Cenozoic | C | KT Boundary clay | C-14 | ¹⁴C | ¹⁴N | 0.0000 | 5.7300e3 years | 3.8330e-12 s⁻¹ | beyond range | 65.0 kyr | f64 | 5000 | 4837.50 | 208333.33 |
| Lascaux paintings | Cenozoic | C | Lascaux paintings | C-14 | ¹⁴C | ¹⁴N | 0.8200 | 5.7300e3 years | 3.8330e-12 s⁻¹ | 1641 yr | 17.0 kyr | f64 | 5000 | 4742.27 | 217391.30 |
## Analytics
### Category Distribution
| Category | Count | Avg (ns) | Min (ns) | Max (ns) |
|---|---|---|---|---|
| Hadean | 2 | 3734.61 | 3713.78 | 3755.43 |
| Archean | 3 | 4726.46 | 3999.39 | 5376.54 |
| Proterozoic | 2 | 3936.07 | 3884.42 | 3987.71 |
| Mesozoic | 1 | 4827.58 | 4827.58 | 4827.58 |
| Cenozoic | 2 | 4789.88 | 4742.27 | 4837.50 |

### Top 10 Fastest Entries
| # | Entry | Avg (ns) |
|---|---|---|
| 1 | Jack Hills zircon | 3713.78 |
| 2 | Acasta Gneiss | 3755.43 |
| 3 | Sudbury Impact | 3884.42 |
| 4 | Bushveld Complex | 3987.71 |
| 5 | Kaapvaal Craton | 3999.39 |
| 6 | Lascaux paintings | 4742.27 |
| 7 | Isua Greenstone | 4803.45 |
| 8 | Deccan Traps | 4827.58 |
| 9 | KT Boundary clay | 4837.50 |
| 10 | Pilbara Craton | 5376.54 |

### Top 10 Slowest Entries
| # | Entry | Avg (ns) |
|---|---|---|
| 1 | Pilbara Craton | 5376.54 |
| 2 | KT Boundary clay | 4837.50 |
| 3 | Deccan Traps | 4827.58 |
| 4 | Isua Greenstone | 4803.45 |
| 5 | Lascaux paintings | 4742.27 |
| 6 | Kaapvaal Craton | 3999.39 |
| 7 | Bushveld Complex | 3987.71 |
| 8 | Sudbury Impact | 3884.42 |
| 9 | Acasta Gneiss | 3755.43 |
| 10 | Jack Hills zircon | 3713.78 |

