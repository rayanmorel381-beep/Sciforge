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
| Water synthesis | Synthesis | Water synthesis | 2H₂ + O₂ → 2H₂O | 36.0300 g/mol | 36.0300 g/mol | 100.0% | f64 | 5000 | 19382.79 | 52083.33 |
| Combustion of methane | Combustion | Combustion of methane | CH₄ + 2O₂ → CO₂ + 2H₂O | 80.0390 g/mol | 80.0390 g/mol | 100.0% | f64 | 5000 | 23231.81 | 43103.45 |
| Rust formation | Oxidation | Rust formation | 4Fe + 3O₂ → 2Fe₂O₃ | 319.3740 g/mol | 319.3740 g/mol | 100.0% | f64 | 5000 | 31202.24 | 32051.28 |
| Photosynthesis | Biochemistry | Photosynthesis | 6CO₂ + 6H₂O → C₆H₁₂O₆ + 6O₂ | 372.1440 g/mol | 372.1440 g/mol | 100.0% | f64 | 5000 | 21875.61 | 45871.56 |
| Ammonia synthesis (Haber) | Synthesis | Ammonia synthesis (Haber) | N₂ + 3H₂ → 2NH₃ | 34.0620 g/mol | 34.0620 g/mol | 100.0% | f64 | 5000 | 16018.27 | 62500.00 |
| Sodium chloride | Synthesis | Sodium chloride | 2Na + Cl₂ → 2NaCl | 116.8795 g/mol | 116.8795 g/mol | 100.0% | f64 | 5000 | 29278.17 | 34246.58 |
| Sulfuric acid neutralization | Acid-Base | Sulfuric acid neutralization | H₂SO₄ + 2NaOH → Na₂SO₄ + 2H₂O | 178.0655 g/mol | 178.0655 g/mol | 100.0% | f64 | 5000 | 36130.70 | 27777.78 |
| Thermite reaction | Oxidation | Thermite reaction | 2Al + Fe₂O₃ → Al₂O₃ + 2Fe | 213.6501 g/mol | 213.6501 g/mol | 100.0% | f64 | 5000 | 44899.57 | 22321.43 |
## Analytics
### Category Distribution
| Category | Count | Avg (ns) | Min (ns) | Max (ns) |
|---|---|---|---|---|
| Synthesis | 3 | 21559.74 | 16018.27 | 29278.17 |
| Combustion | 1 | 23231.81 | 23231.81 | 23231.81 |
| Oxidation | 2 | 38050.90 | 31202.24 | 44899.57 |
| Biochemistry | 1 | 21875.61 | 21875.61 | 21875.61 |
| Acid-Base | 1 | 36130.70 | 36130.70 | 36130.70 |

### Top 10 Fastest Entries
| # | Entry | Avg (ns) |
|---|---|---|
| 1 | Ammonia synthesis (Haber) | 16018.27 |
| 2 | Water synthesis | 19382.79 |
| 3 | Photosynthesis | 21875.61 |
| 4 | Combustion of methane | 23231.81 |
| 5 | Sodium chloride | 29278.17 |
| 6 | Rust formation | 31202.24 |
| 7 | Sulfuric acid neutralization | 36130.70 |
| 8 | Thermite reaction | 44899.57 |

### Top 10 Slowest Entries
| # | Entry | Avg (ns) |
|---|---|---|
| 1 | Thermite reaction | 44899.57 |
| 2 | Sulfuric acid neutralization | 36130.70 |
| 3 | Rust formation | 31202.24 |
| 4 | Sodium chloride | 29278.17 |
| 5 | Combustion of methane | 23231.81 |
| 6 | Photosynthesis | 21875.61 |
| 7 | Water synthesis | 19382.79 |
| 8 | Ammonia synthesis (Haber) | 16018.27 |

