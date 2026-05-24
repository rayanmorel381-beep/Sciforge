use crate::engine::{BenchmarkMetrics, CSV_HEADER};

pub fn generate_csv(metrics: &BenchmarkMetrics<'_>, result: &str) -> String {
    format!("{}\n{},{}", CSV_HEADER, metrics.to_csv_row(), result)
}
