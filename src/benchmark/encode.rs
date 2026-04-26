use super::engine::{
    BENCHMARK_HEADER_SIZE, BENCHMARK_MAGIC, BENCHMARK_VERSION, BenchmarkEncodeError,
    BenchmarkMetrics,
};

pub fn encoded_size(metrics: &BenchmarkMetrics<'_>) -> Option<usize> {
    BENCHMARK_HEADER_SIZE
        .checked_add(metrics.experiment_name.len())?
        .checked_add(metrics.precision.len())
}

pub fn encode(
    metrics: &BenchmarkMetrics<'_>,
    out: &mut [u8],
) -> Result<usize, BenchmarkEncodeError> {
    if metrics.experiment_name.len() > u16::MAX as usize
        || metrics.precision.len() > u16::MAX as usize
    {
        return Err(BenchmarkEncodeError::InvalidFormat);
    }
    let needed = encoded_size(metrics).ok_or(BenchmarkEncodeError::InvalidFormat)?;
    if out.len() < needed {
        return Err(BenchmarkEncodeError::BufferTooSmall);
    }

    out[0..4].copy_from_slice(&BENCHMARK_MAGIC);
    out[4..6].copy_from_slice(&BENCHMARK_VERSION.to_le_bytes());
    out[6..8].copy_from_slice(&0u16.to_le_bytes());
    out[8..16].copy_from_slice(&metrics.elapsed_ms.to_le_bytes());
    out[16..24].copy_from_slice(&metrics.iterations.to_le_bytes());
    out[24..32].copy_from_slice(&metrics.input_samples.to_le_bytes());
    out[32..36].copy_from_slice(&metrics.avg_time_ns.to_le_bytes());
    out[36..40].copy_from_slice(&metrics.last_time_ns.to_le_bytes());
    out[40..48].copy_from_slice(&(metrics.output_bytes as u64).to_le_bytes());
    out[48..56].copy_from_slice(&metrics.total_flops.to_le_bytes());
    out[56..60].copy_from_slice(&metrics.step_count.to_le_bytes());
    out[60..64].copy_from_slice(&metrics.input_dim.to_le_bytes());
    out[64..68].copy_from_slice(&metrics.output_dim.to_le_bytes());
    out[68..76].copy_from_slice(&metrics.benchmark_flags.to_le_bytes());
    out[76..84].copy_from_slice(&metrics.input_bytes.to_le_bytes());
    out[84..92].copy_from_slice(&metrics.result_bytes.to_le_bytes());
    out[92..96].copy_from_slice(&metrics.min_time_ns.to_le_bytes());
    out[96..100].copy_from_slice(&metrics.max_time_ns.to_le_bytes());
    out[100..104].copy_from_slice(&metrics.time_stddev.to_le_bytes());
    out[104..108].copy_from_slice(&metrics.iterations_per_sec.to_le_bytes());
    out[108..112].copy_from_slice(&metrics.samples_per_sec.to_le_bytes());
    out[112..116].copy_from_slice(&metrics.eval_error.to_le_bytes());
    out[116..120].copy_from_slice(&metrics.eval_accuracy.to_le_bytes());
    out[120..124].copy_from_slice(&metrics.eval_r_squared.to_le_bytes());
    out[124..128].copy_from_slice(&metrics.eval_mae.to_le_bytes());
    out[128..136].copy_from_slice(&metrics.eval_samples.to_le_bytes());
    out[136..144].copy_from_slice(&metrics.eval_dataset_hash.to_le_bytes());
    out[144..148].copy_from_slice(&metrics.logical_cores.to_le_bytes());
    out[148..152].copy_from_slice(&metrics.avg_frequency_mhz.to_le_bytes());
    out[152..156].copy_from_slice(&metrics.max_frequency_mhz.to_le_bytes());
    out[156..160].copy_from_slice(&metrics.max_workers.to_le_bytes());
    out[160..164].copy_from_slice(&metrics.target_cpu_utilization.to_le_bytes());
    out[164..166].copy_from_slice(&(metrics.experiment_name.len() as u16).to_le_bytes());
    out[166..168].copy_from_slice(&(metrics.precision.len() as u16).to_le_bytes());

    let mut cursor = BENCHMARK_HEADER_SIZE;
    let name_bytes = metrics.experiment_name.as_bytes();
    out[cursor..cursor + name_bytes.len()].copy_from_slice(name_bytes);
    cursor += name_bytes.len();

    let prec_bytes = metrics.precision.as_bytes();
    out[cursor..cursor + prec_bytes.len()].copy_from_slice(prec_bytes);
    cursor += prec_bytes.len();

    Ok(cursor)
}
