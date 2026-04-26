# Chemical Reactions — Molar Masses & Atom Economy

8 benchmark entries across 5 formats.

---

## Grid
```
  Synthesis       :   ?    ?    ?
  Combustion      :   ?
  Oxidation       :   ?    ?
  Biochemistry    :   ?
  Acid-Base       :   ?
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
| Label | category | name | formula | reactant_mass | product_mass | atom_economy | Precision | Iterations | Avg (ns) | Iters/s |
|---|---|---|---|---|---|---|---|---|---|---|
| Water synthesis | Synthesis | Water synthesis | 2H₂ + O₂ → 2H₂O | 36.0300 g/mol | 36.0300 g/mol | 100.0% | f64 | 5000 | 18921.46 | 53191.49 |
| Combustion of methane | Combustion | Combustion of methane | CH₄ + 2O₂ → CO₂ + 2H₂O | 80.0390 g/mol | 80.0390 g/mol | 100.0% | f64 | 5000 | 23088.09 | 43478.26 |
| Rust formation | Oxidation | Rust formation | 4Fe + 3O₂ → 2Fe₂O₃ | 319.3740 g/mol | 319.3740 g/mol | 100.0% | f64 | 5000 | 33188.98 | 30303.03 |
| Photosynthesis | Biochemistry | Photosynthesis | 6CO₂ + 6H₂O → C₆H₁₂O₆ + 6O₂ | 372.1440 g/mol | 372.1440 g/mol | 100.0% | f64 | 5000 | 23809.53 | 42016.81 |
| Ammonia synthesis (Haber) | Synthesis | Ammonia synthesis (Haber) | N₂ + 3H₂ → 2NH₃ | 34.0620 g/mol | 34.0620 g/mol | 100.0% | f64 | 5000 | 17668.62 | 56818.18 |
| Sodium chloride | Synthesis | Sodium chloride | 2Na + Cl₂ → 2NaCl | 116.8795 g/mol | 116.8795 g/mol | 100.0% | f64 | 5000 | 31005.74 | 32258.06 |
| Sulfuric acid neutralization | Acid-Base | Sulfuric acid neutralization | H₂SO₄ + 2NaOH → Na₂SO₄ + 2H₂O | 178.0655 g/mol | 178.0655 g/mol | 100.0% | f64 | 5000 | 36085.16 | 27777.78 |
| Thermite reaction | Oxidation | Thermite reaction | 2Al + Fe₂O₃ → Al₂O₃ + 2Fe | 213.6501 g/mol | 213.6501 g/mol | 100.0% | f64 | 5000 | 44191.38 | 22727.27 |
## Analytics
### Category Distribution
| Category | Count | Avg (ns) | Min (ns) | Max (ns) |
|---|---|---|---|---|
| Synthesis | 3 | 22531.94 | 17668.62 | 31005.74 |
| Combustion | 1 | 23088.09 | 23088.09 | 23088.09 |
| Oxidation | 2 | 38690.18 | 33188.98 | 44191.38 |
| Biochemistry | 1 | 23809.53 | 23809.53 | 23809.53 |
| Acid-Base | 1 | 36085.16 | 36085.16 | 36085.16 |

### Top 10 Fastest Entries
| # | Entry | Avg (ns) |
|---|---|---|
| 1 | Ammonia synthesis (Haber) | 17668.62 |
| 2 | Water synthesis | 18921.46 |
| 3 | Combustion of methane | 23088.09 |
| 4 | Photosynthesis | 23809.53 |
| 5 | Sodium chloride | 31005.74 |
| 6 | Rust formation | 33188.98 |
| 7 | Sulfuric acid neutralization | 36085.16 |
| 8 | Thermite reaction | 44191.38 |

### Top 10 Slowest Entries
| # | Entry | Avg (ns) |
|---|---|---|
| 1 | Thermite reaction | 44191.38 |
| 2 | Sulfuric acid neutralization | 36085.16 |
| 3 | Rust formation | 33188.98 |
| 4 | Sodium chloride | 31005.74 |
| 5 | Photosynthesis | 23809.53 |
| 6 | Combustion of methane | 23088.09 |
| 7 | Water synthesis | 18921.46 |
| 8 | Ammonia synthesis (Haber) | 17668.62 |

