use sciforge::benchmark::decode::decode;
use sciforge::benchmark::engine::bench;
use sciforge::benchmark::export::{Entry, export};
use sciforge::benchmark::report::generate;
use sciforge::hub::prelude::astronomy::stellar;
use sciforge::hub::prelude::constants::*;
use std::fs;
use std::path::Path;

struct Star {
    name: &'static str,
    spectral: &'static str,
    category: &'static str,
    mass_solar: f64,
    radius_solar: f64,
    temp_k: f64,
    distance_pc: f64,
    apparent_mag: f64,
}

const STARS: [Star; 10] = [
    Star {
        name: "Sun",
        spectral: "G2V",
        category: "Main Sequence",
        mass_solar: 1.0,
        radius_solar: 1.0,
        temp_k: 5778.0,
        distance_pc: 4.848e-6,
        apparent_mag: -26.74,
    },
    Star {
        name: "Sirius A",
        spectral: "A1V",
        category: "Main Sequence",
        mass_solar: 2.063,
        radius_solar: 1.711,
        temp_k: 9940.0,
        distance_pc: 2.637,
        apparent_mag: -1.46,
    },
    Star {
        name: "Betelgeuse",
        spectral: "M1Ia",
        category: "Supergiant",
        mass_solar: 16.5,
        radius_solar: 887.0,
        temp_k: 3600.0,
        distance_pc: 168.1,
        apparent_mag: 0.42,
    },
    Star {
        name: "Rigel",
        spectral: "B8Ia",
        category: "Supergiant",
        mass_solar: 21.0,
        radius_solar: 78.9,
        temp_k: 12100.0,
        distance_pc: 264.6,
        apparent_mag: 0.13,
    },
    Star {
        name: "Vega",
        spectral: "A0V",
        category: "Main Sequence",
        mass_solar: 2.135,
        radius_solar: 2.362,
        temp_k: 9602.0,
        distance_pc: 7.68,
        apparent_mag: 0.03,
    },
    Star {
        name: "Proxima Centauri",
        spectral: "M5.5Ve",
        category: "Red Dwarf",
        mass_solar: 0.122,
        radius_solar: 0.1542,
        temp_k: 3042.0,
        distance_pc: 1.301,
        apparent_mag: 11.13,
    },
    Star {
        name: "Polaris",
        spectral: "F7Ib",
        category: "Supergiant",
        mass_solar: 5.4,
        radius_solar: 37.5,
        temp_k: 6015.0,
        distance_pc: 132.6,
        apparent_mag: 1.98,
    },
    Star {
        name: "Aldebaran",
        spectral: "K5III",
        category: "Giant",
        mass_solar: 1.16,
        radius_solar: 44.13,
        temp_k: 3910.0,
        distance_pc: 20.43,
        apparent_mag: 0.86,
    },
    Star {
        name: "Deneb",
        spectral: "A2Ia",
        category: "Supergiant",
        mass_solar: 19.0,
        radius_solar: 203.0,
        temp_k: 8525.0,
        distance_pc: 802.0,
        apparent_mag: 1.25,
    },
    Star {
        name: "Barnard's Star",
        spectral: "M4Ve",
        category: "Red Dwarf",
        mass_solar: 0.144,
        radius_solar: 0.196,
        temp_k: 3134.0,
        distance_pc: 1.834,
        apparent_mag: 9.51,
    },
];

fn main() {
    let dir = Path::new("output/examples/star_catalog");
    fs::create_dir_all(dir).unwrap();

    let lum_strs: Vec<String> = STARS
        .iter()
        .map(|s| {
            let r = s.radius_solar * SOLAR_RADIUS;
            let l = stellar::luminosity_from_radius_temp(r, s.temp_k);
            format!("{:.3e} W", l)
        })
        .collect();

    let abs_mag_strs: Vec<String> = STARS
        .iter()
        .map(|s| {
            let m = stellar::absolute_magnitude(s.apparent_mag, s.distance_pc);
            format!("{:.2}", m)
        })
        .collect();

    let schwarzschild_strs: Vec<String> = STARS
        .iter()
        .map(|s| {
            let r = stellar::schwarzschild_radius(s.mass_solar * SOLAR_MASS);
            format!("{:.2} m", r)
        })
        .collect();

    let lifetime_strs: Vec<String> = STARS
        .iter()
        .map(|s| {
            let l_solar = stellar::mass_luminosity_relation(s.mass_solar);
            let t = stellar::main_sequence_lifetime(s.mass_solar, l_solar);
            let gyr = t / (3.156e7 * 1e9);
            if gyr > 1.0 {
                format!("{:.2} Gyr", gyr)
            } else {
                format!("{:.1} Myr", gyr * 1000.0)
            }
        })
        .collect();

    let wien_strs: Vec<String> = STARS
        .iter()
        .map(|s| {
            let wl = stellar::wien_peak_wavelength(s.temp_k);
            format!("{:.0} nm", wl * 1e9)
        })
        .collect();

    let mass_strs: Vec<String> = STARS
        .iter()
        .map(|s| format!("{:.3} M\u{2609}", s.mass_solar))
        .collect();
    let radius_strs: Vec<String> = STARS
        .iter()
        .map(|s| format!("{:.3} R\u{2609}", s.radius_solar))
        .collect();
    let temp_strs: Vec<String> = STARS.iter().map(|s| format!("{:.0} K", s.temp_k)).collect();

    let mut metrics_store: Vec<(String, String, _)> = Vec::new();

    let exp_names: Vec<String> = STARS
        .iter()
        .map(|s| format!("{}_stellar", s.name))
        .collect();

    for (idx, s) in STARS.iter().enumerate() {
        let r_m = s.radius_solar * SOLAR_RADIUS;
        let t_k = s.temp_k;
        let mass_s = s.mass_solar;
        let m = bench(&exp_names[idx], "f64", 5000, move || {
            let mut v = 0u64;
            for i in 0..100 {
                let l = stellar::luminosity_from_radius_temp(r_m, t_k + i as f64);
                let rs = stellar::schwarzschild_radius(mass_s * SOLAR_MASS);
                v = v.wrapping_add(l.to_bits()).wrapping_add(rs.to_bits());
            }
            v
        });
        let result = format!(
            "{} ({}) | L={} | M_abs={} | R_s={} | MS lifetime={} | Wien \u{03bb}_max={}",
            s.name,
            s.spectral,
            lum_strs[idx],
            abs_mag_strs[idx],
            schwarzschild_strs[idx],
            lifetime_strs[idx],
            wien_strs[idx]
        );
        let rpt = generate(&m, &result);
        assert!(rpt.csv.starts_with("experiment_name"));
        metrics_store.push((s.name.to_string(), result, m));
    }

    let spectral_symbols: Vec<&str> = STARS
        .iter()
        .map(|s| match s.spectral.chars().next().unwrap() {
            'O' => "O",
            'B' => "B",
            'A' => "A",
            'F' => "F",
            'G' => "G",
            'K' => "K",
            'M' => "M",
            _ => "?",
        })
        .collect();

    let entries: Vec<Entry<'_>> = metrics_store
        .iter()
        .enumerate()
        .map(|(idx, (label, result, m))| Entry {
            metrics: m,
            result: result.as_str(),
            label: label.as_str(),
            tags: vec![
                ("category", STARS[idx].category),
                ("symbol", spectral_symbols[idx]),
                ("name", STARS[idx].name),
                ("spectral", STARS[idx].spectral),
                ("mass", mass_strs[idx].as_str()),
                ("radius", radius_strs[idx].as_str()),
                ("temperature", temp_strs[idx].as_str()),
                ("luminosity", lum_strs[idx].as_str()),
                ("abs_magnitude", abs_mag_strs[idx].as_str()),
                ("schwarzschild", schwarzschild_strs[idx].as_str()),
                ("ms_lifetime", lifetime_strs[idx].as_str()),
                ("wien_peak", wien_strs[idx].as_str()),
            ],
            grid_row: Some((idx as u8 / 5) + 1),
            grid_col: Some((idx as u8 % 5) + 1),
        })
        .collect();

    let summary = export(
        "Star Catalog \u{2014} Luminosity, Magnitude, Schwarzschild Radius & Main Sequence Lifetime",
        &entries,
        dir,
    ).unwrap();
    assert!(summary.files_written > 0);

    let bmk_path = dir.join("bmk/star_catalog.bmk");
    let bytes = fs::read(&bmk_path).unwrap();
    let view = decode(&bytes).unwrap();
    assert!(!view.experiment_name.is_empty());
}
