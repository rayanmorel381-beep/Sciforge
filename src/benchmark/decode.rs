use super::engine::{
    BENCHMARK_HEADER_SIZE, BENCHMARK_MAGIC, BENCHMARK_VERSION, BenchmarkEncodeError,
    BenchmarkMetrics,
};

pub fn decode<'a>(bytes: &'a [u8]) -> Result<BenchmarkMetrics<'a>, BenchmarkEncodeError> {
    decode_full(bytes).or_else(|_| decode_compact(bytes))
}

fn decode_full<'a>(bytes: &'a [u8]) -> Result<BenchmarkMetrics<'a>, BenchmarkEncodeError> {
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
    let avg_time_ns = f32::from_le_bytes(
        bytes[32..36]
            .try_into()
            .map_err(|_| BenchmarkEncodeError::InvalidFormat)?,
    );
    let last_time_ns = f32::from_le_bytes(
        bytes[36..40]
            .try_into()
            .map_err(|_| BenchmarkEncodeError::InvalidFormat)?,
    );
    let output_bytes = u64::from_le_bytes(
        bytes[40..48]
            .try_into()
            .map_err(|_| BenchmarkEncodeError::InvalidFormat)?,
    ) as usize;
    let total_flops = u64::from_le_bytes(
        bytes[48..56]
            .try_into()
            .map_err(|_| BenchmarkEncodeError::InvalidFormat)?,
    );
    let step_count = u32::from_le_bytes(
        bytes[56..60]
            .try_into()
            .map_err(|_| BenchmarkEncodeError::InvalidFormat)?,
    );
    let input_dim = u32::from_le_bytes(
        bytes[60..64]
            .try_into()
            .map_err(|_| BenchmarkEncodeError::InvalidFormat)?,
    );
    let output_dim = u32::from_le_bytes(
        bytes[64..68]
            .try_into()
            .map_err(|_| BenchmarkEncodeError::InvalidFormat)?,
    );
    let benchmark_flags = u64::from_le_bytes(
        bytes[68..76]
            .try_into()
            .map_err(|_| BenchmarkEncodeError::InvalidFormat)?,
    );
    let input_bytes = u64::from_le_bytes(
        bytes[76..84]
            .try_into()
            .map_err(|_| BenchmarkEncodeError::InvalidFormat)?,
    );
    let result_bytes = u64::from_le_bytes(
        bytes[84..92]
            .try_into()
            .map_err(|_| BenchmarkEncodeError::InvalidFormat)?,
    );
    let min_time_ns = f32::from_le_bytes(
        bytes[92..96]
            .try_into()
            .map_err(|_| BenchmarkEncodeError::InvalidFormat)?,
    );
    let max_time_ns = f32::from_le_bytes(
        bytes[96..100]
            .try_into()
            .map_err(|_| BenchmarkEncodeError::InvalidFormat)?,
    );
    let time_stddev = f32::from_le_bytes(
        bytes[100..104]
            .try_into()
            .map_err(|_| BenchmarkEncodeError::InvalidFormat)?,
    );
    let iterations_per_sec = f32::from_le_bytes(
        bytes[104..108]
            .try_into()
            .map_err(|_| BenchmarkEncodeError::InvalidFormat)?,
    );
    let samples_per_sec = f32::from_le_bytes(
        bytes[108..112]
            .try_into()
            .map_err(|_| BenchmarkEncodeError::InvalidFormat)?,
    );
    let eval_error = f32::from_le_bytes(
        bytes[112..116]
            .try_into()
            .map_err(|_| BenchmarkEncodeError::InvalidFormat)?,
    );
    let eval_accuracy = f32::from_le_bytes(
        bytes[116..120]
            .try_into()
            .map_err(|_| BenchmarkEncodeError::InvalidFormat)?,
    );
    let eval_r_squared = f32::from_le_bytes(
        bytes[120..124]
            .try_into()
            .map_err(|_| BenchmarkEncodeError::InvalidFormat)?,
    );
    let eval_mae = f32::from_le_bytes(
        bytes[124..128]
            .try_into()
            .map_err(|_| BenchmarkEncodeError::InvalidFormat)?,
    );
    let eval_samples = u64::from_le_bytes(
        bytes[128..136]
            .try_into()
            .map_err(|_| BenchmarkEncodeError::InvalidFormat)?,
    );
    let eval_dataset_hash = u64::from_le_bytes(
        bytes[136..144]
            .try_into()
            .map_err(|_| BenchmarkEncodeError::InvalidFormat)?,
    );
    let logical_cores = u32::from_le_bytes(
        bytes[144..148]
            .try_into()
            .map_err(|_| BenchmarkEncodeError::InvalidFormat)?,
    );
    let avg_frequency_mhz = u32::from_le_bytes(
        bytes[148..152]
            .try_into()
            .map_err(|_| BenchmarkEncodeError::InvalidFormat)?,
    );
    let max_frequency_mhz = u32::from_le_bytes(
        bytes[152..156]
            .try_into()
            .map_err(|_| BenchmarkEncodeError::InvalidFormat)?,
    );
    let max_workers = u32::from_le_bytes(
        bytes[156..160]
            .try_into()
            .map_err(|_| BenchmarkEncodeError::InvalidFormat)?,
    );
    let target_cpu_utilization = f32::from_le_bytes(
        bytes[160..164]
            .try_into()
            .map_err(|_| BenchmarkEncodeError::InvalidFormat)?,
    );
    let name_len = u16::from_le_bytes(
        bytes[164..166]
            .try_into()
            .map_err(|_| BenchmarkEncodeError::InvalidFormat)?,
    ) as usize;
    let prec_len = u16::from_le_bytes(
        bytes[166..168]
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

fn decode_compact(bytes: &[u8]) -> Result<BenchmarkMetrics<'_>, BenchmarkEncodeError> {
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
    let last_time_ns = f32::from_le_bytes(
        bytes[24..28]
            .try_into()
            .map_err(|_| BenchmarkEncodeError::InvalidFormat)?,
    );
    let iterations = u32::from_le_bytes(
        bytes[28..32]
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
