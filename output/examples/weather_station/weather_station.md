# Weather Station — Atmospheric Pressure, Humidity & Dew Point

10 benchmark entries across 5 formats.

---

## Grid
```
  Temperate       :   P    T
  Alpine          :   D    M    L
  Polar           :  Mc    R
  Tropical        :   N    S
  Arid            :  DV
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
| Label | category | symbol | name | altitude | temperature | pressure | relative_humidity | saturation_vp | dew_point | density_altitude | potential_temp | mixing_ratio | lcl | Precision | Iterations | Avg (ns) | Iters/s |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Paris CDG | Temperate | P | Paris CDG | 119 m | 12.4 °C | 1013.2 hPa | 78% | 14.39 hPa | 8.7 °C | -164 m | 285.5 K | 6.97 g/kg | 381 m AGL | f64 | 5000 | 5650.99 | 178571.43 |
| Tokyo Narita | Temperate | T | Tokyo Narita | 44 m | 16.3 °C | 1015.0 hPa | 71% | 18.52 hPa | 11.0 °C | 211 m | 289.3 K | 8.16 g/kg | 539 m AGL | f64 | 5000 | 5584.68 | 185185.19 |
| Denver Intl | Alpine | D | Denver Intl | 1655 m | 10.1 °C | 835.0 hPa | 45% | 12.35 hPa | -1.3 °C | 1464 m | 299.4 K | 4.17 g/kg | 1166 m AGL | f64 | 5000 | 5547.55 | 185185.19 |
| Mexico City | Alpine | M | Mexico City | 2250 m | 16.0 °C | 773.0 hPa | 52% | 18.17 hPa | 6.2 °C | 2910 m | 312.4 K | 7.70 g/kg | 1008 m AGL | f64 | 5000 | 5568.53 | 185185.19 |
| McMurdo Base | Polar | Mc | McMurdo Base | 10 m | -18.0 °C | 984.0 hPa | 60% | 1.49 hPa | -23.9 °C | -3948 m | 257.3 K | 0.57 g/kg | 602 m AGL | f64 | 5000 | 6746.35 | 151515.15 |
| Nairobi | Tropical | N | Nairobi | 1795 m | 19.0 °C | 815.0 hPa | 65% | 21.96 hPa | 12.3 °C | 2706 m | 310.9 K | 11.09 g/kg | 688 m AGL | f64 | 5000 | 6022.63 | 166666.67 |
| Lhasa | Alpine | L | Lhasa | 3650 m | 8.5 °C | 652.0 hPa | 35% | 11.09 hPa | -6.1 °C | 3746 m | 319.5 K | 3.73 g/kg | 1494 m AGL | f64 | 5000 | 5979.66 | 172413.79 |
| Death Valley | Arid | DV | Death Valley | -86 m | 38.5 °C | 1025.0 hPa | 12% | 68.21 hPa | 4.1 °C | 2713 m | 310.6 K | 5.01 g/kg | 3521 m AGL | f64 | 5000 | 5659.51 | 178571.43 |
| Reykjavik | Polar | R | Reykjavik | 61 m | 4.6 °C | 1005.0 hPa | 82% | 8.48 hPa | 1.8 °C | -1172 m | 278.4 K | 4.33 g/kg | 287 m AGL | f64 | 5000 | 5605.35 | 178571.43 |
| Singapore | Tropical | S | Singapore | 16 m | 27.5 °C | 1010.0 hPa | 84% | 36.72 hPa | 24.6 °C | 1520 m | 300.9 K | 19.59 g/kg | 301 m AGL | f64 | 5000 | 5538.39 | 185185.19 |
## Analytics
### Category Distribution
| Category | Count | Avg (ns) | Min (ns) | Max (ns) |
|---|---|---|---|---|
| Temperate | 2 | 5617.83 | 5584.68 | 5650.99 |
| Alpine | 3 | 5698.58 | 5547.55 | 5979.66 |
| Polar | 2 | 6175.85 | 5605.35 | 6746.35 |
| Tropical | 2 | 5780.51 | 5538.39 | 6022.63 |
| Arid | 1 | 5659.51 | 5659.51 | 5659.51 |

### Top 10 Fastest Entries
| # | Entry | Avg (ns) |
|---|---|---|
| 1 | Singapore | 5538.39 |
| 2 | Denver Intl | 5547.55 |
| 3 | Mexico City | 5568.53 |
| 4 | Tokyo Narita | 5584.68 |
| 5 | Reykjavik | 5605.35 |
| 6 | Paris CDG | 5650.99 |
| 7 | Death Valley | 5659.51 |
| 8 | Lhasa | 5979.66 |
| 9 | Nairobi | 6022.63 |
| 10 | McMurdo Base | 6746.35 |

### Top 10 Slowest Entries
| # | Entry | Avg (ns) |
|---|---|---|
| 1 | McMurdo Base | 6746.35 |
| 2 | Nairobi | 6022.63 |
| 3 | Lhasa | 5979.66 |
| 4 | Death Valley | 5659.51 |
| 5 | Paris CDG | 5650.99 |
| 6 | Reykjavik | 5605.35 |
| 7 | Tokyo Narita | 5584.68 |
| 8 | Mexico City | 5568.53 |
| 9 | Denver Intl | 5547.55 |
| 10 | Singapore | 5538.39 |

