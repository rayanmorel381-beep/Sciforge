use super::Entry;
use crate::report::generate_toml_tagged;
use sciforge_parser::toml::{push_toml_array_section, push_toml_num, push_toml_str};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

pub(super) fn build_toml(
    entries: &[Entry<'_>],
    title: &str,
    dir: &Path,
) -> std::io::Result<(BTreeMap<String, String>, usize)> {
    let toml_dir = dir.join("toml");
    fs::create_dir_all(&toml_dir)?;
    let mut files_written = 0usize;
    let mut toml_files: BTreeMap<String, String> = BTreeMap::new();

    let mut toml_all = String::with_capacity(512);
    push_toml_str(&mut toml_all, "title", title);
    push_toml_num(&mut toml_all, "count", &entries.len().to_string());
    toml_all.push('\n');

    for entry in entries {
        let m = entry.metrics;

        let toml = generate_toml_tagged(m, entry.result, &entry.tags);
        let tname = format!("{}.toml", entry.label);
        fs::write(toml_dir.join(&tname), &toml)?;
        toml_files.insert(tname, toml.clone());
        files_written += 1;

        push_toml_array_section(&mut toml_all, "entries");
        push_toml_str(&mut toml_all, "label", entry.label);
        for (k, v) in &entry.tags {
            push_toml_str(&mut toml_all, k, v);
        }
        push_toml_num(
            &mut toml_all,
            "avg_time_ns",
            &format!("{:.2}", m.avg_time_ns),
        );
        push_toml_num(
            &mut toml_all,
            "iterations_per_sec",
            &format!("{:.2}", m.iterations_per_sec),
        );
        toml_all.push('\n');
    }

    toml_files.insert("summary.toml".into(), toml_all.clone());
    fs::write(toml_dir.join("summary.toml"), &toml_all)?;
    files_written += 1;

    Ok((toml_files, files_written))
}
