use super::super::{DecayMode, Element, Isotope};

pub fn element() -> Element {
    Element {
        symbol: "Md",
        name: "Mendelevium",
        atomic_number: 101,
        atomic_mass: 258.098431_f64,
        electronegativity: Some(1.3_f64),
        group: None,
        period: 7,
        category: "actinide",
        electron_configuration: "[Rn] 5f¹³ 7s²",
        isotopes: vec![
        Isotope {
            name: "Mendelevium-257",
            symbol: "²⁵⁷Md",
            mass_number: 257,
            neutrons: 156,
            atomic_mass: 257.095541_f64,
            half_life: Some(5.52_f64),
            half_life_unit: Some("hours"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "electron_capture",
                branching_ratio: 0.85_f64,
                daughter: Some("Fermium-257"),
            },
            DecayMode {
                mode: "alpha",
                branching_ratio: 0.15_f64,
                daughter: Some("Einsteinium-253"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Mendelevium-258",
            symbol: "²⁵⁸Md",
            mass_number: 258,
            neutrons: 157,
            atomic_mass: 258.098431_f64,
            half_life: Some(51.5_f64),
            half_life_unit: Some("days"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "alpha",
                branching_ratio: 1.0_f64,
                daughter: Some("Einsteinium-254"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Mendelevium-260",
            symbol: "²⁶⁰Md",
            mass_number: 260,
            neutrons: 159,
            atomic_mass: 260.10365_f64,
            half_life: Some(31.8_f64),
            half_life_unit: Some("days"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "spontaneous_fission",
                branching_ratio: 0.5_f64,
                daughter: None,
            },
            DecayMode {
                mode: "alpha",
                branching_ratio: 0.25_f64,
                daughter: Some("Einsteinium-256"),
            },
            DecayMode {
                mode: "electron_capture",
                branching_ratio: 0.15_f64,
                daughter: Some("Fermium-260"),
            },
            DecayMode {
                mode: "beta-",
                branching_ratio: 0.1_f64,
                daughter: Some("Nobelium-260"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        ],
    }
}
