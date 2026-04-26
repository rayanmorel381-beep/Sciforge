use super::Entry;
use crate::benchmark::engine::CSV_HEADER;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

pub(super) fn build_csv(
    entries: &[Entry<'_>],
    tag_keys: &[&str],
    dir: &Path,
) -> std::io::Result<(BTreeMap<String, String>, usize)> {
    let csv_dir = dir.join("csv");
    fs::create_dir_all(&csv_dir)?;
    let mut files_written = 0usize;
    let mut csv_files: BTreeMap<String, String> = BTreeMap::new();

    let mut csv_all = String::from(CSV_HEADER);
    for k in tag_keys {
        csv_all.push(',');
        csv_all.push_str(k);
    }
    csv_all.push('\n');

    let mut group_csv: BTreeMap<String, String> = BTreeMap::new();

    for entry in entries {
        let m = entry.metrics;
        csv_all.push_str(&m.to_csv_row());
        csv_all.push(',');
        csv_all.push_str(entry.result);
        for (_, v) in &entry.tags {
            csv_all.push(',');
            csv_all.push_str(v);
        }
        csv_all.push('\n');

        if let Some(group_tag) = entry.tags.first() {
            let key = group_tag.1.replace(' ', "_");
            let bucket = group_csv.entry(key.clone()).or_insert_with(|| {
                let mut h = String::from(CSV_HEADER);
                for k in tag_keys {
                    h.push(',');
                    h.push_str(k);
                }
                h.push('\n');
                h
            });
            bucket.push_str(&m.to_csv_row());
            bucket.push(',');
            bucket.push_str(entry.result);
            for (_, v) in &entry.tags {
                bucket.push(',');
                bucket.push_str(v);
            }
            bucket.push('\n');
        }
    }

    csv_files.insert("all.csv".into(), csv_all.clone());
    fs::write(csv_dir.join("all.csv"), &csv_all)?;
    files_written += 1;

    for (group, content) in &group_csv {
        let fname = format!("{group}.csv");
        csv_files.insert(fname.clone(), content.clone());
        fs::write(csv_dir.join(&fname), content)?;
        files_written += 1;
    }

    Ok((csv_files, files_written))
}
