use super::Entry;
use crate::report::generate_json_tagged;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

pub(super) fn build_json(
    entries: &[Entry<'_>],
    dir: &Path,
) -> std::io::Result<(BTreeMap<String, String>, usize)> {
    let json_dir = dir.join("json");
    fs::create_dir_all(&json_dir)?;
    let mut files_written = 0usize;
    let mut json_files: BTreeMap<String, String> = BTreeMap::new();
    let mut json_arr = String::from("[\n");

    for (i, entry) in entries.iter().enumerate() {
        let json = generate_json_tagged(entry.metrics, entry.result, &entry.tags);
        let jname = format!("{}.json", entry.label);
        fs::write(json_dir.join(&jname), &json)?;
        json_files.insert(jname, json.clone());
        files_written += 1;

        if i > 0 {
            json_arr.push_str(",\n");
        }
        json_arr.push_str(&json);
    }

    json_arr.push_str("\n]");
    json_files.insert("summary.json".into(), json_arr.clone());
    fs::write(json_dir.join("summary.json"), &json_arr)?;
    files_written += 1;

    Ok((json_files, files_written))
}
