use super::super::{DecayMode, Element, Isotope};

pub fn element() -> Element {
    Element {
        symbol: "Cf",
        name: "Californium",
        atomic_number: 98,
        atomic_mass: 251.079587_f64,
        electronegativity: Some(1.3_f64),
        group: None,
        period: 7,
        category: "actinide",
        electron_configuration: "[Rn] 5f¹⁰ 7s²",
        isotopes: vec![
        Isotope {
            name: "Californium-249",
            symbol: "²⁴⁹Cf",
            mass_number: 249,
            neutrons: 151,
            atomic_mass: 249.074854_f64,
            half_life: Some(351.0_f64),
            half_life_unit: Some("years"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "alpha",
                branching_ratio: 1.0_f64,
                daughter: Some("Curium-245"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Californium-250",
            symbol: "²⁵⁰Cf",
            mass_number: 250,
            neutrons: 152,
            atomic_mass: 250.076406_f64,
            half_life: Some(13.08_f64),
            half_life_unit: Some("years"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "alpha",
                branching_ratio: 0.9986_f64,
                daughter: Some("Curium-246"),
            },
            DecayMode {
                mode: "spontaneous_fission",
                branching_ratio: 0.0014_f64,
                daughter: None,
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Californium-251",
            symbol: "²⁵¹Cf",
            mass_number: 251,
            neutrons: 153,
            atomic_mass: 251.079587_f64,
            half_life: Some(900.0_f64),
            half_life_unit: Some("years"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "alpha",
                branching_ratio: 1.0_f64,
                daughter: Some("Curium-247"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Californium-252",
            symbol: "²⁵²Cf",
            mass_number: 252,
            neutrons: 154,
            atomic_mass: 252.081627_f64,
            half_life: Some(2.645_f64),
            half_life_unit: Some("years"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "alpha",
                branching_ratio: 0.9691_f64,
                daughter: Some("Curium-248"),
            },
            DecayMode {
                mode: "spontaneous_fission",
                branching_ratio: 0.0309_f64,
                daughter: None,
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        ],
    }
}
