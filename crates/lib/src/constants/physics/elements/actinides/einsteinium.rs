use super::super::{DecayMode, Element, Isotope};

pub fn element() -> Element {
    Element {
        symbol: "Es",
        name: "Einsteinium",
        atomic_number: 99,
        atomic_mass: 252.08298_f64,
        electronegativity: Some(1.3_f64),
        group: None,
        period: 7,
        category: "actinide",
        electron_configuration: "[Rn] 5f¹¹ 7s²",
        isotopes: vec![
        Isotope {
            name: "Einsteinium-252",
            symbol: "²⁵²Es",
            mass_number: 252,
            neutrons: 153,
            atomic_mass: 252.08298_f64,
            half_life: Some(471.7_f64),
            half_life_unit: Some("days"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "alpha",
                branching_ratio: 0.78_f64,
                daughter: Some("Berkelium-248"),
            },
            DecayMode {
                mode: "electron_capture",
                branching_ratio: 0.22_f64,
                daughter: Some("Californium-252"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Einsteinium-253",
            symbol: "²⁵³Es",
            mass_number: 253,
            neutrons: 154,
            atomic_mass: 253.084825_f64,
            half_life: Some(20.47_f64),
            half_life_unit: Some("days"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "alpha",
                branching_ratio: 1.0_f64,
                daughter: Some("Berkelium-249"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Einsteinium-254",
            symbol: "²⁵⁴Es",
            mass_number: 254,
            neutrons: 155,
            atomic_mass: 254.088022_f64,
            half_life: Some(275.7_f64),
            half_life_unit: Some("days"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "alpha",
                branching_ratio: 1.0_f64,
                daughter: Some("Berkelium-250"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Einsteinium-255",
            symbol: "²⁵⁵Es",
            mass_number: 255,
            neutrons: 156,
            atomic_mass: 255.090275_f64,
            half_life: Some(39.8_f64),
            half_life_unit: Some("days"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "beta-",
                branching_ratio: 0.92_f64,
                daughter: Some("Fermium-255"),
            },
            DecayMode {
                mode: "alpha",
                branching_ratio: 0.08_f64,
                daughter: Some("Berkelium-251"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        ],
    }
}
