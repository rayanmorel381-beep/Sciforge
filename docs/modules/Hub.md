# Hub Module

The Hub is SciForge's **central orchestration layer**. It connects all scientific modules through a unified API, providing experiment dispatch, simulation execution, campaign management, and tooling infrastructure. It is the entry point for running computations programmatically or via HTTP.

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


## Architecture

The Hub follows a **layered hexagonal architecture**:

```
Hub
├── API           — External interface (CLI, HTTP, DTOs)
├── Domain        — Shared domain concepts (constants, errors, units)
├── Engine        — Computation core (dispatch, experiments, simulations, workers)
├── Tools         — Infrastructure (benchmarking, config, logging, metrics)
└── Prelude       — Ergonomic re-exports for common usage
```

The data flow for a computation request:

```
ComputeRequest → Router → Domain Dispatcher → Scientific Module → ComputeResponse
```

---

## 1. API

External interface layer for interacting with SciForge computations.

### HTTP Server

A minimal TCP-based HTTP server built on `std::net::TcpListener` (no framework dependencies). The server:
- Binds to a configurable address
- Parses raw HTTP requests (method, path, body)
- Delegates to a handler function `Fn(&str, &str, &str) -> HttpResponse`
- Returns responses with proper HTTP/1.1 headers (status, content-type, content-length)

```rust
pub struct HttpResponse {
    pub status_code: u16,
    pub status_text: String,
    pub content_type: String,  // "application/json" by default
    pub body: String,
}
```

### DTOs (Data Transfer Objects)

**`ComputeRequest`** — Structured experiment request:
```rust
pub struct ComputeRequest {
    pub domain: String,       // "physics", "chemistry", etc.
    pub function: String,     // "projectile_motion", "nernst_potential", etc.
    pub params: HashMap<String, ParamValue>,
}
```

Parameter types: `Scalar(f64)`, `Integer(i64)`, `Text(String)`, `Boolean(bool)`, `Array(Vec<f64>)`. Builder pattern: `ComputeRequest::new("physics", "free_fall").with_scalar("height", 100.0)`.

**`ComputeResponse`** — Structured computation result:
```rust
pub struct ComputeResponse {
    pub success: bool,
    pub result: Option<ResultData>,
    pub error: Option<String>,
    pub elapsed_ms: f64,
}
```

Result variants: `Scalar`, `Pair`, `Triple`, `Vector`, `Matrix`, `TimeSeries { times, values }`, `Text`, `Boolean`. Factory methods: `ok_scalar()`, `ok_vector()`, `ok_time_series()`, `err()`.

### Routes

- **`formatter.rs`** — Formats `ComputeResponse` into different output targets (JSON, plain text, etc.)
- **`params.rs`** — Parses and validates incoming request parameters with type coercion

---

## 2. Domain

Shared domain concepts used across the Hub.

### Units

Strongly typed unit system with enumerations for physical quantities:

| Quantity | Variants |
|---|---|
| `LengthUnit` | Meter, Kilometer, Mile, AU, Parsec, LightYear, ... |
| `MassUnit` | Kilogram, Gram, SolarMass, ElectronMass, AMU, ... |
| `TimeUnit` | Second, Minute, Hour, Day, Year, ... |
| `TemperatureUnit` | Kelvin, Celsius, Fahrenheit |
| `EnergyUnit` | Joule, eV, keV, MeV, Calorie, ... |
| `PressureUnit` | Pascal, Atmosphere, Bar, ... |
| `AngleUnit` | Radian, Degree |

### Errors

Unified error handling: `HubError` enum with variants for invalid input, dispatch failures, simulation errors, etc. `HubResult<T> = Result<T, HubError>`.

---

## 3. Engine

The computational core of the Hub.

### Dispatch

Routes experiment requests to the correct scientific module. One dispatcher per domain:

| Dispatcher | Module | Description |
|---|---|---|
| `astronomy.rs` | Astronomy | Stellar, orbital, cosmology functions |
| `biology.rs` | Biology | 44 submodules, 176 source files |
| `chemistry.rs` | Chemistry | 26 submodules |
| `geology.rs` | Geology | Dating, petrology, seismology, tectonics |
| `maths.rs` | Mathematics | 17 submodules |
| `meteorology.rs` | Meteorology | Atmosphere, dynamics, precipitation, radiation |
| `physics.rs` | Physics | 11 submodules |

The astronomy dispatcher directly exposes the cosmology stack (for example `e_z`, `e_z_lcdm`, `e_z_wcdm`, `hubble_at_z`, `hubble_at_z_lcdm`, `comoving_distance_from_z`, `luminosity_distance_from_z`, `angular_diameter_distance_from_z`, and general/wCDM/CPL distance variants), so redshift-expansion and distance workflows are available through the standard Hub API.

**Parameter extraction** (`params.rs`) provides typed accessors from the parameter list:
- `get_f(params, "name")` → `HubResult<f64>` (scalar)
- `get_i(params, "name")` → `HubResult<i64>` (integer)
- `get_u(params, "name")` → `HubResult<usize>` (unsigned)
- `get_v(params, "name")` → `HubResult<&[f64]>` (vector)
- `get_m(params, "name")` → `HubResult<&[Vec<f64>]>` (matrix)
- `get_c(params, "name")` → `HubResult<Complex>` (complex number)

### Experiment

The `Experiment` struct is a chainable builder for computation requests:

```rust
pub enum DomainType { Math, Physics, Chemistry, Biology, Astronomy, Geology, Meteorology }

pub enum ParameterValue {
    Scalar(f64), Integer(i64), Vector(Vec<f64>), Matrix(Vec<Vec<f64>>),
    Boolean(bool), Text(String), Complex(f64, f64), ComplexVector(Vec<(f64, f64)>),
    Polynomial(Vec<f64>), IntVector(Vec<usize>), IntMatrix(Vec<Vec<usize>>),
    EdgeList(Vec<(usize, usize, f64)>),
    Sparse { rows, cols, row_ptr, col_idx, values },
    Tensor { data: Vec<f64>, shape: Vec<usize> },
}

// Usage:
let exp = Experiment::new(DomainType::Physics, "projectile_range")
    .param("velocity", ParameterValue::Scalar(50.0))
    .param("angle", ParameterValue::Scalar(45.0));
```

The **Runner** executes experiments and returns typed `RunOutput` variants.

### Simulation

Time-stepping numerical simulation framework.

**`DynamicalSystem` trait** — interface for any ODE system:
```rust
pub trait DynamicalSystem {
    fn dimension(&self) -> usize;
    fn derivatives(&self, t: f64, state: &[f64], out: &mut [f64]);
    fn initial_state(&self) -> Vec<f64>;
    fn time_span(&self) -> (f64, f64);
}
```

**Built-in models:**
- **`HarmonicOscillator`**: $\ddot{x} = -\omega^2 x$ (2D state: position + velocity)
- **`LotkaVolterra`**: predator-prey dynamics
- **`SimpleModel`**: generic system from a boxed closure

The **Integrator** advances the state forward using numerical methods (Euler, RK4, adaptive RK45). The **Solver** orchestrates the integration loop and collects `SimulationResult` time series.

### Pipeline

Multi-step computation workflows via the **Flow** module. Allows chaining computations where the output of one step feeds as input to the next.

### Worker

Concurrent execution infrastructure:

- **`Scheduler`** — Priority-based task scheduling with dependency resolution. Tasks have `Priority` levels (Low, Normal, High, Critical) and dependency lists. The scheduler performs topological sorting with priority ordering.
- **`Queue`** — Task queue management
- **`Executor`** — Dispatches tasks sequentially or in parallel
- **`Context`** — Carries execution state, parameters, and result collection

---

## 4. Campaign

Multi-experiment batch execution. A campaign collects multiple `Experiment` instances for sequential dispatch and result aggregation. Enables parameter sweeps (varying one parameter across a range) and combinatorial experiment generation.

---

## 5. Tools

Infrastructure utilities.

- **`Config`** — Key-value configuration store backed by `HashMap<String, String>`. Typed accessors: `get_f64()`, `get_usize()`, `get_bool()`. Builder: `Config::new().set("key", "value")`.
- **`Logger`** — Structured logging with severity levels (`Debug`, `Info`, `Warn`, `Error`). Prefixed output with timestamps for experiment tracing.
- **`Metrics`** — Runtime metrics collection: execution time, iteration counts, throughput measurements.
- **`Benchmark`** — Integration point with the Benchmark module for performance profiling of Hub operations.
- **`Utils`** — General-purpose utility functions.

---

## 6. Prelude

Convenience re-exports for ergonomic API usage:

```rust
pub use crate::hub::domain::common::errors::{HubError, HubResult};
pub use crate::hub::domain::common::units::*;
pub use crate::hub::tools::config::Config;
pub use crate::hub::tools::logger::Level;
pub use crate::hub::engine::experience::experiment::{DomainType, Experiment, ParameterValue};
pub use crate::hub::engine::simulation::model::DynamicalSystem;
pub use crate::hub::engine::simulation::result::SimulationResult;
pub use crate::hub::api::dto::request::ComputeRequest;
pub use crate::hub::api::dto::response::ComputeResponse;
```

Import with `use sciforge::hub::prelude::*` for immediate access to all Hub types.

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
