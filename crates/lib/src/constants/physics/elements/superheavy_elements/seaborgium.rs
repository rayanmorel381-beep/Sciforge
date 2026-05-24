use super::super::{DecayMode, Element, Isotope};

pub fn element() -> Element {
    Element {
        symbol: "Sg",
        name: "Seaborgium",
        atomic_number: 106,
        atomic_mass: 269.129_f64,
        electronegativity: None,
        group: Some(6),
        period: 7,
        category: "transition_metal",
        electron_configuration: "[Rn] 5f¹⁴ 6d⁴ 7s²",
        isotopes: vec![
        Isotope {
            name: "Seaborgium-265",
            symbol: "²⁶⁵Sg",
            mass_number: 265,
            neutrons: 159,
            atomic_mass: 265.12109_f64,
            half_life: Some(8.9_f64),
            half_life_unit: Some("seconds"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "alpha",
                branching_ratio: 1.0_f64,
                daughter: Some("Rutherfordium-261"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Seaborgium-269",
            symbol: "²⁶⁹Sg",
            mass_number: 269,
            neutrons: 163,
            atomic_mass: 269.129_f64,
            half_life: Some(14.0_f64),
            half_life_unit: Some("minutes"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "alpha",
                branching_ratio: 1.0_f64,
                daughter: Some("Rutherfordium-265"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Seaborgium-271",
            symbol: "²⁷¹Sg",
            mass_number: 271,
            neutrons: 165,
            atomic_mass: 271.134_f64,
            half_life: Some(1.9_f64),
            half_life_unit: Some("minutes"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "alpha",
                branching_ratio: 0.67_f64,
                daughter: Some("Rutherfordium-267"),
            },
            DecayMode {
                mode: "spontaneous_fission",
                branching_ratio: 0.33_f64,
                daughter: None,
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        ],
    }
}
