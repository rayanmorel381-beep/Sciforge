use std::hint::black_box;
use std::time::Instant;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BenchmarkMetrics<'a> {
    pub experiment_name: &'a str,
    pub precision: &'a str,
    pub elapsed_ms: u64,
    pub iterations: u64,
    pub input_samples: u64,
    pub avg_time_ns: f64,
    pub last_time_ns: f64,
    pub output_bytes: usize,
    pub total_flops: u64,
    pub step_count: u32,
    pub input_dim: u32,
    pub output_dim: u32,
    pub benchmark_flags: u64,
    pub input_bytes: u64,
    pub result_bytes: u64,
    pub min_time_ns: f64,
    pub max_time_ns: f64,
    pub time_stddev: f64,
    pub iterations_per_sec: f64,
    pub samples_per_sec: f64,
    pub eval_error: f64,
    pub eval_accuracy: f64,
    pub eval_r_squared: f64,
    pub eval_mae: f64,
    pub eval_samples: u64,
    pub eval_dataset_hash: u64,
    pub logical_cores: u32,
    pub avg_frequency_mhz: u32,
    pub max_frequency_mhz: u32,
    pub max_workers: u32,
    pub target_cpu_utilization: f64,
}

pub const CSV_HEADER: &str = "experiment_name,precision,elapsed_ms,iterations,avg_time_ns,min_time_ns,max_time_ns,time_stddev,iterations_per_sec,result";

impl BenchmarkMetrics<'_> {
    pub fn to_csv_row(&self) -> String {
        format!(
            "{},{},{},{},{},{},{},{},{}",
            self.experiment_name,
            self.precision,
            self.elapsed_ms,
            self.iterations,
            self.avg_time_ns,
            self.min_time_ns,
            self.max_time_ns,
            self.time_stddev,
            self.iterations_per_sec,
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BenchmarkEncodeError {
    BufferTooSmall,
    InvalidFormat,
}

pub const BENCHMARK_MAGIC: [u8; 4] = [b'B', b'M', b'K', 0x01];
pub const BENCHMARK_VERSION: u16 = 5;
pub const BENCHMARK_HEADER_SIZE: usize = 4
    + 2
    + 2
    + 8
    + 8
    + 8
    + 8
    + 8
    + 8
    + 8
    + 4
    + 4
    + 4
    + 8
    + 8
    + 8
    + 8
    + 8
    + 8
    + 8
    + 8
    + 8
    + 8
    + 8
    + 8
    + 8
    + 8
    + 4
    + 4
    + 4
    + 4
    + 8
    + 2
    + 2;

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
