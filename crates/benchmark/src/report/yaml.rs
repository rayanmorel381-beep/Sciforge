use crate::engine::BenchmarkMetrics;
use sciforge_parser::yaml::write_yaml_map;

pub fn generate_yaml(metrics: &BenchmarkMetrics<'_>, result: &str) -> String {
    generate_yaml_tagged(metrics, result, &[])
}

pub fn generate_yaml_tagged(
    metrics: &BenchmarkMetrics<'_>,
    result: &str,
    tags: &[(&str, &str)],
) -> String {
    let elapsed = metrics.elapsed_ms.to_string();
    let iterations = metrics.iterations.to_string();
    let avg = format!("{:.2}", metrics.avg_time_ns);
    let min = format!("{:.2}", metrics.min_time_ns);
    let max = format!("{:.2}", metrics.max_time_ns);
    let stddev = format!("{:.4}", metrics.time_stddev);
    let ips = format!("{:.2}", metrics.iterations_per_sec);

    let mut fields: Vec<(&str, &str)> = Vec::with_capacity(tags.len() + 10);
    for (k, v) in tags {
        fields.push((k, v));
    }
    fields.push(("experiment_name", metrics.experiment_name));
    fields.push(("precision", metrics.precision));
    fields.push(("elapsed_ms", &elapsed));
    fields.push(("iterations", &iterations));
    fields.push(("avg_time_ns", &avg));
    fields.push(("min_time_ns", &min));
    fields.push(("max_time_ns", &max));
    fields.push(("time_stddev", &stddev));
    fields.push(("iterations_per_sec", &ips));
    fields.push(("result", result));
    write_yaml_map(&fields)
}
