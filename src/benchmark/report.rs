use super::engine::{BenchmarkMetrics, CSV_HEADER};
use crate::parser::html::{push_html_escaped, push_html_row};
use crate::parser::json::{JsonVal, write_json_object};
use crate::parser::markdown::{write_md_code_block, write_md_heading, write_md_row};
use crate::parser::toml::{push_toml_num, push_toml_section, push_toml_str};
use crate::parser::yaml::write_yaml_map;

pub struct BenchmarkReport {
    pub csv: String,
    pub markdown: String,
    pub html: String,
}

pub fn generate(metrics: &BenchmarkMetrics<'_>, result: &str) -> BenchmarkReport {
    let csv = generate_csv(metrics, result);
    let markdown = generate_markdown(metrics, result);
    let md_filename = format!("{}.md", sanitize_filename(metrics.experiment_name));
    let html = generate_html(metrics, result, &md_filename, &markdown);
    BenchmarkReport {
        csv,
        markdown,
        html,
    }
}

pub fn generate_csv(metrics: &BenchmarkMetrics<'_>, result: &str) -> String {
    format!("{}\n{},{}", CSV_HEADER, metrics.to_csv_row(), result)
}

fn generate_markdown(metrics: &BenchmarkMetrics<'_>, result: &str) -> String {
    let mut out = String::with_capacity(1024);
    out.push_str(&write_md_heading(
        1,
        &format!("Benchmark Report: {}", metrics.experiment_name),
    ));
    out.push_str(&write_md_heading(2, "Summary"));
    out.push_str("| Metric | Value |\n|---|---|\n");
    write_md_row(&mut out, "Experiment", metrics.experiment_name);
    write_md_row(&mut out, "Precision", metrics.precision);
    write_md_row(&mut out, "Elapsed (ms)", &metrics.elapsed_ms.to_string());
    write_md_row(&mut out, "Iterations", &metrics.iterations.to_string());
    write_md_row(
        &mut out,
        "Avg time (ns)",
        &format!("{:.2}", metrics.avg_time_ns),
    );
    write_md_row(
        &mut out,
        "Min time (ns)",
        &format!("{:.2}", metrics.min_time_ns),
    );
    write_md_row(
        &mut out,
        "Max time (ns)",
        &format!("{:.2}", metrics.max_time_ns),
    );
    write_md_row(&mut out, "Std dev", &format!("{:.4}", metrics.time_stddev));
    write_md_row(
        &mut out,
        "Iterations/sec",
        &format!("{:.2}", metrics.iterations_per_sec),
    );

    if metrics.total_flops > 0 {
        write_md_row(&mut out, "Total FLOPS", &metrics.total_flops.to_string());
    }
    if metrics.step_count > 0 {
        write_md_row(&mut out, "Steps", &metrics.step_count.to_string());
    }
    if metrics.eval_accuracy > 0.0 {
        write_md_row(
            &mut out,
            "Accuracy",
            &format!("{:.6}", metrics.eval_accuracy),
        );
    }
    if metrics.eval_error > 0.0 {
        write_md_row(&mut out, "Error", &format!("{:.6}", metrics.eval_error));
    }
    if metrics.eval_r_squared > 0.0 {
        write_md_row(&mut out, "R²", &format!("{:.6}", metrics.eval_r_squared));
    }

    out.push_str(&write_md_heading(2, "Result"));
    out.push_str(&write_md_code_block(result));

    out
}

fn generate_html(
    metrics: &BenchmarkMetrics<'_>,
    result: &str,
    md_filename: &str,
    md_content: &str,
) -> String {
    let json = generate_json(metrics, result);
    let yaml = generate_yaml(metrics, result);
    let csv_data = generate_csv(metrics, result);
    let toml_data = generate_toml(metrics, result);

    let mut h = String::with_capacity(4096);
    h.push_str(
        "<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n<meta charset=\"utf-8\">\n<title>Benchmark: ",
    );
    push_html_escaped(&mut h, metrics.experiment_name);
    h.push_str("</title>\n<style>\n");
    h.push_str(
        "body{font-family:system-ui,sans-serif;margin:2em;background:#0d1117;color:#c9d1d9}",
    );
    h.push_str(
        "h1{color:#58a6ff}h2{color:#79c0ff;border-bottom:1px solid #21262d;padding-bottom:.3em}",
    );
    h.push_str("table{border-collapse:collapse;margin:1em 0}th,td{border:1px solid #30363d;padding:.4em .8em;text-align:left}");
    h.push_str("th{background:#161b22}tr:nth-child(even){background:#161b22}");
    h.push_str("pre{background:#161b22;padding:1em;border-radius:6px;overflow-x:auto}");
    h.push_str("code{font-family:ui-monospace,monospace;font-size:.9em}");
    h.push_str("a{color:#58a6ff}.tabs{display:flex;gap:0;margin-top:1em}");
    h.push_str(".tab{padding:.5em 1em;cursor:pointer;background:#161b22;border:1px solid #30363d;border-bottom:none;border-radius:6px 6px 0 0;color:#8b949e}");
    h.push_str(".tab.active{background:#0d1117;color:#c9d1d9;border-bottom:1px solid #0d1117}");
    h.push_str(
        ".panel{display:none;border:1px solid #30363d;padding:1em;border-radius:0 6px 6px 6px}",
    );
    h.push_str(".panel.active{display:block}");
    h.push_str("</style>\n</head>\n<body>\n");

    h.push_str("<h1>Benchmark: ");
    push_html_escaped(&mut h, metrics.experiment_name);
    h.push_str("</h1>\n");

    h.push_str("<h2>Summary</h2>\n<table>\n<tr><th>Metric</th><th>Value</th></tr>\n");
    push_html_row(&mut h, "Experiment", metrics.experiment_name);
    push_html_row(&mut h, "Precision", metrics.precision);
    push_html_row(&mut h, "Elapsed (ms)", &metrics.elapsed_ms.to_string());
    push_html_row(&mut h, "Iterations", &metrics.iterations.to_string());
    push_html_row(
        &mut h,
        "Avg time (ns)",
        &format!("{:.2}", metrics.avg_time_ns),
    );
    push_html_row(
        &mut h,
        "Min time (ns)",
        &format!("{:.2}", metrics.min_time_ns),
    );
    push_html_row(
        &mut h,
        "Max time (ns)",
        &format!("{:.2}", metrics.max_time_ns),
    );
    push_html_row(&mut h, "Std dev", &format!("{:.4}", metrics.time_stddev));
    push_html_row(
        &mut h,
        "Iterations/sec",
        &format!("{:.2}", metrics.iterations_per_sec),
    );
    h.push_str("</table>\n");

    h.push_str("\n<h2>Result</h2>\n<pre><code>");
    push_html_escaped(&mut h, result);
    h.push_str("</code></pre>\n");

    h.push_str("\n<h2>Full Report</h2>\n<p><a href=\"");
    push_html_escaped(&mut h, md_filename);
    h.push_str("\">");
    push_html_escaped(&mut h, md_filename);
    h.push_str("</a></p>\n");

    h.push_str("\n<h2>Data</h2>\n");
    h.push_str("<div class=\"tabs\">\n");
    h.push_str("<div class=\"tab active\" onclick=\"switchTab(event,'csv')\">CSV</div>\n");
    h.push_str("<div class=\"tab\" onclick=\"switchTab(event,'json')\">JSON</div>\n");
    h.push_str("<div class=\"tab\" onclick=\"switchTab(event,'yaml')\">YAML</div>\n");
    h.push_str("<div class=\"tab\" onclick=\"switchTab(event,'toml')\">TOML</div>\n");
    h.push_str("</div>\n");

    h.push_str("<div id=\"csv\" class=\"panel active\"><pre><code>");
    push_html_escaped(&mut h, &csv_data);
    h.push_str("</code></pre></div>\n");

    h.push_str("<div id=\"json\" class=\"panel\"><pre><code>");
    push_html_escaped(&mut h, &json);
    h.push_str("</code></pre></div>\n");

    h.push_str("<div id=\"yaml\" class=\"panel\"><pre><code>");
    push_html_escaped(&mut h, &yaml);
    h.push_str("</code></pre></div>\n");

    h.push_str("<div id=\"toml\" class=\"panel\"><pre><code>");
    push_html_escaped(&mut h, &toml_data);
    h.push_str("</code></pre></div>\n");

    h.push_str("\n<h2>Markdown Preview</h2>\n<pre><code>");
    push_html_escaped(&mut h, md_content);
    h.push_str("</code></pre>\n");

    h.push_str("\n<script>\nfunction switchTab(e,id){");
    h.push_str("document.querySelectorAll('.tab').forEach(t=>t.classList.remove('active'));");
    h.push_str("document.querySelectorAll('.panel').forEach(p=>p.classList.remove('active'));");
    h.push_str(
        "e.target.classList.add('active');document.getElementById(id).classList.add('active');",
    );
    h.push_str("}\n</script>\n</body>\n</html>\n");

    h
}

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

pub fn sanitize_filename(name: &str) -> String {
    name.chars()
        .map(|c| match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' | '.' => c,
            '/' | ':' | ' ' => '_',
            _ => '_',
        })
        .collect()
}
