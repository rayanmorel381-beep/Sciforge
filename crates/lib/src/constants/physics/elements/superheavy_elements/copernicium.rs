use super::super::{DecayMode, Element, Isotope};

pub fn element() -> Element {
    Element {
        symbol: "Cn",
        name: "Copernicium",
        atomic_number: 112,
        atomic_mass: 285.177_f64,
        electronegativity: None,
        group: Some(12),
        period: 7,
        category: "unknown",
        electron_configuration: "[Rn] 5f¹⁴ 6d¹⁰ 7s²",
        isotopes: vec![
        Isotope {
            name: "Copernicium-283",
            symbol: "²⁸³Cn",
            mass_number: 283,
            neutrons: 171,
            atomic_mass: 283.173_f64,
            half_life: Some(4.0_f64),
            half_life_unit: Some("seconds"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "alpha",
                branching_ratio: 0.9_f64,
                daughter: Some("Darmstadtium-279"),
            },
            DecayMode {
                mode: "spontaneous_fission",
                branching_ratio: 0.1_f64,
                daughter: None,
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Copernicium-285",
            symbol: "²⁸⁵Cn",
            mass_number: 285,
            neutrons: 173,
            atomic_mass: 285.177_f64,
            half_life: Some(29.0_f64),
            half_life_unit: Some("seconds"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "alpha",
                branching_ratio: 1.0_f64,
                daughter: Some("Darmstadtium-281"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        ],
    }
}
