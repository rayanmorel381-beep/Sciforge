use super::super::{DecayMode, Element, Isotope};

pub fn element() -> Element {
    Element {
        symbol: "Hs",
        name: "Hassium",
        atomic_number: 108,
        atomic_mass: 269.134_f64,
        electronegativity: None,
        group: Some(8),
        period: 7,
        category: "transition_metal",
        electron_configuration: "[Rn] 5f¹⁴ 6d⁶ 7s²",
        isotopes: vec![
        Isotope {
            name: "Hassium-269",
            symbol: "²⁶⁹Hs",
            mass_number: 269,
            neutrons: 161,
            atomic_mass: 269.134_f64,
            half_life: Some(16.0_f64),
            half_life_unit: Some("seconds"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "alpha",
                branching_ratio: 1.0_f64,
                daughter: Some("Seaborgium-265"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Hassium-270",
            symbol: "²⁷⁰Hs",
            mass_number: 270,
            neutrons: 162,
            atomic_mass: 270.135_f64,
            half_life: Some(10.0_f64),
            half_life_unit: Some("seconds"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "alpha",
                branching_ratio: 1.0_f64,
                daughter: Some("Seaborgium-266"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Hassium-277",
            symbol: "²⁷⁷Hs",
            mass_number: 277,
            neutrons: 169,
            atomic_mass: 277.15_f64,
            half_life: Some(11.0_f64),
            half_life_unit: Some("milliseconds"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "spontaneous_fission",
                branching_ratio: 1.0_f64,
                daughter: None,
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        ],
    }
}
