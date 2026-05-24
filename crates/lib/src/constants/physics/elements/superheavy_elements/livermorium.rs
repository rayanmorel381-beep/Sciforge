use super::super::{DecayMode, Element, Isotope};

pub fn element() -> Element {
    Element {
        symbol: "Lv",
        name: "Livermorium",
        atomic_number: 116,
        atomic_mass: 293.205_f64,
        electronegativity: None,
        group: Some(16),
        period: 7,
        category: "unknown",
        electron_configuration: "[Rn] 5f¹⁴ 6d¹⁰ 7s² 7p⁴",
        isotopes: vec![
        Isotope {
            name: "Livermorium-290",
            symbol: "²⁹⁰Lv",
            mass_number: 290,
            neutrons: 174,
            atomic_mass: 290.199_f64,
            half_life: Some(7.1_f64),
            half_life_unit: Some("milliseconds"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "alpha",
                branching_ratio: 1.0_f64,
                daughter: Some("Flerovium-286"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Livermorium-291",
            symbol: "²⁹¹Lv",
            mass_number: 291,
            neutrons: 175,
            atomic_mass: 291.201_f64,
            half_life: Some(18.0_f64),
            half_life_unit: Some("milliseconds"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "alpha",
                branching_ratio: 1.0_f64,
                daughter: Some("Flerovium-287"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Livermorium-292",
            symbol: "²⁹²Lv",
            mass_number: 292,
            neutrons: 176,
            atomic_mass: 292.202_f64,
            half_life: Some(13.0_f64),
            half_life_unit: Some("milliseconds"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "alpha",
                branching_ratio: 1.0_f64,
                daughter: Some("Flerovium-288"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Livermorium-293",
            symbol: "²⁹³Lv",
            mass_number: 293,
            neutrons: 177,
            atomic_mass: 293.205_f64,
            half_life: Some(53.0_f64),
            half_life_unit: Some("milliseconds"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "alpha",
                branching_ratio: 1.0_f64,
                daughter: Some("Flerovium-289"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        ],
    }
}
