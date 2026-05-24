use super::{generate_csv, generate_json, generate_toml, generate_yaml};
use crate::engine::BenchmarkMetrics;
use sciforge_parser::html::{push_html_escaped, push_html_row};

pub fn generate_html(
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
