use super::super::{DecayMode, Element, Isotope};

pub fn element() -> Element {
    Element {
        symbol: "Mt",
        name: "Meitnerium",
        atomic_number: 109,
        atomic_mass: 278.156_f64,
        electronegativity: None,
        group: Some(9),
        period: 7,
        category: "unknown",
        electron_configuration: "[Rn] 5f¹⁴ 6d⁷ 7s²",
        isotopes: vec![
        Isotope {
            name: "Meitnerium-274",
            symbol: "²⁷⁴Mt",
            mass_number: 274,
            neutrons: 165,
            atomic_mass: 274.147_f64,
            half_life: Some(0.44_f64),
            half_life_unit: Some("seconds"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "alpha",
                branching_ratio: 1.0_f64,
                daughter: Some("Bohrium-270"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Meitnerium-276",
            symbol: "²⁷⁶Mt",
            mass_number: 276,
            neutrons: 167,
            atomic_mass: 276.152_f64,
            half_life: Some(0.72_f64),
            half_life_unit: Some("seconds"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "alpha",
                branching_ratio: 1.0_f64,
                daughter: Some("Bohrium-272"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Meitnerium-278",
            symbol: "²⁷⁸Mt",
            mass_number: 278,
            neutrons: 169,
            atomic_mass: 278.156_f64,
            half_life: Some(4.5_f64),
            half_life_unit: Some("seconds"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "alpha",
                branching_ratio: 1.0_f64,
                daughter: Some("Bohrium-274"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        ],
    }
}
