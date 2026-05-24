use super::super::{DecayMode, Element, Isotope};

pub fn element() -> Element {
    Element {
        symbol: "Fl",
        name: "Flerovium",
        atomic_number: 114,
        atomic_mass: 289.19_f64,
        electronegativity: None,
        group: Some(14),
        period: 7,
        category: "unknown",
        electron_configuration: "[Rn] 5f¹⁴ 6d¹⁰ 7s² 7p²",
        isotopes: vec![
        Isotope {
            name: "Flerovium-286",
            symbol: "²⁸⁶Fl",
            mass_number: 286,
            neutrons: 172,
            atomic_mass: 286.184_f64,
            half_life: Some(0.12_f64),
            half_life_unit: Some("seconds"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "spontaneous_fission",
                branching_ratio: 0.6_f64,
                daughter: None,
            },
            DecayMode {
                mode: "alpha",
                branching_ratio: 0.4_f64,
                daughter: Some("Copernicium-282"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Flerovium-287",
            symbol: "²⁸⁷Fl",
            mass_number: 287,
            neutrons: 173,
            atomic_mass: 287.187_f64,
            half_life: Some(0.48_f64),
            half_life_unit: Some("seconds"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "alpha",
                branching_ratio: 1.0_f64,
                daughter: Some("Copernicium-283"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Flerovium-288",
            symbol: "²⁸⁸Fl",
            mass_number: 288,
            neutrons: 174,
            atomic_mass: 288.188_f64,
            half_life: Some(0.66_f64),
            half_life_unit: Some("seconds"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "alpha",
                branching_ratio: 1.0_f64,
                daughter: Some("Copernicium-284"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Flerovium-289",
            symbol: "²⁸⁹Fl",
            mass_number: 289,
            neutrons: 175,
            atomic_mass: 289.19_f64,
            half_life: Some(1.9_f64),
            half_life_unit: Some("seconds"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "alpha",
                branching_ratio: 1.0_f64,
                daughter: Some("Copernicium-285"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        ],
    }
}
