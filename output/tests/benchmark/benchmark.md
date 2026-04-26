# Benchmark Integration Test

3 benchmark entries across 5 formats.

---

## Grid
```
  other           :   ?    ?    ?
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
| Label | group | Precision | Iterations | Avg (ns) | Iters/s |
|---|---|---|---|---|---|
| bench_alpha | alpha | f64 | 100 | 9.70 | 0.00 |
| bench_beta | beta | f32 | 100 | 9.00 | 0.00 |
| bench_gamma | gamma | f64 | 100 | 8.70 | 0.00 |
## Analytics
### Group Distribution
| Group | Count | Avg (ns) | Min (ns) | Max (ns) |
|---|---|---|---|---|
| alpha | 1 | 9.70 | 9.70 | 9.70 |
| beta | 1 | 9.00 | 9.00 | 9.00 |
| gamma | 1 | 8.70 | 8.70 | 8.70 |

### Top 10 Fastest Entries
| # | Entry | Avg (ns) |
|---|---|---|
| 1 | bench_gamma | 8.70 |
| 2 | bench_beta | 9.00 |
| 3 | bench_alpha | 9.70 |

### Top 10 Slowest Entries
| # | Entry | Avg (ns) |
|---|---|---|
| 1 | bench_alpha | 9.70 |
| 2 | bench_beta | 9.00 |
| 3 | bench_gamma | 8.70 |

