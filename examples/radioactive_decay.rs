use sciforge_hub::benchmark::decode::decode;
use sciforge_hub::benchmark::engine::bench;
use sciforge_hub::benchmark::export::{Entry, export};
use sciforge_hub::benchmark::report::generate;
use sciforge_hub::prelude::chemistry::nuclear::decay;
use sciforge_hub::prelude::chemistry::nuclear::energy;
use sciforge_hub::prelude::constants::elements;
use sciforge_hub::prelude::constants::*;
use std::fs;
use std::path::Path;

fn main() {
    let dir = Path::new("output/examples/radioactive_decay");
    fs::create_dir_all(dir).unwrap();
    let all = elements::all();

    let targets: &[(&str, &str)] = &[
        ("U", "\u{00b2}\u{00b3}\u{2078}U"),
        ("U", "\u{00b2}\u{00b3}\u{2075}U"),
        ("Th", "\u{00b2}\u{00b3}\u{00b2}Th"),
        ("K", "\u{2074}\u{2070}K"),
        ("C", "\u{00b9}\u{2074}C"),
        ("Ra", "\u{00b2}\u{00b2}\u{2076}Ra"),
        ("Sr", "\u{2079}\u{2070}Sr"),
        ("Cs", "\u{00b9}\u{00b3}\u{2077}Cs"),
        ("I", "\u{00b9}\u{00b3}\u{00b9}I"),
        ("Co", "\u{2076}\u{2070}Co"),
        ("Pu", "\u{00b2}\u{00b3}\u{2079}Pu"),
        ("Am", "\u{00b2}\u{2074}\u{00b9}Am"),
    ];

    struct IsoData {
        element_name: String,
        iso_symbol: String,
        z: u32,
        a: u32,
        n: u32,
        half_life_s: f64,
        half_life_display: String,
        stable: bool,
        decay_mode: String,
        category: String,
    }

    let mut data: Vec<IsoData> = Vec::new();

    for &(sym, iso_sym) in targets {
        let e = all.iter().find(|e| e.symbol == sym).unwrap();
        if let Some(iso) = e.isotopes.iter().find(|i| i.symbol == iso_sym) {
            let hl = iso.half_life.unwrap_or(0.0);
            let unit = iso.half_life_unit.unwrap_or("s");
            let hl_s = match unit {
                "years" | "year" => hl * 3.156e7,
                "days" | "day" => hl * 86400.0,
                "hours" | "hour" => hl * 3600.0,
                "minutes" | "minute" => hl * 60.0,
                _ => hl,
            };
            let hl_display = if hl > 1e9 {
                format!("{:.3e} {}", hl, unit)
            } else if hl > 1.0 {
                format!("{:.2} {}", hl, unit)
            } else {
                format!("{:.3e} {}", hl, unit)
            };
            let dm = iso.decay_modes.first().map(|d| d.mode).unwrap_or("-");
            let cat = match dm {
                "alpha" | "\u{03b1}" => "Alpha",
                "beta_minus" | "\u{03b2}\u{207b}" | "beta-" | "B-" => "Beta",
                "electron_capture" | "EC" => "EC",
                _ => "Other",
            };
            data.push(IsoData {
                element_name: e.name.to_string(),
                iso_symbol: iso.symbol.to_string(),
                z: e.atomic_number,
                a: iso.mass_number,
                n: iso.neutrons,
                half_life_s: hl_s,
                half_life_display: hl_display,
                stable: iso.stable,
                decay_mode: dm.to_string(),
                category: cat.to_string(),
            });
        }
    }

    let lambda_strs: Vec<String> = data
        .iter()
        .map(|d| {
            if !d.stable {
                let lam = decay::half_life_to_decay_constant(d.half_life_s);
                format!("{:.4e} s\u{207b}\u{00b9}", lam)
            } else {
                "stable".to_string()
            }
        })
        .collect();

    let activity_strs: Vec<String> = data
        .iter()
        .map(|d| {
            if !d.stable {
                let lam = decay::half_life_to_decay_constant(d.half_life_s);
                let sa = decay::specific_activity(lam, N_A, d.a as f64);
                format!("{:.3e} Bq/g", sa)
            } else {
                "-".to_string()
            }
        })
        .collect();

    let remaining_strs: Vec<String> = data
        .iter()
        .map(|d| {
            if !d.stable {
                let lam = decay::half_life_to_decay_constant(d.half_life_s);
                let n = decay::remaining_nuclei(1e6, lam, d.half_life_s * 3.0);
                format!("{:.0} / 1M after 3 half-lives", n)
            } else {
                "-".to_string()
            }
        })
        .collect();

    let be_strs: Vec<String> = data
        .iter()
        .map(|d| {
            let md = energy::mass_defect(d.z, d.n, d.a as f64 * AMU_TO_KG / AMU_TO_KG);
            let be = energy::binding_energy(md);
            let be_per_a = energy::binding_energy_per_nucleon(be, d.a);
            format!("{:.3} MeV/nucleon", be_per_a)
        })
        .collect();

    let hl_strs: Vec<String> = data.iter().map(|d| d.half_life_display.clone()).collect();
    let dm_strs: Vec<String> = data.iter().map(|d| d.decay_mode.clone()).collect();
    let cat_strs: Vec<String> = data.iter().map(|d| d.category.clone()).collect();
    let z_strs: Vec<String> = data.iter().map(|d| format!("{}", d.z)).collect();
    let a_strs: Vec<String> = data.iter().map(|d| format!("{}", d.a)).collect();
    let n_strs: Vec<String> = data.iter().map(|d| format!("{}", d.n)).collect();

    let mut metrics_store: Vec<(String, String, _)> = Vec::new();

    let exp_names: Vec<String> = data
        .iter()
        .map(|d| format!("{}_{}_decay", d.element_name, d.a))
        .collect();

    for (idx, d) in data.iter().enumerate() {
        let hl_s = d.half_life_s;
        let a_val = d.a;
        let is_stable = d.stable;
        let m = bench(&exp_names[idx], "f64", 5000, move || {
            let mut v = 0u64;
            if !is_stable {
                let lam = decay::half_life_to_decay_constant(hl_s);
                for i in 0..100 {
                    let t = hl_s * (i as f64) / 100.0;
                    let n = decay::remaining_nuclei(1e6, lam, t);
                    let act = decay::activity(lam, n);
                    v = v.wrapping_add(n.to_bits()).wrapping_add(act.to_bits());
                }
                let sa = decay::specific_activity(lam, N_A, a_val as f64);
                v = v.wrapping_add(sa.to_bits());
            }
            v
        });
        let result = format!(
            "{} ({}) | t\u{00bd}={} | \u{03bb}={} | {} | A_sp={}",
            d.iso_symbol,
            d.element_name,
            d.half_life_display,
            lambda_strs[idx],
            d.decay_mode,
            activity_strs[idx]
        );
        let rpt = generate(&m, &result);
        assert!(rpt.csv.starts_with("experiment_name"));
        metrics_store.push((d.iso_symbol.clone(), result, m));
    }

    let entries: Vec<Entry<'_>> = metrics_store
        .iter()
        .enumerate()
        .map(|(idx, (label, result, m))| Entry {
            metrics: m,
            result: result.as_str(),
            label: label.as_str(),
            tags: vec![
                ("category", cat_strs[idx].as_str()),
                ("symbol", data[idx].iso_symbol.as_str()),
                ("name", data[idx].element_name.as_str()),
                ("Z", z_strs[idx].as_str()),
                ("A", a_strs[idx].as_str()),
                ("N", n_strs[idx].as_str()),
                ("half_life", hl_strs[idx].as_str()),
                ("decay_constant", lambda_strs[idx].as_str()),
                ("decay_mode", dm_strs[idx].as_str()),
                ("specific_activity", activity_strs[idx].as_str()),
                ("remaining_3hl", remaining_strs[idx].as_str()),
                ("binding_energy", be_strs[idx].as_str()),
            ],
            grid_row: Some((idx as u8 / 4) + 1),
            grid_col: Some((idx as u8 % 4) + 1),
        })
        .collect();

    let summary = export(
        "Radioactive Decay \u{2014} Chains, Half-lives & Activities from Isotope Data",
        &entries,
        dir,
    )
    .unwrap();
    assert!(summary.files_written > 0);

    let bmk_path = dir.join("bmk/radioactive_decay.bmk");
    let bytes = fs::read(&bmk_path).unwrap();
    let view = decode(&bytes).unwrap();
    assert!(!view.experiment_name.is_empty());
}
