use super::metrics::BenchmarkMetrics;
use std::hint::black_box;
use std::time::Instant;

pub fn bench<'a, T, F: Fn() -> T>(
    experiment_name: &'a str,
    precision: &'a str,
    iterations: u64,
    f: F,
) -> BenchmarkMetrics<'a> {
    let start = Instant::now();
    for _ in 0..iterations {
        black_box(f());
    }
    let elapsed = start.elapsed();
    let elapsed_ms = elapsed.as_millis() as u64;
    let avg_ns = elapsed.as_nanos() as f64 / iterations as f64;
    let ips = if elapsed_ms > 0 {
        iterations as f64 * 1000.0 / elapsed_ms as f64
    } else {
        0.0
    };
    BenchmarkMetrics {
        experiment_name,
        precision,
        elapsed_ms,
        iterations,
        input_samples: 0,
        avg_time_ns: avg_ns,
        last_time_ns: avg_ns,
        output_bytes: 0,
        total_flops: 0,
        step_count: 0,
        input_dim: 0,
        output_dim: 0,
        benchmark_flags: 0,
        input_bytes: 0,
        result_bytes: 0,
        min_time_ns: avg_ns,
        max_time_ns: avg_ns,
        time_stddev: 0.0,
        iterations_per_sec: ips,
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
    }
}
