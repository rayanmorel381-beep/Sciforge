use sciforge_hub::benchmark::decode::decode;
use sciforge_hub::benchmark::encode::{encode, encoded_size};
use sciforge_hub::benchmark::engine::bench;
use sciforge_hub::benchmark::export::{Entry, export};
use sciforge_hub::benchmark::report::{
    generate, generate_csv, generate_json, generate_toml, generate_yaml,
};
use std::fs;
use std::path::Path;

mod encode;
mod engine;
mod report;
mod simulation;

#[test]
fn bench_timing_consistency() {
    let m = bench("timing", "f64", 500, || {
        let mut acc = 0u64;
        for i in 0..100 {
            acc = acc.wrapping_add(i);
        }
        acc
    });
    assert!(m.elapsed_ms < 10_000);
    assert!(m.avg_time_ns > 0.0);
    assert!(m.min_time_ns <= m.avg_time_ns);
    assert!(m.max_time_ns >= m.avg_time_ns);
    assert!(m.iterations_per_sec >= 0.0);
}

#[test]
fn bench_stddev_nonnegative() {
    let m = bench("stddev", "f32", 200, || 42);
    assert!(m.time_stddev >= 0.0);
}

#[test]
fn report_all_formats() {
    let m = bench("formats", "f64", 10, || 1 + 1);
    let csv = generate_csv(&m, "result_ok");
    let json = generate_json(&m, "result_ok");
    let yaml = generate_yaml(&m, "result_ok");
    let toml = generate_toml(&m, "result_ok");
    assert!(csv.contains("formats"));
    assert!(json.contains("formats"));
    assert!(yaml.contains("formats"));
    assert!(toml.contains("formats"));
    assert!(csv.contains("result_ok"));
    assert!(json.contains("result_ok"));
}

#[test]
fn report_generate_all_fields() {
    let m = bench("fields", "f64", 50, || {});
    let r = generate(&m, "val");
    assert!(r.csv.starts_with("experiment_name"));
    assert!(r.html.starts_with("<!DOCTYPE html>"));
    assert!(r.markdown.starts_with("# Benchmark Report"));
    assert!(r.csv.lines().count() == 2);
}

#[test]
fn encode_decode_multiple() {
    for i in 0..5 {
        let name = format!("multi_{i}");
        let m = bench(&name, "f64", 10 * (i + 1), || i * i);
        let size = encoded_size(&m).unwrap();
        let mut buf = vec![0u8; size];
        encode(&m, &mut buf).unwrap();
        let view = decode(&buf).unwrap();
        assert_eq!(view.experiment_name, name);
        assert_eq!(view.iterations, 10 * (i + 1));
    }
}

#[test]
fn export_generates_files() {
    let dir = Path::new("output/tests/benchmark");
    let names = ["alpha", "beta", "gamma"];
    let labels: Vec<String> = names.iter().map(|n| format!("bench_{n}")).collect();
    let results: Vec<String> = names.iter().map(|n| format!("{n} completed")).collect();
    let metrics: Vec<_> = names
        .iter()
        .enumerate()
        .map(|(idx, _)| {
            bench(
                &labels[idx],
                if idx % 2 == 0 { "f64" } else { "f32" },
                100,
                || idx * idx,
            )
        })
        .collect();

    let entries: Vec<Entry<'_>> = metrics
        .iter()
        .enumerate()
        .map(|(idx, m)| Entry {
            metrics: m,
            result: results[idx].as_str(),
            label: labels[idx].as_str(),
            tags: vec![("group", names[idx])],
            grid_row: Some((idx + 1) as u8),
            grid_col: Some(1),
        })
        .collect();

    let summary = export("Benchmark Integration Test", &entries, dir).unwrap();
    assert!(summary.files_written > 0);
    assert!(summary.html_size > 0);
    assert!(summary.md_size > 0);

    assert!(dir.join("benchmark.html").exists());
    assert!(dir.join("benchmark.md").exists());

    let csv_dir = dir.join("csv");
    let csv_read = fs::read_to_string(csv_dir.join("all.csv")).unwrap();
    assert_eq!(csv_read.lines().count(), names.len() + 1);

    let json_dir = dir.join("json");
    assert!(fs::read_dir(&json_dir).unwrap().count() >= names.len());

    let bmk_dir = dir.join("bmk");
    let bmk_data = fs::read(bmk_dir.join("benchmark.bmk")).unwrap();
    let view = decode(&bmk_data).unwrap();
    assert!(view.experiment_name.starts_with("bench_"));

    let html = fs::read_to_string(dir.join("benchmark.html")).unwrap();
    assert!(html.contains("CSV"));
    assert!(html.contains("JSON"));
    assert!(html.contains("Benchmark"));
}

#[test]
fn export_html_contains_entries() {
    let dir = Path::new("output/tests/benchmark/entries");
    fs::create_dir_all(dir).unwrap();
    let m = bench("entry_test", "f64", 50, || 99);
    let entries = vec![Entry {
        metrics: &m,
        result: "test_result",
        label: "lbl",
        tags: vec![("k", "v")],
        grid_row: Some(1),
        grid_col: Some(1),
    }];
    let summary = export("Entry Test", &entries, dir).unwrap();
    assert!(summary.files_written > 0);
    let html = fs::read_to_string(dir.join("entries.html")).unwrap();
    assert!(html.contains("entry_test"));
}

#[test]
fn bmk_roundtrip_from_export() {
    let dir = Path::new("output/tests/benchmark/roundtrip");
    fs::create_dir_all(dir).unwrap();
    let m = bench("rt_export", "f32", 25, || 7);
    let entries = vec![Entry {
        metrics: &m,
        result: "roundtrip",
        label: "rt",
        tags: vec![],
        grid_row: None,
        grid_col: None,
    }];
    export("Roundtrip", &entries, dir).unwrap();
    let bmk_data = fs::read(dir.join("bmk/roundtrip.bmk")).unwrap();
    let view = decode(&bmk_data).unwrap();
    assert_eq!(view.precision, "f32");
    assert_eq!(view.iterations, 25);
}
