use sciforge_hub::benchmark::decode::decode;
use sciforge_hub::benchmark::engine::bench;
use sciforge_hub::benchmark::export::{Entry, export};
use sciforge_hub::benchmark::report::generate;
use sciforge_hub::prelude::*;
use std::fs;
use std::path::Path;

struct Conversion {
    name: &'static str,
    category: &'static str,
    value: f64,
    from: &'static str,
    to: &'static str,
}

fn main() {
    let dir = Path::new("output/examples/unit_converter");
    fs::create_dir_all(dir).unwrap();

    let conversions: Vec<Conversion> = vec![
        Conversion {
            name: "Marathon distance",
            category: "Length",
            value: 42.195,
            from: "km",
            to: "mi",
        },
        Conversion {
            name: "Everest height",
            category: "Length",
            value: 8848.86,
            from: "m",
            to: "ft",
        },
        Conversion {
            name: "Light-year",
            category: "Length",
            value: 1.0,
            from: "ly",
            to: "km",
        },
        Conversion {
            name: "Human body temp",
            category: "Temperature",
            value: 37.0,
            from: "\u{00b0}C",
            to: "K",
        },
        Conversion {
            name: "Liquid nitrogen",
            category: "Temperature",
            value: 77.36,
            from: "K",
            to: "\u{00b0}C",
        },
        Conversion {
            name: "Sun surface",
            category: "Temperature",
            value: 5778.0,
            from: "K",
            to: "\u{00b0}F",
        },
        Conversion {
            name: "Atmospheric",
            category: "Pressure",
            value: 101325.0,
            from: "Pa",
            to: "atm",
        },
        Conversion {
            name: "Deep ocean",
            category: "Pressure",
            value: 1100.0,
            from: "atm",
            to: "Pa",
        },
        Conversion {
            name: "Earth mass",
            category: "Mass",
            value: 5.972e24,
            from: "kg",
            to: "lb",
        },
        Conversion {
            name: "Proton mass",
            category: "Mass",
            value: 1.672e-27,
            from: "kg",
            to: "amu",
        },
        Conversion {
            name: "1 eV",
            category: "Energy",
            value: 1.0,
            from: "eV",
            to: "J",
        },
        Conversion {
            name: "TNT equivalent",
            category: "Energy",
            value: 4.184e9,
            from: "J",
            to: "cal",
        },
        Conversion {
            name: "1 day",
            category: "Time",
            value: 1.0,
            from: "day",
            to: "s",
        },
        Conversion {
            name: "1 year",
            category: "Time",
            value: 1.0,
            from: "year",
            to: "h",
        },
        Conversion {
            name: "Right angle",
            category: "Angle",
            value: 90.0,
            from: "deg",
            to: "rad",
        },
    ];

    let result_strs: Vec<String> = conversions
        .iter()
        .map(|c| {
            let converted = match c.category {
                "Length" => {
                    let (from_u, to_u) = match (c.from, c.to) {
                        ("km", "mi") => (LengthUnit::Kilometer, LengthUnit::Mile),
                        ("m", "ft") => (LengthUnit::Meter, LengthUnit::Foot),
                        ("ly", "km") => (LengthUnit::LightYear, LengthUnit::Kilometer),
                        _ => (LengthUnit::Meter, LengthUnit::Meter),
                    };
                    let si = length_to_si(c.value, from_u);
                    length_from_si(si, to_u)
                }
                "Temperature" => {
                    let (from_u, to_u) = match (c.from, c.to) {
                        ("\u{00b0}C", "K") => (TemperatureUnit::Celsius, TemperatureUnit::Kelvin),
                        ("K", "\u{00b0}C") => (TemperatureUnit::Kelvin, TemperatureUnit::Celsius),
                        ("K", "\u{00b0}F") => {
                            (TemperatureUnit::Kelvin, TemperatureUnit::Fahrenheit)
                        }
                        _ => (TemperatureUnit::Kelvin, TemperatureUnit::Kelvin),
                    };
                    let k = temperature_to_kelvin(c.value, from_u);
                    kelvin_to_temperature(k, to_u)
                }
                "Pressure" => {
                    let (from_u, to_u) = match (c.from, c.to) {
                        ("Pa", "atm") => (PressureUnit::Pascal, PressureUnit::Atmosphere),
                        ("atm", "Pa") => (PressureUnit::Atmosphere, PressureUnit::Pascal),
                        _ => (PressureUnit::Pascal, PressureUnit::Pascal),
                    };
                    let si = pressure_to_si(c.value, from_u);
                    pressure_from_si(si, to_u)
                }
                "Mass" => {
                    let (from_u, to_u) = match (c.from, c.to) {
                        ("kg", "lb") => (MassUnit::Kilogram, MassUnit::Pound),
                        ("kg", "amu") => (MassUnit::Kilogram, MassUnit::Dalton),
                        _ => (MassUnit::Kilogram, MassUnit::Kilogram),
                    };
                    let si = mass_to_si(c.value, from_u);
                    mass_from_si(si, to_u)
                }
                "Energy" => {
                    let (from_u, to_u) = match (c.from, c.to) {
                        ("eV", "J") => (EnergyUnit::ElectronVolt, EnergyUnit::Joule),
                        ("J", "cal") => (EnergyUnit::Joule, EnergyUnit::Calorie),
                        _ => (EnergyUnit::Joule, EnergyUnit::Joule),
                    };
                    let si = energy_to_si(c.value, from_u);
                    energy_from_si(si, to_u)
                }
                "Time" => {
                    let (from_u, to_u) = match (c.from, c.to) {
                        ("day", "s") => (TimeUnit::Day, TimeUnit::Second),
                        ("year", "h") => (TimeUnit::Year, TimeUnit::Hour),
                        _ => (TimeUnit::Second, TimeUnit::Second),
                    };
                    let si = time_to_si(c.value, from_u);
                    time_from_si(si, to_u)
                }
                "Angle" => {
                    let from_u = match c.from {
                        "deg" => AngleUnit::Degree,
                        _ => AngleUnit::Radian,
                    };
                    let to_u = match c.to {
                        "rad" => AngleUnit::Radian,
                        _ => AngleUnit::Degree,
                    };
                    let rad = angle_to_radian(c.value, from_u);
                    radian_to_angle(rad, to_u)
                }
                _ => c.value,
            };
            format!("{:.6e}", converted)
        })
        .collect();

    let display_strs: Vec<String> = conversions
        .iter()
        .zip(result_strs.iter())
        .map(|(c, r)| format!("{} {} \u{2192} {} {}", c.value, c.from, r, c.to))
        .collect();

    let cat_strs: Vec<String> = conversions.iter().map(|c| c.category.to_string()).collect();
    let from_strs: Vec<String> = conversions
        .iter()
        .map(|c| format!("{} {}", c.value, c.from))
        .collect();
    let to_strs: Vec<String> = conversions
        .iter()
        .zip(result_strs.iter())
        .map(|(c, r)| format!("{} {}", r, c.to))
        .collect();

    let mut metrics_store: Vec<(String, String, _)> = Vec::new();

    let exp_names: Vec<String> = conversions
        .iter()
        .map(|c| format!("{}_convert", c.name))
        .collect();

    for (idx, c) in conversions.iter().enumerate() {
        let val = c.value;
        let cat = c.category;
        let m = bench(&exp_names[idx], "f64", 10000, move || {
            let mut v = 0u64;
            for i in 0..100 {
                let x = val + (i as f64) * 0.001;
                let r = match cat {
                    "Length" => {
                        length_from_si(length_to_si(x, LengthUnit::Meter), LengthUnit::Foot)
                    }
                    "Temperature" => kelvin_to_temperature(
                        temperature_to_kelvin(x, TemperatureUnit::Celsius),
                        TemperatureUnit::Fahrenheit,
                    ),
                    "Pressure" => pressure_from_si(
                        pressure_to_si(x, PressureUnit::Pascal),
                        PressureUnit::Atmosphere,
                    ),
                    "Mass" => mass_from_si(mass_to_si(x, MassUnit::Kilogram), MassUnit::Pound),
                    "Energy" => {
                        energy_from_si(energy_to_si(x, EnergyUnit::Joule), EnergyUnit::Calorie)
                    }
                    "Time" => time_from_si(time_to_si(x, TimeUnit::Day), TimeUnit::Hour),
                    "Angle" => {
                        radian_to_angle(angle_to_radian(x, AngleUnit::Degree), AngleUnit::Radian)
                    }
                    _ => x,
                };
                v = v.wrapping_add(r.to_bits());
            }
            v
        });
        let result = format!("{} | {}", c.name, display_strs[idx]);
        let rpt = generate(&m, &result);
        assert!(rpt.csv.starts_with("experiment_name"));
        metrics_store.push((c.name.to_string(), result, m));
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
                ("symbol", conversions[idx].from),
                ("name", conversions[idx].name),
                ("from", from_strs[idx].as_str()),
                ("to", to_strs[idx].as_str()),
                ("conversion", display_strs[idx].as_str()),
            ],
            grid_row: Some((idx as u8 / 5) + 1),
            grid_col: Some((idx as u8 % 5) + 1),
        })
        .collect();

    let summary = export(
        "Unit Converter \u{2014} SI, CGS & Common Unit Conversions",
        &entries,
        dir,
    )
    .unwrap();
    assert!(summary.files_written > 0);

    let bmk_path = dir.join("bmk/unit_converter.bmk");
    let bytes = fs::read(&bmk_path).unwrap();
    let view = decode(&bytes).unwrap();
    assert!(!view.experiment_name.is_empty());
}
