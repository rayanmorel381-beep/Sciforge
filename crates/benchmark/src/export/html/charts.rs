use super::super::{capitalize_first, compute_group_stats};
use super::{Entry, group_colors};

pub(super) fn build_charts(h: &mut String, entries: &[Entry<'_>]) {
    if entries.is_empty() {
        return;
    }

    let group_key = entries[0].tags.first().map(|(k, _)| *k).unwrap_or("group");
    let cap_key = capitalize_first(group_key);
    let stats = compute_group_stats(entries);

    h.push_str("<h2>Analytics</h2>\n<div class=\"charts\">\n");

    let max_count = stats
        .groups
        .iter()
        .map(|g| stats.counts.get(g).copied().unwrap_or(0))
        .max()
        .unwrap_or(1);
    let bar_h = 28u32;
    let label_w = 160u32;
    let chart_w = 650u32;
    let bar_area = chart_w - label_w - 60;
    let svg_h = stats.groups.len() as u32 * bar_h + 10;

    h.push_str(&format!(
        "<div class=\"chart-box\">\n<h3>{cap_key} Distribution</h3>\n"
    ));
    h.push_str(&format!(
        "<svg viewBox=\"0 0 {chart_w} {svg_h}\" style=\"width:100%;max-width:{chart_w}px\">\n"
    ));
    for (i, &grp) in stats.groups.iter().enumerate() {
        let count = stats.counts.get(grp).copied().unwrap_or(0);
        let y = i as u32 * bar_h + 4;
        let w = if max_count > 0 {
            (count as u32 * bar_area) / max_count as u32
        } else {
            0
        };
        let (color, bg) = group_colors(i);
        h.push_str(&format!(
            "<text x=\"{label_w}\" y=\"{}\" fill=\"#8b949e\" font-size=\"12\" text-anchor=\"end\" dominant-baseline=\"middle\">{grp}</text>\n",
            y + bar_h / 2
        ));
        h.push_str(&format!(
            "<rect x=\"{}\" y=\"{y}\" width=\"{bar_area}\" height=\"{}\" rx=\"4\" fill=\"{bg}\"/>\n",
            label_w + 8, bar_h - 6
        ));
        h.push_str(&format!(
            "<rect x=\"{}\" y=\"{y}\" width=\"{w}\" height=\"{}\" rx=\"4\" fill=\"{color}\" opacity=\".8\"/>\n",
            label_w + 8, bar_h - 6
        ));
        h.push_str(&format!(
            "<text x=\"{}\" y=\"{}\" fill=\"#c9d1d9\" font-size=\"11\" dominant-baseline=\"middle\">{count}</text>\n",
            label_w + 8 + w + 6, y + bar_h / 2
        ));
    }
    h.push_str("</svg>\n</div>\n\n");

    let max_avg: f64 = stats
        .groups
        .iter()
        .filter_map(|g| {
            let cnt = *stats.counts.get(g)?;
            if cnt == 0 {
                return None;
            }
            Some(stats.sums.get(g).copied().unwrap_or(0.0) / cnt as f64)
        })
        .fold(0.0f64, f64::max);

    h.push_str(&format!(
        "<div class=\"chart-box\">\n<h3>Average Time per {cap_key} (ns)</h3>\n"
    ));
    h.push_str(&format!(
        "<svg viewBox=\"0 0 {chart_w} {svg_h}\" style=\"width:100%;max-width:{chart_w}px\">\n"
    ));
    for (i, &grp) in stats.groups.iter().enumerate() {
        let count = stats.counts.get(grp).copied().unwrap_or(0);
        let avg = if count > 0 {
            stats.sums.get(grp).copied().unwrap_or(0.0) / count as f64
        } else {
            0.0
        };
        let y = i as u32 * bar_h + 4;
        let w = if max_avg > 0.0 {
            (avg / max_avg * bar_area as f64) as u32
        } else {
            0
        };
        let (color, bg) = group_colors(i);
        h.push_str(&format!(
            "<text x=\"{label_w}\" y=\"{}\" fill=\"#8b949e\" font-size=\"12\" text-anchor=\"end\" dominant-baseline=\"middle\">{grp}</text>\n",
            y + bar_h / 2
        ));
        h.push_str(&format!(
            "<rect x=\"{}\" y=\"{y}\" width=\"{bar_area}\" height=\"{}\" rx=\"4\" fill=\"{bg}\"/>\n",
            label_w + 8, bar_h - 6
        ));
        h.push_str(&format!(
            "<rect x=\"{}\" y=\"{y}\" width=\"{w}\" height=\"{}\" rx=\"4\" fill=\"{color}\" opacity=\".8\"/>\n",
            label_w + 8, bar_h - 6
        ));
        h.push_str(&format!(
            "<text x=\"{}\" y=\"{}\" fill=\"#c9d1d9\" font-size=\"11\" dominant-baseline=\"middle\">{:.1}</text>\n",
            label_w + 8 + w + 6, y + bar_h / 2, avg
        ));
    }
    h.push_str("</svg>\n</div>\n\n");

    let mut sorted: Vec<(&str, usize, f64)> = entries
        .iter()
        .enumerate()
        .map(|(idx, e)| (e.label, idx, e.metrics.avg_time_ns))
        .collect();
    sorted.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap_or(core::cmp::Ordering::Equal));

    let top_n = 10.min(sorted.len());
    let top_max = sorted.last().map(|e| e.2).unwrap_or(1.0);

    h.push_str("<div class=\"chart-box\">\n<h3>Top 10 Fastest Entries</h3>\n");
    let fast_h = top_n as u32 * bar_h + 10;
    h.push_str(&format!(
        "<svg viewBox=\"0 0 {chart_w} {fast_h}\" style=\"width:100%;max-width:{chart_w}px\">\n"
    ));
    for (i, &(lbl, ei, ns)) in sorted.iter().take(top_n).enumerate() {
        let y = i as u32 * bar_h + 4;
        let w = if top_max > 0.0 {
            (ns / top_max * bar_area as f64) as u32
        } else {
            0
        };
        let w = w.max(2);
        let gi = stats
            .groups
            .iter()
            .position(|g| entries[ei].tags.first().map(|(_, v)| *v) == Some(g))
            .unwrap_or(0);
        let (color, _) = group_colors(gi);
        let short = if lbl.len() > 20 {
            &lbl[lbl.len() - 20..]
        } else {
            lbl
        };
        h.push_str(&format!(
            "<text x=\"{label_w}\" y=\"{}\" fill=\"#8b949e\" font-size=\"11\" text-anchor=\"end\" dominant-baseline=\"middle\">{short}</text>\n",
            y + bar_h / 2
        ));
        h.push_str(&format!(
            "<rect x=\"{}\" y=\"{y}\" width=\"{w}\" height=\"{}\" rx=\"4\" fill=\"{color}\" opacity=\".8\"/>\n",
            label_w + 8, bar_h - 6
        ));
        h.push_str(&format!(
            "<text x=\"{}\" y=\"{}\" fill=\"#c9d1d9\" font-size=\"11\" dominant-baseline=\"middle\">{ns:.1} ns</text>\n",
            label_w + 8 + w + 6, y + bar_h / 2
        ));
    }
    h.push_str("</svg>\n</div>\n\n");

    h.push_str("<div class=\"chart-box\">\n<h3>Top 10 Slowest Entries</h3>\n");
    h.push_str(&format!(
        "<svg viewBox=\"0 0 {chart_w} {fast_h}\" style=\"width:100%;max-width:{chart_w}px\">\n"
    ));
    for (i, &(lbl, ei, ns)) in sorted.iter().rev().take(top_n).enumerate() {
        let y = i as u32 * bar_h + 4;
        let w = if top_max > 0.0 {
            (ns / top_max * bar_area as f64) as u32
        } else {
            0
        };
        let w = w.max(2);
        let gi = stats
            .groups
            .iter()
            .position(|g| entries[ei].tags.first().map(|(_, v)| *v) == Some(g))
            .unwrap_or(0);
        let (color, _) = group_colors(gi);
        let short = if lbl.len() > 20 {
            &lbl[lbl.len() - 20..]
        } else {
            lbl
        };
        h.push_str(&format!(
            "<text x=\"{label_w}\" y=\"{}\" fill=\"#8b949e\" font-size=\"11\" text-anchor=\"end\" dominant-baseline=\"middle\">{short}</text>\n",
            y + bar_h / 2
        ));
        h.push_str(&format!(
            "<rect x=\"{}\" y=\"{y}\" width=\"{w}\" height=\"{}\" rx=\"4\" fill=\"{color}\" opacity=\".8\"/>\n",
            label_w + 8, bar_h - 6
        ));
        h.push_str(&format!(
            "<text x=\"{}\" y=\"{}\" fill=\"#c9d1d9\" font-size=\"11\" dominant-baseline=\"middle\">{ns:.1} ns</text>\n",
            label_w + 8 + w + 6, y + bar_h / 2
        ));
    }
    h.push_str("</svg>\n</div>\n\n");

    let cat_cards_h = bar_h + 6;
    h.push_str(&format!(
        "<div class=\"chart-box\" style=\"grid-column:1/-1\">\n<h3>{cap_key} Statistics</h3>\n"
    ));
    let card_svg_h = stats.groups.len() as u32 * cat_cards_h + 10;
    h.push_str(&format!(
        "<svg viewBox=\"0 0 900 {card_svg_h}\" style=\"width:100%;max-width:900px\">\n"
    ));
    for (i, &grp) in stats.groups.iter().enumerate() {
        let y = i as u32 * cat_cards_h + 4;
        let count = stats.counts.get(grp).copied().unwrap_or(0);
        let avg = if count > 0 {
            stats.sums.get(grp).copied().unwrap_or(0.0) / count as f64
        } else {
            0.0
        };
        let mn = stats.mins.get(grp).copied().unwrap_or(0.0);
        let mx = stats.maxs.get(grp).copied().unwrap_or(0.0);
        let (color, _) = group_colors(i);
        h.push_str(&format!(
            "<rect x=\"0\" y=\"{y}\" width=\"900\" height=\"{bar_h}\" rx=\"4\" fill=\"#161b22\"/>\n"
        ));
        h.push_str(&format!(
            "<rect x=\"0\" y=\"{y}\" width=\"4\" height=\"{bar_h}\" rx=\"2\" fill=\"{color}\"/>\n"
        ));
        h.push_str(&format!(
            "<text x=\"14\" y=\"{}\" fill=\"{color}\" font-size=\"12\" font-weight=\"bold\" dominant-baseline=\"middle\">{grp}</text>\n",
            y + bar_h / 2
        ));
        h.push_str(&format!(
            "<text x=\"250\" y=\"{}\" fill=\"#8b949e\" font-size=\"11\" dominant-baseline=\"middle\">n={count}</text>\n",
            y + bar_h / 2
        ));
        h.push_str(&format!(
            "<text x=\"350\" y=\"{}\" fill=\"#c9d1d9\" font-size=\"11\" dominant-baseline=\"middle\">avg {avg:.1} ns</text>\n",
            y + bar_h / 2
        ));
        h.push_str(&format!(
            "<text x=\"520\" y=\"{}\" fill=\"#8b949e\" font-size=\"11\" dominant-baseline=\"middle\">min {mn:.1} ns</text>\n",
            y + bar_h / 2
        ));
        h.push_str(&format!(
            "<text x=\"680\" y=\"{}\" fill=\"#8b949e\" font-size=\"11\" dominant-baseline=\"middle\">max {mx:.1} ns</text>\n",
            y + bar_h / 2
        ));
    }
    h.push_str("</svg>\n</div>\n\n");

    h.push_str("</div>\n\n");
}
