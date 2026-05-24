# Benchmark — ChangeLog

---

## v0.0.4

### Precision Upgrade — `BenchmarkMetrics` — f32 → f64 (12 fields)

All timing and performance fields in `BenchmarkMetrics` migrated from `f32` to `f64` for full double-precision accuracy.

| Change | Detail |
|---|---|
| `BENCHMARK_VERSION` | 4 → 5 (binary format version bump) |
| `BENCHMARK_HEADER_SIZE` | 168 → 216 bytes (12 × 4 extra bytes from f32 → f64) |
| Fields migrated | `avg_time_ns`, `last_time_ns`, `min_time_ns`, `max_time_ns`, `time_stddev`, `iterations_per_sec`, `samples_per_sec`, `eval_error`, `eval_accuracy`, `eval_r_squared`, `eval_mae`, `target_cpu_utilization` |
| Files modified | `engine.rs`, `encode.rs`, `decode.rs`, `export/mod.rs`, `export/md.rs`, `export/html/charts.rs` |
| Tests updated | `tests/benchmark/engine.rs` — assertions for version 5 and header size 216 |
| Stress test updated | `tests/stress/main.rs` — `metric()` signature and calculations `f32` → `f64` |

---

## v0.0.3

### Refactoring — `benchmark` — export.rs → export/ (8 files)

Monolithic `export.rs` split into `export/` module with per-format files.

| Change | Detail |
|---|---|
| `export.rs` → `export/` | 8 files: `mod.rs`, `bmk.rs`, `csv.rs`, `html.rs`, `json.rs`, `md.rs`, `toml.rs`, `yaml.rs` |
| Modified | `decode.rs`, `encode.rs`, `engine.rs` |

No new functions — see `testing.md` for test details.

---

## v0.0.2

### Changes — `benchmark` — 7 files modified

| Change | Detail |
|---|---|
| Files modified | `decode.rs`, `encode.rs`, `engine.rs`, `export.rs`, `report.rs`, `simulation.rs`, `mod.rs` |

---

## v0.0.1

### 1️⃣ Engine — `benchmark::engine` — 2 pub fn + 1 pub struct + 1 pub enum + 4 pub const

| Item | Declaration | Description | Module |
|---|---|---|---|
| `BenchmarkMetrics<'a>` | `pub struct` | 30 fields: `experiment_name`, `precision`, `elapsed_ms`, `iterations`, `input_samples`, `avg_time_ns`, `last_time_ns`, `output_bytes`, `total_flops`, `step_count`, `input_dim`, `output_dim`, `benchmark_flags`, `input_bytes`, `result_bytes`, `min_time_ns`, `max_time_ns`, `time_stddev`, `iterations_per_sec`, `samples_per_sec`, `eval_error`, `eval_accuracy`, `eval_r_squared`, `eval_mae`, `eval_samples`, `eval_dataset_hash`, `logical_cores`, `avg_frequency_mhz`, `max_frequency_mhz`, `max_workers`, `target_cpu_utilization` | `benchmark::engine` |
| `BenchmarkEncodeError` | `pub enum` | `BufferTooSmall`, `InvalidFormat` | `benchmark::engine` |
| `CSV_HEADER` | `pub const CSV_HEADER: &str` | CSV header row | `benchmark::engine` |
| `BENCHMARK_MAGIC` | `pub const BENCHMARK_MAGIC: [u8; 4]` | Binary .bmk magic bytes | `benchmark::engine` |
| `BENCHMARK_VERSION` | `pub const BENCHMARK_VERSION: u16` | Binary format version | `benchmark::engine` |
| `BENCHMARK_HEADER_SIZE` | `pub const BENCHMARK_HEADER_SIZE: usize` | Header byte count | `benchmark::engine` |

| Function | Signature | Description | Module |
|---|---|---|---|
| Benchmark runner | `bench<'a, T, F: Fn() → T>(experiment_name: &'a str, precision: &'a str, iterations: u64, f: F) → BenchmarkMetrics<'a>` | Run benchmark with timing | `benchmark::engine` |
| CSV row | `BenchmarkMetrics::to_csv_row(&self) → String` | Metrics as CSV row | `benchmark::engine` |

### 2️⃣ Encode — `benchmark::encode` — 2 pub fn

| Function | Signature | Description | Module |
|---|---|---|---|
| Encoded size | `encoded_size(metrics: &BenchmarkMetrics<'_>) → Option<usize>` | Compute buffer size | `benchmark::encode` |
| Encode | `encode(metrics: &BenchmarkMetrics<'_>, out: &mut [u8]) → Result<usize, BenchmarkEncodeError>` | Binary .bmk serialization | `benchmark::encode` |

### 3️⃣ Decode — `benchmark::decode` — 1 pub fn

| Function | Signature | Description | Module |
|---|---|---|---|
| Decode | `decode<'a>(bytes: &'a [u8]) → Result<BenchmarkMetrics<'a>, BenchmarkEncodeError>` | Binary .bmk deserialization | `benchmark::decode` |

### 4️⃣ Simulation — `benchmark::simulation` — 3 pub fn + 3 pub struct + 2 pub trait

| Item | Declaration | Description | Module |
|---|---|---|---|
| `SimState` | `pub struct` | Fields: `positions: Vec<[f64; 3]>`, `velocities: Vec<[f64; 3]>`, `time: f64`, `step: u64` | `benchmark::simulation` |
| `SimConfig` | `pub struct` | Fields: `dt: f64`, `max_steps: u64`, `body_count: usize` | `benchmark::simulation` |
| `SimResult` | `pub struct` | Fields: `final_state: SimState`, `total_steps: u64`, `elapsed_ms: u64`, `avg_step_ns: f64` | `benchmark::simulation` |
| `StepFn` | `pub trait` | `fn integrate(&self, state: &mut SimState, dt: f64)` | `benchmark::simulation` |
| `RenderSink` | `pub trait` | `fn frame(&mut self, state: &SimState)` | `benchmark::simulation` |

| Function | Signature | Description | Module |
|---|---|---|---|
| New state | `SimState::new(body_count: usize) → Self` | Initialize simulation state | `benchmark::simulation` |
| Run simulation | `run<S: StepFn, R: RenderSink>(config: &SimConfig, step_fn: &S, sink: &mut R, state: &mut SimState) → SimResult` | Full simulation with rendering | `benchmark::simulation` |
| Run headless | `run_headless<S: StepFn>(config: &SimConfig, step_fn: &S, state: &mut SimState) → SimResult` | Simulation without rendering | `benchmark::simulation` |

### 5️⃣ Report — `benchmark::report` — 9 pub fn + 1 pub struct

| Item | Declaration | Description | Module |
|---|---|---|---|
| `BenchmarkReport` | `pub struct` | Fields: `csv: String`, `markdown: String`, `html: String` | `benchmark::report` |

| Function | Signature | Description | Module |
|---|---|---|---|
| Generate all | `generate(metrics: &BenchmarkMetrics<'_>, result: &str) → BenchmarkReport` | CSV + Markdown + HTML | `benchmark::report` |
| Generate CSV | `generate_csv(metrics: &BenchmarkMetrics<'_>, result: &str) → String` | CSV output | `benchmark::report` |
| Generate JSON | `generate_json(metrics: &BenchmarkMetrics<'_>, result: &str) → String` | JSON output | `benchmark::report` |
| Generate JSON tagged | `generate_json_tagged(metrics: &BenchmarkMetrics<'_>, result: &str, tags: &[(&str, &str)]) → String` | JSON with tags | `benchmark::report` |
| Generate YAML | `generate_yaml(metrics: &BenchmarkMetrics<'_>, result: &str) → String` | YAML output | `benchmark::report` |
| Generate YAML tagged | `generate_yaml_tagged(metrics: &BenchmarkMetrics<'_>, result: &str, tags: &[(&str, &str)]) → String` | YAML with tags | `benchmark::report` |
| Generate TOML | `generate_toml(metrics: &BenchmarkMetrics<'_>, result: &str) → String` | TOML output | `benchmark::report` |
| Generate TOML tagged | `generate_toml_tagged(metrics: &BenchmarkMetrics<'_>, result: &str, tags: &[(&str, &str)]) → String` | TOML with tags | `benchmark::report` |
| Sanitize filename | `sanitize_filename(name: &str) → String` | Safe filesystem name | `benchmark::report` |

### 6️⃣ Export — `benchmark::export` — 3 pub fn + 3 pub struct + 1 pub const

| Item | Declaration | Description | Module |
|---|---|---|---|
| `PTABLE_CATS` | `pub const PTABLE_CATS: [&str; 10]` | Periodic table categories | `benchmark::export` |
| `GroupStats<'a>` | `pub struct` | Fields: `groups: Vec<&'a str>`, `counts: BTreeMap<&'a str, usize>`, `sums/mins/maxs: BTreeMap<&'a str, f32>` | `benchmark::export` |
| `Entry<'a>` | `pub struct` | Fields: `metrics`, `result`, `label`, `tags`, `grid_row`, `grid_col` | `benchmark::export` |
| `ExportSummary` | `pub struct` | Fields: `files_written: usize`, `html_size: usize`, `md_size: usize` | `benchmark::export` |

| Function | Signature | Description | Module |
|---|---|---|---|
| Group statistics | `compute_group_stats<'a>(entries: &[Entry<'a>]) → GroupStats<'a>` | Aggregate by category | `benchmark::export` |
| Capitalize | `capitalize_first(s: &str) → String` | First letter uppercase | `benchmark::export` |
| Export all | `export(title: &str, entries: &[Entry<'_>], dir: &Path) → std::io::Result<ExportSummary>` | Multi-format export pipeline | `benchmark::export` |
