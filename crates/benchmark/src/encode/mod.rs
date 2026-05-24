mod header;
mod payload;

use crate::engine::{BENCHMARK_HEADER_SIZE, BenchmarkEncodeError, BenchmarkMetrics};

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

    header::write_header(metrics, out);
    let cursor = payload::write_payload(metrics, out, BENCHMARK_HEADER_SIZE);
    Ok(cursor)
}
