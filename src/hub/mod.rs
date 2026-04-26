//! Central hub orchestrating all Sciforge subsystems.
//!
//! This module wires together the [`api`] layer (REST / CLI), the scientific
//! [`domain`] implementations, the computation [`engine`], and the shared
//! [`tools`] (benchmarking, profiling, validation, …).

/// REST/CLI interface layer: routing, DTOs, HTTP and CLI adapters.
pub mod api;
/// Scientific domain implementations: maths, physics, chemistry, biology, astronomy, geology, meteorology.
pub mod domain;
/// Computation engine: dispatch, experiments, campaigns, pipelines, simulations, queries.
pub mod engine;
/// Ergonomic re-exports of the most commonly used Hub types and functions.
pub mod prelude;
/// Toolbox: arena allocator, benchmarking, configuration, deterministic RNG, logging, profiling, validation, visualization.
pub mod tools;
