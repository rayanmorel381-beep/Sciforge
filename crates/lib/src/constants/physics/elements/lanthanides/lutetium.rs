use super::super::{DecayMode, Element, Isotope};

pub fn element() -> Element {
    Element {
        symbol: "Lu",
        name: "Lutetium",
        atomic_number: 71,
        atomic_mass: 174.9668_f64,
        electronegativity: Some(1.27_f64),
        group: Some(3),
        period: 6,
        category: "lanthanide",
        electron_configuration: "[Xe] 4f¹⁴ 5d¹ 6s²",
        isotopes: vec![
        Isotope {
            name: "Lutetium-175",
            symbol: "¹⁷⁵Lu",
            mass_number: 175,
            neutrons: 104,
            atomic_mass: 174.940772_f64,
            half_life: None,
            half_life_unit: None,
            stable: true,
            decay_modes: vec![],
            natural_abundance: 0.97401_f64,
            nuclear_spin: Some("7/2+"),
        },
        Isotope {
            name: "Lutetium-176",
            symbol: "¹⁷⁶Lu",
            mass_number: 176,
            neutrons: 105,
            atomic_mass: 175.942686_f64,
            half_life: Some(37600000000.0_f64),
            half_life_unit: Some("years"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "beta-",
                branching_ratio: 1.0_f64,
                daughter: Some("Hafnium-176"),
            },
            ],
            natural_abundance: 0.02599_f64,
            nuclear_spin: Some("7-"),
        },
        Isotope {
            name: "Lutetium-177",
            symbol: "¹⁷⁷Lu",
            mass_number: 177,
            neutrons: 106,
            atomic_mass: 176.943758_f64,
            half_life: Some(6.647_f64),
            half_life_unit: Some("days"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "beta-",
                branching_ratio: 1.0_f64,
                daughter: Some("Hafnium-177"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        ],
    }
}
