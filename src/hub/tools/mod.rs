//! Internal hub tooling: memory arena, benchmarking, configuration,
//! determinism, logging, metrics, profiling, utilities, validation, and visualization.

/// Bump-allocator arena for temporary allocations (slices, matrices, scratch pools).
pub mod arena;
/// Benchmarking harness: timed experiment execution with CSV output.
pub mod benchmark;
/// Runtime configuration (iterations, precision, output paths).
pub mod config;
/// Deterministic computation: reproducible RNG, Kahan summation, fingerprinting.
pub mod deterministic;
/// Structured logging with configurable severity levels.
pub mod logger;
/// Metrics collection: counters, timers, histograms.
pub mod metrics;
/// Profiling: per-experiment and batch timing with statistical reports.
pub mod profiler;
/// General-purpose helpers: linspace, logspace, formatting, approximate equality.
pub mod utils;
/// Scientific validation pipeline: reference cases, monotonicity checks, NaN safety, blocking thresholds.
pub mod validation;
/// Chart generation: line, scatter, bar, histogram, heatmap (SVG output).
pub mod visualization;
