# Solar System — Orbital Mechanics (Kepler, Hohmann, Escape Velocities)

8 benchmark entries across 5 formats.

---

## Grid
```
  Terrestrial     :   ☿    ♀    ♁    ♂
  Gas Giant       :   ♃    ♄
  Ice Giant       :   ⛢    ♆
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
| Label | category | symbol | name | mass | radius | semi_major | kepler_period | circular_vel | escape_vel | hohmann_dv | Precision | Iterations | Avg (ns) | Iters/s |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Mercury | Terrestrial | ☿ | Mercury | 3.301e23 kg | 2440 km | 0.387 AU | 87.95 days | 47.88 km/s | 4.25 km/s | 17.15 km/s | f64 | 5000 | 1863.60 | 555555.56 |
| Venus | Terrestrial | ♀ | Venus | 4.868e24 kg | 6052 km | 0.723 AU | 224.67 days | 35.03 km/s | 10.36 km/s | 5.20 km/s | f64 | 5000 | 1768.45 | 625000.00 |
| Earth | Terrestrial | ♁ | Earth | 5.972e24 kg | 6372 km | 1.000 AU | 365.19 days | 29.79 km/s | 11.19 km/s | — (origin) | f64 | 5000 | 1747.73 | 625000.00 |
| Mars | Terrestrial | ♂ | Mars | 6.417e23 kg | 3390 km | 1.524 AU | 686.86 days | 24.13 km/s | 5.03 km/s | 5.59 km/s | f64 | 5000 | 1752.94 | 625000.00 |
| Jupiter | Gas Giant | ♃ | Jupiter | 1.898e27 kg | 69911 km | 5.204 AU | 4335.92 days | 13.06 km/s | 60.20 km/s | 14.44 km/s | f64 | 5000 | 1762.57 | 625000.00 |
| Saturn | Gas Giant | ♄ | Saturn | 5.683e26 kg | 58232 km | 9.582 AU | 10832.57 days | 9.62 km/s | 36.09 km/s | 15.74 km/s | f64 | 5000 | 1761.93 | 625000.00 |
| Uranus | Ice Giant | ⛢ | Uranus | 8.681e25 kg | 25362 km | 19.201 AU | 30727.34 days | 6.80 km/s | 21.38 km/s | 15.94 km/s | f64 | 5000 | 1820.37 | 555555.56 |
| Neptune | Ice Giant | ♆ | Neptune | 1.024e26 kg | 24622 km | 30.048 AU | 60151.16 days | 5.43 km/s | 23.56 km/s | 15.71 km/s | f64 | 5000 | 1726.02 | 625000.00 |
## Analytics
### Category Distribution
| Category | Count | Avg (ns) | Min (ns) | Max (ns) |
|---|---|---|---|---|
| Terrestrial | 4 | 1783.18 | 1747.73 | 1863.60 |
| Gas Giant | 2 | 1762.25 | 1761.93 | 1762.57 |
| Ice Giant | 2 | 1773.20 | 1726.02 | 1820.37 |

### Top 10 Fastest Entries
| # | Entry | Avg (ns) |
|---|---|---|
| 1 | Neptune | 1726.02 |
| 2 | Earth | 1747.73 |
| 3 | Mars | 1752.94 |
| 4 | Saturn | 1761.93 |
| 5 | Jupiter | 1762.57 |
| 6 | Venus | 1768.45 |
| 7 | Uranus | 1820.37 |
| 8 | Mercury | 1863.60 |

### Top 10 Slowest Entries
| # | Entry | Avg (ns) |
|---|---|---|
| 1 | Mercury | 1863.60 |
| 2 | Uranus | 1820.37 |
| 3 | Venus | 1768.45 |
| 4 | Jupiter | 1762.57 |
| 5 | Saturn | 1761.93 |
| 6 | Mars | 1752.94 |
| 7 | Earth | 1747.73 |
| 8 | Neptune | 1726.02 |

