mod bmk;
mod csv;
mod html;
mod json;
mod md;
mod toml;
mod yaml;

use super::engine::BenchmarkMetrics;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

pub const PTABLE_CATS: [&str; 10] = [
    "Nonmetal",
    "Noble Gas",
    "Alkali Metal",
    "Alkaline Earth",
    "Transition Metal",
    "Post-Transition",
    "Metalloid",
    "Halogen",
    "Lanthanide",
    "Actinide",
];

pub struct GroupStats<'a> {
    pub groups: Vec<&'a str>,
    pub counts: BTreeMap<&'a str, usize>,
    pub sums: BTreeMap<&'a str, f32>,
    pub mins: BTreeMap<&'a str, f32>,
    pub maxs: BTreeMap<&'a str, f32>,
}

pub fn compute_group_stats<'a>(entries: &[Entry<'a>]) -> GroupStats<'a> {
    let mut groups: Vec<&'a str> = Vec::new();
    let mut counts: BTreeMap<&'a str, usize> = BTreeMap::new();
    let mut sums: BTreeMap<&'a str, f32> = BTreeMap::new();
    let mut mins: BTreeMap<&'a str, f32> = BTreeMap::new();
    let mut maxs: BTreeMap<&'a str, f32> = BTreeMap::new();

    for entry in entries {
        let val = entry.tags.first().map(|(_, v)| *v).unwrap_or("other");
        if !groups.contains(&val) {
            groups.push(val);
        }
        *counts.entry(val).or_insert(0) += 1;
        *sums.entry(val).or_insert(0.0) += entry.metrics.avg_time_ns;
        let mn = mins.entry(val).or_insert(f32::MAX);
        if entry.metrics.avg_time_ns < *mn {
            *mn = entry.metrics.avg_time_ns;
        }
        let mx = maxs.entry(val).or_insert(0.0);
        if entry.metrics.avg_time_ns > *mx {
            *mx = entry.metrics.avg_time_ns;
        }
    }

    GroupStats {
        groups,
        counts,
        sums,
        mins,
        maxs,
    }
}

pub fn capitalize_first(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().to_string() + c.as_str(),
    }
}

pub struct Entry<'a> {
    pub metrics: &'a BenchmarkMetrics<'a>,
    pub result: &'a str,
    pub label: &'a str,
    pub tags: Vec<(&'a str, &'a str)>,
    pub grid_row: Option<u8>,
    pub grid_col: Option<u8>,
}

pub struct ExportSummary {
    pub files_written: usize,
    pub html_size: usize,
    pub md_size: usize,
}

pub fn export(title: &str, entries: &[Entry<'_>], dir: &Path) -> std::io::Result<ExportSummary> {
    let tag_keys: Vec<&str> = if let Some(first) = entries.first() {
        first.tags.iter().map(|(k, _)| *k).collect()
    } else {
        Vec::new()
    };

    let group_key = tag_keys.first().copied().unwrap_or("group");

    let (csv_files, csv_count) = csv::build_csv(entries, &tag_keys, dir)?;
    let (json_files, json_count) = json::build_json(entries, dir)?;
    let (yaml_files, yaml_count) = yaml::build_yaml(entries, title, group_key, dir)?;
    let (toml_files, toml_count) = toml::build_toml(entries, title, dir)?;
    let (bmk_files, bmk_count) = bmk::build_bmk(entries, dir)?;

    let mut files_written = csv_count + json_count + yaml_count + toml_count + bmk_count;

    let md_content = md::build_md(title, entries, &tag_keys);

    let html_content = html::build_html(
        title,
        entries,
        &tag_keys,
        &csv_files,
        &json_files,
        &yaml_files,
        &toml_files,
        &md_content,
        &bmk_files,
    );

    let base_name = dir
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("benchmark");
    fs::write(dir.join(format!("{base_name}.html")), &html_content)?;
    files_written += 1;

    fs::write(dir.join(format!("{base_name}.md")), &md_content)?;
    files_written += 1;

    Ok(ExportSummary {
        files_written,
        html_size: html_content.len(),
        md_size: md_content.len(),
    })
}
