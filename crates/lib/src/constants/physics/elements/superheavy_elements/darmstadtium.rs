use super::super::{DecayMode, Element, Isotope};

pub fn element() -> Element {
    Element {
        symbol: "Ds",
        name: "Darmstadtium",
        atomic_number: 110,
        atomic_mass: 281.165_f64,
        electronegativity: None,
        group: Some(10),
        period: 7,
        category: "unknown",
        electron_configuration: "[Rn] 5f¹⁴ 6d⁸ 7s²",
        isotopes: vec![
        Isotope {
            name: "Darmstadtium-279",
            symbol: "²⁷⁹Ds",
            mass_number: 279,
            neutrons: 169,
            atomic_mass: 279.16_f64,
            half_life: Some(0.18_f64),
            half_life_unit: Some("seconds"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "alpha",
                branching_ratio: 0.9_f64,
                daughter: Some("Hassium-275"),
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
            name: "Darmstadtium-281",
            symbol: "²⁸¹Ds",
            mass_number: 281,
            neutrons: 171,
            atomic_mass: 281.165_f64,
            half_life: Some(12.7_f64),
            half_life_unit: Some("seconds"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "spontaneous_fission",
                branching_ratio: 0.94_f64,
                daughter: None,
            },
            DecayMode {
                mode: "alpha",
                branching_ratio: 0.06_f64,
                daughter: Some("Hassium-277"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Darmstadtium-282",
            symbol: "²⁸²Ds",
            mass_number: 282,
            neutrons: 172,
            atomic_mass: 282.169_f64,
            half_life: Some(0.067_f64),
            half_life_unit: Some("seconds"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "alpha",
                branching_ratio: 1.0_f64,
                daughter: Some("Hassium-278"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        ],
    }
}
