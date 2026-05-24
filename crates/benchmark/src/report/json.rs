use crate::engine::BenchmarkMetrics;
use sciforge_parser::json::{JsonVal, write_json_object};

pub fn generate_json(metrics: &BenchmarkMetrics<'_>, result: &str) -> String {
    generate_json_tagged(metrics, result, &[])
}

pub fn generate_json_tagged(
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

    let mut fields: Vec<(&str, JsonVal<'_>)> = Vec::with_capacity(tags.len() + 10);
    for (k, v) in tags {
        fields.push((k, JsonVal::Str(v)));
    }
    fields.push(("experiment_name", JsonVal::Str(metrics.experiment_name)));
    fields.push(("precision", JsonVal::Str(metrics.precision)));
    fields.push(("elapsed_ms", JsonVal::Raw(&elapsed)));
    fields.push(("iterations", JsonVal::Raw(&iterations)));
    fields.push(("avg_time_ns", JsonVal::Raw(&avg)));
    fields.push(("min_time_ns", JsonVal::Raw(&min)));
    fields.push(("max_time_ns", JsonVal::Raw(&max)));
    fields.push(("time_stddev", JsonVal::Raw(&stddev)));
    fields.push(("iterations_per_sec", JsonVal::Raw(&ips)));
    fields.push(("result", JsonVal::Str(result)));
    write_json_object(&fields)
}
