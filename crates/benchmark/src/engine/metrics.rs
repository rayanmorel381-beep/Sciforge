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
