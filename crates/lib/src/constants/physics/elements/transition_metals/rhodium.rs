use super::super::{DecayMode, Element, Isotope};

pub fn element() -> Element {
    Element {
        symbol: "Rh",
        name: "Rhodium",
        atomic_number: 45,
        atomic_mass: 102.9055_f64,
        electronegativity: Some(2.28_f64),
        group: Some(9),
        period: 5,
        category: "transition metal",
        electron_configuration: "[Kr] 4d⁸ 5s¹",
        isotopes: vec![
        Isotope {
            name: "Rhodium-99",
            symbol: "⁹⁹Rh",
            mass_number: 99,
            neutrons: 54,
            atomic_mass: 98.908132_f64,
            half_life: Some(16.1_f64),
            half_life_unit: Some("days"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "beta+ / electron capture",
                branching_ratio: 1.0_f64,
                daughter: Some("Ruthenium-99"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Rhodium-101",
            symbol: "¹⁰¹Rh",
            mass_number: 101,
            neutrons: 56,
            atomic_mass: 100.906164_f64,
            half_life: Some(3.3_f64),
            half_life_unit: Some("years"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "electron capture",
                branching_ratio: 1.0_f64,
                daughter: Some("Ruthenium-101"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Rhodium-103",
            symbol: "¹⁰³Rh",
            mass_number: 103,
            neutrons: 58,
            atomic_mass: 102.905504_f64,
            half_life: None,
            half_life_unit: None,
            stable: true,
            decay_modes: vec![],
            natural_abundance: 1.0_f64,
            nuclear_spin: Some("1/2-"),
        },
        Isotope {
            name: "Rhodium-105",
            symbol: "¹⁰⁵Rh",
            mass_number: 105,
            neutrons: 60,
            atomic_mass: 104.905694_f64,
            half_life: Some(35.36_f64),
            half_life_unit: Some("hours"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "beta-",
                branching_ratio: 1.0_f64,
                daughter: Some("Palladium-105"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        ],
    }
}
