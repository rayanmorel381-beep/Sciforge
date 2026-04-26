use super::{Entry, PTABLE_CATS, capitalize_first, compute_group_stats};
use crate::parser::markdown::{write_md_code_block, write_md_heading, write_md_table};
use std::collections::BTreeMap;

pub(super) fn build_md(title: &str, entries: &[Entry<'_>], tag_keys: &[&str]) -> String {
    let mut md = String::with_capacity(entries.len() * 256);
    md.push_str(&write_md_heading(1, title));
    md.push_str(&format!(
        "\n{} benchmark entries across 5 formats.\n\n---\n\n",
        entries.len()
    ));

    let has_grid = entries.iter().any(|e| e.grid_row.is_some());

    if has_grid {
        let is_ptable_md = entries.iter().any(|e| {
            e.tags
                .iter()
                .any(|(k, v)| *k == "category" && PTABLE_CATS.contains(v))
        });

        let mut grid: BTreeMap<(u8, u8), &str> = BTreeMap::new();
        for entry in entries {
            if let (Some(r), Some(c)) = (entry.grid_row, entry.grid_col) {
                let sym = entry
                    .tags
                    .iter()
                    .find(|(k, _)| *k == "symbol")
                    .map(|(_, v)| *v)
                    .unwrap_or("?");
                grid.insert((r, c), sym);
            }
        }

        if is_ptable_md {
            md.push_str(&write_md_heading(2, "Periodic Table"));
            let mut grid_content = String::new();
            grid_content.push_str("       ");
            for c in 1..=18u8 {
                grid_content.push_str(&format!("{c:>4} "));
            }
            grid_content.push('\n');

            let rows: &[(u8, &str)] = &[
                (1, "  1"),
                (2, "  2"),
                (3, "  3"),
                (4, "  4"),
                (5, "  5"),
                (6, "  6"),
                (7, "  7"),
                (9, "  *"),
                (10, " **"),
            ];

            for &(r, label) in rows {
                grid_content.push_str(label);
                grid_content.push_str("    ");
                for c in 1..=18u8 {
                    if let Some(sym) = grid.get(&(r, c)) {
                        grid_content.push_str(&format!("{sym:>4} "));
                    } else if r == 6 && c == 3 {
                        grid_content.push_str("   * ");
                    } else if r == 7 && c == 3 {
                        grid_content.push_str("  ** ");
                    } else {
                        grid_content.push_str("     ");
                    }
                }
                grid_content.push('\n');
                if r == 7 {
                    grid_content.push('\n');
                }
            }
            md.push_str(&write_md_code_block(&grid_content));
            md.push_str("**Legend:** * Lanthanides (57-71) | ** Actinides (89-103)\n\n---\n\n");
        } else {
            let mut cats: Vec<&str> = Vec::new();
            let mut cat_syms: Vec<Vec<&str>> = Vec::new();
            for entry in entries {
                if entry.grid_row.is_none() || entry.grid_col.is_none() {
                    continue;
                }
                let cat = entry
                    .tags
                    .iter()
                    .find(|(k, _)| *k == "category")
                    .map(|(_, v)| *v)
                    .unwrap_or("other");
                let sym = entry
                    .tags
                    .iter()
                    .find(|(k, _)| *k == "symbol")
                    .map(|(_, v)| *v)
                    .unwrap_or("?");
                if let Some(pos) = cats.iter().position(|c| *c == cat) {
                    cat_syms[pos].push(sym);
                } else {
                    cats.push(cat);
                    cat_syms.push(vec![sym]);
                }
            }
            md.push_str(&write_md_heading(2, "Grid"));
            let mut grid_content = String::new();
            for (i, cat) in cats.iter().enumerate() {
                grid_content.push_str(&format!("  {cat:<16}: "));
                for (j, sym) in cat_syms[i].iter().enumerate() {
                    if j > 0 {
                        grid_content.push_str("  ");
                    }
                    grid_content.push_str(&format!("{sym:>3}"));
                }
                grid_content.push('\n');
            }
            md.push_str(&write_md_code_block(&grid_content));
            md.push_str("---\n\n");
        }
    }

    md.push_str(&write_md_heading(2, "Directory Layout"));
    md.push_str(&write_md_code_block("csv/\n  all.csv\njson/\n  {label}.json\n  summary.json\nyaml/\n  {label}.yaml\n  summary.yaml\ntoml/\n  {label}.toml\n  summary.toml\nbmk/\n  {label}.bmk\nbenchmark.html\nbenchmark.md"));
    md.push('\n');

    md.push_str(&write_md_heading(2, "Results"));
    let mut headers: Vec<&str> = vec!["Label"];
    for k in tag_keys {
        headers.push(k);
    }
    headers.extend_from_slice(&["Precision", "Iterations", "Avg (ns)", "Iters/s"]);
    let rows: Vec<Vec<String>> = entries
        .iter()
        .map(|entry| {
            let m = entry.metrics;
            let mut row = vec![entry.label.to_string()];
            for (_, v) in &entry.tags {
                row.push(v.to_string());
            }
            row.push(m.precision.to_string());
            row.push(m.iterations.to_string());
            row.push(format!("{:.2}", m.avg_time_ns));
            row.push(format!("{:.2}", m.iterations_per_sec));
            row
        })
        .collect();
    md.push_str(&write_md_table(&headers, &rows));

    if !entries.is_empty() {
        let group_key = entries[0].tags.first().map(|(k, _)| *k).unwrap_or("group");
        let cap_key = capitalize_first(group_key);

        let stats = compute_group_stats(entries);

        md.push_str(&write_md_heading(2, "Analytics"));
        md.push_str(&write_md_heading(3, &format!("{cap_key} Distribution")));
        let dist_headers: Vec<&str> = vec![&cap_key, "Count", "Avg (ns)", "Min (ns)", "Max (ns)"];
        let mut dist_rows: Vec<Vec<String>> = Vec::new();
        for &grp in &stats.groups {
            let count = stats.counts.get(grp).copied().unwrap_or(0);
            let avg = if count > 0 {
                stats.sums.get(grp).copied().unwrap_or(0.0) / count as f32
            } else {
                0.0
            };
            let mn = stats.mins.get(grp).copied().unwrap_or(0.0);
            let mx = stats.maxs.get(grp).copied().unwrap_or(0.0);
            dist_rows.push(vec![
                grp.to_string(),
                count.to_string(),
                format!("{avg:.2}"),
                format!("{mn:.2}"),
                format!("{mx:.2}"),
            ]);
        }
        md.push_str(&write_md_table(&dist_headers, &dist_rows));
        md.push('\n');

        let mut sorted: Vec<(&str, f32)> = entries
            .iter()
            .map(|e| (e.label, e.metrics.avg_time_ns))
            .collect();
        sorted.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(core::cmp::Ordering::Equal));

        let top_n = 10.min(sorted.len());

        let top_headers: &[&str] = &["#", "Entry", "Avg (ns)"];
        md.push_str(&write_md_heading(3, "Top 10 Fastest Entries"));
        let fast_rows: Vec<Vec<String>> = sorted
            .iter()
            .take(top_n)
            .enumerate()
            .map(|(i, &(lbl, ns))| vec![(i + 1).to_string(), lbl.to_string(), format!("{ns:.2}")])
            .collect();
        md.push_str(&write_md_table(top_headers, &fast_rows));
        md.push('\n');

        md.push_str(&write_md_heading(3, "Top 10 Slowest Entries"));
        let slow_rows: Vec<Vec<String>> = sorted
            .iter()
            .rev()
            .take(top_n)
            .enumerate()
            .map(|(i, &(lbl, ns))| vec![(i + 1).to_string(), lbl.to_string(), format!("{ns:.2}")])
            .collect();
        md.push_str(&write_md_table(top_headers, &slow_rows));
        md.push('\n');
    }

    md
}
