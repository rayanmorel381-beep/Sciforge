use super::super::{DecayMode, Element, Isotope};

pub fn element() -> Element {
    Element {
        symbol: "Fm",
        name: "Fermium",
        atomic_number: 100,
        atomic_mass: 257.095105_f64,
        electronegativity: Some(1.3_f64),
        group: None,
        period: 7,
        category: "actinide",
        electron_configuration: "[Rn] 5f¹² 7s²",
        isotopes: vec![
        Isotope {
            name: "Fermium-252",
            symbol: "²⁵²Fm",
            mass_number: 252,
            neutrons: 152,
            atomic_mass: 252.082467_f64,
            half_life: Some(25.39_f64),
            half_life_unit: Some("hours"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "alpha",
                branching_ratio: 1.0_f64,
                daughter: Some("Californium-248"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Fermium-255",
            symbol: "²⁵⁵Fm",
            mass_number: 255,
            neutrons: 155,
            atomic_mass: 255.089964_f64,
            half_life: Some(20.07_f64),
            half_life_unit: Some("hours"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "alpha",
                branching_ratio: 1.0_f64,
                daughter: Some("Californium-251"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Fermium-257",
            symbol: "²⁵⁷Fm",
            mass_number: 257,
            neutrons: 157,
            atomic_mass: 257.095105_f64,
            half_life: Some(100.5_f64),
            half_life_unit: Some("days"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "alpha",
                branching_ratio: 0.9979_f64,
                daughter: Some("Californium-253"),
            },
            DecayMode {
                mode: "spontaneous_fission",
                branching_ratio: 0.0021_f64,
                daughter: None,
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Fermium-258",
            symbol: "²⁵⁸Fm",
            mass_number: 258,
            neutrons: 158,
            atomic_mass: 258.09708_f64,
            half_life: Some(370.0_f64),
            half_life_unit: Some("microseconds"),
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
