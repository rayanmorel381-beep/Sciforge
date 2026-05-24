use super::super::{DecayMode, Element, Isotope};

pub fn element() -> Element {
    Element {
        symbol: "Nh",
        name: "Nihonium",
        atomic_number: 113,
        atomic_mass: 286.182_f64,
        electronegativity: None,
        group: Some(13),
        period: 7,
        category: "unknown",
        electron_configuration: "[Rn] 5f¹⁴ 6d¹⁰ 7s² 7p¹",
        isotopes: vec![
        Isotope {
            name: "Nihonium-278",
            symbol: "²⁷⁸Nh",
            mass_number: 278,
            neutrons: 165,
            atomic_mass: 278.16_f64,
            half_life: Some(1.4_f64),
            half_life_unit: Some("milliseconds"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "alpha",
                branching_ratio: 1.0_f64,
                daughter: Some("Roentgenium-274"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Nihonium-282",
            symbol: "²⁸²Nh",
            mass_number: 282,
            neutrons: 169,
            atomic_mass: 282.171_f64,
            half_life: Some(73.0_f64),
            half_life_unit: Some("milliseconds"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "alpha",
                branching_ratio: 1.0_f64,
                daughter: Some("Roentgenium-278"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Nihonium-284",
            symbol: "²⁸⁴Nh",
            mass_number: 284,
            neutrons: 171,
            atomic_mass: 284.178_f64,
            half_life: Some(0.91_f64),
            half_life_unit: Some("seconds"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "alpha",
                branching_ratio: 1.0_f64,
                daughter: Some("Roentgenium-280"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Nihonium-286",
            symbol: "²⁸⁶Nh",
            mass_number: 286,
            neutrons: 173,
            atomic_mass: 286.182_f64,
            half_life: Some(9.5_f64),
            half_life_unit: Some("seconds"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "alpha",
                branching_ratio: 1.0_f64,
                daughter: Some("Roentgenium-282"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        ],
    }
}
