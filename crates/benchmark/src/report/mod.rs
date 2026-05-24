mod csv;
mod html;
mod json;
mod markdown;
mod toml;
mod yaml;

pub use csv::generate_csv;
pub use html::generate_html;
pub use json::{generate_json, generate_json_tagged};
pub use markdown::generate_markdown;
pub use toml::{generate_toml, generate_toml_tagged};
pub use yaml::{generate_yaml, generate_yaml_tagged};

use crate::engine::BenchmarkMetrics;

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

pub fn sanitize_filename(name: &str) -> String {
    name.chars()
        .map(|c| match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' | '.' => c,
            '/' | ':' | ' ' => '_',
            _ => '_',
        })
        .collect()
}
