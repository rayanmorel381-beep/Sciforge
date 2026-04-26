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
| Paris CDG | Temperate | P | Paris CDG | 119 m | 12.4 °C | 1013.2 hPa | 78% | 14.39 hPa | 8.7 °C | -164 m | 285.5 K | 6.97 g/kg | 381 m AGL | f64 | 5000 | 5629.84 | 178571.42 |
| Tokyo Narita | Temperate | T | Tokyo Narita | 44 m | 16.3 °C | 1015.0 hPa | 71% | 18.52 hPa | 11.0 °C | 211 m | 289.3 K | 8.16 g/kg | 539 m AGL | f64 | 5000 | 5656.88 | 178571.42 |
| Denver Intl | Alpine | D | Denver Intl | 1655 m | 10.1 °C | 835.0 hPa | 45% | 12.35 hPa | -1.3 °C | 1464 m | 299.4 K | 4.17 g/kg | 1166 m AGL | f64 | 5000 | 5595.95 | 185185.19 |
| Mexico City | Alpine | M | Mexico City | 2250 m | 16.0 °C | 773.0 hPa | 52% | 18.17 hPa | 6.2 °C | 2910 m | 312.4 K | 7.70 g/kg | 1008 m AGL | f64 | 5000 | 5636.10 | 178571.42 |
| McMurdo Base | Polar | Mc | McMurdo Base | 10 m | -18.0 °C | 984.0 hPa | 60% | 1.49 hPa | -23.9 °C | -3948 m | 257.3 K | 0.57 g/kg | 602 m AGL | f64 | 5000 | 5812.67 | 172413.80 |
| Nairobi | Tropical | N | Nairobi | 1795 m | 19.0 °C | 815.0 hPa | 65% | 21.96 hPa | 12.3 °C | 2706 m | 310.9 K | 11.09 g/kg | 688 m AGL | f64 | 5000 | 5534.76 | 185185.19 |
| Lhasa | Alpine | L | Lhasa | 3650 m | 8.5 °C | 652.0 hPa | 35% | 11.09 hPa | -6.1 °C | 3746 m | 319.5 K | 3.73 g/kg | 1494 m AGL | f64 | 5000 | 5548.93 | 185185.19 |
| Death Valley | Arid | DV | Death Valley | -86 m | 38.5 °C | 1025.0 hPa | 12% | 68.21 hPa | 4.1 °C | 2713 m | 310.6 K | 5.01 g/kg | 3521 m AGL | f64 | 5000 | 5569.10 | 185185.19 |
| Reykjavik | Polar | R | Reykjavik | 61 m | 4.6 °C | 1005.0 hPa | 82% | 8.48 hPa | 1.8 °C | -1172 m | 278.4 K | 4.33 g/kg | 287 m AGL | f64 | 5000 | 5642.68 | 178571.42 |
| Singapore | Tropical | S | Singapore | 16 m | 27.5 °C | 1010.0 hPa | 84% | 36.72 hPa | 24.6 °C | 1520 m | 300.9 K | 19.59 g/kg | 301 m AGL | f64 | 5000 | 5640.50 | 178571.42 |
## Analytics
### Category Distribution
| Category | Count | Avg (ns) | Min (ns) | Max (ns) |
|---|---|---|---|---|
| Temperate | 2 | 5643.36 | 5629.84 | 5656.88 |
| Alpine | 3 | 5593.66 | 5548.93 | 5636.10 |
| Polar | 2 | 5727.68 | 5642.68 | 5812.67 |
| Tropical | 2 | 5587.63 | 5534.76 | 5640.50 |
| Arid | 1 | 5569.10 | 5569.10 | 5569.10 |

### Top 10 Fastest Entries
| # | Entry | Avg (ns) |
|---|---|---|
| 1 | Nairobi | 5534.76 |
| 2 | Lhasa | 5548.93 |
| 3 | Death Valley | 5569.10 |
| 4 | Denver Intl | 5595.95 |
| 5 | Paris CDG | 5629.84 |
| 6 | Mexico City | 5636.10 |
| 7 | Singapore | 5640.50 |
| 8 | Reykjavik | 5642.68 |
| 9 | Tokyo Narita | 5656.88 |
| 10 | McMurdo Base | 5812.67 |

### Top 10 Slowest Entries
| # | Entry | Avg (ns) |
|---|---|---|
| 1 | McMurdo Base | 5812.67 |
| 2 | Tokyo Narita | 5656.88 |
| 3 | Reykjavik | 5642.68 |
| 4 | Singapore | 5640.50 |
| 5 | Mexico City | 5636.10 |
| 6 | Paris CDG | 5629.84 |
| 7 | Denver Intl | 5595.95 |
| 8 | Death Valley | 5569.10 |
| 9 | Lhasa | 5548.93 |
| 10 | Nairobi | 5534.76 |

