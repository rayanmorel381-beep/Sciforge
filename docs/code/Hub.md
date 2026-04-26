# Hub Source Code Guide

This page documents the source implementation behind `sciforge::hub`, including API layer, engine internals, dispatch routing, and tooling modules.

## Source Coverage

### What is explained
- File-level implementation layout in `src/hub/`.
- Main orchestration components and where they are implemented.
- Runtime call path from API entry points to domain dispatchers.

### Visibility and external access
- `hub` is public from `src/lib.rs` (`pub mod hub;`).
- External consumers should prefer `sciforge::hub::prelude::*` for stable entry points.
- This is the recommended external path to reach `pub(crate)` science domains.


## Source Code Explanation

### Relevant file structure
- Main implementation: `src/hub/`
- Module entry point: `src/hub/mod.rs`
- Hub routing (when applicable): `src/hub/engine/dispatch/*.rs`

### Internal execution flow
1. The module exposes subdomains through `mod.rs` and targeted `pub use` exports.
2. Each `.rs` file implements a coherent orchestration or tooling block.
3. The engine dispatchers route computation requests to domain implementations.

### What to check while reading code
- Exact signature: input/output types and argument order.
- Routing logic and parameter coercion in dispatch modules.
- Error propagation paths and domain boundary assumptions.

## Modules

- `api` — HTTP and CLI interfaces, DTOs, routes
- `domain` — `DomainType` enum and domain metadata
- `engine` — experiment execution, pipelines, task scheduling
	- `experience` — `Experiment` / `ExperimentRunner`
	- `pipeline` — `Pipeline` / `Stage` composition
	- `worker` — `Scheduler`, `TaskQueue`, `Executor`, `ExecutionContext`
	- `dispatch` — domain-specific dispatch handlers (astronomy, biology, chemistry, geology, maths, meteorology, physics)
- `tools` — numerical utilities, visualization, profiling, validation, logging
- `prelude` — convenience re-exports

---

## Prelude

```rust
use sciforge::hub::prelude::*;
```

Exports: `Experiment`, `ExperimentRunner`, `DomainType`, `ParameterValue`, `Pipeline`, `Stage`, `Scheduler`, `TaskQueue`, `Executor`.

---

## Engine — Experiment API

### Structs

| Struct | Description |
|---|---|
| `Experiment` | A named experiment with a domain, function name, and parameter map |
| `ExperimentRunner` | Runs an `Experiment` by dispatching to the correct domain handler |

### Usage

```rust
let exp = Experiment::new(DomainType::Physics, "lorentz_factor")
				.param("velocity", ParameterValue::Scalar(2.0e8));

let result = ExperimentRunner::new().run(&exp)?;
```

---

## Engine — Pipeline API

### Structs

| Struct | Description |
|---|---|
| `Pipeline` | An ordered sequence of `Stage` transforms applied to a data vector |
| `Stage` | A single named transform step (function `Vec<f64> -> Vec<f64>`) |

### Built-in stage constructors

- `normalize_stage()` — normalizes vector to [0, 1]
- `filter_positive_stage()` — keeps only positive values
- `scale_stage(factor: f64)` — multiplies each element by factor

### Usage

```rust
let pipeline = Pipeline::new()
		.add(normalize_stage())
		.add(scale_stage(100.0));

let output = pipeline.run(input_data);
```

---

## Engine — Worker / Scheduler API

### Structs

| Struct | Description |
|---|---|
| `Scheduler` | Manages a queue of `ScheduledTask` with priority and timing |
| `ScheduledTask` | A task with an execution time and priority |
| `Task` | A named unit of work with associated `TaskResult` |
| `TaskQueue` | FIFO/priority queue of tasks |
| `TaskResult` | Output and status of a completed task |
| `ExecutionContext` | Runtime context passed to each task during execution |
| `Executor` | Drives task execution loop, dispatching from a `TaskQueue` |

---

## Tools — Visualization

Generate ASCII/text-based plots for quick inspection:

- `line_chart(data: &[f64], title: &str)` — ASCII line chart
- `bar_chart(data: &[f64], labels: &[&str])` — ASCII bar chart
- `scatter_plot(xs: &[f64], ys: &[f64])` — ASCII scatter plot
- `histogram(data: &[f64], bins: usize)` — ASCII histogram
- `heatmap(matrix: &[Vec<f64>])` — ASCII heatmap

---

## Tools — Numerical Utilities

- `kahan_sum(data: &[f64])` — compensated summation (Kahan)
- `kahan_dot(a: &[f64], b: &[f64])` — compensated dot product
- `fingerprint(data: &[f64])` — u64 hash fingerprint of a float slice
- `fingerprint_scalar(v: f64)` — u64 hash of a scalar
- `linspace(start, end, n)` → `Vec<f64>` — evenly spaced points
- `logspace(start_exp, end_exp, n)` → `Vec<f64>` — logarithmically spaced points
- `normalize(data: &[f64])` → `Vec<f64>` — rescale to [0, 1]
- `moving_average(data: &[f64], window: usize)` → `Vec<f64>`
- `clamp_vec(data: &[f64], min, max)` → `Vec<f64>`
- `cumulative_sum(data: &[f64])` → `Vec<f64>`
- `relative_error(computed, reference)` → `f64`
- `scalar_value(v: f64)` — wraps a scalar for use with the pipeline

---

## Tools — Formatting

- `format_scientific(v: f64, precision: usize)` → `String` — e.g. `"3.14e+00"`
- `format_si(v: f64, unit: &str)` → `String` — SI prefix scaling (k, M, G, …)
- `format_ns(duration_ns: u64)` → `String` — nanoseconds as human-readable duration

---

## Tools — Profiling

- `profile_experiment(exp: &Experiment, runner: &ExperimentRunner)` — time a single experiment
- `profile_batch(exps: &[Experiment], runner: &ExperimentRunner)` — time multiple experiments
- `quick_profile<F: Fn()>(f: F)` → `Duration` — time any closure
- `run_timed<F: Fn() -> T>(f: F)` → `(T, Duration)` — run and return result + duration

---

## Tools — Validation

- `run_validation(data: &[f64])` — run standard suite of checks
- `check_monotonicity(data: &[f64])` — assert data is non-decreasing
- `check_nan_safety(data: &[f64])` — assert no NaN or Inf values
- `assert_reproducible(f: F, n: usize)` — run f n times, assert identical results
- `compare_entries(a: &[f64], b: &[f64], tol: f64)` — element-wise comparison
- `approx_equal(a: f64, b: f64, tol: f64)` → `bool`

---

## Tools — Logging

Thread-safe, level-gated logger:

- `info(msg: &str)`, `warn(msg: &str)`, `error(msg: &str)`
- `debug(msg: &str)`, `trace(msg: &str)`
- `log(level: LogLevel, msg: &str)` — generic log call
- `set_level(level: LogLevel)` — configure minimum log level
- `current_level()` → `LogLevel`
- `usage()` — print module usage summary

---

## Tools — Reporting

- `report_to_latex(metrics)` → `String` — LaTeX table from benchmark metrics
- `report_to_tsv(metrics)` → `String` — TSV table from benchmark metrics

---

## Domain Dispatch

The `DomainType` enum routes function calls to the correct scientific module:

| Variant | Dispatches to |
|---|---|
| `DomainType::Astronomy` | `src/hub/engine/dispatch/astronomy.rs` |
| `DomainType::Biology` | `src/hub/engine/dispatch/biology.rs` |
| `DomainType::Chemistry` | `src/hub/engine/dispatch/chemistry.rs` |
| `DomainType::Geology` | `src/hub/engine/dispatch/geology.rs` |
| `DomainType::Maths` | `src/hub/engine/dispatch/maths.rs` |
| `DomainType::Meteorology` | `src/hub/engine/dispatch/meteorology.rs` |
| `DomainType::Physics` | `src/hub/engine/dispatch/physics.rs` |
