mod charts;
mod layout;

use super::{Entry, PTABLE_CATS};

fn css_cat(cat: &str) -> &'static str {
    match cat {
        "nonmetal" => "nonmetal",
        "noble gas" => "noble-gas",
        "alkali metal" => "alkali",
        "alkaline earth" => "alkaline",
        "transition metal" => "transition",
        "post-transition" => "post-trans",
        "metalloid" => "metalloid",
        "halogen" => "halogen",
        "lanthanide" => "lanthanide",
        "actinide" => "actinide",
        _ => "unknown",
    }
}

pub(super) fn group_colors(idx: usize) -> (&'static str, &'static str) {
    const COLORS: &[(&str, &str)] = &[
        ("#7ee787", "#1a3a1a"),
        ("#79c0ff", "#0a2a3a"),
        ("#ffa657", "#3a2a0a"),
        ("#d2a8ff", "#3a1a2e"),
        ("#ff7eb6", "#3a1a2a"),
        ("#f0b27a", "#3a2a1a"),
        ("#7ee7b0", "#1a3a2a"),
        ("#bc8cff", "#2a1a3a"),
        ("#e2b0ff", "#2e1a3a"),
        ("#ffd700", "#3a3a0a"),
        ("#ff6b6b", "#3a1a1a"),
        ("#4ecdc4", "#0a3a36"),
    ];
    let i = idx % COLORS.len();
    COLORS[i]
}

pub(super) use layout::build_html;
