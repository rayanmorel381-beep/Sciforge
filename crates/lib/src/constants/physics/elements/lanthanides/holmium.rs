use super::super::{DecayMode, Element, Isotope};

pub fn element() -> Element {
    Element {
        symbol: "Ho",
        name: "Holmium",
        atomic_number: 67,
        atomic_mass: 164.93033_f64,
        electronegativity: Some(1.23_f64),
        group: None,
        period: 6,
        category: "lanthanide",
        electron_configuration: "[Xe] 4f¹¹ 6s²",
        isotopes: vec![
        Isotope {
            name: "Holmium-163",
            symbol: "¹⁶³Ho",
            mass_number: 163,
            neutrons: 96,
            atomic_mass: 162.928734_f64,
            half_life: Some(4570.0_f64),
            half_life_unit: Some("years"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "electron capture",
                branching_ratio: 1.0_f64,
                daughter: Some("Dysprosium-163"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Holmium-165",
            symbol: "¹⁶⁵Ho",
            mass_number: 165,
            neutrons: 98,
            atomic_mass: 164.930322_f64,
            half_life: None,
            half_life_unit: None,
            stable: true,
            decay_modes: vec![],
            natural_abundance: 1.0_f64,
            nuclear_spin: Some("7/2-"),
        },
        Isotope {
            name: "Holmium-166",
            symbol: "¹⁶⁶Ho",
            mass_number: 166,
            neutrons: 99,
            atomic_mass: 165.932284_f64,
            half_life: Some(26.824_f64),
            half_life_unit: Some("hours"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "beta-",
                branching_ratio: 1.0_f64,
                daughter: Some("Erbium-166"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        ],
    }
}
