# Roundtrip

1 benchmark entries across 5 formats.

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
| Label | Precision | Iterations | Avg (ns) | Iters/s |
|---|---|---|---|---|
| rt | f32 | 25 | 28.40 | 0.00 |
## Analytics
### Group Distribution
| Group | Count | Avg (ns) | Min (ns) | Max (ns) |
|---|---|---|---|---|
| other | 1 | 28.40 | 28.40 | 28.40 |

### Top 10 Fastest Entries
| # | Entry | Avg (ns) |
|---|---|---|
| 1 | rt | 28.40 |

### Top 10 Slowest Entries
| # | Entry | Avg (ns) |
|---|---|---|
| 1 | rt | 28.40 |

