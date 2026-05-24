use sciforge_hub::benchmark::decode::decode;
use sciforge_hub::benchmark::engine::bench;
use sciforge_hub::benchmark::export::{Entry, export};
use sciforge_hub::benchmark::report::generate;
use sciforge_hub::prelude::astronomy::orbits;
use sciforge_hub::prelude::constants::*;
use std::fs;
use std::path::Path;

struct Planet {
    name: &'static str,
    symbol: &'static str,
    mass: f64,
    radius: f64,
    semi_major: f64,
    category: &'static str,
}

const PLANETS: [Planet; 8] = [
    Planet {
        name: "Mercury",
        symbol: "\u{263F}",
        mass: MERCURY_MASS,
        radius: MERCURY_RADIUS,
        semi_major: MERCURY_SEMI_MAJOR_AXIS,
        category: "Terrestrial",
    },
    Planet {
        name: "Venus",
        symbol: "\u{2640}",
        mass: VENUS_MASS,
        radius: VENUS_RADIUS,
        semi_major: VENUS_SEMI_MAJOR_AXIS,
        category: "Terrestrial",
    },
    Planet {
        name: "Earth",
        symbol: "\u{2641}",
        mass: EARTH_MASS,
        radius: EARTH_RADIUS,
        semi_major: EARTH_SEMI_MAJOR_AXIS,
        category: "Terrestrial",
    },
    Planet {
        name: "Mars",
        symbol: "\u{2642}",
        mass: MARS_MASS,
        radius: MARS_RADIUS,
        semi_major: MARS_SEMI_MAJOR_AXIS,
        category: "Terrestrial",
    },
    Planet {
        name: "Jupiter",
        symbol: "\u{2643}",
        mass: JUPITER_MASS,
        radius: JUPITER_RADIUS,
        semi_major: JUPITER_SEMI_MAJOR_AXIS,
        category: "Gas Giant",
    },
    Planet {
        name: "Saturn",
        symbol: "\u{2644}",
        mass: SATURN_MASS,
        radius: SATURN_RADIUS,
        semi_major: SATURN_SEMI_MAJOR_AXIS,
        category: "Gas Giant",
    },
    Planet {
        name: "Uranus",
        symbol: "\u{26E2}",
        mass: URANUS_MASS,
        radius: URANUS_RADIUS,
        semi_major: URANUS_SEMI_MAJOR_AXIS,
        category: "Ice Giant",
    },
    Planet {
        name: "Neptune",
        symbol: "\u{2646}",
        mass: NEPTUNE_MASS,
        radius: NEPTUNE_RADIUS,
        semi_major: NEPTUNE_SEMI_MAJOR_AXIS,
        category: "Ice Giant",
    },
];

fn main() {
    let dir = Path::new("output/examples/solar_system");
    fs::create_dir_all(dir).unwrap();

    let mu_sun = G * SOLAR_MASS;

    let kepler_strs: Vec<String> = PLANETS
        .iter()
        .map(|p| {
            let t = orbits::kepler_period(p.semi_major, mu_sun);
            format!("{:.2} days", t / 86400.0)
        })
        .collect();

    let circ_vel_strs: Vec<String> = PLANETS
        .iter()
        .map(|p| {
            let v = orbits::circular_velocity(mu_sun, p.semi_major);
            format!("{:.2} km/s", v / 1000.0)
        })
        .collect();

    let esc_vel_strs: Vec<String> = PLANETS
        .iter()
        .map(|p| {
            let v = orbits::escape_velocity(G * p.mass, p.radius);
            format!("{:.2} km/s", v / 1000.0)
        })
        .collect();

    let hohmann_strs: Vec<String> = PLANETS
        .iter()
        .map(|p| {
            if (p.semi_major - EARTH_SEMI_MAJOR_AXIS).abs() < 1.0 {
                "— (origin)".to_string()
            } else {
                let dv = orbits::hohmann_delta_v(mu_sun, EARTH_SEMI_MAJOR_AXIS, p.semi_major);
                format!("{:.2} km/s", dv / 1000.0)
            }
        })
        .collect();

    let mass_strs: Vec<String> = PLANETS
        .iter()
        .map(|p| format!("{:.3e} kg", p.mass))
        .collect();
    let radius_strs: Vec<String> = PLANETS
        .iter()
        .map(|p| format!("{:.0} km", p.radius / 1000.0))
        .collect();
    let sma_strs: Vec<String> = PLANETS
        .iter()
        .map(|p| format!("{:.3} AU", p.semi_major / AU))
        .collect();

    let mut metrics_store: Vec<(String, String, _)> = Vec::new();

    let exp_names: Vec<String> = PLANETS
        .iter()
        .map(|p| format!("{}_orbit", p.name))
        .collect();

    for (idx, p) in PLANETS.iter().enumerate() {
        let semi = p.semi_major;
        let m = bench(&exp_names[idx], "f64", 5000, move || {
            let mut v = 0u64;
            for i in 0..100 {
                let t = orbits::kepler_period(semi + i as f64, mu_sun);
                v = v.wrapping_add(t.to_bits());
            }
            v
        });
        let result = format!(
            "{} | T={} | v_circ={} | v_esc={} | Hohmann \u{0394}v={}",
            p.name, kepler_strs[idx], circ_vel_strs[idx], esc_vel_strs[idx], hohmann_strs[idx]
        );
        let r = generate(&m, &result);
        assert!(r.csv.starts_with("experiment_name"));
        metrics_store.push((p.name.to_string(), result, m));
    }

    let cat_strs: Vec<String> = PLANETS.iter().map(|p| p.category.to_string()).collect();

    let entries: Vec<Entry<'_>> = metrics_store
        .iter()
        .enumerate()
        .map(|(idx, (label, result, m))| Entry {
            metrics: m,
            result: result.as_str(),
            label: label.as_str(),
            tags: vec![
                ("category", cat_strs[idx].as_str()),
                ("symbol", PLANETS[idx].symbol),
                ("name", PLANETS[idx].name),
                ("mass", mass_strs[idx].as_str()),
                ("radius", radius_strs[idx].as_str()),
                ("semi_major", sma_strs[idx].as_str()),
                ("kepler_period", kepler_strs[idx].as_str()),
                ("circular_vel", circ_vel_strs[idx].as_str()),
                ("escape_vel", esc_vel_strs[idx].as_str()),
                ("hohmann_dv", hohmann_strs[idx].as_str()),
            ],
            grid_row: Some(1),
            grid_col: Some((idx + 1) as u8),
        })
        .collect();

    let summary = export(
        "Solar System \u{2014} Orbital Mechanics (Kepler, Hohmann, Escape Velocities)",
        &entries,
        dir,
    )
    .unwrap();
    assert!(summary.files_written > 0);

    let bmk_dir = dir.join("bmk");
    let last_bmk = fs::read(bmk_dir.join("solar_system.bmk")).unwrap();
    let view = decode(&last_bmk).unwrap();
    assert!(!view.experiment_name.is_empty());
}
