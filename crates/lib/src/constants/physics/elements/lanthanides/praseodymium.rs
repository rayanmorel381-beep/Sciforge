use super::super::{DecayMode, Element, Isotope};

pub fn element() -> Element {
    Element {
        symbol: "Pr",
        name: "Praseodymium",
        atomic_number: 59,
        atomic_mass: 140.90766_f64,
        electronegativity: Some(1.13_f64),
        group: None,
        period: 6,
        category: "lanthanide",
        electron_configuration: "[Xe] 4f³ 6s²",
        isotopes: vec![
        Isotope {
            name: "Praseodymium-141",
            symbol: "¹⁴¹Pr",
            mass_number: 141,
            neutrons: 82,
            atomic_mass: 140.907653_f64,
            half_life: None,
            half_life_unit: None,
            stable: true,
            decay_modes: vec![],
            natural_abundance: 1.0_f64,
            nuclear_spin: Some("5/2+"),
        },
        Isotope {
            name: "Praseodymium-142",
            symbol: "¹⁴²Pr",
            mass_number: 142,
            neutrons: 83,
            atomic_mass: 141.910045_f64,
            half_life: Some(19.12_f64),
            half_life_unit: Some("hours"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "beta-",
                branching_ratio: 0.99984_f64,
                daughter: Some("Neodymium-142"),
            },
            DecayMode {
                mode: "electron capture",
                branching_ratio: 0.00016_f64,
                daughter: Some("Cerium-142"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Praseodymium-143",
            symbol: "¹⁴³Pr",
            mass_number: 143,
            neutrons: 84,
            atomic_mass: 142.910817_f64,
            half_life: Some(13.57_f64),
            half_life_unit: Some("days"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "beta-",
                branching_ratio: 1.0_f64,
                daughter: Some("Neodymium-143"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        ],
    }
}
