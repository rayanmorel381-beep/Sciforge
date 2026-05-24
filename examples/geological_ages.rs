use sciforge_hub::benchmark::decode::decode;
use sciforge_hub::benchmark::engine::bench;
use sciforge_hub::benchmark::export::{Entry, export};
use sciforge_hub::benchmark::report::generate;
use sciforge_hub::prelude::constants::elements;
use sciforge_hub::prelude::geology::dating;
use std::fs;
use std::path::Path;

struct GeoSample {
    name: &'static str,
    method: &'static str,
    category: &'static str,
    parent_sym: &'static str,
    parent_iso: &'static str,
    daughter: &'static str,
    ratio: f64,
    expected_age_myr: f64,
}

const SAMPLES: [GeoSample; 10] = [
    GeoSample {
        name: "Jack Hills zircon",
        method: "U-Pb (238)",
        category: "Hadean",
        parent_sym: "U",
        parent_iso: "\u{00b2}\u{00b3}\u{2078}U",
        daughter: "\u{00b2}\u{2070}\u{2076}Pb",
        ratio: 1.02,
        expected_age_myr: 4400.0,
    },
    GeoSample {
        name: "Acasta Gneiss",
        method: "U-Pb (238)",
        category: "Hadean",
        parent_sym: "U",
        parent_iso: "\u{00b2}\u{00b3}\u{2078}U",
        daughter: "\u{00b2}\u{2070}\u{2076}Pb",
        ratio: 0.85,
        expected_age_myr: 4030.0,
    },
    GeoSample {
        name: "Isua Greenstone",
        method: "U-Pb (235)",
        category: "Archean",
        parent_sym: "U",
        parent_iso: "\u{00b2}\u{00b3}\u{2075}U",
        daughter: "\u{00b2}\u{2070}\u{2077}Pb",
        ratio: 0.58,
        expected_age_myr: 3800.0,
    },
    GeoSample {
        name: "Pilbara Craton",
        method: "K-Ar",
        category: "Archean",
        parent_sym: "K",
        parent_iso: "\u{2074}\u{2070}K",
        daughter: "\u{2074}\u{2070}Ar",
        ratio: 0.11,
        expected_age_myr: 3500.0,
    },
    GeoSample {
        name: "Kaapvaal Craton",
        method: "U-Pb (238)",
        category: "Archean",
        parent_sym: "U",
        parent_iso: "\u{00b2}\u{00b3}\u{2078}U",
        daughter: "\u{00b2}\u{2070}\u{2076}Pb",
        ratio: 0.65,
        expected_age_myr: 3100.0,
    },
    GeoSample {
        name: "Bushveld Complex",
        method: "U-Pb (238)",
        category: "Proterozoic",
        parent_sym: "U",
        parent_iso: "\u{00b2}\u{00b3}\u{2078}U",
        daughter: "\u{00b2}\u{2070}\u{2076}Pb",
        ratio: 0.32,
        expected_age_myr: 2055.0,
    },
    GeoSample {
        name: "Sudbury Impact",
        method: "U-Pb (238)",
        category: "Proterozoic",
        parent_sym: "U",
        parent_iso: "\u{00b2}\u{00b3}\u{2078}U",
        daughter: "\u{00b2}\u{2070}\u{2076}Pb",
        ratio: 0.28,
        expected_age_myr: 1849.0,
    },
    GeoSample {
        name: "Deccan Traps",
        method: "K-Ar",
        category: "Mesozoic",
        parent_sym: "K",
        parent_iso: "\u{2074}\u{2070}K",
        daughter: "\u{2074}\u{2070}Ar",
        ratio: 0.005,
        expected_age_myr: 66.0,
    },
    GeoSample {
        name: "KT Boundary clay",
        method: "C-14",
        category: "Cenozoic",
        parent_sym: "C",
        parent_iso: "\u{00b9}\u{2074}C",
        daughter: "\u{00b9}\u{2074}N",
        ratio: 0.0,
        expected_age_myr: 0.065,
    },
    GeoSample {
        name: "Lascaux paintings",
        method: "C-14",
        category: "Cenozoic",
        parent_sym: "C",
        parent_iso: "\u{00b9}\u{2074}C",
        daughter: "\u{00b9}\u{2074}N",
        ratio: 0.82,
        expected_age_myr: 0.017,
    },
];

fn main() {
    let dir = Path::new("output/examples/geological_ages");
    fs::create_dir_all(dir).unwrap();
    let all = elements::all();

    let hl_strs: Vec<String> = SAMPLES
        .iter()
        .map(|s| {
            let e = all.iter().find(|e| e.symbol == s.parent_sym).unwrap();
            if let Some(iso) = e.isotopes.iter().find(|i| i.symbol == s.parent_iso) {
                let hl = iso.half_life.unwrap_or(0.0);
                let unit = iso.half_life_unit.unwrap_or("s");
                format!("{:.4e} {}", hl, unit)
            } else {
                "-".to_string()
            }
        })
        .collect();

    let computed_age_strs: Vec<String> = SAMPLES
        .iter()
        .map(|s| match s.method {
            "U-Pb (238)" => {
                let age_yr = dating::uranium_lead_age(s.ratio, 1.0);
                format!("{:.1} Myr", age_yr / 1e6)
            }
            "U-Pb (235)" => {
                let age_yr = dating::uranium_235_lead_age(s.ratio, 1.0);
                format!("{:.1} Myr", age_yr / 1e6)
            }
            "K-Ar" => {
                let age_yr = dating::potassium_argon_age(s.ratio, 1.0);
                format!("{:.1} Myr", age_yr / 1e6)
            }
            "C-14" => {
                if s.ratio > 0.0 {
                    let age_yr = dating::carbon14_age(s.ratio);
                    format!("{:.0} yr", age_yr)
                } else {
                    "beyond range".to_string()
                }
            }
            _ => "-".to_string(),
        })
        .collect();

    let decay_const_strs: Vec<String> = SAMPLES
        .iter()
        .map(|s| {
            let e = all.iter().find(|e| e.symbol == s.parent_sym).unwrap();
            if let Some(iso) = e.isotopes.iter().find(|i| i.symbol == s.parent_iso) {
                let hl = iso.half_life.unwrap_or(0.0);
                let unit = iso.half_life_unit.unwrap_or("s");
                let hl_s = match unit {
                    "years" | "year" => hl * 3.156e7,
                    "days" | "day" => hl * 86400.0,
                    _ => hl,
                };
                if hl_s > 0.0 {
                    let lam = dating::decay_constant(hl_s);
                    format!("{:.4e} s\u{207b}\u{00b9}", lam)
                } else {
                    "-".to_string()
                }
            } else {
                "-".to_string()
            }
        })
        .collect();

    let method_strs: Vec<String> = SAMPLES.iter().map(|s| s.method.to_string()).collect();
    let ratio_strs: Vec<String> = SAMPLES.iter().map(|s| format!("{:.4}", s.ratio)).collect();
    let expected_strs: Vec<String> = SAMPLES
        .iter()
        .map(|s| {
            if s.expected_age_myr > 1.0 {
                format!("{:.0} Myr", s.expected_age_myr)
            } else {
                format!("{:.1} kyr", s.expected_age_myr * 1000.0)
            }
        })
        .collect();

    let mut metrics_store: Vec<(String, String, _)> = Vec::new();

    let exp_names: Vec<String> = SAMPLES
        .iter()
        .map(|s| format!("{}_dating", s.name))
        .collect();

    for (idx, s) in SAMPLES.iter().enumerate() {
        let r = s.ratio;
        let method = s.method;
        let m = bench(&exp_names[idx], "f64", 5000, move || {
            let mut v = 0u64;
            for i in 0..100 {
                let ratio = r + (i as f64) * 0.001;
                let age = match method {
                    "U-Pb (238)" => dating::uranium_lead_age(ratio, 1.0),
                    "U-Pb (235)" => dating::uranium_235_lead_age(ratio, 1.0),
                    "K-Ar" => dating::potassium_argon_age(ratio, 1.0),
                    "C-14" if ratio > 0.0 => dating::carbon14_age(ratio),
                    _ => 0.0,
                };
                v = v.wrapping_add(age.to_bits());
            }
            v
        });
        let result = format!(
            "{} | {} | D/P={} | t\u{00bd}={} | Computed: {} | Expected: {}",
            s.name,
            s.method,
            ratio_strs[idx],
            hl_strs[idx],
            computed_age_strs[idx],
            expected_strs[idx]
        );
        let rpt = generate(&m, &result);
        assert!(rpt.csv.starts_with("experiment_name"));
        metrics_store.push((s.name.to_string(), result, m));
    }

    let entries: Vec<Entry<'_>> = metrics_store
        .iter()
        .enumerate()
        .map(|(idx, (label, result, m))| Entry {
            metrics: m,
            result: result.as_str(),
            label: label.as_str(),
            tags: vec![
                ("category", SAMPLES[idx].category),
                ("symbol", SAMPLES[idx].parent_sym),
                ("name", SAMPLES[idx].name),
                ("method", method_strs[idx].as_str()),
                ("parent", SAMPLES[idx].parent_iso),
                ("daughter", SAMPLES[idx].daughter),
                ("ratio", ratio_strs[idx].as_str()),
                ("half_life", hl_strs[idx].as_str()),
                ("decay_constant", decay_const_strs[idx].as_str()),
                ("computed_age", computed_age_strs[idx].as_str()),
                ("expected_age", expected_strs[idx].as_str()),
            ],
            grid_row: Some((idx as u8 / 5) + 1),
            grid_col: Some((idx as u8 % 5) + 1),
        })
        .collect();

    let summary = export(
        "Geological Ages \u{2014} Radiometric Dating with Isotope Half-lives",
        &entries,
        dir,
    )
    .unwrap();
    assert!(summary.files_written > 0);

    let bmk_path = dir.join("bmk/geological_ages.bmk");
    let bytes = fs::read(&bmk_path).unwrap();
    let view = decode(&bytes).unwrap();
    assert!(!view.experiment_name.is_empty());
}
