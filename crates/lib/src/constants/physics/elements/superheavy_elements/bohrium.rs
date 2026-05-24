use super::super::{DecayMode, Element, Isotope};

pub fn element() -> Element {
    Element {
        symbol: "Bh",
        name: "Bohrium",
        atomic_number: 107,
        atomic_mass: 270.134_f64,
        electronegativity: None,
        group: Some(7),
        period: 7,
        category: "transition_metal",
        electron_configuration: "[Rn] 5f¹⁴ 6d⁵ 7s²",
        isotopes: vec![
        Isotope {
            name: "Bohrium-267",
            symbol: "²⁶⁷Bh",
            mass_number: 267,
            neutrons: 160,
            atomic_mass: 267.1275_f64,
            half_life: Some(17.0_f64),
            half_life_unit: Some("seconds"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "alpha",
                branching_ratio: 1.0_f64,
                daughter: Some("Dubnium-263"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Bohrium-270",
            symbol: "²⁷⁰Bh",
            mass_number: 270,
            neutrons: 163,
            atomic_mass: 270.134_f64,
            half_life: Some(61.0_f64),
            half_life_unit: Some("seconds"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "alpha",
                branching_ratio: 1.0_f64,
                daughter: Some("Dubnium-266"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Bohrium-274",
            symbol: "²⁷⁴Bh",
            mass_number: 274,
            neutrons: 167,
            atomic_mass: 274.144_f64,
            half_life: Some(54.0_f64),
            half_life_unit: Some("seconds"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "alpha",
                branching_ratio: 1.0_f64,
                daughter: Some("Dubnium-270"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        ],
    }
}
