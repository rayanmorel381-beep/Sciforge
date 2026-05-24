# Benchmark Module

The benchmark module provides **6 submodules** for performance measurement, binary encoding, simulation profiling, and multi-format report generation. It serves as SciForge's internal benchmarking infrastructure.

---

## Chapter 1 - Module Scope

### Scientific Purpose
This section defines the scope: which problem this module solves, which abstraction level it targets, and which outputs are expected.

### Modeling Assumptions
- The equations are numerical models, not analytical proofs.
- Inputs must respect the documented units.
- Validity domains (linear regime, local approximation, and so on) should be verified by the reader.

### Reading Strategy
1. Read the module structure.
2. Identify equations and their conditions.
3. Map these equations to Rust functions in `docs/code/`.


## Reading Guide

### Goal
This page explains the module from a scientific perspective: assumptions, models, equations, and usage limits.

### How To Read This Document
- Start with the domain section relevant to your case.
- Identify key equations and their usage conditions.
- Then verify the corresponding Rust signatures in `docs/code/`.

### Conventions
- Units: SI unless explicitly stated.
- Variables: standard mathematical notation for the domain.
- Equations are presented as reference models, not formal proofs.


## 1. Engine

Core benchmark execution framework.

The engine defines the `BenchmarkMetrics` struct that captures all measurable dimensions of an experiment:

| Field | Type | Description |
|---|---|---|
| `experiment_name` | `&str` | Identifier for the benchmark |
| `precision` | `&str` | Numeric precision label (e.g. "f64", "f32") |
| `elapsed_ms` | `u64` | Total wall-clock time in milliseconds |
| `iterations` | `u64` | Number of measured iterations |
| `avg_time_ns` | `f32` | Mean time per iteration (ns) |
| `min_time_ns` / `max_time_ns` | `f32` | Min/max iteration time |
| `time_stddev` | `f32` | Standard deviation of iteration times |
| `iterations_per_sec` | `f32` | Throughput: $1 / \bar{t}$ |
| `samples_per_sec` | `f32` | Data throughput |
| `total_flops` | `u64` | Floating-point operations counted |
| `eval_error` | `f32` | Numerical error metric |
| `eval_accuracy` | `f32` | Accuracy score |
| `eval_r_squared` | `f32` | Coefficient of determination $R^2$ |
| `eval_mae` | `f32` | Mean absolute error |
| `logical_cores` | `u32` | System CPU core count |

The runner uses `std::hint::black_box` to prevent compiler dead-code elimination, and `std::time::Instant` for nanosecond-precision timing. A CSV header is provided for direct row export:

```
experiment_name,precision,elapsed_ms,iterations,avg_time_ns,min_time_ns,max_time_ns,time_stddev,iterations_per_sec,result
```

**Statistical aggregation.** The standard deviation of timing samples quantifies measurement noise:

$$\sigma_t = \sqrt{\frac{1}{N-1} \sum_{i=1}^{N} (t_i - \bar{t})^2}$$

Coefficient of variation $\text{CV} = \sigma_t / \bar{t}$ indicates benchmark stability — values below 5% suggest reliable measurements.

---

## 2. Encode

Binary serialization of benchmark metrics into a compact binary format.

The encoding uses a **fixed-size header** (`BENCHMARK_HEADER_SIZE`) followed by variable-length experiment name and precision strings. The binary layout:

| Offset | Size | Field |
|---|---|---|
| 0–3 | 4 bytes | Magic number (`BENCHMARK_MAGIC`) |
| 4–5 | 2 bytes | Format version (little-endian `u16`) |
| 8–15 | 8 bytes | `elapsed_ms` (LE u64) |
| 16–23 | 8 bytes | `iterations` (LE u64) |
| 24–31 | 8 bytes | `input_samples` (LE u64) |
| 32–35 | 4 bytes | `avg_time_ns` (LE f32) |
| ... | ... | (all metrics fields in sequence) |
| 164–165 | 2 bytes | experiment name length |
| 166–167 | 2 bytes | precision string length |
| header+ | variable | experiment name + precision bytes |

All multi-byte fields are **little-endian**. The `encoded_size()` function computes the exact buffer requirement via checked arithmetic to prevent overflow. Encode returns the number of bytes written.

---

## 3. Decode

Deserialization of binary benchmark data back into `BenchmarkMetricsView`.

Decoding attempts two strategies: **full format** (with magic+version header validation) and **compact format** (fallback). The decoder validates:
- Minimum buffer size ≥ `BENCHMARK_HEADER_SIZE`
- Magic bytes match `BENCHMARK_MAGIC`
- Version matches `BENCHMARK_VERSION`
- String length fields do not exceed buffer bounds

`BenchmarkMetricsView` borrows string slices directly from the input buffer (zero-copy), returning `&'a str` references with lifetime tied to the source bytes.

---

## 4. Simulation

Physics simulation benchmarking with configurable N-body state.

The simulation framework provides:

- **`SimState`** — Positions and velocities for $N$ bodies in 3D: `positions: Vec<[f64; 3]>`, `velocities: Vec<[f64; 3]>`, plus current simulation time and step count.
- **`SimConfig`** — Timestep $\Delta t$, maximum steps, body count.
- **`StepFn` trait** — Integration interface: `fn integrate(&self, state: &mut SimState, dt: f64)`. Any ODE integrator (Euler, RK4, Verlet) can be plugged in.
- **`RenderSink` trait** — Frame callback for visualization or data collection during simulation.

The benchmark runner measures:
- **Wall-clock time** per simulation step
- **Total elapsed time** for the full run
- **Throughput**: steps per second, bodies × steps per second
- **Scaling behavior**: execution time vs. $N$ (expected $O(N^2)$ for direct N-body)

---

## 5. Report

Multi-format benchmark report generation.

From a `BenchmarkMetrics` instance, the report module generates three synchronized outputs:

**CSV.** Single-row data suitable for spreadsheet import or time-series tracking:
```
header_row
metrics_row,result
```

**Markdown.** Structured report with:
- Summary table (all metrics as key-value rows)
- FLOPS, accuracy, $R^2$ (when nonzero)
- Fenced code block for the result output

**HTML.** Self-contained single-file report embedding the Markdown content, with inline CSS for readability.

**Structured data.** Additional generators produce:
- **JSON** (`generate_json_tagged`): tagged key-value output with custom metadata
- **YAML** (`generate_yaml_tagged`): human-readable structured output
- **TOML** (`generate_toml_tagged`): configuration-style output

All string output uses safe escaping (`push_escaped`) to prevent injection in generated reports.

---

## 6. Export

File-based export orchestrator for benchmark suites.

The `export()` function takes a title, a slice of `Entry` records (each wrapping a `BenchmarkMetrics` with optional tags, labels, and grid coordinates), and an output directory. It produces:

- **CSV directory** with per-experiment CSV files
- **Markdown directory** with individual `.md` reports
- **HTML dashboard** with a grid layout (using `grid_row`/`grid_col` placement)
- **Binary `.bench` files** with encoded metrics for machine consumption

The `ExportSummary` struct reports: files written, HTML size (bytes), Markdown size (bytes).

Element category CSS mapping supports periodic table visualization in exported HTML dashboards, with classes for each IUPAC element category (nonmetal, noble-gas, alkali, transition, lanthanide, actinide, etc.).

---

## Chapter 3 - Limits, Precision, and Validation

### Numerical Limits
- `f64` rounding errors can accumulate in long simulations.
- Extreme regimes (very large or very small scales) require explicit numerical stability checks.

### Recommended Verification
- Compare against a simple analytical case when available.
- Check the order of magnitude of results.
- Run sensitivity analysis on dominant parameters.

### Transition to Implementation
For concrete function calls, Rust signatures and module paths are documented in `docs/code/`.
