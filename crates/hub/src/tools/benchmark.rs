//! Timed experiment execution with CSV benchmark output.
//!
//! [`run_timed`] measures the execution time of an [`Experiment`]
//! and returns the result alongside a CSV metrics line.

use sciforge_benchmark::engine::{BenchmarkMetrics, CSV_HEADER};
use crate::domain::common::errors::HubResult;
use crate::engine::experience::experiment::Experiment;
use crate::engine::experience::runner::{ExperimentRunner, RunOutput};
use std::time::Instant;

/// Runs `experiment` with timing and returns `(result, csv)`.
pub fn run_timed(experiment: &Experiment) -> HubResult<(RunOutput, String)> {
    let runner = ExperimentRunner::new();
    let start = Instant::now();
    let result = runner.run(experiment)?;
    let elapsed = start.elapsed();
    let name: String =
        format!("{:?}/{}", experiment.domain, experiment.function_name).to_lowercase();
    let metrics = BenchmarkMetrics {
        experiment_name: &name,
        precision: "f64",
        elapsed_ms: elapsed.as_millis() as u64,
        iterations: 1,
        input_samples: 0,
        avg_time_ns: elapsed.as_nanos() as f64,
        last_time_ns: elapsed.as_nanos() as f64,
        output_bytes: 0,
        total_flops: 0,
        step_count: 0,
        input_dim: 0,
        output_dim: 0,
        benchmark_flags: 0,
        input_bytes: 0,
        result_bytes: 0,
        min_time_ns: elapsed.as_nanos() as f64,
        max_time_ns: elapsed.as_nanos() as f64,
        time_stddev: 0.0,
        iterations_per_sec: if elapsed.as_millis() > 0 {
            1000.0 / elapsed.as_millis() as f64
        } else {
            0.0
        },
        samples_per_sec: 0.0,
        eval_error: 0.0,
        eval_accuracy: 0.0,
        eval_r_squared: 0.0,
        eval_mae: 0.0,
        eval_samples: 0,
        eval_dataset_hash: 0,
        logical_cores: 0,
        avg_frequency_mhz: 0,
        max_frequency_mhz: 0,
        max_workers: 0,
        target_cpu_utilization: 0.0,
    };
    let csv = format!("{}\n{}", CSV_HEADER, metrics.to_csv_row());
    Ok((result, csv))
}
