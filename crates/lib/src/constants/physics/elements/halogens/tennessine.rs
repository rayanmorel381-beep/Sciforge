use super::super::{DecayMode, Element, Isotope};

pub fn element() -> Element {
    Element {
        symbol: "Ts",
        name: "Tennessine",
        atomic_number: 117,
        atomic_mass: 294.0_f64,
        electronegativity: None,
        group: Some(17),
        period: 7,
        category: "halogen",
        electron_configuration: "[Rn] 5f¹⁴ 6d¹⁰ 7s² 7p⁵ (predicted)",
        isotopes: vec![
        Isotope {
            name: "Tennessine-293",
            symbol: "²⁹³Ts",
            mass_number: 293,
            neutrons: 176,
            atomic_mass: 293.20824_f64,
            half_life: Some(22.0_f64),
            half_life_unit: Some("milliseconds"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "alpha emission",
                branching_ratio: 1.0_f64,
                daughter: Some("Moscovium-289"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Tennessine-294",
            symbol: "²⁹⁴Ts",
            mass_number: 294,
            neutrons: 177,
            atomic_mass: 294.21046_f64,
            half_life: Some(51.0_f64),
            half_life_unit: Some("milliseconds"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "alpha emission",
                branching_ratio: 1.0_f64,
                daughter: Some("Moscovium-290"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        ],
    }
}
