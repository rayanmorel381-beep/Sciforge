use sciforge_hub::benchmark::decode::decode;
use sciforge_hub::benchmark::engine::bench;
use sciforge_hub::benchmark::export::{Entry, export};
use sciforge_hub::benchmark::report::generate;
use sciforge_hub::prelude::constants::elements;
use std::fs;
use std::path::Path;

fn grid_position(e: &elements::Element) -> (Option<u8>, Option<u8>) {
    let z = e.atomic_number;
    match z {
        57..=71 => (Some(9), Some((z - 57 + 4) as u8)),
        89..=103 => (Some(10), Some((z - 89 + 4) as u8)),
        _ => (Some(e.period as u8), e.group.map(|g| g as u8)),
    }
}

fn main() {
    let dir = Path::new("output/examples/periodic_table");
    fs::create_dir_all(dir).unwrap();
    let all = elements::all();

    let precisions = ["f32", "f64"];
    let iters_choices: [u64; 3] = [1000, 5000, 10000];

    let experiments: Vec<String> = all
        .iter()
        .map(|e| format!("{}_{}_benchmark", e.symbol, e.name))
        .collect();

    let mass_strs: Vec<String> = all.iter().map(|e| format!("{}", e.atomic_mass)).collect();

    let mut metrics_store: Vec<(String, String, _)> = Vec::with_capacity(all.len());

    for (idx, e) in all.iter().enumerate() {
        let prec = precisions[idx % 2];
        let iters = iters_choices[idx % 3];
        let z = e.atomic_number;
        let m = bench(&experiments[idx], prec, iters, move || {
            let mut v = z as u64;
            for i in 0..(z as u64 * 10) {
                v = v.wrapping_mul(6364136223846793005).wrapping_add(i);
            }
            v
        });
        let result_str = format!(
            "Z={} | {} ({}) | {} u | {} | {}",
            e.atomic_number, e.symbol, e.name, e.atomic_mass, e.category, e.electron_configuration
        );
        let label = format!("{:03}_{}", e.atomic_number, e.symbol);
        let r = generate(&m, &result_str);
        assert!(r.csv.starts_with("experiment_name"));
        assert!(r.html.starts_with("<!DOCTYPE html>"));
        metrics_store.push((label, result_str, m));
    }

    let entries: Vec<Entry<'_>> = metrics_store
        .iter()
        .enumerate()
        .map(|(idx, (label, result, m))| {
            let e = &all[idx];
            let (row, col) = grid_position(e);
            Entry {
                metrics: m,
                result: result.as_str(),
                label: label.as_str(),
                tags: vec![
                    ("category", e.category),
                    ("symbol", e.symbol),
                    ("name", e.name),
                    ("config", e.electron_configuration),
                    ("mass", mass_strs[idx].as_str()),
                ],
                grid_row: row,
                grid_col: col,
            }
        })
        .collect();

    let summary = export(
        "Periodic Table of Elements \u{2014} Data from IUPAC, 2024 edition",
        &entries,
        dir,
    )
    .unwrap();
    assert!(summary.files_written > 0);

    let bmk_dir = dir.join("bmk");
    let last_bmk = fs::read(bmk_dir.join("periodic_table.bmk")).unwrap();
    let view = decode(&last_bmk).unwrap();
    assert!(!view.experiment_name.is_empty());
}
