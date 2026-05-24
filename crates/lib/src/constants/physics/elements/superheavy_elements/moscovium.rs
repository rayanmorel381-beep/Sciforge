use super::super::{DecayMode, Element, Isotope};

pub fn element() -> Element {
    Element {
        symbol: "Mc",
        name: "Moscovium",
        atomic_number: 115,
        atomic_mass: 290.196_f64,
        electronegativity: None,
        group: Some(15),
        period: 7,
        category: "unknown",
        electron_configuration: "[Rn] 5f¹⁴ 6d¹⁰ 7s² 7p³",
        isotopes: vec![
        Isotope {
            name: "Moscovium-287",
            symbol: "²⁸⁷Mc",
            mass_number: 287,
            neutrons: 172,
            atomic_mass: 287.191_f64,
            half_life: Some(37.0_f64),
            half_life_unit: Some("milliseconds"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "alpha",
                branching_ratio: 1.0_f64,
                daughter: Some("Nihonium-283"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Moscovium-288",
            symbol: "²⁸⁸Mc",
            mass_number: 288,
            neutrons: 173,
            atomic_mass: 288.193_f64,
            half_life: Some(164.0_f64),
            half_life_unit: Some("milliseconds"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "alpha",
                branching_ratio: 1.0_f64,
                daughter: Some("Nihonium-284"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Moscovium-289",
            symbol: "²⁸⁹Mc",
            mass_number: 289,
            neutrons: 174,
            atomic_mass: 289.194_f64,
            half_life: Some(330.0_f64),
            half_life_unit: Some("milliseconds"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "alpha",
                branching_ratio: 1.0_f64,
                daughter: Some("Nihonium-285"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Moscovium-290",
            symbol: "²⁹⁰Mc",
            mass_number: 290,
            neutrons: 175,
            atomic_mass: 290.196_f64,
            half_life: Some(0.65_f64),
            half_life_unit: Some("seconds"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "alpha",
                branching_ratio: 1.0_f64,
                daughter: Some("Nihonium-286"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        ],
    }
}
