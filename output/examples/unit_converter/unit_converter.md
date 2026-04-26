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
| Marathon distance | Length | km | Marathon distance | 42.195 km | 2.621876e1 mi | 42.195 km → 2.621876e1 mi | f64 | 10000 | 2419.22 | 416666.66 |
| Everest height | Length | m | Everest height | 8848.86 m | 2.903169e4 ft | 8848.86 m → 2.903169e4 ft | f64 | 10000 | 2472.56 | 416666.66 |
| Light-year | Length | ly | Light-year | 1 ly | 9.460730e12 km | 1 ly → 9.460730e12 km | f64 | 10000 | 2447.65 | 416666.66 |
| Human body temp | Temperature | °C | Human body temp | 37 °C | 3.101500e2 K | 37 °C → 3.101500e2 K | f64 | 10000 | 3439.03 | 294117.66 |
| Liquid nitrogen | Temperature | K | Liquid nitrogen | 77.36 K | -1.957900e2 °C | 77.36 K → -1.957900e2 °C | f64 | 10000 | 3463.80 | 294117.66 |
| Sun surface | Temperature | K | Sun surface | 5778 K | 9.940730e3 °F | 5778 K → 9.940730e3 °F | f64 | 10000 | 3407.44 | 294117.66 |
| Atmospheric | Pressure | Pa | Atmospheric | 101325 Pa | 1.000000e0 atm | 101325 Pa → 1.000000e0 atm | f64 | 10000 | 3394.73 | 303030.31 |
| Deep ocean | Pressure | atm | Deep ocean | 1100 atm | 1.114575e8 Pa | 1100 atm → 1.114575e8 Pa | f64 | 10000 | 3402.82 | 294117.66 |
| Earth mass | Mass | kg | Earth mass | 5972000000000000000000000 kg | 1.316601e25 lb | 5972000000000000000000000 kg → 1.316601e25 lb | f64 | 10000 | 3899.77 | 263157.91 |
| Proton mass | Mass | kg | Proton mass | 0.000000000000000000000000001672 kg | 1.006902e0 amu | 0.000000000000000000000000001672 kg → 1.006902e0 amu | f64 | 10000 | 3953.62 | 256410.25 |
| 1 eV | Energy | eV | 1 eV | 1 eV | 1.602177e-19 J | 1 eV → 1.602177e-19 J | f64 | 10000 | 4721.06 | 212765.95 |
| TNT equivalent | Energy | J | TNT equivalent | 4184000000 J | 1.000000e9 cal | 4184000000 J → 1.000000e9 cal | f64 | 10000 | 4737.55 | 212765.95 |
| 1 day | Time | day | 1 day | 1 day | 8.640000e4 s | 1 day → 8.640000e4 s | f64 | 10000 | 4989.79 | 204081.62 |
| 1 year | Time | year | 1 year | 1 year | 8.766000e3 h | 1 year → 8.766000e3 h | f64 | 10000 | 5427.91 | 185185.19 |
| Right angle | Angle | deg | Right angle | 90 deg | 1.570796e0 rad | 90 deg → 1.570796e0 rad | f64 | 10000 | 5282.32 | 192307.69 |
## Analytics
### Category Distribution
| Category | Count | Avg (ns) | Min (ns) | Max (ns) |
|---|---|---|---|---|
| Length | 3 | 2446.48 | 2419.22 | 2472.56 |
| Temperature | 3 | 3436.75 | 3407.44 | 3463.80 |
| Pressure | 2 | 3398.78 | 3394.73 | 3402.82 |
| Mass | 2 | 3926.69 | 3899.77 | 3953.62 |
| Energy | 2 | 4729.31 | 4721.06 | 4737.55 |
| Time | 2 | 5208.85 | 4989.79 | 5427.91 |
| Angle | 1 | 5282.32 | 5282.32 | 5282.32 |

### Top 10 Fastest Entries
| # | Entry | Avg (ns) |
|---|---|---|
| 1 | Marathon distance | 2419.22 |
| 2 | Light-year | 2447.65 |
| 3 | Everest height | 2472.56 |
| 4 | Atmospheric | 3394.73 |
| 5 | Deep ocean | 3402.82 |
| 6 | Sun surface | 3407.44 |
| 7 | Human body temp | 3439.03 |
| 8 | Liquid nitrogen | 3463.80 |
| 9 | Earth mass | 3899.77 |
| 10 | Proton mass | 3953.62 |

### Top 10 Slowest Entries
| # | Entry | Avg (ns) |
|---|---|---|
| 1 | 1 year | 5427.91 |
| 2 | Right angle | 5282.32 |
| 3 | 1 day | 4989.79 |
| 4 | TNT equivalent | 4737.55 |
| 5 | 1 eV | 4721.06 |
| 6 | Proton mass | 3953.62 |
| 7 | Earth mass | 3899.77 |
| 8 | Liquid nitrogen | 3463.80 |
| 9 | Human body temp | 3439.03 |
| 10 | Sun surface | 3407.44 |

