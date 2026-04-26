use sciforge::benchmark::engine::{
    BENCHMARK_HEADER_SIZE, BENCHMARK_MAGIC, BENCHMARK_VERSION, CSV_HEADER, bench,
};

#[test]
fn bench_runs_closure() {
    let m = bench("add", "f64", 100, || 2 + 2);
    assert_eq!(m.experiment_name, "add");
    assert_eq!(m.precision, "f64");
    assert_eq!(m.iterations, 100);
    assert!(m.avg_time_ns > 0.0);
}

#[test]
fn bench_zero_iterations() {
    let m = bench("noop", "f64", 0, || {});
    assert_eq!(m.iterations, 0);
    assert_eq!(m.elapsed_ms, 0);
}

#[test]
fn csv_header_field_count() {
    let cols: Vec<&str> = CSV_HEADER.split(',').collect();
    assert_eq!(cols[0], "experiment_name");
    assert_eq!(cols.len(), 10);
    assert_eq!(*cols.last().unwrap(), "result");
}

#[test]
fn to_csv_row_matches_header() {
    let m = bench("x", "f32", 10, || 1 + 1);
    let row = m.to_csv_row();
    assert_eq!(row.split(',').count(), CSV_HEADER.split(',').count() - 1);
}

#[test]
fn magic_bytes() {
    assert_eq!(&BENCHMARK_MAGIC, &[b'B', b'M', b'K', 0x01]);
}

#[test]
fn version_is_4() {
    assert_eq!(BENCHMARK_VERSION, 4);
}

#[test]
fn header_size_is_168() {
    assert_eq!(BENCHMARK_HEADER_SIZE, 168);
}
