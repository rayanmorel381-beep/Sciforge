use sciforge_hub::benchmark::engine::{BenchmarkMetrics, bench};
use sciforge_hub::benchmark::export::{Entry, export};
use std::fs;
use std::path::Path;
use std::process::Command;

mod aging;
mod bioelectricity;
mod bioenergetics;
mod biogeography;
mod bioinformatics;
mod biomechanics;
mod biophysics;
mod biostatistics;
mod cancer_biology;
mod cell;
mod chronobiology;
mod cryobiology;
mod developmental;
mod dispatch;
mod ecology;
mod endocrinology;
mod enzyme;
mod epigenetics;
mod ethology;
mod evolution;
mod genetics;
mod genomics;
mod immunology;
mod marine_biology;
mod microbiology;
mod mycology;
mod neuroscience;
mod nutrition;
mod paleobiology;
mod parasitology;
mod pharmacology;
mod phylogenetics;
mod physiology;
mod plant_biology;
mod population;
mod proteomics;
mod radiobiology;
mod reproduction;
mod stem_cell;
mod structural;
mod synthetic_biology;
mod systems_biology;
mod tissue_engineering;
mod toxicology;
mod virology;

fn sanitize_label(name: &str) -> String {
    let mut label = String::with_capacity(name.len());
    for ch in name.chars() {
        if ch.is_ascii_alphanumeric() {
            label.push(ch);
        } else {
            label.push('_');
        }
    }
    while label.contains("__") {
        label = label.replace("__", "_");
    }
    label.trim_matches('_').to_string()
}

fn make_symbol(fname: &str) -> String {
    let parts: Vec<&str> = fname.split('_').filter(|s| !s.is_empty()).collect();
    if parts.len() == 1 {
        let w = parts[0];
        let mut c = w.chars();
        let first = c.next().unwrap_or('?').to_ascii_uppercase();
        let second = c.next().unwrap_or('x').to_ascii_lowercase();
        format!("{first}{second}")
    } else {
        parts
            .iter()
            .take(3)
            .map(|w| w.chars().next().unwrap_or('?').to_ascii_uppercase())
            .collect()
    }
}

fn list_tests() -> Vec<(String, String, String, String, String)> {
    let exe = std::env::current_exe().unwrap();
    let output = Command::new(exe).arg("--list").output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    let mut tests = Vec::new();
    for line in stdout.lines() {
        let Some((name, kind)) = line.rsplit_once(": ") else {
            continue;
        };
        if kind.trim() != "test" {
            continue;
        }
        let test_name = name.trim();
        if test_name == "generate_output_files" {
            continue;
        }
        let module = test_name.split("::").next().unwrap_or("other").to_string();
        let label = sanitize_label(test_name);
        let fname = test_name.rsplit("::").next().unwrap_or(test_name);
        let readable = fname.replace('_', " ");
        let display_name = if readable.is_empty() {
            readable
        } else {
            let mut c = readable.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
            }
        };
        let symbol = make_symbol(fname);
        tests.push((test_name.to_string(), module, label, display_name, symbol));
    }
    tests.sort();
    tests
}

fn assert_artifacts(domain: &str, dir: &Path, test_count: usize) {
    assert!(dir.join(format!("{domain}.html")).exists());
    assert!(dir.join(format!("{domain}.md")).exists());
    assert!(dir.join("csv/all.csv").exists());
    assert!(dir.join("json/summary.json").exists());
    assert!(dir.join("yaml/summary.yaml").exists());
    assert!(dir.join("toml/summary.toml").exists());
    let count_files = |p: &Path| -> usize {
        fs::read_dir(p)
            .unwrap()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().map(|t| t.is_file()).unwrap_or(false))
            .count()
    };
    let has_entries = test_count > 0;
    assert!(count_files(&dir.join("csv")) >= if has_entries { 2 } else { 1 });
    assert!(count_files(&dir.join("json")) > test_count);
    assert!(count_files(&dir.join("yaml")) >= if has_entries { 2 } else { 1 });
    assert!(count_files(&dir.join("toml")) > test_count);
    assert!(count_files(&dir.join("bmk")) >= 1);
}

fn export_outputs(domain: &'static str, title: &'static str) -> (std::path::PathBuf, usize) {
    let tests = list_tests();
    let test_count = tests.len();
    let exe = std::env::current_exe().unwrap();

    let mut module_order: Vec<String> = Vec::new();
    let mut module_counts: Vec<u8> = Vec::new();
    let mut positions: Vec<(u8, u8)> = Vec::new();
    for (_, module, _, _, _) in &tests {
        let mi = match module_order.iter().position(|m| m == module) {
            Some(p) => p,
            None => {
                module_order.push(module.clone());
                module_counts.push(0);
                module_order.len() - 1
            }
        };
        module_counts[mi] += 1;
        positions.push(((mi as u8) + 1, module_counts[mi]));
    }

    let metrics: Vec<BenchmarkMetrics<'_>> = tests
        .iter()
        .map(|(test_name, _, _, _, _)| {
            bench(test_name, "f64", 1, || {
                let _ = Command::new(&exe)
                    .args(["--exact", test_name, "--test-threads=1", "-q"])
                    .output();
            })
        })
        .collect();
    let entries: Vec<Entry<'_>> = metrics
        .iter()
        .zip(tests.iter())
        .zip(positions.iter())
        .map(
            |((metric, (test_name, module, label, display_name, symbol)), &(row, col))| Entry {
                metrics: metric,
                result: "pass",
                label,
                tags: vec![
                    ("category", module.as_str()),
                    ("symbol", symbol.as_str()),
                    ("name", display_name.as_str()),
                    ("domain", domain),
                    ("test", test_name.as_str()),
                ],
                grid_row: Some(row),
                grid_col: Some(col),
            },
        )
        .collect();
    let dir = Path::new("output/tests").join(domain);
    export(title, &entries, &dir).unwrap();
    (dir, test_count)
}

#[test]
fn generate_output_files() {
    let (dir, test_count) = export_outputs("biology", "SciForge Biology Benchmarks");
    assert_artifacts("biology", &dir, test_count);
}
