use sciforge_hub::benchmark::decode::decode;
use sciforge_hub::benchmark::engine::bench;
use sciforge_hub::benchmark::export::{Entry, export};
use sciforge_hub::benchmark::report::generate;
use sciforge_hub::prelude::meteorology::atmosphere;
use std::fs;
use std::path::Path;

struct Station {
    name: &'static str,
    category: &'static str,
    symbol: &'static str,
    altitude_m: f64,
    temp_c: f64,
    pressure_hpa: f64,
    rh_pct: f64,
}

const STATIONS: [Station; 10] = [
    Station {
        name: "Paris CDG",
        category: "Temperate",
        symbol: "P",
        altitude_m: 119.0,
        temp_c: 12.4,
        pressure_hpa: 1013.25,
        rh_pct: 78.0,
    },
    Station {
        name: "Tokyo Narita",
        category: "Temperate",
        symbol: "T",
        altitude_m: 44.0,
        temp_c: 16.3,
        pressure_hpa: 1015.0,
        rh_pct: 71.0,
    },
    Station {
        name: "Denver Intl",
        category: "Alpine",
        symbol: "D",
        altitude_m: 1655.0,
        temp_c: 10.1,
        pressure_hpa: 835.0,
        rh_pct: 45.0,
    },
    Station {
        name: "Mexico City",
        category: "Alpine",
        symbol: "M",
        altitude_m: 2250.0,
        temp_c: 16.0,
        pressure_hpa: 773.0,
        rh_pct: 52.0,
    },
    Station {
        name: "McMurdo Base",
        category: "Polar",
        symbol: "Mc",
        altitude_m: 10.0,
        temp_c: -18.0,
        pressure_hpa: 984.0,
        rh_pct: 60.0,
    },
    Station {
        name: "Nairobi",
        category: "Tropical",
        symbol: "N",
        altitude_m: 1795.0,
        temp_c: 19.0,
        pressure_hpa: 815.0,
        rh_pct: 65.0,
    },
    Station {
        name: "Lhasa",
        category: "Alpine",
        symbol: "L",
        altitude_m: 3650.0,
        temp_c: 8.5,
        pressure_hpa: 652.0,
        rh_pct: 35.0,
    },
    Station {
        name: "Death Valley",
        category: "Arid",
        symbol: "DV",
        altitude_m: -86.0,
        temp_c: 38.5,
        pressure_hpa: 1025.0,
        rh_pct: 12.0,
    },
    Station {
        name: "Reykjavik",
        category: "Polar",
        symbol: "R",
        altitude_m: 61.0,
        temp_c: 4.6,
        pressure_hpa: 1005.0,
        rh_pct: 82.0,
    },
    Station {
        name: "Singapore",
        category: "Tropical",
        symbol: "S",
        altitude_m: 16.0,
        temp_c: 27.5,
        pressure_hpa: 1010.0,
        rh_pct: 84.0,
    },
];

fn main() {
    let dir = Path::new("output/examples/weather_station");
    fs::create_dir_all(dir).unwrap();

    let sat_vp_strs: Vec<String> = STATIONS
        .iter()
        .map(|s| {
            let es = atmosphere::saturation_vapor_pressure(s.temp_c);
            format!("{:.2} hPa", es)
        })
        .collect();

    let dew_point_strs: Vec<String> = STATIONS
        .iter()
        .map(|s| {
            let dp = atmosphere::dew_point(s.temp_c, s.rh_pct);
            format!("{:.1} \u{00b0}C", dp)
        })
        .collect();

    let density_alt_strs: Vec<String> = STATIONS
        .iter()
        .map(|s| {
            let da = atmosphere::density_altitude(s.altitude_m, s.temp_c);
            format!("{:.0} m", da)
        })
        .collect();

    let pot_temp_strs: Vec<String> = STATIONS
        .iter()
        .map(|s| {
            let t_k = s.temp_c + 273.15;
            let p_pa = s.pressure_hpa * 100.0;
            let theta = atmosphere::potential_temperature(t_k, p_pa, 101325.0);
            format!("{:.1} K", theta)
        })
        .collect();

    let mixing_strs: Vec<String> = STATIONS
        .iter()
        .map(|s| {
            let es = atmosphere::saturation_vapor_pressure(s.temp_c);
            let e = es * s.rh_pct / 100.0;
            let p_pa = s.pressure_hpa * 100.0;
            let w = atmosphere::mixing_ratio(e * 100.0, p_pa);
            format!("{:.2} g/kg", w * 1000.0)
        })
        .collect();

    let lcl_strs: Vec<String> = STATIONS
        .iter()
        .map(|s| {
            let dp = atmosphere::dew_point(s.temp_c, s.rh_pct);
            let lcl = atmosphere::lifted_condensation_level(s.temp_c + 273.15, dp + 273.15);
            format!("{:.0} m AGL", lcl)
        })
        .collect();

    let alt_strs: Vec<String> = STATIONS
        .iter()
        .map(|s| format!("{:.0} m", s.altitude_m))
        .collect();
    let temp_strs: Vec<String> = STATIONS
        .iter()
        .map(|s| format!("{:.1} \u{00b0}C", s.temp_c))
        .collect();
    let rh_strs: Vec<String> = STATIONS
        .iter()
        .map(|s| format!("{:.0}%", s.rh_pct))
        .collect();
    let pres_strs: Vec<String> = STATIONS
        .iter()
        .map(|s| format!("{:.1} hPa", s.pressure_hpa))
        .collect();

    let mut metrics_store: Vec<(String, String, _)> = Vec::new();

    let exp_names: Vec<String> = STATIONS
        .iter()
        .map(|s| format!("{}_weather", s.name))
        .collect();

    for (idx, s) in STATIONS.iter().enumerate() {
        let tc = s.temp_c;
        let rh = s.rh_pct;
        let alt = s.altitude_m;
        let m = bench(&exp_names[idx], "f64", 5000, move || {
            let mut v = 0u64;
            for i in 0..100 {
                let t = tc + (i as f64) * 0.1;
                let es = atmosphere::saturation_vapor_pressure(t);
                let dp = atmosphere::dew_point(t, rh);
                let da = atmosphere::density_altitude(alt, t);
                v = v
                    .wrapping_add(es.to_bits())
                    .wrapping_add(dp.to_bits())
                    .wrapping_add(da.to_bits());
            }
            v
        });
        let result = format!(
            "{} | {}m | {} | RH={} | Td={} | DA={} | \u{03b8}={}",
            s.name,
            s.altitude_m as i32,
            temp_strs[idx],
            rh_strs[idx],
            dew_point_strs[idx],
            density_alt_strs[idx],
            pot_temp_strs[idx]
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
                ("category", STATIONS[idx].category),
                ("symbol", STATIONS[idx].symbol),
                ("name", STATIONS[idx].name),
                ("altitude", alt_strs[idx].as_str()),
                ("temperature", temp_strs[idx].as_str()),
                ("pressure", pres_strs[idx].as_str()),
                ("relative_humidity", rh_strs[idx].as_str()),
                ("saturation_vp", sat_vp_strs[idx].as_str()),
                ("dew_point", dew_point_strs[idx].as_str()),
                ("density_altitude", density_alt_strs[idx].as_str()),
                ("potential_temp", pot_temp_strs[idx].as_str()),
                ("mixing_ratio", mixing_strs[idx].as_str()),
                ("lcl", lcl_strs[idx].as_str()),
            ],
            grid_row: Some((idx as u8 / 5) + 1),
            grid_col: Some((idx as u8 % 5) + 1),
        })
        .collect();

    let summary = export(
        "Weather Station \u{2014} Atmospheric Pressure, Humidity & Dew Point",
        &entries,
        dir,
    )
    .unwrap();
    assert!(summary.files_written > 0);

    let bmk_path = dir.join("bmk/weather_station.bmk");
    let bytes = fs::read(&bmk_path).unwrap();
    let view = decode(&bytes).unwrap();
    assert!(!view.experiment_name.is_empty());
}
