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
| Sun | Main Sequence | G | Sun | G2V | 1.000 M☉ | 1.000 R☉ | 5778 K | 3.844e26 W | 4.83 | 2954.28 m | 0.0 Myr | 502 nm | f64 | 5000 | 1815.82 | 555555.56 |
| Sirius A | Main Sequence | A | Sirius A | A1V | 2.063 M☉ | 1.711 R☉ | 9940 K | 9.856e27 W | 1.43 | 6094.67 m | 0.0 Myr | 292 nm | f64 | 5000 | 1876.73 | 555555.56 |
| Betelgeuse | Supergiant | M | Betelgeuse | M1Ia | 16.500 M☉ | 887.000 R☉ | 3600 K | 4.557e31 W | -5.71 | 48745.54 m | 0.0 Myr | 805 nm | f64 | 5000 | 1751.97 | 625000.00 |
| Rigel | Supergiant | B | Rigel | B8Ia | 21.000 M☉ | 78.900 R☉ | 12100 K | 4.602e31 W | -6.98 | 62039.78 m | 0.0 Myr | 239 nm | f64 | 5000 | 1738.15 | 625000.00 |
| Vega | Main Sequence | A | Vega | A0V | 2.135 M☉ | 2.362 R☉ | 9602 K | 1.636e28 W | 0.60 | 6307.38 m | 0.0 Myr | 302 nm | f64 | 5000 | 1792.81 | 625000.00 |
| Proxima Centauri | Red Dwarf | M | Proxima Centauri | M5.5Ve | 0.122 M☉ | 0.154 R☉ | 3042 K | 7.022e23 W | 15.56 | 360.42 m | 0.0 Myr | 953 nm | f64 | 5000 | 1794.42 | 625000.00 |
| Polaris | Supergiant | F | Polaris | F7Ib | 5.400 M☉ | 37.500 R☉ | 6015 K | 6.348e29 W | -3.63 | 15953.09 m | 0.0 Myr | 482 nm | f64 | 5000 | 1788.72 | 625000.00 |
| Aldebaran | Giant | K | Aldebaran | K5III | 1.160 M☉ | 44.130 R☉ | 3910 K | 1.570e29 W | -0.69 | 3426.96 m | 0.0 Myr | 741 nm | f64 | 5000 | 1846.47 | 555555.56 |
| Deneb | Supergiant | A | Deneb | A2Ia | 19.000 M☉ | 203.000 R☉ | 8525 K | 7.506e31 W | -8.27 | 56131.23 m | 0.0 Myr | 340 nm | f64 | 5000 | 1838.03 | 555555.56 |
| Barnard's Star | Red Dwarf | M | Barnard's Star | M4Ve | 0.144 M☉ | 0.196 R☉ | 3134 K | 1.278e24 W | 13.19 | 425.42 m | 0.0 Myr | 925 nm | f64 | 5000 | 1756.05 | 625000.00 |
## Analytics
### Category Distribution
| Category | Count | Avg (ns) | Min (ns) | Max (ns) |
|---|---|---|---|---|
| Main Sequence | 3 | 1828.45 | 1792.81 | 1876.73 |
| Supergiant | 4 | 1779.22 | 1738.15 | 1838.03 |
| Red Dwarf | 2 | 1775.23 | 1756.05 | 1794.42 |
| Giant | 1 | 1846.47 | 1846.47 | 1846.47 |

### Top 10 Fastest Entries
| # | Entry | Avg (ns) |
|---|---|---|
| 1 | Rigel | 1738.15 |
| 2 | Betelgeuse | 1751.97 |
| 3 | Barnard's Star | 1756.05 |
| 4 | Polaris | 1788.72 |
| 5 | Vega | 1792.81 |
| 6 | Proxima Centauri | 1794.42 |
| 7 | Sun | 1815.82 |
| 8 | Deneb | 1838.03 |
| 9 | Aldebaran | 1846.47 |
| 10 | Sirius A | 1876.73 |

### Top 10 Slowest Entries
| # | Entry | Avg (ns) |
|---|---|---|
| 1 | Sirius A | 1876.73 |
| 2 | Aldebaran | 1846.47 |
| 3 | Deneb | 1838.03 |
| 4 | Sun | 1815.82 |
| 5 | Proxima Centauri | 1794.42 |
| 6 | Vega | 1792.81 |
| 7 | Polaris | 1788.72 |
| 8 | Barnard's Star | 1756.05 |
| 9 | Betelgeuse | 1751.97 |
| 10 | Rigel | 1738.15 |

