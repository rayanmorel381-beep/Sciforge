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
| bench_alpha | alpha | f64 | 100 | 16.50 | 0.00 |
| bench_beta | beta | f32 | 100 | 14.10 | 0.00 |
| bench_gamma | gamma | f64 | 100 | 13.20 | 0.00 |
## Analytics
### Group Distribution
| Group | Count | Avg (ns) | Min (ns) | Max (ns) |
|---|---|---|---|---|
| alpha | 1 | 16.50 | 16.50 | 16.50 |
| beta | 1 | 14.10 | 14.10 | 14.10 |
| gamma | 1 | 13.20 | 13.20 | 13.20 |

### Top 10 Fastest Entries
| # | Entry | Avg (ns) |
|---|---|---|
| 1 | bench_gamma | 13.20 |
| 2 | bench_beta | 14.10 |
| 3 | bench_alpha | 16.50 |

### Top 10 Slowest Entries
| # | Entry | Avg (ns) |
|---|---|---|
| 1 | bench_alpha | 16.50 |
| 2 | bench_beta | 14.10 |
| 3 | bench_gamma | 13.20 |

