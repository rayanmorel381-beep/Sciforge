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
| Mercury | Terrestrial | ☿ | Mercury | 3.301e23 kg | 2440 km | 0.387 AU | 87.95 days | 47.88 km/s | 4.25 km/s | 17.15 km/s | f64 | 5000 | 1855.87 | 555555.56 |
| Venus | Terrestrial | ♀ | Venus | 4.868e24 kg | 6052 km | 0.723 AU | 224.67 days | 35.03 km/s | 10.36 km/s | 5.20 km/s | f64 | 5000 | 1741.47 | 625000.00 |
| Earth | Terrestrial | ♁ | Earth | 5.972e24 kg | 6372 km | 1.000 AU | 365.19 days | 29.79 km/s | 11.19 km/s | — (origin) | f64 | 5000 | 1730.09 | 625000.00 |
| Mars | Terrestrial | ♂ | Mars | 6.417e23 kg | 3390 km | 1.524 AU | 686.86 days | 24.13 km/s | 5.03 km/s | 5.59 km/s | f64 | 5000 | 1792.81 | 625000.00 |
| Jupiter | Gas Giant | ♃ | Jupiter | 1.898e27 kg | 69911 km | 5.204 AU | 4335.92 days | 13.06 km/s | 60.20 km/s | 14.44 km/s | f64 | 5000 | 1797.72 | 625000.00 |
| Saturn | Gas Giant | ♄ | Saturn | 5.683e26 kg | 58232 km | 9.582 AU | 10832.57 days | 9.62 km/s | 36.09 km/s | 15.74 km/s | f64 | 5000 | 1849.66 | 555555.56 |
| Uranus | Ice Giant | ⛢ | Uranus | 8.681e25 kg | 25362 km | 19.201 AU | 30727.34 days | 6.80 km/s | 21.38 km/s | 15.94 km/s | f64 | 5000 | 1818.38 | 555555.56 |
| Neptune | Ice Giant | ♆ | Neptune | 1.024e26 kg | 24622 km | 30.048 AU | 60151.16 days | 5.43 km/s | 23.56 km/s | 15.71 km/s | f64 | 5000 | 1813.08 | 555555.56 |
## Analytics
### Category Distribution
| Category | Count | Avg (ns) | Min (ns) | Max (ns) |
|---|---|---|---|---|
| Terrestrial | 4 | 1780.06 | 1730.09 | 1855.87 |
| Gas Giant | 2 | 1823.69 | 1797.72 | 1849.66 |
| Ice Giant | 2 | 1815.73 | 1813.08 | 1818.38 |

### Top 10 Fastest Entries
| # | Entry | Avg (ns) |
|---|---|---|
| 1 | Earth | 1730.09 |
| 2 | Venus | 1741.47 |
| 3 | Mars | 1792.81 |
| 4 | Jupiter | 1797.72 |
| 5 | Neptune | 1813.08 |
| 6 | Uranus | 1818.38 |
| 7 | Saturn | 1849.66 |
| 8 | Mercury | 1855.87 |

### Top 10 Slowest Entries
| # | Entry | Avg (ns) |
|---|---|---|
| 1 | Mercury | 1855.87 |
| 2 | Saturn | 1849.66 |
| 3 | Uranus | 1818.38 |
| 4 | Neptune | 1813.08 |
| 5 | Jupiter | 1797.72 |
| 6 | Mars | 1792.81 |
| 7 | Venus | 1741.47 |
| 8 | Earth | 1730.09 |

