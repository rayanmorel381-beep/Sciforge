use super::super::{DecayMode, Element, Isotope};

pub fn element() -> Element {
    Element {
        symbol: "No",
        name: "Nobelium",
        atomic_number: 102,
        atomic_mass: 259.10103_f64,
        electronegativity: Some(1.3_f64),
        group: None,
        period: 7,
        category: "actinide",
        electron_configuration: "[Rn] 5f¹⁴ 7s²",
        isotopes: vec![
        Isotope {
            name: "Nobelium-253",
            symbol: "²⁵³No",
            mass_number: 253,
            neutrons: 151,
            atomic_mass: 253.09068_f64,
            half_life: Some(1.62_f64),
            half_life_unit: Some("minutes"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "alpha",
                branching_ratio: 0.8_f64,
                daughter: Some("Fermium-249"),
            },
            DecayMode {
                mode: "electron_capture",
                branching_ratio: 0.2_f64,
                daughter: Some("Mendelevium-253"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Nobelium-255",
            symbol: "²⁵⁵No",
            mass_number: 255,
            neutrons: 153,
            atomic_mass: 255.09324_f64,
            half_life: Some(3.52_f64),
            half_life_unit: Some("minutes"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "alpha",
                branching_ratio: 0.61_f64,
                daughter: Some("Fermium-251"),
            },
            DecayMode {
                mode: "electron_capture",
                branching_ratio: 0.39_f64,
                daughter: Some("Mendelevium-255"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Nobelium-259",
            symbol: "²⁵⁹No",
            mass_number: 259,
            neutrons: 157,
            atomic_mass: 259.10103_f64,
            half_life: Some(58.0_f64),
            half_life_unit: Some("minutes"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "alpha",
                branching_ratio: 0.75_f64,
                daughter: Some("Fermium-255"),
            },
            DecayMode {
                mode: "electron_capture",
                branching_ratio: 0.25_f64,
                daughter: Some("Mendelevium-259"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        ],
    }
}
