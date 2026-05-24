use super::super::{DecayMode, Element, Isotope};

pub fn element() -> Element {
    Element {
        symbol: "Rg",
        name: "Roentgenium",
        atomic_number: 111,
        atomic_mass: 282.169_f64,
        electronegativity: None,
        group: Some(11),
        period: 7,
        category: "unknown",
        electron_configuration: "[Rn] 5f¹⁴ 6d⁹ 7s²",
        isotopes: vec![
        Isotope {
            name: "Roentgenium-279",
            symbol: "²⁷⁹Rg",
            mass_number: 279,
            neutrons: 168,
            atomic_mass: 279.163_f64,
            half_life: Some(0.17_f64),
            half_life_unit: Some("seconds"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "alpha",
                branching_ratio: 1.0_f64,
                daughter: Some("Meitnerium-275"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Roentgenium-280",
            symbol: "²⁸⁰Rg",
            mass_number: 280,
            neutrons: 169,
            atomic_mass: 280.165_f64,
            half_life: Some(3.6_f64),
            half_life_unit: Some("seconds"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "alpha",
                branching_ratio: 1.0_f64,
                daughter: Some("Meitnerium-276"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Roentgenium-282",
            symbol: "²⁸²Rg",
            mass_number: 282,
            neutrons: 171,
            atomic_mass: 282.169_f64,
            half_life: Some(0.73_f64),
            half_life_unit: Some("minutes"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "alpha",
                branching_ratio: 1.0_f64,
                daughter: Some("Meitnerium-278"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        ],
    }
}
