use super::super::{DecayMode, Element, Isotope};

pub fn element() -> Element {
    Element {
        symbol: "Tc",
        name: "Technetium",
        atomic_number: 43,
        atomic_mass: 97.907216_f64,
        electronegativity: Some(1.9_f64),
        group: Some(7),
        period: 5,
        category: "transition metal",
        electron_configuration: "[Kr] 4d⁵ 5s²",
        isotopes: vec![
        Isotope {
            name: "Technetium-95m",
            symbol: "⁹⁵ᵐTc",
            mass_number: 95,
            neutrons: 52,
            atomic_mass: 94.907657_f64,
            half_life: Some(61.0_f64),
            half_life_unit: Some("days"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "electron capture",
                branching_ratio: 1.0_f64,
                daughter: Some("Molybdenum-95"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Technetium-97",
            symbol: "⁹⁷Tc",
            mass_number: 97,
            neutrons: 54,
            atomic_mass: 96.906365_f64,
            half_life: Some(4210000.0_f64),
            half_life_unit: Some("years"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "electron capture",
                branching_ratio: 1.0_f64,
                daughter: Some("Molybdenum-97"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Technetium-98",
            symbol: "⁹⁸Tc",
            mass_number: 98,
            neutrons: 55,
            atomic_mass: 97.907216_f64,
            half_life: Some(4200000.0_f64),
            half_life_unit: Some("years"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "beta-",
                branching_ratio: 1.0_f64,
                daughter: Some("Ruthenium-98"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Technetium-99",
            symbol: "⁹⁹Tc",
            mass_number: 99,
            neutrons: 56,
            atomic_mass: 98.906255_f64,
            half_life: Some(211100.0_f64),
            half_life_unit: Some("years"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "beta-",
                branching_ratio: 1.0_f64,
                daughter: Some("Ruthenium-99"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Technetium-99m",
            symbol: "⁹⁹ᵐTc",
            mass_number: 99,
            neutrons: 56,
            atomic_mass: 98.906255_f64,
            half_life: Some(6.0067_f64),
            half_life_unit: Some("hours"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "isomeric transition",
                branching_ratio: 1.0_f64,
                daughter: Some("Technetium-99"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        ],
    }
}
