use crate::engine::{
    BENCHMARK_MAGIC, BENCHMARK_VERSION, BenchmarkEncodeError, BenchmarkMetrics,
};

pub fn decode_compact(bytes: &[u8]) -> Result<BenchmarkMetrics<'_>, BenchmarkEncodeError> {
    if bytes.len() < 32 {
        return Err(BenchmarkEncodeError::InvalidFormat);
    }
    if bytes[0..4] != BENCHMARK_MAGIC {
        return Err(BenchmarkEncodeError::InvalidFormat);
    }
    let version = u16::from_le_bytes([bytes[4], bytes[5]]);
    if version != BENCHMARK_VERSION {
        return Err(BenchmarkEncodeError::InvalidFormat);
    }
    let step_count = bytes[7] as u32;
    let input_dim = u32::from_le_bytes(
        bytes[8..12]
            .try_into()
            .map_err(|_| BenchmarkEncodeError::InvalidFormat)?,
    );
    let output_dim = u32::from_le_bytes(
        bytes[12..16]
            .try_into()
            .map_err(|_| BenchmarkEncodeError::InvalidFormat)?,
    );
    let total_flops = u32::from_le_bytes(
        bytes[16..20]
            .try_into()
            .map_err(|_| BenchmarkEncodeError::InvalidFormat)?,
    ) as u64;
    let last_time_ns = f64::from_le_bytes(
        bytes[24..32]
            .try_into()
            .map_err(|_| BenchmarkEncodeError::InvalidFormat)?,
    );
    let iterations = u32::from_le_bytes(
        bytes[32..36]
            .try_into()
            .map_err(|_| BenchmarkEncodeError::InvalidFormat)?,
    ) as u64;

    Ok(BenchmarkMetrics {
        experiment_name: "",
        precision: "",
        elapsed_ms: 0,
        iterations,
        input_samples: 0,
        avg_time_ns: 0.0,
        last_time_ns,
        output_bytes: 0,
        total_flops,
        step_count,
        input_dim,
        output_dim,
        benchmark_flags: 0,
        input_bytes: 0,
        result_bytes: 0,
        min_time_ns: 0.0,
        max_time_ns: 0.0,
        time_stddev: 0.0,
        iterations_per_sec: 0.0,
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
    })
}
