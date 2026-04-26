# Unit Converter — SI, CGS & Common Unit Conversions

15 benchmark entries across 5 formats.

---

## Grid
```
  Length          :  km    m   ly
  Temperature     :  °C    K    K
  Pressure        :  Pa  atm
  Mass            :  kg   kg
  Energy          :  eV    J
  Time            : day  year
  Angle           : deg
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
| Label | category | symbol | name | from | to | conversion | Precision | Iterations | Avg (ns) | Iters/s |
|---|---|---|---|---|---|---|---|---|---|---|
| Marathon distance | Length | km | Marathon distance | 42.195 km | 2.621876e1 mi | 42.195 km → 2.621876e1 mi | f64 | 10000 | 2648.79 | 384615.38 |
| Everest height | Length | m | Everest height | 8848.86 m | 2.903169e4 ft | 8848.86 m → 2.903169e4 ft | f64 | 10000 | 3121.24 | 322580.65 |
| Light-year | Length | ly | Light-year | 1 ly | 9.460730e12 km | 1 ly → 9.460730e12 km | f64 | 10000 | 3035.24 | 333333.33 |
| Human body temp | Temperature | °C | Human body temp | 37 °C | 3.101500e2 K | 37 °C → 3.101500e2 K | f64 | 10000 | 3572.52 | 285714.29 |
| Liquid nitrogen | Temperature | K | Liquid nitrogen | 77.36 K | -1.957900e2 °C | 77.36 K → -1.957900e2 °C | f64 | 10000 | 4050.60 | 250000.00 |
| Sun surface | Temperature | K | Sun surface | 5778 K | 9.940730e3 °F | 5778 K → 9.940730e3 °F | f64 | 10000 | 4939.82 | 204081.63 |
| Atmospheric | Pressure | Pa | Atmospheric | 101325 Pa | 1.000000e0 atm | 101325 Pa → 1.000000e0 atm | f64 | 10000 | 4529.37 | 222222.22 |
| Deep ocean | Pressure | atm | Deep ocean | 1100 atm | 1.114575e8 Pa | 1100 atm → 1.114575e8 Pa | f64 | 10000 | 4173.55 | 243902.44 |
| Earth mass | Mass | kg | Earth mass | 5972000000000000000000000 kg | 1.316601e25 lb | 5972000000000000000000000 kg → 1.316601e25 lb | f64 | 10000 | 3833.50 | 263157.89 |
| Proton mass | Mass | kg | Proton mass | 0.000000000000000000000000001672 kg | 1.006902e0 amu | 0.000000000000000000000000001672 kg → 1.006902e0 amu | f64 | 10000 | 3802.66 | 263157.89 |
| 1 eV | Energy | eV | 1 eV | 1 eV | 1.602177e-19 J | 1 eV → 1.602177e-19 J | f64 | 10000 | 5882.23 | 172413.79 |
| TNT equivalent | Energy | J | TNT equivalent | 4184000000 J | 1.000000e9 cal | 4184000000 J → 1.000000e9 cal | f64 | 10000 | 5189.91 | 196078.43 |
| 1 day | Time | day | 1 day | 1 day | 8.640000e4 s | 1 day → 8.640000e4 s | f64 | 10000 | 5201.60 | 192307.69 |
| 1 year | Time | year | 1 year | 1 year | 8.766000e3 h | 1 year → 8.766000e3 h | f64 | 10000 | 5337.81 | 188679.25 |
| Right angle | Angle | deg | Right angle | 90 deg | 1.570796e0 rad | 90 deg → 1.570796e0 rad | f64 | 10000 | 5389.85 | 188679.25 |
## Analytics
### Category Distribution
| Category | Count | Avg (ns) | Min (ns) | Max (ns) |
|---|---|---|---|---|
| Length | 3 | 2935.09 | 2648.79 | 3121.24 |
| Temperature | 3 | 4187.65 | 3572.52 | 4939.82 |
| Pressure | 2 | 4351.46 | 4173.55 | 4529.37 |
| Mass | 2 | 3818.08 | 3802.66 | 3833.50 |
| Energy | 2 | 5536.07 | 5189.91 | 5882.23 |
| Time | 2 | 5269.71 | 5201.60 | 5337.81 |
| Angle | 1 | 5389.85 | 5389.85 | 5389.85 |

### Top 10 Fastest Entries
| # | Entry | Avg (ns) |
|---|---|---|
| 1 | Marathon distance | 2648.79 |
| 2 | Light-year | 3035.24 |
| 3 | Everest height | 3121.24 |
| 4 | Human body temp | 3572.52 |
| 5 | Proton mass | 3802.66 |
| 6 | Earth mass | 3833.50 |
| 7 | Liquid nitrogen | 4050.60 |
| 8 | Deep ocean | 4173.55 |
| 9 | Atmospheric | 4529.37 |
| 10 | Sun surface | 4939.82 |

### Top 10 Slowest Entries
| # | Entry | Avg (ns) |
|---|---|---|
| 1 | 1 eV | 5882.23 |
| 2 | Right angle | 5389.85 |
| 3 | 1 year | 5337.81 |
| 4 | 1 day | 5201.60 |
| 5 | TNT equivalent | 5189.91 |
| 6 | Sun surface | 4939.82 |
| 7 | Atmospheric | 4529.37 |
| 8 | Deep ocean | 4173.55 |
| 9 | Liquid nitrogen | 4050.60 |
| 10 | Earth mass | 3833.50 |

