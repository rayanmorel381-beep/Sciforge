# Benchmark Source Code Guide

This page documents the source implementation behind `sciforge::benchmark`, including benchmark engine internals, encoding/decoding, simulation flow, report generation, and exporters.

## Source Coverage

### What is explained
- File-level implementation layout in `src/benchmark/`.
- Main algorithms and where they are implemented.
- Runtime call path for benchmark execution, reporting, and exports.

### Visibility and external access
- Visibility: public (`pub mod benchmark;` in `src/lib.rs`).
- External direct path: `sciforge::benchmark::*`.
- Hub integration is optional for orchestration workflows.


## Source Code Explanation

### Relevant file structure
- Main implementation: `src/benchmark/`
- Module entry point: `src/benchmark/mod.rs`
- Hub routing (when applicable): `(n/a - benchmark is direct)`

### Internal logic
1. The module exposes subdomains through `mod.rs` and targeted `pub use` exports.
2. Each `.rs` file implements a coherent block of equations and functions.
3. Hub integration is optional for orchestration workflows.

### What to verify in source code
- Exact signature: input/output types and argument order.
- Numerical preconditions: divisions, roots, logarithms, and validity ranges.
- Implicit conventions: units, normalization, bounds, and tolerances.

## Modules

- `engine` — core benchmarking loop, metrics struct, constants (`bench`, `BenchmarkMetrics`, `CSV_HEADER`)
- `simulation` — deterministic simulation runner (`SimConfig`, `SimState`, `SimResult`, `StepFn`, `RenderSink`)
- `encode` — binary serialization of benchmark metrics
- `decode` — binary deserialization of encoded benchmark data
- `report` — report generation in CSV, JSON, YAML, TOML, Markdown, HTML (`BenchmarkReport`)
- `export` — multi-format batch export to filesystem (`Entry`, `GroupStats`, `ExportSummary`)

---

## Engine API (6 items)

### Structs

| Struct | Fields |
|---|---|
| `BenchmarkMetrics<'a>` | `experiment_name: &'a str`, `precision: &'a str`, `elapsed_ms: u64`, `iterations: u64`, `input_samples: u64`, `avg_time_ns: f64`, `last_time_ns: f64`, `output_bytes: usize`, `total_flops: u64`, `step_count: u32`, `input_dim: u32`, `output_dim: u32`, `benchmark_flags: u64`, `input_bytes: u64`, `result_bytes: u64`, `min_time_ns: f64`, `max_time_ns: f64`, `time_stddev: f64`, `iterations_per_sec: f64`, `samples_per_sec: f64`, `eval_error: f64`, `eval_accuracy: f64`, `eval_r_squared: f64`, `eval_mae: f64`, `eval_samples: u64`, `eval_dataset_hash: u64`, `logical_cores: u32`, `avg_frequency_mhz: u32`, `max_frequency_mhz: u32`, `max_workers: u32`, `target_cpu_utilization: f64` |

### Enums

| Enum | Variants |
|---|---|
| `BenchmarkEncodeError` | `BufferTooSmall`, `InvalidFormat` |

### Constants

| Constant | Type | Value |
|---|---|---|
| `CSV_HEADER` | `&str` | `"experiment_name,precision,elapsed_ms,iterations,avg_time_ns,min_time_ns,max_time_ns,time_stddev,iterations_per_sec,result"` |
| `BENCHMARK_MAGIC` | `[u8; 4]` | `[b'B', b'M', b'K', 0x01]` |
| `BENCHMARK_VERSION` | `u16` | `5` |
| `BENCHMARK_HEADER_SIZE` | `usize` | `216` |

### Functions

| Function | Signature |
|---|---|
| `bench` | `pub fn bench<'a, T, F: Fn() -> T>(experiment_name: &'a str, precision: &'a str, iterations: u64, f: F) -> BenchmarkMetrics<'a>` |
| `BenchmarkMetrics::to_csv_row` | `pub fn to_csv_row(&self) -> String` |

---

## Simulation API (7 items)

### Structs

| Struct | Fields |
|---|---|
| `SimState` | `positions: Vec<[f64; 3]>`, `velocities: Vec<[f64; 3]>`, `time: f64`, `step: u64` |
| `SimConfig` | `dt: f64`, `max_steps: u64`, `body_count: usize` |
| `SimResult` | `final_state: SimState`, `total_steps: u64`, `elapsed_ms: u64`, `avg_step_ns: f64` |

### Traits

| Trait | Method |
|---|---|
| `StepFn` | `fn integrate(&self, state: &mut SimState, dt: f64)` |
| `RenderSink` | `fn frame(&mut self, state: &SimState)` |

### Functions

| Function | Signature |
|---|---|
| `SimState::new` | `pub fn new(body_count: usize) -> Self` |
| `run` | `pub fn run<S: StepFn, R: RenderSink>(config: &SimConfig, step_fn: &S, sink: &mut R, state: &mut SimState) -> SimResult` |
| `run_headless` | `pub fn run_headless<S: StepFn>(config: &SimConfig, step_fn: &S, state: &mut SimState) -> SimResult` |

---

## Encode API (2 items)

| Function | Signature |
|---|---|
| `encoded_size` | `pub fn encoded_size(metrics: &BenchmarkMetrics<'_>) -> Option<usize>` |
| `encode` | `pub fn encode(metrics: &BenchmarkMetrics<'_>, out: &mut [u8]) -> Result<usize, BenchmarkEncodeError>` |

---

## Decode API (1 item)

| Function | Signature |
|---|---|
| `decode` | `pub fn decode<'a>(bytes: &'a [u8]) -> Result<BenchmarkMetrics<'a>, BenchmarkEncodeError>` |

---

## Report API (10 items)

### Structs

| Struct | Fields |
|---|---|
| `BenchmarkReport` | `csv: String`, `markdown: String`, `html: String` |

### Functions

| Function | Signature |
|---|---|
| `generate` | `pub fn generate(metrics: &BenchmarkMetrics<'_>, result: &str) -> BenchmarkReport` |
| `generate_csv` | `pub fn generate_csv(metrics: &BenchmarkMetrics<'_>, result: &str) -> String` |
| `generate_json` | `pub fn generate_json(metrics: &BenchmarkMetrics<'_>, result: &str) -> String` |
| `generate_json_tagged` | `pub fn generate_json_tagged(metrics: &BenchmarkMetrics<'_>, result: &str, tags: &[(&str, &str)]) -> String` |
| `generate_yaml` | `pub fn generate_yaml(metrics: &BenchmarkMetrics<'_>, result: &str) -> String` |
| `generate_yaml_tagged` | `pub fn generate_yaml_tagged(metrics: &BenchmarkMetrics<'_>, result: &str, tags: &[(&str, &str)]) -> String` |
| `generate_toml` | `pub fn generate_toml(metrics: &BenchmarkMetrics<'_>, result: &str) -> String` |
| `generate_toml_tagged` | `pub fn generate_toml_tagged(metrics: &BenchmarkMetrics<'_>, result: &str, tags: &[(&str, &str)]) -> String` |
| `sanitize_filename` | `pub fn sanitize_filename(name: &str) -> String` |

---

## Export API (7 items)

### Constants

| Constant | Type | Value |
|---|---|---|
| `PTABLE_CATS` | `[&str; 10]` | `["Nonmetal", "Noble Gas", "Alkali Metal", "Alkaline Earth", "Transition Metal", "Post-Transition", "Metalloid", "Halogen", "Lanthanide", "Actinide"]` |

### Structs

| Struct | Fields |
|---|---|
| `GroupStats<'a>` | `groups: Vec<&'a str>`, `counts: BTreeMap<&'a str, usize>`, `sums: BTreeMap<&'a str, f32>`, `mins: BTreeMap<&'a str, f32>`, `maxs: BTreeMap<&'a str, f32>` |
| `Entry<'a>` | `metrics: &'a BenchmarkMetrics<'a>`, `result: &'a str`, `label: &'a str`, `tags: Vec<(&'a str, &'a str)>`, `grid_row: Option<u8>`, `grid_col: Option<u8>` |
| `ExportSummary` | `files_written: usize`, `html_size: usize`, `md_size: usize` |

### Functions

| Function | Signature |
|---|---|
| `compute_group_stats` | `pub fn compute_group_stats<'a>(entries: &[Entry<'a>]) -> GroupStats<'a>` |
| `capitalize_first` | `pub fn capitalize_first(s: &str) -> String` |
| `export` | `pub fn export(title: &str, entries: &[Entry<'_>], dir: &Path) -> std::io::Result<ExportSummary>` |
