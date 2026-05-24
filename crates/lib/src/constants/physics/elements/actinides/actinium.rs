use super::super::{DecayMode, Element, Isotope};

pub fn element() -> Element {
    Element {
        symbol: "Ac",
        name: "Actinium",
        atomic_number: 89,
        atomic_mass: 227.027752_f64,
        electronegativity: Some(1.1_f64),
        group: None,
        period: 7,
        category: "actinide",
        electron_configuration: "[Rn] 6d¹ 7s²",
        isotopes: vec![
        Isotope {
            name: "Actinium-225",
            symbol: "²²⁵Ac",
            mass_number: 225,
            neutrons: 136,
            atomic_mass: 225.02323_f64,
            half_life: Some(9.92_f64),
            half_life_unit: Some("days"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "alpha",
                branching_ratio: 1.0_f64,
                daughter: Some("Francium-221"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Actinium-227",
            symbol: "²²⁷Ac",
            mass_number: 227,
            neutrons: 138,
            atomic_mass: 227.027752_f64,
            half_life: Some(21.772_f64),
            half_life_unit: Some("years"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "beta-",
                branching_ratio: 0.9862_f64,
                daughter: Some("Thorium-227"),
            },
            DecayMode {
                mode: "alpha",
                branching_ratio: 0.0138_f64,
                daughter: Some("Francium-223"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Actinium-228",
            symbol: "²²⁸Ac",
            mass_number: 228,
            neutrons: 139,
            atomic_mass: 228.031021_f64,
            half_life: Some(6.15_f64),
            half_life_unit: Some("hours"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "beta-",
                branching_ratio: 1.0_f64,
                daughter: Some("Thorium-228"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        ],
    }
}
