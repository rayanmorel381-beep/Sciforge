use super::Entry;
use crate::benchmark::report::generate_yaml_tagged;
use crate::parser::yaml::write_yaml_document;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

pub(super) fn build_yaml(
    entries: &[Entry<'_>],
    title: &str,
    group_key: &str,
    dir: &Path,
) -> std::io::Result<(BTreeMap<String, String>, usize)> {
    let yaml_dir = dir.join("yaml");
    fs::create_dir_all(&yaml_dir)?;
    let mut files_written = 0usize;
    let mut yaml_files: BTreeMap<String, String> = BTreeMap::new();
    let mut yaml_items_data: Vec<Vec<(String, String)>> = Vec::new();
    let mut group_yaml_data: BTreeMap<String, Vec<Vec<(String, String)>>> = BTreeMap::new();

    for entry in entries {
        let m = entry.metrics;

        let yaml = generate_yaml_tagged(m, entry.result, &entry.tags);
        let yname = format!("{}.yaml", entry.label);
        fs::write(yaml_dir.join(&yname), &yaml)?;
        yaml_files.insert(yname, yaml.clone());
        files_written += 1;

        let mut fields = vec![("label".to_string(), entry.label.to_string())];
        for (k, v) in &entry.tags {
            fields.push((k.to_string(), v.to_string()));
        }
        fields.push(("avg_time_ns".to_string(), format!("{:.2}", m.avg_time_ns)));
        fields.push((
            "iterations_per_sec".to_string(),
            format!("{:.2}", m.iterations_per_sec),
        ));
        yaml_items_data.push(fields);

        if let Some(group_tag) = entry.tags.first() {
            let items = group_yaml_data.entry(group_tag.1.to_string()).or_default();
            let mut gfields = vec![("label".to_string(), entry.label.to_string())];
            for (k, v) in entry.tags.iter().skip(1) {
                gfields.push((k.to_string(), v.to_string()));
            }
            gfields.push(("avg_time_ns".to_string(), format!("{:.2}", m.avg_time_ns)));
            items.push(gfields);
        }
    }

    let count_str = entries.len().to_string();
    let yaml_header: Vec<(&str, &str)> = vec![("title", title), ("count", &count_str)];
    let yaml_items: Vec<Vec<(&str, &str)>> = yaml_items_data
        .iter()
        .map(|item| item.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect())
        .collect();
    let yaml_all = write_yaml_document(&yaml_header, "entries", &yaml_items);
    yaml_files.insert("summary.yaml".into(), yaml_all.clone());
    fs::write(yaml_dir.join("summary.yaml"), &yaml_all)?;
    files_written += 1;

    for (cat, items_data) in &group_yaml_data {
        let grp_count_str = items_data.len().to_string();
        let header: Vec<(&str, &str)> = vec![(group_key, cat.as_str()), ("count", &grp_count_str)];
        let items: Vec<Vec<(&str, &str)>> = items_data
            .iter()
            .map(|item| item.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect())
            .collect();
        let y = write_yaml_document(&header, "entries", &items);
        let fname = format!("{}.yaml", cat.replace(' ', "_"));
        yaml_files.insert(fname.clone(), y.clone());
        fs::write(yaml_dir.join(&fname), &y)?;
        files_written += 1;
    }

    Ok((yaml_files, files_written))
}
