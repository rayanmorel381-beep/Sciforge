use crate::engine::BenchmarkMetrics;
use sciforge_parser::toml::{push_toml_num, push_toml_section, push_toml_str};

pub fn generate_toml(metrics: &BenchmarkMetrics<'_>, result: &str) -> String {
    generate_toml_tagged(metrics, result, &[])
}

pub fn generate_toml_tagged(
    metrics: &BenchmarkMetrics<'_>,
    result: &str,
    tags: &[(&str, &str)],
) -> String {
    let mut t = String::with_capacity(512);
    push_toml_section(&mut t, "benchmark");
    for (k, v) in tags {
        push_toml_str(&mut t, k, v);
    }
    push_toml_str(&mut t, "experiment_name", metrics.experiment_name);
    push_toml_str(&mut t, "precision", metrics.precision);
    push_toml_num(&mut t, "elapsed_ms", &metrics.elapsed_ms.to_string());
    push_toml_num(&mut t, "iterations", &metrics.iterations.to_string());
    push_toml_num(
        &mut t,
        "avg_time_ns",
        &format!("{:.2}", metrics.avg_time_ns),
    );
    push_toml_num(
        &mut t,
        "min_time_ns",
        &format!("{:.2}", metrics.min_time_ns),
    );
    push_toml_num(
        &mut t,
        "max_time_ns",
        &format!("{:.2}", metrics.max_time_ns),
    );
    push_toml_num(
        &mut t,
        "time_stddev",
        &format!("{:.4}", metrics.time_stddev),
    );
    push_toml_num(
        &mut t,
        "iterations_per_sec",
        &format!("{:.2}", metrics.iterations_per_sec),
    );
    push_toml_str(&mut t, "result", result);
    t
}
