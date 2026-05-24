use crate::engine::{
    BENCHMARK_HEADER_SIZE, BENCHMARK_MAGIC, BENCHMARK_VERSION, BenchmarkEncodeError,
    BenchmarkMetrics,
};

pub fn decode_full<'a>(bytes: &'a [u8]) -> Result<BenchmarkMetrics<'a>, BenchmarkEncodeError> {
    if bytes.len() < BENCHMARK_HEADER_SIZE {
        return Err(BenchmarkEncodeError::InvalidFormat);
    }
    if bytes[0..4] != BENCHMARK_MAGIC {
        return Err(BenchmarkEncodeError::InvalidFormat);
    }
    let version = u16::from_le_bytes([bytes[4], bytes[5]]);
    if version != BENCHMARK_VERSION {
        return Err(BenchmarkEncodeError::InvalidFormat);
    }

    let elapsed_ms = u64::from_le_bytes(
        bytes[8..16]
            .try_into()
            .map_err(|_| BenchmarkEncodeError::InvalidFormat)?,
    );
    let iterations = u64::from_le_bytes(
        bytes[16..24]
            .try_into()
            .map_err(|_| BenchmarkEncodeError::InvalidFormat)?,
    );
    let input_samples = u64::from_le_bytes(
        bytes[24..32]
            .try_into()
            .map_err(|_| BenchmarkEncodeError::InvalidFormat)?,
    );
    let avg_time_ns = f64::from_le_bytes(
        bytes[32..40]
            .try_into()
            .map_err(|_| BenchmarkEncodeError::InvalidFormat)?,
    );
    let last_time_ns = f64::from_le_bytes(
        bytes[40..48]
            .try_into()
            .map_err(|_| BenchmarkEncodeError::InvalidFormat)?,
    );
    let output_bytes = u64::from_le_bytes(
        bytes[48..56]
            .try_into()
            .map_err(|_| BenchmarkEncodeError::InvalidFormat)?,
    ) as usize;
    let total_flops = u64::from_le_bytes(
        bytes[56..64]
            .try_into()
            .map_err(|_| BenchmarkEncodeError::InvalidFormat)?,
    );
    let step_count = u32::from_le_bytes(
        bytes[64..68]
            .try_into()
            .map_err(|_| BenchmarkEncodeError::InvalidFormat)?,
    );
    let input_dim = u32::from_le_bytes(
        bytes[68..72]
            .try_into()
            .map_err(|_| BenchmarkEncodeError::InvalidFormat)?,
    );
    let output_dim = u32::from_le_bytes(
        bytes[72..76]
            .try_into()
            .map_err(|_| BenchmarkEncodeError::InvalidFormat)?,
    );
    let benchmark_flags = u64::from_le_bytes(
        bytes[76..84]
            .try_into()
            .map_err(|_| BenchmarkEncodeError::InvalidFormat)?,
    );
    let input_bytes = u64::from_le_bytes(
        bytes[84..92]
            .try_into()
            .map_err(|_| BenchmarkEncodeError::InvalidFormat)?,
    );
    let result_bytes = u64::from_le_bytes(
        bytes[92..100]
            .try_into()
            .map_err(|_| BenchmarkEncodeError::InvalidFormat)?,
    );
    let min_time_ns = f64::from_le_bytes(
        bytes[100..108]
            .try_into()
            .map_err(|_| BenchmarkEncodeError::InvalidFormat)?,
    );
    let max_time_ns = f64::from_le_bytes(
        bytes[108..116]
            .try_into()
            .map_err(|_| BenchmarkEncodeError::InvalidFormat)?,
    );
    let time_stddev = f64::from_le_bytes(
        bytes[116..124]
            .try_into()
            .map_err(|_| BenchmarkEncodeError::InvalidFormat)?,
    );
    let iterations_per_sec = f64::from_le_bytes(
        bytes[124..132]
            .try_into()
            .map_err(|_| BenchmarkEncodeError::InvalidFormat)?,
    );
    let samples_per_sec = f64::from_le_bytes(
        bytes[132..140]
            .try_into()
            .map_err(|_| BenchmarkEncodeError::InvalidFormat)?,
    );
    let eval_error = f64::from_le_bytes(
        bytes[140..148]
            .try_into()
            .map_err(|_| BenchmarkEncodeError::InvalidFormat)?,
    );
    let eval_accuracy = f64::from_le_bytes(
        bytes[148..156]
            .try_into()
            .map_err(|_| BenchmarkEncodeError::InvalidFormat)?,
    );
    let eval_r_squared = f64::from_le_bytes(
        bytes[156..164]
            .try_into()
            .map_err(|_| BenchmarkEncodeError::InvalidFormat)?,
    );
    let eval_mae = f64::from_le_bytes(
        bytes[164..172]
            .try_into()
            .map_err(|_| BenchmarkEncodeError::InvalidFormat)?,
    );
    let eval_samples = u64::from_le_bytes(
        bytes[172..180]
            .try_into()
            .map_err(|_| BenchmarkEncodeError::InvalidFormat)?,
    );
    let eval_dataset_hash = u64::from_le_bytes(
        bytes[180..188]
            .try_into()
            .map_err(|_| BenchmarkEncodeError::InvalidFormat)?,
    );
    let logical_cores = u32::from_le_bytes(
        bytes[188..192]
            .try_into()
            .map_err(|_| BenchmarkEncodeError::InvalidFormat)?,
    );
    let avg_frequency_mhz = u32::from_le_bytes(
        bytes[192..196]
            .try_into()
            .map_err(|_| BenchmarkEncodeError::InvalidFormat)?,
    );
    let max_frequency_mhz = u32::from_le_bytes(
        bytes[196..200]
            .try_into()
            .map_err(|_| BenchmarkEncodeError::InvalidFormat)?,
    );
    let max_workers = u32::from_le_bytes(
        bytes[200..204]
            .try_into()
            .map_err(|_| BenchmarkEncodeError::InvalidFormat)?,
    );
    let target_cpu_utilization = f64::from_le_bytes(
        bytes[204..212]
            .try_into()
            .map_err(|_| BenchmarkEncodeError::InvalidFormat)?,
    );
    let name_len = u16::from_le_bytes(
        bytes[212..214]
            .try_into()
            .map_err(|_| BenchmarkEncodeError::InvalidFormat)?,
    ) as usize;
    let prec_len = u16::from_le_bytes(
        bytes[214..216]
            .try_into()
            .map_err(|_| BenchmarkEncodeError::InvalidFormat)?,
    ) as usize;

    let name_end = BENCHMARK_HEADER_SIZE
        .checked_add(name_len)
        .ok_or(BenchmarkEncodeError::InvalidFormat)?;
    let prec_end = name_end
        .checked_add(prec_len)
        .ok_or(BenchmarkEncodeError::InvalidFormat)?;
    if prec_end > bytes.len() {
        return Err(BenchmarkEncodeError::InvalidFormat);
    }

    let experiment_name = core::str::from_utf8(&bytes[BENCHMARK_HEADER_SIZE..name_end])
        .map_err(|_| BenchmarkEncodeError::InvalidFormat)?;
    let precision = core::str::from_utf8(&bytes[name_end..prec_end])
        .map_err(|_| BenchmarkEncodeError::InvalidFormat)?;

    Ok(BenchmarkMetrics {
        experiment_name,
        precision,
        elapsed_ms,
        iterations,
        input_samples,
        avg_time_ns,
        last_time_ns,
        output_bytes,
        total_flops,
        step_count,
        input_dim,
        output_dim,
        benchmark_flags,
        input_bytes,
        result_bytes,
        min_time_ns,
        max_time_ns,
        time_stddev,
        iterations_per_sec,
        samples_per_sec,
        eval_error,
        eval_accuracy,
        eval_r_squared,
        eval_mae,
        eval_samples,
        eval_dataset_hash,
        logical_cores,
        avg_frequency_mhz,
        max_frequency_mhz,
        max_workers,
        target_cpu_utilization,
    })
}
