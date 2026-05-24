use super::super::{DecayMode, Element, Isotope};

pub fn element() -> Element {
    Element {
        symbol: "Am",
        name: "Americium",
        atomic_number: 95,
        atomic_mass: 243.061381_f64,
        electronegativity: Some(1.13_f64),
        group: None,
        period: 7,
        category: "actinide",
        electron_configuration: "[Rn] 5f⁷ 7s²",
        isotopes: vec![
        Isotope {
            name: "Americium-241",
            symbol: "²⁴¹Am",
            mass_number: 241,
            neutrons: 146,
            atomic_mass: 241.056829_f64,
            half_life: Some(432.2_f64),
            half_life_unit: Some("years"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "alpha",
                branching_ratio: 1.0_f64,
                daughter: Some("Neptunium-237"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Americium-242m",
            symbol: "²⁴²ᵐAm",
            mass_number: 242,
            neutrons: 147,
            atomic_mass: 242.059549_f64,
            half_life: Some(141.0_f64),
            half_life_unit: Some("years"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "isomeric transition",
                branching_ratio: 0.995_f64,
                daughter: Some("Americium-242"),
            },
            DecayMode {
                mode: "alpha",
                branching_ratio: 0.005_f64,
                daughter: Some("Neptunium-238"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Americium-243",
            symbol: "²⁴³Am",
            mass_number: 243,
            neutrons: 148,
            atomic_mass: 243.061381_f64,
            half_life: Some(7370.0_f64),
            half_life_unit: Some("years"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "alpha",
                branching_ratio: 1.0_f64,
                daughter: Some("Neptunium-239"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        ],
    }
}
