use super::super::{DecayMode, Element, Isotope};

pub fn element() -> Element {
    Element {
        symbol: "Al",
        name: "Aluminium",
        atomic_number: 13,
        atomic_mass: 26.9815385_f64,
        electronegativity: Some(1.61_f64),
        group: Some(13),
        period: 3,
        category: "post-transition metal",
        electron_configuration: "[Ne] 3s² 3p¹",
        isotopes: vec![
        Isotope {
            name: "Aluminium-26",
            symbol: "²⁶Al",
            mass_number: 26,
            neutrons: 13,
            atomic_mass: 25.986892_f64,
            half_life: Some(717000.0_f64),
            half_life_unit: Some("years"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "beta+",
                branching_ratio: 0.8174_f64,
                daughter: Some("Magnesium-26"),
            },
            DecayMode {
                mode: "electron capture",
                branching_ratio: 0.1826_f64,
                daughter: Some("Magnesium-26"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Aluminium-27",
            symbol: "²⁷Al",
            mass_number: 27,
            neutrons: 14,
            atomic_mass: 26.981539_f64,
            half_life: None,
            half_life_unit: None,
            stable: true,
            decay_modes: vec![],
            natural_abundance: 1.0_f64,
            nuclear_spin: Some("5/2+"),
        },
        ],
    }
}
