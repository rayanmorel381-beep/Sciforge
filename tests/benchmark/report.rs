use sciforge_hub::benchmark::engine::bench;
use sciforge_hub::benchmark::report::generate;

#[test]
fn report_three_outputs() {
    let m = bench("rpt", "f64", 10, || {});
    let r = generate(&m, "42.0");
    assert!(!r.csv.is_empty());
    assert!(!r.markdown.is_empty());
    assert!(!r.html.is_empty());
}

#[test]
fn report_csv_header_and_row() {
    let m = bench("rpt", "f64", 10, || {});
    let r = generate(&m, "42.0");
    let lines: Vec<&str> = r.csv.lines().collect();
    assert_eq!(lines.len(), 2);
    assert!(lines[0].starts_with("experiment_name"));
    assert!(lines[1].starts_with("rpt"));
}

#[test]
fn report_csv_ends_with_result() {
    let m = bench("rpt", "f64", 10, || {});
    let r = generate(&m, "RES");
    assert!(r.csv.ends_with("RES"));
}

#[test]
fn report_markdown_heading() {
    let m = bench("rpt", "f64", 10, || {});
    let r = generate(&m, "ok");
    assert!(r.markdown.starts_with("# Benchmark Report: rpt"));
}

#[test]
fn report_markdown_table() {
    let m = bench("rpt", "f64", 10, || {});
    let r = generate(&m, "ok");
    assert!(r.markdown.contains("| Metric | Value |"));
    assert!(r.markdown.contains("| Experiment | rpt |"));
}

#[test]
fn report_html_document() {
    let m = bench("rpt", "f64", 10, || {});
    let r = generate(&m, "ok");
    assert!(r.html.starts_with("<!DOCTYPE html>"));
    assert!(r.html.contains("</html>"));
}

#[test]
fn report_html_title() {
    let m = bench("rpt", "f64", 10, || {});
    let r = generate(&m, "ok");
    assert!(r.html.contains("<title>Benchmark: rpt</title>"));
}

#[test]
fn report_html_escapes() {
    let m = bench("rpt", "f64", 10, || {});
    let r = generate(&m, "<script>alert(1)</script>");
    assert!(!r.html.contains("<script>alert"));
    assert!(r.html.contains("&lt;script&gt;"));
}
