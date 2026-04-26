# Star Catalog — Luminosity, Magnitude, Schwarzschild Radius & Main Sequence Lifetime

10 benchmark entries across 5 formats.

---

## Grid
```
  Main Sequence   :   G    A    A
  Supergiant      :   M    B    F    A
  Red Dwarf       :   M    M
  Giant           :   K
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
| Label | category | symbol | name | spectral | mass | radius | temperature | luminosity | abs_magnitude | schwarzschild | ms_lifetime | wien_peak | Precision | Iterations | Avg (ns) | Iters/s |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Sun | Main Sequence | G | Sun | G2V | 1.000 M☉ | 1.000 R☉ | 5778 K | 3.844e26 W | 4.83 | 2954.28 m | 0.0 Myr | 502 nm | f64 | 5000 | 1744.29 | 625000.00 |
| Sirius A | Main Sequence | A | Sirius A | A1V | 2.063 M☉ | 1.711 R☉ | 9940 K | 9.856e27 W | 1.43 | 6094.67 m | 0.0 Myr | 292 nm | f64 | 5000 | 1677.32 | 625000.00 |
| Betelgeuse | Supergiant | M | Betelgeuse | M1Ia | 16.500 M☉ | 887.000 R☉ | 3600 K | 4.557e31 W | -5.71 | 48745.54 m | 0.0 Myr | 805 nm | f64 | 5000 | 1696.30 | 625000.00 |
| Rigel | Supergiant | B | Rigel | B8Ia | 21.000 M☉ | 78.900 R☉ | 12100 K | 4.602e31 W | -6.98 | 62039.78 m | 0.0 Myr | 239 nm | f64 | 5000 | 1671.57 | 625000.00 |
| Vega | Main Sequence | A | Vega | A0V | 2.135 M☉ | 2.362 R☉ | 9602 K | 1.636e28 W | 0.60 | 6307.38 m | 0.0 Myr | 302 nm | f64 | 5000 | 1681.65 | 625000.00 |
| Proxima Centauri | Red Dwarf | M | Proxima Centauri | M5.5Ve | 0.122 M☉ | 0.154 R☉ | 3042 K | 7.022e23 W | 15.56 | 360.42 m | 0.0 Myr | 953 nm | f64 | 5000 | 1704.27 | 625000.00 |
| Polaris | Supergiant | F | Polaris | F7Ib | 5.400 M☉ | 37.500 R☉ | 6015 K | 6.348e29 W | -3.63 | 15953.09 m | 0.0 Myr | 482 nm | f64 | 5000 | 1646.64 | 625000.00 |
| Aldebaran | Giant | K | Aldebaran | K5III | 1.160 M☉ | 44.130 R☉ | 3910 K | 1.570e29 W | -0.69 | 3426.96 m | 0.0 Myr | 741 nm | f64 | 5000 | 1723.64 | 625000.00 |
| Deneb | Supergiant | A | Deneb | A2Ia | 19.000 M☉ | 203.000 R☉ | 8525 K | 7.506e31 W | -8.27 | 56131.23 m | 0.0 Myr | 340 nm | f64 | 5000 | 1713.84 | 625000.00 |
| Barnard's Star | Red Dwarf | M | Barnard's Star | M4Ve | 0.144 M☉ | 0.196 R☉ | 3134 K | 1.278e24 W | 13.19 | 425.42 m | 0.0 Myr | 925 nm | f64 | 5000 | 1759.21 | 625000.00 |
## Analytics
### Category Distribution
| Category | Count | Avg (ns) | Min (ns) | Max (ns) |
|---|---|---|---|---|
| Main Sequence | 3 | 1701.08 | 1677.32 | 1744.29 |
| Supergiant | 4 | 1682.09 | 1646.64 | 1713.84 |
| Red Dwarf | 2 | 1731.74 | 1704.27 | 1759.21 |
| Giant | 1 | 1723.64 | 1723.64 | 1723.64 |

### Top 10 Fastest Entries
| # | Entry | Avg (ns) |
|---|---|---|
| 1 | Polaris | 1646.64 |
| 2 | Rigel | 1671.57 |
| 3 | Sirius A | 1677.32 |
| 4 | Vega | 1681.65 |
| 5 | Betelgeuse | 1696.30 |
| 6 | Proxima Centauri | 1704.27 |
| 7 | Deneb | 1713.84 |
| 8 | Aldebaran | 1723.64 |
| 9 | Sun | 1744.29 |
| 10 | Barnard's Star | 1759.21 |

### Top 10 Slowest Entries
| # | Entry | Avg (ns) |
|---|---|---|
| 1 | Barnard's Star | 1759.21 |
| 2 | Sun | 1744.29 |
| 3 | Aldebaran | 1723.64 |
| 4 | Deneb | 1713.84 |
| 5 | Proxima Centauri | 1704.27 |
| 6 | Betelgeuse | 1696.30 |
| 7 | Vega | 1681.65 |
| 8 | Sirius A | 1677.32 |
| 9 | Rigel | 1671.57 |
| 10 | Polaris | 1646.64 |

