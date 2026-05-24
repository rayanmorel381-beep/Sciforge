use super::super::{DecayMode, Element, Isotope};

pub fn element() -> Element {
    Element {
        symbol: "Ta",
        name: "Tantalum",
        atomic_number: 73,
        atomic_mass: 180.94788_f64,
        electronegativity: Some(1.5_f64),
        group: Some(5),
        period: 6,
        category: "transition metal",
        electron_configuration: "[Xe] 4f¹⁴ 5d³ 6s²",
        isotopes: vec![
        Isotope {
            name: "Tantalum-180m",
            symbol: "¹⁸⁰ᵐTa",
            mass_number: 180,
            neutrons: 107,
            atomic_mass: 179.947465_f64,
            half_life: Some(1200000000000000.0_f64),
            half_life_unit: Some("years"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "isomeric transition",
                branching_ratio: 1.0_f64,
                daughter: Some("Tantalum-180"),
            },
            ],
            natural_abundance: 0.0001201_f64,
            nuclear_spin: None,
        },
        Isotope {
            name: "Tantalum-181",
            symbol: "¹⁸¹Ta",
            mass_number: 181,
            neutrons: 108,
            atomic_mass: 180.947996_f64,
            half_life: None,
            half_life_unit: None,
            stable: true,
            decay_modes: vec![],
            natural_abundance: 0.9998799_f64,
            nuclear_spin: Some("7/2+"),
        },
        Isotope {
            name: "Tantalum-182",
            symbol: "¹⁸²Ta",
            mass_number: 182,
            neutrons: 109,
            atomic_mass: 181.950152_f64,
            half_life: Some(114.43_f64),
            half_life_unit: Some("days"),
            stable: false,
            decay_modes: vec![
            DecayMode {
                mode: "beta-",
                branching_ratio: 1.0_f64,
                daughter: Some("Tungsten-182"),
            },
            ],
            natural_abundance: 0.0_f64,
            nuclear_spin: None,
        },
        ],
    }
}
