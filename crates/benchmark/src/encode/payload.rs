use crate::engine::BenchmarkMetrics;

pub(super) fn write_payload(
    metrics: &BenchmarkMetrics<'_>,
    out: &mut [u8],
    start: usize,
) -> usize {
    let mut cursor = start;
    let name_bytes = metrics.experiment_name.as_bytes();
    out[cursor..cursor + name_bytes.len()].copy_from_slice(name_bytes);
    cursor += name_bytes.len();

    let prec_bytes = metrics.precision.as_bytes();
    out[cursor..cursor + prec_bytes.len()].copy_from_slice(prec_bytes);
    cursor += prec_bytes.len();

    cursor
}
